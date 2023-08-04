use std::{path::PathBuf, sync::Arc};

use parking_lot::Mutex;
use swc_common::sync::Lazy;
use wasmer::Store;
use wasmer_wasix::Runtime;

/// A shared instance to plugin runtime engine.
/// ref: https://github.com/wasmerio/wasmer/issues/3793#issuecomment-1607117480
static ENGINE: Lazy<Mutex<wasmer::Engine>> = Lazy::new(|| {
    // Use empty enumset to disable simd.
    use enumset::EnumSet;
    use wasmer::{BaseTunables, CompilerConfig, EngineBuilder, Target, Triple};
    let mut set = EnumSet::new();

    // [TODO]: Should we use is_x86_feature_detected! macro instead?
    #[cfg(target_arch = "x86_64")]
    set.insert(wasmer::CpuFeature::SSE2);
    let target = Target::new(Triple::host(), set);

    let config = wasmer_compiler_cranelift::Cranelift::default();
    let mut engine = EngineBuilder::new(Box::new(config) as Box<dyn CompilerConfig>)
        .set_target(Some(target))
        .engine();
    let tunables = BaseTunables::for_target(engine.target());
    engine.set_tunables(tunables);
    parking_lot::Mutex::new(wasmer::Engine::from(engine))
});

/// Dummy http client for wasix runtime to avoid instantiation failure for the
/// default pluggable runtime. We don't support network in the host runtime
/// anyway (we init vnet instead), and for the default runtime mostly it's for
/// the wapm registry which is redundant for the plugin.
#[derive(Debug)]
struct StubHttpClient;

impl wasmer_wasix::http::HttpClient for StubHttpClient {
    fn request(
        &self,
        _request: wasmer_wasix::http::HttpRequest,
    ) -> futures::future::BoxFuture<'_, Result<wasmer_wasix::http::HttpResponse, anyhow::Error>>
    {
        unimplemented!()
    }
}

/// Construct a runtime for the wasix engine depends on the compilation
/// features.
///
/// This is mainly for the case if a host already sets up its runtime, which
/// makes wasix initialization fails due to conflicting runtime. When specified,
/// instead of using default runtime it'll try to use shared one.
pub fn build_wasi_runtime(
    _fs_cache_path: Option<PathBuf>,
) -> Option<Arc<dyn Runtime + Send + Sync>> {
    #[cfg(feature = "plugin_transform_host_native_shared_runtime")]
    let task_mgr = tokio::runtime::Handle::current();

    #[cfg(not(feature = "plugin_transform_host_native_shared_runtime"))]
    let task_mgr = wasmer_wasix::runtime::task_manager::tokio::TokioTaskManager::default();

    Some(Arc::new(wasmer_wasix::PluggableRuntime::new(Arc::new(
        task_mgr,
    ))))
}

/// Creates an instnace of [Store] with custom engine instead of default one to
/// disable simd for certain platform targets
#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_mut)]
pub(crate) fn new_store() -> Store {
    let engine = ENGINE.lock().clone();
    Store::new(engine)
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn new_store() -> Store {
    Store::default()
}

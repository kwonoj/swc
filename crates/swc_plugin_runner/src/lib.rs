use std::path::Path;

use anyhow::{Context, Error};
use once_cell::sync::Lazy;
use swc_common::{comments::SingleThreadedComments, plugin::Serialized};
use swc_plugin_comments::{PluginStorage, COMMENTS};
use transform_executor::TransformExecutor;

pub mod cache;
mod context;
mod imported_fn;
mod load_plugin;
mod memory_interop;
mod transform_executor;

// entrypoint fn swc calls to perform its transform via plugin.
pub fn apply_transform_plugin<'cmt>(
    plugin_name: &str,
    path: &Path,
    cache: &Lazy<cache::PluginModuleCache>,
    program: Serialized,
    comments: Option<&'cmt SingleThreadedComments>,
    config_json: Serialized,
    context_json: Serialized,
) -> Result<Serialized, Error> {
    (|| -> Result<_, Error> {
        COMMENTS.set(PluginStorage { inner: comments }, || {
            let mut transform_tracker = TransformExecutor::new(path, cache)?;
            transform_tracker.transform(&program, &config_json, &context_json)
        })
    })()
    .with_context(|| {
        format!(
            "failed to invoke `{}` as js transform plugin at {}",
            plugin_name,
            path.display()
        )
    })
}

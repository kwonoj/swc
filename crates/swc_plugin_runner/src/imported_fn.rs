//! Functions imported into the guests (plugin) runtime allows interop between
//! host's state to plugin.
//! All of these fn is being called inside of guest's memory space, which calls
//! appropriate host fn as needed.

use swc_common::{
    comments::Comments,
    errors::{Diagnostic, HANDLER},
    hygiene::MutableMarkContext,
    plugin::Serialized,
    BytePos, Mark, SyntaxContext,
};
use swc_plugin_comments::{PluginStorage, COMMENTS};

use crate::{
    context::HostEnvironment,
    memory_interop::{copy_bytes_into_host, write_into_memory_view},
};

/// Set plugin's transformed result into host's enviroment.
/// This is an `imported` fn - when we instantiate plugin module, we inject this
/// fn into pluging's export space. Once transform completes, plugin will call
/// this to set its result back to host.
pub fn set_transform_result(env: &HostEnvironment, bytes_ptr: i32, bytes_ptr_len: i32) {
    if let Some(memory) = env.memory_ref() {
        (*env.transform_result.lock()) = copy_bytes_into_host(memory, bytes_ptr, bytes_ptr_len);
    }
}

pub fn emit_diagnostics(env: &HostEnvironment, bytes_ptr: i32, bytes_ptr_len: i32) {
    if let Some(memory) = env.memory_ref() {
        if HANDLER.is_set() {
            HANDLER.with(|handler| {
                let diagnostics_bytes = copy_bytes_into_host(memory, bytes_ptr, bytes_ptr_len);
                let serialized = Serialized::new_for_plugin(&diagnostics_bytes[..], bytes_ptr_len);
                let diagnostic = Serialized::deserialize::<Diagnostic>(&serialized)
                    .expect("Should able to be deserialized into diagnsotic");

                let mut builder =
                    swc_common::errors::DiagnosticBuilder::new_diagnostic(handler, diagnostic);
                builder.emit();
            })
        }
    }
}

/// A proxy to Mark::fresh() that can be used in plugin.
/// This it not direcly called by plugin, instead `impl Mark` will selectively
/// call this depends on the running context.
pub fn mark_fresh_proxy(parent: u32) -> u32 {
    Mark::fresh(Mark::from_u32(parent)).as_u32()
}

pub fn mark_parent_proxy(self_mark: u32) -> u32 {
    Mark::from_u32(self_mark).parent().as_u32()
}

pub fn mark_is_builtin_proxy(self_mark: u32) -> u32 {
    Mark::from_u32(self_mark).is_builtin() as u32
}

pub fn mark_set_builtin_proxy(self_mark: u32, is_builtin: u32) {
    Mark::from_u32(self_mark).set_is_builtin(is_builtin != 0);
}

/// A proxy to Mark::is_descendant_of_() that can be used in plugin.
/// Origianl call site have mutable param, which we'll pass over as return value
/// via serialized MutableMarkContext.
/// Inside of guest context, once this host function returns it'll assign params
/// with return value accordingly.
pub fn mark_is_descendant_of_proxy(
    env: &HostEnvironment,
    self_mark: u32,
    ancestor: u32,
    allocated_ptr: i32,
) {
    let self_mark = Mark::from_u32(self_mark);
    let ancestor = Mark::from_u32(ancestor);

    let return_value = self_mark.is_descendant_of(ancestor);

    if let Some(memory) = env.memory_ref() {
        let serialized_bytes = Serialized::serialize(&MutableMarkContext(
            self_mark.as_u32(),
            0,
            return_value as u32,
        ))
        .expect("Should be serializable");

        write_into_memory_view(memory, &serialized_bytes, |_| allocated_ptr);
    }
}

pub fn mark_least_ancestor_proxy(env: &HostEnvironment, a: u32, b: u32, allocated_ptr: i32) {
    let a = Mark::from_u32(a);
    let b = Mark::from_u32(b);

    let return_value = Mark::least_ancestor(a, b).as_u32();

    if let Some(memory) = env.memory_ref() {
        let serialized_bytes =
            Serialized::serialize(&MutableMarkContext(a.as_u32(), b.as_u32(), return_value))
                .expect("Should be serializable");

        write_into_memory_view(memory, &serialized_bytes, |_| allocated_ptr);
    }
}

pub fn syntax_context_apply_mark_proxy(self_syntax_context: u32, mark: u32) -> u32 {
    SyntaxContext::from_u32(self_syntax_context)
        .apply_mark(Mark::from_u32(mark))
        .as_u32()
}

pub fn syntax_context_remove_mark_proxy(env: &HostEnvironment, self_mark: u32, allocated_ptr: i32) {
    let mut self_mark = SyntaxContext::from_u32(self_mark);

    let return_value = self_mark.remove_mark();

    if let Some(memory) = env.memory_ref() {
        let serialized_bytes = Serialized::serialize(&MutableMarkContext(
            self_mark.as_u32(),
            0,
            return_value.as_u32(),
        ))
        .expect("Should be serializable");

        write_into_memory_view(memory, &serialized_bytes, |_| allocated_ptr);
    }
}

pub fn syntax_context_outer_proxy(self_mark: u32) -> u32 {
    SyntaxContext::from_u32(self_mark).outer().as_u32()
}

// Since get_leading_comment returns non-deterministic Option<Vec<Comment>>,
// guest cannot pre-allocated its memory. We have to call this first to get the
// size to allocate.
pub fn get_leading_comment_len_proxy(byte_pos: u32) -> u32 {
    if COMMENTS.is_set() {
        return COMMENTS.with(|storage: PluginStorage| {
            if let Some(comments) = storage.inner {
                let x = comments.get_leading(BytePos(byte_pos)).map_or(0, |v| {
                    Serialized::serialize(&v)
                        .expect("Should be serializable")
                        .as_ref()
                        .len()
                        .try_into()
                        .expect("")
                });
                x
            } else {
                0
            }
        });
    }
    0
}

pub fn get_leading_comment_proxy(
    env: &HostEnvironment,
    byte_pos: u32,
    vec_allocated_ptr: i32,
    vec_allocated_len: i32,
) {
    if let Some(memory) = env.memory_ref() {
        if COMMENTS.is_set() {
            COMMENTS.with(|storage: PluginStorage| {
                if let Some(comments) = storage.inner {
                    let byte_pos = BytePos(byte_pos);
                    let value = comments.get_leading(byte_pos);

                    if let Some(value) = value {
                        let serialized_comment_vec_bytes =
                            Serialized::serialize(&value).expect("Should be serializable");

                        println!(
                            " serialized {:#?} {:#?}",
                            serialized_comment_vec_bytes.as_ref().len(),
                            vec_allocated_len
                        );
                        write_into_memory_view(memory, &serialized_comment_vec_bytes, |_| {
                            vec_allocated_ptr
                        });
                        println!("write done");
                    }
                }
            });
        }
    }
}

/// Internal thread-local storage to share `SingleThreadedComments` between host
/// to the plugin while running transform plugin.
///
/// Each time before plugin runs this will be set corresponding file's parsed
/// comments struct. Plugin will use interface to the comments to access
/// necessary comments if needed.
///
/// This allow lazy request to the comments for the plugin only if they need to
/// access instead of eagerly serialize & allocated whole comments into plugin's
/// memory space.
///
/// NOTE: THIS IS STRICTLY INTERNAL INTERFACE. Do not attempt to use it other
/// than plugin_runner / plugin sdk.
mod comments;
pub use comments::*;

mod spec;
pub use spec::AuiSpec;

use yew::{hook, use_context};
#[hook]
pub fn use_spec() -> AuiSpec {
    use_context::<AuiSpec>().expect("no accessible-ui spec provider")
}

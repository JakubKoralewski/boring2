// Callback to add a `link_name` macro with the prefix to all generated bindings
#[derive(Debug)]
pub struct PrefixCallback;

fn add_prefix(name: &str) -> String {
    format!("b2_{}", name)
}

impl bindgen::callbacks::ParseCallbacks for PrefixCallback {
    fn generated_link_name_override(
        &self,
        item_info: bindgen::callbacks::ItemInfo<'_>,
    ) -> Option<String> {
        Some(add_prefix(item_info.name))
    }
}

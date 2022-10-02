pub mod api;
pub mod util;
pub mod systemd;
pub fn register_types(registry: &mut ::protokit::reflect::Registry) {
    api::register_types(registry);
    util::register_types(registry);
    systemd::register_types(registry);
}

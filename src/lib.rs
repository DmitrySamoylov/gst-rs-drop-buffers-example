use gst::{glib, plugin_define};

mod filter;

plugin_define!(
    filter,
    "Description",
    plugin_init,
    env!("CARGO_PKG_VERSION"),
    "Proprietary",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    "Repo",
    "2020-01-01"
);

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    filter::register(plugin)?;

    Ok(())
}

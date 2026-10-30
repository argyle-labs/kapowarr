//! Dynamic (subprocess) entrypoint for the kapowarr plugin.
//!
//! Service-backend only — deploy/backup/restore of the Kapowarr instance
//! itself. No `#[orca_tool]` surface to force-link.
plugin_toolkit::instrument::bootstrap!();
use plugin_toolkit::plugin::Plugin;
fn main() -> plugin_toolkit::anyhow::Result<()> {
    Plugin::named("kapowarr")
        .version(env!("CARGO_PKG_VERSION"))
        .service(kapowarr::KapowarrServiceBackend::new("kapowarr"))
        .serve()
}

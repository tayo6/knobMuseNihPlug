use nih_plug_xtask::{build, bundle, collect_args, BundleConfig, BuildConfig};
fn main() -> anyhow::Result<()> {
    let args = collect_args();
    match args.command.as_str() {
        "bundle" => {
            build(BuildConfig::default())?;
            bundle(BundleConfig::from_args(&args)?)?;
        }
        _ => build(BuildConfig::from_args(&args)?),
    }
    Ok(())
}
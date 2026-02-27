use {{project_name}}::init_tracing;

fn main() ->anyhow::Result<()> {
    let _guard = init_tracing()?;
    tracing::info!("Hello, world!");
    Ok(())
}

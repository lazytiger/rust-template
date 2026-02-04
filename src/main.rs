use guardns::init_tracing;

fn main() {
    init_tracing().unwrap();
    tracing::info!("Hello, world!");
}

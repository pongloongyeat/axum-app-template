use backend::core::App;

#[tokio::main]
async fn main() {
    App::serve().await
}

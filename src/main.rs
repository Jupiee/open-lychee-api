use open_lychee_api::run;

#[tokio::main]
async fn main() {

    let app= run().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");
    
    axum::serve(listener, app).await.unwrap();


}
use tide::{Body, Request};
use tide_websockets::{Message as WSMessage, WebSocket, WebSocketConnection};
use futures_util::StreamExt;


#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let mut app = tide::new();

    // serve public dir for assets
    app.at("/public").serve_dir("./public/")?;

    // index route
    app.at("/").get(|_| async { Ok(Body::from_file("./public/index.html").await?) });

    // board route
    app.at("/:id")
        .with(WebSocket::new(
            |_req: Request<_>, mut wsc: WebSocketConnection| async move {
                while let Some(Ok(WSMessage::Text(message))) = wsc.next().await {
                    println!("{:?}", message);
                }

                Ok(())
            },
        ))
        .get(|_| async { Ok(Body::from_file("./public/board.html").await?) });
    
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("127.0.0.1:{}", port);
    println!("Starting server listening at {}", addr);
    app.listen(addr).await?;

    Ok(())
}

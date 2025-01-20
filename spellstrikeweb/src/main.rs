use actix_files as fs;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
/*
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}
*/
// This function will handle requests to the "/server" URL
async fn game_server() -> impl Responder {
    HttpResponse::Ok().body("Game server response")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("INFO: addr - 127.0.0.1:4000");
  HttpServer::new(|| {
      App::new()
          // Serve static files from the "static" directory
          .service(fs::Files::new("/", "./static").index_file("index.html"))
          // Define the route for the root URL ("/")
 //         .route("/", web::get().to(index))
          // Define the route for the "/server" URL
          .route("/server", web::get().to(game_server))
  })
  .bind("127.0.0.1:4000")? // Bind to the specified IP address and port
  .run()
  .await
}

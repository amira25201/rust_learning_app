mod question;
mod web;
mod p_papers_questions;

#[tokio::main]
async fn main() {
    println!("Starting Kuromi Maths...");

    web::run_server().await;
}

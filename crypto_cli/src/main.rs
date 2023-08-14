mod api;
mod cli;


#[tokio::main]
async fn main(){
    cli::cli().await;
}
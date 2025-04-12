use a2a_rust::agents::openai_agent::{OpenAIAgent, server::create_router};
use clap::Parser;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host address to bind to
    #[arg(short, long, default_value = "127.0.0.1")]
    host: String,

    /// Port to listen on
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Parse command line arguments
    let args = Args::parse();
    let addr = format!("{}:{}", args.host, args.port)
        .parse::<SocketAddr>()
        .expect("Invalid address");

    // Create the OpenAI agent
    let agent = OpenAIAgent::new()?;
    
    // Create the router
    let app = create_router(agent);

    // Start the server
    println!("Starting OpenAI A2A agent on {}", addr);
    println!("Make sure you have set the OPENAI_API_KEY environment variable");
    println!("You can optionally set OPENAI_MODEL (defaults to gpt-3.5-turbo)");
    
    axum::serve(tokio::net::TcpListener::bind(&addr).await?, app)
        .await?;
    
    Ok(())
}

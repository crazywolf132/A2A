use a2a_rust::A2AClient;
use clap::Parser;
use std::io::{self, Write};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// A2A server URL
    #[arg(short, long, default_value = "http://localhost:3000")]
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Parse command line arguments
    let args = Args::parse();
    
    // Create A2A client
    let client = A2AClient::new(&args.url);
    
    println!("A2A Rust Client");
    println!("Type 'exit' to quit");
    println!("Connected to {}", args.url);
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        
        if input == "exit" {
            break;
        }
        
        match client.send_task(input).await {
            Ok(task) => {
                if let Some(message) = &task.status.message {
                    for part in &message.parts {
                        match part {
                            a2a_rust::types::Part::Text(text_part) => {
                                println!("Agent: {}", text_part.text);
                            }
                            _ => {
                                println!("Agent: [Non-text response]");
                            }
                        }
                    }
                } else {
                    println!("Agent: [No response]");
                }
                
                if let Some(artifacts) = &task.artifacts {
                    println!("Artifacts:");
                    for (i, artifact) in artifacts.iter().enumerate() {
                        println!("  {}. {}", i + 1, artifact.name.as_deref().unwrap_or("Unnamed"));
                        for part in &artifact.parts {
                            match part {
                                a2a_rust::types::Part::Text(text_part) => {
                                    println!("     {}", text_part.text);
                                }
                                _ => {
                                    println!("     [Non-text content]");
                                }
                            }
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
    
    Ok(())
}

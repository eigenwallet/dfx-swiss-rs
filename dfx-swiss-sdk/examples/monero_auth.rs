use dfx_swiss_sdk::{DfxClient, SignRequest, DfxError};
use monero_sys::{Daemon, WalletHandle};
use tokio::sync::{mpsc, oneshot};
use tempfile::TempDir;
use std::sync::Arc;

const MAINNET_REMOTE_NODE: &str = "http://xmr-de.boldsuck.org:18081";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .init();

    println!("🔸 DFX Swiss SDK with Real Monero Wallet Example");
    println!("================================================");
    
    // Create temporary directory for wallet
    println!("1. Creating temporary wallet directory...");
    let temp_dir = TempDir::new()?;
    let wallet_path = temp_dir.path().join("dfx_wallet").display().to_string();
    
    // Configure Monero daemon
    let daemon = Daemon {
        address: MAINNET_REMOTE_NODE.into(),
        ssl: false,
    };
    
    // Create new Monero wallet
    println!("2. Creating new Monero wallet...");
    let wallet = WalletHandle::open_or_create(
        wallet_path,
        daemon,
        monero::Network::Mainnet,
        false,
    )
    .await?;
    
    println!("Wallet created successfully!");
    let address = wallet.main_address().await;
    println!("Wallet address: {}", address);
    
    // Create channel for authentication
    let (auth_tx, mut auth_rx) = mpsc::channel::<(SignRequest, oneshot::Sender<String>)>(10);
    
    // Wrap wallet in Arc for sharing between tasks
    let wallet_arc = Arc::new(wallet);
    let wallet_clone = wallet_arc.clone();
    
    // Start signing task
    let signing_task = tokio::spawn(async move {
        println!("   🔄 Signing service is listening for requests...");
        
        while let Some((sign_request, response_tx)) = auth_rx.recv().await {
            println!("   📨 Received signing request:");
            println!("      Message: {}", sign_request.message);
            println!("      Blockchains: {:?}", sign_request.blockchains);
            
            // Sign the message using the real Monero wallet
            let signature = match wallet_clone
                .sign_message(&sign_request.message, None, false) 
                .await 
            {
                Ok(sig) => {
                    println!("   ✅ Message signed successfully!");
                    println!("      Signature: {}...", &sig[..std::cmp::min(50, sig.len())]);
                    sig
                }
                Err(e) => {
                    eprintln!("   ❌ Failed to sign message: {:?}", e);
                    format!("signature_error_{}", e)
                }
            };
            
            // Send signature back to DFX client
            if let Err(e) = response_tx.send(signature) {
                eprintln!("   ❌ Failed to send signature response: {:?}", e);
            }
        }
    });
    
    // Create DFX client
    println!("5. Initializing DFX client...");
    let mut client = DfxClient::new(
        address.to_string(),
        Some("https://api.dfx.swiss".to_string()),
        auth_tx,
    );
    println!("DFX client initialized");
    
    // Start authentication process
    println!("6. Starting DFX authentication...");
    println!("   🔐 Requesting authentication challenge...");
    
    match client.authenticate().await {
        Ok(()) => {
            println!("   ✅ Authentication completed successfully!");
            println!("   🎟️  JWT token obtained and stored");
            
            // Test authenticated API call
            println!("7. Testing authenticated API call...");
            println!("   📊 Fetching user information...");
            
            match client.get_user().await {
                Ok(user) => {
                    println!("User data retrieved successfully!");
                    println!("User Details:");
                    println!("      Account ID: {:?}", user.account_id);
                    println!("      Account Type: {:?}", user.account_type);
                    println!("      Email: {:?}", user.mail);
                    println!("      Phone: {:?}", user.phone);
                    println!("      Language: {:?}", user.language);
                }
                Err(e) => {
                    eprintln!("Failed to get user data: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Authentication failed: {:?}", e);
            match e {
                DfxError::ChannelError(msg) => {
                    eprintln!("Channel communication issue: {}", msg);
                    if msg.contains("Timeout") {
                        eprintln!("This is expected since sign_message panics");
                    }
                }
                DfxError::AuthenticationError(msg) => {
                    eprintln!("Authentication issue: {}", msg);
                }
                DfxError::RawApiError(msg) => {
                    eprintln!("API error: {}", msg);
                }
                _ => {}
            }
        }
    }
    
    Ok(())
}
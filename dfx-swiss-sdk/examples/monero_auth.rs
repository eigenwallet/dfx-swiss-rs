use dfx_swiss_sdk::{DfxClient, SignRequest};
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
    
    // Create temporary Monero wallet
    let temp_dir = TempDir::new().expect("Failed to create temporary directory for wallet files");
    let wallet_path = temp_dir.path().join("dfx_wallet").display().to_string();
    
    let daemon = Daemon {
        address: MAINNET_REMOTE_NODE.into(),
        ssl: false,
    };
    
    println!("2. Creating new Monero wallet...");
    let wallet = WalletHandle::open_or_create(
        wallet_path,
        daemon,
        monero::Network::Mainnet,
        false,
    )
    .await
    .expect("Failed to create Monero wallet");
    
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
        println!("🔄 Signing service is listening for requests...");
        
        while let Some((sign_request, response_tx)) = auth_rx.recv().await {
            println!("📨 Received signing request:");
            println!("  Message: {}", sign_request.message);
            println!("  Blockchains: {:?}", sign_request.blockchains);
            
            // Sign the message using the real Monero wallet
            let signature = wallet_clone
                .sign_message(&sign_request.message, None, false) 
                .await
                .expect("Failed to sign message");
        
            println!("✅ Message signed successfully!");
            println!("  Signature: {}...", &signature[..std::cmp::min(50, signature.len())]);
            
            // Send signature back to DFX client
            response_tx.send(signature).expect("Failed to send signature response through channel to DFX client");
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
    println!("🔐 Requesting authentication challenge...");
    
    client.authenticate().await.expect("Failed to authenticate");

    println!("✅ Authentication completed successfully!");
    
    // Open logged in browser session
    let token = client.access_token.as_ref().expect("No access token available").clone();
    let kyc_url = format!("https://app.dfx.swiss/kyc?session={}", token);
    println!("🌐 Opening browser with KYC URL...");
    
    match open::that(&kyc_url) {
        Ok(()) => {
            println!("✅ Browser opened successfully!");
        }
        Err(e) => {
            eprintln!("❌ Failed to open browser: {}", e);
            println!("🔗 Please manually open: {}", kyc_url);
        }
    }

    // Test authenticated API call
    println!("7. Testing authenticated API call...");

    let user = client.get_user().await.expect("Failed to get user data");
    
    println!("User data retrieved successfully!");
    println!("User Details:");
    println!("      Account ID: {:?}", user.account_id);
    println!("      Account Type: {:?}", user.account_type);
    println!("      Email: {:?}", user.mail);
    println!("      Phone: {:?}", user.phone);
    println!("      Language: {:?}", user.language);
    
    Ok(())
}
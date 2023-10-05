use anyhow::Result;
use clap::Parser;
use num_bigint::BigUint;

pub mod zkp_auth
{
    include!("./zkp_auth.rs");
}

use zkp_auth::{
    auth_client::AuthClient, AuthenticationAnswerRequest, AuthenticationChallengeRequest,
    RegisterRequest,
};
use zkp_chaum_pedersen::ZKP;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
/// Chaum-Pedersen Zero Knowledge Proof (client)
struct Args
{
    /// User name
    #[arg(short, long, required = true)]
    user_name: String,

    /// Password
    #[arg(short, long, required = true)]
    password: String,

    /// Server endpoint
    #[arg(
        short,
        long,
        required = false,
        default_value = "http://127.0.0.1:50051"
    )]
    server: String,
}

#[tokio::main]
async fn main() -> Result<(), String>
{
    let args = &Args::parse();

    let (alpha, beta, p, q) = ZKP::get_constants();
    let zkp = ZKP::new(&p, &q, &alpha, &beta);

    let mut client = match AuthClient::connect(args.server.clone()).await
    {
        Ok(x) => x,
        Err(e) =>
        {
            return Err(format!("Connect failed to {}: {:?}", args.server, e));
        }
    };

    println!("✅ Connected to the server");
    let password = BigUint::from_bytes_be(args.password.trim().as_bytes());

    let (y1, y2) = zkp.compute_pair(&password);

    let username = args.user_name.clone();
    let request = RegisterRequest {
        user: username.clone(),
        y1:   y1.to_bytes_be(),
        y2:   y2.to_bytes_be(),
    };

    let _ = match client.register(request).await
    {
        Ok(x) => x,
        Err(e) =>
        {
            return Err(format!("Could not register user name with server {:?}", e));
        }
    };
    println!("✅ Registration was successful");

    println!("Please provide the password (to login):");
    let password = BigUint::from_bytes_be(args.password.trim().as_bytes());

    let k = ZKP::generate_random_number_below(&q);
    let (r1, r2) = zkp.compute_pair(&k);

    let request = AuthenticationChallengeRequest {
        user: username,
        r1:   r1.to_bytes_be(),
        r2:   r2.to_bytes_be(),
    };

    let response = match client.create_authentication_challenge(request).await
    {
        Ok(x) => x,
        Err(e) =>
        {
            return Err(format!("Could not request challenge to server {:?}", e));
        }
    }
    .into_inner();

    let auth_id = response.auth_id;
    let c = BigUint::from_bytes_be(&response.c);
    let s = zkp.solve(&k, &c, &password);

    let request = AuthenticationAnswerRequest {
        auth_id,
        s: s.to_bytes_be(),
    };

    let response = match client.verify_authentication(request).await
    {
        Ok(x) => x,
        Err(e) =>
        {
            return Err(format!("Could not verify authentication in server {:?}", e));
        }
    }
    .into_inner();

    println!("✅Login successful! session_id: {}", response.session_id);
    Ok(())
}

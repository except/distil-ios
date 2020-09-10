mod solver;
use serde::Deserialize;
use solver::{Device, Solver};
use std::error::Error;

#[derive(Deserialize, Debug, Default)]
struct ChallengeReponse {
    session: String,
    question: String,
}
#[derive(Deserialize, Debug, Default)]
struct TokenResponse {
    token: String,
    expires_in: usize,
    debug_information: Option<String>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::ClientBuilder::new()
        .use_rustls_tls()
        .brotli(true)
        .user_agent("Densimeter Axiom iOS")
        .build()?;

    let device = Device::new();
    let payload = solver::initial_payload(None)?;
    let url = format!(
        "https://api2.endclothing.com/rSa9Vzy3KajA9f9m/v1/challenge?p={}",
        &payload
    );
    let response = client
        .get(&url)
        .header("x-d-domain", "api2.endclothing.com")
        .send()
        .await?;

    let challenge: ChallengeReponse = response.json().await?;

    let solver = Solver::new(&challenge.question, &challenge.session, &device);
    let payload = solver.solve()?;


    let response = client
        .post("https://api2.endclothing.com/rSa9Vzy3KajA9f9m/v1/challenge")
        .body(payload)
        .header("x-d-domain", "api2.endclothing.com")
        .send()
        .await?;

    let token: TokenResponse = response.json().await?;
    dbg!(&token);
    Ok(())
}

use std::error::Error;
use reqwest::blocking::Client;

mod constantes; 

fn main() -> Result<(), Box<dyn Error>> {
    let base_url = "https://api.brawlstars.com/v1/players/%23";
    
    for player in constantes::PLAYER.iter(){
        get_player_info(player, base_url)?;
    }
    
    Ok(())
}

fn get_player_info(tag: &str, base_url: &str) -> Result<(), Box<dyn Error>> {
    let full_url = format!("{}{}", base_url, tag);  
    let client = Client::new();

    let response = client
        .get(&full_url)
        .header("Authorization", format!("Bearer {}", constantes::TOKEN))
        .send()?;  

    println!("Status: {}", response.status());
    let body = response.text()?;  
    println!("Body: {}", body);

    Ok(())
}

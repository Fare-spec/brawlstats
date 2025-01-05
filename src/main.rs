use std::{error::Error, fs};
use reqwest::blocking::Client;

mod constantes; 
mod fusion;
fn main() -> Result<(), Box<dyn Error>> {
    let base_url = "https://api.brawlstars.com/v1/players/%23";
    
    for player in constantes::PLAYER.iter(){
        create_files(&player);
        get_player_battlelog(player, base_url)?;
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
fn get_player_battlelog(tag: &str, base_url: &str) -> Result<(), Box<dyn Error>> {
    let full_url = format!("{}{}/battlelog", base_url, tag);  
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


fn create_files(player_tag: &str)->(){
    let _ = fs::create_dir(format!("#{}",player_tag));
    let _ = fs::create_dir(format!("#{}/player",player_tag));
    let _ = fs::create_dir(format!("#{}/battle",player_tag));
    let _ = fs::create_dir(format!("#{}/club",player_tag));

}



fn merge_json(dir_path: &str) -> Result<(), std::io::Error> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.to_str() {
                        files.push(file_name.to_string());
                    }
                }
            }
        }
    } else {
        eprintln!("Error while reading '{}'", dir_path);
    }

    fusion::merge_files(files.iter().map(|s| s.as_str()).collect())
}

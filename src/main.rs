use std::{error::Error, fs::{self, File}, io::Write, thread::sleep, time::Duration};
use chrono::Local;
use reqwest::blocking::Client;

mod constantes; 
mod fusion;
fn main() -> Result<(), Box<dyn Error>> {
    let base_url = "https://api.brawlstars.com/v1/players/%23";
    
    for player in constantes::PLAYER.iter(){
        create_files(&player);
    }
    for i in 1..300_000{
        for player in constantes::PLAYER.iter(){
            let content = match get_player_battlelog(player, base_url) {
                Ok(content) => {
                    println!("{}", i);
                    content
                }
                Err(e) => {
                    eprintln!("Erreur lors de la requête à l'API pour le joueur {} : {}", player, e);
                    "null".to_string()
                }
            };
            write_into_file(format!("#{}/battle/{}",player,Local::now()).as_str(), content);
            let _ = merge_json(format!("#{}/battle/",player).as_str(),format!("#{}/battle/{}",player,Local::now()).as_str());
        }
        sleep(Duration::from_secs(120));
    }
    
    Ok(())
}

fn get_player_info(tag: &str, base_url: &str) -> Result<String, Box<dyn Error>> {
    let full_url = format!("{}{}", base_url, tag);
    
    let client = Client::new();
    
    let response = client
        .get(&full_url)
        .header("Authorization", format!("Bearer {}", constantes::TOKEN))
        .send()?;

    let body = response.text()?;
    
    Ok(body)
}
fn get_player_battlelog(tag: &str, base_url: &str) -> Result<String, Box<dyn Error>> {
    let full_url = format!("{}{}/battlelog", base_url, tag);
    
    let client = Client::new();
    
    let response = client
        .get(&full_url)
        .header("Authorization", format!("Bearer {}", constantes::TOKEN))
        .send()?;

    let body = response.text()?;
    
    Ok(body)
}


fn create_files(player_tag: &str)->(){
    let _ = fs::create_dir(format!("#{}",player_tag));
    let _ = fs::create_dir(format!("#{}/player",player_tag));
    let _ = fs::create_dir(format!("#{}/battle",player_tag));
    let _ = fs::create_dir(format!("#{}/club",player_tag));

}

fn write_into_file(file_path: &str,content:String) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}



fn merge_json(dir_path: &str,path_o: &str) -> Result<(), std::io::Error> {
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

    fusion::merge_files(files.iter().map(|s| s.as_str()).collect(),&path_o)
}


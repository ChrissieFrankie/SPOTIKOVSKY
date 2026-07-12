use std::io::{Write};

pub struct AuthManager
{
    pub user_id:String
}

impl AuthManager
{
    pub fn new()-> Self 
    {    
        println!("Load your Spotify profile, your unique id will be in the URL.");
        Self 
        {
            user_id: String::new()
        }
    }

    pub fn prompt_user_id()-> Self
    {
        print!("Enter your profile URL:");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read"); 
        Self{
            user_id: input.rsplit('/').last().unwrap_or("").to_string()
        }
    }
}
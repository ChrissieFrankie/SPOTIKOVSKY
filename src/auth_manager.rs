use std::io::{Write};

pub struct AuthManager // structure
{
    pub user_id:String
}

impl AuthManager // implementation
{
    pub fn new()-> Self // provide instructions
    {    
        println!("Load your Spotify profile, your unique id will be in the URL.");
        Self 
        {
            user_id: String::new()
        }
    }

    pub fn prompt_user_id(&mut self)// prompt for user id
    {
        print!("Enter your profile URL:");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read"); 
        self.user_id = input.split('/').last().unwrap_or("").to_string();
    }

    pub fn get_user_id(my_auth_manager: Self)-> String // return user id
    {
        my_auth_manager.user_id
    }
}
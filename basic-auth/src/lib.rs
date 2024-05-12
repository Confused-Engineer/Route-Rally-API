use std::io::{Error, Write};
use ini::Ini;
use rand::{distributions::Alphanumeric, Rng};


pub struct Config {
    file_location: String,
    auth: Ini,
    admin_tokens: Vec<String>,
    user_tokens: Vec<String>,

}

impl Default for Config {
    fn default() -> Self {
        Self {
            file_location: "/confusedengineer/conf/auth".to_string(),
            auth: Config::load_auth(),
            
            admin_tokens: Config::load_config_tokens("admins"),
            user_tokens: Config::load_config_tokens("users"),
        }
        
    }

    
}

impl Config {

    fn load_auth() -> Ini
    {
        
        let file = "/confusedengineer/conf/auth";
        use std::fs::File;
        let auth = Ini::load_from_file(file);
        if auth.is_err()
        {
            let auth_file = File::create(file);
            let _ = auth_file.unwrap().write_all(b"
[users]
            
[admins]
admin1=desired_admin_token_here
");
        }

        Ini::load_from_file(file).unwrap()
    }


    fn load_config_tokens(section: &str) -> Vec<String>
    {
        let mut tokens: Vec<String> = Vec::new();
        
        let binding = Config::load_auth();
        let auth = binding.section(Some(section)).unwrap();
        for (_key, value) in auth.iter(){
            if !value.is_empty()
            {
                
                tokens.push(value.to_string());
            }
        }
        //println!("{section}");
        //println!("{:?}", tokens);
        
        return tokens;
    }

    pub fn add_user(&mut self, admin_token: String, user: String) -> Result<String, Error>
    {
        let add_user_error = "Add User Error: ".to_string();
        

        for token in &self.admin_tokens
        {
            if token == &admin_token
            {

                let mut binding = self.auth.with_section(Some("users"));
                let user_exits = binding.get(&user);
                if user_exits.is_some()
                {
                    return Err(Error::new(std::io::ErrorKind::NotFound, add_user_error + "Error User Already Exists"));
                }

                let new_token: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(64)
                    .map(char::from)
                    .collect();

                self.auth.with_section(Some("users")).set(&user, &new_token);
                let user_add = self.auth.write_to_file(&self.file_location);
                if user_add.is_ok()
                {
                    return Ok(format!("{user} created: {new_token}"));
                } else {
                    return Err(Error::new(std::io::ErrorKind::NotFound, add_user_error + "Error Adding User"));
                }
                
            }
        }
        
        return Err(Error::new(std::io::ErrorKind::NotFound, add_user_error + "Admin Token Invalid"));
    }

    pub fn update_user(&mut self, admin_token: String, user: String) -> Result<String, Error>
    {
        
        let update_user_error: String = "Update User Error: ".to_string();

        for token in &self.admin_tokens
        {
            if token == &admin_token
            {

                let mut binding = self.auth.with_section(Some("users"));
                let user_exits = binding.get(&user);
                if user_exits.is_none()
                {
                    return Err(Error::new(std::io::ErrorKind::NotFound, update_user_error + "Error User Does Not Exist"));
                }

                let new_token: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(64)
                    .map(char::from)
                    .collect();

                self.auth.with_section(Some("users")).set(&user, &new_token);
                let user_add = self.auth.write_to_file(&self.file_location);
                if user_add.is_ok()
                {
                    return Ok(format!("{user} updated: {new_token}"));
                } else {
                    return Err(Error::new(std::io::ErrorKind::NotFound, update_user_error + "Error Saving User to File"));
                }
                
            }
        }
        
        return Err(Error::new(std::io::ErrorKind::NotFound, update_user_error + "Admin Token Invalid"));
    }

    pub fn remove_user(&mut self, admin_token: String, user: String) -> Result<String, Error>
    {
        let remove_user_error = "Remove User Error: ".to_string();
        

        for token in &self.admin_tokens
        {
            if token == &admin_token
            {

                let mut binding = self.auth.with_section(Some("users"));
                let user_exits = binding.get(&user);
                if user_exits.is_none()
                {
                    return Err(Error::new(std::io::ErrorKind::NotFound, remove_user_error + "Error User Does Not Exist"));
                }



                self.auth.with_section(Some("users")).delete(&user);
                let user_remove = self.auth.write_to_file(&self.file_location);
                if user_remove.is_ok()
                {
                    return Ok(format!("{user} deleted"));
                } else {
                    return Err(Error::new(std::io::ErrorKind::NotFound, remove_user_error + "Error Adding User"));
                }
                
            }
        }
        
        return Err(Error::new(std::io::ErrorKind::NotFound, remove_user_error + "Admin Token Invalid"));
    }

    pub fn validate_user(&self, user_token: String) -> bool
    {
        for token in &self.admin_tokens
        {
            if &user_token == token
            {
                return true;
            }

        }

        for token in &self.user_tokens
        {
            if &user_token == token
            {
                return true;
            }

        }

        return false;
    }

    pub fn list_user(&self, admin_token: String) -> Result<String, Error>
    {
        let list_user_error = "List User Error: ".to_string();
        let mut user_list = String::new();

        for token in &self.admin_tokens
        {
            if token == &admin_token
            {

                let binding = self.auth.section(Some("users"));
                for (key, _) in binding.unwrap()
                {
                    user_list = user_list.clone() + ", " + key;
                }

                return Ok(format!("Users: {}", user_list));

                
            }
        }
        
        return Err(Error::new(std::io::ErrorKind::NotFound, list_user_error + "Admin Token Invalid"));
    }
}







use dotenv::from_filename;
use std::env;

pub struct Environment;
impl Environment {
    fn load() {
        // determining .env type
        let env_file = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

        let filename = format!(".env.{}", env_file);
        from_filename(&filename).expect("Failed to load .env file");
    }

    pub fn get_appname() -> (String, String) {
        Self::load(); // ensure env loaded
        let app_name = env::var("APP_NAME").expect("Missing app_name in your .env file");
        let app_ver = env::var("APP_VER").expect("Missing app_ver in your .env file");
        (app_name, app_ver)
    }

    pub fn get_def_database(t: Option<String>) -> String {
        Self::load(); // ensure env loaded
        let db_type = t.unwrap_or_else(|| "default".to_string());

        let mut host = String::new();
        let mut user = String::new();
        let mut pass = String::new();
        let mut name = String::new();
        let mut port = String::new();

        // load database by type send
        if db_type == "default" {
            host = env::var("DEFAULT_HOST").expect("Missing default_host in your .env file");
            user = env::var("DEFAULT_USER").expect("Missing default_user in your .env file");
            pass = env::var("DEFAULT_PASS").expect("Missing default_pass in your .env file");
            name = env::var("DEFAULT_NAME").expect("Missing default_name in your .env file");
            port = env::var("DEFAULT_PORT").expect("Missing default_port in your .env file");
        }

        // creating database connection string
        format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, name)
    }
}
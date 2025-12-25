use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub storage_path: String,
    pub max_file_size: usize,
    pub default_expiry_seconds: i64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        Self {
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT должен быть числом"),
            
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:./data/lambdadrop.db".to_string()),
            
            storage_path: env::var("STORAGE_PATH")
                .unwrap_or_else(|_| "./storage".to_string()),
            
            max_file_size: env::var("MAX_FILE_SIZE_MB")
                .unwrap_or_else(|_| "100".to_string())
                .parse::<usize>()
                .expect("MAX_FILE_SIZE_MB должен быть числом") 
                * 1024 * 1024,
            
            default_expiry_seconds: env::var("DEFAULT_EXPIRY_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse::<i64>()
                .expect("DEFAULT_EXPIRY_HOURS должен быть числом") 
                * 3600,
        }
    }
}
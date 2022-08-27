use mongodb::{error::Error, options, Client};

pub async fn db_pool() -> Result<Client, Error> {
    dotenv::from_filename(".env").ok();

    let db = dotenv::var("MONGOURI").unwrap();

    let db_address = format!("{}", db);

    let mut client_option = options::ClientOptions::parse(&db_address).await?;
    client_option.retry_writes = Some(false);

    let client = Client::with_options(client_option);

    match client {
        Ok(c) => {
            println!("Connected to database");
            Ok(c)
        }
        Err(e) => Err(e),
    }
}

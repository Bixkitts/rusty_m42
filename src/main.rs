mod dispatcher;
mod communication;
//use communication::commune;
mod configuration;
use configuration::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new().unwrap(); 
    dispatcher::authoritate(&mut config).await?;

    // Placeholder strings, this will in future be called by the webserver instead with client supplied strings (uh oh)
    let data = dispatcher::get_people(&config, "Sean", "FirstName").await?;
    print!("{:#?}", data);

    Ok(())
}


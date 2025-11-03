use std::env;
use std::error::Error;
use reqwest;


fn main() -> Result<(), Box<dyn Error>> {
    let url = env::var("ANNOUNCEMENTS_HOOK")?;
    let client = reqwest::blocking::Client::new();
    let params = [("content", "test")];

    let res = client.post(url)
        .form(&params)
        .send()?;

    println!("{:#?}", res);
    Ok(())
}

use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error>{
    let client = Client::new();

    let user = "testuser".to_string();
    let passwd: Option<String> = None;

    let response = client
    .get("http://httpbin.org/get")
    .basic_auth(user, passwd)
    .send();

    println!("{:?}", response);

    Ok(())
}


// this program uses the knowledge of the previous excersices. We'll also make a request in this program, but this time it's all about authentication.
// we'll be calling a specific API (http://httpbin.org/#/Auth) and we want to get 'status 200'
// GET -> /basic-auth/{user}/{passwd} -> Prompts the user for authorization using HTTP Basic Auth.
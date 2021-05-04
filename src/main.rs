use nasa_apod::{Body, E};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = nasa_apod::construct_url();
    //requesting data
    let res = match reqwest::get(&url).await {
        Ok(response) => {
            if response.status() == 400 {
                nasa_apod::print_err(&response.json::<E>().await?.msg);
                std::process::exit(1);
            }
            //converting reqwest::Response to text
            response.text().await?
        }
        Err(e) => {
            if e.is_connect() {
                nasa_apod::print_err("Connect to internet.");
                std::process::exit(1);
            }
            if e.is_timeout() {
                nasa_apod::print_err("request timeout.")
            }
            std::process::exit(1);
        }
    };
    //converting text to vector of json objects
    let json_value = match serde_json::from_str::<Vec<Body>>(&res) {
        Ok(v) => v,
        //for single json object response
        Err(_) => vec![serde_json::from_str::<Body>(&res)?],
    };
    //displaying output
    nasa_apod::print_to_stdout(json_value);
    Ok(())
}

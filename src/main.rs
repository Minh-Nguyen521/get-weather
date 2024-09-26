use std::io;
use reqwest::Error;
use serde::Deserialize;

// Get your API key from https://home.openweathermap.org
// My API key doesn't work anymore so you have to get your own API key
const API_KEY: &str = "340baf8dd2a3be2840ac044394854931";

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

#[derive(Deserialize, Debug)]
struct Country {
    code: String,
    name: String,
}

async fn fetch_weather(city: &str, country_code: &str) -> Result<WeatherResponse, Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, API_KEY
    );
    let response = reqwest::get(&url).await?;
    let response_json = response.json::<WeatherResponse>().await?;
    Ok(response_json)
}

fn display_weather(response: &WeatherResponse) {
    println!("City: {}", response.name);
    println!("Temperature: {}°C", response.main.temp);
    println!("Humidity: {}%", response.main.humidity);
    println!("Wind Speed: {} m/s", response.wind.speed);
    println!("Description: {}", response.weather[0].description);
    println!("Pressure: {} hPa", response.main.pressure);
}

#[tokio::main] 
async fn main() {
    let mut city = String::new();
    let mut country_code = String::new();

    println!("Enter your city:");
    io::stdin().read_line(&mut city).expect("Failed to read input");

    println!("Enter your country code:");
    io::stdin().read_line(&mut country_code).expect("Failed to read input");

    let city = city.trim();
    let country_code = country_code.trim();

    let url = "https://countrycode.org/api/countryCode/countryMenu";
    let response = reqwest::get(url).await.expect("Failed to send request");
    let response_body = response.text().await.expect("Failed to read response body");

    let country_data: Vec<Country> = serde_json::from_str(&response_body).expect("Failed to parse JSON");
    let country = country_data.iter().find(|c| c.name == country_code).expect("Country code not found");

    match fetch_weather(city, &country.code).await {
        Ok(weather_response) => {
            display_weather(&weather_response);
        }
        Err(err) => {
            eprintln!("Error fetching weather: {}", err);
        }
    }
}

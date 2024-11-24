
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use sqlx::{Pool, Postgres, Row};
// use serde_json::Value;
use crate::configs::database::connect;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Antares {
    #[serde(rename = "m2m:cin")]
    m2m_cin: M2MCin,
}
#[derive(Deserialize, Debug)]
struct M2MCin {
    cnf: String,
    con: String,
}

#[derive(Debug)]
struct Setting {
    name: String,
    value: String
}

async fn connection() -> Result<Vec<Setting>, String> {
    let pool: Pool<Postgres> = match connect().await {
        Ok(resp) => resp,
        Err(_) => {
            return Err("Failed to connect to database".to_string());
        }
    };
    let rows = match sqlx::query("SELECT name,value FROM settings WHERE name = $1 or name = $2")
        .bind("ANTARES_URL")
        .bind("ANTARES_KEY")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return Err("Something wrong when getting data setting".to_string());
        }
    };

    let settings: Vec<Setting> = rows
        .into_iter()
        .map(|row| {
            let name: String = row.try_get("name").unwrap_or_else(|_| "-".to_string());
            let value: String = row.try_get("value").unwrap_or_else(|_| "-".to_string());
            Setting {name: name, value: value}
        })
        .collect();

    Ok(settings)
}

pub async fn parser(period: String) {
    // connect database to get data settings
    let settings = match connection().await {
        Ok(resp) => resp,
        Err(err) => {
            eprintln!("Something wrong, {}", err);
            return;
        }
    };

    // declaring static attributes for calling host
    let mut antares_url = String::new();
    let mut antares_key = String::new();
    for setting in settings {
        if setting.name == "ANTARES_URL" {
            antares_url = setting.value;
        } else if setting.name == "ANTARES_KEY" {
            antares_key = setting.value;
        }
    }
    let device_name: String = "07173-KTFLYTCS-6021090625".to_string(); // [static] device
    let full_url: String = format!("{}/{}/la", antares_url, device_name);

    // start calling host
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("X-M2M-Origin", HeaderValue::from_str(&antares_key).unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let response = match client.get(full_url).headers(headers).send().await {
        Ok(resp) => resp,
        Err(err) => {
            eprintln!("Failed to get request: {}", err);
            return;
        }
    };
    let result: Antares = match response.json().await {
        Ok(resp) => resp,
        Err(err) => {
            eprintln!("Failred to deserialize response: {}", err);
            return;
        }
    };

    // print out all responses
    println!("Hello this is from parser with period {}", period);
    println!(
        "Here the result from backend service as a CNF, {:?}",
        result.m2m_cin.cnf
    );
    println!(
        "Here the result from backend service as a CON, {:?}",
        result.m2m_cin.con
    );
}

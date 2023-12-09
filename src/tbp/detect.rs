use std::{env, f32::consts::E};
use serde::Deserialize;
use reqwest::{Client, header::AUTHORIZATION, Error};

const LANGUAGE_DETECTION_URL: &str =  "https://ws.detectlanguage.com/0.2/detect";
const DETECTION_CLIENT: Client = Client::new();

//{"data":{"detections":[{"language":"en","isReliable":true,"confidence":11.94}]}}

#[derive(Deserialize, Debug)]
struct LanguageDetection {
  language: String,
  isReliable: bool,
  confidence: f32,
}

#[derive(Deserialize, Debug)]
struct LanguageDetectionResponse {
  data: Vec<LanguageDetection>,
}

async fn detect_language(text: &str) -> Result<Vec<&str>, Vec<&str>> {
  let api_key = env::var("LANGUAGE_DETECTION_API_KEY")
    .unwrap_or_default();
  if api_key.len() == 0 {
    return Err(Vec::new());
  }
  let detection_request_url = format!("{}?q={}", LANGUAGE_DETECTION_URL, text);
  let response = match DETECTION_CLIENT
    .get(&detection_request_url)
    .header(AUTHORIZATION, format!("Bearer {}", api_key))
    .send()
    .await {
      Ok(response) => response.json::<LanguageDetectionResponse>(),
      Err(_) => return Err(Vec::new()),
    }
    .await
    .unwrap()
    .data;

  if response.len() == 0 {
    return Err(Vec::new());
  }

  let detected_languages = response
    .iter()
    .map(|detection| detection.language.as_str())
    .collect::<Vec<&str>>()
    .clone();

  return Ok(detected_languages);
}

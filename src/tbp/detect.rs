use std::{env, iter::Map};
use serde::Deserialize;
use reqwest::{Client, header::AUTHORIZATION};

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

async fn detect_language(text: &str) -> Result<Vec<&str>, &str> {
  let api_key = env::var("LANGUAGE_DETECTION_API_KEY")
    .unwrap_or_default();
  if api_key.len() == 0 {
    return Err("Please set the LANGUAGE_DETECTION_API_KEY environment variable");
  }
  let detection_request_url = format!("{}?q={}", LANGUAGE_DETECTION_URL, text);
  let response = DETECTION_CLIENT
    .get(&detection_request_url)
    .header(AUTHORIZATION, format!("Bearer {}", api_key))
    .send()
    .await?
    .json::<LanguageDetectionResponse>();

  let language_detection = response.await?;
  let detected_languages = language_detection.data
    .iter()
    .map(|detection| detection.language.as_str())
    .collect::<Vec<&str>>();
  if detected_languages.len() == 0 {
    println!("No languages detected");
    return Err("No languages detected");
  }
  return Ok(detected_languages);
}

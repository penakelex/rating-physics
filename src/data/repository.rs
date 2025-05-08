use std::sync::Arc;

use ciphered_file_type::CipheredFileType;
use error::GetRatingDataError;
use reqwest::{
    multipart::{Form, Part},
    Client,
};

use crate::domain::model::rating_data::RatingData;

pub mod error;
pub mod ciphered_file_type;

const BASE_URL: &str = "http://45.90.46.187:8000/rating_physics";

pub struct RatingRepository {
    client: Client,
}

impl RatingRepository {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            client: Client::new(),
        })
    }
}

impl RatingRepository {
    pub async fn get_latest_version(&self) -> Option<String> {
        let response = self
            .client
            .get(format!("{BASE_URL}/desktop/latest_version"))
            .send()
            .await
            .ok()?;

        if !response.status().is_success() {
            return None;
        }

        response.text().await.ok()
    }

    pub async fn get_rating_data(
        &self,
        password: u32,
        file_bytes: Vec<u8>,
        file_type: CipheredFileType,
    ) -> Result<RatingData, GetRatingDataError> {
        let response = self
            .client
            .post(format!("{BASE_URL}/file/decipher"))
            .multipart(
                Form::new()
                    .part(
                        "file",
                        Part::bytes(file_bytes)
                            .mime_str(file_type.as_mime_type_str())
                            .unwrap(),
                    )
                    .part(
                        "password",
                        Part::text(password.to_string())
                            .mime_str("application/json")
                            .unwrap(),
                    ),
            )
            .send()
            .await;

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    let data = response
                        .text()
                        .await
                        .map_err(|_| GetRatingDataError::InvalidRatingDataFormat)?;

                    serde_json::from_str(data.as_str())
                        .map_err(|_| GetRatingDataError::InvalidRatingDataFormat)
                } else {
                    Err(GetRatingDataError::InvalidPassword)
                }
            }

            Err(_) => Err(GetRatingDataError::CantAccesServer),
        }
    }
}
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_json::json;

use crate::{errors::ProdError, utils::env};

const API_ENDPOINT: &str = "https://llm.api.cloud.yandex.net/foundationModels/v1/completion";

pub struct LLM {
    api_key: String,
    folder_id: String,
    api_endpoint: String,
    system_prompt: String,
    temperature: f32,
    max_tokens: u32,
}

impl LLM {
    pub fn new(api_key: String, folder_id: String) -> Self {
        Self {
            api_key,
            folder_id,
            api_endpoint: API_ENDPOINT.to_string(),
            system_prompt:
            r#"
            Ты рабодаешь модератором рекламы для крупной компании. Твоя единственная задача - не пропускать рекламу с матерными словами. Но можешь пропускать небольшие оскорбления, если они являются шуткой.
            На вход тебе будет подаваться строчка в JSON формате:

            ```
            {
                "message": "ТЕКСТ"
            }
            ```

            Твоя задача - отправить в ответ валидный JSON в формате:
            {
                "blocked": (true|false),
                "message": "Если blocked = true, причина блокировки. Иначе, можешь не возвращать значние message"
            }

            Примеры:

              - Запрос:
                {
                    "message": "Привет, видел новую игру? Она просто огонь!"
                }
                Ответ:
                {
                    "blocked": false
                }

              - Запрос:
                {
                    "message": "Я люблю программировать на Rust!"
                }

                Ответ:
                {
                    "blocked": false
                }

              - Запрос:
                {
                    "message": "Когда кобель увидел суку, он заплакал"
                }

                Ответ:
                {
                    "blocked": false
                }

              - Запрос:
                {
                    "message": "Я твою маму трахал, сын шлюхи!"
                }

                Ответ:
                {
                    "blocked": true,
                    "message": "Матерное слово "трахал" может быть расценено как матерное."
                }


            "#.to_string(),
            temperature: 0.3,
            max_tokens: 500,
        }
    }

    pub fn new_from_env() -> Self {
        let api_key = env("YANDEX_API_KEY");
        let folder_id = env("YANDEX_FOLDER_ID");
        Self::new(api_key, folder_id)
    }

    pub async fn validate(&self, value: &str) -> Result<String, reqwest::Error> {
        let client = Client::new();
        let response = client
            .post(&self.api_endpoint)
            .header("Authorization", format!("Api-Key {}", self.api_key))
            .header("x-folder-id", &self.folder_id)
            .body(
                json!({
                    "modelUri": format!("gpt://{}/yandexgpt/rc", self.folder_id),
                    "completionOptions": {
                        "maxTokens": self.max_tokens,
                        "temperature": self.temperature,
                    },
                    "messages": [
                        {
                            "role": "system",
                            "text": self.system_prompt
                        },
                        {
                            "role": "user",
                            "text": format!("{{\"message\": {:?}}}", value),
                        }
                    ]
                })
                .to_string(),
            )
            .send()
            .await?;

        let response = response.text().await?;
        Ok(response)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Alternative {
    message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
struct Alternatives {
    alternatives: Vec<Alternative>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    result: Alternatives,
}

#[derive(Debug, Serialize, Deserialize)]
struct FinalResponse {
    blocked: bool,
    message: Option<String>,
}

pub async fn llm_validate(value: &str) -> Result<(), ProdError> {
    let llm = LLM::new_from_env();
    let response = llm
        .validate(value)
        .await
        .map_err(|e| ProdError::Forbidden(e.to_string()))?;

    let response = serde_json::from_str::<Response>(&response)
        .map_err(|e| ProdError::Forbidden(e.to_string()))?;

    response
        .result
        .alternatives
        .iter()
        .try_for_each(|alternative| {
            let text = alternative.message.text.trim_matches('`').trim();
            let message: FinalResponse =
                from_str(text).map_err(|e| ProdError::Forbidden(e.to_string()))?;

            if message.blocked {
                let error_message = message
                    .message
                    .unwrap_or_else(|| "Unknown error".to_string());
                return Err(ProdError::Forbidden(error_message));
            }

            Ok(())
        })?;

    Ok(())
}

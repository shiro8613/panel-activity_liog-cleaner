use reqwest::{Client, Error, multipart};
use serde::{Deserialize, Serialize};

pub struct Webhook {
    client: Client,
    url: String
}

#[derive(Serialize, Deserialize)]
struct SendPayload {
    username: String,
    content: String
}

impl Webhook {
    pub fn new(url :&str) -> Self {
        let client = Client::new();
        Self {
            client,
            url: url.to_string(),
        }
    }

    pub async fn send(&self, title :&str, content :Vec<u8>) -> Result<(), Error> {
        let payload = SendPayload {
            username: "Activity_log Backup".to_string(),
            content: format!("backup at {}", title)
        };
        let payload_json = serde_json::to_string(&payload).unwrap();
        let part_payload = multipart::Part::text(payload_json);
        let part = multipart::Part::bytes(content)
            .file_name(format!("{}.json", title))
            .mime_str("application/json").unwrap();
        let form = multipart::Form::new()
            .part("payload_json", part_payload)
            .part("files[0]", part);

        let res = self.client
            .post(self.url.as_str())
            .query(&[("wait", "true")])
            .multipart(form)
            .send()
            .await;

        match res {
            Ok(r) => {
                if r.status() == 200 {
                    println!("Send Discord");
                } else {
                    println!("error: {:?}", r.error_for_status())
                }
                Ok(())
            },
            Err(e) => Err(e),
        }
    }
}
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper_util::client::legacy::Client;
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerImage {
  #[serde(rename = "Id")]
  pub id: String,
  #[serde(rename = "RepoTags")]
  pub tags: Vec<String>,
}

pub struct DockerClient {
  client: Client<UnixConnector, Full<Bytes>>,
  socket: String,
}

impl DockerClient {
  pub fn new(socket: &str) -> Self {
    Self {
      client: Client::unix(),
      socket: socket.to_owned(),
    }
  }

  async fn get(&self, path: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let url = Uri::new(&self.socket, path).into();

    let mut response = self.client.get(url).await?;
    let mut result: String = String::new();
    while let Some(frame_result) = response.frame().await {
        let frame = frame_result?;
        if let Some(segment) = frame.data_ref() {
            result.push_str(std::str::from_utf8(segment).unwrap());
        }
    }
    Ok(result)
  }

  pub async fn get_images(&self) -> Result<Vec<DockerImage>, Box<dyn Error + Send + Sync>> {
    let result = self.get("/images/json").await?;
    let images: Vec<DockerImage> = serde_json::from_str(&result).expect("Failed to parse JSON");
    Ok(images)
  }
}

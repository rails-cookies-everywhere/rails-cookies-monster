use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::{Method, Request};
use hyper_util::client::legacy::Client;
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use serde::{Deserialize, Serialize};
use std::error::Error;

use urlencoding::encode;

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

  async fn post(&self, path: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let url: Uri = Uri::new(&self.socket, path).into();

    // let mut response = self.client.get(url).await?;
    let req: Request<Full<Bytes>> = Request::builder()
      .method(Method::POST)
      .uri(url)
      .body(Full::new(Bytes::new()))?;
    let mut response = self.client.request(req).await?;
    let mut result: String = String::new();
    while let Some(frame_result) = response.frame().await {
      let frame = frame_result?;
      if let Some(segment) = frame.data_ref() {
        result.push_str(std::str::from_utf8(segment).unwrap());
      }
    }
    println!("Response: {:?}", result);
    Ok(result)
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

  pub async fn build_image(&self, query: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let full_path = format!("/build?{}", query);

    let mut response = self.post(&full_path).await?;
    let mut result = String::new();
    while let Some(frame_result) = response.frame().await {
      let frame = frame_result?;
      if let Some(segment) = frame.data_ref() {
        result.push_str(std::str::from_utf8(segment).unwrap());
      }
    }
    Ok(())
  }
}

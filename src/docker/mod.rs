use lazy_static::lazy_static;
use std::io::Error;
use std::process::{Command, Output};
use std::sync::Arc;
use tokio::sync::Mutex;

mod client;
use client::DockerClient;

lazy_static! {
  pub static ref DOCKER_CLIENT: Arc<Mutex<DockerClient>> =
    Arc::new(Mutex::new(DockerClient::new("/var/run/docker.sock")));
}

pub async fn exists(image_tag: &str) -> bool {
  let docker = DOCKER_CLIENT.lock().await;
  let images = docker.get_images().await.unwrap();
  images
    .iter()
    .any(|image| image.tags.iter().any(|tag| tag == image_tag))
}

pub async fn build(rails_version_tag: &str) -> Result<Output, Error> {
  let image_tag = format!("rails:v{}", rails_version_tag);
  let vtag = format!("RAILS_VERSION_TAG={}", rails_version_tag);
  Command::new("docker")
    .arg("build")
    .args(&["-t", &image_tag])
    .args(&["--build-arg", &vtag])
    .arg(".")
    .output()
}

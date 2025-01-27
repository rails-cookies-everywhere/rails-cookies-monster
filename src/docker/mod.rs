use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use urlencoding::encode;

mod client;
use client::DockerClient;
use std::error::Error;

use bollard::image::ListImagesOptions;
use bollard::Docker;

lazy_static! {
  pub static ref DOCKER: Arc<Mutex<Docker>> =
    Arc::new(Mutex::new(Docker::connect_with_unix_defaults().unwrap()));
}

lazy_static! {
  pub static ref DOCKER_CLIENT: Arc<Mutex<DockerClient>> =
    Arc::new(Mutex::new(DockerClient::new("/var/run/docker.sock")));
}

pub async fn image_exists(image_tag: &str) -> bool {
  let image = format!("rails_cookies_everywhere={}", image_tag);
  let list_images_opts = ListImagesOptions::<String> {
    all: true,
    filters: HashMap::from([("label".to_string(), Vec::from([image]))]),
    ..Default::default()
  };
  !DOCKER
    .lock()
    .await
    .list_images(Some(list_images_opts))
    .await
    .unwrap()
    .is_empty()
}

pub async fn build(rails_version_tag: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
  let image_tag = format!("rails:v{}", rails_version_tag);
  let Ok(mut current_dir) = std::env::current_dir() else {
    return Err("Could not get current directory".into());
  };
  let dockerfile_path = encode(&current_dir.to_str().unwrap());

  let build_arg = format!("{{ \"RAILS_VERSION_TAG\": \"v{}\" }}", rails_version_tag);
  let build_arg = encode(&build_arg);
  let query = format!(
    "context={}&t={}&buildargs={}",
    dockerfile_path, image_tag, build_arg
  );

  let docker = DOCKER_CLIENT.lock().await;
  docker.build_image(&query).await
}

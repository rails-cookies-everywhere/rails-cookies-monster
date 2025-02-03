use std::collections::HashSet;
use std::sync::Arc;
use std::sync::OnceLock;

use dockworker::Docker;
use lazy_static::lazy_static;
use log::error;
use tokio::sync::Mutex;

pub(crate) mod build;

pub(crate) static IMAGES: OnceLock<HashSet<String>> = OnceLock::new();

lazy_static! {
  pub(crate) static ref DOCKER: Arc<Mutex<Docker>> =
    Arc::new(Mutex::new(Docker::connect_with_defaults().unwrap()));
}

pub(crate) async fn cache_images() {
  let mut images = list_images().await;
  images.sort();
  let Ok(_) = IMAGES.set(images.into_iter().collect()) else {
    error!("Error: Failed to cache available Docker images");
    return;
  };
}

async fn list_images() -> Vec<String> {
  DOCKER
    .lock()
    .await
    .images(true)
    .await
    .unwrap()
    .iter()
    .filter(|image| {
      image
        .RepoTags
        .iter()
        .any(|tag| tag.starts_with("rails-cookies-everywhere:"))
    })
    .flat_map(|image| image.RepoTags.iter().cloned())
    .collect()
}

pub(crate) fn image_exists(image_tag: &str) -> bool {
  let image_full_tag = if image_tag.starts_with("rails-cookies-everywhere:") {
    image_tag.to_string()
  } else {
    format!("rails-cookies-everywhere:{}", image_tag)
  };
  IMAGES.get().unwrap().contains(&image_full_tag)
}

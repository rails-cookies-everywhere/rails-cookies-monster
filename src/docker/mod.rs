use std::collections::HashSet;
use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::Arc;

use lazy_static::lazy_static;
use tokio::sync::Mutex;
use dockworker::Docker;
use dockworker::ContainerBuildOptions;
use log::{error, info, debug, trace};

pub(crate) mod build;

pub(crate) static IMAGES: OnceLock<HashSet<String>> = OnceLock::new();

lazy_static! {
  pub(crate) static ref DOCKER: Arc<Mutex<Docker>> =
    Arc::new(Mutex::new(Docker::connect_with_defaults().unwrap()));
}

pub(crate) async fn cache_images() {
  let mut images = list_images().await;
  images.sort();
  for image in &images {
    trace!("Found image: {}", image);
  }
  let Ok(_) =  IMAGES.set(images.into_iter().collect()) else {
    error!("Error: Failed to cache available Docker images");
    return;
  };
}

async fn list_images() -> Vec<String>{  
  DOCKER
    .lock()
    .await
    .images(true)
    .await
    .unwrap()
    .iter()
    .filter(|image| {
      image.RepoTags.iter().any(|tag| tag.starts_with("rails-cookies-everywhere:"))
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


pub async fn build_image(version: &str) -> Result<(), dockworker::response::Response> {
  let (options, tar_path) = if version == "rails-base" {
    let base_options = ContainerBuildOptions {
      dockerfile: "Dockerfile".into(),
      t: vec!["rails-cookies-everywhere:rails-base".to_string()],
      ..ContainerBuildOptions::default()
    };
    (base_options, "./rails-base.tar")
  } else {
    let version_options = ContainerBuildOptions {
      dockerfile: "Dockerfile".into(),
      t: vec![format!("rails-cookies-everywhere:rails-v{}", version)],
      buildargs: Some(HashMap::from([("RAILS_VERSION_TAG".to_owned(), version.to_owned())])),
      ..ContainerBuildOptions::default()
    };
    (version_options, "./rails-base.tar")
  };
  build::build(options, tar_path).await
}

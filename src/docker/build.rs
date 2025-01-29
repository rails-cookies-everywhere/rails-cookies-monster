use dockworker::response::Response;
use dockworker::ContainerBuildOptions;
use futures::stream::StreamExt;
use log::trace;
use std::path::Path;
use std::collections::HashMap;

use super::DOCKER;

pub(crate) async fn build(options: ContainerBuildOptions, tar_file: &str) -> Result<(), Response> {
  let mut stream = DOCKER.
    lock().
    await.
    build_image(options, Path::new(tar_file)).
    await.
    unwrap();

  while let Some(Ok(msg)) = stream.next().await {
    // trace!("{:?}", msg);
    if matches!(msg, Response::Error(_)) {
      return Err(msg);
    }
  }
  Ok(())
}

// static DOCKER_BASE: &[u8] = include_bytes!("../../rails-base.tar");
pub async fn build_base() -> Result<(), Response> {
  let image_tag= "rails-base";
  let options = ContainerBuildOptions {
    dockerfile: "Dockerfile".into(),
    t: vec!["rails-cookies-everywhere:rails-base".to_string()],
    ..ContainerBuildOptions::default()
  };
  build(options, "./rails-base.tar").await
}

// static DOCKER_VERSION: &[u8] = include_bytes!("../../rails-base.tar");
pub async fn build_version(version: &str) -> Result<(), Response> {
  let image_tag = format!("rails-v{}", version);
  let options = ContainerBuildOptions {
    dockerfile: "Dockerfile".into(),
    t: vec![format!("rails-cookies-everywhere:rails-v{}", version)],
    buildargs: Some(HashMap::from([("RAILS_VERSION_TAG".to_owned(), version.to_owned())])),
    ..ContainerBuildOptions::default()
  };
  build(options, "./rails-versions.tar").await
}

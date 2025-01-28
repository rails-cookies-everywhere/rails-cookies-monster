use dockworker::ContainerBuildOptions;
use futures::stream::StreamExt;
use std::path::Path;
use std::collections::HashMap;

use super::DOCKER;

async fn build(options: ContainerBuildOptions, tar_file: &str) -> Result<(), dockworker::errors::Error> {
  let mut stream = DOCKER.
    lock().
    await.
    build_image(options, Path::new(tar_file)).
    await.
    unwrap();

  while let Some(msg) = stream.next().await {
    if msg.is_err() {
      return Err(msg.unwrap_err());
    } /*else {
      println!("{:?}", msg);
    }*/
  }
  Ok(())
}

// static DOCKER_BASE: &[u8] = include_bytes!("../../rails-base.tar");

pub async fn build_base() -> Result<(), dockworker::errors::Error> {
  let options = ContainerBuildOptions {
    dockerfile: "Dockerfile".into(),
    t: vec!["rails-cookies-everywhere:rails-base".to_owned()],
    ..ContainerBuildOptions::default()
  };
  build(options, "./rails-base.tar").await
}

// static DOCKER_VERSION: &[u8] = include_bytes!("../../rails-base.tar");
pub async fn build_version(version: &str) -> Result<(), dockworker::errors::Error> {
  let options = ContainerBuildOptions {
    dockerfile: "Dockerfile".into(),
    t: vec![format!("rails-cookies-everywhere:rails-v{}", version)],
    buildargs: Some(HashMap::from([("RAILS_VERSION_TAG".to_owned(), version.to_owned())])),
    ..ContainerBuildOptions::default()
  };
  build(options, "./rails-versions.tar").await
}

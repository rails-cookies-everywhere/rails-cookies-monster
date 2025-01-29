use std::collections::HashMap;
use std::path::Path;
use dockworker::ContainerBuildOptions;
use dockworker::response::Response;
use futures::stream::StreamExt;

use log::trace;

use super::DOCKER;

pub(crate) async fn build(options: ContainerBuildOptions, tar_file: &Path) -> Result<(), Response> {
  let mut stream = DOCKER.
    lock().
    await.
    build_image(options, tar_file).
    await.
    unwrap();

  while let Some(Ok(msg)) = stream.next().await {
    trace!("{:?}", msg);
    if matches!(msg, Response::Error(_)) {
      return Err(msg);
    }
  }
  Ok(())
}

// // static DOCKER_BASE: &[u8] = include_bytes!("../../ruby-base.tar");
pub async fn base(base: &str) -> Result<(), dockworker::response::Response> {
  let args = [
    ("BASE_IMAGE_TAG".to_owned(), base.to_owned()),
  ];
  let options = ContainerBuildOptions {
    dockerfile: "Dockerfile".into(),
    t: vec![format!("rails-cookies-everywhere:ruby-base-{}", base)],
    buildargs: Some(HashMap::from(args)),
    q: true,
    ..ContainerBuildOptions::default()
  };
  let cargo_path = &std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_PATH not set");
  let cwd = Path::new(cargo_path);
  build(options, &cwd.join("ruby-base.tar")).await
}

// // static DOCKER_VERSION: &[u8] = include_bytes!("../../rails-version.tar");
pub async fn version(base: &str, version: &str, patch: &str) -> Result<(), dockworker::response::Response> { 
  let args = [
    ("BASE_IMAGE_TAG".to_owned(), base.to_owned()),
    ("RAILS_VERSION_TAG".to_owned(), version.to_owned()),
    ("RAILS_PATCH".to_owned(), patch.to_owned()),
  ];
  let options = ContainerBuildOptions {
    dockerfile: "Dockerfile".into(),
    t: vec![format!("rails-cookies-everywhere:rails-v{}", version)],
    buildargs: Some(HashMap::from(args)),
    q: true,
    ..ContainerBuildOptions::default()
  };
  let cargo_path = &std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_PATH not set");
  let cwd = Path::new(cargo_path);
  build(options, &cwd.join("rails-versions.tar")).await
}

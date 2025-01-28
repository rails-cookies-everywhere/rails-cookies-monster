use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;
use dockworker::Docker;

mod build;

lazy_static! {
  pub static ref DOCKER: Arc<Mutex<Docker>> =
    Arc::new(Mutex::new(Docker::connect_with_defaults().unwrap()));
}

pub async fn image_exists(image_tag: &str) -> bool {
  DOCKER.
    lock().
    await.
    images(true).
    await.
    unwrap().
    iter().
    any(|image| {
      image.RepoTags.iter().any(|tag| tag == image_tag)
    })
  //   for_each(|image| {
  //     println!("{:?}", image);
  //   });
  // false
  // let image = format!("rails-cookies-everywhere={}", image_tag);
  // let list_images_opts = ListImagesOptions::<String> {
  //   all: true,
  //   filters: HashMap::from([("label".to_string(), Vec::from([image]))]),
  //   ..Default::default()
  // };
  // !DOCKER
  //   .lock()
  //   .await
  //   .list_images(Some(list_images_opts))
  //   .await
  //   .unwrap()
  //   .is_empty()
}

pub async fn build(image_tag: &str) -> Result<(), dockworker::errors::Error> {
  if image_tag == "rails-base" {
    build::build_base().await
  } else {
    build::build_version(image_tag).await
  }
  // let image_tag = format!("rails:v{}", rails_version_tag);
  // let Ok(mut current_dir) = std::env::current_dir() else {
  //   return Err("Could not get current directory".into());
  // };
  // let dockerfile_path = encode(&current_dir.to_str().unwrap());

  // let build_arg = format!("{{ \"RAILS_VERSION_TAG\": \"v{}\" }}", rails_version_tag);
  // let build_arg = encode(&build_arg);
  // let query = format!(
  //   "context={}&t={}&buildargs={}",
  //   dockerfile_path, image_tag, build_arg
  // );

  // // let docker = DOCKER_CLIENT.lock().await;
  // // docker.build_image(&query).await
}

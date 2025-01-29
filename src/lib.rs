use std::collections::HashSet;
use dockworker::response::Response;
use docker::image_exists;
use futures::future::join_all;
use log::{error, info, debug, trace};

use semver::VersionReq;

pub mod docker;
pub mod rails;


/// A instance of Rails Cookies Monster tests.
/// 
/// * versions: The versions that will be checked during this run
#[derive(Default)]
pub struct RailsCookiesMonster {
  versions: HashSet<String>
}

impl RailsCookiesMonster {
  pub fn new() -> Self {
    Self::default()
  }

  /// Add version requirements to the instance.
  pub fn add_version_requirement(&mut self, rails_versions_requirements: &str) {
    info!("Adding version requirement: {}", rails_versions_requirements);

    let Ok(reqs) = VersionReq::parse(rails_versions_requirements) else {
      return error!("-> Error: Cannot parse version requirement: {}", rails_versions_requirements);
    };
    let add_versions = crate::rails::versions::match_versions(&reqs);
    info!("-> Added versions: {}", add_versions.join(", "));
    self.versions.extend(add_versions);
  }

  /// Returns a sorted vector of all Rails versions that this instance will check.
  /// 
  /// This method collects all the versions from the internal HashSet, sorts them,
  /// and returns them as a Vec<String>. The versions are sorted in ascending order
  /// according to their string representation.
  /// 
  /// # Returns
  /// 
  /// * `Vec<String>` - A sorted vector containing all Rails versions to be checked
  /// 
  /// # Examples
  /// 
  /// ```
  /// let mut monster = RailsCookiesMonster::new();
  /// monster.add_requirement(">=7.0");
  /// let versions = monster.versions();
  /// // Returns something like ["7.0.0", "7.0.1", "7.0.2", ...]
  /// assert_eq(versions[0], "7.0.0");
  /// ```
  pub fn versions(&self) -> Vec<String> {
    let mut versions: Vec<String> = self.versions.iter().cloned().collect();
    versions.sort();
    versions
  }

  async fn cache_available_images(&self) {
    if docker::IMAGES.get().is_none() {
      debug!("Caching available Docker images");
      docker::cache_images().await;
      debug!("-> Cached {} Docker images", docker::IMAGES.get().unwrap().len());
    } else {
      trace!("Docker images already cached");
    }
  }

  pub async fn build_base_image(&self) {
    self.cache_available_images().await;
    if image_exists("rails-base") {
      return trace!("Rails base image already built");
    }
    docker::build_image("rails-base")
      .await
      .expect("Could not build Rails base image");
  }

  pub async fn build_versions_images(&self) {
    self.cache_available_images().await;

    let missing_versions: Vec<String> = self
      .versions()
      .iter()
      .filter(|version| {
        let image_tag = format!("rails-v{}", version);
        !docker::image_exists(&image_tag)
      })
      .cloned()
      .collect();
    if missing_versions.is_empty() {
      return trace!("All Rails version images are already built");
    }
    info!("Building {} Rails version images", missing_versions.len());

    let tasks = missing_versions
      .iter()
      .cloned()
      .map(|missing_version| {
        tokio::spawn(async move {
          info!("Building Rails v{} image", missing_version);
          let task = docker::build_image(&missing_version).await;
          match &task {
            Ok(_) => trace!("-> Built Rails v{} image", missing_version),
            Err(Response::Error(e)) => {
              error!("-> Coult not build Rails v{} image: {}", missing_version, e.error)
            },
            Err(e) => {
              error!("-> Coult not build Rails v{} image: {:?}", missing_version, e)
            }
          }
          return task;
        })
      });
      join_all(tasks).await;
      // if image_exists(version) {
      //   trace!("Rails version image already built: {}", version);
      // } else {
      //   docker::build::build_version(version).await.expect("Could not build Rails version image");
      // }
  }

}

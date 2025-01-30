use std::collections::HashSet;
use itertools::Itertools;
use dockworker::response::Response;
use futures::future::join_all;
use log::{error, info, debug, trace};

use semver::VersionReq;

pub mod docker;
pub mod rails;
use rails::versions::RailsVersion;


/// A instance of Rails Cookies Monster tests.
/// 
/// * versions: The versions that will be checked during this run
#[derive(Default)]
pub struct RailsCookiesMonster {
  versions: HashSet<RailsVersion>
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
    
    self.versions.extend(add_versions);
  }

  pub fn ruby_versions(&self) -> Vec<String> {
    let mut ruby_versions: Vec<String> = self
      .versions
      .iter()
      .cloned()
      .map(|version| version.ruby.to_string())
      .collect();
    ruby_versions.sort();
    ruby_versions
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
  /// monster.add_version_requirement(">=7.0");
  /// let versions = monster.rails_versions();
  /// // Returns something like ["7.0.0", "7.0.1", "7.0.2", ...]
  /// assert_eq!(versions[0], "7.0.0");
  /// ```
  pub fn rails_versions(&self) -> Vec<(String, String, String)> {
    let mut rails_versions: Vec<(String, String, String)> = self
      .versions
      .iter()
      .cloned()
      .map(|version| {
        (
          version.ruby.to_owned(),
          version.rails.to_string(),
          version.patch.to_owned()
        )
      })
      .collect();
    rails_versions.sort();
    rails_versions
  } 

  async fn cache_available_images(&self) {
    if docker::IMAGES.get().is_none() {
      debug!("Caching list of available Docker images");
      docker::cache_images().await;
      debug!("-> Cached list of {} Docker images", docker::IMAGES.get().unwrap().len());
    } else {
      trace!("Docker images list already cached");
    }
  }

  pub async fn build_base_image(&self) -> Result<(), Vec<(String, String)>> {
    self.cache_available_images().await;

    let missing_bases: Vec<String> = self
      .ruby_versions()
      .iter()
      .unique()
      .filter(|version| {
        let image_tag = format!("ruby-base-{}", version);
        !docker::image_exists(&image_tag)
      })
      .cloned()
      .collect();
    if missing_bases.is_empty() {
      trace!("All Ruby base images are already built!");
      return Ok(());
    }

    info!("Building {} Ruby version images", missing_bases.len());
    let tasks = missing_bases
      .iter()
      .cloned()
      .map(|missing_base| {
        tokio::spawn(async move {
          info!("Building ruby-{} image", missing_base);
          let task = docker::build::base(&missing_base).await;
          match &task {
            Ok(_) => Ok(()),
            Err(error) => Err((missing_base, error.clone()))
          }
        })
      });
    
    let results = join_all(tasks).await;
    let errors: Vec<_> = results
      .into_iter()
      .filter_map(|result| {
        match result {
          Ok(Err(e)) => Some(e),
          _ => None,
        }
      })
      .collect();

    if !errors.is_empty() {
      Err(errors)
    } else {
      Ok(())
    }
  }




  pub async fn build_versions_images(&self) -> Result<(), Vec<(String, String)>> {
    self.cache_available_images().await;

    let missing_versions: Vec<(String, String, String)> = self
      .rails_versions()
      .iter()
      .filter(|(_, rails_version, _)| {
        let image_tag = format!("rails-v{}", rails_version);
        !docker::image_exists(&image_tag)
      })
      .cloned()
      .collect();
    if missing_versions.is_empty() {
      trace!("All Rails version images are already built");
      return Ok(());
    }
    info!("Building {} Rails version images", missing_versions.len());

    let tasks = missing_versions
      .iter()
      .cloned()
      .map(|(ruby_version, rails_version, patch)| {
        tokio::spawn(async move {
          info!("Building Rails v{} image", rails_version);
          let task = docker::build::version(&ruby_version, &rails_version, &patch).await;
          match &task {
            Ok(_) => Ok(()),
            Err(error) => Err((rails_version, error.clone()))
          }
        })
      });

    let results = join_all(tasks).await;
    let errors: Vec<_> = results
      .into_iter()
      .filter_map(|result| {
        match result {
          Ok(Err(e)) => Some(e),
          _ => None,
        }
      })
      .collect();

    if !errors.is_empty() {
      Err(errors)
    } else {
      Ok(())
    }
  }

}

use std::collections::HashSet;
use itertools::Itertools;
use futures::future::join_all;
use log::{error, info, debug, trace};

use lazy_static::lazy_static;
use semver::VersionReq;
use dockworker::ContainerCreateOptions;
use dockworker::ContainerHostConfig;
use dockworker::ExposedPorts;
use dockworker::PortBindings;

pub mod docker;
pub mod rails;
use docker::image_exists;
use rails::versions::RailsVersion;

lazy_static! {
  #[derive(Debug)]
  pub(crate) static ref SECRET_KEY_BASE: String = match std::env::var("SECRET_KEY_BASE") {
    Ok(value) => value,
    Err(_) => {
      std::env::set_var("SECRET_KEY_BASE", "rails-cookies-everywhere");
      "rails-cookies-everywhere".to_string()
    }
  };
}

lazy_static! {
  #[derive(Debug)]
  pub(crate) static ref CANARY_VALUE: String = match std::env::var("CANARY_VALUE") {
    Ok(value) => value,
    Err(_) => {
      std::env::set_var("CANARY_VALUE", "correct-horse-battery-staple");
      "correct-horse-battery-staple".to_string()
    }
  };
}


/// A instance of Rails Cookies Monster tests.
/// 
/// * versions: The versions that will be checked during this run
#[derive(Default)]
pub struct RailsCookiesMonster {
  versions: HashSet<RailsVersion>,
  containers: HashSet<(String, String)>
}

impl RailsCookiesMonster {
  pub fn new() -> Self {
    debug!("Initialization:");
    debug!("- Using SECRET_KEY_BASE: {}", crate::SECRET_KEY_BASE.to_string());
    debug!("- Using CANARY_VALUE: {}", crate::CANARY_VALUE.to_string());

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
  /// * `Vec<String>` - A sorted vector containing all Rails versions to be checked
  /// 
  /// # Examples
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
      .filter(|version| !image_exists(&format!("ruby-base-{}", version)))
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
      .filter(|(_, rails_version, _)| !image_exists(&format!("rails-v{}", rails_version)))
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

  pub async fn start_containers(&mut self) {
    let mut versions_list: Vec<_> = self
      .rails_versions()
      .iter()
      .map(|(_, rails_version, _)| rails_version)
      .cloned()
      .collect();
    versions_list.sort();
    let ids = versions_list.iter()
      .cloned()
      .enumerate()
      .map(|(i, rails_version)| {
        tokio::spawn(async move {
          let image_tag = format!("rails-cookies-everywhere:rails-v{}", rails_version);
          let mut host_config = ContainerHostConfig::new();
          host_config.port_bindings(PortBindings(vec![(3000, "tcp".to_string(), 3000 + i as u16)]));
          let mut options = ContainerCreateOptions::new(&image_tag);
          options.env(format!("SECRET_KEY_BASE={}", SECRET_KEY_BASE.to_string()))
            .env(format!("CANARY_VALUE={}", CANARY_VALUE.to_string()))
            .exposed_ports(ExposedPorts(vec![(3000, "tcp".to_string())]))
            .host_config(host_config);
          
          let container_tag = format!("rails-cookies-everywhere-rails-v{}", rails_version);
          let container = docker::DOCKER.lock()
            .await
            .create_container(Some(&container_tag), &options)
            .await
            .unwrap();
          docker::DOCKER.lock()
            .await
            .start_container(&container.id)
            .await
            .unwrap();
          return (rails_version, container.id);
        })
      });

    let results = join_all(ids).await;
    debug!("Started {} containers", results.len());
    results.iter()
      .filter_map(|r| r.as_ref().ok())
      .for_each(|(rails_version, container_id)| {
        debug!("- Container for {}: {}", &rails_version, &container_id);
        self.containers.insert((rails_version.to_owned(), container_id.to_owned()));
      });
    ()
  }

  pub async fn stop_containers(&self) {
    let containers = self.containers
      .iter()
      .map(|(_, container_id)| container_id)
      .cloned()
      .collect();
    RailsCookiesMonster::drop_containers(containers).await;
  }

  pub async fn drop_containers(containers: Vec<String>) {
    trace!("Dropping {} containers", containers.len());
    let tasks = containers.iter()
      .map(|container_id| {
        let id_to_kill = container_id.clone();
        tokio::spawn(async move {
          // Do we really need to stop it if we remove it right after?
          // docker::DOCKER.lock()
          //   .await
          //   .stop_container(&id_to_kill, Duration::from_secs(1))
          //   .await
          //   .unwrap();
          // trace!("Stopped container {}", id_to_kill);
          docker::DOCKER.lock()
            .await
            .remove_container(&id_to_kill, Some(true), Some(true), None)
            .await
            .unwrap();
          trace!("- Removed container: {}", id_to_kill);
        })
      });
    let _ = join_all(tasks).await;
  }

}

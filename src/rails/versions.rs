use lazy_static::lazy_static;
use semver::Version;
use semver::VersionReq;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct RailsVersion {
  pub ruby: String,
  pub rails: Version,
  pub patch: String,
}

impl RailsVersion {
  fn new(ruby: &str, major: u64, minor: u64, patch: u64, patchfile: &str) -> Self {
    Self {
      ruby: ruby.to_string(),
      rails: Version::new(major, minor, patch),
      patch: patchfile.to_string(),
    }
  }
}

lazy_static! {
  /**
   * I'd appreciate if:
   * - 1. someone could help me with this list.
   * - 2. the latest compatible ruby version was used for each rails version.
   */
  pub static ref RAILS_VERSIONS: Vec<RailsVersion> = Vec::from([
    // // Rails 6.0.0 to 6.1.7
    // Not sure what makes them work yet.
    // RailsVersion::new("2.6.10", 6, 0, 0, "none"),
    // RailsVersion::new("3.0.7", 6, 0, 1, "none"),
    // RailsVersion::new("3.0.7", 6, 0, 2, "none"),
    // RailsVersion::new("3.0.7", 6, 0, 3, "none"),
    // RailsVersion::new("3.0.7", 6, 0, 4, "none"),
    // RailsVersion::new("3.0.7", 6, 0, 5, "none"),
    // RailsVersion::new("3.0.7", 6, 0, 6, "none"),
    // RailsVersion::new("3.0.7", 6, 1, 0, "none"),
    // RailsVersion::new("3.0.7", 6, 1, 1, "none"),
    // RailsVersion::new("3.0.7", 6, 1, 2, "none"),
    // RailsVersion::new("3.0.7", 6, 1, 3, "none"),
    // RailsVersion::new("3.0.7", 6, 1, 4, "none"),
    // RailsVersion::new("3.0.7", 6, 1, 5, "none"),
    // RailsVersion::new("3.0.7", 6, 1, 6, "none"),
    // RailsVersion::new("3.0.7", 6, 1, 7, "none"),


    // Rails 7.0.0
    // Does not work on 3.1.0 for some strange reasons.
    RailsVersion::new("3.0.7", 7, 0, 0, "7.0.x"),

    // Rails 7.0.1 to 7.0.8
    // An actual pain in the ass to diagnostic, but it seems a simple require does
    // the trick. Also: careful of the "force_ssl: true" in production.
    RailsVersion::new("3.3.7", 7, 0, 1, "7.0.x"),
    RailsVersion::new("3.3.7", 7, 0, 2, "7.0.x"),
    RailsVersion::new("3.3.7", 7, 0, 3, "7.0.x"),
    RailsVersion::new("3.3.7", 7, 0, 4, "7.0.x"),
    RailsVersion::new("3.3.7", 7, 0, 5, "7.0.x"),
    RailsVersion::new("3.3.7", 7, 0, 6, "7.0.x"),
    RailsVersion::new("3.3.7", 7, 0, 7, "7.0.x"),
    RailsVersion::new("3.3.7", 7, 0, 8, "7.0.x"),

    // Rails 7.1.0 to 7.2.2
    RailsVersion::new("latest", 7, 1, 0, "none"),
    RailsVersion::new("latest", 7, 1, 1, "none"),
    RailsVersion::new("latest", 7, 1, 2, "none"),
    RailsVersion::new("latest", 7, 1, 3, "none"),
    RailsVersion::new("latest", 7, 1, 4, "none"),
    RailsVersion::new("latest", 7, 1, 5, "none"),
    RailsVersion::new("latest", 7, 2, 0, "none"),
    RailsVersion::new("latest", 7, 2, 1, "none"),
    RailsVersion::new("latest", 7, 2, 2, "none"),
    // Rails 8.0.0 to 8.0.1
    RailsVersion::new("latest", 8, 0, 0, "none"),
    RailsVersion::new("latest", 8, 0, 1, "none"),
  ]);
}

pub fn match_versions(requirement: &VersionReq) -> Vec<RailsVersion> {
  RAILS_VERSIONS
    .iter()
    .filter(|version| requirement.matches(&version.rails))
    .cloned()
    .collect()
}

use rayon::prelude::*;
use semver::VersionReq;
use semver::Version;

const RAILS_VERSIONS: [Version; 20] = [
  // Rails 7.0.0 to 7.2.2
  Version::new(7, 0, 0),
  Version::new(7, 0, 1),
  Version::new(7, 0, 2),
  Version::new(7, 0, 3),
  Version::new(7, 0, 4),
  Version::new(7, 0, 5),
  Version::new(7, 0, 6),
  Version::new(7, 0, 7),
  Version::new(7, 0, 8),
  Version::new(7, 1, 0),
  Version::new(7, 1, 1),
  Version::new(7, 1, 2),
  Version::new(7, 1, 3),
  Version::new(7, 1, 4),
  Version::new(7, 1, 5),
  Version::new(7, 2, 0),
  Version::new(7, 2, 1),
  Version::new(7, 2, 2),
  // Rails 8.0.0 to 8.0.1
  Version::new(8, 0, 0),
  Version::new(8, 0, 1),
];

pub fn match_versions(requirement: &VersionReq) -> Vec<String> {
  RAILS_VERSIONS.
    par_iter().
    filter(|version| requirement.matches(version)).
    map(|version| version.to_string()).
    collect()
}

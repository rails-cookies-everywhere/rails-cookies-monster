use rayon::prelude::*;
use semver::VersionReq;
use semver::Version;

#[derive(Debug)]
struct RailsVersion<'v> {
  pub(crate) ruby: &'v str,
  pub(crate) rails: Version
}

static RAILS_VERSIONS: [RailsVersion; 20] = [
  // Rails 7.0.0 to 7.0.8
  // It seems there's been an update of a minor gem version somewhere, mixed with
  // a ruby version, so all these versions require either:
  // - A ruby version inferior or equal to 3.1.2
  // - `gem 'concurrent-ruby', '1.3.4'
  RailsVersion{ ruby: "3.1.2", rails: Version::new(7, 0, 0) },
  RailsVersion{ ruby: "3.1.2", rails: Version::new(7, 0, 1) },
  RailsVersion{ ruby: "3.1.2", rails: Version::new(7, 0, 2) },
  RailsVersion{ ruby: "3.1.2", rails: Version::new(7, 0, 3) },
  RailsVersion{ ruby: "3.1.2", rails: Version::new(7, 0, 4) },
  RailsVersion{ ruby: "3.1.2", rails: Version::new(7, 0, 5) },
  RailsVersion{ ruby: "3.1.2", rails: Version::new(7, 0, 6) },
  RailsVersion{ ruby: "3.1.2", rails: Version::new(7, 0, 7) },
  RailsVersion{ ruby: "3.1.2", rails: Version::new(7, 0, 8) },
  // Rails 7.1.0 to 7.2.2
  RailsVersion{ ruby: "latest", rails: Version::new(7, 1, 0) },
  RailsVersion{ ruby: "latest", rails: Version::new(7, 1, 1) },
  RailsVersion{ ruby: "latest", rails: Version::new(7, 1, 2) },
  RailsVersion{ ruby: "latest", rails: Version::new(7, 1, 3) },
  RailsVersion{ ruby: "latest", rails: Version::new(7, 1, 4) },
  RailsVersion{ ruby: "latest", rails: Version::new(7, 1, 5) },
  RailsVersion{ ruby: "latest", rails: Version::new(7, 2, 0) },
  RailsVersion{ ruby: "latest", rails: Version::new(7, 2, 1) },
  RailsVersion{ ruby: "latest", rails: Version::new(7, 2, 2) },
  // Rails 8.0.0 to 8.0.1
  RailsVersion{ ruby: "latest", rails: Version::new(8, 0, 0) },
  RailsVersion{ ruby: "latest", rails: Version::new(8, 0, 1) },
];

pub fn match_versions(requirement: &VersionReq) -> Vec<String> {
  RAILS_VERSIONS.
    par_iter().
    filter(|version| requirement.matches(&version.rails)).
    map(|version| version.rails.to_string()).
    collect()
}

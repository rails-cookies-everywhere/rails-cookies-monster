use std::io::Error;
use std::process::{Command, Output};

pub async fn build(rails_version_tag: &str) -> Result<Output, Error> {
  // Build the Docker image
  let image_tag = format!("rails:v{}", rails_version_tag);
  println!("Building Docker image: {}", image_tag);
  let build_arg = format!("RAILS_VERSION_TAG={}", rails_version_tag);
  Command::new("docker")
    .args(&["build", "-t", &image_tag, "--build-arg", &build_arg, "."])
    .output()
}

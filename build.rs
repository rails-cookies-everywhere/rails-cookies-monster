use std::fs::File;
use std::path::Path;
use tar::Builder;

fn main() {
  let cargo_path = &std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_PATH not set");
  let cwd = Path::new(cargo_path);
  println!("CWD _{}_", cwd.display());
  println!("cargo:rerun-if-changed={}/build.rs", cwd.display());

  // Create ruby-base.tar from docker/base
  println!("cargo:rerun-if-changed={}/docker/base/Dockerfile", cwd.display());
  println!("Building ruby-base.tar");
  let base_tar = File::create(cwd.join("ruby-base.tar")).unwrap();
  let mut base_arc = Builder::new(base_tar);
  base_arc
    .append_path_with_name(
  cwd.join("docker/base/Dockerfile"),
  Path::new("Dockerfile"),
    )
    .unwrap();
  base_arc.finish().unwrap();

  // Create rails-versions.tar from docker/rails
  println!("cargo:rerun-if-changed={}/build.rs", cwd.display());
  println!("cargo:rerun-if-changed={}/docker/rails/Dockerfile", cwd.display());
  println!("cargo:rerun-if-changed={}/docker/rails/rails-7.0.x.patch", cwd.display());
  println!("cargo:rerun-if-changed={}/docker/rails/rails_patch", cwd.display());
  println!("Building rails-versions.tar");
  let versions_tar = File::create(cwd.join("rails-versions.tar")).unwrap();
  let mut versions_arc = Builder::new(versions_tar);
  versions_arc
    .append_path_with_name(
      cwd.join("docker/rails/Dockerfile"),
      Path::new("Dockerfile"),
    )
    .unwrap();
  versions_arc
    .append_path_with_name(
      cwd.join("docker/rails/rails-7.0.x.patch"),
      Path::new("rails-7.0.x.patch"),
    )
    .unwrap();
  versions_arc
    .append_dir_all(
      Path::new("rails_patch"),
      cwd.join("docker/rails/rails_patch"),
    )
    .unwrap();
  versions_arc.finish().unwrap();
}

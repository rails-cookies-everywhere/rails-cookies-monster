use bollard::Docker;

const BASE_DOCKERFILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Dockerfile.base"));
const RAILS_DOCKERFILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Dockerfile"));

pub async fn build_base() -> Result<(), Box<dyn std::error::Error>> {
    // let mut builder = Docker::Builder::new();
    // builder.dockerfile(BASE_DOCKERFILE);
    // let image = builder.build("rails-cookies-everywhere:rails-base").await?;
    Ok(())
}

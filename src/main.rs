use std::env;
use std::process::Command;
use tokio::task;
use tokio::time::Duration;

mod docker;
// mod rails;

#[tokio::main]
async fn main() {
  // Extract RAILS_VERSION_TAG from the first argument
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("Usage: {} <RAILS_VERSION_TAG>", args[0]);
    std::process::exit(1);
  }
  let rails_version_tag = &args[1];
  let image_tag = format!("rails:v{}", rails_version_tag);

  println!("Containers?");
  if docker::exists(&image_tag).await {
    println!("Container already exists");
  } else {
    println!("Container does not exist");
    if let Err(err) = docker::build(rails_version_tag).await {
      eprintln!("Docker: Failed to build rails:v{}",rails_version_tag);
      eprintln!("- Full error:\n {:?}",err);
      std::process::exit(1);
    }  
  }
  println!("Containers!");


  

  // Run the Docker container in a separate thread
  let container_id = task::spawn_blocking(move || {
    let run_output = Command::new("docker")
      .args(&["run", "-d", "-p", "3000:3000", &image_tag])
      .output()
      .expect("Failed to run Docker container");
    let container_id = String::from_utf8(run_output.stdout)
      .expect("Failed to parse Docker run output")
      .trim()
      .to_string();
    println!("Docker run container ID: {:?}", container_id);
    container_id
  })
  .await
  .expect("Failed to spawn Docker run task");

  // Query the running container in the main thread
  tokio::time::sleep(Duration::from_secs(5)).await;  // Wait for the container to be fully up
  let query_output = Command::new("curl")
    .arg("http://localhost:3000")
    .arg("-v")
    .output()
    .expect("Failed to query Docker container");
  println!("Curl output: {:?}", String::from_utf8(query_output.stderr).expect("Failed to parse Curl output"));

  // Stop and remove the Docker container
  let stop_output = Command::new("docker")
    .args(&["stop", &container_id])
    .output()
    .expect("Failed to stop Docker container");
  println!("Docker stop output: {:?}", stop_output);

  let rm_output = Command::new("docker")
    .args(&["rm", &container_id])
    .output()
    .expect("Failed to remove Docker container");
  println!("Docker rm output: {:?}", rm_output);

}

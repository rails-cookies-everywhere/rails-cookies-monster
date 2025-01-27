use lazy_static::lazy_static;
use std::env;
use std::process::Command;
use tokio::task;
use tokio::time::Duration;
use urlencoding::decode;

mod docker;
mod rails;

lazy_static! {
  pub(crate) static ref SECRET_KEY_BASE: String = match std::env::var("SECRET_KEY_BASE") {
    Ok(value) => value,
    Err(_) => {
      std::env::set_var("SECRET_KEY_BASE", "rails-cookies-everywhere");
      "rails-cookies-everywhere".to_string()
    }
  };
}

lazy_static! {
  pub(crate) static ref CANARY_VALUE: String = match std::env::var("CANARY_VALUE") {
    Ok(value) => value,
    Err(_) => {
      std::env::set_var("CANARY_VALUE", "correct-horse-battery-staple");
      "correct-horse-battery-staple".to_string()
    }
  };
}

#[tokio::main]
async fn main() {
  // Extract RAILS_VERSION_TAG from the first argument
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("Usage: {} <RAILS_VERSION_TAG>", args[0]);
    std::process::exit(1);
  }

  println!("********************************************************************************");
  println!("***** 0. Setup *****************************************************************");
  println!(
    "-> SECRET: {:?}\n-> CANARY: {:?}",
    *SECRET_KEY_BASE, *CANARY_VALUE
  );
  let rails_version_tag = &args[1];
  let image_tag = format!("rails:v{}", rails_version_tag);
  println!(
    "-> VERSION: {:?}\n-> IMAGE: {:?}",
    rails_version_tag, image_tag
  );

  println!("\n********************************************************************************");
  println!("***** 1. Set up Docker image ***************************************************");
  if docker::exists(&image_tag).await {
    println!("-> Image already exists, skipping.");
  } else {
    println!("-> Image does not exist, building.");
    if let Err(build_output) = docker::build(rails_version_tag).await {
      eprintln!("Docker: Failed to build rails:v{}", rails_version_tag);
      eprintln!("- Full error:\n {:?}", build_output);
      std::process::exit(1);
    } else {
      println!("-> Docker: Built rails:v{}", rails_version_tag);
    }
  }

  println!("\n********************************************************************************");
  println!("***** 2. Running server container **********************************************");
  // Run the Docker container in a separate thread
  let container_id = task::spawn_blocking(move || {
    let secret = &format!("SECRET_KEY_BASE={}", &SECRET_KEY_BASE.to_string());
    let canary = &format!("CANARY_VALUE={}", &CANARY_VALUE.to_string());
    let run_output = Command::new("docker")
      .arg("run")
      .arg("-d")
      .args(&["-e", secret])
      .args(&["-e", canary])
      .args(&["-p", "3000:3000"])
      .arg(&image_tag)
      .output()
      .expect("Failed to run Docker container");
    let container_id = String::from_utf8(run_output.stdout)
      .expect("Failed to parse Docker run output")
      .trim()
      .to_string();
    println!("-> Docker run container ID: {:?}", container_id);
    container_id
  })
  .await
  .expect("Failed to spawn Docker run task");

  println!("\n********************************************************************************");
  println!("***** 3. Querying server container *********************************************");
  // Query the running container in the main thread
  tokio::time::sleep(Duration::from_secs(5)).await; // Wait for the container to be fully up
  let query_output = Command::new("curl")
    .arg("http://localhost:3000")
    .arg("-v")
    .output()
    .expect("Failed to query Docker container");
  let output = String::from_utf8(query_output.stderr).expect("Failed to parse Curl output");

  let cookies: Vec<(String, String)> = output
    .lines()
    .filter(|line| line.starts_with("< set-cookie:"))
    .map(|line| {
      return line
        .strip_prefix("< set-cookie: ")
        .unwrap()
        .split_once(';')
        .unwrap()
        .0
        .split_once('=')
        .unwrap();
    })
    .map(|(key, val)| return (key.to_owned(), decode(val).unwrap().to_string()))
    .collect();
  println!("-> Extracted {} cookies", cookies.len());
  
  println!("\n********************************************************************************");
  println!("***** 4. Cleaning up container *************************************************");
  let _stop_output = Command::new("docker")
    .arg("stop")
    .arg(&container_id)
    .output()
    .expect("Failed to stop Docker container");

  let _rm_output = Command::new("docker")
    .args(&["rm", &container_id])
    .output()
    .expect("Failed to remove Docker container");

  println!("\n********************************************************************************");
  println!("***** 5. Checking cookies decyphering ******************************************");
  cookies.
    iter().
    // filter(|(key, _)| *key != "regular" && *key != "signed" ).//"_cookie_monster_session" ).
    for_each(|(key, value)| {
      if *key == "signed" { return; }
      if *key == "regular"{
        println!("Cookie:\n -> KEY: {}\n -> VAL: {}", key, value);
        return;
      }
      println!("Cookie:\n -> KEY: {}", key);
      let deciphered = rails::decipher_cookie(rails_version_tag, &value).expect("Could not decipher cookie");
      println!(" -> VAL: {}", deciphered);
      if deciphered.contains(&CANARY_VALUE.to_string()) {
        println!(" -> Decyphering is correct!")
      } else {
        println!(" -> Decyphering is incorrect!")
      }
    });
}

use std::env;
use std::process::Command;
use tokio::task;

use rails_cookies_monster::RailsCookiesMonster;

#[tokio::main]
async fn main() {
  // Extract RAILS_VERSION_TAG from the first argument
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("Usage: {} <RAILS_VERSION_TAG>", args[0]);
    std::process::exit(1);
  }
  env_logger::init();

  // Set up Monster
  let mut monster = RailsCookiesMonster::new();
  monster.add_version_requirement(&args[1]);
  if monster.ruby_versions().is_empty() {
    eprintln!("Error: No version matching requirement {}", args[1]);
    std::process::exit(1);
  }

  // Set up images
  if let Err(errors) = monster.build_base_image().await {
    eprintln!("Failed to build {} ruby images", errors.len());
    for (rubyver, errror) in errors {
      eprintln!("- Failed to build image ruby-{}: {:?}", rubyver, errror);
    }
    eprintln!("Exiting...");
    std::process::exit(1);
  }
  if let Err(errors) = monster.build_versions_images().await {
    eprintln!("Failed to build {} rails images", errors.len());
    for (railsver, errror) in errors {
      eprintln!("- Failed to build image rails-v{}: {:?}", railsver, errror);
    }
    eprintln!("Exiting...");
    std::process::exit(1);
  }
  monster.start_containers().await;
  

  monster.stop_containers().await;

}

// // #[tokio::main]
// async fn old_main() {
  


//   println!("\n********************************************************************************");
//   println!("***** 2. Running server container **********************************************");
//   // Run the Docker container in a separate thread
//   let container_id = task::spawn_blocking(move || {
//     let secret = &format!("SECRET_KEY_BASE={}", &SECRET_KEY_BASE.to_string());
//     let canary = &format!("CANARY_VALUE={}", &CANARY_VALUE.to_string());
//     let run_output = Command::new("docker")
//       .arg("run")
//       .arg("-d")
//       .args(&["-e", secret])
//       .args(&["-e", canary])
//       .args(&["-p", "3000:3000"])
//       .arg(&image_tag)
//       .output()
//       .expect("Failed to run Docker container");
//     let container_id = String::from_utf8(run_output.stdout)
//       .expect("Failed to parse Docker run output")
//       .trim()
//       .to_string();
//     println!("-> Docker run container ID: {:?}", container_id);
//     container_id
//   })
//   .await
//   .expect("Failed to spawn Docker run task");

//   println!("\n********************************************************************************");
//   println!("***** 3. Querying server container *********************************************");
//   // Query the running container in the main thread
//   tokio::time::sleep(Duration::from_secs(5)).await; // Wait for the container to be fully up
//   let query_output = Command::new("curl")
//     .arg("http://localhost:3000")
//     .arg("-v")
//     .output()
//     .expect("Failed to query Docker container");
//   let output = String::from_utf8(query_output.stderr).expect("Failed to parse Curl output");

//   let cookies: Vec<(String, String)> = output
//     .lines()
//     .filter(|line| line.starts_with("< set-cookie:"))
//     .map(|line| {
//       return line
//         .strip_prefix("< set-cookie: ")
//         .unwrap()
//         .split_once(';')
//         .unwrap()
//         .0
//         .split_once('=')
//         .unwrap();
//     })
//     .map(|(key, val)| return (key.to_owned(), decode(val).unwrap().to_string()))
//     .collect();
//   println!("-> Extracted {} cookies", cookies.len());

//   println!("\n********************************************************************************");
//   println!("***** 4. Cleaning up container *************************************************");
//   let _stop_output = Command::new("docker")
//     .arg("stop")
//     .arg(&container_id)
//     .output()
//     .expect("Failed to stop Docker container");

//   let _rm_output = Command::new("docker")
//     .args(&["rm", &container_id])
//     .output()
//     .expect("Failed to remove Docker container");

//   println!("\n********************************************************************************");
//   println!("***** 5. Checking cookies decyphering ******************************************");
//   cookies.
//     iter().
//     // filter(|(key, _)| *key != "regular" && *key != "signed" ).//"_cookie_monster_session" ).
//     for_each(|(key, value)| {
//       if *key == "signed" { return; }
//       if *key == "regular"{
//         println!("Cookie:\n -> KEY: {}\n -> VAL: {}", key, value);
//         return;
//       }
//       println!("Cookie:\n -> KEY: {}", key);
//       let deciphered = rails::decipher_cookie(rails_version_tag, &value).expect("Could not decipher cookie");
//       println!(" -> VAL: {}", deciphered);
//       if deciphered.contains(&CANARY_VALUE.to_string()) {
//         println!(" -> Decyphering is correct!")
//       } else {
//         println!(" -> Decyphering is incorrect!")
//       }
//     });
// }

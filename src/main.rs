use std::env;

use rails_cookies_monster::rails;
use rails_cookies_monster::RailsCookiesMonster;
use log::info;

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

  let cookies = monster.query_containers().await;

  monster.stop_containers().await;

  let canary = std::env::var("CANARY_VALUE").unwrap();
  for (version, cookie) in cookies {
    // println!("Version: {}", version);
    let (cookie_name, cookie_value) = cookie.split_once(';').unwrap().0.split_once('=').unwrap();
    // println!(" => COOKIES: _{}", cookie_name);
    let message =
      rails::decipher_cookie(&version, cookie_value).expect("Could not decipher cookie");
    // println!(" => MESSAGE: _{}", message);

    match cookie_name {
      "encrypted" => {
        let message = message.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
        assert_eq!(canary, message, "Decrypted cookie does not contain the canary value: {} (Version {}, canary: {})", message, version, canary);
        info!("Version {}, found canary in cookie  {}!", version, cookie_name);
      },
      "_cookie_monster_session" => {
        let message = message.split('"').nth(7).unwrap();
        assert_eq!(canary, message, "Decrypted session does not contain the canary value: {} (Version {}, canary: {})", message, version, canary);
        info!("Version {}, found canary in session {}!", version, cookie_name);
      },
      _ => { unimplemented!() }
    }

  }
}

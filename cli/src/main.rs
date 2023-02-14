use std::fs::{create_dir_all, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::exit;

use clap::{Arg, ArgAction, ArgMatches, Command};
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use daemon::DaemonOptions;

const DEFAULT_BEACON_ID: &str = "default";

fn cli() -> Command {
    Command::new("drand-cli")
        .about("A tool for interacting with a drand daemon")
        .subcommand(
            Command::new("generate-keypair")
                .about("Generate a keypair for a specific beacon on a daemon")
                .arg(Arg::new("beacon_id")
                    .long("id")
                    .help("The beacon ID you wish to generate a keypair for")
                    .action(ArgAction::Set)
                    .default_value(DEFAULT_BEACON_ID)
                )
                .arg(Arg::new("folder")
                    .short('f')
                    .long("folder")
                    .help("generate a keypair for a specific beacon on a daemon")
                    .action(ArgAction::Set)
                    .default_value("~/.drand")
                )
        )
        .subcommand(
            Command::new("start")
                .about("Start a drand daemon with a network for every folder in the multibeacon directory under the folder passed by --folder")
                .arg(Arg::new("folder")
                    .short('f')
                    .long("folder")
                    .help("generate a keypair for a specific beacon on a daemon")
                    .action(ArgAction::Set)
                    .default_value("~/.drand")
                )
        )
}

fn main() {
    let mut cli = cli();
    let matches = cli.clone().get_matches();

    let result = match matches.subcommand() {
        Some(("generate-keypair", args)) => generate_keypair(args),
        Some(("start", args)) => start_daemon(args),
        _ => Err("command not found".to_string())
    };

    match result {
        Ok(msg) => println!("{}", msg),
        Err(msg) => {
            println!("{}", msg);
            cli.render_usage();
            exit(1)
        }
    }
}

fn generate_keypair(args: &ArgMatches) -> Result<String, String> {
    let default_beacon_id = DEFAULT_BEACON_ID.clone().to_string();
    let beacon_id = args.get_one::<String>("beacon_id").unwrap_or_else(|| &default_beacon_id);
    let folder = args.get_one::<String>("folder").ok_or("folder argument was empty")?;

    println!("generating keypair for beacon {} into folder {}", beacon_id, folder);
    let mut csprng = OsRng {};
    let keypair = Keypair::generate(&mut csprng);

    let key_path = format!("{}/multibeacon/{}/key", folder, beacon_id);
    create_dir_all(key_path.clone()).map_err(|err| format!("couldn't create folder {}: {}", key_path, err.to_string()))?;

    let priv_key_path = format!("{}/{}", key_path, "priv.key");
    if Path::new(&priv_key_path).exists() {
        return Err(format!("private key already exists at path {} - delete it first before regenerating", priv_key_path));
    }

    let pub_key_path = format!("{}/{}", key_path, "pub.key");
    if Path::new(&pub_key_path).exists() {
        return Err(format!("public key already exists at path {} - delete it first before regenerating", pub_key_path));
    }

    let mut sk = File::create(priv_key_path).map_err(|err| format!("error creating private key: {}", err.to_string()))?;
    let mut pk = File::create(pub_key_path).map_err(|err| format!("error creating public key: {}", err.to_string()))?;

    sk.metadata()
        .map_err(|err| format!("error setting file permissions for private key: {}", err.to_string()))?
        .permissions()
        .set_mode(0o600);
    sk.write(&keypair.secret.to_bytes())
        .map_err(|err| format!("error writing private key: {}", err.to_string()))?;

    pk.metadata()
        .map_err(|err| format!("error setting file permissions for public key: {}", err.to_string()))?
        .permissions()
        .set_mode(0o740);
    pk.write(&keypair.public.to_bytes())
        .map_err(|err| format!("error writing public key: {}", err.to_string()))?;

    Ok("keypair generated succesfully".to_string())
}

fn start_daemon(args: &ArgMatches) -> Result<String, String> {
    let folder = args.get_one::<String>("folder").ok_or("folder argument was empty")?;

    let mut daemon = daemon::Daemon::new();
    daemon.start(DaemonOptions { folder: folder.clone() })
        .map(|_| "daemon started successfully".to_string())
}
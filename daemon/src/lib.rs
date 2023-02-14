extern crate core;

use std::fs::{DirEntry, read, read_dir};
use std::path;
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use log::{error, info};

pub struct DaemonOptions {
    pub folder: String,
}

pub struct Daemon {
    beacons: Vec<Network>,
}

pub struct Network {
    id: String,
    keypair: Keypair,
}

impl Daemon {
    pub fn new() -> Daemon {
        Daemon {
            beacons: vec![],
        }
    }
    pub fn start(&mut self, options: DaemonOptions) -> Result<&mut Self, String> {
        simple_logger::init().map_err(|err| format!("error initialising logger: {}", err.to_string()))?;
        info!("starting drand daemon");
        self.beacons = load_networks(options.folder)?;

        for x in self.beacons.as_slice() {
            info!("loaded network: {}", x.id)
        }

        return Ok(self);
    }
}

fn load_networks(folder: String) -> Result<Vec<Network>, String> {
    let networks_path = format!("{}/multibeacon", folder);
    let network_dir = read_dir(networks_path.clone())
        .map_err(|err| format!("could not open config path {}: {} - note: paths must be absolute", networks_path.clone(), err.to_string()))?;

    let mut networks: Vec<Network> = vec![];

    for dir in network_dir {
        let d = dir.map_err(|err| format!("error opening key directory: {}", err))?;
        let beacon_id = d.file_name().to_str().ok_or("filename invalid")?.to_string();
        let keypair = load_keypair(d)?;
        let network = Network {
            id: beacon_id,
            keypair,
        };
        networks.push(network);
    }

    return Ok(networks);
}

fn load_keypair(dir: DirEntry) -> Result<Keypair, String> {
    let dir_file_type = dir.file_type().map_err(|_| "couldn't fetch the key dir file type")?;
    if !dir_file_type.is_dir() {
        return Err(format!("the keypair directory was not a directory"));
    }

    let sk = read(dir.path().join("key/priv.key"))
        .map_err(|err| format!("could not read private key: {}", err.to_string()))
        .and_then(|vec| SecretKey::from_bytes(vec.as_slice())
            .map_err(|err| format!("secret key invalid: {}", err))
        )?;


    let pk = read(dir.path().join("key/pub.key"))
        .map_err(|err| format!("could not read public key: {}", err.to_string()))
        .and_then(|vec| PublicKey::from_bytes(vec.as_slice())
            .map_err(|err| format!("public key invalid: {}", err))
        )?;

    return Ok(Keypair {
        secret: sk,
        public: pk,
    });
}
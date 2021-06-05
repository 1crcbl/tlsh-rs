use std::{
    collections::HashMap,
    env,
    fs::{read_dir, File},
    io::{BufReader, Read},
    path::Path,
};

use tlsh::TlshBuilder;

/// In this example, we will compute the hash values for all files in a directory.
fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = Path::new(args.get(1).unwrap());
    let mut buffer = [0; 1024];

    let mut hm = HashMap::new();

    for entry in read_dir(dir).unwrap() {
        assert!(
            entry.is_ok(),
            "Error reading file: {:?}",
            entry.err().unwrap()
        );

        let d = entry.unwrap();
        let pb = d.path();

        if pb.is_file() {
            let mut builder = TlshBuilder::new(
                tlsh::BucketKind::Bucket128,
                tlsh::ChecksumKind::ThreeByte,
                tlsh::Version::Version4,
            );
            let file = File::open(&pb).unwrap();
            let mut reader = BufReader::new(file);

            let mut byte_read = 1;
            while byte_read != 0 {
                match reader.read(&mut buffer) {
                    Ok(n) => {
                        byte_read = n;
                        builder.update_from(&buffer, 0, byte_read);
                    }
                    Err(_) => panic!("Failed to read file: {:?}", &pb),
                };
            }

            hm.insert(
                pb.as_os_str().to_os_string().into_string().unwrap(),
                builder.build().unwrap(),
            );
        }
    }

    for (p1, tlsh1) in &hm {
        println!("++ File: {}", p1);
        println!("   Hash: {}", tlsh1.hash());

        for (p2, tlsh2) in &hm {
            if p1 == p2 {
                continue;
            }

            println!("   diff with {}: {}", p2, tlsh1.diff(tlsh2, true));
        }
    }
}

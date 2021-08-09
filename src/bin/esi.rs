//! Load ESI (EtherCAT Slave Definition) file and print it.
//!
//! Run with e.g.
//!
//! ```bash
//! ETHERCAT_PATH=$(realpath ../ethercat) cargo run --bin esi ./src/AKD\ EtherCAT\ Device\ Description.xml
//! ```

use ethercat_esi::{EtherCatInfo, Pdo, PdoEntry};
use std::{
    env,
    fs::File,
    io::{self, prelude::*},
};

fn print_pdos(pdos: &[Pdo]) {
    for pdo in pdos.iter() {
        println!(
            "{:32} {:#04x}",
            pdo.name.as_ref().unwrap_or(&String::from("(unknown)")),
            u16::from(pdo.idx)
        );

        for entry in pdo.entries.iter() {
            println!(
                "    {:32} {:#04x} - {:#04x} ({} bit {})",
                entry.name.as_ref().unwrap_or(&String::from("(unknown)")),
                u16::from(entry.entry_idx.idx),
                u8::from(entry.entry_idx.sub_idx),
                entry.bit_len,
                entry
                    .data_type
                    .as_ref()
                    .unwrap_or(&String::from("unknown type"))
            );
        }
    }
}

fn main() -> io::Result<()> {
    let esi = match env::args().nth(1) {
        None => panic!("Missing filename"),

        Some(file_name) => {
            let mut xml_file = File::open(file_name)?;
            let mut xml_string = String::new();

            xml_file.read_to_string(&mut xml_string)?;

            let info = EtherCatInfo::from_xml_str(&xml_string)?;

            println!("{:#?}", info);

            info
        }
    };

    // TODO: Read this from the drive
    let revision = 2;

    let device = esi
        .description
        .devices
        .iter()
        .find(|device| device.revision_no == revision)
        .expect("Definition for revision not found");

    // let rx = device
    //     .rx_pdo
    //     .iter()
    //     .flat_map(|pdo| pdo.entries.clone())
    //     .collect::<Vec<_>>();

    // let tx = device
    //     .tx_pdo
    //     .iter()
    //     .flat_map(|pdo| pdo.entries.clone())
    //     .collect::<Vec<_>>();

    println!("RX");

    print_pdos(&device.rx_pdo);

    println!("TX");

    print_pdos(&device.tx_pdo);

    Ok(())
}

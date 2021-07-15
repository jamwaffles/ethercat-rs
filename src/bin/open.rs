//! Open the EtherCAT master interface and do nothing else.

use ethercat::{Master, MasterAccess};

fn main() -> Result<(), std::io::Error> {
    let master = Master::open(0, MasterAccess::ReadWrite)?;
    let info = master.get_info();
    println!("EtherCAT Master: {:#?}", info);
    Ok(())
}

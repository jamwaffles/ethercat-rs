//! Current feedback value logging from the `IL.FB` property at address 0x3558/0.

use ethercat::{AlState, Master, MasterAccess, SdoEntryAddr, SdoIdx, SlavePos};

fn main() -> Result<(), std::io::Error> {
    let slave_pos = SlavePos::from(0);
    let mut master = Master::open(0, MasterAccess::ReadWrite)?;

    master.request_state(slave_pos, AlState::PreOp)?;

    // let thing = master.get_sdo_entry(0.into(), SdoEntryAddr::ByIdx(SdoIdx::new(0x3558, 0)));

    let info = master.get_slave_info(slave_pos)?;

    dbg!(&info);

    let sdo_count = info.sdo_count;

    if sdo_count == 0 {
        println!("Could not find any SDOs");
        return Ok(());
    }

    let info = master.get_info();
    println!("EtherCAT Master: {:#?}", info);
    Ok(())
}

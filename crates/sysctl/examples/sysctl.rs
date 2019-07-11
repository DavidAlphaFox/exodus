extern crate sysctl;

use std::io;

#[allow(dead_code)]
fn walk() -> Result<(), io::Error> {
    let root = sysctl::Mib::default();
    for item in root.iter()? {
        let mib = item?;
        println!("{}", mib.name()?);
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    // walk()?;
    
    #[cfg(target_os = "linux")]
    let key = "net.ipv4.conf.all.forwarding";

    #[cfg(target_os = "macos")]
    let key = "net.inet.ip.forwarding";

    let mib = key.parse::<sysctl::Mib>()?;
    let old_val = mib.value()?;
    println!("Get Value {}: {:?}", mib.name()?, old_val);

    let one = sysctl::Value::I32(1);
    let zero = sysctl::Value::I32(0);

    let val = if old_val == one { zero } else { one };
    println!("Set Value {:?} on {:?}", val, mib.name()?);

    mib.set_value(val)?;

    println!("Get Value {}: {:?}", mib.name()?, mib.value()?);

    Ok(())
}

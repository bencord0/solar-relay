use mcp3008::Mcp3008;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Reading spidev0.0");
    let mut p0 = Mcp3008::new("/dev/spidev0.0")?;
    let mut p1 = Mcp3008::new("/dev/spidev0.1")?;

    let v_ref = 50_f32;
    for pin in 0..8 {
        let value = p0.read_adc(pin)?;
        let v = (value as f32 / 1024_f32) * v_ref;
        println!("pin{pin}: {value} {v:.2}V");
    }

    println!("Reading spidev0.1");
    for pin in 0..8 {
        let value = p1.read_adc(pin)?;
        let v = (value as f32 / 1024_f32) * v_ref;
        println!("pin{pin}: {value} {v:.2}V");
    }

    Ok(())
}

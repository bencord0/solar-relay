use mcp3008::Mcp3008;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let mut p0 = Mcp3008::new("/dev/spidev0.0")?;

    let value = p0.read_adc(0)?;

    println!("{value}");
    Ok(())
}

use core::time::Duration;
use lazy_static::lazy_static;
use mcp3008::Mcp3008;
use prometheus::{
    Encoder, IntGaugeVec, Opts, Registry, TextEncoder,
};
use warp::{Filter, Reply, Rejection};

lazy_static!{
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref VOLTAGE: IntGaugeVec = IntGaugeVec::new(
        Opts::new("adc", "ADC Output"),
        &["device", "pin"],
    ).unwrap();
}

fn register_metrics() {
    REGISTRY.register(Box::new(VOLTAGE.clone())).unwrap();
}

#[tokio::main]
async fn main() {
    register_metrics();

    let metrics = warp::path!("metrics").and_then(metrics_handler);

    tokio::task::spawn(run_adc());

    println!("Started on port 8000");
    warp::serve(metrics)
        .run(([0_u8; 16], 8000))
        .await;
}

async fn metrics_handler() -> Result< impl Reply, Rejection>{
    let encoder = TextEncoder::new();

    let mut buffer = Vec::new();
    encoder.encode(&REGISTRY.gather(), &mut buffer).unwrap();
    let result = String::from_utf8(buffer.clone()).unwrap();
    buffer.clear();

    Ok(result)
}

async fn run_adc() {
    let mut interval = tokio::time::interval(Duration::from_secs(10));
    let device = "/dev/spidev0.0";
    let pin = 0;

    loop {
        interval.tick().await;

        if let Ok(adc_value) = read_adc(device, pin) {
            VOLTAGE
                .with_label_values(&[device, &format!("{}", pin)])
                .set(adc_value.into());
        }
    }
}

fn read_adc(device: &str, pin: u8) -> Result<u16, Box<dyn std::error::Error>> {
    let mut p0 = Mcp3008::new(device)?;
    let value = p0.read_adc(pin)?;
    println!("{device}/{pin}: {value}");
    Ok(value)
}

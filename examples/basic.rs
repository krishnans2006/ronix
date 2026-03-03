use serde::Serialize;

#[derive(Serialize)]
struct FanPoint {
    temp: u32,
    pwm: u8,
}

#[derive(Serialize)]
struct Config {
    name: String,
    enabled: bool,
    poll_interval_ms: u64,
    curve: Vec<FanPoint>,
    log_path: Option<String>,
}

fn main() {
    let config = Config {
        name: "cpu_cooler".into(),
        enabled: true,
        poll_interval_ms: 2000,
        curve: vec![
            FanPoint { temp: 40, pwm: 20 },
            FanPoint { temp: 60, pwm: 100 },
            FanPoint { temp: 80, pwm: 200 },
        ],
        log_path: Some("/var/log/smartcool.log".into()),
    };

    // Serialize to a standalone Nix expression
    let nix = ronix::to_nix(&config).unwrap();
    println!("--- to_nix ---");
    println!("{nix}");

    // Wrap in a NixOS module
    let module = ronix::to_nix_module(&config, "services.smartcool.settings").unwrap();
    println!("--- to_nix_module ---");
    println!("{module}");

    // Convert a RON string directly
    let ron_input = r#"(debug: false, workers: 4)"#;
    let nix = ronix::ron_to_nix(ron_input).unwrap();
    println!("--- ron_to_nix ---");
    println!("{nix}");
}

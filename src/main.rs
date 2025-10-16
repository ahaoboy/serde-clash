use serde_clash::Config;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: serde-clash <FILE.yaml>");
        eprintln!("Example: serde-clash config.yaml");
        process::exit(1);
    }

    let file_path = &args[1];

    // Read configuration file
    let config_content = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Error: Failed to read file '{}': {}", file_path, err);
        process::exit(1);
    });

    // Parse YAML
    let config: Config = serde_yaml::from_str(&config_content).unwrap_or_else(|err| {
        eprintln!("Error: Failed to parse YAML: {}", err);
        process::exit(1);
    });

    // Print parsed configuration
    println!("✓ Configuration parsed successfully!\n");
    println!("=== Basic Settings ===");
    println!("Port: {}", config.port);
    println!("SOCKS Port: {}", config.socks_port);
    println!("Allow LAN: {}", config.allow_lan);
    println!("Mode: {}", config.mode);
    println!("Log Level: {}", config.log_level);
    println!("External Controller: {}", config.external_controller);

    println!("\n=== DNS Configuration ===");
    if let Some(dns) = config.dns {
        println!("Enabled: {}", dns.enable);
        println!("Nameservers:");
        for ns in &dns.nameserver {
            println!("  - {}", ns);
        }
        println!("Fallback Servers:");
        for fb in &dns.fallback {
            println!("  - {}", fb);
        }
    }

    println!("\n=== Proxies ({} total) ===", config.proxies.len());
    for (i, proxy) in config.proxies.iter().enumerate() {
        println!("[{}] {}", i + 1, proxy.name);
        println!("    Server: {}:{}", proxy.server, proxy.port);
        println!("    Type: {}", proxy.proxy_type);
        if let Some(cipher) = &proxy.cipher {
            println!("    Cipher: {}", cipher);
        }
        if let Some(udp) = proxy.udp {
            println!("    UDP: {}", udp);
        }
        if let Some(fingerprint) = &proxy.client_fingerprint {
            println!("    Client Fingerprint: {}", fingerprint);
        }
    }

    println!(
        "\n=== Proxy Groups ({} total) ===",
        config.proxy_groups.len()
    );
    for group in &config.proxy_groups {
        println!("• {} (Type: {})", group.name, group.group_type);
        println!("  Proxies: {} nodes", group.proxies.len());
        if let Some(url) = &group.url {
            println!("  Test URL: {}", url);
        }
        if let Some(interval) = group.interval {
            println!("  Interval: {}s", interval);
        }
        if let Some(tolerance) = group.tolerance {
            println!("  Tolerance: {}ms", tolerance);
        }
    }

    println!("\n=== Rules ({} total) ===", config.rules.len());
    let display_count = config.rules.len().min(5);
    for rule in config.rules.iter().take(display_count) {
        println!("  - {}", rule);
    }
    if config.rules.len() > display_count {
        println!(
            "  ... and {} more rules",
            config.rules.len() - display_count
        );
    }

    println!("\n✓ Parse completed successfully!");
}

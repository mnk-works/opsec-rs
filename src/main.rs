use std::thread;
use std::time::Duration;
use std::io::{self, Write};

fn main() {
    println!("\x1b[1;31m");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║  ██████╗ ███████╗███████╗████████╗██╗  ██╗███████╗██████╗                   ║");
    println!("║  ██╔══██╗██╔════╝██╔════╝╚══██╔══╝██║  ██║██╔════╝██╔══██╗                  ║");
    println!("║  ██████╔╝█████╗  █████╗     ██║   ███████║█████╗  ██████╔╝                  ║");
    println!("║  ██╔═══╝ ██╔══╝  ██╔══╝     ██║   ██╔══██║██╔══╝  ██╔══██╗                  ║");
    println!("║  ██║     ███████╗███████╗   ██║   ██║  ██║███████╗██║  ██║                  ║");
    println!("║  ╚═╝     ╚══════╝╚══════╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝                  ║");
    println!("║                                                                              ║");
    println!("║         ██████╗ ███████╗███╗   ██╗███████╗██████╗  ██████╗                  ║");
    println!("║         ██╔══██╗██╔════╝████╗  ██║██╔════╝██╔══██╗██╔═══██╗                 ║");
    println!("║         ██████╔╝█████╗  ██╔██╗ ██║█████╗  ██████╔╝██║   ██║                 ║");
    println!("║         ██╔═══╝ ██╔══╝  ██║╚██╗██║██╔══╝  ██╔══██╗██║   ██║                 ║");
    println!("║         ██║     ███████╗██║ ╚████║███████╗██║  ██║╚██████╔╝                 ║");
    println!("║         ╚═╝     ╚══════╝╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝ ╚═════╝                  ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!("\x1b[0m");

    fake_delay("Initializing OPSEC subsystem...", 800);
    fake_delay("Loading paranoia modules...", 1200);
    fake_delay("Calibrating tinfoil hat frequency...", 1500);
    fake_delay("Establishing air-gapped connection to localhost...", 2000);
    fake_delay("Generating 4096-bit RSA keys for your toaster...", 2500);
    fake_delay("Encrypting RAM with AES-256-GCM (and hope)...", 1800);
    fake_delay("Spawning decoy processes: chrome.exe, svchost.exe, minesweeper.exe...", 2200);
    fake_delay("Routing traffic through 7 proxies, 3 VPNs, and a potato...", 3000);
    fake_delay("Verifying you're not a glowie... [PENDING]", 1500);
    fake_delay("Scanning for hardware backdoors in your mechanical keyboard...", 2000);
    fake_delay("Injecting entropy from /dev/urandom and your mouse jiggles...", 1800);
    fake_delay("Compiling kernel module to hide your anime folder...", 2500);
    fake_delay("Setting up dead man's switch (triggers if you touch grass)...", 2000);
    fake_delay("Configuring TEMPEST shielding for your monitor emissions...", 2200);
    fake_delay("Burning bridge to clearnet... ████████████ 100%", 1500);
    fake_delay("Initializing blockchain-based identity verification...", 2000);
    fake_delay("Minting NFT of your threat model...", 1800);
    fake_delay("Deploying honeypot disguised as 'passwords.txt'...", 1500);
    fake_delay("Teaching your CPU to speak in cipher...", 1200);
    fake_delay("Finalizing zero-trust architecture (trust no one, not even yourself)...", 2000);

    println!("\n\x1b[1;32m");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║  ✅ OPSEC-rs v13.37.420-INSTALLED                                          ║");
    println!("║                                                                              ║");
    println!("║  Your system is now:                                                        ║");
    println!("║    • Air-gapped (from reality)                                              ║");
    println!("║    • Quantum-resistant (to common sense)                                    ║");
    println!("║    • Zero-trust (you don't trust this output)                               ║");
    println!("║    • Fully encrypted (including this message)                               ║");
    println!("║    • Running on a VM inside a VM inside a container on a Raspberry Pi       ║");
    println!("║                                                                              ║");
    println!("║  Threat model: \"Someone might see my GitHub stars\"                         ║");
    println!("║  Actual risk:  You forgot to push to main branch                            ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!("\x1b[0m");

    println!("\n\x1b[1;33m💡 Pro tips for maximum OPSEC:\x1b[0m");
    let tips = [
        "Never use your real name. Or any name. Become the void.",
        "Type passwords with your eyes closed. Muscle memory only.",
        "Your threat model includes your cat. Especially your cat.",
        "Rotate SSH keys daily. Or hourly. Or every keystroke.",
        "Use Qubes OS. On a ThinkPad X230. Running Coreboot. In a Faraday cage.",
        "All commits signed with a YubiKey you swallowed for safekeeping.",
        "Your browser fingerprint is unique. So is your typing rhythm. Stop typing.",
        "HTTPS is a honeypot. Use carrier pigeons with OTP encryption.",
        "The cloud is just someone else's computer. The ground is just someone else's dirt.",
        "OPSEC is a lifestyle. You are the lifestyle. Embrace the paranoia.",
    ];
    for (i, tip) in tips.iter().enumerate() {
        print!("   {}. ", i + 1);
        io::stdout().flush().unwrap();
        for c in tip.chars() {
            print!("{}", c);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(15));
        }
        println!();
        thread::sleep(Duration::from_millis(300));
    }

    println!("\n\x1b[1;36m");
    println!("   ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄");
    println!("   █  REMEMBER: The best OPSEC is not installing opsec-rs            █");
    println!("   █  But you already did. Welcome to the watchlist. 🫡            █");
    println!("   ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀");
    println!("\x1b[0m\n");
}

fn fake_delay(msg: &str, ms: u64) {
    print!("\x1b[1;34m[*] \x1b[1;37m{}\x1b[0m ", msg);
    io::stdout().flush().unwrap();
    let steps: usize = 10;
    let step_ms = ms / steps as u64;
    for i in 0..=steps {
        let pct = (i * 100) / steps;
        print!("\r\x1b[1;34m[*] \x1b[1;37m{} \x1b[1;32m[{:<10}] {}%\x1b[0m", msg, "█".repeat(i), pct);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(step_ms));
    }
    println!("\r\x1b[1;34m[✓] \x1b[1;37m{}\x1b[0m", msg);
}
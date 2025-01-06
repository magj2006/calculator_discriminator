use sha2::{Digest, Sha256};

fn get_discriminator(instruction_name: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(format!("global:{}", instruction_name));
    let result = hasher.finalize();
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&result[..8]);
    discriminator
}

fn main() {
    let instruction_names = vec![
        "initialize",
        "pump",
        "vanityPump",
        "buy",
        "sell",
        "graduate",
        "updateConfig",
        "transferOwnership",
        "transferOperatorship",
    ];

    for instruction in instruction_names {
        let discriminator = get_discriminator(instruction);
        println!(
            "Instruction: {:<20} Discriminator: {:?}",
            instruction, discriminator
        );
    }
}

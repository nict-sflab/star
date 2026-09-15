#[path = "../../protocol.rs"]
mod protocol;

use key_manager::KeyManager;
use std::io;

fn main() -> io::Result<()> {
    let mut input = io::stdin().lock();
    let mut output = io::stdout().lock();
    let mut manager = None;
    while let Some((op, payload)) = protocol::read(&mut input)? {
        let result = match op {
            protocol::INIT => KeyManager::restore(&payload).map(|(restored, sealed)| {
                manager = Some(restored);
                sealed
            }),
            _ => manager
                .as_ref()
                .ok_or_else(|| "key manager is not initialized".to_string())
                .and_then(|manager| match op {
                    protocol::PUBLIC_KEY => Ok(manager.public_key().to_vec()),
                    protocol::ATTEST => manager.attest(),
                    protocol::EXPORT_KEYS => manager.export(&payload),
                    _ => Err("unknown operation".into()),
                }),
        };
        match result {
            Ok(payload) => protocol::write(&mut output, protocol::OK, &payload)?,
            Err(err) => protocol::write(&mut output, protocol::ERROR, err.as_bytes())?,
        }
    }
    Ok(())
}

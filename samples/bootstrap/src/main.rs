use ::windows_app::*;
use ::windows_app::Microsoft::Windows::System::Power::*;

fn main() -> ::windows::runtime::Result<()> {
    bootstrap::initialize()
        .and_then(|_| {
            println!(
                "Remaining charge: {}%",
                PowerManager::RemainingChargePercent()?
            );
            Ok(())
        })
        .and_then(|_| bootstrap::uninitialize())?;
    Ok(())
}

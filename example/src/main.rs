use ni_fpga::fxp::UnsignedFXP;

mod rio {
    use lvbitfile2rust_macros::lvbitfile2rust;
    lvbitfile2rust!("/boot/user.lvbitx");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Turn PWM on if the RSL is on!
    // My scope tells me the RSL blinks at 5Hz in teleop mode!
    let peripherals = rio::Peripherals::take("RIO0")?;
    loop {
        let leds = peripherals.LEDs.read()?;
        peripherals
            .PWM_Hdr0
            .write(&UnsignedFXP::<12, 12>::from_raw({
                if leds.RSL {
                    (1 << 12) - 1
                } else {
                    0
                }
            })?)?;
    }
}

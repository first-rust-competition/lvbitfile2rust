use ni_fpga::fxp::UnsignedFXP;

mod rio {
    use lvbitfile2rust_macros::lvbitfile2rust;
    lvbitfile2rust!("/boot/user.lvbitx");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Turn PWM on if the RSL is on!
    // My scope tells me the RSL blinks at 2.5Hz in teleop mode!
    let peripherals = rio::Peripherals::take("RIO0")?;
    loop {
        let leds = peripherals.LEDs.read()?;
        peripherals.PWM_Hdr0.write(&{
            if leds.RSL {
                UnsignedFXP::max_value()
            } else {
                UnsignedFXP::min_value()
            }
        })?;
    }
}

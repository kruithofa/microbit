#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;

use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{
        gpio::{Level, OpenDrainConfig},
        prelude::*,
        saadc::SaadcConfig,
        Saadc, Timer,
    },
};

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut display = Display::new(board.display_pins);

        // initialize adc
        let saadc_config = SaadcConfig::default();
        let mut saadc = Saadc::new(board.SAADC, saadc_config);
        let mut mic_in = board.microphone_pins.mic_in.into_floating_input();

        // enable microphone
        board
            .microphone_pins
            .mic_run
            .into_open_drain_output(OpenDrainConfig::Disconnect0HighDrive1, Level::High);

        let mut count: u64 = 0;
        let mut sum: u64 = 0;
        let mut max_value: u16 = 0;
	let mut new_value: u16 = 0;
	let mut init_value: u16 = 0;

	let image1 = [ [ 1, 0, 1, 0, 1],
	[ 1, 0, 1, 0, 1],
	[ 1, 0, 1, 0, 1],
	[ 1, 0, 1, 0, 1],
	[ 1, 0, 1, 0, 1], ];
	display.show(&mut timer, image1, 100);

init_value = saadc
.read(&mut mic_in)
.expect("Error") as u16;
        loop {
            let mic_value = saadc
                .read(&mut mic_in)
                .expect("could not read value of microphone") as u16;

            // Smoothen the signal as audio comes in waves
            max_value = max_value.max(mic_value);

            sum += mic_value as u64;
            count += 1;

            if count % 100 == 0 {
                let avg = (sum / count) as u16;
//		init_value = avg;
		  new_value = max_value;

                let image = [
                    [if new_value > init_value + 100 { 1 } else { 0 }; 5],
                    [if new_value >  init_value + 80 { 1 } else { 0 }; 5],
                    [if new_value >  init_value + 60 { 1 } else { 0 }; 5],
                    [if new_value >  init_value + 40 { 1 } else { 0 }; 5],
                    [if new_value >  init_value + 20 { 1 } else { 0 }; 5],
                ];
                display.show(&mut timer, image, 10);
                max_value = 0;
            }
        }
    }

    panic!("End");
}
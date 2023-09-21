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

	let mut image1 = [ [ 1, 1, 1, 1, 1],
	[ 1, 1, 1, 1, 1],
	[ 1, 1, 1, 1, 1],
	[ 1, 1, 1, 1, 1],
	[ 1, 1, 1, 1, 1], ];
	let mut image = image1;
	// startup image, we also need a delay for the adc to initialise
	display.show(&mut timer, image1, 100);

// we find the offset value, i.e. the background noise
        init_value = saadc
                     .read(&mut mic_in)
                     .expect("Error") as u16;
        init_value = init_value + 5;
        loop {
            let mic_value = saadc
                .read(&mut mic_in)
                .expect("could not read value of microphone") as u16;

            // Smoothen the signal as audio comes in waves
            new_value = new_value.max(mic_value);
            max_value = max_value.max(new_value);
            sum += mic_value as u64;
            count += 1;

            if count % 100 == 0 {
                let avg = (sum / count) as u16;

// this should be done the rust way
		for row in 0..5 {
		for column in 1..5 {
		image1[row][column] = image[row][column-1];
		}
		}

// this should be done the rust way
               image = image1;
	       let range = 5; //  (max_value - init_value)/5;
	       if (new_value > init_value + 16*range) {image[0][0] = 1} else {image[0][0] = 0}
	       if (new_value > init_value + 8*range) {image[1][0] = 1} else {image[1][0] = 0}
	       if (new_value > init_value + 4*range) {image[2][0] = 1} else {image[2][0] = 0}
	       if (new_value > init_value + 2*range) {image[3][0] = 1} else {image[3][0] = 0}
	       if (new_value > init_value + range) {image[4][0] = 1} else {image[4][0] = 0}


                display.show(&mut timer, image, 10);
                new_value = 0;
            }
        }
    }

    panic!("End");
}
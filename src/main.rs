#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{
  board::Board,
  hal::{prelude::*, timer::Timer},
};
use panic_halt as _;

extern crate alloc;
use alloc::vec::Vec;


use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
{
  use core::mem::MaybeUninit;
  const HEAP_SIZE: usize = 8192; // 8KiB
  static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
  unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}
  let mut board = Board::take().expect("Failed to take board");
  let mut timer = Timer::new(board.TIMER0);
  let mut row = board.display_pins.row2;
  let delay = 150u16;

  board.display_pins.col1.set_low().expect("Failed to set col1 low");
  board.display_pins.col2.set_low().expect("Failed to set col1 low");

let mut vec = Vec::new();
vec.push(true);
vec.push(false);
vec.push(true);
vec.push(false);
vec.push(false);
vec.push(false);

//vec.iter().cycle().for_each(|v| {
//	match v {
//		true => row.set_high().expect("Failed to set row high"),
///		false => row.set_low().expect("Failed to set row low"),
//	}
//	timer.delay_ms(delay);
//});

//loop {}

  loop {
    row.set_high().expect("Failed to set row1 high");
    timer.delay_ms(delay);
    row.set_low().expect("Failed to set row1 low");
    timer.delay_ms(delay);
  }
}

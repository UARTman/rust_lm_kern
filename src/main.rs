#![no_std]
#![no_main]
#![feature(generic_const_exprs)]

mod arch;
mod framebuffer;
#[allow(dead_code)]
mod psf;
mod serial;

use core::{mem::MaybeUninit, panic::PanicInfo};

use core::fmt::Write;
use framebuffer::{Color, DoubleBufferedFramebuffer, Framebuffer, FramebufferWriter};

static mut FRAMEBUFFER_REQ: limine::FramebufferRequest = limine::FramebufferRequest::new(0);

const FONT: &[u8] = include_bytes!("ter-powerline-v16b.psf");

static mut FB_WRITER: MaybeUninit<FramebufferWriter> = MaybeUninit::uninit();


#[no_mangle]
extern "C" fn _start() -> ! {
    let response = unsafe { FRAMEBUFFER_REQ.get_response().get_mut().unwrap() };
    let mut fb = unsafe { Framebuffer::new(&response.framebuffers()[0]) };
    for i in 0..100 {
        fb.write_pixel(i, i, framebuffer::Color::WHITE);
    }
    unsafe {
        FB_WRITER = MaybeUninit::new(FramebufferWriter::new(DoubleBufferedFramebuffer::new(fb), FONT, 100, 0, Color::WHITE, Color::BLACK));
        let fb_writer = FB_WRITER.assume_init_mut();
        _ = write!(fb_writer, "Hello world!\nI am anton.\n");
        for i in 0..200 {
            fb_writer.write_byte(b'*');
        }
        fb_writer.write_byte(b'\n');
        for i in 0..100 {
            for j in 0..1000000 {}
            _ = write!(fb_writer, "{i}\n");
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        FB_WRITER.assume_init_mut().fb.fb.write_pixel(0, 0, Color {r: 255, g: 0, b: 0});   
    }
    loop {}
}

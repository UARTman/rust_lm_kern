use core::arch::asm;

/// Read a byte from an IO port.
///
/// # Safety
///
/// Needs IO privileges.
#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let mut x: u8;
    asm!("in {x},dx", 
        x = out(reg_byte) x, 
        in("dx") port);
    x
}

/// Write a byte to an IO port.
///
/// # Safety
///
/// Needs IO privileges.
#[inline]
pub unsafe fn outb(port: u16, val: u8) {
    asm!("out dx,{x}", 
        x = in(reg_byte) val, 
        in("dx") port)
}

/// Read a word from an IO port.
///
/// # Safety
///
/// Needs IO privileges.
#[inline]
pub unsafe fn inw(port: u16) -> u16 {
    let mut x: u16;
    asm!("in {0:x},dx", 
        out(reg) x, 
        in("dx") port);
    x
}

/// Write a word to an IO port.
///
/// # Safety
///
/// Needs IO privileges.
#[inline]
pub unsafe fn outw(port: u16, val: u16) {
    asm!("out dx,{0:x}", 
        in(reg) val, 
        in("dx") port)
}

/// Read a 32-bit value from an IO port.
///
/// # Safety
///
/// Needs IO privileges.
#[inline]
pub unsafe fn inl(port: u16) -> u32 {
    let mut x: u32;
    asm!("in {x},dx", 
        x = out(reg) x, 
        in("dx") port);
    x
}

/// Write a 32-bit value to an IO port.
///
/// # Safety
///
/// Needs IO privileges.
#[inline]
pub unsafe fn outl(port: u16, val: u32) {
    asm!("out dx,{x}", 
        x = in(reg) val, 
        in("dx") port)
}

/// Wait for a very small tick of time.
///
/// # Safety
///
/// Needs IO privileges.
#[inline]
pub unsafe fn io_wait() {
    outb(0x80, 0);
}

/// Get value of FLAGS register.
/// 
/// # Safety
/// 
/// Needs a properly set up stack.
#[inline]
pub unsafe fn save_flags() -> u32 {
    let f: u32;
    asm!("pushf
    pop ax", out("ax") f);
    f
}

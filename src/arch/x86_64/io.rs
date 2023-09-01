use super::asm;

/// x86 IO port
pub struct IOPort<const PORT: u16>;

impl<const PORT: u16> IOPort<PORT> {
    /// 8-bit write
    ///
    /// # Safety
    ///
    /// Needs IO privileges.
    #[inline]
    pub unsafe fn inb(&mut self) -> u8 {
        asm::inb(PORT)
    }

    /// 8-bit write
    ///
    /// # Safety
    ///
    /// Needs IO privileges.
    #[inline]
    pub unsafe fn outb(&mut self, val: u8) {
        asm::outb(PORT, val)
    }

    /// 16-bit read
    ///
    /// # Safety
    ///
    /// Needs IO privileges.
    #[inline]
    pub unsafe fn inw(&mut self) -> u16 {
        asm::inw(PORT)
    }

    /// 16-bit write
    ///
    /// # Safety
    ///
    /// Needs IO privileges.
    #[inline]
    pub unsafe fn outw(&mut self, val: u16) {
        asm::outw(PORT, val)
    }

    /// 32-bit read
    /// 
    /// # Safety
    /// 
    /// Needs IO privileges
    #[inline]
    pub unsafe fn inl(&mut self) -> u32 {
        asm::inl(PORT)
    }

    /// 32-bit write
    /// 
    /// # Safety
    /// 
    /// Needs IO privileges
    #[inline]
    pub unsafe fn outl(&mut self, val: u32) {
        asm::outl(PORT, val)
    }
}

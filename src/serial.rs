use crate::arch::x86_64::io::IOPort;

pub struct SerialPort<const BASE_PORT: u16>
where
    IOPort<{ BASE_PORT + 1 }>: Sized,
    IOPort<{ BASE_PORT + 1 }>: Sized,
    IOPort<{ BASE_PORT + 2 }>: Sized,
    IOPort<{ BASE_PORT + 3 }>: Sized,
    IOPort<{ BASE_PORT + 4 }>: Sized,
    IOPort<{ BASE_PORT + 5 }>: Sized,
    IOPort<{ BASE_PORT + 6 }>: Sized,
    IOPort<{ BASE_PORT + 7 }>: Sized,
{
    data_reg: IOPort<BASE_PORT>,
    int_enable_reg: IOPort<{ BASE_PORT + 1 }>,
    int_id_reg: IOPort<{ BASE_PORT + 2 }>,
    line_control_reg: IOPort<{ BASE_PORT + 3 }>,
    modem_control_reg: IOPort<{ BASE_PORT + 4 }>,
    line_status_reg: IOPort<{ BASE_PORT + 5 }>,
    modem_status_reg: IOPort<{ BASE_PORT + 6 }>,
    scratch_register: IOPort<{ BASE_PORT + 7 }>,
}

impl<const BASE_PORT: u16> SerialPort<BASE_PORT>
where
    IOPort<{ BASE_PORT + 1 }>: Sized,
    IOPort<{ BASE_PORT + 1 }>: Sized,
    IOPort<{ BASE_PORT + 2 }>: Sized,
    IOPort<{ BASE_PORT + 3 }>: Sized,
    IOPort<{ BASE_PORT + 4 }>: Sized,
    IOPort<{ BASE_PORT + 5 }>: Sized,
    IOPort<{ BASE_PORT + 6 }>: Sized,
    IOPort<{ BASE_PORT + 7 }>: Sized,
{
    pub unsafe fn try_new() -> Option<Self> {
        let mut port = SerialPort::<BASE_PORT> {
            data_reg: IOPort,
            int_enable_reg: IOPort,
            int_id_reg: IOPort,
            line_control_reg: IOPort,
            modem_control_reg: IOPort,
            line_status_reg: IOPort,
            modem_status_reg: IOPort,
            scratch_register: IOPort,
        };
        port.interrupt_disable();
        port.set_baud_divisor(3); // Baud divisor 38400
        port.line_control_reg.outb(0x03); // 8 bits, no parity, 1 stop
        port.int_id_reg.outb(0xC7); // Enable FIFO, clear them, 14 byte treshold
        port.modem_control_reg.outb(0x0B); // IRQ enable, RTS/DSR set
        port.modem_control_reg.outb(0x1E); // Set in loopback for test
        port.data_reg.outb(0xAE);
        if port.data_reg.inb() != 0xAE {
            None
        } else {
            port.modem_control_reg.outb(0x0F);
            Some(port)
        }
    }

    unsafe fn interrupt_disable(&mut self) {
        self.int_enable_reg.outb(0x00);
    }

    unsafe fn set_baud_divisor(&mut self, divisor: u16) {
        let old_flag = self.line_control_reg.inb();
        self.line_control_reg.outb(old_flag | 0x80);
        self.data_reg.outb((divisor & 0xFF) as u8);
        self.int_enable_reg.outb((divisor >> 8) as u8);
        self.line_control_reg.outb(old_flag);
    }

    unsafe fn serial_received(&mut self) -> bool {
        self.line_status_reg.inb() & 1 != 0
    }

    pub unsafe fn read_byte(&mut self) -> u8 {
        while !self.serial_received() {}
        self.data_reg.inb()
    }

    unsafe fn transmit_empty(&mut self) -> bool {
        self.line_status_reg.inb() & 0x20 != 0
    }

    pub unsafe fn write_byte(&mut self, b: u8) {
        while !self.transmit_empty() {}
        self.data_reg.outb(b);
    }
}
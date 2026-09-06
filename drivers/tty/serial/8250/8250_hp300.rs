//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_hp300.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
//
// Driver for the 98626/98644/internal serial interface on hp300/hp400
// (based on the National Semiconductor INS8250/NS16550AF/WD16C552 UARTs)
//
// Ported from 2.2 and modified to use the normal 8250 driver
// by Kars de Jong <jongk@linux-m68k.org>, May 2004.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hp300_port {
    pub /: *mut *mut *mut hp300_port next; / next port,
    pub /: *mut *mut int line; / line (tty) number,
}

    static struct hp300_port *hp300_ports;

    static int hpdca_init_one(struct dio_dev *d,
    const struct dio_device_id *ent);
    static void hpdca_remove_one(struct dio_dev *d);
    static struct dio_device_id hpdca_dio_tbl[] = {
    { DIO_ID_DCA0 },
    { DIO_ID_DCA0REM },
    { DIO_ID_DCA1 },
    { DIO_ID_DCA1REM },
    { 0 }
    };
    static struct dio_driver hpdca_driver = {
    .name      = "hpdca",
    .id_table  = hpdca_dio_tbl,
    .probe     = hpdca_init_one,
    .remove    = hpdca_remove_one,
    };

    static unsigned int num_ports;
    extern int hp300_uart_scode;
// Offset to UART registers from base of DCA
pub const UART_OFFSET: c_int = 17;
pub const DCA_ID: c_uint = 0x01	/* ID (read), reset (write) */;
pub const DCA_IC: c_uint = 0x03	/* Interrupt control        */;
// Interrupt control
pub const DCA_IC_IE: c_uint = 0x80	/* Master interrupt enable  */;
pub const HPDCA_BAUD_BASE: c_int = 153600;
// Base address of the Frodo part

//
// Where we find the 8250-like APCI ports, and how far apart they are.
//
pub const FRODO_APCIBASE: c_uint = 0x0;
pub const FRODO_APCISPACE: c_uint = 0x20;

pub const HPAPCI_BAUD_BASE: c_int = 500400;

//
// Parse the bootinfo to find descriptions for headless console and
// debug serial ports and register them with the 8250 driver.
//
#[no_mangle]
pub unsafe extern "C" fn hp300_setup_serial_console() -> int __init {
    int __init hp300_setup_serial_console(void)
    {
    int scode;
    struct uart_port port;
    memset(&port, 0, sizeof(port));
    if (hp300_uart_scode < 0 || hp300_uart_scode > DIO_SCMAX)
    return 0;
    if (DIO_SCINHOLE(hp300_uart_scode))
    return 0;
    scode = hp300_uart_scode;
// Memory mapped I/O
    port.iotype = UPIO_MEM;
    port.flags = UPF_SKIP_TEST | UPF_SHARE_IRQ | UPF_BOOT_AUTOCONF;
    port.type = PORT_UNKNOWN;
// Check for APCI console
    if (scode == 256) {

    pr_info("Serial console is HP APCI 1\n");
    port.uartclk = HPAPCI_BAUD_BASE * 16;
    port.mapbase = (FRODO_BASE + FRODO_APCI_OFFSET(1));
    port.membase = (char *)(port.mapbase + DIO_VIRADDRBASE);
    port.regshift = 2;
    add_preferred_console("ttyS", port.line, "9600n8");

    pr_warn("Serial console is APCI but support is disabled (CONFIG_HPAPCI)!\n");
    return 0;

    } else {

    let mut pa: c_ulong = dio_scodetophysaddr(scode);
    if (!pa)
    return 0;
    pr_info("Serial console is HP DCA at select code %d\n", scode);
    port.uartclk = HPDCA_BAUD_BASE * 16;
    port.mapbase = (pa + UART_OFFSET);
    port.membase = (char *)(port.mapbase + DIO_VIRADDRBASE);
    port.regshift = 1;
    port.irq = DIO_IPL(pa + DIO_VIRADDRBASE);
// Enable board-interrupts
    out_8(pa + DIO_VIRADDRBASE + DCA_IC, DCA_IC_IE);
    if (DIO_ID(pa + DIO_VIRADDRBASE) & 0x80)
    add_preferred_console("ttyS", port.line, "9600n8");

    pr_warn("Serial console is DCA but support is disabled (CONFIG_HPDCA)!\n");
    return 0;

    }
    if (early_serial_setup(&port) < 0)
    pr_warn("%s: early_serial_setup() failed.\n", __func__);
    return 0;
    }

    static int hpdca_init_one(struct dio_dev *d,
    const struct dio_device_id *ent)
    {
    struct uart_8250_port uart;
    int line;

    if (hp300_uart_scode == d.scode) {
// Already got it.
    return 0;
    }

    memset(&uart, 0, sizeof(uart));
// Memory mapped I/O
    uart.port.iotype = UPIO_MEM;
    uart.port.flags = UPF_SKIP_TEST | UPF_SHARE_IRQ | UPF_BOOT_AUTOCONF;
    uart.port.irq = d.ipl;
    uart.port.uartclk = HPDCA_BAUD_BASE * 16;
    uart.port.mapbase = (d.resource.start + UART_OFFSET);
    uart.port.membase = (char *)(uart.port.mapbase + DIO_VIRADDRBASE);
    uart.port.regshift = 1;
    uart.port.dev = &d.dev;
    line = serial8250_register_8250_port(&uart);
    if (line < 0) {
    dev_notice(&d.dev,
    "8250_hp300: register_serial() DCA scode %d irq %d failed\n",
    d.scode, uart.port.irq);
    return -ENOMEM;
    }
// Enable board-interrupts
    out_8(d.resource.start + DIO_VIRADDRBASE + DCA_IC, DCA_IC_IE);
    dio_set_drvdata(d, (void *)line);
// Reset the DCA
    out_8(d.resource.start + DIO_VIRADDRBASE + DCA_ID, 0xff);
    udelay(100);
    num_ports++;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn hp300_8250_init() -> int __init {
    static int __init hp300_8250_init(void)
    {
    static int called;

    int line;
    unsigned long base;
    struct uart_8250_port uart;
    struct hp300_port *port;
    int i;

    if (called)
    return -ENODEV;
    called = 1;
    if (!MACH_IS_HP300)
    return -ENODEV;

    dio_register_driver(&hpdca_driver);

    if (hp300_model < HP_400) {
    if (!num_ports)
    return -ENODEV;
    return 0;
    }
// These models have the Frodo chip.
// Port 0 is reserved for the Apollo Domain keyboard.
// Port 1 is either the console or the DCA.
//
    for (i = 1; i < 4; i++) {
// Port 1 is the console on a 425e, on other machines it's
// mapped to DCA.
//

    if (i == 1)
    continue;

// Create new serial device
    port = kmalloc_obj(struct hp300_port);
    if (!port)
    return -ENOMEM;
    memset(&uart, 0, sizeof(uart));
    base = (FRODO_BASE + FRODO_APCI_OFFSET(i));
// Memory mapped I/O
    uart.port.iotype = UPIO_MEM;
    uart.port.flags = UPF_SKIP_TEST | UPF_SHARE_IRQ
    | UPF_BOOT_AUTOCONF;
// XXX - no interrupt support yet
    uart.port.irq = 0;
    uart.port.uartclk = HPAPCI_BAUD_BASE * 16;
    uart.port.mapbase = base;
    uart.port.membase = (char *)(base + DIO_VIRADDRBASE);
    uart.port.regshift = 2;
    line = serial8250_register_8250_port(&uart);
    if (line < 0) {
    dev_notice(uart.port.dev,
    "8250_hp300: register_serial() APCI %d irq %d failed\n",
    i, uart.port.irq);
    kfree(port);
    continue;
    }
    port.line = line;
    port.next = hp300_ports;
    hp300_ports = port;
    num_ports++;
    }

// Any boards found?
    if (!num_ports)
    return -ENODEV;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn hpdca_remove_one(d: *mut dio_dev) {
    static void hpdca_remove_one(struct dio_dev *d)
    {
    int line;
    line = (int) dio_get_drvdata(d);
    if (d.resource.start) {
// Disable board-interrupts
    out_8(d.resource.start + DIO_VIRADDRBASE + DCA_IC, 0);
    }
    serial8250_unregister_port(line);
    }

#[no_mangle]
unsafe extern "C" fn hp300_8250_exit() -> void __exit {
    static void __exit hp300_8250_exit(void)
    {

    struct hp300_port *port, *to_free;
    for (port = hp300_ports; port; ) {
    serial8250_unregister_port(port.line);
    to_free = port;
    port = port.next;
    kfree(to_free);
    }
    hp300_ports = core::ptr::null_mut();

    dio_unregister_driver(&hpdca_driver);

    }
    module_init(hp300_8250_init);
    module_exit(hp300_8250_exit);
    MODULE_DESCRIPTION("HP DCA/APCI serial driver");
    MODULE_AUTHOR("Kars de Jong <jongk@linux-m68k.org>");
    MODULE_LICENSE("GPL");

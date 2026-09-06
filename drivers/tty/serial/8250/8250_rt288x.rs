//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_rt288x.c
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
// RT288x/Au1xxx driver
//

pub const RT288X_DL: c_uint = 0x28;
// Au1x00/RT288x UART hardware has a weird register layout
    static const u8 au_io_in_map[7] = {
    [UART_RX]	= 0,
    [UART_IER]	= 2,
    [UART_IIR]	= 3,
    [UART_LCR]	= 5,
    [UART_MCR]	= 6,
    [UART_LSR]	= 7,
    [UART_MSR]	= 8,
    };
    static const u8 au_io_out_map[5] = {
    [UART_TX]	= 1,
    [UART_IER]	= 2,
    [UART_FCR]	= 4,
    [UART_LCR]	= 5,
    [UART_MCR]	= 6,
    };
#[no_mangle]
unsafe extern "C" fn au_serial_in(p: *mut uart_port, offset: c_uint) -> u32 {
    static u32 au_serial_in(struct uart_port *p, unsigned int offset)
    {
    if (offset >= ARRAY_SIZE(au_io_in_map))
    return UINT_MAX;
    offset = au_io_in_map[offset];
    return __raw_readl(p.membase + (offset << p.regshift));
    }
#[no_mangle]
unsafe extern "C" fn au_serial_out(p: *mut uart_port, offset: c_uint, value: u32) {
    static void au_serial_out(struct uart_port *p, unsigned int offset, u32 value)
    {
    if (offset >= ARRAY_SIZE(au_io_out_map))
    return;
    offset = au_io_out_map[offset];
    __raw_writel(value, p.membase + (offset << p.regshift));
    }
// Au1x00 haven't got a standard divisor latch
#[no_mangle]
unsafe extern "C" fn au_serial_dl_read(up: *mut uart_8250_port) -> u32 {
    static u32 au_serial_dl_read(struct uart_8250_port *up)
    {
    return __raw_readl(up.port.membase + RT288X_DL);
    }
#[no_mangle]
unsafe extern "C" fn au_serial_dl_write(up: *mut uart_8250_port, value: u32) {
    static void au_serial_dl_write(struct uart_8250_port *up, u32 value)
    {
    __raw_writel(value, up.port.membase + RT288X_DL);
    }
#[no_mangle]
pub unsafe extern "C" fn au_platform_setup(p: *mut plat_serial8250_port) -> c_int {
    int au_platform_setup(struct plat_serial8250_port *p)
    {
    p.iotype = UPIO_AU;
    p.serial_in = au_serial_in;
    p.serial_out = au_serial_out;
    p.dl_read = au_serial_dl_read;
    p.dl_write = au_serial_dl_write;
    p.mapsize = 0x1000;
    p.bugs |= UART_BUG_NOMSR;
    return 0;
    }
    EXPORT_SYMBOL_GPL(au_platform_setup);
#[no_mangle]
pub unsafe extern "C" fn rt288x_setup(p: *mut uart_port) -> c_int {
    int rt288x_setup(struct uart_port *p)
    {
    struct uart_8250_port *up = up_to_u8250p(p);
    p.iotype = UPIO_AU;
    p.serial_in = au_serial_in;
    p.serial_out = au_serial_out;
    up.dl_read = au_serial_dl_read;
    up.dl_write = au_serial_dl_write;
    p.mapsize = 0x100;
    up.bugs |= UART_BUG_NOMSR;
    return 0;
    }
    EXPORT_SYMBOL_GPL(rt288x_setup);

#[no_mangle]
unsafe extern "C" fn au_putc(port: *mut uart_port, c: c_uchar) {
    static void au_putc(struct uart_port *port, unsigned char c)
    {
    unsigned int status;
    au_serial_out(port, UART_TX, c);
    for (;;) {
    status = au_serial_in(port, UART_LSR);
    if (uart_lsr_tx_empty(status))
    break;
    cpu_relax();
    }
    }
    static void au_early_serial8250_write(struct console *console,
    const char *s, unsigned int count)
    {
    struct earlycon_device *device = console.data;
    struct uart_port *port = &device.port;
    uart_console_write(port, s, count, au_putc);
    }
#[no_mangle]
unsafe extern "C" fn early_au_setup(dev: *mut earlycon_device, opt: *const c_char) -> int __init {
    static int __init early_au_setup(struct earlycon_device *dev, const char *opt)
    {
    rt288x_setup(&dev.port);
    dev.con.write = au_early_serial8250_write;
    return 0;
    }
    OF_EARLYCON_DECLARE(palmchip, "ralink,rt2880-uart", early_au_setup);

    MODULE_DESCRIPTION("RT288x/Au1xxx UART driver");
    MODULE_LICENSE("GPL");

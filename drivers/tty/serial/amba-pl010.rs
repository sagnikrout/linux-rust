//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/amba-pl010.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Driver for AMBA serial ports
//
// Based on drivers/char/serial.c, by Linus Torvalds, Theodore Ts'o.
//
// Copyright 1999 ARM Limited
// Copyright (C) 2000 Deep Blue Solutions Ltd.
//
// This is a generic driver for ARM AMBA-type serial ports.  They
// have a lot of 16550-like features, but are not register compatible.
// Note that although they do have CTS, DCD and DSR inputs, they do
// not have an RI input, nor do they have DTR or RTS outputs.  If
// required, these have to be supplied via some other means (eg, GPIO)
// and hooked into this driver.
//

pub const UART_NR: c_int = 8;
pub const SERIAL_AMBA_MAJOR: c_int = 204;
pub const SERIAL_AMBA_MINOR: c_int = 16;

pub const AMBA_ISR_PASS_LIMIT: c_int = 256;

pub const UART_DUMMY_RSR_RX: c_int = 256;
pub const UART_PORT_SIZE: c_int = 64;
//
// We wrap our port structure around the generic uart_port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uart_amba_port {
    pub port: uart_port,
    pub clk: *mut clk,
    pub dev: *mut amba_device,
    pub data: *mut amba_pl010_data,
    pub old_status: c_uint,
}

#[no_mangle]
unsafe extern "C" fn pl010_stop_tx(port: *mut uart_port) {
    static void pl010_stop_tx(struct uart_port *port)
    {
    struct uart_amba_port *uap =
    container_of(port, struct uart_amba_port, port);
    unsigned int cr;
    cr = readb(uap.port.membase + UART010_CR);
    cr &= ~UART010_CR_TIE;
    writel(cr, uap.port.membase + UART010_CR);
    }
#[no_mangle]
unsafe extern "C" fn pl010_start_tx(port: *mut uart_port) {
    static void pl010_start_tx(struct uart_port *port)
    {
    struct uart_amba_port *uap =
    container_of(port, struct uart_amba_port, port);
    unsigned int cr;
    cr = readb(uap.port.membase + UART010_CR);
    cr |= UART010_CR_TIE;
    writel(cr, uap.port.membase + UART010_CR);
    }
#[no_mangle]
unsafe extern "C" fn pl010_stop_rx(port: *mut uart_port) {
    static void pl010_stop_rx(struct uart_port *port)
    {
    struct uart_amba_port *uap =
    container_of(port, struct uart_amba_port, port);
    unsigned int cr;
    cr = readb(uap.port.membase + UART010_CR);
    cr &= ~(UART010_CR_RIE | UART010_CR_RTIE);
    writel(cr, uap.port.membase + UART010_CR);
    }
#[no_mangle]
unsafe extern "C" fn pl010_disable_ms(port: *mut uart_port) {
    static void pl010_disable_ms(struct uart_port *port)
    {
    struct uart_amba_port *uap = (struct uart_amba_port *)port;
    unsigned int cr;
    cr = readb(uap.port.membase + UART010_CR);
    cr &= ~UART010_CR_MSIE;
    writel(cr, uap.port.membase + UART010_CR);
    }
#[no_mangle]
unsafe extern "C" fn pl010_enable_ms(port: *mut uart_port) {
    static void pl010_enable_ms(struct uart_port *port)
    {
    struct uart_amba_port *uap =
    container_of(port, struct uart_amba_port, port);
    unsigned int cr;
    cr = readb(uap.port.membase + UART010_CR);
    cr |= UART010_CR_MSIE;
    writel(cr, uap.port.membase + UART010_CR);
    }
#[no_mangle]
unsafe extern "C" fn pl010_rx_chars(port: *mut uart_port) {
    static void pl010_rx_chars(struct uart_port *port)
    {
    unsigned int status, rsr, max_count = 256;
    u8 ch, flag;
    status = readb(port.membase + UART01x_FR);
    while (UART_RX_DATA(status) && max_count--) {
    ch = readb(port.membase + UART01x_DR);
    flag = TTY_NORMAL;
    port.icount.rx++;
//
// Note that the error handling code is
// out of the main execution path
//
    rsr = readb(port.membase + UART01x_RSR) | UART_DUMMY_RSR_RX;
    if (unlikely(rsr & UART01x_RSR_ANY)) {
    writel(0, port.membase + UART01x_ECR);
    if (rsr & UART01x_RSR_BE) {
    rsr &= ~(UART01x_RSR_FE | UART01x_RSR_PE);
    port.icount.brk++;
    if (uart_handle_break(port))
    goto ignore_char;
    } else if (rsr & UART01x_RSR_PE)
    port.icount.parity++;
#[no_mangle]
pub unsafe extern "C" fn if(UART01x_RSR_FE: rsr &) -> else {
    else if (rsr & UART01x_RSR_FE)
    port.icount.frame++;
    if (rsr & UART01x_RSR_OE)
    port.icount.overrun++;
    rsr &= port.read_status_mask;
    if (rsr & UART01x_RSR_BE)
    flag = TTY_BREAK;
#[no_mangle]
pub unsafe extern "C" fn if(UART01x_RSR_PE: rsr &) -> else {
    else if (rsr & UART01x_RSR_PE)
    flag = TTY_PARITY;
#[no_mangle]
pub unsafe extern "C" fn if(UART01x_RSR_FE: rsr &) -> else {
    else if (rsr & UART01x_RSR_FE)
    flag = TTY_FRAME;
    }
    if (uart_handle_sysrq_char(port, ch))
    goto ignore_char;
    uart_insert_char(port, rsr, UART01x_RSR_OE, ch, flag);
    ignore_char:
    status = readb(port.membase + UART01x_FR);
    }
    tty_flip_buffer_push(&port.state.port);
    }
#[no_mangle]
unsafe extern "C" fn pl010_tx_chars(port: *mut uart_port) {
    static void pl010_tx_chars(struct uart_port *port)
    {
    u8 ch;
    uart_port_tx_limited(port, ch, port.fifosize >> 1,
    true,
    writel(ch, port.membase + UART01x_DR),
    ({}));
    }
#[no_mangle]
unsafe extern "C" fn pl010_modem_status(uap: *mut uart_amba_port) {
    static void pl010_modem_status(struct uart_amba_port *uap)
    {
    struct uart_port *port = &uap.port;
    unsigned int status, delta;
    writel(0, port.membase + UART010_ICR);
    status = readb(port.membase + UART01x_FR) & UART01x_FR_MODEM_ANY;
    delta = status ^ uap.old_status;
    uap.old_status = status;
    if (!delta)
    return;
    if (delta & UART01x_FR_DCD)
    uart_handle_dcd_change(port, status & UART01x_FR_DCD);
    if (delta & UART01x_FR_DSR)
    port.icount.dsr++;
    if (delta & UART01x_FR_CTS)
    uart_handle_cts_change(port, status & UART01x_FR_CTS);
    wake_up_interruptible(&port.state.port.delta_msr_wait);
    }
#[no_mangle]
unsafe extern "C" fn pl010_int(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pl010_int(int irq, void *dev_id)
    {
    struct uart_amba_port *uap = dev_id;
    struct uart_port *port = &uap.port;
    unsigned int status, pass_counter = AMBA_ISR_PASS_LIMIT;
    let mut handled: c_int = 0;
    uart_port_lock(port);
    status = readb(port.membase + UART010_IIR);
    if (status) {
    do {
    if (status & (UART010_IIR_RTIS | UART010_IIR_RIS))
    pl010_rx_chars(port);
    if (status & UART010_IIR_MIS)
    pl010_modem_status(uap);
    if (status & UART010_IIR_TIS)
    pl010_tx_chars(port);
    if (pass_counter-- == 0)
    break;
    status = readb(port.membase + UART010_IIR);
    } while (status & (UART010_IIR_RTIS | UART010_IIR_RIS |
    UART010_IIR_TIS));
    handled = 1;
    }
    uart_port_unlock(port);
    return IRQ_RETVAL(handled);
    }
#[no_mangle]
unsafe extern "C" fn pl010_tx_empty(port: *mut uart_port) -> c_uint {
    static unsigned int pl010_tx_empty(struct uart_port *port)
    {
    let mut status: c_uint = readb(port.membase + UART01x_FR);
    return status & UART01x_FR_BUSY ? 0 : TIOCSER_TEMT;
    }
#[no_mangle]
unsafe extern "C" fn pl010_get_mctrl(port: *mut uart_port) -> c_uint {
    static unsigned int pl010_get_mctrl(struct uart_port *port)
    {
    let mut result: c_uint = 0;
    unsigned int status;
    status = readb(port.membase + UART01x_FR);
    if (status & UART01x_FR_DCD)
    result |= TIOCM_CAR;
    if (status & UART01x_FR_DSR)
    result |= TIOCM_DSR;
    if (status & UART01x_FR_CTS)
    result |= TIOCM_CTS;
    return result;
    }
#[no_mangle]
unsafe extern "C" fn pl010_set_mctrl(port: *mut uart_port, mctrl: c_uint) {
    static void pl010_set_mctrl(struct uart_port *port, unsigned int mctrl)
    {
    struct uart_amba_port *uap =
    container_of(port, struct uart_amba_port, port);
    if (uap.data)
    uap.data.set_mctrl(uap.dev, port.membase, mctrl);
    }
#[no_mangle]
unsafe extern "C" fn pl010_break_ctl(port: *mut uart_port, break_state: c_int) {
    static void pl010_break_ctl(struct uart_port *port, int break_state)
    {
    unsigned long flags;
    unsigned int lcr_h;
    uart_port_lock_irqsave(port, &flags);
    lcr_h = readb(port.membase + UART010_LCRH);
    if (break_state == -1)
    lcr_h |= UART01x_LCRH_BRK;
    else
    lcr_h &= ~UART01x_LCRH_BRK;
    writel(lcr_h, port.membase + UART010_LCRH);
    uart_port_unlock_irqrestore(port, flags);
    }
#[no_mangle]
unsafe extern "C" fn pl010_startup(port: *mut uart_port) -> c_int {
    static int pl010_startup(struct uart_port *port)
    {
    struct uart_amba_port *uap =
    container_of(port, struct uart_amba_port, port);
    int retval;
//
// Try to enable the clock producer.
//
    retval = clk_prepare_enable(uap.clk);
    if (retval)
    goto out;
    port.uartclk = clk_get_rate(uap.clk);
//
// Allocate the IRQ
//
    retval = request_irq(port.irq, pl010_int, 0, "uart-pl010", uap);
    if (retval)
    goto clk_dis;
//
// initialise the old status of the modem signals
//
    uap.old_status = readb(port.membase + UART01x_FR) & UART01x_FR_MODEM_ANY;
//
// Finally, enable interrupts
//
    writel(UART01x_CR_UARTEN | UART010_CR_RIE | UART010_CR_RTIE,
    port.membase + UART010_CR);
    return 0;
    clk_dis:
    clk_disable_unprepare(uap.clk);
    out:
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn pl010_shutdown(port: *mut uart_port) {
    static void pl010_shutdown(struct uart_port *port)
    {
    struct uart_amba_port *uap =
    container_of(port, struct uart_amba_port, port);
//
// Free the interrupt
//
    free_irq(port.irq, uap);
//
// disable all interrupts, disable the port
//
    writel(0, port.membase + UART010_CR);
// disable break condition and fifos
    writel(readb(port.membase + UART010_LCRH) &
    ~(UART01x_LCRH_BRK | UART01x_LCRH_FEN),
    port.membase + UART010_LCRH);
//
// Shut down the clock producer
//
    clk_disable_unprepare(uap.clk);
    }
    static void
    pl010_set_termios(struct uart_port *port, struct ktermios *termios,
    const struct ktermios *old)
    {
    unsigned int lcr_h, old_cr;
    unsigned long flags;
    unsigned int baud, quot;
//
// Ask the core to calculate the divisor for us.
//
    baud = uart_get_baud_rate(port, termios, old, 0, port.uartclk / 16);
    quot = uart_get_divisor(port, baud);
    switch (termios.c_cflag & CSIZE) {
    case CS5:
    lcr_h = UART01x_LCRH_WLEN_5;
    break;
    case CS6:
    lcr_h = UART01x_LCRH_WLEN_6;
    break;
    case CS7:
    lcr_h = UART01x_LCRH_WLEN_7;
    break;
    default: // CS8
    lcr_h = UART01x_LCRH_WLEN_8;
    break;
    }
    if (termios.c_cflag & CSTOPB)
    lcr_h |= UART01x_LCRH_STP2;
    if (termios.c_cflag & PARENB) {
    lcr_h |= UART01x_LCRH_PEN;
    if (!(termios.c_cflag & PARODD))
    lcr_h |= UART01x_LCRH_EPS;
    }
    if (port.fifosize > 1)
    lcr_h |= UART01x_LCRH_FEN;
    uart_port_lock_irqsave(port, &flags);
//
// Update the per-port timeout.
//
    uart_update_timeout(port, termios.c_cflag, baud);
    port.read_status_mask = UART01x_RSR_OE;
    if (termios.c_iflag & INPCK)
    port.read_status_mask |= UART01x_RSR_FE | UART01x_RSR_PE;
    if (termios.c_iflag & (IGNBRK | BRKINT | PARMRK))
    port.read_status_mask |= UART01x_RSR_BE;
//
// Characters to ignore
//
    port.ignore_status_mask = 0;
    if (termios.c_iflag & IGNPAR)
    port.ignore_status_mask |= UART01x_RSR_FE | UART01x_RSR_PE;
    if (termios.c_iflag & IGNBRK) {
    port.ignore_status_mask |= UART01x_RSR_BE;
//
// If we're ignoring parity and break indicators,
// ignore overruns too (for real raw support).
//
    if (termios.c_iflag & IGNPAR)
    port.ignore_status_mask |= UART01x_RSR_OE;
    }
//
// Ignore all characters if CREAD is not set.
//
    if ((termios.c_cflag & CREAD) == 0)
    port.ignore_status_mask |= UART_DUMMY_RSR_RX;
    old_cr = readb(port.membase + UART010_CR) & ~UART010_CR_MSIE;
    if (UART_ENABLE_MS(port, termios.c_cflag))
    old_cr |= UART010_CR_MSIE;
// Set baud rate
    quot -= 1;
    writel((quot & 0xf00) >> 8, port.membase + UART010_LCRM);
    writel(quot & 0xff, port.membase + UART010_LCRL);
//
// ----------v----------v----------v----------v-----
// NOTE: MUST BE WRITTEN AFTER UARTLCR_M & UARTLCR_L
// ----------^----------^----------^----------^-----
//
    writel(lcr_h, port.membase + UART010_LCRH);
    writel(old_cr, port.membase + UART010_CR);
    uart_port_unlock_irqrestore(port, flags);
    }
#[no_mangle]
unsafe extern "C" fn pl010_set_ldisc(port: *mut uart_port, termios: *mut ktermios) {
    static void pl010_set_ldisc(struct uart_port *port, struct ktermios *termios)
    {
    if (termios.c_line == N_PPS) {
    port.flags |= UPF_HARDPPS_CD;
    uart_port_lock_irq(port);
    pl010_enable_ms(port);
    uart_port_unlock_irq(port);
    } else {
    port.flags &= ~UPF_HARDPPS_CD;
    if (!UART_ENABLE_MS(port, termios.c_cflag)) {
    uart_port_lock_irq(port);
    pl010_disable_ms(port);
    uart_port_unlock_irq(port);
    }
    }
    }
    static const char *pl010_type(struct uart_port *port)
    {
    return port.type == PORT_AMBA ? "AMBA" : core::ptr::null_mut();
    }
//
// Release the memory region(s) being used by 'port'
//
#[no_mangle]
unsafe extern "C" fn pl010_release_port(port: *mut uart_port) {
    static void pl010_release_port(struct uart_port *port)
    {
    release_mem_region(port.mapbase, UART_PORT_SIZE);
    }
//
// Request the memory region(s) being used by 'port'
//
#[no_mangle]
unsafe extern "C" fn pl010_request_port(port: *mut uart_port) -> c_int {
    static int pl010_request_port(struct uart_port *port)
    {
#[no_mangle]
pub unsafe extern "C" fn request_mem_region(_arg: port->mapbase, _arg: UART_PORT_SIZE, _arg: "uart-pl010") -> return {
    return request_mem_region(port.mapbase, UART_PORT_SIZE, "uart-pl010")
    != core::ptr::null_mut() ? 0 : -EBUSY;
    }
//
// Configure/autoconfigure the port.
//
#[no_mangle]
unsafe extern "C" fn pl010_config_port(port: *mut uart_port, flags: c_int) {
    static void pl010_config_port(struct uart_port *port, int flags)
    {
    if (flags & UART_CONFIG_TYPE) {
    port.type = PORT_AMBA;
    pl010_request_port(port);
    }
    }
//
// verify the new serial_struct (for TIOCSSERIAL).
//
#[no_mangle]
unsafe extern "C" fn pl010_verify_port(port: *mut uart_port, ser: *mut serial_struct) -> c_int {
    static int pl010_verify_port(struct uart_port *port, struct serial_struct *ser)
    {
    let mut ret: c_int = 0;
    if (ser.type != PORT_UNKNOWN && ser.type != PORT_AMBA)
    ret = -EINVAL;
    if (ser.irq < 0 || ser.irq >= irq_get_nr_irqs())
    ret = -EINVAL;
    if (ser.baud_base < 9600)
    ret = -EINVAL;
    return ret;
    }
    static const struct uart_ops amba_pl010_pops = {
    .tx_empty	= pl010_tx_empty,
    .set_mctrl	= pl010_set_mctrl,
    .get_mctrl	= pl010_get_mctrl,
    .stop_tx	= pl010_stop_tx,
    .start_tx	= pl010_start_tx,
    .stop_rx	= pl010_stop_rx,
    .enable_ms	= pl010_enable_ms,
    .break_ctl	= pl010_break_ctl,
    .startup	= pl010_startup,
    .shutdown	= pl010_shutdown,
    .set_termios	= pl010_set_termios,
    .set_ldisc	= pl010_set_ldisc,
    .type		= pl010_type,
    .release_port	= pl010_release_port,
    .request_port	= pl010_request_port,
    .config_port	= pl010_config_port,
    .verify_port	= pl010_verify_port,
    };
    static struct uart_amba_port *amba_ports[UART_NR];

#[no_mangle]
unsafe extern "C" fn pl010_console_putchar(port: *mut uart_port, ch: c_uchar) {
    static void pl010_console_putchar(struct uart_port *port, unsigned char ch)
    {
    unsigned int status;
    do {
    status = readb(port.membase + UART01x_FR);
    barrier();
    } while (!UART_TX_READY(status));
    writel(ch, port.membase + UART01x_DR);
    }
    static void
    pl010_console_write(struct console *co, const char *s, unsigned int count)
    {
    struct uart_amba_port *uap = amba_ports[co.index];
    struct uart_port *port = &uap.port;
    unsigned int status, old_cr;
    clk_enable(uap.clk);
//
// First save the CR then disable the interrupts
//
    old_cr = readb(port.membase + UART010_CR);
    writel(UART01x_CR_UARTEN, port.membase + UART010_CR);
    uart_console_write(port, s, count, pl010_console_putchar);
//
// Finally, wait for transmitter to become empty
// and restore the TCR
//
    do {
    status = readb(port.membase + UART01x_FR);
    barrier();
    } while (status & UART01x_FR_BUSY);
    writel(old_cr, port.membase + UART010_CR);
    clk_disable(uap.clk);
    }
    static void __init
    pl010_console_get_options(struct uart_amba_port *uap, int *baud,
    int *parity, int *bits)
    {
    if (readb(uap.port.membase + UART010_CR) & UART01x_CR_UARTEN) {
    unsigned int lcr_h, quot;
    lcr_h = readb(uap.port.membase + UART010_LCRH);
// parity = 'n';
    if (lcr_h & UART01x_LCRH_PEN) {
    if (lcr_h & UART01x_LCRH_EPS)
// parity = 'e';
    else
// parity = 'o';
    }
    if ((lcr_h & 0x60) == UART01x_LCRH_WLEN_7)
// bits = 7;
    else
// bits = 8;
    quot = readb(uap.port.membase + UART010_LCRL) |
    readb(uap.port.membase + UART010_LCRM) << 8;
// baud = uap->port.uartclk / (16 * (quot + 1));
    }
    }
#[no_mangle]
unsafe extern "C" fn pl010_console_setup(co: *mut console, options: *mut c_char) -> int __init {
    static int __init pl010_console_setup(struct console *co, char *options)
    {
    struct uart_amba_port *uap;
    let mut baud: c_int = 38400;
    let mut bits: c_int = 8;
    let mut parity: c_int = 'n';
    let mut flow: c_int = 'n';
    int ret;
//
// Check whether an invalid uart number has been specified, and
// if so, search for the first available port that does have
// console support.
//
    if (co.index >= UART_NR)
    co.index = 0;
    uap = amba_ports[co.index];
    if (!uap)
    return -ENODEV;
    ret = clk_prepare(uap.clk);
    if (ret)
    return ret;
    uap.port.uartclk = clk_get_rate(uap.clk);
    if (options)
    uart_parse_options(options, &baud, &parity, &bits, &flow);
    else
    pl010_console_get_options(uap, &baud, &parity, &bits);
    return uart_set_options(&uap.port, co, baud, parity, bits, flow);
    }
    static struct uart_driver amba_reg;
    static struct console amba_console = {
    .name		= "ttyAM",
    .write		= pl010_console_write,
    .device		= uart_console_device,
    .setup		= pl010_console_setup,
    .flags		= CON_PRINTBUFFER,
    .index		= -1,
    .data		= &amba_reg,
    };

    static DEFINE_MUTEX(amba_reg_lock);
    static struct uart_driver amba_reg = {
    .owner			= THIS_MODULE,
    .driver_name		= "ttyAM",
    .dev_name		= "ttyAM",
    .major			= SERIAL_AMBA_MAJOR,
    .minor			= SERIAL_AMBA_MINOR,
    .nr			= UART_NR,
    .cons			= AMBA_CONSOLE,
    };
#[no_mangle]
unsafe extern "C" fn pl010_probe(dev: *mut amba_device, id: *const amba_id) -> c_int {
    static int pl010_probe(struct amba_device *dev, const struct amba_id *id)
    {
    struct uart_amba_port *uap;
    void __iomem *base;
    int i, ret;
    for (i = 0; i < ARRAY_SIZE(amba_ports); i++)
    if (amba_ports[i] == core::ptr::null_mut())
    break;
    if (i == ARRAY_SIZE(amba_ports))
    return -EBUSY;
    uap = devm_kzalloc(&dev.dev, sizeof(struct uart_amba_port),
    GFP_KERNEL);
    if (!uap)
    return -ENOMEM;
    base = devm_ioremap(&dev.dev, dev.res.start,
    resource_size(&dev.res));
    if (!base)
    return -ENOMEM;
    uap.clk = devm_clk_get(&dev.dev, core::ptr::null_mut());
    if (IS_ERR(uap.clk))
    return PTR_ERR(uap.clk);
    uap.port.dev = &dev.dev;
    uap.port.mapbase = dev.res.start;
    uap.port.membase = base;
    uap.port.iotype = UPIO_MEM;
    uap.port.irq = dev.irq[0];
    uap.port.fifosize = 16;
    uap.port.has_sysrq = IS_ENABLED(CONFIG_SERIAL_AMBA_PL010_CONSOLE);
    uap.port.ops = &amba_pl010_pops;
    uap.port.flags = UPF_BOOT_AUTOCONF;
    uap.port.line = i;
    uap.dev = dev;
    uap.data = dev_get_platdata(&dev.dev);
    amba_ports[i] = uap;
    amba_set_drvdata(dev, uap);
    mutex_lock(&amba_reg_lock);
    if (!amba_reg.state) {
    ret = uart_register_driver(&amba_reg);
    if (ret < 0) {
    mutex_unlock(&amba_reg_lock);
    dev_err(uap.port.dev,
    "Failed to register AMBA-PL010 driver\n");
    return ret;
    }
    }
    mutex_unlock(&amba_reg_lock);
    ret = uart_add_one_port(&amba_reg, &uap.port);
    if (ret)
    amba_ports[i] = core::ptr::null_mut();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pl010_remove(dev: *mut amba_device) {
    static void pl010_remove(struct amba_device *dev)
    {
    struct uart_amba_port *uap = amba_get_drvdata(dev);
    int i;
    let mut busy: bool = false;
    uart_remove_one_port(&amba_reg, &uap.port);
    for (i = 0; i < ARRAY_SIZE(amba_ports); i++)
    if (amba_ports[i] == uap)
    amba_ports[i] = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: amba_ports[i]) -> else {
    else if (amba_ports[i])
    busy = true;
    if (!busy)
    uart_unregister_driver(&amba_reg);
    }

#[no_mangle]
unsafe extern "C" fn pl010_suspend(dev: *mut device) -> c_int {
    static int pl010_suspend(struct device *dev)
    {
    struct uart_amba_port *uap = dev_get_drvdata(dev);
    if (uap)
    uart_suspend_port(&amba_reg, &uap.port);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pl010_resume(dev: *mut device) -> c_int {
    static int pl010_resume(struct device *dev)
    {
    struct uart_amba_port *uap = dev_get_drvdata(dev);
    if (uap)
    uart_resume_port(&amba_reg, &uap.port);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(pl010_dev_pm_ops, pl010_suspend, pl010_resume);
    static const struct amba_id pl010_ids[] = {
    {
    .id	= 0x00041010,
    .mask	= 0x000fffff,
    },
    { 0, 0 },
    };
    MODULE_DEVICE_TABLE(amba, pl010_ids);
    static struct amba_driver pl010_driver = {
    .drv = {
    .name	= "uart-pl010",
    .pm	= &pl010_dev_pm_ops,
    },
    .id_table	= pl010_ids,
    .probe		= pl010_probe,
    .remove		= pl010_remove,
    };
#[no_mangle]
unsafe extern "C" fn pl010_init() -> int __init {
    static int __init pl010_init(void)
    {
    printk(KERN_INFO "Serial: AMBA driver\n");
    return  amba_driver_register(&pl010_driver);
    }
#[no_mangle]
unsafe extern "C" fn pl010_exit() -> void __exit {
    static void __exit pl010_exit(void)
    {
    amba_driver_unregister(&pl010_driver);
    }
    module_init(pl010_init);
    module_exit(pl010_exit);
    MODULE_AUTHOR("ARM Ltd/Deep Blue Solutions Ltd");
    MODULE_DESCRIPTION("ARM AMBA serial port driver");
    MODULE_LICENSE("GPL");

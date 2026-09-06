//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/ar933x_uart.c
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
// Atheros AR933X SoC built-in UART driver
//
// Copyright (C) 2011 Gabor Juhos <juhosg@openwrt.org>
//
// Based on drivers/char/serial.c, by Linus Torvalds, Theodore Ts'o.
//

pub const AR933X_UART_MAX_SCALE: c_uint = 0xff;
pub const AR933X_UART_MAX_STEP: c_uint = 0xffff;
pub const AR933X_UART_MIN_BAUD: c_int = 300;
pub const AR933X_UART_MAX_BAUD: c_int = 3000000;
pub const AR933X_DUMMY_STATUS_RD: c_uint = 0x01;
    static struct uart_driver ar933x_uart_driver;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar933x_uart_port {
    pub port: uart_port,
    pub /: *mut *mut unsigned int ier; / shadow Interrupt Enable Register,
    pub min_baud: c_uint,
    pub max_baud: c_uint,
    pub clk: *mut clk,
    pub gpios: *mut mctrl_gpios,
    pub rts_gpiod: *mut gpio_desc,
}

    static inline unsigned int ar933x_uart_read(struct ar933x_uart_port *up,
    int offset)
    {
    return readl(up.port.membase + offset);
    }
    static inline void ar933x_uart_write(struct ar933x_uart_port *up,
    int offset, unsigned int value)
    {
    writel(value, up.port.membase + offset);
    }
    static inline void ar933x_uart_rmw(struct ar933x_uart_port *up,
    unsigned int offset,
    unsigned int mask,
    unsigned int val)
    {
    unsigned int t;
    t = ar933x_uart_read(up, offset);
    t &= ~mask;
    t |= val;
    ar933x_uart_write(up, offset, t);
    }
    static inline void ar933x_uart_rmw_set(struct ar933x_uart_port *up,
    unsigned int offset,
    unsigned int val)
    {
    ar933x_uart_rmw(up, offset, 0, val);
    }
    static inline void ar933x_uart_rmw_clear(struct ar933x_uart_port *up,
    unsigned int offset,
    unsigned int val)
    {
    ar933x_uart_rmw(up, offset, val, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn ar933x_uart_start_tx_interrupt(up: *mut ar933x_uart_port) {
    static inline void ar933x_uart_start_tx_interrupt(struct ar933x_uart_port *up)
    {
    up.ier |= AR933X_UART_INT_TX_EMPTY;
    ar933x_uart_write(up, AR933X_UART_INT_EN_REG, up.ier);
    }
#[no_mangle]
pub unsafe extern "C" fn ar933x_uart_stop_tx_interrupt(up: *mut ar933x_uart_port) {
    static inline void ar933x_uart_stop_tx_interrupt(struct ar933x_uart_port *up)
    {
    up.ier &= ~AR933X_UART_INT_TX_EMPTY;
    ar933x_uart_write(up, AR933X_UART_INT_EN_REG, up.ier);
    }
#[no_mangle]
pub unsafe extern "C" fn ar933x_uart_start_rx_interrupt(up: *mut ar933x_uart_port) {
    static inline void ar933x_uart_start_rx_interrupt(struct ar933x_uart_port *up)
    {
    up.ier |= AR933X_UART_INT_RX_VALID;
    ar933x_uart_write(up, AR933X_UART_INT_EN_REG, up.ier);
    }
#[no_mangle]
pub unsafe extern "C" fn ar933x_uart_stop_rx_interrupt(up: *mut ar933x_uart_port) {
    static inline void ar933x_uart_stop_rx_interrupt(struct ar933x_uart_port *up)
    {
    up.ier &= ~AR933X_UART_INT_RX_VALID;
    ar933x_uart_write(up, AR933X_UART_INT_EN_REG, up.ier);
    }
#[no_mangle]
pub unsafe extern "C" fn ar933x_uart_putc(up: *mut ar933x_uart_port, ch: c_int) {
    static inline void ar933x_uart_putc(struct ar933x_uart_port *up, int ch)
    {
    unsigned int rdata;
    rdata = ch & AR933X_UART_DATA_TX_RX_MASK;
    rdata |= AR933X_UART_DATA_TX_CSR;
    ar933x_uart_write(up, AR933X_UART_DATA_REG, rdata);
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_tx_empty(port: *mut uart_port) -> c_uint {
    static unsigned int ar933x_uart_tx_empty(struct uart_port *port)
    {
    struct ar933x_uart_port *up =
    container_of(port, struct ar933x_uart_port, port);
    unsigned long flags;
    unsigned int rdata;
    uart_port_lock_irqsave(&up.port, &flags);
    rdata = ar933x_uart_read(up, AR933X_UART_DATA_REG);
    uart_port_unlock_irqrestore(&up.port, flags);
    return (rdata & AR933X_UART_DATA_TX_CSR) ? 0 : TIOCSER_TEMT;
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_get_mctrl(port: *mut uart_port) -> c_uint {
    static unsigned int ar933x_uart_get_mctrl(struct uart_port *port)
    {
    struct ar933x_uart_port *up =
    container_of(port, struct ar933x_uart_port, port);
    let mut ret: c_int = TIOCM_CTS | TIOCM_DSR | TIOCM_CAR;
    mctrl_gpio_get(up.gpios, &ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_set_mctrl(port: *mut uart_port, mctrl: c_uint) {
    static void ar933x_uart_set_mctrl(struct uart_port *port, unsigned int mctrl)
    {
    struct ar933x_uart_port *up =
    container_of(port, struct ar933x_uart_port, port);
    mctrl_gpio_set(up.gpios, mctrl);
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_start_tx(port: *mut uart_port) {
    static void ar933x_uart_start_tx(struct uart_port *port)
    {
    struct ar933x_uart_port *up =
    container_of(port, struct ar933x_uart_port, port);
    ar933x_uart_start_tx_interrupt(up);
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_wait_tx_complete(up: *mut ar933x_uart_port) {
    static void ar933x_uart_wait_tx_complete(struct ar933x_uart_port *up)
    {
    unsigned int status;
    let mut timeout: c_uint = 60000;
// Wait up to 60ms for the character(s) to be sent.
    do {
    status = ar933x_uart_read(up, AR933X_UART_CS_REG);
    if (--timeout == 0)
    break;
    udelay(1);
    } while (status & AR933X_UART_CS_TX_BUSY);
    if (timeout == 0)
    dev_err(up.port.dev, "waiting for TX timed out\n");
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_rx_flush(up: *mut ar933x_uart_port) {
    static void ar933x_uart_rx_flush(struct ar933x_uart_port *up)
    {
    unsigned int status;
// clear RX_VALID interrupt
    ar933x_uart_write(up, AR933X_UART_INT_REG, AR933X_UART_INT_RX_VALID);
// remove characters from the RX FIFO
    do {
    ar933x_uart_write(up, AR933X_UART_DATA_REG, AR933X_UART_DATA_RX_CSR);
    status = ar933x_uart_read(up, AR933X_UART_DATA_REG);
    } while (status & AR933X_UART_DATA_RX_CSR);
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_stop_tx(port: *mut uart_port) {
    static void ar933x_uart_stop_tx(struct uart_port *port)
    {
    struct ar933x_uart_port *up =
    container_of(port, struct ar933x_uart_port, port);
    ar933x_uart_stop_tx_interrupt(up);
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_stop_rx(port: *mut uart_port) {
    static void ar933x_uart_stop_rx(struct uart_port *port)
    {
    struct ar933x_uart_port *up =
    container_of(port, struct ar933x_uart_port, port);
    ar933x_uart_stop_rx_interrupt(up);
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_break_ctl(port: *mut uart_port, break_state: c_int) {
    static void ar933x_uart_break_ctl(struct uart_port *port, int break_state)
    {
    struct ar933x_uart_port *up =
    container_of(port, struct ar933x_uart_port, port);
    unsigned long flags;
    uart_port_lock_irqsave(&up.port, &flags);
    if (break_state == -1)
    ar933x_uart_rmw_set(up, AR933X_UART_CS_REG,
    AR933X_UART_CS_TX_BREAK);
    else
    ar933x_uart_rmw_clear(up, AR933X_UART_CS_REG,
    AR933X_UART_CS_TX_BREAK);
    uart_port_unlock_irqrestore(&up.port, flags);
    }
//
// baudrate = (clk / (scale + 1)) * (step * (1 / 2^17))
//
    static unsigned long ar933x_uart_get_baud(unsigned int clk,
    unsigned int scale,
    unsigned int step)
    {
    u64 t;
    u32 div;
    div = (2 << 16) * (scale + 1);
    t = clk;
    t *= step;
    t += (div / 2);
    do_div(t, div);
    return t;
    }
    static void ar933x_uart_get_scale_step(unsigned int clk,
    unsigned int baud,
    unsigned int *scale,
    unsigned int *step)
    {
    unsigned int tscale;
    long min_diff;
// scale = 0;
// step = 0;
    min_diff = baud;
    for (tscale = 0; tscale < AR933X_UART_MAX_SCALE; tscale++) {
    u64 tstep;
    int diff;
    tstep = baud * (tscale + 1);
    tstep *= (2 << 16);
    do_div(tstep, clk);
    if (tstep > AR933X_UART_MAX_STEP)
    break;
    diff = abs(ar933x_uart_get_baud(clk, tscale, tstep) - baud);
    if (diff < min_diff) {
    min_diff = diff;
// scale = tscale;
// step = tstep;
    }
    }
    }
    static void ar933x_uart_set_termios(struct uart_port *port,
    struct ktermios *new,
    const struct ktermios *old)
    {
    struct ar933x_uart_port *up =
    container_of(port, struct ar933x_uart_port, port);
    unsigned int cs;
    unsigned long flags;
    unsigned int baud, scale, step;
// Only CS8 is supported
    new.c_cflag &= ~CSIZE;
    new.c_cflag |= CS8;
// Only one stop bit is supported
    new.c_cflag &= ~CSTOPB;
    cs = 0;
    if (new.c_cflag & PARENB) {
    if (!(new.c_cflag & PARODD))
    cs |= AR933X_UART_CS_PARITY_EVEN;
    else
    cs |= AR933X_UART_CS_PARITY_ODD;
    } else {
    cs |= AR933X_UART_CS_PARITY_NONE;
    }
// Mark/space parity is not supported
    new.c_cflag &= ~CMSPAR;
    baud = uart_get_baud_rate(port, new, old, up.min_baud, up.max_baud);
    ar933x_uart_get_scale_step(port.uartclk, baud, &scale, &step);
//
// Ok, we're now changing the port state. Do it with
// interrupts disabled.
//
    uart_port_lock_irqsave(&up.port, &flags);
// disable the UART
    ar933x_uart_rmw_clear(up, AR933X_UART_CS_REG,
    AR933X_UART_CS_IF_MODE_M << AR933X_UART_CS_IF_MODE_S);
// Update the per-port timeout.
    uart_update_timeout(port, new.c_cflag, baud);
    up.port.ignore_status_mask = 0;
// ignore all characters if CREAD is not set
    if ((new.c_cflag & CREAD) == 0)
    up.port.ignore_status_mask |= AR933X_DUMMY_STATUS_RD;
    ar933x_uart_write(up, AR933X_UART_CLOCK_REG,
    scale << AR933X_UART_CLOCK_SCALE_S | step);
// setup configuration register
    ar933x_uart_rmw(up, AR933X_UART_CS_REG, AR933X_UART_CS_PARITY_M, cs);
// enable host interrupt
    ar933x_uart_rmw_set(up, AR933X_UART_CS_REG,
    AR933X_UART_CS_HOST_INT_EN);
// enable RX and TX ready overide
    ar933x_uart_rmw_set(up, AR933X_UART_CS_REG,
    AR933X_UART_CS_TX_READY_ORIDE | AR933X_UART_CS_RX_READY_ORIDE);
// reenable the UART
    ar933x_uart_rmw(up, AR933X_UART_CS_REG,
    AR933X_UART_CS_IF_MODE_M << AR933X_UART_CS_IF_MODE_S,
    AR933X_UART_CS_IF_MODE_DCE << AR933X_UART_CS_IF_MODE_S);
    uart_port_unlock_irqrestore(&up.port, flags);
    if (tty_termios_baud_rate(new))
    tty_termios_encode_baud_rate(new, baud, baud);
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_rx_chars(up: *mut ar933x_uart_port) {
    static void ar933x_uart_rx_chars(struct ar933x_uart_port *up)
    {
    struct tty_port *port = &up.port.state.port;
    let mut max_count: c_int = 256;
    do {
    unsigned int rdata;
    unsigned char ch;
    rdata = ar933x_uart_read(up, AR933X_UART_DATA_REG);
    if ((rdata & AR933X_UART_DATA_RX_CSR) == 0)
    break;
// remove the character from the FIFO
    ar933x_uart_write(up, AR933X_UART_DATA_REG,
    AR933X_UART_DATA_RX_CSR);
    up.port.icount.rx++;
    ch = rdata & AR933X_UART_DATA_TX_RX_MASK;
    if (uart_prepare_sysrq_char(&up.port, ch))
    continue;
    if ((up.port.ignore_status_mask & AR933X_DUMMY_STATUS_RD) == 0)
    tty_insert_flip_char(port, ch, TTY_NORMAL);
    } while (max_count-- > 0);
    tty_flip_buffer_push(port);
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_tx_chars(up: *mut ar933x_uart_port) {
    static void ar933x_uart_tx_chars(struct ar933x_uart_port *up)
    {
    struct tty_port *tport = &up.port.state.port;
    struct serial_rs485 *rs485conf = &up.port.rs485;
    int count;
    let mut half_duplex_send: bool = false;
    if (uart_tx_stopped(&up.port))
    return;
    if ((rs485conf.flags & SER_RS485_ENABLED) &&
    (up.port.x_char || !kfifo_is_empty(&tport.xmit_fifo))) {
    ar933x_uart_stop_rx_interrupt(up);
    gpiod_set_value(up.rts_gpiod, !!(rs485conf.flags & SER_RS485_RTS_ON_SEND));
    half_duplex_send = true;
    }
    count = up.port.fifosize;
    do {
    unsigned int rdata;
    unsigned char c;
    rdata = ar933x_uart_read(up, AR933X_UART_DATA_REG);
    if ((rdata & AR933X_UART_DATA_TX_CSR) == 0)
    break;
    if (up.port.x_char) {
    ar933x_uart_putc(up, up.port.x_char);
    up.port.icount.tx++;
    up.port.x_char = 0;
    continue;
    }
    if (!uart_fifo_get(&up.port, &c))
    break;
    ar933x_uart_putc(up, c);
    } while (--count > 0);
    if (kfifo_len(&tport.xmit_fifo) < WAKEUP_CHARS)
    uart_write_wakeup(&up.port);
    if (!kfifo_is_empty(&tport.xmit_fifo)) {
    ar933x_uart_start_tx_interrupt(up);
    } else if (half_duplex_send) {
    ar933x_uart_wait_tx_complete(up);
    ar933x_uart_rx_flush(up);
    ar933x_uart_start_rx_interrupt(up);
    gpiod_set_value(up.rts_gpiod, !!(rs485conf.flags & SER_RS485_RTS_AFTER_SEND));
    }
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ar933x_uart_interrupt(int irq, void *dev_id)
    {
    struct ar933x_uart_port *up = dev_id;
    unsigned int status;
    status = ar933x_uart_read(up, AR933X_UART_CS_REG);
    if ((status & AR933X_UART_CS_HOST_INT) == 0)
    return IRQ_NONE;
    uart_port_lock(&up.port);
    status = ar933x_uart_read(up, AR933X_UART_INT_REG);
    status &= ar933x_uart_read(up, AR933X_UART_INT_EN_REG);
    if (status & AR933X_UART_INT_RX_VALID) {
    ar933x_uart_write(up, AR933X_UART_INT_REG,
    AR933X_UART_INT_RX_VALID);
    ar933x_uart_rx_chars(up);
    }
    if (status & AR933X_UART_INT_TX_EMPTY) {
    ar933x_uart_write(up, AR933X_UART_INT_REG,
    AR933X_UART_INT_TX_EMPTY);
    ar933x_uart_stop_tx_interrupt(up);
    ar933x_uart_tx_chars(up);
    }
    uart_unlock_and_check_sysrq(&up.port);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_startup(port: *mut uart_port) -> c_int {
    static int ar933x_uart_startup(struct uart_port *port)
    {
    struct ar933x_uart_port *up =
    container_of(port, struct ar933x_uart_port, port);
    unsigned long flags;
    int ret;
    ret = request_irq(up.port.irq, ar933x_uart_interrupt,
    up.port.irqflags, dev_name(up.port.dev), up);
    if (ret)
    return ret;
    uart_port_lock_irqsave(&up.port, &flags);
// Enable HOST interrupts
    ar933x_uart_rmw_set(up, AR933X_UART_CS_REG,
    AR933X_UART_CS_HOST_INT_EN);
// enable RX and TX ready overide
    ar933x_uart_rmw_set(up, AR933X_UART_CS_REG,
    AR933X_UART_CS_TX_READY_ORIDE | AR933X_UART_CS_RX_READY_ORIDE);
// Enable RX interrupts
    ar933x_uart_start_rx_interrupt(up);
    uart_port_unlock_irqrestore(&up.port, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_shutdown(port: *mut uart_port) {
    static void ar933x_uart_shutdown(struct uart_port *port)
    {
    struct ar933x_uart_port *up =
    container_of(port, struct ar933x_uart_port, port);
// Disable all interrupts
    up.ier = 0;
    ar933x_uart_write(up, AR933X_UART_INT_EN_REG, up.ier);
// Disable break condition
    ar933x_uart_rmw_clear(up, AR933X_UART_CS_REG,
    AR933X_UART_CS_TX_BREAK);
    free_irq(up.port.irq, up);
    }
    static const char *ar933x_uart_type(struct uart_port *port)
    {
    return (port.type == PORT_AR933X) ? "AR933X UART" : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_release_port(port: *mut uart_port) {
    static void ar933x_uart_release_port(struct uart_port *port)
    {
// Nothing to release ...
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_request_port(port: *mut uart_port) -> c_int {
    static int ar933x_uart_request_port(struct uart_port *port)
    {
// UARTs always present
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_config_port(port: *mut uart_port, flags: c_int) {
    static void ar933x_uart_config_port(struct uart_port *port, int flags)
    {
    if (flags & UART_CONFIG_TYPE)
    port.type = PORT_AR933X;
    }
    static int ar933x_uart_verify_port(struct uart_port *port,
    struct serial_struct *ser)
    {
    struct ar933x_uart_port *up =
    container_of(port, struct ar933x_uart_port, port);
    if (ser.type != PORT_UNKNOWN &&
    ser.type != PORT_AR933X)
    return -EINVAL;
    if (ser.irq < 0 || ser.irq >= NR_IRQS)
    return -EINVAL;
    if (ser.baud_base < up.min_baud ||
    ser.baud_base > up.max_baud)
    return -EINVAL;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ar933x_poll_get_char(port: *mut uart_port) -> c_int {
    static int ar933x_poll_get_char(struct uart_port *port)
    {
    struct ar933x_uart_port *up =
    container_of(port, struct ar933x_uart_port, port);
    unsigned int rdata;
    unsigned char ch;
    u32 imr;
// Disable all interrupts
    imr = ar933x_uart_read(up, AR933X_UART_INT_EN_REG);
    ar933x_uart_write(up, AR933X_UART_INT_EN_REG, 0);
    rdata = ar933x_uart_read(up, AR933X_UART_DATA_REG);
    if ((rdata & AR933X_UART_DATA_RX_CSR) == 0) {
// Enable interrupts
    ar933x_uart_write(up, AR933X_UART_INT_EN_REG, imr);
    return NO_POLL_CHAR;
    }
// remove the character from the FIFO
    ar933x_uart_write(up, AR933X_UART_DATA_REG,
    AR933X_UART_DATA_RX_CSR);
    ch = rdata & AR933X_UART_DATA_TX_RX_MASK;
// Enable interrupts
    ar933x_uart_write(up, AR933X_UART_INT_EN_REG, imr);
    return ch;
    }
#[no_mangle]
unsafe extern "C" fn ar933x_poll_put_char(port: *mut uart_port, c: c_uchar) {
    static void ar933x_poll_put_char(struct uart_port *port, unsigned char c)
    {
    struct ar933x_uart_port *up =
    container_of(port, struct ar933x_uart_port, port);
    u32 imr;
// Disable all interrupts
    imr = ar933x_uart_read(up, AR933X_UART_INT_EN_REG);
    ar933x_uart_write(up, AR933X_UART_INT_EN_REG, 0);
// Wait until FIFO is empty
    while (!(ar933x_uart_read(up, AR933X_UART_DATA_REG) & AR933X_UART_DATA_TX_CSR))
    cpu_relax();
// Write a character
    ar933x_uart_putc(up, c);
// Wait until FIFO is empty
    while (!(ar933x_uart_read(up, AR933X_UART_DATA_REG) & AR933X_UART_DATA_TX_CSR))
    cpu_relax();
// Enable interrupts
    ar933x_uart_write(up, AR933X_UART_INT_EN_REG, imr);
    }

    static const struct uart_ops ar933x_uart_ops = {
    .tx_empty	= ar933x_uart_tx_empty,
    .set_mctrl	= ar933x_uart_set_mctrl,
    .get_mctrl	= ar933x_uart_get_mctrl,
    .stop_tx	= ar933x_uart_stop_tx,
    .start_tx	= ar933x_uart_start_tx,
    .stop_rx	= ar933x_uart_stop_rx,
    .break_ctl	= ar933x_uart_break_ctl,
    .startup	= ar933x_uart_startup,
    .shutdown	= ar933x_uart_shutdown,
    .set_termios	= ar933x_uart_set_termios,
    .type		= ar933x_uart_type,
    .release_port	= ar933x_uart_release_port,
    .request_port	= ar933x_uart_request_port,
    .config_port	= ar933x_uart_config_port,
    .verify_port	= ar933x_uart_verify_port,

    .poll_get_char	= ar933x_poll_get_char,
    .poll_put_char	= ar933x_poll_put_char,

    };
    static int ar933x_config_rs485(struct uart_port *port, struct ktermios *termios,
    struct serial_rs485 *rs485conf)
    {
    struct ar933x_uart_port *up =
    container_of(port, struct ar933x_uart_port, port);
    if (port.rs485.flags & SER_RS485_ENABLED)
    gpiod_set_value(up.rts_gpiod,
    !!(rs485conf.flags & SER_RS485_RTS_AFTER_SEND));
    return 0;
    }

    static struct ar933x_uart_port *
    ar933x_console_ports[CONFIG_SERIAL_AR933X_NR_UARTS];
#[no_mangle]
unsafe extern "C" fn ar933x_uart_wait_xmitr(up: *mut ar933x_uart_port) {
    static void ar933x_uart_wait_xmitr(struct ar933x_uart_port *up)
    {
    unsigned int status;
    let mut timeout: c_uint = 60000;
// Wait up to 60ms for the character(s) to be sent.
    do {
    status = ar933x_uart_read(up, AR933X_UART_DATA_REG);
    if (--timeout == 0)
    break;
    udelay(1);
    } while ((status & AR933X_UART_DATA_TX_CSR) == 0);
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_console_putchar(port: *mut uart_port, ch: c_uchar) {
    static void ar933x_uart_console_putchar(struct uart_port *port, unsigned char ch)
    {
    struct ar933x_uart_port *up =
    container_of(port, struct ar933x_uart_port, port);
    ar933x_uart_wait_xmitr(up);
    ar933x_uart_putc(up, ch);
    }
    static void ar933x_uart_console_write(struct console *co, const char *s,
    unsigned int count)
    {
    struct ar933x_uart_port *up = ar933x_console_ports[co.index];
    unsigned long flags;
    unsigned int int_en;
    let mut locked: c_int = 1;
    if (oops_in_progress)
    locked = uart_port_trylock_irqsave(&up.port, &flags);
    else
    uart_port_lock_irqsave(&up.port, &flags);
//
// First save the IER then disable the interrupts
//
    int_en = ar933x_uart_read(up, AR933X_UART_INT_EN_REG);
    ar933x_uart_write(up, AR933X_UART_INT_EN_REG, 0);
    uart_console_write(&up.port, s, count, ar933x_uart_console_putchar);
//
// Finally, wait for transmitter to become empty
// and restore the IER
//
    ar933x_uart_wait_xmitr(up);
    ar933x_uart_write(up, AR933X_UART_INT_EN_REG, int_en);
    ar933x_uart_write(up, AR933X_UART_INT_REG, AR933X_UART_INT_ALLINTS);
    if (locked)
    uart_port_unlock_irqrestore(&up.port, flags);
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_console_setup(co: *mut console, options: *mut c_char) -> c_int {
    static int ar933x_uart_console_setup(struct console *co, char *options)
    {
    struct ar933x_uart_port *up;
    let mut baud: c_int = 115200;
    let mut bits: c_int = 8;
    let mut parity: c_int = 'n';
    let mut flow: c_int = 'n';
    if (co.index < 0 || co.index >= CONFIG_SERIAL_AR933X_NR_UARTS)
    return -EINVAL;
    up = ar933x_console_ports[co.index];
    if (!up)
    return -ENODEV;
    if (options)
    uart_parse_options(options, &baud, &parity, &bits, &flow);
    return uart_set_options(&up.port, co, baud, parity, bits, flow);
    }
    static struct console ar933x_uart_console = {
    .name		= "ttyATH",
    .write		= ar933x_uart_console_write,
    .device		= uart_console_device,
    .setup		= ar933x_uart_console_setup,
    .flags		= CON_PRINTBUFFER,
    .index		= -1,
    .data		= &ar933x_uart_driver,
    };

    static struct uart_driver ar933x_uart_driver = {
    .owner		= THIS_MODULE,
    .driver_name	= DRIVER_NAME,
    .dev_name	= "ttyATH",
    .nr		= CONFIG_SERIAL_AR933X_NR_UARTS,
    .cons		= core::ptr::null_mut(), /* filled in runtime */
    };
    static const struct serial_rs485 ar933x_rs485_supported = {
    .flags = SER_RS485_ENABLED | SER_RS485_RTS_ON_SEND | SER_RS485_RTS_AFTER_SEND,
    };
#[no_mangle]
unsafe extern "C" fn ar933x_uart_probe(pdev: *mut platform_device) -> c_int {
    static int ar933x_uart_probe(struct platform_device *pdev)
    {
    struct ar933x_uart_port *up;
    struct uart_port *port;
    struct resource *mem_res;
    struct device_node *np;
    unsigned int baud;
    int id;
    int ret;
    int irq;
    np = pdev.dev.of_node;
    if (IS_ENABLED(CONFIG_OF) && np) {
    id = of_alias_get_id(np, "serial");
    if (id < 0) {
    dev_err(&pdev.dev, "unable to get alias id, err=%d\n",
    id);
    return id;
    }
    } else {
    id = pdev.id;
    if (id == -1)
    id = 0;
    }
    if (id >= CONFIG_SERIAL_AR933X_NR_UARTS)
    return -EINVAL;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    up = devm_kzalloc(&pdev.dev, sizeof(struct ar933x_uart_port),
    GFP_KERNEL);
    if (!up)
    return -ENOMEM;
    up.clk = devm_clk_get(&pdev.dev, "uart");
    if (IS_ERR(up.clk)) {
    dev_err(&pdev.dev, "unable to get UART clock\n");
    return PTR_ERR(up.clk);
    }
    port = &up.port;
    port.membase = devm_platform_get_and_ioremap_resource(pdev, 0, &mem_res);
    if (IS_ERR(port.membase))
    return PTR_ERR(port.membase);
    ret = clk_prepare_enable(up.clk);
    if (ret)
    return ret;
    port.uartclk = clk_get_rate(up.clk);
    if (!port.uartclk) {
    ret = -EINVAL;
    goto err_disable_clk;
    }
    port.mapbase = mem_res.start;
    port.line = id;
    port.irq = irq;
    port.dev = &pdev.dev;
    port.type = PORT_AR933X;
    port.iotype = UPIO_MEM32;
    port.regshift = 2;
    port.fifosize = AR933X_UART_FIFO_SIZE;
    port.ops = &ar933x_uart_ops;
    port.rs485_config = ar933x_config_rs485;
    port.rs485_supported = ar933x_rs485_supported;
    baud = ar933x_uart_get_baud(port.uartclk, AR933X_UART_MAX_SCALE, 1);
    up.min_baud = max_t(unsigned int, baud, AR933X_UART_MIN_BAUD);
    baud = ar933x_uart_get_baud(port.uartclk, 0, AR933X_UART_MAX_STEP);
    up.max_baud = min_t(unsigned int, baud, AR933X_UART_MAX_BAUD);
    ret = uart_get_rs485_mode(port);
    if (ret)
    goto err_disable_clk;
    up.gpios = mctrl_gpio_init(port, 0);
    if (IS_ERR(up.gpios) && PTR_ERR(up.gpios) != -ENOSYS) {
    ret = PTR_ERR(up.gpios);
    goto err_disable_clk;
    }
    up.rts_gpiod = mctrl_gpio_to_gpiod(up.gpios, UART_GPIO_RTS);
    if (!up.rts_gpiod) {
    port.rs485_supported.flags &= ~SER_RS485_ENABLED;
    if (port.rs485.flags & SER_RS485_ENABLED) {
    dev_err(&pdev.dev, "lacking rts-gpio, disabling RS485\n");
    port.rs485.flags &= ~SER_RS485_ENABLED;
    }
    }

    ar933x_console_ports[up.port.line] = up;

    ret = uart_add_one_port(&ar933x_uart_driver, &up.port);
    if (ret)
    goto err_disable_clk;
    platform_set_drvdata(pdev, up);
    return 0;
    err_disable_clk:
    clk_disable_unprepare(up.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_remove(pdev: *mut platform_device) {
    static void ar933x_uart_remove(struct platform_device *pdev)
    {
    struct ar933x_uart_port *up;
    up = platform_get_drvdata(pdev);
    if (up) {
    uart_remove_one_port(&ar933x_uart_driver, &up.port);
    clk_disable_unprepare(up.clk);
    }
    }

    static const struct of_device_id ar933x_uart_of_ids[] = {
    { .compatible = "qca,ar9330-uart" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ar933x_uart_of_ids);

    static struct platform_driver ar933x_uart_platform_driver = {
    .probe		= ar933x_uart_probe,
    .remove		= ar933x_uart_remove,
    .driver		= {
    .name		= DRIVER_NAME,
    .of_match_table = of_match_ptr(ar933x_uart_of_ids),
    },
    };
#[no_mangle]
unsafe extern "C" fn ar933x_uart_init() -> int __init {
    static int __init ar933x_uart_init(void)
    {
    int ret;

    ar933x_uart_driver.cons = &ar933x_uart_console;

    ret = uart_register_driver(&ar933x_uart_driver);
    if (ret)
    goto err_out;
    ret = platform_driver_register(&ar933x_uart_platform_driver);
    if (ret)
    goto err_unregister_uart_driver;
    return 0;
    err_unregister_uart_driver:
    uart_unregister_driver(&ar933x_uart_driver);
    err_out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ar933x_uart_exit() -> void __exit {
    static void __exit ar933x_uart_exit(void)
    {
    platform_driver_unregister(&ar933x_uart_platform_driver);
    uart_unregister_driver(&ar933x_uart_driver);
    }
    module_init(ar933x_uart_init);
    module_exit(ar933x_uart_exit);
    MODULE_DESCRIPTION("Atheros AR933X UART driver");
    MODULE_AUTHOR("Gabor Juhos <juhosg@openwrt.org>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:" DRIVER_NAME);

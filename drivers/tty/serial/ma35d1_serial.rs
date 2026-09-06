//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/ma35d1_serial.c
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
// MA35D1 serial driver
// Copyright (C) 2023 Nuvoton Technology Corp.
//

pub const MA35_UART_NR: c_int = 17;
pub const MA35_RBR_REG: c_uint = 0x00;
pub const MA35_THR_REG: c_uint = 0x00;
pub const MA35_IER_REG: c_uint = 0x04;
pub const MA35_FCR_REG: c_uint = 0x08;
pub const MA35_LCR_REG: c_uint = 0x0C;
pub const MA35_MCR_REG: c_uint = 0x10;
pub const MA35_MSR_REG: c_uint = 0x14;
pub const MA35_FSR_REG: c_uint = 0x18;
pub const MA35_ISR_REG: c_uint = 0x1C;
pub const MA35_TOR_REG: c_uint = 0x20;
pub const MA35_BAUD_REG: c_uint = 0x24;
pub const MA35_ALTCTL_REG: c_uint = 0x2C;
pub const MA35_FUN_SEL_REG: c_uint = 0x30;
pub const MA35_WKCTL_REG: c_uint = 0x40;
pub const MA35_WKSTS_REG: c_uint = 0x44;
// MA35_IER_REG - Interrupt Enable Register

// MA35_FCR_REG - FIFO Control Register

// MA35_LCR_REG - Line Control Register

// MA35_MCR_REG - Modem Control Register

// MA35_MSR_REG - Modem Status Register

// MA35_FSR_REG - FIFO Status Register

// MA35_ISR_REG - Interrupt Status Register

pub const MA35_ISR_ALL: c_uint = 0xFFFFFFFF;
// MA35_BAUD_REG - Baud Rate Divider Register

// MA35_ALTCTL_REG - Alternate Control/Status Register

// MA35_FUN_SEL_REG - Function Select Register

// The constrain for MA35D1 UART baud rate divider
pub const MA35_BAUD_DIV_MAX: c_uint = 0xFFFF;
pub const MA35_BAUD_DIV_MIN: c_int = 11;
// UART FIFO depth
pub const MA35_UART_FIFO_DEPTH: c_int = 32;
// UART console clock

// UART register ioremap size
pub const MA35_UART_REG_SIZE: c_uint = 0x100;
// Rx Timeout
pub const MA35_UART_RX_TOUT: c_uint = 0x40;

    MA35_IER_TIME_OUT_EN | MA35_IER_BUFERR_IEN)

    MA35_ISR_THRE_INT | MA35_ISR_BUFEIF)

    static struct uart_driver ma35d1serial_reg;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uart_ma35d1_port {
    pub port: uart_port,
    pub clk: *mut clk,
    pub /: *mut *mut u16 capabilities; / port capabilities,
    pub ier: u8,
    pub lcr: u8,
    pub mcr: u8,
    pub baud_rate: u32,
    pub console_baud_rate: u32,
    pub console_line: u32,
    pub console_int: u32,
}

    static struct uart_ma35d1_port ma35d1serial_ports[MA35_UART_NR];
    static struct uart_ma35d1_port *to_ma35d1_uart_port(struct uart_port *uart)
    {
    return container_of(uart, struct uart_ma35d1_port, port);
    }
#[no_mangle]
unsafe extern "C" fn serial_in(p: *mut uart_ma35d1_port, offset: u32) -> u32 {
    static u32 serial_in(struct uart_ma35d1_port *p, u32 offset)
    {
    return readl_relaxed(p.port.membase + offset);
    }
#[no_mangle]
unsafe extern "C" fn serial_out(p: *mut uart_ma35d1_port, offset: u32, value: u32) {
    static void serial_out(struct uart_ma35d1_port *p, u32 offset, u32 value)
    {
    writel_relaxed(value, p.port.membase + offset);
    }
#[no_mangle]
unsafe extern "C" fn __stop_tx(p: *mut uart_ma35d1_port) {
    static void __stop_tx(struct uart_ma35d1_port *p)
    {
    u32 ier;
    ier = serial_in(p, MA35_IER_REG);
    if (ier & MA35_IER_THRE_IEN)
    serial_out(p, MA35_IER_REG, ier & ~MA35_IER_THRE_IEN);
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_stop_tx(port: *mut uart_port) {
    static void ma35d1serial_stop_tx(struct uart_port *port)
    {
    struct uart_ma35d1_port *up = to_ma35d1_uart_port(port);
    __stop_tx(up);
    }
#[no_mangle]
unsafe extern "C" fn transmit_chars(up: *mut uart_ma35d1_port) {
    static void transmit_chars(struct uart_ma35d1_port *up)
    {
    u32 count;
    u8 ch;
    if (uart_tx_stopped(&up.port)) {
    ma35d1serial_stop_tx(&up.port);
    return;
    }
    count = MA35_UART_FIFO_DEPTH - FIELD_GET(MA35_FSR_TXPTR_MSK,
    serial_in(up, MA35_FSR_REG));
    uart_port_tx_limited(&up.port, ch, count,
    !(serial_in(up, MA35_FSR_REG) & MA35_FSR_TX_FULL),
    serial_out(up, MA35_THR_REG, ch),
    ({}));
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_start_tx(port: *mut uart_port) {
    static void ma35d1serial_start_tx(struct uart_port *port)
    {
    struct uart_ma35d1_port *up = to_ma35d1_uart_port(port);
    u32 ier;
    ier = serial_in(up, MA35_IER_REG);
    serial_out(up, MA35_IER_REG, ier & ~MA35_IER_THRE_IEN);
    transmit_chars(up);
    serial_out(up, MA35_IER_REG, ier | MA35_IER_THRE_IEN);
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_stop_rx(port: *mut uart_port) {
    static void ma35d1serial_stop_rx(struct uart_port *port)
    {
    struct uart_ma35d1_port *up = to_ma35d1_uart_port(port);
    u32 ier;
    ier = serial_in(up, MA35_IER_REG);
    ier &= ~MA35_IER_RDA_IEN;
    serial_out(up, MA35_IER_REG, ier);
    }
#[no_mangle]
unsafe extern "C" fn receive_chars(up: *mut uart_ma35d1_port) {
    static void receive_chars(struct uart_ma35d1_port *up)
    {
    let mut max_count: c_int = 256;
    u8 ch, flag;
    u32 fsr;
    fsr = serial_in(up, MA35_FSR_REG);
    do {
    flag = TTY_NORMAL;
    up.port.icount.rx++;
    if (unlikely(fsr & (MA35_FSR_BIF | MA35_FSR_FEF |
    MA35_FSR_PEF | MA35_FSR_RX_OVER_IF))) {
    if (fsr & MA35_FSR_BIF) {
    up.port.icount.brk++;
    if (uart_handle_break(&up.port))
    continue;
    }
    if (fsr & MA35_FSR_FEF)
    up.port.icount.frame++;
    if (fsr & MA35_FSR_PEF)
    up.port.icount.parity++;
    if (fsr & MA35_FSR_RX_OVER_IF)
    up.port.icount.overrun++;
    serial_out(up, MA35_FSR_REG,
    fsr & (MA35_FSR_BIF | MA35_FSR_FEF |
    MA35_FSR_PEF | MA35_FSR_RX_OVER_IF));
    if (fsr & MA35_FSR_BIF)
    flag = TTY_BREAK;
#[no_mangle]
pub unsafe extern "C" fn if(MA35_FSR_PEF: fsr &) -> else {
    else if (fsr & MA35_FSR_PEF)
    flag = TTY_PARITY;
#[no_mangle]
pub unsafe extern "C" fn if(MA35_FSR_FEF: fsr &) -> else {
    else if (fsr & MA35_FSR_FEF)
    flag = TTY_FRAME;
    }
    ch = serial_in(up, MA35_RBR_REG);
    if (uart_handle_sysrq_char(&up.port, ch))
    continue;
    uart_port_lock(&up.port);
    uart_insert_char(&up.port, fsr, MA35_FSR_RX_OVER_IF, ch, flag);
    uart_port_unlock(&up.port);
    fsr = serial_in(up, MA35_FSR_REG);
    } while (!(fsr & MA35_FSR_RX_EMPTY) && (max_count-- > 0));
    uart_port_lock(&up.port);
    tty_flip_buffer_push(&up.port.state.port);
    uart_port_unlock(&up.port);
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ma35d1serial_interrupt(int irq, void *dev_id)
    {
    struct uart_port *port = dev_id;
    struct uart_ma35d1_port *up = to_ma35d1_uart_port(port);
    u32 isr, fsr;
    isr = serial_in(up, MA35_ISR_REG);
    fsr = serial_in(up, MA35_FSR_REG);
    if (!(isr & MA35_ISR_IF_CHECK))
    return IRQ_NONE;
    if (isr & (MA35_ISR_RDA_IF | MA35_ISR_RXTO_IF))
    receive_chars(up);
    if (isr & MA35_ISR_THRE_INT)
    transmit_chars(up);
    if (fsr & MA35_FSR_TX_OVER_IF)
    serial_out(up, MA35_FSR_REG, MA35_FSR_TX_OVER_IF);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_tx_empty(port: *mut uart_port) -> u32 {
    static u32 ma35d1serial_tx_empty(struct uart_port *port)
    {
    struct uart_ma35d1_port *up = to_ma35d1_uart_port(port);
    u32 fsr;
    fsr = serial_in(up, MA35_FSR_REG);
    if ((fsr & MA35_FSR_TX_BOTH_EMPTY) == MA35_FSR_TX_BOTH_EMPTY)
    return TIOCSER_TEMT;
    else
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_get_mctrl(port: *mut uart_port) -> u32 {
    static u32 ma35d1serial_get_mctrl(struct uart_port *port)
    {
    struct uart_ma35d1_port *up = to_ma35d1_uart_port(port);
    u32 status;
    let mut ret: u32 = 0;
    status = serial_in(up, MA35_MSR_REG);
    if (!(status & MA35_MSR_CTSSTS))
    ret |= TIOCM_CTS;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_set_mctrl(port: *mut uart_port, mctrl: u32) {
    static void ma35d1serial_set_mctrl(struct uart_port *port, u32 mctrl)
    {
    struct uart_ma35d1_port *up = to_ma35d1_uart_port(port);
    u32 mcr, msr, ier;
    mcr = serial_in(up, MA35_MCR_REG);
    mcr &= ~MA35_MCR_RTS_CTRL;
    if (mctrl & TIOCM_RTS)
    mcr |= MA35_MCR_RTSACTLV;
    else
    mcr &= ~MA35_MCR_RTSACTLV;
    if (up.mcr & UART_MCR_AFE) {
    ier = serial_in(up, MA35_IER_REG);
    ier |= MA35_IER_AUTO_RTS | MA35_IER_AUTO_CTS;
    serial_out(up, MA35_IER_REG, ier);
    up.port.flags |= UPF_HARD_FLOW;
    } else {
    ier = serial_in(up, MA35_IER_REG);
    ier &= ~(MA35_IER_AUTO_RTS | MA35_IER_AUTO_CTS);
    serial_out(up, MA35_IER_REG, ier);
    up.port.flags &= ~UPF_HARD_FLOW;
    }
    msr = serial_in(up, MA35_MSR_REG);
    msr |= MA35_MSR_CTSACTLV;
    serial_out(up, MA35_MSR_REG, msr);
    serial_out(up, MA35_MCR_REG, mcr);
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_break_ctl(port: *mut uart_port, break_state: c_int) {
    static void ma35d1serial_break_ctl(struct uart_port *port, int break_state)
    {
    struct uart_ma35d1_port *up = to_ma35d1_uart_port(port);
    unsigned long flags;
    u32 lcr;
    uart_port_lock_irqsave(&up.port, &flags);
    lcr = serial_in(up, MA35_LCR_REG);
    if (break_state != 0)
    lcr |= MA35_LCR_BREAK;
    else
    lcr &= ~MA35_LCR_BREAK;
    serial_out(up, MA35_LCR_REG, lcr);
    uart_port_unlock_irqrestore(&up.port, flags);
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_startup(port: *mut uart_port) -> c_int {
    static int ma35d1serial_startup(struct uart_port *port)
    {
    struct uart_ma35d1_port *up = to_ma35d1_uart_port(port);
    u32 fcr;
    int retval;
// Reset FIFO
    serial_out(up, MA35_FCR_REG, MA35_FCR_TFR | MA35_FCR_RFR);
// Clear pending interrupts
    serial_out(up, MA35_ISR_REG, MA35_ISR_ALL);
    retval = request_irq(port.irq, ma35d1serial_interrupt, 0,
    dev_name(port.dev), port);
    if (retval) {
    dev_err(up.port.dev, "request irq failed.\n");
    return retval;
    }
    fcr = serial_in(up, MA35_FCR_REG);
    fcr |= MA35_FCR_RFITL_4BYTES | MA35_FCR_RTSTL_8BYTES;
    serial_out(up, MA35_FCR_REG, fcr);
    serial_out(up, MA35_LCR_REG, MA35_LCR_WLS_8BITS);
    serial_out(up, MA35_TOR_REG, MA35_UART_RX_TOUT);
    serial_out(up, MA35_IER_REG, MA35_IER_CONFIG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_shutdown(port: *mut uart_port) {
    static void ma35d1serial_shutdown(struct uart_port *port)
    {
    struct uart_ma35d1_port *up = to_ma35d1_uart_port(port);
    serial_out(up, MA35_IER_REG, 0);
    free_irq(port.irq, port);
    }
    static void ma35d1serial_set_termios(struct uart_port *port,
    struct ktermios *termios,
    const struct ktermios *old)
    {
    struct uart_ma35d1_port *up = to_ma35d1_uart_port(port);
    unsigned long flags;
    u32 baud, quot;
    let mut lcr: u32 = 0;
    lcr = UART_LCR_WLEN(tty_get_char_size(termios.c_cflag));
    if (termios.c_cflag & CSTOPB)
    lcr |= MA35_LCR_NSB;
    if (termios.c_cflag & PARENB)
    lcr |= MA35_LCR_PBE;
    if (!(termios.c_cflag & PARODD))
    lcr |= MA35_LCR_EPE;
    if (termios.c_cflag & CMSPAR)
    lcr |= MA35_LCR_SPE;
    baud = uart_get_baud_rate(port, termios, old,
    port.uartclk / MA35_BAUD_DIV_MAX,
    port.uartclk / MA35_BAUD_DIV_MIN);
// MA35D1 UART baud rate equation: baudrate = UART_CLK / (quot + 2)
    quot = (port.uartclk / baud) - 2;
//
// Ok, we're now changing the port state.  Do it with
// interrupts disabled.
//
    uart_port_lock_irqsave(&up.port, &flags);
    up.port.read_status_mask = MA35_FSR_RX_OVER_IF;
    if (termios.c_iflag & INPCK)
    up.port.read_status_mask |= MA35_FSR_FEF | MA35_FSR_PEF;
    if (termios.c_iflag & (BRKINT | PARMRK))
    up.port.read_status_mask |= MA35_FSR_BIF;
// Characteres to ignore
    up.port.ignore_status_mask = 0;
    if (termios.c_iflag & IGNPAR)
    up.port.ignore_status_mask |= MA35_FSR_FEF | MA35_FSR_PEF;
    if (termios.c_iflag & IGNBRK) {
    up.port.ignore_status_mask |= MA35_FSR_BIF;
//
// If we're ignoring parity and break indicators,
// ignore overruns too (for real raw support).
//
    if (termios.c_iflag & IGNPAR)
    up.port.ignore_status_mask |= MA35_FSR_RX_OVER_IF;
    }
    if (termios.c_cflag & CRTSCTS)
    up.mcr |= UART_MCR_AFE;
    else
    up.mcr &= ~UART_MCR_AFE;
    uart_update_timeout(port, termios.c_cflag, baud);
    ma35d1serial_set_mctrl(&up.port, up.port.mctrl);
    serial_out(up, MA35_BAUD_REG, MA35_BAUD_MODE2 | FIELD_PREP(MA35_BAUD_MASK, quot));
    serial_out(up, MA35_LCR_REG, lcr);
    uart_port_unlock_irqrestore(&up.port, flags);
    }
    static const char *ma35d1serial_type(struct uart_port *port)
    {
    return "ma35d1-uart";
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_config_port(port: *mut uart_port, flags: c_int) {
    static void ma35d1serial_config_port(struct uart_port *port, int flags)
    {
//
// Driver core for serial ports forces a non-zero value for port type.
// Write an arbitrary value here to accommodate the serial core driver,
// as ID part of UAPI is redundant.
//
    port.type = 1;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_verify_port(port: *mut uart_port, ser: *mut serial_struct) -> c_int {
    static int ma35d1serial_verify_port(struct uart_port *port, struct serial_struct *ser)
    {
    if (port.type != PORT_UNKNOWN && ser.type != 1)
    return -EINVAL;
    return 0;
    }
    static const struct uart_ops ma35d1serial_ops = {
    .tx_empty     = ma35d1serial_tx_empty,
    .set_mctrl    = ma35d1serial_set_mctrl,
    .get_mctrl    = ma35d1serial_get_mctrl,
    .stop_tx      = ma35d1serial_stop_tx,
    .start_tx     = ma35d1serial_start_tx,
    .stop_rx      = ma35d1serial_stop_rx,
    .break_ctl    = ma35d1serial_break_ctl,
    .startup      = ma35d1serial_startup,
    .shutdown     = ma35d1serial_shutdown,
    .set_termios  = ma35d1serial_set_termios,
    .type         = ma35d1serial_type,
    .config_port  = ma35d1serial_config_port,
    .verify_port  = ma35d1serial_verify_port,
    };
    static const struct of_device_id ma35d1_serial_of_match[] = {
    { .compatible = "nuvoton,ma35d1-uart" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ma35d1_serial_of_match);

    static struct device_node *ma35d1serial_uart_nodes[MA35_UART_NR];
#[no_mangle]
unsafe extern "C" fn wait_for_xmitr(up: *mut uart_ma35d1_port) {
    static void wait_for_xmitr(struct uart_ma35d1_port *up)
    {
    let mut reg: c_uint = 0;
    read_poll_timeout_atomic(serial_in, reg, reg & MA35_FSR_TX_EMPTY,
    1, 10000, false,
    up, MA35_FSR_REG);
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_console_putchar(port: *mut uart_port, ch: c_uchar) {
    static void ma35d1serial_console_putchar(struct uart_port *port, unsigned char ch)
    {
    struct uart_ma35d1_port *up = to_ma35d1_uart_port(port);
    wait_for_xmitr(up);
    serial_out(up, MA35_THR_REG, ch);
    }
//
// Print a string to the serial port trying not to disturb
// any possible real use of the port...
//
// The console_lock must be held when we get here.
//
#[no_mangle]
unsafe extern "C" fn ma35d1serial_console_write(co: *mut console, s: *const c_char, count: u32) {
    static void ma35d1serial_console_write(struct console *co, const char *s, u32 count)
    {
    struct uart_ma35d1_port *up;
    unsigned long flags;
    let mut locked: c_int = 1;
    u32 ier;
    if ((co.index < 0) || (co.index >= MA35_UART_NR)) {
    pr_warn("Failed to write on console port %x, out of range\n",
    co.index);
    return;
    }
    up = &ma35d1serial_ports[co.index];
    if (up.port.sysrq)
    locked = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: oops_in_progress) -> else {
    else if (oops_in_progress)
    locked = uart_port_trylock_irqsave(&up.port, &flags);
    else
    uart_port_lock_irqsave(&up.port, &flags);
//
// First save the IER then disable the interrupts
//
    ier = serial_in(up, MA35_IER_REG);
    serial_out(up, MA35_IER_REG, 0);
    uart_console_write(&up.port, s, count, ma35d1serial_console_putchar);
    wait_for_xmitr(up);
    serial_out(up, MA35_IER_REG, ier);
    if (locked)
    uart_port_unlock_irqrestore(&up.port, flags);
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_console_setup(co: *mut console, options: *mut c_char) -> int __init {
    static int __init ma35d1serial_console_setup(struct console *co, char *options)
    {
    struct device_node *np;
    struct uart_ma35d1_port *p;
    u32 val32[4];
    struct uart_port *port;
    let mut baud: c_int = 115200;
    let mut bits: c_int = 8;
    let mut parity: c_int = 'n';
    let mut flow: c_int = 'n';
    if ((co.index < 0) || (co.index >= MA35_UART_NR)) {
    pr_debug("Console Port%x out of range\n", co.index);
    return -EINVAL;
    }
    np = ma35d1serial_uart_nodes[co.index];
    p = &ma35d1serial_ports[co.index];
    if (!np || !p)
    return -ENODEV;
    if (of_property_read_u32_array(np, "reg", val32, ARRAY_SIZE(val32)) != 0) {
    of_node_put(np);
    ma35d1serial_uart_nodes[co.index] = core::ptr::null_mut();
    return -EINVAL;
    }
    of_node_put(np);
    ma35d1serial_uart_nodes[co.index] = core::ptr::null_mut();
    p.port.iobase = val32[1];
    p.port.membase = ioremap(p.port.iobase, MA35_UART_REG_SIZE);
    if (!p.port.membase)
    return -ENOMEM;
    p.port.ops = &ma35d1serial_ops;
    p.port.line = 0;
    p.port.uartclk = MA35_UART_CONSOLE_CLK;
    port = &ma35d1serial_ports[co.index].port;
    if (options)
    uart_parse_options(options, &baud, &parity, &bits, &flow);
    return uart_set_options(port, co, baud, parity, bits, flow);
    }
    static struct console ma35d1serial_console = {
    .name    = "ttyNVT",
    .write   = ma35d1serial_console_write,
    .device  = uart_console_device,
    .setup   = ma35d1serial_console_setup,
    .flags   = CON_PRINTBUFFER | CON_ENABLED,
    .index   = -1,
    .data    = &ma35d1serial_reg,
    };
#[no_mangle]
unsafe extern "C" fn ma35d1serial_console_init_port() {
    static void ma35d1serial_console_init_port(void)
    {
    let mut i: u32 = 0;
    struct device_node *np;
    for_each_matching_node(np, ma35d1_serial_of_match) {
    if (ma35d1serial_uart_nodes[i] == core::ptr::null_mut()) {
    of_node_get(np);
    ma35d1serial_uart_nodes[i] = np;
    i++;
    if (i == MA35_UART_NR) {
    of_node_put(np);
    break;
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_console_init() -> int __init {
    static int __init ma35d1serial_console_init(void)
    {
    ma35d1serial_console_init_port();
    register_console(&ma35d1serial_console);
    return 0;
    }
    console_initcall(ma35d1serial_console_init);

    static struct uart_driver ma35d1serial_reg = {
    .owner        = THIS_MODULE,
    .driver_name  = "serial",
    .dev_name     = "ttyNVT",
    .major        = TTY_MAJOR,
    .minor        = 64,
    .cons         = MA35D1SERIAL_CONSOLE,
    .nr           = MA35_UART_NR,
    };
//
// Register a set of serial devices attached to a platform device.
// The list is terminated with a zero flags entry, which means we expect
// all entries to have at least UPF_BOOT_AUTOCONF set.
//
#[no_mangle]
unsafe extern "C" fn ma35d1serial_probe(pdev: *mut platform_device) -> c_int {
    static int ma35d1serial_probe(struct platform_device *pdev)
    {
    struct resource *res_mem;
    struct uart_ma35d1_port *up;
    let mut ret: c_int = 0;
    if (!pdev.dev.of_node)
    return -ENODEV;
    ret = of_alias_get_id(pdev.dev.of_node, "serial");
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to get alias/pdev id, errno %d\n", ret);
    return ret;
    }
    up = &ma35d1serial_ports[ret];
    up.port.line = ret;
    res_mem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res_mem)
    return -ENODEV;
    up.port.iobase = res_mem.start;
    up.port.membase = ioremap(up.port.iobase, MA35_UART_REG_SIZE);
    if (!up.port.membase)
    return -ENOMEM;
    up.port.ops = &ma35d1serial_ops;
    spin_lock_init(&up.port.lock);
    up.clk = of_clk_get(pdev.dev.of_node, 0);
    if (IS_ERR(up.clk)) {
    ret = PTR_ERR(up.clk);
    dev_err(&pdev.dev, "failed to get core clk: %d\n", ret);
    goto err_iounmap;
    }
    ret = clk_prepare_enable(up.clk);
    if (ret)
    goto err_iounmap;
    if (up.port.line != 0)
    up.port.uartclk = clk_get_rate(up.clk);
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    goto err_clk_disable;
    up.port.irq = ret;
    up.port.dev = &pdev.dev;
    up.port.flags = UPF_BOOT_AUTOCONF;
    platform_set_drvdata(pdev, up);
    ret = uart_add_one_port(&ma35d1serial_reg, &up.port);
    if (ret < 0)
    goto err_free_irq;
    return 0;
    err_free_irq:
    free_irq(up.port.irq, &up.port);
    err_clk_disable:
    clk_disable_unprepare(up.clk);
    err_iounmap:
    iounmap(up.port.membase);
    return ret;
    }
//
// Remove serial ports registered against a platform device.
//
#[no_mangle]
unsafe extern "C" fn ma35d1serial_remove(dev: *mut platform_device) {
    static void ma35d1serial_remove(struct platform_device *dev)
    {
    struct uart_port *port = platform_get_drvdata(dev);
    struct uart_ma35d1_port *up = to_ma35d1_uart_port(port);
    uart_remove_one_port(&ma35d1serial_reg, port);
    clk_disable_unprepare(up.clk);
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_suspend(dev: *mut platform_device, state: pm_message_t) -> c_int {
    static int ma35d1serial_suspend(struct platform_device *dev, pm_message_t state)
    {
    struct uart_port *port = platform_get_drvdata(dev);
    struct uart_ma35d1_port *up = to_ma35d1_uart_port(port);
    uart_suspend_port(&ma35d1serial_reg, &up.port);
    if (up.port.line == 0) {
    up.console_baud_rate = serial_in(up, MA35_BAUD_REG);
    up.console_line = serial_in(up, MA35_LCR_REG);
    up.console_int = serial_in(up, MA35_IER_REG);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_resume(dev: *mut platform_device) -> c_int {
    static int ma35d1serial_resume(struct platform_device *dev)
    {
    struct uart_port *port = platform_get_drvdata(dev);
    struct uart_ma35d1_port *up = to_ma35d1_uart_port(port);
    if (up.port.line == 0) {
    serial_out(up, MA35_BAUD_REG, up.console_baud_rate);
    serial_out(up, MA35_LCR_REG, up.console_line);
    serial_out(up, MA35_IER_REG, up.console_int);
    }
    uart_resume_port(&ma35d1serial_reg, &up.port);
    return 0;
    }
    static struct platform_driver ma35d1serial_driver = {
    .probe      = ma35d1serial_probe,
    .remove     = ma35d1serial_remove,
    .suspend    = ma35d1serial_suspend,
    .resume     = ma35d1serial_resume,
    .driver     = {
    .name   = "ma35d1-uart",
    .of_match_table = ma35d1_serial_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn ma35d1serial_init() -> int __init {
    static int __init ma35d1serial_init(void)
    {
    int ret;
    ret = uart_register_driver(&ma35d1serial_reg);
    if (ret)
    return ret;
    ret = platform_driver_register(&ma35d1serial_driver);
    if (ret)
    uart_unregister_driver(&ma35d1serial_reg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1serial_exit() -> void __exit {
    static void __exit ma35d1serial_exit(void)
    {
    platform_driver_unregister(&ma35d1serial_driver);
    uart_unregister_driver(&ma35d1serial_reg);
    }
    module_init(ma35d1serial_init);
    module_exit(ma35d1serial_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("MA35D1 serial driver");

//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/21285.c
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
// Driver for the serial port on the 21285 StrongArm-110 core logic chip.
//
// Based on drivers/char/serial.c
//

pub const SERIAL_21285_MAJOR: c_int = 204;
pub const SERIAL_21285_MINOR: c_int = 4;
pub const RXSTAT_DUMMY_READ: c_uint = 0x80000000;

    static const char serial21285_name[] = "Footbridge UART";
//
// We only need 2 bits of data, so instead of creating a whole structure for
// this, use bits of the private_data pointer of the uart port structure.
//
pub const tx_enabled_bit: c_int = 0;
pub const rx_enabled_bit: c_int = 1;
#[no_mangle]
unsafe extern "C" fn is_enabled(port: *mut uart_port, bit: c_int) -> bool {
    static bool is_enabled(struct uart_port *port, int bit)
    {
    unsigned long *private_data = (unsigned long *)&port.private_data;
    if (test_bit(bit, private_data))
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn enable(port: *mut uart_port, bit: c_int) {
    static void enable(struct uart_port *port, int bit)
    {
    unsigned long *private_data = (unsigned long *)&port.private_data;
    set_bit(bit, private_data);
    }
#[no_mangle]
unsafe extern "C" fn disable(port: *mut uart_port, bit: c_int) {
    static void disable(struct uart_port *port, int bit)
    {
    unsigned long *private_data = (unsigned long *)&port.private_data;
    clear_bit(bit, private_data);
    }

//
// The documented expression for selecting the divisor is:
// BAUD_BASE / baud - 1
// However, typically BAUD_BASE is not divisible by baud, so
// we want to select the divisor that gives us the minimum
// error.  Therefore, we want:
// int(BAUD_BASE / baud - 0.5) ->
// int(BAUD_BASE / baud - (baud >> 1) / baud) ->
// int((BAUD_BASE - (baud >> 1)) / baud)
//
#[no_mangle]
unsafe extern "C" fn serial21285_stop_tx(port: *mut uart_port) {
    static void serial21285_stop_tx(struct uart_port *port)
    {
    if (is_tx_enabled(port)) {
    disable_irq_nosync(IRQ_CONTX);
    tx_disable(port);
    }
    }
#[no_mangle]
unsafe extern "C" fn serial21285_start_tx(port: *mut uart_port) {
    static void serial21285_start_tx(struct uart_port *port)
    {
    if (!is_tx_enabled(port)) {
    enable_irq(IRQ_CONTX);
    tx_enable(port);
    }
    }
#[no_mangle]
unsafe extern "C" fn serial21285_stop_rx(port: *mut uart_port) {
    static void serial21285_stop_rx(struct uart_port *port)
    {
    if (is_rx_enabled(port)) {
    disable_irq_nosync(IRQ_CONRX);
    rx_disable(port);
    }
    }
#[no_mangle]
unsafe extern "C" fn serial21285_rx_chars(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t serial21285_rx_chars(int irq, void *dev_id)
    {
    struct uart_port *port = dev_id;
    unsigned int status, rxs, max_count = 256;
    u8 ch, flag;
    status = *CSR_UARTFLG;
    while (!(status & 0x10) && max_count--) {
    ch = *CSR_UARTDR;
    flag = TTY_NORMAL;
    port.icount.rx++;
    rxs = *CSR_RXSTAT | RXSTAT_DUMMY_READ;
    if (unlikely(rxs & RXSTAT_ANYERR)) {
    if (rxs & RXSTAT_PARITY)
    port.icount.parity++;
#[no_mangle]
pub unsafe extern "C" fn if(RXSTAT_FRAME: rxs &) -> else {
    else if (rxs & RXSTAT_FRAME)
    port.icount.frame++;
    if (rxs & RXSTAT_OVERRUN)
    port.icount.overrun++;
    rxs &= port.read_status_mask;
    if (rxs & RXSTAT_PARITY)
    flag = TTY_PARITY;
#[no_mangle]
pub unsafe extern "C" fn if(RXSTAT_FRAME: rxs &) -> else {
    else if (rxs & RXSTAT_FRAME)
    flag = TTY_FRAME;
    }
    uart_insert_char(port, rxs, RXSTAT_OVERRUN, ch, flag);
    status = *CSR_UARTFLG;
    }
    tty_flip_buffer_push(&port.state.port);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn serial21285_tx_chars(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t serial21285_tx_chars(int irq, void *dev_id)
    {
    struct uart_port *port = dev_id;
    u8 ch;
    uart_port_tx_limited(port, ch, 256,
    !(*CSR_UARTFLG & 0x20),
// CSR_UARTDR = ch,
    ({}));
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn serial21285_tx_empty(port: *mut uart_port) -> c_uint {
    static unsigned int serial21285_tx_empty(struct uart_port *port)
    {
    return (*CSR_UARTFLG & 8) ? 0 : TIOCSER_TEMT;
    }
// no modem control lines
#[no_mangle]
unsafe extern "C" fn serial21285_get_mctrl(port: *mut uart_port) -> c_uint {
    static unsigned int serial21285_get_mctrl(struct uart_port *port)
    {
    return TIOCM_CAR | TIOCM_DSR | TIOCM_CTS;
    }
#[no_mangle]
unsafe extern "C" fn serial21285_set_mctrl(port: *mut uart_port, mctrl: c_uint) {
    static void serial21285_set_mctrl(struct uart_port *port, unsigned int mctrl)
    {
    }
#[no_mangle]
unsafe extern "C" fn serial21285_break_ctl(port: *mut uart_port, break_state: c_int) {
    static void serial21285_break_ctl(struct uart_port *port, int break_state)
    {
    unsigned long flags;
    unsigned int h_lcr;
    uart_port_lock_irqsave(port, &flags);
    h_lcr = *CSR_H_UBRLCR;
    if (break_state)
    h_lcr |= H_UBRLCR_BREAK;
    else
    h_lcr &= ~H_UBRLCR_BREAK;
// CSR_H_UBRLCR = h_lcr;
    uart_port_unlock_irqrestore(port, flags);
    }
#[no_mangle]
unsafe extern "C" fn serial21285_startup(port: *mut uart_port) -> c_int {
    static int serial21285_startup(struct uart_port *port)
    {
    int ret;
    tx_enable(port);
    rx_enable(port);
    ret = request_irq(IRQ_CONRX, serial21285_rx_chars, 0,
    serial21285_name, port);
    if (ret == 0) {
    ret = request_irq(IRQ_CONTX, serial21285_tx_chars, 0,
    serial21285_name, port);
    if (ret)
    free_irq(IRQ_CONRX, port);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn serial21285_shutdown(port: *mut uart_port) {
    static void serial21285_shutdown(struct uart_port *port)
    {
    free_irq(IRQ_CONTX, port);
    free_irq(IRQ_CONRX, port);
    }
    static void
    serial21285_set_termios(struct uart_port *port, struct ktermios *termios,
    const struct ktermios *old)
    {
    unsigned long flags;
    unsigned int baud, quot, h_lcr, b;
//
// We don't support modem control lines.
//
    termios.c_cflag &= ~(HUPCL | CRTSCTS | CMSPAR);
    termios.c_cflag |= CLOCAL;
//
// We don't support BREAK character recognition.
//
    termios.c_iflag &= ~(IGNBRK | BRKINT);
//
// Ask the core to calculate the divisor for us.
//
    baud = uart_get_baud_rate(port, termios, old, 0, port.uartclk/16);
    quot = uart_get_divisor(port, baud);
    b = port.uartclk / (16 * quot);
    tty_termios_encode_baud_rate(termios, b, b);
    switch (termios.c_cflag & CSIZE) {
    case CS5:
    h_lcr = 0x00;
    break;
    case CS6:
    h_lcr = 0x20;
    break;
    case CS7:
    h_lcr = 0x40;
    break;
    default: /* CS8 */
    h_lcr = 0x60;
    break;
    }
    if (termios.c_cflag & CSTOPB)
    h_lcr |= H_UBRLCR_STOPB;
    if (termios.c_cflag & PARENB) {
    h_lcr |= H_UBRLCR_PARENB;
    if (!(termios.c_cflag & PARODD))
    h_lcr |= H_UBRLCR_PAREVN;
    }
    if (port.fifosize)
    h_lcr |= H_UBRLCR_FIFO;
    uart_port_lock_irqsave(port, &flags);
//
// Update the per-port timeout.
//
    uart_update_timeout(port, termios.c_cflag, baud);
//
// Which character status flags are we interested in?
//
    port.read_status_mask = RXSTAT_OVERRUN;
    if (termios.c_iflag & INPCK)
    port.read_status_mask |= RXSTAT_FRAME | RXSTAT_PARITY;
//
// Which character status flags should we ignore?
//
    port.ignore_status_mask = 0;
    if (termios.c_iflag & IGNPAR)
    port.ignore_status_mask |= RXSTAT_FRAME | RXSTAT_PARITY;
    if (termios.c_iflag & IGNBRK && termios.c_iflag & IGNPAR)
    port.ignore_status_mask |= RXSTAT_OVERRUN;
//
// Ignore all characters if CREAD is not set.
//
    if ((termios.c_cflag & CREAD) == 0)
    port.ignore_status_mask |= RXSTAT_DUMMY_READ;
    quot -= 1;
// CSR_UARTCON = 0;
// CSR_L_UBRLCR = quot & 0xff;
// CSR_M_UBRLCR = (quot >> 8) & 0x0f;
// CSR_H_UBRLCR = h_lcr;
// CSR_UARTCON = 1;
    uart_port_unlock_irqrestore(port, flags);
    }
    static const char *serial21285_type(struct uart_port *port)
    {
    return port.type == PORT_21285 ? "DC21285" : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn serial21285_release_port(port: *mut uart_port) {
    static void serial21285_release_port(struct uart_port *port)
    {
    release_mem_region(port.mapbase, 32);
    }
#[no_mangle]
unsafe extern "C" fn serial21285_request_port(port: *mut uart_port) -> c_int {
    static int serial21285_request_port(struct uart_port *port)
    {
#[no_mangle]
pub unsafe extern "C" fn request_mem_region(_arg: port->mapbase, _arg: 32, _arg: serial21285_name) -> return {
    return request_mem_region(port.mapbase, 32, serial21285_name)
    != core::ptr::null_mut() ? 0 : -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn serial21285_config_port(port: *mut uart_port, flags: c_int) {
    static void serial21285_config_port(struct uart_port *port, int flags)
    {
    if (flags & UART_CONFIG_TYPE && serial21285_request_port(port) == 0)
    port.type = PORT_21285;
    }
//
// verify the new serial_struct (for TIOCSSERIAL).
//
#[no_mangle]
unsafe extern "C" fn serial21285_verify_port(port: *mut uart_port, ser: *mut serial_struct) -> c_int {
    static int serial21285_verify_port(struct uart_port *port, struct serial_struct *ser)
    {
    let mut ret: c_int = 0;
    if (ser.type != PORT_UNKNOWN && ser.type != PORT_21285)
    ret = -EINVAL;
    if (ser.irq <= 0)
    ret = -EINVAL;
    if (ser.baud_base != port.uartclk / 16)
    ret = -EINVAL;
    return ret;
    }
    static const struct uart_ops serial21285_ops = {
    .tx_empty	= serial21285_tx_empty,
    .get_mctrl	= serial21285_get_mctrl,
    .set_mctrl	= serial21285_set_mctrl,
    .stop_tx	= serial21285_stop_tx,
    .start_tx	= serial21285_start_tx,
    .stop_rx	= serial21285_stop_rx,
    .break_ctl	= serial21285_break_ctl,
    .startup	= serial21285_startup,
    .shutdown	= serial21285_shutdown,
    .set_termios	= serial21285_set_termios,
    .type		= serial21285_type,
    .release_port	= serial21285_release_port,
    .request_port	= serial21285_request_port,
    .config_port	= serial21285_config_port,
    .verify_port	= serial21285_verify_port,
    };
    static struct uart_port serial21285_port = {
    .mapbase	= 0x42000160,
    .iotype		= UPIO_MEM,
    .irq		= 0,
    .fifosize	= 16,
    .ops		= &serial21285_ops,
    .flags		= UPF_BOOT_AUTOCONF,
    };
#[no_mangle]
unsafe extern "C" fn serial21285_setup_ports() {
    static void serial21285_setup_ports(void)
    {
    serial21285_port.uartclk = mem_fclk_21285 / 4;
    }

#[no_mangle]
unsafe extern "C" fn serial21285_console_putchar(port: *mut uart_port, ch: c_uchar) {
    static void serial21285_console_putchar(struct uart_port *port, unsigned char ch)
    {
    while (*CSR_UARTFLG & 0x20)
    barrier();
// CSR_UARTDR = ch;
    }
    static void
    serial21285_console_write(struct console *co, const char *s,
    unsigned int count)
    {
    uart_console_write(&serial21285_port, s, count, serial21285_console_putchar);
    }
    static void __init
    serial21285_get_options(struct uart_port *port, int *baud,
    int *parity, int *bits)
    {
    if (*CSR_UARTCON == 1) {
    unsigned int tmp;
    tmp = *CSR_H_UBRLCR;
    switch (tmp & 0x60) {
    case 0x00:
// bits = 5;
    break;
    case 0x20:
// bits = 6;
    break;
    case 0x40:
// bits = 7;
    break;
    default:
    case 0x60:
// bits = 8;
    break;
    }
    if (tmp & H_UBRLCR_PARENB) {
// parity = 'o';
    if (tmp & H_UBRLCR_PAREVN)
// parity = 'e';
    }
    tmp = *CSR_L_UBRLCR | (*CSR_M_UBRLCR << 8);
// baud = port->uartclk / (16 * (tmp + 1));
    }
    }
#[no_mangle]
unsafe extern "C" fn serial21285_console_setup(co: *mut console, options: *mut c_char) -> int __init {
    static int __init serial21285_console_setup(struct console *co, char *options)
    {
    struct uart_port *port = &serial21285_port;
    let mut baud: c_int = 9600;
    let mut bits: c_int = 8;
    let mut parity: c_int = 'n';
    let mut flow: c_int = 'n';
//
// Check whether an invalid uart number has been specified, and
// if so, search for the first available port that does have
// console support.
//
    if (options)
    uart_parse_options(options, &baud, &parity, &bits, &flow);
    else
    serial21285_get_options(port, &baud, &parity, &bits);
    return uart_set_options(port, co, baud, parity, bits, flow);
    }
    static struct uart_driver serial21285_reg;
    static struct console serial21285_console =
    {
    .name		= SERIAL_21285_NAME,
    .write		= serial21285_console_write,
    .device		= uart_console_device,
    .setup		= serial21285_console_setup,
    .flags		= CON_PRINTBUFFER,
    .index		= -1,
    .data		= &serial21285_reg,
    };
#[no_mangle]
unsafe extern "C" fn rs285_console_init() -> int __init {
    static int __init rs285_console_init(void)
    {
    serial21285_setup_ports();
    register_console(&serial21285_console);
    return 0;
    }
    console_initcall(rs285_console_init);

    static struct uart_driver serial21285_reg = {
    .owner			= THIS_MODULE,
    .driver_name		= "ttyFB",
    .dev_name		= "ttyFB",
    .major			= SERIAL_21285_MAJOR,
    .minor			= SERIAL_21285_MINOR,
    .nr			= 1,
    .cons			= SERIAL_21285_CONSOLE,
    };
#[no_mangle]
unsafe extern "C" fn serial21285_init() -> int __init {
    static int __init serial21285_init(void)
    {
    int ret;
    printk(KERN_INFO "Serial: 21285 driver\n");
    serial21285_setup_ports();
    ret = uart_register_driver(&serial21285_reg);
    if (ret == 0)
    uart_add_one_port(&serial21285_reg, &serial21285_port);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn serial21285_exit() -> void __exit {
    static void __exit serial21285_exit(void)
    {
    uart_remove_one_port(&serial21285_reg, &serial21285_port);
    uart_unregister_driver(&serial21285_reg);
    }
    module_init(serial21285_init);
    module_exit(serial21285_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Intel Footbridge (21285) serial driver");
    MODULE_ALIAS_CHARDEV(SERIAL_21285_MAJOR, SERIAL_21285_MINOR);

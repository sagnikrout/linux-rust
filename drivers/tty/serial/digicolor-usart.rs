//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/digicolor-usart.c
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
// Driver for Conexant Digicolor serial ports (USART)
//
// Author: Baruch Siach <baruch@tkos.co.il>
//
// Copyright (C) 2014 Paradox Innovation Ltd.
//

pub const UA_ENABLE: c_uint = 0x00;

pub const UA_CONTROL: c_uint = 0x01;

pub const UA_STATUS: c_uint = 0x02;

pub const UA_CONFIG: c_uint = 0x03;

pub const UA_EMI_REC: c_uint = 0x04;
pub const UA_HBAUD_LO: c_uint = 0x08;
pub const UA_HBAUD_HI: c_uint = 0x09;
pub const UA_STATUS_FIFO: c_uint = 0x0a;

pub const UA_CONFIG_FIFO: c_uint = 0x0b;
pub const UA_CONFIG_FIFO_RX_THRESH: c_int = 7;

pub const UA_INTFLAG_CLEAR: c_uint = 0x1c;
pub const UA_INTFLAG_SET: c_uint = 0x1d;
pub const UA_INT_ENABLE: c_uint = 0x1e;
pub const UA_INT_STATUS: c_uint = 0x1f;

pub const DIGICOLOR_USART_NR: c_int = 3;
//
// We use the 16 bytes hardware FIFO to buffer Rx traffic. Rx interrupt is
// only produced when the FIFO is filled more than a certain configurable
// threshold. Unfortunately, there is no way to set this threshold below half
// FIFO. This means that we must periodically poll the FIFO status register to
// see whether there are waiting Rx bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct digicolor_port {
    pub port: uart_port,
    pub rx_poll_work: delayed_work,
}

    static struct uart_port *digicolor_ports[DIGICOLOR_USART_NR];
#[no_mangle]
unsafe extern "C" fn digicolor_uart_tx_full(port: *mut uart_port) -> bool {
    static bool digicolor_uart_tx_full(struct uart_port *port)
    {
    return !!(readb_relaxed(port.membase + UA_STATUS_FIFO) &
    UA_STATUS_FIFO_TX_FULL);
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_rx_empty(port: *mut uart_port) -> bool {
    static bool digicolor_uart_rx_empty(struct uart_port *port)
    {
    return !!(readb_relaxed(port.membase + UA_STATUS_FIFO) &
    UA_STATUS_FIFO_RX_EMPTY);
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_stop_tx(port: *mut uart_port) {
    static void digicolor_uart_stop_tx(struct uart_port *port)
    {
    let mut int_enable: u8 = readb_relaxed(port.membase + UA_INT_ENABLE);
    int_enable &= ~UA_INT_TX;
    writeb_relaxed(int_enable, port.membase + UA_INT_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_start_tx(port: *mut uart_port) {
    static void digicolor_uart_start_tx(struct uart_port *port)
    {
    let mut int_enable: u8 = readb_relaxed(port.membase + UA_INT_ENABLE);
    int_enable |= UA_INT_TX;
    writeb_relaxed(int_enable, port.membase + UA_INT_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_stop_rx(port: *mut uart_port) {
    static void digicolor_uart_stop_rx(struct uart_port *port)
    {
    let mut int_enable: u8 = readb_relaxed(port.membase + UA_INT_ENABLE);
    int_enable &= ~UA_INT_RX;
    writeb_relaxed(int_enable, port.membase + UA_INT_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn digicolor_rx_poll(work: *mut work_struct) {
    static void digicolor_rx_poll(struct work_struct *work)
    {
    struct digicolor_port *dp =
    container_of(to_delayed_work(work),
    struct digicolor_port, rx_poll_work);
    if (!digicolor_uart_rx_empty(&dp.port))
// force RX interrupt
    writeb_relaxed(UA_INT_RX, dp.port.membase + UA_INTFLAG_SET);
    schedule_delayed_work(&dp.rx_poll_work, msecs_to_jiffies(100));
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_rx(port: *mut uart_port) {
    static void digicolor_uart_rx(struct uart_port *port)
    {
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
    while (1) {
    u8 status, ch, ch_flag;
    if (digicolor_uart_rx_empty(port))
    break;
    ch = readb_relaxed(port.membase + UA_EMI_REC);
    status = readb_relaxed(port.membase + UA_STATUS);
    port.icount.rx++;
    ch_flag = TTY_NORMAL;
    if (status) {
    if (status & UA_STATUS_PARITY_ERR)
    port.icount.parity++;
#[no_mangle]
pub unsafe extern "C" fn if(UA_STATUS_FRAME_ERR: status &) -> else {
    else if (status & UA_STATUS_FRAME_ERR)
    port.icount.frame++;
#[no_mangle]
pub unsafe extern "C" fn if(UA_STATUS_OVERRUN_ERR: status &) -> else {
    else if (status & UA_STATUS_OVERRUN_ERR)
    port.icount.overrun++;
    status &= port.read_status_mask;
    if (status & UA_STATUS_PARITY_ERR)
    ch_flag = TTY_PARITY;
#[no_mangle]
pub unsafe extern "C" fn if(UA_STATUS_FRAME_ERR: status &) -> else {
    else if (status & UA_STATUS_FRAME_ERR)
    ch_flag = TTY_FRAME;
#[no_mangle]
pub unsafe extern "C" fn if(UA_STATUS_OVERRUN_ERR: status &) -> else {
    else if (status & UA_STATUS_OVERRUN_ERR)
    ch_flag = TTY_OVERRUN;
    }
    if (status & port.ignore_status_mask)
    continue;
    uart_insert_char(port, status, UA_STATUS_OVERRUN_ERR, ch,
    ch_flag);
    }
    uart_port_unlock_irqrestore(port, flags);
    tty_flip_buffer_push(&port.state.port);
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_tx(port: *mut uart_port) {
    static void digicolor_uart_tx(struct uart_port *port)
    {
    struct tty_port *tport = &port.state.port;
    unsigned long flags;
    unsigned char c;
    if (digicolor_uart_tx_full(port))
    return;
    uart_port_lock_irqsave(port, &flags);
    if (port.x_char) {
    writeb_relaxed(port.x_char, port.membase + UA_EMI_REC);
    port.icount.tx++;
    port.x_char = 0;
    goto out;
    }
    if (kfifo_is_empty(&tport.xmit_fifo) || uart_tx_stopped(port)) {
    digicolor_uart_stop_tx(port);
    goto out;
    }
    while (uart_fifo_get(port, &c)) {
    writeb(c, port.membase + UA_EMI_REC);
    if (digicolor_uart_tx_full(port))
    break;
    }
    if (kfifo_len(&tport.xmit_fifo) < WAKEUP_CHARS)
    uart_write_wakeup(port);
    out:
    uart_port_unlock_irqrestore(port, flags);
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_int(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t digicolor_uart_int(int irq, void *dev_id)
    {
    struct uart_port *port = dev_id;
    let mut int_status: u8 = readb_relaxed(port.membase + UA_INT_STATUS);
    writeb_relaxed(UA_INT_RX | UA_INT_TX,
    port.membase + UA_INTFLAG_CLEAR);
    if (int_status & UA_INT_RX)
    digicolor_uart_rx(port);
    if (int_status & UA_INT_TX)
    digicolor_uart_tx(port);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_tx_empty(port: *mut uart_port) -> c_uint {
    static unsigned int digicolor_uart_tx_empty(struct uart_port *port)
    {
    let mut status: u8 = readb_relaxed(port.membase + UA_STATUS);
    return (status & UA_STATUS_TX_READY) ? TIOCSER_TEMT : 0;
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_get_mctrl(port: *mut uart_port) -> c_uint {
    static unsigned int digicolor_uart_get_mctrl(struct uart_port *port)
    {
    return TIOCM_CTS;
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_set_mctrl(port: *mut uart_port, mctrl: c_uint) {
    static void digicolor_uart_set_mctrl(struct uart_port *port, unsigned int mctrl)
    {
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_break_ctl(port: *mut uart_port, state: c_int) {
    static void digicolor_uart_break_ctl(struct uart_port *port, int state)
    {
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_startup(port: *mut uart_port) -> c_int {
    static int digicolor_uart_startup(struct uart_port *port)
    {
    struct digicolor_port *dp =
    container_of(port, struct digicolor_port, port);
    writeb_relaxed(UA_ENABLE_ENABLE, port.membase + UA_ENABLE);
    writeb_relaxed(UA_CONTROL_SOFT_RESET, port.membase + UA_CONTROL);
    writeb_relaxed(0, port.membase + UA_CONTROL);
    writeb_relaxed(UA_CONFIG_FIFO_RX_FIFO_MODE
    | UA_CONFIG_FIFO_TX_FIFO_MODE | UA_CONFIG_FIFO_RX_THRESH,
    port.membase + UA_CONFIG_FIFO);
    writeb_relaxed(UA_STATUS_FIFO_RX_INT_ALMOST,
    port.membase + UA_STATUS_FIFO);
    writeb_relaxed(UA_CONTROL_RX_ENABLE | UA_CONTROL_TX_ENABLE,
    port.membase + UA_CONTROL);
    writeb_relaxed(UA_INT_TX | UA_INT_RX,
    port.membase + UA_INT_ENABLE);
    schedule_delayed_work(&dp.rx_poll_work, msecs_to_jiffies(100));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_shutdown(port: *mut uart_port) {
    static void digicolor_uart_shutdown(struct uart_port *port)
    {
    struct digicolor_port *dp =
    container_of(port, struct digicolor_port, port);
    writeb_relaxed(0, port.membase + UA_ENABLE);
    cancel_delayed_work_sync(&dp.rx_poll_work);
    }
    static void digicolor_uart_set_termios(struct uart_port *port,
    struct ktermios *termios,
    const struct ktermios *old)
    {
    unsigned int baud, divisor;
    let mut config: u8 = 0;
    unsigned long flags;
// Mask termios capabilities we don't support
    termios.c_cflag &= ~CMSPAR;
    termios.c_iflag &= ~(BRKINT | IGNBRK);
// Limit baud rates so that we don't need the fractional divider
    baud = uart_get_baud_rate(port, termios, old,
    port.uartclk / (0x10000*16),
    port.uartclk / 256);
    divisor = uart_get_divisor(port, baud) - 1;
    switch (termios.c_cflag & CSIZE) {
    case CS7:
    break;
    case CS8:
    default:
    config |= UA_CONFIG_CHAR_LEN;
    termios.c_cflag &= ~CSIZE;
    termios.c_cflag |= CS8;
    break;
    }
    if (termios.c_cflag & CSTOPB)
    config |= UA_CONFIG_STOP_BITS;
    if (termios.c_cflag & PARENB) {
    config |= UA_CONFIG_PARITY;
    if (termios.c_cflag & PARODD)
    config |= UA_CONFIG_ODD_PARITY;
    }
// Set read status mask
    port.read_status_mask = UA_STATUS_OVERRUN_ERR;
    if (termios.c_iflag & INPCK)
    port.read_status_mask |= UA_STATUS_PARITY_ERR
    | UA_STATUS_FRAME_ERR;
// Set status ignore mask
    port.ignore_status_mask = 0;
    if (!(termios.c_cflag & CREAD))
    port.ignore_status_mask |= UA_STATUS_OVERRUN_ERR
    | UA_STATUS_PARITY_ERR | UA_STATUS_FRAME_ERR;
    uart_port_lock_irqsave(port, &flags);
    uart_update_timeout(port, termios.c_cflag, baud);
    writeb_relaxed(config, port.membase + UA_CONFIG);
    writeb_relaxed(divisor & 0xff, port.membase + UA_HBAUD_LO);
    writeb_relaxed(divisor >> 8, port.membase + UA_HBAUD_HI);
    uart_port_unlock_irqrestore(port, flags);
    }
    static const char *digicolor_uart_type(struct uart_port *port)
    {
    return (port.type == PORT_DIGICOLOR) ? "DIGICOLOR USART" : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_config_port(port: *mut uart_port, flags: c_int) {
    static void digicolor_uart_config_port(struct uart_port *port, int flags)
    {
    if (flags & UART_CONFIG_TYPE)
    port.type = PORT_DIGICOLOR;
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_release_port(port: *mut uart_port) {
    static void digicolor_uart_release_port(struct uart_port *port)
    {
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_request_port(port: *mut uart_port) -> c_int {
    static int digicolor_uart_request_port(struct uart_port *port)
    {
    return 0;
    }
    static const struct uart_ops digicolor_uart_ops = {
    .tx_empty	= digicolor_uart_tx_empty,
    .set_mctrl	= digicolor_uart_set_mctrl,
    .get_mctrl	= digicolor_uart_get_mctrl,
    .stop_tx	= digicolor_uart_stop_tx,
    .start_tx	= digicolor_uart_start_tx,
    .stop_rx	= digicolor_uart_stop_rx,
    .break_ctl	= digicolor_uart_break_ctl,
    .startup	= digicolor_uart_startup,
    .shutdown	= digicolor_uart_shutdown,
    .set_termios	= digicolor_uart_set_termios,
    .type		= digicolor_uart_type,
    .config_port	= digicolor_uart_config_port,
    .release_port	= digicolor_uart_release_port,
    .request_port	= digicolor_uart_request_port,
    };
#[no_mangle]
unsafe extern "C" fn digicolor_uart_console_putchar(port: *mut uart_port, ch: c_uchar) {
    static void digicolor_uart_console_putchar(struct uart_port *port, unsigned char ch)
    {
    while (digicolor_uart_tx_full(port))
    cpu_relax();
    writeb_relaxed(ch, port.membase + UA_EMI_REC);
    }
    static void digicolor_uart_console_write(struct console *co, const char *c,
    unsigned n)
    {
    struct uart_port *port = digicolor_ports[co.index];
    u8 status;
    unsigned long flags;
    let mut locked: c_int = 1;
    if (oops_in_progress)
    locked = uart_port_trylock_irqsave(port, &flags);
    else
    uart_port_lock_irqsave(port, &flags);
    uart_console_write(port, c, n, digicolor_uart_console_putchar);
    if (locked)
    uart_port_unlock_irqrestore(port, flags);
// Wait for transmitter to become empty
    do {
    status = readb_relaxed(port.membase + UA_STATUS);
    } while ((status & UA_STATUS_TX_READY) == 0);
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_console_setup(co: *mut console, options: *mut c_char) -> c_int {
    static int digicolor_uart_console_setup(struct console *co, char *options)
    {
    let mut baud: c_int = 115200, bits = 8, parity = 'n', flow = 'n';
    struct uart_port *port;
    if (co.index < 0 || co.index >= DIGICOLOR_USART_NR)
    return -EINVAL;
    port = digicolor_ports[co.index];
    if (!port)
    return -ENODEV;
    if (options)
    uart_parse_options(options, &baud, &parity, &bits, &flow);
    return uart_set_options(port, co, baud, parity, bits, flow);
    }
    static struct console digicolor_console = {
    .name	= "ttyS",
    .device	= uart_console_device,
    .write	= digicolor_uart_console_write,
    .setup	= digicolor_uart_console_setup,
    .flags	= CON_PRINTBUFFER,
    .index	= -1,
    };
    static struct uart_driver digicolor_uart = {
    .driver_name	= "digicolor-usart",
    .dev_name	= "ttyS",
    .nr		= DIGICOLOR_USART_NR,
    };
#[no_mangle]
unsafe extern "C" fn digicolor_uart_probe(pdev: *mut platform_device) -> c_int {
    static int digicolor_uart_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    int irq, ret, index;
    struct digicolor_port *dp;
    struct resource *res;
    struct clk *uart_clk;
    if (!np) {
    dev_err(&pdev.dev, "Missing device tree node\n");
    return -ENXIO;
    }
    index = of_alias_get_id(np, "serial");
    if (index < 0 || index >= DIGICOLOR_USART_NR)
    return -EINVAL;
    dp = devm_kzalloc(&pdev.dev, sizeof(*dp), GFP_KERNEL);
    if (!dp)
    return -ENOMEM;
    uart_clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(uart_clk))
    return PTR_ERR(uart_clk);
    dp.port.membase = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(dp.port.membase))
    return PTR_ERR(dp.port.membase);
    dp.port.mapbase = res.start;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    dp.port.irq = irq;
    dp.port.iotype = UPIO_MEM;
    dp.port.uartclk = clk_get_rate(uart_clk);
    dp.port.fifosize = 16;
    dp.port.dev = &pdev.dev;
    dp.port.ops = &digicolor_uart_ops;
    dp.port.line = index;
    dp.port.type = PORT_DIGICOLOR;
    spin_lock_init(&dp.port.lock);
    digicolor_ports[index] = &dp.port;
    platform_set_drvdata(pdev, &dp.port);
    INIT_DELAYED_WORK(&dp.rx_poll_work, digicolor_rx_poll);
    ret = devm_request_irq(&pdev.dev, dp.port.irq, digicolor_uart_int, 0,
    dev_name(&pdev.dev), &dp.port);
    if (ret)
    return ret;
    return uart_add_one_port(&digicolor_uart, &dp.port);
    }
#[no_mangle]
unsafe extern "C" fn digicolor_uart_remove(pdev: *mut platform_device) {
    static void digicolor_uart_remove(struct platform_device *pdev)
    {
    struct uart_port *port = platform_get_drvdata(pdev);
    uart_remove_one_port(&digicolor_uart, port);
    }
    static const struct of_device_id digicolor_uart_dt_ids[] = {
    { .compatible = "cnxt,cx92755-usart", },
    { }
    };
    MODULE_DEVICE_TABLE(of, digicolor_uart_dt_ids);
    static struct platform_driver digicolor_uart_platform = {
    .driver = {
    .name		= "digicolor-usart",
    .of_match_table	= of_match_ptr(digicolor_uart_dt_ids),
    },
    .probe	= digicolor_uart_probe,
    .remove = digicolor_uart_remove,
    };
#[no_mangle]
unsafe extern "C" fn digicolor_uart_init() -> int __init {
    static int __init digicolor_uart_init(void)
    {
    int ret;
    if (IS_ENABLED(CONFIG_SERIAL_CONEXANT_DIGICOLOR_CONSOLE)) {
    digicolor_uart.cons = &digicolor_console;
    digicolor_console.data = &digicolor_uart;
    }
    ret = uart_register_driver(&digicolor_uart);
    if (ret)
    return ret;
    ret = platform_driver_register(&digicolor_uart_platform);
    if (ret)
    uart_unregister_driver(&digicolor_uart);
    return ret;
    }
    module_init(digicolor_uart_init);
#[no_mangle]
unsafe extern "C" fn digicolor_uart_exit() -> void __exit {
    static void __exit digicolor_uart_exit(void)
    {
    platform_driver_unregister(&digicolor_uart_platform);
    uart_unregister_driver(&digicolor_uart);
    }
    module_exit(digicolor_uart_exit);
    MODULE_AUTHOR("Baruch Siach <baruch@tkos.co.il>");
    MODULE_DESCRIPTION("Conexant Digicolor USART serial driver");
    MODULE_LICENSE("GPL");

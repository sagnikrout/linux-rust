//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/lpc32xx_hs.c
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
// High Speed Serial Ports on NXP LPC32xx SoC
//
// Authors: Kevin Wells <kevin.wells@nxp.com>
// Roland Stigge <stigge@antcom.de>
//
// Copyright (C) 2010 NXP Semiconductors
// Copyright (C) 2012 Roland Stigge
//

//
// High Speed UART register offsets
//

pub const LPC32XX_MAIN_OSC_FREQ: c_int = 13000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc32xx_hsuart_port {
    pub port: uart_port,
}

pub const FIFO_READ_LIMIT: c_int = 128;
pub const MAX_PORTS: c_int = 3;

    static struct lpc32xx_hsuart_port lpc32xx_hs_ports[MAX_PORTS];

#[no_mangle]
unsafe extern "C" fn wait_for_xmit_empty(port: *mut uart_port) {
    static void wait_for_xmit_empty(struct uart_port *port)
    {
    let mut timeout: c_uint = 10000;
    do {
    if (LPC32XX_HSU_TX_LEV(readl(LPC32XX_HSUART_LEVEL(
    port.membase))) == 0)
    break;
    if (--timeout == 0)
    break;
    udelay(1);
    } while (1);
    }
#[no_mangle]
unsafe extern "C" fn wait_for_xmit_ready(port: *mut uart_port) {
    static void wait_for_xmit_ready(struct uart_port *port)
    {
    let mut timeout: c_uint = 10000;
    while (1) {
    if (LPC32XX_HSU_TX_LEV(readl(LPC32XX_HSUART_LEVEL(
    port.membase))) < 32)
    break;
    if (--timeout == 0)
    break;
    udelay(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_hsuart_console_putchar(port: *mut uart_port, ch: c_uchar) {
    static void lpc32xx_hsuart_console_putchar(struct uart_port *port, unsigned char ch)
    {
    wait_for_xmit_ready(port);
    writel((u32)ch, LPC32XX_HSUART_FIFO(port.membase));
    }
    static void lpc32xx_hsuart_console_write(struct console *co, const char *s,
    unsigned int count)
    {
    struct lpc32xx_hsuart_port *up = &lpc32xx_hs_ports[co.index];
    unsigned long flags;
    let mut locked: c_int = 1;
    touch_nmi_watchdog();
    if (oops_in_progress)
    locked = uart_port_trylock_irqsave(&up.port, &flags);
    else
    uart_port_lock_irqsave(&up.port, &flags);
    uart_console_write(&up.port, s, count, lpc32xx_hsuart_console_putchar);
    wait_for_xmit_empty(&up.port);
    if (locked)
    uart_port_unlock_irqrestore(&up.port, flags);
    }
    static int __init lpc32xx_hsuart_console_setup(struct console *co,
    char *options)
    {
    struct uart_port *port;
    let mut baud: c_int = 115200;
    let mut bits: c_int = 8;
    let mut parity: c_int = 'n';
    let mut flow: c_int = 'n';
    if (co.index >= MAX_PORTS)
    co.index = 0;
    port = &lpc32xx_hs_ports[co.index].port;
    if (!port.membase)
    return -ENODEV;
    if (options)
    uart_parse_options(options, &baud, &parity, &bits, &flow);
    lpc32xx_loopback_set(port.mapbase, 0); /* get out of loopback mode */
    return uart_set_options(port, co, baud, parity, bits, flow);
    }
    static struct uart_driver lpc32xx_hsuart_reg;
    static struct console lpc32xx_hsuart_console = {
    .name		= LPC32XX_TTY_NAME,
    .write		= lpc32xx_hsuart_console_write,
    .device		= uart_console_device,
    .setup		= lpc32xx_hsuart_console_setup,
    .flags		= CON_PRINTBUFFER,
    .index		= -1,
    .data		= &lpc32xx_hsuart_reg,
    };
#[no_mangle]
unsafe extern "C" fn lpc32xx_hsuart_console_init() -> int __init {
    static int __init lpc32xx_hsuart_console_init(void)
    {
    register_console(&lpc32xx_hsuart_console);
    return 0;
    }
    console_initcall(lpc32xx_hsuart_console_init);

    static struct uart_driver lpc32xx_hs_reg = {
    .owner		= THIS_MODULE,
    .driver_name	= MODNAME,
    .dev_name	= LPC32XX_TTY_NAME,
    .nr		= MAX_PORTS,
    .cons		= LPC32XX_HSUART_CONSOLE,
    };
    static int uarts_registered;
    static unsigned int __serial_get_clock_div(unsigned long uartclk,
    unsigned long rate)
    {
    u32 div, goodrate, hsu_rate, l_hsu_rate, comprate;
    u32 rate_diff;
// Find the closest divider to get the desired clock rate
    div = uartclk / rate;
    goodrate = hsu_rate = (div / 14) - 1;
    if (hsu_rate != 0)
    hsu_rate--;
// Tweak divider
    l_hsu_rate = hsu_rate + 3;
    rate_diff = 0xFFFFFFFF;
    while (hsu_rate < l_hsu_rate) {
    comprate = uartclk / ((hsu_rate + 1) * 14);
    if (abs(comprate - rate) < rate_diff) {
    goodrate = hsu_rate;
    rate_diff = abs(comprate - rate);
    }
    hsu_rate++;
    }
    return goodrate;
    }
#[no_mangle]
unsafe extern "C" fn __serial_uart_flush(port: *mut uart_port) {
    static void __serial_uart_flush(struct uart_port *port)
    {
    let mut cnt: c_int = 0;
    while ((readl(LPC32XX_HSUART_LEVEL(port.membase)) > 0) &&
    (cnt++ < FIFO_READ_LIMIT))
    readl(LPC32XX_HSUART_FIFO(port.membase));
    }
#[no_mangle]
unsafe extern "C" fn __serial_lpc32xx_rx(port: *mut uart_port) {
    static void __serial_lpc32xx_rx(struct uart_port *port)
    {
    struct tty_port *tport = &port.state.port;
    unsigned int tmp, flag;
// Read data from FIFO and push into terminal
    tmp = readl(LPC32XX_HSUART_FIFO(port.membase));
    while (!(tmp & LPC32XX_HSU_RX_EMPTY)) {
    flag = TTY_NORMAL;
    port.icount.rx++;
    if (tmp & LPC32XX_HSU_ERROR_DATA) {
// Framing error
    writel(LPC32XX_HSU_FE_INT,
    LPC32XX_HSUART_IIR(port.membase));
    port.icount.frame++;
    flag = TTY_FRAME;
    tty_insert_flip_char(tport, 0, TTY_FRAME);
    }
    if (!uart_prepare_sysrq_char(port, tmp & 0xff))
    tty_insert_flip_char(tport, (tmp & 0xFF), flag);
    tmp = readl(LPC32XX_HSUART_FIFO(port.membase));
    }
    tty_flip_buffer_push(tport);
    }
#[no_mangle]
unsafe extern "C" fn serial_lpc32xx_tx_ready(port: *mut uart_port) -> bool {
    static bool serial_lpc32xx_tx_ready(struct uart_port *port)
    {
    let mut level: u32 = readl(LPC32XX_HSUART_LEVEL(port.membase));
    return LPC32XX_HSU_TX_LEV(level) < 64;
    }
#[no_mangle]
unsafe extern "C" fn __serial_lpc32xx_tx(port: *mut uart_port) {
    static void __serial_lpc32xx_tx(struct uart_port *port)
    {
    u8 ch;
    uart_port_tx(port, ch,
    serial_lpc32xx_tx_ready(port),
    writel(ch, LPC32XX_HSUART_FIFO(port.membase)));
    }
#[no_mangle]
unsafe extern "C" fn serial_lpc32xx_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t serial_lpc32xx_interrupt(int irq, void *dev_id)
    {
    struct uart_port *port = dev_id;
    struct tty_port *tport = &port.state.port;
    u32 status;
    uart_port_lock(port);
// Read UART status and clear latched interrupts
    status = readl(LPC32XX_HSUART_IIR(port.membase));
    if (status & LPC32XX_HSU_BRK_INT) {
// Break received
    writel(LPC32XX_HSU_BRK_INT, LPC32XX_HSUART_IIR(port.membase));
    port.icount.brk++;
    uart_handle_break(port);
    }
// Framing error
    if (status & LPC32XX_HSU_FE_INT)
    writel(LPC32XX_HSU_FE_INT, LPC32XX_HSUART_IIR(port.membase));
    if (status & LPC32XX_HSU_RX_OE_INT) {
// Receive FIFO overrun
    writel(LPC32XX_HSU_RX_OE_INT,
    LPC32XX_HSUART_IIR(port.membase));
    port.icount.overrun++;
    tty_insert_flip_char(tport, 0, TTY_OVERRUN);
    tty_flip_buffer_push(tport);
    }
// Data received?
    if (status & (LPC32XX_HSU_RX_TIMEOUT_INT | LPC32XX_HSU_RX_TRIG_INT))
    __serial_lpc32xx_rx(port);
// Transmit data request?
    if ((status & LPC32XX_HSU_TX_INT) && (!uart_tx_stopped(port))) {
    writel(LPC32XX_HSU_TX_INT, LPC32XX_HSUART_IIR(port.membase));
    __serial_lpc32xx_tx(port);
    }
    uart_unlock_and_check_sysrq(port);
    return IRQ_HANDLED;
    }
// port->lock is not held.
#[no_mangle]
unsafe extern "C" fn serial_lpc32xx_tx_empty(port: *mut uart_port) -> c_uint {
    static unsigned int serial_lpc32xx_tx_empty(struct uart_port *port)
    {
    let mut ret: c_uint = 0;
    if (LPC32XX_HSU_TX_LEV(readl(LPC32XX_HSUART_LEVEL(port.membase))) == 0)
    ret = TIOCSER_TEMT;
    return ret;
    }
// port->lock held by caller.
    static void serial_lpc32xx_set_mctrl(struct uart_port *port,
    unsigned int mctrl)
    {
// No signals are supported on HS UARTs
    }
// port->lock is held by caller and interrupts are disabled.
#[no_mangle]
unsafe extern "C" fn serial_lpc32xx_get_mctrl(port: *mut uart_port) -> c_uint {
    static unsigned int serial_lpc32xx_get_mctrl(struct uart_port *port)
    {
// No signals are supported on HS UARTs
    return TIOCM_CAR | TIOCM_DSR | TIOCM_CTS;
    }
// port->lock held by caller.
#[no_mangle]
unsafe extern "C" fn serial_lpc32xx_stop_tx(port: *mut uart_port) {
    static void serial_lpc32xx_stop_tx(struct uart_port *port)
    {
    u32 tmp;
    tmp = readl(LPC32XX_HSUART_CTRL(port.membase));
    tmp &= ~LPC32XX_HSU_TX_INT_EN;
    writel(tmp, LPC32XX_HSUART_CTRL(port.membase));
    }
// port->lock held by caller.
#[no_mangle]
unsafe extern "C" fn serial_lpc32xx_start_tx(port: *mut uart_port) {
    static void serial_lpc32xx_start_tx(struct uart_port *port)
    {
    u32 tmp;
    __serial_lpc32xx_tx(port);
    tmp = readl(LPC32XX_HSUART_CTRL(port.membase));
    tmp |= LPC32XX_HSU_TX_INT_EN;
    writel(tmp, LPC32XX_HSUART_CTRL(port.membase));
    }
// port->lock held by caller.
#[no_mangle]
unsafe extern "C" fn serial_lpc32xx_stop_rx(port: *mut uart_port) {
    static void serial_lpc32xx_stop_rx(struct uart_port *port)
    {
    u32 tmp;
    tmp = readl(LPC32XX_HSUART_CTRL(port.membase));
    tmp &= ~(LPC32XX_HSU_RX_INT_EN | LPC32XX_HSU_ERR_INT_EN);
    writel(tmp, LPC32XX_HSUART_CTRL(port.membase));
    writel((LPC32XX_HSU_BRK_INT | LPC32XX_HSU_RX_OE_INT |
    LPC32XX_HSU_FE_INT), LPC32XX_HSUART_IIR(port.membase));
    }
// port->lock is not held.
    static void serial_lpc32xx_break_ctl(struct uart_port *port,
    int break_state)
    {
    unsigned long flags;
    u32 tmp;
    uart_port_lock_irqsave(port, &flags);
    tmp = readl(LPC32XX_HSUART_CTRL(port.membase));
    if (break_state != 0)
    tmp |= LPC32XX_HSU_BREAK;
    else
    tmp &= ~LPC32XX_HSU_BREAK;
    writel(tmp, LPC32XX_HSUART_CTRL(port.membase));
    uart_port_unlock_irqrestore(port, flags);
    }
// port->lock is not held.
#[no_mangle]
unsafe extern "C" fn serial_lpc32xx_startup(port: *mut uart_port) -> c_int {
    static int serial_lpc32xx_startup(struct uart_port *port)
    {
    int retval;
    unsigned long flags;
    u32 tmp;
    uart_port_lock_irqsave(port, &flags);
    __serial_uart_flush(port);
    writel((LPC32XX_HSU_TX_INT | LPC32XX_HSU_FE_INT |
    LPC32XX_HSU_BRK_INT | LPC32XX_HSU_RX_OE_INT),
    LPC32XX_HSUART_IIR(port.membase));
    writel(0xFF, LPC32XX_HSUART_RATE(port.membase));
//
// Set receiver timeout, HSU offset of 20, no break, no interrupts,
// and default FIFO trigger levels
//
    tmp = LPC32XX_HSU_TX_TL8B | LPC32XX_HSU_RX_TL32B |
    LPC32XX_HSU_OFFSET(20) | LPC32XX_HSU_TMO_INACT_4B;
    writel(tmp, LPC32XX_HSUART_CTRL(port.membase));
    lpc32xx_loopback_set(port.mapbase, 0); /* get out of loopback mode */
    uart_port_unlock_irqrestore(port, flags);
    retval = request_irq(port.irq, serial_lpc32xx_interrupt,
    0, MODNAME, port);
    if (!retval)
    writel((tmp | LPC32XX_HSU_RX_INT_EN | LPC32XX_HSU_ERR_INT_EN),
    LPC32XX_HSUART_CTRL(port.membase));
    return retval;
    }
// port->lock is not held.
#[no_mangle]
unsafe extern "C" fn serial_lpc32xx_shutdown(port: *mut uart_port) {
    static void serial_lpc32xx_shutdown(struct uart_port *port)
    {
    u32 tmp;
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
    tmp = LPC32XX_HSU_TX_TL8B | LPC32XX_HSU_RX_TL32B |
    LPC32XX_HSU_OFFSET(20) | LPC32XX_HSU_TMO_INACT_4B;
    writel(tmp, LPC32XX_HSUART_CTRL(port.membase));
    lpc32xx_loopback_set(port.mapbase, 1); /* go to loopback mode */
    uart_port_unlock_irqrestore(port, flags);
    free_irq(port.irq, port);
    }
// port->lock is not held.
    static void serial_lpc32xx_set_termios(struct uart_port *port,
    struct ktermios *termios,
    const struct ktermios *old)
    {
    unsigned long flags;
    unsigned int baud, quot;
    u32 tmp;
// Always 8-bit, no parity, 1 stop bit
    termios.c_cflag &= ~(CSIZE | CSTOPB | PARENB | PARODD);
    termios.c_cflag |= CS8;
    termios.c_cflag &= ~(HUPCL | CMSPAR | CLOCAL | CRTSCTS);
    baud = uart_get_baud_rate(port, termios, old, 0,
    port.uartclk / 14);
    quot = __serial_get_clock_div(port.uartclk, baud);
    uart_port_lock_irqsave(port, &flags);
// Ignore characters?
    tmp = readl(LPC32XX_HSUART_CTRL(port.membase));
    if ((termios.c_cflag & CREAD) == 0)
    tmp &= ~(LPC32XX_HSU_RX_INT_EN | LPC32XX_HSU_ERR_INT_EN);
    else
    tmp |= LPC32XX_HSU_RX_INT_EN | LPC32XX_HSU_ERR_INT_EN;
    writel(tmp, LPC32XX_HSUART_CTRL(port.membase));
    writel(quot, LPC32XX_HSUART_RATE(port.membase));
    uart_update_timeout(port, termios.c_cflag, baud);
    uart_port_unlock_irqrestore(port, flags);
// Don't rewrite B0
    if (tty_termios_baud_rate(termios))
    tty_termios_encode_baud_rate(termios, baud, baud);
    }
    static const char *serial_lpc32xx_type(struct uart_port *port)
    {
    return MODNAME;
    }
#[no_mangle]
unsafe extern "C" fn serial_lpc32xx_release_port(port: *mut uart_port) {
    static void serial_lpc32xx_release_port(struct uart_port *port)
    {
    if ((port.iotype == UPIO_MEM32) && (port.mapbase)) {
    if (port.flags & UPF_IOREMAP) {
    iounmap(port.membase);
    port.membase = core::ptr::null_mut();
    }
    release_mem_region(port.mapbase, SZ_4K);
    }
    }
#[no_mangle]
unsafe extern "C" fn serial_lpc32xx_request_port(port: *mut uart_port) -> c_int {
    static int serial_lpc32xx_request_port(struct uart_port *port)
    {
    let mut ret: c_int = -ENODEV;
    if ((port.iotype == UPIO_MEM32) && (port.mapbase)) {
    ret = 0;
    if (!request_mem_region(port.mapbase, SZ_4K, MODNAME))
    ret = -EBUSY;
#[no_mangle]
pub unsafe extern "C" fn if(UPF_IOREMAP: port->flags &) -> else {
    port.membase = ioremap(port.mapbase, SZ_4K);
    if (!port.membase) {
    release_mem_region(port.mapbase, SZ_4K);
    ret = -ENOMEM;
    }
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn serial_lpc32xx_config_port(port: *mut uart_port, uflags: c_int) {
    static void serial_lpc32xx_config_port(struct uart_port *port, int uflags)
    {
    int ret;
    ret = serial_lpc32xx_request_port(port);
    if (ret < 0)
    return;
    port.type = PORT_UART00;
    port.fifosize = 64;
    __serial_uart_flush(port);
    writel((LPC32XX_HSU_TX_INT | LPC32XX_HSU_FE_INT |
    LPC32XX_HSU_BRK_INT | LPC32XX_HSU_RX_OE_INT),
    LPC32XX_HSUART_IIR(port.membase));
    writel(0xFF, LPC32XX_HSUART_RATE(port.membase));
// Set receiver timeout, HSU offset of 20, no break, no interrupts,
    and default FIFO trigger levels */
    writel(LPC32XX_HSU_TX_TL8B | LPC32XX_HSU_RX_TL32B |
    LPC32XX_HSU_OFFSET(20) | LPC32XX_HSU_TMO_INACT_4B,
    LPC32XX_HSUART_CTRL(port.membase));
    }
    static int serial_lpc32xx_verify_port(struct uart_port *port,
    struct serial_struct *ser)
    {
    let mut ret: c_int = 0;
    if (ser.type != PORT_UART00)
    ret = -EINVAL;
    return ret;
    }
    static const struct uart_ops serial_lpc32xx_pops = {
    .tx_empty	= serial_lpc32xx_tx_empty,
    .set_mctrl	= serial_lpc32xx_set_mctrl,
    .get_mctrl	= serial_lpc32xx_get_mctrl,
    .stop_tx	= serial_lpc32xx_stop_tx,
    .start_tx	= serial_lpc32xx_start_tx,
    .stop_rx	= serial_lpc32xx_stop_rx,
    .break_ctl	= serial_lpc32xx_break_ctl,
    .startup	= serial_lpc32xx_startup,
    .shutdown	= serial_lpc32xx_shutdown,
    .set_termios	= serial_lpc32xx_set_termios,
    .type		= serial_lpc32xx_type,
    .release_port	= serial_lpc32xx_release_port,
    .request_port	= serial_lpc32xx_request_port,
    .config_port	= serial_lpc32xx_config_port,
    .verify_port	= serial_lpc32xx_verify_port,
    };
//
// Register a set of serial devices attached to a platform device
//
#[no_mangle]
unsafe extern "C" fn serial_hs_lpc32xx_probe(pdev: *mut platform_device) -> c_int {
    static int serial_hs_lpc32xx_probe(struct platform_device *pdev)
    {
    struct lpc32xx_hsuart_port *p = &lpc32xx_hs_ports[uarts_registered];
    let mut ret: c_int = 0;
    struct resource *res;
    if (uarts_registered >= MAX_PORTS) {
    dev_err(&pdev.dev,
    "Error: Number of possible ports exceeded (%d)!\n",
    uarts_registered + 1);
    return -ENXIO;
    }
    memset(p, 0, sizeof(*p));
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res) {
    dev_err(&pdev.dev,
    "Error getting mem resource for HS UART port %d\n",
    uarts_registered);
    return -ENXIO;
    }
    p.port.mapbase = res.start;
    p.port.membase = core::ptr::null_mut();
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    return ret;
    p.port.irq = ret;
    p.port.iotype = UPIO_MEM32;
    p.port.uartclk = LPC32XX_MAIN_OSC_FREQ;
    p.port.regshift = 2;
    p.port.flags = UPF_BOOT_AUTOCONF | UPF_FIXED_PORT | UPF_IOREMAP;
    p.port.dev = &pdev.dev;
    p.port.ops = &serial_lpc32xx_pops;
    p.port.line = uarts_registered++;
    spin_lock_init(&p.port.lock);
// send port to loopback mode by default
    lpc32xx_loopback_set(p.port.mapbase, 1);
    ret = uart_add_one_port(&lpc32xx_hs_reg, &p.port);
    platform_set_drvdata(pdev, p);
    return ret;
    }
//
// Remove serial ports registered against a platform device.
//
#[no_mangle]
unsafe extern "C" fn serial_hs_lpc32xx_remove(pdev: *mut platform_device) {
    static void serial_hs_lpc32xx_remove(struct platform_device *pdev)
    {
    struct lpc32xx_hsuart_port *p = platform_get_drvdata(pdev);
    uart_remove_one_port(&lpc32xx_hs_reg, &p.port);
    }

    static int serial_hs_lpc32xx_suspend(struct platform_device *pdev,
    pm_message_t state)
    {
    struct lpc32xx_hsuart_port *p = platform_get_drvdata(pdev);
    uart_suspend_port(&lpc32xx_hs_reg, &p.port);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serial_hs_lpc32xx_resume(pdev: *mut platform_device) -> c_int {
    static int serial_hs_lpc32xx_resume(struct platform_device *pdev)
    {
    struct lpc32xx_hsuart_port *p = platform_get_drvdata(pdev);
    uart_resume_port(&lpc32xx_hs_reg, &p.port);
    return 0;
    }

    static const struct of_device_id serial_hs_lpc32xx_dt_ids[] = {
    { .compatible = "nxp,lpc3220-hsuart" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, serial_hs_lpc32xx_dt_ids);
    static struct platform_driver serial_hs_lpc32xx_driver = {
    .probe		= serial_hs_lpc32xx_probe,
    .remove		= serial_hs_lpc32xx_remove,
    .suspend	= serial_hs_lpc32xx_suspend,
    .resume		= serial_hs_lpc32xx_resume,
    .driver		= {
    .name	= MODNAME,
    .of_match_table	= serial_hs_lpc32xx_dt_ids,
    },
    };
#[no_mangle]
unsafe extern "C" fn lpc32xx_hsuart_init() -> int __init {
    static int __init lpc32xx_hsuart_init(void)
    {
    int ret;
    ret = uart_register_driver(&lpc32xx_hs_reg);
    if (ret)
    return ret;
    ret = platform_driver_register(&serial_hs_lpc32xx_driver);
    if (ret)
    uart_unregister_driver(&lpc32xx_hs_reg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_hsuart_exit() -> void __exit {
    static void __exit lpc32xx_hsuart_exit(void)
    {
    platform_driver_unregister(&serial_hs_lpc32xx_driver);
    uart_unregister_driver(&lpc32xx_hs_reg);
    }
    module_init(lpc32xx_hsuart_init);
    module_exit(lpc32xx_hsuart_exit);
    MODULE_AUTHOR("Kevin Wells <kevin.wells@nxp.com>");
    MODULE_AUTHOR("Roland Stigge <stigge@antcom.de>");
    MODULE_DESCRIPTION("NXP LPC32XX High Speed UART driver");
    MODULE_LICENSE("GPL");

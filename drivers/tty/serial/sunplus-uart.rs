//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/sunplus-uart.c
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
// Sunplus SoC UART driver
//
// Author: Hammer Hsieh <hammerh0314@gmail.com>
//
// Note1: This driver is 8250-like uart, but are not register compatible.
//
// Note2: On some buses, for preventing data incoherence, must do a read
// for ensure write made it to hardware. In this driver, function startup
// and shutdown did not do a read but only do a write directly. For what?
// In Sunplus bus communication between memory bus and peripheral bus with
// posted write, it will send a specific command after last write command
// to make sure write done. Then memory bus identify the specific command
// and send done signal back to master device. After master device received
// done signal, then proceed next write command. It is no need to do a read
// before write.
//

// Register offsets
pub const SUP_UART_DATA: c_uint = 0x00;
pub const SUP_UART_LSR: c_uint = 0x04;
pub const SUP_UART_MSR: c_uint = 0x08;
pub const SUP_UART_LCR: c_uint = 0x0C;
pub const SUP_UART_MCR: c_uint = 0x10;
pub const SUP_UART_DIV_L: c_uint = 0x14;
pub const SUP_UART_DIV_H: c_uint = 0x18;
pub const SUP_UART_ISC: c_uint = 0x1C;
pub const SUP_UART_TX_RESIDUE: c_uint = 0x20;
pub const SUP_UART_RX_RESIDUE: c_uint = 0x24;
// Line Status Register bits

pub const SUP_UART_LSR_TX_NOT_FULL: c_int = 1;

// Line Control Register bits

// Modem Control Register bits

// Interrupt Status/Control Register bits

pub const SUP_UART_NR: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunplus_uart_port {
    pub port: uart_port,
    pub clk: *mut clk,
    pub rstc: *mut reset_control,
}

#[no_mangle]
unsafe extern "C" fn sp_uart_put_char(port: *mut uart_port, ch: c_uint) {
    static void sp_uart_put_char(struct uart_port *port, unsigned int ch)
    {
    writel(ch, port.membase + SUP_UART_DATA);
    }
#[no_mangle]
unsafe extern "C" fn sunplus_tx_buf_not_full(port: *mut uart_port) -> u32 {
    static u32 sunplus_tx_buf_not_full(struct uart_port *port)
    {
    let mut lsr: c_uint = readl(port.membase + SUP_UART_LSR);
    return (lsr & SUP_UART_LSR_TX) ? SUP_UART_LSR_TX_NOT_FULL : 0;
    }
#[no_mangle]
unsafe extern "C" fn sunplus_tx_empty(port: *mut uart_port) -> c_uint {
    static unsigned int sunplus_tx_empty(struct uart_port *port)
    {
    let mut lsr: c_uint = readl(port.membase + SUP_UART_LSR);
    return (lsr & UART_LSR_TEMT) ? TIOCSER_TEMT : 0;
    }
#[no_mangle]
unsafe extern "C" fn sunplus_set_mctrl(port: *mut uart_port, mctrl: c_uint) {
    static void sunplus_set_mctrl(struct uart_port *port, unsigned int mctrl)
    {
    let mut mcr: c_uint = readl(port.membase + SUP_UART_MCR);
    if (mctrl & TIOCM_DTR)
    mcr |= UART_MCR_DTR;
    else
    mcr &= ~UART_MCR_DTR;
    if (mctrl & TIOCM_RTS)
    mcr |= UART_MCR_RTS;
    else
    mcr &= ~UART_MCR_RTS;
    if (mctrl & TIOCM_CAR)
    mcr |= SUP_UART_MCR_DCD;
    else
    mcr &= ~SUP_UART_MCR_DCD;
    if (mctrl & TIOCM_RI)
    mcr |= SUP_UART_MCR_RI;
    else
    mcr &= ~SUP_UART_MCR_RI;
    if (mctrl & TIOCM_LOOP)
    mcr |= UART_MCR_LOOP;
    else
    mcr &= ~UART_MCR_LOOP;
    writel(mcr, port.membase + SUP_UART_MCR);
    }
#[no_mangle]
unsafe extern "C" fn sunplus_get_mctrl(port: *mut uart_port) -> c_uint {
    static unsigned int sunplus_get_mctrl(struct uart_port *port)
    {
    unsigned int mcr, ret = 0;
    mcr = readl(port.membase + SUP_UART_MCR);
    if (mcr & UART_MCR_DTR)
    ret |= TIOCM_DTR;
    if (mcr & UART_MCR_RTS)
    ret |= TIOCM_RTS;
    if (mcr & SUP_UART_MCR_DCD)
    ret |= TIOCM_CAR;
    if (mcr & SUP_UART_MCR_RI)
    ret |= TIOCM_RI;
    if (mcr & UART_MCR_LOOP)
    ret |= TIOCM_LOOP;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sunplus_stop_tx(port: *mut uart_port) {
    static void sunplus_stop_tx(struct uart_port *port)
    {
    unsigned int isc;
    isc = readl(port.membase + SUP_UART_ISC);
    isc &= ~SUP_UART_ISC_TXM;
    writel(isc, port.membase + SUP_UART_ISC);
    }
#[no_mangle]
unsafe extern "C" fn sunplus_start_tx(port: *mut uart_port) {
    static void sunplus_start_tx(struct uart_port *port)
    {
    unsigned int isc;
    isc = readl(port.membase + SUP_UART_ISC);
    isc |= SUP_UART_ISC_TXM;
    writel(isc, port.membase + SUP_UART_ISC);
    }
#[no_mangle]
unsafe extern "C" fn sunplus_stop_rx(port: *mut uart_port) {
    static void sunplus_stop_rx(struct uart_port *port)
    {
    unsigned int isc;
    isc = readl(port.membase + SUP_UART_ISC);
    isc &= ~SUP_UART_ISC_RXM;
    writel(isc, port.membase + SUP_UART_ISC);
    }
#[no_mangle]
unsafe extern "C" fn sunplus_break_ctl(port: *mut uart_port, ctl: c_int) {
    static void sunplus_break_ctl(struct uart_port *port, int ctl)
    {
    unsigned long flags;
    unsigned int lcr;
    uart_port_lock_irqsave(port, &flags);
    lcr = readl(port.membase + SUP_UART_LCR);
    if (ctl)
    lcr |= SUP_UART_LCR_SBC; /* start break */
    else
    lcr &= ~SUP_UART_LCR_SBC; /* stop break */
    writel(lcr, port.membase + SUP_UART_LCR);
    uart_port_unlock_irqrestore(port, flags);
    }
#[no_mangle]
unsafe extern "C" fn transmit_chars(port: *mut uart_port) {
    static void transmit_chars(struct uart_port *port)
    {
    struct tty_port *tport = &port.state.port;
    if (port.x_char) {
    sp_uart_put_char(port, port.x_char);
    port.icount.tx++;
    port.x_char = 0;
    return;
    }
    if (kfifo_is_empty(&tport.xmit_fifo) || uart_tx_stopped(port)) {
    sunplus_stop_tx(port);
    return;
    }
    do {
    unsigned char ch;
    if (!uart_fifo_get(port, &ch))
    break;
    sp_uart_put_char(port, ch);
    } while (sunplus_tx_buf_not_full(port));
    if (kfifo_len(&tport.xmit_fifo) < WAKEUP_CHARS)
    uart_write_wakeup(port);
    if (kfifo_is_empty(&tport.xmit_fifo))
    sunplus_stop_tx(port);
    }
#[no_mangle]
unsafe extern "C" fn receive_chars(port: *mut uart_port) {
    static void receive_chars(struct uart_port *port)
    {
    let mut lsr: c_uint = readl(port.membase + SUP_UART_LSR);
    u8 ch, flag;
    do {
    ch = readl(port.membase + SUP_UART_DATA);
    flag = TTY_NORMAL;
    port.icount.rx++;
    if (unlikely(lsr & SUP_UART_LSR_BRK_ERROR_BITS)) {
    if (lsr & SUP_UART_LSR_BC) {
    lsr &= ~(SUP_UART_LSR_FE | SUP_UART_LSR_PE);
    port.icount.brk++;
    flag = TTY_BREAK;
    if (uart_handle_break(port))
    goto ignore_char;
    } else if (lsr & SUP_UART_LSR_PE) {
    port.icount.parity++;
    flag = TTY_PARITY;
    } else if (lsr & SUP_UART_LSR_FE) {
    port.icount.frame++;
    flag = TTY_FRAME;
    }
    if (lsr & SUP_UART_LSR_OE)
    port.icount.overrun++;
    }
    if (port.ignore_status_mask & SUP_DUMMY_READ)
    goto ignore_char;
    if (uart_prepare_sysrq_char(port, ch))
    goto ignore_char;
    uart_insert_char(port, lsr, SUP_UART_LSR_OE, ch, flag);
    ignore_char:
    lsr = readl(port.membase + SUP_UART_LSR);
    } while (lsr & SUP_UART_LSR_RX);
    tty_flip_buffer_push(&port.state.port);
    }
#[no_mangle]
unsafe extern "C" fn sunplus_uart_irq(irq: c_int, args: *mut c_void) -> irqreturn_t {
    static irqreturn_t sunplus_uart_irq(int irq, void *args)
    {
    struct uart_port *port = args;
    unsigned int isc;
    uart_port_lock(port);
    isc = readl(port.membase + SUP_UART_ISC);
    if (isc & SUP_UART_ISC_RX)
    receive_chars(port);
    if (isc & SUP_UART_ISC_TX)
    transmit_chars(port);
    uart_unlock_and_check_sysrq(port);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sunplus_startup(port: *mut uart_port) -> c_int {
    static int sunplus_startup(struct uart_port *port)
    {
    unsigned long flags;
    let mut isc: c_uint = 0;
    int ret;
    ret = request_irq(port.irq, sunplus_uart_irq, 0, "sunplus_uart", port);
    if (ret)
    return ret;
    uart_port_lock_irqsave(port, &flags);
// isc define Bit[7:4] int setting, Bit[3:0] int status
// isc register will clean Bit[3:0] int status after read
// only do a write to Bit[7:4] int setting
//
    isc |= SUP_UART_ISC_RXM;
    writel(isc, port.membase + SUP_UART_ISC);
    uart_port_unlock_irqrestore(port, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sunplus_shutdown(port: *mut uart_port) {
    static void sunplus_shutdown(struct uart_port *port)
    {
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
// isc define Bit[7:4] int setting, Bit[3:0] int status
// isc register will clean Bit[3:0] int status after read
// only do a write to Bit[7:4] int setting
//
    writel(0, port.membase + SUP_UART_ISC); /* disable all interrupt */
    uart_port_unlock_irqrestore(port, flags);
    free_irq(port.irq, port);
    }
    static void sunplus_set_termios(struct uart_port *port,
    struct ktermios *termios,
    const struct ktermios *oldtermios)
    {
    u32 ext, div, div_l, div_h, baud, lcr;
    let mut clk: u32 = port.uartclk;
    unsigned long flags;
    baud = uart_get_baud_rate(port, termios, oldtermios, 0, port.uartclk / 16);
// baud rate = uartclk / ((16 * divisor + 1) + divisor_ext)
    clk += baud >> 1;
    div = clk / baud;
    ext = div & 0x0F;
    div = (div >> 4) - 1;
    div_l = (div & 0xFF) | (ext << 12);
    div_h = div >> 8;
    switch (termios.c_cflag & CSIZE) {
    case CS5:
    lcr = UART_LCR_WLEN5;
    break;
    case CS6:
    lcr = UART_LCR_WLEN6;
    break;
    case CS7:
    lcr = UART_LCR_WLEN7;
    break;
    default:
    lcr = UART_LCR_WLEN8;
    break;
    }
    if (termios.c_cflag & CSTOPB)
    lcr |= UART_LCR_STOP;
    if (termios.c_cflag & PARENB) {
    lcr |= UART_LCR_PARITY;
    if (!(termios.c_cflag & PARODD))
    lcr |= UART_LCR_EPAR;
    }
    uart_port_lock_irqsave(port, &flags);
    uart_update_timeout(port, termios.c_cflag, baud);
    port.read_status_mask = 0;
    if (termios.c_iflag & INPCK)
    port.read_status_mask |= SUP_UART_LSR_PE | SUP_UART_LSR_FE;
    if (termios.c_iflag & (BRKINT | PARMRK))
    port.read_status_mask |= SUP_UART_LSR_BC;
// Characters to ignore
    port.ignore_status_mask = 0;
    if (termios.c_iflag & IGNPAR)
    port.ignore_status_mask |= SUP_UART_LSR_FE | SUP_UART_LSR_PE;
    if (termios.c_iflag & IGNBRK) {
    port.ignore_status_mask |= SUP_UART_LSR_BC;
    if (termios.c_iflag & IGNPAR)
    port.ignore_status_mask |= SUP_UART_LSR_OE;
    }
// Ignore all characters if CREAD is not set
    if ((termios.c_cflag & CREAD) == 0) {
    port.ignore_status_mask |= SUP_DUMMY_READ;
// flush rx data FIFO
    writel(0, port.membase + SUP_UART_RX_RESIDUE);
    }
// Settings for baud rate divisor and lcr
    writel(div_h, port.membase + SUP_UART_DIV_H);
    writel(div_l, port.membase + SUP_UART_DIV_L);
    writel(lcr, port.membase + SUP_UART_LCR);
    uart_port_unlock_irqrestore(port, flags);
    }
#[no_mangle]
unsafe extern "C" fn sunplus_set_ldisc(port: *mut uart_port, termios: *mut ktermios) {
    static void sunplus_set_ldisc(struct uart_port *port, struct ktermios *termios)
    {
    let mut new: c_int = termios.c_line;
    if (new == N_PPS)
    port.flags |= UPF_HARDPPS_CD;
    else
    port.flags &= ~UPF_HARDPPS_CD;
    }
    static const char *sunplus_type(struct uart_port *port)
    {
    return port.type == PORT_SUNPLUS ? "sunplus_uart" : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn sunplus_config_port(port: *mut uart_port, type: c_int) {
    static void sunplus_config_port(struct uart_port *port, int type)
    {
    if (type & UART_CONFIG_TYPE)
    port.type = PORT_SUNPLUS;
    }
#[no_mangle]
unsafe extern "C" fn sunplus_verify_port(port: *mut uart_port, ser: *mut serial_struct) -> c_int {
    static int sunplus_verify_port(struct uart_port *port, struct serial_struct *ser)
    {
    if (ser.type != PORT_UNKNOWN && ser.type != PORT_SUNPLUS)
    return -EINVAL;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn wait_for_xmitr(port: *mut uart_port) {
    static void wait_for_xmitr(struct uart_port *port)
    {
    unsigned int val;
    int ret;
// Wait while FIFO is full or timeout
    ret = readl_poll_timeout_atomic(port.membase + SUP_UART_LSR, val,
    (val & SUP_UART_LSR_TX), 1, 10000);
    if (ret == -ETIMEDOUT) {
    dev_err(port.dev, "Timeout waiting while UART TX FULL\n");
    return;
    }
    }

#[no_mangle]
unsafe extern "C" fn sunplus_poll_put_char(port: *mut uart_port, data: c_uchar) {
    static void sunplus_poll_put_char(struct uart_port *port, unsigned char data)
    {
    wait_for_xmitr(port);
    sp_uart_put_char(port, data);
    }
#[no_mangle]
unsafe extern "C" fn sunplus_poll_get_char(port: *mut uart_port) -> c_int {
    static int sunplus_poll_get_char(struct uart_port *port)
    {
    let mut lsr: c_uint = readl(port.membase + SUP_UART_LSR);
    if (!(lsr & SUP_UART_LSR_RX))
    return NO_POLL_CHAR;
    return readl(port.membase + SUP_UART_DATA);
    }

    static const struct uart_ops sunplus_uart_ops = {
    .tx_empty	= sunplus_tx_empty,
    .set_mctrl	= sunplus_set_mctrl,
    .get_mctrl	= sunplus_get_mctrl,
    .stop_tx	= sunplus_stop_tx,
    .start_tx	= sunplus_start_tx,
    .stop_rx	= sunplus_stop_rx,
    .break_ctl	= sunplus_break_ctl,
    .startup	= sunplus_startup,
    .shutdown	= sunplus_shutdown,
    .set_termios	= sunplus_set_termios,
    .set_ldisc	= sunplus_set_ldisc,
    .type		= sunplus_type,
    .config_port	= sunplus_config_port,
    .verify_port	= sunplus_verify_port,

    .poll_put_char	= sunplus_poll_put_char,
    .poll_get_char	= sunplus_poll_get_char,

    };

    static struct sunplus_uart_port *sunplus_console_ports[SUP_UART_NR];
    static void sunplus_uart_console_putchar(struct uart_port *port,
    unsigned char ch)
    {
    wait_for_xmitr(port);
    sp_uart_put_char(port, ch);
    }
    static void sunplus_console_write(struct console *co,
    const char *s,
    unsigned int count)
    {
    unsigned long flags;
    let mut locked: c_int = 1;
    if (oops_in_progress)
    locked = uart_port_trylock_irqsave(&sunplus_console_ports[co.index].port, &flags);
    else
    uart_port_lock_irqsave(&sunplus_console_ports[co.index].port, &flags);
    uart_console_write(&sunplus_console_ports[co.index].port, s, count,
    sunplus_uart_console_putchar);
    if (locked)
    uart_port_unlock_irqrestore(&sunplus_console_ports[co.index].port, flags);
    }
#[no_mangle]
unsafe extern "C" fn sunplus_console_setup(co: *mut console, options: *mut c_char) -> int __init {
    static int __init sunplus_console_setup(struct console *co, char *options)
    {
    struct sunplus_uart_port *sup;
    let mut baud: c_int = 115200;
    let mut bits: c_int = 8;
    let mut parity: c_int = 'n';
    let mut flow: c_int = 'n';
    if (co.index < 0 || co.index >= SUP_UART_NR)
    return -EINVAL;
    sup = sunplus_console_ports[co.index];
    if (!sup)
    return -ENODEV;
    if (options)
    uart_parse_options(options, &baud, &parity, &bits, &flow);
    return uart_set_options(&sup.port, co, baud, parity, bits, flow);
    }
    static struct uart_driver sunplus_uart_driver;
    static struct console sunplus_uart_console = {
    .name		= "ttySUP",
    .write		= sunplus_console_write,
    .device		= uart_console_device,
    .setup		= sunplus_console_setup,
    .flags		= CON_PRINTBUFFER,
    .index		= -1,
    .data		= &sunplus_uart_driver
    };

    static struct uart_driver sunplus_uart_driver = {
    .owner		= THIS_MODULE,
    .driver_name	= "sunplus_uart",
    .dev_name	= "ttySUP",
    .major		= TTY_MAJOR,
    .minor		= 64,
    .nr		= SUP_UART_NR,
    .cons		= SERIAL_SUNPLUS_CONSOLE,
    };
#[no_mangle]
unsafe extern "C" fn sunplus_uart_disable_unprepare(data: *mut c_void) {
    static void sunplus_uart_disable_unprepare(void *data)
    {
    clk_disable_unprepare(data);
    }
#[no_mangle]
unsafe extern "C" fn sunplus_uart_reset_control_assert(data: *mut c_void) {
    static void sunplus_uart_reset_control_assert(void *data)
    {
    reset_control_assert(data);
    }
#[no_mangle]
unsafe extern "C" fn sunplus_uart_probe(pdev: *mut platform_device) -> c_int {
    static int sunplus_uart_probe(struct platform_device *pdev)
    {
    struct sunplus_uart_port *sup;
    struct uart_port *port;
    struct resource *res;
    int ret, irq;
    pdev.id = of_alias_get_id(pdev.dev.of_node, "serial");
    if (pdev.id < 0 || pdev.id >= SUP_UART_NR)
    return -EINVAL;
    sup = devm_kzalloc(&pdev.dev, sizeof(*sup), GFP_KERNEL);
    if (!sup)
    return -ENOMEM;
    sup.clk = devm_clk_get_optional(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(sup.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(sup.clk), "clk not found\n");
    ret = clk_prepare_enable(sup.clk);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(&pdev.dev, sunplus_uart_disable_unprepare, sup.clk);
    if (ret)
    return ret;
    sup.rstc = devm_reset_control_get_exclusive(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(sup.rstc))
    return dev_err_probe(&pdev.dev, PTR_ERR(sup.rstc), "rstc not found\n");
    port = &sup.port;
    port.membase = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(port.membase))
    return dev_err_probe(&pdev.dev, PTR_ERR(port.membase), "membase not found\n");
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    port.mapbase = res.start;
    port.uartclk = clk_get_rate(sup.clk);
    port.line = pdev.id;
    port.irq = irq;
    port.dev = &pdev.dev;
    port.iotype = UPIO_MEM;
    port.ops = &sunplus_uart_ops;
    port.flags = UPF_BOOT_AUTOCONF;
    port.fifosize = 128;
    ret = reset_control_deassert(sup.rstc);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(&pdev.dev, sunplus_uart_reset_control_assert, sup.rstc);
    if (ret)
    return ret;

    sunplus_console_ports[sup.port.line] = sup;

    platform_set_drvdata(pdev, &sup.port);
    ret = uart_add_one_port(&sunplus_uart_driver, &sup.port);

    if (ret)
    sunplus_console_ports[sup.port.line] = core::ptr::null_mut();

    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sunplus_uart_remove(pdev: *mut platform_device) {
    static void sunplus_uart_remove(struct platform_device *pdev)
    {
    struct sunplus_uart_port *sup = platform_get_drvdata(pdev);
    uart_remove_one_port(&sunplus_uart_driver, &sup.port);
    }
#[no_mangle]
unsafe extern "C" fn sunplus_uart_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused sunplus_uart_suspend(struct device *dev)
    {
    struct sunplus_uart_port *sup = dev_get_drvdata(dev);
    if (!uart_console(&sup.port))
    uart_suspend_port(&sunplus_uart_driver, &sup.port);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sunplus_uart_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused sunplus_uart_resume(struct device *dev)
    {
    struct sunplus_uart_port *sup = dev_get_drvdata(dev);
    if (!uart_console(&sup.port))
    uart_resume_port(&sunplus_uart_driver, &sup.port);
    return 0;
    }
    static const struct dev_pm_ops sunplus_uart_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(sunplus_uart_suspend, sunplus_uart_resume)
    };
    static const struct of_device_id sp_uart_of_match[] = {
    { .compatible = "sunplus,sp7021-uart" },
    {}
    };
    MODULE_DEVICE_TABLE(of, sp_uart_of_match);
    static struct platform_driver sunplus_uart_platform_driver = {
    .probe		= sunplus_uart_probe,
    .remove		= sunplus_uart_remove,
    .driver = {
    .name	= "sunplus_uart",
    .of_match_table = sp_uart_of_match,
    .pm     = &sunplus_uart_pm_ops,
    }
    };
#[no_mangle]
unsafe extern "C" fn sunplus_uart_init() -> int __init {
    static int __init sunplus_uart_init(void)
    {
    int ret;
    ret = uart_register_driver(&sunplus_uart_driver);
    if (ret)
    return ret;
    ret = platform_driver_register(&sunplus_uart_platform_driver);
    if (ret)
    uart_unregister_driver(&sunplus_uart_driver);
    return ret;
    }
    module_init(sunplus_uart_init);
#[no_mangle]
unsafe extern "C" fn sunplus_uart_exit() -> void __exit {
    static void __exit sunplus_uart_exit(void)
    {
    platform_driver_unregister(&sunplus_uart_platform_driver);
    uart_unregister_driver(&sunplus_uart_driver);
    }
    module_exit(sunplus_uart_exit);

#[no_mangle]
unsafe extern "C" fn sunplus_uart_putc(port: *mut uart_port, c: c_uchar) {
    static void sunplus_uart_putc(struct uart_port *port, unsigned char c)
    {
    unsigned int val;
    int ret;
    ret = readl_poll_timeout_atomic(port.membase + SUP_UART_LSR, val,
    (val & UART_LSR_TEMT), 1, 10000);
    if (ret)
    return;
    writel(c, port.membase + SUP_UART_DATA);
    }
#[no_mangle]
unsafe extern "C" fn sunplus_uart_early_write(con: *mut console, s: *const c_char, n: c_uint) {
    static void sunplus_uart_early_write(struct console *con, const char *s, unsigned int n)
    {
    struct earlycon_device *dev = con.data;
    uart_console_write(&dev.port, s, n, sunplus_uart_putc);
    }
    static int __init
    sunplus_uart_early_setup(struct earlycon_device *dev, const char *opt)
    {
    if (!(dev.port.membase || dev.port.iobase))
    return -ENODEV;
    dev.con.write = sunplus_uart_early_write;
    return 0;
    }
    OF_EARLYCON_DECLARE(sunplus_uart, "sunplus,sp7021-uart", sunplus_uart_early_setup);

    MODULE_DESCRIPTION("Sunplus UART driver");
    MODULE_AUTHOR("Hammer Hsieh <hammerh0314@gmail.com>");
    MODULE_LICENSE("GPL v2");

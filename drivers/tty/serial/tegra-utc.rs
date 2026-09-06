//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/tegra-utc.c
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


// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2025 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
// NVIDIA Tegra UTC (UART Trace Controller) driver.

pub const TEGRA_UTC_ENABLE: c_uint = 0x000;

pub const TEGRA_UTC_FIFO_THRESHOLD: c_uint = 0x008;
pub const TEGRA_UTC_COMMAND: c_uint = 0x00c;

pub const TEGRA_UTC_DATA: c_uint = 0x020;
pub const TEGRA_UTC_FIFO_STATUS: c_uint = 0x100;

pub const TEGRA_UTC_FIFO_OCCUPANCY: c_uint = 0x104;
pub const TEGRA_UTC_INTR_STATUS: c_uint = 0x108;
pub const TEGRA_UTC_INTR_SET: c_uint = 0x10c;
pub const TEGRA_UTC_INTR_MASK: c_uint = 0x110;
pub const TEGRA_UTC_INTR_CLEAR: c_uint = 0x114;

pub const TEGRA_UTC_UART_NR: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_utc_port {

    pub console: console,

    pub port: uart_port,
    pub rx_base: *mut void __iomem,
    pub tx_base: *mut void __iomem,
    pub tx_irqmask: u32,
    pub rx_irqmask: u32,
    pub fifosize: c_uint,
    pub tx_threshold: u32,
    pub rx_threshold: u32,
}

#[no_mangle]
unsafe extern "C" fn tegra_utc_rx_readl(tup: *mut tegra_utc_port, offset: c_uint) -> u32 {
    static u32 tegra_utc_rx_readl(struct tegra_utc_port *tup, unsigned int offset)
    {
    void __iomem *addr = tup.rx_base + offset;
    return readl_relaxed(addr);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_rx_writel(tup: *mut tegra_utc_port, val: u32, offset: c_uint) {
    static void tegra_utc_rx_writel(struct tegra_utc_port *tup, u32 val, unsigned int offset)
    {
    void __iomem *addr = tup.rx_base + offset;
    writel_relaxed(val, addr);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_tx_readl(tup: *mut tegra_utc_port, offset: c_uint) -> u32 {
    static u32 tegra_utc_tx_readl(struct tegra_utc_port *tup, unsigned int offset)
    {
    void __iomem *addr = tup.tx_base + offset;
    return readl_relaxed(addr);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_tx_writel(tup: *mut tegra_utc_port, val: u32, offset: c_uint) {
    static void tegra_utc_tx_writel(struct tegra_utc_port *tup, u32 val, unsigned int offset)
    {
    void __iomem *addr = tup.tx_base + offset;
    writel_relaxed(val, addr);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_enable_tx_irq(tup: *mut tegra_utc_port) {
    static void tegra_utc_enable_tx_irq(struct tegra_utc_port *tup)
    {
    tup.tx_irqmask = TEGRA_UTC_INTR_REQ;
    tegra_utc_tx_writel(tup, tup.tx_irqmask, TEGRA_UTC_INTR_MASK);
    tegra_utc_tx_writel(tup, tup.tx_irqmask, TEGRA_UTC_INTR_SET);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_disable_tx_irq(tup: *mut tegra_utc_port) {
    static void tegra_utc_disable_tx_irq(struct tegra_utc_port *tup)
    {
    tup.tx_irqmask = 0x0;
    tegra_utc_tx_writel(tup, tup.tx_irqmask, TEGRA_UTC_INTR_MASK);
    tegra_utc_tx_writel(tup, tup.tx_irqmask, TEGRA_UTC_INTR_SET);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_stop_tx(port: *mut uart_port) {
    static void tegra_utc_stop_tx(struct uart_port *port)
    {
    struct tegra_utc_port *tup = container_of(port, struct tegra_utc_port, port);
    tegra_utc_disable_tx_irq(tup);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_init_tx(tup: *mut tegra_utc_port) {
    static void tegra_utc_init_tx(struct tegra_utc_port *tup)
    {
// Disable TX.
    tegra_utc_tx_writel(tup, 0x0, TEGRA_UTC_ENABLE);
// Update the FIFO Threshold.
    tegra_utc_tx_writel(tup, tup.tx_threshold, TEGRA_UTC_FIFO_THRESHOLD);
// Clear and mask all the interrupts.
    tegra_utc_tx_writel(tup, TEGRA_UTC_INTR_COMMON, TEGRA_UTC_INTR_CLEAR);
    tegra_utc_disable_tx_irq(tup);
// Enable TX.
    tegra_utc_tx_writel(tup, TEGRA_UTC_ENABLE_CLIENT_ENABLE, TEGRA_UTC_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_init_rx(tup: *mut tegra_utc_port) {
    static void tegra_utc_init_rx(struct tegra_utc_port *tup)
    {
    tup.rx_irqmask = TEGRA_UTC_INTR_REQ | TEGRA_UTC_INTR_TIMEOUT;
    tegra_utc_rx_writel(tup, TEGRA_UTC_COMMAND_RESET, TEGRA_UTC_COMMAND);
    tegra_utc_rx_writel(tup, tup.rx_threshold, TEGRA_UTC_FIFO_THRESHOLD);
// Clear all the pending interrupts.
    tegra_utc_rx_writel(tup, TEGRA_UTC_INTR_TIMEOUT | TEGRA_UTC_INTR_OVERFLOW |
    TEGRA_UTC_INTR_COMMON, TEGRA_UTC_INTR_CLEAR);
    tegra_utc_rx_writel(tup, tup.rx_irqmask, TEGRA_UTC_INTR_MASK);
    tegra_utc_rx_writel(tup, tup.rx_irqmask, TEGRA_UTC_INTR_SET);
// Enable RX.
    tegra_utc_rx_writel(tup, TEGRA_UTC_ENABLE_CLIENT_ENABLE, TEGRA_UTC_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_tx_chars(tup: *mut tegra_utc_port) -> bool {
    static bool tegra_utc_tx_chars(struct tegra_utc_port *tup)
    {
    struct uart_port *port = &tup.port;
    unsigned int pending;
    u8 c;
    pending = uart_port_tx(port, c,
    !(tegra_utc_tx_readl(tup, TEGRA_UTC_FIFO_STATUS) & TEGRA_UTC_FIFO_FULL),
    tegra_utc_tx_writel(tup, c, TEGRA_UTC_DATA));
    return pending;
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_rx_chars(tup: *mut tegra_utc_port) {
    static void tegra_utc_rx_chars(struct tegra_utc_port *tup)
    {
    struct tty_port *port = &tup.port.state.port;
    let mut max_chars: c_uint = 256;
    u32 status;
    int sysrq;
    u32 ch;
    while (max_chars--) {
    status = tegra_utc_rx_readl(tup, TEGRA_UTC_FIFO_STATUS);
    if (status & TEGRA_UTC_FIFO_EMPTY)
    break;
    ch = tegra_utc_rx_readl(tup, TEGRA_UTC_DATA);
    tup.port.icount.rx++;
    if (status & TEGRA_UTC_FIFO_OVERFLOW)
    tup.port.icount.overrun++;
    uart_port_unlock(&tup.port);
    sysrq = uart_handle_sysrq_char(&tup.port, ch);
    uart_port_lock(&tup.port);
    if (!sysrq)
    tty_insert_flip_char(port, ch, TTY_NORMAL);
    }
    tty_flip_buffer_push(port);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t tegra_utc_isr(int irq, void *dev_id)
    {
    struct tegra_utc_port *tup = dev_id;
    let mut handled: c_uint = 0;
    u32 status;
    uart_port_lock(&tup.port);
// Process RX_REQ and RX_TIMEOUT interrupts.
    do {
    status = tegra_utc_rx_readl(tup, TEGRA_UTC_INTR_STATUS) & tup.rx_irqmask;
    if (status) {
    tegra_utc_rx_writel(tup, tup.rx_irqmask, TEGRA_UTC_INTR_CLEAR);
    tegra_utc_rx_chars(tup);
    handled = 1;
    }
    } while (status);
// Process TX_REQ interrupt.
    do {
    status = tegra_utc_tx_readl(tup, TEGRA_UTC_INTR_STATUS) & tup.tx_irqmask;
    if (status) {
    tegra_utc_tx_writel(tup, tup.tx_irqmask, TEGRA_UTC_INTR_CLEAR);
    tegra_utc_tx_chars(tup);
    handled = 1;
    }
    } while (status);
    uart_port_unlock(&tup.port);
    return IRQ_RETVAL(handled);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_tx_empty(port: *mut uart_port) -> c_uint {
    static unsigned int tegra_utc_tx_empty(struct uart_port *port)
    {
    struct tegra_utc_port *tup = container_of(port, struct tegra_utc_port, port);
    return tegra_utc_tx_readl(tup, TEGRA_UTC_FIFO_OCCUPANCY) ? 0 : TIOCSER_TEMT;
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_set_mctrl(port: *mut uart_port, mctrl: c_uint) {
    static void tegra_utc_set_mctrl(struct uart_port *port, unsigned int mctrl)
    {
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_get_mctrl(port: *mut uart_port) -> c_uint {
    static unsigned int tegra_utc_get_mctrl(struct uart_port *port)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_start_tx(port: *mut uart_port) {
    static void tegra_utc_start_tx(struct uart_port *port)
    {
    struct tegra_utc_port *tup = container_of(port, struct tegra_utc_port, port);
    if (tegra_utc_tx_chars(tup))
    tegra_utc_enable_tx_irq(tup);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_stop_rx(port: *mut uart_port) {
    static void tegra_utc_stop_rx(struct uart_port *port)
    {
    struct tegra_utc_port *tup = container_of(port, struct tegra_utc_port, port);
    tup.rx_irqmask = 0x0;
    tegra_utc_rx_writel(tup, tup.rx_irqmask, TEGRA_UTC_INTR_MASK);
    tegra_utc_rx_writel(tup, tup.rx_irqmask, TEGRA_UTC_INTR_SET);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_hw_init(tup: *mut tegra_utc_port) {
    static void tegra_utc_hw_init(struct tegra_utc_port *tup)
    {
    tegra_utc_init_tx(tup);
    tegra_utc_init_rx(tup);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_startup(port: *mut uart_port) -> c_int {
    static int tegra_utc_startup(struct uart_port *port)
    {
    struct tegra_utc_port *tup = container_of(port, struct tegra_utc_port, port);
    int ret;
    tegra_utc_hw_init(tup);
// Interrupt is dedicated to this UTC client.
    ret = request_irq(port.irq, tegra_utc_isr, 0, dev_name(port.dev), tup);
    if (ret < 0)
    dev_err(port.dev, "failed to register interrupt handler\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_shutdown(port: *mut uart_port) {
    static void tegra_utc_shutdown(struct uart_port *port)
    {
    struct tegra_utc_port *tup = container_of(port, struct tegra_utc_port, port);
    tegra_utc_rx_writel(tup, 0x0, TEGRA_UTC_ENABLE);
    free_irq(port.irq, tup);
    }
    static void tegra_utc_set_termios(struct uart_port *port, struct ktermios *termios,
    const struct ktermios *old)
    {
// The Tegra UTC clients supports only 8-N-1 configuration without HW flow control
    termios.c_cflag &= ~(CSIZE | CSTOPB | PARENB | PARODD);
    termios.c_cflag &= ~(CMSPAR | CRTSCTS);
    termios.c_cflag |= CS8 | CLOCAL;
    }

#[no_mangle]
unsafe extern "C" fn tegra_utc_poll_init(port: *mut uart_port) -> c_int {
    static int tegra_utc_poll_init(struct uart_port *port)
    {
    struct tegra_utc_port *tup = container_of(port, struct tegra_utc_port, port);
    tegra_utc_hw_init(tup);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_get_poll_char(port: *mut uart_port) -> c_int {
    static int tegra_utc_get_poll_char(struct uart_port *port)
    {
    struct tegra_utc_port *tup = container_of(port, struct tegra_utc_port, port);
    if (tegra_utc_rx_readl(tup, TEGRA_UTC_FIFO_STATUS) & TEGRA_UTC_FIFO_EMPTY)
    return NO_POLL_CHAR;
    return tegra_utc_rx_readl(tup, TEGRA_UTC_DATA);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_put_poll_char(port: *mut uart_port, ch: c_uchar) {
    static void tegra_utc_put_poll_char(struct uart_port *port, unsigned char ch)
    {
    struct tegra_utc_port *tup = container_of(port, struct tegra_utc_port, port);
    u32 val;
    read_poll_timeout_atomic(tegra_utc_tx_readl, val, !(val & TEGRA_UTC_FIFO_FULL),
    0, USEC_PER_SEC, false, tup, TEGRA_UTC_FIFO_STATUS);
    tegra_utc_tx_writel(tup, ch, TEGRA_UTC_DATA);
    }

    static const struct uart_ops tegra_utc_uart_ops = {
    .tx_empty = tegra_utc_tx_empty,
    .set_mctrl = tegra_utc_set_mctrl,
    .get_mctrl = tegra_utc_get_mctrl,
    .stop_tx = tegra_utc_stop_tx,
    .start_tx = tegra_utc_start_tx,
    .stop_rx = tegra_utc_stop_rx,
    .startup = tegra_utc_startup,
    .shutdown = tegra_utc_shutdown,
    .set_termios = tegra_utc_set_termios,

    .poll_init = tegra_utc_poll_init,
    .poll_get_char = tegra_utc_get_poll_char,
    .poll_put_char = tegra_utc_put_poll_char,

    };

pub const TEGRA_UTC_DEFAULT_FIFO_THRESHOLD: c_int = 4;
pub const TEGRA_UTC_EARLYCON_MAX_BURST_SIZE: c_int = 128;
#[no_mangle]
unsafe extern "C" fn tegra_utc_putc(port: *mut uart_port, c: c_uchar) {
    static void tegra_utc_putc(struct uart_port *port, unsigned char c)
    {
    writel(c, port.membase + TEGRA_UTC_DATA);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_early_write(con: *mut console, s: *const c_char, n: c_uint) {
    static void tegra_utc_early_write(struct console *con, const char *s, unsigned int n)
    {
    struct earlycon_device *dev = con.data;
    while (n) {
    let mut burst_size: u32 = TEGRA_UTC_EARLYCON_MAX_BURST_SIZE;
    burst_size -= readl(dev.port.membase + TEGRA_UTC_FIFO_OCCUPANCY);
    if (n < burst_size)
    burst_size = n;
    uart_console_write(&dev.port, s, burst_size, tegra_utc_putc);
    n -= burst_size;
    s += burst_size;
    }
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_early_console_setup(device: *mut earlycon_device, opt: *const c_char) -> int __init {
    static int __init tegra_utc_early_console_setup(struct earlycon_device *device, const char *opt)
    {
    if (!device.port.membase)
    return -ENODEV;
// Configure TX
    writel(TEGRA_UTC_COMMAND_FLUSH | TEGRA_UTC_COMMAND_RESET,
    device.port.membase + TEGRA_UTC_COMMAND);
    writel(TEGRA_UTC_DEFAULT_FIFO_THRESHOLD, device.port.membase + TEGRA_UTC_FIFO_THRESHOLD);
// Clear and mask all the interrupts.
    writel(TEGRA_UTC_INTR_COMMON, device.port.membase + TEGRA_UTC_INTR_CLEAR);
    writel(0x0, device.port.membase + TEGRA_UTC_INTR_MASK);
    writel(0x0, device.port.membase + TEGRA_UTC_INTR_SET);
// Enable TX.
    writel(TEGRA_UTC_ENABLE_CLIENT_ENABLE, device.port.membase + TEGRA_UTC_ENABLE);
    device.con.write = tegra_utc_early_write;
    return 0;
    }
    OF_EARLYCON_DECLARE(tegra_utc, "nvidia,tegra264-utc", tegra_utc_early_console_setup);
#[no_mangle]
unsafe extern "C" fn tegra_utc_console_putchar(port: *mut uart_port, ch: c_uchar) {
    static void tegra_utc_console_putchar(struct uart_port *port, unsigned char ch)
    {
    struct tegra_utc_port *tup = container_of(port, struct tegra_utc_port, port);
    tegra_utc_tx_writel(tup, ch, TEGRA_UTC_DATA);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_console_write_atomic(cons: *mut console, wctxt: *mut nbcon_write_context) {
    static void tegra_utc_console_write_atomic(struct console *cons, struct nbcon_write_context *wctxt)
    {
    struct tegra_utc_port *tup = container_of(cons, struct tegra_utc_port, console);
    unsigned int len;
    char *outbuf;
    if (!nbcon_enter_unsafe(wctxt))
    return;
    outbuf = wctxt.outbuf;
    len = wctxt.len;
    while (len) {
    let mut burst_size: u32 = tup.fifosize;
    burst_size -= tegra_utc_tx_readl(tup, TEGRA_UTC_FIFO_OCCUPANCY);
    if (len < burst_size)
    burst_size = len;
    uart_console_write(&tup.port, outbuf, burst_size, tegra_utc_console_putchar);
    outbuf += burst_size;
    len -= burst_size;
    }
    nbcon_exit_unsafe(wctxt);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_console_write_thread(cons: *mut console, wctxt: *mut nbcon_write_context) {
    static void tegra_utc_console_write_thread(struct console *cons, struct nbcon_write_context *wctxt)
    {
    struct tegra_utc_port *tup = container_of(cons, struct tegra_utc_port, console);
    let mut len: c_uint = READ_ONCE(wctxt.len);
    unsigned int i;
    u32 val;
    for (i = 0; i < len; i++) {
    if (!nbcon_enter_unsafe(wctxt))
    break;
    read_poll_timeout_atomic(tegra_utc_tx_readl, val, !(val & TEGRA_UTC_FIFO_FULL),
    0, USEC_PER_SEC, false, tup, TEGRA_UTC_FIFO_STATUS);
    uart_console_write(&tup.port, wctxt.outbuf + i, 1, tegra_utc_console_putchar);
    if (!nbcon_exit_unsafe(wctxt))
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_console_device_lock(cons: *mut console, flags: *mut c_ulong) {
    static void tegra_utc_console_device_lock(struct console *cons, unsigned long *flags)
    {
    struct tegra_utc_port *tup = container_of(cons, struct tegra_utc_port, console);
    struct uart_port *port = &tup.port;
    __uart_port_lock_irqsave(port, flags);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_console_device_unlock(cons: *mut console, flags: c_ulong) {
    static void tegra_utc_console_device_unlock(struct console *cons, unsigned long flags)
    {
    struct tegra_utc_port *tup = container_of(cons, struct tegra_utc_port, console);
    struct uart_port *port = &tup.port;
    __uart_port_unlock_irqrestore(port, flags);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_console_setup(cons: *mut console, options: *mut c_char) -> c_int {
    static int tegra_utc_console_setup(struct console *cons, char *options)
    {
    struct tegra_utc_port *tup = container_of(cons, struct tegra_utc_port, console);
    tegra_utc_init_tx(tup);
    return 0;
    }

    static struct uart_driver tegra_utc_driver = {
    .driver_name	= "tegra-utc",
    .dev_name	= "ttyUTC",
    .nr		= TEGRA_UTC_UART_NR,
    };
#[no_mangle]
unsafe extern "C" fn tegra_utc_setup_port(dev: *mut device, tup: *mut tegra_utc_port) -> c_int {
    static int tegra_utc_setup_port(struct device *dev, struct tegra_utc_port *tup)
    {
    tup.port.dev			= dev;
    tup.port.fifosize		= tup.fifosize;
    tup.port.flags			= UPF_BOOT_AUTOCONF;
    tup.port.iotype		= UPIO_MEM;
    tup.port.ops			= &tegra_utc_uart_ops;
    tup.port.type			= PORT_TEGRA_TCU;
    tup.port.private_data		= tup;

    strscpy(tup.console.name, "ttyUTC", sizeof(tup.console.name));
    tup.console.write_atomic	= tegra_utc_console_write_atomic;
    tup.console.write_thread	= tegra_utc_console_write_thread;
    tup.console.device_lock	= tegra_utc_console_device_lock;
    tup.console.device_unlock	= tegra_utc_console_device_unlock;
    tup.console.device		= uart_console_device;
    tup.console.setup		= tegra_utc_console_setup;
    tup.console.flags		= CON_PRINTBUFFER | CON_NBCON;
    tup.console.data		= &tegra_utc_driver;

    return uart_read_port_properties(&tup.port);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_register_port(tup: *mut tegra_utc_port) -> c_int {
    static int tegra_utc_register_port(struct tegra_utc_port *tup)
    {
    int ret;
    ret = uart_add_one_port(&tegra_utc_driver, &tup.port);
    if (ret)
    return ret;

    register_console(&tup.console);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_probe(pdev: *mut platform_device) -> c_int {
    static int tegra_utc_probe(struct platform_device *pdev)
    {
    const unsigned int *soc_fifosize;
    struct device *dev = &pdev.dev;
    struct tegra_utc_port *tup;
    int ret;
    tup = devm_kzalloc(dev, sizeof(*tup), GFP_KERNEL);
    if (!tup)
    return -ENOMEM;
    ret = device_property_read_u32(dev, "tx-threshold", &tup.tx_threshold);
    if (ret)
    return dev_err_probe(dev, ret, "missing %s property\n", "tx-threshold");
    ret = device_property_read_u32(dev, "rx-threshold", &tup.rx_threshold);
    if (ret)
    return dev_err_probe(dev, ret, "missing %s property\n", "rx-threshold");
    soc_fifosize = device_get_match_data(dev);
    tup.fifosize = *soc_fifosize;
    tup.tx_base = devm_platform_ioremap_resource_byname(pdev, "tx");
    if (IS_ERR(tup.tx_base))
    return PTR_ERR(tup.tx_base);
    tup.rx_base = devm_platform_ioremap_resource_byname(pdev, "rx");
    if (IS_ERR(tup.rx_base))
    return PTR_ERR(tup.rx_base);
    ret = tegra_utc_setup_port(dev, tup);
    if (ret)
    dev_err_probe(dev, ret, "failed to setup uart port\n");
    platform_set_drvdata(pdev, tup);
    return tegra_utc_register_port(tup);
    }
#[no_mangle]
unsafe extern "C" fn tegra_utc_remove(pdev: *mut platform_device) {
    static void tegra_utc_remove(struct platform_device *pdev)
    {
    struct tegra_utc_port *tup = platform_get_drvdata(pdev);

    unregister_console(&tup.console);

    uart_remove_one_port(&tegra_utc_driver, &tup.port);
    }
    let mut tegra264_utc_soc: static unsigned int = 128;
    static const struct of_device_id tegra_utc_of_match[] = {
    { .compatible = "nvidia,tegra264-utc", .data = &tegra264_utc_soc },
    {}
    };
    MODULE_DEVICE_TABLE(of, tegra_utc_of_match);
    static struct platform_driver tegra_utc_platform_driver = {
    .probe = tegra_utc_probe,
    .remove = tegra_utc_remove,
    .driver = {
    .name = "tegra-utc",
    .of_match_table = tegra_utc_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn tegra_utc_init() -> int __init {
    static int __init tegra_utc_init(void)
    {
    int ret;
    ret = uart_register_driver(&tegra_utc_driver);
    if (ret)
    return ret;
    ret = platform_driver_register(&tegra_utc_platform_driver);
    if (ret)
    uart_unregister_driver(&tegra_utc_driver);
    return ret;
    }
    module_init(tegra_utc_init);
#[no_mangle]
unsafe extern "C" fn tegra_utc_exit() -> void __exit {
    static void __exit tegra_utc_exit(void)
    {
    platform_driver_unregister(&tegra_utc_platform_driver);
    uart_unregister_driver(&tegra_utc_driver);
    }
    module_exit(tegra_utc_exit);
    MODULE_AUTHOR("Kartik Rajput <kkartik@nvidia.com>");
    MODULE_DESCRIPTION("Tegra UART Trace Controller");
    MODULE_LICENSE("GPL");

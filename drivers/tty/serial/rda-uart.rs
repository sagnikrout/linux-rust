//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/rda-uart.c
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
// RDA8810PL serial device driver
//
// Copyright RDA Microelectronics Company Limited
// Copyright (c) 2017 Andreas Färber
// Copyright (c) 2018 Manivannan Sadhasivam
//

pub const RDA_UART_PORT_NUM: c_int = 3;

pub const RDA_UART_CTRL: c_uint = 0x00;
pub const RDA_UART_STATUS: c_uint = 0x04;
pub const RDA_UART_RXTX_BUFFER: c_uint = 0x08;
pub const RDA_UART_IRQ_MASK: c_uint = 0x0c;
pub const RDA_UART_IRQ_CAUSE: c_uint = 0x10;
pub const RDA_UART_IRQ_TRIGGERS: c_uint = 0x14;
pub const RDA_UART_CMD_SET: c_uint = 0x18;
pub const RDA_UART_CMD_CLR: c_uint = 0x1c;
// UART_CTRL Bits

// UART_STATUS Bits

// UART_RXTX_BUFFER Bits

// UART_IRQ_MASK Bits

// UART_IRQ_CAUSE Bits

// UART_TRIGGERS Bits

// UART_CMD_SET Bits

pub const RDA_UART_TX_FIFO_SIZE: c_int = 16;
    static struct uart_driver rda_uart_driver;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rda_uart_port {
    pub port: uart_port,
    pub clk: *mut clk,
}

    static struct rda_uart_port *rda_uart_ports[RDA_UART_PORT_NUM];
    static inline void rda_uart_write(struct uart_port *port, u32 val,
    unsigned int off)
    {
    writel(val, port.membase + off);
    }
#[no_mangle]
pub unsafe extern "C" fn rda_uart_read(port: *mut uart_port, off: c_uint) -> u32 {
    static inline u32 rda_uart_read(struct uart_port *port, unsigned int off)
    {
    return readl(port.membase + off);
    }
#[no_mangle]
unsafe extern "C" fn rda_uart_tx_empty(port: *mut uart_port) -> c_uint {
    static unsigned int rda_uart_tx_empty(struct uart_port *port)
    {
    unsigned long flags;
    unsigned int ret;
    u32 val;
    uart_port_lock_irqsave(port, &flags);
    val = rda_uart_read(port, RDA_UART_STATUS);
    ret = (val & RDA_UART_TX_FIFO_MASK) ? TIOCSER_TEMT : 0;
    uart_port_unlock_irqrestore(port, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rda_uart_get_mctrl(port: *mut uart_port) -> c_uint {
    static unsigned int rda_uart_get_mctrl(struct uart_port *port)
    {
    let mut mctrl: c_uint = 0;
    u32 cmd_set, status;
    cmd_set = rda_uart_read(port, RDA_UART_CMD_SET);
    status = rda_uart_read(port, RDA_UART_STATUS);
    if (cmd_set & RDA_UART_RTS)
    mctrl |= TIOCM_RTS;
    if (!(status & RDA_UART_CTS))
    mctrl |= TIOCM_CTS;
    return mctrl;
    }
#[no_mangle]
unsafe extern "C" fn rda_uart_set_mctrl(port: *mut uart_port, mctrl: c_uint) {
    static void rda_uart_set_mctrl(struct uart_port *port, unsigned int mctrl)
    {
    u32 val;
    if (mctrl & TIOCM_RTS) {
    val = rda_uart_read(port, RDA_UART_CMD_SET);
    rda_uart_write(port, (val | RDA_UART_RTS), RDA_UART_CMD_SET);
    } else {
// Clear RTS to stop to receive.
    val = rda_uart_read(port, RDA_UART_CMD_CLR);
    rda_uart_write(port, (val | RDA_UART_RTS), RDA_UART_CMD_CLR);
    }
    val = rda_uart_read(port, RDA_UART_CTRL);
    if (mctrl & TIOCM_LOOP)
    val |= RDA_UART_LOOP_BACK_EN;
    else
    val &= ~RDA_UART_LOOP_BACK_EN;
    rda_uart_write(port, val, RDA_UART_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn rda_uart_stop_tx(port: *mut uart_port) {
    static void rda_uart_stop_tx(struct uart_port *port)
    {
    u32 val;
    val = rda_uart_read(port, RDA_UART_IRQ_MASK);
    val &= ~RDA_UART_TX_DATA_NEEDED;
    rda_uart_write(port, val, RDA_UART_IRQ_MASK);
    val = rda_uart_read(port, RDA_UART_CMD_SET);
    val |= RDA_UART_TX_FIFO_RESET;
    rda_uart_write(port, val, RDA_UART_CMD_SET);
    }
#[no_mangle]
unsafe extern "C" fn rda_uart_stop_rx(port: *mut uart_port) {
    static void rda_uart_stop_rx(struct uart_port *port)
    {
    u32 val;
    val = rda_uart_read(port, RDA_UART_IRQ_MASK);
    val &= ~(RDA_UART_RX_DATA_AVAILABLE | RDA_UART_RX_TIMEOUT);
    rda_uart_write(port, val, RDA_UART_IRQ_MASK);
// Read Rx buffer before reset to avoid Rx timeout interrupt
    val = rda_uart_read(port, RDA_UART_RXTX_BUFFER);
    val = rda_uart_read(port, RDA_UART_CMD_SET);
    val |= RDA_UART_RX_FIFO_RESET;
    rda_uart_write(port, val, RDA_UART_CMD_SET);
    }
#[no_mangle]
unsafe extern "C" fn rda_uart_start_tx(port: *mut uart_port) {
    static void rda_uart_start_tx(struct uart_port *port)
    {
    u32 val;
    if (uart_tx_stopped(port)) {
    rda_uart_stop_tx(port);
    return;
    }
    val = rda_uart_read(port, RDA_UART_IRQ_MASK);
    val |= RDA_UART_TX_DATA_NEEDED;
    rda_uart_write(port, val, RDA_UART_IRQ_MASK);
    }
    static void rda_uart_change_baudrate(struct rda_uart_port *rda_port,
    unsigned long baud)
    {
    clk_set_rate(rda_port.clk, baud * 8);
    }
    static void rda_uart_set_termios(struct uart_port *port,
    struct ktermios *termios,
    const struct ktermios *old)
    {
    struct rda_uart_port *rda_port = to_rda_uart_port(port);
    unsigned long flags;
    unsigned int ctrl, cmd_set, cmd_clr, triggers;
    unsigned int baud;
    u32 irq_mask;
    uart_port_lock_irqsave(port, &flags);
    baud = uart_get_baud_rate(port, termios, old, 9600, port.uartclk / 4);
    rda_uart_change_baudrate(rda_port, baud);
    ctrl = rda_uart_read(port, RDA_UART_CTRL);
    cmd_set = rda_uart_read(port, RDA_UART_CMD_SET);
    cmd_clr = rda_uart_read(port, RDA_UART_CMD_CLR);
    switch (termios.c_cflag & CSIZE) {
    case CS5:
    case CS6:
    dev_warn(port.dev, "bit size not supported, using 7 bits\n");
    fallthrough;
    case CS7:
    ctrl &= ~RDA_UART_DBITS_8;
    termios.c_cflag &= ~CSIZE;
    termios.c_cflag |= CS7;
    break;
    default:
    ctrl |= RDA_UART_DBITS_8;
    break;
    }
// stop bits
    if (termios.c_cflag & CSTOPB)
    ctrl |= RDA_UART_TX_SBITS_2;
    else
    ctrl &= ~RDA_UART_TX_SBITS_2;
// parity check
    if (termios.c_cflag & PARENB) {
    ctrl |= RDA_UART_PARITY_EN;
// Mark or Space parity
    if (termios.c_cflag & CMSPAR) {
    if (termios.c_cflag & PARODD)
    ctrl |= RDA_UART_PARITY_MARK;
    else
    ctrl |= RDA_UART_PARITY_SPACE;
    } else if (termios.c_cflag & PARODD) {
    ctrl |= RDA_UART_PARITY_ODD;
    } else {
    ctrl |= RDA_UART_PARITY_EVEN;
    }
    } else {
    ctrl &= ~RDA_UART_PARITY_EN;
    }
// Hardware handshake (RTS/CTS)
    if (termios.c_cflag & CRTSCTS) {
    ctrl   |= RDA_UART_FLOW_CNT_EN;
    cmd_set |= RDA_UART_RTS;
    } else {
    ctrl   &= ~RDA_UART_FLOW_CNT_EN;
    cmd_clr |= RDA_UART_RTS;
    }
    ctrl |= RDA_UART_ENABLE;
    ctrl &= ~RDA_UART_DMA_EN;
    triggers  = (RDA_UART_AFC_LEVEL(20) | RDA_UART_RX_TRIGGER(16));
    irq_mask = rda_uart_read(port, RDA_UART_IRQ_MASK);
    rda_uart_write(port, 0, RDA_UART_IRQ_MASK);
    rda_uart_write(port, triggers, RDA_UART_IRQ_TRIGGERS);
    rda_uart_write(port, ctrl, RDA_UART_CTRL);
    rda_uart_write(port, cmd_set, RDA_UART_CMD_SET);
    rda_uart_write(port, cmd_clr, RDA_UART_CMD_CLR);
    rda_uart_write(port, irq_mask, RDA_UART_IRQ_MASK);
// Don't rewrite B0
    if (tty_termios_baud_rate(termios))
    tty_termios_encode_baud_rate(termios, baud, baud);
// update the per-port timeout
    uart_update_timeout(port, termios.c_cflag, baud);
    uart_port_unlock_irqrestore(port, flags);
    }
#[no_mangle]
unsafe extern "C" fn rda_uart_send_chars(port: *mut uart_port) {
    static void rda_uart_send_chars(struct uart_port *port)
    {
    struct tty_port *tport = &port.state.port;
    unsigned char ch;
    u32 val;
    if (uart_tx_stopped(port))
    return;
    if (port.x_char) {
    while (!(rda_uart_read(port, RDA_UART_STATUS) &
    RDA_UART_TX_FIFO_MASK))
    cpu_relax();
    rda_uart_write(port, port.x_char, RDA_UART_RXTX_BUFFER);
    port.icount.tx++;
    port.x_char = 0;
    }
    while ((rda_uart_read(port, RDA_UART_STATUS) & RDA_UART_TX_FIFO_MASK) &&
    uart_fifo_get(port, &ch))
    rda_uart_write(port, ch, RDA_UART_RXTX_BUFFER);
    if (kfifo_len(&tport.xmit_fifo) < WAKEUP_CHARS)
    uart_write_wakeup(port);
    if (!kfifo_is_empty(&tport.xmit_fifo)) {
// Re-enable Tx FIFO interrupt
    val = rda_uart_read(port, RDA_UART_IRQ_MASK);
    val |= RDA_UART_TX_DATA_NEEDED;
    rda_uart_write(port, val, RDA_UART_IRQ_MASK);
    }
    }
#[no_mangle]
unsafe extern "C" fn rda_uart_receive_chars(port: *mut uart_port) {
    static void rda_uart_receive_chars(struct uart_port *port)
    {
    u32 status, val;
    status = rda_uart_read(port, RDA_UART_STATUS);
    while ((status & RDA_UART_RX_FIFO_MASK)) {
    let mut flag: c_char = TTY_NORMAL;
    if (status & RDA_UART_RX_PARITY_ERR) {
    port.icount.parity++;
    flag = TTY_PARITY;
    }
    if (status & RDA_UART_RX_FRAMING_ERR) {
    port.icount.frame++;
    flag = TTY_FRAME;
    }
    if (status & RDA_UART_RX_OVERFLOW_ERR) {
    port.icount.overrun++;
    flag = TTY_OVERRUN;
    }
    val = rda_uart_read(port, RDA_UART_RXTX_BUFFER);
    val &= 0xff;
    port.icount.rx++;
    if (!uart_prepare_sysrq_char(port, val))
    tty_insert_flip_char(&port.state.port, val, flag);
    status = rda_uart_read(port, RDA_UART_STATUS);
    }
    tty_flip_buffer_push(&port.state.port);
    }
#[no_mangle]
unsafe extern "C" fn rda_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rda_interrupt(int irq, void *dev_id)
    {
    struct uart_port *port = dev_id;
    u32 val, irq_mask;
    uart_port_lock(port);
// Clear IRQ cause
    val = rda_uart_read(port, RDA_UART_IRQ_CAUSE);
    rda_uart_write(port, val, RDA_UART_IRQ_CAUSE);
    if (val & (RDA_UART_RX_DATA_AVAILABLE | RDA_UART_RX_TIMEOUT))
    rda_uart_receive_chars(port);
    if (val & (RDA_UART_TX_DATA_NEEDED)) {
    irq_mask = rda_uart_read(port, RDA_UART_IRQ_MASK);
    irq_mask &= ~RDA_UART_TX_DATA_NEEDED;
    rda_uart_write(port, irq_mask, RDA_UART_IRQ_MASK);
    rda_uart_send_chars(port);
    }
    uart_unlock_and_check_sysrq(port);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rda_uart_startup(port: *mut uart_port) -> c_int {
    static int rda_uart_startup(struct uart_port *port)
    {
    unsigned long flags;
    int ret;
    u32 val;
    uart_port_lock_irqsave(port, &flags);
    rda_uart_write(port, 0, RDA_UART_IRQ_MASK);
    uart_port_unlock_irqrestore(port, flags);
    ret = request_irq(port.irq, rda_interrupt, IRQF_NO_SUSPEND,
    "rda-uart", port);
    if (ret)
    return ret;
    uart_port_lock_irqsave(port, &flags);
    val = rda_uart_read(port, RDA_UART_CTRL);
    val |= RDA_UART_ENABLE;
    rda_uart_write(port, val, RDA_UART_CTRL);
// enable rx interrupt
    val = rda_uart_read(port, RDA_UART_IRQ_MASK);
    val |= (RDA_UART_RX_DATA_AVAILABLE | RDA_UART_RX_TIMEOUT);
    rda_uart_write(port, val, RDA_UART_IRQ_MASK);
    uart_port_unlock_irqrestore(port, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rda_uart_shutdown(port: *mut uart_port) {
    static void rda_uart_shutdown(struct uart_port *port)
    {
    unsigned long flags;
    u32 val;
    uart_port_lock_irqsave(port, &flags);
    rda_uart_stop_tx(port);
    rda_uart_stop_rx(port);
    val = rda_uart_read(port, RDA_UART_CTRL);
    val &= ~RDA_UART_ENABLE;
    rda_uart_write(port, val, RDA_UART_CTRL);
    uart_port_unlock_irqrestore(port, flags);
    }
    static const char *rda_uart_type(struct uart_port *port)
    {
    return (port.type == PORT_RDA) ? "rda-uart" : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn rda_uart_request_port(port: *mut uart_port) -> c_int {
    static int rda_uart_request_port(struct uart_port *port)
    {
    struct platform_device *pdev = to_platform_device(port.dev);
    struct resource *res;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENXIO;
    if (!devm_request_mem_region(port.dev, port.mapbase,
    resource_size(res), dev_name(port.dev)))
    return -EBUSY;
    if (port.flags & UPF_IOREMAP) {
    port.membase = devm_ioremap(port.dev, port.mapbase,
    resource_size(res));
    if (!port.membase)
    return -EBUSY;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rda_uart_config_port(port: *mut uart_port, flags: c_int) {
    static void rda_uart_config_port(struct uart_port *port, int flags)
    {
    unsigned long irq_flags;
    if (flags & UART_CONFIG_TYPE) {
    port.type = PORT_RDA;
    rda_uart_request_port(port);
    }
    uart_port_lock_irqsave(port, &irq_flags);
// Clear mask, so no surprise interrupts.
    rda_uart_write(port, 0, RDA_UART_IRQ_MASK);
// Clear status register
    rda_uart_write(port, 0, RDA_UART_STATUS);
    uart_port_unlock_irqrestore(port, irq_flags);
    }
#[no_mangle]
unsafe extern "C" fn rda_uart_release_port(port: *mut uart_port) {
    static void rda_uart_release_port(struct uart_port *port)
    {
    struct platform_device *pdev = to_platform_device(port.dev);
    struct resource *res;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return;
    if (port.flags & UPF_IOREMAP) {
    devm_release_mem_region(port.dev, port.mapbase,
    resource_size(res));
    devm_iounmap(port.dev, port.membase);
    port.membase = core::ptr::null_mut();
    }
    }
    static int rda_uart_verify_port(struct uart_port *port,
    struct serial_struct *ser)
    {
    if (port.type != PORT_RDA)
    return -EINVAL;
    if (port.irq != ser.irq)
    return -EINVAL;
    return 0;
    }
    static const struct uart_ops rda_uart_ops = {
    .tx_empty       = rda_uart_tx_empty,
    .get_mctrl      = rda_uart_get_mctrl,
    .set_mctrl      = rda_uart_set_mctrl,
    .start_tx       = rda_uart_start_tx,
    .stop_tx        = rda_uart_stop_tx,
    .stop_rx        = rda_uart_stop_rx,
    .startup        = rda_uart_startup,
    .shutdown       = rda_uart_shutdown,
    .set_termios    = rda_uart_set_termios,
    .type           = rda_uart_type,
    .request_port	= rda_uart_request_port,
    .release_port	= rda_uart_release_port,
    .config_port	= rda_uart_config_port,
    .verify_port	= rda_uart_verify_port,
    };

#[no_mangle]
unsafe extern "C" fn rda_console_putchar(port: *mut uart_port, ch: c_uchar) {
    static void rda_console_putchar(struct uart_port *port, unsigned char ch)
    {
    if (!port.membase)
    return;
    while (!(rda_uart_read(port, RDA_UART_STATUS) & RDA_UART_TX_FIFO_MASK))
    cpu_relax();
    rda_uart_write(port, ch, RDA_UART_RXTX_BUFFER);
    }
    static void rda_uart_port_write(struct uart_port *port, const char *s,
    u_int count)
    {
    u32 old_irq_mask;
    unsigned long flags;
    let mut locked: c_int = 1;
    if (oops_in_progress)
    locked = uart_port_trylock_irqsave(port, &flags);
    else
    uart_port_lock_irqsave(port, &flags);
    old_irq_mask = rda_uart_read(port, RDA_UART_IRQ_MASK);
    rda_uart_write(port, 0, RDA_UART_IRQ_MASK);
    uart_console_write(port, s, count, rda_console_putchar);
// wait until all contents have been sent out
    while (!(rda_uart_read(port, RDA_UART_STATUS) & RDA_UART_TX_FIFO_MASK))
    cpu_relax();
    rda_uart_write(port, old_irq_mask, RDA_UART_IRQ_MASK);
    if (locked)
    uart_port_unlock_irqrestore(port, flags);
    }
    static void rda_uart_console_write(struct console *co, const char *s,
    u_int count)
    {
    struct rda_uart_port *rda_port;
    rda_port = rda_uart_ports[co.index];
    if (!rda_port)
    return;
    rda_uart_port_write(&rda_port.port, s, count);
    }
#[no_mangle]
unsafe extern "C" fn rda_uart_console_setup(co: *mut console, options: *mut c_char) -> c_int {
    static int rda_uart_console_setup(struct console *co, char *options)
    {
    struct rda_uart_port *rda_port;
    let mut baud: c_int = 921600;
    let mut bits: c_int = 8;
    let mut parity: c_int = 'n';
    let mut flow: c_int = 'n';
    if (co.index < 0 || co.index >= RDA_UART_PORT_NUM)
    return -EINVAL;
    rda_port = rda_uart_ports[co.index];
    if (!rda_port || !rda_port.port.membase)
    return -ENODEV;
    if (options)
    uart_parse_options(options, &baud, &parity, &bits, &flow);
    return uart_set_options(&rda_port.port, co, baud, parity, bits, flow);
    }
    static struct console rda_uart_console = {
    .name = RDA_UART_DEV_NAME,
    .write = rda_uart_console_write,
    .device = uart_console_device,
    .setup = rda_uart_console_setup,
    .flags = CON_PRINTBUFFER,
    .index = -1,
    .data = &rda_uart_driver,
    };
#[no_mangle]
unsafe extern "C" fn rda_uart_console_init() -> int __init {
    static int __init rda_uart_console_init(void)
    {
    register_console(&rda_uart_console);
    return 0;
    }
    console_initcall(rda_uart_console_init);
    static void rda_uart_early_console_write(struct console *co,
    const char *s,
    u_int count)
    {
    struct earlycon_device *dev = co.data;
    rda_uart_port_write(&dev.port, s, count);
    }
    static int __init
    rda_uart_early_console_setup(struct earlycon_device *device, const char *opt)
    {
    if (!device.port.membase)
    return -ENODEV;
    device.con.write = rda_uart_early_console_write;
    return 0;
    }
    OF_EARLYCON_DECLARE(rda, "rda,8810pl-uart",
    rda_uart_early_console_setup);

    static struct uart_driver rda_uart_driver = {
    .owner = THIS_MODULE,
    .driver_name = "rda-uart",
    .dev_name = RDA_UART_DEV_NAME,
    .nr = RDA_UART_PORT_NUM,
    .cons = RDA_UART_CONSOLE,
    };
    static const struct of_device_id rda_uart_dt_matches[] = {
    { .compatible = "rda,8810pl-uart" },
    { }
    };
    MODULE_DEVICE_TABLE(of, rda_uart_dt_matches);
#[no_mangle]
unsafe extern "C" fn rda_uart_probe(pdev: *mut platform_device) -> c_int {
    static int rda_uart_probe(struct platform_device *pdev)
    {
    struct resource *res_mem;
    struct rda_uart_port *rda_port;
    int ret, irq;
    if (pdev.dev.of_node)
    pdev.id = of_alias_get_id(pdev.dev.of_node, "serial");
    if (pdev.id < 0 || pdev.id >= RDA_UART_PORT_NUM) {
    dev_err(&pdev.dev, "id %d out of range\n", pdev.id);
    return -EINVAL;
    }
    res_mem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res_mem) {
    dev_err(&pdev.dev, "could not get mem\n");
    return -ENODEV;
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    if (rda_uart_ports[pdev.id]) {
    dev_err(&pdev.dev, "port %d already allocated\n", pdev.id);
    return -EBUSY;
    }
    rda_port = devm_kzalloc(&pdev.dev, sizeof(*rda_port), GFP_KERNEL);
    if (!rda_port)
    return -ENOMEM;
    rda_port.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(rda_port.clk)) {
    dev_err(&pdev.dev, "could not get clk\n");
    return PTR_ERR(rda_port.clk);
    }
    rda_port.port.dev = &pdev.dev;
    rda_port.port.regshift = 0;
    rda_port.port.line = pdev.id;
    rda_port.port.type = PORT_RDA;
    rda_port.port.iotype = UPIO_MEM;
    rda_port.port.mapbase = res_mem.start;
    rda_port.port.irq = irq;
    rda_port.port.uartclk = clk_get_rate(rda_port.clk);
    if (rda_port.port.uartclk == 0) {
    dev_err(&pdev.dev, "clock rate is zero\n");
    return -EINVAL;
    }
    rda_port.port.flags = UPF_BOOT_AUTOCONF | UPF_IOREMAP |
    UPF_LOW_LATENCY;
    rda_port.port.x_char = 0;
    rda_port.port.fifosize = RDA_UART_TX_FIFO_SIZE;
    rda_port.port.ops = &rda_uart_ops;
    rda_uart_ports[pdev.id] = rda_port;
    platform_set_drvdata(pdev, rda_port);
    ret = uart_add_one_port(&rda_uart_driver, &rda_port.port);
    if (ret)
    rda_uart_ports[pdev.id] = core::ptr::null_mut();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rda_uart_remove(pdev: *mut platform_device) {
    static void rda_uart_remove(struct platform_device *pdev)
    {
    struct rda_uart_port *rda_port = platform_get_drvdata(pdev);
    uart_remove_one_port(&rda_uart_driver, &rda_port.port);
    rda_uart_ports[pdev.id] = core::ptr::null_mut();
    }
    static struct platform_driver rda_uart_platform_driver = {
    .probe = rda_uart_probe,
    .remove = rda_uart_remove,
    .driver = {
    .name = "rda-uart",
    .of_match_table = rda_uart_dt_matches,
    },
    };
#[no_mangle]
unsafe extern "C" fn rda_uart_init() -> int __init {
    static int __init rda_uart_init(void)
    {
    int ret;
    ret = uart_register_driver(&rda_uart_driver);
    if (ret)
    return ret;
    ret = platform_driver_register(&rda_uart_platform_driver);
    if (ret)
    uart_unregister_driver(&rda_uart_driver);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rda_uart_exit() -> void __exit {
    static void __exit rda_uart_exit(void)
    {
    platform_driver_unregister(&rda_uart_platform_driver);
    uart_unregister_driver(&rda_uart_driver);
    }
    module_init(rda_uart_init);
    module_exit(rda_uart_exit);
    MODULE_AUTHOR("Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>");
    MODULE_DESCRIPTION("RDA8810PL serial device driver");
    MODULE_LICENSE("GPL");

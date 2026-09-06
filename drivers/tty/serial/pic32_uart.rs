//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/pic32_uart.c
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
// PIC32 Integrated Serial Driver.
//
// Copyright (C) 2015 Microchip Technology, Inc.
//
// Authors:
// Sorin-Andrei Pistirica <andrei.pistirica@microchip.com>
//

// UART name and device definitions

pub const PIC32_MAX_UARTS: c_int = 6;

pub const PIC32_UART_DFLT_BRATE: c_int = 9600;
pub const PIC32_UART_TX_FIFO_DEPTH: c_int = 8;
pub const PIC32_UART_RX_FIFO_DEPTH: c_int = 8;
pub const PIC32_UART_MODE: c_uint = 0x00;
pub const PIC32_UART_STA: c_uint = 0x10;
pub const PIC32_UART_TX: c_uint = 0x20;
pub const PIC32_UART_RX: c_uint = 0x30;
pub const PIC32_UART_BRG: c_uint = 0x40;
// struct pic32_sport - pic32 serial port descriptor
// @port: uart port descriptor
// @idx: port index
// @irq_fault: virtual fault interrupt number
// @irq_fault_name: irq fault name
// @irq_rx: virtual rx interrupt number
// @irq_rx_name: irq rx name
// @irq_tx: virtual tx interrupt number
// @irq_tx_name: irq tx name
// @cts_gpiod: clear to send GPIO
// @dev: device descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pic32_sport {
    pub port: uart_port,
    pub idx: c_int,
    pub irq_fault: c_int,
    pub irq_fault_name: *const c_char,
    pub irq_rx: c_int,
    pub irq_rx_name: *const c_char,
    pub irq_tx: c_int,
    pub irq_tx_name: *const c_char,
    pub enable_tx_irq: bool,
    pub cts_gpiod: *mut gpio_desc,
    pub clk: *mut clk,
    pub dev: *mut device,
}

    static inline struct pic32_sport *to_pic32_sport(struct uart_port *port)
    {
    return container_of(port, struct pic32_sport, port);
    }
    static inline void pic32_uart_writel(struct pic32_sport *sport,
    u32 reg, u32 val)
    {
    __raw_writel(val, sport.port.membase + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn pic32_uart_readl(sport: *mut pic32_sport, reg: u32) -> u32 {
    static inline u32 pic32_uart_readl(struct pic32_sport *sport, u32 reg)
    {
    return	__raw_readl(sport.port.membase + reg);
    }
// pic32 uart mode register bits

// pic32 uart status register bits

// pic32_sport pointer for console use
    static struct pic32_sport *pic32_sports[PIC32_MAX_UARTS];
#[no_mangle]
pub unsafe extern "C" fn pic32_wait_deplete_txbuf(sport: *mut pic32_sport) {
    static inline void pic32_wait_deplete_txbuf(struct pic32_sport *sport)
    {
// wait for tx empty, otherwise chars will be lost or corrupted
    while (!(pic32_uart_readl(sport, PIC32_UART_STA) & PIC32_UART_STA_TRMT))
    udelay(1);
    }
// serial core request to check if uart tx buffer is empty
#[no_mangle]
unsafe extern "C" fn pic32_uart_tx_empty(port: *mut uart_port) -> c_uint {
    static unsigned int pic32_uart_tx_empty(struct uart_port *port)
    {
    struct pic32_sport *sport = to_pic32_sport(port);
    let mut val: u32 = pic32_uart_readl(sport, PIC32_UART_STA);
    return (val & PIC32_UART_STA_TRMT) ? 1 : 0;
    }
// serial core request to set UART outputs
#[no_mangle]
unsafe extern "C" fn pic32_uart_set_mctrl(port: *mut uart_port, mctrl: c_uint) {
    static void pic32_uart_set_mctrl(struct uart_port *port, unsigned int mctrl)
    {
    struct pic32_sport *sport = to_pic32_sport(port);
// set loopback mode
    if (mctrl & TIOCM_LOOP)
    pic32_uart_writel(sport, PIC32_SET(PIC32_UART_MODE),
    PIC32_UART_MODE_LPBK);
    else
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_MODE),
    PIC32_UART_MODE_LPBK);
    }
// serial core request to return the state of misc UART input pins
#[no_mangle]
unsafe extern "C" fn pic32_uart_get_mctrl(port: *mut uart_port) -> c_uint {
    static unsigned int pic32_uart_get_mctrl(struct uart_port *port)
    {
    struct pic32_sport *sport = to_pic32_sport(port);
    let mut mctrl: c_uint = 0;
// get the state of CTS input pin for this port
    if (!sport.cts_gpiod)
    mctrl |= TIOCM_CTS;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: gpiod_get_value(sport->cts_gpiod)) -> else {
    else if (gpiod_get_value(sport.cts_gpiod))
    mctrl |= TIOCM_CTS;
// DSR and CD are not supported in PIC32, so return 1
// RI is not supported in PIC32, so return 0
//
    mctrl |= TIOCM_CD;
    mctrl |= TIOCM_DSR;
    return mctrl;
    }
// stop tx and start tx are not called in pairs, therefore a flag indicates
// the status of irq to control the irq-depth.
//
#[no_mangle]
pub unsafe extern "C" fn pic32_uart_irqtxen(sport: *mut pic32_sport, en: u8) {
    static inline void pic32_uart_irqtxen(struct pic32_sport *sport, u8 en)
    {
    if (en && !sport.enable_tx_irq) {
    enable_irq(sport.irq_tx);
    sport.enable_tx_irq = true;
    } else if (!en && sport.enable_tx_irq) {
// use disable_irq_nosync() and not disable_irq() to avoid self
// imposed deadlock by not waiting for irq handler to end,
// since this callback is called from interrupt context.
//
    disable_irq_nosync(sport.irq_tx);
    sport.enable_tx_irq = false;
    }
    }
// serial core request to disable tx ASAP (used for flow control)
#[no_mangle]
unsafe extern "C" fn pic32_uart_stop_tx(port: *mut uart_port) {
    static void pic32_uart_stop_tx(struct uart_port *port)
    {
    struct pic32_sport *sport = to_pic32_sport(port);
    if (!(pic32_uart_readl(sport, PIC32_UART_MODE) & PIC32_UART_MODE_ON))
    return;
    if (!(pic32_uart_readl(sport, PIC32_UART_STA) & PIC32_UART_STA_UTXEN))
    return;
// wait for tx empty
    pic32_wait_deplete_txbuf(sport);
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_STA),
    PIC32_UART_STA_UTXEN);
    pic32_uart_irqtxen(sport, 0);
    }
// serial core request to (re)enable tx
#[no_mangle]
unsafe extern "C" fn pic32_uart_start_tx(port: *mut uart_port) {
    static void pic32_uart_start_tx(struct uart_port *port)
    {
    struct pic32_sport *sport = to_pic32_sport(port);
    pic32_uart_irqtxen(sport, 1);
    pic32_uart_writel(sport, PIC32_SET(PIC32_UART_STA),
    PIC32_UART_STA_UTXEN);
    }
// serial core request to stop rx, called before port shutdown
#[no_mangle]
unsafe extern "C" fn pic32_uart_stop_rx(port: *mut uart_port) {
    static void pic32_uart_stop_rx(struct uart_port *port)
    {
    struct pic32_sport *sport = to_pic32_sport(port);
// disable rx interrupts
    disable_irq(sport.irq_rx);
// receiver Enable bit OFF
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_STA),
    PIC32_UART_STA_URXEN);
    }
// serial core request to start/stop emitting break char
#[no_mangle]
unsafe extern "C" fn pic32_uart_break_ctl(port: *mut uart_port, ctl: c_int) {
    static void pic32_uart_break_ctl(struct uart_port *port, int ctl)
    {
    struct pic32_sport *sport = to_pic32_sport(port);
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
    if (ctl)
    pic32_uart_writel(sport, PIC32_SET(PIC32_UART_STA),
    PIC32_UART_STA_UTXBRK);
    else
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_STA),
    PIC32_UART_STA_UTXBRK);
    uart_port_unlock_irqrestore(port, flags);
    }
// get port type in string format
    static const char *pic32_uart_type(struct uart_port *port)
    {
    return (port.type == PORT_PIC32) ? PIC32_DEV_NAME : core::ptr::null_mut();
    }
// read all chars in rx fifo and send them to core
#[no_mangle]
unsafe extern "C" fn pic32_uart_do_rx(port: *mut uart_port) {
    static void pic32_uart_do_rx(struct uart_port *port)
    {
    struct pic32_sport *sport = to_pic32_sport(port);
    struct tty_port *tty;
    unsigned int max_count;
// limit number of char read in interrupt, should not be
// higher than fifo size anyway since we're much faster than
// serial port
//
    max_count = PIC32_UART_RX_FIFO_DEPTH;
    uart_port_lock(port);
    tty = &port.state.port;
    do {
    u32 sta_reg, c;
    char flag;
// get overrun/fifo empty information from status register
    sta_reg = pic32_uart_readl(sport, PIC32_UART_STA);
    if (unlikely(sta_reg & PIC32_UART_STA_OERR)) {
// fifo reset is required to clear interrupt
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_STA),
    PIC32_UART_STA_OERR);
    port.icount.overrun++;
    tty_insert_flip_char(tty, 0, TTY_OVERRUN);
    }
// Can at least one more character can be read?
    if (!(sta_reg & PIC32_UART_STA_URXDA))
    break;
// read the character and increment the rx counter
    c = pic32_uart_readl(sport, PIC32_UART_RX);
    port.icount.rx++;
    flag = TTY_NORMAL;
    c &= 0xff;
    if (unlikely((sta_reg & PIC32_UART_STA_PERR) ||
    (sta_reg & PIC32_UART_STA_FERR))) {
// do stats first
    if (sta_reg & PIC32_UART_STA_PERR)
    port.icount.parity++;
    if (sta_reg & PIC32_UART_STA_FERR)
    port.icount.frame++;
// update flag wrt read_status_mask
    sta_reg &= port.read_status_mask;
    if (sta_reg & PIC32_UART_STA_FERR)
    flag = TTY_FRAME;
    if (sta_reg & PIC32_UART_STA_PERR)
    flag = TTY_PARITY;
    }
    if (uart_handle_sysrq_char(port, c))
    continue;
    if ((sta_reg & port.ignore_status_mask) == 0)
    tty_insert_flip_char(tty, c, flag);
    } while (--max_count);
    uart_port_unlock(port);
    tty_flip_buffer_push(tty);
    }
// fill tx fifo with chars to send, stop when fifo is about to be full
// or when all chars have been sent.
//
#[no_mangle]
unsafe extern "C" fn pic32_uart_do_tx(port: *mut uart_port) {
    static void pic32_uart_do_tx(struct uart_port *port)
    {
    struct pic32_sport *sport = to_pic32_sport(port);
    struct tty_port *tport = &port.state.port;
    let mut max_count: c_uint = PIC32_UART_TX_FIFO_DEPTH;
    if (port.x_char) {
    pic32_uart_writel(sport, PIC32_UART_TX, port.x_char);
    port.icount.tx++;
    port.x_char = 0;
    return;
    }
    if (uart_tx_stopped(port)) {
    pic32_uart_stop_tx(port);
    return;
    }
    if (kfifo_is_empty(&tport.xmit_fifo))
    goto txq_empty;
// keep stuffing chars into uart tx buffer
// 1) until uart fifo is full
// or
// 2) until the circ buffer is empty
// (all chars have been sent)
// or
// 3) until the max count is reached
// (prevents lingering here for too long in certain cases)
//
    while (!(PIC32_UART_STA_UTXBF &
    pic32_uart_readl(sport, PIC32_UART_STA))) {
    unsigned char c;
    if (!uart_fifo_get(port, &c))
    break;
    pic32_uart_writel(sport, PIC32_UART_TX, c);
    if (--max_count == 0)
    break;
    }
    if (kfifo_len(&tport.xmit_fifo) < WAKEUP_CHARS)
    uart_write_wakeup(port);
    if (kfifo_is_empty(&tport.xmit_fifo))
    goto txq_empty;
    return;
    txq_empty:
    pic32_uart_irqtxen(sport, 0);
    }
// RX interrupt handler
#[no_mangle]
unsafe extern "C" fn pic32_uart_rx_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pic32_uart_rx_interrupt(int irq, void *dev_id)
    {
    struct uart_port *port = dev_id;
    pic32_uart_do_rx(port);
    return IRQ_HANDLED;
    }
// TX interrupt handler
#[no_mangle]
unsafe extern "C" fn pic32_uart_tx_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pic32_uart_tx_interrupt(int irq, void *dev_id)
    {
    struct uart_port *port = dev_id;
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
    pic32_uart_do_tx(port);
    uart_port_unlock_irqrestore(port, flags);
    return IRQ_HANDLED;
    }
// FAULT interrupt handler
#[no_mangle]
unsafe extern "C" fn pic32_uart_fault_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pic32_uart_fault_interrupt(int irq, void *dev_id)
    {
// do nothing: pic32_uart_do_rx() handles faults.
    return IRQ_HANDLED;
    }
// enable rx & tx operation on uart
#[no_mangle]
unsafe extern "C" fn pic32_uart_en_and_unmask(port: *mut uart_port) {
    static void pic32_uart_en_and_unmask(struct uart_port *port)
    {
    struct pic32_sport *sport = to_pic32_sport(port);
    pic32_uart_writel(sport, PIC32_SET(PIC32_UART_STA),
    PIC32_UART_STA_UTXEN | PIC32_UART_STA_URXEN);
    pic32_uart_writel(sport, PIC32_SET(PIC32_UART_MODE),
    PIC32_UART_MODE_ON);
    }
// disable rx & tx operation on uart
#[no_mangle]
unsafe extern "C" fn pic32_uart_dsbl_and_mask(port: *mut uart_port) {
    static void pic32_uart_dsbl_and_mask(struct uart_port *port)
    {
    struct pic32_sport *sport = to_pic32_sport(port);
// wait for tx empty, otherwise chars will be lost or corrupted
    pic32_wait_deplete_txbuf(sport);
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_STA),
    PIC32_UART_STA_UTXEN | PIC32_UART_STA_URXEN);
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_MODE),
    PIC32_UART_MODE_ON);
    }
// serial core request to initialize uart and start rx operation
#[no_mangle]
unsafe extern "C" fn pic32_uart_startup(port: *mut uart_port) -> c_int {
    static int pic32_uart_startup(struct uart_port *port)
    {
    struct pic32_sport *sport = to_pic32_sport(port);
    let mut dflt_baud: u32 = (port.uartclk / PIC32_UART_DFLT_BRATE / 16) - 1;
    unsigned long flags;
    int ret;
    local_irq_save(flags);
    ret = clk_prepare_enable(sport.clk);
    if (ret) {
    local_irq_restore(flags);
    goto out_done;
    }
// clear status and mode registers
    pic32_uart_writel(sport, PIC32_UART_MODE, 0);
    pic32_uart_writel(sport, PIC32_UART_STA, 0);
// disable uart and mask all interrupts
    pic32_uart_dsbl_and_mask(port);
// set default baud
    pic32_uart_writel(sport, PIC32_UART_BRG, dflt_baud);
    local_irq_restore(flags);
// Each UART of a PIC32 has three interrupts therefore,
// we setup driver to register the 3 irqs for the device.
//
// For each irq request_irq() is called with interrupt disabled.
// And the irq is enabled as soon as we are ready to handle them.
//
    sport.enable_tx_irq = false;
    sport.irq_fault_name = kasprintf(GFP_KERNEL, "%s%d-fault",
    pic32_uart_type(port),
    sport.idx);
    if (!sport.irq_fault_name) {
    dev_err(port.dev, "%s: kasprintf err!", __func__);
    ret = -ENOMEM;
    goto out_disable_clk;
    }
    irq_set_status_flags(sport.irq_fault, IRQ_NOAUTOEN);
    ret = request_irq(sport.irq_fault, pic32_uart_fault_interrupt,
    IRQF_NO_THREAD, sport.irq_fault_name, port);
    if (ret) {
    dev_err(port.dev, "%s: request irq(%d) err! ret:%d name:%s\n",
    __func__, sport.irq_fault, ret,
    pic32_uart_type(port));
    goto out_f;
    }
    sport.irq_rx_name = kasprintf(GFP_KERNEL, "%s%d-rx",
    pic32_uart_type(port),
    sport.idx);
    if (!sport.irq_rx_name) {
    dev_err(port.dev, "%s: kasprintf err!", __func__);
    ret = -ENOMEM;
    goto out_f;
    }
    irq_set_status_flags(sport.irq_rx, IRQ_NOAUTOEN);
    ret = request_irq(sport.irq_rx, pic32_uart_rx_interrupt,
    IRQF_NO_THREAD, sport.irq_rx_name, port);
    if (ret) {
    dev_err(port.dev, "%s: request irq(%d) err! ret:%d name:%s\n",
    __func__, sport.irq_rx, ret,
    pic32_uart_type(port));
    goto out_r;
    }
    sport.irq_tx_name = kasprintf(GFP_KERNEL, "%s%d-tx",
    pic32_uart_type(port),
    sport.idx);
    if (!sport.irq_tx_name) {
    dev_err(port.dev, "%s: kasprintf err!", __func__);
    ret = -ENOMEM;
    goto out_r;
    }
    irq_set_status_flags(sport.irq_tx, IRQ_NOAUTOEN);
    ret = request_irq(sport.irq_tx, pic32_uart_tx_interrupt,
    IRQF_NO_THREAD, sport.irq_tx_name, port);
    if (ret) {
    dev_err(port.dev, "%s: request irq(%d) err! ret:%d name:%s\n",
    __func__, sport.irq_tx, ret,
    pic32_uart_type(port));
    goto out_t;
    }
    local_irq_save(flags);
// set rx interrupt on first receive
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_STA),
    PIC32_UART_STA_URXISEL1 | PIC32_UART_STA_URXISEL0);
// set interrupt on empty
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_STA),
    PIC32_UART_STA_UTXISEL1);
// enable all interrupts and eanable uart
    pic32_uart_en_and_unmask(port);
    local_irq_restore(flags);
    enable_irq(sport.irq_rx);
    return 0;
    out_t:
    free_irq(sport.irq_tx, port);
    kfree(sport.irq_tx_name);
    out_r:
    free_irq(sport.irq_rx, port);
    kfree(sport.irq_rx_name);
    out_f:
    free_irq(sport.irq_fault, port);
    kfree(sport.irq_fault_name);
    out_disable_clk:
    clk_disable_unprepare(sport.clk);
    out_done:
    return ret;
    }
// serial core request to flush & disable uart
#[no_mangle]
unsafe extern "C" fn pic32_uart_shutdown(port: *mut uart_port) {
    static void pic32_uart_shutdown(struct uart_port *port)
    {
    struct pic32_sport *sport = to_pic32_sport(port);
    unsigned long flags;
// disable uart
    uart_port_lock_irqsave(port, &flags);
    pic32_uart_dsbl_and_mask(port);
    uart_port_unlock_irqrestore(port, flags);
    clk_disable_unprepare(sport.clk);
// free all 3 interrupts for this UART
    free_irq(sport.irq_fault, port);
    kfree(sport.irq_fault_name);
    free_irq(sport.irq_tx, port);
    kfree(sport.irq_tx_name);
    free_irq(sport.irq_rx, port);
    kfree(sport.irq_rx_name);
    }
// serial core request to change current uart setting
    static void pic32_uart_set_termios(struct uart_port *port,
    struct ktermios *new,
    const struct ktermios *old)
    {
    struct pic32_sport *sport = to_pic32_sport(port);
    unsigned int baud;
    unsigned int quot;
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
// disable uart and mask all interrupts while changing speed
    pic32_uart_dsbl_and_mask(port);
// stop bit options
    if (new.c_cflag & CSTOPB)
    pic32_uart_writel(sport, PIC32_SET(PIC32_UART_MODE),
    PIC32_UART_MODE_STSEL);
    else
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_MODE),
    PIC32_UART_MODE_STSEL);
// parity options
    if (new.c_cflag & PARENB) {
    if (new.c_cflag & PARODD) {
    pic32_uart_writel(sport, PIC32_SET(PIC32_UART_MODE),
    PIC32_UART_MODE_PDSEL1);
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_MODE),
    PIC32_UART_MODE_PDSEL0);
    } else {
    pic32_uart_writel(sport, PIC32_SET(PIC32_UART_MODE),
    PIC32_UART_MODE_PDSEL0);
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_MODE),
    PIC32_UART_MODE_PDSEL1);
    }
    } else {
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_MODE),
    PIC32_UART_MODE_PDSEL1 |
    PIC32_UART_MODE_PDSEL0);
    }
// if hw flow ctrl, then the pins must be specified in device tree
    if ((new.c_cflag & CRTSCTS) && sport.cts_gpiod) {
// enable hardware flow control
    pic32_uart_writel(sport, PIC32_SET(PIC32_UART_MODE),
    PIC32_UART_MODE_UEN1);
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_MODE),
    PIC32_UART_MODE_UEN0);
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_MODE),
    PIC32_UART_MODE_RTSMD);
    } else {
// disable hardware flow control
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_MODE),
    PIC32_UART_MODE_UEN1);
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_MODE),
    PIC32_UART_MODE_UEN0);
    pic32_uart_writel(sport, PIC32_CLR(PIC32_UART_MODE),
    PIC32_UART_MODE_RTSMD);
    }
// Always 8-bit
    new.c_cflag |= CS8;
// Mark/Space parity is not supported
    new.c_cflag &= ~CMSPAR;
// update baud
    baud = uart_get_baud_rate(port, new, old, 0, port.uartclk / 16);
    quot = uart_get_divisor(port, baud) - 1;
    pic32_uart_writel(sport, PIC32_UART_BRG, quot);
    uart_update_timeout(port, new.c_cflag, baud);
    if (tty_termios_baud_rate(new))
    tty_termios_encode_baud_rate(new, baud, baud);
// enable uart
    pic32_uart_en_and_unmask(port);
    uart_port_unlock_irqrestore(port, flags);
    }
// serial core request to claim uart iomem
#[no_mangle]
unsafe extern "C" fn pic32_uart_request_port(port: *mut uart_port) -> c_int {
    static int pic32_uart_request_port(struct uart_port *port)
    {
    struct platform_device *pdev = to_platform_device(port.dev);
    struct resource *res_mem;
    res_mem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (unlikely(!res_mem))
    return -EINVAL;
    if (!request_mem_region(port.mapbase, resource_size(res_mem),
    "pic32_uart_mem"))
    return -EBUSY;
    port.membase = devm_ioremap(port.dev, port.mapbase,
    resource_size(res_mem));
    if (!port.membase) {
    dev_err(port.dev, "Unable to map registers\n");
    release_mem_region(port.mapbase, resource_size(res_mem));
    return -ENOMEM;
    }
    return 0;
    }
// serial core request to release uart iomem
#[no_mangle]
unsafe extern "C" fn pic32_uart_release_port(port: *mut uart_port) {
    static void pic32_uart_release_port(struct uart_port *port)
    {
    struct platform_device *pdev = to_platform_device(port.dev);
    struct resource *res_mem;
    unsigned int res_size;
    res_mem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (unlikely(!res_mem))
    return;
    res_size = resource_size(res_mem);
    release_mem_region(port.mapbase, res_size);
    }
// serial core request to do any port required auto-configuration
#[no_mangle]
unsafe extern "C" fn pic32_uart_config_port(port: *mut uart_port, flags: c_int) {
    static void pic32_uart_config_port(struct uart_port *port, int flags)
    {
    if (flags & UART_CONFIG_TYPE) {
    if (pic32_uart_request_port(port))
    return;
    port.type = PORT_PIC32;
    }
    }
// serial core request to check that port information in serinfo are suitable
    static int pic32_uart_verify_port(struct uart_port *port,
    struct serial_struct *serinfo)
    {
    if (port.type != PORT_PIC32)
    return -EINVAL;
    if (port.irq != serinfo.irq)
    return -EINVAL;
    if (port.iotype != serinfo.io_type)
    return -EINVAL;
    if (port.mapbase != (unsigned long)serinfo.iomem_base)
    return -EINVAL;
    return 0;
    }
// serial core callbacks
    static const struct uart_ops pic32_uart_ops = {
    .tx_empty	= pic32_uart_tx_empty,
    .get_mctrl	= pic32_uart_get_mctrl,
    .set_mctrl	= pic32_uart_set_mctrl,
    .start_tx	= pic32_uart_start_tx,
    .stop_tx	= pic32_uart_stop_tx,
    .stop_rx	= pic32_uart_stop_rx,
    .break_ctl	= pic32_uart_break_ctl,
    .startup	= pic32_uart_startup,
    .shutdown	= pic32_uart_shutdown,
    .set_termios	= pic32_uart_set_termios,
    .type		= pic32_uart_type,
    .release_port	= pic32_uart_release_port,
    .request_port	= pic32_uart_request_port,
    .config_port	= pic32_uart_config_port,
    .verify_port	= pic32_uart_verify_port,
    };

// output given char
#[no_mangle]
unsafe extern "C" fn pic32_console_putchar(port: *mut uart_port, ch: c_uchar) {
    static void pic32_console_putchar(struct uart_port *port, unsigned char ch)
    {
    struct pic32_sport *sport = to_pic32_sport(port);
    if (!(pic32_uart_readl(sport, PIC32_UART_MODE) & PIC32_UART_MODE_ON))
    return;
    if (!(pic32_uart_readl(sport, PIC32_UART_STA) & PIC32_UART_STA_UTXEN))
    return;
// wait for tx empty
    pic32_wait_deplete_txbuf(sport);
    pic32_uart_writel(sport, PIC32_UART_TX, ch & 0xff);
    }
// console core request to output given string
    static void pic32_console_write(struct console *co, const char *s,
    unsigned int count)
    {
    struct pic32_sport *sport = pic32_sports[co.index];
// call uart helper to deal with \r\n
    uart_console_write(&sport.port, s, count, pic32_console_putchar);
    }
// console core request to setup given console, find matching uart
// port and setup it.
//
#[no_mangle]
unsafe extern "C" fn pic32_console_setup(co: *mut console, options: *mut c_char) -> c_int {
    static int pic32_console_setup(struct console *co, char *options)
    {
    struct pic32_sport *sport;
    let mut baud: c_int = 115200;
    let mut bits: c_int = 8;
    let mut parity: c_int = 'n';
    let mut flow: c_int = 'n';
    let mut ret: c_int = 0;
    if (unlikely(co.index < 0 || co.index >= PIC32_MAX_UARTS))
    return -ENODEV;
    sport = pic32_sports[co.index];
    if (!sport)
    return -ENODEV;
    ret = clk_prepare_enable(sport.clk);
    if (ret)
    return ret;
    if (options)
    uart_parse_options(options, &baud, &parity, &bits, &flow);
    return uart_set_options(&sport.port, co, baud, parity, bits, flow);
    }
    static struct uart_driver pic32_uart_driver;
    static struct console pic32_console = {
    .name		= PIC32_SDEV_NAME,
    .write		= pic32_console_write,
    .device		= uart_console_device,
    .setup		= pic32_console_setup,
    .flags		= CON_PRINTBUFFER,
    .index		= -1,
    .data		= &pic32_uart_driver,
    };

#[no_mangle]
unsafe extern "C" fn pic32_console_init() -> int __init {
    static int __init pic32_console_init(void)
    {
    register_console(&pic32_console);
    return 0;
    }
    console_initcall(pic32_console_init);
//
// Late console initialization.
//
#[no_mangle]
unsafe extern "C" fn pic32_late_console_init() -> int __init {
    static int __init pic32_late_console_init(void)
    {
    if (!console_is_registered(&pic32_console))
    register_console(&pic32_console);
    return 0;
    }
    core_initcall(pic32_late_console_init);

    static struct uart_driver pic32_uart_driver = {
    .owner			= THIS_MODULE,
    .driver_name		= PIC32_DEV_NAME,
    .dev_name		= PIC32_SDEV_NAME,
    .nr			= PIC32_MAX_UARTS,
    .cons			= PIC32_SCONSOLE,
    };
#[no_mangle]
unsafe extern "C" fn pic32_uart_probe(pdev: *mut platform_device) -> c_int {
    static int pic32_uart_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct pic32_sport *sport;
    let mut uart_idx: c_int = 0;
    struct resource *res_mem;
    struct uart_port *port;
    int ret;
    uart_idx = of_alias_get_id(np, "serial");
    if (uart_idx < 0 || uart_idx >= PIC32_MAX_UARTS)
    return -EINVAL;
    res_mem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res_mem)
    return -EINVAL;
    sport = devm_kzalloc(&pdev.dev, sizeof(*sport), GFP_KERNEL);
    if (!sport)
    return -ENOMEM;
    sport.idx		= uart_idx;
    sport.irq_fault	= irq_of_parse_and_map(np, 0);
    sport.irq_rx		= irq_of_parse_and_map(np, 1);
    sport.irq_tx		= irq_of_parse_and_map(np, 2);
    sport.clk		= devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(sport.clk))
    return PTR_ERR(sport.clk);
    sport.dev		= &pdev.dev;
// Hardware flow control: gpios
// !Note: Basically, CTS is needed for reading the status.
//
    sport.cts_gpiod = devm_gpiod_get_optional(dev, "cts", GPIOD_IN);
    if (IS_ERR(sport.cts_gpiod))
    return dev_err_probe(dev, PTR_ERR(sport.cts_gpiod), "error requesting CTS GPIO\n");
    gpiod_set_consumer_name(sport.cts_gpiod, "CTS");
    pic32_sports[uart_idx] = sport;
    port = &sport.port;
    port.iotype	= UPIO_MEM;
    port.mapbase	= res_mem.start;
    port.ops	= &pic32_uart_ops;
    port.flags	= UPF_BOOT_AUTOCONF;
    port.dev	= &pdev.dev;
    port.fifosize	= PIC32_UART_TX_FIFO_DEPTH;
    port.uartclk	= clk_get_rate(sport.clk);
    port.line	= uart_idx;
    ret = uart_add_one_port(&pic32_uart_driver, port);
    if (ret) {
    port.membase = core::ptr::null_mut();
    dev_err(port.dev, "%s: uart add port error!\n", __func__);
    goto err;
    }

    if (uart_console_registered(port)) {
// The peripheral clock has been enabled by console_setup,
// so disable it till the port is used.
//
    clk_disable_unprepare(sport.clk);
    }

    platform_set_drvdata(pdev, port);
    dev_info(&pdev.dev, "%s: uart(%d) driver initialized.\n",
    __func__, uart_idx);
    return 0;
    err:
// automatic unroll of sport and gpios
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pic32_uart_remove(pdev: *mut platform_device) {
    static void pic32_uart_remove(struct platform_device *pdev)
    {
    struct uart_port *port = platform_get_drvdata(pdev);
    struct pic32_sport *sport = to_pic32_sport(port);
    uart_remove_one_port(&pic32_uart_driver, port);
    clk_disable_unprepare(sport.clk);
    platform_set_drvdata(pdev, core::ptr::null_mut());
    pic32_sports[sport.idx] = core::ptr::null_mut();
    }
    static const struct of_device_id pic32_serial_dt_ids[] = {
    { .compatible = "microchip,pic32mzda-uart" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, pic32_serial_dt_ids);
    static struct platform_driver pic32_uart_platform_driver = {
    .probe		= pic32_uart_probe,
    .remove		= pic32_uart_remove,
    .driver		= {
    .name	= PIC32_DEV_NAME,
    .of_match_table	= of_match_ptr(pic32_serial_dt_ids),
    .suppress_bind_attrs = IS_BUILTIN(CONFIG_SERIAL_PIC32),
    },
    };
#[no_mangle]
unsafe extern "C" fn pic32_uart_init() -> int __init {
    static int __init pic32_uart_init(void)
    {
    int ret;
    ret = uart_register_driver(&pic32_uart_driver);
    if (ret) {
    pr_err("failed to register %s:%d\n",
    pic32_uart_driver.driver_name, ret);
    return ret;
    }
    ret = platform_driver_register(&pic32_uart_platform_driver);
    if (ret) {
    pr_err("fail to register pic32 uart\n");
    uart_unregister_driver(&pic32_uart_driver);
    }
    return ret;
    }
    arch_initcall(pic32_uart_init);
#[no_mangle]
unsafe extern "C" fn pic32_uart_exit() -> void __exit {
    static void __exit pic32_uart_exit(void)
    {

    unregister_console(&pic32_console);

    platform_driver_unregister(&pic32_uart_platform_driver);
    uart_unregister_driver(&pic32_uart_driver);
    }
    module_exit(pic32_uart_exit);
    MODULE_AUTHOR("Sorin-Andrei Pistirica <andrei.pistirica@microchip.com>");
    MODULE_DESCRIPTION("Microchip PIC32 integrated serial port driver");
    MODULE_LICENSE("GPL v2");

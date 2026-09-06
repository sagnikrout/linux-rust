//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/fsl_linflexuart.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Freescale LINFlexD UART serial port driver
//
// Copyright 2012-2016 Freescale Semiconductor, Inc.
// Copyright 2017-2019 NXP
//

// All registers are 32-bit width
pub const LINCR1: c_uint = 0x0000	/* LIN control register				*/;
pub const LINIER: c_uint = 0x0004	/* LIN interrupt enable register		*/;
pub const LINSR: c_uint = 0x0008	/* LIN status register				*/;
pub const LINESR: c_uint = 0x000C	/* LIN error status register			*/;
pub const UARTCR: c_uint = 0x0010	/* UART mode control register			*/;
pub const UARTSR: c_uint = 0x0014	/* UART mode status register			*/;
pub const LINTCSR: c_uint = 0x0018	/* LIN timeout control status register		*/;
pub const LINOCR: c_uint = 0x001C	/* LIN output compare register			*/;
pub const LINTOCR: c_uint = 0x0020	/* LIN timeout control register			*/;
pub const LINFBRR: c_uint = 0x0024	/* LIN fractional baud rate register		*/;
pub const LINIBRR: c_uint = 0x0028	/* LIN integer baud rate register		*/;
pub const LINCFR: c_uint = 0x002C	/* LIN checksum field register			*/;
pub const LINCR2: c_uint = 0x0030	/* LIN control register 2			*/;
pub const BIDR: c_uint = 0x0034	/* Buffer identifier register			*/;
pub const BDRL: c_uint = 0x0038	/* Buffer data register least significant	*/;
pub const BDRM: c_uint = 0x003C	/* Buffer data register most significant	*/;
pub const IFER: c_uint = 0x0040	/* Identifier filter enable register		*/;
pub const IFMI: c_uint = 0x0044	/* Identifier filter match index		*/;
pub const IFMR: c_uint = 0x0048	/* Identifier filter mode register		*/;
pub const GCR: c_uint = 0x004C	/* Global control register			*/;
pub const UARTPTO: c_uint = 0x0050	/* UART preset timeout register			*/;
pub const UARTCTO: c_uint = 0x0054	/* UART current timeout register		*/;
//
// Register field definitions
//

    & LINFLEXD_UARTCR_OSR_MASK) >> 24)

    LINFLEXD_UARTSR_PE1 |\
    LINFLEXD_UARTSR_PE2 |\
    LINFLEXD_UARTSR_PE3)

pub const UART_NR: c_int = 4;
pub const EARLYCON_BUFFER_INITIAL_CAP: c_int = 8;

    static const struct of_device_id linflex_dt_ids[] = {
    {
    .compatible = "fsl,s32v234-linflexuart",
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, linflex_dt_ids);

    static struct uart_port *earlycon_port;
    static bool linflex_earlycon_same_instance;
    static DEFINE_SPINLOCK(init_lock);
    static bool during_init;
    static struct {
    char *content;
    unsigned int len, cap;
    } earlycon_buf;

#[no_mangle]
unsafe extern "C" fn linflex_stop_tx(port: *mut uart_port) {
    static void linflex_stop_tx(struct uart_port *port)
    {
    unsigned long ier;
    ier = readl(port.membase + LINIER);
    ier &= ~(LINFLEXD_LINIER_DTIE);
    writel(ier, port.membase + LINIER);
    }
#[no_mangle]
unsafe extern "C" fn linflex_stop_rx(port: *mut uart_port) {
    static void linflex_stop_rx(struct uart_port *port)
    {
    unsigned long ier;
    ier = readl(port.membase + LINIER);
    writel(ier & ~LINFLEXD_LINIER_DRIE, port.membase + LINIER);
    }
#[no_mangle]
unsafe extern "C" fn linflex_put_char(sport: *mut uart_port, c: c_uchar) {
    static void linflex_put_char(struct uart_port *sport, unsigned char c)
    {
    unsigned long status;
    writeb(c, sport.membase + BDRL);
// Waiting for data transmission completed.
    while (((status = readl(sport.membase + UARTSR)) &
    LINFLEXD_UARTSR_DTFTFF) !=
    LINFLEXD_UARTSR_DTFTFF)
    ;
    writel(status | LINFLEXD_UARTSR_DTFTFF, sport.membase + UARTSR);
    }
#[no_mangle]
pub unsafe extern "C" fn linflex_transmit_buffer(sport: *mut uart_port) {
    static inline void linflex_transmit_buffer(struct uart_port *sport)
    {
    struct tty_port *tport = &sport.state.port;
    unsigned char c;
    while (uart_fifo_get(sport, &c)) {
    linflex_put_char(sport, c);
    sport.icount.tx++;
    }
    if (kfifo_len(&tport.xmit_fifo) < WAKEUP_CHARS)
    uart_write_wakeup(sport);
    if (kfifo_is_empty(&tport.xmit_fifo))
    linflex_stop_tx(sport);
    }
#[no_mangle]
unsafe extern "C" fn linflex_start_tx(port: *mut uart_port) {
    static void linflex_start_tx(struct uart_port *port)
    {
    unsigned long ier;
    linflex_transmit_buffer(port);
    ier = readl(port.membase + LINIER);
    writel(ier | LINFLEXD_LINIER_DTIE, port.membase + LINIER);
    }
#[no_mangle]
unsafe extern "C" fn linflex_txint(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t linflex_txint(int irq, void *dev_id)
    {
    struct uart_port *sport = dev_id;
    struct tty_port *tport = &sport.state.port;
    unsigned long flags;
    uart_port_lock_irqsave(sport, &flags);
    if (sport.x_char) {
    linflex_put_char(sport, sport.x_char);
    goto out;
    }
    if (kfifo_is_empty(&tport.xmit_fifo) || uart_tx_stopped(sport)) {
    linflex_stop_tx(sport);
    goto out;
    }
    linflex_transmit_buffer(sport);
    out:
    uart_port_unlock_irqrestore(sport, flags);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn linflex_rxint(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t linflex_rxint(int irq, void *dev_id)
    {
    struct uart_port *sport = dev_id;
    unsigned int flg;
    struct tty_port *port = &sport.state.port;
    unsigned long flags, status;
    unsigned char rx;
    bool brk;
    uart_port_lock_irqsave(sport, &flags);
    status = readl(sport.membase + UARTSR);
    while (status & LINFLEXD_UARTSR_RMB) {
    rx = readb(sport.membase + BDRM);
    brk = false;
    flg = TTY_NORMAL;
    sport.icount.rx++;
    if (status & (LINFLEXD_UARTSR_BOF | LINFLEXD_UARTSR_FEF |
    LINFLEXD_UARTSR_PE)) {
    if (status & LINFLEXD_UARTSR_BOF)
    sport.icount.overrun++;
    if (status & LINFLEXD_UARTSR_FEF) {
    if (!rx) {
    brk = true;
    sport.icount.brk++;
    } else
    sport.icount.frame++;
    }
    if (status & LINFLEXD_UARTSR_PE)
    sport.icount.parity++;
    }
    writel(status, sport.membase + UARTSR);
    status = readl(sport.membase + UARTSR);
    if (brk) {
    uart_handle_break(sport);
    } else {
    if (uart_handle_sysrq_char(sport, (unsigned char)rx))
    continue;
    tty_insert_flip_char(port, rx, flg);
    }
    }
    uart_port_unlock_irqrestore(sport, flags);
    tty_flip_buffer_push(port);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn linflex_int(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t linflex_int(int irq, void *dev_id)
    {
    struct uart_port *sport = dev_id;
    unsigned long status;
    status = readl(sport.membase + UARTSR);
    if (status & LINFLEXD_UARTSR_DRFRFE)
    linflex_rxint(irq, dev_id);
    if (status & LINFLEXD_UARTSR_DTFTFF)
    linflex_txint(irq, dev_id);
    return IRQ_HANDLED;
    }
// return TIOCSER_TEMT when transmitter is not busy
#[no_mangle]
unsafe extern "C" fn linflex_tx_empty(port: *mut uart_port) -> c_uint {
    static unsigned int linflex_tx_empty(struct uart_port *port)
    {
    unsigned long status;
    status = readl(port.membase + UARTSR) & LINFLEXD_UARTSR_DTFTFF;
    return status ? TIOCSER_TEMT : 0;
    }
#[no_mangle]
unsafe extern "C" fn linflex_get_mctrl(port: *mut uart_port) -> c_uint {
    static unsigned int linflex_get_mctrl(struct uart_port *port)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn linflex_set_mctrl(port: *mut uart_port, mctrl: c_uint) {
    static void linflex_set_mctrl(struct uart_port *port, unsigned int mctrl)
    {
    }
#[no_mangle]
unsafe extern "C" fn linflex_break_ctl(port: *mut uart_port, break_state: c_int) {
    static void linflex_break_ctl(struct uart_port *port, int break_state)
    {
    }
#[no_mangle]
unsafe extern "C" fn linflex_setup_watermark(sport: *mut uart_port) {
    static void linflex_setup_watermark(struct uart_port *sport)
    {
    unsigned long cr, ier, cr1;
// Disable transmission/reception
    ier = readl(sport.membase + LINIER);
    ier &= ~(LINFLEXD_LINIER_DRIE | LINFLEXD_LINIER_DTIE);
    writel(ier, sport.membase + LINIER);
    cr = readl(sport.membase + UARTCR);
    cr &= ~(LINFLEXD_UARTCR_RXEN | LINFLEXD_UARTCR_TXEN);
    writel(cr, sport.membase + UARTCR);
// Enter initialization mode by setting INIT bit
// set the Linflex in master mode and activate by-pass filter
    cr1 = LINFLEXD_LINCR1_BF | LINFLEXD_LINCR1_MME
    | LINFLEXD_LINCR1_INIT;
    writel(cr1, sport.membase + LINCR1);
// wait for init mode entry
    while ((readl(sport.membase + LINSR)
    & LINFLEXD_LINSR_LINS_MASK)
    != LINFLEXD_LINSR_LINS_INITMODE)
    ;
//
// UART = 0x1;		- Linflex working in UART mode
// TXEN = 0x1;		- Enable transmission of data now
// RXEn = 0x1;		- Receiver enabled
// WL0 = 0x1;		- 8 bit data
// PCE = 0x0;		- No parity
//
// set UART bit to allow writing other bits
    writel(LINFLEXD_UARTCR_UART, sport.membase + UARTCR);
    cr = (LINFLEXD_UARTCR_RXEN | LINFLEXD_UARTCR_TXEN |
    LINFLEXD_UARTCR_WL0 | LINFLEXD_UARTCR_UART);
    writel(cr, sport.membase + UARTCR);
    cr1 &= ~(LINFLEXD_LINCR1_INIT);
    writel(cr1, sport.membase + LINCR1);
    ier = readl(sport.membase + LINIER);
    ier |= LINFLEXD_LINIER_DRIE;
    ier |= LINFLEXD_LINIER_DTIE;
    writel(ier, sport.membase + LINIER);
    }
#[no_mangle]
unsafe extern "C" fn linflex_startup(port: *mut uart_port) -> c_int {
    static int linflex_startup(struct uart_port *port)
    {
    let mut ret: c_int = 0;
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
    linflex_setup_watermark(port);
    uart_port_unlock_irqrestore(port, flags);
    ret = devm_request_irq(port.dev, port.irq, linflex_int, 0,
    DRIVER_NAME, port);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn linflex_shutdown(port: *mut uart_port) {
    static void linflex_shutdown(struct uart_port *port)
    {
    unsigned long ier;
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
// disable interrupts
    ier = readl(port.membase + LINIER);
    ier &= ~(LINFLEXD_LINIER_DRIE | LINFLEXD_LINIER_DTIE);
    writel(ier, port.membase + LINIER);
    uart_port_unlock_irqrestore(port, flags);
    devm_free_irq(port.dev, port.irq, port);
    }
    static void
    linflex_set_termios(struct uart_port *port, struct ktermios *termios,
    const struct ktermios *old)
    {
    unsigned long flags;
    unsigned long cr, old_cr, cr1;
    let mut old_csize: c_uint = old ? old.c_cflag & CSIZE : CS8;
    cr = readl(port.membase + UARTCR);
    old_cr = cr;
// Enter initialization mode by setting INIT bit
    cr1 = readl(port.membase + LINCR1);
    cr1 |= LINFLEXD_LINCR1_INIT;
    writel(cr1, port.membase + LINCR1);
// wait for init mode entry
    while ((readl(port.membase + LINSR)
    & LINFLEXD_LINSR_LINS_MASK)
    != LINFLEXD_LINSR_LINS_INITMODE)
    ;
//
// only support CS8 and CS7, and for CS7 must enable PE.
// supported mode:
// - (7,e/o,1)
// - (8,n,1)
// - (8,e/o,1)
//
// enter the UART into configuration mode
    while ((termios.c_cflag & CSIZE) != CS8 &&
    (termios.c_cflag & CSIZE) != CS7) {
    termios.c_cflag &= ~CSIZE;
    termios.c_cflag |= old_csize;
    old_csize = CS8;
    }
    if ((termios.c_cflag & CSIZE) == CS7) {
// Word length: WL1WL0:00
    cr = old_cr & ~LINFLEXD_UARTCR_WL1 & ~LINFLEXD_UARTCR_WL0;
    }
    if ((termios.c_cflag & CSIZE) == CS8) {
// Word length: WL1WL0:01
    cr = (old_cr | LINFLEXD_UARTCR_WL0) & ~LINFLEXD_UARTCR_WL1;
    }
    if (termios.c_cflag & CMSPAR) {
    if ((termios.c_cflag & CSIZE) != CS8) {
    termios.c_cflag &= ~CSIZE;
    termios.c_cflag |= CS8;
    }
// has a space/sticky bit
    cr |= LINFLEXD_UARTCR_WL0;
    }
    if (termios.c_cflag & CSTOPB)
    termios.c_cflag &= ~CSTOPB;
// parity must be enabled when CS7 to match 8-bits format
    if ((termios.c_cflag & CSIZE) == CS7)
    termios.c_cflag |= PARENB;
    if ((termios.c_cflag & PARENB)) {
    cr |= LINFLEXD_UARTCR_PCE;
    if (termios.c_cflag & PARODD)
    cr = (cr | LINFLEXD_UARTCR_PC0) &
    (~LINFLEXD_UARTCR_PC1);
    else
    cr = cr & (~LINFLEXD_UARTCR_PC1 &
    ~LINFLEXD_UARTCR_PC0);
    } else {
    cr &= ~LINFLEXD_UARTCR_PCE;
    }
    uart_port_lock_irqsave(port, &flags);
    port.read_status_mask = 0;
    if (termios.c_iflag & INPCK)
    port.read_status_mask |=	(LINFLEXD_UARTSR_FEF |
    LINFLEXD_UARTSR_PE0 |
    LINFLEXD_UARTSR_PE1 |
    LINFLEXD_UARTSR_PE2 |
    LINFLEXD_UARTSR_PE3);
    if (termios.c_iflag & (IGNBRK | BRKINT | PARMRK))
    port.read_status_mask |= LINFLEXD_UARTSR_FEF;
// characters to ignore
    port.ignore_status_mask = 0;
    if (termios.c_iflag & IGNPAR)
    port.ignore_status_mask |= LINFLEXD_UARTSR_PE;
    if (termios.c_iflag & IGNBRK) {
    port.ignore_status_mask |= LINFLEXD_UARTSR_PE;
//
// if we're ignoring parity and break indicators,
// ignore overruns too (for real raw support).
//
    if (termios.c_iflag & IGNPAR)
    port.ignore_status_mask |= LINFLEXD_UARTSR_BOF;
    }
    writel(cr, port.membase + UARTCR);
    cr1 &= ~(LINFLEXD_LINCR1_INIT);
    writel(cr1, port.membase + LINCR1);
    uart_port_unlock_irqrestore(port, flags);
    }
    static const char *linflex_type(struct uart_port *port)
    {
    return "FSL_LINFLEX";
    }
#[no_mangle]
unsafe extern "C" fn linflex_release_port(port: *mut uart_port) {
    static void linflex_release_port(struct uart_port *port)
    {
// nothing to do
    }
#[no_mangle]
unsafe extern "C" fn linflex_request_port(port: *mut uart_port) -> c_int {
    static int linflex_request_port(struct uart_port *port)
    {
    return 0;
    }
// configure/auto-configure the port
#[no_mangle]
unsafe extern "C" fn linflex_config_port(port: *mut uart_port, flags: c_int) {
    static void linflex_config_port(struct uart_port *port, int flags)
    {
    if (flags & UART_CONFIG_TYPE)
    port.type = PORT_LINFLEXUART;
    }
    static const struct uart_ops linflex_pops = {
    .tx_empty	= linflex_tx_empty,
    .set_mctrl	= linflex_set_mctrl,
    .get_mctrl	= linflex_get_mctrl,
    .stop_tx	= linflex_stop_tx,
    .start_tx	= linflex_start_tx,
    .stop_rx	= linflex_stop_rx,
    .break_ctl	= linflex_break_ctl,
    .startup	= linflex_startup,
    .shutdown	= linflex_shutdown,
    .set_termios	= linflex_set_termios,
    .type		= linflex_type,
    .request_port	= linflex_request_port,
    .release_port	= linflex_release_port,
    .config_port	= linflex_config_port,
    };
    static struct uart_port *linflex_ports[UART_NR];

#[no_mangle]
unsafe extern "C" fn linflex_console_putchar(port: *mut uart_port, ch: c_uchar) {
    static void linflex_console_putchar(struct uart_port *port, unsigned char ch)
    {
    unsigned long cr;
    cr = readl(port.membase + UARTCR);
    writeb(ch, port.membase + BDRL);
    if (!(cr & LINFLEXD_UARTCR_TFBM))
    while ((readl(port.membase + UARTSR) &
    LINFLEXD_UARTSR_DTFTFF)
    != LINFLEXD_UARTSR_DTFTFF)
    ;
    else
    while (readl(port.membase + UARTSR) &
    LINFLEXD_UARTSR_DTFTFF)
    ;
    if (!(cr & LINFLEXD_UARTCR_TFBM)) {
    writel((readl(port.membase + UARTSR) |
    LINFLEXD_UARTSR_DTFTFF),
    port.membase + UARTSR);
    }
    }
#[no_mangle]
unsafe extern "C" fn linflex_earlycon_putchar(port: *mut uart_port, ch: c_uchar) {
    static void linflex_earlycon_putchar(struct uart_port *port, unsigned char ch)
    {
    unsigned long flags;
    char *ret;
    if (!linflex_earlycon_same_instance) {
    linflex_console_putchar(port, ch);
    return;
    }
    spin_lock_irqsave(&init_lock, flags);
    if (!during_init)
    goto outside_init;
    if (earlycon_buf.len >= 1 << CONFIG_LOG_BUF_SHIFT)
    goto init_release;
    if (!earlycon_buf.cap) {
    earlycon_buf.content = kmalloc(EARLYCON_BUFFER_INITIAL_CAP,
    GFP_ATOMIC);
    earlycon_buf.cap = earlycon_buf.content ?
    EARLYCON_BUFFER_INITIAL_CAP : 0;
    } else if (earlycon_buf.len == earlycon_buf.cap) {
    ret = krealloc(earlycon_buf.content, earlycon_buf.cap << 1,
    GFP_ATOMIC);
    if (ret) {
    earlycon_buf.content = ret;
    earlycon_buf.cap <<= 1;
    }
    }
    if (earlycon_buf.len < earlycon_buf.cap)
    earlycon_buf.content[earlycon_buf.len++] = ch;
    goto init_release;
    outside_init:
    linflex_console_putchar(port, ch);
    init_release:
    spin_unlock_irqrestore(&init_lock, flags);
    }
    static void linflex_string_write(struct uart_port *sport, const char *s,
    unsigned int count)
    {
    unsigned long cr, ier = 0;
    ier = readl(sport.membase + LINIER);
    linflex_stop_tx(sport);
    cr = readl(sport.membase + UARTCR);
    cr |= (LINFLEXD_UARTCR_TXEN);
    writel(cr, sport.membase + UARTCR);
    uart_console_write(sport, s, count, linflex_console_putchar);
    writel(ier, sport.membase + LINIER);
    }
    static void
    linflex_console_write(struct console *co, const char *s, unsigned int count)
    {
    struct uart_port *sport = linflex_ports[co.index];
    unsigned long flags;
    let mut locked: c_int = 1;
    if (sport.sysrq)
    locked = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: oops_in_progress) -> else {
    else if (oops_in_progress)
    locked = uart_port_trylock_irqsave(sport, &flags);
    else
    uart_port_lock_irqsave(sport, &flags);
    linflex_string_write(sport, s, count);
    if (locked)
    uart_port_unlock_irqrestore(sport, flags);
    }
//
// if the port was already initialised (eg, by a boot loader),
// try to determine the current setup.
//
    static void __init
    linflex_console_get_options(struct uart_port *sport, int *parity, int *bits)
    {
    unsigned long cr;
    cr = readl(sport.membase + UARTCR);
    cr &= LINFLEXD_UARTCR_RXEN | LINFLEXD_UARTCR_TXEN;
    if (!cr)
    return;
// ok, the port was enabled
// parity = 'n';
    if (cr & LINFLEXD_UARTCR_PCE) {
    if (cr & LINFLEXD_UARTCR_PC0)
// parity = 'o';
    else
// parity = 'e';
    }
    if ((cr & LINFLEXD_UARTCR_WL0) && ((cr & LINFLEXD_UARTCR_WL1) == 0)) {
    if (cr & LINFLEXD_UARTCR_PCE)
// bits = 9;
    else
// bits = 8;
    }
    }
#[no_mangle]
unsafe extern "C" fn linflex_console_setup(co: *mut console, options: *mut c_char) -> int __init {
    static int __init linflex_console_setup(struct console *co, char *options)
    {
    struct uart_port *sport;
    let mut baud: c_int = 115200;
    let mut bits: c_int = 8;
    let mut parity: c_int = 'n';
    let mut flow: c_int = 'n';
    int ret;
    int i;
    unsigned long flags;
//
// check whether an invalid uart number has been specified, and
// if so, search for the first available port that does have
// console support.
//
    if (co.index == -1 || co.index >= ARRAY_SIZE(linflex_ports))
    co.index = 0;
    sport = linflex_ports[co.index];
    if (!sport)
    return -ENODEV;
    if (options)
    uart_parse_options(options, &baud, &parity, &bits, &flow);
    else
    linflex_console_get_options(sport, &parity, &bits);
    if (earlycon_port && sport.mapbase == earlycon_port.mapbase) {
    linflex_earlycon_same_instance = true;
    spin_lock_irqsave(&init_lock, flags);
    during_init = true;
    spin_unlock_irqrestore(&init_lock, flags);
// Workaround for character loss or output of many invalid
// characters, when INIT mode is entered shortly after a
// character has just been printed.
//
    udelay(PREINIT_DELAY);
    }
    linflex_setup_watermark(sport);
    ret = uart_set_options(sport, co, baud, parity, bits, flow);
    if (!linflex_earlycon_same_instance)
    goto done;
    spin_lock_irqsave(&init_lock, flags);
// Emptying buffer
    if (earlycon_buf.len) {
    for (i = 0; i < earlycon_buf.len; i++)
    linflex_console_putchar(earlycon_port,
    earlycon_buf.content[i]);
    kfree(earlycon_buf.content);
    earlycon_buf.len = 0;
    }
    during_init = false;
    spin_unlock_irqrestore(&init_lock, flags);
    done:
    return ret;
    }
    static struct uart_driver linflex_reg;
    static struct console linflex_console = {
    .name		= DEV_NAME,
    .write		= linflex_console_write,
    .device		= uart_console_device,
    .setup		= linflex_console_setup,
    .flags		= CON_PRINTBUFFER,
    .index		= -1,
    .data		= &linflex_reg,
    };
    static void linflex_earlycon_write(struct console *con, const char *s,
    unsigned int n)
    {
    struct earlycon_device *dev = con.data;
    uart_console_write(&dev.port, s, n, linflex_earlycon_putchar);
    }
    static int __init linflex_early_console_setup(struct earlycon_device *device,
    const char *options)
    {
    if (!device.port.membase)
    return -ENODEV;
    device.con.write = linflex_earlycon_write;
    earlycon_port = &device.port;
    return 0;
    }
    OF_EARLYCON_DECLARE(linflex, "fsl,s32v234-linflexuart",
    linflex_early_console_setup);

    static struct uart_driver linflex_reg = {
    .owner		= THIS_MODULE,
    .driver_name	= DRIVER_NAME,
    .dev_name	= DEV_NAME,
    .nr		= ARRAY_SIZE(linflex_ports),
    .cons		= LINFLEX_CONSOLE,
    };
#[no_mangle]
unsafe extern "C" fn linflex_probe(pdev: *mut platform_device) -> c_int {
    static int linflex_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct uart_port *sport;
    struct resource *res;
    int ret;
    sport = devm_kzalloc(&pdev.dev, sizeof(*sport), GFP_KERNEL);
    if (!sport)
    return -ENOMEM;
    ret = of_alias_get_id(np, "serial");
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to get alias id, errno %d\n", ret);
    return ret;
    }
    if (ret >= UART_NR) {
    dev_err(&pdev.dev, "driver limited to %d serial ports\n",
    UART_NR);
    return -ENOMEM;
    }
    sport.line = ret;
    sport.membase = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(sport.membase))
    return PTR_ERR(sport.membase);
    sport.mapbase = res.start;
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    return ret;
    sport.dev = &pdev.dev;
    sport.iotype = UPIO_MEM;
    sport.irq = ret;
    sport.ops = &linflex_pops;
    sport.flags = UPF_BOOT_AUTOCONF;
    sport.has_sysrq = IS_ENABLED(CONFIG_SERIAL_FSL_LINFLEXUART_CONSOLE);
    linflex_ports[sport.line] = sport;
    platform_set_drvdata(pdev, sport);
    return uart_add_one_port(&linflex_reg, sport);
    }
#[no_mangle]
unsafe extern "C" fn linflex_remove(pdev: *mut platform_device) {
    static void linflex_remove(struct platform_device *pdev)
    {
    struct uart_port *sport = platform_get_drvdata(pdev);
    uart_remove_one_port(&linflex_reg, sport);
    }

#[no_mangle]
unsafe extern "C" fn linflex_suspend(dev: *mut device) -> c_int {
    static int linflex_suspend(struct device *dev)
    {
    struct uart_port *sport = dev_get_drvdata(dev);
    uart_suspend_port(&linflex_reg, sport);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn linflex_resume(dev: *mut device) -> c_int {
    static int linflex_resume(struct device *dev)
    {
    struct uart_port *sport = dev_get_drvdata(dev);
    uart_resume_port(&linflex_reg, sport);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(linflex_pm_ops, linflex_suspend, linflex_resume);
    static struct platform_driver linflex_driver = {
    .probe		= linflex_probe,
    .remove		= linflex_remove,
    .driver		= {
    .name	= DRIVER_NAME,
    .of_match_table	= linflex_dt_ids,
    .pm	= &linflex_pm_ops,
    },
    };
#[no_mangle]
unsafe extern "C" fn linflex_serial_init() -> int __init {
    static int __init linflex_serial_init(void)
    {
    int ret;
    ret = uart_register_driver(&linflex_reg);
    if (ret)
    return ret;
    ret = platform_driver_register(&linflex_driver);
    if (ret)
    uart_unregister_driver(&linflex_reg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn linflex_serial_exit() -> void __exit {
    static void __exit linflex_serial_exit(void)
    {
    platform_driver_unregister(&linflex_driver);
    uart_unregister_driver(&linflex_reg);
    }
    module_init(linflex_serial_init);
    module_exit(linflex_serial_exit);
    MODULE_DESCRIPTION("Freescale LINFlexD serial port driver");
    MODULE_LICENSE("GPL v2");

//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/core/sdio_uart.c
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
// SDIO UART/GPS driver
//
// Based on drivers/serial/8250.c and drivers/serial/serial_core.c
// by Russell King.
//
// Author:	Nicolas Pitre
// Created:	June 15, 2007
// Copyright:	MontaVista Software, Inc.
//
// Note: Although this driver assumes a 16550A-like UART implementation,
// it is not possible to leverage the common 8250/16550 driver, nor the
// core UART infrastructure, as they assumes direct access to the hardware
// registers, often under a spinlock.  This is not possible in the SDIO
// context as SDIO access functions must be able to sleep.
//
// Because we need to lock the SDIO host to ensure an exclusive access to
// the card, we simply rely on that lock to also prevent and serialize
// concurrent access to the same port.
//

pub const WAKEUP_CHARS: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uart_icount {
    pub cts: __u32,
    pub dsr: __u32,
    pub rng: __u32,
    pub dcd: __u32,
    pub rx: __u32,
    pub tx: __u32,
    pub frame: __u32,
    pub overrun: __u32,
    pub parity: __u32,
    pub brk: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdio_uart_port {
    pub port: tty_port,
    pub index: c_uint,
    pub func: *mut sdio_func,
    pub func_lock: mutex,
    pub in_sdio_uart_irq: *mut task_struct,
    pub regs_offset: c_uint,
    pub xmit_fifo: kfifo,
    pub write_lock: spinlock_t,
    pub icount: uart_icount,
    pub uartclk: c_uint,
    pub mctrl: c_uint,
    pub rx_mctrl: c_uint,
    pub read_status_mask: c_uint,
    pub ignore_status_mask: c_uint,
    pub x_char: c_uchar,
    pub ier: c_uchar,
    pub lcr: c_uchar,
}

    static struct sdio_uart_port *sdio_uart_table[UART_NR];
    static DEFINE_SPINLOCK(sdio_uart_table_lock);
#[no_mangle]
unsafe extern "C" fn sdio_uart_add_port(port: *mut sdio_uart_port) -> c_int {
    static int sdio_uart_add_port(struct sdio_uart_port *port)
    {
    int index, ret = -EBUSY;
    mutex_init(&port.func_lock);
    spin_lock_init(&port.write_lock);
    if (kfifo_alloc(&port.xmit_fifo, FIFO_SIZE, GFP_KERNEL))
    return -ENOMEM;
    spin_lock(&sdio_uart_table_lock);
    for (index = 0; index < UART_NR; index++) {
    if (!sdio_uart_table[index]) {
    port.index = index;
    sdio_uart_table[index] = port;
    ret = 0;
    break;
    }
    }
    spin_unlock(&sdio_uart_table_lock);
    return ret;
    }
    static struct sdio_uart_port *sdio_uart_port_get(unsigned index)
    {
    struct sdio_uart_port *port;
    if (index >= UART_NR)
    return core::ptr::null_mut();
    spin_lock(&sdio_uart_table_lock);
    port = sdio_uart_table[index];
    if (port)
    tty_port_get(&port.port);
    spin_unlock(&sdio_uart_table_lock);
    return port;
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_port_put(port: *mut sdio_uart_port) {
    static void sdio_uart_port_put(struct sdio_uart_port *port)
    {
    tty_port_put(&port.port);
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_port_remove(port: *mut sdio_uart_port) {
    static void sdio_uart_port_remove(struct sdio_uart_port *port)
    {
    struct sdio_func *func;
    spin_lock(&sdio_uart_table_lock);
    sdio_uart_table[port.index] = core::ptr::null_mut();
    spin_unlock(&sdio_uart_table_lock);
//
// We're killing a port that potentially still is in use by
// the tty layer. Be careful to prevent any further access
// to the SDIO function and arrange for the tty layer to
// give up on that port ASAP.
// Beware: the lock ordering is critical.
//
    mutex_lock(&port.port.mutex);
    mutex_lock(&port.func_lock);
    func = port.func;
    sdio_claim_host(func);
    port.func = core::ptr::null_mut();
    mutex_unlock(&port.func_lock);
// tty_hangup is async so is this safe as is ??
    tty_port_tty_hangup(&port.port, false);
    mutex_unlock(&port.port.mutex);
    sdio_release_irq(func);
    sdio_disable_func(func);
    sdio_release_host(func);
    sdio_uart_port_put(port);
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_claim_func(port: *mut sdio_uart_port) -> c_int {
    static int sdio_uart_claim_func(struct sdio_uart_port *port)
    {
    mutex_lock(&port.func_lock);
    if (unlikely(!port.func)) {
    mutex_unlock(&port.func_lock);
    return -ENODEV;
    }
    if (likely(port.in_sdio_uart_irq != current))
    sdio_claim_host(port.func);
    mutex_unlock(&port.func_lock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sdio_uart_release_func(port: *mut sdio_uart_port) {
    static inline void sdio_uart_release_func(struct sdio_uart_port *port)
    {
    if (likely(port.in_sdio_uart_irq != current))
    sdio_release_host(port.func);
    }
#[no_mangle]
pub unsafe extern "C" fn sdio_in(port: *mut sdio_uart_port, offset: c_int) -> u8 {
    static inline u8 sdio_in(struct sdio_uart_port *port, int offset)
    {
    return sdio_readb(port.func, port.regs_offset + offset, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn sdio_out(port: *mut sdio_uart_port, offset: c_int, value: c_int) {
    static inline void sdio_out(struct sdio_uart_port *port, int offset, int value)
    {
    sdio_writeb(port.func, value, port.regs_offset + offset, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_get_mctrl(port: *mut sdio_uart_port) -> c_uint {
    static unsigned int sdio_uart_get_mctrl(struct sdio_uart_port *port)
    {
    unsigned int ret;
    u8 status;
// FIXME: What stops this losing the delta bits and breaking
    sdio_uart_check_modem_status ? */
    status = sdio_in(port, UART_MSR);
    ret = 0;
    if (status & UART_MSR_DCD)
    ret |= TIOCM_CAR;
    if (status & UART_MSR_RI)
    ret |= TIOCM_RNG;
    if (status & UART_MSR_DSR)
    ret |= TIOCM_DSR;
    if (status & UART_MSR_CTS)
    ret |= TIOCM_CTS;
    return ret;
    }
    static void sdio_uart_write_mctrl(struct sdio_uart_port *port,
    unsigned int mctrl)
    {
    let mut mcr: c_uchar = 0;
    if (mctrl & TIOCM_RTS)
    mcr |= UART_MCR_RTS;
    if (mctrl & TIOCM_DTR)
    mcr |= UART_MCR_DTR;
    if (mctrl & TIOCM_OUT1)
    mcr |= UART_MCR_OUT1;
    if (mctrl & TIOCM_OUT2)
    mcr |= UART_MCR_OUT2;
    if (mctrl & TIOCM_LOOP)
    mcr |= UART_MCR_LOOP;
    sdio_out(port, UART_MCR, mcr);
    }
    static inline void sdio_uart_update_mctrl(struct sdio_uart_port *port,
    unsigned int set, unsigned int clear)
    {
    unsigned int old;
    old = port.mctrl;
    port.mctrl = (old & ~clear) | set;
    if (old != port.mctrl)
    sdio_uart_write_mctrl(port, port.mctrl);
    }

    static void sdio_uart_change_speed(struct sdio_uart_port *port,
    struct ktermios *termios,
    const struct ktermios *old)
    {
    unsigned char cval, fcr = 0;
    unsigned int baud, quot;
    cval = UART_LCR_WLEN(tty_get_char_size(termios.c_cflag));
    if (termios.c_cflag & CSTOPB)
    cval |= UART_LCR_STOP;
    if (termios.c_cflag & PARENB)
    cval |= UART_LCR_PARITY;
    if (!(termios.c_cflag & PARODD))
    cval |= UART_LCR_EPAR;
    for (;;) {
    baud = tty_termios_baud_rate(termios);
    if (baud == 0)
    baud = 9600;  /* Special case: B0 rate. */
    if (baud <= port.uartclk)
    break;
//
// Oops, the quotient was zero.  Try again with the old
// baud rate if possible, otherwise default to 9600.
//
    termios.c_cflag &= ~CBAUD;
    if (old) {
    termios.c_cflag |= old.c_cflag & CBAUD;
    old = core::ptr::null_mut();
    } else
    termios.c_cflag |= B9600;
    }
    quot = (2 * port.uartclk + baud) / (2 * baud);
    if (baud < 2400)
    fcr = UART_FCR_ENABLE_FIFO | UART_FCR_TRIGGER_1;
    else
    fcr = UART_FCR_ENABLE_FIFO | UART_FCR_R_TRIG_10;
    port.read_status_mask = UART_LSR_OE | UART_LSR_THRE | UART_LSR_DR;
    if (termios.c_iflag & INPCK)
    port.read_status_mask |= UART_LSR_FE | UART_LSR_PE;
    if (termios.c_iflag & (BRKINT | PARMRK))
    port.read_status_mask |= UART_LSR_BI;
//
// Characters to ignore
//
    port.ignore_status_mask = 0;
    if (termios.c_iflag & IGNPAR)
    port.ignore_status_mask |= UART_LSR_PE | UART_LSR_FE;
    if (termios.c_iflag & IGNBRK) {
    port.ignore_status_mask |= UART_LSR_BI;
//
// If we're ignoring parity and break indicators,
// ignore overruns too (for real raw support).
//
    if (termios.c_iflag & IGNPAR)
    port.ignore_status_mask |= UART_LSR_OE;
    }
//
// ignore all characters if CREAD is not set
//
    if ((termios.c_cflag & CREAD) == 0)
    port.ignore_status_mask |= UART_LSR_DR;
//
// CTS flow control flag and modem status interrupts
//
    port.ier &= ~UART_IER_MSI;
    if ((termios.c_cflag & CRTSCTS) || !(termios.c_cflag & CLOCAL))
    port.ier |= UART_IER_MSI;
    port.lcr = cval;
    sdio_out(port, UART_IER, port.ier);
    sdio_out(port, UART_LCR, cval | UART_LCR_DLAB);
    sdio_out(port, UART_DLL, quot & 0xff);
    sdio_out(port, UART_DLM, quot >> 8);
    sdio_out(port, UART_LCR, cval);
    sdio_out(port, UART_FCR, fcr);
    sdio_uart_write_mctrl(port, port.mctrl);
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_start_tx(port: *mut sdio_uart_port) {
    static void sdio_uart_start_tx(struct sdio_uart_port *port)
    {
    if (!(port.ier & UART_IER_THRI)) {
    port.ier |= UART_IER_THRI;
    sdio_out(port, UART_IER, port.ier);
    }
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_stop_tx(port: *mut sdio_uart_port) {
    static void sdio_uart_stop_tx(struct sdio_uart_port *port)
    {
    if (port.ier & UART_IER_THRI) {
    port.ier &= ~UART_IER_THRI;
    sdio_out(port, UART_IER, port.ier);
    }
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_stop_rx(port: *mut sdio_uart_port) {
    static void sdio_uart_stop_rx(struct sdio_uart_port *port)
    {
    port.ier &= ~UART_IER_RLSI;
    port.read_status_mask &= ~UART_LSR_DR;
    sdio_out(port, UART_IER, port.ier);
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_receive_chars(port: *mut sdio_uart_port, status: *mut u8) {
    static void sdio_uart_receive_chars(struct sdio_uart_port *port, u8 *status)
    {
    let mut max_count: c_int = 256;
    do {
    let mut ch: u8 = sdio_in(port, UART_RX);
    let mut flag: u8 = TTY_NORMAL;
    port.icount.rx++;
    if (unlikely(*status & (UART_LSR_BI | UART_LSR_PE |
    UART_LSR_FE | UART_LSR_OE))) {
//
// For statistics only
//
    if (*status & UART_LSR_BI) {
// status &= ~(UART_LSR_FE | UART_LSR_PE);
    port.icount.brk++;
    } else if (*status & UART_LSR_PE)
    port.icount.parity++;
#[no_mangle]
pub unsafe extern "C" fn if(UART_LSR_FE: *mut *mut status &) -> else {
    else if (*status & UART_LSR_FE)
    port.icount.frame++;
    if (*status & UART_LSR_OE)
    port.icount.overrun++;
//
// Mask off conditions which should be ignored.
//
// status &= port->read_status_mask;
    if (*status & UART_LSR_BI)
    flag = TTY_BREAK;
#[no_mangle]
pub unsafe extern "C" fn if(UART_LSR_PE: *mut *mut status &) -> else {
    else if (*status & UART_LSR_PE)
    flag = TTY_PARITY;
#[no_mangle]
pub unsafe extern "C" fn if(UART_LSR_FE: *mut *mut status &) -> else {
    else if (*status & UART_LSR_FE)
    flag = TTY_FRAME;
    }
    if ((*status & port.ignore_status_mask & ~UART_LSR_OE) == 0)
    tty_insert_flip_char(&port.port, ch, flag);
//
// Overrun is special.  Since it's reported immediately,
// it doesn't affect the current character.
//
    if (*status & ~port.ignore_status_mask & UART_LSR_OE)
    tty_insert_flip_char(&port.port, 0, TTY_OVERRUN);
// status = sdio_in(port, UART_LSR);
    } while ((*status & UART_LSR_DR) && (max_count-- > 0));
    tty_flip_buffer_push(&port.port);
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_transmit_chars(port: *mut sdio_uart_port) {
    static void sdio_uart_transmit_chars(struct sdio_uart_port *port)
    {
    struct kfifo *xmit = &port.xmit_fifo;
    int count;
    struct tty_struct *tty;
    u8 iobuf[16];
    int len;
    if (port.x_char) {
    sdio_out(port, UART_TX, port.x_char);
    port.icount.tx++;
    port.x_char = 0;
    return;
    }
    tty = tty_port_tty_get(&port.port);
    if (tty == core::ptr::null_mut() || !kfifo_len(xmit) ||
    tty.flow.stopped || tty.hw_stopped) {
    sdio_uart_stop_tx(port);
    tty_kref_put(tty);
    return;
    }
    len = kfifo_out_locked(xmit, iobuf, 16, &port.write_lock);
    for (count = 0; count < len; count++) {
    sdio_out(port, UART_TX, iobuf[count]);
    port.icount.tx++;
    }
    len = kfifo_len(xmit);
    if (len < WAKEUP_CHARS) {
    tty_wakeup(tty);
    if (len == 0)
    sdio_uart_stop_tx(port);
    }
    tty_kref_put(tty);
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_check_modem_status(port: *mut sdio_uart_port) {
    static void sdio_uart_check_modem_status(struct sdio_uart_port *port)
    {
    struct tty_struct *tty;
    u8 status;
    status = sdio_in(port, UART_MSR);
    if ((status & UART_MSR_ANY_DELTA) == 0)
    return;
    if (status & UART_MSR_TERI)
    port.icount.rng++;
    if (status & UART_MSR_DDSR)
    port.icount.dsr++;
    if (status & UART_MSR_DDCD) {
    port.icount.dcd++;
// DCD raise - wake for open
    if (status & UART_MSR_DCD)
    wake_up_interruptible(&port.port.open_wait);
    else {
// DCD drop - hang up if tty attached
    tty_port_tty_hangup(&port.port, false);
    }
    }
    if (status & UART_MSR_DCTS) {
    port.icount.cts++;
    tty = tty_port_tty_get(&port.port);
    if (tty && C_CRTSCTS(tty)) {
    let mut cts: bool = status & UART_MSR_CTS;
    if (tty.hw_stopped) {
    if (cts) {
    tty.hw_stopped = false;
    sdio_uart_start_tx(port);
    tty_wakeup(tty);
    }
    } else {
    if (!cts) {
    tty.hw_stopped = true;
    sdio_uart_stop_tx(port);
    }
    }
    }
    tty_kref_put(tty);
    }
    }
//
// This handles the interrupt from one port.
//
#[no_mangle]
unsafe extern "C" fn sdio_uart_irq(func: *mut sdio_func) {
    static void sdio_uart_irq(struct sdio_func *func)
    {
    struct sdio_uart_port *port = sdio_get_drvdata(func);
    u8 iir, lsr;
//
// In a few places sdio_uart_irq() is called directly instead of
// waiting for the actual interrupt to be raised and the SDIO IRQ
// thread scheduled in order to reduce latency.  However, some
// interaction with the tty core may end up calling us back
// (serial echo, flow control, etc.) through those same places
// causing undesirable effects.  Let's stop the recursion here.
//
    if (unlikely(port.in_sdio_uart_irq == current))
    return;
    iir = sdio_in(port, UART_IIR);
    if (iir & UART_IIR_NO_INT)
    return;
    port.in_sdio_uart_irq = current;
    lsr = sdio_in(port, UART_LSR);
    if (lsr & UART_LSR_DR)
    sdio_uart_receive_chars(port, &lsr);
    sdio_uart_check_modem_status(port);
    if (lsr & UART_LSR_THRE)
    sdio_uart_transmit_chars(port);
    port.in_sdio_uart_irq = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn uart_carrier_raised(tport: *mut tty_port) -> bool {
    static bool uart_carrier_raised(struct tty_port *tport)
    {
    struct sdio_uart_port *port =
    container_of(tport, struct sdio_uart_port, port);
    let mut ret: c_uint = sdio_uart_claim_func(port);
    if (ret)	/* Missing hardware shouldn't block for carrier */
    return 1;
    ret = sdio_uart_get_mctrl(port);
    sdio_uart_release_func(port);
    return ret & TIOCM_CAR;
    }
//
// uart_dtr_rts		-	 port helper to set uart signals
// @tport: tty port to be updated
// @active: set to turn on DTR/RTS
//
// Called by the tty port helpers when the modem signals need to be
// adjusted during an open, close and hangup.
//
#[no_mangle]
unsafe extern "C" fn uart_dtr_rts(tport: *mut tty_port, active: bool) {
    static void uart_dtr_rts(struct tty_port *tport, bool active)
    {
    struct sdio_uart_port *port =
    container_of(tport, struct sdio_uart_port, port);
    let mut ret: c_int = sdio_uart_claim_func(port);
    if (ret)
    return;
    if (!active)
    sdio_uart_clear_mctrl(port, TIOCM_DTR | TIOCM_RTS);
    else
    sdio_uart_set_mctrl(port, TIOCM_DTR | TIOCM_RTS);
    sdio_uart_release_func(port);
    }
//
// sdio_uart_activate	-	start up hardware
// @tport: tty port to activate
// @tty: tty bound to this port
//
// Activate a tty port. The port locking guarantees us this will be
// run exactly once per set of opens, and if successful will see the
// shutdown method run exactly once to match. Start up and shutdown are
// protected from each other by the internal locking and will not run
// at the same time even during a hangup event.
//
// If we successfully start up the port we take an extra kref as we
// will keep it around until shutdown when the kref is dropped.
//
#[no_mangle]
unsafe extern "C" fn sdio_uart_activate(tport: *mut tty_port, tty: *mut tty_struct) -> c_int {
    static int sdio_uart_activate(struct tty_port *tport, struct tty_struct *tty)
    {
    struct sdio_uart_port *port =
    container_of(tport, struct sdio_uart_port, port);
    int ret;
//
// Set the TTY IO error marker - we will only clear this
// once we have successfully opened the port.
//
    set_bit(TTY_IO_ERROR, &tty.flags);
    kfifo_reset(&port.xmit_fifo);
    ret = sdio_uart_claim_func(port);
    if (ret)
    return ret;
    ret = sdio_enable_func(port.func);
    if (ret)
    goto err1;
    ret = sdio_claim_irq(port.func, sdio_uart_irq);
    if (ret)
    goto err2;
//
// Clear the FIFO buffers and disable them.
// (they will be reenabled in sdio_change_speed())
//
    sdio_out(port, UART_FCR, UART_FCR_ENABLE_FIFO);
    sdio_out(port, UART_FCR, UART_FCR_ENABLE_FIFO |
    UART_FCR_CLEAR_RCVR | UART_FCR_CLEAR_XMIT);
    sdio_out(port, UART_FCR, 0);
//
// Clear the interrupt registers.
//
    (void) sdio_in(port, UART_LSR);
    (void) sdio_in(port, UART_RX);
    (void) sdio_in(port, UART_IIR);
    (void) sdio_in(port, UART_MSR);
//
// Now, initialize the UART
//
    sdio_out(port, UART_LCR, UART_LCR_WLEN8);
    port.ier = UART_IER_RLSI|UART_IER_RDI|UART_IER_RTOIE|UART_IER_UUE;
    port.mctrl = TIOCM_OUT2;
    sdio_uart_change_speed(port, &tty.termios, core::ptr::null_mut());
    if (C_BAUD(tty))
    sdio_uart_set_mctrl(port, TIOCM_RTS | TIOCM_DTR);
    if (C_CRTSCTS(tty))
    if (!(sdio_uart_get_mctrl(port) & TIOCM_CTS))
    tty.hw_stopped = true;
    clear_bit(TTY_IO_ERROR, &tty.flags);
// Kick the IRQ handler once while we're still holding the host lock
    sdio_uart_irq(port.func);
    sdio_uart_release_func(port);
    return 0;
    err2:
    sdio_disable_func(port.func);
    err1:
    sdio_uart_release_func(port);
    return ret;
    }
//
// sdio_uart_shutdown	-	stop hardware
// @tport: tty port to shut down
//
// Deactivate a tty port. The port locking guarantees us this will be
// run only if a successful matching activate already ran. The two are
// protected from each other by the internal locking and will not run
// at the same time even during a hangup event.
//
#[no_mangle]
unsafe extern "C" fn sdio_uart_shutdown(tport: *mut tty_port) {
    static void sdio_uart_shutdown(struct tty_port *tport)
    {
    struct sdio_uart_port *port =
    container_of(tport, struct sdio_uart_port, port);
    int ret;
    ret = sdio_uart_claim_func(port);
    if (ret)
    return;
    sdio_uart_stop_rx(port);
// Disable interrupts from this port
    sdio_release_irq(port.func);
    port.ier = 0;
    sdio_out(port, UART_IER, 0);
    sdio_uart_clear_mctrl(port, TIOCM_OUT2);
// Disable break condition and FIFOs.
    port.lcr &= ~UART_LCR_SBC;
    sdio_out(port, UART_LCR, port.lcr);
    sdio_out(port, UART_FCR, UART_FCR_ENABLE_FIFO |
    UART_FCR_CLEAR_RCVR |
    UART_FCR_CLEAR_XMIT);
    sdio_out(port, UART_FCR, 0);
    sdio_disable_func(port.func);
    sdio_uart_release_func(port);
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_port_destroy(tport: *mut tty_port) {
    static void sdio_uart_port_destroy(struct tty_port *tport)
    {
    struct sdio_uart_port *port =
    container_of(tport, struct sdio_uart_port, port);
    kfifo_free(&port.xmit_fifo);
    kfree(port);
    }
//
// sdio_uart_install	-	install method
// @driver: the driver in use (sdio_uart in our case)
// @tty: the tty being bound
//
// Look up and bind the tty and the driver together. Initialize
// any needed private data (in our case the termios)
//
#[no_mangle]
unsafe extern "C" fn sdio_uart_install(driver: *mut tty_driver, tty: *mut tty_struct) -> c_int {
    static int sdio_uart_install(struct tty_driver *driver, struct tty_struct *tty)
    {
    let mut idx: c_int = tty.index;
    struct sdio_uart_port *port = sdio_uart_port_get(idx);
    let mut ret: c_int = tty_standard_install(driver, tty);
    if (ret == 0)
// This is the ref sdio_uart_port get provided
    tty.driver_data = port;
    else
    sdio_uart_port_put(port);
    return ret;
    }
//
// sdio_uart_cleanup	-	called on the last tty kref drop
// @tty: the tty being destroyed
//
// Called asynchronously when the last reference to the tty is dropped.
// We cannot destroy the tty->driver_data port kref until this point
//
#[no_mangle]
unsafe extern "C" fn sdio_uart_cleanup(tty: *mut tty_struct) {
    static void sdio_uart_cleanup(struct tty_struct *tty)
    {
    struct sdio_uart_port *port = tty.driver_data;
    tty.driver_data = core::ptr::null_mut();	/* Bug trap */
    sdio_uart_port_put(port);
    }
//
// Open/close/hangup is now entirely boilerplate
//
#[no_mangle]
unsafe extern "C" fn sdio_uart_open(tty: *mut tty_struct, filp: *mut file) -> c_int {
    static int sdio_uart_open(struct tty_struct *tty, struct file *filp)
    {
    struct sdio_uart_port *port = tty.driver_data;
    return tty_port_open(&port.port, tty, filp);
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_close(tty: *mut tty_struct, filp: *mut *mut file) {
    static void sdio_uart_close(struct tty_struct *tty, struct file * filp)
    {
    struct sdio_uart_port *port = tty.driver_data;
    tty_port_close(&port.port, tty, filp);
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_hangup(tty: *mut tty_struct) {
    static void sdio_uart_hangup(struct tty_struct *tty)
    {
    struct sdio_uart_port *port = tty.driver_data;
    tty_port_hangup(&port.port);
    }
    static ssize_t sdio_uart_write(struct tty_struct *tty, const u8 *buf,
    size_t count)
    {
    struct sdio_uart_port *port = tty.driver_data;
    int ret;
    if (!port.func)
    return -ENODEV;
    ret = kfifo_in_locked(&port.xmit_fifo, buf, count, &port.write_lock);
    if (!(port.ier & UART_IER_THRI)) {
    let mut err: c_int = sdio_uart_claim_func(port);
    if (!err) {
    sdio_uart_start_tx(port);
    sdio_uart_irq(port.func);
    sdio_uart_release_func(port);
    } else
    ret = err;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_write_room(tty: *mut tty_struct) -> c_uint {
    static unsigned int sdio_uart_write_room(struct tty_struct *tty)
    {
    struct sdio_uart_port *port = tty.driver_data;
    return FIFO_SIZE - kfifo_len(&port.xmit_fifo);
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_chars_in_buffer(tty: *mut tty_struct) -> c_uint {
    static unsigned int sdio_uart_chars_in_buffer(struct tty_struct *tty)
    {
    struct sdio_uart_port *port = tty.driver_data;
    return kfifo_len(&port.xmit_fifo);
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_send_xchar(tty: *mut tty_struct, ch: u8) {
    static void sdio_uart_send_xchar(struct tty_struct *tty, u8 ch)
    {
    struct sdio_uart_port *port = tty.driver_data;
    port.x_char = ch;
    if (ch && !(port.ier & UART_IER_THRI)) {
    if (sdio_uart_claim_func(port) != 0)
    return;
    sdio_uart_start_tx(port);
    sdio_uart_irq(port.func);
    sdio_uart_release_func(port);
    }
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_throttle(tty: *mut tty_struct) {
    static void sdio_uart_throttle(struct tty_struct *tty)
    {
    struct sdio_uart_port *port = tty.driver_data;
    if (!I_IXOFF(tty) && !C_CRTSCTS(tty))
    return;
    if (sdio_uart_claim_func(port) != 0)
    return;
    if (I_IXOFF(tty)) {
    port.x_char = STOP_CHAR(tty);
    sdio_uart_start_tx(port);
    }
    if (C_CRTSCTS(tty))
    sdio_uart_clear_mctrl(port, TIOCM_RTS);
    sdio_uart_irq(port.func);
    sdio_uart_release_func(port);
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_unthrottle(tty: *mut tty_struct) {
    static void sdio_uart_unthrottle(struct tty_struct *tty)
    {
    struct sdio_uart_port *port = tty.driver_data;
    if (!I_IXOFF(tty) && !C_CRTSCTS(tty))
    return;
    if (sdio_uart_claim_func(port) != 0)
    return;
    if (I_IXOFF(tty)) {
    if (port.x_char) {
    port.x_char = 0;
    } else {
    port.x_char = START_CHAR(tty);
    sdio_uart_start_tx(port);
    }
    }
    if (C_CRTSCTS(tty))
    sdio_uart_set_mctrl(port, TIOCM_RTS);
    sdio_uart_irq(port.func);
    sdio_uart_release_func(port);
    }
    static void sdio_uart_set_termios(struct tty_struct *tty,
    const struct ktermios *old_termios)
    {
    struct sdio_uart_port *port = tty.driver_data;
    let mut cflag: c_uint = tty.termios.c_cflag;
    if (sdio_uart_claim_func(port) != 0)
    return;
    sdio_uart_change_speed(port, &tty.termios, old_termios);
// Handle transition to B0 status
    if ((old_termios.c_cflag & CBAUD) && !(cflag & CBAUD))
    sdio_uart_clear_mctrl(port, TIOCM_RTS | TIOCM_DTR);
// Handle transition away from B0 status
    if (!(old_termios.c_cflag & CBAUD) && (cflag & CBAUD)) {
    let mut mask: c_uint = TIOCM_DTR;
    if (!(cflag & CRTSCTS) || !tty_throttled(tty))
    mask |= TIOCM_RTS;
    sdio_uart_set_mctrl(port, mask);
    }
// Handle turning off CRTSCTS
    if ((old_termios.c_cflag & CRTSCTS) && !(cflag & CRTSCTS)) {
    tty.hw_stopped = false;
    sdio_uart_start_tx(port);
    }
// Handle turning on CRTSCTS
    if (!(old_termios.c_cflag & CRTSCTS) && (cflag & CRTSCTS)) {
    if (!(sdio_uart_get_mctrl(port) & TIOCM_CTS)) {
    tty.hw_stopped = true;
    sdio_uart_stop_tx(port);
    }
    }
    sdio_uart_release_func(port);
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_break_ctl(tty: *mut tty_struct, break_state: c_int) -> c_int {
    static int sdio_uart_break_ctl(struct tty_struct *tty, int break_state)
    {
    struct sdio_uart_port *port = tty.driver_data;
    int result;
    result = sdio_uart_claim_func(port);
    if (result != 0)
    return result;
    if (break_state == -1)
    port.lcr |= UART_LCR_SBC;
    else
    port.lcr &= ~UART_LCR_SBC;
    sdio_out(port, UART_LCR, port.lcr);
    sdio_uart_release_func(port);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_tiocmget(tty: *mut tty_struct) -> c_int {
    static int sdio_uart_tiocmget(struct tty_struct *tty)
    {
    struct sdio_uart_port *port = tty.driver_data;
    int result;
    result = sdio_uart_claim_func(port);
    if (!result) {
    result = port.mctrl | sdio_uart_get_mctrl(port);
    sdio_uart_release_func(port);
    }
    return result;
    }
    static int sdio_uart_tiocmset(struct tty_struct *tty,
    unsigned int set, unsigned int clear)
    {
    struct sdio_uart_port *port = tty.driver_data;
    int result;
    result = sdio_uart_claim_func(port);
    if (!result) {
    sdio_uart_update_mctrl(port, set, clear);
    sdio_uart_release_func(port);
    }
    return result;
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_proc_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int sdio_uart_proc_show(struct seq_file *m, void *v)
    {
    int i;
    seq_printf(m, "serinfo:1.0 driver%s%s revision:%s\n",
    "", "", "");
    for (i = 0; i < UART_NR; i++) {
    struct sdio_uart_port *port = sdio_uart_port_get(i);
    if (port) {
    seq_printf(m, "%d: uart:SDIO", i);
    if (capable(CAP_SYS_ADMIN)) {
    seq_printf(m, " tx:%d rx:%d",
    port.icount.tx, port.icount.rx);
    if (port.icount.frame)
    seq_printf(m, " fe:%d",
    port.icount.frame);
    if (port.icount.parity)
    seq_printf(m, " pe:%d",
    port.icount.parity);
    if (port.icount.brk)
    seq_printf(m, " brk:%d",
    port.icount.brk);
    if (port.icount.overrun)
    seq_printf(m, " oe:%d",
    port.icount.overrun);
    if (port.icount.cts)
    seq_printf(m, " cts:%d",
    port.icount.cts);
    if (port.icount.dsr)
    seq_printf(m, " dsr:%d",
    port.icount.dsr);
    if (port.icount.rng)
    seq_printf(m, " rng:%d",
    port.icount.rng);
    if (port.icount.dcd)
    seq_printf(m, " dcd:%d",
    port.icount.dcd);
    }
    sdio_uart_port_put(port);
    seq_putc(m, '\n');
    }
    }
    return 0;
    }
    static const struct tty_port_operations sdio_uart_port_ops = {
    .dtr_rts = uart_dtr_rts,
    .carrier_raised = uart_carrier_raised,
    .shutdown = sdio_uart_shutdown,
    .activate = sdio_uart_activate,
    .destruct = sdio_uart_port_destroy,
    };
    static const struct tty_operations sdio_uart_ops = {
    .open			= sdio_uart_open,
    .close			= sdio_uart_close,
    .write			= sdio_uart_write,
    .write_room		= sdio_uart_write_room,
    .chars_in_buffer	= sdio_uart_chars_in_buffer,
    .send_xchar		= sdio_uart_send_xchar,
    .throttle		= sdio_uart_throttle,
    .unthrottle		= sdio_uart_unthrottle,
    .set_termios		= sdio_uart_set_termios,
    .hangup			= sdio_uart_hangup,
    .break_ctl		= sdio_uart_break_ctl,
    .tiocmget		= sdio_uart_tiocmget,
    .tiocmset		= sdio_uart_tiocmset,
    .install		= sdio_uart_install,
    .cleanup		= sdio_uart_cleanup,
    .proc_show		= sdio_uart_proc_show,
    };
    static struct tty_driver *sdio_uart_tty_driver;
    static int sdio_uart_probe(struct sdio_func *func,
    const struct sdio_device_id *id)
    {
    struct sdio_uart_port *port;
    int ret;
    port = kzalloc_obj(struct sdio_uart_port);
    if (!port)
    return -ENOMEM;
    if (func.class == SDIO_CLASS_UART) {
    pr_warn("%s: need info on UART class basic setup\n",
    sdio_func_id(func));
    kfree(port);
    return -ENOSYS;
    } else if (func.class == SDIO_CLASS_GPS) {
//
// We need tuple 0x91.  It contains SUBTPL_SIOREG
// and SUBTPL_RCVCAPS.
//
    struct sdio_func_tuple *tpl;
    for (tpl = func.tuples; tpl; tpl = tpl.next) {
    if (tpl.code != 0x91)
    continue;
    if (tpl.size < 10)
    continue;
    if (tpl.data[1] == 0)  /* SUBTPL_SIOREG */
    break;
    }
    if (!tpl) {
    pr_warn("%s: can't find tuple 0x91 subtuple 0 (SUBTPL_SIOREG) for GPS class\n",
    sdio_func_id(func));
    kfree(port);
    return -EINVAL;
    }
    pr_debug("%s: Register ID = 0x%02x, Exp ID = 0x%02x\n",
    sdio_func_id(func), tpl.data[2], tpl.data[3]);
    port.regs_offset = (tpl.data[4] << 0) |
    (tpl.data[5] << 8) |
    (tpl.data[6] << 16);
    pr_debug("%s: regs offset = 0x%x\n",
    sdio_func_id(func), port.regs_offset);
    port.uartclk = tpl.data[7] * 115200;
    if (port.uartclk == 0)
    port.uartclk = 115200;
    pr_debug("%s: clk %d baudcode %u 4800-div %u\n",
    sdio_func_id(func), port.uartclk,
    tpl.data[7], tpl.data[8] | (tpl.data[9] << 8));
    } else {
    kfree(port);
    return -EINVAL;
    }
    port.func = func;
    sdio_set_drvdata(func, port);
    tty_port_init(&port.port);
    port.port.ops = &sdio_uart_port_ops;
    ret = sdio_uart_add_port(port);
    if (ret) {
    kfree(port);
    } else {
    struct device *dev;
    dev = tty_port_register_device(&port.port,
    sdio_uart_tty_driver, port.index, &func.dev);
    if (IS_ERR(dev)) {
    sdio_uart_port_remove(port);
    ret = PTR_ERR(dev);
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_remove(func: *mut sdio_func) {
    static void sdio_uart_remove(struct sdio_func *func)
    {
    struct sdio_uart_port *port = sdio_get_drvdata(func);
    tty_unregister_device(sdio_uart_tty_driver, port.index);
    sdio_uart_port_remove(port);
    }
    static const struct sdio_device_id sdio_uart_ids[] = {
    { SDIO_DEVICE_CLASS(SDIO_CLASS_UART)		},
    { SDIO_DEVICE_CLASS(SDIO_CLASS_GPS)		},
    { /* end: all zeroes */				},
    };
    MODULE_DEVICE_TABLE(sdio, sdio_uart_ids);
    static struct sdio_driver sdio_uart_driver = {
    .probe		= sdio_uart_probe,
    .remove		= sdio_uart_remove,
    .name		= "sdio_uart",
    .id_table	= sdio_uart_ids,
    };
#[no_mangle]
unsafe extern "C" fn sdio_uart_init() -> int __init {
    static int __init sdio_uart_init(void)
    {
    int ret;
    struct tty_driver *tty_drv;
    sdio_uart_tty_driver = tty_drv = tty_alloc_driver(UART_NR,
    TTY_DRIVER_REAL_RAW | TTY_DRIVER_DYNAMIC_DEV);
    if (IS_ERR(tty_drv))
    return PTR_ERR(tty_drv);
    tty_drv.driver_name = "sdio_uart";
    tty_drv.name =   "ttySDIO";
    tty_drv.major = 0;  /* dynamically allocated */
    tty_drv.minor_start = 0;
    tty_drv.type = TTY_DRIVER_TYPE_SERIAL;
    tty_drv.subtype = SERIAL_TYPE_NORMAL;
    tty_drv.init_termios = tty_std_termios;
    tty_drv.init_termios.c_cflag = B4800 | CS8 | CREAD | HUPCL | CLOCAL;
    tty_drv.init_termios.c_ispeed = 4800;
    tty_drv.init_termios.c_ospeed = 4800;
    tty_set_operations(tty_drv, &sdio_uart_ops);
    ret = tty_register_driver(tty_drv);
    if (ret)
    goto err1;
    ret = sdio_register_driver(&sdio_uart_driver);
    if (ret)
    goto err2;
    return 0;
    err2:
    tty_unregister_driver(tty_drv);
    err1:
    tty_driver_kref_put(tty_drv);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sdio_uart_exit() -> void __exit {
    static void __exit sdio_uart_exit(void)
    {
    sdio_unregister_driver(&sdio_uart_driver);
    tty_unregister_driver(sdio_uart_tty_driver);
    tty_driver_kref_put(sdio_uart_tty_driver);
    }
    module_init(sdio_uart_init);
    module_exit(sdio_uart_exit);
    MODULE_AUTHOR("Nicolas Pitre");
    MODULE_DESCRIPTION("SDIO UART/GPS driver");
    MODULE_LICENSE("GPL");

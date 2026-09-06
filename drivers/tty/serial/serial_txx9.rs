//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/serial_txx9.c
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
// Derived from many drivers using generic_serial interface,
// especially serial_tx3912.c by Steven J. Hill and r39xx_serial.c
// (was in Linux/VR tree) by Jim Pick.
//
// Copyright (C) 1999 Harald Koerfgen
// Copyright (C) 2000 Jim Pick <jim@jimpick.com>
// Copyright (C) 2001 Steven J. Hill (sjhill@realitydiluted.com)
// Copyright (C) 2000-2002 Toshiba Corporation
//
// Serial driver for TX3927/TX4927/TX4925/TX4938 internal SIO controller
//

pub const PASS_LIMIT: c_int = 256;

// "ttyS" is used for standard serial driver

pub const TXX9_TTY_MINOR_START: c_int = 196;
pub const TXX9_TTY_MAJOR: c_int = 204;

// acts like standard serial driver

pub const TXX9_TTY_MINOR_START: c_int = 64;

// flag aliases

// support for Toshiba TC86C001 SIO
// Macro flag: #define ENABLE_SERIAL_TXX9_PCI

//
// Number of serial ports
//

pub const TXX9_REGION_SIZE: c_uint = 0x24;
// TXX9 Serial Registers
pub const TXX9_SILCR: c_uint = 0x00;
pub const TXX9_SIDICR: c_uint = 0x04;
pub const TXX9_SIDISR: c_uint = 0x08;
pub const TXX9_SICISR: c_uint = 0x0c;
pub const TXX9_SIFCR: c_uint = 0x10;
pub const TXX9_SIFLCR: c_uint = 0x14;
pub const TXX9_SIBGR: c_uint = 0x18;
pub const TXX9_SITFIFO: c_uint = 0x1c;
pub const TXX9_SIRFIFO: c_uint = 0x20;
// SILCR : Line Control
pub const TXX9_SILCR_SCS_MASK: c_uint = 0x00000060;
pub const TXX9_SILCR_SCS_IMCLK: c_uint = 0x00000000;
pub const TXX9_SILCR_SCS_IMCLK_BG: c_uint = 0x00000020;
pub const TXX9_SILCR_SCS_SCLK: c_uint = 0x00000040;
pub const TXX9_SILCR_SCS_SCLK_BG: c_uint = 0x00000060;
pub const TXX9_SILCR_UEPS: c_uint = 0x00000010;
pub const TXX9_SILCR_UPEN: c_uint = 0x00000008;
pub const TXX9_SILCR_USBL_MASK: c_uint = 0x00000004;
pub const TXX9_SILCR_USBL_1BIT: c_uint = 0x00000000;
pub const TXX9_SILCR_USBL_2BIT: c_uint = 0x00000004;
pub const TXX9_SILCR_UMODE_MASK: c_uint = 0x00000003;
pub const TXX9_SILCR_UMODE_8BIT: c_uint = 0x00000000;
pub const TXX9_SILCR_UMODE_7BIT: c_uint = 0x00000001;
// SIDICR : DMA/Int. Control
pub const TXX9_SIDICR_TDE: c_uint = 0x00008000;
pub const TXX9_SIDICR_RDE: c_uint = 0x00004000;
pub const TXX9_SIDICR_TIE: c_uint = 0x00002000;
pub const TXX9_SIDICR_RIE: c_uint = 0x00001000;
pub const TXX9_SIDICR_SPIE: c_uint = 0x00000800;
pub const TXX9_SIDICR_CTSAC: c_uint = 0x00000600;
pub const TXX9_SIDICR_STIE_MASK: c_uint = 0x0000003f;
pub const TXX9_SIDICR_STIE_OERS: c_uint = 0x00000020;
pub const TXX9_SIDICR_STIE_CTSS: c_uint = 0x00000010;
pub const TXX9_SIDICR_STIE_RBRKD: c_uint = 0x00000008;
pub const TXX9_SIDICR_STIE_TRDY: c_uint = 0x00000004;
pub const TXX9_SIDICR_STIE_TXALS: c_uint = 0x00000002;
pub const TXX9_SIDICR_STIE_UBRKD: c_uint = 0x00000001;
// SIDISR : DMA/Int. Status
pub const TXX9_SIDISR_UBRK: c_uint = 0x00008000;
pub const TXX9_SIDISR_UVALID: c_uint = 0x00004000;
pub const TXX9_SIDISR_UFER: c_uint = 0x00002000;
pub const TXX9_SIDISR_UPER: c_uint = 0x00001000;
pub const TXX9_SIDISR_UOER: c_uint = 0x00000800;
pub const TXX9_SIDISR_ERI: c_uint = 0x00000400;
pub const TXX9_SIDISR_TOUT: c_uint = 0x00000200;
pub const TXX9_SIDISR_TDIS: c_uint = 0x00000100;
pub const TXX9_SIDISR_RDIS: c_uint = 0x00000080;
pub const TXX9_SIDISR_STIS: c_uint = 0x00000040;
pub const TXX9_SIDISR_RFDN_MASK: c_uint = 0x0000001f;
// SICISR : Change Int. Status
pub const TXX9_SICISR_OERS: c_uint = 0x00000020;
pub const TXX9_SICISR_CTSS: c_uint = 0x00000010;
pub const TXX9_SICISR_RBRKD: c_uint = 0x00000008;
pub const TXX9_SICISR_TRDY: c_uint = 0x00000004;
pub const TXX9_SICISR_TXALS: c_uint = 0x00000002;
pub const TXX9_SICISR_UBRKD: c_uint = 0x00000001;
// SIFCR : FIFO Control
pub const TXX9_SIFCR_SWRST: c_uint = 0x00008000;
pub const TXX9_SIFCR_RDIL_MASK: c_uint = 0x00000180;
pub const TXX9_SIFCR_RDIL_1: c_uint = 0x00000000;
pub const TXX9_SIFCR_RDIL_4: c_uint = 0x00000080;
pub const TXX9_SIFCR_RDIL_8: c_uint = 0x00000100;
pub const TXX9_SIFCR_RDIL_12: c_uint = 0x00000180;
pub const TXX9_SIFCR_RDIL_MAX: c_uint = 0x00000180;
pub const TXX9_SIFCR_TDIL_MASK: c_uint = 0x00000018;
pub const TXX9_SIFCR_TDIL_1: c_uint = 0x00000000;
pub const TXX9_SIFCR_TDIL_4: c_uint = 0x00000001;
pub const TXX9_SIFCR_TDIL_8: c_uint = 0x00000010;
pub const TXX9_SIFCR_TDIL_MAX: c_uint = 0x00000010;
pub const TXX9_SIFCR_TFRST: c_uint = 0x00000004;
pub const TXX9_SIFCR_RFRST: c_uint = 0x00000002;
pub const TXX9_SIFCR_FRSTE: c_uint = 0x00000001;
pub const TXX9_SIO_TX_FIFO: c_int = 8;
pub const TXX9_SIO_RX_FIFO: c_int = 16;
// SIFLCR : Flow Control
pub const TXX9_SIFLCR_RCS: c_uint = 0x00001000;
pub const TXX9_SIFLCR_TES: c_uint = 0x00000800;
pub const TXX9_SIFLCR_RTSSC: c_uint = 0x00000200;
pub const TXX9_SIFLCR_RSDE: c_uint = 0x00000100;
pub const TXX9_SIFLCR_TSDE: c_uint = 0x00000080;
pub const TXX9_SIFLCR_RTSTL_MASK: c_uint = 0x0000001e;
pub const TXX9_SIFLCR_RTSTL_MAX: c_uint = 0x0000001e;
pub const TXX9_SIFLCR_TBRK: c_uint = 0x00000001;
// SIBGR : Baudrate Control
pub const TXX9_SIBGR_BCLK_MASK: c_uint = 0x00000300;
pub const TXX9_SIBGR_BCLK_T0: c_uint = 0x00000000;
pub const TXX9_SIBGR_BCLK_T2: c_uint = 0x00000100;
pub const TXX9_SIBGR_BCLK_T4: c_uint = 0x00000200;
pub const TXX9_SIBGR_BCLK_T6: c_uint = 0x00000300;
pub const TXX9_SIBGR_BRD_MASK: c_uint = 0x000000ff;
#[no_mangle]
pub unsafe extern "C" fn sio_in(up: *mut uart_port, offset: c_int) -> c_uint {
    static inline unsigned int sio_in(struct uart_port *up, int offset)
    {
    switch (up.iotype) {
    default:
    return __raw_readl(up.membase + offset);
    case UPIO_PORT:
    return inl(up.iobase + offset);
    }
    }
    static inline void
    sio_out(struct uart_port *up, int offset, int value)
    {
    switch (up.iotype) {
    default:
    __raw_writel(value, up.membase + offset);
    break;
    case UPIO_PORT:
    outl(value, up.iobase + offset);
    break;
    }
    }
    static inline void
    sio_mask(struct uart_port *up, int offset, unsigned int value)
    {
    sio_out(up, offset, sio_in(up, offset) & ~value);
    }
    static inline void
    sio_set(struct uart_port *up, int offset, unsigned int value)
    {
    sio_out(up, offset, sio_in(up, offset) | value);
    }
    static inline void
    sio_quot_set(struct uart_port *up, int quot)
    {
    quot >>= 1;
    if (quot < 256)
    sio_out(up, TXX9_SIBGR, quot | TXX9_SIBGR_BCLK_T0);
#[no_mangle]
pub unsafe extern "C" fn if(2): quot < (256 <<) -> else {
    else if (quot < (256 << 2))
    sio_out(up, TXX9_SIBGR, (quot >> 2) | TXX9_SIBGR_BCLK_T2);
#[no_mangle]
pub unsafe extern "C" fn if(4): quot < (256 <<) -> else {
    else if (quot < (256 << 4))
    sio_out(up, TXX9_SIBGR, (quot >> 4) | TXX9_SIBGR_BCLK_T4);
#[no_mangle]
pub unsafe extern "C" fn if(6): quot < (256 <<) -> else {
    else if (quot < (256 << 6))
    sio_out(up, TXX9_SIBGR, (quot >> 6) | TXX9_SIBGR_BCLK_T6);
    else
    sio_out(up, TXX9_SIBGR, 0xff | TXX9_SIBGR_BCLK_T6);
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_stop_tx(up: *mut uart_port) {
    static void serial_txx9_stop_tx(struct uart_port *up)
    {
    sio_mask(up, TXX9_SIDICR, TXX9_SIDICR_TIE);
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_start_tx(up: *mut uart_port) {
    static void serial_txx9_start_tx(struct uart_port *up)
    {
    sio_set(up, TXX9_SIDICR, TXX9_SIDICR_TIE);
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_stop_rx(up: *mut uart_port) {
    static void serial_txx9_stop_rx(struct uart_port *up)
    {
    up.read_status_mask &= ~TXX9_SIDISR_RDIS;
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_initialize(up: *mut uart_port) {
    static void serial_txx9_initialize(struct uart_port *up)
    {
    let mut tmout: c_uint = 10000;
    sio_out(up, TXX9_SIFCR, TXX9_SIFCR_SWRST);
// TX4925 BUG WORKAROUND.  Accessing SIOC register
// immediately after soft reset causes bus error.
    udelay(1);
    while ((sio_in(up, TXX9_SIFCR) & TXX9_SIFCR_SWRST) && --tmout)
    udelay(1);
// TX Int by FIFO Empty, RX Int by Receiving 1 char.
    sio_set(up, TXX9_SIFCR,
    TXX9_SIFCR_TDIL_MAX | TXX9_SIFCR_RDIL_1);
// initial settings
    sio_out(up, TXX9_SILCR,
    TXX9_SILCR_UMODE_8BIT | TXX9_SILCR_USBL_1BIT |
    ((up.flags & UPF_TXX9_USE_SCLK) ?
    TXX9_SILCR_SCS_SCLK_BG : TXX9_SILCR_SCS_IMCLK_BG));
    sio_quot_set(up, uart_get_divisor(up, 9600));
    sio_out(up, TXX9_SIFLCR, TXX9_SIFLCR_RTSTL_MAX /* 15 */);
    sio_out(up, TXX9_SIDICR, 0);
    }
    static inline void
    receive_chars(struct uart_port *up, unsigned int *status)
    {
    let mut disr: c_uint = *status;
    let mut max_count: c_int = 256;
    unsigned int next_ignore_status_mask;
    u8 ch, flag;
    do {
    ch = sio_in(up, TXX9_SIRFIFO);
    flag = TTY_NORMAL;
    up.icount.rx++;
// mask out RFDN_MASK bit added by previous overrun
    next_ignore_status_mask =
    up.ignore_status_mask & ~TXX9_SIDISR_RFDN_MASK;
    if (unlikely(disr & (TXX9_SIDISR_UBRK | TXX9_SIDISR_UPER |
    TXX9_SIDISR_UFER | TXX9_SIDISR_UOER))) {
//
// For statistics only
//
    if (disr & TXX9_SIDISR_UBRK) {
    disr &= ~(TXX9_SIDISR_UFER | TXX9_SIDISR_UPER);
    up.icount.brk++;
//
// We do the SysRQ and SAK checking
// here because otherwise the break
// may get masked by ignore_status_mask
// or read_status_mask.
//
    if (uart_handle_break(up))
    goto ignore_char;
    } else if (disr & TXX9_SIDISR_UPER)
    up.icount.parity++;
#[no_mangle]
pub unsafe extern "C" fn if(TXX9_SIDISR_UFER: disr &) -> else {
    else if (disr & TXX9_SIDISR_UFER)
    up.icount.frame++;
    if (disr & TXX9_SIDISR_UOER) {
    up.icount.overrun++;
//
// The receiver read buffer still hold
// a char which caused overrun.
// Ignore next char by adding RFDN_MASK
// to ignore_status_mask temporarily.
//
    next_ignore_status_mask |=
    TXX9_SIDISR_RFDN_MASK;
    }
//
// Mask off conditions which should be ingored.
//
    disr &= up.read_status_mask;
    if (disr & TXX9_SIDISR_UBRK) {
    flag = TTY_BREAK;
    } else if (disr & TXX9_SIDISR_UPER)
    flag = TTY_PARITY;
#[no_mangle]
pub unsafe extern "C" fn if(TXX9_SIDISR_UFER: disr &) -> else {
    else if (disr & TXX9_SIDISR_UFER)
    flag = TTY_FRAME;
    }
    if (uart_handle_sysrq_char(up, ch))
    goto ignore_char;
    uart_insert_char(up, disr, TXX9_SIDISR_UOER, ch, flag);
    ignore_char:
    up.ignore_status_mask = next_ignore_status_mask;
    disr = sio_in(up, TXX9_SIDISR);
    } while (!(disr & TXX9_SIDISR_UVALID) && (max_count-- > 0));
    tty_flip_buffer_push(&up.state.port);
// status = disr;
    }
#[no_mangle]
pub unsafe extern "C" fn transmit_chars(up: *mut uart_port) {
    static inline void transmit_chars(struct uart_port *up)
    {
    u8 ch;
    uart_port_tx_limited(up, ch, TXX9_SIO_TX_FIFO,
    true,
    sio_out(up, TXX9_SITFIFO, ch),
    ({}));
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t serial_txx9_interrupt(int irq, void *dev_id)
    {
    let mut pass_counter: c_int = 0;
    struct uart_port *up = dev_id;
    unsigned int status;
    while (1) {
    uart_port_lock(up);
    status = sio_in(up, TXX9_SIDISR);
    if (!(sio_in(up, TXX9_SIDICR) & TXX9_SIDICR_TIE))
    status &= ~TXX9_SIDISR_TDIS;
    if (!(status & (TXX9_SIDISR_TDIS | TXX9_SIDISR_RDIS |
    TXX9_SIDISR_TOUT))) {
    uart_port_unlock(up);
    break;
    }
    if (status & TXX9_SIDISR_RDIS)
    receive_chars(up, &status);
    if (status & TXX9_SIDISR_TDIS)
    transmit_chars(up);
// Clear TX/RX Int. Status
    sio_mask(up, TXX9_SIDISR,
    TXX9_SIDISR_TDIS | TXX9_SIDISR_RDIS |
    TXX9_SIDISR_TOUT);
    uart_port_unlock(up);
    if (pass_counter++ > PASS_LIMIT)
    break;
    }
    return pass_counter ? IRQ_HANDLED : IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_tx_empty(up: *mut uart_port) -> c_uint {
    static unsigned int serial_txx9_tx_empty(struct uart_port *up)
    {
    unsigned long flags;
    unsigned int ret;
    uart_port_lock_irqsave(up, &flags);
    ret = (sio_in(up, TXX9_SICISR) & TXX9_SICISR_TXALS) ? TIOCSER_TEMT : 0;
    uart_port_unlock_irqrestore(up, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_get_mctrl(up: *mut uart_port) -> c_uint {
    static unsigned int serial_txx9_get_mctrl(struct uart_port *up)
    {
    unsigned int ret;
// no modem control lines
    ret = TIOCM_CAR | TIOCM_DSR;
    ret |= (sio_in(up, TXX9_SIFLCR) & TXX9_SIFLCR_RTSSC) ? 0 : TIOCM_RTS;
    ret |= (sio_in(up, TXX9_SICISR) & TXX9_SICISR_CTSS) ? 0 : TIOCM_CTS;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_set_mctrl(up: *mut uart_port, mctrl: c_uint) {
    static void serial_txx9_set_mctrl(struct uart_port *up, unsigned int mctrl)
    {
    if (mctrl & TIOCM_RTS)
    sio_mask(up, TXX9_SIFLCR, TXX9_SIFLCR_RTSSC);
    else
    sio_set(up, TXX9_SIFLCR, TXX9_SIFLCR_RTSSC);
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_break_ctl(up: *mut uart_port, break_state: c_int) {
    static void serial_txx9_break_ctl(struct uart_port *up, int break_state)
    {
    unsigned long flags;
    uart_port_lock_irqsave(up, &flags);
    if (break_state == -1)
    sio_set(up, TXX9_SIFLCR, TXX9_SIFLCR_TBRK);
    else
    sio_mask(up, TXX9_SIFLCR, TXX9_SIFLCR_TBRK);
    uart_port_unlock_irqrestore(up, flags);
    }

//
// Wait for transmitter & holding register to empty
//
#[no_mangle]
unsafe extern "C" fn wait_for_xmitr(up: *mut uart_port) {
    static void wait_for_xmitr(struct uart_port *up)
    {
    let mut tmout: c_uint = 10000;
// Wait up to 10ms for the character(s) to be sent.
    while (--tmout &&
    !(sio_in(up, TXX9_SICISR) & TXX9_SICISR_TXALS))
    udelay(1);
// Wait up to 1s for flow control if necessary
    if (uart_cons_flow_enabled(up)) {
    tmout = 1000000;
    while (--tmout &&
    (sio_in(up, TXX9_SICISR) & TXX9_SICISR_CTSS))
    udelay(1);
    }
    }

//
// Console polling routines for writing and reading from the uart while
// in an interrupt or debug context.
//
#[no_mangle]
unsafe extern "C" fn serial_txx9_get_poll_char(up: *mut uart_port) -> c_int {
    static int serial_txx9_get_poll_char(struct uart_port *up)
    {
    unsigned int ier;
    unsigned char c;
//
// First save the IER then disable the interrupts
//
    ier = sio_in(up, TXX9_SIDICR);
    sio_out(up, TXX9_SIDICR, 0);
    while (sio_in(up, TXX9_SIDISR) & TXX9_SIDISR_UVALID)
    ;
    c = sio_in(up, TXX9_SIRFIFO);
//
// Finally, clear RX interrupt status
// and restore the IER
//
    sio_mask(up, TXX9_SIDISR, TXX9_SIDISR_RDIS);
    sio_out(up, TXX9_SIDICR, ier);
    return c;
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_put_poll_char(up: *mut uart_port, c: c_uchar) {
    static void serial_txx9_put_poll_char(struct uart_port *up, unsigned char c)
    {
    unsigned int ier;
//
// First save the IER then disable the interrupts
//
    ier = sio_in(up, TXX9_SIDICR);
    sio_out(up, TXX9_SIDICR, 0);
    wait_for_xmitr(up);
//
// Send the character out.
//
    sio_out(up, TXX9_SITFIFO, c);
//
// Finally, wait for transmitter to become empty
// and restore the IER
//
    wait_for_xmitr(up);
    sio_out(up, TXX9_SIDICR, ier);
    }

#[no_mangle]
unsafe extern "C" fn serial_txx9_startup(up: *mut uart_port) -> c_int {
    static int serial_txx9_startup(struct uart_port *up)
    {
    unsigned long flags;
    int retval;
//
// Clear the FIFO buffers and disable them.
// (they will be reenabled in set_termios())
//
    sio_set(up, TXX9_SIFCR,
    TXX9_SIFCR_TFRST | TXX9_SIFCR_RFRST | TXX9_SIFCR_FRSTE);
// clear reset
    sio_mask(up, TXX9_SIFCR,
    TXX9_SIFCR_TFRST | TXX9_SIFCR_RFRST | TXX9_SIFCR_FRSTE);
    sio_out(up, TXX9_SIDICR, 0);
//
// Clear the interrupt registers.
//
    sio_out(up, TXX9_SIDISR, 0);
    retval = request_irq(up.irq, serial_txx9_interrupt,
    IRQF_SHARED, "serial_txx9", up);
    if (retval)
    return retval;
//
// Now, initialize the UART
//
    uart_port_lock_irqsave(up, &flags);
    serial_txx9_set_mctrl(up, up.mctrl);
    uart_port_unlock_irqrestore(up, flags);
// Enable RX/TX
    sio_mask(up, TXX9_SIFLCR, TXX9_SIFLCR_RSDE | TXX9_SIFLCR_TSDE);
//
// Finally, enable interrupts.
//
    sio_set(up, TXX9_SIDICR, TXX9_SIDICR_RIE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_shutdown(up: *mut uart_port) {
    static void serial_txx9_shutdown(struct uart_port *up)
    {
    unsigned long flags;
//
// Disable interrupts from this port
//
    sio_out(up, TXX9_SIDICR, 0);	/* disable all intrs */
    uart_port_lock_irqsave(up, &flags);
    serial_txx9_set_mctrl(up, up.mctrl);
    uart_port_unlock_irqrestore(up, flags);
//
// Disable break condition
//
    sio_mask(up, TXX9_SIFLCR, TXX9_SIFLCR_TBRK);

    if (up.cons && up.line == up.cons.index) {
    free_irq(up.irq, up);
    return;
    }

// reset FIFOs
    sio_set(up, TXX9_SIFCR,
    TXX9_SIFCR_TFRST | TXX9_SIFCR_RFRST | TXX9_SIFCR_FRSTE);
// clear reset
    sio_mask(up, TXX9_SIFCR,
    TXX9_SIFCR_TFRST | TXX9_SIFCR_RFRST | TXX9_SIFCR_FRSTE);
// Disable RX/TX
    sio_set(up, TXX9_SIFLCR, TXX9_SIFLCR_RSDE | TXX9_SIFLCR_TSDE);
    free_irq(up.irq, up);
    }
    static void
    serial_txx9_set_termios(struct uart_port *up, struct ktermios *termios,
    const struct ktermios *old)
    {
    unsigned int cval, fcr = 0;
    unsigned long flags;
    unsigned int baud, quot;
//
// We don't support modem control lines.
//
    termios.c_cflag &= ~(HUPCL | CMSPAR);
    termios.c_cflag |= CLOCAL;
    cval = sio_in(up, TXX9_SILCR);
// byte size and parity
    cval &= ~TXX9_SILCR_UMODE_MASK;
    switch (termios.c_cflag & CSIZE) {
    case CS7:
    cval |= TXX9_SILCR_UMODE_7BIT;
    break;
    default:
    case CS5:	/* not supported */
    case CS6:	/* not supported */
    case CS8:
    cval |= TXX9_SILCR_UMODE_8BIT;
    termios.c_cflag &= ~CSIZE;
    termios.c_cflag |= CS8;
    break;
    }
    cval &= ~TXX9_SILCR_USBL_MASK;
    if (termios.c_cflag & CSTOPB)
    cval |= TXX9_SILCR_USBL_2BIT;
    else
    cval |= TXX9_SILCR_USBL_1BIT;
    cval &= ~(TXX9_SILCR_UPEN | TXX9_SILCR_UEPS);
    if (termios.c_cflag & PARENB)
    cval |= TXX9_SILCR_UPEN;
    if (!(termios.c_cflag & PARODD))
    cval |= TXX9_SILCR_UEPS;
//
// Ask the core to calculate the divisor for us.
//
    baud = uart_get_baud_rate(up, termios, old, 0, up.uartclk/16/2);
    quot = uart_get_divisor(up, baud);
// Set up FIFOs
// TX Int by FIFO Empty, RX Int by Receiving 1 char.
    fcr = TXX9_SIFCR_TDIL_MAX | TXX9_SIFCR_RDIL_1;
//
// Ok, we're now changing the port state.  Do it with
// interrupts disabled.
//
    uart_port_lock_irqsave(up, &flags);
//
// Update the per-port timeout.
//
    uart_update_timeout(up, termios.c_cflag, baud);
    up.read_status_mask = TXX9_SIDISR_UOER |
    TXX9_SIDISR_TDIS | TXX9_SIDISR_RDIS;
    if (termios.c_iflag & INPCK)
    up.read_status_mask |= TXX9_SIDISR_UFER | TXX9_SIDISR_UPER;
    if (termios.c_iflag & (IGNBRK | BRKINT | PARMRK))
    up.read_status_mask |= TXX9_SIDISR_UBRK;
//
// Characteres to ignore
//
    up.ignore_status_mask = 0;
    if (termios.c_iflag & IGNPAR)
    up.ignore_status_mask |= TXX9_SIDISR_UPER | TXX9_SIDISR_UFER;
    if (termios.c_iflag & IGNBRK) {
    up.ignore_status_mask |= TXX9_SIDISR_UBRK;
//
// If we're ignoring parity and break indicators,
// ignore overruns too (for real raw support).
//
    if (termios.c_iflag & IGNPAR)
    up.ignore_status_mask |= TXX9_SIDISR_UOER;
    }
//
// ignore all characters if CREAD is not set
//
    if ((termios.c_cflag & CREAD) == 0)
    up.ignore_status_mask |= TXX9_SIDISR_RDIS;
// CTS flow control flag
    if ((termios.c_cflag & CRTSCTS) &&
    (up.flags & UPF_TXX9_HAVE_CTS_LINE)) {
    sio_set(up, TXX9_SIFLCR,
    TXX9_SIFLCR_RCS | TXX9_SIFLCR_TES);
    } else {
    sio_mask(up, TXX9_SIFLCR,
    TXX9_SIFLCR_RCS | TXX9_SIFLCR_TES);
    }
    sio_out(up, TXX9_SILCR, cval);
    sio_quot_set(up, quot);
    sio_out(up, TXX9_SIFCR, fcr);
    serial_txx9_set_mctrl(up, up.mctrl);
    uart_port_unlock_irqrestore(up, flags);
    }
    static void
    serial_txx9_pm(struct uart_port *port, unsigned int state,
    unsigned int oldstate)
    {
//
// If oldstate was -1 this is called from
// uart_configure_port().  In this case do not initialize the
// port now, because the port was already initialized (for
// non-console port) or should not be initialized here (for
// console port).  If we initialized the port here we lose
// serial console settings.
//
    if (state == 0 && oldstate != -1)
    serial_txx9_initialize(port);
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_request_resource(up: *mut uart_port) -> c_int {
    static int serial_txx9_request_resource(struct uart_port *up)
    {
    let mut size: c_uint = TXX9_REGION_SIZE;
    let mut ret: c_int = 0;
    switch (up.iotype) {
    default:
    if (!up.mapbase)
    break;
    if (!request_mem_region(up.mapbase, size, "serial_txx9")) {
    ret = -EBUSY;
    break;
    }
    if (up.flags & UPF_IOREMAP) {
    up.membase = ioremap(up.mapbase, size);
    if (!up.membase) {
    release_mem_region(up.mapbase, size);
    ret = -ENOMEM;
    }
    }
    break;
    case UPIO_PORT:
    if (!request_region(up.iobase, size, "serial_txx9"))
    ret = -EBUSY;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_release_resource(up: *mut uart_port) {
    static void serial_txx9_release_resource(struct uart_port *up)
    {
    let mut size: c_uint = TXX9_REGION_SIZE;
    switch (up.iotype) {
    default:
    if (!up.mapbase)
    break;
    if (up.flags & UPF_IOREMAP) {
    iounmap(up.membase);
    up.membase = core::ptr::null_mut();
    }
    release_mem_region(up.mapbase, size);
    break;
    case UPIO_PORT:
    release_region(up.iobase, size);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_release_port(up: *mut uart_port) {
    static void serial_txx9_release_port(struct uart_port *up)
    {
    serial_txx9_release_resource(up);
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_request_port(up: *mut uart_port) -> c_int {
    static int serial_txx9_request_port(struct uart_port *up)
    {
    return serial_txx9_request_resource(up);
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_config_port(up: *mut uart_port, uflags: c_int) {
    static void serial_txx9_config_port(struct uart_port *up, int uflags)
    {
    int ret;
//
// Find the region that we can probe for.  This in turn
// tells us whether we can probe for the type of port.
//
    ret = serial_txx9_request_resource(up);
    if (ret < 0)
    return;
    up.type = PORT_TXX9;
    up.fifosize = TXX9_SIO_TX_FIFO;

    if (up.line == up.cons.index)
    return;

    serial_txx9_initialize(up);
    }
    static const char *
    serial_txx9_type(struct uart_port *port)
    {
    return "txx9";
    }
    static const struct uart_ops serial_txx9_pops = {
    .tx_empty	= serial_txx9_tx_empty,
    .set_mctrl	= serial_txx9_set_mctrl,
    .get_mctrl	= serial_txx9_get_mctrl,
    .stop_tx	= serial_txx9_stop_tx,
    .start_tx	= serial_txx9_start_tx,
    .stop_rx	= serial_txx9_stop_rx,
    .break_ctl	= serial_txx9_break_ctl,
    .startup	= serial_txx9_startup,
    .shutdown	= serial_txx9_shutdown,
    .set_termios	= serial_txx9_set_termios,
    .pm		= serial_txx9_pm,
    .type		= serial_txx9_type,
    .release_port	= serial_txx9_release_port,
    .request_port	= serial_txx9_request_port,
    .config_port	= serial_txx9_config_port,

    .poll_get_char	= serial_txx9_get_poll_char,
    .poll_put_char	= serial_txx9_put_poll_char,

    };
    static struct uart_port serial_txx9_ports[UART_NR];
    static void __init serial_txx9_register_ports(struct uart_driver *drv,
    struct device *dev)
    {
    int i;
    for (i = 0; i < UART_NR; i++) {
    struct uart_port *up = &serial_txx9_ports[i];
    up.line = i;
    up.ops = &serial_txx9_pops;
    up.dev = dev;
    if (up.iobase || up.mapbase)
    uart_add_one_port(drv, up);
    }
    }

#[no_mangle]
unsafe extern "C" fn serial_txx9_console_putchar(up: *mut uart_port, ch: c_uchar) {
    static void serial_txx9_console_putchar(struct uart_port *up, unsigned char ch)
    {
    wait_for_xmitr(up);
    sio_out(up, TXX9_SITFIFO, ch);
    }
//
// Print a string to the serial port trying not to disturb
// any possible real use of the port...
//
// The console_lock must be held when we get here.
//
    static void
    serial_txx9_console_write(struct console *co, const char *s, unsigned int count)
    {
    struct uart_port *up = &serial_txx9_ports[co.index];
    unsigned int ier, flcr;
//
// First save the UER then disable the interrupts
//
    ier = sio_in(up, TXX9_SIDICR);
    sio_out(up, TXX9_SIDICR, 0);
//
// Disable flow-control if enabled (and unnecessary)
//
    flcr = sio_in(up, TXX9_SIFLCR);
    if (!uart_cons_flow_enabled(up) && (flcr & TXX9_SIFLCR_TES))
    sio_out(up, TXX9_SIFLCR, flcr & ~TXX9_SIFLCR_TES);
    uart_console_write(up, s, count, serial_txx9_console_putchar);
//
// Finally, wait for transmitter to become empty
// and restore the IER
//
    wait_for_xmitr(up);
    sio_out(up, TXX9_SIFLCR, flcr);
    sio_out(up, TXX9_SIDICR, ier);
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_console_setup(co: *mut console, options: *mut c_char) -> int __init {
    static int __init serial_txx9_console_setup(struct console *co, char *options)
    {
    struct uart_port *up;
    let mut baud: c_int = 9600;
    let mut bits: c_int = 8;
    let mut parity: c_int = 'n';
    let mut flow: c_int = 'n';
//
// Check whether an invalid uart number has been specified, and
// if so, search for the first available port that does have
// console support.
//
    if (co.index >= UART_NR)
    co.index = 0;
    up = &serial_txx9_ports[co.index];
    if (!up.ops)
    return -ENODEV;
    serial_txx9_initialize(up);
    if (options)
    uart_parse_options(options, &baud, &parity, &bits, &flow);
    return uart_set_options(up, co, baud, parity, bits, flow);
    }
    static struct uart_driver serial_txx9_reg;
    static struct console serial_txx9_console = {
    .name		= TXX9_TTY_NAME,
    .write		= serial_txx9_console_write,
    .device		= uart_console_device,
    .setup		= serial_txx9_console_setup,
    .flags		= CON_PRINTBUFFER,
    .index		= -1,
    .data		= &serial_txx9_reg,
    };
#[no_mangle]
unsafe extern "C" fn serial_txx9_console_init() -> int __init {
    static int __init serial_txx9_console_init(void)
    {
    register_console(&serial_txx9_console);
    return 0;
    }
    console_initcall(serial_txx9_console_init);

    static struct uart_driver serial_txx9_reg = {
    .owner			= THIS_MODULE,
    .driver_name		= "serial_txx9",
    .dev_name		= TXX9_TTY_NAME,
    .major			= TXX9_TTY_MAJOR,
    .minor			= TXX9_TTY_MINOR_START,
    .nr			= UART_NR,
    .cons			= SERIAL_TXX9_CONSOLE,
    };
#[no_mangle]
pub unsafe extern "C" fn early_serial_txx9_setup(port: *mut uart_port) -> int __init {
    int __init early_serial_txx9_setup(struct uart_port *port)
    {
    if (port.line >= ARRAY_SIZE(serial_txx9_ports))
    return -ENODEV;
    serial_txx9_ports[port.line] = *port;
    serial_txx9_ports[port.line].ops = &serial_txx9_pops;
    serial_txx9_ports[port.line].flags |=
    UPF_BOOT_AUTOCONF | UPF_FIXED_PORT;
    return 0;
    }
    static DEFINE_MUTEX(serial_txx9_mutex);
//
// serial_txx9_register_port - register a serial port
// @port: serial port template
//
// Configure the serial port specified by the request.
//
// The port is then probed and if necessary the IRQ is autodetected
// If this fails an error is returned.
//
// On success the port is ready to use and the line number is returned.
//
#[no_mangle]
unsafe extern "C" fn serial_txx9_register_port(port: *mut uart_port) -> c_int {
    static int serial_txx9_register_port(struct uart_port *port)
    {
    int i;
    struct uart_port *uart;
    let mut ret: c_int = -ENOSPC;
    mutex_lock(&serial_txx9_mutex);
    for (i = 0; i < UART_NR; i++) {
    uart = &serial_txx9_ports[i];
    if (uart_match_port(uart, port)) {
    uart_remove_one_port(&serial_txx9_reg, uart);
    break;
    }
    }
    if (i == UART_NR) {
// Find unused port
    for (i = 0; i < UART_NR; i++) {
    uart = &serial_txx9_ports[i];
    if (!(uart.iobase || uart.mapbase))
    break;
    }
    }
    if (i < UART_NR) {
    uart.iobase = port.iobase;
    uart.membase = port.membase;
    uart.irq      = port.irq;
    uart.uartclk  = port.uartclk;
    uart.iotype   = port.iotype;
    uart.flags    = port.flags
    | UPF_BOOT_AUTOCONF | UPF_FIXED_PORT;
    uart.mapbase  = port.mapbase;
    if (port.dev)
    uart.dev = port.dev;
    ret = uart_add_one_port(&serial_txx9_reg, uart);
    if (ret == 0)
    ret = uart.line;
    }
    mutex_unlock(&serial_txx9_mutex);
    return ret;
    }
//
// serial_txx9_unregister_port - remove a txx9 serial port at runtime
// @line: serial line number
//
// Remove one serial port.  This may not be called from interrupt
// context.  We hand the port back to the our control.
//
#[no_mangle]
unsafe extern "C" fn serial_txx9_unregister_port(line: c_int) {
    static void serial_txx9_unregister_port(int line)
    {
    struct uart_port *uart = &serial_txx9_ports[line];
    mutex_lock(&serial_txx9_mutex);
    uart_remove_one_port(&serial_txx9_reg, uart);
    uart.flags = 0;
    uart.type = PORT_UNKNOWN;
    uart.iobase = 0;
    uart.mapbase = 0;
    uart.membase = core::ptr::null_mut();
    uart.dev = core::ptr::null_mut();
    mutex_unlock(&serial_txx9_mutex);
    }
//
// Register a set of serial devices attached to a platform device.
//
#[no_mangle]
unsafe extern "C" fn serial_txx9_probe(dev: *mut platform_device) -> c_int {
    static int serial_txx9_probe(struct platform_device *dev)
    {
    struct uart_port *p = dev_get_platdata(&dev.dev);
    struct uart_port port;
    int ret, i;
    memset(&port, 0, sizeof(struct uart_port));
    for (i = 0; p && p.uartclk != 0; p++, i++) {
    port.iobase	= p.iobase;
    port.membase	= p.membase;
    port.irq	= p.irq;
    port.uartclk	= p.uartclk;
    port.iotype	= p.iotype;
    port.flags	= p.flags;
    port.mapbase	= p.mapbase;
    port.dev	= &dev.dev;
    port.has_sysrq	= IS_ENABLED(CONFIG_SERIAL_TXX9_CONSOLE);
    ret = serial_txx9_register_port(&port);
    if (ret < 0) {
    dev_err(&dev.dev, "unable to register port at index %d "
    "(IO%lx MEM%llx IRQ%d): %d\n", i,
    p.iobase, (unsigned long long)p.mapbase,
    p.irq, ret);
    }
    }
    return 0;
    }
//
// Remove serial ports registered against a platform device.
//
#[no_mangle]
unsafe extern "C" fn serial_txx9_remove(dev: *mut platform_device) {
    static void serial_txx9_remove(struct platform_device *dev)
    {
    int i;
    for (i = 0; i < UART_NR; i++) {
    struct uart_port *up = &serial_txx9_ports[i];
    if (up.dev == &dev.dev)
    serial_txx9_unregister_port(i);
    }
    }

#[no_mangle]
unsafe extern "C" fn serial_txx9_suspend(dev: *mut platform_device, state: pm_message_t) -> c_int {
    static int serial_txx9_suspend(struct platform_device *dev, pm_message_t state)
    {
    int i;
    for (i = 0; i < UART_NR; i++) {
    struct uart_port *up = &serial_txx9_ports[i];
    if (up.type != PORT_UNKNOWN && up.dev == &dev.dev)
    uart_suspend_port(&serial_txx9_reg, up);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_resume(dev: *mut platform_device) -> c_int {
    static int serial_txx9_resume(struct platform_device *dev)
    {
    int i;
    for (i = 0; i < UART_NR; i++) {
    struct uart_port *up = &serial_txx9_ports[i];
    if (up.type != PORT_UNKNOWN && up.dev == &dev.dev)
    uart_resume_port(&serial_txx9_reg, up);
    }
    return 0;
    }

    static struct platform_driver serial_txx9_plat_driver = {
    .probe		= serial_txx9_probe,
    .remove		= serial_txx9_remove,

    .suspend	= serial_txx9_suspend,
    .resume		= serial_txx9_resume,

    .driver		= {
    .name	= "serial_txx9",
    },
    };

//
// Probe one serial board.  Unfortunately, there is no rhyme nor reason
// to the arrangement of serial ports on a PCI card.
//
    static int
    pciserial_txx9_init_one(struct pci_dev *dev, const struct pci_device_id *ent)
    {
    struct uart_port port;
    int line;
    int rc;
    rc = pci_enable_device(dev);
    if (rc)
    return rc;
    memset(&port, 0, sizeof(port));
    port.ops = &serial_txx9_pops;
    port.flags |= UPF_TXX9_HAVE_CTS_LINE;
    port.uartclk = 66670000;
    port.irq = dev.irq;
    port.iotype = UPIO_PORT;
    port.iobase = pci_resource_start(dev, 1);
    port.dev = &dev.dev;
    line = serial_txx9_register_port(&port);
    if (line < 0) {
    printk(KERN_WARNING "Couldn't register serial port %s: %d\n", pci_name(dev), line);
    pci_disable_device(dev);
    return line;
    }
    pci_set_drvdata(dev, &serial_txx9_ports[line]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pciserial_txx9_remove_one(dev: *mut pci_dev) {
    static void pciserial_txx9_remove_one(struct pci_dev *dev)
    {
    struct uart_port *up = pci_get_drvdata(dev);
    if (up) {
    serial_txx9_unregister_port(up.line);
    pci_disable_device(dev);
    }
    }

#[no_mangle]
unsafe extern "C" fn pciserial_txx9_suspend_one(dev: *mut pci_dev, state: pm_message_t) -> c_int {
    static int pciserial_txx9_suspend_one(struct pci_dev *dev, pm_message_t state)
    {
    struct uart_port *up = pci_get_drvdata(dev);
    if (up)
    uart_suspend_port(&serial_txx9_reg, up);
    pci_save_state(dev);
    pci_set_power_state(dev, pci_choose_state(dev, state));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pciserial_txx9_resume_one(dev: *mut pci_dev) -> c_int {
    static int pciserial_txx9_resume_one(struct pci_dev *dev)
    {
    struct uart_port *up = pci_get_drvdata(dev);
    pci_set_power_state(dev, PCI_D0);
    pci_restore_state(dev);
    if (up)
    uart_resume_port(&serial_txx9_reg, up);
    return 0;
    }

    static const struct pci_device_id serial_txx9_pci_tbl[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_TOSHIBA_2, PCI_DEVICE_ID_TOSHIBA_TC86C001_MISC) },
    { 0, }
    };
    static struct pci_driver serial_txx9_pci_driver = {
    .name		= "serial_txx9",
    .probe		= pciserial_txx9_init_one,
    .remove		= pciserial_txx9_remove_one,

    .suspend	= pciserial_txx9_suspend_one,
    .resume		= pciserial_txx9_resume_one,

    .id_table	= serial_txx9_pci_tbl,
    };
    MODULE_DEVICE_TABLE(pci, serial_txx9_pci_tbl);

    static struct platform_device *serial_txx9_plat_devs;
#[no_mangle]
unsafe extern "C" fn serial_txx9_init() -> int __init {
    static int __init serial_txx9_init(void)
    {
    int ret;
    ret = uart_register_driver(&serial_txx9_reg);
    if (ret)
    goto out;
    serial_txx9_plat_devs = platform_device_alloc("serial_txx9", -1);
    if (!serial_txx9_plat_devs) {
    ret = -ENOMEM;
    goto unreg_uart_drv;
    }
    ret = platform_device_add(serial_txx9_plat_devs);
    if (ret)
    goto put_dev;
    serial_txx9_register_ports(&serial_txx9_reg,
    &serial_txx9_plat_devs.dev);
    ret = platform_driver_register(&serial_txx9_plat_driver);
    if (ret)
    goto del_dev;

    ret = pci_register_driver(&serial_txx9_pci_driver);
    if (ret) {
    platform_driver_unregister(&serial_txx9_plat_driver);
    }

    if (ret == 0)
    goto out;
    del_dev:
    platform_device_del(serial_txx9_plat_devs);
    put_dev:
    platform_device_put(serial_txx9_plat_devs);
    unreg_uart_drv:
    uart_unregister_driver(&serial_txx9_reg);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn serial_txx9_exit() -> void __exit {
    static void __exit serial_txx9_exit(void)
    {
    int i;

    pci_unregister_driver(&serial_txx9_pci_driver);

    platform_driver_unregister(&serial_txx9_plat_driver);
    platform_device_unregister(serial_txx9_plat_devs);
    for (i = 0; i < UART_NR; i++) {
    struct uart_port *up = &serial_txx9_ports[i];
    if (up.iobase || up.mapbase)
    uart_remove_one_port(&serial_txx9_reg, up);
    }
    uart_unregister_driver(&serial_txx9_reg);
    }
    module_init(serial_txx9_init);
    module_exit(serial_txx9_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("TX39/49 serial driver");
    MODULE_ALIAS_CHARDEV_MAJOR(TXX9_TTY_MAJOR);

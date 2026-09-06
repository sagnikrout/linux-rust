//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/st-asc.c
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
// st-asc.c: ST Asynchronous serial controller (ASC) driver
//
// Copyright (C) 2003-2013 STMicroelectronics (R&D) Limited
//

pub const ASC_FIFO_SIZE: c_int = 16;
pub const ASC_MAX_PORTS: c_int = 8;
// Pinctrl states
pub const DEFAULT: c_int = 0;
pub const NO_HW_FLOWCTRL: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asc_port {
    pub port: uart_port,
    pub rts: *mut gpio_desc,
    pub clk: *mut clk,
    pub pinctrl: *mut pinctrl,
    pub states: [*mut pinctrl_state; 2],
    pub hw_flow_control:1: c_uint,
    pub force_m1:1: c_uint,
}

    static struct asc_port asc_ports[ASC_MAX_PORTS];
    static struct uart_driver asc_uart_driver;
// ---- UART Register definitions ------------------------------
// Register offsets
pub const ASC_BAUDRATE: c_uint = 0x00;
pub const ASC_TXBUF: c_uint = 0x04;
pub const ASC_RXBUF: c_uint = 0x08;
pub const ASC_CTL: c_uint = 0x0C;
pub const ASC_INTEN: c_uint = 0x10;
pub const ASC_STA: c_uint = 0x14;
pub const ASC_GUARDTIME: c_uint = 0x18;
pub const ASC_TIMEOUT: c_uint = 0x1C;
pub const ASC_TXRESET: c_uint = 0x20;
pub const ASC_RXRESET: c_uint = 0x24;
pub const ASC_RETRIES: c_uint = 0x28;
// ASC_RXBUF
pub const ASC_RXBUF_PE: c_uint = 0x100;
pub const ASC_RXBUF_FE: c_uint = 0x200;
//
// Some of status comes from higher bits of the character and some come from
// the status register. Combining both of them in to single status using dummy
// bits.
//
pub const ASC_RXBUF_DUMMY_RX: c_uint = 0x10000;
pub const ASC_RXBUF_DUMMY_BE: c_uint = 0x20000;
pub const ASC_RXBUF_DUMMY_OE: c_uint = 0x40000;
// ASC_CTL
pub const ASC_CTL_MODE_MSK: c_uint = 0x0007;
pub const ASC_CTL_MODE_8BIT: c_uint = 0x0001;
pub const ASC_CTL_MODE_7BIT_PAR: c_uint = 0x0003;
pub const ASC_CTL_MODE_9BIT: c_uint = 0x0004;
pub const ASC_CTL_MODE_8BIT_WKUP: c_uint = 0x0005;
pub const ASC_CTL_MODE_8BIT_PAR: c_uint = 0x0007;
pub const ASC_CTL_STOP_MSK: c_uint = 0x0018;
pub const ASC_CTL_STOP_HALFBIT: c_uint = 0x0000;
pub const ASC_CTL_STOP_1BIT: c_uint = 0x0008;
pub const ASC_CTL_STOP_1_HALFBIT: c_uint = 0x0010;
pub const ASC_CTL_STOP_2BIT: c_uint = 0x0018;
pub const ASC_CTL_PARITYODD: c_uint = 0x0020;
pub const ASC_CTL_LOOPBACK: c_uint = 0x0040;
pub const ASC_CTL_RUN: c_uint = 0x0080;
pub const ASC_CTL_RXENABLE: c_uint = 0x0100;
pub const ASC_CTL_SCENABLE: c_uint = 0x0200;
pub const ASC_CTL_FIFOENABLE: c_uint = 0x0400;
pub const ASC_CTL_CTSENABLE: c_uint = 0x0800;
pub const ASC_CTL_BAUDMODE: c_uint = 0x1000;
// ASC_GUARDTIME
pub const ASC_GUARDTIME_MSK: c_uint = 0x00FF;
// ASC_INTEN
pub const ASC_INTEN_RBE: c_uint = 0x0001;
pub const ASC_INTEN_TE: c_uint = 0x0002;
pub const ASC_INTEN_THE: c_uint = 0x0004;
pub const ASC_INTEN_PE: c_uint = 0x0008;
pub const ASC_INTEN_FE: c_uint = 0x0010;
pub const ASC_INTEN_OE: c_uint = 0x0020;
pub const ASC_INTEN_TNE: c_uint = 0x0040;
pub const ASC_INTEN_TOI: c_uint = 0x0080;
pub const ASC_INTEN_RHF: c_uint = 0x0100;
// ASC_RETRIES
pub const ASC_RETRIES_MSK: c_uint = 0x00FF;
// ASC_RXBUF
pub const ASC_RXBUF_MSK: c_uint = 0x03FF;
// ASC_STA
pub const ASC_STA_RBF: c_uint = 0x0001;
pub const ASC_STA_TE: c_uint = 0x0002;
pub const ASC_STA_THE: c_uint = 0x0004;
pub const ASC_STA_PE: c_uint = 0x0008;
pub const ASC_STA_FE: c_uint = 0x0010;
pub const ASC_STA_OE: c_uint = 0x0020;
pub const ASC_STA_TNE: c_uint = 0x0040;
pub const ASC_STA_TOI: c_uint = 0x0080;
pub const ASC_STA_RHF: c_uint = 0x0100;
pub const ASC_STA_TF: c_uint = 0x0200;
pub const ASC_STA_NKD: c_uint = 0x0400;
// ASC_TIMEOUT
pub const ASC_TIMEOUT_MSK: c_uint = 0x00FF;
// ASC_TXBUF
pub const ASC_TXBUF_MSK: c_uint = 0x01FF;
// ---- Inline function definitions ---------------------------
    static inline struct asc_port *to_asc_port(struct uart_port *port)
    {
    return container_of(port, struct asc_port, port);
    }
#[no_mangle]
pub unsafe extern "C" fn asc_in(port: *mut uart_port, offset: u32) -> u32 {
    static inline u32 asc_in(struct uart_port *port, u32 offset)
    {

    return readl_relaxed(port.membase + offset);

    return readl(port.membase + offset);

    }
#[no_mangle]
pub unsafe extern "C" fn asc_out(port: *mut uart_port, offset: u32, value: u32) {
    static inline void asc_out(struct uart_port *port, u32 offset, u32 value)
    {

    writel_relaxed(value, port.membase + offset);

    writel(value, port.membase + offset);

    }
//
// Some simple utility functions to enable and disable interrupts.
// Note that these need to be called with interrupts disabled.
//
#[no_mangle]
pub unsafe extern "C" fn asc_disable_tx_interrupts(port: *mut uart_port) {
    static inline void asc_disable_tx_interrupts(struct uart_port *port)
    {
    let mut intenable: u32 = asc_in(port, ASC_INTEN) & ~ASC_INTEN_THE;
    asc_out(port, ASC_INTEN, intenable);
    (void)asc_in(port, ASC_INTEN);	/* Defeat bus write posting */
    }
#[no_mangle]
pub unsafe extern "C" fn asc_enable_tx_interrupts(port: *mut uart_port) {
    static inline void asc_enable_tx_interrupts(struct uart_port *port)
    {
    let mut intenable: u32 = asc_in(port, ASC_INTEN) | ASC_INTEN_THE;
    asc_out(port, ASC_INTEN, intenable);
    }
#[no_mangle]
pub unsafe extern "C" fn asc_disable_rx_interrupts(port: *mut uart_port) {
    static inline void asc_disable_rx_interrupts(struct uart_port *port)
    {
    let mut intenable: u32 = asc_in(port, ASC_INTEN) & ~ASC_INTEN_RBE;
    asc_out(port, ASC_INTEN, intenable);
    (void)asc_in(port, ASC_INTEN);	/* Defeat bus write posting */
    }
#[no_mangle]
pub unsafe extern "C" fn asc_enable_rx_interrupts(port: *mut uart_port) {
    static inline void asc_enable_rx_interrupts(struct uart_port *port)
    {
    let mut intenable: u32 = asc_in(port, ASC_INTEN) | ASC_INTEN_RBE;
    asc_out(port, ASC_INTEN, intenable);
    }
#[no_mangle]
pub unsafe extern "C" fn asc_txfifo_is_empty(port: *mut uart_port) -> u32 {
    static inline u32 asc_txfifo_is_empty(struct uart_port *port)
    {
    return asc_in(port, ASC_STA) & ASC_STA_TE;
    }
#[no_mangle]
pub unsafe extern "C" fn asc_txfifo_is_half_empty(port: *mut uart_port) -> u32 {
    static inline u32 asc_txfifo_is_half_empty(struct uart_port *port)
    {
    return asc_in(port, ASC_STA) & ASC_STA_THE;
    }
    static inline const char *asc_port_name(struct uart_port *port)
    {
    return to_platform_device(port.dev).name;
    }
// ----------------------------------------------------------------------
//
// This section contains code to support the use of the ASC as a
// generic serial port.
//
#[no_mangle]
pub unsafe extern "C" fn asc_hw_txroom(port: *mut uart_port) -> unsigned {
    static inline unsigned asc_hw_txroom(struct uart_port *port)
    {
    let mut status: u32 = asc_in(port, ASC_STA);
    if (status & ASC_STA_THE)
    return port.fifosize / 2;
#[no_mangle]
pub unsafe extern "C" fn if(ASC_STA_TF): !(status &) -> else {
    else if (!(status & ASC_STA_TF))
    return 1;
    return 0;
    }
//
// Start transmitting chars.
// This is called from both interrupt and task level.
// Either way interrupts are disabled.
//
#[no_mangle]
unsafe extern "C" fn asc_transmit_chars(port: *mut uart_port) {
    static void asc_transmit_chars(struct uart_port *port)
    {
    u8 ch;
    uart_port_tx_limited(port, ch, asc_hw_txroom(port),
    true,
    asc_out(port, ASC_TXBUF, ch),
    ({}));
    }
#[no_mangle]
unsafe extern "C" fn asc_receive_chars(port: *mut uart_port) {
    static void asc_receive_chars(struct uart_port *port)
    {
    struct tty_port *tport = &port.state.port;
    unsigned long status, mode;
    let mut c: c_ulong = 0;
    u8 flag;
    let mut ignore_pe: bool = false;
//
// Datasheet states: If the MODE field selects an 8-bit frame then
// this [parity error] bit is undefined. Software should ignore this
// bit when reading 8-bit frames.
//
    mode = asc_in(port, ASC_CTL) & ASC_CTL_MODE_MSK;
    if (mode == ASC_CTL_MODE_8BIT || mode == ASC_CTL_MODE_8BIT_PAR)
    ignore_pe = true;
    if (irqd_is_wakeup_set(irq_get_irq_data(port.irq)))
    pm_wakeup_event(tport.tty.dev, 0);
    while ((status = asc_in(port, ASC_STA)) & ASC_STA_RBF) {
    c = asc_in(port, ASC_RXBUF) | ASC_RXBUF_DUMMY_RX;
    flag = TTY_NORMAL;
    port.icount.rx++;
    if (status & ASC_STA_OE || c & ASC_RXBUF_FE ||
    (c & ASC_RXBUF_PE && !ignore_pe)) {
    if (c & ASC_RXBUF_FE) {
    if (c == (ASC_RXBUF_FE | ASC_RXBUF_DUMMY_RX)) {
    port.icount.brk++;
    if (uart_handle_break(port))
    continue;
    c |= ASC_RXBUF_DUMMY_BE;
    } else {
    port.icount.frame++;
    }
    } else if (c & ASC_RXBUF_PE) {
    port.icount.parity++;
    }
//
// Reading any data from the RX FIFO clears the
// overflow error condition.
//
    if (status & ASC_STA_OE) {
    port.icount.overrun++;
    c |= ASC_RXBUF_DUMMY_OE;
    }
    c &= port.read_status_mask;
    if (c & ASC_RXBUF_DUMMY_BE)
    flag = TTY_BREAK;
#[no_mangle]
pub unsafe extern "C" fn if(ASC_RXBUF_PE: c &) -> else {
    else if (c & ASC_RXBUF_PE)
    flag = TTY_PARITY;
#[no_mangle]
pub unsafe extern "C" fn if(ASC_RXBUF_FE: c &) -> else {
    else if (c & ASC_RXBUF_FE)
    flag = TTY_FRAME;
    }
    if (uart_handle_sysrq_char(port, c & 0xff))
    continue;
    uart_insert_char(port, c, ASC_RXBUF_DUMMY_OE, c & 0xff, flag);
    }
// Tell the rest of the system the news. New characters!
    tty_flip_buffer_push(tport);
    }
#[no_mangle]
unsafe extern "C" fn asc_interrupt(irq: c_int, ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t asc_interrupt(int irq, void *ptr)
    {
    struct uart_port *port = ptr;
    u32 status;
    uart_port_lock(port);
    status = asc_in(port, ASC_STA);
    if (status & ASC_STA_RBF) {
// Receive FIFO not empty
    asc_receive_chars(port);
    }
    if ((status & ASC_STA_THE) &&
    (asc_in(port, ASC_INTEN) & ASC_INTEN_THE)) {
// Transmitter FIFO at least half empty
    asc_transmit_chars(port);
    }
    uart_port_unlock(port);
    return IRQ_HANDLED;
    }
// ----------------------------------------------------------------------
//
// UART Functions
//
#[no_mangle]
unsafe extern "C" fn asc_tx_empty(port: *mut uart_port) -> c_uint {
    static unsigned int asc_tx_empty(struct uart_port *port)
    {
    return asc_txfifo_is_empty(port) ? TIOCSER_TEMT : 0;
    }
#[no_mangle]
unsafe extern "C" fn asc_set_mctrl(port: *mut uart_port, mctrl: c_uint) {
    static void asc_set_mctrl(struct uart_port *port, unsigned int mctrl)
    {
    struct asc_port *ascport = to_asc_port(port);
//
// This routine is used for seting signals of: DTR, DCD, CTS and RTS.
// We use ASC's hardware for CTS/RTS when hardware flow-control is
// enabled, however if the RTS line is required for another purpose,
// commonly controlled using HUP from userspace, then we need to toggle
// it manually, using GPIO.
//
// Some boards also have DTR and DCD implemented using PIO pins, code to
// do this should be hooked in here.
//
    if (!ascport.rts)
    return;
// If HW flow-control is enabled, we can't fiddle with the RTS line
    if (asc_in(port, ASC_CTL) & ASC_CTL_CTSENABLE)
    return;
    gpiod_set_value(ascport.rts, mctrl & TIOCM_RTS);
    }
#[no_mangle]
unsafe extern "C" fn asc_get_mctrl(port: *mut uart_port) -> c_uint {
    static unsigned int asc_get_mctrl(struct uart_port *port)
    {
//
// This routine is used for geting signals of: DTR, DCD, DSR, RI,
// and CTS/RTS
//
    return TIOCM_CAR | TIOCM_DSR | TIOCM_CTS;
    }
// There are probably characters waiting to be transmitted.
#[no_mangle]
unsafe extern "C" fn asc_start_tx(port: *mut uart_port) {
    static void asc_start_tx(struct uart_port *port)
    {
    struct tty_port *tport = &port.state.port;
    if (!kfifo_is_empty(&tport.xmit_fifo))
    asc_enable_tx_interrupts(port);
    }
// Transmit stop
#[no_mangle]
unsafe extern "C" fn asc_stop_tx(port: *mut uart_port) {
    static void asc_stop_tx(struct uart_port *port)
    {
    asc_disable_tx_interrupts(port);
    }
// Receive stop
#[no_mangle]
unsafe extern "C" fn asc_stop_rx(port: *mut uart_port) {
    static void asc_stop_rx(struct uart_port *port)
    {
    asc_disable_rx_interrupts(port);
    }
// Handle breaks - ignored by us
#[no_mangle]
unsafe extern "C" fn asc_break_ctl(port: *mut uart_port, break_state: c_int) {
    static void asc_break_ctl(struct uart_port *port, int break_state)
    {
// Nothing here yet ..
    }
//
// Enable port for reception.
//
#[no_mangle]
unsafe extern "C" fn asc_startup(port: *mut uart_port) -> c_int {
    static int asc_startup(struct uart_port *port)
    {
    if (request_irq(port.irq, asc_interrupt, 0,
    asc_port_name(port), port)) {
    dev_err(port.dev, "cannot allocate irq.\n");
    return -ENODEV;
    }
    asc_transmit_chars(port);
    asc_enable_rx_interrupts(port);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asc_shutdown(port: *mut uart_port) {
    static void asc_shutdown(struct uart_port *port)
    {
    asc_disable_tx_interrupts(port);
    asc_disable_rx_interrupts(port);
    free_irq(port.irq, port);
    }
    static void asc_pm(struct uart_port *port, unsigned int state,
    unsigned int oldstate)
    {
    struct asc_port *ascport = to_asc_port(port);
    unsigned long flags;
    u32 ctl;
    switch (state) {
    case UART_PM_STATE_ON:
    clk_prepare_enable(ascport.clk);
    break;
    case UART_PM_STATE_OFF:
//
// Disable the ASC baud rate generator, which is as close as
// we can come to turning it off. Note this is not called with
// the port spinlock held.
//
    uart_port_lock_irqsave(port, &flags);
    ctl = asc_in(port, ASC_CTL) & ~ASC_CTL_RUN;
    asc_out(port, ASC_CTL, ctl);
    uart_port_unlock_irqrestore(port, flags);
    clk_disable_unprepare(ascport.clk);
    break;
    }
    }
    static void asc_set_termios(struct uart_port *port, struct ktermios *termios,
    const struct ktermios *old)
    {
    struct asc_port *ascport = to_asc_port(port);
    bool manual_rts, toggle_rts = false;
    struct gpio_desc *gpiod;
    unsigned int baud;
    u32 ctrl_val;
    tcflag_t cflag;
    unsigned long flags;
// Update termios to reflect hardware capabilities
    termios.c_cflag &= ~(CMSPAR |
    (ascport.hw_flow_control ? 0 : CRTSCTS));
    port.uartclk = clk_get_rate(ascport.clk);
    baud = uart_get_baud_rate(port, termios, old, 0, port.uartclk/16);
    cflag = termios.c_cflag;
    uart_port_lock_irqsave(port, &flags);
// read control register
    ctrl_val = asc_in(port, ASC_CTL);
// stop serial port and reset value
    asc_out(port, ASC_CTL, (ctrl_val & ~ASC_CTL_RUN));
    ctrl_val = ASC_CTL_RXENABLE | ASC_CTL_FIFOENABLE;
// reset fifo rx & tx
    asc_out(port, ASC_TXRESET, 1);
    asc_out(port, ASC_RXRESET, 1);
// set character length
    if ((cflag & CSIZE) == CS7) {
    ctrl_val |= ASC_CTL_MODE_7BIT_PAR;
    cflag |= PARENB;
    } else {
    ctrl_val |= (cflag & PARENB) ?  ASC_CTL_MODE_8BIT_PAR :
    ASC_CTL_MODE_8BIT;
    cflag &= ~CSIZE;
    cflag |= CS8;
    }
    termios.c_cflag = cflag;
// set stop bit
    ctrl_val |= (cflag & CSTOPB) ? ASC_CTL_STOP_2BIT : ASC_CTL_STOP_1BIT;
// odd parity
    if (cflag & PARODD)
    ctrl_val |= ASC_CTL_PARITYODD;
// hardware flow control
    if ((cflag & CRTSCTS)) {
    ctrl_val |= ASC_CTL_CTSENABLE;
// If flow-control selected, stop handling RTS manually
    if (ascport.rts) {
    toggle_rts = true;
    manual_rts = false;
    }
    } else {
// If flow-control disabled, it's safe to handle RTS manually
    if (!ascport.rts && ascport.states[NO_HW_FLOWCTRL])
    manual_rts = toggle_rts = true;
    }
    if ((baud < 19200) && !ascport.force_m1) {
    asc_out(port, ASC_BAUDRATE, (port.uartclk / (16 * baud)));
    } else {
//
// MODE 1: recommended for high bit rates (above 19.2K)
//
// baudrate * 16 * 2^16
// ASCBaudRate =   ------------------------
// inputclock
//
// To keep maths inside 64bits, we divide inputclock by 16.
//
    let mut dividend: u64 = (u64)baud * (1 << 16);
    do_div(dividend, port.uartclk / 16);
    asc_out(port, ASC_BAUDRATE, dividend);
    ctrl_val |= ASC_CTL_BAUDMODE;
    }
    uart_update_timeout(port, cflag, baud);
    ascport.port.read_status_mask = ASC_RXBUF_DUMMY_OE;
    if (termios.c_iflag & INPCK)
    ascport.port.read_status_mask |= ASC_RXBUF_FE | ASC_RXBUF_PE;
    if (termios.c_iflag & (IGNBRK | BRKINT | PARMRK))
    ascport.port.read_status_mask |= ASC_RXBUF_DUMMY_BE;
//
// Characters to ignore
//
    ascport.port.ignore_status_mask = 0;
    if (termios.c_iflag & IGNPAR)
    ascport.port.ignore_status_mask |= ASC_RXBUF_FE | ASC_RXBUF_PE;
    if (termios.c_iflag & IGNBRK) {
    ascport.port.ignore_status_mask |= ASC_RXBUF_DUMMY_BE;
//
// If we're ignoring parity and break indicators,
// ignore overruns too (for real raw support).
//
    if (termios.c_iflag & IGNPAR)
    ascport.port.ignore_status_mask |= ASC_RXBUF_DUMMY_OE;
    }
//
// Ignore all characters if CREAD is not set.
//
    if (!(termios.c_cflag & CREAD))
    ascport.port.ignore_status_mask |= ASC_RXBUF_DUMMY_RX;
// Set the timeout
    asc_out(port, ASC_TIMEOUT, 20);
// write final value and enable port
    asc_out(port, ASC_CTL, (ctrl_val | ASC_CTL_RUN));
    uart_port_unlock_irqrestore(port, flags);
    if (toggle_rts) {
    if (manual_rts) {
    pinctrl_select_state(ascport.pinctrl,
    ascport.states[NO_HW_FLOWCTRL]);
    gpiod = devm_gpiod_get(port.dev, "rts", GPIOD_OUT_LOW);
    if (!IS_ERR(gpiod)) {
    gpiod_set_consumer_name(gpiod,
    port.dev.of_node.name);
    ascport.rts = gpiod;
    }
    } else {
    devm_gpiod_put(port.dev, ascport.rts);
    ascport.rts = core::ptr::null_mut();
    pinctrl_select_state(ascport.pinctrl,
    ascport.states[DEFAULT]);
    }
    }
    }
    static const char *asc_type(struct uart_port *port)
    {
    return (port.type == PORT_ASC) ? DRIVER_NAME : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn asc_release_port(port: *mut uart_port) {
    static void asc_release_port(struct uart_port *port)
    {
    }
#[no_mangle]
unsafe extern "C" fn asc_request_port(port: *mut uart_port) -> c_int {
    static int asc_request_port(struct uart_port *port)
    {
    return 0;
    }
//
// Called when the port is opened, and UPF_BOOT_AUTOCONF flag is set
// Set type field if successful
//
#[no_mangle]
unsafe extern "C" fn asc_config_port(port: *mut uart_port, flags: c_int) {
    static void asc_config_port(struct uart_port *port, int flags)
    {
    if ((flags & UART_CONFIG_TYPE))
    port.type = PORT_ASC;
    }
    static int
    asc_verify_port(struct uart_port *port, struct serial_struct *ser)
    {
// No user changeable parameters
    return -EINVAL;
    }

//
// Console polling routines for writing and reading from the uart while
// in an interrupt or debug context (i.e. kgdb).
//
#[no_mangle]
unsafe extern "C" fn asc_get_poll_char(port: *mut uart_port) -> c_int {
    static int asc_get_poll_char(struct uart_port *port)
    {
    if (!(asc_in(port, ASC_STA) & ASC_STA_RBF))
    return NO_POLL_CHAR;
    return asc_in(port, ASC_RXBUF);
    }
#[no_mangle]
unsafe extern "C" fn asc_put_poll_char(port: *mut uart_port, c: c_uchar) {
    static void asc_put_poll_char(struct uart_port *port, unsigned char c)
    {
    while (!asc_txfifo_is_half_empty(port))
    cpu_relax();
    asc_out(port, ASC_TXBUF, c);
    }

// ---------------------------------------------------------------------
    static const struct uart_ops asc_uart_ops = {
    .tx_empty	= asc_tx_empty,
    .set_mctrl	= asc_set_mctrl,
    .get_mctrl	= asc_get_mctrl,
    .start_tx	= asc_start_tx,
    .stop_tx	= asc_stop_tx,
    .stop_rx	= asc_stop_rx,
    .break_ctl	= asc_break_ctl,
    .startup	= asc_startup,
    .shutdown	= asc_shutdown,
    .set_termios	= asc_set_termios,
    .type		= asc_type,
    .release_port	= asc_release_port,
    .request_port	= asc_request_port,
    .config_port	= asc_config_port,
    .verify_port	= asc_verify_port,
    .pm		= asc_pm,

    .poll_get_char = asc_get_poll_char,
    .poll_put_char = asc_put_poll_char,

    };
    static int asc_init_port(struct asc_port *ascport,
    struct platform_device *pdev)
    {
    struct uart_port *port = &ascport.port;
    struct resource *res;
    int ret;
    port.iotype	= UPIO_MEM;
    port.flags	= UPF_BOOT_AUTOCONF;
    port.ops	= &asc_uart_ops;
    port.fifosize	= ASC_FIFO_SIZE;
    port.dev	= &pdev.dev;
    port.irq	= platform_get_irq(pdev, 0);
    port.has_sysrq = IS_ENABLED(CONFIG_SERIAL_ST_ASC_CONSOLE);
    port.membase = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(port.membase))
    return PTR_ERR(port.membase);
    port.mapbase = res.start;
    spin_lock_init(&port.lock);
    ascport.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (WARN_ON(IS_ERR(ascport.clk)))
    return -EINVAL;
// ensure that clk rate is correct by enabling the clk
    ret = clk_prepare_enable(ascport.clk);
    if (ret)
    return ret;
    ascport.port.uartclk = clk_get_rate(ascport.clk);
    WARN_ON(ascport.port.uartclk == 0);
    clk_disable_unprepare(ascport.clk);
    ascport.pinctrl = devm_pinctrl_get(&pdev.dev);
    if (IS_ERR(ascport.pinctrl)) {
    ret = PTR_ERR(ascport.pinctrl);
    dev_err(&pdev.dev, "Failed to get Pinctrl: %d\n", ret);
    return ret;
    }
    ascport.states[DEFAULT] =
    pinctrl_lookup_state(ascport.pinctrl, "default");
    if (IS_ERR(ascport.states[DEFAULT])) {
    ret = PTR_ERR(ascport.states[DEFAULT]);
    dev_err(&pdev.dev,
    "Failed to look up Pinctrl state 'default': %d\n", ret);
    return ret;
    }
// "no-hw-flowctrl" state is optional
    ascport.states[NO_HW_FLOWCTRL] =
    pinctrl_lookup_state(ascport.pinctrl, "no-hw-flowctrl");
    if (IS_ERR(ascport.states[NO_HW_FLOWCTRL]))
    ascport.states[NO_HW_FLOWCTRL] = core::ptr::null_mut();
    return 0;
    }
    static struct asc_port *asc_of_get_asc_port(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    int id;
    if (!np)
    return core::ptr::null_mut();
    id = of_alias_get_id(np, "serial");
    if (id < 0)
    id = of_alias_get_id(np, ASC_SERIAL_NAME);
    if (id < 0)
    id = 0;
    if (WARN_ON(id >= ASC_MAX_PORTS))
    return core::ptr::null_mut();
    asc_ports[id].hw_flow_control = of_property_read_bool(np,
    "uart-has-rtscts");
    asc_ports[id].force_m1 =  of_property_read_bool(np, "st,force-m1");
    asc_ports[id].port.line = id;
    asc_ports[id].rts = core::ptr::null_mut();
    return &asc_ports[id];
    }

    static const struct of_device_id asc_match[] = {
    { .compatible = "st,asc", },
    {},
    };
    MODULE_DEVICE_TABLE(of, asc_match);

#[no_mangle]
unsafe extern "C" fn asc_serial_probe(pdev: *mut platform_device) -> c_int {
    static int asc_serial_probe(struct platform_device *pdev)
    {
    int ret;
    struct asc_port *ascport;
    ascport = asc_of_get_asc_port(pdev);
    if (!ascport)
    return -ENODEV;
    ret = asc_init_port(ascport, pdev);
    if (ret)
    return ret;
    ret = uart_add_one_port(&asc_uart_driver, &ascport.port);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, &ascport.port);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asc_serial_remove(pdev: *mut platform_device) {
    static void asc_serial_remove(struct platform_device *pdev)
    {
    struct uart_port *port = platform_get_drvdata(pdev);
    uart_remove_one_port(&asc_uart_driver, port);
    }
#[no_mangle]
unsafe extern "C" fn asc_serial_suspend(dev: *mut device) -> c_int {
    static int asc_serial_suspend(struct device *dev)
    {
    struct uart_port *port = dev_get_drvdata(dev);
    return uart_suspend_port(&asc_uart_driver, port);
    }
#[no_mangle]
unsafe extern "C" fn asc_serial_resume(dev: *mut device) -> c_int {
    static int asc_serial_resume(struct device *dev)
    {
    struct uart_port *port = dev_get_drvdata(dev);
    return uart_resume_port(&asc_uart_driver, port);
    }
// ----------------------------------------------------------------------

#[no_mangle]
unsafe extern "C" fn asc_console_putchar(port: *mut uart_port, ch: c_uchar) {
    static void asc_console_putchar(struct uart_port *port, unsigned char ch)
    {
    let mut timeout: c_uint = 1000000;
// Wait for upto 1 second in case flow control is stopping us.
    while (--timeout && !asc_txfifo_is_half_empty(port))
    udelay(1);
    asc_out(port, ASC_TXBUF, ch);
    }
//
// Print a string to the serial port trying not to disturb
// any possible real use of the port...
//
#[no_mangle]
unsafe extern "C" fn asc_console_write(co: *mut console, s: *const c_char, count: unsigned) {
    static void asc_console_write(struct console *co, const char *s, unsigned count)
    {
    struct uart_port *port = &asc_ports[co.index].port;
    unsigned long flags;
    let mut timeout: c_ulong = 1000000;
    let mut locked: c_int = 1;
    u32 intenable;
    if (port.sysrq)
    locked = 0; /* asc_interrupt has already claimed the lock */
#[no_mangle]
pub unsafe extern "C" fn if(_arg: oops_in_progress) -> else {
    else if (oops_in_progress)
    locked = uart_port_trylock_irqsave(port, &flags);
    else
    uart_port_lock_irqsave(port, &flags);
//
// Disable interrupts so we don't get the IRQ line bouncing
// up and down while interrupts are disabled.
//
    intenable = asc_in(port, ASC_INTEN);
    asc_out(port, ASC_INTEN, 0);
    (void)asc_in(port, ASC_INTEN);	/* Defeat bus write posting */
    uart_console_write(port, s, count, asc_console_putchar);
    while (--timeout && !asc_txfifo_is_empty(port))
    udelay(1);
    asc_out(port, ASC_INTEN, intenable);
    if (locked)
    uart_port_unlock_irqrestore(port, flags);
    }
#[no_mangle]
unsafe extern "C" fn asc_console_setup(co: *mut console, options: *mut c_char) -> c_int {
    static int asc_console_setup(struct console *co, char *options)
    {
    struct asc_port *ascport;
    let mut baud: c_int = 115200;
    let mut bits: c_int = 8;
    let mut parity: c_int = 'n';
    let mut flow: c_int = 'n';
    if (co.index >= ASC_MAX_PORTS)
    return -ENODEV;
    ascport = &asc_ports[co.index];
//
// This driver does not support early console initialization
// (use ARM early printk support instead), so we only expect
// this to be called during the uart port registration when the
// driver gets probed and the port should be mapped at that point.
//
    if (ascport.port.mapbase == 0 || ascport.port.membase == core::ptr::null_mut())
    return -ENXIO;
    if (options)
    uart_parse_options(options, &baud, &parity, &bits, &flow);
    return uart_set_options(&ascport.port, co, baud, parity, bits, flow);
    }
    static struct console asc_console = {
    .name		= ASC_SERIAL_NAME,
    .device		= uart_console_device,
    .write		= asc_console_write,
    .setup		= asc_console_setup,
    .flags		= CON_PRINTBUFFER,
    .index		= -1,
    .data		= &asc_uart_driver,
    };

    static struct uart_driver asc_uart_driver = {
    .owner		= THIS_MODULE,
    .driver_name	= DRIVER_NAME,
    .dev_name	= ASC_SERIAL_NAME,
    .major		= 0,
    .minor		= 0,
    .nr		= ASC_MAX_PORTS,
    .cons		= ASC_SERIAL_CONSOLE,
    };
    static DEFINE_SIMPLE_DEV_PM_OPS(asc_serial_pm_ops, asc_serial_suspend,
    asc_serial_resume);
    static struct platform_driver asc_serial_driver = {
    .probe		= asc_serial_probe,
    .remove		= asc_serial_remove,
    .driver	= {
    .name	= DRIVER_NAME,
    .pm	= pm_sleep_ptr(&asc_serial_pm_ops),
    .of_match_table = of_match_ptr(asc_match),
    },
    };
#[no_mangle]
unsafe extern "C" fn asc_init() -> int __init {
    static int __init asc_init(void)
    {
    int ret;
    static const char banner[] __initconst =
    KERN_INFO "STMicroelectronics ASC driver initialized\n";
    printk(banner);
    ret = uart_register_driver(&asc_uart_driver);
    if (ret)
    return ret;
    ret = platform_driver_register(&asc_serial_driver);
    if (ret)
    uart_unregister_driver(&asc_uart_driver);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn asc_exit() -> void __exit {
    static void __exit asc_exit(void)
    {
    platform_driver_unregister(&asc_serial_driver);
    uart_unregister_driver(&asc_uart_driver);
    }
    module_init(asc_init);
    module_exit(asc_exit);
    MODULE_ALIAS("platform:" DRIVER_NAME);
    MODULE_AUTHOR("STMicroelectronics (R&D) Limited");
    MODULE_DESCRIPTION("STMicroelectronics ASC serial port driver");
    MODULE_LICENSE("GPL");

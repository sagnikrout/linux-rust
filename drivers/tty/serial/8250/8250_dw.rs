//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_dw.c
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
// Synopsys DesignWare 8250 driver.
//
// Copyright 2011 Picochip, Jamie Iles.
// Copyright 2013 Intel Corporation
//
// The Synopsys DesignWare 8250 has an extra feature whereby it detects if the
// LCR is written whilst busy.  If it is, then a busy detect interrupt is
// raised, the LCR needs to be rewritten and the uart status register read.
//

pub const OCTEON_UART_USR: c_uint = 0x27 /* UART Status Register */;
pub const RZN1_UART_TDMACR: c_uint = 0x10c /* DMA Control Register Transmit Mode */;
pub const RZN1_UART_RDMACR: c_uint = 0x110 /* DMA Control Register Receive Mode */;
// Renesas specific register fields

// Quirks

//
// Number of consecutive IIR_NO_INT interrupts required to trigger interrupt
// storm prevention code.
//
pub const DW_UART_QUIRK_IER_KICK_THRES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw8250_platform_data {
    pub usr_reg: u8,
    pub cpr_value: u32,
    pub quirks: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw8250_data {
    pub data: dw8250_port_data,
    pub pdata: *const dw8250_platform_data,
    pub msr_mask_on: u32,
    pub msr_mask_off: u32,
    pub clk: *mut clk,
    pub pclk: *mut clk,
    pub rst: *mut reset_control,
    pub skip_autocfg:1: c_uint,
    pub uart_16550_compatible:1: c_uint,
    pub in_idle:1: c_uint,
    pub no_int_count: u8,
}

    static inline struct dw8250_data *to_dw8250_data(struct dw8250_port_data *data)
    {
    return container_of(data, struct dw8250_data, data);
    }
#[no_mangle]
pub unsafe extern "C" fn dw8250_modify_msr(p: *mut uart_port, offset: c_uint, value: u32) -> u32 {
    static inline u32 dw8250_modify_msr(struct uart_port *p, unsigned int offset, u32 value)
    {
    struct dw8250_data *d = to_dw8250_data(p.private_data);
// Override any modem control signals if needed
    if (offset == UART_MSR) {
    value |= d.msr_mask_on;
    value &= ~d.msr_mask_off;
    }
    return value;
    }
#[no_mangle]
unsafe extern "C" fn dw8250_idle_exit(p: *mut uart_port) {
    static void dw8250_idle_exit(struct uart_port *p)
    {
    struct dw8250_data *d = to_dw8250_data(p.private_data);
    struct uart_8250_port *up = up_to_u8250p(p);
    if (d.uart_16550_compatible)
    return;
    if (up.capabilities & UART_CAP_FIFO)
    serial_port_out(p, UART_FCR, up.fcr);
    serial_port_out(p, UART_MCR, up.mcr);
    serial_port_out(p, UART_IER, up.ier);
// DMA Rx is restarted by IRQ handler as needed.
    if (up.dma)
    serial8250_tx_dma_resume(up);
    d.in_idle = 0;
    }
//
// Ensure BUSY is not asserted. If DW UART is configured with
// !uart_16550_compatible, the writes to LCR, DLL, and DLH fail while
// BUSY is asserted.
//
// Context: port's lock must be held
//
#[no_mangle]
unsafe extern "C" fn dw8250_idle_enter(p: *mut uart_port) -> c_int {
    static int dw8250_idle_enter(struct uart_port *p)
    {
    struct dw8250_data *d = to_dw8250_data(p.private_data);
    let mut usr_reg: c_uint = d.pdata ? d.pdata.usr_reg : DW_UART_USR;
    struct uart_8250_port *up = up_to_u8250p(p);
    int retries;
    u32 lsr;
    lockdep_assert_held_once(&p.lock);
    if (d.uart_16550_compatible)
    return 0;
    d.in_idle = 1;
// Prevent triggering interrupt from RBR filling
    serial_port_out(p, UART_IER, 0);
    if (up.dma) {
    serial8250_rx_dma_flush(up);
    if (serial8250_tx_dma_running(up))
    serial8250_tx_dma_pause(up);
    }
//
// Wait until Tx becomes empty + one extra frame time to ensure all bits
// have been sent on the wire.
//
// FIXME: frame_time delay is too long with very low baudrates.
//
    serial8250_fifo_wait_for_lsr_thre(up, core::ptr::null_mut(), p.fifosize);
    ndelay(p.frame_time);
    serial_port_out(p, UART_MCR, up.mcr | UART_MCR_LOOP);
    retries = 4;	/* Arbitrary limit, 2 was always enough in tests */
    do {
    serial8250_clear_fifos(up);
    if (!(serial_port_in(p, usr_reg) & DW_UART_USR_BUSY))
    break;
// FIXME: frame_time delay is too long with very low baudrates.
    ndelay(p.frame_time);
    } while (--retries);
    lsr = serial_lsr_in(up);
    if (lsr & UART_LSR_DR) {
    serial_port_in(p, UART_RX);
    up.lsr_saved_flags = 0;
    }
// Now guaranteed to have BUSY deasserted? Just sanity check
    if (serial_port_in(p, usr_reg) & DW_UART_USR_BUSY) {
    dw8250_idle_exit(p);
    return -EBUSY;
    }
    return 0;
    }
    static void dw8250_set_divisor(struct uart_port *p, unsigned int baud,
    unsigned int quot, unsigned int quot_frac)
    {
    struct uart_8250_port *up = up_to_u8250p(p);
    int ret;
    ret = dw8250_idle_enter(p);
    if (ret < 0)
    return;
    serial_port_out(p, UART_LCR, up.lcr | UART_LCR_DLAB);
    if (!(serial_port_in(p, UART_LCR) & UART_LCR_DLAB))
    goto idle_failed;
    serial_dl_write(up, quot);
    serial_port_out(p, UART_LCR, up.lcr);
    idle_failed:
    dw8250_idle_exit(p);
    }
//
// This function is being called as part of the uart_port::serial_out()
// routine. Hence, special care must be taken when serial_port_out() or
// serial_out() against the modified registers here, i.e. LCR (d->in_idle is
// used to break recursion loop).
//
#[no_mangle]
unsafe extern "C" fn dw8250_check_lcr(p: *mut uart_port, offset: c_uint, value: u32) {
    static void dw8250_check_lcr(struct uart_port *p, unsigned int offset, u32 value)
    {
    struct dw8250_data *d = to_dw8250_data(p.private_data);
    u32 lcr;
    int ret;
    if (offset != UART_LCR || d.uart_16550_compatible)
    return;
    lcr = serial_port_in(p, UART_LCR);
// Make sure LCR write wasn't ignored
    if ((value & ~UART_LCR_SPAR) == (lcr & ~UART_LCR_SPAR))
    return;
    if (d.in_idle)
    goto write_err;
    ret = dw8250_idle_enter(p);
    if (ret < 0)
    goto write_err;
    serial_port_out(p, UART_LCR, value);
    dw8250_idle_exit(p);
    return;
    write_err:
//
// FIXME: this deadlocks if port->lock is already held
// dev_err(p->dev, "Couldn't set LCR to %d\n", value);
//
    return;		/* Silences "label at the end of compound statement" */
    }
//
// With BUSY, LCR writes can be very expensive (IRQ + complex retry logic).
// If the write does not change the value of the LCR register, skip it entirely.
//
#[no_mangle]
unsafe extern "C" fn dw8250_can_skip_reg_write(p: *mut uart_port, offset: c_uint, value: u32) -> bool {
    static bool dw8250_can_skip_reg_write(struct uart_port *p, unsigned int offset, u32 value)
    {
    struct dw8250_data *d = to_dw8250_data(p.private_data);
    u32 lcr;
    if (offset != UART_LCR || d.uart_16550_compatible)
    return false;
    lcr = serial_port_in(p, offset);
    let mut lcr: return = = value;
    }
// Returns once the transmitter is empty or we run out of retries
#[no_mangle]
unsafe extern "C" fn dw8250_tx_wait_empty(p: *mut uart_port) {
    static void dw8250_tx_wait_empty(struct uart_port *p)
    {
    struct uart_8250_port *up = up_to_u8250p(p);
    let mut tries: c_uint = 20000;
    let mut delay_threshold: c_uint = tries - 1000;
    unsigned int lsr;
    while (tries--) {
    lsr = readb (p.membase + (UART_LSR << p.regshift));
    up.lsr_saved_flags |= lsr & up.lsr_save_mask;
    if (lsr & UART_LSR_TEMT)
    break;
// The device is first given a chance to empty without delay,
// to avoid slowdowns at high bitrates. If after 1000 tries
// the buffer has still not emptied, allow more time for low-
// speed links.
    if (tries < delay_threshold)
    udelay (1);
    }
    }
#[no_mangle]
unsafe extern "C" fn dw8250_serial_out(p: *mut uart_port, offset: c_uint, value: u32) {
    static void dw8250_serial_out(struct uart_port *p, unsigned int offset, u32 value)
    {
    if (dw8250_can_skip_reg_write(p, offset, value))
    return;
    writeb(value, p.membase + (offset << p.regshift));
    dw8250_check_lcr(p, offset, value);
    }
#[no_mangle]
unsafe extern "C" fn dw8250_serial_out38x(p: *mut uart_port, offset: c_uint, value: u32) {
    static void dw8250_serial_out38x(struct uart_port *p, unsigned int offset, u32 value)
    {
    if (dw8250_can_skip_reg_write(p, offset, value))
    return;
// Allow the TX to drain before we reconfigure
    if (offset == UART_LCR)
    dw8250_tx_wait_empty(p);
    dw8250_serial_out(p, offset, value);
    }
#[no_mangle]
unsafe extern "C" fn dw8250_serial_in(p: *mut uart_port, offset: c_uint) -> u32 {
    static u32 dw8250_serial_in(struct uart_port *p, unsigned int offset)
    {
    let mut value: u32 = readb(p.membase + (offset << p.regshift));
    return dw8250_modify_msr(p, offset, value);
    }

#[no_mangle]
unsafe extern "C" fn dw8250_serial_inq(p: *mut uart_port, offset: c_uint) -> u32 {
    static u32 dw8250_serial_inq(struct uart_port *p, unsigned int offset)
    {
    let mut value: u8 = __raw_readq(p.membase + (offset << p.regshift));
    return dw8250_modify_msr(p, offset, value);
    }
#[no_mangle]
unsafe extern "C" fn dw8250_serial_outq(p: *mut uart_port, offset: c_uint, value: u32) {
    static void dw8250_serial_outq(struct uart_port *p, unsigned int offset, u32 value)
    {
    if (dw8250_can_skip_reg_write(p, offset, value))
    return;
    value &= 0xff;
    __raw_writeq(value, p.membase + (offset << p.regshift));
// Read back to ensure register write ordering.
    __raw_readq(p.membase + (UART_LCR << p.regshift));
    dw8250_check_lcr(p, offset, value);
    }

#[no_mangle]
unsafe extern "C" fn dw8250_serial_out32(p: *mut uart_port, offset: c_uint, value: u32) {
    static void dw8250_serial_out32(struct uart_port *p, unsigned int offset, u32 value)
    {
    if (dw8250_can_skip_reg_write(p, offset, value))
    return;
    writel(value, p.membase + (offset << p.regshift));
    dw8250_check_lcr(p, offset, value);
    }
#[no_mangle]
unsafe extern "C" fn dw8250_serial_in32(p: *mut uart_port, offset: c_uint) -> u32 {
    static u32 dw8250_serial_in32(struct uart_port *p, unsigned int offset)
    {
    let mut value: u32 = readl(p.membase + (offset << p.regshift));
    return dw8250_modify_msr(p, offset, value);
    }
#[no_mangle]
unsafe extern "C" fn dw8250_serial_out32be(p: *mut uart_port, offset: c_uint, value: u32) {
    static void dw8250_serial_out32be(struct uart_port *p, unsigned int offset, u32 value)
    {
    if (dw8250_can_skip_reg_write(p, offset, value))
    return;
    iowrite32be(value, p.membase + (offset << p.regshift));
    dw8250_check_lcr(p, offset, value);
    }
#[no_mangle]
unsafe extern "C" fn dw8250_serial_in32be(p: *mut uart_port, offset: c_uint) -> u32 {
    static u32 dw8250_serial_in32be(struct uart_port *p, unsigned int offset)
    {
    let mut value: u32 = ioread32be(p.membase + (offset << p.regshift));
    return dw8250_modify_msr(p, offset, value);
    }
//
// INTC10EE UART can IRQ storm while reporting IIR_NO_INT. Inducing IIR value
// change has been observed to break the storm.
//
// If Tx is empty (THRE asserted), we use here IER_THRI to cause IIR_NO_INT ->
// IIR_THRI transition.
//
#[no_mangle]
unsafe extern "C" fn dw8250_quirk_ier_kick(p: *mut uart_port) {
    static void dw8250_quirk_ier_kick(struct uart_port *p)
    {
    struct uart_8250_port *up = up_to_u8250p(p);
    u32 lsr;
    if (up.ier & UART_IER_THRI)
    return;
    lsr = serial_lsr_in(up);
    if (!(lsr & UART_LSR_THRE))
    return;
    serial_port_out(p, UART_IER, up.ier | UART_IER_THRI);
    serial_port_in(p, UART_LCR);		/* safe, no side-effects */
    serial_port_out(p, UART_IER, up.ier);
    }
#[no_mangle]
unsafe extern "C" fn dw8250_handle_irq(p: *mut uart_port) -> c_int {
    static int dw8250_handle_irq(struct uart_port *p)
    {
    struct uart_8250_port *up = up_to_u8250p(p);
    struct dw8250_data *d = to_dw8250_data(p.private_data);
    let mut iir: c_uint = serial_port_in(p, UART_IIR);
    let mut rx_timeout: bool = (iir & 0x3f) == UART_IIR_RX_TIMEOUT;
    let mut quirks: c_uint = d.pdata.quirks;
    unsigned int status;
    guard(uart_port_lock_check_sysrq_irqsave)(p);
    switch (FIELD_GET(DW_UART_IIR_IID, iir)) {
    case UART_IIR_NO_INT:
    if (d.uart_16550_compatible || up.dma)
    return 0;
    if (quirks & DW_UART_QUIRK_IER_KICK &&
    d.no_int_count == (DW_UART_QUIRK_IER_KICK_THRES - 1))
    dw8250_quirk_ier_kick(p);
    d.no_int_count = (d.no_int_count + 1) % DW_UART_QUIRK_IER_KICK_THRES;
    return 0;
    case UART_IIR_BUSY:
// Clear the USR
    serial_port_in(p, d.pdata.usr_reg);
    d.no_int_count = 0;
    return 1;
    }
    d.no_int_count = 0;
//
// There are ways to get Designware-based UARTs into a state where
// they are asserting UART_IIR_RX_TIMEOUT but there is no actual
// data available.  If we see such a case then we'll do a bogus
// read.  If we don't do this then the "RX TIMEOUT" interrupt will
// fire forever.
//
// This problem has only been observed so far when not in DMA mode
// so we limit the workaround only to non-DMA mode.
//
    if (!up.dma && rx_timeout) {
    status = serial_lsr_in(up);
    if (!(status & (UART_LSR_DR | UART_LSR_BI)))
    serial_port_in(p, UART_RX);
    }
// Manually stop the Rx DMA transfer when acting as flow controller
    if (quirks & DW_UART_QUIRK_IS_DMA_FC && up.dma && up.dma.rx_running && rx_timeout) {
    status = serial_lsr_in(up);
    if (status & (UART_LSR_DR | UART_LSR_BI)) {
    dw8250_writel_ext(p, RZN1_UART_RDMACR, 0);
    dw8250_writel_ext(p, DW_UART_DMASA, 1);
    }
    }
    serial8250_handle_irq_locked(p, iir);
    return 1;
    }
    static void
    dw8250_do_pm(struct uart_port *port, unsigned int state, unsigned int old)
    {
    if (!state)
    pm_runtime_get_sync(port.dev);
    serial8250_do_pm(port, state, old);
    if (state)
    pm_runtime_put_sync_suspend(port.dev);
    }
    static void dw8250_set_termios(struct uart_port *p, struct ktermios *termios,
    const struct ktermios *old)
    {
    let mut newrate: c_ulong = tty_termios_baud_rate(termios) * 16;
    struct dw8250_data *d = to_dw8250_data(p.private_data);
    long rate;
    int ret;
    clk_disable_unprepare(d.clk);
    rate = clk_round_rate(d.clk, newrate);
    if (rate > 0) {
    ret = clk_set_rate(d.clk, newrate);
    if (!ret)
    p.uartclk = rate;
    }
    clk_prepare_enable(d.clk);
    dw8250_do_set_termios(p, termios, old);
    }
#[no_mangle]
unsafe extern "C" fn dw8250_set_ldisc(p: *mut uart_port, termios: *mut ktermios) {
    static void dw8250_set_ldisc(struct uart_port *p, struct ktermios *termios)
    {
    struct uart_8250_port *up = up_to_u8250p(p);
    let mut mcr: c_uint = serial_port_in(p, UART_MCR);
    if (up.capabilities & UART_CAP_IRDA) {
    if (termios.c_line == N_IRDA)
    mcr |= DW_UART_MCR_SIRE;
    else
    mcr &= ~DW_UART_MCR_SIRE;
    serial_port_out(p, UART_MCR, mcr);
    }
    serial8250_do_set_ldisc(p, termios);
    }
//
// dw8250_fallback_dma_filter will prevent the UART from getting just any free
// channel on platforms that have DMA engines, but don't have any channels
// assigned to the UART.
//
// REVISIT: This is a work around for limitation in the DMA Engine API. Once the
// core problem is fixed, this function is no longer needed.
//
#[no_mangle]
unsafe extern "C" fn dw8250_fallback_dma_filter(chan: *mut dma_chan, param: *mut c_void) -> bool {
    static bool dw8250_fallback_dma_filter(struct dma_chan *chan, void *param)
    {
    return false;
    }
#[no_mangle]
unsafe extern "C" fn dw8250_idma_filter(chan: *mut dma_chan, param: *mut c_void) -> bool {
    static bool dw8250_idma_filter(struct dma_chan *chan, void *param)
    {
    let mut param: return = = chan.device.dev;
    }
#[no_mangle]
unsafe extern "C" fn dw8250_setup_dma_filter(p: *mut uart_port, data: *mut dw8250_data) {
    static void dw8250_setup_dma_filter(struct uart_port *p, struct dw8250_data *data)
    {
// Platforms with iDMA 64-bit
    if (platform_get_resource_byname(to_platform_device(p.dev), IORESOURCE_MEM, "lpss_priv")) {
    data.data.dma.rx_param = p.dev.parent;
    data.data.dma.tx_param = p.dev.parent;
    data.data.dma.fn = dw8250_idma_filter;
    } else {
    data.data.dma.fn = dw8250_fallback_dma_filter;
    }
    }
#[no_mangle]
unsafe extern "C" fn dw8250_rzn1_get_dmacr_burst(max_burst: c_int) -> u32 {
    static u32 dw8250_rzn1_get_dmacr_burst(int max_burst)
    {
    if (max_burst >= 8)
    return RZN1_UART_xDMACR_8_WORD_BURST;
#[no_mangle]
pub unsafe extern "C" fn if(4: max_burst >=) -> else {
    else if (max_burst >= 4)
    return RZN1_UART_xDMACR_4_WORD_BURST;
    else
    return RZN1_UART_xDMACR_1_WORD_BURST;
    }
#[no_mangle]
unsafe extern "C" fn dw8250_prepare_tx_dma(p: *mut uart_8250_port) {
    static void dw8250_prepare_tx_dma(struct uart_8250_port *p)
    {
    struct uart_port *up = &p.port;
    struct uart_8250_dma *dma = p.dma;
    u32 val;
    dw8250_writel_ext(up, RZN1_UART_TDMACR, 0);
    val = dw8250_rzn1_get_dmacr_burst(dma.txconf.dst_maxburst) |
    RZN1_UART_xDMACR_BLK_SZ(dma.tx_size) |
    RZN1_UART_xDMACR_DMA_EN;
    dw8250_writel_ext(up, RZN1_UART_TDMACR, val);
    }
#[no_mangle]
unsafe extern "C" fn dw8250_prepare_rx_dma(p: *mut uart_8250_port) {
    static void dw8250_prepare_rx_dma(struct uart_8250_port *p)
    {
    struct uart_port *up = &p.port;
    struct uart_8250_dma *dma = p.dma;
    u32 val;
    dw8250_writel_ext(up, RZN1_UART_RDMACR, 0);
    val = dw8250_rzn1_get_dmacr_burst(dma.rxconf.src_maxburst) |
    RZN1_UART_xDMACR_BLK_SZ(dma.rx_size) |
    RZN1_UART_xDMACR_DMA_EN;
    dw8250_writel_ext(up, RZN1_UART_RDMACR, val);
    }
#[no_mangle]
unsafe extern "C" fn dw8250_quirks(p: *mut uart_port, data: *mut dw8250_data) {
    static void dw8250_quirks(struct uart_port *p, struct dw8250_data *data)
    {
    let mut quirks: c_uint = data.pdata.quirks;
    let mut cpr_value: u32 = data.pdata.cpr_value;
    if (quirks & DW_UART_QUIRK_CPR_VALUE)
    data.data.cpr_value = cpr_value;

    if (quirks & DW_UART_QUIRK_OCTEON) {
    p.serial_in = dw8250_serial_inq;
    p.serial_out = dw8250_serial_outq;
    p.flags = UPF_SKIP_TEST | UPF_SHARE_IRQ | UPF_FIXED_TYPE;
    p.type = PORT_OCTEON;
    data.skip_autocfg = true;
    }

    if (quirks & DW_UART_QUIRK_ARMADA_38X)
    p.serial_out = dw8250_serial_out38x;
    if (quirks & DW_UART_QUIRK_SKIP_SET_RATE)
    p.set_termios = dw8250_do_set_termios;
    if (quirks & DW_UART_QUIRK_IS_DMA_FC) {
    data.data.dma.txconf.device_fc = 1;
    data.data.dma.rxconf.device_fc = 1;
    data.data.dma.prepare_tx_dma = dw8250_prepare_tx_dma;
    data.data.dma.prepare_rx_dma = dw8250_prepare_rx_dma;
    }
    if (quirks & DW_UART_QUIRK_APMC0D08) {
    p.iotype = UPIO_MEM32;
    p.regshift = 2;
    p.serial_in = dw8250_serial_in32;
    data.uart_16550_compatible = true;
    }
    }
#[no_mangle]
unsafe extern "C" fn dw8250_reset_control_assert(data: *mut c_void) {
    static void dw8250_reset_control_assert(void *data)
    {
    reset_control_assert(data);
    }
#[no_mangle]
unsafe extern "C" fn dw8250_shutdown(port: *mut uart_port) {
    static void dw8250_shutdown(struct uart_port *port)
    {
    struct dw8250_data *d = to_dw8250_data(port.private_data);
    serial8250_do_shutdown(port);
    d.no_int_count = 0;
    }
#[no_mangle]
unsafe extern "C" fn dw8250_probe(pdev: *mut platform_device) -> c_int {
    static int dw8250_probe(struct platform_device *pdev)
    {
    let mut uart: uart_8250_port = {}, *up = &uart;
    struct uart_port *p = &up.port;
    struct device *dev = &pdev.dev;
    struct dw8250_data *data;
    struct resource *regs;
    int err;
    regs = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!regs)
    return dev_err_probe(dev, -EINVAL, "no registers defined\n");
    spin_lock_init(&p.lock);
    p.pm		= dw8250_do_pm;
    p.type		= PORT_8250;
    p.flags	= UPF_FIXED_PORT;
    p.dev		= dev;
    p.set_ldisc	= dw8250_set_ldisc;
    p.set_termios	= dw8250_set_termios;
    p.set_divisor	= dw8250_set_divisor;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    p.private_data = &data.data;
    p.mapbase = regs.start;
    p.mapsize = resource_size(regs);
    p.membase = devm_ioremap(dev, p.mapbase, p.mapsize);
    if (!p.membase)
    return -ENOMEM;
    err = uart_read_port_properties(p);
// no interrupt -> fall back to polling
    if (err == -ENXIO)
    err = 0;
    if (err)
    return err;
    switch (p.iotype) {
    case UPIO_MEM:
    p.serial_in = dw8250_serial_in;
    p.serial_out = dw8250_serial_out;
    break;
    case UPIO_MEM32:
    p.serial_in = dw8250_serial_in32;
    p.serial_out = dw8250_serial_out32;
    break;
    case UPIO_MEM32BE:
    p.serial_in = dw8250_serial_in32be;
    p.serial_out = dw8250_serial_out32be;
    break;
    default:
    return -ENODEV;
    }
    if (device_property_read_bool(dev, "dcd-override")) {
// Always report DCD as active
    data.msr_mask_on |= UART_MSR_DCD;
    data.msr_mask_off |= UART_MSR_DDCD;
    }
    if (device_property_read_bool(dev, "dsr-override")) {
// Always report DSR as active
    data.msr_mask_on |= UART_MSR_DSR;
    data.msr_mask_off |= UART_MSR_DDSR;
    }
    if (device_property_read_bool(dev, "cts-override")) {
// Always report CTS as active
    data.msr_mask_on |= UART_MSR_CTS;
    data.msr_mask_off |= UART_MSR_DCTS;
    }
    if (device_property_read_bool(dev, "ri-override")) {
// Always report Ring indicator as inactive
    data.msr_mask_off |= UART_MSR_RI;
    data.msr_mask_off |= UART_MSR_TERI;
    }
// If there is separate baudclk, get the rate from it.
    data.clk = devm_clk_get_optional_enabled(dev, "baudclk");
    if (data.clk == core::ptr::null_mut())
    data.clk = devm_clk_get_optional_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(data.clk))
    return dev_err_probe(dev, PTR_ERR(data.clk),
    "failed to get baudclk\n");
    if (data.clk)
    p.uartclk = clk_get_rate(data.clk);
// If no clock rate is defined, fail.
    if (!p.uartclk)
    return dev_err_probe(dev, -EINVAL, "clock rate not defined\n");
    data.pclk = devm_clk_get_optional_enabled(dev, "apb_pclk");
    if (IS_ERR(data.pclk))
    return PTR_ERR(data.pclk);
    data.rst = devm_reset_control_array_get_optional_exclusive(dev);
    if (IS_ERR(data.rst))
    return PTR_ERR(data.rst);
    err = reset_control_deassert(data.rst);
    if (err)
    return dev_err_probe(dev, err, "failed to deassert resets\n");
    err = devm_add_action_or_reset(dev, dw8250_reset_control_assert, data.rst);
    if (err)
    return err;
    err = pm_runtime_set_active(dev);
    if (err)
    return dev_err_probe(dev, err, "Failed to set the runtime suspend as active\n");
    data.uart_16550_compatible = device_property_read_bool(dev, "snps,uart-16550-compatible");
    data.pdata = device_get_match_data(p.dev);
    if (data.pdata)
    dw8250_quirks(p, data);
// If the Busy Functionality is not implemented, don't handle it
    if (data.uart_16550_compatible) {
    p.handle_irq = core::ptr::null_mut();
    } else if (data.pdata) {
    p.handle_irq = dw8250_handle_irq;
    p.shutdown = dw8250_shutdown;
    }
    dw8250_setup_dma_filter(p, data);
    if (!data.skip_autocfg)
    dw8250_setup_port(p);
// If we have a valid fifosize, try hooking up DMA
    if (p.fifosize) {
    data.data.dma.rxconf.src_maxburst = p.fifosize / 4;
    data.data.dma.txconf.dst_maxburst = p.fifosize / 4;
    up.dma = &data.data.dma;
    }
    data.data.line = serial8250_register_8250_port(up);
    if (data.data.line < 0)
    return data.data.line;
    platform_set_drvdata(pdev, data);
    pm_runtime_enable(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw8250_remove(pdev: *mut platform_device) {
    static void dw8250_remove(struct platform_device *pdev)
    {
    struct dw8250_data *data = platform_get_drvdata(pdev);
    struct device *dev = &pdev.dev;
    pm_runtime_get_sync(dev);
    serial8250_unregister_port(data.data.line);
    pm_runtime_disable(dev);
    pm_runtime_put_noidle(dev);
    }
#[no_mangle]
unsafe extern "C" fn dw8250_suspend(dev: *mut device) -> c_int {
    static int dw8250_suspend(struct device *dev)
    {
    struct dw8250_data *data = dev_get_drvdata(dev);
    serial8250_suspend_port(data.data.line);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw8250_resume(dev: *mut device) -> c_int {
    static int dw8250_resume(struct device *dev)
    {
    struct dw8250_data *data = dev_get_drvdata(dev);
    serial8250_resume_port(data.data.line);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw8250_runtime_suspend(dev: *mut device) -> c_int {
    static int dw8250_runtime_suspend(struct device *dev)
    {
    struct dw8250_data *data = dev_get_drvdata(dev);
    clk_disable_unprepare(data.clk);
    clk_disable_unprepare(data.pclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw8250_runtime_resume(dev: *mut device) -> c_int {
    static int dw8250_runtime_resume(struct device *dev)
    {
    int ret;
    struct dw8250_data *data = dev_get_drvdata(dev);
    ret = clk_prepare_enable(data.pclk);
    if (ret)
    return ret;
    ret = clk_prepare_enable(data.clk);
    if (ret) {
    clk_disable_unprepare(data.pclk);
    return ret;
    }
    return 0;
    }
    static _DEFINE_DEV_PM_OPS(dw8250_pm_ops, dw8250_suspend, dw8250_resume,
    dw8250_runtime_suspend, dw8250_runtime_resume,
    core::ptr::null_mut());
    static const struct dw8250_platform_data dw8250_dw_apb = {
    .usr_reg = DW_UART_USR,
    };
    static const struct dw8250_platform_data dw8250_octeon_3860_data = {
    .usr_reg = OCTEON_UART_USR,
    .quirks = DW_UART_QUIRK_OCTEON,
    };
    static const struct dw8250_platform_data dw8250_armada_38x_data = {
    .usr_reg = DW_UART_USR,
    .quirks = DW_UART_QUIRK_ARMADA_38X,
    };
    static const struct dw8250_platform_data dw8250_renesas_rzn1_data = {
    .usr_reg = DW_UART_USR,
    .cpr_value = FIELD_PREP_CONST(DW_UART_CPR_ABP_DATA_WIDTH, 2) |
    DW_UART_CPR_AFCE_MODE |
    DW_UART_CPR_THRE_MODE |
    DW_UART_CPR_ADDITIONAL_FEATURES |
    DW_UART_CPR_FIFO_ACCESS |
    DW_UART_CPR_FIFO_STAT |
    DW_UART_CPR_SHADOW |
    DW_UART_CPR_DMA_EXTRA |
    DW_UART_CPR_FIFO_MODE_FROM_SIZE(16),
    .quirks = DW_UART_QUIRK_CPR_VALUE | DW_UART_QUIRK_IS_DMA_FC,
    };
    static const struct dw8250_platform_data dw8250_skip_set_rate_data = {
    .usr_reg = DW_UART_USR,
    .quirks = DW_UART_QUIRK_SKIP_SET_RATE,
    };
    static const struct dw8250_platform_data dw8250_intc10ee = {
    .usr_reg = DW_UART_USR,
    .quirks = DW_UART_QUIRK_IER_KICK,
    };
    static const struct dw8250_platform_data dw8250_ultrarisc_dp1000_data = {
    .usr_reg = DW_UART_USR,
    .cpr_value = FIELD_PREP_CONST(DW_UART_CPR_ABP_DATA_WIDTH, 2) |
    DW_UART_CPR_THRE_MODE |
    DW_UART_CPR_DMA_EXTRA |
    DW_UART_CPR_FIFO_MODE_FROM_SIZE(32),
    .quirks = DW_UART_QUIRK_CPR_VALUE,
    };
    static const struct of_device_id dw8250_of_match[] = {
    { .compatible = "snps,dw-apb-uart", .data = &dw8250_dw_apb },
    { .compatible = "cavium,octeon-3860-uart", .data = &dw8250_octeon_3860_data },
    { .compatible = "marvell,armada-38x-uart", .data = &dw8250_armada_38x_data },
    { .compatible = "renesas,rzn1-uart", .data = &dw8250_renesas_rzn1_data },
    { .compatible = "sophgo,sg2044-uart", .data = &dw8250_skip_set_rate_data },
    { .compatible = "starfive,jh7100-uart", .data = &dw8250_skip_set_rate_data },
    { .compatible = "ultrarisc,dp1000-uart", .data = &dw8250_ultrarisc_dp1000_data },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, dw8250_of_match);
    static const struct dw8250_platform_data dw8250_apmc0d08 = {
    .usr_reg = DW_UART_USR,
    .quirks = DW_UART_QUIRK_APMC0D08,
    };
    static const struct acpi_device_id dw8250_acpi_match[] = {
    { "80860F0A", (kernel_ulong_t)&dw8250_dw_apb },
    { "8086228A", (kernel_ulong_t)&dw8250_dw_apb },
    { "AMD0020", (kernel_ulong_t)&dw8250_dw_apb },
    { "AMDI0020", (kernel_ulong_t)&dw8250_dw_apb },
    { "AMDI0022", (kernel_ulong_t)&dw8250_dw_apb },
    { "APMC0D08", (kernel_ulong_t)&dw8250_apmc0d08 },
    { "BRCM2032", (kernel_ulong_t)&dw8250_dw_apb },
    { "HISI0031", (kernel_ulong_t)&dw8250_dw_apb },
    { "INT33C4", (kernel_ulong_t)&dw8250_dw_apb },
    { "INT33C5", (kernel_ulong_t)&dw8250_dw_apb },
    { "INT3434", (kernel_ulong_t)&dw8250_dw_apb },
    { "INT3435", (kernel_ulong_t)&dw8250_dw_apb },
    { "INTC10EE", (kernel_ulong_t)&dw8250_intc10ee },
    { },
    };
    MODULE_DEVICE_TABLE(acpi, dw8250_acpi_match);
    static struct platform_driver dw8250_platform_driver = {
    .driver = {
    .name		= "dw-apb-uart",
    .pm		= pm_ptr(&dw8250_pm_ops),
    .of_match_table	= dw8250_of_match,
    .acpi_match_table = dw8250_acpi_match,
    },
    .probe			= dw8250_probe,
    .remove			= dw8250_remove,
    };
    module_platform_driver(dw8250_platform_driver);
    MODULE_IMPORT_NS("SERIAL_8250");
    MODULE_AUTHOR("Jamie Iles");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Synopsys DesignWare 8250 serial port driver");
    MODULE_ALIAS("platform:dw-apb-uart");

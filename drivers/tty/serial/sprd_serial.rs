//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/sprd_serial.c
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
// Copyright (C) 2012-2015 Spreadtrum Communications Inc.
//

// device name
pub const UART_NR_MAX: c_int = 8;

pub const SPRD_FIFO_SIZE: c_int = 128;
pub const SPRD_DEF_RATE: c_int = 26000000;
pub const SPRD_BAUD_IO_LIMIT: c_int = 3000000;
pub const SPRD_TIMEOUT: c_int = 256000;
// the offset of serial registers and BITs for them
// data registers
pub const SPRD_TXD: c_uint = 0x0000;
pub const SPRD_RXD: c_uint = 0x0004;
// line status register and its BITs
pub const SPRD_LSR: c_uint = 0x0008;

// data number in TX and RX fifo
pub const SPRD_STS1: c_uint = 0x000C;

// interrupt enable register and its BITs
pub const SPRD_IEN: c_uint = 0x0010;

// interrupt clear register
pub const SPRD_ICLR: c_uint = 0x0014;

// line control register
pub const SPRD_LCR: c_uint = 0x0018;
pub const SPRD_LCR_STOP_1BIT: c_uint = 0x10;
pub const SPRD_LCR_STOP_2BIT: c_uint = 0x30;

pub const SPRD_LCR_DATA_LEN5: c_uint = 0x0;
pub const SPRD_LCR_DATA_LEN6: c_uint = 0x4;
pub const SPRD_LCR_DATA_LEN7: c_uint = 0x8;
pub const SPRD_LCR_DATA_LEN8: c_uint = 0xc;

pub const SPRD_LCR_PARITY_EN: c_uint = 0x2;
pub const SPRD_LCR_EVEN_PAR: c_uint = 0x0;
pub const SPRD_LCR_ODD_PAR: c_uint = 0x1;
// control register 1
pub const SPRD_CTL1: c_uint = 0x001C;

pub const RX_TOUT_THLD_DEF: c_uint = 0x3E00;
pub const RX_HFC_THLD_DEF: c_uint = 0x40;
// fifo threshold register
pub const SPRD_CTL2: c_uint = 0x0020;
pub const THLD_TX_EMPTY: c_uint = 0x40;
pub const THLD_TX_EMPTY_SHIFT: c_int = 8;
pub const THLD_RX_FULL: c_uint = 0x40;

// config baud rate register
pub const SPRD_CLKD0: c_uint = 0x0024;

pub const SPRD_CLKD1: c_uint = 0x0028;

pub const SPRD_CLKD1_SHIFT: c_int = 16;
// interrupt mask status register
pub const SPRD_IMSR: c_uint = 0x002C;

pub const SPRD_DEFAULT_SOURCE_CLK: c_int = 26000000;
pub const SPRD_RX_DMA_STEP: c_int = 1;
pub const SPRD_RX_FIFO_FULL: c_int = 1;
pub const SPRD_TX_FIFO_FULL: c_uint = 0x20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_uart_dma {
    pub chn: *mut dma_chan,
    pub virt: *mut c_uchar,
    pub phys_addr: dma_addr_t,
    pub cookie: dma_cookie_t,
    pub trans_len: u32,
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_uart_data {
    pub timeout_ien: c_uint,
    pub timeout_iclr: c_uint,
    pub timeout_imsr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_uart_port {
    pub port: uart_port,
    pub name: [c_char; 16],
    pub clk: *mut clk,
    pub tx_dma: sprd_uart_dma,
    pub rx_dma: sprd_uart_dma,
    pub pos: dma_addr_t,
    pub rx_buf_tail: *mut c_uchar,
    pub pdata: *const sprd_uart_data,
}

    static struct sprd_uart_port *sprd_port[UART_NR_MAX];
    static int sprd_ports_num;
    static int sprd_start_dma_rx(struct uart_port *port);
    static int sprd_tx_dma_config(struct uart_port *port);
    static const struct sprd_uart_data sc9836_data = {
    .timeout_ien = SPRD_IEN_TIMEOUT,
    .timeout_iclr = SPRD_ICLR_TIMEOUT,
    .timeout_imsr = SPRD_IMSR_TIMEOUT,
    };
    static const struct sprd_uart_data sc9632_data = {
    .timeout_ien = SPRD_IEN_DATA_TIMEOUT,
    .timeout_iclr = SPRD_ICLR_DATA_TIMEOUT,
    .timeout_imsr = SPRD_IMSR_DATA_TIMEOUT,
    };
    static inline unsigned int serial_in(struct uart_port *port,
    unsigned int offset)
    {
    return readl_relaxed(port.membase + offset);
    }
    static inline void serial_out(struct uart_port *port, unsigned int offset,
    int value)
    {
    writel_relaxed(value, port.membase + offset);
    }
#[no_mangle]
unsafe extern "C" fn sprd_tx_empty(port: *mut uart_port) -> c_uint {
    static unsigned int sprd_tx_empty(struct uart_port *port)
    {
    if (serial_in(port, SPRD_STS1) & SPRD_TX_FIFO_CNT_MASK)
    return 0;
    else
    return TIOCSER_TEMT;
    }
#[no_mangle]
unsafe extern "C" fn sprd_get_mctrl(port: *mut uart_port) -> c_uint {
    static unsigned int sprd_get_mctrl(struct uart_port *port)
    {
    return TIOCM_DSR | TIOCM_CTS;
    }
#[no_mangle]
unsafe extern "C" fn sprd_set_mctrl(port: *mut uart_port, mctrl: c_uint) {
    static void sprd_set_mctrl(struct uart_port *port, unsigned int mctrl)
    {
    let mut val: u32 = serial_in(port, SPRD_CTL1);
    if (mctrl & TIOCM_LOOP)
    val |= SPRD_LOOPBACK_EN;
    else
    val &= ~SPRD_LOOPBACK_EN;
    serial_out(port, SPRD_CTL1, val);
    }
#[no_mangle]
unsafe extern "C" fn sprd_stop_rx(port: *mut uart_port) {
    static void sprd_stop_rx(struct uart_port *port)
    {
    struct sprd_uart_port *sp =
    container_of(port, struct sprd_uart_port, port);
    unsigned int ien, iclr;
    if (sp.rx_dma.enable)
    dmaengine_terminate_all(sp.rx_dma.chn);
    iclr = serial_in(port, SPRD_ICLR);
    ien = serial_in(port, SPRD_IEN);
    ien &= ~(SPRD_IEN_RX_FULL | SPRD_IEN_BREAK_DETECT);
    iclr |= SPRD_IEN_RX_FULL | SPRD_IEN_BREAK_DETECT;
    serial_out(port, SPRD_IEN, ien);
    serial_out(port, SPRD_ICLR, iclr);
    }
#[no_mangle]
unsafe extern "C" fn sprd_uart_dma_enable(port: *mut uart_port, enable: bool) {
    static void sprd_uart_dma_enable(struct uart_port *port, bool enable)
    {
    let mut val: u32 = serial_in(port, SPRD_CTL1);
    if (enable)
    val |= SPRD_DMA_EN;
    else
    val &= ~SPRD_DMA_EN;
    serial_out(port, SPRD_CTL1, val);
    }
#[no_mangle]
unsafe extern "C" fn sprd_stop_tx_dma(port: *mut uart_port) {
    static void sprd_stop_tx_dma(struct uart_port *port)
    {
    struct sprd_uart_port *sp =
    container_of(port, struct sprd_uart_port, port);
    struct dma_tx_state state;
    u32 trans_len;
    dmaengine_pause(sp.tx_dma.chn);
    dmaengine_tx_status(sp.tx_dma.chn, sp.tx_dma.cookie, &state);
    if (state.residue) {
    trans_len = state.residue - sp.tx_dma.phys_addr;
    uart_xmit_advance(port, trans_len);
    dma_unmap_single(port.dev, sp.tx_dma.phys_addr,
    sp.tx_dma.trans_len, DMA_TO_DEVICE);
    }
    dmaengine_terminate_all(sp.tx_dma.chn);
    sp.tx_dma.trans_len = 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_tx_buf_remap(port: *mut uart_port) -> c_int {
    static int sprd_tx_buf_remap(struct uart_port *port)
    {
    struct sprd_uart_port *sp =
    container_of(port, struct sprd_uart_port, port);
    struct tty_port *tport = &port.state.port;
    unsigned char *tail;
    sp.tx_dma.trans_len = kfifo_out_linear_ptr(&tport.xmit_fifo, &tail,
    UART_XMIT_SIZE);
    sp.tx_dma.phys_addr = dma_map_single(port.dev, tail,
    sp.tx_dma.trans_len,
    DMA_TO_DEVICE);
    return dma_mapping_error(port.dev, sp.tx_dma.phys_addr);
    }
#[no_mangle]
unsafe extern "C" fn sprd_complete_tx_dma(data: *mut c_void) {
    static void sprd_complete_tx_dma(void *data)
    {
    struct uart_port *port = (struct uart_port *)data;
    struct sprd_uart_port *sp =
    container_of(port, struct sprd_uart_port, port);
    struct tty_port *tport = &port.state.port;
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
    dma_unmap_single(port.dev, sp.tx_dma.phys_addr,
    sp.tx_dma.trans_len, DMA_TO_DEVICE);
    uart_xmit_advance(port, sp.tx_dma.trans_len);
    if (kfifo_len(&tport.xmit_fifo) < WAKEUP_CHARS)
    uart_write_wakeup(port);
    if (kfifo_is_empty(&tport.xmit_fifo) || sprd_tx_buf_remap(port) ||
    sprd_tx_dma_config(port))
    sp.tx_dma.trans_len = 0;
    uart_port_unlock_irqrestore(port, flags);
    }
    static int sprd_uart_dma_submit(struct uart_port *port,
    struct sprd_uart_dma *ud, u32 trans_len,
    enum dma_transfer_direction direction,
    dma_async_tx_callback callback)
    {
    struct dma_async_tx_descriptor *dma_des;
    unsigned long flags;
    flags = SPRD_DMA_FLAGS(SPRD_DMA_CHN_MODE_NONE,
    SPRD_DMA_NO_TRG,
    SPRD_DMA_FRAG_REQ,
    SPRD_DMA_TRANS_INT);
    dma_des = dmaengine_prep_slave_single(ud.chn, ud.phys_addr, trans_len,
    direction, flags);
    if (!dma_des)
    return -ENODEV;
    dma_des.callback = callback;
    dma_des.callback_param = port;
    ud.cookie = dmaengine_submit(dma_des);
    if (dma_submit_error(ud.cookie))
    return dma_submit_error(ud.cookie);
    dma_async_issue_pending(ud.chn);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_tx_dma_config(port: *mut uart_port) -> c_int {
    static int sprd_tx_dma_config(struct uart_port *port)
    {
    struct sprd_uart_port *sp =
    container_of(port, struct sprd_uart_port, port);
    u32 burst = sp.tx_dma.trans_len > SPRD_TX_FIFO_FULL ?
    SPRD_TX_FIFO_FULL : sp.tx_dma.trans_len;
    int ret;
    struct dma_slave_config cfg = {
    .dst_addr = port.mapbase + SPRD_TXD,
    .src_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE,
    .dst_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE,
    .src_maxburst = burst,
    };
    ret = dmaengine_slave_config(sp.tx_dma.chn, &cfg);
    if (ret < 0)
    return ret;
    return sprd_uart_dma_submit(port, &sp.tx_dma, sp.tx_dma.trans_len,
    DMA_MEM_TO_DEV, sprd_complete_tx_dma);
    }
#[no_mangle]
unsafe extern "C" fn sprd_start_tx_dma(port: *mut uart_port) {
    static void sprd_start_tx_dma(struct uart_port *port)
    {
    struct sprd_uart_port *sp =
    container_of(port, struct sprd_uart_port, port);
    struct tty_port *tport = &port.state.port;
    if (port.x_char) {
    serial_out(port, SPRD_TXD, port.x_char);
    port.icount.tx++;
    port.x_char = 0;
    return;
    }
    if (kfifo_is_empty(&tport.xmit_fifo) || uart_tx_stopped(port)) {
    sprd_stop_tx_dma(port);
    return;
    }
    if (sp.tx_dma.trans_len)
    return;
    if (sprd_tx_buf_remap(port) || sprd_tx_dma_config(port))
    sp.tx_dma.trans_len = 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_rx_full_thld(port: *mut uart_port, thld: u32) {
    static void sprd_rx_full_thld(struct uart_port *port, u32 thld)
    {
    let mut val: u32 = serial_in(port, SPRD_CTL2);
    val &= ~THLD_RX_FULL_MASK;
    val |= thld & THLD_RX_FULL_MASK;
    serial_out(port, SPRD_CTL2, val);
    }
#[no_mangle]
unsafe extern "C" fn sprd_rx_alloc_buf(sp: *mut sprd_uart_port) -> c_int {
    static int sprd_rx_alloc_buf(struct sprd_uart_port *sp)
    {
    sp.rx_dma.virt = dma_alloc_coherent(sp.port.dev, SPRD_UART_RX_SIZE,
    &sp.rx_dma.phys_addr, GFP_KERNEL);
    if (!sp.rx_dma.virt)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_rx_free_buf(sp: *mut sprd_uart_port) {
    static void sprd_rx_free_buf(struct sprd_uart_port *sp)
    {
    if (sp.rx_dma.virt)
    dma_free_coherent(sp.port.dev, SPRD_UART_RX_SIZE,
    sp.rx_dma.virt, sp.rx_dma.phys_addr);
    sp.rx_dma.virt = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn sprd_rx_dma_config(port: *mut uart_port, burst: u32) -> c_int {
    static int sprd_rx_dma_config(struct uart_port *port, u32 burst)
    {
    struct sprd_uart_port *sp =
    container_of(port, struct sprd_uart_port, port);
    struct dma_slave_config cfg = {
    .src_addr = port.mapbase + SPRD_RXD,
    .src_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE,
    .dst_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE,
    .src_maxburst = burst,
    };
    return dmaengine_slave_config(sp.rx_dma.chn, &cfg);
    }
#[no_mangle]
unsafe extern "C" fn sprd_uart_dma_rx(port: *mut uart_port) {
    static void sprd_uart_dma_rx(struct uart_port *port)
    {
    struct sprd_uart_port *sp =
    container_of(port, struct sprd_uart_port, port);
    struct tty_port *tty = &port.state.port;
    port.icount.rx += sp.rx_dma.trans_len;
    tty_insert_flip_string(tty, sp.rx_buf_tail, sp.rx_dma.trans_len);
    tty_flip_buffer_push(tty);
    }
#[no_mangle]
unsafe extern "C" fn sprd_uart_dma_irq(port: *mut uart_port) {
    static void sprd_uart_dma_irq(struct uart_port *port)
    {
    struct sprd_uart_port *sp =
    container_of(port, struct sprd_uart_port, port);
    struct dma_tx_state state;
    enum dma_status status;
    status = dmaengine_tx_status(sp.rx_dma.chn,
    sp.rx_dma.cookie, &state);
    if (status == DMA_ERROR)
    sprd_stop_rx(port);
    if (!state.residue && sp.pos == sp.rx_dma.phys_addr)
    return;
    if (!state.residue) {
    sp.rx_dma.trans_len = SPRD_UART_RX_SIZE +
    sp.rx_dma.phys_addr - sp.pos;
    sp.pos = sp.rx_dma.phys_addr;
    } else {
    sp.rx_dma.trans_len = state.residue - sp.pos;
    sp.pos = state.residue;
    }
    sprd_uart_dma_rx(port);
    sp.rx_buf_tail += sp.rx_dma.trans_len;
    }
#[no_mangle]
unsafe extern "C" fn sprd_complete_rx_dma(data: *mut c_void) {
    static void sprd_complete_rx_dma(void *data)
    {
    struct uart_port *port = (struct uart_port *)data;
    struct sprd_uart_port *sp =
    container_of(port, struct sprd_uart_port, port);
    struct dma_tx_state state;
    enum dma_status status;
    unsigned long flags;
    uart_port_lock_irqsave(port, &flags);
    status = dmaengine_tx_status(sp.rx_dma.chn,
    sp.rx_dma.cookie, &state);
    if (status != DMA_COMPLETE) {
    sprd_stop_rx(port);
    uart_port_unlock_irqrestore(port, flags);
    return;
    }
    if (sp.pos != sp.rx_dma.phys_addr) {
    sp.rx_dma.trans_len =  SPRD_UART_RX_SIZE +
    sp.rx_dma.phys_addr - sp.pos;
    sprd_uart_dma_rx(port);
    sp.rx_buf_tail += sp.rx_dma.trans_len;
    }
    if (sprd_start_dma_rx(port))
    sprd_stop_rx(port);
    uart_port_unlock_irqrestore(port, flags);
    }
#[no_mangle]
unsafe extern "C" fn sprd_start_dma_rx(port: *mut uart_port) -> c_int {
    static int sprd_start_dma_rx(struct uart_port *port)
    {
    struct sprd_uart_port *sp =
    container_of(port, struct sprd_uart_port, port);
    int ret;
    if (!sp.rx_dma.enable)
    return 0;
    sp.pos = sp.rx_dma.phys_addr;
    sp.rx_buf_tail = sp.rx_dma.virt;
    sprd_rx_full_thld(port, SPRD_RX_FIFO_FULL);
    ret = sprd_rx_dma_config(port, SPRD_RX_DMA_STEP);
    if (ret)
    return ret;
    return sprd_uart_dma_submit(port, &sp.rx_dma, SPRD_UART_RX_SIZE,
    DMA_DEV_TO_MEM, sprd_complete_rx_dma);
    }
#[no_mangle]
unsafe extern "C" fn sprd_release_dma(port: *mut uart_port) {
    static void sprd_release_dma(struct uart_port *port)
    {
    struct sprd_uart_port *sp =
    container_of(port, struct sprd_uart_port, port);
    sprd_uart_dma_enable(port, false);
    if (sp.rx_dma.enable)
    dma_release_channel(sp.rx_dma.chn);
    if (sp.tx_dma.enable)
    dma_release_channel(sp.tx_dma.chn);
    sp.tx_dma.enable = false;
    sp.rx_dma.enable = false;
    }
#[no_mangle]
unsafe extern "C" fn sprd_request_dma(port: *mut uart_port) {
    static void sprd_request_dma(struct uart_port *port)
    {
    struct sprd_uart_port *sp =
    container_of(port, struct sprd_uart_port, port);
    sp.tx_dma.enable = true;
    sp.rx_dma.enable = true;
    sp.tx_dma.chn = dma_request_chan(port.dev, "tx");
    if (IS_ERR(sp.tx_dma.chn)) {
    dev_err(port.dev, "request TX DMA channel failed, ret = %ld\n",
    PTR_ERR(sp.tx_dma.chn));
    sp.tx_dma.enable = false;
    }
    sp.rx_dma.chn = dma_request_chan(port.dev, "rx");
    if (IS_ERR(sp.rx_dma.chn)) {
    dev_err(port.dev, "request RX DMA channel failed, ret = %ld\n",
    PTR_ERR(sp.rx_dma.chn));
    sp.rx_dma.enable = false;
    }
    }
#[no_mangle]
unsafe extern "C" fn sprd_stop_tx(port: *mut uart_port) {
    static void sprd_stop_tx(struct uart_port *port)
    {
    struct sprd_uart_port *sp = container_of(port, struct sprd_uart_port,
    port);
    unsigned int ien, iclr;
    if (sp.tx_dma.enable) {
    sprd_stop_tx_dma(port);
    return;
    }
    iclr = serial_in(port, SPRD_ICLR);
    ien = serial_in(port, SPRD_IEN);
    iclr |= SPRD_IEN_TX_EMPTY;
    ien &= ~SPRD_IEN_TX_EMPTY;
    serial_out(port, SPRD_IEN, ien);
    serial_out(port, SPRD_ICLR, iclr);
    }
#[no_mangle]
unsafe extern "C" fn sprd_start_tx(port: *mut uart_port) {
    static void sprd_start_tx(struct uart_port *port)
    {
    struct sprd_uart_port *sp = container_of(port, struct sprd_uart_port,
    port);
    unsigned int ien;
    if (sp.tx_dma.enable) {
    sprd_start_tx_dma(port);
    return;
    }
    ien = serial_in(port, SPRD_IEN);
    if (!(ien & SPRD_IEN_TX_EMPTY)) {
    ien |= SPRD_IEN_TX_EMPTY;
    serial_out(port, SPRD_IEN, ien);
    }
    }
// The Sprd serial does not support this function.
#[no_mangle]
unsafe extern "C" fn sprd_break_ctl(port: *mut uart_port, break_state: c_int) {
    static void sprd_break_ctl(struct uart_port *port, int break_state)
    {
// nothing to do
    }
    static int handle_lsr_errors(struct uart_port *port,
    u8 *flag,
    unsigned int *lsr)
    {
    let mut ret: c_int = 0;
// statistics
    if (*lsr & SPRD_LSR_BI) {
// lsr &= ~(SPRD_LSR_FE | SPRD_LSR_PE);
    port.icount.brk++;
    ret = uart_handle_break(port);
    if (ret)
    return ret;
    } else if (*lsr & SPRD_LSR_PE)
    port.icount.parity++;
#[no_mangle]
pub unsafe extern "C" fn if(SPRD_LSR_FE: *mut *mut lsr &) -> else {
    else if (*lsr & SPRD_LSR_FE)
    port.icount.frame++;
    if (*lsr & SPRD_LSR_OE)
    port.icount.overrun++;
// mask off conditions which should be ignored
// lsr &= port->read_status_mask;
    if (*lsr & SPRD_LSR_BI)
// flag = TTY_BREAK;
#[no_mangle]
pub unsafe extern "C" fn if(SPRD_LSR_PE: *mut *mut lsr &) -> else {
    else if (*lsr & SPRD_LSR_PE)
// flag = TTY_PARITY;
#[no_mangle]
pub unsafe extern "C" fn if(SPRD_LSR_FE: *mut *mut lsr &) -> else {
    else if (*lsr & SPRD_LSR_FE)
// flag = TTY_FRAME;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sprd_rx(port: *mut uart_port) {
    static inline void sprd_rx(struct uart_port *port)
    {
    struct sprd_uart_port *sp = container_of(port, struct sprd_uart_port,
    port);
    struct tty_port *tty = &port.state.port;
    unsigned int lsr, max_count = SPRD_TIMEOUT;
    u8 ch, flag;
    if (sp.rx_dma.enable) {
    sprd_uart_dma_irq(port);
    return;
    }
    while ((serial_in(port, SPRD_STS1) & SPRD_RX_FIFO_CNT_MASK) &&
    max_count--) {
    lsr = serial_in(port, SPRD_LSR);
    ch = serial_in(port, SPRD_RXD);
    flag = TTY_NORMAL;
    port.icount.rx++;
    if (lsr & (SPRD_LSR_BI | SPRD_LSR_PE |
    SPRD_LSR_FE | SPRD_LSR_OE))
    if (handle_lsr_errors(port, &flag, &lsr))
    continue;
    if (uart_handle_sysrq_char(port, ch))
    continue;
    uart_insert_char(port, lsr, SPRD_LSR_OE, ch, flag);
    }
    tty_flip_buffer_push(tty);
    }
#[no_mangle]
pub unsafe extern "C" fn sprd_tx(port: *mut uart_port) {
    static inline void sprd_tx(struct uart_port *port)
    {
    u8 ch;
    uart_port_tx_limited(port, ch, THLD_TX_EMPTY,
    true,
    serial_out(port, SPRD_TXD, ch),
    ({}));
    }
// this handles the interrupt from one port
#[no_mangle]
unsafe extern "C" fn sprd_handle_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sprd_handle_irq(int irq, void *dev_id)
    {
    struct uart_port *port = dev_id;
    unsigned int ims;
    struct sprd_uart_port *sp =
    container_of(port, struct sprd_uart_port, port);
    uart_port_lock(port);
    ims = serial_in(port, SPRD_IMSR);
    if (!ims) {
    uart_port_unlock(port);
    return IRQ_NONE;
    }
    if (ims & sp.pdata.timeout_imsr)
    serial_out(port, SPRD_ICLR, sp.pdata.timeout_iclr);
    if (ims & SPRD_IMSR_BREAK_DETECT)
    serial_out(port, SPRD_ICLR, SPRD_IMSR_BREAK_DETECT);
    if (ims & (SPRD_IMSR_RX_FIFO_FULL | SPRD_IMSR_BREAK_DETECT |
    sp.pdata.timeout_imsr))
    sprd_rx(port);
    if (ims & SPRD_IMSR_TX_FIFO_EMPTY)
    sprd_tx(port);
    uart_port_unlock(port);
    return IRQ_HANDLED;
    }
    static void sprd_uart_dma_startup(struct uart_port *port,
    struct sprd_uart_port *sp)
    {
    int ret;
    sprd_request_dma(port);
    if (!(sp.rx_dma.enable || sp.tx_dma.enable))
    return;
    ret = sprd_start_dma_rx(port);
    if (ret) {
    sp.rx_dma.enable = false;
    dma_release_channel(sp.rx_dma.chn);
    dev_warn(port.dev, "fail to start RX dma mode\n");
    }
    sprd_uart_dma_enable(port, true);
    }
#[no_mangle]
unsafe extern "C" fn sprd_startup(port: *mut uart_port) -> c_int {
    static int sprd_startup(struct uart_port *port)
    {
    let mut ret: c_int = 0;
    unsigned int ien, fc;
    unsigned int timeout;
    struct sprd_uart_port *sp;
    unsigned long flags;
    serial_out(port, SPRD_CTL2,
    THLD_TX_EMPTY << THLD_TX_EMPTY_SHIFT | THLD_RX_FULL);
// clear rx fifo
    timeout = SPRD_TIMEOUT;
    while (timeout-- && serial_in(port, SPRD_STS1) & SPRD_RX_FIFO_CNT_MASK)
    serial_in(port, SPRD_RXD);
// clear tx fifo
    timeout = SPRD_TIMEOUT;
    while (timeout-- && serial_in(port, SPRD_STS1) & SPRD_TX_FIFO_CNT_MASK)
    cpu_relax();
// clear interrupt
    serial_out(port, SPRD_IEN, 0);
    serial_out(port, SPRD_ICLR, ~0);
// allocate irq
    sp = container_of(port, struct sprd_uart_port, port);
    snprintf(sp.name, sizeof(sp.name), "sprd_serial%d", port.line);
    sprd_uart_dma_startup(port, sp);
    ret = devm_request_irq(port.dev, port.irq, sprd_handle_irq,
    IRQF_SHARED, sp.name, port);
    if (ret)
    return ret;
    fc = serial_in(port, SPRD_CTL1);
    fc |= RX_TOUT_THLD_DEF | RX_HFC_THLD_DEF;
    serial_out(port, SPRD_CTL1, fc);
// enable interrupt
    uart_port_lock_irqsave(port, &flags);
    ien = serial_in(port, SPRD_IEN);
    ien |= SPRD_IEN_BREAK_DETECT | sp.pdata.timeout_ien;
    if (!sp.rx_dma.enable)
    ien |= SPRD_IEN_RX_FULL;
    serial_out(port, SPRD_IEN, ien);
    uart_port_unlock_irqrestore(port, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_shutdown(port: *mut uart_port) {
    static void sprd_shutdown(struct uart_port *port)
    {
    sprd_release_dma(port);
    serial_out(port, SPRD_IEN, 0);
    serial_out(port, SPRD_ICLR, ~0);
    devm_free_irq(port.dev, port.irq, port);
    }
    static void sprd_set_termios(struct uart_port *port, struct ktermios *termios,
    const struct ktermios *old)
    {
    unsigned int baud, quot;
    let mut lcr: c_uint = 0, fc;
    unsigned long flags;
// ask the core to calculate the divisor for us
    baud = uart_get_baud_rate(port, termios, old, 0, SPRD_BAUD_IO_LIMIT);
    quot = port.uartclk / baud;
// set data length
    switch (termios.c_cflag & CSIZE) {
    case CS5:
    lcr |= SPRD_LCR_DATA_LEN5;
    break;
    case CS6:
    lcr |= SPRD_LCR_DATA_LEN6;
    break;
    case CS7:
    lcr |= SPRD_LCR_DATA_LEN7;
    break;
    case CS8:
    default:
    lcr |= SPRD_LCR_DATA_LEN8;
    break;
    }
// calculate stop bits
    lcr &= ~(SPRD_LCR_STOP_1BIT | SPRD_LCR_STOP_2BIT);
    if (termios.c_cflag & CSTOPB)
    lcr |= SPRD_LCR_STOP_2BIT;
    else
    lcr |= SPRD_LCR_STOP_1BIT;
// calculate parity
    lcr &= ~SPRD_LCR_PARITY;
    termios.c_cflag &= ~CMSPAR;	/* no support mark/space */
    if (termios.c_cflag & PARENB) {
    lcr |= SPRD_LCR_PARITY_EN;
    if (termios.c_cflag & PARODD)
    lcr |= SPRD_LCR_ODD_PAR;
    else
    lcr |= SPRD_LCR_EVEN_PAR;
    }
    uart_port_lock_irqsave(port, &flags);
// update the per-port timeout
    uart_update_timeout(port, termios.c_cflag, baud);
    port.read_status_mask = SPRD_LSR_OE;
    if (termios.c_iflag & INPCK)
    port.read_status_mask |= SPRD_LSR_FE | SPRD_LSR_PE;
    if (termios.c_iflag & (IGNBRK | BRKINT | PARMRK))
    port.read_status_mask |= SPRD_LSR_BI;
// characters to ignore
    port.ignore_status_mask = 0;
    if (termios.c_iflag & IGNPAR)
    port.ignore_status_mask |= SPRD_LSR_PE | SPRD_LSR_FE;
    if (termios.c_iflag & IGNBRK) {
    port.ignore_status_mask |= SPRD_LSR_BI;
//
// If we're ignoring parity and break indicators,
// ignore overruns too (for real raw support).
//
    if (termios.c_iflag & IGNPAR)
    port.ignore_status_mask |= SPRD_LSR_OE;
    }
// flow control
    fc = serial_in(port, SPRD_CTL1);
    fc &= ~(RX_HW_FLOW_CTL_THLD | RX_HW_FLOW_CTL_EN | TX_HW_FLOW_CTL_EN);
    if (termios.c_cflag & CRTSCTS) {
    fc |= RX_HW_FLOW_CTL_THLD;
    fc |= RX_HW_FLOW_CTL_EN;
    fc |= TX_HW_FLOW_CTL_EN;
    }
// clock divider bit0~bit15
    serial_out(port, SPRD_CLKD0, quot & SPRD_CLKD0_MASK);
// clock divider bit16~bit20
    serial_out(port, SPRD_CLKD1,
    (quot & SPRD_CLKD1_MASK) >> SPRD_CLKD1_SHIFT);
    serial_out(port, SPRD_LCR, lcr);
    fc |= RX_TOUT_THLD_DEF | RX_HFC_THLD_DEF;
    serial_out(port, SPRD_CTL1, fc);
    uart_port_unlock_irqrestore(port, flags);
// Don't rewrite B0
    if (tty_termios_baud_rate(termios))
    tty_termios_encode_baud_rate(termios, baud, baud);
    }
    static const char *sprd_type(struct uart_port *port)
    {
    return "SPX";
    }
#[no_mangle]
unsafe extern "C" fn sprd_release_port(port: *mut uart_port) {
    static void sprd_release_port(struct uart_port *port)
    {
// nothing to do
    }
#[no_mangle]
unsafe extern "C" fn sprd_request_port(port: *mut uart_port) -> c_int {
    static int sprd_request_port(struct uart_port *port)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_config_port(port: *mut uart_port, flags: c_int) {
    static void sprd_config_port(struct uart_port *port, int flags)
    {
    if (flags & UART_CONFIG_TYPE)
    port.type = PORT_SPRD;
    }
#[no_mangle]
unsafe extern "C" fn sprd_verify_port(port: *mut uart_port, ser: *mut serial_struct) -> c_int {
    static int sprd_verify_port(struct uart_port *port, struct serial_struct *ser)
    {
    if (ser.type != PORT_SPRD)
    return -EINVAL;
    if (port.irq != ser.irq)
    return -EINVAL;
    if (port.iotype != ser.io_type)
    return -EINVAL;
    return 0;
    }
    static void sprd_pm(struct uart_port *port, unsigned int state,
    unsigned int oldstate)
    {
    struct sprd_uart_port *sup =
    container_of(port, struct sprd_uart_port, port);
    switch (state) {
    case UART_PM_STATE_ON:
    clk_prepare_enable(sup.clk);
    break;
    case UART_PM_STATE_OFF:
    clk_disable_unprepare(sup.clk);
    break;
    }
    }

#[no_mangle]
unsafe extern "C" fn sprd_poll_init(port: *mut uart_port) -> c_int {
    static int sprd_poll_init(struct uart_port *port)
    {
    if (port.state.pm_state != UART_PM_STATE_ON) {
    sprd_pm(port, UART_PM_STATE_ON, 0);
    port.state.pm_state = UART_PM_STATE_ON;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_poll_get_char(port: *mut uart_port) -> c_int {
    static int sprd_poll_get_char(struct uart_port *port)
    {
    while (!(serial_in(port, SPRD_STS1) & SPRD_RX_FIFO_CNT_MASK))
    cpu_relax();
    return serial_in(port, SPRD_RXD);
    }
#[no_mangle]
unsafe extern "C" fn sprd_poll_put_char(port: *mut uart_port, ch: c_uchar) {
    static void sprd_poll_put_char(struct uart_port *port, unsigned char ch)
    {
    while (serial_in(port, SPRD_STS1) & SPRD_TX_FIFO_CNT_MASK)
    cpu_relax();
    serial_out(port, SPRD_TXD, ch);
    }

    static const struct uart_ops serial_sprd_ops = {
    .tx_empty = sprd_tx_empty,
    .get_mctrl = sprd_get_mctrl,
    .set_mctrl = sprd_set_mctrl,
    .stop_tx = sprd_stop_tx,
    .start_tx = sprd_start_tx,
    .stop_rx = sprd_stop_rx,
    .break_ctl = sprd_break_ctl,
    .startup = sprd_startup,
    .shutdown = sprd_shutdown,
    .set_termios = sprd_set_termios,
    .type = sprd_type,
    .release_port = sprd_release_port,
    .request_port = sprd_request_port,
    .config_port = sprd_config_port,
    .verify_port = sprd_verify_port,
    .pm = sprd_pm,

    .poll_init	= sprd_poll_init,
    .poll_get_char	= sprd_poll_get_char,
    .poll_put_char	= sprd_poll_put_char,

    };

#[no_mangle]
unsafe extern "C" fn wait_for_xmitr(port: *mut uart_port) {
    static void wait_for_xmitr(struct uart_port *port)
    {
    unsigned int status, tmout = 10000;
// wait up to 10ms for the character(s) to be sent
    do {
    status = serial_in(port, SPRD_STS1);
    if (--tmout == 0)
    break;
    udelay(1);
    } while (status & SPRD_TX_FIFO_CNT_MASK);
    }
#[no_mangle]
unsafe extern "C" fn sprd_console_putchar(port: *mut uart_port, ch: c_uchar) {
    static void sprd_console_putchar(struct uart_port *port, unsigned char ch)
    {
    wait_for_xmitr(port);
    serial_out(port, SPRD_TXD, ch);
    }
    static void sprd_console_write(struct console *co, const char *s,
    unsigned int count)
    {
    struct uart_port *port = &sprd_port[co.index].port;
    let mut locked: c_int = 1;
    unsigned long flags;
    if (port.sysrq)
    locked = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: oops_in_progress) -> else {
    else if (oops_in_progress)
    locked = uart_port_trylock_irqsave(port, &flags);
    else
    uart_port_lock_irqsave(port, &flags);
    uart_console_write(port, s, count, sprd_console_putchar);
// wait for transmitter to become empty
    wait_for_xmitr(port);
    if (locked)
    uart_port_unlock_irqrestore(port, flags);
    }
#[no_mangle]
unsafe extern "C" fn sprd_console_setup(co: *mut console, options: *mut c_char) -> c_int {
    static int sprd_console_setup(struct console *co, char *options)
    {
    struct sprd_uart_port *sprd_uart_port;
    let mut baud: c_int = 115200;
    let mut bits: c_int = 8;
    let mut parity: c_int = 'n';
    let mut flow: c_int = 'n';
    if (co.index >= UART_NR_MAX || co.index < 0)
    co.index = 0;
    sprd_uart_port = sprd_port[co.index];
    if (!sprd_uart_port || !sprd_uart_port.port.membase) {
    pr_info("serial port %d not yet initialized\n", co.index);
    return -ENODEV;
    }
    if (options)
    uart_parse_options(options, &baud, &parity, &bits, &flow);
    return uart_set_options(&sprd_uart_port.port, co, baud,
    parity, bits, flow);
    }
    static struct uart_driver sprd_uart_driver;
    static struct console sprd_console = {
    .name = SPRD_TTY_NAME,
    .write = sprd_console_write,
    .device = uart_console_device,
    .setup = sprd_console_setup,
    .flags = CON_PRINTBUFFER,
    .index = -1,
    .data = &sprd_uart_driver,
    };
#[no_mangle]
unsafe extern "C" fn sprd_serial_console_init() -> int __init {
    static int __init sprd_serial_console_init(void)
    {
    register_console(&sprd_console);
    return 0;
    }
    console_initcall(sprd_serial_console_init);

// Support for earlycon
#[no_mangle]
unsafe extern "C" fn sprd_putc(port: *mut uart_port, c: c_uchar) {
    static void sprd_putc(struct uart_port *port, unsigned char c)
    {
    let mut timeout: c_uint = SPRD_TIMEOUT;
    while (timeout-- &&
    !(readl(port.membase + SPRD_LSR) & SPRD_LSR_TX_OVER))
    cpu_relax();
    writeb(c, port.membase + SPRD_TXD);
    }
#[no_mangle]
unsafe extern "C" fn sprd_early_write(con: *mut console, s: *const c_char, n: c_uint) {
    static void sprd_early_write(struct console *con, const char *s, unsigned int n)
    {
    struct earlycon_device *dev = con.data;
    uart_console_write(&dev.port, s, n, sprd_putc);
    }
    static int __init sprd_early_console_setup(struct earlycon_device *device,
    const char *opt)
    {
    if (!device.port.membase)
    return -ENODEV;
    device.con.write = sprd_early_write;
    return 0;
    }
    OF_EARLYCON_DECLARE(sprd_serial, "sprd,sc9836-uart",
    sprd_early_console_setup);

    static struct uart_driver sprd_uart_driver = {
    .owner = THIS_MODULE,
    .driver_name = "sprd_serial",
    .dev_name = SPRD_TTY_NAME,
    .major = 0,
    .minor = 0,
    .nr = UART_NR_MAX,
    .cons = SPRD_CONSOLE,
    };
#[no_mangle]
unsafe extern "C" fn sprd_remove(dev: *mut platform_device) {
    static void sprd_remove(struct platform_device *dev)
    {
    struct sprd_uart_port *sup = platform_get_drvdata(dev);
    if (sup) {
    uart_remove_one_port(&sprd_uart_driver, &sup.port);
    sprd_port[sup.port.line] = core::ptr::null_mut();
    sprd_rx_free_buf(sup);
    sprd_ports_num--;
    }
    if (!sprd_ports_num)
    uart_unregister_driver(&sprd_uart_driver);
    }
#[no_mangle]
unsafe extern "C" fn sprd_uart_is_console(uport: *mut uart_port) -> bool {
    static bool sprd_uart_is_console(struct uart_port *uport)
    {
    struct console *cons = sprd_uart_driver.cons;
    if ((cons && cons.index >= 0 && cons.index == uport.line) ||
    of_console_check(uport.dev.of_node, SPRD_TTY_NAME, uport.line))
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn sprd_clk_init(uport: *mut uart_port) -> c_int {
    static int sprd_clk_init(struct uart_port *uport)
    {
    struct clk *clk_uart, *clk_parent;
    struct sprd_uart_port *u = container_of(uport, struct sprd_uart_port, port);
    clk_uart = devm_clk_get(uport.dev, "uart");
    if (IS_ERR(clk_uart)) {
    if (PTR_ERR(clk_uart) == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    dev_warn(uport.dev, "uart%d can't get uart clock\n",
    uport.line);
    clk_uart = core::ptr::null_mut();
    }
    clk_parent = devm_clk_get(uport.dev, "source");
    if (IS_ERR(clk_parent)) {
    if (PTR_ERR(clk_parent) == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    dev_warn(uport.dev, "uart%d can't get source clock\n",
    uport.line);
    clk_parent = core::ptr::null_mut();
    }
    if (!clk_uart || clk_set_parent(clk_uart, clk_parent))
    uport.uartclk = SPRD_DEFAULT_SOURCE_CLK;
    else
    uport.uartclk = clk_get_rate(clk_uart);
    u.clk = devm_clk_get(uport.dev, "enable");
    if (IS_ERR(u.clk)) {
    if (PTR_ERR(u.clk) == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    dev_warn(uport.dev, "uart%d can't get enable clock\n",
    uport.line);
// To keep console alive even if the error occurred
    if (!sprd_uart_is_console(uport))
    return PTR_ERR(u.clk);
    u.clk = core::ptr::null_mut();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_probe(pdev: *mut platform_device) -> c_int {
    static int sprd_probe(struct platform_device *pdev)
    {
    struct resource *res;
    struct uart_port *up;
    struct sprd_uart_port *sport;
    int irq;
    int index;
    int ret;
    index = of_alias_get_id(pdev.dev.of_node, "serial");
    if (index < 0 || index >= UART_NR_MAX) {
    dev_err(&pdev.dev, "got a wrong serial alias id %d\n", index);
    return -EINVAL;
    }
    sport = devm_kzalloc(&pdev.dev, sizeof(*sport), GFP_KERNEL);
    if (!sport)
    return -ENOMEM;
    up = &sport.port;
    up.dev = &pdev.dev;
    up.line = index;
    up.type = PORT_SPRD;
    up.iotype = UPIO_MEM;
    up.uartclk = SPRD_DEF_RATE;
    up.fifosize = SPRD_FIFO_SIZE;
    up.ops = &serial_sprd_ops;
    up.flags = UPF_BOOT_AUTOCONF;
    up.has_sysrq = IS_ENABLED(CONFIG_SERIAL_SPRD_CONSOLE);
    ret = sprd_clk_init(up);
    if (ret)
    return ret;
    up.membase = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(up.membase))
    return PTR_ERR(up.membase);
    up.mapbase = res.start;
    sport.pdata = of_device_get_match_data(&pdev.dev);
    if (!sport.pdata) {
    dev_err(&pdev.dev, "get match data failed!\n");
    return -EINVAL;
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    up.irq = irq;
//
// Allocate one dma buffer to prepare for receive transfer, in case
// memory allocation failure at runtime.
//
    ret = sprd_rx_alloc_buf(sport);
    if (ret)
    return ret;
    if (!sprd_ports_num) {
    ret = uart_register_driver(&sprd_uart_driver);
    if (ret < 0) {
    pr_err("Failed to register SPRD-UART driver\n");
    goto free_rx_buf;
    }
    }
    sprd_ports_num++;
    sprd_port[index] = sport;
    ret = uart_add_one_port(&sprd_uart_driver, up);
    if (ret)
    goto clean_port;
    platform_set_drvdata(pdev, up);
    return 0;
    clean_port:
    sprd_port[index] = core::ptr::null_mut();
    if (--sprd_ports_num == 0)
    uart_unregister_driver(&sprd_uart_driver);
    free_rx_buf:
    sprd_rx_free_buf(sport);
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn sprd_suspend(dev: *mut device) -> c_int {
    static int sprd_suspend(struct device *dev)
    {
    struct sprd_uart_port *sup = dev_get_drvdata(dev);
    uart_suspend_port(&sprd_uart_driver, &sup.port);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_resume(dev: *mut device) -> c_int {
    static int sprd_resume(struct device *dev)
    {
    struct sprd_uart_port *sup = dev_get_drvdata(dev);
    uart_resume_port(&sprd_uart_driver, &sup.port);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(sprd_pm_ops, sprd_suspend, sprd_resume);
    static const struct of_device_id serial_ids[] = {
    {.compatible = "sprd,sc9836-uart", .data = &sc9836_data},
    {.compatible = "sprd,sc9632-uart", .data = &sc9632_data},
    {}
    };
    MODULE_DEVICE_TABLE(of, serial_ids);
    static struct platform_driver sprd_platform_driver = {
    .probe		= sprd_probe,
    .remove		= sprd_remove,
    .driver		= {
    .name	= "sprd_serial",
    .of_match_table = serial_ids,
    .pm	= &sprd_pm_ops,
    },
    };
    module_platform_driver(sprd_platform_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Spreadtrum SoC serial driver series");

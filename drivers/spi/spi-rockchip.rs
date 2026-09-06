//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-rockchip.c
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
//
// Copyright (c) 2014, Fuzhou Rockchip Electronics Co., Ltd
// Author: Addy Ke <addy.ke@rock-chips.com>
//

    writel_relaxed(readl_relaxed(reg) & ~(bits), reg)

    writel_relaxed(readl_relaxed(reg) | (bits), reg)
// SPI register offsets
pub const ROCKCHIP_SPI_CTRLR0: c_uint = 0x0000;
pub const ROCKCHIP_SPI_CTRLR1: c_uint = 0x0004;
pub const ROCKCHIP_SPI_SSIENR: c_uint = 0x0008;
pub const ROCKCHIP_SPI_SER: c_uint = 0x000c;
pub const ROCKCHIP_SPI_BAUDR: c_uint = 0x0010;
pub const ROCKCHIP_SPI_TXFTLR: c_uint = 0x0014;
pub const ROCKCHIP_SPI_RXFTLR: c_uint = 0x0018;
pub const ROCKCHIP_SPI_TXFLR: c_uint = 0x001c;
pub const ROCKCHIP_SPI_RXFLR: c_uint = 0x0020;
pub const ROCKCHIP_SPI_SR: c_uint = 0x0024;
pub const ROCKCHIP_SPI_IPR: c_uint = 0x0028;
pub const ROCKCHIP_SPI_IMR: c_uint = 0x002c;
pub const ROCKCHIP_SPI_ISR: c_uint = 0x0030;
pub const ROCKCHIP_SPI_RISR: c_uint = 0x0034;
pub const ROCKCHIP_SPI_ICR: c_uint = 0x0038;
pub const ROCKCHIP_SPI_DMACR: c_uint = 0x003c;
pub const ROCKCHIP_SPI_DMATDLR: c_uint = 0x0040;
pub const ROCKCHIP_SPI_DMARDLR: c_uint = 0x0044;
pub const ROCKCHIP_SPI_VERSION: c_uint = 0x0048;
pub const ROCKCHIP_SPI_TXDR: c_uint = 0x0400;
pub const ROCKCHIP_SPI_RXDR: c_uint = 0x0800;
// Bit fields in CTRLR0
pub const CR0_DFS_OFFSET: c_int = 0;
pub const CR0_DFS_4BIT: c_uint = 0x0;
pub const CR0_DFS_8BIT: c_uint = 0x1;
pub const CR0_DFS_16BIT: c_uint = 0x2;
pub const CR0_CFS_OFFSET: c_int = 2;
pub const CR0_SCPH_OFFSET: c_int = 6;
pub const CR0_SCPOL_OFFSET: c_int = 7;
pub const CR0_CSM_OFFSET: c_int = 8;
pub const CR0_CSM_KEEP: c_uint = 0x0;
// ss_n be high for half sclk_out cycles

// ss_n be high for one sclk_out cycle
pub const CR0_CSM_ONE: c_uint = 0x2;
// ss_n to sclk_out delay
pub const CR0_SSD_OFFSET: c_int = 10;
//
// The period between ss_n active and
// sclk_out active is half sclk_out cycles
//
pub const CR0_SSD_HALF: c_uint = 0x0;
//
// The period between ss_n active and
// sclk_out active is one sclk_out cycle
//
pub const CR0_SSD_ONE: c_uint = 0x1;
pub const CR0_EM_OFFSET: c_int = 11;
pub const CR0_EM_LITTLE: c_uint = 0x0;
pub const CR0_EM_BIG: c_uint = 0x1;
pub const CR0_FBM_OFFSET: c_int = 12;
pub const CR0_FBM_MSB: c_uint = 0x0;
pub const CR0_FBM_LSB: c_uint = 0x1;
pub const CR0_BHT_OFFSET: c_int = 13;
pub const CR0_BHT_16BIT: c_uint = 0x0;
pub const CR0_BHT_8BIT: c_uint = 0x1;
pub const CR0_RSD_OFFSET: c_int = 14;
pub const CR0_RSD_MAX: c_uint = 0x3;
pub const CR0_FRF_OFFSET: c_int = 16;
pub const CR0_FRF_SPI: c_uint = 0x0;
pub const CR0_FRF_SSP: c_uint = 0x1;
pub const CR0_FRF_MICROWIRE: c_uint = 0x2;
pub const CR0_XFM_OFFSET: c_int = 18;
pub const CR0_XFM_TR: c_uint = 0x0;
pub const CR0_XFM_TO: c_uint = 0x1;
pub const CR0_XFM_RO: c_uint = 0x2;
pub const CR0_OPM_OFFSET: c_int = 20;
pub const CR0_OPM_HOST: c_uint = 0x0;
pub const CR0_OPM_TARGET: c_uint = 0x1;
pub const CR0_SOI_OFFSET: c_int = 23;
// Bit fields in SER, 2bit
pub const SER_MASK: c_uint = 0x3;
// Bit fields in BAUDR
pub const BAUDR_SCKDV_MIN: c_int = 2;
pub const BAUDR_SCKDV_MAX: c_int = 65534;
// Bit fields in SR, 6bit
pub const SR_MASK: c_uint = 0x3f;

// Bit fields in ISR, IMR, ISR, RISR, 5bit
pub const INT_MASK: c_uint = 0x1f;

// Bit fields in ICR, 4bit
pub const ICR_MASK: c_uint = 0x0f;

// Bit fields in DMACR

// Driver state flags

// sclk_out: spi host internal logic in rk3x can support 50Mhz

//
// SPI_CTRLR1 is 16-bits, so we should support lengths of 0xffff + 1. However,
// the controller seems to hang when given 0x10000, so stick with this for now.
//
pub const ROCKCHIP_SPI_MAX_TRANLEN: c_uint = 0xffff;
pub const ROCKCHIP_SPI_MAX_NATIVE_CS_NUM: c_int = 2;
pub const ROCKCHIP_SPI_VER2_TYPE1: c_uint = 0x05EC0002;
pub const ROCKCHIP_SPI_VER2_TYPE2: c_uint = 0x00110002;
pub const ROCKCHIP_AUTOSUSPEND_TIMEOUT: c_int = 2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_spi {
    pub dev: *mut device,
    pub spiclk: *mut clk,
    pub apb_pclk: *mut clk,
    pub regs: *mut void __iomem,
    pub dma_addr_rx: dma_addr_t,
    pub dma_addr_tx: dma_addr_t,
    pub tx: *const c_void,
    pub rx: *mut c_void,
    pub tx_left: c_uint,
    pub rx_left: c_uint,
    pub state: core::sync::atomic::AtomicI32,
// depth of the FIFO buffer
    pub fifo_len: u32,
// frequency of spiclk
    pub freq: u32,
    pub n_bytes: u8,
    pub rsd: u8,
    pub target_abort: bool,
    pub /: *mut *mut bool cs_inactive; / spi target transmission stop when cs inactive,
    pub /: *mut *mut bool cs_high_supported; / native CS supports active-high polarity,
    pub /: *mut *mut *mut spi_transfer xfer; / Store xfer temporarily,
}

#[no_mangle]
pub unsafe extern "C" fn spi_enable_chip(rs: *mut rockchip_spi, enable: bool) {
    static inline void spi_enable_chip(struct rockchip_spi *rs, bool enable)
    {
    writel_relaxed((enable ? 1U : 0U), rs.regs + ROCKCHIP_SPI_SSIENR);
    }
#[no_mangle]
pub unsafe extern "C" fn wait_for_tx_idle(rs: *mut rockchip_spi, target_mode: bool) {
    static inline void wait_for_tx_idle(struct rockchip_spi *rs, bool target_mode)
    {
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(5);
    do {
    if (target_mode) {
    if (!(readl_relaxed(rs.regs + ROCKCHIP_SPI_SR) & SR_TARGET_TX_BUSY) &&
    !((readl_relaxed(rs.regs + ROCKCHIP_SPI_SR) & SR_BUSY)))
    return;
    } else {
    if (!(readl_relaxed(rs.regs + ROCKCHIP_SPI_SR) & SR_BUSY))
    return;
    }
    } while (!time_after(jiffies, timeout));
    dev_warn(rs.dev, "spi controller is in busy state!\n");
    }
#[no_mangle]
unsafe extern "C" fn get_fifo_len(rs: *mut rockchip_spi) -> u32 {
    static u32 get_fifo_len(struct rockchip_spi *rs)
    {
    u32 ver;
    ver = readl_relaxed(rs.regs + ROCKCHIP_SPI_VERSION);
    switch (ver) {
    case ROCKCHIP_SPI_VER2_TYPE1:
    case ROCKCHIP_SPI_VER2_TYPE2:
    return 64;
    default:
    return 32;
    }
    }
#[no_mangle]
unsafe extern "C" fn rockchip_spi_set_cs(spi: *mut spi_device, enable: bool) {
    static void rockchip_spi_set_cs(struct spi_device *spi, bool enable)
    {
    struct spi_controller *ctlr = spi.controller;
    struct rockchip_spi *rs = spi_controller_get_devdata(ctlr);
    let mut cs_asserted: bool = spi.mode & SPI_CS_HIGH ? enable : !enable;
    bool cs_actual;
//
// SPI subsystem tries to avoid no-op calls that would break the PM
// refcount below. It can't however for the first time it is used.
// To detect this case we read it here and bail out early for no-ops.
//
    if (spi_get_csgpiod(spi, 0))
    cs_actual = !!(readl_relaxed(rs.regs + ROCKCHIP_SPI_SER) & 1);
    else
    cs_actual = !!(readl_relaxed(rs.regs + ROCKCHIP_SPI_SER) &
    BIT(spi_get_chipselect(spi, 0)));
    if (unlikely(cs_actual == cs_asserted))
    return;
    if (cs_asserted) {
// Keep things powered as long as CS is asserted
    pm_runtime_get_sync(rs.dev);
    if (spi_get_csgpiod(spi, 0))
    ROCKCHIP_SPI_SET_BITS(rs.regs + ROCKCHIP_SPI_SER, 1);
    else
    ROCKCHIP_SPI_SET_BITS(rs.regs + ROCKCHIP_SPI_SER,
    BIT(spi_get_chipselect(spi, 0)));
    } else {
    if (spi_get_csgpiod(spi, 0))
    ROCKCHIP_SPI_CLR_BITS(rs.regs + ROCKCHIP_SPI_SER, 1);
    else
    ROCKCHIP_SPI_CLR_BITS(rs.regs + ROCKCHIP_SPI_SER,
    BIT(spi_get_chipselect(spi, 0)));
// Drop reference from when we first asserted CS
    pm_runtime_put(rs.dev);
    }
    }
    static void rockchip_spi_handle_err(struct spi_controller *ctlr,
    struct spi_message *msg)
    {
    struct rockchip_spi *rs = spi_controller_get_devdata(ctlr);
// stop running spi transfer
// this also flushes both rx and tx fifos
//
    spi_enable_chip(rs, false);
// make sure all interrupts are masked and status cleared
    writel_relaxed(0, rs.regs + ROCKCHIP_SPI_IMR);
    writel_relaxed(0xffffffff, rs.regs + ROCKCHIP_SPI_ICR);
    if (atomic_read(&rs.state) & TXDMA)
    dmaengine_terminate_async(ctlr.dma_tx);
    if (atomic_read(&rs.state) & RXDMA)
    dmaengine_terminate_async(ctlr.dma_rx);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_spi_pio_writer(rs: *mut rockchip_spi) {
    static void rockchip_spi_pio_writer(struct rockchip_spi *rs)
    {
    let mut tx_free: u32 = rs.fifo_len - readl_relaxed(rs.regs + ROCKCHIP_SPI_TXFLR);
    let mut words: u32 = min(rs.tx_left, tx_free);
    rs.tx_left -= words;
    for (; words; words--) {
    u32 txw;
    if (rs.n_bytes == 1)
    txw = *(u8 *)rs.tx;
    else
    txw = *(u16 *)rs.tx;
    writel_relaxed(txw, rs.regs + ROCKCHIP_SPI_TXDR);
    rs.tx += rs.n_bytes;
    }
    }
#[no_mangle]
unsafe extern "C" fn rockchip_spi_pio_reader(rs: *mut rockchip_spi) {
    static void rockchip_spi_pio_reader(struct rockchip_spi *rs)
    {
    let mut words: u32 = readl_relaxed(rs.regs + ROCKCHIP_SPI_RXFLR);
    let mut rx_left: u32 = (rs.rx_left > words) ? rs.rx_left - words : 0;
// the hardware doesn't allow us to change fifo threshold
// level while spi is enabled, so instead make sure to leave
// enough words in the rx fifo to get the last interrupt
// exactly when all words have been received
//
    if (rx_left) {
    let mut ftl: u32 = readl_relaxed(rs.regs + ROCKCHIP_SPI_RXFTLR) + 1;
    if (rx_left < ftl) {
    rx_left = ftl;
    words = rs.rx_left - rx_left;
    }
    }
    rs.rx_left = rx_left;
    for (; words; words--) {
    let mut rxw: u32 = readl_relaxed(rs.regs + ROCKCHIP_SPI_RXDR);
    if (!rs.rx)
    continue;
    if (rs.n_bytes == 1)
// (u8 *)rs->rx = (u8)rxw;
    else
// (u16 *)rs->rx = (u16)rxw;
    rs.rx += rs.n_bytes;
    }
    }
#[no_mangle]
unsafe extern "C" fn rockchip_spi_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rockchip_spi_isr(int irq, void *dev_id)
    {
    struct spi_controller *ctlr = dev_id;
    struct rockchip_spi *rs = spi_controller_get_devdata(ctlr);
// When int_cs_inactive comes, spi target abort
    if (rs.cs_inactive &&
    (readl_relaxed(rs.regs + ROCKCHIP_SPI_ISR) & INT_CS_INACTIVE)) {
    ctlr.target_abort(ctlr);
    writel_relaxed(0, rs.regs + ROCKCHIP_SPI_IMR);
    writel_relaxed(0xffffffff, rs.regs + ROCKCHIP_SPI_ICR);
    return IRQ_HANDLED;
    }
    if (rs.tx_left)
    rockchip_spi_pio_writer(rs);
    rockchip_spi_pio_reader(rs);
    if (!rs.rx_left) {
    spi_enable_chip(rs, false);
    writel_relaxed(0, rs.regs + ROCKCHIP_SPI_IMR);
    writel_relaxed(0xffffffff, rs.regs + ROCKCHIP_SPI_ICR);
    spi_finalize_current_transfer(ctlr);
    }
    return IRQ_HANDLED;
    }
    static int rockchip_spi_prepare_irq(struct rockchip_spi *rs,
    struct spi_controller *ctlr,
    struct spi_transfer *xfer)
    {
    rs.tx = xfer.tx_buf;
    rs.rx = xfer.rx_buf;
    rs.tx_left = rs.tx ? xfer.len / rs.n_bytes : 0;
    rs.rx_left = xfer.len / rs.n_bytes;
    writel_relaxed(0xffffffff, rs.regs + ROCKCHIP_SPI_ICR);
    spi_enable_chip(rs, true);
    if (rs.tx_left)
    rockchip_spi_pio_writer(rs);
    if (rs.cs_inactive)
    writel_relaxed(INT_RF_FULL | INT_CS_INACTIVE, rs.regs + ROCKCHIP_SPI_IMR);
    else
    writel_relaxed(INT_RF_FULL, rs.regs + ROCKCHIP_SPI_IMR);
// 1 means the transfer is in progress
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_spi_dma_rxcb(data: *mut c_void) {
    static void rockchip_spi_dma_rxcb(void *data)
    {
    struct spi_controller *ctlr = data;
    struct rockchip_spi *rs = spi_controller_get_devdata(ctlr);
    let mut state: c_int = atomic_fetch_andnot(RXDMA, &rs.state);
    if (state & TXDMA && !rs.target_abort)
    return;
    if (rs.cs_inactive)
    writel_relaxed(0, rs.regs + ROCKCHIP_SPI_IMR);
    spi_enable_chip(rs, false);
    spi_finalize_current_transfer(ctlr);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_spi_dma_txcb(data: *mut c_void) {
    static void rockchip_spi_dma_txcb(void *data)
    {
    struct spi_controller *ctlr = data;
    struct rockchip_spi *rs = spi_controller_get_devdata(ctlr);
    let mut state: c_int = atomic_fetch_andnot(TXDMA, &rs.state);
    if (state & RXDMA && !rs.target_abort)
    return;
// Wait until the FIFO data completely.
    wait_for_tx_idle(rs, ctlr.target);
    spi_enable_chip(rs, false);
    spi_finalize_current_transfer(ctlr);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_spi_calc_burst_size(data_len: u32) -> u32 {
    static u32 rockchip_spi_calc_burst_size(u32 data_len)
    {
    u32 i;
// burst size: 1, 2, 4, 8
    for (i = 1; i < 8; i <<= 1) {
    if (data_len & i)
    break;
    }
    return i;
    }
    static int rockchip_spi_prepare_dma(struct rockchip_spi *rs,
    struct spi_controller *ctlr, struct spi_transfer *xfer)
    {
    struct dma_async_tx_descriptor *rxdesc, *txdesc;
    atomic_set(&rs.state, 0);
    rs.tx = xfer.tx_buf;
    rs.rx = xfer.rx_buf;
    rxdesc = core::ptr::null_mut();
    if (xfer.rx_buf) {
    struct dma_slave_config rxconf = {
    .direction = DMA_DEV_TO_MEM,
    .src_addr = rs.dma_addr_rx,
    .src_addr_width = rs.n_bytes,
    .src_maxburst = rockchip_spi_calc_burst_size(xfer.len / rs.n_bytes),
    };
    dmaengine_slave_config(ctlr.dma_rx, &rxconf);
    rxdesc = dmaengine_prep_slave_sg(
    ctlr.dma_rx,
    xfer.rx_sg.sgl, xfer.rx_sg.nents,
    DMA_DEV_TO_MEM, DMA_PREP_INTERRUPT);
    if (!rxdesc)
    return -EINVAL;
    rxdesc.callback = rockchip_spi_dma_rxcb;
    rxdesc.callback_param = ctlr;
    }
    txdesc = core::ptr::null_mut();
    if (xfer.tx_buf) {
    struct dma_slave_config txconf = {
    .direction = DMA_MEM_TO_DEV,
    .dst_addr = rs.dma_addr_tx,
    .dst_addr_width = rs.n_bytes,
    .dst_maxburst = rs.fifo_len / 4,
    };
    dmaengine_slave_config(ctlr.dma_tx, &txconf);
    txdesc = dmaengine_prep_slave_sg(
    ctlr.dma_tx,
    xfer.tx_sg.sgl, xfer.tx_sg.nents,
    DMA_MEM_TO_DEV, DMA_PREP_INTERRUPT);
    if (!txdesc) {
    if (rxdesc)
    dmaengine_terminate_sync(ctlr.dma_rx);
    return -EINVAL;
    }
    txdesc.callback = rockchip_spi_dma_txcb;
    txdesc.callback_param = ctlr;
    }
// rx must be started before tx due to spi instinct
    if (rxdesc) {
    atomic_or(RXDMA, &rs.state);
    ctlr.dma_rx.cookie = dmaengine_submit(rxdesc);
    dma_async_issue_pending(ctlr.dma_rx);
    }
    if (rs.cs_inactive)
    writel_relaxed(INT_CS_INACTIVE, rs.regs + ROCKCHIP_SPI_IMR);
    spi_enable_chip(rs, true);
    if (txdesc) {
    atomic_or(TXDMA, &rs.state);
    dmaengine_submit(txdesc);
    dma_async_issue_pending(ctlr.dma_tx);
    }
// 1 means the transfer is in progress
    return 1;
    }
    static int rockchip_spi_config(struct rockchip_spi *rs,
    struct spi_device *spi, struct spi_transfer *xfer,
    bool use_dma, bool target_mode)
    {
    u32 cr0 = CR0_FRF_SPI  << CR0_FRF_OFFSET
    | CR0_BHT_8BIT << CR0_BHT_OFFSET
    | CR0_SSD_ONE  << CR0_SSD_OFFSET
    | CR0_EM_BIG   << CR0_EM_OFFSET;
    u32 cr1;
    let mut dmacr: u32 = 0;
    if (target_mode)
    cr0 |= CR0_OPM_TARGET << CR0_OPM_OFFSET;
    rs.target_abort = false;
    cr0 |= rs.rsd << CR0_RSD_OFFSET;
    cr0 |= (spi.mode & 0x3U) << CR0_SCPH_OFFSET;
    if (spi.mode & SPI_LSB_FIRST)
    cr0 |= CR0_FBM_LSB << CR0_FBM_OFFSET;
    if ((spi.mode & SPI_CS_HIGH) && !(spi_get_csgpiod(spi, 0)))
    cr0 |= BIT(spi_get_chipselect(spi, 0)) << CR0_SOI_OFFSET;
    if (xfer.rx_buf && xfer.tx_buf)
    cr0 |= CR0_XFM_TR << CR0_XFM_OFFSET;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: xfer->rx_buf) -> else {
    else if (xfer.rx_buf)
    cr0 |= CR0_XFM_RO << CR0_XFM_OFFSET;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: use_dma) -> else {
    else if (use_dma)
    cr0 |= CR0_XFM_TO << CR0_XFM_OFFSET;
    switch (xfer.bits_per_word) {
    case 4:
    cr0 |= CR0_DFS_4BIT << CR0_DFS_OFFSET;
    cr1 = xfer.len - 1;
    break;
    case 8:
    cr0 |= CR0_DFS_8BIT << CR0_DFS_OFFSET;
    cr1 = xfer.len - 1;
    break;
    case 16:
    cr0 |= CR0_DFS_16BIT << CR0_DFS_OFFSET;
    cr1 = xfer.len / 2 - 1;
    break;
    default:
// we only whitelist 4, 8 and 16 bit words in
// ctlr->bits_per_word_mask, so this shouldn't
// happen
//
    dev_err(rs.dev, "unknown bits per word: %d\n",
    xfer.bits_per_word);
    return -EINVAL;
    }
    if (use_dma) {
    if (xfer.tx_buf)
    dmacr |= TF_DMA_EN;
    if (xfer.rx_buf)
    dmacr |= RF_DMA_EN;
    }
    writel_relaxed(cr0, rs.regs + ROCKCHIP_SPI_CTRLR0);
    writel_relaxed(cr1, rs.regs + ROCKCHIP_SPI_CTRLR1);
// unfortunately setting the fifo threshold level to generate an
// interrupt exactly when the fifo is full doesn't seem to work,
// so we need the strict inequality here
//
    if ((xfer.len / rs.n_bytes) < rs.fifo_len)
    writel_relaxed(xfer.len / rs.n_bytes - 1, rs.regs + ROCKCHIP_SPI_RXFTLR);
    else
    writel_relaxed(rs.fifo_len / 2 - 1, rs.regs + ROCKCHIP_SPI_RXFTLR);
    writel_relaxed(rs.fifo_len / 2 - 1, rs.regs + ROCKCHIP_SPI_DMATDLR);
    writel_relaxed(rockchip_spi_calc_burst_size(xfer.len / rs.n_bytes) - 1,
    rs.regs + ROCKCHIP_SPI_DMARDLR);
    writel_relaxed(dmacr, rs.regs + ROCKCHIP_SPI_DMACR);
// the hardware only supports an even clock divisor, so
// round divisor = spiclk / speed up to nearest even number
// so that the resulting speed is <= the requested speed
//
    writel_relaxed(2 * DIV_ROUND_UP(rs.freq, 2 * xfer.speed_hz),
    rs.regs + ROCKCHIP_SPI_BAUDR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_spi_max_transfer_size(spi: *mut spi_device) -> usize {
    static size_t rockchip_spi_max_transfer_size(struct spi_device *spi)
    {
    return ROCKCHIP_SPI_MAX_TRANLEN;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_spi_target_abort(ctlr: *mut spi_controller) -> c_int {
    static int rockchip_spi_target_abort(struct spi_controller *ctlr)
    {
    struct rockchip_spi *rs = spi_controller_get_devdata(ctlr);
    u32 rx_fifo_left;
    struct dma_tx_state state;
    enum dma_status status;
// Get current dma rx point
    if (atomic_read(&rs.state) & RXDMA) {
    dmaengine_pause(ctlr.dma_rx);
    status = dmaengine_tx_status(ctlr.dma_rx, ctlr.dma_rx.cookie, &state);
    if (status == DMA_ERROR) {
    rs.rx = rs.xfer.rx_buf;
    rs.xfer.len = 0;
    rx_fifo_left = readl_relaxed(rs.regs + ROCKCHIP_SPI_RXFLR);
    for (; rx_fifo_left; rx_fifo_left--)
    readl_relaxed(rs.regs + ROCKCHIP_SPI_RXDR);
    goto out;
    } else {
    rs.rx += rs.xfer.len - rs.n_bytes * state.residue;
    }
    }
// Get the valid data left in rx fifo and set rs->xfer->len real rx size
    if (rs.rx) {
    rx_fifo_left = readl_relaxed(rs.regs + ROCKCHIP_SPI_RXFLR);
    for (; rx_fifo_left; rx_fifo_left--) {
    let mut rxw: u32 = readl_relaxed(rs.regs + ROCKCHIP_SPI_RXDR);
    if (rs.n_bytes == 1)
// (u8 *)rs->rx = (u8)rxw;
    else
// (u16 *)rs->rx = (u16)rxw;
    rs.rx += rs.n_bytes;
    }
    rs.xfer.len = (unsigned int)(rs.rx - rs.xfer.rx_buf);
    }
    out:
    if (atomic_read(&rs.state) & RXDMA)
    dmaengine_terminate_sync(ctlr.dma_rx);
    if (atomic_read(&rs.state) & TXDMA)
    dmaengine_terminate_sync(ctlr.dma_tx);
    atomic_set(&rs.state, 0);
    spi_enable_chip(rs, false);
    rs.target_abort = true;
    spi_finalize_current_transfer(ctlr);
    return 0;
    }
    static int rockchip_spi_transfer_one(
    struct spi_controller *ctlr,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct rockchip_spi *rs = spi_controller_get_devdata(ctlr);
    int ret;
    bool use_dma;
// Zero length transfers won't trigger an interrupt on completion
    if (!xfer.len) {
    spi_finalize_current_transfer(ctlr);
    return 1;
    }
    WARN_ON(readl_relaxed(rs.regs + ROCKCHIP_SPI_SSIENR) &&
    (readl_relaxed(rs.regs + ROCKCHIP_SPI_SR) & SR_BUSY));
    if (!xfer.tx_buf && !xfer.rx_buf) {
    dev_err(rs.dev, "No buffer for transfer\n");
    return -EINVAL;
    }
    if (xfer.len > ROCKCHIP_SPI_MAX_TRANLEN) {
    dev_err(rs.dev, "Transfer is too long (%d)\n", xfer.len);
    return -EINVAL;
    }
    rs.n_bytes = xfer.bits_per_word <= 8 ? 1 : 2;
    rs.xfer = xfer;
    use_dma = ctlr.can_dma ? ctlr.can_dma(ctlr, spi, xfer) : false;
    ret = rockchip_spi_config(rs, spi, xfer, use_dma, ctlr.target);
    if (ret)
    return ret;
    if (use_dma)
    return rockchip_spi_prepare_dma(rs, ctlr, xfer);
    return rockchip_spi_prepare_irq(rs, ctlr, xfer);
    }
    static bool rockchip_spi_can_dma(struct spi_controller *ctlr,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct rockchip_spi *rs = spi_controller_get_devdata(ctlr);
    let mut bytes_per_word: c_uint = xfer.bits_per_word <= 8 ? 1 : 2;
// if the numbor of spi words to transfer is less than the fifo
// length we can just fill the fifo and wait for a single irq,
// so don't bother setting up dma
//
    return xfer.len / bytes_per_word >= rs.fifo_len;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_spi_setup(spi: *mut spi_device) -> c_int {
    static int rockchip_spi_setup(struct spi_device *spi)
    {
    struct rockchip_spi *rs = spi_controller_get_devdata(spi.controller);
    u32 cr0;
    if (!spi_get_csgpiod(spi, 0) && (spi.mode & SPI_CS_HIGH) && !rs.cs_high_supported) {
    dev_warn(&spi.dev, "setup: non GPIO CS can't be active-high\n");
    return -EINVAL;
    }
    pm_runtime_get_sync(rs.dev);
    cr0 = readl_relaxed(rs.regs + ROCKCHIP_SPI_CTRLR0);
    cr0 &= ~(0x3 << CR0_SCPH_OFFSET);
    cr0 |= ((spi.mode & 0x3) << CR0_SCPH_OFFSET);
    if (spi.mode & SPI_CS_HIGH && spi_get_chipselect(spi, 0) <= 1)
    cr0 |= BIT(spi_get_chipselect(spi, 0)) << CR0_SOI_OFFSET;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: spi_get_chipselect(spi, 1: 0) <=) -> else {
    else if (spi_get_chipselect(spi, 0) <= 1)
    cr0 &= ~(BIT(spi_get_chipselect(spi, 0)) << CR0_SOI_OFFSET);
    writel_relaxed(cr0, rs.regs + ROCKCHIP_SPI_CTRLR0);
    pm_runtime_put(rs.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_spi_probe(pdev: *mut platform_device) -> c_int {
    static int rockchip_spi_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct spi_controller *ctlr;
    struct rockchip_spi *rs;
    struct resource *mem;
    u32 rsd_nsecs, num_cs;
    bool target_mode;
    int ret;
    target_mode = of_property_read_bool(np, "spi-slave");
    if (target_mode)
    ctlr = devm_spi_alloc_target(&pdev.dev, sizeof(*rs));
    else
    ctlr = devm_spi_alloc_host(&pdev.dev, sizeof(*rs));
    if (!ctlr)
    return -ENOMEM;
    platform_set_drvdata(pdev, ctlr);
    rs = spi_controller_get_devdata(ctlr);
// Get basic io resource and map it
    rs.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mem);
    if (IS_ERR(rs.regs))
    return PTR_ERR(rs.regs);
    rs.apb_pclk = devm_clk_get_enabled(&pdev.dev, "apb_pclk");
    if (IS_ERR(rs.apb_pclk)) {
    return dev_err_probe(&pdev.dev, PTR_ERR(rs.apb_pclk),
    "Failed to get apb_pclk\n");
    }
    rs.spiclk = devm_clk_get_enabled(&pdev.dev, "spiclk");
    if (IS_ERR(rs.spiclk)) {
    return dev_err_probe(&pdev.dev, PTR_ERR(rs.spiclk),
    "Failed to get spi_pclk\n");
    }
    spi_enable_chip(rs, false);
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    return ret;
    ret = devm_request_irq(&pdev.dev, ret, rockchip_spi_isr, 0,
    dev_name(&pdev.dev), ctlr);
    if (ret)
    return ret;
    rs.dev = &pdev.dev;
    rs.freq = clk_get_rate(rs.spiclk);
    if (!of_property_read_u32(pdev.dev.of_node, "rx-sample-delay-ns",
    &rsd_nsecs)) {
// rx sample delay is expressed in parent clock cycles (max 3)
    let mut rsd: u32 = DIV_ROUND_CLOSEST(rsd_nsecs * (rs.freq >> 8), 1000000000 >> 8);
    if (!rsd) {
    dev_warn(rs.dev, "%u Hz are too slow to express %u ns delay\n",
    rs.freq, rsd_nsecs);
    } else if (rsd > CR0_RSD_MAX) {
    rsd = CR0_RSD_MAX;
    dev_warn(rs.dev,
    "%u Hz are too fast to express %u ns delay, clamping at %u ns\n",
    rs.freq, rsd_nsecs, CR0_RSD_MAX * 1000000000U / rs.freq);
    }
    rs.rsd = rsd;
    }
    rs.fifo_len = get_fifo_len(rs);
    if (!rs.fifo_len)
    return dev_err_probe(&pdev.dev, -EINVAL, "Failed to get fifo length\n");
    pm_runtime_set_autosuspend_delay(&pdev.dev, ROCKCHIP_AUTOSUSPEND_TIMEOUT);
    pm_runtime_use_autosuspend(&pdev.dev);
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    ctlr.auto_runtime_pm = true;
    ctlr.bus_num = pdev.id;
    ctlr.mode_bits = SPI_CPOL | SPI_CPHA | SPI_LOOP | SPI_LSB_FIRST;
    if (target_mode) {
    ctlr.mode_bits |= SPI_NO_CS;
    ctlr.target_abort = rockchip_spi_target_abort;
    } else {
    ctlr.flags = SPI_CONTROLLER_GPIO_SS;
    ctlr.max_native_cs = ROCKCHIP_SPI_MAX_NATIVE_CS_NUM;
//
// rk spi0 has two native cs, spi1..5 one cs only
// if num-cs is missing in the dts, default to 1
//
    if (of_property_read_u32(np, "num-cs", &num_cs))
    num_cs = 1;
    ctlr.num_chipselect = num_cs;
    ctlr.use_gpio_descriptors = true;
    }
    ctlr.bits_per_word_mask = SPI_BPW_MASK(16) | SPI_BPW_MASK(8) | SPI_BPW_MASK(4);
    ctlr.min_speed_hz = rs.freq / BAUDR_SCKDV_MAX;
    ctlr.max_speed_hz = min(rs.freq / BAUDR_SCKDV_MIN, MAX_SCLK_OUT);
    ctlr.setup = rockchip_spi_setup;
    ctlr.set_cs = rockchip_spi_set_cs;
    ctlr.transfer_one = rockchip_spi_transfer_one;
    ctlr.max_transfer_size = rockchip_spi_max_transfer_size;
    ctlr.handle_err = rockchip_spi_handle_err;
    ctlr.dma_tx = dma_request_chan(rs.dev, "tx");
    if (IS_ERR(ctlr.dma_tx)) {
// Check tx to see if we need to defer driver probing
    ret = dev_warn_probe(rs.dev, PTR_ERR(ctlr.dma_tx),
    "Failed to request optional TX DMA channel\n");
    if (ret == -EPROBE_DEFER)
    goto err_disable_pm_runtime;
    ctlr.dma_tx = core::ptr::null_mut();
    }
    ctlr.dma_rx = dma_request_chan(rs.dev, "rx");
    if (IS_ERR(ctlr.dma_rx)) {
// Check rx to see if we need to defer driver probing
    ret = dev_warn_probe(rs.dev, PTR_ERR(ctlr.dma_rx),
    "Failed to request optional RX DMA channel\n");
    if (ret == -EPROBE_DEFER)
    goto err_free_dma_tx;
    ctlr.dma_rx = core::ptr::null_mut();
    }
    if (ctlr.dma_tx && ctlr.dma_rx) {
    rs.dma_addr_tx = mem.start + ROCKCHIP_SPI_TXDR;
    rs.dma_addr_rx = mem.start + ROCKCHIP_SPI_RXDR;
    ctlr.can_dma = rockchip_spi_can_dma;
    }
    switch (readl_relaxed(rs.regs + ROCKCHIP_SPI_VERSION)) {
    case ROCKCHIP_SPI_VER2_TYPE2:
    rs.cs_high_supported = true;
    ctlr.mode_bits |= SPI_CS_HIGH;
    if (ctlr.can_dma && target_mode)
    rs.cs_inactive = true;
    else
    rs.cs_inactive = false;
    break;
    default:
    rs.cs_inactive = false;
    break;
    }
    ret = spi_register_controller(ctlr);
    if (ret < 0) {
    dev_err(&pdev.dev, "Failed to register controller\n");
    goto err_free_dma_rx;
    }
    return 0;
    err_free_dma_rx:
    if (ctlr.dma_rx)
    dma_release_channel(ctlr.dma_rx);
    err_free_dma_tx:
    if (ctlr.dma_tx)
    dma_release_channel(ctlr.dma_tx);
    err_disable_pm_runtime:
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_spi_remove(pdev: *mut platform_device) {
    static void rockchip_spi_remove(struct platform_device *pdev)
    {
    struct spi_controller *ctlr = platform_get_drvdata(pdev);
    pm_runtime_get_sync(&pdev.dev);
    spi_unregister_controller(ctlr);
    pm_runtime_put_noidle(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    pm_runtime_set_suspended(&pdev.dev);
    if (ctlr.dma_tx)
    dma_release_channel(ctlr.dma_tx);
    if (ctlr.dma_rx)
    dma_release_channel(ctlr.dma_rx);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_spi_suspend(dev: *mut device) -> c_int {
    static int rockchip_spi_suspend(struct device *dev)
    {
    int ret;
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    ret = spi_controller_suspend(ctlr);
    if (ret < 0)
    return ret;
    ret = pm_runtime_force_suspend(dev);
    if (ret < 0) {
    spi_controller_resume(ctlr);
    return ret;
    }
    pinctrl_pm_select_sleep_state(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_spi_resume(dev: *mut device) -> c_int {
    static int rockchip_spi_resume(struct device *dev)
    {
    int ret;
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    pinctrl_pm_select_default_state(dev);
    ret = pm_runtime_force_resume(dev);
    if (ret < 0)
    return ret;
    return spi_controller_resume(ctlr);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_spi_runtime_suspend(dev: *mut device) -> c_int {
    static int rockchip_spi_runtime_suspend(struct device *dev)
    {
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    struct rockchip_spi *rs = spi_controller_get_devdata(ctlr);
    clk_disable_unprepare(rs.spiclk);
    clk_disable_unprepare(rs.apb_pclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_spi_runtime_resume(dev: *mut device) -> c_int {
    static int rockchip_spi_runtime_resume(struct device *dev)
    {
    int ret;
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    struct rockchip_spi *rs = spi_controller_get_devdata(ctlr);
    ret = clk_prepare_enable(rs.apb_pclk);
    if (ret < 0)
    return ret;
    ret = clk_prepare_enable(rs.spiclk);
    if (ret < 0)
    clk_disable_unprepare(rs.apb_pclk);
    return 0;
    }
    static const struct dev_pm_ops rockchip_spi_pm = {
    NOIRQ_SYSTEM_SLEEP_PM_OPS(rockchip_spi_suspend, rockchip_spi_resume)
    RUNTIME_PM_OPS(rockchip_spi_runtime_suspend,
    rockchip_spi_runtime_resume, core::ptr::null_mut())
    };
    static const struct of_device_id rockchip_spi_dt_match[] = {
    { .compatible = "rockchip,px30-spi", },
    { .compatible = "rockchip,rk3036-spi", },
    { .compatible = "rockchip,rk3066-spi", },
    { .compatible = "rockchip,rk3188-spi", },
    { .compatible = "rockchip,rk3228-spi", },
    { .compatible = "rockchip,rk3288-spi", },
    { .compatible = "rockchip,rk3308-spi", },
    { .compatible = "rockchip,rk3328-spi", },
    { .compatible = "rockchip,rk3368-spi", },
    { .compatible = "rockchip,rk3399-spi", },
    { .compatible = "rockchip,rv1108-spi", },
    { .compatible = "rockchip,rv1126-spi", },
    { },
    };
    MODULE_DEVICE_TABLE(of, rockchip_spi_dt_match);
    static struct platform_driver rockchip_spi_driver = {
    .driver = {
    .name	= DRIVER_NAME,
    .pm = pm_ptr(&rockchip_spi_pm),
    .of_match_table = of_match_ptr(rockchip_spi_dt_match),
    },
    .probe = rockchip_spi_probe,
    .remove = rockchip_spi_remove,
    };
    module_platform_driver(rockchip_spi_driver);
    MODULE_AUTHOR("Addy Ke <addy.ke@rock-chips.com>");
    MODULE_DESCRIPTION("ROCKCHIP SPI Controller Driver");
    MODULE_LICENSE("GPL v2");

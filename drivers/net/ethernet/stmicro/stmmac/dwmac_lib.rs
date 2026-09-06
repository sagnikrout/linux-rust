//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac_lib.c
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
    Copyright (C) 2007-2009  STMicroelectronics Ltd
    Author: Giuseppe Cavallaro <peppe.cavallaro@st.com>
//

pub const GMAC_HI_REG_AE: c_uint = 0x80000000;
#[no_mangle]
pub unsafe extern "C" fn dwmac_dma_reset(ioaddr: *mut void __iomem) -> c_int {
    int dwmac_dma_reset(void __iomem *ioaddr)
    {
    let mut value: u32 = readl(ioaddr + DMA_BUS_MODE);
// DMA SW reset
    value |= DMA_BUS_MODE_SFT_RESET;
    writel(value, ioaddr + DMA_BUS_MODE);
    return readl_poll_timeout(ioaddr + DMA_BUS_MODE, value,
    !(value & DMA_BUS_MODE_SFT_RESET),
    10000, 200000);
    }
// CSR1 enables the transmit DMA to check for new descriptor
#[no_mangle]
pub unsafe extern "C" fn dwmac_enable_dma_transmission(ioaddr: *mut void __iomem, chan: u32) {
    void dwmac_enable_dma_transmission(void __iomem *ioaddr, u32 chan)
    {
    writel(1, ioaddr + DMA_CHAN_XMT_POLL_DEMAND(chan));
    }
#[no_mangle]
pub unsafe extern "C" fn dwmac_enable_dma_reception(ioaddr: *mut void __iomem, chan: u32) {
    void dwmac_enable_dma_reception(void __iomem *ioaddr, u32 chan)
    {
    writel(1, ioaddr + DMA_CHAN_RCV_POLL_DEMAND(chan));
    }
    void dwmac_enable_dma_irq(struct stmmac_priv *priv, void __iomem *ioaddr,
    u32 chan, bool rx, bool tx)
    {
    let mut value: u32 = readl(ioaddr + DMA_CHAN_INTR_ENA(chan));
    if (rx)
    value |= DMA_INTR_DEFAULT_RX;
    if (tx)
    value |= DMA_INTR_DEFAULT_TX;
    writel(value, ioaddr + DMA_CHAN_INTR_ENA(chan));
    }
    void dwmac_disable_dma_irq(struct stmmac_priv *priv, void __iomem *ioaddr,
    u32 chan, bool rx, bool tx)
    {
    let mut value: u32 = readl(ioaddr + DMA_CHAN_INTR_ENA(chan));
    if (rx)
    value &= ~DMA_INTR_DEFAULT_RX;
    if (tx)
    value &= ~DMA_INTR_DEFAULT_TX;
    writel(value, ioaddr + DMA_CHAN_INTR_ENA(chan));
    }
    void dwmac_dma_start_tx(struct stmmac_priv *priv, void __iomem *ioaddr,
    u32 chan)
    {
    let mut value: u32 = readl(ioaddr + DMA_CHAN_CONTROL(chan));
    value |= DMA_CONTROL_ST;
    writel(value, ioaddr + DMA_CHAN_CONTROL(chan));
    }
#[no_mangle]
pub unsafe extern "C" fn dwmac_dma_stop_tx(priv: *mut stmmac_priv, ioaddr: *mut void __iomem, chan: u32) {
    void dwmac_dma_stop_tx(struct stmmac_priv *priv, void __iomem *ioaddr, u32 chan)
    {
    let mut value: u32 = readl(ioaddr + DMA_CHAN_CONTROL(chan));
    value &= ~DMA_CONTROL_ST;
    writel(value, ioaddr + DMA_CHAN_CONTROL(chan));
    }
    void dwmac_dma_start_rx(struct stmmac_priv *priv, void __iomem *ioaddr,
    u32 chan)
    {
    let mut value: u32 = readl(ioaddr + DMA_CHAN_CONTROL(chan));
    value |= DMA_CONTROL_SR;
    writel(value, ioaddr + DMA_CHAN_CONTROL(chan));
    }
#[no_mangle]
pub unsafe extern "C" fn dwmac_dma_stop_rx(priv: *mut stmmac_priv, ioaddr: *mut void __iomem, chan: u32) {
    void dwmac_dma_stop_rx(struct stmmac_priv *priv, void __iomem *ioaddr, u32 chan)
    {
    let mut value: u32 = readl(ioaddr + DMA_CHAN_CONTROL(chan));
    value &= ~DMA_CONTROL_SR;
    writel(value, ioaddr + DMA_CHAN_CONTROL(chan));
    }

#[no_mangle]
unsafe extern "C" fn show_tx_process_state(status: c_uint) {
    static void show_tx_process_state(unsigned int status)
    {
    switch (FIELD_GET(DMA_STATUS_TS_MASK, status)) {
    case 0:
    pr_debug("- TX (Stopped): Reset or Stop command\n");
    break;
    case 1:
    pr_debug("- TX (Running): Fetching the Tx desc\n");
    break;
    case 2:
    pr_debug("- TX (Running): Waiting for end of tx\n");
    break;
    case 3:
    pr_debug("- TX (Running): Reading the data "
    "and queuing the data into the Tx buf\n");
    break;
    case 6:
    pr_debug("- TX (Suspended): Tx Buff Underflow "
    "or an unavailable Transmit descriptor\n");
    break;
    case 7:
    pr_debug("- TX (Running): Closing Tx descriptor\n");
    break;
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn show_rx_process_state(status: c_uint) {
    static void show_rx_process_state(unsigned int status)
    {
    switch (FIELD_GET(DMA_STATUS_RS_MASK, status)) {
    case 0:
    pr_debug("- RX (Stopped): Reset or Stop command\n");
    break;
    case 1:
    pr_debug("- RX (Running): Fetching the Rx desc\n");
    break;
    case 2:
    pr_debug("- RX (Running): Checking for end of pkt\n");
    break;
    case 3:
    pr_debug("- RX (Running): Waiting for Rx pkt\n");
    break;
    case 4:
    pr_debug("- RX (Suspended): Unavailable Rx buf\n");
    break;
    case 5:
    pr_debug("- RX (Running): Closing Rx descriptor\n");
    break;
    case 6:
    pr_debug("- RX(Running): Flushing the current frame"
    " from the Rx buf\n");
    break;
    case 7:
    pr_debug("- RX (Running): Queuing the Rx frame"
    " from the Rx buf into memory\n");
    break;
    default:
    break;
    }
    }

    int dwmac_dma_interrupt(struct stmmac_priv *priv, void __iomem *ioaddr,
    struct stmmac_extra_stats *x, u32 chan, u32 dir)
    {
    struct stmmac_pcpu_stats *stats = this_cpu_ptr(priv.xstats.pcpu_stats);
    let mut ret: c_int = 0;
// read the status register (CSR5)
    let mut intr_status: u32 = readl(ioaddr + DMA_CHAN_STATUS(chan));

// Enable it to monitor DMA rx/tx status in case of critical problems
    pr_debug("%s: [CSR5: 0x%08x]\n", __func__, intr_status);
    show_tx_process_state(intr_status);
    show_rx_process_state(intr_status);

    if (dir == DMA_DIR_RX)
    intr_status &= DMA_STATUS_MSK_RX;
#[no_mangle]
pub unsafe extern "C" fn if(DMA_DIR_TX: dir ==) -> else {
    else if (dir == DMA_DIR_TX)
    intr_status &= DMA_STATUS_MSK_TX;
// ABNORMAL interrupts
    if (unlikely(intr_status & DMA_STATUS_AIS)) {
    if (unlikely(intr_status & DMA_STATUS_UNF)) {
    ret = tx_hard_error_bump_tc;
    x.tx_undeflow_irq++;
    }
    if (unlikely(intr_status & DMA_STATUS_TJT))
    x.tx_jabber_irq++;
    if (unlikely(intr_status & DMA_STATUS_OVF))
    x.rx_overflow_irq++;
    if (unlikely(intr_status & DMA_STATUS_RU))
    x.rx_buf_unav_irq++;
    if (unlikely(intr_status & DMA_STATUS_RPS))
    x.rx_process_stopped_irq++;
    if (unlikely(intr_status & DMA_STATUS_RWT))
    x.rx_watchdog_irq++;
    if (unlikely(intr_status & DMA_STATUS_ETI))
    x.tx_early_irq++;
    if (unlikely(intr_status & DMA_STATUS_TPS)) {
    x.tx_process_stopped_irq++;
    ret = tx_hard_error;
    }
    if (unlikely(intr_status & DMA_STATUS_FBI)) {
    x.fatal_bus_error_irq++;
    ret = tx_hard_error;
    }
    }
// TX/RX NORMAL interrupts
    if (likely(intr_status & DMA_STATUS_NIS)) {
    if (likely(intr_status & DMA_STATUS_RI)) {
    let mut value: u32 = readl(ioaddr + DMA_INTR_ENA);
// to schedule NAPI on real RIE event.
    if (likely(value & DMA_INTR_ENA_RIE)) {
    u64_stats_update_begin(&stats.syncp);
    u64_stats_inc(&stats.rx_normal_irq_n[chan]);
    u64_stats_update_end(&stats.syncp);
    ret |= handle_rx;
    }
    }
    if (likely(intr_status & DMA_STATUS_TI)) {
    u64_stats_update_begin(&stats.syncp);
    u64_stats_inc(&stats.tx_normal_irq_n[chan]);
    u64_stats_update_end(&stats.syncp);
    ret |= handle_tx;
    }
    if (unlikely(intr_status & DMA_STATUS_ERI))
    x.rx_early_irq++;
    }
// Optional hardware blocks, interrupts should be disabled
    if (unlikely(intr_status &
    (DMA_STATUS_GPI | DMA_STATUS_GMI | DMA_STATUS_GLI)))
    pr_warn("%s: unexpected status %08x\n", __func__, intr_status);
// Clear the interrupt by writing a logic 1 to the CSR5[15-0]
    writel((intr_status & 0x1ffff), ioaddr + DMA_STATUS);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dwmac_dma_flush_tx_fifo(ioaddr: *mut void __iomem) {
    void dwmac_dma_flush_tx_fifo(void __iomem *ioaddr)
    {
    let mut csr6: u32 = readl(ioaddr + DMA_CONTROL);
    writel((csr6 | DMA_CONTROL_FTF), ioaddr + DMA_CONTROL);
    do {} while ((readl(ioaddr + DMA_CONTROL) & DMA_CONTROL_FTF));
    }
    void stmmac_set_mac_addr(void __iomem *ioaddr, const u8 addr[6],
    unsigned int high, unsigned int low)
    {
    u32 data;
    data = (addr[5] << 8) | addr[4];
// For MAC Addr registers we have to set the Address Enable (AE)
// bit that has no effect on the High Reg 0 where the bit 31 (MO)
// is RO.
//
    writel(data | GMAC_HI_REG_AE, ioaddr + high);
    data = (addr[3] << 24) | (addr[2] << 16) | (addr[1] << 8) | addr[0];
    writel(data, ioaddr + low);
    }
    EXPORT_SYMBOL_GPL(stmmac_set_mac_addr);
// Enable disable MAC RX/TX
#[no_mangle]
pub unsafe extern "C" fn stmmac_set_mac(ioaddr: *mut void __iomem, enable: bool) {
    void stmmac_set_mac(void __iomem *ioaddr, bool enable)
    {
    u32 old_val, value;
    old_val = readl(ioaddr + MAC_CTRL_REG);
    value = old_val;
    if (enable)
    value |= MAC_ENABLE_RX | MAC_ENABLE_TX;
    else
    value &= ~(MAC_ENABLE_TX | MAC_ENABLE_RX);
    if (value != old_val)
    writel(value, ioaddr + MAC_CTRL_REG);
    }
    void stmmac_get_mac_addr(void __iomem *ioaddr, unsigned char *addr,
    unsigned int high, unsigned int low)
    {
    unsigned int hi_addr, lo_addr;
// Read the MAC address from the hardware
    hi_addr = readl(ioaddr + high);
    lo_addr = readl(ioaddr + low);
// Extract the MAC address from the high and low words
    addr[0] = lo_addr & 0xff;
    addr[1] = (lo_addr >> 8) & 0xff;
    addr[2] = (lo_addr >> 16) & 0xff;
    addr[3] = (lo_addr >> 24) & 0xff;
    addr[4] = hi_addr & 0xff;
    addr[5] = (hi_addr >> 8) & 0xff;
    }
    EXPORT_SYMBOL_GPL(stmmac_get_mac_addr);

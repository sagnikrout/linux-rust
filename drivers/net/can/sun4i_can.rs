//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/sun4i_can.c
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


//
// sun4i_can.c - CAN bus controller driver for Allwinner SUN4I&SUN7I based SoCs
//
// Copyright (C) 2013 Peter Chen
// Copyright (C) 2015 Gerhard Bertelsmann
// All rights reserved.
//
// Parts of this software are based on (derived from) the SJA1000 code by:
// Copyright (C) 2014 Oliver Hartkopp <oliver.hartkopp@volkswagen.de>
// Copyright (C) 2007 Wolfgang Grandegger <wg@grandegger.com>
// Copyright (C) 2002-2007 Volkswagen Group Electronic Research
// Copyright (C) 2003 Matthias Brukner, Trajet Gmbh, Rebenring 33,
// 38106 Braunschweig, GERMANY
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of Volkswagen nor the names of its contributors
// may be used to endorse or promote products derived from this software
// without specific prior written permission.
//
// Alternatively, provided that this notice is retained in full, this
// software may be distributed under the terms of the GNU General
// Public License ("GPL") version 2, in which case the provisions of the
// GPL apply INSTEAD OF those given above.
//
// The provided data structures and external interfaces from this code
// are not restricted to be used by modules with a GPL compatible license.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
// DAMAGE.
//

// Registers address (physical base address 0x01C2BC00)
pub const SUN4I_REG_MSEL_ADDR: c_uint = 0x0000	/* CAN Mode Select */;
pub const SUN4I_REG_CMD_ADDR: c_uint = 0x0004	/* CAN Command */;
pub const SUN4I_REG_STA_ADDR: c_uint = 0x0008	/* CAN Status */;
pub const SUN4I_REG_INT_ADDR: c_uint = 0x000c	/* CAN Interrupt Flag */;
pub const SUN4I_REG_INTEN_ADDR: c_uint = 0x0010	/* CAN Interrupt Enable */;
pub const SUN4I_REG_BTIME_ADDR: c_uint = 0x0014	/* CAN Bus Timing 0 */;
pub const SUN4I_REG_TEWL_ADDR: c_uint = 0x0018	/* CAN Tx Error Warning Limit */;
pub const SUN4I_REG_ERRC_ADDR: c_uint = 0x001c	/* CAN Error Counter */;
pub const SUN4I_REG_RMCNT_ADDR: c_uint = 0x0020	/* CAN Receive Message Counter */;
pub const SUN4I_REG_RBUFSA_ADDR: c_uint = 0x0024	/* CAN Receive Buffer Start Address */;
pub const SUN4I_REG_BUF0_ADDR: c_uint = 0x0040	/* CAN Tx/Rx Buffer 0 */;
pub const SUN4I_REG_BUF1_ADDR: c_uint = 0x0044	/* CAN Tx/Rx Buffer 1 */;
pub const SUN4I_REG_BUF2_ADDR: c_uint = 0x0048	/* CAN Tx/Rx Buffer 2 */;
pub const SUN4I_REG_BUF3_ADDR: c_uint = 0x004c	/* CAN Tx/Rx Buffer 3 */;
pub const SUN4I_REG_BUF4_ADDR: c_uint = 0x0050	/* CAN Tx/Rx Buffer 4 */;
pub const SUN4I_REG_BUF5_ADDR: c_uint = 0x0054	/* CAN Tx/Rx Buffer 5 */;
pub const SUN4I_REG_BUF6_ADDR: c_uint = 0x0058	/* CAN Tx/Rx Buffer 6 */;
pub const SUN4I_REG_BUF7_ADDR: c_uint = 0x005c	/* CAN Tx/Rx Buffer 7 */;
pub const SUN4I_REG_BUF8_ADDR: c_uint = 0x0060	/* CAN Tx/Rx Buffer 8 */;
pub const SUN4I_REG_BUF9_ADDR: c_uint = 0x0064	/* CAN Tx/Rx Buffer 9 */;
pub const SUN4I_REG_BUF10_ADDR: c_uint = 0x0068	/* CAN Tx/Rx Buffer 10 */;
pub const SUN4I_REG_BUF11_ADDR: c_uint = 0x006c	/* CAN Tx/Rx Buffer 11 */;
pub const SUN4I_REG_BUF12_ADDR: c_uint = 0x0070	/* CAN Tx/Rx Buffer 12 */;
pub const SUN4I_REG_ACPC_ADDR: c_uint = 0x0040	/* CAN Acceptance Code 0 */;
pub const SUN4I_REG_ACPM_ADDR: c_uint = 0x0044	/* CAN Acceptance Mask 0 */;
pub const SUN4I_REG_ACPC_ADDR_D1: c_uint = 0x0028	/* CAN Acceptance Code 0 on the D1 */;
pub const SUN4I_REG_ACPM_ADDR_D1: c_uint = 0x002C	/* CAN Acceptance Mask 0 on the D1 */;
pub const SUN4I_REG_RBUF_RBACK_START_ADDR: c_uint = 0x0180	/* CAN transmit buffer start */;
pub const SUN4I_REG_RBUF_RBACK_END_ADDR: c_uint = 0x01b0	/* CAN transmit buffer end */;
// Controller Register Description
// mode select register (r/w)
// offset:0x0000 default:0x0000_0001
//

// command register (w)
// offset:0x0004 default:0x0000_0000
//

// status register (r)
// offset:0x0008 default:0x0000_003c
//

// interrupt register (r)
// offset:0x000c default:0x0000_0000
//

// interrupt enable register (r/w)
// offset:0x0010 default:0x0000_0000
//

// error code

// filter mode
pub const SUN4I_FILTER_CLOSE: c_int = 0;
pub const SUN4I_SINGLE_FLTER_MODE: c_int = 1;
pub const SUN4I_DUAL_FILTER_MODE: c_int = 2;
// message buffer flags

// max. number of interrupts handled in ISR
pub const SUN4I_CAN_MAX_IRQ: c_int = 20;
pub const SUN4I_MODE_MAX_RETRIES: c_int = 100;
//
// struct sun4ican_quirks - Differences between SoC variants.
//
// @has_reset: SoC needs reset deasserted.
// @acp_offset: Offset of ACPC and ACPM registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4ican_quirks {
    pub has_reset: bool,
    pub acp_offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4ican_priv {
    pub can: can_priv,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub reset: *mut reset_control,
    pub /: *mut *mut spinlock_t cmdreg_lock; / lock for concurrent cmd register writes,
    pub acp_offset: c_int,
}

    static const struct can_bittiming_const sun4ican_bittiming_const = {
    .name = DRV_NAME,
    .tseg1_min = 1,
    .tseg1_max = 16,
    .tseg2_min = 1,
    .tseg2_max = 8,
    .sjw_max = 4,
    .brp_min = 1,
    .brp_max = 64,
    .brp_inc = 1,
    };
#[no_mangle]
unsafe extern "C" fn sun4i_can_write_cmdreg(priv: *mut sun4ican_priv, val: u8) {
    static void sun4i_can_write_cmdreg(struct sun4ican_priv *priv, u8 val)
    {
    unsigned long flags;
    spin_lock_irqsave(&priv.cmdreg_lock, flags);
    writel(val, priv.base + SUN4I_REG_CMD_ADDR);
    spin_unlock_irqrestore(&priv.cmdreg_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn set_normal_mode(dev: *mut net_device) -> c_int {
    static int set_normal_mode(struct net_device *dev)
    {
    struct sun4ican_priv *priv = netdev_priv(dev);
    let mut retry: c_int = SUN4I_MODE_MAX_RETRIES;
    let mut mod_reg_val: u32 = 0;
    do {
    mod_reg_val = readl(priv.base + SUN4I_REG_MSEL_ADDR);
    mod_reg_val &= ~SUN4I_MSEL_RESET_MODE;
    writel(mod_reg_val, priv.base + SUN4I_REG_MSEL_ADDR);
    } while (retry-- && (mod_reg_val & SUN4I_MSEL_RESET_MODE));
    if (readl(priv.base + SUN4I_REG_MSEL_ADDR) & SUN4I_MSEL_RESET_MODE) {
    netdev_err(dev,
    "setting controller into normal mode failed!\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_reset_mode(dev: *mut net_device) -> c_int {
    static int set_reset_mode(struct net_device *dev)
    {
    struct sun4ican_priv *priv = netdev_priv(dev);
    let mut retry: c_int = SUN4I_MODE_MAX_RETRIES;
    let mut mod_reg_val: u32 = 0;
    do {
    mod_reg_val = readl(priv.base + SUN4I_REG_MSEL_ADDR);
    mod_reg_val |= SUN4I_MSEL_RESET_MODE;
    writel(mod_reg_val, priv.base + SUN4I_REG_MSEL_ADDR);
    } while (retry-- && !(mod_reg_val & SUN4I_MSEL_RESET_MODE));
    if (!(readl(priv.base + SUN4I_REG_MSEL_ADDR) &
    SUN4I_MSEL_RESET_MODE)) {
    netdev_err(dev, "setting controller into reset mode failed!\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
// bittiming is called in reset_mode only
#[no_mangle]
unsafe extern "C" fn sun4ican_set_bittiming(dev: *mut net_device) -> c_int {
    static int sun4ican_set_bittiming(struct net_device *dev)
    {
    struct sun4ican_priv *priv = netdev_priv(dev);
    struct can_bittiming *bt = &priv.can.bittiming;
    u32 cfg;
    cfg = ((bt.brp - 1) & 0x3FF) |
    (((bt.sjw - 1) & 0x3) << 14) |
    (((bt.prop_seg + bt.phase_seg1 - 1) & 0xf) << 16) |
    (((bt.phase_seg2 - 1) & 0x7) << 20);
    if (priv.can.ctrlmode & CAN_CTRLMODE_3_SAMPLES)
    cfg |= 0x800000;
    netdev_dbg(dev, "setting BITTIMING=0x%08x\n", cfg);
    writel(cfg, priv.base + SUN4I_REG_BTIME_ADDR);
    return 0;
    }
    static int sun4ican_get_berr_counter(const struct net_device *dev,
    struct can_berr_counter *bec)
    {
    struct sun4ican_priv *priv = netdev_priv(dev);
    u32 errors;
    int err;
    err = clk_prepare_enable(priv.clk);
    if (err) {
    netdev_err(dev, "could not enable clock\n");
    return err;
    }
    errors = readl(priv.base + SUN4I_REG_ERRC_ADDR);
    bec.txerr = errors & 0xFF;
    bec.rxerr = (errors >> 16) & 0xFF;
    clk_disable_unprepare(priv.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_can_start(dev: *mut net_device) -> c_int {
    static int sun4i_can_start(struct net_device *dev)
    {
    struct sun4ican_priv *priv = netdev_priv(dev);
    int err;
    u32 mod_reg_val;
// we need to enter the reset mode
    err = set_reset_mode(dev);
    if (err) {
    netdev_err(dev, "could not enter reset mode\n");
    return err;
    }
// set filters - we accept all
    writel(0x00000000, priv.base + SUN4I_REG_ACPC_ADDR + priv.acp_offset);
    writel(0xFFFFFFFF, priv.base + SUN4I_REG_ACPM_ADDR + priv.acp_offset);
// clear error counters and error code capture
    writel(0, priv.base + SUN4I_REG_ERRC_ADDR);
// enable interrupts
    if (priv.can.ctrlmode & CAN_CTRLMODE_BERR_REPORTING)
    writel(0xFF, priv.base + SUN4I_REG_INTEN_ADDR);
    else
    writel(0xFF & ~SUN4I_INTEN_BERR,
    priv.base + SUN4I_REG_INTEN_ADDR);
// enter the selected mode
    mod_reg_val = readl(priv.base + SUN4I_REG_MSEL_ADDR);
    if (priv.can.ctrlmode & CAN_CTRLMODE_LOOPBACK)
    mod_reg_val |= SUN4I_MSEL_LOOPBACK_MODE;
#[no_mangle]
pub unsafe extern "C" fn if(CAN_CTRLMODE_LISTENONLY: priv->can.ctrlmode &) -> else {
    else if (priv.can.ctrlmode & CAN_CTRLMODE_LISTENONLY)
    mod_reg_val |= SUN4I_MSEL_LISTEN_ONLY_MODE;
    writel(mod_reg_val, priv.base + SUN4I_REG_MSEL_ADDR);
    err = sun4ican_set_bittiming(dev);
    if (err)
    return err;
// we are ready to enter the normal mode
    err = set_normal_mode(dev);
    if (err) {
    netdev_err(dev, "could not enter normal mode\n");
    return err;
    }
    priv.can.state = CAN_STATE_ERROR_ACTIVE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_can_stop(dev: *mut net_device) -> c_int {
    static int sun4i_can_stop(struct net_device *dev)
    {
    struct sun4ican_priv *priv = netdev_priv(dev);
    int err;
    priv.can.state = CAN_STATE_STOPPED;
// we need to enter reset mode
    err = set_reset_mode(dev);
    if (err) {
    netdev_err(dev, "could not enter reset mode\n");
    return err;
    }
// disable all interrupts
    writel(0, priv.base + SUN4I_REG_INTEN_ADDR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun4ican_set_mode(dev: *mut net_device, mode: enum can_mode) -> c_int {
    static int sun4ican_set_mode(struct net_device *dev, enum can_mode mode)
    {
    int err;
    switch (mode) {
    case CAN_MODE_START:
    err = sun4i_can_start(dev);
    if (err) {
    netdev_err(dev, "starting CAN controller failed!\n");
    return err;
    }
    if (netif_queue_stopped(dev))
    netif_wake_queue(dev);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
// transmit a CAN message
// message layout in the sk_buff should be like this:
// xx xx xx xx         ff         ll 00 11 22 33 44 55 66 77
// [ can_id ] [flags] [len] [can data (up to 8 bytes]
//
#[no_mangle]
unsafe extern "C" fn sun4ican_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t sun4ican_start_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct sun4ican_priv *priv = netdev_priv(dev);
    struct can_frame *cf = (struct can_frame *)skb.data;
    u8 dlc;
    u32 dreg, msg_flag_n;
    canid_t id;
    int i;
    if (can_dev_dropped_skb(dev, skb))
    return NETDEV_TX_OK;
    netif_stop_queue(dev);
    id = cf.can_id;
    dlc = cf.len;
    msg_flag_n = dlc;
    if (id & CAN_RTR_FLAG)
    msg_flag_n |= SUN4I_MSG_RTR_FLAG;
    if (id & CAN_EFF_FLAG) {
    msg_flag_n |= SUN4I_MSG_EFF_FLAG;
    dreg = SUN4I_REG_BUF5_ADDR;
    writel((id >> 21) & 0xFF, priv.base + SUN4I_REG_BUF1_ADDR);
    writel((id >> 13) & 0xFF, priv.base + SUN4I_REG_BUF2_ADDR);
    writel((id >> 5)  & 0xFF, priv.base + SUN4I_REG_BUF3_ADDR);
    writel((id << 3)  & 0xF8, priv.base + SUN4I_REG_BUF4_ADDR);
    } else {
    dreg = SUN4I_REG_BUF3_ADDR;
    writel((id >> 3) & 0xFF, priv.base + SUN4I_REG_BUF1_ADDR);
    writel((id << 5) & 0xE0, priv.base + SUN4I_REG_BUF2_ADDR);
    }
    for (i = 0; i < dlc; i++)
    writel(cf.data[i], priv.base + (dreg + i * 4));
    writel(msg_flag_n, priv.base + SUN4I_REG_BUF0_ADDR);
    can_put_echo_skb(skb, dev, 0, 0);
    if (priv.can.ctrlmode & CAN_CTRLMODE_LOOPBACK)
    sun4i_can_write_cmdreg(priv, SUN4I_CMD_SELF_RCV_REQ);
    else
    sun4i_can_write_cmdreg(priv, SUN4I_CMD_TRANS_REQ);
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_can_rx(dev: *mut net_device) {
    static void sun4i_can_rx(struct net_device *dev)
    {
    struct sun4ican_priv *priv = netdev_priv(dev);
    struct net_device_stats *stats = &dev.stats;
    struct can_frame *cf;
    struct sk_buff *skb;
    u8 fi;
    u32 dreg;
    canid_t id;
    int i;
// create zero'ed CAN frame buffer
    skb = alloc_can_skb(dev, &cf);
    if (!skb)
    return;
    fi = readl(priv.base + SUN4I_REG_BUF0_ADDR);
    cf.len = can_cc_dlc2len(fi & 0x0F);
    if (fi & SUN4I_MSG_EFF_FLAG) {
    dreg = SUN4I_REG_BUF5_ADDR;
    id = (readl(priv.base + SUN4I_REG_BUF1_ADDR) << 21) |
    (readl(priv.base + SUN4I_REG_BUF2_ADDR) << 13) |
    (readl(priv.base + SUN4I_REG_BUF3_ADDR) << 5)  |
    ((readl(priv.base + SUN4I_REG_BUF4_ADDR) >> 3)  & 0x1f);
    id |= CAN_EFF_FLAG;
    } else {
    dreg = SUN4I_REG_BUF3_ADDR;
    id = (readl(priv.base + SUN4I_REG_BUF1_ADDR) << 3) |
    ((readl(priv.base + SUN4I_REG_BUF2_ADDR) >> 5) & 0x7);
    }
// remote frame ?
    if (fi & SUN4I_MSG_RTR_FLAG) {
    id |= CAN_RTR_FLAG;
    } else {
    for (i = 0; i < cf.len; i++)
    cf.data[i] = readl(priv.base + dreg + i * 4);
    stats.rx_bytes += cf.len;
    }
    stats.rx_packets++;
    cf.can_id = id;
    sun4i_can_write_cmdreg(priv, SUN4I_CMD_RELEASE_RBUF);
    netif_rx(skb);
    }
#[no_mangle]
unsafe extern "C" fn sun4i_can_err(dev: *mut net_device, isrc: u8, status: u8) -> c_int {
    static int sun4i_can_err(struct net_device *dev, u8 isrc, u8 status)
    {
    struct sun4ican_priv *priv = netdev_priv(dev);
    struct net_device_stats *stats = &dev.stats;
    struct can_frame *cf;
    struct sk_buff *skb;
    let mut state: enum can_state = priv.can.state;
    enum can_state rx_state, tx_state;
    unsigned int rxerr, txerr, errc;
    u32 ecc, alc;
// we don't skip if alloc fails because we want the stats anyhow
    skb = alloc_can_err_skb(dev, &cf);
    errc = readl(priv.base + SUN4I_REG_ERRC_ADDR);
    rxerr = (errc >> 16) & 0xFF;
    txerr = errc & 0xFF;
    if (isrc & SUN4I_INT_DATA_OR) {
// data overrun interrupt
    netdev_dbg(dev, "data overrun interrupt\n");
    if (likely(skb)) {
    cf.can_id |= CAN_ERR_CRTL;
    cf.data[1] = CAN_ERR_CRTL_RX_OVERFLOW;
    }
    stats.rx_over_errors++;
    stats.rx_errors++;
// reset the CAN IP by entering reset mode
// ignoring timeout error
//
    set_reset_mode(dev);
    set_normal_mode(dev);
// clear bit
    sun4i_can_write_cmdreg(priv, SUN4I_CMD_CLEAR_OR_FLAG);
    }
    if (isrc & SUN4I_INT_ERR_WRN) {
// error warning interrupt
    netdev_dbg(dev, "error warning interrupt\n");
    if (status & SUN4I_STA_BUS_OFF)
    state = CAN_STATE_BUS_OFF;
#[no_mangle]
pub unsafe extern "C" fn if(SUN4I_STA_ERR_STA: status &) -> else {
    else if (status & SUN4I_STA_ERR_STA)
    state = CAN_STATE_ERROR_WARNING;
    else
    state = CAN_STATE_ERROR_ACTIVE;
    }
    if (likely(skb) && state != CAN_STATE_BUS_OFF) {
    cf.can_id |= CAN_ERR_CNT;
    cf.data[6] = txerr;
    cf.data[7] = rxerr;
    }
    if (isrc & SUN4I_INT_BUS_ERR) {
// bus error interrupt
    netdev_dbg(dev, "bus error interrupt\n");
    priv.can.can_stats.bus_error++;
    ecc = readl(priv.base + SUN4I_REG_STA_ADDR);
    if (likely(skb)) {
    cf.can_id |= CAN_ERR_PROT | CAN_ERR_BUSERROR;
    switch (ecc & SUN4I_STA_MASK_ERR) {
    case SUN4I_STA_BIT_ERR:
    cf.data[2] |= CAN_ERR_PROT_BIT;
    break;
    case SUN4I_STA_FORM_ERR:
    cf.data[2] |= CAN_ERR_PROT_FORM;
    break;
    case SUN4I_STA_STUFF_ERR:
    cf.data[2] |= CAN_ERR_PROT_STUFF;
    break;
    default:
    cf.data[3] = (ecc & SUN4I_STA_ERR_SEG_CODE)
    >> 16;
    break;
    }
    }
// error occurred during transmission?
    if ((ecc & SUN4I_STA_ERR_DIR) == 0) {
    if (likely(skb))
    cf.data[2] |= CAN_ERR_PROT_TX;
    stats.tx_errors++;
    } else {
    stats.rx_errors++;
    }
    }
    if (isrc & SUN4I_INT_ERR_PASSIVE) {
// error passive interrupt
    netdev_dbg(dev, "error passive interrupt\n");
    if (state == CAN_STATE_ERROR_PASSIVE)
    state = CAN_STATE_ERROR_WARNING;
    else
    state = CAN_STATE_ERROR_PASSIVE;
    }
    if (isrc & SUN4I_INT_ARB_LOST) {
// arbitration lost interrupt
    netdev_dbg(dev, "arbitration lost interrupt\n");
    alc = readl(priv.base + SUN4I_REG_STA_ADDR);
    priv.can.can_stats.arbitration_lost++;
    if (likely(skb)) {
    cf.can_id |= CAN_ERR_LOSTARB;
    cf.data[0] = (alc >> 8) & 0x1f;
    }
    }
    if (state != priv.can.state) {
    tx_state = txerr >= rxerr ? state : 0;
    rx_state = txerr <= rxerr ? state : 0;
// The skb allocation might fail, but can_change_state()
// handles cf == NULL.
//
    can_change_state(dev, cf, tx_state, rx_state);
    if (state == CAN_STATE_BUS_OFF)
    can_bus_off(dev);
    }
    if (likely(skb))
    netif_rx(skb);
    else
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_can_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sun4i_can_interrupt(int irq, void *dev_id)
    {
    struct net_device *dev = (struct net_device *)dev_id;
    struct sun4ican_priv *priv = netdev_priv(dev);
    struct net_device_stats *stats = &dev.stats;
    u8 isrc, status;
    let mut n: c_int = 0;
    while ((n < SUN4I_CAN_MAX_IRQ) &&
    (isrc = readl(priv.base + SUN4I_REG_INT_ADDR))) {
    n++;
    status = readl(priv.base + SUN4I_REG_STA_ADDR);
    if (isrc & SUN4I_INT_WAKEUP)
    netdev_warn(dev, "wakeup interrupt\n");
    if (isrc & SUN4I_INT_TBUF_VLD) {
// transmission complete interrupt
    stats.tx_bytes += can_get_echo_skb(dev, 0, core::ptr::null_mut());
    stats.tx_packets++;
    netif_wake_queue(dev);
    }
    if ((isrc & SUN4I_INT_RBUF_VLD) &&
    !(isrc & SUN4I_INT_DATA_OR)) {
// receive interrupt - don't read if overrun occurred
    while (status & SUN4I_STA_RBUF_RDY) {
// RX buffer is not empty
    sun4i_can_rx(dev);
    status = readl(priv.base + SUN4I_REG_STA_ADDR);
    }
    }
    if (isrc &
    (SUN4I_INT_DATA_OR | SUN4I_INT_ERR_WRN | SUN4I_INT_BUS_ERR |
    SUN4I_INT_ERR_PASSIVE | SUN4I_INT_ARB_LOST)) {
// error interrupt
    if (sun4i_can_err(dev, isrc, status))
    netdev_err(dev, "can't allocate buffer - clearing pending interrupts\n");
    }
// clear interrupts
    writel(isrc, priv.base + SUN4I_REG_INT_ADDR);
    readl(priv.base + SUN4I_REG_INT_ADDR);
    }
    if (n >= SUN4I_CAN_MAX_IRQ)
    netdev_dbg(dev, "%d messages handled in ISR", n);
    return (n) ? IRQ_HANDLED : IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn sun4ican_open(dev: *mut net_device) -> c_int {
    static int sun4ican_open(struct net_device *dev)
    {
    struct sun4ican_priv *priv = netdev_priv(dev);
    int err;
// common open
    err = open_candev(dev);
    if (err)
    return err;
// register interrupt handler
    err = request_irq(dev.irq, sun4i_can_interrupt, 0, dev.name, dev);
    if (err) {
    netdev_err(dev, "request_irq err: %d\n", err);
    goto exit_irq;
    }
// software reset deassert
    err = reset_control_deassert(priv.reset);
    if (err) {
    netdev_err(dev, "could not deassert CAN reset\n");
    goto exit_soft_reset;
    }
// turn on clocking for CAN peripheral block
    err = clk_prepare_enable(priv.clk);
    if (err) {
    netdev_err(dev, "could not enable CAN peripheral clock\n");
    goto exit_clock;
    }
    err = sun4i_can_start(dev);
    if (err) {
    netdev_err(dev, "could not start CAN peripheral\n");
    goto exit_can_start;
    }
    netif_start_queue(dev);
    return 0;
    exit_can_start:
    clk_disable_unprepare(priv.clk);
    exit_clock:
    reset_control_assert(priv.reset);
    exit_soft_reset:
    free_irq(dev.irq, dev);
    exit_irq:
    close_candev(dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sun4ican_close(dev: *mut net_device) -> c_int {
    static int sun4ican_close(struct net_device *dev)
    {
    struct sun4ican_priv *priv = netdev_priv(dev);
    netif_stop_queue(dev);
    sun4i_can_stop(dev);
    clk_disable_unprepare(priv.clk);
    reset_control_assert(priv.reset);
    free_irq(dev.irq, dev);
    close_candev(dev);
    return 0;
    }
    static const struct net_device_ops sun4ican_netdev_ops = {
    .ndo_open = sun4ican_open,
    .ndo_stop = sun4ican_close,
    .ndo_start_xmit = sun4ican_start_xmit,
    };
    static const struct ethtool_ops sun4ican_ethtool_ops = {
    .get_ts_info = ethtool_op_get_ts_info,
    };
    static const struct sun4ican_quirks sun4ican_quirks_a10 = {
    .has_reset = false,
    .acp_offset = 0,
    };
    static const struct sun4ican_quirks sun4ican_quirks_r40 = {
    .has_reset = true,
    .acp_offset = 0,
    };
    static const struct sun4ican_quirks sun4ican_quirks_d1 = {
    .has_reset = true,
    .acp_offset = (SUN4I_REG_ACPC_ADDR_D1 - SUN4I_REG_ACPC_ADDR),
    };
    static const struct of_device_id sun4ican_of_match[] = {
    {
    .compatible = "allwinner,sun4i-a10-can",
    .data = &sun4ican_quirks_a10
    }, {
    .compatible = "allwinner,sun7i-a20-can",
    .data = &sun4ican_quirks_a10
    }, {
    .compatible = "allwinner,sun8i-r40-can",
    .data = &sun4ican_quirks_r40
    }, {
    .compatible = "allwinner,sun20i-d1-can",
    .data = &sun4ican_quirks_d1
    }, {
// sentinel
    },
    };
    MODULE_DEVICE_TABLE(of, sun4ican_of_match);
#[no_mangle]
unsafe extern "C" fn sun4ican_remove(pdev: *mut platform_device) {
    static void sun4ican_remove(struct platform_device *pdev)
    {
    struct net_device *dev = platform_get_drvdata(pdev);
    unregister_netdev(dev);
    free_candev(dev);
    }
#[no_mangle]
unsafe extern "C" fn sun4ican_probe(pdev: *mut platform_device) -> c_int {
    static int sun4ican_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct clk *clk;
    struct reset_control *reset = core::ptr::null_mut();
    void __iomem *addr;
    int err, irq;
    struct net_device *dev;
    struct sun4ican_priv *priv;
    const struct sun4ican_quirks *quirks;
    quirks = of_device_get_match_data(&pdev.dev);
    if (!quirks) {
    dev_err(&pdev.dev, "failed to determine the quirks to use\n");
    err = -ENODEV;
    goto exit;
    }
    if (quirks.has_reset) {
    reset = devm_reset_control_get_exclusive(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(reset)) {
    dev_err(&pdev.dev, "unable to request reset\n");
    err = PTR_ERR(reset);
    goto exit;
    }
    }
    clk = of_clk_get(np, 0);
    if (IS_ERR(clk)) {
    dev_err(&pdev.dev, "unable to request clock\n");
    err = -ENODEV;
    goto exit;
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    err = -ENODEV;
    goto exit;
    }
    addr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(addr)) {
    err = PTR_ERR(addr);
    goto exit;
    }
    dev = alloc_candev(sizeof(struct sun4ican_priv), 1);
    if (!dev) {
    dev_err(&pdev.dev,
    "could not allocate memory for CAN device\n");
    err = -ENOMEM;
    goto exit;
    }
    dev.netdev_ops = &sun4ican_netdev_ops;
    dev.ethtool_ops = &sun4ican_ethtool_ops;
    dev.irq = irq;
    dev.flags |= IFF_ECHO;
    priv = netdev_priv(dev);
    priv.can.clock.freq = clk_get_rate(clk);
    priv.can.bittiming_const = &sun4ican_bittiming_const;
    priv.can.do_set_mode = sun4ican_set_mode;
    priv.can.do_get_berr_counter = sun4ican_get_berr_counter;
    priv.can.ctrlmode_supported = CAN_CTRLMODE_BERR_REPORTING |
    CAN_CTRLMODE_LISTENONLY |
    CAN_CTRLMODE_LOOPBACK |
    CAN_CTRLMODE_3_SAMPLES;
    priv.base = addr;
    priv.clk = clk;
    priv.reset = reset;
    priv.acp_offset = quirks.acp_offset;
    spin_lock_init(&priv.cmdreg_lock);
    platform_set_drvdata(pdev, dev);
    SET_NETDEV_DEV(dev, &pdev.dev);
    err = register_candev(dev);
    if (err) {
    dev_err(&pdev.dev, "registering %s failed (err=%d)\n",
    DRV_NAME, err);
    goto exit_free;
    }
    dev_info(&pdev.dev, "device registered (base=%p, irq=%d)\n",
    priv.base, dev.irq);
    return 0;
    exit_free:
    free_candev(dev);
    exit:
    return err;
    }
    static struct platform_driver sun4i_can_driver = {
    .driver = {
    .name = DRV_NAME,
    .of_match_table = sun4ican_of_match,
    },
    .probe = sun4ican_probe,
    .remove = sun4ican_remove,
    };
    module_platform_driver(sun4i_can_driver);
    MODULE_AUTHOR("Peter Chen <xingkongcp@gmail.com>");
    MODULE_AUTHOR("Gerhard Bertelsmann <info@gerhard-bertelsmann.de>");
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_DESCRIPTION("CAN driver for Allwinner SoCs (A10/A20/D1)");

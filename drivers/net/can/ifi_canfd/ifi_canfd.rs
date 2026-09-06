//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/ifi_canfd/ifi_canfd.c
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
// CAN bus driver for IFI CANFD controller
//
// Copyright (C) 2016 Marek Vasut <marex@denx.de>
//
// Details about this controller can be found at
// http://www.ifi-pld.de/IP/CANFD/canfd.html
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

pub const IFI_CANFD_STCMD: c_uint = 0x0;
pub const IFI_CANFD_STCMD_HARDRESET: c_uint = 0xDEADCAFD;

pub const IFI_CANFD_RXSTCMD: c_uint = 0x4;

pub const IFI_CANFD_TXSTCMD: c_uint = 0x8;

pub const IFI_CANFD_INTERRUPT: c_uint = 0xc;

pub const IFI_CANFD_IRQMASK: c_uint = 0x10;

pub const IFI_CANFD_TIME: c_uint = 0x14;
pub const IFI_CANFD_FTIME: c_uint = 0x18;
pub const IFI_CANFD_TIME_TIMEB_OFF: c_int = 0;
pub const IFI_CANFD_TIME_TIMEA_OFF: c_int = 8;
pub const IFI_CANFD_TIME_PRESCALE_OFF: c_int = 16;
pub const IFI_CANFD_TIME_SJW_OFF_7_9_8_8: c_int = 25;
pub const IFI_CANFD_TIME_SJW_OFF_4_12_6_6: c_int = 28;

pub const IFI_CANFD_TDELAY: c_uint = 0x1c;
pub const IFI_CANFD_TDELAY_DEFAULT: c_uint = 0xb;
pub const IFI_CANFD_TDELAY_MASK: c_uint = 0x3fff;

pub const IFI_CANFD_ERROR: c_uint = 0x20;
pub const IFI_CANFD_ERROR_TX_OFFSET: c_int = 0;
pub const IFI_CANFD_ERROR_TX_MASK: c_uint = 0xff;
pub const IFI_CANFD_ERROR_RX_OFFSET: c_int = 16;
pub const IFI_CANFD_ERROR_RX_MASK: c_uint = 0xff;
pub const IFI_CANFD_ERRCNT: c_uint = 0x24;
pub const IFI_CANFD_SUSPEND: c_uint = 0x28;
pub const IFI_CANFD_REPEAT: c_uint = 0x2c;
pub const IFI_CANFD_TRAFFIC: c_uint = 0x30;
pub const IFI_CANFD_TSCONTROL: c_uint = 0x34;
pub const IFI_CANFD_TSC: c_uint = 0x38;
pub const IFI_CANFD_TST: c_uint = 0x3c;
pub const IFI_CANFD_RES1: c_uint = 0x40;
pub const IFI_CANFD_ERROR_CTR: c_uint = 0x44;
pub const IFI_CANFD_ERROR_CTR_UNLOCK_MAGIC: c_uint = 0x21302899;

pub const IFI_CANFD_ERROR_CTR_BITPOSITION_OFFSET: c_int = 16;
pub const IFI_CANFD_ERROR_CTR_BITPOSITION_MASK: c_uint = 0xff;

pub const IFI_CANFD_PAR: c_uint = 0x48;
pub const IFI_CANFD_CANCLOCK: c_uint = 0x4c;
pub const IFI_CANFD_SYSCLOCK: c_uint = 0x50;
pub const IFI_CANFD_VER: c_uint = 0x54;
pub const IFI_CANFD_VER_REV_MASK: c_uint = 0xff;
pub const IFI_CANFD_VER_REV_MIN_SUPPORTED: c_uint = 0x15;
pub const IFI_CANFD_IP_ID: c_uint = 0x58;
pub const IFI_CANFD_IP_ID_VALUE: c_uint = 0xD073CAFD;
pub const IFI_CANFD_TEST: c_uint = 0x5c;
pub const IFI_CANFD_RXFIFO_TS_63_32: c_uint = 0x60;
pub const IFI_CANFD_RXFIFO_TS_31_0: c_uint = 0x64;
pub const IFI_CANFD_RXFIFO_DLC: c_uint = 0x68;
pub const IFI_CANFD_RXFIFO_DLC_DLC_OFFSET: c_int = 0;
pub const IFI_CANFD_RXFIFO_DLC_DLC_MASK: c_uint = 0xf;

pub const IFI_CANFD_RXFIFO_DLC_OBJ_OFFSET: c_int = 8;
pub const IFI_CANFD_RXFIFO_DLC_OBJ_MASK: c_uint = 0x1ff;
pub const IFI_CANFD_RXFIFO_DLC_FNR_OFFSET: c_int = 24;
pub const IFI_CANFD_RXFIFO_DLC_FNR_MASK: c_uint = 0xff;
pub const IFI_CANFD_RXFIFO_ID: c_uint = 0x6c;
pub const IFI_CANFD_RXFIFO_ID_ID_OFFSET: c_int = 0;

pub const IFI_CANFD_RXFIFO_ID_ID_STD_OFFSET: c_int = 0;
pub const IFI_CANFD_RXFIFO_ID_ID_STD_WIDTH: c_int = 10;

pub const IFI_CANFD_RXFIFO_ID_ID_XTD_OFFSET: c_int = 11;
pub const IFI_CANFD_RXFIFO_ID_ID_XTD_WIDTH: c_int = 18;

pub const IFI_CANFD_RXFIFO_DATA: c_uint = 0x70	/* 0x70..0xac */;
pub const IFI_CANFD_TXFIFO_SUSPEND_US: c_uint = 0xb0;
pub const IFI_CANFD_TXFIFO_REPEATCOUNT: c_uint = 0xb4;
pub const IFI_CANFD_TXFIFO_DLC: c_uint = 0xb8;
pub const IFI_CANFD_TXFIFO_DLC_DLC_OFFSET: c_int = 0;
pub const IFI_CANFD_TXFIFO_DLC_DLC_MASK: c_uint = 0xf;

pub const IFI_CANFD_TXFIFO_DLC_FNR_OFFSET: c_int = 24;
pub const IFI_CANFD_TXFIFO_DLC_FNR_MASK: c_uint = 0xff;
pub const IFI_CANFD_TXFIFO_ID: c_uint = 0xbc;
pub const IFI_CANFD_TXFIFO_ID_ID_OFFSET: c_int = 0;

pub const IFI_CANFD_TXFIFO_ID_ID_STD_OFFSET: c_int = 0;
pub const IFI_CANFD_TXFIFO_ID_ID_STD_WIDTH: c_int = 10;

pub const IFI_CANFD_TXFIFO_ID_ID_XTD_OFFSET: c_int = 11;
pub const IFI_CANFD_TXFIFO_ID_ID_XTD_WIDTH: c_int = 18;

pub const IFI_CANFD_TXFIFO_DATA: c_uint = 0xc0	/* 0xb0..0xfc */;

// IFI CANFD private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifi_canfd_priv {
    pub /: *mut *mut can_priv can; / must be the first member,
    pub napi: napi_struct,
    pub ndev: *mut net_device,
    pub base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn ifi_canfd_irq_enable(ndev: *mut net_device, enable: bool) {
    static void ifi_canfd_irq_enable(struct net_device *ndev, bool enable)
    {
    struct ifi_canfd_priv *priv = netdev_priv(ndev);
    let mut enirq: u32 = 0;
    if (enable) {
    enirq = IFI_CANFD_IRQMASK_TXFIFO_EMPTY |
    IFI_CANFD_IRQMASK_RXFIFO_NEMPTY |
    IFI_CANFD_IRQMASK_ERROR_STATE_CHG |
    IFI_CANFD_IRQMASK_ERROR_WARNING |
    IFI_CANFD_IRQMASK_ERROR_BUSOFF;
    if (priv.can.ctrlmode & CAN_CTRLMODE_BERR_REPORTING)
    enirq |= IFI_CANFD_INTERRUPT_ERROR_COUNTER;
    }
    writel(IFI_CANFD_IRQMASK_SET_ERR |
    IFI_CANFD_IRQMASK_SET_TS |
    IFI_CANFD_IRQMASK_SET_TX |
    IFI_CANFD_IRQMASK_SET_RX | enirq,
    priv.base + IFI_CANFD_IRQMASK);
    }
#[no_mangle]
unsafe extern "C" fn ifi_canfd_read_fifo(ndev: *mut net_device) {
    static void ifi_canfd_read_fifo(struct net_device *ndev)
    {
    struct net_device_stats *stats = &ndev.stats;
    struct ifi_canfd_priv *priv = netdev_priv(ndev);
    struct canfd_frame *cf;
    struct sk_buff *skb;
    const u32 rx_irq_mask = IFI_CANFD_INTERRUPT_RXFIFO_NEMPTY |
    IFI_CANFD_INTERRUPT_RXFIFO_NEMPTY_PER;
    u32 rxdlc, rxid;
    u32 dlc, id;
    int i;
    rxdlc = readl(priv.base + IFI_CANFD_RXFIFO_DLC);
    if (rxdlc & IFI_CANFD_RXFIFO_DLC_EDL)
    skb = alloc_canfd_skb(ndev, &cf);
    else
    skb = alloc_can_skb(ndev, (struct can_frame **)&cf);
    if (!skb) {
    stats.rx_dropped++;
    return;
    }
    dlc = (rxdlc >> IFI_CANFD_RXFIFO_DLC_DLC_OFFSET) &
    IFI_CANFD_RXFIFO_DLC_DLC_MASK;
    if (rxdlc & IFI_CANFD_RXFIFO_DLC_EDL)
    cf.len = can_fd_dlc2len(dlc);
    else
    cf.len = can_cc_dlc2len(dlc);
    rxid = readl(priv.base + IFI_CANFD_RXFIFO_ID);
    id = (rxid >> IFI_CANFD_RXFIFO_ID_ID_OFFSET);
    if (id & IFI_CANFD_RXFIFO_ID_IDE) {
    id &= IFI_CANFD_RXFIFO_ID_ID_XTD_MASK;
//
// In case the Extended ID frame is received, the standard
// and extended part of the ID are swapped in the register,
// so swap them back to obtain the correct ID.
//
    id = (id >> IFI_CANFD_RXFIFO_ID_ID_XTD_OFFSET) |
    ((id & IFI_CANFD_RXFIFO_ID_ID_STD_MASK) <<
    IFI_CANFD_RXFIFO_ID_ID_XTD_WIDTH);
    id |= CAN_EFF_FLAG;
    } else {
    id &= IFI_CANFD_RXFIFO_ID_ID_STD_MASK;
    }
    cf.can_id = id;
    if (rxdlc & IFI_CANFD_RXFIFO_DLC_ESI) {
    cf.flags |= CANFD_ESI;
    netdev_dbg(ndev, "ESI Error\n");
    }
    if (!(rxdlc & IFI_CANFD_RXFIFO_DLC_EDL) &&
    (rxdlc & IFI_CANFD_RXFIFO_DLC_RTR)) {
    cf.can_id |= CAN_RTR_FLAG;
    } else {
    if (rxdlc & IFI_CANFD_RXFIFO_DLC_BRS)
    cf.flags |= CANFD_BRS;
    for (i = 0; i < cf.len; i += 4) {
// (u32 *)(cf->data + i) =
    readl(priv.base + IFI_CANFD_RXFIFO_DATA + i);
    }
    stats.rx_bytes += cf.len;
    }
    stats.rx_packets++;
// Remove the packet from FIFO
    writel(IFI_CANFD_RXSTCMD_REMOVE_MSG, priv.base + IFI_CANFD_RXSTCMD);
    writel(rx_irq_mask, priv.base + IFI_CANFD_INTERRUPT);
    netif_receive_skb(skb);
    }
#[no_mangle]
unsafe extern "C" fn ifi_canfd_do_rx_poll(ndev: *mut net_device, quota: c_int) -> c_int {
    static int ifi_canfd_do_rx_poll(struct net_device *ndev, int quota)
    {
    struct ifi_canfd_priv *priv = netdev_priv(ndev);
    let mut pkts: u32 = 0;
    u32 rxst;
    rxst = readl(priv.base + IFI_CANFD_RXSTCMD);
    if (rxst & IFI_CANFD_RXSTCMD_EMPTY) {
    netdev_dbg(ndev, "No messages in RX FIFO\n");
    return 0;
    }
    for (;;) {
    if (rxst & IFI_CANFD_RXSTCMD_EMPTY)
    break;
    if (quota <= 0)
    break;
    ifi_canfd_read_fifo(ndev);
    quota--;
    pkts++;
    rxst = readl(priv.base + IFI_CANFD_RXSTCMD);
    }
    return pkts;
    }
#[no_mangle]
unsafe extern "C" fn ifi_canfd_handle_lost_msg(ndev: *mut net_device) -> c_int {
    static int ifi_canfd_handle_lost_msg(struct net_device *ndev)
    {
    struct net_device_stats *stats = &ndev.stats;
    struct sk_buff *skb;
    struct can_frame *frame;
    netdev_err(ndev, "RX FIFO overflow, message(s) lost.\n");
    stats.rx_errors++;
    stats.rx_over_errors++;
    skb = alloc_can_err_skb(ndev, &frame);
    if (unlikely(!skb))
    return 0;
    frame.can_id |= CAN_ERR_CRTL;
    frame.data[1] = CAN_ERR_CRTL_RX_OVERFLOW;
    netif_receive_skb(skb);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn ifi_canfd_handle_lec_err(ndev: *mut net_device) -> c_int {
    static int ifi_canfd_handle_lec_err(struct net_device *ndev)
    {
    struct ifi_canfd_priv *priv = netdev_priv(ndev);
    struct net_device_stats *stats = &ndev.stats;
    struct can_frame *cf;
    struct sk_buff *skb;
    let mut errctr: u32 = readl(priv.base + IFI_CANFD_ERROR_CTR);
    const u32 errmask = IFI_CANFD_ERROR_CTR_OVERLOAD_FIRST |
    IFI_CANFD_ERROR_CTR_ACK_ERROR_FIRST |
    IFI_CANFD_ERROR_CTR_BIT0_ERROR_FIRST |
    IFI_CANFD_ERROR_CTR_BIT1_ERROR_FIRST |
    IFI_CANFD_ERROR_CTR_STUFF_ERROR_FIRST |
    IFI_CANFD_ERROR_CTR_CRC_ERROR_FIRST |
    IFI_CANFD_ERROR_CTR_FORM_ERROR_FIRST;
    if (!(errctr & errmask))	/* No error happened. */
    return 0;
    priv.can.can_stats.bus_error++;
// Propagate the error condition to the CAN stack.
    skb = alloc_can_err_skb(ndev, &cf);
// Read the error counter register and check for new errors.
    if (likely(skb))
    cf.can_id |= CAN_ERR_PROT | CAN_ERR_BUSERROR;
    if (errctr & IFI_CANFD_ERROR_CTR_OVERLOAD_FIRST) {
    stats.rx_errors++;
    if (likely(skb))
    cf.data[2] |= CAN_ERR_PROT_OVERLOAD;
    }
    if (errctr & IFI_CANFD_ERROR_CTR_ACK_ERROR_FIRST) {
    stats.tx_errors++;
    if (likely(skb))
    cf.data[3] = CAN_ERR_PROT_LOC_ACK;
    }
    if (errctr & IFI_CANFD_ERROR_CTR_BIT0_ERROR_FIRST) {
    stats.tx_errors++;
    if (likely(skb))
    cf.data[2] |= CAN_ERR_PROT_BIT0;
    }
    if (errctr & IFI_CANFD_ERROR_CTR_BIT1_ERROR_FIRST) {
    stats.tx_errors++;
    if (likely(skb))
    cf.data[2] |= CAN_ERR_PROT_BIT1;
    }
    if (errctr & IFI_CANFD_ERROR_CTR_STUFF_ERROR_FIRST) {
    stats.rx_errors++;
    if (likely(skb))
    cf.data[2] |= CAN_ERR_PROT_STUFF;
    }
    if (errctr & IFI_CANFD_ERROR_CTR_CRC_ERROR_FIRST) {
    stats.rx_errors++;
    if (likely(skb))
    cf.data[3] = CAN_ERR_PROT_LOC_CRC_SEQ;
    }
    if (errctr & IFI_CANFD_ERROR_CTR_FORM_ERROR_FIRST) {
    stats.rx_errors++;
    if (likely(skb))
    cf.data[2] |= CAN_ERR_PROT_FORM;
    }
// Reset the error counter, ack the IRQ and re-enable the counter.
    writel(IFI_CANFD_ERROR_CTR_ER_RESET, priv.base + IFI_CANFD_ERROR_CTR);
    writel(IFI_CANFD_INTERRUPT_ERROR_COUNTER,
    priv.base + IFI_CANFD_INTERRUPT);
    writel(IFI_CANFD_ERROR_CTR_ER_ENABLE, priv.base + IFI_CANFD_ERROR_CTR);
    if (unlikely(!skb))
    return 0;
    netif_receive_skb(skb);
    return 1;
    }
    static int ifi_canfd_get_berr_counter(const struct net_device *ndev,
    struct can_berr_counter *bec)
    {
    struct ifi_canfd_priv *priv = netdev_priv(ndev);
    u32 err;
    err = readl(priv.base + IFI_CANFD_ERROR);
    bec.rxerr = (err >> IFI_CANFD_ERROR_RX_OFFSET) &
    IFI_CANFD_ERROR_RX_MASK;
    bec.txerr = (err >> IFI_CANFD_ERROR_TX_OFFSET) &
    IFI_CANFD_ERROR_TX_MASK;
    return 0;
    }
    static int ifi_canfd_handle_state_change(struct net_device *ndev,
    enum can_state new_state)
    {
    struct ifi_canfd_priv *priv = netdev_priv(ndev);
    struct can_frame *cf;
    struct sk_buff *skb;
    struct can_berr_counter bec;
    switch (new_state) {
    case CAN_STATE_ERROR_ACTIVE:
// error active state
    priv.can.can_stats.error_warning++;
    priv.can.state = CAN_STATE_ERROR_ACTIVE;
    break;
    case CAN_STATE_ERROR_WARNING:
// error warning state
    priv.can.can_stats.error_warning++;
    priv.can.state = CAN_STATE_ERROR_WARNING;
    break;
    case CAN_STATE_ERROR_PASSIVE:
// error passive state
    priv.can.can_stats.error_passive++;
    priv.can.state = CAN_STATE_ERROR_PASSIVE;
    break;
    case CAN_STATE_BUS_OFF:
// bus-off state
    priv.can.state = CAN_STATE_BUS_OFF;
    ifi_canfd_irq_enable(ndev, 0);
    priv.can.can_stats.bus_off++;
    can_bus_off(ndev);
    break;
    default:
    break;
    }
// propagate the error condition to the CAN stack
    skb = alloc_can_err_skb(ndev, &cf);
    if (unlikely(!skb))
    return 0;
    ifi_canfd_get_berr_counter(ndev, &bec);
    switch (new_state) {
    case CAN_STATE_ERROR_WARNING:
// error warning state
    cf.can_id |= CAN_ERR_CRTL | CAN_ERR_CNT;
    cf.data[1] = (bec.txerr > bec.rxerr) ?
    CAN_ERR_CRTL_TX_WARNING :
    CAN_ERR_CRTL_RX_WARNING;
    cf.data[6] = bec.txerr;
    cf.data[7] = bec.rxerr;
    break;
    case CAN_STATE_ERROR_PASSIVE:
// error passive state
    cf.can_id |= CAN_ERR_CRTL | CAN_ERR_CNT;
    cf.data[1] |= CAN_ERR_CRTL_RX_PASSIVE;
    if (bec.txerr > 127)
    cf.data[1] |= CAN_ERR_CRTL_TX_PASSIVE;
    cf.data[6] = bec.txerr;
    cf.data[7] = bec.rxerr;
    break;
    case CAN_STATE_BUS_OFF:
// bus-off state
    cf.can_id |= CAN_ERR_BUSOFF;
    break;
    default:
    break;
    }
    netif_receive_skb(skb);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn ifi_canfd_handle_state_errors(ndev: *mut net_device) -> c_int {
    static int ifi_canfd_handle_state_errors(struct net_device *ndev)
    {
    struct ifi_canfd_priv *priv = netdev_priv(ndev);
    let mut stcmd: u32 = readl(priv.base + IFI_CANFD_STCMD);
    let mut work_done: c_int = 0;
    if ((stcmd & IFI_CANFD_STCMD_ERROR_ACTIVE) &&
    (priv.can.state != CAN_STATE_ERROR_ACTIVE)) {
    netdev_dbg(ndev, "Error, entered active state\n");
    work_done += ifi_canfd_handle_state_change(ndev,
    CAN_STATE_ERROR_ACTIVE);
    }
    if ((stcmd & IFI_CANFD_STCMD_ERROR_WARNING) &&
    (priv.can.state != CAN_STATE_ERROR_WARNING)) {
    netdev_dbg(ndev, "Error, entered warning state\n");
    work_done += ifi_canfd_handle_state_change(ndev,
    CAN_STATE_ERROR_WARNING);
    }
    if ((stcmd & IFI_CANFD_STCMD_ERROR_PASSIVE) &&
    (priv.can.state != CAN_STATE_ERROR_PASSIVE)) {
    netdev_dbg(ndev, "Error, entered passive state\n");
    work_done += ifi_canfd_handle_state_change(ndev,
    CAN_STATE_ERROR_PASSIVE);
    }
    if ((stcmd & IFI_CANFD_STCMD_BUSOFF) &&
    (priv.can.state != CAN_STATE_BUS_OFF)) {
    netdev_dbg(ndev, "Error, entered bus-off state\n");
    work_done += ifi_canfd_handle_state_change(ndev,
    CAN_STATE_BUS_OFF);
    }
    return work_done;
    }
#[no_mangle]
unsafe extern "C" fn ifi_canfd_poll(napi: *mut napi_struct, quota: c_int) -> c_int {
    static int ifi_canfd_poll(struct napi_struct *napi, int quota)
    {
    struct net_device *ndev = napi.dev;
    struct ifi_canfd_priv *priv = netdev_priv(ndev);
    let mut rxstcmd: u32 = readl(priv.base + IFI_CANFD_RXSTCMD);
    let mut work_done: c_int = 0;
// Handle bus state changes
    work_done += ifi_canfd_handle_state_errors(ndev);
// Handle lost messages on RX
    if (rxstcmd & IFI_CANFD_RXSTCMD_OVERFLOW)
    work_done += ifi_canfd_handle_lost_msg(ndev);
// Handle lec errors on the bus
    if (priv.can.ctrlmode & CAN_CTRLMODE_BERR_REPORTING)
    work_done += ifi_canfd_handle_lec_err(ndev);
// Handle normal messages on RX
    if (!(rxstcmd & IFI_CANFD_RXSTCMD_EMPTY))
    work_done += ifi_canfd_do_rx_poll(ndev, quota - work_done);
    if (work_done < quota) {
    napi_complete_done(napi, work_done);
    ifi_canfd_irq_enable(ndev, 1);
    }
    return work_done;
    }
#[no_mangle]
unsafe extern "C" fn ifi_canfd_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ifi_canfd_isr(int irq, void *dev_id)
    {
    struct net_device *ndev = (struct net_device *)dev_id;
    struct ifi_canfd_priv *priv = netdev_priv(ndev);
    struct net_device_stats *stats = &ndev.stats;
    const u32 rx_irq_mask = IFI_CANFD_INTERRUPT_RXFIFO_NEMPTY |
    IFI_CANFD_INTERRUPT_RXFIFO_NEMPTY_PER |
    IFI_CANFD_INTERRUPT_ERROR_COUNTER |
    IFI_CANFD_INTERRUPT_ERROR_STATE_CHG |
    IFI_CANFD_INTERRUPT_ERROR_WARNING |
    IFI_CANFD_INTERRUPT_ERROR_BUSOFF;
    const u32 tx_irq_mask = IFI_CANFD_INTERRUPT_TXFIFO_EMPTY |
    IFI_CANFD_INTERRUPT_TXFIFO_REMOVE;
    let mut clr_irq_mask: u32 = ~((u32)IFI_CANFD_INTERRUPT_SET_IRQ);
    u32 isr;
    isr = readl(priv.base + IFI_CANFD_INTERRUPT);
// No interrupt
    if (isr == 0)
    return IRQ_NONE;
// Clear all pending interrupts but ErrWarn
    writel(clr_irq_mask, priv.base + IFI_CANFD_INTERRUPT);
// RX IRQ or bus warning, start NAPI
    if (isr & rx_irq_mask) {
    ifi_canfd_irq_enable(ndev, 0);
    napi_schedule(&priv.napi);
    }
// TX IRQ
    if (isr & IFI_CANFD_INTERRUPT_TXFIFO_REMOVE) {
    stats.tx_bytes += can_get_echo_skb(ndev, 0, core::ptr::null_mut());
    stats.tx_packets++;
    }
    if (isr & tx_irq_mask)
    netif_wake_queue(ndev);
    return IRQ_HANDLED;
    }
    static const struct can_bittiming_const ifi_canfd_bittiming_const = {
    .name		= KBUILD_MODNAME,
    .tseg1_min	= 1,	/* Time segment 1 = prop_seg + phase_seg1 */
    .tseg1_max	= 256,
    .tseg2_min	= 2,	/* Time segment 2 = phase_seg2 */
    .tseg2_max	= 256,
    .sjw_max	= 128,
    .brp_min	= 2,
    .brp_max	= 512,
    .brp_inc	= 1,
    };
#[no_mangle]
unsafe extern "C" fn ifi_canfd_set_bittiming(ndev: *mut net_device) {
    static void ifi_canfd_set_bittiming(struct net_device *ndev)
    {
    struct ifi_canfd_priv *priv = netdev_priv(ndev);
    const struct can_bittiming *bt = &priv.can.bittiming;
    const struct can_bittiming *dbt = &priv.can.fd.data_bittiming;
    u16 brp, sjw, tseg1, tseg2, tdc;
// Configure bit timing
    brp = bt.brp - 2;
    sjw = bt.sjw - 1;
    tseg1 = bt.prop_seg + bt.phase_seg1 - 1;
    tseg2 = bt.phase_seg2 - 2;
    writel((tseg2 << IFI_CANFD_TIME_TIMEB_OFF) |
    (tseg1 << IFI_CANFD_TIME_TIMEA_OFF) |
    (brp << IFI_CANFD_TIME_PRESCALE_OFF) |
    (sjw << IFI_CANFD_TIME_SJW_OFF_7_9_8_8),
    priv.base + IFI_CANFD_TIME);
// Configure data bit timing
    brp = dbt.brp - 2;
    sjw = dbt.sjw - 1;
    tseg1 = dbt.prop_seg + dbt.phase_seg1 - 1;
    tseg2 = dbt.phase_seg2 - 2;
    writel((tseg2 << IFI_CANFD_TIME_TIMEB_OFF) |
    (tseg1 << IFI_CANFD_TIME_TIMEA_OFF) |
    (brp << IFI_CANFD_TIME_PRESCALE_OFF) |
    (sjw << IFI_CANFD_TIME_SJW_OFF_7_9_8_8),
    priv.base + IFI_CANFD_FTIME);
// Configure transmitter delay
    tdc = dbt.brp * (dbt.prop_seg + dbt.phase_seg1);
    tdc &= IFI_CANFD_TDELAY_MASK;
    writel(IFI_CANFD_TDELAY_EN | tdc, priv.base + IFI_CANFD_TDELAY);
    }
    static void ifi_canfd_set_filter(struct net_device *ndev, const u32 id,
    const u32 mask, const u32 ident)
    {
    struct ifi_canfd_priv *priv = netdev_priv(ndev);
    writel(mask, priv.base + IFI_CANFD_FILTER_MASK(id));
    writel(ident, priv.base + IFI_CANFD_FILTER_IDENT(id));
    }
#[no_mangle]
unsafe extern "C" fn ifi_canfd_set_filters(ndev: *mut net_device) {
    static void ifi_canfd_set_filters(struct net_device *ndev)
    {
// Receive all CAN frames (standard ID)
    ifi_canfd_set_filter(ndev, 0,
    IFI_CANFD_FILTER_MASK_VALID |
    IFI_CANFD_FILTER_MASK_EXT,
    IFI_CANFD_FILTER_IDENT_VALID);
// Receive all CAN frames (extended ID)
    ifi_canfd_set_filter(ndev, 1,
    IFI_CANFD_FILTER_MASK_VALID |
    IFI_CANFD_FILTER_MASK_EXT,
    IFI_CANFD_FILTER_IDENT_VALID |
    IFI_CANFD_FILTER_IDENT_IDE);
// Receive all CANFD frames
    ifi_canfd_set_filter(ndev, 2,
    IFI_CANFD_FILTER_MASK_VALID |
    IFI_CANFD_FILTER_MASK_EDL |
    IFI_CANFD_FILTER_MASK_EXT,
    IFI_CANFD_FILTER_IDENT_VALID |
    IFI_CANFD_FILTER_IDENT_CANFD |
    IFI_CANFD_FILTER_IDENT_IDE);
    }
#[no_mangle]
unsafe extern "C" fn ifi_canfd_start(ndev: *mut net_device) {
    static void ifi_canfd_start(struct net_device *ndev)
    {
    struct ifi_canfd_priv *priv = netdev_priv(ndev);
    u32 stcmd;
// Reset the IP
    writel(IFI_CANFD_STCMD_HARDRESET, priv.base + IFI_CANFD_STCMD);
    writel(IFI_CANFD_STCMD_ENABLE_7_9_8_8_TIMING,
    priv.base + IFI_CANFD_STCMD);
    ifi_canfd_set_bittiming(ndev);
    ifi_canfd_set_filters(ndev);
// Reset FIFOs
    writel(IFI_CANFD_RXSTCMD_RESET, priv.base + IFI_CANFD_RXSTCMD);
    writel(0, priv.base + IFI_CANFD_RXSTCMD);
    writel(IFI_CANFD_TXSTCMD_RESET, priv.base + IFI_CANFD_TXSTCMD);
    writel(0, priv.base + IFI_CANFD_TXSTCMD);
// Repeat transmission until successful
    writel(0, priv.base + IFI_CANFD_REPEAT);
    writel(0, priv.base + IFI_CANFD_SUSPEND);
// Clear all pending interrupts
    writel((u32)(~IFI_CANFD_INTERRUPT_SET_IRQ),
    priv.base + IFI_CANFD_INTERRUPT);
    stcmd = IFI_CANFD_STCMD_ENABLE | IFI_CANFD_STCMD_NORMAL_MODE |
    IFI_CANFD_STCMD_ENABLE_7_9_8_8_TIMING;
    if (priv.can.ctrlmode & CAN_CTRLMODE_LISTENONLY)
    stcmd |= IFI_CANFD_STCMD_BUSMONITOR;
    if (priv.can.ctrlmode & CAN_CTRLMODE_LOOPBACK)
    stcmd |= IFI_CANFD_STCMD_LOOPBACK;
    if ((priv.can.ctrlmode & CAN_CTRLMODE_FD) &&
    !(priv.can.ctrlmode & CAN_CTRLMODE_FD_NON_ISO))
    stcmd |= IFI_CANFD_STCMD_ENABLE_ISO;
    if (!(priv.can.ctrlmode & CAN_CTRLMODE_FD))
    stcmd |= IFI_CANFD_STCMD_DISABLE_CANFD;
    priv.can.state = CAN_STATE_ERROR_ACTIVE;
    ifi_canfd_irq_enable(ndev, 1);
// Unlock, reset and enable the error counter.
    writel(IFI_CANFD_ERROR_CTR_UNLOCK_MAGIC,
    priv.base + IFI_CANFD_ERROR_CTR);
    writel(IFI_CANFD_ERROR_CTR_ER_RESET, priv.base + IFI_CANFD_ERROR_CTR);
    writel(IFI_CANFD_ERROR_CTR_ER_ENABLE, priv.base + IFI_CANFD_ERROR_CTR);
// Enable controller
    writel(stcmd, priv.base + IFI_CANFD_STCMD);
    }
#[no_mangle]
unsafe extern "C" fn ifi_canfd_stop(ndev: *mut net_device) {
    static void ifi_canfd_stop(struct net_device *ndev)
    {
    struct ifi_canfd_priv *priv = netdev_priv(ndev);
// Reset and disable the error counter.
    writel(IFI_CANFD_ERROR_CTR_ER_RESET, priv.base + IFI_CANFD_ERROR_CTR);
    writel(0, priv.base + IFI_CANFD_ERROR_CTR);
// Reset the IP
    writel(IFI_CANFD_STCMD_HARDRESET, priv.base + IFI_CANFD_STCMD);
// Mask all interrupts
    writel(~0, priv.base + IFI_CANFD_IRQMASK);
// Clear all pending interrupts
    writel((u32)(~IFI_CANFD_INTERRUPT_SET_IRQ),
    priv.base + IFI_CANFD_INTERRUPT);
// Set the state as STOPPED
    priv.can.state = CAN_STATE_STOPPED;
    }
#[no_mangle]
unsafe extern "C" fn ifi_canfd_set_mode(ndev: *mut net_device, mode: enum can_mode) -> c_int {
    static int ifi_canfd_set_mode(struct net_device *ndev, enum can_mode mode)
    {
    switch (mode) {
    case CAN_MODE_START:
    ifi_canfd_start(ndev);
    netif_wake_queue(ndev);
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ifi_canfd_open(ndev: *mut net_device) -> c_int {
    static int ifi_canfd_open(struct net_device *ndev)
    {
    struct ifi_canfd_priv *priv = netdev_priv(ndev);
    int ret;
    ret = open_candev(ndev);
    if (ret) {
    netdev_err(ndev, "Failed to open CAN device\n");
    return ret;
    }
// Register interrupt handler
    ret = request_irq(ndev.irq, ifi_canfd_isr, IRQF_SHARED,
    ndev.name, ndev);
    if (ret < 0) {
    netdev_err(ndev, "Failed to request interrupt\n");
    goto err_irq;
    }
    ifi_canfd_start(ndev);
    napi_enable(&priv.napi);
    netif_start_queue(ndev);
    return 0;
    err_irq:
    close_candev(ndev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ifi_canfd_close(ndev: *mut net_device) -> c_int {
    static int ifi_canfd_close(struct net_device *ndev)
    {
    struct ifi_canfd_priv *priv = netdev_priv(ndev);
    netif_stop_queue(ndev);
    napi_disable(&priv.napi);
    ifi_canfd_stop(ndev);
    free_irq(ndev.irq, ndev);
    close_candev(ndev);
    return 0;
    }
    static netdev_tx_t ifi_canfd_start_xmit(struct sk_buff *skb,
    struct net_device *ndev)
    {
    struct ifi_canfd_priv *priv = netdev_priv(ndev);
    struct canfd_frame *cf = (struct canfd_frame *)skb.data;
    u32 txst, txid, txdlc;
    int i;
    if (can_dev_dropped_skb(ndev, skb))
    return NETDEV_TX_OK;
// Check if the TX buffer is full
    txst = readl(priv.base + IFI_CANFD_TXSTCMD);
    if (txst & IFI_CANFD_TXSTCMD_FULL) {
    netif_stop_queue(ndev);
    netdev_err(ndev, "BUG! TX FIFO full when queue awake!\n");
    return NETDEV_TX_BUSY;
    }
    netif_stop_queue(ndev);
    if (cf.can_id & CAN_EFF_FLAG) {
    txid = cf.can_id & CAN_EFF_MASK;
//
// In case the Extended ID frame is transmitted, the
// standard and extended part of the ID are swapped
// in the register, so swap them back to send the
// correct ID.
//
    txid = (txid >> IFI_CANFD_TXFIFO_ID_ID_XTD_WIDTH) |
    ((txid & IFI_CANFD_TXFIFO_ID_ID_XTD_MASK) <<
    IFI_CANFD_TXFIFO_ID_ID_XTD_OFFSET);
    txid |= IFI_CANFD_TXFIFO_ID_IDE;
    } else {
    txid = cf.can_id & CAN_SFF_MASK;
    }
    txdlc = can_fd_len2dlc(cf.len);
    if ((priv.can.ctrlmode & CAN_CTRLMODE_FD) && can_is_canfd_skb(skb)) {
    txdlc |= IFI_CANFD_TXFIFO_DLC_EDL;
    if (cf.flags & CANFD_BRS)
    txdlc |= IFI_CANFD_TXFIFO_DLC_BRS;
    }
    if (cf.can_id & CAN_RTR_FLAG)
    txdlc |= IFI_CANFD_TXFIFO_DLC_RTR;
// message ram configuration
    writel(txid, priv.base + IFI_CANFD_TXFIFO_ID);
    writel(txdlc, priv.base + IFI_CANFD_TXFIFO_DLC);
    for (i = 0; i < cf.len; i += 4) {
    writel(*(u32 *)(cf.data + i),
    priv.base + IFI_CANFD_TXFIFO_DATA + i);
    }
    writel(0, priv.base + IFI_CANFD_TXFIFO_REPEATCOUNT);
    writel(0, priv.base + IFI_CANFD_TXFIFO_SUSPEND_US);
    can_put_echo_skb(skb, ndev, 0, 0);
// Start the transmission
    writel(IFI_CANFD_TXSTCMD_ADD_MSG, priv.base + IFI_CANFD_TXSTCMD);
    return NETDEV_TX_OK;
    }
    static const struct net_device_ops ifi_canfd_netdev_ops = {
    .ndo_open	= ifi_canfd_open,
    .ndo_stop	= ifi_canfd_close,
    .ndo_start_xmit	= ifi_canfd_start_xmit,
    };
    static const struct ethtool_ops ifi_canfd_ethtool_ops = {
    .get_ts_info = ethtool_op_get_ts_info,
    };
#[no_mangle]
unsafe extern "C" fn ifi_canfd_plat_probe(pdev: *mut platform_device) -> c_int {
    static int ifi_canfd_plat_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct net_device *ndev;
    struct ifi_canfd_priv *priv;
    void __iomem *addr;
    int irq, ret;
    u32 id, rev;
    addr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(addr))
    return PTR_ERR(addr);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return -EINVAL;
    id = readl(addr + IFI_CANFD_IP_ID);
    if (id != IFI_CANFD_IP_ID_VALUE) {
    dev_err(dev, "This block is not IFI CANFD, id=%08x\n", id);
    return -EINVAL;
    }
    rev = readl(addr + IFI_CANFD_VER) & IFI_CANFD_VER_REV_MASK;
    if (rev < IFI_CANFD_VER_REV_MIN_SUPPORTED) {
    dev_err(dev, "This block is too old (rev %i), minimum supported is rev %i\n",
    rev, IFI_CANFD_VER_REV_MIN_SUPPORTED);
    return -EINVAL;
    }
    ndev = alloc_candev(sizeof(*priv), 1);
    if (!ndev)
    return -ENOMEM;
    ndev.irq = irq;
    ndev.flags |= IFF_ECHO;	/* we support local echo */
    ndev.netdev_ops = &ifi_canfd_netdev_ops;
    ndev.ethtool_ops = &ifi_canfd_ethtool_ops;
    priv = netdev_priv(ndev);
    priv.ndev = ndev;
    priv.base = addr;
    netif_napi_add(ndev, &priv.napi, ifi_canfd_poll);
    priv.can.state = CAN_STATE_STOPPED;
    priv.can.clock.freq = readl(addr + IFI_CANFD_CANCLOCK);
    priv.can.bittiming_const = &ifi_canfd_bittiming_const;
    priv.can.fd.data_bittiming_const = &ifi_canfd_bittiming_const;
    priv.can.do_set_mode = ifi_canfd_set_mode;
    priv.can.do_get_berr_counter = ifi_canfd_get_berr_counter;
// IFI CANFD can do both Bosch FD and ISO FD
    priv.can.ctrlmode = CAN_CTRLMODE_FD;
// IFI CANFD can do both Bosch FD and ISO FD
    priv.can.ctrlmode_supported = CAN_CTRLMODE_LOOPBACK |
    CAN_CTRLMODE_LISTENONLY |
    CAN_CTRLMODE_FD |
    CAN_CTRLMODE_FD_NON_ISO |
    CAN_CTRLMODE_BERR_REPORTING;
    platform_set_drvdata(pdev, ndev);
    SET_NETDEV_DEV(ndev, dev);
    ret = register_candev(ndev);
    if (ret) {
    dev_err(dev, "Failed to register (ret=%d)\n", ret);
    goto err_reg;
    }
    dev_info(dev, "Driver registered: regs=%p, irq=%d, clock=%d\n",
    priv.base, ndev.irq, priv.can.clock.freq);
    return 0;
    err_reg:
    free_candev(ndev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ifi_canfd_plat_remove(pdev: *mut platform_device) {
    static void ifi_canfd_plat_remove(struct platform_device *pdev)
    {
    struct net_device *ndev = platform_get_drvdata(pdev);
    unregister_candev(ndev);
    platform_set_drvdata(pdev, core::ptr::null_mut());
    free_candev(ndev);
    }
    static const struct of_device_id ifi_canfd_of_table[] = {
    { .compatible = "ifi,canfd-1.0", .data = core::ptr::null_mut() },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, ifi_canfd_of_table);
    static struct platform_driver ifi_canfd_plat_driver = {
    .driver = {
    .name		= KBUILD_MODNAME,
    .of_match_table	= ifi_canfd_of_table,
    },
    .probe	= ifi_canfd_plat_probe,
    .remove = ifi_canfd_plat_remove,
    };
    module_platform_driver(ifi_canfd_plat_driver);
    MODULE_AUTHOR("Marek Vasut <marex@denx.de>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("CAN bus driver for IFI CANFD controller");

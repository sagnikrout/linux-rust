//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/hisilicon/hip04_eth.c
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
// Copyright (c) 2014 Linaro Ltd.
// Copyright (c) 2014 Hisilicon Limited.
//

pub const SC_PPE_RESET_DREQ: c_uint = 0x026C;
pub const PPE_CFG_RX_ADDR: c_uint = 0x100;
pub const PPE_CFG_POOL_GRP: c_uint = 0x300;
pub const PPE_CFG_RX_BUF_SIZE: c_uint = 0x400;
pub const PPE_CFG_RX_FIFO_SIZE: c_uint = 0x500;
pub const PPE_CURR_BUF_CNT: c_uint = 0xa200;
pub const GE_DUPLEX_TYPE: c_uint = 0x08;
pub const GE_MAX_FRM_SIZE_REG: c_uint = 0x3c;
pub const GE_PORT_MODE: c_uint = 0x40;
pub const GE_PORT_EN: c_uint = 0x44;
pub const GE_SHORT_RUNTS_THR_REG: c_uint = 0x50;
pub const GE_TX_LOCAL_PAGE_REG: c_uint = 0x5c;
pub const GE_TRANSMIT_CONTROL_REG: c_uint = 0x60;
pub const GE_CF_CRC_STRIP_REG: c_uint = 0x1b0;
pub const GE_MODE_CHANGE_REG: c_uint = 0x1b4;
pub const GE_RECV_CONTROL_REG: c_uint = 0x1e0;
pub const GE_STATION_MAC_ADDRESS: c_uint = 0x210;
pub const PPE_CFG_BUS_CTRL_REG: c_uint = 0x424;
pub const PPE_CFG_RX_CTRL_REG: c_uint = 0x428;

pub const PPE_CFG_CPU_ADD_ADDR: c_uint = 0x6D0;
pub const PPE_CFG_MAX_FRAME_LEN_REG: c_uint = 0x500;
pub const PPE_CFG_RX_PKT_MODE_REG: c_uint = 0x504;
pub const PPE_CFG_QOS_VMID_GEN: c_uint = 0x520;
pub const PPE_CFG_RX_PKT_INT: c_uint = 0x740;
pub const PPE_INTEN: c_uint = 0x700;
pub const PPE_INTSTS: c_uint = 0x708;
pub const PPE_RINT: c_uint = 0x704;
pub const PPE_CFG_STS_MODE: c_uint = 0x880;

pub const PPE_CFG_CPU_ADD_ADDR: c_uint = 0x580;
pub const PPE_CFG_MAX_FRAME_LEN_REG: c_uint = 0x408;
pub const PPE_CFG_RX_PKT_MODE_REG: c_uint = 0x438;
pub const PPE_CFG_QOS_VMID_GEN: c_uint = 0x500;
pub const PPE_CFG_RX_PKT_INT: c_uint = 0x538;
pub const PPE_INTEN: c_uint = 0x600;
pub const PPE_INTSTS: c_uint = 0x608;
pub const PPE_RINT: c_uint = 0x604;
pub const PPE_CFG_STS_MODE: c_uint = 0x700;

pub const PPE_HIS_RX_PKT_CNT: c_uint = 0x804;
pub const RESET_DREQ_ALL: c_uint = 0xffffffff;
// REG_INTERRUPT

// TX descriptor config

pub const TX_POOL_SHIFT: c_int = 16;

// RX error

pub const SGMII_SPEED_1000: c_uint = 0x08;
pub const SGMII_SPEED_100: c_uint = 0x07;
pub const SGMII_SPEED_10: c_uint = 0x06;
pub const MII_SPEED_100: c_uint = 0x01;
pub const MII_SPEED_10: c_uint = 0x00;

pub const GE_DUPLEX_HALF: c_uint = 0x00;

pub const GE_RX_TIMEOUT: c_uint = 0x04;

pub const PPE_CFG_QOS_VMID_GRP_SHIFT: c_int = 4;
pub const PPE_CFG_RX_CTRL_ALIGN_SHIFT: c_int = 7;

// buf unit size is cache_line_size, which is 64, so the shift is 6
pub const PPE_BUF_SIZE_SHIFT: c_int = 6;

pub const SOC_CACHE_LINE_MASK: c_uint = 0x3F;

pub const PPE_CFG_QOS_VMID_GRP_SHIFT: c_int = 8;
pub const PPE_CFG_RX_CTRL_ALIGN_SHIFT: c_int = 11;

// buf unit size is 1, so the shift is 6
pub const PPE_BUF_SIZE_SHIFT: c_int = 0;
pub const PPE_TX_BUF_HOLD: c_int = 0;

pub const PPE_CFG_RX_DEPTH_SHIFT: c_int = 16;
pub const PPE_CFG_RX_START_SHIFT: c_int = 0;

pub const RX_DESC_NUM: c_int = 128;
pub const TX_DESC_NUM: c_int = 256;

pub const GMAC_PPE_RX_PKT_MAX_LEN: c_int = 379;
pub const GMAC_MAX_PKT_LEN: c_int = 1516;
pub const GMAC_MIN_PKT_LEN: c_int = 31;
pub const RX_BUF_SIZE: c_int = 1600;
pub const RESET_TIMEOUT: c_int = 1000;

pub const HIP04_MAX_TX_COALESCE_USECS: c_int = 200;
pub const HIP04_MIN_TX_COALESCE_USECS: c_int = 100;
pub const HIP04_MAX_TX_COALESCE_FRAMES: c_int = 200;
pub const HIP04_MIN_TX_COALESCE_FRAMES: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_desc {
    pub reserved1: [u32; 2],
    pub send_addr: u32,
    pub send_size: u16,
    pub data_offset: u16,
    pub reserved2: [u32; 7],
    pub cfg: u32,
    pub wb_addr: u32,
    pub reserved3: [u32; 3],
    pub send_addr: u32,
    pub send_size: u32,
    pub next_addr: u32,
    pub cfg: u32,
    pub wb_addr: u32,

    pub __aligned(64): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_desc {
    pub reserved1: [u32; 3],
    pub pkt_len: u16,
    pub reserved_16: u16,
    pub reserved2: [u32; 6],
    pub pkt_err: u32,
    pub reserved3: [u32; 5],
    pub reserved_16: u16,
    pub pkt_len: u16,
    pub reserve1: [u32; 3],
    pub pkt_err: u32,
    pub reserve2: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hip04_priv {
    pub base: *mut void __iomem,

    pub sysctrl_base: *mut void __iomem,

    pub phy_mode: phy_interface_t,
    pub chan: c_int,
    pub port: c_uint,
    pub group: c_uint,
    pub speed: c_uint,
    pub duplex: c_uint,
    pub reg_inten: c_uint,
    pub napi: napi_struct,
    pub dev: *mut device,
    pub ndev: *mut net_device,
    pub tx_desc: *mut tx_desc,
    pub tx_desc_dma: dma_addr_t,
    pub tx_skb: [*mut sk_buff; TX_DESC_NUM],
    pub tx_phys: [dma_addr_t; TX_DESC_NUM],
    pub tx_head: c_uint,
    pub tx_coalesce_frames: c_int,
    pub tx_coalesce_usecs: c_int,
    pub tx_coalesce_timer: hrtimer,
    pub rx_buf: [*mut c_uchar; RX_DESC_NUM],
    pub rx_phys: [dma_addr_t; RX_DESC_NUM],
    pub rx_head: c_uint,
    pub rx_buf_size: c_uint,
    pub rx_cnt_remaining: c_uint,
    pub phy_node: *mut device_node,
    pub phy: *mut phy_device,
    pub map: *mut regmap,
    pub tx_timeout_task: work_struct,
// written only by tx cleanup
    pub ____cacheline_aligned_in_smp: unsigned int tx_tail,
}

#[no_mangle]
pub unsafe extern "C" fn tx_count(head: c_uint, tail: c_uint) -> c_uint {
    static inline unsigned int tx_count(unsigned int head, unsigned int tail)
    {
    return (head - tail) % TX_DESC_NUM;
    }
#[no_mangle]
unsafe extern "C" fn hip04_config_port(ndev: *mut net_device, speed: u32, duplex: u32) {
    static void hip04_config_port(struct net_device *ndev, u32 speed, u32 duplex)
    {
    struct hip04_priv *priv = netdev_priv(ndev);
    u32 val;
    priv.speed = speed;
    priv.duplex = duplex;
    switch (priv.phy_mode) {
    case PHY_INTERFACE_MODE_SGMII:
    if (speed == SPEED_1000)
    val = SGMII_SPEED_1000;
#[no_mangle]
pub unsafe extern "C" fn if(SPEED_100: speed ==) -> else {
    else if (speed == SPEED_100)
    val = SGMII_SPEED_100;
    else
    val = SGMII_SPEED_10;
    break;
    case PHY_INTERFACE_MODE_MII:
    if (speed == SPEED_100)
    val = MII_SPEED_100;
    else
    val = MII_SPEED_10;
    break;
    default:
    netdev_warn(ndev, "not supported mode\n");
    val = MII_SPEED_10;
    break;
    }
    writel_relaxed(val, priv.base + GE_PORT_MODE);
    val = duplex ? GE_DUPLEX_FULL : GE_DUPLEX_HALF;
    writel_relaxed(val, priv.base + GE_DUPLEX_TYPE);
    val = GE_MODE_CHANGE_EN;
    writel_relaxed(val, priv.base + GE_MODE_CHANGE_REG);
    }
#[no_mangle]
unsafe extern "C" fn hip04_reset_dreq(priv: *mut hip04_priv) {
    static void hip04_reset_dreq(struct hip04_priv *priv)
    {

    writel_relaxed(RESET_DREQ_ALL, priv.sysctrl_base + SC_PPE_RESET_DREQ);

    }
#[no_mangle]
unsafe extern "C" fn hip04_reset_ppe(priv: *mut hip04_priv) {
    static void hip04_reset_ppe(struct hip04_priv *priv)
    {
    u32 val, tmp, timeout = 0;
    do {
    regmap_read(priv.map, priv.port * 4 + PPE_CURR_BUF_CNT, &val);
    regmap_read(priv.map, priv.port * 4 + PPE_CFG_RX_ADDR, &tmp);
    if (timeout++ > RESET_TIMEOUT)
    break;
    } while (val & 0xfff);
    }
#[no_mangle]
unsafe extern "C" fn hip04_config_fifo(priv: *mut hip04_priv) {
    static void hip04_config_fifo(struct hip04_priv *priv)
    {
    u32 val;
    val = readl_relaxed(priv.base + PPE_CFG_STS_MODE);
    val |= PPE_CFG_STS_RX_PKT_CNT_RC;
    writel_relaxed(val, priv.base + PPE_CFG_STS_MODE);
    val = BIT(priv.group);
    regmap_write(priv.map, priv.port * 4 + PPE_CFG_POOL_GRP, val);
    val = priv.group << PPE_CFG_QOS_VMID_GRP_SHIFT;
    val |= PPE_CFG_QOS_VMID_MODE;
    writel_relaxed(val, priv.base + PPE_CFG_QOS_VMID_GEN);
    val = RX_BUF_SIZE >> PPE_BUF_SIZE_SHIFT;
    regmap_write(priv.map, priv.port * 4 + PPE_CFG_RX_BUF_SIZE, val);
    val = RX_DESC_NUM << PPE_CFG_RX_DEPTH_SHIFT;
    val |= PPE_CFG_RX_FIFO_FSFU;
    val |= priv.chan << PPE_CFG_RX_START_SHIFT;
    regmap_write(priv.map, priv.port * 4 + PPE_CFG_RX_FIFO_SIZE, val);
    val = NET_IP_ALIGN << PPE_CFG_RX_CTRL_ALIGN_SHIFT;
    writel_relaxed(val, priv.base + PPE_CFG_RX_CTRL_REG);
    val = PPE_CFG_RX_PKT_ALIGN;
    writel_relaxed(val, priv.base + PPE_CFG_RX_PKT_MODE_REG);
    val = PPE_CFG_BUS_LOCAL_REL | PPE_CFG_BUS_BIG_ENDIEN;
    writel_relaxed(val, priv.base + PPE_CFG_BUS_CTRL_REG);
    val = GMAC_PPE_RX_PKT_MAX_LEN;
    writel_relaxed(val, priv.base + PPE_CFG_MAX_FRAME_LEN_REG);
    val = GMAC_MAX_PKT_LEN;
    writel_relaxed(val, priv.base + GE_MAX_FRM_SIZE_REG);
    val = GMAC_MIN_PKT_LEN;
    writel_relaxed(val, priv.base + GE_SHORT_RUNTS_THR_REG);
    val = readl_relaxed(priv.base + GE_TRANSMIT_CONTROL_REG);
    val |= GE_TX_AUTO_NEG | GE_TX_ADD_CRC | GE_TX_SHORT_PAD_THROUGH;
    writel_relaxed(val, priv.base + GE_TRANSMIT_CONTROL_REG);
    val = GE_RX_STRIP_CRC;
    writel_relaxed(val, priv.base + GE_CF_CRC_STRIP_REG);
    val = readl_relaxed(priv.base + GE_RECV_CONTROL_REG);
    val |= GE_RX_STRIP_PAD | GE_RX_PAD_EN;
    writel_relaxed(val, priv.base + GE_RECV_CONTROL_REG);

    val = GE_AUTO_NEG_CTL;
    writel_relaxed(val, priv.base + GE_TX_LOCAL_PAGE_REG);

    }
#[no_mangle]
unsafe extern "C" fn hip04_mac_enable(ndev: *mut net_device) {
    static void hip04_mac_enable(struct net_device *ndev)
    {
    struct hip04_priv *priv = netdev_priv(ndev);
    u32 val;
// enable tx & rx
    val = readl_relaxed(priv.base + GE_PORT_EN);
    val |= GE_RX_PORT_EN | GE_TX_PORT_EN;
    writel_relaxed(val, priv.base + GE_PORT_EN);
// clear rx int
    val = RCV_INT;
    writel_relaxed(val, priv.base + PPE_RINT);
// config recv int
    val = GE_RX_INT_THRESHOLD | GE_RX_TIMEOUT;
    writel_relaxed(val, priv.base + PPE_CFG_RX_PKT_INT);
// enable interrupt
    priv.reg_inten = DEF_INT_MASK;
    writel_relaxed(priv.reg_inten, priv.base + PPE_INTEN);
    }
#[no_mangle]
unsafe extern "C" fn hip04_mac_disable(ndev: *mut net_device) {
    static void hip04_mac_disable(struct net_device *ndev)
    {
    struct hip04_priv *priv = netdev_priv(ndev);
    u32 val;
// disable int
    priv.reg_inten &= ~(DEF_INT_MASK);
    writel_relaxed(priv.reg_inten, priv.base + PPE_INTEN);
// disable tx & rx
    val = readl_relaxed(priv.base + GE_PORT_EN);
    val &= ~(GE_RX_PORT_EN | GE_TX_PORT_EN);
    writel_relaxed(val, priv.base + GE_PORT_EN);
    }
#[no_mangle]
unsafe extern "C" fn hip04_set_xmit_desc(priv: *mut hip04_priv, phys: dma_addr_t) {
    static void hip04_set_xmit_desc(struct hip04_priv *priv, dma_addr_t phys)
    {
    u32 val;
    val = phys >> PPE_BUF_SIZE_SHIFT | PPE_TX_BUF_HOLD;
    writel(val, priv.base + PPE_CFG_CPU_ADD_ADDR);
    }
#[no_mangle]
unsafe extern "C" fn hip04_set_recv_desc(priv: *mut hip04_priv, phys: dma_addr_t) {
    static void hip04_set_recv_desc(struct hip04_priv *priv, dma_addr_t phys)
    {
    u32 val;
    val = phys >> PPE_BUF_SIZE_SHIFT;
    regmap_write(priv.map, priv.port * 4 + PPE_CFG_RX_ADDR, val);
    }
#[no_mangle]
unsafe extern "C" fn hip04_recv_cnt(priv: *mut hip04_priv) -> u32 {
    static u32 hip04_recv_cnt(struct hip04_priv *priv)
    {
    return readl(priv.base + PPE_HIS_RX_PKT_CNT);
    }
#[no_mangle]
unsafe extern "C" fn hip04_update_mac_address(ndev: *mut net_device) {
    static void hip04_update_mac_address(struct net_device *ndev)
    {
    struct hip04_priv *priv = netdev_priv(ndev);
    writel_relaxed(((ndev.dev_addr[0] << 8) | (ndev.dev_addr[1])),
    priv.base + GE_STATION_MAC_ADDRESS);
    writel_relaxed(((ndev.dev_addr[2] << 24) | (ndev.dev_addr[3] << 16) |
    (ndev.dev_addr[4] << 8) | (ndev.dev_addr[5])),
    priv.base + GE_STATION_MAC_ADDRESS + 4);
    }
#[no_mangle]
unsafe extern "C" fn hip04_set_mac_address(ndev: *mut net_device, addr: *mut c_void) -> c_int {
    static int hip04_set_mac_address(struct net_device *ndev, void *addr)
    {
    eth_mac_addr(ndev, addr);
    hip04_update_mac_address(ndev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hip04_tx_reclaim(ndev: *mut net_device, force: bool) -> c_int {
    static int hip04_tx_reclaim(struct net_device *ndev, bool force)
    {
    struct hip04_priv *priv = netdev_priv(ndev);
    let mut tx_tail: unsigned = priv.tx_tail;
    struct tx_desc *desc;
    let mut bytes_compl: c_uint = 0, pkts_compl = 0;
    unsigned int count;
    smp_rmb();
    count = tx_count(READ_ONCE(priv.tx_head), tx_tail);
    if (count == 0)
    goto out;
    while (count) {
    desc = &priv.tx_desc[tx_tail];
    if (desc.send_addr != 0) {
    if (force)
    desc.send_addr = 0;
    else
    break;
    }
    if (priv.tx_phys[tx_tail]) {
    dma_unmap_single(priv.dev, priv.tx_phys[tx_tail],
    priv.tx_skb[tx_tail].len,
    DMA_TO_DEVICE);
    priv.tx_phys[tx_tail] = 0;
    }
    pkts_compl++;
    bytes_compl += priv.tx_skb[tx_tail].len;
    dev_kfree_skb(priv.tx_skb[tx_tail]);
    priv.tx_skb[tx_tail] = core::ptr::null_mut();
    tx_tail = TX_NEXT(tx_tail);
    count--;
    }
    priv.tx_tail = tx_tail;
    smp_wmb(); /* Ensure tx_tail visible to xmit */
    out:
    if (pkts_compl || bytes_compl)
    netdev_completed_queue(ndev, pkts_compl, bytes_compl);
    if (unlikely(netif_queue_stopped(ndev)) && (count < (TX_DESC_NUM - 1)))
    netif_wake_queue(ndev);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn hip04_start_tx_timer(priv: *mut hip04_priv) {
    static void hip04_start_tx_timer(struct hip04_priv *priv)
    {
    let mut ns: c_ulong = priv.tx_coalesce_usecs * NSEC_PER_USEC / 2;
// allow timer to fire after half the time at the earliest
    hrtimer_start_range_ns(&priv.tx_coalesce_timer, ns_to_ktime(ns),
    ns, HRTIMER_MODE_REL);
    }
    static netdev_tx_t
    hip04_mac_start_xmit(struct sk_buff *skb, struct net_device *ndev)
    {
    struct hip04_priv *priv = netdev_priv(ndev);
    struct net_device_stats *stats = &ndev.stats;
    let mut tx_head: c_uint = priv.tx_head, count;
    struct tx_desc *desc = &priv.tx_desc[tx_head];
    dma_addr_t phys;
    smp_rmb();
    count = tx_count(tx_head, READ_ONCE(priv.tx_tail));
    if (count == (TX_DESC_NUM - 1)) {
    netif_stop_queue(ndev);
    return NETDEV_TX_BUSY;
    }
    phys = dma_map_single(priv.dev, skb.data, skb.len, DMA_TO_DEVICE);
    if (dma_mapping_error(priv.dev, phys)) {
    dev_kfree_skb(skb);
    return NETDEV_TX_OK;
    }
    priv.tx_skb[tx_head] = skb;
    priv.tx_phys[tx_head] = phys;
    desc.send_size = ( u32)cpu_to_be32(skb.len);

    desc.cfg = ( u32)cpu_to_be32(TX_CLEAR_WB | TX_FINISH_CACHE_INV
    | TX_RELEASE_TO_PPE | priv.port << TX_POOL_SHIFT);
    desc.data_offset = ( u32)cpu_to_be32(phys & SOC_CACHE_LINE_MASK);
    desc.send_addr =  ( u32)cpu_to_be32(phys & ~SOC_CACHE_LINE_MASK);

    desc.cfg = ( u32)cpu_to_be32(TX_CLEAR_WB | TX_FINISH_CACHE_INV);
    desc.send_addr = ( u32)cpu_to_be32(phys);

    phys = priv.tx_desc_dma + tx_head * sizeof(struct tx_desc);
    desc.wb_addr = ( u32)cpu_to_be32(phys +
    offsetof(struct tx_desc, send_addr));
    skb_tx_timestamp(skb);
    hip04_set_xmit_desc(priv, phys);
    count++;
    netdev_sent_queue(ndev, skb.len);
    priv.tx_head = TX_NEXT(tx_head);
    stats.tx_bytes += skb.len;
    stats.tx_packets++;
// Ensure tx_head update visible to tx reclaim
    smp_wmb();
// queue is getting full, better start cleaning up now
    if (count >= priv.tx_coalesce_frames) {
    if (napi_schedule_prep(&priv.napi)) {
// disable rx interrupt and timer
    priv.reg_inten &= ~(RCV_INT);
    writel_relaxed(DEF_INT_MASK & ~RCV_INT,
    priv.base + PPE_INTEN);
    hrtimer_cancel(&priv.tx_coalesce_timer);
    __napi_schedule(&priv.napi);
    }
    } else if (!hrtimer_is_queued(&priv.tx_coalesce_timer)) {
// cleanup not pending yet, start a new timer
    hip04_start_tx_timer(priv);
    }
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn hip04_rx_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int hip04_rx_poll(struct napi_struct *napi, int budget)
    {
    struct hip04_priv *priv = container_of(napi, struct hip04_priv, napi);
    struct net_device *ndev = priv.ndev;
    struct net_device_stats *stats = &ndev.stats;
    struct rx_desc *desc;
    struct sk_buff *skb;
    unsigned char *buf;
    let mut last: bool = false;
    dma_addr_t phys;
    let mut rx: c_int = 0;
    int tx_remaining;
    u16 len;
    u32 err;
// clean up tx descriptors
    tx_remaining = hip04_tx_reclaim(ndev, false);
    priv.rx_cnt_remaining += hip04_recv_cnt(priv);
    while (priv.rx_cnt_remaining && !last) {
    buf = priv.rx_buf[priv.rx_head];
    skb = build_skb(buf, priv.rx_buf_size);
    if (unlikely(!skb)) {
    net_dbg_ratelimited("build_skb failed\n");
// Retain the slot; return budget so NAPI retries this
// buffer. Refill would overwrite rx_buf[]/rx_phys[]
// and leak them.
//
    return budget;
    }
    dma_unmap_single(priv.dev, priv.rx_phys[priv.rx_head],
    RX_BUF_SIZE, DMA_FROM_DEVICE);
    priv.rx_phys[priv.rx_head] = 0;
    desc = (struct rx_desc *)skb.data;
    len = be16_to_cpu(( __be16)desc.pkt_len);
    err = be32_to_cpu(( __be32)desc.pkt_err);
    if (0 == len) {
    dev_kfree_skb_any(skb);
    last = true;
    } else if ((err & RX_PKT_ERR) || (len >= GMAC_MAX_PKT_LEN)) {
    dev_kfree_skb_any(skb);
    stats.rx_dropped++;
    stats.rx_errors++;
    } else {
    skb_reserve(skb, NET_SKB_PAD + NET_IP_ALIGN);
    skb_put(skb, len);
    skb.protocol = eth_type_trans(skb, ndev);
    napi_gro_receive(&priv.napi, skb);
    stats.rx_packets++;
    stats.rx_bytes += len;
    rx++;
    }
    buf = netdev_alloc_frag(priv.rx_buf_size);
    if (!buf)
    goto done;
    phys = dma_map_single(priv.dev, buf,
    RX_BUF_SIZE, DMA_FROM_DEVICE);
    if (dma_mapping_error(priv.dev, phys)) {
    skb_free_frag(buf);
    goto done;
    }
    priv.rx_buf[priv.rx_head] = buf;
    priv.rx_phys[priv.rx_head] = phys;
    hip04_set_recv_desc(priv, phys);
    priv.rx_head = RX_NEXT(priv.rx_head);
    if (rx >= budget) {
    --priv.rx_cnt_remaining;
    goto done;
    }
    if (--priv.rx_cnt_remaining == 0)
    priv.rx_cnt_remaining += hip04_recv_cnt(priv);
    }
    if (!(priv.reg_inten & RCV_INT)) {
// enable rx interrupt
    priv.reg_inten |= RCV_INT;
    writel_relaxed(priv.reg_inten, priv.base + PPE_INTEN);
    }
    napi_complete_done(napi, rx);
    done:
// start a new timer if necessary
    if (rx < budget && tx_remaining)
    hip04_start_tx_timer(priv);
    return rx;
    }
#[no_mangle]
unsafe extern "C" fn hip04_mac_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t hip04_mac_interrupt(int irq, void *dev_id)
    {
    struct net_device *ndev = (struct net_device *)dev_id;
    struct hip04_priv *priv = netdev_priv(ndev);
    struct net_device_stats *stats = &ndev.stats;
    let mut ists: u32 = readl_relaxed(priv.base + PPE_INTSTS);
    if (!ists)
    return IRQ_NONE;
    writel_relaxed(DEF_INT_MASK, priv.base + PPE_RINT);
    if (unlikely(ists & DEF_INT_ERR)) {
    if (ists & (RCV_NOBUF | RCV_DROP)) {
    stats.rx_errors++;
    stats.rx_dropped++;
    netdev_err(ndev, "rx drop\n");
    }
    if (ists & TX_DROP) {
    stats.tx_dropped++;
    netdev_err(ndev, "tx drop\n");
    }
    }
    if (ists & RCV_INT && napi_schedule_prep(&priv.napi)) {
// disable rx interrupt
    priv.reg_inten &= ~(RCV_INT);
    writel_relaxed(DEF_INT_MASK & ~RCV_INT, priv.base + PPE_INTEN);
    hrtimer_cancel(&priv.tx_coalesce_timer);
    __napi_schedule(&priv.napi);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tx_done(hrtimer: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart tx_done(struct hrtimer *hrtimer)
    {
    struct hip04_priv *priv;
    priv = container_of(hrtimer, struct hip04_priv, tx_coalesce_timer);
    if (napi_schedule_prep(&priv.napi)) {
// disable rx interrupt
    priv.reg_inten &= ~(RCV_INT);
    writel_relaxed(DEF_INT_MASK & ~RCV_INT, priv.base + PPE_INTEN);
    __napi_schedule(&priv.napi);
    }
    return HRTIMER_NORESTART;
    }
#[no_mangle]
unsafe extern "C" fn hip04_adjust_link(ndev: *mut net_device) {
    static void hip04_adjust_link(struct net_device *ndev)
    {
    struct hip04_priv *priv = netdev_priv(ndev);
    struct phy_device *phy = priv.phy;
    if ((priv.speed != phy.speed) || (priv.duplex != phy.duplex)) {
    hip04_config_port(ndev, phy.speed, phy.duplex);
    phy_print_status(phy);
    }
    }
#[no_mangle]
unsafe extern "C" fn hip04_mac_open(ndev: *mut net_device) -> c_int {
    static int hip04_mac_open(struct net_device *ndev)
    {
    struct hip04_priv *priv = netdev_priv(ndev);
    int i;
    priv.rx_head = 0;
    priv.rx_cnt_remaining = 0;
    priv.tx_head = 0;
    priv.tx_tail = 0;
    hip04_reset_ppe(priv);
    for (i = 0; i < RX_DESC_NUM; i++) {
    dma_addr_t phys;
    phys = dma_map_single(priv.dev, priv.rx_buf[i],
    RX_BUF_SIZE, DMA_FROM_DEVICE);
    if (dma_mapping_error(priv.dev, phys))
    return -EIO;
    priv.rx_phys[i] = phys;
    hip04_set_recv_desc(priv, phys);
    }
    if (priv.phy)
    phy_start(priv.phy);
    netdev_reset_queue(ndev);
    netif_start_queue(ndev);
    hip04_mac_enable(ndev);
    napi_enable(&priv.napi);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hip04_mac_stop(ndev: *mut net_device) -> c_int {
    static int hip04_mac_stop(struct net_device *ndev)
    {
    struct hip04_priv *priv = netdev_priv(ndev);
    int i;
    napi_disable(&priv.napi);
    netif_stop_queue(ndev);
    hip04_mac_disable(ndev);
    hip04_tx_reclaim(ndev, true);
    hip04_reset_ppe(priv);
    if (priv.phy)
    phy_stop(priv.phy);
    for (i = 0; i < RX_DESC_NUM; i++) {
    if (priv.rx_phys[i]) {
    dma_unmap_single(priv.dev, priv.rx_phys[i],
    RX_BUF_SIZE, DMA_FROM_DEVICE);
    priv.rx_phys[i] = 0;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hip04_timeout(ndev: *mut net_device, txqueue: c_uint) {
    static void hip04_timeout(struct net_device *ndev, unsigned int txqueue)
    {
    struct hip04_priv *priv = netdev_priv(ndev);
    schedule_work(&priv.tx_timeout_task);
    }
#[no_mangle]
unsafe extern "C" fn hip04_tx_timeout_task(work: *mut work_struct) {
    static void hip04_tx_timeout_task(struct work_struct *work)
    {
    struct hip04_priv *priv;
    priv = container_of(work, struct hip04_priv, tx_timeout_task);
    hip04_mac_stop(priv.ndev);
    hip04_mac_open(priv.ndev);
    }
    static int hip04_get_coalesce(struct net_device *netdev,
    struct ethtool_coalesce *ec,
    struct kernel_ethtool_coalesce *kernel_coal,
    struct netlink_ext_ack *extack)
    {
    struct hip04_priv *priv = netdev_priv(netdev);
    ec.tx_coalesce_usecs = priv.tx_coalesce_usecs;
    ec.tx_max_coalesced_frames = priv.tx_coalesce_frames;
    return 0;
    }
    static int hip04_set_coalesce(struct net_device *netdev,
    struct ethtool_coalesce *ec,
    struct kernel_ethtool_coalesce *kernel_coal,
    struct netlink_ext_ack *extack)
    {
    struct hip04_priv *priv = netdev_priv(netdev);
    if ((ec.tx_coalesce_usecs > HIP04_MAX_TX_COALESCE_USECS ||
    ec.tx_coalesce_usecs < HIP04_MIN_TX_COALESCE_USECS) ||
    (ec.tx_max_coalesced_frames > HIP04_MAX_TX_COALESCE_FRAMES ||
    ec.tx_max_coalesced_frames < HIP04_MIN_TX_COALESCE_FRAMES))
    return -EINVAL;
    priv.tx_coalesce_usecs = ec.tx_coalesce_usecs;
    priv.tx_coalesce_frames = ec.tx_max_coalesced_frames;
    return 0;
    }
    static void hip04_get_drvinfo(struct net_device *netdev,
    struct ethtool_drvinfo *drvinfo)
    {
    strscpy(drvinfo.driver, DRV_NAME, sizeof(drvinfo.driver));
    strscpy(drvinfo.version, DRV_VERSION, sizeof(drvinfo.version));
    }
    static const struct ethtool_ops hip04_ethtool_ops = {
    .supported_coalesce_params = ETHTOOL_COALESCE_TX_USECS |
    ETHTOOL_COALESCE_TX_MAX_FRAMES,
    .get_coalesce		= hip04_get_coalesce,
    .set_coalesce		= hip04_set_coalesce,
    .get_drvinfo		= hip04_get_drvinfo,
    };
    static const struct net_device_ops hip04_netdev_ops = {
    .ndo_open		= hip04_mac_open,
    .ndo_stop		= hip04_mac_stop,
    .ndo_start_xmit		= hip04_mac_start_xmit,
    .ndo_set_mac_address	= hip04_set_mac_address,
    .ndo_tx_timeout         = hip04_timeout,
    .ndo_validate_addr	= eth_validate_addr,
    };
#[no_mangle]
unsafe extern "C" fn hip04_alloc_ring(ndev: *mut net_device, d: *mut device) -> c_int {
    static int hip04_alloc_ring(struct net_device *ndev, struct device *d)
    {
    struct hip04_priv *priv = netdev_priv(ndev);
    int i;
    priv.tx_desc = dma_alloc_coherent(d,
    TX_DESC_NUM * sizeof(struct tx_desc),
    &priv.tx_desc_dma, GFP_KERNEL);
    if (!priv.tx_desc)
    return -ENOMEM;
    priv.rx_buf_size = RX_BUF_SIZE +
    SKB_DATA_ALIGN(sizeof(struct skb_shared_info));
    for (i = 0; i < RX_DESC_NUM; i++) {
    priv.rx_buf[i] = netdev_alloc_frag(priv.rx_buf_size);
    if (!priv.rx_buf[i])
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hip04_free_ring(ndev: *mut net_device, d: *mut device) {
    static void hip04_free_ring(struct net_device *ndev, struct device *d)
    {
    struct hip04_priv *priv = netdev_priv(ndev);
    int i;
    for (i = 0; i < RX_DESC_NUM; i++)
    if (priv.rx_buf[i])
    skb_free_frag(priv.rx_buf[i]);
    for (i = 0; i < TX_DESC_NUM; i++)
    if (priv.tx_skb[i])
    dev_kfree_skb_any(priv.tx_skb[i]);
    dma_free_coherent(d, TX_DESC_NUM * sizeof(struct tx_desc),
    priv.tx_desc, priv.tx_desc_dma);
    }
#[no_mangle]
unsafe extern "C" fn hip04_mac_probe(pdev: *mut platform_device) -> c_int {
    static int hip04_mac_probe(struct platform_device *pdev)
    {
    struct device *d = &pdev.dev;
    struct device_node *node = d.of_node;
    struct of_phandle_args arg;
    struct net_device *ndev;
    struct hip04_priv *priv;
    int irq;
    int ret;
    ndev = alloc_etherdev(sizeof(struct hip04_priv));
    if (!ndev)
    return -ENOMEM;
    priv = netdev_priv(ndev);
    priv.dev = d;
    priv.ndev = ndev;
    platform_set_drvdata(pdev, ndev);
    SET_NETDEV_DEV(ndev, &pdev.dev);
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base)) {
    ret = PTR_ERR(priv.base);
    goto init_fail;
    }

    priv.sysctrl_base = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(priv.sysctrl_base)) {
    ret = PTR_ERR(priv.sysctrl_base);
    goto init_fail;
    }

    ret = of_parse_phandle_with_fixed_args(node, "port-handle", 3, 0, &arg);
    if (ret < 0) {
    dev_warn(d, "no port-handle\n");
    goto init_fail;
    }
    priv.port = arg.args[0];
    priv.chan = arg.args[1] * RX_DESC_NUM;
    priv.group = arg.args[2];
// BQL will try to keep the TX queue as short as possible, but it can't
// be faster than tx_coalesce_usecs, so we need a fast timeout here,
// but also long enough to gather up enough frames to ensure we don't
// get more interrupts than necessary.
// 200us is enough for 16 frames of 1500 bytes at gigabit ethernet rate
//
    priv.tx_coalesce_frames = TX_DESC_NUM * 3 / 4;
    priv.tx_coalesce_usecs = 200;
    hrtimer_setup(&priv.tx_coalesce_timer, tx_done, CLOCK_MONOTONIC, HRTIMER_MODE_REL);
    priv.map = syscon_node_to_regmap(arg.np);
    of_node_put(arg.np);
    if (IS_ERR(priv.map)) {
    dev_warn(d, "no syscon hisilicon,hip04-ppe\n");
    ret = PTR_ERR(priv.map);
    goto init_fail;
    }
    ret = of_get_phy_mode(node, &priv.phy_mode);
    if (ret) {
    dev_warn(d, "not find phy-mode\n");
    goto init_fail;
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    ret = irq;
    goto init_fail;
    }
    ret = devm_request_irq(d, irq, hip04_mac_interrupt,
    0, pdev.name, ndev);
    if (ret) {
    netdev_err(ndev, "devm_request_irq failed\n");
    goto init_fail;
    }
    priv.phy_node = of_parse_phandle(node, "phy-handle", 0);
    if (priv.phy_node) {
    priv.phy = of_phy_connect(ndev, priv.phy_node,
    &hip04_adjust_link,
    0, priv.phy_mode);
    if (!priv.phy) {
    ret = -EPROBE_DEFER;
    goto init_fail;
    }
    }
    INIT_WORK(&priv.tx_timeout_task, hip04_tx_timeout_task);
    ndev.netdev_ops = &hip04_netdev_ops;
    ndev.ethtool_ops = &hip04_ethtool_ops;
    ndev.watchdog_timeo = TX_TIMEOUT;
    ndev.priv_flags |= IFF_UNICAST_FLT;
    ndev.irq = irq;
    netif_napi_add(ndev, &priv.napi, hip04_rx_poll);
    hip04_reset_dreq(priv);
    hip04_reset_ppe(priv);
    if (priv.phy_mode == PHY_INTERFACE_MODE_MII)
    hip04_config_port(ndev, SPEED_100, DUPLEX_FULL);
    hip04_config_fifo(priv);
    eth_hw_addr_random(ndev);
    hip04_update_mac_address(ndev);
    ret = hip04_alloc_ring(ndev, d);
    if (ret) {
    netdev_err(ndev, "alloc ring fail\n");
    goto alloc_fail;
    }
    ret = register_netdev(ndev);
    if (ret)
    goto alloc_fail;
    return 0;
    alloc_fail:
    hip04_free_ring(ndev, d);
    init_fail:
    of_node_put(priv.phy_node);
    free_netdev(ndev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hip04_remove(pdev: *mut platform_device) {
    static void hip04_remove(struct platform_device *pdev)
    {
    struct net_device *ndev = platform_get_drvdata(pdev);
    struct hip04_priv *priv = netdev_priv(ndev);
    struct device *d = &pdev.dev;
    if (priv.phy)
    phy_disconnect(priv.phy);
    hip04_free_ring(ndev, d);
    unregister_netdev(ndev);
    of_node_put(priv.phy_node);
    cancel_work_sync(&priv.tx_timeout_task);
    free_netdev(ndev);
    }
    static const struct of_device_id hip04_mac_match[] = {
    { .compatible = "hisilicon,hip04-mac" },
    { }
    };
    MODULE_DEVICE_TABLE(of, hip04_mac_match);
    static struct platform_driver hip04_mac_driver = {
    .probe	= hip04_mac_probe,
    .remove = hip04_remove,
    .driver	= {
    .name		= DRV_NAME,
    .of_match_table	= hip04_mac_match,
    },
    };
    module_platform_driver(hip04_mac_driver);
    MODULE_DESCRIPTION("HISILICON P04 Ethernet driver");
    MODULE_LICENSE("GPL");

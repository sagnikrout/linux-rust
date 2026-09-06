//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/hisilicon/hix5hd2_gmac.c
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

pub const STATION_ADDR_LOW: c_uint = 0x0000;
pub const STATION_ADDR_HIGH: c_uint = 0x0004;
pub const MAC_DUPLEX_HALF_CTRL: c_uint = 0x0008;
pub const MAX_FRM_SIZE: c_uint = 0x003c;
pub const PORT_MODE: c_uint = 0x0040;
pub const PORT_EN: c_uint = 0x0044;

pub const REC_FILT_CONTROL: c_uint = 0x0064;

pub const PORT_MC_ADDR_LOW: c_uint = 0x0068;
pub const PORT_MC_ADDR_HIGH: c_uint = 0x006C;
pub const CF_CRC_STRIP: c_uint = 0x01b0;
pub const MODE_CHANGE_EN: c_uint = 0x01b4;

pub const COL_SLOT_TIME: c_uint = 0x01c0;
pub const RECV_CONTROL: c_uint = 0x01e0;

pub const CONTROL_WORD: c_uint = 0x0214;
pub const MDIO_SINGLE_CMD: c_uint = 0x03c0;
pub const MDIO_SINGLE_DATA: c_uint = 0x03c4;
pub const MDIO_CTRL: c_uint = 0x03cc;
pub const MDIO_RDATA_STATUS: c_uint = 0x03d0;

pub const RX_FQ_START_ADDR: c_uint = 0x0500;
pub const RX_FQ_DEPTH: c_uint = 0x0504;
pub const RX_FQ_WR_ADDR: c_uint = 0x0508;
pub const RX_FQ_RD_ADDR: c_uint = 0x050c;
pub const RX_FQ_VLDDESC_CNT: c_uint = 0x0510;
pub const RX_FQ_ALEMPTY_TH: c_uint = 0x0514;
pub const RX_FQ_REG_EN: c_uint = 0x0518;

pub const RX_FQ_ALFULL_TH: c_uint = 0x051c;
pub const RX_BQ_START_ADDR: c_uint = 0x0520;
pub const RX_BQ_DEPTH: c_uint = 0x0524;
pub const RX_BQ_WR_ADDR: c_uint = 0x0528;
pub const RX_BQ_RD_ADDR: c_uint = 0x052c;
pub const RX_BQ_FREE_DESC_CNT: c_uint = 0x0530;
pub const RX_BQ_ALEMPTY_TH: c_uint = 0x0534;
pub const RX_BQ_REG_EN: c_uint = 0x0538;

pub const RX_BQ_ALFULL_TH: c_uint = 0x053c;
pub const TX_BQ_START_ADDR: c_uint = 0x0580;
pub const TX_BQ_DEPTH: c_uint = 0x0584;
pub const TX_BQ_WR_ADDR: c_uint = 0x0588;
pub const TX_BQ_RD_ADDR: c_uint = 0x058c;
pub const TX_BQ_VLDDESC_CNT: c_uint = 0x0590;
pub const TX_BQ_ALEMPTY_TH: c_uint = 0x0594;
pub const TX_BQ_REG_EN: c_uint = 0x0598;

pub const TX_BQ_ALFULL_TH: c_uint = 0x059c;
pub const TX_RQ_START_ADDR: c_uint = 0x05a0;
pub const TX_RQ_DEPTH: c_uint = 0x05a4;
pub const TX_RQ_WR_ADDR: c_uint = 0x05a8;
pub const TX_RQ_RD_ADDR: c_uint = 0x05ac;
pub const TX_RQ_FREE_DESC_CNT: c_uint = 0x05b0;
pub const TX_RQ_ALEMPTY_TH: c_uint = 0x05b4;
pub const TX_RQ_REG_EN: c_uint = 0x05b8;

pub const TX_RQ_ALFULL_TH: c_uint = 0x05bc;
pub const RAW_PMU_INT: c_uint = 0x05c0;
pub const ENA_PMU_INT: c_uint = 0x05c4;
pub const STATUS_PMU_INT: c_uint = 0x05c8;

    TX_RQ_IN_INT | TX_RQ_IN_TIMEOUT_INT)
pub const DESC_WR_RD_ENA: c_uint = 0x05cc;
pub const IN_QUEUE_TH: c_uint = 0x05d8;
pub const OUT_QUEUE_TH: c_uint = 0x05dc;
pub const QUEUE_TX_BQ_SHIFT: c_int = 16;
pub const RX_BQ_IN_TIMEOUT_TH: c_uint = 0x05e0;
pub const TX_RQ_IN_TIMEOUT_TH: c_uint = 0x05e4;
pub const STOP_CMD: c_uint = 0x05e8;

pub const FLUSH_CMD: c_uint = 0x05eC;

pub const RX_CFF_NUM_REG: c_uint = 0x05f0;
pub const PMU_FSM_REG: c_uint = 0x05f8;
pub const RX_FIFO_PKT_IN_NUM: c_uint = 0x05fc;
pub const RX_FIFO_PKT_OUT_NUM: c_uint = 0x0600;
pub const RGMII_SPEED_1000: c_uint = 0x2c;
pub const RGMII_SPEED_100: c_uint = 0x2f;
pub const RGMII_SPEED_10: c_uint = 0x2d;
pub const MII_SPEED_100: c_uint = 0x0f;
pub const MII_SPEED_10: c_uint = 0x0d;
pub const GMAC_SPEED_1000: c_uint = 0x05;
pub const GMAC_SPEED_100: c_uint = 0x01;
pub const GMAC_SPEED_10: c_uint = 0x00;

pub const RX_BQ_INT_THRESHOLD: c_uint = 0x01;
pub const TX_RQ_INT_THRESHOLD: c_uint = 0x01;
pub const RX_BQ_IN_TIMEOUT: c_uint = 0x10000;
pub const TX_RQ_IN_TIMEOUT: c_uint = 0x50000;
pub const MAC_MAX_FRAME_SIZE: c_int = 1600;
pub const DESC_SIZE: c_int = 32;
pub const RX_DESC_NUM: c_int = 1024;
pub const TX_DESC_NUM: c_int = 1024;
pub const DESC_VLD_FREE: c_int = 0;
pub const DESC_VLD_BUSY: c_uint = 0x80000000;
pub const DESC_FL_MID: c_int = 0;
pub const DESC_FL_LAST: c_uint = 0x20000000;
pub const DESC_FL_FIRST: c_uint = 0x40000000;
pub const DESC_FL_FULL: c_uint = 0x60000000;
pub const DESC_DATA_LEN_OFF: c_int = 16;
pub const DESC_BUFF_LEN_OFF: c_int = 0;
pub const DESC_DATA_MASK: c_uint = 0x7ff;

pub const DESC_FRAGS_NUM_OFF: c_int = 11;
// DMA descriptor ring helpers

pub const GEMAC_V1: c_int = 0;

    enum phy_reset_delays {
    PRE_DELAY,
    PULSE,
    POST_DELAY,
    DELAYS_NUM,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hix5hd2_desc {
    pub buff_addr: __le32,
    pub cmd: __le32,
    pub __aligned(32): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hix5hd2_desc_sw {
    pub desc: *mut hix5hd2_desc,
    pub phys_addr: dma_addr_t,
    pub count: c_uint,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hix5hd2_sg_desc_ring {
    pub desc: *mut sg_desc,
    pub phys_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frags_info {
    pub addr: __le32,
    pub size: __le32,
}

// hardware supported max skb frags num
pub const SG_MAX_SKB_FRAGS: c_int = 17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_desc {
    pub total_len: __le32,
    pub resvd0: __le32,
    pub linear_addr: __le32,
    pub linear_len: __le32,
// reserve one more frags for memory alignment
    pub 1]: frags_info frags[SG_MAX_SKB_FRAGS +,
}

pub const QUEUE_NUMS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hix5hd2_priv {
    pub pool: [hix5hd2_desc_sw; QUEUE_NUMS],
    pub tx_ring: hix5hd2_sg_desc_ring,
    pub base: *mut void __iomem,
    pub ctrl_base: *mut void __iomem,
    pub tx_skb: [*mut sk_buff; TX_DESC_NUM],
    pub rx_skb: [*mut sk_buff; RX_DESC_NUM],
    pub dev: *mut device,
    pub netdev: *mut net_device,
    pub phy_node: *mut device_node,
    pub phy_mode: phy_interface_t,
    pub hw_cap: c_ulong,
    pub speed: c_uint,
    pub duplex: c_uint,
    pub mac_core_clk: *mut clk,
    pub mac_ifc_clk: *mut clk,
    pub mac_core_rst: *mut reset_control,
    pub mac_ifc_rst: *mut reset_control,
    pub phy_rst: *mut reset_control,
    pub phy_reset_delays: [u32; DELAYS_NUM],
    pub bus: *mut mii_bus,
    pub napi: napi_struct,
    pub tx_timeout_task: work_struct,
}

#[no_mangle]
pub unsafe extern "C" fn hix5hd2_mac_interface_reset(priv: *mut hix5hd2_priv) {
    static inline void hix5hd2_mac_interface_reset(struct hix5hd2_priv *priv)
    {
    if (!priv.mac_ifc_rst)
    return;
    reset_control_assert(priv.mac_ifc_rst);
    reset_control_deassert(priv.mac_ifc_rst);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_config_port(dev: *mut net_device, speed: u32, duplex: u32) {
    static void hix5hd2_config_port(struct net_device *dev, u32 speed, u32 duplex)
    {
    struct hix5hd2_priv *priv = netdev_priv(dev);
    u32 val;
    priv.speed = speed;
    priv.duplex = duplex;
    switch (priv.phy_mode) {
    case PHY_INTERFACE_MODE_RGMII:
    if (speed == SPEED_1000)
    val = RGMII_SPEED_1000;
#[no_mangle]
pub unsafe extern "C" fn if(SPEED_100: speed ==) -> else {
    else if (speed == SPEED_100)
    val = RGMII_SPEED_100;
    else
    val = RGMII_SPEED_10;
    break;
    case PHY_INTERFACE_MODE_MII:
    if (speed == SPEED_100)
    val = MII_SPEED_100;
    else
    val = MII_SPEED_10;
    break;
    default:
    netdev_warn(dev, "not supported mode\n");
    val = MII_SPEED_10;
    break;
    }
    if (duplex)
    val |= GMAC_FULL_DUPLEX;
    writel_relaxed(val, priv.ctrl_base);
    hix5hd2_mac_interface_reset(priv);
    writel_relaxed(BIT_MODE_CHANGE_EN, priv.base + MODE_CHANGE_EN);
    if (speed == SPEED_1000)
    val = GMAC_SPEED_1000;
#[no_mangle]
pub unsafe extern "C" fn if(SPEED_100: speed ==) -> else {
    else if (speed == SPEED_100)
    val = GMAC_SPEED_100;
    else
    val = GMAC_SPEED_10;
    writel_relaxed(val, priv.base + PORT_MODE);
    writel_relaxed(0, priv.base + MODE_CHANGE_EN);
    writel_relaxed(duplex, priv.base + MAC_DUPLEX_HALF_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_set_desc_depth(priv: *mut hix5hd2_priv, rx: c_int, tx: c_int) {
    static void hix5hd2_set_desc_depth(struct hix5hd2_priv *priv, int rx, int tx)
    {
    writel_relaxed(BITS_RX_FQ_DEPTH_EN, priv.base + RX_FQ_REG_EN);
    writel_relaxed(rx << 3, priv.base + RX_FQ_DEPTH);
    writel_relaxed(0, priv.base + RX_FQ_REG_EN);
    writel_relaxed(BITS_RX_BQ_DEPTH_EN, priv.base + RX_BQ_REG_EN);
    writel_relaxed(rx << 3, priv.base + RX_BQ_DEPTH);
    writel_relaxed(0, priv.base + RX_BQ_REG_EN);
    writel_relaxed(BITS_TX_BQ_DEPTH_EN, priv.base + TX_BQ_REG_EN);
    writel_relaxed(tx << 3, priv.base + TX_BQ_DEPTH);
    writel_relaxed(0, priv.base + TX_BQ_REG_EN);
    writel_relaxed(BITS_TX_RQ_DEPTH_EN, priv.base + TX_RQ_REG_EN);
    writel_relaxed(tx << 3, priv.base + TX_RQ_DEPTH);
    writel_relaxed(0, priv.base + TX_RQ_REG_EN);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_set_rx_fq(priv: *mut hix5hd2_priv, phy_addr: dma_addr_t) {
    static void hix5hd2_set_rx_fq(struct hix5hd2_priv *priv, dma_addr_t phy_addr)
    {
    writel_relaxed(BITS_RX_FQ_START_ADDR_EN, priv.base + RX_FQ_REG_EN);
    writel_relaxed(phy_addr, priv.base + RX_FQ_START_ADDR);
    writel_relaxed(0, priv.base + RX_FQ_REG_EN);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_set_rx_bq(priv: *mut hix5hd2_priv, phy_addr: dma_addr_t) {
    static void hix5hd2_set_rx_bq(struct hix5hd2_priv *priv, dma_addr_t phy_addr)
    {
    writel_relaxed(BITS_RX_BQ_START_ADDR_EN, priv.base + RX_BQ_REG_EN);
    writel_relaxed(phy_addr, priv.base + RX_BQ_START_ADDR);
    writel_relaxed(0, priv.base + RX_BQ_REG_EN);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_set_tx_bq(priv: *mut hix5hd2_priv, phy_addr: dma_addr_t) {
    static void hix5hd2_set_tx_bq(struct hix5hd2_priv *priv, dma_addr_t phy_addr)
    {
    writel_relaxed(BITS_TX_BQ_START_ADDR_EN, priv.base + TX_BQ_REG_EN);
    writel_relaxed(phy_addr, priv.base + TX_BQ_START_ADDR);
    writel_relaxed(0, priv.base + TX_BQ_REG_EN);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_set_tx_rq(priv: *mut hix5hd2_priv, phy_addr: dma_addr_t) {
    static void hix5hd2_set_tx_rq(struct hix5hd2_priv *priv, dma_addr_t phy_addr)
    {
    writel_relaxed(BITS_TX_RQ_START_ADDR_EN, priv.base + TX_RQ_REG_EN);
    writel_relaxed(phy_addr, priv.base + TX_RQ_START_ADDR);
    writel_relaxed(0, priv.base + TX_RQ_REG_EN);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_set_desc_addr(priv: *mut hix5hd2_priv) {
    static void hix5hd2_set_desc_addr(struct hix5hd2_priv *priv)
    {
    hix5hd2_set_rx_fq(priv, priv.rx_fq.phys_addr);
    hix5hd2_set_rx_bq(priv, priv.rx_bq.phys_addr);
    hix5hd2_set_tx_rq(priv, priv.tx_rq.phys_addr);
    hix5hd2_set_tx_bq(priv, priv.tx_bq.phys_addr);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_hw_init(priv: *mut hix5hd2_priv) {
    static void hix5hd2_hw_init(struct hix5hd2_priv *priv)
    {
    u32 val;
// disable and clear all interrupts
    writel_relaxed(0, priv.base + ENA_PMU_INT);
    writel_relaxed(~0, priv.base + RAW_PMU_INT);
    writel_relaxed(BIT_CRC_ERR_PASS, priv.base + REC_FILT_CONTROL);
    writel_relaxed(MAC_MAX_FRAME_SIZE, priv.base + CONTROL_WORD);
    writel_relaxed(0, priv.base + COL_SLOT_TIME);
    val = RX_BQ_INT_THRESHOLD | TX_RQ_INT_THRESHOLD << QUEUE_TX_BQ_SHIFT;
    writel_relaxed(val, priv.base + IN_QUEUE_TH);
    writel_relaxed(RX_BQ_IN_TIMEOUT, priv.base + RX_BQ_IN_TIMEOUT_TH);
    writel_relaxed(TX_RQ_IN_TIMEOUT, priv.base + TX_RQ_IN_TIMEOUT_TH);
    hix5hd2_set_desc_depth(priv, RX_DESC_NUM, TX_DESC_NUM);
    hix5hd2_set_desc_addr(priv);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_irq_enable(priv: *mut hix5hd2_priv) {
    static void hix5hd2_irq_enable(struct hix5hd2_priv *priv)
    {
    writel_relaxed(DEF_INT_MASK, priv.base + ENA_PMU_INT);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_irq_disable(priv: *mut hix5hd2_priv) {
    static void hix5hd2_irq_disable(struct hix5hd2_priv *priv)
    {
    writel_relaxed(0, priv.base + ENA_PMU_INT);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_port_enable(priv: *mut hix5hd2_priv) {
    static void hix5hd2_port_enable(struct hix5hd2_priv *priv)
    {
    writel_relaxed(0xf, priv.base + DESC_WR_RD_ENA);
    writel_relaxed(BITS_RX_EN | BITS_TX_EN, priv.base + PORT_EN);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_port_disable(priv: *mut hix5hd2_priv) {
    static void hix5hd2_port_disable(struct hix5hd2_priv *priv)
    {
    writel_relaxed(~(u32)(BITS_RX_EN | BITS_TX_EN), priv.base + PORT_EN);
    writel_relaxed(0, priv.base + DESC_WR_RD_ENA);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_hw_set_mac_addr(dev: *mut net_device) {
    static void hix5hd2_hw_set_mac_addr(struct net_device *dev)
    {
    struct hix5hd2_priv *priv = netdev_priv(dev);
    const unsigned char *mac = dev.dev_addr;
    u32 val;
    val = mac[1] | (mac[0] << 8);
    writel_relaxed(val, priv.base + STATION_ADDR_HIGH);
    val = mac[5] | (mac[4] << 8) | (mac[3] << 16) | (mac[2] << 24);
    writel_relaxed(val, priv.base + STATION_ADDR_LOW);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_net_set_mac_address(dev: *mut net_device, p: *mut c_void) -> c_int {
    static int hix5hd2_net_set_mac_address(struct net_device *dev, void *p)
    {
    int ret;
    ret = eth_mac_addr(dev, p);
    if (!ret)
    hix5hd2_hw_set_mac_addr(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_adjust_link(dev: *mut net_device) {
    static void hix5hd2_adjust_link(struct net_device *dev)
    {
    struct hix5hd2_priv *priv = netdev_priv(dev);
    struct phy_device *phy = dev.phydev;
    if ((priv.speed != phy.speed) || (priv.duplex != phy.duplex)) {
    hix5hd2_config_port(dev, phy.speed, phy.duplex);
    phy_print_status(phy);
    }
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_rx_refill(priv: *mut hix5hd2_priv) {
    static void hix5hd2_rx_refill(struct hix5hd2_priv *priv)
    {
    struct hix5hd2_desc *desc;
    struct sk_buff *skb;
    u32 start, end, num, pos, i;
    let mut len: u32 = MAC_MAX_FRAME_SIZE;
    dma_addr_t addr;
// software write pointer
    start = dma_cnt(readl_relaxed(priv.base + RX_FQ_WR_ADDR));
// logic read pointer
    end = dma_cnt(readl_relaxed(priv.base + RX_FQ_RD_ADDR));
    num = CIRC_SPACE(start, end, RX_DESC_NUM);
    for (i = 0, pos = start; i < num; i++) {
    if (priv.rx_skb[pos]) {
    break;
    } else {
    skb = netdev_alloc_skb_ip_align(priv.netdev, len);
    if (unlikely(skb == core::ptr::null_mut()))
    break;
    }
    addr = dma_map_single(priv.dev, skb.data, len, DMA_FROM_DEVICE);
    if (dma_mapping_error(priv.dev, addr)) {
    dev_kfree_skb_any(skb);
    break;
    }
    desc = priv.rx_fq.desc + pos;
    desc.buff_addr = cpu_to_le32(addr);
    priv.rx_skb[pos] = skb;
    desc.cmd = cpu_to_le32(DESC_VLD_FREE |
    (len - 1) << DESC_BUFF_LEN_OFF);
    pos = dma_ring_incr(pos, RX_DESC_NUM);
    }
// ensure desc updated
    wmb();
    if (pos != start)
    writel_relaxed(dma_byte(pos), priv.base + RX_FQ_WR_ADDR);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_rx(dev: *mut net_device, limit: c_int) -> c_int {
    static int hix5hd2_rx(struct net_device *dev, int limit)
    {
    struct hix5hd2_priv *priv = netdev_priv(dev);
    struct sk_buff *skb;
    struct hix5hd2_desc *desc;
    dma_addr_t addr;
    u32 start, end, num, pos, i, len;
// software read pointer
    start = dma_cnt(readl_relaxed(priv.base + RX_BQ_RD_ADDR));
// logic write pointer
    end = dma_cnt(readl_relaxed(priv.base + RX_BQ_WR_ADDR));
    num = CIRC_CNT(end, start, RX_DESC_NUM);
    if (num > limit)
    num = limit;
// ensure get updated desc
    rmb();
    for (i = 0, pos = start; i < num; i++) {
    skb = priv.rx_skb[pos];
    if (unlikely(!skb)) {
    netdev_err(dev, "inconsistent rx_skb\n");
    break;
    }
    priv.rx_skb[pos] = core::ptr::null_mut();
    desc = priv.rx_bq.desc + pos;
    len = (le32_to_cpu(desc.cmd) >> DESC_DATA_LEN_OFF) &
    DESC_DATA_MASK;
    addr = le32_to_cpu(desc.buff_addr);
    dma_unmap_single(priv.dev, addr, MAC_MAX_FRAME_SIZE,
    DMA_FROM_DEVICE);
    skb_put(skb, len);
    if (skb.len > MAC_MAX_FRAME_SIZE) {
    netdev_err(dev, "rcv len err, len = %d\n", skb.len);
    dev.stats.rx_errors++;
    dev.stats.rx_length_errors++;
    dev_kfree_skb_any(skb);
    goto next;
    }
    skb.protocol = eth_type_trans(skb, dev);
    napi_gro_receive(&priv.napi, skb);
    dev.stats.rx_packets++;
    dev.stats.rx_bytes += len;
    next:
    pos = dma_ring_incr(pos, RX_DESC_NUM);
    }
    if (pos != start)
    writel_relaxed(dma_byte(pos), priv.base + RX_BQ_RD_ADDR);
    hix5hd2_rx_refill(priv);
    return num;
    }
    static void hix5hd2_clean_sg_desc(struct hix5hd2_priv *priv,
    struct sk_buff *skb, u32 pos)
    {
    struct sg_desc *desc;
    dma_addr_t addr;
    u32 len;
    int i;
    desc = priv.tx_ring.desc + pos;
    addr = le32_to_cpu(desc.linear_addr);
    len = le32_to_cpu(desc.linear_len);
    dma_unmap_single(priv.dev, addr, len, DMA_TO_DEVICE);
    for (i = 0; i < skb_shinfo(skb).nr_frags; i++) {
    addr = le32_to_cpu(desc.frags[i].addr);
    len = le32_to_cpu(desc.frags[i].size);
    dma_unmap_page(priv.dev, addr, len, DMA_TO_DEVICE);
    }
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_xmit_reclaim(dev: *mut net_device) {
    static void hix5hd2_xmit_reclaim(struct net_device *dev)
    {
    struct sk_buff *skb;
    struct hix5hd2_desc *desc;
    struct hix5hd2_priv *priv = netdev_priv(dev);
    let mut bytes_compl: c_uint = 0, pkts_compl = 0;
    u32 start, end, num, pos, i;
    dma_addr_t addr;
    netif_tx_lock(dev);
// software read
    start = dma_cnt(readl_relaxed(priv.base + TX_RQ_RD_ADDR));
// logic write
    end = dma_cnt(readl_relaxed(priv.base + TX_RQ_WR_ADDR));
    num = CIRC_CNT(end, start, TX_DESC_NUM);
    for (i = 0, pos = start; i < num; i++) {
    skb = priv.tx_skb[pos];
    if (unlikely(!skb)) {
    netdev_err(dev, "inconsistent tx_skb\n");
    break;
    }
    pkts_compl++;
    bytes_compl += skb.len;
    desc = priv.tx_rq.desc + pos;
    if (skb_shinfo(skb).nr_frags) {
    hix5hd2_clean_sg_desc(priv, skb, pos);
    } else {
    addr = le32_to_cpu(desc.buff_addr);
    dma_unmap_single(priv.dev, addr, skb.len,
    DMA_TO_DEVICE);
    }
    priv.tx_skb[pos] = core::ptr::null_mut();
    dev_consume_skb_any(skb);
    pos = dma_ring_incr(pos, TX_DESC_NUM);
    }
    if (pos != start)
    writel_relaxed(dma_byte(pos), priv.base + TX_RQ_RD_ADDR);
    netif_tx_unlock(dev);
    if (pkts_compl || bytes_compl)
    netdev_completed_queue(dev, pkts_compl, bytes_compl);
    if (unlikely(netif_queue_stopped(priv.netdev)) && pkts_compl)
    netif_wake_queue(priv.netdev);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int hix5hd2_poll(struct napi_struct *napi, int budget)
    {
    struct hix5hd2_priv *priv = container_of(napi,
    struct hix5hd2_priv, napi);
    struct net_device *dev = priv.netdev;
    let mut work_done: c_int = 0, task = budget;
    int ints, num;
    do {
    hix5hd2_xmit_reclaim(dev);
    num = hix5hd2_rx(dev, task);
    work_done += num;
    task -= num;
    if ((work_done >= budget) || (num == 0))
    break;
    ints = readl_relaxed(priv.base + RAW_PMU_INT);
    writel_relaxed(ints, priv.base + RAW_PMU_INT);
    } while (ints & DEF_INT_MASK);
    if (work_done < budget) {
    napi_complete_done(napi, work_done);
    hix5hd2_irq_enable(priv);
    }
    return work_done;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t hix5hd2_interrupt(int irq, void *dev_id)
    {
    struct net_device *dev = (struct net_device *)dev_id;
    struct hix5hd2_priv *priv = netdev_priv(dev);
    let mut ints: c_int = readl_relaxed(priv.base + RAW_PMU_INT);
    writel_relaxed(ints, priv.base + RAW_PMU_INT);
    if (likely(ints & DEF_INT_MASK)) {
    hix5hd2_irq_disable(priv);
    napi_schedule(&priv.napi);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_get_desc_cmd(skb: *mut sk_buff, hw_cap: c_ulong) -> u32 {
    static u32 hix5hd2_get_desc_cmd(struct sk_buff *skb, unsigned long hw_cap)
    {
    let mut cmd: u32 = 0;
    if (HAS_CAP_TSO(hw_cap)) {
    if (skb_shinfo(skb).nr_frags)
    cmd |= DESC_SG;
    cmd |= skb_shinfo(skb).nr_frags << DESC_FRAGS_NUM_OFF;
    } else {
    cmd |= DESC_FL_FULL |
    ((skb.len & DESC_DATA_MASK) << DESC_BUFF_LEN_OFF);
    }
    cmd |= (skb.len & DESC_DATA_MASK) << DESC_DATA_LEN_OFF;
    cmd |= DESC_VLD_BUSY;
    return cmd;
    }
    static int hix5hd2_fill_sg_desc(struct hix5hd2_priv *priv,
    struct sk_buff *skb, u32 pos)
    {
    struct sg_desc *desc;
    dma_addr_t addr;
    int ret;
    int i;
    desc = priv.tx_ring.desc + pos;
    desc.total_len = cpu_to_le32(skb.len);
    addr = dma_map_single(priv.dev, skb.data, skb_headlen(skb),
    DMA_TO_DEVICE);
    if (unlikely(dma_mapping_error(priv.dev, addr)))
    return -EINVAL;
    desc.linear_addr = cpu_to_le32(addr);
    desc.linear_len = cpu_to_le32(skb_headlen(skb));
    for (i = 0; i < skb_shinfo(skb).nr_frags; i++) {
    skb_frag_t *frag = &skb_shinfo(skb).frags[i];
    let mut len: c_int = skb_frag_size(frag);
    addr = skb_frag_dma_map(priv.dev, frag, 0, len, DMA_TO_DEVICE);
    ret = dma_mapping_error(priv.dev, addr);
    if (unlikely(ret))
    return -EINVAL;
    desc.frags[i].addr = cpu_to_le32(addr);
    desc.frags[i].size = cpu_to_le32(len);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_net_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t hix5hd2_net_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct hix5hd2_priv *priv = netdev_priv(dev);
    struct hix5hd2_desc *desc;
    dma_addr_t addr;
    u32 pos;
    u32 cmd;
    int ret;
// software write pointer
    pos = dma_cnt(readl_relaxed(priv.base + TX_BQ_WR_ADDR));
    if (unlikely(priv.tx_skb[pos])) {
    dev.stats.tx_dropped++;
    dev.stats.tx_fifo_errors++;
    netif_stop_queue(dev);
    return NETDEV_TX_BUSY;
    }
    desc = priv.tx_bq.desc + pos;
    cmd = hix5hd2_get_desc_cmd(skb, priv.hw_cap);
    desc.cmd = cpu_to_le32(cmd);
    if (skb_shinfo(skb).nr_frags) {
    ret = hix5hd2_fill_sg_desc(priv, skb, pos);
    if (unlikely(ret)) {
    dev_kfree_skb_any(skb);
    dev.stats.tx_dropped++;
    return NETDEV_TX_OK;
    }
    addr = priv.tx_ring.phys_addr + pos * sizeof(struct sg_desc);
    } else {
    addr = dma_map_single(priv.dev, skb.data, skb.len,
    DMA_TO_DEVICE);
    if (unlikely(dma_mapping_error(priv.dev, addr))) {
    dev_kfree_skb_any(skb);
    dev.stats.tx_dropped++;
    return NETDEV_TX_OK;
    }
    }
    desc.buff_addr = cpu_to_le32(addr);
    priv.tx_skb[pos] = skb;
// ensure desc updated
    wmb();
    pos = dma_ring_incr(pos, TX_DESC_NUM);
    writel_relaxed(dma_byte(pos), priv.base + TX_BQ_WR_ADDR);
    netif_trans_update(dev);
    dev.stats.tx_packets++;
    dev.stats.tx_bytes += skb.len;
    netdev_sent_queue(dev, skb.len);
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_free_dma_desc_rings(priv: *mut hix5hd2_priv) {
    static void hix5hd2_free_dma_desc_rings(struct hix5hd2_priv *priv)
    {
    struct hix5hd2_desc *desc;
    dma_addr_t addr;
    int i;
    for (i = 0; i < RX_DESC_NUM; i++) {
    struct sk_buff *skb = priv.rx_skb[i];
    if (skb == core::ptr::null_mut())
    continue;
    desc = priv.rx_fq.desc + i;
    addr = le32_to_cpu(desc.buff_addr);
    dma_unmap_single(priv.dev, addr,
    MAC_MAX_FRAME_SIZE, DMA_FROM_DEVICE);
    dev_kfree_skb_any(skb);
    priv.rx_skb[i] = core::ptr::null_mut();
    }
    for (i = 0; i < TX_DESC_NUM; i++) {
    struct sk_buff *skb = priv.tx_skb[i];
    if (skb == core::ptr::null_mut())
    continue;
    desc = priv.tx_rq.desc + i;
    addr = le32_to_cpu(desc.buff_addr);
    dma_unmap_single(priv.dev, addr, skb.len, DMA_TO_DEVICE);
    dev_kfree_skb_any(skb);
    priv.tx_skb[i] = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_net_open(dev: *mut net_device) -> c_int {
    static int hix5hd2_net_open(struct net_device *dev)
    {
    struct hix5hd2_priv *priv = netdev_priv(dev);
    struct phy_device *phy;
    int ret;
    ret = clk_prepare_enable(priv.mac_core_clk);
    if (ret < 0) {
    netdev_err(dev, "failed to enable mac core clk %d\n", ret);
    return ret;
    }
    ret = clk_prepare_enable(priv.mac_ifc_clk);
    if (ret < 0) {
    clk_disable_unprepare(priv.mac_core_clk);
    netdev_err(dev, "failed to enable mac ifc clk %d\n", ret);
    return ret;
    }
    phy = of_phy_connect(dev, priv.phy_node,
    &hix5hd2_adjust_link, 0, priv.phy_mode);
    if (!phy) {
    clk_disable_unprepare(priv.mac_ifc_clk);
    clk_disable_unprepare(priv.mac_core_clk);
    return -ENODEV;
    }
    phy_start(phy);
    hix5hd2_hw_init(priv);
    hix5hd2_rx_refill(priv);
    netdev_reset_queue(dev);
    netif_start_queue(dev);
    napi_enable(&priv.napi);
    hix5hd2_port_enable(priv);
    hix5hd2_irq_enable(priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_net_close(dev: *mut net_device) -> c_int {
    static int hix5hd2_net_close(struct net_device *dev)
    {
    struct hix5hd2_priv *priv = netdev_priv(dev);
    hix5hd2_port_disable(priv);
    hix5hd2_irq_disable(priv);
    napi_disable(&priv.napi);
    netif_stop_queue(dev);
    hix5hd2_free_dma_desc_rings(priv);
    if (dev.phydev) {
    phy_stop(dev.phydev);
    phy_disconnect(dev.phydev);
    }
    clk_disable_unprepare(priv.mac_ifc_clk);
    clk_disable_unprepare(priv.mac_core_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_tx_timeout_task(work: *mut work_struct) {
    static void hix5hd2_tx_timeout_task(struct work_struct *work)
    {
    struct hix5hd2_priv *priv;
    priv = container_of(work, struct hix5hd2_priv, tx_timeout_task);
    hix5hd2_net_close(priv.netdev);
    hix5hd2_net_open(priv.netdev);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_net_timeout(dev: *mut net_device, txqueue: c_uint) {
    static void hix5hd2_net_timeout(struct net_device *dev, unsigned int txqueue)
    {
    struct hix5hd2_priv *priv = netdev_priv(dev);
    schedule_work(&priv.tx_timeout_task);
    }
    static const struct net_device_ops hix5hd2_netdev_ops = {
    .ndo_open		= hix5hd2_net_open,
    .ndo_stop		= hix5hd2_net_close,
    .ndo_start_xmit		= hix5hd2_net_xmit,
    .ndo_tx_timeout		= hix5hd2_net_timeout,
    .ndo_set_mac_address	= hix5hd2_net_set_mac_address,
    };
    static const struct ethtool_ops hix5hd2_ethtools_ops = {
    .get_link		= ethtool_op_get_link,
    .get_link_ksettings     = phy_ethtool_get_link_ksettings,
    .set_link_ksettings     = phy_ethtool_set_link_ksettings,
    };
#[no_mangle]
unsafe extern "C" fn hix5hd2_mdio_wait_ready(bus: *mut mii_bus) -> c_int {
    static int hix5hd2_mdio_wait_ready(struct mii_bus *bus)
    {
    struct hix5hd2_priv *priv = bus.priv;
    void __iomem *base = priv.base;
    int i, timeout = 10000;
    for (i = 0; readl_relaxed(base + MDIO_SINGLE_CMD) & MDIO_START; i++) {
    if (i == timeout)
    return -ETIMEDOUT;
    usleep_range(10, 20);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_mdio_read(bus: *mut mii_bus, phy: c_int, reg: c_int) -> c_int {
    static int hix5hd2_mdio_read(struct mii_bus *bus, int phy, int reg)
    {
    struct hix5hd2_priv *priv = bus.priv;
    void __iomem *base = priv.base;
    int val, ret;
    ret = hix5hd2_mdio_wait_ready(bus);
    if (ret < 0)
    goto out;
    writel_relaxed(MDIO_READ | phy << 8 | reg, base + MDIO_SINGLE_CMD);
    ret = hix5hd2_mdio_wait_ready(bus);
    if (ret < 0)
    goto out;
    val = readl_relaxed(base + MDIO_RDATA_STATUS);
    if (val & MDIO_R_VALID) {
    dev_err(bus.parent, "SMI bus read not valid\n");
    ret = -ENODEV;
    goto out;
    }
    val = readl_relaxed(priv.base + MDIO_SINGLE_DATA);
    ret = (val >> 16) & 0xFFFF;
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_mdio_write(bus: *mut mii_bus, phy: c_int, reg: c_int, val: u16) -> c_int {
    static int hix5hd2_mdio_write(struct mii_bus *bus, int phy, int reg, u16 val)
    {
    struct hix5hd2_priv *priv = bus.priv;
    void __iomem *base = priv.base;
    int ret;
    ret = hix5hd2_mdio_wait_ready(bus);
    if (ret < 0)
    goto out;
    writel_relaxed(val, base + MDIO_SINGLE_DATA);
    writel_relaxed(MDIO_WRITE | phy << 8 | reg, base + MDIO_SINGLE_CMD);
    ret = hix5hd2_mdio_wait_ready(bus);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_destroy_hw_desc_queue(priv: *mut hix5hd2_priv) {
    static void hix5hd2_destroy_hw_desc_queue(struct hix5hd2_priv *priv)
    {
    int i;
    for (i = 0; i < QUEUE_NUMS; i++) {
    if (priv.pool[i].desc) {
    dma_free_coherent(priv.dev, priv.pool[i].size,
    priv.pool[i].desc,
    priv.pool[i].phys_addr);
    priv.pool[i].desc = core::ptr::null_mut();
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_init_hw_desc_queue(priv: *mut hix5hd2_priv) -> c_int {
    static int hix5hd2_init_hw_desc_queue(struct hix5hd2_priv *priv)
    {
    struct device *dev = priv.dev;
    struct hix5hd2_desc *virt_addr;
    dma_addr_t phys_addr;
    int size, i;
    priv.rx_fq.count = RX_DESC_NUM;
    priv.rx_bq.count = RX_DESC_NUM;
    priv.tx_bq.count = TX_DESC_NUM;
    priv.tx_rq.count = TX_DESC_NUM;
    for (i = 0; i < QUEUE_NUMS; i++) {
    size = priv.pool[i].count * sizeof(struct hix5hd2_desc);
    virt_addr = dma_alloc_coherent(dev, size, &phys_addr,
    GFP_KERNEL);
    if (virt_addr == core::ptr::null_mut())
    goto error_free_pool;
    priv.pool[i].size = size;
    priv.pool[i].desc = virt_addr;
    priv.pool[i].phys_addr = phys_addr;
    }
    return 0;
    error_free_pool:
    hix5hd2_destroy_hw_desc_queue(priv);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_init_sg_desc_queue(priv: *mut hix5hd2_priv) -> c_int {
    static int hix5hd2_init_sg_desc_queue(struct hix5hd2_priv *priv)
    {
    struct sg_desc *desc;
    dma_addr_t phys_addr;
    desc = dma_alloc_coherent(priv.dev,
    TX_DESC_NUM * sizeof(struct sg_desc),
    &phys_addr, GFP_KERNEL);
    if (!desc)
    return -ENOMEM;
    priv.tx_ring.desc = desc;
    priv.tx_ring.phys_addr = phys_addr;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_destroy_sg_desc_queue(priv: *mut hix5hd2_priv) {
    static void hix5hd2_destroy_sg_desc_queue(struct hix5hd2_priv *priv)
    {
    if (priv.tx_ring.desc) {
    dma_free_coherent(priv.dev,
    TX_DESC_NUM * sizeof(struct sg_desc),
    priv.tx_ring.desc, priv.tx_ring.phys_addr);
    priv.tx_ring.desc = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hix5hd2_mac_core_reset(priv: *mut hix5hd2_priv) {
    static inline void hix5hd2_mac_core_reset(struct hix5hd2_priv *priv)
    {
    if (!priv.mac_core_rst)
    return;
    reset_control_assert(priv.mac_core_rst);
    reset_control_deassert(priv.mac_core_rst);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_sleep_us(time_us: u32) {
    static void hix5hd2_sleep_us(u32 time_us)
    {
    u32 time_ms;
    if (!time_us)
    return;
    time_ms = DIV_ROUND_UP(time_us, 1000);
    if (time_ms < 20)
    usleep_range(time_us, time_us + 500);
    else
    msleep(time_ms);
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_phy_reset(priv: *mut hix5hd2_priv) {
    static void hix5hd2_phy_reset(struct hix5hd2_priv *priv)
    {
// To make sure PHY hardware reset success,
// we must keep PHY in deassert state first and
// then complete the hardware reset operation
//
    reset_control_deassert(priv.phy_rst);
    hix5hd2_sleep_us(priv.phy_reset_delays[PRE_DELAY]);
    reset_control_assert(priv.phy_rst);
// delay some time to ensure reset ok,
// this depends on PHY hardware feature
//
    hix5hd2_sleep_us(priv.phy_reset_delays[PULSE]);
    reset_control_deassert(priv.phy_rst);
// delay some time to ensure later MDIO access
    hix5hd2_sleep_us(priv.phy_reset_delays[POST_DELAY]);
    }
    static const struct of_device_id hix5hd2_of_match[];
#[no_mangle]
unsafe extern "C" fn hix5hd2_dev_probe(pdev: *mut platform_device) -> c_int {
    static int hix5hd2_dev_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    struct net_device *ndev;
    struct hix5hd2_priv *priv;
    struct mii_bus *bus;
    int ret;
    ndev = alloc_etherdev(sizeof(struct hix5hd2_priv));
    if (!ndev)
    return -ENOMEM;
    platform_set_drvdata(pdev, ndev);
    priv = netdev_priv(ndev);
    priv.dev = dev;
    priv.netdev = ndev;
    priv.hw_cap = (unsigned long)device_get_match_data(dev);
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base)) {
    ret = PTR_ERR(priv.base);
    goto out_free_netdev;
    }
    priv.ctrl_base = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(priv.ctrl_base)) {
    ret = PTR_ERR(priv.ctrl_base);
    goto out_free_netdev;
    }
    priv.mac_core_clk = devm_clk_get(&pdev.dev, "mac_core");
    if (IS_ERR(priv.mac_core_clk)) {
    netdev_err(ndev, "failed to get mac core clk\n");
    ret = -ENODEV;
    goto out_free_netdev;
    }
    ret = clk_prepare_enable(priv.mac_core_clk);
    if (ret < 0) {
    netdev_err(ndev, "failed to enable mac core clk %d\n", ret);
    goto out_free_netdev;
    }
    priv.mac_ifc_clk = devm_clk_get(&pdev.dev, "mac_ifc");
    if (IS_ERR(priv.mac_ifc_clk))
    priv.mac_ifc_clk = core::ptr::null_mut();
    ret = clk_prepare_enable(priv.mac_ifc_clk);
    if (ret < 0) {
    netdev_err(ndev, "failed to enable mac ifc clk %d\n", ret);
    goto out_disable_mac_core_clk;
    }
    priv.mac_core_rst = devm_reset_control_get(dev, "mac_core");
    if (IS_ERR(priv.mac_core_rst))
    priv.mac_core_rst = core::ptr::null_mut();
    hix5hd2_mac_core_reset(priv);
    priv.mac_ifc_rst = devm_reset_control_get(dev, "mac_ifc");
    if (IS_ERR(priv.mac_ifc_rst))
    priv.mac_ifc_rst = core::ptr::null_mut();
    priv.phy_rst = devm_reset_control_get(dev, "phy");
    if (IS_ERR(priv.phy_rst)) {
    priv.phy_rst = core::ptr::null_mut();
    } else {
    ret = of_property_read_u32_array(node,
    PHY_RESET_DELAYS_PROPERTY,
    priv.phy_reset_delays,
    DELAYS_NUM);
    if (ret)
    goto out_disable_clk;
    hix5hd2_phy_reset(priv);
    }
    bus = mdiobus_alloc();
    if (bus == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto out_disable_clk;
    }
    bus.priv = priv;
    bus.name = "hix5hd2_mii_bus";
    bus.read = hix5hd2_mdio_read;
    bus.write = hix5hd2_mdio_write;
    bus.parent = &pdev.dev;
    snprintf(bus.id, MII_BUS_ID_SIZE, "%s-mii", dev_name(&pdev.dev));
    priv.bus = bus;
    ret = of_mdiobus_register(bus, node);
    if (ret)
    goto err_free_mdio;
    ret = of_get_phy_mode(node, &priv.phy_mode);
    if (ret) {
    netdev_err(ndev, "not find phy-mode\n");
    goto err_mdiobus;
    }
    priv.phy_node = of_parse_phandle(node, "phy-handle", 0);
    if (!priv.phy_node) {
    netdev_err(ndev, "not find phy-handle\n");
    ret = -EINVAL;
    goto err_mdiobus;
    }
    ndev.irq = platform_get_irq(pdev, 0);
    if (ndev.irq < 0) {
    ret = ndev.irq;
    goto out_phy_node;
    }
    ret = devm_request_irq(dev, ndev.irq, hix5hd2_interrupt,
    0, pdev.name, ndev);
    if (ret) {
    netdev_err(ndev, "devm_request_irq failed\n");
    goto out_phy_node;
    }
    ret = of_get_ethdev_address(node, ndev);
    if (ret) {
    eth_hw_addr_random(ndev);
    netdev_warn(ndev, "using random MAC address %pM\n",
    ndev.dev_addr);
    }
    INIT_WORK(&priv.tx_timeout_task, hix5hd2_tx_timeout_task);
    ndev.watchdog_timeo = 6 * HZ;
    ndev.priv_flags |= IFF_UNICAST_FLT;
    ndev.netdev_ops = &hix5hd2_netdev_ops;
    ndev.ethtool_ops = &hix5hd2_ethtools_ops;
    SET_NETDEV_DEV(ndev, dev);
    if (HAS_CAP_TSO(priv.hw_cap))
    ndev.hw_features |= NETIF_F_SG;
    ndev.features |= ndev.hw_features | NETIF_F_HIGHDMA;
    ndev.vlan_features |= ndev.features;
    ret = hix5hd2_init_hw_desc_queue(priv);
    if (ret)
    goto out_phy_node;
    netif_napi_add(ndev, &priv.napi, hix5hd2_poll);
    if (HAS_CAP_TSO(priv.hw_cap)) {
    ret = hix5hd2_init_sg_desc_queue(priv);
    if (ret)
    goto out_destroy_queue;
    }
    ret = register_netdev(priv.netdev);
    if (ret) {
    netdev_err(ndev, "register_netdev failed!");
    goto out_destroy_queue;
    }
    clk_disable_unprepare(priv.mac_ifc_clk);
    clk_disable_unprepare(priv.mac_core_clk);
    return ret;
    out_destroy_queue:
    if (HAS_CAP_TSO(priv.hw_cap))
    hix5hd2_destroy_sg_desc_queue(priv);
    netif_napi_del(&priv.napi);
    hix5hd2_destroy_hw_desc_queue(priv);
    out_phy_node:
    of_node_put(priv.phy_node);
    err_mdiobus:
    mdiobus_unregister(bus);
    err_free_mdio:
    mdiobus_free(bus);
    out_disable_clk:
    clk_disable_unprepare(priv.mac_ifc_clk);
    out_disable_mac_core_clk:
    clk_disable_unprepare(priv.mac_core_clk);
    out_free_netdev:
    free_netdev(ndev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hix5hd2_dev_remove(pdev: *mut platform_device) {
    static void hix5hd2_dev_remove(struct platform_device *pdev)
    {
    struct net_device *ndev = platform_get_drvdata(pdev);
    struct hix5hd2_priv *priv = netdev_priv(ndev);
    unregister_netdev(ndev);
    mdiobus_unregister(priv.bus);
    mdiobus_free(priv.bus);
    if (HAS_CAP_TSO(priv.hw_cap))
    hix5hd2_destroy_sg_desc_queue(priv);
    hix5hd2_destroy_hw_desc_queue(priv);
    of_node_put(priv.phy_node);
    cancel_work_sync(&priv.tx_timeout_task);
    free_netdev(ndev);
    }
    static const struct of_device_id hix5hd2_of_match[] = {
    { .compatible = "hisilicon,hisi-gmac-v1", .data = (void *)GEMAC_V1 },
    { .compatible = "hisilicon,hisi-gmac-v2", .data = (void *)GEMAC_V2 },
    { .compatible = "hisilicon,hix5hd2-gmac", .data = (void *)GEMAC_V1 },
    { .compatible = "hisilicon,hi3798cv200-gmac", .data = (void *)GEMAC_V2 },
    { .compatible = "hisilicon,hi3516a-gmac", .data = (void *)GEMAC_V2 },
    {},
    };
    MODULE_DEVICE_TABLE(of, hix5hd2_of_match);
    static struct platform_driver hix5hd2_dev_driver = {
    .driver = {
    .name = "hisi-gmac",
    .of_match_table = hix5hd2_of_match,
    },
    .probe = hix5hd2_dev_probe,
    .remove = hix5hd2_dev_remove,
    };
    module_platform_driver(hix5hd2_dev_driver);
    MODULE_DESCRIPTION("HISILICON Gigabit Ethernet MAC driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:hisi-gmac");

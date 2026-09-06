//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mediatek/mtk_star_emac.c
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
// Copyright (c) 2020 MediaTek Corporation
// Copyright (c) 2020 BayLibre SAS
//
// Author: Bartosz Golaszewski <bgolaszewski@baylibre.com>
//

pub const MTK_STAR_WAIT_TIMEOUT: c_int = 300;
pub const MTK_STAR_MAX_FRAME_SIZE: c_int = 1514;
pub const MTK_STAR_SKB_ALIGNMENT: c_int = 16;
pub const MTK_STAR_HASHTABLE_MC_LIMIT: c_int = 256;
pub const MTK_STAR_HASHTABLE_SIZE_MAX: c_int = 512;

// Normally we'd use NET_IP_ALIGN but on arm64 its value is 0 and it doesn't
// work for this controller.
//
pub const MTK_STAR_IP_ALIGN: c_int = 2;
    static const char *const mtk_star_clk_names[] = { "core", "reg", "trans" };

// PHY Control Register 0
pub const MTK_STAR_REG_PHY_CTRL0: c_uint = 0x0000;

pub const MTK_STAR_OFF_PHY_CTRL0_PREG: c_int = 8;

pub const MTK_STAR_OFF_PHY_CTRL0_RWDATA: c_int = 16;
// PHY Control Register 1
pub const MTK_STAR_REG_PHY_CTRL1: c_uint = 0x0004;

pub const MTK_STAR_OFF_PHY_CTRL1_FORCE_SPD: c_int = 9;
pub const MTK_STAR_VAL_PHY_CTRL1_FORCE_SPD_10M: c_uint = 0x00;
pub const MTK_STAR_VAL_PHY_CTRL1_FORCE_SPD_100M: c_uint = 0x01;
pub const MTK_STAR_VAL_PHY_CTRL1_FORCE_SPD_1000M: c_uint = 0x02;

// MAC Configuration Register
pub const MTK_STAR_REG_MAC_CFG: c_uint = 0x0008;
pub const MTK_STAR_OFF_MAC_CFG_IPG: c_int = 10;

// Flow-Control Configuration Register
pub const MTK_STAR_REG_FC_CFG: c_uint = 0x000c;

pub const MTK_STAR_OFF_FC_CFG_SEND_PAUSE_TH: c_int = 16;

pub const MTK_STAR_VAL_FC_CFG_SEND_PAUSE_TH_2K: c_uint = 0x800;
// ARL Configuration Register
pub const MTK_STAR_REG_ARL_CFG: c_uint = 0x0010;

// MAC High and Low Bytes Registers
pub const MTK_STAR_REG_MY_MAC_H: c_uint = 0x0014;
pub const MTK_STAR_REG_MY_MAC_L: c_uint = 0x0018;
// Hash Table Control Register
pub const MTK_STAR_REG_HASH_CTRL: c_uint = 0x001c;

// TX DMA Control Register
pub const MTK_STAR_REG_TX_DMA_CTRL: c_uint = 0x0034;

// RX DMA Control Register
pub const MTK_STAR_REG_RX_DMA_CTRL: c_uint = 0x0038;

// DMA Address Registers
pub const MTK_STAR_REG_TX_DPTR: c_uint = 0x003c;
pub const MTK_STAR_REG_RX_DPTR: c_uint = 0x0040;
pub const MTK_STAR_REG_TX_BASE_ADDR: c_uint = 0x0044;
pub const MTK_STAR_REG_RX_BASE_ADDR: c_uint = 0x0048;
// Interrupt Status Register
pub const MTK_STAR_REG_INT_STS: c_uint = 0x0050;

// Interrupt Mask Register
pub const MTK_STAR_REG_INT_MASK: c_uint = 0x0054;

// Delay-Macro Register
pub const MTK_STAR_REG_TEST0: c_uint = 0x0058;

// Misc. Config Register
pub const MTK_STAR_REG_TEST1: c_uint = 0x005c;

// Extended Configuration Register
pub const MTK_STAR_REG_EXT_CFG: c_uint = 0x0060;
pub const MTK_STAR_OFF_EXT_CFG_SND_PAUSE_RLS: c_int = 16;

pub const MTK_STAR_VAL_EXT_CFG_SND_PAUSE_RLS_1K: c_uint = 0x400;
// EthSys Configuration Register
pub const MTK_STAR_REG_SYS_CONF: c_uint = 0x0094;

// MAC Clock Configuration Register
pub const MTK_STAR_REG_MAC_CLK_CONF: c_uint = 0x00ac;

pub const MTK_STAR_BIT_CLK_DIV_10: c_uint = 0x0a;
pub const MTK_STAR_BIT_CLK_DIV_50: c_uint = 0x32;
// Counter registers.
pub const MTK_STAR_REG_C_RXOKPKT: c_uint = 0x0100;
pub const MTK_STAR_REG_C_RXOKBYTE: c_uint = 0x0104;
pub const MTK_STAR_REG_C_RXRUNT: c_uint = 0x0108;
pub const MTK_STAR_REG_C_RXLONG: c_uint = 0x010c;
pub const MTK_STAR_REG_C_RXDROP: c_uint = 0x0110;
pub const MTK_STAR_REG_C_RXCRC: c_uint = 0x0114;
pub const MTK_STAR_REG_C_RXARLDROP: c_uint = 0x0118;
pub const MTK_STAR_REG_C_RXVLANDROP: c_uint = 0x011c;
pub const MTK_STAR_REG_C_RXCSERR: c_uint = 0x0120;
pub const MTK_STAR_REG_C_RXPAUSE: c_uint = 0x0124;
pub const MTK_STAR_REG_C_TXOKPKT: c_uint = 0x0128;
pub const MTK_STAR_REG_C_TXOKBYTE: c_uint = 0x012c;
pub const MTK_STAR_REG_C_TXPAUSECOL: c_uint = 0x0130;
pub const MTK_STAR_REG_C_TXRTY: c_uint = 0x0134;
pub const MTK_STAR_REG_C_TXSKIP: c_uint = 0x0138;
pub const MTK_STAR_REG_C_TX_ARP: c_uint = 0x013c;
pub const MTK_STAR_REG_C_RX_RERR: c_uint = 0x01d8;
pub const MTK_STAR_REG_C_RX_UNI: c_uint = 0x01dc;
pub const MTK_STAR_REG_C_RX_MULTI: c_uint = 0x01e0;
pub const MTK_STAR_REG_C_RX_BROAD: c_uint = 0x01e4;
pub const MTK_STAR_REG_C_RX_ALIGNERR: c_uint = 0x01e8;
pub const MTK_STAR_REG_C_TX_UNI: c_uint = 0x01ec;
pub const MTK_STAR_REG_C_TX_MULTI: c_uint = 0x01f0;
pub const MTK_STAR_REG_C_TX_BROAD: c_uint = 0x01f4;
pub const MTK_STAR_REG_C_TX_TIMEOUT: c_uint = 0x01f8;
pub const MTK_STAR_REG_C_TX_LATECOL: c_uint = 0x01fc;
pub const MTK_STAR_REG_C_RX_LENGTHERR: c_uint = 0x0214;
pub const MTK_STAR_REG_C_RX_TWIST: c_uint = 0x0218;
// Ethernet CFG Control
pub const MTK_PERICFG_REG_NIC_CFG0_CON: c_uint = 0x03c4;
pub const MTK_PERICFG_REG_NIC_CFG1_CON: c_uint = 0x03c8;
pub const MTK_PERICFG_REG_NIC_CFG_CON_V2: c_uint = 0x0c10;

pub const MTK_PERICFG_BIT_NIC_CFG_CON_MII: c_int = 0;
pub const MTK_PERICFG_BIT_NIC_CFG_CON_RMII: c_int = 1;

// Represents the actual structure of descriptors used by the MAC. We can
// reuse the same structure for both TX and RX - the layout is the same, only
// the flags differ slightly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_star_ring_desc {
// Contains both the status flags as well as packet length.
    pub status: u32,
    pub data_ptr: u32,
    pub vtag: u32,
    pub reserved: u32,
}

// Helper structure for storing data read from/written to descriptors in order
// to limit reads from/writes to DMA memory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_star_ring_desc_data {
    pub len: c_uint,
    pub flags: c_uint,
    pub dma_addr: dma_addr_t,
    pub skb: *mut sk_buff,
}

pub const MTK_STAR_RING_NUM_DESCS: c_int = 512;

    (MTK_STAR_NUM_DESCS_TOTAL * sizeof(struct mtk_star_ring_desc))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_star_ring {
    pub descs: *mut mtk_star_ring_desc,
    pub skbs: [*mut sk_buff; MTK_STAR_RING_NUM_DESCS],
    pub dma_addrs: [dma_addr_t; MTK_STAR_RING_NUM_DESCS],
    pub head: c_uint,
    pub tail: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_star_compat {
    pub ndev): *mut *mut int (set_interface_mode)(struct net_device,
    pub bit_clk_div: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_star_priv {
    pub ndev: *mut net_device,
    pub regs: *mut regmap,
    pub pericfg: *mut regmap,
    pub clks: [clk_bulk_data; MTK_STAR_NCLKS],
    pub ring_base: *mut c_void,
    pub descs_base: *mut mtk_star_ring_desc,
    pub dma_addr: dma_addr_t,
    pub tx_ring: mtk_star_ring,
    pub rx_ring: mtk_star_ring,
    pub mii: *mut mii_bus,
    pub tx_napi: napi_struct,
    pub rx_napi: napi_struct,
    pub phy_node: *mut device_node,
    pub phy_intf: phy_interface_t,
    pub phydev: *mut phy_device,
    pub link: c_uint,
    pub speed: c_int,
    pub duplex: c_int,
    pub pause: c_int,
    pub rmii_rxc: bool,
    pub rx_inv: bool,
    pub tx_inv: bool,
    pub compat_data: *const mtk_star_compat,
// Protects against concurrent descriptor access.
    pub lock: spinlock_t,
    pub stats: rtnl_link_stats64,
}

    static struct device *mtk_star_get_dev(struct mtk_star_priv *priv)
    {
    return priv.ndev.dev.parent;
    }
    static const struct regmap_config mtk_star_regmap_config = {
    .reg_bits		= 32,
    .val_bits		= 32,
    .reg_stride		= 4,
    .disable_locking	= true,
    };
    static void mtk_star_ring_init(struct mtk_star_ring *ring,
    struct mtk_star_ring_desc *descs)
    {
    memset(ring, 0, sizeof(*ring));
    ring.descs = descs;
    ring.head = 0;
    ring.tail = 0;
    }
    static int mtk_star_ring_pop_tail(struct mtk_star_ring *ring,
    struct mtk_star_ring_desc_data *desc_data)
    {
    struct mtk_star_ring_desc *desc = &ring.descs[ring.tail];
    unsigned int status;
    status = READ_ONCE(desc.status);
    dma_rmb(); /* Make sure we read the status bits before checking it. */
    if (!(status & MTK_STAR_DESC_BIT_COWN))
    return -1;
    desc_data.len = status & MTK_STAR_DESC_MSK_LEN;
    desc_data.flags = status & ~MTK_STAR_DESC_MSK_LEN;
    desc_data.dma_addr = ring.dma_addrs[ring.tail];
    desc_data.skb = ring.skbs[ring.tail];
    ring.dma_addrs[ring.tail] = 0;
    ring.skbs[ring.tail] = core::ptr::null_mut();
    status &= MTK_STAR_DESC_BIT_COWN | MTK_STAR_DESC_BIT_EOR;
    WRITE_ONCE(desc.data_ptr, 0);
    WRITE_ONCE(desc.status, status);
    ring.tail = (ring.tail + 1) % MTK_STAR_RING_NUM_DESCS;
    return 0;
    }
    static void mtk_star_ring_push_head(struct mtk_star_ring *ring,
    struct mtk_star_ring_desc_data *desc_data,
    unsigned int flags)
    {
    struct mtk_star_ring_desc *desc = &ring.descs[ring.head];
    unsigned int status;
    status = READ_ONCE(desc.status);
    ring.skbs[ring.head] = desc_data.skb;
    ring.dma_addrs[ring.head] = desc_data.dma_addr;
    status |= desc_data.len;
    if (flags)
    status |= flags;
    WRITE_ONCE(desc.data_ptr, desc_data.dma_addr);
    WRITE_ONCE(desc.status, status);
    status &= ~MTK_STAR_DESC_BIT_COWN;
// Flush previous modifications before ownership change.
    dma_wmb();
    WRITE_ONCE(desc.status, status);
    ring.head = (ring.head + 1) % MTK_STAR_RING_NUM_DESCS;
    }
    static void
    mtk_star_ring_push_head_rx(struct mtk_star_ring *ring,
    struct mtk_star_ring_desc_data *desc_data)
    {
    mtk_star_ring_push_head(ring, desc_data, 0);
    }
    static void
    mtk_star_ring_push_head_tx(struct mtk_star_ring *ring,
    struct mtk_star_ring_desc_data *desc_data)
    {
    static const unsigned int flags = MTK_STAR_DESC_BIT_FS |
    MTK_STAR_DESC_BIT_LS |
    MTK_STAR_DESC_BIT_INT;
    mtk_star_ring_push_head(ring, desc_data, flags);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_tx_ring_avail(ring: *mut mtk_star_ring) -> c_uint {
    static unsigned int mtk_star_tx_ring_avail(struct mtk_star_ring *ring)
    {
    u32 avail;
    if (ring.tail > ring.head)
    avail = ring.tail - ring.head - 1;
    else
    avail = MTK_STAR_RING_NUM_DESCS - ring.head + ring.tail - 1;
    return avail;
    }
    static dma_addr_t mtk_star_dma_map_rx(struct mtk_star_priv *priv,
    struct sk_buff *skb)
    {
    struct device *dev = mtk_star_get_dev(priv);
// Data pointer for the RX DMA descriptor must be aligned to 4N + 2.
    return dma_map_single(dev, skb_tail_pointer(skb) - 2,
    skb_tailroom(skb), DMA_FROM_DEVICE);
    }
    static void mtk_star_dma_unmap_rx(struct mtk_star_priv *priv,
    struct mtk_star_ring_desc_data *desc_data)
    {
    struct device *dev = mtk_star_get_dev(priv);
    dma_unmap_single(dev, desc_data.dma_addr,
    skb_tailroom(desc_data.skb), DMA_FROM_DEVICE);
    }
    static dma_addr_t mtk_star_dma_map_tx(struct mtk_star_priv *priv,
    struct sk_buff *skb)
    {
    struct device *dev = mtk_star_get_dev(priv);
    return dma_map_single(dev, skb.data, skb_headlen(skb), DMA_TO_DEVICE);
    }
    static void mtk_star_dma_unmap_tx(struct mtk_star_priv *priv,
    struct mtk_star_ring_desc_data *desc_data)
    {
    struct device *dev = mtk_star_get_dev(priv);
    return dma_unmap_single(dev, desc_data.dma_addr,
    skb_headlen(desc_data.skb), DMA_TO_DEVICE);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_nic_disable_pd(priv: *mut mtk_star_priv) {
    static void mtk_star_nic_disable_pd(struct mtk_star_priv *priv)
    {
    regmap_clear_bits(priv.regs, MTK_STAR_REG_MAC_CFG,
    MTK_STAR_BIT_MAC_CFG_NIC_PD);
    }
    static void mtk_star_enable_dma_irq(struct mtk_star_priv *priv,
    bool rx, bool tx)
    {
    u32 value;
    regmap_read(priv.regs, MTK_STAR_REG_INT_MASK, &value);
    if (tx)
    value &= ~MTK_STAR_BIT_INT_STS_TNTC;
    if (rx)
    value &= ~MTK_STAR_BIT_INT_STS_FNRC;
    regmap_write(priv.regs, MTK_STAR_REG_INT_MASK, value);
    }
    static void mtk_star_disable_dma_irq(struct mtk_star_priv *priv,
    bool rx, bool tx)
    {
    u32 value;
    regmap_read(priv.regs, MTK_STAR_REG_INT_MASK, &value);
    if (tx)
    value |= MTK_STAR_BIT_INT_STS_TNTC;
    if (rx)
    value |= MTK_STAR_BIT_INT_STS_FNRC;
    regmap_write(priv.regs, MTK_STAR_REG_INT_MASK, value);
    }
// Unmask the three interrupts we care about, mask all others.
#[no_mangle]
unsafe extern "C" fn mtk_star_intr_enable(priv: *mut mtk_star_priv) {
    static void mtk_star_intr_enable(struct mtk_star_priv *priv)
    {
    unsigned int val = MTK_STAR_BIT_INT_STS_TNTC |
    MTK_STAR_BIT_INT_STS_FNRC |
    MTK_STAR_REG_INT_STS_MIB_CNT_TH;
    regmap_write(priv.regs, MTK_STAR_REG_INT_MASK, ~val);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_intr_disable(priv: *mut mtk_star_priv) {
    static void mtk_star_intr_disable(struct mtk_star_priv *priv)
    {
    regmap_write(priv.regs, MTK_STAR_REG_INT_MASK, ~0);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_intr_ack_all(priv: *mut mtk_star_priv) -> c_uint {
    static unsigned int mtk_star_intr_ack_all(struct mtk_star_priv *priv)
    {
    unsigned int val;
    regmap_read(priv.regs, MTK_STAR_REG_INT_STS, &val);
    regmap_write(priv.regs, MTK_STAR_REG_INT_STS, val);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_dma_init(priv: *mut mtk_star_priv) {
    static void mtk_star_dma_init(struct mtk_star_priv *priv)
    {
    struct mtk_star_ring_desc *desc;
    unsigned int val;
    int i;
    priv.descs_base = (struct mtk_star_ring_desc *)priv.ring_base;
    for (i = 0; i < MTK_STAR_NUM_DESCS_TOTAL; i++) {
    desc = &priv.descs_base[i];
    memset(desc, 0, sizeof(*desc));
    desc.status = MTK_STAR_DESC_BIT_COWN;
    if ((i == MTK_STAR_NUM_TX_DESCS - 1) ||
    (i == MTK_STAR_NUM_DESCS_TOTAL - 1))
    desc.status |= MTK_STAR_DESC_BIT_EOR;
    }
    mtk_star_ring_init(&priv.tx_ring, priv.descs_base);
    mtk_star_ring_init(&priv.rx_ring,
    priv.descs_base + MTK_STAR_NUM_TX_DESCS);
// Set DMA pointers.
    val = (unsigned int)priv.dma_addr;
    regmap_write(priv.regs, MTK_STAR_REG_TX_BASE_ADDR, val);
    regmap_write(priv.regs, MTK_STAR_REG_TX_DPTR, val);
    val += sizeof(struct mtk_star_ring_desc) * MTK_STAR_NUM_TX_DESCS;
    regmap_write(priv.regs, MTK_STAR_REG_RX_BASE_ADDR, val);
    regmap_write(priv.regs, MTK_STAR_REG_RX_DPTR, val);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_dma_start(priv: *mut mtk_star_priv) {
    static void mtk_star_dma_start(struct mtk_star_priv *priv)
    {
    regmap_set_bits(priv.regs, MTK_STAR_REG_TX_DMA_CTRL,
    MTK_STAR_BIT_TX_DMA_CTRL_START);
    regmap_set_bits(priv.regs, MTK_STAR_REG_RX_DMA_CTRL,
    MTK_STAR_BIT_RX_DMA_CTRL_START);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_dma_stop(priv: *mut mtk_star_priv) {
    static void mtk_star_dma_stop(struct mtk_star_priv *priv)
    {
    regmap_write(priv.regs, MTK_STAR_REG_TX_DMA_CTRL,
    MTK_STAR_BIT_TX_DMA_CTRL_STOP);
    regmap_write(priv.regs, MTK_STAR_REG_RX_DMA_CTRL,
    MTK_STAR_BIT_RX_DMA_CTRL_STOP);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_dma_disable(priv: *mut mtk_star_priv) {
    static void mtk_star_dma_disable(struct mtk_star_priv *priv)
    {
    int i;
    mtk_star_dma_stop(priv);
// Take back all descriptors.
    for (i = 0; i < MTK_STAR_NUM_DESCS_TOTAL; i++)
    priv.descs_base[i].status |= MTK_STAR_DESC_BIT_COWN;
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_dma_resume_rx(priv: *mut mtk_star_priv) {
    static void mtk_star_dma_resume_rx(struct mtk_star_priv *priv)
    {
    regmap_set_bits(priv.regs, MTK_STAR_REG_RX_DMA_CTRL,
    MTK_STAR_BIT_RX_DMA_CTRL_RESUME);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_dma_resume_tx(priv: *mut mtk_star_priv) {
    static void mtk_star_dma_resume_tx(struct mtk_star_priv *priv)
    {
    regmap_set_bits(priv.regs, MTK_STAR_REG_TX_DMA_CTRL,
    MTK_STAR_BIT_TX_DMA_CTRL_RESUME);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_set_mac_addr(ndev: *mut net_device) {
    static void mtk_star_set_mac_addr(struct net_device *ndev)
    {
    struct mtk_star_priv *priv = netdev_priv(ndev);
    const u8 *mac_addr = ndev.dev_addr;
    unsigned int high, low;
    high = mac_addr[0] << 8 | mac_addr[1] << 0;
    low = mac_addr[2] << 24 | mac_addr[3] << 16 |
    mac_addr[4] << 8 | mac_addr[5];
    regmap_write(priv.regs, MTK_STAR_REG_MY_MAC_H, high);
    regmap_write(priv.regs, MTK_STAR_REG_MY_MAC_L, low);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_reset_counters(priv: *mut mtk_star_priv) {
    static void mtk_star_reset_counters(struct mtk_star_priv *priv)
    {
    static const unsigned int counter_regs[] = {
    MTK_STAR_REG_C_RXOKPKT,
    MTK_STAR_REG_C_RXOKBYTE,
    MTK_STAR_REG_C_RXRUNT,
    MTK_STAR_REG_C_RXLONG,
    MTK_STAR_REG_C_RXDROP,
    MTK_STAR_REG_C_RXCRC,
    MTK_STAR_REG_C_RXARLDROP,
    MTK_STAR_REG_C_RXVLANDROP,
    MTK_STAR_REG_C_RXCSERR,
    MTK_STAR_REG_C_RXPAUSE,
    MTK_STAR_REG_C_TXOKPKT,
    MTK_STAR_REG_C_TXOKBYTE,
    MTK_STAR_REG_C_TXPAUSECOL,
    MTK_STAR_REG_C_TXRTY,
    MTK_STAR_REG_C_TXSKIP,
    MTK_STAR_REG_C_TX_ARP,
    MTK_STAR_REG_C_RX_RERR,
    MTK_STAR_REG_C_RX_UNI,
    MTK_STAR_REG_C_RX_MULTI,
    MTK_STAR_REG_C_RX_BROAD,
    MTK_STAR_REG_C_RX_ALIGNERR,
    MTK_STAR_REG_C_TX_UNI,
    MTK_STAR_REG_C_TX_MULTI,
    MTK_STAR_REG_C_TX_BROAD,
    MTK_STAR_REG_C_TX_TIMEOUT,
    MTK_STAR_REG_C_TX_LATECOL,
    MTK_STAR_REG_C_RX_LENGTHERR,
    MTK_STAR_REG_C_RX_TWIST,
    };
    unsigned int i, val;
    for (i = 0; i < ARRAY_SIZE(counter_regs); i++)
    regmap_read(priv.regs, counter_regs[i], &val);
    }
    static void mtk_star_update_stat(struct mtk_star_priv *priv,
    unsigned int reg, u64 *stat)
    {
    unsigned int val;
    regmap_read(priv.regs, reg, &val);
// stat += val;
    }
// Try to get as many stats as possible from the internal registers instead
// of tracking them ourselves.
//
#[no_mangle]
unsafe extern "C" fn mtk_star_update_stats(priv: *mut mtk_star_priv) {
    static void mtk_star_update_stats(struct mtk_star_priv *priv)
    {
    struct rtnl_link_stats64 *stats = &priv.stats;
// OK packets and bytes.
    mtk_star_update_stat(priv, MTK_STAR_REG_C_RXOKPKT, &stats.rx_packets);
    mtk_star_update_stat(priv, MTK_STAR_REG_C_TXOKPKT, &stats.tx_packets);
    mtk_star_update_stat(priv, MTK_STAR_REG_C_RXOKBYTE, &stats.rx_bytes);
    mtk_star_update_stat(priv, MTK_STAR_REG_C_TXOKBYTE, &stats.tx_bytes);
// RX & TX multicast.
    mtk_star_update_stat(priv, MTK_STAR_REG_C_RX_MULTI, &stats.multicast);
    mtk_star_update_stat(priv, MTK_STAR_REG_C_TX_MULTI, &stats.multicast);
// Collisions.
    mtk_star_update_stat(priv, MTK_STAR_REG_C_TXPAUSECOL,
    &stats.collisions);
    mtk_star_update_stat(priv, MTK_STAR_REG_C_TX_LATECOL,
    &stats.collisions);
    mtk_star_update_stat(priv, MTK_STAR_REG_C_RXRUNT, &stats.collisions);
// RX Errors.
    mtk_star_update_stat(priv, MTK_STAR_REG_C_RX_LENGTHERR,
    &stats.rx_length_errors);
    mtk_star_update_stat(priv, MTK_STAR_REG_C_RXLONG,
    &stats.rx_over_errors);
    mtk_star_update_stat(priv, MTK_STAR_REG_C_RXCRC, &stats.rx_crc_errors);
    mtk_star_update_stat(priv, MTK_STAR_REG_C_RX_ALIGNERR,
    &stats.rx_frame_errors);
    mtk_star_update_stat(priv, MTK_STAR_REG_C_RXDROP,
    &stats.rx_fifo_errors);
// Sum of the general RX error counter + all of the above.
    mtk_star_update_stat(priv, MTK_STAR_REG_C_RX_RERR, &stats.rx_errors);
    stats.rx_errors += stats.rx_length_errors;
    stats.rx_errors += stats.rx_over_errors;
    stats.rx_errors += stats.rx_crc_errors;
    stats.rx_errors += stats.rx_frame_errors;
    stats.rx_errors += stats.rx_fifo_errors;
    }
    static struct sk_buff *mtk_star_alloc_skb(struct net_device *ndev)
    {
    uintptr_t tail, offset;
    struct sk_buff *skb;
    skb = dev_alloc_skb(MTK_STAR_MAX_FRAME_SIZE);
    if (!skb)
    return core::ptr::null_mut();
// Align to 16 bytes.
    tail = (uintptr_t)skb_tail_pointer(skb);
    if (tail & (MTK_STAR_SKB_ALIGNMENT - 1)) {
    offset = tail & (MTK_STAR_SKB_ALIGNMENT - 1);
    skb_reserve(skb, MTK_STAR_SKB_ALIGNMENT - offset);
    }
// Ensure 16-byte alignment of the skb pointer: eth_type_trans() will
// extract the Ethernet header (14 bytes) so we need two more bytes.
//
    skb_reserve(skb, MTK_STAR_IP_ALIGN);
    return skb;
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_prepare_rx_skbs(ndev: *mut net_device) -> c_int {
    static int mtk_star_prepare_rx_skbs(struct net_device *ndev)
    {
    struct mtk_star_priv *priv = netdev_priv(ndev);
    struct mtk_star_ring *ring = &priv.rx_ring;
    struct device *dev = mtk_star_get_dev(priv);
    struct mtk_star_ring_desc *desc;
    struct sk_buff *skb;
    dma_addr_t dma_addr;
    int i;
    for (i = 0; i < MTK_STAR_NUM_RX_DESCS; i++) {
    skb = mtk_star_alloc_skb(ndev);
    if (!skb)
    return -ENOMEM;
    dma_addr = mtk_star_dma_map_rx(priv, skb);
    if (dma_mapping_error(dev, dma_addr)) {
    dev_kfree_skb(skb);
    return -ENOMEM;
    }
    desc = &ring.descs[i];
    desc.data_ptr = dma_addr;
    desc.status |= skb_tailroom(skb) & MTK_STAR_DESC_MSK_LEN;
    desc.status &= ~MTK_STAR_DESC_BIT_COWN;
    ring.skbs[i] = skb;
    ring.dma_addrs[i] = dma_addr;
    }
    return 0;
    }
    static void
    mtk_star_ring_free_skbs(struct mtk_star_priv *priv, struct mtk_star_ring *ring,
    void (*unmap_func)(struct mtk_star_priv *,
    struct mtk_star_ring_desc_data *))
    {
    struct mtk_star_ring_desc_data desc_data;
    int i;
    for (i = 0; i < MTK_STAR_RING_NUM_DESCS; i++) {
    if (!ring.dma_addrs[i])
    continue;
    desc_data.dma_addr = ring.dma_addrs[i];
    desc_data.skb = ring.skbs[i];
    unmap_func(priv, &desc_data);
    dev_kfree_skb(desc_data.skb);
    }
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_free_rx_skbs(priv: *mut mtk_star_priv) {
    static void mtk_star_free_rx_skbs(struct mtk_star_priv *priv)
    {
    struct mtk_star_ring *ring = &priv.rx_ring;
    mtk_star_ring_free_skbs(priv, ring, mtk_star_dma_unmap_rx);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_free_tx_skbs(priv: *mut mtk_star_priv) {
    static void mtk_star_free_tx_skbs(struct mtk_star_priv *priv)
    {
    struct mtk_star_ring *ring = &priv.tx_ring;
    mtk_star_ring_free_skbs(priv, ring, mtk_star_dma_unmap_tx);
    }
//
// mtk_star_handle_irq - Interrupt Handler.
// @irq: interrupt number.
// @data: pointer to a network interface device structure.
// Description : this is the driver interrupt service routine.
// it mainly handles:
// 1. tx complete interrupt for frame transmission.
// 2. rx complete interrupt for frame reception.
// 3. MAC Management Counter interrupt to avoid counter overflow.
//
#[no_mangle]
unsafe extern "C" fn mtk_star_handle_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t mtk_star_handle_irq(int irq, void *data)
    {
    struct net_device *ndev = data;
    struct mtk_star_priv *priv = netdev_priv(ndev);
    let mut intr_status: c_uint = mtk_star_intr_ack_all(priv);
    bool rx, tx;
    rx = (intr_status & MTK_STAR_BIT_INT_STS_FNRC) &&
    napi_schedule_prep(&priv.rx_napi);
    tx = (intr_status & MTK_STAR_BIT_INT_STS_TNTC) &&
    napi_schedule_prep(&priv.tx_napi);
    if (rx || tx) {
    spin_lock(&priv.lock);
// mask Rx and TX Complete interrupt
    mtk_star_disable_dma_irq(priv, rx, tx);
    spin_unlock(&priv.lock);
    if (rx)
    __napi_schedule(&priv.rx_napi);
    if (tx)
    __napi_schedule(&priv.tx_napi);
    }
// interrupt is triggered once any counters reach 0x8000000
    if (intr_status & MTK_STAR_REG_INT_STS_MIB_CNT_TH) {
    mtk_star_update_stats(priv);
    mtk_star_reset_counters(priv);
    }
    return IRQ_HANDLED;
    }
// Wait for the completion of any previous command - CMD_START bit must be
// cleared by hardware.
//
#[no_mangle]
unsafe extern "C" fn mtk_star_hash_wait_cmd_start(priv: *mut mtk_star_priv) -> c_int {
    static int mtk_star_hash_wait_cmd_start(struct mtk_star_priv *priv)
    {
    unsigned int val;
    return regmap_read_poll_timeout_atomic(priv.regs,
    MTK_STAR_REG_HASH_CTRL, val,
    !(val & MTK_STAR_BIT_HASH_CTRL_CMD_START),
    10, MTK_STAR_WAIT_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_hash_wait_ok(priv: *mut mtk_star_priv) -> c_int {
    static int mtk_star_hash_wait_ok(struct mtk_star_priv *priv)
    {
    unsigned int val;
    int ret;
// Wait for BIST_DONE bit.
    ret = regmap_read_poll_timeout_atomic(priv.regs,
    MTK_STAR_REG_HASH_CTRL, val,
    val & MTK_STAR_BIT_HASH_CTRL_BIST_DONE,
    10, MTK_STAR_WAIT_TIMEOUT);
    if (ret)
    return ret;
// Check the BIST_OK bit.
    if (!regmap_test_bits(priv.regs, MTK_STAR_REG_HASH_CTRL,
    MTK_STAR_BIT_HASH_CTRL_BIST_OK))
    return -EIO;
    return 0;
    }
    static int mtk_star_set_hashbit(struct mtk_star_priv *priv,
    unsigned int hash_addr)
    {
    unsigned int val;
    int ret;
    ret = mtk_star_hash_wait_cmd_start(priv);
    if (ret)
    return ret;
    val = hash_addr & MTK_STAR_MSK_HASH_CTRL_HASH_BIT_ADDR;
    val |= MTK_STAR_BIT_HASH_CTRL_ACC_CMD;
    val |= MTK_STAR_BIT_HASH_CTRL_CMD_START;
    val |= MTK_STAR_BIT_HASH_CTRL_BIST_EN;
    val |= MTK_STAR_BIT_HASH_CTRL_HASH_BIT_DATA;
    regmap_write(priv.regs, MTK_STAR_REG_HASH_CTRL, val);
    return mtk_star_hash_wait_ok(priv);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_reset_hash_table(priv: *mut mtk_star_priv) -> c_int {
    static int mtk_star_reset_hash_table(struct mtk_star_priv *priv)
    {
    int ret;
    ret = mtk_star_hash_wait_cmd_start(priv);
    if (ret)
    return ret;
    regmap_set_bits(priv.regs, MTK_STAR_REG_HASH_CTRL,
    MTK_STAR_BIT_HASH_CTRL_BIST_EN);
    regmap_set_bits(priv.regs, MTK_STAR_REG_TEST1,
    MTK_STAR_BIT_TEST1_RST_HASH_MBIST);
    return mtk_star_hash_wait_ok(priv);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_phy_config(priv: *mut mtk_star_priv) {
    static void mtk_star_phy_config(struct mtk_star_priv *priv)
    {
    unsigned int val;
    if (priv.speed == SPEED_1000)
    val = MTK_STAR_VAL_PHY_CTRL1_FORCE_SPD_1000M;
#[no_mangle]
pub unsafe extern "C" fn if(SPEED_100: priv->speed ==) -> else {
    else if (priv.speed == SPEED_100)
    val = MTK_STAR_VAL_PHY_CTRL1_FORCE_SPD_100M;
    else
    val = MTK_STAR_VAL_PHY_CTRL1_FORCE_SPD_10M;
    val <<= MTK_STAR_OFF_PHY_CTRL1_FORCE_SPD;
    val |= MTK_STAR_BIT_PHY_CTRL1_AN_EN;
    if (priv.pause) {
    val |= MTK_STAR_BIT_PHY_CTRL1_FORCE_FC_RX;
    val |= MTK_STAR_BIT_PHY_CTRL1_FORCE_FC_TX;
    val |= MTK_STAR_BIT_PHY_CTRL1_FORCE_DPX;
    } else {
    val &= ~MTK_STAR_BIT_PHY_CTRL1_FORCE_FC_RX;
    val &= ~MTK_STAR_BIT_PHY_CTRL1_FORCE_FC_TX;
    val &= ~MTK_STAR_BIT_PHY_CTRL1_FORCE_DPX;
    }
    regmap_write(priv.regs, MTK_STAR_REG_PHY_CTRL1, val);
    val = MTK_STAR_VAL_FC_CFG_SEND_PAUSE_TH_2K;
    val <<= MTK_STAR_OFF_FC_CFG_SEND_PAUSE_TH;
    val |= MTK_STAR_BIT_FC_CFG_UC_PAUSE_DIR;
    regmap_update_bits(priv.regs, MTK_STAR_REG_FC_CFG,
    MTK_STAR_MSK_FC_CFG_SEND_PAUSE_TH |
    MTK_STAR_BIT_FC_CFG_UC_PAUSE_DIR, val);
    val = MTK_STAR_VAL_EXT_CFG_SND_PAUSE_RLS_1K;
    val <<= MTK_STAR_OFF_EXT_CFG_SND_PAUSE_RLS;
    regmap_update_bits(priv.regs, MTK_STAR_REG_EXT_CFG,
    MTK_STAR_MSK_EXT_CFG_SND_PAUSE_RLS, val);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_adjust_link(ndev: *mut net_device) {
    static void mtk_star_adjust_link(struct net_device *ndev)
    {
    struct mtk_star_priv *priv = netdev_priv(ndev);
    struct phy_device *phydev = priv.phydev;
    let mut new_state: bool = false;
    if (phydev.link) {
    if (!priv.link) {
    priv.link = phydev.link;
    new_state = true;
    }
    if (priv.speed != phydev.speed) {
    priv.speed = phydev.speed;
    new_state = true;
    }
    if (priv.pause != phydev.pause) {
    priv.pause = phydev.pause;
    new_state = true;
    }
    } else {
    if (priv.link) {
    priv.link = phydev.link;
    new_state = true;
    }
    }
    if (new_state) {
    if (phydev.link)
    mtk_star_phy_config(priv);
    phy_print_status(ndev.phydev);
    }
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_init_config(priv: *mut mtk_star_priv) {
    static void mtk_star_init_config(struct mtk_star_priv *priv)
    {
    unsigned int val;
    val = (MTK_STAR_BIT_MII_PAD_OUT_ENABLE |
    MTK_STAR_BIT_EXT_MDC_MODE |
    MTK_STAR_BIT_SWC_MII_MODE);
    regmap_write(priv.regs, MTK_STAR_REG_SYS_CONF, val);
    regmap_update_bits(priv.regs, MTK_STAR_REG_MAC_CLK_CONF,
    MTK_STAR_MSK_MAC_CLK_CONF,
    priv.compat_data.bit_clk_div);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_enable(ndev: *mut net_device) -> c_int {
    static int mtk_star_enable(struct net_device *ndev)
    {
    struct mtk_star_priv *priv = netdev_priv(ndev);
    unsigned int val;
    int ret;
    mtk_star_nic_disable_pd(priv);
    mtk_star_intr_disable(priv);
    mtk_star_dma_stop(priv);
    mtk_star_set_mac_addr(ndev);
// Configure the MAC
    val = MTK_STAR_VAL_MAC_CFG_IPG_96BIT;
    val <<= MTK_STAR_OFF_MAC_CFG_IPG;
    val |= MTK_STAR_BIT_MAC_CFG_MAXLEN_1522;
    val |= MTK_STAR_BIT_MAC_CFG_AUTO_PAD;
    val |= MTK_STAR_BIT_MAC_CFG_CRC_STRIP;
    regmap_write(priv.regs, MTK_STAR_REG_MAC_CFG, val);
// Enable Hash Table BIST and reset it
    ret = mtk_star_reset_hash_table(priv);
    if (ret)
    return ret;
// Setup the hashing algorithm
    regmap_clear_bits(priv.regs, MTK_STAR_REG_ARL_CFG,
    MTK_STAR_BIT_ARL_CFG_HASH_ALG |
    MTK_STAR_BIT_ARL_CFG_MISC_MODE);
// Don't strip VLAN tags
    regmap_clear_bits(priv.regs, MTK_STAR_REG_MAC_CFG,
    MTK_STAR_BIT_MAC_CFG_VLAN_STRIP);
// Setup DMA
    mtk_star_dma_init(priv);
    ret = mtk_star_prepare_rx_skbs(ndev);
    if (ret)
    goto err_out;
// Request the interrupt
    ret = request_irq(ndev.irq, mtk_star_handle_irq,
    IRQF_TRIGGER_NONE, ndev.name, ndev);
    if (ret)
    goto err_free_skbs;
    napi_enable(&priv.tx_napi);
    napi_enable(&priv.rx_napi);
    mtk_star_intr_ack_all(priv);
    mtk_star_intr_enable(priv);
// Connect to and start PHY
    priv.phydev = of_phy_connect(ndev, priv.phy_node,
    mtk_star_adjust_link, 0, priv.phy_intf);
    if (!priv.phydev) {
    netdev_err(ndev, "failed to connect to PHY\n");
    ret = -ENODEV;
    goto err_free_irq;
    }
    mtk_star_dma_start(priv);
    phy_start(priv.phydev);
    netif_start_queue(ndev);
    return 0;
    err_free_irq:
    napi_disable(&priv.rx_napi);
    napi_disable(&priv.tx_napi);
    free_irq(ndev.irq, ndev);
    err_free_skbs:
    mtk_star_free_rx_skbs(priv);
    err_out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_disable(ndev: *mut net_device) {
    static void mtk_star_disable(struct net_device *ndev)
    {
    struct mtk_star_priv *priv = netdev_priv(ndev);
    netif_stop_queue(ndev);
    napi_disable(&priv.tx_napi);
    napi_disable(&priv.rx_napi);
    mtk_star_intr_disable(priv);
    mtk_star_dma_disable(priv);
    mtk_star_intr_ack_all(priv);
    phy_stop(priv.phydev);
    phy_disconnect(priv.phydev);
    free_irq(ndev.irq, ndev);
    mtk_star_free_rx_skbs(priv);
    mtk_star_free_tx_skbs(priv);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_netdev_open(ndev: *mut net_device) -> c_int {
    static int mtk_star_netdev_open(struct net_device *ndev)
    {
    return mtk_star_enable(ndev);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_netdev_stop(ndev: *mut net_device) -> c_int {
    static int mtk_star_netdev_stop(struct net_device *ndev)
    {
    mtk_star_disable(ndev);
    return 0;
    }
    static int mtk_star_netdev_ioctl(struct net_device *ndev,
    struct ifreq *req, int cmd)
    {
    if (!netif_running(ndev))
    return -EINVAL;
    return phy_mii_ioctl(ndev.phydev, req, cmd);
    }
#[no_mangle]
unsafe extern "C" fn __mtk_star_maybe_stop_tx(priv: *mut mtk_star_priv, size: u16) -> c_int {
    static int __mtk_star_maybe_stop_tx(struct mtk_star_priv *priv, u16 size)
    {
    netif_stop_queue(priv.ndev);
// Might race with mtk_star_tx_poll, check again
    smp_mb();
    if (likely(mtk_star_tx_ring_avail(&priv.tx_ring) < size))
    return -EBUSY;
    netif_start_queue(priv.ndev);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_star_maybe_stop_tx(priv: *mut mtk_star_priv, size: u16) -> c_int {
    static inline int mtk_star_maybe_stop_tx(struct mtk_star_priv *priv, u16 size)
    {
    if (likely(mtk_star_tx_ring_avail(&priv.tx_ring) >= size))
    return 0;
    return __mtk_star_maybe_stop_tx(priv, size);
    }
    static netdev_tx_t mtk_star_netdev_start_xmit(struct sk_buff *skb,
    struct net_device *ndev)
    {
    struct mtk_star_priv *priv = netdev_priv(ndev);
    struct mtk_star_ring *ring = &priv.tx_ring;
    struct device *dev = mtk_star_get_dev(priv);
    struct mtk_star_ring_desc_data desc_data;
    let mut nfrags: c_int = skb_shinfo(skb).nr_frags;
    if (unlikely(mtk_star_tx_ring_avail(ring) < nfrags + 1)) {
    if (!netif_queue_stopped(ndev)) {
    netif_stop_queue(ndev);
// This is a hard error, log it.
    pr_err_ratelimited("Tx ring full when queue awake\n");
    }
    return NETDEV_TX_BUSY;
    }
    desc_data.dma_addr = mtk_star_dma_map_tx(priv, skb);
    if (dma_mapping_error(dev, desc_data.dma_addr))
    goto err_drop_packet;
    desc_data.skb = skb;
    desc_data.len = skb.len;
    mtk_star_ring_push_head_tx(ring, &desc_data);
    netdev_sent_queue(ndev, skb.len);
    mtk_star_maybe_stop_tx(priv, MTK_STAR_DESC_NEEDED);
    mtk_star_dma_resume_tx(priv);
    return NETDEV_TX_OK;
    err_drop_packet:
    dev_kfree_skb(skb);
    ndev.stats.tx_dropped++;
    return NETDEV_TX_OK;
    }
// Returns the number of bytes sent or a negative number on the first
// descriptor owned by DMA.
//
#[no_mangle]
unsafe extern "C" fn mtk_star_tx_complete_one(priv: *mut mtk_star_priv) -> c_int {
    static int mtk_star_tx_complete_one(struct mtk_star_priv *priv)
    {
    struct mtk_star_ring *ring = &priv.tx_ring;
    struct mtk_star_ring_desc_data desc_data;
    int ret;
    ret = mtk_star_ring_pop_tail(ring, &desc_data);
    if (ret)
    return ret;
    mtk_star_dma_unmap_tx(priv, &desc_data);
    ret = desc_data.skb.len;
    dev_kfree_skb_irq(desc_data.skb);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_tx_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int mtk_star_tx_poll(struct napi_struct *napi, int budget)
    {
    struct mtk_star_priv *priv = container_of(napi, struct mtk_star_priv,
    tx_napi);
    let mut ret: c_int = 0, pkts_compl = 0, bytes_compl = 0, count = 0;
    struct mtk_star_ring *ring = &priv.tx_ring;
    struct net_device *ndev = priv.ndev;
    let mut head: c_uint = ring.head;
    let mut entry: c_uint = ring.tail;
    unsigned long flags;
    while (entry != head && count < (MTK_STAR_RING_NUM_DESCS - 1)) {
    ret = mtk_star_tx_complete_one(priv);
    if (ret < 0)
    break;
    count++;
    pkts_compl++;
    bytes_compl += ret;
    entry = ring.tail;
    }
    netdev_completed_queue(ndev, pkts_compl, bytes_compl);
    if (unlikely(netif_queue_stopped(ndev)) &&
    (mtk_star_tx_ring_avail(ring) > MTK_STAR_TX_THRESH))
    netif_wake_queue(ndev);
    if (napi_complete(napi)) {
    spin_lock_irqsave(&priv.lock, flags);
    mtk_star_enable_dma_irq(priv, false, true);
    spin_unlock_irqrestore(&priv.lock, flags);
    }
    return 0;
    }
    static void mtk_star_netdev_get_stats64(struct net_device *ndev,
    struct rtnl_link_stats64 *stats)
    {
    struct mtk_star_priv *priv = netdev_priv(ndev);
    mtk_star_update_stats(priv);
    memcpy(stats, &priv.stats, sizeof(*stats));
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_set_rx_mode(ndev: *mut net_device) {
    static void mtk_star_set_rx_mode(struct net_device *ndev)
    {
    struct mtk_star_priv *priv = netdev_priv(ndev);
    struct netdev_hw_addr *hw_addr;
    unsigned int hash_addr, i;
    int ret;
    if (ndev.flags & IFF_PROMISC) {
    regmap_set_bits(priv.regs, MTK_STAR_REG_ARL_CFG,
    MTK_STAR_BIT_ARL_CFG_MISC_MODE);
    } else if (netdev_mc_count(ndev) > MTK_STAR_HASHTABLE_MC_LIMIT ||
    ndev.flags & IFF_ALLMULTI) {
    for (i = 0; i < MTK_STAR_HASHTABLE_SIZE_MAX; i++) {
    ret = mtk_star_set_hashbit(priv, i);
    if (ret)
    goto hash_fail;
    }
    } else {
// Clear previous settings.
    ret = mtk_star_reset_hash_table(priv);
    if (ret)
    goto hash_fail;
    netdev_for_each_mc_addr(hw_addr, ndev) {
    hash_addr = (hw_addr.addr[0] & 0x01) << 8;
    hash_addr += hw_addr.addr[5];
    ret = mtk_star_set_hashbit(priv, hash_addr);
    if (ret)
    goto hash_fail;
    }
    }
    return;
    hash_fail:
    if (ret == -ETIMEDOUT)
    netdev_err(ndev, "setting hash bit timed out\n");
    else
// Should be -EIO
    netdev_err(ndev, "unable to set hash bit");
    }
    static const struct net_device_ops mtk_star_netdev_ops = {
    .ndo_open		= mtk_star_netdev_open,
    .ndo_stop		= mtk_star_netdev_stop,
    .ndo_start_xmit		= mtk_star_netdev_start_xmit,
    .ndo_get_stats64	= mtk_star_netdev_get_stats64,
    .ndo_set_rx_mode	= mtk_star_set_rx_mode,
    .ndo_eth_ioctl		= mtk_star_netdev_ioctl,
    .ndo_set_mac_address	= eth_mac_addr,
    .ndo_validate_addr	= eth_validate_addr,
    };
    static void mtk_star_get_drvinfo(struct net_device *dev,
    struct ethtool_drvinfo *info)
    {
    strscpy(info.driver, MTK_STAR_DRVNAME, sizeof(info.driver));
    }
// TODO Add ethtool stats.
    static const struct ethtool_ops mtk_star_ethtool_ops = {
    .get_drvinfo		= mtk_star_get_drvinfo,
    .get_link		= ethtool_op_get_link,
    .get_link_ksettings	= phy_ethtool_get_link_ksettings,
    .set_link_ksettings	= phy_ethtool_set_link_ksettings,
    };
#[no_mangle]
unsafe extern "C" fn mtk_star_rx(priv: *mut mtk_star_priv, budget: c_int) -> c_int {
    static int mtk_star_rx(struct mtk_star_priv *priv, int budget)
    {
    struct mtk_star_ring *ring = &priv.rx_ring;
    struct device *dev = mtk_star_get_dev(priv);
    struct mtk_star_ring_desc_data desc_data;
    struct net_device *ndev = priv.ndev;
    struct sk_buff *curr_skb, *new_skb;
    dma_addr_t new_dma_addr;
    int ret, count = 0;
    while (count < budget) {
    ret = mtk_star_ring_pop_tail(ring, &desc_data);
    if (ret)
    return -1;
    curr_skb = desc_data.skb;
    if ((desc_data.flags & MTK_STAR_DESC_BIT_RX_CRCE) ||
    (desc_data.flags & MTK_STAR_DESC_BIT_RX_OSIZE)) {
// Error packet -> drop and reuse skb.
    new_skb = curr_skb;
    goto push_new_skb;
    }
// Prepare new skb before receiving the current one.
// Reuse the current skb if we fail at any point.
//
    new_skb = mtk_star_alloc_skb(ndev);
    if (!new_skb) {
    ndev.stats.rx_dropped++;
    new_skb = curr_skb;
    goto push_new_skb;
    }
    new_dma_addr = mtk_star_dma_map_rx(priv, new_skb);
    if (dma_mapping_error(dev, new_dma_addr)) {
    ndev.stats.rx_dropped++;
    dev_kfree_skb(new_skb);
    new_skb = curr_skb;
    netdev_err(ndev, "DMA mapping error of RX descriptor\n");
    goto push_new_skb;
    }
// We can't fail anymore at this point:
// it's safe to unmap the skb.
//
    mtk_star_dma_unmap_rx(priv, &desc_data);
    skb_put(desc_data.skb, desc_data.len);
    desc_data.skb.ip_summed = CHECKSUM_NONE;
    desc_data.skb.protocol = eth_type_trans(desc_data.skb, ndev);
    desc_data.skb.dev = ndev;
    netif_receive_skb(desc_data.skb);
// update dma_addr for new skb
    desc_data.dma_addr = new_dma_addr;
    push_new_skb:
    count++;
    desc_data.len = skb_tailroom(new_skb);
    desc_data.skb = new_skb;
    mtk_star_ring_push_head_rx(ring, &desc_data);
    }
    mtk_star_dma_resume_rx(priv);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_rx_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int mtk_star_rx_poll(struct napi_struct *napi, int budget)
    {
    struct mtk_star_priv *priv;
    unsigned long flags;
    let mut work_done: c_int = 0;
    priv = container_of(napi, struct mtk_star_priv, rx_napi);
    work_done = mtk_star_rx(priv, budget);
    if (work_done < budget && napi_complete_done(napi, work_done)) {
    spin_lock_irqsave(&priv.lock, flags);
    mtk_star_enable_dma_irq(priv, true, false);
    spin_unlock_irqrestore(&priv.lock, flags);
    }
    return work_done;
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_mdio_rwok_clear(priv: *mut mtk_star_priv) {
    static void mtk_star_mdio_rwok_clear(struct mtk_star_priv *priv)
    {
    regmap_write(priv.regs, MTK_STAR_REG_PHY_CTRL0,
    MTK_STAR_BIT_PHY_CTRL0_RWOK);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_mdio_rwok_wait(priv: *mut mtk_star_priv) -> c_int {
    static int mtk_star_mdio_rwok_wait(struct mtk_star_priv *priv)
    {
    unsigned int val;
    return regmap_read_poll_timeout(priv.regs, MTK_STAR_REG_PHY_CTRL0,
    val, val & MTK_STAR_BIT_PHY_CTRL0_RWOK,
    10, MTK_STAR_WAIT_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_mdio_read(mii: *mut mii_bus, phy_id: c_int, regnum: c_int) -> c_int {
    static int mtk_star_mdio_read(struct mii_bus *mii, int phy_id, int regnum)
    {
    struct mtk_star_priv *priv = mii.priv;
    unsigned int val, data;
    int ret;
    mtk_star_mdio_rwok_clear(priv);
    val = (regnum << MTK_STAR_OFF_PHY_CTRL0_PREG);
    val &= MTK_STAR_MSK_PHY_CTRL0_PREG;
    val |= MTK_STAR_BIT_PHY_CTRL0_RDCMD;
    regmap_write(priv.regs, MTK_STAR_REG_PHY_CTRL0, val);
    ret = mtk_star_mdio_rwok_wait(priv);
    if (ret)
    return ret;
    regmap_read(priv.regs, MTK_STAR_REG_PHY_CTRL0, &data);
    data &= MTK_STAR_MSK_PHY_CTRL0_RWDATA;
    data >>= MTK_STAR_OFF_PHY_CTRL0_RWDATA;
    return data;
    }
    static int mtk_star_mdio_write(struct mii_bus *mii, int phy_id,
    int regnum, u16 data)
    {
    struct mtk_star_priv *priv = mii.priv;
    unsigned int val;
    mtk_star_mdio_rwok_clear(priv);
    val = data;
    val <<= MTK_STAR_OFF_PHY_CTRL0_RWDATA;
    val &= MTK_STAR_MSK_PHY_CTRL0_RWDATA;
    regnum <<= MTK_STAR_OFF_PHY_CTRL0_PREG;
    regnum &= MTK_STAR_MSK_PHY_CTRL0_PREG;
    val |= regnum;
    val |= MTK_STAR_BIT_PHY_CTRL0_WTCMD;
    regmap_write(priv.regs, MTK_STAR_REG_PHY_CTRL0, val);
    return mtk_star_mdio_rwok_wait(priv);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_mdio_init(ndev: *mut net_device) -> c_int {
    static int mtk_star_mdio_init(struct net_device *ndev)
    {
    struct mtk_star_priv *priv = netdev_priv(ndev);
    struct device *dev = mtk_star_get_dev(priv);
    struct device_node *of_node, *mdio_node;
    int ret;
    of_node = dev.of_node;
    mdio_node = of_get_available_child_by_name(of_node, "mdio");
    if (!mdio_node)
    return -ENODEV;
    priv.mii = devm_mdiobus_alloc(dev);
    if (!priv.mii) {
    ret = -ENOMEM;
    goto out_put_node;
    }
    snprintf(priv.mii.id, MII_BUS_ID_SIZE, "%s", dev_name(dev));
    priv.mii.name = "mtk-mac-mdio";
    priv.mii.parent = dev;
    priv.mii.read = mtk_star_mdio_read;
    priv.mii.write = mtk_star_mdio_write;
    priv.mii.priv = priv;
    ret = devm_of_mdiobus_register(dev, priv.mii, mdio_node);
    out_put_node:
    of_node_put(mdio_node);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_suspend(dev: *mut device) -> __maybe_unused int {
    static __maybe_unused int mtk_star_suspend(struct device *dev)
    {
    struct mtk_star_priv *priv;
    struct net_device *ndev;
    ndev = dev_get_drvdata(dev);
    priv = netdev_priv(ndev);
    if (netif_running(ndev))
    mtk_star_disable(ndev);
    netif_device_detach(ndev);
    clk_bulk_disable_unprepare(MTK_STAR_NCLKS, priv.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_resume(dev: *mut device) -> __maybe_unused int {
    static __maybe_unused int mtk_star_resume(struct device *dev)
    {
    struct mtk_star_priv *priv;
    struct net_device *ndev;
    int ret;
    ndev = dev_get_drvdata(dev);
    priv = netdev_priv(ndev);
    ret = clk_bulk_prepare_enable(MTK_STAR_NCLKS, priv.clks);
    if (ret)
    return ret;
    if (netif_running(ndev)) {
    ret = mtk_star_enable(ndev);
    if (ret)
    clk_bulk_disable_unprepare(MTK_STAR_NCLKS, priv.clks);
    }
    netif_device_attach(ndev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_clk_disable_unprepare(data: *mut c_void) {
    static void mtk_star_clk_disable_unprepare(void *data)
    {
    struct mtk_star_priv *priv = data;
    clk_bulk_disable_unprepare(MTK_STAR_NCLKS, priv.clks);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_set_timing(priv: *mut mtk_star_priv) -> c_int {
    static int mtk_star_set_timing(struct mtk_star_priv *priv)
    {
    struct device *dev = mtk_star_get_dev(priv);
    let mut delay_val: c_uint = 0;
    switch (priv.phy_intf) {
    case PHY_INTERFACE_MODE_MII:
    case PHY_INTERFACE_MODE_RMII:
    delay_val |= FIELD_PREP(MTK_STAR_BIT_INV_RX_CLK, priv.rx_inv);
    delay_val |= FIELD_PREP(MTK_STAR_BIT_INV_TX_CLK, priv.tx_inv);
    break;
    default:
    dev_err(dev, "This interface not supported\n");
    return -EINVAL;
    }
    return regmap_write(priv.regs, MTK_STAR_REG_TEST0, delay_val);
    }
#[no_mangle]
unsafe extern "C" fn mtk_star_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_star_probe(struct platform_device *pdev)
    {
    struct device_node *of_node;
    struct mtk_star_priv *priv;
    struct phy_device *phydev;
    struct net_device *ndev;
    struct device *dev;
    void __iomem *base;
    int ret, i;
    dev = &pdev.dev;
    of_node = dev.of_node;
    ndev = devm_alloc_etherdev(dev, sizeof(*priv));
    if (!ndev)
    return -ENOMEM;
    priv = netdev_priv(ndev);
    priv.ndev = ndev;
    priv.compat_data = of_device_get_match_data(&pdev.dev);
    SET_NETDEV_DEV(ndev, dev);
    platform_set_drvdata(pdev, ndev);
    ndev.min_mtu = ETH_ZLEN;
    ndev.max_mtu = MTK_STAR_MAX_FRAME_SIZE;
    spin_lock_init(&priv.lock);
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
// We won't be checking the return values of regmap read & write
// functions. They can only fail for mmio if there's a clock attached
// to regmap which is not the case here.
//
    priv.regs = devm_regmap_init_mmio(dev, base,
    &mtk_star_regmap_config);
    if (IS_ERR(priv.regs))
    return PTR_ERR(priv.regs);
    priv.pericfg = syscon_regmap_lookup_by_phandle(of_node,
    "mediatek,pericfg");
    if (IS_ERR(priv.pericfg)) {
    dev_err(dev, "Failed to lookup the PERICFG syscon\n");
    return PTR_ERR(priv.pericfg);
    }
    ndev.irq = platform_get_irq(pdev, 0);
    if (ndev.irq < 0)
    return ndev.irq;
    for (i = 0; i < MTK_STAR_NCLKS; i++)
    priv.clks[i].id = mtk_star_clk_names[i];
    ret = devm_clk_bulk_get(dev, MTK_STAR_NCLKS, priv.clks);
    if (ret)
    return ret;
    ret = clk_bulk_prepare_enable(MTK_STAR_NCLKS, priv.clks);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev,
    mtk_star_clk_disable_unprepare, priv);
    if (ret)
    return ret;
    ret = of_get_phy_mode(of_node, &priv.phy_intf);
    if (ret) {
    return ret;
    } else if (priv.phy_intf != PHY_INTERFACE_MODE_RMII &&
    priv.phy_intf != PHY_INTERFACE_MODE_MII) {
    dev_err(dev, "unsupported phy mode: %s\n",
    phy_modes(priv.phy_intf));
    return -EINVAL;
    }
    priv.phy_node = of_parse_phandle(of_node, "phy-handle", 0);
    if (!priv.phy_node) {
    dev_err(dev, "failed to retrieve the phy handle from device tree\n");
    return -ENODEV;
    }
    priv.rmii_rxc = of_property_read_bool(of_node, "mediatek,rmii-rxc");
    priv.rx_inv = of_property_read_bool(of_node, "mediatek,rxc-inverse");
    priv.tx_inv = of_property_read_bool(of_node, "mediatek,txc-inverse");
    if (priv.compat_data.set_interface_mode) {
    ret = priv.compat_data.set_interface_mode(ndev);
    if (ret) {
    dev_err(dev, "Failed to set phy interface, err = %d\n", ret);
    return -EINVAL;
    }
    }
    ret = mtk_star_set_timing(priv);
    if (ret) {
    dev_err(dev, "Failed to set timing, err = %d\n", ret);
    return -EINVAL;
    }
    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(32));
    if (ret) {
    dev_err(dev, "unsupported DMA mask\n");
    return ret;
    }
    priv.ring_base = dmam_alloc_coherent(dev, MTK_STAR_DMA_SIZE,
    &priv.dma_addr,
    GFP_KERNEL | GFP_DMA);
    if (!priv.ring_base)
    return -ENOMEM;
    mtk_star_nic_disable_pd(priv);
    mtk_star_init_config(priv);
    ret = mtk_star_mdio_init(ndev);
    if (ret)
    return ret;
    ret = platform_get_ethdev_address(dev, ndev);
    if (ret || !is_valid_ether_addr(ndev.dev_addr))
    eth_hw_addr_random(ndev);
    ndev.netdev_ops = &mtk_star_netdev_ops;
    ndev.ethtool_ops = &mtk_star_ethtool_ops;
    netif_napi_add(ndev, &priv.rx_napi, mtk_star_rx_poll);
    netif_napi_add_tx(ndev, &priv.tx_napi, mtk_star_tx_poll);
    phydev = of_phy_find_device(priv.phy_node);
    if (phydev) {
    phydev.mac_managed_pm = true;
    put_device(&phydev.mdio.dev);
    }
    return devm_register_netdev(dev, ndev);
    }

#[no_mangle]
unsafe extern "C" fn mt8516_set_interface_mode(ndev: *mut net_device) -> c_int {
    static int mt8516_set_interface_mode(struct net_device *ndev)
    {
    struct mtk_star_priv *priv = netdev_priv(ndev);
    struct device *dev = mtk_star_get_dev(priv);
    unsigned int intf_val, ret, rmii_rxc;
    switch (priv.phy_intf) {
    case PHY_INTERFACE_MODE_MII:
    intf_val = MTK_PERICFG_BIT_NIC_CFG_CON_MII;
    rmii_rxc = 0;
    break;
    case PHY_INTERFACE_MODE_RMII:
    intf_val = MTK_PERICFG_BIT_NIC_CFG_CON_RMII;
    rmii_rxc = priv.rmii_rxc ? 0 : MTK_PERICFG_BIT_NIC_CFG_CON_CLK;
    break;
    default:
    dev_err(dev, "This interface not supported\n");
    return -EINVAL;
    }
    ret = regmap_update_bits(priv.pericfg,
    MTK_PERICFG_REG_NIC_CFG1_CON,
    MTK_PERICFG_BIT_NIC_CFG_CON_CLK,
    rmii_rxc);
    if (ret)
    return ret;
    return regmap_update_bits(priv.pericfg,
    MTK_PERICFG_REG_NIC_CFG0_CON,
    MTK_PERICFG_REG_NIC_CFG_CON_CFG_INTF,
    intf_val);
    }
#[no_mangle]
unsafe extern "C" fn mt8365_set_interface_mode(ndev: *mut net_device) -> c_int {
    static int mt8365_set_interface_mode(struct net_device *ndev)
    {
    struct mtk_star_priv *priv = netdev_priv(ndev);
    struct device *dev = mtk_star_get_dev(priv);
    unsigned int intf_val;
    switch (priv.phy_intf) {
    case PHY_INTERFACE_MODE_MII:
    intf_val = MTK_PERICFG_BIT_NIC_CFG_CON_MII;
    break;
    case PHY_INTERFACE_MODE_RMII:
    intf_val = MTK_PERICFG_BIT_NIC_CFG_CON_RMII;
    intf_val |= priv.rmii_rxc ? 0 : MTK_PERICFG_BIT_NIC_CFG_CON_CLK_V2;
    break;
    default:
    dev_err(dev, "This interface not supported\n");
    return -EINVAL;
    }
    return regmap_update_bits(priv.pericfg,
    MTK_PERICFG_REG_NIC_CFG_CON_V2,
    MTK_PERICFG_REG_NIC_CFG_CON_CFG_INTF |
    MTK_PERICFG_BIT_NIC_CFG_CON_CLK_V2,
    intf_val);
    }
    static const struct mtk_star_compat mtk_star_mt8516_compat = {
    .set_interface_mode = mt8516_set_interface_mode,
    .bit_clk_div = MTK_STAR_BIT_CLK_DIV_10,
    };
    static const struct mtk_star_compat mtk_star_mt8365_compat = {
    .set_interface_mode = mt8365_set_interface_mode,
    .bit_clk_div = MTK_STAR_BIT_CLK_DIV_50,
    };
    static const struct of_device_id mtk_star_of_match[] = {
    { .compatible = "mediatek,mt8516-eth",
    .data = &mtk_star_mt8516_compat },
    { .compatible = "mediatek,mt8518-eth",
    .data = &mtk_star_mt8516_compat },
    { .compatible = "mediatek,mt8175-eth",
    .data = &mtk_star_mt8516_compat },
    { .compatible = "mediatek,mt8365-eth",
    .data = &mtk_star_mt8365_compat },
    { }
    };
    MODULE_DEVICE_TABLE(of, mtk_star_of_match);

    static SIMPLE_DEV_PM_OPS(mtk_star_pm_ops,
    mtk_star_suspend, mtk_star_resume);
    static struct platform_driver mtk_star_driver = {
    .driver = {
    .name = MTK_STAR_DRVNAME,
    .pm = &mtk_star_pm_ops,
    .of_match_table = of_match_ptr(mtk_star_of_match),
    },
    .probe = mtk_star_probe,
    };
    module_platform_driver(mtk_star_driver);
    MODULE_AUTHOR("Bartosz Golaszewski <bgolaszewski@baylibre.com>");
    MODULE_DESCRIPTION("Mediatek STAR Ethernet MAC Driver");
    MODULE_LICENSE("GPL");

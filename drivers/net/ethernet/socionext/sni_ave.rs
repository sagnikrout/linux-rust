//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/socionext/sni_ave.c
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
// sni_ave.c - Socionext UniPhier AVE ethernet driver
// Copyright 2014 Panasonic Corporation
// Copyright 2015-2017 Socionext Inc.
//

// General Register Group
pub const AVE_IDR: c_uint = 0x000	/* ID */;
pub const AVE_VR: c_uint = 0x004	/* Version */;
pub const AVE_GRR: c_uint = 0x008	/* Global Reset */;
pub const AVE_CFGR: c_uint = 0x00c	/* Configuration */;
// Interrupt Register Group
pub const AVE_GIMR: c_uint = 0x100	/* Global Interrupt Mask */;
pub const AVE_GISR: c_uint = 0x104	/* Global Interrupt Status */;
// MAC Register Group
pub const AVE_TXCR: c_uint = 0x200	/* TX Setup */;
pub const AVE_RXCR: c_uint = 0x204	/* RX Setup */;
pub const AVE_RXMAC1R: c_uint = 0x208	/* MAC address (lower) */;
pub const AVE_RXMAC2R: c_uint = 0x20c	/* MAC address (upper) */;
pub const AVE_MDIOCTR: c_uint = 0x214	/* MDIO Control */;
pub const AVE_MDIOAR: c_uint = 0x218	/* MDIO Address */;
pub const AVE_MDIOWDR: c_uint = 0x21c	/* MDIO Data */;
pub const AVE_MDIOSR: c_uint = 0x220	/* MDIO Status */;
pub const AVE_MDIORDR: c_uint = 0x224	/* MDIO Rd Data */;
// Descriptor Control Register Group
pub const AVE_DESCC: c_uint = 0x300	/* Descriptor Control */;
pub const AVE_TXDC: c_uint = 0x304	/* TX Descriptor Configuration */;
pub const AVE_RXDC0: c_uint = 0x308	/* RX Descriptor Ring0 Configuration */;
pub const AVE_IIRQC: c_uint = 0x34c	/* Interval IRQ Control */;
// Packet Filter Register Group
pub const AVE_PKTF_BASE: c_uint = 0x800	/* PF Base Address */;
pub const AVE_PFMBYTE_BASE: c_uint = 0xd00	/* PF Mask Byte Base Address */;
pub const AVE_PFMBIT_BASE: c_uint = 0xe00	/* PF Mask Bit Base Address */;
pub const AVE_PFSEL_BASE: c_uint = 0xf00	/* PF Selector Base Address */;
pub const AVE_PFEN: c_uint = 0xffc	/* Packet Filter Enable */;

// 64bit descriptor memory

pub const AVE_TXDM_64: c_uint = 0x1000	/* Tx Descriptor Memory */;
pub const AVE_RXDM_64: c_uint = 0x1c00	/* Rx Descriptor Memory */;
pub const AVE_TXDM_SIZE_64: c_uint = 0x0ba0	/* Tx Descriptor Memory Size 3KB */;
pub const AVE_RXDM_SIZE_64: c_uint = 0x6000	/* Rx Descriptor Memory Size 24KB */;
// 32bit descriptor memory

pub const AVE_TXDM_32: c_uint = 0x1000	/* Tx Descriptor Memory */;
pub const AVE_RXDM_32: c_uint = 0x1800	/* Rx Descriptor Memory */;
pub const AVE_TXDM_SIZE_32: c_uint = 0x07c0	/* Tx Descriptor Memory Size 2KB */;
pub const AVE_RXDM_SIZE_32: c_uint = 0x4000	/* Rx Descriptor Memory Size 16KB */;
// RMII Bridge Register Group
pub const AVE_RSTCTRL: c_uint = 0x8028	/* Reset control */;

pub const AVE_LINKSEL: c_uint = 0x8034	/* Link speed setting */;

// AVE_GRR

// AVE_CFGR

// AVE_GISR (common with GIMR)

// AVE_TXCR

// AVE_RXCR

// AVE_MDIOCTR

// AVE_MDIOSR

// AVE_DESCC

// AVE_TXDC

pub const AVE_TXDC_ADDR_START: c_int = 0;
// AVE_RXDC0

pub const AVE_RXDC0_ADDR_START: c_int = 0;
// AVE_IIRQC

// Command status for descriptor

// TX

// RX

// Packet filter

// NETIF Message control

    NETIF_MSG_PROBE  |	\
    NETIF_MSG_LINK   |	\
    NETIF_MSG_TIMER  |	\
    NETIF_MSG_IFDOWN |	\
    NETIF_MSG_IFUP   |	\
    NETIF_MSG_RX_ERR |	\
    NETIF_MSG_TX_ERR)
// Parameter for descriptor

pub const AVE_DESC_OFS_CMDSTS: c_int = 0;
pub const AVE_DESC_OFS_ADDRL: c_int = 4;
pub const AVE_DESC_OFS_ADDRU: c_int = 8;
// Parameter for ethernet frame
pub const AVE_MAX_ETHFRAME: c_int = 1518;
pub const AVE_FRAME_HEADROOM: c_int = 2;
// Parameter for interrupt
pub const AVE_INTM_COUNT: c_int = 20;
pub const AVE_FORCE_TXINTCNT: c_int = 1;
// SG
pub const SG_ETPINMODE: c_uint = 0x540;

pub const AVE_MAX_CLKS: c_int = 4;
pub const AVE_MAX_RSTS: c_int = 2;
    enum desc_id {
    AVE_DESCID_RX,
    AVE_DESCID_TX,
    };
    enum desc_state {
    AVE_DESC_RX_PERMIT,
    AVE_DESC_RX_SUSPEND,
    AVE_DESC_START,
    AVE_DESC_STOP,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ave_desc {
    pub skbs: *mut sk_buff,
    pub skbs_dma: dma_addr_t,
    pub skbs_dmalen: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ave_desc_info {
    pub /: *mut *mut u32 ndesc; / number of descriptor,
    pub /: *mut *mut u32 daddr; / start address of descriptor,
    pub /: *mut *mut u32 proc_idx; / index of processing packet,
    pub /: *mut *mut u32 done_idx; / index of processed packet,
    pub /: *mut *mut *mut ave_desc desc; / skb info related descriptor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ave_stats {
    pub syncp: u64_stats_sync,
    pub packets: u64,
    pub bytes: u64,
    pub errors: u64,
    pub dropped: u64,
    pub collisions: u64,
    pub fifo_errors: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ave_private {
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub phy_id: c_int,
    pub desc_size: c_uint,
    pub msg_enable: u32,
    pub nclks: c_int,
    pub clk: [*mut clk; AVE_MAX_CLKS],
    pub nrsts: c_int,
    pub rst: [*mut reset_control; AVE_MAX_RSTS],
    pub phy_mode: phy_interface_t,
    pub phydev: *mut phy_device,
    pub mdio: *mut mii_bus,
    pub regmap: *mut regmap,
    pub pinmode_mask: c_uint,
    pub pinmode_val: c_uint,
    pub wolopts: u32,
// stats
    pub stats_rx: ave_stats,
    pub stats_tx: ave_stats,
// NAPI support
    pub ndev: *mut net_device,
    pub napi_rx: napi_struct,
    pub napi_tx: napi_struct,
// descriptor
    pub rx: ave_desc_info,
    pub tx: ave_desc_info,
// flow control
    pub pause_auto: c_int,
    pub pause_rx: c_int,
    pub pause_tx: c_int,
    pub data: *const ave_soc_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ave_soc_data {
    pub is_desc_64bit: bool,
    pub clock_names: [*const c_char; AVE_MAX_CLKS],
    pub reset_names: [*const c_char; AVE_MAX_RSTS],
    int	(*get_pinmode)(struct ave_private *priv,
    pub arg): phy_interface_t phy_mode, u32,
}

    static u32 ave_desc_read(struct net_device *ndev, enum desc_id id, int entry,
    int offset)
    {
    struct ave_private *priv = netdev_priv(ndev);
    u32 addr;
    addr = ((id == AVE_DESCID_TX) ? priv.tx.daddr : priv.rx.daddr)
    + entry * priv.desc_size + offset;
    return readl(priv.base + addr);
    }
    static u32 ave_desc_read_cmdsts(struct net_device *ndev, enum desc_id id,
    int entry)
    {
    return ave_desc_read(ndev, id, entry, AVE_DESC_OFS_CMDSTS);
    }
    static void ave_desc_write(struct net_device *ndev, enum desc_id id,
    int entry, int offset, u32 val)
    {
    struct ave_private *priv = netdev_priv(ndev);
    u32 addr;
    addr = ((id == AVE_DESCID_TX) ? priv.tx.daddr : priv.rx.daddr)
    + entry * priv.desc_size + offset;
    writel(val, priv.base + addr);
    }
    static void ave_desc_write_cmdsts(struct net_device *ndev, enum desc_id id,
    int entry, u32 val)
    {
    ave_desc_write(ndev, id, entry, AVE_DESC_OFS_CMDSTS, val);
    }
    static void ave_desc_write_addr(struct net_device *ndev, enum desc_id id,
    int entry, dma_addr_t paddr)
    {
    struct ave_private *priv = netdev_priv(ndev);
    ave_desc_write(ndev, id, entry, AVE_DESC_OFS_ADDRL,
    lower_32_bits(paddr));
    if (IS_DESC_64BIT(priv))
    ave_desc_write(ndev, id,
    entry, AVE_DESC_OFS_ADDRU,
    upper_32_bits(paddr));
    }
#[no_mangle]
unsafe extern "C" fn ave_irq_disable_all(ndev: *mut net_device) -> u32 {
    static u32 ave_irq_disable_all(struct net_device *ndev)
    {
    struct ave_private *priv = netdev_priv(ndev);
    u32 ret;
    ret = readl(priv.base + AVE_GIMR);
    writel(0, priv.base + AVE_GIMR);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ave_irq_restore(ndev: *mut net_device, val: u32) {
    static void ave_irq_restore(struct net_device *ndev, u32 val)
    {
    struct ave_private *priv = netdev_priv(ndev);
    writel(val, priv.base + AVE_GIMR);
    }
#[no_mangle]
unsafe extern "C" fn ave_irq_enable(ndev: *mut net_device, bitflag: u32) {
    static void ave_irq_enable(struct net_device *ndev, u32 bitflag)
    {
    struct ave_private *priv = netdev_priv(ndev);
    writel(readl(priv.base + AVE_GIMR) | bitflag, priv.base + AVE_GIMR);
    writel(bitflag, priv.base + AVE_GISR);
    }
    static void ave_hw_write_macaddr(struct net_device *ndev,
    const unsigned char *mac_addr,
    int reg1, int reg2)
    {
    struct ave_private *priv = netdev_priv(ndev);
    writel(mac_addr[0] | mac_addr[1] << 8 |
    mac_addr[2] << 16 | mac_addr[3] << 24, priv.base + reg1);
    writel(mac_addr[4] | mac_addr[5] << 8, priv.base + reg2);
    }
#[no_mangle]
unsafe extern "C" fn ave_hw_read_version(ndev: *mut net_device, buf: *mut c_char, len: c_int) {
    static void ave_hw_read_version(struct net_device *ndev, char *buf, int len)
    {
    struct ave_private *priv = netdev_priv(ndev);
    u32 major, minor, vr;
    vr = readl(priv.base + AVE_VR);
    major = (vr & GENMASK(15, 8)) >> 8;
    minor = (vr & GENMASK(7, 0));
    snprintf(buf, len, "v%u.%u", major, minor);
    }
    static void ave_ethtool_get_drvinfo(struct net_device *ndev,
    struct ethtool_drvinfo *info)
    {
    struct device *dev = ndev.dev.parent;
    strscpy(info.driver, dev.driver.name, sizeof(info.driver));
    strscpy(info.bus_info, dev_name(dev), sizeof(info.bus_info));
    ave_hw_read_version(ndev, info.fw_version, sizeof(info.fw_version));
    }
#[no_mangle]
unsafe extern "C" fn ave_ethtool_get_msglevel(ndev: *mut net_device) -> u32 {
    static u32 ave_ethtool_get_msglevel(struct net_device *ndev)
    {
    struct ave_private *priv = netdev_priv(ndev);
    return priv.msg_enable;
    }
#[no_mangle]
unsafe extern "C" fn ave_ethtool_set_msglevel(ndev: *mut net_device, val: u32) {
    static void ave_ethtool_set_msglevel(struct net_device *ndev, u32 val)
    {
    struct ave_private *priv = netdev_priv(ndev);
    priv.msg_enable = val;
    }
    static void ave_ethtool_get_wol(struct net_device *ndev,
    struct ethtool_wolinfo *wol)
    {
    wol.supported = 0;
    wol.wolopts   = 0;
    if (ndev.phydev)
    phy_ethtool_get_wol(ndev.phydev, wol);
    }
    static int __ave_ethtool_set_wol(struct net_device *ndev,
    struct ethtool_wolinfo *wol)
    {
    if (!ndev.phydev ||
    (wol.wolopts & (WAKE_ARP | WAKE_MAGICSECURE)))
    return -EOPNOTSUPP;
    return phy_ethtool_set_wol(ndev.phydev, wol);
    }
    static int ave_ethtool_set_wol(struct net_device *ndev,
    struct ethtool_wolinfo *wol)
    {
    int ret;
    ret = __ave_ethtool_set_wol(ndev, wol);
    if (!ret)
    device_set_wakeup_enable(&ndev.dev, !!wol.wolopts);
    return ret;
    }
    static void ave_ethtool_get_pauseparam(struct net_device *ndev,
    struct ethtool_pauseparam *pause)
    {
    struct ave_private *priv = netdev_priv(ndev);
    pause.autoneg  = priv.pause_auto;
    pause.rx_pause = priv.pause_rx;
    pause.tx_pause = priv.pause_tx;
    }
    static int ave_ethtool_set_pauseparam(struct net_device *ndev,
    struct ethtool_pauseparam *pause)
    {
    struct ave_private *priv = netdev_priv(ndev);
    struct phy_device *phydev = ndev.phydev;
    if (!phydev)
    return -EINVAL;
    priv.pause_auto = pause.autoneg;
    priv.pause_rx   = pause.rx_pause;
    priv.pause_tx   = pause.tx_pause;
    phy_set_asym_pause(phydev, pause.rx_pause, pause.tx_pause);
    return 0;
    }
    static const struct ethtool_ops ave_ethtool_ops = {
    .get_link_ksettings	= phy_ethtool_get_link_ksettings,
    .set_link_ksettings	= phy_ethtool_set_link_ksettings,
    .get_drvinfo		= ave_ethtool_get_drvinfo,
    .nway_reset		= phy_ethtool_nway_reset,
    .get_link		= ethtool_op_get_link,
    .get_msglevel		= ave_ethtool_get_msglevel,
    .set_msglevel		= ave_ethtool_set_msglevel,
    .get_wol		= ave_ethtool_get_wol,
    .set_wol		= ave_ethtool_set_wol,
    .get_pauseparam         = ave_ethtool_get_pauseparam,
    .set_pauseparam         = ave_ethtool_set_pauseparam,
    };
#[no_mangle]
unsafe extern "C" fn ave_mdiobus_read(bus: *mut mii_bus, phyid: c_int, regnum: c_int) -> c_int {
    static int ave_mdiobus_read(struct mii_bus *bus, int phyid, int regnum)
    {
    struct net_device *ndev = bus.priv;
    struct ave_private *priv;
    u32 mdioctl, mdiosr;
    int ret;
    priv = netdev_priv(ndev);
// write address
    writel((phyid << 8) | regnum, priv.base + AVE_MDIOAR);
// read request
    mdioctl = readl(priv.base + AVE_MDIOCTR);
    writel((mdioctl | AVE_MDIOCTR_RREQ) & ~AVE_MDIOCTR_WREQ,
    priv.base + AVE_MDIOCTR);
    ret = readl_poll_timeout(priv.base + AVE_MDIOSR, mdiosr,
    !(mdiosr & AVE_MDIOSR_STS), 20, 2000);
    if (ret) {
    netdev_err(ndev, "failed to read (phy:%d reg:%x)\n",
    phyid, regnum);
    return ret;
    }
    return readl(priv.base + AVE_MDIORDR) & GENMASK(15, 0);
    }
    static int ave_mdiobus_write(struct mii_bus *bus, int phyid, int regnum,
    u16 val)
    {
    struct net_device *ndev = bus.priv;
    struct ave_private *priv;
    u32 mdioctl, mdiosr;
    int ret;
    priv = netdev_priv(ndev);
// write address
    writel((phyid << 8) | regnum, priv.base + AVE_MDIOAR);
// write data
    writel(val, priv.base + AVE_MDIOWDR);
// write request
    mdioctl = readl(priv.base + AVE_MDIOCTR);
    writel((mdioctl | AVE_MDIOCTR_WREQ) & ~AVE_MDIOCTR_RREQ,
    priv.base + AVE_MDIOCTR);
    ret = readl_poll_timeout(priv.base + AVE_MDIOSR, mdiosr,
    !(mdiosr & AVE_MDIOSR_STS), 20, 2000);
    if (ret)
    netdev_err(ndev, "failed to write (phy:%d reg:%x)\n",
    phyid, regnum);
    return ret;
    }
    static int ave_dma_map(struct net_device *ndev, struct ave_desc *desc,
    void *ptr, size_t len, enum dma_data_direction dir,
    dma_addr_t *paddr)
    {
    dma_addr_t map_addr;
    map_addr = dma_map_single(ndev.dev.parent, ptr, len, dir);
    if (unlikely(dma_mapping_error(ndev.dev.parent, map_addr)))
    return -ENOMEM;
    desc.skbs_dma = map_addr;
    desc.skbs_dmalen = len;
// paddr = map_addr;
    return 0;
    }
    static void ave_dma_unmap(struct net_device *ndev, struct ave_desc *desc,
    enum dma_data_direction dir)
    {
    if (!desc.skbs_dma)
    return;
    dma_unmap_single(ndev.dev.parent,
    desc.skbs_dma, desc.skbs_dmalen, dir);
    desc.skbs_dma = 0;
    }
// Prepare Rx descriptor and memory
#[no_mangle]
unsafe extern "C" fn ave_rxdesc_prepare(ndev: *mut net_device, entry: c_int) -> c_int {
    static int ave_rxdesc_prepare(struct net_device *ndev, int entry)
    {
    struct ave_private *priv = netdev_priv(ndev);
    struct sk_buff *skb;
    dma_addr_t paddr;
    int ret;
    skb = priv.rx.desc[entry].skbs;
    if (!skb) {
    skb = netdev_alloc_skb(ndev, AVE_MAX_ETHFRAME);
    if (!skb)
    return -ENOMEM;
    skb.data += AVE_FRAME_HEADROOM;
    skb.tail += AVE_FRAME_HEADROOM;
    }
// set disable to cmdsts
    ave_desc_write_cmdsts(ndev, AVE_DESCID_RX, entry,
    AVE_STS_INTR | AVE_STS_OWN);
// map Rx buffer
// Rx buffer set to the Rx descriptor has two restrictions:
// - Rx buffer address is 4 byte aligned.
// - Rx buffer begins with 2 byte headroom, and data will be put from
// (buffer + 2).
// To satisfy this, specify the address to put back the buffer
// pointer advanced by AVE_FRAME_HEADROOM, and expand the map size
// by AVE_FRAME_HEADROOM.
//
    ret = ave_dma_map(ndev, &priv.rx.desc[entry],
    skb.data - AVE_FRAME_HEADROOM,
    AVE_MAX_ETHFRAME + AVE_FRAME_HEADROOM,
    DMA_FROM_DEVICE, &paddr);
    if (ret) {
    netdev_err(ndev, "can't map skb for Rx\n");
    dev_kfree_skb_any(skb);
    return ret;
    }
    priv.rx.desc[entry].skbs = skb;
// set buffer pointer
    ave_desc_write_addr(ndev, AVE_DESCID_RX, entry, paddr);
// set enable to cmdsts
    ave_desc_write_cmdsts(ndev, AVE_DESCID_RX, entry,
    AVE_STS_INTR | AVE_MAX_ETHFRAME);
    return ret;
    }
// Switch state of descriptor
#[no_mangle]
unsafe extern "C" fn ave_desc_switch(ndev: *mut net_device, state: enum desc_state) -> c_int {
    static int ave_desc_switch(struct net_device *ndev, enum desc_state state)
    {
    struct ave_private *priv = netdev_priv(ndev);
    let mut ret: c_int = 0;
    u32 val;
    switch (state) {
    case AVE_DESC_START:
    writel(AVE_DESCC_TD | AVE_DESCC_RD0, priv.base + AVE_DESCC);
    break;
    case AVE_DESC_STOP:
    writel(0, priv.base + AVE_DESCC);
    if (readl_poll_timeout(priv.base + AVE_DESCC, val, !val,
    150, 15000)) {
    netdev_err(ndev, "can't stop descriptor\n");
    ret = -EBUSY;
    }
    break;
    case AVE_DESC_RX_SUSPEND:
    val = readl(priv.base + AVE_DESCC);
    val |= AVE_DESCC_RDSTP;
    val &= ~AVE_DESCC_STATUS_MASK;
    writel(val, priv.base + AVE_DESCC);
    if (readl_poll_timeout(priv.base + AVE_DESCC, val,
    val & (AVE_DESCC_RDSTP << 16),
    150, 150000)) {
    netdev_err(ndev, "can't suspend descriptor\n");
    ret = -EBUSY;
    }
    break;
    case AVE_DESC_RX_PERMIT:
    val = readl(priv.base + AVE_DESCC);
    val &= ~AVE_DESCC_RDSTP;
    val &= ~AVE_DESCC_STATUS_MASK;
    writel(val, priv.base + AVE_DESCC);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ave_tx_complete(ndev: *mut net_device) -> c_int {
    static int ave_tx_complete(struct net_device *ndev)
    {
    struct ave_private *priv = netdev_priv(ndev);
    u32 proc_idx, done_idx, ndesc, cmdsts;
    let mut nr_freebuf: c_uint = 0;
    let mut tx_packets: c_uint = 0;
    let mut tx_bytes: c_uint = 0;
    proc_idx = priv.tx.proc_idx;
    done_idx = priv.tx.done_idx;
    ndesc    = priv.tx.ndesc;
// free pre-stored skb from done_idx to proc_idx
    while (proc_idx != done_idx) {
    cmdsts = ave_desc_read_cmdsts(ndev, AVE_DESCID_TX, done_idx);
// do nothing if owner is HW (==1 for Tx)
    if (cmdsts & AVE_STS_OWN)
    break;
// check Tx status and updates statistics
    if (cmdsts & AVE_STS_OK) {
    tx_bytes += cmdsts & AVE_STS_PKTLEN_TX_MASK;
// success
    if (cmdsts & AVE_STS_LAST)
    tx_packets++;
    } else {
// error
    if (cmdsts & AVE_STS_LAST) {
    priv.stats_tx.errors++;
    if (cmdsts & (AVE_STS_OWC | AVE_STS_EC))
    priv.stats_tx.collisions++;
    }
    }
// release skb
    if (priv.tx.desc[done_idx].skbs) {
    ave_dma_unmap(ndev, &priv.tx.desc[done_idx],
    DMA_TO_DEVICE);
    dev_consume_skb_any(priv.tx.desc[done_idx].skbs);
    priv.tx.desc[done_idx].skbs = core::ptr::null_mut();
    nr_freebuf++;
    }
    done_idx = (done_idx + 1) % ndesc;
    }
    priv.tx.done_idx = done_idx;
// update stats
    u64_stats_update_begin(&priv.stats_tx.syncp);
    priv.stats_tx.packets += tx_packets;
    priv.stats_tx.bytes   += tx_bytes;
    u64_stats_update_end(&priv.stats_tx.syncp);
// wake queue for freeing buffer
    if (unlikely(netif_queue_stopped(ndev)) && nr_freebuf)
    netif_wake_queue(ndev);
    return nr_freebuf;
    }
#[no_mangle]
unsafe extern "C" fn ave_rx_receive(ndev: *mut net_device, num: c_int) -> c_int {
    static int ave_rx_receive(struct net_device *ndev, int num)
    {
    struct ave_private *priv = netdev_priv(ndev);
    let mut rx_packets: c_uint = 0;
    let mut rx_bytes: c_uint = 0;
    u32 proc_idx, done_idx;
    struct sk_buff *skb;
    unsigned int pktlen;
    int restpkt, npkts;
    u32 ndesc, cmdsts;
    proc_idx = priv.rx.proc_idx;
    done_idx = priv.rx.done_idx;
    ndesc    = priv.rx.ndesc;
    restpkt  = ((proc_idx + ndesc - 1) - done_idx) % ndesc;
    for (npkts = 0; npkts < num; npkts++) {
// we can't receive more packet, so fill desc quickly
    if (--restpkt < 0)
    break;
    cmdsts = ave_desc_read_cmdsts(ndev, AVE_DESCID_RX, proc_idx);
// do nothing if owner is HW (==0 for Rx)
    if (!(cmdsts & AVE_STS_OWN))
    break;
    if (!(cmdsts & AVE_STS_OK)) {
    priv.stats_rx.errors++;
    proc_idx = (proc_idx + 1) % ndesc;
    continue;
    }
    pktlen = cmdsts & AVE_STS_PKTLEN_RX_MASK;
// get skbuff for rx
    skb = priv.rx.desc[proc_idx].skbs;
    priv.rx.desc[proc_idx].skbs = core::ptr::null_mut();
    ave_dma_unmap(ndev, &priv.rx.desc[proc_idx], DMA_FROM_DEVICE);
    skb.dev = ndev;
    skb_put(skb, pktlen);
    skb.protocol = eth_type_trans(skb, ndev);
    if ((cmdsts & AVE_STS_CSSV) && (!(cmdsts & AVE_STS_CSER)))
    skb.ip_summed = CHECKSUM_UNNECESSARY;
    rx_packets++;
    rx_bytes += pktlen;
    netif_receive_skb(skb);
    proc_idx = (proc_idx + 1) % ndesc;
    }
    priv.rx.proc_idx = proc_idx;
// update stats
    u64_stats_update_begin(&priv.stats_rx.syncp);
    priv.stats_rx.packets += rx_packets;
    priv.stats_rx.bytes   += rx_bytes;
    u64_stats_update_end(&priv.stats_rx.syncp);
// refill the Rx buffers
    while (proc_idx != done_idx) {
    if (ave_rxdesc_prepare(ndev, done_idx))
    break;
    done_idx = (done_idx + 1) % ndesc;
    }
    priv.rx.done_idx = done_idx;
    return npkts;
    }
#[no_mangle]
unsafe extern "C" fn ave_napi_poll_rx(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int ave_napi_poll_rx(struct napi_struct *napi, int budget)
    {
    struct ave_private *priv;
    struct net_device *ndev;
    int num;
    priv = container_of(napi, struct ave_private, napi_rx);
    ndev = priv.ndev;
    num = ave_rx_receive(ndev, budget);
    if (num < budget) {
    napi_complete_done(napi, num);
// enable Rx interrupt when NAPI finishes
    ave_irq_enable(ndev, AVE_GI_RXIINT);
    }
    return num;
    }
#[no_mangle]
unsafe extern "C" fn ave_napi_poll_tx(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int ave_napi_poll_tx(struct napi_struct *napi, int budget)
    {
    struct ave_private *priv;
    struct net_device *ndev;
    int num;
    priv = container_of(napi, struct ave_private, napi_tx);
    ndev = priv.ndev;
    num = ave_tx_complete(ndev);
    napi_complete(napi);
// enable Tx interrupt when NAPI finishes
    ave_irq_enable(ndev, AVE_GI_TX);
    return num;
    }
#[no_mangle]
unsafe extern "C" fn ave_global_reset(ndev: *mut net_device) {
    static void ave_global_reset(struct net_device *ndev)
    {
    struct ave_private *priv = netdev_priv(ndev);
    u32 val;
// set config register
    val = AVE_CFGR_FLE | AVE_CFGR_IPFCEN | AVE_CFGR_CHE;
    if (!phy_interface_mode_is_rgmii(priv.phy_mode))
    val |= AVE_CFGR_MII;
    writel(val, priv.base + AVE_CFGR);
// reset RMII register
    val = readl(priv.base + AVE_RSTCTRL);
    val &= ~AVE_RSTCTRL_RMIIRST;
    writel(val, priv.base + AVE_RSTCTRL);
// assert reset
    writel(AVE_GRR_GRST | AVE_GRR_PHYRST, priv.base + AVE_GRR);
    msleep(20);
// 1st, negate PHY reset only
    writel(AVE_GRR_GRST, priv.base + AVE_GRR);
    msleep(40);
// negate reset
    writel(0, priv.base + AVE_GRR);
    msleep(40);
// negate RMII register
    val = readl(priv.base + AVE_RSTCTRL);
    val |= AVE_RSTCTRL_RMIIRST;
    writel(val, priv.base + AVE_RSTCTRL);
    ave_irq_disable_all(ndev);
    }
#[no_mangle]
unsafe extern "C" fn ave_rxfifo_reset(ndev: *mut net_device) {
    static void ave_rxfifo_reset(struct net_device *ndev)
    {
    struct ave_private *priv = netdev_priv(ndev);
    u32 rxcr_org;
// save and disable MAC receive op
    rxcr_org = readl(priv.base + AVE_RXCR);
    writel(rxcr_org & (~AVE_RXCR_RXEN), priv.base + AVE_RXCR);
// suspend Rx descriptor
    ave_desc_switch(ndev, AVE_DESC_RX_SUSPEND);
// receive all packets before descriptor starts
    ave_rx_receive(ndev, priv.rx.ndesc);
// assert reset
    writel(AVE_GRR_RXFFR, priv.base + AVE_GRR);
    udelay(50);
// negate reset
    writel(0, priv.base + AVE_GRR);
    udelay(20);
// negate interrupt status
    writel(AVE_GI_RXOVF, priv.base + AVE_GISR);
// permit descriptor
    ave_desc_switch(ndev, AVE_DESC_RX_PERMIT);
// restore MAC reccieve op
    writel(rxcr_org, priv.base + AVE_RXCR);
    }
#[no_mangle]
unsafe extern "C" fn ave_irq_handler(irq: c_int, netdev: *mut c_void) -> irqreturn_t {
    static irqreturn_t ave_irq_handler(int irq, void *netdev)
    {
    struct net_device *ndev = (struct net_device *)netdev;
    struct ave_private *priv = netdev_priv(ndev);
    u32 gimr_val, gisr_val;
    gimr_val = ave_irq_disable_all(ndev);
// get interrupt status
    gisr_val = readl(priv.base + AVE_GISR);
// PHY
    if (gisr_val & AVE_GI_PHY)
    writel(AVE_GI_PHY, priv.base + AVE_GISR);
// check exceeding packet
    if (gisr_val & AVE_GI_RXERR) {
    writel(AVE_GI_RXERR, priv.base + AVE_GISR);
    netdev_err(ndev, "receive a packet exceeding frame buffer\n");
    }
    gisr_val &= gimr_val;
    if (!gisr_val)
    goto exit_isr;
// RxFIFO overflow
    if (gisr_val & AVE_GI_RXOVF) {
    priv.stats_rx.fifo_errors++;
    ave_rxfifo_reset(ndev);
    goto exit_isr;
    }
// Rx drop
    if (gisr_val & AVE_GI_RXDROP) {
    priv.stats_rx.dropped++;
    writel(AVE_GI_RXDROP, priv.base + AVE_GISR);
    }
// Rx interval
    if (gisr_val & AVE_GI_RXIINT) {
    napi_schedule(&priv.napi_rx);
// still force to disable Rx interrupt until NAPI finishes
    gimr_val &= ~AVE_GI_RXIINT;
    }
// Tx completed
    if (gisr_val & AVE_GI_TX) {
    napi_schedule(&priv.napi_tx);
// still force to disable Tx interrupt until NAPI finishes
    gimr_val &= ~AVE_GI_TX;
    }
    exit_isr:
    ave_irq_restore(ndev, gimr_val);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ave_pfsel_start(ndev: *mut net_device, entry: c_uint) -> c_int {
    static int ave_pfsel_start(struct net_device *ndev, unsigned int entry)
    {
    struct ave_private *priv = netdev_priv(ndev);
    u32 val;
    if (WARN_ON(entry > AVE_PF_SIZE))
    return -EINVAL;
    val = readl(priv.base + AVE_PFEN);
    writel(val | BIT(entry), priv.base + AVE_PFEN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ave_pfsel_stop(ndev: *mut net_device, entry: c_uint) -> c_int {
    static int ave_pfsel_stop(struct net_device *ndev, unsigned int entry)
    {
    struct ave_private *priv = netdev_priv(ndev);
    u32 val;
    if (WARN_ON(entry > AVE_PF_SIZE))
    return -EINVAL;
    val = readl(priv.base + AVE_PFEN);
    writel(val & ~BIT(entry), priv.base + AVE_PFEN);
    return 0;
    }
    static int ave_pfsel_set_macaddr(struct net_device *ndev,
    unsigned int entry,
    const unsigned char *mac_addr,
    unsigned int set_size)
    {
    struct ave_private *priv = netdev_priv(ndev);
    if (WARN_ON(entry > AVE_PF_SIZE))
    return -EINVAL;
    if (WARN_ON(set_size > 6))
    return -EINVAL;
    ave_pfsel_stop(ndev, entry);
// set MAC address for the filter
    ave_hw_write_macaddr(ndev, mac_addr,
    AVE_PKTF(entry), AVE_PKTF(entry) + 4);
// set byte mask
    writel(GENMASK(31, set_size) & AVE_PFMBYTE_MASK0,
    priv.base + AVE_PFMBYTE(entry));
    writel(AVE_PFMBYTE_MASK1, priv.base + AVE_PFMBYTE(entry) + 4);
// set bit mask filter
    writel(AVE_PFMBIT_MASK, priv.base + AVE_PFMBIT(entry));
// set selector to ring 0
    writel(0, priv.base + AVE_PFSEL(entry));
// restart filter
    ave_pfsel_start(ndev, entry);
    return 0;
    }
    static void ave_pfsel_set_promisc(struct net_device *ndev,
    unsigned int entry, u32 rxring)
    {
    struct ave_private *priv = netdev_priv(ndev);
    if (WARN_ON(entry > AVE_PF_SIZE))
    return;
    ave_pfsel_stop(ndev, entry);
// set byte mask
    writel(AVE_PFMBYTE_MASK0, priv.base + AVE_PFMBYTE(entry));
    writel(AVE_PFMBYTE_MASK1, priv.base + AVE_PFMBYTE(entry) + 4);
// set bit mask filter
    writel(AVE_PFMBIT_MASK, priv.base + AVE_PFMBIT(entry));
// set selector to rxring
    writel(rxring, priv.base + AVE_PFSEL(entry));
    ave_pfsel_start(ndev, entry);
    }
#[no_mangle]
unsafe extern "C" fn ave_pfsel_init(ndev: *mut net_device) {
    static void ave_pfsel_init(struct net_device *ndev)
    {
    unsigned char bcast_mac[ETH_ALEN];
    int i;
    eth_broadcast_addr(bcast_mac);
    for (i = 0; i < AVE_PF_SIZE; i++)
    ave_pfsel_stop(ndev, i);
// promiscious entry, select ring 0
    ave_pfsel_set_promisc(ndev, AVE_PFNUM_FILTER, 0);
// unicast entry
    ave_pfsel_set_macaddr(ndev, AVE_PFNUM_UNICAST, ndev.dev_addr, 6);
// broadcast entry
    ave_pfsel_set_macaddr(ndev, AVE_PFNUM_BROADCAST, bcast_mac, 6);
    }
#[no_mangle]
unsafe extern "C" fn ave_phy_adjust_link(ndev: *mut net_device) {
    static void ave_phy_adjust_link(struct net_device *ndev)
    {
    struct ave_private *priv = netdev_priv(ndev);
    struct phy_device *phydev = ndev.phydev;
    u32 val, txcr, rxcr, rxcr_org;
    let mut rmt_adv: u16 = 0, lcl_adv = 0;
    u8 cap;
// set RGMII speed
    val = readl(priv.base + AVE_TXCR);
    val &= ~(AVE_TXCR_TXSPD_100 | AVE_TXCR_TXSPD_1G);
    if (phy_interface_is_rgmii(phydev) && phydev.speed == SPEED_1000)
    val |= AVE_TXCR_TXSPD_1G;
#[no_mangle]
pub unsafe extern "C" fn if(SPEED_100: phydev->speed ==) -> else {
    else if (phydev.speed == SPEED_100)
    val |= AVE_TXCR_TXSPD_100;
    writel(val, priv.base + AVE_TXCR);
// set RMII speed (100M/10M only)
    if (!phy_interface_is_rgmii(phydev)) {
    val = readl(priv.base + AVE_LINKSEL);
    if (phydev.speed == SPEED_10)
    val &= ~AVE_LINKSEL_100M;
    else
    val |= AVE_LINKSEL_100M;
    writel(val, priv.base + AVE_LINKSEL);
    }
// check current RXCR/TXCR
    rxcr = readl(priv.base + AVE_RXCR);
    txcr = readl(priv.base + AVE_TXCR);
    rxcr_org = rxcr;
    if (phydev.duplex) {
    rxcr |= AVE_RXCR_FDUPEN;
    if (phydev.pause)
    rmt_adv |= LPA_PAUSE_CAP;
    if (phydev.asym_pause)
    rmt_adv |= LPA_PAUSE_ASYM;
    lcl_adv = linkmode_adv_to_lcl_adv_t(phydev.advertising);
    cap = mii_resolve_flowctrl_fdx(lcl_adv, rmt_adv);
    if (cap & FLOW_CTRL_TX)
    txcr |= AVE_TXCR_FLOCTR;
    else
    txcr &= ~AVE_TXCR_FLOCTR;
    if (cap & FLOW_CTRL_RX)
    rxcr |= AVE_RXCR_FLOCTR;
    else
    rxcr &= ~AVE_RXCR_FLOCTR;
    } else {
    rxcr &= ~AVE_RXCR_FDUPEN;
    rxcr &= ~AVE_RXCR_FLOCTR;
    txcr &= ~AVE_TXCR_FLOCTR;
    }
    if (rxcr_org != rxcr) {
// disable Rx mac
    writel(rxcr & ~AVE_RXCR_RXEN, priv.base + AVE_RXCR);
// change and enable TX/Rx mac
    writel(txcr, priv.base + AVE_TXCR);
    writel(rxcr, priv.base + AVE_RXCR);
    }
    phy_print_status(phydev);
    }
#[no_mangle]
unsafe extern "C" fn ave_macaddr_init(ndev: *mut net_device) {
    static void ave_macaddr_init(struct net_device *ndev)
    {
    ave_hw_write_macaddr(ndev, ndev.dev_addr, AVE_RXMAC1R, AVE_RXMAC2R);
// pfsel unicast entry
    ave_pfsel_set_macaddr(ndev, AVE_PFNUM_UNICAST, ndev.dev_addr, 6);
    }
#[no_mangle]
unsafe extern "C" fn ave_init(ndev: *mut net_device) -> c_int {
    static int ave_init(struct net_device *ndev)
    {
    let mut wol: ethtool_wolinfo = { .cmd = ETHTOOL_GWOL };
    struct ave_private *priv = netdev_priv(ndev);
    struct device *dev = ndev.dev.parent;
    struct device_node *np = dev.of_node;
    struct device_node *mdio_np;
    struct phy_device *phydev;
    int nc, nr, ret;
// enable clk because of hw access until ndo_open
    for (nc = 0; nc < priv.nclks; nc++) {
    ret = clk_prepare_enable(priv.clk[nc]);
    if (ret) {
    dev_err(dev, "can't enable clock\n");
    goto out_clk_disable;
    }
    }
    for (nr = 0; nr < priv.nrsts; nr++) {
    ret = reset_control_deassert(priv.rst[nr]);
    if (ret) {
    dev_err(dev, "can't deassert reset\n");
    goto out_reset_assert;
    }
    }
    ret = regmap_update_bits(priv.regmap, SG_ETPINMODE,
    priv.pinmode_mask, priv.pinmode_val);
    if (ret)
    goto out_reset_assert;
    ave_global_reset(ndev);
    mdio_np = of_get_child_by_name(np, "mdio");
    if (!mdio_np) {
    dev_err(dev, "mdio node not found\n");
    ret = -EINVAL;
    goto out_reset_assert;
    }
    ret = of_mdiobus_register(priv.mdio, mdio_np);
    of_node_put(mdio_np);
    if (ret) {
    dev_err(dev, "failed to register mdiobus\n");
    goto out_reset_assert;
    }
    phydev = of_phy_get_and_connect(ndev, np, ave_phy_adjust_link);
    if (!phydev) {
    dev_err(dev, "could not attach to PHY\n");
    ret = -ENODEV;
    goto out_mdio_unregister;
    }
    priv.phydev = phydev;
    ave_ethtool_get_wol(ndev, &wol);
    device_set_wakeup_capable(&ndev.dev, !!wol.supported);
// set wol initial state disabled
    wol.wolopts = 0;
    __ave_ethtool_set_wol(ndev, &wol);
    if (!phy_interface_is_rgmii(phydev))
    phy_set_max_speed(phydev, SPEED_100);
    phy_support_asym_pause(phydev);
    phydev.mac_managed_pm = true;
    phy_attached_info(phydev);
    return 0;
    out_mdio_unregister:
    mdiobus_unregister(priv.mdio);
    out_reset_assert:
    while (--nr >= 0)
    reset_control_assert(priv.rst[nr]);
    out_clk_disable:
    while (--nc >= 0)
    clk_disable_unprepare(priv.clk[nc]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ave_uninit(ndev: *mut net_device) {
    static void ave_uninit(struct net_device *ndev)
    {
    struct ave_private *priv = netdev_priv(ndev);
    int i;
    phy_disconnect(priv.phydev);
    mdiobus_unregister(priv.mdio);
// disable clk because of hw access after ndo_stop
    for (i = 0; i < priv.nrsts; i++)
    reset_control_assert(priv.rst[i]);
    for (i = 0; i < priv.nclks; i++)
    clk_disable_unprepare(priv.clk[i]);
    }
#[no_mangle]
unsafe extern "C" fn ave_open(ndev: *mut net_device) -> c_int {
    static int ave_open(struct net_device *ndev)
    {
    struct ave_private *priv = netdev_priv(ndev);
    int entry;
    int ret;
    u32 val;
    ret = request_irq(priv.irq, ave_irq_handler, IRQF_SHARED, ndev.name,
    ndev);
    if (ret)
    return ret;
    priv.tx.desc = kzalloc_objs(*priv.tx.desc, priv.tx.ndesc);
    if (!priv.tx.desc) {
    ret = -ENOMEM;
    goto out_free_irq;
    }
    priv.rx.desc = kzalloc_objs(*priv.rx.desc, priv.rx.ndesc);
    if (!priv.rx.desc) {
    kfree(priv.tx.desc);
    ret = -ENOMEM;
    goto out_free_irq;
    }
// initialize Tx work and descriptor
    priv.tx.proc_idx = 0;
    priv.tx.done_idx = 0;
    for (entry = 0; entry < priv.tx.ndesc; entry++) {
    ave_desc_write_cmdsts(ndev, AVE_DESCID_TX, entry, 0);
    ave_desc_write_addr(ndev, AVE_DESCID_TX, entry, 0);
    }
    writel(AVE_TXDC_ADDR_START |
    (((priv.tx.ndesc * priv.desc_size) << 16) & AVE_TXDC_SIZE),
    priv.base + AVE_TXDC);
// initialize Rx work and descriptor
    priv.rx.proc_idx = 0;
    priv.rx.done_idx = 0;
    for (entry = 0; entry < priv.rx.ndesc; entry++) {
    if (ave_rxdesc_prepare(ndev, entry))
    break;
    }
    writel(AVE_RXDC0_ADDR_START |
    (((priv.rx.ndesc * priv.desc_size) << 16) & AVE_RXDC0_SIZE),
    priv.base + AVE_RXDC0);
    ave_desc_switch(ndev, AVE_DESC_START);
    ave_pfsel_init(ndev);
    ave_macaddr_init(ndev);
// set Rx configuration
// full duplex, enable pause drop, enalbe flow control
    val = AVE_RXCR_RXEN | AVE_RXCR_FDUPEN | AVE_RXCR_DRPEN |
    AVE_RXCR_FLOCTR | (AVE_MAX_ETHFRAME & AVE_RXCR_MPSIZ_MASK);
    writel(val, priv.base + AVE_RXCR);
// set Tx configuration
// enable flow control, disable loopback
    writel(AVE_TXCR_FLOCTR, priv.base + AVE_TXCR);
// enable timer, clear EN,INTM, and mask interval unit(BSCK)
    val = readl(priv.base + AVE_IIRQC) & AVE_IIRQC_BSCK;
    val |= AVE_IIRQC_EN0 | (AVE_INTM_COUNT << 16);
    writel(val, priv.base + AVE_IIRQC);
    val = AVE_GI_RXIINT | AVE_GI_RXOVF | AVE_GI_TX | AVE_GI_RXDROP;
    ave_irq_restore(ndev, val);
    napi_enable(&priv.napi_rx);
    napi_enable(&priv.napi_tx);
    phy_start(ndev.phydev);
    phy_start_aneg(ndev.phydev);
    netif_start_queue(ndev);
    return 0;
    out_free_irq:
    disable_irq(priv.irq);
    free_irq(priv.irq, ndev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ave_stop(ndev: *mut net_device) -> c_int {
    static int ave_stop(struct net_device *ndev)
    {
    struct ave_private *priv = netdev_priv(ndev);
    int entry;
    ave_irq_disable_all(ndev);
    disable_irq(priv.irq);
    free_irq(priv.irq, ndev);
    netif_tx_disable(ndev);
    phy_stop(ndev.phydev);
    napi_disable(&priv.napi_tx);
    napi_disable(&priv.napi_rx);
    ave_desc_switch(ndev, AVE_DESC_STOP);
// free Tx buffer
    for (entry = 0; entry < priv.tx.ndesc; entry++) {
    if (!priv.tx.desc[entry].skbs)
    continue;
    ave_dma_unmap(ndev, &priv.tx.desc[entry], DMA_TO_DEVICE);
    dev_kfree_skb_any(priv.tx.desc[entry].skbs);
    priv.tx.desc[entry].skbs = core::ptr::null_mut();
    }
    priv.tx.proc_idx = 0;
    priv.tx.done_idx = 0;
// free Rx buffer
    for (entry = 0; entry < priv.rx.ndesc; entry++) {
    if (!priv.rx.desc[entry].skbs)
    continue;
    ave_dma_unmap(ndev, &priv.rx.desc[entry], DMA_FROM_DEVICE);
    dev_kfree_skb_any(priv.rx.desc[entry].skbs);
    priv.rx.desc[entry].skbs = core::ptr::null_mut();
    }
    priv.rx.proc_idx = 0;
    priv.rx.done_idx = 0;
    kfree(priv.tx.desc);
    kfree(priv.rx.desc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ave_start_xmit(skb: *mut sk_buff, ndev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t ave_start_xmit(struct sk_buff *skb, struct net_device *ndev)
    {
    struct ave_private *priv = netdev_priv(ndev);
    u32 proc_idx, done_idx, ndesc, cmdsts;
    int ret, freepkt;
    dma_addr_t paddr;
    proc_idx = priv.tx.proc_idx;
    done_idx = priv.tx.done_idx;
    ndesc = priv.tx.ndesc;
    freepkt = ((done_idx + ndesc - 1) - proc_idx) % ndesc;
// stop queue when not enough entry
    if (unlikely(freepkt < 1)) {
    netif_stop_queue(ndev);
    return NETDEV_TX_BUSY;
    }
// add padding for short packet
    if (skb_put_padto(skb, ETH_ZLEN)) {
    priv.stats_tx.dropped++;
    return NETDEV_TX_OK;
    }
// map Tx buffer
// Tx buffer set to the Tx descriptor doesn't have any restriction.
//
    ret = ave_dma_map(ndev, &priv.tx.desc[proc_idx],
    skb.data, skb.len, DMA_TO_DEVICE, &paddr);
    if (ret) {
    dev_kfree_skb_any(skb);
    priv.stats_tx.dropped++;
    return NETDEV_TX_OK;
    }
    priv.tx.desc[proc_idx].skbs = skb;
    ave_desc_write_addr(ndev, AVE_DESCID_TX, proc_idx, paddr);
    cmdsts = AVE_STS_OWN | AVE_STS_1ST | AVE_STS_LAST |
    (skb.len & AVE_STS_PKTLEN_TX_MASK);
// set interrupt per AVE_FORCE_TXINTCNT or when queue is stopped
    if (!(proc_idx % AVE_FORCE_TXINTCNT) || netif_queue_stopped(ndev))
    cmdsts |= AVE_STS_INTR;
// disable checksum calculation when skb doesn't calurate checksum
    if (skb.ip_summed == CHECKSUM_NONE ||
    skb.ip_summed == CHECKSUM_UNNECESSARY)
    cmdsts |= AVE_STS_NOCSUM;
    ave_desc_write_cmdsts(ndev, AVE_DESCID_TX, proc_idx, cmdsts);
    priv.tx.proc_idx = (proc_idx + 1) % ndesc;
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn ave_ioctl(ndev: *mut net_device, ifr: *mut ifreq, cmd: c_int) -> c_int {
    static int ave_ioctl(struct net_device *ndev, struct ifreq *ifr, int cmd)
    {
    return phy_mii_ioctl(ndev.phydev, ifr, cmd);
    }
    static const u8 v4multi_macadr[] = { 0x01, 0x00, 0x00, 0x00, 0x00, 0x00 };
    static const u8 v6multi_macadr[] = { 0x33, 0x00, 0x00, 0x00, 0x00, 0x00 };
#[no_mangle]
unsafe extern "C" fn ave_set_rx_mode(ndev: *mut net_device) {
    static void ave_set_rx_mode(struct net_device *ndev)
    {
    struct ave_private *priv = netdev_priv(ndev);
    struct netdev_hw_addr *hw_adr;
    int count, mc_cnt;
    u32 val;
// MAC addr filter enable for promiscious mode
    mc_cnt = netdev_mc_count(ndev);
    val = readl(priv.base + AVE_RXCR);
    if (ndev.flags & IFF_PROMISC || !mc_cnt)
    val &= ~AVE_RXCR_AFEN;
    else
    val |= AVE_RXCR_AFEN;
    writel(val, priv.base + AVE_RXCR);
// set all multicast address
    if ((ndev.flags & IFF_ALLMULTI) || mc_cnt > AVE_PF_MULTICAST_SIZE) {
    ave_pfsel_set_macaddr(ndev, AVE_PFNUM_MULTICAST,
    v4multi_macadr, 1);
    ave_pfsel_set_macaddr(ndev, AVE_PFNUM_MULTICAST + 1,
    v6multi_macadr, 1);
    } else {
// stop all multicast filter
    for (count = 0; count < AVE_PF_MULTICAST_SIZE; count++)
    ave_pfsel_stop(ndev, AVE_PFNUM_MULTICAST + count);
// set multicast addresses
    count = 0;
    netdev_for_each_mc_addr(hw_adr, ndev) {
    if (count == mc_cnt)
    break;
    ave_pfsel_set_macaddr(ndev, AVE_PFNUM_MULTICAST + count,
    hw_adr.addr, 6);
    count++;
    }
    }
    }
    static void ave_get_stats64(struct net_device *ndev,
    struct rtnl_link_stats64 *stats)
    {
    struct ave_private *priv = netdev_priv(ndev);
    unsigned int start;
    do {
    start = u64_stats_fetch_begin(&priv.stats_rx.syncp);
    stats.rx_packets = priv.stats_rx.packets;
    stats.rx_bytes	  = priv.stats_rx.bytes;
    } while (u64_stats_fetch_retry(&priv.stats_rx.syncp, start));
    do {
    start = u64_stats_fetch_begin(&priv.stats_tx.syncp);
    stats.tx_packets = priv.stats_tx.packets;
    stats.tx_bytes	  = priv.stats_tx.bytes;
    } while (u64_stats_fetch_retry(&priv.stats_tx.syncp, start));
    stats.rx_errors      = priv.stats_rx.errors;
    stats.tx_errors      = priv.stats_tx.errors;
    stats.rx_dropped     = priv.stats_rx.dropped;
    stats.tx_dropped     = priv.stats_tx.dropped;
    stats.rx_fifo_errors = priv.stats_rx.fifo_errors;
    stats.collisions     = priv.stats_tx.collisions;
    }
#[no_mangle]
unsafe extern "C" fn ave_set_mac_address(ndev: *mut net_device, p: *mut c_void) -> c_int {
    static int ave_set_mac_address(struct net_device *ndev, void *p)
    {
    let mut ret: c_int = eth_mac_addr(ndev, p);
    if (ret)
    return ret;
    ave_macaddr_init(ndev);
    return 0;
    }
    static const struct net_device_ops ave_netdev_ops = {
    .ndo_init		= ave_init,
    .ndo_uninit		= ave_uninit,
    .ndo_open		= ave_open,
    .ndo_stop		= ave_stop,
    .ndo_start_xmit		= ave_start_xmit,
    .ndo_eth_ioctl		= ave_ioctl,
    .ndo_set_rx_mode	= ave_set_rx_mode,
    .ndo_get_stats64	= ave_get_stats64,
    .ndo_set_mac_address	= ave_set_mac_address,
    };
#[no_mangle]
unsafe extern "C" fn ave_probe(pdev: *mut platform_device) -> c_int {
    static int ave_probe(struct platform_device *pdev)
    {
    const struct ave_soc_data *data;
    struct device *dev = &pdev.dev;
    char buf[ETHTOOL_FWVERS_LEN];
    struct of_phandle_args args;
    phy_interface_t phy_mode;
    struct ave_private *priv;
    struct net_device *ndev;
    struct device_node *np;
    void __iomem *base;
    const char *name;
    int i, irq, ret;
    u64 dma_mask;
    u32 ave_id;
    data = of_device_get_match_data(dev);
    if (WARN_ON(!data))
    return -EINVAL;
    np = dev.of_node;
    ret = of_get_phy_mode(np, &phy_mode);
    if (ret) {
    dev_err(dev, "phy-mode not found\n");
    return ret;
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    ndev = devm_alloc_etherdev(dev, sizeof(struct ave_private));
    if (!ndev) {
    dev_err(dev, "can't allocate ethernet device\n");
    return -ENOMEM;
    }
    ndev.netdev_ops = &ave_netdev_ops;
    ndev.ethtool_ops = &ave_ethtool_ops;
    SET_NETDEV_DEV(ndev, dev);
    ndev.features    |= (NETIF_F_IP_CSUM | NETIF_F_RXCSUM);
    ndev.hw_features |= (NETIF_F_IP_CSUM | NETIF_F_RXCSUM);
    ndev.max_mtu = AVE_MAX_ETHFRAME - (ETH_HLEN + ETH_FCS_LEN);
    ret = of_get_ethdev_address(np, ndev);
    if (ret) {
// if the mac address is invalid, use random mac address
    eth_hw_addr_random(ndev);
    dev_warn(dev, "Using random MAC address: %pM\n",
    ndev.dev_addr);
    }
    priv = netdev_priv(ndev);
    priv.base = base;
    priv.irq = irq;
    priv.ndev = ndev;
    priv.msg_enable = netif_msg_init(-1, AVE_DEFAULT_MSG_ENABLE);
    priv.phy_mode = phy_mode;
    priv.data = data;
    if (IS_DESC_64BIT(priv)) {
    priv.desc_size = AVE_DESC_SIZE_64;
    priv.tx.daddr  = AVE_TXDM_64;
    priv.rx.daddr  = AVE_RXDM_64;
    dma_mask = DMA_BIT_MASK(64);
    } else {
    priv.desc_size = AVE_DESC_SIZE_32;
    priv.tx.daddr  = AVE_TXDM_32;
    priv.rx.daddr  = AVE_RXDM_32;
    dma_mask = DMA_BIT_MASK(32);
    }
    ret = dma_set_mask(dev, dma_mask);
    if (ret)
    return ret;
    priv.tx.ndesc = AVE_NR_TXDESC;
    priv.rx.ndesc = AVE_NR_RXDESC;
    u64_stats_init(&priv.stats_tx.syncp);
    u64_stats_init(&priv.stats_rx.syncp);
    for (i = 0; i < AVE_MAX_CLKS; i++) {
    name = priv.data.clock_names[i];
    if (!name)
    break;
    priv.clk[i] = devm_clk_get(dev, name);
    if (IS_ERR(priv.clk[i]))
    return PTR_ERR(priv.clk[i]);
    priv.nclks++;
    }
    for (i = 0; i < AVE_MAX_RSTS; i++) {
    name = priv.data.reset_names[i];
    if (!name)
    break;
    priv.rst[i] = devm_reset_control_get_shared(dev, name);
    if (IS_ERR(priv.rst[i]))
    return PTR_ERR(priv.rst[i]);
    priv.nrsts++;
    }
    ret = of_parse_phandle_with_fixed_args(np,
    "socionext,syscon-phy-mode",
    1, 0, &args);
    if (ret) {
    dev_err(dev, "can't get syscon-phy-mode property\n");
    return ret;
    }
    priv.regmap = syscon_node_to_regmap(args.np);
    of_node_put(args.np);
    if (IS_ERR(priv.regmap)) {
    dev_err(dev, "can't map syscon-phy-mode\n");
    return PTR_ERR(priv.regmap);
    }
    ret = priv.data.get_pinmode(priv, phy_mode, args.args[0]);
    if (ret) {
    dev_err(dev, "invalid phy-mode setting\n");
    return ret;
    }
    priv.mdio = devm_mdiobus_alloc(dev);
    if (!priv.mdio)
    return -ENOMEM;
    priv.mdio.priv = ndev;
    priv.mdio.parent = dev;
    priv.mdio.read = ave_mdiobus_read;
    priv.mdio.write = ave_mdiobus_write;
    priv.mdio.name = "uniphier-mdio";
    snprintf(priv.mdio.id, MII_BUS_ID_SIZE, "%s-%x",
    pdev.name, pdev.id);
// Register as a NAPI supported driver
    netif_napi_add(ndev, &priv.napi_rx, ave_napi_poll_rx);
    netif_napi_add_tx(ndev, &priv.napi_tx, ave_napi_poll_tx);
    platform_set_drvdata(pdev, ndev);
    ret = register_netdev(ndev);
    if (ret) {
    dev_err(dev, "failed to register netdevice\n");
    goto out_del_napi;
    }
// get ID and version
    ave_id = readl(priv.base + AVE_IDR);
    ave_hw_read_version(ndev, buf, sizeof(buf));
    dev_info(dev, "Socionext %c%c%c%c Ethernet IP %s (irq=%d, phy=%s)\n",
    (ave_id >> 24) & 0xff, (ave_id >> 16) & 0xff,
    (ave_id >> 8) & 0xff, (ave_id >> 0) & 0xff,
    buf, priv.irq, phy_modes(phy_mode));
    return 0;
    out_del_napi:
    netif_napi_del(&priv.napi_rx);
    netif_napi_del(&priv.napi_tx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ave_remove(pdev: *mut platform_device) {
    static void ave_remove(struct platform_device *pdev)
    {
    struct net_device *ndev = platform_get_drvdata(pdev);
    struct ave_private *priv = netdev_priv(ndev);
    unregister_netdev(ndev);
    netif_napi_del(&priv.napi_rx);
    netif_napi_del(&priv.napi_tx);
    }

#[no_mangle]
unsafe extern "C" fn ave_suspend(dev: *mut device) -> c_int {
    static int ave_suspend(struct device *dev)
    {
    let mut wol: ethtool_wolinfo = { .cmd = ETHTOOL_GWOL };
    struct net_device *ndev = dev_get_drvdata(dev);
    struct ave_private *priv = netdev_priv(ndev);
    let mut ret: c_int = 0;
    if (netif_running(ndev)) {
    ret = ave_stop(ndev);
    netif_device_detach(ndev);
    }
    ave_ethtool_get_wol(ndev, &wol);
    priv.wolopts = wol.wolopts;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ave_resume(dev: *mut device) -> c_int {
    static int ave_resume(struct device *dev)
    {
    let mut wol: ethtool_wolinfo = { .cmd = ETHTOOL_GWOL };
    struct net_device *ndev = dev_get_drvdata(dev);
    struct ave_private *priv = netdev_priv(ndev);
    let mut ret: c_int = 0;
    ave_global_reset(ndev);
    ret = phy_init_hw(ndev.phydev);
    if (ret)
    return ret;
    ave_ethtool_get_wol(ndev, &wol);
    wol.wolopts = priv.wolopts;
    __ave_ethtool_set_wol(ndev, &wol);
    if (netif_running(ndev)) {
    ret = ave_open(ndev);
    netif_device_attach(ndev);
    }
    return ret;
    }
    static SIMPLE_DEV_PM_OPS(ave_pm_ops, ave_suspend, ave_resume);

    static int ave_pro4_get_pinmode(struct ave_private *priv,
    phy_interface_t phy_mode, u32 arg)
    {
    if (arg > 0)
    return -EINVAL;
    priv.pinmode_mask = SG_ETPINMODE_RMII(0);
    switch (phy_mode) {
    case PHY_INTERFACE_MODE_RMII:
    priv.pinmode_val = SG_ETPINMODE_RMII(0);
    break;
    case PHY_INTERFACE_MODE_MII:
    case PHY_INTERFACE_MODE_RGMII:
    case PHY_INTERFACE_MODE_RGMII_ID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    case PHY_INTERFACE_MODE_RGMII_TXID:
    priv.pinmode_val = 0;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int ave_ld11_get_pinmode(struct ave_private *priv,
    phy_interface_t phy_mode, u32 arg)
    {
    if (arg > 0)
    return -EINVAL;
    priv.pinmode_mask = SG_ETPINMODE_EXTPHY | SG_ETPINMODE_RMII(0);
    switch (phy_mode) {
    case PHY_INTERFACE_MODE_INTERNAL:
    priv.pinmode_val = 0;
    break;
    case PHY_INTERFACE_MODE_RMII:
    priv.pinmode_val = SG_ETPINMODE_EXTPHY | SG_ETPINMODE_RMII(0);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int ave_ld20_get_pinmode(struct ave_private *priv,
    phy_interface_t phy_mode, u32 arg)
    {
    if (arg > 0)
    return -EINVAL;
    priv.pinmode_mask = SG_ETPINMODE_RMII(0);
    switch (phy_mode) {
    case PHY_INTERFACE_MODE_RMII:
    priv.pinmode_val = SG_ETPINMODE_RMII(0);
    break;
    case PHY_INTERFACE_MODE_RGMII:
    case PHY_INTERFACE_MODE_RGMII_ID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    case PHY_INTERFACE_MODE_RGMII_TXID:
    priv.pinmode_val = 0;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int ave_pxs3_get_pinmode(struct ave_private *priv,
    phy_interface_t phy_mode, u32 arg)
    {
    if (arg > 1)
    return -EINVAL;
    priv.pinmode_mask = SG_ETPINMODE_RMII(arg);
    switch (phy_mode) {
    case PHY_INTERFACE_MODE_RMII:
    priv.pinmode_val = SG_ETPINMODE_RMII(arg);
    break;
    case PHY_INTERFACE_MODE_RGMII:
    case PHY_INTERFACE_MODE_RGMII_ID:
    case PHY_INTERFACE_MODE_RGMII_RXID:
    case PHY_INTERFACE_MODE_RGMII_TXID:
    priv.pinmode_val = 0;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct ave_soc_data ave_pro4_data = {
    .is_desc_64bit = false,
    .clock_names = {
    "gio", "ether", "ether-gb", "ether-phy",
    },
    .reset_names = {
    "gio", "ether",
    },
    .get_pinmode = ave_pro4_get_pinmode,
    };
    static const struct ave_soc_data ave_pxs2_data = {
    .is_desc_64bit = false,
    .clock_names = {
    "ether",
    },
    .reset_names = {
    "ether",
    },
    .get_pinmode = ave_pro4_get_pinmode,
    };
    static const struct ave_soc_data ave_ld11_data = {
    .is_desc_64bit = false,
    .clock_names = {
    "ether",
    },
    .reset_names = {
    "ether",
    },
    .get_pinmode = ave_ld11_get_pinmode,
    };
    static const struct ave_soc_data ave_ld20_data = {
    .is_desc_64bit = true,
    .clock_names = {
    "ether",
    },
    .reset_names = {
    "ether",
    },
    .get_pinmode = ave_ld20_get_pinmode,
    };
    static const struct ave_soc_data ave_pxs3_data = {
    .is_desc_64bit = false,
    .clock_names = {
    "ether",
    },
    .reset_names = {
    "ether",
    },
    .get_pinmode = ave_pxs3_get_pinmode,
    };
    static const struct ave_soc_data ave_nx1_data = {
    .is_desc_64bit = true,
    .clock_names = {
    "ether",
    },
    .reset_names = {
    "ether",
    },
    .get_pinmode = ave_pxs3_get_pinmode,
    };
    static const struct of_device_id of_ave_match[] = {
    {
    .compatible = "socionext,uniphier-pro4-ave4",
    .data = &ave_pro4_data,
    },
    {
    .compatible = "socionext,uniphier-pxs2-ave4",
    .data = &ave_pxs2_data,
    },
    {
    .compatible = "socionext,uniphier-ld11-ave4",
    .data = &ave_ld11_data,
    },
    {
    .compatible = "socionext,uniphier-ld20-ave4",
    .data = &ave_ld20_data,
    },
    {
    .compatible = "socionext,uniphier-pxs3-ave4",
    .data = &ave_pxs3_data,
    },
    {
    .compatible = "socionext,uniphier-nx1-ave4",
    .data = &ave_nx1_data,
    },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_ave_match);
    static struct platform_driver ave_driver = {
    .probe  = ave_probe,
    .remove = ave_remove,
    .driver	= {
    .name = "ave",
    .pm   = AVE_PM_OPS,
    .of_match_table	= of_ave_match,
    },
    };
    module_platform_driver(ave_driver);
    MODULE_AUTHOR("Kunihiko Hayashi <hayashi.kunihiko@socionext.com>");
    MODULE_DESCRIPTION("Socionext UniPhier AVE ethernet driver");
    MODULE_LICENSE("GPL v2");

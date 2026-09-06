//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/xscale/ixp4xx_eth.c
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
// Intel IXP4xx Ethernet driver for Linux
//
// Copyright (C) 2007 Krzysztof Halasa <khc@pm.waw.pl>
//
// Ethernet port config (0x00 is not present on IXP42X):
//
// logical port		0x00		0x10		0x20
// NPE			0 (NPE-A)	1 (NPE-B)	2 (NPE-C)
// physical PortId	2		0		1
// TX queue		23		24		25
// RX-free queue	26		27		28
// TX-done queue is always 31, per-port RX and TX-ready queues are configurable
//
// Queue entries:
// bits 0 -> 1	- NPE ID (RX and TX-done)
// bits 0 -> 2	- priority (TX, per 802.1D)
// bits 3 -> 4	- port ID (user-set?)
// bits 5 -> 31	- physical descriptor address
//

pub const IXP4XX_ETH_NPEA: c_uint = 0x00;
pub const IXP4XX_ETH_NPEB: c_uint = 0x10;
pub const IXP4XX_ETH_NPEC: c_uint = 0x20;

pub const DEBUG_DESC: c_int = 0;
pub const DEBUG_RX: c_int = 0;
pub const DEBUG_TX: c_int = 0;
pub const DEBUG_PKT_BYTES: c_int = 0;
pub const DEBUG_MDIO: c_int = 0;
pub const DEBUG_CLOSE: c_int = 0;

pub const MAX_NPES: c_int = 3;

pub const REGS_SIZE: c_uint = 0x1000;
// MRU is said to be 14320 in a code dump, the SW manual says that
// MRU/MTU is 16320 and includes VLAN and ethernet headers.
// See "IXP400 Software Programmer's Guide" section 10.3.2, page 161.
//
// FIXME: we have chosen the safe default (14320) but if you can test
// jumboframes, experiment with 16320 and see what happens!
//

pub const NAPI_WEIGHT: c_int = 16;

pub const TXDONE_QUEUE: c_int = 31;
pub const PTP_SLAVE_MODE: c_int = 1;
pub const PTP_MASTER_MODE: c_int = 2;

// TX Control Registers
pub const TX_CNTRL0_TX_EN: c_uint = 0x01;
pub const TX_CNTRL0_HALFDUPLEX: c_uint = 0x02;
pub const TX_CNTRL0_RETRY: c_uint = 0x04;
pub const TX_CNTRL0_PAD_EN: c_uint = 0x08;
pub const TX_CNTRL0_APPEND_FCS: c_uint = 0x10;
pub const TX_CNTRL0_2DEFER: c_uint = 0x20;
pub const TX_CNTRL0_RMII: c_uint = 0x40 /* reduced MII */;
pub const TX_CNTRL1_RETRIES: c_uint = 0x0F /* 4 bits */;
// RX Control Registers
pub const RX_CNTRL0_RX_EN: c_uint = 0x01;
pub const RX_CNTRL0_PADSTRIP_EN: c_uint = 0x02;
pub const RX_CNTRL0_SEND_FCS: c_uint = 0x04;
pub const RX_CNTRL0_PAUSE_EN: c_uint = 0x08;
pub const RX_CNTRL0_LOOP_EN: c_uint = 0x10;
pub const RX_CNTRL0_ADDR_FLTR_EN: c_uint = 0x20;
pub const RX_CNTRL0_RX_RUNT_EN: c_uint = 0x40;
pub const RX_CNTRL0_BCAST_DIS: c_uint = 0x80;
pub const RX_CNTRL1_DEFER_EN: c_uint = 0x01;
// Core Control Register
pub const CORE_RESET: c_uint = 0x01;
pub const CORE_RX_FIFO_FLUSH: c_uint = 0x02;
pub const CORE_TX_FIFO_FLUSH: c_uint = 0x04;
pub const CORE_SEND_JAM: c_uint = 0x08;
pub const CORE_MDC_EN: c_uint = 0x10 /* MDIO using NPE-B ETH-0 only */;

    TX_CNTRL0_PAD_EN | TX_CNTRL0_APPEND_FCS | \
    TX_CNTRL0_2DEFER)

// NPE message codes
pub const NPE_GETSTATUS: c_uint = 0x00;
pub const NPE_EDB_SETPORTADDRESS: c_uint = 0x01;
pub const NPE_EDB_GETMACADDRESSDATABASE: c_uint = 0x02;
pub const NPE_EDB_SETMACADDRESSSDATABASE: c_uint = 0x03;
pub const NPE_GETSTATS: c_uint = 0x04;
pub const NPE_RESETSTATS: c_uint = 0x05;
pub const NPE_SETMAXFRAMELENGTHS: c_uint = 0x06;
pub const NPE_VLAN_SETRXTAGMODE: c_uint = 0x07;
pub const NPE_VLAN_SETDEFAULTRXVID: c_uint = 0x08;
pub const NPE_VLAN_SETPORTVLANTABLEENTRY: c_uint = 0x09;
pub const NPE_VLAN_SETPORTVLANTABLERANGE: c_uint = 0x0A;
pub const NPE_VLAN_SETRXQOSENTRY: c_uint = 0x0B;
pub const NPE_VLAN_SETPORTIDEXTRACTIONMODE: c_uint = 0x0C;
pub const NPE_STP_SETBLOCKINGSTATE: c_uint = 0x0D;
pub const NPE_FW_SETFIREWALLMODE: c_uint = 0x0E;
pub const NPE_PC_SETFRAMECONTROLDURATIONID: c_uint = 0x0F;
pub const NPE_PC_SETAPMACTABLE: c_uint = 0x11;
pub const NPE_SETLOOPBACK_MODE: c_uint = 0x12;
pub const NPE_PC_SETBSSIDTABLE: c_uint = 0x13;
pub const NPE_ADDRESS_FILTER_CONFIG: c_uint = 0x14;
pub const NPE_APPENDFCSCONFIG: c_uint = 0x15;
pub const NPE_NOTIFY_MAC_RECOVERY_DONE: c_uint = 0x16;
pub const NPE_MAC_RECOVERY_START: c_uint = 0x17;

    typedef struct sk_buff buffer_t;

    typedef void buffer_t;

// Information about built-in Ethernet MAC interfaces
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_plat_info {
    pub /: *mut *mut u8 rxq; / configurable, currently 0 - 31 only,
    pub txreadyq: u8,
    pub hwaddr: [u8; ETH_ALEN],
    pub /: *mut *mut u8 npe; / NPE instance used by this interface,
    pub /: *mut *mut bool has_mdio; / If this instance has an MDIO bus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_regs {
    pub /: *mut *mut u32 tx_control[2], __res1[2]; / 000,
    pub /: *mut *mut u32 rx_control[2], __res2[2]; / 010,
    pub /: *mut *mut u32 random_seed, __res3[3]; / 020,
    pub /: *mut *mut u32 partial_empty_threshold, __res4; / 030,
    pub /: *mut *mut u32 partial_full_threshold, __res5; / 038,
    pub /: *mut *mut u32 tx_start_bytes, __res6[3]; / 040,
    pub /: *mut *mut u32 tx_deferral, rx_deferral, __res7[2];/ 050,
    pub /: *mut *mut u32 tx_2part_deferral[2], __res8[2]; / 060,
    pub /: *mut *mut u32 slot_time, __res9[3]; / 070,
    pub /: *mut *mut u32 mdio_command[4]; / 080,
    pub /: *mut *mut u32 mdio_status[4]; / 090,
    pub /: *mut *mut u32 mcast_mask[6], __res10[2]; / 0A0,
    pub /: *mut *mut u32 mcast_addr[6], __res11[2]; / 0C0,
    pub /: *mut *mut u32 int_clock_threshold, __res12[3]; / 0E0,
    pub /: *mut *mut u32 hw_addr[6], __res13[61]; / 0F0,
    pub /: *mut *mut u32 core_control; / 1FC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct port {
    pub regs: *mut eth_regs __iomem,
    pub timesync_regs: *mut ixp46x_ts_regs __iomem,
    pub phc_index: c_int,
    pub npe: *mut npe,
    pub netdev: *mut net_device,
    pub napi: napi_struct,
    pub plat: *mut eth_plat_info,
    pub tx_buff_tab: [*mut *mut buffer_t rx_buff_tab[RX_DESCS],; TX_DESCS],
    pub /: *mut *mut *mut desc desc_tab; / coherent,
    pub desc_tab_phys: dma_addr_t,
    pub /: *mut *mut int id; / logical port ID,
    pub duplex: int speed,,
    pub firmware: [u8; 4],
    pub hwts_tx_en: c_int,
    pub hwts_rx_en: c_int,
}

// NPE message structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg {

    pub byte3: u8 cmd, eth_id, byte2,,
    pub byte7: u8 byte4, byte5, byte6,,

    pub cmd: u8 byte3, byte2, eth_id,,
    pub byte4: u8 byte7, byte6, byte5,,

}

// Ethernet packet descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct desc {
    pub /: *mut *mut u32 next; / pointer to next buffer, unused,

    pub /: *mut *mut u16 buf_len; / buffer length,
    pub /: *mut *mut u16 pkt_len; / packet length,
    pub /: *mut *mut u32 data; / pointer to data buffer in RAM,
    pub dest_id: u8,
    pub src_id: u8,
    pub flags: u16,
    pub qos: u8,
    pub padlen: u8,
    pub vlan_tci: u16,

    pub /: *mut *mut u16 pkt_len; / packet length,
    pub /: *mut *mut u16 buf_len; / buffer length,
    pub /: *mut *mut u32 data; / pointer to data buffer in RAM,
    pub flags: u16,
    pub src_id: u8,
    pub dest_id: u8,
    pub vlan_tci: u16,
    pub padlen: u8,
    pub qos: u8,

    pub dst_mac_3: u8 dst_mac_0, dst_mac_1, dst_mac_2,,
    pub src_mac_1: u8 dst_mac_4, dst_mac_5, src_mac_0,,
    pub src_mac_5: u8 src_mac_2, src_mac_3, src_mac_4,,

    pub dst_mac_0: u8 dst_mac_3, dst_mac_2, dst_mac_1,,
    pub dst_mac_4: u8 src_mac_1, src_mac_0, dst_mac_5,,
    pub src_mac_2: u8 src_mac_5, src_mac_4, src_mac_3,,

}

    (n) * sizeof(struct desc))

    ((n) + RX_DESCS) * sizeof(struct desc))

#[no_mangle]
pub unsafe extern "C" fn memcpy_swab32(dest: *mut u32, src: *mut u32, cnt: c_int) {
    static inline void memcpy_swab32(u32 *dest, u32 *src, int cnt)
    {
    int i;
    for (i = 0; i < cnt; i++)
    dest[i] = swab32(src[i]);
    }

    static DEFINE_SPINLOCK(mdio_lock);
    static struct eth_regs __iomem *mdio_regs; /* mdio command and status only */
    static struct mii_bus *mdio_bus;
    static struct device_node *mdio_bus_np;
    static int ports_open;
    static struct port *npe_port_tab[MAX_NPES];
    static struct dma_pool *dma_pool;
#[no_mangle]
unsafe extern "C" fn ixp_ptp_match(skb: *mut sk_buff, uid_hi: u16, uid_lo: u32, seqid: u16) -> c_int {
    static int ixp_ptp_match(struct sk_buff *skb, u16 uid_hi, u32 uid_lo, u16 seqid)
    {
    u8 *data = skb.data;
    unsigned int offset;
    u16 *hi, *id;
    u32 lo;
    if (ptp_classify_raw(skb) != PTP_CLASS_V1_IPV4)
    return 0;
    offset = ETH_HLEN + IPV4_HLEN(data) + UDP_HLEN;
    if (skb.len < offset + OFF_PTP_SEQUENCE_ID + sizeof(seqid))
    return 0;
    hi = (u16 *)(data + offset + OFF_PTP_SOURCE_UUID);
    id = (u16 *)(data + offset + OFF_PTP_SEQUENCE_ID);
    memcpy(&lo, &hi[1], sizeof(lo));
    return (uid_hi == ntohs(*hi) &&
    uid_lo == ntohl(lo) &&
    seqid  == ntohs(*id));
    }
#[no_mangle]
unsafe extern "C" fn ixp_rx_timestamp(port: *mut port, skb: *mut sk_buff) {
    static void ixp_rx_timestamp(struct port *port, struct sk_buff *skb)
    {
    struct skb_shared_hwtstamps *shhwtstamps;
    struct ixp46x_ts_regs *regs;
    u64 ns;
    u32 ch, hi, lo, val;
    u16 uid, seq;
    if (!port.hwts_rx_en)
    return;
    ch = PORT2CHANNEL(port);
    regs = port.timesync_regs;
    val = __raw_readl(&regs.channel[ch].ch_event);
    if (!(val & RX_SNAPSHOT_LOCKED))
    return;
    lo = __raw_readl(&regs.channel[ch].src_uuid_lo);
    hi = __raw_readl(&regs.channel[ch].src_uuid_hi);
    uid = hi & 0xffff;
    seq = (hi >> 16) & 0xffff;
    if (!ixp_ptp_match(skb, htons(uid), htonl(lo), htons(seq)))
    goto out;
    lo = __raw_readl(&regs.channel[ch].rx_snap_lo);
    hi = __raw_readl(&regs.channel[ch].rx_snap_hi);
    ns = ((u64) hi) << 32;
    ns |= lo;
    ns <<= TICKS_NS_SHIFT;
    shhwtstamps = skb_hwtstamps(skb);
    memset(shhwtstamps, 0, sizeof(*shhwtstamps));
    shhwtstamps.hwtstamp = ns_to_ktime(ns);
    out:
    __raw_writel(RX_SNAPSHOT_LOCKED, &regs.channel[ch].ch_event);
    }
#[no_mangle]
unsafe extern "C" fn ixp_tx_timestamp(port: *mut port, skb: *mut sk_buff) {
    static void ixp_tx_timestamp(struct port *port, struct sk_buff *skb)
    {
    struct skb_shared_hwtstamps shhwtstamps;
    struct ixp46x_ts_regs *regs;
    struct skb_shared_info *shtx;
    u64 ns;
    u32 ch, cnt, hi, lo, val;
    shtx = skb_shinfo(skb);
    if (unlikely(shtx.tx_flags & SKBTX_HW_TSTAMP && port.hwts_tx_en))
    shtx.tx_flags |= SKBTX_IN_PROGRESS;
    else
    return;
    ch = PORT2CHANNEL(port);
    regs = port.timesync_regs;
//
// This really stinks, but we have to poll for the Tx time stamp.
// Usually, the time stamp is ready after 4 to 6 microseconds.
//
    for (cnt = 0; cnt < 100; cnt++) {
    val = __raw_readl(&regs.channel[ch].ch_event);
    if (val & TX_SNAPSHOT_LOCKED)
    break;
    udelay(1);
    }
    if (!(val & TX_SNAPSHOT_LOCKED)) {
    shtx.tx_flags &= ~SKBTX_IN_PROGRESS;
    return;
    }
    lo = __raw_readl(&regs.channel[ch].tx_snap_lo);
    hi = __raw_readl(&regs.channel[ch].tx_snap_hi);
    ns = ((u64) hi) << 32;
    ns |= lo;
    ns <<= TICKS_NS_SHIFT;
    memset(&shhwtstamps, 0, sizeof(shhwtstamps));
    shhwtstamps.hwtstamp = ns_to_ktime(ns);
    skb_tstamp_tx(skb, &shhwtstamps);
    __raw_writel(TX_SNAPSHOT_LOCKED, &regs.channel[ch].ch_event);
    }
    static int ixp4xx_hwtstamp_set(struct net_device *netdev,
    struct kernel_hwtstamp_config *cfg,
    struct netlink_ext_ack *extack)
    {
    struct ixp46x_ts_regs *regs;
    struct port *port = netdev_priv(netdev);
    int ret;
    int ch;
    if (!netif_running(netdev))
    return -EINVAL;
    ret = ixp46x_ptp_find(&port.timesync_regs, &port.phc_index);
    if (ret)
    return -EOPNOTSUPP;
    ch = PORT2CHANNEL(port);
    regs = port.timesync_regs;
    if (cfg.tx_type != HWTSTAMP_TX_OFF && cfg.tx_type != HWTSTAMP_TX_ON)
    return -ERANGE;
    switch (cfg.rx_filter) {
    case HWTSTAMP_FILTER_NONE:
    port.hwts_rx_en = 0;
    break;
    case HWTSTAMP_FILTER_PTP_V1_L4_SYNC:
    port.hwts_rx_en = PTP_SLAVE_MODE;
    __raw_writel(0, &regs.channel[ch].ch_control);
    break;
    case HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ:
    port.hwts_rx_en = PTP_MASTER_MODE;
    __raw_writel(MASTER_MODE, &regs.channel[ch].ch_control);
    break;
    default:
    return -ERANGE;
    }
    port.hwts_tx_en = cfg.tx_type == HWTSTAMP_TX_ON;
// Clear out any old time stamps.
    __raw_writel(TX_SNAPSHOT_LOCKED | RX_SNAPSHOT_LOCKED,
    &regs.channel[ch].ch_event);
    return 0;
    }
    static int ixp4xx_hwtstamp_get(struct net_device *netdev,
    struct kernel_hwtstamp_config *cfg)
    {
    struct port *port = netdev_priv(netdev);
    if (!cpu_is_ixp46x())
    return -EOPNOTSUPP;
    if (!netif_running(netdev))
    return -EINVAL;
    cfg.flags = 0;
    cfg.tx_type = port.hwts_tx_en ? HWTSTAMP_TX_ON : HWTSTAMP_TX_OFF;
    switch (port.hwts_rx_en) {
    case 0:
    cfg.rx_filter = HWTSTAMP_FILTER_NONE;
    break;
    case PTP_SLAVE_MODE:
    cfg.rx_filter = HWTSTAMP_FILTER_PTP_V1_L4_SYNC;
    break;
    case PTP_MASTER_MODE:
    cfg.rx_filter = HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ;
    break;
    default:
    WARN_ON_ONCE(1);
    return -ERANGE;
    }
    return 0;
    }
    static int ixp4xx_mdio_cmd(struct mii_bus *bus, int phy_id, int location,
    int write, u16 cmd)
    {
    let mut cycles: c_int = 0;
    if (__raw_readl(&mdio_regs.mdio_command[3]) & 0x80) {
    printk(KERN_ERR "%s: MII not ready to transmit\n", bus.name);
    return -1;
    }
    if (write) {
    __raw_writel(cmd & 0xFF, &mdio_regs.mdio_command[0]);
    __raw_writel(cmd >> 8, &mdio_regs.mdio_command[1]);
    }
    __raw_writel(((phy_id << 5) | location) & 0xFF,
    &mdio_regs.mdio_command[2]);
    __raw_writel((phy_id >> 3) | (write << 2) | 0x80 /* GO */,
    &mdio_regs.mdio_command[3]);
    while ((cycles < MAX_MDIO_RETRIES) &&
    (__raw_readl(&mdio_regs.mdio_command[3]) & 0x80)) {
    udelay(1);
    cycles++;
    }
    if (cycles == MAX_MDIO_RETRIES) {
    printk(KERN_ERR "%s #%i: MII write failed\n", bus.name,
    phy_id);
    return -1;
    }

    printk(KERN_DEBUG "%s #%i: mdio_%s() took %i cycles\n", bus.name,
    phy_id, write ? "write" : "read", cycles);

    if (write)
    return 0;
    if (__raw_readl(&mdio_regs.mdio_status[3]) & 0x80) {

    printk(KERN_DEBUG "%s #%i: MII read failed\n", bus.name,
    phy_id);

    return 0xFFFF; /* don't return error */
    }
    return (__raw_readl(&mdio_regs.mdio_status[0]) & 0xFF) |
    ((__raw_readl(&mdio_regs.mdio_status[1]) & 0xFF) << 8);
    }
#[no_mangle]
unsafe extern "C" fn ixp4xx_mdio_read(bus: *mut mii_bus, phy_id: c_int, location: c_int) -> c_int {
    static int ixp4xx_mdio_read(struct mii_bus *bus, int phy_id, int location)
    {
    unsigned long flags;
    int ret;
    spin_lock_irqsave(&mdio_lock, flags);
    ret = ixp4xx_mdio_cmd(bus, phy_id, location, 0, 0);
    spin_unlock_irqrestore(&mdio_lock, flags);

    printk(KERN_DEBUG "%s #%i: MII read [%i] . 0x%X\n", bus.name,
    phy_id, location, ret);

    return ret;
    }
    static int ixp4xx_mdio_write(struct mii_bus *bus, int phy_id, int location,
    u16 val)
    {
    unsigned long flags;
    int ret;
    spin_lock_irqsave(&mdio_lock, flags);
    ret = ixp4xx_mdio_cmd(bus, phy_id, location, 1, val);
    spin_unlock_irqrestore(&mdio_lock, flags);

    printk(KERN_DEBUG "%s #%i: MII write [%i] <- 0x%X, err = %i\n",
    bus.name, phy_id, location, val, ret);

    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ixp4xx_mdio_register(regs: *mut eth_regs __iomem) -> c_int {
    static int ixp4xx_mdio_register(struct eth_regs __iomem *regs)
    {
    int err;
    if (!(mdio_bus = mdiobus_alloc()))
    return -ENOMEM;
    mdio_regs = regs;
    __raw_writel(DEFAULT_CORE_CNTRL, &mdio_regs.core_control);
    mdio_bus.name = "IXP4xx MII Bus";
    mdio_bus.read = &ixp4xx_mdio_read;
    mdio_bus.write = &ixp4xx_mdio_write;
    snprintf(mdio_bus.id, MII_BUS_ID_SIZE, "ixp4xx-eth-0");
    err = of_mdiobus_register(mdio_bus, mdio_bus_np);
    if (err)
    mdiobus_free(mdio_bus);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ixp4xx_mdio_remove() {
    static void ixp4xx_mdio_remove(void)
    {
    mdiobus_unregister(mdio_bus);
    mdiobus_free(mdio_bus);
    }
#[no_mangle]
unsafe extern "C" fn ixp4xx_adjust_link(dev: *mut net_device) {
    static void ixp4xx_adjust_link(struct net_device *dev)
    {
    struct port *port = netdev_priv(dev);
    struct phy_device *phydev = dev.phydev;
    if (!phydev.link) {
    if (port.speed) {
    port.speed = 0;
    printk(KERN_INFO "%s: link down\n", dev.name);
    }
    return;
    }
    if (port.speed == phydev.speed && port.duplex == phydev.duplex)
    return;
    port.speed = phydev.speed;
    port.duplex = phydev.duplex;
    if (port.duplex)
    __raw_writel(DEFAULT_TX_CNTRL0 & ~TX_CNTRL0_HALFDUPLEX,
    &port.regs.tx_control[0]);
    else
    __raw_writel(DEFAULT_TX_CNTRL0 | TX_CNTRL0_HALFDUPLEX,
    &port.regs.tx_control[0]);
    netdev_info(dev, "%s: link up, speed %u Mb/s, %s duplex\n",
    dev.name, port.speed, port.duplex ? "full" : "half");
    }
    static inline void debug_pkt(struct net_device *dev, const char *func,
    u8 *data, int len)
    {

    int i;
    netdev_debug(dev, "%s(%i) ", func, len);
    for (i = 0; i < len; i++) {
    if (i >= DEBUG_PKT_BYTES)
    break;
    printk("%s%02X",
    ((i == 6) || (i == 12) || (i >= 14)) ? " " : "",
    data[i]);
    }
    printk("\n");

    }
#[no_mangle]
pub unsafe extern "C" fn debug_desc(phys: u32, desc: *mut desc) {
    static inline void debug_desc(u32 phys, struct desc *desc)
    {

    printk(KERN_DEBUG "%X: %X %3X %3X %08X %2X < %2X %4X %X"
    " %X %X %02X%02X%02X%02X%02X%02X < %02X%02X%02X%02X%02X%02X\n",
    phys, desc.next, desc.buf_len, desc.pkt_len,
    desc.data, desc.dest_id, desc.src_id, desc.flags,
    desc.qos, desc.padlen, desc.vlan_tci,
    desc.dst_mac_0, desc.dst_mac_1, desc.dst_mac_2,
    desc.dst_mac_3, desc.dst_mac_4, desc.dst_mac_5,
    desc.src_mac_0, desc.src_mac_1, desc.src_mac_2,
    desc.src_mac_3, desc.src_mac_4, desc.src_mac_5);

    }
    static inline int queue_get_desc(unsigned int queue, struct port *port,
    int is_tx)
    {
    u32 phys, tab_phys, n_desc;
    struct desc *tab;
    if (!(phys = qmgr_get_entry(queue)))
    return -1;
    phys &= ~0x1F; /* mask out non-address bits */
    tab_phys = is_tx ? tx_desc_phys(port, 0) : rx_desc_phys(port, 0);
    tab = is_tx ? tx_desc_ptr(port, 0) : rx_desc_ptr(port, 0);
    n_desc = (phys - tab_phys) / sizeof(struct desc);
    BUG_ON(n_desc >= (is_tx ? TX_DESCS : RX_DESCS));
    debug_desc(phys, &tab[n_desc]);
    BUG_ON(tab[n_desc].next);
    return n_desc;
    }
    static inline void queue_put_desc(unsigned int queue, u32 phys,
    struct desc *desc)
    {
    debug_desc(phys, desc);
    BUG_ON(phys & 0x1F);
    qmgr_put_entry(queue, phys);
// Don't check for queue overflow here, we've allocated sufficient
    length and queues >= 32 don't support this check anyway. */
    }
#[no_mangle]
pub unsafe extern "C" fn dma_unmap_tx(port: *mut port, desc: *mut desc) {
    static inline void dma_unmap_tx(struct port *port, struct desc *desc)
    {

    dma_unmap_single(&port.netdev.dev, desc.data,
    desc.buf_len, DMA_TO_DEVICE);

    dma_unmap_single(&port.netdev.dev, desc.data & ~3,
    ALIGN((desc.data & 3) + desc.buf_len, 4),
    DMA_TO_DEVICE);

    }
#[no_mangle]
unsafe extern "C" fn eth_rx_irq(pdev: *mut c_void) {
    static void eth_rx_irq(void *pdev)
    {
    struct net_device *dev = pdev;
    struct port *port = netdev_priv(dev);

    printk(KERN_DEBUG "%s: eth_rx_irq\n", dev.name);

    qmgr_disable_irq(port.plat.rxq);
    napi_schedule(&port.napi);
    }
#[no_mangle]
unsafe extern "C" fn eth_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int eth_poll(struct napi_struct *napi, int budget)
    {
    struct port *port = container_of(napi, struct port, napi);
    struct net_device *dev = port.netdev;
    let mut rxq: c_uint = port.plat.rxq, rxfreeq = RXFREE_QUEUE(port.id);
    let mut received: c_int = 0;

    netdev_debug(dev, "eth_poll\n");

    while (received < budget) {
    struct sk_buff *skb;
    struct desc *desc;
    int n;

    struct sk_buff *temp;
    u32 phys;

    if ((n = queue_get_desc(rxq, port, 0)) < 0) {

    netdev_debug(dev, "eth_poll napi_complete\n");

    napi_complete(napi);
    qmgr_enable_irq(rxq);
    if (!qmgr_stat_below_low_watermark(rxq) &&
    napi_schedule(napi)) { /* not empty again */

    netdev_debug(dev, "eth_poll napi_schedule succeeded\n");

    qmgr_disable_irq(rxq);
    continue;
    }

    netdev_debug(dev, "eth_poll all done\n");

    return received; /* all work done */
    }
    desc = rx_desc_ptr(port, n);

    if ((skb = netdev_alloc_skb(dev, RX_BUFF_SIZE))) {
    phys = dma_map_single(&dev.dev, skb.data,
    RX_BUFF_SIZE, DMA_FROM_DEVICE);
    if (dma_mapping_error(&dev.dev, phys)) {
    dev_kfree_skb(skb);
    skb = core::ptr::null_mut();
    }
    }

    skb = netdev_alloc_skb(dev,
    ALIGN(NET_IP_ALIGN + desc.pkt_len, 4));

    if (!skb) {
    dev.stats.rx_dropped++;
// put the desc back on RX-ready queue
    desc.buf_len = MAX_MRU;
    desc.pkt_len = 0;
    queue_put_desc(rxfreeq, rx_desc_phys(port, n), desc);
    continue;
    }
// process received frame

    temp = skb;
    skb = port.rx_buff_tab[n];
    dma_unmap_single(&dev.dev, desc.data - NET_IP_ALIGN,
    RX_BUFF_SIZE, DMA_FROM_DEVICE);

    dma_sync_single_for_cpu(&dev.dev, desc.data - NET_IP_ALIGN,
    RX_BUFF_SIZE, DMA_FROM_DEVICE);
    memcpy_swab32((u32 *)skb.data, (u32 *)port.rx_buff_tab[n],
    ALIGN(NET_IP_ALIGN + desc.pkt_len, 4) / 4);

    skb_reserve(skb, NET_IP_ALIGN);
    skb_put(skb, desc.pkt_len);
    debug_pkt(dev, "eth_poll", skb.data, skb.len);
    ixp_rx_timestamp(port, skb);
    skb.protocol = eth_type_trans(skb, dev);
    dev.stats.rx_packets++;
    dev.stats.rx_bytes += skb.len;
    netif_receive_skb(skb);
// put the new buffer on RX-free queue

    port.rx_buff_tab[n] = temp;
    desc.data = phys + NET_IP_ALIGN;

    desc.buf_len = MAX_MRU;
    desc.pkt_len = 0;
    queue_put_desc(rxfreeq, rx_desc_phys(port, n), desc);
    received++;
    }

    netdev_debug(dev, "eth_poll(): end, not all work done\n");

    return received;		/* not all work done */
    }
#[no_mangle]
unsafe extern "C" fn eth_txdone_irq(unused: *mut c_void) {
    static void eth_txdone_irq(void *unused)
    {
    u32 phys;

    printk(KERN_DEBUG DRV_NAME ": eth_txdone_irq\n");

    while ((phys = qmgr_get_entry(TXDONE_QUEUE)) != 0) {
    u32 npe_id, n_desc;
    struct port *port;
    struct desc *desc;
    int start;
    npe_id = phys & 3;
    BUG_ON(npe_id >= MAX_NPES);
    port = npe_port_tab[npe_id];
    BUG_ON(!port);
    phys &= ~0x1F; /* mask out non-address bits */
    n_desc = (phys - tx_desc_phys(port, 0)) / sizeof(struct desc);
    BUG_ON(n_desc >= TX_DESCS);
    desc = tx_desc_ptr(port, n_desc);
    debug_desc(phys, desc);
    if (port.tx_buff_tab[n_desc]) { /* not the draining packet */
    port.netdev.stats.tx_packets++;
    port.netdev.stats.tx_bytes += desc.pkt_len;
    dma_unmap_tx(port, desc);

    printk(KERN_DEBUG "%s: eth_txdone_irq free %p\n",
    port.netdev.name, port.tx_buff_tab[n_desc]);

    free_buffer_irq(port.tx_buff_tab[n_desc]);
    port.tx_buff_tab[n_desc] = core::ptr::null_mut();
    }
    start = qmgr_stat_below_low_watermark(port.plat.txreadyq);
    queue_put_desc(port.plat.txreadyq, phys, desc);
    if (start) { /* TX-ready queue was empty */

    printk(KERN_DEBUG "%s: eth_txdone_irq xmit ready\n",
    port.netdev.name);

    netif_wake_queue(port.netdev);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn eth_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t eth_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct port *port = netdev_priv(dev);
    let mut txreadyq: c_uint = port.plat.txreadyq;
    int len, offset, bytes, n;
    void *mem;
    u32 phys;
    struct desc *desc;

    netdev_debug(dev, "eth_xmit\n");

    if (unlikely(skb.len > MAX_MRU)) {
    dev_kfree_skb(skb);
    dev.stats.tx_errors++;
    return NETDEV_TX_OK;
    }
    debug_pkt(dev, "eth_xmit", skb.data, skb.len);
    len = skb.len;

    offset = 0; /* no need to keep alignment */
    bytes = len;
    mem = skb.data;

    offset = (uintptr_t)skb.data & 3; /* keep 32-bit alignment */
    bytes = ALIGN(offset + len, 4);
    if (!(mem = kmalloc(bytes, GFP_ATOMIC))) {
    dev_kfree_skb(skb);
    dev.stats.tx_dropped++;
    return NETDEV_TX_OK;
    }
    memcpy_swab32(mem, (u32 *)((uintptr_t)skb.data & ~3), bytes / 4);

    phys = dma_map_single(&dev.dev, mem, bytes, DMA_TO_DEVICE);
    if (dma_mapping_error(&dev.dev, phys)) {
    dev_kfree_skb(skb);

    kfree(mem);

    dev.stats.tx_dropped++;
    return NETDEV_TX_OK;
    }
    n = queue_get_desc(txreadyq, port, 1);
    BUG_ON(n < 0);
    desc = tx_desc_ptr(port, n);

    port.tx_buff_tab[n] = skb;

    port.tx_buff_tab[n] = mem;

    desc.data = phys + offset;
    desc.buf_len = desc.pkt_len = len;
// NPE firmware pads short frames with zeros internally
    wmb();
    queue_put_desc(TX_QUEUE(port.id), tx_desc_phys(port, n), desc);
    if (qmgr_stat_below_low_watermark(txreadyq)) { /* empty */

    netdev_debug(dev, "eth_xmit queue full\n");

    netif_stop_queue(dev);
// we could miss TX ready interrupt
// really empty in fact
    if (!qmgr_stat_below_low_watermark(txreadyq)) {

    netdev_debug(dev, "eth_xmit ready again\n");

    netif_wake_queue(dev);
    }
    }

    netdev_debug(dev, "eth_xmit end\n");

    ixp_tx_timestamp(port, skb);
    skb_tx_timestamp(skb);

    dev_kfree_skb(skb);

    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn eth_set_mcast_list(dev: *mut net_device) {
    static void eth_set_mcast_list(struct net_device *dev)
    {
    struct port *port = netdev_priv(dev);
    struct netdev_hw_addr *ha;
    u8 diffs[ETH_ALEN], *addr;
    int i;
    static const u8 allmulti[] = { 0x01, 0x00, 0x00, 0x00, 0x00, 0x00 };
    if ((dev.flags & IFF_ALLMULTI) && !(dev.flags & IFF_PROMISC)) {
    for (i = 0; i < ETH_ALEN; i++) {
    __raw_writel(allmulti[i], &port.regs.mcast_addr[i]);
    __raw_writel(allmulti[i], &port.regs.mcast_mask[i]);
    }
    __raw_writel(DEFAULT_RX_CNTRL0 | RX_CNTRL0_ADDR_FLTR_EN,
    &port.regs.rx_control[0]);
    return;
    }
    if ((dev.flags & IFF_PROMISC) || netdev_mc_empty(dev)) {
    __raw_writel(DEFAULT_RX_CNTRL0 & ~RX_CNTRL0_ADDR_FLTR_EN,
    &port.regs.rx_control[0]);
    return;
    }
    eth_zero_addr(diffs);
    addr = core::ptr::null_mut();
    netdev_for_each_mc_addr(ha, dev) {
    if (!addr)
    addr = ha.addr; /* first MAC address */
    for (i = 0; i < ETH_ALEN; i++)
    diffs[i] |= addr[i] ^ ha.addr[i];
    }
    for (i = 0; i < ETH_ALEN; i++) {
    __raw_writel(addr[i], &port.regs.mcast_addr[i]);
    __raw_writel(~diffs[i], &port.regs.mcast_mask[i]);
    }
    __raw_writel(DEFAULT_RX_CNTRL0 | RX_CNTRL0_ADDR_FLTR_EN,
    &port.regs.rx_control[0]);
    }
// ethtool support
    static void ixp4xx_get_drvinfo(struct net_device *dev,
    struct ethtool_drvinfo *info)
    {
    struct port *port = netdev_priv(dev);
    strscpy(info.driver, DRV_NAME, sizeof(info.driver));
    snprintf(info.fw_version, sizeof(info.fw_version), "%u:%u:%u:%u",
    port.firmware[0], port.firmware[1],
    port.firmware[2], port.firmware[3]);
    strscpy(info.bus_info, "internal", sizeof(info.bus_info));
    }
    static int ixp4xx_get_ts_info(struct net_device *dev,
    struct kernel_ethtool_ts_info *info)
    {
    struct port *port = netdev_priv(dev);
    if (port.phc_index < 0)
    ixp46x_ptp_find(&port.timesync_regs, &port.phc_index);
    info.phc_index = port.phc_index;
    if (info.phc_index < 0) {
    info.so_timestamping =
    SOF_TIMESTAMPING_TX_SOFTWARE;
    return 0;
    }
    info.so_timestamping =
    SOF_TIMESTAMPING_TX_HARDWARE |
    SOF_TIMESTAMPING_RX_HARDWARE |
    SOF_TIMESTAMPING_RAW_HARDWARE;
    info.tx_types =
    (1 << HWTSTAMP_TX_OFF) |
    (1 << HWTSTAMP_TX_ON);
    info.rx_filters =
    (1 << HWTSTAMP_FILTER_NONE) |
    (1 << HWTSTAMP_FILTER_PTP_V1_L4_SYNC) |
    (1 << HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ);
    return 0;
    }
    static const struct ethtool_ops ixp4xx_ethtool_ops = {
    .get_drvinfo = ixp4xx_get_drvinfo,
    .nway_reset = phy_ethtool_nway_reset,
    .get_link = ethtool_op_get_link,
    .get_ts_info = ixp4xx_get_ts_info,
    .get_link_ksettings = phy_ethtool_get_link_ksettings,
    .set_link_ksettings = phy_ethtool_set_link_ksettings,
    };
#[no_mangle]
unsafe extern "C" fn request_queues(port: *mut port) -> c_int {
    static int request_queues(struct port *port)
    {
    int err;
    err = qmgr_request_queue(RXFREE_QUEUE(port.id), RX_DESCS, 0, 0,
    "%s:RX-free", port.netdev.name);
    if (err)
    return err;
    err = qmgr_request_queue(port.plat.rxq, RX_DESCS, 0, 0,
    "%s:RX", port.netdev.name);
    if (err)
    goto rel_rxfree;
    err = qmgr_request_queue(TX_QUEUE(port.id), TX_DESCS, 0, 0,
    "%s:TX", port.netdev.name);
    if (err)
    goto rel_rx;
    err = qmgr_request_queue(port.plat.txreadyq, TX_DESCS, 0, 0,
    "%s:TX-ready", port.netdev.name);
    if (err)
    goto rel_tx;
// TX-done queue handles skbs sent out by the NPEs
    if (!ports_open) {
    err = qmgr_request_queue(TXDONE_QUEUE, TXDONE_QUEUE_LEN, 0, 0,
    "%s:TX-done", DRV_NAME);
    if (err)
    goto rel_txready;
    }
    return 0;
    rel_txready:
    qmgr_release_queue(port.plat.txreadyq);
    rel_tx:
    qmgr_release_queue(TX_QUEUE(port.id));
    rel_rx:
    qmgr_release_queue(port.plat.rxq);
    rel_rxfree:
    qmgr_release_queue(RXFREE_QUEUE(port.id));
    printk(KERN_DEBUG "%s: unable to request hardware queues\n",
    port.netdev.name);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn release_queues(port: *mut port) {
    static void release_queues(struct port *port)
    {
    qmgr_release_queue(RXFREE_QUEUE(port.id));
    qmgr_release_queue(port.plat.rxq);
    qmgr_release_queue(TX_QUEUE(port.id));
    qmgr_release_queue(port.plat.txreadyq);
    if (!ports_open)
    qmgr_release_queue(TXDONE_QUEUE);
    }
#[no_mangle]
unsafe extern "C" fn init_queues(port: *mut port) -> c_int {
    static int init_queues(struct port *port)
    {
    int i;
    if (!ports_open) {
    dma_pool = dma_pool_create(DRV_NAME, &port.netdev.dev,
    POOL_ALLOC_SIZE, 32, 0);
    if (!dma_pool)
    return -ENOMEM;
    }
    port.desc_tab = dma_pool_zalloc(dma_pool, GFP_KERNEL, &port.desc_tab_phys);
    if (!port.desc_tab)
    return -ENOMEM;
    memset(port.rx_buff_tab, 0, sizeof(port.rx_buff_tab)); /* tables */
    memset(port.tx_buff_tab, 0, sizeof(port.tx_buff_tab));
// Setup RX buffers
    for (i = 0; i < RX_DESCS; i++) {
    struct desc *desc = rx_desc_ptr(port, i);
    buffer_t *buff; /* skb or kmalloc()ated memory */
    void *data;

    if (!(buff = netdev_alloc_skb(port.netdev, RX_BUFF_SIZE)))
    return -ENOMEM;
    data = buff.data;

    if (!(buff = kmalloc(RX_BUFF_SIZE, GFP_KERNEL)))
    return -ENOMEM;
    data = buff;

    desc.buf_len = MAX_MRU;
    desc.data = dma_map_single(&port.netdev.dev, data,
    RX_BUFF_SIZE, DMA_FROM_DEVICE);
    if (dma_mapping_error(&port.netdev.dev, desc.data)) {
    free_buffer(buff);
    return -EIO;
    }
    desc.data += NET_IP_ALIGN;
    port.rx_buff_tab[i] = buff;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn destroy_queues(port: *mut port) {
    static void destroy_queues(struct port *port)
    {
    int i;
    if (port.desc_tab) {
    for (i = 0; i < RX_DESCS; i++) {
    struct desc *desc = rx_desc_ptr(port, i);
    buffer_t *buff = port.rx_buff_tab[i];
    if (buff) {
    dma_unmap_single(&port.netdev.dev,
    desc.data - NET_IP_ALIGN,
    RX_BUFF_SIZE, DMA_FROM_DEVICE);
    free_buffer(buff);
    }
    }
    for (i = 0; i < TX_DESCS; i++) {
    struct desc *desc = tx_desc_ptr(port, i);
    buffer_t *buff = port.tx_buff_tab[i];
    if (buff) {
    dma_unmap_tx(port, desc);
    free_buffer(buff);
    }
    }
    dma_pool_free(dma_pool, port.desc_tab, port.desc_tab_phys);
    port.desc_tab = core::ptr::null_mut();
    }
    if (!ports_open && dma_pool) {
    dma_pool_destroy(dma_pool);
    dma_pool = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn ixp4xx_do_change_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int {
    static int ixp4xx_do_change_mtu(struct net_device *dev, int new_mtu)
    {
    struct port *port = netdev_priv(dev);
    struct npe *npe = port.npe;
    int framesize, chunks;
    let mut msg: msg = {};
// adjust for ethernet headers
    framesize = new_mtu + VLAN_ETH_HLEN;
// max rx/tx 64 byte chunks
    chunks = DIV_ROUND_UP(framesize, 64);
    msg.cmd = NPE_SETMAXFRAMELENGTHS;
    msg.eth_id = port.id;
// Firmware wants to know buffer size in 64 byte chunks
    msg.byte2 = chunks << 8;
    msg.byte3 = chunks << 8;
    msg.byte4 = msg.byte6 = framesize >> 8;
    msg.byte5 = msg.byte7 = framesize & 0xff;
    if (npe_send_recv_message(npe, &msg, "ETH_SET_MAX_FRAME_LENGTH"))
    return -EIO;
    netdev_dbg(dev, "set MTU on NPE %s to %d bytes\n",
    npe_name(npe), new_mtu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ixp4xx_eth_change_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int {
    static int ixp4xx_eth_change_mtu(struct net_device *dev, int new_mtu)
    {
    int ret;
// MTU can only be changed when the interface is up. We also
// set the MTU from dev->mtu when opening the device.
//
    if (dev.flags & IFF_UP) {
    ret = ixp4xx_do_change_mtu(dev, new_mtu);
    if (ret < 0)
    return ret;
    }
    WRITE_ONCE(dev.mtu, new_mtu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eth_open(dev: *mut net_device) -> c_int {
    static int eth_open(struct net_device *dev)
    {
    struct port *port = netdev_priv(dev);
    struct npe *npe = port.npe;
    struct msg msg;
    int i, err;
    if (!npe_running(npe)) {
    err = npe_load_firmware(npe, npe_name(npe), &dev.dev);
    if (err)
    return err;
    if (npe_recv_message(npe, &msg, "ETH_GET_STATUS")) {
    netdev_err(dev, "%s not responding\n", npe_name(npe));
    return -EIO;
    }
    port.firmware[0] = msg.byte4;
    port.firmware[1] = msg.byte5;
    port.firmware[2] = msg.byte6;
    port.firmware[3] = msg.byte7;
    }
    memset(&msg, 0, sizeof(msg));
    msg.cmd = NPE_VLAN_SETRXQOSENTRY;
    msg.eth_id = port.id;
    msg.byte5 = port.plat.rxq | 0x80;
    msg.byte7 = port.plat.rxq << 4;
    for (i = 0; i < 8; i++) {
    msg.byte3 = i;
    if (npe_send_recv_message(port.npe, &msg, "ETH_SET_RXQ"))
    return -EIO;
    }
    msg.cmd = NPE_EDB_SETPORTADDRESS;
    msg.eth_id = PHYSICAL_ID(port.id);
    msg.byte2 = dev.dev_addr[0];
    msg.byte3 = dev.dev_addr[1];
    msg.byte4 = dev.dev_addr[2];
    msg.byte5 = dev.dev_addr[3];
    msg.byte6 = dev.dev_addr[4];
    msg.byte7 = dev.dev_addr[5];
    if (npe_send_recv_message(port.npe, &msg, "ETH_SET_MAC"))
    return -EIO;
    memset(&msg, 0, sizeof(msg));
    msg.cmd = NPE_FW_SETFIREWALLMODE;
    msg.eth_id = port.id;
    if (npe_send_recv_message(port.npe, &msg, "ETH_SET_FIREWALL_MODE"))
    return -EIO;
    ixp4xx_do_change_mtu(dev, dev.mtu);
    if ((err = request_queues(port)) != 0)
    return err;
    if ((err = init_queues(port)) != 0) {
    destroy_queues(port);
    release_queues(port);
    return err;
    }
    port.speed = 0;	/* force "link up" message */
    phy_start(dev.phydev);
    for (i = 0; i < ETH_ALEN; i++)
    __raw_writel(dev.dev_addr[i], &port.regs.hw_addr[i]);
    __raw_writel(0x08, &port.regs.random_seed);
    __raw_writel(0x12, &port.regs.partial_empty_threshold);
    __raw_writel(0x30, &port.regs.partial_full_threshold);
    __raw_writel(0x08, &port.regs.tx_start_bytes);
    __raw_writel(0x15, &port.regs.tx_deferral);
    __raw_writel(0x08, &port.regs.tx_2part_deferral[0]);
    __raw_writel(0x07, &port.regs.tx_2part_deferral[1]);
    __raw_writel(0x80, &port.regs.slot_time);
    __raw_writel(0x01, &port.regs.int_clock_threshold);
// Populate queues with buffers, no failure after this point
    for (i = 0; i < TX_DESCS; i++)
    queue_put_desc(port.plat.txreadyq,
    tx_desc_phys(port, i), tx_desc_ptr(port, i));
    for (i = 0; i < RX_DESCS; i++)
    queue_put_desc(RXFREE_QUEUE(port.id),
    rx_desc_phys(port, i), rx_desc_ptr(port, i));
    __raw_writel(TX_CNTRL1_RETRIES, &port.regs.tx_control[1]);
    __raw_writel(DEFAULT_TX_CNTRL0, &port.regs.tx_control[0]);
    __raw_writel(0, &port.regs.rx_control[1]);
    __raw_writel(DEFAULT_RX_CNTRL0, &port.regs.rx_control[0]);
    napi_enable(&port.napi);
    eth_set_mcast_list(dev);
    netif_start_queue(dev);
    qmgr_set_irq(port.plat.rxq, QUEUE_IRQ_SRC_NOT_EMPTY,
    eth_rx_irq, dev);
    if (!ports_open) {
    qmgr_set_irq(TXDONE_QUEUE, QUEUE_IRQ_SRC_NOT_EMPTY,
    eth_txdone_irq, core::ptr::null_mut());
    qmgr_enable_irq(TXDONE_QUEUE);
    }
    ports_open++;
// we may already have RX data, enables IRQ
    napi_schedule(&port.napi);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eth_close(dev: *mut net_device) -> c_int {
    static int eth_close(struct net_device *dev)
    {
    struct port *port = netdev_priv(dev);
    struct msg msg;
    int buffs = RX_DESCS; /* allocated RX buffers */
    int i;
    ports_open--;
    qmgr_disable_irq(port.plat.rxq);
    napi_disable(&port.napi);
    netif_stop_queue(dev);
    while (queue_get_desc(RXFREE_QUEUE(port.id), port, 0) >= 0)
    buffs--;
    memset(&msg, 0, sizeof(msg));
    msg.cmd = NPE_SETLOOPBACK_MODE;
    msg.eth_id = port.id;
    msg.byte3 = 1;
    if (npe_send_recv_message(port.npe, &msg, "ETH_ENABLE_LOOPBACK"))
    netdev_crit(dev, "unable to enable loopback\n");
    i = 0;
    do {			/* drain RX buffers */
    while (queue_get_desc(port.plat.rxq, port, 0) >= 0)
    buffs--;
    if (!buffs)
    break;
    if (qmgr_stat_empty(TX_QUEUE(port.id))) {
// we have to inject some packet
    struct desc *desc;
    u32 phys;
    let mut n: c_int = queue_get_desc(port.plat.txreadyq, port, 1);
    BUG_ON(n < 0);
    desc = tx_desc_ptr(port, n);
    phys = tx_desc_phys(port, n);
    desc.buf_len = desc.pkt_len = 1;
    wmb();
    queue_put_desc(TX_QUEUE(port.id), phys, desc);
    }
    udelay(1);
    } while (++i < MAX_CLOSE_WAIT);
    if (buffs)
    netdev_crit(dev, "unable to drain RX queue, %i buffer(s)"
    " left in NPE\n", buffs);

    if (!buffs)
    netdev_debug(dev, "draining RX queue took %i cycles\n", i);

    buffs = TX_DESCS;
    while (queue_get_desc(TX_QUEUE(port.id), port, 1) >= 0)
    buffs--; /* cancel TX */
    i = 0;
    do {
    while (queue_get_desc(port.plat.txreadyq, port, 1) >= 0)
    buffs--;
    if (!buffs)
    break;
    } while (++i < MAX_CLOSE_WAIT);
    if (buffs)
    netdev_crit(dev, "unable to drain TX queue, %i buffer(s) "
    "left in NPE\n", buffs);

    if (!buffs)
    netdev_debug(dev, "draining TX queues took %i cycles\n", i);

    msg.byte3 = 0;
    if (npe_send_recv_message(port.npe, &msg, "ETH_DISABLE_LOOPBACK"))
    netdev_crit(dev, "unable to disable loopback\n");
    phy_stop(dev.phydev);
    if (!ports_open)
    qmgr_disable_irq(TXDONE_QUEUE);
    destroy_queues(port);
    release_queues(port);
    return 0;
    }
    static const struct net_device_ops ixp4xx_netdev_ops = {
    .ndo_open = eth_open,
    .ndo_stop = eth_close,
    .ndo_change_mtu = ixp4xx_eth_change_mtu,
    .ndo_start_xmit = eth_xmit,
    .ndo_set_rx_mode = eth_set_mcast_list,
    .ndo_eth_ioctl = phy_do_ioctl_running,
    .ndo_set_mac_address = eth_mac_addr,
    .ndo_validate_addr = eth_validate_addr,
    .ndo_hwtstamp_get = ixp4xx_hwtstamp_get,
    .ndo_hwtstamp_set = ixp4xx_hwtstamp_set,
    };
    static struct eth_plat_info *ixp4xx_of_get_platdata(struct device *dev)
    {
    struct device_node *np = dev.of_node;
    struct of_phandle_args queue_spec;
    struct of_phandle_args npe_spec;
    struct device_node *mdio_np;
    struct eth_plat_info *plat;
    u8 mac[ETH_ALEN];
    int ret;
    plat = devm_kzalloc(dev, sizeof(*plat), GFP_KERNEL);
    if (!plat)
    return core::ptr::null_mut();
    ret = of_parse_phandle_with_fixed_args(np, "intel,npe-handle", 1, 0,
    &npe_spec);
    if (ret) {
    dev_err(dev, "no NPE engine specified\n");
    return core::ptr::null_mut();
    }
// NPE ID 0x00, 0x10, 0x20...
    plat.npe = (npe_spec.args[0] << 4);
// Check if this device has an MDIO bus
    mdio_np = of_get_child_by_name(np, "mdio");
    if (mdio_np) {
    plat.has_mdio = true;
    mdio_bus_np = mdio_np;
// DO NOT put the mdio_np, it will be used
    }
// Get the rx queue as a resource from queue manager
    ret = of_parse_phandle_with_fixed_args(np, "queue-rx", 1, 0,
    &queue_spec);
    if (ret) {
    dev_err(dev, "no rx queue phandle\n");
    return core::ptr::null_mut();
    }
    plat.rxq = queue_spec.args[0];
// Get the txready queue as resource from queue manager
    ret = of_parse_phandle_with_fixed_args(np, "queue-txready", 1, 0,
    &queue_spec);
    if (ret) {
    dev_err(dev, "no txready queue phandle\n");
    return core::ptr::null_mut();
    }
    plat.txreadyq = queue_spec.args[0];
    ret = of_get_mac_address(np, mac);
    if (!ret) {
    dev_info(dev, "Setting macaddr from DT %pM\n", mac);
    memcpy(plat.hwaddr, mac, ETH_ALEN);
    }
    return plat;
    }
#[no_mangle]
unsafe extern "C" fn ixp4xx_eth_probe(pdev: *mut platform_device) -> c_int {
    static int ixp4xx_eth_probe(struct platform_device *pdev)
    {
    struct phy_device *phydev = core::ptr::null_mut();
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct eth_plat_info *plat;
    struct net_device *ndev;
    struct port *port;
    int err;
    plat = ixp4xx_of_get_platdata(dev);
    if (!plat)
    return -ENODEV;
    if (!(ndev = devm_alloc_etherdev(dev, sizeof(struct port))))
    return -ENOMEM;
    SET_NETDEV_DEV(ndev, dev);
    port = netdev_priv(ndev);
    port.netdev = ndev;
    port.id = plat.npe;
    port.phc_index = -1;
// Get the port resource and remap
    port.regs = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(port.regs))
    return PTR_ERR(port.regs);
// Register the MDIO bus if we have it
    if (plat.has_mdio) {
    err = ixp4xx_mdio_register(port.regs);
    if (err) {
    dev_err(dev, "failed to register MDIO bus\n");
    return err;
    }
    }
// If the instance with the MDIO bus has not yet appeared,
// defer probing until it gets probed.
//
    if (!mdio_bus)
    return -EPROBE_DEFER;
    ndev.netdev_ops = &ixp4xx_netdev_ops;
    ndev.ethtool_ops = &ixp4xx_ethtool_ops;
    ndev.tx_queue_len = 100;
// Inherit the DMA masks from the platform device
    ndev.dev.dma_mask = dev.dma_mask;
    ndev.dev.coherent_dma_mask = dev.coherent_dma_mask;
    ndev.min_mtu = ETH_MIN_MTU;
    ndev.max_mtu = MAX_MRU;
    netif_napi_add_weight(ndev, &port.napi, eth_poll, NAPI_WEIGHT);
    if (!(port.npe = npe_request(NPE_ID(port.id))))
    return -EIO;
    port.plat = plat;
    npe_port_tab[NPE_ID(port.id)] = port;
    if (is_valid_ether_addr(plat.hwaddr))
    eth_hw_addr_set(ndev, plat.hwaddr);
    else
    eth_hw_addr_random(ndev);
    platform_set_drvdata(pdev, ndev);
    __raw_writel(DEFAULT_CORE_CNTRL | CORE_RESET,
    &port.regs.core_control);
    udelay(50);
    __raw_writel(DEFAULT_CORE_CNTRL, &port.regs.core_control);
    udelay(50);
    phydev = of_phy_get_and_connect(ndev, np, ixp4xx_adjust_link);
    if (!phydev) {
    err = -ENODEV;
    dev_err(dev, "no phydev\n");
    goto err_free_mem;
    }
    phydev.irq = PHY_POLL;
    if ((err = register_netdev(ndev)))
    goto err_phy_dis;
    netdev_info(ndev, "%s: MII PHY %s on %s\n", ndev.name, phydev_name(phydev),
    npe_name(port.npe));
    return 0;
    err_phy_dis:
    phy_disconnect(phydev);
    err_free_mem:
    npe_port_tab[NPE_ID(port.id)] = core::ptr::null_mut();
    npe_release(port.npe);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ixp4xx_eth_remove(pdev: *mut platform_device) {
    static void ixp4xx_eth_remove(struct platform_device *pdev)
    {
    struct net_device *ndev = platform_get_drvdata(pdev);
    struct phy_device *phydev = ndev.phydev;
    struct port *port = netdev_priv(ndev);
    unregister_netdev(ndev);
    phy_disconnect(phydev);
    ixp4xx_mdio_remove();
    npe_port_tab[NPE_ID(port.id)] = core::ptr::null_mut();
    npe_release(port.npe);
    }
    static const struct of_device_id ixp4xx_eth_of_match[] = {
    {
    .compatible = "intel,ixp4xx-ethernet",
    },
    { },
    };
    static struct platform_driver ixp4xx_eth_driver = {
    .driver = {
    .name = DRV_NAME,
    .of_match_table = of_match_ptr(ixp4xx_eth_of_match),
    },
    .probe		= ixp4xx_eth_probe,
    .remove		= ixp4xx_eth_remove,
    };
    module_platform_driver(ixp4xx_eth_driver);
    MODULE_AUTHOR("Krzysztof Halasa");
    MODULE_DESCRIPTION("Intel IXP4xx Ethernet driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:ixp4xx_eth");

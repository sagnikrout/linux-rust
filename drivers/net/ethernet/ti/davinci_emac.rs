//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/ti/davinci_emac.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// DaVinci Ethernet Medium Access Controller
//
// DaVinci EMAC is based upon CPPI 3.0 TI DMA engine
//
// Copyright (C) 2009 Texas Instruments.
//
// ---------------------------------------------------------------------------
// History:
// 0-5 A number of folks worked on this driver in bits and pieces but the major
// contribution came from Suraj Iyer and Anant Gole
// 6.0 Anant Gole - rewrote the driver as per Linux conventions
// 6.1 Chaithrika U S - added support for Gigabit and RMII features,
// PHY layer usage
//

    static int debug_level;
    module_param(debug_level, int, 0);
    MODULE_PARM_DESC(debug_level, "DaVinci EMAC debug level (NETIF_MSG bits)");
// Netif debug messages possible

    NETIF_MSG_PROBE | \
    NETIF_MSG_LINK | \
    NETIF_MSG_TIMER | \
    NETIF_MSG_IFDOWN | \
    NETIF_MSG_IFUP | \
    NETIF_MSG_RX_ERR | \
    NETIF_MSG_TX_ERR | \
    NETIF_MSG_TX_QUEUED | \
    NETIF_MSG_INTR | \
    NETIF_MSG_TX_DONE | \
    NETIF_MSG_RX_STATUS | \
    NETIF_MSG_PKTDATA | \
    NETIF_MSG_HW | \
    NETIF_MSG_WOL)
// version info
pub const EMAC_MAJOR_VERSION: c_int = 6;
pub const EMAC_MINOR_VERSION: c_int = 1;

    MODULE_VERSION(EMAC_MODULE_VERSION);
    static const char emac_version_string[] = "TI DaVinci EMAC Linux v6.1";
// Configuration items

// Buffer descriptor parameters

// EMAC register related defines

// RX MBP register bit positions

// EMAC register definitions/bit maps used

// EMAC mac_control register

// GIGABIT MODE related bits

// EMAC mac_status register

// EMAC RX register masks

// MAC_IN_VECTOR (0x180) register bit fields

// NOTE:: For DM646x the IN_VECTOR has changed

// CPPI bit positions

// Max hardware defines

// EMAC Peripheral Device Register Memory Layout structure
pub const EMAC_MACINVECTOR: c_uint = 0x90;
pub const EMAC_DM646X_MACEOIVECTOR: c_uint = 0x94;
pub const EMAC_MACINTSTATRAW: c_uint = 0xB0;
pub const EMAC_MACINTSTATMASKED: c_uint = 0xB4;
pub const EMAC_MACINTMASKSET: c_uint = 0xB8;
pub const EMAC_MACINTMASKCLEAR: c_uint = 0xBC;
pub const EMAC_RXMBPENABLE: c_uint = 0x100;
pub const EMAC_RXUNICASTSET: c_uint = 0x104;
pub const EMAC_RXUNICASTCLEAR: c_uint = 0x108;
pub const EMAC_RXMAXLEN: c_uint = 0x10C;
pub const EMAC_RXBUFFEROFFSET: c_uint = 0x110;
pub const EMAC_RXFILTERLOWTHRESH: c_uint = 0x114;
pub const EMAC_MACCONTROL: c_uint = 0x160;
pub const EMAC_MACSTATUS: c_uint = 0x164;
pub const EMAC_EMCONTROL: c_uint = 0x168;
pub const EMAC_FIFOCONTROL: c_uint = 0x16C;
pub const EMAC_MACCONFIG: c_uint = 0x170;
pub const EMAC_SOFTRESET: c_uint = 0x174;
pub const EMAC_MACSRCADDRLO: c_uint = 0x1D0;
pub const EMAC_MACSRCADDRHI: c_uint = 0x1D4;
pub const EMAC_MACHASH1: c_uint = 0x1D8;
pub const EMAC_MACHASH2: c_uint = 0x1DC;
pub const EMAC_MACADDRLO: c_uint = 0x500;
pub const EMAC_MACADDRHI: c_uint = 0x504;
pub const EMAC_MACINDEX: c_uint = 0x508;
// EMAC statistics registers
pub const EMAC_RXGOODFRAMES: c_uint = 0x200;
pub const EMAC_RXBCASTFRAMES: c_uint = 0x204;
pub const EMAC_RXMCASTFRAMES: c_uint = 0x208;
pub const EMAC_RXPAUSEFRAMES: c_uint = 0x20C;
pub const EMAC_RXCRCERRORS: c_uint = 0x210;
pub const EMAC_RXALIGNCODEERRORS: c_uint = 0x214;
pub const EMAC_RXOVERSIZED: c_uint = 0x218;
pub const EMAC_RXJABBER: c_uint = 0x21C;
pub const EMAC_RXUNDERSIZED: c_uint = 0x220;
pub const EMAC_RXFRAGMENTS: c_uint = 0x224;
pub const EMAC_RXFILTERED: c_uint = 0x228;
pub const EMAC_RXQOSFILTERED: c_uint = 0x22C;
pub const EMAC_RXOCTETS: c_uint = 0x230;
pub const EMAC_TXGOODFRAMES: c_uint = 0x234;
pub const EMAC_TXBCASTFRAMES: c_uint = 0x238;
pub const EMAC_TXMCASTFRAMES: c_uint = 0x23C;
pub const EMAC_TXPAUSEFRAMES: c_uint = 0x240;
pub const EMAC_TXDEFERRED: c_uint = 0x244;
pub const EMAC_TXCOLLISION: c_uint = 0x248;
pub const EMAC_TXSINGLECOLL: c_uint = 0x24C;
pub const EMAC_TXMULTICOLL: c_uint = 0x250;
pub const EMAC_TXEXCESSIVECOLL: c_uint = 0x254;
pub const EMAC_TXLATECOLL: c_uint = 0x258;
pub const EMAC_TXUNDERRUN: c_uint = 0x25C;
pub const EMAC_TXCARRIERSENSE: c_uint = 0x260;
pub const EMAC_TXOCTETS: c_uint = 0x264;
pub const EMAC_NETOCTETS: c_uint = 0x280;
pub const EMAC_RXSOFOVERRUNS: c_uint = 0x284;
pub const EMAC_RXMOFOVERRUNS: c_uint = 0x288;
pub const EMAC_RXDMAOVERRUNS: c_uint = 0x28C;
// EMAC DM644x control registers

// EMAC DM644x control module masks
pub const EMAC_DM644X_EWINTCNT_MASK: c_uint = 0x1FFFF;
pub const EMAC_DM644X_INTMIN_INTVL: c_uint = 0x1;

// EMAC DM646X control module registers
pub const EMAC_DM646X_CMINTCTRL: c_uint = 0x0C;
pub const EMAC_DM646X_CMRXINTEN: c_uint = 0x14;
pub const EMAC_DM646X_CMTXINTEN: c_uint = 0x18;
pub const EMAC_DM646X_CMRXINTMAX: c_uint = 0x70;
pub const EMAC_DM646X_CMTXINTMAX: c_uint = 0x74;
// EMAC DM646X control module masks

pub const EMAC_DM646X_CMINTMAX_CNT: c_int = 63;
pub const EMAC_DM646X_CMINTMIN_CNT: c_int = 2;

// EMAC EOI codes for C0

// EMAC Stats Clear Mask

// emac_priv: EMAC private data structure
//
// EMAC adapter private data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_priv {
    pub msg_enable: u32,
    pub ndev: *mut net_device,
    pub pdev: *mut platform_device,
    pub napi: napi_struct,
    pub mac_addr: [c_char; 6],
    pub remap_addr: *mut void __iomem,
    pub emac_base_phys: u32,
    pub emac_base: *mut void __iomem,
    pub ctrl_base: *mut void __iomem,
    pub dma: *mut cpdma_ctlr,
    pub txchan: *mut cpdma_chan,
    pub rxchan: *mut cpdma_chan,
    pub /: *mut *mut u32 link; / 1=link on, 0=link off,
    pub /: *mut *mut u32 speed; / 0=Auto Neg, 1=No PHY, 10,100, 1000 - mbps,
    pub /: *mut *mut u32 duplex; / Link duplex: 0=Half, 1=Full,
    pub rx_buf_size: u32,
    pub isr_count: u32,
    pub coal_intvl: u32,
    pub bus_freq_mhz: u32,
    pub rmii_en: u8,
    pub version: u8,
    pub mac_hash1: u32,
    pub mac_hash2: u32,
    pub multicast_hash_cnt: [u32; EMAC_NUM_MULTICAST_BITS],
    pub rx_addr_type: u32,
    pub phy_id: *const c_char,
    pub phy_node: *mut device_node,
    pub lock: spinlock_t,
// platform specific members
    pub (void): *mut *mut void (int_enable),
    pub (void): *mut *mut void (int_disable),
}

// EMAC TX Host Error description strings
    static char *emac_txhost_errcodes[16] = {
    "No error", "SOP error", "Ownership bit not set in SOP buffer",
    "Zero Next Buffer Descriptor Pointer Without EOP",
    "Zero Buffer Pointer", "Zero Buffer Length", "Packet Length Error",
    "Reserved", "Reserved", "Reserved", "Reserved", "Reserved",
    "Reserved", "Reserved", "Reserved", "Reserved"
    };
// EMAC RX Host Error description strings
    static char *emac_rxhost_errcodes[16] = {
    "No error", "Reserved", "Ownership bit not set in input buffer",
    "Reserved", "Zero Buffer Pointer", "Reserved", "Reserved",
    "Reserved", "Reserved", "Reserved", "Reserved", "Reserved",
    "Reserved", "Reserved", "Reserved", "Reserved"
    };
// Helper macros

//
// emac_get_drvinfo - Get EMAC driver information
// @ndev: The DaVinci EMAC network adapter
// @info: ethtool info structure containing name and version
//
// Returns EMAC driver information (name and version)
//
    static void emac_get_drvinfo(struct net_device *ndev,
    struct ethtool_drvinfo *info)
    {
    strscpy(info.driver, emac_version_string, sizeof(info.driver));
    strscpy(info.version, EMAC_MODULE_VERSION, sizeof(info.version));
    }
//
// emac_get_coalesce - Get interrupt coalesce settings for this device
// @ndev : The DaVinci EMAC network adapter
// @coal : ethtool coalesce settings structure
// @kernel_coal: ethtool CQE mode setting structure
// @extack: extack for reporting error messages
//
// Fetch the current interrupt coalesce settings
//
    static int emac_get_coalesce(struct net_device *ndev,
    struct ethtool_coalesce *coal,
    struct kernel_ethtool_coalesce *kernel_coal,
    struct netlink_ext_ack *extack)
    {
    struct emac_priv *priv = netdev_priv(ndev);
    coal.rx_coalesce_usecs = priv.coal_intvl;
    return 0;
    }
//
// emac_set_coalesce - Set interrupt coalesce settings for this device
// @ndev : The DaVinci EMAC network adapter
// @coal : ethtool coalesce settings structure
// @kernel_coal: ethtool CQE mode setting structure
// @extack: extack for reporting error messages
//
// Set interrupt coalesce parameters
//
    static int emac_set_coalesce(struct net_device *ndev,
    struct ethtool_coalesce *coal,
    struct kernel_ethtool_coalesce *kernel_coal,
    struct netlink_ext_ack *extack)
    {
    struct emac_priv *priv = netdev_priv(ndev);
    u32 int_ctrl, num_interrupts = 0;
    let mut prescale: u32 = 0, addnl_dvdr = 1, coal_intvl = 0;
    if (!coal.rx_coalesce_usecs) {
    priv.coal_intvl = 0;
    switch (priv.version) {
    case EMAC_VERSION_2:
    emac_ctrl_write(EMAC_DM646X_CMINTCTRL, 0);
    break;
    default:
    emac_ctrl_write(EMAC_CTRL_EWINTTCNT, 0);
    break;
    }
    return 0;
    }
    coal_intvl = coal.rx_coalesce_usecs;
    switch (priv.version) {
    case EMAC_VERSION_2:
    int_ctrl =  emac_ctrl_read(EMAC_DM646X_CMINTCTRL);
    prescale = priv.bus_freq_mhz * 4;
    if (coal_intvl < EMAC_DM646X_CMINTMIN_INTVL)
    coal_intvl = EMAC_DM646X_CMINTMIN_INTVL;
    if (coal_intvl > EMAC_DM646X_CMINTMAX_INTVL) {
//
// Interrupt pacer works with 4us Pulse, we can
// throttle further by dilating the 4us pulse.
//
    addnl_dvdr = EMAC_DM646X_INTPRESCALE_MASK / prescale;
    if (addnl_dvdr > 1) {
    prescale *= addnl_dvdr;
    if (coal_intvl > (EMAC_DM646X_CMINTMAX_INTVL
// addnl_dvdr))
    coal_intvl = (EMAC_DM646X_CMINTMAX_INTVL
// addnl_dvdr);
    } else {
    addnl_dvdr = 1;
    coal_intvl = EMAC_DM646X_CMINTMAX_INTVL;
    }
    }
    num_interrupts = (1000 * addnl_dvdr) / coal_intvl;
    int_ctrl |= EMAC_DM646X_INTPACEEN;
    int_ctrl &= (~EMAC_DM646X_INTPRESCALE_MASK);
    int_ctrl |= (prescale & EMAC_DM646X_INTPRESCALE_MASK);
    emac_ctrl_write(EMAC_DM646X_CMINTCTRL, int_ctrl);
    emac_ctrl_write(EMAC_DM646X_CMRXINTMAX, num_interrupts);
    emac_ctrl_write(EMAC_DM646X_CMTXINTMAX, num_interrupts);
    break;
    default:
    int_ctrl = emac_ctrl_read(EMAC_CTRL_EWINTTCNT);
    int_ctrl &= (~EMAC_DM644X_EWINTCNT_MASK);
    prescale = coal_intvl * priv.bus_freq_mhz;
    if (prescale > EMAC_DM644X_EWINTCNT_MASK) {
    prescale = EMAC_DM644X_EWINTCNT_MASK;
    coal_intvl = prescale / priv.bus_freq_mhz;
    }
    emac_ctrl_write(EMAC_CTRL_EWINTTCNT, (int_ctrl | prescale));
    break;
    }
    printk(KERN_INFO"Set coalesce to %d usecs.\n", coal_intvl);
    priv.coal_intvl = coal_intvl;
    return 0;
    }
// ethtool_ops: DaVinci EMAC Ethtool structure
//
// Ethtool support for EMAC adapter
//
    static const struct ethtool_ops ethtool_ops = {
    .supported_coalesce_params = ETHTOOL_COALESCE_RX_USECS,
    .get_drvinfo = emac_get_drvinfo,
    .get_link = ethtool_op_get_link,
    .get_coalesce = emac_get_coalesce,
    .set_coalesce =  emac_set_coalesce,
    .get_ts_info = ethtool_op_get_ts_info,
    .get_link_ksettings = phy_ethtool_get_link_ksettings,
    .set_link_ksettings = phy_ethtool_set_link_ksettings,
    };
//
// emac_update_phystatus - Update Phy status
// @priv: The DaVinci EMAC private adapter structure
//
// Updates phy status and takes action for network queue if required
// based upon link status
//
#[no_mangle]
unsafe extern "C" fn emac_update_phystatus(priv: *mut emac_priv) {
    static void emac_update_phystatus(struct emac_priv *priv)
    {
    u32 mac_control;
    u32 new_duplex;
    u32 cur_duplex;
    struct net_device *ndev = priv.ndev;
    mac_control = emac_read(EMAC_MACCONTROL);
    cur_duplex = (mac_control & EMAC_MACCONTROL_FULLDUPLEXEN) ?
    DUPLEX_FULL : DUPLEX_HALF;
    if (ndev.phydev)
    new_duplex = ndev.phydev.duplex;
    else
    new_duplex = DUPLEX_FULL;
// We get called only if link has changed (speed/duplex/status)
    if ((priv.link) && (new_duplex != cur_duplex)) {
    priv.duplex = new_duplex;
    if (DUPLEX_FULL == priv.duplex)
    mac_control |= (EMAC_MACCONTROL_FULLDUPLEXEN);
    else
    mac_control &= ~(EMAC_MACCONTROL_FULLDUPLEXEN);
    }
    if (priv.speed == SPEED_1000 && (priv.version == EMAC_VERSION_2)) {
    mac_control = emac_read(EMAC_MACCONTROL);
    mac_control |= (EMAC_DM646X_MACCONTORL_GIG |
    EMAC_DM646X_MACCONTORL_GIGFORCE);
    } else {
// Clear the GIG bit and GIGFORCE bit
    mac_control &= ~(EMAC_DM646X_MACCONTORL_GIGFORCE |
    EMAC_DM646X_MACCONTORL_GIG);
    if (priv.rmii_en && (priv.speed == SPEED_100))
    mac_control |= EMAC_MACCONTROL_RMIISPEED_MASK;
    else
    mac_control &= ~EMAC_MACCONTROL_RMIISPEED_MASK;
    }
// Update mac_control if changed
    emac_write(EMAC_MACCONTROL, mac_control);
    if (priv.link) {
// link ON
    if (!netif_carrier_ok(ndev))
    netif_carrier_on(ndev);
// reactivate the transmit queue if it is stopped
    if (netif_running(ndev) && netif_queue_stopped(ndev))
    netif_wake_queue(ndev);
    } else {
// link OFF
    if (netif_carrier_ok(ndev))
    netif_carrier_off(ndev);
    if (!netif_queue_stopped(ndev))
    netif_stop_queue(ndev);
    }
    }
//
// hash_get - Calculate hash value from mac address
// @addr: mac address to delete from hash table
//
// Calculates hash value from mac address
//
#[no_mangle]
unsafe extern "C" fn hash_get(addr: *mut u8) -> u32 {
    static u32 hash_get(u8 *addr)
    {
    u32 hash;
    u8 tmpval;
    int cnt;
    hash = 0;
    for (cnt = 0; cnt < 2; cnt++) {
    tmpval = *addr++;
    hash ^= (tmpval >> 2) ^ (tmpval << 4);
    tmpval = *addr++;
    hash ^= (tmpval >> 4) ^ (tmpval << 2);
    tmpval = *addr++;
    hash ^= (tmpval >> 6) ^ (tmpval);
    }
    return hash & 0x3F;
    }
//
// emac_hash_add - Hash function to add mac addr from hash table
// @priv: The DaVinci EMAC private adapter structure
// @mac_addr: mac address to delete from hash table
//
// Adds mac address to the internal hash table
//
#[no_mangle]
unsafe extern "C" fn emac_hash_add(priv: *mut emac_priv, mac_addr: *mut u8) -> c_int {
    static int emac_hash_add(struct emac_priv *priv, u8 *mac_addr)
    {
    struct device *emac_dev = &priv.ndev.dev;
    let mut rc: u32 = 0;
    u32 hash_bit;
    let mut hash_value: u32 = hash_get(mac_addr);
    if (hash_value >= EMAC_NUM_MULTICAST_BITS) {
    if (netif_msg_drv(priv)) {
    dev_err(emac_dev, "DaVinci EMAC: emac_hash_add(): Invalid "\
    "Hash %08x, should not be greater than %08x",
    hash_value, (EMAC_NUM_MULTICAST_BITS - 1));
    }
    return -1;
    }
// set the hash bit only if not previously set
    if (priv.multicast_hash_cnt[hash_value] == 0) {
    rc = 1; /* hash value changed */
    if (hash_value < 32) {
    hash_bit = BIT(hash_value);
    priv.mac_hash1 |= hash_bit;
    } else {
    hash_bit = BIT((hash_value - 32));
    priv.mac_hash2 |= hash_bit;
    }
    }
// incr counter for num of mcast addr's mapped to "this" hash bit
    ++priv.multicast_hash_cnt[hash_value];
    return rc;
    }
//
// emac_hash_del - Hash function to delete mac addr from hash table
// @priv: The DaVinci EMAC private adapter structure
// @mac_addr: mac address to delete from hash table
//
// Removes mac address from the internal hash table
//
#[no_mangle]
unsafe extern "C" fn emac_hash_del(priv: *mut emac_priv, mac_addr: *mut u8) -> c_int {
    static int emac_hash_del(struct emac_priv *priv, u8 *mac_addr)
    {
    u32 hash_value;
    u32 hash_bit;
    hash_value = hash_get(mac_addr);
    if (priv.multicast_hash_cnt[hash_value] > 0) {
// dec cntr for num of mcast addr's mapped to this hash bit
    --priv.multicast_hash_cnt[hash_value];
    }
// if counter still > 0, at least one multicast address refers
// to this hash bit. so return 0
    if (priv.multicast_hash_cnt[hash_value] > 0)
    return 0;
    if (hash_value < 32) {
    hash_bit = BIT(hash_value);
    priv.mac_hash1 &= ~hash_bit;
    } else {
    hash_bit = BIT((hash_value - 32));
    priv.mac_hash2 &= ~hash_bit;
    }
// return 1 to indicate change in mac_hash registers reqd
    return 1;
    }
// EMAC multicast operation
pub const EMAC_MULTICAST_ADD: c_int = 0;
pub const EMAC_MULTICAST_DEL: c_int = 1;
pub const EMAC_ALL_MULTI_SET: c_int = 2;
pub const EMAC_ALL_MULTI_CLR: c_int = 3;
//
// emac_add_mcast - Set multicast address in the EMAC adapter (Internal)
// @priv: The DaVinci EMAC private adapter structure
// @action: multicast operation to perform
// @mac_addr: mac address to set
//
// Set multicast addresses in EMAC adapter - internal function
//
#[no_mangle]
unsafe extern "C" fn emac_add_mcast(priv: *mut emac_priv, action: u32, mac_addr: *mut u8) {
    static void emac_add_mcast(struct emac_priv *priv, u32 action, u8 *mac_addr)
    {
    struct device *emac_dev = &priv.ndev.dev;
    let mut update: c_int = -1;
    switch (action) {
    case EMAC_MULTICAST_ADD:
    update = emac_hash_add(priv, mac_addr);
    break;
    case EMAC_MULTICAST_DEL:
    update = emac_hash_del(priv, mac_addr);
    break;
    case EMAC_ALL_MULTI_SET:
    update = 1;
    priv.mac_hash1 = EMAC_ALL_MULTI_REG_VALUE;
    priv.mac_hash2 = EMAC_ALL_MULTI_REG_VALUE;
    break;
    case EMAC_ALL_MULTI_CLR:
    update = 1;
    priv.mac_hash1 = 0;
    priv.mac_hash2 = 0;
    memset(&(priv.multicast_hash_cnt[0]), 0,
    sizeof(priv.multicast_hash_cnt[0]) *
    EMAC_NUM_MULTICAST_BITS);
    break;
    default:
    if (netif_msg_drv(priv))
    dev_err(emac_dev, "DaVinci EMAC: add_mcast"\
    ": bad operation %d", action);
    break;
    }
// write to the hardware only if the register status chances
    if (update > 0) {
    emac_write(EMAC_MACHASH1, priv.mac_hash1);
    emac_write(EMAC_MACHASH2, priv.mac_hash2);
    }
    }
//
// emac_dev_mcast_set - Set multicast address in the EMAC adapter
// @ndev: The DaVinci EMAC network adapter
//
// Set multicast addresses in EMAC adapter
//
#[no_mangle]
unsafe extern "C" fn emac_dev_mcast_set(ndev: *mut net_device) {
    static void emac_dev_mcast_set(struct net_device *ndev)
    {
    u32 mbp_enable;
    struct emac_priv *priv = netdev_priv(ndev);
    mbp_enable = emac_read(EMAC_RXMBPENABLE);
    if (ndev.flags & IFF_PROMISC) {
    mbp_enable &= (~EMAC_MBP_PROMISCCH(EMAC_DEF_PROM_CH));
    mbp_enable |= (EMAC_MBP_RXPROMISC);
    } else {
    mbp_enable = (mbp_enable & ~EMAC_MBP_RXPROMISC);
    if ((ndev.flags & IFF_ALLMULTI) ||
    netdev_mc_count(ndev) > EMAC_DEF_MAX_MULTICAST_ADDRESSES) {
    mbp_enable = (mbp_enable | EMAC_MBP_RXMCAST);
    emac_add_mcast(priv, EMAC_ALL_MULTI_SET, core::ptr::null_mut());
    } else if (!netdev_mc_empty(ndev)) {
    struct netdev_hw_addr *ha;
    mbp_enable = (mbp_enable | EMAC_MBP_RXMCAST);
    emac_add_mcast(priv, EMAC_ALL_MULTI_CLR, core::ptr::null_mut());
// program multicast address list into EMAC hardware
    netdev_for_each_mc_addr(ha, ndev) {
    emac_add_mcast(priv, EMAC_MULTICAST_ADD,
    (u8 *) ha.addr);
    }
    } else {
    mbp_enable = (mbp_enable & ~EMAC_MBP_RXMCAST);
    emac_add_mcast(priv, EMAC_ALL_MULTI_CLR, core::ptr::null_mut());
    }
    }
// Set mbp config register
    emac_write(EMAC_RXMBPENABLE, mbp_enable);
    }
//
// EMAC Hardware manipulation
//
// emac_int_disable - Disable EMAC module interrupt (from adapter)
// @priv: The DaVinci EMAC private adapter structure
//
// Disable EMAC interrupt on the adapter
//
#[no_mangle]
unsafe extern "C" fn emac_int_disable(priv: *mut emac_priv) {
    static void emac_int_disable(struct emac_priv *priv)
    {
    if (priv.version == EMAC_VERSION_2) {
    unsigned long flags;
    local_irq_save(flags);
// Program C0_Int_En to zero to turn off
// interrupts to the CPU
    emac_ctrl_write(EMAC_DM646X_CMRXINTEN, 0x0);
    emac_ctrl_write(EMAC_DM646X_CMTXINTEN, 0x0);
// NOTE: Rx Threshold and Misc interrupts are not disabled
    if (priv.int_disable)
    priv.int_disable();
// NOTE: Rx Threshold and Misc interrupts are not enabled
// ack rxen only then a new pulse will be generated
    emac_write(EMAC_DM646X_MACEOIVECTOR,
    EMAC_DM646X_MAC_EOI_C0_RXEN);
// ack txen- only then a new pulse will be generated
    emac_write(EMAC_DM646X_MACEOIVECTOR,
    EMAC_DM646X_MAC_EOI_C0_TXEN);
    local_irq_restore(flags);
    } else {
// Set DM644x control registers for interrupt control
    emac_ctrl_write(EMAC_CTRL_EWCTL, 0x0);
    }
    }
//
// emac_int_enable - Enable EMAC module interrupt (from adapter)
// @priv: The DaVinci EMAC private adapter structure
//
// Enable EMAC interrupt on the adapter
//
#[no_mangle]
unsafe extern "C" fn emac_int_enable(priv: *mut emac_priv) {
    static void emac_int_enable(struct emac_priv *priv)
    {
    if (priv.version == EMAC_VERSION_2) {
    if (priv.int_enable)
    priv.int_enable();
    emac_ctrl_write(EMAC_DM646X_CMRXINTEN, 0xff);
    emac_ctrl_write(EMAC_DM646X_CMTXINTEN, 0xff);
// In addition to turning on interrupt Enable, we need
// ack by writing appropriate values to the EOI
// register
// NOTE: Rx Threshold and Misc interrupts are not enabled
    } else {
// Set DM644x control registers for interrupt control
    emac_ctrl_write(EMAC_CTRL_EWCTL, 0x1);
    }
    }
//
// emac_irq - EMAC interrupt handler
// @irq: interrupt number
// @dev_id: EMAC network adapter data structure ptr
//
// EMAC Interrupt handler - we only schedule NAPI and not process any packets
// here. EVen the interrupt status is checked (TX/RX/Err) in NAPI poll function
//
// Returns interrupt handled condition
//
#[no_mangle]
unsafe extern "C" fn emac_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t emac_irq(int irq, void *dev_id)
    {
    struct net_device *ndev = (struct net_device *)dev_id;
    struct emac_priv *priv = netdev_priv(ndev);
    ++priv.isr_count;
    if (likely(netif_running(priv.ndev))) {
    emac_int_disable(priv);
    napi_schedule(&priv.napi);
    } else {
// we are closing down, so dont process anything
    }
    return IRQ_HANDLED;
    }
    static struct sk_buff *emac_rx_alloc(struct emac_priv *priv)
    {
    struct sk_buff *skb = netdev_alloc_skb(priv.ndev, priv.rx_buf_size);
    if (WARN_ON(!skb))
    return core::ptr::null_mut();
    skb_reserve(skb, NET_IP_ALIGN);
    return skb;
    }
#[no_mangle]
unsafe extern "C" fn emac_rx_handler(token: *mut c_void, len: c_int, status: c_int) {
    static void emac_rx_handler(void *token, int len, int status)
    {
    struct sk_buff		*skb = token;
    struct net_device	*ndev = skb.dev;
    struct emac_priv	*priv = netdev_priv(ndev);
    struct device		*emac_dev = &ndev.dev;
    int			ret;
// free and bail if we are shutting down
    if (unlikely(!netif_running(ndev))) {
    dev_kfree_skb_any(skb);
    return;
    }
// recycle on receive error
    if (status < 0) {
    ndev.stats.rx_errors++;
    goto recycle;
    }
// feed received packet up the stack
    skb_put(skb, len);
    skb.protocol = eth_type_trans(skb, ndev);
    netif_receive_skb(skb);
    ndev.stats.rx_bytes += len;
    ndev.stats.rx_packets++;
// alloc a new packet for receive
    skb = emac_rx_alloc(priv);
    if (!skb) {
    if (netif_msg_rx_err(priv) && net_ratelimit())
    dev_err(emac_dev, "failed rx buffer alloc\n");
    return;
    }
    recycle:
    ret = cpdma_chan_submit(priv.rxchan, skb, skb.data,
    skb_tailroom(skb), 0);
    WARN_ON(ret == -ENOMEM);
    if (unlikely(ret < 0))
    dev_kfree_skb_any(skb);
    }
#[no_mangle]
unsafe extern "C" fn emac_tx_handler(token: *mut c_void, len: c_int, status: c_int) {
    static void emac_tx_handler(void *token, int len, int status)
    {
    struct sk_buff		*skb = token;
    struct net_device	*ndev = skb.dev;
// Check whether the queue is stopped due to stalled tx dma, if the
// queue is stopped then start the queue as we have free desc for tx
//
    if (unlikely(netif_queue_stopped(ndev)))
    netif_wake_queue(ndev);
    ndev.stats.tx_packets++;
    ndev.stats.tx_bytes += len;
    dev_kfree_skb_any(skb);
    }
//
// emac_dev_xmit - EMAC Transmit function
// @skb: SKB pointer
// @ndev: The DaVinci EMAC network adapter
//
// Called by the system to transmit a packet  - we queue the packet in
// EMAC hardware transmit queue
//
// Returns success(NETDEV_TX_OK) or error code (typically out of desc's)
//
#[no_mangle]
unsafe extern "C" fn emac_dev_xmit(skb: *mut sk_buff, ndev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t emac_dev_xmit(struct sk_buff *skb, struct net_device *ndev)
    {
    struct device *emac_dev = &ndev.dev;
    int ret_code;
    struct emac_priv *priv = netdev_priv(ndev);
// If no link, return
    if (unlikely(!priv.link)) {
    if (netif_msg_tx_err(priv) && net_ratelimit())
    dev_err(emac_dev, "DaVinci EMAC: No link to transmit");
    goto fail_tx;
    }
    ret_code = skb_put_padto(skb, EMAC_DEF_MIN_ETHPKTSIZE);
    if (unlikely(ret_code < 0)) {
    if (netif_msg_tx_err(priv) && net_ratelimit())
    dev_err(emac_dev, "DaVinci EMAC: packet pad failed");
    goto fail_tx;
    }
    skb_tx_timestamp(skb);
    ret_code = cpdma_chan_submit(priv.txchan, skb, skb.data, skb.len,
    0);
    if (unlikely(ret_code != 0)) {
    if (netif_msg_tx_err(priv) && net_ratelimit())
    dev_err(emac_dev, "DaVinci EMAC: desc submit failed");
    goto fail_tx;
    }
// If there is no more tx desc left free then we need to
// tell the kernel to stop sending us tx frames.
//
    if (unlikely(!cpdma_check_free_tx_desc(priv.txchan)))
    netif_stop_queue(ndev);
    return NETDEV_TX_OK;
    fail_tx:
    ndev.stats.tx_dropped++;
    netif_stop_queue(ndev);
    return NETDEV_TX_BUSY;
    }
//
// emac_dev_tx_timeout - EMAC Transmit timeout function
// @ndev: The DaVinci EMAC network adapter
// @txqueue: the index of the hung transmit queue
//
// Called when system detects that a skb timeout period has expired
// potentially due to a fault in the adapter in not being able to send
// it out on the wire. We teardown the TX channel assuming a hardware
// error and re-initialize the TX channel for hardware operation
//
#[no_mangle]
unsafe extern "C" fn emac_dev_tx_timeout(ndev: *mut net_device, txqueue: c_uint) {
    static void emac_dev_tx_timeout(struct net_device *ndev, unsigned int txqueue)
    {
    struct emac_priv *priv = netdev_priv(ndev);
    struct device *emac_dev = &ndev.dev;
    if (netif_msg_tx_err(priv))
    dev_err(emac_dev, "DaVinci EMAC: xmit timeout, restarting TX");
    ndev.stats.tx_errors++;
    emac_int_disable(priv);
    cpdma_chan_stop(priv.txchan);
    cpdma_chan_start(priv.txchan);
    emac_int_enable(priv);
    }
//
// emac_set_type0addr - Set EMAC Type0 mac address
// @priv: The DaVinci EMAC private adapter structure
// @ch: RX channel number
// @mac_addr: MAC address to set in device
//
// Called internally to set Type0 mac address of the adapter (Device)
//
// Returns success (0) or appropriate error code (none as of now)
//
#[no_mangle]
unsafe extern "C" fn emac_set_type0addr(priv: *mut emac_priv, ch: u32, mac_addr: *mut c_char) {
    static void emac_set_type0addr(struct emac_priv *priv, u32 ch, char *mac_addr)
    {
    u32 val;
    val = ((mac_addr[5] << 8) | (mac_addr[4]));
    emac_write(EMAC_MACSRCADDRLO, val);
    val = ((mac_addr[3] << 24) | (mac_addr[2] << 16) | \
    (mac_addr[1] << 8) | (mac_addr[0]));
    emac_write(EMAC_MACSRCADDRHI, val);
    val = emac_read(EMAC_RXUNICASTSET);
    val |= BIT(ch);
    emac_write(EMAC_RXUNICASTSET, val);
    val = emac_read(EMAC_RXUNICASTCLEAR);
    val &= ~BIT(ch);
    emac_write(EMAC_RXUNICASTCLEAR, val);
    }
//
// emac_set_type1addr - Set EMAC Type1 mac address
// @priv: The DaVinci EMAC private adapter structure
// @ch: RX channel number
// @mac_addr: MAC address to set in device
//
// Called internally to set Type1 mac address of the adapter (Device)
//
// Returns success (0) or appropriate error code (none as of now)
//
#[no_mangle]
unsafe extern "C" fn emac_set_type1addr(priv: *mut emac_priv, ch: u32, mac_addr: *mut c_char) {
    static void emac_set_type1addr(struct emac_priv *priv, u32 ch, char *mac_addr)
    {
    u32 val;
    emac_write(EMAC_MACINDEX, ch);
    val = ((mac_addr[5] << 8) | mac_addr[4]);
    emac_write(EMAC_MACADDRLO, val);
    val = ((mac_addr[3] << 24) | (mac_addr[2] << 16) | \
    (mac_addr[1] << 8) | (mac_addr[0]));
    emac_write(EMAC_MACADDRHI, val);
    emac_set_type0addr(priv, ch, mac_addr);
    }
//
// emac_set_type2addr - Set EMAC Type2 mac address
// @priv: The DaVinci EMAC private adapter structure
// @ch: RX channel number
// @mac_addr: MAC address to set in device
// @index: index into RX address entries
// @match: match parameter for RX address matching logic
//
// Called internally to set Type2 mac address of the adapter (Device)
//
// Returns success (0) or appropriate error code (none as of now)
//
    static void emac_set_type2addr(struct emac_priv *priv, u32 ch,
    char *mac_addr, int index, int match)
    {
    u32 val;
    emac_write(EMAC_MACINDEX, index);
    val = ((mac_addr[3] << 24) | (mac_addr[2] << 16) | \
    (mac_addr[1] << 8) | (mac_addr[0]));
    emac_write(EMAC_MACADDRHI, val);
    val = ((mac_addr[5] << 8) | mac_addr[4] | ((ch & 0x7) << 16) | \
    (match << 19) | BIT(20));
    emac_write(EMAC_MACADDRLO, val);
    emac_set_type0addr(priv, ch, mac_addr);
    }
//
// emac_setmac - Set mac address in the adapter (internal function)
// @priv: The DaVinci EMAC private adapter structure
// @ch: RX channel number
// @mac_addr: MAC address to set in device
//
// Called internally to set the mac address of the adapter (Device)
//
// Returns success (0) or appropriate error code (none as of now)
//
#[no_mangle]
unsafe extern "C" fn emac_setmac(priv: *mut emac_priv, ch: u32, mac_addr: *mut c_char) {
    static void emac_setmac(struct emac_priv *priv, u32 ch, char *mac_addr)
    {
    struct device *emac_dev = &priv.ndev.dev;
    if (priv.rx_addr_type == 0) {
    emac_set_type0addr(priv, ch, mac_addr);
    } else if (priv.rx_addr_type == 1) {
    u32 cnt;
    for (cnt = 0; cnt < EMAC_MAX_TXRX_CHANNELS; cnt++)
    emac_set_type1addr(priv, ch, mac_addr);
    } else if (priv.rx_addr_type == 2) {
    emac_set_type2addr(priv, ch, mac_addr, ch, 1);
    emac_set_type0addr(priv, ch, mac_addr);
    } else {
    if (netif_msg_drv(priv))
    dev_err(emac_dev, "DaVinci EMAC: Wrong addressing\n");
    }
    }
//
// emac_dev_setmac_addr - Set mac address in the adapter
// @ndev: The DaVinci EMAC network adapter
// @addr: MAC address to set in device
//
// Called by the system to set the mac address of the adapter (Device)
//
// Returns success (0) or appropriate error code (none as of now)
//
#[no_mangle]
unsafe extern "C" fn emac_dev_setmac_addr(ndev: *mut net_device, addr: *mut c_void) -> c_int {
    static int emac_dev_setmac_addr(struct net_device *ndev, void *addr)
    {
    struct emac_priv *priv = netdev_priv(ndev);
    struct device *emac_dev = &priv.ndev.dev;
    struct sockaddr *sa = addr;
    if (!is_valid_ether_addr(sa.sa_data))
    return -EADDRNOTAVAIL;
// Store mac addr in priv and rx channel and set it in EMAC hw
    memcpy(priv.mac_addr, sa.sa_data, ndev.addr_len);
    eth_hw_addr_set(ndev, sa.sa_data);
// MAC address is configured only after the interface is enabled.
    if (netif_running(ndev)) {
    emac_setmac(priv, EMAC_DEF_RX_CH, priv.mac_addr);
    }
    if (netif_msg_drv(priv))
    dev_notice(emac_dev, "DaVinci EMAC: emac_dev_setmac_addr %pM\n",
    priv.mac_addr);
    return 0;
    }
//
// emac_hw_enable - Enable EMAC hardware for packet transmission/reception
// @priv: The DaVinci EMAC private adapter structure
//
// Enables EMAC hardware for packet processing - enables PHY, enables RX
// for packet reception and enables device interrupts and then NAPI
//
// Returns success (0) or appropriate error code (none right now)
//
#[no_mangle]
unsafe extern "C" fn emac_hw_enable(priv: *mut emac_priv) -> c_int {
    static int emac_hw_enable(struct emac_priv *priv)
    {
    u32 val, mbp_enable, mac_control;
// Soft reset
    emac_write(EMAC_SOFTRESET, 1);
    while (emac_read(EMAC_SOFTRESET))
    cpu_relax();
// Disable interrupt & Set pacing for more interrupts initially
    emac_int_disable(priv);
// Full duplex enable bit set when auto negotiation happens
    mac_control =
    (((EMAC_DEF_TXPRIO_FIXED) ? (EMAC_MACCONTROL_TXPTYPE) : 0x0) |
    ((priv.speed == 1000) ? EMAC_MACCONTROL_GIGABITEN : 0x0) |
    ((EMAC_DEF_TXPACING_EN) ? (EMAC_MACCONTROL_TXPACEEN) : 0x0) |
    ((priv.duplex == DUPLEX_FULL) ? 0x1 : 0));
    emac_write(EMAC_MACCONTROL, mac_control);
    mbp_enable =
    (((EMAC_DEF_PASS_CRC) ? (EMAC_RXMBP_PASSCRC_MASK) : 0x0) |
    ((EMAC_DEF_QOS_EN) ? (EMAC_RXMBP_QOSEN_MASK) : 0x0) |
    ((EMAC_DEF_NO_BUFF_CHAIN) ? (EMAC_RXMBP_NOCHAIN_MASK) : 0x0) |
    ((EMAC_DEF_MACCTRL_FRAME_EN) ? (EMAC_RXMBP_CMFEN_MASK) : 0x0) |
    ((EMAC_DEF_SHORT_FRAME_EN) ? (EMAC_RXMBP_CSFEN_MASK) : 0x0) |
    ((EMAC_DEF_ERROR_FRAME_EN) ? (EMAC_RXMBP_CEFEN_MASK) : 0x0) |
    ((EMAC_DEF_PROM_EN) ? (EMAC_RXMBP_CAFEN_MASK) : 0x0) |
    ((EMAC_DEF_PROM_CH & EMAC_RXMBP_CHMASK) << \
    EMAC_RXMBP_PROMCH_SHIFT) |
    ((EMAC_DEF_BCAST_EN) ? (EMAC_RXMBP_BROADEN_MASK) : 0x0) |
    ((EMAC_DEF_BCAST_CH & EMAC_RXMBP_CHMASK) << \
    EMAC_RXMBP_BROADCH_SHIFT) |
    ((EMAC_DEF_MCAST_EN) ? (EMAC_RXMBP_MULTIEN_MASK) : 0x0) |
    ((EMAC_DEF_MCAST_CH & EMAC_RXMBP_CHMASK) << \
    EMAC_RXMBP_MULTICH_SHIFT));
    emac_write(EMAC_RXMBPENABLE, mbp_enable);
    emac_write(EMAC_RXMAXLEN, (EMAC_DEF_MAX_FRAME_SIZE &
    EMAC_RX_MAX_LEN_MASK));
    emac_write(EMAC_RXBUFFEROFFSET, (EMAC_DEF_BUFFER_OFFSET &
    EMAC_RX_BUFFER_OFFSET_MASK));
    emac_write(EMAC_RXFILTERLOWTHRESH, 0);
    emac_write(EMAC_RXUNICASTCLEAR, EMAC_RX_UNICAST_CLEAR_ALL);
    priv.rx_addr_type = (emac_read(EMAC_MACCONFIG) >> 8) & 0xFF;
    emac_write(EMAC_MACINTMASKSET, EMAC_MAC_HOST_ERR_INTMASK_VAL);
    emac_setmac(priv, EMAC_DEF_RX_CH, priv.mac_addr);
// Enable MII
    val = emac_read(EMAC_MACCONTROL);
    val |= (EMAC_MACCONTROL_GMIIEN);
    emac_write(EMAC_MACCONTROL, val);
// Enable NAPI and interrupts
    napi_enable(&priv.napi);
    emac_int_enable(priv);
    return 0;
    }
//
// emac_poll - EMAC NAPI Poll function
// @napi: pointer to the napi_struct containing The DaVinci EMAC network adapter
// @budget: Number of receive packets to process (as told by NAPI layer)
//
// NAPI Poll function implemented to process packets as per budget. We check
// the type of interrupt on the device and accordingly call the TX or RX
// packet processing functions. We follow the budget for RX processing and
// also put a cap on number of TX pkts processed through config param. The
// NAPI schedule function is called if more packets pending.
//
// Returns number of packets received (in most cases; else TX pkts - rarely)
//
#[no_mangle]
unsafe extern "C" fn emac_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int emac_poll(struct napi_struct *napi, int budget)
    {
    unsigned int mask;
    struct emac_priv *priv = container_of(napi, struct emac_priv, napi);
    struct net_device *ndev = priv.ndev;
    struct device *emac_dev = &ndev.dev;
    let mut status: u32 = 0;
    let mut num_rx_pkts: u32 = 0;
// Check interrupt vectors and call packet processing
    status = emac_read(EMAC_MACINVECTOR);
    mask = EMAC_DM644X_MAC_IN_VECTOR_TX_INT_VEC;
    if (priv.version == EMAC_VERSION_2)
    mask = EMAC_DM646X_MAC_IN_VECTOR_TX_INT_VEC;
    if (status & mask) {
    cpdma_chan_process(priv.txchan, EMAC_DEF_TX_MAX_SERVICE);
    } /* TX processing */
    mask = EMAC_DM644X_MAC_IN_VECTOR_RX_INT_VEC;
    if (priv.version == EMAC_VERSION_2)
    mask = EMAC_DM646X_MAC_IN_VECTOR_RX_INT_VEC;
    if (status & mask) {
    num_rx_pkts = cpdma_chan_process(priv.rxchan, budget);
    } /* RX processing */
    mask = EMAC_DM644X_MAC_IN_VECTOR_HOST_INT;
    if (priv.version == EMAC_VERSION_2)
    mask = EMAC_DM646X_MAC_IN_VECTOR_HOST_INT;
    if (unlikely(status & mask)) {
    u32 ch, cause;
    dev_err(emac_dev, "DaVinci EMAC: Fatal Hardware Error\n");
    netif_stop_queue(ndev);
    napi_disable(&priv.napi);
    status = emac_read(EMAC_MACSTATUS);
    cause = ((status & EMAC_MACSTATUS_TXERRCODE_MASK) >>
    EMAC_MACSTATUS_TXERRCODE_SHIFT);
    if (cause) {
    ch = ((status & EMAC_MACSTATUS_TXERRCH_MASK) >>
    EMAC_MACSTATUS_TXERRCH_SHIFT);
    if (net_ratelimit()) {
    dev_err(emac_dev, "TX Host error %s on ch=%d\n",
    &emac_txhost_errcodes[cause][0], ch);
    }
    }
    cause = ((status & EMAC_MACSTATUS_RXERRCODE_MASK) >>
    EMAC_MACSTATUS_RXERRCODE_SHIFT);
    if (cause) {
    ch = ((status & EMAC_MACSTATUS_RXERRCH_MASK) >>
    EMAC_MACSTATUS_RXERRCH_SHIFT);
    if (netif_msg_hw(priv) && net_ratelimit())
    dev_err(emac_dev, "RX Host error %s on ch=%d\n",
    &emac_rxhost_errcodes[cause][0], ch);
    }
    } else if (num_rx_pkts < budget) {
    napi_complete_done(napi, num_rx_pkts);
    emac_int_enable(priv);
    }
    return num_rx_pkts;
    }

//
// emac_poll_controller - EMAC Poll controller function
// @ndev: The DaVinci EMAC network adapter
//
// Polled functionality used by netconsole and others in non interrupt mode
//
#[no_mangle]
unsafe extern "C" fn emac_poll_controller(ndev: *mut net_device) {
    static void emac_poll_controller(struct net_device *ndev)
    {
    struct emac_priv *priv = netdev_priv(ndev);
    emac_int_disable(priv);
    emac_irq(ndev.irq, ndev);
    emac_int_enable(priv);
    }

#[no_mangle]
unsafe extern "C" fn emac_adjust_link(ndev: *mut net_device) {
    static void emac_adjust_link(struct net_device *ndev)
    {
    struct emac_priv *priv = netdev_priv(ndev);
    struct phy_device *phydev = ndev.phydev;
    unsigned long flags;
    let mut new_state: c_int = 0;
    spin_lock_irqsave(&priv.lock, flags);
    if (phydev.link) {
// check the mode of operation - full/half duplex
    if (phydev.duplex != priv.duplex) {
    new_state = 1;
    priv.duplex = phydev.duplex;
    }
    if (phydev.speed != priv.speed) {
    new_state = 1;
    priv.speed = phydev.speed;
    }
    if (!priv.link) {
    new_state = 1;
    priv.link = 1;
    }
    } else if (priv.link) {
    new_state = 1;
    priv.link = 0;
    priv.speed = 0;
    priv.duplex = ~0;
    }
    if (new_state) {
    emac_update_phystatus(priv);
    phy_print_status(ndev.phydev);
    }
    spin_unlock_irqrestore(&priv.lock, flags);
    }
//
// Linux Driver Model
//
// emac_devioctl - EMAC adapter ioctl
// @ndev: The DaVinci EMAC network adapter
// @ifrq: request parameter
// @cmd: command parameter
//
// EMAC driver ioctl function
//
// Returns success(0) or appropriate error code
//
#[no_mangle]
unsafe extern "C" fn emac_devioctl(ndev: *mut net_device, ifrq: *mut ifreq, cmd: c_int) -> c_int {
    static int emac_devioctl(struct net_device *ndev, struct ifreq *ifrq, int cmd)
    {
    if (!(netif_running(ndev)))
    return -EINVAL;
// TODO: Add phy read and write and private statistics get feature
    if (ndev.phydev)
    return phy_mii_ioctl(ndev.phydev, ifrq, cmd);
    else
    return -EOPNOTSUPP;
    }
//
// emac_dev_open - EMAC device open
// @ndev: The DaVinci EMAC network adapter
//
// Called when system wants to start the interface. We init TX/RX channels
// and enable the hardware for packet reception/transmission and start the
// network queue.
//
// Returns 0 for a successful open, or appropriate error code
//
#[no_mangle]
unsafe extern "C" fn emac_dev_open(ndev: *mut net_device) -> c_int {
    static int emac_dev_open(struct net_device *ndev)
    {
    struct device *emac_dev = &ndev.dev;
    struct resource *res;
    int q, m, ret;
    let mut res_num: c_int = 0, irq_num = 0;
    let mut i: c_int = 0;
    struct emac_priv *priv = netdev_priv(ndev);
    struct phy_device *phydev = core::ptr::null_mut();
    ret = pm_runtime_resume_and_get(&priv.pdev.dev);
    if (ret < 0) {
    dev_err(&priv.pdev.dev, "%s: failed to get_sync(%d)\n",
    __func__, ret);
    return ret;
    }
    netif_carrier_off(ndev);
    eth_hw_addr_set(ndev, priv.mac_addr);
// Configuration items
    priv.rx_buf_size = EMAC_DEF_MAX_FRAME_SIZE + NET_IP_ALIGN;
    priv.mac_hash1 = 0;
    priv.mac_hash2 = 0;
    emac_write(EMAC_MACHASH1, 0);
    emac_write(EMAC_MACHASH2, 0);
    for (i = 0; i < EMAC_DEF_RX_NUM_DESC; i++) {
    struct sk_buff *skb = emac_rx_alloc(priv);
    if (!skb)
    break;
    ret = cpdma_chan_idle_submit(priv.rxchan, skb, skb.data,
    skb_tailroom(skb), 0);
    if (WARN_ON(ret < 0))
    break;
    }
// Request IRQ
    if (dev_of_node(&priv.pdev.dev)) {
    while ((ret = platform_get_irq_optional(priv.pdev, res_num)) != -ENXIO) {
    if (ret < 0)
    goto rollback;
    ret = request_irq(ret, emac_irq, 0, ndev.name, ndev);
    if (ret) {
    dev_err(emac_dev, "DaVinci EMAC: request_irq() failed\n");
    goto rollback;
    }
    res_num++;
    }
    } else {
    while ((res = platform_get_resource(priv.pdev, IORESOURCE_IRQ, res_num))) {
    for (irq_num = res.start; irq_num <= res.end; irq_num++) {
    ret = request_irq(irq_num, emac_irq, 0, ndev.name, ndev);
    if (ret) {
    dev_err(emac_dev, "DaVinci EMAC: request_irq() failed\n");
    goto rollback;
    }
    }
    res_num++;
    }
// prepare counters for rollback in case of an error
    res_num--;
    irq_num--;
    }
// Start/Enable EMAC hardware
    emac_hw_enable(priv);
// Enable Interrupt pacing if configured
    if (priv.coal_intvl != 0) {
    struct ethtool_coalesce coal;
    coal.rx_coalesce_usecs = (priv.coal_intvl << 4);
    emac_set_coalesce(ndev, &coal, core::ptr::null_mut(), core::ptr::null_mut());
    }
    cpdma_ctlr_start(priv.dma);
    if (priv.phy_node) {
    phydev = of_phy_connect(ndev, priv.phy_node,
    &emac_adjust_link, 0, 0);
    if (!phydev) {
    dev_err(emac_dev, "could not connect to phy %pOF\n",
    priv.phy_node);
    ret = -ENODEV;
    goto err;
    }
    }
// if no phy-handle and no fixed link, use the first phy on the bus
    if (!phydev && !priv.phy_id) {
    struct device_node *np;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ti,davinci_mdio");
    if (np) {
    struct mii_bus *bus = of_mdio_find_bus(np);
    if (bus) {
    struct phy_device *phy = phy_find_first(bus);
    if (phy)
    priv.phy_id = phydev_name(phy);
    put_device(&bus.dev); /* of_mdio_find_bus */
    }
    of_node_put(np); /* of_find_compatible_node */
    }
    }
    if (!phydev && priv.phy_id && *priv.phy_id) {
    phydev = phy_connect(ndev, priv.phy_id,
    &emac_adjust_link,
    PHY_INTERFACE_MODE_MII);
    if (IS_ERR(phydev)) {
    dev_err(emac_dev, "could not connect to phy %s\n",
    priv.phy_id);
    ret = PTR_ERR(phydev);
    goto err;
    }
    priv.link = 0;
    priv.speed = 0;
    priv.duplex = ~0;
    phy_attached_info(phydev);
    }
    if (!phydev) {
// No PHY , fix the link, speed and duplex settings
    dev_notice(emac_dev, "no phy, defaulting to 100/full\n");
    priv.link = 1;
    priv.speed = SPEED_100;
    priv.duplex = DUPLEX_FULL;
    emac_update_phystatus(priv);
    }
    if (netif_msg_drv(priv))
    dev_notice(emac_dev, "DaVinci EMAC: Opened %s\n", ndev.name);
    if (phydev)
    phy_start(phydev);
    return 0;
    err:
    emac_int_disable(priv);
    napi_disable(&priv.napi);
    rollback:
    if (dev_of_node(&priv.pdev.dev)) {
    for (q = res_num - 1; q >= 0; q--) {
    irq_num = platform_get_irq(priv.pdev, q);
    if (irq_num > 0)
    free_irq(irq_num, ndev);
    }
    } else {
    for (q = res_num; q >= 0; q--) {
    res = platform_get_resource(priv.pdev, IORESOURCE_IRQ, q);
// at the first iteration, irq_num is already set to the
// right value
//
    if (q != res_num)
    irq_num = res.end;
    for (m = irq_num; m >= res.start; m--)
    free_irq(m, ndev);
    }
    }
    cpdma_ctlr_stop(priv.dma);
    pm_runtime_put(&priv.pdev.dev);
    return ret;
    }
//
// emac_dev_stop - EMAC device stop
// @ndev: The DaVinci EMAC network adapter
//
// Called when system wants to stop or down the interface. We stop the network
// queue, disable interrupts and cleanup TX/RX channels.
//
// We return the statistics in net_device_stats structure pulled from emac
//
#[no_mangle]
unsafe extern "C" fn emac_dev_stop(ndev: *mut net_device) -> c_int {
    static int emac_dev_stop(struct net_device *ndev)
    {
    struct resource *res;
    let mut i: c_int = 0;
    int irq_num;
    struct emac_priv *priv = netdev_priv(ndev);
    struct device *emac_dev = &ndev.dev;
    let mut ret: c_int = 0;
// inform the upper layers.
    netif_stop_queue(ndev);
    napi_disable(&priv.napi);
    netif_carrier_off(ndev);
    emac_int_disable(priv);
    cpdma_ctlr_stop(priv.dma);
    emac_write(EMAC_SOFTRESET, 1);
    if (ndev.phydev)
    phy_disconnect(ndev.phydev);
// Free IRQ
    if (dev_of_node(&priv.pdev.dev)) {
    do {
    ret = platform_get_irq_optional(priv.pdev, i);
    if (ret < 0 && ret != -ENXIO)
    break;
    if (ret > 0) {
    free_irq(ret, priv.ndev);
    } else {
    ret = 0;
    break;
    }
    } while (++i);
    } else {
    while ((res = platform_get_resource(priv.pdev, IORESOURCE_IRQ, i))) {
    for (irq_num = res.start; irq_num <= res.end; irq_num++)
    free_irq(irq_num, priv.ndev);
    i++;
    }
    }
    if (netif_msg_drv(priv))
    dev_notice(emac_dev, "DaVinci EMAC: %s stopped\n", ndev.name);
    pm_runtime_put(&priv.pdev.dev);
    return ret;
    }
//
// emac_dev_getnetstats - EMAC get statistics function
// @ndev: The DaVinci EMAC network adapter
//
// Called when system wants to get statistics from the device.
//
// We return the statistics in net_device_stats structure pulled from emac
//
    static struct net_device_stats *emac_dev_getnetstats(struct net_device *ndev)
    {
    struct emac_priv *priv = netdev_priv(ndev);
    u32 mac_control;
    u32 stats_clear_mask;
    int err;
    err = pm_runtime_resume_and_get(&priv.pdev.dev);
    if (err < 0) {
    dev_err(&priv.pdev.dev, "%s: failed to get_sync(%d)\n",
    __func__, err);
    return &ndev.stats;
    }
// update emac hardware stats and reset the registers
    mac_control = emac_read(EMAC_MACCONTROL);
    if (mac_control & EMAC_MACCONTROL_GMIIEN)
    stats_clear_mask = EMAC_STATS_CLR_MASK;
    else
    stats_clear_mask = 0;
    ndev.stats.multicast += emac_read(EMAC_RXMCASTFRAMES);
    emac_write(EMAC_RXMCASTFRAMES, stats_clear_mask);
    ndev.stats.collisions += (emac_read(EMAC_TXCOLLISION) +
    emac_read(EMAC_TXSINGLECOLL) +
    emac_read(EMAC_TXMULTICOLL));
    emac_write(EMAC_TXCOLLISION, stats_clear_mask);
    emac_write(EMAC_TXSINGLECOLL, stats_clear_mask);
    emac_write(EMAC_TXMULTICOLL, stats_clear_mask);
    ndev.stats.rx_length_errors += (emac_read(EMAC_RXOVERSIZED) +
    emac_read(EMAC_RXJABBER) +
    emac_read(EMAC_RXUNDERSIZED));
    emac_write(EMAC_RXOVERSIZED, stats_clear_mask);
    emac_write(EMAC_RXJABBER, stats_clear_mask);
    emac_write(EMAC_RXUNDERSIZED, stats_clear_mask);
    ndev.stats.rx_over_errors += (emac_read(EMAC_RXSOFOVERRUNS) +
    emac_read(EMAC_RXMOFOVERRUNS));
    emac_write(EMAC_RXSOFOVERRUNS, stats_clear_mask);
    emac_write(EMAC_RXMOFOVERRUNS, stats_clear_mask);
    ndev.stats.rx_fifo_errors += emac_read(EMAC_RXDMAOVERRUNS);
    emac_write(EMAC_RXDMAOVERRUNS, stats_clear_mask);
    ndev.stats.tx_carrier_errors +=
    emac_read(EMAC_TXCARRIERSENSE);
    emac_write(EMAC_TXCARRIERSENSE, stats_clear_mask);
    ndev.stats.tx_fifo_errors += emac_read(EMAC_TXUNDERRUN);
    emac_write(EMAC_TXUNDERRUN, stats_clear_mask);
    pm_runtime_put(&priv.pdev.dev);
    return &ndev.stats;
    }
    static const struct net_device_ops emac_netdev_ops = {
    .ndo_open		= emac_dev_open,
    .ndo_stop		= emac_dev_stop,
    .ndo_start_xmit		= emac_dev_xmit,
    .ndo_set_rx_mode	= emac_dev_mcast_set,
    .ndo_set_mac_address	= emac_dev_setmac_addr,
    .ndo_eth_ioctl		= emac_devioctl,
    .ndo_tx_timeout		= emac_dev_tx_timeout,
    .ndo_get_stats		= emac_dev_getnetstats,

    .ndo_poll_controller	= emac_poll_controller,

    };
    static struct emac_platform_data *
    davinci_emac_of_get_pdata(struct platform_device *pdev, struct emac_priv *priv)
    {
    struct device_node *np;
    const struct emac_platform_data *auxdata;
    struct emac_platform_data *pdata = core::ptr::null_mut();
    if (!IS_ENABLED(CONFIG_OF) || !pdev.dev.of_node)
    return dev_get_platdata(&pdev.dev);
    pdata = devm_kzalloc(&pdev.dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return core::ptr::null_mut();
    np = pdev.dev.of_node;
    pdata.version = EMAC_VERSION_2;
    if (!is_valid_ether_addr(pdata.mac_addr))
    of_get_mac_address(np, pdata.mac_addr);
    of_property_read_u32(np, "ti,davinci-ctrl-reg-offset",
    &pdata.ctrl_reg_offset);
    of_property_read_u32(np, "ti,davinci-ctrl-mod-reg-offset",
    &pdata.ctrl_mod_reg_offset);
    of_property_read_u32(np, "ti,davinci-ctrl-ram-offset",
    &pdata.ctrl_ram_offset);
    of_property_read_u32(np, "ti,davinci-ctrl-ram-size",
    &pdata.ctrl_ram_size);
    of_property_read_u8(np, "ti,davinci-rmii-en", &pdata.rmii_en);
    pdata.no_bd_ram = of_property_read_bool(np, "ti,davinci-no-bd-ram");
    priv.phy_node = of_parse_phandle(np, "phy-handle", 0);
    if (!priv.phy_node) {
    if (!of_phy_is_fixed_link(np))
    pdata.phy_id = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn if(0: of_phy_register_fixed_link(np) >=) -> else {
    else if (of_phy_register_fixed_link(np) >= 0)
    priv.phy_node = of_node_get(np);
    }
    auxdata = pdev.dev.platform_data;
    if (auxdata) {
    pdata.interrupt_enable = auxdata.interrupt_enable;
    pdata.interrupt_disable = auxdata.interrupt_disable;
    }
    auxdata = device_get_match_data(&pdev.dev);
    if (auxdata) {
    pdata.version = auxdata.version;
    pdata.hw_ram_addr = auxdata.hw_ram_addr;
    }
    return  pdata;
    }
    static int davinci_emac_try_get_mac(struct platform_device *pdev,
    int instance, u8 *mac_addr)
    {
    if (!pdev.dev.of_node)
    return -EINVAL;
    return ti_cm_get_macid(&pdev.dev, instance, mac_addr);
    }
//
// davinci_emac_probe - EMAC device probe
// @pdev: The DaVinci EMAC device that we are removing
//
// Called when probing for emac devicesr. We get details of instances and
// resource information from platform init and register a network device
// and allocate resources necessary for driver to perform
//
#[no_mangle]
unsafe extern "C" fn davinci_emac_probe(pdev: *mut platform_device) -> c_int {
    static int davinci_emac_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    let mut rc: c_int = 0;
    struct resource *res, *res_ctrl;
    struct net_device *ndev;
    struct emac_priv *priv;
    unsigned long hw_ram_addr;
    struct emac_platform_data *pdata;
    struct cpdma_params dma_params;
    struct clk *emac_clk;
    unsigned long emac_bus_frequency;
// obtain emac clock from kernel
    emac_clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(emac_clk)) {
    dev_err(&pdev.dev, "failed to get EMAC clock\n");
    return -EBUSY;
    }
    emac_bus_frequency = clk_get_rate(emac_clk);
    devm_clk_put(&pdev.dev, emac_clk);
// TODO: Probe PHY here if possible
    ndev = alloc_etherdev(sizeof(struct emac_priv));
    if (!ndev)
    return -ENOMEM;
    platform_set_drvdata(pdev, ndev);
    priv = netdev_priv(ndev);
    priv.pdev = pdev;
    priv.ndev = ndev;
    priv.msg_enable = netif_msg_init(debug_level, DAVINCI_EMAC_DEBUG);
    spin_lock_init(&priv.lock);
    pdata = davinci_emac_of_get_pdata(pdev, priv);
    if (!pdata) {
    dev_err(&pdev.dev, "no platform data\n");
    rc = -ENODEV;
    goto err_free_netdev;
    }
// MAC addr and PHY mask , RMII enable info from platform_data
    memcpy(priv.mac_addr, pdata.mac_addr, ETH_ALEN);
    priv.phy_id = pdata.phy_id;
    priv.rmii_en = pdata.rmii_en;
    priv.version = pdata.version;
    priv.int_enable = pdata.interrupt_enable;
    priv.int_disable = pdata.interrupt_disable;
    priv.coal_intvl = 0;
    priv.bus_freq_mhz = (u32)(emac_bus_frequency / 1000000);
// Get EMAC platform data
    priv.remap_addr = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(priv.remap_addr)) {
    rc = PTR_ERR(priv.remap_addr);
    goto no_pdata;
    }
    priv.emac_base_phys = res.start + pdata.ctrl_reg_offset;
    res_ctrl = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    if (res_ctrl) {
    priv.ctrl_base =
    devm_ioremap_resource(&pdev.dev, res_ctrl);
    if (IS_ERR(priv.ctrl_base)) {
    rc = PTR_ERR(priv.ctrl_base);
    goto no_pdata;
    }
    } else {
    priv.ctrl_base = priv.remap_addr + pdata.ctrl_mod_reg_offset;
    }
    priv.emac_base = priv.remap_addr + pdata.ctrl_reg_offset;
    ndev.base_addr = (unsigned long)priv.remap_addr;
    hw_ram_addr = pdata.hw_ram_addr;
    if (!hw_ram_addr)
    hw_ram_addr = (u32 )res.start + pdata.ctrl_ram_offset;
    memset(&dma_params, 0, sizeof(dma_params));
    dma_params.dev			= &pdev.dev;
    dma_params.dmaregs		= priv.emac_base;
    dma_params.rxthresh		= priv.emac_base + 0x120;
    dma_params.rxfree		= priv.emac_base + 0x140;
    dma_params.txhdp		= priv.emac_base + 0x600;
    dma_params.rxhdp		= priv.emac_base + 0x620;
    dma_params.txcp			= priv.emac_base + 0x640;
    dma_params.rxcp			= priv.emac_base + 0x660;
    dma_params.num_chan		= EMAC_MAX_TXRX_CHANNELS;
    dma_params.min_packet_size	= EMAC_DEF_MIN_ETHPKTSIZE;
    dma_params.desc_hw_addr		= hw_ram_addr;
    dma_params.desc_mem_size	= pdata.ctrl_ram_size;
    dma_params.desc_align		= 16;
    dma_params.desc_mem_phys = pdata.no_bd_ram ? 0 :
    (u32 )res.start + pdata.ctrl_ram_offset;
    priv.dma = cpdma_ctlr_create(&dma_params);
    if (!priv.dma) {
    dev_err(&pdev.dev, "error initializing DMA\n");
    rc = -ENOMEM;
    goto no_pdata;
    }
    priv.txchan = cpdma_chan_create(priv.dma, EMAC_DEF_TX_CH,
    emac_tx_handler, 0);
    if (IS_ERR(priv.txchan)) {
    dev_err(&pdev.dev, "error initializing tx dma channel\n");
    rc = PTR_ERR(priv.txchan);
    goto err_free_dma;
    }
    priv.rxchan = cpdma_chan_create(priv.dma, EMAC_DEF_RX_CH,
    emac_rx_handler, 1);
    if (IS_ERR(priv.rxchan)) {
    dev_err(&pdev.dev, "error initializing rx dma channel\n");
    rc = PTR_ERR(priv.rxchan);
    goto err_free_txchan;
    }
    rc = platform_get_irq(pdev, 0);
    if (rc < 0)
    goto err_free_rxchan;
    ndev.irq = rc;
// If the MAC address is not present, read the registers from the SoC
    if (!is_valid_ether_addr(priv.mac_addr)) {
    rc = davinci_emac_try_get_mac(pdev, res_ctrl ? 0 : 1, priv.mac_addr);
    if (!rc)
    eth_hw_addr_set(ndev, priv.mac_addr);
    if (!is_valid_ether_addr(priv.mac_addr)) {
// Use random MAC if still none obtained.
    eth_hw_addr_random(ndev);
    memcpy(priv.mac_addr, ndev.dev_addr, ndev.addr_len);
    dev_warn(&pdev.dev, "using random MAC addr: %pM\n",
    priv.mac_addr);
    }
    }
    ndev.netdev_ops = &emac_netdev_ops;
    ndev.ethtool_ops = &ethtool_ops;
    netif_napi_add(ndev, &priv.napi, emac_poll);
    pm_runtime_enable(&pdev.dev);
    rc = pm_runtime_resume_and_get(&pdev.dev);
    if (rc < 0) {
    dev_err(&pdev.dev, "%s: failed to get_sync(%d)\n",
    __func__, rc);
    goto err_napi_del;
    }
// register the network device
    SET_NETDEV_DEV(ndev, &pdev.dev);
    rc = register_netdev(ndev);
    if (rc) {
    dev_err(&pdev.dev, "error in register_netdev\n");
    rc = -ENODEV;
    pm_runtime_put(&pdev.dev);
    goto err_napi_del;
    }
    if (netif_msg_probe(priv)) {
    dev_notice(&pdev.dev, "DaVinci EMAC Probe found device "
    "(regs: %pa, irq: %d)\n",
    &priv.emac_base_phys, ndev.irq);
    }
    pm_runtime_put(&pdev.dev);
    return 0;
    err_napi_del:
    netif_napi_del(&priv.napi);
    err_free_rxchan:
    cpdma_chan_destroy(priv.rxchan);
    err_free_txchan:
    cpdma_chan_destroy(priv.txchan);
    err_free_dma:
    cpdma_ctlr_destroy(priv.dma);
    no_pdata:
    if (of_phy_is_fixed_link(np))
    of_phy_deregister_fixed_link(np);
    of_node_put(priv.phy_node);
    err_free_netdev:
    free_netdev(ndev);
    return rc;
    }
//
// davinci_emac_remove - EMAC device remove
// @pdev: The DaVinci EMAC device that we are removing
//
// Called when removing the device driver. We disable clock usage and release
// the resources taken up by the driver and unregister network device
//
#[no_mangle]
unsafe extern "C" fn davinci_emac_remove(pdev: *mut platform_device) {
    static void davinci_emac_remove(struct platform_device *pdev)
    {
    struct net_device *ndev = platform_get_drvdata(pdev);
    struct emac_priv *priv = netdev_priv(ndev);
    struct device_node *np = pdev.dev.of_node;
    dev_notice(&ndev.dev, "DaVinci EMAC: davinci_emac_remove()\n");
    if (priv.txchan)
    cpdma_chan_destroy(priv.txchan);
    if (priv.rxchan)
    cpdma_chan_destroy(priv.rxchan);
    cpdma_ctlr_destroy(priv.dma);
    unregister_netdev(ndev);
    of_node_put(priv.phy_node);
    pm_runtime_disable(&pdev.dev);
    if (of_phy_is_fixed_link(np))
    of_phy_deregister_fixed_link(np);
    free_netdev(ndev);
    }
#[no_mangle]
unsafe extern "C" fn davinci_emac_suspend(dev: *mut device) -> c_int {
    static int davinci_emac_suspend(struct device *dev)
    {
    struct net_device *ndev = dev_get_drvdata(dev);
    if (netif_running(ndev))
    emac_dev_stop(ndev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn davinci_emac_resume(dev: *mut device) -> c_int {
    static int davinci_emac_resume(struct device *dev)
    {
    struct net_device *ndev = dev_get_drvdata(dev);
    if (netif_running(ndev))
    emac_dev_open(ndev);
    return 0;
    }
    static const struct dev_pm_ops davinci_emac_pm_ops = {
    .suspend	= davinci_emac_suspend,
    .resume		= davinci_emac_resume,
    };
    static const struct emac_platform_data am3517_emac_data = {
    .version		= EMAC_VERSION_2,
    .hw_ram_addr		= 0x01e20000,
    };
    static const struct emac_platform_data dm816_emac_data = {
    .version		= EMAC_VERSION_2,
    };
    static const struct of_device_id davinci_emac_of_match[] = {
    {.compatible = "ti,davinci-dm6467-emac", },
    {.compatible = "ti,am3517-emac", .data = &am3517_emac_data, },
    {.compatible = "ti,dm816-emac", .data = &dm816_emac_data, },
    {},
    };
    MODULE_DEVICE_TABLE(of, davinci_emac_of_match);
// davinci_emac_driver: EMAC platform driver structure
    static struct platform_driver davinci_emac_driver = {
    .driver = {
    .name	 = "davinci_emac",
    .pm	 = &davinci_emac_pm_ops,
    .of_match_table = davinci_emac_of_match,
    },
    .probe = davinci_emac_probe,
    .remove = davinci_emac_remove,
    };
//
// davinci_emac_init - EMAC driver module init
//
// Called when initializing the driver. We register the driver with
// the platform.
//
#[no_mangle]
unsafe extern "C" fn davinci_emac_init() -> int __init {
    static int __init davinci_emac_init(void)
    {
    return platform_driver_register(&davinci_emac_driver);
    }
    late_initcall(davinci_emac_init);
//
// davinci_emac_exit - EMAC driver module exit
//
// Called when exiting the driver completely. We unregister the driver with
// the platform and exit
//
#[no_mangle]
unsafe extern "C" fn davinci_emac_exit() -> void __exit {
    static void __exit davinci_emac_exit(void)
    {
    platform_driver_unregister(&davinci_emac_driver);
    }
    module_exit(davinci_emac_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("DaVinci EMAC Maintainer: Anant Gole <anantgole@ti.com>");
    MODULE_AUTHOR("DaVinci EMAC Maintainer: Chaithrika U S <chaithrika@ti.com>");
    MODULE_DESCRIPTION("DaVinci EMAC Ethernet driver");

//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/socionext/netsec.c
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

pub const NETSEC_REG_SOFT_RST: c_uint = 0x104;
pub const NETSEC_REG_COM_INIT: c_uint = 0x120;
pub const NETSEC_REG_TOP_STATUS: c_uint = 0x200;

pub const NETSEC_REG_TOP_INTEN: c_uint = 0x204;
pub const NETSEC_REG_INTEN_SET: c_uint = 0x234;
pub const NETSEC_REG_INTEN_CLR: c_uint = 0x238;
pub const NETSEC_REG_NRM_TX_STATUS: c_uint = 0x400;
pub const NETSEC_REG_NRM_TX_INTEN: c_uint = 0x404;
pub const NETSEC_REG_NRM_TX_INTEN_SET: c_uint = 0x428;
pub const NETSEC_REG_NRM_TX_INTEN_CLR: c_uint = 0x42c;

pub const NETSEC_REG_NRM_RX_STATUS: c_uint = 0x440;
pub const NETSEC_REG_NRM_RX_INTEN: c_uint = 0x444;
pub const NETSEC_REG_NRM_RX_INTEN_SET: c_uint = 0x468;
pub const NETSEC_REG_NRM_RX_INTEN_CLR: c_uint = 0x46c;

pub const NETSEC_REG_PKT_CMD_BUF: c_uint = 0xd0;
pub const NETSEC_REG_CLK_EN: c_uint = 0x100;
pub const NETSEC_REG_PKT_CTRL: c_uint = 0x140;
pub const NETSEC_REG_DMA_TMR_CTRL: c_uint = 0x20c;
pub const NETSEC_REG_F_TAIKI_MC_VER: c_uint = 0x22c;
pub const NETSEC_REG_F_TAIKI_VER: c_uint = 0x230;
pub const NETSEC_REG_DMA_HM_CTRL: c_uint = 0x214;
pub const NETSEC_REG_DMA_MH_CTRL: c_uint = 0x220;
pub const NETSEC_REG_ADDR_DIS_CORE: c_uint = 0x218;
pub const NETSEC_REG_DMAC_HM_CMD_BUF: c_uint = 0x210;
pub const NETSEC_REG_DMAC_MH_CMD_BUF: c_uint = 0x21c;
pub const NETSEC_REG_NRM_TX_PKTCNT: c_uint = 0x410;
pub const NETSEC_REG_NRM_TX_DONE_PKTCNT: c_uint = 0x414;
pub const NETSEC_REG_NRM_TX_DONE_TXINT_PKTCNT: c_uint = 0x418;
pub const NETSEC_REG_NRM_TX_TMR: c_uint = 0x41c;
pub const NETSEC_REG_NRM_RX_PKTCNT: c_uint = 0x454;
pub const NETSEC_REG_NRM_RX_RXINT_PKTCNT: c_uint = 0x458;
pub const NETSEC_REG_NRM_TX_TXINT_TMR: c_uint = 0x420;
pub const NETSEC_REG_NRM_RX_RXINT_TMR: c_uint = 0x460;
pub const NETSEC_REG_NRM_RX_TMR: c_uint = 0x45c;
pub const NETSEC_REG_NRM_TX_DESC_START_UP: c_uint = 0x434;
pub const NETSEC_REG_NRM_TX_DESC_START_LW: c_uint = 0x408;
pub const NETSEC_REG_NRM_RX_DESC_START_UP: c_uint = 0x474;
pub const NETSEC_REG_NRM_RX_DESC_START_LW: c_uint = 0x448;
pub const NETSEC_REG_NRM_TX_CONFIG: c_uint = 0x430;
pub const NETSEC_REG_NRM_RX_CONFIG: c_uint = 0x470;
pub const MAC_REG_STATUS: c_uint = 0x1024;
pub const MAC_REG_DATA: c_uint = 0x11c0;
pub const MAC_REG_CMD: c_uint = 0x11c4;
pub const MAC_REG_FLOW_TH: c_uint = 0x11cc;
pub const MAC_REG_INTF_SEL: c_uint = 0x11d4;
pub const MAC_REG_DESC_INIT: c_uint = 0x11fc;
pub const MAC_REG_DESC_SOFT_RST: c_uint = 0x1204;
pub const NETSEC_REG_MODE_TRANS_COMP_STATUS: c_uint = 0x500;
pub const GMAC_REG_MCR: c_uint = 0x0000;
pub const GMAC_REG_MFFR: c_uint = 0x0004;
pub const GMAC_REG_GAR: c_uint = 0x0010;
pub const GMAC_REG_GDR: c_uint = 0x0014;
pub const GMAC_REG_FCR: c_uint = 0x0018;
pub const GMAC_REG_BMR: c_uint = 0x1000;
pub const GMAC_REG_RDLAR: c_uint = 0x100c;
pub const GMAC_REG_TDLAR: c_uint = 0x1010;
pub const GMAC_REG_OMR: c_uint = 0x1018;

pub const NETSEC_TX_SHIFT_OWN_FIELD: c_int = 31;
pub const NETSEC_TX_SHIFT_LD_FIELD: c_int = 30;
pub const NETSEC_TX_SHIFT_DRID_FIELD: c_int = 24;
pub const NETSEC_TX_SHIFT_PT_FIELD: c_int = 21;
pub const NETSEC_TX_SHIFT_TDRID_FIELD: c_int = 16;
pub const NETSEC_TX_SHIFT_CC_FIELD: c_int = 15;
pub const NETSEC_TX_SHIFT_FS_FIELD: c_int = 9;
pub const NETSEC_TX_LAST: c_int = 8;
pub const NETSEC_TX_SHIFT_CO: c_int = 7;
pub const NETSEC_TX_SHIFT_SO: c_int = 6;
pub const NETSEC_TX_SHIFT_TRS_FIELD: c_int = 4;
pub const NETSEC_RX_PKT_OWN_FIELD: c_int = 31;
pub const NETSEC_RX_PKT_LD_FIELD: c_int = 30;
pub const NETSEC_RX_PKT_SDRID_FIELD: c_int = 24;
pub const NETSEC_RX_PKT_FR_FIELD: c_int = 23;
pub const NETSEC_RX_PKT_ER_FIELD: c_int = 21;
pub const NETSEC_RX_PKT_ERR_FIELD: c_int = 16;
pub const NETSEC_RX_PKT_TDRID_FIELD: c_int = 12;
pub const NETSEC_RX_PKT_FS_FIELD: c_int = 9;
pub const NETSEC_RX_PKT_LS_FIELD: c_int = 8;
pub const NETSEC_RX_PKT_CO_FIELD: c_int = 6;
pub const NETSEC_RX_PKT_ERR_MASK: c_int = 3;
pub const NETSEC_MAX_TX_PKT_LEN: c_int = 1518;
pub const NETSEC_MAX_TX_JUMBO_PKT_LEN: c_int = 9018;
pub const NETSEC_RING_GMAC: c_int = 15;
pub const NETSEC_RING_MAX: c_int = 2;
pub const NETSEC_TCP_SEG_LEN_MAX: c_int = 1460;
pub const NETSEC_TCP_JUMBO_SEG_LEN_MAX: c_int = 8960;
pub const NETSEC_RX_CKSUM_NOTAVAIL: c_int = 0;
pub const NETSEC_RX_CKSUM_OK: c_int = 1;
pub const NETSEC_RX_CKSUM_NG: c_int = 2;

pub const NETSEC_INT_PKTCNT_MAX: c_int = 2047;
pub const NETSEC_FLOW_START_TH_MAX: c_int = 95;
pub const NETSEC_FLOW_STOP_TH_MAX: c_int = 95;
pub const NETSEC_FLOW_PAUSE_TIME_MIN: c_int = 5;
pub const NETSEC_CLK_EN_REG_DOM_ALL: c_uint = 0x3f;

    NETSEC_COM_INIT_REG_DB)
pub const NETSEC_SOFT_RST_REG_RESET: c_int = 0;

pub const NETSEC_DMA_CTRL_REG_STOP: c_int = 1;

pub const NETSEC_GMAC_CMD_ST_READ: c_int = 0;

pub const NETSEC_GMAC_BMR_REG_COMMON: c_uint = 0x00412080;
pub const NETSEC_GMAC_BMR_REG_RESET: c_uint = 0x00020181;
pub const NETSEC_GMAC_BMR_REG_SWR: c_uint = 0x00000001;

pub const NETSEC_GMAC_MCR_REG_FULL_DUPLEX_COMMON: c_uint = 0x0000280c;
pub const NETSEC_GMAC_MCR_REG_HALF_DUPLEX_COMMON: c_uint = 0x0001a00c;

pub const NETSEC_GMAC_GAR_REG_SHIFT_PA: c_int = 11;
pub const NETSEC_GMAC_GAR_REG_SHIFT_GR: c_int = 6;
pub const GMAC_REG_SHIFT_CR_GAR: c_int = 2;
pub const NETSEC_GMAC_GAR_REG_CR_25_35_MHZ: c_int = 2;
pub const NETSEC_GMAC_GAR_REG_CR_35_60_MHZ: c_int = 3;
pub const NETSEC_GMAC_GAR_REG_CR_60_100_MHZ: c_int = 0;
pub const NETSEC_GMAC_GAR_REG_CR_100_150_MHZ: c_int = 1;
pub const NETSEC_GMAC_GAR_REG_CR_150_250_MHZ: c_int = 4;
pub const NETSEC_GMAC_GAR_REG_CR_250_300_MHZ: c_int = 5;
pub const NETSEC_GMAC_RDLAR_REG_COMMON: c_uint = 0x18000;
pub const NETSEC_GMAC_TDLAR_REG_COMMON: c_uint = 0x1c000;
pub const NETSEC_REG_NETSEC_VER_F_TAIKI: c_uint = 0x50000;

pub const NETSEC_REG_DESC_TMR_MODE: c_int = 4;
pub const NETSEC_REG_DESC_ENDIAN: c_int = 0;
pub const NETSEC_MAC_DESC_SOFT_RST_SOFT_RST: c_int = 1;
pub const NETSEC_MAC_DESC_INIT_REG_INIT: c_int = 1;
pub const NETSEC_EEPROM_MAC_ADDRESS: c_uint = 0x00;
pub const NETSEC_EEPROM_HM_ME_ADDRESS_H: c_uint = 0x08;
pub const NETSEC_EEPROM_HM_ME_ADDRESS_L: c_uint = 0x0C;
pub const NETSEC_EEPROM_HM_ME_SIZE: c_uint = 0x10;
pub const NETSEC_EEPROM_MH_ME_ADDRESS_H: c_uint = 0x14;
pub const NETSEC_EEPROM_MH_ME_ADDRESS_L: c_uint = 0x18;
pub const NETSEC_EEPROM_MH_ME_SIZE: c_uint = 0x1C;
pub const NETSEC_EEPROM_PKT_ME_ADDRESS: c_uint = 0x20;
pub const NETSEC_EEPROM_PKT_ME_SIZE: c_uint = 0x24;
pub const DESC_NUM: c_int = 256;

    NET_IP_ALIGN)

    SKB_DATA_ALIGN(sizeof(struct skb_shared_info)))

pub const NETSEC_XDP_PASS: c_int = 0;

    enum ring_id {
    NETSEC_RING_TX = 0,
    NETSEC_RING_RX
    };
    enum buf_type {
    TYPE_NETSEC_SKB = 0,
    TYPE_NETSEC_XDP_TX,
    TYPE_NETSEC_XDP_NDO,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netsec_desc {
    union {
    pub skb: *mut sk_buff,
    pub xdpf: *mut xdp_frame,
}

    dma_addr_t dma_addr;
    void *addr;
    u16 len;
    u8 buf_type;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netsec_desc_ring {
    pub desc_dma: dma_addr_t,
    pub desc: *mut netsec_desc,
    pub vaddr: *mut c_void,
    pub tail: u16 head,,
    pub /: *mut *mut u16 xdp_xmit; / netsec_xdp_xmit packets,
    pub page_pool: *mut page_pool,
    pub xdp_rxq: xdp_rxq_info,
    pub /: *mut *mut spinlock_t lock; / XDP tx queue locking,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netsec_priv {
    pub desc_ring: [netsec_desc_ring; NETSEC_RING_MAX],
    pub et_coalesce: ethtool_coalesce,
    pub xdp_prog: *mut bpf_prog,
    pub /: *mut *mut spinlock_t reglock; / protect reg access,
    pub napi: napi_struct,
    pub phy_interface: phy_interface_t,
    pub ndev: *mut net_device,
    pub phy_np: *mut device_node,
    pub phydev: *mut phy_device,
    pub mii_bus: *mut mii_bus,
    pub ioaddr: *mut void __iomem,
    pub eeprom_base: *mut void __iomem,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub msg_enable: u32,
    pub freq: u32,
    pub phy_addr: u32,
    pub rx_cksum_offload_flag: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netsec_de {
    pub attr: u32,
    pub data_buf_addr_up: u32,
    pub data_buf_addr_lw: u32,
    pub buf_len_info: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netsec_tx_pkt_ctrl {
    pub tcp_seg_len: u16,
    pub tcp_seg_offload_flag: bool,
    pub cksum_offload_flag: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netsec_rx_pkt_info {
    pub rx_cksum_result: c_int,
    pub err_code: c_int,
    pub err_flag: bool,
}

#[no_mangle]
unsafe extern "C" fn netsec_write(priv: *mut netsec_priv, reg_addr: u32, val: u32) {
    static void netsec_write(struct netsec_priv *priv, u32 reg_addr, u32 val)
    {
    writel(val, priv.ioaddr + reg_addr);
    }
#[no_mangle]
unsafe extern "C" fn netsec_read(priv: *mut netsec_priv, reg_addr: u32) -> u32 {
    static u32 netsec_read(struct netsec_priv *priv, u32 reg_addr)
    {
    return readl(priv.ioaddr + reg_addr);
    }
// MDIO BUS OPS FOLLOW
pub const TIMEOUT_SPINS_MAC: c_int = 1000;
pub const TIMEOUT_SECONDARY_MS_MAC: c_int = 100;
#[no_mangle]
unsafe extern "C" fn netsec_clk_type(freq: u32) -> u32 {
    static u32 netsec_clk_type(u32 freq)
    {
    if (freq < MHZ(35))
    return NETSEC_GMAC_GAR_REG_CR_25_35_MHZ;
    if (freq < MHZ(60))
    return NETSEC_GMAC_GAR_REG_CR_35_60_MHZ;
    if (freq < MHZ(100))
    return NETSEC_GMAC_GAR_REG_CR_60_100_MHZ;
    if (freq < MHZ(150))
    return NETSEC_GMAC_GAR_REG_CR_100_150_MHZ;
    if (freq < MHZ(250))
    return NETSEC_GMAC_GAR_REG_CR_150_250_MHZ;
    return NETSEC_GMAC_GAR_REG_CR_250_300_MHZ;
    }
#[no_mangle]
unsafe extern "C" fn netsec_wait_while_busy(priv: *mut netsec_priv, addr: u32, mask: u32) -> c_int {
    static int netsec_wait_while_busy(struct netsec_priv *priv, u32 addr, u32 mask)
    {
    let mut timeout: u32 = TIMEOUT_SPINS_MAC;
    while (--timeout && netsec_read(priv, addr) & mask)
    cpu_relax();
    if (timeout)
    return 0;
    timeout = TIMEOUT_SECONDARY_MS_MAC;
    while (--timeout && netsec_read(priv, addr) & mask)
    usleep_range(1000, 2000);
    if (timeout)
    return 0;
    netdev_WARN(priv.ndev, "%s: timeout\n", __func__);
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn netsec_mac_write(priv: *mut netsec_priv, addr: u32, value: u32) -> c_int {
    static int netsec_mac_write(struct netsec_priv *priv, u32 addr, u32 value)
    {
    netsec_write(priv, MAC_REG_DATA, value);
    netsec_write(priv, MAC_REG_CMD, addr | NETSEC_GMAC_CMD_ST_WRITE);
    return netsec_wait_while_busy(priv,
    MAC_REG_CMD, NETSEC_GMAC_CMD_ST_BUSY);
    }
#[no_mangle]
unsafe extern "C" fn netsec_mac_read(priv: *mut netsec_priv, addr: u32, read: *mut u32) -> c_int {
    static int netsec_mac_read(struct netsec_priv *priv, u32 addr, u32 *read)
    {
    int ret;
    netsec_write(priv, MAC_REG_CMD, addr | NETSEC_GMAC_CMD_ST_READ);
    ret = netsec_wait_while_busy(priv,
    MAC_REG_CMD, NETSEC_GMAC_CMD_ST_BUSY);
    if (ret)
    return ret;
// read = netsec_read(priv, MAC_REG_DATA);
    return 0;
    }
    static int netsec_mac_wait_while_busy(struct netsec_priv *priv,
    u32 addr, u32 mask)
    {
    let mut timeout: u32 = TIMEOUT_SPINS_MAC;
    int ret, data;
    do {
    ret = netsec_mac_read(priv, addr, &data);
    if (ret)
    break;
    cpu_relax();
    } while (--timeout && (data & mask));
    if (timeout)
    return 0;
    timeout = TIMEOUT_SECONDARY_MS_MAC;
    do {
    usleep_range(1000, 2000);
    ret = netsec_mac_read(priv, addr, &data);
    if (ret)
    break;
    cpu_relax();
    } while (--timeout && (data & mask));
    if (timeout && !ret)
    return 0;
    netdev_WARN(priv.ndev, "%s: timeout\n", __func__);
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn netsec_mac_update_to_phy_state(priv: *mut netsec_priv) -> c_int {
    static int netsec_mac_update_to_phy_state(struct netsec_priv *priv)
    {
    struct phy_device *phydev = priv.ndev.phydev;
    let mut value: u32 = 0;
    value = phydev.duplex ? NETSEC_GMAC_MCR_REG_FULL_DUPLEX_COMMON :
    NETSEC_GMAC_MCR_REG_HALF_DUPLEX_COMMON;
    if (phydev.speed != SPEED_1000)
    value |= NETSEC_MCR_PS;
    if (priv.phy_interface != PHY_INTERFACE_MODE_GMII &&
    phydev.speed == SPEED_100)
    value |= NETSEC_GMAC_MCR_REG_FES;
    value |= NETSEC_GMAC_MCR_REG_CST | NETSEC_GMAC_MCR_REG_JE;
    if (phy_interface_mode_is_rgmii(priv.phy_interface))
    value |= NETSEC_GMAC_MCR_REG_IBN;
    if (netsec_mac_write(priv, GMAC_REG_MCR, value))
    return -ETIMEDOUT;
    return 0;
    }
    static int netsec_phy_read(struct mii_bus *bus, int phy_addr, int reg_addr);
    static int netsec_phy_write(struct mii_bus *bus,
    int phy_addr, int reg, u16 val)
    {
    int status;
    struct netsec_priv *priv = bus.priv;
    if (netsec_mac_write(priv, GMAC_REG_GDR, val))
    return -ETIMEDOUT;
    if (netsec_mac_write(priv, GMAC_REG_GAR,
    phy_addr << NETSEC_GMAC_GAR_REG_SHIFT_PA |
    reg << NETSEC_GMAC_GAR_REG_SHIFT_GR |
    NETSEC_GMAC_GAR_REG_GW | NETSEC_GMAC_GAR_REG_GB |
    (netsec_clk_type(priv.freq) <<
    GMAC_REG_SHIFT_CR_GAR)))
    return -ETIMEDOUT;
    status = netsec_mac_wait_while_busy(priv, GMAC_REG_GAR,
    NETSEC_GMAC_GAR_REG_GB);
// Developerbox implements RTL8211E PHY and there is
// a compatibility problem with F_GMAC4.
// RTL8211E expects MDC clock must be kept toggling for several
// clock cycle with MDIO high before entering the IDLE state.
// To meet this requirement, netsec driver needs to issue dummy
// read(e.g. read PHYID1(offset 0x2) register) right after write.
//
    netsec_phy_read(bus, phy_addr, MII_PHYSID1);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn netsec_phy_read(bus: *mut mii_bus, phy_addr: c_int, reg_addr: c_int) -> c_int {
    static int netsec_phy_read(struct mii_bus *bus, int phy_addr, int reg_addr)
    {
    struct netsec_priv *priv = bus.priv;
    u32 data;
    int ret;
    if (netsec_mac_write(priv, GMAC_REG_GAR, NETSEC_GMAC_GAR_REG_GB |
    phy_addr << NETSEC_GMAC_GAR_REG_SHIFT_PA |
    reg_addr << NETSEC_GMAC_GAR_REG_SHIFT_GR |
    (netsec_clk_type(priv.freq) <<
    GMAC_REG_SHIFT_CR_GAR)))
    return -ETIMEDOUT;
    ret = netsec_mac_wait_while_busy(priv, GMAC_REG_GAR,
    NETSEC_GMAC_GAR_REG_GB);
    if (ret)
    return ret;
    ret = netsec_mac_read(priv, GMAC_REG_GDR, &data);
    if (ret)
    return ret;
    return data;
    }
// ETHTOOL_OPS FOLLOW
    static void netsec_et_get_drvinfo(struct net_device *net_device,
    struct ethtool_drvinfo *info)
    {
    strscpy(info.driver, "netsec", sizeof(info.driver));
    strscpy(info.bus_info, dev_name(net_device.dev.parent),
    sizeof(info.bus_info));
    }
    static int netsec_et_get_coalesce(struct net_device *net_device,
    struct ethtool_coalesce *et_coalesce,
    struct kernel_ethtool_coalesce *kernel_coal,
    struct netlink_ext_ack *extack)
    {
    struct netsec_priv *priv = netdev_priv(net_device);
// et_coalesce = priv->et_coalesce;
    return 0;
    }
    static int netsec_et_set_coalesce(struct net_device *net_device,
    struct ethtool_coalesce *et_coalesce,
    struct kernel_ethtool_coalesce *kernel_coal,
    struct netlink_ext_ack *extack)
    {
    struct netsec_priv *priv = netdev_priv(net_device);
    priv.et_coalesce = *et_coalesce;
    if (priv.et_coalesce.tx_coalesce_usecs < 50)
    priv.et_coalesce.tx_coalesce_usecs = 50;
    if (priv.et_coalesce.tx_max_coalesced_frames < 1)
    priv.et_coalesce.tx_max_coalesced_frames = 1;
    netsec_write(priv, NETSEC_REG_NRM_TX_DONE_TXINT_PKTCNT,
    priv.et_coalesce.tx_max_coalesced_frames);
    netsec_write(priv, NETSEC_REG_NRM_TX_TXINT_TMR,
    priv.et_coalesce.tx_coalesce_usecs);
    netsec_write(priv, NETSEC_REG_NRM_TX_INTEN_SET, NRM_TX_ST_TXDONE);
    netsec_write(priv, NETSEC_REG_NRM_TX_INTEN_SET, NRM_TX_ST_TMREXP);
    if (priv.et_coalesce.rx_coalesce_usecs < 50)
    priv.et_coalesce.rx_coalesce_usecs = 50;
    if (priv.et_coalesce.rx_max_coalesced_frames < 1)
    priv.et_coalesce.rx_max_coalesced_frames = 1;
    netsec_write(priv, NETSEC_REG_NRM_RX_RXINT_PKTCNT,
    priv.et_coalesce.rx_max_coalesced_frames);
    netsec_write(priv, NETSEC_REG_NRM_RX_RXINT_TMR,
    priv.et_coalesce.rx_coalesce_usecs);
    netsec_write(priv, NETSEC_REG_NRM_RX_INTEN_SET, NRM_RX_ST_PKTCNT);
    netsec_write(priv, NETSEC_REG_NRM_RX_INTEN_SET, NRM_RX_ST_TMREXP);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netsec_et_get_msglevel(dev: *mut net_device) -> u32 {
    static u32 netsec_et_get_msglevel(struct net_device *dev)
    {
    struct netsec_priv *priv = netdev_priv(dev);
    return priv.msg_enable;
    }
#[no_mangle]
unsafe extern "C" fn netsec_et_set_msglevel(dev: *mut net_device, datum: u32) {
    static void netsec_et_set_msglevel(struct net_device *dev, u32 datum)
    {
    struct netsec_priv *priv = netdev_priv(dev);
    priv.msg_enable = datum;
    }
    static const struct ethtool_ops netsec_ethtool_ops = {
    .supported_coalesce_params = ETHTOOL_COALESCE_USECS |
    ETHTOOL_COALESCE_MAX_FRAMES,
    .get_drvinfo		= netsec_et_get_drvinfo,
    .get_link_ksettings	= phy_ethtool_get_link_ksettings,
    .set_link_ksettings	= phy_ethtool_set_link_ksettings,
    .get_link		= ethtool_op_get_link,
    .get_coalesce		= netsec_et_get_coalesce,
    .set_coalesce		= netsec_et_set_coalesce,
    .get_msglevel		= netsec_et_get_msglevel,
    .set_msglevel		= netsec_et_set_msglevel,
    };
// NETDEV_OPS FOLLOW
    static void netsec_set_rx_de(struct netsec_priv *priv,
    struct netsec_desc_ring *dring, u16 idx,
    const struct netsec_desc *desc)
    {
    struct netsec_de *de = dring.vaddr + DESC_SZ * idx;
    u32 attr = (1 << NETSEC_RX_PKT_OWN_FIELD) |
    (1 << NETSEC_RX_PKT_FS_FIELD) |
    (1 << NETSEC_RX_PKT_LS_FIELD);
    if (idx == DESC_NUM - 1)
    attr |= (1 << NETSEC_RX_PKT_LD_FIELD);
    de.data_buf_addr_up = upper_32_bits(desc.dma_addr);
    de.data_buf_addr_lw = lower_32_bits(desc.dma_addr);
    de.buf_len_info = desc.len;
    de.attr = attr;
    dma_wmb();
    dring.desc[idx].dma_addr = desc.dma_addr;
    dring.desc[idx].addr = desc.addr;
    dring.desc[idx].len = desc.len;
    }
#[no_mangle]
unsafe extern "C" fn netsec_clean_tx_dring(priv: *mut netsec_priv) -> bool {
    static bool netsec_clean_tx_dring(struct netsec_priv *priv)
    {
    struct netsec_desc_ring *dring = &priv.desc_ring[NETSEC_RING_TX];
    struct xdp_frame_bulk bq;
    struct netsec_de *entry;
    let mut tail: c_int = dring.tail;
    unsigned int bytes;
    let mut cnt: c_int = 0;
    spin_lock(&dring.lock);
    bytes = 0;
    xdp_frame_bulk_init(&bq);
    entry = dring.vaddr + DESC_SZ * tail;
    rcu_read_lock(); /* need for xdp_return_frame_bulk */
    while (!(entry.attr & (1U << NETSEC_TX_SHIFT_OWN_FIELD)) &&
    cnt < DESC_NUM) {
    struct netsec_desc *desc;
    int eop;
    desc = &dring.desc[tail];
    eop = (entry.attr >> NETSEC_TX_LAST) & 1;
    dma_rmb();
// if buf_type is either TYPE_NETSEC_SKB or
// TYPE_NETSEC_XDP_NDO we mapped it
//
    if (desc.buf_type != TYPE_NETSEC_XDP_TX)
    dma_unmap_single(priv.dev, desc.dma_addr, desc.len,
    DMA_TO_DEVICE);
    if (!eop)
    goto next;
    if (desc.buf_type == TYPE_NETSEC_SKB) {
    bytes += desc.skb.len;
    dev_kfree_skb(desc.skb);
    } else {
    bytes += desc.xdpf.len;
    if (desc.buf_type == TYPE_NETSEC_XDP_TX)
    xdp_return_frame_rx_napi(desc.xdpf);
    else
    xdp_return_frame_bulk(desc.xdpf, &bq);
    }
    next:
// clean up so netsec_uninit_pkt_dring() won't free the skb
// again
//
// desc = (struct netsec_desc){};
// entry->attr is not going to be accessed by the NIC until
// netsec_set_tx_de() is called. No need for a dma_wmb() here
//
    entry.attr = 1U << NETSEC_TX_SHIFT_OWN_FIELD;
// move tail ahead
    dring.tail = (tail + 1) % DESC_NUM;
    tail = dring.tail;
    entry = dring.vaddr + DESC_SZ * tail;
    cnt++;
    }
    xdp_flush_frame_bulk(&bq);
    rcu_read_unlock();
    spin_unlock(&dring.lock);
    if (!cnt)
    return false;
// reading the register clears the irq
    netsec_read(priv, NETSEC_REG_NRM_TX_DONE_PKTCNT);
    priv.ndev.stats.tx_packets += cnt;
    priv.ndev.stats.tx_bytes += bytes;
    netdev_completed_queue(priv.ndev, cnt, bytes);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn netsec_process_tx(priv: *mut netsec_priv) {
    static void netsec_process_tx(struct netsec_priv *priv)
    {
    struct net_device *ndev = priv.ndev;
    bool cleaned;
    cleaned = netsec_clean_tx_dring(priv);
    if (cleaned && netif_queue_stopped(ndev)) {
// Make sure we update the value, anyone stopping the queue
// after this will read the proper consumer idx
//
    smp_wmb();
    netif_wake_queue(ndev);
    }
    }
    static void *netsec_alloc_rx_data(struct netsec_priv *priv,
    dma_addr_t *dma_handle, u16 *desc_len)
    {
    struct netsec_desc_ring *dring = &priv.desc_ring[NETSEC_RING_RX];
    struct page *page;
    page = page_pool_dev_alloc_pages(dring.page_pool);
    if (!page)
    return core::ptr::null_mut();
// We allocate the same buffer length for XDP and non-XDP cases.
// page_pool API will map the whole page, skip what's needed for
// network payloads and/or XDP
//
// dma_handle = page_pool_get_dma_addr(page) + NETSEC_RXBUF_HEADROOM;
// Make sure the incoming payload fits in the page for XDP and non-XDP
// cases and reserve enough space for headroom + skb_shared_info
//
// desc_len = NETSEC_RX_BUF_SIZE;
    return page_address(page);
    }
#[no_mangle]
unsafe extern "C" fn netsec_rx_fill(priv: *mut netsec_priv, from: u16, num: u16) {
    static void netsec_rx_fill(struct netsec_priv *priv, u16 from, u16 num)
    {
    struct netsec_desc_ring *dring = &priv.desc_ring[NETSEC_RING_RX];
    let mut idx: u16 = from;
    while (num) {
    netsec_set_rx_de(priv, dring, idx, &dring.desc[idx]);
    idx++;
    if (idx >= DESC_NUM)
    idx = 0;
    num--;
    }
    }
#[no_mangle]
unsafe extern "C" fn netsec_xdp_ring_tx_db(priv: *mut netsec_priv, pkts: u16) {
    static void netsec_xdp_ring_tx_db(struct netsec_priv *priv, u16 pkts)
    {
    if (likely(pkts))
    netsec_write(priv, NETSEC_REG_NRM_TX_PKTCNT, pkts);
    }
    static void netsec_finalize_xdp_rx(struct netsec_priv *priv, u32 xdp_res,
    u16 pkts)
    {
    if (xdp_res & NETSEC_XDP_REDIR)
    xdp_do_flush();
    if (xdp_res & NETSEC_XDP_TX)
    netsec_xdp_ring_tx_db(priv, pkts);
    }
    static void netsec_set_tx_de(struct netsec_priv *priv,
    struct netsec_desc_ring *dring,
    const struct netsec_tx_pkt_ctrl *tx_ctrl,
    const struct netsec_desc *desc, void *buf)
    {
    let mut idx: c_int = dring.head;
    struct netsec_de *de;
    u32 attr;
    de = dring.vaddr + (DESC_SZ * idx);
    attr = (1 << NETSEC_TX_SHIFT_OWN_FIELD) |
    (1 << NETSEC_TX_SHIFT_PT_FIELD) |
    (NETSEC_RING_GMAC << NETSEC_TX_SHIFT_TDRID_FIELD) |
    (1 << NETSEC_TX_SHIFT_FS_FIELD) |
    (1 << NETSEC_TX_LAST) |
    (tx_ctrl.cksum_offload_flag << NETSEC_TX_SHIFT_CO) |
    (tx_ctrl.tcp_seg_offload_flag << NETSEC_TX_SHIFT_SO) |
    (1 << NETSEC_TX_SHIFT_TRS_FIELD);
    if (idx == DESC_NUM - 1)
    attr |= (1 << NETSEC_TX_SHIFT_LD_FIELD);
    de.data_buf_addr_up = upper_32_bits(desc.dma_addr);
    de.data_buf_addr_lw = lower_32_bits(desc.dma_addr);
    de.buf_len_info = (tx_ctrl.tcp_seg_len << 16) | desc.len;
    de.attr = attr;
    dring.desc[idx] = *desc;
    if (desc.buf_type == TYPE_NETSEC_SKB)
    dring.desc[idx].skb = buf;
    else if (desc.buf_type == TYPE_NETSEC_XDP_TX ||
    desc.buf_type == TYPE_NETSEC_XDP_NDO)
    dring.desc[idx].xdpf = buf;
// move head ahead
    dring.head = (dring.head + 1) % DESC_NUM;
    }
// The current driver only supports 1 Txq, this should run under spin_lock()
    static u32 netsec_xdp_queue_one(struct netsec_priv *priv,
    struct xdp_frame *xdpf, bool is_ndo)
    {
    struct netsec_desc_ring *tx_ring = &priv.desc_ring[NETSEC_RING_TX];
    struct page *page = virt_to_page(xdpf.data);
    let mut tx_ctrl: netsec_tx_pkt_ctrl = {};
    struct netsec_desc tx_desc;
    dma_addr_t dma_handle;
    u16 filled;
    if (tx_ring.head >= tx_ring.tail)
    filled = tx_ring.head - tx_ring.tail;
    else
    filled = tx_ring.head + DESC_NUM - tx_ring.tail;
    if (DESC_NUM - filled <= 1)
    return NETSEC_XDP_CONSUMED;
    if (is_ndo) {
// this is for ndo_xdp_xmit, the buffer needs mapping before
// sending
//
    dma_handle = dma_map_single(priv.dev, xdpf.data, xdpf.len,
    DMA_TO_DEVICE);
    if (dma_mapping_error(priv.dev, dma_handle))
    return NETSEC_XDP_CONSUMED;
    tx_desc.buf_type = TYPE_NETSEC_XDP_NDO;
    } else {
// This is the device Rx buffer from page_pool. No need to remap
// just sync and send it
//
    struct netsec_desc_ring *rx_ring =
    &priv.desc_ring[NETSEC_RING_RX];
    enum dma_data_direction dma_dir =
    page_pool_get_dma_dir(rx_ring.page_pool);
    dma_handle = page_pool_get_dma_addr(page) + xdpf.headroom +
    sizeof(*xdpf);
    dma_sync_single_for_device(priv.dev, dma_handle, xdpf.len,
    dma_dir);
    tx_desc.buf_type = TYPE_NETSEC_XDP_TX;
    }
    tx_desc.dma_addr = dma_handle;
    tx_desc.addr = xdpf.data;
    tx_desc.len = xdpf.len;
    netdev_sent_queue(priv.ndev, xdpf.len);
    netsec_set_tx_de(priv, tx_ring, &tx_ctrl, &tx_desc, xdpf);
    return NETSEC_XDP_TX;
    }
#[no_mangle]
unsafe extern "C" fn netsec_xdp_xmit_back(priv: *mut netsec_priv, xdp: *mut xdp_buff) -> u32 {
    static u32 netsec_xdp_xmit_back(struct netsec_priv *priv, struct xdp_buff *xdp)
    {
    struct netsec_desc_ring *tx_ring = &priv.desc_ring[NETSEC_RING_TX];
    struct xdp_frame *xdpf = xdp_convert_buff_to_frame(xdp);
    u32 ret;
    if (unlikely(!xdpf))
    return NETSEC_XDP_CONSUMED;
    spin_lock(&tx_ring.lock);
    ret = netsec_xdp_queue_one(priv, xdpf, false);
    spin_unlock(&tx_ring.lock);
    return ret;
    }
    static u32 netsec_run_xdp(struct netsec_priv *priv, struct bpf_prog *prog,
    struct xdp_buff *xdp)
    {
    struct netsec_desc_ring *dring = &priv.desc_ring[NETSEC_RING_RX];
    unsigned int sync, len = xdp.data_end - xdp.data;
    let mut ret: u32 = NETSEC_XDP_PASS;
    struct page *page;
    int err;
    u32 act;
    act = bpf_prog_run_xdp(prog, xdp);
// Due xdp_adjust_tail: DMA sync for_device cover max len CPU touch
    sync = xdp.data_end - xdp.data_hard_start - NETSEC_RXBUF_HEADROOM;
    sync = max(sync, len);
    switch (act) {
    case XDP_PASS:
    ret = NETSEC_XDP_PASS;
    break;
    case XDP_TX:
    ret = netsec_xdp_xmit_back(priv, xdp);
    if (ret != NETSEC_XDP_TX) {
    page = virt_to_head_page(xdp.data);
    page_pool_put_page(dring.page_pool, page, sync, true);
    }
    break;
    case XDP_REDIRECT:
    err = xdp_do_redirect(priv.ndev, xdp, prog);
    if (!err) {
    ret = NETSEC_XDP_REDIR;
    } else {
    ret = NETSEC_XDP_CONSUMED;
    page = virt_to_head_page(xdp.data);
    page_pool_put_page(dring.page_pool, page, sync, true);
    }
    break;
    default:
    bpf_warn_invalid_xdp_action(priv.ndev, prog, act);
    fallthrough;
    case XDP_ABORTED:
    trace_xdp_exception(priv.ndev, prog, act);
    fallthrough;	/* handle aborts by dropping packet */
    case XDP_DROP:
    ret = NETSEC_XDP_CONSUMED;
    page = virt_to_head_page(xdp.data);
    page_pool_put_page(dring.page_pool, page, sync, true);
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn netsec_process_rx(priv: *mut netsec_priv, budget: c_int) -> c_int {
    static int netsec_process_rx(struct netsec_priv *priv, int budget)
    {
    struct netsec_desc_ring *dring = &priv.desc_ring[NETSEC_RING_RX];
    struct net_device *ndev = priv.ndev;
    struct netsec_rx_pkt_info rx_info;
    enum dma_data_direction dma_dir;
    struct bpf_prog *xdp_prog;
    struct xdp_buff xdp;
    let mut xdp_xmit: u16 = 0;
    let mut xdp_act: u32 = 0;
    let mut done: c_int = 0;
    xdp_init_buff(&xdp, PAGE_SIZE, &dring.xdp_rxq);
    xdp_prog = READ_ONCE(priv.xdp_prog);
    dma_dir = page_pool_get_dma_dir(dring.page_pool);
    while (done < budget) {
    let mut idx: u16 = dring.tail;
    struct netsec_de *de = dring.vaddr + (DESC_SZ * idx);
    struct netsec_desc *desc = &dring.desc[idx];
    struct page *page = virt_to_page(desc.addr);
    u32 metasize, xdp_result = NETSEC_XDP_PASS;
    struct sk_buff *skb = core::ptr::null_mut();
    u16 pkt_len, desc_len;
    dma_addr_t dma_handle;
    void *buf_addr;
    if (de.attr & (1U << NETSEC_RX_PKT_OWN_FIELD)) {
// reading the register clears the irq
    netsec_read(priv, NETSEC_REG_NRM_RX_PKTCNT);
    break;
    }
// This  barrier is needed to keep us from reading
// any other fields out of the netsec_de until we have
// verified the descriptor has been written back
//
    dma_rmb();
    done++;
    pkt_len = de.buf_len_info >> 16;
    rx_info.err_code = (de.attr >> NETSEC_RX_PKT_ERR_FIELD) &
    NETSEC_RX_PKT_ERR_MASK;
    rx_info.err_flag = (de.attr >> NETSEC_RX_PKT_ER_FIELD) & 1;
    if (rx_info.err_flag) {
    netif_err(priv, drv, priv.ndev,
    "%s: rx fail err(%d)\n", __func__,
    rx_info.err_code);
    ndev.stats.rx_dropped++;
    dring.tail = (dring.tail + 1) % DESC_NUM;
// reuse buffer page frag
    netsec_rx_fill(priv, idx, 1);
    continue;
    }
    rx_info.rx_cksum_result =
    (de.attr >> NETSEC_RX_PKT_CO_FIELD) & 3;
// allocate a fresh buffer and map it to the hardware.
// This will eventually replace the old buffer in the hardware
//
    buf_addr = netsec_alloc_rx_data(priv, &dma_handle, &desc_len);
    if (unlikely(!buf_addr))
    break;
    dma_sync_single_for_cpu(priv.dev, desc.dma_addr, pkt_len,
    dma_dir);
    prefetch(desc.addr);
    xdp_prepare_buff(&xdp, desc.addr, NETSEC_RXBUF_HEADROOM,
    pkt_len, true);
    if (xdp_prog) {
    xdp_result = netsec_run_xdp(priv, xdp_prog, &xdp);
    if (xdp_result != NETSEC_XDP_PASS) {
    xdp_act |= xdp_result;
    if (xdp_result == NETSEC_XDP_TX)
    xdp_xmit++;
    goto next;
    }
    }
    skb = build_skb(desc.addr, desc.len + NETSEC_RX_BUF_NON_DATA);
    if (unlikely(!skb)) {
// If skb fails recycle_direct will either unmap and
// free the page or refill the cache depending on the
// cache state. Since we paid the allocation cost if
// building an skb fails try to put the page into cache
//
    page_pool_put_page(dring.page_pool, page, pkt_len,
    true);
    netif_err(priv, drv, priv.ndev,
    "rx failed to build skb\n");
    break;
    }
    skb_mark_for_recycle(skb);
    skb_reserve(skb, xdp.data - xdp.data_hard_start);
    skb_put(skb, xdp.data_end - xdp.data);
    metasize = xdp.data - xdp.data_meta;
    if (metasize)
    skb_metadata_set(skb, metasize);
    skb.protocol = eth_type_trans(skb, priv.ndev);
    if (priv.rx_cksum_offload_flag &&
    rx_info.rx_cksum_result == NETSEC_RX_CKSUM_OK)
    skb.ip_summed = CHECKSUM_UNNECESSARY;
    next:
    if (skb)
    napi_gro_receive(&priv.napi, skb);
    if (skb || xdp_result) {
    ndev.stats.rx_packets++;
    ndev.stats.rx_bytes += xdp.data_end - xdp.data;
    }
// Update the descriptor with fresh buffers
    desc.len = desc_len;
    desc.dma_addr = dma_handle;
    desc.addr = buf_addr;
    netsec_rx_fill(priv, idx, 1);
    dring.tail = (dring.tail + 1) % DESC_NUM;
    }
    netsec_finalize_xdp_rx(priv, xdp_act, xdp_xmit);
    return done;
    }
#[no_mangle]
unsafe extern "C" fn netsec_napi_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int netsec_napi_poll(struct napi_struct *napi, int budget)
    {
    struct netsec_priv *priv;
    int done;
    priv = container_of(napi, struct netsec_priv, napi);
    netsec_process_tx(priv);
    done = netsec_process_rx(priv, budget);
    if (done < budget && napi_complete_done(napi, done)) {
    unsigned long flags;
    spin_lock_irqsave(&priv.reglock, flags);
    netsec_write(priv, NETSEC_REG_INTEN_SET,
    NETSEC_IRQ_RX | NETSEC_IRQ_TX);
    spin_unlock_irqrestore(&priv.reglock, flags);
    }
    return done;
    }
#[no_mangle]
unsafe extern "C" fn netsec_desc_used(dring: *mut netsec_desc_ring) -> c_int {
    static int netsec_desc_used(struct netsec_desc_ring *dring)
    {
    int used;
    if (dring.head >= dring.tail)
    used = dring.head - dring.tail;
    else
    used = dring.head + DESC_NUM - dring.tail;
    return used;
    }
#[no_mangle]
unsafe extern "C" fn netsec_check_stop_tx(priv: *mut netsec_priv, used: c_int) -> c_int {
    static int netsec_check_stop_tx(struct netsec_priv *priv, int used)
    {
    struct netsec_desc_ring *dring = &priv.desc_ring[NETSEC_RING_TX];
// keep tail from touching the queue
    if (DESC_NUM - used < 2) {
    netif_stop_queue(priv.ndev);
// Make sure we read the updated value in case
// descriptors got freed
//
    smp_rmb();
    used = netsec_desc_used(dring);
    if (DESC_NUM - used < 2)
    return NETDEV_TX_BUSY;
    netif_wake_queue(priv.ndev);
    }
    return 0;
    }
    static netdev_tx_t netsec_netdev_start_xmit(struct sk_buff *skb,
    struct net_device *ndev)
    {
    struct netsec_priv *priv = netdev_priv(ndev);
    struct netsec_desc_ring *dring = &priv.desc_ring[NETSEC_RING_TX];
    let mut tx_ctrl: netsec_tx_pkt_ctrl = {};
    struct netsec_desc tx_desc;
    let mut tso_seg_len: u16 = 0;
    int filled;
    spin_lock_bh(&dring.lock);
    filled = netsec_desc_used(dring);
    if (netsec_check_stop_tx(priv, filled)) {
    spin_unlock_bh(&dring.lock);
    net_warn_ratelimited("%s %s Tx queue full\n",
    dev_name(priv.dev), ndev.name);
    return NETDEV_TX_BUSY;
    }
    if (skb.ip_summed == CHECKSUM_PARTIAL)
    tx_ctrl.cksum_offload_flag = true;
    if (skb_is_gso(skb))
    tso_seg_len = skb_shinfo(skb).gso_size;
    if (tso_seg_len > 0) {
    if (skb.protocol == htons(ETH_P_IP)) {
    ip_hdr(skb).tot_len = 0;
    tcp_hdr(skb).check =
    ~tcp_v4_check(0, ip_hdr(skb).saddr,
    ip_hdr(skb).daddr, 0);
    } else {
    tcp_v6_gso_csum_prep(skb);
    }
    tx_ctrl.tcp_seg_offload_flag = true;
    tx_ctrl.tcp_seg_len = tso_seg_len;
    }
    tx_desc.dma_addr = dma_map_single(priv.dev, skb.data,
    skb_headlen(skb), DMA_TO_DEVICE);
    if (dma_mapping_error(priv.dev, tx_desc.dma_addr)) {
    spin_unlock_bh(&dring.lock);
    netif_err(priv, drv, priv.ndev,
    "%s: DMA mapping failed\n", __func__);
    ndev.stats.tx_dropped++;
    dev_kfree_skb_any(skb);
    return NETDEV_TX_OK;
    }
    tx_desc.addr = skb.data;
    tx_desc.len = skb_headlen(skb);
    tx_desc.buf_type = TYPE_NETSEC_SKB;
    skb_tx_timestamp(skb);
    netdev_sent_queue(priv.ndev, skb.len);
    netsec_set_tx_de(priv, dring, &tx_ctrl, &tx_desc, skb);
    spin_unlock_bh(&dring.lock);
    netsec_write(priv, NETSEC_REG_NRM_TX_PKTCNT, 1); /* submit another tx */
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn netsec_uninit_pkt_dring(priv: *mut netsec_priv, id: c_int) {
    static void netsec_uninit_pkt_dring(struct netsec_priv *priv, int id)
    {
    struct netsec_desc_ring *dring = &priv.desc_ring[id];
    struct netsec_desc *desc;
    u16 idx;
    if (!dring.vaddr || !dring.desc)
    return;
    for (idx = 0; idx < DESC_NUM; idx++) {
    desc = &dring.desc[idx];
    if (!desc.addr)
    continue;
    if (id == NETSEC_RING_RX) {
    struct page *page = virt_to_page(desc.addr);
    page_pool_put_full_page(dring.page_pool, page, false);
    } else if (id == NETSEC_RING_TX) {
    dma_unmap_single(priv.dev, desc.dma_addr, desc.len,
    DMA_TO_DEVICE);
    dev_kfree_skb(desc.skb);
    }
    }
// Rx is currently using page_pool
    if (id == NETSEC_RING_RX) {
    if (xdp_rxq_info_is_reg(&dring.xdp_rxq))
    xdp_rxq_info_unreg(&dring.xdp_rxq);
    page_pool_destroy(dring.page_pool);
    }
    memset(dring.desc, 0, sizeof(struct netsec_desc) * DESC_NUM);
    memset(dring.vaddr, 0, DESC_SZ * DESC_NUM);
    dring.head = 0;
    dring.tail = 0;
    if (id == NETSEC_RING_TX)
    netdev_reset_queue(priv.ndev);
    }
#[no_mangle]
unsafe extern "C" fn netsec_free_dring(priv: *mut netsec_priv, id: c_int) {
    static void netsec_free_dring(struct netsec_priv *priv, int id)
    {
    struct netsec_desc_ring *dring = &priv.desc_ring[id];
    if (dring.vaddr) {
    dma_free_coherent(priv.dev, DESC_SZ * DESC_NUM,
    dring.vaddr, dring.desc_dma);
    dring.vaddr = core::ptr::null_mut();
    }
    kfree(dring.desc);
    dring.desc = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn netsec_alloc_dring(priv: *mut netsec_priv, id: enum ring_id) -> c_int {
    static int netsec_alloc_dring(struct netsec_priv *priv, enum ring_id id)
    {
    struct netsec_desc_ring *dring = &priv.desc_ring[id];
    dring.vaddr = dma_alloc_coherent(priv.dev, DESC_SZ * DESC_NUM,
    &dring.desc_dma, GFP_KERNEL);
    if (!dring.vaddr)
    goto err;
    dring.desc = kzalloc_objs(*dring.desc, DESC_NUM);
    if (!dring.desc)
    goto err;
    return 0;
    err:
    netsec_free_dring(priv, id);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn netsec_setup_tx_dring(priv: *mut netsec_priv) {
    static void netsec_setup_tx_dring(struct netsec_priv *priv)
    {
    struct netsec_desc_ring *dring = &priv.desc_ring[NETSEC_RING_TX];
    int i;
    for (i = 0; i < DESC_NUM; i++) {
    struct netsec_de *de;
    de = dring.vaddr + (DESC_SZ * i);
// de->attr is not going to be accessed by the NIC
// until netsec_set_tx_de() is called.
// No need for a dma_wmb() here
//
    de.attr = 1U << NETSEC_TX_SHIFT_OWN_FIELD;
    }
    }
#[no_mangle]
unsafe extern "C" fn netsec_setup_rx_dring(priv: *mut netsec_priv) -> c_int {
    static int netsec_setup_rx_dring(struct netsec_priv *priv)
    {
    struct netsec_desc_ring *dring = &priv.desc_ring[NETSEC_RING_RX];
    struct bpf_prog *xdp_prog = READ_ONCE(priv.xdp_prog);
    struct page_pool_params pp_params = {
    .order = 0,
// internal DMA mapping in page_pool
    .flags = PP_FLAG_DMA_MAP | PP_FLAG_DMA_SYNC_DEV,
    .pool_size = DESC_NUM,
    .nid = NUMA_NO_NODE,
    .dev = priv.dev,
    .dma_dir = xdp_prog ? DMA_BIDIRECTIONAL : DMA_FROM_DEVICE,
    .offset = NETSEC_RXBUF_HEADROOM,
    .max_len = NETSEC_RX_BUF_SIZE,
    .napi = &priv.napi,
    .netdev = priv.ndev,
    };
    int i, err;
    dring.page_pool = page_pool_create(&pp_params);
    if (IS_ERR(dring.page_pool)) {
    err = PTR_ERR(dring.page_pool);
    dring.page_pool = core::ptr::null_mut();
    goto err_out;
    }
    err = xdp_rxq_info_reg(&dring.xdp_rxq, priv.ndev, 0, priv.napi.napi_id);
    if (err)
    goto err_out;
    err = xdp_rxq_info_reg_mem_model(&dring.xdp_rxq, MEM_TYPE_PAGE_POOL,
    dring.page_pool);
    if (err)
    goto err_out;
    for (i = 0; i < DESC_NUM; i++) {
    struct netsec_desc *desc = &dring.desc[i];
    dma_addr_t dma_handle;
    void *buf;
    u16 len;
    buf = netsec_alloc_rx_data(priv, &dma_handle, &len);
    if (!buf) {
    err = -ENOMEM;
    goto err_out;
    }
    desc.dma_addr = dma_handle;
    desc.addr = buf;
    desc.len = len;
    }
    netsec_rx_fill(priv, 0, DESC_NUM);
    return 0;
    err_out:
    netsec_uninit_pkt_dring(priv, NETSEC_RING_RX);
    return err;
    }
    static int netsec_netdev_load_ucode_region(struct netsec_priv *priv, u32 reg,
    u32 addr_h, u32 addr_l, u32 size)
    {
    let mut base: u64 = (u64)addr_h << 32 | addr_l;
    void __iomem *ucode;
    u32 i;
    ucode = ioremap(base, size * sizeof(u32));
    if (!ucode)
    return -ENOMEM;
    for (i = 0; i < size; i++)
    netsec_write(priv, reg, readl(ucode + i * 4));
    iounmap(ucode);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netsec_netdev_load_microcode(priv: *mut netsec_priv) -> c_int {
    static int netsec_netdev_load_microcode(struct netsec_priv *priv)
    {
    u32 addr_h, addr_l, size;
    int err;
    addr_h = readl(priv.eeprom_base + NETSEC_EEPROM_HM_ME_ADDRESS_H);
    addr_l = readl(priv.eeprom_base + NETSEC_EEPROM_HM_ME_ADDRESS_L);
    size = readl(priv.eeprom_base + NETSEC_EEPROM_HM_ME_SIZE);
    err = netsec_netdev_load_ucode_region(priv, NETSEC_REG_DMAC_HM_CMD_BUF,
    addr_h, addr_l, size);
    if (err)
    return err;
    addr_h = readl(priv.eeprom_base + NETSEC_EEPROM_MH_ME_ADDRESS_H);
    addr_l = readl(priv.eeprom_base + NETSEC_EEPROM_MH_ME_ADDRESS_L);
    size = readl(priv.eeprom_base + NETSEC_EEPROM_MH_ME_SIZE);
    err = netsec_netdev_load_ucode_region(priv, NETSEC_REG_DMAC_MH_CMD_BUF,
    addr_h, addr_l, size);
    if (err)
    return err;
    addr_h = 0;
    addr_l = readl(priv.eeprom_base + NETSEC_EEPROM_PKT_ME_ADDRESS);
    size = readl(priv.eeprom_base + NETSEC_EEPROM_PKT_ME_SIZE);
    err = netsec_netdev_load_ucode_region(priv, NETSEC_REG_PKT_CMD_BUF,
    addr_h, addr_l, size);
    if (err)
    return err;
    return 0;
    }
    static int netsec_reset_hardware(struct netsec_priv *priv,
    bool load_ucode)
    {
    u32 value;
    int err;
// stop DMA engines
    if (!netsec_read(priv, NETSEC_REG_ADDR_DIS_CORE)) {
    netsec_write(priv, NETSEC_REG_DMA_HM_CTRL,
    NETSEC_DMA_CTRL_REG_STOP);
    netsec_write(priv, NETSEC_REG_DMA_MH_CTRL,
    NETSEC_DMA_CTRL_REG_STOP);
    while (netsec_read(priv, NETSEC_REG_DMA_HM_CTRL) &
    NETSEC_DMA_CTRL_REG_STOP)
    cpu_relax();
    while (netsec_read(priv, NETSEC_REG_DMA_MH_CTRL) &
    NETSEC_DMA_CTRL_REG_STOP)
    cpu_relax();
    }
    netsec_write(priv, NETSEC_REG_SOFT_RST, NETSEC_SOFT_RST_REG_RESET);
    netsec_write(priv, NETSEC_REG_SOFT_RST, NETSEC_SOFT_RST_REG_RUN);
    netsec_write(priv, NETSEC_REG_COM_INIT, NETSEC_COM_INIT_REG_ALL);
    while (netsec_read(priv, NETSEC_REG_COM_INIT) != 0)
    cpu_relax();
// set desc_start addr
    netsec_write(priv, NETSEC_REG_NRM_RX_DESC_START_UP,
    upper_32_bits(priv.desc_ring[NETSEC_RING_RX].desc_dma));
    netsec_write(priv, NETSEC_REG_NRM_RX_DESC_START_LW,
    lower_32_bits(priv.desc_ring[NETSEC_RING_RX].desc_dma));
    netsec_write(priv, NETSEC_REG_NRM_TX_DESC_START_UP,
    upper_32_bits(priv.desc_ring[NETSEC_RING_TX].desc_dma));
    netsec_write(priv, NETSEC_REG_NRM_TX_DESC_START_LW,
    lower_32_bits(priv.desc_ring[NETSEC_RING_TX].desc_dma));
// set normal tx dring ring config
    netsec_write(priv, NETSEC_REG_NRM_TX_CONFIG,
    1 << NETSEC_REG_DESC_ENDIAN);
    netsec_write(priv, NETSEC_REG_NRM_RX_CONFIG,
    1 << NETSEC_REG_DESC_ENDIAN);
    if (load_ucode) {
    err = netsec_netdev_load_microcode(priv);
    if (err) {
    netif_err(priv, probe, priv.ndev,
    "%s: failed to load microcode (%d)\n",
    __func__, err);
    return err;
    }
    }
// start DMA engines
    netsec_write(priv, NETSEC_REG_DMA_TMR_CTRL, priv.freq / 1000000 - 1);
    netsec_write(priv, NETSEC_REG_ADDR_DIS_CORE, 0);
    usleep_range(1000, 2000);
    if (!(netsec_read(priv, NETSEC_REG_TOP_STATUS) &
    NETSEC_TOP_IRQ_REG_CODE_LOAD_END)) {
    netif_err(priv, probe, priv.ndev,
    "microengine start failed\n");
    return -ENXIO;
    }
    netsec_write(priv, NETSEC_REG_TOP_STATUS,
    NETSEC_TOP_IRQ_REG_CODE_LOAD_END);
    value = NETSEC_PKT_CTRL_REG_MODE_NRM;
    if (priv.ndev.mtu > ETH_DATA_LEN)
    value |= NETSEC_PKT_CTRL_REG_EN_JUMBO;
// change to normal mode
    netsec_write(priv, NETSEC_REG_DMA_MH_CTRL, MH_CTRL__MODE_TRANS);
    netsec_write(priv, NETSEC_REG_PKT_CTRL, value);
    while ((netsec_read(priv, NETSEC_REG_MODE_TRANS_COMP_STATUS) &
    NETSEC_MODE_TRANS_COMP_IRQ_T2N) == 0)
    cpu_relax();
// clear any pending EMPTY/ERR irq status
    netsec_write(priv, NETSEC_REG_NRM_TX_STATUS, ~0);
// Disable TX & RX intr
    netsec_write(priv, NETSEC_REG_INTEN_CLR, ~0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netsec_start_gmac(priv: *mut netsec_priv) -> c_int {
    static int netsec_start_gmac(struct netsec_priv *priv)
    {
    struct phy_device *phydev = priv.ndev.phydev;
    let mut value: u32 = 0;
    int ret;
    if (phydev.speed != SPEED_1000)
    value = (NETSEC_GMAC_MCR_REG_CST |
    NETSEC_GMAC_MCR_REG_HALF_DUPLEX_COMMON);
    if (netsec_mac_write(priv, GMAC_REG_MCR, value))
    return -ETIMEDOUT;
    if (netsec_mac_write(priv, GMAC_REG_BMR,
    NETSEC_GMAC_BMR_REG_RESET))
    return -ETIMEDOUT;
// Wait soft reset
    usleep_range(1000, 5000);
    ret = netsec_mac_read(priv, GMAC_REG_BMR, &value);
    if (ret)
    return ret;
    if (value & NETSEC_GMAC_BMR_REG_SWR)
    return -EAGAIN;
    netsec_write(priv, MAC_REG_DESC_SOFT_RST, 1);
    if (netsec_wait_while_busy(priv, MAC_REG_DESC_SOFT_RST, 1))
    return -ETIMEDOUT;
    netsec_write(priv, MAC_REG_DESC_INIT, 1);
    if (netsec_wait_while_busy(priv, MAC_REG_DESC_INIT, 1))
    return -ETIMEDOUT;
    if (netsec_mac_write(priv, GMAC_REG_BMR,
    NETSEC_GMAC_BMR_REG_COMMON))
    return -ETIMEDOUT;
    if (netsec_mac_write(priv, GMAC_REG_RDLAR,
    NETSEC_GMAC_RDLAR_REG_COMMON))
    return -ETIMEDOUT;
    if (netsec_mac_write(priv, GMAC_REG_TDLAR,
    NETSEC_GMAC_TDLAR_REG_COMMON))
    return -ETIMEDOUT;
    if (netsec_mac_write(priv, GMAC_REG_MFFR, 0x80000001))
    return -ETIMEDOUT;
    ret = netsec_mac_update_to_phy_state(priv);
    if (ret)
    return ret;
    ret = netsec_mac_read(priv, GMAC_REG_OMR, &value);
    if (ret)
    return ret;
    value |= NETSEC_GMAC_OMR_REG_SR;
    value |= NETSEC_GMAC_OMR_REG_ST;
    netsec_write(priv, NETSEC_REG_NRM_RX_INTEN_CLR, ~0);
    netsec_write(priv, NETSEC_REG_NRM_TX_INTEN_CLR, ~0);
    netsec_et_set_coalesce(priv.ndev, &priv.et_coalesce, core::ptr::null_mut(), core::ptr::null_mut());
    if (netsec_mac_write(priv, GMAC_REG_OMR, value))
    return -ETIMEDOUT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netsec_stop_gmac(priv: *mut netsec_priv) -> c_int {
    static int netsec_stop_gmac(struct netsec_priv *priv)
    {
    u32 value;
    int ret;
    ret = netsec_mac_read(priv, GMAC_REG_OMR, &value);
    if (ret)
    return ret;
    value &= ~NETSEC_GMAC_OMR_REG_SR;
    value &= ~NETSEC_GMAC_OMR_REG_ST;
// disable all interrupts
    netsec_write(priv, NETSEC_REG_NRM_RX_INTEN_CLR, ~0);
    netsec_write(priv, NETSEC_REG_NRM_TX_INTEN_CLR, ~0);
    return netsec_mac_write(priv, GMAC_REG_OMR, value);
    }
#[no_mangle]
unsafe extern "C" fn netsec_phy_adjust_link(ndev: *mut net_device) {
    static void netsec_phy_adjust_link(struct net_device *ndev)
    {
    struct netsec_priv *priv = netdev_priv(ndev);
    if (ndev.phydev.link)
    netsec_start_gmac(priv);
    else
    netsec_stop_gmac(priv);
    phy_print_status(ndev.phydev);
    }
#[no_mangle]
unsafe extern "C" fn netsec_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t netsec_irq_handler(int irq, void *dev_id)
    {
    struct netsec_priv *priv = dev_id;
    u32 val, status = netsec_read(priv, NETSEC_REG_TOP_STATUS);
    unsigned long flags;
// Disable interrupts
    if (status & NETSEC_IRQ_TX) {
    val = netsec_read(priv, NETSEC_REG_NRM_TX_STATUS);
    netsec_write(priv, NETSEC_REG_NRM_TX_STATUS, val);
    }
    if (status & NETSEC_IRQ_RX) {
    val = netsec_read(priv, NETSEC_REG_NRM_RX_STATUS);
    netsec_write(priv, NETSEC_REG_NRM_RX_STATUS, val);
    }
    spin_lock_irqsave(&priv.reglock, flags);
    netsec_write(priv, NETSEC_REG_INTEN_CLR, NETSEC_IRQ_RX | NETSEC_IRQ_TX);
    spin_unlock_irqrestore(&priv.reglock, flags);
    napi_schedule(&priv.napi);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn netsec_netdev_open(ndev: *mut net_device) -> c_int {
    static int netsec_netdev_open(struct net_device *ndev)
    {
    struct netsec_priv *priv = netdev_priv(ndev);
    int ret;
    pm_runtime_get_sync(priv.dev);
    netsec_setup_tx_dring(priv);
    ret = netsec_setup_rx_dring(priv);
    if (ret) {
    netif_err(priv, probe, priv.ndev,
    "%s: fail setup ring\n", __func__);
    goto err1;
    }
    ret = request_irq(priv.ndev.irq, netsec_irq_handler,
    IRQF_SHARED, "netsec", priv);
    if (ret) {
    netif_err(priv, drv, priv.ndev, "request_irq failed\n");
    goto err2;
    }
    if (dev_of_node(priv.dev)) {
    if (!of_phy_connect(priv.ndev, priv.phy_np,
    netsec_phy_adjust_link, 0,
    priv.phy_interface)) {
    netif_err(priv, link, priv.ndev, "missing PHY\n");
    ret = -ENODEV;
    goto err3;
    }
    } else {
    ret = phy_connect_direct(priv.ndev, priv.phydev,
    netsec_phy_adjust_link,
    priv.phy_interface);
    if (ret) {
    netif_err(priv, link, priv.ndev,
    "phy_connect_direct() failed (%d)\n", ret);
    goto err3;
    }
    }
    phy_start(ndev.phydev);
    netsec_start_gmac(priv);
    napi_enable(&priv.napi);
    netif_start_queue(ndev);
// Enable TX+RX intr.
    netsec_write(priv, NETSEC_REG_INTEN_SET, NETSEC_IRQ_RX | NETSEC_IRQ_TX);
    return 0;
    err3:
    free_irq(priv.ndev.irq, priv);
    err2:
    netsec_uninit_pkt_dring(priv, NETSEC_RING_RX);
    err1:
    pm_runtime_put_sync(priv.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn netsec_netdev_stop(ndev: *mut net_device) -> c_int {
    static int netsec_netdev_stop(struct net_device *ndev)
    {
    int ret;
    struct netsec_priv *priv = netdev_priv(ndev);
    netif_stop_queue(priv.ndev);
    dma_wmb();
    napi_disable(&priv.napi);
    netsec_write(priv, NETSEC_REG_INTEN_CLR, ~0);
    netsec_stop_gmac(priv);
    free_irq(priv.ndev.irq, priv);
    netsec_uninit_pkt_dring(priv, NETSEC_RING_TX);
    netsec_uninit_pkt_dring(priv, NETSEC_RING_RX);
    phy_stop(ndev.phydev);
    phy_disconnect(ndev.phydev);
    ret = netsec_reset_hardware(priv, false);
    pm_runtime_put_sync(priv.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn netsec_netdev_init(ndev: *mut net_device) -> c_int {
    static int netsec_netdev_init(struct net_device *ndev)
    {
    struct netsec_priv *priv = netdev_priv(ndev);
    int ret;
    u16 data;
    BUILD_BUG_ON_NOT_POWER_OF_2(DESC_NUM);
    ret = netsec_alloc_dring(priv, NETSEC_RING_TX);
    if (ret)
    return ret;
    ret = netsec_alloc_dring(priv, NETSEC_RING_RX);
    if (ret)
    goto err1;
// set phy power down
    data = netsec_phy_read(priv.mii_bus, priv.phy_addr, MII_BMCR);
    netsec_phy_write(priv.mii_bus, priv.phy_addr, MII_BMCR,
    data | BMCR_PDOWN);
    ret = netsec_reset_hardware(priv, true);
    if (ret)
    goto err2;
// Restore phy power state
    netsec_phy_write(priv.mii_bus, priv.phy_addr, MII_BMCR, data);
    spin_lock_init(&priv.desc_ring[NETSEC_RING_TX].lock);
    spin_lock_init(&priv.desc_ring[NETSEC_RING_RX].lock);
    return 0;
    err2:
    netsec_free_dring(priv, NETSEC_RING_RX);
    err1:
    netsec_free_dring(priv, NETSEC_RING_TX);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn netsec_netdev_uninit(ndev: *mut net_device) {
    static void netsec_netdev_uninit(struct net_device *ndev)
    {
    struct netsec_priv *priv = netdev_priv(ndev);
    netsec_free_dring(priv, NETSEC_RING_RX);
    netsec_free_dring(priv, NETSEC_RING_TX);
    }
    static int netsec_netdev_set_features(struct net_device *ndev,
    netdev_features_t features)
    {
    struct netsec_priv *priv = netdev_priv(ndev);
    priv.rx_cksum_offload_flag = !!(features & NETIF_F_RXCSUM);
    return 0;
    }
    static int netsec_xdp_xmit(struct net_device *ndev, int n,
    struct xdp_frame **frames, u32 flags)
    {
    struct netsec_priv *priv = netdev_priv(ndev);
    struct netsec_desc_ring *tx_ring = &priv.desc_ring[NETSEC_RING_TX];
    int i, nxmit = 0;
    if (unlikely(flags & ~XDP_XMIT_FLAGS_MASK))
    return -EINVAL;
    spin_lock(&tx_ring.lock);
    for (i = 0; i < n; i++) {
    struct xdp_frame *xdpf = frames[i];
    int err;
    err = netsec_xdp_queue_one(priv, xdpf, true);
    if (err != NETSEC_XDP_TX)
    break;
    tx_ring.xdp_xmit++;
    nxmit++;
    }
    spin_unlock(&tx_ring.lock);
    if (unlikely(flags & XDP_XMIT_FLUSH)) {
    netsec_xdp_ring_tx_db(priv, tx_ring.xdp_xmit);
    tx_ring.xdp_xmit = 0;
    }
    return nxmit;
    }
    static int netsec_xdp_setup(struct netsec_priv *priv, struct bpf_prog *prog,
    struct netlink_ext_ack *extack)
    {
    struct net_device *dev = priv.ndev;
    struct bpf_prog *old_prog;
// For now just support only the usual MTU sized frames
    if (prog && dev.mtu > 1500) {
    NL_SET_ERR_MSG_MOD(extack, "Jumbo frames not supported on XDP");
    return -EOPNOTSUPP;
    }
    if (netif_running(dev))
    netsec_netdev_stop(dev);
// Detach old prog, if any
    old_prog = xchg(&priv.xdp_prog, prog);
    if (old_prog)
    bpf_prog_put(old_prog);
    if (netif_running(dev))
    netsec_netdev_open(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netsec_xdp(ndev: *mut net_device, xdp: *mut netdev_bpf) -> c_int {
    static int netsec_xdp(struct net_device *ndev, struct netdev_bpf *xdp)
    {
    struct netsec_priv *priv = netdev_priv(ndev);
    switch (xdp.command) {
    case XDP_SETUP_PROG:
    return netsec_xdp_setup(priv, xdp.prog, xdp.extack);
    default:
    return -EINVAL;
    }
    }
    static const struct net_device_ops netsec_netdev_ops = {
    .ndo_init		= netsec_netdev_init,
    .ndo_uninit		= netsec_netdev_uninit,
    .ndo_open		= netsec_netdev_open,
    .ndo_stop		= netsec_netdev_stop,
    .ndo_start_xmit		= netsec_netdev_start_xmit,
    .ndo_set_features	= netsec_netdev_set_features,
    .ndo_set_mac_address    = eth_mac_addr,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_eth_ioctl		= phy_do_ioctl,
    .ndo_xdp_xmit		= netsec_xdp_xmit,
    .ndo_bpf		= netsec_xdp,
    };
    static int netsec_of_probe(struct platform_device *pdev,
    struct netsec_priv *priv, u32 *phy_addr)
    {
    int err;
    err = of_get_phy_mode(pdev.dev.of_node, &priv.phy_interface);
    if (err) {
    dev_err(&pdev.dev, "missing required property 'phy-mode'\n");
    return err;
    }
//
// SynQuacer is physically configured with TX and RX delays
// but the standard firmware claimed otherwise for a long
// time, ignore it.
//
    if (of_machine_is_compatible("socionext,developer-box") &&
    priv.phy_interface != PHY_INTERFACE_MODE_RGMII_ID) {
    dev_warn(&pdev.dev, "Outdated firmware reports incorrect PHY mode, overriding\n");
    priv.phy_interface = PHY_INTERFACE_MODE_RGMII_ID;
    }
    priv.phy_np = of_parse_phandle(pdev.dev.of_node, "phy-handle", 0);
    if (!priv.phy_np) {
    dev_err(&pdev.dev, "missing required property 'phy-handle'\n");
    return -EINVAL;
    }
// phy_addr = of_mdio_parse_addr(&pdev->dev, priv->phy_np);
    priv.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut()); /* get by 'phy_ref_clk' */
    if (IS_ERR(priv.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(priv.clk),
    "phy_ref_clk not found\n");
    priv.freq = clk_get_rate(priv.clk);
    return 0;
    }
    static int netsec_acpi_probe(struct platform_device *pdev,
    struct netsec_priv *priv, u32 *phy_addr)
    {
    int ret;
    if (!IS_ENABLED(CONFIG_ACPI))
    return -ENODEV;
// ACPI systems are assumed to configure the PHY in firmware, so
// there is really no need to discover the PHY mode from the DSDT.
// Since firmware is known to exist in the field that configures the
// PHY correctly but passes the wrong mode string in the phy-mode
// device property, we have no choice but to ignore it.
//
    priv.phy_interface = PHY_INTERFACE_MODE_NA;
    ret = device_property_read_u32(&pdev.dev, "phy-channel", phy_addr);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "missing required property 'phy-channel'\n");
    ret = device_property_read_u32(&pdev.dev,
    "socionext,phy-clock-frequency",
    &priv.freq);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "missing required property 'socionext,phy-clock-frequency'\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netsec_unregister_mdio(priv: *mut netsec_priv) {
    static void netsec_unregister_mdio(struct netsec_priv *priv)
    {
    struct phy_device *phydev = priv.phydev;
    if (!dev_of_node(priv.dev) && phydev) {
    phy_device_remove(phydev);
    phy_device_free(phydev);
    }
    mdiobus_unregister(priv.mii_bus);
    }
#[no_mangle]
unsafe extern "C" fn netsec_register_mdio(priv: *mut netsec_priv, phy_addr: u32) -> c_int {
    static int netsec_register_mdio(struct netsec_priv *priv, u32 phy_addr)
    {
    struct mii_bus *bus;
    int ret;
    bus = devm_mdiobus_alloc(priv.dev);
    if (!bus)
    return -ENOMEM;
    snprintf(bus.id, MII_BUS_ID_SIZE, "%s", dev_name(priv.dev));
    bus.priv = priv;
    bus.name = "SNI NETSEC MDIO";
    bus.read = netsec_phy_read;
    bus.write = netsec_phy_write;
    bus.parent = priv.dev;
    priv.mii_bus = bus;
    if (dev_of_node(priv.dev)) {
    struct device_node *mdio_node, *parent = dev_of_node(priv.dev);
    mdio_node = of_get_child_by_name(parent, "mdio");
    if (mdio_node) {
    parent = mdio_node;
    } else {
// older f/w doesn't populate the mdio subnode,
// allow relaxed upgrade of f/w in due time.
//
    dev_info(priv.dev, "Upgrade f/w for mdio subnode!\n");
    }
    ret = of_mdiobus_register(bus, parent);
    of_node_put(mdio_node);
    if (ret) {
    dev_err(priv.dev, "mdiobus register err(%d)\n", ret);
    return ret;
    }
    } else {
// Mask out all PHYs from auto probing.
    bus.phy_mask = ~0;
    ret = mdiobus_register(bus);
    if (ret) {
    dev_err(priv.dev, "mdiobus register err(%d)\n", ret);
    return ret;
    }
    priv.phydev = get_phy_device(bus, phy_addr, false);
    if (IS_ERR(priv.phydev)) {
    ret = PTR_ERR(priv.phydev);
    dev_err(priv.dev, "get_phy_device err(%d)\n", ret);
    priv.phydev = core::ptr::null_mut();
    mdiobus_unregister(bus);
    return -ENODEV;
    }
    ret = phy_device_register(priv.phydev);
    if (ret) {
    phy_device_free(priv.phydev);
    mdiobus_unregister(bus);
    dev_err(priv.dev,
    "phy_device_register err(%d)\n", ret);
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn netsec_probe(pdev: *mut platform_device) -> c_int {
    static int netsec_probe(struct platform_device *pdev)
    {
    struct resource *mmio_res, *eeprom_res;
    struct netsec_priv *priv;
    u32 hw_ver, phy_addr = 0;
    struct net_device *ndev;
    int ret;
    int irq;
    mmio_res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!mmio_res) {
    dev_err(&pdev.dev, "No MMIO resource found.\n");
    return -ENODEV;
    }
    eeprom_res = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    if (!eeprom_res) {
    dev_info(&pdev.dev, "No EEPROM resource found.\n");
    return -ENODEV;
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ndev = alloc_etherdev(sizeof(*priv));
    if (!ndev)
    return -ENOMEM;
    priv = netdev_priv(ndev);
    spin_lock_init(&priv.reglock);
    SET_NETDEV_DEV(ndev, &pdev.dev);
    platform_set_drvdata(pdev, priv);
    ndev.irq = irq;
    priv.dev = &pdev.dev;
    priv.ndev = ndev;
    priv.msg_enable = NETIF_MSG_TX_ERR | NETIF_MSG_HW | NETIF_MSG_DRV |
    NETIF_MSG_LINK | NETIF_MSG_PROBE;
    priv.ioaddr = devm_ioremap(&pdev.dev, mmio_res.start,
    resource_size(mmio_res));
    if (!priv.ioaddr) {
    dev_err(&pdev.dev, "devm_ioremap() failed\n");
    ret = -ENXIO;
    goto free_ndev;
    }
    priv.eeprom_base = devm_ioremap(&pdev.dev, eeprom_res.start,
    resource_size(eeprom_res));
    if (!priv.eeprom_base) {
    dev_err(&pdev.dev, "devm_ioremap() failed for EEPROM\n");
    ret = -ENXIO;
    goto free_ndev;
    }
    ret = device_get_ethdev_address(&pdev.dev, ndev);
    if (ret && priv.eeprom_base) {
    void __iomem *macp = priv.eeprom_base +
    NETSEC_EEPROM_MAC_ADDRESS;
    u8 addr[ETH_ALEN];
    addr[0] = readb(macp + 3);
    addr[1] = readb(macp + 2);
    addr[2] = readb(macp + 1);
    addr[3] = readb(macp + 0);
    addr[4] = readb(macp + 7);
    addr[5] = readb(macp + 6);
    eth_hw_addr_set(ndev, addr);
    }
    if (!is_valid_ether_addr(ndev.dev_addr)) {
    dev_warn(&pdev.dev, "No MAC address found, using random\n");
    eth_hw_addr_random(ndev);
    }
    if (dev_of_node(&pdev.dev))
    ret = netsec_of_probe(pdev, priv, &phy_addr);
    else
    ret = netsec_acpi_probe(pdev, priv, &phy_addr);
    if (ret)
    goto free_ndev;
    priv.phy_addr = phy_addr;
    if (!priv.freq) {
    dev_err(&pdev.dev, "missing PHY reference clock frequency\n");
    ret = -ENODEV;
    goto free_ndev;
    }
// default for throughput
    priv.et_coalesce.rx_coalesce_usecs = 500;
    priv.et_coalesce.rx_max_coalesced_frames = 8;
    priv.et_coalesce.tx_coalesce_usecs = 500;
    priv.et_coalesce.tx_max_coalesced_frames = 8;
    ret = device_property_read_u32(&pdev.dev, "max-frame-size",
    &ndev.max_mtu);
    if (ret < 0)
    ndev.max_mtu = ETH_DATA_LEN;
// runtime_pm coverage just for probe, open/close also cover it
    pm_runtime_enable(&pdev.dev);
    pm_runtime_get_sync(&pdev.dev);
    hw_ver = netsec_read(priv, NETSEC_REG_F_TAIKI_VER);
// this driver only supports F_TAIKI style NETSEC
    if (NETSEC_F_NETSEC_VER_MAJOR_NUM(hw_ver) !=
    NETSEC_F_NETSEC_VER_MAJOR_NUM(NETSEC_REG_NETSEC_VER_F_TAIKI)) {
    ret = -ENODEV;
    goto pm_disable;
    }
    dev_info(&pdev.dev, "hardware revision %d.%d\n",
    hw_ver >> 16, hw_ver & 0xffff);
    netif_napi_add(ndev, &priv.napi, netsec_napi_poll);
    ndev.netdev_ops = &netsec_netdev_ops;
    ndev.ethtool_ops = &netsec_ethtool_ops;
    ndev.features |= NETIF_F_HIGHDMA | NETIF_F_RXCSUM | NETIF_F_GSO |
    NETIF_F_IP_CSUM | NETIF_F_IPV6_CSUM;
    ndev.hw_features = ndev.features;
    ndev.xdp_features = NETDEV_XDP_ACT_BASIC | NETDEV_XDP_ACT_REDIRECT |
    NETDEV_XDP_ACT_NDO_XMIT;
    priv.rx_cksum_offload_flag = true;
    ret = netsec_register_mdio(priv, phy_addr);
    if (ret)
    goto unreg_napi;
    if (dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(40)))
    dev_warn(&pdev.dev, "Failed to set DMA mask\n");
    ret = register_netdev(ndev);
    if (ret) {
    netif_err(priv, probe, ndev, "register_netdev() failed\n");
    goto unreg_mii;
    }
    pm_runtime_put_sync(&pdev.dev);
    return 0;
    unreg_mii:
    netsec_unregister_mdio(priv);
    unreg_napi:
    netif_napi_del(&priv.napi);
    pm_disable:
    pm_runtime_put_sync(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    free_ndev:
    free_netdev(ndev);
    dev_err(&pdev.dev, "init failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn netsec_remove(pdev: *mut platform_device) {
    static void netsec_remove(struct platform_device *pdev)
    {
    struct netsec_priv *priv = platform_get_drvdata(pdev);
    unregister_netdev(priv.ndev);
    netsec_unregister_mdio(priv);
    netif_napi_del(&priv.napi);
    pm_runtime_disable(&pdev.dev);
    free_netdev(priv.ndev);
    }

#[no_mangle]
unsafe extern "C" fn netsec_runtime_suspend(dev: *mut device) -> c_int {
    static int netsec_runtime_suspend(struct device *dev)
    {
    struct netsec_priv *priv = dev_get_drvdata(dev);
    netsec_write(priv, NETSEC_REG_CLK_EN, 0);
    clk_disable_unprepare(priv.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn netsec_runtime_resume(dev: *mut device) -> c_int {
    static int netsec_runtime_resume(struct device *dev)
    {
    struct netsec_priv *priv = dev_get_drvdata(dev);
    clk_prepare_enable(priv.clk);
    netsec_write(priv, NETSEC_REG_CLK_EN, NETSEC_CLK_EN_REG_DOM_D |
    NETSEC_CLK_EN_REG_DOM_C |
    NETSEC_CLK_EN_REG_DOM_G);
    return 0;
    }

    static const struct dev_pm_ops netsec_pm_ops = {
    SET_RUNTIME_PM_OPS(netsec_runtime_suspend, netsec_runtime_resume, core::ptr::null_mut())
    };
    static const struct of_device_id netsec_dt_ids[] = {
    { .compatible = "socionext,synquacer-netsec" },
    { }
    };
    MODULE_DEVICE_TABLE(of, netsec_dt_ids);

    static const struct acpi_device_id netsec_acpi_ids[] = {
    { "SCX0001" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, netsec_acpi_ids);

    static struct platform_driver netsec_driver = {
    .probe	= netsec_probe,
    .remove = netsec_remove,
    .driver = {
    .name = "netsec",
    .pm = &netsec_pm_ops,
    .of_match_table = netsec_dt_ids,
    .acpi_match_table = ACPI_PTR(netsec_acpi_ids),
    },
    };
    module_platform_driver(netsec_driver);
    MODULE_AUTHOR("Jassi Brar <jaswinder.singh@linaro.org>");
    MODULE_AUTHOR("Ard Biesheuvel <ard.biesheuvel@linaro.org>");
    MODULE_DESCRIPTION("NETSEC Ethernet driver");
    MODULE_LICENSE("GPL");

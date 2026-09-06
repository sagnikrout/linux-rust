//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/mt7530.h
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
// Copyright (C) 2017 Sean Wang <sean.wang@mediatek.com>
//
pub const MT7530_NUM_PORTS: c_int = 7;
pub const MT7530_NUM_PHYS: c_int = 5;
pub const MT7530_NUM_FDB_RECORDS: c_int = 2048;
pub const MT7530_ALL_MEMBERS: c_uint = 0xff;
pub const MTK_HDR_LEN: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt753x_id {
    ID_MT7530 = 0,
    ID_MT7621 = 1,
    ID_MT7531 = 2,
    ID_MT7988 = 3,
    ID_EN7581 = 4,
    ID_AN7583 = 5,
    ID_EN7528 = 6,
}

pub const NUM_TRGMII_CTRL: c_int = 5;

// Registers to ethsys access
pub const ETHSYS_CLKCFG0: c_uint = 0x2c;

pub const SYSC_REG_RSTCTRL: c_uint = 0x34;

// Register for ARL global control
pub const MT753X_AGC: c_uint = 0xc;

// Register for MAC forward control
pub const MT753X_MFC: c_uint = 0x10;

// Register for CPU forward control
pub const MT7531_CFC: c_uint = 0x4;

// Register for BPDU and PAE frame control
pub const MT753X_BPC: c_uint = 0x24;

// Register for 01-80-C2-00-00-[01,02] MAC DA frame control
pub const MT753X_RGAC1: c_uint = 0x28;

// Register for 01-80-C2-00-00-[03,0E] MAC DA frame control
pub const MT753X_RGAC2: c_uint = 0x2c;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt753x_to_cpu_fw {
    TO_CPU_FW_SYSTEM_DEFAULT,
    TO_CPU_FW_CPU_EXCLUDE = 4,
    TO_CPU_FW_CPU_INCLUDE = 5,
    TO_CPU_FW_CPU_ONLY = 6,
    TO_CPU_FW_DROP = 7,
}

// Registers for address table access
pub const MT7530_ATA1: c_uint = 0x74;
pub const STATIC_EMP: c_int = 0;
pub const STATIC_ENT: c_int = 3;
pub const MT7530_ATA2: c_uint = 0x78;

// Register for address table write data
pub const MT7530_ATWD: c_uint = 0x7c;
// Register for address table control
pub const MT7530_ATC: c_uint = 0x80;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7530_fdb_cmd {
    MT7530_FDB_READ	= 0,
    MT7530_FDB_WRITE = 1,
    MT7530_FDB_FLUSH = 2,
    MT7530_FDB_START = 4,
    MT7530_FDB_NEXT = 5,
}

// Registers for table search read address
pub const MT7530_TSRA1: c_uint = 0x84;
pub const MAC_BYTE_0: c_int = 24;
pub const MAC_BYTE_1: c_int = 16;
pub const MAC_BYTE_2: c_int = 8;
pub const MAC_BYTE_3: c_int = 0;
pub const MAC_BYTE_MASK: c_uint = 0xff;
pub const MT7530_TSRA2: c_uint = 0x88;
pub const MAC_BYTE_4: c_int = 24;
pub const MAC_BYTE_5: c_int = 16;
pub const CVID: c_int = 0;
pub const CVID_MASK: c_uint = 0xfff;
pub const MT7530_ATRD: c_uint = 0x8C;
pub const AGE_TIMER: c_int = 24;
pub const AGE_TIMER_MASK: c_uint = 0xff;
pub const PORT_MAP: c_int = 4;
pub const PORT_MAP_MASK: c_uint = 0xff;
pub const ENT_STATUS: c_int = 2;
pub const ENT_STATUS_MASK: c_uint = 0x3;
// Register for vlan table control
pub const MT7530_VTCR: c_uint = 0x90;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7530_vlan_cmd {
// Read/Write the specified VID entry from VAWD register based
// on VID.
//
    MT7530_VTCR_RD_VID = 0,
    MT7530_VTCR_WR_VID = 1,
}

// Register for setup vlan and acl write data
pub const MT7530_VAWD1: c_uint = 0x94;

// Independent VLAN Learning

// Egress Tag Consistent

// Per VLAN Egress Tag Control

// VLAN Member Control

// Filter ID

// VLAN Entry Valid

pub const PORT_MEM_SHFT: c_int = 16;
pub const PORT_MEM_MASK: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7530_fid {
    FID_STANDALONE = 0,
    FID_BRIDGED = 1,
}

pub const MT7530_VAWD2: c_uint = 0x98;
// Egress Tag Control

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7530_vlan_egress_attr {
    MT7530_VLAN_EGRESS_UNTAG = 0,
    MT7530_VLAN_EGRESS_TAG = 2,
    MT7530_VLAN_EGRESS_STACK = 3,
}

// Register for address age control
pub const MT7530_AAC: c_uint = 0xa0;
// Disable ageing

// Age count

pub const AGE_CNT_MAX: c_uint = 0xff;

// Age unit

pub const AGE_UNIT_MAX: c_uint = 0xfff;

pub const MT753X_GERLCR: c_uint = 0x10e0;

pub const EGR_BC_CRC: c_uint = 0x4	/* crc */;
pub const EGR_BC_CRC_IPG_PREAMBLE: c_uint = 0x18	/* crc + ipg + preamble */;
// Register for port STP state control

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7530_stp_state {
    MT7530_STP_DISABLED = 0,
    MT7530_STP_BLOCKING = 1,
    MT7530_STP_LISTENING = 1,
    MT7530_STP_LEARNING = 2,
    MT7530_STP_FORWARDING  = 3
}

// Register for port control

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7530_port_mode {
// Port Matrix Mode: Frames are forwarded by the PCR_MATRIX members.
    MT7530_PORT_MATRIX_MODE = PORT_VLAN(0),

// Fallback Mode: Forward received frames with ingress ports that do
// not belong to the VLAN member. Frames whose VID is not listed on
// the VLAN table are forwarded by the PCR_MATRIX members.
//
    MT7530_PORT_FALLBACK_MODE = PORT_VLAN(1),

// Security Mode: Discard any frame due to ingress membership
// violation or VID missed on the VLAN table.
//
    MT7530_PORT_SECURITY_MODE = PORT_VLAN(3),
}

// Register for port security control

// Register for port vlan control

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7530_vlan_port_eg_tag {
    MT7530_VLAN_EG_DISABLED = 0,
    MT7530_VLAN_EG_CONSISTENT = 1,
    MT7530_VLAN_EG_UNTAGGED = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7530_vlan_port_attr {
    MT7530_VLAN_USER = 0,
    MT7530_VLAN_TRANSPARENT = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7530_vlan_port_acc_frm {
    MT7530_VLAN_ACC_ALL = 0,
    MT7530_VLAN_ACC_TAGGED = 1,
    MT7530_VLAN_ACC_UNTAGGED = 2,
}

// Register for port port-and-protocol based vlan 1 control

// Register for port MAC control register

pub const PMSR_SPEED_10: c_uint = 0x00;

// Register for port debug count

pub const MT7530_GMACCR: c_uint = 0x30e0;

pub const MAX_RX_PKT_LEN_1522: c_uint = 0x0;
pub const MAX_RX_PKT_LEN_1536: c_uint = 0x1;
pub const MAX_RX_PKT_LEN_1552: c_uint = 0x2;
pub const MAX_RX_PKT_LEN_JUMBO: c_uint = 0x3;
// Register for MIB

// Each define is an offset of MT7530_PORT_MIB_COUNTER
pub const MT7530_PORT_MIB_TX_DROP: c_uint = 0x00;
pub const MT7530_PORT_MIB_TX_CRC_ERR: c_uint = 0x04;
pub const MT7530_PORT_MIB_TX_UNICAST: c_uint = 0x08;
pub const MT7530_PORT_MIB_TX_MULTICAST: c_uint = 0x0c;
pub const MT7530_PORT_MIB_TX_BROADCAST: c_uint = 0x10;
pub const MT7530_PORT_MIB_TX_COLLISION: c_uint = 0x14;
pub const MT7530_PORT_MIB_TX_SINGLE_COLLISION: c_uint = 0x18;
pub const MT7530_PORT_MIB_TX_MULTIPLE_COLLISION: c_uint = 0x1c;
pub const MT7530_PORT_MIB_TX_DEFERRED: c_uint = 0x20;
pub const MT7530_PORT_MIB_TX_LATE_COLLISION: c_uint = 0x24;
pub const MT7530_PORT_MIB_TX_EXCESSIVE_COLLISION: c_uint = 0x28;
pub const MT7530_PORT_MIB_TX_PAUSE: c_uint = 0x2c;
pub const MT7530_PORT_MIB_TX_PKT_SZ_64: c_uint = 0x30;
pub const MT7530_PORT_MIB_TX_PKT_SZ_65_TO_127: c_uint = 0x34;
pub const MT7530_PORT_MIB_TX_PKT_SZ_128_TO_255: c_uint = 0x38;
pub const MT7530_PORT_MIB_TX_PKT_SZ_256_TO_511: c_uint = 0x3c;
pub const MT7530_PORT_MIB_TX_PKT_SZ_512_TO_1023: c_uint = 0x40;
pub const MT7530_PORT_MIB_TX_PKT_SZ_1024_TO_MAX: c_uint = 0x44;
pub const MT7530_PORT_MIB_TX_BYTES: c_uint = 0x48 /* 64 bytes */;
pub const MT7530_PORT_MIB_RX_DROP: c_uint = 0x60;
pub const MT7530_PORT_MIB_RX_FILTERING: c_uint = 0x64;
pub const MT7530_PORT_MIB_RX_UNICAST: c_uint = 0x68;
pub const MT7530_PORT_MIB_RX_MULTICAST: c_uint = 0x6c;
pub const MT7530_PORT_MIB_RX_BROADCAST: c_uint = 0x70;
pub const MT7530_PORT_MIB_RX_ALIGN_ERR: c_uint = 0x74;
pub const MT7530_PORT_MIB_RX_CRC_ERR: c_uint = 0x78;
pub const MT7530_PORT_MIB_RX_UNDER_SIZE_ERR: c_uint = 0x7c;
pub const MT7530_PORT_MIB_RX_FRAG_ERR: c_uint = 0x80;
pub const MT7530_PORT_MIB_RX_OVER_SZ_ERR: c_uint = 0x84;
pub const MT7530_PORT_MIB_RX_JABBER_ERR: c_uint = 0x88;
pub const MT7530_PORT_MIB_RX_PAUSE: c_uint = 0x8c;
pub const MT7530_PORT_MIB_RX_PKT_SZ_64: c_uint = 0x90;
pub const MT7530_PORT_MIB_RX_PKT_SZ_65_TO_127: c_uint = 0x94;
pub const MT7530_PORT_MIB_RX_PKT_SZ_128_TO_255: c_uint = 0x98;
pub const MT7530_PORT_MIB_RX_PKT_SZ_256_TO_511: c_uint = 0x9c;
pub const MT7530_PORT_MIB_RX_PKT_SZ_512_TO_1023: c_uint = 0xa0;
pub const MT7530_PORT_MIB_RX_PKT_SZ_1024_TO_MAX: c_uint = 0xa4;
pub const MT7530_PORT_MIB_RX_BYTES: c_uint = 0xa8 /* 64 bytes */;
pub const MT7530_PORT_MIB_RX_CTRL_DROP: c_uint = 0xb0;
pub const MT7530_PORT_MIB_RX_INGRESS_DROP: c_uint = 0xb4;
pub const MT7530_PORT_MIB_RX_ARL_DROP: c_uint = 0xb8;
pub const MT7530_MIB_CCR: c_uint = 0x4fe0;

// MT7531 SGMII register group

pub const MT7531_PHYA_CTRL_SIGNAL3: c_uint = 0x128;
// Register for system reset
pub const MT7530_SYS_CTRL: c_uint = 0x7000;

// Register for system interrupt
pub const MT7530_SYS_INT_EN: c_uint = 0x7008;
// Register for system interrupt status
pub const MT7530_SYS_INT_STS: c_uint = 0x700c;
// Register for PHY Indirect Access Control
pub const MT7531_PHY_IAC: c_uint = 0x701C;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7531_phy_iac_cmd {
    MT7531_MDIO_ADDR = 0,
    MT7531_MDIO_WRITE = 1,
    MT7531_MDIO_READ = 2,
    MT7531_MDIO_READ_CL45 = 3,
}

// MDIO_ST: MDIO start field
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7531_mdio_st {
    MT7531_MDIO_ST_CL45 = 0,
    MT7531_MDIO_ST_CL22 = 1,
}

// Register for RGMII clock phase
pub const MT7531_CLKGEN_CTRL: c_uint = 0x7500;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7531_gp_mode {
    MT7531_GP_MODE_RGMII = 0,
    MT7531_GP_MODE_MII = 1,
    MT7531_GP_MODE_REV_MII = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7531_clk_skew {
    MT7531_CLK_SKEW_NO_CHG = 0,
    MT7531_CLK_SKEW_DLY_100PPS = 1,
    MT7531_CLK_SKEW_DLY_200PPS = 2,
    MT7531_CLK_SKEW_REVERSE = 3,
}

// Register for trap status
pub const MT753X_TRAP: c_uint = 0x7800;

// Register for trap modification
pub const MT753X_MTRAP: c_uint = 0x7804;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7531_xtal_fsel {
    MT7531_XTAL_FSEL_25MHZ,
    MT7531_XTAL_FSEL_40MHZ,
}

// Register for TOP signal control
pub const MT7530_TOP_SIG_CTRL: c_uint = 0x7808;

pub const MT7531_TOP_SIG_SR: c_uint = 0x780c;

pub const MT7530_IO_DRV_CR: c_uint = 0x7810;

pub const MT7531_CHIP_REV: c_uint = 0x781C;
pub const MT7531_PLLGP_EN: c_uint = 0x7820;

pub const MT7530_P6ECR: c_uint = 0x7830;
pub const P6_INTF_MODE_MASK: c_uint = 0x3;

pub const MT7531_PLLGP_CR0: c_uint = 0x78a8;

pub const RG_COREPLL_POSDIV_S: c_int = 23;
pub const RG_COREPLL_POSDIV_M: c_uint = 0x3800000;
pub const RG_COREPLL_SDM_PCW_S: c_int = 1;
pub const RG_COREPLL_SDM_PCW_M: c_uint = 0x3ffffe;

// Registers for RGMII and SGMII PLL clock
pub const MT7531_ANA_PLLGP_CR2: c_uint = 0x78b0;
pub const MT7531_ANA_PLLGP_CR5: c_uint = 0x78bc;
// Registers for TRGMII on the both side
pub const MT7530_TRGMII_RCK_CTRL: c_uint = 0x7a00;

pub const DQSI0_TAP_MASK: c_uint = 0x7f;

pub const MT7530_TRGMII_RCK_RTT: c_uint = 0x7a04;

pub const RD_TAP_MASK: c_uint = 0x7f;

pub const MT7530_TRGMII_TXCTRL: c_uint = 0x7a40;

pub const MT7530_TRGMII_TCK_CTRL: c_uint = 0x7a78;

pub const MT7530_P5RGMIIRXCR: c_uint = 0x7b00;

pub const MT7530_P5RGMIITXCR: c_uint = 0x7b04;

// Registers for GPIO mode
pub const MT7531_GPIO_MODE0: c_uint = 0x7c0c;

pub const MT7531_GPIO0_INTERRUPT: c_int = 1;
pub const MT7531_GPIO_MODE1: c_uint = 0x7c10;

pub const MT753X_CPORT_SPTAG_CFG: c_uint = 0x7c10;

pub const AN7583_GEPHY_CONN_CFG: c_uint = 0x7c14;

// Registers for LED GPIO control (MT7530 only)
// All registers follow this pattern:
// [ 2: 0]  port 0
// [ 6: 4]  port 1
// [10: 8]  port 2
// [14:12]  port 3
// [18:16]  port 4
//
// LED enable, 0: Disable, 1: Enable (Default)
pub const MT7530_LED_EN: c_uint = 0x7d00;
// LED mode, 0: GPIO mode, 1: PHY mode (Default)
pub const MT7530_LED_IO_MODE: c_uint = 0x7d04;
// GPIO direction, 0: Input, 1: Output
pub const MT7530_LED_GPIO_DIR: c_uint = 0x7d10;
// GPIO output enable, 0: Disable, 1: Enable
pub const MT7530_LED_GPIO_OE: c_uint = 0x7d14;
// GPIO value, 0: Low, 1: High
pub const MT7530_LED_GPIO_DATA: c_uint = 0x7d18;
pub const MT7530_CREV: c_uint = 0x7ffc;
pub const CHIP_NAME_SHIFT: c_int = 16;
pub const MT7530_ID: c_uint = 0x7530;
pub const MT7531_CREV: c_uint = 0x781C;
pub const CHIP_REV_M: c_uint = 0x0f;
pub const MT7531_ID: c_uint = 0x7531;
// Registers for core PLL access through mmd indirect
pub const CORE_PLL_GROUP2: c_uint = 0x401;

pub const CORE_PLL_GROUP4: c_uint = 0x403;

pub const CORE_PLL_GROUP5: c_uint = 0x404;

pub const CORE_PLL_GROUP6: c_uint = 0x405;

pub const CORE_PLL_GROUP7: c_uint = 0x406;

pub const CORE_PLL_GROUP10: c_uint = 0x409;

pub const CORE_PLL_GROUP11: c_uint = 0x40a;

pub const CORE_GSWPLL_GRP1: c_uint = 0x40d;

pub const CORE_GSWPLL_GRP2: c_uint = 0x40e;

pub const CORE_TRGMII_GSW_CLK_CG: c_uint = 0x410;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7530_mib_desc {
    pub size: c_uint,
    pub offset: c_uint,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7530_fdb {
    pub vid: u16,
    pub port_mask: u8,
    pub aging: u8,
    pub mac: [u8; 6],
    pub noarp: bool,
}

// struct mt7530_port -	This is the main data structure for holding the state
// of the port.
// @enable:	The status used for show port is enabled or not.
// @pm:		The matrix used to show all connections with the port.
// @pvid:	The VLAN specified is to be considered a PVID at ingress.  Any
// untagged frames will be assigned to the related VLAN.
// @sgmii_pcs:	Pointer to PCS instance for SerDes ports
// @stats:	Cached port statistics for MDIO-connected switches
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7530_port {
    pub enable: bool,
    pub isolated: bool,
    pub pm: u32,
    pub pvid: u16,
    pub sgmii_pcs: *mut phylink_pcs,
    pub stats: rtnl_link_stats64,
}

// Port 5 mode definitions of the MT7530 switch
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7530_p5_mode {
    GMAC5,
    MUX_PHY_P0,
    MUX_PHY_P4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt753x_pcs {
    pub pcs: phylink_pcs,
    pub priv: *mut mt7530_priv,
    pub port: c_int,
}

// struct mt753x_info -	This is the main data structure for holding the specific
// part for each supported device
// @id:			Holding the identifier to a switch model
// @pcs_ops:		Holding the pointer to the MAC PCS operations structure
// @sw_setup:		Holding the handler to a device initialization
// @phy_read_c22:	Holding the way reading PHY port using C22
// @phy_write_c22:	Holding the way writing PHY port using C22
// @phy_read_c45:	Holding the way reading PHY port using C45
// @phy_write_c45:	Holding the way writing PHY port using C45
// @mac_port_get_caps:	Holding the handler that provides MAC capabilities
// @mac_port_config:	Holding the way setting up the PHY attribute to a
// certain MAC port
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt753x_info {
    pub id: mt753x_id,
    pub pcs_ops: *const phylink_pcs_ops,
    pub ds): *mut *mut int (sw_setup)(struct dsa_switch,
    pub regnum): *mut *mut *mut int (phy_read_c22)(struct mt7530_priv priv, int port, int,
    pub val): u16,
    pub regnum): c_int,
    pub val): int regnum, u16,
    pub config): *mut phylink_config,
    pub interface): phy_interface_t,
}

// struct mt7530_priv -	This is the main data structure for holding the state
// of the driver
// @dev:		The device pointer
// @ds:			The pointer to the dsa core structure
// @bus:		The bus used for the device and built-in PHY
// @regmap:		The regmap instance representing all switch registers
// @rstc:		The pointer to reset control used by MCM
// @core_pwr:		The power supplied into the core
// @io_pwr:		The power supplied into the I/O
// @reset:		The descriptor for GPIO line tied to its reset pin
// @mcm:		Flag for distinguishing if standalone IC or module
// coupling
// @ports:		Holding the state among ports
// @reg_mutex:		The lock for protecting among process accessing
// registers
// @p5_mode:		Holding the current mode of port 5 of the MT7530 switch
// @p5_sgmii:		Flag for distinguishing if port 5 of the MT7531 switch
// has got SGMII
// @irq_domain:		IRQ domain of the switch irq_chip
// @create_sgmii:	Pointer to function creating SGMII PCS instance(s)
// @active_cpu_ports:	Holding the active CPU ports
// @mdiodev:		The pointer to the MDIO device structure
// @stats_lock:		Protects cached per-port stats from concurrent access
// @stats_work:		Delayed work for polling MIB counters on MDIO switches
// @stats_last:		Jiffies timestamp of last MIB counter poll
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7530_priv {
    pub dev: *mut device,
    pub ds: *mut dsa_switch,
    pub bus: *mut mii_bus,
    pub regmap: *mut regmap,
    pub rstc: *mut reset_control,
    pub core_pwr: *mut regulator,
    pub io_pwr: *mut regulator,
    pub reset: *mut gpio_desc,
    pub info: *const mt753x_info,
    pub id: c_uint,
    pub mcm: bool,
    pub p5_mode: mt7530_p5_mode,
    pub p5_sgmii: bool,
    pub mirror_rx: u8,
    pub mirror_tx: u8,
    pub ports: [mt7530_port; MT7530_NUM_PORTS],
    pub pcs: [mt753x_pcs; MT7530_NUM_PORTS],
// protect among processes for registers access
    pub reg_mutex: mutex,
    pub irq_domain: *mut irq_domain,
    pub priv): *mut *mut int (create_sgmii)(struct mt7530_priv,
    pub active_cpu_ports: u8,
    pub mdiodev: *mut mdio_device,
    pub /: *mut *mut spinlock_t stats_lock; / protects cached stats counters,
    pub stats_work: delayed_work,
    pub stats_last: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7530_hw_vlan_entry {
    pub port: c_int,
    pub old_members: u8,
    pub untagged: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7530_hw_stats {
    pub string: *const c_char,
    pub reg: u16,
    pub sizeof_stat: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7530_dummy_poll {
    pub priv: *mut mt7530_priv,
    pub reg: u32,
}

extern "C" {
    pub fn mt7530_probe_common(priv: *mut mt7530_priv) -> c_int;
}
extern "C" {
    pub fn mt7530_remove_common(priv: *mut mt7530_priv);
}

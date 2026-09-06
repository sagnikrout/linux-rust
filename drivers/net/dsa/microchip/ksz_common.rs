//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/microchip/ksz_common.h
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
// Microchip switch driver common header
//
// Copyright (C) 2017-2025 Microchip Technology Inc.
//

pub const KSZ_MAX_NUM_PORTS: c_int = 8;
// all KSZ switches count ports from 1
pub const KSZ_PORT_1: c_int = 0;
pub const KSZ_PORT_2: c_int = 1;
pub const KSZ_PORT_4: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ksz_regmap_width {
    KSZ_REGMAP_8,
    KSZ_REGMAP_16,
    KSZ_REGMAP_32,
    __KSZ_NUM_REGMAPS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_table {
    pub table: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_stats_raw {
    pub rx_hi: u64,
    pub rx_undersize: u64,
    pub rx_fragments: u64,
    pub rx_oversize: u64,
    pub rx_jabbers: u64,
    pub rx_symbol_err: u64,
    pub rx_crc_err: u64,
    pub rx_align_err: u64,
    pub rx_mac_ctrl: u64,
    pub rx_pause: u64,
    pub rx_bcast: u64,
    pub rx_mcast: u64,
    pub rx_ucast: u64,
    pub rx_64_or_less: u64,
    pub rx_65_127: u64,
    pub rx_128_255: u64,
    pub rx_256_511: u64,
    pub rx_512_1023: u64,
    pub rx_1024_1522: u64,
    pub rx_1523_2000: u64,
    pub rx_2001: u64,
    pub tx_hi: u64,
    pub tx_late_col: u64,
    pub tx_pause: u64,
    pub tx_bcast: u64,
    pub tx_mcast: u64,
    pub tx_ucast: u64,
    pub tx_deferred: u64,
    pub tx_total_col: u64,
    pub tx_exc_col: u64,
    pub tx_single_col: u64,
    pub tx_mult_col: u64,
    pub rx_total: u64,
    pub tx_total: u64,
    pub rx_discards: u64,
    pub tx_discards: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_port_mib {
    pub /: *mut *mut mutex cnt_mutex; / structure access,
    pub cnt_ptr: u8,
    pub counters: *mut u64,
    pub stats64: rtnl_link_stats64,
    pub pause_stats: ethtool_pause_stats,
    pub stats64_lock: spinlock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_mib_names {
    pub index: c_int,
    pub string: [c_char; ETH_GSTRING_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_chip_data {
    pub chip_id: u32,
    pub dev_name: *const c_char,
    pub num_vlans: c_int,
    pub num_alus: c_int,
    pub num_statics: c_int,
    pub cpu_ports: c_int,
    pub port_cnt: c_int,
    pub port_nirqs: u8,
    pub num_tx_queues: u8,
    pub /: *mut *mut u8 num_ipms; / number of Internal Priority Maps,
    pub tc_cbs_supported: bool,
//
// @phy_side_mdio_supported: Indicates if the chip supports an additional
// side MDIO channel for accessing integrated PHYs.
//
    pub phy_side_mdio_supported: bool,
    pub ops: *const ksz_dev_ops,
    pub switch_ops: *const dsa_switch_ops,
    pub phylink_mac_ops: *const phylink_mac_ops,
    pub phy_errata_9477: bool,
    pub ksz87xx_eee_link_erratum: bool,
    pub mib_names: *const ksz_mib_names,
    pub mib_cnt: c_int,
    pub reg_mib_cnt: u8,
    pub regs: *const u16,
    pub masks: *const u32,
    pub shifts: *const u8,
    pub xmii_ctrl0: *const u8,
    pub xmii_ctrl1: *const u8,
    pub stp_ctrl_reg: c_int,
    pub broadcast_ctrl_reg: c_int,
    pub multicast_ctrl_reg: c_int,
    pub start_ctrl_reg: c_int,
    pub supports_mii: [bool; KSZ_MAX_NUM_PORTS],
    pub supports_rmii: [bool; KSZ_MAX_NUM_PORTS],
    pub supports_rgmii: [bool; KSZ_MAX_NUM_PORTS],
    pub internal_phy: [bool; KSZ_MAX_NUM_PORTS],
    pub gbit_capable: [bool; KSZ_MAX_NUM_PORTS],
    pub ptp_capable: bool,
    pub sgmii_port: u8,
    pub wr_table: *const regmap_access_table,
    pub rd_table: *const regmap_access_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_irq {
    pub masked: u16,
    pub reg_mask: u16,
    pub reg_status: u16,
    pub domain: *mut irq_domain,
    pub nirqs: c_int,
    pub irq_num: c_int,
    pub name: [c_char; 16],
    pub dev: *mut ksz_device,
    pub irq0_offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_ptp_irq {
    pub port: *mut ksz_port,
    pub ts_reg: u16,
    pub ts_en: bool,
    pub name: [c_char; 16],
    pub num: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_switch_macaddr {
    pub addr: [c_uchar; ETH_ALEN],
    pub refcount: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_port {
    pub /: *mut *mut bool remove_tag; / Remove Tag flag set, for ksz8795 only,
    pub learning: bool,
    pub isolated: bool,
    pub stp_state: c_int,
    pub speed: c_int,
    pub duplex: c_int,
    pub link: bool,
    pub /: *mut *mut u32 fiber:1; / port is fiber,
    pub force:1: u32,
    pub /: *mut *mut u32 read:1; / read MIB counters in background,
    pub /: *mut *mut u32 freeze:1; / MIB counter freeze is enabled,
    pub sgmii_adv_write:1: u32,
    pub mib: ksz_port_mib,
    pub interface: phy_interface_t,
    pub rgmii_tx_val: u32,
    pub rgmii_rx_val: u32,
    pub ksz_dev: *mut ksz_device,
    pub acl_priv: *mut c_void,
    pub pirq: ksz_irq,
    pub num: u8,
    pub pcs: *mut phylink_pcs,

    pub tstamp_config: kernel_hwtstamp_config,
    pub hwts_tx_en: bool,
    pub hwts_rx_en: bool,
    pub last_tx_is_pdelayresp: bool,
    pub ptpirq: ksz_irq,
    pub ptpmsg_irq: [ksz_ptp_irq; 3],
    pub tstamp_msg: ktime_t,
    pub tstamp_msg_comp: completion,

    pub manual_flow: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_device {
    pub ds: *mut dsa_switch,
    pub pdata: *mut ksz_platform_data,
    pub info: *const ksz_chip_data,
    pub /: *mut *mut mutex dev_mutex; / device access,
    pub /: *mut *mut mutex regmap_mutex; / regmap access,
    pub /: *mut *mut mutex alu_mutex; / ALU access,
    pub /: *mut *mut mutex vlan_mutex; / vlan access,
    pub dev_ops: *const ksz_dev_ops,
    pub dev: *mut device,
    pub regmap: [*mut regmap; __KSZ_NUM_REGMAPS],
    pub priv: *mut c_void,
    pub irq: c_int,
    pub /: *mut *mut *mut gpio_desc reset_gpio; / Optional reset GPIO,
// chip specific data
    pub chip_id: u32,
    pub chip_rev: u8,
    pub /: *mut *mut int cpu_port; / port connected to CPU,
    pub phy_port_cnt: c_int,
    pub compat_interface: phy_interface_t,
    pub synclko_125: bool,
    pub synclko_disable: bool,
    pub wakeup_source: bool,
    pub pme_active_high: bool,
    pub vlan_cache: *mut vlan_table,
    pub ports: *mut ksz_port,
    pub mib_read: delayed_work,
    pub mib_read_interval: c_ulong,
    pub mirror_rx: u16,
    pub mirror_tx: u16,
    pub port_mask: u16,
    pub /: *mut *mut mutex lock_irq; / IRQ Access,
    pub girq: ksz_irq,
    pub ptp_data: ksz_ptp_data,
    pub switch_macaddr: *mut ksz_switch_macaddr,
    pub /: *mut *mut *mut net_device hsr_dev; / HSR,
    pub hsr_ports: u8,
//
// @phy_addr_map: Array mapping switch ports to their corresponding PHY
// addresses.
//
    pub phy_addr_map: [u8; KSZ_MAX_NUM_PORTS],
//
// @parent_mdio_bus: Pointer to the external MDIO bus controller.
//
// This points to an external MDIO bus controller that is used to access
// the  PHYs integrated within the switch. Unlike an integrated MDIO
// bus, this external controller provides a direct path for managing
// the switch’s internal PHYs, bypassing the main SPI interface.
//
    pub parent_mdio_bus: *mut mii_bus,
// KSZ87xx low-loss tuning state
    pub /: *mut *mut *mut u8 lpf_bw; / KSZ87XX_PHY_LPF_,
    pub /: *mut *mut u8 eq_init; / DSP EQ initial value,
}

// List of supported models
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ksz_model {
    KSZ8463,
    KSZ8563,
    KSZ8567,
    KSZ8795,
    KSZ8794,
    KSZ8765,
    KSZ88X3,
    KSZ8864,
    KSZ8895,
    KSZ9477,
    KSZ9896,
    KSZ9897,
    KSZ9893,
    KSZ9563,
    KSZ9567,
    LAN9370,
    LAN9371,
    LAN9372,
    LAN9373,
    LAN9374,
    LAN9646,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ksz_regs {
    REG_SW_MAC_ADDR,
    REG_IND_CTRL_0,
    REG_IND_DATA_8,
    REG_IND_DATA_CHECK,
    REG_IND_DATA_HI,
    REG_IND_DATA_LO,
    REG_IND_MIB_CHECK,
    REG_IND_BYTE,
    P_FORCE_CTRL,
    P_LINK_STATUS,
    P_LOCAL_CTRL,
    P_NEG_RESTART_CTRL,
    P_REMOTE_STATUS,
    P_SPEED_STATUS,
    S_TAIL_TAG_CTRL,
    P_STP_CTRL,
    S_START_CTRL,
    S_BROADCAST_CTRL,
    S_MULTICAST_CTRL,
    P_XMII_CTRL_0,
    P_XMII_CTRL_1,
    REG_SW_PME_CTRL,
    REG_PORT_PME_STATUS,
    REG_PORT_PME_CTRL,
    PTP_CLK_CTRL,
    PTP_RTC_NANOSEC,
    PTP_RTC_SEC,
    PTP_RTC_SUB_NANOSEC,
    PTP_SUBNANOSEC_RATE,
    PTP_MSG_CONF1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ksz_masks {
    PORT_802_1P_REMAPPING,
    SW_TAIL_TAG_ENABLE,
    MIB_COUNTER_OVERFLOW,
    MIB_COUNTER_VALID,
    VLAN_TABLE_FID,
    VLAN_TABLE_MEMBERSHIP,
    VLAN_TABLE_VALID,
    STATIC_MAC_TABLE_VALID,
    STATIC_MAC_TABLE_USE_FID,
    STATIC_MAC_TABLE_FID,
    STATIC_MAC_TABLE_OVERRIDE,
    STATIC_MAC_TABLE_FWD_PORTS,
    DYNAMIC_MAC_TABLE_ENTRIES_H,
    DYNAMIC_MAC_TABLE_MAC_EMPTY,
    DYNAMIC_MAC_TABLE_NOT_READY,
    DYNAMIC_MAC_TABLE_ENTRIES,
    DYNAMIC_MAC_TABLE_FID,
    DYNAMIC_MAC_TABLE_SRC_PORT,
    DYNAMIC_MAC_TABLE_TIMESTAMP,
    ALU_STAT_WRITE,
    ALU_STAT_READ,
    ALU_STAT_DIRECT,
    ALU_RESV_MCAST_ADDR,
    P_MII_TX_FLOW_CTRL,
    P_MII_RX_FLOW_CTRL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ksz_shifts {
    VLAN_TABLE_MEMBERSHIP_S,
    VLAN_TABLE,
    STATIC_MAC_FWD_PORTS,
    STATIC_MAC_FID,
    DYNAMIC_MAC_ENTRIES_H,
    DYNAMIC_MAC_ENTRIES,
    DYNAMIC_MAC_FID,
    DYNAMIC_MAC_TIMESTAMP,
    DYNAMIC_MAC_SRC_PORT,
    ALU_STAT_INDEX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ksz_xmii_ctrl0 {
    P_MII_100MBIT,
    P_MII_10MBIT,
    P_MII_FULL_DUPLEX,
    P_MII_HALF_DUPLEX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ksz_xmii_ctrl1 {
    P_RGMII_SEL,
    P_RMII_SEL,
    P_GMII_SEL,
    P_MII_SEL,
    P_GMII_1GBIT,
    P_GMII_NOT_1GBIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alu_struct {
// entry 1
    pub is_static:1: u8,
    pub is_src_filter:1: u8,
    pub is_dst_filter:1: u8,
    pub prio_age:3: u8,
    pub _reserv_0_1:23: u32,
    pub mstp:3: u8,
// entry 2
    pub is_override:1: u8,
    pub is_use_fid:1: u8,
    pub _reserv_1_1:23: u32,
    pub port_forward:7: u8,
// entry 3 & 4
    pub _reserv_2_1:9: u32,
    pub fid:7: u8,
    pub mac: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_dev_ops {
    pub offset): *mut *mut u32 (get_port_addr)(int port, int,
    pub member): *mut *mut *mut void (cfg_port_member)(struct ksz_device dev, int port, u8,
    pub cnt): *mut u64,
    pub cnt): *mut *mut u64 dropped, u64,
    pub port): *mut *mut *mut void (r_mib_stat64)(struct ksz_device dev, int,
    pub value): *mut *mut *mut int (pme_write8)(struct ksz_device dev, u32 reg, u8,
    pub data): *mut u8,
    pub data): u8,
    pub freeze): *mut *mut *mut void (freeze_mib)(struct ksz_device dev, int port, bool,
    pub port): *mut *mut *mut void (port_init_cnt)(struct ksz_device dev, int,
    pub val): *mut *mut *mut int (tc_cbs_set_cinc)(struct ksz_device dev, int port, u32,
    pub dev): *mut *mut int (init)(struct ksz_device,
}

extern "C" {
    pub fn ksz_switch_register(dev: *mut ksz_device) -> c_int;
}
extern "C" {
    pub fn ksz_switch_remove(dev: *mut ksz_device);
}
extern "C" {
    pub fn ksz_switch_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ksz_switch_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ksz_teardown(ds: *mut dsa_switch);
}
extern "C" {
    pub fn ksz_init_mib_timer(dev: *mut ksz_device);
}
extern "C" {
    pub fn ksz_r_mib_stats64(dev: *mut ksz_device, port: c_int);
}
extern "C" {
    pub fn ksz_port_stp_state_set(ds: *mut dsa_switch, port: c_int, state: u8);
}
extern "C" {
    pub fn ksz_switch_macaddr_put(ds: *mut dsa_switch);
}
extern "C" {
    pub fn ksz_switch_shutdown(dev: *mut ksz_device);
}
extern "C" {
    pub fn ksz_handle_wake_reason(dev: *mut ksz_device, port: c_int) -> c_int;
}
extern "C" {
    pub fn ksz_sset_count(ds: *mut dsa_switch, port: c_int, sset: c_int) -> c_int;
}
extern "C" {
    pub fn ksz_phylink_mac_disable_tx_lpi(config: *mut phylink_config);
}
extern "C" {
    pub fn ksz_set_xmii(dev: *mut ksz_device, port: c_int, interface: phy_interface_t);
}
extern "C" {
    pub fn ksz_phylink_need_config(config: *mut phylink_config, mode: c_uint) -> bool;
}
extern "C" {
    pub fn ksz_suspend(ds: *mut dsa_switch) -> c_int;
}
extern "C" {
    pub fn ksz_resume(ds: *mut dsa_switch) -> c_int;
}
extern "C" {
    pub fn ksz_sw_mdio_read(bus: *mut mii_bus, addr: c_int, regnum: c_int) -> c_int;
}
extern "C" {
    pub fn ksz_sw_mdio_write(bus: *mut mii_bus, addr: c_int, regnum: c_int, val: u16) -> c_int;
}
extern "C" {
    pub fn ksz_parent_mdio_read(bus: *mut mii_bus, addr: c_int, regnum: c_int) -> c_int;
}
extern "C" {
    pub fn ksz_parent_mdio_write(bus: *mut mii_bus, addr: c_int, regnum: c_int, val: u16) -> c_int;
}
extern "C" {
    pub fn ksz_mdio_register(dev: *mut ksz_device) -> c_int;
}
extern "C" {
    pub fn ksz_irq_bus_lock(d: *mut irq_data);
}
extern "C" {
    pub fn ksz_irq_bus_sync_unlock(d: *mut irq_data);
}
extern "C" {
    pub fn ksz_pirq_setup(dev: *mut ksz_device, p: u8) -> c_int;
}
extern "C" {
    pub fn ksz_girq_setup(dev: *mut ksz_device) -> c_int;
}
extern "C" {
    pub fn ksz_irq_free(kirq: *mut ksz_irq);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_driver_strength_prop {
    pub name: *const c_char,
    pub offset: c_int,
    pub value: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ksz_driver_strength_type {
    KSZ_DRIVER_STRENGTH_HI,
    KSZ_DRIVER_STRENGTH_LO,
    KSZ_DRIVER_STRENGTH_IO,
}

//
// struct ksz_drive_strength - drive strength mapping
// @reg_val:	register value
// @microamp:	microamp value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_drive_strength {
    pub reg_val: u32,
    pub microamp: u32,
}

// Common register access functions
// val = value;
// val = (u64)value[0] << 32 | value[1];
// Ick! ToDo: Add 64bit R/W to regmap on 32bit systems
extern "C" {
    pub fn regmap_bulk_write(_arg: ksz_regmap_32(dev), _arg: reg, _arg: val, _arg: 2) -> return;
}
extern "C" {
    pub fn ksz_read8(_arg: dev, _arg: dev->dev_ops->get_port_addr(port, _arg: offset), _arg: data) -> return;
}
extern "C" {
    pub fn ksz_read16(_arg: dev, _arg: dev->dev_ops->get_port_addr(port, _arg: offset), _arg: data) -> return;
}
extern "C" {
    pub fn ksz_read32(_arg: dev, _arg: dev->dev_ops->get_port_addr(port, _arg: offset), _arg: data) -> return;
}
extern "C" {
    pub fn ksz_write8(_arg: dev, _arg: dev->dev_ops->get_port_addr(port, _arg: offset), _arg: data) -> return;
}
// STP State Defines

// Switch ID Defines
pub const REG_CHIP_ID0: c_uint = 0x00;

pub const KSZ84_FAMILY_ID: c_uint = 0x84;
pub const KSZ87_FAMILY_ID: c_uint = 0x87;
pub const KSZ88_FAMILY_ID: c_uint = 0x88;
pub const KSZ8895_FAMILY_ID: c_uint = 0x95;
pub const KSZ8_PORT_STATUS_0: c_uint = 0x08;

pub const KSZ87_CHIP_ID_94: c_uint = 0x6;
pub const KSZ87_CHIP_ID_95: c_uint = 0x9;
pub const KSZ88_CHIP_ID_63: c_uint = 0x3;
pub const KSZ8895_CHIP_ID_95: c_uint = 0x4;
pub const KSZ8895_CHIP_ID_95R: c_uint = 0x6;
// KSZ8895 specific register
pub const REG_KSZ8864_CHIP_ID: c_uint = 0xFE;

// KSZ9893, KSZ9563, KSZ8563 specific register
pub const REG_CHIP_ID4: c_uint = 0x0f;
pub const SKU_ID_KSZ8563: c_uint = 0x3c;
pub const SKU_ID_KSZ9563: c_uint = 0x1c;
// Driver set switch broadcast storm protection at 10% rate.
pub const BROADCAST_STORM_PROT_RATE: c_int = 10;
// 148,800 frames * 67 ms / 100
pub const BROADCAST_STORM_VALUE: c_int = 9969;
pub const BROADCAST_STORM_RATE_HI: c_uint = 0x07;
pub const BROADCAST_STORM_RATE_LO: c_uint = 0xFF;
pub const BROADCAST_STORM_RATE: c_uint = 0x07FF;

pub const SW_START: c_uint = 0x01;
// xMII configuration

pub const P_MII_SEL_M: c_uint = 0x3;
// KSZ9477, KSZ87xx Wake-on-LAN (WoL) masks

pub const KSZ87XX_REG_INT_EN: c_uint = 0x7D;

// Interrupt
pub const REG_SW_PORT_INT_STATUS__1: c_uint = 0x001B;
pub const REG_SW_PORT_INT_MASK__1: c_uint = 0x001F;
pub const REG_PORT_INT_STATUS: c_uint = 0x001B;
pub const REG_PORT_INT_MASK: c_uint = 0x001F;
pub const PORT_SRC_PHY_INT: c_int = 1;
pub const PORT_SRC_PTP_INT: c_int = 2;
pub const KSZ8795_HUGE_PACKET_SIZE: c_int = 2000;
pub const KSZ8863_HUGE_PACKET_SIZE: c_int = 1916;
pub const KSZ8863_NORMAL_PACKET_SIZE: c_int = 1536;
pub const KSZ8_LEGAL_PACKET_SIZE: c_int = 1518;
pub const KSZ9477_MAX_FRAME_SIZE: c_int = 9000;
pub const KSZ8873_REG_GLOBAL_CTRL_12: c_uint = 0x0e;
// Drive Strength of I/O Pad
// 0: 8mA, 1: 16mA
//

pub const KSZ8795_REG_SW_CTRL_20: c_uint = 0xa3;
pub const KSZ9477_REG_SW_IO_STRENGTH: c_uint = 0x010d;
pub const SW_DRIVE_STRENGTH_M: c_uint = 0x7;
pub const SW_DRIVE_STRENGTH_2MA: c_int = 0;
pub const SW_DRIVE_STRENGTH_4MA: c_int = 1;
pub const SW_DRIVE_STRENGTH_8MA: c_int = 2;
pub const SW_DRIVE_STRENGTH_12MA: c_int = 3;
pub const SW_DRIVE_STRENGTH_16MA: c_int = 4;
pub const SW_DRIVE_STRENGTH_20MA: c_int = 5;
pub const SW_DRIVE_STRENGTH_24MA: c_int = 6;
pub const SW_DRIVE_STRENGTH_28MA: c_int = 7;
pub const SW_HI_SPEED_DRIVE_STRENGTH_S: c_int = 4;
pub const SW_LO_SPEED_DRIVE_STRENGTH_S: c_int = 0;
// TXQ Split Control Register for per-port, per-queue configuration.
// Register 0xAF is TXQ Split for Q3 on Port 1.
// Register offset formula: 0xAF + (port * 4) + (3 - queue)
// where: port = 0..2, queue = 0..3
//

// Bit 7 selects between:
// 0 = Strict priority mode (highest-priority queue first)
// 1 = Weighted Fair Queuing (WFQ) mode:
// Queue weights: Q3:Q2:Q1:Q0 = 8:4:2:1
// If any queues are empty, weight is redistributed.
//
// Note: This is referred to as "Weighted Fair Queuing" (WFQ) in KSZ8863/8873
// documentation, and as "Weighted Round Robin" (WRR) in KSZ9477 family docs.
//

pub const KSZ9477_REG_PORT_OUT_RATE_0: c_uint = 0x0420;
pub const KSZ9477_OUT_RATE_NO_LIMIT: c_int = 0;
pub const KSZ9477_PORT_MRI_TC_MAP__4: c_uint = 0x0808;
pub const KSZ9477_PORT_TC_MAP_S: c_int = 4;
// CBS related registers
pub const REG_PORT_MTI_QUEUE_INDEX__4: c_uint = 0x0900;
pub const REG_PORT_MTI_QUEUE_CTRL_0: c_uint = 0x0914;

pub const MTI_SCHEDULE_STRICT_PRIO: c_int = 0;
pub const MTI_SCHEDULE_WRR: c_int = 2;

pub const MTI_SHAPING_OFF: c_int = 0;
pub const MTI_SHAPING_SRP: c_int = 1;
pub const MTI_SHAPING_TIME_AWARE: c_int = 2;
pub const KSZ9477_PORT_MTI_QUEUE_CTRL_1: c_uint = 0x0915;
pub const KSZ9477_DEFAULT_WRR_WEIGHT: c_int = 1;
pub const REG_PORT_MTI_HI_WATER_MARK: c_uint = 0x0916;
pub const REG_PORT_MTI_LO_WATER_MARK: c_uint = 0x0918;
// Regmap tables generation
pub const KSZ_SPI_OP_RD: c_int = 3;
pub const KSZ_SPI_OP_WR: c_int = 2;
pub const swabnot_used(x): c_int = 0;


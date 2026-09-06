//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/sja1105/sja1105.h
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
// Copyright (c) 2018, Sensor-Technik Wiedemann GmbH
// Copyright (c) 2018-2019, Vladimir Oltean <olteanv@gmail.com>
//

pub const SJA1105ET_FDB_BIN_SIZE: c_int = 4;
// The hardware value is in multiples of 10 ms.
// The passed parameter is in multiples of 1 ms.
//

// Calculated assuming 1Gbps, where the clock has 125 MHz (8 ns period)
// To avoid floating point operations, we'll multiply the degrees by 10
// to get a "phase" and get 1 decimal point precision.
//

// Valid range in degrees is a value between 73.8 and 101.7
// in 0.9 degree increments
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sja1105_stats_area {
    MAC,
    HL1,
    HL2,
    ETHER,
    __MAX_SJA1105_STATS_AREA,
}

// Keeps the different addresses between E/T and P/Q/R/S
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_regs {
    pub device_id: u64,
    pub prod_id: u64,
    pub status: u64,
    pub port_control: u64,
    pub rgu: u64,
    pub vl_status: u64,
    pub config: u64,
    pub rmii_pll1: u64,
    pub ptppinst: u64,
    pub ptppindur: u64,
    pub ptp_control: u64,
    pub ptpclkval: u64,
    pub ptpclkrate: u64,
    pub ptpclkcorp: u64,
    pub ptpsyncts: u64,
    pub ptpschtm: u64,
    pub ptpegr_ts: [u64; SJA1105_MAX_NUM_PORTS],
    pub pad_mii_tx: [u64; SJA1105_MAX_NUM_PORTS],
    pub pad_mii_rx: [u64; SJA1105_MAX_NUM_PORTS],
    pub pad_mii_id: [u64; SJA1105_MAX_NUM_PORTS],
    pub cgu_idiv: [u64; SJA1105_MAX_NUM_PORTS],
    pub mii_tx_clk: [u64; SJA1105_MAX_NUM_PORTS],
    pub mii_rx_clk: [u64; SJA1105_MAX_NUM_PORTS],
    pub mii_ext_tx_clk: [u64; SJA1105_MAX_NUM_PORTS],
    pub mii_ext_rx_clk: [u64; SJA1105_MAX_NUM_PORTS],
    pub rgmii_tx_clk: [u64; SJA1105_MAX_NUM_PORTS],
    pub rmii_ref_clk: [u64; SJA1105_MAX_NUM_PORTS],
    pub rmii_ext_tx_clk: [u64; SJA1105_MAX_NUM_PORTS],
    pub stats: [u64; __MAX_SJA1105_STATS_AREA][SJA1105_MAX_NUM_PORTS],
    pub mdio_100base_tx: u64,
    pub mdio_100base_t1: u64,
    pub pcs_base: [u64; SJA1105_MAX_NUM_PORTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_mdio_private {
    pub priv: *mut sja1105_private,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sja1105_internal_phy_t {
    SJA1105_NO_PHY		= 0,
    SJA1105_PHY_BASE_TX,
    SJA1105_PHY_BASE_T1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_info {
    pub device_id: u64,
// Needed for distinction between P and R, and between Q and S
// (since the parts with/without SGMII share the same
// switch core and device_id)
//
    pub part_no: u64,
// E/T and P/Q/R/S have partial timestamps of different sizes.
// They must be reconstructed on both families anyway to get the full
// 64-bit values back.
//
    pub ptp_ts_bits: c_int,
// Also SPI commands are of different sizes to retrieve
// the egress timestamps.
//
    pub ptpegr_ts_bytes: c_int,
    pub num_cbs_shapers: c_int,
    pub max_frame_mem: c_int,
    pub num_ports: c_int,
    pub multiple_cascade_ports: bool,
// Every {port, TXQ} has its own CBS shaper
    pub fixed_cbs_mapping: bool,
    pub tag_proto: dsa_tag_protocol,
    pub dyn_ops: *const sja1105_dynamic_table_ops,
    pub static_ops: *const sja1105_table_ops,
    pub regs: *const sja1105_regs,
    pub can_limit_mcast_flood: bool,
    pub ds): *mut *mut int (reset_cmd)(struct dsa_switch,
    pub port): *const *const *const int (setup_rgmii_delay)(void ctx, int,
// Prototypes from include/net/dsa.h
    pub vid): *const *const unsigned char addr, u16,
    pub vid): *const *const unsigned char addr, u16,
    pub op): packing_op,
    pub skb): *mut *mut *mut bool (rxtstamp)(struct dsa_switch ds, int port, struct sk_buff,
    pub skb): *mut *mut *mut void (txtstamp)(struct dsa_switch ds, int port, struct sk_buff,
    pub priv): *mut *mut int (clocking_setup)(struct sja1105_private,
    pub reg): c_int,
    pub val): int reg, u16,
    pub priv): *mut *mut int (disable_microcontroller)(struct sja1105_private,
    pub name: *const c_char,
    pub supports_mii: [bool; SJA1105_MAX_NUM_PORTS],
    pub supports_rmii: [bool; SJA1105_MAX_NUM_PORTS],
    pub supports_rgmii: [bool; SJA1105_MAX_NUM_PORTS],
    pub supports_sgmii: [bool; SJA1105_MAX_NUM_PORTS],
    pub supports_2500basex: [bool; SJA1105_MAX_NUM_PORTS],
    pub internal_phy: [sja1105_internal_phy_t; SJA1105_MAX_NUM_PORTS],
    pub port_speed: [u64; SJA1105_SPEED_MAX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sja1105_key_type {
    SJA1105_KEY_BCAST,
    SJA1105_KEY_TC,
    SJA1105_KEY_VLAN_UNAWARE_VL,
    SJA1105_KEY_VLAN_AWARE_VL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_key {
    pub type: sja1105_key_type,
// SJA1105_KEY_TC
    pub pcp: c_int,
    pub tc: },
// SJA1105_KEY_VLAN_UNAWARE_VL
// SJA1105_KEY_VLAN_AWARE_VL
    pub dmac: u64,
    pub vid: u16,
    pub pcp: u16,
    pub vl: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sja1105_rule_type {
    SJA1105_RULE_BCAST_POLICER,
    SJA1105_RULE_TC_POLICER,
    SJA1105_RULE_VL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sja1105_vl_type {
    SJA1105_VL_NONCRITICAL,
    SJA1105_VL_RATE_CONSTRAINED,
    SJA1105_VL_TIME_TRIGGERED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_rule {
    pub list: list_head,
    pub cookie: c_ulong,
    pub port_mask: c_ulong,
    pub key: sja1105_key,
    pub type: sja1105_rule_type,
// Action
// SJA1105_RULE_BCAST_POLICER
    pub sharindx: c_int,
    pub bcast_pol: },
// SJA1105_RULE_TC_POLICER
    pub sharindx: c_int,
    pub tc_pol: },
// SJA1105_RULE_VL
    pub type: sja1105_vl_type,
    pub destports: c_ulong,
    pub sharindx: c_int,
    pub maxlen: c_int,
    pub ipv: c_int,
    pub base_time: u64,
    pub cycle_time: u64,
    pub num_entries: c_int,
    pub entries: *mut action_gate_entry,
    pub stats: flow_stats,
    pub vl: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_flow_block {
    pub rules: list_head,
    pub l2_policer_used: [bool; SJA1105_NUM_L2_POLICERS],
    pub num_virtual_links: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_private {
    pub static_config: sja1105_static_config,
    pub rgmii_rx_delay_ps: [c_int; SJA1105_MAX_NUM_PORTS],
    pub rgmii_tx_delay_ps: [c_int; SJA1105_MAX_NUM_PORTS],
    pub phy_mode: [phy_interface_t; SJA1105_MAX_NUM_PORTS],
    pub fixed_link: [bool; SJA1105_MAX_NUM_PORTS],
    pub ucast_egress_floods: c_ulong,
    pub bcast_egress_floods: c_ulong,
    pub hwts_tx_en: c_ulong,
    pub hwts_rx_en: c_ulong,
    pub info: *const sja1105_info,
    pub max_xfer_len: usize,
    pub spidev: *mut spi_device,
    pub ds: *mut dsa_switch,
    pub bridge_pvid: [u16; SJA1105_MAX_NUM_PORTS],
    pub tag_8021q_pvid: [u16; SJA1105_MAX_NUM_PORTS],
    pub flow_block: sja1105_flow_block,
// Serializes transmission of management frames so that
// the switch doesn't confuse them with one another.
//
    pub mgmt_lock: mutex,
// Serializes accesses to the FDB
    pub fdb_lock: mutex,
// PTP two-step TX timestamp ID, and its serialization lock
    pub ts_id_lock: spinlock_t,
    pub ts_id: u8,
// Serializes access to the dynamic config interface
    pub dynamic_config_lock: mutex,
    pub regions: *mut devlink_region,
    pub cbs: *mut sja1105_cbs_entry,
    pub mdio_base_t1: *mut mii_bus,
    pub mdio_base_tx: *mut mii_bus,
    pub mdio_pcs: *mut mii_bus,
    pub pcs: [*mut phylink_pcs; SJA1105_MAX_NUM_PORTS],
    pub ptp_data: sja1105_ptp_data,
    pub tas_data: sja1105_tas_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_spi_message {
    pub access: u64,
    pub read_count: u64,
    pub address: u64,
}

// From sja1105_main.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sja1105_reset_reason {
    SJA1105_VLAN_FILTERING = 0,
    SJA1105_AGEING_TIME,
    SJA1105_SCHEDULING,
    SJA1105_BEST_EFFORT_POLICING,
    SJA1105_VIRTUAL_LINKS,
}

extern "C" {
    pub fn sja1105_frame_memory_partitioning(priv: *mut sja1105_private);
}
// From sja1105_mdio.c
extern "C" {
    pub fn sja1105_mdiobus_register(ds: *mut dsa_switch) -> c_int;
}
extern "C" {
    pub fn sja1105_mdiobus_unregister(ds: *mut dsa_switch);
}
extern "C" {
    pub fn sja1105_pcs_mdio_read_c45(bus: *mut mii_bus, phy: c_int, mmd: c_int, reg: c_int) -> c_int;
}
extern "C" {
    pub fn sja1110_pcs_mdio_read_c45(bus: *mut mii_bus, phy: c_int, mmd: c_int, reg: c_int) -> c_int;
}
// From sja1105_devlink.c
extern "C" {
    pub fn sja1105_devlink_setup(ds: *mut dsa_switch) -> c_int;
}
extern "C" {
    pub fn sja1105_devlink_teardown(ds: *mut dsa_switch);
}
// From sja1105_spi.c
extern "C" {
    pub fn sja1105_static_config_upload(priv: *mut sja1105_private) -> c_int;
}
// From sja1105_clocking.c
extern "C" {
    pub fn sja1105pqrs_setup_rgmii_delay(ctx: *const c_void, port: c_int) -> c_int;
}
extern "C" {
    pub fn sja1110_setup_rgmii_delay(ctx: *const c_void, port: c_int) -> c_int;
}
extern "C" {
    pub fn sja1105_clocking_setup_port(priv: *mut sja1105_private, port: c_int) -> c_int;
}
extern "C" {
    pub fn sja1105_clocking_setup(priv: *mut sja1105_private) -> c_int;
}
extern "C" {
    pub fn sja1110_disable_microcontroller(priv: *mut sja1105_private) -> c_int;
}
// From sja1105_ethtool.c
extern "C" {
    pub fn sja1105_get_ethtool_stats(ds: *mut dsa_switch, port: c_int, data: *mut u64);
}
extern "C" {
    pub fn sja1105_get_sset_count(ds: *mut dsa_switch, port: c_int, sset: c_int) -> c_int;
}
// From sja1105_dynamic_config.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sja1105_iotag {
    SJA1105_C_TAG = 0, /* Inner VLAN header */
    SJA1105_S_TAG = 1, /* Outer VLAN header */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sja1110_vlan_type {
    SJA1110_VLAN_INVALID = 0,
    SJA1110_VLAN_C_TAG = 1, /* Single inner VLAN tag */
    SJA1110_VLAN_S_TAG = 2, /* Single outer VLAN tag */
    SJA1110_VLAN_D_TAG = 3, /* Double tagged, use outer tag for lookup */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sja1110_shaper_type {
    SJA1110_LEAKY_BUCKET_SHAPER = 0,
    SJA1110_CBS_SHAPER = 1,
}

extern "C" {
    pub fn sja1105et_fdb_hash(priv: *mut sja1105_private, addr: *const u8, vid: u16) -> u8;
}
// From sja1105_flower.c
extern "C" {
    pub fn sja1105_flower_setup(ds: *mut dsa_switch);
}
extern "C" {
    pub fn sja1105_flower_teardown(ds: *mut dsa_switch);
}

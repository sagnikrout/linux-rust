//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/qca/qca8k.h
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
// Copyright (C) 2009 Felix Fietkau <nbd@nbd.name>
// Copyright (C) 2011-2012 Gabor Juhos <juhosg@openwrt.org>
// Copyright (c) 2015, The Linux Foundation. All rights reserved.
//

pub const QCA8K_ETHERNET_MDIO_PRIORITY: c_int = 7;
pub const QCA8K_ETHERNET_PHY_PRIORITY: c_int = 6;

pub const QCA8K_NUM_PORTS: c_int = 7;
pub const QCA8K_NUM_CPU_PORTS: c_int = 2;
pub const QCA8K_MAX_MTU: c_int = 9000;
pub const QCA8K_NUM_LAGS: c_int = 4;
pub const QCA8K_NUM_PORTS_FOR_LAG: c_int = 4;
pub const PHY_ID_QCA8327: c_uint = 0x004dd034;
pub const QCA8K_ID_QCA8327: c_uint = 0x12;
pub const PHY_ID_QCA8337: c_uint = 0x004dd036;
pub const QCA8K_ID_QCA8337: c_uint = 0x13;
pub const QCA8K_QCA832X_MIB_COUNT: c_int = 39;
pub const QCA8K_QCA833X_MIB_COUNT: c_int = 41;
pub const QCA8K_BUSY_WAIT_TIMEOUT: c_int = 2000;
pub const QCA8K_NUM_FDB_RECORDS: c_int = 2048;
pub const QCA8K_PORT_VID_DEF: c_int = 1;
// Global control registers
pub const QCA8K_REG_MASK_CTRL: c_uint = 0x000;

pub const QCA8K_REG_PORT0_PAD_CTRL: c_uint = 0x004;

pub const QCA8K_REG_PORT5_PAD_CTRL: c_uint = 0x008;
pub const QCA8K_REG_PORT6_PAD_CTRL: c_uint = 0x00c;

pub const QCA8K_REG_PWS: c_uint = 0x010;

// This reg is only valid for QCA832x and toggle the package
// type from 176 pin (by default) to 148 pin used on QCA8327
//

pub const QCA8K_REG_MODULE_EN: c_uint = 0x030;

pub const QCA8K_REG_MIB: c_uint = 0x034;

pub const QCA8K_MDIO_MASTER_CTRL: c_uint = 0x3c;

pub const QCA8K_MDIO_MASTER_WRITE: c_int = 0;

pub const QCA8K_MDIO_MASTER_MAX_PORTS: c_int = 5;
pub const QCA8K_MDIO_MASTER_MAX_REG: c_int = 32;
// LED control register
pub const QCA8K_LED_PORT_COUNT: c_int = 3;

pub const QCA8K_LED_RULE_COUNT: c_int = 6;
pub const QCA8K_LED_RULE_MAX: c_int = 11;

pub const QCA8K_LED_PHY0123_CONTROL_RULE_SHIFT: c_int = 0;
pub const QCA8K_LED_PHY4_CONTROL_RULE_SHIFT: c_int = 16;

pub const QCA8K_LED_CTRL0_REG: c_uint = 0x50;
pub const QCA8K_LED_CTRL1_REG: c_uint = 0x54;
pub const QCA8K_LED_CTRL2_REG: c_uint = 0x58;
pub const QCA8K_LED_CTRL3_REG: c_uint = 0x5C;

pub const QCA8K_LED_BLINK_FREQ_SHITF: c_int = 0;
pub const QCA8K_LED_BLINK_2HZ: c_int = 0;
pub const QCA8K_LED_BLINK_4HZ: c_int = 1;
pub const QCA8K_LED_BLINK_8HZ: c_int = 2;
pub const QCA8K_LED_BLINK_AUTO: c_int = 3;

pub const QCA8K_LED_PATTERN_EN_SHIFT: c_int = 14;
pub const QCA8K_LED_ALWAYS_OFF: c_int = 0;
pub const QCA8K_LED_ALWAYS_BLINK_4HZ: c_int = 1;
pub const QCA8K_LED_ALWAYS_ON: c_int = 2;
pub const QCA8K_LED_RULE_CONTROLLED: c_int = 3;
pub const QCA8K_GOL_MAC_ADDR0: c_uint = 0x60;
pub const QCA8K_GOL_MAC_ADDR1: c_uint = 0x64;
pub const QCA8K_MAX_FRAME_SIZE: c_uint = 0x78;

pub const QCA8K_PORT_STATUS_SPEED_10: c_int = 0;
pub const QCA8K_PORT_STATUS_SPEED_100: c_uint = 0x1;
pub const QCA8K_PORT_STATUS_SPEED_1000: c_uint = 0x2;

pub const QCA8K_PORT_HDR_CTRL_ALL: c_int = 2;
pub const QCA8K_PORT_HDR_CTRL_MGMT: c_int = 1;
pub const QCA8K_PORT_HDR_CTRL_NONE: c_int = 0;
pub const QCA8K_REG_SGMII_CTRL: c_uint = 0x0e0;

// MAC_PWR_SEL registers
pub const QCA8K_REG_MAC_PWR_SEL: c_uint = 0x0e4;

// EEE control registers
pub const QCA8K_REG_EEE_CTRL: c_uint = 0x100;

// TRUNK_HASH_EN registers
pub const QCA8K_TRUNK_HASH_EN_CTRL: c_uint = 0x270;

// ACL registers

pub const QCA8K_REG_IPV4_PRI_BASE_ADDR: c_uint = 0x470;
pub const QCA8K_REG_IPV4_PRI_ADDR_MASK: c_uint = 0x474;
// Lookup registers

pub const QCA8K_REG_ATU_DATA0: c_uint = 0x600;

pub const QCA8K_REG_ATU_DATA1: c_uint = 0x604;

pub const QCA8K_REG_ATU_DATA2: c_uint = 0x608;

pub const QCA8K_ATU_STATUS_STATIC: c_uint = 0xf;
pub const QCA8K_REG_ATU_FUNC: c_uint = 0x60c;

pub const QCA8K_REG_VTU_FUNC0: c_uint = 0x610;

// QCA8K_VTU_FUNC0_EG_MODE_MASK			GENMASK(17, 4)
// It does contain VLAN_MODE for each port [5:4] for port0,
// [7:6] for port1 ... [17:16] for port6. Use virtual port
// define to handle this.
//

pub const QCA8K_REG_VTU_FUNC1: c_uint = 0x614;

pub const QCA8K_REG_ATU_CTRL: c_uint = 0x618;

pub const QCA8K_REG_GLOBAL_FW_CTRL0: c_uint = 0x620;

pub const QCA8K_REG_GLOBAL_FW_CTRL1: c_uint = 0x624;

pub const QCA8K_REG_GOL_TRUNK_CTRL0: c_uint = 0x700;
// 4 max trunk first
// first 6 bit for member bitmap
// 7th bit is to enable trunk port
//

// 0x704 for TRUNK 0-1 --- 0x708 for TRUNK 2-3

// Complex shift: FIRST shift for port THEN shift for trunk

pub const QCA8K_REG_GLOBAL_FC_THRESH: c_uint = 0x800;

// Pkt edit registers

// L3 registers
pub const QCA8K_HROUTER_CONTROL: c_uint = 0xe00;

pub const QCA8K_HROUTER_CONTROL_GLB_LOCKTIME_S: c_int = 16;
pub const QCA8K_HROUTER_CONTROL_ARP_AGE_MODE: c_int = 1;
pub const QCA8K_HROUTER_PBASED_CONTROL1: c_uint = 0xe08;
pub const QCA8K_HROUTER_PBASED_CONTROL2: c_uint = 0xe0c;
pub const QCA8K_HNAT_CONTROL: c_uint = 0xe38;
// MIB registers

// QCA specific MII registers
pub const MII_ATH_MMD_ADDR: c_uint = 0x0d;
pub const MII_ATH_MMD_DATA: c_uint = 0x0e;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qca8k_fdb_cmd {
    QCA8K_FDB_FLUSH	= 1,
    QCA8K_FDB_LOAD = 2,
    QCA8K_FDB_PURGE = 3,
    QCA8K_FDB_FLUSH_PORT = 5,
    QCA8K_FDB_NEXT = 6,
    QCA8K_FDB_SEARCH = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qca8k_vlan_cmd {
    QCA8K_VLAN_FLUSH = 1,
    QCA8K_VLAN_LOAD = 2,
    QCA8K_VLAN_PURGE = 3,
    QCA8K_VLAN_REMOVE_PORT = 4,
    QCA8K_VLAN_NEXT = 5,
    QCA8K_VLAN_READ = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qca8k_mid_cmd {
    QCA8K_MIB_FLUSH = 1,
    QCA8K_MIB_FLUSH_PORT = 2,
    QCA8K_MIB_CAST = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca8k_info_ops {
    pub data): *mut *mut *mut int (autocast_mib)(struct dsa_switch ds, int port, u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca8k_match_data {
    pub id: u8,
    pub reduced_package: bool,
    pub mib_count: u8,
    pub ops: *const qca8k_info_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca8k_mgmt_eth_data {
    pub rw_done: completion,
    pub /: *mut *mut mutex mutex; / Enforce one mdio read/write at time,
    pub ack: bool,
    pub seq: u32,
    pub data: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca8k_mib_eth_data {
    pub rw_done: completion,
    pub /: *mut *mut mutex mutex; / Process one command at time,
    pub /: *mut *mut refcount_t port_parsed; / Counter to track parsed port,
    pub req_port: u8,
    pub /: *mut *mut *mut u64 data; / pointer to ethtool data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca8k_ports_config {
    pub sgmii_rx_clk_falling_edge: bool,
    pub sgmii_tx_clk_falling_edge: bool,
    pub sgmii_enable_pll: bool,
    pub /: *mut *mut u8 rgmii_rx_delay[QCA8K_NUM_CPU_PORTS]; / 0: CPU port0, 1: CPU port6,
    pub /: *mut *mut u8 rgmii_tx_delay[QCA8K_NUM_CPU_PORTS]; / 0: CPU port0, 1: CPU port6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca8k_mdio_cache {
// The 32bit switch registers are accessed indirectly. To achieve this we need
// to set the page of the register. Track the last page that was set to reduce
// mdio writes
//
    pub page: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca8k_pcs {
    pub pcs: phylink_pcs,
    pub priv: *mut qca8k_priv,
    pub port: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca8k_led_pattern_en {
    pub reg: u32,
    pub shift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca8k_led {
    pub port_num: u8,
    pub led_num: u8,
    pub old_rule: u16,
    pub priv: *mut qca8k_priv,
    pub cdev: led_classdev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca8k_priv {
    pub switch_id: u8,
    pub switch_revision: u8,
    pub mirror_rx: u8,
    pub mirror_tx: u8,
    pub lag_hash_mode: u8,
// Each bit correspond to a port. This switch can support a max of 7 port.
// Bit 1: port enabled. Bit 0: port disabled.
//
    pub port_enabled_map: u8,
    pub port_isolated_map: u8,
    pub ports_config: qca8k_ports_config,
    pub regmap: *mut regmap,
    pub bus: *mut mii_bus,
    pub internal_mdio_bus: *mut mii_bus,
    pub ds: *mut dsa_switch,
    pub reg_mutex: mutex,
    pub dev: *mut device,
    pub reset_gpio: *mut gpio_desc,
    pub /: *mut *mut *mut net_device mgmt_conduit; / Track if mdio/mib Ethernet is available,
    pub mgmt_eth_data: qca8k_mgmt_eth_data,
    pub mib_eth_data: qca8k_mib_eth_data,
    pub mdio_cache: qca8k_mdio_cache,
    pub pcs_port_0: qca8k_pcs,
    pub pcs_port_6: qca8k_pcs,
    pub info: *const qca8k_match_data,
    pub ports_led: [qca8k_led; QCA8K_LED_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca8k_mib_desc {
    pub size: c_uint,
    pub offset: c_uint,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca8k_fdb {
    pub vid: u16,
    pub port_mask: u8,
    pub aging: u8,
    pub mac: [u8; 6],
}

// From Andrew Lunn:
// Port 0 has no internal phy.
// Port 1 has an internal PHY at MDIO address 0.
// Port 2 has an internal PHY at MDIO address 1.
// ...
// Port 5 has an internal PHY at MDIO address 4.
// Port 6 has no internal PHY.
//
// Common setup function
extern "C" {
    pub fn qca8k_mib_init(priv: *mut qca8k_priv) -> c_int;
}
extern "C" {
    pub fn qca8k_port_set_status(priv: *mut qca8k_priv, port: c_int, enable: c_int);
}
extern "C" {
    pub fn qca8k_read_switch_id(priv: *mut qca8k_priv) -> c_int;
}
// Common read/write/rmw function
extern "C" {
    pub fn qca8k_read(priv: *mut qca8k_priv, reg: u32, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn qca8k_write(priv: *mut qca8k_priv, reg: u32, val: u32) -> c_int;
}
extern "C" {
    pub fn qca8k_rmw(priv: *mut qca8k_priv, reg: u32, mask: u32, write_val: u32) -> c_int;
}
// Common ops function
extern "C" {
    pub fn qca8k_fdb_flush(priv: *mut qca8k_priv);
}
// Common ethtool stats function
extern "C" {
    pub fn qca8k_get_strings(ds: *mut dsa_switch, port: c_int, stringset: u32, data: *mut u8);
}
extern "C" {
    pub fn qca8k_get_sset_count(ds: *mut dsa_switch, port: c_int, sset: c_int) -> c_int;
}
// Common eee function
extern "C" {
    pub fn qca8k_set_mac_eee(ds: *mut dsa_switch, port: c_int, eee: *mut ethtool_keee) -> c_int;
}
// Common bridge function
extern "C" {
    pub fn qca8k_port_stp_state_set(ds: *mut dsa_switch, port: c_int, state: u8);
}
// Common port enable/disable function
extern "C" {
    pub fn qca8k_port_disable(ds: *mut dsa_switch, port: c_int);
}
// Common MTU function
extern "C" {
    pub fn qca8k_port_change_mtu(ds: *mut dsa_switch, port: c_int, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn qca8k_port_max_mtu(ds: *mut dsa_switch, port: c_int) -> c_int;
}
// Common fast age function
extern "C" {
    pub fn qca8k_port_fast_age(ds: *mut dsa_switch, port: c_int);
}
extern "C" {
    pub fn qca8k_set_ageing_time(ds: *mut dsa_switch, msecs: c_uint) -> c_int;
}
// Common FDB function
// Common MDB function
// Common port mirror function
// Common port VLAN function
// Common port LAG function

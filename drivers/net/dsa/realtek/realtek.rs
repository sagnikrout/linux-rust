//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/realtek/realtek.h
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
// Realtek SMI interface driver defines
//
// Copyright (C) 2017 Linus Walleij <linus.walleij@linaro.org>
// Copyright (C) 2009-2010 Gabor Juhos <juhosg@openwrt.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8366_mib_counter {
    pub base: c_uint,
    pub offset: c_uint,
    pub length: c_uint,
    pub name: *const c_char,
}

//
// struct rtl8366_vlan_mc - Virtual LAN member configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8366_vlan_mc {
    pub vid: u16,
    pub untag: u16,
    pub member: u16,
    pub fid: u8,
    pub priority: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8366_vlan_4k {
    pub vid: u16,
    pub untag: u16,
    pub member: u16,
    pub fid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct realtek_fdb_entry {
    pub mac_addr: [u8; ETH_ALEN],
    pub vid: u16,
    pub is_static: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct realtek_priv {
    pub dev: *mut device,
    pub reset_ctl: *mut reset_control,
    pub reset: *mut gpio_desc,
    pub mdc: *mut gpio_desc,
    pub mdio: *mut gpio_desc,
    pub map: *mut regmap,
    pub map_nolock: *mut regmap,
    pub map_lock: mutex,
// vlan_lock protects against concurrent Read-Modify-Write operations
// on the global VLAN 4K and VLANMC tables, such as when adding or
// deleting port VLAN memberships and PVID configurations.
//
    pub vlan_lock: mutex,
// l2_lock is used to prevent concurrent modifications of L2 table
// entries while another function is reading it. l2_(add,del)_mc
// is an example that first read current table entry and then
// create/update it. l2_(add|del)_uc uses a single table op and,
// internally, it might not need this lock. However, altering FDB
// may still collide, as well as l2_flush, with fdb_dump iterating
// over FDB.
//
    pub l2_lock: mutex,
    pub user_mii_bus: *mut mii_bus,
    pub bus: *mut mii_bus,
    pub mdio_addr: c_int,
    pub variant: *const realtek_variant,
    pub /: *mut *mut spinlock_t lock; / Locks around command writes,
    pub ds: dsa_switch,
    pub irqdomain: *mut irq_domain,
    pub leds_disabled: bool,
    pub cpu_port: c_uint,
    pub num_ports: c_uint,
    pub num_vlan_mc: c_uint,
    pub num_mib_counters: c_uint,
    pub mib_counters: *mut rtl8366_mib_counter,
    pub ops: *const realtek_ops,
    pub data): *mut *mut *mut int (write_reg_noack)(void ctx, u32 addr, u32,
    pub vlan_enabled: c_int,
    pub vlan4k_enabled: c_int,
    pub buf: [c_char; 4096],
    pub /: *mut *mut *mut void chip_data; / Per-chip extra variant data,
}

//
// struct realtek_ops - vtable for the per-SMI-chiptype operations
// @detect: detects the chiptype
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct realtek_ops {
    pub priv): *mut *mut int (detect)(struct realtek_priv,
    pub priv): *mut *mut int (reset_chip)(struct realtek_priv,
    pub priv): *mut *mut int (setup)(struct realtek_priv,
    pub mibvalue): *mut u64,
    pub vlanmc): *mut rtl8366_vlan_mc,
    pub vlanmc): *const rtl8366_vlan_mc,
    pub vlan4k): *mut rtl8366_vlan_4k,
    pub vlan4k): *const rtl8366_vlan_4k,
    pub val): *mut *mut *mut int (get_mc_index)(struct realtek_priv priv, int port, int,
    pub index): *mut *mut *mut int (set_mc_index)(struct realtek_priv priv, int port, int,
    pub vlan): *mut *mut *mut bool (is_vlan_valid)(struct realtek_priv priv, unsigned int,
    pub enable): *mut *mut *mut int (enable_vlan)(struct realtek_priv priv, bool,
    pub enable): *mut *mut *mut int (enable_vlan4k)(struct realtek_priv priv, bool,
    pub enable): *mut *mut *mut int (enable_port)(struct realtek_priv priv, int port, bool,
    pub mask): u32,
    pub mask): u32,
    pub efid): *mut *mut *mut int (port_set_efid)(struct realtek_priv priv, int port, u32,
    pub enable): bool,
    pub enable): bool,
    pub enable): bool,
    pub enable): bool,
    pub vid): u16 efid, u16,
    pub vid): u16 efid, u16,
    pub entry): *mut int port, struct realtek_fdb_entry,
    pub vid): unsigned char addr[ETH_ALEN], u16,
    pub vid): unsigned char addr[ETH_ALEN], u16,
    pub vid): *mut *mut *mut int (l2_flush)(struct realtek_priv priv, int port, u16,
    pub regnum): *mut *mut *mut int (phy_read)(struct realtek_priv priv, int phy, int,
    pub val): u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct realtek_variant {
    pub ds_ops: *const dsa_switch_ops,
    pub ops: *const realtek_ops,
    pub phylink_mac_ops: *const phylink_mac_ops,
    pub clk_delay: c_uint,
    pub cmd_read: u8,
    pub cmd_write: u8,
    pub chip_data_sz: usize,
}

// RTL8366 library helpers
extern "C" {
    pub fn rtl8366_mc_is_used(priv: *mut realtek_priv, mc_index: c_int, used: *mut c_int) -> c_int;
}
extern "C" {
    pub fn rtl8366_enable_vlan4k(priv: *mut realtek_priv, enable: bool) -> c_int;
}
extern "C" {
    pub fn rtl8366_enable_vlan(priv: *mut realtek_priv, enable: bool) -> c_int;
}
extern "C" {
    pub fn rtl8366_reset_vlan(priv: *mut realtek_priv) -> c_int;
}
extern "C" {
    pub fn rtl8366_get_sset_count(ds: *mut dsa_switch, port: c_int, sset: c_int) -> c_int;
}
extern "C" {
    pub fn rtl8366_get_ethtool_stats(ds: *mut dsa_switch, port: c_int, data: *mut u64);
}

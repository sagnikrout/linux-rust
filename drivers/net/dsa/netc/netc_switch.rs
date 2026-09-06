//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/netc/netc_switch.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Copyright 2025-2026 NXP
//

pub const NETC_REGS_BAR: c_int = 0;
pub const NETC_REGS_SIZE: c_uint = 0x80000;
pub const NETC_MSIX_TBL_BAR: c_int = 2;
pub const NETC_REGS_PORT_BASE: c_uint = 0x4000;
// register block size per port
pub const NETC_REGS_PORT_SIZE: c_uint = 0x4000;

pub const NETC_REGS_GLOBAL_BASE: c_uint = 0x70000;
pub const NETC_SWITCH_REV_4_3: c_uint = 0x0403;
pub const NETC_TC_NUM: c_int = 8;
pub const NETC_CBDR_NUM: c_int = 2;
pub const NETC_IPV_NUM: c_int = 8;
pub const NETC_MAX_FRAME_LEN: c_int = 9600;
pub const NETC_STANDALONE_PVID: c_int = 0;

// Threshold format: MANT (bits 11:4) * 2^EXP (bits 3:0)
// Unit: Memory words (average of 20 bytes each)
// NETC_BP_THRESH = 0x8c3, MANT = 0x8c, EXP = 3. Threshold: 1120 words
// NETC_FC_THRESH_ON = 0x733, MANT = 0x73, EXP = 3. Threshold: 920 words
// NETC_FC_THRESH_OFF = 0x263, MANT = 0x26, EXP = 3. Threshold: 304 words
//
pub const NETC_BP_THRESH: c_uint = 0x8c3;
pub const NETC_FC_THRESH_ON: c_uint = 0x733;
pub const NETC_FC_THRESH_OFF: c_uint = 0x263;
// PAUSE quanta: 0xFFFF = 65535 quanta (each quanta = 512 bit times)
pub const NETC_PAUSE_QUANTA: c_uint = 0xFFFF;
// PAUSE refresh threshold: send refresh when timer reaches this value
pub const NETC_PAUSE_THRESH: c_uint = 0x7FFF;

pub const NETC_FDBT_AGEING_THRESH: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_switch_info {
    pub num_ports: u32,
    pub config): *mut *mut void (phylink_get_caps)(int port, struct phylink_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_port_caps {
    pub /: *mut *mut u32 half_duplex:1; / indicates whether the port support half-duplex,
    pub /: *mut *mut u32 pmac:1; / indicates whether the port has preemption MAC,
    pub pseudo_link:1: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netc_host_reason {
// Software defined host reasons
    NETC_HR_HOST_FLOOD = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_port {
    pub iobase: *mut void __iomem,
    pub switch_priv: *mut netc_switch,
    pub caps: netc_port_caps,
    pub dp: *mut dsa_port,
    pub /: *mut *mut *mut clk ref_clk; / RGMII/RMII reference clock,
    pub emdio: *mut mii_bus,
    pub ett_offset: c_int,
    pub enable:1: u16,
    pub uc:1: u16,
    pub mc:1: u16,
    pub pvid: u16,
    pub host_flood: *mut ipft_entry_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_switch_regs {
    pub base: *mut void __iomem,
    pub port: *mut void __iomem,
    pub global: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_fdb_entry {
    pub entry_id: u32,
    pub cfge: fdbt_cfge_data,
    pub keye: fdbt_keye_data,
    pub node: hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_vlan_entry {
    pub vid: u16,
    pub ect_gid: u32,
    pub untagged_port_bitmap: u32,
    pub cfge: vft_cfge_data,
    pub node: hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_port_stat {
    pub reg: c_int,
    pub __nonstring: char name[ETH_GSTRING_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_switch {
    pub pdev: *mut pci_dev,
    pub dev: *mut device,
    pub ds: *mut dsa_switch,
    pub revision: u16,
    pub info: *const netc_switch_info,
    pub regs: netc_switch_regs,
    pub ports: *mut netc_port,
    pub /: *mut *mut u32 port_bitmap; / bitmap of available ports,
    pub ntmp: ntmp_user,
    pub fdb_list: hlist_head,
    pub /: *mut *mut mutex fdbt_lock; / FDB table lock,
    pub fdbt_ageing_work: delayed_work,
// (fdbt_ageing_delay * NETC_FDBT_AGEING_THRESH) is ageing time
    pub fdbt_ageing_delay: c_ulong,
    pub br_cnt: core::sync::atomic::AtomicI32,
    pub vlan_list: hlist_head,
    pub /: *mut *mut mutex vft_lock; / VLAN filter table lock,
// Switch hardware capabilities
    pub htmcapr_num_words: u32,
    pub num_bp: u32,
    pub bpt_list: *mut bpt_cfge_data,
}

// Write/Read Switch base registers

// Write/Read registers of Switch Port (including pseudo MAC port)

// Write/Read Switch global registers

extern "C" {
    pub fn netc_switch_platform_probe(priv: *mut netc_switch) -> c_int;
}
// ethtool APIs
extern "C" {
    pub fn netc_port_get_sset_count(ds: *mut dsa_switch, port: c_int, sset: c_int) -> c_int;
}
extern "C" {
    pub fn netc_port_get_ethtool_stats(ds: *mut dsa_switch, port: c_int, data: *mut u64);
}

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mediatek/mtk_ppe.h
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
// Copyright (C) 2020 Felix Fietkau <nbd@nbd.name>

pub const MTK_PPE_ENTRIES_SHIFT: c_int = 4;

pub const MTK_PPE_WAIT_TIMEOUT_US: c_int = 1000000;

// CONFIG_MEDIATEK_NETSYS_V2

// CONFIG_MEDIATEK_NETSYS_V2

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_foe_mac_info {
    pub vlan1: u16,
    pub etype: u16,
    pub dest_mac_hi: u32,
    pub vlan2: u16,
    pub dest_mac_lo: u16,
    pub src_mac_hi: u32,
    pub pppoe_id: u16,
    pub src_mac_lo: u16,
// netsys_v2
    pub minfo: u16,
    pub winfo: u16,
// netsys_v3
    pub w3info: u32,
    pub amsdu: u32,
}

// software-only entry type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_foe_bridge {
    pub dest_mac: [u8; ETH_ALEN],
    pub src_mac: [u8; ETH_ALEN],
    pub vlan: u16,
    pub key_end: {},
    pub ib2: u32,
    pub l2: mtk_foe_mac_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_ipv4_tuple {
    pub src_ip: u32,
    pub dest_ip: u32,
    pub dest_port: u16,
    pub src_port: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_foe_ipv4 {
    pub orig: mtk_ipv4_tuple,
    pub ib2: u32,
    pub new: mtk_ipv4_tuple,
    pub timestamp: u16,
    pub _rsv0: [u16; 3],
    pub udf_tsid: u32,
    pub l2: mtk_foe_mac_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_foe_ipv4_dslite {
    pub ip4: mtk_ipv4_tuple,
    pub tunnel_src_ip: [u32; 4],
    pub tunnel_dest_ip: [u32; 4],
    pub flow_label: [u8; 3],
    pub priority: u8,
    pub udf_tsid: u32,
    pub ib2: u32,
    pub l2: mtk_foe_mac_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_foe_ipv6 {
    pub src_ip: [u32; 4],
    pub dest_ip: [u32; 4],
    pub protocol: u8,
    pub /: *mut *mut u8 _pad[3]; / fill with 0xa5a5a5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_foe_ipv6_6rd {
    pub src_ip: [u32; 4],
    pub dest_ip: [u32; 4],
    pub dest_port: u16,
    pub src_port: u16,
    pub tunnel_src_ip: u32,
    pub tunnel_dest_ip: u32,
    pub hdr_csum: u16,
    pub dscp: u8,
    pub ttl: u8,
    pub flag: u8,
    pub pad: u8,
    pub per_flow_6rd_id: u8,
    pub pad2: u8,
    pub ib2: u32,
    pub l2: mtk_foe_mac_info,
}

pub const MTK_FOE_ENTRY_V1_SIZE: c_int = 80;
pub const MTK_FOE_ENTRY_V2_SIZE: c_int = 96;
pub const MTK_FOE_ENTRY_V3_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_foe_entry {
    pub ib1: u32,
    pub bridge: mtk_foe_bridge,
    pub ipv4: mtk_foe_ipv4,
    pub dslite: mtk_foe_ipv4_dslite,
    pub ipv6: mtk_foe_ipv6,
    pub ipv6_6rd: mtk_foe_ipv6_6rd,
    pub data: [u32; 31],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_flow_entry {
    pub list: hlist_node,
    pub l2_node: rhash_head,
    pub l2_flows: hlist_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mib_entry {
    pub byt_cnt_l: u32,
    pub byt_cnt_h: u16,
    pub pkt_cnt_l: u32,
    pub pkt_cnt_h: u8,
    pub _rsv0: u8,
    pub _rsv1: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_foe_accounting {
    pub bytes: u64,
    pub packets: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_ppe {
    pub eth: *mut mtk_eth,
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub version: c_int,
    pub dirname: [c_char; 5],
    pub accounting: bool,
    pub foe_table: *mut c_void,
    pub foe_phys: dma_addr_t,
    pub mib_table: *mut mtk_mib_entry,
    pub mib_phys: dma_addr_t,
    pub foe_check_time: [u16; MTK_PPE_ENTRIES],
    pub foe_flow: *mut hlist_head,
    pub l2_flows: rhashtable,
    pub acct_table: *mut c_void,
}

extern "C" {
    pub fn mtk_ppe_deinit(eth: *mut mtk_eth);
}
extern "C" {
    pub fn mtk_ppe_update_mtu(ppe: *mut mtk_ppe, mtu: c_int);
}
extern "C" {
    pub fn mtk_ppe_start(ppe: *mut mtk_ppe);
}
extern "C" {
    pub fn mtk_ppe_stop(ppe: *mut mtk_ppe) -> c_int;
}
extern "C" {
    pub fn mtk_ppe_prepare_reset(ppe: *mut mtk_ppe) -> c_int;
}
extern "C" {
    pub fn __mtk_ppe_check_skb(ppe: *mut mtk_ppe, skb: *mut sk_buff, hash: u16);
}
extern "C" {
    pub fn mtk_foe_entry_commit(ppe: *mut mtk_ppe, entry: *mut mtk_flow_entry) -> c_int;
}
extern "C" {
    pub fn mtk_foe_entry_clear(ppe: *mut mtk_ppe, entry: *mut mtk_flow_entry);
}
extern "C" {
    pub fn mtk_foe_entry_idle_time(ppe: *mut mtk_ppe, entry: *mut mtk_flow_entry) -> c_int;
}
extern "C" {
    pub fn mtk_ppe_debugfs_init(ppe: *mut mtk_ppe, index: c_int) -> c_int;
}

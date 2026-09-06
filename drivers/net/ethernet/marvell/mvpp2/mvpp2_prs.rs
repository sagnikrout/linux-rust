//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/mvpp2/mvpp2_prs.h
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
//
// Header Parser definitions for Marvell PPv2 Network Controller
//
// Copyright (C) 2014 Marvell
//
// Marcin Wojtas <mw@semihalf.com>
//

// Parser constants
pub const MVPP2_PRS_TCAM_SRAM_SIZE: c_int = 256;
pub const MVPP2_PRS_TCAM_WORDS: c_int = 6;
pub const MVPP2_PRS_SRAM_WORDS: c_int = 4;
pub const MVPP2_PRS_FLOW_ID_SIZE: c_int = 64;
pub const MVPP2_PRS_FLOW_ID_MASK: c_uint = 0x3f;
pub const MVPP2_PRS_TCAM_ENTRY_INVALID: c_int = 1;

pub const MVPP2_PRS_IPV4_HEAD: c_uint = 0x40;
pub const MVPP2_PRS_IPV4_HEAD_MASK: c_uint = 0xf0;
pub const MVPP2_PRS_IPV4_MC: c_uint = 0xe0;
pub const MVPP2_PRS_IPV4_MC_MASK: c_uint = 0xf0;
pub const MVPP2_PRS_IPV4_BC_MASK: c_uint = 0xff;
pub const MVPP2_PRS_IPV4_IHL_MIN: c_uint = 0x5;
pub const MVPP2_PRS_IPV4_IHL_MAX: c_uint = 0xf;
pub const MVPP2_PRS_IPV4_IHL_MASK: c_uint = 0xf;
pub const MVPP2_PRS_IPV6_MC: c_uint = 0xff;
pub const MVPP2_PRS_IPV6_MC_MASK: c_uint = 0xff;
pub const MVPP2_PRS_IPV6_HOP_MASK: c_uint = 0xff;
pub const MVPP2_PRS_TCAM_PROTO_MASK: c_uint = 0xff;
pub const MVPP2_PRS_TCAM_PROTO_MASK_L: c_uint = 0x3f;
pub const MVPP2_PRS_DBL_VLANS_MAX: c_int = 100;

pub const MVPP2_PRS_UCAST_VAL: c_uint = 0x0;
// Tcam structure:
// - lookup ID - 4 bits
// - port ID - 1 byte
// - additional information - 1 byte
// - header data - 8 bytes
// The fields are represented by MVPP2_PRS_TCAM_DATA_REG(5)->(0).
//
pub const MVPP2_PRS_AI_BITS: c_int = 8;
pub const MVPP2_PRS_AI_MASK: c_uint = 0xff;
pub const MVPP2_PRS_PORT_MASK: c_uint = 0xff;
pub const MVPP2_PRS_LU_MASK: c_uint = 0xf;
// TCAM entries in registers are accessed using 16 data bits + 16 enable bits

pub const MVPP2_PRS_TCAM_AI_WORD: c_int = 4;

pub const MVPP2_PRS_TCAM_PORT_WORD: c_int = 4;

pub const MVPP2_PRS_TCAM_LU_WORD: c_int = 5;

pub const MVPP2_PRS_TCAM_INV_WORD: c_int = 5;
pub const MVPP2_PRS_VID_TCAM_BYTE: c_int = 2;
// TCAM range for unicast and multicast filtering. We have 25 entries per port,
// with 4 dedicated to UC filtering and the rest to multicast filtering.
// Additionnally we reserve one entry for the broadcast address, and one for
// each port's own address.
//
pub const MVPP2_PRS_MAC_UC_MC_FILT_MAX: c_int = 25;
pub const MVPP2_PRS_MAC_RANGE_SIZE: c_int = 80;
// Number of entries per port dedicated to UC and MC filtering
pub const MVPP2_PRS_MAC_UC_FILT_MAX: c_int = 4;

// There is a TCAM range reserved for VLAN filtering entries, range size is 33
// 10 VLAN ID filter entries per port
// 1 default VLAN filter entry per port
// It is assumed that there are 3 ports for filter, not including loopback port
//
pub const MVPP2_PRS_VLAN_FILT_MAX: c_int = 11;
pub const MVPP2_PRS_VLAN_FILT_RANGE_SIZE: c_int = 33;

// Tcam entries ID
pub const MVPP2_PE_DROP_ALL: c_int = 0;
pub const MVPP2_PE_FIRST_FREE_TID: c_int = 1;
// MAC filtering range

// VLAN filtering range

// Index of default vid filter for given port

// Sram structure
// The fields are represented by MVPP2_PRS_TCAM_DATA_REG(3)->(0).
//
pub const MVPP2_PRS_SRAM_RI_OFFS: c_int = 0;
pub const MVPP2_PRS_SRAM_RI_WORD: c_int = 0;
pub const MVPP2_PRS_SRAM_RI_CTRL_OFFS: c_int = 32;
pub const MVPP2_PRS_SRAM_RI_CTRL_WORD: c_int = 1;
pub const MVPP2_PRS_SRAM_RI_CTRL_BITS: c_int = 32;
pub const MVPP2_PRS_SRAM_SHIFT_OFFS: c_int = 64;
pub const MVPP2_PRS_SRAM_SHIFT_SIGN_BIT: c_int = 72;
pub const MVPP2_PRS_SRAM_SHIFT_MASK: c_uint = 0xff;
pub const MVPP2_PRS_SRAM_UDF_OFFS: c_int = 73;
pub const MVPP2_PRS_SRAM_UDF_BITS: c_int = 8;
pub const MVPP2_PRS_SRAM_UDF_MASK: c_uint = 0xff;
pub const MVPP2_PRS_SRAM_UDF_SIGN_BIT: c_int = 81;
pub const MVPP2_PRS_SRAM_UDF_TYPE_OFFS: c_int = 82;
pub const MVPP2_PRS_SRAM_UDF_TYPE_MASK: c_uint = 0x7;
pub const MVPP2_PRS_SRAM_UDF_TYPE_L3: c_int = 1;
pub const MVPP2_PRS_SRAM_UDF_TYPE_L4: c_int = 4;
pub const MVPP2_PRS_SRAM_OP_SEL_SHIFT_OFFS: c_int = 85;
pub const MVPP2_PRS_SRAM_OP_SEL_SHIFT_MASK: c_uint = 0x3;
pub const MVPP2_PRS_SRAM_OP_SEL_SHIFT_ADD: c_int = 1;
pub const MVPP2_PRS_SRAM_OP_SEL_SHIFT_IP4_ADD: c_int = 2;
pub const MVPP2_PRS_SRAM_OP_SEL_SHIFT_IP6_ADD: c_int = 3;
pub const MVPP2_PRS_SRAM_OP_SEL_UDF_OFFS: c_int = 87;
pub const MVPP2_PRS_SRAM_OP_SEL_UDF_BITS: c_int = 2;
pub const MVPP2_PRS_SRAM_OP_SEL_UDF_MASK: c_uint = 0x3;
pub const MVPP2_PRS_SRAM_OP_SEL_UDF_ADD: c_int = 0;
pub const MVPP2_PRS_SRAM_OP_SEL_UDF_IP4_ADD: c_int = 2;
pub const MVPP2_PRS_SRAM_OP_SEL_UDF_IP6_ADD: c_int = 3;
pub const MVPP2_PRS_SRAM_OP_SEL_BASE_OFFS: c_int = 89;
pub const MVPP2_PRS_SRAM_AI_OFFS: c_int = 90;
pub const MVPP2_PRS_SRAM_AI_CTRL_OFFS: c_int = 98;
pub const MVPP2_PRS_SRAM_AI_CTRL_BITS: c_int = 8;
pub const MVPP2_PRS_SRAM_AI_MASK: c_uint = 0xff;
pub const MVPP2_PRS_SRAM_NEXT_LU_OFFS: c_int = 106;
pub const MVPP2_PRS_SRAM_NEXT_LU_MASK: c_uint = 0xf;
pub const MVPP2_PRS_SRAM_LU_DONE_BIT: c_int = 110;
pub const MVPP2_PRS_SRAM_LU_GEN_BIT: c_int = 111;
// Sram result info bits assignment
pub const MVPP2_PRS_RI_MAC_ME_MASK: c_uint = 0x1;
pub const MVPP2_PRS_RI_DSA_MASK: c_uint = 0x2;

pub const MVPP2_PRS_RI_VLAN_NONE: c_uint = 0x0;

pub const MVPP2_PRS_RI_CPU_CODE_MASK: c_uint = 0x70;

pub const MVPP2_PRS_RI_L2_UCAST: c_uint = 0x0;

pub const MVPP2_PRS_RI_PPPOE_MASK: c_uint = 0x800;

pub const MVPP2_PRS_RI_L3_UN: c_uint = 0x0;

pub const MVPP2_PRS_RI_L3_UCAST: c_uint = 0x0;

pub const MVPP2_PRS_RI_IP_FRAG_MASK: c_uint = 0x20000;

pub const MVPP2_PRS_RI_UDF3_MASK: c_uint = 0x300000;

pub const MVPP2_PRS_RI_L4_PROTO_MASK: c_uint = 0x1c00000;

pub const MVPP2_PRS_RI_UDF7_MASK: c_uint = 0x60000000;

pub const MVPP2_PRS_RI_DROP_MASK: c_uint = 0x80000000;

// Sram additional info bits assignment

pub const MVPP2_PRS_SINGLE_VLAN_AI: c_int = 0;

// DSA/EDSA type

// MAC entries, shadow udf
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvpp2_prs_udf {
    MVPP2_PRS_UDF_MAC_DEF,
    MVPP2_PRS_UDF_MAC_RANGE,
    MVPP2_PRS_UDF_L2_DEF,
    MVPP2_PRS_UDF_L2_DEF_COPY,
    MVPP2_PRS_UDF_L2_USER,
}

// Lookup ID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvpp2_prs_lookup {
    MVPP2_PRS_LU_MH,
    MVPP2_PRS_LU_MAC,
    MVPP2_PRS_LU_DSA,
    MVPP2_PRS_LU_VLAN,
    MVPP2_PRS_LU_VID,
    MVPP2_PRS_LU_L2,
    MVPP2_PRS_LU_PPPOE,
    MVPP2_PRS_LU_IP4,
    MVPP2_PRS_LU_IP6,
    MVPP2_PRS_LU_FLOWS,
    MVPP2_PRS_LU_LAST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_prs_entry {
    pub index: u32,
    pub tcam: [u32; MVPP2_PRS_TCAM_WORDS],
    pub sram: [u32; MVPP2_PRS_SRAM_WORDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_prs_result_info {
    pub ri: u32,
    pub ri_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_prs_shadow {
    pub valid: bool,
    pub finish: bool,
// Lookup ID
    pub lu: c_int,
// User defined offset
    pub udf: c_int,
// Result info
    pub ri: u32,
    pub ri_mask: u32,
}

extern "C" {
    pub fn mvpp2_prs_default_init(pdev: *mut platform_device, priv: *mut mvpp2) -> c_int;
}
extern "C" {
    pub fn mvpp2_prs_tcam_port_map_get(pe: *mut mvpp2_prs_entry) -> c_uint;
}
extern "C" {
    pub fn mvpp2_prs_mac_da_accept(port: *mut mvpp2_port, da: *const u8, add: bool) -> c_int;
}
extern "C" {
    pub fn mvpp2_prs_tag_mode_set(priv: *mut mvpp2, port: c_int, type: c_int) -> c_int;
}
extern "C" {
    pub fn mvpp2_prs_add_flow(priv: *mut mvpp2, flow: c_int, ri: u32, ri_mask: u32) -> c_int;
}
extern "C" {
    pub fn mvpp2_prs_def_flow(port: *mut mvpp2_port) -> c_int;
}
extern "C" {
    pub fn mvpp2_prs_vid_enable_filtering(port: *mut mvpp2_port);
}
extern "C" {
    pub fn mvpp2_prs_vid_disable_filtering(port: *mut mvpp2_port);
}
extern "C" {
    pub fn mvpp2_prs_vid_entry_add(port: *mut mvpp2_port, vid: u16) -> c_int;
}
extern "C" {
    pub fn mvpp2_prs_vid_entry_remove(port: *mut mvpp2_port, vid: u16);
}
extern "C" {
    pub fn mvpp2_prs_vid_remove_all(port: *mut mvpp2_port);
}
extern "C" {
    pub fn mvpp2_prs_mac_del_all(port: *mut mvpp2_port);
}
extern "C" {
    pub fn mvpp2_prs_update_mac_da(dev: *mut net_device, da: *const u8) -> c_int;
}
extern "C" {
    pub fn mvpp2_prs_hits(priv: *mut mvpp2, index: c_int) -> c_int;
}

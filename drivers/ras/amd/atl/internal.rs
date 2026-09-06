//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ras/amd/atl/internal.h
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
// AMD Address Translation Library
//
// internal.h : Helper functions and common defines
//
// Copyright (c) 2023, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Author: Yazen Ghannam <Yazen.Ghannam@amd.com>
//

// Maximum possible number of Coherent Stations within a single Data Fabric.
pub const MAX_COH_ST_CHANNELS: c_int = 32;
// PCI ID for Zen4 Server DF Function 0.
pub const DF_FUNC0_ID_ZEN4_SERVER: c_uint = 0x14AD1022;
// PCI IDs for MI300 DF Function 0.
pub const DF_FUNC0_ID_MI300: c_uint = 0x15281022;
// Shift needed for adjusting register values to true values.
pub const DF_DRAM_BASE_LIMIT_LSB: c_int = 28;
pub const MI300_DRAM_LIMIT_LSB: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum df_revisions {
    UNKNOWN,
    DF2,
    DF3,
    DF3p5,
    DF4,
    DF4p5,
}

// These are mapped 1:1 to the hardware values. Special cases are set at > 0x20.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intlv_modes {
    NONE				= 0x00,
    NOHASH_2CHAN			= 0x01,
    NOHASH_4CHAN			= 0x03,
    NOHASH_8CHAN			= 0x05,
    DF3_6CHAN			= 0x06,
    NOHASH_16CHAN			= 0x07,
    NOHASH_32CHAN			= 0x08,
    DF3_COD4_2CHAN_HASH		= 0x0C,
    DF3_COD2_4CHAN_HASH		= 0x0D,
    DF3_COD1_8CHAN_HASH		= 0x0E,
    DF4_NPS4_2CHAN_HASH		= 0x10,
    DF4_NPS2_4CHAN_HASH		= 0x11,
    DF4_NPS1_8CHAN_HASH		= 0x12,
    DF4_NPS4_3CHAN_HASH		= 0x13,
    DF4_NPS2_6CHAN_HASH		= 0x14,
    DF4_NPS1_12CHAN_HASH		= 0x15,
    DF4_NPS2_5CHAN_HASH		= 0x16,
    DF4_NPS1_10CHAN_HASH		= 0x17,
    MI3_HASH_8CHAN			= 0x18,
    MI3_HASH_16CHAN			= 0x19,
    MI3_HASH_32CHAN			= 0x1A,
    DF2_2CHAN_HASH			= 0x21,
// DF4.5 modes are all IntLvNumChan + 0x20
    DF4p5_NPS1_16CHAN_1K_HASH	= 0x2C,
    DF4p5_NPS0_24CHAN_1K_HASH	= 0x2E,
    DF4p5_NPS4_2CHAN_1K_HASH	= 0x30,
    DF4p5_NPS2_4CHAN_1K_HASH	= 0x31,
    DF4p5_NPS1_8CHAN_1K_HASH	= 0x32,
    DF4p5_NPS4_3CHAN_1K_HASH	= 0x33,
    DF4p5_NPS2_6CHAN_1K_HASH	= 0x34,
    DF4p5_NPS1_12CHAN_1K_HASH	= 0x35,
    DF4p5_NPS2_5CHAN_1K_HASH	= 0x36,
    DF4p5_NPS1_10CHAN_1K_HASH	= 0x37,
    DF4p5_NPS4_2CHAN_2K_HASH	= 0x40,
    DF4p5_NPS2_4CHAN_2K_HASH	= 0x41,
    DF4p5_NPS1_8CHAN_2K_HASH	= 0x42,
    DF4p5_NPS1_16CHAN_2K_HASH	= 0x43,
    DF4p5_NPS4_3CHAN_2K_HASH	= 0x44,
    DF4p5_NPS2_6CHAN_2K_HASH	= 0x45,
    DF4p5_NPS1_12CHAN_2K_HASH	= 0x46,
    DF4p5_NPS0_24CHAN_2K_HASH	= 0x47,
    DF4p5_NPS2_5CHAN_2K_HASH	= 0x48,
    DF4p5_NPS1_10CHAN_2K_HASH	= 0x49,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct df4p5_denorm_ctx {
// Indicates the number of "lost" bits. This will be 1, 2, or 3.
    pub perm_shift: u8,
// A mask indicating the bits that need to be rehashed.
    pub rehash_vector: u16,
//
// Represents the value that the high bits of the normalized address
// are divided by during normalization. This value will be 3 for
// interleave modes with a number of channels divisible by 3 or the
// value will be 5 for interleave modes with a number of channels
// divisible by 5. Power-of-two interleave modes are handled
// separately.
//
    pub mod_value: u8,
//
// Represents the bits that can be directly pulled from the normalized
// address. In each case, pass through bits [7:0] of the normalized
// address. The other bits depend on the interleave bit position which
// will be bit 10 for 1K interleave stripe cases and bit 11 for 2K
// interleave stripe cases.
//
    pub base_denorm_addr: u64,
//
// Represents the high bits of the physical address that have been
// divided by the mod_value.
//
    pub div_addr: u64,
    pub current_spa: u64,
    pub resolved_spa: u64,
    pub coh_st_fabric_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct df_flags {
    pub 4: __reserved_0 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct df_config {
    pub rev: df_revisions,
//
// These masks operate on the 16-bit Coherent Station IDs,
// e.g. Instance, Fabric, Destination, etc.
//
    pub component_id_mask: u16,
    pub die_id_mask: u16,
    pub node_id_mask: u16,
    pub socket_id_mask: u16,
//
// Least-significant bit of Node ID portion of the
// system-wide Coherent Station Fabric ID.
//
    pub node_id_shift: u8,
//
// Least-significant bit of Die portion of the Node ID.
// Adjusted to include the Node ID shift in order to apply
// to the Coherent Station Fabric ID.
//
    pub die_id_shift: u8,
//
// Least-significant bit of Socket portion of the Node ID.
// Adjusted to include the Node ID shift in order to apply
// to the Coherent Station Fabric ID.
//
    pub socket_id_shift: u8,
// Number of DRAM Address maps visible in a Coherent Station.
    pub num_coh_st_maps: u8,
    pub dram_hole_base: u32,
// Global flags to handle special cases.
    pub flags: df_flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dram_addr_map {
//
// Each DRAM Address Map can operate independently
// in different interleaving modes.
//
    pub intlv_mode: intlv_modes,
// System-wide number for this address map.
    pub num: u8,
// Raw register values
    pub base: u32,
    pub limit: u32,
    pub ctl: u32,
    pub intlv: u32,
//
// Logical to Physical Coherent Station Remapping array
//
// Index: Logical Coherent Station Instance ID
// Value: Physical Coherent Station Instance ID
//
// phys_coh_st_inst_id = remap_array[log_coh_st_inst_id]
//
    pub remap_array: [u8; MAX_COH_ST_CHANNELS],
//
// Number of bits covering DRAM Address map 0
// when interleaving is non-power-of-2.
//
// Used only for DF3_6CHAN.
//
    pub np2_bits: u8,
// Position of the 'interleave bit'.
    pub intlv_bit_pos: u8,
// Number of channels interleaved in this map.
    pub num_intlv_chan: u8,
// Number of dies interleaved in this map.
    pub num_intlv_dies: u8,
// Number of sockets interleaved in this map.
    pub num_intlv_sockets: u8,
//
// Total number of channels interleaved accounting
// for die and socket interleaving.
//
    pub total_intlv_chan: u8,
// Total bits needed to cover 'total_intlv_chan'.
    pub total_intlv_bits: u8,
}

// Original input values cached for debug printing.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_ctx_inputs {
    pub norm_addr: u64,
    pub socket_id: u8,
    pub die_id: u8,
    pub coh_st_inst_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_ctx {
    pub ret_addr: u64,
    pub inputs: addr_ctx_inputs,
    pub map: dram_addr_map,
// AMD Node ID calculated from Socket and Die IDs.
    pub node_id: u8,
//
// Coherent Station Instance ID
// Local ID used within a 'node'.
//
    pub inst_id: u16,
//
// Coherent Station Fabric ID
// System-wide ID that includes 'node' bits.
//
    pub coh_st_fabric_id: u16,
}

extern "C" {
    pub fn df_indirect_read_instance(node: u16, func: u8, reg: u16, instance_id: u8, lo: *mut u32) -> c_int;
}
extern "C" {
    pub fn df_indirect_read_broadcast(node: u16, func: u8, reg: u16, lo: *mut u32) -> c_int;
}
extern "C" {
    pub fn get_df_system_info() -> c_int;
}
extern "C" {
    pub fn determine_node_id(ctx: *mut addr_ctx, socket_num: u8, die_num: u8) -> c_int;
}
extern "C" {
    pub fn get_umc_info_mi300() -> c_int;
}
extern "C" {
    pub fn get_address_map(ctx: *mut addr_ctx) -> c_int;
}
extern "C" {
    pub fn denormalize_address(ctx: *mut addr_ctx) -> c_int;
}
extern "C" {
    pub fn dehash_address(ctx: *mut addr_ctx) -> c_int;
}
extern "C" {
    pub fn norm_to_sys_addr(socket_id: u8, die_id: u8, coh_st_inst_id: u8, addr: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn convert_umc_mca_addr_to_sys_addr(err: *mut atl_err) -> c_ulong;
}
extern "C" {
    pub fn add_base_and_hole(ctx: *mut addr_ctx, addr: u64) -> u64;
}
extern "C" {
    pub fn remove_base_and_hole(ctx: *mut addr_ctx, addr: u64) -> u64;
}
// GUIDs for PRM handlers

extern "C" {
    pub fn prm_umc_norm_to_sys_addr(socket_id: u8, umc_bank_inst_id: u64, addr: c_ulong) -> c_ulong;
}

//
// Make a gap in @data that is @num_bits long starting at @bit_num.
// e.g. data		= 11111111'b
// bit_num		= 3
// num_bits	= 2
// result		= 1111100111'b
//
// Remove bits in @data between @low_bit and @high_bit inclusive.
// e.g. data		= XXXYYZZZ'b
// low_bit		= 3
// high_bit	= 4
// result		= XXXZZZ'b
//


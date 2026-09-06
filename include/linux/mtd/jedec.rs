//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/jedec.h
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
// Copyright © 2000-2010 David Woodhouse <dwmw2@infradead.org>
// Steven J. Hill <sjhill@realitydiluted.com>
// Thomas Gleixner <tglx@kernel.org>
//
// Contains all JEDEC related definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jedec_ecc_info {
    pub ecc_bits: u8,
    pub codeword_size: u8,
    pub bb_per_lun: __le16,
    pub block_endurance: __le16,
    pub reserved: [u8; 2],
    pub __packed: },
// JEDEC features

// JEDEC Optional Commands

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_jedec_params {
// rev info and features block
// 'J' 'E' 'S' 'D'
    pub sig: [u8; 4],
    pub revision: __le16,
    pub features: __le16,
    pub opt_cmd: [u8; 3],
    pub sec_cmd: __le16,
    pub num_of_param_pages: u8,
    pub reserved0: [u8; 18],
// manufacturer information block
    pub manufacturer: [c_char; 12],
    pub model: [c_char; 20],
    pub jedec_id: [u8; 6],
    pub reserved1: [u8; 10],
// memory organization block
    pub byte_per_page: __le32,
    pub spare_bytes_per_page: __le16,
    pub reserved2: [u8; 6],
    pub pages_per_block: __le32,
    pub blocks_per_lun: __le32,
    pub lun_count: u8,
    pub addr_cycles: u8,
    pub bits_per_cell: u8,
    pub programs_per_page: u8,
    pub multi_plane_addr: u8,
    pub multi_plane_op_attr: u8,
    pub reserved3: [u8; 38],
// electrical parameter block
    pub async_sdr_speed_grade: __le16,
    pub toggle_ddr_speed_grade: __le16,
    pub sync_ddr_speed_grade: __le16,
    pub async_sdr_features: u8,
    pub toggle_ddr_features: u8,
    pub sync_ddr_features: u8,
    pub t_prog: __le16,
    pub t_bers: __le16,
    pub t_r: __le16,
    pub t_r_multi_plane: __le16,
    pub t_ccs: __le16,
    pub io_pin_capacitance_typ: __le16,
    pub input_pin_capacitance_typ: __le16,
    pub clk_pin_capacitance_typ: __le16,
    pub driver_strength_support: u8,
    pub t_adl: __le16,
    pub reserved4: [u8; 36],
// ECC and endurance block
    pub guaranteed_good_blocks: u8,
    pub guaranteed_block_endurance: __le16,
    pub ecc_info: [jedec_ecc_info; 4],
    pub reserved5: [u8; 29],
// reserved
    pub reserved6: [u8; 148],
// vendor
    pub vendor_rev_num: __le16,
    pub reserved7: [u8; 88],
// CRC for Parameter Page
    pub crc: __le16,
    pub __packed: },

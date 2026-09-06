//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/edac/pnd2_edac.h
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
// Register bitfield descriptions for Pondicherry2 memory controller.
//
// Copyright (c) 2016, Intel Corporation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b_cr_touud_lo_pci {
    pub 1: u32 lock :,
    pub 19: u32 reserved_1 :,
    pub 12: u32 touud :,
}

pub const b_cr_touud_lo_pci_port: c_uint = 0x4c;
pub const b_cr_touud_lo_pci_offset: c_uint = 0xa8;
pub const b_cr_touud_lo_pci_r_opcode: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b_cr_touud_hi_pci {
    pub 7: u32 touud :,
    pub 25: u32 reserved_0 :,
}

pub const b_cr_touud_hi_pci_port: c_uint = 0x4c;
pub const b_cr_touud_hi_pci_offset: c_uint = 0xac;
pub const b_cr_touud_hi_pci_r_opcode: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b_cr_tolud_pci {
    pub 1: u32 lock :,
    pub 19: u32 reserved_0 :,
    pub 12: u32 tolud :,
}

pub const b_cr_tolud_pci_port: c_uint = 0x4c;
pub const b_cr_tolud_pci_offset: c_uint = 0xbc;
pub const b_cr_tolud_pci_r_opcode: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b_cr_mchbar_lo_pci {
    pub 1: u32 enable :,
    pub 3: u32 pad_3_1 :,
    pub 11: u32 pad_14_4:,
    pub 17: u32 base:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b_cr_mchbar_hi_pci {
    pub 7: u32 base :,
    pub 25: u32 pad_31_7 :,
}

// Symmetric region
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b_cr_slice_channel_hash {
    pub 1: u64 slice_1_disabled :,
    pub 1: u64 hvm_mode :,
    pub 2: u64 interleave_mode :,
    pub 1: u64 slice_0_mem_disabled :,
    pub 1: u64 reserved_0 :,
    pub 14: u64 slice_hash_mask :,
    pub 11: u64 reserved_1 :,
    pub 1: u64 enable_pmi_dual_data_mode :,
    pub 1: u64 ch_1_disabled :,
    pub 1: u64 reserved_2 :,
    pub 2: u64 sym_slice0_channel_enabled :,
    pub 2: u64 sym_slice1_channel_enabled :,
    pub 14: u64 ch_hash_mask :,
    pub 11: u64 reserved_3 :,
    pub 1: u64 lock :,
}

pub const b_cr_slice_channel_hash_port: c_uint = 0x4c;
pub const b_cr_slice_channel_hash_offset: c_uint = 0x4c58;
pub const b_cr_slice_channel_hash_r_opcode: c_uint = 0x06;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b_cr_mot_out_base_mchbar {
    pub 14: u32 reserved_0 :,
    pub 15: u32 mot_out_base :,
    pub 1: u32 reserved_1 :,
    pub 1: u32 tr_en :,
    pub 1: u32 imr_en :,
}

pub const b_cr_mot_out_base_mchbar_port: c_uint = 0x4c;
pub const b_cr_mot_out_base_mchbar_offset: c_uint = 0x6af0;
pub const b_cr_mot_out_base_mchbar_r_opcode: c_uint = 0x00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b_cr_mot_out_mask_mchbar {
    pub 14: u32 reserved_0 :,
    pub 15: u32 mot_out_mask :,
    pub 1: u32 reserved_1 :,
    pub 1: u32 ia_iwb_en :,
    pub 1: u32 gt_iwb_en :,
}

pub const b_cr_mot_out_mask_mchbar_port: c_uint = 0x4c;
pub const b_cr_mot_out_mask_mchbar_offset: c_uint = 0x6af4;
pub const b_cr_mot_out_mask_mchbar_r_opcode: c_uint = 0x00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b_cr_asym_mem_region0_mchbar {
    pub 4: u32 pad :,
    pub 11: u32 slice0_asym_base :,
    pub 4: u32 pad_18_15 :,
    pub 11: u32 slice0_asym_limit :,
    pub 1: u32 slice0_asym_channel_select :,
    pub 1: u32 slice0_asym_enable :,
}

pub const b_cr_asym_mem_region0_mchbar_port: c_uint = 0x4c;
pub const b_cr_asym_mem_region0_mchbar_offset: c_uint = 0x6e40;
pub const b_cr_asym_mem_region0_mchbar_r_opcode: c_uint = 0x00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b_cr_asym_mem_region1_mchbar {
    pub 4: u32 pad :,
    pub 11: u32 slice1_asym_base :,
    pub 4: u32 pad_18_15 :,
    pub 11: u32 slice1_asym_limit :,
    pub 1: u32 slice1_asym_channel_select :,
    pub 1: u32 slice1_asym_enable :,
}

pub const b_cr_asym_mem_region1_mchbar_port: c_uint = 0x4c;
pub const b_cr_asym_mem_region1_mchbar_offset: c_uint = 0x6e44;
pub const b_cr_asym_mem_region1_mchbar_r_opcode: c_uint = 0x00;
// Some bit fields moved in above two structs on Denverton
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b_cr_asym_mem_region_denverton {
    pub 4: u32 pad :,
    pub 8: u32 slice_asym_base :,
    pub 8: u32 pad_19_12 :,
    pub 8: u32 slice_asym_limit :,
    pub 3: u32 pad_28_30 :,
    pub 1: u32 slice_asym_enable :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b_cr_asym_2way_mem_region_mchbar {
    pub 2: u32 pad :,
    pub 2: u32 asym_2way_intlv_mode :,
    pub 11: u32 asym_2way_base :,
    pub 2: u32 pad_16_15 :,
    pub 11: u32 asym_2way_limit :,
    pub 3: u32 pad_30_28 :,
    pub 1: u32 asym_2way_interleave_enable :,
}

pub const b_cr_asym_2way_mem_region_mchbar_port: c_uint = 0x4c;
pub const b_cr_asym_2way_mem_region_mchbar_offset: c_uint = 0x6e50;
pub const b_cr_asym_2way_mem_region_mchbar_r_opcode: c_uint = 0x00;
// Apollo Lake d-unit
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d_cr_drp0 {
    pub 1: u32 rken0 :,
    pub 1: u32 rken1 :,
    pub 1: u32 ddmen :,
    pub 1: u32 rsvd3 :,
    pub 2: u32 dwid :,
    pub 3: u32 dden :,
    pub 5: u32 rsvd13_9 :,
    pub 1: u32 rsien :,
    pub 1: u32 bahen :,
    pub 3: u32 rsvd18_16 :,
    pub 2: u32 caswizzle :,
    pub 1: u32 eccen :,
    pub 3: u32 dramtype :,
    pub 3: u32 blmode :,
    pub 2: u32 addrdec :,
    pub 2: u32 dramdevice_pr :,
}

pub const d_cr_drp0_offset: c_uint = 0x1400;
pub const d_cr_drp0_r_opcode: c_uint = 0x00;
// Denverton d-unit
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d_cr_dsch {
    pub 1: u32 ch0en :,
    pub 1: u32 ch1en :,
    pub 1: u32 ddr4en :,
    pub 1: u32 coldwake :,
    pub 1: u32 newbypdis :,
    pub 1: u32 chan_width :,
    pub 1: u32 rsvd6_6 :,
    pub 1: u32 ooodis :,
    pub 11: u32 rsvd18_8 :,
    pub 1: u32 ic :,
    pub 12: u32 rsvd31_20 :,
}

pub const d_cr_dsch_port: c_uint = 0x16;
pub const d_cr_dsch_offset: c_uint = 0x0;
pub const d_cr_dsch_r_opcode: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d_cr_ecc_ctrl {
    pub 1: u32 eccen :,
    pub 31: u32 rsvd31_1 :,
}

pub const d_cr_ecc_ctrl_offset: c_uint = 0x180;
pub const d_cr_ecc_ctrl_r_opcode: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d_cr_drp {
    pub 1: u32 rken0 :,
    pub 1: u32 rken1 :,
    pub 1: u32 rken2 :,
    pub 1: u32 rken3 :,
    pub 2: u32 dimmdwid0 :,
    pub 2: u32 dimmdden0 :,
    pub 2: u32 dimmdwid1 :,
    pub 2: u32 dimmdden1 :,
    pub 4: u32 rsvd15_12 :,
    pub 1: u32 dimmflip :,
    pub 15: u32 rsvd31_17 :,
}

pub const d_cr_drp_offset: c_uint = 0x158;
pub const d_cr_drp_r_opcode: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d_cr_dmap {
    pub 5: u32 ba0 :,
    pub 5: u32 ba1 :,
    pub /: *mut *mut u32 bg0 : 5; / if ddr3, ba2 = bg0,
    pub /: *mut *mut u32 bg1 : 5; / if ddr3, ba3 = bg1,
    pub 5: u32 rs0 :,
    pub 5: u32 rs1 :,
    pub 2: u32 rsvd :,
}

pub const d_cr_dmap_offset: c_uint = 0x174;
pub const d_cr_dmap_r_opcode: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d_cr_dmap1 {
    pub 6: u32 ca11 :,
    pub 1: u32 bxor :,
    pub 25: u32 rsvd :,
}

pub const d_cr_dmap1_offset: c_uint = 0xb4;
pub const d_cr_dmap1_r_opcode: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d_cr_dmap2 {
    pub 5: u32 row0 :,
    pub 5: u32 row1 :,
    pub 5: u32 row2 :,
    pub 5: u32 row3 :,
    pub 5: u32 row4 :,
    pub 5: u32 row5 :,
    pub 2: u32 rsvd :,
}

pub const d_cr_dmap2_offset: c_uint = 0x148;
pub const d_cr_dmap2_r_opcode: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d_cr_dmap3 {
    pub 5: u32 row6 :,
    pub 5: u32 row7 :,
    pub 5: u32 row8 :,
    pub 5: u32 row9 :,
    pub 5: u32 row10 :,
    pub 5: u32 row11 :,
    pub 2: u32 rsvd :,
}

pub const d_cr_dmap3_offset: c_uint = 0x14c;
pub const d_cr_dmap3_r_opcode: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d_cr_dmap4 {
    pub 5: u32 row12 :,
    pub 5: u32 row13 :,
    pub 5: u32 row14 :,
    pub 5: u32 row15 :,
    pub 5: u32 row16 :,
    pub 5: u32 row17 :,
    pub 2: u32 rsvd :,
}

pub const d_cr_dmap4_offset: c_uint = 0x150;
pub const d_cr_dmap4_r_opcode: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct d_cr_dmap5 {
    pub 4: u32 ca3 :,
    pub 4: u32 ca4 :,
    pub 4: u32 ca5 :,
    pub 4: u32 ca6 :,
    pub 4: u32 ca7 :,
    pub 4: u32 ca8 :,
    pub 4: u32 ca9 :,
    pub 4: u32 rsvd :,
}

pub const d_cr_dmap5_offset: c_uint = 0x154;
pub const d_cr_dmap5_r_opcode: c_uint = 0x0;

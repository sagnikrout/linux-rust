//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/bios/bios_parser_types_internal2.h
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


//
// Copyright 2012-15 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

// use atomfirmware_bringup.h only. Not atombios.h anymore
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_data_revision {
    pub major: u32,
    pub minor: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct object_info_table {
    pub revision: atom_data_revision,
    pub v1_4: *mut display_object_info_table_v1_4,
    pub v1_5: *mut display_object_info_table_v1_5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spread_spectrum_id {
    SS_ID_UNKNOWN = 0,
    SS_ID_DP1 = 0xf1,
    SS_ID_DP2 = 0xf2,
    SS_ID_LVLINK_2700MHZ = 0xf3,
    SS_ID_LVLINK_1620MHZ = 0xf4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bios_parser {
    pub base: dc_bios,
    pub object_info_tbl: object_info_table,
    pub object_info_tbl_offset: u32,
    pub master_data_tbl: *mut atom_master_data_table_v2_1,
    pub bios_helper: *const bios_parser_helper,
    pub cmd_helper: *const command_table_helper,
    pub cmd_tbl: cmd_tbl,
    pub remap_device_tags: bool,
}

// Bios Parser from DC Bios


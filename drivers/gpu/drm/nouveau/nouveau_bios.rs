//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_bios.h
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
// Copyright 2007-2008 Nouveau Project
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//
pub const DCB_MAX_NUM_ENTRIES: c_int = 16;
pub const DCB_MAX_NUM_I2C_ENTRIES: c_int = 16;
pub const DCB_MAX_NUM_GPIO_ENTRIES: c_int = 32;
pub const DCB_MAX_NUM_CONNECTOR_ENTRIES: c_int = 16;
pub const DCB_LOC_ON_CHIP: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bit_entry {
    pub id: u8,
    pub version: u8,
    pub length: u16,
    pub offset: u16,
    pub data: *mut u8,
}

extern "C" {
    pub fn bit_table(: *mut drm_device, id: u8, : *mut bit_entry) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcb_table {
    pub version: u8,
    pub entries: c_int,
    pub entry: [dcb_output; DCB_MAX_NUM_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nouveau_or {
    DCB_OUTPUT_A = (1 << 0),
    DCB_OUTPUT_B = (1 << 1),
    DCB_OUTPUT_C = (1 << 2)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LVDS_script {
// Order *does* matter here
    LVDS_INIT = 1,
    LVDS_RESET,
    LVDS_BACKLIGHT_ON,
    LVDS_BACKLIGHT_OFF,
    LVDS_PANEL_ON,
    LVDS_PANEL_OFF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvbios {
    pub dev: *mut drm_device,
    pub type: },
    pub offset: u16,
    pub length: u32,
    pub data: *mut u8,
    pub chip_version: u8,
    pub dactestval: u32,
    pub tvdactestval: u32,
    pub digital_min_front_porch: u8,
    pub fp_no_ddc: bool,
    pub lock: spinlock_t,
    pub execute: bool,
    pub major_version: u8,
    pub feature_byte: u8,
    pub is_mobile: bool,
    pub fminvco: uint32_t fmaxvco,,
    pub old_style_init: bool,
    pub init_script_tbls_ptr: u16,
    pub extra_init_script_tbl_ptr: u16,
    pub ram_restrict_tbl_ptr: u16,
    pub ram_restrict_group_count: u8,
    pub dcb: dcb_table,
    pub crtchead: c_int,
    pub state: },
    pub /: *mut *mut uint16_t fptablepointer; / also used by tmds,
    pub fpxlatetableptr: u16,
    pub xlatwidth: c_int,
    pub lvdsmanufacturerpointer: u16,
    pub fpxlatemanufacturertableptr: u16,
    pub mode_ptr: u16,
    pub xlated_entry: u16,
    pub power_off_for_reset: bool,
    pub reset_after_pclk_change: bool,
    pub dual_link: bool,
    pub link_c_increment: bool,
    pub if_is_24bit: bool,
    pub duallink_transition_clk: c_int,
    pub strapless_is_24bit: u8,
    pub edid: *mut u8,
// will need resetting after suspend
    pub last_script_invoc: c_int,
    pub lvds_init_run: bool,
    pub fp: },
    pub output0_script_ptr: u16,
    pub output1_script_ptr: u16,
    pub tmds: },
    pub mem_init_tbl_ptr: u16,
    pub sdr_seq_tbl_ptr: u16,
    pub ddr_seq_tbl_ptr: u16,
    pub panel: uint8_t crt, tv,,
    pub i2c_indices: },
    pub lvds_single_a_script_ptr: u16,
    pub legacy: },
}

extern "C" {
    pub fn nouveau_bios_init(: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn nouveau_bios_takedown(dev: *mut drm_device);
}
extern "C" {
    pub fn nouveau_run_vbios_init(: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn nouveau_bios_fp_mode(: *mut drm_device, : *mut drm_display_mode) -> bool;
}

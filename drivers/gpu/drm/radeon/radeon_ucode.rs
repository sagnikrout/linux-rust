//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/radeon_ucode.h
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
// Copyright 2012 Advanced Micro Devices, Inc.
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
// CP
pub const R600_PFP_UCODE_SIZE: c_int = 576;
pub const R600_PM4_UCODE_SIZE: c_int = 1792;
pub const R700_PFP_UCODE_SIZE: c_int = 848;
pub const R700_PM4_UCODE_SIZE: c_int = 1360;
pub const EVERGREEN_PFP_UCODE_SIZE: c_int = 1120;
pub const EVERGREEN_PM4_UCODE_SIZE: c_int = 1376;
pub const CAYMAN_PFP_UCODE_SIZE: c_int = 2176;
pub const CAYMAN_PM4_UCODE_SIZE: c_int = 2176;
pub const SI_PFP_UCODE_SIZE: c_int = 2144;
pub const SI_PM4_UCODE_SIZE: c_int = 2144;
pub const SI_CE_UCODE_SIZE: c_int = 2144;
pub const CIK_PFP_UCODE_SIZE: c_int = 2144;
pub const CIK_ME_UCODE_SIZE: c_int = 2144;
pub const CIK_CE_UCODE_SIZE: c_int = 2144;
// MEC
pub const CIK_MEC_UCODE_SIZE: c_int = 4192;
// RLC
pub const R600_RLC_UCODE_SIZE: c_int = 768;
pub const R700_RLC_UCODE_SIZE: c_int = 1024;
pub const EVERGREEN_RLC_UCODE_SIZE: c_int = 768;
pub const CAYMAN_RLC_UCODE_SIZE: c_int = 1024;
pub const ARUBA_RLC_UCODE_SIZE: c_int = 1536;
pub const SI_RLC_UCODE_SIZE: c_int = 2048;
pub const BONAIRE_RLC_UCODE_SIZE: c_int = 2048;
pub const KB_RLC_UCODE_SIZE: c_int = 2560;
pub const KV_RLC_UCODE_SIZE: c_int = 2560;
pub const ML_RLC_UCODE_SIZE: c_int = 2560;
// MC
pub const BTC_MC_UCODE_SIZE: c_int = 6024;
pub const CAYMAN_MC_UCODE_SIZE: c_int = 6037;
pub const SI_MC_UCODE_SIZE: c_int = 7769;
pub const TAHITI_MC_UCODE_SIZE: c_int = 7808;
pub const PITCAIRN_MC_UCODE_SIZE: c_int = 7775;
pub const VERDE_MC_UCODE_SIZE: c_int = 7875;
pub const OLAND_MC_UCODE_SIZE: c_int = 7863;
pub const BONAIRE_MC_UCODE_SIZE: c_int = 7866;
pub const BONAIRE_MC2_UCODE_SIZE: c_int = 7948;
pub const HAWAII_MC_UCODE_SIZE: c_int = 7933;
pub const HAWAII_MC2_UCODE_SIZE: c_int = 8091;
// SDMA
pub const CIK_SDMA_UCODE_SIZE: c_int = 1050;
pub const CIK_SDMA_UCODE_VERSION: c_int = 64;
// SMC
pub const RV770_SMC_UCODE_START: c_uint = 0x0100;
pub const RV770_SMC_UCODE_SIZE: c_uint = 0x410d;
pub const RV770_SMC_INT_VECTOR_START: c_uint = 0xffc0;
pub const RV770_SMC_INT_VECTOR_SIZE: c_uint = 0x0040;
pub const RV730_SMC_UCODE_START: c_uint = 0x0100;
pub const RV730_SMC_UCODE_SIZE: c_uint = 0x412c;
pub const RV730_SMC_INT_VECTOR_START: c_uint = 0xffc0;
pub const RV730_SMC_INT_VECTOR_SIZE: c_uint = 0x0040;
pub const RV710_SMC_UCODE_START: c_uint = 0x0100;
pub const RV710_SMC_UCODE_SIZE: c_uint = 0x3f1f;
pub const RV710_SMC_INT_VECTOR_START: c_uint = 0xffc0;
pub const RV710_SMC_INT_VECTOR_SIZE: c_uint = 0x0040;
pub const RV740_SMC_UCODE_START: c_uint = 0x0100;
pub const RV740_SMC_UCODE_SIZE: c_uint = 0x41c5;
pub const RV740_SMC_INT_VECTOR_START: c_uint = 0xffc0;
pub const RV740_SMC_INT_VECTOR_SIZE: c_uint = 0x0040;
pub const CEDAR_SMC_UCODE_START: c_uint = 0x0100;
pub const CEDAR_SMC_UCODE_SIZE: c_uint = 0x5d50;
pub const CEDAR_SMC_INT_VECTOR_START: c_uint = 0xffc0;
pub const CEDAR_SMC_INT_VECTOR_SIZE: c_uint = 0x0040;
pub const REDWOOD_SMC_UCODE_START: c_uint = 0x0100;
pub const REDWOOD_SMC_UCODE_SIZE: c_uint = 0x5f0a;
pub const REDWOOD_SMC_INT_VECTOR_START: c_uint = 0xffc0;
pub const REDWOOD_SMC_INT_VECTOR_SIZE: c_uint = 0x0040;
pub const JUNIPER_SMC_UCODE_START: c_uint = 0x0100;
pub const JUNIPER_SMC_UCODE_SIZE: c_uint = 0x5f1f;
pub const JUNIPER_SMC_INT_VECTOR_START: c_uint = 0xffc0;
pub const JUNIPER_SMC_INT_VECTOR_SIZE: c_uint = 0x0040;
pub const CYPRESS_SMC_UCODE_START: c_uint = 0x0100;
pub const CYPRESS_SMC_UCODE_SIZE: c_uint = 0x61f7;
pub const CYPRESS_SMC_INT_VECTOR_START: c_uint = 0xffc0;
pub const CYPRESS_SMC_INT_VECTOR_SIZE: c_uint = 0x0040;
pub const BARTS_SMC_UCODE_START: c_uint = 0x0100;
pub const BARTS_SMC_UCODE_SIZE: c_uint = 0x6107;
pub const BARTS_SMC_INT_VECTOR_START: c_uint = 0xffc0;
pub const BARTS_SMC_INT_VECTOR_SIZE: c_uint = 0x0040;
pub const TURKS_SMC_UCODE_START: c_uint = 0x0100;
pub const TURKS_SMC_UCODE_SIZE: c_uint = 0x605b;
pub const TURKS_SMC_INT_VECTOR_START: c_uint = 0xffc0;
pub const TURKS_SMC_INT_VECTOR_SIZE: c_uint = 0x0040;
pub const CAICOS_SMC_UCODE_START: c_uint = 0x0100;
pub const CAICOS_SMC_UCODE_SIZE: c_uint = 0x5fbd;
pub const CAICOS_SMC_INT_VECTOR_START: c_uint = 0xffc0;
pub const CAICOS_SMC_INT_VECTOR_SIZE: c_uint = 0x0040;
pub const CAYMAN_SMC_UCODE_START: c_uint = 0x0100;
pub const CAYMAN_SMC_UCODE_SIZE: c_uint = 0x79ec;
pub const CAYMAN_SMC_INT_VECTOR_START: c_uint = 0xffc0;
pub const CAYMAN_SMC_INT_VECTOR_SIZE: c_uint = 0x0040;
pub const TAHITI_SMC_UCODE_START: c_uint = 0x10000;
pub const TAHITI_SMC_UCODE_SIZE: c_uint = 0xf458;
pub const PITCAIRN_SMC_UCODE_START: c_uint = 0x10000;
pub const PITCAIRN_SMC_UCODE_SIZE: c_uint = 0xe9f4;
pub const VERDE_SMC_UCODE_START: c_uint = 0x10000;
pub const VERDE_SMC_UCODE_SIZE: c_uint = 0xebe4;
pub const OLAND_SMC_UCODE_START: c_uint = 0x10000;
pub const OLAND_SMC_UCODE_SIZE: c_uint = 0xe7b4;
pub const HAINAN_SMC_UCODE_START: c_uint = 0x10000;
pub const HAINAN_SMC_UCODE_SIZE: c_uint = 0xe67C;
pub const BONAIRE_SMC_UCODE_START: c_uint = 0x20000;
pub const BONAIRE_SMC_UCODE_SIZE: c_uint = 0x1FDEC;
pub const HAWAII_SMC_UCODE_START: c_uint = 0x20000;
pub const HAWAII_SMC_UCODE_SIZE: c_uint = 0x1FDEC;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_firmware_header {
    pub /: *mut *mut uint32_t size_bytes; / size of the entire header+image(s) in bytes,
    pub /: *mut *mut uint32_t header_size_bytes; / size of just the header in bytes,
    pub /: *mut *mut uint16_t header_version_major; / header version,
    pub /: *mut *mut uint16_t header_version_minor; / header version,
    pub /: *mut *mut uint16_t ip_version_major; / IP version,
    pub /: *mut *mut uint16_t ip_version_minor; / IP version,
    pub ucode_version: u32,
    pub /: *mut *mut uint32_t ucode_size_bytes; / size of ucode in bytes,
    pub /: *mut *mut uint32_t ucode_array_offset_bytes; / payload offset from the start of the header,
    pub /: *mut *mut uint32_t crc32; / crc32 checksum of the payload,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub /: *mut *mut uint32_t io_debug_size_bytes; / size of debug array in dwords,
    pub /: *mut *mut uint32_t io_debug_array_offset_bytes; / payload offset from the start of the header,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub ucode_start_addr: u32,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfx_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub ucode_feature_version: u32,
    pub /: *mut *mut uint32_t jt_offset; / jt location,
    pub /: *mut *mut uint32_t jt_size; / size of jt,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rlc_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub ucode_feature_version: u32,
    pub save_and_restore_offset: u32,
    pub clear_state_descriptor_offset: u32,
    pub avail_scratch_ram_locations: u32,
    pub master_pkt_description_offset: u32,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdma_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub ucode_feature_version: u32,
    pub ucode_change_version: u32,
    pub /: *mut *mut uint32_t jt_offset; / jt location,
    pub /: *mut *mut uint32_t jt_size; / size of jt,
}

// header is fixed size
#[repr(C)]
#[derive(Copy, Clone)]
pub union radeon_firmware_header {
    pub common: common_firmware_header,
    pub mc: mc_firmware_header_v1_0,
    pub smc: smc_firmware_header_v1_0,
    pub gfx: gfx_firmware_header_v1_0,
    pub rlc: rlc_firmware_header_v1_0,
    pub sdma: sdma_firmware_header_v1_0,
    pub raw: [u8; 0x100],
}

extern "C" {
    pub fn radeon_ucode_print_mc_hdr(hdr: *const common_firmware_header);
}
extern "C" {
    pub fn radeon_ucode_print_smc_hdr(hdr: *const common_firmware_header);
}
extern "C" {
    pub fn radeon_ucode_print_gfx_hdr(hdr: *const common_firmware_header);
}
extern "C" {
    pub fn radeon_ucode_print_rlc_hdr(hdr: *const common_firmware_header);
}
extern "C" {
    pub fn radeon_ucode_print_sdma_hdr(hdr: *const common_firmware_header);
}
extern "C" {
    pub fn radeon_ucode_validate(fw: *const firmware) -> c_int;
}

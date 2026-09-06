//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/cirrus/wmfw.h
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
// wmfw.h - Wolfson firmware format information
//
// Copyright 2012 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

pub const WMFW_MAX_ALG_NAME: c_int = 256;
pub const WMFW_MAX_ALG_DESCR_NAME: c_int = 256;
pub const WMFW_MAX_COEFF_NAME: c_int = 256;
pub const WMFW_MAX_COEFF_DESCR_NAME: c_int = 256;
pub const WMFW_CTL_FLAG_SYS: c_uint = 0x8000;
pub const WMFW_CTL_FLAG_VOLATILE: c_uint = 0x0004;
pub const WMFW_CTL_FLAG_WRITEABLE: c_uint = 0x0002;
pub const WMFW_CTL_FLAG_READABLE: c_uint = 0x0001;
pub const WMFW_CTL_TYPE_BYTES: c_uint = 0x0004 /* byte control */;
// Non-ALSA coefficient types start at 0x1000
pub const WMFW_CTL_TYPE_ACKED: c_uint = 0x1000 /* acked control */;
pub const WMFW_CTL_TYPE_HOSTEVENT: c_uint = 0x1001 /* event control */;
pub const WMFW_CTL_TYPE_HOST_BUFFER: c_uint = 0x1002 /* host buffer pointer */;
pub const WMFW_CTL_TYPE_FWEVENT: c_uint = 0x1004 /* firmware event control */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_header {
    pub magic: [c_char; 4],
    pub len: __le32,
    pub rev: __le16,
    pub core: u8,
    pub ver: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_footer {
    pub timestamp: __le64,
    pub checksum: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_adsp1_sizes {
    pub dm: __le32,
    pub pm: __le32,
    pub zm: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_adsp2_sizes {
    pub xm: __le32,
    pub ym: __le32,
    pub pm: __le32,
    pub zm: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_region {
    pub type: __be32,
    pub offset: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_id_hdr {
    pub core_id: __be32,
    pub core_rev: __be32,
    pub id: __be32,
    pub ver: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_v3_id_hdr {
    pub core_id: __be32,
    pub block_rev: __be32,
    pub vendor_id: __be32,
    pub id: __be32,
    pub ver: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_adsp1_id_hdr {
    pub fw: wmfw_id_hdr,
    pub zm: __be32,
    pub dm: __be32,
    pub n_algs: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_adsp2_id_hdr {
    pub fw: wmfw_id_hdr,
    pub zm: __be32,
    pub xm: __be32,
    pub ym: __be32,
    pub n_algs: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_halo_id_hdr {
    pub fw: wmfw_v3_id_hdr,
    pub xm_base: __be32,
    pub xm_size: __be32,
    pub ym_base: __be32,
    pub ym_size: __be32,
    pub n_algs: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_alg_hdr {
    pub id: __be32,
    pub ver: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_adsp1_alg_hdr {
    pub alg: wmfw_alg_hdr,
    pub zm: __be32,
    pub dm: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_adsp2_alg_hdr {
    pub alg: wmfw_alg_hdr,
    pub zm: __be32,
    pub xm: __be32,
    pub ym: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_halo_alg_hdr {
    pub alg: wmfw_alg_hdr,
    pub xm_base: __be32,
    pub xm_size: __be32,
    pub ym_base: __be32,
    pub ym_size: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_adsp_alg_data {
    pub id: __le32,
    pub name: [u8; WMFW_MAX_ALG_NAME],
    pub descr: [u8; WMFW_MAX_ALG_DESCR_NAME],
    pub ncoeff: __le32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_adsp_coeff_data {
    pub offset: __le16,
    pub type: __le16,
    pub size: __le32,
    pub hdr: },
    pub name: [u8; WMFW_MAX_COEFF_NAME],
    pub descr: [u8; WMFW_MAX_COEFF_DESCR_NAME],
    pub ctl_type: __le16,
    pub flags: __le16,
    pub len: __le32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_coeff_hdr {
    pub magic: [u8; 4],
    pub len: __le32,
    pub rev: __be32,
    pub ver: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_coeff_item {
    pub offset: __le16,
    pub type: __le16,
    pub id: __le32,
    pub ver: __le32,
    pub offset32: __le32,
    pub len: __le32,
    pub data: [u8; ],
    pub __packed: },
pub const WMFW_ADSP1: c_int = 1;
pub const WMFW_ADSP2: c_int = 2;
pub const WMFW_HALO: c_int = 4;
pub const WMFW_ABSOLUTE: c_uint = 0xf0;
pub const WMFW_ALGORITHM_DATA: c_uint = 0xf2;
pub const WMFW_METADATA: c_uint = 0xfc;
pub const WMFW_NAME_TEXT: c_uint = 0xfe;
pub const WMFW_INFO_TEXT: c_uint = 0xff;
pub const WMFW_ADSP1_PM: c_int = 2;
pub const WMFW_ADSP1_DM: c_int = 3;
pub const WMFW_ADSP1_ZM: c_int = 4;
pub const WMFW_ADSP2_PM: c_int = 2;
pub const WMFW_ADSP2_ZM: c_int = 4;
pub const WMFW_ADSP2_XM: c_int = 5;
pub const WMFW_ADSP2_YM: c_int = 6;
pub const WMFW_HALO_PM_PACKED: c_uint = 0x10;
pub const WMFW_HALO_XM_PACKED: c_uint = 0x11;
pub const WMFW_HALO_YM_PACKED: c_uint = 0x12;
pub const WMFW_ADSP2_XM_LONG: c_uint = 0xf405;
pub const WMFW_ADSP2_YM_LONG: c_uint = 0xf406;
pub const WMFW_HALO_XM_PACKED_LONG: c_uint = 0xf411;
pub const WMFW_HALO_YM_PACKED_LONG: c_uint = 0xf412;

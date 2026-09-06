//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/shmobile/shmob_drm_regs.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// shmob_drm_regs.h  --  SH Mobile DRM registers
//
// Copyright (C) 2012 Renesas Electronics Corporation
//
// Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

// Register definitions
pub const LDDCKPAT1R: c_uint = 0x400;
pub const LDDCKPAT2R: c_uint = 0x404;
pub const LDDCKR: c_uint = 0x410;

pub const LDDCKSTPR: c_uint = 0x414;

pub const LDMT1R: c_uint = 0x418;

pub const LDMT2R: c_uint = 0x41c;

pub const LDMT2R_CSUP_SHIFT: c_int = 26;

pub const LDMT2R_WCSC_SHIFT: c_int = 16;

pub const LDMT2R_WCEC_SHIFT: c_int = 8;

pub const LDMT2R_WCLW_SHIFT: c_int = 0;
pub const LDMT3R: c_uint = 0x420;

pub const LDMT3R_RDLC_SHIFT: c_int = 24;

pub const LDMT3R_RCSC_SHIFT: c_int = 16;

pub const LDMT3R_RCEC_SHIFT: c_int = 8;

pub const LDMT3R_RCLW_SHIFT: c_int = 0;
pub const LDDFR: c_uint = 0x424;

pub const LDSM1R: c_uint = 0x428;

pub const LDSM2R: c_uint = 0x42c;

pub const LDSA1R: c_uint = 0x430;
pub const LDSA2R: c_uint = 0x434;
pub const LDMLSR: c_uint = 0x438;
pub const LDWBFR: c_uint = 0x43c;
pub const LDWBCNTR: c_uint = 0x440;
pub const LDWBAR: c_uint = 0x444;
pub const LDHCNR: c_uint = 0x448;
pub const LDHSYNR: c_uint = 0x44c;
pub const LDVLNR: c_uint = 0x450;
pub const LDVSYNR: c_uint = 0x454;
pub const LDHPDR: c_uint = 0x458;
pub const LDVPDR: c_uint = 0x45c;
pub const LDPMR: c_uint = 0x460;

pub const LDINTR: c_uint = 0x468;

pub const LDSR: c_uint = 0x46c;

pub const LDCNT1R: c_uint = 0x470;

pub const LDCNT2R: c_uint = 0x474;

pub const LDRCNTR: c_uint = 0x478;

pub const LDDDSR: c_uint = 0x47c;

pub const LDHAJR: c_uint = 0x4a0;
pub const LDDWD0R: c_uint = 0x800;

pub const LDDRDR: c_uint = 0x840;

pub const LDDWAR: c_uint = 0x900;

pub const LDDRAR: c_uint = 0x904;

pub const LDBCR: c_uint = 0xb00;

pub const LDBBSIFR_LAY_SHIFT: c_int = 16;

pub const LDBBSIFR_ROP3_SHIFT: c_int = 16;

pub const LDBBSSZR_BVSS_SHIFT: c_int = 16;

pub const LDBBSSZR_BHSS_SHIFT: c_int = 0;

pub const LDBBLOCR_CVLC_SHIFT: c_int = 16;

pub const LDBBLOCR_CHLC_SHIFT: c_int = 0;

pub const LDBBSMWR_BSMWA_SHIFT: c_int = 16;

pub const LDBBSMWR_BSMW_SHIFT: c_int = 0;

pub const LDBBSAYR_FG1A_SHIFT: c_int = 24;

pub const LDBBSAYR_FG1R_SHIFT: c_int = 16;

pub const LDBBSAYR_FG1G_SHIFT: c_int = 8;

pub const LDBBSAYR_FG1B_SHIFT: c_int = 0;

pub const LDBBSACR_FG2A_SHIFT: c_int = 24;

pub const LDBBSACR_FG2R_SHIFT: c_int = 16;

pub const LDBBSACR_FG2G_SHIFT: c_int = 8;

pub const LDBBSACR_FG2B_SHIFT: c_int = 0;

pub const LDBBSAAR_AP_SHIFT: c_int = 24;

pub const LDBBSAAR_R_SHIFT: c_int = 16;

pub const LDBBSAAR_GY_SHIFT: c_int = 8;

pub const LDBBSAAR_B_SHIFT: c_int = 0;

pub const LDBBPPCR_AP_SHIFT: c_int = 24;

pub const LDBBPPCR_R_SHIFT: c_int = 16;

pub const LDBBPPCR_GY_SHIFT: c_int = 8;

pub const LDBBPPCR_B_SHIFT: c_int = 0;

pub const LDBBBGCL_BGA_SHIFT: c_int = 24;

pub const LDBBBGCL_BGR_SHIFT: c_int = 16;

pub const LDBBBGCL_BGG_SHIFT: c_int = 8;

pub const LDBBBGCL_BGB_SHIFT: c_int = 0;
pub const LCDC_SIDE_B_OFFSET: c_uint = 0x1000;
pub const LCDC_MIRROR_OFFSET: c_uint = 0x2000;
extern "C" {
    pub fn ioread32(reg: sdev->mmio +) -> return;
}

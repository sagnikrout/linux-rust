//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/memory/jedec_ddr.h
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
// Definitions for DDR memories based on JEDEC specs
//
// Copyright (C) 2012 Texas Instruments, Inc.
//
// Aneesh V <aneesh@ti.com>
//

// DDR Densities
pub const DDR_DENSITY_64Mb: c_int = 1;
pub const DDR_DENSITY_128Mb: c_int = 2;
pub const DDR_DENSITY_256Mb: c_int = 3;
pub const DDR_DENSITY_512Mb: c_int = 4;
pub const DDR_DENSITY_1Gb: c_int = 5;
pub const DDR_DENSITY_2Gb: c_int = 6;
pub const DDR_DENSITY_4Gb: c_int = 7;
pub const DDR_DENSITY_8Gb: c_int = 8;
pub const DDR_DENSITY_16Gb: c_int = 9;
pub const DDR_DENSITY_32Gb: c_int = 10;
// DDR type
pub const DDR_TYPE_DDR2: c_int = 1;
pub const DDR_TYPE_DDR3: c_int = 2;
pub const DDR_TYPE_LPDDR2_S4: c_int = 3;
pub const DDR_TYPE_LPDDR2_S2: c_int = 4;
pub const DDR_TYPE_LPDDR2_NVM: c_int = 5;
pub const DDR_TYPE_LPDDR3: c_int = 6;
// DDR IO width
pub const DDR_IO_WIDTH_4: c_int = 1;
pub const DDR_IO_WIDTH_8: c_int = 2;
pub const DDR_IO_WIDTH_16: c_int = 3;
pub const DDR_IO_WIDTH_32: c_int = 4;
// Number of Row bits
pub const R9: c_int = 9;
pub const R10: c_int = 10;
pub const R11: c_int = 11;
pub const R12: c_int = 12;
pub const R13: c_int = 13;
pub const R14: c_int = 14;
pub const R15: c_int = 15;
pub const R16: c_int = 16;
// Number of Column bits
pub const C7: c_int = 7;
pub const C8: c_int = 8;
pub const C9: c_int = 9;
pub const C10: c_int = 10;
pub const C11: c_int = 11;
pub const C12: c_int = 12;
// Number of Banks
pub const B1: c_int = 0;
pub const B2: c_int = 1;
pub const B4: c_int = 2;
pub const B8: c_int = 3;
// Refresh rate in nano-seconds
pub const T_REFI_15_6: c_int = 15600;
pub const T_REFI_7_8: c_int = 7800;
pub const T_REFI_3_9: c_int = 3900;
// tRFC values
pub const T_RFC_90: c_int = 90000;
pub const T_RFC_110: c_int = 110000;
pub const T_RFC_130: c_int = 130000;
pub const T_RFC_160: c_int = 160000;
pub const T_RFC_210: c_int = 210000;
pub const T_RFC_300: c_int = 300000;
pub const T_RFC_350: c_int = 350000;
// Mode register numbers
pub const DDR_MR0: c_int = 0;
pub const DDR_MR1: c_int = 1;
pub const DDR_MR2: c_int = 2;
pub const DDR_MR3: c_int = 3;
pub const DDR_MR4: c_int = 4;
pub const DDR_MR5: c_int = 5;
pub const DDR_MR6: c_int = 6;
pub const DDR_MR7: c_int = 7;
pub const DDR_MR8: c_int = 8;
pub const DDR_MR9: c_int = 9;
pub const DDR_MR10: c_int = 10;
pub const DDR_MR11: c_int = 11;
pub const DDR_MR16: c_int = 16;
pub const DDR_MR17: c_int = 17;
pub const DDR_MR18: c_int = 18;
//
// LPDDR2 related defines
//
// MR4 register fields
pub const MR4_SDRAM_REF_RATE_SHIFT: c_int = 0;
pub const MR4_SDRAM_REF_RATE_MASK: c_int = 7;
pub const MR4_TUF_SHIFT: c_int = 7;

// MR4 SDRAM Refresh Rate field values
pub const SDRAM_TEMP_NOMINAL: c_uint = 0x3;
pub const SDRAM_TEMP_RESERVED_4: c_uint = 0x4;
pub const SDRAM_TEMP_HIGH_DERATE_REFRESH: c_uint = 0x5;
pub const SDRAM_TEMP_HIGH_DERATE_REFRESH_AND_TIMINGS: c_uint = 0x6;
pub const SDRAM_TEMP_VERY_HIGH_SHUTDOWN: c_uint = 0x7;
pub const NUM_DDR_ADDR_TABLE_ENTRIES: c_int = 11;
pub const NUM_DDR_TIMING_TABLE_ENTRIES: c_int = 4;
pub const LPDDR2_MANID_SAMSUNG: c_int = 1;
pub const LPDDR2_MANID_QIMONDA: c_int = 2;
pub const LPDDR2_MANID_ELPIDA: c_int = 3;
pub const LPDDR2_MANID_ETRON: c_int = 4;
pub const LPDDR2_MANID_NANYA: c_int = 5;
pub const LPDDR2_MANID_HYNIX: c_int = 6;
pub const LPDDR2_MANID_MOSEL: c_int = 7;
pub const LPDDR2_MANID_WINBOND: c_int = 8;
pub const LPDDR2_MANID_ESMT: c_int = 9;
pub const LPDDR2_MANID_SPANSION: c_int = 11;
pub const LPDDR2_MANID_SST: c_int = 12;
pub const LPDDR2_MANID_ZMOS: c_int = 13;
pub const LPDDR2_MANID_INTEL: c_int = 14;
pub const LPDDR2_MANID_NUMONYX: c_int = 254;
pub const LPDDR2_MANID_MICRON: c_int = 255;
pub const LPDDR2_TYPE_S4: c_int = 0;
pub const LPDDR2_TYPE_S2: c_int = 1;
pub const LPDDR2_TYPE_NVM: c_int = 2;
// Structure for DDR addressing info from the JEDEC spec
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpddr2_addressing {
    pub num_banks: u32,
    pub tREFI_ns: u32,
    pub tRFCab_ps: u32,
}

//
// Structure for timings from the LPDDR2 datasheet
// All parameters are in pico seconds(ps) unless explicitly indicated
// with a suffix like tRAS_max_ns below
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpddr2_timings {
    pub max_freq: u32,
    pub min_freq: u32,
    pub tRPab: u32,
    pub tRCD: u32,
    pub tWR: u32,
    pub tRAS_min: u32,
    pub tRRD: u32,
    pub tWTR: u32,
    pub tXP: u32,
    pub tRTP: u32,
    pub tCKESR: u32,
    pub tDQSCK_max: u32,
    pub tDQSCK_max_derated: u32,
    pub tFAW: u32,
    pub tZQCS: u32,
    pub tZQCL: u32,
    pub tZQinit: u32,
    pub tRAS_max_ns: u32,
}

//
// Min value for some parameters in terms of number of tCK cycles(nCK)
// Please set to zero parameters that are not valid for a given memory
// type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpddr2_min_tck {
    pub tRPab: u32,
    pub tRCD: u32,
    pub tWR: u32,
    pub tRASmin: u32,
    pub tRRD: u32,
    pub tWTR: u32,
    pub tXP: u32,
    pub tRTP: u32,
    pub tCKE: u32,
    pub tCKESR: u32,
    pub tFAW: u32,
}

// Structure of MR8
#[repr(C)]
#[derive(Copy, Clone)]
pub union lpddr2_basic_config4 {
    pub value: u32,
    pub 2: unsigned int arch_type :,
    pub 4: unsigned int density :,
    pub 2: unsigned int io_width :,
    pub __packed: },
}

//
// Structure for information about LPDDR2 chip. All parameters are
// matching raw values of standard mode register bitfields or set to
// -ENOENT if info unavailable.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpddr2_info {
    pub arch_type: c_int,
    pub density: c_int,
    pub io_width: c_int,
    pub manufacturer_id: c_int,
    pub revision_id1: c_int,
    pub revision_id2: c_int,
}

//
// Structure for timings for LPDDR3 based on LPDDR2 plus additional fields.
// All parameters are in pico seconds(ps) excluding max_freq, min_freq which
// are in Hz.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpddr3_timings {
    pub max_freq: u32,
    pub min_freq: u32,
    pub tRFC: u32,
    pub tRRD: u32,
    pub tRPab: u32,
    pub tRPpb: u32,
    pub tRCD: u32,
    pub tRC: u32,
    pub tRAS: u32,
    pub tWTR: u32,
    pub tWR: u32,
    pub tRTP: u32,
    pub tW2W_C2C: u32,
    pub tR2R_C2C: u32,
    pub tWL: u32,
    pub tDQSCK: u32,
    pub tRL: u32,
    pub tFAW: u32,
    pub tXSR: u32,
    pub tXP: u32,
    pub tCKE: u32,
    pub tCKESR: u32,
    pub tMRD: u32,
}

//
// Min value for some parameters in terms of number of tCK cycles(nCK)
// Please set to zero parameters that are not valid for a given memory
// type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpddr3_min_tck {
    pub tRFC: u32,
    pub tRRD: u32,
    pub tRPab: u32,
    pub tRPpb: u32,
    pub tRCD: u32,
    pub tRC: u32,
    pub tRAS: u32,
    pub tWTR: u32,
    pub tWR: u32,
    pub tRTP: u32,
    pub tW2W_C2C: u32,
    pub tR2R_C2C: u32,
    pub tWL: u32,
    pub tDQSCK: u32,
    pub tRL: u32,
    pub tFAW: u32,
    pub tXSR: u32,
    pub tXP: u32,
    pub tCKE: u32,
    pub tCKESR: u32,
    pub tMRD: u32,
}

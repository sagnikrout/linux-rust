//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/cfi.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright © 2000-2010 David Woodhouse <dwmw2@infradead.org> et al.
//

pub const cfi_interleave(cfi): c_int = 1;

// NB: these values must represents the number of bytes needed to meet the
// device type (x8, x16, x32).  Eg. a 32 bit device is 4 x 8 bytes.
// These numbers are used in calculations.
//

// Device Interface Code Assignments from the "Common Flash Memory Interface
// Publication 100" dated December 1, 2001.
//
pub const CFI_INTERFACE_X8_ASYNC: c_uint = 0x0000;
pub const CFI_INTERFACE_X16_ASYNC: c_uint = 0x0001;
pub const CFI_INTERFACE_X8_BY_X16_ASYNC: c_uint = 0x0002;
pub const CFI_INTERFACE_X32_ASYNC: c_uint = 0x0003;
pub const CFI_INTERFACE_X16_BY_X32_ASYNC: c_uint = 0x0005;
pub const CFI_INTERFACE_NOT_ALLOWED: c_uint = 0xffff;
// NB: We keep these structures in memory in HOST byteorder, except
// where individually noted.
//
// Basic Query Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_ident {
    pub qry: [u8; 3],
    pub P_ID: u16,
    pub P_ADR: u16,
    pub A_ID: u16,
    pub A_ADR: u16,
    pub VccMin: u8,
    pub VccMax: u8,
    pub VppMin: u8,
    pub VppMax: u8,
    pub WordWriteTimeoutTyp: u8,
    pub BufWriteTimeoutTyp: u8,
    pub BlockEraseTimeoutTyp: u8,
    pub ChipEraseTimeoutTyp: u8,
    pub WordWriteTimeoutMax: u8,
    pub BufWriteTimeoutMax: u8,
    pub BlockEraseTimeoutMax: u8,
    pub ChipEraseTimeoutMax: u8,
    pub DevSize: u8,
    pub InterfaceDesc: u16,
    pub MaxBufWriteSize: u16,
    pub NumEraseRegions: u8,
    pub /: *mut *mut uint32_t EraseRegionInfo[]; / Not host ordered,
    pub __packed: },
// Extended Query Structure for both PRI and ALT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_extquery {
    pub pri: [u8; 3],
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub __packed: },
// Vendor-Specific PRI for Intel/Sharp Extended Command Set (0x0001)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_pri_intelext {
    pub pri: [u8; 3],
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub feature: *mut *mut uint32_t FeatureSupport; / if bit 31 is set then an additional uint32_t,
    pub SuspendCmdSupport: u8,
    pub BlkStatusRegMask: u16,
    pub VccOptimal: u8,
    pub VppOptimal: u8,
    pub NumProtectionFields: u8,
    pub ProtRegAddr: u16,
    pub FactProtRegSize: u8,
    pub UserProtRegSize: u8,
    pub extra: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_intelext_otpinfo {
    pub ProtRegAddr: u32,
    pub FactGroups: u16,
    pub FactProtRegSize: u8,
    pub UserGroups: u16,
    pub UserProtRegSize: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_intelext_blockinfo {
    pub NumIdentBlocks: u16,
    pub BlockSize: u16,
    pub MinBlockEraseCycles: u16,
    pub BitsPerCell: u8,
    pub BlockCap: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_intelext_regioninfo {
    pub NumIdentPartitions: u16,
    pub NumOpAllowed: u8,
    pub NumOpAllowedSimProgMode: u8,
    pub NumOpAllowedSimEraMode: u8,
    pub NumBlockTypes: u8,
    pub BlockTypes: [cfi_intelext_blockinfo; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_intelext_programming_regioninfo {
    pub ProgRegShift: u8,
    pub Reserved1: u8,
    pub ControlValid: u8,
    pub Reserved2: u8,
    pub ControlInvalid: u8,
    pub Reserved3: u8,
    pub __packed: },
// Vendor-Specific PRI for AMD/Fujitsu Extended Command Set (0x0002)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_pri_amdstd {
    pub pri: [u8; 3],
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub /: *mut *mut uint8_t SiliconRevision; / bits 1-0: Address Sensitive Unlock,
    pub EraseSuspend: u8,
    pub BlkProt: u8,
    pub TmpBlkUnprotect: u8,
    pub BlkProtUnprot: u8,
    pub SimultaneousOps: u8,
    pub BurstMode: u8,
    pub PageMode: u8,
    pub VppMin: u8,
    pub VppMax: u8,
    pub TopBottom: u8,
// Below field are added from version 1.5
    pub ProgramSuspend: u8,
    pub UnlockBypass: u8,
    pub SecureSiliconSector: u8,
    pub SoftwareFeatures: u8,

    pub __packed: },
// Vendor-Specific PRI for Atmel chips (command set 0x0002)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_pri_atmel {
    pub pri: [u8; 3],
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub Features: u8,
    pub BottomBoot: u8,
    pub BurstMode: u8,
    pub PageMode: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_pri_query {
    pub NumFields: u8,
    pub /: *mut *mut uint32_t ProtField[1]; / Not host ordered,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_bri_query {
    pub PageModeReadCap: u8,
    pub NumFields: u8,
    pub /: *mut *mut uint32_t ConfField[1]; / Not host ordered,
    pub __packed: },
pub const P_ID_NONE: c_uint = 0x0000;
pub const P_ID_INTEL_EXT: c_uint = 0x0001;
pub const P_ID_AMD_STD: c_uint = 0x0002;
pub const P_ID_INTEL_STD: c_uint = 0x0003;
pub const P_ID_AMD_EXT: c_uint = 0x0004;
pub const P_ID_WINBOND: c_uint = 0x0006;
pub const P_ID_ST_ADV: c_uint = 0x0020;
pub const P_ID_MITSUBISHI_STD: c_uint = 0x0100;
pub const P_ID_MITSUBISHI_EXT: c_uint = 0x0101;
pub const P_ID_SST_PAGE: c_uint = 0x0102;
pub const P_ID_SST_OLD: c_uint = 0x0701;
pub const P_ID_INTEL_PERFORMANCE: c_uint = 0x0200;
pub const P_ID_INTEL_DATA: c_uint = 0x0210;
pub const P_ID_RESERVED: c_uint = 0xffff;
pub const CFI_MODE_CFI: c_int = 1;
pub const CFI_MODE_JEDEC: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_private {
    pub cmdset: u16,
    pub cmdset_priv: *mut c_void,
    pub interleave: c_int,
    pub device_type: c_int,
    pub /: *mut *mut int cfi_mode; / Are we a JEDEC device pretending to be CFI?,
    pub addr_unlock1: c_int,
    pub addr_unlock2: c_int,
    pub ): *mut *mut *mut mtd_info (cmdset_setup)(map_info,
    pub devs: *mut *mut *mut cfi_ident cfiq; / For now only one. We insist that all,
    pub id: int mfr,,
    pub numchips: c_int,
    pub sector_erase_cmd: map_word,
    pub /: *mut *mut unsigned long chipshift; / Because they're of the same type,
    pub /: *const *const *const char im_name; / inter_module name for cmdset_setup,
    pub quirks: c_ulong,
    pub /: *mut *mut flchip chips[] __counted_by(numchips); / per-chip data structure for each chip,
}

extern "C" {
    pub fn cfi_build_cmd(cmd: u_long, map: *mut map_info, cfi: *mut cfi_private) -> map_word;
}

extern "C" {
    pub fn cfi16_to_cpu(_arg: map, _arg: val.x[0]) -> return;
}
//
// No point in a 64-bit byteswap since that would just be
// swapping the responses from different chips, and we are
// only interested in one chip (a representative sample)
//
extern "C" {
    pub fn cfi32_to_cpu(_arg: map, _arg: val.x[0]) -> return;
}
extern "C" {
    pub fn cfi16_to_cpu(_arg: map, _arg: val.x[0]) -> return;
}
//
// No point in a 64-bit byteswap since that would just be
// swapping the responses from different chips, and we are
// only interested in one chip (a representative sample)
//
extern "C" {
    pub fn cfi32_to_cpu(_arg: map, _arg: val.x[0]) -> return;
}
extern "C" {
    pub fn cfi_udelay(us: c_int);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfi_fixup {
    pub mfr: u16,
    pub id: u16,
    pub mtd): *mut *mut void (fixup)(struct mtd_info,
}

pub const CFI_MFR_ANY: c_uint = 0xFFFF;
pub const CFI_ID_ANY: c_uint = 0xFFFF;
pub const CFI_MFR_CONTINUATION: c_uint = 0x007F;
pub const CFI_MFR_AMD: c_uint = 0x0001;
pub const CFI_MFR_AMIC: c_uint = 0x0037;
pub const CFI_MFR_ATMEL: c_uint = 0x001F;
pub const CFI_MFR_EON: c_uint = 0x001C;
pub const CFI_MFR_FUJITSU: c_uint = 0x0004;
pub const CFI_MFR_HYUNDAI: c_uint = 0x00AD;
pub const CFI_MFR_INTEL: c_uint = 0x0089;
pub const CFI_MFR_MACRONIX: c_uint = 0x00C2;
pub const CFI_MFR_NEC: c_uint = 0x0010;
pub const CFI_MFR_PMC: c_uint = 0x009D;
pub const CFI_MFR_SAMSUNG: c_uint = 0x00EC;
pub const CFI_MFR_SHARP: c_uint = 0x00B0;
pub const CFI_MFR_SST: c_uint = 0x00BF;
pub const CFI_MFR_ST: c_uint = 0x0020 /* STMicroelectronics */;
pub const CFI_MFR_MICRON: c_uint = 0x002C /* Micron */;
pub const CFI_MFR_TOSHIBA: c_uint = 0x0098;
pub const CFI_MFR_WINBOND: c_uint = 0x00DA;
extern "C" {
    pub fn cfi_fixup(mtd: *mut mtd_info, fixups: *mut *mut cfi_fixup);
}

//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/doc2000.h
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
// Linux driver for Disk-On-Chip devices
//
// Copyright © 1999 Machine Vision Holdings, Inc.
// Copyright © 1999-2010 David Woodhouse <dwmw2@infradead.org>
// Copyright © 2002-2003 Greg Ungerer <gerg@snapgear.com>
// Copyright © 2002-2003 SnapGear Inc
//

pub const DoC_Sig1: c_int = 0;
pub const DoC_Sig2: c_int = 1;
pub const DoC_ChipID: c_uint = 0x1000;
pub const DoC_DOCStatus: c_uint = 0x1001;
pub const DoC_DOCControl: c_uint = 0x1002;
pub const DoC_FloorSelect: c_uint = 0x1003;
pub const DoC_CDSNControl: c_uint = 0x1004;
pub const DoC_CDSNDeviceSelect: c_uint = 0x1005;
pub const DoC_ECCConf: c_uint = 0x1006;
pub const DoC_2k_ECCStatus: c_uint = 0x1007;
pub const DoC_CDSNSlowIO: c_uint = 0x100d;
pub const DoC_ECCSyndrome0: c_uint = 0x1010;
pub const DoC_ECCSyndrome1: c_uint = 0x1011;
pub const DoC_ECCSyndrome2: c_uint = 0x1012;
pub const DoC_ECCSyndrome3: c_uint = 0x1013;
pub const DoC_ECCSyndrome4: c_uint = 0x1014;
pub const DoC_ECCSyndrome5: c_uint = 0x1015;
pub const DoC_AliasResolution: c_uint = 0x101b;
pub const DoC_ConfigInput: c_uint = 0x101c;
pub const DoC_ReadPipeInit: c_uint = 0x101d;
pub const DoC_WritePipeTerm: c_uint = 0x101e;
pub const DoC_LastDataRead: c_uint = 0x101f;
pub const DoC_NOP: c_uint = 0x1020;
pub const DoC_Mil_CDSN_IO: c_uint = 0x0800;
pub const DoC_2k_CDSN_IO: c_uint = 0x1800;
pub const DoC_Mplus_NOP: c_uint = 0x1002;
pub const DoC_Mplus_AliasResolution: c_uint = 0x1004;
pub const DoC_Mplus_DOCControl: c_uint = 0x1006;
pub const DoC_Mplus_AccessStatus: c_uint = 0x1008;
pub const DoC_Mplus_DeviceSelect: c_uint = 0x1008;
pub const DoC_Mplus_Configuration: c_uint = 0x100a;
pub const DoC_Mplus_OutputControl: c_uint = 0x100c;
pub const DoC_Mplus_FlashControl: c_uint = 0x1020;
pub const DoC_Mplus_FlashSelect: c_uint = 0x1022;
pub const DoC_Mplus_FlashCmd: c_uint = 0x1024;
pub const DoC_Mplus_FlashAddress: c_uint = 0x1026;
pub const DoC_Mplus_FlashData0: c_uint = 0x1028;
pub const DoC_Mplus_FlashData1: c_uint = 0x1029;
pub const DoC_Mplus_ReadPipeInit: c_uint = 0x102a;
pub const DoC_Mplus_LastDataRead: c_uint = 0x102c;
pub const DoC_Mplus_LastDataRead1: c_uint = 0x102d;
pub const DoC_Mplus_WritePipeTerm: c_uint = 0x102e;
pub const DoC_Mplus_ECCSyndrome0: c_uint = 0x1040;
pub const DoC_Mplus_ECCSyndrome1: c_uint = 0x1041;
pub const DoC_Mplus_ECCSyndrome2: c_uint = 0x1042;
pub const DoC_Mplus_ECCSyndrome3: c_uint = 0x1043;
pub const DoC_Mplus_ECCSyndrome4: c_uint = 0x1044;
pub const DoC_Mplus_ECCSyndrome5: c_uint = 0x1045;
pub const DoC_Mplus_ECCConf: c_uint = 0x1046;
pub const DoC_Mplus_Toggle: c_uint = 0x1046;
pub const DoC_Mplus_DownloadStatus: c_uint = 0x1074;
pub const DoC_Mplus_CtrlConfirm: c_uint = 0x1076;
pub const DoC_Mplus_Power: c_uint = 0x1fff;
// How to access the device?
// On ARM, it'll be mmap'd directly with 32-bit wide accesses.
// On PPC, it's mmap'd and 16-bit wide.
// Others use readb/writeb
//

extern "C" {
    pub fn __raw_readl(reg: addr +) -> return;
}
pub const DOC_IOREMAP_LEN: c_uint = 0x8000;

extern "C" {
    pub fn __raw_readw(reg: addr +) -> return;
}
pub const DOC_IOREMAP_LEN: c_uint = 0x4000;

pub const DOC_IOREMAP_LEN: c_uint = 0x2000;

// Macro flag: #define USE_MEMCPY

// These are provided to directly use the DoC_xxx defines

pub const DOC_MODE_RESET: c_int = 0;
pub const DOC_MODE_NORMAL: c_int = 1;
pub const DOC_MODE_RESERVED1: c_int = 2;
pub const DOC_MODE_RESERVED2: c_int = 3;
pub const DOC_MODE_CLR_ERR: c_uint = 0x80;
pub const DOC_MODE_RST_LAT: c_uint = 0x10;
pub const DOC_MODE_BDECT: c_uint = 0x08;
pub const DOC_MODE_MDWREN: c_uint = 0x04;
pub const DOC_ChipID_Doc2k: c_uint = 0x20;
pub const DOC_ChipID_Doc2kTSOP: c_uint = 0x21	/* internal number for MTD */;
pub const DOC_ChipID_DocMil: c_uint = 0x30;
pub const DOC_ChipID_DocMilPlus32: c_uint = 0x40;
pub const DOC_ChipID_DocMilPlus16: c_uint = 0x41;
pub const CDSN_CTRL_FR_B: c_uint = 0x80;
pub const CDSN_CTRL_FR_B0: c_uint = 0x40;
pub const CDSN_CTRL_FR_B1: c_uint = 0x80;
pub const CDSN_CTRL_ECC_IO: c_uint = 0x20;
pub const CDSN_CTRL_FLASH_IO: c_uint = 0x10;
pub const CDSN_CTRL_WP: c_uint = 0x08;
pub const CDSN_CTRL_ALE: c_uint = 0x04;
pub const CDSN_CTRL_CLE: c_uint = 0x02;
pub const CDSN_CTRL_CE: c_uint = 0x01;
pub const DOC_ECC_RESET: c_int = 0;
pub const DOC_ECC_ERROR: c_uint = 0x80;
pub const DOC_ECC_RW: c_uint = 0x20;
pub const DOC_ECC__EN: c_uint = 0x08;
pub const DOC_TOGGLE_BIT: c_uint = 0x04;
pub const DOC_ECC_RESV: c_uint = 0x02;
pub const DOC_ECC_IGNORE: c_uint = 0x01;
pub const DOC_FLASH_CE: c_uint = 0x80;
pub const DOC_FLASH_WP: c_uint = 0x40;
pub const DOC_FLASH_BANK: c_uint = 0x02;
// We have to also set the reserved bit 1 for enable

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Nand {
    pub chip: char floor,,
    pub curadr: c_ulong,
    pub curmode: c_uchar,
// Also some erase/write/pipeline info when we get that far
}

pub const MAX_FLOORS: c_int = 4;
pub const MAX_CHIPS: c_int = 4;
pub const MAX_FLOORS_MIL: c_int = 1;
pub const MAX_CHIPS_MIL: c_int = 1;
pub const MAX_FLOORS_MPLUS: c_int = 2;
pub const MAX_CHIPS_MPLUS: c_int = 1;
pub const ADDR_COLUMN: c_int = 1;
pub const ADDR_PAGE: c_int = 2;
pub const ADDR_COLUMN_PAGE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DiskOnChip {
    pub physadr: c_ulong,
    pub virtadr: *mut void __iomem,
    pub totlen: c_ulong,
    pub /: *mut *mut unsigned char ChipID; / Type of DiskOnChip,
    pub ioreg: c_int,
    pub /: *mut *mut unsigned long mfr; / Flash IDs - only one type of flash per device,
    pub id: c_ulong,
    pub chipshift: c_int,
    pub page256: c_char,
    pub pageadrlen: c_char,
    pub /: *mut *mut char interleave; / Internal interleaving - Millennium Plus style,
    pub erasesize: c_ulong,
    pub curfloor: c_int,
    pub curchip: c_int,
    pub numchips: c_int,
    pub chips: *mut Nand,
    pub nextdoc: *mut mtd_info,
    pub lock: mutex,
}

extern "C" {
    pub fn doc_decode_ecc(sector[512]: c_uchar, ecc1[6]: c_uchar) -> c_int;
}

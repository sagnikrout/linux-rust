//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/io_edgeport.h
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
// io_edgeport.h	Edgeport Linux Interface definitions
//
// Copyright (C) 2000 Inside Out Networks, Inc.
//

// typedefs that the insideout headers need

//
// Product information read from the Edgeport
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edgeport_product_info {
    pub /: *mut *mut __u16 ProductId; / Product Identifier,
    pub /: *mut *mut __u8 NumPorts; / Number of ports on edgeport,
    pub /: *mut *mut __u8 ProdInfoVer; / What version of structure is this?,
    pub /: *mut *mut __u32 IsServer :1; / Set if Server,
    pub /: *mut *mut __u32 IsRS232 :1; / Set if RS-232 ports exist,
    pub /: *mut *mut __u32 IsRS422 :1; / Set if RS-422 ports exist,
    pub /: *mut *mut __u32 IsRS485 :1; / Set if RS-485 ports exist,
    pub /: *mut *mut __u32 IsReserved :28; / Reserved for later expansion,
    pub /: *mut *mut __u8 RomSize; / Size of ROM/E2PROM in K,
    pub /: *mut *mut __u8 RamSize; / Size of external RAM in K,
    pub /: *mut *mut __u8 CpuRev; / CPU revision level (chg only if s/w visible),
    pub /: *mut *mut __u8 BoardRev; / PCB revision level (chg only if s/w visible),
    pub /: *mut *mut __u8 BootMajorVersion; / Boot Firmware version: xx.,
    pub /: *mut *mut __u8 BootMinorVersion; / yy.,
    pub /: *mut *mut __le16 BootBuildNumber; / zzzz (LE format),
    pub /: *mut *mut __u8 FirmwareMajorVersion; / Operational Firmware version:xx.,
    pub /: *mut *mut __u8 FirmwareMinorVersion; / yy.,
    pub /: *mut *mut __le16 FirmwareBuildNumber; / zzzz (LE format),
    pub /: *mut *mut __u8 ManufactureDescDate[3]; / MM/DD/YY when descriptor template was compiled,
    pub HardwareType: __u8,
    pub /: *mut *mut __u8 iDownloadFile; / What to download to EPiC device,
    pub /: *mut *mut __u8 EpicVer; / What version of EPiC spec this device supports,
    pub Epic: edge_compatibility_bits,
}

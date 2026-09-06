//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/edd.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// linux/include/linux/edd.h
// Copyright (C) 2002, 2003, 2004 Dell Inc.
// by Matt Domsch <Matt_Domsch@dell.com>
//
// structures and definitions for the int 13h, ax={41,48}h
// BIOS Enhanced Disk Drive Services
// This is based on the T13 group document D1572 Revision 0 (August 14 2002)
// available at http://www.t13.org/docs2002/d1572r0.pdf.  It is
// very similar to D1484 Revision 3 http://www.t13.org/docs2002/d1484r3.pdf
//
// In a nutshell, arch/{i386,x86_64}/boot/setup.S populates a scratch
// table in the boot_params that contains a list of BIOS-enumerated
// boot devices.
// In arch/{i386,x86_64}/kernel/setup.c, this information is
// transferred into the edd structure, and in drivers/firmware/edd.c, that
// information is used to identify BIOS boot disk.  The code in setup.S
// is very sensitive to the size of these structures.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License v2.0 as published by
// the Free Software Foundation
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

pub const EDDNR: c_uint = 0x1e9		/* addr of number of edd_info structs at EDDBUF;
pub const EDDBUF: c_uint = 0xd00		/* addr of edd_info structs in boot_params */;

pub const EDDPARMSIZE: c_int = 74;
pub const CHECKEXTENSIONSPRESENT: c_uint = 0x41;
pub const GETDEVICEPARAMETERS: c_uint = 0x48;
pub const LEGACYGETDEVICEPARAMETERS: c_uint = 0x08;
pub const EDDMAGIC1: c_uint = 0x55AA;
pub const EDDMAGIC2: c_uint = 0xAA55;
pub const READ_SECTORS: c_uint = 0x02         /* int13 AH=0x02 is READ_SECTORS command */;
pub const EDD_MBR_SIG_OFFSET: c_uint = 0x1B8  /* offset of signature in the MBR */;
pub const EDD_MBR_SIG_BUF: c_uint = 0x290  /* addr in boot params */;

pub const EDD_MBR_SIG_NR_BUF: c_uint = 0x1ea  /* addr of number of MBR signtaures at EDD_MBR_SIG_BUF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct edd_device_params {
    pub length: __u16,
    pub info_flags: __u16,
    pub num_default_cylinders: __u32,
    pub num_default_heads: __u32,
    pub sectors_per_track: __u32,
    pub number_of_sectors: __u64,
    pub bytes_per_sector: __u16,
    pub /: *mut *mut __u32 dpte_ptr; / 0xFFFFFFFF for our purposes,
    pub /: *mut *mut __u16 key; / = 0xBEDD,
    pub /: *mut *mut __u8 device_path_info_length; / = 44,
    pub reserved2: __u8,
    pub reserved3: __u16,
    pub host_bus_type: [__u8; 4],
    pub interface_type: [__u8; 8],
    pub base_address: __u16,
    pub reserved1: __u16,
    pub reserved2: __u32,
// C attribute field omitted
    pub bus: __u8,
    pub slot: __u8,
    pub function: __u8,
    pub channel: __u8,
    pub reserved: __u32,
// C attribute field omitted
// pcix is same as pci
    pub reserved: __u64,
// C attribute field omitted
    pub reserved: __u64,
// C attribute field omitted
    pub reserved: __u64,
// C attribute field omitted
    pub reserved: __u64,
// C attribute field omitted
    pub interface_path: },
    pub device: __u8,
    pub reserved1: __u8,
    pub reserved2: __u16,
    pub reserved3: __u32,
    pub reserved4: __u64,
// C attribute field omitted
    pub device: __u8,
    pub lun: __u8,
    pub reserved1: __u8,
    pub reserved2: __u8,
    pub reserved3: __u32,
    pub reserved4: __u64,
// C attribute field omitted
    pub id: __u16,
    pub lun: __u64,
    pub reserved1: __u16,
    pub reserved2: __u32,
// C attribute field omitted
    pub serial_number: __u64,
    pub reserved: __u64,
// C attribute field omitted
    pub eui: __u64,
    pub reserved: __u64,
// C attribute field omitted
    pub wwid: __u64,
    pub lun: __u64,
// C attribute field omitted
    pub identity_tag: __u64,
    pub reserved: __u64,
// C attribute field omitted
    pub array_number: __u32,
    pub reserved1: __u32,
    pub reserved2: __u64,
// C attribute field omitted
    pub device: __u8,
    pub reserved1: __u8,
    pub reserved2: __u16,
    pub reserved3: __u32,
    pub reserved4: __u64,
// C attribute field omitted
    pub reserved1: __u64,
    pub reserved2: __u64,
// C attribute field omitted
    pub device_path: },
    pub reserved4: __u8,
    pub checksum: __u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edd_info {
    pub device: __u8,
    pub version: __u8,
    pub interface_support: __u16,
    pub legacy_max_cylinder: __u16,
    pub legacy_max_head: __u8,
    pub legacy_sectors_per_track: __u8,
    pub params: edd_device_params,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edd {
    pub mbr_signature: [c_uint; EDD_MBR_SIG_MAX],
    pub edd_info: [edd_info; EDDMAXNR],
    pub mbr_signature_nr: c_uchar,
    pub edd_info_nr: c_uchar,
}


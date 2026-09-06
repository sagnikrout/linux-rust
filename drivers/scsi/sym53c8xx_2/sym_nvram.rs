//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/sym53c8xx_2/sym_nvram.h
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
// Device driver for the SYMBIOS/LSILOGIC 53C8XX and 53C1010 family
// of PCI-SCSI IO processors.
//
// Copyright (C) 1999-2001  Gerard Roudier <groudier@free.fr>
//
// This driver is derived from the Linux sym53c8xx driver.
// Copyright (C) 1998-2000  Gerard Roudier
//
// The sym53c8xx driver is derived from the ncr53c8xx driver that had been
// a port of the FreeBSD ncr driver to Linux-1.2.13.
//
// The original ncr driver has been written for 386bsd and FreeBSD by
// Wolfgang Stanglmeier        <wolf@cologne.de>
// Stefan Esser                <se@mi.Uni-Koeln.de>
// Copyright (C) 1994  Wolfgang Stanglmeier
//
// Other major contributions:
//
// NVRAM detection and reading.
// Copyright (C) 1997 Richard Waltham <dormouse@farsrobt.demon.co.uk>
//
// -----------------------------------------------------------------------------
//

//
// Symbios NVRAM data format
//
pub const SYMBIOS_NVRAM_SIZE: c_int = 368;
pub const SYMBIOS_NVRAM_ADDRESS: c_uint = 0x100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Symbios_nvram {
// Header 6 bytes
    pub /: *mut *mut u_short type; / 0x0000,
    pub /: *mut *mut u_short byte_count; / excluding header/trailer,
    pub checksum: u_short,
// Controller set up 20 bytes
    pub /: *mut *mut u_char v_major; / 0x00,
    pub /: *mut *mut u_char v_minor; / 0x30,
    pub boot_crc: u32,
    pub flags: u_short,

    pub flags1: u_short,

    pub term_state: u_short,

    pub rmvbl_flags: u_short,

    pub host_id: u_char,
    pub /: *mut *mut u_char num_hba; / 0x04,
    pub /: *mut *mut u_char num_devices; / 0x10,
    pub /: *mut *mut u_char max_scam_devices; / 0x04,
    pub /: *mut *mut u_char num_valid_scam_devices; / 0x00,
    pub flags2: u_char,

// Boot order 14 bytes * 4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Symbios_host {
    pub /: *mut *mut u_short type; / 4:8xx / 0:nok,
    pub /: *mut *mut u_short device_id; / PCI device id,
    pub /: *mut *mut u_short vendor_id; / PCI vendor id,
    pub /: *mut *mut u_char bus_nr; / PCI bus number,
    pub 3*/: *mut *mut u_char device_fn; / PCI device/function number <<,
    pub word8: u_short,
    pub flags: u_short,

    pub /: *mut *mut u_short io_port; / PCI io_port address,
    pub host: [}; 4],
// Targets 8 bytes * 16
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Symbios_target {
    pub flags: u_char,

    pub rsvd: u_char,
    pub /: *mut *mut u_char bus_width; / 0x08/0x10,
    pub sync_offset: u_char,
    pub /: *mut *mut *mut u_short sync_period; / 4period factor,
    pub timeout: u_short,
    pub target: [}; 16],
// Scam table 8 bytes * 4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Symbios_scam {
    pub id: u_short,
    pub method: u_short,

    pub status: u_short,

    pub target_id: u_char,
    pub rsvd: u_char,
    pub scam: [}; 4],
    pub spare_devices: [*mut u_char; 15*8],
    pub /: *mut *mut u_char trailer[6]; / 0xfe 0xfe 0x00 0x00 0x00 0x00,
}

pub type Symbios_nvram = Symbios_nvram;
pub type Symbios_host = Symbios_host;
pub type Symbios_target = Symbios_target;
pub type Symbios_scam = Symbios_scam;
//
// Tekram NvRAM data format.
//
pub const TEKRAM_NVRAM_SIZE: c_int = 64;
pub const TEKRAM_93C46_NVRAM_ADDRESS: c_int = 0;
pub const TEKRAM_24C16_NVRAM_ADDRESS: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Tekram_nvram {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Tekram_target {
    pub flags: u_char,

    pub sync_index: u_char,
    pub word2: u_short,
    pub target: [}; 16],
    pub host_id: u_char,
    pub flags: u_char,

// 1: boot device; 2:all
    pub boot_delay_index: u_char,
    pub max_tags_index: u_char,
    pub flags1: u_short,
    pub spare: [u_short; 29],
}

pub type Tekram_nvram = Tekram_nvram;
pub type Tekram_target = Tekram_target;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdc_initiator {

//
// Union of supported NVRAM formats.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_nvram {
    pub type: c_int,

    pub Symbios: Symbios_nvram,
    pub Tekram: Tekram_nvram,
    pub parisc: pdc_initiator,
    pub data: },

}

extern "C" {
    pub fn sym_nvram_setup_host(shost: *mut Scsi_Host, np: *mut sym_hcb, nvram: *mut sym_nvram);
}
extern "C" {
    pub fn sym_nvram_setup_target(tp: *mut sym_tcb, target: c_int, nvp: *mut sym_nvram);
}
extern "C" {
    pub fn sym_read_nvram(np: *mut sym_device, nvp: *mut sym_nvram) -> c_int;
}


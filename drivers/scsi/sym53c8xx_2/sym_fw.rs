//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/sym53c8xx_2/sym_fw.h
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
// Macro used to generate interfaces for script A.
//

//
// Macro used to generate interfaces for script B.
//

//
// Macro used to generate interfaces for script Z.
//

//
// Generates structure interface that contains
// offsets within script A, B and Z.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_fwa_ofs {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_fwb_ofs {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_fwz_ofs {
}

//
// Generates structure interface that contains
// bus addresses within script A, B and Z.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_fwa_ba {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_fwb_ba {
    pub start64): SYM_GEN_B(u32,,
    pub pm_handle): SYM_GEN_B(u32,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_fwz_ba {
}

//
// Let cc know about the name of the controller data structure.
// We need this for function prototype declarations just below.
//
// Generic structure that defines a firmware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_fw {
    pub /: *mut *mut *mut char name; / Name we want to print out,
    pub /: *mut *mut *mut u32 a_base; / Pointer to script A template,
    pub /: *mut *mut int a_size; / Size of script A,
// a_ofs;		/* Useful offsets in script A
    pub /: *mut *mut *mut u32 b_base; / Pointer to script B template,
    pub /: *mut *mut int b_size; / Size of script B,
// b_ofs;		/* Useful offsets in script B
    pub /: *mut *mut *mut u32 z_base; / Pointer to script Z template,
    pub /: *mut *mut int z_size; / Size of script Z,
// z_ofs;		/* Useful offsets in script Z
// Setup and patch methods for this firmware
    pub ): *mut *mut *mut void (setup)(struct sym_hcb , struct sym_fw,
    pub ): *mut *mut void (patch)(struct Scsi_Host,
}

//
// Macro used to declare a firmware.
//

//
// Macros used from the C code to get useful
// SCRIPTS bus addresses.
//

//
// Macros used by scripts definitions.
//
// HADDR_1 generates a reference to a field of the controller data.
// HADDR_2 generates a reference to a field of the controller data
// with offset.
// RADDR_1 generates a reference to a script processor register.
// RADDR_2 generates a reference to a script processor register
// with offset.
// PADDR_A generates a reference to another part of script A.
// PADDR_B generates a reference to another part of script B.
//
// SYM_GEN_PADDR_A and SYM_GEN_PADDR_B are used to define respectively
// the PADDR_A and PADDR_B macros for each firmware by setting argument
// `s' to the name of the corresponding structure.
//
// SCR_DATA_ZERO is used to allocate a DWORD of data in scripts areas.
//
pub const RELOC_SOFTC: c_uint = 0x40000000;
pub const RELOC_LABEL_A: c_uint = 0x50000000;
pub const RELOC_REGISTER: c_uint = 0x60000000;
pub const RELOC_LABEL_B: c_uint = 0x80000000;
pub const RELOC_MASK: c_uint = 0xf0000000;

pub const SCR_DATA_ZERO: c_uint = 0xf00ff00f;

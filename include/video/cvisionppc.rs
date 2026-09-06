//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/cvisionppc.h
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
// Phase5 CybervisionPPC (TVP4020) definitions for the Permedia2 framebuffer
// driver.
//
// Copyright (c) 1998-1999 Ilario Nardinocchi (nardinoc@CS.UniBO.IT)
// --------------------------------------------------------------------------
// $Id: cvisionppc.h,v 1.8 1999/01/28 13:18:07 illo Exp $
// --------------------------------------------------------------------------
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvppc_par {
    pub pci_config: *mut *mut c_uchar,
    pub pci_bridge: *mut *mut c_uchar,
    pub user_flags: u32,
}

pub const CSPPC_PCI_BRIDGE: c_uint = 0xfffe0000;
pub const CSPPC_BRIDGE_ENDIAN: c_uint = 0x0000;
pub const CSPPC_BRIDGE_INT: c_uint = 0x0010;
pub const CVPPC_PCI_CONFIG: c_uint = 0xfffc0000;
pub const CVPPC_ROM_ADDRESS: c_uint = 0xe2000001;
pub const CVPPC_REGS_REGION: c_uint = 0xef000000;
pub const CVPPC_FB_APERTURE_ONE: c_uint = 0xe0000000;
pub const CVPPC_FB_APERTURE_TWO: c_uint = 0xe1000000;
pub const CVPPC_FB_SIZE: c_uint = 0x00800000;
pub const CVPPC_MEM_CONFIG_OLD: c_uint = 0xed61fcaa	/* FIXME Fujitsu?? */;
pub const CVPPC_MEM_CONFIG_NEW: c_uint = 0xed41c532	/* FIXME USA?? */;

// CVPPC_BRIDGE_ENDIAN
pub const CSPPCF_BRIDGE_BIG_ENDIAN: c_uint = 0x02;
// CVPPC_BRIDGE_INT
pub const CSPPCF_BRIDGE_ACTIVE_INT2: c_uint = 0x01;

//
// That's all folks!
//

//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/atmdev.h
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
// atmdev.h - ATM device driver declarations and various related items
// Written 1995-2000 by Werner Almesberger, EPFL LRC/ICA

pub const ESI_LEN: c_int = 6;

// OC3 link rate:  155520000 bps

// 25 Mbps ATM cell rate (59111)

// OC12 link rate: 622080000 bps

// DS3: 12 cells in a 125 usec time slot

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_aal_stats {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_dev_stats {
    pub aal0: atm_aal_stats,
    pub aal34: atm_aal_stats,
    pub aal5: atm_aal_stats,
    pub __ATM_API_ALIGN: },

// get link rate

// get interface names (numbers)

// get interface type name

// get interface ESI

// get connection identifier range

// set connection identifier range

// set interface ESI

// force interface ESI

// get AAL layer statistics

// get AAL layer statistics and zero

// get loopback mode

// set loopback mode

// query supported loopback modes

// enable or disable single-copy

// set backend handler

// use backend to make new if
//
// These are backend handkers that can be set via the ATM_SETBACKEND call
// above.  In the future we may support dynamic loading of these - for now,
// they're just being used to share the ATMIOC_BACKEND ioctls
//
pub const ATM_BACKEND_RAW: c_int = 0;

// for ATM_GETTYPE

//
// Loopback modes for ATM_{PHY,SAR}_{GET,SET}LOOP
//
// Point of loopback				CPU-->SAR-->PHY-->line--> ...

// RESERVED		4	loop back on PHY side  ---'

// Direction of loopback

//
// Note: ATM_LM_LOC_* and ATM_LM_RMT_* can be combined, provided that
// __ATM_LM_XTLOC(x) <= __ATM_LM_XTRMT(x)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_iobuf {
    pub length: c_int,
    pub buffer: *mut void __user,
}

// for ATM_GETCIRANGE / ATM_SETCIRANGE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_cirange {
    pub /: *mut *mut signed char vpi_bits; / 1..8, ATM_CI_MAX (-1) for maximum,
    pub /: *mut *mut signed char vci_bits; / 1..16, ATM_CI_MAX (-1) for maximum,
}

// for ATM_SETSC; actually taken from the ATM_VF number space

// MF: change_qos (Modify) flags

//
// ATM_VS_* are used to express VC state in a human-friendly way.
//


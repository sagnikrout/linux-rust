//! Automatically rewritten from C Header to Rust Module
//! Source: fs/isofs/rock.h
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


// SPDX-License-Identifier: GPL-2.0
//
// These structs are used by the system-use-sharing protocol, in which the
// Rock Ridge extensions are embedded.  It is quite possible that other
// extensions are present on the disk, and this is fine as long as they
// all use SUSP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SU_SP_s {
    pub magic: [__u8; 2],
    pub skip: __u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SU_CE_s {
    pub extent: [__u8; 8],
    pub offset: [__u8; 8],
    pub size: [__u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SU_ER_s {
    pub len_id: __u8,
    pub len_des: __u8,
    pub len_src: __u8,
    pub ext_ver: __u8,
    pub data: [__u8; ],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RR_RR_s {
    pub flags: [__u8; 1],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RR_PX_s {
    pub mode: [__u8; 8],
    pub n_links: [__u8; 8],
    pub uid: [__u8; 8],
    pub gid: [__u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RR_PN_s {
    pub dev_high: [__u8; 8],
    pub dev_low: [__u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SL_component {
    pub flags: __u8,
    pub len: __u8,
    pub __counted_by(len): __u8 text[],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RR_SL_s {
    pub flags: __u8,
    pub link: SL_component,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RR_NM_s {
    pub flags: __u8,
    pub name: [c_char; ],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RR_CL_s {
    pub location: [__u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RR_PL_s {
    pub location: [__u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RR_TF_s {
    pub flags: __u8,
    pub data: [__u8; ],
// C attribute field omitted
// Linux-specific extension for transparent decompression
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RR_ZF_s {
    pub algorithm: [__u8; 2],
    pub parms: [__u8; 2],
    pub real_size: [__u8; 8],
}

//
// These are the bits and their meanings for flags in the TF structure.
//
pub const TF_CREATE: c_int = 1;
pub const TF_MODIFY: c_int = 2;
pub const TF_ACCESS: c_int = 4;
pub const TF_ATTRIBUTES: c_int = 8;
pub const TF_BACKUP: c_int = 16;
pub const TF_EXPIRATION: c_int = 32;
pub const TF_EFFECTIVE: c_int = 64;
pub const TF_LONG_FORM: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rock_ridge {
    pub signature: [__u8; 2],
    pub len: __u8,
    pub version: __u8,
    pub SP: SU_SP_s,
    pub CE: SU_CE_s,
    pub ER: SU_ER_s,
    pub RR: RR_RR_s,
    pub PX: RR_PX_s,
    pub PN: RR_PN_s,
    pub SL: RR_SL_s,
    pub NM: RR_NM_s,
    pub CL: RR_CL_s,
    pub PL: RR_PL_s,
    pub TF: RR_TF_s,
    pub ZF: RR_ZF_s,
    pub u: },
}


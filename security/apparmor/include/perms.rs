//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/perms.h
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
// AppArmor security module
//
// This file contains AppArmor basic permission sets definitions.
//
// Copyright 2017 Canonical Ltd.
//

pub const AA_MAY_CREATE: c_uint = 0x0010;
pub const AA_MAY_DELETE: c_uint = 0x0020;
pub const AA_MAY_OPEN: c_uint = 0x0040;
pub const AA_MAY_RENAME: c_uint = 0x0080		/* pair */;
pub const AA_MAY_SETATTR: c_uint = 0x0100		/* meta write */;
pub const AA_MAY_GETATTR: c_uint = 0x0200		/* meta read */;
pub const AA_MAY_SETCRED: c_uint = 0x0400		/* security cred/attr */;
pub const AA_MAY_GETCRED: c_uint = 0x0800;
pub const AA_MAY_CHMOD: c_uint = 0x1000		/* pair */;
pub const AA_MAY_CHOWN: c_uint = 0x2000		/* pair */;
pub const AA_MAY_CHGRP: c_uint = 0x4000		/* pair */;
pub const AA_MAY_LOCK: c_uint = 0x8000		/* LINK_SUBSET overlaid */;
pub const AA_EXEC_MMAP: c_uint = 0x00010000;
pub const AA_MAY_MPROT: c_uint = 0x00020000	/* extend conditions */;
pub const AA_MAY_LINK: c_uint = 0x00040000	/* pair */;
pub const AA_MAY_SNAPSHOT: c_uint = 0x00080000	/* pair */;
// Macro flag: #define AA_MAY_DELEGATE
pub const AA_CONT_MATCH: c_uint = 0x08000000;
pub const AA_MAY_STACK: c_uint = 0x10000000;
pub const AA_MAY_ONEXEC: c_uint = 0x20000000 /* either stack or change_profile */;
pub const AA_MAY_CHANGE_PROFILE: c_uint = 0x40000000;
pub const AA_MAY_CHANGEHAT: c_uint = 0x80000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_perms {
    pub allow: u32,
    pub /: *mut *mut u32 deny; / explicit deny, or conflict if allow also set,
    pub /: *mut *mut u32 subtree; / allow perm on full subtree only when allow is set,
    pub /: *mut *mut u32 cond; / set only when ~allow and ~deny,
    pub /: *mut *mut u32 kill; / set only when ~allow | deny,
    pub /: *mut *mut u32 complain; / accumulates only used when ~allow & ~deny,
    pub /: *mut *mut u32 prompt; / accumulates only used when ~allow & ~deny,
    pub /: *mut *mut u32 audit; / set only when allow is set,
    pub /: *mut *mut u32 quiet; / set only when ~allow | deny,
    pub /: *mut *mut u32 hide; / set only when ~allow | deny,
    pub xindex: u32,
    pub /: *mut *mut u32 tag; / tag string index, if present,
    pub /: *mut *mut u32 label; / label string index, if present,
}

//
// Indexes are broken into a 24 bit index and 8 bit flag.
// For the index to be valid there must be a value in the flag
//
pub const AA_INDEX_MASK: c_uint = 0x00ffffff;
pub const AA_INDEX_FLAG_MASK: c_uint = 0xff000000;
pub const AA_INDEX_NONE: c_int = 0;
pub const ALL_PERMS_MASK: c_uint = 0xffffffff;
//
// aa_perms_accum_raw - accumulate perms with out masking off overlapping perms
// @accum: perms struct to accumulate into
// @addend: perms struct to add to @accum
//
// aa_perms_accum - accumulate perms, masking off overlapping perms
// @accum: perms struct to accumulate into
// @addend: perms struct to add to @accum
//

//
// TODO: update for labels pointing to labels instead of profiles
// TODO: optimize the walk, currently does subwalk of L2 for each P in L1
// gah this doesn't allow for label compound check!!!!
//

// Do the cross check but applying FN at the profiles level

extern "C" {
    pub fn aa_perms_accum(accum: *mut aa_perms, addend: *const aa_perms);
}
extern "C" {
    pub fn aa_perms_accum_raw(accum: *mut aa_perms, addend: *const aa_perms);
}

//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/match.h
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
// This file contains AppArmor policy dfa matching engine definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2012 Canonical Ltd.
//

pub const DFA_NOMATCH: c_int = 0;
pub const DFA_START: c_int = 1;
//
// The format used for transition tables is based on the GNU flex table
// file format (--tables-file option; see Table File Format in the flex
// info pages and the flex sources for documentation). The magic number
// used in the header is 0x1B5E783D instead of 0xF13C57B1 though, because
// new tables have been defined and others YY_ID_CHK (check) and YY_ID_DEF
// (default) tables are used slightly differently (see the apparmor-parser
// package).
//
// The data in the packed dfa is stored in network byte order, and the tables
// are arranged for flexibility.  We convert the table data to host native
// byte order.
//
// The dfa begins with a table set header, and is followed by the actual
// tables.
//
pub const YYTH_MAGIC: c_uint = 0x1B5E783D;
pub const YYTH_FLAG_DIFF_ENCODE: c_int = 1;
pub const YYTH_FLAG_OOB_TRANS: c_int = 2;

pub const MAX_OOB_SUPPORTED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct table_set_header {
    pub /: *mut *mut u32 th_magic; / YYTH_MAGIC,
    pub th_hsize: u32,
    pub th_ssize: u32,
    pub th_flags: u16,
    pub th_version: [c_char; ],
}

// The YYTD_ID are one less than flex table mappings.  The flex id
// has 1 subtracted at table load time, this allows us to directly use the
// ID's as indexes.
//
pub const YYTD_ID_ACCEPT: c_int = 0;
pub const YYTD_ID_BASE: c_int = 1;
pub const YYTD_ID_CHK: c_int = 2;
pub const YYTD_ID_DEF: c_int = 3;
pub const YYTD_ID_EC: c_int = 4;
pub const YYTD_ID_META: c_int = 5;
pub const YYTD_ID_ACCEPT2: c_int = 6;
pub const YYTD_ID_NXT: c_int = 7;
pub const YYTD_ID_TSIZE: c_int = 8;
pub const YYTD_ID_MAX: c_int = 8;
pub const YYTD_DATA8: c_int = 1;
pub const YYTD_DATA16: c_int = 2;
pub const YYTD_DATA32: c_int = 4;
pub const YYTD_DATA64: c_int = 8;
// ACCEPT & ACCEPT2 tables gets 6 dedicated flags, YYTD_DATAX define the
// first flags
//

pub const DFA_FLAG_VERIFY_STATES: c_uint = 0x1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct table_header {
    pub td_id: u16,
    pub td_flags: u16,
    pub td_hilen: u32,
    pub td_lolen: u32,
    pub td_data: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_dfa {
    pub count: kref,
    pub flags: u16,
    pub max_oob: u32,
    pub tables: [*mut table_header; YYTD_ID_TSIZE],
}

extern "C" {
    pub fn ALIGN(el_size: *mut *mut sizeof(struct table_header) + len, _arg: 8) -> return;
}

extern "C" {
    pub fn aa_dfa_free_kref(kref: *mut kref);
}
// This needs to be a power of 2
pub const WB_HISTORY_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct match_workbuf {
    pub pos: c_uint,
    pub len: c_uint,
    pub history: [aa_state_t; WB_HISTORY_SIZE],
}

//
// aa_get_dfa - increment refcount on dfa @p
// @dfa: dfa  (MAYBE NULL)
//
// Returns: pointer to @dfa if @dfa is NULL will return NULL
// Requires: @dfa must be held with valid refcount when called
//
// aa_put_dfa - put a dfa refcount
// @dfa: dfa to put refcount   (MAYBE NULL)
//
// Requires: if @dfa != NULL that a valid refcount be held
//
pub const MATCH_FLAG_DIFF_ENCODE: c_uint = 0x80000000;
pub const MARK_DIFF_ENCODE: c_uint = 0x40000000;
pub const MATCH_FLAG_OOB_TRANSITION: c_uint = 0x20000000;
pub const MARK_DIFF_ENCODE_VERIFIED: c_uint = 0x10000000;
pub const MATCH_FLAGS_MASK: c_uint = 0xff000000;


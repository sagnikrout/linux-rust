//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fs_parser.h
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
// Filesystem parameter description and parser
//
// Copyright (C) 2018 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct constant_table {
    pub name: *const c_char,
    pub value: c_int,
}

//
// The type of parameter expected.
//
// Specification of the type of value a parameter wants.
//
// Note that the fsparam_flag(), fsparam_string(), fsparam_u32(), ... macros
// should be used to generate elements of this type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_parameter_spec {
    pub name: *const c_char,
    pub /: *mut *mut *mut fs_param_type type; / The desired parameter type,
    pub /: *mut *mut u8 opt; / Option number (returned by fs_parse()),
    pub flags: c_ushort,
pub const fs_param_neg_with_no: c_uint = 0x0002	/* "noxxx" is negative param */;
pub const fs_param_can_be_empty: c_uint = 0x0004	/* "xxx=" is allowed */;
pub const fs_param_deprecated: c_uint = 0x0008	/* The param is deprecated */;
    pub data: *const c_void,
}

//
// Result of parse.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_parse_result {
    pub /: *mut *mut bool negated; / T if param was "noxxx",
    pub /: *mut *mut bool boolean; / For spec_bool,
    pub /: *mut *mut int int_32; / For spec_s32/spec_enum,
    pub /: *mut *mut unsigned int uint_32; / For spec_u32{,_octal,_hex}/spec_enum,
    pub /: *mut *mut u64 uint_64; / For spec_u64,
    pub uid: kuid_t,
    pub gid: kgid_t,
}

extern "C" {
    pub fn __fs_parse(_arg: &fc->log, _arg: desc, _arg: param, _arg: result) -> return;
}
extern "C" {
    pub fn lookup_constant(tbl[]: constant_table, name: *const c_char, not_found: c_int) -> c_int;
}

//
// Parameter type, name, index and flags element constructors.  Use as:
//
// fsparam_xxxx("foo", Opt_foo)
//
// If existing helpers are not enough, direct use of __fsparam() would
// work, but any such case is probably a sign that new helper is needed.
// Helpers will remain stable; low-level implementation may change.
//

// String parameter that allows empty argument


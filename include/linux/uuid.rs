//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/uuid.h
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
// UUID/GUID definition
//
// Copyright (C) 2010, 2016 Intel Corp.
// Huang Ying <ying.huang@intel.com>
//

pub const UUID_SIZE: c_int = 16;

//
// The length of a UUID string ("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee")
// not including trailing NUL.
//
pub const UUID_STRING_LEN: c_int = 36;
extern "C" {
    pub fn guid_equal(_arg: guid, _arg: &guid_null) -> return;
}
extern "C" {
    pub fn uuid_equal(_arg: uuid, _arg: &uuid_null) -> return;
}
extern "C" {
    pub fn generate_random_uuid(uuid[16]: c_uchar);
}
extern "C" {
    pub fn generate_random_guid(guid[16]: c_uchar);
}
extern "C" {
    pub fn guid_gen(u: *mut guid_t);
}
extern "C" {
    pub fn uuid_gen(u: *mut uuid_t);
}
extern "C" {
    pub fn uuid_is_valid(uuid: *const c_char) -> bool __must_check;
}
extern "C" {
    pub fn guid_parse(uuid: *const c_char, u: *mut guid_t) -> c_int;
}
extern "C" {
    pub fn uuid_parse(uuid: *const c_char, u: *mut uuid_t) -> c_int;
}

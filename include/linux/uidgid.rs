//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/uidgid.h
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
// A set of types for the internal kernel types representing uids and gids.
//
// The types defined in this header allow distinguishing which uids and gids in
// the kernel are values used by userspace and which uid and gid values are
// the internal kernel values.  With the addition of user namespaces the values
// can be different.  Using the type system makes it possible for the compiler
// to detect when we overlook these differences.
//

extern "C" {
    pub fn __kuid_val(__kuid_val(right: left) ==) -> return;
}
extern "C" {
    pub fn __kgid_val(__kgid_val(right: left) ==) -> return;
}
extern "C" {
    pub fn __kuid_val(__kuid_val(right: left) >) -> return;
}
extern "C" {
    pub fn __kgid_val(__kgid_val(right: left) >) -> return;
}
extern "C" {
    pub fn __kuid_val(__kuid_val(right: left) >=) -> return;
}
extern "C" {
    pub fn __kgid_val(__kgid_val(right: left) >=) -> return;
}
extern "C" {
    pub fn __kuid_val(__kuid_val(right: left) <) -> return;
}
extern "C" {
    pub fn __kgid_val(__kgid_val(right: left) <) -> return;
}
extern "C" {
    pub fn __kuid_val(__kuid_val(right: left) <=) -> return;
}
extern "C" {
    pub fn __kgid_val(__kgid_val(right: left) <=) -> return;
}

extern "C" {
    pub fn make_kuid(from: *mut user_namespace, uid: uid_t) -> kuid_t;
}
extern "C" {
    pub fn make_kgid(from: *mut user_namespace, gid: gid_t) -> kgid_t;
}
extern "C" {
    pub fn from_kuid(to: *mut user_namespace, uid: kuid_t) -> uid_t;
}
extern "C" {
    pub fn from_kgid(to: *mut user_namespace, gid: kgid_t) -> gid_t;
}
extern "C" {
    pub fn from_kuid_munged(to: *mut user_namespace, uid: kuid_t) -> uid_t;
}
extern "C" {
    pub fn from_kgid_munged(to: *mut user_namespace, gid: kgid_t) -> gid_t;
}
extern "C" {
    pub fn map_id_down(map: *mut uid_gid_map, id: u32) -> u32;
}
extern "C" {
    pub fn map_id_up(map: *mut uid_gid_map, id: u32) -> u32;
}
extern "C" {
    pub fn map_id_range_up(map: *mut uid_gid_map, id: u32, count: u32) -> u32;
}

extern "C" {
    pub fn KUIDT_INIT(_arg: uid) -> return;
}
extern "C" {
    pub fn KGIDT_INIT(_arg: gid) -> return;
}
extern "C" {
    pub fn __kuid_val(_arg: kuid) -> return;
}
extern "C" {
    pub fn __kgid_val(_arg: kgid) -> return;
}
extern "C" {
    pub fn uid_valid(_arg: uid) -> return;
}
extern "C" {
    pub fn gid_valid(_arg: gid) -> return;
}


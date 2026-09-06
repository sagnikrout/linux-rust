//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/ss/ebitmap.h
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
// An extensible bitmap is a bitmap that supports an
// arbitrary number of bits.  Extensible bitmaps are
// used to represent sets of values, such as types,
// roles, categories, and classes.
//
// Each extensible bitmap is implemented as a linked
// list of bitmap nodes, where each bitmap node has
// an explicitly specified starting bit position within
// the total bitmap.
//
// Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
//

pub const EBITMAP_NODE_SIZE: c_int = 64;

pub const EBITMAP_NODE_SIZE: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebitmap_node {
    pub next: *mut ebitmap_node,
    pub maps: [c_ulong; EBITMAP_UNIT_NUMS],
    pub startbit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebitmap {
    pub /: *mut *mut *mut ebitmap_node node; / first node in the bitmap,
    pub /: *mut *mut u32 highbit; / highest position in the total bitmap,
}

extern "C" {
    pub fn ebitmap_length(_arg: e) -> return;
}
extern "C" {
    pub fn ebitmap_length(_arg: e) -> return;
}

extern "C" {
    pub fn ebitmap_equal(e1: *const ebitmap, e2: *const ebitmap) -> bool;
}
extern "C" {
    pub fn ebitmap_cpy(dst: *mut ebitmap, src: *const ebitmap) -> c_int;
}
extern "C" {
    pub fn ebitmap_get_highest_set_bit(e: *const ebitmap) -> u32;
}
extern "C" {
    pub fn ebitmap_get_bit(e: *const ebitmap, bit: u32) -> c_int;
}
extern "C" {
    pub fn ebitmap_set_bit(e: *mut ebitmap, bit: u32, value: c_int) -> c_int;
}
extern "C" {
    pub fn ebitmap_destroy(e: *mut ebitmap);
}
extern "C" {
    pub fn ebitmap_read(e: *mut ebitmap, fp: *mut policy_file) -> c_int;
}
extern "C" {
    pub fn ebitmap_write(e: *const ebitmap, fp: *mut policy_file) -> c_int;
}
extern "C" {
    pub fn ebitmap_hash(e: *const ebitmap, hash: u32) -> u32;
}


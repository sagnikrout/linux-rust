//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/types.h
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

extern "C" {
    pub fn __attribute__(_arg: (aligned(16))) -> typedef __signed__ __int128 __s128;
}
extern "C" {
    pub fn __attribute__(_arg: (aligned(16))) -> typedef unsigned __int128 __u128;
}

//
// We define u64 as uint64_t for every architecture
// so that we can print it with "%"PRIx64 without getting warnings.
//
// typedef __u64 u64;
// typedef __s64 s64;
//
pub type u64 = u64;
pub type s64 = i64;
pub type u32 = __u32;
pub type s32 = __s32;
pub type u16 = __u16;
pub type s16 = __s16;
pub type u8 = __u8;
pub type s8 = __s8;
pub type ullong = c_ulonglong;

// Macro flag: #define 

// Macro flag: #define 
// This is defined in linux/compiler_types.h and is left for backward
// compatibility.
//

// Macro flag: #define __user

// Macro flag: #define __must_check
// Macro flag: #define __cold
pub type __le16 = __u16 ;
pub type __be16 = __u16 ;
pub type __le32 = __u32 ;
pub type __be32 = __u32 ;
pub type __le64 = __u64 ;
pub type __be64 = __u64 ;
pub type __sum16 = __u16 ;
pub type __wsum = __u32 ;

pub type phys_addr_t = u64;

pub type phys_addr_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub prev: *mut *mut list_head next,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_node {
    pub pprev: *mut *mut hlist_node next,,
}

//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfilter/nf_conntrack_proto_gre.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_gre {
    pub stream_timeout: c_uint,
    pub timeout: c_uint,
}

// structure for original <-> reply keymap
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_gre_keymap {
    pub list: list_head,
    pub tuple: nf_conntrack_tuple,
    pub rcu: rcu_head,
}

// add tuple->key_reply pairs to keymap
// delete keymap entries
extern "C" {
    pub fn nf_ct_gre_keymap_destroy(ct: *mut nf_conn);
}

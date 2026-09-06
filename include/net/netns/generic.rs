//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netns/generic.h
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
// generic net pointers
//

//
// Generic net pointers are to be used by modules to put some private
// stuff on the struct net without explicit struct net modification
//
// The rules are simple:
// 1. set pernet_operations->id.  After register_pernet_device you
// will have the id of your private pointer.
// 2. set pernet_operations->size to have the code allocate and free
// a private structure pointed to from struct net.
// 3. do not change this pointer while the net is alive;
// 4. do not try to have any private reference on the net_generic object.
//
// After accomplishing all of the above, the private pointer can be
// accessed with the net_generic() call.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_generic {
    pub len: c_uint,
    pub rcu: rcu_head,
    pub s: },
    pub ptr): *mut *mut DECLARE_FLEX_ARRAY(void ,,
}

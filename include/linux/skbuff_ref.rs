//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/skbuff_ref.h
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
//
// Skb ref helpers.
//

//
// __skb_frag_ref - take an addition reference on a paged fragment.
// @frag: the paged fragment
//
// Takes an additional reference on the paged fragment @frag.
//
// skb_frag_ref - take an addition reference on a paged fragment of an skb.
// @skb: the buffer
// @f: the fragment offset.
//
// Takes an additional reference on the @f'th paged fragment of @skb.
//
extern "C" {
    pub fn napi_pp_put_page(netmem: netmem_ref) -> bool;
}

//
// __skb_frag_unref - release a reference on a paged fragment.
// @frag: the paged fragment
// @recycle: recycle the page if allocated via page_pool
//
// Releases a reference on the paged fragment @frag
// or recycles the page via the page_pool API.
//
// skb_frag_unref - release a reference on a paged fragment of an skb.
// @skb: the buffer
// @f: the fragment offset
//
// Releases a reference on the @f'th paged fragment of @skb.
//

//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/dst_cache.h
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
pub struct dst_cache {
    pub cache: *mut dst_cache_pcpu __percpu,
    pub reset_ts: c_ulong,
}

//
// dst_cache_get - perform cache lookup
// @dst_cache: the cache
//
// The caller should use dst_cache_get_ip4() if it need to retrieve the
// source address to be used when xmitting to the cached dst.
// local BH must be disabled.
//
// dst_cache_get_ip4 - perform cache lookup and fetch ipv4 source address
// @dst_cache: the cache
// @saddr: return value for the retrieved source address
//
// local BH must be disabled.
//
// dst_cache_set_ip4 - store the ipv4 dst into the cache
// @dst_cache: the cache
// @dst: the entry to be cached
// @saddr: the source address to be stored inside the cache
//
// local BH must be disabled.
//

//
// dst_cache_set_ip6 - store the ipv6 dst into the cache
// @dst_cache: the cache
// @dst: the entry to be cached
// @saddr: the source address to be stored inside the cache
//
// local BH must be disabled.
//
// dst_cache_get_ip6 - perform cache lookup and fetch ipv6 source address
// @dst_cache: the cache
// @saddr: return value for the retrieved source address
//
// local BH must be disabled.
//

//
// dst_cache_reset - invalidate the cache contents
// @dst_cache: the cache
//
// This does not free the cached dst to avoid races and contentions.
// the dst will be freed on later cache lookup.
//
// dst_cache_reset_now - invalidate the cache contents immediately
// @dst_cache: the cache
//
// The caller must be sure there are no concurrent users, as this frees
// all dst_cache users immediately, rather than waiting for the next
// per-cpu usage like dst_cache_reset does. Most callers should use the
// higher speed lazily-freed dst_cache_reset function instead.
//
extern "C" {
    pub fn dst_cache_reset_now(dst_cache: *mut dst_cache);
}
//
// dst_cache_init - initialize the cache, allocating the required storage
// @dst_cache: the cache
// @gfp: allocation flags
//
extern "C" {
    pub fn dst_cache_init(dst_cache: *mut dst_cache, gfp: gfp_t) -> c_int;
}
//
// dst_cache_destroy - empty the cache and free the allocated storage
// @dst_cache: the cache
//
// No synchronization is enforced: it must be called only when the cache
// is unused.
//
extern "C" {
    pub fn dst_cache_destroy(dst_cache: *mut dst_cache);
}

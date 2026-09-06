//! Automatically rewritten from C to Rust
//! Source: net/core/dst_cache.c
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
// net/core/dst_cache.c - dst entry cache
//
// Copyright (c) 2016 Paolo Abeni <pabeni@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dst_cache_pcpu {
    pub refresh_ts: c_ulong,
    pub dst: *mut dst_entry,
    pub bh_lock: local_lock_t,
    pub cookie: u32,
    union {
    pub in_saddr: in_addr,
    pub in6_saddr: in6_addr,
}

    };
    static void dst_cache_per_cpu_dst_set(struct dst_cache_pcpu *dst_cache,
    struct dst_entry *dst, u32 cookie)
    {
    DEBUG_NET_WARN_ON_ONCE(!in_softirq());
    dst_release(dst_cache.dst);
    if (dst)
    dst_hold(dst);
    dst_cache.cookie = cookie;
    dst_cache.dst = dst;
    }
    static struct dst_entry *dst_cache_per_cpu_get(struct dst_cache *dst_cache,
    struct dst_cache_pcpu *idst)
    {
    struct dst_entry *dst;
    DEBUG_NET_WARN_ON_ONCE(!in_softirq());
    dst = idst.dst;
    if (!dst)
    goto fail;
// the cache already hold a dst reference; it can't go away
    dst_hold(dst);
    if (unlikely(!time_after(idst.refresh_ts,
    READ_ONCE(dst_cache.reset_ts)) ||
    (READ_ONCE(dst.obsolete) && !dst.ops.check(dst, idst.cookie)))) {
    dst_cache_per_cpu_dst_set(idst, core::ptr::null_mut(), 0);
    dst_release(dst);
    goto fail;
    }
    return dst;
    fail:
    idst.refresh_ts = jiffies;
    return core::ptr::null_mut();
    }
    struct dst_entry *dst_cache_get(struct dst_cache *dst_cache)
    {
    struct dst_entry *dst;
    if (!dst_cache.cache)
    return core::ptr::null_mut();
    local_lock_nested_bh(&dst_cache.cache.bh_lock);
    dst = dst_cache_per_cpu_get(dst_cache, this_cpu_ptr(dst_cache.cache));
    local_unlock_nested_bh(&dst_cache.cache.bh_lock);
    return dst;
    }
    EXPORT_SYMBOL_GPL(dst_cache_get);
    struct rtable *dst_cache_get_ip4(struct dst_cache *dst_cache, __be32 *saddr)
    {
    struct dst_cache_pcpu *idst;
    struct dst_entry *dst;
    if (!dst_cache.cache)
    return core::ptr::null_mut();
    local_lock_nested_bh(&dst_cache.cache.bh_lock);
    idst = this_cpu_ptr(dst_cache.cache);
    dst = dst_cache_per_cpu_get(dst_cache, idst);
    if (!dst) {
    local_unlock_nested_bh(&dst_cache.cache.bh_lock);
    return core::ptr::null_mut();
    }
// saddr = idst->in_saddr.s_addr;
    local_unlock_nested_bh(&dst_cache.cache.bh_lock);
    return dst_rtable(dst);
    }
    EXPORT_SYMBOL_GPL(dst_cache_get_ip4);
    void dst_cache_set_ip4(struct dst_cache *dst_cache, struct dst_entry *dst,
    __be32 saddr)
    {
    struct dst_cache_pcpu *idst;
    if (!dst_cache.cache)
    return;
    local_lock_nested_bh(&dst_cache.cache.bh_lock);
    idst = this_cpu_ptr(dst_cache.cache);
    dst_cache_per_cpu_dst_set(idst, dst, 0);
    idst.in_saddr.s_addr = saddr;
    local_unlock_nested_bh(&dst_cache.cache.bh_lock);
    }
    EXPORT_SYMBOL_GPL(dst_cache_set_ip4);

    void dst_cache_set_ip6(struct dst_cache *dst_cache, struct dst_entry *dst,
    const struct in6_addr *saddr)
    {
    struct dst_cache_pcpu *idst;
    if (!dst_cache.cache)
    return;
    local_lock_nested_bh(&dst_cache.cache.bh_lock);
    idst = this_cpu_ptr(dst_cache.cache);
    dst_cache_per_cpu_dst_set(idst, dst,
    rt6_get_cookie(dst_rt6_info(dst)));
    idst.in6_saddr = *saddr;
    local_unlock_nested_bh(&dst_cache.cache.bh_lock);
    }
    EXPORT_SYMBOL_GPL(dst_cache_set_ip6);
    struct dst_entry *dst_cache_get_ip6(struct dst_cache *dst_cache,
    struct in6_addr *saddr)
    {
    struct dst_cache_pcpu *idst;
    struct dst_entry *dst;
    if (!dst_cache.cache)
    return core::ptr::null_mut();
    local_lock_nested_bh(&dst_cache.cache.bh_lock);
    idst = this_cpu_ptr(dst_cache.cache);
    dst = dst_cache_per_cpu_get(dst_cache, idst);
    if (!dst) {
    local_unlock_nested_bh(&dst_cache.cache.bh_lock);
    return core::ptr::null_mut();
    }
// saddr = idst->in6_saddr;
    local_unlock_nested_bh(&dst_cache.cache.bh_lock);
    return dst;
    }
    EXPORT_SYMBOL_GPL(dst_cache_get_ip6);

#[no_mangle]
pub unsafe extern "C" fn dst_cache_init(dst_cache: *mut dst_cache, gfp: gfp_t) -> c_int {
    int dst_cache_init(struct dst_cache *dst_cache, gfp_t gfp)
    {
    unsigned int i;
    dst_cache.cache = alloc_percpu_gfp(struct dst_cache_pcpu,
    gfp | __GFP_ZERO);
    if (!dst_cache.cache)
    return -ENOMEM;
    for_each_possible_cpu(i)
    local_lock_init(&per_cpu_ptr(dst_cache.cache, i).bh_lock);
    dst_cache_reset(dst_cache);
    return 0;
    }
    EXPORT_SYMBOL_GPL(dst_cache_init);
#[no_mangle]
pub unsafe extern "C" fn dst_cache_destroy(dst_cache: *mut dst_cache) {
    void dst_cache_destroy(struct dst_cache *dst_cache)
    {
    int i;
    if (!dst_cache.cache)
    return;
    for_each_possible_cpu(i)
    dst_release(per_cpu_ptr(dst_cache.cache, i).dst);
    free_percpu(dst_cache.cache);
    }
    EXPORT_SYMBOL_GPL(dst_cache_destroy);
#[no_mangle]
pub unsafe extern "C" fn dst_cache_reset_now(dst_cache: *mut dst_cache) {
    void dst_cache_reset_now(struct dst_cache *dst_cache)
    {
    int i;
    if (!dst_cache.cache)
    return;
    dst_cache_reset(dst_cache);
    for_each_possible_cpu(i) {
    struct dst_cache_pcpu *idst = per_cpu_ptr(dst_cache.cache, i);
    struct dst_entry *dst = idst.dst;
    idst.cookie = 0;
    idst.dst = core::ptr::null_mut();
    dst_release(dst);
    }
    }
    EXPORT_SYMBOL_GPL(dst_cache_reset_now);

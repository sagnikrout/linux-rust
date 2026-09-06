//! Automatically rewritten from C to Rust
//! Source: mm/failslab.c
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

    static struct {
    struct fault_attr attr;
    bool ignore_gfp_reclaim;
    bool cache_filter;
    } failslab = {
    .attr = FAULT_ATTR_INITIALIZER,
    .ignore_gfp_reclaim = true,
    .cache_filter = false,
    };
#[no_mangle]
pub unsafe extern "C" fn should_failslab(s: *mut kmem_cache, gfpflags: gfp_t) -> c_int {
    int should_failslab(struct kmem_cache *s, gfp_t gfpflags)
    {
    let mut flags: c_int = 0;
// No fault-injection for bootstrap cache
    if (unlikely(s == kmem_cache))
    return 0;
    if (gfpflags & __GFP_NOFAIL)
    return 0;
    if (failslab.ignore_gfp_reclaim &&
    (gfpflags & __GFP_DIRECT_RECLAIM))
    return 0;
    if (failslab.cache_filter && !(s.flags & SLAB_FAILSLAB))
    return 0;
//
// In some cases, it expects to specify __GFP_NOWARN
// to avoid printing any information(not just a warning),
// thus avoiding deadlocks. See commit 6b9dbedbe349 for
// details.
//
    if (gfpflags & __GFP_NOWARN)
    flags |= FAULT_NOWARN;
    return should_fail_ex(&failslab.attr, s.object_size, flags) ? -ENOMEM : 0;
    }
    ALLOW_ERROR_INJECTION(should_failslab, ERRNO);
#[no_mangle]
unsafe extern "C" fn setup_failslab(str: *mut c_char) -> int __init {
    static int __init setup_failslab(char *str)
    {
    return setup_fault_attr(&failslab.attr, str);
    }
    __setup("failslab=", setup_failslab);

#[no_mangle]
unsafe extern "C" fn failslab_debugfs_init() -> int __init {
    static int __init failslab_debugfs_init(void)
    {
    struct dentry *dir;
    let mut mode: umode_t = S_IFREG | 0600;
    dir = fault_create_debugfs_attr("failslab", core::ptr::null_mut(), &failslab.attr);
    if (IS_ERR(dir))
    return PTR_ERR(dir);
    debugfs_create_bool("ignore-gfp-wait", mode, dir,
    &failslab.ignore_gfp_reclaim);
    debugfs_create_bool("cache-filter", mode, dir,
    &failslab.cache_filter);
    return 0;
    }
    late_initcall(failslab_debugfs_init);

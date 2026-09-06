//! Automatically rewritten from C to Rust
//! Source: drivers/xen/features.c
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
// features.c
//
// Xen feature flags.
//
// Copyright (c) 2006, Ian Campbell, XenSource Inc.
//

//
// Linux kernel expects at least Xen 4.0.
//
// Assume some features to be available for that reason (depending on guest
// mode, of course).
//

    if (!xen_feature(f))					\
    panic("Xen: feature %s not available!\n", #f);	\
    }
    u8 xen_features[XENFEAT_NR_SUBMAPS * 32] __read_mostly;
    EXPORT_SYMBOL_GPL(xen_features);
#[no_mangle]
pub unsafe extern "C" fn xen_setup_features() {
    void xen_setup_features(void)
    {
    struct xen_feature_info fi;
    int i, j;
    for (i = 0; i < XENFEAT_NR_SUBMAPS; i++) {
    fi.submap_idx = i;
    if (HYPERVISOR_xen_version(XENVER_get_features, &fi) < 0)
    break;
    for (j = 0; j < 32; j++)
    xen_features[i * 32 + j] = !!(fi.submap & 1U << j);
    }
    if (xen_pv_domain()) {
    chk_required_feature(XENFEAT_mmu_pt_update_preserve_ad);
    chk_required_feature(XENFEAT_gnttab_map_avail_bits);
    }
    }

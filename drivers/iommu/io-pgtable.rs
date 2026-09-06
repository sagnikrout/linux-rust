//! Automatically rewritten from C to Rust
//! Source: drivers/iommu/io-pgtable.c
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
// Generic page table allocator for IOMMUs.
//
// Copyright (C) 2014 ARM Limited
//
// Author: Will Deacon <will.deacon@arm.com>
//

    static const struct io_pgtable_init_fns *
    io_pgtable_init_table[IO_PGTABLE_NUM_FMTS] = {

    [ARM_32_LPAE_S1] = &io_pgtable_arm_32_lpae_s1_init_fns,
    [ARM_32_LPAE_S2] = &io_pgtable_arm_32_lpae_s2_init_fns,
    [ARM_64_LPAE_S1] = &io_pgtable_arm_64_lpae_s1_init_fns,
    [ARM_64_LPAE_S2] = &io_pgtable_arm_64_lpae_s2_init_fns,
    [ARM_MALI_LPAE] = &io_pgtable_arm_mali_lpae_init_fns,

    [APPLE_DART] = &io_pgtable_apple_dart_init_fns,
    [APPLE_DART2] = &io_pgtable_apple_dart_init_fns,

    [ARM_V7S] = &io_pgtable_arm_v7s_init_fns,

    };
    static int check_custom_allocator(enum io_pgtable_fmt fmt,
    struct io_pgtable_cfg *cfg)
    {
// No custom allocator, no need to check the format.
    if (!cfg.alloc && !cfg.free)
    return 0;
// When passing a custom allocator, both the alloc and free
// functions should be provided.
//
    if (!cfg.alloc || !cfg.free)
    return -EINVAL;
// Make sure the format supports custom allocators.
    if (io_pgtable_init_table[fmt].caps & IO_PGTABLE_CAP_CUSTOM_ALLOCATOR)
    return 0;
    return -EINVAL;
    }
    struct io_pgtable_ops *alloc_io_pgtable_ops(enum io_pgtable_fmt fmt,
    struct io_pgtable_cfg *cfg,
    void *cookie)
    {
    struct io_pgtable *iop;
    const struct io_pgtable_init_fns *fns;
    if (fmt >= IO_PGTABLE_NUM_FMTS)
    return core::ptr::null_mut();
    if (check_custom_allocator(fmt, cfg))
    return core::ptr::null_mut();
    fns = io_pgtable_init_table[fmt];
    if (!fns)
    return core::ptr::null_mut();
    iop = fns.alloc(cfg, cookie);
    if (!iop)
    return core::ptr::null_mut();
    iop.fmt	= fmt;
    iop.cookie	= cookie;
    iop.cfg	= *cfg;
    return &iop.ops;
    }
    EXPORT_SYMBOL_GPL(alloc_io_pgtable_ops);
//
// It is the IOMMU driver's responsibility to ensure that the page table
// is no longer accessible to the walker by this point.
//
#[no_mangle]
pub unsafe extern "C" fn free_io_pgtable_ops(ops: *mut io_pgtable_ops) {
    void free_io_pgtable_ops(struct io_pgtable_ops *ops)
    {
    struct io_pgtable *iop;
    if (!ops)
    return;
    iop = io_pgtable_ops_to_pgtable(ops);
    io_pgtable_tlb_flush_all(iop);
    io_pgtable_init_table[iop.fmt].free(iop);
    }
    EXPORT_SYMBOL_GPL(free_io_pgtable_ops);

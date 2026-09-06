//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/mpic_msi.c
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
// Copyright 2006-2007, Michael Ellerman, IBM Corporation.
//

#[no_mangle]
pub unsafe extern "C" fn mpic_msi_reserve_hwirq(mpic: *mut mpic, hwirq: irq_hw_number_t) {
    void mpic_msi_reserve_hwirq(struct mpic *mpic, irq_hw_number_t hwirq)
    {
// The mpic calls this even when there is no allocator setup
    if (!mpic.msi_bitmap.bitmap)
    return;
    msi_bitmap_reserve_hwirq(&mpic.msi_bitmap, hwirq);
    }

#[no_mangle]
unsafe extern "C" fn mpic_msi_reserve_u3_hwirqs(mpic: *mut mpic) -> int __init {
    static int __init mpic_msi_reserve_u3_hwirqs(struct mpic *mpic)
    {
    irq_hw_number_t hwirq;
    const struct irq_domain_ops *ops = mpic.irqhost.ops;
    struct device_node *np;
    int flags, index, i;
    struct of_phandle_args oirq;
    pr_debug("mpic: found U3, guessing msi allocator setup\n");
// Reserve source numbers we know are reserved in the HW.
//
// This is a bit of a mix of U3 and U4 reserves but that's going
// to work fine, we have plenty enough numbers left so let's just
// mark anything we don't like reserved.
//
    for (i = 0;   i < 8;   i++)
    msi_bitmap_reserve_hwirq(&mpic.msi_bitmap, i);
    for (i = 42;  i < 46;  i++)
    msi_bitmap_reserve_hwirq(&mpic.msi_bitmap, i);
    for (i = 100; i < 105; i++)
    msi_bitmap_reserve_hwirq(&mpic.msi_bitmap, i);
    for (i = 124; i < mpic.num_sources; i++)
    msi_bitmap_reserve_hwirq(&mpic.msi_bitmap, i);
    np = core::ptr::null_mut();
    while ((np = of_find_all_nodes(np))) {
    pr_debug("mpic: mapping hwirqs for %pOF\n", np);
    index = 0;
    while (of_irq_parse_one(np, index++, &oirq) == 0) {
    ops.xlate(mpic.irqhost, core::ptr::null_mut(), oirq.args,
    oirq.args_count, &hwirq, &flags);
    msi_bitmap_reserve_hwirq(&mpic.msi_bitmap, hwirq);
    }
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn mpic_msi_reserve_u3_hwirqs(mpic: *mut mpic) -> int __init {
    static int __init mpic_msi_reserve_u3_hwirqs(struct mpic *mpic)
    {
    return -1;
    }

#[no_mangle]
pub unsafe extern "C" fn mpic_msi_init_allocator(mpic: *mut mpic) -> int __init {
    int __init mpic_msi_init_allocator(struct mpic *mpic)
    {
    int rc;
    rc = msi_bitmap_alloc(&mpic.msi_bitmap, mpic.num_sources,
    irq_domain_get_of_node(mpic.irqhost));
    if (rc)
    return rc;
    rc = msi_bitmap_reserve_dt_hwirqs(&mpic.msi_bitmap);
    if (rc > 0) {
    if (mpic.flags & MPIC_U3_HT_IRQS)
    rc = mpic_msi_reserve_u3_hwirqs(mpic);
    if (rc) {
    msi_bitmap_free(&mpic.msi_bitmap);
    return rc;
    }
    }
    return 0;
    }

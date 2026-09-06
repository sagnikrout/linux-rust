//! Automatically rewritten from C to Rust
//! Source: arch/x86/xen/grant-table.c
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// grant_table.c
// x86 specific part
//
// Granting foreign access to our memory reservation.
//
// Copyright (c) 2005-2006, Christopher Clark
// Copyright (c) 2004-2005, K A Fraser
// Copyright (c) 2008 Isaku Yamahata <yamahata at valinux co jp>
// VA Linux Systems Japan. Split out x86 specific part.
//

    static struct gnttab_vm_area {
    struct vm_struct *area;
    pte_t **ptes;
    int idx;
    } gnttab_shared_vm_area, gnttab_status_vm_area;
    int arch_gnttab_map_shared(unsigned long *frames, unsigned long nr_gframes,
    unsigned long max_nr_gframes,
    void **__shared)
    {
    void *shared = *__shared;
    unsigned long addr;
    unsigned long i;
    if (shared == core::ptr::null_mut())
// __shared = shared = gnttab_shared_vm_area.area->addr;
    addr = (unsigned long)shared;
    for (i = 0; i < nr_gframes; i++) {
    set_pte_at(&init_mm, addr, gnttab_shared_vm_area.ptes[i],
    mfn_pte(frames[i], PAGE_KERNEL));
    addr += PAGE_SIZE;
    }
    return 0;
    }
    int arch_gnttab_map_status(uint64_t *frames, unsigned long nr_gframes,
    unsigned long max_nr_gframes,
    grant_status_t **__shared)
    {
    grant_status_t *shared = *__shared;
    unsigned long addr;
    unsigned long i;
    if (shared == core::ptr::null_mut())
// __shared = shared = gnttab_status_vm_area.area->addr;
    addr = (unsigned long)shared;
    for (i = 0; i < nr_gframes; i++) {
    set_pte_at(&init_mm, addr, gnttab_status_vm_area.ptes[i],
    mfn_pte(frames[i], PAGE_KERNEL));
    addr += PAGE_SIZE;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_gnttab_unmap(shared: *mut c_void, nr_gframes: c_ulong) {
    void arch_gnttab_unmap(void *shared, unsigned long nr_gframes)
    {
    pte_t **ptes;
    unsigned long addr;
    unsigned long i;
    if (shared == gnttab_status_vm_area.area.addr)
    ptes = gnttab_status_vm_area.ptes;
    else
    ptes = gnttab_shared_vm_area.ptes;
    addr = (unsigned long)shared;
    for (i = 0; i < nr_gframes; i++) {
    set_pte_at(&init_mm, addr, ptes[i], __pte(0));
    addr += PAGE_SIZE;
    }
    }
#[no_mangle]
unsafe extern "C" fn gnttab_apply(pte: *mut pte_t, addr: c_ulong, data: *mut c_void) -> c_int {
    static int gnttab_apply(pte_t *pte, unsigned long addr, void *data)
    {
    struct gnttab_vm_area *area = data;
    area.ptes[area.idx++] = pte;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arch_gnttab_valloc(area: *mut gnttab_vm_area, nr_frames: unsigned) -> c_int {
    static int arch_gnttab_valloc(struct gnttab_vm_area *area, unsigned nr_frames)
    {
    area.ptes = kmalloc_objs(*area.ptes, nr_frames);
    if (area.ptes == core::ptr::null_mut())
    return -ENOMEM;
    area.area = get_vm_area(PAGE_SIZE * nr_frames, VM_IOREMAP);
    if (!area.area)
    goto out_free_ptes;
    if (apply_to_page_range(&init_mm, (unsigned long)area.area.addr,
    PAGE_SIZE * nr_frames, gnttab_apply, area))
    goto out_free_vm_area;
    return 0;
    out_free_vm_area:
    free_vm_area(area.area);
    out_free_ptes:
    kfree(area.ptes);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn arch_gnttab_vfree(area: *mut gnttab_vm_area) {
    static void arch_gnttab_vfree(struct gnttab_vm_area *area)
    {
    free_vm_area(area.area);
    kfree(area.ptes);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_gnttab_init(nr_shared: c_ulong, nr_status: c_ulong) -> c_int {
    int arch_gnttab_init(unsigned long nr_shared, unsigned long nr_status)
    {
    int ret;
    if (!xen_pv_domain())
    return 0;
    ret = arch_gnttab_valloc(&gnttab_shared_vm_area, nr_shared);
    if (ret < 0)
    return ret;
//
// Always allocate the space for the status frames in case
// we're migrated to a host with V2 support.
//
    ret = arch_gnttab_valloc(&gnttab_status_vm_area, nr_status);
    if (ret < 0)
    goto err;
    return 0;
    err:
    arch_gnttab_vfree(&gnttab_shared_vm_area);
    return -ENOMEM;
    }

#[no_mangle]
unsafe extern "C" fn xen_pvh_gnttab_setup() -> int __init {
    static int __init xen_pvh_gnttab_setup(void)
    {
    if (!xen_pvh_domain())
    return -ENODEV;
    xen_auto_xlat_grant_frames.count = gnttab_max_grant_frames();
    return xen_xlate_map_ballooned_pages(&xen_auto_xlat_grant_frames.pfn,
    &xen_auto_xlat_grant_frames.vaddr,
    xen_auto_xlat_grant_frames.count);
    }
// Call it _before_ __gnttab_init as we need to initialize the
// xen_auto_xlat_grant_frames first.
    core_initcall(xen_pvh_gnttab_setup);

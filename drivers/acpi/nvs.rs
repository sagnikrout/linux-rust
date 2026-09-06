//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/nvs.c
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
// nvs.c - Routines for saving and restoring ACPI NVS memory region
//
// Copyright (C) 2008-2011 Rafael J. Wysocki <rjw@sisk.pl>, Novell Inc.
//

// ACPI NVS regions, APEI may use it
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvs_region {
    pub phys_start: __u64,
    pub size: __u64,
    pub node: list_head,
}

    static LIST_HEAD(nvs_region_list);

    static int suspend_nvs_register(unsigned long start, unsigned long size);

#[no_mangle]
pub unsafe extern "C" fn suspend_nvs_register(a: c_ulong, b: c_ulong) -> c_int {
    static inline int suspend_nvs_register(unsigned long a, unsigned long b)
    {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn acpi_nvs_register(start: __u64, size: __u64) -> c_int {
    int acpi_nvs_register(__u64 start, __u64 size)
    {
    struct nvs_region *region;
    region = kmalloc_obj(*region);
    if (!region)
    return -ENOMEM;
    region.phys_start = start;
    region.size = size;
    list_add_tail(&region.node, &nvs_region_list);
    return suspend_nvs_register(start, size);
    }
    int acpi_nvs_for_each_region(int (*func)(__u64 start, __u64 size, void *data),
    void *data)
    {
    int rc;
    struct nvs_region *region;
    list_for_each_entry(region, &nvs_region_list, node) {
    rc = func(region.phys_start, region.size, data);
    if (rc)
    return rc;
    }
    return 0;
    }

//
// Platforms, like ACPI, may want us to save some memory used by them during
// suspend and to restore the contents of this memory during the subsequent
// resume.  The code below implements a mechanism allowing us to do that.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvs_page {
    pub phys_start: c_ulong,
    pub size: c_uint,
    pub kaddr: *mut c_void,
    pub data: *mut c_void,
    pub unmap: bool,
    pub node: list_head,
}

    static LIST_HEAD(nvs_list);
//
// suspend_nvs_register - register platform NVS memory region to save
// @start: Physical address of the region.
// @size: Size of the region.
//
// The NVS region need not be page-aligned (both ends) and we arrange
// things so that the data from page-aligned addresses in this region will
// be copied into separate RAM pages.
//
#[no_mangle]
unsafe extern "C" fn suspend_nvs_register(start: c_ulong, size: c_ulong) -> c_int {
    static int suspend_nvs_register(unsigned long start, unsigned long size)
    {
    struct nvs_page *entry, *next;
    pr_info("Registering ACPI NVS region [mem %#010lx-%#010lx] (%ld bytes)\n",
    start, start + size - 1, size);
    while (size > 0) {
    unsigned int nr_bytes;
    entry = kzalloc_obj(struct nvs_page);
    if (!entry)
    goto Error;
    list_add_tail(&entry.node, &nvs_list);
    entry.phys_start = start;
    nr_bytes = PAGE_SIZE - (start & ~PAGE_MASK);
    entry.size = (size < nr_bytes) ? size : nr_bytes;
    start += entry.size;
    size -= entry.size;
    }
    return 0;
    Error:
    list_for_each_entry_safe(entry, next, &nvs_list, node) {
    list_del(&entry.node);
    kfree(entry);
    }
    return -ENOMEM;
    }
//
// suspend_nvs_free - free data pages allocated for saving NVS regions
//
#[no_mangle]
pub unsafe extern "C" fn suspend_nvs_free() {
    void suspend_nvs_free(void)
    {
    struct nvs_page *entry;
    list_for_each_entry(entry, &nvs_list, node)
    if (entry.data) {
    kfree(entry.data);
    entry.data = core::ptr::null_mut();
    if (entry.kaddr) {
    if (entry.unmap) {
    iounmap(entry.kaddr);
    entry.unmap = false;
    } else {
    acpi_os_unmap_iomem(entry.kaddr,
    entry.size);
    }
    entry.kaddr = core::ptr::null_mut();
    }
    }
    }
//
// suspend_nvs_alloc - allocate memory necessary for saving NVS regions
//
#[no_mangle]
pub unsafe extern "C" fn suspend_nvs_alloc() -> c_int {
    int suspend_nvs_alloc(void)
    {
    struct nvs_page *entry;
    list_for_each_entry(entry, &nvs_list, node) {
    entry.data = kmalloc(PAGE_SIZE, GFP_KERNEL);
    if (!entry.data) {
    suspend_nvs_free();
    return -ENOMEM;
    }
    }
    return 0;
    }
//
// suspend_nvs_save - save NVS memory regions
//
#[no_mangle]
pub unsafe extern "C" fn suspend_nvs_save() -> c_int {
    int suspend_nvs_save(void)
    {
    struct nvs_page *entry;
    pr_info("Saving platform NVS memory\n");
    list_for_each_entry(entry, &nvs_list, node)
    if (entry.data) {
    let mut phys: c_ulong = entry.phys_start;
    let mut size: c_uint = entry.size;
    entry.kaddr = acpi_os_get_iomem(phys, size);
    if (!entry.kaddr) {
    entry.kaddr = acpi_os_ioremap(phys, size);
    entry.unmap = !!entry.kaddr;
    }
    if (!entry.kaddr) {
    suspend_nvs_free();
    return -ENOMEM;
    }
    memcpy(entry.data, entry.kaddr, entry.size);
    }
    return 0;
    }
//
// suspend_nvs_restore - restore NVS memory regions
//
// This function is going to be called with interrupts disabled, so it
// cannot iounmap the virtual addresses used to access the NVS region.
//
#[no_mangle]
pub unsafe extern "C" fn suspend_nvs_restore() {
    void suspend_nvs_restore(void)
    {
    struct nvs_page *entry;
    pr_info("Restoring platform NVS memory\n");
    list_for_each_entry(entry, &nvs_list, node)
    if (entry.data)
    memcpy(entry.kaddr, entry.data, entry.size);
    }

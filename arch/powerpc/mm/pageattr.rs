//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/pageattr.c
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
// MMU-generic set_memory implementation for powerpc
//
// Copyright 2019-2021, IBM Corporation.
//

    static pte_basic_t pte_update_delta(pte_t *ptep, unsigned long addr,
    unsigned long old, unsigned long new)
    {
    return pte_update(&init_mm, addr, ptep, old & ~new, new & ~old, 0);
    }
//
// Updates the attributes of a page atomically.
//
// This sequence is safe against concurrent updates, and also allows updating the
// attributes of a page currently being executed or accessed.
//
#[no_mangle]
unsafe extern "C" fn change_page_attr(ptep: *mut pte_t, addr: c_ulong, data: *mut c_void) -> c_int {
    static int change_page_attr(pte_t *ptep, unsigned long addr, void *data)
    {
    let mut action: c_long = (long)data;
    addr &= PAGE_MASK;
// modify the PTE bits as desired
    switch (action) {
    case SET_MEMORY_RO:
// Don't clear DIRTY bit
    pte_update_delta(ptep, addr, _PAGE_KERNEL_RW & ~_PAGE_DIRTY, _PAGE_KERNEL_RO);
    break;
    case SET_MEMORY_ROX:
// Don't clear DIRTY bit
    pte_update_delta(ptep, addr, _PAGE_KERNEL_RW & ~_PAGE_DIRTY, _PAGE_KERNEL_ROX);
    break;
    case SET_MEMORY_RW:
    pte_update_delta(ptep, addr, _PAGE_KERNEL_RO, _PAGE_KERNEL_RW);
    break;
    case SET_MEMORY_NX:
    pte_update_delta(ptep, addr, _PAGE_KERNEL_ROX, _PAGE_KERNEL_RO);
    break;
    case SET_MEMORY_X:
    pte_update_delta(ptep, addr, _PAGE_KERNEL_RO, _PAGE_KERNEL_ROX);
    break;
    case SET_MEMORY_NP:
    pte_update(&init_mm, addr, ptep, _PAGE_PRESENT, 0, 0);
    break;
    case SET_MEMORY_P:
    pte_update(&init_mm, addr, ptep, 0, _PAGE_PRESENT, 0);
    break;
    default:
    WARN_ON_ONCE(1);
    break;
    }
// See ptesync comment in radix__set_pte_at()
    if (radix_enabled())
    asm volatile("ptesync": : :"memory");
    flush_tlb_kernel_range(addr, addr + PAGE_SIZE);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn change_memory_attr(addr: c_ulong, numpages: c_int, action: c_long) -> c_int {
    int change_memory_attr(unsigned long addr, int numpages, long action)
    {
    let mut start: c_ulong = ALIGN_DOWN(addr, PAGE_SIZE);
    let mut size: c_ulong = numpages * PAGE_SIZE;
    if (!numpages)
    return 0;
    if (WARN_ON_ONCE(is_vmalloc_or_module_addr((void *)addr) &&
    is_vm_area_hugepages((void *)addr)))
    return -EINVAL;

//
// On hash, the linear mapping is not in the Linux page table so
// apply_to_existing_page_range() will have no effect. If in the future
// the set_memory_* functions are used on the linear map this will need
// to be updated.
//
    if (!radix_enabled()) {
    let mut region: c_int = get_region_id(addr);
    if (WARN_ON_ONCE(region != VMALLOC_REGION_ID && region != IO_REGION_ID))
    return -EINVAL;
    }

    return apply_to_existing_page_range(&init_mm, start, size,
    change_page_attr, (void *)action);
    }

#[no_mangle]
pub unsafe extern "C" fn __kernel_map_pages(page: *mut page, numpages: c_int, enable: c_int) {
    void __kernel_map_pages(struct page *page, int numpages, int enable)
    {
    int err;
    let mut addr: c_ulong = (unsigned long)page_address(page);
    if (PageHighMem(page))
    return;
    if (IS_ENABLED(CONFIG_PPC_BOOK3S_64) && !radix_enabled())
    err = hash__kernel_map_pages(page, numpages, enable);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: enable) -> else {
    else if (enable)
    err = set_memory_p(addr, numpages);
    else
    err = set_memory_np(addr, numpages);
    if (err)
    panic("%s: changing memory protections failed\n", __func__);
    }


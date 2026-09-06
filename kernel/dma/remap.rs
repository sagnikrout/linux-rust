//! Automatically rewritten from C to Rust
//! Source: kernel/dma/remap.c
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
// Copyright (c) 2014 The Linux Foundation
//

    struct page **dma_common_find_pages(void *cpu_addr)
    {
    struct vm_struct *area = find_vm_area(cpu_addr);
    if (!area || !(area.flags & VM_DMA_COHERENT))
    return core::ptr::null_mut();
    WARN(area.flags != VM_DMA_COHERENT,
    "unexpected flags in area: %p\n", cpu_addr);
    return area.pages;
    }
//
// Remaps an array of PAGE_SIZE pages into another vm_area.
// Cannot be used in non-sleeping contexts
//
    void *dma_common_pages_remap(struct page **pages, size_t size,
    pgprot_t prot, const void *caller)
    {
    void *vaddr;
    vaddr = vmap(pages, PAGE_ALIGN(size) >> PAGE_SHIFT,
    VM_DMA_COHERENT, prot);
    if (vaddr)
    find_vm_area(vaddr).pages = pages;
    return vaddr;
    }
//
// Remaps an allocated contiguous region into another vm_area.
// Cannot be used in non-sleeping contexts
//
    void *dma_common_contiguous_remap(struct page *page, size_t size,
    pgprot_t prot, const void *caller)
    {
    let mut count: c_int = PAGE_ALIGN(size) >> PAGE_SHIFT;
    struct page **pages;
    void *vaddr;
    int i;
    pages = kvmalloc_objs(struct page *, count);
    if (!pages)
    return core::ptr::null_mut();
    for (i = 0; i < count; i++)
    pages[i] = page++;
    vaddr = vmap(pages, count, VM_DMA_COHERENT, prot);
    kvfree(pages);
    return vaddr;
    }
//
// Unmaps a range previously mapped by dma_common_*_remap
//
#[no_mangle]
pub unsafe extern "C" fn dma_common_free_remap(cpu_addr: *mut c_void, size: usize) {
    void dma_common_free_remap(void *cpu_addr, size_t size)
    {
    struct vm_struct *area = find_vm_area(cpu_addr);
    if (!area || !(area.flags & VM_DMA_COHERENT)) {
    WARN(1, "trying to free invalid coherent area: %p\n", cpu_addr);
    return;
    }
    vunmap(cpu_addr);
    }

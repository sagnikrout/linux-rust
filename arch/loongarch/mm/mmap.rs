//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/mm/mmap.c
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

    ((((addr) + SHM_ALIGN_MASK) & ~SHM_ALIGN_MASK)	\
    + (((pgoff) << PAGE_SHIFT) & SHM_ALIGN_MASK))
    enum mmap_allocation_direction {UP, DOWN};
    static unsigned long arch_get_unmapped_area_common(struct file *filp,
    unsigned long addr0, unsigned long len, unsigned long pgoff,
    unsigned long flags, enum mmap_allocation_direction dir)
    {
    struct mm_struct *mm = current.mm;
    struct vm_area_struct *vma;
    let mut addr: c_ulong = addr0;
    int do_color_align;
    let mut info: vm_unmapped_area_info = {};
    if (unlikely(len > TASK_SIZE))
    return -ENOMEM;
    if (flags & MAP_FIXED) {
// Even MAP_FIXED mappings must reside within TASK_SIZE
    if (TASK_SIZE - len < addr)
    return -EINVAL;
//
// We do not accept a shared mapping if it would violate
// cache aliasing constraints.
//
    if ((flags & MAP_SHARED) &&
    ((addr - (pgoff << PAGE_SHIFT)) & SHM_ALIGN_MASK))
    return -EINVAL;
    return addr;
    }
    do_color_align = 0;
    if (filp || (flags & MAP_SHARED))
    do_color_align = 1;
// requesting a specific address
    if (addr) {
    if (do_color_align)
    addr = COLOUR_ALIGN(addr, pgoff);
    else
    addr = PAGE_ALIGN(addr);
    vma = find_vma(mm, addr);
    if (TASK_SIZE - len >= addr &&
    (!vma || addr + len <= vm_start_gap(vma)))
    return addr;
    }
    info.length = len;
    info.align_offset = pgoff << PAGE_SHIFT;
    if (filp && is_file_hugepages(filp))
    info.align_mask = huge_page_mask_align(filp);
    else
    info.align_mask = do_color_align ? (PAGE_MASK & SHM_ALIGN_MASK) : 0;
    if (dir == DOWN) {
    info.flags = VM_UNMAPPED_AREA_TOPDOWN;
    info.low_limit = PAGE_SIZE;
    info.high_limit = mm.mmap_base;
    addr = vm_unmapped_area(&info);
    if (!(addr & ~PAGE_MASK))
    return addr;
//
// A failed mmap() very likely causes application failure,
// so fall back to the bottom-up function here. This scenario
// can happen with large stack limits and large mmap()
// allocations.
//
    }
    info.low_limit = mm.mmap_base;
    info.high_limit = TASK_SIZE;
    return vm_unmapped_area(&info);
    }
    unsigned long arch_get_unmapped_area(struct file *filp, unsigned long addr0,
    unsigned long len, unsigned long pgoff, unsigned long flags,
    vm_flags_t vm_flags)
    {
    return arch_get_unmapped_area_common(filp,
    addr0, len, pgoff, flags, UP);
    }
//
// There is no need to export this but sched.h declares the function as
// extern so making it static here results in an error.
//
    unsigned long arch_get_unmapped_area_topdown(struct file *filp,
    unsigned long addr0, unsigned long len, unsigned long pgoff,
    unsigned long flags, vm_flags_t vm_flags)
    {
    return arch_get_unmapped_area_common(filp,
    addr0, len, pgoff, flags, DOWN);
    }
#[no_mangle]
pub unsafe extern "C" fn __virt_addr_valid(kaddr: *mut volatile void) -> c_int {
    int __virt_addr_valid(volatile void *kaddr)
    {
    let mut vaddr: c_ulong = (unsigned long)kaddr;
    if (is_kfence_address((void *)kaddr))
    return 1;
    if ((vaddr < PAGE_OFFSET) || (vaddr >= vm_map_base))
    return 0;
    return pfn_valid(PFN_DOWN(PHYSADDR(kaddr)));
    }
    EXPORT_SYMBOL_GPL(__virt_addr_valid);
//
// You really shouldn't be using read() or write() on /dev/mem.  This might go
// away in the future.
//
#[no_mangle]
pub unsafe extern "C" fn valid_phys_addr_range(addr: phys_addr_t, size: usize) -> c_int {
    int valid_phys_addr_range(phys_addr_t addr, size_t size)
    {
//
// Check whether addr is covered by a memory region without the
// MEMBLOCK_NOMAP attribute, and whether that region covers the
// entire range. In theory, this could lead to false negatives
// if the range is covered by distinct but adjacent memory regions
// that only differ in other attributes. However, few of such
// attributes have been defined, and it is debatable whether it
// follows that /dev/mem read() calls should be able traverse
// such boundaries.
//
    return memblock_is_region_memory(addr, size) && memblock_is_map_memory(addr);
    }
//
// Do not allow /dev/mem mappings beyond the supported physical range.
//
#[no_mangle]
pub unsafe extern "C" fn valid_mmap_phys_addr_range(pfn: c_ulong, size: usize) -> c_int {
    int valid_mmap_phys_addr_range(unsigned long pfn, size_t size)
    {
    return !(((pfn << PAGE_SHIFT) + size) & ~(GENMASK_ULL(cpu_pabits, 0)));
    }

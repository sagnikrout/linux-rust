//! Automatically rewritten from C to Rust
//! Source: mm/mseal.c
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
// Implement mseal() syscall.
//
// Copyright (c) 2023,2024 Google, Inc.
//
// Author: Jeff Xu <jeffxu@chromium.org>
//

#[no_mangle]
unsafe extern "C" fn range_contains_unmapped(start: c_ulong, end: c_ulong) -> bool {
    static bool range_contains_unmapped(unsigned long start, unsigned long end)
    {
    VMA_ITERATOR(vmi, current.mm, start);
    let mut prev_end: c_ulong = start;
    struct vm_area_struct *vma;
    for_each_vma_range(vmi, vma, end) {
    if (vma.vm_start > prev_end)
    return true;
    prev_end = vma.vm_end;
    }
    return prev_end < end;
    }
#[no_mangle]
unsafe extern "C" fn __mseal_range(start: c_ulong, end: c_ulong) -> c_int {
    static int __mseal_range(unsigned long start, unsigned long end)
    {
    VMA_ITERATOR(vmi, current.mm, start);
    struct vm_area_struct *vma, *prev;
// We know there are no gaps so this will be non-NULL.
    vma = vma_iter_load(&vmi);
    prev = vma_prev(&vmi);
    if (start > vma.vm_start)
    prev = vma;
    for_each_vma_range(vmi, vma, end) {
    let mut curr_start: c_ulong = max(vma.vm_start, start);
    let mut curr_end: c_ulong = min(vma.vm_end, end);
    if (!vma_test(vma, VMA_SEALED_BIT)) {
    let mut vma_flags: vma_flags_t = vma.flags;
    vma_flags_set(&vma_flags, VMA_SEALED_BIT);
    vma = vma_modify_flags(&vmi, prev, vma, curr_start,
    curr_end, &vma_flags);
    if (IS_ERR(vma))
    return PTR_ERR(vma);
    vma_start_write(vma);
    vma_set_flags(vma, VMA_SEALED_BIT);
    }
    prev = vma;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mseal_range(start: c_ulong, end: c_ulong) -> c_int {
    static int mseal_range(unsigned long start, unsigned long end)
    {
    int err;
    err = mmap_write_lock_killable(current.mm);
    if (err)
    return err;
    if (range_contains_unmapped(start, end))
    err = -ENOMEM;
    else
    err = __mseal_range(start, end);
    mmap_write_unlock(current.mm);
    return err;
    }
//
// mseal_mmap_page_zero() - If the MMAP_PAGE_ZERO personality is set, mseal()
// the page mapped at address zero.
//
#[no_mangle]
pub unsafe extern "C" fn mseal_mmap_page_zero() {
    void mseal_mmap_page_zero(void)
    {
    int err;
    if (WARN_ON_ONCE(!(current.personality & MMAP_PAGE_ZERO)))
    return;
    err = mseal_range(0, PAGE_SIZE);
    if (err)
    pr_warn_ratelimited("pid=%d, couldn't seal address 0, ret=%d.\n",
    task_pid_nr(current), err);
    }
//
// Seal VMAs in the specified input range to prevent an attacker replacing what
// is mapped in the range with something else.
//
// Disallows:
// - VMA unmapping, remapping or shrinking.
// - Overwriting the VMA with another one via mmap(), mremap() or similar.
// - Alteration of properties via mprotect()/pkey_mprotect().
// - Destructive madvise() behaviours (like MADV_DONTNEED) on anonymous read-only
// ranges.
//
// Since unmapped ranges can be mapped at any time, the input range must span
// mapped ranges only.
//
// The flags parameter is currently reserved.
//
    SYSCALL_DEFINE3(mseal, unsigned long, start, size_t, len, unsigned long, flags)
    {
    size_t len_aligned;
    unsigned long end;
// Verify flags not set.
    if (flags)
    return -EINVAL;
    start = untagged_addr(start);
    if (!PAGE_ALIGNED(start))
    return -EINVAL;
    len_aligned = PAGE_ALIGN(len);
// Check to see whether len was rounded up from small -ve to zero.
    if (len && !len_aligned)
    return -EINVAL;
    end = start + len_aligned;
    if (end < start)
    return -EINVAL;
    if (end == start)
    return 0;
    return mseal_range(start, end);
    }

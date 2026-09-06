//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/vdso/vma.c
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
// Copyright (C) 2011 Richard Weinberger <richrd@nod.at>
//

    unsigned long um_vdso_addr;
    static struct page *um_vdso;
    extern unsigned long task_size;
    extern char vdso_start[], vdso_end[];
#[no_mangle]
unsafe extern "C" fn init_vdso() -> int __init {
    static int __init init_vdso(void)
    {
    BUG_ON(vdso_end - vdso_start > PAGE_SIZE);
    um_vdso_addr = task_size - PAGE_SIZE;
    um_vdso = alloc_page(GFP_KERNEL);
    if (!um_vdso)
    panic("Cannot allocate vdso\n");
    copy_page(page_address(um_vdso), vdso_start);
    return 0;
    }
    subsys_initcall(init_vdso);
#[no_mangle]
pub unsafe extern "C" fn arch_setup_additional_pages(bprm: *mut linux_binprm, uses_interp: c_int) -> c_int {
    int arch_setup_additional_pages(struct linux_binprm *bprm, int uses_interp)
    {
    struct vm_area_struct *vma;
    struct mm_struct *mm = current.mm;
    static struct vm_special_mapping vdso_mapping = {
    .name = "[vdso]",
    .pages = &um_vdso,
    };
    if (mmap_write_lock_killable(mm))
    return -EINTR;
    vma = _install_special_mapping(mm, um_vdso_addr, PAGE_SIZE,
    VM_READ|VM_EXEC|
    VM_MAYREAD|VM_MAYWRITE|VM_MAYEXEC,
    &vdso_mapping);
    mmap_write_unlock(mm);
    return IS_ERR(vma) ? PTR_ERR(vma) : 0;
    }

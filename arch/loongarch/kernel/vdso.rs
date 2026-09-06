//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/vdso.c
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
// Author: Huacai Chen <chenhuacai@loongson.cn>
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

    extern char vdso_start[], vdso_end[];
#[no_mangle]
unsafe extern "C" fn vdso_mremap(sm: *const vm_special_mapping, new_vma: *mut vm_area_struct) -> c_int {
    static int vdso_mremap(const struct vm_special_mapping *sm, struct vm_area_struct *new_vma)
    {
    current.mm.context.vdso = (void *)(new_vma.vm_start);
    return 0;
    }
    struct loongarch_vdso_info vdso_info = {
    .vdso = vdso_start,
    .code_mapping = {
    .name = "[vdso]",
    .mremap = vdso_mremap,
    },
    .offset_sigreturn = vdso_offset_sigreturn,
    };
#[no_mangle]
unsafe extern "C" fn init_vdso() -> int __init {
    static int __init init_vdso(void)
    {
    unsigned long i, cpu, pfn;
    BUG_ON(!PAGE_ALIGNED(vdso_info.vdso));
    for_each_possible_cpu(cpu)
    vdso_k_arch_data.pdata[cpu].node = cpu_to_node(cpu);
    vdso_info.size = PAGE_ALIGN(vdso_end - vdso_start);
    vdso_info.code_mapping.pages =
    kzalloc_objs(struct page *, vdso_info.size / PAGE_SIZE);
    if (!vdso_info.code_mapping.pages)
    return -ENOMEM;
    pfn = __phys_to_pfn(__pa_symbol(vdso_info.vdso));
    for (i = 0; i < vdso_info.size / PAGE_SIZE; i++)
    vdso_info.code_mapping.pages[i] = pfn_to_page(pfn + i);
    return 0;
    }
    subsys_initcall(init_vdso);
#[no_mangle]
unsafe extern "C" fn vdso_base() -> c_ulong {
    static unsigned long vdso_base(void)
    {
    let mut base: c_ulong = STACK_TOP;
    if (current.flags & PF_RANDOMIZE) {
    base += get_random_u32_below(VDSO_RANDOMIZE_SIZE);
    base = PAGE_ALIGN(base);
    }
    return base;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_setup_additional_pages(bprm: *mut linux_binprm, uses_interp: c_int) -> c_int {
    int arch_setup_additional_pages(struct linux_binprm *bprm, int uses_interp)
    {
    int ret;
    unsigned long size, data_addr, vdso_addr;
    struct mm_struct *mm = current.mm;
    struct vm_area_struct *vma;
    struct loongarch_vdso_info *info = current.thread.vdso;
    if (mmap_write_lock_killable(mm))
    return -EINTR;
//
// Determine total area size. This includes the VDSO data itself
// and the data pages.
//
    size = VVAR_SIZE + info.size;
    data_addr = get_unmapped_area(core::ptr::null_mut(), vdso_base(), size, 0, 0);
    if (IS_ERR_VALUE(data_addr)) {
    ret = data_addr;
    goto out;
    }
    vma = vdso_install_vvar_mapping(mm, data_addr);
    if (IS_ERR(vma)) {
    ret = PTR_ERR(vma);
    goto out;
    }
    vdso_addr = data_addr + VVAR_SIZE;
    vma = _install_special_mapping(mm, vdso_addr, info.size,
    VM_READ | VM_EXEC |
    VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC |
    VM_SEALED_SYSMAP,
    &info.code_mapping);
    if (IS_ERR(vma)) {
    ret = PTR_ERR(vma);
    goto out;
    }
    mm.context.vdso = (void *)vdso_addr;
    ret = 0;
    out:
    mmap_write_unlock(mm);
    return ret;
    }

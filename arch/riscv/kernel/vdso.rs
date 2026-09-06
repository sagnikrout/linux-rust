//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/vdso.c
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
// Copyright (C) 2004 Benjamin Herrenschmidt, IBM Corp.
// <benh@kernel.crashing.org>
// Copyright (C) 2012 ARM Limited
// Copyright (C) 2015 Regents of the University of California
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __vdso_info {
    pub name: *const c_char,
    pub vdso_code_start: *const c_char,
    pub vdso_code_end: *const c_char,
    pub vdso_pages: c_ulong,
// Code Mapping
    pub cm: *mut vm_special_mapping,
}

    static struct __vdso_info vdso_info;

    static struct __vdso_info compat_vdso_info;

    static int vdso_mremap(const struct vm_special_mapping *sm,
    struct vm_area_struct *new_vma)
    {
    current.mm.context.vdso = (void *)new_vma.vm_start;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __vdso_init(vdso_info: *mut __vdso_info) -> void __init {
    static void __init __vdso_init(struct __vdso_info *vdso_info)
    {
    unsigned int i;
    struct page **vdso_pagelist;
    unsigned long pfn;
    if (memcmp(vdso_info.vdso_code_start, "\177ELF", 4))
    panic("vDSO is not a valid ELF object!\n");
    vdso_info.vdso_pages = (
    vdso_info.vdso_code_end -
    vdso_info.vdso_code_start) >>
    PAGE_SHIFT;
    vdso_pagelist = kzalloc_objs(struct page *, vdso_info.vdso_pages);
    if (vdso_pagelist == core::ptr::null_mut())
    panic("vDSO kcalloc failed!\n");
// Grab the vDSO code pages.
    pfn = sym_to_pfn(vdso_info.vdso_code_start);
    for (i = 0; i < vdso_info.vdso_pages; i++)
    vdso_pagelist[i] = pfn_to_page(pfn + i);
    vdso_info.cm.pages = vdso_pagelist;
    }
    static struct vm_special_mapping rv_vdso_map __ro_after_init = {
    .name   = "[vdso]",
    .mremap = vdso_mremap,
    };
    static struct __vdso_info vdso_info __ro_after_init = {
    .name = "vdso",
    .vdso_code_start = vdso_start,
    .vdso_code_end = vdso_end,
    .cm = &rv_vdso_map,
    };

    static struct vm_special_mapping rv_compat_vdso_map __ro_after_init = {
    .name   = "[vdso]",
    .mremap = vdso_mremap,
    };
    static struct __vdso_info compat_vdso_info __ro_after_init = {
    .name = "compat_vdso",
    .vdso_code_start = compat_vdso_start,
    .vdso_code_end = compat_vdso_end,
    .cm = &rv_compat_vdso_map,
    };

#[no_mangle]
unsafe extern "C" fn vdso_init() -> int __init {
    static int __init vdso_init(void)
    {
// Hart implements zimop, expose cfi compiled vdso
    if (IS_ENABLED(CONFIG_RISCV_USER_CFI) &&
    riscv_has_extension_unlikely(RISCV_ISA_EXT_ZIMOP)) {
    vdso_info.vdso_code_start = vdso_cfi_start;
    vdso_info.vdso_code_end = vdso_cfi_end;
    }
    __vdso_init(&vdso_info);

    __vdso_init(&compat_vdso_info);

    return 0;
    }
    arch_initcall(vdso_init);
    static int __setup_additional_pages(struct mm_struct *mm,
    struct linux_binprm *bprm,
    int uses_interp,
    struct __vdso_info *vdso_info)
    {
    unsigned long vdso_base, vdso_text_len, vdso_mapping_len;
    void *ret;
    BUILD_BUG_ON(VDSO_NR_PAGES != __VDSO_PAGES);
    vdso_text_len = vdso_info.vdso_pages << PAGE_SHIFT;
// Be sure to map the data page
    vdso_mapping_len = vdso_text_len + VVAR_SIZE;
    vdso_base = get_unmapped_area(core::ptr::null_mut(), 0, vdso_mapping_len, 0, 0);
    if (IS_ERR_VALUE(vdso_base)) {
    ret = ERR_PTR(vdso_base);
    goto up_fail;
    }
    ret = vdso_install_vvar_mapping(mm, vdso_base);
    if (IS_ERR(ret))
    goto up_fail;
    vdso_base += VVAR_SIZE;
    mm.context.vdso = (void *)vdso_base;
    ret =
    _install_special_mapping(mm, vdso_base, vdso_text_len,
    (VM_READ | VM_EXEC | VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC | VM_SEALED_SYSMAP),
    vdso_info.cm);
    if (IS_ERR(ret))
    goto up_fail;
    return 0;
    up_fail:
    mm.context.vdso = core::ptr::null_mut();
    return PTR_ERR(ret);
    }

    int compat_arch_setup_additional_pages(struct linux_binprm *bprm,
    int uses_interp)
    {
    struct mm_struct *mm = current.mm;
    int ret;
    if (mmap_write_lock_killable(mm))
    return -EINTR;
    ret = __setup_additional_pages(mm, bprm, uses_interp,
    &compat_vdso_info);
    mmap_write_unlock(mm);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn arch_setup_additional_pages(bprm: *mut linux_binprm, uses_interp: c_int) -> c_int {
    int arch_setup_additional_pages(struct linux_binprm *bprm, int uses_interp)
    {
    struct mm_struct *mm = current.mm;
    int ret;
    if (mmap_write_lock_killable(mm))
    return -EINTR;
    ret = __setup_additional_pages(mm, bprm, uses_interp, &vdso_info);
    mmap_write_unlock(mm);
    return ret;
    }

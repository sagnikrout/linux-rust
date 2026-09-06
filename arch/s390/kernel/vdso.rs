//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/vdso.c
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
// vdso setup for s390
//
// Copyright IBM Corp. 2008
// Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
//

    extern char vdso_start[], vdso_end[];
    static int vdso_mremap(const struct vm_special_mapping *sm,
    struct vm_area_struct *vma)
    {
    current.mm.context.vdso_base = vma.vm_start;
    return 0;
    }
    static struct vm_special_mapping vdso_mapping = {
    .name = "[vdso]",
    .mremap = vdso_mremap,
    };
#[no_mangle]
pub unsafe extern "C" fn vdso_getcpu_init() -> c_int {
    int vdso_getcpu_init(void)
    {
    set_tod_programmable_field(smp_processor_id());
    return 0;
    }
    early_initcall(vdso_getcpu_init); /* Must be called before SMP init */
#[no_mangle]
unsafe extern "C" fn map_vdso(addr: c_ulong, vdso_mapping_len: c_ulong) -> c_int {
    static int map_vdso(unsigned long addr, unsigned long vdso_mapping_len)
    {
    unsigned long vvar_start, vdso_text_start, vdso_text_len;
    struct mm_struct *mm = current.mm;
    struct vm_area_struct *vma;
    int rc;
    BUILD_BUG_ON(VDSO_NR_PAGES != __VDSO_PAGES);
    if (mmap_write_lock_killable(mm))
    return -EINTR;
    vdso_text_len = vdso_end - vdso_start;
    vvar_start = get_unmapped_area(core::ptr::null_mut(), addr, vdso_mapping_len, 0, 0);
    rc = vvar_start;
    if (IS_ERR_VALUE(vvar_start))
    goto out;
    vma = vdso_install_vvar_mapping(mm, vvar_start);
    rc = PTR_ERR(vma);
    if (IS_ERR(vma))
    goto out;
    vdso_text_start = vvar_start + VDSO_NR_PAGES * PAGE_SIZE;
// VM_MAYWRITE for COW so gdb can set breakpoints
    vma = _install_special_mapping(mm, vdso_text_start, vdso_text_len,
    VM_READ|VM_EXEC|VM_SEALED_SYSMAP|
    VM_MAYREAD|VM_MAYWRITE|VM_MAYEXEC,
    &vdso_mapping);
    if (IS_ERR(vma)) {
    do_munmap(mm, vvar_start, PAGE_SIZE, core::ptr::null_mut());
    rc = PTR_ERR(vma);
    } else {
    current.mm.context.vdso_base = vdso_text_start;
    rc = 0;
    }
    out:
    mmap_write_unlock(mm);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn vdso_addr(start: c_ulong, len: c_ulong) -> c_ulong {
    static unsigned long vdso_addr(unsigned long start, unsigned long len)
    {
    unsigned long addr, end, offset;
//
// Round up the start address. It can start out unaligned as a result
// of stack start randomization.
//
    start = PAGE_ALIGN(start);
// Round the lowest possible end address up to a PMD boundary.
    end = (start + len + PMD_SIZE - 1) & PMD_MASK;
    if (end >= VDSO_BASE)
    end = VDSO_BASE;
    end -= len;
    if (end > start) {
    offset = get_random_u32_below(((end - start) >> PAGE_SHIFT) + 1);
    addr = start + (offset << PAGE_SHIFT);
    } else {
    addr = start;
    }
    return addr;
    }
#[no_mangle]
pub unsafe extern "C" fn vdso_text_size() -> c_ulong {
    unsigned long vdso_text_size(void)
    {
    return PAGE_ALIGN(vdso_end - vdso_start);
    }
#[no_mangle]
pub unsafe extern "C" fn vdso_size() -> c_ulong {
    unsigned long vdso_size(void)
    {
    return vdso_text_size() + VDSO_NR_PAGES * PAGE_SIZE;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_setup_additional_pages(bprm: *mut linux_binprm, uses_interp: c_int) -> c_int {
    int arch_setup_additional_pages(struct linux_binprm *bprm, int uses_interp)
    {
    let mut addr: c_ulong = VDSO_BASE;
    let mut size: c_ulong = vdso_size();
    if (current.flags & PF_RANDOMIZE)
    addr = vdso_addr(current.mm.start_stack + PAGE_SIZE, size);
    return map_vdso(addr, size);
    }
#[no_mangle]
unsafe extern "C" fn vdso_setup_pages(start: *mut c_void, end: *mut c_void) -> *mut *mut page  __init {
    static struct page ** __init vdso_setup_pages(void *start, void *end)
    {
    let mut pages: c_int = (end - start) >> PAGE_SHIFT;
    struct page **pagelist;
    int i;
    pagelist = kzalloc_objs(struct page *, pages + 1);
    if (!pagelist)
    panic("%s: Cannot allocate page list for VDSO", __func__);
    for (i = 0; i < pages; i++)
    pagelist[i] = virt_to_page(start + i * PAGE_SIZE);
    return pagelist;
    }
#[no_mangle]
unsafe extern "C" fn vdso_apply_alternatives() {
    static void vdso_apply_alternatives(void)
    {
    const struct elf64_shdr *alt, *shdr;
    struct alt_instr *start, *end;
    const struct elf64_hdr *hdr;
    hdr = (struct elf64_hdr *)vdso_start;
    shdr = (void *)hdr + hdr.e_shoff;
    alt = find_section(hdr, shdr, ".altinstructions");
    if (!alt)
    return;
    start = (void *)hdr + alt.sh_offset;
    end = (void *)hdr + alt.sh_offset + alt.sh_size;
    apply_alternatives(start, end);
    }
#[no_mangle]
unsafe extern "C" fn vdso_init() -> int __init {
    static int __init vdso_init(void)
    {
    vdso_apply_alternatives();
    vdso_mapping.pages = vdso_setup_pages(vdso_start, vdso_end);
    return 0;
    }
    arch_initcall(vdso_init);

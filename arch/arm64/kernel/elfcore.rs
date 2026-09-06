//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/elfcore.c
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

    if (system_supports_mte())					\
    for (i = 0, m = cprm.vma_meta;				\
    i < cprm.vma_count;				\
    i++, m = cprm.vma_meta + i)			\
    if (m.flags & VM_MTE)
#[no_mangle]
unsafe extern "C" fn mte_vma_tag_dump_size(m: *mut core_vma_metadata) -> c_ulong {
    static unsigned long mte_vma_tag_dump_size(struct core_vma_metadata *m)
    {
    return (m.dump_size >> PAGE_SHIFT) * MTE_PAGE_TAG_STORAGE;
    }
// Derived from dump_user_range(); start/end must be page-aligned
    static int mte_dump_tag_range(struct coredump_params *cprm,
    unsigned long start, unsigned long len)
    {
    let mut ret: c_int = 1;
    unsigned long addr;
    void *tags = core::ptr::null_mut();
    let mut locked: c_int = 0;
    for (addr = start; addr < start + len; addr += PAGE_SIZE) {
    struct page *page = get_dump_page(addr, &locked);
//
// get_dump_page() returns NULL when encountering an empty
// page table entry that would otherwise have been filled with
// the zero page. Skip the equivalent tag dump which would
// have been all zeros.
//
    if (!page) {
    dump_skip(cprm, MTE_PAGE_TAG_STORAGE);
    continue;
    }
//
// Pages mapped in user space as !pte_access_permitted() (e.g.
// PROT_EXEC only) may not have the PG_mte_tagged flag set.
//
    if (!page_mte_tagged(page)) {
    put_page(page);
    dump_skip(cprm, MTE_PAGE_TAG_STORAGE);
    continue;
    }
    if (!tags) {
    tags = mte_allocate_tag_storage();
    if (!tags) {
    put_page(page);
    ret = 0;
    break;
    }
    }
    mte_save_page_tags(page_address(page), tags);
    put_page(page);
    if (!dump_emit(cprm, tags, MTE_PAGE_TAG_STORAGE)) {
    ret = 0;
    break;
    }
    }
    if (tags)
    mte_free_tag_storage(tags);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn elf_core_extra_phdrs(cprm: *mut coredump_params) -> Elf_Half {
    Elf_Half elf_core_extra_phdrs(struct coredump_params *cprm)
    {
    int i;
    struct core_vma_metadata *m;
    let mut vma_count: c_int = 0;
    for_each_mte_vma(cprm, i, m)
    vma_count++;
    return vma_count;
    }
#[no_mangle]
pub unsafe extern "C" fn elf_core_write_extra_phdrs(cprm: *mut coredump_params, offset: loff_t) -> c_int {
    int elf_core_write_extra_phdrs(struct coredump_params *cprm, loff_t offset)
    {
    int i;
    struct core_vma_metadata *m;
    for_each_mte_vma(cprm, i, m) {
    struct elf_phdr phdr;
    phdr.p_type = PT_AARCH64_MEMTAG_MTE;
    phdr.p_offset = offset;
    phdr.p_vaddr = m.start;
    phdr.p_paddr = 0;
    phdr.p_filesz = mte_vma_tag_dump_size(m);
    phdr.p_memsz = m.end - m.start;
    offset += phdr.p_filesz;
    phdr.p_flags = 0;
    phdr.p_align = 0;
    if (!dump_emit(cprm, &phdr, sizeof(phdr)))
    return 0;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn elf_core_extra_data_size(cprm: *mut coredump_params) -> usize {
    size_t elf_core_extra_data_size(struct coredump_params *cprm)
    {
    int i;
    struct core_vma_metadata *m;
    let mut data_size: usize = 0;
    for_each_mte_vma(cprm, i, m)
    data_size += mte_vma_tag_dump_size(m);
    return data_size;
    }
#[no_mangle]
pub unsafe extern "C" fn elf_core_write_extra_data(cprm: *mut coredump_params) -> c_int {
    int elf_core_write_extra_data(struct coredump_params *cprm)
    {
    int i;
    struct core_vma_metadata *m;
    for_each_mte_vma(cprm, i, m) {
    if (!mte_dump_tag_range(cprm, m.start, m.dump_size))
    return 0;
    }
    return 1;
    }

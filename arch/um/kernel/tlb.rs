//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/tlb.c
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
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_ops {
    pub mm_idp: *mut mm_id,
    int (*mmap)(struct mm_id *mm_idp,
    unsigned long virt, unsigned long len, int prot,
    pub offset): int phys_fd, unsigned long long,
    int (*unmap)(struct mm_id *mm_idp,
    pub len): unsigned long virt, unsigned long,
}

    static int kern_map(struct mm_id *mm_idp,
    unsigned long virt, unsigned long len, int prot,
    int phys_fd, unsigned long long offset)
    {
    return os_map_memory((void *)virt, phys_fd, offset, len,
    prot & UM_PROT_READ, prot & UM_PROT_WRITE,
    prot & UM_PROT_EXEC);
    }
    static int kern_unmap(struct mm_id *mm_idp,
    unsigned long virt, unsigned long len)
    {
    return os_unmap_memory((void *)virt, len);
    }
#[no_mangle]
pub unsafe extern "C" fn report_enomem() {
    void report_enomem(void)
    {
    printk(KERN_ERR "UML ran out of memory on the host side! "
    "This can happen due to a memory limitation or "
    "vm.max_map_count has been reached.\n");
    }
    static inline int update_pte_range(pmd_t *pmd, unsigned long addr,
    unsigned long end,
    struct vm_ops *ops)
    {
    pte_t *pte;
    let mut ret: c_int = 0;
    pte = pte_offset_kernel(pmd, addr);
    do {
    if (!pte_needsync(*pte))
    continue;
    if (pte_present(*pte)) {
    __u64 offset;
    let mut phys: c_ulong = pte_val(*pte) & PAGE_MASK;
    let mut fd: c_int = phys_mapping(phys, &offset);
    int r, w, x, prot;
    r = pte_read(*pte);
    w = pte_write(*pte);
    x = pte_exec(*pte);
    if (!pte_young(*pte)) {
    r = 0;
    w = 0;
    } else if (!pte_dirty(*pte))
    w = 0;
    prot = (r ? UM_PROT_READ : 0) |
    (w ? UM_PROT_WRITE : 0) |
    (x ? UM_PROT_EXEC : 0);
    ret = ops.mmap(ops.mm_idp, addr, PAGE_SIZE,
    prot, fd, offset);
    } else
    ret = ops.unmap(ops.mm_idp, addr, PAGE_SIZE);
// pte = pte_mkuptodate(*pte);
    } while (pte++, addr += PAGE_SIZE, ((addr < end) && !ret));
    return ret;
    }
    static inline int update_pmd_range(pud_t *pud, unsigned long addr,
    unsigned long end,
    struct vm_ops *ops)
    {
    pmd_t *pmd;
    unsigned long next;
    let mut ret: c_int = 0;
    pmd = pmd_offset(pud, addr);
    do {
    next = pmd_addr_end(addr, end);
    if (!pmd_present(*pmd)) {
    if (pmd_needsync(*pmd)) {
    ret = ops.unmap(ops.mm_idp, addr,
    next - addr);
    pmd_mkuptodate(*pmd);
    }
    }
    let mut ret: else = update_pte_range(pmd, addr, next, ops);
    } while (pmd++, addr = next, ((addr < end) && !ret));
    return ret;
    }
    static inline int update_pud_range(p4d_t *p4d, unsigned long addr,
    unsigned long end,
    struct vm_ops *ops)
    {
    pud_t *pud;
    unsigned long next;
    let mut ret: c_int = 0;
    pud = pud_offset(p4d, addr);
    do {
    next = pud_addr_end(addr, end);
    if (!pud_present(*pud)) {
    if (pud_needsync(*pud)) {
    ret = ops.unmap(ops.mm_idp, addr,
    next - addr);
    pud_mkuptodate(*pud);
    }
    }
    let mut ret: else = update_pmd_range(pud, addr, next, ops);
    } while (pud++, addr = next, ((addr < end) && !ret));
    return ret;
    }
    static inline int update_p4d_range(pgd_t *pgd, unsigned long addr,
    unsigned long end,
    struct vm_ops *ops)
    {
    p4d_t *p4d;
    unsigned long next;
    let mut ret: c_int = 0;
    p4d = p4d_offset(pgd, addr);
    do {
    next = p4d_addr_end(addr, end);
    if (!p4d_present(*p4d)) {
    if (p4d_needsync(*p4d)) {
    ret = ops.unmap(ops.mm_idp, addr,
    next - addr);
    p4d_mkuptodate(*p4d);
    }
    } else
    ret = update_pud_range(p4d, addr, next, ops);
    } while (p4d++, addr = next, ((addr < end) && !ret));
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn um_tlb_sync(mm: *mut mm_struct) -> c_int {
    int um_tlb_sync(struct mm_struct *mm)
    {
    pgd_t *pgd;
    struct vm_ops ops;
    unsigned long addr, next;
    let mut ret: c_int = 0;
    guard(spinlock_irqsave)(&mm.page_table_lock);
    guard(spinlock_irqsave)(&mm.context.sync_tlb_lock);
    if (mm.context.sync_tlb_range_to == 0)
    return 0;
    ops.mm_idp = &mm.context.id;
    if (mm == &init_mm) {
    ops.mmap = kern_map;
    ops.unmap = kern_unmap;
    } else {
    ops.mmap = map;
    ops.unmap = unmap;
    }
    addr = mm.context.sync_tlb_range_from;
    pgd = pgd_offset(mm, addr);
    do {
    next = pgd_addr_end(addr, mm.context.sync_tlb_range_to);
    if (!pgd_present(*pgd)) {
    if (pgd_needsync(*pgd)) {
    ret = ops.unmap(ops.mm_idp, addr,
    next - addr);
    pgd_mkuptodate(*pgd);
    }
    } else
    ret = update_p4d_range(pgd, addr, next, &ops);
    } while (pgd++, addr = next,
    ((addr < mm.context.sync_tlb_range_to) && !ret));
    if (ret == -ENOMEM)
    report_enomem();
    mm.context.sync_tlb_range_from = 0;
    mm.context.sync_tlb_range_to = 0;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn flush_tlb_all() {
    void flush_tlb_all(void)
    {
//
// Don't bother flushing if this address space is about to be
// destroyed.
//
    if (atomic_read(&current.mm.mm_users) == 0)
    return;
    flush_tlb_mm(current.mm);
    }
#[no_mangle]
pub unsafe extern "C" fn flush_tlb_mm(mm: *mut mm_struct) {
    void flush_tlb_mm(struct mm_struct *mm)
    {
    struct vm_area_struct *vma;
    VMA_ITERATOR(vmi, mm, 0);
    for_each_vma(vmi, vma)
    um_tlb_mark_sync(mm, vma.vm_start, vma.vm_end);
    }

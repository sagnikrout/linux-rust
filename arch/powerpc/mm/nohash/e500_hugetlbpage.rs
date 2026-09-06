//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/nohash/e500_hugetlbpage.c
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
// PPC Huge TLB Page Support for Book3E MMU
//
// Copyright (C) 2009 David Gibson, IBM Corporation.
// Copyright (C) 2011 Becky Bruce, Freescale Semiconductor
//

#[no_mangle]
pub unsafe extern "C" fn tlb1_next() -> c_int {
    static inline int tlb1_next(void)
    {
    struct paca_struct *paca = get_paca();
    struct tlb_core_data *tcd;
    int this, next;
    tcd = paca.tcd_ptr;
    this = tcd.esel_next;
    next = this + 1;
    if (next >= tcd.esel_max)
    next = tcd.esel_first;
    tcd.esel_next = next;
    return this;
    }
#[no_mangle]
pub unsafe extern "C" fn book3e_tlb_lock() {
    static inline void book3e_tlb_lock(void)
    {
    struct paca_struct *paca = get_paca();
    unsigned long tmp;
    let mut token: c_int = smp_processor_id() + 1;
//
// Besides being unnecessary in the absence of SMT, this
// check prevents trying to do lbarx/stbcx. on e5500 which
// doesn't implement either feature.
//
    if (!cpu_has_feature(CPU_FTR_SMT))
    return;
    asm volatile(".machine push;"
    ".machine e6500;"
    "1: lbarx %0, 0, %1;"
    "cmpwi %0, 0;"
    "bne 2f;"
    "stbcx. %2, 0, %1;"
    "bne 1b;"
    "b 3f;"
    "2: lbzx %0, 0, %1;"
    "cmpwi %0, 0;"
    "bne 2b;"
    "b 1b;"
    "3:"
    ".machine pop;"
    : "=&r" (tmp)
    : "r" (&paca.tcd_ptr.lock), "r" (token)
    : "memory");
    }
#[no_mangle]
pub unsafe extern "C" fn book3e_tlb_unlock() {
    static inline void book3e_tlb_unlock(void)
    {
    struct paca_struct *paca = get_paca();
    if (!cpu_has_feature(CPU_FTR_SMT))
    return;
    isync();
    paca.tcd_ptr.lock = 0;
    }

#[no_mangle]
pub unsafe extern "C" fn tlb1_next() -> c_int {
    static inline int tlb1_next(void)
    {
    int index, ncams;
    ncams = mfspr(SPRN_TLB1CFG) & TLBnCFG_N_ENTRY;
    index = this_cpu_read(next_tlbcam_idx);
// Just round-robin the entries and wrap when we hit the end
    if (unlikely(index == ncams - 1))
    __this_cpu_write(next_tlbcam_idx, tlbcam_index);
    else
    __this_cpu_inc(next_tlbcam_idx);
    return index;
    }
#[no_mangle]
pub unsafe extern "C" fn book3e_tlb_lock() {
    static inline void book3e_tlb_lock(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn book3e_tlb_unlock() {
    static inline void book3e_tlb_unlock(void)
    {
    }

#[no_mangle]
pub unsafe extern "C" fn book3e_tlb_exists(ea: c_ulong, pid: c_ulong) -> c_int {
    static inline int book3e_tlb_exists(unsigned long ea, unsigned long pid)
    {
    let mut found: c_int = 0;
    mtspr(SPRN_MAS6, pid << 16);
    asm volatile(
    "tlbsx	0,%1\n"
    "mfspr	%0,0x271\n"
    "srwi	%0,%0,31\n"
    : "=&r"(found) : "r"(ea));
    return found;
    }
    static void
    book3e_hugetlb_preload(struct vm_area_struct *vma, unsigned long ea, pte_t pte)
    {
    unsigned long mas1, mas2;
    u64 mas7_3;
    unsigned long psize, tsize, shift;
    unsigned long flags;
    struct mm_struct *mm;
    int index;
    if (unlikely(is_kernel_addr(ea)))
    return;
    mm = vma.vm_mm;
    psize = vma_mmu_pagesize(vma);
    shift = __ilog2(psize);
    tsize = shift - 10;
//
// We can't be interrupted while we're setting up the MAS
// registers or after we've confirmed that no tlb exists.
//
    local_irq_save(flags);
    book3e_tlb_lock();
    if (unlikely(book3e_tlb_exists(ea, mm.context.id))) {
    book3e_tlb_unlock();
    local_irq_restore(flags);
    return;
    }
// We have to use the CAM(TLB1) on FSL parts for hugepages
    index = tlb1_next();
    mtspr(SPRN_MAS0, MAS0_ESEL(index) | MAS0_TLBSEL(1));
    mas1 = MAS1_VALID | MAS1_TID(mm.context.id) | MAS1_TSIZE(tsize);
    mas2 = ea & ~((1UL << shift) - 1);
    mas2 |= (pte_val(pte) >> PTE_WIMGE_SHIFT) & MAS2_WIMGE_MASK;
    mas7_3 = (u64)pte_pfn(pte) << PAGE_SHIFT;
    mas7_3 |= (pte_val(pte) >> PTE_BAP_SHIFT) & MAS3_BAP_MASK;
    if (!pte_dirty(pte))
    mas7_3 &= ~(MAS3_SW|MAS3_UW);
    mtspr(SPRN_MAS1, mas1);
    mtspr(SPRN_MAS2, mas2);
    if (mmu_has_feature(MMU_FTR_BIG_PHYS))
    mtspr(SPRN_MAS7, upper_32_bits(mas7_3));
    mtspr(SPRN_MAS3, lower_32_bits(mas7_3));
    asm volatile ("tlbwe");
    book3e_tlb_unlock();
    local_irq_restore(flags);
    }
//
// This is called at the end of handling a user page fault, when the
// fault has been handled by updating a PTE in the linux page tables.
//
// This must always be called with the pte lock held.
//
#[no_mangle]
pub unsafe extern "C" fn __update_mmu_cache(vma: *mut vm_area_struct, address: c_ulong, ptep: *mut pte_t) {
    void __update_mmu_cache(struct vm_area_struct *vma, unsigned long address, pte_t *ptep)
    {
    if (is_vm_hugetlb_page(vma))
    book3e_hugetlb_preload(vma, address, *ptep);
    }
#[no_mangle]
pub unsafe extern "C" fn flush_hugetlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong) {
    void flush_hugetlb_page(struct vm_area_struct *vma, unsigned long vmaddr)
    {
    struct hstate *hstate = hstate_file(vma.vm_file);
    let mut tsize: c_ulong = huge_page_shift(hstate) - 10;
    __flush_tlb_page(vma.vm_mm, vmaddr, tsize, 0);
    }

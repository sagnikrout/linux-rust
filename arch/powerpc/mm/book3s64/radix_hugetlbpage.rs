//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/book3s64/radix_hugetlbpage.c
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

#[no_mangle]
pub unsafe extern "C" fn radix__flush_hugetlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong) {
    void radix__flush_hugetlb_page(struct vm_area_struct *vma, unsigned long vmaddr)
    {
    int psize;
    struct hstate *hstate = hstate_file(vma.vm_file);
    psize = hstate_get_psize(hstate);
    radix__flush_tlb_page_psize(vma.vm_mm, vmaddr, psize);
    }
#[no_mangle]
pub unsafe extern "C" fn radix__local_flush_hugetlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong) {
    void radix__local_flush_hugetlb_page(struct vm_area_struct *vma, unsigned long vmaddr)
    {
    int psize;
    struct hstate *hstate = hstate_file(vma.vm_file);
    psize = hstate_get_psize(hstate);
    radix__local_flush_tlb_page_psize(vma.vm_mm, vmaddr, psize);
    }
    void radix__flush_hugetlb_tlb_range(struct vm_area_struct *vma, unsigned long start,
    unsigned long end)
    {
    int psize;
    struct hstate *hstate = hstate_file(vma.vm_file);
    psize = hstate_get_psize(hstate);
//
// Flush PWC even if we get PUD_SIZE hugetlb invalidate to keep this simpler.
//
    if (end - start >= PUD_SIZE)
    radix__flush_tlb_pwc_range_psize(vma.vm_mm, start, end, psize);
    else
    radix__flush_tlb_range_psize(vma.vm_mm, start, end, psize);
    mmu_notifier_arch_invalidate_secondary_tlbs(vma.vm_mm, start, end);
    }
    void radix__huge_ptep_modify_prot_commit(struct vm_area_struct *vma,
    unsigned long addr, pte_t *ptep,
    pte_t old_pte, pte_t pte)
    {
    struct mm_struct *mm = vma.vm_mm;
    let mut psize: c_ulong = huge_page_size(hstate_vma(vma));
//
// POWER9 NMMU must flush the TLB after clearing the PTE before
// installing a PTE with more relaxed access permissions, see
// radix__ptep_set_access_flags.
//
    if (!cpu_has_feature(CPU_FTR_ARCH_31) &&
    is_pte_rw_upgrade(pte_val(old_pte), pte_val(pte)) &&
    atomic_read(&mm.context.copros) > 0)
    radix__flush_hugetlb_page(vma, addr);
    set_huge_pte_at(vma.vm_mm, addr, ptep, pte, psize);
    }

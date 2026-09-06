//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/mm/hugetlbpage.c
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

    pte_t *huge_pte_alloc(struct mm_struct *mm, struct vm_area_struct *vma,
    unsigned long addr, unsigned long sz)
    {
    pgd_t *pgd;
    p4d_t *p4d;
    pud_t *pud;
    pte_t *pte = core::ptr::null_mut();
    pgd = pgd_offset(mm, addr);
    p4d = p4d_alloc(mm, pgd, addr);
    pud = pud_alloc(mm, p4d, addr);
    if (pud)
    pte = (pte_t *)pmd_alloc(mm, pud, addr);
    return pte;
    }
    pte_t *huge_pte_offset(struct mm_struct *mm, unsigned long addr,
    unsigned long sz)
    {
    pgd_t *pgd;
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd = core::ptr::null_mut();
    pgd = pgd_offset(mm, addr);
    if (pgd_present(pgdp_get(pgd))) {
    p4d = p4d_offset(pgd, addr);
    if (p4d_present(p4dp_get(p4d))) {
    pud = pud_offset(p4d, addr);
    if (pud_present(pudp_get(pud)))
    pmd = pmd_offset(pud, addr);
    }
    }
    return (!pmd || pmd_none(pmdp_get(pmd))) ? core::ptr::null_mut() : (pte_t *) pmd;
    }
#[no_mangle]
pub unsafe extern "C" fn pmd_to_entrylo(pmd_val: c_ulong) -> u64 {
    uint64_t pmd_to_entrylo(unsigned long pmd_val)
    {
    uint64_t val;
// PMD as PTE. Must be huge page
    if (!pmd_leaf(__pmd(pmd_val)))
    panic("%s", __func__);
    val = pmd_val ^ _PAGE_HUGE;
    val |= ((val & _PAGE_HGLOBAL) >>
    (_PAGE_HGLOBAL_SHIFT - _PAGE_GLOBAL_SHIFT));
    return val;
    }

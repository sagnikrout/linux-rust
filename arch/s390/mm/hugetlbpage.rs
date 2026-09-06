//! Automatically rewritten from C to Rust
//! Source: arch/s390/mm/hugetlbpage.c
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
// IBM System z Huge TLB Page Support for Kernel.
//
// Copyright IBM Corp. 2007,2020
// Author(s): Gerald Schaefer <gerald.schaefer@de.ibm.com>
//

//
// If the bit selected by single-bit bitmask "a" is set within "x", move
// it to the position indicated by single-bit bitmask "b".
//

#[no_mangle]
pub unsafe extern "C" fn __pte_to_rste(pte: pte_t) -> c_ulong {
    static inline unsigned long __pte_to_rste(pte_t pte)
    {
    swp_entry_t arch_entry;
    unsigned long rste;
//
// Convert encoding		  pte bits	pmd / pud bits
// lIR.uswrdy.p	dy..R...I...wr
// empty			010.000000.0 -> 00..0...1...00
// prot-none, clean, old	111.000000.1 -> 00..1...1...00
// prot-none, clean, young	111.000001.1 -> 01..1...1...00
// prot-none, dirty, old	111.000010.1 -> 10..1...1...00
// prot-none, dirty, young	111.000011.1 -> 11..1...1...00
// read-only, clean, old	111.000100.1 -> 00..1...1...01
// read-only, clean, young	101.000101.1 -> 01..1...0...01
// read-only, dirty, old	111.000110.1 -> 10..1...1...01
// read-only, dirty, young	101.000111.1 -> 11..1...0...01
// read-write, clean, old	111.001100.1 -> 00..1...1...11
// read-write, clean, young	101.001101.1 -> 01..1...0...11
// read-write, dirty, old	110.001110.1 -> 10..0...1...11
// read-write, dirty, young	100.001111.1 -> 11..0...0...11
// HW-bits: R read-only, I invalid
// SW-bits: p present, y young, d dirty, r read, w write, s special,
// u unused, l large
//
    if (pte_present(pte)) {
    rste = pte_val(pte) & PAGE_MASK;
    rste |= _SEGMENT_ENTRY_PRESENT;
    rste |= move_set_bit(pte_val(pte), _PAGE_READ,
    _SEGMENT_ENTRY_READ);
    rste |= move_set_bit(pte_val(pte), _PAGE_WRITE,
    _SEGMENT_ENTRY_WRITE);
    rste |= move_set_bit(pte_val(pte), _PAGE_INVALID,
    _SEGMENT_ENTRY_INVALID);
    rste |= move_set_bit(pte_val(pte), _PAGE_PROTECT,
    _SEGMENT_ENTRY_PROTECT);
    rste |= move_set_bit(pte_val(pte), _PAGE_DIRTY,
    _SEGMENT_ENTRY_DIRTY);
    rste |= move_set_bit(pte_val(pte), _PAGE_YOUNG,
    _SEGMENT_ENTRY_YOUNG);

    rste |= move_set_bit(pte_val(pte), _PAGE_SOFT_DIRTY,
    _SEGMENT_ENTRY_SOFT_DIRTY);

    rste |= move_set_bit(pte_val(pte), _PAGE_NOEXEC,
    _SEGMENT_ENTRY_NOEXEC);
    } else if (!pte_none(pte)) {
// swap pte
    arch_entry = __pte_to_swp_entry(pte);
    rste = mk_swap_rste(__swp_type(arch_entry), __swp_offset(arch_entry));
    } else
    rste = _SEGMENT_ENTRY_EMPTY;
    return rste;
    }
#[no_mangle]
pub unsafe extern "C" fn __rste_to_pte(rste: c_ulong) -> pte_t {
    static inline pte_t __rste_to_pte(unsigned long rste)
    {
    swp_entry_t arch_entry;
    unsigned long pteval;
    int present, none;
    pte_t pte;
    if ((rste & _REGION_ENTRY_TYPE_MASK) == _REGION_ENTRY_TYPE_R3) {
    present = pud_present(__pud(rste));
    none = pud_none(__pud(rste));
    } else {
    present = pmd_present(__pmd(rste));
    none = pmd_none(__pmd(rste));
    }
//
// Convert encoding		pmd / pud bits	    pte bits
// dy..R...I...wr	  lIR.uswrdy.p
// empty			00..0...1...00 -> 010.000000.0
// prot-none, clean, old	00..1...1...00 -> 111.000000.1
// prot-none, clean, young	01..1...1...00 -> 111.000001.1
// prot-none, dirty, old	10..1...1...00 -> 111.000010.1
// prot-none, dirty, young	11..1...1...00 -> 111.000011.1
// read-only, clean, old	00..1...1...01 -> 111.000100.1
// read-only, clean, young	01..1...0...01 -> 101.000101.1
// read-only, dirty, old	10..1...1...01 -> 111.000110.1
// read-only, dirty, young	11..1...0...01 -> 101.000111.1
// read-write, clean, old	00..1...1...11 -> 111.001100.1
// read-write, clean, young	01..1...0...11 -> 101.001101.1
// read-write, dirty, old	10..0...1...11 -> 110.001110.1
// read-write, dirty, young	11..0...0...11 -> 100.001111.1
// HW-bits: R read-only, I invalid
// SW-bits: p present, y young, d dirty, r read, w write, s special,
// u unused, l large
//
    if (present) {
    pteval = rste & _SEGMENT_ENTRY_ORIGIN_LARGE;
    pteval |= _PAGE_LARGE | _PAGE_PRESENT;
    pteval |= move_set_bit(rste, _SEGMENT_ENTRY_READ, _PAGE_READ);
    pteval |= move_set_bit(rste, _SEGMENT_ENTRY_WRITE, _PAGE_WRITE);
    pteval |= move_set_bit(rste, _SEGMENT_ENTRY_INVALID, _PAGE_INVALID);
    pteval |= move_set_bit(rste, _SEGMENT_ENTRY_PROTECT, _PAGE_PROTECT);
    pteval |= move_set_bit(rste, _SEGMENT_ENTRY_DIRTY, _PAGE_DIRTY);
    pteval |= move_set_bit(rste, _SEGMENT_ENTRY_YOUNG, _PAGE_YOUNG);

    pteval |= move_set_bit(rste, _SEGMENT_ENTRY_SOFT_DIRTY, _PAGE_SOFT_DIRTY);

    pteval |= move_set_bit(rste, _SEGMENT_ENTRY_NOEXEC, _PAGE_NOEXEC);
    } else if (!none) {
// swap rste
    arch_entry = __rste_to_swp_entry(rste);
    pte = mk_swap_pte(__swp_type_rste(arch_entry), __swp_offset_rste(arch_entry));
    pteval = pte_val(pte);
    } else
    pteval = _PAGE_INVALID;
    return __pte(pteval);
    }
    void __set_huge_pte_at(struct mm_struct *mm, unsigned long addr,
    pte_t *ptep, pte_t pte)
    {
    unsigned long rste;
    rste = __pte_to_rste(pte);
// Set correct table type for 2G hugepages
    if ((pte_val(ptep_get(ptep)) & _REGION_ENTRY_TYPE_MASK) == _REGION_ENTRY_TYPE_R3) {
    if (likely(pte_present(pte)))
    rste |= _REGION3_ENTRY_LARGE;
    rste |= _REGION_ENTRY_TYPE_R3;
    set_pud((pud_t *)ptep, __pud(rste));
    } else {
    if (likely(pte_present(pte)))
    rste |= _SEGMENT_ENTRY_LARGE;
    set_pmd((pmd_t *)ptep, __pmd(rste));
    }
    }
    void set_huge_pte_at(struct mm_struct *mm, unsigned long addr,
    pte_t *ptep, pte_t pte, unsigned long sz)
    {
    __set_huge_pte_at(mm, addr, ptep, pte);
    }
#[no_mangle]
pub unsafe extern "C" fn huge_ptep_get(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t) -> pte_t {
    pte_t huge_ptep_get(struct mm_struct *mm, unsigned long addr, pte_t *ptep)
    {
    return __rste_to_pte(pte_val(ptep_get(ptep)));
    }
    pte_t __huge_ptep_get_and_clear(struct mm_struct *mm,
    unsigned long addr, pte_t *ptep)
    {
    let mut pte: pte_t = huge_ptep_get(mm, addr, ptep);
    pmd_t *pmdp = (pmd_t *) ptep;
    pud_t *pudp = (pud_t *) ptep;
    if ((pte_val(ptep_get(ptep)) & _REGION_ENTRY_TYPE_MASK) == _REGION_ENTRY_TYPE_R3)
    pudp_xchg_direct(mm, addr, pudp, __pud(_REGION3_ENTRY_EMPTY));
    else
    pmdp_xchg_direct(mm, addr, pmdp, __pmd(_SEGMENT_ENTRY_EMPTY));
    return pte;
    }
    pte_t *huge_pte_alloc(struct mm_struct *mm, struct vm_area_struct *vma,
    unsigned long addr, unsigned long sz)
    {
    pgd_t *pgdp;
    p4d_t *p4dp;
    pud_t *pudp;
    pmd_t *pmdp = core::ptr::null_mut();
    pgdp = pgd_offset(mm, addr);
    p4dp = p4d_alloc(mm, pgdp, addr);
    if (p4dp) {
    pudp = pud_alloc(mm, p4dp, addr);
    if (pudp) {
    if (sz == PUD_SIZE)
    return (pte_t *) pudp;
#[no_mangle]
pub unsafe extern "C" fn if(PMD_SIZE: sz ==) -> else {
    else if (sz == PMD_SIZE)
    pmdp = pmd_alloc(mm, pudp, addr);
    }
    }
    return (pte_t *) pmdp;
    }
    pte_t *huge_pte_offset(struct mm_struct *mm,
    unsigned long addr, unsigned long sz)
    {
    pgd_t *pgdp;
    p4d_t *p4dp;
    pud_t *pudp;
    pmd_t *pmdp = core::ptr::null_mut();
    pgdp = pgd_offset(mm, addr);
    if (pgd_present(pgdp_get(pgdp))) {
    p4dp = p4d_offset(pgdp, addr);
    if (p4d_present(p4dp_get(p4dp))) {
    pudp = pud_offset(p4dp, addr);
    if (sz == PUD_SIZE)
    return (pte_t *)pudp;
    if (pud_present(pudp_get(pudp)))
    pmdp = pmd_offset(pudp, addr);
    }
    }
    return (pte_t *) pmdp;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_hugetlb_valid_size(size: c_ulong) -> bool __init {
    bool __init arch_hugetlb_valid_size(unsigned long size)
    {
    if (cpu_has_edat1() && size == PMD_SIZE)
    return true;
#[no_mangle]
pub unsafe extern "C" fn if(PUD_SIZE: cpu_has_edat2() && size ==) -> else {
    else if (cpu_has_edat2() && size == PUD_SIZE)
    return true;
    else
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_hugetlb_cma_order() -> unsigned int __init {
    unsigned int __init arch_hugetlb_cma_order(void)
    {
    if (cpu_has_edat2())
    return PUD_SHIFT - PAGE_SHIFT;
    return 0;
    }

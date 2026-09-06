//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/pgtable-frag.c
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
// Handling Page Tables through page fragments
//

#[no_mangle]
pub unsafe extern "C" fn pte_frag_destroy(pte_frag: *mut c_void) {
    void pte_frag_destroy(void *pte_frag)
    {
    int count;
    struct ptdesc *ptdesc;
    ptdesc = virt_to_ptdesc(pte_frag);
// drop all the pending references
    count = ((unsigned long)pte_frag & ~PAGE_MASK) >> PTE_FRAG_SIZE_SHIFT;
// We allow PTE_FRAG_NR fragments from a PTE page
    if (atomic_sub_and_test(PTE_FRAG_NR - count, &ptdesc.pt_frag_refcount)) {
    folio_clear_active(ptdesc_folio(ptdesc));
    pagetable_dtor(ptdesc);
    pagetable_free(ptdesc);
    }
    }
    static pte_t *get_pte_from_cache(struct mm_struct *mm)
    {
    void *pte_frag, *ret;
    if (PTE_FRAG_NR == 1)
    return core::ptr::null_mut();
    spin_lock(&mm.page_table_lock);
    ret = pte_frag_get(&mm.context);
    if (ret) {
    pte_frag = ret + PTE_FRAG_SIZE;
//
// If we have taken up all the fragments mark PTE page NULL
//
    if (((unsigned long)pte_frag & ~PAGE_MASK) == 0)
    pte_frag = core::ptr::null_mut();
    pte_frag_set(&mm.context, pte_frag);
    }
    spin_unlock(&mm.page_table_lock);
    return (pte_t *)ret;
    }
    static pte_t *__alloc_for_ptecache(struct mm_struct *mm, int kernel)
    {
    void *ret = core::ptr::null_mut();
    struct ptdesc *ptdesc;
    let mut gfp: gfp_t = PGALLOC_GFP;
    if (!kernel)
    gfp |= __GFP_ACCOUNT;
    ptdesc = pagetable_alloc(gfp, 0);
    if (!ptdesc)
    return core::ptr::null_mut();
    if (!pagetable_pte_ctor(mm, ptdesc)) {
    pagetable_free(ptdesc);
    return core::ptr::null_mut();
    }
    atomic_set(&ptdesc.pt_frag_refcount, 1);
    ret = ptdesc_address(ptdesc);
//
// if we support only one fragment just return the
// allocated page.
//
    if (PTE_FRAG_NR == 1)
    return ret;
    spin_lock(&mm.page_table_lock);
//
// If we find ptdesc_page set, we return
// the allocated page with single fragment
// count.
//
    if (likely(!pte_frag_get(&mm.context))) {
    atomic_set(&ptdesc.pt_frag_refcount, PTE_FRAG_NR);
    pte_frag_set(&mm.context, ret + PTE_FRAG_SIZE);
    }
    spin_unlock(&mm.page_table_lock);
    return (pte_t *)ret;
    }
    pte_t *pte_fragment_alloc(struct mm_struct *mm, int kernel)
    {
    pte_t *pte;
    pte = get_pte_from_cache(mm);
    if (pte)
    return pte;
    return __alloc_for_ptecache(mm, kernel);
    }
#[no_mangle]
unsafe extern "C" fn pte_free_now(head: *mut rcu_head) {
    static void pte_free_now(struct rcu_head *head)
    {
    struct ptdesc *ptdesc;
    ptdesc = container_of(head, struct ptdesc, pt_rcu_head);
    pagetable_dtor(ptdesc);
    pagetable_free(ptdesc);
    }
#[no_mangle]
pub unsafe extern "C" fn pte_fragment_free(table: *mut c_ulong, kernel: c_int) {
    void pte_fragment_free(unsigned long *table, int kernel)
    {
    struct ptdesc *ptdesc = virt_to_ptdesc(table);
    if (pagetable_is_reserved(ptdesc))
    return free_reserved_ptdesc(ptdesc);
    BUG_ON(atomic_read(&ptdesc.pt_frag_refcount) <= 0);
    if (atomic_dec_and_test(&ptdesc.pt_frag_refcount)) {
    if (kernel || !folio_test_clear_active(ptdesc_folio(ptdesc)))
    pte_free_now(&ptdesc.pt_rcu_head);
    else
    call_rcu(&ptdesc.pt_rcu_head, pte_free_now);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn pte_free_defer(mm: *mut mm_struct, pgtable: pgtable_t) {
    void pte_free_defer(struct mm_struct *mm, pgtable_t pgtable)
    {
    struct folio *folio;
    folio = virt_to_folio(pgtable);
    folio_set_active(folio);
    pte_fragment_free((unsigned long *)pgtable, 0);
    }

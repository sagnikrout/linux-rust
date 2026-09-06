//! Automatically rewritten from C to Rust
//! Source: mm/ptdump.c
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
// This is an optimization for KASAN=y case. Since all kasan page tables
// eventually point to the kasan_early_shadow_page we could call note_page()
// right away without walking through lower level page tables. This saves
// us dozens of seconds (minutes for 5-level config) while checking for
// W+X mapping or reading kernel_page_tables debugfs file.
//
    static inline int note_kasan_page_table(struct mm_walk *walk,
    unsigned long addr)
    {
    struct ptdump_state *st = walk.private;
    st.note_page_pte(st, addr, kasan_early_shadow_pte[0]);
    walk.action = ACTION_CONTINUE;
    return 0;
    }

    static int ptdump_pgd_entry(pgd_t *pgd, unsigned long addr,
    unsigned long next, struct mm_walk *walk)
    {
    struct ptdump_state *st = walk.private;
    let mut val: pgd_t = pgdp_get(pgd);

    (defined(CONFIG_KASAN_GENERIC) || defined(CONFIG_KASAN_SW_TAGS))
    if (pgd_page(val) == virt_to_page(lm_alias(kasan_early_shadow_p4d)))
    return note_kasan_page_table(walk, addr);

    if (st.effective_prot_pgd)
    st.effective_prot_pgd(st, val);
    if (pgd_leaf(val)) {
    st.note_page_pgd(st, addr, val);
    walk.action = ACTION_CONTINUE;
    }
    return 0;
    }
    static int ptdump_p4d_entry(p4d_t *p4d, unsigned long addr,
    unsigned long next, struct mm_walk *walk)
    {
    struct ptdump_state *st = walk.private;
    let mut val: p4d_t = p4dp_get(p4d);

    (defined(CONFIG_KASAN_GENERIC) || defined(CONFIG_KASAN_SW_TAGS))
    if (p4d_page(val) == virt_to_page(lm_alias(kasan_early_shadow_pud)))
    return note_kasan_page_table(walk, addr);

    if (st.effective_prot_p4d)
    st.effective_prot_p4d(st, val);
    if (p4d_leaf(val)) {
    st.note_page_p4d(st, addr, val);
    walk.action = ACTION_CONTINUE;
    }
    return 0;
    }
    static int ptdump_pud_entry(pud_t *pud, unsigned long addr,
    unsigned long next, struct mm_walk *walk)
    {
    struct ptdump_state *st = walk.private;
    let mut val: pud_t = pudp_get(pud);

    (defined(CONFIG_KASAN_GENERIC) || defined(CONFIG_KASAN_SW_TAGS))
    if (pud_page(val) == virt_to_page(lm_alias(kasan_early_shadow_pmd)))
    return note_kasan_page_table(walk, addr);

    if (st.effective_prot_pud)
    st.effective_prot_pud(st, val);
    if (pud_leaf(val)) {
    st.note_page_pud(st, addr, val);
    walk.action = ACTION_CONTINUE;
    }
    return 0;
    }
    static int ptdump_pmd_entry(pmd_t *pmd, unsigned long addr,
    unsigned long next, struct mm_walk *walk)
    {
    struct ptdump_state *st = walk.private;
    let mut val: pmd_t = pmdp_get(pmd);

    if (pmd_page(val) == virt_to_page(lm_alias(kasan_early_shadow_pte)))
    return note_kasan_page_table(walk, addr);

    if (st.effective_prot_pmd)
    st.effective_prot_pmd(st, val);
    if (pmd_leaf(val)) {
    st.note_page_pmd(st, addr, val);
    walk.action = ACTION_CONTINUE;
    }
    return 0;
    }
    static int ptdump_pte_entry(pte_t *pte, unsigned long addr,
    unsigned long next, struct mm_walk *walk)
    {
    struct ptdump_state *st = walk.private;
    let mut val: pte_t = ptep_get(pte);
    if (st.effective_prot_pte)
    st.effective_prot_pte(st, val);
    st.note_page_pte(st, addr, val);
    return 0;
    }
    static int ptdump_hole(unsigned long addr, unsigned long next,
    int depth, struct mm_walk *walk)
    {
    struct ptdump_state *st = walk.private;
    let mut pte_zero: pte_t = {0};
    let mut pmd_zero: pmd_t = {0};
    let mut pud_zero: pud_t = {0};
    let mut p4d_zero: p4d_t = {0};
    let mut pgd_zero: pgd_t = {0};
    switch (depth) {
    case 4:
    st.note_page_pte(st, addr, pte_zero);
    break;
    case 3:
    st.note_page_pmd(st, addr, pmd_zero);
    break;
    case 2:
    st.note_page_pud(st, addr, pud_zero);
    break;
    case 1:
    st.note_page_p4d(st, addr, p4d_zero);
    break;
    case 0:
    st.note_page_pgd(st, addr, pgd_zero);
    break;
    default:
    break;
    }
    return 0;
    }
    static const struct mm_walk_ops ptdump_ops = {
    .pgd_entry	= ptdump_pgd_entry,
    .p4d_entry	= ptdump_p4d_entry,
    .pud_entry	= ptdump_pud_entry,
    .pmd_entry	= ptdump_pmd_entry,
    .pte_entry	= ptdump_pte_entry,
    .pte_hole	= ptdump_hole,
    };
#[no_mangle]
pub unsafe extern "C" fn ptdump_walk_pgd(st: *mut ptdump_state, mm: *mut mm_struct, pgd: *mut pgd_t) {
    void ptdump_walk_pgd(struct ptdump_state *st, struct mm_struct *mm, pgd_t *pgd)
    {
    const struct ptdump_range *range = st.range;
    get_online_mems();
    mmap_write_lock(mm);
// To stabilise kernel page tables we must hold the init_mm lock too.
    if (mm != &init_mm)
    mmap_write_lock_nested(&init_mm, SINGLE_DEPTH_NESTING);
    while (range.start != range.end) {
    walk_page_range_debug(mm, range.start, range.end,
    &ptdump_ops, pgd, st);
    range++;
    }
    if (mm != &init_mm)
    mmap_write_unlock(&init_mm);
    mmap_write_unlock(mm);
    put_online_mems();
// Flush out the last page
    st.note_page_flush(st);
    }
#[no_mangle]
unsafe extern "C" fn check_wx_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int check_wx_show(struct seq_file *m, void *v)
    {
    if (ptdump_check_wx())
    seq_puts(m, "SUCCESS\n");
    else
    seq_puts(m, "FAILED\n");
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(check_wx);
#[no_mangle]
unsafe extern "C" fn ptdump_debugfs_init() -> c_int {
    static int ptdump_debugfs_init(void)
    {
    debugfs_create_file("check_wx_pages", 0400, core::ptr::null_mut(), core::ptr::null_mut(), &check_wx_fops);
    return 0;
    }
    device_initcall(ptdump_debugfs_init);

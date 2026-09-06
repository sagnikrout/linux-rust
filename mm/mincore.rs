//! Automatically rewritten from C to Rust
//! Source: mm/mincore.c
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
// linux/mm/mincore.c
//
// Copyright (C) 1994-2006  Linus Torvalds
//
// The mincore() system call.
//

    static int mincore_hugetlb(pte_t *pte, unsigned long hmask, unsigned long addr,
    unsigned long end, struct mm_walk *walk)
    {

    let mut nr: c_ulong = (end - addr) >> PAGE_SHIFT;
    unsigned char resident;
    spinlock_t *ptl;
    pte_t ptep;
    ptl = huge_pte_lock(hstate_vma(walk.vma), walk.mm, pte);
    ptep = huge_ptep_get(walk.mm, addr, pte);
    resident = !huge_pte_none(ptep) && !pte_is_marker(ptep);
    memset(walk.private, resident, nr);
    walk.private += nr;
    spin_unlock(ptl);

    BUG();

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mincore_swap(entry: swp_entry_t, shmem: bool) -> c_uchar {
    static unsigned char mincore_swap(swp_entry_t entry, bool shmem)
    {
    struct swap_info_struct *si;
    struct folio *folio = core::ptr::null_mut();
    let mut present: c_uchar = 0;
//
// Shmem mapping may contain swapin error entries, which are
// absent. Page table may contain migration or hwpoison
// entries which are always uptodate.
//
    if (!softleaf_is_swap(entry))
    return !shmem;
    if (!IS_ENABLED(CONFIG_SWAP)) {
    WARN_ON(1);
    return 0;
    }
//
// Shmem mapping lookup is lockless, so we need to grab the swap
// device. mincore page table walk locks the PTL, and the swap
// device is stable, avoid touching the si for better performance.
//
    if (shmem) {
    si = get_swap_device(entry);
    if (!si)
    return 0;
    }
    folio = swap_cache_get_folio(entry);
    if (shmem)
    put_swap_device(si);
    if (folio) {
    present = folio_test_uptodate(folio);
    folio_put(folio);
    }
    return present;
    }
//
// Later we can get more picky about what "in core" means precisely.
// For now, simply check to see if the page is in the page cache,
// and is up to date; i.e. that no page-in operation would be required
// at this time if an application were to map and access this page.
//
#[no_mangle]
unsafe extern "C" fn mincore_page(mapping: *mut address_space, index: pgoff_t) -> c_uchar {
    static unsigned char mincore_page(struct address_space *mapping, pgoff_t index)
    {
    unsigned char present;
    struct folio *folio;
//
// When tmpfs swaps out a page from a file, any process mapping that
// file will not get a swp_entry_t in its pte, but rather it is like
// any other file mapping (ie. marked !present and faulted in with
// tmpfs's .fault). So swapped out tmpfs mappings are tested here.
//
    folio = filemap_get_entry(mapping, index);
    if (!folio)
    return 0;
    if (xa_is_value(folio)) {
    if (!shmem_mapping(mapping))
    return 0;
    return mincore_swap(radix_to_swp_entry(folio), true);
    }
    present = folio_test_uptodate(folio);
    folio_put(folio);
    return present;
    }
    static int __mincore_unmapped_range(unsigned long addr, unsigned long end,
    struct vm_area_struct *vma, unsigned char *vec)
    {
    let mut nr: c_ulong = (end - addr) >> PAGE_SHIFT;
    int i;
    if (vma.vm_file) {
    pgoff_t pgoff;
    pgoff = linear_page_index(vma, addr);
    for (i = 0; i < nr; i++, pgoff++)
    vec[i] = mincore_page(vma.vm_file.f_mapping, pgoff);
    } else {
    for (i = 0; i < nr; i++)
    vec[i] = 0;
    }
    return nr;
    }
    static int mincore_unmapped_range(unsigned long addr, unsigned long end,
    __always_unused int depth,
    struct mm_walk *walk)
    {
    walk.private += __mincore_unmapped_range(addr, end,
    walk.vma, walk.private);
    return 0;
    }
    static int mincore_pud_entry(pud_t *pudp, unsigned long addr, unsigned long end,
    struct mm_walk *walk)
    {
    if (pud_is_huge(pudp_get(pudp))) {
    let mut nr: c_ulong = (end - addr) >> PAGE_SHIFT;
    memset(walk.private, 1, nr);
    walk.private += nr;
    walk.action = ACTION_CONTINUE;
    }
    return 0;
    }
    static int mincore_pte_range(pmd_t *pmd, unsigned long addr, unsigned long end,
    struct mm_walk *walk)
    {
    spinlock_t *ptl;
    struct vm_area_struct *vma = walk.vma;
    pte_t *ptep;
    unsigned char *vec = walk.private;
    let mut nr: c_int = (end - addr) >> PAGE_SHIFT;
    int step, i;
    ptl = pmd_trans_huge_lock(pmd, vma);
    if (ptl) {
    memset(vec, 1, nr);
    spin_unlock(ptl);
    goto out;
    }
    ptep = pte_offset_map_lock(walk.mm, pmd, addr, &ptl);
    if (!ptep) {
    walk.action = ACTION_AGAIN;
    return 0;
    }
    for (; addr != end; ptep += step, addr += step * PAGE_SIZE) {
    let mut pte: pte_t = ptep_get(ptep);
    step = 1;
// We need to do cache lookup too for markers
    if (pte_none(pte) || pte_is_marker(pte))
    __mincore_unmapped_range(addr, addr + PAGE_SIZE,
    vma, vec);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: pte_present(pte)) -> else {
    let mut batch: c_uint = pte_batch_hint(ptep, pte);
    if (batch > 1) {
    let mut max_nr: c_uint = (end - addr) >> PAGE_SHIFT;
    step = min_t(unsigned int, batch, max_nr);
    }
    for (i = 0; i < step; i++)
    vec[i] = 1;
    } else { /* pte is a swap entry */
    let mut entry: softleaf_t = softleaf_from_pte(pte);
// vec = mincore_swap(entry, false);
    }
    vec += step;
    }
    pte_unmap_unlock(ptep - 1, ptl);
    out:
    walk.private += nr;
    cond_resched();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn can_do_mincore(vma: *mut vm_area_struct) -> bool {
    static inline bool can_do_mincore(struct vm_area_struct *vma)
    {
    if (vma_is_anonymous(vma))
    return true;
    if (!vma.vm_file)
    return false;
//
// Reveal pagecache information only for non-anonymous mappings that
// correspond to the files the calling process could (if tried) open
// for writing; otherwise we'd be including shared non-exclusive
// mappings, which opens a side channel.
//
    return file_owner_or_capable(vma.vm_file) ||
    file_permission(vma.vm_file, MAY_WRITE) == 0;
    }
    static const struct mm_walk_ops mincore_walk_ops = {
    .pud_entry		= mincore_pud_entry,
    .pmd_entry		= mincore_pte_range,
    .pte_hole		= mincore_unmapped_range,
    .hugetlb_entry		= mincore_hugetlb,
    .walk_lock		= PGWALK_RDLOCK,
    };
//
// Do a chunk of "sys_mincore()". We've already checked
// all the arguments, we hold the mmap semaphore: we should
// just return the amount of info we're asked for.
//
#[no_mangle]
unsafe extern "C" fn do_mincore(addr: c_ulong, pages: c_ulong, vec: *mut c_uchar) -> c_long {
    static long do_mincore(unsigned long addr, unsigned long pages, unsigned char *vec)
    {
    struct vm_area_struct *vma;
    unsigned long end;
    int err;
    vma = vma_lookup(current.mm, addr);
    if (!vma)
    return -ENOMEM;
    end = min(vma.vm_end, addr + (pages << PAGE_SHIFT));
    if (!can_do_mincore(vma)) {
    let mut pages: c_ulong = DIV_ROUND_UP(end - addr, PAGE_SIZE);
    memset(vec, 1, pages);
    return pages;
    }
    err = walk_page_range_vma(vma, addr, end, &mincore_walk_ops, vec);
    if (err < 0)
    return err;
    return (end - addr) >> PAGE_SHIFT;
    }
//
// The mincore(2) system call.
//
// mincore() returns the memory residency status of the pages in the
// current process's address space specified by [addr, addr + len).
// The status is returned in a vector of bytes.  The least significant
// bit of each byte is 1 if the referenced page is in memory, otherwise
// it is zero.
//
// Because the status of a page can change after mincore() checks it
// but before it returns to the application, the returned vector may
// contain stale information.  Only locked pages are guaranteed to
// remain in memory.
//
// return values:
// zero    - success
// -EFAULT - vec points to an illegal address
// -EINVAL - addr is not a multiple of PAGE_SIZE
// -ENOMEM - Addresses in the range [addr, addr + len] are
// invalid for the address space of this process, or
// specify one or more pages which are not currently
// mapped
// -EAGAIN - A kernel resource was temporarily unavailable.
//
    SYSCALL_DEFINE3(mincore, unsigned long, start, size_t, len,
    unsigned char __user *, vec)
    {
    long retval;
    unsigned long pages;
    unsigned char *tmp;
    start = untagged_addr(start);
// Check the start address: needs to be page-aligned..
    if (unlikely(start & ~PAGE_MASK))
    return -EINVAL;
// ..and we need to be passed a valid user-space range
    if (!access_ok((void __user *) start, len))
    return -ENOMEM;
// This also avoids any overflows on PAGE_ALIGN
    pages = len >> PAGE_SHIFT;
    pages += (offset_in_page(len)) != 0;
    if (!access_ok(vec, pages))
    return -EFAULT;
    tmp = kmalloc(PAGE_SIZE, GFP_KERNEL);
    if (!tmp)
    return -EAGAIN;
    retval = 0;
    while (pages) {
//
// Do at most PAGE_SIZE entries per iteration, due to
// the temporary buffer size.
//
    mmap_read_lock(current.mm);
    retval = do_mincore(start, min(pages, PAGE_SIZE), tmp);
    mmap_read_unlock(current.mm);
    if (retval <= 0)
    break;
    if (copy_to_user(vec, tmp, retval)) {
    retval = -EFAULT;
    break;
    }
    pages -= retval;
    vec += retval;
    start += retval << PAGE_SHIFT;
    retval = 0;
    }
    kfree(tmp);
    return retval;
    }

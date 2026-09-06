//! Automatically rewritten from C to Rust
//! Source: mm/mlock.c
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
// linux/mm/mlock.c
//
// (C) Copyright 1995 Linus Torvalds
// (C) Copyright 2002 Christoph Hellwig
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlock_fbatch {
    pub lock: local_lock_t,
    pub fbatch: folio_batch,
}

    static DEFINE_PER_CPU(struct mlock_fbatch, mlock_fbatch) = {
    .lock = INIT_LOCAL_LOCK(lock),
    };
#[no_mangle]
pub unsafe extern "C" fn can_do_mlock() -> bool {
    bool can_do_mlock(void)
    {
    if (rlimit(RLIMIT_MEMLOCK) != 0)
    return true;
    if (capable(CAP_IPC_LOCK))
    return true;
    return false;
    }
    EXPORT_SYMBOL(can_do_mlock);
//
// Mlocked folios are marked with the PG_mlocked flag for efficient testing
// in vmscan and, possibly, the fault path; and to support semi-accurate
// statistics.
//
// An mlocked folio [folio_test_mlocked(folio)] is unevictable.  As such, it
// will be ostensibly placed on the LRU "unevictable" list (actually no such
// list exists), rather than the [in]active lists. PG_unevictable is set to
// indicate the unevictable state.
//
    static struct lruvec *__mlock_folio(struct folio *folio, struct lruvec *lruvec)
    {
// There is nothing more we can do while it's off LRU
    if (!folio_test_clear_lru(folio))
    return lruvec;
    lruvec = folio_lruvec_relock_irq(folio, lruvec);
    if (unlikely(folio_evictable(folio))) {
//
// This is a little surprising, but quite possible: PG_mlocked
// must have got cleared already by another CPU.  Could this
// folio be unevictable?  I'm not sure, but move it now if so.
//
    if (folio_test_unevictable(folio)) {
    lruvec_del_folio(lruvec, folio);
    folio_clear_unevictable(folio);
    lruvec_add_folio(lruvec, folio);
    __count_vm_events(UNEVICTABLE_PGRESCUED,
    folio_nr_pages(folio));
    }
    goto out;
    }
    if (folio_test_unevictable(folio)) {
    if (folio_test_mlocked(folio))
    folio.mlock_count++;
    goto out;
    }
    lruvec_del_folio(lruvec, folio);
    folio_clear_active(folio);
    folio_set_unevictable(folio);
    folio.mlock_count = !!folio_test_mlocked(folio);
    lruvec_add_folio(lruvec, folio);
    __count_vm_events(UNEVICTABLE_PGCULLED, folio_nr_pages(folio));
    out:
    folio_set_lru(folio);
    return lruvec;
    }
    static struct lruvec *__mlock_new_folio(struct folio *folio, struct lruvec *lruvec)
    {
    VM_BUG_ON_FOLIO(folio_test_lru(folio), folio);
    lruvec = folio_lruvec_relock_irq(folio, lruvec);
// As above, this is a little surprising, but possible
    if (unlikely(folio_evictable(folio)))
    goto out;
    folio_set_unevictable(folio);
    folio.mlock_count = !!folio_test_mlocked(folio);
    __count_vm_events(UNEVICTABLE_PGCULLED, folio_nr_pages(folio));
    out:
    lruvec_add_folio(lruvec, folio);
    folio_set_lru(folio);
    return lruvec;
    }
    static struct lruvec *__munlock_folio(struct folio *folio, struct lruvec *lruvec)
    {
    let mut nr_pages: c_int = folio_nr_pages(folio);
    let mut isolated: bool = false;
    if (!folio_test_clear_lru(folio))
    goto munlock;
    isolated = true;
    lruvec = folio_lruvec_relock_irq(folio, lruvec);
    if (folio_test_unevictable(folio)) {
// Then mlock_count is maintained, but might undercount
    if (folio.mlock_count)
    folio.mlock_count--;
    if (folio.mlock_count)
    goto out;
    }
// else assume that was the last mlock: reclaim will fix it if not
    munlock:
    if (folio_test_clear_mlocked(folio)) {
    __zone_stat_mod_folio(folio, NR_MLOCK, -nr_pages);
    if (isolated || !folio_test_unevictable(folio))
    __count_vm_events(UNEVICTABLE_PGMUNLOCKED, nr_pages);
    else
    __count_vm_events(UNEVICTABLE_PGSTRANDED, nr_pages);
    }
// folio_evictable() has to be checked *after* clearing Mlocked
    if (isolated && folio_test_unevictable(folio) && folio_evictable(folio)) {
    lruvec_del_folio(lruvec, folio);
    folio_clear_unevictable(folio);
    lruvec_add_folio(lruvec, folio);
    __count_vm_events(UNEVICTABLE_PGRESCUED, nr_pages);
    }
    out:
    if (isolated)
    folio_set_lru(folio);
    return lruvec;
    }
//
// Flags held in the low bits of a struct folio pointer on the mlock_fbatch.
//
pub const LRU_FOLIO: c_uint = 0x1;
pub const NEW_FOLIO: c_uint = 0x2;
    static inline struct folio *mlock_lru(struct folio *folio)
    {
    return (struct folio *)((unsigned long)folio + LRU_FOLIO);
    }
    static inline struct folio *mlock_new(struct folio *folio)
    {
    return (struct folio *)((unsigned long)folio + NEW_FOLIO);
    }
//
// mlock_folio_batch() is derived from folio_batch_move_lru(): perhaps that can
// make use of such folio pointer flags in future, but for now just keep it for
// mlock.  We could use three separate folio batches instead, but one feels
// better (munlocking a full folio batch does not need to drain mlocking folio
// batches first).
//
#[no_mangle]
unsafe extern "C" fn mlock_folio_batch(fbatch: *mut folio_batch) {
    static void mlock_folio_batch(struct folio_batch *fbatch)
    {
    struct lruvec *lruvec = core::ptr::null_mut();
    unsigned long mlock;
    struct folio *folio;
    int i;
    for (i = 0; i < folio_batch_count(fbatch); i++) {
    folio = fbatch.folios[i];
    mlock = (unsigned long)folio & (LRU_FOLIO | NEW_FOLIO);
    folio = (struct folio *)((unsigned long)folio - mlock);
    fbatch.folios[i] = folio;
    if (mlock & LRU_FOLIO)
    lruvec = __mlock_folio(folio, lruvec);
#[no_mangle]
pub unsafe extern "C" fn if(NEW_FOLIO: mlock &) -> else {
    else if (mlock & NEW_FOLIO)
    lruvec = __mlock_new_folio(folio, lruvec);
    else
    lruvec = __munlock_folio(folio, lruvec);
    }
    if (lruvec)
    lruvec_unlock_irq(lruvec);
    folios_put(fbatch);
    }
#[no_mangle]
pub unsafe extern "C" fn mlock_drain_local() {
    void mlock_drain_local(void)
    {
    struct folio_batch *fbatch;
    local_lock(&mlock_fbatch.lock);
    fbatch = this_cpu_ptr(&mlock_fbatch.fbatch);
    if (folio_batch_count(fbatch))
    mlock_folio_batch(fbatch);
    local_unlock(&mlock_fbatch.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn mlock_drain_remote(cpu: c_int) {
    void mlock_drain_remote(int cpu)
    {
    struct folio_batch *fbatch;
    WARN_ON_ONCE(cpu_online(cpu));
    fbatch = &per_cpu(mlock_fbatch.fbatch, cpu);
    if (folio_batch_count(fbatch))
    mlock_folio_batch(fbatch);
    }
#[no_mangle]
pub unsafe extern "C" fn need_mlock_drain(cpu: c_int) -> bool {
    bool need_mlock_drain(int cpu)
    {
    return folio_batch_count(&per_cpu(mlock_fbatch.fbatch, cpu));
    }
//
// mlock_folio - mlock a folio already on (or temporarily off) LRU
// @folio: folio to be mlocked.
//
#[no_mangle]
pub unsafe extern "C" fn mlock_folio(folio: *mut folio) {
    void mlock_folio(struct folio *folio)
    {
    struct folio_batch *fbatch;
    local_lock(&mlock_fbatch.lock);
    fbatch = this_cpu_ptr(&mlock_fbatch.fbatch);
    if (!folio_test_set_mlocked(folio)) {
    let mut nr_pages: c_int = folio_nr_pages(folio);
    zone_stat_mod_folio(folio, NR_MLOCK, nr_pages);
    __count_vm_events(UNEVICTABLE_PGMLOCKED, nr_pages);
    }
    folio_get(folio);
    if (!folio_batch_add(fbatch, mlock_lru(folio)) ||
    !folio_may_be_lru_cached(folio) || lru_cache_disabled())
    mlock_folio_batch(fbatch);
    local_unlock(&mlock_fbatch.lock);
    }
//
// mlock_new_folio - mlock a newly allocated folio not yet on LRU
// @folio: folio to be mlocked, either normal or a THP head.
//
#[no_mangle]
pub unsafe extern "C" fn mlock_new_folio(folio: *mut folio) {
    void mlock_new_folio(struct folio *folio)
    {
    struct folio_batch *fbatch;
    let mut nr_pages: c_int = folio_nr_pages(folio);
    local_lock(&mlock_fbatch.lock);
    fbatch = this_cpu_ptr(&mlock_fbatch.fbatch);
    folio_set_mlocked(folio);
    zone_stat_mod_folio(folio, NR_MLOCK, nr_pages);
    __count_vm_events(UNEVICTABLE_PGMLOCKED, nr_pages);
    folio_get(folio);
    if (!folio_batch_add(fbatch, mlock_new(folio)) ||
    !folio_may_be_lru_cached(folio) || lru_cache_disabled())
    mlock_folio_batch(fbatch);
    local_unlock(&mlock_fbatch.lock);
    }
//
// munlock_folio - munlock a folio
// @folio: folio to be munlocked, either normal or a THP head.
//
#[no_mangle]
pub unsafe extern "C" fn munlock_folio(folio: *mut folio) {
    void munlock_folio(struct folio *folio)
    {
    struct folio_batch *fbatch;
    local_lock(&mlock_fbatch.lock);
    fbatch = this_cpu_ptr(&mlock_fbatch.fbatch);
//
// folio_test_clear_mlocked(folio) must be left to __munlock_folio(),
// which will check whether the folio is multiply mlocked.
//
    folio_get(folio);
    if (!folio_batch_add(fbatch, folio) ||
    !folio_may_be_lru_cached(folio) || lru_cache_disabled())
    mlock_folio_batch(fbatch);
    local_unlock(&mlock_fbatch.lock);
    }
    static inline unsigned int folio_mlock_step(struct folio *folio,
    pte_t *pte, unsigned long addr, unsigned long end)
    {
    let mut count: c_uint = (end - addr) >> PAGE_SHIFT;
    let mut ptent: pte_t = ptep_get(pte);
    if (!folio_test_large(folio))
    return 1;
    return folio_pte_batch(folio, pte, ptent, count);
    }
    static inline bool allow_mlock_munlock(struct folio *folio,
    struct vm_area_struct *vma, unsigned long start,
    unsigned long end, unsigned int step)
    {
//
// For unlock, allow munlock large folio which is partially
// mapped to VMA. As it's possible that large folio is
// mlocked and VMA is split later.
//
// During memory pressure, such kind of large folio can
// be split. And the pages are not in VM_LOCKed VMA
// can be reclaimed.
//
    if (!vma_test(vma, VMA_LOCKED_BIT))
    return true;
// folio_within_range() cannot take KSM, but any small folio is OK
    if (!folio_test_large(folio))
    return true;
// folio not in range [start, end), skip mlock
    if (!folio_within_range(folio, vma, start, end))
    return false;
// folio is not fully mapped, skip mlock
    if (step != folio_nr_pages(folio))
    return false;
    return true;
    }
    static int mlock_pte_range(pmd_t *pmd, unsigned long addr,
    unsigned long end, struct mm_walk *walk)
    {
    struct vm_area_struct *vma = walk.vma;
    spinlock_t *ptl;
    pte_t *start_pte, *pte;
    pte_t ptent;
    struct folio *folio;
    let mut step: c_uint = 1;
    let mut start: c_ulong = addr;
    ptl = pmd_trans_huge_lock(pmd, vma);
    if (ptl) {
    if (!pmd_present(*pmd))
    goto out;
    if (is_huge_zero_pmd(*pmd))
    goto out;
    folio = pmd_folio(*pmd);
    if (folio_is_zone_device(folio))
    goto out;
    if (vma_test(vma, VMA_LOCKED_BIT))
    mlock_folio(folio);
    else
    munlock_folio(folio);
    goto out;
    }
    start_pte = pte_offset_map_lock(vma.vm_mm, pmd, addr, &ptl);
    if (!start_pte) {
    walk.action = ACTION_AGAIN;
    return 0;
    }
    for (pte = start_pte; addr != end; pte++, addr += PAGE_SIZE) {
    ptent = ptep_get(pte);
    if (!pte_present(ptent))
    continue;
    folio = vm_normal_folio(vma, addr, ptent);
    if (!folio || folio_is_zone_device(folio))
    continue;
    step = folio_mlock_step(folio, pte, addr, end);
    if (!allow_mlock_munlock(folio, vma, start, end, step))
    goto next_entry;
    if (vma_test(vma, VMA_LOCKED_BIT))
    mlock_folio(folio);
    else
    munlock_folio(folio);
    next_entry:
    pte += step - 1;
    addr += (step - 1) << PAGE_SHIFT;
    }
    pte_unmap(start_pte);
    out:
    spin_unlock(ptl);
    cond_resched();
    return 0;
    }
//
// mlock_vma_pages_range() - mlock any pages already in the range,
// or munlock all pages in the range.
// @vma - vma containing range to be mlock()ed or munlock()ed
// @start - start address in @vma of the range
// @end - end of range in @vma
// @new_vma_flags - the new set of flags for @vma.
//
// Called for mlock(), mlock2() and mlockall(), to set @vma VMA_LOCKED_BIT;
// called for munlock() and munlockall(), to clear VMA_LOCKED_BIT from @vma.
//
    static void mlock_vma_pages_range(struct vm_area_struct *vma,
    unsigned long start, unsigned long end,
    vma_flags_t *new_vma_flags)
    {
    static const struct mm_walk_ops mlock_walk_ops = {
    .pmd_entry = mlock_pte_range,
    .walk_lock = PGWALK_WRLOCK_VERIFY,
    };
//
// There is a slight chance that concurrent page migration,
// or page reclaim finding a page of this now-VMA_LOCKED_BIT vma,
// will call mlock_vma_folio() and raise page's mlock_count:
// double counting, leaving the page unevictable indefinitely.
// Communicate this danger to mlock_vma_folio() with VMA_IO_BIT,
// which is a VMA_SPECIAL_FLAGS flag not allowed on VMA_LOCKED_BIT vmas.
// mmap_lock is held in write mode here, so this weird
// combination should not be visible to other mmap_lock users;
// but WRITE_ONCE so rmap walkers must see VMA_IO_BIT if VMA_LOCKED_BIT.
//
    if (vma_flags_test(new_vma_flags, VMA_LOCKED_BIT))
    vma_flags_set(new_vma_flags, VMA_IO_BIT);
    vma_start_write(vma);
    vma_flags_reset_once(vma, new_vma_flags);
    lru_add_drain();
    walk_page_range_vma(vma, start, end, &mlock_walk_ops, core::ptr::null_mut());
    lru_add_drain();
    if (vma_flags_test(new_vma_flags, VMA_IO_BIT)) {
    vma_flags_clear(new_vma_flags, VMA_IO_BIT);
    vma_flags_reset_once(vma, new_vma_flags);
    }
    }
//
// mlock_fixup  - handle mlock[all]/munlock[all] requests.
//
// Filters out "special" vmas -- VMA_LOCKED_BIT never gets set for these, and
// munlock is a no-op.  However, for some special vmas, we go ahead and
// populate the ptes.
//
// For vmas that pass the filters, merge/split as appropriate.
//
    static int mlock_fixup(struct vma_iterator *vmi, struct vm_area_struct *vma,
    struct vm_area_struct **prev, unsigned long start,
    unsigned long end, vma_flags_t *new_vma_flags)
    {
    let mut old_vma_flags: vma_flags_t = vma.flags;
    struct mm_struct *mm = vma.vm_mm;
    int nr_pages;
    let mut ret: c_int = 0;
    if (vma_flags_same_pair(&old_vma_flags, new_vma_flags) ||
    vma_is_secretmem(vma) || !vma_supports_mlock(vma)) {
//
// Don't set VMA_LOCKED_BIT or VMA_LOCKONFAULT_BIT and don't
// count.  For secretmem, don't allow the memory to be unlocked.
//
    goto out;
    }
    vma = vma_modify_flags(vmi, *prev, vma, start, end, new_vma_flags);
    if (IS_ERR(vma)) {
    ret = PTR_ERR(vma);
    goto out;
    }
//
// Keep track of amount of locked VM.
//
    nr_pages = (end - start) >> PAGE_SHIFT;
    if (!vma_flags_test(new_vma_flags, VMA_LOCKED_BIT))
    nr_pages = -nr_pages;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: vma_flags_test(&old_vma_flags, _arg: VMA_LOCKED_BIT)) -> else {
    else if (vma_flags_test(&old_vma_flags, VMA_LOCKED_BIT))
    nr_pages = 0;
    mm.locked_vm += nr_pages;
//
// vm_flags is protected by the mmap_lock held in write mode.
// It's okay if try_to_unmap_one unmaps a page just after we
// set VMA_LOCKED_BIT, populate_vma_page_range will bring it back.
//
    if (vma_flags_test(new_vma_flags, VMA_LOCKED_BIT) &&
    vma_flags_test(&old_vma_flags, VMA_LOCKED_BIT)) {
// No work to do, and mlocking twice would be wrong
    vma_start_write(vma);
    vma.flags = *new_vma_flags;
    } else {
    mlock_vma_pages_range(vma, start, end, new_vma_flags);
    }
    out:
// prev = vma;
    return ret;
    }
    static int apply_vma_lock_flags(unsigned long start, size_t len,
    const vma_flags_t *flags)
    {
    unsigned long nstart, end, tmp;
    struct vm_area_struct *vma, *prev;
    VMA_ITERATOR(vmi, current.mm, start);
    VM_BUG_ON(offset_in_page(start));
    VM_BUG_ON(len != PAGE_ALIGN(len));
    end = start + len;
    if (end < start)
    return -EINVAL;
    if (end == start)
    return 0;
    vma = vma_iter_load(&vmi);
    if (!vma)
    return -ENOMEM;
    prev = vma_prev(&vmi);
    if (start > vma.vm_start)
    prev = vma;
    nstart = start;
    tmp = vma.vm_start;
    for_each_vma_range(vmi, vma, end) {
    int error;
    vma_flags_t newflags;
    if (vma.vm_start != tmp)
    return -ENOMEM;
    newflags = vma.flags;
    vma_flags_clear_mask(&newflags, VMA_LOCKED_MASK);
    vma_flags_set_mask(&newflags, *flags);
// Here we know that  vma->vm_start <= nstart < vma->vm_end.
    tmp = vma.vm_end;
    if (tmp > end)
    tmp = end;
    error = mlock_fixup(&vmi, vma, &prev, nstart, tmp, &newflags);
    if (error)
    return error;
    tmp = vma_iter_end(&vmi);
    nstart = tmp;
    }
    if (tmp < end)
    return -ENOMEM;
    return 0;
    }
//
// Go through vma areas and sum size of mlocked
// vma pages, as return value.
// Note deferred memory locking case(mlock2(,,MLOCK_ONFAULT)
// is also counted.
// Return value: previously mlocked page counts
//
    static unsigned long count_mm_mlocked_page_nr(struct mm_struct *mm,
    unsigned long start, size_t len)
    {
    struct vm_area_struct *vma;
    let mut count: c_ulong = 0;
    unsigned long end;
    VMA_ITERATOR(vmi, mm, start);
// Don't overflow past ULONG_MAX
    if (unlikely(ULONG_MAX - len < start))
    end = ULONG_MAX;
    else
    end = start + len;
    for_each_vma_range(vmi, vma, end) {
    if (vma_test(vma, VMA_LOCKED_BIT)) {
    if (start > vma.vm_start)
    count -= (start - vma.vm_start);
    if (end < vma.vm_end) {
    count += end - vma.vm_start;
    break;
    }
    count += vma.vm_end - vma.vm_start;
    }
    }
    return count >> PAGE_SHIFT;
    }
//
// convert get_user_pages() return value to posix mlock() error
//
#[no_mangle]
unsafe extern "C" fn __mlock_posix_error_return(retval: c_long) -> c_int {
    static int __mlock_posix_error_return(long retval)
    {
    if (retval == -EFAULT)
    retval = -ENOMEM;
#[no_mangle]
pub unsafe extern "C" fn if(-ENOMEM: retval ==) -> else {
    else if (retval == -ENOMEM)
    retval = -EAGAIN;
    return retval;
    }
    static __must_check int do_mlock(unsigned long start, size_t len,
    vma_flags_t *flags)
    {
    unsigned long locked;
    unsigned long lock_limit;
    let mut error: c_int = -ENOMEM;
    start = untagged_addr(start);
    if (!can_do_mlock())
    return -EPERM;
    len = PAGE_ALIGN(len + (offset_in_page(start)));
    start &= PAGE_MASK;
    lock_limit = rlimit(RLIMIT_MEMLOCK);
    lock_limit >>= PAGE_SHIFT;
    locked = len >> PAGE_SHIFT;
    if (mmap_write_lock_killable(current.mm))
    return -EINTR;
    locked += current.mm.locked_vm;
    if ((locked > lock_limit) && (!capable(CAP_IPC_LOCK))) {
//
// It is possible that the regions requested intersect with
// previously mlocked areas, that part area in "mm->locked_vm"
// should not be counted to new mlock increment count. So check
// and adjust locked count if necessary.
//
    locked -= count_mm_mlocked_page_nr(current.mm,
    start, len);
    }
// check against resource limits
    if ((locked <= lock_limit) || capable(CAP_IPC_LOCK))
    error = apply_vma_lock_flags(start, len, flags);
    mmap_write_unlock(current.mm);
    if (error)
    return error;
    error = __mm_populate(start, len, 0);
    if (error)
    return __mlock_posix_error_return(error);
    return 0;
    }
    SYSCALL_DEFINE2(mlock, unsigned long, start, size_t, len)
    {
    let mut flags: vma_flags_t = mk_vma_flags(VMA_LOCKED_BIT);
    return do_mlock(start, len, &flags);
    }
    SYSCALL_DEFINE3(mlock2, unsigned long, start, size_t, len, int, flags)
    {
    let mut vma_flags: vma_flags_t = mk_vma_flags(VMA_LOCKED_BIT);
    if (flags & ~MLOCK_ONFAULT)
    return -EINVAL;
    if (flags & MLOCK_ONFAULT)
    vma_flags_set(&vma_flags, VMA_LOCKONFAULT_BIT);
    return do_mlock(start, len, &vma_flags);
    }
    SYSCALL_DEFINE2(munlock, unsigned long, start, size_t, len)
    {
    let mut flags: vma_flags_t = EMPTY_VMA_FLAGS;
    int ret;
    start = untagged_addr(start);
    len = PAGE_ALIGN(len + (offset_in_page(start)));
    start &= PAGE_MASK;
    if (mmap_write_lock_killable(current.mm))
    return -EINTR;
    ret = apply_vma_lock_flags(start, len, &flags);
    mmap_write_unlock(current.mm);
    return ret;
    }
//
// Take the MCL_* flags passed into mlockall (or 0 if called from munlockall)
// and translate into the appropriate modifications to mm->def_vma_flags and/or
// the flags for all current VMAs.
//
// There are a couple of subtleties with this.  If mlockall() is called multiple
// times with different flags, the values do not necessarily stack.  If mlockall
// is called once including the MCL_FUTURE flag and then a second time without
// it, VMA_LOCKED_BIT and VMA_LOCKONFAULT_BIT will be cleared from
// mm->def_vma_flags.
//
#[no_mangle]
unsafe extern "C" fn apply_mlockall_flags(flags: c_int) -> c_int {
    static int apply_mlockall_flags(int flags)
    {
    VMA_ITERATOR(vmi, current.mm, 0);
    struct mm_struct *mm = current.mm;
    struct vm_area_struct *vma, *prev = core::ptr::null_mut();
    let mut to_add: vma_flags_t = EMPTY_VMA_FLAGS;
    vma_flags_clear_mask(&mm.def_vma_flags, VMA_LOCKED_MASK);
    if (flags & MCL_FUTURE) {
    vma_flags_set(&mm.def_vma_flags, VMA_LOCKED_BIT);
    if (flags & MCL_ONFAULT)
    vma_flags_set(&mm.def_vma_flags, VMA_LOCKONFAULT_BIT);
    if (!(flags & MCL_CURRENT))
    goto out;
    }
    if (flags & MCL_CURRENT) {
    vma_flags_set(&to_add, VMA_LOCKED_BIT);
    if (flags & MCL_ONFAULT)
    vma_flags_set(&to_add, VMA_LOCKONFAULT_BIT);
    }
    for_each_vma(vmi, vma) {
    int error;
    let mut newflags: vma_flags_t = vma.flags;
    vma_flags_clear_mask(&newflags, VMA_LOCKED_MASK);
    vma_flags_set_mask(&newflags, to_add);
    error = mlock_fixup(&vmi, vma, &prev, vma.vm_start, vma.vm_end,
    &newflags);
// Ignore errors, but prev needs fixing up.
    if (error)
    prev = vma;
    cond_resched();
    }
    out:
    return 0;
    }
    SYSCALL_DEFINE1(mlockall, int, flags)
    {
    unsigned long lock_limit;
    int ret;
    if (!flags || (flags & ~(MCL_CURRENT | MCL_FUTURE | MCL_ONFAULT)) ||
    flags == MCL_ONFAULT)
    return -EINVAL;
    if (!can_do_mlock())
    return -EPERM;
    lock_limit = rlimit(RLIMIT_MEMLOCK);
    lock_limit >>= PAGE_SHIFT;
    if (mmap_write_lock_killable(current.mm))
    return -EINTR;
    ret = -ENOMEM;
    if (!(flags & MCL_CURRENT) || (current.mm.total_vm <= lock_limit) ||
    capable(CAP_IPC_LOCK))
    ret = apply_mlockall_flags(flags);
    mmap_write_unlock(current.mm);
    if (!ret && (flags & MCL_CURRENT))
    mm_populate(0, TASK_SIZE);
    return ret;
    }
    SYSCALL_DEFINE0(munlockall)
    {
    int ret;
    if (mmap_write_lock_killable(current.mm))
    return -EINTR;
    ret = apply_mlockall_flags(0);
    mmap_write_unlock(current.mm);
    return ret;
    }
//
// Objects with different lifetime than processes (SHM_LOCK and SHM_HUGETLB
// shm segments) get accounted against the user_struct instead.
//
    static DEFINE_SPINLOCK(shmlock_user_lock);
#[no_mangle]
pub unsafe extern "C" fn user_shm_lock(size: usize, ucounts: *mut ucounts) -> c_int {
    int user_shm_lock(size_t size, struct ucounts *ucounts)
    {
    unsigned long lock_limit, locked;
    long memlock;
    let mut allowed: c_int = 0;
    locked = (size + PAGE_SIZE - 1) >> PAGE_SHIFT;
    lock_limit = rlimit(RLIMIT_MEMLOCK);
    if (lock_limit != RLIM_INFINITY)
    lock_limit >>= PAGE_SHIFT;
    spin_lock(&shmlock_user_lock);
    memlock = inc_rlimit_ucounts(ucounts, UCOUNT_RLIMIT_MEMLOCK, locked);
    if ((memlock == LONG_MAX || memlock > lock_limit) && !capable(CAP_IPC_LOCK)) {
    dec_rlimit_ucounts(ucounts, UCOUNT_RLIMIT_MEMLOCK, locked);
    goto out;
    }
    if (!get_ucounts(ucounts)) {
    dec_rlimit_ucounts(ucounts, UCOUNT_RLIMIT_MEMLOCK, locked);
    allowed = 0;
    goto out;
    }
    allowed = 1;
    out:
    spin_unlock(&shmlock_user_lock);
    return allowed;
    }
#[no_mangle]
pub unsafe extern "C" fn user_shm_unlock(size: usize, ucounts: *mut ucounts) {
    void user_shm_unlock(size_t size, struct ucounts *ucounts)
    {
    spin_lock(&shmlock_user_lock);
    dec_rlimit_ucounts(ucounts, UCOUNT_RLIMIT_MEMLOCK, (size + PAGE_SIZE - 1) >> PAGE_SHIFT);
    spin_unlock(&shmlock_user_lock);
    put_ucounts(ucounts);
    }

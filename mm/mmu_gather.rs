//! Automatically rewritten from C to Rust
//! Source: mm/mmu_gather.c
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
unsafe extern "C" fn tlb_next_batch(tlb: *mut mmu_gather) -> bool {
    static bool tlb_next_batch(struct mmu_gather *tlb)
    {
    struct mmu_gather_batch *batch;
// Limit batching if we have delayed rmaps pending
    if (tlb.delayed_rmap && tlb.active != &tlb.local)
    return false;
    batch = tlb.active;
    if (batch.next) {
    tlb.active = batch.next;
    return true;
    }
    if (tlb.batch_count == MAX_GATHER_BATCH_COUNT)
    return false;
    batch = (void *)__get_free_page(GFP_NOWAIT);
    if (!batch)
    return false;
    tlb.batch_count++;
    batch.next = core::ptr::null_mut();
    batch.nr   = 0;
    batch.max  = MAX_GATHER_BATCH;
    tlb.active.next = batch;
    tlb.active = batch;
    return true;
    }

#[no_mangle]
unsafe extern "C" fn tlb_flush_rmap_batch(batch: *mut mmu_gather_batch, vma: *mut vm_area_struct) {
    static void tlb_flush_rmap_batch(struct mmu_gather_batch *batch, struct vm_area_struct *vma)
    {
    struct encoded_page **pages = batch.encoded_pages;
    for (int i = 0; i < batch.nr; i++) {
    struct encoded_page *enc = pages[i];
    if (encoded_page_flags(enc) & ENCODED_PAGE_BIT_DELAY_RMAP) {
    struct page *page = encoded_page_ptr(enc);
    let mut nr_pages: c_uint = 1;
    if (unlikely(encoded_page_flags(enc) &
    ENCODED_PAGE_BIT_NR_PAGES_NEXT))
    nr_pages = encoded_nr_pages(pages[++i]);
    folio_remove_rmap_ptes(page_folio(page), page, nr_pages,
    vma);
    }
    }
    }
//
// tlb_flush_rmaps - do pending rmap removals after we have flushed the TLB
// @tlb: the current mmu_gather
// @vma: The memory area from which the pages are being removed.
//
// Note that because of how tlb_next_batch() above works, we will
// never start multiple new batches with pending delayed rmaps, so
// we only need to walk through the current active batch and the
// original local one.
//
#[no_mangle]
pub unsafe extern "C" fn tlb_flush_rmaps(tlb: *mut mmu_gather, vma: *mut vm_area_struct) {
    void tlb_flush_rmaps(struct mmu_gather *tlb, struct vm_area_struct *vma)
    {
    if (!tlb.delayed_rmap)
    return;
    tlb_flush_rmap_batch(&tlb.local, vma);
    if (tlb.active != &tlb.local)
    tlb_flush_rmap_batch(tlb.active, vma);
    tlb.delayed_rmap = 0;
    }

//
// We might end up freeing a lot of pages. Reschedule on a regular
// basis to avoid soft lockups in configurations without full
// preemption enabled. The magic number of 512 folios seems to work.
//
pub const MAX_NR_FOLIOS_PER_FREE: c_int = 512;
#[no_mangle]
unsafe extern "C" fn __tlb_batch_free_encoded_pages(batch: *mut mmu_gather_batch) {
    static void __tlb_batch_free_encoded_pages(struct mmu_gather_batch *batch)
    {
    struct encoded_page **pages = batch.encoded_pages;
    unsigned int nr, nr_pages;
    while (batch.nr) {
    if (!page_poisoning_enabled_static() && !want_init_on_free()) {
    nr = min(MAX_NR_FOLIOS_PER_FREE, batch.nr);
//
// Make sure we cover page + nr_pages, and don't leave
// nr_pages behind when capping the number of entries.
//
    if (unlikely(encoded_page_flags(pages[nr - 1]) &
    ENCODED_PAGE_BIT_NR_PAGES_NEXT))
    nr++;
    } else {
//
// With page poisoning and init_on_free, the time it
// takes to free memory grows proportionally with the
// actual memory size. Therefore, limit based on the
// actual memory size and not the number of involved
// folios.
//
    for (nr = 0, nr_pages = 0;
    nr < batch.nr && nr_pages < MAX_NR_FOLIOS_PER_FREE;
    nr++) {
    if (unlikely(encoded_page_flags(pages[nr]) &
    ENCODED_PAGE_BIT_NR_PAGES_NEXT))
    nr_pages += encoded_nr_pages(pages[++nr]);
    else
    nr_pages++;
    }
    }
    free_pages_and_swap_cache(pages, nr);
    pages += nr;
    batch.nr -= nr;
    cond_resched();
    }
    }
#[no_mangle]
unsafe extern "C" fn tlb_batch_pages_flush(tlb: *mut mmu_gather) {
    static void tlb_batch_pages_flush(struct mmu_gather *tlb)
    {
    struct mmu_gather_batch *batch;
    for (batch = &tlb.local; batch && batch.nr; batch = batch.next)
    __tlb_batch_free_encoded_pages(batch);
    tlb.active = &tlb.local;
    }
#[no_mangle]
unsafe extern "C" fn tlb_batch_list_free(tlb: *mut mmu_gather) {
    static void tlb_batch_list_free(struct mmu_gather *tlb)
    {
    struct mmu_gather_batch *batch, *next;
    for (batch = tlb.local.next; batch; batch = next) {
    next = batch.next;
    free_pages((unsigned long)batch, 0);
    }
    tlb.local.next = core::ptr::null_mut();
    }
    static bool __tlb_remove_folio_pages_size(struct mmu_gather *tlb,
    struct page *page, unsigned int nr_pages, bool delay_rmap,
    int page_size)
    {
    let mut flags: c_int = delay_rmap ? ENCODED_PAGE_BIT_DELAY_RMAP : 0;
    struct mmu_gather_batch *batch;
    VM_BUG_ON(!tlb.end);

    VM_WARN_ON(tlb.page_size != page_size);
    VM_WARN_ON_ONCE(nr_pages != 1 && page_size != PAGE_SIZE);
    VM_WARN_ON_ONCE(page_folio(page) != page_folio(page + nr_pages - 1));

    batch = tlb.active;
//
// Add the page and check if we are full. If so
// force a flush.
//
    if (likely(nr_pages == 1)) {
    batch.encoded_pages[batch.nr++] = encode_page(page, flags);
    } else {
    flags |= ENCODED_PAGE_BIT_NR_PAGES_NEXT;
    batch.encoded_pages[batch.nr++] = encode_page(page, flags);
    batch.encoded_pages[batch.nr++] = encode_nr_pages(nr_pages);
    }
//
// Make sure that we can always add another "page" + "nr_pages",
// requiring two entries instead of only a single one.
//
    if (batch.nr >= batch.max - 1) {
    if (!tlb_next_batch(tlb))
    return true;
    batch = tlb.active;
    }
    VM_BUG_ON_PAGE(batch.nr > batch.max - 1, page);
    return false;
    }
    bool __tlb_remove_folio_pages(struct mmu_gather *tlb, struct page *page,
    unsigned int nr_pages, bool delay_rmap)
    {
    return __tlb_remove_folio_pages_size(tlb, page, nr_pages, delay_rmap,
    PAGE_SIZE);
    }
#[no_mangle]
pub unsafe extern "C" fn __tlb_remove_page_size(tlb: *mut mmu_gather, page: *mut page, page_size: c_int) -> bool {
    bool __tlb_remove_page_size(struct mmu_gather *tlb, struct page *page, int page_size)
    {
    return __tlb_remove_folio_pages_size(tlb, page, 1, false, page_size);
    }

#[no_mangle]
unsafe extern "C" fn __tlb_remove_table_free(batch: *mut mmu_table_batch) {
    static void __tlb_remove_table_free(struct mmu_table_batch *batch)
    {
    int i;
    for (i = 0; i < batch.nr; i++)
    __tlb_remove_table(batch.tables[i]);
    free_page((unsigned long)batch);
    }

//
// Semi RCU freeing of the page directories.
//
// This is needed by some architectures to implement software pagetable walkers.
//
// gup_fast() and other software pagetable walkers do a lockless page-table
// walk and therefore needs some synchronization with the freeing of the page
// directories. The chosen means to accomplish that is by disabling IRQs over
// the walk.
//
// Architectures that use IPIs to flush TLBs will then automagically DTRT,
// since we unlink the page, flush TLBs, free the page. Since the disabling of
// IRQs delays the completion of the TLB flush we can never observe an already
// freed page.
//
// Not all systems IPI every CPU for this purpose:
//
// - Some architectures have HW support for cross-CPU synchronisation of TLB
// flushes, so there's no IPI at all.
//
// - Paravirt guests can do this TLB flushing in the hypervisor, or coordinate
// with the hypervisor to defer flushing on preempted vCPUs.
//
// Such systems need to delay the freeing by some other means, this is that
// means.
//
// What we do is batch the freed directory pages (tables) and RCU free them.
// We use the sched RCU variant, as that guarantees that IRQ/preempt disabling
// holds off grace periods.
//
// However, in order to batch these pages we need to allocate storage, this
// allocation is deep inside the MM code and can thus easily fail on memory
// pressure. To guarantee progress we fall back to single table freeing, see
// the implementation of tlb_remove_table_one().
//
#[no_mangle]
unsafe extern "C" fn tlb_remove_table_smp_sync(arg: *mut c_void) {
    static void tlb_remove_table_smp_sync(void *arg)
    {
// Simply deliver the interrupt
    }
#[no_mangle]
pub unsafe extern "C" fn tlb_remove_table_sync_one() {
    void tlb_remove_table_sync_one(void)
    {
//
// This isn't an RCU grace period and hence the page-tables cannot be
// assumed to be actually RCU-freed.
//
// It is however sufficient for software page-table walkers that rely on
// IRQ disabling.
//
    smp_call_function(tlb_remove_table_smp_sync, core::ptr::null_mut(), 1);
    }
#[no_mangle]
unsafe extern "C" fn tlb_remove_table_rcu(head: *mut rcu_head) {
    static void tlb_remove_table_rcu(struct rcu_head *head)
    {
    __tlb_remove_table_free(container_of(head, struct mmu_table_batch, rcu));
    }
#[no_mangle]
unsafe extern "C" fn tlb_remove_table_free(batch: *mut mmu_table_batch) {
    static void tlb_remove_table_free(struct mmu_table_batch *batch)
    {
    call_rcu(&batch.rcu, tlb_remove_table_rcu);
    }
//
// tlb_remove_table_sync_rcu - synchronize with software page-table walkers
//
// Like tlb_remove_table_sync_one() but uses RCU grace period instead of IPI
// broadcast. Use in slow paths where sleeping is acceptable.
//
// Software/Lockless page-table walkers use local_irq_disable(), which is also
// an RCU read-side critical section. synchronize_rcu() waits for all such
// sections, providing the same guarantee as tlb_remove_table_sync_one() but
// without disrupting all CPUs with IPIs.
//
// Do not use for freeing memory. Use RCU callbacks instead to avoid latency
// spikes.
//
#[no_mangle]
pub unsafe extern "C" fn tlb_remove_table_sync_rcu() {
    void tlb_remove_table_sync_rcu(void)
    {
    synchronize_rcu();
    }

#[no_mangle]
unsafe extern "C" fn tlb_remove_table_free(batch: *mut mmu_table_batch) {
    static void tlb_remove_table_free(struct mmu_table_batch *batch)
    {
    __tlb_remove_table_free(batch);
    }

//
// If we want tlb_remove_table() to imply TLB invalidates.
//
#[no_mangle]
pub unsafe extern "C" fn tlb_table_invalidate(tlb: *mut mmu_gather) {
    static inline void tlb_table_invalidate(struct mmu_gather *tlb)
    {
    if (tlb_needs_table_invalidate()) {
//
// Invalidate page-table caches used by hardware walkers. Then
// we still need to RCU-sched wait while freeing the pages
// because software walkers can still be in-flight.
//
    tlb_flush_mmu_tlbonly(tlb);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn __tlb_remove_table_one_rcu(head: *mut rcu_head) {
    static inline void __tlb_remove_table_one_rcu(struct rcu_head *head)
    {
    struct ptdesc *ptdesc;
    ptdesc = container_of(head, struct ptdesc, pt_rcu_head);
    __tlb_remove_table(ptdesc);
    }
#[no_mangle]
pub unsafe extern "C" fn __tlb_remove_table_one(table: *mut c_void) {
    static inline void __tlb_remove_table_one(void *table)
    {
    struct ptdesc *ptdesc;
    ptdesc = table;
    call_rcu(&ptdesc.pt_rcu_head, __tlb_remove_table_one_rcu);
    }

#[no_mangle]
pub unsafe extern "C" fn __tlb_remove_table_one(table: *mut c_void) {
    static inline void __tlb_remove_table_one(void *table)
    {
    tlb_remove_table_sync_rcu();
    __tlb_remove_table(table);
    }

#[no_mangle]
unsafe extern "C" fn tlb_remove_table_one(table: *mut c_void) {
    static void tlb_remove_table_one(void *table)
    {
    __tlb_remove_table_one(table);
    }
#[no_mangle]
unsafe extern "C" fn tlb_table_flush(tlb: *mut mmu_gather) {
    static void tlb_table_flush(struct mmu_gather *tlb)
    {
    struct mmu_table_batch **batch = &tlb.batch;
    if (*batch) {
    tlb_table_invalidate(tlb);
    tlb_remove_table_free(*batch);
// batch = NULL;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tlb_remove_table(tlb: *mut mmu_gather, table: *mut c_void) {
    void tlb_remove_table(struct mmu_gather *tlb, void *table)
    {
    struct mmu_table_batch **batch = &tlb.batch;
    if (*batch == core::ptr::null_mut()) {
// batch = (struct mmu_table_batch *)__get_free_page(GFP_NOWAIT);
    if (*batch == core::ptr::null_mut()) {
    tlb_table_invalidate(tlb);
    tlb_remove_table_one(table);
    return;
    }
    (*batch).nr = 0;
    }
    (*batch).tables[(*batch).nr++] = table;
    if ((*batch).nr == MAX_TABLE_BATCH)
    tlb_table_flush(tlb);
    }
#[no_mangle]
pub unsafe extern "C" fn tlb_table_init(tlb: *mut mmu_gather) {
    static inline void tlb_table_init(struct mmu_gather *tlb)
    {
    tlb.batch = core::ptr::null_mut();
    }

    static inline void tlb_table_flush(struct mmu_gather *tlb) { }
    static inline void tlb_table_init(struct mmu_gather *tlb) { }

#[no_mangle]
unsafe extern "C" fn tlb_flush_mmu_free(tlb: *mut mmu_gather) {
    static void tlb_flush_mmu_free(struct mmu_gather *tlb)
    {
    tlb_table_flush(tlb);

    tlb_batch_pages_flush(tlb);

    }
#[no_mangle]
pub unsafe extern "C" fn tlb_flush_mmu(tlb: *mut mmu_gather) {
    void tlb_flush_mmu(struct mmu_gather *tlb)
    {
    tlb_flush_mmu_tlbonly(tlb);
    tlb_flush_mmu_free(tlb);
    }
    static void __tlb_gather_mmu(struct mmu_gather *tlb, struct mm_struct *mm,
    bool fullmm)
    {
    tlb.mm = mm;
    tlb.fullmm = fullmm;

    tlb.need_flush_all = 0;
    tlb.local.next = core::ptr::null_mut();
    tlb.local.nr   = 0;
    tlb.local.max  = ARRAY_SIZE(tlb.__pages);
    tlb.active     = &tlb.local;
    tlb.batch_count = 0;

    tlb.delayed_rmap = 0;
    tlb_table_init(tlb);

    tlb.page_size = 0;

    tlb.vma_pfn = 0;
    tlb.fully_unshared_tables = 0;
    __tlb_reset_range(tlb);
    inc_tlb_flush_pending(tlb.mm);
    }
//
// tlb_gather_mmu - initialize an mmu_gather structure for page-table tear-down
// @tlb: the mmu_gather structure to initialize
// @mm: the mm_struct of the target address space
//
// Called to initialize an (on-stack) mmu_gather structure for page-table
// tear-down from @mm.
//
#[no_mangle]
pub unsafe extern "C" fn tlb_gather_mmu(tlb: *mut mmu_gather, mm: *mut mm_struct) {
    void tlb_gather_mmu(struct mmu_gather *tlb, struct mm_struct *mm)
    {
    __tlb_gather_mmu(tlb, mm, false);
    }
//
// tlb_gather_mmu_fullmm - initialize an mmu_gather structure for page-table tear-down
// @tlb: the mmu_gather structure to initialize
// @mm: the mm_struct of the target address space
//
// In this case, @mm is without users and we're going to destroy the
// full address space (exit/execve).
//
// Called to initialize an (on-stack) mmu_gather structure for page-table
// tear-down from @mm.
//
#[no_mangle]
pub unsafe extern "C" fn tlb_gather_mmu_fullmm(tlb: *mut mmu_gather, mm: *mut mm_struct) {
    void tlb_gather_mmu_fullmm(struct mmu_gather *tlb, struct mm_struct *mm)
    {
    __tlb_gather_mmu(tlb, mm, true);
    }
//
// tlb_gather_mmu_vma - initialize an mmu_gather structure for operating on a
// single VMA
// @tlb: the mmu_gather structure to initialize
// @vma: the vm_area_struct
//
// Called to initialize an (on-stack) mmu_gather structure for operating on
// a single VMA. In contrast to tlb_gather_mmu(), calling this function will
// not require another call to tlb_start_vma(). In contrast to tlb_start_vma(),
// this function will *not* call flush_cache_range().
//
// For hugetlb VMAs, this function will also initialize the mmu_gather
// page_size accordingly, not requiring a separate call to
// tlb_change_page_size().
//
#[no_mangle]
pub unsafe extern "C" fn tlb_gather_mmu_vma(tlb: *mut mmu_gather, vma: *mut vm_area_struct) {
    void tlb_gather_mmu_vma(struct mmu_gather *tlb, struct vm_area_struct *vma)
    {
    tlb_gather_mmu(tlb, vma.vm_mm);
    tlb_update_vma_flags(tlb, vma);
    if (is_vm_hugetlb_page(vma))
// All entries have the same size.
    tlb_change_page_size(tlb, huge_page_size(hstate_vma(vma)));
    }
//
// tlb_finish_mmu - finish an mmu_gather structure
// @tlb: the mmu_gather structure to finish
//
// Called at the end of the shootdown operation to free up any resources that
// were required.
//
#[no_mangle]
pub unsafe extern "C" fn tlb_finish_mmu(tlb: *mut mmu_gather) {
    void tlb_finish_mmu(struct mmu_gather *tlb)
    {
//
// We expect an earlier huge_pmd_unshare_flush() call to sort this out,
// due to complicated locking requirements with page table unsharing.
//
    VM_WARN_ON_ONCE(tlb.fully_unshared_tables);
//
// If there are parallel threads are doing PTE changes on same range
// under non-exclusive lock (e.g., mmap_lock read-side) but defer TLB
// flush by batching, one thread may end up seeing inconsistent PTEs
// and result in having stale TLB entries.  So flush TLB forcefully
// if we detect parallel PTE batching threads.
//
// However, some syscalls, e.g. munmap(), may free page tables, this
// needs force flush everything in the given range. Otherwise this
// may result in having stale TLB entries for some architectures,
// e.g. aarch64, that could specify flush what level TLB.
//
    if (mm_tlb_flush_nested(tlb.mm)) {
//
// The aarch64 yields better performance with fullmm by
// avoiding multiple CPUs spamming TLBI messages at the
// same time.
//
// On x86 non-fullmm doesn't yield significant difference
// against fullmm.
//
    tlb.fullmm = 1;
    __tlb_reset_range(tlb);
    tlb.freed_tables = 1;
    }
    tlb_flush_mmu(tlb);

    tlb_batch_list_free(tlb);

    dec_tlb_flush_pending(tlb.mm);
    }

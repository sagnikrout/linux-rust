//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-kcopyd.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2002 Sistina Software (UK) Limited.
// Copyright (C) 2006 Red Hat GmbH
//
// This file is released under the GPL.
//
// Kcopyd provides a simple interface for copying an area of one
// block-device to one or more other block-devices, with an asynchronous
// completion notification.
//

pub const SPLIT_COUNT: c_int = 8;
pub const MIN_JOBS: c_int = 8;
pub const DEFAULT_SUB_JOB_SIZE_KB: c_int = 512;
pub const MAX_SUB_JOB_SIZE_KB: c_int = 1024;
    let mut kcopyd_subjob_size_kb: static unsigned int = DEFAULT_SUB_JOB_SIZE_KB;
    module_param(kcopyd_subjob_size_kb, uint, 0644);
    MODULE_PARM_DESC(kcopyd_subjob_size_kb, "Sub-job size for dm-kcopyd clients");
#[no_mangle]
unsafe extern "C" fn dm_get_kcopyd_subjob_size() -> c_uint {
    static unsigned int dm_get_kcopyd_subjob_size(void)
    {
    unsigned int sub_job_size_kb;
    sub_job_size_kb = __dm_get_module_param(&kcopyd_subjob_size_kb,
    DEFAULT_SUB_JOB_SIZE_KB,
    MAX_SUB_JOB_SIZE_KB);
    return sub_job_size_kb << 1;
    }
//
// ----------------------------------------------------------------
// Each kcopyd client has its own little pool of preallocated
// pages for kcopyd io.
// ---------------------------------------------------------------
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_kcopyd_client {
    pub pages: *mut page_list,
    pub nr_reserved_pages: c_uint,
    pub nr_free_pages: c_uint,
    pub sub_job_size: c_uint,
    pub io_client: *mut dm_io_client,
    pub destroyq: wait_queue_head_t,
    pub job_pool: mempool_t,
    pub kcopyd_wq: *mut workqueue_struct,
    pub kcopyd_work: work_struct,
    pub throttle: *mut dm_kcopyd_throttle,
    pub nr_jobs: core::sync::atomic::AtomicI32,
//
// We maintain four lists of jobs:
//
// i)   jobs waiting for pages
// ii)  jobs that have pages, and are waiting for the io to be issued.
// iii) jobs that don't need to do any IO and just run a callback
// iv) jobs that have completed.
//
// All four of these are protected by job_lock.
//
    pub job_lock: spinlock_t,
    pub callback_jobs: list_head,
    pub complete_jobs: list_head,
    pub io_jobs: list_head,
    pub pages_jobs: list_head,
}

    static struct page_list zero_page_list;
    static DEFINE_SPINLOCK(throttle_spinlock);
//
// IO/IDLE accounting slowly decays after (1 << ACCOUNT_INTERVAL_SHIFT) period.
// When total_period >= (1 << ACCOUNT_INTERVAL_SHIFT) the counters are divided
// by 2.
//

//
// Sleep this number of milliseconds.
//
// The value was decided experimentally.
// Smaller values seem to cause an increased copy rate above the limit.
// The reason for this is unknown but possibly due to jiffies rounding errors
// or read/write cache inside the disk.
//
pub const SLEEP_USEC: c_int = 100000;
//
// Maximum number of sleep events. There is a theoretical livelock if more
// kcopyd clients do work simultaneously which this limit avoids.
//
pub const MAX_SLEEPS: c_int = 10;
#[no_mangle]
unsafe extern "C" fn io_job_start(t: *mut dm_kcopyd_throttle) {
    static void io_job_start(struct dm_kcopyd_throttle *t)
    {
    unsigned int throttle, now, difference;
    let mut slept: c_int = 0, skew;
    if (unlikely(!t))
    return;
    try_again:
    spin_lock_irq(&throttle_spinlock);
    throttle = READ_ONCE(t.throttle);
    if (likely(throttle >= 100))
    goto skip_limit;
    now = jiffies;
    difference = now - t.last_jiffies;
    t.last_jiffies = now;
    if (t.num_io_jobs)
    t.io_period += difference;
    t.total_period += difference;
//
// Maintain sane values if we got a temporary overflow.
//
    if (unlikely(t.io_period > t.total_period))
    t.io_period = t.total_period;
    if (unlikely(t.total_period >= (1 << ACCOUNT_INTERVAL_SHIFT))) {
    let mut shift: c_int = fls(t.total_period >> ACCOUNT_INTERVAL_SHIFT);
    t.total_period >>= shift;
    t.io_period >>= shift;
    }
    skew = t.io_period - throttle * t.total_period / 100;
    if (unlikely(skew > 0) && slept < MAX_SLEEPS) {
    slept++;
    spin_unlock_irq(&throttle_spinlock);
    fsleep(SLEEP_USEC);
    goto try_again;
    }
    skip_limit:
    t.num_io_jobs++;
    spin_unlock_irq(&throttle_spinlock);
    }
#[no_mangle]
unsafe extern "C" fn io_job_finish(t: *mut dm_kcopyd_throttle) {
    static void io_job_finish(struct dm_kcopyd_throttle *t)
    {
    unsigned long flags;
    if (unlikely(!t))
    return;
    spin_lock_irqsave(&throttle_spinlock, flags);
    t.num_io_jobs--;
    if (likely(READ_ONCE(t.throttle) >= 100))
    goto skip_limit;
    if (!t.num_io_jobs) {
    unsigned int now, difference;
    now = jiffies;
    difference = now - t.last_jiffies;
    t.last_jiffies = now;
    t.io_period += difference;
    t.total_period += difference;
//
// Maintain sane values if we got a temporary overflow.
//
    if (unlikely(t.io_period > t.total_period))
    t.io_period = t.total_period;
    }
    skip_limit:
    spin_unlock_irqrestore(&throttle_spinlock, flags);
    }
#[no_mangle]
unsafe extern "C" fn wake(kc: *mut dm_kcopyd_client) {
    static void wake(struct dm_kcopyd_client *kc)
    {
    queue_work(kc.kcopyd_wq, &kc.kcopyd_work);
    }
//
// Obtain one page for the use of kcopyd.
//
    static struct page_list *alloc_pl(gfp_t gfp)
    {
    struct page_list *pl;
    pl = kmalloc_obj(*pl, gfp);
    if (!pl)
    return core::ptr::null_mut();
    pl.page = alloc_page(gfp | __GFP_HIGHMEM);
    if (!pl.page) {
    kfree(pl);
    return core::ptr::null_mut();
    }
    return pl;
    }
#[no_mangle]
unsafe extern "C" fn free_pl(pl: *mut page_list) {
    static void free_pl(struct page_list *pl)
    {
    __free_page(pl.page);
    kfree(pl);
    }
//
// Add the provided pages to a client's free page list, releasing
// back to the system any beyond the reserved_pages limit.
//
#[no_mangle]
unsafe extern "C" fn kcopyd_put_pages(kc: *mut dm_kcopyd_client, pl: *mut page_list) {
    static void kcopyd_put_pages(struct dm_kcopyd_client *kc, struct page_list *pl)
    {
    struct page_list *next;
    do {
    next = pl.next;
    if (kc.nr_free_pages >= kc.nr_reserved_pages)
    free_pl(pl);
    else {
    pl.next = kc.pages;
    kc.pages = pl;
    kc.nr_free_pages++;
    }
    pl = next;
    } while (pl);
    }
    static int kcopyd_get_pages(struct dm_kcopyd_client *kc,
    unsigned int nr, struct page_list **pages)
    {
    struct page_list *pl;
// pages = NULL;
    do {
    pl = alloc_pl(__GFP_NOWARN | __GFP_NORETRY | __GFP_KSWAPD_RECLAIM);
    if (unlikely(!pl)) {
// Use reserved pages
    pl = kc.pages;
    if (unlikely(!pl))
    goto out_of_memory;
    kc.pages = pl.next;
    kc.nr_free_pages--;
    }
    pl.next = *pages;
// pages = pl;
    } while (--nr);
    return 0;
    out_of_memory:
    if (*pages)
    kcopyd_put_pages(kc, *pages);
    return -ENOMEM;
    }
//
// These three functions resize the page pool.
//
#[no_mangle]
unsafe extern "C" fn drop_pages(pl: *mut page_list) {
    static void drop_pages(struct page_list *pl)
    {
    struct page_list *next;
    while (pl) {
    next = pl.next;
    free_pl(pl);
    pl = next;
    }
    }
//
// Allocate and reserve nr_pages for the use of a specific client.
//
#[no_mangle]
unsafe extern "C" fn client_reserve_pages(kc: *mut dm_kcopyd_client, nr_pages: c_uint) -> c_int {
    static int client_reserve_pages(struct dm_kcopyd_client *kc, unsigned int nr_pages)
    {
    unsigned int i;
    struct page_list *pl = core::ptr::null_mut(), *next;
    for (i = 0; i < nr_pages; i++) {
    next = alloc_pl(GFP_KERNEL);
    if (!next) {
    if (pl)
    drop_pages(pl);
    return -ENOMEM;
    }
    next.next = pl;
    pl = next;
    }
    kc.nr_reserved_pages += nr_pages;
    kcopyd_put_pages(kc, pl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn client_free_pages(kc: *mut dm_kcopyd_client) {
    static void client_free_pages(struct dm_kcopyd_client *kc)
    {
    BUG_ON(kc.nr_free_pages != kc.nr_reserved_pages);
    drop_pages(kc.pages);
    kc.pages = core::ptr::null_mut();
    kc.nr_free_pages = kc.nr_reserved_pages = 0;
    }
//
// ---------------------------------------------------------------
// kcopyd_jobs need to be allocated by the *clients* of kcopyd,
// for this reason we use a mempool to prevent the client from
// ever having to do io (which could cause a deadlock).
// ---------------------------------------------------------------
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcopyd_job {
    pub kc: *mut dm_kcopyd_client,
    pub list: list_head,
    pub flags: c_uint,
//
// Error state of the job.
//
    pub read_err: c_int,
    pub write_err: c_ulong,
//
// REQ_OP_READ, REQ_OP_WRITE or REQ_OP_WRITE_ZEROES.
//
    pub op: enum req_op,
    pub source: dm_io_region,
//
// The destinations for the transfer.
//
    pub num_dests: c_uint,
    pub dests: [dm_io_region; DM_KCOPYD_MAX_REGIONS],
    pub pages: *mut page_list,
//
// Set this to ensure you are notified when the job has
// completed.  'context' is for callback to use.
//
    pub fn: dm_kcopyd_notify_fn,
    pub context: *mut c_void,
//
// These fields are only used if the job has been split
// into more manageable parts.
//
    pub lock: mutex,
    pub sub_jobs: core::sync::atomic::AtomicI32,
    pub progress: sector_t,
    pub write_offset: sector_t,
    pub master_job: *mut kcopyd_job,
}

    static struct kmem_cache *_job_cache;
#[no_mangle]
pub unsafe extern "C" fn dm_kcopyd_init() -> int __init {
    int __init dm_kcopyd_init(void)
    {
    _job_cache = kmem_cache_create("kcopyd_job",
    sizeof(struct kcopyd_job) * (SPLIT_COUNT + 1),
    __alignof__(struct kcopyd_job), 0, core::ptr::null_mut());
    if (!_job_cache)
    return -ENOMEM;
    zero_page_list.next = &zero_page_list;
    zero_page_list.page = ZERO_PAGE(0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dm_kcopyd_exit() {
    void dm_kcopyd_exit(void)
    {
    kmem_cache_destroy(_job_cache);
    _job_cache = core::ptr::null_mut();
    }
//
// Functions to push and pop a job onto the head of a given job
// list.
//
    static struct kcopyd_job *pop_io_job(struct list_head *jobs,
    struct dm_kcopyd_client *kc)
    {
    struct kcopyd_job *job;
//
// For I/O jobs, pop any read, any write without sequential write
// constraint and sequential writes that are at the right position.
//
    list_for_each_entry(job, jobs, list) {
    if (job.op == REQ_OP_READ ||
    !(job.flags & BIT(DM_KCOPYD_WRITE_SEQ))) {
    list_del(&job.list);
    return job;
    }
    if (job.write_offset == job.master_job.write_offset) {
    job.master_job.write_offset += job.source.count;
    list_del(&job.list);
    return job;
    }
    }
    return core::ptr::null_mut();
    }
    static struct kcopyd_job *pop(struct list_head *jobs,
    struct dm_kcopyd_client *kc)
    {
    struct kcopyd_job *job = core::ptr::null_mut();
    spin_lock_irq(&kc.job_lock);
    if (!list_empty(jobs)) {
    if (jobs == &kc.io_jobs)
    job = pop_io_job(jobs, kc);
    else {
    job = list_entry(jobs.next, struct kcopyd_job, list);
    list_del(&job.list);
    }
    }
    spin_unlock_irq(&kc.job_lock);
    return job;
    }
#[no_mangle]
unsafe extern "C" fn push(jobs: *mut list_head, job: *mut kcopyd_job) {
    static void push(struct list_head *jobs, struct kcopyd_job *job)
    {
    unsigned long flags;
    struct dm_kcopyd_client *kc = job.kc;
    spin_lock_irqsave(&kc.job_lock, flags);
    list_add_tail(&job.list, jobs);
    spin_unlock_irqrestore(&kc.job_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn push_head(jobs: *mut list_head, job: *mut kcopyd_job) {
    static void push_head(struct list_head *jobs, struct kcopyd_job *job)
    {
    struct dm_kcopyd_client *kc = job.kc;
    spin_lock_irq(&kc.job_lock);
    list_add(&job.list, jobs);
    spin_unlock_irq(&kc.job_lock);
    }
//
// These three functions process 1 item from the corresponding
// job list.
//
// They return:
// < 0: error
// 0: success
// > 0: can't process yet.
//
#[no_mangle]
unsafe extern "C" fn run_complete_job(job: *mut kcopyd_job) -> c_int {
    static int run_complete_job(struct kcopyd_job *job)
    {
    void *context = job.context;
    let mut read_err: c_int = job.read_err;
    let mut write_err: c_ulong = job.write_err;
    let mut fn: dm_kcopyd_notify_fn = job.fn;
    struct dm_kcopyd_client *kc = job.kc;
    if (job.pages && job.pages != &zero_page_list)
    kcopyd_put_pages(kc, job.pages);
//
// If this is the master job, the sub jobs have already
// completed so we can free everything.
//
    if (job.master_job == job) {
    mutex_destroy(&job.lock);
    mempool_free(job, &kc.job_pool);
    }
    fn(read_err, write_err, context);
    if (atomic_dec_and_test(&kc.nr_jobs))
    wake_up(&kc.destroyq);
    cond_resched();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn complete_io(error: c_ulong, unsup: c_ulong, context: *mut c_void) {
    static void complete_io(unsigned long error, unsigned long unsup, void *context)
    {
    struct kcopyd_job *job = context;
    struct dm_kcopyd_client *kc = job.kc;
    io_job_finish(kc.throttle);
    if (unlikely((error | unsup) != 0)) {
    if (op_is_write(job.op))
    job.write_err |= error | unsup;
    else
    job.read_err = 1;
    if (!(job.flags & BIT(DM_KCOPYD_IGNORE_ERROR))) {
    push(&kc.complete_jobs, job);
    wake(kc);
    return;
    }
    }
    if (op_is_write(job.op))
    push(&kc.complete_jobs, job);
    else {
    job.op = REQ_OP_WRITE;
    push(&kc.io_jobs, job);
    }
    wake(kc);
    }
//
// Request io on as many buffer heads as we can currently get for
// a particular job.
//
#[no_mangle]
unsafe extern "C" fn run_io_job(job: *mut kcopyd_job) -> c_int {
    static int run_io_job(struct kcopyd_job *job)
    {
    int r;
    struct dm_io_request io_req = {
    .bi_opf = job.op,
    .mem.type = DM_IO_PAGE_LIST,
    .mem.ptr.pl = job.pages,
    .mem.offset = 0,
    .notify.fn = complete_io,
    .notify.context = job,
    .client = job.kc.io_client,
    };
//
// If we need to write sequentially and some reads or writes failed,
// no point in continuing.
//
    if (job.flags & BIT(DM_KCOPYD_WRITE_SEQ) &&
    job.master_job.write_err) {
    job.write_err = job.master_job.write_err;
    return -EIO;
    }
    io_job_start(job.kc.throttle);
    if (job.op == REQ_OP_READ)
    r = dm_io(&io_req, 1, &job.source, core::ptr::null_mut(), core::ptr::null_mut(), IOPRIO_DEFAULT);
    else
    r = dm_io(&io_req, job.num_dests, job.dests, core::ptr::null_mut(), core::ptr::null_mut(), IOPRIO_DEFAULT);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn run_pages_job(job: *mut kcopyd_job) -> c_int {
    static int run_pages_job(struct kcopyd_job *job)
    {
    int r;
    let mut nr_pages: c_uint = dm_div_up(job.dests[0].count, PAGE_SIZE >> 9);
    r = kcopyd_get_pages(job.kc, nr_pages, &job.pages);
    if (!r) {
// this job is ready for io
    push(&job.kc.io_jobs, job);
    return 0;
    }
    if (r == -ENOMEM)
// can't complete now
    return 1;
    return r;
    }
//
// Run through a list for as long as possible.  Returns the count
// of successful jobs.
//
    static int process_jobs(struct list_head *jobs, struct dm_kcopyd_client *kc,
    int (*fn)(struct kcopyd_job *))
    {
    struct kcopyd_job *job;
    int r, count = 0;
    while ((job = pop(jobs, kc))) {
    r = fn(job);
    if (r < 0) {
// error this rogue job
    if (op_is_write(job.op))
    job.write_err = (unsigned long) -1L;
    else
    job.read_err = 1;
    push(&kc.complete_jobs, job);
    wake(kc);
    break;
    }
    if (r > 0) {
//
// We couldn't service this job ATM, so
// push this job back onto the list.
//
    push_head(jobs, job);
    break;
    }
    count++;
    }
    return count;
    }
//
// kcopyd does this every time it's woken up.
//
#[no_mangle]
unsafe extern "C" fn do_work(work: *mut work_struct) {
    static void do_work(struct work_struct *work)
    {
    struct dm_kcopyd_client *kc = container_of(work,
    struct dm_kcopyd_client, kcopyd_work);
    struct blk_plug plug;
//
// The order that these are called is *very* important.
// complete jobs can free some pages for pages jobs.
// Pages jobs when successful will jump onto the io jobs
// list.  io jobs call wake when they complete and it all
// starts again.
//
    spin_lock_irq(&kc.job_lock);
    list_splice_tail_init(&kc.callback_jobs, &kc.complete_jobs);
    spin_unlock_irq(&kc.job_lock);
    blk_start_plug(&plug);
    process_jobs(&kc.complete_jobs, kc, run_complete_job);
    process_jobs(&kc.pages_jobs, kc, run_pages_job);
    process_jobs(&kc.io_jobs, kc, run_io_job);
    blk_finish_plug(&plug);
    }
//
// If we are copying a small region we just dispatch a single job
// to do the copy, otherwise the io has to be split up into many
// jobs.
//
#[no_mangle]
unsafe extern "C" fn dispatch_job(job: *mut kcopyd_job) {
    static void dispatch_job(struct kcopyd_job *job)
    {
    struct dm_kcopyd_client *kc = job.kc;
    atomic_inc(&kc.nr_jobs);
    if (unlikely(!job.source.count))
    push(&kc.callback_jobs, job);
#[no_mangle]
pub unsafe extern "C" fn if(&zero_page_list: job->pages ==) -> else {
    else if (job.pages == &zero_page_list)
    push(&kc.io_jobs, job);
    else
    push(&kc.pages_jobs, job);
    wake(kc);
    }
    static void segment_complete(int read_err, unsigned long write_err,
    void *context)
    {
// FIXME: tidy this function
    let mut progress: sector_t = 0;
    let mut count: sector_t = 0;
    struct kcopyd_job *sub_job = context;
    struct kcopyd_job *job = sub_job.master_job;
    struct dm_kcopyd_client *kc = job.kc;
    mutex_lock(&job.lock);
// update the error
    if (read_err)
    job.read_err = 1;
    if (write_err)
    job.write_err |= write_err;
//
// Only dispatch more work if there hasn't been an error.
//
    if ((!job.read_err && !job.write_err) ||
    job.flags & BIT(DM_KCOPYD_IGNORE_ERROR)) {
// get the next chunk of work
    progress = job.progress;
    count = job.source.count - progress;
    if (count) {
    if (count > kc.sub_job_size)
    count = kc.sub_job_size;
    job.progress += count;
    }
    }
    mutex_unlock(&job.lock);
    if (count) {
    int i;
// sub_job = *job;
    sub_job.write_offset = progress;
    sub_job.source.sector += progress;
    sub_job.source.count = count;
    for (i = 0; i < job.num_dests; i++) {
    sub_job.dests[i].sector += progress;
    sub_job.dests[i].count = count;
    }
    sub_job.fn = segment_complete;
    sub_job.context = sub_job;
    dispatch_job(sub_job);
    } else if (atomic_dec_and_test(&job.sub_jobs)) {
//
// Queue the completion callback to the kcopyd thread.
//
// Some callers assume that all the completions are called
// from a single thread and don't race with each other.
//
// We must not call the callback directly here because this
// code may not be executing in the thread.
//
    push(&kc.complete_jobs, job);
    wake(kc);
    }
    }
//
// Create some sub jobs to share the work between them.
//
#[no_mangle]
unsafe extern "C" fn split_job(master_job: *mut kcopyd_job) {
    static void split_job(struct kcopyd_job *master_job)
    {
    int i;
    atomic_inc(&master_job.kc.nr_jobs);
    atomic_set(&master_job.sub_jobs, SPLIT_COUNT);
    for (i = 0; i < SPLIT_COUNT; i++) {
    master_job[i + 1].master_job = master_job;
    segment_complete(0, 0u, &master_job[i + 1]);
    }
    }
    void dm_kcopyd_copy(struct dm_kcopyd_client *kc, struct dm_io_region *from,
    unsigned int num_dests, struct dm_io_region *dests,
    unsigned int flags, dm_kcopyd_notify_fn fn, void *context)
    {
    struct kcopyd_job *job;
    int i;
//
// Allocate an array of jobs consisting of one master job
// followed by SPLIT_COUNT sub jobs.
//
    job = mempool_alloc(&kc.job_pool, GFP_NOIO);
    mutex_init(&job.lock);
//
// set up for the read.
//
    job.kc = kc;
    job.flags = flags;
    job.read_err = 0;
    job.write_err = 0;
    job.num_dests = num_dests;
    memcpy(&job.dests, dests, sizeof(*dests) * num_dests);
//
// If one of the destination is a host-managed zoned block device,
// we need to write sequentially. If one of the destination is a
// host-aware device, then leave it to the caller to choose what to do.
//
    if (!(job.flags & BIT(DM_KCOPYD_WRITE_SEQ))) {
    for (i = 0; i < job.num_dests; i++) {
    if (bdev_is_zoned(dests[i].bdev)) {
    job.flags |= BIT(DM_KCOPYD_WRITE_SEQ);
    break;
    }
    }
    }
//
// If we need to write sequentially, errors cannot be ignored.
//
    if (job.flags & BIT(DM_KCOPYD_WRITE_SEQ) &&
    job.flags & BIT(DM_KCOPYD_IGNORE_ERROR))
    job.flags &= ~BIT(DM_KCOPYD_IGNORE_ERROR);
    if (from) {
    job.source = *from;
    job.pages = core::ptr::null_mut();
    job.op = REQ_OP_READ;
    } else {
    memset(&job.source, 0, sizeof(job.source));
    job.source.count = job.dests[0].count;
    job.pages = &zero_page_list;
//
// Use WRITE ZEROES to optimize zeroing if all dests support it.
//
    job.op = REQ_OP_WRITE_ZEROES;
    for (i = 0; i < job.num_dests; i++)
    if (!bdev_write_zeroes_sectors(job.dests[i].bdev)) {
    job.op = REQ_OP_WRITE;
    break;
    }
    }
    job.fn = fn;
    job.context = context;
    job.master_job = job;
    job.write_offset = 0;
    if (job.source.count <= kc.sub_job_size)
    dispatch_job(job);
    else {
    job.progress = 0;
    split_job(job);
    }
    }
    EXPORT_SYMBOL(dm_kcopyd_copy);
    void dm_kcopyd_zero(struct dm_kcopyd_client *kc,
    unsigned int num_dests, struct dm_io_region *dests,
    unsigned int flags, dm_kcopyd_notify_fn fn, void *context)
    {
    dm_kcopyd_copy(kc, core::ptr::null_mut(), num_dests, dests, flags, fn, context);
    }
    EXPORT_SYMBOL(dm_kcopyd_zero);
    void *dm_kcopyd_prepare_callback(struct dm_kcopyd_client *kc,
    dm_kcopyd_notify_fn fn, void *context)
    {
    struct kcopyd_job *job;
    job = mempool_alloc(&kc.job_pool, GFP_NOIO);
    memset(job, 0, sizeof(struct kcopyd_job));
    job.kc = kc;
    job.fn = fn;
    job.context = context;
    job.master_job = job;
    atomic_inc(&kc.nr_jobs);
    return job;
    }
    EXPORT_SYMBOL(dm_kcopyd_prepare_callback);
#[no_mangle]
pub unsafe extern "C" fn dm_kcopyd_do_callback(j: *mut c_void, read_err: c_int, write_err: c_ulong) {
    void dm_kcopyd_do_callback(void *j, int read_err, unsigned long write_err)
    {
    struct kcopyd_job *job = j;
    struct dm_kcopyd_client *kc = job.kc;
    job.read_err = read_err;
    job.write_err = write_err;
    push(&kc.callback_jobs, job);
    wake(kc);
    }
    EXPORT_SYMBOL(dm_kcopyd_do_callback);
//
// Cancels a kcopyd job, eg. someone might be deactivating a
// mirror.
//

#[no_mangle]
pub unsafe extern "C" fn kcopyd_cancel(job: *mut kcopyd_job, block: c_int) -> c_int {
    int kcopyd_cancel(struct kcopyd_job *job, int block)
    {
// FIXME: finish
    return -1;
    }

//
// ---------------------------------------------------------------
// Client setup
// ---------------------------------------------------------------
//
    struct dm_kcopyd_client *dm_kcopyd_client_create(struct dm_kcopyd_throttle *throttle)
    {
    int r;
    unsigned int reserve_pages;
    struct dm_kcopyd_client *kc;
    kc = kzalloc_obj(*kc);
    if (!kc)
    return ERR_PTR(-ENOMEM);
    spin_lock_init(&kc.job_lock);
    INIT_LIST_HEAD(&kc.callback_jobs);
    INIT_LIST_HEAD(&kc.complete_jobs);
    INIT_LIST_HEAD(&kc.io_jobs);
    INIT_LIST_HEAD(&kc.pages_jobs);
    kc.throttle = throttle;
    r = mempool_init_slab_pool(&kc.job_pool, MIN_JOBS, _job_cache);
    if (r)
    goto bad_slab;
    INIT_WORK(&kc.kcopyd_work, do_work);
    kc.kcopyd_wq = alloc_workqueue("kcopyd", WQ_MEM_RECLAIM | WQ_PERCPU,
    0);
    if (!kc.kcopyd_wq) {
    r = -ENOMEM;
    goto bad_workqueue;
    }
    kc.sub_job_size = dm_get_kcopyd_subjob_size();
    reserve_pages = DIV_ROUND_UP(kc.sub_job_size << SECTOR_SHIFT, PAGE_SIZE);
    kc.pages = core::ptr::null_mut();
    kc.nr_reserved_pages = kc.nr_free_pages = 0;
    r = client_reserve_pages(kc, reserve_pages);
    if (r)
    goto bad_client_pages;
    kc.io_client = dm_io_client_create();
    if (IS_ERR(kc.io_client)) {
    r = PTR_ERR(kc.io_client);
    goto bad_io_client;
    }
    init_waitqueue_head(&kc.destroyq);
    atomic_set(&kc.nr_jobs, 0);
    return kc;
    bad_io_client:
    client_free_pages(kc);
    bad_client_pages:
    destroy_workqueue(kc.kcopyd_wq);
    bad_workqueue:
    mempool_exit(&kc.job_pool);
    bad_slab:
    kfree(kc);
    return ERR_PTR(r);
    }
    EXPORT_SYMBOL(dm_kcopyd_client_create);
#[no_mangle]
pub unsafe extern "C" fn dm_kcopyd_client_destroy(kc: *mut dm_kcopyd_client) {
    void dm_kcopyd_client_destroy(struct dm_kcopyd_client *kc)
    {
// Wait for completion of all jobs submitted by this client.
    wait_event(kc.destroyq, !atomic_read(&kc.nr_jobs));
    BUG_ON(!list_empty(&kc.callback_jobs));
    BUG_ON(!list_empty(&kc.complete_jobs));
    BUG_ON(!list_empty(&kc.io_jobs));
    BUG_ON(!list_empty(&kc.pages_jobs));
    destroy_workqueue(kc.kcopyd_wq);
    dm_io_client_destroy(kc.io_client);
    client_free_pages(kc);
    mempool_exit(&kc.job_pool);
    kfree(kc);
    }
    EXPORT_SYMBOL(dm_kcopyd_client_destroy);
#[no_mangle]
pub unsafe extern "C" fn dm_kcopyd_client_flush(kc: *mut dm_kcopyd_client) {
    void dm_kcopyd_client_flush(struct dm_kcopyd_client *kc)
    {
    flush_workqueue(kc.kcopyd_wq);
    }
    EXPORT_SYMBOL(dm_kcopyd_client_flush);

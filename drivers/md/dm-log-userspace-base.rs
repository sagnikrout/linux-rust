//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-log-userspace-base.c
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
// Copyright (C) 2006-2009 Red Hat, Inc.
//
// This file is released under the LGPL.
//

pub const FLUSH_ENTRY_POOL_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_dirty_log_flush_entry {
    pub type: c_int,
    pub region: region_t,
    pub list: list_head,
}

//
// This limit on the number of mark and clear request is, to a degree,
// arbitrary.  However, there is some basis for the choice in the limits
// imposed on the size of data payload by dm-log-userspace-transfer.c:
// dm_consult_userspace().
//
pub const MAX_FLUSH_GROUP_COUNT: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct log_c {
    pub ti: *mut dm_target,
    pub log_dev: *mut dm_dev,
    pub usr_argv_str: *mut c_char,
    pub usr_argc: u32,
    pub region_size: u32,
    pub region_count: region_t,
    pub luid: u64,
    pub uuid: [c_char; DM_UUID_LEN],
//
// Mark and clear requests are held until a flush is issued
// so that we can group, and thereby limit, the amount of
// network traffic between kernel and userspace.  The 'flush_lock'
// is used to protect these lists.
//
    pub flush_lock: spinlock_t,
    pub mark_list: list_head,
    pub clear_list: list_head,
//
// in_sync_hint gets set when doing is_remote_recovering.  It
// represents the first region that needs recovery.  IOW, the
// first zero bit of sync_bits.  This can be useful for to limit
// traffic for calls like is_remote_recovering and get_resync_work,
// but be take care in its use for anything else.
//
    pub in_sync_hint: u64,
//
// Workqueue for flush of clear region requests.
//
    pub dmlog_wq: *mut workqueue_struct,
    pub flush_log_work: delayed_work,
    pub sched_flush: core::sync::atomic::AtomicI32,
//
// Combine userspace flush and mark requests for efficiency.
//
    pub integrated_flush: u32,
    pub flush_entry_pool: mempool_t,
}

    static struct kmem_cache *_flush_entry_cache;
    static int userspace_do_request(struct log_c *lc, const char *uuid,
    int request_type, char *data, size_t data_size,
    char *rdata, size_t *rdata_size)
    {
    int r;
//
// If the server isn't there, -ESRCH is returned,
// and we must keep trying until the server is
// restored.
//
    retry:
    r = dm_consult_userspace(uuid, lc.luid, request_type, data,
    data_size, rdata, rdata_size);
    if (r != -ESRCH)
    return r;
    DMERR(" Userspace log server not found.");
    while (1) {
    set_current_state(TASK_INTERRUPTIBLE);
    schedule_timeout(2*HZ);
    DMWARN("Attempting to contact userspace log server...");
    r = dm_consult_userspace(uuid, lc.luid, DM_ULOG_CTR,
    lc.usr_argv_str,
    strlen(lc.usr_argv_str) + 1,
    core::ptr::null_mut(), core::ptr::null_mut());
    if (!r)
    break;
    }
    DMINFO("Reconnected to userspace log server... DM_ULOG_CTR complete");
    r = dm_consult_userspace(uuid, lc.luid, DM_ULOG_RESUME, core::ptr::null_mut(),
    0, core::ptr::null_mut(), core::ptr::null_mut());
    if (!r)
    goto retry;
    DMERR("Error trying to resume userspace log: %d", r);
    return -ESRCH;
    }
    static int build_constructor_string(struct dm_target *ti,
    unsigned int argc, char **argv,
    char **ctr_str)
    {
    int i, str_size;
    char *str = core::ptr::null_mut();
// ctr_str = NULL;
//
// Determine overall size of the string.
//
    for (i = 0, str_size = 0; i < argc; i++)
    str_size += strlen(argv[i]) + 1; /* +1 for space between args */
    str_size += 20; /* Max number of chars in a printed u64 number */
    str_size++; /* For NUL-terminator */
    str = kzalloc(str_size, GFP_KERNEL);
    if (!str) {
    DMWARN("Unable to allocate memory for constructor string");
    return -ENOMEM;
    }
    str_size = sprintf(str, "%llu", (unsigned long long)ti.len);
    for (i = 0; i < argc; i++)
    str_size += sprintf(str + str_size, " %s", argv[i]);
// ctr_str = str;
    return str_size;
    }
#[no_mangle]
unsafe extern "C" fn do_flush(work: *mut work_struct) {
    static void do_flush(struct work_struct *work)
    {
    int r;
    struct log_c *lc = container_of(work, struct log_c, flush_log_work.work);
    atomic_set(&lc.sched_flush, 0);
    r = userspace_do_request(lc, lc.uuid, DM_ULOG_FLUSH, core::ptr::null_mut(), 0, core::ptr::null_mut(), core::ptr::null_mut());
    if (r)
    dm_table_event(lc.ti.table);
    }
//
// userspace_ctr
//
// argv contains:
// <UUID> [integrated_flush] <other args>
// Where 'other args' are the userspace implementation-specific log
// arguments.
//
// Example:
// <UUID> [integrated_flush] clustered-disk <arg count> <log dev>
// <region_size> [[no]sync]
//
// This module strips off the <UUID> and uses it for identification
// purposes when communicating with userspace about a log.
//
// If integrated_flush is defined, the kernel combines flush
// and mark requests.
//
// The rest of the line, beginning with 'clustered-disk', is passed
// to the userspace ctr function.
//
    static int userspace_ctr(struct dm_dirty_log *log, struct dm_target *ti,
    unsigned int argc, char **argv)
    {
    let mut r: c_int = 0;
    int str_size;
    char *ctr_str = core::ptr::null_mut();
    struct log_c *lc = core::ptr::null_mut();
    uint64_t rdata;
    let mut rdata_size: usize = sizeof(rdata);
    char *devices_rdata = core::ptr::null_mut();
    let mut devices_rdata_size: usize = DM_NAME_LEN;
    if (argc < 3) {
    DMWARN("Too few arguments to userspace dirty log");
    return -EINVAL;
    }
    lc = kzalloc_obj(*lc);
    if (!lc) {
    DMWARN("Unable to allocate userspace log context.");
    return -ENOMEM;
    }
// The ptr value is sufficient for local unique id
    lc.luid = (unsigned long)lc;
    lc.ti = ti;
    if (strlen(argv[0]) > (DM_UUID_LEN - 1)) {
    DMWARN("UUID argument too long.");
    kfree(lc);
    return -EINVAL;
    }
    lc.usr_argc = argc;
    strscpy(lc.uuid, argv[0], sizeof(lc.uuid));
    argc--;
    argv++;
    spin_lock_init(&lc.flush_lock);
    INIT_LIST_HEAD(&lc.mark_list);
    INIT_LIST_HEAD(&lc.clear_list);
    if (!strcasecmp(argv[0], "integrated_flush")) {
    lc.integrated_flush = 1;
    argc--;
    argv++;
    }
    str_size = build_constructor_string(ti, argc, argv, &ctr_str);
    if (str_size < 0) {
    kfree(lc);
    return str_size;
    }
    devices_rdata = kzalloc(devices_rdata_size, GFP_KERNEL);
    if (!devices_rdata) {
    DMERR("Failed to allocate memory for device information");
    r = -ENOMEM;
    goto out;
    }
    r = mempool_init_slab_pool(&lc.flush_entry_pool, FLUSH_ENTRY_POOL_SIZE,
    _flush_entry_cache);
    if (r) {
    DMERR("Failed to create flush_entry_pool");
    goto out;
    }
//
// Send table string and get back any opened device.
//
    r = dm_consult_userspace(lc.uuid, lc.luid, DM_ULOG_CTR,
    ctr_str, str_size,
    devices_rdata, &devices_rdata_size);
    if (r < 0) {
    if (r == -ESRCH)
    DMERR("Userspace log server not found");
    else
    DMERR("Userspace log server failed to create log");
    goto out;
    }
// Since the region size does not change, get it now
    rdata_size = sizeof(rdata);
    r = dm_consult_userspace(lc.uuid, lc.luid, DM_ULOG_GET_REGION_SIZE,
    core::ptr::null_mut(), 0, (char *)&rdata, &rdata_size);
    if (r) {
    DMERR("Failed to get region size of dirty log");
    goto out;
    }
    lc.region_size = (uint32_t)rdata;
    lc.region_count = dm_sector_div_up(ti.len, lc.region_size);
    if (devices_rdata_size) {
    if (devices_rdata[devices_rdata_size - 1] != '\0') {
    DMERR("DM_ULOG_CTR device return string not properly terminated");
    r = -EINVAL;
    goto out;
    }
    r = dm_get_device(ti, devices_rdata,
    dm_table_get_mode(ti.table), &lc.log_dev);
    if (r)
    DMERR("Failed to register %s with device-mapper",
    devices_rdata);
    }
    if (lc.integrated_flush) {
    lc.dmlog_wq = alloc_workqueue("dmlogd",
    WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    if (!lc.dmlog_wq) {
    DMERR("couldn't start dmlogd");
    r = -ENOMEM;
    goto out;
    }
    INIT_DELAYED_WORK(&lc.flush_log_work, do_flush);
    atomic_set(&lc.sched_flush, 0);
    }
    out:
    kfree(devices_rdata);
    if (r) {
    mempool_exit(&lc.flush_entry_pool);
    kfree(lc);
    kfree(ctr_str);
    } else {
    lc.usr_argv_str = ctr_str;
    log.context = lc;
    }
    return r;
    }
#[no_mangle]
unsafe extern "C" fn userspace_dtr(log: *mut dm_dirty_log) {
    static void userspace_dtr(struct dm_dirty_log *log)
    {
    struct log_c *lc = log.context;
    if (lc.integrated_flush) {
// flush workqueue
    if (atomic_read(&lc.sched_flush))
    flush_delayed_work(&lc.flush_log_work);
    destroy_workqueue(lc.dmlog_wq);
    }
    (void) dm_consult_userspace(lc.uuid, lc.luid, DM_ULOG_DTR,
    core::ptr::null_mut(), 0, core::ptr::null_mut(), core::ptr::null_mut());
    if (lc.log_dev)
    dm_put_device(lc.ti, lc.log_dev);
    mempool_exit(&lc.flush_entry_pool);
    kfree(lc.usr_argv_str);
    kfree(lc);
    }
#[no_mangle]
unsafe extern "C" fn userspace_presuspend(log: *mut dm_dirty_log) -> c_int {
    static int userspace_presuspend(struct dm_dirty_log *log)
    {
    int r;
    struct log_c *lc = log.context;
    r = dm_consult_userspace(lc.uuid, lc.luid, DM_ULOG_PRESUSPEND,
    core::ptr::null_mut(), 0, core::ptr::null_mut(), core::ptr::null_mut());
    return r;
    }
#[no_mangle]
unsafe extern "C" fn userspace_postsuspend(log: *mut dm_dirty_log) -> c_int {
    static int userspace_postsuspend(struct dm_dirty_log *log)
    {
    int r;
    struct log_c *lc = log.context;
//
// Run planned flush earlier.
//
    if (lc.integrated_flush && atomic_read(&lc.sched_flush))
    flush_delayed_work(&lc.flush_log_work);
    r = dm_consult_userspace(lc.uuid, lc.luid, DM_ULOG_POSTSUSPEND,
    core::ptr::null_mut(), 0, core::ptr::null_mut(), core::ptr::null_mut());
    return r;
    }
#[no_mangle]
unsafe extern "C" fn userspace_resume(log: *mut dm_dirty_log) -> c_int {
    static int userspace_resume(struct dm_dirty_log *log)
    {
    int r;
    struct log_c *lc = log.context;
    lc.in_sync_hint = 0;
    r = dm_consult_userspace(lc.uuid, lc.luid, DM_ULOG_RESUME,
    core::ptr::null_mut(), 0, core::ptr::null_mut(), core::ptr::null_mut());
    return r;
    }
#[no_mangle]
unsafe extern "C" fn userspace_get_region_size(log: *mut dm_dirty_log) -> u32 {
    static uint32_t userspace_get_region_size(struct dm_dirty_log *log)
    {
    struct log_c *lc = log.context;
    return lc.region_size;
    }
//
// userspace_is_clean
//
// Check whether a region is clean.  If there is any sort of
// failure when consulting the server, we return not clean.
//
// Returns: 1 if clean, 0 otherwise
//
#[no_mangle]
unsafe extern "C" fn userspace_is_clean(log: *mut dm_dirty_log, region: region_t) -> c_int {
    static int userspace_is_clean(struct dm_dirty_log *log, region_t region)
    {
    int r;
    let mut region64: u64 = (uint64_t)region;
    int64_t is_clean;
    size_t rdata_size;
    struct log_c *lc = log.context;
    rdata_size = sizeof(is_clean);
    r = userspace_do_request(lc, lc.uuid, DM_ULOG_IS_CLEAN,
    (char *)&region64, sizeof(region64),
    (char *)&is_clean, &rdata_size);
    return (r) ? 0 : (int)is_clean;
    }
//
// userspace_in_sync
//
// Check if the region is in-sync.  If there is any sort
// of failure when consulting the server, we assume that
// the region is not in sync.
//
// If 'can_block' is set, return immediately
//
// Returns: 1 if in-sync, 0 if not-in-sync, -EWOULDBLOCK
//
    static int userspace_in_sync(struct dm_dirty_log *log, region_t region,
    int can_block)
    {
    int r;
    let mut region64: u64 = region;
    int64_t in_sync;
    size_t rdata_size;
    struct log_c *lc = log.context;
//
// We can never respond directly - even if in_sync_hint is
// set.  This is because another machine could see a device
// failure and mark the region out-of-sync.  If we don't go
// to userspace to ask, we might think the region is in-sync
// and allow a read to pick up data that is stale.  (This is
// very unlikely if a device actually fails; but it is very
// likely if a connection to one device from one machine fails.)
//
// There still might be a problem if the mirror caches the region
// state as in-sync... but then this call would not be made.  So,
// that is a mirror problem.
//
    if (!can_block)
    return -EWOULDBLOCK;
    rdata_size = sizeof(in_sync);
    r = userspace_do_request(lc, lc.uuid, DM_ULOG_IN_SYNC,
    (char *)&region64, sizeof(region64),
    (char *)&in_sync, &rdata_size);
    return (r) ? 0 : (int)in_sync;
    }
#[no_mangle]
unsafe extern "C" fn flush_one_by_one(lc: *mut log_c, flush_list: *mut list_head) -> c_int {
    static int flush_one_by_one(struct log_c *lc, struct list_head *flush_list)
    {
    let mut r: c_int = 0;
    struct dm_dirty_log_flush_entry *fe;
    list_for_each_entry(fe, flush_list, list) {
    r = userspace_do_request(lc, lc.uuid, fe.type,
    (char *)&fe.region,
    sizeof(fe.region),
    core::ptr::null_mut(), core::ptr::null_mut());
    if (r)
    break;
    }
    return r;
    }
    static int flush_by_group(struct log_c *lc, struct list_head *flush_list,
    int flush_with_payload)
    {
    let mut r: c_int = 0;
    int count;
    let mut type: u32 = 0;
    struct dm_dirty_log_flush_entry *fe, *tmp_fe;
    LIST_HEAD(tmp_list);
    uint64_t group[MAX_FLUSH_GROUP_COUNT];
//
// Group process the requests
//
    while (!list_empty(flush_list)) {
    count = 0;
    list_for_each_entry_safe(fe, tmp_fe, flush_list, list) {
    group[count] = fe.region;
    count++;
    list_move(&fe.list, &tmp_list);
    type = fe.type;
    if (count >= MAX_FLUSH_GROUP_COUNT)
    break;
    }
    if (flush_with_payload) {
    r = userspace_do_request(lc, lc.uuid, DM_ULOG_FLUSH,
    (char *)(group),
    count * sizeof(uint64_t),
    core::ptr::null_mut(), core::ptr::null_mut());
//
// Integrated flush failed.
//
    if (r)
    break;
    } else {
    r = userspace_do_request(lc, lc.uuid, type,
    (char *)(group),
    count * sizeof(uint64_t),
    core::ptr::null_mut(), core::ptr::null_mut());
    if (r) {
//
// Group send failed.  Attempt one-by-one.
//
    list_splice_init(&tmp_list, flush_list);
    r = flush_one_by_one(lc, flush_list);
    break;
    }
    }
    }
//
// Must collect flush_entrys that were successfully processed
// as a group so that they will be free'd by the caller.
//
    list_splice_init(&tmp_list, flush_list);
    return r;
    }
//
// userspace_flush
//
// This function is ok to block.
// The flush happens in two stages.  First, it sends all
// clear/mark requests that are on the list.  Then it
// tells the server to commit them.  This gives the
// server a chance to optimise the commit, instead of
// doing it for every request.
//
// Additionally, we could implement another thread that
// sends the requests up to the server - reducing the
// load on flush.  Then the flush would have less in
// the list and be responsible for the finishing commit.
//
// Returns: 0 on success, < 0 on failure
//
#[no_mangle]
unsafe extern "C" fn userspace_flush(log: *mut dm_dirty_log) -> c_int {
    static int userspace_flush(struct dm_dirty_log *log)
    {
    let mut r: c_int = 0;
    unsigned long flags;
    struct log_c *lc = log.context;
    LIST_HEAD(mark_list);
    LIST_HEAD(clear_list);
    int mark_list_is_empty;
    int clear_list_is_empty;
    struct dm_dirty_log_flush_entry *fe, *tmp_fe;
    mempool_t *flush_entry_pool = &lc.flush_entry_pool;
    spin_lock_irqsave(&lc.flush_lock, flags);
    list_splice_init(&lc.mark_list, &mark_list);
    list_splice_init(&lc.clear_list, &clear_list);
    spin_unlock_irqrestore(&lc.flush_lock, flags);
    mark_list_is_empty = list_empty(&mark_list);
    clear_list_is_empty = list_empty(&clear_list);
    if (mark_list_is_empty && clear_list_is_empty)
    return 0;
    r = flush_by_group(lc, &clear_list, 0);
    if (r)
    goto out;
    if (!lc.integrated_flush) {
    r = flush_by_group(lc, &mark_list, 0);
    if (r)
    goto out;
    r = userspace_do_request(lc, lc.uuid, DM_ULOG_FLUSH,
    core::ptr::null_mut(), 0, core::ptr::null_mut(), core::ptr::null_mut());
    goto out;
    }
//
// Send integrated flush request with mark_list as payload.
//
    r = flush_by_group(lc, &mark_list, 1);
    if (r)
    goto out;
    if (mark_list_is_empty && !atomic_read(&lc.sched_flush)) {
//
// When there are only clear region requests,
// we schedule a flush in the future.
//
    queue_delayed_work(lc.dmlog_wq, &lc.flush_log_work, 3 * HZ);
    atomic_set(&lc.sched_flush, 1);
    } else {
//
// Cancel pending flush because we
// have already flushed in mark_region.
//
    cancel_delayed_work(&lc.flush_log_work);
    atomic_set(&lc.sched_flush, 0);
    }
    out:
//
// We can safely remove these entries, even after failure.
// Calling code will receive an error and will know that
// the log facility has failed.
//
    list_for_each_entry_safe(fe, tmp_fe, &mark_list, list) {
    list_del(&fe.list);
    mempool_free(fe, flush_entry_pool);
    }
    list_for_each_entry_safe(fe, tmp_fe, &clear_list, list) {
    list_del(&fe.list);
    mempool_free(fe, flush_entry_pool);
    }
    if (r)
    dm_table_event(lc.ti.table);
    return r;
    }
//
// userspace_mark_region
//
// This function should avoid blocking unless absolutely required.
// (Memory allocation is valid for blocking.)
//
#[no_mangle]
unsafe extern "C" fn userspace_mark_region(log: *mut dm_dirty_log, region: region_t) {
    static void userspace_mark_region(struct dm_dirty_log *log, region_t region)
    {
    unsigned long flags;
    struct log_c *lc = log.context;
    struct dm_dirty_log_flush_entry *fe;
// Wait for an allocation, but _never_ fail
    fe = mempool_alloc(&lc.flush_entry_pool, GFP_NOIO);
    BUG_ON(!fe);
    spin_lock_irqsave(&lc.flush_lock, flags);
    fe.type = DM_ULOG_MARK_REGION;
    fe.region = region;
    list_add(&fe.list, &lc.mark_list);
    spin_unlock_irqrestore(&lc.flush_lock, flags);
    }
//
// userspace_clear_region
//
// This function must not block.
// So, the alloc can't block.  In the worst case, it is ok to
// fail.  It would simply mean we can't clear the region.
// Does nothing to current sync context, but does mean
// the region will be re-sync'ed on a reload of the mirror
// even though it is in-sync.
//
#[no_mangle]
unsafe extern "C" fn userspace_clear_region(log: *mut dm_dirty_log, region: region_t) {
    static void userspace_clear_region(struct dm_dirty_log *log, region_t region)
    {
    unsigned long flags;
    struct log_c *lc = log.context;
    struct dm_dirty_log_flush_entry *fe;
//
// If we fail to allocate, we skip the clearing of
// the region.  This doesn't hurt us in any way, except
// to cause the region to be resync'ed when the
// device is activated next time.
//
    fe = mempool_alloc(&lc.flush_entry_pool, GFP_ATOMIC);
    if (!fe) {
    DMERR("Failed to allocate memory to clear region.");
    return;
    }
    spin_lock_irqsave(&lc.flush_lock, flags);
    fe.type = DM_ULOG_CLEAR_REGION;
    fe.region = region;
    list_add(&fe.list, &lc.clear_list);
    spin_unlock_irqrestore(&lc.flush_lock, flags);
    }
//
// userspace_get_resync_work
//
// Get a region that needs recovery.  It is valid to return
// an error for this function.
//
// Returns: 1 if region filled, 0 if no work, <0 on error
//
#[no_mangle]
unsafe extern "C" fn userspace_get_resync_work(log: *mut dm_dirty_log, region: *mut region_t) -> c_int {
    static int userspace_get_resync_work(struct dm_dirty_log *log, region_t *region)
    {
    int r;
    size_t rdata_size;
    struct log_c *lc = log.context;
    struct {
    int64_t i; /* 64-bit for mix arch compatibility */
    region_t r;
    } pkg;
    if (lc.in_sync_hint >= lc.region_count)
    return 0;
    rdata_size = sizeof(pkg);
    r = userspace_do_request(lc, lc.uuid, DM_ULOG_GET_RESYNC_WORK,
    core::ptr::null_mut(), 0, (char *)&pkg, &rdata_size);
// region = pkg.r;
    return (r) ? r : (int)pkg.i;
    }
//
// userspace_set_region_sync
//
// Set the sync status of a given region.  This function
// must not fail.
//
    static void userspace_set_region_sync(struct dm_dirty_log *log,
    region_t region, int in_sync)
    {
    struct log_c *lc = log.context;
    struct {
    region_t r;
    int64_t i;
    } pkg;
    pkg.r = region;
    pkg.i = (int64_t)in_sync;
    (void) userspace_do_request(lc, lc.uuid, DM_ULOG_SET_REGION_SYNC,
    (char *)&pkg, sizeof(pkg), core::ptr::null_mut(), core::ptr::null_mut());
//
// It would be nice to be able to report failures.
// However, it is easy enough to detect and resolve.
//
    }
//
// userspace_get_sync_count
//
// If there is any sort of failure when consulting the server,
// we assume that the sync count is zero.
//
// Returns: sync count on success, 0 on failure
//
#[no_mangle]
unsafe extern "C" fn userspace_get_sync_count(log: *mut dm_dirty_log) -> region_t {
    static region_t userspace_get_sync_count(struct dm_dirty_log *log)
    {
    int r;
    size_t rdata_size;
    uint64_t sync_count;
    struct log_c *lc = log.context;
    rdata_size = sizeof(sync_count);
    r = userspace_do_request(lc, lc.uuid, DM_ULOG_GET_SYNC_COUNT,
    core::ptr::null_mut(), 0, (char *)&sync_count, &rdata_size);
    if (r)
    return 0;
    if (sync_count >= lc.region_count)
    lc.in_sync_hint = lc.region_count;
    return (region_t)sync_count;
    }
//
// userspace_status
//
// Returns: amount of space consumed
//
    static int userspace_status(struct dm_dirty_log *log, status_type_t status_type,
    char *result, unsigned int maxlen)
    {
    let mut r: c_int = 0;
    char *table_args;
    let mut sz: usize = (size_t)maxlen;
    struct log_c *lc = log.context;
    switch (status_type) {
    case STATUSTYPE_INFO:
    r = userspace_do_request(lc, lc.uuid, DM_ULOG_STATUS_INFO,
    core::ptr::null_mut(), 0, result, &sz);
    if (r) {
    sz = 0;
    DMEMIT("%s 1 COM_FAILURE", log.type.name);
    }
    break;
    case STATUSTYPE_TABLE:
    sz = 0;
    table_args = strchr(lc.usr_argv_str, ' ');
    BUG_ON(!table_args); /* There will always be a ' ' */
    table_args++;
    DMEMIT("%s %u %s ", log.type.name, lc.usr_argc, lc.uuid);
    if (lc.integrated_flush)
    DMEMIT("integrated_flush ");
    DMEMIT("%s ", table_args);
    break;
    case STATUSTYPE_IMA:
// result = '\0';
    break;
    }
    return (r) ? 0 : (int)sz;
    }
//
// userspace_is_remote_recovering
//
// Returns: 1 if region recovering, 0 otherwise
//
    static int userspace_is_remote_recovering(struct dm_dirty_log *log,
    region_t region)
    {
    int r;
    let mut region64: u64 = region;
    struct log_c *lc = log.context;
    static unsigned long limit;
    struct {
    int64_t is_recovering;
    uint64_t in_sync_hint;
    } pkg;
    let mut rdata_size: usize = sizeof(pkg);
//
// Once the mirror has been reported to be in-sync,
// it will never again ask for recovery work.  So,
// we can safely say there is not a remote machine
// recovering if the device is in-sync.  (in_sync_hint
// must be reset at resume time.)
//
    if (region < lc.in_sync_hint)
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: time_after(limit, _arg: jiffies)) -> else {
    else if (time_after(limit, jiffies))
    return 1;
    limit = jiffies + (HZ / 4);
    r = userspace_do_request(lc, lc.uuid, DM_ULOG_IS_REMOTE_RECOVERING,
    (char *)&region64, sizeof(region64),
    (char *)&pkg, &rdata_size);
    if (r)
    return 1;
    lc.in_sync_hint = pkg.in_sync_hint;
    return (int)pkg.is_recovering;
    }
    static struct dm_dirty_log_type _userspace_type = {
    .name = "userspace",
    .module = THIS_MODULE,
    .ctr = userspace_ctr,
    .dtr = userspace_dtr,
    .presuspend = userspace_presuspend,
    .postsuspend = userspace_postsuspend,
    .resume = userspace_resume,
    .get_region_size = userspace_get_region_size,
    .is_clean = userspace_is_clean,
    .in_sync = userspace_in_sync,
    .flush = userspace_flush,
    .mark_region = userspace_mark_region,
    .clear_region = userspace_clear_region,
    .get_resync_work = userspace_get_resync_work,
    .set_region_sync = userspace_set_region_sync,
    .get_sync_count = userspace_get_sync_count,
    .status = userspace_status,
    .is_remote_recovering = userspace_is_remote_recovering,
    };
#[no_mangle]
unsafe extern "C" fn userspace_dirty_log_init() -> int __init {
    static int __init userspace_dirty_log_init(void)
    {
    let mut r: c_int = 0;
    _flush_entry_cache = KMEM_CACHE(dm_dirty_log_flush_entry, 0);
    if (!_flush_entry_cache) {
    DMWARN("Unable to create flush_entry_cache: No memory.");
    return -ENOMEM;
    }
    r = dm_ulog_tfr_init();
    if (r) {
    DMWARN("Unable to initialize userspace log communications");
    kmem_cache_destroy(_flush_entry_cache);
    return r;
    }
    r = dm_dirty_log_type_register(&_userspace_type);
    if (r) {
    DMWARN("Couldn't register userspace dirty log type");
    dm_ulog_tfr_exit();
    kmem_cache_destroy(_flush_entry_cache);
    return r;
    }
    DMINFO("version " DM_LOG_USERSPACE_VSN " loaded");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn userspace_dirty_log_exit() -> void __exit {
    static void __exit userspace_dirty_log_exit(void)
    {
    dm_dirty_log_type_unregister(&_userspace_type);
    dm_ulog_tfr_exit();
    kmem_cache_destroy(_flush_entry_cache);
    DMINFO("version " DM_LOG_USERSPACE_VSN " unloaded");
    }
    module_init(userspace_dirty_log_init);
    module_exit(userspace_dirty_log_exit);
    MODULE_DESCRIPTION(DM_NAME " userspace dirty log link");
    MODULE_AUTHOR("Jonathan Brassow <dm-devel@lists.linux.dev>");
    MODULE_LICENSE("GPL");

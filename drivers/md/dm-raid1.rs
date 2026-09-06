//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-raid1.c
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
// Copyright (C) 2003 Sistina Software Limited.
// Copyright (C) 2005-2008 Red Hat, Inc. All rights reserved.
//
// This file is released under the GPL.
//

    static struct workqueue_struct *dm_raid1_wq;

pub const DM_RAID1_HANDLE_ERRORS: c_uint = 0x01;
pub const DM_RAID1_KEEP_LOG: c_uint = 0x02;

    static DECLARE_WAIT_QUEUE_HEAD(_kmirrord_recovery_stopped);
//
// ---------------------------------------------------------------
// Mirror set structures.
// ---------------------------------------------------------------
//
    enum dm_raid1_error {
    DM_RAID1_WRITE_ERROR,
    DM_RAID1_FLUSH_ERROR,
    DM_RAID1_SYNC_ERROR,
    DM_RAID1_READ_ERROR
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mirror {
    pub ms: *mut mirror_set,
    pub error_count: core::sync::atomic::AtomicI32,
    pub error_type: c_ulong,
    pub dev: *mut dm_dev,
    pub offset: sector_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mirror_set {
    pub ti: *mut dm_target,
    pub list: list_head,
    pub features: u64,
    pub /: *mut *mut spinlock_t lock; / protects the lists,
    pub reads: bio_list,
    pub writes: bio_list,
    pub failures: bio_list,
    pub /: *mut *mut bio_list holds; / bios are waiting until suspend,
    pub rh: *mut dm_region_hash,
    pub kcopyd_client: *mut dm_kcopyd_client,
    pub io_client: *mut dm_io_client,
// recovery
    pub nr_regions: region_t,
    pub in_sync: c_int,
    pub log_failure: c_int,
    pub leg_failure: c_int,
    pub suspend: core::sync::atomic::AtomicI32,
    pub /: *mut *mut atomic_t default_mirror; / Default mirror,
    pub kmirrord_wq: *mut workqueue_struct,
    pub kmirrord_work: work_struct,
    pub timer: timer_list,
    pub timer_pending: c_ulong,
    pub trigger_event: work_struct,
    pub nr_mirrors: c_uint,
    pub mirror: [mirror; ],
}

    DECLARE_DM_KCOPYD_THROTTLE_WITH_MODULE_PARM(raid1_resync_throttle,
    "A percentage of time allocated for raid resynchronization");
#[no_mangle]
unsafe extern "C" fn wakeup_mirrord(context: *mut c_void) {
    static void wakeup_mirrord(void *context)
    {
    struct mirror_set *ms = context;
    queue_work(ms.kmirrord_wq, &ms.kmirrord_work);
    }
#[no_mangle]
unsafe extern "C" fn delayed_wake_fn(t: *mut timer_list) {
    static void delayed_wake_fn(struct timer_list *t)
    {
    struct mirror_set *ms = timer_container_of(ms, t, timer);
    clear_bit(0, &ms.timer_pending);
    wakeup_mirrord(ms);
    }
#[no_mangle]
unsafe extern "C" fn delayed_wake(ms: *mut mirror_set) {
    static void delayed_wake(struct mirror_set *ms)
    {
    if (test_and_set_bit(0, &ms.timer_pending))
    return;
    ms.timer.expires = jiffies + HZ / 5;
    add_timer(&ms.timer);
    }
#[no_mangle]
unsafe extern "C" fn wakeup_all_recovery_waiters(context: *mut c_void) {
    static void wakeup_all_recovery_waiters(void *context)
    {
    wake_up_all(&_kmirrord_recovery_stopped);
    }
#[no_mangle]
unsafe extern "C" fn queue_bio(ms: *mut mirror_set, bio: *mut bio, rw: c_int) {
    static void queue_bio(struct mirror_set *ms, struct bio *bio, int rw)
    {
    unsigned long flags;
    let mut should_wake: c_int = 0;
    struct bio_list *bl;
    bl = (rw == WRITE) ? &ms.writes : &ms.reads;
    spin_lock_irqsave(&ms.lock, flags);
    should_wake = !(bl.head);
    bio_list_add(bl, bio);
    if (should_wake)
    wakeup_mirrord(ms);
    spin_unlock_irqrestore(&ms.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn dispatch_bios(context: *mut c_void, bio_list: *mut bio_list) {
    static void dispatch_bios(void *context, struct bio_list *bio_list)
    {
    struct mirror_set *ms = context;
    struct bio *bio;
    while ((bio = bio_list_pop(bio_list)))
    queue_bio(ms, bio, WRITE);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_raid1_bio_record {
    pub m: *mut mirror,
// if details->bi_bdev == NULL, details were not saved
    pub details: dm_bio_details,
    pub write_region: region_t,
}

//
// Every mirror should look like this one.
//
pub const DEFAULT_MIRROR: c_int = 0;
//
// This is yucky.  We squirrel the mirror struct away inside
// bi_next for read/write buffers.  This is safe since the bh
// doesn't get submitted to the lower levels of block layer.
//
    static struct mirror *bio_get_m(struct bio *bio)
    {
    return (struct mirror *) bio.bi_next;
    }
#[no_mangle]
unsafe extern "C" fn bio_set_m(bio: *mut bio, m: *mut mirror) {
    static void bio_set_m(struct bio *bio, struct mirror *m)
    {
    bio.bi_next = (struct bio *) m;
    }
    static struct mirror *get_default_mirror(struct mirror_set *ms)
    {
    return &ms.mirror[atomic_read(&ms.default_mirror)];
    }
#[no_mangle]
unsafe extern "C" fn set_default_mirror(m: *mut mirror) {
    static void set_default_mirror(struct mirror *m)
    {
    struct mirror_set *ms = m.ms;
    struct mirror *m0 = &(ms.mirror[0]);
    atomic_set(&ms.default_mirror, m - m0);
    }
    static struct mirror *get_valid_mirror(struct mirror_set *ms)
    {
    struct mirror *m;
    for (m = ms.mirror; m < ms.mirror + ms.nr_mirrors; m++)
    if (!atomic_read(&m.error_count))
    return m;
    return core::ptr::null_mut();
    }
// fail_mirror
// @m: mirror device to fail
// @error_type: one of the enum's, DM_RAID1_*_ERROR
//
// If errors are being handled, record the type of
// error encountered for this device.  If this type
// of error has already been recorded, we can return;
// otherwise, we must signal userspace by triggering
// an event.  Additionally, if the device is the
// primary device, we must choose a new primary, but
// only if the mirror is in-sync.
//
// This function must not block.
//
#[no_mangle]
unsafe extern "C" fn fail_mirror(m: *mut mirror, error_type: enum dm_raid1_error) {
    static void fail_mirror(struct mirror *m, enum dm_raid1_error error_type)
    {
    struct mirror_set *ms = m.ms;
    struct mirror *new;
    ms.leg_failure = 1;
//
// error_count is used for nothing more than a
// simple way to tell if a device has encountered
// errors.
//
    atomic_inc(&m.error_count);
    if (test_and_set_bit(error_type, &m.error_type))
    return;
    if (!errors_handled(ms))
    return;
    if (m != get_default_mirror(ms))
    goto out;
    if (!ms.in_sync && !keep_log(ms)) {
//
// Better to issue requests to same failing device
// than to risk returning corrupt data.
//
    DMERR("Primary mirror (%s) failed while out-of-sync: Reads may fail.",
    m.dev.name);
    goto out;
    }
    new = get_valid_mirror(ms);
    if (new)
    set_default_mirror(new);
    else
    DMWARN("All sides of mirror have failed.");
    out:
    queue_work(dm_raid1_wq, &ms.trigger_event);
    }
#[no_mangle]
unsafe extern "C" fn mirror_flush(ti: *mut dm_target) -> c_int {
    static int mirror_flush(struct dm_target *ti)
    {
    struct mirror_set *ms = ti.private;
    unsigned long error_bits, unsup_bits;
    unsigned int i;
    struct dm_io_region io[MAX_NR_MIRRORS];
    struct mirror *m;
    struct dm_io_request io_req = {
    .bi_opf = REQ_OP_WRITE | REQ_PREFLUSH | REQ_SYNC,
    .mem.type = DM_IO_KMEM,
    .mem.ptr.addr = core::ptr::null_mut(),
    .client = ms.io_client,
    };
    for (i = 0, m = ms.mirror; i < ms.nr_mirrors; i++, m++) {
    io[i].bdev = m.dev.bdev;
    io[i].sector = 0;
    io[i].count = 0;
    }
    error_bits = -1;
    dm_io(&io_req, ms.nr_mirrors, io, &error_bits, &unsup_bits, IOPRIO_DEFAULT);
    if (unlikely((error_bits | unsup_bits) != 0)) {
    for (i = 0; i < ms.nr_mirrors; i++)
    if (test_bit(i, &error_bits))
    fail_mirror(ms.mirror + i,
    DM_RAID1_FLUSH_ERROR);
    return -EIO;
    }
    return 0;
    }
//
// ---------------------------------------------------------------
// Recovery.
//
// When a mirror is first activated we may find that some regions
// are in the no-sync state.  We have to recover these by
// recopying from the default mirror to all the others.
// ---------------------------------------------------------------
//
    static void recovery_complete(int read_err, unsigned long write_err,
    void *context)
    {
    struct dm_region *reg = context;
    struct mirror_set *ms = dm_rh_region_context(reg);
    int m, bit = 0;
    if (read_err) {
// Read error means the failure of default mirror.
    DMERR_LIMIT("Unable to read primary mirror during recovery");
    fail_mirror(get_default_mirror(ms), DM_RAID1_SYNC_ERROR);
    }
    if (write_err) {
    DMERR_LIMIT("Write error during recovery (error = 0x%lx)",
    write_err);
//
// Bits correspond to devices (excluding default mirror).
// The default mirror cannot change during recovery.
//
    for (m = 0; m < ms.nr_mirrors; m++) {
    if (&ms.mirror[m] == get_default_mirror(ms))
    continue;
    if (test_bit(bit, &write_err))
    fail_mirror(ms.mirror + m,
    DM_RAID1_SYNC_ERROR);
    bit++;
    }
    }
    dm_rh_recovery_end(reg, !(read_err || write_err));
    }
#[no_mangle]
unsafe extern "C" fn recover(ms: *mut mirror_set, reg: *mut dm_region) {
    static void recover(struct mirror_set *ms, struct dm_region *reg)
    {
    unsigned int i;
    struct dm_io_region from, to[DM_KCOPYD_MAX_REGIONS], *dest;
    struct mirror *m;
    let mut flags: c_ulong = 0;
    let mut key: region_t = dm_rh_get_region_key(reg);
    let mut region_size: sector_t = dm_rh_get_region_size(ms.rh);
// fill in the source
    m = get_default_mirror(ms);
    from.bdev = m.dev.bdev;
    from.sector = m.offset + dm_rh_region_to_sector(ms.rh, key);
    if (key == (ms.nr_regions - 1)) {
//
// The final region may be smaller than
// region_size.
//
    from.count = ms.ti.len & (region_size - 1);
    if (!from.count)
    from.count = region_size;
    } else
    from.count = region_size;
// fill in the destinations
    for (i = 0, dest = to; i < ms.nr_mirrors; i++) {
    if (&ms.mirror[i] == get_default_mirror(ms))
    continue;
    m = ms.mirror + i;
    dest.bdev = m.dev.bdev;
    dest.sector = m.offset + dm_rh_region_to_sector(ms.rh, key);
    dest.count = from.count;
    dest++;
    }
// hand to kcopyd
    if (!errors_handled(ms))
    flags |= BIT(DM_KCOPYD_IGNORE_ERROR);
    dm_kcopyd_copy(ms.kcopyd_client, &from, ms.nr_mirrors - 1, to,
    flags, recovery_complete, reg);
    }
#[no_mangle]
unsafe extern "C" fn reset_ms_flags(ms: *mut mirror_set) {
    static void reset_ms_flags(struct mirror_set *ms)
    {
    unsigned int m;
    ms.leg_failure = 0;
    for (m = 0; m < ms.nr_mirrors; m++) {
    atomic_set(&(ms.mirror[m].error_count), 0);
    ms.mirror[m].error_type = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn do_recovery(ms: *mut mirror_set) {
    static void do_recovery(struct mirror_set *ms)
    {
    struct dm_region *reg;
    struct dm_dirty_log *log = dm_rh_dirty_log(ms.rh);
//
// Start quiescing some regions.
//
    dm_rh_recovery_prepare(ms.rh);
//
// Copy any already quiesced regions.
//
    while ((reg = dm_rh_recovery_start(ms.rh)))
    recover(ms, reg);
//
// Update the in sync flag.
//
    if (!ms.in_sync &&
    (log.type.get_sync_count(log) == ms.nr_regions)) {
// the sync is complete
    dm_table_event(ms.ti.table);
    ms.in_sync = 1;
    reset_ms_flags(ms);
    }
    }
//
// ---------------------------------------------------------------
// Reads
// ---------------------------------------------------------------
//
    static struct mirror *choose_mirror(struct mirror_set *ms, sector_t sector)
    {
    struct mirror *m = get_default_mirror(ms);
    do {
    if (likely(!atomic_read(&m.error_count)))
    return m;
    if (m-- == ms.mirror)
    m += ms.nr_mirrors;
    } while (m != get_default_mirror(ms));
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn default_ok(m: *mut mirror) -> c_int {
    static int default_ok(struct mirror *m)
    {
    struct mirror *default_mirror = get_default_mirror(m.ms);
    return !atomic_read(&default_mirror.error_count);
    }
#[no_mangle]
unsafe extern "C" fn mirror_available(ms: *mut mirror_set, bio: *mut bio) -> c_int {
    static int mirror_available(struct mirror_set *ms, struct bio *bio)
    {
    struct dm_dirty_log *log = dm_rh_dirty_log(ms.rh);
    let mut region: region_t = dm_rh_bio_to_region(ms.rh, bio);
    if (log.type.in_sync(log, region, 0))
    return choose_mirror(ms,  bio.bi_iter.bi_sector) ? 1 : 0;
    return 0;
    }
//
// remap a buffer to a particular mirror.
//
#[no_mangle]
unsafe extern "C" fn map_sector(m: *mut mirror, bio: *mut bio) -> sector_t {
    static sector_t map_sector(struct mirror *m, struct bio *bio)
    {
    if (unlikely(!bio.bi_iter.bi_size))
    return 0;
    return m.offset + dm_target_offset(m.ms.ti, bio.bi_iter.bi_sector);
    }
#[no_mangle]
unsafe extern "C" fn map_bio(m: *mut mirror, bio: *mut bio) {
    static void map_bio(struct mirror *m, struct bio *bio)
    {
    bio_set_dev(bio, m.dev.bdev);
    bio.bi_iter.bi_sector = map_sector(m, bio);
    }
    static void map_region(struct dm_io_region *io, struct mirror *m,
    struct bio *bio)
    {
    io.bdev = m.dev.bdev;
    io.sector = map_sector(m, bio);
    io.count = bio_sectors(bio);
    }
#[no_mangle]
unsafe extern "C" fn hold_bio(ms: *mut mirror_set, bio: *mut bio) {
    static void hold_bio(struct mirror_set *ms, struct bio *bio)
    {
//
// Lock is required to avoid race condition during suspend
// process.
//
    spin_lock_irq(&ms.lock);
    if (atomic_read(&ms.suspend)) {
    spin_unlock_irq(&ms.lock);
//
// If device is suspended, complete the bio.
//
    if (dm_noflush_suspending(ms.ti))
    bio.bi_status = BLK_STS_DM_REQUEUE;
    else
    bio.bi_status = BLK_STS_IOERR;
    bio_endio(bio);
    return;
    }
//
// Hold bio until the suspend is complete.
//
    bio_list_add(&ms.holds, bio);
    spin_unlock_irq(&ms.lock);
    }
//
// ---------------------------------------------------------------
// Reads
// ---------------------------------------------------------------
//
#[no_mangle]
unsafe extern "C" fn read_callback(error: c_ulong, unsup: c_ulong, context: *mut c_void) {
    static void read_callback(unsigned long error, unsigned long unsup, void *context)
    {
    struct bio *bio = context;
    struct mirror *m;
    m = bio_get_m(bio);
    bio_set_m(bio, core::ptr::null_mut());
    if (likely(!error)) {
    if (unlikely(unsup != 0))
    bio.bi_status = BLK_STS_INVAL;
    bio_endio(bio);
    return;
    }
    fail_mirror(m, DM_RAID1_READ_ERROR);
    if (likely(default_ok(m)) || mirror_available(m.ms, bio)) {
    DMWARN_LIMIT("Read failure on mirror device %s. Trying alternative device.",
    m.dev.name);
    queue_bio(m.ms, bio, bio_data_dir(bio));
    return;
    }
    DMERR_LIMIT("Read failure on mirror device %s.  Failing I/O.",
    m.dev.name);
    bio_io_error(bio);
    }
// Asynchronous read.
#[no_mangle]
unsafe extern "C" fn read_async_bio(m: *mut mirror, bio: *mut bio) {
    static void read_async_bio(struct mirror *m, struct bio *bio)
    {
    struct dm_io_region io;
    struct dm_io_request io_req = {
    .bi_opf = REQ_OP_READ,
    .mem.type = DM_IO_BIO,
    .mem.ptr.bio = bio,
    .notify.fn = read_callback,
    .notify.context = bio,
    .client = m.ms.io_client,
    };
    map_region(&io, m, bio);
    bio_set_m(bio, m);
    BUG_ON(dm_io(&io_req, 1, &io, core::ptr::null_mut(), core::ptr::null_mut(), IOPRIO_DEFAULT));
    }
    static inline int region_in_sync(struct mirror_set *ms, region_t region,
    int may_block)
    {
    let mut state: c_int = dm_rh_get_state(ms.rh, region, may_block);
    let mut state: return = = DM_RH_CLEAN || state == DM_RH_DIRTY;
    }
#[no_mangle]
unsafe extern "C" fn do_reads(ms: *mut mirror_set, reads: *mut bio_list) {
    static void do_reads(struct mirror_set *ms, struct bio_list *reads)
    {
    region_t region;
    struct bio *bio;
    struct mirror *m;
    while ((bio = bio_list_pop(reads))) {
    region = dm_rh_bio_to_region(ms.rh, bio);
    m = get_default_mirror(ms);
//
// We can only read balance if the region is in sync.
//
    if (likely(region_in_sync(ms, region, 1)))
    m = choose_mirror(ms, bio.bi_iter.bi_sector);
#[no_mangle]
pub unsafe extern "C" fn if(atomic_read(&m->error_count): m &&) -> else {
    else if (m && atomic_read(&m.error_count))
    m = core::ptr::null_mut();
    if (likely(m))
    read_async_bio(m, bio);
    else
    bio_io_error(bio);
    }
    }
//
// ---------------------------------------------------------------------
// Writes.
//
// We do different things with the write io depending on the
// state of the region that it's in:
//
// SYNC:	increment pending, use kcopyd to write to *all* mirrors
// RECOVERING:	delay the io until recovery completes
// NOSYNC:	increment pending, just write to the default mirror
// ---------------------------------------------------------------------
//
#[no_mangle]
unsafe extern "C" fn write_callback(error: c_ulong, unsup: c_ulong, context: *mut c_void) {
    static void write_callback(unsigned long error, unsigned long unsup, void *context)
    {
    unsigned int i;
    struct bio *bio = context;
    struct mirror_set *ms;
    let mut should_wake: c_int = 0;
    unsigned long flags;
    ms = bio_get_m(bio).ms;
    bio_set_m(bio, core::ptr::null_mut());
//
// NOTE: We don't decrement the pending count here,
// instead it is done by the targets endio function.
// This way we handle both writes to SYNC and NOSYNC
// regions with the same code.
//
    if (likely(!(error | unsup))) {
    bio_endio(bio);
    return;
    }
//
// If the bio is discard, return an error, but do not
// degrade the array.
//
    if (bio_op(bio) == REQ_OP_DISCARD) {
    bio.bi_status = BLK_STS_NOTSUPP;
    bio_endio(bio);
    return;
    }
    if (!error && unsup) {
    bio.bi_status = BLK_STS_INVAL;
    bio_endio(bio);
    return;
    }
    for (i = 0; i < ms.nr_mirrors; i++)
    if (test_bit(i, &error))
    fail_mirror(ms.mirror + i, DM_RAID1_WRITE_ERROR);
//
// Need to raise event.  Since raising
// events can block, we need to do it in
// the main thread.
//
    spin_lock_irqsave(&ms.lock, flags);
    if (!ms.failures.head)
    should_wake = 1;
    bio_list_add(&ms.failures, bio);
    if (should_wake)
    wakeup_mirrord(ms);
    spin_unlock_irqrestore(&ms.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn do_write(ms: *mut mirror_set, bio: *mut bio) {
    static void do_write(struct mirror_set *ms, struct bio *bio)
    {
    unsigned int i;
    struct dm_io_region io[MAX_NR_MIRRORS], *dest = io;
    struct mirror *m;
    let mut op_flags: blk_opf_t = bio.bi_opf & (REQ_FUA | REQ_PREFLUSH | REQ_ATOMIC);
    struct dm_io_request io_req = {
    .bi_opf = REQ_OP_WRITE | op_flags,
    .mem.type = DM_IO_BIO,
    .mem.ptr.bio = bio,
    .notify.fn = write_callback,
    .notify.context = bio,
    .client = ms.io_client,
    };
    if (bio_op(bio) == REQ_OP_DISCARD) {
    io_req.bi_opf = REQ_OP_DISCARD | op_flags;
    io_req.mem.type = DM_IO_KMEM;
    io_req.mem.ptr.addr = core::ptr::null_mut();
    }
    for (i = 0, m = ms.mirror; i < ms.nr_mirrors; i++, m++)
    map_region(dest++, m, bio);
//
// Use default mirror because we only need it to retrieve the reference
// to the mirror set in write_callback().
//
    bio_set_m(bio, get_default_mirror(ms));
    BUG_ON(dm_io(&io_req, ms.nr_mirrors, io, core::ptr::null_mut(), core::ptr::null_mut(), IOPRIO_DEFAULT));
    }
#[no_mangle]
unsafe extern "C" fn do_writes(ms: *mut mirror_set, writes: *mut bio_list) {
    static void do_writes(struct mirror_set *ms, struct bio_list *writes)
    {
    int state;
    struct bio *bio;
    struct bio_list sync, nosync, recover, *this_list = core::ptr::null_mut();
    struct bio_list requeue;
    struct dm_dirty_log *log = dm_rh_dirty_log(ms.rh);
    region_t region;
    if (!writes.head)
    return;
//
// Classify each write.
//
    bio_list_init(&sync);
    bio_list_init(&nosync);
    bio_list_init(&recover);
    bio_list_init(&requeue);
    while ((bio = bio_list_pop(writes))) {
    if ((bio.bi_opf & REQ_PREFLUSH) ||
    (bio_op(bio) == REQ_OP_DISCARD)) {
    bio_list_add(&sync, bio);
    continue;
    }
    region = dm_rh_bio_to_region(ms.rh, bio);
    if (log.type.is_remote_recovering &&
    log.type.is_remote_recovering(log, region)) {
    bio_list_add(&requeue, bio);
    continue;
    }
    state = dm_rh_get_state(ms.rh, region, 1);
    switch (state) {
    case DM_RH_CLEAN:
    case DM_RH_DIRTY:
    this_list = &sync;
    break;
    case DM_RH_NOSYNC:
    this_list = &nosync;
    break;
    case DM_RH_RECOVERING:
    this_list = &recover;
    break;
    }
    bio_list_add(this_list, bio);
    }
//
// Add bios that are delayed due to remote recovery
// back on to the write queue
//
    if (unlikely(requeue.head)) {
    spin_lock_irq(&ms.lock);
    bio_list_merge(&ms.writes, &requeue);
    spin_unlock_irq(&ms.lock);
    delayed_wake(ms);
    }
//
// Increment the pending counts for any regions that will
// be written to (writes to recover regions are going to
// be delayed).
//
    dm_rh_inc_pending(ms.rh, &sync);
    dm_rh_inc_pending(ms.rh, &nosync);
//
// If the flush fails on a previous call and succeeds here,
// we must not reset the log_failure variable.  We need
// userspace interaction to do that.
//
    ms.log_failure = dm_rh_flush(ms.rh) ? 1 : ms.log_failure;
//
// Dispatch io.
//
    if (unlikely(ms.log_failure) && errors_handled(ms)) {
    spin_lock_irq(&ms.lock);
    bio_list_merge(&ms.failures, &sync);
    spin_unlock_irq(&ms.lock);
    wakeup_mirrord(ms);
    } else
    while ((bio = bio_list_pop(&sync)))
    do_write(ms, bio);
    while ((bio = bio_list_pop(&recover)))
    dm_rh_delay(ms.rh, bio);
    while ((bio = bio_list_pop(&nosync))) {
    if (unlikely(ms.leg_failure) && errors_handled(ms) && !keep_log(ms)) {
    spin_lock_irq(&ms.lock);
    bio_list_add(&ms.failures, bio);
    spin_unlock_irq(&ms.lock);
    wakeup_mirrord(ms);
    } else {
    map_bio(get_default_mirror(ms), bio);
    submit_bio_noacct(bio);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn do_failures(ms: *mut mirror_set, failures: *mut bio_list) {
    static void do_failures(struct mirror_set *ms, struct bio_list *failures)
    {
    struct bio *bio;
    if (likely(!failures.head))
    return;
//
// If the log has failed, unattempted writes are being
// put on the holds list.  We can't issue those writes
// until a log has been marked, so we must store them.
//
// If a 'noflush' suspend is in progress, we can requeue
// the I/O's to the core.  This give userspace a chance
// to reconfigure the mirror, at which point the core
// will reissue the writes.  If the 'noflush' flag is
// not set, we have no choice but to return errors.
//
// Some writes on the failures list may have been
// submitted before the log failure and represent a
// failure to write to one of the devices.  It is ok
// for us to treat them the same and requeue them
// as well.
//
    while ((bio = bio_list_pop(failures))) {
    if (!ms.log_failure) {
    ms.in_sync = 0;
    dm_rh_mark_nosync(ms.rh, bio);
    }
//
// If all the legs are dead, fail the I/O.
// If the device has failed and keep_log is enabled,
// fail the I/O.
//
// If we have been told to handle errors, and keep_log
// isn't enabled, hold the bio and wait for userspace to
// deal with the problem.
//
// Otherwise pretend that the I/O succeeded. (This would
// be wrong if the failed leg returned after reboot and
// got replicated back to the good legs.)
//
    if (unlikely(!get_valid_mirror(ms) || (keep_log(ms) && ms.log_failure)))
    bio_io_error(bio);
#[no_mangle]
pub unsafe extern "C" fn if(!keep_log(ms): errors_handled(ms) &&) -> else {
    else if (errors_handled(ms) && !keep_log(ms))
    hold_bio(ms, bio);
    else
    bio_endio(bio);
    }
    }
#[no_mangle]
unsafe extern "C" fn trigger_event(work: *mut work_struct) {
    static void trigger_event(struct work_struct *work)
    {
    struct mirror_set *ms =
    container_of(work, struct mirror_set, trigger_event);
    dm_table_event(ms.ti.table);
    }
//
// ---------------------------------------------------------------
// kmirrord
// ---------------------------------------------------------------
//
#[no_mangle]
unsafe extern "C" fn do_mirror(work: *mut work_struct) {
    static void do_mirror(struct work_struct *work)
    {
    struct mirror_set *ms = container_of(work, struct mirror_set,
    kmirrord_work);
    struct bio_list reads, writes, failures;
    unsigned long flags;
    spin_lock_irqsave(&ms.lock, flags);
    reads = ms.reads;
    writes = ms.writes;
    failures = ms.failures;
    bio_list_init(&ms.reads);
    bio_list_init(&ms.writes);
    bio_list_init(&ms.failures);
    spin_unlock_irqrestore(&ms.lock, flags);
    dm_rh_update_states(ms.rh, errors_handled(ms));
    do_recovery(ms);
    do_reads(ms, &reads);
    do_writes(ms, &writes);
    do_failures(ms, &failures);
    }
//
// ---------------------------------------------------------------
// Target functions
// ---------------------------------------------------------------
//
    static struct mirror_set *alloc_context(unsigned int nr_mirrors,
    uint32_t region_size,
    struct dm_target *ti,
    struct dm_dirty_log *dl)
    {
    struct mirror_set *ms =
    kzalloc_flex(*ms, mirror, nr_mirrors);
    if (!ms) {
    ti.error = "Cannot allocate mirror context";
    return core::ptr::null_mut();
    }
    spin_lock_init(&ms.lock);
    bio_list_init(&ms.reads);
    bio_list_init(&ms.writes);
    bio_list_init(&ms.failures);
    bio_list_init(&ms.holds);
    ms.ti = ti;
    ms.nr_mirrors = nr_mirrors;
    ms.nr_regions = dm_sector_div_up(ti.len, region_size);
    ms.in_sync = 0;
    ms.log_failure = 0;
    ms.leg_failure = 0;
    atomic_set(&ms.suspend, 0);
    atomic_set(&ms.default_mirror, DEFAULT_MIRROR);
    ms.io_client = dm_io_client_create();
    if (IS_ERR(ms.io_client)) {
    ti.error = "Error creating dm_io client";
    kfree(ms);
    return core::ptr::null_mut();
    }
    ms.rh = dm_region_hash_create(ms, dispatch_bios, wakeup_mirrord,
    wakeup_all_recovery_waiters,
    ms.ti.begin, MAX_RECOVERY,
    dl, region_size, ms.nr_regions);
    if (IS_ERR(ms.rh)) {
    ti.error = "Error creating dirty region hash";
    dm_io_client_destroy(ms.io_client);
    kfree(ms);
    return core::ptr::null_mut();
    }
    return ms;
    }
    static void free_context(struct mirror_set *ms, struct dm_target *ti,
    unsigned int m)
    {
    while (m--)
    dm_put_device(ti, ms.mirror[m].dev);
    dm_io_client_destroy(ms.io_client);
    dm_region_hash_destroy(ms.rh);
    kfree(ms);
    }
    static int get_mirror(struct mirror_set *ms, struct dm_target *ti,
    unsigned int mirror, char **argv)
    {
    unsigned long long offset;
    char dummy;
    int ret;
    if (sscanf(argv[1], "%llu%c", &offset, &dummy) != 1 ||
    offset != (sector_t)offset) {
    ti.error = "Invalid offset";
    return -EINVAL;
    }
    ret = dm_get_device(ti, argv[0], dm_table_get_mode(ti.table),
    &ms.mirror[mirror].dev);
    if (ret) {
    ti.error = "Device lookup failure";
    return ret;
    }
    ms.mirror[mirror].ms = ms;
    atomic_set(&(ms.mirror[mirror].error_count), 0);
    ms.mirror[mirror].error_type = 0;
    ms.mirror[mirror].offset = offset;
    return 0;
    }
//
// Create dirty log: log_type #log_params <log_params>
//
    static struct dm_dirty_log *create_dirty_log(struct dm_target *ti,
    unsigned int argc, char **argv,
    unsigned int *args_used)
    {
    unsigned int param_count;
    struct dm_dirty_log *dl;
    char dummy;
    if (argc < 2) {
    ti.error = "Insufficient mirror log arguments";
    return core::ptr::null_mut();
    }
    if (sscanf(argv[1], "%u%c", &param_count, &dummy) != 1) {
    ti.error = "Invalid mirror log argument count";
    return core::ptr::null_mut();
    }
    if (param_count > argc - 2) {
    ti.error = "Insufficient mirror log arguments";
    return core::ptr::null_mut();
    }
// args_used = 2 + param_count;
    dl = dm_dirty_log_create(argv[0], ti, mirror_flush, param_count,
    argv + 2);
    if (!dl) {
    ti.error = "Error creating mirror dirty log";
    return core::ptr::null_mut();
    }
    return dl;
    }
    static int parse_features(struct mirror_set *ms, unsigned int argc, char **argv,
    unsigned int *args_used)
    {
    unsigned int num_features;
    struct dm_target *ti = ms.ti;
    char dummy;
    int i;
// args_used = 0;
    if (!argc)
    return 0;
    if (sscanf(argv[0], "%u%c", &num_features, &dummy) != 1) {
    ti.error = "Invalid number of features";
    return -EINVAL;
    }
    argc--;
    argv++;
    (*args_used)++;
    if (num_features > argc) {
    ti.error = "Not enough arguments to support feature count";
    return -EINVAL;
    }
    for (i = 0; i < num_features; i++) {
    if (!strcmp("handle_errors", argv[0]))
    ms.features |= DM_RAID1_HANDLE_ERRORS;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp("keep_log", _arg: argv[0])) -> else {
    else if (!strcmp("keep_log", argv[0]))
    ms.features |= DM_RAID1_KEEP_LOG;
    else {
    ti.error = "Unrecognised feature requested";
    return -EINVAL;
    }
    argc--;
    argv++;
    (*args_used)++;
    }
    if (!errors_handled(ms) && keep_log(ms)) {
    ti.error = "keep_log feature requires the handle_errors feature";
    return -EINVAL;
    }
    return 0;
    }
//
// Construct a mirror mapping:
//
// log_type #log_params <log_params>
// #mirrors [mirror_path offset]{2,}
// [#features <features>]
//
// log_type is "core" or "disk"
// #log_params is between 1 and 3
//
// If present, supported features are "handle_errors" and "keep_log".
//
#[no_mangle]
unsafe extern "C" fn mirror_ctr(ti: *mut dm_target, argc: c_uint, argv: *mut c_char) -> c_int {
    static int mirror_ctr(struct dm_target *ti, unsigned int argc, char **argv)
    {
    int r;
    unsigned int nr_mirrors, m, args_used;
    struct mirror_set *ms;
    struct dm_dirty_log *dl;
    char dummy;
    dl = create_dirty_log(ti, argc, argv, &args_used);
    if (!dl)
    return -EINVAL;
    argv += args_used;
    argc -= args_used;
    if (!argc || sscanf(argv[0], "%u%c", &nr_mirrors, &dummy) != 1 ||
    nr_mirrors < 2 || nr_mirrors > MAX_NR_MIRRORS) {
    ti.error = "Invalid number of mirrors";
    dm_dirty_log_destroy(dl);
    return -EINVAL;
    }
    argv++, argc--;
    if (argc < nr_mirrors * 2) {
    ti.error = "Too few mirror arguments";
    dm_dirty_log_destroy(dl);
    return -EINVAL;
    }
    ms = alloc_context(nr_mirrors, dl.type.get_region_size(dl), ti, dl);
    if (!ms) {
    dm_dirty_log_destroy(dl);
    return -ENOMEM;
    }
// Get the mirror parameter sets
    for (m = 0; m < nr_mirrors; m++) {
    r = get_mirror(ms, ti, m, argv);
    if (r) {
    free_context(ms, ti, m);
    return r;
    }
    argv += 2;
    argc -= 2;
    }
    ti.private = ms;
    r = dm_set_target_max_io_len(ti, dm_rh_get_region_size(ms.rh));
    if (r)
    goto err_free_context;
    ti.num_flush_bios = 1;
    ti.num_discard_bios = 1;
    ti.per_io_data_size = sizeof(struct dm_raid1_bio_record);
    ms.kmirrord_wq = alloc_workqueue("kmirrord",
    WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    if (!ms.kmirrord_wq) {
    DMERR("couldn't start kmirrord");
    r = -ENOMEM;
    goto err_free_context;
    }
    INIT_WORK(&ms.kmirrord_work, do_mirror);
    timer_setup(&ms.timer, delayed_wake_fn, 0);
    ms.timer_pending = 0;
    INIT_WORK(&ms.trigger_event, trigger_event);
    r = parse_features(ms, argc, argv, &args_used);
    if (r)
    goto err_destroy_wq;
    argv += args_used;
    argc -= args_used;
//
// Any read-balancing addition depends on the
// DM_RAID1_HANDLE_ERRORS flag being present.
// This is because the decision to balance depends
// on the sync state of a region.  If the above
// flag is not present, we ignore errors; and
// the sync state may be inaccurate.
//
    if (argc) {
    ti.error = "Too many mirror arguments";
    r = -EINVAL;
    goto err_destroy_wq;
    }
    ms.kcopyd_client = dm_kcopyd_client_create(&dm_kcopyd_throttle);
    if (IS_ERR(ms.kcopyd_client)) {
    r = PTR_ERR(ms.kcopyd_client);
    goto err_destroy_wq;
    }
    wakeup_mirrord(ms);
    return 0;
    err_destroy_wq:
    destroy_workqueue(ms.kmirrord_wq);
    err_free_context:
    free_context(ms, ti, ms.nr_mirrors);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn mirror_dtr(ti: *mut dm_target) {
    static void mirror_dtr(struct dm_target *ti)
    {
    struct mirror_set *ms = ti.private;
    timer_delete_sync(&ms.timer);
    flush_workqueue(ms.kmirrord_wq);
    flush_work(&ms.trigger_event);
    dm_kcopyd_client_destroy(ms.kcopyd_client);
    destroy_workqueue(ms.kmirrord_wq);
    free_context(ms, ti, ms.nr_mirrors);
    }
//
// Mirror mapping function
//
#[no_mangle]
unsafe extern "C" fn mirror_map(ti: *mut dm_target, bio: *mut bio) -> c_int {
    static int mirror_map(struct dm_target *ti, struct bio *bio)
    {
    int r, rw = bio_data_dir(bio);
    struct mirror *m;
    struct mirror_set *ms = ti.private;
    struct dm_dirty_log *log = dm_rh_dirty_log(ms.rh);
    struct dm_raid1_bio_record *bio_record =
    dm_per_bio_data(bio, sizeof(struct dm_raid1_bio_record));
    bio_record.details.bi_bdev = core::ptr::null_mut();
    if (rw == WRITE) {
// Save region for mirror_end_io() handler
    bio_record.write_region = dm_rh_bio_to_region(ms.rh, bio);
    queue_bio(ms, bio, rw);
    return DM_MAPIO_SUBMITTED;
    }
    r = log.type.in_sync(log, dm_rh_bio_to_region(ms.rh, bio), 0);
    if (r < 0 && r != -EWOULDBLOCK)
    return DM_MAPIO_KILL;
//
// If region is not in-sync queue the bio.
//
    if (!r || (r == -EWOULDBLOCK)) {
    if (bio.bi_opf & REQ_RAHEAD)
    return DM_MAPIO_KILL;
    queue_bio(ms, bio, rw);
    return DM_MAPIO_SUBMITTED;
    }
//
// The region is in-sync and we can perform reads directly.
// Store enough information so we can retry if it fails.
//
    m = choose_mirror(ms, bio.bi_iter.bi_sector);
    if (unlikely(!m))
    return DM_MAPIO_KILL;
    dm_bio_record(&bio_record.details, bio);
    bio_record.m = m;
    map_bio(m, bio);
    return DM_MAPIO_REMAPPED;
    }
    static int mirror_end_io(struct dm_target *ti, struct bio *bio,
    blk_status_t *error)
    {
    let mut rw: c_int = bio_data_dir(bio);
    struct mirror_set *ms = ti.private;
    struct mirror *m = core::ptr::null_mut();
    struct dm_bio_details *bd = core::ptr::null_mut();
    struct dm_raid1_bio_record *bio_record =
    dm_per_bio_data(bio, sizeof(struct dm_raid1_bio_record));
//
// We need to dec pending if this was a write.
//
    if (rw == WRITE) {
    if (!(bio.bi_opf & REQ_PREFLUSH) &&
    bio_op(bio) != REQ_OP_DISCARD)
    dm_rh_dec(ms.rh, bio_record.write_region);
    return DM_ENDIO_DONE;
    }
    if (*error == BLK_STS_NOTSUPP || *error == BLK_STS_INVAL)
    goto out;
    if (bio.bi_opf & REQ_RAHEAD)
    goto out;
    if (unlikely(*error)) {
    if (!bio_record.details.bi_bdev) {
//
// There wasn't enough memory to record necessary
// information for a retry or there was no other
// mirror in-sync.
//
    DMERR_LIMIT("Mirror read failed.");
    return DM_ENDIO_DONE;
    }
    m = bio_record.m;
    DMERR("Mirror read failed from %s. Trying alternative device.",
    m.dev.name);
    fail_mirror(m, DM_RAID1_READ_ERROR);
//
// A failed read is requeued for another attempt using an intact
// mirror.
//
    if (default_ok(m) || mirror_available(ms, bio)) {
    bd = &bio_record.details;
    dm_bio_restore(bd, bio);
    bio_record.details.bi_bdev = core::ptr::null_mut();
    bio.bi_status = 0;
    queue_bio(ms, bio, rw);
    return DM_ENDIO_INCOMPLETE;
    }
    DMERR("All replicated volumes dead, failing I/O");
    }
    out:
    bio_record.details.bi_bdev = core::ptr::null_mut();
    return DM_ENDIO_DONE;
    }
#[no_mangle]
unsafe extern "C" fn mirror_presuspend(ti: *mut dm_target) {
    static void mirror_presuspend(struct dm_target *ti)
    {
    struct mirror_set *ms = ti.private;
    struct dm_dirty_log *log = dm_rh_dirty_log(ms.rh);
    struct bio_list holds;
    struct bio *bio;
    atomic_set(&ms.suspend, 1);
//
// Process bios in the hold list to start recovery waiting
// for bios in the hold list. After the process, no bio has
// a chance to be added in the hold list because ms->suspend
// is set.
//
    spin_lock_irq(&ms.lock);
    holds = ms.holds;
    bio_list_init(&ms.holds);
    spin_unlock_irq(&ms.lock);
    while ((bio = bio_list_pop(&holds)))
    hold_bio(ms, bio);
//
// We must finish up all the work that we've
// generated (i.e. recovery work).
//
    dm_rh_stop_recovery(ms.rh);
    wait_event(_kmirrord_recovery_stopped,
    !dm_rh_recovery_in_flight(ms.rh));
    if (log.type.presuspend && log.type.presuspend(log))
// FIXME: need better error handling
    DMWARN("log presuspend failed");
//
// Now that recovery is complete/stopped and the
// delayed bios are queued, we need to wait for
// the worker thread to complete.  This way,
// we know that all of our I/O has been pushed.
//
    flush_workqueue(ms.kmirrord_wq);
    }
#[no_mangle]
unsafe extern "C" fn mirror_postsuspend(ti: *mut dm_target) {
    static void mirror_postsuspend(struct dm_target *ti)
    {
    struct mirror_set *ms = ti.private;
    struct dm_dirty_log *log = dm_rh_dirty_log(ms.rh);
    if (log.type.postsuspend && log.type.postsuspend(log))
// FIXME: need better error handling
    DMWARN("log postsuspend failed");
    }
#[no_mangle]
unsafe extern "C" fn mirror_resume(ti: *mut dm_target) {
    static void mirror_resume(struct dm_target *ti)
    {
    struct mirror_set *ms = ti.private;
    struct dm_dirty_log *log = dm_rh_dirty_log(ms.rh);
    atomic_set(&ms.suspend, 0);
    if (log.type.resume && log.type.resume(log))
// FIXME: need better error handling
    DMWARN("log resume failed");
    dm_rh_start_recovery(ms.rh);
    }
//
// device_status_char
// @m: mirror device/leg we want the status of
//
// We return one character representing the most severe error
// we have encountered.
// A => Alive - No failures
// D => Dead - A write failure occurred leaving mirror out-of-sync
// S => Sync - A sychronization failure occurred, mirror out-of-sync
// R => Read - A read failure occurred, mirror data unaffected
//
// Returns: <char>
//
#[no_mangle]
unsafe extern "C" fn device_status_char(m: *mut mirror) -> c_char {
    static char device_status_char(struct mirror *m)
    {
    if (!atomic_read(&(m.error_count)))
    return 'A';
    return (test_bit(DM_RAID1_FLUSH_ERROR, &(m.error_type))) ? 'F' :
    (test_bit(DM_RAID1_WRITE_ERROR, &(m.error_type))) ? 'D' :
    (test_bit(DM_RAID1_SYNC_ERROR, &(m.error_type))) ? 'S' :
    (test_bit(DM_RAID1_READ_ERROR, &(m.error_type))) ? 'R' : 'U';
    }
    static void mirror_status(struct dm_target *ti, status_type_t type,
    unsigned int status_flags, char *result, unsigned int maxlen)
    {
    unsigned int m, sz = 0;
    let mut num_feature_args: c_int = 0;
    struct mirror_set *ms = ti.private;
    struct dm_dirty_log *log = dm_rh_dirty_log(ms.rh);
    char buffer[MAX_NR_MIRRORS + 1];
    switch (type) {
    case STATUSTYPE_INFO:
    DMEMIT("%d ", ms.nr_mirrors);
    for (m = 0; m < ms.nr_mirrors; m++) {
    DMEMIT("%s ", ms.mirror[m].dev.name);
    buffer[m] = device_status_char(&(ms.mirror[m]));
    }
    buffer[m] = '\0';
    DMEMIT("%llu/%llu 1 %s ",
    (unsigned long long)log.type.get_sync_count(log),
    (unsigned long long)ms.nr_regions, buffer);
    sz += log.type.status(log, type, result+sz, maxlen-sz);
    break;
    case STATUSTYPE_TABLE:
    sz = log.type.status(log, type, result, maxlen);
    DMEMIT("%d", ms.nr_mirrors);
    for (m = 0; m < ms.nr_mirrors; m++)
    DMEMIT(" %s %llu", ms.mirror[m].dev.name,
    (unsigned long long)ms.mirror[m].offset);
    num_feature_args += !!errors_handled(ms);
    num_feature_args += !!keep_log(ms);
    if (num_feature_args) {
    DMEMIT(" %d", num_feature_args);
    if (errors_handled(ms))
    DMEMIT(" handle_errors");
    if (keep_log(ms))
    DMEMIT(" keep_log");
    }
    break;
    case STATUSTYPE_IMA:
    DMEMIT_TARGET_NAME_VERSION(ti.type);
    DMEMIT(",nr_mirrors=%d", ms.nr_mirrors);
    for (m = 0; m < ms.nr_mirrors; m++) {
    DMEMIT(",mirror_device_%d=%s", m, ms.mirror[m].dev.name);
    DMEMIT(",mirror_device_%d_status=%c",
    m, device_status_char(&(ms.mirror[m])));
    }
    DMEMIT(",handle_errors=%c", errors_handled(ms) ? 'y' : 'n');
    DMEMIT(",keep_log=%c", keep_log(ms) ? 'y' : 'n');
    DMEMIT(",log_type_status=");
    sz += log.type.status(log, type, result+sz, maxlen-sz);
    DMEMIT(";");
    break;
    }
    }
    static int mirror_iterate_devices(struct dm_target *ti,
    iterate_devices_callout_fn fn, void *data)
    {
    struct mirror_set *ms = ti.private;
    let mut ret: c_int = 0;
    unsigned int i;
    for (i = 0; !ret && i < ms.nr_mirrors; i++)
    ret = fn(ti, ms.mirror[i].dev,
    ms.mirror[i].offset, ti.len, data);
    return ret;
    }
    static struct target_type mirror_target = {
    .name	 = "mirror",
    .version = {1, 15, 0},
    .module	 = THIS_MODULE,
    .features = DM_TARGET_ATOMIC_WRITES,
    .ctr	 = mirror_ctr,
    .dtr	 = mirror_dtr,
    .map	 = mirror_map,
    .end_io	 = mirror_end_io,
    .presuspend = mirror_presuspend,
    .postsuspend = mirror_postsuspend,
    .resume	 = mirror_resume,
    .status	 = mirror_status,
    .iterate_devices = mirror_iterate_devices,
    };
#[no_mangle]
unsafe extern "C" fn dm_mirror_init() -> int __init {
    static int __init dm_mirror_init(void)
    {
    int r;
    dm_raid1_wq = alloc_workqueue("dm_raid1_wq", WQ_PERCPU, 0);
    if (!dm_raid1_wq) {
    DMERR("Failed to alloc workqueue");
    return -ENOMEM;
    }
    r = dm_register_target(&mirror_target);
    if (r < 0) {
    destroy_workqueue(dm_raid1_wq);
    return r;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dm_mirror_exit() -> void __exit {
    static void __exit dm_mirror_exit(void)
    {
    destroy_workqueue(dm_raid1_wq);
    dm_unregister_target(&mirror_target);
    }
// Module hooks
    module_init(dm_mirror_init);
    module_exit(dm_mirror_exit);
    MODULE_DESCRIPTION(DM_NAME " mirror target");
    MODULE_AUTHOR("Joe Thornber");
    MODULE_LICENSE("GPL");

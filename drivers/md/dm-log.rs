//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-log.c
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
// Copyright (C) 2003 Sistina Software
// Copyright (C) 2004-2008 Red Hat, Inc. All rights reserved.
//
// This file is released under the LGPL.
//

    static LIST_HEAD(_log_types);
    static DEFINE_SPINLOCK(_lock);
    static struct dm_dirty_log_type *__find_dirty_log_type(const char *name)
    {
    struct dm_dirty_log_type *log_type;
    list_for_each_entry(log_type, &_log_types, list)
    if (!strcmp(name, log_type.name))
    return log_type;
    return core::ptr::null_mut();
    }
    static struct dm_dirty_log_type *_get_dirty_log_type(const char *name)
    {
    struct dm_dirty_log_type *log_type;
    spin_lock(&_lock);
    log_type = __find_dirty_log_type(name);
    if (log_type && !try_module_get(log_type.module))
    log_type = core::ptr::null_mut();
    spin_unlock(&_lock);
    return log_type;
    }
//
// get_type
// @type_name
//
// Attempt to retrieve the dm_dirty_log_type by name.  If not already
// available, attempt to load the appropriate module.
//
// Log modules are named "dm-log-" followed by the 'type_name'.
// Modules may contain multiple types.
// This function will first try the module "dm-log-<type_name>",
// then truncate 'type_name' on the last '-' and try again.
//
// For example, if type_name was "clustered-disk", it would search
// 'dm-log-clustered-disk' then 'dm-log-clustered'.
//
// Returns: dirty_log_type* on success, NULL on failure
//
    static struct dm_dirty_log_type *get_type(const char *type_name)
    {
    char *p, *type_name_dup;
    struct dm_dirty_log_type *log_type;
    if (!type_name)
    return core::ptr::null_mut();
    log_type = _get_dirty_log_type(type_name);
    if (log_type)
    return log_type;
    type_name_dup = kstrdup(type_name, GFP_KERNEL);
    if (!type_name_dup) {
    DMWARN("No memory left to attempt log module load for \"%s\"",
    type_name);
    return core::ptr::null_mut();
    }
    while (request_module("dm-log-%s", type_name_dup) ||
    !(log_type = _get_dirty_log_type(type_name))) {
    p = strrchr(type_name_dup, '-');
    if (!p)
    break;
    p[0] = '\0';
    }
    if (!log_type)
    DMWARN("Module for logging type \"%s\" not found.", type_name);
    kfree(type_name_dup);
    return log_type;
    }
#[no_mangle]
unsafe extern "C" fn put_type(type: *mut dm_dirty_log_type) {
    static void put_type(struct dm_dirty_log_type *type)
    {
    if (!type)
    return;
    spin_lock(&_lock);
    if (!__find_dirty_log_type(type.name))
    goto out;
    module_put(type.module);
    out:
    spin_unlock(&_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn dm_dirty_log_type_register(type: *mut dm_dirty_log_type) -> c_int {
    int dm_dirty_log_type_register(struct dm_dirty_log_type *type)
    {
    let mut r: c_int = 0;
    spin_lock(&_lock);
    if (!__find_dirty_log_type(type.name))
    list_add(&type.list, &_log_types);
    else
    r = -EBUSY;
    spin_unlock(&_lock);
    return r;
    }
    EXPORT_SYMBOL(dm_dirty_log_type_register);
#[no_mangle]
pub unsafe extern "C" fn dm_dirty_log_type_unregister(type: *mut dm_dirty_log_type) -> c_int {
    int dm_dirty_log_type_unregister(struct dm_dirty_log_type *type)
    {
    spin_lock(&_lock);
    if (!__find_dirty_log_type(type.name)) {
    spin_unlock(&_lock);
    return -EINVAL;
    }
    list_del(&type.list);
    spin_unlock(&_lock);
    return 0;
    }
    EXPORT_SYMBOL(dm_dirty_log_type_unregister);
    struct dm_dirty_log *dm_dirty_log_create(const char *type_name,
    struct dm_target *ti,
    int (*flush_callback_fn)(struct dm_target *ti),
    unsigned int argc, char **argv)
    {
    struct dm_dirty_log_type *type;
    struct dm_dirty_log *log;
    log = kmalloc_obj(*log);
    if (!log)
    return core::ptr::null_mut();
    type = get_type(type_name);
    if (!type) {
    kfree(log);
    return core::ptr::null_mut();
    }
    log.flush_callback_fn = flush_callback_fn;
    log.type = type;
    if (type.ctr(log, ti, argc, argv)) {
    kfree(log);
    put_type(type);
    return core::ptr::null_mut();
    }
    return log;
    }
    EXPORT_SYMBOL(dm_dirty_log_create);
#[no_mangle]
pub unsafe extern "C" fn dm_dirty_log_destroy(log: *mut dm_dirty_log) {
    void dm_dirty_log_destroy(struct dm_dirty_log *log)
    {
    log.type.dtr(log);
    put_type(log.type);
    kfree(log);
    }
    EXPORT_SYMBOL(dm_dirty_log_destroy);
//
// ---------------------------------------------------------------
// Persistent and core logs share a lot of their implementation.
// FIXME: need a reload method to be called from a resume
// ---------------------------------------------------------------
//
// Magic for persistent mirrors: "MiRr"
//
pub const MIRROR_MAGIC: c_uint = 0x4D695272;
//
// The on-disk version of the metadata.
//
pub const MIRROR_DISK_VERSION: c_int = 2;
pub const LOG_OFFSET: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct log_header_disk {
    pub magic: __le32,
//
// Simple, incrementing version. no backward
// compatibility.
//
    pub version: __le32,
    pub nr_regions: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct log_header_core {
    pub magic: u32,
    pub version: u32,
    pub nr_regions: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct log_c {
    pub ti: *mut dm_target,
    pub touched_dirtied: c_int,
    pub touched_cleaned: c_int,
    pub flush_failed: c_int,
    pub region_size: u32,
    pub region_count: c_uint,
    pub sync_count: region_t,
    pub bitset_uint32_count: c_uint,
    pub clean_bits: *mut u32,
    pub sync_bits: *mut u32,
    pub /: *mut *mut *mut uint32_t recovering_bits; / FIXME: this seems excessive,
    pub sync_search: c_int,
// Resync flag
    enum sync {
    DEFAULTSYNC,	/* Synchronize if necessary */
    NOSYNC,		/* Devices known to be already in sync */
    FORCESYNC,	/* Force a sync to happen */
    pub sync: },
    pub io_req: dm_io_request,
//
// Disk log fields
//
    pub log_dev_failed: c_int,
    pub log_dev_flush_failed: c_int,
    pub log_dev: *mut dm_dev,
    pub header: log_header_core,
    pub header_location: dm_io_region,
    pub disk_header: *mut log_header_disk,
}

//
// The touched member needs to be updated every time we access
// one of the bitsets.
//
#[no_mangle]
pub unsafe extern "C" fn log_test_bit(bs: *mut u32, bit: c_uint) -> c_int {
    static inline int log_test_bit(uint32_t *bs, unsigned int bit)
    {
    return test_bit_le(bit, bs) ? 1 : 0;
    }
    static inline void log_set_bit(struct log_c *l,
    uint32_t *bs, unsigned int bit)
    {
    __set_bit_le(bit, bs);
    l.touched_cleaned = 1;
    }
    static inline void log_clear_bit(struct log_c *l,
    uint32_t *bs, unsigned int bit)
    {
    __clear_bit_le(bit, bs);
    l.touched_dirtied = 1;
    }
//
// ---------------------------------------------------------------
// Header IO
// --------------------------------------------------------------
//
#[no_mangle]
unsafe extern "C" fn header_to_disk(core: *mut log_header_core, disk: *mut log_header_disk) {
    static void header_to_disk(struct log_header_core *core, struct log_header_disk *disk)
    {
    disk.magic = cpu_to_le32(core.magic);
    disk.version = cpu_to_le32(core.version);
    disk.nr_regions = cpu_to_le64(core.nr_regions);
    }
#[no_mangle]
unsafe extern "C" fn header_from_disk(core: *mut log_header_core, disk: *mut log_header_disk) {
    static void header_from_disk(struct log_header_core *core, struct log_header_disk *disk)
    {
    core.magic = le32_to_cpu(disk.magic);
    core.version = le32_to_cpu(disk.version);
    core.nr_regions = le64_to_cpu(disk.nr_regions);
    }
#[no_mangle]
unsafe extern "C" fn rw_header(lc: *mut log_c, op: enum req_op) -> c_int {
    static int rw_header(struct log_c *lc, enum req_op op)
    {
    lc.io_req.bi_opf = op;
    return dm_io(&lc.io_req, 1, &lc.header_location, core::ptr::null_mut(), core::ptr::null_mut(), IOPRIO_DEFAULT);
    }
#[no_mangle]
unsafe extern "C" fn flush_header(lc: *mut log_c) -> c_int {
    static int flush_header(struct log_c *lc)
    {
    struct dm_io_region null_location = {
    .bdev = lc.header_location.bdev,
    .sector = 0,
    .count = 0,
    };
    lc.io_req.bi_opf = REQ_OP_WRITE | REQ_PREFLUSH;
    return dm_io(&lc.io_req, 1, &null_location, core::ptr::null_mut(), core::ptr::null_mut(), IOPRIO_DEFAULT);
    }
#[no_mangle]
unsafe extern "C" fn read_header(log: *mut log_c) -> c_int {
    static int read_header(struct log_c *log)
    {
    int r;
    r = rw_header(log, REQ_OP_READ);
    if (r)
    return r;
    header_from_disk(&log.header, log.disk_header);
// New log required?
    if (log.sync != DEFAULTSYNC || log.header.magic != MIRROR_MAGIC) {
    log.header.magic = MIRROR_MAGIC;
    log.header.version = MIRROR_DISK_VERSION;
    log.header.nr_regions = 0;
    }

    if (log.header.version == 1)
    log.header.version = 2;

    if (log.header.version != MIRROR_DISK_VERSION) {
    DMWARN("incompatible disk log version");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn _check_region_size(ti: *mut dm_target, region_size: u32) -> c_int {
    static int _check_region_size(struct dm_target *ti, uint32_t region_size)
    {
    if (region_size < 2 || region_size > ti.len)
    return 0;
    if (!is_power_of_2(region_size))
    return 0;
    return 1;
    }
//
// --------------------------------------------------------------
// core log constructor/destructor
//
// argv contains region_size followed optionally by [no]sync
// --------------------------------------------------------------
//
pub const BYTE_SHIFT: c_int = 3;
    static int create_log_context(struct dm_dirty_log *log, struct dm_target *ti,
    unsigned int argc, char **argv,
    struct dm_dev *dev)
    {
    let mut sync: enum sync = DEFAULTSYNC;
    struct log_c *lc;
    uint32_t region_size;
    sector_t region_count;
    size_t bitset_size, buf_size;
    int r;
    char dummy;
    if (argc < 1 || argc > 2) {
    DMWARN("wrong number of arguments to dirty region log");
    return -EINVAL;
    }
    if (argc > 1) {
    if (!strcmp(argv[1], "sync"))
    sync = FORCESYNC;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(argv[1], _arg: "nosync")) -> else {
    else if (!strcmp(argv[1], "nosync"))
    sync = NOSYNC;
    else {
    DMWARN("unrecognised sync argument to dirty region log: %s", argv[1]);
    return -EINVAL;
    }
    }
    if (sscanf(argv[0], "%u%c", &region_size, &dummy) != 1 ||
    !_check_region_size(ti, region_size)) {
    DMWARN("invalid region size %s", argv[0]);
    return -EINVAL;
    }
    region_count = dm_sector_div_up(ti.len, region_size);
    if (region_count > UINT_MAX) {
    DMWARN("region count exceeds limit of %u", UINT_MAX);
    return -EINVAL;
    }
    lc = kmalloc_obj(*lc);
    if (!lc) {
    DMWARN("couldn't allocate core log");
    return -ENOMEM;
    }
    lc.ti = ti;
    lc.touched_dirtied = 0;
    lc.touched_cleaned = 0;
    lc.flush_failed = 0;
    lc.region_size = region_size;
    lc.region_count = region_count;
    lc.sync = sync;
//
// Work out how many "unsigned long"s we need to hold the bitset.
//
    bitset_size = dm_round_up(region_count, BITS_PER_LONG);
    bitset_size >>= BYTE_SHIFT;
// Handle dm_round_up rollover on 32-bit systems
    if (!bitset_size)
    bitset_size = 1UL << (BITS_PER_LONG - BYTE_SHIFT);
    lc.bitset_uint32_count = bitset_size / sizeof(*lc.clean_bits);
//
// Disk log?
//
    if (!dev) {
    lc.clean_bits = vmalloc(bitset_size);
    if (!lc.clean_bits) {
    DMWARN("couldn't allocate clean bitset");
    kfree(lc);
    return -ENOMEM;
    }
    lc.disk_header = core::ptr::null_mut();
    } else {
    lc.log_dev = dev;
    lc.log_dev_failed = 0;
    lc.log_dev_flush_failed = 0;
    lc.header_location.bdev = lc.log_dev.bdev;
    lc.header_location.sector = 0;
//
// Buffer holds both header and bitset.
//
    buf_size =
    dm_round_up((LOG_OFFSET << SECTOR_SHIFT) + bitset_size,
    bdev_logical_block_size(lc.header_location.bdev));
    if (buf_size > bdev_nr_bytes(dev.bdev)) {
    DMWARN("log device %s too small: need %llu bytes",
    dev.name, (unsigned long long)buf_size);
    kfree(lc);
    return -EINVAL;
    }
    lc.header_location.count = buf_size >> SECTOR_SHIFT;
    lc.io_req.mem.type = DM_IO_VMA;
    lc.io_req.notify.fn = core::ptr::null_mut();
    lc.io_req.client = dm_io_client_create();
    if (IS_ERR(lc.io_req.client)) {
    r = PTR_ERR(lc.io_req.client);
    DMWARN("couldn't allocate disk io client");
    kfree(lc);
    return r;
    }
    lc.disk_header = vmalloc(buf_size);
    if (!lc.disk_header) {
    DMWARN("couldn't allocate disk log buffer");
    dm_io_client_destroy(lc.io_req.client);
    kfree(lc);
    return -ENOMEM;
    }
    lc.io_req.mem.ptr.vma = lc.disk_header;
    lc.clean_bits = (void *)lc.disk_header +
    (LOG_OFFSET << SECTOR_SHIFT);
    }
    memset(lc.clean_bits, -1, bitset_size);
    lc.sync_bits = vmalloc(bitset_size);
    if (!lc.sync_bits) {
    DMWARN("couldn't allocate sync bitset");
    if (!dev)
    vfree(lc.clean_bits);
    else
    dm_io_client_destroy(lc.io_req.client);
    vfree(lc.disk_header);
    kfree(lc);
    return -ENOMEM;
    }
    memset(lc.sync_bits, (sync == NOSYNC) ? -1 : 0, bitset_size);
    lc.sync_count = (sync == NOSYNC) ? region_count : 0;
    lc.recovering_bits = vzalloc(bitset_size);
    if (!lc.recovering_bits) {
    DMWARN("couldn't allocate sync bitset");
    vfree(lc.sync_bits);
    if (!dev)
    vfree(lc.clean_bits);
    else
    dm_io_client_destroy(lc.io_req.client);
    vfree(lc.disk_header);
    kfree(lc);
    return -ENOMEM;
    }
    lc.sync_search = 0;
    log.context = lc;
    return 0;
    }
    static int core_ctr(struct dm_dirty_log *log, struct dm_target *ti,
    unsigned int argc, char **argv)
    {
    return create_log_context(log, ti, argc, argv, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn destroy_log_context(lc: *mut log_c) {
    static void destroy_log_context(struct log_c *lc)
    {
    vfree(lc.sync_bits);
    vfree(lc.recovering_bits);
    kfree(lc);
    }
#[no_mangle]
unsafe extern "C" fn core_dtr(log: *mut dm_dirty_log) {
    static void core_dtr(struct dm_dirty_log *log)
    {
    struct log_c *lc = log.context;
    vfree(lc.clean_bits);
    destroy_log_context(lc);
    }
//
// ---------------------------------------------------------------------
// disk log constructor/destructor
//
// argv contains log_device region_size followed optionally by [no]sync
// ---------------------------------------------------------------------
//
    static int disk_ctr(struct dm_dirty_log *log, struct dm_target *ti,
    unsigned int argc, char **argv)
    {
    int r;
    struct dm_dev *dev;
    if (argc < 2 || argc > 3) {
    DMWARN("wrong number of arguments to disk dirty region log");
    return -EINVAL;
    }
    r = dm_get_device(ti, argv[0], dm_table_get_mode(ti.table), &dev);
    if (r)
    return r;
    r = create_log_context(log, ti, argc - 1, argv + 1, dev);
    if (r) {
    dm_put_device(ti, dev);
    return r;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn disk_dtr(log: *mut dm_dirty_log) {
    static void disk_dtr(struct dm_dirty_log *log)
    {
    struct log_c *lc = log.context;
    dm_put_device(lc.ti, lc.log_dev);
    vfree(lc.disk_header);
    dm_io_client_destroy(lc.io_req.client);
    destroy_log_context(lc);
    }
#[no_mangle]
unsafe extern "C" fn fail_log_device(lc: *mut log_c) {
    static void fail_log_device(struct log_c *lc)
    {
    if (lc.log_dev_failed)
    return;
    lc.log_dev_failed = 1;
    dm_table_event(lc.ti.table);
    }
#[no_mangle]
unsafe extern "C" fn disk_resume(log: *mut dm_dirty_log) -> c_int {
    static int disk_resume(struct dm_dirty_log *log)
    {
    int r;
    unsigned int i;
    struct log_c *lc = log.context;
    let mut size: usize = lc.bitset_uint32_count * sizeof(uint32_t);
// read the disk header
    r = read_header(lc);
    if (r) {
    DMWARN("%s: Failed to read header on dirty region log device",
    lc.log_dev.name);
    fail_log_device(lc);
//
// If the log device cannot be read, we must assume
// all regions are out-of-sync.  If we simply return
// here, the state will be uninitialized and could
// lead us to return 'in-sync' status for regions
// that are actually 'out-of-sync'.
//
    lc.header.nr_regions = 0;
    }
// set or clear any new bits -- device has grown
    if (lc.sync == NOSYNC)
    for (i = lc.header.nr_regions; i < lc.region_count; i++)
// FIXME: amazingly inefficient
    log_set_bit(lc, lc.clean_bits, i);
    else
    for (i = lc.header.nr_regions; i < lc.region_count; i++)
// FIXME: amazingly inefficient
    log_clear_bit(lc, lc.clean_bits, i);
// clear any old bits -- device has shrunk
    for (i = lc.region_count; i % BITS_PER_LONG; i++)
    log_clear_bit(lc, lc.clean_bits, i);
// copy clean across to sync
    memcpy(lc.sync_bits, lc.clean_bits, size);
    lc.sync_count = memweight(lc.clean_bits,
    lc.bitset_uint32_count * sizeof(uint32_t));
    lc.sync_search = 0;
// set the correct number of regions in the header
    lc.header.nr_regions = lc.region_count;
    header_to_disk(&lc.header, lc.disk_header);
// write the new header
    r = rw_header(lc, REQ_OP_WRITE);
    if (!r) {
    r = flush_header(lc);
    if (r)
    lc.log_dev_flush_failed = 1;
    }
    if (r) {
    DMWARN("%s: Failed to write header on dirty region log device",
    lc.log_dev.name);
    fail_log_device(lc);
    }
    return r;
    }
#[no_mangle]
unsafe extern "C" fn core_get_region_size(log: *mut dm_dirty_log) -> u32 {
    static uint32_t core_get_region_size(struct dm_dirty_log *log)
    {
    struct log_c *lc = log.context;
    return lc.region_size;
    }
#[no_mangle]
unsafe extern "C" fn core_resume(log: *mut dm_dirty_log) -> c_int {
    static int core_resume(struct dm_dirty_log *log)
    {
    struct log_c *lc = log.context;
    lc.sync_search = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn core_is_clean(log: *mut dm_dirty_log, region: region_t) -> c_int {
    static int core_is_clean(struct dm_dirty_log *log, region_t region)
    {
    struct log_c *lc = log.context;
    return log_test_bit(lc.clean_bits, region);
    }
#[no_mangle]
unsafe extern "C" fn core_in_sync(log: *mut dm_dirty_log, region: region_t, block: c_int) -> c_int {
    static int core_in_sync(struct dm_dirty_log *log, region_t region, int block)
    {
    struct log_c *lc = log.context;
    return log_test_bit(lc.sync_bits, region);
    }
#[no_mangle]
unsafe extern "C" fn core_flush(log: *mut dm_dirty_log) -> c_int {
    static int core_flush(struct dm_dirty_log *log)
    {
// no op
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn disk_flush(log: *mut dm_dirty_log) -> c_int {
    static int disk_flush(struct dm_dirty_log *log)
    {
    int r, i;
    struct log_c *lc = log.context;
// only write if the log has changed
    if (!lc.touched_cleaned && !lc.touched_dirtied)
    return 0;
    if (lc.touched_cleaned && log.flush_callback_fn &&
    log.flush_callback_fn(lc.ti)) {
//
// At this point it is impossible to determine which
// regions are clean and which are dirty (without
// re-reading the log off disk). So mark all of them
// dirty.
//
    lc.flush_failed = 1;
    for (i = 0; i < lc.region_count; i++)
    log_clear_bit(lc, lc.clean_bits, i);
    }
    r = rw_header(lc, REQ_OP_WRITE);
    if (r)
    fail_log_device(lc);
    else {
    if (lc.touched_dirtied) {
    r = flush_header(lc);
    if (r) {
    lc.log_dev_flush_failed = 1;
    fail_log_device(lc);
    } else
    lc.touched_dirtied = 0;
    }
    lc.touched_cleaned = 0;
    }
    return r;
    }
#[no_mangle]
unsafe extern "C" fn core_mark_region(log: *mut dm_dirty_log, region: region_t) {
    static void core_mark_region(struct dm_dirty_log *log, region_t region)
    {
    struct log_c *lc = log.context;
    log_clear_bit(lc, lc.clean_bits, region);
    }
#[no_mangle]
unsafe extern "C" fn core_clear_region(log: *mut dm_dirty_log, region: region_t) {
    static void core_clear_region(struct dm_dirty_log *log, region_t region)
    {
    struct log_c *lc = log.context;
    if (likely(!lc.flush_failed))
    log_set_bit(lc, lc.clean_bits, region);
    }
#[no_mangle]
unsafe extern "C" fn core_get_resync_work(log: *mut dm_dirty_log, region: *mut region_t) -> c_int {
    static int core_get_resync_work(struct dm_dirty_log *log, region_t *region)
    {
    struct log_c *lc = log.context;
    if (lc.sync_search >= lc.region_count)
    return 0;
    do {
// region = find_next_zero_bit_le(lc->sync_bits,
    lc.region_count,
    lc.sync_search);
    lc.sync_search = *region + 1;
    if (*region >= lc.region_count)
    return 0;
    } while (log_test_bit(lc.recovering_bits, *region));
    log_set_bit(lc, lc.recovering_bits, *region);
    return 1;
    }
    static void core_set_region_sync(struct dm_dirty_log *log, region_t region,
    int in_sync)
    {
    struct log_c *lc = log.context;
    log_clear_bit(lc, lc.recovering_bits, region);
    if (in_sync) {
    log_set_bit(lc, lc.sync_bits, region);
    lc.sync_count++;
    } else if (log_test_bit(lc.sync_bits, region)) {
    lc.sync_count--;
    log_clear_bit(lc, lc.sync_bits, region);
    }
    }
#[no_mangle]
unsafe extern "C" fn core_get_sync_count(log: *mut dm_dirty_log) -> region_t {
    static region_t core_get_sync_count(struct dm_dirty_log *log)
    {
    struct log_c *lc = log.context;
    return lc.sync_count;
    }

    do { \
    if (lc.sync != DEFAULTSYNC) \
    DMEMIT("%ssync ", lc.sync == NOSYNC ? "no" : ""); \
    } while (0)
    static int core_status(struct dm_dirty_log *log, status_type_t status,
    char *result, unsigned int maxlen)
    {
    let mut sz: c_int = 0;
    struct log_c *lc = log.context;
    switch (status) {
    case STATUSTYPE_INFO:
    DMEMIT("1 %s", log.type.name);
    break;
    case STATUSTYPE_TABLE:
    DMEMIT("%s %u %u ", log.type.name,
    lc.sync == DEFAULTSYNC ? 1 : 2, lc.region_size);
    DMEMIT_SYNC;
    break;
    case STATUSTYPE_IMA:
// result = '\0';
    break;
    }
    return sz;
    }
    static int disk_status(struct dm_dirty_log *log, status_type_t status,
    char *result, unsigned int maxlen)
    {
    let mut sz: c_int = 0;
    struct log_c *lc = log.context;
    switch (status) {
    case STATUSTYPE_INFO:
    DMEMIT("3 %s %s %c", log.type.name, lc.log_dev.name,
    lc.log_dev_flush_failed ? 'F' :
    lc.log_dev_failed ? 'D' :
    'A');
    break;
    case STATUSTYPE_TABLE:
    DMEMIT("%s %u %s %u ", log.type.name,
    lc.sync == DEFAULTSYNC ? 2 : 3, lc.log_dev.name,
    lc.region_size);
    DMEMIT_SYNC;
    break;
    case STATUSTYPE_IMA:
// result = '\0';
    break;
    }
    return sz;
    }
    static struct dm_dirty_log_type _core_type = {
    .name = "core",
    .module = THIS_MODULE,
    .ctr = core_ctr,
    .dtr = core_dtr,
    .resume = core_resume,
    .get_region_size = core_get_region_size,
    .is_clean = core_is_clean,
    .in_sync = core_in_sync,
    .flush = core_flush,
    .mark_region = core_mark_region,
    .clear_region = core_clear_region,
    .get_resync_work = core_get_resync_work,
    .set_region_sync = core_set_region_sync,
    .get_sync_count = core_get_sync_count,
    .status = core_status,
    };
    static struct dm_dirty_log_type _disk_type = {
    .name = "disk",
    .module = THIS_MODULE,
    .ctr = disk_ctr,
    .dtr = disk_dtr,
    .postsuspend = disk_flush,
    .resume = disk_resume,
    .get_region_size = core_get_region_size,
    .is_clean = core_is_clean,
    .in_sync = core_in_sync,
    .flush = disk_flush,
    .mark_region = core_mark_region,
    .clear_region = core_clear_region,
    .get_resync_work = core_get_resync_work,
    .set_region_sync = core_set_region_sync,
    .get_sync_count = core_get_sync_count,
    .status = disk_status,
    };
#[no_mangle]
unsafe extern "C" fn dm_dirty_log_init() -> int __init {
    static int __init dm_dirty_log_init(void)
    {
    int r;
    r = dm_dirty_log_type_register(&_core_type);
    if (r)
    DMWARN("couldn't register core log");
    r = dm_dirty_log_type_register(&_disk_type);
    if (r) {
    DMWARN("couldn't register disk type");
    dm_dirty_log_type_unregister(&_core_type);
    }
    return r;
    }
#[no_mangle]
unsafe extern "C" fn dm_dirty_log_exit() -> void __exit {
    static void __exit dm_dirty_log_exit(void)
    {
    dm_dirty_log_type_unregister(&_disk_type);
    dm_dirty_log_type_unregister(&_core_type);
    }
    module_init(dm_dirty_log_init);
    module_exit(dm_dirty_log_exit);
    MODULE_DESCRIPTION(DM_NAME " dirty region log");
    MODULE_AUTHOR("Joe Thornber, Heinz Mauelshagen <dm-devel@lists.linux.dev>");
    MODULE_LICENSE("GPL");

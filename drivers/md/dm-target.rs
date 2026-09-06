//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-target.c
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
// Copyright (C) 2001 Sistina Software (UK) Limited
//
// This file is released under the GPL.
//

    static LIST_HEAD(_targets);
    static DECLARE_RWSEM(_lock);
    static inline struct target_type *__find_target_type(const char *name)
    {
    struct target_type *tt;
    list_for_each_entry(tt, &_targets, list)
    if (!strcmp(name, tt.name))
    return tt;
    return core::ptr::null_mut();
    }
    static struct target_type *get_target_type(const char *name)
    {
    struct target_type *tt;
    down_read(&_lock);
    tt = __find_target_type(name);
    if (tt && !try_module_get(tt.module))
    tt = core::ptr::null_mut();
    up_read(&_lock);
    return tt;
    }
#[no_mangle]
unsafe extern "C" fn load_module(name: *const c_char) {
    static void load_module(const char *name)
    {
    request_module("dm-%s", name);
    }
    struct target_type *dm_get_target_type(const char *name)
    {
    struct target_type *tt = get_target_type(name);
    if (!tt) {
    load_module(name);
    tt = get_target_type(name);
    }
    return tt;
    }
#[no_mangle]
pub unsafe extern "C" fn dm_put_target_type(tt: *mut target_type) {
    void dm_put_target_type(struct target_type *tt)
    {
    down_read(&_lock);
    module_put(tt.module);
    up_read(&_lock);
    }
    int dm_target_iterate(void (*iter_func)(struct target_type *tt,
    void *param), void *param)
    {
    struct target_type *tt;
    down_read(&_lock);
    list_for_each_entry(tt, &_targets, list)
    iter_func(tt, param);
    up_read(&_lock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dm_register_target(tt: *mut target_type) -> c_int {
    int dm_register_target(struct target_type *tt)
    {
    let mut rv: c_int = 0;
    down_write(&_lock);
    if (__find_target_type(tt.name)) {
    DMERR("%s: '%s' target already registered",
    __func__, tt.name);
    rv = -EBUSY;
    } else {
    list_add(&tt.list, &_targets);
    }
    up_write(&_lock);
    return rv;
    }
    EXPORT_SYMBOL(dm_register_target);
#[no_mangle]
pub unsafe extern "C" fn dm_unregister_target(tt: *mut target_type) {
    void dm_unregister_target(struct target_type *tt)
    {
    down_write(&_lock);
    if (!__find_target_type(tt.name)) {
    DMCRIT("Unregistering unrecognised target: %s", tt.name);
    BUG();
    }
    list_del(&tt.list);
    up_write(&_lock);
    }
    EXPORT_SYMBOL(dm_unregister_target);
//
// io-err: always fails an io, useful for bringing
// up LVs that have holes in them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_err_c {
    pub dev: *mut dm_dev,
    pub start: sector_t,
}

#[no_mangle]
unsafe extern "C" fn io_err_get_args(tt: *mut dm_target, argc: c_uint, args: *mut c_char) -> c_int {
    static int io_err_get_args(struct dm_target *tt, unsigned int argc, char **args)
    {
    unsigned long long start;
    struct io_err_c *ioec;
    char dummy;
    int ret;
    ioec = kmalloc_obj(*ioec);
    if (!ioec) {
    tt.error = "Cannot allocate io_err context";
    return -ENOMEM;
    }
    ret = -EINVAL;
    if (sscanf(args[1], "%llu%c", &start, &dummy) != 1 ||
    start != (sector_t)start) {
    tt.error = "Invalid device sector";
    goto bad;
    }
    ioec.start = start;
    ret = dm_get_device(tt, args[0], dm_table_get_mode(tt.table), &ioec.dev);
    if (ret) {
    tt.error = "Device lookup failed";
    goto bad;
    }
    tt.private = ioec;
    return 0;
    bad:
    kfree(ioec);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn io_err_ctr(tt: *mut dm_target, argc: c_uint, args: *mut c_char) -> c_int {
    static int io_err_ctr(struct dm_target *tt, unsigned int argc, char **args)
    {
//
// If we have arguments, assume it is the path to the backing
// block device and its mapping start sector (same as dm-linear).
// In this case, get the device so that we can get its limits.
//
    if (argc == 2) {
    let mut ret: c_int = io_err_get_args(tt, argc, args);
    if (ret)
    return ret;
    }
//
// Return error for discards instead of -EOPNOTSUPP
//
    tt.num_discard_bios = 1;
    tt.discards_supported = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn io_err_dtr(tt: *mut dm_target) {
    static void io_err_dtr(struct dm_target *tt)
    {
    struct io_err_c *ioec = tt.private;
    if (ioec) {
    dm_put_device(tt, ioec.dev);
    kfree(ioec);
    }
    }
#[no_mangle]
unsafe extern "C" fn io_err_map(tt: *mut dm_target, bio: *mut bio) -> c_int {
    static int io_err_map(struct dm_target *tt, struct bio *bio)
    {
    return DM_MAPIO_KILL;
    }
    static int io_err_clone_and_map_rq(struct dm_target *ti, struct request *rq,
    union map_info *map_context,
    struct request **clone)
    {
    return DM_MAPIO_KILL;
    }
    static void io_err_release_clone_rq(struct request *clone,
    union map_info *map_context)
    {
    }

#[no_mangle]
unsafe extern "C" fn io_err_map_sector(ti: *mut dm_target, bi_sector: sector_t) -> sector_t {
    static sector_t io_err_map_sector(struct dm_target *ti, sector_t bi_sector)
    {
    struct io_err_c *ioec = ti.private;
    return ioec.start + dm_target_offset(ti, bi_sector);
    }
    static int io_err_report_zones(struct dm_target *ti,
    struct dm_report_zones_args *args, unsigned int nr_zones)
    {
    struct io_err_c *ioec = ti.private;
//
// This should never be called when we do not have a backing device
// as that mean the target is not a zoned one.
//
    if (WARN_ON_ONCE(!ioec))
    return -EIO;
    return dm_report_zones(ioec.dev.bdev, ioec.start,
    io_err_map_sector(ti, args.next_sector),
    args, nr_zones);
    }

    static int io_err_iterate_devices(struct dm_target *ti,
    iterate_devices_callout_fn fn, void *data)
    {
    struct io_err_c *ioec = ti.private;
    if (!ioec)
    return 0;
    return fn(ti, ioec.dev, ioec.start, ti.len, data);
    }
#[no_mangle]
unsafe extern "C" fn io_err_io_hints(ti: *mut dm_target, limits: *mut queue_limits) {
    static void io_err_io_hints(struct dm_target *ti, struct queue_limits *limits)
    {
    limits.max_hw_discard_sectors = UINT_MAX;
    limits.discard_granularity = 512;
    }
    static long io_err_dax_direct_access(struct dm_target *ti, pgoff_t pgoff,
    long nr_pages, enum dax_access_mode mode, void **kaddr,
    unsigned long *pfn)
    {
    return -EIO;
    }
    static struct target_type error_target = {
    .name = "error",
    .version = {1, 7, 0},
    .features = DM_TARGET_WILDCARD | DM_TARGET_ZONED_HM |
    DM_TARGET_PASSES_INTEGRITY,
    .ctr  = io_err_ctr,
    .dtr  = io_err_dtr,
    .map  = io_err_map,
    .clone_and_map_rq = io_err_clone_and_map_rq,
    .release_clone_rq = io_err_release_clone_rq,
    .iterate_devices = io_err_iterate_devices,
    .io_hints = io_err_io_hints,
    .direct_access = io_err_dax_direct_access,
    .report_zones = io_err_report_zones,
    };
#[no_mangle]
pub unsafe extern "C" fn dm_target_init() -> int __init {
    int __init dm_target_init(void)
    {
    return dm_register_target(&error_target);
    }
#[no_mangle]
pub unsafe extern "C" fn dm_target_exit() {
    void dm_target_exit(void)
    {
    dm_unregister_target(&error_target);
    }

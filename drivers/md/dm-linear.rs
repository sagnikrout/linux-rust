//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-linear.c
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
// Copyright (C) 2001-2003 Sistina Software (UK) Limited.
//
// This file is released under the GPL.
//

//
// Linear: maps a linear range of a device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct linear_c {
    pub dev: *mut dm_dev,
    pub start: sector_t,
}

//
// Construct a linear mapping: <dev_path> <offset>
//
#[no_mangle]
unsafe extern "C" fn linear_ctr(ti: *mut dm_target, argc: c_uint, argv: *mut c_char) -> c_int {
    static int linear_ctr(struct dm_target *ti, unsigned int argc, char **argv)
    {
    struct linear_c *lc;
    unsigned long long tmp;
    char dummy;
    int ret;
    if (argc != 2) {
    ti.error = "Invalid argument count";
    return -EINVAL;
    }
    lc = kmalloc_obj(*lc);
    if (lc == core::ptr::null_mut()) {
    ti.error = "Cannot allocate linear context";
    return -ENOMEM;
    }
    ret = -EINVAL;
    if (sscanf(argv[1], "%llu%c", &tmp, &dummy) != 1 || tmp != (sector_t)tmp) {
    ti.error = "Invalid device sector";
    goto bad;
    }
    lc.start = tmp;
    ret = dm_get_device(ti, argv[0], dm_table_get_mode(ti.table), &lc.dev);
    if (ret) {
    ti.error = "Device lookup failed";
    goto bad;
    }
    ti.num_flush_bios = 1;
    ti.num_discard_bios = 1;
    ti.num_secure_erase_bios = 1;
    ti.num_write_zeroes_bios = 1;
    ti.flush_bypasses_map = true;
    ti.private = lc;
    return 0;
    bad:
    kfree(lc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn linear_dtr(ti: *mut dm_target) {
    static void linear_dtr(struct dm_target *ti)
    {
    struct linear_c *lc = ti.private;
    dm_put_device(ti, lc.dev);
    kfree(lc);
    }
#[no_mangle]
unsafe extern "C" fn linear_map_sector(ti: *mut dm_target, bi_sector: sector_t) -> sector_t {
    static sector_t linear_map_sector(struct dm_target *ti, sector_t bi_sector)
    {
    struct linear_c *lc = ti.private;
    return lc.start + dm_target_offset(ti, bi_sector);
    }
#[no_mangle]
pub unsafe extern "C" fn linear_map(ti: *mut dm_target, bio: *mut bio) -> c_int {
    int linear_map(struct dm_target *ti, struct bio *bio)
    {
    struct linear_c *lc = ti.private;
    bio_set_dev(bio, lc.dev.bdev);
    bio.bi_iter.bi_sector = linear_map_sector(ti, bio.bi_iter.bi_sector);
    return DM_MAPIO_REMAPPED;
    }
    static void linear_status(struct dm_target *ti, status_type_t type,
    unsigned int status_flags, char *result, unsigned int maxlen)
    {
    struct linear_c *lc = ti.private;
    let mut sz: usize = 0;
    switch (type) {
    case STATUSTYPE_INFO:
    result[0] = '\0';
    break;
    case STATUSTYPE_TABLE:
    DMEMIT("%s %llu", lc.dev.name, (unsigned long long)lc.start);
    break;
    case STATUSTYPE_IMA:
    DMEMIT_TARGET_NAME_VERSION(ti.type);
    DMEMIT(",device_name=%s,start=%llu;", lc.dev.name,
    (unsigned long long)lc.start);
    break;
    }
    }
    static int linear_prepare_ioctl(struct dm_target *ti, struct block_device **bdev,
    unsigned int cmd, unsigned long arg,
    bool *forward)
    {
    struct linear_c *lc = ti.private;
    struct dm_dev *dev = lc.dev;
// bdev = dev->bdev;
//
// Only pass ioctls through if the device sizes match exactly.
//
    if (lc.start || ti.len != bdev_nr_sectors(dev.bdev))
    return 1;
    return 0;
    }

    static int linear_report_zones(struct dm_target *ti,
    struct dm_report_zones_args *args, unsigned int nr_zones)
    {
    struct linear_c *lc = ti.private;
    return dm_report_zones(lc.dev.bdev, lc.start,
    linear_map_sector(ti, args.next_sector),
    args, nr_zones);
    }

    static int linear_iterate_devices(struct dm_target *ti,
    iterate_devices_callout_fn fn, void *data)
    {
    struct linear_c *lc = ti.private;
    return fn(ti, lc.dev, lc.start, ti.len, data);
    }

    static struct dax_device *linear_dax_pgoff(struct dm_target *ti, pgoff_t *pgoff)
    {
    struct linear_c *lc = ti.private;
    let mut sector: sector_t = linear_map_sector(ti, *pgoff << PAGE_SECTORS_SHIFT);
// pgoff = (get_start_sect(lc->dev->bdev) + sector) >> PAGE_SECTORS_SHIFT;
    return lc.dev.dax_dev;
    }
    static long linear_dax_direct_access(struct dm_target *ti, pgoff_t pgoff,
    long nr_pages, enum dax_access_mode mode, void **kaddr,
    unsigned long *pfn)
    {
    struct dax_device *dax_dev = linear_dax_pgoff(ti, &pgoff);
    return dax_direct_access(dax_dev, pgoff, nr_pages, mode, kaddr, pfn);
    }
    static int linear_dax_zero_page_range(struct dm_target *ti, pgoff_t pgoff,
    size_t nr_pages)
    {
    struct dax_device *dax_dev = linear_dax_pgoff(ti, &pgoff);
    return dax_zero_page_range(dax_dev, pgoff, nr_pages);
    }
    static size_t linear_dax_recovery_write(struct dm_target *ti, pgoff_t pgoff,
    void *addr, size_t bytes, struct iov_iter *i)
    {
    struct dax_device *dax_dev = linear_dax_pgoff(ti, &pgoff);
    return dax_recovery_write(dax_dev, pgoff, addr, bytes, i);
    }

    static struct target_type linear_target = {
    .name   = "linear",
    .version = {1, 5, 0},
    .features = DM_TARGET_PASSES_INTEGRITY | DM_TARGET_NOWAIT |
    DM_TARGET_ZONED_HM | DM_TARGET_PASSES_CRYPTO |
    DM_TARGET_ATOMIC_WRITES,
    .report_zones = linear_report_zones,
    .module = THIS_MODULE,
    .ctr    = linear_ctr,
    .dtr    = linear_dtr,
    .map    = linear_map,
    .status = linear_status,
    .prepare_ioctl = linear_prepare_ioctl,
    .iterate_devices = linear_iterate_devices,
    .direct_access = linear_dax_direct_access,
    .dax_zero_page_range = linear_dax_zero_page_range,
    .dax_recovery_write = linear_dax_recovery_write,
    };
#[no_mangle]
pub unsafe extern "C" fn dm_linear_init() -> int __init {
    int __init dm_linear_init(void)
    {
    let mut r: c_int = dm_register_target(&linear_target);
    if (r < 0)
    DMERR("register failed %d", r);
    return r;
    }
#[no_mangle]
pub unsafe extern "C" fn dm_linear_exit() {
    void dm_linear_exit(void)
    {
    dm_unregister_target(&linear_target);
    }

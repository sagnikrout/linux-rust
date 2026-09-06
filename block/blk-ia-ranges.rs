//! Automatically rewritten from C to Rust
//! Source: block/blk-ia-ranges.c
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
// Block device concurrent positioning ranges.
//
// Copyright (C) 2021 Western Digital Corporation or its Affiliates.
//

    static ssize_t
    blk_ia_range_sector_show(struct blk_independent_access_range *iar,
    char *buf)
    {
    return sprintf(buf, "%llu\n", iar.sector);
    }
    static ssize_t
    blk_ia_range_nr_sectors_show(struct blk_independent_access_range *iar,
    char *buf)
    {
    return sprintf(buf, "%llu\n", iar.nr_sectors);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_ia_range_sysfs_entry {
    pub attr: attribute,
    pub buf): *mut *mut *mut ssize_t (show)(struct blk_independent_access_range iar, char,
}

    static const struct blk_ia_range_sysfs_entry blk_ia_range_sector_entry = {
    .attr = { .name = "sector", .mode = 0444 },
    .show = blk_ia_range_sector_show,
    };
    static const struct blk_ia_range_sysfs_entry blk_ia_range_nr_sectors_entry = {
    .attr = { .name = "nr_sectors", .mode = 0444 },
    .show = blk_ia_range_nr_sectors_show,
    };
    static const struct attribute *const blk_ia_range_attrs[] = {
    &blk_ia_range_sector_entry.attr,
    &blk_ia_range_nr_sectors_entry.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(blk_ia_range);
    static ssize_t blk_ia_range_sysfs_show(struct kobject *kobj,
    struct attribute *attr, char *buf)
    {
    struct blk_ia_range_sysfs_entry *entry =
    container_of(attr, struct blk_ia_range_sysfs_entry, attr);
    struct blk_independent_access_range *iar =
    container_of(kobj, struct blk_independent_access_range, kobj);
    return entry.show(iar, buf);
    }
    static const struct sysfs_ops blk_ia_range_sysfs_ops = {
    .show	= blk_ia_range_sysfs_show,
    };
//
// Independent access range entries are not freed individually, but alltogether
// with struct blk_independent_access_ranges and its array of ranges. Since
// kobject_add() takes a reference on the parent kobject contained in
// struct blk_independent_access_ranges, the array of independent access range
// entries cannot be freed until kobject_del() is called for all entries.
// So we do not need to do anything here, but still need this no-op release
// operation to avoid complaints from the kobject code.
//
#[no_mangle]
unsafe extern "C" fn blk_ia_range_sysfs_nop_release(kobj: *mut kobject) {
    static void blk_ia_range_sysfs_nop_release(struct kobject *kobj)
    {
    }
    static const struct kobj_type blk_ia_range_ktype = {
    .sysfs_ops	= &blk_ia_range_sysfs_ops,
    .default_groups	= blk_ia_range_groups,
    .release	= blk_ia_range_sysfs_nop_release,
    };
//
// This will be executed only after all independent access range entries are
// removed with kobject_del(), at which point, it is safe to free everything,
// including the array of ranges.
//
#[no_mangle]
unsafe extern "C" fn blk_ia_ranges_sysfs_release(kobj: *mut kobject) {
    static void blk_ia_ranges_sysfs_release(struct kobject *kobj)
    {
    struct blk_independent_access_ranges *iars =
    container_of(kobj, struct blk_independent_access_ranges, kobj);
    kfree(iars);
    }
    static const struct kobj_type blk_ia_ranges_ktype = {
    .release	= blk_ia_ranges_sysfs_release,
    };
//
// disk_register_independent_access_ranges - register with sysfs a set of
// independent access ranges
// @disk:	Target disk
//
// Register with sysfs a set of independent access ranges for @disk.
//
#[no_mangle]
pub unsafe extern "C" fn disk_register_independent_access_ranges(disk: *mut gendisk) -> c_int {
    int disk_register_independent_access_ranges(struct gendisk *disk)
    {
    struct blk_independent_access_ranges *iars = disk.ia_ranges;
    struct request_queue *q = disk.queue;
    int i, ret;
    lockdep_assert_held(&q.sysfs_lock);
    if (!iars)
    return 0;
//
// At this point, iars is the new set of sector access ranges that needs
// to be registered with sysfs.
//
    WARN_ON(iars.sysfs_registered);
    ret = kobject_init_and_add(&iars.kobj, &blk_ia_ranges_ktype,
    &disk.queue_kobj, "%s",
    "independent_access_ranges");
    if (ret) {
    disk.ia_ranges = core::ptr::null_mut();
    kobject_put(&iars.kobj);
    return ret;
    }
    for (i = 0; i < iars.nr_ia_ranges; i++) {
    ret = kobject_init_and_add(&iars.ia_range[i].kobj,
    &blk_ia_range_ktype, &iars.kobj,
    "%d", i);
    if (ret) {
    while (--i >= 0)
    kobject_del(&iars.ia_range[i].kobj);
    kobject_del(&iars.kobj);
    kobject_put(&iars.kobj);
    return ret;
    }
    }
    iars.sysfs_registered = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn disk_unregister_independent_access_ranges(disk: *mut gendisk) {
    void disk_unregister_independent_access_ranges(struct gendisk *disk)
    {
    struct request_queue *q = disk.queue;
    struct blk_independent_access_ranges *iars = disk.ia_ranges;
    int i;
    lockdep_assert_held(&q.sysfs_lock);
    if (!iars)
    return;
    if (iars.sysfs_registered) {
    for (i = 0; i < iars.nr_ia_ranges; i++)
    kobject_del(&iars.ia_range[i].kobj);
    kobject_del(&iars.kobj);
    kobject_put(&iars.kobj);
    } else {
    kfree(iars);
    }
    disk.ia_ranges = core::ptr::null_mut();
    }
    static struct blk_independent_access_range *
    disk_find_ia_range(struct blk_independent_access_ranges *iars,
    sector_t sector)
    {
    struct blk_independent_access_range *iar;
    int i;
    for (i = 0; i < iars.nr_ia_ranges; i++) {
    iar = &iars.ia_range[i];
    if (sector >= iar.sector &&
    sector < iar.sector + iar.nr_sectors)
    return iar;
    }
    return core::ptr::null_mut();
    }
    static bool disk_check_ia_ranges(struct gendisk *disk,
    struct blk_independent_access_ranges *iars)
    {
    struct blk_independent_access_range *iar, *tmp;
    let mut capacity: sector_t = get_capacity(disk);
    let mut sector: sector_t = 0;
    int i;
    if (WARN_ON_ONCE(!iars.nr_ia_ranges))
    return false;
//
// While sorting the ranges in increasing LBA order, check that the
// ranges do not overlap, that there are no sector holes and that all
// sectors belong to one range.
//
    for (i = 0; i < iars.nr_ia_ranges; i++) {
    tmp = disk_find_ia_range(iars, sector);
    if (!tmp || tmp.sector != sector) {
    pr_warn("Invalid non-contiguous independent access ranges\n");
    return false;
    }
    iar = &iars.ia_range[i];
    if (tmp != iar) {
    swap(iar.sector, tmp.sector);
    swap(iar.nr_sectors, tmp.nr_sectors);
    }
    sector += iar.nr_sectors;
    }
    if (sector != capacity) {
    pr_warn("Independent access ranges do not match disk capacity\n");
    return false;
    }
    return true;
    }
    static bool disk_ia_ranges_changed(struct gendisk *disk,
    struct blk_independent_access_ranges *new)
    {
    struct blk_independent_access_ranges *old = disk.ia_ranges;
    int i;
    if (!old)
    return true;
    if (old.nr_ia_ranges != new.nr_ia_ranges)
    return true;
    for (i = 0; i < old.nr_ia_ranges; i++) {
    if (new.ia_range[i].sector != old.ia_range[i].sector ||
    new.ia_range[i].nr_sectors != old.ia_range[i].nr_sectors)
    return true;
    }
    return false;
    }
//
// disk_alloc_independent_access_ranges - Allocate an independent access ranges
// data structure
// @disk:		target disk
// @nr_ia_ranges:	Number of independent access ranges
//
// Allocate a struct blk_independent_access_ranges structure with @nr_ia_ranges
// access range descriptors.
//
    struct blk_independent_access_ranges *
    disk_alloc_independent_access_ranges(struct gendisk *disk, int nr_ia_ranges)
    {
    struct blk_independent_access_ranges *iars;
    iars = kzalloc_node(struct_size(iars, ia_range, nr_ia_ranges),
    GFP_KERNEL, disk.queue.node);
    if (iars)
    iars.nr_ia_ranges = nr_ia_ranges;
    return iars;
    }
    EXPORT_SYMBOL_GPL(disk_alloc_independent_access_ranges);
//
// disk_set_independent_access_ranges - Set a disk independent access ranges
// @disk:	target disk
// @iars:	independent access ranges structure
//
// Set the independent access ranges information of the request queue
// of @disk to @iars. If @iars is NULL and the independent access ranges
// structure already set is cleared. If there are no differences between
// @iars and the independent access ranges structure already set, @iars
// is freed.
//
    void disk_set_independent_access_ranges(struct gendisk *disk,
    struct blk_independent_access_ranges *iars)
    {
    struct request_queue *q = disk.queue;
    mutex_lock(&q.sysfs_lock);
    if (iars && !disk_check_ia_ranges(disk, iars)) {
    kfree(iars);
    iars = core::ptr::null_mut();
    }
    if (iars && !disk_ia_ranges_changed(disk, iars)) {
    kfree(iars);
    goto unlock;
    }
//
// This may be called for a registered queue. E.g. during a device
// revalidation. If that is the case, we need to unregister the old
// set of independent access ranges and register the new set. If the
// queue is not registered, registration of the device request queue
// will register the independent access ranges.
//
    disk_unregister_independent_access_ranges(disk);
    disk.ia_ranges = iars;
    if (blk_queue_registered(q))
    disk_register_independent_access_ranges(disk);
    unlock:
    mutex_unlock(&q.sysfs_lock);
    }
    EXPORT_SYMBOL_GPL(disk_set_independent_access_ranges);

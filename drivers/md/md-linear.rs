//! Automatically rewritten from C to Rust
//! Source: drivers/md/md-linear.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// linear.c : Multiple Devices driver for Linux Copyright (C) 1994-96 Marc
// ZYNGIER <zyngier@ufr-info-p7.ibp.fr> or <maz@gloups.fdn.fr>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_info {
    pub rdev: *mut md_rdev,
    pub end_sector: sector_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct linear_conf {
    pub rcu: rcu_head,
    pub array_sectors: sector_t,
// a copy of mddev->raid_disks
    pub raid_disks: c_int,
    pub __counted_by(raid_disks): dev_info disks[],
}

//
// find which device holds a particular offset
//
    static inline struct dev_info *which_dev(struct mddev *mddev, sector_t sector)
    {
    int lo, mid, hi;
    struct linear_conf *conf;
    lo = 0;
    hi = mddev.raid_disks - 1;
    conf = mddev.private;
//
// Binary Search
//
    while (hi > lo) {
    mid = (hi + lo) / 2;
    if (sector < conf.disks[mid].end_sector)
    hi = mid;
    else
    lo = mid + 1;
    }
    return conf.disks + lo;
    }
#[no_mangle]
unsafe extern "C" fn linear_size(mddev: *mut mddev, sectors: sector_t, raid_disks: c_int) -> sector_t {
    static sector_t linear_size(struct mddev *mddev, sector_t sectors, int raid_disks)
    {
    struct linear_conf *conf;
    sector_t array_sectors;
    conf = mddev.private;
    WARN_ONCE(sectors || raid_disks,
    "%s does not support generic reshape\n", __func__);
    array_sectors = conf.array_sectors;
    return array_sectors;
    }
#[no_mangle]
unsafe extern "C" fn linear_set_limits(mddev: *mut mddev) -> c_int {
    static int linear_set_limits(struct mddev *mddev)
    {
    struct queue_limits lim;
    int err;
    md_init_stacking_limits(&lim);
    lim.features |= BLK_FEAT_NOWAIT;
    lim.max_hw_sectors = mddev.chunk_sectors;
    lim.logical_block_size = mddev.logical_block_size;
    lim.max_write_zeroes_sectors = mddev.chunk_sectors;
    lim.max_hw_wzeroes_unmap_sectors = mddev.chunk_sectors;
    lim.io_min = mddev.chunk_sectors << 9;
    lim.features |= BLK_FEAT_ATOMIC_WRITES;
    err = mddev_stack_rdev_limits(mddev, &lim, MDDEV_STACK_INTEGRITY);
    if (err)
    return err;
    return queue_limits_set(mddev.gendisk.queue, &lim);
    }
    static struct linear_conf *linear_conf(struct mddev *mddev, int raid_disks)
    {
    struct linear_conf *conf;
    struct md_rdev *rdev;
    let mut ret: c_int = -EINVAL;
    int cnt;
    int i;
    conf = kzalloc_flex(*conf, disks, raid_disks);
    if (!conf)
    return ERR_PTR(-ENOMEM);
//
// conf->raid_disks is copy of mddev->raid_disks. The reason to
// keep a copy of mddev->raid_disks in struct linear_conf is,
// mddev->raid_disks may not be consistent with pointers number of
// conf->disks[] when it is updated in linear_add() and used to
// iterate old conf->disks[] earray in linear_congested().
// Here conf->raid_disks is always consitent with number of
// pointers in conf->disks[] array, and mddev->private is updated
// with rcu_assign_pointer() in linear_addr(), such race can be
// avoided.
//
    conf.raid_disks = raid_disks;
    cnt = 0;
    conf.array_sectors = 0;
    rdev_for_each(rdev, mddev) {
    let mut j: c_int = rdev.raid_disk;
    struct dev_info *disk = conf.disks + j;
    sector_t sectors;
    if (j < 0 || j >= raid_disks || disk.rdev) {
    pr_warn("md/linear:%s: disk numbering problem. Aborting!\n",
    mdname(mddev));
    goto out;
    }
    disk.rdev = rdev;
    if (mddev.chunk_sectors) {
    sectors = rdev.sectors;
    sector_div(sectors, mddev.chunk_sectors);
    rdev.sectors = sectors * mddev.chunk_sectors;
    }
    conf.array_sectors += rdev.sectors;
    cnt++;
    }
    if (cnt != raid_disks) {
    pr_warn("md/linear:%s: not enough drives present. Aborting!\n",
    mdname(mddev));
    goto out;
    }
//
// Here we calculate the device offsets.
//
    conf.disks[0].end_sector = conf.disks[0].rdev.sectors;
    for (i = 1; i < raid_disks; i++)
    conf.disks[i].end_sector =
    conf.disks[i-1].end_sector +
    conf.disks[i].rdev.sectors;
    if (!mddev_is_dm(mddev)) {
    ret = linear_set_limits(mddev);
    if (ret)
    goto out;
    }
    return conf;
    out:
    kfree(conf);
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn linear_run(mddev: *mut mddev) -> c_int {
    static int linear_run(struct mddev *mddev)
    {
    struct linear_conf *conf;
    int ret;
    if (md_check_no_bitmap(mddev))
    return -EINVAL;
    conf = linear_conf(mddev, mddev.raid_disks);
    if (IS_ERR(conf))
    return PTR_ERR(conf);
    mddev.private = conf;
    md_set_array_sectors(mddev, linear_size(mddev, 0, 0));
    ret =  md_integrity_register(mddev);
    if (ret) {
    kfree(conf);
    mddev.private = core::ptr::null_mut();
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn linear_add(mddev: *mut mddev, rdev: *mut md_rdev) -> c_int {
    static int linear_add(struct mddev *mddev, struct md_rdev *rdev)
    {
// Adding a drive to a linear array allows the array to grow.
// It is permitted if the new drive has a matching superblock
// already on it, with raid_disk equal to raid_disks.
// It is achieved by creating a new linear_private_data structure
// and swapping it in in-place of the current one.
// The current one is never freed until the array is stopped.
// This avoids races.
//
    struct linear_conf *newconf, *oldconf;
    if (rdev.saved_raid_disk != mddev.raid_disks)
    return -EINVAL;
    rdev.raid_disk = rdev.saved_raid_disk;
    rdev.saved_raid_disk = -1;
    newconf = linear_conf(mddev, mddev.raid_disks + 1);
    if (IS_ERR(newconf))
    return PTR_ERR(newconf);
// newconf->raid_disks already keeps a copy of * the increased
// value of mddev->raid_disks, WARN_ONCE() is just used to make
// sure of this. It is possible that oldconf is still referenced
// in linear_congested(), therefore kfree_rcu() is used to free
// oldconf until no one uses it anymore.
//
    oldconf = rcu_dereference_protected(mddev.private,
    lockdep_is_held(&mddev.reconfig_mutex));
    mddev.raid_disks++;
    WARN_ONCE(mddev.raid_disks != newconf.raid_disks,
    "copied raid_disks doesn't match mddev.raid_disks");
    rcu_assign_pointer(mddev.private, newconf);
    md_set_array_sectors(mddev, linear_size(mddev, 0, 0));
    set_capacity_and_notify(mddev.gendisk, mddev.array_sectors);
    kfree_rcu(oldconf, rcu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn linear_free(mddev: *mut mddev, priv: *mut c_void) {
    static void linear_free(struct mddev *mddev, void *priv)
    {
    struct linear_conf *conf = priv;
    kfree(conf);
    }
#[no_mangle]
unsafe extern "C" fn linear_make_request(mddev: *mut mddev, bio: *mut bio) -> bool {
    static bool linear_make_request(struct mddev *mddev, struct bio *bio)
    {
    struct dev_info *tmp_dev;
    sector_t start_sector, end_sector, data_offset;
    let mut bio_sector: sector_t = bio.bi_iter.bi_sector;
    if (unlikely(bio.bi_opf & REQ_PREFLUSH)
    && md_flush_request(mddev, bio))
    return true;
    tmp_dev = which_dev(mddev, bio_sector);
    start_sector = tmp_dev.end_sector - tmp_dev.rdev.sectors;
    end_sector = tmp_dev.end_sector;
    data_offset = tmp_dev.rdev.data_offset;
    if (unlikely(bio_sector >= end_sector ||
    bio_sector < start_sector))
    goto out_of_bounds;
    if (unlikely(is_rdev_broken(tmp_dev.rdev))) {
    md_error(mddev, tmp_dev.rdev);
    bio_io_error(bio);
    return true;
    }
    if (unlikely(bio_end_sector(bio) > end_sector)) {
// This bio crosses a device boundary, so we have to split it
    bio = bio_submit_split_bioset(bio, end_sector - bio_sector,
    &mddev.bio_set);
    if (!bio)
    return true;
    }
    md_account_bio(mddev, &bio);
    bio_set_dev(bio, tmp_dev.rdev.bdev);
    bio.bi_iter.bi_sector = bio.bi_iter.bi_sector -
    start_sector + data_offset;
    if (unlikely((bio_op(bio) == REQ_OP_DISCARD) &&
    !bdev_max_discard_sectors(bio.bi_bdev))) {
// Just ignore it
    bio_endio(bio);
    } else {
    if (mddev.gendisk)
    trace_block_bio_remap(bio, disk_devt(mddev.gendisk),
    bio_sector);
    mddev_check_write_zeroes(mddev, bio);
    submit_bio_noacct(bio);
    }
    return true;
    out_of_bounds:
    pr_err("md/linear:%s: make_request: Sector %llu out of bounds on dev %pg: %llu sectors, offset %llu\n",
    mdname(mddev),
    (unsigned long long)bio.bi_iter.bi_sector,
    tmp_dev.rdev.bdev,
    (unsigned long long)tmp_dev.rdev.sectors,
    (unsigned long long)start_sector);
    bio_io_error(bio);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn linear_status(seq: *mut seq_file, mddev: *mut mddev) {
    static void linear_status(struct seq_file *seq, struct mddev *mddev)
    {
    seq_printf(seq, " %dk rounding", mddev.chunk_sectors / 2);
    }
#[no_mangle]
unsafe extern "C" fn linear_error(mddev: *mut mddev, rdev: *mut md_rdev) {
    static void linear_error(struct mddev *mddev, struct md_rdev *rdev)
    {
    if (!test_and_set_bit(MD_BROKEN, &mddev.flags)) {
    char *md_name = mdname(mddev);
    pr_crit("md/linear%s: Disk failure on %pg detected, failing array.\n",
    md_name, rdev.bdev);
    }
    }
#[no_mangle]
unsafe extern "C" fn linear_quiesce(mddev: *mut mddev, state: c_int) {
    static void linear_quiesce(struct mddev *mddev, int state)
    {
    }
    static struct md_personality linear_personality = {
    .head = {
    .type	= MD_PERSONALITY,
    .id	= ID_LINEAR,
    .name	= "linear",
    .owner	= THIS_MODULE,
    },
    .make_request	= linear_make_request,
    .run		= linear_run,
    .free		= linear_free,
    .status		= linear_status,
    .hot_add_disk	= linear_add,
    .size		= linear_size,
    .quiesce	= linear_quiesce,
    .error_handler	= linear_error,
    };
#[no_mangle]
unsafe extern "C" fn linear_init() -> int __init {
    static int __init linear_init(void)
    {
    return register_md_submodule(&linear_personality.head);
    }
#[no_mangle]
unsafe extern "C" fn linear_exit() {
    static void linear_exit(void)
    {
    unregister_md_submodule(&linear_personality.head);
    }
    module_init(linear_init);
    module_exit(linear_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Linear device concatenation personality for MD (deprecated)");
    MODULE_ALIAS("md-personality-1"); /* LINEAR - deprecated*/
    MODULE_ALIAS("md-linear");
    MODULE_ALIAS("md-level--1");

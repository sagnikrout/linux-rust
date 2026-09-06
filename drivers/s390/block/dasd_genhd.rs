//! Automatically rewritten from C to Rust
//! Source: drivers/s390/block/dasd_genhd.c
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
// Author(s)......: Holger Smolinski <Holger.Smolinski@de.ibm.com>
// Horst Hummel <Horst.Hummel@de.ibm.com>
// Carsten Otte <Cotte@de.ibm.com>
// Martin Schwidefsky <schwidefsky@de.ibm.com>
// Bugreports.to..: <Linux390@de.ibm.com>
// Copyright IBM Corp. 1999, 2001
//
// gendisk related functions for the dasd driver.
//

    let mut queue_depth: static unsigned int = 32;
    let mut nr_hw_queues: static unsigned int = 4;
    static void dasd_gd_free(struct gendisk *gdp);
    module_param(queue_depth, uint, 0444);
    MODULE_PARM_DESC(queue_depth, "Default queue depth for new DASD devices");
    module_param(nr_hw_queues, uint, 0444);
    MODULE_PARM_DESC(nr_hw_queues, "Default number of hardware queues for new DASD devices");
//
// Set device name.
// dasda - dasdz : 26 devices
// dasdaa - dasdzz : 676 devices, added up = 702
// dasdaaa - dasdzzz : 17576 devices, added up = 18278
// dasdaaaa - dasdzzzz : 456976 devices, added up = 475252
//
#[no_mangle]
unsafe extern "C" fn dasd_name_format(prefix: *mut c_char, index: c_int, buf: *mut c_char, buflen: c_int) -> c_int {
    static int dasd_name_format(char *prefix, int index, char *buf, int buflen)
    {
    let mut base: c_int = 'z' - 'a' + 1;
    char *begin = buf + strlen(prefix);
    char *end = buf + buflen;
    char *p;
    int unit;
    p = end - 1;
// p = '\0';
    unit = base;
    do {
    if (p == begin)
    return -EINVAL;
// --p = 'a' + (index % unit);
    index = (index / unit) - 1;
    } while (index >= 0);
    memmove(begin, p, end - p);
    memcpy(buf, prefix, strlen(prefix));
    return 0;
    }
//
// Allocate and register gendisk structure for device.
//
#[no_mangle]
pub unsafe extern "C" fn dasd_gendisk_alloc(block: *mut dasd_block) -> c_int {
    int dasd_gendisk_alloc(struct dasd_block *block)
    {
    struct queue_limits lim = {
//
// With page sized segments, each segment can be translated into
// one idaw/tidaw.
//
    .max_segment_size = PAGE_SIZE,
    .seg_boundary_mask = PAGE_SIZE - 1,
    .max_segments = USHRT_MAX,
    };
    struct gendisk *gdp;
    struct dasd_device *base;
    unsigned int devindex;
    int rc;
// Make sure the minor for this device exists.
    base = block.base;
    devindex = base.devindex;
    if (devindex >= DASD_PER_MAJOR)
    return -EBUSY;
    block.tag_set.ops = &dasd_mq_ops;
    block.tag_set.cmd_size = sizeof(struct dasd_ccw_req);
    block.tag_set.nr_hw_queues = nr_hw_queues;
    block.tag_set.queue_depth = queue_depth;
    block.tag_set.numa_node = NUMA_NO_NODE;
    rc = blk_mq_alloc_tag_set(&block.tag_set);
    if (rc)
    return rc;
    gdp = blk_mq_alloc_disk(&block.tag_set, &lim, block);
    if (IS_ERR(gdp)) {
    blk_mq_free_tag_set(&block.tag_set);
    return PTR_ERR(gdp);
    }
// Initialize gendisk structure.
    gdp.major = DASD_MAJOR;
    gdp.first_minor = devindex << DASD_PARTN_BITS;
    gdp.minors = 1 << DASD_PARTN_BITS;
    gdp.fops = &dasd_device_operations;
    rc = dasd_name_format("dasd", devindex, gdp.disk_name, sizeof(gdp.disk_name));
    if (rc) {
    DBF_DEV_EVENT(DBF_ERR, block.base,
    "setting disk name failed, rc %d", rc);
    dasd_gd_free(gdp);
    return rc;
    }
    if (base.features & DASD_FEATURE_READONLY ||
    test_bit(DASD_FLAG_DEVICE_RO, &base.flags))
    set_disk_ro(gdp, 1);
    dasd_add_link_to_gendisk(gdp, base);
    block.gdp = gdp;
    set_capacity(block.gdp, 0);
    rc = device_add_disk(&base.cdev.dev, block.gdp, core::ptr::null_mut());
    if (rc) {
    dasd_gendisk_free(block);
    return rc;
    }
    return 0;
    }
//
// Free gendisk structure
//
#[no_mangle]
unsafe extern "C" fn dasd_gd_free(gd: *mut gendisk) {
    static void dasd_gd_free(struct gendisk *gd)
    {
    del_gendisk(gd);
    gd.private_data = core::ptr::null_mut();
    put_disk(gd);
    }
//
// Unregister and free gendisk structure for device.
//
#[no_mangle]
pub unsafe extern "C" fn dasd_gendisk_free(block: *mut dasd_block) {
    void dasd_gendisk_free(struct dasd_block *block)
    {
    if (block.gdp) {
    dasd_gd_free(block.gdp);
    block.gdp = core::ptr::null_mut();
    blk_mq_free_tag_set(&block.tag_set);
    }
    }
//
// Trigger a partition detection.
//
#[no_mangle]
pub unsafe extern "C" fn dasd_scan_partitions(block: *mut dasd_block) -> c_int {
    int dasd_scan_partitions(struct dasd_block *block)
    {
    struct file *bdev_file;
    int rc;
    bdev_file = bdev_file_open_by_dev(disk_devt(block.gdp), BLK_OPEN_READ,
    core::ptr::null_mut(), core::ptr::null_mut());
    if (IS_ERR(bdev_file)) {
    DBF_DEV_EVENT(DBF_ERR, block.base,
    "scan partitions error, blkdev_get returned %ld",
    PTR_ERR(bdev_file));
    return -ENODEV;
    }
    mutex_lock(&block.gdp.open_mutex);
    rc = bdev_disk_changed(block.gdp, false);
    mutex_unlock(&block.gdp.open_mutex);
    if (rc)
    DBF_DEV_EVENT(DBF_ERR, block.base,
    "scan partitions error, rc %d", rc);
//
// Since the matching fput() call to the
// bdev_file_open_by_path() in this function is not called before
// dasd_destroy_partitions the offline open_count limit needs to be
// increased from 0 to 1. This is done by setting device->bdev_file
// (see dasd_generic_set_offline). As long as the partition detection
// is running no offline should be allowed. That is why the assignment
// to block->bdev_file is done AFTER the BLKRRPART ioctl.
//
    block.bdev_file = bdev_file;
    return 0;
    }
//
// Remove all inodes in the system for a device, delete the
// partitions and make device unusable by setting its size to zero.
//
#[no_mangle]
pub unsafe extern "C" fn dasd_destroy_partitions(block: *mut dasd_block) {
    void dasd_destroy_partitions(struct dasd_block *block)
    {
    struct file *bdev_file;
//
// Get the bdev_file pointer from the device structure and clear
// device->bdev_file to lower the offline open_count limit again.
//
    bdev_file = block.bdev_file;
    block.bdev_file = core::ptr::null_mut();
    mutex_lock(&file_bdev(bdev_file).bd_disk.open_mutex);
    bdev_disk_changed(file_bdev(bdev_file).bd_disk, true);
    mutex_unlock(&file_bdev(bdev_file).bd_disk.open_mutex);
// Matching blkdev_put to the blkdev_get in dasd_scan_partitions.
    fput(bdev_file);
    }
#[no_mangle]
pub unsafe extern "C" fn dasd_gendisk_init() -> c_int {
    int dasd_gendisk_init(void)
    {
    int rc;
// Register to static dasd major 94
    rc = register_blkdev(DASD_MAJOR, "dasd");
    if (rc != 0) {
    pr_warn("Registering the device driver with major number %d failed\n",
    DASD_MAJOR);
    return rc;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dasd_gendisk_exit() {
    void dasd_gendisk_exit(void)
    {
    unregister_blkdev(DASD_MAJOR, "dasd");
    }

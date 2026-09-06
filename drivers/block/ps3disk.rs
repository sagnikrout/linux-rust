//! Automatically rewritten from C to Rust
//! Source: drivers/block/ps3disk.c
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
// PS3 Disk Storage Driver
//
// Copyright (C) 2007 Sony Computer Entertainment Inc.
// Copyright 2007 Sony Corp.
//

pub const PS3DISK_MAX_DISKS: c_int = 16;
pub const PS3DISK_MINORS: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3disk_private {
    pub /: *mut *mut spinlock_t lock; / Request queue spinlock,
    pub tag_set: blk_mq_tag_set,
    pub gendisk: *mut gendisk,
    pub blocking_factor: c_uint,
    pub req: *mut request,
    pub raw_capacity: u64,
    pub model: [c_uchar; ATA_ID_PROD_LEN+1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lv1_ata_cmnd_block {
    pub features: u16,
    pub sector_count: u16,
    pub LBA_low: u16,
    pub LBA_mid: u16,
    pub LBA_high: u16,
    pub device: u8,
    pub command: u8,
    pub is_ext: u32,
    pub proto: u32,
    pub in_out: u32,
    pub size: u32,
    pub buffer: u64,
    pub arglen: u32,
}

    enum lv1_ata_proto {
    NON_DATA_PROTO     = 0,
    PIO_DATA_IN_PROTO  = 1,
    PIO_DATA_OUT_PROTO = 2,
    DMA_PROTO = 3
    };
    enum lv1_ata_in_out {
    DIR_WRITE = 0,			/* memory . device */
    DIR_READ = 1			/* device . memory */
    };
    static int ps3disk_major;
    static const struct block_device_operations ps3disk_fops = {
    .owner		= THIS_MODULE,
    };
    static void ps3disk_scatter_gather(struct ps3_storage_device *dev,
    struct request *req, int gather)
    {
    let mut offset: c_uint = 0;
    struct req_iterator iter;
    struct bio_vec bvec;
    rq_for_each_segment(bvec, req, iter) {
    dev_dbg(&dev.sbd.core, "%s:%u: %u sectors from %llu\n",
    __func__, __LINE__, bio_sectors(iter.bio),
    iter.bio.bi_iter.bi_sector);
    if (gather)
    memcpy_from_bvec(dev.bounce_buf + offset, &bvec);
    else
    memcpy_to_bvec(&bvec, dev.bounce_buf + offset);
    offset += bvec.bv_len;
    }
    }
    static blk_status_t ps3disk_submit_request_sg(struct ps3_storage_device *dev,
    struct request *req)
    {
    struct ps3disk_private *priv = ps3_system_bus_get_drvdata(&dev.sbd);
    let mut write: c_int = rq_data_dir(req), res;
    const char *op = write ? "write" : "read";
    u64 start_sector, sectors;
    let mut region_id: c_uint = dev.regions[dev.region_idx].id;

    let mut n: c_uint = 0;
    struct bio_vec bv;
    struct req_iterator iter;
    rq_for_each_segment(bv, req, iter)
    n++;
    dev_dbg(&dev.sbd.core,
    "%s:%u: %s req has %u bvecs for %u sectors\n",
    __func__, __LINE__, op, n, blk_rq_sectors(req));

    start_sector = blk_rq_pos(req) * priv.blocking_factor;
    sectors = blk_rq_sectors(req) * priv.blocking_factor;
    dev_dbg(&dev.sbd.core, "%s:%u: %s %llu sectors starting at %llu\n",
    __func__, __LINE__, op, sectors, start_sector);
    if (write) {
    ps3disk_scatter_gather(dev, req, 1);
    res = lv1_storage_write(dev.sbd.dev_id, region_id,
    start_sector, sectors, 0,
    dev.bounce_lpar, &dev.tag);
    } else {
    res = lv1_storage_read(dev.sbd.dev_id, region_id,
    start_sector, sectors, 0,
    dev.bounce_lpar, &dev.tag);
    }
    if (res) {
    dev_err(&dev.sbd.core, "%s:%u: %s failed %d\n", __func__,
    __LINE__, op, res);
    return BLK_STS_IOERR;
    }
    priv.req = req;
    return BLK_STS_OK;
    }
    static blk_status_t ps3disk_submit_flush_request(struct ps3_storage_device *dev,
    struct request *req)
    {
    struct ps3disk_private *priv = ps3_system_bus_get_drvdata(&dev.sbd);
    u64 res;
    dev_dbg(&dev.sbd.core, "%s:%u: flush request\n", __func__, __LINE__);
    res = lv1_storage_send_device_command(dev.sbd.dev_id,
    LV1_STORAGE_ATA_HDDOUT, 0, 0, 0,
    0, &dev.tag);
    if (res) {
    dev_err(&dev.sbd.core, "%s:%u: sync cache failed 0x%llx\n",
    __func__, __LINE__, res);
    return BLK_STS_IOERR;
    }
    priv.req = req;
    return BLK_STS_OK;
    }
    static blk_status_t ps3disk_do_request(struct ps3_storage_device *dev,
    struct request *req)
    {
    dev_dbg(&dev.sbd.core, "%s:%u\n", __func__, __LINE__);
    switch (req_op(req)) {
    case REQ_OP_FLUSH:
    return ps3disk_submit_flush_request(dev, req);
    case REQ_OP_READ:
    case REQ_OP_WRITE:
    return ps3disk_submit_request_sg(dev, req);
    default:
    blk_dump_rq_flags(req, DEVICE_NAME " bad request");
    return BLK_STS_IOERR;
    }
    }
    static blk_status_t ps3disk_queue_rq(struct blk_mq_hw_ctx *hctx,
    const struct blk_mq_queue_data *bd)
    {
    struct request_queue *q = hctx.queue;
    struct ps3_storage_device *dev = q.queuedata;
    struct ps3disk_private *priv = ps3_system_bus_get_drvdata(&dev.sbd);
    blk_status_t ret;
    blk_mq_start_request(bd.rq);
    spin_lock_irq(&priv.lock);
    ret = ps3disk_do_request(dev, bd.rq);
    spin_unlock_irq(&priv.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ps3disk_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ps3disk_interrupt(int irq, void *data)
    {
    struct ps3_storage_device *dev = data;
    struct ps3disk_private *priv;
    struct request *req;
    int res, read;
    blk_status_t error;
    u64 tag, status;
    const char *op;
    res = lv1_storage_get_async_status(dev.sbd.dev_id, &tag, &status);
    if (tag != dev.tag)
    dev_err(&dev.sbd.core,
    "%s:%u: tag mismatch, got %llx, expected %llx\n",
    __func__, __LINE__, tag, dev.tag);
    if (res) {
    dev_err(&dev.sbd.core, "%s:%u: res=%d status=0x%llx\n",
    __func__, __LINE__, res, status);
    return IRQ_HANDLED;
    }
    priv = ps3_system_bus_get_drvdata(&dev.sbd);
    req = priv.req;
    if (!req) {
    dev_dbg(&dev.sbd.core,
    "%s:%u non-block layer request completed\n", __func__,
    __LINE__);
    dev.lv1_status = status;
    complete(&dev.done);
    return IRQ_HANDLED;
    }
    if (req_op(req) == REQ_OP_FLUSH) {
    read = 0;
    op = "flush";
    } else {
    read = !rq_data_dir(req);
    op = read ? "read" : "write";
    }
    if (status) {
    dev_dbg(&dev.sbd.core, "%s:%u: %s failed 0x%llx\n", __func__,
    __LINE__, op, status);
    error = BLK_STS_IOERR;
    } else {
    dev_dbg(&dev.sbd.core, "%s:%u: %s completed\n", __func__,
    __LINE__, op);
    error = 0;
    if (read)
    ps3disk_scatter_gather(dev, req, 0);
    }
    spin_lock(&priv.lock);
    priv.req = core::ptr::null_mut();
    blk_mq_end_request(req, error);
    spin_unlock(&priv.lock);
    blk_mq_run_hw_queues(priv.gendisk.queue, true);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ps3disk_sync_cache(dev: *mut ps3_storage_device) -> c_int {
    static int ps3disk_sync_cache(struct ps3_storage_device *dev)
    {
    u64 res;
    dev_dbg(&dev.sbd.core, "%s:%u: sync cache\n", __func__, __LINE__);
    res = ps3stor_send_command(dev, LV1_STORAGE_ATA_HDDOUT, 0, 0, 0, 0);
    if (res) {
    dev_err(&dev.sbd.core, "%s:%u: sync cache failed 0x%llx\n",
    __func__, __LINE__, res);
    return -EIO;
    }
    return 0;
    }
// ATA helpers copied from drivers/ata/libata-core.c
#[no_mangle]
unsafe extern "C" fn swap_buf_le16(buf: *mut u16, buf_words: c_uint) {
    static void swap_buf_le16(u16 *buf, unsigned int buf_words)
    {

    unsigned int i;
    for (i = 0; i < buf_words; i++)
    buf[i] = le16_to_cpu(buf[i]);

    }
#[no_mangle]
unsafe extern "C" fn ata_id_n_sectors(id: *const u16) -> u64 {
    static u64 ata_id_n_sectors(const u16 *id)
    {
    if (ata_id_has_lba(id)) {
    if (ata_id_has_lba48(id))
    return ata_id_u64(id, 100);
    else
    return ata_id_u32(id, 60);
    } else {
    if (ata_id_current_chs_valid(id))
    return ata_id_u32(id, 57);
    else
    return id[1] * id[3] * id[6];
    }
    }
    static void ata_id_string(const u16 *id, unsigned char *s, unsigned int ofs,
    unsigned int len)
    {
    unsigned int c;
    while (len > 0) {
    c = id[ofs] >> 8;
// s = c;
    s++;
    c = id[ofs] & 0xff;
// s = c;
    s++;
    ofs++;
    len -= 2;
    }
    }
    static void ata_id_c_string(const u16 *id, unsigned char *s, unsigned int ofs,
    unsigned int len)
    {
    unsigned char *p;
    WARN_ON(!(len & 1));
    ata_id_string(id, s, ofs, len - 1);
    p = s + strnlen(s, len - 1);
    while (p > s && p[-1] == ' ')
    p--;
// p = '\0';
    }
#[no_mangle]
unsafe extern "C" fn ps3disk_identify(dev: *mut ps3_storage_device) -> c_int {
    static int ps3disk_identify(struct ps3_storage_device *dev)
    {
    struct ps3disk_private *priv = ps3_system_bus_get_drvdata(&dev.sbd);
    struct lv1_ata_cmnd_block ata_cmnd;
    u16 *id = dev.bounce_buf;
    u64 res;
    dev_dbg(&dev.sbd.core, "%s:%u: identify disk\n", __func__, __LINE__);
    memset(&ata_cmnd, 0, sizeof(struct lv1_ata_cmnd_block));
    ata_cmnd.command = ATA_CMD_ID_ATA;
    ata_cmnd.sector_count = 1;
    ata_cmnd.size = ata_cmnd.arglen = ATA_ID_WORDS * 2;
    ata_cmnd.buffer = dev.bounce_lpar;
    ata_cmnd.proto = PIO_DATA_IN_PROTO;
    ata_cmnd.in_out = DIR_READ;
    res = ps3stor_send_command(dev, LV1_STORAGE_SEND_ATA_COMMAND,
    ps3_mm_phys_to_lpar(__pa(&ata_cmnd)),
    sizeof(ata_cmnd), ata_cmnd.buffer,
    ata_cmnd.arglen);
    if (res) {
    dev_err(&dev.sbd.core, "%s:%u: identify disk failed 0x%llx\n",
    __func__, __LINE__, res);
    return -EIO;
    }
    swap_buf_le16(id, ATA_ID_WORDS);
// All we're interested in are raw capacity and model name
    priv.raw_capacity = ata_id_n_sectors(id);
    ata_id_c_string(id, priv.model, ATA_ID_PROD, sizeof(priv.model));
    return 0;
    }
    static unsigned long ps3disk_mask;
    static DEFINE_MUTEX(ps3disk_mask_mutex);
    static const struct blk_mq_ops ps3disk_mq_ops = {
    .queue_rq	= ps3disk_queue_rq,
    };
#[no_mangle]
unsafe extern "C" fn ps3disk_probe(_dev: *mut ps3_system_bus_device) -> c_int {
    static int ps3disk_probe(struct ps3_system_bus_device *_dev)
    {
    struct ps3_storage_device *dev = to_ps3_storage_device(&_dev.core);
    struct ps3disk_private *priv;
    int error;
    unsigned int devidx;
    struct queue_limits lim = {
    .logical_block_size	= dev.blk_size,
    .max_hw_sectors		= BOUNCE_SIZE >> 9,
    .max_segments		= -1,
    .max_segment_size	= BOUNCE_SIZE,
    .dma_alignment		= dev.blk_size - 1,
    .features		= BLK_FEAT_WRITE_CACHE |
    BLK_FEAT_ROTATIONAL,
    };
    struct gendisk *gendisk;
    if (dev.blk_size < 512) {
    dev_err(&dev.sbd.core,
    "%s:%u: cannot handle block size %llu\n", __func__,
    __LINE__, dev.blk_size);
    return -EINVAL;
    }
    BUILD_BUG_ON(PS3DISK_MAX_DISKS > BITS_PER_LONG);
    mutex_lock(&ps3disk_mask_mutex);
    devidx = find_first_zero_bit(&ps3disk_mask, PS3DISK_MAX_DISKS);
    if (devidx >= PS3DISK_MAX_DISKS) {
    dev_err(&dev.sbd.core, "%s:%u: Too many disks\n", __func__,
    __LINE__);
    mutex_unlock(&ps3disk_mask_mutex);
    return -ENOSPC;
    }
    __set_bit(devidx, &ps3disk_mask);
    mutex_unlock(&ps3disk_mask_mutex);
    priv = kzalloc_obj(*priv);
    if (!priv) {
    error = -ENOMEM;
    goto fail;
    }
    ps3_system_bus_set_drvdata(_dev, priv);
    spin_lock_init(&priv.lock);
    dev.bounce_size = BOUNCE_SIZE;
    dev.bounce_buf = kmalloc(BOUNCE_SIZE, GFP_DMA);
    if (!dev.bounce_buf) {
    error = -ENOMEM;
    goto fail_free_priv;
    }
    error = ps3stor_setup(dev, ps3disk_interrupt);
    if (error)
    goto fail_free_bounce;
    ps3disk_identify(dev);
    error = blk_mq_alloc_sq_tag_set(&priv.tag_set, &ps3disk_mq_ops, 1, 0);
    if (error)
    goto fail_teardown;
    gendisk = blk_mq_alloc_disk(&priv.tag_set, &lim, dev);
    if (IS_ERR(gendisk)) {
    dev_err(&dev.sbd.core, "%s:%u: blk_mq_alloc_disk failed\n",
    __func__, __LINE__);
    error = PTR_ERR(gendisk);
    goto fail_free_tag_set;
    }
    priv.gendisk = gendisk;
    gendisk.major = ps3disk_major;
    gendisk.first_minor = devidx * PS3DISK_MINORS;
    gendisk.minors = PS3DISK_MINORS;
    gendisk.fops = &ps3disk_fops;
    gendisk.private_data = dev;
    snprintf(gendisk.disk_name, sizeof(gendisk.disk_name), PS3DISK_NAME,
    devidx+'a');
    priv.blocking_factor = dev.blk_size >> 9;
    set_capacity(gendisk,
    dev.regions[dev.region_idx].size*priv.blocking_factor);
    dev_info(&dev.sbd.core,
    "%s is a %s (%llu MiB total, %llu MiB for OtherOS)\n",
    gendisk.disk_name, priv.model, priv.raw_capacity >> 11,
    get_capacity(gendisk) >> 11);
    error = device_add_disk(&dev.sbd.core, gendisk, core::ptr::null_mut());
    if (error)
    goto fail_cleanup_disk;
    return 0;
    fail_cleanup_disk:
    put_disk(gendisk);
    fail_free_tag_set:
    blk_mq_free_tag_set(&priv.tag_set);
    fail_teardown:
    ps3stor_teardown(dev);
    fail_free_bounce:
    kfree(dev.bounce_buf);
    fail_free_priv:
    kfree(priv);
    ps3_system_bus_set_drvdata(_dev, core::ptr::null_mut());
    fail:
    mutex_lock(&ps3disk_mask_mutex);
    __clear_bit(devidx, &ps3disk_mask);
    mutex_unlock(&ps3disk_mask_mutex);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn ps3disk_remove(_dev: *mut ps3_system_bus_device) {
    static void ps3disk_remove(struct ps3_system_bus_device *_dev)
    {
    struct ps3_storage_device *dev = to_ps3_storage_device(&_dev.core);
    struct ps3disk_private *priv = ps3_system_bus_get_drvdata(&dev.sbd);
    mutex_lock(&ps3disk_mask_mutex);
    __clear_bit(MINOR(disk_devt(priv.gendisk)) / PS3DISK_MINORS,
    &ps3disk_mask);
    mutex_unlock(&ps3disk_mask_mutex);
    del_gendisk(priv.gendisk);
    put_disk(priv.gendisk);
    blk_mq_free_tag_set(&priv.tag_set);
    dev_notice(&dev.sbd.core, "Synchronizing disk cache\n");
    ps3disk_sync_cache(dev);
    ps3stor_teardown(dev);
    kfree(dev.bounce_buf);
    kfree(priv);
    ps3_system_bus_set_drvdata(_dev, core::ptr::null_mut());
    }
    static struct ps3_system_bus_driver ps3disk = {
    .match_id	= PS3_MATCH_ID_STOR_DISK,
    .core.name	= DEVICE_NAME,
    .core.owner	= THIS_MODULE,
    .probe		= ps3disk_probe,
    .remove		= ps3disk_remove,
    .shutdown	= ps3disk_remove,
    };
#[no_mangle]
unsafe extern "C" fn ps3disk_init() -> int __init {
    static int __init ps3disk_init(void)
    {
    int error;
    if (!firmware_has_feature(FW_FEATURE_PS3_LV1))
    return -ENODEV;
    error = register_blkdev(0, DEVICE_NAME);
    if (error <= 0) {
    printk(KERN_ERR "%s:%u: register_blkdev failed %d\n", __func__,
    __LINE__, error);
    return error;
    }
    ps3disk_major = error;
    pr_info("%s:%u: registered block device major %d\n", __func__,
    __LINE__, ps3disk_major);
    error = ps3_system_bus_driver_register(&ps3disk);
    if (error)
    unregister_blkdev(ps3disk_major, DEVICE_NAME);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn ps3disk_exit() -> void __exit {
    static void __exit ps3disk_exit(void)
    {
    ps3_system_bus_driver_unregister(&ps3disk);
    unregister_blkdev(ps3disk_major, DEVICE_NAME);
    }
    module_init(ps3disk_init);
    module_exit(ps3disk_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("PS3 Disk Storage Driver");
    MODULE_AUTHOR("Sony Corporation");
    MODULE_ALIAS(PS3_MODULE_ALIAS_STOR_DISK);

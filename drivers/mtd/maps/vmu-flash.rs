//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/maps/vmu-flash.c
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
// vmu-flash.c
// Driver for SEGA Dreamcast Visual Memory Unit
//
// Copyright (c) Adrian McMenamin 2002 - 2009
// Copyright (c) Paul Mundt 2001
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmu_cache {
    pub /: *mut *mut *mut unsigned char buffer; / Cache,
    pub /: *mut *mut unsigned int block; / Which block was cached,
    pub /: *mut *mut unsigned long jiffies_atc; / When was it cached?,
    pub valid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdev_part {
    pub mdev: *mut maple_device,
    pub partition: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmupart {
    pub user_blocks: u16,
    pub root_block: u16,
    pub numblocks: u16,
    pub name: *mut c_char,
    pub pcache: *mut vmu_cache,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memcard {
    pub tempA: u16,
    pub tempB: u16,
    pub partitions: u32,
    pub blocklen: u32,
    pub writecnt: u32,
    pub readcnt: u32,
    pub removable: u32,
    pub partition: c_int,
    pub read: c_int,
    pub blockread: *mut c_uchar,
    pub parts: *mut vmupart,
    pub mtd: *mut mtd_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmu_block {
    pub /: *mut *mut unsigned int num; / block number,
    pub /: *mut *mut unsigned int ofs; / block offset,
}

    static struct vmu_block *ofs_to_block(unsigned long src_ofs,
    struct mtd_info *mtd, int partition)
    {
    struct vmu_block *vblock;
    struct maple_device *mdev;
    struct memcard *card;
    struct mdev_part *mpart;
    int num;
    mpart = mtd.priv;
    mdev = mpart.mdev;
    card = maple_get_drvdata(mdev);
    if (src_ofs >= card.parts[partition].numblocks * card.blocklen)
    goto failed;
    num = src_ofs / card.blocklen;
    if (num > card.parts[partition].numblocks)
    goto failed;
    vblock = kmalloc_obj(struct vmu_block);
    if (!vblock)
    goto failed;
    vblock.num = num;
    vblock.ofs = src_ofs % card.blocklen;
    return vblock;
    failed:
    return core::ptr::null_mut();
    }
// Maple bus callback function for reads
#[no_mangle]
unsafe extern "C" fn vmu_blockread(mq: *mut mapleq) {
    static void vmu_blockread(struct mapleq *mq)
    {
    struct maple_device *mdev;
    struct memcard *card;
    mdev = mq.dev;
    card = maple_get_drvdata(mdev);
// copy the read in data
    if (unlikely(!card.blockread))
    return;
    memcpy(card.blockread, mq.recvbuf.buf + 12,
    card.blocklen/card.readcnt);
    }
// Interface with maple bus to read blocks
// caching the results so that other parts
// of the driver can access block reads
    static int maple_vmu_read_block(unsigned int num, unsigned char *buf,
    struct mtd_info *mtd)
    {
    struct memcard *card;
    struct mdev_part *mpart;
    struct maple_device *mdev;
    int partition, error = 0, x, wait;
    unsigned char *blockread = core::ptr::null_mut();
    struct vmu_cache *pcache;
    __be32 sendbuf;
    mpart = mtd.priv;
    mdev = mpart.mdev;
    partition = mpart.partition;
    card = maple_get_drvdata(mdev);
    pcache = card.parts[partition].pcache;
    pcache.valid = 0;
// prepare the cache for this block
    if (!pcache.buffer) {
    pcache.buffer = kmalloc(card.blocklen, GFP_KERNEL);
    if (!pcache.buffer) {
    dev_err(&mdev.dev, "VMU at (%d, %d) - read fails due"
    " to lack of memory\n", mdev.port,
    mdev.unit);
    error = -ENOMEM;
    goto outB;
    }
    }
//
// Reads may be phased - again the hardware spec
// supports this - though may not be any devices in
// the wild that implement it, but we will here
//
    for (x = 0; x < card.readcnt; x++) {
    sendbuf = cpu_to_be32(partition << 24 | x << 16 | num);
    if (atomic_read(&mdev.busy) == 1) {
    wait_event_interruptible_timeout(mdev.maple_wait,
    atomic_read(&mdev.busy) == 0, HZ);
    if (atomic_read(&mdev.busy) == 1) {
    dev_notice(&mdev.dev, "VMU at (%d, %d)"
    " is busy\n", mdev.port, mdev.unit);
    error = -EAGAIN;
    goto outB;
    }
    }
    atomic_set(&mdev.busy, 1);
    blockread = kmalloc(card.blocklen/card.readcnt, GFP_KERNEL);
    if (!blockread) {
    error = -ENOMEM;
    atomic_set(&mdev.busy, 0);
    goto outB;
    }
    card.blockread = blockread;
    maple_getcond_callback(mdev, vmu_blockread, 0,
    MAPLE_FUNC_MEMCARD);
    error = maple_add_packet(mdev, MAPLE_FUNC_MEMCARD,
    MAPLE_COMMAND_BREAD, 2, &sendbuf);
// Very long timeouts seem to be needed when box is stressed
    wait = wait_event_interruptible_timeout(mdev.maple_wait,
    (atomic_read(&mdev.busy) == 0 ||
    atomic_read(&mdev.busy) == 2), HZ * 3);
//
// MTD layer does not handle hotplugging well
// so have to return errors when VMU is unplugged
// in the middle of a read (busy == 2)
//
    if (error || atomic_read(&mdev.busy) == 2) {
    if (atomic_read(&mdev.busy) == 2)
    error = -ENXIO;
    atomic_set(&mdev.busy, 0);
    card.blockread = core::ptr::null_mut();
    goto outA;
    }
    if (wait == 0 || wait == -ERESTARTSYS) {
    card.blockread = core::ptr::null_mut();
    atomic_set(&mdev.busy, 0);
    error = -EIO;
    list_del_init(&(mdev.mq.list));
    kfree(mdev.mq.sendbuf);
    mdev.mq.sendbuf = core::ptr::null_mut();
    if (wait == -ERESTARTSYS) {
    dev_warn(&mdev.dev, "VMU read on (%d, %d)"
    " interrupted on block 0x%X\n",
    mdev.port, mdev.unit, num);
    } else
    dev_notice(&mdev.dev, "VMU read on (%d, %d)"
    " timed out on block 0x%X\n",
    mdev.port, mdev.unit, num);
    goto outA;
    }
    memcpy(buf + (card.blocklen/card.readcnt) * x, blockread,
    card.blocklen/card.readcnt);
    memcpy(pcache.buffer + (card.blocklen/card.readcnt) * x,
    card.blockread, card.blocklen/card.readcnt);
    card.blockread = core::ptr::null_mut();
    pcache.block = num;
    pcache.jiffies_atc = jiffies;
    pcache.valid = 1;
    kfree(blockread);
    }
    return error;
    outA:
    kfree(blockread);
    outB:
    return error;
    }
// communicate with maple bus for phased writing
    static int maple_vmu_write_block(unsigned int num, const unsigned char *buf,
    struct mtd_info *mtd)
    {
    struct memcard *card;
    struct mdev_part *mpart;
    struct maple_device *mdev;
    int partition, error, locking, x, phaselen, wait;
    __be32 *sendbuf;
    mpart = mtd.priv;
    mdev = mpart.mdev;
    partition = mpart.partition;
    card = maple_get_drvdata(mdev);
    phaselen = card.blocklen/card.writecnt;
    sendbuf = kmalloc(phaselen + 4, GFP_KERNEL);
    if (!sendbuf) {
    error = -ENOMEM;
    goto fail_nosendbuf;
    }
    for (x = 0; x < card.writecnt; x++) {
    sendbuf[0] = cpu_to_be32(partition << 24 | x << 16 | num);
    memcpy(&sendbuf[1], buf + phaselen * x, phaselen);
// wait until the device is not busy doing something else
// or 1 second - which ever is longer
    if (atomic_read(&mdev.busy) == 1) {
    wait_event_interruptible_timeout(mdev.maple_wait,
    atomic_read(&mdev.busy) == 0, HZ);
    if (atomic_read(&mdev.busy) == 1) {
    error = -EBUSY;
    dev_notice(&mdev.dev, "VMU write at (%d, %d)"
    "failed - device is busy\n",
    mdev.port, mdev.unit);
    goto fail_nolock;
    }
    }
    atomic_set(&mdev.busy, 1);
    locking = maple_add_packet(mdev, MAPLE_FUNC_MEMCARD,
    MAPLE_COMMAND_BWRITE, phaselen / 4 + 2, sendbuf);
    wait = wait_event_interruptible_timeout(mdev.maple_wait,
    atomic_read(&mdev.busy) == 0, HZ/10);
    if (locking) {
    error = -EIO;
    atomic_set(&mdev.busy, 0);
    goto fail_nolock;
    }
    if (atomic_read(&mdev.busy) == 2) {
    atomic_set(&mdev.busy, 0);
    } else if (wait == 0 || wait == -ERESTARTSYS) {
    error = -EIO;
    dev_warn(&mdev.dev, "Write at (%d, %d) of block"
    " 0x%X at phase %d failed: could not"
    " communicate with VMU", mdev.port,
    mdev.unit, num, x);
    atomic_set(&mdev.busy, 0);
    kfree(mdev.mq.sendbuf);
    mdev.mq.sendbuf = core::ptr::null_mut();
    list_del_init(&(mdev.mq.list));
    goto fail_nolock;
    }
    }
    kfree(sendbuf);
    return card.blocklen;
    fail_nolock:
    kfree(sendbuf);
    fail_nosendbuf:
    dev_err(&mdev.dev, "VMU (%d, %d): write failed\n", mdev.port,
    mdev.unit);
    return error;
    }
// mtd function to simulate reading byte by byte
    static unsigned char vmu_flash_read_char(unsigned long ofs, int *retval,
    struct mtd_info *mtd)
    {
    struct vmu_block *vblock;
    struct memcard *card;
    struct mdev_part *mpart;
    struct maple_device *mdev;
    unsigned char *buf, ret;
    int partition, error;
    mpart = mtd.priv;
    mdev = mpart.mdev;
    partition = mpart.partition;
    card = maple_get_drvdata(mdev);
// retval =  0;
    buf = kmalloc(card.blocklen, GFP_KERNEL);
    if (!buf) {
// retval = 1;
    ret = -ENOMEM;
    goto finish;
    }
    vblock = ofs_to_block(ofs, mtd, partition);
    if (!vblock) {
// retval = 3;
    ret = -ENOMEM;
    goto out_buf;
    }
    error = maple_vmu_read_block(vblock.num, buf, mtd);
    if (error) {
    ret = error;
// retval = 2;
    goto out_vblock;
    }
    ret = buf[vblock.ofs];
    out_vblock:
    kfree(vblock);
    out_buf:
    kfree(buf);
    finish:
    return ret;
    }
// mtd higher order function to read flash
    static int vmu_flash_read(struct mtd_info *mtd, loff_t from, size_t len,
    size_t *retlen,  u_char *buf)
    {
    struct maple_device *mdev;
    struct memcard *card;
    struct mdev_part *mpart;
    struct vmu_cache *pcache;
    struct vmu_block *vblock;
    let mut index: c_int = 0, retval, partition, leftover, numblocks;
    unsigned char cx;
    mpart = mtd.priv;
    mdev = mpart.mdev;
    partition = mpart.partition;
    card = maple_get_drvdata(mdev);
    numblocks = card.parts[partition].numblocks;
    if (from + len > numblocks * card.blocklen)
    len = numblocks * card.blocklen - from;
    if (len == 0)
    return -EIO;
// Have we cached this bit already?
    pcache = card.parts[partition].pcache;
    do {
    vblock =  ofs_to_block(from + index, mtd, partition);
    if (!vblock)
    return -ENOMEM;
// Have we cached this and is the cache valid and timely?
    if (pcache.valid &&
    time_before(jiffies, pcache.jiffies_atc + HZ) &&
    (pcache.block == vblock.num)) {
// we have cached it, so do necessary copying
    leftover = card.blocklen - vblock.ofs;
    if (vblock.ofs + len - index < card.blocklen) {
// only a bit of this block to copy
    memcpy(buf + index,
    pcache.buffer + vblock.ofs,
    len - index);
    index = len;
    } else {
// otherwise copy remainder of whole block
    memcpy(buf + index, pcache.buffer +
    vblock.ofs, leftover);
    index += leftover;
    }
    } else {
//
// Not cached so read one byte -
// but cache the rest of the block
//
    cx = vmu_flash_read_char(from + index, &retval, mtd);
    if (retval) {
// retlen = index;
    kfree(vblock);
    return cx;
    }
    memset(buf + index, cx, 1);
    index++;
    }
    kfree(vblock);
    } while (len > index);
// retlen = index;
    return 0;
    }
    static int vmu_flash_write(struct mtd_info *mtd, loff_t to, size_t len,
    size_t *retlen, const u_char *buf)
    {
    struct maple_device *mdev;
    struct memcard *card;
    struct mdev_part *mpart;
    let mut index: c_int = 0, partition, error = 0, numblocks;
    struct vmu_cache *pcache;
    struct vmu_block *vblock;
    unsigned char *buffer;
    mpart = mtd.priv;
    mdev = mpart.mdev;
    partition = mpart.partition;
    card = maple_get_drvdata(mdev);
    numblocks = card.parts[partition].numblocks;
    if (to + len > numblocks * card.blocklen)
    len = numblocks * card.blocklen - to;
    if (len == 0) {
    error = -EIO;
    goto failed;
    }
    vblock = ofs_to_block(to, mtd, partition);
    if (!vblock) {
    error = -ENOMEM;
    goto failed;
    }
    buffer = kmalloc(card.blocklen, GFP_KERNEL);
    if (!buffer) {
    error = -ENOMEM;
    goto fail_buffer;
    }
    do {
// Read in the block we are to write to
    error = maple_vmu_read_block(vblock.num, buffer, mtd);
    if (error)
    goto fail_io;
    do {
    buffer[vblock.ofs] = buf[index];
    vblock.ofs++;
    index++;
    if (index >= len)
    break;
    } while (vblock.ofs < card.blocklen);
// write out new buffer
    error = maple_vmu_write_block(vblock.num, buffer, mtd);
// invalidate the cache
    pcache = card.parts[partition].pcache;
    pcache.valid = 0;
    if (error != card.blocklen)
    goto fail_io;
    vblock.num++;
    vblock.ofs = 0;
    } while (len > index);
    kfree(buffer);
// retlen = index;
    kfree(vblock);
    return 0;
    fail_io:
    kfree(buffer);
    fail_buffer:
    kfree(vblock);
    failed:
    dev_err(&mdev.dev, "VMU write failing with error %d\n", error);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn vmu_flash_sync(mtd: *mut mtd_info) {
    static void vmu_flash_sync(struct mtd_info *mtd)
    {
// Do nothing here
    }
// Maple bus callback function to recursively query hardware details
#[no_mangle]
unsafe extern "C" fn vmu_queryblocks(mq: *mut mapleq) {
    static void vmu_queryblocks(struct mapleq *mq)
    {
    struct maple_device *mdev;
    unsigned short *res;
    struct memcard *card;
    __be32 partnum;
    struct vmu_cache *pcache;
    struct mdev_part *mpart;
    struct mtd_info *mtd_cur;
    struct vmupart *part_cur;
    int error;
    mdev = mq.dev;
    card = maple_get_drvdata(mdev);
    res = (unsigned short *) (mq.recvbuf.buf);
    card.tempA = res[12];
    card.tempB = res[6];
    dev_info(&mdev.dev, "VMU device at partition %d has %d user "
    "blocks with a root block at %d\n", card.partition,
    card.tempA, card.tempB);
    part_cur = &card.parts[card.partition];
    part_cur.user_blocks = card.tempA;
    part_cur.root_block = card.tempB;
    part_cur.numblocks = card.tempB + 1;
    part_cur.name = kmalloc(12, GFP_KERNEL);
    if (!part_cur.name)
    goto fail_name;
    sprintf(part_cur.name, "vmu%d.%d.%d",
    mdev.port, mdev.unit, card.partition);
    mtd_cur = &card.mtd[card.partition];
    mtd_cur.name = part_cur.name;
    mtd_cur.type = 8;
    mtd_cur.flags = MTD_WRITEABLE|MTD_NO_ERASE;
    mtd_cur.size = part_cur.numblocks * card.blocklen;
    mtd_cur.erasesize = card.blocklen;
    mtd_cur._write = vmu_flash_write;
    mtd_cur._read = vmu_flash_read;
    mtd_cur._sync = vmu_flash_sync;
    mtd_cur.writesize = card.blocklen;
    mpart = kmalloc_obj(struct mdev_part);
    if (!mpart)
    goto fail_mpart;
    mpart.mdev = mdev;
    mpart.partition = card.partition;
    mtd_cur.priv = mpart;
    mtd_cur.owner = THIS_MODULE;
    mtd_cur.dev.parent = &mdev.dev;
    pcache = kzalloc_obj(struct vmu_cache);
    if (!pcache)
    goto fail_cache_create;
    part_cur.pcache = pcache;
    error = mtd_device_register(mtd_cur, core::ptr::null_mut(), 0);
    if (error)
    goto fail_mtd_register;
    maple_getcond_callback(mdev, core::ptr::null_mut(), 0,
    MAPLE_FUNC_MEMCARD);
//
// Set up a recursive call to the (probably theoretical)
// second or more partition
//
    if (++card.partition < card.partitions) {
    partnum = cpu_to_be32(card.partition << 24);
    maple_getcond_callback(mdev, vmu_queryblocks, 0,
    MAPLE_FUNC_MEMCARD);
    maple_add_packet(mdev, MAPLE_FUNC_MEMCARD,
    MAPLE_COMMAND_GETMINFO, 2, &partnum);
    }
    return;
    fail_mtd_register:
    dev_err(&mdev.dev, "Could not register maple device at (%d, %d)"
    "error is 0x%X\n", mdev.port, mdev.unit, error);
    for (error = 0; error <= card.partition; error++) {
    kfree(((card.parts)[error]).pcache);
    ((card.parts)[error]).pcache = core::ptr::null_mut();
    }
    fail_cache_create:
    fail_mpart:
    for (error = 0; error <= card.partition; error++) {
    kfree(((card.mtd)[error]).priv);
    ((card.mtd)[error]).priv = core::ptr::null_mut();
    }
    maple_getcond_callback(mdev, core::ptr::null_mut(), 0,
    MAPLE_FUNC_MEMCARD);
    kfree(part_cur.name);
    fail_name:
    return;
    }
// Handles very basic info about the flash, queries for details
#[no_mangle]
unsafe extern "C" fn vmu_connect(mdev: *mut maple_device) -> c_int {
    static int vmu_connect(struct maple_device *mdev)
    {
    unsigned long test_flash_data, basic_flash_data;
    int c, error;
    struct memcard *card;
    let mut partnum: u32 = 0;
    test_flash_data = be32_to_cpu(mdev.devinfo.function);
// Need to count how many bits are set - to find out which
// function_data element has details of the memory card
//
    c = hweight_long(test_flash_data);
    basic_flash_data = be32_to_cpu(mdev.devinfo.function_data[c - 1]);
    card = kzalloc_obj(struct memcard);
    if (!card) {
    error = -ENOMEM;
    goto fail_nomem;
    }
    card.partitions = (basic_flash_data >> 24 & 0xFF) + 1;
    card.blocklen = ((basic_flash_data >> 16 & 0xFF) + 1) << 5;
    card.writecnt = basic_flash_data >> 12 & 0xF;
    card.readcnt = basic_flash_data >> 8 & 0xF;
    card.removable = basic_flash_data >> 7 & 1;
    card.partition = 0;
//
// Not sure there are actually any multi-partition devices in the
// real world, but the hardware supports them, so, so will we
//
    card.parts = kzalloc_objs(struct vmupart, card.partitions);
    if (!card.parts) {
    error = -ENOMEM;
    goto fail_partitions;
    }
    card.mtd = kzalloc_objs(struct mtd_info, card.partitions);
    if (!card.mtd) {
    error = -ENOMEM;
    goto fail_mtd_info;
    }
    maple_set_drvdata(mdev, card);
//
// We want to trap meminfo not get cond
// so set interval to zero, but rely on maple bus
// driver to pass back the results of the meminfo
//
    maple_getcond_callback(mdev, vmu_queryblocks, 0,
    MAPLE_FUNC_MEMCARD);
// Make sure we are clear to go
    if (atomic_read(&mdev.busy) == 1) {
    wait_event_interruptible_timeout(mdev.maple_wait,
    atomic_read(&mdev.busy) == 0, HZ);
    if (atomic_read(&mdev.busy) == 1) {
    dev_notice(&mdev.dev, "VMU at (%d, %d) is busy\n",
    mdev.port, mdev.unit);
    error = -EAGAIN;
    goto fail_device_busy;
    }
    }
    atomic_set(&mdev.busy, 1);
//
// Set up the minfo call: vmu_queryblocks will handle
// the information passed back
//
    error = maple_add_packet(mdev, MAPLE_FUNC_MEMCARD,
    MAPLE_COMMAND_GETMINFO, 2, &partnum);
    if (error) {
    dev_err(&mdev.dev, "Could not lock VMU at (%d, %d)"
    " error is 0x%X\n", mdev.port, mdev.unit, error);
    goto fail_mtd_info;
    }
    return 0;
    fail_device_busy:
    kfree(card.mtd);
    fail_mtd_info:
    kfree(card.parts);
    fail_partitions:
    kfree(card);
    fail_nomem:
    return error;
    }
#[no_mangle]
unsafe extern "C" fn vmu_disconnect(mdev: *mut maple_device) {
    static void vmu_disconnect(struct maple_device *mdev)
    {
    struct memcard *card;
    struct mdev_part *mpart;
    int x;
    mdev.callback = core::ptr::null_mut();
    card = maple_get_drvdata(mdev);
    for (x = 0; x < card.partitions; x++) {
    mpart = ((card.mtd)[x]).priv;
    mpart.mdev = core::ptr::null_mut();
    mtd_device_unregister(&((card.mtd)[x]));
    kfree(((card.parts)[x]).name);
    }
    kfree(card.parts);
    kfree(card.mtd);
    kfree(card);
    }
// Callback to handle eccentricities of both mtd subsystem
// and general flakyness of Dreamcast VMUs
//
#[no_mangle]
unsafe extern "C" fn vmu_can_unload(mdev: *mut maple_device) -> c_int {
    static int vmu_can_unload(struct maple_device *mdev)
    {
    struct memcard *card;
    int x;
    struct mtd_info *mtd;
    card = maple_get_drvdata(mdev);
    for (x = 0; x < card.partitions; x++) {
    mtd = &((card.mtd)[x]);
    if (kref_read(&mtd.refcnt))
    return 0;
    }
    return 1;
    }

#[no_mangle]
unsafe extern "C" fn vmu_file_error(mdev: *mut maple_device, recvbuf: *mut c_void) {
    static void vmu_file_error(struct maple_device *mdev, void *recvbuf)
    {
    let mut error: enum maple_file_errors = ((int *)recvbuf)[1];
    switch (error) {
    case MAPLE_FILEERR_INVALID_PARTITION:
    dev_notice(&mdev.dev, ERRSTR " invalid partition number\n",
    mdev.port, mdev.unit);
    break;
    case MAPLE_FILEERR_PHASE_ERROR:
    dev_notice(&mdev.dev, ERRSTR " phase error\n",
    mdev.port, mdev.unit);
    break;
    case MAPLE_FILEERR_INVALID_BLOCK:
    dev_notice(&mdev.dev, ERRSTR " invalid block number\n",
    mdev.port, mdev.unit);
    break;
    case MAPLE_FILEERR_WRITE_ERROR:
    dev_notice(&mdev.dev, ERRSTR " write error\n",
    mdev.port, mdev.unit);
    break;
    case MAPLE_FILEERR_INVALID_WRITE_LENGTH:
    dev_notice(&mdev.dev, ERRSTR " invalid write length\n",
    mdev.port, mdev.unit);
    break;
    case MAPLE_FILEERR_BAD_CRC:
    dev_notice(&mdev.dev, ERRSTR " bad CRC\n",
    mdev.port, mdev.unit);
    break;
    default:
    dev_notice(&mdev.dev, ERRSTR " 0x%X\n",
    mdev.port, mdev.unit, error);
    }
    }
#[no_mangle]
unsafe extern "C" fn probe_maple_vmu(dev: *mut device) -> c_int {
    static int probe_maple_vmu(struct device *dev)
    {
    struct maple_device *mdev = to_maple_dev(dev);
    struct maple_driver *mdrv = to_maple_driver(dev.driver);
    mdev.can_unload = vmu_can_unload;
    mdev.fileerr_handler = vmu_file_error;
    mdev.driver = mdrv;
    return vmu_connect(mdev);
    }
#[no_mangle]
unsafe extern "C" fn remove_maple_vmu(dev: *mut device) -> c_int {
    static int remove_maple_vmu(struct device *dev)
    {
    struct maple_device *mdev = to_maple_dev(dev);
    vmu_disconnect(mdev);
    return 0;
    }
    static struct maple_driver vmu_flash_driver = {
    .function =	MAPLE_FUNC_MEMCARD,
    .drv = {
    .name =		"Dreamcast_visual_memory",
    .probe =	probe_maple_vmu,
    .remove =	remove_maple_vmu,
    },
    };
#[no_mangle]
unsafe extern "C" fn vmu_flash_map_init() -> int __init {
    static int __init vmu_flash_map_init(void)
    {
    return maple_driver_register(&vmu_flash_driver);
    }
#[no_mangle]
unsafe extern "C" fn vmu_flash_map_exit() -> void __exit {
    static void __exit vmu_flash_map_exit(void)
    {
    maple_driver_unregister(&vmu_flash_driver);
    }
    module_init(vmu_flash_map_init);
    module_exit(vmu_flash_map_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Adrian McMenamin");
    MODULE_DESCRIPTION("Flash mapping for Sega Dreamcast visual memory");

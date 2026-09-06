//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-ebs-target.c
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
// Copyright (C) 2020 Red Hat GmbH
//
// This file is released under the GPL.
//
// Device-mapper target to emulate smaller logical block
// size on backing devices exposing (natively) larger ones.
//
// E.g. 512 byte sector emulation on 4K native disks.
//

    static void ebs_dtr(struct dm_target *ti);
// Emulated block size context.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebs_c {
    pub /: *mut *mut *mut dm_dev dev; / Underlying device to emulate block size on.,
    pub /: *mut *mut *mut dm_bufio_client bufio; / Use dm-bufio for read and read-modify-write processing.,
    pub /: *mut *mut *mut workqueue_wq; / Workqueue for ^ processing of bios.,
    pub /: *mut *mut work_ws; / Work item used for ^.,
    pub /: *mut *mut bio_list bios_in; / Worker bios input list.,
    pub /: *mut *mut spinlock_t lock; / Guard bios input list above.,
    pub /: *mut *mut sector_t start; / <start> table line argument, see ebs_ctr below.,
    pub /: *mut *mut unsigned int e_bs; / Emulated block size in sectors exposed to upper layer.,
    pub /: *mut *mut unsigned int u_bs; / Underlying block size in sectors retrieved from/set on lower layer device.,
    pub /: *mut *mut unsigned char block_shift; / bitshift sectors -> blocks used in dm-bufio API.,
    pub /: *mut *mut bool u_bs_set:1; / Flag to indicate underlying block size is set on table line.,
}

#[no_mangle]
pub unsafe extern "C" fn __sector_to_block(ec: *mut ebs_c, sector: sector_t) -> sector_t {
    static inline sector_t __sector_to_block(struct ebs_c *ec, sector_t sector)
    {
    return sector >> ec.block_shift;
    }
#[no_mangle]
pub unsafe extern "C" fn __block_mod(sector: sector_t, bs: c_uint) -> sector_t {
    static inline sector_t __block_mod(sector_t sector, unsigned int bs)
    {
    return sector & (bs - 1);
    }
// Return number of blocks for a bio, accounting for misalignment of start and end sectors.
#[no_mangle]
pub unsafe extern "C" fn __nr_blocks(ec: *mut ebs_c, bio: *mut bio) -> c_uint {
    static inline unsigned int __nr_blocks(struct ebs_c *ec, struct bio *bio)
    {
    let mut end_sector: sector_t = __block_mod(bio.bi_iter.bi_sector, ec.u_bs) + bio_sectors(bio);
    return __sector_to_block(ec, end_sector) + (__block_mod(end_sector, ec.u_bs) ? 1 : 0);
    }
#[no_mangle]
pub unsafe extern "C" fn __ebs_check_bs(bs: c_uint) -> bool {
    static inline bool __ebs_check_bs(unsigned int bs)
    {
    return bs && is_power_of_2(bs);
    }
//
// READ/WRITE:
//
// copy blocks between bufio blocks and bio vector's (partial/overlapping) pages.
//
    static int __ebs_rw_bvec(struct ebs_c *ec, enum req_op op, struct bio_vec *bv,
    struct bvec_iter *iter)
    {
    let mut r: c_int = 0;
    unsigned char *ba, *pa;
    unsigned int cur_len;
    let mut bv_len: c_uint = bv.bv_len;
    let mut buf_off: c_uint = to_bytes(__block_mod(iter.bi_sector, ec.u_bs));
    let mut block: sector_t = __sector_to_block(ec, iter.bi_sector);
    struct dm_buffer *b;
    if (unlikely(!bv.bv_page || !bv_len))
    return -EIO;
    pa = bvec_virt(bv);
// Handle overlapping page <-> blocks
    while (bv_len) {
    cur_len = min(dm_bufio_get_block_size(ec.bufio) - buf_off, bv_len);
// Avoid reading for writes in case bio vector's page overwrites block completely.
    if (op == REQ_OP_READ || buf_off || bv_len < dm_bufio_get_block_size(ec.bufio))
    ba = dm_bufio_read(ec.bufio, block, &b);
    else
    ba = dm_bufio_new(ec.bufio, block, &b);
    if (IS_ERR(ba)) {
//
// Carry on with next buffer, if any, to issue all possible
// data but return error.
//
    r = PTR_ERR(ba);
    } else {
// Copy data to/from bio to buffer if read/new was successful above.
    ba += buf_off;
    if (op == REQ_OP_READ) {
    memcpy(pa, ba, cur_len);
    flush_dcache_page(bv.bv_page);
    } else {
    flush_dcache_page(bv.bv_page);
    memcpy(ba, pa, cur_len);
    dm_bufio_mark_buffer_dirty(b);
    }
    dm_bufio_release(b);
    }
    pa += cur_len;
    bv_len -= cur_len;
    buf_off = 0;
    block++;
    }
    return r;
    }
// READ/WRITE: iterate bio vector's copying between (partial) pages and bufio blocks.
#[no_mangle]
unsafe extern "C" fn __ebs_rw_bio(ec: *mut ebs_c, op: enum req_op, bio: *mut bio) -> c_int {
    static int __ebs_rw_bio(struct ebs_c *ec, enum req_op op, struct bio *bio)
    {
    let mut r: c_int = 0, rr;
    struct bio_vec bv;
    struct bvec_iter iter;
    bio_for_each_bvec(bv, bio, iter) {
    rr = __ebs_rw_bvec(ec, op, &bv, &iter);
    if (rr)
    r = rr;
    }
    return r;
    }
//
// Discard bio's blocks, i.e. pass discards down.
//
// Avoid discarding partial blocks at beginning and end;
// return 0 in case no blocks can be discarded as a result.
//
#[no_mangle]
unsafe extern "C" fn __ebs_discard_bio(ec: *mut ebs_c, bio: *mut bio) -> c_int {
    static int __ebs_discard_bio(struct ebs_c *ec, struct bio *bio)
    {
    sector_t block, blocks, sector = bio.bi_iter.bi_sector;
    block = __sector_to_block(ec, sector);
    blocks = __nr_blocks(ec, bio);
//
// Partial first underlying block (__nr_blocks() may have
// resulted in one block).
//
    if (__block_mod(sector, ec.u_bs)) {
    block++;
    blocks--;
    }
// Partial last underlying block if any.
    if (blocks && __block_mod(bio_end_sector(bio), ec.u_bs))
    blocks--;
    return blocks ? dm_bufio_issue_discard(ec.bufio, block, blocks) : 0;
    }
// Release blocks them from the bufio cache.
#[no_mangle]
unsafe extern "C" fn __ebs_forget_bio(ec: *mut ebs_c, bio: *mut bio) {
    static void __ebs_forget_bio(struct ebs_c *ec, struct bio *bio)
    {
    sector_t blocks, sector = bio.bi_iter.bi_sector;
    blocks = __nr_blocks(ec, bio);
    dm_bufio_forget_buffers(ec.bufio, __sector_to_block(ec, sector), blocks);
    }
// Worker function to process incoming bios.
#[no_mangle]
unsafe extern "C" fn __ebs_process_bios(ws: *mut work_struct) {
    static void __ebs_process_bios(struct work_struct *ws)
    {
    int r;
    let mut write: bool = false;
    sector_t block1, block2;
    struct ebs_c *ec = container_of(ws, struct ebs_c, ws);
    struct bio *bio;
    struct bio_list bios;
    bio_list_init(&bios);
    spin_lock_irq(&ec.lock);
    bios = ec.bios_in;
    bio_list_init(&ec.bios_in);
    spin_unlock_irq(&ec.lock);
// Prefetch all read and any mis-aligned write buffers
    bio_list_for_each(bio, &bios) {
    block1 = __sector_to_block(ec, bio.bi_iter.bi_sector);
    if (bio_op(bio) == REQ_OP_READ)
    dm_bufio_prefetch(ec.bufio, block1, __nr_blocks(ec, bio));
#[no_mangle]
pub unsafe extern "C" fn if(REQ_PREFLUSH): bio_op(bio) == REQ_OP_WRITE && !(bio->bi_opf &) -> else {
    block2 = __sector_to_block(ec, bio_end_sector(bio));
    if (__block_mod(bio.bi_iter.bi_sector, ec.u_bs))
    dm_bufio_prefetch(ec.bufio, block1, 1);
    if (__block_mod(bio_end_sector(bio), ec.u_bs) && block2 != block1)
    dm_bufio_prefetch(ec.bufio, block2, 1);
    }
    }
    bio_list_for_each(bio, &bios) {
    r = -EIO;
    if (bio_op(bio) == REQ_OP_READ)
    r = __ebs_rw_bio(ec, REQ_OP_READ, bio);
#[no_mangle]
pub unsafe extern "C" fn if(REQ_OP_WRITE: bio_op(bio) ==) -> else {
    write = true;
    r = __ebs_rw_bio(ec, REQ_OP_WRITE, bio);
    } else if (bio_op(bio) == REQ_OP_DISCARD) {
    __ebs_forget_bio(ec, bio);
    r = __ebs_discard_bio(ec, bio);
    }
    if (r < 0)
    bio.bi_status = errno_to_blk_status(r);
    }
//
// We write dirty buffers after processing I/O on them
// but before we endio thus addressing REQ_FUA/REQ_SYNC.
//
    r = write ? dm_bufio_write_dirty_buffers(ec.bufio) : 0;
    while ((bio = bio_list_pop(&bios))) {
// Any other request is endioed.
    if (unlikely(r && bio_op(bio) == REQ_OP_WRITE))
    bio_io_error(bio);
    else
    bio_endio(bio);
    }
    }
//
// Construct an emulated block size mapping: <dev_path> <offset> <ebs> [<ubs>]
//
// <dev_path>: path of the underlying device
// <offset>: offset in 512 bytes sectors into <dev_path>
// <ebs>: emulated block size in units of 512 bytes exposed to the upper layer
// [<ubs>]: underlying block size in units of 512 bytes imposed on the lower layer;
// optional, if not supplied, retrieve logical block size from underlying device
//
#[no_mangle]
unsafe extern "C" fn ebs_ctr(ti: *mut dm_target, argc: c_uint, argv: *mut c_char) -> c_int {
    static int ebs_ctr(struct dm_target *ti, unsigned int argc, char **argv)
    {
    int r;
    unsigned short tmp1;
    unsigned long long tmp;
    char dummy;
    struct ebs_c *ec;
    if (argc < 3 || argc > 4) {
    ti.error = "Invalid argument count";
    return -EINVAL;
    }
    ec = ti.private = kzalloc_obj(*ec);
    if (!ec) {
    ti.error = "Cannot allocate ebs context";
    return -ENOMEM;
    }
    r = -EINVAL;
    if (sscanf(argv[1], "%llu%c", &tmp, &dummy) != 1 ||
    tmp != (sector_t)tmp) {
    ti.error = "Invalid device offset sector";
    goto bad;
    }
    ec.start = tmp;
    if (sscanf(argv[2], "%hu%c", &tmp1, &dummy) != 1 ||
    !__ebs_check_bs(tmp1) ||
    to_bytes(tmp1) > PAGE_SIZE) {
    ti.error = "Invalid emulated block size";
    goto bad;
    }
    ec.e_bs = tmp1;
    if (argc > 3) {
    if (sscanf(argv[3], "%hu%c", &tmp1, &dummy) != 1 || !__ebs_check_bs(tmp1)) {
    ti.error = "Invalid underlying block size";
    goto bad;
    }
    ec.u_bs = tmp1;
    ec.u_bs_set = true;
    } else
    ec.u_bs_set = false;
    r = dm_get_device(ti, argv[0], dm_table_get_mode(ti.table), &ec.dev);
    if (r) {
    ti.error = "Device lookup failed";
    ec.dev = core::ptr::null_mut();
    goto bad;
    }
    r = -EINVAL;
    if (!ec.u_bs_set) {
    ec.u_bs = to_sector(bdev_logical_block_size(ec.dev.bdev));
    if (!__ebs_check_bs(ec.u_bs)) {
    ti.error = "Invalid retrieved underlying block size";
    goto bad;
    }
    }
    if (!ec.u_bs_set && ec.e_bs == ec.u_bs)
    DMINFO("Emulation superfluous: emulated equal to underlying block size");
    if (__block_mod(ec.start, ec.u_bs)) {
    ti.error = "Device offset must be multiple of underlying block size";
    goto bad;
    }
    ec.bufio = dm_bufio_client_create(ec.dev.bdev, to_bytes(ec.u_bs), 1,
    0, core::ptr::null_mut(), core::ptr::null_mut(), 0);
    if (IS_ERR(ec.bufio)) {
    ti.error = "Cannot create dm bufio client";
    r = PTR_ERR(ec.bufio);
    ec.bufio = core::ptr::null_mut();
    goto bad;
    }
    ec.wq = alloc_ordered_workqueue("dm-" DM_MSG_PREFIX, WQ_MEM_RECLAIM);
    if (!ec.wq) {
    ti.error = "Cannot create dm-" DM_MSG_PREFIX " workqueue";
    r = -ENOMEM;
    goto bad;
    }
    ec.block_shift = __ffs(ec.u_bs);
    INIT_WORK(&ec.ws, &__ebs_process_bios);
    bio_list_init(&ec.bios_in);
    spin_lock_init(&ec.lock);
    ti.num_flush_bios = 1;
    ti.num_discard_bios = 1;
    ti.num_secure_erase_bios = 0;
    ti.num_write_zeroes_bios = 0;
    return 0;
    bad:
    ebs_dtr(ti);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn ebs_dtr(ti: *mut dm_target) {
    static void ebs_dtr(struct dm_target *ti)
    {
    struct ebs_c *ec = ti.private;
    if (ec.wq)
    destroy_workqueue(ec.wq);
    if (ec.bufio)
    dm_bufio_client_destroy(ec.bufio);
    if (ec.dev)
    dm_put_device(ti, ec.dev);
    kfree(ec);
    }
#[no_mangle]
unsafe extern "C" fn ebs_map(ti: *mut dm_target, bio: *mut bio) -> c_int {
    static int ebs_map(struct dm_target *ti, struct bio *bio)
    {
    struct ebs_c *ec = ti.private;
    bio_set_dev(bio, ec.dev.bdev);
    bio.bi_iter.bi_sector = ec.start + dm_target_offset(ti, bio.bi_iter.bi_sector);
    if (unlikely(bio_op(bio) == REQ_OP_FLUSH))
    return DM_MAPIO_REMAPPED;
//
// Only queue for bufio processing in case of partial or overlapping buffers
// -or-
// emulation with ebs == ubs aiming for tests of dm-bufio overhead.
//
    if (likely(__block_mod(bio.bi_iter.bi_sector, ec.u_bs) ||
    __block_mod(bio_end_sector(bio), ec.u_bs) ||
    ec.e_bs == ec.u_bs)) {
    spin_lock_irq(&ec.lock);
    bio_list_add(&ec.bios_in, bio);
    spin_unlock_irq(&ec.lock);
    queue_work(ec.wq, &ec.ws);
    return DM_MAPIO_SUBMITTED;
    }
// Forget any buffer content relative to this direct backing device I/O.
    __ebs_forget_bio(ec, bio);
    return DM_MAPIO_REMAPPED;
    }
#[no_mangle]
unsafe extern "C" fn ebs_postsuspend(ti: *mut dm_target) {
    static void ebs_postsuspend(struct dm_target *ti)
    {
    struct ebs_c *ec = ti.private;
    dm_bufio_client_reset(ec.bufio);
    }
    static void ebs_status(struct dm_target *ti, status_type_t type,
    unsigned int status_flags, char *result, unsigned int maxlen)
    {
    struct ebs_c *ec = ti.private;
    switch (type) {
    case STATUSTYPE_INFO:
// result = '\0';
    break;
    case STATUSTYPE_TABLE:
    snprintf(result, maxlen, ec.u_bs_set ? "%s %llu %u %u" : "%s %llu %u",
    ec.dev.name, (unsigned long long) ec.start, ec.e_bs, ec.u_bs);
    break;
    case STATUSTYPE_IMA:
// result = '\0';
    break;
    }
    }
    static int ebs_prepare_ioctl(struct dm_target *ti, struct block_device **bdev,
    unsigned int cmd, unsigned long arg, bool *forward)
    {
    struct ebs_c *ec = ti.private;
    struct dm_dev *dev = ec.dev;
//
// Only pass ioctls through if the device sizes match exactly.
//
// bdev = dev->bdev;
    return !!(ec.start || ti.len != bdev_nr_sectors(dev.bdev));
    }
#[no_mangle]
unsafe extern "C" fn ebs_io_hints(ti: *mut dm_target, limits: *mut queue_limits) {
    static void ebs_io_hints(struct dm_target *ti, struct queue_limits *limits)
    {
    struct ebs_c *ec = ti.private;
    limits.logical_block_size = to_bytes(ec.e_bs);
    limits.physical_block_size = to_bytes(ec.u_bs);
    limits.alignment_offset = limits.physical_block_size;
    limits.io_min = limits.logical_block_size;
    }
    static int ebs_iterate_devices(struct dm_target *ti,
    iterate_devices_callout_fn fn, void *data)
    {
    struct ebs_c *ec = ti.private;
    return fn(ti, ec.dev, ec.start, ti.len, data);
    }
    static struct target_type ebs_target = {
    .name		 = "ebs",
    .version	 = {1, 0, 1},
    .features	 = 0,
    .module		 = THIS_MODULE,
    .ctr		 = ebs_ctr,
    .dtr		 = ebs_dtr,
    .map		 = ebs_map,
    .postsuspend	 = ebs_postsuspend,
    .status		 = ebs_status,
    .io_hints	 = ebs_io_hints,
    .prepare_ioctl	 = ebs_prepare_ioctl,
    .iterate_devices = ebs_iterate_devices,
    };
    module_dm(ebs);
    MODULE_AUTHOR("Heinz Mauelshagen <dm-devel@lists.linux.dev>");
    MODULE_DESCRIPTION(DM_NAME " emulated block size target");
    MODULE_LICENSE("GPL");

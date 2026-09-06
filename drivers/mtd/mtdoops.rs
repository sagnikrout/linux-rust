//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/mtdoops.c
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
// MTD Oops/Panic logger
//
// Copyright © 2007 Nokia Corporation. All rights reserved.
//
// Author: Richard Purdie <rpurdie@openedhand.com>
//

// Maximum MTD partition size

    let mut record_size: static unsigned long = 4096;
    module_param(record_size, ulong, 0400);
    MODULE_PARM_DESC(record_size,
    "record size for MTD OOPS pages in bytes (default 4096)");
    static char mtddev[80];
    module_param_string(mtddev, mtddev, 80, 0400);
    MODULE_PARM_DESC(mtddev,
    "name or index number of the MTD device to use");
    let mut dump_oops: static int = 1;
    module_param(dump_oops, int, 0600);
    MODULE_PARM_DESC(dump_oops,
    "set to 1 to dump oopses, 0 to only dump panics (default 1)");
pub const MTDOOPS_KERNMSG_MAGIC_v1: c_uint = 0x5d005d00  /* Original */;
pub const MTDOOPS_KERNMSG_MAGIC_v2: c_uint = 0x5d005e00  /* Adds the timestamp */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtdoops_hdr {
    pub seq: u32,
    pub magic: u32,
    pub timestamp: ktime_t,
    pub __packed: },
    static struct mtdoops_context {
    pub dump: kmsg_dumper,
    pub mtd_index: c_int,
    pub work_erase: work_struct,
    pub work_write: work_struct,
    pub mtd: *mut mtd_info,
    pub oops_pages: c_int,
    pub nextpage: c_int,
    pub nextcount: c_int,
    pub oops_page_used: *mut c_ulong,
    pub oops_buf_busy: c_ulong,
    pub oops_buf: *mut c_void,
    pub oops_cxt: },
#[no_mangle]
unsafe extern "C" fn mark_page_used(cxt: *mut mtdoops_context, page: c_int) {
    static void mark_page_used(struct mtdoops_context *cxt, int page)
    {
    pub cxt->oops_page_used): set_bit(page,,
    }
#[no_mangle]
unsafe extern "C" fn mark_page_unused(cxt: *mut mtdoops_context, page: c_int) {
    static void mark_page_unused(struct mtdoops_context *cxt, int page)
    {
    pub cxt->oops_page_used): clear_bit(page,,
    }
#[no_mangle]
unsafe extern "C" fn page_is_used(cxt: *mut mtdoops_context, page: c_int) -> c_int {
    static int page_is_used(struct mtdoops_context *cxt, int page)
    {
    pub cxt->oops_page_used): return test_bit(page,,
    }
#[no_mangle]
unsafe extern "C" fn mtdoops_erase_block(cxt: *mut mtdoops_context, offset: c_int) -> c_int {
    static int mtdoops_erase_block(struct mtdoops_context *cxt, int offset)
    {
    pub cxt->mtd: *mut *mut mtd_info mtd =,
    pub mtd->erasesize: *mut *mut u32 start_page_offset = mtd_div_by_eb(offset, mtd),
    pub record_size: u32 start_page = start_page_offset /,
    pub record_size: u32 erase_pages = mtd->erasesize /,
    pub erase: erase_info,
    pub ret: c_int,
    pub page: c_int,
    pub offset: erase.addr =,
    pub mtd->erasesize: erase.len =,
    pub &erase): ret = mtd_erase(mtd,,
    if (ret) {
    pr_warn("erase of region [0x%llx, 0x%llx] on \"%s\" failed\n",
    (unsigned long long)erase.addr,
    pub mtddev): (unsigned long long)erase.len,,
    pub ret: return,
    }
// Mark pages as unused
    pub page++): for (page = start_page; page < start_page + erase_pages;,
    pub page): mark_page_unused(cxt,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn mtdoops_erase(cxt: *mut mtdoops_context) {
    static void mtdoops_erase(struct mtdoops_context *cxt)
    {
    pub cxt->mtd: *mut *mut mtd_info mtd =,
    pub mod: int i = 0, j, ret,,
// We were unregistered
    if (!mtd)
    pub mtd->erasesize: *mut *mut mod = (cxt->nextpage  record_size) %,
    if (mod != 0) {
    pub record_size): cxt->nextpage = cxt->nextpage + ((mtd->erasesize - mod) /,
    if (cxt.nextpage >= cxt.oops_pages)
    pub 0: cxt->nextpage =,
    }
    while ((ret = mtd_block_isbad(mtd, cxt.nextpage * record_size)) > 0) {
    badblock:
    pr_warn("bad block at %08lx\n",
    pub record_size): *mut *mut cxt->nextpage,
    pub record_size): cxt->nextpage = cxt->nextpage + (mtd->erasesize /,
    if (cxt.nextpage >= cxt.oops_pages)
    pub 0: cxt->nextpage =,
    if (i == cxt.oops_pages / (mtd.erasesize / record_size)) {
    pub bad!\n"): pr_err("all blocks,
    }
    }
    if (ret < 0) {
    pub aborting\n"): pr_err("mtd_block_isbad failed,,
    }
    pub j++): for (j = 0, ret = -1; (j < 3) && (ret < 0);,
    pub record_size): *mut *mut ret = mtdoops_erase_block(cxt, cxt->nextpage,
    if (ret >= 0) {
    pr_debug("ready %d, %d\n",
    pub cxt->nextcount): cxt->nextpage,,
    }
    if (ret == -EIO) {
    pub record_size): *mut *mut ret = mtd_block_markbad(mtd, cxt->nextpage,
    if (ret < 0 && ret != -EOPNOTSUPP) {
    pub aborting\n"): pr_err("block_markbad failed,,
    }
    }
    pub badblock: goto,
    }
// Scheduled work - when we can't proceed without erasing a block
#[no_mangle]
unsafe extern "C" fn mtdoops_workfunc_erase(work: *mut work_struct) {
    static void mtdoops_workfunc_erase(struct work_struct *work)
    {
    struct mtdoops_context *cxt =
    pub work_erase): container_of(work, struct mtdoops_context,,
    }
#[no_mangle]
unsafe extern "C" fn mtdoops_inc_counter(cxt: *mut mtdoops_context, panic: c_int) {
    static void mtdoops_inc_counter(struct mtdoops_context *cxt, int panic)
    {
    if (cxt.nextpage >= cxt.oops_pages)
    pub 0: cxt->nextpage =,
    if (cxt.nextcount == 0xffffffff)
    pub 0: cxt->nextcount =,
    if (page_is_used(cxt, cxt.nextpage)) {
    pr_debug("not ready %d, %d (erase %s)\n",
    cxt.nextpage, cxt.nextcount,
    pub "scheduled"): panic ? "immediately" :,
    if (panic) {
// In case of panic, erase immediately
    } else {
// Otherwise, schedule work to erase it "nicely"
    }
    } else {
    pr_debug("ready %d, %d (no erase)\n",
    pub cxt->nextcount): cxt->nextpage,,
    }
    }
#[no_mangle]
unsafe extern "C" fn mtdoops_write(cxt: *mut mtdoops_context, panic: c_int) {
    static void mtdoops_write(struct mtdoops_context *cxt, int panic)
    {
    pub cxt->mtd: *mut *mut mtd_info mtd =,
    pub retlen: usize,
    pub hdr: *mut mtdoops_hdr,
    pub ret: c_int,
    if (test_and_set_bit(0, &cxt.oops_buf_busy))
// Add mtdoops header to the buffer
    pub )cxt->oops_buf: *mut hdr = (struct mtdoops_hdr,
    pub cxt->nextcount: hdr->seq =,
    pub MTDOOPS_KERNMSG_MAGIC_v2: hdr->magic =,
    pub ktime_get_real(): hdr->timestamp =,
    if (panic) {
    ret = mtd_panic_write(mtd, cxt.nextpage * record_size,
    pub cxt->oops_buf): record_size, &retlen,,
    if (ret == -EOPNOTSUPP) {
    pub panic_write\n"): pr_err("Cannot write from panic without,
    pub out: goto,
    }
    } else
    ret = mtd_write(mtd, cxt.nextpage * record_size,
    pub cxt->oops_buf): record_size, &retlen,,
    if (retlen != record_size || ret < 0)
    pr_err("write failure at %ld (%td of %ld written), error %d\n",
    pub ret): *mut *mut cxt->nextpage  record_size, retlen, record_size,,
    pub cxt->nextpage): mark_page_used(cxt,,
    pub record_size): memset(cxt->oops_buf, 0xff,,
    pub panic): mtdoops_inc_counter(cxt,,
    out:
    pub &cxt->oops_buf_busy): clear_bit(0,,
    }
#[no_mangle]
unsafe extern "C" fn mtdoops_workfunc_write(work: *mut work_struct) {
    static void mtdoops_workfunc_write(struct work_struct *work)
    {
    struct mtdoops_context *cxt =
    pub work_write): container_of(work, struct mtdoops_context,,
    pub 0): mtdoops_write(cxt,,
    }
#[no_mangle]
unsafe extern "C" fn find_next_position(cxt: *mut mtdoops_context) {
    static void find_next_position(struct mtdoops_context *cxt)
    {
    pub cxt->mtd: *mut *mut mtd_info mtd =,
    pub hdr: mtdoops_hdr,
    pub 0: int ret, page, maxpos =,
    pub 0xffffffff: u32 maxcount =,
    pub retlen: usize,
    pub {: for (page = 0; page < cxt->oops_pages; page++),
    if (mtd_block_isbad(mtd, page * record_size))
// Assume the page is used
    pub page): mark_page_used(cxt,,
    ret = mtd_read(mtd, page * record_size, sizeof(hdr),
    pub )&hdr): *mut &retlen, (u_char,
    if (retlen != sizeof(hdr) ||
    (ret < 0 && !mtd_is_bitflip(ret))) {
    pr_err("read failure at %ld (%zu of %zu read), err %d\n",
    pub ret): *mut *mut page  record_size, retlen, sizeof(hdr),,
    }
    if (hdr.seq == 0xffffffff && hdr.magic == 0xffffffff)
    pub page): mark_page_unused(cxt,,
    if (hdr.seq == 0xffffffff ||
    (hdr.magic != MTDOOPS_KERNMSG_MAGIC_v1 &&
    hdr.magic != MTDOOPS_KERNMSG_MAGIC_v2))
    if (maxcount == 0xffffffff) {
    pub hdr.seq: maxcount =,
    pub page: maxpos =,
    } else if (hdr.seq < 0x40000000 && maxcount > 0xc0000000) {
    pub hdr.seq: maxcount =,
    pub page: maxpos =,
    } else if (hdr.seq > maxcount && hdr.seq < 0xc0000000) {
    pub hdr.seq: maxcount =,
    pub page: maxpos =,
    } else if (hdr.seq > maxcount && hdr.seq > 0xc0000000
    && maxcount > 0x80000000) {
    pub hdr.seq: maxcount =,
    pub page: maxpos =,
    }
    }
    if (maxcount == 0xffffffff) {
    pub 1: cxt->nextpage = cxt->oops_pages -,
    pub 0: cxt->nextcount =,
    }
    else {
    pub maxpos: cxt->nextpage =,
    pub maxcount: cxt->nextcount =,
    }
    pub 0): mtdoops_inc_counter(cxt,,
    }
    static void mtdoops_do_dump(struct kmsg_dumper *dumper,
    struct kmsg_dump_detail *detail)
    {
    struct mtdoops_context *cxt = container_of(dumper,
    pub dump): mtdoops_context,,
    pub iter: kmsg_dump_iter,
// Only dump oopses if dump_oops is set
    if (detail.reason == KMSG_DUMP_OOPS && !dump_oops)
    if (test_and_set_bit(0, &cxt.oops_buf_busy))
    kmsg_dump_get_buffer(&iter, true,
    cxt.oops_buf + sizeof(struct mtdoops_hdr),
    pub NULL): record_size - sizeof(struct mtdoops_hdr),,
    pub &cxt->oops_buf_busy): clear_bit(0,,
    if (detail.reason != KMSG_DUMP_OOPS) {
// Panics must be written immediately
    pub 1): mtdoops_write(cxt,,
    } else {
// For other cases, schedule work to write it "nicely"
    }
    }
#[no_mangle]
unsafe extern "C" fn mtdoops_notify_add(mtd: *mut mtd_info) {
    static void mtdoops_notify_add(struct mtd_info *mtd)
    {
    pub &oops_cxt: *mut *mut mtdoops_context cxt =,
    pub record_size): u64 mtdoops_pages = div_u64(mtd->size,,
    pub err: c_int,
    if (!strcmp(mtd.name, mtddev))
    pub mtd->index: cxt->mtd_index =,
    if (mtd.index != cxt.mtd_index || cxt.mtd_index < 0)
    if (mtd.size < mtd.erasesize * 2) {
    pr_err("MTD partition %d not big enough for mtdoops\n",
    }
    if (mtd.erasesize < record_size) {
    pr_err("eraseblock size of MTD partition %d too small\n",
    }
    if (mtd.size > MTDOOPS_MAX_MTD_SIZE) {
    pr_err("mtd%d is too large (limit is %d MiB)\n",
    pub 1024): mtd->index, MTDOOPS_MAX_MTD_SIZE / 1024 /,
    }
// oops_page_used is a bit field
    cxt.oops_page_used =
    vmalloc_array(DIV_ROUND_UP(mtdoops_pages, BITS_PER_LONG),
    pub long)): sizeof(unsigned,
    if (!cxt.oops_page_used) {
    pub array\n"): pr_err("could not allocate page,
    }
    pub KMSG_DUMP_OOPS: cxt->dump.max_reason =,
    pub mtdoops_do_dump: cxt->dump.dump =,
    pub kmsg_dump_register(&cxt->dump): err =,
    if (err) {
    pub err): pr_err("registering kmsg dumper failed, error %d\n",,
    pub NULL: cxt->oops_page_used =,
    }
    pub mtd: cxt->mtd =,
    pub record_size: cxt->oops_pages = (int)mtd->size /,
    pub mtd->index): pr_info("Attached to MTD device %d\n",,
    }
#[no_mangle]
unsafe extern "C" fn mtdoops_notify_remove(mtd: *mut mtd_info) {
    static void mtdoops_notify_remove(struct mtd_info *mtd)
    {
    pub &oops_cxt: *mut *mut mtdoops_context cxt =,
    if (mtd.index != cxt.mtd_index || cxt.mtd_index < 0)
    if (kmsg_dump_unregister(&cxt.dump) < 0)
    pub kmsg_dumper\n"): pr_warn("could not unregister,
    pub NULL: cxt->mtd =,
    pub NULL: cxt->oops_page_used =,
    pub 0: cxt->oops_pages =,
    }
    static struct mtd_notifier mtdoops_notifier = {
    .add	= mtdoops_notify_add,
    .remove	= mtdoops_notify_remove,
}

#[no_mangle]
unsafe extern "C" fn mtdoops_init() -> int __init {
    static int __init mtdoops_init(void)
    {
    struct mtdoops_context *cxt = &oops_cxt;
    unsigned int mtd_index;
    if (strlen(mtddev) == 0) {
    pr_err("mtd device (mtddev=name/number) must be supplied\n");
    return -EINVAL;
    }
    if ((record_size & 4095) != 0) {
    pr_err("record_size must be a multiple of 4096\n");
    return -EINVAL;
    }
    if (record_size < 4096) {
    pr_err("record_size must be over 4096 bytes\n");
    return -EINVAL;
    }
// Setup the MTD device to use
    cxt.mtd_index = -1;
    if (kstrtouint(mtddev, 0, &mtd_index) == 0) {
    cxt.mtd_index = mtd_index;
    }
    cxt.oops_buf = vmalloc(record_size);
    if (!cxt.oops_buf)
    return -ENOMEM;
    memset(cxt.oops_buf, 0xff, record_size);
    cxt.oops_buf_busy = 0;
    INIT_WORK(&cxt.work_erase, mtdoops_workfunc_erase);
    INIT_WORK(&cxt.work_write, mtdoops_workfunc_write);
    register_mtd_user(&mtdoops_notifier);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtdoops_exit() -> void __exit {
    static void __exit mtdoops_exit(void)
    {
    struct mtdoops_context *cxt = &oops_cxt;
    unregister_mtd_user(&mtdoops_notifier);
    vfree(cxt.oops_buf);
    vfree(cxt.oops_page_used);
    }
    module_init(mtdoops_init);
    module_exit(mtdoops_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Richard Purdie <rpurdie@openedhand.com>");
    MODULE_DESCRIPTION("MTD Oops/Panic console logger/driver");

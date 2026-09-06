//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/mtdswap.c
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
// Swap block device support for MTDs
// Turns an MTD device into a swap device with block wear leveling
//
// Copyright © 2007,2011 Nokia Corporation. All rights reserved.
//
// Authors: Jarkko Lavinen <jarkko.lavinen@nokia.com>
//
// Based on Richard Purdie's earlier implementation in 2007. Background
// support and lock-less operation written by Adrian Hunter.
//

//
// The number of free eraseblocks when GC should stop
//
pub const CLEAN_BLOCK_THRESHOLD: c_int = 20;
//
// Number of free eraseblocks below which GC can also collect low frag
// blocks.
//
pub const LOW_FRAG_GC_THRESHOLD: c_int = 5;
//
// Wear level cost amortization. We want to do wear leveling on the background
// without disturbing gc too much. This is made by defining max GC frequency.
// Frequency value 6 means 1/6 of the GC passes will pick an erase block based
// on the biggest wear difference rather than the biggest dirtiness.
//
// The lower freq2 should be chosen so that it makes sure the maximum erase
// difference will decrease even if a malicious application is deliberately
// trying to make erase differences large.
//
pub const MAX_ERASE_DIFF: c_int = 4000;

pub const COLLECT_NONDIRTY_FREQ1: c_int = 6;
pub const COLLECT_NONDIRTY_FREQ2: c_int = 4;

pub const EBLOCK_IDX_SHIFT: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swap_eb {
    pub rb: rb_node,
    pub root: *mut rb_root,
    pub flags: c_uint,
    pub active_count: c_uint,
    pub erase_count: c_uint,
    pub /: *mut *mut unsigned int pad; / speeds up pointer decrement,
}

    rb).erase_count)

    rb).erase_count)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtdswap_tree {
    pub root: rb_root,
    pub count: c_uint,
}

    enum {
    MTDSWAP_CLEAN,
    MTDSWAP_USED,
    MTDSWAP_LOWFRAG,
    MTDSWAP_HIFRAG,
    MTDSWAP_DIRTY,
    MTDSWAP_BITFLIP,
    MTDSWAP_FAILING,
    MTDSWAP_TREE_CNT,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtdswap_dev {
    pub mbd_dev: *mut mtd_blktrans_dev,
    pub mtd: *mut mtd_info,
    pub dev: *mut device,
    pub page_data: *mut c_uint,
    pub revmap: *mut c_uint,
    pub eblks: c_uint,
    pub spare_eblks: c_uint,
    pub pages_per_eblk: c_uint,
    pub max_erase_count: c_uint,
    pub eb_data: *mut swap_eb,
    pub trees: [mtdswap_tree; MTDSWAP_TREE_CNT],
    pub sect_read_count: c_ulonglong,
    pub sect_write_count: c_ulonglong,
    pub mtd_write_count: c_ulonglong,
    pub mtd_read_count: c_ulonglong,
    pub discard_count: c_ulonglong,
    pub discard_page_count: c_ulonglong,
    pub curr_write_pos: c_uint,
    pub curr_write: *mut swap_eb,
    pub page_buf: *mut c_char,
    pub oob_buf: *mut c_char,
    pub debugfs_stats: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtdswap_oobdata {
    pub magic: __le16,
    pub count: __le32,
    pub __packed: },
pub const MTDSWAP_MAGIC_CLEAN: c_uint = 0x2095;

pub const MTDSWAP_TYPE_CLEAN: c_int = 0;
pub const MTDSWAP_TYPE_DIRTY: c_int = 1;

pub const MTDSWAP_IO_RETRIES: c_int = 3;
    enum {
    MTDSWAP_SCANNED_CLEAN,
    MTDSWAP_SCANNED_DIRTY,
    MTDSWAP_SCANNED_BITFLIP,
    MTDSWAP_SCANNED_BAD,
}

//
// In the worst case mtdswap_writesect() has allocated the last clean
// page from the current block and is then pre-empted by the GC
// thread. The thread can consume a full erase block when moving a
// block.
//
pub const MIN_SPARE_EBLOCKS: c_int = 2;

    static char partitions[128] = "";
    module_param_string(partitions, partitions, sizeof(partitions), 0444);
    MODULE_PARM_DESC(partitions, "MTD partition numbers to use as swap "
    "partitions=\"1,3,5\"");
    let mut spare_eblocks: static unsigned int = 10;
    module_param(spare_eblocks, uint, 0444);
    MODULE_PARM_DESC(spare_eblocks, "Percentage of spare erase blocks for "
    "garbage collection (default 10%)");
    static bool header; /* false */
    module_param(header, bool, 0444);
    MODULE_PARM_DESC(header,
    "Include builtin swap header (default 0, without header)");
    static int mtdswap_gc(struct mtdswap_dev *d, unsigned int background);
#[no_mangle]
unsafe extern "C" fn mtdswap_eb_offset(d: *mut mtdswap_dev, eb: *mut swap_eb) -> loff_t {
    static loff_t mtdswap_eb_offset(struct mtdswap_dev *d, struct swap_eb *eb)
    {
    return (loff_t)(eb - d.eb_data) * d.mtd.erasesize;
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_eb_detach(d: *mut mtdswap_dev, eb: *mut swap_eb) {
    static void mtdswap_eb_detach(struct mtdswap_dev *d, struct swap_eb *eb)
    {
    unsigned int oldidx;
    struct mtdswap_tree *tp;
    if (eb.root) {
    tp = container_of(eb.root, struct mtdswap_tree, root);
    oldidx = tp - &d.trees[0];
    d.trees[oldidx].count--;
    rb_erase(&eb.rb, eb.root);
    }
    }
#[no_mangle]
unsafe extern "C" fn __mtdswap_rb_add(root: *mut rb_root, eb: *mut swap_eb) {
    static void __mtdswap_rb_add(struct rb_root *root, struct swap_eb *eb)
    {
    struct rb_node **p, *parent = core::ptr::null_mut();
    struct swap_eb *cur;
    p = &root.rb_node;
    while (*p) {
    parent = *p;
    cur = rb_entry(parent, struct swap_eb, rb);
    if (eb.erase_count > cur.erase_count)
    p = &(*p).rb_right;
    else
    p = &(*p).rb_left;
    }
    rb_link_node(&eb.rb, parent, p);
    rb_insert_color(&eb.rb, root);
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_rb_add(d: *mut mtdswap_dev, eb: *mut swap_eb, idx: c_int) {
    static void mtdswap_rb_add(struct mtdswap_dev *d, struct swap_eb *eb, int idx)
    {
    struct rb_root *root;
    if (eb.root == &d.trees[idx].root)
    return;
    mtdswap_eb_detach(d, eb);
    root = &d.trees[idx].root;
    __mtdswap_rb_add(root, eb);
    eb.root = root;
    d.trees[idx].count++;
    }
    static struct rb_node *mtdswap_rb_index(struct rb_root *root, unsigned int idx)
    {
    struct rb_node *p;
    unsigned int i;
    p = rb_first(root);
    i = 0;
    while (i < idx && p) {
    p = rb_next(p);
    i++;
    }
    return p;
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_handle_badblock(d: *mut mtdswap_dev, eb: *mut swap_eb) -> c_int {
    static int mtdswap_handle_badblock(struct mtdswap_dev *d, struct swap_eb *eb)
    {
    int ret;
    loff_t offset;
    d.spare_eblks--;
    eb.flags |= EBLOCK_BAD;
    mtdswap_eb_detach(d, eb);
    eb.root = core::ptr::null_mut();
// badblocks not supported
    if (!mtd_can_have_bb(d.mtd))
    return 1;
    offset = mtdswap_eb_offset(d, eb);
    dev_warn(d.dev, "Marking bad block at %08llx\n", offset);
    ret = mtd_block_markbad(d.mtd, offset);
    if (ret) {
    dev_warn(d.dev, "Mark block bad failed for block at %08llx "
    "error %d\n", offset, ret);
    return ret;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_handle_write_error(d: *mut mtdswap_dev, eb: *mut swap_eb) -> c_int {
    static int mtdswap_handle_write_error(struct mtdswap_dev *d, struct swap_eb *eb)
    {
    let mut marked: c_uint = eb.flags & EBLOCK_FAILED;
    struct swap_eb *curr_write = d.curr_write;
    eb.flags |= EBLOCK_FAILED;
    if (curr_write == eb) {
    d.curr_write = core::ptr::null_mut();
    if (!marked && d.curr_write_pos != 0) {
    mtdswap_rb_add(d, eb, MTDSWAP_FAILING);
    return 0;
    }
    }
    return mtdswap_handle_badblock(d, eb);
    }
    static int mtdswap_read_oob(struct mtdswap_dev *d, loff_t from,
    struct mtd_oob_ops *ops)
    {
    let mut ret: c_int = mtd_read_oob(d.mtd, from, ops);
    if (mtd_is_bitflip(ret))
    return ret;
    if (ret) {
    dev_warn(d.dev, "Read OOB failed %d for block at %08llx\n",
    ret, from);
    return ret;
    }
    if (ops.oobretlen < ops.ooblen) {
    dev_warn(d.dev, "Read OOB return short read (%zd bytes not "
    "%zd) for block at %08llx\n",
    ops.oobretlen, ops.ooblen, from);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_read_markers(d: *mut mtdswap_dev, eb: *mut swap_eb) -> c_int {
    static int mtdswap_read_markers(struct mtdswap_dev *d, struct swap_eb *eb)
    {
    struct mtdswap_oobdata *data, *data2;
    int ret;
    loff_t offset;
    let mut ops: mtd_oob_ops = { };
    offset = mtdswap_eb_offset(d, eb);
// Check first if the block is bad.
    if (mtd_can_have_bb(d.mtd) && mtd_block_isbad(d.mtd, offset))
    return MTDSWAP_SCANNED_BAD;
    ops.ooblen = 2 * d.mtd.oobavail;
    ops.oobbuf = d.oob_buf;
    ops.ooboffs = 0;
    ops.datbuf = core::ptr::null_mut();
    ops.mode = MTD_OPS_AUTO_OOB;
    ret = mtdswap_read_oob(d, offset, &ops);
    if (ret && !mtd_is_bitflip(ret))
    return ret;
    data = (struct mtdswap_oobdata *)d.oob_buf;
    data2 = (struct mtdswap_oobdata *)
    (d.oob_buf + d.mtd.oobavail);
    if (le16_to_cpu(data.magic) == MTDSWAP_MAGIC_CLEAN) {
    eb.erase_count = le32_to_cpu(data.count);
    if (mtd_is_bitflip(ret))
    ret = MTDSWAP_SCANNED_BITFLIP;
    else {
    if (le16_to_cpu(data2.magic) == MTDSWAP_MAGIC_DIRTY)
    ret = MTDSWAP_SCANNED_DIRTY;
    else
    ret = MTDSWAP_SCANNED_CLEAN;
    }
    } else {
    eb.flags |= EBLOCK_NOMAGIC;
    ret = MTDSWAP_SCANNED_DIRTY;
    }
    return ret;
    }
    static int mtdswap_write_marker(struct mtdswap_dev *d, struct swap_eb *eb,
    u16 marker)
    {
    struct mtdswap_oobdata n;
    int ret;
    loff_t offset;
    let mut ops: mtd_oob_ops = { };
    ops.ooboffs = 0;
    ops.oobbuf = (uint8_t *)&n;
    ops.mode = MTD_OPS_AUTO_OOB;
    ops.datbuf = core::ptr::null_mut();
    if (marker == MTDSWAP_TYPE_CLEAN) {
    n.magic = cpu_to_le16(MTDSWAP_MAGIC_CLEAN);
    n.count = cpu_to_le32(eb.erase_count);
    ops.ooblen = MTDSWAP_OOBSIZE;
    offset = mtdswap_eb_offset(d, eb);
    } else {
    n.magic = cpu_to_le16(MTDSWAP_MAGIC_DIRTY);
    ops.ooblen = sizeof(n.magic);
    offset = mtdswap_eb_offset(d, eb) + d.mtd.writesize;
    }
    ret = mtd_write_oob(d.mtd, offset, &ops);
    if (ret) {
    dev_warn(d.dev, "Write OOB failed for block at %08llx "
    "error %d\n", offset, ret);
    if (ret == -EIO || mtd_is_eccerr(ret))
    mtdswap_handle_write_error(d, eb);
    return ret;
    }
    if (ops.oobretlen != ops.ooblen) {
    dev_warn(d.dev, "Short OOB write for block at %08llx: "
    "%zd not %zd\n",
    offset, ops.oobretlen, ops.ooblen);
    return ret;
    }
    return 0;
    }
//
// Are there any erase blocks without MAGIC_CLEAN header, presumably
// because power was cut off after erase but before header write? We
// need to guestimate the erase count.
//
#[no_mangle]
unsafe extern "C" fn mtdswap_check_counts(d: *mut mtdswap_dev) {
    static void mtdswap_check_counts(struct mtdswap_dev *d)
    {
    let mut hist_root: rb_root = RB_ROOT;
    struct rb_node *medrb;
    struct swap_eb *eb;
    unsigned int i, cnt, median;
    cnt = 0;
    for (i = 0; i < d.eblks; i++) {
    eb = d.eb_data + i;
    if (eb.flags & (EBLOCK_NOMAGIC | EBLOCK_BAD | EBLOCK_READERR))
    continue;
    __mtdswap_rb_add(&hist_root, eb);
    cnt++;
    }
    if (cnt == 0)
    return;
    medrb = mtdswap_rb_index(&hist_root, cnt / 2);
    median = rb_entry(medrb, struct swap_eb, rb).erase_count;
    d.max_erase_count = MTDSWAP_ECNT_MAX(&hist_root);
    for (i = 0; i < d.eblks; i++) {
    eb = d.eb_data + i;
    if (eb.flags & (EBLOCK_NOMAGIC | EBLOCK_READERR))
    eb.erase_count = median;
    if (eb.flags & (EBLOCK_NOMAGIC | EBLOCK_BAD | EBLOCK_READERR))
    continue;
    rb_erase(&eb.rb, &hist_root);
    }
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_scan_eblks(d: *mut mtdswap_dev) {
    static void mtdswap_scan_eblks(struct mtdswap_dev *d)
    {
    int status;
    unsigned int i, idx;
    struct swap_eb *eb;
    for (i = 0; i < d.eblks; i++) {
    eb = d.eb_data + i;
    status = mtdswap_read_markers(d, eb);
    if (status < 0)
    eb.flags |= EBLOCK_READERR;
#[no_mangle]
pub unsafe extern "C" fn if(MTDSWAP_SCANNED_BAD: status ==) -> else {
    eb.flags |= EBLOCK_BAD;
    continue;
    }
    switch (status) {
    case MTDSWAP_SCANNED_CLEAN:
    idx = MTDSWAP_CLEAN;
    break;
    case MTDSWAP_SCANNED_DIRTY:
    case MTDSWAP_SCANNED_BITFLIP:
    idx = MTDSWAP_DIRTY;
    break;
    default:
    idx = MTDSWAP_FAILING;
    }
    eb.flags |= (idx << EBLOCK_IDX_SHIFT);
    }
    mtdswap_check_counts(d);
    for (i = 0; i < d.eblks; i++) {
    eb = d.eb_data + i;
    if (eb.flags & EBLOCK_BAD)
    continue;
    idx = eb.flags >> EBLOCK_IDX_SHIFT;
    mtdswap_rb_add(d, eb, idx);
    }
    }
//
// Place eblk into a tree corresponding to its number of active blocks
// it contains.
//
#[no_mangle]
unsafe extern "C" fn mtdswap_store_eb(d: *mut mtdswap_dev, eb: *mut swap_eb) {
    static void mtdswap_store_eb(struct mtdswap_dev *d, struct swap_eb *eb)
    {
    let mut weight: c_uint = eb.active_count;
    let mut maxweight: c_uint = d.pages_per_eblk;
    if (eb == d.curr_write)
    return;
    if (eb.flags & EBLOCK_BITFLIP)
    mtdswap_rb_add(d, eb, MTDSWAP_BITFLIP);
#[no_mangle]
pub unsafe extern "C" fn if(EBLOCK_FAILED): eb->flags & (EBLOCK_READERR |) -> else {
    else if (eb.flags & (EBLOCK_READERR | EBLOCK_FAILED))
    mtdswap_rb_add(d, eb, MTDSWAP_FAILING);
    if (weight == maxweight)
    mtdswap_rb_add(d, eb, MTDSWAP_USED);
#[no_mangle]
pub unsafe extern "C" fn if(0: weight ==) -> else {
    else if (weight == 0)
    mtdswap_rb_add(d, eb, MTDSWAP_DIRTY);
#[no_mangle]
pub unsafe extern "C" fn if((maxweight/2): weight >) -> else {
    else if (weight > (maxweight/2))
    mtdswap_rb_add(d, eb, MTDSWAP_LOWFRAG);
    else
    mtdswap_rb_add(d, eb, MTDSWAP_HIFRAG);
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_erase_block(d: *mut mtdswap_dev, eb: *mut swap_eb) -> c_int {
    static int mtdswap_erase_block(struct mtdswap_dev *d, struct swap_eb *eb)
    {
    struct mtd_info *mtd = d.mtd;
    struct erase_info erase;
    let mut retries: c_uint = 0;
    int ret;
    eb.erase_count++;
    if (eb.erase_count > d.max_erase_count)
    d.max_erase_count = eb.erase_count;
    retry:
    memset(&erase, 0, sizeof(struct erase_info));
    erase.addr	= mtdswap_eb_offset(d, eb);
    erase.len	= mtd.erasesize;
    ret = mtd_erase(mtd, &erase);
    if (ret) {
    if (retries++ < MTDSWAP_ERASE_RETRIES) {
    dev_warn(d.dev,
    "erase of erase block %#llx on %s failed",
    erase.addr, mtd.name);
    yield();
    goto retry;
    }
    dev_err(d.dev, "Cannot erase erase block %#llx on %s\n",
    erase.addr, mtd.name);
    mtdswap_handle_badblock(d, eb);
    return -EIO;
    }
    return 0;
    }
    static int mtdswap_map_free_block(struct mtdswap_dev *d, unsigned int page,
    unsigned int *block)
    {
    int ret;
    struct swap_eb *old_eb = d.curr_write;
    struct rb_root *clean_root;
    struct swap_eb *eb;
    if (old_eb == core::ptr::null_mut() || d.curr_write_pos >= d.pages_per_eblk) {
    do {
    if (TREE_EMPTY(d, CLEAN))
    return -ENOSPC;
    clean_root = TREE_ROOT(d, CLEAN);
    eb = rb_entry(rb_first(clean_root), struct swap_eb, rb);
    rb_erase(&eb.rb, clean_root);
    eb.root = core::ptr::null_mut();
    TREE_COUNT(d, CLEAN)--;
    ret = mtdswap_write_marker(d, eb, MTDSWAP_TYPE_DIRTY);
    } while (ret == -EIO || mtd_is_eccerr(ret));
    if (ret)
    return ret;
    d.curr_write_pos = 0;
    d.curr_write = eb;
    if (old_eb)
    mtdswap_store_eb(d, old_eb);
    }
// block = (d->curr_write - d->eb_data) * d->pages_per_eblk +
    d.curr_write_pos;
    d.curr_write.active_count++;
    d.revmap[*block] = page;
    d.curr_write_pos++;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_free_page_cnt(d: *mut mtdswap_dev) -> c_uint {
    static unsigned int mtdswap_free_page_cnt(struct mtdswap_dev *d)
    {
    return TREE_COUNT(d, CLEAN) * d.pages_per_eblk +
    d.pages_per_eblk - d.curr_write_pos;
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_enough_free_pages(d: *mut mtdswap_dev) -> c_uint {
    static unsigned int mtdswap_enough_free_pages(struct mtdswap_dev *d)
    {
    return mtdswap_free_page_cnt(d) > d.pages_per_eblk;
    }
    static int mtdswap_write_block(struct mtdswap_dev *d, char *buf,
    unsigned int page, unsigned int *bp, int gc_context)
    {
    struct mtd_info *mtd = d.mtd;
    struct swap_eb *eb;
    size_t retlen;
    loff_t writepos;
    int ret;
    retry:
    if (!gc_context)
    while (!mtdswap_enough_free_pages(d))
    if (mtdswap_gc(d, 0) > 0)
    return -ENOSPC;
    ret = mtdswap_map_free_block(d, page, bp);
    eb = d.eb_data + (*bp / d.pages_per_eblk);
    if (ret == -EIO || mtd_is_eccerr(ret)) {
    d.curr_write = core::ptr::null_mut();
    eb.active_count--;
    d.revmap[*bp] = PAGE_UNDEF;
    goto retry;
    }
    if (ret < 0)
    return ret;
    writepos = (loff_t)*bp << PAGE_SHIFT;
    ret =  mtd_write(mtd, writepos, PAGE_SIZE, &retlen, buf);
    if (ret == -EIO || mtd_is_eccerr(ret)) {
    d.curr_write_pos--;
    eb.active_count--;
    d.revmap[*bp] = PAGE_UNDEF;
    mtdswap_handle_write_error(d, eb);
    goto retry;
    }
    if (ret < 0) {
    dev_err(d.dev, "Write to MTD device failed: %d (%zd written)",
    ret, retlen);
    goto err;
    }
    if (retlen != PAGE_SIZE) {
    dev_err(d.dev, "Short write to MTD device: %zd written",
    retlen);
    ret = -EIO;
    goto err;
    }
    return ret;
    err:
    d.curr_write_pos--;
    eb.active_count--;
    d.revmap[*bp] = PAGE_UNDEF;
    return ret;
    }
    static int mtdswap_move_block(struct mtdswap_dev *d, unsigned int oldblock,
    unsigned int *newblock)
    {
    struct mtd_info *mtd = d.mtd;
    struct swap_eb *eb, *oldeb;
    int ret;
    size_t retlen;
    unsigned int page, retries;
    loff_t readpos;
    page = d.revmap[oldblock];
    readpos = (loff_t) oldblock << PAGE_SHIFT;
    retries = 0;
    retry:
    ret = mtd_read(mtd, readpos, PAGE_SIZE, &retlen, d.page_buf);
    if (ret < 0 && !mtd_is_bitflip(ret)) {
    oldeb = d.eb_data + oldblock / d.pages_per_eblk;
    oldeb.flags |= EBLOCK_READERR;
    dev_err(d.dev, "Read Error: %d (block %u)\n", ret,
    oldblock);
    retries++;
    if (retries < MTDSWAP_IO_RETRIES)
    goto retry;
    goto read_error;
    }
    if (retlen != PAGE_SIZE) {
    dev_err(d.dev, "Short read: %zd (block %u)\n", retlen,
    oldblock);
    ret = -EIO;
    goto read_error;
    }
    ret = mtdswap_write_block(d, d.page_buf, page, newblock, 1);
    if (ret < 0) {
    d.page_data[page] = BLOCK_ERROR;
    dev_err(d.dev, "Write error: %d\n", ret);
    return ret;
    }
    d.page_data[page] = *newblock;
    d.revmap[oldblock] = PAGE_UNDEF;
    eb = d.eb_data + oldblock / d.pages_per_eblk;
    eb.active_count--;
    return 0;
    read_error:
    d.page_data[page] = BLOCK_ERROR;
    d.revmap[oldblock] = PAGE_UNDEF;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_gc_eblock(d: *mut mtdswap_dev, eb: *mut swap_eb) -> c_int {
    static int mtdswap_gc_eblock(struct mtdswap_dev *d, struct swap_eb *eb)
    {
    unsigned int i, block, eblk_base, newblock;
    int ret, errcode;
    errcode = 0;
    eblk_base = (eb - d.eb_data) * d.pages_per_eblk;
    for (i = 0; i < d.pages_per_eblk; i++) {
    if (d.spare_eblks < MIN_SPARE_EBLOCKS)
    return -ENOSPC;
    block = eblk_base + i;
    if (d.revmap[block] == PAGE_UNDEF)
    continue;
    ret = mtdswap_move_block(d, block, &newblock);
    if (ret < 0 && !errcode)
    errcode = ret;
    }
    return errcode;
    }
#[no_mangle]
unsafe extern "C" fn __mtdswap_choose_gc_tree(d: *mut mtdswap_dev) -> c_int {
    static int __mtdswap_choose_gc_tree(struct mtdswap_dev *d)
    {
    int idx, stopat;
    if (TREE_COUNT(d, CLEAN) < LOW_FRAG_GC_THRESHOLD)
    stopat = MTDSWAP_LOWFRAG;
    else
    stopat = MTDSWAP_HIFRAG;
    for (idx = MTDSWAP_BITFLIP; idx >= stopat; idx--)
    if (d.trees[idx].root.rb_node != core::ptr::null_mut())
    return idx;
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_wlfreq(maxdiff: c_uint) -> c_int {
    static int mtdswap_wlfreq(unsigned int maxdiff)
    {
    unsigned int h, x, y, dist, base;
//
// Calculate linear ramp down from f1 to f2 when maxdiff goes from
// MAX_ERASE_DIFF to MAX_ERASE_DIFF + COLLECT_NONDIRTY_BASE.  Similar
// to triangle with height f1 - f1 and width COLLECT_NONDIRTY_BASE.
//
    dist = maxdiff - MAX_ERASE_DIFF;
    if (dist > COLLECT_NONDIRTY_BASE)
    dist = COLLECT_NONDIRTY_BASE;
//
// Modelling the slop as right angular triangle with base
// COLLECT_NONDIRTY_BASE and height freq1 - freq2. The ratio y/x is
// equal to the ratio h/base.
//
    h = COLLECT_NONDIRTY_FREQ1 - COLLECT_NONDIRTY_FREQ2;
    base = COLLECT_NONDIRTY_BASE;
    x = dist - base;
    y = (x * h + base / 2) / base;
    return COLLECT_NONDIRTY_FREQ2 + y;
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_choose_wl_tree(d: *mut mtdswap_dev) -> c_int {
    static int mtdswap_choose_wl_tree(struct mtdswap_dev *d)
    {
    static unsigned int pick_cnt;
    unsigned int i, idx = -1, wear, max;
    struct rb_root *root;
    max = 0;
    for (i = 0; i <= MTDSWAP_DIRTY; i++) {
    root = &d.trees[i].root;
    if (root.rb_node == core::ptr::null_mut())
    continue;
    wear = d.max_erase_count - MTDSWAP_ECNT_MIN(root);
    if (wear > max) {
    max = wear;
    idx = i;
    }
    }
    if (max > MAX_ERASE_DIFF && pick_cnt >= mtdswap_wlfreq(max) - 1) {
    pick_cnt = 0;
    return idx;
    }
    pick_cnt++;
    return -1;
    }
    static int mtdswap_choose_gc_tree(struct mtdswap_dev *d,
    unsigned int background)
    {
    int idx;
    if (TREE_NONEMPTY(d, FAILING) &&
    (background || (TREE_EMPTY(d, CLEAN) && TREE_EMPTY(d, DIRTY))))
    return MTDSWAP_FAILING;
    idx = mtdswap_choose_wl_tree(d);
    if (idx >= MTDSWAP_CLEAN)
    return idx;
    return __mtdswap_choose_gc_tree(d);
    }
    static struct swap_eb *mtdswap_pick_gc_eblk(struct mtdswap_dev *d,
    unsigned int background)
    {
    struct rb_root *rp = core::ptr::null_mut();
    struct swap_eb *eb = core::ptr::null_mut();
    int idx;
    if (background && TREE_COUNT(d, CLEAN) > CLEAN_BLOCK_THRESHOLD &&
    TREE_EMPTY(d, DIRTY) && TREE_EMPTY(d, FAILING))
    return core::ptr::null_mut();
    idx = mtdswap_choose_gc_tree(d, background);
    if (idx < 0)
    return core::ptr::null_mut();
    rp = &d.trees[idx].root;
    eb = rb_entry(rb_first(rp), struct swap_eb, rb);
    rb_erase(&eb.rb, rp);
    eb.root = core::ptr::null_mut();
    d.trees[idx].count--;
    return eb;
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_test_patt(i: c_uint) -> c_uint {
    static unsigned int mtdswap_test_patt(unsigned int i)
    {
    return i % 2 ? 0x55555555 : 0xAAAAAAAA;
    }
    static unsigned int mtdswap_eblk_passes(struct mtdswap_dev *d,
    struct swap_eb *eb)
    {
    struct mtd_info *mtd = d.mtd;
    unsigned int test, i, j, patt, mtd_pages;
    loff_t base, pos;
    unsigned int *p1 = (unsigned int *)d.page_buf;
    unsigned char *p2 = (unsigned char *)d.oob_buf;
    let mut ops: mtd_oob_ops = { };
    int ret;
    ops.mode = MTD_OPS_AUTO_OOB;
    ops.len = mtd.writesize;
    ops.ooblen = mtd.oobavail;
    ops.ooboffs = 0;
    ops.datbuf = d.page_buf;
    ops.oobbuf = d.oob_buf;
    base = mtdswap_eb_offset(d, eb);
    mtd_pages = d.pages_per_eblk * PAGE_SIZE / mtd.writesize;
    for (test = 0; test < 2; test++) {
    pos = base;
    for (i = 0; i < mtd_pages; i++) {
    patt = mtdswap_test_patt(test + i);
    memset(d.page_buf, patt, mtd.writesize);
    memset(d.oob_buf, patt, mtd.oobavail);
    ret = mtd_write_oob(mtd, pos, &ops);
    if (ret)
    goto error;
    pos += mtd.writesize;
    }
    pos = base;
    for (i = 0; i < mtd_pages; i++) {
    ret = mtd_read_oob(mtd, pos, &ops);
    if (ret)
    goto error;
    patt = mtdswap_test_patt(test + i);
    for (j = 0; j < mtd.writesize/sizeof(int); j++)
    if (p1[j] != patt)
    goto error;
    for (j = 0; j < mtd.oobavail; j++)
    if (p2[j] != (unsigned char)patt)
    goto error;
    pos += mtd.writesize;
    }
    ret = mtdswap_erase_block(d, eb);
    if (ret)
    goto error;
    }
    eb.flags &= ~EBLOCK_READERR;
    return 1;
    error:
    mtdswap_handle_badblock(d, eb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_gc(d: *mut mtdswap_dev, background: c_uint) -> c_int {
    static int mtdswap_gc(struct mtdswap_dev *d, unsigned int background)
    {
    struct swap_eb *eb;
    int ret;
    if (d.spare_eblks < MIN_SPARE_EBLOCKS)
    return 1;
    eb = mtdswap_pick_gc_eblk(d, background);
    if (!eb)
    return 1;
    ret = mtdswap_gc_eblock(d, eb);
    if (ret == -ENOSPC)
    return 1;
    if (eb.flags & EBLOCK_FAILED) {
    mtdswap_handle_badblock(d, eb);
    return 0;
    }
    eb.flags &= ~EBLOCK_BITFLIP;
    ret = mtdswap_erase_block(d, eb);
    if ((eb.flags & EBLOCK_READERR) &&
    (ret || !mtdswap_eblk_passes(d, eb)))
    return 0;
    if (ret == 0)
    ret = mtdswap_write_marker(d, eb, MTDSWAP_TYPE_CLEAN);
    if (ret == 0)
    mtdswap_rb_add(d, eb, MTDSWAP_CLEAN);
#[no_mangle]
pub unsafe extern "C" fn if(!mtd_is_eccerr(ret): ret != -EIO &&) -> else {
    else if (ret != -EIO && !mtd_is_eccerr(ret))
    mtdswap_rb_add(d, eb, MTDSWAP_DIRTY);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_background(dev: *mut mtd_blktrans_dev) {
    static void mtdswap_background(struct mtd_blktrans_dev *dev)
    {
    struct mtdswap_dev *d = MTDSWAP_MBD_TO_MTDSWAP(dev);
    int ret;
    while (1) {
    ret = mtdswap_gc(d, 1);
    if (ret || mtd_blktrans_cease_background(dev))
    return;
    }
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_cleanup(d: *mut mtdswap_dev) {
    static void mtdswap_cleanup(struct mtdswap_dev *d)
    {
    vfree(d.eb_data);
    vfree(d.revmap);
    vfree(d.page_data);
    kfree(d.oob_buf);
    kfree(d.page_buf);
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_flush(dev: *mut mtd_blktrans_dev) -> c_int {
    static int mtdswap_flush(struct mtd_blktrans_dev *dev)
    {
    struct mtdswap_dev *d = MTDSWAP_MBD_TO_MTDSWAP(dev);
    mtd_sync(d.mtd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_badblocks(mtd: *mut mtd_info, size: u64) -> c_uint {
    static unsigned int mtdswap_badblocks(struct mtd_info *mtd, uint64_t size)
    {
    loff_t offset;
    unsigned int badcnt;
    badcnt = 0;
    if (mtd_can_have_bb(mtd))
    for (offset = 0; offset < size; offset += mtd.erasesize)
    if (mtd_block_isbad(mtd, offset))
    badcnt++;
    return badcnt;
    }
    static int mtdswap_writesect(struct mtd_blktrans_dev *dev,
    unsigned long page, char *buf)
    {
    struct mtdswap_dev *d = MTDSWAP_MBD_TO_MTDSWAP(dev);
    unsigned int newblock, mapped;
    struct swap_eb *eb;
    int ret;
    d.sect_write_count++;
    if (d.spare_eblks < MIN_SPARE_EBLOCKS)
    return -ENOSPC;
    if (header) {
// Ignore writes to the header page
    if (unlikely(page == 0))
    return 0;
    page--;
    }
    mapped = d.page_data[page];
    if (mapped <= BLOCK_MAX) {
    eb = d.eb_data + (mapped / d.pages_per_eblk);
    eb.active_count--;
    mtdswap_store_eb(d, eb);
    d.page_data[page] = BLOCK_UNDEF;
    d.revmap[mapped] = PAGE_UNDEF;
    }
    ret = mtdswap_write_block(d, buf, page, &newblock, 0);
    d.mtd_write_count++;
    if (ret < 0)
    return ret;
    d.page_data[page] = newblock;
    return 0;
    }
// Provide a dummy swap header for the kernel
#[no_mangle]
unsafe extern "C" fn mtdswap_auto_header(d: *mut mtdswap_dev, buf: *mut c_char) -> c_int {
    static int mtdswap_auto_header(struct mtdswap_dev *d, char *buf)
    {
    union swap_header *hd = (union swap_header *)(buf);
    memset(buf, 0, PAGE_SIZE - 10);
    hd.info.version = 1;
    hd.info.last_page = d.mbd_dev.size - 1;
    hd.info.nr_badpages = 0;
    memcpy(buf + PAGE_SIZE - 10, "SWAPSPACE2", 10);
    return 0;
    }
    static int mtdswap_readsect(struct mtd_blktrans_dev *dev,
    unsigned long page, char *buf)
    {
    struct mtdswap_dev *d = MTDSWAP_MBD_TO_MTDSWAP(dev);
    struct mtd_info *mtd = d.mtd;
    unsigned int realblock, retries;
    loff_t readpos;
    struct swap_eb *eb;
    size_t retlen;
    int ret;
    d.sect_read_count++;
    if (header) {
    if (unlikely(page == 0))
    return mtdswap_auto_header(d, buf);
    page--;
    }
    realblock = d.page_data[page];
    if (realblock > BLOCK_MAX) {
    memset(buf, 0x0, PAGE_SIZE);
    if (realblock == BLOCK_UNDEF)
    return 0;
    else
    return -EIO;
    }
    eb = d.eb_data + (realblock / d.pages_per_eblk);
    BUG_ON(d.revmap[realblock] == PAGE_UNDEF);
    readpos = (loff_t)realblock << PAGE_SHIFT;
    retries = 0;
    retry:
    ret = mtd_read(mtd, readpos, PAGE_SIZE, &retlen, buf);
    d.mtd_read_count++;
    if (mtd_is_bitflip(ret)) {
    eb.flags |= EBLOCK_BITFLIP;
    mtdswap_rb_add(d, eb, MTDSWAP_BITFLIP);
    ret = 0;
    }
    if (ret < 0) {
    dev_err(d.dev, "Read error %d\n", ret);
    eb.flags |= EBLOCK_READERR;
    mtdswap_rb_add(d, eb, MTDSWAP_FAILING);
    retries++;
    if (retries < MTDSWAP_IO_RETRIES)
    goto retry;
    return ret;
    }
    if (retlen != PAGE_SIZE) {
    dev_err(d.dev, "Short read %zd\n", retlen);
    return -EIO;
    }
    return 0;
    }
    static int mtdswap_discard(struct mtd_blktrans_dev *dev, unsigned long first,
    unsigned nr_pages)
    {
    struct mtdswap_dev *d = MTDSWAP_MBD_TO_MTDSWAP(dev);
    unsigned long page;
    struct swap_eb *eb;
    unsigned int mapped;
    d.discard_count++;
    for (page = first; page < first + nr_pages; page++) {
    mapped = d.page_data[page];
    if (mapped <= BLOCK_MAX) {
    eb = d.eb_data + (mapped / d.pages_per_eblk);
    eb.active_count--;
    mtdswap_store_eb(d, eb);
    d.page_data[page] = BLOCK_UNDEF;
    d.revmap[mapped] = PAGE_UNDEF;
    d.discard_page_count++;
    } else if (mapped == BLOCK_ERROR) {
    d.page_data[page] = BLOCK_UNDEF;
    d.discard_page_count++;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int mtdswap_show(struct seq_file *s, void *data)
    {
    struct mtdswap_dev *d = (struct mtdswap_dev *) s.private;
    unsigned long sum;
    unsigned int count[MTDSWAP_TREE_CNT];
    unsigned int min[MTDSWAP_TREE_CNT];
    unsigned int max[MTDSWAP_TREE_CNT];
    unsigned int i, cw = 0, cwp = 0, cwecount = 0, bb_cnt, mapped, pages;
    uint64_t use_size;
    static const char * const name[] = {
    "clean", "used", "low", "high", "dirty", "bitflip", "failing"
    };
    mutex_lock(&d.mbd_dev.lock);
    for (i = 0; i < MTDSWAP_TREE_CNT; i++) {
    struct rb_root *root = &d.trees[i].root;
    if (root.rb_node) {
    count[i] = d.trees[i].count;
    min[i] = MTDSWAP_ECNT_MIN(root);
    max[i] = MTDSWAP_ECNT_MAX(root);
    } else
    count[i] = 0;
    }
    if (d.curr_write) {
    cw = 1;
    cwp = d.curr_write_pos;
    cwecount = d.curr_write.erase_count;
    }
    sum = 0;
    for (i = 0; i < d.eblks; i++)
    sum += d.eb_data[i].erase_count;
    use_size = (uint64_t)d.eblks * d.mtd.erasesize;
    bb_cnt = mtdswap_badblocks(d.mtd, use_size);
    mapped = 0;
    pages = d.mbd_dev.size;
    for (i = 0; i < pages; i++)
    if (d.page_data[i] != BLOCK_UNDEF)
    mapped++;
    mutex_unlock(&d.mbd_dev.lock);
    for (i = 0; i < MTDSWAP_TREE_CNT; i++) {
    if (!count[i])
    continue;
    if (min[i] != max[i])
    seq_printf(s, "%s:\t%5d erase blocks, erased min %d, "
    "max %d times\n",
    name[i], count[i], min[i], max[i]);
    else
    seq_printf(s, "%s:\t%5d erase blocks, all erased %d "
    "times\n", name[i], count[i], min[i]);
    }
    if (bb_cnt)
    seq_printf(s, "bad:\t%5u erase blocks\n", bb_cnt);
    if (cw)
    seq_printf(s, "current erase block: %u pages used, %u free, "
    "erased %u times\n",
    cwp, d.pages_per_eblk - cwp, cwecount);
    seq_printf(s, "total erasures: %lu\n", sum);
    seq_puts(s, "\n");
    seq_printf(s, "mtdswap_readsect count: %llu\n", d.sect_read_count);
    seq_printf(s, "mtdswap_writesect count: %llu\n", d.sect_write_count);
    seq_printf(s, "mtdswap_discard count: %llu\n", d.discard_count);
    seq_printf(s, "mtd read count: %llu\n", d.mtd_read_count);
    seq_printf(s, "mtd write count: %llu\n", d.mtd_write_count);
    seq_printf(s, "discarded pages count: %llu\n", d.discard_page_count);
    seq_puts(s, "\n");
    seq_printf(s, "total pages: %u\n", pages);
    seq_printf(s, "pages mapped: %u\n", mapped);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(mtdswap);
#[no_mangle]
unsafe extern "C" fn mtdswap_add_debugfs(d: *mut mtdswap_dev) -> c_int {
    static int mtdswap_add_debugfs(struct mtdswap_dev *d)
    {
    struct dentry *root = d.mtd.dbg.dfs_dir;
    if (!IS_ENABLED(CONFIG_DEBUG_FS))
    return 0;
    if (IS_ERR_OR_NULL(root))
    return -1;
    d.debugfs_stats = debugfs_create_file("mtdswap_stats", 0400, root,
    d, &mtdswap_fops);
    return 0;
    }
    static int mtdswap_init(struct mtdswap_dev *d, unsigned int eblocks,
    unsigned int spare_cnt)
    {
    struct mtd_info *mtd = d.mbd_dev.mtd;
    unsigned int i, eblk_bytes, pages, blocks;
    let mut ret: c_int = -ENOMEM;
    d.mtd = mtd;
    d.eblks = eblocks;
    d.spare_eblks = spare_cnt;
    d.pages_per_eblk = mtd.erasesize >> PAGE_SHIFT;
    pages = d.mbd_dev.size;
    blocks = eblocks * d.pages_per_eblk;
    for (i = 0; i < MTDSWAP_TREE_CNT; i++)
    d.trees[i].root = RB_ROOT;
    d.page_data = vmalloc_array(pages, sizeof(int));
    if (!d.page_data)
    goto page_data_fail;
    d.revmap = vmalloc_array(blocks, sizeof(int));
    if (!d.revmap)
    goto revmap_fail;
    eblk_bytes = sizeof(struct swap_eb)*d.eblks;
    d.eb_data = vzalloc(eblk_bytes);
    if (!d.eb_data)
    goto eb_data_fail;
    for (i = 0; i < pages; i++)
    d.page_data[i] = BLOCK_UNDEF;
    for (i = 0; i < blocks; i++)
    d.revmap[i] = PAGE_UNDEF;
    d.page_buf = kmalloc(PAGE_SIZE, GFP_KERNEL);
    if (!d.page_buf)
    goto page_buf_fail;
    d.oob_buf = kmalloc_array(2, mtd.oobavail, GFP_KERNEL);
    if (!d.oob_buf)
    goto oob_buf_fail;
    mtdswap_scan_eblks(d);
    return 0;
    oob_buf_fail:
    kfree(d.page_buf);
    page_buf_fail:
    vfree(d.eb_data);
    eb_data_fail:
    vfree(d.revmap);
    revmap_fail:
    vfree(d.page_data);
    page_data_fail:
    printk(KERN_ERR "%s: init failed (%d)\n", MTDSWAP_PREFIX, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_add_mtd(tr: *mut mtd_blktrans_ops, mtd: *mut mtd_info) {
    static void mtdswap_add_mtd(struct mtd_blktrans_ops *tr, struct mtd_info *mtd)
    {
    struct mtdswap_dev *d;
    struct mtd_blktrans_dev *mbd_dev;
    char *parts;
    char *this_opt;
    unsigned long part;
    unsigned int eblocks, eavailable, bad_blocks, spare_cnt;
    uint64_t swap_size, use_size, size_limit;
    int ret;
    parts = &partitions[0];
    if (!*parts)
    return;
    while ((this_opt = strsep(&parts, ",")) != core::ptr::null_mut()) {
    if (kstrtoul(this_opt, 0, &part) < 0)
    return;
    if (mtd.index == part)
    break;
    }
    if (mtd.index != part)
    return;
    if (mtd.erasesize < PAGE_SIZE || mtd.erasesize % PAGE_SIZE) {
    printk(KERN_ERR "%s: Erase size %u not multiple of PAGE_SIZE "
    "%lu\n", MTDSWAP_PREFIX, mtd.erasesize, PAGE_SIZE);
    return;
    }
    if (PAGE_SIZE % mtd.writesize || mtd.writesize > PAGE_SIZE) {
    printk(KERN_ERR "%s: PAGE_SIZE %lu not multiple of write size"
    " %u\n", MTDSWAP_PREFIX, PAGE_SIZE, mtd.writesize);
    return;
    }
    if (!mtd.oobsize || mtd.oobavail < MTDSWAP_OOBSIZE) {
    printk(KERN_ERR "%s: Not enough free bytes in OOB, "
    "%d available, %zu needed.\n",
    MTDSWAP_PREFIX, mtd.oobavail, MTDSWAP_OOBSIZE);
    return;
    }
    if (spare_eblocks > 100)
    spare_eblocks = 100;
    use_size = mtd.size;
    size_limit = (uint64_t) BLOCK_MAX * PAGE_SIZE;
    if (mtd.size > size_limit) {
    printk(KERN_WARNING "%s: Device too large. Limiting size to "
    "%llu bytes\n", MTDSWAP_PREFIX, size_limit);
    use_size = size_limit;
    }
    eblocks = mtd_div_by_eb(use_size, mtd);
    use_size = (uint64_t)eblocks * mtd.erasesize;
    bad_blocks = mtdswap_badblocks(mtd, use_size);
    eavailable = eblocks - bad_blocks;
    if (eavailable < MIN_ERASE_BLOCKS) {
    printk(KERN_ERR "%s: Not enough erase blocks. %u available, "
    "%d needed\n", MTDSWAP_PREFIX, eavailable,
    MIN_ERASE_BLOCKS);
    return;
    }
    spare_cnt = div_u64((uint64_t)eavailable * spare_eblocks, 100);
    if (spare_cnt < MIN_SPARE_EBLOCKS)
    spare_cnt = MIN_SPARE_EBLOCKS;
    if (spare_cnt > eavailable - 1)
    spare_cnt = eavailable - 1;
    swap_size = (uint64_t)(eavailable - spare_cnt) * mtd.erasesize +
    (header ? PAGE_SIZE : 0);
    printk(KERN_INFO "%s: Enabling MTD swap on device %lu, size %llu KB, "
    "%u spare, %u bad blocks\n",
    MTDSWAP_PREFIX, part, swap_size / 1024, spare_cnt, bad_blocks);
    d = kzalloc_obj(struct mtdswap_dev);
    if (!d)
    return;
    mbd_dev = kzalloc_obj(struct mtd_blktrans_dev);
    if (!mbd_dev) {
    kfree(d);
    return;
    }
    d.mbd_dev = mbd_dev;
    mbd_dev.priv = d;
    mbd_dev.mtd = mtd;
    mbd_dev.devnum = mtd.index;
    mbd_dev.size = swap_size >> PAGE_SHIFT;
    mbd_dev.tr = tr;
    if (!(mtd.flags & MTD_WRITEABLE))
    mbd_dev.readonly = 1;
    if (mtdswap_init(d, eblocks, spare_cnt) < 0)
    goto init_failed;
    if (add_mtd_blktrans_dev(mbd_dev) < 0)
    goto cleanup;
    d.dev = disk_to_dev(mbd_dev.disk);
    ret = mtdswap_add_debugfs(d);
    if (ret < 0)
    goto debugfs_failed;
    return;
    debugfs_failed:
    del_mtd_blktrans_dev(mbd_dev);
    mbd_dev = core::ptr::null_mut();
    cleanup:
    mtdswap_cleanup(d);
    init_failed:
    kfree(mbd_dev);
    kfree(d);
    }
#[no_mangle]
unsafe extern "C" fn mtdswap_remove_dev(dev: *mut mtd_blktrans_dev) {
    static void mtdswap_remove_dev(struct mtd_blktrans_dev *dev)
    {
    struct mtdswap_dev *d = MTDSWAP_MBD_TO_MTDSWAP(dev);
    debugfs_remove(d.debugfs_stats);
    del_mtd_blktrans_dev(dev);
    mtdswap_cleanup(d);
    kfree(d);
    }
    static struct mtd_blktrans_ops mtdswap_ops = {
    .name		= "mtdswap",
    .major		= 0,
    .part_bits	= 0,
    .blksize	= PAGE_SIZE,
    .flush		= mtdswap_flush,
    .readsect	= mtdswap_readsect,
    .writesect	= mtdswap_writesect,
    .discard	= mtdswap_discard,
    .background	= mtdswap_background,
    .add_mtd	= mtdswap_add_mtd,
    .remove_dev	= mtdswap_remove_dev,
    .owner		= THIS_MODULE,
    };
    module_mtd_blktrans(mtdswap_ops);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Jarkko Lavinen <jarkko.lavinen@nokia.com>");
    MODULE_DESCRIPTION("Block device access to an MTD suitable for using as "
    "swap space");

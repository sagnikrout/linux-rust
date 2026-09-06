//! Automatically rewritten from C to Rust
//! Source: fs/jfs/inode.c
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
// Copyright (C) International Business Machines Corp., 2000-2004
// Portions Copyright (C) Christoph Hellwig, 2001-2002
//

    struct inode *jfs_iget(struct super_block *sb, unsigned long ino)
    {
    struct inode *inode;
    int ret;
    inode = iget_locked(sb, ino);
    if (!inode)
    return ERR_PTR(-ENOMEM);
    if (!(inode_state_read_once(inode) & I_NEW))
    return inode;
    ret = diRead(inode);
    if (ret < 0) {
    iget_failed(inode);
    return ERR_PTR(ret);
    }
    if (S_ISREG(inode.i_mode)) {
    inode.i_op = &jfs_file_inode_operations;
    inode.i_fop = &jfs_file_operations;
    inode.i_mapping.a_ops = &jfs_aops;
    } else if (S_ISDIR(inode.i_mode)) {
    inode.i_op = &jfs_dir_inode_operations;
    inode.i_fop = &jfs_dir_operations;
    } else if (S_ISLNK(inode.i_mode)) {
    if (inode.i_size >= IDATASIZE) {
    inode.i_op = &page_symlink_inode_operations;
    inode_nohighmem(inode);
    inode.i_mapping.a_ops = &jfs_aops;
    } else {
    inode.i_op = &jfs_fast_symlink_inode_operations;
    inode.i_link = JFS_IP(inode).i_inline;
//
// The inline data should be null-terminated, but
// don't let on-disk corruption crash the kernel
//
    inode.i_link[inode.i_size] = '\0';
    }
    } else if (S_ISCHR(inode.i_mode) || S_ISBLK(inode.i_mode) ||
    S_ISFIFO(inode.i_mode) || S_ISSOCK(inode.i_mode)) {
    inode.i_op = &jfs_file_inode_operations;
    init_special_inode(inode, inode.i_mode, inode.i_rdev);
    } else {
    printk(KERN_DEBUG "JFS: Invalid file type 0%04o for inode %llu.\n",
    inode.i_mode, inode.i_ino);
    iget_failed(inode);
    return ERR_PTR(-EIO);
    }
    unlock_new_inode(inode);
    return inode;
    }
//
// Workhorse of both fsync & write_inode
//
#[no_mangle]
pub unsafe extern "C" fn jfs_commit_inode(inode: *mut inode, wait: c_int) -> c_int {
    int jfs_commit_inode(struct inode *inode, int wait)
    {
    let mut rc: c_int = 0;
    tid_t tid;
    let mut noisy: static int = 5;
    jfs_info("In jfs_commit_inode, inode = 0x%p", inode);
//
// Don't commit if inode has been committed since last being
// marked dirty, or if it has been deleted.
//
    if (inode.i_nlink == 0 || !test_cflag(COMMIT_Dirty, inode))
    return 0;
    if (isReadOnly(inode)) {
// kernel allows writes to devices on read-only
// partitions and may think inode is dirty
//
    if (!special_file(inode.i_mode) && noisy) {
    jfs_err("jfs_commit_inode(0x%p) called on read-only volume",
    inode);
    jfs_err("Is remount racy?");
    noisy--;
    }
    return 0;
    }
    tid = txBegin(inode.i_sb, COMMIT_INODE);
    mutex_lock(&JFS_IP(inode).commit_mutex);
//
// Retest inode state after taking commit_mutex
//
    if (inode.i_nlink && test_cflag(COMMIT_Dirty, inode))
    rc = txCommit(tid, 1, &inode, wait ? COMMIT_SYNC : 0);
    txEnd(tid);
    mutex_unlock(&JFS_IP(inode).commit_mutex);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn jfs_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> c_int {
    int jfs_write_inode(struct inode *inode, struct writeback_control *wbc)
    {
    let mut wait: c_int = wbc.sync_mode == WB_SYNC_ALL;
    if (inode.i_nlink == 0)
    return 0;
//
// If COMMIT_DIRTY is not set, the inode isn't really dirty.
// It has been committed since the last change, but was still
// on the dirty inode list.
//
    if (!test_cflag(COMMIT_Dirty, inode)) {
// Make sure committed changes hit the disk
    jfs_flush_journal(JFS_SBI(inode.i_sb).log, wait);
    return 0;
    }
    if (jfs_commit_inode(inode, wait)) {
    jfs_err("jfs_write_inode: jfs_commit_inode failed!");
    return -EIO;
    } else
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn jfs_evict_inode(inode: *mut inode) {
    void jfs_evict_inode(struct inode *inode)
    {
    struct jfs_inode_info *ji = JFS_IP(inode);
    jfs_info("In jfs_evict_inode, inode = 0x%p", inode);
    if (!inode.i_nlink && !is_bad_inode(inode)) {
    dquot_initialize(inode);
    truncate_inode_pages_final(&inode.i_data);
    if (JFS_IP(inode).fileset == FILESYSTEM_I) {
    struct inode *ipimap = JFS_SBI(inode.i_sb).ipimap;
    if (test_cflag(COMMIT_Freewmap, inode))
    jfs_free_zero_link(inode);
    if (ipimap && JFS_IP(ipimap).i_imap)
    diFree(inode);
//
// Free the inode from the quota allocation.
//
    dquot_free_inode(inode);
    }
    } else {
    truncate_inode_pages_final(&inode.i_data);
    }
    clear_inode(inode);
    dquot_drop(inode);
    BUG_ON(!list_empty(&ji.anon_inode_list));
    spin_lock_irq(&ji.ag_lock);
    if (ji.active_ag != -1) {
    struct bmap *bmap = JFS_SBI(inode.i_sb).bmap;
    atomic_dec(&bmap.db_active[ji.active_ag]);
    ji.active_ag = -1;
    }
    spin_unlock_irq(&ji.ag_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn jfs_dirty_inode(inode: *mut inode, flags: c_int) {
    void jfs_dirty_inode(struct inode *inode, int flags)
    {
    let mut noisy: static int = 5;
    if (isReadOnly(inode)) {
    if (!special_file(inode.i_mode) && noisy) {
// kernel allows writes to devices on read-only
// partitions and may try to mark inode dirty
//
    jfs_err("jfs_dirty_inode called on read-only volume");
    jfs_err("Is remount racy?");
    noisy--;
    }
    return;
    }
    set_cflag(COMMIT_Dirty, inode);
    }
    int jfs_get_block(struct inode *ip, sector_t lblock,
    struct buffer_head *bh_result, int create)
    {
    let mut lblock64: i64 = lblock;
    let mut rc: c_int = 0;
    xad_t xad;
    s64 xaddr;
    int xflag;
    let mut xlen: i32 = bh_result.b_size >> ip.i_blkbits;
//
// Take appropriate lock on inode
//
    if (create)
    IWRITE_LOCK(ip, RDWRLOCK_NORMAL);
    else
    IREAD_LOCK(ip, RDWRLOCK_NORMAL);
    if (((lblock64 << ip.i_sb.s_blocksize_bits) < ip.i_size) &&
    (!xtLookup(ip, lblock64, xlen, &xflag, &xaddr, &xlen, 0)) &&
    xaddr) {
    if (xflag & XAD_NOTRECORDED) {
    if (!create)
//
// Allocated but not recorded, read treats
// this as a hole
//
    goto unlock;
    XADoffset(&xad, lblock64);
    XADlength(&xad, xlen);
    XADaddress(&xad, xaddr);
    rc = extRecord(ip, &xad);
    if (rc)
    goto unlock;
    set_buffer_new(bh_result);
    }
    map_bh(bh_result, ip.i_sb, xaddr);
    bh_result.b_size = xlen << ip.i_blkbits;
    goto unlock;
    }
    if (!create)
    goto unlock;
//
// Allocate a new block
//
    if ((rc = extHint(ip, lblock64 << ip.i_sb.s_blocksize_bits, &xad)))
    goto unlock;
    rc = extAlloc(ip, xlen, lblock64, &xad, false);
    if (rc)
    goto unlock;
    set_buffer_new(bh_result);
    map_bh(bh_result, ip.i_sb, addressXAD(&xad));
    bh_result.b_size = lengthXAD(&xad) << ip.i_blkbits;
    unlock:
//
// Release lock on inode
//
    if (create)
    IWRITE_UNLOCK(ip);
    else
    IREAD_UNLOCK(ip);
    return rc;
    }
    static int jfs_writepages(struct address_space *mapping,
    struct writeback_control *wbc)
    {
    return mpage_writepages(mapping, wbc, jfs_get_block);
    }
#[no_mangle]
unsafe extern "C" fn jfs_read_folio(file: *mut file, folio: *mut folio) -> c_int {
    static int jfs_read_folio(struct file *file, struct folio *folio)
    {
    return mpage_read_folio(folio, jfs_get_block);
    }
#[no_mangle]
unsafe extern "C" fn jfs_readahead(rac: *mut readahead_control) {
    static void jfs_readahead(struct readahead_control *rac)
    {
    mpage_readahead(rac, jfs_get_block);
    }
#[no_mangle]
unsafe extern "C" fn jfs_write_failed(mapping: *mut address_space, to: loff_t) {
    static void jfs_write_failed(struct address_space *mapping, loff_t to)
    {
    struct inode *inode = mapping.host;
    if (to > inode.i_size) {
    truncate_pagecache(inode, inode.i_size);
    jfs_truncate(inode);
    }
    }
    static int jfs_write_begin(const struct kiocb *iocb,
    struct address_space *mapping,
    loff_t pos, unsigned len,
    struct folio **foliop, void **fsdata)
    {
    int ret;
    ret = block_write_begin(mapping, pos, len, foliop, jfs_get_block);
    if (unlikely(ret))
    jfs_write_failed(mapping, pos + len);
    return ret;
    }
    static int jfs_write_end(const struct kiocb *iocb,
    struct address_space *mapping,
    loff_t pos, unsigned len, unsigned copied,
    struct folio *folio, void *fsdata)
    {
    int ret;
    ret = generic_write_end(iocb, mapping, pos, len, copied, folio, fsdata);
    if (ret < len)
    jfs_write_failed(mapping, pos + len);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn jfs_bmap(mapping: *mut address_space, block: sector_t) -> sector_t {
    static sector_t jfs_bmap(struct address_space *mapping, sector_t block)
    {
    return generic_block_bmap(mapping, block, jfs_get_block);
    }
#[no_mangle]
unsafe extern "C" fn jfs_direct_IO(iocb: *mut kiocb, iter: *mut iov_iter) -> isize {
    static ssize_t jfs_direct_IO(struct kiocb *iocb, struct iov_iter *iter)
    {
    struct file *file = iocb.ki_filp;
    struct address_space *mapping = file.f_mapping;
    struct inode *inode = file.f_mapping.host;
    let mut count: usize = iov_iter_count(iter);
    ssize_t ret;
    ret = blockdev_direct_IO(iocb, inode, iter, jfs_get_block);
//
// In case of error extending write may have instantiated a few
// blocks outside i_size. Trim these off again.
//
    if (unlikely(iov_iter_rw(iter) == WRITE && ret < 0)) {
    let mut isize: loff_t = i_size_read(inode);
    let mut end: loff_t = iocb.ki_pos + count;
    if (end > isize)
    jfs_write_failed(mapping, end);
    }
    return ret;
    }
    const struct address_space_operations jfs_aops = {
    .dirty_folio	= block_dirty_folio,
    .invalidate_folio = block_invalidate_folio,
    .read_folio	= jfs_read_folio,
    .readahead	= jfs_readahead,
    .writepages	= jfs_writepages,
    .write_begin	= jfs_write_begin,
    .write_end	= jfs_write_end,
    .bmap		= jfs_bmap,
    .direct_IO	= jfs_direct_IO,
    .migrate_folio	= buffer_migrate_folio,
    };
//
// Guts of jfs_truncate.  Called with locks already held.  Can be called
// with directory for truncating directory index table.
//
#[no_mangle]
pub unsafe extern "C" fn jfs_truncate_nolock(ip: *mut inode, length: loff_t) {
    void jfs_truncate_nolock(struct inode *ip, loff_t length)
    {
    loff_t newsize;
    tid_t tid;
    ASSERT(length >= 0);
    if (test_cflag(COMMIT_Nolink, ip) || isReadOnly(ip)) {
    xtTruncate(0, ip, length, COMMIT_WMAP);
    return;
    }
    do {
    tid = txBegin(ip.i_sb, 0);
//
// The commit_mutex cannot be taken before txBegin.
// txBegin may block and there is a chance the inode
// could be marked dirty and need to be committed
// before txBegin unblocks
//
    mutex_lock(&JFS_IP(ip).commit_mutex);
    newsize = xtTruncate(tid, ip, length,
    COMMIT_TRUNCATE | COMMIT_PWMAP);
    if (newsize < 0) {
    txEnd(tid);
    mutex_unlock(&JFS_IP(ip).commit_mutex);
    break;
    }
    inode_set_mtime_to_ts(ip, inode_set_ctime_current(ip));
    mark_inode_dirty(ip);
    txCommit(tid, 1, &ip, 0);
    txEnd(tid);
    mutex_unlock(&JFS_IP(ip).commit_mutex);
    } while (newsize > length);	/* Truncate isn't always atomic */
    }
#[no_mangle]
pub unsafe extern "C" fn jfs_truncate(ip: *mut inode) {
    void jfs_truncate(struct inode *ip)
    {
    jfs_info("jfs_truncate: size = 0x%lx", (ulong) ip.i_size);
    block_truncate_page(ip.i_mapping, ip.i_size, jfs_get_block);
    IWRITE_LOCK(ip, RDWRLOCK_NORMAL);
    jfs_truncate_nolock(ip, ip.i_size);
    IWRITE_UNLOCK(ip);
    }

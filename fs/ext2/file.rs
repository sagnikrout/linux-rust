//! Automatically rewritten from C to Rust
//! Source: fs/ext2/file.c
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
// linux/fs/ext2/file.c
//
// Copyright (C) 1992, 1993, 1994, 1995
// Remy Card (card@masi.ibp.fr)
// Laboratoire MASI - Institut Blaise Pascal
// Universite Pierre et Marie Curie (Paris VI)
//
// from
//
// linux/fs/minix/file.c
//
// Copyright (C) 1991, 1992  Linus Torvalds
//
// ext2 fs regular file handling primitives
//
// 64-bit file support on 64-bit platforms by Jakub Jelinek
// (jj@sunsite.ms.mff.cuni.cz)
//

//
// Called when filp is released. This happens when all file descriptors
// for a single struct file are closed. Note that different open() calls
// for the same file yield different struct file structures.
//
#[no_mangle]
unsafe extern "C" fn ext2_release_file(inode: *mut *mut inode, filp: *mut *mut file) -> c_int {
    static int ext2_release_file (struct inode * inode, struct file * filp)
    {
    if (filp.f_mode & FMODE_WRITE) {
    mutex_lock(&EXT2_I(inode).truncate_mutex);
    ext2_discard_reservation(inode);
    mutex_unlock(&EXT2_I(inode).truncate_mutex);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ext2_dio_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> isize {
    static ssize_t ext2_dio_read_iter(struct kiocb *iocb, struct iov_iter *to)
    {
    struct file *file = iocb.ki_filp;
    struct inode *inode = file.f_mapping.host;
    ssize_t ret;
    trace_ext2_dio_read_begin(iocb, to, 0);
    inode_lock_shared(inode);
    ret = iomap_dio_rw(iocb, to, &ext2_iomap_ops, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    inode_unlock_shared(inode);
    trace_ext2_dio_read_end(iocb, to, ret);
    return ret;
    }
    static int ext2_dio_write_end_io(struct kiocb *iocb, ssize_t size,
    int error, unsigned int flags)
    {
    let mut pos: loff_t = iocb.ki_pos;
    struct inode *inode = file_inode(iocb.ki_filp);
    if (error)
    goto out;
//
// If we are extending the file, we have to update i_size here before
// page cache gets invalidated in iomap_dio_rw(). This prevents racing
// buffered reads from zeroing out too much from page cache pages.
// Note that all extending writes always happens synchronously with
// inode lock held by ext2_dio_write_iter(). So it is safe to update
// inode size here for extending file writes.
//
    pos += size;
    if (pos > i_size_read(inode)) {
    i_size_write(inode, pos);
    mark_inode_dirty(inode);
    }
    out:
    trace_ext2_dio_write_endio(iocb, size, error);
    return error;
    }
    static const struct iomap_dio_ops ext2_dio_write_ops = {
    .end_io = ext2_dio_write_end_io,
    };
#[no_mangle]
unsafe extern "C" fn ext2_dio_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize {
    static ssize_t ext2_dio_write_iter(struct kiocb *iocb, struct iov_iter *from)
    {
    struct file *file = iocb.ki_filp;
    struct inode *inode = file.f_mapping.host;
    ssize_t ret;
    let mut flags: c_uint = 0;
    let mut blocksize: c_ulong = inode.i_sb.s_blocksize;
    let mut offset: loff_t = iocb.ki_pos;
    let mut count: loff_t = iov_iter_count(from);
    let mut status: isize = 0;
    trace_ext2_dio_write_begin(iocb, from, 0);
    inode_lock(inode);
    ret = generic_write_checks(iocb, from);
    if (ret <= 0)
    goto out_unlock;
    ret = kiocb_modified(iocb);
    if (ret)
    goto out_unlock;
// use IOMAP_DIO_FORCE_WAIT for unaligned or extending writes
    if (iocb.ki_pos + iov_iter_count(from) > i_size_read(inode) ||
    (!IS_ALIGNED(iocb.ki_pos | iov_iter_alignment(from), blocksize)))
    flags |= IOMAP_DIO_FORCE_WAIT;
    ret = iomap_dio_rw(iocb, from, &ext2_iomap_ops, &ext2_dio_write_ops,
    flags, core::ptr::null_mut(), 0);
// ENOTBLK is magic return value for fallback to buffered-io
    if (ret == -ENOTBLK)
    ret = 0;
    if (ret < 0 && ret != -EIOCBQUEUED)
    ext2_write_failed(inode.i_mapping, offset + count);
// handle case for partial write and for fallback to buffered write
    if (ret >= 0 && iov_iter_count(from)) {
    loff_t pos, endbyte;
    int ret2;
    iocb.ki_flags &= ~IOCB_DIRECT;
    pos = iocb.ki_pos;
    status = generic_perform_write(iocb, from);
    if (unlikely(status < 0)) {
    ret = status;
    goto out_unlock;
    }
    ret += status;
    endbyte = pos + status - 1;
    ret2 = filemap_write_and_wait_range(inode.i_mapping, pos,
    endbyte);
    if (!ret2) {
    invalidate_mapping_pages(inode.i_mapping,
    pos >> PAGE_SHIFT,
    endbyte >> PAGE_SHIFT);
    if (ret > 0)
    ret = generic_write_sync(iocb, ret);
    } else {
    ret = ret2;
    }
    }
    out_unlock:
    inode_unlock(inode);
    if (status)
    trace_ext2_dio_write_buff_end(iocb, from, status);
    trace_ext2_dio_write_end(iocb, from, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ext2_file_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> isize {
    static ssize_t ext2_file_read_iter(struct kiocb *iocb, struct iov_iter *to)
    {
    if (iocb.ki_flags & IOCB_DIRECT)
    return ext2_dio_read_iter(iocb, to);
    return generic_file_read_iter(iocb, to);
    }
#[no_mangle]
unsafe extern "C" fn ext2_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize {
    static ssize_t ext2_file_write_iter(struct kiocb *iocb, struct iov_iter *from)
    {
    if (iocb.ki_flags & IOCB_DIRECT)
    return ext2_dio_write_iter(iocb, from);
    return generic_file_write_iter(iocb, from);
    }
#[no_mangle]
unsafe extern "C" fn ext2_file_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int ext2_file_open(struct inode *inode, struct file *filp)
    {
    filp.f_mode |= FMODE_CAN_ODIRECT;
    return dquot_file_open(inode, filp);
    }
    const struct file_operations ext2_file_operations = {
    .llseek		= generic_file_llseek,
    .read_iter	= ext2_file_read_iter,
    .write_iter	= ext2_file_write_iter,
    .unlocked_ioctl = ext2_ioctl,

    .compat_ioctl	= ext2_compat_ioctl,

    .mmap_prepare	= generic_file_mmap_prepare,
    .open		= ext2_file_open,
    .release	= ext2_release_file,
    .fsync		= simple_fsync,
    .get_unmapped_area = thp_get_unmapped_area,
    .splice_read	= filemap_splice_read,
    .splice_write	= iter_file_splice_write,
    .setlease	= generic_setlease,
    };
    const struct inode_operations ext2_file_inode_operations = {
    .listxattr	= ext2_listxattr,
    .getattr	= ext2_getattr,
    .setattr	= ext2_setattr,
    .get_inode_acl	= ext2_get_acl,
    .set_acl	= ext2_set_acl,
    .fiemap		= ext2_fiemap,
    .fileattr_get	= ext2_fileattr_get,
    .fileattr_set	= ext2_fileattr_set,
    };

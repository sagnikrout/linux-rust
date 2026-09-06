//! Automatically rewritten from C to Rust
//! Source: fs/nfs/io.c
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
// Copyright (c) 2016 Trond Myklebust
//
// I/O and data path helper functionality.
//

//
// nfs_start_io_read - declare the file is being used for buffered reads
// @inode: file inode
//
// Declare that a buffered read operation is about to start, and ensure
// that we block all direct I/O.
// On exit, the function ensures that the NFS_INO_ODIRECT flag is unset,
// and holds a shared lock on inode->i_rwsem to ensure that the flag
// cannot be changed.
// In practice, this means that buffered read operations are allowed to
// execute in parallel, thanks to the shared lock, whereas direct I/O
// operations need to wait to grab an exclusive lock in order to set
// NFS_INO_ODIRECT.
// Note that buffered writes and truncates both take a write lock on
// inode->i_rwsem, meaning that those are serialised w.r.t. the reads.
//
    int
    nfs_start_io_read(struct inode *inode)
    {
    struct nfs_inode *nfsi = NFS_I(inode);
    int err;
// Be an optimist!
    err = down_read_killable(&inode.i_rwsem);
    if (err)
    return err;
    if (test_bit(NFS_INO_ODIRECT, &nfsi.flags) == 0)
    return 0;
    up_read(&inode.i_rwsem);
// Slow path....
    err = down_write_killable(&inode.i_rwsem);
    if (err)
    return err;
    nfs_file_block_o_direct(nfsi);
    downgrade_write(&inode.i_rwsem);
    return 0;
    }
//
// nfs_end_io_read - declare that the buffered read operation is done
// @inode: file inode
//
// Declare that a buffered read operation is done, and release the shared
// lock on inode->i_rwsem.
//
    void
    nfs_end_io_read(struct inode *inode)
    {
    up_read(&inode.i_rwsem);
    }
//
// nfs_start_io_write - declare the file is being used for buffered writes
// @inode: file inode
//
// Declare that a buffered read operation is about to start, and ensure
// that we block all direct I/O.
//
    int
    nfs_start_io_write(struct inode *inode)
    {
    int err;
    err = down_write_killable(&inode.i_rwsem);
    if (!err)
    nfs_file_block_o_direct(NFS_I(inode));
    return err;
    }
    EXPORT_SYMBOL_GPL(nfs_start_io_write);
//
// nfs_end_io_write - declare that the buffered write operation is done
// @inode: file inode
//
// Declare that a buffered write operation is done, and release the
// lock on inode->i_rwsem.
//
    void
    nfs_end_io_write(struct inode *inode)
    {
    up_write(&inode.i_rwsem);
    }
    EXPORT_SYMBOL_GPL(nfs_end_io_write);
// Call with exclusively locked inode->i_rwsem
#[no_mangle]
unsafe extern "C" fn nfs_block_buffered(nfsi: *mut nfs_inode, inode: *mut inode) {
    static void nfs_block_buffered(struct nfs_inode *nfsi, struct inode *inode)
    {
    if (!test_bit(NFS_INO_ODIRECT, &nfsi.flags)) {
    set_bit(NFS_INO_ODIRECT, &nfsi.flags);
    nfs_sync_mapping(inode.i_mapping);
    }
    }
#[no_mangle]
unsafe extern "C" fn nfs_block_buffered_nowait(nfsi: *mut nfs_inode, inode: *mut inode) -> c_int {
    static int nfs_block_buffered_nowait(struct nfs_inode *nfsi, struct inode *inode)
    {
    if (!test_bit(NFS_INO_ODIRECT, &nfsi.flags)) {
    if (inode.i_mapping.nrpages != 0)
    return 1;
    set_bit(NFS_INO_ODIRECT, &nfsi.flags);
    }
    return 0;
    }
//
// nfs_start_io_direct - declare the file is being used for direct i/o
// @inode: file inode
//
// Declare that a direct I/O operation is about to start, and ensure
// that we block all buffered I/O.
// On exit, the function ensures that the NFS_INO_ODIRECT flag is set,
// and holds a shared lock on inode->i_rwsem to ensure that the flag
// cannot be changed.
// In practice, this means that direct I/O operations are allowed to
// execute in parallel, thanks to the shared lock, whereas buffered I/O
// operations need to wait to grab an exclusive lock in order to clear
// NFS_INO_ODIRECT.
// Note that buffered writes and truncates both take a write lock on
// inode->i_rwsem, meaning that those are serialised w.r.t. O_DIRECT.
//
    int
    nfs_start_io_direct(struct inode *inode)
    {
    struct nfs_inode *nfsi = NFS_I(inode);
    int err;
// Be an optimist!
    err = down_read_killable(&inode.i_rwsem);
    if (err)
    return err;
    if (test_bit(NFS_INO_ODIRECT, &nfsi.flags) != 0)
    return 0;
    up_read(&inode.i_rwsem);
// Slow path....
    err = down_write_killable(&inode.i_rwsem);
    if (err)
    return err;
    nfs_block_buffered(nfsi, inode);
    downgrade_write(&inode.i_rwsem);
    return 0;
    }
//
// nfs_start_io_direct_nowait - non-blocking variant of nfs_start_io_direct()
// @inode: file inode
//
// Try to declare that a direct I/O operation is about to start without
// blocking.
// Ensure all buffered I/O is blocked.
// If this could not be done without blocking then returns -EAGAIN.
//
    int
    nfs_start_io_direct_nowait(struct inode *inode)
    {
    struct nfs_inode *nfsi = NFS_I(inode);
    if (!down_read_trylock(&inode.i_rwsem))
    return -EAGAIN;
    if (test_bit(NFS_INO_ODIRECT, &nfsi.flags))
    return 0;
    up_read(&inode.i_rwsem);
// Slow path: try to flip NFS_INO_ODIRECT without blocking.
    if (!down_write_trylock(&inode.i_rwsem))
    return -EAGAIN;
    if (nfs_block_buffered_nowait(nfsi, inode)) {
    up_write(&inode.i_rwsem);
    return -EAGAIN;
    }
    downgrade_write(&inode.i_rwsem);
    return 0;
    }
//
// nfs_end_io_direct - declare that the direct i/o operation is done
// @inode: file inode
//
// Declare that a direct I/O operation is done, and release the shared
// lock on inode->i_rwsem.
//
    void
    nfs_end_io_direct(struct inode *inode)
    {
    up_read(&inode.i_rwsem);
    }

//! Automatically rewritten from C to Rust
//! Source: fs/jfs/jfs_umount.c
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
//
// jfs_umount.c
//
// note: file system in transition to aggregate/fileset:
// (ref. jfs_mount.c)
//
// file system unmount is interpreted as mount of the single/only
// fileset in the aggregate and, if unmount of the last fileset,
// as unmount of the aggerate;
//

//
// NAME:	jfs_umount(vfsp, flags, crp)
//
// FUNCTION:	vfs_umount()
//
// PARAMETERS:	vfsp	- virtual file system pointer
// flags	- unmount for shutdown
// crp	- credential
//
// RETURN :	EBUSY	- device has open files
//
#[no_mangle]
pub unsafe extern "C" fn jfs_umount(sb: *mut super_block) -> c_int {
    int jfs_umount(struct super_block *sb)
    {
    struct jfs_sb_info *sbi = JFS_SBI(sb);
    struct inode *ipbmap = sbi.ipbmap;
    struct inode *ipimap = sbi.ipimap;
    struct inode *ipaimap = sbi.ipaimap;
    struct inode *ipaimap2 = sbi.ipaimap2;
    struct jfs_log *log;
    let mut rc: c_int = 0;
    jfs_info("UnMount JFS: sb:0x%p", sb);
//
// update superblock and close log
//
// if mounted read-write and log based recovery was enabled
//
    if ((log = sbi.log))
//
// Wait for outstanding transactions to be written to log:
//
    jfs_flush_journal(log, 2);
//
// Hold log lock so write_special_inodes (lmLogSync) cannot see
// this sbi with a NULL inode pointer while iterating log->sb_list.
//
    if (log)
    LOG_LOCK(log);
//
// close fileset inode allocation map (aka fileset inode)
//
    diUnmount(ipimap, 0);
    diFreeSpecial(ipimap);
    sbi.ipimap = core::ptr::null_mut();
//
// close secondary aggregate inode allocation map
//
    if (ipaimap2) {
    diUnmount(ipaimap2, 0);
    diFreeSpecial(ipaimap2);
    sbi.ipaimap2 = core::ptr::null_mut();
    }
//
// close aggregate inode allocation map
//
    diUnmount(ipaimap, 0);
    diFreeSpecial(ipaimap);
    sbi.ipaimap = core::ptr::null_mut();
//
// close aggregate block allocation map
//
    dbUnmount(ipbmap, 0);
    diFreeSpecial(ipbmap);
    sbi.ipbmap = core::ptr::null_mut();
//
// Make sure all metadata makes it to disk before we mark
// the superblock as clean
//
    filemap_write_and_wait(sbi.direct_inode.i_mapping);
    if (log)
    LOG_UNLOCK(log);
//
// ensure all file system file pages are propagated to their
// home blocks on disk (and their in-memory buffer pages are
// invalidated) BEFORE updating file system superblock state
// (to signify file system is unmounted cleanly, and thus in
// consistent state) and log superblock active file system
// list (to signify skip logredo()).
//
    if (log) {		/* log = core::ptr::null_mut() if read-only mount */
    updateSuper(sb, FM_CLEAN);
//
// close log:
//
// remove file system from log active file system list.
//
    rc = lmLogClose(sb);
    }
    jfs_info("UnMount JFS Complete: rc = %d", rc);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn jfs_umount_rw(sb: *mut super_block) -> c_int {
    int jfs_umount_rw(struct super_block *sb)
    {
    struct jfs_sb_info *sbi = JFS_SBI(sb);
    struct jfs_log *log = sbi.log;
    if (!log)
    return 0;
//
// close log:
//
// remove file system from log active file system list.
//
    jfs_flush_journal(log, 2);
//
// Make sure all metadata makes it to disk
//
    dbSync(sbi.ipbmap);
    diSync(sbi.ipimap);
//
// Note that we have to do this even if sync_blockdev() will
// do exactly the same a few instructions later:  We can't
// mark the superblock clean before everything is flushed to
// disk.
//
    filemap_write_and_wait(sbi.direct_inode.i_mapping);
    updateSuper(sb, FM_CLEAN);
    return lmLogClose(sb);
    }

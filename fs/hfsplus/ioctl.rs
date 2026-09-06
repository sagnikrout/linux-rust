//! Automatically rewritten from C to Rust
//! Source: fs/hfsplus/ioctl.c
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
// linux/fs/hfsplus/ioctl.c
//
// Copyright (C) 2003
// Ethan Benson <erbenson@alaska.net>
// partially derived from linux/fs/ext2/ioctl.c
// Copyright (C) 1993, 1994, 1995
// Remy Card (card@masi.ibp.fr)
// Laboratoire MASI - Institut Blaise Pascal
// Universite Pierre et Marie Curie (Paris VI)
//
// hfsplus ioctls
//

//
// "Blessing" an HFS+ filesystem writes metadata to the superblock informing
// the platform firmware which file to boot from
//
#[no_mangle]
unsafe extern "C" fn hfsplus_ioctl_bless(file: *mut file, user_flags: *mut int __user) -> c_int {
    static int hfsplus_ioctl_bless(struct file *file, int __user *user_flags)
    {
    struct dentry *dentry = file.f_path.dentry;
    struct inode *inode = d_inode(dentry);
    struct hfsplus_sb_info *sbi = HFSPLUS_SB(inode.i_sb);
    struct hfsplus_vh *vh = sbi.s_vhdr;
    struct hfsplus_vh *bvh = sbi.s_backup_vhdr;
    let mut cnid: u32 = (unsigned long)dentry.d_fsdata;
    if (!capable(CAP_SYS_ADMIN))
    return -EPERM;
    mutex_lock(&sbi.vh_mutex);
// Directory containing the bootable system
    vh.finder_info[0] = bvh.finder_info[0] =
    cpu_to_be32(d_parent_ino(dentry));
//
// Bootloader. Just using the inode here breaks in the case of
// hard links - the firmware wants the ID of the hard link file,
// but the inode points at the indirect inode
//
    vh.finder_info[1] = bvh.finder_info[1] = cpu_to_be32(cnid);
// Per spec, the OS X system folder - same as finder_info[0] here
    vh.finder_info[5] = bvh.finder_info[5] =
    cpu_to_be32(d_parent_ino(dentry));
    mutex_unlock(&sbi.vh_mutex);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hfsplus_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    long hfsplus_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    void __user *argp = (void __user *)arg;
    switch (cmd) {
    case HFSPLUS_IOC_BLESS:
    return hfsplus_ioctl_bless(file, argp);
    default:
    return -ENOTTY;
    }
    }

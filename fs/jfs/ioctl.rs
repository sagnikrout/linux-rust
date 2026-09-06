//! Automatically rewritten from C to Rust
//! Source: fs/jfs/ioctl.c
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
// linux/fs/jfs/ioctl.c
//
// Copyright (C) 2006 Herbert Poetzl
// adapted from Remy Card's ext2/ioctl.c
//

    static struct {
    long jfs_flag;
    long ext2_flag;
    } jfs_map[] = {
    {JFS_NOATIME_FL,	FS_NOATIME_FL},
    {JFS_DIRSYNC_FL,	FS_DIRSYNC_FL},
    {JFS_SYNC_FL,		FS_SYNC_FL},
    {JFS_SECRM_FL,		FS_SECRM_FL},
    {JFS_UNRM_FL,		FS_UNRM_FL},
    {JFS_APPEND_FL,		FS_APPEND_FL},
    {JFS_IMMUTABLE_FL,	FS_IMMUTABLE_FL},
    {0, 0},
    };
#[no_mangle]
unsafe extern "C" fn jfs_map_ext2(flags: c_ulong, from: c_int) -> c_long {
    static long jfs_map_ext2(unsigned long flags, int from)
    {
    let mut index: c_int = 0;
    let mut mapped: c_long = 0;
    while (jfs_map[index].jfs_flag) {
    if (from) {
    if (jfs_map[index].ext2_flag & flags)
    mapped |= jfs_map[index].jfs_flag;
    } else {
    if (jfs_map[index].jfs_flag & flags)
    mapped |= jfs_map[index].ext2_flag;
    }
    index++;
    }
    return mapped;
    }
#[no_mangle]
pub unsafe extern "C" fn jfs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int {
    int jfs_fileattr_get(struct dentry *dentry, struct file_kattr *fa)
    {
    struct jfs_inode_info *jfs_inode = JFS_IP(d_inode(dentry));
    let mut flags: c_uint = jfs_inode.mode2 & JFS_FL_USER_VISIBLE;
    if (d_is_special(dentry))
    return -ENOTTY;
    fileattr_fill_flags(fa, jfs_map_ext2(flags, 0));
    return 0;
    }
    int jfs_fileattr_set(struct mnt_idmap *idmap,
    struct dentry *dentry, struct file_kattr *fa)
    {
    struct inode *inode = d_inode(dentry);
    struct jfs_inode_info *jfs_inode = JFS_IP(inode);
    unsigned int flags;
    if (d_is_special(dentry))
    return -ENOTTY;
    if (fileattr_has_fsx(fa))
    return -EOPNOTSUPP;
    flags = jfs_map_ext2(fa.flags, 1);
    if (!S_ISDIR(inode.i_mode))
    flags &= ~JFS_DIRSYNC_FL;
// Is it quota file? Do not allow user to mess with it
    if (IS_NOQUOTA(inode))
    return -EPERM;
    flags = flags & JFS_FL_USER_MODIFIABLE;
    flags |= jfs_inode.mode2 & ~JFS_FL_USER_MODIFIABLE;
    jfs_inode.mode2 = flags;
    jfs_set_inode_flags(inode);
    inode_set_ctime_current(inode);
    mark_inode_dirty(inode);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn jfs_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    long jfs_ioctl(struct file *filp, unsigned int cmd, unsigned long arg)
    {
    struct inode *inode = file_inode(filp);
    switch (cmd) {
    case FITRIM:
    {
    struct super_block *sb = inode.i_sb;
    struct fstrim_range range;
    let mut ret: i64 = 0;
    if (!capable(CAP_SYS_ADMIN))
    return -EPERM;
    if (!bdev_max_discard_sectors(sb.s_bdev)) {
    jfs_warn("FITRIM not supported on device");
    return -EOPNOTSUPP;
    }
    if (copy_from_user(&range, (struct fstrim_range __user *)arg,
    sizeof(range)))
    return -EFAULT;
    range.minlen = max_t(unsigned int, range.minlen,
    bdev_discard_granularity(sb.s_bdev));
    ret = jfs_ioc_trim(inode, &range);
    if (ret < 0)
    return ret;
    if (copy_to_user((struct fstrim_range __user *)arg, &range,
    sizeof(range)))
    return -EFAULT;
    return 0;
    }
    default:
    return -ENOTTY;
    }
    }

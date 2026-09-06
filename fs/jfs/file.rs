//! Automatically rewritten from C to Rust
//! Source: fs/jfs/file.c
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
// Copyright (C) International Business Machines Corp., 2000-2002
// Portions Copyright (C) Christoph Hellwig, 2001-2002
//

#[no_mangle]
pub unsafe extern "C" fn jfs_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int {
    int jfs_fsync(struct file *file, loff_t start, loff_t end, int datasync)
    {
    struct inode *inode = file.f_mapping.host;
    let mut rc: c_int = 0;
    rc = file_write_and_wait_range(file, start, end);
    if (rc)
    return rc;
    inode_lock(inode);
    if (!(inode_state_read_once(inode) & I_DIRTY_ALL) ||
    (datasync && !(inode_state_read_once(inode) & I_DIRTY_DATASYNC))) {
// Make sure committed changes hit the disk
    jfs_flush_journal(JFS_SBI(inode.i_sb).log, 1);
    inode_unlock(inode);
    return rc;
    }
    rc |= jfs_commit_inode(inode, 1);
    inode_unlock(inode);
    return rc ? -EIO : 0;
    }
#[no_mangle]
unsafe extern "C" fn jfs_open(inode: *mut inode, file: *mut file) -> c_int {
    static int jfs_open(struct inode *inode, struct file *file)
    {
    int rc;
    if (S_ISREG(inode.i_mode) && inode.i_size < 0)
    return -EIO;
    if ((rc = dquot_file_open(inode, file)))
    return rc;
//
// We attempt to allow only one "active" file open per aggregate
// group.  Otherwise, appending to files in parallel can cause
// fragmentation within the files.
//
// If the file is empty, it was probably just created and going
// to be written to.  If it has a size, we'll hold off until the
// file is actually grown.
//
    if (S_ISREG(inode.i_mode) && file.f_mode & FMODE_WRITE &&
    (inode.i_size == 0)) {
    struct jfs_inode_info *ji = JFS_IP(inode);
    spin_lock_irq(&ji.ag_lock);
    if (ji.active_ag == -1) {
    struct jfs_sb_info *jfs_sb = JFS_SBI(inode.i_sb);
    ji.active_ag = BLKTOAG(addressPXD(&ji.ixpxd), jfs_sb);
    atomic_inc(&jfs_sb.bmap.db_active[ji.active_ag]);
    }
    spin_unlock_irq(&ji.ag_lock);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jfs_release(inode: *mut inode, file: *mut file) -> c_int {
    static int jfs_release(struct inode *inode, struct file *file)
    {
    struct jfs_inode_info *ji = JFS_IP(inode);
    spin_lock_irq(&ji.ag_lock);
    if (ji.active_ag != -1) {
    struct bmap *bmap = JFS_SBI(inode.i_sb).bmap;
    atomic_dec(&bmap.db_active[ji.active_ag]);
    ji.active_ag = -1;
    }
    spin_unlock_irq(&ji.ag_lock);
    return 0;
    }
    int jfs_setattr(struct mnt_idmap *idmap, struct dentry *dentry,
    struct iattr *iattr)
    {
    struct inode *inode = d_inode(dentry);
    int rc;
    rc = setattr_prepare(&nop_mnt_idmap, dentry, iattr);
    if (rc)
    return rc;
    if (is_quota_modification(&nop_mnt_idmap, inode, iattr)) {
    rc = dquot_initialize(inode);
    if (rc)
    return rc;
    }
    if ((iattr.ia_valid & ATTR_UID && !uid_eq(iattr.ia_uid, inode.i_uid)) ||
    (iattr.ia_valid & ATTR_GID && !gid_eq(iattr.ia_gid, inode.i_gid))) {
    rc = dquot_transfer(&nop_mnt_idmap, inode, iattr);
    if (rc)
    return rc;
    }
    if ((iattr.ia_valid & ATTR_SIZE) &&
    iattr.ia_size != i_size_read(inode)) {
    inode_dio_wait(inode);
    rc = inode_newsize_ok(inode, iattr.ia_size);
    if (rc)
    return rc;
    truncate_setsize(inode, iattr.ia_size);
    jfs_truncate(inode);
    }
    setattr_copy(&nop_mnt_idmap, inode, iattr);
    mark_inode_dirty(inode);
    if (iattr.ia_valid & ATTR_MODE)
    rc = posix_acl_chmod(&nop_mnt_idmap, dentry, inode.i_mode);
    return rc;
    }
    const struct inode_operations jfs_file_inode_operations = {
    .listxattr	= jfs_listxattr,
    .setattr	= jfs_setattr,
    .fileattr_get	= jfs_fileattr_get,
    .fileattr_set	= jfs_fileattr_set,

    .get_inode_acl	= jfs_get_acl,
    .set_acl	= jfs_set_acl,

    };
    const struct file_operations jfs_file_operations = {
    .open		= jfs_open,
    .llseek		= generic_file_llseek,
    .read_iter	= generic_file_read_iter,
    .write_iter	= generic_file_write_iter,
    .mmap_prepare	= generic_file_mmap_prepare,
    .splice_read	= filemap_splice_read,
    .splice_write	= iter_file_splice_write,
    .fsync		= jfs_fsync,
    .release	= jfs_release,
    .unlocked_ioctl = jfs_ioctl,
    .compat_ioctl	= compat_ptr_ioctl,
    .setlease	= generic_setlease,
    };

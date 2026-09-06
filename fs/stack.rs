//! Automatically rewritten from C to Rust
//! Source: fs/stack.c
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

// does _NOT_ require i_rwsem to be held.
//
// This function cannot be inlined since i_size_{read,write} is rather
// heavy-weight on 32-bit systems
//
#[no_mangle]
pub unsafe extern "C" fn fsstack_copy_inode_size(dst: *mut inode, src: *mut inode) {
    void fsstack_copy_inode_size(struct inode *dst, struct inode *src)
    {
    loff_t i_size;
    blkcnt_t i_blocks;
//
// i_size_read() includes its own seqlocking and protection from
// preemption (see include/linux/fs.h): we need nothing extra for
// that here, and prefer to avoid nesting locks than attempt to keep
// i_size and i_blocks in sync together.
//
    i_size = i_size_read(src);
//
// But on 32-bit, we ought to make an effort to keep the two halves of
// i_blocks in sync despite SMP or PREEMPTION - though stat's
// generic_fillattr() doesn't bother, and we won't be applying quotas
// (where i_blocks does become important) at the upper level.
//
// We don't actually know what locking is used at the lower level;
// but if it's a filesystem that supports quotas, it will be using
// i_lock as in inode_add_bytes().
//
    if (sizeof(i_blocks) > sizeof(long))
    spin_lock(&src.i_lock);
    i_blocks = src.i_blocks;
    if (sizeof(i_blocks) > sizeof(long))
    spin_unlock(&src.i_lock);
//
// If CONFIG_SMP or CONFIG_PREEMPTION on 32-bit, it's vital for
// fsstack_copy_inode_size() to hold some lock around
// i_size_write(), otherwise i_size_read() may spin forever (see
// include/linux/fs.h).  We don't necessarily hold i_rwsem when this
// is called, so take i_lock for that case.
//
// And if on 32-bit, continue our effort to keep the two halves of
// i_blocks in sync despite SMP or PREEMPTION: use i_lock for that case
// too, and do both at once by combining the tests.
//
// There is none of this locking overhead in the 64-bit case.
//
    if (sizeof(i_size) > sizeof(long) || sizeof(i_blocks) > sizeof(long))
    spin_lock(&dst.i_lock);
    i_size_write(dst, i_size);
    dst.i_blocks = i_blocks;
    if (sizeof(i_size) > sizeof(long) || sizeof(i_blocks) > sizeof(long))
    spin_unlock(&dst.i_lock);
    }
    EXPORT_SYMBOL_GPL(fsstack_copy_inode_size);
// copy all attributes
#[no_mangle]
pub unsafe extern "C" fn fsstack_copy_attr_all(dest: *mut inode, src: *const inode) {
    void fsstack_copy_attr_all(struct inode *dest, const struct inode *src)
    {
    dest.i_mode = src.i_mode;
    dest.i_uid = src.i_uid;
    dest.i_gid = src.i_gid;
    dest.i_rdev = src.i_rdev;
    inode_set_atime_to_ts(dest, inode_get_atime(src));
    inode_set_mtime_to_ts(dest, inode_get_mtime(src));
    inode_set_ctime_to_ts(dest, inode_get_ctime(src));
    dest.i_blkbits = src.i_blkbits;
    dest.i_flags = src.i_flags;
    set_nlink(dest, src.i_nlink);
    }
    EXPORT_SYMBOL_GPL(fsstack_copy_attr_all);

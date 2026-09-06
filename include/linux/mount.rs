//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mount.h
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
// Definitions for mount interface. This describes the in the kernel build
// linkedlist with mounted filesystems.
//
// Author:  Marco van Wieringen <mvw@planets.elm.net>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mount_flags {
    MNT_NOSUID	= 0x01,
    MNT_NODEV	= 0x02,
    MNT_NOEXEC	= 0x04,
    MNT_NOATIME	= 0x08,
    MNT_NODIRATIME	= 0x10,
    MNT_RELATIME	= 0x20,
    MNT_READONLY	= 0x40, /* does the user want this to be r/o? */
    MNT_NOSYMFOLLOW	= 0x80,

    MNT_SHRINKABLE	= 0x100,

    MNT_INTERNAL	= 0x4000,

    MNT_LOCK_ATIME		= 0x040000,
    MNT_LOCK_NOEXEC		= 0x080000,
    MNT_LOCK_NOSUID		= 0x100000,
    MNT_LOCK_NODEV		= 0x200000,
    MNT_LOCK_READONLY	= 0x400000,
    MNT_LOCKED		= 0x800000,
    MNT_DOOMED		= 0x1000000,
    MNT_SYNC_UMOUNT		= 0x2000000,
    MNT_UMOUNT		= 0x8000000,

    MNT_USER_SETTABLE_MASK  = MNT_NOSUID | MNT_NODEV | MNT_NOEXEC
    | MNT_NOATIME | MNT_NODIRATIME | MNT_RELATIME
    | MNT_READONLY | MNT_NOSYMFOLLOW,
    MNT_ATIME_MASK = MNT_NOATIME | MNT_NODIRATIME | MNT_RELATIME,

    MNT_INTERNAL_FLAGS = MNT_INTERNAL | MNT_DOOMED |
    MNT_SYNC_UMOUNT | MNT_LOCKED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfsmount {
    pub /: *mut *mut *mut dentry mnt_root; / root of the mounted tree,
    pub /: *mut *mut *mut super_block mnt_sb; / pointer to superblock,
    pub mnt_flags: c_int,
    pub mnt_idmap: *mut mnt_idmap,
    pub __randomize_layout: },
// Pairs with smp_store_release() in do_idmap_mount().
    pub READ_ONCE(mnt->mnt_idmap): return,
    pub mnt): *mut extern int mnt_want_write(struct vfsmount,
    pub file): *mut extern int mnt_want_write_file(struct file,
    pub mnt): *mut extern void mnt_drop_write(struct vfsmount,
    pub file): *mut extern void mnt_drop_write_file(struct file,
    pub mnt): *mut extern void mntput(struct vfsmount,
    pub mnt): *mut *mut extern struct vfsmount mntget(struct vfsmount,
    pub mnt): *mut extern void mnt_make_shortterm(struct vfsmount,
    pub path): *const *const extern struct vfsmount mnt_clone_internal(struct path,
    pub mnt): *const extern bool __mnt_is_readonly(struct vfsmount,
    pub mnt): *mut extern bool mnt_may_suid(struct vfsmount,
    pub path): *const *const extern struct vfsmount clone_private_mount(struct path,
    pub mnt): *mut int mnt_get_write_access(struct vfsmount,
    pub mnt): *mut void mnt_put_write_access(struct vfsmount,
    pub fc): *mut *mut extern struct vfsmount fc_mount(struct fs_context,
    pub fc): *mut *mut extern struct vfsmount fc_mount_longterm(struct fs_context,
    pub fc): *mut *mut extern struct vfsmount vfs_create_mount(struct fs_context,
    pub data): *mut c_void,
    pub expiry_list): *mut *mut extern void mnt_set_expiry(struct vfsmount mnt, struct list_head,
    pub mounts): *mut extern void mark_mounts_for_expiry(struct list_head,
    pub path): *const extern bool path_is_mountpoint(struct path,
    pub mnt): *mut extern bool our_mnt(struct vfsmount,
    pub ): *mut *mut extern struct vfsmount kern_mount(struct file_system_type,
    pub mnt): *mut extern void kern_unmount(struct vfsmount,
    pub ): *mut extern int may_umount_tree(struct vfsmount,
    pub ): *mut extern int may_umount(struct vfsmount,
    pub ): *const *const char , unsigned long, void,
    pub unsigned): *const *const *const *const extern struct path collect_paths(struct path , struct path ,,
    pub ): *const *const extern void drop_collected_paths(struct path , struct path,
    pub num): *mut *mut extern void kern_unmount_array(struct vfsmount mnt[], unsigned int,
    pub opts): *mut *mut *mut extern int cifs_root_data(char dev, char,

//! Automatically rewritten from C Header to Rust Module
//! Source: samples/vfs/samples-vfs.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct statmount {
    pub /: *mut *mut __u32 size; / Total size, including strings,
    pub /: *mut *mut __u32 mnt_opts; / [str] Options (comma separated, escaped),
    pub /: *mut *mut __u64 mask; / What results were written,
    pub /: *mut *mut __u32 sb_dev_major; / Device ID,
    pub sb_dev_minor: __u32,
    pub /: *mut *mut __u64 sb_magic; / ..._SUPER_MAGIC,
    pub /: *mut *mut __u32 sb_flags; / SB_{RDONLY,SYNCHRONOUS,DIRSYNC,LAZYTIME},
    pub /: *mut *mut __u32 fs_type; / [str] Filesystem type,
    pub /: *mut *mut __u64 mnt_id; / Unique ID of mount,
    pub /: *mut *mut __u64 mnt_parent_id; / Unique ID of parent (for root == mnt_id),
    pub /: *mut *mut __u32 mnt_id_old; / Reused IDs used in proc/.../mountinfo,
    pub mnt_parent_id_old: __u32,
    pub /: *mut *mut __u64 mnt_attr; / MOUNT_ATTR_...,
    pub /: *mut *mut __u64 mnt_propagation; / MS_{SHARED,SLAVE,PRIVATE,UNBINDABLE},
    pub /: *mut *mut __u64 mnt_peer_group; / ID of shared peer group,
    pub /: *mut *mut __u64 mnt_master; / Mount receives propagation from this ID,
    pub /: *mut *mut __u64 propagate_from; / Propagation from in current namespace,
    pub /: *mut *mut __u32 mnt_root; / [str] Root of mount relative to root of fs,
    pub /: *mut *mut __u32 mnt_point; / [str] Mountpoint relative to current root,
    pub /: *mut *mut __u64 mnt_ns_id; / ID of the mount namespace,
    pub /: *mut *mut __u32 fs_subtype; / [str] Subtype of fs_type (if any),
    pub /: *mut *mut __u32 sb_source; / [str] Source string of the mount,
    pub /: *mut *mut __u32 opt_num; / Number of fs options,
    pub /: *mut *mut __u32 opt_array; / [str] Array of nul terminated fs options,
    pub /: *mut *mut __u32 opt_sec_num; / Number of security options,
    pub /: *mut *mut __u32 opt_sec_array; / [str] Array of nul terminated security options,
    pub /: *mut *mut __u32 mnt_uidmap_num; / Number of uid mappings,
    pub /: *mut *mut __u32 mnt_uidmap; / [str] Array of uid mappings,
    pub /: *mut *mut __u32 mnt_gidmap_num; / Number of gid mappings,
    pub /: *mut *mut __u32 mnt_gidmap; / [str] Array of gid mappings,
    pub __spare2: [__u64; 44],
    pub /: *mut *mut char str[]; / Variable size part containing strings,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mnt_id_req {
    pub size: __u32,
    pub spare: __u32,
    pub mnt_id: __u64,
    pub param: __u64,
    pub mnt_ns_id: __u64,
}

// Get the id for a mount namespace

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mnt_ns_info {
    pub size: __u32,
    pub nr_mounts: __u32,
    pub mnt_ns_id: __u64,
}

pub const __NR_listmount: c_int = 458;

pub const __NR_statmount: c_int = 457;

pub const LSMT_ROOT: c_uint = 0xffffffffffffffff	/* root mount */;

// @mask bits for statmount(2)

pub const STATMOUNT_SB_BASIC: c_uint = 0x00000001U /* Want/got sb_... */;

pub const STATMOUNT_MNT_BASIC: c_uint = 0x00000002U /* Want/got mnt_... */;

pub const STATMOUNT_PROPAGATE_FROM: c_uint = 0x00000004U /* Want/got propagate_from */;

pub const STATMOUNT_MNT_ROOT: c_uint = 0x00000008U /* Want/got mnt_root  */;

pub const STATMOUNT_MNT_POINT: c_uint = 0x00000010U /* Want/got mnt_point */;

pub const STATMOUNT_FS_TYPE: c_uint = 0x00000020U /* Want/got fs_type */;

pub const STATMOUNT_MNT_NS_ID: c_uint = 0x00000040U /* Want/got mnt_ns_id */;

pub const STATMOUNT_MNT_OPTS: c_uint = 0x00000080U /* Want/got mnt_opts */;

pub const STATMOUNT_FS_SUBTYPE: c_uint = 0x00000100U /* Want/got fs_subtype */;

pub const STATMOUNT_SB_SOURCE: c_uint = 0x00000200U /* Want/got sb_source */;

pub const STATMOUNT_OPT_ARRAY: c_uint = 0x00000400U /* Want/got opt_... */;

pub const STATMOUNT_OPT_SEC_ARRAY: c_uint = 0x00000800U /* Want/got opt_sec... */;

pub const STATX_MNT_ID_UNIQUE: c_uint = 0x00004000U /* Want/got extended stx_mount_id */;

pub const STATMOUNT_MNT_UIDMAP: c_uint = 0x00002000U	/* Want/got uidmap... */;

pub const STATMOUNT_MNT_GIDMAP: c_uint = 0x00004000U	/* Want/got gidmap... */;

pub const MOUNT_ATTR_RDONLY: c_uint = 0x00000001 /* Mount read-only */;

pub const MOUNT_ATTR_NOSUID: c_uint = 0x00000002 /* Ignore suid and sgid bits */;

pub const MOUNT_ATTR_NODEV: c_uint = 0x00000004 /* Disallow access to device special files */;

pub const MOUNT_ATTR_NOEXEC: c_uint = 0x00000008 /* Disallow program execution */;

pub const MOUNT_ATTR__ATIME: c_uint = 0x00000070 /* Setting on how atime should be updated */;

pub const MOUNT_ATTR_RELATIME: c_uint = 0x00000000 /* - Update atime relative to mtime/ctime. */;

pub const MOUNT_ATTR_NOATIME: c_uint = 0x00000010 /* - Do not update access times. */;

pub const MOUNT_ATTR_STRICTATIME: c_uint = 0x00000020 /* - Always perform atime updates */;

pub const MOUNT_ATTR_NODIRATIME: c_uint = 0x00000080 /* Do not update directory access times */;

pub const MOUNT_ATTR_IDMAP: c_uint = 0x00100000 /* Idmap mount to @userns_fd in struct mount_attr. */;

pub const MOUNT_ATTR_NOSYMFOLLOW: c_uint = 0x00200000 /* Do not follow symlinks */;


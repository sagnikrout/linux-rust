//! Automatically rewritten from C Header to Rust Module
//! Source: block/partitions/ldm.h
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0-or-later
//
// ldm - Part of the Linux-NTFS project.
//
// Copyright (C) 2001,2002 Richard Russon <ldm@flatcap.org>
// Copyright (c) 2001-2007 Anton Altaparmakov
// Copyright (C) 2001,2002 Jakob Kemi <jakob.kemi@telia.com>
//
// Documentation is available at http://www.linux-ntfs.org/doku.php?id=downloads
//

// Magic numbers in CPU format.
pub const MAGIC_VMDB: c_uint = 0x564D4442		/* VMDB */;
pub const MAGIC_VBLK: c_uint = 0x56424C4B		/* VBLK */;
pub const MAGIC_PRIVHEAD: c_uint = 0x5052495648454144ULL	/* PRIVHEAD */;
pub const MAGIC_TOCBLOCK: c_uint = 0x544F43424C4F434BULL	/* TOCBLOCK */;
// The defined vblk types.
pub const VBLK_VOL5: c_uint = 0x51		/* Volume,     version 5 */;
pub const VBLK_CMP3: c_uint = 0x32		/* Component,  version 3 */;
pub const VBLK_PRT3: c_uint = 0x33		/* Partition,  version 3 */;
pub const VBLK_DSK3: c_uint = 0x34		/* Disk,       version 3 */;
pub const VBLK_DSK4: c_uint = 0x44		/* Disk,       version 4 */;
pub const VBLK_DGR3: c_uint = 0x35		/* Disk Group, version 3 */;
pub const VBLK_DGR4: c_uint = 0x45		/* Disk Group, version 4 */;
// vblk flags indicating extra information will be present
pub const VBLK_FLAG_COMP_STRIPE: c_uint = 0x10;
pub const VBLK_FLAG_PART_INDEX: c_uint = 0x08;
pub const VBLK_FLAG_DGR3_IDS: c_uint = 0x08;
pub const VBLK_FLAG_DGR4_IDS: c_uint = 0x08;
pub const VBLK_FLAG_VOLU_ID1: c_uint = 0x08;
pub const VBLK_FLAG_VOLU_ID2: c_uint = 0x20;
pub const VBLK_FLAG_VOLU_SIZE: c_uint = 0x80;
pub const VBLK_FLAG_VOLU_DRIVE: c_uint = 0x02;
// size of a vblk's static parts
pub const VBLK_SIZE_HEAD: c_int = 16;

pub const VBLK_SIZE_DGR3: c_int = 12;
pub const VBLK_SIZE_DGR4: c_int = 44;
pub const VBLK_SIZE_DSK3: c_int = 12;
pub const VBLK_SIZE_DSK4: c_int = 45;
pub const VBLK_SIZE_PRT3: c_int = 28;
pub const VBLK_SIZE_VOL5: c_int = 58;
// component types
pub const COMP_STRIPE: c_uint = 0x01		/* Stripe-set */;
pub const COMP_BASIC: c_uint = 0x02		/* Basic disk */;
pub const COMP_RAID: c_uint = 0x03		/* Raid-set */;
// Other constants.

// Offsets to structures within the LDM Database in sectors.

pub const OFF_PRIV3: c_int = 2047;

pub const OFF_TOCB2: c_int = 2;
pub const OFF_TOCB3: c_int = 2045;
pub const OFF_TOCB4: c_int = 2046;

pub const LDM_PARTITION: c_uint = 0x42		/* Formerly SFS (Landis). */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frag {
    pub list: list_head,
    pub group: u32,
//     pub /: *mut *mut u8 num; / Total number of records,
//     pub /: *mut *mut u8 rec; / This is record number n,
//     pub /: *mut *mut u8 map; / Which portions are in use,
    pub data: [u8; 0],
}

// In memory LDM database structures.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct privhead {
    pub ver_major: u16,
    pub ver_minor: u16,
    pub logical_disk_start: u64,
    pub logical_disk_size: u64,
    pub config_start: u64,
    pub config_size: u64,
    pub disk_id: uuid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tocblock {
    pub bitmap1_name: [u8; 16],
    pub bitmap1_start: u64,
    pub bitmap1_size: u64,
    pub bitmap2_name: [u8; 16],
    pub bitmap2_start: u64,
    pub bitmap2_size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmdb {
    pub ver_major: u16,
    pub ver_minor: u16,
    pub vblk_size: u32,
    pub vblk_offset: u32,
    pub last_vblk_seq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vblk_comp {
    pub state: [u8; 16],
    pub parent_id: u64,
    pub type: u8,
    pub children: u8,
    pub chunksize: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vblk_dgrp {
    pub disk_id: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vblk_disk {
    pub disk_id: uuid_t,
    pub alt_name: [u8; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vblk_part {
    pub start: u64,
//     pub /: *mut *mut u64 size; / start, size and vol_off in sectors,
    pub volume_offset: u64,
    pub parent_id: u64,
    pub disk_id: u64,
    pub partnum: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vblk_volu {
    pub volume_type: [u8; 16],
    pub volume_state: [u8; 16],
    pub guid: [u8; 16],
    pub drive_hint: [u8; 4],
    pub size: u64,
    pub partition_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vblk_head {
    pub group: u32,
    pub rec: u16,
    pub nrec: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vblk {
    pub name: [u8; 64],
    pub obj_id: u64,
    pub sequence: u32,
    pub flags: u8,
    pub type: u8,
    pub comp: vblk_comp,
    pub dgrp: vblk_dgrp,
    pub disk: vblk_disk,
    pub part: vblk_part,
    pub volu: vblk_volu,
    pub vblk: },
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ldmdb {
    pub ph: privhead,
    pub toc: tocblock,
    pub vm: vmdb,
    pub v_dgrp: list_head,
    pub v_disk: list_head,
    pub v_volu: list_head,
    pub v_comp: list_head,
    pub v_part: list_head,
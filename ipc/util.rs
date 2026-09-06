//! Automatically rewritten from C Header to Rust Module
//! Source: ipc/util.h
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
macro_rules! printk { ($($tt:tt)*) => { 0 }; }
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
macro_rules! rootfs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! pure_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! min_t { ($($tt:tt)*) => { 0 }; }
macro_rules! max_t { ($($tt:tt)*) => { 0 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! MKDEV { ($($tt:tt)*) => { 0u32 }; }
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
macro_rules! pr_warn_once { ($($tt:tt)*) => {}; }
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
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_ids { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

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
pub type compat_uptr_t = u32;
pub type compat_long_t = i32;
pub type compat_ulong_t = u32;
pub type compat_size_t = u32;
pub type __compat_uid_t = u32;
pub type __compat_gid_t = u32;
pub type compat_mode_t = u32;
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
pub const ENOSYS: c_int = 38;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;
pub const SHMLBA: usize = 4096;
pub const COMPAT_SHMLBA: usize = 4096;

// Standard File Mode Constants
pub const S_IFCHR: u32 = 0x2000;
pub const S_IFDIR: u32 = 0x4000;
pub const S_IFREG: u32 = 0x8000;
pub const S_IFBLK: u32 = 0x6000;
pub const S_IFIFO: u32 = 0x1000;
pub const S_IFLNK: u32 = 0xa000;
pub const S_IFSOCK: u32 = 0xc000;
pub const S_IRWXU: u32 = 0x01c0;
pub const S_IRUSR: u32 = 0x0100;
pub const S_IWUSR: u32 = 0x0080;
pub const S_IXUSR: u32 = 0x0040;
pub const S_IRUGO: u32 = 0x0124;
pub const S_IWUGO: u32 = 0x0092;
pub const S_IXUGO: u32 = 0x0049;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn usermodehelper_enable();
    pub fn new_encode_dev(dev: u32) -> u32;
}

pub unsafe fn init_mkdir<T>(_path: T, _mode: u32) -> c_int { 0 }
pub unsafe fn init_mknod<T>(_path: T, _mode: u32, _dev: u32) -> c_int { 0 }
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0
//
// linux/ipc/util.h
// Copyright (C) 1999 Christoph Rohland
//
// ipc helper functions (c) 1999 Manfred Spraul <manfred@colorfullife.com>
// namespaces support.      2006 OpenVZ, SWsoft Inc.
// Pavel Emelianov <xemul@openvz.org>
//

//
// The IPC ID contains 2 separate numbers - index and sequence number.
// By default,
// bits  0-14: index (32k, 15 bits)
// bits 15-30: sequence number (64k, 16 bits)
//
// When IPCMNI extension mode is turned on, the composition changes:
// bits  0-23: index (16M, 24 bits)
// bits 24-30: sequence number (128, 7 bits)
//
pub const IPCMNI_SHIFT: c_int = 15;
pub const IPCMNI_EXTEND_SHIFT: c_int = 24;

extern "C" {
    pub fn sem_init();
}
extern "C" {
    pub fn msg_init();
}
extern "C" {
    pub fn shm_init();
}

extern "C" {
    pub fn mq_clear_sbinfo(ns: *mut ipc_namespace);
}

extern "C" {
    pub fn sem_init_ns(ns: *mut ipc_namespace);
}
extern "C" {
    pub fn msg_init_ns(ns: *mut ipc_namespace) -> c_int;
}
extern "C" {
    pub fn shm_init_ns(ns: *mut ipc_namespace);
}
extern "C" {
    pub fn sem_exit_ns(ns: *mut ipc_namespace);
}
extern "C" {
    pub fn msg_exit_ns(ns: *mut ipc_namespace);
}
extern "C" {
    pub fn shm_exit_ns(ns: *mut ipc_namespace);
}

//
// Structure that holds the parameters needed by the ipc operations
// (see after)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params {
    pub key: key_t,
    pub flg: c_int,
//     pub /: *mut *mut size_t size; / for shared memories,
//     pub /: *mut *mut int nsems; / for semaphores,
//     pub /: *mut *mut } u; / holds the getnew() specific param,
}

//
// Structure that holds some ipc operations. This structure is used to unify
// the calls to sys_msgget(), sys_semget(), sys_shmget()
// . routine to call to create a new ipc object. Can be one of newque,
// newary, newseg
// . routine to call to check permissions for a new ipc object.
// Can be one of security_msg_associate, security_sem_associate,
// security_shm_associate
// . routine to call for an extra check if needed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_ops {
// fn ptr field
// fn ptr field
// fn ptr field
}

extern "C" {
    pub fn ipc_init_ids(ids: *mut ipc_ids);
}

pub const IPC_SEM_IDS: c_int = 0;
pub const IPC_MSG_IDS: c_int = 1;
pub const IPC_SHM_IDS: c_int = 2;

// must be called with ids->rwsem acquired for writing
extern "C" {
    pub fn ipc_addid(_: *mut ipc_ids, _: *mut kern_ipc_perm, _arg: c_int) -> c_int;
}
// must be called with both locks acquired.
extern "C" {
    pub fn ipc_rmid(_: *mut ipc_ids, _: *mut kern_ipc_perm);
}
// must be called with both locks acquired.
extern "C" {
    pub fn ipc_set_key_private(_: *mut ipc_ids, _: *mut kern_ipc_perm);
}
// must be called with ipcp locked
extern "C" {
    pub fn ipcperms(ns: *mut ipc_namespace, ipcp: *mut kern_ipc_perm, flg: c_short) -> c_int;
}
//
// ipc_get_maxidx - get the highest assigned index
// @ids: ipc identifier set
//
// The function returns the highest assigned index for @ids. The function
// doesn't scan the idr tree, it uses a cached value.
//
// Called with ipc_ids.rwsem held for reading.
//
// For allocation that need to be freed by RCU.
// Objects are reference counted, they start with reference count 1.
// getref increases the refcount, the putref call that reduces the recount
// to 0 schedules the rcu destruction. Caller must guarantee locking.
//
// refcount is initialized by ipc_addid(), before that point call_rcu()
// must be used.
//
extern "C" {
    pub fn ipc_rcu_getref(ptr: *mut kern_ipc_perm) -> bool;
}
extern "C" {
    pub fn kernel_to_ipc64_perm(r#in: *mut kern_ipc_perm, out: *mut ipc64_perm);
}
extern "C" {
    pub fn ipc64_perm_to_ipc_perm(r#in: *mut ipc64_perm, out: *mut ipc_perm);
}
extern "C" {
    pub fn ipc_update_perm(r#in: *mut ipc64_perm, out: *mut kern_ipc_perm) -> c_int;
}
// pos = get_pid(pid);

extern "C" {
    pub fn ipc_parse_version(cmd: *mut c_int) -> c_int;
}

extern "C" {
    pub fn free_msg(msg: *mut msg_msg);
}
extern "C" {
    pub fn store_msg(dest: *mut c_void , msg: *mut msg_msg, len: usize) -> c_int;
}
//
// ipc_valid_object() - helper to sort out IPC_RMID races for codepaths
// where the respective ipc_ids.rwsem is not being held down.
// Checks whether the ipc object is still around or if it's gone already, as
// ipc_rmid() may have already freed the ID while the ipc lock was spinning.
// Needs to be called with kern_ipc_perm.lock held -- exception made for one
// checkpoint case at sys_semtimedop() as noted in code commentary.
//
// Check semmni range [0, ipc_mni]
// semmni is the last element of sem_ctls[4] array
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm {
    pub key: key_t,
    pub uid: __compat_uid_t,
    pub gid: __compat_gid_t,
    pub cuid: __compat_uid_t,
    pub cgid: __compat_gid_t,
    pub mode: compat_mode_t,
    pub seq: c_ushort,
}

extern "C" {
    pub fn to_compat_ipc_perm(_: *mut compat_ipc_perm, _: *mut ipc64_perm);
}
extern "C" {
    pub fn to_compat_ipc64_perm(_: *mut compat_ipc64_perm, _: *mut ipc64_perm);
}
extern "C" {
    pub fn get_compat_ipc_perm(_: *mut ipc64_perm, _: *mut compat_ipc_perm ) -> c_int;
}
// cmd &= !IPC_64;
extern "C" {
    pub fn compat_ksys_old_semctl(semid: c_int, semnum: c_int, cmd: c_int, arg: c_int) -> c_long;
}
extern "C" {
    pub fn compat_ksys_old_msgctl(msqid: c_int, cmd: c_int, uptr: *mut c_void ) -> c_long;
}
extern "C" {
    pub fn compat_ksys_old_shmctl(shmid: c_int, cmd: c_int, uptr: *mut c_void ) -> c_long;
}
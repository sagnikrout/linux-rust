//! Automatically rewritten from C to Rust
//! Source: block/blk-ia-ranges.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Block device concurrent positioning ranges.
//
// Copyright (C) 2021 Western Digital Corporation or its Affiliates.
//

#[no_mangle]
pub unsafe extern "C" fn blk_ia_range_sector_show(iar: *mut blk_independent_access_range, buf: *mut c_char) -> ssize_t {
    return sprintf(buf, "%llu\n", iar.sector);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_ia_range_nr_sectors_show(iar: *mut blk_independent_access_range, buf: *mut c_char) -> ssize_t {
    return sprintf(buf, "%llu\n", iar.nr_sectors);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_ia_range_sysfs_entry {
    pub attr: attribute,
    pub buf): *mut *mut *mut ssize_t (show)(blk_independent_access_range iar, char,
}

pub static mut blk_ia_range_sysfs_entry: usize = 0;
pub static mut blk_ia_range_sysfs_entry: usize = 0;
    static const struct attribute *const blk_ia_range_attrs[] = {
    &blk_ia_range_sector_entry.attr,
    &blk_ia_range_nr_sectors_entry.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(blk_ia_range);
#[no_mangle]
pub unsafe extern "C" fn blk_ia_range_sysfs_show(kobj: *mut kobject, attr: *mut attribute, buf: *mut c_char) -> ssize_t {
    let mut entry = container_of!(attr, blk_ia_range_sysfs_entry, attr);
    let mut iar = container_of!(kobj, blk_independent_access_range, kobj);
    return entry.show(iar, buf);
    }
pub static mut sysfs_ops: usize = 0;
//
// Independent access range entries are not freed individually, but alltogether
// with struct blk_independent_access_ranges and its array of ranges. Since
// kobject_add() takes a reference on the parent kobject contained in
// struct blk_independent_access_ranges, the array of independent access range
// entries cannot be freed until kobject_del() is called for all entries.
// So we do not need to do anything here, but still need this no-op release
// operation to avoid complaints from the kobject code.
//
#[no_mangle]
unsafe extern "C" fn blk_ia_range_sysfs_nop_release(kobj: *mut kobject) {
    }
pub static mut kobj_type: usize = 0;
//
// This will be executed only after all independent access range entries are
// removed with kobject_del(), at which point, it is safe to free everything,
// including the array of ranges.
//
#[no_mangle]
unsafe extern "C" fn blk_ia_ranges_sysfs_release(kobj: *mut kobject) {
    let mut iars = container_of!(kobj, blk_independent_access_ranges, kobj);
    kfree(iars);
    }
pub static mut kobj_type: usize = 0;
//
// disk_register_independent_access_ranges - register with sysfs a set of
// independent access ranges
// @disk:	Target disk
//
// Register with sysfs a set of independent access ranges for @disk.
//
#[no_mangle]
pub unsafe extern "C" fn disk_register_independent_access_ranges(disk: *mut gendisk) -> c_int {
    let mut iars = disk.ia_ranges;
    let mut q = disk.queue;
    let mut i = 0;
    let mut ret = 0;
    lockdep_assert_held(&q.sysfs_lock);
    if (!iars) {
    return 0;
    }
//
// At this point, iars is the new set of sector access ranges that needs
// to be registered with sysfs.
//
    WARN_ON!(iars.sysfs_registered);
    ret = kobject_init_and_add(&iars.kobj, &blk_ia_ranges_ktype,
    &disk.queue_kobj, "%s",
    "independent_access_ranges");
    if (ret) {
    disk.ia_ranges = core::ptr::null_mut();
    kobject_put(&iars.kobj);
    return ret;
    }
    while (i < iars.nr_ia_ranges) {
    ret = kobject_init_and_add(&iars.ia_range[i].kobj,
    &blk_ia_range_ktype, &iars.kobj,
    "%d", i);
    if (ret) {
    while (--i >= 0) {
    kobject_del(&iars.ia_range[i].kobj);
    }
    kobject_del(&iars.kobj);
    kobject_put(&iars.kobj);
    return ret;
    }
    }
    iars.sysfs_registered = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn disk_unregister_independent_access_ranges(disk: *mut gendisk) {
    let mut q = disk.queue;
    let mut iars = disk.ia_ranges;
    let mut i = 0;
    lockdep_assert_held(&q.sysfs_lock);
    if (!iars) {
    return;
    }
    if (iars.sysfs_registered) {
    for (i = 0; i < iars.nr_ia_ranges; i++) {
    kobject_del(&iars.ia_range[i].kobj);
    }
    kobject_del(&iars.kobj);
    kobject_put(&iars.kobj);
    } else {
    kfree(iars);
    }
    disk.ia_ranges = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn disk_find_ia_range(iars: *mut blk_independent_access_ranges, sector: sector_t) -> *mut c_void {
pub static mut iar: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < iars.nr_ia_ranges) {
    iar = &iars.ia_range[i];
    if (sector >= iar.sector &&
    sector < iar.sector + iar.nr_sectors) {
    return iar;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn disk_check_ia_ranges(disk: *mut gendisk, iars: *mut blk_independent_access_ranges) -> bool {
    let mut iar = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
pub static mut capacity: sector_t = 0;
pub static mut sector: sector_t = 0;
    let mut i = 0;
    if (WARN_ON_ONCE!(!iars.nr_ia_ranges)) {
    return false;
    }
//
// While sorting the ranges in increasing LBA order, check that the
// ranges do not overlap, that there are no sector holes and that all
// sectors belong to one range.
//
    while (i < iars.nr_ia_ranges) {
    tmp = disk_find_ia_range(iars, sector);
    if (!tmp || tmp.sector != sector) {
    pr_warn!("Invalid non-contiguous independent access ranges\n");
    return false;
    }
    iar = &iars.ia_range[i];
    if (tmp != iar) {
    swap(iar.sector, tmp.sector);
    swap(iar.nr_sectors, tmp.nr_sectors);
    }
    sector += iar.nr_sectors;
    }
    if (sector != capacity) {
    pr_warn!("Independent access ranges do not match disk capacity\n");
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn disk_ia_ranges_changed(disk: *mut gendisk, new: *mut blk_independent_access_ranges) -> bool {
    let mut old = disk.ia_ranges;
    let mut i = 0;
    if (!old) {
    return true;
    }
    if (old.nr_ia_ranges != new.nr_ia_ranges) {
    return true;
    }
    while (i < old.nr_ia_ranges) {
    if (new.ia_range[i].sector != old.ia_range[i].sector ||
    new.ia_range[i].nr_sectors != old.ia_range[i].nr_sectors) {
    return true;
    }
    }
    return false;
    }
//
// disk_alloc_independent_access_ranges - Allocate an independent access ranges
// data structure
// @disk:		target disk
// @nr_ia_ranges:	Number of independent access ranges
//
// Allocate a struct blk_independent_access_ranges structure with @nr_ia_ranges
// access range descriptors.
//
#[no_mangle]
pub unsafe extern "C" fn disk_alloc_independent_access_ranges(disk: *mut gendisk, nr_ia_ranges: c_int) -> *mut c_void {
pub static mut iars: *mut c_void = core::ptr::null_mut();
    iars = kzalloc_node(struct_size(iars, ia_range, nr_ia_ranges),
    GFP_KERNEL, disk.queue.node);
    if (iars) {
    iars.nr_ia_ranges = nr_ia_ranges;
    }
    return iars;
    }
    EXPORT_SYMBOL_GPL(disk_alloc_independent_access_ranges);
//
// disk_set_independent_access_ranges - Set a disk independent access ranges
// @disk:	target disk
// @iars:	independent access ranges structure
//
// Set the independent access ranges information of the request queue
// of @disk to @iars. If @iars is NULL and the independent access ranges
// structure already set is cleared. If there are no differences between
// @iars and the independent access ranges structure already set, @iars
// is freed.
//
#[no_mangle]
pub unsafe extern "C" fn disk_set_independent_access_ranges(disk: *mut gendisk, iars: *mut blk_independent_access_ranges) {
    let mut q = disk.queue;
    mutex_lock(&q.sysfs_lock);
    if (iars && !disk_check_ia_ranges(disk, iars)) {
    kfree(iars);
    iars = core::ptr::null_mut();
    }
    if (iars && !disk_ia_ranges_changed(disk, iars)) {
    kfree(iars);
// goto;
    }
//
// This may be called for a registered queue. E.g. during a device
// revalidation. If that is the case, we need to unregister the old
// set of independent access ranges and register the new set. If the
// queue is not registered, registration of the device request queue
// will register the independent access ranges.
//
    disk_unregister_independent_access_ranges(disk);
    disk.ia_ranges = iars;
    if (blk_queue_registered(q)) {
    disk_register_independent_access_ranges(disk);
    }
// label;
    mutex_unlock(&q.sysfs_lock);
    }
    EXPORT_SYMBOL_GPL(disk_set_independent_access_ranges);
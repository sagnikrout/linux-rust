//! Automatically rewritten from C to Rust
//! Source: block/partitions/cmdline.c
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
// Copyright (C) 2013 HUAWEI
// Author: Cai Zhiyong <caizhiyong@huawei.com>
//
// Read block device partition table from the command line.
// Typically used for fixed block (eMMC) embedded devices.
// It has no MBR, so saves storage space. Bootloader can be easily accessed
// by absolute address of data on the block device.
// Users can easily change the partition.
//
// The format for the command line is just like mtdparts.
//
// For further information, see "Documentation/block/cmdline-partition.rst"
//

// partition flags
pub const PF_RDONLY: c_uint = 0x01 /* Device is read only */;
pub const PF_POWERUP_LOCK: c_uint = 0x02 /* Always locked after reset */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdline_subpart {
//     pub /: *mut *mut char name[BDEVNAME_SIZE]; / partition name, such as 'rootfs',
    pub from: sector_t,
    pub size: sector_t,
    pub flags: c_int,
    pub next_subpart: *mut cmdline_subpart,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdline_parts {
//     pub /: *mut *mut char name[BDEVNAME_SIZE]; / block device, such as 'mmcblk0',
    pub nr_subparts: c_uint,
    pub subpart: *mut cmdline_subpart,
    pub next_parts: *mut cmdline_parts,
}

#[no_mangle]
unsafe extern "C" fn parse_subpart(subpart: *mut cmdline_subpart, partdef: *mut c_char) -> c_int {
pub static mut ret: c_int = 0;
pub static mut new_subpart: *mut c_void = core::ptr::null_mut();
// subpart = NULL;
    new_subpart = kzalloc_obj(cmdline_subpart);
    if (!new_subpart) {
    return -ENOMEM;
    }
    if (*partdef == '-') {
    new_subpart.size = (sector_t)(~0ULL);
    partdef += 1;
    } else {
    new_subpart.size = (sector_t)memparse(partdef, &partdef);
    if (new_subpart.size < (sector_t)PAGE_SIZE) {
    pr_warn!("cmdline partition size is invalid.");
    ret = -EINVAL;
// goto;
    }
    }
    if (*partdef == '@') {
    partdef += 1;
    new_subpart.from = (sector_t)memparse(partdef, &partdef);
    } else {
    new_subpart.from = (sector_t)(~0ULL);
    }
    if (*partdef == '(') {
    partdef += 1;
    let mut next = strsep(&partdef, ")");
    if (!next) {
    pr_warn!("cmdline partition format is invalid.");
    ret = -EINVAL;
// goto;
    }
    strscpy(new_subpart.name, next, sizeof!(new_subpart.name));
    } else {
    new_subpart.name[0] = '\0';
    }
    new_subpart.flags = 0;
    if (!strncmp(partdef, "ro", 2)) {
    new_subpart.flags |= PF_RDONLY;
    partdef += 2;
    }
    if (!strncmp(partdef, "lk", 2)) {
    new_subpart.flags |= PF_POWERUP_LOCK;
    partdef += 2;
    }
// subpart = new_subpart;
    return 0;
// label;
    kfree(new_subpart);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn free_subpart(parts: *mut cmdline_parts) {
pub static mut subpart: *mut c_void = core::ptr::null_mut();
    while (parts.subpart) {
    subpart = parts.subpart;
    parts.subpart = subpart.next_subpart;
    kfree(subpart);
    }
    }
#[no_mangle]
unsafe extern "C" fn parse_parts(parts: *mut cmdline_parts, bdevdef: *mut c_char) -> c_int {
pub static mut ret: c_int = 0;
pub static mut next: *mut c_void = core::ptr::null_mut();
pub static mut next_subpart: *mut c_void = core::ptr::null_mut();
pub static mut newparts: *mut c_void = core::ptr::null_mut();
// parts = NULL;
    newparts = kzalloc_obj(cmdline_parts);
    if (!newparts) {
    return -ENOMEM;
    }
    next = strsep(&bdevdef, ":");
    if (!next) {
    pr_warn!("cmdline partition has no block device.");
// goto;
    }
    strscpy(newparts.name, next, sizeof!(newparts.name));
    newparts.nr_subparts = 0;
    next_subpart = &newparts.subpart;
    while ((next = strsep(&bdevdef, ","))) {
    ret = parse_subpart(next_subpart, next);
    if (ret) {
// goto;
    }
    newparts.nr_subparts += 1;
    next_subpart = &(*next_subpart).next_subpart;
    }
    if (!newparts.subpart) {
    pr_warn!("cmdline partition has no valid partition.");
    ret = -EINVAL;
// goto;
    }
// parts = newparts;
    return 0;
// label;
    free_subpart(newparts);
    kfree(newparts);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cmdline_parts_free(parts: *mut cmdline_parts) {
pub static mut next_parts: *mut c_void = core::ptr::null_mut();
    while (*parts) {
    next_parts = (*parts).next_parts;
    free_subpart(*parts);
    kfree(*parts);
// parts = next_parts;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cmdline_parts_parse(parts: *mut *mut cmdline_parts, cmdline: *mut c_char) -> c_int {
    let mut ret = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
pub static mut pbuf: *mut c_void = core::ptr::null_mut();
pub static mut next: *mut c_void = core::ptr::null_mut();
pub static mut next_parts: *mut c_void = core::ptr::null_mut();
// parts = NULL;
    pbuf = buf = kstrdup(cmdline, GFP_KERNEL);
    if (!buf) {
    return -ENOMEM;
    }
    next_parts = parts;
    while ((next = strsep(&pbuf, ";"))) {
    ret = parse_parts(next_parts, next);
    if (ret) {
// goto;
    }
    next_parts = &(*next_parts).next_parts;
    }
    if (!*parts) {
    pr_warn!("cmdline partition has no valid partition.");
    ret = -EINVAL;
// goto;
    }
    ret = 0;
// label;
    kfree(buf);
    return ret;
// label;
    cmdline_parts_free(parts);
// goto;
    }
#[no_mangle]
pub unsafe extern "C" fn cmdline_parts_find(parts: *mut cmdline_parts, bdev: *mut c_char) -> *mut c_void {
    while (parts && strncmp(bdev, parts.name, sizeof!(parts.name))) {
    parts = parts.next_parts;
    }
    return parts;
    }
pub static mut cmdline: *mut c_void = core::ptr::null_mut();
pub static mut bdev_parts: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn add_part(slot: c_int, subpart: *mut cmdline_subpart, state: *mut parsed_partitions) -> c_int {
pub static mut info: *mut c_void = core::ptr::null_mut();
    if (slot >= state.limit) {
    return 1;
    }
    put_partition(state, slot, subpart.from >> 9,
    subpart.size >> 9);
    if (subpart.flags & PF_RDONLY) {
    state.parts[slot].flags |= ADDPART_FLAG_READONLY;
    }
    info = &state.parts[slot].info;
    strscpy(info.volname, subpart.name, sizeof!(info.volname));
    seq_buf_printf(&state.pp_buf, "(%s)", info.volname);
    state.parts[slot].has_info = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cmdline_parts_set(parts: *mut cmdline_parts, disk_size: sector_t, state: *mut parsed_partitions) -> c_int {
pub static mut from: sector_t = 0;
pub static mut subpart: *mut c_void = core::ptr::null_mut();
pub static mut slot: c_int = 1;
    while (subpart) {
    if (subpart.from == (sector_t)(~0ULL)) {
    subpart.from = from;
    }
    else {
    from = subpart.from;
    }
    if (from >= disk_size) {
    break;
    }
    if (subpart.size > (disk_size - from)) {
    subpart.size = disk_size - from;
    }
    from += subpart.size;
    if (add_part(slot, subpart, state)) {
    break;
    }
    }
    return slot;
    }
#[no_mangle]
unsafe extern "C" fn cmdline_parts_setup(s: *mut c_char) -> c_int {
    cmdline = s;
    return 1;
    }
    __setup!("blkdevparts=", cmdline_parts_setup);
#[no_mangle]
pub unsafe extern "C" fn has_overlaps(from: sector_t, size: sector_t, from2: sector_t, size2: sector_t) -> bool {
pub static mut end: sector_t = 0;
pub static mut end2: sector_t = 0;
    if (from >= from2 && from < end2) {
    return true;
    }
    if (end > from2 && end <= end2) {
    return true;
    }
    if (from2 >= from && from2 < end) {
    return true;
    }
    if (end2 > from && end2 <= end) {
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn overlaps_warns_header() {
    pr_warn!("Overlapping partitions are used in command line partitions.");
    pr_warn!("Don't use filesystems on overlapping partitions:");
    }
#[no_mangle]
unsafe extern "C" fn cmdline_parts_verifier(slot: c_int, state: *mut parsed_partitions) {
    let mut i = 0;
pub static mut header: bool = true;
    while (slot < state.limit && state.parts[slot].has_info) {
    while (i < state.limit && state.parts[i].has_info) {
    if (has_overlaps(state.parts[slot].from,
    state.parts[slot].size,
    state.parts[i].from,
    state.parts[i].size)) {
    if (header) {
    header = false;
    overlaps_warns_header();
    }
    pr_warn!("%s[%llu,%llu] overlaps with "
    "%s[%llu,%llu].",
    state.parts[slot].info.volname,
    (u64)state.parts[slot].from << 9,
    (u64)state.parts[slot].size << 9,
    state.parts[i].info.volname,
    (u64)state.parts[i].from << 9,
    (u64)state.parts[i].size << 9);
    }
    }
    }
    }
//
// Purpose: allocate cmdline partitions.
// Returns:
// -1 if unable to read the partition table
// 0 if this isn't our partition table
// 1 if successful
//
#[no_mangle]
pub unsafe extern "C" fn cmdline_partition(state: *mut parsed_partitions) -> c_int {
    let mut disk_size;
pub static mut parts: *mut c_void = core::ptr::null_mut();
    if (cmdline) {
    if (bdev_parts) {
    cmdline_parts_free(&bdev_parts);
    }
    if (cmdline_parts_parse(&bdev_parts, cmdline)) {
    cmdline = core::ptr::null_mut();
    return -1;
    }
    cmdline = core::ptr::null_mut();
    }
    if (!bdev_parts) {
    return 0;
    }
    parts = cmdline_parts_find(bdev_parts, state.disk.disk_name);
    if (!parts) {
    return 0;
    }
    disk_size = get_capacity(state.disk) << 9;
    cmdline_parts_set(parts, disk_size, state);
    cmdline_parts_verifier(1, state);
    seq_buf_puts(&state.pp_buf, "\n");
    return 1;
    }
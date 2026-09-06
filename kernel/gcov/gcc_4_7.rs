//! Automatically rewritten from C to Rust
//! Source: kernel/gcov/gcc_4_7.c
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
// This code provides functions to handle gcc's profiling data format
// introduced with gcc 4.7.
//
// This file is based heavily on gcc_3_4.c file.
//
// For a better understanding, refer to gcc source:
// gcc/gcov-io.h
// libgcc/libgcov.c
//
// Uses gcc-internal data definitions.
//

pub const GCOV_COUNTERS: c_int = 10;

pub const GCOV_COUNTERS: c_int = 9;

pub const GCOV_COUNTERS: c_int = 8;

pub const GCOV_COUNTERS: c_int = 9;

pub const GCOV_TAG_FUNCTION_LENGTH: c_int = 3;
// Since GCC 12.1 sizes are in BYTES and not in WORDS (4B).

pub const GCOV_UNIT_SIZE: c_int = 4;

pub const GCOV_UNIT_SIZE: c_int = 1;

pub static mut gcov_info_head: *mut c_void = core::ptr::null_mut();
//
// struct gcov_ctr_info - information about counters for a single function
// @num: number of counter values for this type
// @values: array of counter values for this type
//
// This data is generated by gcc during compilation and doesn't change
// at run-time with the exception of the values array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gcov_ctr_info {
    pub num: c_uint,
    pub values: *mut gcov_type,
}

//
// struct gcov_fn_info - profiling meta data per function
// @key: comdat key
// @ident: unique ident of function
// @lineno_checksum: function lineo_checksum
// @cfg_checksum: function cfg checksum
// @ctrs: instrumented counters
//
// This data is generated by gcc during compilation and doesn't change
// at run-time.
//
// Information about a single function.  This uses the trailing array
// idiom. The number of counters is determined from the merge pointer
// array in gcov_info.  The key is used to detect which of a set of
// comdat functions was selected -- it points to the gcov_info object
// of the object file containing the selected comdat function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gcov_fn_info {
    pub key: *const gcov_info,
    pub ident: c_uint,
    pub lineno_checksum: c_uint,
    pub cfg_checksum: c_uint,
    pub ctrs: [gcov_ctr_info; 0],
}

//
// struct gcov_info - profiling data per object file
// @version: gcov version magic indicating the gcc version used for compilation
// @next: list head for a singly-linked list
// @stamp: uniquifying time stamp
// @checksum: unique object checksum
// @filename: name of the associated gcov data file
// @merge: merge functions (null for unused counter type)
// @n_functions: number of instrumented functions
// @functions: pointer to pointers to function information
//
// This data is generated by gcc during compilation and doesn't change
// at run-time with the exception of the next pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gcov_info {
    pub version: c_uint,
    pub next: *mut gcov_info,
    pub stamp: c_uint,
// Since GCC 12.1 a checksum field is added.

    pub checksum: c_uint,

    pub filename: *const c_char,
    pub int): *mut *mut *mut c_void (merge[GCOV_COUNTERS])(gcov_type , unsigned,
    pub n_functions: c_uint,
    pub functions: *mut gcov_fn_info,
}

//
// gcov_info_filename - return info filename
// @info: profiling data set
//
    const char *gcov_info_filename(gcov_info *info)
    {
    return info.filename;
    }
//
// gcov_info_version - return info version
// @info: profiling data set
//
#[no_mangle]
pub unsafe extern "C" fn gcov_info_version(info: *mut gcov_info) -> c_uint {
    return info.version;
    }
//
// gcov_info_next - return next profiling data set
// @info: profiling data set
//
// Returns next gcov_info following @info or first gcov_info in the chain if
// @info is %NULL.
//
#[no_mangle]
pub unsafe extern "C" fn gcov_info_next(info: *mut gcov_info) -> *mut c_void {
    if (!info) {
    return gcov_info_head;
    }
    return info.next;
    }
//
// gcov_info_link - link/add profiling data set to the list
// @info: profiling data set
//
#[no_mangle]
pub unsafe extern "C" fn gcov_info_link(info: *mut gcov_info) {
    info.next = gcov_info_head;
    gcov_info_head = info;
    }
//
// gcov_info_unlink - unlink/remove profiling data set from the list
// @prev: previous profiling data set
// @info: profiling data set
//
#[no_mangle]
pub unsafe extern "C" fn gcov_info_unlink(prev: *mut gcov_info, info: *mut gcov_info) {
    if (prev) {
    prev.next = info.next;
    }
    else {
    gcov_info_head = info.next;
    }
    }
//
// gcov_info_within_module - check if a profiling data set belongs to a module
// @info: profiling data set
// @mod: module
//
// Returns true if profiling data belongs module, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn gcov_info_within_module(info: *mut gcov_info, mod: *mut module) -> bool {
    return within_module((unsigned long)info, mod);
    }
// Symbolic links to be created for each profiling data file.
pub static mut gcov_link: usize = 0;
//
// Determine whether a counter is active. Doesn't change at run-time.
//
#[no_mangle]
unsafe extern "C" fn counter_active(info: *mut gcov_info, type: c_uint) -> c_int {
    return info.merge[type] ? 1 : 0;
    }
// Determine number of active counters. Based on gcc magic.
#[no_mangle]
unsafe extern "C" fn num_counter_active(info: *mut gcov_info) -> c_uint {
    let mut i = 0;
pub static mut result: c_uint = 0;
    while (i < GCOV_COUNTERS) {
    if (counter_active(info, i)) {
    result += 1;
    }
    }
    return result;
    }
//
// gcov_info_reset - reset profiling data to zero
// @info: profiling data set
//
#[no_mangle]
pub unsafe extern "C" fn gcov_info_reset(info: *mut gcov_info) {
pub static mut ci_ptr: *mut c_void = core::ptr::null_mut();
    let mut fi_idx = 0;
    let mut ct_idx = 0;
    while (fi_idx < info.n_functions) {
    ci_ptr = info.functions[fi_idx].ctrs;
    while (ct_idx < GCOV_COUNTERS) {
    if (!counter_active(info, ct_idx)) {
    continue;
    }
    memset(ci_ptr.values, 0,
    sizeof!(gcov_type) * ci_ptr.num);
    ci_ptr += 1;
    }
    }
    }
//
// gcov_info_is_compatible - check if profiling data can be added
// @info1: first profiling data set
// @info2: second profiling data set
//
// Returns non-zero if profiling data can be added, zero otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn gcov_info_is_compatible(info1: *mut gcov_info, info2: *mut gcov_info) -> c_int {
    return (info1.stamp == info2.stamp);
    }
//
// gcov_info_add - add up profiling data
// @dst: profiling data set to which data is added
// @src: profiling data set which is added
//
// Adds profiling counts of @src to @dst.
//
#[no_mangle]
pub unsafe extern "C" fn gcov_info_add(dst: *mut gcov_info, src: *mut gcov_info) {
pub static mut dci_ptr: *mut c_void = core::ptr::null_mut();
pub static mut sci_ptr: *mut c_void = core::ptr::null_mut();
    let mut fi_idx = 0;
    let mut ct_idx = 0;
    let mut val_idx = 0;
    while (fi_idx < src.n_functions) {
    dci_ptr = dst.functions[fi_idx].ctrs;
    sci_ptr = src.functions[fi_idx].ctrs;
    while (ct_idx < GCOV_COUNTERS) {
    if (!counter_active(src, ct_idx)) {
    continue;
    }
    for (val_idx = 0; val_idx < sci_ptr.num; val_idx++) {
    dci_ptr.values[val_idx] +=
    sci_ptr.values[val_idx];
    }
    dci_ptr += 1;
    sci_ptr += 1;
    }
    }
    }
//
// gcov_info_dup - duplicate profiling data set
// @info: profiling data set to duplicate
//
// Return newly allocated duplicate on success, %NULL on error.
//
#[no_mangle]
pub unsafe extern "C" fn gcov_info_dup(info: *mut gcov_info) -> *mut c_void {
pub static mut dup: *mut c_void = core::ptr::null_mut();
pub static mut dci_ptr: *mut c_void = core::ptr::null_mut(); /* dst counter info */
pub static mut sci_ptr: *mut c_void = core::ptr::null_mut(); /* src counter info */
    let mut active = 0;
    let mut fi_idx = 0; /* function info idx */
    let mut ct_idx = 0; /* counter type idx */
    let mut fi_size = 0; /* function info size */
    let mut cv_size = 0; /* counter values size */
    dup = kmemdup(info, sizeof!(*dup), GFP_KERNEL);
    if (!dup) {
    return core::ptr::null_mut();
    }
    dup.next = core::ptr::null_mut();
    dup.filename = core::ptr::null_mut();
    dup.functions = core::ptr::null_mut();
    dup.filename = kstrdup(info.filename, GFP_KERNEL);
    if (!dup.filename) {
// goto;
    }
    dup.functions = kzalloc_objs(gcov_fn_info *, info.n_functions);
    if (!dup.functions) {
// goto;
    }
    active = num_counter_active(info);
    fi_size = sizeof!(gcov_fn_info);
    fi_size += sizeof!(gcov_ctr_info) * active;
    while (fi_idx < info.n_functions) {
    dup.functions[fi_idx] = kzalloc(fi_size, GFP_KERNEL);
    if (!dup.functions[fi_idx]) {
// goto;
    }
// (dup->functions[fi_idx]) = *(info->functions[fi_idx]);
    sci_ptr = info.functions[fi_idx].ctrs;
    dci_ptr = dup.functions[fi_idx].ctrs;
    while (ct_idx < active) {
    cv_size = sizeof!(gcov_type) * sci_ptr.num;
    dci_ptr.values = kvmalloc(cv_size, GFP_KERNEL);
    if (!dci_ptr.values) {
// goto;
    }
    dci_ptr.num = sci_ptr.num;
    memcpy(dci_ptr.values, sci_ptr.values, cv_size);
    sci_ptr += 1;
    dci_ptr += 1;
    }
    }
    return dup;
// label;
    gcov_info_free(dup);
    return core::ptr::null_mut();
    }
//
// gcov_info_free - release memory for profiling data set duplicate
// @info: profiling data set duplicate to free
//
#[no_mangle]
pub unsafe extern "C" fn gcov_info_free(info: *mut gcov_info) {
    let mut active = 0;
    let mut fi_idx = 0;
    let mut ct_idx = 0;
pub static mut ci_ptr: *mut c_void = core::ptr::null_mut();
    if (!info.functions) {
// goto;
    }
    active = num_counter_active(info);
    while (fi_idx < info.n_functions) {
    if (!info.functions[fi_idx]) {
    continue;
    }
    ci_ptr = info.functions[fi_idx].ctrs;
    for (ct_idx = 0; ct_idx < active; ct_idx++, ci_ptr++) {
    kvfree(ci_ptr.values);
    }
    kfree(info.functions[fi_idx]);
    }
// label;
    kfree(info.functions);
    kfree(info.filename);
    kfree(info);
    }
//
// convert_to_gcda - convert profiling data set to gcda file format
// @buffer: the buffer to store file data or %NULL if no data should be stored
// @info: profiling data set to be converted
//
// Returns the number of bytes that were/would have been stored into the buffer.
//
#[no_mangle]
pub unsafe extern "C" fn convert_to_gcda(buffer: *mut c_char, info: *mut gcov_info) -> usize {
pub static mut fi_ptr: *mut c_void = core::ptr::null_mut();
pub static mut ci_ptr: *mut c_void = core::ptr::null_mut();
    let mut fi_idx = 0;
    let mut ct_idx = 0;
    let mut cv_idx = 0;
pub static mut pos: usize = 0;
// File header.
    pos += store_gcov_u32(buffer, pos, GCOV_DATA_MAGIC);
    pos += store_gcov_u32(buffer, pos, info.version);
    pos += store_gcov_u32(buffer, pos, info.stamp);

// Use zero as checksum of the compilation unit.
    pos += store_gcov_u32(buffer, pos, 0);

    while (fi_idx < info.n_functions) {
    fi_ptr = info.functions[fi_idx];
// Function record.
    pos += store_gcov_u32(buffer, pos, GCOV_TAG_FUNCTION);
    pos += store_gcov_u32(buffer, pos,
    GCOV_TAG_FUNCTION_LENGTH * GCOV_UNIT_SIZE);
    pos += store_gcov_u32(buffer, pos, fi_ptr.ident);
    pos += store_gcov_u32(buffer, pos, fi_ptr.lineno_checksum);
    pos += store_gcov_u32(buffer, pos, fi_ptr.cfg_checksum);
    ci_ptr = fi_ptr.ctrs;
    while (ct_idx < GCOV_COUNTERS) {
    if (!counter_active(info, ct_idx)) {
    continue;
    }
// Counter record.
    pos += store_gcov_u32(buffer, pos,
    GCOV_TAG_FOR_COUNTER(ct_idx));
    pos += store_gcov_u32(buffer, pos,
    ci_ptr.num * 2 * GCOV_UNIT_SIZE);
    while (cv_idx < ci_ptr.num) {
    pos += store_gcov_u64(buffer, pos,
    ci_ptr.values[cv_idx]);
    }
    ci_ptr += 1;
    }
    }
    return pos;
    }
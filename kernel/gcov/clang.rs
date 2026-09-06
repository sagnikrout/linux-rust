//! Automatically rewritten from C to Rust
//! Source: kernel/gcov/clang.c
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
// Copyright (C) 2019 Google, Inc.
// modified from kernel/gcov/gcc_4_7.c
//
// This software is licensed under the terms of the GNU General Public
// License version 2, as published by the Free Software Foundation, and
// may be copied, distributed, and modified under those terms.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// LLVM uses profiling data that's deliberately similar to GCC, but has a
// very different way of exporting that data.  LLVM calls llvm_gcov_init() once
// per module, and provides a couple of callbacks that we can use to ask for
// more data.
//
// We care about the "writeout" callback, which in turn calls back into
// compiler-rt/this module to dump all the gathered coverage data to disk:
//
// llvm_gcda_start_file()
// llvm_gcda_emit_function()
// llvm_gcda_emit_arcs()
// llvm_gcda_emit_function()
// llvm_gcda_emit_arcs()
// [... repeats for each function ...]
// llvm_gcda_summary_info()
// llvm_gcda_end_file()
//
// This design is much more stateless and unstructured than gcc's, and is
// intended to run at process exit.  This forces us to keep some local state
// about which module we're dealing with at the moment.  On the other hand, it
// also means we don't depend as much on how LLVM represents profiling data
// internally.
//
// See LLVM's lib/Transforms/Instrumentation/GCOVProfiling.cpp for more
// details on how this works, particularly GCOVProfiler::emitProfileArcs(),
// GCOVProfiler::insertCounterWriteout(), and
// GCOVProfiler::insertFlush().
//

    typedef void (*llvm_gcov_callback)(void);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gcov_info {
    pub head: list_head,
    pub filename: *const c_char,
    pub version: c_uint,
    pub checksum: u32,
    pub functions: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gcov_fn_info {
    pub head: list_head,
    pub ident: u32,
    pub checksum: u32,
    pub cfg_checksum: u32,
    pub num_counters: u32,
    pub counters: *mut u64,
}

pub static mut current_info: *mut c_void = core::ptr::null_mut();
pub static mut clang_gcov_list: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn llvm_gcov_init(writeout: llvm_gcov_callback, flush: llvm_gcov_callback) {
    let mut info = kzalloc_obj(*info);
    if (!info) {
    return;
    }
    INIT_LIST_HEAD(&info.head);
    INIT_LIST_HEAD(&info.functions);
    mutex_lock(&gcov_lock);
    list_add_tail(&info.head, &clang_gcov_list);
    current_info = info;
    writeout();
    current_info = core::ptr::null_mut();
    if (gcov_events_enabled) {
    gcov_event(GCOV_ADD, info);
    }
    mutex_unlock(&gcov_lock);
    }
    EXPORT_SYMBOL(llvm_gcov_init);
#[no_mangle]
pub unsafe extern "C" fn llvm_gcda_start_file(orig_filename: *const c_char, version: u32, checksum: u32) {
    current_info.filename = orig_filename;
    current_info.version = version;
    current_info.checksum = checksum;
    }
    EXPORT_SYMBOL(llvm_gcda_start_file);
#[no_mangle]
pub unsafe extern "C" fn llvm_gcda_emit_function(ident: u32, func_checksum: u32, cfg_checksum: u32) {
    let mut info = kzalloc_obj(*info);
    if (!info) {
    return;
    }
    INIT_LIST_HEAD(&info.head);
    info.ident = ident;
    info.checksum = func_checksum;
    info.cfg_checksum = cfg_checksum;
    list_add_tail(&info.head, &current_info.functions);
    }
    EXPORT_SYMBOL(llvm_gcda_emit_function);
#[no_mangle]
pub unsafe extern "C" fn llvm_gcda_emit_arcs(num_counters: u32, counters: *mut u64) {
    let mut info = list_last_entry(&current_info.functions, gcov_fn_info, head);
    info.num_counters = num_counters;
    info.counters = counters;
    }
    EXPORT_SYMBOL(llvm_gcda_emit_arcs);
#[no_mangle]
pub unsafe extern "C" fn llvm_gcda_summary_info() {
    }
    EXPORT_SYMBOL(llvm_gcda_summary_info);
#[no_mangle]
pub unsafe extern "C" fn llvm_gcda_end_file() {
    }
    EXPORT_SYMBOL(llvm_gcda_end_file);
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
    return list_first_entry_or_null(&clang_gcov_list, gcov_info, head);
    }
    if (list_is_last(&info.head, &clang_gcov_list)) {
    return core::ptr::null_mut();
    }
    return list_next_entry(info, head);
    }
//
// gcov_info_link - link/add profiling data set to the list
// @info: profiling data set
//
#[no_mangle]
pub unsafe extern "C" fn gcov_info_link(info: *mut gcov_info) {
    list_add_tail(&info.head, &clang_gcov_list);
    }
//
// gcov_info_unlink - unlink/remove profiling data set from the list
// @prev: previous profiling data set
// @info: profiling data set
//
#[no_mangle]
pub unsafe extern "C" fn gcov_info_unlink(prev: *mut gcov_info, info: *mut gcov_info) {
// Generic code unlinks while iterating.
    __list_del_entry(&info.head);
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
    return within_module((unsigned long)info.filename, mod);
    }
// Symbolic links to be created for each profiling data file.
pub static mut gcov_link: usize = 0;
//
// gcov_info_reset - reset profiling data to zero
// @info: profiling data set
//
#[no_mangle]
pub unsafe extern "C" fn gcov_info_reset(info: *mut gcov_info) {
pub static mut fn: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(fn, &info.functions, head) {
    memset(fn.counters, 0,
    sizeof!(fn.counters[0]) * fn.num_counters);
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
    let mut fn_ptr1 = list_first_entry_or_null(
    &info1.functions, gcov_fn_info, head);
    let mut fn_ptr2 = list_first_entry_or_null(
    &info2.functions, gcov_fn_info, head);
    if (info1.checksum != info2.checksum) {
    return false;
    }
    if (!fn_ptr1) {
pub static mut fn_ptr1: return = 0;
    }
    while (!list_is_last(&fn_ptr1.head, &info1.functions) &&
    !list_is_last(&fn_ptr2.head, &info2.functions)) {
    if (fn_ptr1.checksum != fn_ptr2.checksum) {
    return false;
    }
    if (fn_ptr1.cfg_checksum != fn_ptr2.cfg_checksum) {
    return false;
    }
    fn_ptr1 = list_next_entry(fn_ptr1, head);
    fn_ptr2 = list_next_entry(fn_ptr2, head);
    }
    return list_is_last(&fn_ptr1.head, &info1.functions) &&
    list_is_last(&fn_ptr2.head, &info2.functions);
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
pub static mut dfn_ptr: *mut c_void = core::ptr::null_mut();
    let mut sfn_ptr = list_first_entry_or_null(&src.functions, gcov_fn_info, head);
    list_for_each_entry(dfn_ptr, &dst.functions, head) {
    let mut i = 0;
    for (i = 0; i < sfn_ptr.num_counters; i++) {
    dfn_ptr.counters[i] += sfn_ptr.counters[i];
    }
    sfn_ptr = list_next_entry(sfn_ptr, head);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn gcov_fn_info_dup(fn: *mut gcov_fn_info) -> *mut c_void {
    let mut cv_size = 0; /* counter values size */
    let mut fn_dup = kmemdup(fn, sizeof!(*fn),
    GFP_KERNEL);
    if (!fn_dup) {
    return core::ptr::null_mut();
    }
    INIT_LIST_HEAD(&fn_dup.head);
    cv_size = fn.num_counters * sizeof!(fn.counters[0]);
    fn_dup.counters = kvmalloc(cv_size, GFP_KERNEL);
    if (!fn_dup.counters) {
    kfree(fn_dup);
    return core::ptr::null_mut();
    }
    memcpy(fn_dup.counters, fn.counters, cv_size);
    return fn_dup;
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
pub static mut fn: *mut c_void = core::ptr::null_mut();
    dup = kmemdup(info, sizeof!(*dup), GFP_KERNEL);
    if (!dup) {
    return core::ptr::null_mut();
    }
    INIT_LIST_HEAD(&dup.head);
    INIT_LIST_HEAD(&dup.functions);
    dup.filename = kstrdup(info.filename, GFP_KERNEL);
    if (!dup.filename) {
// goto;
    }
    list_for_each_entry(fn, &info.functions, head) {
    let mut fn_dup = gcov_fn_info_dup(fn);
    if (!fn_dup) {
// goto;
    }
    list_add_tail(&fn_dup.head, &dup.functions);
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
    let mut fn = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    list_for_each_entry_safe(fn, tmp, &info.functions, head) {
    kvfree(fn.counters);
    list_del(&fn.head);
    kfree(fn);
    }
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
pub static mut pos: usize = 0;
// File header.
    pos += store_gcov_u32(buffer, pos, GCOV_DATA_MAGIC);
    pos += store_gcov_u32(buffer, pos, info.version);
    pos += store_gcov_u32(buffer, pos, info.checksum);
    list_for_each_entry(fi_ptr, &info.functions, head) {
    let mut i = 0;
    pos += store_gcov_u32(buffer, pos, GCOV_TAG_FUNCTION);
    pos += store_gcov_u32(buffer, pos, 3);
    pos += store_gcov_u32(buffer, pos, fi_ptr.ident);
    pos += store_gcov_u32(buffer, pos, fi_ptr.checksum);
    pos += store_gcov_u32(buffer, pos, fi_ptr.cfg_checksum);
    pos += store_gcov_u32(buffer, pos, GCOV_TAG_COUNTER_BASE);
    pos += store_gcov_u32(buffer, pos, fi_ptr.num_counters * 2);
    for (i = 0; i < fi_ptr.num_counters; i++) {
    pos += store_gcov_u64(buffer, pos, fi_ptr.counters[i]);
    }
    }
    return pos;
    }
//! Automatically rewritten from C to Rust
//! Source: mm/kmsan/report.c
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
// KMSAN error reporting routines.
//
// Copyright (C) 2019-2022 Google LLC
// Author: Alexander Potapenko <glider@google.com>
//

pub static mut kmsan_report_lock: usize = 0;
pub const DESCR_SIZE: c_int = 128;
// Protected by kmsan_report_lock
    static char report_local_descr[DESCR_SIZE];
    let mut panic_on_kmsan = 0;
    EXPORT_SYMBOL_GPL(panic_on_kmsan);

    module_param_named!(panic, panic_on_kmsan, int, 0);
//
// Skip internal KMSAN frames.
//
#[no_mangle]
pub unsafe extern "C" fn get_stack_skipnr(num_entries: c_int) -> c_int {
    let mut len = 0;
    let mut skip = 0;
    char buf[64];
    while (skip < num_entries) {
    len = scnprintf(buf, sizeof!(buf), "%ps",
    stack_entries[skip]);
// Never show __msan_* or kmsan_* functions.
    if ((strnstr(buf, "__msan_", len) == buf) ||
    (strnstr(buf, "kmsan_", len) == buf)) {
    continue;
    }
//
// No match for runtime functions -- @skip entries to skip to
// get to first frame of interest.
//
    break;
    }
    return skip;
    }
//
// Currently the descriptions of locals generated by Clang look as follows:
// ----local_name@function_name
// We want to print only the name of the local, as other information in that
// description can be confusing.
// The meaningful part of the description is copied to a global buffer to avoid
// allocating memory.
//
#[no_mangle]
pub unsafe extern "C" fn pretty_descr(descr: *mut c_char) -> *mut c_void {
pub static mut pos: c_int = 0;
    while (i < len) {
    if (descr[i] == '@') {
    break;
    }
    if (descr[i] == '-') {
    continue;
    }
    report_local_descr[pos] = descr[i];
    if (pos + 1 == DESCR_SIZE) {
    break;
    }
    pos += 1;
    }
    report_local_descr[pos] = 0;
    return report_local_descr;
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_print_origin(origin: depot_stack_handle_t) {
    let mut entries = core::ptr::null_mut(), *chained_entries = core::ptr::null_mut();
    let mut nr_entries = 0;
    let mut chained_nr_entries = 0;
    let mut skipnr = 0;
    let mut pc1 = core::ptr::null_mut(), *pc2 = core::ptr::null_mut();
    let mut head;
    let mut magic = 0;
    let mut descr = core::ptr::null_mut();
    let mut depth = 0;
    if (!origin) {
    return;
    }
    while (true) {
    nr_entries = stack_depot_fetch(origin, &entries);
    depth = kmsan_depth_from_eb(stack_depot_get_extra_bits(origin));
    magic = nr_entries ? entries[0] : 0;
    if ((nr_entries == 4) && (magic == KMSAN_ALLOCA_MAGIC_ORIGIN)) {
    descr = entries[1];
    pc1 = entries[2];
    pc2 = entries[3];
    pr_err!("Local variable %s created at:\n",
    pretty_descr(descr));
    if (pc1) {
    pr_err!(" %pSb\n", pc1);
    }
    if (pc2) {
    pr_err!(" %pSb\n", pc2);
    }
    break;
    }
    if ((nr_entries == 3) && (magic == KMSAN_CHAIN_MAGIC_ORIGIN)) {
//
// Origin chains deeper than KMSAN_MAX_ORIGIN_DEPTH are
// not stored, so the output may be incomplete.
//
    if (depth == KMSAN_MAX_ORIGIN_DEPTH) {
    pr_err!("<Zero or more stacks not recorded to save memory>\n\n");
    }
    head = entries[1];
    origin = entries[2];
    pr_err!("Uninit was stored to memory at:\n");
    chained_nr_entries =
    stack_depot_fetch(head, &chained_entries);
    kmsan_internal_unpoison_memory(
    chained_entries,
    chained_nr_entries * sizeof!(*chained_entries),
// checked*/ false);
    skipnr = get_stack_skipnr(chained_entries,
    chained_nr_entries);
    stack_trace_print(chained_entries + skipnr,
    chained_nr_entries - skipnr, 0);
    pr_err!("\n");
    continue;
    }
    pr_err!("Uninit was created at:\n");
    if (nr_entries) {
    skipnr = get_stack_skipnr(entries, nr_entries);
    stack_trace_print(entries + skipnr, nr_entries - skipnr,
    0);
    } else {
    pr_err!("(stack is not available)\n");
    }
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_report(origin: depot_stack_handle_t, address: *mut c_void, size: c_int, off_first: c_int, off_last: c_int, user_addr: *mut c_void, reason: kmsan_bug_reason) {
    unsigned long stack_entries[KMSAN_STACK_DEPTH];
    let mut num_stack_entries = 0;
    let mut skipnr = 0;
    let mut bug_type = core::ptr::null_mut();
    let mut ua_flags = 0;
    let mut is_uaf = 0;
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return;
    }
    if (current.kmsan_ctx.depth) {
    return;
    }
    if (!origin) {
    return;
    }
    kmsan_enter_runtime();
    ua_flags = user_access_save();
    raw_spin_lock(&kmsan_report_lock);
    pr_err!("=====================================================\n");
    is_uaf = kmsan_uaf_from_eb(stack_depot_get_extra_bits(origin));
    match (reason) {
    REASON_ANY => {
    bug_type = is_uaf ? "use-after-free" : "uninit-value";
    // break;
    }
    REASON_COPY_TO_USER => {
    bug_type = is_uaf ? "kernel-infoleak-after-free" :
    "kernel-infoleak";
    // break;
    }
    REASON_SUBMIT_URB => {
    bug_type = is_uaf ? "kernel-usb-infoleak-after-free" :
    "kernel-usb-infoleak";
    // break;
    }
    }
    num_stack_entries =
    stack_trace_save(stack_entries, KMSAN_STACK_DEPTH, 1);
    skipnr = get_stack_skipnr(stack_entries, num_stack_entries);
    pr_err!("BUG: KMSAN: %s in %pSb\n", bug_type,
    stack_entries[skipnr]);
    stack_trace_print(stack_entries + skipnr, num_stack_entries - skipnr,
    0);
    pr_err!("\n");
    kmsan_print_origin(origin);
    if (size) {
    pr_err!("\n");
    if (off_first == off_last) {
    pr_err!("Byte %d of %d is uninitialized\n", off_first,
    size);
    }
    else {
    pr_err!("Bytes %d-%d of %d are uninitialized\n",
    off_first, off_last, size);
    }
    }
    if (address) {
    pr_err!("Memory access of size %d starts at %px\n", size,
    address);
    }
    if (user_addr && reason == REASON_COPY_TO_USER) {
    pr_err!("Data copied to user address %px\n", user_addr);
    }
    pr_err!("\n");
    dump_stack_print_info(KERN_ERR);
    pr_err!("=====================================================\n");
    add_taint(TAINT_BAD_PAGE, LOCKDEP_NOW_UNRELIABLE);
    raw_spin_unlock(&kmsan_report_lock);
    if (panic_on_kmsan) {
    panic("kmsan.panic set ...\n");
    }
    user_access_restore(ua_flags);
    kmsan_leave_runtime();
    }
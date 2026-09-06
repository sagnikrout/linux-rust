//! Automatically rewritten from C to Rust
//! Source: kernel/power/wakelock.c
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
// kernel/power/wakelock.c
//
// User space wakeup sources support.
//
// Copyright (C) 2012 Rafael J. Wysocki <rjw@sisk.pl>
//
// This code is based on the analogous interface allowing user space to
// manipulate wakelocks on Android.
//

pub static mut wakelocks_lock: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wakelock {
    pub name: *mut c_char,
    pub node: rb_node,
    pub ws: *mut wakeup_source,

    pub lru: list_head,

}

pub static mut wakelocks_tree: rb_root = 0;
#[no_mangle]
pub unsafe extern "C" fn pm_show_wakelocks(buf: *mut c_char, show_active: bool) -> isize {
pub static mut node: *mut c_void = core::ptr::null_mut();
pub static mut wl: *mut c_void = core::ptr::null_mut();
pub static mut len: c_int = 0;
    mutex_lock(&wakelocks_lock);
    for (node = rb_first(&wakelocks_tree); node; node = rb_next(node)) {
    wl = rb_entry(node, wakelock, node);
    if (wl.ws.active == show_active) {
    len += sysfs_emit_at(buf, len, "%s ", wl.name);
    }
    }
    if (len > 0) {
    len -= 1;
    }
    len += sysfs_emit_at(buf, len, "\n");
    mutex_unlock(&wakelocks_lock);
    return len;
    }

    static unsigned int number_of_wakelocks;
#[no_mangle]
pub unsafe extern "C" fn wakelocks_limit_exceeded() -> bool {
    return number_of_wakelocks >= CONFIG_PM_WAKELOCKS_LIMIT;
    }
#[no_mangle]
pub unsafe extern "C" fn increment_wakelocks_number() {
    number_of_wakelocks += 1;
    }
#[no_mangle]
pub unsafe extern "C" fn decrement_wakelocks_number() {
    number_of_wakelocks -= 1;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: wakelocks_limit_exceeded
pub unsafe extern "C" fn wakelocks_limit_exceeded_dup() -> bool { return false; }
#[no_mangle]
#[no_mangle]
// duplicate fn: increment_wakelocks_number
pub unsafe extern "C" fn increment_wakelocks_number_dup() {}
#[no_mangle]
#[no_mangle]
// duplicate fn: decrement_wakelocks_number
pub unsafe extern "C" fn decrement_wakelocks_number_dup() {}

pub const WL_GC_COUNT_MAX: c_int = 100;
pub const WL_GC_TIME_SEC: c_int = 300;
// forward_decl: __wakelocks_gc;
pub static mut wakelocks_lru_list: usize = 0;
pub static mut wakelock_work: usize = 0;
    static unsigned int wakelocks_gc_count;
#[no_mangle]
pub unsafe extern "C" fn wakelocks_lru_add(wl: *mut wakelock) {
    list_add(&wl.lru, &wakelocks_lru_list);
    }
#[no_mangle]
pub unsafe extern "C" fn wakelocks_lru_most_recent(wl: *mut wakelock) {
    list_move(&wl.lru, &wakelocks_lru_list);
    }
#[no_mangle]
unsafe extern "C" fn __wakelocks_gc(work: *mut work_struct) {
    let mut wl = core::ptr::null_mut();
    let mut aux = core::ptr::null_mut();
    let mut now;
    mutex_lock(&wakelocks_lock);
    now = ktime_get();
    list_for_each_entry_safe_reverse(wl, aux, &wakelocks_lru_list, lru) {
    let mut idle_time_ns = 0;
    let mut active = 0;
    spin_lock_irq(&wl.ws.lock);
    idle_time_ns = ktime_to_ns(ktime_sub(now, wl.ws.last_time));
    active = wl.ws.active;
    spin_unlock_irq(&wl.ws.lock);
    if (idle_time_ns < ((u64)WL_GC_TIME_SEC * NSEC_PER_SEC)) {
    break;
    }
    if (!active) {
    wakeup_source_unregister(wl.ws);
    rb_erase(&wl.node, &wakelocks_tree);
    list_del(&wl.lru);
    kfree(wl.name);
    kfree(wl);
    decrement_wakelocks_number();
    }
    }
    wakelocks_gc_count = 0;
    mutex_unlock(&wakelocks_lock);
    }
#[no_mangle]
unsafe extern "C" fn wakelocks_gc() {
    if (++wakelocks_gc_count <= WL_GC_COUNT_MAX) {
    return;
    }
    schedule_work(&wakelock_work);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: wakelocks_lru_add
pub unsafe extern "C" fn wakelocks_lru_add_dup(wl: *mut wakelock) {}
#[no_mangle]
#[no_mangle]
// duplicate fn: wakelocks_lru_most_recent
pub unsafe extern "C" fn wakelocks_lru_most_recent_dup(wl: *mut wakelock) {}
#[no_mangle]
pub unsafe extern "C" fn wakelocks_gc() {}

#[no_mangle]
pub unsafe extern "C" fn wakelock_lookup_add(name: *mut c_char, len: size_t, add_if_not_found: bool) -> *mut c_void {
    let mut node = &wakelocks_tree.rb_node;
    let mut parent = *node;
pub static mut wl: *mut c_void = core::ptr::null_mut();
    while (*node) {
    let mut diff = 0;
    parent = *node;
    wl = rb_entry(*node, wakelock, node);
    diff = strncmp(name, wl.name, len);
    if (diff == 0) {
    if (wl.name[len]) {
    diff = -1;
    }
    else {
    return wl;
    }
    }
    if (diff < 0) {
    node = &(*node).rb_left;
    }
    else {
    node = &(*node).rb_right;
    }
    }
    if (!add_if_not_found) {
    return ERR_PTR(-EINVAL);
    }
    if (wakelocks_limit_exceeded()) {
    return ERR_PTR(-ENOSPC);
    }
// Not found, we have to add a new one.
    wl = kzalloc_obj(*wl);
    if (!wl) {
    return ERR_PTR(-ENOMEM);
    }
    wl.name = kstrndup(name, len, GFP_KERNEL);
    if (!wl.name) {
    kfree(wl);
    return ERR_PTR(-ENOMEM);
    }
    wl.ws = wakeup_source_register(core::ptr::null_mut(), wl.name);
    if (!wl.ws) {
    kfree(wl.name);
    kfree(wl);
    return ERR_PTR(-ENOMEM);
    }
    wl.ws.last_time = ktime_get();
    rb_link_node(&wl.node, parent, node);
    rb_insert_color(&wl.node, &wakelocks_tree);
    wakelocks_lru_add(wl);
    increment_wakelocks_number();
    return wl;
    }
#[no_mangle]
pub unsafe extern "C" fn pm_wake_lock(buf: *const c_char) -> c_int {
    let mut str = buf;
pub static mut wl: *mut c_void = core::ptr::null_mut();
pub static mut timeout_ns: u64 = 0;
    let mut len = 0;
pub static mut ret: c_int = 0;
    if (!capable(CAP_BLOCK_SUSPEND)) {
    return -EPERM;
    }
    while (*str && !isspace(*str)) {
    str += 1;
    }
    len = str - buf;
    if (!len) {
    return -EINVAL;
    }
    if (*str && *str != '\n') {
// Find out if there's a valid timeout string appended.
    ret = kstrtou64(skip_spaces(str), 10, &timeout_ns);
    if (ret) {
    return -EINVAL;
    }
    }
    mutex_lock(&wakelocks_lock);
    wl = wakelock_lookup_add(buf, len, true);
    if (IS_ERR(wl)) {
    ret = PTR_ERR(wl);
// goto;
    }
    if (timeout_ns) {
pub static mut timeout_ms: u64 = 0;
    do_div(timeout_ms, NSEC_PER_MSEC);
    __pm_wakeup_event(wl.ws, timeout_ms);
    } else {
    __pm_stay_awake(wl.ws);
    }
    wakelocks_lru_most_recent(wl);
// label;
    mutex_unlock(&wakelocks_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn pm_wake_unlock(buf: *const c_char) -> c_int {
pub static mut wl: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
pub static mut ret: c_int = 0;
    if (!capable(CAP_BLOCK_SUSPEND)) {
    return -EPERM;
    }
    len = strlen(buf);
    if (!len) {
    return -EINVAL;
    }
    if (buf[len-1] == '\n') {
    len -= 1;
    }
    if (!len) {
    return -EINVAL;
    }
    mutex_lock(&wakelocks_lock);
    wl = wakelock_lookup_add(buf, len, false);
    if (IS_ERR(wl)) {
    ret = PTR_ERR(wl);
// goto;
    }
    __pm_relax(wl.ws);
    wakelocks_lru_most_recent(wl);
    wakelocks_gc();
// label;
    mutex_unlock(&wakelocks_lock);
    return ret;
    }
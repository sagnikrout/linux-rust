//! Automatically rewritten from C to Rust
//! Source: kernel/cgroup/misc.c
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
// Miscellaneous cgroup controller
//
// Copyright 2020 Google LLC
// Author: Vipin Sharma <vipinsh@google.com>
//

// Miscellaneous res name, keep it in sync with enum misc_res_type
    static const char *const misc_res_name[] = {

// AMD SEV ASIDs resource
    "sev",
// AMD SEV-ES ASIDs resource
    "sev_es",

// Intel TDX HKIDs resource
    "tdx",

    };
// Root misc cgroup
pub static mut root_cg: usize = 0;
//
// Miscellaneous resources capacity for the entire machine. 0 capacity means
// resource is not initialized or not present in the host.
//
// root_cg.max and capacity are independent of each other. root_cg.max can be
// more than the actual capacity. We are using Limits resource distribution
// model of cgroup for miscellaneous controller.
//
    static u64 misc_res_capacity[MISC_CG_RES_TYPES];
//
// parent_misc() - Get the parent of the passed misc cgroup.
// @cgroup: cgroup whose parent needs to be fetched.
//
// Context: Any context.
// Return:
// * struct misc_cg* - Parent of the @cgroup.
// * %NULL - If @cgroup is null or the passed cgroup does not have a parent.
//
#[no_mangle]
pub unsafe extern "C" fn parent_misc(cgroup: *mut misc_cg) -> *mut c_void {
    return cgroup ? css_misc(cgroup.css.parent) : core::ptr::null_mut();
    }
//
// valid_type() - Check if @type is valid or not.
// @type: misc res type.
//
// Context: Any context.
// Return:
// * true - If valid type.
// * false - If not valid type.
//
#[no_mangle]
pub unsafe extern "C" fn valid_type(type: misc_res_type) -> bool {
    return type >= 0 && type < MISC_CG_RES_TYPES;
    }
//
// misc_cg_set_capacity() - Set the capacity of the misc cgroup res.
// @type: Type of the misc res.
// @capacity: Supported capacity of the misc res on the host.
//
// If capacity is 0 then the charging a misc cgroup fails for that type.
//
// Context: Any context.
// Return:
// * %0 - Successfully registered the capacity.
// * %-EINVAL - If @type is invalid.
//
#[no_mangle]
pub unsafe extern "C" fn misc_cg_set_capacity(type: misc_res_type, capacity: u64) -> c_int {
    if (!valid_type(type)) {
    return -EINVAL;
    }
    WRITE_ONCE(misc_res_capacity[type], capacity);
    return 0;
    }
    EXPORT_SYMBOL_GPL(misc_cg_set_capacity);
//
// misc_cg_cancel_charge() - Cancel the charge from the misc cgroup.
// @type: Misc res type in misc cg to cancel the charge from.
// @cg: Misc cgroup to cancel charge from.
// @amount: Amount to cancel.
//
// Context: Any context.
//
#[no_mangle]
pub unsafe extern "C" fn misc_cg_cancel_charge(type: misc_res_type, cg: *mut misc_cg, amount: u64) {
    WARN_ONCE(atomic64_add_negative(-amount, &cg.res[type].usage),
    "misc cgroup resource %s became less than 0",
    misc_res_name[type]);
    }
#[no_mangle]
unsafe extern "C" fn misc_cg_update_watermark(res: *mut misc_res, new_usage: u64) {
    let mut old = 0;
    while (true) {
    old = atomic64_read(&res.watermark);
    if (new_usage <= old) {
    break;
    }
    if (atomic64_cmpxchg(&res.watermark, old, new_usage) == old) {
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn misc_cg_event(type: misc_res_type, cg: *mut misc_cg) {
    atomic64_inc(&cg.res[type].events_local);
    cgroup_file_notify(&cg.events_local_file);
    for (; parent_misc(cg); cg = parent_misc(cg)) {
    atomic64_inc(&cg.res[type].events);
    cgroup_file_notify(&cg.events_file);
    }
    }
//
// misc_cg_try_charge() - Try charging the misc cgroup.
// @type: Misc res type to charge.
// @cg: Misc cgroup which will be charged.
// @amount: Amount to charge.
//
// Charge @amount to the misc cgroup. Caller must use the same cgroup during
// the uncharge call.
//
// Context: Any context.
// Return:
// * %0 - If successfully charged.
// * -EINVAL - If @type is invalid or misc res has 0 capacity.
// * -EBUSY - If max limit will be crossed or total usage will be more than the
// capacity.
//
#[no_mangle]
pub unsafe extern "C" fn misc_cg_try_charge(type: misc_res_type, cg: *mut misc_cg, amount: u64) -> c_int {
    let mut i = core::ptr::null_mut();
    let mut j = core::ptr::null_mut();
    let mut ret = 0;
pub static mut res: *mut c_void = core::ptr::null_mut();
    let mut new_usage = 0;
    if (!(valid_type(type) && cg && READ_ONCE(misc_res_capacity[type]))) {
    return -EINVAL;
    }
    if (!amount) {
    return 0;
    }
    for (i = cg; i; i = parent_misc(i)) {
    res = &i.res[type];
    new_usage = atomic64_add_return(amount, &res.usage);
    if (new_usage > READ_ONCE(res.max) ||
    new_usage > READ_ONCE(misc_res_capacity[type])) {
    ret = -EBUSY;
// goto;
    }
    misc_cg_update_watermark(res, new_usage);
    }
    return 0;
// label;
    misc_cg_event(type, i);
    for (j = cg; j != i; j = parent_misc(j)) {
    misc_cg_cancel_charge(type, j, amount);
    }
    misc_cg_cancel_charge(type, i, amount);
    return ret;
    }
    EXPORT_SYMBOL_GPL(misc_cg_try_charge);
//
// misc_cg_uncharge() - Uncharge the misc cgroup.
// @type: Misc res type which was charged.
// @cg: Misc cgroup which will be uncharged.
// @amount: Charged amount.
//
// Context: Any context.
//
#[no_mangle]
pub unsafe extern "C" fn misc_cg_uncharge(type: misc_res_type, cg: *mut misc_cg, amount: u64) {
pub static mut i: *mut c_void = core::ptr::null_mut();
    if (!(amount && valid_type(type) && cg)) {
    return;
    }
    for (i = cg; i; i = parent_misc(i)) {
    misc_cg_cancel_charge(type, i, amount);
    }
    }
    EXPORT_SYMBOL_GPL(misc_cg_uncharge);
//
// misc_cg_max_show() - Show the misc cgroup max limit.
// @sf: Interface file
// @v: Arguments passed
//
// Context: Any context.
// Return: 0 to denote successful print.
//
#[no_mangle]
unsafe extern "C" fn misc_cg_max_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    let mut i = 0;
    let mut cg = css_misc(seq_css(sf));
    let mut max = 0;
    while (i < MISC_CG_RES_TYPES) {
    if (READ_ONCE(misc_res_capacity[i])) {
    max = READ_ONCE(cg.res[i].max);
    if (max == MAX_NUM) {
    seq_printf(sf, "%s max\n", misc_res_name[i]);
    }
    else {
    seq_printf(sf, "%s %llu\n", misc_res_name[i],
    max);
    }
    }
    }
    return 0;
    }
//
// misc_cg_max_write() - Update the maximum limit of the cgroup.
// @of: Handler for the file.
// @buf: Data from the user. It should be either "max", 0, or a positive
// integer.
// @nbytes: Number of bytes of the data.
// @off: Offset in the file.
//
// User can pass data like:
// echo sev 23 > misc.max, OR
// echo sev max > misc.max
//
// Context: Any context.
// Return:
// * >= 0 - Number of bytes processed in the input.
// * -EINVAL - If buf is not valid.
// * -ERANGE - If number is bigger than the u64 capacity.
//
#[no_mangle]
pub unsafe extern "C" fn misc_cg_max_write(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t {
pub static mut cg: *mut c_void = core::ptr::null_mut();
    let mut max = 0;
pub static mut ret: c_int = 0;
pub static mut type: misc_res_type = 0;
pub static mut token: *mut c_void = core::ptr::null_mut();
    buf = strstrip(buf);
    token = strsep(&buf, " ");
    if (!token || !buf) {
    return -EINVAL;
    }
    while (i < MISC_CG_RES_TYPES) {
    if (!strcmp(misc_res_name[i], token)) {
    type = i;
    break;
    }
    }
    if (type == MISC_CG_RES_TYPES) {
    return -EINVAL;
    }
    if (!strcmp(MAX_STR, buf)) {
    max = MAX_NUM;
    } else {
    ret = kstrtou64(buf, 0, &max);
    if (ret) {
    return ret;
    }
    }
    cg = css_misc(of_css(of));
    if (READ_ONCE(misc_res_capacity[type])) {
    WRITE_ONCE(cg.res[type].max, max);
    }
    else {
    ret = -EINVAL;
    }
    return ret ? ret : nbytes;
    }
//
// misc_cg_current_show() - Show the current usage of the misc cgroup.
// @sf: Interface file
// @v: Arguments passed
//
// Context: Any context.
// Return: 0 to denote successful print.
//
#[no_mangle]
unsafe extern "C" fn misc_cg_current_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    let mut i = 0;
    let mut usage = 0;
    let mut cg = css_misc(seq_css(sf));
    while (i < MISC_CG_RES_TYPES) {
    usage = atomic64_read(&cg.res[i].usage);
    if (READ_ONCE(misc_res_capacity[i]) || usage) {
    seq_printf(sf, "%s %llu\n", misc_res_name[i], usage);
    }
    }
    return 0;
    }
//
// misc_cg_peak_show() - Show the peak usage of the misc cgroup.
// @sf: Interface file
// @v: Arguments passed
//
// Context: Any context.
// Return: 0 to denote successful print.
//
#[no_mangle]
unsafe extern "C" fn misc_cg_peak_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    let mut i = 0;
    let mut watermark = 0;
    let mut cg = css_misc(seq_css(sf));
    while (i < MISC_CG_RES_TYPES) {
    watermark = atomic64_read(&cg.res[i].watermark);
    if (READ_ONCE(misc_res_capacity[i]) || watermark) {
    seq_printf(sf, "%s %llu\n", misc_res_name[i], watermark);
    }
    }
    return 0;
    }
//
// misc_cg_capacity_show() - Show the total capacity of misc res on the host.
// @sf: Interface file
// @v: Arguments passed
//
// Only present in the root cgroup directory.
//
// Context: Any context.
// Return: 0 to denote successful print.
//
#[no_mangle]
unsafe extern "C" fn misc_cg_capacity_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    let mut i = 0;
    let mut cap = 0;
    while (i < MISC_CG_RES_TYPES) {
    cap = READ_ONCE(misc_res_capacity[i]);
    if (cap) {
    seq_printf(sf, "%s %llu\n", misc_res_name[i], cap);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __misc_events_show(sf: *mut seq_file, local: bool) -> c_int {
    let mut cg = css_misc(seq_css(sf));
    let mut events = 0;
    let mut i = 0;
    while (i < MISC_CG_RES_TYPES) {
    if (local) {
    events = atomic64_read(&cg.res[i].events_local);
    }
    else {
    events = atomic64_read(&cg.res[i].events);
    }
    if (READ_ONCE(misc_res_capacity[i]) || events) {
    seq_printf(sf, "%s.max %llu\n", misc_res_name[i], events);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn misc_events_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    return __misc_events_show(sf, false);
    }
#[no_mangle]
unsafe extern "C" fn misc_events_local_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    return __misc_events_show(sf, true);
    }
// Misc cgroup interface files
pub static mut cftype: usize = 0;
//
// misc_cg_alloc() - Allocate misc cgroup.
// @parent_css: Parent cgroup.
//
// Context: Process context.
// Return:
// * struct cgroup_subsys_state* - css of the allocated cgroup.
// * ERR_PTR(-ENOMEM) - No memory available to allocate.
//
#[no_mangle]
pub unsafe extern "C" fn misc_cg_alloc(parent_css: *mut cgroup_subsys_state) -> *mut c_void {
    enum misc_res_type i;
pub static mut cg: *mut c_void = core::ptr::null_mut();
    if (!parent_css) {
    cg = &root_cg;
    } else {
    cg = kzalloc_obj(*cg);
    if (!cg) {
    return ERR_PTR(-ENOMEM);
    }
    }
    while (i < MISC_CG_RES_TYPES) {
    WRITE_ONCE(cg.res[i].max, MAX_NUM);
    atomic64_set(&cg.res[i].usage, 0);
    }
    return &cg.css;
    }
//
// misc_cg_free() - Free the misc cgroup.
// @css: cgroup subsys object.
//
// Context: Any context.
//
#[no_mangle]
unsafe extern "C" fn misc_cg_free(css: *mut cgroup_subsys_state) {
    kfree(css_misc(css));
    }
// Cgroup controller callbacks
pub static mut cgroup_subsys: usize = 0;
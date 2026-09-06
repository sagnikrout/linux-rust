//! Automatically rewritten from C to Rust
//! Source: kernel/fail_function.c
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
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

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
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0
//
// fail_function.c: Function-based error injection
//

// forward_decl: fei_kprobe_handler;
#[no_mangle]
pub unsafe extern "C" fn fei_post_handler() {
//
// A dummy post handler is required to prohibit optimizing, because
// jump optimization does not support execution path overriding.
//
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fei_attr {
    pub list: list_head,
    pub kp: kprobe,
    pub retval: c_ulong,
}
// static DEFINE_MUTEX(fei_lock);
// static LIST_HEAD(fei_attr_list);
// static DECLARE_FAULT_ATTR(fei_fault_attr);
pub static mut fei_debugfs_dir: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn adjust_error_retval(addr: c_ulong, retv: c_ulong) -> c_ulong {
    switch (get_injectable_error_type(addr)) {
    EI_ETYPE_NULL => {
    return 0;
    EI_ETYPE_ERRNO => {
    if (retv < (unsigned long)-MAX_ERRNO) {
    return (unsigned long)-EINVAL;
    }
    break;
    EI_ETYPE_ERRNO_NULL => {
    if (retv != 0 && retv < (unsigned long)-MAX_ERRNO) {
    return (unsigned long)-EINVAL;
    }
    break;
    EI_ETYPE_TRUE => {
    return 1;
    }
    return retv;
    }
#[no_mangle]
pub unsafe extern "C" fn fei_attr_new() {
    let mut attr = core::ptr::null_mut();
    attr = kzalloc_obj(*attr);
    if (attr) {
    attr.kp.symbol_name = kstrdup(sym, GFP_KERNEL);
    if (!attr.kp.symbol_name) {
    kfree(attr);
    return core::ptr::null_mut();
    }
    attr.kp.pre_handler = fei_kprobe_handler;
    attr.kp.post_handler = fei_post_handler;
    attr.retval = adjust_error_retval(addr, 0);
// INIT_LIST_HEAD;
    }
    return attr;
    }
#[no_mangle]
unsafe extern "C" fn fei_attr_free(attr: *mut fei_attr) {
    if (attr) {
    kfree(attr.kp.symbol_name);
    kfree(attr);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn fei_attr_lookup() {
    let mut attr = core::ptr::null_mut();
    list_for_each_entry(attr, &fei_attr_list, list) {
    if (!strcmp(attr.kp.symbol_name, sym)) {
    return attr;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn fei_attr_is_valid(_attr: *mut fei_attr) -> bool {
    let mut attr = core::ptr::null_mut();
    list_for_each_entry(attr, &fei_attr_list, list) {
    if (attr == _attr) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn fei_retval_set(data: *mut c_void, val: u64) -> c_int {
    let mut attr = data;
pub static mut retv: c_ulong = 0;
pub static mut err: c_int = 0;
    mutex_lock(&fei_lock);
//
// Since this operation can be done after retval file is removed,
// It is safer to check the attr is still valid before accessing
// its member.
//
    if (!fei_attr_is_valid(attr)) {
    err = -ENOENT;
// goto;
    }
    if (attr.kp.addr) {
    if (adjust_error_retval((unsigned long)attr.kp.addr,
    val) != retv) {
    err = -EINVAL;
    }
    }
    if (!err) {
    attr.retval = val;
    }
// label;
    mutex_unlock(&fei_lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn fei_retval_get(data: *mut c_void, val: *mut u64) -> c_int {
    let mut attr = data;
pub static mut err: c_int = 0;
    mutex_lock(&fei_lock);
// Here we also validate @attr to ensure it still exists.
    if (!fei_attr_is_valid(attr)) {
    err = -ENOENT;
    }
    else {
// val = attr->retval;
    }
    mutex_unlock(&fei_lock);
    return err;
    }
pub static mut fei_retval_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn fei_debugfs_add_attr(attr: *mut fei_attr) {
    let mut dir = core::ptr::null_mut();
    dir = debugfs_create_dir(attr.kp.symbol_name, fei_debugfs_dir);
    debugfs_create_file("retval", 0600, dir, attr, &fei_retval_ops);
    }
#[no_mangle]
unsafe extern "C" fn fei_debugfs_remove_attr(attr: *mut fei_attr) {
    debugfs_lookup_and_remove(attr.kp.symbol_name, fei_debugfs_dir);
    }
#[no_mangle]
unsafe extern "C" fn fei_kprobe_handler(kp: *mut kprobe, regs: *mut pt_regs) -> c_int {
    let mut attr = container_of!(kp, fei_attr, kp);
    if (should_fail(&fei_fault_attr, 1)) {
    regs_set_return_value(regs, attr.retval);
    override_function_with_return(regs);
    return 1;
    }
    return 0;
    }
    NOKPROBE_SYMBOL(fei_kprobe_handler)
#[no_mangle]
pub unsafe extern "C" fn fei_seq_start() {
    mutex_lock(&fei_lock);
    return seq_list_start(&fei_attr_list, *pos);
    }
#[no_mangle]
unsafe extern "C" fn fei_seq_stop(m: *mut seq_file, v: *mut c_void) {
    mutex_unlock(&fei_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn fei_seq_next() {
    return seq_list_next(v, &fei_attr_list, pos);
    }
#[no_mangle]
unsafe extern "C" fn fei_seq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut attr = list_entry(v, fei_attr, list);
    seq_printf(m, "%ps\n", attr.kp.addr);
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn fei_open(inode: *mut inode, file: *mut file) -> c_int {
    return seq_open(file, &fei_seq_ops);
    }
#[no_mangle]
unsafe extern "C" fn fei_attr_remove(attr: *mut fei_attr) {
    fei_debugfs_remove_attr(attr);
    unregister_kprobe(&attr.kp);
    list_del(&attr.list);
    fei_attr_free(attr);
    }
#[no_mangle]
unsafe extern "C" fn fei_attr_remove_all() {
    let mut attr = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
    list_for_each_entry_safe(attr, n, &fei_attr_list, list) {
    fei_attr_remove(attr);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn fei_write() {
    let mut attr = core::ptr::null_mut();
    let mut addr = 0;
    let mut buf = core::ptr::null_mut();
    let mut sym = core::ptr::null_mut();
    let mut ret = 0;
// cut off if it is too long
    if (count > KSYM_NAME_LEN) {
    count = KSYM_NAME_LEN;
    }
    buf = memdup_user_nul(buffer, count);
    if (IS_ERR(buf)) {
    return PTR_ERR(buf);
    }
    sym = strstrip(buf);
    mutex_lock(&fei_lock);
// Writing just spaces will remove all injection points
    if (sym[0] == '\0') {
    fei_attr_remove_all();
    ret = count;
// goto;
    }
// Writing !function will remove one injection point
    if (sym[0] == '!') {
    attr = fei_attr_lookup(sym + 1);
    if (!attr) {
    ret = -ENOENT;
// goto;
    }
    fei_attr_remove(attr);
    ret = count;
// goto;
    }
    addr = kallsyms_lookup_name(sym);
    if (!addr) {
    ret = -EINVAL;
// goto;
    }
    if (!within_error_injection_list(addr)) {
    ret = -ERANGE;
// goto;
    }
    if (fei_attr_lookup(sym)) {
    ret = -EBUSY;
// goto;
    }
    attr = fei_attr_new(sym, addr);
    if (!attr) {
    ret = -ENOMEM;
// goto;
    }
    ret = register_kprobe(&attr.kp);
    if (ret) {
    fei_attr_free(attr);
// goto;
    }
    fei_debugfs_add_attr(attr);
    list_add_tail(&attr.list, &fei_attr_list);
    ret = count;
// label;
    mutex_unlock(&fei_lock);
    kfree(buf);
    return ret;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn fei_debugfs_init() -> c_int {
    let mut dir = core::ptr::null_mut();
    dir = fault_create_debugfs_attr("fail_function", core::ptr::null_mut(),
    &fei_fault_attr);
    if (IS_ERR(dir)) {
    return PTR_ERR(dir);
    }
// injectable attribute is just a symlink of error_inject/list
    debugfs_create_symlink("injectable", dir, "../error_injection/list");
    debugfs_create_file("inject", 0600, dir, core::ptr::null_mut(), &fei_ops);
    fei_debugfs_dir = dir;
    return 0;
    }
// late_initcall;
}
}
}
}
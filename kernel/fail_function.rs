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

    static int fei_kprobe_handler(struct kprobe *kp, struct pt_regs *regs);
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
    static struct dentry *fei_debugfs_dir;
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
    struct fei_attr *attr = data;
pub static mut retv: c_ulong = (unsigned long)val;
pub static mut err: c_int = 0;
    mutex_lock(&fei_lock);
//
// Since this operation can be done after retval file is removed,
// It is safer to check the attr is still valid before accessing
// its member.
//
    if (!fei_attr_is_valid(attr)) {
    err = -ENOENT;
    goto out;
    }
    if (attr.kp.addr) {
    if (adjust_error_retval((unsigned long)attr.kp.addr,
    val) != retv)
    err = -EINVAL;
    }
    if (!err) {
    attr.retval = val;
    }
    out:
    mutex_unlock(&fei_lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn fei_retval_get(data: *mut c_void, val: *mut u64) -> c_int {
    struct fei_attr *attr = data;
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
    DEFINE_DEBUGFS_ATTRIBUTE(fei_retval_ops, fei_retval_get, fei_retval_set,
    "%llx\n");
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
    struct fei_attr *attr = container_of(kp, struct fei_attr, kp);
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
    struct fei_attr *attr = list_entry(v, struct fei_attr, list);
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
    struct fei_attr *attr, *n;
    list_for_each_entry_safe(attr, n, &fei_attr_list, list) {
    fei_attr_remove(attr);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn fei_write() {
    let mut attr = core::ptr::null_mut();
    let mut addr = 0;
    char *buf, *sym;
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
    goto out;
    }
// Writing !function will remove one injection point
    if (sym[0] == '!') {
    attr = fei_attr_lookup(sym + 1);
    if (!attr) {
    ret = -ENOENT;
    goto out;
    }
    fei_attr_remove(attr);
    ret = count;
    goto out;
    }
    addr = kallsyms_lookup_name(sym);
    if (!addr) {
    ret = -EINVAL;
    goto out;
    }
    if (!within_error_injection_list(addr)) {
    ret = -ERANGE;
    goto out;
    }
    if (fei_attr_lookup(sym)) {
    ret = -EBUSY;
    goto out;
    }
    attr = fei_attr_new(sym, addr);
    if (!attr) {
    ret = -ENOMEM;
    goto out;
    }
    ret = register_kprobe(&attr.kp);
    if (ret) {
    fei_attr_free(attr);
    goto out;
    }
    fei_debugfs_add_attr(attr);
    list_add_tail(&attr.list, &fei_attr_list);
    ret = count;
    out:
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
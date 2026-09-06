//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_events_inject.c
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
// trace_events_inject - trace event injection
//
// Copyright (C) 2019 Cong Wang <cwang@twitter.com>
//

#[no_mangle]
pub unsafe extern "C" fn trace_inject_entry(file: *mut trace_event_file, rec: *mut c_void, len: c_int) -> c_int {
pub static mut fbuffer: usize = 0;
pub static mut written: c_int = 0;
pub static mut entry: *mut c_void = core::ptr::null_mut();
    rcu_read_lock_sched();
    entry = trace_event_buffer_reserve(&fbuffer, file, len);
    if (entry) {
    memcpy(entry, rec, len);
    written = len;
    trace_event_buffer_commit(&fbuffer);
    }
    rcu_read_unlock_sched();
    return written;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_field(str: *mut c_char, call: *mut trace_event_call, pf: *mut *mut ftrace_event_field, pv: *mut u64) -> c_int {
pub static mut field: *mut c_void = core::ptr::null_mut();
pub static mut field_name: *mut c_void = core::ptr::null_mut();
    int s, i = 0;
    let mut len = 0;
    let mut val = 0;
    if (!str[i]) {
    return 0;
    }
// First find the field to associate to
    while (isspace(str[i])) {
    i += 1;
    }
    s = i;
    while (isalnum(str[i]) || str[i] == '_') {
    i += 1;
    }
    len = i - s;
    if (!len) {
    return -EINVAL;
    }
    field_name = kmemdup_nul(str + s, len, GFP_KERNEL);
    if (!field_name) {
    return -ENOMEM;
    }
    field = trace_find_event_field(call, field_name);
    kfree(field_name);
    if (!field) {
    return -ENOENT;
    }
// pf = field;
    while (isspace(str[i])) {
    i += 1;
    }
    if (str[i] != '=') {
    return -EINVAL;
    }
    i += 1;
    while (isspace(str[i])) {
    i += 1;
    }
    s = i;
    if (isdigit(str[i]) || str[i] == '-') {
    char *num, c;
    let mut ret = 0;
// Make sure the field is not a string
    if (is_string_field(field)) {
    return -EINVAL;
    }
    if (str[i] == '-') {
    i += 1;
    }
// We allow 0xDEADBEEF
    while (isalnum(str[i])) {
    i += 1;
    }
    num = str + s;
    c = str[i];
    if (c != '\0' && !isspace(c)) {
    return -EINVAL;
    }
    str[i] = '\0';
// Make sure it is a value
    if (field.is_signed) {
    ret = kstrtoll(num, 0, &val);
    }
    else {
    ret = kstrtoull(num, 0, &val);
    }
    str[i] = c;
    if (ret) {
    return ret;
    }
// pv = val;
    return i;
    } else if (str[i] == '\'' || str[i] == '"') {
pub static mut q: c_char = 0;
// Make sure the field is OK for strings
    if (!is_string_field(field)) {
    return -EINVAL;
    }
    while (str[i]) {
    if (str[i] == '\\' && str[i + 1]) {
    i += 1;
    continue;
    }
    if (str[i] == q) {
    break;
    }
    }
    if (!str[i]) {
    return -EINVAL;
    }
// Skip quotes
    s += 1;
    len = i - s;
    if (len >= MAX_FILTER_STR_VAL) {
    return -EINVAL;
    }
// pv = (unsigned long)(str + s);
    str[i] = 0;
// go past the last quote
    i += 1;
    return i;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn trace_get_entry_size(call: *mut trace_event_call) -> c_int {
pub static mut field: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut size: c_int = 0;
    head = trace_get_fields(call);
    list_for_each_entry(field, head, link) {
    if (field.size + field.offset > size) {
    size = field.size + field.offset;
    }
    }
    return size;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_alloc_entry(call: *mut trace_event_call, size: *mut c_int) -> *mut c_void {
pub static mut entry_size: c_int = 0;
pub static mut field: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut entry = core::ptr::null_mut();
// We need an extra '\0' at the end.
    entry = kzalloc(entry_size + 1, GFP_KERNEL);
    if (!entry) {
    return core::ptr::null_mut();
    }
    head = trace_get_fields(call);
    list_for_each_entry(field, head, link) {
    if (!is_string_field(field)) {
    continue;
    }
    if (field.filter_type == FILTER_STATIC_STRING) {
    continue;
    }
    if (field.filter_type == FILTER_DYN_STRING ||
    field.filter_type == FILTER_RDYN_STRING) {
pub static mut str_item: *mut c_void = core::ptr::null_mut();
pub static mut str_loc: c_int = 0;
    if (field.filter_type == FILTER_RDYN_STRING) {
    str_loc -= field.offset + field.size;
    }
    str_item = (entry + field.offset);
// str_item = str_loc; // string length is 0.
    } else {
pub static mut paddr: *mut c_void = core::ptr::null_mut();
    paddr = (entry + field.offset);
// paddr = "";
    }
    }
// size = entry_size + 1;
    return entry;
    }

// Caller is responsible to free the *pentry.
#[no_mangle]
unsafe extern "C" fn parse_entry(str: *mut c_char, call: *mut trace_event_call, pentry: *mut c_void) -> c_int {
pub static mut field: *mut c_void = core::ptr::null_mut();
    let mut entry = core::ptr::null_mut();
    let mut entry_size = 0;
pub static mut val: u64 = 0;
    let mut len = 0;
    entry = trace_alloc_entry(call, &entry_size);
// pentry = entry;
    if (!entry) {
    return -ENOMEM;
    }
    tracing_generic_entry_update(entry, call.event.type,
    tracing_gen_ctx());
    while ((len = parse_field(str, call, &field, &val)) > 0) {
    if (is_function_field(field)) {
    return -EINVAL;
    }
    if (is_string_field(field)) {
    let mut addr = (unsigned long) val;
    if (field.filter_type == FILTER_STATIC_STRING) {
    strscpy(entry + field.offset, addr, field.size);
    } else if (field.filter_type == FILTER_DYN_STRING ||
    field.filter_type == FILTER_RDYN_STRING) {
pub static mut str_len: c_int = 0;
pub static mut str_loc: c_int = 0;
pub static mut str_item: *mut c_void = core::ptr::null_mut();
    entry_size += str_len;
// pentry = krealloc(entry, entry_size, GFP_KERNEL);
    if (!*pentry) {
    kfree(entry);
    return -ENOMEM;
    }
    entry = *pentry;
    strscpy(entry + (entry_size - str_len), addr, str_len);
    str_item = (entry + field.offset);
    if (field.filter_type == FILTER_RDYN_STRING) {
    str_loc -= field.offset + field.size;
    }
// str_item = (str_len << 16) | str_loc;
    } else {
pub static mut paddr: *mut c_void = core::ptr::null_mut();
    paddr = (entry + field.offset);
// paddr = INJECT_STRING;
    }
    } else {
    match (field.size) {
    1 => {
pub static mut tmp: u8 = 0;
    memcpy(entry + field.offset, &tmp, 1);
    // break;
    }
    }
    case 2: {
pub static mut tmp: u16 = 0;
    memcpy(entry + field.offset, &tmp, 2);
    break;
    }
    case 4: {
pub static mut tmp: u32 = 0;
    memcpy(entry + field.offset, &tmp, 4);
    break;
    }
    case 8:
    memcpy(entry + field.offset, &val, 8);
    break;
// label;
    return -EINVAL;
    }
    }
    str += len;
    }
    if (len < 0) {
    return len;
    }
    return entry_size;
    }
#[no_mangle]
pub unsafe extern "C" fn event_inject_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut call: *mut c_void = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    let mut entry = core::ptr::null_mut();
pub static mut buf: *mut c_void = core::ptr::null_mut();
    if (cnt >= PAGE_SIZE) {
    return -EINVAL;
    }
    buf = memdup_user_nul(ubuf, cnt);
    if (IS_ERR(buf)) {
    return PTR_ERR(buf);
    }
    strim(buf);
    mutex_lock(&event_mutex);
    file = event_file_file(filp);
    if (file) {
    call = file.event_call;
    size = parse_entry(buf, call, &entry);
    if (size < 0) {
    err = size;
    }
    else {
    err = trace_inject_entry(file, entry, size);
    }
    }
    mutex_unlock(&event_mutex);
    kfree(entry);
    kfree(buf);
    if (err < 0) {
    return err;
    }
// ppos += err;
    return cnt;
    }
#[no_mangle]
pub unsafe extern "C" fn event_inject_read(file: *mut file, buf: *mut c_char, size: size_t, ppos: *mut loff_t) -> ssize_t {
    return -EPERM;
    }
pub static mut file_operations: usize = 0;
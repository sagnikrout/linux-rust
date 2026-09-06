//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_events.c
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
// event tracer
//
// Copyright (C) 2008 Red Hat Inc, Steven Rostedt <srostedt@redhat.com>
//
// - Added format output of fields of the trace point.
// This was based off of work by Tom Zanussi <tzanussi@gmail.com>.
//

pub static mut event_mutex: usize = 0;
pub static mut ftrace_events: usize = 0;
pub static mut ftrace_generic_fields: usize = 0;
pub static mut ftrace_common_fields: usize = 0;
    static bool eventdir_initialized;
pub static mut module_strings: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct module_string {
    pub next: list_head,
    pub module: *mut module,
    pub str: *mut c_char,
}

pub static mut field_cachep: *mut c_void = core::ptr::null_mut();
pub static mut file_cachep: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn system_refcount(system: *mut event_subsystem) -> c_int {
    return system.ref_count;
    }
#[no_mangle]
unsafe extern "C" fn system_refcount_inc(system: *mut event_subsystem) -> c_int {
    return system.ref_count += 1;
    }
#[no_mangle]
unsafe extern "C" fn system_refcount_dec(system: *mut event_subsystem) -> c_int {
    return --system.ref_count;
    }
// Double loops, do not use break, only goto's work

    list_for_each_entry(tr, &ftrace_trace_arrays, list) {	
    list_for_each_entry(file, &tr.events, list) {

    list_for_each_entry(tr, &ftrace_trace_arrays, list) {	
    }
pub static mut ___n: *mut c_void = core::ptr::null_mut();				
    list_for_each_entry_safe(file, ___n, &tr.events, list) {

    }
#[no_mangle]
pub unsafe extern "C" fn __find_event_field(head: *mut list_head, name: *mut c_char) -> *mut c_void {
    }
pub static mut field: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(field, head, link) {
    if (!strcmp(field.name, name)) {
    return field;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn trace_find_event_field(call: *mut trace_event_call, name: *mut c_char) -> *mut c_void {
pub static mut field: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    head = trace_get_fields(call);
    field = __find_event_field(head, name);
    if (field) {
    return field;
    }
    field = __find_event_field(&ftrace_generic_fields, name);
    if (field) {
    return field;
    }
    return __find_event_field(&ftrace_common_fields, name);
    }
#[no_mangle]
pub unsafe extern "C" fn __trace_define_field(head: *mut list_head, type: *mut c_char, name: *mut c_char, offset: c_int, size: c_int, is_signed: c_int, filter_type: c_int, len: c_int, need_test: c_int) -> c_int {
pub static mut field: *mut c_void = core::ptr::null_mut();
    field = kmem_cache_alloc(field_cachep, GFP_TRACE);
    if (!field) {
    return -ENOMEM;
    }
    field.name = name;
    field.type = type;
    if (filter_type == FILTER_OTHER) {
    field.filter_type = filter_assign_type(type);
    }
    else {
    field.filter_type = filter_type;
    }
    field.offset = offset;
    field.size = size;
    field.is_signed = is_signed;
    field.needs_test = need_test;
    field.len = len;
    list_add(&field.link, head);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_define_field(call: *mut trace_event_call, type: *mut c_char, name: *mut c_char, offset: c_int, size: c_int, is_signed: c_int, filter_type: c_int) -> c_int {
pub static mut head: *mut c_void = core::ptr::null_mut();
    if (WARN_ON!(!call.class)) {
    return 0;
    }
    head = trace_get_fields(call);
    return __trace_define_field(head, type, name, offset, size,
    is_signed, filter_type, 0, 0);
    }
    EXPORT_SYMBOL_GPL(trace_define_field);
#[no_mangle]
pub unsafe extern "C" fn trace_define_field_ext(call: *mut trace_event_call, type: *mut c_char, name: *mut c_char, offset: c_int, size: c_int, is_signed: c_int, filter_type: c_int, len: c_int, need_test: c_int) -> c_int {
pub static mut head: *mut c_void = core::ptr::null_mut();
    if (WARN_ON!(!call.class)) {
    return 0;
    }
    head = trace_get_fields(call);
    return __trace_define_field(head, type, name, offset, size,
    is_signed, filter_type, len, need_test);
    }

    ret = __trace_define_field(&ftrace_generic_fields, #type,	

    filter_type, 0, 0);			
    if (ret)							 {
    return ret;
    }

    ret = __trace_define_field(&ftrace_common_fields, #type,	
    "common_" #item,			
    offsetof(typeof(ent), item),		
    sizeof!(ent.item),			
    is_signed_type(type), FILTER_OTHER,	
    0, 0);				
    if (ret)							 {
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn trace_define_generic_fields() -> c_int {
    let mut ret = 0;
    __generic_field(int, CPU, FILTER_CPU);
    __generic_field(int, cpu, FILTER_CPU);
    __generic_field(int, common_cpu, FILTER_CPU);
    __generic_field(char *, COMM, FILTER_COMM);
    __generic_field(char *, comm, FILTER_COMM);
    __generic_field(char *, stacktrace, FILTER_STACKTRACE);
    __generic_field(char *, STACKTRACE, FILTER_STACKTRACE);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn trace_define_common_fields() -> c_int {
    let mut ret = 0;
pub static mut ent: usize = 0;
    __common_field(unsigned short, type);
    __common_field(unsigned char, flags);
// Holds both preempt_count and migrate_disable
    __common_field(unsigned char, preempt_count);
    __common_field(int, pid);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn trace_destroy_fields(call: *mut trace_event_call) {
    let mut field = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    head = trace_get_fields(call);
    list_for_each_entry_safe(field, next, head, link) {
    list_del(&field.link);
    kmem_cache_free(field_cachep, field);
    }
    }
//
// run-time version of trace_event_get_offsets_<call>() that returns the last
// accessible offset of trace fields excluding __dynamic_array bytes
//
#[no_mangle]
pub unsafe extern "C" fn trace_event_get_offsets(call: *mut trace_event_call) -> c_int {
pub static mut tail: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    head = trace_get_fields(call);
//
// head->next points to the last field with the largest offset,
since it was added last by trace_define_field()
//
    tail = list_first_entry(head, ftrace_event_field, link);
    return tail.offset + tail.size;
    }
#[no_mangle]
pub unsafe extern "C" fn find_event_field(fmt: *mut c_char, call: *mut trace_event_call) -> *mut c_void {
    let mut field = call.class.fields_array;
    let mut p = fmt;
    let mut len = 0;
    if (!(len = str_has_prefix(fmt, "REC."))) {
    return core::ptr::null_mut();
    }
    fmt += len;
    while (*p) {
    if (!isalnum(*p) && *p != '_') {
    break;
    }
    }
    len = p - fmt;
    while (field.type) {
    if (strncmp(field.name, fmt, len) || field.name[len]) {
    continue;
    }
    return field;
    }
    return core::ptr::null_mut();
    }
//
// Check if the referenced field is an array and return true,
// as arrays are OK to dereference.
//
#[no_mangle]
unsafe extern "C" fn test_field(fmt: *const c_char, call: *mut trace_event_call) -> bool {
pub static mut field: *mut c_void = core::ptr::null_mut();
    field = find_event_field(fmt, call);
    if (!field) {
    return false;
    }
// This is an array and is OK to dereference.
    return strchr(field.type, '[') != core::ptr::null_mut();
    }
// Look for a string within an argument
#[no_mangle]
unsafe extern "C" fn find_print_string(arg: *const c_char, str: *const c_char, end: *const c_char) -> bool {
pub static mut r: *mut c_void = core::ptr::null_mut();
    r = strstr(arg, str);
    return r && r < end;
    }
// Return true if the argument pointer is safe
#[no_mangle]
unsafe extern "C" fn process_pointer(fmt: *const c_char, len: c_int, call: *mut trace_event_call) -> bool {
    let mut r = core::ptr::null_mut();
    let mut e = core::ptr::null_mut();
    let mut a = core::ptr::null_mut();
    e = fmt + len;
// Find the REC-> in the argument
    r = strstr(fmt, "REC.");
    if (r && r < e) {
//
// Addresses of events on the buffer, or an array on the buffer is
// OK to dereference. There's ways to fool this, but
// this is to catch common mistakes, not malicious code.
//
    a = strchr(fmt, '&');
    if ((a && (a < r)) || test_field(r, call)) {
    return true;
    }
    } else if (find_print_string(fmt, "__get_dynamic_array(", e)) {
    return true;
    } else if (find_print_string(fmt, "__get_rel_dynamic_array(", e)) {
    return true;
    } else if (find_print_string(fmt, "__get_dynamic_array_len(", e)) {
    return true;
    } else if (find_print_string(fmt, "__get_rel_dynamic_array_len(", e)) {
    return true;
    } else if (find_print_string(fmt, "__get_sockaddr(", e)) {
    return true;
    } else if (find_print_string(fmt, "__get_rel_sockaddr(", e)) {
    return true;
    }
    return false;
    }
// Return true if the string is safe
#[no_mangle]
unsafe extern "C" fn process_string(fmt: *const c_char, len: c_int, call: *mut trace_event_call) -> bool {
pub static mut field: *mut c_void = core::ptr::null_mut();
    let mut r = core::ptr::null_mut();
    let mut e = core::ptr::null_mut();
    let mut s = core::ptr::null_mut();
    e = fmt + len;
//
// There are several helper functions that return strings.
// If the argument contains a function, then assume its field is valid.
// It is considered that the argument has a function if it has:
// alphanumeric or '_' before a parenthesis.
//
    s = fmt;
    do {
    r = strstr(s, "(");
    if (!r || r >= e) {
    break;
    }
    while (r - i >= s) {
pub static mut ch: c_char = 0;
    if (isspace(ch)) {
    continue;
    }
    if (isalnum(ch) || ch == '_') {
    return true;
    }
// Anything else, this isn't a function
    break;
    }
// A function could be wrapped in parenthesis, try the next one
    s = r + 1;
    } while (s < e);
//
// Check for arrays. If the argument has: foo[REC->val]
// then it is very likely that foo is an array of strings
// that are safe to use.
//
    r = strstr(s, "[");
    if (r && r < e) {
    r = strstr(r, "REC.");
    if (r && r < e) {
    return true;
    }
    }
//
// If there's any strings in the argument consider this arg OK as it
// could be: REC->field ? "foo" : "bar" and we don't want to get into
// verifying that logic here.
//
    if (find_print_string(fmt, "\"", e)) {
    return true;
    }
// Dereferenced strings are also valid like any other pointer
    if (process_pointer(fmt, len, call)) {
    return true;
    }
// Make sure the field is found
    field = find_event_field(fmt, call);
    if (!field) {
    return false;
    }
// Test this field's string before printing the event
    call.flags |= TRACE_EVENT_FL_TEST_STR;
    field.needs_test = 1;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn test_double_dereference(str: *mut c_char, len: c_int, call: *mut trace_event_call) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    let mut end = str + len;
    ptr = strstr(str, "REC.");
    while (ptr && ptr < end) {
    ptr += 5;
    while (ptr < end) {
    if (ptr[0] == '-' && ptr[1] == '>') {
    pr_warn!("TRACE EVENT ERROR: Event %s has double dereference in TP_printk: %.*s\n",
    trace_event_name(call), len, str);
    WARN_ONCE(1, "Event %s has double dereference in TP_printk: %.*s\n",
    trace_event_name(call), len, str);
    return;
    }
    if (!isalnum(*ptr) && *ptr != '_') {
    break;
    }
    }
    ptr = strstr(ptr, "REC.");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn handle_dereference_arg(arg_str: *mut c_char, string_flags: u64, len: c_int, dereference_flags: *mut u64, arg: c_int, call: *mut trace_event_call) {
    if (string_flags & (1ULL << arg)) {
    if (process_string(arg_str, len, call)) {
// dereference_flags &= ~(1ULL << arg);
    }
    } else if (process_pointer(arg_str, len, call)) {
// dereference_flags &= ~(1ULL << arg);
    }
    else {
    pr_warn!("TRACE EVENT ERROR: Bad dereference argument: '%.*s'\n",
    len, arg_str);
    }
    }
//
// Examine the print fmt of the event looking for unsafe dereference
// pointers using %p* that could be recorded in the trace event and
// much later referenced after the pointer was freed. Dereferencing
// pointers are OK, if it is dereferenced into the event itself.
//
#[no_mangle]
unsafe extern "C" fn test_event_printk(call: *mut trace_event_call) {
pub static mut dereference_flags: u64 = 0;
pub static mut string_flags: u64 = 0;
pub static mut first: bool = true;
pub static mut fmt: *mut c_void = core::ptr::null_mut();
pub static mut parens: c_int = 0;
pub static mut in_quote: c_char = 0;
pub static mut start_arg: c_int = 0;
pub static mut arg: c_int = 0;
    let mut i = 0;
    let mut e = 0;
    fmt = call.print_fmt;
    if (!fmt) {
    return;
    }
    while (fmt[i]) {
    match (fmt[i]) {
    '\\' => {
    i += 1;
    if (!fmt[i]) {
    return;
    }
    continue;
    }
    '"' => {
    }
    '\'' => {
//
// The print fmt starts with a string that
// is processed first to find %p* usage,
// then after the first string, the print fmt
// contains arguments that are used to check
// if the dereferenced %p* usage is safe.
//
    if (first) {
    if (fmt[i] == '\'') {
    continue;
    }
    if (in_quote) {
    arg = 0;
    first = false;
    }
    }
    if (in_quote) {
    if (in_quote == fmt[i]) {
    in_quote = 0;
    }
    } else {
    in_quote = fmt[i];
    }
    continue;
    }
    '%' => {
    if (!first || !in_quote) {
    continue;
    }
    i += 1;
    if (!fmt[i]) {
    return;
    }
    match (fmt[i]) {
    '%' => {
    continue;
    }
    'p' => {
// label;
// Find dereferencing fields
    match (fmt[i + 1]) {
    'B' => {
    }
    'b' => {
    }
    'I' => {
    }
    'U' => {
    }
    'a' => {
    }
    'g' => {
    }
    'O' => {
    if (WARN_ONCE(arg == 63,
    "Too many args for event: %s",
    trace_event_name(call))) {
    return;
    }
    dereference_flags |= 1ULL << arg;
    }
    }
    break;
// label;
    {
pub static mut star: bool = false;
    let mut j = 0;
// Increment arg if %*s exists.
    while (fmt[i + j]) {
    if (isdigit(fmt[i + j]) ||
    fmt[i + j] == '.') {
    continue;
    }
    if (fmt[i + j] == '*') {
    star = true;
// Handle %*pbl case
    if (!j && fmt[i + 1] == 'p') {
    arg += 1;
    i += 1;
// goto;
    }
    continue;
    }
    if ((fmt[i + j] == 's')) {
    if (star) {
    arg += 1;
    }
    if (WARN_ONCE(arg == 63,
    "Too many args for event: %s",
    trace_event_name(call))) {
    return;
    }
    dereference_flags |= 1ULL << arg;
    string_flags |= 1ULL << arg;
    }
    break;
    }
    break;
    } /* default */
    } /* switch */
    arg += 1;
    continue;
    case '(':
    if (in_quote) {
    continue;
    }
    parens += 1;
    continue;
    case ')':
    if (in_quote) {
    continue;
    }
    parens -= 1;
    if (WARN_ONCE(parens < 0,
    "Paren mismatch for event: %s\narg='%s'\n%*s",
    trace_event_name(call),
    fmt + start_arg,
    (i - start_arg) + 5, "^")) {
    return;
    }
    continue;
    case ',':
    if (in_quote || parens) {
    continue;
    }
    e = i;
    i += 1;
    while (isspace(fmt[i])) {
    i += 1;
    }
//
// If start_arg is zero, then this is the start of the
// first argument. The processing of the argument happens
// when the end of the argument is found, as it needs to
// handle parenthesis and such.
//
    if (!start_arg) {
    start_arg = i;
// Balance out the i++ in the for loop
    i -= 1;
    continue;
    }
    test_double_dereference(fmt + start_arg, e - start_arg, call);
    if (dereference_flags & (1ULL << arg)) {
    handle_dereference_arg(fmt + start_arg, string_flags,
    e - start_arg,
    &dereference_flags, arg, call);
    }
    start_arg = i;
    arg += 1;
// Balance out the i++ in the for loop
    i -= 1;
    }
    }
    test_double_dereference(fmt + start_arg, i - start_arg, call);
    if (dereference_flags & (1ULL << arg)) {
    handle_dereference_arg(fmt + start_arg, string_flags,
    i - start_arg,
    &dereference_flags, arg, call);
    }
//
// If you triggered the below warning, the trace event reported
// uses an unsafe dereference pointer %p*. As the data stored
// at the trace event time may no longer exist when the trace
// event is printed, dereferencing to the original source is
// unsafe. The source of the dereference must be copied into the
// event itself, and the dereference must access the copy instead.
//
    if (WARN_ON_ONCE!(dereference_flags)) {
    arg = 1;
    while (!(dereference_flags & 1)) {
    dereference_flags >>= 1;
    arg += 1;
    }
    pr_warn!("event %s has unsafe dereference of argument %d\n",
    trace_event_name(call), arg);
    pr_warn!("print_fmt: %s\n", fmt);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn trace_event_raw_init(call: *mut trace_event_call) -> c_int {
    let mut id = 0;
    id = register_trace_event(&call.event);
    if (!id) {
    return -ENODEV;
    }
    test_event_printk(call);
    return 0;
    }
    EXPORT_SYMBOL_GPL(trace_event_raw_init);
#[no_mangle]
pub unsafe extern "C" fn trace_event_ignore_this_pid(trace_file: *mut trace_event_file) -> bool {
    let mut tr = trace_file.tr;
pub static mut no_pid_list: *mut c_void = core::ptr::null_mut();
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
    pid_list = rcu_dereference_raw(tr.filtered_pids);
    no_pid_list = rcu_dereference_raw(tr.filtered_no_pids);
    if (!pid_list && !no_pid_list) {
    return false;
    }
//
// This is recorded at every sched_switch for this task.
// Thus, even if the task migrates the ignore value will be the same.
//
    return this_cpu_read(tr.array_buffer.data.ignore_pid) != 0;
    }
    EXPORT_SYMBOL_GPL(trace_event_ignore_this_pid);
//
// trace_event_buffer_reserve - reserve space on the ring buffer for an event
// @fbuffer: information about how to save the event
// @trace_file: the instance file descriptor for the event
// @len: The length of the event
//
// The @fbuffer has information about the ring buffer and data will
// be added to it to be used by the call to trace_event_buffer_commit().
// The @trace_file is the desrciptor with information about the status
// of the given event for a specific trace_array instance.
// The @len is the length of data to save for the event.
//
// Returns a pointer to the data on the ring buffer or NULL if the
// event was not reserved (event was filtered, too big, or the buffer
// simply was disabled for write).
//
#[no_mangle]
pub unsafe extern "C" fn trace_event_buffer_reserve(fbuffer: *mut trace_event_buffer, trace_file: *mut trace_event_file, len: c_ulong) -> *mut c_void {
    let mut event_call = trace_file.event_call;
    if ((trace_file.flags & EVENT_FILE_FL_PID_FILTER) &&
    trace_event_ignore_this_pid(trace_file)) {
    return core::ptr::null_mut();
    }
//
// If CONFIG_PREEMPTION is enabled, then the tracepoint itself disables
// preemption (adding one to the preempt_count). Since we are
// interested in the preempt_count at the time the tracepoint was
// hit, we need to subtract one to offset the increment.
//
    fbuffer.trace_ctx = tracing_gen_ctx_dec();
    fbuffer.trace_file = trace_file;
    fbuffer.event =
    trace_event_buffer_lock_reserve(&fbuffer.buffer, trace_file,
    event_call.event.type, len,
    fbuffer.trace_ctx);
    if (!fbuffer.event) {
    return core::ptr::null_mut();
    }
    fbuffer.regs = core::ptr::null_mut();
    fbuffer.entry = ring_buffer_event_data(fbuffer.event);
    return fbuffer.entry;
    }
    EXPORT_SYMBOL_GPL(trace_event_buffer_reserve);
#[no_mangle]
pub unsafe extern "C" fn trace_event_reg(call: *mut trace_event_call, type: trace_reg, data: *mut c_void) -> c_int {
    let mut file = data;
    WARN_ON!(!(call.flags & TRACE_EVENT_FL_TRACEPOINT));
    match (type) {
    TRACE_REG_REGISTER => {
    return tracepoint_probe_register(call.tp,
    call.class.probe,
    file);
    }
    TRACE_REG_UNREGISTER => {
    tracepoint_probe_unregister(call.tp,
    call.class.probe,
    file);
    return 0;

    }
    TRACE_REG_PERF_REGISTER => {
    if (!call.class.perf_probe) {
    return -ENODEV;
    }
    return tracepoint_probe_register(call.tp,
    call.class.perf_probe,
    call);
    }
    TRACE_REG_PERF_UNREGISTER => {
    tracepoint_probe_unregister(call.tp,
    call.class.perf_probe,
    call);
    return 0;
    }
    TRACE_REG_PERF_OPEN => {
    }
    TRACE_REG_PERF_CLOSE => {
    }
    TRACE_REG_PERF_ADD => {
    }
    TRACE_REG_PERF_DEL => {
    return 0;

    }
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(trace_event_reg);
#[no_mangle]
pub unsafe extern "C" fn trace_event_enable_cmd_record(enable: bool) {
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut tr: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&event_mutex);
    do_for_each_event_file(tr, file) {
    if (!(file.flags & EVENT_FILE_FL_ENABLED)) {
    continue;
    }
    if (enable) {
    tracing_start_cmdline_record();
    set_bit(EVENT_FILE_FL_RECORDED_CMD_BIT, &file.flags);
    } else {
    tracing_stop_cmdline_record();
    clear_bit(EVENT_FILE_FL_RECORDED_CMD_BIT, &file.flags);
    }
    } while_for_each_event_file();
    }
#[no_mangle]
pub unsafe extern "C" fn trace_event_enable_tgid_record(enable: bool) {
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut tr: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&event_mutex);
    do_for_each_event_file(tr, file) {
    if (!(file.flags & EVENT_FILE_FL_ENABLED)) {
    continue;
    }
    if (enable) {
    tracing_start_tgid_record();
    set_bit(EVENT_FILE_FL_RECORDED_TGID_BIT, &file.flags);
    } else {
    tracing_stop_tgid_record();
    clear_bit(EVENT_FILE_FL_RECORDED_TGID_BIT,
    &file.flags);
    }
    } while_for_each_event_file();
    }
#[no_mangle]
pub unsafe extern "C" fn __ftrace_event_enable_disable(file: *mut trace_event_file, enable: c_int, soft_disable: c_int) -> c_int {
    let mut call = file.event_call;
    let mut tr = file.tr;
pub static mut soft_mode: bool = false;
pub static mut ret: c_int = 0;
    let mut disable = 0;
    match (enable) {
    0 => {
//
// When soft_disable is set and enable is cleared, the sm_ref
// reference counter is decremented. If it reaches 0, we want
// to clear the SOFT_DISABLED flag but leave the event in the
// state that it was. That is, if the event was enabled and
// SOFT_DISABLED isn't set, then do nothing. But if SOFT_DISABLED
// is set we do not want the event to be enabled before we
// clear the bit.
//
// When soft_disable is not set but the soft_mode is,
// we do nothing. Do not disable the tracepoint, otherwise
// "soft enable"s (clearing the SOFT_DISABLED bit) won't work.
//
    if (soft_disable) {
    if (atomic_dec_return(&file.sm_ref) > 0) {
    // break;
    }
    disable = file.flags & EVENT_FILE_FL_SOFT_DISABLED;
    soft_mode = false;
// Disable use of trace_buffered_event
    trace_buffered_event_disable();
    } else {
    disable = !soft_mode;
    }
    if (disable && (file.flags & EVENT_FILE_FL_ENABLED)) {
    clear_bit(EVENT_FILE_FL_ENABLED_BIT, &file.flags);
    if (file.flags & EVENT_FILE_FL_RECORDED_CMD) {
    tracing_stop_cmdline_record();
    clear_bit(EVENT_FILE_FL_RECORDED_CMD_BIT, &file.flags);
    }
    if (file.flags & EVENT_FILE_FL_RECORDED_TGID) {
    tracing_stop_tgid_record();
    clear_bit(EVENT_FILE_FL_RECORDED_TGID_BIT, &file.flags);
    }
    ret = call.class.reg(call, TRACE_REG_UNREGISTER, file);
    WARN_ON_ONCE!(ret);
    }
// If in soft mode, just set the SOFT_DISABLE_BIT, else clear it
    if (soft_mode) {
    set_bit(EVENT_FILE_FL_SOFT_DISABLED_BIT, &file.flags);
    }
    else {
    clear_bit(EVENT_FILE_FL_SOFT_DISABLED_BIT, &file.flags);
    }
    // break;
    }
    1 => {
//
// When soft_disable is set and enable is set, we want to
// register the tracepoint for the event, but leave the event
// as is. That means, if the event was already enabled, we do
// nothing. If the event is disabled, we set SOFT_DISABLED
// before enabling the event tracepoint, so it still seems
// to be disabled.
//
    if (!soft_disable) {
    clear_bit(EVENT_FILE_FL_SOFT_DISABLED_BIT, &file.flags);
    }
    else {
    if (atomic_inc_return(&file.sm_ref) > 1) {
    // break;
    }
// Enable use of trace_buffered_event
    trace_buffered_event_enable();
    }
    if (!(file.flags & EVENT_FILE_FL_ENABLED)) {
pub static mut cmd: bool = false;
// Keep the event disabled, when going to soft mode.
    if (soft_disable) {
    set_bit(EVENT_FILE_FL_SOFT_DISABLED_BIT, &file.flags);
    }
    if (tr.trace_flags & TRACE_ITER(RECORD_CMD)) {
    cmd = true;
    tracing_start_cmdline_record();
    set_bit(EVENT_FILE_FL_RECORDED_CMD_BIT, &file.flags);
    }
    if (tr.trace_flags & TRACE_ITER(RECORD_TGID)) {
    tgid = true;
    tracing_start_tgid_record();
    set_bit(EVENT_FILE_FL_RECORDED_TGID_BIT, &file.flags);
    }
    ret = call.class.reg(call, TRACE_REG_REGISTER, file);
    if (ret) {
    if (cmd) {
    tracing_stop_cmdline_record();
    }
    if (tgid) {
    tracing_stop_tgid_record();
    }
    pr_info!("event trace: Could not enable event "
    "%s\n", trace_event_name(call));
    // break;
    }
    set_bit(EVENT_FILE_FL_ENABLED_BIT, &file.flags);
// WAS_ENABLED gets set but never cleared.
    set_bit(EVENT_FILE_FL_WAS_ENABLED_BIT, &file.flags);
    }
    // break;
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_event_enable_disable(file: *mut trace_event_file, enable: c_int, soft_disable: c_int) -> c_int {
    return __ftrace_event_enable_disable(file, enable, soft_disable);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_event_enable_disable(file: *mut trace_event_file, enable: c_int) -> c_int {
    return __ftrace_event_enable_disable(file, enable, 0);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_mod_load {
    pub list: list_head,
    pub module: *mut c_char,
    pub match: *mut c_char,
    pub system: *mut c_char,
    pub event: *mut c_char,
}

#[no_mangle]
unsafe extern "C" fn free_event_mod(event_mod: *mut event_mod_load) {
    list_del(&event_mod.list);
    kfree(event_mod.module);
    kfree(event_mod.match);
    kfree(event_mod.system);
    kfree(event_mod.event);
    kfree(event_mod);
    }
#[no_mangle]
unsafe extern "C" fn clear_mod_events(tr: *mut trace_array) {
    let mut event_mod = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
    list_for_each_entry_safe(event_mod, n, &tr.mod_events, list) {
    free_event_mod(event_mod);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn remove_cache_mod(tr: *mut trace_array, mod: *mut c_char, match: *mut c_char, system: *mut c_char, event: *mut c_char) -> c_int {
    let mut event_mod = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    list_for_each_entry_safe(event_mod, n, &tr.mod_events, list) {
    if (strcmp(event_mod.module, mod) != 0) {
    continue;
    }
    if (match && (!event_mod.match || strcmp(event_mod.match, match) != 0)) {
    continue;
    }
    if (system &&
    (!event_mod.system || strcmp(event_mod.system, system) != 0)) {
    continue;
    }
    if (event &&
    (!event_mod.event || strcmp(event_mod.event, event) != 0)) {
    continue;
    }
    free_event_mod(event_mod);
    ret = 0;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cache_mod(tr: *mut trace_array, mod: *mut c_char, set: c_int, match: *mut c_char, system: *mut c_char, event: *mut c_char) -> c_int {
pub static mut event_mod: *mut c_void = core::ptr::null_mut();
// If the module exists, then this just failed to find an event
    if (module_exists!(mod)) {
    return -EINVAL;
    }
// See if this is to remove a cached filter
    if (!set) {
    return remove_cache_mod(tr, mod, match, system, event);
    }
    event_mod = kzalloc_obj(*event_mod);
    if (!event_mod) {
    return -ENOMEM;
    }
    INIT_LIST_HEAD(&event_mod.list);
    event_mod.module = kstrdup(mod, GFP_KERNEL);
    if (!event_mod.module) {
// goto;
    }
    if (match) {
    event_mod.match = kstrdup(match, GFP_KERNEL);
    if (!event_mod.match) {
// goto;
    }
    }
    if (system) {
    event_mod.system = kstrdup(system, GFP_KERNEL);
    if (!event_mod.system) {
// goto;
    }
    }
    if (event) {
    event_mod.event = kstrdup(event, GFP_KERNEL);
    if (!event_mod.event) {
// goto;
    }
    }
    list_add(&event_mod.list, &tr.mod_events);
    return 0;
// label;
    free_event_mod(event_mod);
    return -ENOMEM;
    }

#[no_mangle]
pub unsafe extern "C" fn clear_mod_events(tr: *mut trace_array) { }
#[no_mangle]
#[no_mangle]
// duplicate fn: cache_mod
pub unsafe extern "C" fn cache_mod_dup(tr: *mut trace_array, mod: *mut c_char, set: c_int, match: *mut c_char, system: *mut c_char, event: *mut c_char) -> c_int {
    return -EINVAL;
    }

#[no_mangle]
unsafe extern "C" fn ftrace_clear_events(tr: *mut trace_array) {
pub static mut file: *mut c_void = core::ptr::null_mut();
    mutex_lock(&event_mutex);
    list_for_each_entry(file, &tr.events, list) {
    ftrace_event_enable_disable(file, 0);
    }
    clear_mod_events(tr);
    mutex_unlock(&event_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn event_filter_pid_sched_process_exit(data: *mut c_void, task: *mut task_struct) {
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
    let mut tr = data;
    guard(preempt)();
    pid_list = rcu_dereference_raw(tr.filtered_pids);
    trace_filter_add_remove_task(pid_list, core::ptr::null_mut(), task);
    pid_list = rcu_dereference_raw(tr.filtered_no_pids);
    trace_filter_add_remove_task(pid_list, core::ptr::null_mut(), task);
    }
#[no_mangle]
pub unsafe extern "C" fn event_filter_pid_sched_process_fork(data: *mut c_void, self: *mut task_struct, task: *mut task_struct) {
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
    let mut tr = data;
    guard(preempt)();
    pid_list = rcu_dereference_sched(tr.filtered_pids);
    trace_filter_add_remove_task(pid_list, self, task);
    pid_list = rcu_dereference_sched(tr.filtered_no_pids);
    trace_filter_add_remove_task(pid_list, self, task);
    }
#[no_mangle]
pub unsafe extern "C" fn trace_event_follow_fork(tr: *mut trace_array, enable: bool) {
    if (enable) {
    register_trace_prio_sched_process_fork(event_filter_pid_sched_process_fork,
    tr, INT_MIN);
    register_trace_prio_sched_process_free(event_filter_pid_sched_process_exit,
    tr, INT_MAX);
    } else {
    unregister_trace_sched_process_fork(event_filter_pid_sched_process_fork,
    tr);
    unregister_trace_sched_process_free(event_filter_pid_sched_process_exit,
    tr);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn event_filter_pid_sched_switch_probe_pre(data: *mut c_void, preempt: bool, prev: *mut task_struct, next: *mut task_struct, prev_state: c_uint) {
    let mut tr = data;
pub static mut no_pid_list: *mut c_void = core::ptr::null_mut();
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    pid_list = rcu_dereference_sched(tr.filtered_pids);
    no_pid_list = rcu_dereference_sched(tr.filtered_no_pids);
//
// Sched switch is funny, as we only want to ignore it
// in the notrace case if both prev and next should be ignored.
//
    ret = trace_ignore_this_task(core::ptr::null_mut(), no_pid_list, prev) &&
    trace_ignore_this_task(core::ptr::null_mut(), no_pid_list, next);
    this_cpu_write(tr.array_buffer.data.ignore_pid, ret ||
    (trace_ignore_this_task(pid_list, core::ptr::null_mut(), prev) &&
    trace_ignore_this_task(pid_list, core::ptr::null_mut(), next)));
    }
#[no_mangle]
pub unsafe extern "C" fn event_filter_pid_sched_switch_probe_post(data: *mut c_void, preempt: bool, prev: *mut task_struct, next: *mut task_struct, prev_state: c_uint) {
    let mut tr = data;
pub static mut no_pid_list: *mut c_void = core::ptr::null_mut();
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
    pid_list = rcu_dereference_sched(tr.filtered_pids);
    no_pid_list = rcu_dereference_sched(tr.filtered_no_pids);
    this_cpu_write(tr.array_buffer.data.ignore_pid,
    trace_ignore_this_task(pid_list, no_pid_list, next));
    }
#[no_mangle]
pub unsafe extern "C" fn event_filter_pid_sched_wakeup_probe_pre(data: *mut c_void, task: *mut task_struct) {
    let mut tr = data;
pub static mut no_pid_list: *mut c_void = core::ptr::null_mut();
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
// Nothing to do if we are already tracing
    if (!this_cpu_read(tr.array_buffer.data.ignore_pid)) {
    return;
    }
    pid_list = rcu_dereference_sched(tr.filtered_pids);
    no_pid_list = rcu_dereference_sched(tr.filtered_no_pids);
    this_cpu_write(tr.array_buffer.data.ignore_pid,
    trace_ignore_this_task(pid_list, no_pid_list, task));
    }
#[no_mangle]
pub unsafe extern "C" fn event_filter_pid_sched_wakeup_probe_post(data: *mut c_void, task: *mut task_struct) {
    let mut tr = data;
pub static mut no_pid_list: *mut c_void = core::ptr::null_mut();
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
// Nothing to do if we are not tracing
    if (this_cpu_read(tr.array_buffer.data.ignore_pid)) {
    return;
    }
    pid_list = rcu_dereference_sched(tr.filtered_pids);
    no_pid_list = rcu_dereference_sched(tr.filtered_no_pids);
// Set tracing if current is enabled
    this_cpu_write(tr.array_buffer.data.ignore_pid,
    trace_ignore_this_task(pid_list, no_pid_list, current));
    }
#[no_mangle]
unsafe extern "C" fn unregister_pid_events(tr: *mut trace_array) {
    unregister_trace_sched_switch(event_filter_pid_sched_switch_probe_pre, tr);
    unregister_trace_sched_switch(event_filter_pid_sched_switch_probe_post, tr);
    unregister_trace_sched_wakeup(event_filter_pid_sched_wakeup_probe_pre, tr);
    unregister_trace_sched_wakeup(event_filter_pid_sched_wakeup_probe_post, tr);
    unregister_trace_sched_wakeup_new(event_filter_pid_sched_wakeup_probe_pre, tr);
    unregister_trace_sched_wakeup_new(event_filter_pid_sched_wakeup_probe_post, tr);
    unregister_trace_sched_waking(event_filter_pid_sched_wakeup_probe_pre, tr);
    unregister_trace_sched_waking(event_filter_pid_sched_wakeup_probe_post, tr);
    }
#[no_mangle]
unsafe extern "C" fn __ftrace_clear_event_pids(tr: *mut trace_array, type: c_int) {
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
pub static mut no_pid_list: *mut c_void = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    pid_list = rcu_dereference_protected(tr.filtered_pids,
    lockdep_is_held(&event_mutex));
    no_pid_list = rcu_dereference_protected(tr.filtered_no_pids,
    lockdep_is_held(&event_mutex));
// Make sure there's something to do
    if (!pid_type_enabled(type, pid_list, no_pid_list)) {
    return;
    }
    if (!still_need_pid_events(type, pid_list, no_pid_list)) {
    unregister_pid_events(tr);
    list_for_each_entry(file, &tr.events, list) {
    clear_bit(EVENT_FILE_FL_PID_FILTER_BIT, &file.flags);
    }
    for_each_possible_cpu(cpu) {
    per_cpu_ptr(tr.array_buffer.data, cpu).ignore_pid = false;
    }
    }
    if (type & TRACE_PIDS) {
    rcu_assign_pointer(tr.filtered_pids, core::ptr::null_mut());
    }
    if (type & TRACE_NO_PIDS) {
    rcu_assign_pointer(tr.filtered_no_pids, core::ptr::null_mut());
    }
// Wait till all users are no longer using pid filtering
    tracepoint_synchronize_unregister();
    if ((type & TRACE_PIDS) && pid_list) {
    trace_pid_list_free(pid_list);
    }
    if ((type & TRACE_NO_PIDS) && no_pid_list) {
    trace_pid_list_free(no_pid_list);
    }
    }
#[no_mangle]
unsafe extern "C" fn ftrace_clear_event_pids(tr: *mut trace_array, type: c_int) {
    mutex_lock(&event_mutex);
    __ftrace_clear_event_pids(tr, type);
    mutex_unlock(&event_mutex);
    }
#[no_mangle]
unsafe extern "C" fn __put_system(system: *mut event_subsystem) {
    let mut filter = system.filter;
    WARN_ON_ONCE!(system_refcount(system) == 0);
    if (system_refcount_dec(system)) {
    return;
    }
    list_del(&system.list);
    if (filter) {
    kfree(filter.filter_string);
    kfree(filter);
    }
    kfree_const(system.name);
    kfree(system);
    }
#[no_mangle]
unsafe extern "C" fn __get_system(system: *mut event_subsystem) {
    WARN_ON_ONCE!(system_refcount(system) == 0);
    system_refcount_inc(system);
    }
#[no_mangle]
unsafe extern "C" fn __get_system_dir(dir: *mut trace_subsystem_dir) {
    WARN_ON_ONCE!(dir.ref_count == 0);
    dir.ref_count += 1;
    __get_system(dir.subsystem);
    }
#[no_mangle]
unsafe extern "C" fn __put_system_dir(dir: *mut trace_subsystem_dir) {
    WARN_ON_ONCE!(dir.ref_count == 0);
// If the subsystem is about to be freed, the dir must be too
    WARN_ON_ONCE!(system_refcount(dir.subsystem) == 1 && dir.ref_count != 1);
    __put_system(dir.subsystem);
    if (!--dir.ref_count) {
    kfree(dir);
    }
    }
#[no_mangle]
unsafe extern "C" fn put_system(dir: *mut trace_subsystem_dir) {
    mutex_lock(&event_mutex);
    __put_system_dir(dir);
    mutex_unlock(&event_mutex);
    }
#[no_mangle]
unsafe extern "C" fn remove_subsystem(dir: *mut trace_subsystem_dir) {
    if (!dir) {
    return;
    }
    if (!--dir.nr_events) {
    eventfs_remove_dir(dir.ei);
    list_del(&dir.list);
    __put_system_dir(dir);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn event_file_get(file: *mut trace_event_file) {
    refcount_inc(&file.ref);
    }
#[no_mangle]
pub unsafe extern "C" fn event_file_put(file: *mut trace_event_file) {
    if (WARN_ON_ONCE!(!refcount_read(&file.ref))) {
    if (file.flags & EVENT_FILE_FL_FREED) {
    kmem_cache_free(file_cachep, file);
    }
    return;
    }
    if (refcount_dec_and_test(&file.ref)) {
// Count should only go to zero when it is freed
    if (WARN_ON_ONCE!(!(file.flags & EVENT_FILE_FL_FREED))) {
    return;
    }
    kmem_cache_free(file_cachep, file);
    }
    }
#[no_mangle]
unsafe extern "C" fn remove_event_file_dir(file: *mut trace_event_file) {
    eventfs_remove_dir(file.ei);
    list_del(&file.list);
    remove_subsystem(file.system);
    free_event_filter(file.filter);
    file.flags |= EVENT_FILE_FL_FREED;
    event_file_put(file);
// Wake up hist poll waiters to notice the EVENT_FILE_FL_FREED flag.
    hist_poll_wakeup();
    }
//
// __ftrace_set_clr_event(NULL, NULL, NULL, set) will set/unset all events.
//
#[no_mangle]
pub unsafe extern "C" fn __ftrace_set_clr_event_nolock(tr: *mut trace_array, match: *mut c_char, sub: *mut c_char, event: *mut c_char, set: c_int, mod: *mut c_char) -> c_int {
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut call: *mut c_void = core::ptr::null_mut();
    char *module __free(kfree) = core::ptr::null_mut();
pub static mut name: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
pub static mut eret: c_int = 0;
    if (mod) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    module = kstrdup(mod, GFP_KERNEL);
    if (!module) {
    return -ENOMEM;
    }
// Replace all '-' with '_' as that's what modules do
    for (p = strchr(module, '-'); p; p = strchr(p + 1, '-')) {
// p = '_';
    }
    }
    list_for_each_entry(file, &tr.events, list) {
    call = file.event_call;
// If a module is specified, skip events that are not that module
    if (module &&
    ((call.flags & TRACE_EVENT_FL_DYNAMIC) ||
    !call.module || strcmp(module_name!(call.module), module))) {
    continue;
    }
    name = trace_event_name(call);
    if (!name || !call.class || !call.class.reg) {
    continue;
    }
    if (call.flags & TRACE_EVENT_FL_IGNORE_ENABLE) {
    continue;
    }
    if (match &&
    strcmp(match, name) != 0 &&
    strcmp(match, call.class.system) != 0) {
    continue;
    }
    if (sub && strcmp(sub, call.class.system) != 0) {
    continue;
    }
    if (event && strcmp(event, name) != 0) {
    continue;
    }
    ret = ftrace_event_enable_disable(file, set);
//
// Save the first error and return that. Some events
// may still have been enabled, but let the user
// know that something went wrong.
//
    if (ret && !eret) {
    eret = ret;
    }
    ret = eret;
    }
//
// If this is a module setting and nothing was found,
// check if the module was loaded. If it wasn't cache it.
//
    if (module && ret == -EINVAL && !eret) {
    ret = cache_mod(tr, module, set, match, sub, event);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __ftrace_set_clr_event(tr: *mut trace_array, match: *mut c_char, sub: *mut c_char, event: *mut c_char, set: c_int, mod: *mut c_char) -> c_int {
    let mut ret = 0;
    if (trace_array_is_readonly(tr)) {
    return -EACCES;
    }
    mutex_lock(&event_mutex);
    ret = __ftrace_set_clr_event_nolock(tr, match, sub, event, set, mod);
    mutex_unlock(&event_mutex);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_set_clr_event(tr: *mut trace_array, buf: *mut c_char, set: c_int) -> c_int {
    let mut event = core::ptr::null_mut(), *sub = core::ptr::null_mut(), *match, *mod;
    let mut ret = 0;
    if (!tr) {
    return -ENOENT;
    }
// Modules events can be appended with :mod:<module>
    mod = strstr(buf, ":mod:");
    if (mod) {
// mod = '\0';
// move to the module name
    mod += 5;
    }
//
// The buf format can be <subsystem>:<event-name>
// *:<event-name> means any event by that name.
// :<event-name> is the same.
//
// <subsystem>:* means all events in that subsystem
// <subsystem>: means the same.
//
// <name> (no ':') means all events in a subsystem with
// the name <name> or any event that matches <name>
//
    match = strsep(&buf, ":");
    if (buf) {
    sub = match;
    event = buf;
    match = core::ptr::null_mut();
    if (!strlen(sub) || strcmp(sub, "*") == 0) {
    sub = core::ptr::null_mut();
    }
    if (!strlen(event) || strcmp(event, "*") == 0) {
    event = core::ptr::null_mut();
    }
    } else if (mod) {
// Allow wildcard for no length or star
    if (!strlen(match) || strcmp(match, "*") == 0) {
    match = core::ptr::null_mut();
    }
    }
    ret = __ftrace_set_clr_event(tr, match, sub, event, set, mod);
// Put back the colon to allow this to be called again
    if (buf) {
// (buf - 1) = ':';
    }
    return ret;
    }
//
// trace_set_clr_event - enable or disable an event
// @system: system name to match (NULL for any system)
// @event: event name to match (NULL for all events, within system)
// @set: 1 to enable, 0 to disable
//
// This is a way for other parts of the kernel to enable or disable
// event recording.
//
// Returns 0 on success, -EINVAL if the parameters do not match any
// registered events.
//
#[no_mangle]
pub unsafe extern "C" fn trace_set_clr_event(system: *const c_char, event: *const c_char, set: c_int) -> c_int {
    let mut tr = top_trace_array();
    if (!tr) {
    return -ENODEV;
    }
    return __ftrace_set_clr_event(tr, core::ptr::null_mut(), system, event, set, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(trace_set_clr_event);
//
// trace_array_set_clr_event - enable or disable an event for a trace array.
// @tr: concerned trace array.
// @system: system name to match (NULL for any system)
// @event: event name to match (NULL for all events, within system)
// @enable: true to enable, false to disable
//
// This is a way for other parts of the kernel to enable or disable
// event recording.
//
// Returns 0 on success, -EINVAL if the parameters do not match any
// registered events.
//
#[no_mangle]
pub unsafe extern "C" fn trace_array_set_clr_event(tr: *mut trace_array, system: *mut c_char, event: *mut c_char, enable: bool) -> c_int {
    let mut set = 0;
    if (!tr) {
    return -ENOENT;
    }
    set = (enable == true) ? 1 : 0;
    return __ftrace_set_clr_event(tr, core::ptr::null_mut(), system, event, set, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(trace_array_set_clr_event);
// 128 should be much more than enough
pub const EVENT_BUF_SIZE: c_int = 127;
#[no_mangle]
pub unsafe extern "C" fn ftrace_event_write(file: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut parser: usize = 0;
    let mut m = file.private_data;
    let mut tr = m.private;
    ssize_t read, ret;
    if (!cnt) {
    return 0;
    }
    ret = tracing_update_buffers(tr);
    if (ret < 0) {
    return ret;
    }
    if (trace_parser_get_init(&parser, EVENT_BUF_SIZE + 1)) {
    return -ENOMEM;
    }
    read = trace_get_user(&parser, ubuf, cnt, ppos);
    if (read >= 0 && trace_parser_loaded((&parser))) {
pub static mut set: c_int = 1;
    if (*parser.buffer == '!') {
    set = 0;
    }
    ret = ftrace_set_clr_event(tr, parser.buffer + !set, set);
    if (ret) {
// goto;
    }
    }
    ret = read;
// label;
    trace_parser_put(&parser);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn t_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut file = v;
pub static mut call: *mut c_void = core::ptr::null_mut();
    let mut tr = m.private;
    (*pos)++;
    list_for_each_entry_continue(file, &tr.events, list) {
    call = file.event_call;
//
// The ftrace subsystem is for showing formats only.
// They can not be enabled or disabled via the event files.
//
    if (call.class && call.class.reg &&
    !(call.flags & TRACE_EVENT_FL_IGNORE_ENABLE)) {
    return file;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn t_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut tr = m.private;
    let mut l = 0;
    mutex_lock(&event_mutex);
    file = list_entry(&tr.events, trace_event_file, list);
    while (l <= *pos) {
    file = t_next(m, file, &l);
    if (!file) {
    break;
    }
    }
    return file;
    }
    enum set_event_iter_type {
    SET_EVENT_FILE,
    SET_EVENT_MOD,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_event_iter {
    pub type: set_event_iter_type,
    union {
    pub file: *mut trace_event_file,
    pub event_mod: *mut event_mod_load,
}

    };
#[no_mangle]
pub unsafe extern "C" fn s_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut iter = v;
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut tr = m.private;
    (*pos)++;
    if (iter.type == SET_EVENT_FILE) {
    file = iter.file;
    list_for_each_entry_continue(file, &tr.events, list) {
    if (file.flags & EVENT_FILE_FL_ENABLED) {
    iter.file = file;
    return iter;
    }
    }

    iter.type = SET_EVENT_MOD;
    iter.event_mod = list_entry(&tr.mod_events, event_mod_load, list);

    }

    list_for_each_entry_continue(iter.event_mod, &tr.mod_events, list) {
    return iter;
    }

//
// The iter is allocated in s_start() and passed via the 'v'
// parameter. To stop the iterator, NULL must be returned. But
// the return value is what the 'v' parameter in s_stop() receives
// and frees. Free iter here as it will no longer be used.
//
    kfree(iter);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn s_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut tr = m.private;
pub static mut iter: *mut c_void = core::ptr::null_mut();
    let mut l = 0;
    iter = kzalloc_obj(*iter);
    mutex_lock(&event_mutex);
    if (!iter) {
    return core::ptr::null_mut();
    }
    iter.type = SET_EVENT_FILE;
    iter.file = list_entry(&tr.events, trace_event_file, list);
    while (l <= *pos) {
    iter = s_next(m, iter, &l);
    if (!iter) {
    break;
    }
    }
    return iter;
    }
#[no_mangle]
unsafe extern "C" fn t_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut file = v;
    let mut call = file.event_call;
    if (strcmp(call.class.system, TRACE_SYSTEM) != 0) {
    seq_printf(m, "%s:", call.class.system);
    }
    seq_printf(m, "%s\n", trace_event_name(call));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn t_stop(m: *mut seq_file, p: *mut c_void) {
    mutex_unlock(&event_mutex);
    }
#[no_mangle]
unsafe extern "C" fn get_call_len(call: *mut trace_event_call) -> c_int {
    let mut len = 0;
// Get the length of "<system>:<event>"
    len = strlen(call.class.system) + 1;
    len += strlen(trace_event_name(call));
// Set the index to 32 bytes to separate event from data
    return len >= 32 ? 1 : 32 - len;
    }
//
// t_show_filters - seq_file callback to display active event filters
// @m: The seq_file interface for formatted output
// @v: The current trace_event_file being iterated
//
// Identifies and prints active filters for the current event file in the
// iteration. If a filter is applied to the current event and, if so,
// prints the system name, event name, and the filter string.
//
#[no_mangle]
unsafe extern "C" fn t_show_filters(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut file = v;
    let mut call = file.event_call;
pub static mut filter: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    guard(rcu)();
    filter = rcu_dereference(file.filter);
    if (!filter || !filter.filter_string) {
    return 0;
    }
    len = get_call_len(call);
    seq_printf(m, "%s:%s%*s%s\n", call.class.system,
    trace_event_name(call), len, "", filter.filter_string);
    return 0;
    }
//
// t_show_triggers - seq_file callback to display active event triggers
// @m: The seq_file interface for formatted output
// @v: The current trace_event_file being iterated
//
// Iterates through the trigger list of the current event file and prints
// each active trigger's configuration using its associated print
// operation.
//
#[no_mangle]
unsafe extern "C" fn t_show_triggers(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut file = v;
    let mut call = file.event_call;
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
//
// The event_mutex is held by t_start(), protecting the
// file->triggers list traversal.
//
    if (list_empty(&file.triggers)) {
    return 0;
    }
    len = get_call_len(call);
    list_for_each_entry_rcu(data, &file.triggers, list) {
    seq_printf(m, "%s:%s%*s", call.class.system,
    trace_event_name(call), len, "");
    data.cmd_ops.print(m, data);
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn s_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut iter = v;
pub static mut system: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
    if (iter.type == SET_EVENT_FILE) {
    return t_show(m, iter.file);
    }
// When match is set, system and event are not
    if (iter.event_mod.match) {
    seq_printf(m, "%s:mod:%s\n", iter.event_mod.match,
    iter.event_mod.module);
    return 0;
    }
    system = iter.event_mod.system ? : "*";
    event = iter.event_mod.event ? : "*";
    seq_printf(m, "%s:%s:mod:%s\n", system, event, iter.event_mod.module);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn s_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut iter = v;
    return t_show(m, iter.file);
    }

#[no_mangle]
unsafe extern "C" fn s_stop(m: *mut seq_file, v: *mut c_void) {
    kfree(v);
    t_stop(m, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn __next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t, type: c_int) -> *mut c_void {
    let mut tr = m.private;
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
    if (type == TRACE_PIDS) {
    pid_list = rcu_dereference_sched(tr.filtered_pids);
    }
    else {
    pid_list = rcu_dereference_sched(tr.filtered_no_pids);
    }
    return trace_pid_next(pid_list, v, pos);
    }
#[no_mangle]
pub unsafe extern "C" fn p_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    return __next(m, v, pos, TRACE_PIDS);
    }
#[no_mangle]
pub unsafe extern "C" fn np_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    return __next(m, v, pos, TRACE_NO_PIDS);
    }
#[no_mangle]
pub unsafe extern "C" fn __start(m: *mut seq_file, pos: *mut loff_t, RCU: int type)
    __acquires() -> *mut c_void {
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
    let mut tr = m.private;
//
// Grab the mutex, to keep calls to p_next() having the same
// tr->filtered_pids as p_start() has.
// If we just passed the tr->filtered_pids around, then RCU would
// have been enough, but doing that makes things more complex.
//
    mutex_lock(&event_mutex);
    rcu_read_lock_sched();
    if (type == TRACE_PIDS) {
    pid_list = rcu_dereference_sched(tr.filtered_pids);
    }
    else {
    pid_list = rcu_dereference_sched(tr.filtered_no_pids);
    }
    if (!pid_list) {
    return core::ptr::null_mut();
    }
    return trace_pid_start(pid_list, pos);
    }
#[no_mangle]
pub unsafe extern "C" fn p_start(m: *mut seq_file, RCU: *mut loff_tpos)
    __acquires() -> *mut c_void {
    return __start(m, pos, TRACE_PIDS);
    }
#[no_mangle]
pub unsafe extern "C" fn np_start(m: *mut seq_file, RCU: *mut loff_tpos)
    __acquires() -> *mut c_void {
    return __start(m, pos, TRACE_NO_PIDS);
    }
#[no_mangle]
unsafe extern "C" fn p_stop(m: *mut seq_file, p: *mut c_void) {
    rcu_read_unlock_sched();
    mutex_unlock(&event_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn event_enable_read(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    char buf[4] = "0";
    mutex_lock(&event_mutex);
    file = event_file_file(filp);
    if (likely(file)) {
    flags = file.flags;
    }
    mutex_unlock(&event_mutex);
    if (!file) {
    return -ENODEV;
    }
    if (flags & EVENT_FILE_FL_ENABLED &&
    !(flags & EVENT_FILE_FL_SOFT_DISABLED)) {
    strcpy(buf, "1");
    }
    if (atomic_read(&file.sm_ref) != 0) {
    strcat(buf, "*");
    }
    strcat(buf, "\n");
    return simple_read_from_buffer(ubuf, cnt, ppos, buf, strlen(buf));
    }
#[no_mangle]
pub unsafe extern "C" fn event_enable_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut val = 0;
    let mut ret = 0;
    ret = kstrtoul_from_user(ubuf, cnt, 10, &val);
    if (ret) {
    return ret;
    }
    guard(mutex)(&event_mutex);
    match (val) {
    0 => {
    }
    1 => {
    file = event_file_file(filp);
    if (!file) {
    return -ENODEV;
    }
    ret = tracing_update_buffers(file.tr);
    if (ret < 0) {
    return ret;
    }
    ret = ftrace_event_enable_disable(file, val);
    if (ret < 0) {
    return ret;
    }
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
// ppos += cnt;
    return cnt;
    }
//
// Returns:
// 0 : no events exist?
// 1 : all events are disabled
// 2 : all events are enabled
// 3 : some events are enabled and some are enabled
//
#[no_mangle]
pub unsafe extern "C" fn trace_events_enabled(tr: *mut trace_array, system: *const c_char) -> c_int {
pub static mut call: *mut c_void = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut set: c_int = 0;
    guard(mutex)(&event_mutex);
    list_for_each_entry(file, &tr.events, list) {
    call = file.event_call;
    if ((call.flags & TRACE_EVENT_FL_IGNORE_ENABLE) ||
    !trace_event_name(call) || !call.class || !call.class.reg) {
    continue;
    }
    if (system && strcmp(call.class.system, system) != 0) {
    continue;
    }
//
// We need to find out if all the events are set
// or if all events or cleared, or if we have
// a mixture.
//
    set |= (1 << !!(file.flags & EVENT_FILE_FL_ENABLED));
//
// If we have a mixture, no need to look further.
//
    if (set == 3) {
    break;
    }
    }
    return set;
    }
#[no_mangle]
pub unsafe extern "C" fn system_enable_read(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    const char set_to_char[4] = { '?', '0', '1', 'X' };
    let mut dir = filp.private_data;
    let mut system = dir.subsystem;
    let mut tr = dir.tr;
    char buf[2];
    let mut set = 0;
    let mut ret = 0;
    set = trace_events_enabled(tr, system ? system.name : core::ptr::null_mut());
    buf[0] = set_to_char[set];
    buf[1] = '\n';
    ret = simple_read_from_buffer(ubuf, cnt, ppos, buf, 2);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn system_enable_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut dir = filp.private_data;
    let mut system = dir.subsystem;
    let mut name = core::ptr::null_mut();
    let mut val = 0;
    let mut ret = 0;
    ret = kstrtoul_from_user(ubuf, cnt, 10, &val);
    if (ret) {
    return ret;
    }
    ret = tracing_update_buffers(dir.tr);
    if (ret < 0) {
    return ret;
    }
    if (val != 0 && val != 1) {
    return -EINVAL;
    }
//
// Opening of "enable" adds a ref count to system,
// so the name is safe to use.
//
    if (system) {
    name = system.name;
    }
    ret = __ftrace_set_clr_event(dir.tr, core::ptr::null_mut(), name, core::ptr::null_mut(), val, core::ptr::null_mut());
    if (ret) {
// goto;
    }
    ret = cnt;
// label;
// ppos += cnt;
    return ret;
    }
    enum {
    FORMAT_HEADER		= 1,
    FORMAT_FIELD_SEPERATOR	= 2,
    FORMAT_PRINTFMT		= 3,
    };
#[no_mangle]
pub unsafe extern "C" fn f_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut file = event_file_data(m.private);
    let mut call = file.event_call;
    let mut common_head = &ftrace_common_fields;
    let mut head = trace_get_fields(call);
    let mut node = v;
    (*pos)++;
    switch ((unsigned long)v) {
    case FORMAT_HEADER:
    node = common_head;
    break;
    case FORMAT_FIELD_SEPERATOR:
    node = head;
    break;
    case FORMAT_PRINTFMT:
// all done
    return core::ptr::null_mut();
    }
    node = node.prev;
    if (node == common_head) {
    return FORMAT_FIELD_SEPERATOR;
    }

    else if (node == head) {
    return FORMAT_PRINTFMT;
    }
    else {
    return node;
    }
    }
#[no_mangle]
unsafe extern "C" fn f_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut file = event_file_data(m.private);
    let mut call = file.event_call;
pub static mut field: *mut c_void = core::ptr::null_mut();
pub static mut array_descriptor: *mut c_void = core::ptr::null_mut();
    switch ((unsigned long)v) {
    case FORMAT_HEADER:
    seq_printf(m, "name: %s\n", trace_event_name(call));
    seq_printf(m, "ID: %d\n", call.event.type);
    seq_puts(m, "format:\n");
    return 0;
    case FORMAT_FIELD_SEPERATOR:
    seq_putc(m, '\n');
    return 0;
    case FORMAT_PRINTFMT:
    seq_printf(m, "\nprint fmt: %s\n",
    call.print_fmt);
    return 0;
    }
    field = list_entry(v, ftrace_event_field, link);
//
// Smartly shows the array type(except dynamic array).
// Normal:
// field:TYPE VAR
// If TYPE := TYPE[LEN], it is shown:
// field:TYPE VAR[LEN]
//
    array_descriptor = strchr(field.type, '[');
    if (str_has_prefix(field.type, "__data_loc")) {
    array_descriptor = core::ptr::null_mut();
    }
    if (!array_descriptor) {
    seq_printf(m, "\tfield:%s %s;\toffset:%u;\tsize:%u;\tsigned:%d;\n",
    }
    field.type, field.name, field.offset,
    field.size, !!field.is_signed);

    else if (field.len) {
    seq_printf(m, "\tfield:%.*s %s[%d];\toffset:%u;\tsize:%u;\tsigned:%d;\n",
    }
    (int)(array_descriptor - field.type),
    field.type, field.name,
    field.len, field.offset,
    field.size, !!field.is_signed);
    else {
    seq_printf(m, "\tfield:%.*s %s[];\toffset:%u;\tsize:%u;\tsigned:%d;\n",
    }
    (int)(array_descriptor - field.type),
    field.type, field.name,
    field.offset, field.size, !!field.is_signed);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn f_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut p = FORMAT_HEADER;
pub static mut l: loff_t = 0;
// ->stop() is called even if ->start() fails
    mutex_lock(&event_mutex);
    file = event_file_file(m.private);
    if (!file) {
    return ERR_PTR(-ENODEV);
    }
    while (l < *pos && p) {
    p = f_next(m, p, &l);
    }
    return p;
    }
#[no_mangle]
unsafe extern "C" fn f_stop(m: *mut seq_file, p: *mut c_void) {
    mutex_unlock(&event_mutex);
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn trace_format_open(inode: *mut inode, file: *mut file) -> c_int {
pub static mut m: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
// Do we want to hide event format files on tracefs lockdown?
    ret = seq_open(file, &trace_format_seq_ops);
    if (ret < 0) {
    return ret;
    }
    m = file.private_data;
    m.private = file;
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn event_id_read(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
// id is directly in i_private and available for inode's lifetime.
pub static mut id: c_int = 0;
    char buf[32];
    let mut len = 0;
    WARN_ON!(!id);
    len = sprintf(buf, "%d\n", id);
    return simple_read_from_buffer(ubuf, cnt, ppos, buf, len);
    }

#[no_mangle]
pub unsafe extern "C" fn event_btf_ids_read(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut call: *mut c_void = core::ptr::null_mut();
pub static mut t: *mut c_void = core::ptr::null_mut();
    let mut mod = core::ptr::null_mut();
pub static mut raw_id: u32 = 0;
pub static mut ids: *mut c_void = core::ptr::null_mut();
pub static mut btf: *mut c_void = core::ptr::null_mut();
    char buf[128];
    let mut len = 0;
// Module unload could free call->class and ids[] mid-read.
    scoped_guard(mutex, &event_mutex) {
    file = event_file_file(filp);
    if (!file) {
    return -ENODEV;
    }
    call = file.event_call;
    ids = call.class.btf_ids;
    if (!ids) {
    return -ENOENT;
    }
    if (!(call.flags & TRACE_EVENT_FL_DYNAMIC)) {
    mod = call.module;
    }
    btf = btf_get_module_btf(mod);
    if (IS_ERR_OR_NULL(btf)) {
    return -ENOENT;
    }
// Module-local ids in ids[] need base+local relocation.
    tp_id = btf_relocate_id(btf, ids[1]);
//
// Without FL_TRACEPOINT the dispatcher is shared (e.g. all
// per-syscall events fan out from __bpf_trace_sys_enter), so
// raw_btf_id has no per-event attach point — report 0.
//
    if (call.flags & TRACE_EVENT_FL_TRACEPOINT) {
    t = btf_type_by_id(btf, btf_relocate_id(btf, ids[0]));
    raw_id = t ? t.type : 0;
    }
    obj_id = btf_obj_id(btf);
    btf_put(btf);
    }
    len = scnprintf(buf, sizeof!(buf),
    "btf_obj_id: %u\nraw_btf_id: %u\ntp_btf_id: %u\n",
    obj_id, raw_id, tp_id);
    return simple_read_from_buffer(ubuf, cnt, ppos, buf, len);
    }

#[no_mangle]
pub unsafe extern "C" fn event_filter_read(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut s: *mut c_void = core::ptr::null_mut();
pub static mut r: c_int = 0;
    if (*ppos) {
    return 0;
    }
    s = kmalloc_obj(*s);
    if (!s) {
    return -ENOMEM;
    }
    trace_seq_init(s);
    mutex_lock(&event_mutex);
    file = event_file_file(filp);
    if (file) {
    print_event_filter(file, s);
    }
    mutex_unlock(&event_mutex);
    if (file) {
    r = simple_read_from_buffer(ubuf, cnt, ppos,
    s.buffer, trace_seq_used(s));
    }
    kfree(s);
    return r;
    }
#[no_mangle]
pub unsafe extern "C" fn event_filter_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut buf: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    if (cnt >= PAGE_SIZE) {
    return -EINVAL;
    }
    buf = memdup_user_nul(ubuf, cnt);
    if (IS_ERR(buf)) {
    return PTR_ERR(buf);
    }
    mutex_lock(&event_mutex);
    file = event_file_file(filp);
    if (file) {
    err = apply_event_filter(file, buf);
    }
    mutex_unlock(&event_mutex);
    kfree(buf);
    if (err < 0) {
    return err;
    }
// ppos += cnt;
    return cnt;
    }
pub static mut event_subsystems: usize = 0;
#[no_mangle]
unsafe extern "C" fn subsystem_open(inode: *mut inode, filp: *mut file) -> c_int {
    let mut dir = core::ptr::null_mut(), *iter_dir;
    let mut tr = core::ptr::null_mut(), *iter_tr;
    let mut system = core::ptr::null_mut();
    let mut ret = 0;
    if (unlikely(tracing_disabled)) {
    return -ENODEV;
    }
// Make sure the system still exists
    mutex_lock(&event_mutex);
    mutex_lock(&trace_types_lock);
    list_for_each_entry(iter_tr, &ftrace_trace_arrays, list) {
    list_for_each_entry(iter_dir, &iter_tr.systems, list) {
    if (iter_dir == inode.i_private) {
// Don't open systems with no events
    tr = iter_tr;
    dir = iter_dir;
    if (dir.nr_events) {
    __get_system_dir(dir);
    system = dir.subsystem;
    }
// goto;
    }
    }
    }
// label;
    mutex_unlock(&trace_types_lock);
    mutex_unlock(&event_mutex);
    if (!system) {
    return -ENODEV;
    }
// Still need to increment the ref count of the system
    if (trace_array_get(tr) < 0) {
    put_system(dir);
    return -ENODEV;
    }
    ret = tracing_open_generic(inode, filp);
    if (ret < 0) {
    trace_array_put(tr);
    put_system(dir);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn system_tr_open(inode: *mut inode, filp: *mut file) -> c_int {
pub static mut dir: *mut c_void = core::ptr::null_mut();
    let mut tr = inode.i_private;
    let mut ret = 0;
// Make a temporary dir that has no system but points to tr
    dir = kzalloc_obj(*dir);
    if (!dir) {
    return -ENOMEM;
    }
    ret = tracing_open_generic_tr(inode, filp);
    if (ret < 0) {
    kfree(dir);
    return ret;
    }
    dir.tr = tr;
    filp.private_data = dir;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn subsystem_release(inode: *mut inode, file: *mut file) -> c_int {
    let mut dir = file.private_data;
    trace_array_put(dir.tr);
//
// If dir->subsystem is NULL, then this is a temporary
// descriptor that was made for a trace_array to enable
// all subsystems.
//
    if (dir.subsystem) {
    put_system(dir);
    }
    else {
    kfree(dir);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn subsystem_filter_read(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut dir = filp.private_data;
    let mut system = dir.subsystem;
pub static mut s: *mut c_void = core::ptr::null_mut();
    let mut r = 0;
    if (*ppos) {
    return 0;
    }
    s = kmalloc_obj(*s);
    if (!s) {
    return -ENOMEM;
    }
    trace_seq_init(s);
    print_subsystem_event_filter(system, s);
    r = simple_read_from_buffer(ubuf, cnt, ppos,
    s.buffer, trace_seq_used(s));
    kfree(s);
    return r;
    }
#[no_mangle]
pub unsafe extern "C" fn subsystem_filter_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut dir = filp.private_data;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (cnt >= PAGE_SIZE) {
    return -EINVAL;
    }
    buf = memdup_user_nul(ubuf, cnt);
    if (IS_ERR(buf)) {
    return PTR_ERR(buf);
    }
    err = apply_subsystem_event_filter(dir, buf);
    kfree(buf);
    if (err < 0) {
    return err;
    }
// ppos += cnt;
    return cnt;
    }
#[no_mangle]
pub unsafe extern "C" fn show_header_page_file(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut tr = filp.private_data;
pub static mut s: *mut c_void = core::ptr::null_mut();
    let mut r = 0;
    if (*ppos) {
    return 0;
    }
    s = kmalloc_obj(*s);
    if (!s) {
    return -ENOMEM;
    }
    trace_seq_init(s);
    ring_buffer_print_page_header(tr.array_buffer.buffer, s);
    r = simple_read_from_buffer(ubuf, cnt, ppos,
    s.buffer, trace_seq_used(s));
    kfree(s);
    return r;
    }
#[no_mangle]
pub unsafe extern "C" fn show_header_event_file(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut s: *mut c_void = core::ptr::null_mut();
    let mut r = 0;
    if (*ppos) {
    return 0;
    }
    s = kmalloc_obj(*s);
    if (!s) {
    return -ENOMEM;
    }
    trace_seq_init(s);
    ring_buffer_print_entry_header(s);
    r = simple_read_from_buffer(ubuf, cnt, ppos,
    s.buffer, trace_seq_used(s));
    kfree(s);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn ignore_task_cpu(data: *mut c_void) {
    let mut tr = data;
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
pub static mut no_pid_list: *mut c_void = core::ptr::null_mut();
//
// This function is called by on_each_cpu() while the
// event_mutex is held.
//
    pid_list = rcu_dereference_protected(tr.filtered_pids,
    mutex_is_locked(&event_mutex));
    no_pid_list = rcu_dereference_protected(tr.filtered_no_pids,
    mutex_is_locked(&event_mutex));
    this_cpu_write(tr.array_buffer.data.ignore_pid,
    trace_ignore_this_task(pid_list, no_pid_list, current));
    }
#[no_mangle]
unsafe extern "C" fn register_pid_events(tr: *mut trace_array) {
//
// Register a probe that is called before all other probes
// to set ignore_pid if next or prev do not match.
// Register a probe this is called after all other probes
// to only keep ignore_pid set if next pid matches.
//
    register_trace_prio_sched_switch(event_filter_pid_sched_switch_probe_pre,
    tr, INT_MAX);
    register_trace_prio_sched_switch(event_filter_pid_sched_switch_probe_post,
    tr, 0);
    register_trace_prio_sched_wakeup(event_filter_pid_sched_wakeup_probe_pre,
    tr, INT_MAX);
    register_trace_prio_sched_wakeup(event_filter_pid_sched_wakeup_probe_post,
    tr, 0);
    register_trace_prio_sched_wakeup_new(event_filter_pid_sched_wakeup_probe_pre,
    tr, INT_MAX);
    register_trace_prio_sched_wakeup_new(event_filter_pid_sched_wakeup_probe_post,
    tr, 0);
    register_trace_prio_sched_waking(event_filter_pid_sched_wakeup_probe_pre,
    tr, INT_MAX);
    register_trace_prio_sched_waking(event_filter_pid_sched_wakeup_probe_post,
    tr, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn event_pid_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t, type: c_int) -> ssize_t {
    let mut m = filp.private_data;
    let mut tr = m.private;
    let mut filtered_pids = core::ptr::null_mut();
    let mut other_pids = core::ptr::null_mut();
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!cnt) {
    return 0;
    }
    ret = tracing_update_buffers(tr);
    if (ret < 0) {
    return ret;
    }
    guard(mutex)(&event_mutex);
    if (type == TRACE_PIDS) {
    filtered_pids = rcu_dereference_protected(tr.filtered_pids,
    lockdep_is_held(&event_mutex));
    other_pids = rcu_dereference_protected(tr.filtered_no_pids,
    lockdep_is_held(&event_mutex));
    } else {
    filtered_pids = rcu_dereference_protected(tr.filtered_no_pids,
    lockdep_is_held(&event_mutex));
    other_pids = rcu_dereference_protected(tr.filtered_pids,
    lockdep_is_held(&event_mutex));
    }
    ret = trace_pid_write(filtered_pids, &pid_list, ubuf, cnt);
    if (ret < 0) {
    return ret;
    }
    if (type == TRACE_PIDS) {
    rcu_assign_pointer(tr.filtered_pids, pid_list);
    }
    else {
    rcu_assign_pointer(tr.filtered_no_pids, pid_list);
    }
    list_for_each_entry(file, &tr.events, list) {
    set_bit(EVENT_FILE_FL_PID_FILTER_BIT, &file.flags);
    }
    if (filtered_pids) {
    tracepoint_synchronize_unregister();
    trace_pid_list_free(filtered_pids);
    } else if (pid_list && !other_pids) {
    register_pid_events(tr);
    }
//
// Ignoring of pids is done at task switch. But we have to
// check for those tasks that are currently running.
// Always do this in case a pid was appended or removed.
//
    on_each_cpu(ignore_task_cpu, tr, 1);
// ppos += ret;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_event_pid_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    return event_pid_write(filp, ubuf, cnt, ppos, TRACE_PIDS);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_event_npid_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    return event_pid_write(filp, ubuf, cnt, ppos, TRACE_NO_PIDS);
    }
// forward_decl: ftrace_event_avail_open;
// forward_decl: ftrace_event_set_open;
// forward_decl: ftrace_event_show_filters_open;
// forward_decl: ftrace_event_show_triggers_open;
// forward_decl: ftrace_event_set_pid_open;
// forward_decl: ftrace_event_set_npid_open;
// forward_decl: ftrace_event_release;
pub static mut seq_operations: usize = 0;
pub static mut seq_operations: usize = 0;
pub static mut seq_operations: usize = 0;
pub static mut seq_operations: usize = 0;
pub static mut seq_operations: usize = 0;
pub static mut seq_operations: usize = 0;
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;

pub static mut file_operations: usize = 0;

pub static mut file_operations: usize = 0;

pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn ftrace_event_open(inode: *mut inode, file: *mut file, seq_ops: *mut seq_operations) -> c_int {
pub static mut m: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ret = security_locked_down(LOCKDOWN_TRACEFS);
    if (ret) {
    return ret;
    }
    ret = seq_open(file, seq_ops);
    if (ret < 0) {
    return ret;
    }
    m = file.private_data;
// copy tr over to seq ops
    m.private = inode.i_private;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ftrace_event_release(inode: *mut inode, file: *mut file) -> c_int {
    let mut tr = inode.i_private;
    trace_array_put(tr);
    return seq_release(inode, file);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_event_avail_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut seq_ops = &show_event_seq_ops;
// Checks for tracefs lockdown
    return ftrace_event_open(inode, file, seq_ops);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_event_set_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut seq_ops = &show_set_event_seq_ops;
    let mut tr = inode.i_private;
    let mut ret = 0;
    ret = tracing_check_open_get_tr(tr);
    if (ret) {
    return ret;
    }
    if ((file.f_mode & FMODE_WRITE) &&
    (file.f_flags & O_TRUNC)) {
    ftrace_clear_events(tr);
    }
    ret = ftrace_event_open(inode, file, seq_ops);
    if (ret < 0) {
    trace_array_put(tr);
    }
    return ret;
    }
//
// ftrace_event_show_filters_open - open interface for set_event_filters
// @inode: The inode of the file
// @file: The file being opened
//
// Connects the set_event_filters file to the sequence operations
// required to iterate over and display active event filters.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_event_show_filters_open(inode: *mut inode, file: *mut file) -> c_int {
    return ftrace_event_open(inode, file, &show_show_event_filters_seq_ops);
    }
//
// ftrace_event_show_triggers_open - open interface for show_event_triggers
// @inode: The inode of the file
// @file: The file being opened
//
// Connects the show_event_triggers file to the sequence operations
// required to iterate over and display active event triggers.
//
#[no_mangle]
pub unsafe extern "C" fn ftrace_event_show_triggers_open(inode: *mut inode, file: *mut file) -> c_int {
    return ftrace_event_open(inode, file, &show_show_event_triggers_seq_ops);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_event_set_pid_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut seq_ops = &show_set_pid_seq_ops;
    let mut tr = inode.i_private;
    let mut ret = 0;
    ret = tracing_check_open_get_tr(tr);
    if (ret) {
    return ret;
    }
    if ((file.f_mode & FMODE_WRITE) &&
    (file.f_flags & O_TRUNC)) {
    ftrace_clear_event_pids(tr, TRACE_PIDS);
    }
    ret = ftrace_event_open(inode, file, seq_ops);
    if (ret < 0) {
    trace_array_put(tr);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_event_set_npid_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut seq_ops = &show_set_no_pid_seq_ops;
    let mut tr = inode.i_private;
    let mut ret = 0;
    ret = tracing_check_open_get_tr(tr);
    if (ret) {
    return ret;
    }
    if ((file.f_mode & FMODE_WRITE) &&
    (file.f_flags & O_TRUNC)) {
    ftrace_clear_event_pids(tr, TRACE_NO_PIDS);
    }
    ret = ftrace_event_open(inode, file, seq_ops);
    if (ret < 0) {
    trace_array_put(tr);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn create_new_subsystem(name: *mut c_char) -> *mut c_void {
pub static mut system: *mut c_void = core::ptr::null_mut();
// need to create new entry
    system = kmalloc_obj(*system);
    if (!system) {
    return core::ptr::null_mut();
    }
    system.ref_count = 1;
// Only allocate if dynamic (kprobes and modules)
    system.name = kstrdup_const(name, GFP_KERNEL);
    if (!system.name) {
// goto;
    }
    system.filter = kzalloc_obj(event_filter);
    if (!system.filter) {
// goto;
    }
    list_add(&system.list, &event_subsystems);
    return system;
// label;
    kfree_const(system.name);
    kfree(system);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn system_callback(name: *mut c_char, mode: *mut umode_t, data: *mut *mut c_void, fops: *mut *mut file_operations) -> c_int {
    if (strcmp(name, "filter") == 0) {
// fops = &ftrace_subsystem_filter_fops;
    }

    else if (strcmp(name, "enable") == 0) {
// fops = &ftrace_system_enable_fops;
    }
    else {
    return 0;
    }
// mode = TRACE_MODE_WRITE;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn event_subsystem_dir(tr: *mut trace_array, name: *mut c_char, file: *mut trace_event_file, parent: *mut eventfs_inode) -> *mut c_void {
    let mut system = core::ptr::null_mut();
    let mut iter = core::ptr::null_mut();
pub static mut dir: *mut c_void = core::ptr::null_mut();
pub static mut ei: *mut c_void = core::ptr::null_mut();
    let mut nr_entries = 0;
pub static mut eventfs_entry: usize = 0;
// First see if we did not already create this dir
    list_for_each_entry(dir, &tr.systems, list) {
    system = dir.subsystem;
    if (strcmp(system.name, name) == 0) {
    dir.nr_events += 1;
    file.system = dir;
    return dir.ei;
    }
    }
// Now see if the system itself exists.
    system = core::ptr::null_mut();
    list_for_each_entry(iter, &event_subsystems, list) {
    if (strcmp(iter.name, name) == 0) {
    system = iter;
    break;
    }
    }
    dir = kmalloc_obj(*dir);
    if (!dir) {
// goto;
    }
    if (!system) {
    system = create_new_subsystem(name);
    if (!system) {
// goto;
    }
    } else {
    __get_system(system);
    }
// ftrace only has directories no files, readonly instance too.
    if (strcmp(name, "ftrace") == 0 || trace_array_is_readonly(tr)) {
    nr_entries = 0;
    }
    else {
    nr_entries = ARRAY_SIZE!(system_entries);
    }
    ei = eventfs_create_dir(name, parent, system_entries, nr_entries, dir);
    if (IS_ERR(ei)) {
    pr_warn!("Failed to create system directory %s\n", name);
    __put_system(system);
// goto;
    }
    dir.ei = ei;
    dir.tr = tr;
    dir.ref_count = 1;
    dir.nr_events = 1;
    dir.subsystem = system;
    file.system = dir;
    list_add(&dir.list, &tr.systems);
    return dir.ei;
// label;
    kfree(dir);
// label;
// Only print this message if failed on memory allocation
    if (!dir || !system) {
    pr_warn!("No memory to create event subsystem %s\n", name);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn event_define_fields(call: *mut trace_event_call) -> c_int {
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
//
// Other events may have the same class. Only update
// the fields if they are not already defined.
//
    head = trace_get_fields(call);
    if (list_empty(head)) {
    let mut field = call.class.fields_array;
pub static mut offset: c_uint = 0;
    while (field.type) {
    if (field.type == TRACE_FUNCTION_TYPE) {
    field.define_fields(call);
    break;
    }
    offset = ALIGN(offset, field.align);
    ret = trace_define_field_ext(call, field.type, field.name,
    offset, field.size,
    field.is_signed, field.filter_type,
    field.len, field.needs_test);
    if (WARN_ON_ONCE!(ret)) {
    pr_err!("error code is %d\n", ret);
    break;
    }
    offset += field.size;
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn event_callback(name: *mut c_char, mode: *mut umode_t, data: *mut *mut c_void, fops: *mut *mut file_operations) -> c_int {
    let mut file = *data;
    let mut call = file.event_call;
    if (strcmp(name, "format") == 0) {
// mode = TRACE_MODE_READ;
// fops = &ftrace_event_format_fops;
    return 1;
    }
//
// Only event directories that can be enabled should have
// triggers or filters, with the exception of the "print"
// event that can have a "trigger" file.
//
    if (!(call.flags & TRACE_EVENT_FL_IGNORE_ENABLE)) {
    if (call.class.reg && strcmp(name, "enable") == 0) {
// mode = TRACE_MODE_WRITE;
// fops = &ftrace_enable_fops;
    return 1;
    }
    if (strcmp(name, "filter") == 0) {
// mode = TRACE_MODE_WRITE;
// fops = &ftrace_event_filter_fops;
    return 1;
    }
    }
    if (!(call.flags & TRACE_EVENT_FL_IGNORE_ENABLE) ||
    strcmp(trace_event_name(call), "print") == 0) {
    if (strcmp(name, "trigger") == 0) {
// mode = TRACE_MODE_WRITE;
// fops = &event_trigger_fops;
    return 1;
    }
    }

    if (call.event.type && call.class.reg &&
    strcmp(name, "id") == 0) {
// mode = TRACE_MODE_READ;
// data = (long)call->event.type;
// fops = &ftrace_event_id_fops;
    return 1;
    }

    if (call.class.btf_ids && strcmp(name, "btf_ids") == 0) {
// mode = TRACE_MODE_READ;
// fops = &ftrace_event_btf_ids_fops;
    return 1;
    }

    if (strcmp(name, "hist") == 0) {
// mode = TRACE_MODE_READ;
// fops = &event_hist_fops;
    return 1;
    }

    if (strcmp(name, "hist_debug") == 0) {
// mode = TRACE_MODE_READ;
// fops = &event_hist_debug_fops;
    return 1;
    }

    if (call.event.type && call.class.reg &&
    strcmp(name, "inject") == 0) {
// mode = 0200;
// fops = &event_inject_fops;
    return 1;
    }

    return 0;
    }
// The file is incremented on creation and freeing the enable file decrements it
#[no_mangle]
unsafe extern "C" fn event_release(name: *const c_char, data: *mut c_void) {
    let mut file = data;
    event_file_put(file);
    }
#[no_mangle]
pub unsafe extern "C" fn event_create_dir(parent: *mut eventfs_inode, file: *mut trace_event_file) -> c_int {
    let mut call = file.event_call;
    let mut tr = file.tr;
pub static mut e_events: *mut c_void = core::ptr::null_mut();
pub static mut ei: *mut c_void = core::ptr::null_mut();
pub static mut name: *mut c_void = core::ptr::null_mut();
    let mut nr_entries = 0;
    let mut ret = 0;
pub static mut eventfs_entry: usize = 0;
//
// If the trace point header did not define TRACE_SYSTEM
// then the system would be called "TRACE_SYSTEM". This should
// never happen.
//
    if (WARN_ON_ONCE!(strcmp(call.class.system, TRACE_SYSTEM) == 0)) {
    return -ENODEV;
    }
    ret = event_define_fields(call);
    if (ret < 0) {
    pr_warn!("Could not initialize trace point events/%s\n",
    trace_event_name(call));
    return ret;
    }
    e_events = event_subsystem_dir(tr, call.class.system, file, parent);
    if (!e_events) {
    return -ENOMEM;
    }
    if (trace_array_is_readonly(tr)) {
    nr_entries = NR_RO_EVENT_ENTRIES;
    }
    else {
    nr_entries = ARRAY_SIZE!(event_entries);
    }
    name = trace_event_name(call);
    ei = eventfs_create_dir(name, e_events, event_entries, nr_entries, file);
    if (IS_ERR(ei)) {
    pr_warn!("Could not create tracefs '%s' directory\n", name);
    return -1;
    }
    file.ei = ei;
// Gets decremented on freeing of the "enable" file
    event_file_get(file);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn remove_event_from_tracers(call: *mut trace_event_call) {
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut tr: *mut c_void = core::ptr::null_mut();
    do_for_each_event_file_safe(tr, file) {
    if (file.event_call != call) {
    continue;
    }
    remove_event_file_dir(file);
//
// The do_for_each_event_file_safe() is
// a double loop. After finding the call for this
// trace_array, we use break to jump to the next
// trace_array.
//
    break;
    } while_for_each_event_file();
    }
#[no_mangle]
unsafe extern "C" fn event_remove(call: *mut trace_event_call) {
pub static mut tr: *mut c_void = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
    do_for_each_event_file(tr, file) {
    if (file.event_call != call) {
    continue;
    }
    if (file.flags & EVENT_FILE_FL_WAS_ENABLED) {
    tr.clear_trace = true;
    }
    ftrace_event_enable_disable(file, 0);
//
// The do_for_each_event_file() is
// a double loop. After finding the call for this
// trace_array, we use break to jump to the next
// trace_array.
//
    break;
    } while_for_each_event_file();
    if (call.event.funcs) {
    __unregister_trace_event(&call.event);
    }
    remove_event_from_tracers(call);
    list_del(&call.list);
    }
#[no_mangle]
unsafe extern "C" fn event_init(call: *mut trace_event_call) -> c_int {
pub static mut ret: c_int = 0;
pub static mut name: *mut c_void = core::ptr::null_mut();
    name = trace_event_name(call);
    if (WARN_ON!(!name)) {
    return -EINVAL;
    }
    if (call.class.raw_init) {
    ret = call.class.raw_init(call);
    if (ret < 0 && ret != -ENOSYS) {
    pr_warn!("Could not initialize trace events/%s\n", name);
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __register_event(call: *mut trace_event_call, mod: *mut module) -> c_int {
    let mut ret = 0;
    ret = event_init(call);
    if (ret < 0) {
    return ret;
    }
    down_write(&trace_event_sem);
    list_add(&call.list, &ftrace_events);
    up_write(&trace_event_sem);
    if (call.flags & TRACE_EVENT_FL_DYNAMIC) {
    atomic_set(&call.refcnt, 0);
    }
    else {
    call.module = mod;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn eval_replace(ptr: *mut c_char, map: *mut trace_eval_map, len: c_int) -> *mut c_void {
    let mut rlen = 0;
    let mut elen = 0;
// Find the length of the eval value as a string
    elen = snprintf(ptr, 0, "%ld", map.eval_value);
// Make sure there's enough room to replace the string with the value
    if (len < elen) {
    return core::ptr::null_mut();
    }
    snprintf(ptr, elen + 1, "%ld", map.eval_value);
// Get the rest of the string of ptr
    rlen = strlen(ptr + len);
    memmove(ptr + elen, ptr + len, rlen);
// Make sure we end the new string
    ptr[elen + rlen] = 0;
    return ptr + elen;
    }
#[no_mangle]
pub unsafe extern "C" fn update_event_printk(call: *mut trace_event_call, map: *mut trace_eval_map) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut quote: c_int = 0;
pub static mut len: c_int = 0;
    while (*ptr) {
    if (*ptr == '\\') {
    ptr += 1;
// paranoid
    if (!*ptr) {
    break;
    }
    continue;
    }
    if (*ptr == '"') {
    quote ^= 1;
    continue;
    }
    if (quote) {
    continue;
    }
    if (isdigit(*ptr)) {
// skip numbers
    do {
    ptr += 1;
// Check for alpha chars like ULL
    } while (isalnum(*ptr));
    if (!*ptr) {
    break;
    }
//
// A number must have some kind of delimiter after
// it, and we can ignore that too.
//
    continue;
    }
    if (isalpha(*ptr) || *ptr == '_') {
    if (strncmp(map.eval_string, ptr, len) == 0 &&
    !isalnum(ptr[len]) && ptr[len] != '_') {
    ptr = eval_replace(ptr, map, len);
// enum/sizeof string smaller than value
    if (WARN_ON_ONCE!(!ptr)) {
    return;
    }
//
// No need to decrement here, as eval_replace()
// returns the pointer to the character passed
// the eval, and two evals can not be placed
// back to back without something in between.
// We can skip that something in between.
//
    continue;
    }
// label;
    do {
    ptr += 1;
    } while (isalnum(*ptr) || *ptr == '_');
    if (!*ptr) {
    break;
    }
//
// If what comes after this variable is a '.' or
// '->' then we can continue to ignore that string.
//
    if (*ptr == '.' || (ptr[0] == '-' && ptr[1] == '>')) {
    ptr += *ptr == '.' ? 1 : 2;
    if (!*ptr) {
    break;
    }
// goto;
    }
//
// Once again, we can skip the delimiter that came
// after the string.
//
    continue;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn add_str_to_module(module: *mut module, str: *mut c_char) {
pub static mut modstr: *mut c_void = core::ptr::null_mut();
    modstr = kmalloc_obj(*modstr);
//
// If we failed to allocate memory here, then we'll just
// let the str memory leak when the module is removed.
// If this fails to allocate, there's worse problems than
// a leaked string on module removal.
//
    if (WARN_ON_ONCE!(!modstr)) {
    return;
    }
    modstr.module = module;
    modstr.str = str;
    list_add(&modstr.next, &module_strings);
    }

// Remove all __attribute__() from @type. Return allocated string or @type.
#[no_mangle]
pub unsafe extern "C" fn sanitize_field_type(type: *mut c_char) -> *mut c_void {
    char *attr, *tmp, *next, *ret = type;
    let mut depth = 0;
    next = type;
    while ((attr = strstr(next, ATTRIBUTE_STR))) {
// Retry if "__attribute__(" is a part of another word.
    if (attr != next && !isspace(attr[-1])) {
    next = attr + ATTRIBUTE_STR_LEN;
    continue;
    }
    if (ret == type) {
    ret = kstrdup(type, GFP_KERNEL);
    if (WARN_ON_ONCE!(!ret)) {
    return core::ptr::null_mut();
    }
    attr = ret + (attr - type);
    }
// the ATTRIBUTE_STR already has the first '('
    depth = 1;
    next = attr + ATTRIBUTE_STR_LEN;
    do {
    tmp = strpbrk(next, "()");
// There is unbalanced parentheses
    if (WARN_ON_ONCE!(!tmp)) {
    kfree(ret);
    return type;
    }
    if (*tmp == '(')
    depth += 1;
    else {
    depth -= 1;
    }
    next = tmp + 1;
    } while (depth > 0);
    next = skip_spaces(next);
    strcpy(attr, next);
    next = attr;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn find_replacable_eval(type: *mut c_char, eval_string: *mut c_char, len: c_int) -> *mut c_void {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    if (!eval_string) {
    return core::ptr::null_mut();
    }
    ptr = strchr(type, '[');
    if (!ptr) {
    return core::ptr::null_mut();
    }
    ptr += 1;
    if (!isalpha(*ptr) && *ptr != '_') {
    return core::ptr::null_mut();
    }
    if (strncmp(eval_string, ptr, len) != 0) {
    return core::ptr::null_mut();
    }
    return ptr;
    }
#[no_mangle]
pub unsafe extern "C" fn update_event_fields(call: *mut trace_event_call, map: *mut trace_eval_map) {
pub static mut field: *mut c_void = core::ptr::null_mut();
    let mut eval_string = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut len: c_int = 0;
pub static mut ptr: *mut c_void = core::ptr::null_mut();
pub static mut str: *mut c_void = core::ptr::null_mut();
// Dynamic events should never have field maps
    if (call.flags & TRACE_EVENT_FL_DYNAMIC) {
    return;
    }
    if (map) {
    eval_string = map.eval_string;
    len = strlen(map.eval_string);
    }
    head = trace_get_fields(call);
    list_for_each_entry(field, head, link) {
    str = sanitize_field_type(field.type);
    if (!str) {
    return;
    }
    ptr = find_replacable_eval(str, eval_string, len);
    if (ptr) {
    if (str == field.type) {
    str = kstrdup(field.type, GFP_KERNEL);
    if (WARN_ON_ONCE!(!str)) {
    return;
    }
    ptr = str + (ptr - field.type);
    }
    ptr = eval_replace(ptr, map, len);
// enum/sizeof string smaller than value
    if (WARN_ON_ONCE!(!ptr)) {
    kfree(str);
    continue;
    }
    }
    if (str == field.type) {
    continue;
    }
//
// If the event is part of a module, then we need to free the string
// when the module is removed. Otherwise, it will stay allocated
// until a reboot.
//
    if (call.module) {
    add_str_to_module(call.module, str);
    }
    field.type = str;
    if (field.filter_type == FILTER_OTHER) {
    field.filter_type = filter_assign_type(field.type);
    }
    }
    }
// Update all events for replacing eval and sanitizing
#[no_mangle]
pub unsafe extern "C" fn trace_event_update_all(map: *mut trace_eval_map, len: c_int, mod: *mut module) {
    let mut call = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut last_system = core::ptr::null_mut();
pub static mut first: bool = false;
    let mut updated = 0;
    let mut last_i = 0;
    let mut i = 0;
    mutex_lock(&event_mutex);
    down_write(&trace_event_sem);
    list_for_each_entry_safe(call, p, &ftrace_events, list) {
    if (mod && call.module != mod) {
    continue;
    }
// events are usually grouped together with systems
    if (!last_system || call.class.system != last_system) {
    first = true;
    last_i = 0;
    last_system = call.class.system;
    }
    updated = false;
//
// Since calls are grouped by systems, the likelihood that the
// next call in the iteration belongs to the same system as the
// previous call is high. As an optimization, we skip searching
// for a map[] that matches the call's system if the last call
// was from the same system. That's what last_i is for. If the
// call has the same system as the previous call, then last_i
// will be the index of the first map[] that has a matching
// system.
//
    while (i < len) {
    if (call.class.system == map[i].system) {
// Save the first system if need be
    if (first) {
    last_i = i;
    first = false;
    }
    update_event_printk(call, map[i]);
    update_event_fields(call, map[i]);
    updated = true;
    }
    }
// If not updated yet, update field for sanitizing.
    if (!updated) {
    update_event_fields(call, core::ptr::null_mut());
    }
    cond_resched();
    }
    up_write(&trace_event_sem);
    mutex_unlock(&event_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn event_in_systems(call: *mut trace_event_call, systems: *mut c_char) -> bool {
pub static mut system: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (!systems) {
    return true;
    }
    system = call.class.system;
    p = strstr(systems, system);
    if (!p) {
    return false;
    }
    if (p != systems && !isspace(*(p - 1)) && *(p - 1) != ',') {
    return false;
    }
    p += strlen(system);
    return !*p || isspace(*p) || *p == ',';
    }

//
// Wake up waiter on the hist_poll_wq from irq_work because the hist trigger
// may happen in any context.
//
#[no_mangle]
unsafe extern "C" fn hist_poll_event_irq_work(work: *mut irq_work) {
    wake_up_all(&hist_poll_wq);
    }
pub static mut hist_poll_work: usize = 0;
pub static mut hist_poll_wq: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn trace_create_new_event(call: *mut trace_event_call, tr: *mut trace_array) -> *mut c_void {
pub static mut no_pid_list: *mut c_void = core::ptr::null_mut();
pub static mut pid_list: *mut c_void = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut first = 0;
    if (!event_in_systems(call, tr.system_names)) {
    return core::ptr::null_mut();
    }
    file = kmem_cache_alloc(file_cachep, GFP_TRACE);
    if (!file) {
    return ERR_PTR(-ENOMEM);
    }
    pid_list = rcu_dereference_protected(tr.filtered_pids,
    lockdep_is_held(&event_mutex));
    no_pid_list = rcu_dereference_protected(tr.filtered_no_pids,
    lockdep_is_held(&event_mutex));
    if (!trace_pid_list_first(pid_list, &first) ||
    !trace_pid_list_first(no_pid_list, &first)) {
    file.flags |= EVENT_FILE_FL_PID_FILTER;
    }
    file.event_call = call;
    file.tr = tr;
    atomic_set(&file.sm_ref, 0);
    atomic_set(&file.tm_ref, 0);
    INIT_LIST_HEAD(&file.triggers);
    list_add(&file.list, &tr.events);
    refcount_set(&file.ref, 1);
    return file;
    }
pub const MAX_BOOT_TRIGGERS: c_int = 32;
    static struct boot_triggers {
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut trigger: *mut c_void = core::ptr::null_mut();
    } bootup_triggers[MAX_BOOT_TRIGGERS];
    static char bootup_trigger_buf[COMMAND_LINE_SIZE];
    static int boot_trigger_buf_len;
    static int nr_boot_triggers;
#[no_mangle]
unsafe extern "C" fn setup_trace_triggers(str: *mut c_char) -> __init int {
pub static mut trigger: *mut c_void = core::ptr::null_mut();
pub static mut buf: *mut c_void = core::ptr::null_mut();
pub static mut len: c_int = 0;
    let mut i = 0;
    if (len >= COMMAND_LINE_SIZE) {
    return 1;
    }
    strscpy(bootup_trigger_buf + len, str, COMMAND_LINE_SIZE - len);
    trace_set_ring_buffer_expanded(core::ptr::null_mut());
    disable_tracing_selftest("running event triggers");
    buf = bootup_trigger_buf + len;
    boot_trigger_buf_len += strlen(buf) + 1;
    while (i < MAX_BOOT_TRIGGERS) {
    trigger = strsep(&buf, ",");
    if (!trigger) {
    break;
    }
    bootup_triggers[i].event = strsep(&trigger, ".");
    bootup_triggers[i].trigger = trigger;
    if (!bootup_triggers[i].trigger) {
    break;
    }
    }
    nr_boot_triggers = i;
    return 1;
    }
    __setup!("trace_trigger=", setup_trace_triggers);
// Add an event to a trace directory
#[no_mangle]
pub unsafe extern "C" fn __trace_add_new_event(call: *mut trace_event_call, tr: *mut trace_array) -> c_int {
pub static mut file: *mut c_void = core::ptr::null_mut();
    file = trace_create_new_event(call, tr);
//
// trace_create_new_event() returns ERR_PTR(-ENOMEM) if failed
// allocation, or NULL if the event is not part of the tr->system_names.
// When the event is not part of the tr->system_names, return zero, not
// an error.
//
    if (!file) {
    return 0;
    }
    if (IS_ERR(file)) {
    return PTR_ERR(file);
    }
    if (eventdir_initialized) {
    return event_create_dir(tr.event_dir, file);
    }
    else {
    return event_define_fields(call);
    }
    }
#[no_mangle]
unsafe extern "C" fn trace_early_triggers(file: *mut trace_event_file, name: *const c_char) {
    let mut ret = 0;
    let mut i = 0;
    while (i < nr_boot_triggers) {
    if (strcmp(name, bootup_triggers[i].event)) {
    continue;
    }
    mutex_lock(&event_mutex);
    ret = trigger_process_regex(file, bootup_triggers[i].trigger);
    mutex_unlock(&event_mutex);
    if (ret) {
    pr_err!("Failed to register trigger '%s' on event %s\n",
    bootup_triggers[i].trigger,
    bootup_triggers[i].event);
    }
    }
    }
//
// Just create a descriptor for early init. A descriptor is required
// for enabling events at boot. We want to enable events before
// the filesystem is initialized.
//
#[no_mangle]
pub unsafe extern "C" fn __trace_early_add_new_event(call: *mut trace_event_call, tr: *mut trace_array) -> c_int {
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    file = trace_create_new_event(call, tr);
//
// trace_create_new_event() returns ERR_PTR(-ENOMEM) if failed
// allocation, or NULL if the event is not part of the tr->system_names.
// When the event is not part of the tr->system_names, return zero, not
// an error.
//
    if (!file) {
    return 0;
    }
    if (IS_ERR(file)) {
    return PTR_ERR(file);
    }
    ret = event_define_fields(call);
    if (ret) {
    return ret;
    }
    trace_early_triggers(file, trace_event_name(call));
    return 0;
    }
    let mut ftrace_module_file_ops;
// forward_decl: __add_event_to_tracers;
// Add an additional event_call dynamically
#[no_mangle]
pub unsafe extern "C" fn trace_add_event_call(call: *mut trace_event_call) -> c_int {
    let mut ret = 0;
    lockdep_assert_held(&event_mutex);
    guard(mutex)(&trace_types_lock);
    ret = __register_event(call, core::ptr::null_mut());
    if (ret < 0) {
    return ret;
    }
    __add_event_to_tracers(call);
    return ret;
    }
    EXPORT_SYMBOL_GPL(trace_add_event_call);
//
// Must be called under locking of trace_types_lock, event_mutex and
// trace_event_sem.
//
#[no_mangle]
unsafe extern "C" fn __trace_remove_event_call(call: *mut trace_event_call) {
    event_remove(call);
    trace_destroy_fields(call);
    }
#[no_mangle]
unsafe extern "C" fn probe_remove_event_call(call: *mut trace_event_call) -> c_int {
pub static mut tr: *mut c_void = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();

    if (call.perf_refcount) {
    return -EBUSY;
    }

    do_for_each_event_file(tr, file) {
    if (file.event_call != call) {
    continue;
    }
//
// We can't rely on ftrace_event_enable_disable(enable => 0)
// we are going to do, soft mode can suppress
// TRACE_REG_UNREGISTER.
//
    if (file.flags & EVENT_FILE_FL_ENABLED) {
// goto;
    }
    if (file.flags & EVENT_FILE_FL_WAS_ENABLED) {
    tr.clear_trace = true;
    }
//
// The do_for_each_event_file_safe() is
// a double loop. After finding the call for this
// trace_array, we use break to jump to the next
// trace_array.
//
    break;
    } while_for_each_event_file();
    __trace_remove_event_call(call);
    return 0;
// label;
// No need to clear the trace now
    list_for_each_entry(tr, &ftrace_trace_arrays, list) {
    tr.clear_trace = false;
    }
    return -EBUSY;
    }
// Remove an event_call
#[no_mangle]
pub unsafe extern "C" fn trace_remove_event_call(call: *mut trace_event_call) -> c_int {
    let mut ret = 0;
    lockdep_assert_held(&event_mutex);
    mutex_lock(&trace_types_lock);
    down_write(&trace_event_sem);
    ret = probe_remove_event_call(call);
    up_write(&trace_event_sem);
    mutex_unlock(&trace_types_lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(trace_remove_event_call);

    for (event = start;					
    (unsigned long)event < (unsigned long)end;		
    event++) {

#[no_mangle]
unsafe extern "C" fn update_mod_cache(tr: *mut trace_array, mod: *mut module) {
    }
    let mut event_mod = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
    list_for_each_entry_safe(event_mod, n, &tr.mod_events, list) {
    if (strcmp(event_mod.module, mod.name) != 0) {
    continue;
    }
    __ftrace_set_clr_event_nolock(tr, event_mod.match,
    event_mod.system,
    event_mod.event, 1, mod.name);
    free_event_mod(event_mod);
    }
    }
#[no_mangle]
unsafe extern "C" fn update_cache_events(mod: *mut module) {
pub static mut tr: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(tr, &ftrace_trace_arrays, list) {
    update_mod_cache(tr, mod);
    }
    }
#[no_mangle]
unsafe extern "C" fn trace_module_add_events(mod: *mut module) {
    let mut call = core::ptr::null_mut();
    let mut start = core::ptr::null_mut();
    let mut end = core::ptr::null_mut();
    if (!mod.num_trace_events) {
    return;
    }
// Don't add infrastructure for mods without tracepoints
    if (trace_module_has_bad_taint(mod)) {
    pr_err!("%s: module has bad taint, not creating trace events\n",
    mod.name);
    return;
    }
    start = mod.trace_events;
    end = mod.trace_events + mod.num_trace_events;
    for_each_event(call, start, end) {
    if (!__register_event(*call, mod)) {
    __add_event_to_tracers(*call);
    }
    }
    update_cache_events(mod);
    }
#[no_mangle]
unsafe extern "C" fn trace_module_remove_events(mod: *mut module) {
    let mut call = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut modstr = core::ptr::null_mut();
    let mut m = core::ptr::null_mut();
    down_write(&trace_event_sem);
    list_for_each_entry_safe(call, p, &ftrace_events, list) {
    if ((call.flags & TRACE_EVENT_FL_DYNAMIC) || !call.module) {
    continue;
    }
    if (call.module == mod) {
    __trace_remove_event_call(call);
    }
    }
// Check for any strings allocated for this module
    list_for_each_entry_safe(modstr, m, &module_strings, next) {
    if (modstr.module != mod) {
    continue;
    }
    list_del(&modstr.next);
    kfree(modstr.str);
    kfree(modstr);
    }
    up_write(&trace_event_sem);
//
// It is safest to reset the ring buffer if the module being unloaded
// registered any events that were used. The only worry is if
// a new module gets loaded, and takes on the same id as the events
// of this module. When printing out the buffer, traced events left
// over from this module may be passed to the new module events and
// unexpected results may occur.
//
    tracing_reset_all_online_cpus_unlocked();
    }
#[no_mangle]
pub unsafe extern "C" fn trace_module_notify(self: *mut notifier_block, val: c_ulong, data: *mut c_void) -> c_int {
    let mut mod = data;
    mutex_lock(&event_mutex);
    mutex_lock(&trace_types_lock);
    match (val) {
    MODULE_STATE_COMING => {
    trace_module_add_events(mod);
    // break;
    }
    MODULE_STATE_GOING => {
    trace_module_remove_events(mod);
    // break;
    }
    }
    mutex_unlock(&trace_types_lock);
    mutex_unlock(&event_mutex);
    return NOTIFY_OK;
    }
pub static mut notifier_block: usize = 0;

// Create a new event directory structure for a trace directory.
#[no_mangle]
pub unsafe extern "C" fn __trace_add_event_dirs(tr: *mut trace_array) {
pub static mut call: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    lockdep_assert_held(&trace_event_sem);
    list_for_each_entry(call, &ftrace_events, list) {
    ret = __trace_add_new_event(call, tr);
    if (ret < 0) {
    pr_warn!("Could not create directory for event %s\n",
    trace_event_name(call));
    }
    }
    }
// Returns any file that matches the system and event
#[no_mangle]
pub unsafe extern "C" fn __find_event_file(tr: *mut trace_array, system: *mut c_char, event: *mut c_char) -> *mut c_void {
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut call: *mut c_void = core::ptr::null_mut();
pub static mut name: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(file, &tr.events, list) {
    call = file.event_call;
    name = trace_event_name(call);
    if (!name || !call.class) {
    continue;
    }
    if (strcmp(event, name) == 0 &&
    strcmp(system, call.class.system) == 0) {
    return file;
    }
    }
    return core::ptr::null_mut();
    }
// Returns valid trace event files that match system and event
#[no_mangle]
pub unsafe extern "C" fn find_event_file(tr: *mut trace_array, system: *mut c_char, event: *mut c_char) -> *mut c_void {
pub static mut file: *mut c_void = core::ptr::null_mut();
    file = __find_event_file(tr, system, event);
    if (!file || !file.event_call.class.reg ||
    file.event_call.flags & TRACE_EVENT_FL_IGNORE_ENABLE) {
    return core::ptr::null_mut();
    }
    return file;
    }
//
// trace_get_event_file - Find and return a trace event file
// @instance: The name of the trace instance containing the event
// @system: The name of the system containing the event
// @event: The name of the event
//
// Return a trace event file given the trace instance name, trace
// system, and trace event name.  If the instance name is NULL, it
// refers to the top-level trace array.
//
// This function will look it up and return it if found, after calling
// trace_array_get() to prevent the instance from going away, and
// increment the event's module refcount to prevent it from being
// removed.
//
// To release the file, call trace_put_event_file(), which will call
// trace_array_put() and decrement the event's module refcount.
//
// Return: The trace event on success, ERR_PTR otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn trace_get_event_file(instance: *mut c_char, system: *mut c_char, event: *mut c_char) -> *mut c_void {
    let mut tr = top_trace_array();
    let mut file = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (instance) {
    tr = trace_array_find_get(instance);
    if (!tr) {
    return ERR_PTR(-ENOENT);
    }
    } else {
    ret = trace_array_get(tr);
    if (ret) {
    return ERR_PTR(ret);
    }
    }
    guard(mutex)(&event_mutex);
    file = find_event_file(tr, system, event);
    if (!file) {
    trace_array_put(tr);
    return ERR_PTR(-EINVAL);
    }
// Don't let event modules unload while in use
    ret = trace_event_try_get_ref(file.event_call);
    if (!ret) {
    trace_array_put(tr);
    return ERR_PTR(-EBUSY);
    }
    return file;
    }
    EXPORT_SYMBOL_GPL(trace_get_event_file);
//
// trace_put_event_file - Release a file from trace_get_event_file()
// @file: The trace event file
//
// If a file was retrieved using trace_get_event_file(), this should
// be called when it's no longer needed.  It will cancel the previous
// trace_array_get() called by that function, and decrement the
// event's module refcount.
//
#[no_mangle]
pub unsafe extern "C" fn trace_put_event_file(file: *mut trace_event_file) {
    mutex_lock(&event_mutex);
    trace_event_put_ref(file.event_call);
    mutex_unlock(&event_mutex);
    trace_array_put(file.tr);
    }
    EXPORT_SYMBOL_GPL(trace_put_event_file);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_probe_data {
    pub file: *mut trace_event_file,
    pub count: c_ulong,
    pub ref: c_int,
    pub enable: bool,
}

#[no_mangle]
unsafe extern "C" fn update_event_probe(data: *mut event_probe_data) {
    if (data.enable) {
    clear_bit(EVENT_FILE_FL_SOFT_DISABLED_BIT, &data.file.flags);
    }
    else {
    set_bit(EVENT_FILE_FL_SOFT_DISABLED_BIT, &data.file.flags);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn event_enable_probe(ip: c_ulong, parent_ip: c_ulong, tr: *mut trace_array, ops: *mut ftrace_probe_ops, data: *mut c_void) {
    let mut mapper = data;
pub static mut edata: *mut c_void = core::ptr::null_mut();
pub static mut pdata: *mut c_void = core::ptr::null_mut();
    pdata = ftrace_func_mapper_find_ip(mapper, ip);
    if (!pdata || !*pdata) {
    return;
    }
    edata = *pdata;
    update_event_probe(edata);
    }
#[no_mangle]
pub unsafe extern "C" fn event_enable_count_probe(ip: c_ulong, parent_ip: c_ulong, tr: *mut trace_array, ops: *mut ftrace_probe_ops, data: *mut c_void) {
    let mut mapper = data;
pub static mut edata: *mut c_void = core::ptr::null_mut();
pub static mut pdata: *mut c_void = core::ptr::null_mut();
    pdata = ftrace_func_mapper_find_ip(mapper, ip);
    if (!pdata || !*pdata) {
    return;
    }
    edata = *pdata;
    if (!edata.count) {
    return;
    }
// Skip if the event is in a state we want to switch to
    if (edata.enable == !(edata.file.flags & EVENT_FILE_FL_SOFT_DISABLED)) {
    return;
    }
    if (edata.count != -1) {
    (edata.count)--;
    }
    update_event_probe(edata);
    }
#[no_mangle]
pub unsafe extern "C" fn event_enable_print(m: *mut seq_file, ip: c_ulong, ops: *mut ftrace_probe_ops, data: *mut c_void) -> c_int {
    let mut mapper = data;
pub static mut edata: *mut c_void = core::ptr::null_mut();
pub static mut pdata: *mut c_void = core::ptr::null_mut();
    pdata = ftrace_func_mapper_find_ip(mapper, ip);
    if (WARN_ON_ONCE!(!pdata || !*pdata)) {
    return 0;
    }
    edata = *pdata;
    seq_printf(m, "%ps:", ip);
    seq_printf(m, "%s:%s:%s",
    edata.enable ? ENABLE_EVENT_STR : DISABLE_EVENT_STR,
    edata.file.event_call.class.system,
    trace_event_name(edata.file.event_call));
    if (edata.count == -1) {
    seq_puts(m, ":unlimited\n");
    }
    else {
    seq_printf(m, ":count=%ld\n", edata.count);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn event_enable_init(ops: *mut ftrace_probe_ops, tr: *mut trace_array, ip: c_ulong, init_data: *mut c_void, data: *mut *mut c_void) -> c_int {
    let mut mapper = *data;
    let mut edata = init_data;
    let mut ret = 0;
    if (!mapper) {
    mapper = allocate_ftrace_func_mapper();
    if (!mapper) {
    return -ENODEV;
    }
// data = mapper;
    }
    ret = ftrace_func_mapper_add_ip(mapper, ip, edata);
    if (ret < 0) {
    return ret;
    }
    edata.ref += 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_probe_data(data: *mut c_void) -> c_int {
    let mut edata = data;
    edata.ref -= 1;
    if (!edata.ref) {
// Remove soft mode
    __ftrace_event_enable_disable(edata.file, 0, 1);
    trace_event_put_ref(edata.file.event_call);
    kfree(edata);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn event_enable_free(ops: *mut ftrace_probe_ops, tr: *mut trace_array, ip: c_ulong, data: *mut c_void) {
    let mut mapper = data;
pub static mut edata: *mut c_void = core::ptr::null_mut();
    if (!ip) {
    if (!mapper) {
    return;
    }
    free_ftrace_func_mapper(mapper, free_probe_data);
    return;
    }
    edata = ftrace_func_mapper_remove_ip(mapper, ip);
    if (WARN_ON_ONCE!(!edata)) {
    return;
    }
    if (WARN_ON_ONCE!(edata.ref <= 0)) {
    return;
    }
    free_probe_data(edata);
    }
pub static mut ftrace_probe_ops: usize = 0;
pub static mut ftrace_probe_ops: usize = 0;
pub static mut ftrace_probe_ops: usize = 0;
pub static mut ftrace_probe_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn event_enable_func(tr: *mut trace_array, hash: *mut ftrace_hash, glob: *mut c_char, cmd: *mut c_char, param: *mut c_char, enabled: c_int) -> c_int {
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut ops: *mut c_void = core::ptr::null_mut();
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut count: c_ulong = 0;
pub static mut system: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut number: *mut c_void = core::ptr::null_mut();
    let mut enable = 0;
    let mut ret = 0;
    if (!tr) {
    return -ENODEV;
    }
// hash funcs only work with set_ftrace_filter
    if (!enabled || !param) {
    return -EINVAL;
    }
    system = strsep(&param, ":");
    if (!param) {
    return -EINVAL;
    }
    event = strsep(&param, ":");
    guard(mutex)(&event_mutex);
    file = find_event_file(tr, system, event);
    if (!file) {
    return -EINVAL;
    }
    enable = strcmp(cmd, ENABLE_EVENT_STR) == 0;
    if (enable) {
    ops = param ? &event_enable_count_probe_ops : &event_enable_probe_ops;
    }
    else {
    ops = param ? &event_disable_count_probe_ops : &event_disable_probe_ops;
    }
    if (glob[0] == '!') {
    return unregister_ftrace_function_probe_func(glob+1, tr, ops);
    }
    if (param) {
    number = strsep(&param, ":");
    if (!strlen(number)) {
    return -EINVAL;
    }
//
// We use the callback data field (which is a pointer)
// as our counter.
//
    ret = kstrtoul(number, 0, &count);
    if (ret) {
    return ret;
    }
    }
// Don't let event modules unload while probe registered
    ret = trace_event_try_get_ref(file.event_call);
    if (!ret) {
    return -EBUSY;
    }
    ret = __ftrace_event_enable_disable(file, 1, 1);
    if (ret < 0) {
// goto;
    }
    ret = -ENOMEM;
    data = kzalloc_obj(*data);
    if (!data) {
// goto;
    }
    data.enable = enable;
    data.count = count;
    data.file = file;
    ret = register_ftrace_function_probe(glob, tr, ops, data);
//
// The above returns on success the # of functions enabled,
// but if it didn't find any functions it returns zero.
// Consider no functions a failure too.
//
// Just return zero, not the number of enabled functions
    if (ret > 0) {
    return 0;
    }
    kfree(data);
    if (!ret) {
    ret = -ENOENT;
    }
    __ftrace_event_enable_disable(file, 0, 1);
// label;
    trace_event_put_ref(file.event_call);
    return ret;
    }
pub static mut ftrace_func_command: usize = 0;
pub static mut ftrace_func_command: usize = 0;
#[no_mangle]
unsafe extern "C" fn register_event_cmds() -> __init int {
    let mut ret = 0;
    ret = register_ftrace_command(&event_enable_cmd);
    if (WARN_ON!(ret < 0)) {
    return ret;
    }
    ret = register_ftrace_command(&event_disable_cmd);
    if (WARN_ON!(ret < 0)) {
    unregister_ftrace_command(&event_enable_cmd);
    }
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn register_event_cmds() -> c_int { return 0; }

//
// The top level array and trace arrays created by boot-time tracing
// have already had its trace_event_file descriptors created in order
// to allow for early events to be recorded.
// This function is called after the tracefs has been initialized,
// and we now have to create the files associated to the events.
//
#[no_mangle]
unsafe extern "C" fn __trace_early_add_event_dirs(tr: *mut trace_array) {
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    list_for_each_entry(file, &tr.events, list) {
    ret = event_create_dir(tr.event_dir, file);
    if (ret < 0) {
    pr_warn!("Could not create directory for event %s\n",
    trace_event_name(file.event_call));
    }
    }
    }
//
// For early boot up, the top trace array and the trace arrays created
// by boot-time tracing require to have a list of events that can be
// enabled. This must be done before the filesystem is set up in order
// to allow events to be traced early.
//
#[no_mangle]
pub unsafe extern "C" fn __trace_early_add_events(tr: *mut trace_array) {
pub static mut call: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    list_for_each_entry(call, &ftrace_events, list) {
// Early boot up should not have any modules loaded
    if (!(call.flags & TRACE_EVENT_FL_DYNAMIC) &&
    WARN_ON_ONCE!(call.module)) {
    continue;
    }
    ret = __trace_early_add_new_event(call, tr);
    if (ret < 0) {
    pr_warn!("Could not create early event %s\n",
    trace_event_name(call));
    }
    }
    }
// Remove the event directory structure for a trace directory.
#[no_mangle]
pub unsafe extern "C" fn __trace_remove_event_dirs(tr: *mut trace_array) {
    let mut file = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    list_for_each_entry_safe(file, next, &tr.events, list) {
    remove_event_file_dir(file);
    }
    }
#[no_mangle]
unsafe extern "C" fn __add_event_to_tracers(call: *mut trace_event_call) {
pub static mut tr: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(tr, &ftrace_trace_arrays, list) {
    __trace_add_new_event(call, tr);
    }
    }
    extern struct trace_event_call *__start_ftrace_events[];
    extern struct trace_event_call *__stop_ftrace_events[];
    static char bootup_event_buf[COMMAND_LINE_SIZE] __initdata;
    static struct seq_buf bootup_event_seq __initdata = {
    .buffer = bootup_event_buf,
    .size = sizeof!(bootup_event_buf),
    };
#[no_mangle]
unsafe extern "C" fn setup_trace_event(str: *mut c_char) -> __init int {
    if (seq_buf_used(&bootup_event_seq) > 0) {
    seq_buf_puts(&bootup_event_seq, ",");
    }
    seq_buf_puts(&bootup_event_seq, str);
    if (seq_buf_has_overflowed(&bootup_event_seq)) {
    return -ENOMEM;
    }
    trace_set_ring_buffer_expanded(core::ptr::null_mut());
    disable_tracing_selftest("running event tracing");
    return 1;
    }
    __setup!("trace_event=", setup_trace_event);
#[no_mangle]
pub unsafe extern "C" fn events_callback(name: *mut c_char, mode: *mut umode_t, data: *mut *mut c_void, fops: *mut *mut file_operations) -> c_int {
    if (strcmp(name, "enable") == 0) {
// mode = TRACE_MODE_WRITE;
// fops = &ftrace_tr_enable_fops;
    return 1;
    }
    if (strcmp(name, "header_page") == 0) {
// mode = TRACE_MODE_READ;
// fops = &ftrace_show_header_page_fops;
    } else if (strcmp(name, "header_event") == 0) {
// mode = TRACE_MODE_READ;
// fops = &ftrace_show_header_event_fops;
    } else {
    return 0;
    }
    return 1;
    }
// Expects to have event_mutex held when called
#[no_mangle]
pub unsafe extern "C" fn create_event_toplevel_files(parent: *mut dentry, tr: *mut trace_array) -> c_int {
pub static mut e_events: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut nr_entries = 0;
pub static mut eventfs_entry: usize = 0;
    if (!trace_array_is_readonly(tr)) {
    entry = trace_create_file("set_event", TRACE_MODE_WRITE, parent,
    tr, &ftrace_set_event_fops);
    if (!entry) {
    return -ENOMEM;
    }
// There are not as crucial, just warn if they are not created
    trace_create_file("show_event_filters", TRACE_MODE_READ, parent, tr,
    &ftrace_show_event_filters_fops);
    trace_create_file("show_event_triggers", TRACE_MODE_READ, parent, tr,
    &ftrace_show_event_triggers_fops);
    trace_create_file("set_event_pid", TRACE_MODE_WRITE, parent,
    tr, &ftrace_set_event_pid_fops);
    trace_create_file("set_event_notrace_pid",
    TRACE_MODE_WRITE, parent, tr,
    &ftrace_set_event_notrace_pid_fops);
    nr_entries = ARRAY_SIZE!(events_entries);
    } else {
    nr_entries = NR_RO_TOP_ENTRIES;
    }
    e_events = eventfs_create_events_dir("events", parent, events_entries,
    nr_entries, tr);
    if (IS_ERR(e_events)) {
    pr_warn!("Could not create tracefs 'events' directory\n");
    return -ENOMEM;
    }
    tr.event_dir = e_events;
    return 0;
    }
//
// event_trace_add_tracer - add a instance of a trace_array to events
// @parent: The parent dentry to place the files/directories for events in
// @tr: The trace array associated with these events
//
// When a new instance is created, it needs to set up its events
// directory, as well as other files associated with events. It also
// creates the event hierarchy in the @parent/events directory.
//
// Returns 0 on success.
//
// Must be called with event_mutex held.
//
#[no_mangle]
pub unsafe extern "C" fn event_trace_add_tracer(parent: *mut dentry, tr: *mut trace_array) -> c_int {
    let mut ret = 0;
    lockdep_assert_held(&event_mutex);
    ret = create_event_toplevel_files(parent, tr);
    if (ret) {
// goto;
    }
    down_write(&trace_event_sem);
// If tr already has the event list, it is initialized in early boot.
    if (unlikely(!list_empty(&tr.events))) {
    __trace_early_add_event_dirs(tr);
    }
    else {
    __trace_add_event_dirs(tr);
    }
    up_write(&trace_event_sem);
// label;
    return ret;
    }
//
// The top trace array already had its file descriptors created.
// Now the files themselves need to be created.
//
#[no_mangle]
pub unsafe extern "C" fn early_event_add_tracer(parent: *mut dentry, tr: *mut trace_array) -> c_int {
    let mut ret = 0;
    guard(mutex)(&event_mutex);
    ret = create_event_toplevel_files(parent, tr);
    if (ret) {
    return ret;
    }
    down_write(&trace_event_sem);
    __trace_early_add_event_dirs(tr);
    up_write(&trace_event_sem);
    return 0;
    }
// Must be called with event_mutex held
#[no_mangle]
pub unsafe extern "C" fn event_trace_del_tracer(tr: *mut trace_array) -> c_int {
    lockdep_assert_held(&event_mutex);
// Disable any event triggers and associated soft-disabled events
    clear_event_triggers(tr);
// Clear the pid list
    __ftrace_clear_event_pids(tr, TRACE_PIDS | TRACE_NO_PIDS);
// Disable any running events
    __ftrace_set_clr_event_nolock(tr, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), 0, core::ptr::null_mut());
// Make sure no more events are being executed
    tracepoint_synchronize_unregister();
    down_write(&trace_event_sem);
    __trace_remove_event_dirs(tr);
    eventfs_remove_events_dir(tr.event_dir);
    up_write(&trace_event_sem);
    tr.event_dir = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn event_trace_memsetup() -> __init int {
    field_cachep = KMEM_CACHE(ftrace_event_field, SLAB_PANIC);
    file_cachep = KMEM_CACHE(trace_event_file, SLAB_PANIC);
    return 0;
    }
//
// Helper function to enable or disable a comma-separated list of events
// from the bootup buffer.
//
#[no_mangle]
unsafe extern "C" fn __early_set_events(tr: *mut trace_array, buf: *mut c_char, enable: bool) -> __init void {
pub static mut token: *mut c_void = core::ptr::null_mut();
    while ((token = strsep(&buf, ","))) {
    if (*token) {
    if (enable) {
    if (ftrace_set_clr_event(tr, token, 1)) {
    pr_warn!("Failed to enable trace event: %s\n", token);
    }
    } else {
    ftrace_set_clr_event(tr, token, 0);
    }
    }
// Put back the comma to allow this to be called again
    if (buf) {
// (buf - 1) = ',';
    }
    }
    }
//
// early_enable_events - enable events from the bootup buffer
// @tr: The trace array to enable the events in
// @buf: The buffer containing the comma separated list of events
// @disable_first: If true, disable all events in @buf before enabling them
//
// This function enables events from the bootup buffer. If @disable_first
// is true, it will first disable all events in the buffer before enabling
// them.
//
// For syscall events, which rely on a global refcount to register the
// SYSCALL_WORK_SYSCALL_TRACEPOINT flag (especially for pid 1), we must
// ensure the refcount hits zero before re-enabling them. A simple
// "disable then enable" per-event is not enough if multiple syscalls are
// used, as the refcount will stay above zero. Thus, we need a two-phase
// approach: disable all, then enable all.
//
#[no_mangle]
pub unsafe extern "C" fn early_enable_events(tr: *mut trace_array, buf: *mut c_char, disable_first: bool) {
    if (disable_first) {
    __early_set_events(tr, buf, false);
    }
    __early_set_events(tr, buf, true);
    }
#[no_mangle]
unsafe extern "C" fn event_trace_enable() -> __init int {
    let mut tr = top_trace_array();
    let mut iter = core::ptr::null_mut();
    let mut call = core::ptr::null_mut();
    let mut ret = 0;
    if (!tr) {
    return -ENODEV;
    }
    for_each_event(iter, __start_ftrace_events, __stop_ftrace_events) {
    call = *iter;
    ret = event_init(call);
    if (!ret) {
    list_add(&call.list, &ftrace_events);
    }
    }
    register_trigger_cmds();
//
// We need the top trace array to have a working set of trace
// points at early init, before the debug files and directories
// are created. Create the file entries now, and attach them
// to the actual file dentries later.
//
    __trace_early_add_events(tr);
    seq_buf_str(&bootup_event_seq);
    early_enable_events(tr, bootup_event_buf, false);
    trace_printk_start_comm();
    register_event_cmds();
    return 0;
    }
//
// event_trace_enable() is called from trace_event_init() first to
// initialize events and perhaps start any events that are on the
// command line. Unfortunately, there are some events that will not
// start this early, like the system call tracepoints that need
// to set the %SYSCALL_WORK_SYSCALL_TRACEPOINT flag of pid 1. But
// event_trace_enable() is called before pid 1 starts, and this flag
// is never set, making the syscall tracepoint never get reached, but
// the event is enabled regardless (and not doing anything).
//
#[no_mangle]
unsafe extern "C" fn event_trace_enable_again() -> __init int {
pub static mut tr: *mut c_void = core::ptr::null_mut();
    tr = top_trace_array();
    if (!tr) {
    return -ENODEV;
    }
    seq_buf_str(&bootup_event_seq);
    early_enable_events(tr, bootup_event_buf, true);
    return 0;
    }
    early_initcall!(event_trace_enable_again);
// Init fields which doesn't related to the tracefs
#[no_mangle]
unsafe extern "C" fn event_trace_init_fields() -> __init int {
    if (trace_define_generic_fields()) {
    pr_warn!("tracing: Failed to allocated generic fields");
    }
    if (trace_define_common_fields()) {
    pr_warn!("tracing: Failed to allocate common fields");
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn event_trace_init() -> __init int {
pub static mut tr: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    tr = top_trace_array();
    if (!tr) {
    return -ENODEV;
    }
    trace_create_file("available_events", TRACE_MODE_READ,
    core::ptr::null_mut(), tr, &ftrace_avail_fops);
    ret = early_event_add_tracer(core::ptr::null_mut(), tr);
    if (ret) {
    return ret;
    }

    ret = register_module_notifier(&trace_module_nb);
    if (ret) {
    pr_warn!("Failed to register trace events module notifier\n");
    }

    eventdir_initialized = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_event_init()  {
    event_trace_memsetup();
    init_ftrace_syscalls();
    event_trace_enable();
    event_trace_init_fields();
    }

pub static mut test_spinlock: usize = 0;
pub static mut test_spinlock_irq: usize = 0;
pub static mut test_mutex: usize = 0;
#[no_mangle]
unsafe extern "C" fn test_work(dummy: *mut work_struct) -> __init void {
    spin_lock(&test_spinlock);
    spin_lock_irq(&test_spinlock_irq);
    udelay(1);
    spin_unlock_irq(&test_spinlock_irq);
    spin_unlock(&test_spinlock);
    mutex_lock(&test_mutex);
    msleep(1);
    mutex_unlock(&test_mutex);
    }
#[no_mangle]
unsafe extern "C" fn event_test_thread(unused: *mut c_void) -> __init int {
pub static mut test_malloc: *mut c_void = core::ptr::null_mut();
    test_malloc = kmalloc(1234, GFP_KERNEL);
    if (!test_malloc) {
    pr_info!("failed to kmalloc\n");
    }
    schedule_on_each_cpu(test_work);
    kfree(test_malloc);
    set_current_state(TASK_INTERRUPTIBLE);
    while (!kthread_should_stop()) {
    schedule();
    set_current_state(TASK_INTERRUPTIBLE);
    }
    __set_current_state(TASK_RUNNING);
    return 0;
    }
//
// Do various things that may trigger events.
//
#[no_mangle]
unsafe extern "C" fn event_test_stuff() -> __init void {
pub static mut test_thread: *mut c_void = core::ptr::null_mut();
    test_thread = kthread_run(event_test_thread, core::ptr::null_mut(), "test-events");
    if (WARN_ON!(IS_ERR(test_thread))) {
    return;
    }
    msleep(1);
    kthread_stop(test_thread);
    }
//
// For every trace event defined, we will test each trace point separately,
// and then by groups, and finally all trace points.
//
#[no_mangle]
unsafe extern "C" fn event_trace_self_tests() -> __init void {
pub static mut dir: *mut c_void = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut call: *mut c_void = core::ptr::null_mut();
pub static mut system: *mut c_void = core::ptr::null_mut();
pub static mut tr: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    tr = top_trace_array();
    if (!tr) {
    return;
    }
    pr_info!("Running tests on trace events:\n");
    list_for_each_entry(file, &tr.events, list) {
    call = file.event_call;
// Only test those that have a probe
    if (!call.class || !call.class.probe) {
    continue;
    }
//
// Testing syscall events here is pretty useless, but
// we still do it if configured. But this is time consuming.
// What we really need is a user thread to perform the
// syscalls as we test.
//

    if (call.class.system &&
    strcmp(call.class.system, "syscalls") == 0) {
    continue;
    }

    pr_info!("Testing event %s: ", trace_event_name(call));
//
// If an event is already enabled, someone is using
// it and the self test should not be on.
//
    if (file.flags & EVENT_FILE_FL_ENABLED) {
    pr_warn!("Enabled event during self test!\n");
    WARN_ON_ONCE!(1);
    continue;
    }
    ftrace_event_enable_disable(file, 1);
    event_test_stuff();
    ftrace_event_enable_disable(file, 0);
    pr_cont("OK\n");
    }
// Now test at the sub system level
    pr_info!("Running tests on trace event systems:\n");
    list_for_each_entry(dir, &tr.systems, list) {
    system = dir.subsystem;
// the ftrace system is special, skip it
    if (strcmp(system.name, "ftrace") == 0) {
    continue;
    }
    pr_info!("Testing event system %s: ", system.name);
    ret = __ftrace_set_clr_event(tr, core::ptr::null_mut(), system.name, core::ptr::null_mut(), 1, core::ptr::null_mut());
    if (WARN_ON_ONCE!(ret)) {
    pr_warn!("error enabling system %s\n",
    system.name);
    continue;
    }
    event_test_stuff();
    ret = __ftrace_set_clr_event(tr, core::ptr::null_mut(), system.name, core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (WARN_ON_ONCE!(ret)) {
    pr_warn!("error disabling system %s\n",
    system.name);
    continue;
    }
    pr_cont("OK\n");
    }
// Test with all events enabled
    pr_info!("Running tests on all trace events:\n");
    pr_info!("Testing all events: ");
    ret = __ftrace_set_clr_event(tr, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), 1, core::ptr::null_mut());
    if (WARN_ON_ONCE!(ret)) {
    pr_warn!("error enabling all events\n");
    return;
    }
    event_test_stuff();
// reset sysname
    ret = __ftrace_set_clr_event(tr, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (WARN_ON_ONCE!(ret)) {
    pr_warn!("error disabling all events\n");
    return;
    }
    pr_cont("OK\n");
    }

pub static mut atomic_t: usize = 0;
    static struct trace_event_file event_trace_file __initdata;
    static void __init
    function_test_events_call(unsigned long ip, unsigned long parent_ip, ftrace_ops *op, ftrace_regs *regs)
    {
pub static mut buffer: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut trace_ctx = 0;
    let mut disabled = 0;
    let mut cpu = 0;
    trace_ctx = tracing_gen_ctx();
    preempt_disable_notrace();
    cpu = raw_smp_processor_id();
    disabled = atomic_inc_return(&per_cpu(ftrace_test_event_disable, cpu));
    if (disabled != 1) {
// goto;
    }
    event = trace_event_buffer_lock_reserve(&buffer, &event_trace_file,
    TRACE_FN, sizeof!(*entry),
    trace_ctx);
    if (!event) {
// goto;
    }
    entry	= ring_buffer_event_data(event);
    entry.ip			= ip;
    entry.parent_ip		= parent_ip;
    event_trigger_unlock_commit(&event_trace_file, buffer, event,
    entry, trace_ctx);
// label;
    atomic_dec(&per_cpu(ftrace_test_event_disable, cpu));
    preempt_enable_notrace();
    }
    static struct ftrace_ops trace_ops __initdata  =
    {
    .func = function_test_events_call,
    };
#[no_mangle]
unsafe extern "C" fn event_trace_self_test_with_function() -> __init void {
    let mut ret = 0;
    event_trace_file.tr = top_trace_array();
    if (WARN_ON!(!event_trace_file.tr)) {
    return;
    }
    ret = register_ftrace_function(&trace_ops);
    if (WARN_ON!(ret < 0)) {
    pr_info!("Failed to enable function tracer for event tests\n");
    return;
    }
    pr_info!("Running tests again, along with the function tracer\n");
    event_trace_self_tests();
    unregister_ftrace_function(&trace_ops);
    }

#[no_mangle]
unsafe extern "C" fn event_trace_self_test_with_function() -> __init void {
    }

#[no_mangle]
unsafe extern "C" fn event_trace_self_tests_init() -> __init int {
    if (!tracing_selftest_disabled) {
    event_trace_self_tests();
    event_trace_self_test_with_function();
    }
    return 0;
    }
    late_initcall!(event_trace_self_tests_init);
}
}
}

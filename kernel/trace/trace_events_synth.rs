//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_events_synth.c
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
// trace_events_synth - synthetic trace events
//
// Copyright (C) 2015, 2020 Tom Zanussi <tom.zanussi@linux.intel.com>
//

// for gfp flag names

    C(BAD_NAME,		"Illegal name"),		
    C(INVALID_CMD,		"Command must be of the form: <name> field[;field] ..."),
    C(INVALID_DYN_CMD,	"Command must be of the form: s or -:[synthetic/]<name> field[;field] ..."),
    C(EVENT_EXISTS,		"Event already exists"),	
    C(TOO_MANY_FIELDS,	"Too many fields"),		
    C(INCOMPLETE_TYPE,	"Incomplete type"),		
    C(INVALID_TYPE,		"Invalid type"),		
    C(INVALID_FIELD,        "Invalid field"),		
    C(INVALID_ARRAY_SPEC,	"Invalid array specification"),

    enum { ERRORS };

    static const char *err_text[] = { ERRORS };
pub static mut lastcmd_mutex: usize = 0;
pub static mut last_cmd: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn errpos(str: *const c_char) -> c_int {
    guard(mutex)(&lastcmd_mutex);
    if (!str || !last_cmd) {
    return 0;
    }
    return err_pos(last_cmd, str);
    }
#[no_mangle]
unsafe extern "C" fn last_cmd_set(str: *const c_char) {
    if (!str) {
    return;
    }
    mutex_lock(&lastcmd_mutex);
    kfree(last_cmd);
    last_cmd = kstrdup(str, GFP_KERNEL);
    mutex_unlock(&lastcmd_mutex);
    }
#[no_mangle]
unsafe extern "C" fn synth_err(err_type: u8, err_pos: u16) {
    guard(mutex)(&lastcmd_mutex);
    if (!last_cmd) {
    return;
    }
    tracing_log_err(core::ptr::null_mut(), "synthetic_events", last_cmd, err_text,
    err_type, err_pos);
    }
// forward_decl: create_synth_event;
// forward_decl: synth_event_show;
// forward_decl: synth_event_release;
// forward_decl: synth_event_is_busy;
// forward_decl: synth_event_match;
pub static mut dyn_event_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn is_synth_event(ev: *mut dyn_event) -> bool {
    return ev.ops == &synth_event_ops;
    }
#[no_mangle]
pub unsafe extern "C" fn to_synth_event(ev: *mut dyn_event) -> *mut c_void {
    return container_of!(ev, synth_event, devent);
    }
#[no_mangle]
unsafe extern "C" fn synth_event_is_busy(ev: *mut dyn_event) -> bool {
    let mut event = to_synth_event(ev);
    return event.ref != 0;
    }
#[no_mangle]
pub unsafe extern "C" fn synth_event_match(system: *mut c_char, event: *mut c_char, argc: c_int, argv: *mut *mut c_char, ev: *mut dyn_event) -> bool {
    let mut sev = to_synth_event(ev);
    return strcmp(sev.name, event) == 0 &&
    (!system || strcmp(system, SYNTH_SYSTEM) == 0);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synth_trace_event {
    pub ent: trace_entry,
    pub fields: [union trace_synth_field; ],
}

#[no_mangle]
unsafe extern "C" fn synth_event_define_fields(call: *mut trace_event_call) -> c_int {
pub static mut trace: usize = 0;
pub static mut offset: c_int = 0;
    let mut event = call.data;
    let mut i = 0;
    let mut size = 0;
    let mut n_u64 = 0;
    let mut name = core::ptr::null_mut();
    let mut type = core::ptr::null_mut();
    let mut filter_type = 0;
    let mut is_signed = 0;
    let mut is_stack = 0;
pub static mut ret: c_int = 0;
    while (i < event.n_fields) {
    size = event.fields[i].size;
    is_signed = event.fields[i].is_signed;
    type = event.fields[i].type;
    name = event.fields[i].name;
    is_stack = event.fields[i].is_stack;
    filter_type = is_stack ? FILTER_STACKTRACE : FILTER_OTHER;
    ret = trace_define_field(call, type, name, offset, size,
    is_signed, filter_type);
    if (ret) {
    break;
    }
    event.fields[i].offset = n_u64;
    if (event.fields[i].is_string && !event.fields[i].is_dynamic) {
    offset += STR_VAR_LEN_MAX;
    n_u64 += STR_VAR_LEN_MAX / sizeof!(u64);
    } else {
    offset += sizeof!(u64);
    n_u64 += 1;
    }
    }
    event.n_u64 = n_u64;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn synth_field_signed(type: *mut c_char) -> bool {
    if (str_has_prefix(type, "u")) {
    return false;
    }
    if (strcmp(type, "gfp_t") == 0) {
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn synth_field_is_string(type: *mut c_char) -> c_int {
    if (strstr(type, "char[") != core::ptr::null_mut()) {
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn synth_field_is_stack(type: *mut c_char) -> c_int {
    if (strstr(type, "long[") != core::ptr::null_mut()) {
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn synth_field_string_size(type: *mut c_char) -> c_int {
    char buf[4], *end, *start;
    let mut len = 0;
    let mut size = 0;
    let mut err = 0;
    start = strstr(type, "char[");
    if (start == core::ptr::null_mut()) {
    return -EINVAL;
    }
    start += sizeof!("char[") - 1;
    end = strchr(type, ']');
    if (!end || end < start || type + strlen(type) > end + 1) {
    return -EINVAL;
    }
    len = end - start;
    if (len > 3) {
    return -EINVAL;
    }
    if (len == 0) {
    return 0; /* variable-length string */
    }
    memcpy(buf, start, len);
    buf[len] = '\0';
    err = kstrtouint(buf, 0, &size);
    if (err) {
    return err;
    }
    if (size > STR_VAR_LEN_MAX) {
    return -EINVAL;
    }
    return size;
    }
#[no_mangle]
unsafe extern "C" fn synth_field_size(type: *mut c_char) -> c_int {
pub static mut size: c_int = 0;
    if (strcmp(type, "s64") == 0) {
    size = sizeof!(s64);
    }

    else if (strcmp(type, "u64") == 0) {
    size = sizeof!(u64);
    }

    else if (strcmp(type, "s32") == 0) {
    size = sizeof!(s32);
    }

    else if (strcmp(type, "u32") == 0) {
    size = sizeof!(u32);
    }

    else if (strcmp(type, "s16") == 0) {
    size = sizeof!(s16);
    }

    else if (strcmp(type, "u16") == 0) {
    size = sizeof!(u16);
    }

    else if (strcmp(type, "s8") == 0) {
    size = sizeof!(s8);
    }

    else if (strcmp(type, "u8") == 0) {
    size = sizeof!(u8);
    }

    else if (strcmp(type, "char") == 0) {
    size = sizeof!(char);
    }

    else if (strcmp(type, "unsigned char") == 0) {
    size = sizeof!(unsigned char);
    }

    else if (strcmp(type, "int") == 0) {
    size = sizeof!(int);
    }

    else if (strcmp(type, "unsigned int") == 0) {
    size = sizeof!(unsigned int);
    }

    else if (strcmp(type, "long") == 0) {
    size = sizeof!(long);
    }

    else if (strcmp(type, "unsigned long") == 0) {
    size = sizeof!(unsigned long);
    }

    else if (strcmp(type, "bool") == 0) {
    size = sizeof!(bool);
    }

    else if (strcmp(type, "pid_t") == 0) {
    size = sizeof!(pid_t);
    }

    else if (strcmp(type, "gfp_t") == 0) {
    size = sizeof!(gfp_t);
    }

    else if (synth_field_is_string(type)) {
    size = synth_field_string_size(type);
    }

    else if (synth_field_is_stack(type)) {
    size = 0;
    }
    return size;
    }
    static const char *synth_field_fmt(char *type)
    {
    let mut fmt = "%llu";
    if (strcmp(type, "s64") == 0) {
    fmt = "%lld";
    }

    else if (strcmp(type, "u64") == 0) {
    fmt = "%llu";
    }

    else if (strcmp(type, "s32") == 0) {
    fmt = "%d";
    }

    else if (strcmp(type, "u32") == 0) {
    fmt = "%u";
    }

    else if (strcmp(type, "s16") == 0) {
    fmt = "%d";
    }

    else if (strcmp(type, "u16") == 0) {
    fmt = "%u";
    }

    else if (strcmp(type, "s8") == 0) {
    fmt = "%d";
    }

    else if (strcmp(type, "u8") == 0) {
    fmt = "%u";
    }

    else if (strcmp(type, "char") == 0) {
    fmt = "%d";
    }

    else if (strcmp(type, "unsigned char") == 0) {
    fmt = "%u";
    }

    else if (strcmp(type, "int") == 0) {
    fmt = "%d";
    }

    else if (strcmp(type, "unsigned int") == 0) {
    fmt = "%u";
    }

    else if (strcmp(type, "long") == 0) {
    fmt = "%ld";
    }

    else if (strcmp(type, "unsigned long") == 0) {
    fmt = "%lu";
    }

    else if (strcmp(type, "bool") == 0) {
    fmt = "%d";
    }

    else if (strcmp(type, "pid_t") == 0) {
    fmt = "%d";
    }

    else if (strcmp(type, "gfp_t") == 0) {
    fmt = "%x";
    }

    else if (synth_field_is_string(type)) {
    fmt = "%s";
    }

    else if (synth_field_is_stack(type)) {
    fmt = "%s";
    }
    return fmt;
    }
#[no_mangle]
pub unsafe extern "C" fn print_synth_event_num_val(s: *mut trace_seq, print_fmt: *mut c_char, name: *mut c_char, size: c_int, val: *mut union trace_synth_field, space: *mut c_char) {
    match (size) {
    1 => {
    trace_seq_printf(s, print_fmt, name, val.as_u8, space);
    // break;
    }
    2 => {
    trace_seq_printf(s, print_fmt, name, val.as_u16, space);
    // break;
    }
    4 => {
    trace_seq_printf(s, print_fmt, name, val.as_u32, space);
    // break;
    }
    _ => {
    trace_seq_printf(s, print_fmt, name, val.as_u64, space);
    // break;
    }
    }
    }
    static enum print_line_t print_synth_event(trace_iterator *iter,
    int flags, trace_event *event)
    {
    let mut tr = iter.tr;
    let mut s = &iter.seq;
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut se: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut j = 0;
    let mut n_u64 = 0;
    char print_fmt[32];
pub static mut fmt: *mut c_void = core::ptr::null_mut();
    entry = iter.ent;
    se = container_of!(event, synth_event, call.event);
    trace_seq_printf(s, "%s: ", se.name);
    while (i < se.n_fields) {
    if (trace_seq_has_overflowed(s)) {
// goto;
    }
    fmt = synth_field_fmt(se.fields[i].type);
// parameter types
    if (tr && tr.trace_flags & TRACE_ITER(VERBOSE)) {
    trace_seq_printf(s, "%s ", fmt);
    }
    snprintf(print_fmt, sizeof!(print_fmt), "%%s=%s%%s", fmt);
// parameter values
    if (se.fields[i].is_string) {
    if (se.fields[i].is_dynamic) {
    union trace_synth_field *data = &entry.fields[n_u64];
    trace_seq_printf(s, print_fmt, se.fields[i].name,
    entry + data.as_dynamic.offset,
    i == se.n_fields - 1 ? "" : " ");
    n_u64 += 1;
    } else {
    trace_seq_printf(s, print_fmt, se.fields[i].name,
    &entry.fields[n_u64].as_u64,
    i == se.n_fields - 1 ? "" : " ");
    n_u64 += STR_VAR_LEN_MAX / sizeof!(u64);
    }
    } else if (se.fields[i].is_stack) {
    union trace_synth_field *data = &entry.fields[n_u64];
    let mut p = entry + data.as_dynamic.offset;
    trace_seq_printf(s, "%s=STACK:\n", se.fields[i].name);
    for (j = 1; j < data.as_dynamic.len / sizeof!(long); j++) {
    trace_seq_printf(s, "=> %pS\n", p[j]);
    }
    n_u64 += 1;
    } else {
pub static mut trace_print_flags: usize = 0;
    let mut space = (i == se.n_fields - 1 ? "" : " ");
    print_synth_event_num_val(s, print_fmt,
    se.fields[i].name,
    se.fields[i].size,
    &entry.fields[n_u64],
    space);
    if (strcmp(se.fields[i].type, "gfp_t") == 0) {
    trace_seq_puts(s, " (");
    trace_print_flags_seq(s, "|",
    entry.fields[n_u64].as_u64,
    __flags, ARRAY_SIZE!(__flags));
    trace_seq_putc(s, ')');
    }
    n_u64 += 1;
    }
    }
// label;
    trace_seq_putc(s, '\n');
    return trace_handle_return(s);
    }
pub static mut trace_event_functions: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn trace_string(entry: *mut synth_trace_event, event: *mut synth_event, str_val: *mut c_char, is_dynamic: bool, data_size: c_uint, n_u64: *mut c_uint) -> c_uint {
pub static mut len: c_uint = 0;
pub static mut str_field: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (is_dynamic) {
    union trace_synth_field *data = &entry.fields[*n_u64];
    len = fetch_store_strlen((unsigned long)str_val);
    data.as_dynamic.offset = struct_size(entry, fields, event.n_u64) + data_size;
    data.as_dynamic.len = len;
    ret = fetch_store_string((unsigned long)str_val, &entry.fields[*n_u64], entry);
    (*n_u64)++;
    } else {
    str_field = &entry.fields[*n_u64].as_u64;

    if ((unsigned long)str_val < TASK_SIZE) {
    ret = strncpy_from_user_nofault(str_field, str_val, STR_VAR_LEN_MAX);
    }
    else {

    ret = strncpy_from_kernel_nofault(str_field, str_val, STR_VAR_LEN_MAX);
    }
    if (ret < 0) {
    strcpy(str_field, FAULT_STRING);
    }
    (*n_u64) += STR_VAR_LEN_MAX / sizeof!(u64);
    }
    return len;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_stack(entry: *mut synth_trace_event, event: *mut synth_event, stack: *mut c_long, data_size: c_uint, n_u64: *mut c_uint) -> c_uint {
    union trace_synth_field *data = &entry.fields[*n_u64];
    let mut len = 0;
    let mut data_offset = 0;
pub static mut data_loc: *mut c_void = core::ptr::null_mut();
    data_offset = struct_size(entry, fields, event.n_u64);
    data_offset += data_size;
    while (len < HIST_STACKTRACE_DEPTH) {
    if (!stack[len]) {
    break;
    }
    }
    len *= sizeof!(long);
// Find the dynamic section to copy the stack into.
    data_loc = entry + data_offset;
    memcpy(data_loc, stack, len);
// Fill in the field that holds the offset/len combo
    data.as_dynamic.offset = data_offset;
    data.as_dynamic.len = len;
    (*n_u64)++;
    return len;
    }
    static __always_inline int get_field_size(synth_event *event,
    u64 *var_ref_vals,
    unsigned int *var_ref_idx)
    {
    let mut fields_size = 0;
    fields_size = event.n_u64 * sizeof!(u64);
    while (i < event.n_dynamic_fields) {
pub static mut field_pos: c_uint = 0;
pub static mut str_val: *mut c_void = core::ptr::null_mut();
    let mut val_idx = 0;
    let mut len = 0;
    val_idx = var_ref_idx[field_pos];
    str_val = (long)var_ref_vals[val_idx];
    if (event.dynamic_fields[i].is_stack) {
// reserve one extra element for size
    len = *(str_val) + 1;
    len *= sizeof!(unsigned long);
    } else {
    len = fetch_store_strlen((unsigned long)str_val);
    }
    fields_size += len;
    }
    return fields_size;
    }
    static __always_inline void write_synth_entry(synth_event *event, synth_trace_event *entry,
    u64 *var_ref_vals,
    unsigned int *var_ref_idx)
    {
pub static mut data_size: c_int = 0;
    let mut i = 0;
    let mut n_u64 = 0;
    let mut val_idx = 0;
    let mut len = 0;
    while (i < event.n_fields) {
    val_idx = var_ref_idx[i];
    if (event.fields[i].is_string) {
    let mut str_val = (long)var_ref_vals[val_idx];
    len = trace_string(entry, event, str_val,
    event.fields[i].is_dynamic,
    data_size, &n_u64);
    data_size += len; /* only dynamic string increments */
    } else if (event.fields[i].is_stack) {
    let mut stack = (long)var_ref_vals[val_idx];
    len = trace_stack(entry, event, stack,
    data_size, &n_u64);
    data_size += len;
    } else {
    let mut field = event.fields[i];
pub static mut val: u64 = 0;
    match (field.size) {
    1 => {
    entry.fields[n_u64].as_u8 = (u8)val;
    // break;
    }
    2 => {
    entry.fields[n_u64].as_u16 = (u16)val;
    // break;
    }
    4 => {
    entry.fields[n_u64].as_u32 = (u32)val;
    // break;
    }
    _ => {
    entry.fields[n_u64].as_u64 = val;
    // break;
    }
    }
    n_u64 += 1;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn trace_event_raw_event_synth(__data: *mut c_void, var_ref_vals: *mut u64, var_ref_idx: *mut c_uint) {
    let mut trace_file = __data;
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut fbuffer: usize = 0;
pub static mut buffer: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
    let mut fields_size = 0;
    event = trace_file.event_call.data;
    if (trace_trigger_soft_disabled(trace_file)) {
    return;
    }
    fields_size = get_field_size(event, var_ref_vals, var_ref_idx);
//
// Avoid ring buffer recursion detection, as this event
// is being performed within another event.
//
    buffer = trace_file.tr.array_buffer.buffer;
    guard(ring_buffer_nest)(buffer);
    entry = trace_event_buffer_reserve(&fbuffer, trace_file,
    sizeof!(*entry) + fields_size);
    if (!entry) {
    return;
    }
    write_synth_entry(event, entry, var_ref_vals, var_ref_idx);
    trace_event_buffer_commit(&fbuffer);
    }

#[no_mangle]
pub unsafe extern "C" fn perf_event_raw_event_synth(__data: *mut c_void, var_ref_vals: *mut u64, var_ref_idx: *mut c_uint) {
    let mut call = __data;
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut perf_head: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut regs: *mut c_void = core::ptr::null_mut();
    let mut fields_size = 0;
    let mut size = 0;
    let mut context = 0;
    event = call.data;
    perf_head = this_cpu_ptr(call.perf_events);
    if (!perf_head || hlist_empty(perf_head)) {
    return;
    }
    fields_size = get_field_size(event, var_ref_vals, var_ref_idx);
    size = ALIGN(sizeof!(*entry) + fields_size, 8);
    entry = perf_trace_buf_alloc(size, &regs, &context);
    if (unlikely(!entry)) {
    return;
    }
    write_synth_entry(event, entry, var_ref_vals, var_ref_idx);
    perf_fetch_caller_regs(regs);
    perf_trace_buf_submit(entry, size, context,
    call.event.type, 1, regs,
    perf_head, core::ptr::null_mut());
    }

#[no_mangle]
unsafe extern "C" fn free_synth_event_print_fmt(call: *mut trace_event_call) {
    if (call) {
    kfree(call.print_fmt);
    call.print_fmt = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __set_synth_event_print_fmt(event: *mut synth_event, buf: *mut c_char, len: c_int) -> c_int {
pub static mut fmt: *mut c_void = core::ptr::null_mut();
pub static mut pos: c_int = 0;
    let mut i = 0;
// When len=0, we just calculate the needed length

    pos += snprintf(buf + pos, LEN_OR_ZERO, "\"");
    while (i < event.n_fields) {
    fmt = synth_field_fmt(event.fields[i].type);
    pos += snprintf(buf + pos, LEN_OR_ZERO, "%s=%s%s",
    event.fields[i].name, fmt,
    i == event.n_fields - 1 ? "" : " ");
    }
    pos += snprintf(buf + pos, LEN_OR_ZERO, "\"");
    while (i < event.n_fields) {
    if (event.fields[i].is_string &&
    event.fields[i].is_dynamic) {
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    ", __get_str(%s)", event.fields[i].name);
    }

    else if (event.fields[i].is_stack) {
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    ", __get_stacktrace(%s)", event.fields[i].name);
    }
    else {
    pos += snprintf(buf + pos, LEN_OR_ZERO,
    ", REC.%s", event.fields[i].name);
    }
    }

// return the length of print_fmt
    return pos;
    }
#[no_mangle]
unsafe extern "C" fn set_synth_event_print_fmt(call: *mut trace_event_call) -> c_int {
    let mut event = call.data;
pub static mut print_fmt: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
// First: called with 0 length to calculate the needed length
    len = __set_synth_event_print_fmt(event, core::ptr::null_mut(), 0);
    print_fmt = kmalloc(len + 1, GFP_KERNEL);
    if (!print_fmt) {
    return -ENOMEM;
    }
// Second: actually write the @print_fmt
    __set_synth_event_print_fmt(event, print_fmt, len + 1);
    call.print_fmt = print_fmt;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_synth_field(field: *mut synth_field) {
    kfree(field.type);
    kfree(field.name);
    kfree(field);
    }
#[no_mangle]
pub unsafe extern "C" fn check_field_version(prefix: *mut c_char, field_type: *mut c_char, field_name: *mut c_char) -> c_int {
//
// For backward compatibility, the old synthetic event command
// format did not require semicolons, and in order to not
// break user space, that old format must still work. If a new
// feature is added, then the format that uses the new feature
// will be required to have semicolons, as nothing that uses
// the old format would be using the new, yet to be created,
// feature. When a new feature is added, this will detect it,
// and return a number greater than 1, and require the format
// to use semicolons.
//
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_synth_field(argc: c_int, argv: *mut *mut c_char, consumed: *mut c_int, field_version: *mut c_int) -> *mut c_void {
    let mut prefix = core::ptr::null_mut(), *field_type = argv[0], *field_name, *array;
pub static mut field: *mut c_void = core::ptr::null_mut();
    int len, ret = -ENOMEM;
pub static mut s: usize = 0;
    let mut size = 0;
    if (!strcmp(field_type, "unsigned")) {
    if (argc < 3) {
    synth_err(SYNTH_ERR_INCOMPLETE_TYPE, errpos(field_type));
    return ERR_PTR(-EINVAL);
    }
    prefix = "unsigned ";
    field_type = argv[1];
    field_name = argv[2];
// consumed += 3;
    } else {
    field_name = argv[1];
// consumed += 2;
    }
    if (!field_name) {
    synth_err(SYNTH_ERR_INVALID_FIELD, errpos(field_type));
    return ERR_PTR(-EINVAL);
    }
// field_version = check_field_version(prefix, field_type, field_name);
    field = kzalloc_obj(*field);
    if (!field) {
    return ERR_PTR(-ENOMEM);
    }
    len = strlen(field_name);
    array = strchr(field_name, '[');
    if (array) {
    len -= strlen(array);
    }
    field.name = kmemdup_nul(field_name, len, GFP_KERNEL);
    if (!field.name) {
// goto;
    }
    if (!is_good_name(field.name)) {
    synth_err(SYNTH_ERR_BAD_NAME, errpos(field_name));
    ret = -EINVAL;
// goto;
    }
    len = strlen(field_type) + 1;
    if (array) {
    len += strlen(array);
    }
    if (prefix) {
    len += strlen(prefix);
    }
    field.type = kzalloc(len, GFP_KERNEL);
    if (!field.type) {
// goto;
    }
    seq_buf_init(&s, field.type, len);
    if (prefix) {
    seq_buf_puts(&s, prefix);
    }
    seq_buf_puts(&s, field_type);
    if (array) {
    seq_buf_puts(&s, array);
    }
    if (WARN_ON_ONCE!(!seq_buf_buffer_left(&s))) {
// goto;
    }
    s.buffer[s.len] = '\0';
    size = synth_field_size(field.type);
    if (size < 0) {
    if (array) {
    synth_err(SYNTH_ERR_INVALID_ARRAY_SPEC, errpos(field_name));
    }
    else {
    synth_err(SYNTH_ERR_INVALID_TYPE, errpos(field_type));
    }
    ret = -EINVAL;
// goto;
    } else if (size == 0) {
    if (synth_field_is_string(field.type) ||
    synth_field_is_stack(field.type)) {
pub static mut type: *mut c_void = core::ptr::null_mut();
    len = sizeof!("__data_loc ") + strlen(field.type) + 1;
    type = kzalloc(len, GFP_KERNEL);
    if (!type) {
// goto;
    }
    seq_buf_init(&s, type, len);
    seq_buf_puts(&s, "__data_loc ");
    seq_buf_puts(&s, field.type);
    if (WARN_ON_ONCE!(!seq_buf_buffer_left(&s))) {
    kfree(type);
// goto;
    }
    s.buffer[s.len] = '\0';
    kfree(field.type);
    field.type = type;
    field.is_dynamic = true;
    size = sizeof!(u64);
    } else {
    synth_err(SYNTH_ERR_INVALID_TYPE, errpos(field_type));
    ret = -EINVAL;
// goto;
    }
    }
    field.size = size;
    if (synth_field_is_string(field.type)) {
    field.is_string = true;
    }

    else if (synth_field_is_stack(field.type)) {
    field.is_stack = true;
    }
    field.is_signed = synth_field_signed(field.type);
// label;
    return field;
// label;
    free_synth_field(field);
    field = ERR_PTR(ret);
// goto;
    }
#[no_mangle]
unsafe extern "C" fn free_synth_tracepoint(tp: *mut tracepoint) {
    if (!tp) {
    return;
    }
    kfree(tp.name);
    kfree(tp);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_synth_tracepoint(name: *mut c_char) -> *mut c_void {
pub static mut tp: *mut c_void = core::ptr::null_mut();
    tp = kzalloc_obj(*tp);
    if (!tp) {
    return ERR_PTR(-ENOMEM);
    }
    tp.name = kstrdup(name, GFP_KERNEL);
    if (!tp.name) {
    kfree(tp);
    return ERR_PTR(-ENOMEM);
    }
    return tp;
    }
#[no_mangle]
pub unsafe extern "C" fn find_synth_event(name: *mut c_char) -> *mut c_void {
pub static mut pos: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
    for_each_dyn_event(pos) {
    if (!is_synth_event(pos)) {
    continue;
    }
    event = to_synth_event(pos);
    if (strcmp(event.name, name) == 0) {
    return event;
    }
    }
    return core::ptr::null_mut();
    }
pub static mut trace_event_fields: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn synth_event_reg(call: *mut trace_event_call, type: trace_reg, data: *mut c_void) -> c_int {
    let mut event = container_of!(call, synth_event, call);
    match (type) {

    TRACE_REG_PERF_REGISTER => {

    }
    TRACE_REG_REGISTER => {
    if (!try_module_get(event.mod)) {
    return -EBUSY;
    }
    // break;
    }
    _ => {
    // break;
    }
    }
pub static mut ret: c_int = 0;
    match (type) {

    TRACE_REG_PERF_UNREGISTER => {

    }
    TRACE_REG_UNREGISTER => {
    module_put!(event.mod);
    // break;
    }
    _ => {
    // break;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn register_synth_event(event: *mut synth_event) -> c_int {
    let mut call = &event.call;
pub static mut ret: c_int = 0;
    event.call.class = &event.class;
    event.class.system = kstrdup(SYNTH_SYSTEM, GFP_KERNEL);
    if (!event.class.system) {
    ret = -ENOMEM;
// goto;
    }
    event.tp = alloc_synth_tracepoint(event.name);
    if (IS_ERR(event.tp)) {
    ret = PTR_ERR(event.tp);
    event.tp = core::ptr::null_mut();
// goto;
    }
    INIT_LIST_HEAD(&call.class.fields);
    call.event.funcs = &synth_event_funcs;
    call.class.fields_array = synth_event_fields_array;
    ret = register_trace_event(&call.event);
    if (!ret) {
    ret = -ENODEV;
// goto;
    }
    call.flags = TRACE_EVENT_FL_TRACEPOINT;
    call.class.reg = synth_event_reg;
    call.class.probe = trace_event_raw_event_synth;

    call.class.perf_probe = perf_event_raw_event_synth;

    call.data = event;
    call.tp = event.tp;
    ret = trace_add_event_call(call);
    if (ret) {
    pr_warn!("Failed to register synthetic event: %s\n",
    trace_event_name(call));
// goto;
    }
    ret = set_synth_event_print_fmt(call);
// unregister_trace_event() will be called inside
    if (ret < 0) {
    trace_remove_event_call(call);
    }
// label;
    return ret;
// label;
    unregister_trace_event(&call.event);
// goto;
    }
#[no_mangle]
unsafe extern "C" fn unregister_synth_event(event: *mut synth_event) -> c_int {
    let mut call = &event.call;
    let mut ret = 0;
    ret = trace_remove_event_call(call);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn free_synth_event(event: *mut synth_event) {
    let mut i = 0;
    if (!event) {
    return;
    }
    for (i = 0; i < event.n_fields; i++) {
    free_synth_field(event.fields[i]);
    }
    kfree(event.fields);
    kfree(event.dynamic_fields);
    kfree(event.name);
    kfree(event.class.system);
    free_synth_tracepoint(event.tp);
    free_synth_event_print_fmt(&event.call);
    kfree(event);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_synth_event(name: *mut c_char, n_fields: c_int, fields: *mut *mut synth_field) -> *mut c_void {
    unsigned int i, j, n_dynamic_fields = 0;
pub static mut event: *mut c_void = core::ptr::null_mut();
    event = kzalloc_obj(*event);
    if (!event) {
    event = ERR_PTR(-ENOMEM);
// goto;
    }
    event.name = kstrdup(name, GFP_KERNEL);
    if (!event.name) {
    kfree(event);
    event = ERR_PTR(-ENOMEM);
// goto;
    }
    event.fields = kzalloc_objs(*event.fields, n_fields);
    if (!event.fields) {
    free_synth_event(event);
    event = ERR_PTR(-ENOMEM);
// goto;
    }
    for (i = 0; i < n_fields; i++) {
    if (fields[i].is_dynamic)
    n_dynamic_fields += 1;
    }
    if (n_dynamic_fields) {
    event.dynamic_fields = kzalloc_objs(*event.dynamic_fields,
    n_dynamic_fields);
    if (!event.dynamic_fields) {
    free_synth_event(event);
    event = ERR_PTR(-ENOMEM);
// goto;
    }
    }
    dyn_event_init(&event.devent, &synth_event_ops);
    while (i < n_fields) {
    fields[i].field_pos = i;
    event.fields[i] = fields[i];
    if (fields[i].is_dynamic) {
    event.dynamic_fields[j++] = fields[i];
    }
    }
    event.n_dynamic_fields = j;
    event.n_fields = n_fields;
// label;
    return event;
    }
#[no_mangle]
unsafe extern "C" fn synth_event_check_arg_fn(data: *mut c_void) -> c_int {
    let mut arg_pair = data;
    let mut size = 0;
    size = synth_field_size(arg_pair.lhs);
    if (size == 0) {
    if (strstr(arg_pair.lhs, "[")) {
    return 0;
    }
    }
    return size ? 0 : -EINVAL;
    }
//
// synth_event_add_field - Add a new field to a synthetic event cmd
// @cmd: A pointer to the dynevent_cmd struct representing the new event
// @type: The type of the new field to add
// @name: The name of the new field to add
//
// Add a new field to a synthetic event cmd object.  Field ordering is in
// the same order the fields are added.
//
// See synth_field_size() for available types. If field_name contains
// [n] the field is considered to be an array.
//
// Return: 0 if successful, error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn synth_event_add_field(cmd: *mut dynevent_cmd, type: *mut c_char, name: *mut c_char) -> c_int {
pub static mut arg_pair: usize = 0;
    let mut ret = 0;
    if (cmd.type != DYNEVENT_TYPE_SYNTH) {
    return -EINVAL;
    }
    if (!type || !name) {
    return -EINVAL;
    }
    dynevent_arg_pair_init(&arg_pair, 0, ';');
    arg_pair.lhs = type;
    arg_pair.rhs = name;
    ret = dynevent_arg_pair_add(cmd, &arg_pair, synth_event_check_arg_fn);
    if (ret) {
    return ret;
    }
    if (++cmd.n_fields > SYNTH_FIELDS_MAX) {
    ret = -EINVAL;
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(synth_event_add_field);
//
// synth_event_add_field_str - Add a new field to a synthetic event cmd
// @cmd: A pointer to the dynevent_cmd struct representing the new event
// @type_name: The type and name of the new field to add, as a single string
//
// Add a new field to a synthetic event cmd object, as a single
// string.  The @type_name string is expected to be of the form 'type
// name', which will be appended by ';'.  No sanity checking is done -
// what's passed in is assumed to already be well-formed.  Field
// ordering is in the same order the fields are added.
//
// See synth_field_size() for available types. If field_name contains
// [n] the field is considered to be an array.
//
// Return: 0 if successful, error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn synth_event_add_field_str(cmd: *mut dynevent_cmd, type_name: *const c_char) -> c_int {
pub static mut arg: usize = 0;
    let mut ret = 0;
    if (cmd.type != DYNEVENT_TYPE_SYNTH) {
    return -EINVAL;
    }
    if (!type_name) {
    return -EINVAL;
    }
    dynevent_arg_init(&arg, ';');
    arg.str = type_name;
    ret = dynevent_arg_add(cmd, &arg, core::ptr::null_mut());
    if (ret) {
    return ret;
    }
    if (++cmd.n_fields > SYNTH_FIELDS_MAX) {
    ret = -EINVAL;
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(synth_event_add_field_str);
//
// synth_event_add_fields - Add multiple fields to a synthetic event cmd
// @cmd: A pointer to the dynevent_cmd struct representing the new event
// @fields: An array of type/name field descriptions
// @n_fields: The number of field descriptions contained in the fields array
//
// Add a new set of fields to a synthetic event cmd object.  The event
// fields that will be defined for the event should be passed in as an
// array of struct synth_field_desc, and the number of elements in the
// array passed in as n_fields.  Field ordering will retain the
// ordering given in the fields array.
//
// See synth_field_size() for available types. If field_name contains
// [n] the field is considered to be an array.
//
// Return: 0 if successful, error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn synth_event_add_fields(cmd: *mut dynevent_cmd, fields: *mut synth_field_desc, n_fields: c_uint) -> c_int {
    let mut i = 0;
pub static mut ret: c_int = 0;
    while (i < n_fields) {
    if (fields[i].type == core::ptr::null_mut() || fields[i].name == core::ptr::null_mut()) {
    ret = -EINVAL;
    break;
    }
    ret = synth_event_add_field(cmd, fields[i].type, fields[i].name);
    if (ret) {
    break;
    }
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(synth_event_add_fields);
//
// __synth_event_gen_cmd_start - Start a synthetic event command from arg list
// @cmd: A pointer to the dynevent_cmd struct representing the new event
// @name: The name of the synthetic event
// @mod: The module creating the event, NULL if not created from a module
// @...: Variable number of arg (pairs), one pair for each field
//
// NOTE: Users normally won't want to call this function directly, but
// rather use the synth_event_gen_cmd_start() wrapper, which
// automatically adds a NULL to the end of the arg list.  If this
// function is used directly, make sure the last arg in the variable
// arg list is NULL.
//
// Generate a synthetic event command to be executed by
// synth_event_gen_cmd_end().  This function can be used to generate
// the complete command or only the first part of it; in the latter
// case, synth_event_add_field(), synth_event_add_field_str(), or
// synth_event_add_fields() can be used to add more fields following
// this.
//
// There should be an even number variable args, each pair consisting
// of a type followed by a field name.
//
// See synth_field_size() for available types. If field_name contains
// [n] the field is considered to be an array.
//
// Return: 0 if successful, error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn __synth_event_gen_cmd_start(cmd: *mut dynevent_cmd, name: *mut c_char, mod: *mut module) -> c_int {
pub static mut arg: usize = 0;
    let mut args;
    let mut ret = 0;
    cmd.event_name = name;
    cmd.private_data = mod;
    if (cmd.type != DYNEVENT_TYPE_SYNTH) {
    return -EINVAL;
    }
    dynevent_arg_init(&arg, 0);
    arg.str = name;
    ret = dynevent_arg_add(cmd, &arg, core::ptr::null_mut());
    if (ret) {
    return ret;
    }
    va_start(args, mod);
    for (;;) {
    let mut type = core::ptr::null_mut();
    let mut name = core::ptr::null_mut();
    type = va_arg(args, const char *);
    if (!type) {
    break;
    }
    name = va_arg(args, const char *);
    if (!name) {
    break;
    }
    if (++cmd.n_fields > SYNTH_FIELDS_MAX) {
    ret = -EINVAL;
    break;
    }
    ret = synth_event_add_field(cmd, type, name);
    if (ret) {
    break;
    }
    }
    va_end(args);
    return ret;
    }
    EXPORT_SYMBOL_GPL(__synth_event_gen_cmd_start);
//
// synth_event_gen_cmd_array_start - Start synthetic event command from an array
// @cmd: A pointer to the dynevent_cmd struct representing the new event
// @name: The name of the synthetic event
// @mod: The module creating the event, NULL if not created from a module
// @fields: An array of type/name field descriptions
// @n_fields: The number of field descriptions contained in the fields array
//
// Generate a synthetic event command to be executed by
// synth_event_gen_cmd_end().  This function can be used to generate
// the complete command or only the first part of it; in the latter
// case, synth_event_add_field(), synth_event_add_field_str(), or
// synth_event_add_fields() can be used to add more fields following
// this.
//
// The event fields that will be defined for the event should be
// passed in as an array of struct synth_field_desc, and the number of
// elements in the array passed in as n_fields.  Field ordering will
// retain the ordering given in the fields array.
//
// See synth_field_size() for available types. If field_name contains
// [n] the field is considered to be an array.
//
// Return: 0 if successful, error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn synth_event_gen_cmd_array_start(cmd: *mut dynevent_cmd, name: *mut c_char, mod: *mut module, fields: *mut synth_field_desc, n_fields: c_uint) -> c_int {
pub static mut arg: usize = 0;
    let mut i = 0;
pub static mut ret: c_int = 0;
    cmd.event_name = name;
    cmd.private_data = mod;
    if (cmd.type != DYNEVENT_TYPE_SYNTH) {
    return -EINVAL;
    }
    if (n_fields > SYNTH_FIELDS_MAX) {
    return -EINVAL;
    }
    dynevent_arg_init(&arg, 0);
    arg.str = name;
    ret = dynevent_arg_add(cmd, &arg, core::ptr::null_mut());
    if (ret) {
    return ret;
    }
    while (i < n_fields) {
    if (fields[i].type == core::ptr::null_mut() || fields[i].name == core::ptr::null_mut()) {
    return -EINVAL;
    }
    ret = synth_event_add_field(cmd, fields[i].type, fields[i].name);
    if (ret) {
    break;
    }
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(synth_event_gen_cmd_array_start);
#[no_mangle]
unsafe extern "C" fn __create_synth_event(name: *const c_char, raw_fields: *const c_char) -> c_int {
    char **argv, *field_str, *tmp_fields, *saved_fields = core::ptr::null_mut();
    struct synth_field *field, *fields[SYNTH_FIELDS_MAX];
    int consumed, cmd_version = 1, n_fields_this_loop;
    int i, argc, n_fields = 0, ret = 0;
    let mut event = core::ptr::null_mut();
//
// Argument syntax:
// - Add synthetic event: <event_name> field[;field] ...
// - Remove synthetic event: !<event_name> field[;field] ...
// where 'field' = type field_name
//
    if (name[0] == '\0') {
    synth_err(SYNTH_ERR_INVALID_CMD, 0);
    return -EINVAL;
    }
    if (!is_good_name(name)) {
    synth_err(SYNTH_ERR_BAD_NAME, errpos(name));
    return -EINVAL;
    }
    mutex_lock(&event_mutex);
    event = find_synth_event(name);
    if (event) {
    synth_err(SYNTH_ERR_EVENT_EXISTS, errpos(name));
    ret = -EEXIST;
// goto;
    }
    tmp_fields = saved_fields = kstrdup(raw_fields, GFP_KERNEL);
    if (!tmp_fields) {
    ret = -ENOMEM;
// goto;
    }
    while ((field_str = strsep(&tmp_fields, ";")) != core::ptr::null_mut()) {
    argv = argv_split(GFP_KERNEL, field_str, &argc);
    if (!argv) {
    ret = -ENOMEM;
// goto;
    }
    if (!argc) {
    argv_free(argv);
    continue;
    }
    n_fields_this_loop = 0;
    consumed = 0;
    while (argc > consumed) {
    let mut field_version = 0;
    field = parse_synth_field(argc - consumed,
    argv + consumed, &consumed,
    &field_version);
    if (IS_ERR(field)) {
    ret = PTR_ERR(field);
// goto;
    }
//
// Track the highest version of any field we
// found in the command.
//
    if (field_version > cmd_version) {
    cmd_version = field_version;
    }
//
// Now sort out what is and isn't valid for
// each supported version.
//
// If we see more than 1 field per loop, it
// means we have multiple fields between
// semicolons, and that's something we no
// longer support in a version 2 or greater
// command.
//
    if (cmd_version > 1 && n_fields_this_loop >= 1) {
    synth_err(SYNTH_ERR_INVALID_CMD, errpos(field_str));
    ret = -EINVAL;
// goto;
    }
    if (n_fields == SYNTH_FIELDS_MAX) {
    synth_err(SYNTH_ERR_TOO_MANY_FIELDS, 0);
    ret = -EINVAL;
// goto;
    }
    fields[n_fields++] = field;
    n_fields_this_loop += 1;
    }
    argv_free(argv);
    if (consumed < argc) {
    synth_err(SYNTH_ERR_INVALID_CMD, 0);
    ret = -EINVAL;
// goto;
    }
    }
    if (n_fields == 0) {
    synth_err(SYNTH_ERR_INVALID_CMD, 0);
    ret = -EINVAL;
// goto;
    }
    event = alloc_synth_event(name, n_fields, fields);
    if (IS_ERR(event)) {
    ret = PTR_ERR(event);
    event = core::ptr::null_mut();
// goto;
    }
    ret = register_synth_event(event);
    if (!ret) {
    dyn_event_add(&event.devent, &event.call);
    }
    else {
    free_synth_event(event);
    }
// label;
    mutex_unlock(&event_mutex);
    kfree(saved_fields);
    return ret;
// label;
    free_synth_field(field);
// label;
    argv_free(argv);
// label;
    for (i = 0; i < n_fields; i++) {
    free_synth_field(fields[i]);
    }
// goto;
    }
//
// synth_event_create - Create a new synthetic event
// @name: The name of the new synthetic event
// @fields: An array of type/name field descriptions
// @n_fields: The number of field descriptions contained in the fields array
// @mod: The module creating the event, NULL if not created from a module
//
// Create a new synthetic event with the given name under the
// trace/events/synthetic/ directory.  The event fields that will be
// defined for the event should be passed in as an array of struct
// synth_field_desc, and the number elements in the array passed in as
// n_fields. Field ordering will retain the ordering given in the
// fields array.
//
// If the new synthetic event is being created from a module, the mod
// param must be non-NULL.  This will ensure that the trace buffer
// won't contain unreadable events.
//
// The new synth event should be deleted using synth_event_delete()
// function.  The new synthetic event can be generated from modules or
// other kernel code using trace_synth_event() and related functions.
//
// Return: 0 if successful, error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn synth_event_create(name: *mut c_char, fields: *mut synth_field_desc, n_fields: c_uint, mod: *mut module) -> c_int {
pub static mut cmd: usize = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL);
    if (!buf) {
    return -ENOMEM;
    }
    synth_event_cmd_init(&cmd, buf, MAX_DYNEVENT_CMD_LEN);
    ret = synth_event_gen_cmd_array_start(&cmd, name, mod,
    fields, n_fields);
    if (ret) {
// goto;
    }
    ret = synth_event_gen_cmd_end(&cmd);
// label;
    kfree(buf);
    return ret;
    }
    EXPORT_SYMBOL_GPL(synth_event_create);
#[no_mangle]
unsafe extern "C" fn destroy_synth_event(se: *mut synth_event) -> c_int {
    let mut ret = 0;
    if (se.ref) {
    return -EBUSY;
    }
    if (trace_event_dyn_busy(&se.call)) {
    return -EBUSY;
    }
    ret = unregister_synth_event(se);
    if (!ret) {
    dyn_event_remove(&se.devent);
    free_synth_event(se);
    }
    return ret;
    }
//
// synth_event_delete - Delete a synthetic event
// @event_name: The name of the new synthetic event
//
// Delete a synthetic event that was created with synth_event_create().
//
// Return: 0 if successful, error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn synth_event_delete(event_name: *const c_char) -> c_int {
    let mut se = core::ptr::null_mut();
    let mut mod = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    mutex_lock(&event_mutex);
    se = find_synth_event(event_name);
    if (se) {
    mod = se.mod;
    ret = destroy_synth_event(se);
    }
    mutex_unlock(&event_mutex);
    if (mod) {
//
// It is safest to reset the ring buffer if the module
// being unloaded registered any events that were
// used. The only worry is if a new module gets
// loaded, and takes on the same id as the events of
// this module. When printing out the buffer, traced
// events left over from this module may be passed to
// the new module events and unexpected results may
// occur.
//
    tracing_reset_all_online_cpus();
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(synth_event_delete);
#[no_mangle]
unsafe extern "C" fn check_command(raw_command: *const c_char) -> c_int {
    let mut argv = core::ptr::null_mut(), *cmd, *saved_cmd, *name_and_field;
    int argc, ret = 0;
    cmd = saved_cmd = kstrdup(raw_command, GFP_KERNEL);
    if (!cmd) {
    return -ENOMEM;
    }
    name_and_field = strsep(&cmd, ";");
    if (!name_and_field) {
    ret = -EINVAL;
// goto;
    }
    if (name_and_field[0] == '!') {
// goto;
    }
    argv = argv_split(GFP_KERNEL, name_and_field, &argc);
    if (!argv) {
    ret = -ENOMEM;
// goto;
    }
    argv_free(argv);
    if (argc < 3) {
    ret = -EINVAL;
    }
// label;
    kfree(saved_cmd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn create_or_delete_synth_event(raw_command: *const c_char) -> c_int {
    let mut name = core::ptr::null_mut(), *fields, *p;
pub static mut ret: c_int = 0;
    raw_command = skip_spaces(raw_command);
    if (raw_command[0] == '\0') {
    return ret;
    }
    last_cmd_set(raw_command);
    ret = check_command(raw_command);
    if (ret) {
    synth_err(SYNTH_ERR_INVALID_CMD, 0);
    return ret;
    }
    p = strpbrk(raw_command, " \t");
    if (!p && raw_command[0] != '!') {
    synth_err(SYNTH_ERR_INVALID_CMD, 0);
    ret = -EINVAL;
// goto;
    }
    name = kmemdup_nul(raw_command, p ? p - raw_command : strlen(raw_command), GFP_KERNEL);
    if (!name) {
    return -ENOMEM;
    }
    if (name[0] == '!') {
    ret = synth_event_delete(name + 1);
// goto;
    }
    fields = skip_spaces(p);
    ret = __create_synth_event(name, fields);
// label;
    kfree(name);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn synth_event_run_command(cmd: *mut dynevent_cmd) -> c_int {
pub static mut se: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ret = create_or_delete_synth_event(cmd.seq.buffer);
    if (ret) {
    return ret;
    }
    se = find_synth_event(cmd.event_name);
    if (WARN_ON!(!se)) {
    return -ENOENT;
    }
    se.mod = cmd.private_data;
    return ret;
    }
//
// synth_event_cmd_init - Initialize a synthetic event command object
// @cmd: A pointer to the dynevent_cmd struct representing the new event
// @buf: A pointer to the buffer used to build the command
// @maxlen: The length of the buffer passed in @buf
//
// Initialize a synthetic event command object.  Use this before
// calling any of the other dyenvent_cmd functions.
//
#[no_mangle]
pub unsafe extern "C" fn synth_event_cmd_init(cmd: *mut dynevent_cmd, buf: *mut c_char, maxlen: c_int) {
    dynevent_cmd_init(cmd, buf, maxlen, DYNEVENT_TYPE_SYNTH,
    synth_event_run_command);
    }
    EXPORT_SYMBOL_GPL(synth_event_cmd_init);
#[no_mangle]
pub unsafe extern "C" fn __synth_event_trace_init(file: *mut trace_event_file, trace_state: *mut synth_event_trace_state) -> c_int {
pub static mut ret: c_int = 0;
    memset(trace_state, '\0', sizeof!(*trace_state));
//
// Normal event tracing doesn't get called at all unless the
// ENABLED bit is set (which attaches the probe thus allowing
// this code to be called, etc).  Because this is called
// directly by the user, we don't have that but we still need
// to honor not logging when disabled.  For the iterated
// trace case, we save the enabled state upon start and just
// ignore the following data calls.
//
    if (!(file.flags & EVENT_FILE_FL_ENABLED) ||
    trace_trigger_soft_disabled(file)) {
    trace_state.disabled = true;
    ret = -ENOENT;
// goto;
    }
    trace_state.event = file.event_call.data;
// label;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __synth_event_trace_start(file: *mut trace_event_file, trace_state: *mut synth_event_trace_state, dynamic_fields_size: c_int) -> c_int {
    int entry_size, fields_size = 0;
pub static mut ret: c_int = 0;
    fields_size = trace_state.event.n_u64 * sizeof!(u64);
    fields_size += dynamic_fields_size;
//
// Avoid ring buffer recursion detection, as this event
// is being performed within another event.
//
    trace_state.buffer = file.tr.array_buffer.buffer;
    ring_buffer_nest_start(trace_state.buffer);
    entry_size = sizeof!(*trace_state.entry) + fields_size;
    trace_state.entry = trace_event_buffer_reserve(&trace_state.fbuffer,
    file,
    entry_size);
    if (!trace_state.entry) {
    ring_buffer_nest_end(trace_state.buffer);
    ret = -EINVAL;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __synth_event_trace_end(trace_state: *mut synth_event_trace_state) {
    trace_event_buffer_commit(&trace_state.fbuffer);
    ring_buffer_nest_end(trace_state.buffer);
    }
//
// synth_event_trace - Trace a synthetic event
// @file: The trace_event_file representing the synthetic event
// @n_vals: The number of values in vals
// @...: Variable number of args containing the event values
//
// Trace a synthetic event using the values passed in the variable
// argument list.
//
// The argument list should be a list 'n_vals' u64 values.  The number
// of vals must match the number of field in the synthetic event, and
// must be in the same order as the synthetic event fields.
//
// All vals should be cast to u64, and string vals are just pointers
// to strings, cast to u64.  Strings will be copied into space
// reserved in the event for the string, using these pointers.
//
// Return: 0 on success, err otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn synth_event_trace(file: *mut trace_event_file, n_vals: c_uint, ...) -> c_int {
    unsigned int i, n_u64, len, data_size = 0;
pub static mut state: usize = 0;
    let mut args;
    let mut ret = 0;
    ret = __synth_event_trace_init(file, &state);
    if (ret) {
    if (ret == -ENOENT) {
    ret = 0; /* just disabled, not really an error */
    }
    return ret;
    }
    if (state.event.n_dynamic_fields) {
    va_start(args, n_vals);
    while (i < state.event.n_fields) {
pub static mut val: u64 = 0;
    if (state.event.fields[i].is_string &&
    state.event.fields[i].is_dynamic) {
    let mut str_val = (long)val;
    data_size += strlen(str_val) + 1;
    }
    }
    va_end(args);
    }
    ret = __synth_event_trace_start(file, &state, data_size);
    if (ret) {
    return ret;
    }
    if (n_vals != state.event.n_fields) {
    ret = -EINVAL;
// goto;
    }
    data_size = 0;
    va_start(args, n_vals);
    while (i < state.event.n_fields) {
    let mut val = 0;
    val = va_arg(args, u64);
    if (state.event.fields[i].is_string) {
    let mut str_val = (long)val;
    len = trace_string(state.entry, state.event, str_val,
    state.event.fields[i].is_dynamic,
    data_size, &n_u64);
    data_size += len; /* only dynamic string increments */
    } else {
    let mut field = state.event.fields[i];
    match (field.size) {
    1 => {
    state.entry.fields[n_u64].as_u8 = (u8)val;
    // break;
    }
    2 => {
    state.entry.fields[n_u64].as_u16 = (u16)val;
    // break;
    }
    4 => {
    state.entry.fields[n_u64].as_u32 = (u32)val;
    // break;
    }
    _ => {
    state.entry.fields[n_u64].as_u64 = val;
    // break;
    }
    }
    n_u64 += 1;
    }
    }
    va_end(args);
// label;
    __synth_event_trace_end(&state);
    return ret;
    }
    EXPORT_SYMBOL_GPL(synth_event_trace);
//
// synth_event_trace_array - Trace a synthetic event from an array
// @file: The trace_event_file representing the synthetic event
// @vals: Array of values
// @n_vals: The number of values in vals
//
// Trace a synthetic event using the values passed in as 'vals'.
//
// The 'vals' array is just an array of 'n_vals' u64.  The number of
// vals must match the number of field in the synthetic event, and
// must be in the same order as the synthetic event fields.
//
// All vals should be cast to u64, and string vals are just pointers
// to strings, cast to u64.  Strings will be copied into space
// reserved in the event for the string, using these pointers.
//
// Return: 0 on success, err otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn synth_event_trace_array(file: *mut trace_event_file, vals: *mut u64, n_vals: c_uint) -> c_int {
    unsigned int i, n_u64, field_pos, len, data_size = 0;
pub static mut state: usize = 0;
pub static mut str_val: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ret = __synth_event_trace_init(file, &state);
    if (ret) {
    if (ret == -ENOENT) {
    ret = 0; /* just disabled, not really an error */
    }
    return ret;
    }
    if (state.event.n_dynamic_fields) {
    while (i < state.event.n_dynamic_fields) {
    field_pos = state.event.dynamic_fields[i].field_pos;
    str_val = (long)vals[field_pos];
    len = strlen(str_val) + 1;
    data_size += len;
    }
    }
    ret = __synth_event_trace_start(file, &state, data_size);
    if (ret) {
    return ret;
    }
    if (n_vals != state.event.n_fields) {
    ret = -EINVAL;
// goto;
    }
    data_size = 0;
    while (i < state.event.n_fields) {
    if (state.event.fields[i].is_string) {
    let mut str_val = (long)vals[i];
    len = trace_string(state.entry, state.event, str_val,
    state.event.fields[i].is_dynamic,
    data_size, &n_u64);
    data_size += len; /* only dynamic string increments */
    } else {
    let mut field = state.event.fields[i];
pub static mut val: u64 = 0;
    match (field.size) {
    1 => {
    state.entry.fields[n_u64].as_u8 = (u8)val;
    // break;
    }
    2 => {
    state.entry.fields[n_u64].as_u16 = (u16)val;
    // break;
    }
    4 => {
    state.entry.fields[n_u64].as_u32 = (u32)val;
    // break;
    }
    _ => {
    state.entry.fields[n_u64].as_u64 = val;
    // break;
    }
    }
    n_u64 += 1;
    }
    }
// label;
    __synth_event_trace_end(&state);
    return ret;
    }
    EXPORT_SYMBOL_GPL(synth_event_trace_array);
//
// synth_event_trace_start - Start piecewise synthetic event trace
// @file: The trace_event_file representing the synthetic event
// @trace_state: A pointer to object tracking the piecewise trace state
//
// Start the trace of a synthetic event field-by-field rather than all
// at once.
//
// This function 'opens' an event trace, which means space is reserved
// for the event in the trace buffer, after which the event's
// individual field values can be set through either
// synth_event_add_next_val() or synth_event_add_val().
//
// A pointer to a trace_state object is passed in, which will keep
// track of the current event trace state until the event trace is
// closed (and the event finally traced) using
// synth_event_trace_end().
//
// Note that synth_event_trace_end() must be called after all values
// have been added for each event trace, regardless of whether adding
// all field values succeeded or not.
//
// Note also that for a given event trace, all fields must be added
// using either synth_event_add_next_val() or synth_event_add_val()
// but not both together or interleaved.
//
// Return: 0 on success, err otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn synth_event_trace_start(file: *mut trace_event_file, trace_state: *mut synth_event_trace_state) -> c_int {
    let mut ret = 0;
    if (!trace_state) {
    return -EINVAL;
    }
    ret = __synth_event_trace_init(file, trace_state);
    if (ret) {
    if (ret == -ENOENT) {
    ret = 0; /* just disabled, not really an error */
    }
    return ret;
    }
    if (trace_state.event.n_dynamic_fields) {
    return -ENOTSUPP;
    }
    ret = __synth_event_trace_start(file, trace_state, 0);
    return ret;
    }
    EXPORT_SYMBOL_GPL(synth_event_trace_start);
#[no_mangle]
pub unsafe extern "C" fn __synth_event_add_val(field_name: *mut c_char, val: u64, trace_state: *mut synth_event_trace_state) -> c_int {
    let mut field = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
    int i, ret = 0;
    if (!trace_state) {
    ret = -EINVAL;
// goto;
    }
// can't mix add_next_synth_val() with add_synth_val()
    if (field_name) {
    if (trace_state.add_next) {
    ret = -EINVAL;
// goto;
    }
    trace_state.add_name = true;
    } else {
    if (trace_state.add_name) {
    ret = -EINVAL;
// goto;
    }
    trace_state.add_next = true;
    }
    if (trace_state.disabled) {
// goto;
    }
    event = trace_state.event;
    if (trace_state.add_name) {
    while (i < event.n_fields) {
    field = event.fields[i];
    if (strcmp(field.name, field_name) == 0) {
    break;
    }
    }
    if (!field) {
    ret = -EINVAL;
// goto;
    }
    } else {
    if (trace_state.cur_field >= event.n_fields) {
    ret = -EINVAL;
// goto;
    }
    field = event.fields[trace_state.cur_field++];
    }
    entry = trace_state.entry;
    if (field.is_string) {
    let mut str_val = (long)val;
pub static mut str_field: *mut c_void = core::ptr::null_mut();
    if (field.is_dynamic) { /* add_val can't do dynamic strings */ {
    ret = -EINVAL;
    }
// goto;
    }
    if (!str_val) {
    ret = -EINVAL;
// goto;
    }
    str_field = &entry.fields[field.offset];
    strscpy(str_field, str_val, STR_VAR_LEN_MAX);
    } else {
    match (field.size) {
    1 => {
    trace_state.entry.fields[field.offset].as_u8 = (u8)val;
    // break;
    }
    2 => {
    trace_state.entry.fields[field.offset].as_u16 = (u16)val;
    // break;
    }
    4 => {
    trace_state.entry.fields[field.offset].as_u32 = (u32)val;
    // break;
    }
    _ => {
    trace_state.entry.fields[field.offset].as_u64 = val;
    // break;
    }
    }
    }
// label;
    return ret;
    }
//
// synth_event_add_next_val - Add the next field's value to an open synth trace
// @val: The value to set the next field to
// @trace_state: A pointer to object tracking the piecewise trace state
//
// Set the value of the next field in an event that's been opened by
// synth_event_trace_start().
//
// The val param should be the value cast to u64.  If the value points
// to a string, the val param should be a char * cast to u64.
//
// This function assumes all the fields in an event are to be set one
// after another - successive calls to this function are made, one for
// each field, in the order of the fields in the event, until all
// fields have been set.  If you'd rather set each field individually
// without regard to ordering, synth_event_add_val() can be used
// instead.
//
// Note however that synth_event_add_next_val() and
// synth_event_add_val() can't be intermixed for a given event trace -
// one or the other but not both can be used at the same time.
//
// Note also that synth_event_trace_end() must be called after all
// values have been added for each event trace, regardless of whether
// adding all field values succeeded or not.
//
// Return: 0 on success, err otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn synth_event_add_next_val(val: u64, trace_state: *mut synth_event_trace_state) -> c_int {
    return __synth_event_add_val(core::ptr::null_mut(), val, trace_state);
    }
    EXPORT_SYMBOL_GPL(synth_event_add_next_val);
//
// synth_event_add_val - Add a named field's value to an open synth trace
// @field_name: The name of the synthetic event field value to set
// @val: The value to set the named field to
// @trace_state: A pointer to object tracking the piecewise trace state
//
// Set the value of the named field in an event that's been opened by
// synth_event_trace_start().
//
// The val param should be the value cast to u64.  If the value points
// to a string, the val param should be a char * cast to u64.
//
// This function looks up the field name, and if found, sets the field
// to the specified value.  This lookup makes this function more
// expensive than synth_event_add_next_val(), so use that or the
// none-piecewise synth_event_trace() instead if efficiency is more
// important.
//
// Note however that synth_event_add_next_val() and
// synth_event_add_val() can't be intermixed for a given event trace -
// one or the other but not both can be used at the same time.
//
// Note also that synth_event_trace_end() must be called after all
// values have been added for each event trace, regardless of whether
// adding all field values succeeded or not.
//
// Return: 0 on success, err otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn synth_event_add_val(field_name: *mut c_char, val: u64, trace_state: *mut synth_event_trace_state) -> c_int {
    return __synth_event_add_val(field_name, val, trace_state);
    }
    EXPORT_SYMBOL_GPL(synth_event_add_val);
//
// synth_event_trace_end - End piecewise synthetic event trace
// @trace_state: A pointer to object tracking the piecewise trace state
//
// End the trace of a synthetic event opened by
// synth_event_trace__start().
//
// This function 'closes' an event trace, which basically means that
// it commits the reserved event and cleans up other loose ends.
//
// A pointer to a trace_state object is passed in, which will keep
// track of the current event trace state opened with
// synth_event_trace_start().
//
// Note that this function must be called after all values have been
// added for each event trace, regardless of whether adding all field
// values succeeded or not.
//
// Return: 0 on success, err otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn synth_event_trace_end(trace_state: *mut synth_event_trace_state) -> c_int {
    if (!trace_state) {
    return -EINVAL;
    }
    __synth_event_trace_end(trace_state);
    return 0;
    }
    EXPORT_SYMBOL_GPL(synth_event_trace_end);
#[no_mangle]
unsafe extern "C" fn create_synth_event(raw_command: *const c_char) -> c_int {
    let mut fields = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
pub static mut name: *mut c_void = core::ptr::null_mut();
    int len, ret = 0;
    raw_command = skip_spaces(raw_command);
    if (raw_command[0] == '\0') {
    return ret;
    }
    last_cmd_set(raw_command);
    name = raw_command;
// Don't try to process if not our system
    if (name[0] != 's' || name[1] != ':') {
    return -ECANCELED;
    }
    name += 2;
    p = strpbrk(raw_command, " \t");
    if (!p) {
    synth_err(SYNTH_ERR_INVALID_CMD, 0);
    return -EINVAL;
    }
    fields = skip_spaces(p);
// This interface accepts group name prefix
    if (strchr(name, '/')) {
    len = str_has_prefix(name, SYNTH_SYSTEM "/");
    if (len == 0) {
    synth_err(SYNTH_ERR_INVALID_DYN_CMD, 0);
    return -EINVAL;
    }
    name += len;
    }
    len = name - raw_command;
    ret = check_command(raw_command + len);
    if (ret) {
    synth_err(SYNTH_ERR_INVALID_CMD, 0);
    return ret;
    }
    name = kmemdup_nul(raw_command + len, p - raw_command - len, GFP_KERNEL);
    if (!name) {
    return -ENOMEM;
    }
    ret = __create_synth_event(name, fields);
    kfree(name);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn synth_event_release(ev: *mut dyn_event) -> c_int {
    let mut event = to_synth_event(ev);
    let mut ret = 0;
    if (event.ref) {
    return -EBUSY;
    }
    if (trace_event_dyn_busy(&event.call)) {
    return -EBUSY;
    }
    ret = unregister_synth_event(event);
    if (ret) {
    return ret;
    }
    dyn_event_remove(ev);
    free_synth_event(event);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __synth_event_show(m: *mut seq_file, event: *mut synth_event) -> c_int {
pub static mut field: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut type = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    seq_printf(m, "%s\t", event.name);
    while (i < event.n_fields) {
    field = event.fields[i];
    type = field.type;
    t = strstr(type, "__data_loc");
    if (t) { /* __data_loc belongs in format but not event desc */ {
    t += sizeof!("__data_loc");
    }
    type = t;
    }
// parameter values
    seq_printf(m, "%s %s%s", type, field.name,
    i == event.n_fields - 1 ? "" : "; ");
    }
    seq_putc(m, '\n');
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn synth_event_show(m: *mut seq_file, ev: *mut dyn_event) -> c_int {
    let mut event = to_synth_event(ev);
    seq_printf(m, "s:%s/", event.class.system);
    return __synth_event_show(m, event);
    }
#[no_mangle]
unsafe extern "C" fn synth_events_seq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut ev = v;
    if (!is_synth_event(ev)) {
    return 0;
    }
    return __synth_event_show(m, to_synth_event(ev));
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn synth_events_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut ret = 0;
    ret = security_locked_down(LOCKDOWN_TRACEFS);
    if (ret) {
    return ret;
    }
    if ((file.f_mode & FMODE_WRITE) && (file.f_flags & O_TRUNC)) {
    ret = dyn_events_release_all(&synth_event_ops);
    if (ret < 0) {
    return ret;
    }
    }
    return seq_open(file, &synth_events_seq_op);
    }
#[no_mangle]
pub unsafe extern "C" fn synth_events_write(file: *mut file, buffer: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    return trace_parse_run_command(file, buffer, count, ppos,
    create_or_delete_synth_event);
    }
pub static mut file_operations: usize = 0;
//
// Register dynevent at core_initcall. This allows kernel to setup kprobe
// events in postcore_initcall without tracefs.
//
#[no_mangle]
unsafe extern "C" fn trace_events_synth_init_early() -> __init int {
pub static mut err: c_int = 0;
    err = dyn_event_register(&synth_event_ops);
    if (err) {
    pr_warn!("Could not register synth_event_ops\n");
    }
    return err;
    }
    core_initcall!(trace_events_synth_init_early);
#[no_mangle]
unsafe extern "C" fn trace_events_synth_init() -> __init int {
    let mut entry = core::ptr::null_mut();
pub static mut err: c_int = 0;
    err = tracing_init_dentry();
    if (err) {
// goto;
    }
    entry = tracefs_create_file("synthetic_events", TRACE_MODE_WRITE,
    core::ptr::null_mut(), core::ptr::null_mut(), &synth_events_fops);
    if (!entry) {
    err = -ENODEV;
// goto;
    }
    return err;
// label;
    pr_warn!("Could not create tracefs 'synthetic_events' entry\n");
    return err;
    }
    fs_initcall!(trace_events_synth_init);
//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_eprobe.c
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
// event probes
//
// Part of this code was copied from kernel/trace/trace_kprobe.c written by
// Masami Hiramatsu <mhiramat@kernel.org>
//
// Copyright (C) 2021, VMware Inc, Steven Rostedt <rostedt@goodmis.org>
// Copyright (C) 2021, VMware Inc, Tzvetomir Stoyanov tz.stoyanov@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_eprobe {
// tracepoint system
    pub event_system: *const c_char,
// tracepoint event
    pub event_name: *const c_char,
// filter string for the tracepoint
    pub filter_str: *mut c_char,
    pub event: *mut trace_event_call,
    pub devent: dyn_event,
    pub tp: trace_probe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eprobe_data {
    pub file: *mut trace_event_file,
    pub ep: *mut trace_eprobe,
}

    list_for_each_entry(ep, trace_probe_probe_list(_tp), tp.list) {
// forward_decl: __trace_eprobe_create;
    }
#[no_mangle]
unsafe extern "C" fn trace_event_probe_cleanup(ep: *mut trace_eprobe) {
    if (!ep) {
    return;
    }
    trace_probe_cleanup(&ep.tp);
    kfree(ep.event_name);
    kfree(ep.event_system);
    if (ep.event) {
    trace_event_put_ref(ep.event);
    }
    kfree(ep.filter_str);
    kfree(ep);
    }
    DEFINE_FREE(trace_event_probe_cleanup, trace_eprobe *,
    if (!IS_ERR_OR_NULL(_T)) trace_event_probe_cleanup(_T))
#[no_mangle]
pub unsafe extern "C" fn to_trace_eprobe(ev: *mut dyn_event) -> *mut c_void {
    return container_of!(ev, trace_eprobe, devent);
    }
#[no_mangle]
unsafe extern "C" fn eprobe_dyn_event_create(raw_command: *const c_char) -> c_int {
    return trace_probe_create(raw_command, __trace_eprobe_create);
    }
#[no_mangle]
unsafe extern "C" fn eprobe_dyn_event_show(m: *mut seq_file, ev: *mut dyn_event) -> c_int {
    let mut ep = to_trace_eprobe(ev);
    let mut i = 0;
    seq_printf(m, "e:%s/%s", trace_probe_group_name(&ep.tp), {
    trace_probe_name(&ep.tp));
    }
    seq_printf(m, " %s.%s", ep.event_system, ep.event_name);
    for (i = 0; i < ep.tp.nr_args; i++) {
    seq_printf(m, " %s=%s", ep.tp.args[i].name, ep.tp.args[i].comm);
    }
    seq_putc(m, '\n');
    trace_probe_dump_args(m, &ep.tp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn unregister_trace_eprobe(ep: *mut trace_eprobe) -> c_int {
// If other probes are on the event, just unregister eprobe
    if (trace_probe_has_sibling(&ep.tp)) {
// goto;
    }
// Enabled event can not be unregistered
    if (trace_probe_is_enabled(&ep.tp)) {
    return -EBUSY;
    }
// Will fail if probe is being used by ftrace or perf
    if (trace_probe_unregister_event_call(&ep.tp)) {
    return -EBUSY;
    }
// label;
    dyn_event_remove(&ep.devent);
    trace_probe_unlink(&ep.tp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eprobe_dyn_event_release(ev: *mut dyn_event) -> c_int {
    let mut ep = to_trace_eprobe(ev);
pub static mut ret: c_int = 0;
    if (!ret) {
    trace_event_probe_cleanup(ep);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn eprobe_dyn_event_is_busy(ev: *mut dyn_event) -> bool {
    let mut ep = to_trace_eprobe(ev);
    return trace_probe_is_enabled(&ep.tp);
    }
#[no_mangle]
pub unsafe extern "C" fn eprobe_dyn_event_match(system: *mut c_char, event: *mut c_char, argc: c_int, argv: *mut *mut c_char, ev: *mut dyn_event) -> bool {
    let mut ep = to_trace_eprobe(ev);
pub static mut slash: *mut c_void = core::ptr::null_mut();
//
// We match the following:
// event only			- match all eprobes with event name
// system and event only	- match all system/event probes
// system only			- match all system probes
//
// The below has the above satisfied with more arguments:
//
// attached system/event	- If the arg has the system and event
// the probe is attached to, match
// probes with the attachment.
//
// If any more args are given, then it requires a full match.
//
// If system exists, but this probe is not part of that system
// do not match.
//
    if (system && strcmp(trace_probe_group_name(&ep.tp), system) != 0) {
    return false;
    }
// Must match the event name
    if (event[0] != '\0' && strcmp(trace_probe_name(&ep.tp), event) != 0) {
    return false;
    }
// No arguments match all
    if (argc < 1) {
    return true;
    }
// First argument is the system/event the probe is attached to
    slash = strchr(argv[0], '/');
    if (!slash) {
    slash = strchr(argv[0], '.');
    }
    if (!slash) {
    return false;
    }
    if (strncmp(ep.event_system, argv[0], slash - argv[0]) ||
    ep.event_system[slash - argv[0]] != '\0') {
    return false;
    }
    if (strcmp(ep.event_name, slash + 1)) {
    return false;
    }
    argc -= 1;
    argv += 1;
// If there are no other args, then match
    if (argc < 1) {
    return true;
    }
    return trace_probe_match_command_args(&ep.tp, argc, argv);
    }
pub static mut dyn_event_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn alloc_event_probe(group: *mut c_char, this_event: *mut c_char, event: *mut trace_event_call, nargs: c_int) -> *mut c_void {
    struct trace_eprobe *ep __free(trace_event_probe_cleanup) = core::ptr::null_mut();
pub static mut event_name: *mut c_void = core::ptr::null_mut();
pub static mut sys_name: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!event) {
    return ERR_PTR(-ENODEV);
    }
    sys_name = event.class.system;
    event_name = trace_event_name(event);
    ep = kzalloc_flex(*ep, tp.args, nargs);
    if (!ep) {
    trace_event_put_ref(event);
    return ERR_PTR(-ENOMEM);
    }
    ep.event = event;
    ep.event_name = kstrdup(event_name, GFP_KERNEL);
    if (!ep.event_name) {
    return ERR_PTR(-ENOMEM);
    }
    ep.event_system = kstrdup(sys_name, GFP_KERNEL);
    if (!ep.event_system) {
    return ERR_PTR(-ENOMEM);
    }
    ret = trace_probe_init(&ep.tp, this_event, group, false, nargs);
    if (ret < 0) {
    return ERR_PTR(ret);
    }
    dyn_event_init(&ep.devent, &eprobe_dyn_event_ops);
    return_ptr(ep);
    }
#[no_mangle]
unsafe extern "C" fn eprobe_event_define_fields(event_call: *mut trace_event_call) -> c_int {
pub static mut field: usize = 0;
pub static mut tp: *mut c_void = core::ptr::null_mut();
    tp = trace_probe_primary_from_call(event_call);
    if (WARN_ON_ONCE!(!tp)) {
    return -ENOENT;
    }
    return traceprobe_define_arg_fields(event_call, sizeof!(field), tp);
    }
pub static mut trace_event_fields: usize = 0;
// Event entry printers
    static enum print_line_t
    print_eprobe_event(trace_iterator *iter, int flags, trace_event *event)
    {
pub static mut field: *mut c_void = core::ptr::null_mut();
pub static mut pevent: *mut c_void = core::ptr::null_mut();
pub static mut probed_event: *mut c_void = core::ptr::null_mut();
    let mut s = &iter.seq;
pub static mut ep: *mut c_void = core::ptr::null_mut();
pub static mut tp: *mut c_void = core::ptr::null_mut();
    let mut type = 0;
    field = iter.ent;
    tp = trace_probe_primary_from_call(
    container_of!(event, trace_event_call, event));
    if (WARN_ON_ONCE!(!tp)) {
// goto;
    }
    ep = container_of!(tp, trace_eprobe, tp);
    type = ep.event.event.type;
    trace_seq_printf(s, "%s: (", trace_probe_name(tp));
    probed_event = ftrace_find_event(type);
    if (probed_event) {
    pevent = container_of!(probed_event, trace_event_call, event);
    trace_seq_printf(s, "%s.%s", pevent.class.system,
    trace_event_name(pevent));
    } else {
    trace_seq_printf(s, "%u", type);
    }
    trace_seq_putc(s, ')');
    if (trace_probe_print_args(s, tp.args, tp.nr_args,
    &field[1], field) < 0) {
// goto;
    }
    trace_seq_putc(s, '\n');
// label;
    return trace_handle_return(s);
    }
    static nokprobe_inline unsigned long
    get_event_field(fetch_insn *code, void *rec)
    {
    let mut field = code.data;
    let mut val = 0;
pub static mut addr: *mut c_void = core::ptr::null_mut();
    addr = rec + field.offset;
    if (is_string_field(field)) {
    match (field.filter_type) {
    FILTER_DYN_STRING => {
    val = (unsigned long)(rec + (*addr & 0xffff));
    // break;
    }
    FILTER_RDYN_STRING => {
    val = (unsigned long)(addr + (*addr & 0xffff));
    // break;
    }
    FILTER_STATIC_STRING => {
    val = (unsigned long)addr;
    // break;
    }
    FILTER_PTR_STRING => {
    val = *addr;
    // break;
    }
    _ => {
    WARN_ON_ONCE!(1);
    return 0;
    }
    }
    return val;
    }
    match (field.size) {
    1 => {
    if (field.is_signed) {
    val = *addr;
    }
    else {
    val = *addr;
    }
    // break;
    }
    2 => {
    if (field.is_signed) {
    val = *addr;
    }
    else {
    val = *addr;
    }
    // break;
    }
    4 => {
    if (field.is_signed) {
    val = *addr;
    }
    else {
    val = *addr;
    }
    // break;
    }
    _ => {
    if (field.size == sizeof!(long)) {
    if (field.is_signed) {
    val = *addr;
    }
    else {
    val = *addr;
    }
    // break;
    }
// This is an array, point to the addr itself
    val = (unsigned long)addr;
    // break;
    }
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn get_eprobe_size(tp: *mut trace_probe, rec: *mut c_void) -> c_int {
pub static mut code: *mut c_void = core::ptr::null_mut();
pub static mut arg: *mut c_void = core::ptr::null_mut();
    int i, len, ret = 0;
    while (i < tp.nr_args) {
    arg = tp.args + i;
    if (arg.dynamic) {
    let mut val = 0;
    code = arg.code;
// label;
    match (code.op) {
    FETCH_OP_TP_ARG => {
    val = get_event_field(code, rec);
    // break;
    }
    FETCH_NOP_SYMBOL => {
    code += 1;
// goto;
    }
    _ => {
    if (process_common_fetch_insn(code, &val) < 0) {
    continue;
    }
    }
    }
    code += 1;
    len = process_fetch_insn_bottom(code, val, core::ptr::null_mut(), core::ptr::null_mut());
    if (len > 0) {
    ret += len;
    }
    }
    }
    return ret;
    }
// Kprobe specific fetch functions
// Note that we don't verify it, since the code does not come from user space
#[no_mangle]
pub unsafe extern "C" fn process_fetch_insn(code: *mut fetch_insn, rec: *mut c_void, edata: *mut c_void, dest: *mut c_void, base: *mut c_void) -> c_int {
    let mut val = 0;
    let mut ret = 0;
// label;
    match (code.op) {
    FETCH_OP_TP_ARG => {
    val = get_event_field(code, rec);
    // break;
    }
    FETCH_NOP_SYMBOL => {
    code += 1;
// goto;
    }
    _ => {
    ret = process_common_fetch_insn(code, &val);
    if (ret < 0) {
    return ret;
    }
    }
    }
    code += 1;
    return process_fetch_insn_bottom(code, val, dest, base);
    }
    NOKPROBE_SYMBOL(process_fetch_insn)
// eprobe handler
#[no_mangle]
pub unsafe extern "C" fn __eprobe_trace_func(edata: *mut eprobe_data, rec: *mut c_void) {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut call = trace_probe_event_call(&edata.ep.tp);
pub static mut fbuffer: usize = 0;
    let mut dsize = 0;
    if (WARN_ON_ONCE!(call != edata.file.event_call)) {
    return;
    }
    if (trace_trigger_soft_disabled(edata.file)) {
    return;
    }
    dsize = get_eprobe_size(&edata.ep.tp, rec);
    entry = trace_event_buffer_reserve(&fbuffer, edata.file,
    sizeof!(*entry) + edata.ep.tp.size + dsize);
    if (!entry) {
    return;
    }
    entry = fbuffer.entry = ring_buffer_event_data(fbuffer.event);
    store_trace_args(&entry[1], &edata.ep.tp, rec, core::ptr::null_mut(), sizeof!(*entry), dsize);
    trace_event_buffer_commit(&fbuffer);
    }
//
// The event probe implementation uses event triggers to get access to
// the event it is attached to, but is not an actual trigger. The below
// functions are just stubs to fulfill what is needed to use the trigger
// infrastructure.
//
#[no_mangle]
unsafe extern "C" fn eprobe_trigger_init(data: *mut event_trigger_data) -> c_int {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn eprobe_trigger_free(data: *mut event_trigger_data) {
    }
#[no_mangle]
pub unsafe extern "C" fn eprobe_trigger_print(m: *mut seq_file, data: *mut event_trigger_data) -> c_int {
// Do not print eprobe event triggers
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn eprobe_trigger_func(data: *mut event_trigger_data, buffer: *mut trace_buffer, rec: *mut c_void, rbe: *mut ring_buffer_event) {
    let mut edata = data.private_data;
    if (unlikely(!rec)) {
    return;
    }
    __eprobe_trace_func(edata, rec);
    }
#[no_mangle]
pub unsafe extern "C" fn eprobe_trigger_cmd_parse(cmd_ops: *mut event_command, file: *mut trace_event_file, glob: *mut c_char, cmd: *mut c_char, param_and_filter: *mut c_char) -> c_int {
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn eprobe_trigger_reg_func(glob: *mut c_char, data: *mut event_trigger_data, file: *mut trace_event_file) -> c_int {
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn eprobe_trigger_unreg_func(glob: *mut c_char, data: *mut event_trigger_data, file: *mut trace_event_file) {
    }
pub static mut event_command: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn new_eprobe_trigger(ep: *mut trace_eprobe, file: *mut trace_event_file) -> *mut c_void {
pub static mut trigger: *mut c_void = core::ptr::null_mut();
    let mut filter = core::ptr::null_mut();
pub static mut edata: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    edata = kzalloc_obj(*edata);
    trigger = kzalloc_obj(*trigger);
    if (!trigger || !edata) {
    ret = -ENOMEM;
// goto;
    }
    trigger.flags = EVENT_TRIGGER_FL_PROBE;
    trigger.count = -1;
//
// EVENT PROBE triggers are not registered as commands with
// register_event_command(), as they are not controlled by the user
// from the trigger file
//
    trigger.cmd_ops = &event_trigger_cmd;
    INIT_LIST_HEAD(&trigger.list);
    if (ep.filter_str) {
    ret = create_event_filter(file.tr, ep.event,
    ep.filter_str, false, &filter);
    if (ret) {
// goto;
    }
    }
    RCU_INIT_POINTER(trigger.filter, filter);
    edata.file = file;
    edata.ep = ep;
    trigger.private_data = edata;
    return trigger;
// label;
    free_event_filter(filter);
    kfree(edata);
    kfree(trigger);
    return ERR_PTR(ret);
    }
#[no_mangle]
pub unsafe extern "C" fn enable_eprobe(ep: *mut trace_eprobe, eprobe_file: *mut trace_event_file) -> c_int {
pub static mut trigger: *mut c_void = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut tr = eprobe_file.tr;
    file = find_event_file(tr, ep.event_system, ep.event_name);
    if (!file) {
    return -ENOENT;
    }
    trigger = new_eprobe_trigger(ep, eprobe_file);
    if (IS_ERR(trigger)) {
    return PTR_ERR(trigger);
    }
    list_add_tail_rcu(&trigger.list, &file.triggers);
    trace_event_trigger_enable_disable(file, 1);
    update_cond_flag(file);
    return 0;
    }
pub static mut trace_event_functions: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn disable_eprobe(ep: *mut trace_eprobe, tr: *mut trace_array) -> c_int {
    let mut trigger = core::ptr::null_mut(), *iter;
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut filter: *mut c_void = core::ptr::null_mut();
pub static mut edata: *mut c_void = core::ptr::null_mut();
    file = find_event_file(tr, ep.event_system, ep.event_name);
    if (!file) {
    return -ENOENT;
    }
    list_for_each_entry(iter, &file.triggers, list) {
    if (!(iter.flags & EVENT_TRIGGER_FL_PROBE)) {
    continue;
    }
    edata = iter.private_data;
    if (edata.ep == ep) {
    trigger = iter;
    break;
    }
    }
    if (!trigger) {
    return -ENODEV;
    }
    list_del_rcu(&trigger.list);
    trace_event_trigger_enable_disable(file, 0);
    update_cond_flag(file);
// Make sure nothing is using the edata or trigger
    tracepoint_synchronize_unregister();
    filter = rcu_access_pointer(trigger.filter);
    if (filter) {
    free_event_filter(filter);
    }
    kfree(edata);
    kfree(trigger);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn enable_trace_eprobe(call: *mut trace_event_call, file: *mut trace_event_file) -> c_int {
pub static mut tp: *mut c_void = core::ptr::null_mut();
pub static mut ep: *mut c_void = core::ptr::null_mut();
    let mut enabled = 0;
pub static mut ret: c_int = 0;
pub static mut cnt: c_int = 0;
    tp = trace_probe_primary_from_call(call);
    if (WARN_ON_ONCE!(!tp)) {
    return -ENODEV;
    }
    enabled = trace_probe_is_enabled(tp);
// This also changes "enabled" state
    if (file) {
    ret = trace_probe_add_file(tp, file);
    if (ret) {
    return ret;
    }
    } else {
    trace_probe_set_flag(tp, TP_FLAG_PROFILE);
    }
    if (enabled) {
    return 0;
    }
    for_each_trace_eprobe_tp(ep, tp) {
    ret = enable_eprobe(ep, file);
    if (ret) {
    break;
    }
    enabled = true;
    cnt += 1;
    }
    if (ret) {
// Failed to enable one of them. Roll back all
    if (enabled) {
//
// It's a bug if one failed for something other than memory
// not being available but another eprobe succeeded.
//
    WARN_ON_ONCE!(ret != -ENOMEM);
    for_each_trace_eprobe_tp(ep, tp) {
    disable_eprobe(ep, file.tr);
    if (!--cnt) {
    break;
    }
    }
    }
    if (file) {
    trace_probe_remove_file(tp, file);
    }
    else {
    trace_probe_clear_flag(tp, TP_FLAG_PROFILE);
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn disable_trace_eprobe(call: *mut trace_event_call, file: *mut trace_event_file) -> c_int {
pub static mut tp: *mut c_void = core::ptr::null_mut();
pub static mut ep: *mut c_void = core::ptr::null_mut();
    tp = trace_probe_primary_from_call(call);
    if (WARN_ON_ONCE!(!tp)) {
    return -ENODEV;
    }
    if (file) {
    if (!trace_probe_get_file_link(tp, file)) {
    return -ENOENT;
    }
    if (!trace_probe_has_single_file(tp)) {
// goto;
    }
    trace_probe_clear_flag(tp, TP_FLAG_TRACE);
    } else {
    trace_probe_clear_flag(tp, TP_FLAG_PROFILE);
    }
    if (!trace_probe_is_enabled(tp)) {
    for_each_trace_eprobe_tp(ep, tp) {
    disable_eprobe(ep, file.tr);
    }
    }
// label;
    if (file) {
//
// Synchronization is done in below function. For perf event,
// file == NULL and perf_trace_event_unreg() calls
// tracepoint_synchronize_unregister() to ensure synchronize
// event. We don't need to care about it.
//
    trace_probe_remove_file(tp, file);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn eprobe_register(event: *mut trace_event_call, type: trace_reg, data: *mut c_void) -> c_int {
    let mut file = data;
    match (type) {
    TRACE_REG_REGISTER => {
    return enable_trace_eprobe(event, file);
    }
    TRACE_REG_UNREGISTER => {
    return disable_trace_eprobe(event, file);

    }
    TRACE_REG_PERF_REGISTER => {
    }
    TRACE_REG_PERF_UNREGISTER => {
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
#[no_mangle]
pub unsafe extern "C" fn init_trace_eprobe_call(ep: *mut trace_eprobe) {
    let mut call = trace_probe_event_call(&ep.tp);
    call.flags = TRACE_EVENT_FL_EPROBE;
    call.event.funcs = &eprobe_funcs;
    call.class.fields_array = eprobe_fields_array;
    call.class.reg = eprobe_register;
    }
#[no_mangle]
pub unsafe extern "C" fn find_and_get_event(system: *mut c_char, event_name: *mut c_char) -> *mut c_void {
pub static mut tp_event: *mut c_void = core::ptr::null_mut();
pub static mut name: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(tp_event, &ftrace_events, list) {
// Skip other probes and ftrace events
    if (tp_event.flags &
    (TRACE_EVENT_FL_IGNORE_ENABLE |
    TRACE_EVENT_FL_KPROBE |
    TRACE_EVENT_FL_UPROBE |
    TRACE_EVENT_FL_EPROBE)) {
    continue;
    }
    if (!tp_event.class.system ||
    strcmp(system, tp_event.class.system)) {
    continue;
    }
    name = trace_event_name(tp_event);
    if (!name || strcmp(event_name, name)) {
    continue;
    }
    if (!trace_event_try_get_ref(tp_event)) {
    return core::ptr::null_mut();
    }
    return tp_event;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn trace_eprobe_parse_filter(ep: *mut trace_eprobe, argc: c_int, argv[]: *const c_char) -> c_int {
    let mut dummy = core::ptr::null_mut();
    int i, ret, len = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (argc == 0) {
    trace_probe_log_err(0, NO_EP_FILTER);
    return -EINVAL;
    }
// Recover the filter string
    for (i = 0; i < argc; i++) {
    len += strlen(argv[i]) + 1;
    }
    ep.filter_str = kzalloc(len, GFP_KERNEL);
    if (!ep.filter_str) {
    return -ENOMEM;
    }
    p = ep.filter_str;
    while (i < argc) {
    if (i) {
    ret = snprintf(p, len, " %s", argv[i]);
    }
    else {
    ret = snprintf(p, len, "%s", argv[i]);
    }
    p += ret;
    len -= ret;
    }
//
// Ensure the filter string can be parsed correctly. Note, this
// filter string is for the original event, not for the eprobe.
//
    ret = create_event_filter(top_trace_array(), ep.event, ep.filter_str,
    true, &dummy);
    free_event_filter(dummy);
    if (ret) {
    kfree(ep.filter_str);
    ep.filter_str = core::ptr::null_mut();
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __trace_eprobe_create(argc: c_int, argv[]: *const c_char) -> c_int {
//
// Argument syntax:
// e[:[GRP/][ENAME]] SYSTEM.EVENT [FETCHARGS] [if FILTER]
// Fetch args (no space):
// <name>=$<field>[:TYPE]
//
    struct traceprobe_parse_context *ctx __free(traceprobe_parse_context) = core::ptr::null_mut();
    struct trace_eprobe *ep __free(trace_event_probe_cleanup) = core::ptr::null_mut();
    const char *trlog __free(trace_probe_log_clear) = core::ptr::null_mut();
    let mut event = core::ptr::null_mut(), *group = EPROBE_EVENT_SYSTEM;
    let mut sys_event = core::ptr::null_mut(), *sys_name = core::ptr::null_mut();
pub static mut event_call: *mut c_void = core::ptr::null_mut();
    char *buf1 __free(kfree) = core::ptr::null_mut();
    char *buf2 __free(kfree) = core::ptr::null_mut();
    char *gbuf __free(kfree) = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    let mut i = 0;
    let mut filter_cnt = 0;
    if (argc < 2 || argv[0][0] != 'e') {
    return -ECANCELED;
    }
    trlog = trace_probe_log_init("event_probe", argc, argv);
    event = strchr(&argv[0][1], ':');
    if (event) {
    gbuf = kmalloc(MAX_EVENT_NAME_LEN, GFP_KERNEL);
    if (!gbuf) {
    return -ENOMEM;
    }
    event += 1;
    ret = traceprobe_parse_event_name(&event, &group, gbuf,
    event - argv[0]);
    if (ret) {
    return -EINVAL;
    }
    }
    trace_probe_log_set_index(1);
    sys_event = argv[1];
    buf2 = kmalloc(MAX_EVENT_NAME_LEN, GFP_KERNEL);
    if (!buf2) {
    return -ENOMEM;
    }
    ret = traceprobe_parse_event_name(&sys_event, &sys_name, buf2, 0);
    if (ret || !sys_event || !sys_name) {
    trace_probe_log_err(0, NO_EVENT_INFO);
    return -EINVAL;
    }
    if (!event) {
    buf1 = kstrdup(sys_event, GFP_KERNEL);
    if (!buf1) {
    return -ENOMEM;
    }
    event = buf1;
    }
    while (i < argc) {
    if (!strcmp(argv[i], "if")) {
    filter_idx = i + 1;
    filter_cnt = argc - filter_idx;
    argc = i;
    break;
    }
    }
    if (argc - 2 > MAX_TRACE_ARGS) {
    trace_probe_log_set_index(2);
    trace_probe_log_err(0, TOO_MANY_ARGS);
    return -E2BIG;
    }
    scoped_guard(mutex, &event_mutex) {
    event_call = find_and_get_event(sys_name, sys_event);
    ep = alloc_event_probe(group, event, event_call, argc - 2);
    }
    if (IS_ERR(ep)) {
    ret = PTR_ERR(ep);
    if (ret == -ENODEV) {
    trace_probe_log_err(0, BAD_ATTACH_EVENT);
    }
// This must return -ENOMEM or missing event, else there is a bug
    WARN_ON_ONCE!(ret != -ENOMEM && ret != -ENODEV);
    return ret;
    }
    if (filter_idx) {
    trace_probe_log_set_index(filter_idx);
    ret = trace_eprobe_parse_filter(ep, filter_cnt, argv + filter_idx);
    if (ret) {
    return -EINVAL;
    }
    } else {
    ep.filter_str = core::ptr::null_mut();
    }
    ctx = kzalloc_obj(*ctx);
    if (!ctx) {
    return -ENOMEM;
    }
    ctx.event = ep.event;
    ctx.flags = TPARG_FL_KERNEL | TPARG_FL_TEVENT;
    argc -= 2; argv += 2;
// parse arguments
    while (i < argc) {
    trace_probe_log_set_index(i + 2);
    ret = traceprobe_parse_probe_arg(&ep.tp, i, argv[i], ctx);
// Handle symbols "@"
    if (!ret) {
    ret = traceprobe_update_arg(&ep.tp.args[i]);
    }
    if (ret) {
    return ret;
    }
    }
    ret = traceprobe_set_print_fmt(&ep.tp, PROBE_PRINT_EVENT);
    if (ret < 0) {
    return ret;
    }
    init_trace_eprobe_call(ep);
    scoped_guard(mutex, &event_mutex) {
    ret = trace_probe_register_event_call(&ep.tp);
    if (ret) {
    if (ret == -EEXIST) {
    trace_probe_log_set_index(0);
    trace_probe_log_err(0, EVENT_EXIST);
    }
    return ret;
    }
    ret = dyn_event_add(&ep.devent, &ep.tp.event.call);
    if (ret < 0) {
    trace_probe_unregister_event_call(&ep.tp);
    return ret;
    }
// To avoid freeing registered eprobe event, clear ep.
    ep = core::ptr::null_mut();
    }
    return ret;
    }
//
// Register dynevent at core_initcall. This allows kernel to setup eprobe
// events in postcore_initcall without tracefs.
//
#[no_mangle]
unsafe extern "C" fn trace_events_eprobe_init_early() -> __init int {
pub static mut err: c_int = 0;
    err = dyn_event_register(&eprobe_dyn_event_ops);
    if (err) {
    pr_warn!("Could not register eprobe_dyn_event_ops\n");
    }
    return err;
    }
    core_initcall!(trace_events_eprobe_init_early);
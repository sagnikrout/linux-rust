//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_kprobe.c
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
// Kprobes-based tracing events
//
// Created by Masami Hiramatsu <mhiramat@redhat.com>
//

pub const KRETPROBE_MAXACTIVE_MAX: c_int = 4096;
// Kprobe early definition from command line
    static char kprobe_boot_events_buf[COMMAND_LINE_SIZE] __initdata;
#[no_mangle]
unsafe extern "C" fn set_kprobe_boot_events(str: *mut c_char) -> c_int {
    trace_append_boot_param(kprobe_boot_events_buf, str, ';',
    COMMAND_LINE_SIZE);
    disable_tracing_selftest("running kprobe events");
    return 1;
    }
    __setup!("kprobe_event=", set_kprobe_boot_events);
// forward_decl: trace_kprobe_create;
// forward_decl: trace_kprobe_show;
// forward_decl: trace_kprobe_release;
// forward_decl: trace_kprobe_is_busy;
// forward_decl: trace_kprobe_match;
pub static mut dyn_event_operations: usize = 0;
//
// Kprobe event core functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_kprobe {
    pub devent: dyn_event,
//     pub /: *mut *mut kretprobe rp; / Use rp.kp for kprobe use,
    pub nhit: *mut unsigned long ,
//     pub /: *const *const *const char symbol; / symbol name,
    pub tp: trace_probe,
}

#[no_mangle]
unsafe extern "C" fn is_trace_kprobe(ev: *mut dyn_event) -> bool {
    return ev.ops == &trace_kprobe_ops;
    }
#[no_mangle]
pub unsafe extern "C" fn to_trace_kprobe(ev: *mut dyn_event) -> *mut c_void {
    return container_of!(ev, trace_kprobe, devent);
    }
//
// for_each_trace_kprobe - iterate over the trace_kprobe list
// @pos:	the struct trace_kprobe * for each entry
// @dpos:	the struct dyn_event * to use as a loop cursor
//

    for_each_dyn_event(dpos)		 {
    if (is_trace_kprobe(dpos) && (pos = to_trace_kprobe(dpos)))

#[no_mangle]
unsafe extern "C" fn trace_kprobe_is_return(tk: *mut trace_kprobe) -> nokprobe_inline bool {
    }
    return tk.rp.handler != core::ptr::null_mut();
    }
    static nokprobe_inline const char *trace_kprobe_symbol(trace_kprobe *tk)
    {
    return tk.symbol ? tk.symbol : "unknown";
    }
#[no_mangle]
unsafe extern "C" fn trace_kprobe_offset(tk: *mut trace_kprobe) -> nokprobe_inline unsigned long {
    return tk.rp.kp.offset;
    }
#[no_mangle]
unsafe extern "C" fn trace_kprobe_has_gone(tk: *mut trace_kprobe) -> nokprobe_inline bool {
    return kprobe_gone(&tk.rp.kp);
    }
    static nokprobe_inline bool trace_kprobe_within_module(trace_kprobe *tk, module *mod)
    {
pub static mut len: c_int = 0;
    let mut name = trace_kprobe_symbol(tk);
    return strncmp(module_name!(mod), name, len) == 0 && name[len] == ':';
    }

#[no_mangle]
unsafe extern "C" fn trace_kprobe_module_exist(tk: *mut trace_kprobe) -> nokprobe_inline bool {
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!tk.symbol) {
    return false;
    }
    p = strchr(tk.symbol, ':');
    if (!p) {
    return true;
    }
// p = '\0';
    scoped_guard(rcu)
    ret = !!find_module(tk.symbol);
// p = ':';
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn trace_kprobe_module_exist(tk: *mut trace_kprobe) -> bool {
    return false;
    }

#[no_mangle]
unsafe extern "C" fn trace_kprobe_is_busy(ev: *mut dyn_event) -> bool {
    let mut tk = to_trace_kprobe(ev);
    return trace_probe_is_enabled(&tk.tp);
    }
#[no_mangle]
pub unsafe extern "C" fn trace_kprobe_match_command_head(tk: *mut trace_kprobe, argc: c_int, argv: *mut *mut c_char) -> bool {
    char buf[32];
    let mut len = 0;
    if (!argc) {
    return true;
    }
    if (!tk.symbol) {
    snprintf(buf, sizeof!(buf), "0x%p", tk.rp.kp.addr);
    if (strcmp(buf, argv[0])) {
    return false;
    }
    } else if (tk.rp.kp.offset) {
    len = strlen(trace_kprobe_symbol(tk));
    if (strncmp(trace_kprobe_symbol(tk), argv[0], len) ||
    argv[0][len] != '+') {
    return false;
    }
    snprintf(buf, sizeof!(buf), "%u", tk.rp.kp.offset);
    if (strcmp(buf, &argv[0][len + 1])) {
    return false;
    }
    } else if (strcmp(trace_kprobe_symbol(tk), argv[0])) {
    return false;
    }
    argc -= 1; argv += 1;
    return trace_probe_match_command_args(&tk.tp, argc, argv);
    }
#[no_mangle]
pub unsafe extern "C" fn trace_kprobe_match(system: *mut c_char, event: *mut c_char, argc: c_int, argv: *mut *mut c_char, ev: *mut dyn_event) -> bool {
    let mut tk = to_trace_kprobe(ev);
    return (event[0] == '\0' ||
    strcmp(trace_probe_name(&tk.tp), event) == 0) &&
    (!system || strcmp(trace_probe_group_name(&tk.tp), system) == 0) &&
    trace_kprobe_match_command_head(tk, argc, argv);
    }
#[no_mangle]
unsafe extern "C" fn trace_kprobe_nhit(tk: *mut trace_kprobe) -> nokprobe_inline unsigned long {
pub static mut nhit: c_ulong = 0;
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    nhit += *per_cpu_ptr(tk.nhit, cpu);
    }
    return nhit;
    }
#[no_mangle]
unsafe extern "C" fn trace_kprobe_is_registered(tk: *mut trace_kprobe) -> nokprobe_inline bool {
    return !(list_empty(&tk.rp.kp.list) &&
    hlist_unhashed(&tk.rp.kp.hlist));
    }
// Return 0 if it fails to find the symbol address
    static nokprobe_inline
#[no_mangle]
pub unsafe extern "C" fn trace_kprobe_address(tk: *mut trace_kprobe) -> c_ulong {
    let mut addr = 0;
    if (tk.symbol) {
    addr = (unsigned long)
    kallsyms_lookup_name(trace_kprobe_symbol(tk));
    if (addr) {
    addr += tk.rp.kp.offset;
    }
    } else {
    addr = (unsigned long)tk.rp.kp.addr;
    }
    return addr;
    }
    static nokprobe_inline struct trace_kprobe *
    trace_kprobe_primary_from_call(trace_event_call *call)
    {
pub static mut tp: *mut c_void = core::ptr::null_mut();
    tp = trace_probe_primary_from_call(call);
    if (WARN_ON_ONCE!(!tp)) {
    return core::ptr::null_mut();
    }
    return container_of!(tp, trace_kprobe, tp);
    }
#[no_mangle]
pub unsafe extern "C" fn trace_kprobe_on_func_entry(call: *mut trace_event_call) -> bool {
    let mut tk = trace_kprobe_primary_from_call(call);
    return tk ? (kprobe_on_func_entry(tk.rp.kp.addr,
    tk.rp.kp.addr ? core::ptr::null_mut() : tk.rp.kp.symbol_name,
    tk.rp.kp.addr ? 0 : tk.rp.kp.offset) == 0) : false;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_kprobe_error_injectable(call: *mut trace_event_call) -> bool {
    let mut tk = trace_kprobe_primary_from_call(call);
    return tk ? within_error_injection_list(trace_kprobe_address(tk)) :
    false;
    }
// forward_decl: register_kprobe_event;
// forward_decl: unregister_kprobe_event;
// forward_decl: kprobe_dispatcher;
// forward_decl: kretprobe_dispatcher;
#[no_mangle]
unsafe extern "C" fn free_trace_kprobe(tk: *mut trace_kprobe) {
    if (tk) {
    trace_probe_cleanup(&tk.tp);
    kfree(tk.symbol);
    free_percpu(tk.nhit);
    kfree(tk);
    }
    }
    DEFINE_FREE(free_trace_kprobe, trace_kprobe *,
    if (!IS_ERR_OR_NULL(_T)) free_trace_kprobe(_T))
//
// Allocate new trace_probe and initialize it (including kprobes).
//
#[no_mangle]
pub unsafe extern "C" fn alloc_trace_kprobe(group: *mut c_char, event: *mut c_char, addr: *mut c_void, symbol: *mut c_char, offs: c_ulong, maxactive: c_int, nargs: c_int, is_return: bool) -> *mut c_void {
    struct trace_kprobe *tk __free(free_trace_kprobe) = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    tk = kzalloc_flex(*tk, tp.args, nargs);
    if (!tk)
    return ERR_PTR(ret);
    tk.nhit = alloc_percpu(unsigned long);
    if (!tk.nhit)
    return ERR_PTR(ret);
    if (symbol) {
    tk.symbol = kstrdup(symbol, GFP_KERNEL);
    if (!tk.symbol)
    return ERR_PTR(ret);
    tk.rp.kp.symbol_name = tk.symbol;
    tk.rp.kp.offset = offs;
    } else
    tk.rp.kp.addr = addr;
    if (is_return)
    tk.rp.handler = kretprobe_dispatcher;
    else
    tk.rp.kp.pre_handler = kprobe_dispatcher;
    tk.rp.maxactive = maxactive;
    INIT_HLIST_NODE(&tk.rp.kp.hlist);
    INIT_LIST_HEAD(&tk.rp.kp.list);
    ret = trace_probe_init(&tk.tp, event, group, false, nargs);
    if (ret < 0)
    return ERR_PTR(ret);
    dyn_event_init(&tk.devent, &trace_kprobe_ops);
    return_ptr(tk);
    }
#[no_mangle]
pub unsafe extern "C" fn find_trace_kprobe(event: *mut c_char, group: *mut c_char) -> *mut c_void {
pub static mut pos: *mut c_void = core::ptr::null_mut();
pub static mut tk: *mut c_void = core::ptr::null_mut();
    for_each_trace_kprobe(tk, pos)
    if (strcmp(trace_probe_name(&tk.tp), event) == 0 && {
    strcmp(trace_probe_group_name(&tk.tp), group) == 0)
    return tk;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __enable_trace_kprobe(tk: *mut trace_kprobe) -> c_int {
pub static mut ret: c_int = 0;
    if (trace_kprobe_is_registered(tk) && !trace_kprobe_has_gone(tk)) {
    if (trace_kprobe_is_return(tk)) {
    ret = enable_kretprobe(&tk.rp);
    }
    else {
    ret = enable_kprobe(&tk.rp.kp);
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __disable_trace_kprobe(tp: *mut trace_probe) {
pub static mut tk: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(tk, trace_probe_probe_list(tp), tp.list) {
    if (!trace_kprobe_is_registered(tk)) {
    continue;
    }
    if (trace_kprobe_is_return(tk)) {
    disable_kretprobe(&tk.rp);
    }
    else {
    disable_kprobe(&tk.rp.kp);
    }
    }
    }
//
// Enable trace_probe
// if the file is NULL, enable "perf" handler, or enable "trace" handler.
//
#[no_mangle]
pub unsafe extern "C" fn enable_trace_kprobe(call: *mut trace_event_call, file: *mut trace_event_file) -> c_int {
pub static mut tp: *mut c_void = core::ptr::null_mut();
pub static mut tk: *mut c_void = core::ptr::null_mut();
    let mut enabled = 0;
pub static mut ret: c_int = 0;
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
    list_for_each_entry(tk, trace_probe_probe_list(tp), tp.list) {
    if (trace_kprobe_has_gone(tk)) {
    continue;
    }
    ret = __enable_trace_kprobe(tk);
    if (ret) {
    break;
    }
    enabled = true;
    }
    if (ret) {
// Failed to enable one of them. Roll back all
    if (enabled) {
    __disable_trace_kprobe(tp);
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
//
// Disable trace_probe
// if the file is NULL, disable "perf" handler, or disable "trace" handler.
//
#[no_mangle]
pub unsafe extern "C" fn disable_trace_kprobe(call: *mut trace_event_call, file: *mut trace_event_file) -> c_int {
pub static mut tp: *mut c_void = core::ptr::null_mut();
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
    __disable_trace_kprobe(tp);
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

    !defined(CONFIG_KPROBE_EVENTS_ON_NOTRACE)
#[no_mangle]
unsafe extern "C" fn __within_notrace_func(addr: c_ulong) -> bool {
    unsigned long offset, size;
    if (!addr || !kallsyms_lookup_size_offset(addr, &size, &offset)) {
    return false;
    }
// Get the entry address of the target function
    addr -= offset;
//
// Since ftrace_location_range() does inclusive range check, we need
// to subtract 1 byte from the end address.
//
    return !ftrace_location_range(addr, addr + size - 1);
    }
#[no_mangle]
unsafe extern "C" fn within_notrace_func(tk: *mut trace_kprobe) -> bool {
pub static mut addr: c_ulong = 0;
    char symname[KSYM_NAME_LEN], *p;
    if (!__within_notrace_func(addr)) {
    return false;
    }
// Check if the address is on a suffixed-symbol
    if (!lookup_symbol_name(addr, symname)) {
    p = strchr(symname, '.');
    if (!p) {
    return true;
    }
// p = '\0';
    addr = (unsigned long)kprobe_lookup_name(symname, 0);
    if (addr) {
    return __within_notrace_func(addr);
    }
    }
    return true;
    }

// Internal register function - just handle k*probes and flags
#[no_mangle]
unsafe extern "C" fn __register_trace_kprobe(tk: *mut trace_kprobe) -> c_int {
    let mut i = 0;
    let mut ret = 0;
    ret = security_locked_down(LOCKDOWN_KPROBES);
    if (ret) {
    return ret;
    }
    if (trace_kprobe_is_registered(tk)) {
    return -EINVAL;
    }
    if (within_notrace_func(tk)) {
    pr_warn!("Could not probe notrace function %ps\n",
    trace_kprobe_address(tk));
    return -EINVAL;
    }
    while (i < tk.tp.nr_args) {
    ret = traceprobe_update_arg(&tk.tp.args[i]);
    if (ret) {
    return ret;
    }
    }
// Set/clear disabled flag according to tp->flag
    if (trace_probe_is_enabled(&tk.tp)) {
    tk.rp.kp.flags &= ~KPROBE_FLAG_DISABLED;
    }
    else {
    tk.rp.kp.flags |= KPROBE_FLAG_DISABLED;
    }
    if (trace_kprobe_is_return(tk)) {
    ret = register_kretprobe(&tk.rp);
    }
    else {
    ret = register_kprobe(&tk.rp.kp);
    }
    return ret;
    }
// Internal unregister function - just handle k*probes and flags
#[no_mangle]
unsafe extern "C" fn __unregister_trace_kprobe(tk: *mut trace_kprobe) {
    if (trace_kprobe_is_registered(tk)) {
    if (trace_kprobe_is_return(tk)) {
    unregister_kretprobe(&tk.rp);
    }
    else {
    unregister_kprobe(&tk.rp.kp);
    }
// Cleanup kprobe for reuse and mark it unregistered
    INIT_HLIST_NODE(&tk.rp.kp.hlist);
    INIT_LIST_HEAD(&tk.rp.kp.list);
    if (tk.rp.kp.symbol_name) {
    tk.rp.kp.addr = core::ptr::null_mut();
    }
    }
    }
// Unregister a trace_probe and probe_event
#[no_mangle]
unsafe extern "C" fn unregister_trace_kprobe(tk: *mut trace_kprobe) -> c_int {
// If other probes are on the event, just unregister kprobe
    if (trace_probe_has_sibling(&tk.tp)) {
// goto;
    }
// Enabled event can not be unregistered
    if (trace_probe_is_enabled(&tk.tp)) {
    return -EBUSY;
    }
// If there's a reference to the dynamic event
    if (trace_event_dyn_busy(trace_probe_event_call(&tk.tp))) {
    return -EBUSY;
    }
// Will fail if probe is being used by ftrace or perf
    if (unregister_kprobe_event(tk)) {
    return -EBUSY;
    }
// label;
    __unregister_trace_kprobe(tk);
    dyn_event_remove(&tk.devent);
    trace_probe_unlink(&tk.tp);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_kprobe_has_same_kprobe(orig: *mut trace_kprobe, comp: *mut trace_kprobe) -> bool {
    let mut tpe = orig.tp.event;
    let mut i = 0;
    list_for_each_entry(orig, &tpe.probes, tp.list) {
    if (strcmp(trace_kprobe_symbol(orig),
    trace_kprobe_symbol(comp)) ||
    trace_kprobe_offset(orig) != trace_kprobe_offset(comp)) {
    continue;
    }
//
// trace_probe_compare_arg_type() ensured that nr_args and
// each argument name and type are same. Let's compare comm.
//
    while (i < orig.tp.nr_args) {
    if (strcmp(orig.tp.args[i].comm,
    comp.tp.args[i].comm)) {
    break;
    }
    }
    if (i == orig.tp.nr_args) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn append_trace_kprobe(tk: *mut trace_kprobe, to: *mut trace_kprobe) -> c_int {
    let mut ret = 0;
    ret = trace_probe_compare_arg_type(&tk.tp, &to.tp);
    if (ret) {
// Note that argument starts index = 2
    trace_probe_log_set_index(ret + 1);
    trace_probe_log_err(0, DIFF_ARG_TYPE);
    return -EEXIST;
    }
    if (trace_kprobe_has_same_kprobe(to, tk)) {
    trace_probe_log_set_index(0);
    trace_probe_log_err(0, SAME_PROBE);
    return -EEXIST;
    }
// Append to existing event
    ret = trace_probe_append(&tk.tp, &to.tp);
    if (ret) {
    return ret;
    }
// Register k*probe
    ret = __register_trace_kprobe(tk);
    if (ret == -ENOENT && !trace_kprobe_module_exist(tk)) {
    pr_warn!("This probe might be able to register after target module is loaded. Continue.\n");
    ret = 0;
    }
    if (ret) {
    trace_probe_unlink(&tk.tp);
    }
    else {
    dyn_event_add(&tk.devent, trace_probe_event_call(&tk.tp));
    }
    return ret;
    }
// Register a trace_probe and probe_event
#[no_mangle]
unsafe extern "C" fn register_trace_kprobe(tk: *mut trace_kprobe) -> c_int {
pub static mut old_tk: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    guard(mutex)(&event_mutex);
    old_tk = find_trace_kprobe(trace_probe_name(&tk.tp),
    trace_probe_group_name(&tk.tp));
    if (old_tk) {
    if (trace_kprobe_is_return(tk) != trace_kprobe_is_return(old_tk)) {
    trace_probe_log_set_index(0);
    trace_probe_log_err(0, DIFF_PROBE_TYPE);
    return -EEXIST;
    }
    return append_trace_kprobe(tk, old_tk);
    }
// Register new event
    ret = register_kprobe_event(tk);
    if (ret) {
    if (ret == -EEXIST) {
    trace_probe_log_set_index(0);
    trace_probe_log_err(0, EVENT_EXIST);
    } else {
    pr_warn!("Failed to register probe event(%d)\n", ret);
    }
    return ret;
    }
// Register k*probe
    ret = __register_trace_kprobe(tk);
    if (ret == -ENOENT && !trace_kprobe_module_exist(tk)) {
    pr_warn!("This probe might be able to register after target module is loaded. Continue.\n");
    ret = 0;
    }
    if (ret < 0) {
    unregister_kprobe_event(tk);
    }
    else {
    dyn_event_add(&tk.devent, trace_probe_event_call(&tk.tp));
    }
    return ret;
    }

// forward_decl: validate_module_probe_symbol;
#[no_mangle]
unsafe extern "C" fn register_module_trace_kprobe(mod: *mut module, tk: *mut trace_kprobe) -> c_int {
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    p = strchr(trace_kprobe_symbol(tk), ':');
    if (p) {
    ret = validate_module_probe_symbol(module_name!(mod), p + 1);
    }
    if (!ret) {
    ret = __register_trace_kprobe(tk);
    }
    return ret;
    }
// Module notifier call back, checking event on the module
#[no_mangle]
pub unsafe extern "C" fn trace_kprobe_module_callback(nb: *mut notifier_block, val: c_ulong, data: *mut c_void) -> c_int {
    let mut mod = data;
pub static mut pos: *mut c_void = core::ptr::null_mut();
pub static mut tk: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (val != MODULE_STATE_COMING) {
    return NOTIFY_DONE;
    }
// Update probes on coming module
    guard(mutex)(&event_mutex);
    for_each_trace_kprobe(tk, pos) {
    if (trace_kprobe_within_module(tk, mod)) {
// Don't need to check busy - this should have gone.
    __unregister_trace_kprobe(tk);
    ret = register_module_trace_kprobe(mod, tk);
    if (ret) {
    pr_warn!("Failed to re-register probe %s on %s: %d\n",
    trace_probe_name(&tk.tp),
    module_name!(mod), ret);
    }
    }
    }
    return NOTIFY_DONE;
    }
pub static mut notifier_block: usize = 0;
#[no_mangle]
unsafe extern "C" fn trace_kprobe_register_module_notifier() -> c_int {
    return register_module_notifier(&trace_kprobe_module_nb);
    }

#[no_mangle]
unsafe extern "C" fn trace_kprobe_register_module_notifier() -> c_int {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn count_symbols(data: *mut c_void, unused: c_ulong) -> c_int {
    let mut count = data;
    (*count)++;
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_count_ctx {
    pub count: c_uint,
    pub name: *const c_char,
}

#[no_mangle]
unsafe extern "C" fn count_mod_symbols(data: *mut c_void, name: *const c_char, unused: c_ulong) -> c_int {
    let mut ctx = data;
    if (strcmp(name, ctx.name) == 0) {
    ctx.count += 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn number_of_same_symbols(mod: *const c_char, func_name: *const c_char) -> c_uint {
pub static mut ctx: sym_count_ctx = 0;
    if (!mod) {
    kallsyms_on_each_match_symbol(count_symbols, func_name, &ctx.count);
    }
//
// If the symbol is found in vmlinux, use vmlinux resolution only.
// This prevents module symbols from shadowing vmlinux symbols
// and causing -EADDRNOTAVAIL for unqualified kprobe targets.
//
    if (!mod && ctx.count > 0) {
    return ctx.count;
    }
    module_kallsyms_on_each_symbol!(mod, count_mod_symbols, &ctx);
    return ctx.count;
    }
#[no_mangle]
unsafe extern "C" fn validate_module_probe_symbol(modname: *const c_char, symbol: *const c_char) -> c_int {
pub static mut count: c_uint = 0;
    if (count > 1) {
//
// Users should use ADDR to remove the ambiguity of
// using KSYM only.
//
    return -EADDRNOTAVAIL;
    } else if (count == 0) {
//
// We can return ENOENT earlier than when register the
// kprobe.
//
    return -ENOENT;
    }
    return 0;
    }

// Return NULL if the module is not loaded or under unloading.
#[no_mangle]
pub unsafe extern "C" fn try_module_get_by_name(name: *mut c_char) -> *mut c_void {
pub static mut mod: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    mod = find_module(name);
    if (mod && !try_module_get(mod)) {
    mod = core::ptr::null_mut();
    }
    return mod;
    }

#[no_mangle]
unsafe extern "C" fn validate_probe_symbol(symbol: *mut c_char) -> c_int {
    let mut mod = core::ptr::null_mut();
    let mut modname = core::ptr::null_mut(), *p;
pub static mut ret: c_int = 0;
    p = strchr(symbol, ':');
    if (p) {
    modname = symbol;
    symbol = p + 1;
// p = '\0';
    mod = try_module_get_by_name(modname);
    if (!mod) {
// goto;
    }
    }
    ret = validate_module_probe_symbol(modname, symbol);
// label;
    if (p) {
// p = ':';
    }
    if (mod) {
    module_put!(mod);
    }
    return ret;
    }
// forward_decl: trace_kprobe_entry_handler;
#[no_mangle]
pub unsafe extern "C" fn trace_kprobe_create_internal(argc: c_int, ctx: *mut traceprobe_parse_context) -> c_int {
//
// Argument syntax:
// - Add kprobe:
// p[:[GRP/][EVENT]] [MOD:]KSYM[+OFFS]|KADDR [FETCHARGS]
// - Add kretprobe:
// r[MAXACTIVE][:[GRP/][EVENT]] [MOD:]KSYM[+0] [FETCHARGS]
// Or
// p[:[GRP/][EVENT]] [MOD:]KSYM[+0]%return [FETCHARGS]
//
// Fetch args:
// $retval	: fetch return value
// $stack	: fetch stack address
// $stackN	: fetch Nth of stack (N:0-)
// $comm       : fetch current task comm
// @ADDR	: fetch memory at ADDR (ADDR should be in kernel)
// @SYM[+|-offs] : fetch memory at SYM +|- offs (SYM is a data symbol)
// %REG	: fetch register REG
// Dereferencing memory fetch:
// +|-offs(ARG) : fetch memory at ARG +|- offs address.
// Alias name of args:
// NAME=FETCHARG : set NAME as alias of FETCHARG.
// Type of args:
// FETCHARG:TYPE : use TYPE instead of unsigned long.
//
    struct trace_kprobe *tk __free(free_trace_kprobe) = core::ptr::null_mut();
    let mut event = core::ptr::null_mut(), *group = KPROBE_EVENT_SYSTEM;
    const char **new_argv __free(kfree) = core::ptr::null_mut();
    int i, len, new_argc = 0, ret = 0;
    char *symbol __free(kfree) = core::ptr::null_mut();
    char *ebuf __free(kfree) = core::ptr::null_mut();
    char *gbuf __free(kfree) = core::ptr::null_mut();
    char *abuf __free(kfree) = core::ptr::null_mut();
    char *dbuf __free(kfree) = core::ptr::null_mut();
    enum probe_print_type ptype;
pub static mut is_return: bool = false;
pub static mut maxactive: c_int = 0;
    let mut addr = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
pub static mut offset: c_long = 0;
    match (argv[0][0]) {
    'r' => {
    is_return = true;
    // break;
    }
    'p' => {
    // break;
    }
    _ => {
    return -ECANCELED;
    }
    }
    if (argc < 2) {
    return -ECANCELED;
    }
    event = strchr(&argv[0][1], ':');
    if (event) {
    event += 1;
    }
    if (isdigit(argv[0][1])) {
    char *buf __free(kfree) = core::ptr::null_mut();
    if (!is_return) {
    trace_probe_log_err(1, BAD_MAXACT_TYPE);
    return -EINVAL;
    }
    if (event) {
    len = event - &argv[0][1] - 1;
    }
    else {
    len = strlen(&argv[0][1]);
    }
    if (len > MAX_EVENT_NAME_LEN - 1) {
    trace_probe_log_err(1, BAD_MAXACT);
    return -EINVAL;
    }
    buf = kmemdup(&argv[0][1], len + 1, GFP_KERNEL);
    if (!buf) {
    return -ENOMEM;
    }
    buf[len] = '\0';
    ret = kstrtouint(buf, 0, &maxactive);
    if (ret || !maxactive) {
    trace_probe_log_err(1, BAD_MAXACT);
    return -EINVAL;
    }
// kretprobes instances are iterated over via a list. The
// maximum should stay reasonable.
//
    if (maxactive > KRETPROBE_MAXACTIVE_MAX) {
    trace_probe_log_err(1, MAXACT_TOO_BIG);
    return -EINVAL;
    }
    }
// try to parse an address. if that fails, try to read the
// input as a symbol.
    if (kstrtoul(argv[1], 0, &addr)) {
    trace_probe_log_set_index(1);
// Check whether uprobe event specified
    if (strchr(argv[1], '/') && strchr(argv[1], ':')) {
    return -ECANCELED;
    }
// a symbol specified
    symbol = kstrdup(argv[1], GFP_KERNEL);
    if (!symbol) {
    return -ENOMEM;
    }
    tmp = strchr(symbol, '%');
    if (tmp) {
    if (!strcmp(tmp, "%return")) {
// tmp = '\0';
    is_return = true;
    } else {
    trace_probe_log_err(tmp - symbol, BAD_ADDR_SUFFIX);
    return -EINVAL;
    }
    }
// TODO: support .init module functions
    ret = traceprobe_split_symbol_offset(symbol, &offset);
    if (ret || offset < 0 || offset > UINT_MAX) {
    trace_probe_log_err(0, BAD_PROBE_ADDR);
    return -EINVAL;
    }
    ret = validate_probe_symbol(symbol);
    if (ret) {
    if (ret == -EADDRNOTAVAIL) {
    trace_probe_log_err(0, NON_UNIQ_SYMBOL);
    }
    else {
    trace_probe_log_err(0, BAD_PROBE_ADDR);
    }
    return -EINVAL;
    }
    if (is_return) {
    ctx.flags |= TPARG_FL_RETURN;
    }
    ret = kprobe_on_func_entry(core::ptr::null_mut(), symbol, offset);
    if (ret == 0 && !is_return) {
    ctx.flags |= TPARG_FL_FENTRY;
    }
// Defer the ENOENT case until register kprobe
    if (ret == -EINVAL && is_return) {
    trace_probe_log_err(0, BAD_RETPROBE);
    return -EINVAL;
    }
    }
    trace_probe_log_set_index(0);
    if (event) {
    gbuf = kmalloc(MAX_EVENT_NAME_LEN, GFP_KERNEL);
    if (!gbuf) {
    return -ENOMEM;
    }
    ret = traceprobe_parse_event_name(&event, &group, gbuf,
    event - argv[0]);
    if (ret) {
    return ret;
    }
    }
    if (!event) {
// Make a new event name
    ebuf = kmalloc(MAX_EVENT_NAME_LEN, GFP_KERNEL);
    if (!ebuf) {
    return -ENOMEM;
    }
    if (symbol) {
    snprintf(ebuf, MAX_EVENT_NAME_LEN, "%c_%s_%ld",
    is_return ? 'r' : 'p', symbol, offset);
    }
    else {
    snprintf(ebuf, MAX_EVENT_NAME_LEN, "%c_0x%p",
    is_return ? 'r' : 'p', addr);
    }
    sanitize_event_name(ebuf);
    event = ebuf;
    }
    abuf = kmalloc(MAX_BTF_ARGS_LEN, GFP_KERNEL);
    if (!abuf) {
    return -ENOMEM;
    }
    argc -= 2; argv += 2;
    ctx.funcname = symbol;
    new_argv = traceprobe_expand_meta_args(argc, argv, &new_argc,
    abuf, MAX_BTF_ARGS_LEN, ctx);
    if (IS_ERR(new_argv)) {
    ret = PTR_ERR(new_argv);
    new_argv = core::ptr::null_mut();
    return ret;
    }
    if (new_argv) {
    argc = new_argc;
    argv = new_argv;
    }
    if (argc > MAX_TRACE_ARGS) {
    trace_probe_log_set_index(2);
    trace_probe_log_err(0, TOO_MANY_ARGS);
    return -E2BIG;
    }
    ret = traceprobe_expand_dentry_args(argc, argv, &dbuf);
    if (ret) {
    return ret;
    }
// setup a probe
    tk = alloc_trace_kprobe(group, event, addr, symbol, offset, maxactive,
    argc, is_return);
    if (IS_ERR(tk)) {
    ret = PTR_ERR(tk);
// This must return -ENOMEM, else there is a bug
    WARN_ON_ONCE!(ret != -ENOMEM);
    return ret;	/* We know tk is not allocated */
    }
// parse arguments
    while (i < argc) {
    trace_probe_log_set_index(i + 2);
    ctx.offset = 0;
    ret = traceprobe_parse_probe_arg(&tk.tp, i, argv[i], ctx);
    if (ret) {
    return ret;	/* This can be -ENOMEM */
    }
    }
// entry handler for kretprobe
    if (is_return && tk.tp.entry_arg) {
    tk.rp.entry_handler = trace_kprobe_entry_handler;
    tk.rp.data_size = traceprobe_get_entry_data_size(&tk.tp);
    }
    ptype = is_return ? PROBE_PRINT_RETURN : PROBE_PRINT_NORMAL;
    ret = traceprobe_set_print_fmt(&tk.tp, ptype);
    if (ret < 0) {
    return ret;
    }
    ret = register_trace_kprobe(tk);
    if (ret) {
    trace_probe_log_set_index(1);
    if (ret == -EILSEQ) {
    trace_probe_log_err(0, BAD_INSN_BNDRY);
    }

    else if (ret == -ENOENT) {
    trace_probe_log_err(0, BAD_PROBE_ADDR);
    }

    else if (ret != -ENOMEM && ret != -EEXIST) {
    trace_probe_log_err(0, FAIL_REG_PROBE);
    }
    return ret;
    }
//
// Here, 'tk' has been registered to the list successfully,
// so we don't need to free it.
//
    tk = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_kprobe_create_cb(argc: c_int, argv[]: *const c_char) -> c_int {
    struct traceprobe_parse_context *ctx __free(traceprobe_parse_context) = core::ptr::null_mut();
    let mut ret = 0;
    ctx = kzalloc_obj(*ctx);
    if (!ctx) {
    return -ENOMEM;
    }
    ctx.flags = TPARG_FL_KERNEL;
    trace_probe_log_init("trace_kprobe", argc, argv);
    ret = trace_kprobe_create_internal(argc, argv, ctx);
    trace_probe_log_clear();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn trace_kprobe_create(raw_command: *const c_char) -> c_int {
    return trace_probe_create(raw_command, trace_kprobe_create_cb);
    }
#[no_mangle]
unsafe extern "C" fn create_or_delete_trace_kprobe(raw_command: *const c_char) -> c_int {
    let mut ret = 0;
    if (raw_command[0] == '-') {
    return dyn_event_release(raw_command, &trace_kprobe_ops);
    }
    ret = dyn_event_create(raw_command, &trace_kprobe_ops);
pub static mut ret: return = 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_kprobe_run_command(cmd: *mut dynevent_cmd) -> c_int {
    return create_or_delete_trace_kprobe(cmd.seq.buffer);
    }
//
// kprobe_event_cmd_init - Initialize a kprobe event command object
// @cmd: A pointer to the dynevent_cmd struct representing the new event
// @buf: A pointer to the buffer used to build the command
// @maxlen: The length of the buffer passed in @buf
//
// Initialize a synthetic event command object.  Use this before
// calling any of the other kprobe_event functions.
//
#[no_mangle]
pub unsafe extern "C" fn kprobe_event_cmd_init(cmd: *mut dynevent_cmd, buf: *mut c_char, maxlen: c_int) {
    dynevent_cmd_init(cmd, buf, maxlen, DYNEVENT_TYPE_KPROBE,
    trace_kprobe_run_command);
    }
    EXPORT_SYMBOL_GPL(kprobe_event_cmd_init);
//
// __kprobe_event_gen_cmd_start - Generate a kprobe event command from arg list
// @cmd: A pointer to the dynevent_cmd struct representing the new event
// @kretprobe: Is this a return probe?
// @name: The name of the kprobe event
// @loc: The location of the kprobe event
// @...: Variable number of arg (pairs), one pair for each field
//
// NOTE: Users normally won't want to call this function directly, but
// rather use the kprobe_event_gen_cmd_start() wrapper, which automatically
// adds a NULL to the end of the arg list.  If this function is used
// directly, make sure the last arg in the variable arg list is NULL.
//
// Generate a kprobe event command to be executed by
// kprobe_event_gen_cmd_end().  This function can be used to generate the
// complete command or only the first part of it; in the latter case,
// kprobe_event_add_fields() can be used to add more fields following this.
//
// Unlikely the synth_event_gen_cmd_start(), @loc must be specified. This
// returns -EINVAL if @loc == NULL.
//
// Return: 0 if successful, error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn __kprobe_event_gen_cmd_start(cmd: *mut dynevent_cmd, kretprobe: bool, name: *mut c_char, loc: *mut c_char) -> c_int {
    char buf[MAX_EVENT_NAME_LEN];
pub static mut arg: usize = 0;
    let mut args;
    let mut ret = 0;
    if (cmd.type != DYNEVENT_TYPE_KPROBE) {
    return -EINVAL;
    }
    if (!loc) {
    return -EINVAL;
    }
    if (kretprobe) {
    snprintf(buf, MAX_EVENT_NAME_LEN, "r:kprobes/%s", name);
    }
    else {
    snprintf(buf, MAX_EVENT_NAME_LEN, "p:kprobes/%s", name);
    }
    ret = dynevent_str_add(cmd, buf);
    if (ret) {
    return ret;
    }
    dynevent_arg_init(&arg, 0);
    arg.str = loc;
    ret = dynevent_arg_add(cmd, &arg, core::ptr::null_mut());
    if (ret) {
    return ret;
    }
    va_start(args, loc);
    for (;;) {
pub static mut field: *mut c_void = core::ptr::null_mut();
    field = va_arg(args, const char *);
    if (!field) {
    break;
    }
    if (++cmd.n_fields > MAX_TRACE_ARGS) {
    ret = -EINVAL;
    break;
    }
    arg.str = field;
    ret = dynevent_arg_add(cmd, &arg, core::ptr::null_mut());
    if (ret) {
    break;
    }
    }
    va_end(args);
    return ret;
    }
    EXPORT_SYMBOL_GPL(__kprobe_event_gen_cmd_start);
//
// __kprobe_event_add_fields - Add probe fields to a kprobe command from arg list
// @cmd: A pointer to the dynevent_cmd struct representing the new event
// @...: Variable number of arg (pairs), one pair for each field
//
// NOTE: Users normally won't want to call this function directly, but
// rather use the kprobe_event_add_fields() wrapper, which
// automatically adds a NULL to the end of the arg list.  If this
// function is used directly, make sure the last arg in the variable
// arg list is NULL.
//
// Add probe fields to an existing kprobe command using a variable
// list of args.  Fields are added in the same order they're listed.
//
// Return: 0 if successful, error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn __kprobe_event_add_fields(cmd: *mut dynevent_cmd, ...) -> c_int {
pub static mut arg: usize = 0;
    let mut args;
pub static mut ret: c_int = 0;
    if (cmd.type != DYNEVENT_TYPE_KPROBE) {
    return -EINVAL;
    }
    dynevent_arg_init(&arg, 0);
    va_start(args, cmd);
    for (;;) {
pub static mut field: *mut c_void = core::ptr::null_mut();
    field = va_arg(args, const char *);
    if (!field) {
    break;
    }
    if (++cmd.n_fields > MAX_TRACE_ARGS) {
    ret = -EINVAL;
    break;
    }
    arg.str = field;
    ret = dynevent_arg_add(cmd, &arg, core::ptr::null_mut());
    if (ret) {
    break;
    }
    }
    va_end(args);
    return ret;
    }
    EXPORT_SYMBOL_GPL(__kprobe_event_add_fields);
//
// kprobe_event_delete - Delete a kprobe event
// @name: The name of the kprobe event to delete
//
// Delete a kprobe event with the give @name from kernel code rather
// than directly from the command line.
//
// Return: 0 if successful, error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn kprobe_event_delete(name: *const c_char) -> c_int {
    char buf[MAX_EVENT_NAME_LEN];
    snprintf(buf, MAX_EVENT_NAME_LEN, "-:%s", name);
    return create_or_delete_trace_kprobe(buf);
    }
    EXPORT_SYMBOL_GPL(kprobe_event_delete);
#[no_mangle]
unsafe extern "C" fn trace_kprobe_release(ev: *mut dyn_event) -> c_int {
    let mut tk = to_trace_kprobe(ev);
pub static mut ret: c_int = 0;
    if (!ret) {
    free_trace_kprobe(tk);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn trace_kprobe_show(m: *mut seq_file, ev: *mut dyn_event) -> c_int {
    let mut tk = to_trace_kprobe(ev);
    let mut i = 0;
    seq_putc(m, trace_kprobe_is_return(tk) ? 'r' : 'p');
    if (trace_kprobe_is_return(tk) && tk.rp.maxactive) {
    seq_printf(m, "%d", tk.rp.maxactive);
    }
    seq_printf(m, ":%s/%s", trace_probe_group_name(&tk.tp),
    trace_probe_name(&tk.tp));
    if (!tk.symbol) {
    seq_printf(m, " 0x%p", tk.rp.kp.addr);
    }

    else if (tk.rp.kp.offset) {
    seq_printf(m, " %s+%u", trace_kprobe_symbol(tk),
    tk.rp.kp.offset);
    }
    else {
    seq_printf(m, " %s", trace_kprobe_symbol(tk));
    }
    for (i = 0; i < tk.tp.nr_args; i++) {
    seq_printf(m, " %s=%s", tk.tp.args[i].name, tk.tp.args[i].comm);
    }
    seq_putc(m, '\n');
    trace_probe_dump_args(m, &tk.tp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn probes_seq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut ev = v;
    if (!is_trace_kprobe(ev)) {
    return 0;
    }
    return trace_kprobe_show(m, ev);
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn probes_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut ret = 0;
    ret = security_locked_down(LOCKDOWN_TRACEFS);
    if (ret) {
    return ret;
    }
    if ((file.f_mode & FMODE_WRITE) && (file.f_flags & O_TRUNC)) {
    ret = dyn_events_release_all(&trace_kprobe_ops);
    if (ret < 0) {
    return ret;
    }
    }
    return seq_open(file, &probes_seq_op);
    }
#[no_mangle]
pub unsafe extern "C" fn probes_write(file: *mut file, buffer: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    return trace_parse_run_command(file, buffer, count, ppos,
    create_or_delete_trace_kprobe);
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn trace_kprobe_missed(tk: *mut trace_kprobe) -> c_ulong {
    return trace_kprobe_is_return(tk) ?
    tk.rp.kp.nmissed + tk.rp.nmissed : tk.rp.kp.nmissed;
    }
// Probes profiling interfaces
#[no_mangle]
unsafe extern "C" fn probes_profile_seq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut ev = v;
pub static mut tk: *mut c_void = core::ptr::null_mut();
    let mut nmissed = 0;
    if (!is_trace_kprobe(ev)) {
    return 0;
    }
    tk = to_trace_kprobe(ev);
    nmissed = trace_kprobe_missed(tk);
    seq_printf(m, "  %-44s %15lu %15lu\n",
    trace_probe_name(&tk.tp),
    trace_kprobe_nhit(tk),
    nmissed);
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn profile_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut ret = 0;
    ret = security_locked_down(LOCKDOWN_TRACEFS);
    if (ret) {
    return ret;
    }
    return seq_open(file, &profile_seq_op);
    }
pub static mut file_operations: usize = 0;
// Note that we don't verify it, since the code does not come from user space
#[no_mangle]
pub unsafe extern "C" fn process_fetch_insn(code: *mut fetch_insn, rec: *mut c_void, edata: *mut c_void, dest: *mut c_void, base: *mut c_void) -> c_int {
    let mut regs = rec;
    let mut val = 0;
    let mut ret = 0;
// label;
// 1st stage: get value from context
    match (code.op) {
    FETCH_OP_REG => {
    val = regs_get_register(regs, code.param);
    // break;
    }
    FETCH_OP_STACK => {
    val = regs_get_kernel_stack_nth(regs, code.param);
    // break;
    }
    FETCH_OP_STACKP => {
    val = kernel_stack_pointer(regs);
    // break;
    }
    FETCH_OP_RETVAL => {
    val = regs_return_value(regs);
    // break;

    }
    FETCH_OP_ARG => {
    val = regs_get_kernel_argument(regs, code.param);
    // break;
    }
    FETCH_OP_EDATA => {
    val = *((unsigned long)edata + code.offset);
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
// Kprobe handler
    static nokprobe_inline void
    __kprobe_trace_func(trace_kprobe *tk, pt_regs *regs, trace_event_file *trace_file)
    {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut call = trace_probe_event_call(&tk.tp);
pub static mut fbuffer: usize = 0;
    let mut dsize = 0;
    WARN_ON!(call != trace_file.event_call);
    if (trace_trigger_soft_disabled(trace_file)) {
    return;
    }
    dsize = __get_data_size(&tk.tp, regs, core::ptr::null_mut());
    entry = trace_event_buffer_reserve(&fbuffer, trace_file,
    sizeof!(*entry) + tk.tp.size + dsize);
    if (!entry) {
    return;
    }
    fbuffer.regs = regs;
    entry.ip = (unsigned long)tk.rp.kp.addr;
    store_trace_args(&entry[1], &tk.tp, regs, core::ptr::null_mut(), sizeof!(*entry), dsize);
    trace_event_buffer_commit(&fbuffer);
    }
#[no_mangle]
pub unsafe extern "C" fn kprobe_trace_func(tk: *mut trace_kprobe, regs: *mut pt_regs) {
pub static mut link: *mut c_void = core::ptr::null_mut();
    trace_probe_for_each_link_rcu(link, &tk.tp)
    __kprobe_trace_func(tk, regs, link.file);
    }
    NOKPROBE_SYMBOL(kprobe_trace_func);
// Kretprobe handler
#[no_mangle]
pub unsafe extern "C" fn trace_kprobe_entry_handler(ri: *mut kretprobe_instance, regs: *mut pt_regs) -> c_int {
    let mut rp = get_kretprobe(ri);
pub static mut tk: *mut c_void = core::ptr::null_mut();
//
// There is a small chance that get_kretprobe(ri) returns NULL when
// the kretprobe is unregister on another CPU between kretprobe's
// trampoline_handler and this function.
//
    if (unlikely(!rp)) {
    return -ENOENT;
    }
    tk = container_of!(rp, trace_kprobe, rp);
// store argument values into ri->data as entry data
    if (tk.tp.entry_arg) {
    store_trace_entry_data(ri.data, &tk.tp, regs);
    }
    return 0;
    }
    static nokprobe_inline void
    __kretprobe_trace_func(trace_kprobe *tk, kretprobe_instance *ri, pt_regs *regs, trace_event_file *trace_file)
    {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut fbuffer: usize = 0;
    let mut call = trace_probe_event_call(&tk.tp);
    let mut dsize = 0;
    WARN_ON!(call != trace_file.event_call);
    if (trace_trigger_soft_disabled(trace_file)) {
    return;
    }
    dsize = __get_data_size(&tk.tp, regs, ri.data);
    entry = trace_event_buffer_reserve(&fbuffer, trace_file,
    sizeof!(*entry) + tk.tp.size + dsize);
    if (!entry) {
    return;
    }
    fbuffer.regs = regs;
    entry.func = (unsigned long)tk.rp.kp.addr;
    entry.ret_ip = get_kretprobe_retaddr(ri);
    store_trace_args(&entry[1], &tk.tp, regs, ri.data, sizeof!(*entry), dsize);
    trace_event_buffer_commit(&fbuffer);
    }
#[no_mangle]
pub unsafe extern "C" fn kretprobe_trace_func(tk: *mut trace_kprobe, ri: *mut kretprobe_instance, regs: *mut pt_regs) {
pub static mut link: *mut c_void = core::ptr::null_mut();
    trace_probe_for_each_link_rcu(link, &tk.tp)
    __kretprobe_trace_func(tk, ri, regs, link.file);
    }
    NOKPROBE_SYMBOL(kretprobe_trace_func);
// Event entry printers
    static enum print_line_t
    print_kprobe_event(trace_iterator *iter, int flags, trace_event *event)
    {
pub static mut field: *mut c_void = core::ptr::null_mut();
    let mut s = &iter.seq;
pub static mut tp: *mut c_void = core::ptr::null_mut();
    field = iter.ent;
    tp = trace_probe_primary_from_call(
    container_of!(event, trace_event_call, event));
    if (WARN_ON_ONCE!(!tp)) {
// goto;
    }
    trace_seq_printf(s, "%s: (", trace_probe_name(tp));
    if (!seq_print_ip_sym_offset(s, field.ip, flags)) {
// goto;
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
    static enum print_line_t
    print_kretprobe_event(trace_iterator *iter, int flags, trace_event *event)
    {
pub static mut field: *mut c_void = core::ptr::null_mut();
    let mut s = &iter.seq;
pub static mut tp: *mut c_void = core::ptr::null_mut();
    field = iter.ent;
    tp = trace_probe_primary_from_call(
    container_of!(event, trace_event_call, event));
    if (WARN_ON_ONCE!(!tp)) {
// goto;
    }
    trace_seq_printf(s, "%s: (", trace_probe_name(tp));
    if (!seq_print_ip_sym_offset(s, field.ret_ip, flags)) {
// goto;
    }
    trace_seq_puts(s, " <- ");
    if (!seq_print_ip_sym_no_offset(s, field.func, flags)) {
// goto;
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
#[no_mangle]
unsafe extern "C" fn kprobe_event_define_fields(event_call: *mut trace_event_call) -> c_int {
    let mut ret = 0;
pub static mut field: usize = 0;
pub static mut tp: *mut c_void = core::ptr::null_mut();
    tp = trace_probe_primary_from_call(event_call);
    if (WARN_ON_ONCE!(!tp)) {
    return -ENOENT;
    }
pub static mut unsigned long: usize = 0;
    return traceprobe_define_arg_fields(event_call, sizeof!(field), tp);
    }
#[no_mangle]
unsafe extern "C" fn kretprobe_event_define_fields(event_call: *mut trace_event_call) -> c_int {
    let mut ret = 0;
pub static mut field: usize = 0;
pub static mut tp: *mut c_void = core::ptr::null_mut();
    tp = trace_probe_primary_from_call(event_call);
    if (WARN_ON_ONCE!(!tp)) {
    return -ENOENT;
    }
pub static mut unsigned long: usize = 0;
pub static mut unsigned long: usize = 0;
    return traceprobe_define_arg_fields(event_call, sizeof!(field), tp);
    }

// Kprobe profile handler
#[no_mangle]
pub unsafe extern "C" fn kprobe_perf_func(tk: *mut trace_kprobe, regs: *mut pt_regs) -> c_int {
    let mut call = trace_probe_event_call(&tk.tp);
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut __size = 0;
    let mut dsize = 0;
    let mut rctx = 0;
    if (bpf_prog_array_valid(call)) {
pub static mut orig_ip: c_ulong = 0;
    let mut ret = 0;
    ret = trace_call_bpf(call, regs);
//
// We need to check and see if we modified the pc of the
// pt_regs, and if so return 1 so that we don't do the
// single stepping.
//
    if (orig_ip != instruction_pointer(regs)) {
    return 1;
    }
    if (!ret) {
    return 0;
    }
    }
    head = this_cpu_ptr(call.perf_events);
    if (hlist_empty(head)) {
    return 0;
    }
    dsize = __get_data_size(&tk.tp, regs, core::ptr::null_mut());
    __size = sizeof!(*entry) + tk.tp.size + dsize;
    size = ALIGN(__size + sizeof!(u32), sizeof!(u64));
    size -= sizeof!(u32);
    entry = perf_trace_buf_alloc(size, core::ptr::null_mut(), &rctx);
    if (!entry) {
    return 0;
    }
    entry.ip = (unsigned long)tk.rp.kp.addr;
    store_trace_args(&entry[1], &tk.tp, regs, core::ptr::null_mut(), sizeof!(*entry), dsize);
    perf_trace_buf_submit(entry, size, rctx, call.event.type, 1, regs,
    head, core::ptr::null_mut());
    return 0;
    }
    NOKPROBE_SYMBOL(kprobe_perf_func);
// Kretprobe profile handler
#[no_mangle]
pub unsafe extern "C" fn kretprobe_perf_func(tk: *mut trace_kprobe, ri: *mut kretprobe_instance, regs: *mut pt_regs) {
    let mut call = trace_probe_event_call(&tk.tp);
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut __size = 0;
    let mut dsize = 0;
    let mut rctx = 0;
    if (bpf_prog_array_valid(call) && !trace_call_bpf(call, regs)) {
    return;
    }
    head = this_cpu_ptr(call.perf_events);
    if (hlist_empty(head)) {
    return;
    }
    dsize = __get_data_size(&tk.tp, regs, ri.data);
    __size = sizeof!(*entry) + tk.tp.size + dsize;
    size = ALIGN(__size + sizeof!(u32), sizeof!(u64));
    size -= sizeof!(u32);
    entry = perf_trace_buf_alloc(size, core::ptr::null_mut(), &rctx);
    if (!entry) {
    return;
    }
    entry.func = (unsigned long)tk.rp.kp.addr;
    entry.ret_ip = get_kretprobe_retaddr(ri);
    store_trace_args(&entry[1], &tk.tp, regs, ri.data, sizeof!(*entry), dsize);
    perf_trace_buf_submit(entry, size, rctx, call.event.type, 1, regs,
    head, core::ptr::null_mut());
    }
    NOKPROBE_SYMBOL(kretprobe_perf_func);
#[no_mangle]
pub unsafe extern "C" fn bpf_get_kprobe_info(event: *mut perf_event, fd_type: *mut u32, symbol: *mut *mut c_char, probe_offset: *mut u64, probe_addr: *mut u64, missed: *mut c_ulong, perf_type_tracepoint: bool) -> c_int {
    let mut pevent = trace_event_name(event.tp_event);
    let mut group = event.tp_event.class.system;
pub static mut tk: *mut c_void = core::ptr::null_mut();
    if (perf_type_tracepoint) {
    tk = find_trace_kprobe(pevent, group);
    }
    else {
    tk = trace_kprobe_primary_from_call(event.tp_event);
    }
    if (!tk) {
    return -EINVAL;
    }
// fd_type = trace_kprobe_is_return(tk) ? BPF_FD_TYPE_KRETPROBE
    : BPF_FD_TYPE_KPROBE;
// probe_offset = tk->rp.kp.offset;
// probe_addr = kallsyms_show_value(current_cred()) ?
    (unsigned long)tk.rp.kp.addr : 0;
// symbol = tk->symbol;
    if (missed) {
// missed = trace_kprobe_missed(tk);
    }
    return 0;
    }

//
// called by perf_trace_init() or __ftrace_set_clr_event() under event_mutex.
//
// kprobe_trace_self_tests_init() does enable_trace_probe/disable_trace_probe
// lockless, but we can't race with this __init function.
//
#[no_mangle]
pub unsafe extern "C" fn kprobe_register(event: *mut trace_event_call, type: trace_reg, data: *mut c_void) -> c_int {
    let mut file = data;
    match (type) {
    TRACE_REG_REGISTER => {
    return enable_trace_kprobe(event, file);
    }
    TRACE_REG_UNREGISTER => {
    return disable_trace_kprobe(event, file);

    }
    TRACE_REG_PERF_REGISTER => {
    return enable_trace_kprobe(event, core::ptr::null_mut());
    }
    TRACE_REG_PERF_UNREGISTER => {
    return disable_trace_kprobe(event, core::ptr::null_mut());
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
unsafe extern "C" fn kprobe_dispatcher(kp: *mut kprobe, regs: *mut pt_regs) -> c_int {
    let mut tk = container_of!(kp, trace_kprobe, rp.kp);
pub static mut flags: c_uint = 0;
pub static mut ret: c_int = 0;
    raw_cpu_inc(*tk.nhit);
    if (flags & TP_FLAG_TRACE) {
    kprobe_trace_func(tk, regs);
    }

    if (flags & TP_FLAG_PROFILE) {
    ret = kprobe_perf_func(tk, regs);
    }

    return ret;
    }
    NOKPROBE_SYMBOL(kprobe_dispatcher);
#[no_mangle]
pub unsafe extern "C" fn kretprobe_dispatcher(ri: *mut kretprobe_instance, regs: *mut pt_regs) -> c_int {
    let mut rp = get_kretprobe(ri);
pub static mut tk: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
//
// There is a small chance that get_kretprobe(ri) returns NULL when
// the kretprobe is unregister on another CPU between kretprobe's
// trampoline_handler and this function.
//
    if (unlikely(!rp)) {
    return 0;
    }
    tk = container_of!(rp, trace_kprobe, rp);
    raw_cpu_inc(*tk.nhit);
    flags = trace_probe_load_flag(&tk.tp);
    if (flags & TP_FLAG_TRACE) {
    kretprobe_trace_func(tk, ri, regs);
    }

    if (flags & TP_FLAG_PROFILE) {
    kretprobe_perf_func(tk, ri, regs);
    }

    return 0;	/* We don't tweak kernel, so just return 0 */
    }
    NOKPROBE_SYMBOL(kretprobe_dispatcher);
pub static mut trace_event_functions: usize = 0;
pub static mut trace_event_functions: usize = 0;
pub static mut trace_event_fields: usize = 0;
pub static mut trace_event_fields: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn init_trace_event_call(tk: *mut trace_kprobe) {
    let mut call = trace_probe_event_call(&tk.tp);
    if (trace_kprobe_is_return(tk)) {
    call.event.funcs = &kretprobe_funcs;
    call.class.fields_array = kretprobe_fields_array;
    } else {
    call.event.funcs = &kprobe_funcs;
    call.class.fields_array = kprobe_fields_array;
    }
    call.flags = TRACE_EVENT_FL_KPROBE;
    call.class.reg = kprobe_register;
    }
#[no_mangle]
unsafe extern "C" fn register_kprobe_event(tk: *mut trace_kprobe) -> c_int {
    init_trace_event_call(tk);
    return trace_probe_register_event_call(&tk.tp);
    }
#[no_mangle]
unsafe extern "C" fn unregister_kprobe_event(tk: *mut trace_kprobe) -> c_int {
    return trace_probe_unregister_event_call(&tk.tp);
    }

// create a trace_kprobe, but don't add it to global lists
#[no_mangle]
pub unsafe extern "C" fn create_local_trace_kprobe(func: *mut c_char, addr: *mut c_void, offs: c_ulong, is_return: bool) -> *mut c_void {
    enum probe_print_type ptype;
    struct trace_kprobe *tk __free(free_trace_kprobe) = core::ptr::null_mut();
    let mut ret = 0;
pub static mut event: *mut c_void = core::ptr::null_mut();
    if (func) {
    ret = validate_probe_symbol(func);
    if (ret) {
    return ERR_PTR(ret);
    }
    }
//
// local trace_kprobes are not added to dyn_event, so they are never
// searched in find_trace_kprobe(). Therefore, there is no concern of
// duplicated name here.
//
    event = func ? func : "DUMMY_EVENT";
    tk = alloc_trace_kprobe(KPROBE_EVENT_SYSTEM, event, addr, func,
    offs, 0 /* maxactive */, 0 /* nargs */,
    is_return);
    if (IS_ERR(tk)) {
    pr_info!("Failed to allocate trace_probe.(%d)\n",
    (int)PTR_ERR(tk));
    return ERR_CAST(tk);
    }
    init_trace_event_call(tk);
    ptype = trace_kprobe_is_return(tk) ?
    PROBE_PRINT_RETURN : PROBE_PRINT_NORMAL;
    if (traceprobe_set_print_fmt(&tk.tp, ptype) < 0) {
    return ERR_PTR(-ENOMEM);
    }
    ret = __register_trace_kprobe(tk);
    if (ret < 0) {
    return ERR_PTR(ret);
    }
    return trace_probe_event_call(&(no_free_ptr(tk).tp));
    }
#[no_mangle]
pub unsafe extern "C" fn destroy_local_trace_kprobe(event_call: *mut trace_event_call) {
pub static mut tk: *mut c_void = core::ptr::null_mut();
    tk = trace_kprobe_primary_from_call(event_call);
    if (unlikely(!tk)) {
    return;
    }
    if (trace_probe_is_enabled(&tk.tp)) {
    WARN_ON!(1);
    return;
    }
    __unregister_trace_kprobe(tk);
    free_trace_kprobe(tk);
    }

#[no_mangle]
unsafe extern "C" fn enable_boot_kprobe_events() -> __init void {
    let mut tr = top_trace_array();
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut tk: *mut c_void = core::ptr::null_mut();
pub static mut pos: *mut c_void = core::ptr::null_mut();
    if (trace_kprobe_list_empty()) {
    return;
    }
    guard(mutex)(&event_mutex);
    for_each_trace_kprobe(tk, pos) {
    list_for_each_entry(file, &tr.events, list) {
    if (file.event_call == trace_probe_event_call(&tk.tp))
    trace_event_enable_disable(file, 1, 0);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn setup_boot_kprobe_events() -> __init void {
    char *p, *cmd = kprobe_boot_events_buf;
    let mut ret = 0;
    strreplace(kprobe_boot_events_buf, ',', ' ');
    while (cmd && *cmd != '\0') {
    p = strchr(cmd, ';');
    if (p) {
// p++ = '\0';
    }
    ret = create_or_delete_trace_kprobe(cmd);
    if (ret) {
    pr_warn!("Failed to add event(%d): %s\n", ret, cmd);
    }
    cmd = p;
    }
    enable_boot_kprobe_events();
    }
//
// Register dynevent at core_initcall. This allows kernel to setup kprobe
// events in postcore_initcall without tracefs.
//
#[no_mangle]
unsafe extern "C" fn init_kprobe_trace_early() -> __init int {
    let mut ret = 0;
    ret = dyn_event_register(&trace_kprobe_ops);
    if (ret) {
    return ret;
    }
    if (trace_kprobe_register_module_notifier()) {
    return -EINVAL;
    }
    return 0;
    }
    core_initcall!(init_kprobe_trace_early);
// Make a tracefs interface for controlling probe points
#[no_mangle]
unsafe extern "C" fn init_kprobe_trace() -> __init int {
    let mut ret = 0;
    ret = tracing_init_dentry();
    if (ret) {
    return 0;
    }
// Event list interface
    trace_create_file("kprobe_events", TRACE_MODE_WRITE,
    core::ptr::null_mut(), core::ptr::null_mut(), &kprobe_events_ops);
// Profile interface
    trace_create_file("kprobe_profile", TRACE_MODE_READ,
    core::ptr::null_mut(), core::ptr::null_mut(), &kprobe_profile_ops);
// If no 'kprobe_event=' cmd is provided, return directly.
    if (kprobe_boot_events_buf[0] == '\0') {
    return 0;
    }
    setup_boot_kprobe_events();
    return 0;
    }
    fs_initcall!(init_kprobe_trace);

#[no_mangle]
pub unsafe extern "C" fn find_trace_probe_file(tk: *mut trace_kprobe, tr: *mut trace_array) -> *mut c_void {
pub static mut file: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(file, &tr.events, list) {
    if (file.event_call == trace_probe_event_call(&tk.tp))
    return file;
    }
    return core::ptr::null_mut();
    }
//
// Nobody but us can call enable_trace_kprobe/disable_trace_kprobe at this
// stage, we can do this lockless.
//
#[no_mangle]
unsafe extern "C" fn kprobe_trace_self_tests_init() -> __init int {
    int ret, warn = 0;
    int (*target)(int, int, int, int, int, int);
pub static mut tk: *mut c_void = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
    if (unlikely(tracing_disabled)) {
    return -ENODEV;
    }
    if (tracing_selftest_disabled) {
    return 0;
    }
    target = kprobe_trace_selftest_target;
    pr_info!("Testing kprobe tracing: ");
    ret = create_or_delete_trace_kprobe("p:testprobe kprobe_trace_selftest_target $stack $stack0 +0($stack)");
    if (WARN_ONCE(ret, "error on probing function entry.")) {
    warn += 1;
    } else {
// Enable trace point
    tk = find_trace_kprobe("testprobe", KPROBE_EVENT_SYSTEM);
    if (WARN_ONCE(tk == core::ptr::null_mut(), "error on probing function entry.")) {
    warn += 1;
    } else {
    file = find_trace_probe_file(tk, top_trace_array());
    if (WARN_ONCE(file == core::ptr::null_mut(), "error on getting probe file.")) {
    warn += 1;
    } else {
    enable_trace_kprobe(
    trace_probe_event_call(&tk.tp), file);
    }
    }
    }
    ret = create_or_delete_trace_kprobe("r:testprobe2 kprobe_trace_selftest_target $retval");
    if (WARN_ONCE(ret, "error on probing function return.")) {
    warn += 1;
    } else {
// Enable trace point
    tk = find_trace_kprobe("testprobe2", KPROBE_EVENT_SYSTEM);
    if (WARN_ONCE(tk == core::ptr::null_mut(), "error on getting 2nd new probe.")) {
    warn += 1;
    } else {
    file = find_trace_probe_file(tk, top_trace_array());
    if (WARN_ONCE(file == core::ptr::null_mut(), "error on getting probe file.")) {
    warn += 1;
    } else {
    enable_trace_kprobe(
    trace_probe_event_call(&tk.tp), file);
    }
    }
    }
    if (warn) {
// goto;
    }
    ret = target(1, 2, 3, 4, 5, 6);
//
// Not expecting an error here, the check is only to prevent the
// optimizer from removing the call to target() as otherwise there
// are no side-effects and the call is never performed.
//
    if (ret != 21) {
    warn += 1;
    }
// Disable trace points before removing it
    tk = find_trace_kprobe("testprobe", KPROBE_EVENT_SYSTEM);
    if (WARN_ONCE(tk == core::ptr::null_mut(), "error on getting test probe.")) {
    warn += 1;
    } else {
    if (WARN_ONCE(trace_kprobe_nhit(tk) != 1,
    "incorrect number of testprobe hits.")) {
    warn += 1;
    }
    file = find_trace_probe_file(tk, top_trace_array());
    if (WARN_ONCE(file == core::ptr::null_mut(), "error on getting probe file.")) {
    warn += 1;
    } else {
    disable_trace_kprobe(
    trace_probe_event_call(&tk.tp), file);
    }
    }
    tk = find_trace_kprobe("testprobe2", KPROBE_EVENT_SYSTEM);
    if (WARN_ONCE(tk == core::ptr::null_mut(), "error on getting 2nd test probe.")) {
    warn += 1;
    } else {
    if (WARN_ONCE(trace_kprobe_nhit(tk) != 1,
    "incorrect number of testprobe2 hits.")) {
    warn += 1;
    }
    file = find_trace_probe_file(tk, top_trace_array());
    if (WARN_ONCE(file == core::ptr::null_mut(), "error on getting probe file.")) {
    warn += 1;
    } else {
    disable_trace_kprobe(
    trace_probe_event_call(&tk.tp), file);
    }
    }
    ret = create_or_delete_trace_kprobe("-:testprobe");
    if (WARN_ONCE(ret, "error on deleting a probe.")) {
    warn += 1;
    }
    ret = create_or_delete_trace_kprobe("-:testprobe2");
    if (WARN_ONCE(ret, "error on deleting a probe.")) {
    warn += 1;
    }
// label;
//
// Wait for the optimizer work to finish. Otherwise it might fiddle
// with probes in already freed __init text.
//
    wait_for_kprobe_optimizer();
    if (warn) {
    pr_cont("NG: Some tests are failed. Please check them.\n");
    }
    else {
    pr_cont("OK\n");
    }
    return 0;
    }
    late_initcall!(kprobe_trace_self_tests_init);
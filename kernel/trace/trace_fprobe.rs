//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_fprobe.c
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
// Fprobe-based tracing events
// Copyright (C) 2022 Google LLC.
//

pub const RETHOOK_MAXACTIVE_MAX: c_int = 4096;
// forward_decl: trace_fprobe_create;
// forward_decl: trace_fprobe_show;
// forward_decl: trace_fprobe_release;
// forward_decl: trace_fprobe_is_busy;
// forward_decl: trace_fprobe_match;
pub static mut dyn_event_operations: usize = 0;
// List of tracepoint_user
pub static mut tracepoint_user_list: usize = 0;
pub static mut tracepoint_user_mutex: usize = 0;
// While living tracepoint_user, @tpoint can be NULL and @refcount != 0.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tracepoint_user {
    pub list: list_head,
    pub name: *const c_char,
    pub tpoint: *mut tracepoint,
    pub refcount: c_uint,
}

// NOTE: you must lock tracepoint_user_mutex.

    list_for_each_entry(tuser, &tracepoint_user_list, list) {
#[no_mangle]
unsafe extern "C" fn tracepoint_user_register(tuser: *mut tracepoint_user) -> c_int {
    }
    let mut tpoint = tuser.tpoint;
    if (!tpoint) {
    return 0;
    }
    return tracepoint_probe_register_prio_may_exist(tpoint,
    tpoint.probestub, core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn tracepoint_user_unregister(tuser: *mut tracepoint_user) {
    if (!tuser.tpoint) {
    return;
    }
    WARN_ON_ONCE!(tracepoint_probe_unregister(tuser.tpoint, tuser.tpoint.probestub, core::ptr::null_mut()));
    tuser.tpoint = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn tracepoint_user_ip(tuser: *mut tracepoint_user) -> c_ulong {
    if (!tuser.tpoint) {
    return 0UL;
    }
    return (unsigned long)tuser.tpoint.probestub;
    }
#[no_mangle]
unsafe extern "C" fn __tracepoint_user_free(tuser: *mut tracepoint_user) {
    if (!tuser) {
    return;
    }
    kfree(tuser.name);
    kfree(tuser);
    }
    DEFINE_FREE(tuser_free, tracepoint_user *, __tracepoint_user_free(_T))
#[no_mangle]
pub unsafe extern "C" fn __tracepoint_user_init(name: *mut c_char, tpoint: *mut tracepoint) -> *mut c_void {
    struct tracepoint_user *tuser __free(tuser_free) = core::ptr::null_mut();
    let mut ret = 0;
    tuser = kzalloc_obj(*tuser);
    if (!tuser) {
    return core::ptr::null_mut();
    }
    tuser.name = kstrdup(name, GFP_KERNEL);
    if (!tuser.name) {
    return core::ptr::null_mut();
    }
// Register tracepoint if it is loaded.
    if (tpoint) {
    tuser.tpoint = tpoint;
    ret = tracepoint_user_register(tuser);
    if (ret) {
    return ERR_PTR(ret);
    }
    }
    tuser.refcount = 1;
    INIT_LIST_HEAD(&tuser.list);
    list_add(&tuser.list, &tracepoint_user_list);
    return_ptr(tuser);
    }
// forward_decl: find_tracepoint;
//
// Get tracepoint_user if exist, or allocate new one and register it.
// If tracepoint is on a module, get its refcounter too.
// This returns errno or NULL (not loaded yet) or tracepoint_user.
//
#[no_mangle]
pub unsafe extern "C" fn tracepoint_user_find_get(name: *mut c_char, pmod: *mut *mut module) -> *mut c_void {
    struct module *mod __free(module_put) = core::ptr::null_mut();
pub static mut tuser: *mut c_void = core::ptr::null_mut();
pub static mut tpoint: *mut c_void = core::ptr::null_mut();
    if (!name || !pmod) {
    return ERR_PTR(-EINVAL);
    }
// Get and lock the module which has tracepoint.
    tpoint = find_tracepoint(name, &mod);
    guard(mutex)(&tracepoint_user_mutex);
// Search existing tracepoint_user
    for_each_tracepoint_user(tuser) {
    if (!strcmp(tuser.name, name)) {
    tuser.refcount += 1;
// pmod = no_free_ptr(mod);
    return tuser;
    }
    }
// The corresponding tracepoint_user is not found.
    tuser = __tracepoint_user_init(name, tpoint);
    if (!IS_ERR_OR_NULL(tuser)) {
// pmod = no_free_ptr(mod);
    }
    return tuser;
    }
#[no_mangle]
unsafe extern "C" fn tracepoint_user_put(tuser: *mut tracepoint_user) {
    scoped_guard(mutex, &tracepoint_user_mutex) {
    if (--tuser.refcount > 0) {
    return;
    }
    list_del(&tuser.list);
    tracepoint_user_unregister(tuser);
    }
    __tracepoint_user_free(tuser);
    }
    DEFINE_FREE(tuser_put, tracepoint_user *,
    if (!IS_ERR_OR_NULL(_T)) {
    tracepoint_user_put(_T))
//
// Fprobe event core functions
//
// @tprobe is true for tracepoint probe.
// @tuser can be NULL if the trace_fprobe is disabled or the tracepoint is not
// loaded with a module. If @tuser != NULL, this trace_fprobe is enabled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_fprobe {
    }
    pub devent: dyn_event,
    pub fp: fprobe,
    pub symbol: *const c_char,
    pub tprobe: bool,
    pub tuser: *mut tracepoint_user,
    pub tp: trace_probe,
}

#[no_mangle]
unsafe extern "C" fn is_trace_fprobe(ev: *mut dyn_event) -> bool {
    return ev.ops == &trace_fprobe_ops;
    }
#[no_mangle]
pub unsafe extern "C" fn to_trace_fprobe(ev: *mut dyn_event) -> *mut c_void {
    return container_of!(ev, trace_fprobe, devent);
    }
//
// for_each_trace_fprobe - iterate over the trace_fprobe list
// @pos:	the struct trace_fprobe * for each entry
// @dpos:	the struct dyn_event * to use as a loop cursor
//

    for_each_dyn_event(dpos)		 {
    if (is_trace_fprobe(dpos) && (pos = to_trace_fprobe(dpos)))
#[no_mangle]
unsafe extern "C" fn trace_fprobe_is_return(tf: *mut trace_fprobe) -> bool {
    }
    return tf.fp.exit_handler != core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn trace_fprobe_is_tracepoint(tf: *mut trace_fprobe) -> bool {
    return tf.tprobe;
    }
    static const char *trace_fprobe_symbol(trace_fprobe *tf)
    {
    return tf.symbol ? tf.symbol : "unknown";
    }
#[no_mangle]
unsafe extern "C" fn trace_fprobe_is_busy(ev: *mut dyn_event) -> bool {
    let mut tf = to_trace_fprobe(ev);
    return trace_probe_is_enabled(&tf.tp);
    }
#[no_mangle]
pub unsafe extern "C" fn trace_fprobe_match_command_head(tf: *mut trace_fprobe, argc: c_int, argv: *mut *mut c_char) -> bool {
    if (!argc) {
    return true;
    }
    if (strcmp(trace_fprobe_symbol(tf), argv[0])) {
    return false;
    }
    argc -= 1; argv += 1;
    return trace_probe_match_command_args(&tf.tp, argc, argv);
    }
#[no_mangle]
pub unsafe extern "C" fn trace_fprobe_match(system: *mut c_char, event: *mut c_char, argc: c_int, argv: *mut *mut c_char, ev: *mut dyn_event) -> bool {
    let mut tf = to_trace_fprobe(ev);
    if (event[0] != '\0' && strcmp(trace_probe_name(&tf.tp), event)) {
    return false;
    }
    if (system && strcmp(trace_probe_group_name(&tf.tp), system)) {
    return false;
    }
    return trace_fprobe_match_command_head(tf, argc, argv);
    }
#[no_mangle]
unsafe extern "C" fn trace_fprobe_is_registered(tf: *mut trace_fprobe) -> bool {
    return fprobe_is_registered(&tf.fp);
    }
//
// Note that we don't verify the fetch_insn code, since it does not come
// from user space.
//
#[no_mangle]
pub unsafe extern "C" fn process_fetch_insn(code: *mut fetch_insn, rec: *mut c_void, edata: *mut c_void, dest: *mut c_void, base: *mut c_void) -> c_int {
    let mut fregs = rec;
    let mut val = 0;
    let mut ret = 0;
// label;
// 1st stage: get value from context
    match (code.op) {
    FETCH_OP_STACK => {
    val = ftrace_regs_get_kernel_stack_nth(fregs, code.param);
    // break;
    }
    FETCH_OP_STACKP => {
    val = ftrace_regs_get_stack_pointer(fregs);
    // break;
    }
    FETCH_OP_RETVAL => {
    val = ftrace_regs_get_return_value(fregs);
    // break;

    }
    FETCH_OP_ARG => {
    val = ftrace_regs_get_argument(fregs, code.param);
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
// function entry handler
    static nokprobe_inline void
    __fentry_trace_func(trace_fprobe *tf, unsigned long entry_ip, ftrace_regs *fregs, trace_event_file *trace_file)
    {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut call = trace_probe_event_call(&tf.tp);
pub static mut fbuffer: usize = 0;
    let mut dsize = 0;
    if (WARN_ON_ONCE!(call != trace_file.event_call)) {
    return;
    }
    if (trace_trigger_soft_disabled(trace_file)) {
    return;
    }
    dsize = __get_data_size(&tf.tp, fregs, core::ptr::null_mut());
    entry = trace_event_buffer_reserve(&fbuffer, trace_file,
    sizeof!(*entry) + tf.tp.size + dsize);
    if (!entry) {
    return;
    }
    fbuffer.regs = ftrace_get_regs(fregs);
    entry = fbuffer.entry = ring_buffer_event_data(fbuffer.event);
    entry.ip = entry_ip;
    store_trace_args(&entry[1], &tf.tp, fregs, core::ptr::null_mut(), sizeof!(*entry), dsize);
    trace_event_buffer_commit(&fbuffer);
    }
#[no_mangle]
pub unsafe extern "C" fn fentry_trace_func(tf: *mut trace_fprobe, entry_ip: c_ulong, fregs: *mut ftrace_regs) {
pub static mut link: *mut c_void = core::ptr::null_mut();
    trace_probe_for_each_link_rcu(link, &tf.tp)
    __fentry_trace_func(tf, entry_ip, fregs, link.file);
    }
    NOKPROBE_SYMBOL(fentry_trace_func);
    static nokprobe_inline
#[no_mangle]
pub unsafe extern "C" fn store_fprobe_entry_data(edata: *mut c_void, tp: *mut trace_probe, fregs: *mut ftrace_regs) {
    let mut earg = tp.entry_arg;
pub static mut val: c_ulong = 0;
    let mut i = 0;
    if (!earg) {
    return;
    }
    while (i < earg.size) {
    let mut code = &earg.code[i];
    match (code.op) {
    FETCH_OP_ARG => {
    val = ftrace_regs_get_argument(fregs, code.param);
    // break;
    }
    FETCH_OP_ST_EDATA => {
// ((unsigned long)edata + code->offset) = val;
    // break;
    }
    FETCH_OP_END => {
// goto;
    }
    _ => {
    // break;
    }
    }
    }
// label;
    return;
    }
// function exit handler
#[no_mangle]
pub unsafe extern "C" fn trace_fprobe_entry_handler(fp: *mut fprobe, entry_ip: c_ulong, ret_ip: c_ulong, fregs: *mut ftrace_regs, entry_data: *mut c_void) -> c_int {
    let mut tf = container_of!(fp, trace_fprobe, fp);
    if (tf.tp.entry_arg) {
    store_fprobe_entry_data(entry_data, &tf.tp, fregs);
    }
    return 0;
    }
    NOKPROBE_SYMBOL(trace_fprobe_entry_handler)
    static nokprobe_inline void
    __fexit_trace_func(trace_fprobe *tf, unsigned long entry_ip,
    unsigned long ret_ip, ftrace_regs *fregs,
    void *entry_data, trace_event_file *trace_file)
    {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut fbuffer: usize = 0;
    let mut call = trace_probe_event_call(&tf.tp);
    let mut dsize = 0;
    if (WARN_ON_ONCE!(call != trace_file.event_call)) {
    return;
    }
    if (trace_trigger_soft_disabled(trace_file)) {
    return;
    }
    dsize = __get_data_size(&tf.tp, fregs, entry_data);
    entry = trace_event_buffer_reserve(&fbuffer, trace_file,
    sizeof!(*entry) + tf.tp.size + dsize);
    if (!entry) {
    return;
    }
    fbuffer.regs = ftrace_get_regs(fregs);
    entry = fbuffer.entry = ring_buffer_event_data(fbuffer.event);
    entry.func = entry_ip;
    entry.ret_ip = ret_ip;
    store_trace_args(&entry[1], &tf.tp, fregs, entry_data, sizeof!(*entry), dsize);
    trace_event_buffer_commit(&fbuffer);
    }
#[no_mangle]
pub unsafe extern "C" fn fexit_trace_func(tf: *mut trace_fprobe, entry_ip: c_ulong, ret_ip: c_ulong, fregs: *mut ftrace_regs, entry_data: *mut c_void) {
pub static mut link: *mut c_void = core::ptr::null_mut();
    trace_probe_for_each_link_rcu(link, &tf.tp)
    __fexit_trace_func(tf, entry_ip, ret_ip, fregs, entry_data, link.file);
    }
    NOKPROBE_SYMBOL(fexit_trace_func);

#[no_mangle]
pub unsafe extern "C" fn fentry_perf_func(tf: *mut trace_fprobe, entry_ip: c_ulong, fregs: *mut ftrace_regs) -> c_int {
    let mut call = trace_probe_event_call(&tf.tp);
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut __size = 0;
    let mut dsize = 0;
pub static mut regs: *mut c_void = core::ptr::null_mut();
    let mut rctx = 0;
    head = this_cpu_ptr(call.perf_events);
    if (hlist_empty(head)) {
    return 0;
    }
    dsize = __get_data_size(&tf.tp, fregs, core::ptr::null_mut());
    __size = sizeof!(*entry) + tf.tp.size + dsize;
    size = ALIGN(__size + sizeof!(u32), sizeof!(u64));
    size -= sizeof!(u32);
    entry = perf_trace_buf_alloc(size, &regs, &rctx);
    if (!entry) {
    return 0;
    }
    regs = ftrace_fill_perf_regs(fregs, regs);
    entry.ip = entry_ip;
    store_trace_args(&entry[1], &tf.tp, fregs, core::ptr::null_mut(), sizeof!(*entry), dsize);
    perf_trace_buf_submit(entry, size, rctx, call.event.type, 1, regs,
    head, core::ptr::null_mut());
    return 0;
    }
    NOKPROBE_SYMBOL(fentry_perf_func);
#[no_mangle]
pub unsafe extern "C" fn fexit_perf_func(tf: *mut trace_fprobe, entry_ip: c_ulong, ret_ip: c_ulong, fregs: *mut ftrace_regs, entry_data: *mut c_void) {
    let mut call = trace_probe_event_call(&tf.tp);
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut __size = 0;
    let mut dsize = 0;
pub static mut regs: *mut c_void = core::ptr::null_mut();
    let mut rctx = 0;
    head = this_cpu_ptr(call.perf_events);
    if (hlist_empty(head)) {
    return;
    }
    dsize = __get_data_size(&tf.tp, fregs, entry_data);
    __size = sizeof!(*entry) + tf.tp.size + dsize;
    size = ALIGN(__size + sizeof!(u32), sizeof!(u64));
    size -= sizeof!(u32);
    entry = perf_trace_buf_alloc(size, &regs, &rctx);
    if (!entry) {
    return;
    }
    regs = ftrace_fill_perf_regs(fregs, regs);
    entry.func = entry_ip;
    entry.ret_ip = ret_ip;
    store_trace_args(&entry[1], &tf.tp, fregs, entry_data, sizeof!(*entry), dsize);
    perf_trace_buf_submit(entry, size, rctx, call.event.type, 1, regs,
    head, core::ptr::null_mut());
    }
    NOKPROBE_SYMBOL(fexit_perf_func);

#[no_mangle]
pub unsafe extern "C" fn fentry_dispatcher(fp: *mut fprobe, entry_ip: c_ulong, ret_ip: c_ulong, fregs: *mut ftrace_regs, entry_data: *mut c_void) -> c_int {
    let mut tf = container_of!(fp, trace_fprobe, fp);
pub static mut flags: c_uint = 0;
pub static mut ret: c_int = 0;
    if (flags & TP_FLAG_TRACE) {
    fentry_trace_func(tf, entry_ip, fregs);
    }

    if (flags & TP_FLAG_PROFILE) {
    ret = fentry_perf_func(tf, entry_ip, fregs);
    }

    return ret;
    }
    NOKPROBE_SYMBOL(fentry_dispatcher);
#[no_mangle]
pub unsafe extern "C" fn fexit_dispatcher(fp: *mut fprobe, entry_ip: c_ulong, ret_ip: c_ulong, fregs: *mut ftrace_regs, entry_data: *mut c_void) {
    let mut tf = container_of!(fp, trace_fprobe, fp);
pub static mut flags: c_uint = 0;
    if (flags & TP_FLAG_TRACE) {
    fexit_trace_func(tf, entry_ip, ret_ip, fregs, entry_data);
    }

    if (flags & TP_FLAG_PROFILE) {
    fexit_perf_func(tf, entry_ip, ret_ip, fregs, entry_data);
    }

    }
    NOKPROBE_SYMBOL(fexit_dispatcher);
#[no_mangle]
unsafe extern "C" fn free_trace_fprobe(tf: *mut trace_fprobe) {
    if (tf) {
    trace_probe_cleanup(&tf.tp);
    if (tf.tuser) {
    tracepoint_user_put(tf.tuser);
    }
    kfree(tf.symbol);
    kfree(tf);
    }
    }
// Since alloc_trace_fprobe() can return error, check the pointer is ERR too.
    DEFINE_FREE(free_trace_fprobe, trace_fprobe *, if (!IS_ERR_OR_NULL(_T)) free_trace_fprobe(_T))
//
// Allocate new trace_probe and initialize it (including fprobe).
//
#[no_mangle]
pub unsafe extern "C" fn alloc_trace_fprobe(group: *mut c_char, event: *mut c_char, symbol: *mut c_char, nargs: c_int, is_return: bool, is_tracepoint: bool) -> *mut c_void {
    struct trace_fprobe *tf __free(free_trace_fprobe) = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    tf = kzalloc_flex(*tf, tp.args, nargs);
    if (!tf) {
    return ERR_PTR(ret);
    }
    tf.symbol = kstrdup(symbol, GFP_KERNEL);
    if (!tf.symbol) {
    return ERR_PTR(-ENOMEM);
    }
    if (is_return) {
    tf.fp.exit_handler = fexit_dispatcher;
    }
    else {
    tf.fp.entry_handler = fentry_dispatcher;
    }
    tf.tprobe = is_tracepoint;
    ret = trace_probe_init(&tf.tp, event, group, false, nargs);
    if (ret < 0) {
    return ERR_PTR(ret);
    }
    dyn_event_init(&tf.devent, &trace_fprobe_ops);
    return_ptr(tf);
    }
#[no_mangle]
pub unsafe extern "C" fn find_trace_fprobe(event: *mut c_char, group: *mut c_char) -> *mut c_void {
pub static mut pos: *mut c_void = core::ptr::null_mut();
pub static mut tf: *mut c_void = core::ptr::null_mut();
    for_each_trace_fprobe(tf, pos) {
    if (strcmp(trace_probe_name(&tf.tp), event) == 0 &&
    strcmp(trace_probe_group_name(&tf.tp), group) == 0)
    return tf;
    }
    return core::ptr::null_mut();
    }
// Event entry printers
    static enum print_line_t
    print_fentry_event(trace_iterator *iter, int flags, trace_event *event)
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
    print_fexit_event(trace_iterator *iter, int flags, trace_event *event)
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
unsafe extern "C" fn fentry_event_define_fields(event_call: *mut trace_event_call) -> c_int {
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
unsafe extern "C" fn fexit_event_define_fields(event_call: *mut trace_event_call) -> c_int {
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
pub static mut trace_event_functions: usize = 0;
pub static mut trace_event_functions: usize = 0;
pub static mut trace_event_fields: usize = 0;
pub static mut trace_event_fields: usize = 0;
// forward_decl: fprobe_register;
#[no_mangle]
pub unsafe extern "C" fn init_trace_event_call(tf: *mut trace_fprobe) {
    let mut call = trace_probe_event_call(&tf.tp);
    if (trace_fprobe_is_return(tf)) {
    call.event.funcs = &fexit_funcs;
    call.class.fields_array = fexit_fields_array;
    } else {
    call.event.funcs = &fentry_funcs;
    call.class.fields_array = fentry_fields_array;
    }
    call.flags = TRACE_EVENT_FL_FPROBE;
    call.class.reg = fprobe_register;
    }
#[no_mangle]
unsafe extern "C" fn register_fprobe_event(tf: *mut trace_fprobe) -> c_int {
    init_trace_event_call(tf);
    return trace_probe_register_event_call(&tf.tp);
    }
#[no_mangle]
unsafe extern "C" fn unregister_fprobe_event(tf: *mut trace_fprobe) -> c_int {
    return trace_probe_unregister_event_call(&tf.tp);
    }
#[no_mangle]
unsafe extern "C" fn __register_tracepoint_fprobe(tf: *mut trace_fprobe) -> c_int {
    struct tracepoint_user *tuser __free(tuser_put) = core::ptr::null_mut();
    struct module *mod __free(module_put) = core::ptr::null_mut();
    let mut ip = 0;
    let mut ret = 0;
    if (WARN_ON_ONCE!(tf.tuser)) {
    return -EINVAL;
    }
// If the tracepoint is in a module, it must be locked in this function.
    tuser = tracepoint_user_find_get(tf.symbol, &mod);
// This tracepoint is not loaded yet
    if (IS_ERR(tuser)) {
    return PTR_ERR(tuser);
    }
    if (!tuser) {
    return -ENOMEM;
    }
// Register fprobe only if the tracepoint is loaded.
    if (tuser.tpoint) {
    ip = tracepoint_user_ip(tuser);
    if (WARN_ON_ONCE!(!ip)) {
    return -ENOENT;
    }
    ret = register_fprobe_ips(&tf.fp, &ip, 1);
    if (ret < 0) {
    return ret;
    }
    }
    tf.tuser = no_free_ptr(tuser);
    return 0;
    }
// Returns an error if the target function is not available, or 0
#[no_mangle]
unsafe extern "C" fn trace_fprobe_verify_target(tf: *mut trace_fprobe) -> c_int {
    let mut ret = 0;
// Tracepoint should have a stub function.
    if (trace_fprobe_is_tracepoint(tf)) {
    return 0;
    }
//
// Note: since we don't lock the module, even if this succeeded,
// register_fprobe() later can fail.
//
    ret = fprobe_count_ips_from_filter(tf.symbol, core::ptr::null_mut());
    return (ret < 0) ? ret : 0;
    }
// Internal register function - just handle fprobe and flags
#[no_mangle]
unsafe extern "C" fn __register_trace_fprobe(tf: *mut trace_fprobe) -> c_int {
    let mut i = 0;
    let mut ret = 0;
// Should we need new LOCKDOWN flag for fprobe?
    ret = security_locked_down(LOCKDOWN_KPROBES);
    if (ret) {
    return ret;
    }
    if (trace_fprobe_is_registered(tf)) {
    return -EINVAL;
    }
    while (i < tf.tp.nr_args) {
    ret = traceprobe_update_arg(&tf.tp.args[i]);
    if (ret) {
    return ret;
    }
    }
    tf.fp.flags &= ~FPROBE_FL_DISABLED;
    if (trace_fprobe_is_tracepoint(tf)) {
    return __register_tracepoint_fprobe(tf);
    }
// TODO: handle filter, nofilter or symbol list
    return register_fprobe(&tf.fp, tf.symbol, core::ptr::null_mut());
    }
// Internal unregister function - just handle fprobe and flags
#[no_mangle]
unsafe extern "C" fn __unregister_trace_fprobe(tf: *mut trace_fprobe) {
    if (trace_fprobe_is_registered(tf)) {
    unregister_fprobe(&tf.fp);
    }
    if (tf.tuser) {
    tracepoint_user_put(tf.tuser);
    tf.tuser = core::ptr::null_mut();
    }
    }
// TODO: make this trace_*probe common function
// Unregister a trace_probe and probe_event
#[no_mangle]
unsafe extern "C" fn unregister_trace_fprobe(tf: *mut trace_fprobe) -> c_int {
// If other probes are on the event, just unregister fprobe
    if (trace_probe_has_sibling(&tf.tp)) {
// goto;
    }
// Enabled event can not be unregistered
    if (trace_probe_is_enabled(&tf.tp)) {
    return -EBUSY;
    }
// If there's a reference to the dynamic event
    if (trace_event_dyn_busy(trace_probe_event_call(&tf.tp))) {
    return -EBUSY;
    }
// Will fail if probe is being used by ftrace or perf
    if (unregister_fprobe_event(tf)) {
    return -EBUSY;
    }
// label;
    __unregister_trace_fprobe(tf);
    dyn_event_remove(&tf.devent);
    trace_probe_unlink(&tf.tp);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_fprobe_has_same_fprobe(orig: *mut trace_fprobe, comp: *mut trace_fprobe) -> bool {
    let mut tpe = orig.tp.event;
    let mut i = 0;
    list_for_each_entry(orig, &tpe.probes, tp.list) {
    if (strcmp(trace_fprobe_symbol(orig),
    trace_fprobe_symbol(comp))) {
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
unsafe extern "C" fn append_trace_fprobe_event(tf: *mut trace_fprobe, to: *mut trace_fprobe) -> c_int {
    let mut ret = 0;
    if (trace_fprobe_is_return(tf) != trace_fprobe_is_return(to) ||
    trace_fprobe_is_tracepoint(tf) != trace_fprobe_is_tracepoint(to)) {
    trace_probe_log_set_index(0);
    trace_probe_log_err(0, DIFF_PROBE_TYPE);
    return -EEXIST;
    }
    ret = trace_probe_compare_arg_type(&tf.tp, &to.tp);
    if (ret) {
// Note that argument starts index = 2
    trace_probe_log_set_index(ret + 1);
    trace_probe_log_err(0, DIFF_ARG_TYPE);
    return -EEXIST;
    }
    if (trace_fprobe_has_same_fprobe(to, tf)) {
    trace_probe_log_set_index(0);
    trace_probe_log_err(0, SAME_PROBE);
    return -EEXIST;
    }
// Append to existing event
    ret = trace_probe_append(&tf.tp, &to.tp);
    if (ret) {
    return ret;
    }
    ret = trace_fprobe_verify_target(tf);
    if (ret) {
    trace_probe_unlink(&tf.tp);
    }
    else {
    dyn_event_add(&tf.devent, trace_probe_event_call(&tf.tp));
    }
    return ret;
    }
// Register a trace_probe and probe_event, and check the fprobe is available.
#[no_mangle]
unsafe extern "C" fn register_trace_fprobe_event(tf: *mut trace_fprobe) -> c_int {
pub static mut old_tf: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    guard(mutex)(&event_mutex);
    old_tf = find_trace_fprobe(trace_probe_name(&tf.tp),
    trace_probe_group_name(&tf.tp));
    if (old_tf) {
    return append_trace_fprobe_event(tf, old_tf);
    }
// Register new event
    ret = register_fprobe_event(tf);
    if (ret) {
    if (ret == -EEXIST) {
    trace_probe_log_set_index(0);
    trace_probe_log_err(0, EVENT_EXIST);
    } else {
    pr_warn!("Failed to register probe event(%d)\n", ret);
    }
    return ret;
    }
// Verify fprobe is sane.
    ret = trace_fprobe_verify_target(tf);
    if (ret < 0) {
    unregister_fprobe_event(tf);
    }
    else {
    dyn_event_add(&tf.devent, trace_probe_event_call(&tf.tp));
    }
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __find_tracepoint_cb_data {
    pub tp_name: *const c_char,
    pub tpoint: *mut tracepoint,
    pub mod: *mut module,
}

#[no_mangle]
unsafe extern "C" fn __find_tracepoint_module_cb(tp: *mut tracepoint, mod: *mut module, priv: *mut c_void) {
    let mut data = priv;
    if (!data.tpoint && !strcmp(data.tp_name, tp.name)) {
// If module is not specified, try getting module refcount.
    if (!data.mod && mod) {
// If failed to get refcount, ignore this tracepoint.
    if (!try_module_get(mod)) {
    return;
    }
    data.mod = mod;
    }
    data.tpoint = tp;
    }
    }
#[no_mangle]
unsafe extern "C" fn __find_tracepoint_cb(tp: *mut tracepoint, priv: *mut c_void) {
    let mut data = priv;
    if (!data.tpoint && !strcmp(data.tp_name, tp.name)) {
    data.tpoint = tp;
    }
    }
//
// Find a tracepoint from kernel and module. If the tracepoint is on the module,
// the module's refcount is incremented and returned as *@tp_mod. Thus, if it is
// not NULL, caller must call module_put!(*tp_mod) after used the tracepoint.
//
#[no_mangle]
pub unsafe extern "C" fn find_tracepoint(tp_name: *mut c_char, tp_mod: *mut *mut module) -> *mut c_void {
pub static mut __find_tracepoint_cb_data: usize = 0;
    for_each_kernel_tracepoint(__find_tracepoint_cb, &data); {
    if (!data.tpoint && IS_ENABLED!(CONFIG_MODULES)) {
    }
    for_each_module_tracepoint(__find_tracepoint_module_cb, &data); {
// tp_mod = data.mod;
    }
    }
    return data.tpoint;
    }

//
// Find a tracepoint from specified module. In this case, this does not get the
// module's refcount. The caller must ensure the module is not freed.
//
#[no_mangle]
pub unsafe extern "C" fn find_tracepoint_in_module(mod: *mut module, tp_name: *mut c_char) -> *mut c_void {
pub static mut __find_tracepoint_cb_data: usize = 0;
    for_each_tracepoint_in_module(mod, __find_tracepoint_module_cb, &data); {
    return data.tpoint;
    }
    }
// These are CONFIG_MODULES=y specific functions.
#[no_mangle]
pub unsafe extern "C" fn tracepoint_user_within_module(tuser: *mut tracepoint_user, mod: *mut module) -> bool {
    return within_module(tracepoint_user_ip(tuser), mod);
    }
#[no_mangle]
pub unsafe extern "C" fn tracepoint_user_register_again(tuser: *mut tracepoint_user, tpoint: *mut tracepoint) -> c_int {
    tuser.tpoint = tpoint;
    return tracepoint_user_register(tuser);
    }
#[no_mangle]
unsafe extern "C" fn tracepoint_user_unregister_clear(tuser: *mut tracepoint_user) {
    tracepoint_user_unregister(tuser);
    tuser.tpoint = core::ptr::null_mut();
    }
// module callback for tracepoint_user
#[no_mangle]
pub unsafe extern "C" fn __tracepoint_probe_module_cb(self: *mut notifier_block, val: c_ulong, data: *mut c_void) -> c_int {
    let mut tp_mod = data;
pub static mut tuser: *mut c_void = core::ptr::null_mut();
pub static mut tpoint: *mut c_void = core::ptr::null_mut();
    if (val != MODULE_STATE_GOING && val != MODULE_STATE_COMING) {
    return NOTIFY_DONE;
    }
    mutex_lock(&tracepoint_user_mutex);
    for_each_tracepoint_user(tuser) {
    if (val == MODULE_STATE_COMING) {
// This is not a tracepoint in this module. Skip it.
    tpoint = find_tracepoint_in_module(tp_mod.mod, tuser.name);
    if (!tpoint) {
    continue;
    }
    WARN_ON_ONCE!(tracepoint_user_register_again(tuser, tpoint));
    } else if (val == MODULE_STATE_GOING &&
    tracepoint_user_within_module(tuser, tp_mod.mod)) {
// Unregister all tracepoint_user in this module.
    tracepoint_user_unregister_clear(tuser);
    }
    }
    mutex_unlock(&tracepoint_user_mutex);
    return NOTIFY_DONE;
    }
pub static mut notifier_block: usize = 0;
// module callback for tprobe events
#[no_mangle]
pub unsafe extern "C" fn __tprobe_event_module_cb(self: *mut notifier_block, val: c_ulong, data: *mut c_void) -> c_int {
pub static mut tf: *mut c_void = core::ptr::null_mut();
pub static mut pos: *mut c_void = core::ptr::null_mut();
    let mut mod = data;
    if (val != MODULE_STATE_GOING && val != MODULE_STATE_COMING) {
    return NOTIFY_DONE;
    }
    mutex_lock(&event_mutex);
    for_each_trace_fprobe(tf, pos) {
// Skip fprobe and disabled tprobe events.
    if (!trace_fprobe_is_tracepoint(tf) || !tf.tuser) {
    continue;
    }
// Before this notification, tracepoint notifier has already done.
    if (val == MODULE_STATE_COMING &&
    tracepoint_user_within_module(tf.tuser, mod)) {
pub static mut ip: c_ulong = 0;
    WARN_ON_ONCE!(register_fprobe_ips(&tf.fp, &ip, 1));
    } else if (val == MODULE_STATE_GOING &&
//
// tracepoint_user_within_module() does not work here because
// tracepoint_user is already unregistered and cleared tpoint.
// Instead, checking whether the fprobe is registered but
// tpoint is cleared(unregistered). Such unbalance probes
// must be adjusted anyway.
//
    trace_fprobe_is_registered(tf) &&
    !tf.tuser.tpoint) {
    unregister_fprobe(&tf.fp);
    }
    }
    mutex_unlock(&event_mutex);
    return NOTIFY_DONE;
    }
// NOTE: this must be called after tracepoint callback
pub static mut notifier_block: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn parse_symbol_and_return(argc: c_int, symbol: *mut *mut c_char, is_return: *mut bool, is_tracepoint: bool) -> c_int {
    let mut tmp = strchr(argv[1], '%');
    let mut i = 0;
    if (tmp) {
pub static mut len: c_int = 0;
    if (!is_tracepoint && !strcmp(tmp, "%return")) {
// is_return = true;
    } else {
    trace_probe_log_err(len, BAD_ADDR_SUFFIX);
    return -EINVAL;
    }
// symbol = kmemdup_nul(argv[1], len, GFP_KERNEL);
    } else {
// symbol = kstrdup(argv[1], GFP_KERNEL);
    }
    if (!*symbol) {
    return -ENOMEM;
    }
    if (*is_return) {
    return 0;
    }
    if (is_tracepoint) {
    tmp = *symbol;
    while (*tmp && (isalnum(*tmp) || *tmp == '_')) {
    tmp += 1;
    }
    if (*tmp) {
// find a wrong character.
    trace_probe_log_err(tmp - *symbol, BAD_TP_NAME);
    kfree(*symbol);
// symbol = NULL;
    return -EINVAL;
    }
    }
// If there is $retval, this should be a return fprobe.
    while (i < argc) {
    tmp = strstr(argv[i], "$retval");
    if (tmp && !isalnum(tmp[7]) && tmp[7] != '_') {
    if (is_tracepoint) {
    trace_probe_log_set_index(i);
    trace_probe_log_err(tmp - argv[i], RETVAL_ON_PROBE);
    kfree(*symbol);
// symbol = NULL;
    return -EINVAL;
    }
// is_return = true;
    break;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_fprobe_create_internal(argc: c_int, ctx: *mut traceprobe_parse_context) -> c_int {
//
// Argument syntax:
// - Add fentry probe:
// f[:[GRP/][EVENT]] [MOD:]KSYM [FETCHARGS]
// - Add fexit probe:
// f[N][:[GRP/][EVENT]] [MOD:]KSYM%return [FETCHARGS]
// - Add tracepoint probe:
// t[:[GRP/][EVENT]] TRACEPOINT [FETCHARGS]
//
// Fetch args:
// $retval	: fetch return value
// $stack	: fetch stack address
// $stackN	: fetch Nth entry of stack (N:0-)
// $argN	: fetch Nth argument (N:1-)
// $comm       : fetch current task comm
// @ADDR	: fetch memory at ADDR (ADDR should be in kernel)
// @SYM[+|-offs] : fetch memory at SYM +|- offs (SYM is a data symbol)
// Dereferencing memory fetch:
// +|-offs(ARG) : fetch memory at ARG +|- offs address.
// Alias name of args:
// NAME=FETCHARG : set NAME as alias of FETCHARG.
// Type of args:
// FETCHARG:TYPE : use TYPE instead of unsigned long.
//
    struct trace_fprobe *tf __free(free_trace_fprobe) = core::ptr::null_mut();
    let mut event = core::ptr::null_mut(), *group = FPROBE_EVENT_SYSTEM;
    struct module *mod __free(module_put) = core::ptr::null_mut();
    const char **new_argv __free(kfree) = core::ptr::null_mut();
    char *symbol __free(kfree) = core::ptr::null_mut();
    char *ebuf __free(kfree) = core::ptr::null_mut();
    char *gbuf __free(kfree) = core::ptr::null_mut();
    char *sbuf __free(kfree) = core::ptr::null_mut();
    char *abuf __free(kfree) = core::ptr::null_mut();
    char *dbuf __free(kfree) = core::ptr::null_mut();
    int i, new_argc = 0, ret = 0;
pub static mut is_tracepoint: bool = false;
pub static mut is_return: bool = false;
    if ((argv[0][0] != 'f' && argv[0][0] != 't') || argc < 2) {
    return -ECANCELED;
    }
    if (argv[0][0] == 't') {
    is_tracepoint = true;
    group = TRACEPOINT_EVENT_SYSTEM;
    }
    if (argv[0][1] != '\0') {
    if (argv[0][1] != ':') {
    trace_probe_log_set_index(0);
    trace_probe_log_err(1, BAD_MAXACT);
    return -EINVAL;
    }
    event = &argv[0][2];
    }
    trace_probe_log_set_index(1);
// a symbol(or tracepoint) must be specified
    ret = parse_symbol_and_return(argc, argv, &symbol, &is_return, is_tracepoint);
    if (ret < 0) {
    return -EINVAL;
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
    return -EINVAL;
    }
    }
    if (!event) {
    ebuf = kmalloc(MAX_EVENT_NAME_LEN, GFP_KERNEL);
    if (!ebuf) {
    return -ENOMEM;
    }
// Make a new event name
    if (is_tracepoint) {
    snprintf(ebuf, MAX_EVENT_NAME_LEN, "%s%s",
    isdigit(*symbol) ? "_" : "", symbol);
    }
    else {
    snprintf(ebuf, MAX_EVENT_NAME_LEN, "%s__%s", symbol,
    is_return ? "exit" : "entry");
    }
    sanitize_event_name(ebuf);
    event = ebuf;
    }
    if (is_return) {
    ctx.flags |= TPARG_FL_RETURN;
    }
    else {
    ctx.flags |= TPARG_FL_FENTRY;
    }
    ctx.funcname = core::ptr::null_mut();
    if (is_tracepoint) {
// Get tracepoint and lock its module until the end of the registration.
pub static mut tpoint: *mut c_void = core::ptr::null_mut();
    ctx.flags |= TPARG_FL_TPOINT;
    mod = core::ptr::null_mut();
    tpoint = find_tracepoint(symbol, &mod);
    if (tpoint) {
    sbuf = kmalloc(KSYM_NAME_LEN, GFP_KERNEL);
    if (!sbuf) {
    return -ENOMEM;
    }
    ctx.funcname = kallsyms_lookup((unsigned long)tpoint.probestub,
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), sbuf);
    }
    }
    if (!ctx.funcname) {
    ctx.funcname = symbol;
    }
    abuf = kmalloc(MAX_BTF_ARGS_LEN, GFP_KERNEL);
    if (!abuf) {
    return -ENOMEM;
    }
    argc -= 2; argv += 2;
    new_argv = traceprobe_expand_meta_args(argc, argv, &new_argc,
    abuf, MAX_BTF_ARGS_LEN, ctx);
    if (IS_ERR(new_argv)) {
    return PTR_ERR(new_argv);
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
    tf = alloc_trace_fprobe(group, event, symbol, argc, is_return, is_tracepoint);
    if (IS_ERR(tf)) {
    ret = PTR_ERR(tf);
// This must return -ENOMEM, else there is a bug
    WARN_ON_ONCE!(ret != -ENOMEM);
    return ret;
    }
// parse arguments
    while (i < argc) {
    trace_probe_log_set_index(i + 2);
    ctx.offset = 0;
    ret = traceprobe_parse_probe_arg(&tf.tp, i, argv[i], ctx);
    if (ret) {
    return ret;	/* This can be -ENOMEM */
    }
    }
    if (is_return && tf.tp.entry_arg) {
    tf.fp.entry_handler = trace_fprobe_entry_handler;
    tf.fp.entry_data_size = traceprobe_get_entry_data_size(&tf.tp);
    if (ALIGN(tf.fp.entry_data_size, sizeof!(long)) > MAX_FPROBE_DATA_SIZE) {
    trace_probe_log_set_index(2);
    trace_probe_log_err(0, TOO_MANY_EARGS);
    return -E2BIG;
    }
    }
    ret = traceprobe_set_print_fmt(&tf.tp,
    is_return ? PROBE_PRINT_RETURN : PROBE_PRINT_NORMAL);
    if (ret < 0) {
    return ret;
    }
    ret = register_trace_fprobe_event(tf);
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
    return -EINVAL;
    }
// 'tf' is successfully registered. To avoid freeing, assign NULL.
    tf = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_fprobe_create_cb(argc: c_int, argv[]: *const c_char) -> c_int {
    struct traceprobe_parse_context *ctx __free(traceprobe_parse_context) = core::ptr::null_mut();
    let mut ret = 0;
    ctx = kzalloc_obj(*ctx);
    if (!ctx) {
    return -ENOMEM;
    }
    ctx.flags = TPARG_FL_KERNEL | TPARG_FL_FPROBE;
    trace_probe_log_init("trace_fprobe", argc, argv);
    ret = trace_fprobe_create_internal(argc, argv, ctx);
    trace_probe_log_clear();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn trace_fprobe_create(raw_command: *const c_char) -> c_int {
    return trace_probe_create(raw_command, trace_fprobe_create_cb);
    }
#[no_mangle]
unsafe extern "C" fn trace_fprobe_release(ev: *mut dyn_event) -> c_int {
    let mut tf = to_trace_fprobe(ev);
pub static mut ret: c_int = 0;
    if (!ret) {
    free_trace_fprobe(tf);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn trace_fprobe_show(m: *mut seq_file, ev: *mut dyn_event) -> c_int {
    let mut tf = to_trace_fprobe(ev);
    let mut i = 0;
    if (trace_fprobe_is_tracepoint(tf)) {
    seq_putc(m, 't');
    }
    else {
    seq_putc(m, 'f');
    }
    seq_printf(m, ":%s/%s", trace_probe_group_name(&tf.tp),
    trace_probe_name(&tf.tp));
    seq_printf(m, " %s%s", trace_fprobe_symbol(tf),
    trace_fprobe_is_return(tf) ? "%return" : "");
    for (i = 0; i < tf.tp.nr_args; i++) {
    seq_printf(m, " %s=%s", tf.tp.args[i].name, tf.tp.args[i].comm);
    }
    seq_putc(m, '\n');
    trace_probe_dump_args(m, &tf.tp);
    return 0;
    }
//
// Enable trace_probe
// if the file is NULL, enable "perf" handler, or enable "trace" handler.
//
#[no_mangle]
pub unsafe extern "C" fn enable_trace_fprobe(call: *mut trace_event_call, file: *mut trace_event_file) -> c_int {
pub static mut tp: *mut c_void = core::ptr::null_mut();
pub static mut tf: *mut c_void = core::ptr::null_mut();
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
    if (!enabled) {
    list_for_each_entry(tf, trace_probe_probe_list(tp), tp.list) {
    ret = __register_trace_fprobe(tf);
    if (ret < 0) {
// goto;
    }
    }
    }
    return 0;
// label;
// Failed to enable one of them. Roll back all
    list_for_each_entry(tf, trace_probe_probe_list(tp), tp.list) {
    __unregister_trace_fprobe(tf);
    }
    if (file) {
    trace_probe_remove_file(tp, file);
    }
    else {
    trace_probe_clear_flag(tp, TP_FLAG_PROFILE);
    }
    return ret;
    }
//
// Disable trace_probe
// if the file is NULL, disable "perf" handler, or disable "trace" handler.
//
#[no_mangle]
pub unsafe extern "C" fn disable_trace_fprobe(call: *mut trace_event_call, file: *mut trace_event_file) -> c_int {
pub static mut tf: *mut c_void = core::ptr::null_mut();
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
    list_for_each_entry(tf, trace_probe_probe_list(tp), tp.list) {
    unregister_fprobe(&tf.fp);
    if (tf.tuser) {
    tracepoint_user_put(tf.tuser);
    tf.tuser = core::ptr::null_mut();
    }
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
//
// called by perf_trace_init() or __ftrace_set_clr_event() under event_mutex.
//
#[no_mangle]
pub unsafe extern "C" fn fprobe_register(event: *mut trace_event_call, type: trace_reg, data: *mut c_void) -> c_int {
    let mut file = data;
    match (type) {
    TRACE_REG_REGISTER => {
    return enable_trace_fprobe(event, file);
    }
    TRACE_REG_UNREGISTER => {
    return disable_trace_fprobe(event, file);

    }
    TRACE_REG_PERF_REGISTER => {
    return enable_trace_fprobe(event, core::ptr::null_mut());
    }
    TRACE_REG_PERF_UNREGISTER => {
    return disable_trace_fprobe(event, core::ptr::null_mut());
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
//
// Register dynevent at core_initcall. This allows kernel to setup fprobe
// events in postcore_initcall without tracefs.
//
#[no_mangle]
unsafe extern "C" fn init_fprobe_trace_early() -> __init int {
    let mut ret = 0;
    ret = dyn_event_register(&trace_fprobe_ops);
    if (ret) {
    return ret;
    }

    ret = register_tracepoint_module_notifier(&tracepoint_module_nb);
    if (ret) {
    return ret;
    }
    ret = register_module_notifier(&tprobe_event_module_nb);
    if (ret) {
    return ret;
    }

    return 0;
    }
    core_initcall!(init_fprobe_trace_early);
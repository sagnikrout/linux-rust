//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_functions.c
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
// ring buffer based function tracer
//
// Copyright (C) 2007-2008 Steven Rostedt <srostedt@redhat.com>
// Copyright (C) 2008 Ingo Molnar <mingo@redhat.com>
//
// Based on code from the latency_tracer, that is:
//
// Copyright (C) 2004-2006 Ingo Molnar
// Copyright (C) 2004 Nadia Yvette Chambers
//

// forward_decl: tracing_start_function_trace;
// forward_decl: tracing_stop_function_trace;
// forward_decl: function_trace_call;
// forward_decl: function_args_trace_call;
// forward_decl: function_stack_trace_call;
// forward_decl: function_no_repeats_trace_call;
// forward_decl: function_stack_no_repeats_trace_call;
pub static mut func_flags: usize = 0;
// Our option
    enum {
    TRACE_FUNC_NO_OPTS		= 0x0, /* No flags set. */
    TRACE_FUNC_OPT_STACK		= 0x1,
    TRACE_FUNC_OPT_NO_REPEATS	= 0x2,
    TRACE_FUNC_OPT_ARGS		= 0x4,
// Update this to next highest bit.
    TRACE_FUNC_OPT_HIGHEST_BIT	= 0x8
    };

#[no_mangle]
pub unsafe extern "C" fn ftrace_allocate_ftrace_ops(tr: *mut trace_array) -> c_int {
pub static mut ops: *mut c_void = core::ptr::null_mut();
// The top level array uses the "global_ops"
    if (tr.flags & TRACE_ARRAY_FL_GLOBAL) {
    return 0;
    }
    ops = kzalloc_obj(*ops);
    if (!ops) {
    return -ENOMEM;
    }
// Currently only the non stack version is supported
    ops.func = function_trace_call;
    ops.flags = FTRACE_OPS_FL_PID;
    tr.ops = ops;
    ops.private = tr;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_free_ftrace_ops(tr: *mut trace_array) {
    kfree(tr.ops);
    tr.ops = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_create_function_files(tr: *mut trace_array, parent: *mut dentry) -> c_int {
    let mut ret = 0;
//
// The top level array uses the "global_ops", and the files are
// created on boot up.
//
    if (tr.flags & TRACE_ARRAY_FL_GLOBAL) {
    return 0;
    }
    if (!tr.ops) {
    return -EINVAL;
    }
    ret = allocate_fgraph_ops(tr, tr.ops);
    if (ret) {
    kfree(tr.ops);
    return ret;
    }
    ftrace_create_filter_files(tr.ops, parent);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_destroy_function_files(tr: *mut trace_array) {
    ftrace_destroy_filter_files(tr.ops);
    ftrace_free_ftrace_ops(tr);
    free_fgraph_ops(tr);
    }
#[no_mangle]
unsafe extern "C" fn select_trace_function(flags_val: u32) -> ftrace_func_t {
    match (flags_val & TRACE_FUNC_OPT_MASK) {
    TRACE_FUNC_NO_OPTS => {
    return function_trace_call;
    }
    TRACE_FUNC_OPT_ARGS => {
    return function_args_trace_call;
    }
    TRACE_FUNC_OPT_STACK => {
    return function_stack_trace_call;
    }
    TRACE_FUNC_OPT_NO_REPEATS => {
    return function_no_repeats_trace_call;
    }
    TRACE_FUNC_OPT_STACK | TRACE_FUNC_OPT_NO_REPEATS => {
    return function_stack_no_repeats_trace_call;
    }
    _ => {
    return core::ptr::null_mut();
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn handle_func_repeats(tr: *mut trace_array, flags_val: u32) -> bool {
    if (!tr.last_func_repeats &&
    (flags_val & TRACE_FUNC_OPT_NO_REPEATS)) {
    tr.last_func_repeats = alloc_percpu(trace_func_repeats);
    if (!tr.last_func_repeats) {
    return false;
    }
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn function_trace_init(tr: *mut trace_array) -> c_int {
    let mut func;
//
// Instance trace_arrays get their ops allocated
// at instance creation. Unless it failed
// the allocation.
//
    if (!tr.ops) {
    return -ENOMEM;
    }
    func = select_trace_function(tr.current_trace_flags.val);
    if (!func) {
    return -EINVAL;
    }
    if (!handle_func_repeats(tr, tr.current_trace_flags.val)) {
    return -ENOMEM;
    }
    ftrace_init_array_ops(tr, func);
    tr.array_buffer.cpu = raw_smp_processor_id();
    tracing_start_cmdline_record();
    tracing_start_function_trace(tr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn function_trace_reset(tr: *mut trace_array) {
    tracing_stop_function_trace(tr);
    tracing_stop_cmdline_record();
    ftrace_reset_array_ops(tr);
    }
#[no_mangle]
unsafe extern "C" fn function_trace_start(tr: *mut trace_array) {
    tracing_reset_online_cpus(&tr.array_buffer);
    }
// fregs are guaranteed not to be NULL if HAVE_DYNAMIC_FTRACE_WITH_ARGS is set

    static __always_inline unsigned long
    function_get_true_parent_ip(unsigned long parent_ip, ftrace_regs *fregs)
    {
    let mut true_parent_ip = 0;
pub static mut idx: c_int = 0;
    true_parent_ip = parent_ip;
    if (unlikely(parent_ip == (unsigned long)&return_to_handler) && fregs) {
    true_parent_ip = ftrace_graph_ret_addr(current, &idx, parent_ip,
    ftrace_regs_get_stack_pointer(fregs));
    }
    return true_parent_ip;
    }

    static __always_inline unsigned long
    function_get_true_parent_ip(unsigned long parent_ip, ftrace_regs *fregs)
    {
    return parent_ip;
    }

#[no_mangle]
pub unsafe extern "C" fn function_trace_call(ip: c_ulong, parent_ip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    let mut tr = op.private;
    let mut trace_ctx = 0;
    let mut bit = 0;
    if (unlikely(!tr.function_enabled)) {
    return;
    }
    bit = ftrace_test_recursion_trylock(ip, parent_ip);
    if (bit < 0) {
    return;
    }
    parent_ip = function_get_true_parent_ip(parent_ip, fregs);
    trace_ctx = tracing_gen_ctx_dec();
    trace_function(tr, ip, parent_ip, trace_ctx, core::ptr::null_mut());
    ftrace_test_recursion_unlock(bit);
    }
#[no_mangle]
pub unsafe extern "C" fn function_args_trace_call(ip: c_ulong, parent_ip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    let mut tr = op.private;
    let mut trace_ctx = 0;
    let mut bit = 0;
    if (unlikely(!tr.function_enabled)) {
    return;
    }
    bit = ftrace_test_recursion_trylock(ip, parent_ip);
    if (bit < 0) {
    return;
    }
    trace_ctx = tracing_gen_ctx();
    trace_function(tr, ip, parent_ip, trace_ctx, fregs);
    ftrace_test_recursion_unlock(bit);
    }

//
// Skip 2:
//
// function_stack_trace_call()
// ftrace_call()
//
pub const STACK_SKIP: c_int = 2;

//
// Skip 3:
// __trace_stack()
// function_stack_trace_call()
// ftrace_call()
//
pub const STACK_SKIP: c_int = 3;

#[no_mangle]
pub unsafe extern "C" fn function_stack_trace_call(ip: c_ulong, parent_ip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    let mut tr = op.private;
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut disabled = 0;
    let mut cpu = 0;
    let mut trace_ctx = 0;
pub static mut skip: c_int = 0;
    if (unlikely(!tr.function_enabled)) {
    return;
    }
//
// Need to use raw, since this must be called before the
// recursive protection is performed.
//
    local_irq_save(flags);
    parent_ip = function_get_true_parent_ip(parent_ip, fregs);
    cpu = raw_smp_processor_id();
    data = per_cpu_ptr(tr.array_buffer.data, cpu);
    disabled = local_inc_return(&data.disabled);
    if (likely(disabled == 1)) {
    trace_ctx = tracing_gen_ctx_flags(flags);
    trace_function(tr, ip, parent_ip, trace_ctx, core::ptr::null_mut());

    if (ftrace_pids_enabled(op)) {
    skip += 1;
    }

    __trace_stack(tr, trace_ctx, skip);
    }
    local_dec(&data.disabled);
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn is_repeat_check(tr: *mut trace_array, last_info: *mut trace_func_repeats, ip: c_ulong, parent_ip: c_ulong) -> bool {
    if (last_info.ip == ip &&
    last_info.parent_ip == parent_ip &&
    last_info.count < U16_MAX) {
    last_info.ts_last_call =
    ring_buffer_time_stamp(tr.array_buffer.buffer);
    last_info.count += 1;
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn process_repeats(tr: *mut trace_array, ip: c_ulong, parent_ip: c_ulong, last_info: *mut trace_func_repeats, trace_ctx: c_uint) {
    if (last_info.count) {
    trace_last_func_repeats(tr, last_info, trace_ctx);
    last_info.count = 0;
    }
    last_info.ip = ip;
    last_info.parent_ip = parent_ip;
    }
#[no_mangle]
pub unsafe extern "C" fn function_no_repeats_trace_call(ip: c_ulong, parent_ip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
pub static mut last_info: *mut c_void = core::ptr::null_mut();
    let mut tr = op.private;
    let mut trace_ctx = 0;
    let mut bit = 0;
    if (unlikely(!tr.function_enabled)) {
    return;
    }
    bit = ftrace_test_recursion_trylock(ip, parent_ip);
    if (bit < 0) {
    return;
    }
    parent_ip = function_get_true_parent_ip(parent_ip, fregs);
    if (!tracer_tracing_is_on(tr)) {
// goto;
    }
//
// An interrupt may happen at any place here. But as far as I can see,
// the only damage that this can cause is to mess up the repetition
// counter without valuable data being lost.
// TODO: think about a solution that is better than just hoping to be
// lucky.
//
    last_info = this_cpu_ptr(tr.last_func_repeats);
    if (is_repeat_check(tr, last_info, ip, parent_ip)) {
// goto;
    }
    trace_ctx = tracing_gen_ctx_dec();
    process_repeats(tr, ip, parent_ip, last_info, trace_ctx);
    trace_function(tr, ip, parent_ip, trace_ctx, core::ptr::null_mut());
// label;
    ftrace_test_recursion_unlock(bit);
    }
#[no_mangle]
pub unsafe extern "C" fn function_stack_no_repeats_trace_call(ip: c_ulong, parent_ip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
pub static mut last_info: *mut c_void = core::ptr::null_mut();
    let mut tr = op.private;
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut disabled = 0;
    let mut cpu = 0;
    let mut trace_ctx = 0;
    if (unlikely(!tr.function_enabled)) {
    return;
    }
//
// Need to use raw, since this must be called before the
// recursive protection is performed.
//
    local_irq_save(flags);
    parent_ip = function_get_true_parent_ip(parent_ip, fregs);
    cpu = raw_smp_processor_id();
    data = per_cpu_ptr(tr.array_buffer.data, cpu);
    disabled = local_inc_return(&data.disabled);
    if (likely(disabled == 1)) {
    last_info = per_cpu_ptr(tr.last_func_repeats, cpu);
    if (is_repeat_check(tr, last_info, ip, parent_ip)) {
// goto;
    }
    trace_ctx = tracing_gen_ctx_flags(flags);
    process_repeats(tr, ip, parent_ip, last_info, trace_ctx);
    trace_function(tr, ip, parent_ip, trace_ctx, core::ptr::null_mut());
    __trace_stack(tr, trace_ctx, STACK_SKIP);
    }
// label;
    local_dec(&data.disabled);
    local_irq_restore(flags);
    }
pub static mut tracer_opt: usize = 0;
pub static mut tracer_flags: usize = 0;
#[no_mangle]
unsafe extern "C" fn tracing_start_function_trace(tr: *mut trace_array) {
    tr.function_enabled = 0;
    register_ftrace_function(tr.ops);
    tr.function_enabled = 1;
    }
#[no_mangle]
unsafe extern "C" fn tracing_stop_function_trace(tr: *mut trace_array) {
    tr.function_enabled = 0;
    unregister_ftrace_function(tr.ops);
    }
pub static mut function_trace: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn func_set_flag(tr: *mut trace_array, old_flags: u32, bit: u32, set: c_int) -> c_int {
    let mut func;
    let mut new_flags = 0;
// We can change this flag only when current tracer is function.
    if (tr.current_trace != &function_trace) {
    return 0;
    }
// Do nothing if already set.
    if (!!set == !!(tr.current_trace_flags.val & bit)) {
    return 0;
    }
    new_flags = (tr.current_trace_flags.val & ~bit) | (set ? bit : 0);
    func = select_trace_function(new_flags);
    if (!func) {
    return -EINVAL;
    }
// Check if there's anything to change.
    if (tr.ops.func == func) {
    return 0;
    }
    if (!handle_func_repeats(tr, new_flags)) {
    return -ENOMEM;
    }
    unregister_ftrace_function(tr.ops);
    tr.ops.func = func;
    register_ftrace_function(tr.ops);
    return 0;
    }
    static struct tracer function_trace __tracer_data =
    {
    .name		= "function",
    .init		= function_trace_init,
    .reset		= function_trace_reset,
    .start		= function_trace_start,
    .default_flags	= &func_flags,
    .set_flag	= func_set_flag,
    .allow_instances = true,

    .selftest	= trace_selftest_startup_function,

    };

#[no_mangle]
pub unsafe extern "C" fn update_traceon_count(ops: *mut ftrace_probe_ops, ip: c_ulong, tr: *mut trace_array, on: bool, data: *mut c_void) {
    let mut mapper = data;
pub static mut count: *mut c_void = core::ptr::null_mut();
    let mut old_count = 0;
//
// Tracing gets disabled (or enabled) once per count.
// This function can be called at the same time on multiple CPUs.
// It is fine if both disable (or enable) tracing, as disabling
// (or enabling) the second time doesn't do anything as the
// state of the tracer is already disabled (or enabled).
// What needs to be synchronized in this case is that the count
// only gets decremented once, even if the tracer is disabled
// (or enabled) twice, as the second one is really a nop.
//
// The memory barriers guarantee that we only decrement the
// counter once. First the count is read to a local variable
// and a read barrier is used to make sure that it is loaded
// before checking if the tracer is in the state we want.
// If the tracer is not in the state we want, then the count
// is guaranteed to be the old count.
//
// Next the tracer is set to the state we want (disabled or enabled)
// then a write memory barrier is used to make sure that
// the new state is visible before changing the counter by
// one minus the old counter. This guarantees that another CPU
// executing this code will see the new state before seeing
// the new counter value, and would not do anything if the new
// counter is seen.
//
// Note, there is no synchronization between this and a user
// setting the tracing_on file. But we currently don't care
// about that.
//
    count = ftrace_func_mapper_find_ip(mapper, ip);
    old_count = *count;
    if (old_count <= 0) {
    return;
    }
// Make sure we see count before checking tracing state
    smp_rmb();
    if (on == !!tracer_tracing_is_on(tr)) {
    return;
    }
    if (on) {
    tracer_tracing_on(tr);
    }
    else {
    tracer_tracing_off(tr);
    }
// Make sure tracing state is visible before updating count
    smp_wmb();
// count = old_count - 1;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_traceon_count(ip: c_ulong, parent_ip: c_ulong, tr: *mut trace_array, ops: *mut ftrace_probe_ops, data: *mut c_void) {
    update_traceon_count(ops, ip, tr, 1, data);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_traceoff_count(ip: c_ulong, parent_ip: c_ulong, tr: *mut trace_array, ops: *mut ftrace_probe_ops, data: *mut c_void) {
    update_traceon_count(ops, ip, tr, 0, data);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_traceon(ip: c_ulong, parent_ip: c_ulong, tr: *mut trace_array, ops: *mut ftrace_probe_ops, data: *mut c_void) {
    if (tracer_tracing_is_on(tr)) {
    return;
    }
    tracer_tracing_on(tr);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_traceoff(ip: c_ulong, parent_ip: c_ulong, tr: *mut trace_array, ops: *mut ftrace_probe_ops, data: *mut c_void) {
    if (!tracer_tracing_is_on(tr)) {
    return;
    }
    tracer_tracing_off(tr);
    }

//
// Skip 3:
//
// function_trace_probe_call()
// ftrace_ops_assist_func()
// ftrace_call()
//
pub const FTRACE_STACK_SKIP: c_int = 3;

//
// Skip 5:
//
// __trace_stack()
// ftrace_stacktrace()
// function_trace_probe_call()
// ftrace_ops_assist_func()
// ftrace_call()
//
pub const FTRACE_STACK_SKIP: c_int = 5;

#[no_mangle]
unsafe extern "C" fn trace_stack(tr: *mut trace_array) -> __always_inline void {
    __trace_stack(tr, tracing_gen_ctx_dec(), FTRACE_STACK_SKIP);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_stacktrace(ip: c_ulong, parent_ip: c_ulong, tr: *mut trace_array, ops: *mut ftrace_probe_ops, data: *mut c_void) {
    trace_stack(tr);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_stacktrace_count(ip: c_ulong, parent_ip: c_ulong, tr: *mut trace_array, ops: *mut ftrace_probe_ops, data: *mut c_void) {
    let mut mapper = data;
pub static mut count: *mut c_void = core::ptr::null_mut();
    let mut old_count = 0;
    let mut new_count = 0;
    if (!tracing_is_on()) {
    return;
    }
// unlimited?
    if (!mapper) {
    trace_stack(tr);
    return;
    }
    count = ftrace_func_mapper_find_ip(mapper, ip);
//
// Stack traces should only execute the number of times the
// user specified in the counter.
//
    do {
    old_count = *count;
    if (!old_count) {
    return;
    }
    new_count = old_count - 1;
    new_count = cmpxchg(count, old_count, new_count);
    if (new_count == old_count) {
    trace_stack(tr);
    }
    if (!tracing_is_on()) {
    return;
    }
    } while (new_count != old_count);
    }
#[no_mangle]
pub unsafe extern "C" fn update_count(ops: *mut ftrace_probe_ops, ip: c_ulong, data: *mut c_void) -> c_int {
    let mut mapper = data;
    let mut count = core::ptr::null_mut();
    if (mapper) {
    count = ftrace_func_mapper_find_ip(mapper, ip);
    }
    if (count) {
    if (*count <= 0) {
    return 0;
    }
    (*count)--;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_dump_probe(ip: c_ulong, parent_ip: c_ulong, tr: *mut trace_array, ops: *mut ftrace_probe_ops, data: *mut c_void) {
    if (update_count(ops, ip, data)) {
    ftrace_dump(DUMP_ALL);
    }
    }
// Only dump the current CPU buffer.
#[no_mangle]
pub unsafe extern "C" fn ftrace_cpudump_probe(ip: c_ulong, parent_ip: c_ulong, tr: *mut trace_array, ops: *mut ftrace_probe_ops, data: *mut c_void) {
    if (update_count(ops, ip, data)) {
    ftrace_dump(DUMP_ORIG);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_probe_print(name: *mut c_char, m: *mut seq_file, ip: c_ulong, ops: *mut ftrace_probe_ops, data: *mut c_void) -> c_int {
    let mut mapper = data;
    let mut count = core::ptr::null_mut();
    seq_printf(m, "%ps:%s", ip, name);
    if (mapper) {
    count = ftrace_func_mapper_find_ip(mapper, ip);
    }
    if (count) {
    seq_printf(m, ":count=%ld\n", *count);
    }
    else {
    seq_puts(m, ":unlimited\n");
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_traceon_print(m: *mut seq_file, ip: c_ulong, ops: *mut ftrace_probe_ops, data: *mut c_void) -> c_int {
    return ftrace_probe_print("traceon", m, ip, ops, data);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_traceoff_print(m: *mut seq_file, ip: c_ulong, ops: *mut ftrace_probe_ops, data: *mut c_void) -> c_int {
    return ftrace_probe_print("traceoff", m, ip, ops, data);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_stacktrace_print(m: *mut seq_file, ip: c_ulong, ops: *mut ftrace_probe_ops, data: *mut c_void) -> c_int {
    return ftrace_probe_print("stacktrace", m, ip, ops, data);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_dump_print(m: *mut seq_file, ip: c_ulong, ops: *mut ftrace_probe_ops, data: *mut c_void) -> c_int {
    return ftrace_probe_print("dump", m, ip, ops, data);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_cpudump_print(m: *mut seq_file, ip: c_ulong, ops: *mut ftrace_probe_ops, data: *mut c_void) -> c_int {
    return ftrace_probe_print("cpudump", m, ip, ops, data);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_count_init(ops: *mut ftrace_probe_ops, tr: *mut trace_array, ip: c_ulong, init_data: *mut c_void, data: *mut *mut c_void) -> c_int {
    let mut mapper = *data;
    if (!mapper) {
    mapper = allocate_ftrace_func_mapper();
    if (!mapper) {
    return -ENOMEM;
    }
// data = mapper;
    }
    return ftrace_func_mapper_add_ip(mapper, ip, init_data);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_count_free(ops: *mut ftrace_probe_ops, tr: *mut trace_array, ip: c_ulong, data: *mut c_void) {
    let mut mapper = data;
    if (!ip) {
    free_ftrace_func_mapper(mapper, core::ptr::null_mut());
    return;
    }
    ftrace_func_mapper_remove_ip(mapper, ip);
    }
pub static mut ftrace_probe_ops: usize = 0;
pub static mut ftrace_probe_ops: usize = 0;
pub static mut ftrace_probe_ops: usize = 0;
pub static mut ftrace_probe_ops: usize = 0;
pub static mut ftrace_probe_ops: usize = 0;
pub static mut ftrace_probe_ops: usize = 0;
pub static mut ftrace_probe_ops: usize = 0;
pub static mut ftrace_probe_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn ftrace_trace_probe_callback(tr: *mut trace_array, ops: *mut ftrace_probe_ops, hash: *mut ftrace_hash, glob: *mut c_char, cmd: *mut c_char, param: *mut c_char, enable: c_int) -> c_int {
    let mut count = -1;
pub static mut number: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
// hash funcs only work with set_ftrace_filter
    if (!enable) {
    return -EINVAL;
    }
    if (glob[0] == '!') {
    return unregister_ftrace_function_probe_func(glob+1, tr, ops);
    }
    if (!param) {
// goto;
    }
    number = strsep(&param, ":");
    if (!strlen(number)) {
// goto;
    }
//
// We use the callback data field (which is a pointer)
// as our counter.
//
    ret = kstrtoul(number, 0, &count);
    if (ret) {
    return ret;
    }
// label;
    ret = register_ftrace_function_probe(glob, tr, ops, count);
    return ret < 0 ? ret : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_trace_onoff_callback(tr: *mut trace_array, hash: *mut ftrace_hash, glob: *mut c_char, cmd: *mut c_char, param: *mut c_char, enable: c_int) -> c_int {
pub static mut ops: *mut c_void = core::ptr::null_mut();
    if (!tr) {
    return -ENODEV;
    }
// we register both traceon and traceoff to this callback
    if (strcmp(cmd, "traceon") == 0) {
    ops = param ? &traceon_count_probe_ops : &traceon_probe_ops;
    }
    else {
    ops = param ? &traceoff_count_probe_ops : &traceoff_probe_ops;
    }
    return ftrace_trace_probe_callback(tr, ops, hash, glob, cmd,
    param, enable);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_stacktrace_callback(tr: *mut trace_array, hash: *mut ftrace_hash, glob: *mut c_char, cmd: *mut c_char, param: *mut c_char, enable: c_int) -> c_int {
pub static mut ops: *mut c_void = core::ptr::null_mut();
    if (!tr) {
    return -ENODEV;
    }
    ops = param ? &stacktrace_count_probe_ops : &stacktrace_probe_ops;
    return ftrace_trace_probe_callback(tr, ops, hash, glob, cmd,
    param, enable);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_dump_callback(tr: *mut trace_array, hash: *mut ftrace_hash, glob: *mut c_char, cmd: *mut c_char, param: *mut c_char, enable: c_int) -> c_int {
pub static mut ops: *mut c_void = core::ptr::null_mut();
    if (!tr) {
    return -ENODEV;
    }
    ops = &dump_probe_ops;
// Only dump once.
    return ftrace_trace_probe_callback(tr, ops, hash, glob, cmd,
    "1", enable);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_cpudump_callback(tr: *mut trace_array, hash: *mut ftrace_hash, glob: *mut c_char, cmd: *mut c_char, param: *mut c_char, enable: c_int) -> c_int {
pub static mut ops: *mut c_void = core::ptr::null_mut();
    if (!tr) {
    return -ENODEV;
    }
    ops = &cpudump_probe_ops;
// Only dump once.
    return ftrace_trace_probe_callback(tr, ops, hash, glob, cmd,
    "1", enable);
    }
pub static mut ftrace_func_command: usize = 0;
pub static mut ftrace_func_command: usize = 0;
pub static mut ftrace_func_command: usize = 0;
pub static mut ftrace_func_command: usize = 0;
pub static mut ftrace_func_command: usize = 0;
#[no_mangle]
unsafe extern "C" fn init_func_cmd_traceon() -> c_int {
    let mut ret = 0;
    ret = register_ftrace_command(&ftrace_traceoff_cmd);
    if (ret) {
    return ret;
    }
    ret = register_ftrace_command(&ftrace_traceon_cmd);
    if (ret) {
// goto;
    }
    ret = register_ftrace_command(&ftrace_stacktrace_cmd);
    if (ret) {
// goto;
    }
    ret = register_ftrace_command(&ftrace_dump_cmd);
    if (ret) {
// goto;
    }
    ret = register_ftrace_command(&ftrace_cpudump_cmd);
    if (ret) {
// goto;
    }
    return 0;
// label;
    unregister_ftrace_command(&ftrace_dump_cmd);
// label;
    unregister_ftrace_command(&ftrace_stacktrace_cmd);
// label;
    unregister_ftrace_command(&ftrace_traceon_cmd);
// label;
    unregister_ftrace_command(&ftrace_traceoff_cmd);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn init_func_cmd_traceon() -> c_int {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn init_function_trace() -> __init int {
    init_func_cmd_traceon();
    return register_tracer(&function_trace);
    }
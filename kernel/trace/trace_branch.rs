//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_branch.c
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
// unlikely profiler
//
// Copyright (C) 2008 Steven Rostedt <srostedt@redhat.com>
//

pub static mut branch_trace: usize = 0;
    static int branch_tracing_enabled ;
pub static mut branch_tracing_mutex: usize = 0;
pub static mut branch_tracer: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn probe_likely_condition(f: *mut ftrace_likely_data, val: c_int, expect: c_int) {
    let mut tr = branch_tracer;
pub static mut buffer: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut trace_ctx = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (current.trace_recursion & TRACE_BRANCH_BIT) {
    return;
    }
//
// I would love to save just the ftrace_likely_data pointer, but
// this code can also be used by modules. Ugly things can happen
// if the module is unloaded, and then we go and read the
// pointer.  This is slower, but much safer.
//
    if (unlikely(!tr)) {
    return;
    }
    raw_local_irq_save(flags);
    current.trace_recursion |= TRACE_BRANCH_BIT;
    if (!tracer_tracing_is_on_cpu(tr, raw_smp_processor_id())) {
// goto;
    }
    trace_ctx = tracing_gen_ctx_flags(flags);
    buffer = tr.array_buffer.buffer;
    event = trace_buffer_lock_reserve(buffer, TRACE_BRANCH,
    sizeof!(*entry), trace_ctx);
    if (!event) {
// goto;
    }
    entry	= ring_buffer_event_data(event);
// Strip off the path, only save the file
    p = f.data.file + strlen(f.data.file);
    while (p >= f.data.file && *p != '/') {
    p -= 1;
    }
    p += 1;
    strscpy(entry.func, f.data.func);
    strscpy(entry.file, p);
    entry.constant = f.constant;
    entry.line = f.data.line;
    entry.correct = val == expect;
    trace_buffer_unlock_commit_nostack(buffer, event);
// label;
    current.trace_recursion &= ~TRACE_BRANCH_BIT;
    raw_local_irq_restore(flags);
    }
    static inline
#[no_mangle]
pub unsafe extern "C" fn trace_likely_condition(f: *mut ftrace_likely_data, val: c_int, expect: c_int) {
    if (!branch_tracing_enabled) {
    return;
    }
    probe_likely_condition(f, val, expect);
    }
#[no_mangle]
pub unsafe extern "C" fn enable_branch_tracing(tr: *mut trace_array) -> c_int {
    mutex_lock(&branch_tracing_mutex);
    branch_tracer = tr;
//
// Must be seen before enabling. The reader is a condition
// where we do not need a matching rmb()
//
    smp_wmb();
    branch_tracing_enabled += 1;
    mutex_unlock(&branch_tracing_mutex);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn disable_branch_tracing() {
    mutex_lock(&branch_tracing_mutex);
    if (!branch_tracing_enabled) {
// goto;
    }
    branch_tracing_enabled -= 1;
// label;
    mutex_unlock(&branch_tracing_mutex);
    }
#[no_mangle]
unsafe extern "C" fn branch_trace_init(tr: *mut trace_array) -> c_int {
    return enable_branch_tracing(tr);
    }
#[no_mangle]
unsafe extern "C" fn branch_trace_reset(tr: *mut trace_array) {
    disable_branch_tracing();
    }
    static enum print_line_t trace_branch_print(trace_iterator *iter,
    int flags, trace_event *event)
    {
pub static mut field: *mut c_void = core::ptr::null_mut();
    trace_assign_type(field, iter.ent);
    trace_seq_printf(&iter.seq, "[%s] %s:%s:%d\n",
    field.correct ? "  ok  " : " MISS ",
    field.func,
    field.file,
    field.line);
    return trace_handle_return(&iter.seq);
    }
#[no_mangle]
unsafe extern "C" fn branch_print_header(s: *mut seq_file) {
    seq_puts(s, "#           TASK-PID    CPU#    TIMESTAMP  CORRECT"
    "  FUNC:FILE:LINE\n"
    "#              | |       |          |         |   "
    "    |\n");
    }
pub static mut trace_event_functions: usize = 0;
pub static mut trace_event: usize = 0;
    static struct tracer branch_trace  =
    {
    .name		= "branch",
    .init		= branch_trace_init,
    .reset		= branch_trace_reset,

    .selftest	= trace_selftest_startup_branch,

    .print_header	= branch_print_header,
    };
#[no_mangle]
pub unsafe extern "C" fn init_branch_tracer() -> __init static int {
    let mut ret = 0;
    ret = register_trace_event(&trace_branch_event);
    if (!ret) {
    pr_warn!("Warning: could not register branch events\n");
    return 1;
    }
    return register_tracer(&branch_trace);
    }
    core_initcall!(init_branch_tracer);

    static inline
#[no_mangle]
#[no_mangle]
// duplicate fn: trace_likely_condition
pub unsafe extern "C" fn trace_likely_condition_dup(f: *mut ftrace_likely_data, val: c_int, expect: c_int) {
    }

#[no_mangle]
pub unsafe extern "C" fn ftrace_likely_update(f: *mut ftrace_likely_data, val: c_int, expect: c_int, is_constant: c_int) {
pub static mut flags: c_ulong = 0;
// A constant is always correct
    if (is_constant) {
    f.constant += 1;
    val = expect;
    }
//
// I would love to have a trace point here instead, but the
// trace point code is so inundated with unlikely and likely
// conditions that the recursive nightmare that exists is too
// much to try to get working. At least for now.
//
    trace_likely_condition(f, val, expect);
// FIXME: Make this atomic!
    if (val == expect) {
    f.data.correct += 1;
    }
    else {
    f.data.incorrect += 1;
    }
    user_access_restore(flags);
    }
    EXPORT_SYMBOL(ftrace_likely_update);
    extern unsigned long __start_annotated_branch_profile[];
    extern unsigned long __stop_annotated_branch_profile[];
#[no_mangle]
unsafe extern "C" fn annotated_branch_stat_headers(m: *mut seq_file) -> c_int {
    seq_puts(m, " correct incorrect  % "
    "       Function                "
    "  File              Line\n"
    " ------- ---------  - "
    "       --------                "
    "  ----              ----\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn get_incorrect_percent(p: *const ftrace_branch_data) -> c_long {
    let mut percent = 0;
    if (p.correct) {
    percent = p.incorrect * 100;
    percent /= p.correct + p.incorrect;
    } else {
    percent = p.incorrect ? 100 : -1;
    }
    return percent;
    }
    static const char *branch_stat_process_file(ftrace_branch_data *p)
    {
pub static mut f: *mut c_void = core::ptr::null_mut();
// Only print the file, not the path
    f = p.file + strlen(p.file);
    while (f >= p.file && *f != '/') {
    f -= 1;
    }
    return f += 1;
    }
#[no_mangle]
pub unsafe extern "C" fn branch_stat_show(m: *mut seq_file, p: *mut ftrace_branch_data, f: *mut c_char) {
    let mut percent = 0;
//
// The miss is overlayed on correct, and hit on incorrect.
//
    percent = get_incorrect_percent(p);
    if (percent < 0) {
    seq_puts(m, "  X ");
    }
    else {
    seq_printf(m, "%3ld ", percent);
    }
    seq_printf(m, "%-30.30s %-20.20s %d\n", p.func, f, p.line);
    }
#[no_mangle]
pub unsafe extern "C" fn branch_stat_show_normal(m: *mut seq_file, p: *mut ftrace_branch_data, f: *mut c_char) -> c_int {
    seq_printf(m, "%8lu %8lu ",  p.correct, p.incorrect);
    branch_stat_show(m, p, f);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn annotate_branch_stat_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut p = v;
pub static mut f: *mut c_void = core::ptr::null_mut();
    let mut l = 0;
    f = branch_stat_process_file(&p.data);
    if (!p.constant) {
    return branch_stat_show_normal(m, &p.data, f);
    }
    l = snprintf(core::ptr::null_mut(), 0, "/%lu", p.constant);
    l = l > 8 ? 0 : 8 - l;
    seq_printf(m, "%8lu/%lu %*lu ",
    p.data.correct, p.constant, l, p.data.incorrect);
    branch_stat_show(m, &p.data, f);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn annotated_branch_stat_start(trace: *mut tracer_stat) -> *mut c_void {
    return __start_annotated_branch_profile;
    }
#[no_mangle]
pub unsafe extern "C" fn annotated_branch_stat_next(v: *mut c_void, idx: c_int) -> *mut c_void {
    let mut p = v;
    p += 1;
    if (p >= __stop_annotated_branch_profile) {
    return core::ptr::null_mut();
    }
    return p;
    }
#[no_mangle]
unsafe extern "C" fn annotated_branch_stat_cmp(p1: *const c_void, p2: *const c_void) -> c_int {
    let mut a = p1;
    let mut b = p2;
    let mut percent_a = 0;
    let mut percent_b = 0;
    percent_a = get_incorrect_percent(a);
    percent_b = get_incorrect_percent(b);
    if (percent_a < percent_b) {
    return -1;
    }
    if (percent_a > percent_b) {
    return 1;
    }
    if (a.incorrect < b.incorrect) {
    return -1;
    }
    if (a.incorrect > b.incorrect) {
    return 1;
    }
//
// Since the above shows worse (incorrect) cases
// first, we continue that by showing best (correct)
// cases last.
//
    if (a.correct > b.correct) {
    return -1;
    }
    if (a.correct < b.correct) {
    return 1;
    }
    return 0;
    }
pub static mut tracer_stat: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn init_annotated_branch_stats() -> __init static int {
    let mut ret = 0;
    ret = register_stat_tracer(&annotated_branch_stats);
    if (ret) {
    pr_warn!("Warning: could not register annotated branches stats\n");
    return ret;
    }
    return 0;
    }
    fs_initcall!(init_annotated_branch_stats);

    extern unsigned long __start_branch_profile[];
    extern unsigned long __stop_branch_profile[];
#[no_mangle]
unsafe extern "C" fn all_branch_stat_headers(m: *mut seq_file) -> c_int {
    seq_puts(m, "   miss      hit    % "
    "       Function                "
    "  File              Line\n"
    " ------- ---------  - "
    "       --------                "
    "  ----              ----\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn all_branch_stat_start(trace: *mut tracer_stat) -> *mut c_void {
    return __start_branch_profile;
    }
#[no_mangle]
pub unsafe extern "C" fn all_branch_stat_next(v: *mut c_void, idx: c_int) -> *mut c_void {
    let mut p = v;
    p += 1;
    if (p >= __stop_branch_profile) {
    return core::ptr::null_mut();
    }
    return p;
    }
#[no_mangle]
unsafe extern "C" fn all_branch_stat_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut p = v;
pub static mut f: *mut c_void = core::ptr::null_mut();
    f = branch_stat_process_file(p);
    return branch_stat_show_normal(m, p, f);
    }
pub static mut tracer_stat: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn all_annotated_branch_stats() -> __init static int {
    let mut ret = 0;
    ret = register_stat_tracer(&all_branch_stats);
    if (ret) {
    pr_warn!("Warning: could not register all branches stats\n");
    return ret;
    }
    return 0;
    }
    fs_initcall!(all_annotated_branch_stats);
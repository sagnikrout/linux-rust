//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_selftest.c
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
// Include in trace.c

#[no_mangle]
pub unsafe extern "C" fn trace_valid_entry(entry: *mut trace_entry) -> c_int {
    match (entry.type) {
    TRACE_FN => {
    }
    TRACE_CTX => {
    }
    TRACE_WAKE => {
    }
    TRACE_STACK => {
    }
    TRACE_PRINT => {
    }
    TRACE_BRANCH => {
    }
    TRACE_GRAPH_ENT => {
    }
    TRACE_GRAPH_RETADDR_ENT => {
    }
    TRACE_GRAPH_RET => {
    return 1;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_test_buffer_cpu(buf: *mut array_buffer, cpu: c_int) -> c_int {
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut loops: c_uint = 0;
    while ((event = ring_buffer_consume(buf.buffer, cpu, core::ptr::null_mut(), core::ptr::null_mut()))) {
    entry = ring_buffer_event_data(event);
//
// The ring buffer is a size of trace_buf_size, if
// we loop more than the size, there's something wrong
// with the ring buffer.
//
    if (loops++ > trace_buf_size) {
    printk(".. bad ring buffer ");
// goto;
    }
    if (!trace_valid_entry(entry)) {
    printk(".. invalid entry %d ",
    entry.type);
// goto;
    }
    }
    return 0;
// label;
// disable tracing
    tracing_disabled = 1;
    printk(".. corrupted trace buffer .. ");
    return -1;
    }
//
// Test the trace buffer to see if all the elements
// are still sane.
//
#[no_mangle]
unsafe extern "C" fn trace_test_buffer(buf: *mut array_buffer, count: *mut c_ulong) -> int __maybe_unused {
    unsigned long flags, cnt = 0;
    int cpu, ret = 0;
// Don't allow flipping of max traces now
    local_irq_save(flags);
    arch_spin_lock(&buf.tr.max_lock);
    cnt = ring_buffer_entries(buf.buffer);
//
// The trace_test_buffer_cpu runs a while loop to consume all data.
// If the calling tracer is broken, and is constantly filling
// the buffer, this will run forever, and hard lock the box.
// We disable the ring buffer while we do this test to prevent
// a hard lock up.
//
    tracing_off();
    for_each_possible_cpu(cpu) {
    ret = trace_test_buffer_cpu(buf, cpu);
    if (ret) {
    break;
    }
    }
    tracing_on();
    arch_spin_unlock(&buf.tr.max_lock);
    local_irq_restore(flags);
    if (count) {
// count = cnt;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn warn_failed_init_tracer(trace: *mut tracer, init_ret: c_int) {
    printk("Failed to init %s tracer, init returned %d\n",
    trace.name, init_ret);
    }

    static int trace_selftest_test_probe1_cnt;
#[no_mangle]
pub unsafe extern "C" fn trace_selftest_test_probe1_func(ip: c_ulong, pip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    trace_selftest_test_probe1_cnt += 1;
    }
    static int trace_selftest_test_probe2_cnt;
#[no_mangle]
pub unsafe extern "C" fn trace_selftest_test_probe2_func(ip: c_ulong, pip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    trace_selftest_test_probe2_cnt += 1;
    }
    static int trace_selftest_test_probe3_cnt;
#[no_mangle]
pub unsafe extern "C" fn trace_selftest_test_probe3_func(ip: c_ulong, pip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    trace_selftest_test_probe3_cnt += 1;
    }
    static int trace_selftest_test_global_cnt;
#[no_mangle]
pub unsafe extern "C" fn trace_selftest_test_global_func(ip: c_ulong, pip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    trace_selftest_test_global_cnt += 1;
    }
    static int trace_selftest_test_dyn_cnt;
#[no_mangle]
pub unsafe extern "C" fn trace_selftest_test_dyn_func(ip: c_ulong, pip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    trace_selftest_test_dyn_cnt += 1;
    }
pub static mut ftrace_ops: usize = 0;
pub static mut ftrace_ops: usize = 0;
pub static mut ftrace_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn print_counts() {
    printk("(%d %d %d %d %d) ",
    trace_selftest_test_probe1_cnt,
    trace_selftest_test_probe2_cnt,
    trace_selftest_test_probe3_cnt,
    trace_selftest_test_global_cnt,
    trace_selftest_test_dyn_cnt);
    }
#[no_mangle]
unsafe extern "C" fn reset_counts() {
    trace_selftest_test_probe1_cnt = 0;
    trace_selftest_test_probe2_cnt = 0;
    trace_selftest_test_probe3_cnt = 0;
    trace_selftest_test_global_cnt = 0;
    trace_selftest_test_dyn_cnt = 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_selftest_ops(tr: *mut trace_array, cnt: c_int) -> c_int {
pub static mut save_ftrace_enabled: c_int = 0;
pub static mut dyn_ops: *mut c_void = core::ptr::null_mut();
pub static mut func1_name: *mut c_void = core::ptr::null_mut();
pub static mut func2_name: *mut c_void = core::ptr::null_mut();
    let mut len1 = 0;
    let mut len2 = 0;
pub static mut ret: c_int = 0;
    printk("PASSED\n");
    pr_info!("Testing dynamic ftrace ops #%d: ", cnt);
    ftrace_enabled = 1;
    reset_counts();
// Handle PPC64 '.' name
    func1_name = "*" __stringify(DYN_FTRACE_TEST_NAME);
    func2_name = "*" __stringify(DYN_FTRACE_TEST_NAME2);
    len1 = strlen(func1_name);
    len2 = strlen(func2_name);
//
// Probe 1 will trace function 1.
// Probe 2 will trace function 2.
// Probe 3 will trace functions 1 and 2.
//
    ftrace_set_filter(&test_probe1, func1_name, len1, 1);
    ftrace_set_filter(&test_probe2, func2_name, len2, 1);
    ftrace_set_filter(&test_probe3, func1_name, len1, 1);
    ftrace_set_filter(&test_probe3, func2_name, len2, 0);
    register_ftrace_function(&test_probe1);
    register_ftrace_function(&test_probe2);
    register_ftrace_function(&test_probe3);
// First time we are running with main function
    if (cnt > 1) {
    ftrace_init_array_ops(tr, trace_selftest_test_global_func);
    register_ftrace_function(tr.ops);
    }
    DYN_FTRACE_TEST_NAME();
    print_counts();
    if (trace_selftest_test_probe1_cnt != 1) {
// goto;
    }
    if (trace_selftest_test_probe2_cnt != 0) {
// goto;
    }
    if (trace_selftest_test_probe3_cnt != 1) {
// goto;
    }
    if (cnt > 1) {
    if (trace_selftest_test_global_cnt == 0) {
// goto;
    }
    }
    DYN_FTRACE_TEST_NAME2();
    print_counts();
    if (trace_selftest_test_probe1_cnt != 1) {
// goto;
    }
    if (trace_selftest_test_probe2_cnt != 1) {
// goto;
    }
    if (trace_selftest_test_probe3_cnt != 2) {
// goto;
    }
// Add a dynamic probe
    dyn_ops = kzalloc_obj(*dyn_ops);
    if (!dyn_ops) {
    printk("MEMORY ERROR ");
// goto;
    }
    dyn_ops.func = trace_selftest_test_dyn_func;
    register_ftrace_function(dyn_ops);
    trace_selftest_test_global_cnt = 0;
    DYN_FTRACE_TEST_NAME();
    print_counts();
    if (trace_selftest_test_probe1_cnt != 2) {
// goto;
    }
    if (trace_selftest_test_probe2_cnt != 1) {
// goto;
    }
    if (trace_selftest_test_probe3_cnt != 3) {
// goto;
    }
    if (cnt > 1) {
    if (trace_selftest_test_global_cnt == 0) {
// goto;
    }
    }
    if (trace_selftest_test_dyn_cnt == 0) {
// goto;
    }
    DYN_FTRACE_TEST_NAME2();
    print_counts();
    if (trace_selftest_test_probe1_cnt != 2) {
// goto;
    }
    if (trace_selftest_test_probe2_cnt != 2) {
// goto;
    }
    if (trace_selftest_test_probe3_cnt != 4) {
// goto;
    }
// Remove trace function from probe 3
    func1_name = "!" __stringify(DYN_FTRACE_TEST_NAME);
    len1 = strlen(func1_name);
    ftrace_set_filter(&test_probe3, func1_name, len1, 0);
    DYN_FTRACE_TEST_NAME();
    print_counts();
    if (trace_selftest_test_probe1_cnt != 3) {
// goto;
    }
    if (trace_selftest_test_probe2_cnt != 2) {
// goto;
    }
    if (trace_selftest_test_probe3_cnt != 4) {
// goto;
    }
    if (cnt > 1) {
    if (trace_selftest_test_global_cnt == 0) {
// goto;
    }
    }
    if (trace_selftest_test_dyn_cnt == 0) {
// goto;
    }
    DYN_FTRACE_TEST_NAME2();
    print_counts();
    if (trace_selftest_test_probe1_cnt != 3) {
// goto;
    }
    if (trace_selftest_test_probe2_cnt != 3) {
// goto;
    }
    if (trace_selftest_test_probe3_cnt != 5) {
// goto;
    }
    ret = 0;
// label;
    unregister_ftrace_function(dyn_ops);
    kfree(dyn_ops);
// label;
// Purposely unregister in the same order
    unregister_ftrace_function(&test_probe1);
    unregister_ftrace_function(&test_probe2);
    unregister_ftrace_function(&test_probe3);
    if (cnt > 1) {
    unregister_ftrace_function(tr.ops);
    }
    ftrace_reset_array_ops(tr);
// Make sure everything is off
    reset_counts();
    DYN_FTRACE_TEST_NAME();
    DYN_FTRACE_TEST_NAME();
    if (trace_selftest_test_probe1_cnt ||
    trace_selftest_test_probe2_cnt ||
    trace_selftest_test_probe3_cnt ||
    trace_selftest_test_global_cnt ||
    trace_selftest_test_dyn_cnt) {
    ret = -1;
    }
    ftrace_enabled = save_ftrace_enabled;
    return ret;
    }
// Test dynamic code modification and ftrace filters
#[no_mangle]
pub unsafe extern "C" fn trace_selftest_startup_dynamic_tracing(trace: *mut tracer, tr: *mut trace_array) -> c_int {
pub static mut save_ftrace_enabled: c_int = 0;
    let mut count = 0;
pub static mut func_name: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
// The ftrace test PASSED
    printk("PASSED\n");
    pr_info!("Testing dynamic ftrace: ");
// enable tracing, and record the filter function
    ftrace_enabled = 1;
// passed in by parameter to fool gcc from optimizing
    func();
//
// Some archs *cough*PowerPC*cough* add characters to the
// start of the function names. We simply put a '*' to
// accommodate them.
//
    func_name = "*" __stringify(DYN_FTRACE_TEST_NAME);
// filter only on our function
    ftrace_set_global_filter(func_name, strlen(func_name), 1);
// enable tracing
    ret = tracer_init(trace, tr);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
// goto;
    }
// Sleep for a 1/10 of a second
    msleep(100);
// we should have nothing in the buffer
    ret = trace_test_buffer(&tr.array_buffer, &count);
    if (ret) {
// goto;
    }
    if (count) {
    ret = -1;
    printk(".. filter did not filter .. ");
// goto;
    }
// call our function again
    func();
// sleep again
    msleep(100);
// stop the tracing.
    tracing_stop();
    ftrace_enabled = 0;
// check the trace buffer
    ret = trace_test_buffer(&tr.array_buffer, &count);
    ftrace_enabled = 1;
    tracing_start();
// we should only have one item
    if (!ret && count != 1) {
    trace.reset(tr);
    printk(".. filter failed count=%ld ..", count);
    ret = -1;
// goto;
    }
// Test the ops with global tracing running
    ret = trace_selftest_ops(tr, 1);
    trace.reset(tr);
// label;
    ftrace_enabled = save_ftrace_enabled;
// Enable tracing on all functions again
    ftrace_set_global_filter(core::ptr::null_mut(), 0, 1);
// Test the ops with global tracing off
    if (!ret) {
    ret = trace_selftest_ops(tr, 2);
    }
    return ret;
    }
    static int trace_selftest_recursion_cnt;
#[no_mangle]
pub unsafe extern "C" fn trace_selftest_test_recursion_func(ip: c_ulong, pip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
//
// This function is registered without the recursion safe flag.
// The ftrace infrastructure should provide the recursion
// protection. If not, this will crash the kernel!
//
    if (trace_selftest_recursion_cnt++ > 10) {
    return;
    }
    DYN_FTRACE_TEST_NAME();
    }
#[no_mangle]
pub unsafe extern "C" fn trace_selftest_test_recursion_safe_func(ip: c_ulong, pip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
//
// We said we would provide our own recursion. By calling
// this function again, we should recurse back into this function
// and count again. But this only happens if the arch supports
// all of ftrace features and nothing else is using the function
// tracing utility.
//
    if (trace_selftest_recursion_cnt++) {
    return;
    }
    DYN_FTRACE_TEST_NAME();
    }
pub static mut ftrace_ops: usize = 0;
pub static mut ftrace_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn trace_selftest_function_recursion() -> c_int {
pub static mut save_ftrace_enabled: c_int = 0;
pub static mut func_name: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    let mut ret = 0;
// The previous test PASSED
    pr_cont("PASSED\n");
    pr_info!("Testing ftrace recursion: ");
// enable tracing, and record the filter function
    ftrace_enabled = 1;
// Handle PPC64 '.' name
    func_name = "*" __stringify(DYN_FTRACE_TEST_NAME);
    len = strlen(func_name);
    ret = ftrace_set_filter(&test_rec_probe, func_name, len, 1);
    if (ret) {
    pr_cont("*Could not set filter* ");
// goto;
    }
    ret = register_ftrace_function(&test_rec_probe);
    if (ret) {
    pr_cont("*could not register callback* ");
// goto;
    }
    DYN_FTRACE_TEST_NAME();
    unregister_ftrace_function(&test_rec_probe);
    ret = -1;
//
// Recursion allows for transitions between context,
// and may call the callback twice.
//
    if (trace_selftest_recursion_cnt != 1 &&
    trace_selftest_recursion_cnt != 2) {
    pr_cont("*callback not called once (or twice) (%d)* ",
    trace_selftest_recursion_cnt);
// goto;
    }
    trace_selftest_recursion_cnt = 1;
    pr_cont("PASSED\n");
    pr_info!("Testing ftrace recursion safe: ");
    ret = ftrace_set_filter(&test_recsafe_probe, func_name, len, 1);
    if (ret) {
    pr_cont("*Could not set filter* ");
// goto;
    }
    ret = register_ftrace_function(&test_recsafe_probe);
    if (ret) {
    pr_cont("*could not register callback* ");
// goto;
    }
    DYN_FTRACE_TEST_NAME();
    unregister_ftrace_function(&test_recsafe_probe);
    ret = -1;
    if (trace_selftest_recursion_cnt != 2) {
    pr_cont("*callback not called expected 2 times (%d)* ",
    trace_selftest_recursion_cnt);
// goto;
    }
    ret = 0;
// label;
    ftrace_enabled = save_ftrace_enabled;
    return ret;
    }

    static enum {
    TRACE_SELFTEST_REGS_START,
    TRACE_SELFTEST_REGS_FOUND,
    TRACE_SELFTEST_REGS_NOT_FOUND,
    } trace_selftest_regs_stat;
#[no_mangle]
pub unsafe extern "C" fn trace_selftest_test_regs_func(ip: c_ulong, pip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    let mut regs = ftrace_get_regs(fregs);
    if (regs) {
    trace_selftest_regs_stat = TRACE_SELFTEST_REGS_FOUND;
    }
    else {
    trace_selftest_regs_stat = TRACE_SELFTEST_REGS_NOT_FOUND;
    }
    }
pub static mut ftrace_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn trace_selftest_function_regs() -> c_int {
pub static mut save_ftrace_enabled: c_int = 0;
pub static mut func_name: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    let mut ret = 0;
pub static mut supported: c_int = 0;

    supported = 1;

// The previous test PASSED
    pr_cont("PASSED\n");
    pr_info!("Testing ftrace regs%s: ",
    !supported ? "(no arch support)" : "");
// enable tracing, and record the filter function
    ftrace_enabled = 1;
// Handle PPC64 '.' name
    func_name = "*" __stringify(DYN_FTRACE_TEST_NAME);
    len = strlen(func_name);
    ret = ftrace_set_filter(&test_regs_probe, func_name, len, 1);
//
// If DYNAMIC_FTRACE is not set, then we just trace all functions.
// This test really doesn't care.
//
    if (ret && ret != -ENODEV) {
    pr_cont("*Could not set filter* ");
// goto;
    }
    ret = register_ftrace_function(&test_regs_probe);
//
// Now if the arch does not support passing regs, then this should
// have failed.
//
    if (!supported) {
    if (!ret) {
    pr_cont("*registered save-regs without arch support* ");
// goto;
    }
    test_regs_probe.flags |= FTRACE_OPS_FL_SAVE_REGS_IF_SUPPORTED;
    ret = register_ftrace_function(&test_regs_probe);
    }
    if (ret) {
    pr_cont("*could not register callback* ");
// goto;
    }
    DYN_FTRACE_TEST_NAME();
    unregister_ftrace_function(&test_regs_probe);
    ret = -1;
    match (trace_selftest_regs_stat) {
    TRACE_SELFTEST_REGS_START => {
    pr_cont("*callback never called* ");
// goto;
    }
    TRACE_SELFTEST_REGS_FOUND => {
    if (supported) {
    // break;
    }
    pr_cont("*callback received regs without arch support* ");
// goto;
    }
    TRACE_SELFTEST_REGS_NOT_FOUND => {
    if (!supported) {
    // break;
    }
    pr_cont("*callback received core::ptr::null_mut() regs* ");
// goto;
    }
    }
    ret = 0;
// label;
    ftrace_enabled = save_ftrace_enabled;
    return ret;
    }
//
// Simple verification test of ftrace function tracer.
// Enable ftrace, sleep 1/10 second, and then read the trace
// buffer to see if all is in order.
//
#[no_mangle]
pub unsafe extern "C" fn trace_selftest_startup_function(trace: *mut tracer, tr: *mut trace_array) -> c_int {
pub static mut save_ftrace_enabled: c_int = 0;
    let mut count = 0;
    let mut ret = 0;

    if (ftrace_filter_param) {
    printk(" ... kernel command line filter set: force PASS ... ");
    return 0;
    }

// make sure msleep has been recorded
    msleep(1);
// start the tracing
    ftrace_enabled = 1;
    ret = tracer_init(trace, tr);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
// goto;
    }
// Sleep for a 1/10 of a second
    msleep(100);
// stop the tracing.
    tracing_stop();
    ftrace_enabled = 0;
// check the trace buffer
    ret = trace_test_buffer(&tr.array_buffer, &count);
    ftrace_enabled = 1;
    trace.reset(tr);
    tracing_start();
    if (!ret && !count) {
    printk(".. no entries found ..");
    ret = -1;
// goto;
    }
    ret = trace_selftest_startup_dynamic_tracing(trace, tr,
    DYN_FTRACE_TEST_NAME);
    if (ret) {
// goto;
    }
    ret = trace_selftest_function_recursion();
    if (ret) {
// goto;
    }
    ret = trace_selftest_function_regs();
// label;
    ftrace_enabled = save_ftrace_enabled;
// kill ftrace totally if we failed
    if (ret) {
    ftrace_kill();
    }
    return ret;
    }

pub const CHAR_NUMBER: c_int = 123;
pub const SHORT_NUMBER: c_int = 12345;
pub const WORD_NUMBER: c_int = 1234567890;

pub const ERRSTR_BUFLEN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fgraph_fixture {
    pub gops: fgraph_ops,
    pub store_size: c_int,
    pub store_type_name: *const c_char,
    pub error_str_buf: [c_char; ERRSTR_BUFLEN],
    pub error_str: *mut c_char,
}

#[no_mangle]
pub unsafe extern "C" fn store_entry(trace: *mut ftrace_graph_ent, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) -> c_int {
    let mut fixture = container_of!(gops, fgraph_fixture, gops);
    let mut type = fixture.store_type_name;
pub static mut size: c_int = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = fgraph_reserve_data(gops.idx, size);
    if (!p) {
    snprintf(fixture.error_str_buf, ERRSTR_BUFLEN,
    "Failed to reserve %s\n", type);
    return 0;
    }
    match (size) {
    1 => {
// p = CHAR_NUMBER;
    // break;
    }
    2 => {
// p = SHORT_NUMBER;
    // break;
    }
    4 => {
// p = WORD_NUMBER;
    // break;
    }
    8 => {
// p = LONG_NUMBER;
    // break;
    }
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn store_return(trace: *mut ftrace_graph_ret, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) {
    let mut fixture = container_of!(gops, fgraph_fixture, gops);
    let mut type = fixture.store_type_name;
pub static mut expect: c_longlong = 0;
pub static mut found: c_longlong = 0;
    let mut size = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = fgraph_retrieve_data(gops.idx, &size);
    if (!p) {
    snprintf(fixture.error_str_buf, ERRSTR_BUFLEN,
    "Failed to retrieve %s\n", type);
    return;
    }
    if (fixture.store_size > size) {
    snprintf(fixture.error_str_buf, ERRSTR_BUFLEN,
    "Retrieved size %d is smaller than expected %d\n",
    size, (int)fixture.store_size);
    return;
    }
    match (fixture.store_size) {
    1 => {
    expect = CHAR_NUMBER;
    found = *p;
    // break;
    }
    2 => {
    expect = SHORT_NUMBER;
    found = *p;
    // break;
    }
    4 => {
    expect = WORD_NUMBER;
    found = *p;
    // break;
    }
    8 => {
    expect = LONG_NUMBER;
    found = *p;
    // break;
    }
    }
    if (found != expect) {
    snprintf(fixture.error_str_buf, ERRSTR_BUFLEN,
    "%s returned not %lld but %lld\n", type, expect, found);
    return;
    }
    fixture.error_str = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn init_fgraph_fixture(fixture: *mut fgraph_fixture) -> c_int {
pub static mut func_name: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    snprintf(fixture.error_str_buf, ERRSTR_BUFLEN,
    "Failed to execute storage %s\n", fixture.store_type_name);
    fixture.error_str = fixture.error_str_buf;
    func_name = "*" __stringify(DYN_FTRACE_TEST_NAME);
    len = strlen(func_name);
    return ftrace_set_filter(&fixture.gops.ops, func_name, len, 1);
    }
// Test fgraph storage for each size
#[no_mangle]
unsafe extern "C" fn test_graph_storage_single(fixture: *mut fgraph_fixture) -> c_int {
pub static mut size: c_int = 0;
    let mut ret = 0;
    pr_cont("PASSED\n");
    pr_info!("Testing fgraph storage of %d byte%s: ", size, str_plural(size));
    ret = init_fgraph_fixture(fixture);
    if (ret && ret != -ENODEV) {
    pr_cont("*Could not set filter* ");
    return -1;
    }
    ret = register_ftrace_graph(&fixture.gops);
    if (ret) {
    pr_warn!("Failed to init store_bytes fgraph tracing\n");
    return -1;
    }
    DYN_FTRACE_TEST_NAME();
    unregister_ftrace_graph(&fixture.gops);
    if (fixture.error_str) {
    pr_cont("*** %s ***", fixture.error_str);
    return -1;
    }
    return 0;
    }
    static struct fgraph_fixture store_bytes[4] __initdata = {
    [0] = {
    .gops = {
    .entryfunc		= store_entry,
    .retfunc		= store_return,
    },
    .store_size = 1,
    .store_type_name = "byte",
    },
    [1] = {
    .gops = {
    .entryfunc		= store_entry,
    .retfunc		= store_return,
    },
    .store_size = 2,
    .store_type_name = "short",
    },
    [2] = {
    .gops = {
    .entryfunc		= store_entry,
    .retfunc		= store_return,
    },
    .store_size = 4,
    .store_type_name = "word",
    },
    [3] = {
    .gops = {
    .entryfunc		= store_entry,
    .retfunc		= store_return,
    },
    .store_size = 8,
    .store_type_name = "long long",
    },
    };
#[no_mangle]
unsafe extern "C" fn test_graph_storage_multi() -> __init int {
pub static mut fixture: *mut c_void = core::ptr::null_mut();
pub static mut printed: bool = false;
    let mut i = 0;
    let mut j = 0;
    let mut ret = 0;
    pr_cont("PASSED\n");
    pr_info!("Testing multiple fgraph storage on a function: ");
    while (i < ARRAY_SIZE!(store_bytes)) {
    fixture = &store_bytes[i];
    ret = init_fgraph_fixture(fixture);
    if (ret && ret != -ENODEV) {
    pr_cont("*Could not set filter* ");
    printed = true;
// goto;
    }
    }
    while (j < ARRAY_SIZE!(store_bytes)) {
    fixture = &store_bytes[j];
    ret = register_ftrace_graph(&fixture.gops);
    if (ret) {
    pr_warn!("Failed to init store_bytes fgraph tracing\n");
    printed = true;
// goto;
    }
    }
    DYN_FTRACE_TEST_NAME();
// label;
    while (--j >= 0) {
    fixture = &store_bytes[j];
    unregister_ftrace_graph(&fixture.gops);
    if (fixture.error_str && !printed) {
    pr_cont("*** %s ***", fixture.error_str);
    printed = true;
    }
    }
// label;
    while (--i >= 0) {
    fixture = &store_bytes[i];
    ftrace_free_filter(&fixture.gops.ops);
    if (fixture.error_str && !printed) {
    pr_cont("*** %s ***", fixture.error_str);
    printed = true;
    }
    }
    return printed ? -1 : 0;
    }
// Test the storage passed across function_graph entry and return
#[no_mangle]
unsafe extern "C" fn test_graph_storage() -> __init int {
    let mut ret = 0;
    ret = test_graph_storage_single(&store_bytes[0]);
    if (ret) {
    return ret;
    }
    ret = test_graph_storage_single(&store_bytes[1]);
    if (ret) {
    return ret;
    }
    ret = test_graph_storage_single(&store_bytes[2]);
    if (ret) {
    return ret;
    }
    ret = test_graph_storage_single(&store_bytes[3]);
    if (ret) {
    return ret;
    }
    ret = test_graph_storage_multi();
    if (ret) {
    return ret;
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn test_graph_storage() -> c_int { return 0; }

// Maximum number of functions to trace before diagnosing a hang
pub const GRAPH_MAX_FUNC_TEST: c_int = 100000000;
    static unsigned int graph_hang_thresh;
// Wrap the real function entry probe to avoid possible hanging
#[no_mangle]
pub unsafe extern "C" fn trace_graph_entry_watchdog(trace: *mut ftrace_graph_ent, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) -> c_int {
// This is harmlessly racy, we want to approximately detect a hang
    if (unlikely(++graph_hang_thresh > GRAPH_MAX_FUNC_TEST)) {
    ftrace_graph_stop();
    printk("BUG: Function graph tracer hang!\n");
    if (ftrace_dump_on_oops_enabled()) {
    ftrace_dump(DUMP_ALL);
// ftrace_dump() disables tracing
    tracing_on();
    }
    return 0;
    }
    return trace_graph_entry(trace, gops, fregs);
    }
    static struct fgraph_ops fgraph_ops __initdata  = {
    .entryfunc		= &trace_graph_entry_watchdog,
    .retfunc		= &trace_graph_return,
    };

pub static mut direct: usize = 0;

//
// Pretty much the same than for the function tracer from which the selftest
// has been borrowed.
//
#[no_mangle]
pub unsafe extern "C" fn trace_selftest_startup_function_graph(trace: *mut tracer, tr: *mut trace_array) -> c_int {
    let mut ret = 0;
    let mut count = 0;
pub static mut func_name: *mut c_void = core::ptr::null_mut();

    if (ftrace_filter_param) {
    printk(" ... kernel command line filter set: force PASS ... ");
    return 0;
    }

//
// Simulate the init() callback but we attach a watchdog callback
// to detect and recover from possible hangs
//
    tracing_reset_online_cpus(&tr.array_buffer);
    fgraph_ops.private = tr;
    ret = register_ftrace_graph(&fgraph_ops);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
// goto;
    }
    tracing_start_cmdline_record();
// Sleep for a 1/10 of a second
    msleep(100);
// Have we just recovered from a hang?
    if (graph_hang_thresh > GRAPH_MAX_FUNC_TEST) {
    disable_tracing_selftest("recovering from a hang");
    ret = -1;
// goto;
    }
    tracing_stop();
// check the trace buffer
    ret = trace_test_buffer(&tr.array_buffer, &count);
// Need to also simulate the tr->reset to remove this fgraph_ops
    tracing_stop_cmdline_record();
    unregister_ftrace_graph(&fgraph_ops);
    tracing_start();
    if (!ret && !count) {
    printk(".. no entries found ..");
    ret = -1;
// goto;
    }

//
// These tests can take some time to run. Make sure on non PREEMPT
// kernels, we do not trigger the softlockup detector.
//
    cond_resched();
    tracing_reset_online_cpus(&tr.array_buffer);
    fgraph_ops.private = tr;
//
// Some archs *cough*PowerPC*cough* add characters to the
// start of the function names. We simply put a '*' to
// accommodate them.
//
    func_name = "*" __stringify(DYN_FTRACE_TEST_NAME);
    ftrace_set_global_filter(func_name, strlen(func_name), 1);
//
// Register direct function together with graph tracer
// and make sure we get graph trace.
//
    ftrace_set_filter_ip(&direct, (unsigned long)DYN_FTRACE_TEST_NAME, 0, 0);
    ret = register_ftrace_direct(&direct,
    (unsigned long)ftrace_stub_direct_tramp);
    if (ret) {
// goto;
    }
    cond_resched();
    ret = register_ftrace_graph(&fgraph_ops);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
// goto;
    }
    DYN_FTRACE_TEST_NAME();
    count = 0;
    tracing_stop();
// check the trace buffer
    ret = trace_test_buffer(&tr.array_buffer, &count);
    unregister_ftrace_graph(&fgraph_ops);
    ret = unregister_ftrace_direct(&direct,
    (unsigned long)ftrace_stub_direct_tramp,
    true);
    if (ret) {
// goto;
    }
    cond_resched();
    tracing_start();
    if (!ret && !count) {
    ret = -1;
// goto;
    }
// Enable tracing on all functions again
    ftrace_set_global_filter(core::ptr::null_mut(), 0, 1);

    ret = test_graph_storage();
// Don't test dynamic tracing, the function tracer already did
// label;
// Stop it if we failed
    if (ret) {
    ftrace_graph_stop();
    }
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn trace_selftest_startup_irqsoff(trace: *mut tracer, tr: *mut trace_array) -> c_int {
pub static mut save_max: c_ulong = 0;
    let mut count = 0;
    let mut ret = 0;
// start the tracing
    ret = tracer_init(trace, tr);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
    return ret;
    }
// reset the max latency
    tr.max_latency = 0;
// disable interrupts for a bit
    local_irq_disable();
    udelay(100);
    local_irq_enable();
//
// Stop the tracer to avoid a warning subsequent
// to buffer flipping failure because tracing_stop()
// disables the tr and max buffers, making flipping impossible
// in case of parallels max irqs off latencies.
//
    trace.stop(tr);
// stop the tracing.
    tracing_stop();
// check both trace buffers
    ret = trace_test_buffer(&tr.array_buffer, core::ptr::null_mut());
    if (!ret) {
    ret = trace_test_buffer(&tr.snapshot_buffer, &count);
    }
    trace.reset(tr);
    tracing_start();
    if (!ret && !count) {
    printk(".. no entries found ..");
    ret = -1;
    }
    tr.max_latency = save_max;
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn trace_selftest_startup_preemptoff(trace: *mut tracer, tr: *mut trace_array) -> c_int {
pub static mut save_max: c_ulong = 0;
    let mut count = 0;
    let mut ret = 0;
//
// Now that the big kernel lock is no longer preemptible,
// and this is called with the BKL held, it will always
// fail. If preemption is already disabled, simply
// pass the test. When the BKL is removed, or becomes
// preemptible again, we will once again test this,
// so keep it in.
//
    if (preempt_count()) {
    printk("can not test ... force ");
    return 0;
    }
// start the tracing
    ret = tracer_init(trace, tr);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
    return ret;
    }
// reset the max latency
    tr.max_latency = 0;
// disable preemption for a bit
    preempt_disable();
    udelay(100);
    preempt_enable();
//
// Stop the tracer to avoid a warning subsequent
// to buffer flipping failure because tracing_stop()
// disables the tr and max buffers, making flipping impossible
// in case of parallels max preempt off latencies.
//
    trace.stop(tr);
// stop the tracing.
    tracing_stop();
// check both trace buffers
    ret = trace_test_buffer(&tr.array_buffer, core::ptr::null_mut());
    if (!ret) {
    ret = trace_test_buffer(&tr.snapshot_buffer, &count);
    }
    trace.reset(tr);
    tracing_start();
    if (!ret && !count) {
    printk(".. no entries found ..");
    ret = -1;
    }
    tr.max_latency = save_max;
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn trace_selftest_startup_preemptirqsoff(trace: *mut tracer, tr: *mut trace_array) -> c_int {
pub static mut save_max: c_ulong = 0;
    let mut count = 0;
    let mut ret = 0;
//
// Now that the big kernel lock is no longer preemptible,
// and this is called with the BKL held, it will always
// fail. If preemption is already disabled, simply
// pass the test. When the BKL is removed, or becomes
// preemptible again, we will once again test this,
// so keep it in.
//
    if (preempt_count()) {
    printk("can not test ... force ");
    return 0;
    }
// start the tracing
    ret = tracer_init(trace, tr);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
// goto;
    }
// reset the max latency
    tr.max_latency = 0;
// disable preemption and interrupts for a bit
    preempt_disable();
    local_irq_disable();
    udelay(100);
    preempt_enable();
// reverse the order of preempt vs irqs
    local_irq_enable();
//
// Stop the tracer to avoid a warning subsequent
// to buffer flipping failure because tracing_stop()
// disables the tr and max buffers, making flipping impossible
// in case of parallels max irqs/preempt off latencies.
//
    trace.stop(tr);
// stop the tracing.
    tracing_stop();
// check both trace buffers
    ret = trace_test_buffer(&tr.array_buffer, core::ptr::null_mut());
    if (ret) {
// goto;
    }
    ret = trace_test_buffer(&tr.snapshot_buffer, &count);
    if (ret) {
// goto;
    }
    if (!ret && !count) {
    printk(".. no entries found ..");
    ret = -1;
// goto;
    }
// do the test by disabling interrupts first this time
    tr.max_latency = 0;
    tracing_start();
    trace.start(tr);
    preempt_disable();
    local_irq_disable();
    udelay(100);
    preempt_enable();
// reverse the order of preempt vs irqs
    local_irq_enable();
    trace.stop(tr);
// stop the tracing.
    tracing_stop();
// check both trace buffers
    ret = trace_test_buffer(&tr.array_buffer, core::ptr::null_mut());
    if (ret) {
// goto;
    }
    ret = trace_test_buffer(&tr.snapshot_buffer, &count);
    if (!ret && !count) {
    printk(".. no entries found ..");
    ret = -1;
// goto;
    }
// label;
    tracing_start();
// label;
    trace.reset(tr);
    tr.max_latency = save_max;
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn trace_selftest_startup_nop(trace: *mut tracer, tr: *mut trace_array) -> c_int {
// What could possibly go wrong?
    return 0;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wakeup_test_data {
    pub is_ready: completion,
    pub go: c_int,
}

#[no_mangle]
unsafe extern "C" fn trace_wakeup_test_thread(data: *mut c_void) -> c_int {
// Make this a -deadline thread
pub static mut sched_attr: usize = 0;
    let mut x = data;
    sched_setattr(current, &attr);
// Make it know we have a new prio
    complete(&x.is_ready);
// now go to sleep and let the test wake us up
    set_current_state(TASK_INTERRUPTIBLE);
    while (!x.go) {
    schedule();
    set_current_state(TASK_INTERRUPTIBLE);
    }
    complete(&x.is_ready);
    set_current_state(TASK_INTERRUPTIBLE);
// we are awake, now wait to disappear
    while (!kthread_should_stop()) {
    schedule();
    set_current_state(TASK_INTERRUPTIBLE);
    }
    __set_current_state(TASK_RUNNING);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_selftest_startup_wakeup(trace: *mut tracer, tr: *mut trace_array) -> c_int {
pub static mut save_max: c_ulong = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut data: usize = 0;
    let mut count = 0;
    let mut ret = 0;
    memset(&data, 0, sizeof!(data));
    init_completion(&data.is_ready);
// create a -deadline thread
    p = kthread_run(trace_wakeup_test_thread, &data, "ftrace-test");
    if (IS_ERR(p)) {
    printk("Failed to create ftrace wakeup test thread ");
    return -1;
    }
// make sure the thread is running at -deadline policy
    wait_for_completion(&data.is_ready);
// start the tracing
    ret = tracer_init(trace, tr);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
    return ret;
    }
// reset the max latency
    tr.max_latency = 0;
    while (task_is_runnable(p)) {
//
// Sleep to make sure the -deadline thread is asleep too.
// On virtual machines we can't rely on timings,
// but we want to make sure this test still works.
//
    msleep(100);
    }
    init_completion(&data.is_ready);
    data.go = 1;
// memory barrier is in the wake_up_process()
    wake_up_process(p);
// Wait for the task to wake up
    wait_for_completion(&data.is_ready);
// stop the tracing.
    tracing_stop();
// check both trace buffers
    ret = trace_test_buffer(&tr.array_buffer, core::ptr::null_mut());
    if (!ret) {
    ret = trace_test_buffer(&tr.snapshot_buffer, &count);
    }
    trace.reset(tr);
    tracing_start();
    tr.max_latency = save_max;
// kill the thread
    kthread_stop(p);
    if (!ret && !count) {
    printk(".. no entries found ..");
    ret = -1;
    }
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn trace_selftest_startup_branch(trace: *mut tracer, tr: *mut trace_array) -> c_int {
    let mut count = 0;
    let mut ret = 0;
// start the tracing
    ret = tracer_init(trace, tr);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
    return ret;
    }
// Sleep for a 1/10 of a second
    msleep(100);
// stop the tracing.
    tracing_stop();
// check the trace buffer
    ret = trace_test_buffer(&tr.array_buffer, &count);
    trace.reset(tr);
    tracing_start();
    if (!ret && !count) {
    printk(".. no entries found ..");
    ret = -1;
    }
    return ret;
    }
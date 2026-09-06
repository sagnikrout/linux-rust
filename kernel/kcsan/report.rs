//! Automatically rewritten from C to Rust
//! Source: kernel/kcsan/report.c
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
// KCSAN reporting.
//
// Copyright (C) 2019, Google LLC.
//

//
// Max. number of stack entries to show in the report.
//
pub const NUM_STACK_ENTRIES: c_int = 64;
// Common access info.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct access_info {
    pub ptr: *const volatile void,
    pub size: usize,
    pub access_type: c_int,
    pub task_pid: c_int,
    pub cpu_id: c_int,
    pub ip: c_ulong,
}

//
// Other thread info: communicated from other racing thread to thread that set
// up the watchpoint, which then prints the complete report atomically.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct other_info {
    pub ai: access_info,
    pub stack_entries: [c_ulong; NUM_STACK_ENTRIES],
    pub num_stack_entries: c_int,
//
// Optionally pass @current. Typically we do not need to pass @current
// via @other_info since just @task_pid is sufficient. Passing @current
// has additional overhead.
//
// To safely pass @current, we must either use get_task_struct
// put_task_struct, or stall the thread that populated @other_info.
//
// We cannot rely on get_task_struct/put_task_struct in case
// release_report() races with a task being released, and would have to
// free it in release_report(). This may result in deadlock if we want
// to use KCSAN on the allocators.
//
// Since we also want to reliably print held locks for
// CONFIG_KCSAN_VERBOSE, the current implementation stalls the thread
// that populated @other_info until it has been consumed.
//
    pub task: *mut task_struct,
}

//
// To never block any producers of struct other_info, we need as many elements
// as we have watchpoints (upper bound on concurrent races to report).
//
    static struct other_info other_infos[CONFIG_KCSAN_NUM_WATCHPOINTS + NUM_SLOTS-1];
//
// Information about reported races; used to rate limit reporting.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct report_time {
//
// The last time the race was reported.
//
    pub time: c_ulong,
//
// The frames of the 2 threads; if only 1 thread is known, one frame
// will be 0.
//
    pub frame1: c_ulong,
    pub frame2: c_ulong,
}

//
// Since we also want to be able to debug allocators with KCSAN, to avoid
// deadlock, report_times cannot be dynamically resized with krealloc in
// rate_limit_report.
//
// Therefore, we use a fixed-size array, which at most will occupy a page. This
// still adequately rate limits reports, assuming that a) number of unique data
// races is not excessive, and b) occurrence of unique races within the
// same time window is limited.
//

    (CONFIG_KCSAN_REPORT_ONCE_IN_MS > REPORT_TIMES_MAX ?                   
    REPORT_TIMES_MAX :                                            
    CONFIG_KCSAN_REPORT_ONCE_IN_MS)
    static struct report_time report_times[REPORT_TIMES_SIZE];
//
// Spinlock serializing report generation, and access to @other_infos. Although
// it could make sense to have a finer-grained locking story for @other_infos,
// report generation needs to be serialized either way, so not much is gained.
//
pub static mut report_lock: usize = 0;
//
// Checks if the race identified by thread frames frame1 and frame2 has
// been reported since (now - KCSAN_REPORT_ONCE_IN_MS).
//
#[no_mangle]
unsafe extern "C" fn rate_limit_report(frame1: c_ulong, frame2: c_ulong) -> bool {
    let mut use_entry = &report_times[0];
    let mut invalid_before = 0;
    let mut i = 0;
    BUILD_BUG_ON!(CONFIG_KCSAN_REPORT_ONCE_IN_MS != 0 && REPORT_TIMES_SIZE == 0);
    if (CONFIG_KCSAN_REPORT_ONCE_IN_MS == 0) {
    return false;
    }
    invalid_before = jiffies - msecs_to_jiffies(CONFIG_KCSAN_REPORT_ONCE_IN_MS);
// Check if a matching race report exists.
    while (i < REPORT_TIMES_SIZE) {
    let mut rt = &report_times[i];
//
// Must always select an entry for use to store info as we
// cannot resize report_times; at the end of the scan, use_entry
// will be the oldest entry, which ideally also happened before
// KCSAN_REPORT_ONCE_IN_MS ago.
//
    if (time_before(rt.time, use_entry.time)) {
    use_entry = rt;
    }
//
// Initially, no need to check any further as this entry as well
// as following entries have never been used.
//
    if (rt.time == 0) {
    break;
    }
// Check if entry expired.
    if (time_before(rt.time, invalid_before)) {
    continue; /* before KCSAN_REPORT_ONCE_IN_MS ago */
    }
// Reported recently, check if race matches.
    if ((rt.frame1 == frame1 && rt.frame2 == frame2) ||
    (rt.frame1 == frame2 && rt.frame2 == frame1)) {
    return true;
    }
    }
    use_entry.time = jiffies;
    use_entry.frame1 = frame1;
    use_entry.frame2 = frame2;
    return false;
    }
//
// Special rules to skip reporting.
//
#[no_mangle]
pub unsafe extern "C" fn skip_report(value_change: kcsan_value_change, top_frame: c_ulong) -> bool {
// Should never get here if value_change==FALSE.
    WARN_ON_ONCE!(value_change == KCSAN_VALUE_CHANGE_FALSE);
//
// The first call to skip_report always has value_change==TRUE, since we
// cannot know the value written of an instrumented access. For the 2nd
// call there are 6 cases with CONFIG_KCSAN_REPORT_VALUE_CHANGE_ONLY:
//
// 1. read watchpoint, conflicting write (value_change==TRUE): report;
// 2. read watchpoint, conflicting write (value_change==MAYBE): skip;
// 3. write watchpoint, conflicting write (value_change==TRUE): report;
// 4. write watchpoint, conflicting write (value_change==MAYBE): skip;
// 5. write watchpoint, conflicting read (value_change==MAYBE): skip;
// 6. write watchpoint, conflicting read (value_change==TRUE): report;
//
// Cases 1-4 are intuitive and expected; case 5 ensures we do not report
// data races where the write may have rewritten the same value; case 6
// is possible either if the size is larger than what we check value
// changes for or the access type is KCSAN_ACCESS_ASSERT.
//
    if (IS_ENABLED!(CONFIG_KCSAN_REPORT_VALUE_CHANGE_ONLY) &&
    value_change == KCSAN_VALUE_CHANGE_MAYBE) {
//
// The access is a write, but the data value did not change.
//
// We opt-out of this filter for certain functions at request of
// maintainers.
//
    char buf[64];
pub static mut len: c_int = 0;
    if (!strnstr(buf, "rcu_", len) &&
    !strnstr(buf, "_rcu", len) &&
    !strnstr(buf, "_srcu", len)) {
    return true;
    }
    }
    return kcsan_skip_report_debugfs(top_frame);
    }
    static const char *get_access_type(int type)
    {
    if (type & KCSAN_ACCESS_ASSERT) {
    if (type & KCSAN_ACCESS_SCOPED) {
    if (type & KCSAN_ACCESS_WRITE) {
    return "assert no accesses (reordered)";
    }
    else {
    return "assert no writes (reordered)";
    }
    } else {
    if (type & KCSAN_ACCESS_WRITE) {
    return "assert no accesses";
    }
    else {
    return "assert no writes";
    }
    }
    }
    match (type) {
    0 => {
    return "read";
    }
    KCSAN_ACCESS_ATOMIC => {
    return "read (marked)";
    }
    KCSAN_ACCESS_WRITE => {
    return "write";
    }
    KCSAN_ACCESS_WRITE | KCSAN_ACCESS_ATOMIC => {
    return "write (marked)";
    }
    KCSAN_ACCESS_COMPOUND | KCSAN_ACCESS_WRITE => {
    return "read-write";
    }
    KCSAN_ACCESS_COMPOUND | KCSAN_ACCESS_WRITE | KCSAN_ACCESS_ATOMIC => {
    return "read-write (marked)";
    }
    KCSAN_ACCESS_SCOPED => {
    return "read (reordered)";
    }
    KCSAN_ACCESS_SCOPED | KCSAN_ACCESS_ATOMIC => {
    return "read (marked, reordered)";
    }
    KCSAN_ACCESS_SCOPED | KCSAN_ACCESS_WRITE => {
    return "write (reordered)";
    }
    KCSAN_ACCESS_SCOPED | KCSAN_ACCESS_WRITE | KCSAN_ACCESS_ATOMIC => {
    return "write (marked, reordered)";
    }
    KCSAN_ACCESS_SCOPED | KCSAN_ACCESS_COMPOUND | KCSAN_ACCESS_WRITE => {
    return "read-write (reordered)";
    }
    KCSAN_ACCESS_SCOPED | KCSAN_ACCESS_COMPOUND | KCSAN_ACCESS_WRITE | KCSAN_ACCESS_ATOMIC => {
    return "read-write (marked, reordered)";
    }
    _ => {
    BUG();
    }
    }
    }
    static const char *get_bug_type(int type)
    {
    return (type & KCSAN_ACCESS_ASSERT) != 0 ? "assert: race" : "data-race";
    }
// Return thread description: in task or interrupt.
    static const char *get_thread_desc(int task_id)
    {
    if (task_id != -1) {
    static char buf[32]; /* safe: protected by report_lock */
    snprintf(buf, sizeof!(buf), "task %i", task_id);
    return buf;
    }
    return "interrupt";
    }
// Helper to skip KCSAN-related functions in stack-trace.
#[no_mangle]
unsafe extern "C" fn get_stack_skipnr(stack_entries[]: c_ulong, num_entries: c_int) -> c_int {
    char buf[64];
pub static mut cur: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    let mut skip = 0;
    while (skip < num_entries) {
    len = scnprintf(buf, sizeof!(buf), "%ps", stack_entries[skip]);
// Never show tsan_* or {read,write}_once_size.
    if (strnstr(buf, "tsan_", len) ||
    strnstr(buf, "_once_size", len)) {
    continue;
    }
    cur = strnstr(buf, "kcsan_", len);
    if (cur) {
    cur += strlen("kcsan_");
    if (!str_has_prefix(cur, "test")) {
    continue; /* KCSAN runtime function. */
    }
// KCSAN related test.
    }
//
// No match for runtime functions -- @skip entries to skip to
// get to first frame of interest.
//
    break;
    }
    return skip;
    }
//
// Skips to the first entry that matches the function of @ip, and then replaces
// that entry with @ip, returning the entries to skip with @replaced containing
// the replaced entry.
//
#[no_mangle]
pub unsafe extern "C" fn replace_stack_entry(num_entries: c_int, ip: c_ulong, replaced: *mut c_ulong) -> c_int {
    unsigned long symbolsize, offset;
    let mut target_func = 0;
    let mut skip = 0;
    if (kallsyms_lookup_size_offset(ip, &symbolsize, &offset)) {
    target_func = ip - offset;
    }
    else {
// goto;
    }
    while (skip < num_entries) {
pub static mut func: c_ulong = 0;
    if (!kallsyms_lookup_size_offset(func, &symbolsize, &offset)) {
// goto;
    }
    func -= offset;
    if (func == target_func) {
// replaced = stack_entries[skip];
    stack_entries[skip] = ip;
    return skip;
    }
    }
// label;
// Should not happen; the resulting stack trace is likely misleading.
    WARN_ONCE(1, "Cannot find frame for %pS in stack trace", ip);
    return get_stack_skipnr(stack_entries, num_entries);
    }
#[no_mangle]
pub unsafe extern "C" fn sanitize_stack_entries(num_entries: c_int, ip: c_ulong, replaced: *mut c_ulong) -> c_int {
    return ip ? replace_stack_entry(stack_entries, num_entries, ip, replaced) :
    get_stack_skipnr(stack_entries, num_entries);
    }
// Compares symbolized strings of addr1 and addr2.
#[no_mangle]
unsafe extern "C" fn sym_strcmp(addr1: *mut c_void, addr2: *mut c_void) -> c_int {
    char buf1[64];
    char buf2[64];
    snprintf(buf1, sizeof!(buf1), "%pS", addr1);
    snprintf(buf2, sizeof!(buf2), "%pS", addr2);
    return strncmp(buf1, buf2, sizeof!(buf1));
    }
#[no_mangle]
pub unsafe extern "C" fn print_stack_trace(num_entries: c_int, report_lock: unsigned long reordered_to)
    __must_hold(&) {
    stack_trace_print(stack_entries, num_entries, 0);
    if (reordered_to) {
    pr_err!("  |\n  +. reordered to: %pS\n", reordered_to);
    }
    }
#[no_mangle]
unsafe extern "C" fn print_verbose_info(task: *mut task_struct) {
    if (!task) {
    return;
    }
// Restore IRQ state trace for printing.
    kcsan_restore_irqtrace(task);
    pr_err!("\n");
    debug_show_held_locks(task);
    print_irqtrace_events(task);
    }
#[no_mangle]
pub unsafe extern "C" fn print_report(value_change: kcsan_value_change, ai: *mut access_info, other_info: *mut other_info, old: u64, new: u64, report_lock: u64 mask)
    __must_hold(&) {
pub static mut reordered_to: c_ulong = 0;
    unsigned long stack_entries[NUM_STACK_ENTRIES] = { 0 };
pub static mut num_stack_entries: c_int = 0;
pub static mut skipnr: c_int = 0;
pub static mut this_frame: c_ulong = 0;
pub static mut other_reordered_to: c_ulong = 0;
pub static mut other_frame: c_ulong = 0;
    let mut other_skipnr = 0; /* silence uninit warnings */
//
// Must check report filter rules before starting to print.
//
    if (skip_report(KCSAN_VALUE_CHANGE_TRUE, stack_entries[skipnr])) {
    return;
    }
    if (other_info) {
    other_skipnr = sanitize_stack_entries(other_info.stack_entries,
    other_info.num_stack_entries,
    other_info.ai.ip, &other_reordered_to);
    other_frame = other_info.stack_entries[other_skipnr];
// @value_change is only known for the other thread
    if (skip_report(value_change, other_frame)) {
    return;
    }
    }
    if (rate_limit_report(this_frame, other_frame)) {
    return;
    }
// Print report header.
    pr_err!("==================================================================\n");
    if (other_info) {
    let mut cmp = 0;
//
// Order functions lexographically for consistent bug titles.
// Do not print offset of functions to keep title short.
//
    cmp = sym_strcmp(other_frame, this_frame);
    pr_err!("BUG: KCSAN: %s in %ps / %ps\n",
    get_bug_type(ai.access_type | other_info.ai.access_type),
    (cmp < 0 ? other_frame : this_frame),
    (cmp < 0 ? this_frame : other_frame));
    } else {
    pr_err!("BUG: KCSAN: %s in %pS\n", get_bug_type(ai.access_type),
    this_frame);
    }
    pr_err!("\n");
// Print information about the racing accesses.
    if (other_info) {
    pr_err!("%s to 0x%px of %zu bytes by %s on cpu %i:\n",
    get_access_type(other_info.ai.access_type), other_info.ai.ptr,
    other_info.ai.size, get_thread_desc(other_info.ai.task_pid),
    other_info.ai.cpu_id);
// Print the other thread's stack trace.
    print_stack_trace(other_info.stack_entries + other_skipnr,
    other_info.num_stack_entries - other_skipnr,
    other_reordered_to);
    if (IS_ENABLED!(CONFIG_KCSAN_VERBOSE)) {
    print_verbose_info(other_info.task);
    }
    pr_err!("\n");
    pr_err!("%s to 0x%px of %zu bytes by %s on cpu %i:\n",
    get_access_type(ai.access_type), ai.ptr, ai.size,
    get_thread_desc(ai.task_pid), ai.cpu_id);
    } else {
    pr_err!("race at unknown origin, with %s to 0x%px of %zu bytes by %s on cpu %i:\n",
    get_access_type(ai.access_type), ai.ptr, ai.size,
    get_thread_desc(ai.task_pid), ai.cpu_id);
    }
// Print stack trace of this thread.
    print_stack_trace(stack_entries + skipnr, num_stack_entries - skipnr, reordered_to);
    if (IS_ENABLED!(CONFIG_KCSAN_VERBOSE)) {
    print_verbose_info(current);
    }
// Print observed value change.
    if (ai.size <= 8) {
pub static mut hex_len: c_int = 0;
pub static mut diff: u64 = 0;
    if (mask) {
    diff &= mask;
    }
    if (diff) {
    pr_err!("\n");
    pr_err!("value changed: 0x%0*llx . 0x%0*llx\n",
    hex_len, old, hex_len, new);
    if (mask) {
    pr_err!(" bits changed: 0x%0*llx with mask 0x%0*llx\n",
    hex_len, diff, hex_len, mask);
    }
    }
    }
// Print report footer.
    pr_err!("\n");
    pr_err!("Reported by Kernel Concurrency Sanitizer on:\n");
    dump_stack_print_info(KERN_DEFAULT);
    pr_err!("==================================================================\n");
    check_panic_on_warn("KCSAN");
    }
#[no_mangle]
unsafe extern "C" fn release_report(flags: *mut c_ulong, other_info: *mut other_info) {
//
// Use size to denote valid/invalid, since KCSAN entirely ignores
// 0-sized accesses.
//
    other_info.ai.size = 0;
    raw_spin_unlock_irqrestore(&report_lock, *flags);
    }
//
// Sets @other_info->task and awaits consumption of @other_info.
//
#[no_mangle]
pub unsafe extern "C" fn set_other_info_task_blocking(flags: *mut c_ulong, ai: *mut access_info, report_lock: *mut other_infoother_info)
    __must_hold(&) {
//
// We may be instrumenting a code-path where current->state is already
// something other than TASK_RUNNING.
//
pub static mut is_running: bool = false;
//
// To avoid deadlock in case we are in an interrupt here and this is a
// race with a task on the same CPU (KCSAN_INTERRUPT_WATCHER), provide a
// timeout to ensure this works in all contexts.
//
// Await approximately the worst case delay of the reporting thread (if
// we are not interrupted).
//
pub static mut timeout: c_int = 0;
    other_info.task = current;
    do {
    if (is_running) {
//
// Let lockdep know the real task is sleeping, to print
// the held locks (recall we turned lockdep off, so
// locking/unlocking @report_lock won't be recorded).
//
    set_current_state(TASK_UNINTERRUPTIBLE);
    }
    raw_spin_unlock_irqrestore(&report_lock, *flags);
//
// We cannot call schedule() since we also cannot reliably
// determine if sleeping here is permitted -- see in_atomic().
//
    udelay(1);
    raw_spin_lock_irqsave(&report_lock, *flags);
    if (timeout-- < 0) {
//
// Abort. Reset @other_info->task to NULL, since it
// appears the other thread is still going to consume
// it. It will result in no verbose info printed for
// this task.
//
    other_info.task = core::ptr::null_mut();
    break;
    }
//
// If invalid, or @ptr nor @current matches, then @other_info
// has been consumed and we may continue. If not, retry.
//
    } while (other_info.ai.size && other_info.ai.ptr == ai.ptr &&
    other_info.task == current);
    if (is_running) {
    set_current_state(TASK_RUNNING);
    }
    }
// Populate @other_info; requires that the provided @other_info not in use.
#[no_mangle]
pub unsafe extern "C" fn prepare_report_producer(flags: *mut c_ulong, ai: *mut access_info, report_lock: *mut other_infoother_info)
    __must_not_hold(&) {
    raw_spin_lock_irqsave(&report_lock, *flags);
//
// The same @other_infos entry cannot be used concurrently, because
// there is a one-to-one mapping to watchpoint slots (@watchpoints in
// core.c), and a watchpoint is only released for reuse after reporting
// is done by the consumer of @other_info. Therefore, it is impossible
// for another concurrent prepare_report_producer() to set the same
// @other_info, and are guaranteed exclusivity for the @other_infos
// entry pointed to by @other_info.
//
// To check this property holds, size should never be non-zero here,
// because every consumer of struct other_info resets size to 0 in
// release_report().
//
    WARN_ON!(other_info.ai.size);
    other_info.ai = *ai;
    other_info.num_stack_entries = stack_trace_save(other_info.stack_entries, NUM_STACK_ENTRIES, 2);
    if (IS_ENABLED!(CONFIG_KCSAN_VERBOSE)) {
    set_other_info_task_blocking(flags, ai, other_info);
    }
    raw_spin_unlock_irqrestore(&report_lock, *flags);
    }
// Awaits producer to fill @other_info and then returns.
#[no_mangle]
pub unsafe extern "C" fn prepare_report_consumer(flags: *mut c_ulong, ai: *mut access_info, true: *mut other_infoother_info)
    __cond_acquires(, report_lock: &) -> bool {
    raw_spin_lock_irqsave(&report_lock, *flags);
    while (!other_info.ai.size) { /* Await valid @other_info. */ {
    raw_spin_unlock_irqrestore(&report_lock, *flags);
    }
    cpu_relax();
    raw_spin_lock_irqsave(&report_lock, *flags);
    }
// Should always have a matching access based on watchpoint encoding.
    if (WARN_ON!(!matching_access((unsigned long)other_info.ai.ptr & WATCHPOINT_ADDR_MASK, other_info.ai.size,
    (unsigned long)ai.ptr & WATCHPOINT_ADDR_MASK, ai.size))) {
// goto;
    }
    if (!matching_access((unsigned long)other_info.ai.ptr, other_info.ai.size,
    (unsigned long)ai.ptr, ai.size)) {
//
// If the actual accesses to not match, this was a false
// positive due to watchpoint encoding.
//
    atomic_long_inc(&kcsan_counters[KCSAN_COUNTER_ENCODING_FALSE_POSITIVES]);
// goto;
    }
    return true;
// label;
    release_report(flags, other_info);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn prepare_access_info(ptr: *mut c_void, size: size_t, access_type: c_int, ip: c_ulong) {
    return (access_info) {
    .ptr		= ptr,
    .size		= size,
    .access_type	= access_type,
    .task_pid	= in_task() ? task_pid_nr(current) : -1,
    .cpu_id		= raw_smp_processor_id(),
// Only replace stack entry with @ip if scoped access.
    .ip		= (access_type & KCSAN_ACCESS_SCOPED) ? ip : 0,
    };
    }
#[no_mangle]
pub unsafe extern "C" fn kcsan_report_set_info(ptr: *mut c_void, size: size_t, access_type: c_int, ip: c_ulong, watchpoint_idx: c_int) {
pub static mut ai: access_info = 0;
    let mut flags = 0;
    kcsan_disable_current();
    lockdep_off(); /* See kcsan_report_known_origin(). */
    prepare_report_producer(&flags, &ai, &other_infos[watchpoint_idx]);
    lockdep_on();
    kcsan_enable_current();
    }
#[no_mangle]
pub unsafe extern "C" fn kcsan_report_known_origin(ptr: *mut c_void, size: size_t, access_type: c_int, ip: c_ulong, value_change: kcsan_value_change, watchpoint_idx: c_int, old: u64, new: u64, mask: u64) {
pub static mut ai: access_info = 0;
    let mut other_info = &other_infos[watchpoint_idx];
pub static mut flags: c_ulong = 0;
    kcsan_disable_current();
//
// Because we may generate reports when we're in scheduler code, the use
// of printk() could deadlock. Until such time that all printing code
// called in print_report() is scheduler-safe, accept the risk, and just
// get our message out. As such, also disable lockdep to hide the
// warning, and avoid disabling lockdep for the rest of the kernel.
//
    lockdep_off();
    if (!prepare_report_consumer(&flags, &ai, other_info)) {
// goto;
    }
//
// Never report if value_change is FALSE, only when it is
// either TRUE or MAYBE. In case of MAYBE, further filtering may
// be done once we know the full stack trace in print_report().
//
    if (value_change != KCSAN_VALUE_CHANGE_FALSE) {
    print_report(value_change, &ai, other_info, old, new, mask);
    }
    release_report(&flags, other_info);
// label;
    lockdep_on();
    kcsan_enable_current();
    }
#[no_mangle]
pub unsafe extern "C" fn kcsan_report_unknown_origin(ptr: *mut c_void, size: size_t, access_type: c_int, ip: c_ulong, old: u64, new: u64, mask: u64) {
pub static mut ai: access_info = 0;
    let mut flags = 0;
    kcsan_disable_current();
    lockdep_off(); /* See kcsan_report_known_origin(). */
    raw_spin_lock_irqsave(&report_lock, flags);
    print_report(KCSAN_VALUE_CHANGE_TRUE, &ai, core::ptr::null_mut(), old, new, mask);
    raw_spin_unlock_irqrestore(&report_lock, flags);
    lockdep_on();
    kcsan_enable_current();
    }
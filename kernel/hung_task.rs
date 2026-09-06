//! Automatically rewritten from C to Rust
//! Source: kernel/hung_task.c
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
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

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
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-only
//
// Detect Hung Task
//
// kernel/hung_task.c - kernel thread for detecting tasks stuck in D state
//

//
// The number of tasks checked:
//
pub static mut sysctl_hung_task_check_count: int  = 0;
//
// Total number of tasks detected as hung since boot:
//
pub static mut sysctl_hung_task_detect_count: atomic_long_t = 0;
//
// Limit number of tasks checked in a batch.
//
// This value controls the preemptibility of khungtaskd since preemption
// is disabled during the critical section. It also controls the size of
// the RCU grace period. So it needs to be upper-bound.
//

//
// Zero means infinite timeout - no checking done:
//
pub static mut sysctl_hung_task_timeout_secs: unsigned long  = 0;
//
// Zero (default value) means use sysctl_hung_task_timeout_secs:
//
    static unsigned long  sysctl_hung_task_check_interval_secs;
pub static mut sysctl_hung_task_warnings: int  = 10;
    static int  did_panic;
    static bool hung_task_call_panic;
pub static mut watchdog_task: *mut c_void = core::ptr::null_mut();
//
// A bitmask to control what kinds of system info to be printed when
// a hung task is detected, it could be task, memory, lock etc. Refer
// include/linux/sys_info.h for detailed bit definition.
//
    static unsigned long hung_task_si_mask;

//
// Should we dump all CPUs backtraces in a hung task event?
// Defaults to 0, can be changed via sysctl.
//
    static unsigned int  sysctl_hung_task_all_cpu_backtrace;

pub const sysctl_hung_task_all_cpu_backtrace: c_int = 0;

//
// Should we panic (and reboot, if panic_timeout= is set) when a
// hung task is detected:
//
    static unsigned int  sysctl_hung_task_panic =
    CONFIG_BOOTPARAM_HUNG_TASK_PANIC;
#[no_mangle]
pub unsafe extern "C" fn hung_task_panic() {
    did_panic = 1;
    return NOTIFY_DONE;
    }
pub static mut notifier_block: usize = 0;
#[no_mangle]
unsafe extern "C" fn task_is_hung(t: *mut task_struct, timeout: c_ulong) -> bool {
pub static mut switch_count: c_ulong = 0;
pub static mut state: c_uint = 0;
//
// skip the TASK_KILLABLE tasks -- these can be killed
// skip the TASK_IDLE tasks -- those are genuinely idle
// skip the TASK_FROZEN task -- it reasonably stops scheduling by freezer
//
    if (!(state & TASK_UNINTERRUPTIBLE) ||
    (state & (TASK_WAKEKILL | TASK_NOLOAD | TASK_FROZEN))) {
    return false;
    }
//
// When a freshly created task is scheduled once, changes its state to
// TASK_UNINTERRUPTIBLE without having ever been switched out once, it
// musn't be checked.
//
    if (unlikely(!switch_count)) {
    return false;
    }
    if (switch_count != t.last_switch_count) {
    t.last_switch_count = switch_count;
    t.last_switch_time = jiffies;
    return false;
    }
    if (time_is_after_jiffies(t.last_switch_time + timeout * HZ)) {
    return false;
    }
    return true;
    }

#[no_mangle]
unsafe extern "C" fn debug_show_blocker(task: *mut task_struct, timeout: c_ulong) {
    let mut g = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    unsigned long owner, blocker, blocker_type;
    let mut rwsem_blocked_by = core::ptr::null_mut();
    let mut rwsem_blocked_as = core::ptr::null_mut();
// RCU_LOCKDEP_WARN;
    blocker = READ_ONCE(task.blocker);
    if (!blocker) {
    return;
    }
    blocker_type = hung_task_get_blocker_type(blocker);
    match (blocker_type) {
    BLOCKER_TYPE_MUTEX => {
    owner = mutex_get_owner(hung_task_blocker_to_lock(blocker));
    // break;
    }
    BLOCKER_TYPE_SEM => {
    owner = sem_last_holder(hung_task_blocker_to_lock(blocker));
    // break;
    }
    BLOCKER_TYPE_RWSEM_READER => {
    }
    BLOCKER_TYPE_RWSEM_WRITER => {
    owner = (unsigned long)rwsem_owner(
    hung_task_blocker_to_lock(blocker));
    rwsem_blocked_as = (blocker_type == BLOCKER_TYPE_RWSEM_READER) ?
    "reader" : "writer";
    rwsem_blocked_by = is_rwsem_reader_owned(
    hung_task_blocker_to_lock(blocker)) ?
    "reader" : "writer";
    // break;
    }
    _ => {
// WARN_ON_ONCE;
    return;
    }
    }
    if (unlikely(!owner)) {
    match (blocker_type) {
    BLOCKER_TYPE_MUTEX => {
    pr_err!("INFO: task %s:%d is blocked on a mutex, but the owner is not found.\n",
    task.comm, task.pid);
    // break;
    }
    BLOCKER_TYPE_SEM => {
    pr_err!("INFO: task %s:%d is blocked on a semaphore, but the last holder is not found.\n",
    task.comm, task.pid);
    // break;
    }
    BLOCKER_TYPE_RWSEM_READER => {
    }
    BLOCKER_TYPE_RWSEM_WRITER => {
    pr_err!("INFO: task %s:%d is blocked on an rw-semaphore, but the owner is not found.\n",
    task.comm, task.pid);
    // break;
    }
    }
    return;
    }
// Ensure the owner information is correct.
    for_each_process_thread(g, t) {
    if ((unsigned long)t != owner) {
    continue;
    }
    match (blocker_type) {
    BLOCKER_TYPE_MUTEX => {
    pr_err!("INFO: task %s:%d is blocked on a mutex likely owned by task %s:%d.\n",
    task.comm, task.pid, t.comm, t.pid);
    // break;
    }
    BLOCKER_TYPE_SEM => {
    pr_err!("INFO: task %s:%d blocked on a semaphore likely last held by task %s:%d\n",
    task.comm, task.pid, t.comm, t.pid);
    // break;
    }
    BLOCKER_TYPE_RWSEM_READER => {
    }
    BLOCKER_TYPE_RWSEM_WRITER => {
    pr_err!("INFO: task %s:%d <%s> blocked on an rw-semaphore likely owned by task %s:%d <%s>\n",
    task.comm, task.pid, rwsem_blocked_as, t.comm,
    t.pid, rwsem_blocked_by);
    // break;
    }
    }
// Avoid duplicated task dump, skip if the task is also hung.
    if (!task_is_hung(t, timeout)) {
    sched_show_task(t);
    }
    return;
    }
    }

#[no_mangle]
pub unsafe extern "C" fn debug_show_blocker(task: *mut task_struct, timeout: c_ulong) {
    }

//
// hung_task_info - Print diagnostic details for a hung task
// @t: Pointer to the detected hung task.
// @timeout: Timeout threshold for detecting hung tasks
// @this_round_count: Count of hung tasks detected in the current iteration
//
// Print structured information about the specified hung task, if warnings
// are enabled or if the panic batch threshold is exceeded.
//
#[no_mangle]
pub unsafe extern "C" fn hung_task_info() {
    trace_sched_process_hang(t);
    if (sysctl_hung_task_panic && this_round_count >= sysctl_hung_task_panic) {
    console_verbose();
    hung_task_call_panic = true;
    }
//
// The given task did not get scheduled for more than
// CONFIG_DEFAULT_HUNG_TASK_TIMEOUT. Therefore, complain
// accordingly
//
    if (sysctl_hung_task_warnings || hung_task_call_panic) {
    if (sysctl_hung_task_warnings > 0) {
    sysctl_hung_task_warnings -= 1;
    }
    pr_err!("INFO: task %s:%d blocked%s for more than %ld seconds.\n",
    t.comm, t.pid, t.in_iowait ? " in I/O wait" : "",
    (jiffies - t.last_switch_time) / HZ);
    pr_err!("      %s %s %.*s\n",
    print_tainted(), init_utsname().release,
    (int)strcspn(init_utsname().version, " "),
    init_utsname().version);
    if (t.flags & PF_POSTCOREDUMP) {
    pr_err!("      Blocked by coredump.\n");
    }
    pr_err!("\"echo 0 > /proc/sys/kernel/hung_task_timeout_secs\""
    " disables this message.\n");
    sched_show_task(t);
    debug_show_blocker(t, timeout);
    if (!sysctl_hung_task_warnings) {
    pr_info!("Future hung task reports are suppressed, see sysctl kernel.hung_task_warnings\n");
    }
    }
    touch_nmi_watchdog();
    }
//
// To avoid extending the RCU grace period for an unbounded amount of time,
// periodically exit the critical section and enter a new one.
//
// For preemptible RCU it is sufficient to call rcu_read_unlock in order
// to exit the grace period. For classic RCU, a reschedule is required.
//
#[no_mangle]
unsafe extern "C" fn rcu_lock_break(g: *mut task_struct, t: *mut task_struct) -> bool {
    let mut can_cont = 0;
    get_task_struct(g);
    get_task_struct(t);
    rcu_read_unlock();
    cond_resched();
    rcu_read_lock();
    can_cont = pid_alive(g) && pid_alive(t);
    put_task_struct(t);
    put_task_struct(g);
    return can_cont;
    }
//
// Check whether a TASK_UNINTERRUPTIBLE does not get woken up for
// a really long time. If that happens, print out a warning.
//
#[no_mangle]
unsafe extern "C" fn check_hung_uninterruptible_tasks(timeout: c_ulong) {
pub static mut max_count: c_int = 0;
pub static mut last_break: c_ulong = 0;
    let mut g = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    let mut this_round_count = 0;
pub static mut need_warning: c_int = 0;
pub static mut si_mask: c_ulong = 0;
//
// If the system crashed already then all bets are off,
// do not report extra hung tasks:
//
    if (test_taint(TAINT_DIE) || did_panic) {
    return;
    }
    this_round_count = 0;
    rcu_read_lock();
    for_each_process_thread(g, t) {
    if (!max_count--) {
// goto;
    }
    if (time_after(jiffies, last_break + HUNG_TASK_LOCK_BREAK)) {
    if (!rcu_lock_break(g, t)) {
// goto;
    }
    last_break = jiffies;
    }
    if (task_is_hung(t, timeout)) {
//
// Increment the global counter so that userspace could
// start migrating tasks ASAP. But count the current
// round separately because userspace could reset
// the global counter at any time.
//
    atomic_long_inc(&sysctl_hung_task_detect_count);
    this_round_count += 1;
    hung_task_info(t, timeout, this_round_count);
    }
    }
// label;
    rcu_read_unlock();
    if (!this_round_count) {
    return;
    }
    if (need_warning || hung_task_call_panic) {
    si_mask |= SYS_INFO_LOCKS;
    if (sysctl_hung_task_all_cpu_backtrace) {
    si_mask |= SYS_INFO_ALL_BT;
    }
    }
    sys_info(si_mask);
    if (hung_task_call_panic) {
    panic("hung_task: blocked tasks");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hung_timeout_jiffies() {
// timeout of 0 will disable the watchdog
    return timeout ? last_checked - jiffies + timeout * HZ :
    MAX_SCHEDULE_TIMEOUT;
    }

//
// proc_dohung_task_detect_count - proc handler for hung_task_detect_count
// @table: Pointer to the struct ctl_table definition for this proc entry
// @dir: Flag indicating the operation
// @buffer: User space buffer for data transfer
// @lenp: Pointer to the length of the data being transferred
// @ppos: Pointer to the current file offset
//
// This handler is used for reading the current hung task detection count
// and for resetting it to zero when a write operation is performed using a
// zero value only.
// Return: 0 on success, or a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn proc_dohung_task_detect_count() {
    let mut detect_count = 0;
    let mut proxy_table;
    let mut err = 0;
    proxy_table = *table;
    proxy_table.data = &detect_count;
    if (SYSCTL_KERN_TO_USER(dir)) {
    detect_count = atomic_long_read(&sysctl_hung_task_detect_count);
    }
    err = proc_doulongvec_minmax(&proxy_table, dir, buffer, lenp, ppos);
    if (err < 0) {
    return err;
    }
    if (SYSCTL_USER_TO_KERN(dir)) {
    if (detect_count) {
    return -EINVAL;
    }
    atomic_long_set(&sysctl_hung_task_detect_count, 0);
    }
    return 0;
    }
//
// Process updating of timeout sysctl
//
#[no_mangle]
pub unsafe extern "C" fn proc_dohung_task_timeout_secs() {
    let mut ret = 0;
    ret = proc_doulongvec_minmax(table, write, buffer, lenp, ppos);
    if (ret || !write) {
// goto;
    }
    wake_up_process(watchdog_task);
// label;
    return ret;
    }
//
// This is needed for proc_doulongvec_minmax of sysctl_hung_task_timeout_secs
// and hung_task_check_interval_secs
//
pub static mut hung_task_timeout_max: unsigned long = 0;
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn hung_task_sysctl_init() -> c_int {
    register_sysctl_init("kernel", hung_task_sysctls);
    }

pub static mut reset_hung_task: atomic_t = 0;
#[no_mangle]
pub unsafe extern "C" fn reset_hung_task_detector() {
    atomic_set(&reset_hung_task, 1);
    }
// EXPORT_SYMBOL_GPL;
    static bool hung_detector_suspended;
#[no_mangle]
pub unsafe extern "C" fn hungtask_pm_notify() {
    match (action) {
    PM_SUSPEND_PREPARE => {
    }
    PM_HIBERNATION_PREPARE => {
    }
    PM_RESTORE_PREPARE => {
    hung_detector_suspended = true;
    // break;
    }
    PM_POST_SUSPEND => {
    }
    PM_POST_HIBERNATION => {
    }
    PM_POST_RESTORE => {
    hung_detector_suspended = false;
    // break;
    }
    _ => {
    // break;
    }
    }
    return NOTIFY_OK;
    }
//
// kthread which checks for tasks stuck in D state
//
#[no_mangle]
unsafe extern "C" fn watchdog(dummy: *mut c_void) -> c_int {
pub static mut hung_last_checked: c_ulong = 0;
    set_user_nice(current, 0);
    while ( ) {
pub static mut timeout: c_ulong = 0;
pub static mut interval: c_ulong = 0;
    let mut t = 0;
    if (interval == 0) {
    interval = timeout;
    }
    interval = min_t(unsigned long, interval, timeout);
    t = hung_timeout_jiffies(hung_last_checked, interval);
    if (t <= 0) {
    if (!atomic_xchg(&reset_hung_task, 0) &&
    !hung_detector_suspended) {
    check_hung_uninterruptible_tasks(timeout);
    }
    hung_last_checked = jiffies;
    continue;
    }
    schedule_timeout_interruptible(t);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hung_task_init() -> c_int {
    atomic_notifier_chain_register(&panic_notifier_list, &panic_block);
// Disable hung task detector on suspend
    pm_notifier(hungtask_pm_notify, 0);
    watchdog_task = kthread_run(watchdog, core::ptr::null_mut(), "khungtaskd");
    hung_task_sysctl_init();
    return 0;
    }
// subsys_initcall;
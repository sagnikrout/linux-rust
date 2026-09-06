//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_sched_switch.c
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
// trace context switch
//
// Copyright (C) 2007 Steven Rostedt <srostedt@redhat.com>
//

pub const RECORD_CMDLINE: c_int = 1;
pub const RECORD_TGID: c_int = 2;
    static int		sched_cmdline_ref;
    static int		sched_tgid_ref;
pub static mut sched_register_mutex: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn probe_sched_switch(ignore: *mut c_void, preempt: bool, prev: *mut task_struct, next: *mut task_struct, prev_state: c_uint) {
    let mut flags = 0;
    flags = (RECORD_TGID * !!sched_tgid_ref) +
    (RECORD_CMDLINE * !!sched_cmdline_ref);
    if (!flags) {
    return;
    }
    tracing_record_taskinfo_sched_switch(prev, next, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn probe_sched_wakeup(ignore: *mut c_void, wakee: *mut task_struct) {
    let mut flags = 0;
    flags = (RECORD_TGID * !!sched_tgid_ref) +
    (RECORD_CMDLINE * !!sched_cmdline_ref);
    if (!flags) {
    return;
    }
    tracing_record_taskinfo_sched_switch(current, wakee, flags);
    }
#[no_mangle]
unsafe extern "C" fn tracing_sched_register() -> c_int {
    let mut ret = 0;
    ret = register_trace_sched_wakeup(probe_sched_wakeup, core::ptr::null_mut());
    if (ret) {
    pr_info!("wakeup trace: Couldn't activate tracepoint"
    " probe to kernel_sched_wakeup\n");
    return ret;
    }
    ret = register_trace_sched_wakeup_new(probe_sched_wakeup, core::ptr::null_mut());
    if (ret) {
    pr_info!("wakeup trace: Couldn't activate tracepoint"
    " probe to kernel_sched_wakeup_new\n");
// goto;
    }
    ret = register_trace_sched_switch(probe_sched_switch, core::ptr::null_mut());
    if (ret) {
    pr_info!("sched trace: Couldn't activate tracepoint"
    " probe to kernel_sched_switch\n");
// goto;
    }
    return ret;
// label;
    unregister_trace_sched_wakeup_new(probe_sched_wakeup, core::ptr::null_mut());
// label;
    unregister_trace_sched_wakeup(probe_sched_wakeup, core::ptr::null_mut());
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tracing_sched_unregister() {
    unregister_trace_sched_switch(probe_sched_switch, core::ptr::null_mut());
    unregister_trace_sched_wakeup_new(probe_sched_wakeup, core::ptr::null_mut());
    unregister_trace_sched_wakeup(probe_sched_wakeup, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn tracing_start_sched_switch(ops: c_int) {
    let mut sched_register = 0;
    mutex_lock(&sched_register_mutex);
    sched_register = (!sched_cmdline_ref && !sched_tgid_ref);
    match (ops) {
    RECORD_CMDLINE => {
    sched_cmdline_ref += 1;
    // break;
    }
    RECORD_TGID => {
    sched_tgid_ref += 1;
    // break;
    }
    }
    if (sched_register && (sched_cmdline_ref || sched_tgid_ref)) {
    tracing_sched_register();
    }
    mutex_unlock(&sched_register_mutex);
    }
#[no_mangle]
unsafe extern "C" fn tracing_stop_sched_switch(ops: c_int) {
    mutex_lock(&sched_register_mutex);
    match (ops) {
    RECORD_CMDLINE => {
    sched_cmdline_ref -= 1;
    // break;
    }
    RECORD_TGID => {
    sched_tgid_ref -= 1;
    // break;
    }
    }
    if (!sched_cmdline_ref && !sched_tgid_ref) {
    tracing_sched_unregister();
    }
    mutex_unlock(&sched_register_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_start_cmdline_record() {
    tracing_start_sched_switch(RECORD_CMDLINE);
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_stop_cmdline_record() {
    tracing_stop_sched_switch(RECORD_CMDLINE);
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_start_tgid_record() {
    tracing_start_sched_switch(RECORD_TGID);
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_stop_tgid_record() {
    tracing_stop_sched_switch(RECORD_TGID);
    }
//
// The tgid_map array maps from pid to tgid; i.e. the value stored at index i
// is the tgid last observed corresponding to pid=i.
//
pub static mut tgid_map: *mut c_void = core::ptr::null_mut();
// The maximum valid index into tgid_map.
    static size_t tgid_map_max;
pub const SAVED_CMDLINES_DEFAULT: c_int = 128;

//
// Preemption must be disabled before acquiring trace_cmdline_lock.
// The various trace_arrays' max_lock must be acquired in a context
// where interrupt is disabled.
//
pub static mut trace_cmdline_lock: arch_spinlock_t = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saved_cmdlines_buffer {
    pub map_pid_to_cmdline: [unsigned; PID_MAX_DEFAULT+1],
    pub map_cmdline_to_pid: *mut unsigned,
    pub cmdline_num: unsigned,
    pub cmdline_idx: c_int,
    pub saved_cmdlines: [c_char; 0],
}

pub static mut savedcmd: *mut c_void = core::ptr::null_mut();
// Holds the size of a cmdline and pid element

    (TASK_COMM_LEN + sizeof!((s).map_cmdline_to_pid[0]))
#[no_mangle]
pub unsafe extern "C" fn get_saved_cmdlines(idx: c_int) -> *mut c_void {
    return &savedcmd.saved_cmdlines[idx * TASK_COMM_LEN];
    }
#[no_mangle]
pub unsafe extern "C" fn set_cmdline(idx: c_int, cmdline: *const c_char) {
    strscpy(get_saved_cmdlines(idx), cmdline, TASK_COMM_LEN);
    }
#[no_mangle]
unsafe extern "C" fn free_saved_cmdlines_buffer(s: *mut saved_cmdlines_buffer) {
pub static mut order: c_int = 0;
    kmemleak_free(s);
    free_pages((unsigned long)s, order);
    }
#[no_mangle]
pub unsafe extern "C" fn allocate_cmdlines_buffer(val: c_uint) -> *mut c_void {
pub static mut s: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut orig_size = 0;
    let mut size = 0;
    let mut order = 0;
// Figure out how much is needed to hold the given number of cmdlines
    orig_size = sizeof!(*s) + val * SAVED_CMDLINE_MAP_ELEMENT_SIZE(s);
    order = get_order(orig_size);
    size = 1 << (order + PAGE_SHIFT);
    page = alloc_pages(GFP_KERNEL, order);
    if (!page) {
    return core::ptr::null_mut();
    }
    s = page_address(page);
    kmemleak_alloc(s, size, 1, GFP_KERNEL);
    memset(s, 0, sizeof!(*s));
// Round up to actual allocation
    val = (size - sizeof!(*s)) / SAVED_CMDLINE_MAP_ELEMENT_SIZE(s);
    s.cmdline_num = val;
// Place map_cmdline_to_pid array right after saved_cmdlines
    s.map_cmdline_to_pid = &s.saved_cmdlines[val * TASK_COMM_LEN];
    memset(&s.map_pid_to_cmdline, NO_CMDLINE_MAP,
    sizeof!(s.map_pid_to_cmdline));
    memset(s.map_cmdline_to_pid, NO_CMDLINE_MAP,
    val * sizeof!(*s.map_cmdline_to_pid));
    return s;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_create_savedcmd() -> c_int {
    savedcmd = allocate_cmdlines_buffer(SAVED_CMDLINES_DEFAULT);
    return savedcmd ? 0 : -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_save_cmdline(tsk: *mut task_struct) -> c_int {
    let mut tpid = 0;
    let mut idx = 0;
// treat recording of idle task as a success
    if (!tsk.pid) {
    return 1;
    }
    BUILD_BUG_ON!(!is_power_of_2(PID_MAX_DEFAULT));
    tpid = tsk.pid & (PID_MAX_DEFAULT - 1);
//
// It's not the end of the world if we don't get
// the lock, but we also don't want to spin
// nor do we want to disable interrupts,
// so if we miss here, then better luck next time.
//
// This is called within the scheduler and wake up, so interrupts
// had better been disabled and run queue lock been held.
//
    lockdep_assert_preemption_disabled();
    if (!arch_spin_trylock(&trace_cmdline_lock)) {
    return 0;
    }
    idx = savedcmd.map_pid_to_cmdline[tpid];
    if (idx == NO_CMDLINE_MAP) {
    idx = (savedcmd.cmdline_idx + 1) % savedcmd.cmdline_num;
    savedcmd.map_pid_to_cmdline[tpid] = idx;
    savedcmd.cmdline_idx = idx;
    }
    savedcmd.map_cmdline_to_pid[idx] = tsk.pid;
    set_cmdline(idx, tsk.comm);
    arch_spin_unlock(&trace_cmdline_lock);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn __trace_find_cmdline(pid: c_int, comm[]: c_char) {
    let mut map: c_uint = 0;
    let mut tpid = 0;
    if (!pid) {
    strscpy(comm, "<idle>", TASK_COMM_LEN);
    return;
    }
    if (WARN_ON_ONCE!(pid < 0)) {
    strscpy(comm, "<XXX>", TASK_COMM_LEN);
    return;
    }
    tpid = pid & (PID_MAX_DEFAULT - 1);
    map = savedcmd.map_pid_to_cmdline[tpid];
    if (map != NO_CMDLINE_MAP) {
    tpid = savedcmd.map_cmdline_to_pid[map];
    if (tpid == pid) {
    strscpy(comm, get_saved_cmdlines(map), TASK_COMM_LEN);
    return;
    }
    }
    strscpy(comm, "<...>", TASK_COMM_LEN);
    }
#[no_mangle]
pub unsafe extern "C" fn trace_find_cmdline(pid: c_int, comm[]: c_char) {
    preempt_disable();
    arch_spin_lock(&trace_cmdline_lock);
    __trace_find_cmdline(pid, comm);
    arch_spin_unlock(&trace_cmdline_lock);
    preempt_enable();
    }
#[no_mangle]
pub unsafe extern "C" fn trace_find_tgid_ptr(pid: c_int) -> *mut c_void {
//
// Pairs with the smp_store_release in set_tracer_flag() to ensure that
// if we observe a non-NULL tgid_map then we also observe the correct
// tgid_map_max.
//
    let mut map = smp_load_acquire(&tgid_map);
    if (unlikely(!map || pid > tgid_map_max)) {
    return core::ptr::null_mut();
    }
    return &map[pid];
    }
#[no_mangle]
pub unsafe extern "C" fn trace_find_tgid(pid: c_int) -> c_int {
    let mut ptr = trace_find_tgid_ptr(pid);
    return ptr ? *ptr : 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_save_tgid(tsk: *mut task_struct) -> c_int {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
// treat recording of idle task as a success
    if (!tsk.pid) {
    return 1;
    }
    ptr = trace_find_tgid_ptr(tsk.pid);
    if (!ptr) {
    return 0;
    }
// ptr = tsk->tgid;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn tracing_record_taskinfo_skip(flags: c_int) -> bool {
    if (unlikely(!(flags & (TRACE_RECORD_CMDLINE | TRACE_RECORD_TGID)))) {
    return true;
    }
    if (!__this_cpu_read(trace_taskinfo_save)) {
    return true;
    }
    return false;
    }
//
// tracing_record_taskinfo - record the task info of a task
//
// @task:  task to record
// @flags: TRACE_RECORD_CMDLINE for recording comm
// TRACE_RECORD_TGID for recording tgid
//
#[no_mangle]
pub unsafe extern "C" fn tracing_record_taskinfo(task: *mut task_struct, flags: c_int) {
    let mut done = 0;
    if (tracing_record_taskinfo_skip(flags)) {
    return;
    }
//
// Record as much task information as possible. If some fail, continue
// to try to record the others.
//
    done = !(flags & TRACE_RECORD_CMDLINE) || trace_save_cmdline(task);
    done &= !(flags & TRACE_RECORD_TGID) || trace_save_tgid(task);
// If recording any information failed, retry again soon.
    if (!done) {
    return;
    }
    __this_cpu_write(trace_taskinfo_save, false);
    }
//
// tracing_record_taskinfo_sched_switch - record task info for sched_switch
//
// @prev: previous task during sched_switch
// @next: next task during sched_switch
// @flags: TRACE_RECORD_CMDLINE for recording comm
// TRACE_RECORD_TGID for recording tgid
//
#[no_mangle]
pub unsafe extern "C" fn tracing_record_taskinfo_sched_switch(prev: *mut task_struct, next: *mut task_struct, flags: c_int) {
    let mut done = 0;
    if (tracing_record_taskinfo_skip(flags)) {
    return;
    }
//
// Record as much task information as possible. If some fail, continue
// to try to record the others.
//
    done  = !(flags & TRACE_RECORD_CMDLINE) || trace_save_cmdline(prev);
    done &= !(flags & TRACE_RECORD_CMDLINE) || trace_save_cmdline(next);
    done &= !(flags & TRACE_RECORD_TGID) || trace_save_tgid(prev);
    done &= !(flags & TRACE_RECORD_TGID) || trace_save_tgid(next);
// If recording any information failed, retry again soon.
    if (!done) {
    return;
    }
    __this_cpu_write(trace_taskinfo_save, false);
    }
// Helpers to record a specific task information
#[no_mangle]
pub unsafe extern "C" fn tracing_record_cmdline(task: *mut task_struct) {
    tracing_record_taskinfo(task, TRACE_RECORD_CMDLINE);
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_record_tgid(task: *mut task_struct) {
    tracing_record_taskinfo(task, TRACE_RECORD_TGID);
    }
#[no_mangle]
pub unsafe extern "C" fn trace_alloc_tgid_map() -> c_int {
pub static mut map: *mut c_void = core::ptr::null_mut();
    if (tgid_map) {
    return 0;
    }
    tgid_map_max = init_pid_ns.pid_max;
    map = kvzalloc_objs(*tgid_map, tgid_map_max + 1);
    if (!map) {
    return -ENOMEM;
    }
//
// Pairs with smp_load_acquire() in
// trace_find_tgid_ptr() to ensure that if it observes
// the tgid_map we just allocated then it also observes
// the corresponding tgid_map_max value.
//
    smp_store_release(&tgid_map, map);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn saved_tgids_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
pub static mut pid: c_int = 0;
    return trace_find_tgid_ptr(pid);
    }
#[no_mangle]
pub unsafe extern "C" fn saved_tgids_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut pid: c_int = 0;
    return trace_find_tgid_ptr(pid);
    }
#[no_mangle]
unsafe extern "C" fn saved_tgids_stop(m: *mut seq_file, v: *mut c_void) {
    }
#[no_mangle]
unsafe extern "C" fn saved_tgids_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut entry = v;
pub static mut pid: c_int = 0;
pub static mut tgid: c_int = 0;
    if (tgid == 0) {
    return SEQ_SKIP;
    }
    seq_printf(m, "%d %d\n", pid, tgid);
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn tracing_saved_tgids_open(inode: *mut inode, filp: *mut file) -> c_int {
    let mut ret = 0;
    ret = tracing_check_open_get_tr(core::ptr::null_mut());
    if (ret) {
    return ret;
    }
    return seq_open(filp, &tracing_saved_tgids_seq_ops);
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn saved_cmdlines_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut ptr = v;
    if (*pos || m.count) {
    ptr += 1;
    }
    (*pos)++;
    while (ptr < &savedcmd.map_cmdline_to_pid[savedcmd.cmdline_num]) {
    if (*ptr == -1 || *ptr == NO_CMDLINE_MAP) {
    continue;
    }
    return ptr;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn saved_cmdlines_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut v: *mut c_void = core::ptr::null_mut();
pub static mut l: loff_t = 0;
    preempt_disable();
    arch_spin_lock(&trace_cmdline_lock);
    v = &savedcmd.map_cmdline_to_pid[0];
    while (l <= *pos) {
    v = saved_cmdlines_next(m, v, &l);
    if (!v) {
    return core::ptr::null_mut();
    }
    }
    return v;
    }
#[no_mangle]
unsafe extern "C" fn saved_cmdlines_stop(m: *mut seq_file, v: *mut c_void) {
    arch_spin_unlock(&trace_cmdline_lock);
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn saved_cmdlines_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    char buf[TASK_COMM_LEN];
    let mut pid = v;
    __trace_find_cmdline(*pid, buf);
    seq_printf(m, "%d %s\n", *pid, buf);
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn tracing_saved_cmdlines_open(inode: *mut inode, filp: *mut file) -> c_int {
    let mut ret = 0;
    ret = tracing_check_open_get_tr(core::ptr::null_mut());
    if (ret) {
    return ret;
    }
    return seq_open(filp, &tracing_saved_cmdlines_seq_ops);
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn tracing_saved_cmdlines_size_read(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    char buf[64];
    let mut r = 0;
    preempt_disable();
    arch_spin_lock(&trace_cmdline_lock);
    r = scnprintf(buf, sizeof!(buf), "%u\n", savedcmd.cmdline_num);
    arch_spin_unlock(&trace_cmdline_lock);
    preempt_enable();
    return simple_read_from_buffer(ubuf, cnt, ppos, buf, r);
    }
#[no_mangle]
pub unsafe extern "C" fn trace_free_saved_cmdlines_buffer() {
    free_saved_cmdlines_buffer(savedcmd);
    }
#[no_mangle]
unsafe extern "C" fn tracing_resize_saved_cmdlines(val: c_uint) -> c_int {
    let mut s = core::ptr::null_mut();
    let mut savedcmd_temp = core::ptr::null_mut();
    s = allocate_cmdlines_buffer(val);
    if (!s) {
    return -ENOMEM;
    }
    preempt_disable();
    arch_spin_lock(&trace_cmdline_lock);
    savedcmd_temp = savedcmd;
    savedcmd = s;
    arch_spin_unlock(&trace_cmdline_lock);
    preempt_enable();
    free_saved_cmdlines_buffer(savedcmd_temp);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_saved_cmdlines_size_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut val = 0;
    let mut ret = 0;
    ret = kstrtoul_from_user(ubuf, cnt, 10, &val);
    if (ret) {
    return ret;
    }
// must have at least 1 entry or less than PID_MAX_DEFAULT
    if (!val || val > PID_MAX_DEFAULT) {
    return -EINVAL;
    }
    ret = tracing_resize_saved_cmdlines((unsigned int)val);
    if (ret < 0) {
    return ret;
    }
// ppos += cnt;
    return cnt;
    }
pub static mut file_operations: usize = 0;
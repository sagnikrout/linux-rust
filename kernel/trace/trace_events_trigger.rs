//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_events_trigger.c
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
// trace_events_trigger - trace event triggers
//
// Copyright (C) 2013 Tom Zanussi <tom.zanussi@linux.intel.com>
//

pub static mut trigger_commands: usize = 0;
pub static mut trigger_cmd_mutex: usize = 0;
pub static mut trigger_kthread: *mut c_void = core::ptr::null_mut();
pub static mut trigger_data_free_list: usize = 0;
pub static mut trigger_data_kthread_mutex: usize = 0;
// forward_decl: trigger_kthread_fn;
#[no_mangle]
unsafe extern "C" fn trigger_create_kthread_locked() {
    lockdep_assert_held(&trigger_data_kthread_mutex);
    if (!trigger_kthread) {
pub static mut kthread: *mut c_void = core::ptr::null_mut();
    kthread = kthread_create(trigger_kthread_fn, core::ptr::null_mut(),
    "trigger_data_free");
    if (!IS_ERR(kthread)) {
    WRITE_ONCE(trigger_kthread, kthread);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn trigger_data_free_one(data: *mut event_trigger_data) {
    if (data.private_data_free) {
    data.private_data_free(data);
    }
    kfree(data);
    }
#[no_mangle]
unsafe extern "C" fn trigger_data_free_queued_locked() {
    let mut data = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
pub static mut llnodes: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&trigger_data_kthread_mutex);
    llnodes = llist_del_all(&trigger_data_free_list);
    if (!llnodes) {
    return;
    }
    tracepoint_synchronize_unregister();
    llist_for_each_entry_safe(data, tmp, llnodes, llist) {
    trigger_data_free_one(data);
    }
    }
// Bulk garbage collection of event_trigger_data elements
#[no_mangle]
unsafe extern "C" fn trigger_kthread_fn(ignore: *mut c_void) -> c_int {
    let mut data = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
pub static mut llnodes: *mut c_void = core::ptr::null_mut();
// Once this task starts, it lives forever
    for (;;) {
    set_current_state(TASK_INTERRUPTIBLE);
    if (llist_empty(&trigger_data_free_list)) {
    schedule();
    }
    __set_current_state(TASK_RUNNING);
    llnodes = llist_del_all(&trigger_data_free_list);
// make sure current triggers exit before free
    tracepoint_synchronize_unregister();
    llist_for_each_entry_safe(data, tmp, llnodes, llist) {
    trigger_data_free_one(data);
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn trigger_data_free(data: *mut event_trigger_data) {
    if (!data) {
    return;
    }
    if (data.cmd_ops.set_filter) {
    data.cmd_ops.set_filter(core::ptr::null_mut(), data, core::ptr::null_mut());
    }
//
// Boot-time trigger registration can fail before kthread creation
// works. Keep the deferred-free semantics during boot and let late
// init start the kthread to drain the list.
//
    if (system_state == SYSTEM_BOOTING && !trigger_kthread) {
    llist_add(&data.llist, &trigger_data_free_list);
    return;
    }
    if (unlikely(!trigger_kthread)) {
    guard(mutex)(&trigger_data_kthread_mutex);
    trigger_create_kthread_locked();
// Check again after taking mutex
    if (!trigger_kthread) {
    llist_add(&data.llist, &trigger_data_free_list);
// Drain the queued frees synchronously if creation failed.
    trigger_data_free_queued_locked();
    return;
    }
    }
    llist_add(&data.llist, &trigger_data_free_list);
    wake_up_process(trigger_kthread);
    }
#[no_mangle]
unsafe extern "C" fn trigger_data_free_init() -> c_int {
    guard(mutex)(&trigger_data_kthread_mutex);
    if (llist_empty(&trigger_data_free_list)) {
    return 0;
    }
    trigger_create_kthread_locked();
    if (trigger_kthread) {
    wake_up_process(trigger_kthread);
    }
    else {
    trigger_data_free_queued_locked();
    }
    return 0;
    }
    late_initcall!(trigger_data_free_init);
#[no_mangle]
pub unsafe extern "C" fn data_ops_trigger(data: *mut event_trigger_data, buffer: *mut trace_buffer, rec: *mut c_void, event: *mut ring_buffer_event) {
    let mut cmd_ops = data.cmd_ops;
    if (data.flags & EVENT_TRIGGER_FL_COUNT) {
    if (!cmd_ops.count_func(data, buffer, rec, event)) {
    return;
    }
    }
    cmd_ops.trigger(data, buffer, rec, event);
    }
//
// event_triggers_call - Call triggers associated with a trace event
// @file: The trace_event_file associated with the event
// @buffer: The ring buffer that the event is being written to
// @rec: The trace entry for the event, NULL for unconditional invocation
// @event: The event meta data in the ring buffer
//
// For each trigger associated with an event, invoke the trigger
// function registered with the associated trigger command.  If rec is
// non-NULL, it means that the trigger requires further processing and
// shouldn't be unconditionally invoked.  If rec is non-NULL and the
// trigger has a filter associated with it, rec will checked against
// the filter and if the record matches the trigger will be invoked.
// If the trigger is a 'post_trigger', meaning it shouldn't be invoked
// in any case until the current event is written, the trigger
// function isn't invoked but the bit associated with the deferred
// trigger is set in the return value.
//
// Returns an enum event_trigger_type value containing a set bit for
// any trigger that should be deferred, ETT_NONE if nothing to defer.
//
// Called from tracepoint handlers (with rcu_read_lock_sched() held).
//
// Return: an enum event_trigger_type value containing a set bit for
// any trigger that should be deferred, ETT_NONE if nothing to defer.
//
    enum event_trigger_type
    event_triggers_call(trace_event_file *file, trace_buffer *buffer, void *rec, ring_buffer_event *event)
    {
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut tt: event_trigger_type = 0;
pub static mut filter: *mut c_void = core::ptr::null_mut();
    if (list_empty(&file.triggers)) {
    return tt;
    }
    list_for_each_entry_rcu(data, &file.triggers, list) {
    if (data.paused) {
    continue;
    }
    if (!rec) {
    data_ops_trigger(data, buffer, rec, event);
    continue;
    }
    filter = rcu_dereference_sched(data.filter);
    if (filter && !filter_match_preds(filter, rec)) {
    continue;
    }
    if (event_command_post_trigger(data.cmd_ops)) {
    tt |= data.cmd_ops.trigger_type;
    continue;
    }
    data_ops_trigger(data, buffer, rec, event);
    }
    return tt;
    }
    EXPORT_SYMBOL_GPL(event_triggers_call);
#[no_mangle]
pub unsafe extern "C" fn __trace_trigger_soft_disabled(file: *mut trace_event_file) -> bool {
pub static mut eflags: c_ulong = 0;
    if (eflags & EVENT_FILE_FL_TRIGGER_MODE) {
    event_triggers_call(file, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    }
    if (eflags & EVENT_FILE_FL_SOFT_DISABLED) {
    return true;
    }
    if (eflags & EVENT_FILE_FL_PID_FILTER) {
    return trace_event_ignore_this_pid(file);
    }
    return false;
    }
    EXPORT_SYMBOL_GPL(__trace_trigger_soft_disabled);
//
// event_triggers_post_call - Call 'post_triggers' for a trace event
// @file: The trace_event_file associated with the event
// @tt: event_trigger_type containing a set bit for each trigger to invoke
//
// For each trigger associated with an event, invoke the trigger
// function registered with the associated trigger command, if the
// corresponding bit is set in the tt enum passed into this function.
// See @event_triggers_call for details on how those bits are set.
//
// Called from tracepoint handlers (with rcu_read_lock_sched() held).
//
#[no_mangle]
pub unsafe extern "C" fn event_triggers_post_call(file: *mut trace_event_file, tt: event_trigger_type) {
pub static mut data: *mut c_void = core::ptr::null_mut();
    list_for_each_entry_rcu(data, &file.triggers, list) {
    if (data.paused) {
    continue;
    }
    if (data.cmd_ops.trigger_type & tt) {
    data_ops_trigger(data, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    }
    }
    }
    EXPORT_SYMBOL_GPL(event_triggers_post_call);

#[no_mangle]
pub unsafe extern "C" fn trigger_next(m: *mut seq_file, t: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut event_file = event_file_data(m.private);
    if (t == SHOW_AVAILABLE_TRIGGERS) {
    (*pos)++;
    return core::ptr::null_mut();
    }
    return seq_list_next(t, &event_file.triggers, pos);
    }
#[no_mangle]
unsafe extern "C" fn check_user_trigger(file: *mut trace_event_file) -> bool {
pub static mut data: *mut c_void = core::ptr::null_mut();
    list_for_each_entry_rcu(data, &file.triggers, list,
    lockdep_is_held(&event_mutex)) {
    if (data.flags & EVENT_TRIGGER_FL_PROBE) {
    continue;
    }
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn trigger_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
pub static mut event_file: *mut c_void = core::ptr::null_mut();
// ->stop() is called even if ->start() fails
    mutex_lock(&event_mutex);
    event_file = event_file_file(m.private);
    if (unlikely(!event_file)) {
    return ERR_PTR(-ENODEV);
    }
    if (list_empty(&event_file.triggers) || !check_user_trigger(event_file)) {
    let mut pos = = 0 ? SHOW_AVAILABLE_TRIGGERS : core::ptr::null_mut();
    }
    return seq_list_start(&event_file.triggers, *pos);
    }
#[no_mangle]
unsafe extern "C" fn trigger_stop(m: *mut seq_file, t: *mut c_void) {
    mutex_unlock(&event_mutex);
    }
#[no_mangle]
unsafe extern "C" fn trigger_show(m: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (v == SHOW_AVAILABLE_TRIGGERS) {
    seq_puts(m, "# Available triggers:\n");
    seq_putc(m, '#');
    mutex_lock(&trigger_cmd_mutex);
    list_for_each_entry_reverse(p, &trigger_commands, list) {
    seq_printf(m, " %s", p.name);
    }
    seq_putc(m, '\n');
    mutex_unlock(&trigger_cmd_mutex);
    return 0;
    }
    data = list_entry(v, event_trigger_data, list);
    data.cmd_ops.print(m, data);
    return 0;
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn event_trigger_regex_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut ret = 0;
    ret = security_locked_down(LOCKDOWN_TRACEFS);
    if (ret) {
    return ret;
    }
    guard(mutex)(&event_mutex);
    if (unlikely(!event_file_file(file))) {
    return -ENODEV;
    }
    if ((file.f_mode & FMODE_WRITE) &&
    (file.f_flags & O_TRUNC)) {
pub static mut event_file: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    event_file = event_file_data(file);
    list_for_each_entry(p, &trigger_commands, list) {
    if (p.unreg_all) {
    p.unreg_all(event_file);
    }
    }
    }
    if (file.f_mode & FMODE_READ) {
    ret = seq_open(file, &event_triggers_seq_ops);
    if (!ret) {
    let mut m = file.private_data;
    m.private = file;
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn trigger_process_regex(file: *mut trace_event_file, buff: *mut c_char) -> c_int {
    let mut command = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    next = buff = strim(buff);
    command = strsep(&next, ": \t");
    if (next) {
    next = skip_spaces(next);
    if (!*next) {
    next = core::ptr::null_mut();
    }
    }
    command = (command[0] != '!') ? command : command + 1;
    guard(mutex)(&trigger_cmd_mutex);
    list_for_each_entry(p, &trigger_commands, list) {
    if (strcmp(p.name, command) == 0) {
    return p.parse(p, file, buff, command, next);
    }
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn event_trigger_regex_write(file: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut event_file: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    char *buf __free(kfree) = core::ptr::null_mut();
    if (!cnt) {
    return 0;
    }
    if (cnt >= PAGE_SIZE) {
    return -EINVAL;
    }
    buf = memdup_user_nul(ubuf, cnt);
    if (IS_ERR(buf)) {
    return PTR_ERR(buf);
    }
    guard(mutex)(&event_mutex);
    event_file = event_file_file(file);
    if (unlikely(!event_file)) {
    return -ENODEV;
    }
    ret = trigger_process_regex(event_file, buf);
    if (ret < 0) {
    return ret;
    }
// ppos += cnt;
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn event_trigger_regex_release(inode: *mut inode, file: *mut file) -> c_int {
    if (file.f_mode & FMODE_READ) {
    seq_release(inode, file);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn event_trigger_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    return event_trigger_regex_write(filp, ubuf, cnt, ppos);
    }
#[no_mangle]
pub unsafe extern "C" fn event_trigger_open(inode: *mut inode, filp: *mut file) -> c_int {
// Checks for tracefs lockdown
    return event_trigger_regex_open(inode, filp);
    }
#[no_mangle]
pub unsafe extern "C" fn event_trigger_release(inode: *mut inode, file: *mut file) -> c_int {
    return event_trigger_regex_release(inode, file);
    }
pub static mut file_operations: usize = 0;
//
// Currently we only register event commands from __init, so mark this
// __init too.
//
#[no_mangle]
pub unsafe extern "C" fn register_event_command(cmd: *mut event_command) -> __init int {
pub static mut p: *mut c_void = core::ptr::null_mut();
    guard(mutex)(&trigger_cmd_mutex);
    list_for_each_entry(p, &trigger_commands, list) {
    if (strcmp(cmd.name, p.name) == 0) {
    return -EBUSY;
    }
    }
    list_add(&cmd.list, &trigger_commands);
    return 0;
    }
//
// Currently we only unregister event commands from __init, so mark
// this __init too.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_event_command(cmd: *mut event_command) -> __init int {
    let mut p = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
    guard(mutex)(&trigger_cmd_mutex);
    list_for_each_entry_safe(p, n, &trigger_commands, list) {
    if (strcmp(cmd.name, p.name) == 0) {
    list_del_init(&p.list);
    return 0;
    }
    }
    return -ENODEV;
    }
//
// event_trigger_count - Optional count function for event triggers
// @data: Trigger-specific data
// @buffer: The ring buffer that the event is being written to
// @rec: The trace entry for the event, NULL for unconditional invocation
// @event: The event meta data in the ring buffer
//
// For triggers that can take a count parameter that doesn't do anything
// special, they can use this function to assign to their .count_func
// field.
//
// This simply does a count down of the @data->count field.
//
// If the @data->count is greater than zero, it will decrement it.
//
// Returns false if @data->count is zero, otherwise true.
//
#[no_mangle]
pub unsafe extern "C" fn event_trigger_count(data: *mut event_trigger_data, buffer: *mut trace_buffer, rec: *mut c_void, event: *mut ring_buffer_event) -> bool {
    if (!data.count) {
    return false;
    }
    if (data.count != -1) {
    (data.count)--;
    }
    return true;
    }
//
// event_trigger_print - Generic event_command @print implementation
// @name: The name of the event trigger
// @m: The seq_file being printed to
// @data: Trigger-specific data
// @filter_str: filter_str to print, if present
//
// Common implementation for event triggers to print themselves.
//
// Usually wrapped by a function that simply sets the @name of the
// trigger command and then invokes this.
//
// Return: 0 on success, errno otherwise
//
#[no_mangle]
pub unsafe extern "C" fn event_trigger_print(name: *mut c_char, m: *mut seq_file, data: *mut c_void, filter_str: *mut c_char) -> c_int {
pub static mut count: c_long = 0;
    seq_puts(m, name);
    if (count == -1) {
    seq_puts(m, ":unlimited");
    }
    else {
    seq_printf(m, ":count=%ld", count);
    }
    if (filter_str) {
    seq_printf(m, " if %s\n", filter_str);
    }
    else {
    seq_putc(m, '\n');
    }
    return 0;
    }
//
// event_trigger_init - Generic event_command @init implementation
// @data: Trigger-specific data
//
// Common implementation of event trigger initialization.
//
// Usually used directly as the @init method in event trigger
// implementations.
//
// Return: 0 on success, errno otherwise
//
#[no_mangle]
pub unsafe extern "C" fn event_trigger_init(data: *mut event_trigger_data) -> c_int {
    data.ref += 1;
    return 0;
    }
//
// event_trigger_free - Generic event_command @free implementation
// @data: Trigger-specific data
//
// Common implementation of event trigger de-initialization.
//
// Usually used directly as the @free method in event trigger
// implementations.
//
#[no_mangle]
pub unsafe extern "C" fn event_trigger_free(data: *mut event_trigger_data) {
    if (WARN_ON_ONCE!(data.ref <= 0)) {
    return;
    }
    data.ref -= 1;
    if (!data.ref) {
    trigger_data_free(data);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn trace_event_trigger_enable_disable(file: *mut trace_event_file, trigger_enable: c_int) -> c_int {
pub static mut ret: c_int = 0;
    if (trigger_enable) {
    if (atomic_inc_return(&file.tm_ref) > 1) {
    return ret;
    }
    set_bit(EVENT_FILE_FL_TRIGGER_MODE_BIT, &file.flags);
    ret = trace_event_enable_disable(file, 1, 1);
    } else {
    if (atomic_dec_return(&file.tm_ref) > 0) {
    return ret;
    }
    clear_bit(EVENT_FILE_FL_TRIGGER_MODE_BIT, &file.flags);
    ret = trace_event_enable_disable(file, 0, 1);
    }
    return ret;
    }
//
// clear_event_triggers - Clear all triggers associated with a trace array
// @tr: The trace array to clear
//
// For each trigger, the triggering event has its tm_ref decremented
// via trace_event_trigger_enable_disable(), and any associated event
// (in the case of enable/disable_event triggers) will have its sm_ref
// decremented via free()->trace_event_enable_disable().  That
// combination effectively reverses the soft-mode/trigger state added
// by trigger registration.
//
// Must be called with event_mutex held.
//
#[no_mangle]
pub unsafe extern "C" fn clear_event_triggers(tr: *mut trace_array) {
pub static mut file: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(file, &tr.events, list) {
    let mut data = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
    list_for_each_entry_safe(data, n, &file.triggers, list) {
    trace_event_trigger_enable_disable(file, 0);
    list_del_rcu(&data.list);
    if (data.cmd_ops.free) {
    data.cmd_ops.free(data);
    }
    }
    }
    }
//
// update_cond_flag - Set or reset the TRIGGER_COND bit
// @file: The trace_event_file associated with the event
//
// If an event has triggers and any of those triggers has a filter or
// a post_trigger, trigger invocation needs to be deferred until after
// the current event has logged its data, and the event should have
// its TRIGGER_COND bit set, otherwise the TRIGGER_COND bit should be
// cleared.
//
#[no_mangle]
pub unsafe extern "C" fn update_cond_flag(file: *mut trace_event_file) {
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut set_cond: bool = false;
    lockdep_assert_held(&event_mutex);
    list_for_each_entry(data, &file.triggers, list) {
    if (data.filter || event_command_post_trigger(data.cmd_ops) ||
    event_command_needs_rec(data.cmd_ops)) {
    set_cond = true;
    break;
    }
    }
    if (set_cond) {
    set_bit(EVENT_FILE_FL_TRIGGER_COND_BIT, &file.flags);
    }
    else {
    clear_bit(EVENT_FILE_FL_TRIGGER_COND_BIT, &file.flags);
    }
    }
//
// register_trigger - Generic event_command @reg implementation
// @glob: The raw string used to register the trigger
// @data: Trigger-specific data to associate with the trigger
// @file: The trace_event_file associated with the event
//
// Common implementation for event trigger registration.
//
// Usually used directly as the @reg method in event command
// implementations.
//
// Return: 0 on success, errno otherwise
//
#[no_mangle]
pub unsafe extern "C" fn register_trigger(glob: *mut c_char, data: *mut event_trigger_data, file: *mut trace_event_file) -> c_int {
pub static mut test: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    lockdep_assert_held(&event_mutex);
    list_for_each_entry(test, &file.triggers, list) {
    if (test.cmd_ops.trigger_type == data.cmd_ops.trigger_type) {
    return -EEXIST;
    }
    }
    if (data.cmd_ops.init) {
    ret = data.cmd_ops.init(data);
    if (ret < 0) {
    return ret;
    }
    }
    list_add_rcu(&data.list, &file.triggers);
    update_cond_flag(file);
    ret = trace_event_trigger_enable_disable(file, 1);
    if (ret < 0) {
    list_del_rcu(&data.list);
    update_cond_flag(file);
    }
    return ret;
    }
//
// True if the trigger was found and unregistered, else false.
//
#[no_mangle]
pub unsafe extern "C" fn try_unregister_trigger(glob: *mut c_char, test: *mut event_trigger_data, file: *mut trace_event_file) -> bool {
    let mut data = core::ptr::null_mut(), *iter;
    lockdep_assert_held(&event_mutex);
    list_for_each_entry(iter, &file.triggers, list) {
    if (iter.cmd_ops.trigger_type == test.cmd_ops.trigger_type) {
    data = iter;
    list_del_rcu(&data.list);
    trace_event_trigger_enable_disable(file, 0);
    update_cond_flag(file);
    break;
    }
    }
    if (data) {
    if (data.cmd_ops.free) {
    data.cmd_ops.free(data);
    }
    return true;
    }
    return false;
    }
//
// unregister_trigger - Generic event_command @unreg implementation
// @glob: The raw string used to register the trigger
// @test: Trigger-specific data used to find the trigger to remove
// @file: The trace_event_file associated with the event
//
// Common implementation for event trigger unregistration.
//
// Usually used directly as the @unreg method in event command
// implementations.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_trigger(glob: *mut c_char, test: *mut event_trigger_data, file: *mut trace_event_file) {
    try_unregister_trigger(glob, test, file);
    }
//
// Event trigger parsing helper functions.
//
// These functions help make it easier to write an event trigger
// parsing function i.e. the struct event_command.parse() callback
// function responsible for parsing and registering a trigger command
// written to the 'trigger' file.
//
// A trigger command (or just 'trigger' for short) takes the form:
// [trigger] [if filter]
//
// The struct event_command.parse() callback (and other struct
// event_command functions) refer to several components of a trigger
// command.  Those same components are referenced by the event trigger
// parsing helper functions defined below.  These components are:
//
// cmd               - the trigger command name
// glob              - the trigger command name optionally prefaced with '!'
// param_and_filter  - text following cmd and ':'
// param             - text following cmd and ':' and stripped of filter
// filter            - the optional filter text following (and including) 'if'
//
// To illustrate the use of these components, here are some concrete
// examples. For the following triggers:
//
// echo 'traceon:5 if pid == 0' > trigger
// - 'traceon' is both cmd and glob
// - '5 if pid == 0' is the param_and_filter
// - '5' is the param
// - 'if pid == 0' is the filter
//
// echo 'enable_event:sys:event:n' > trigger
// - 'enable_event' is both cmd and glob
// - 'sys:event:n' is the param_and_filter
// - 'sys:event:n' is the param
// - there is no filter
//
// echo 'hist:keys=pid if prio > 50' > trigger
// - 'hist' is both cmd and glob
// - 'keys=pid if prio > 50' is the param_and_filter
// - 'keys=pid' is the param
// - 'if prio > 50' is the filter
//
// echo '!enable_event:sys:event:n' > trigger
// - 'enable_event' the cmd
// - '!enable_event' is the glob
// - 'sys:event:n' is the param_and_filter
// - 'sys:event:n' is the param
// - there is no filter
//
// echo 'traceoff' > trigger
// - 'traceoff' is both cmd and glob
// - there is no param_and_filter
// - there is no param
// - there is no filter
//
// There are a few different categories of event trigger covered by
// these helpers:
//
// - triggers that don't require a parameter e.g. traceon
// - triggers that do require a parameter e.g. enable_event and hist
// - triggers that though they may not require a param may support an
// optional 'n' param (n = number of times the trigger should fire)
// e.g.: traceon:5 or enable_event:sys:event:n
// - triggers that do not support an 'n' param e.g. hist
//
// These functions can be used or ignored as necessary - it all
// depends on the complexity of the trigger, and the granularity of
// the functions supported reflects the fact that some implementations
// may need to customize certain aspects of their implementations and
// won't need certain functions.  For instance, the hist trigger
// implementation doesn't use event_trigger_separate_filter() because
// it has special requirements for handling the filter.
//
// event_trigger_check_remove - check whether an event trigger specifies remove
// @glob: The trigger command string, with optional remove(!) operator
//
// The event trigger callback implementations pass in 'glob' as a
// parameter.  This is the command name either with or without a
// remove(!)  operator.  This function simply parses the glob and
// determines whether the command corresponds to a trigger removal or
// a trigger addition.
//
// Return: true if this is a remove command, false otherwise
//
#[no_mangle]
pub unsafe extern "C" fn event_trigger_check_remove(glob: *const c_char) -> bool {
    return (glob && glob[0] == '!') ? true : false;
    }
//
// event_trigger_empty_param - check whether the param is empty
// @param: The trigger param string
//
// The event trigger callback implementations pass in 'param' as a
// parameter.  This corresponds to the string following the command
// name minus the command name.  This function can be called by a
// callback implementation for any command that requires a param; a
// callback that doesn't require a param can ignore it.
//
// Return: true if this is an empty param, false otherwise
//
#[no_mangle]
pub unsafe extern "C" fn event_trigger_empty_param(param: *const c_char) -> bool {
    return !param;
    }
//
// event_trigger_separate_filter - separate an event trigger from a filter
// @param_and_filter: String containing trigger and possibly filter
// @param: outparam, will be filled with a pointer to the trigger
// @filter: outparam, will be filled with a pointer to the filter
// @param_required: Specifies whether or not the param string is required
//
// Given a param string of the form '[trigger] [if filter]', this
// function separates the filter from the trigger and returns the
// trigger in @param and the filter in @filter.  Either the @param
// or the @filter may be set to NULL by this function - if not set to
// NULL, they will contain strings corresponding to the trigger and
// filter.
//
// There are two cases that need to be handled with respect to the
// passed-in param: either the param is required, or it is not
// required.  If @param_required is set, and there's no param, it will
// return -EINVAL.  If @param_required is not set and there's a param
// that starts with a number, that corresponds to the case of a
// trigger with :n (n = number of times the trigger should fire) and
// the parsing continues normally; otherwise the function just returns
// and assumes param just contains a filter and there's nothing else
// to do.
//
// Return: 0 on success, errno otherwise
//
#[no_mangle]
pub unsafe extern "C" fn event_trigger_separate_filter(param_and_filter: *mut c_char, param: *mut *mut c_char, filter: *mut *mut c_char, param_required: bool) -> c_int {
pub static mut ret: c_int = 0;
// param = *filter = NULL;
    if (!param_and_filter) {
    if (param_required) {
    ret = -EINVAL;
    }
    return ret;
    }
//
// Here we check for an optional param. The only legal
// optional param is :n, and if that's the case, continue
// below. Otherwise we assume what's left is a filter and
// return it as the filter string for the caller to deal with.
//
    if (!param_required && param_and_filter && !isdigit(param_and_filter[0])) {
// filter = param_and_filter;
    return ret;
    }
//
// Separate the param from the filter (param [if filter]).
// Here we have either an optional :n param or a required
// param and an optional filter.
//
// param = strsep(&param_and_filter, " \t");
//
// Here we have a filter, though it may be empty.
//
    if (param_and_filter) {
// filter = skip_spaces(param_and_filter);
    if (!**filter) {
// filter = NULL;
    }
    }
    return ret;
    }
//
// trigger_data_alloc - allocate and init event_trigger_data for a trigger
// @cmd_ops: The event_command operations for the trigger
// @cmd: The cmd string
// @param: The param string
// @private_data: User data to associate with the event trigger
//
// Allocate an event_trigger_data instance and initialize it.  The
// @cmd_ops defines how the trigger will operate. If @param is set,
// and @cmd_ops->trigger_ops->count_func is non NULL, then the
// data->count is set to @param and before the trigger is executed, the
// @cmd_ops->trigger_ops->count_func() is called. If that function returns
// false, the @cmd_ops->trigger_ops->trigger() function will not be called.
// @private_data can also be passed in and associated with the
// event_trigger_data.
//
// Use trigger_data_free() to free an event_trigger_data object.
//
// Return: The trigger_data object success, NULL otherwise
//
#[no_mangle]
pub unsafe extern "C" fn trigger_data_alloc(cmd_ops: *mut event_command, cmd: *mut c_char, param: *mut c_char, private_data: *mut c_void) -> *mut c_void {
pub static mut trigger_data: *mut c_void = core::ptr::null_mut();
    trigger_data = kzalloc_obj(*trigger_data);
    if (!trigger_data) {
    return core::ptr::null_mut();
    }
    trigger_data.count = -1;
    trigger_data.cmd_ops = cmd_ops;
    trigger_data.private_data = private_data;
    if (param && cmd_ops.count_func) {
    trigger_data.flags |= EVENT_TRIGGER_FL_COUNT;
    }
    INIT_LIST_HEAD(&trigger_data.list);
    INIT_LIST_HEAD(&trigger_data.named_list);
    RCU_INIT_POINTER(trigger_data.filter, core::ptr::null_mut());
    return trigger_data;
    }
//
// event_trigger_parse_num - parse and return the number param for a trigger
// @param: The param string
// @trigger_data: The trigger_data for the trigger
//
// Parse the :n (n = number of times the trigger should fire) param
// and set the count variable in the trigger_data to the parsed count.
//
// Return: 0 on success, errno otherwise
//
#[no_mangle]
pub unsafe extern "C" fn event_trigger_parse_num(param: *mut c_char, trigger_data: *mut event_trigger_data) -> c_int {
pub static mut number: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (param) {
    number = strsep(&param, ":");
    if (!strlen(number)) {
    return -EINVAL;
    }
//
// We use the callback data field (which is a pointer)
// as our counter.
//
    ret = kstrtoul(number, 0, &trigger_data.count);
    }
    return ret;
    }
//
// event_trigger_set_filter - set an event trigger's filter
// @cmd_ops: The event_command operations for the trigger
// @file: The event file for the trigger's event
// @param: The string containing the filter
// @trigger_data: The trigger_data for the trigger
//
// Set the filter for the trigger.  If the filter is NULL, just return
// without error.
//
// Return: 0 on success, errno otherwise
//
#[no_mangle]
pub unsafe extern "C" fn event_trigger_set_filter(cmd_ops: *mut event_command, file: *mut trace_event_file, param: *mut c_char, trigger_data: *mut event_trigger_data) -> c_int {
    if (param && cmd_ops.set_filter) {
    return cmd_ops.set_filter(param, trigger_data, file);
    }
    return 0;
    }
//
// event_trigger_reset_filter - reset an event trigger's filter
// @cmd_ops: The event_command operations for the trigger
// @trigger_data: The trigger_data for the trigger
//
// Reset the filter for the trigger to no filter.
//
#[no_mangle]
pub unsafe extern "C" fn event_trigger_reset_filter(cmd_ops: *mut event_command, trigger_data: *mut event_trigger_data) {
    if (cmd_ops.set_filter) {
    cmd_ops.set_filter(core::ptr::null_mut(), trigger_data, core::ptr::null_mut());
    }
    }
//
// event_trigger_register - register an event trigger
// @cmd_ops: The event_command operations for the trigger
// @file: The event file for the trigger's event
// @glob: The trigger command string, with optional remove(!) operator
// @trigger_data: The trigger_data for the trigger
//
// Register an event trigger.  The @cmd_ops are used to call the
// cmd_ops->reg() function which actually does the registration.
//
// Return: 0 on success, errno otherwise
//
#[no_mangle]
pub unsafe extern "C" fn event_trigger_register(cmd_ops: *mut event_command, file: *mut trace_event_file, glob: *mut c_char, trigger_data: *mut event_trigger_data) -> c_int {
    return cmd_ops.reg(glob, trigger_data, file);
    }
//
// event_trigger_unregister - unregister an event trigger
// @cmd_ops: The event_command operations for the trigger
// @file: The event file for the trigger's event
// @glob: The trigger command string, with optional remove(!) operator
// @trigger_data: The trigger_data for the trigger
//
// Unregister an event trigger.  The @cmd_ops are used to call the
// cmd_ops->unreg() function which actually does the unregistration.
//
#[no_mangle]
pub unsafe extern "C" fn event_trigger_unregister(cmd_ops: *mut event_command, file: *mut trace_event_file, glob: *mut c_char, trigger_data: *mut event_trigger_data) {
    cmd_ops.unreg(glob, trigger_data, file);
    }
//
// End event trigger parsing helper functions.
//
// event_trigger_parse - Generic event_command @parse implementation
// @cmd_ops: The command ops, used for trigger registration
// @file: The trace_event_file associated with the event
// @glob: The raw string used to register the trigger
// @cmd: The cmd portion of the string used to register the trigger
// @param_and_filter: The param and filter portion of the string used to register the trigger
//
// Common implementation for event command parsing and trigger
// instantiation.
//
// Usually used directly as the @parse method in event command
// implementations.
//
// Return: 0 on success, errno otherwise
//
#[no_mangle]
pub unsafe extern "C" fn event_trigger_parse(cmd_ops: *mut event_command, file: *mut trace_event_file, glob: *mut c_char, cmd: *mut c_char, param_and_filter: *mut c_char) -> c_int {
pub static mut trigger_data: *mut c_void = core::ptr::null_mut();
    let mut param = core::ptr::null_mut();
    let mut filter = core::ptr::null_mut();
    let mut remove = 0;
    let mut ret = 0;
    remove = event_trigger_check_remove(glob);
    ret = event_trigger_separate_filter(param_and_filter, &param, &filter, false);
    if (ret) {
    return ret;
    }
    ret = -ENOMEM;
    trigger_data = trigger_data_alloc(cmd_ops, cmd, param, file);
    if (!trigger_data) {
    return ret;
    }
    if (remove) {
    event_trigger_unregister(cmd_ops, file, glob+1, trigger_data);
    trigger_data_free(trigger_data);
    return 0;
    }
    ret = event_trigger_parse_num(param, trigger_data);
    if (ret) {
// goto;
    }
    ret = event_trigger_set_filter(cmd_ops, file, filter, trigger_data);
    if (ret < 0) {
// goto;
    }
// Up the trigger_data count to make sure reg doesn't free it on failure
    event_trigger_init(trigger_data);
    ret = event_trigger_register(cmd_ops, file, glob, trigger_data);
    if (ret) {
// goto;
    }
// Down the counter of trigger_data or free it if not used anymore
    event_trigger_free(trigger_data);
    return ret;
// label;
    event_trigger_reset_filter(cmd_ops, trigger_data);
    trigger_data_free(trigger_data);
    return ret;
    }
//
// set_trigger_filter - Generic event_command @set_filter implementation
// @filter_str: The filter string for the trigger, NULL to remove filter
// @trigger_data: Trigger-specific data
// @file: The trace_event_file associated with the event
//
// Common implementation for event command filter parsing and filter
// instantiation.
//
// Usually used directly as the @set_filter method in event command
// implementations.
//
// Also used to remove a filter (if filter_str = NULL).
//
// Return: 0 on success, errno otherwise
//
#[no_mangle]
pub unsafe extern "C" fn set_trigger_filter(filter_str: *mut c_char, trigger_data: *mut event_trigger_data, file: *mut trace_event_file) -> c_int {
    let mut data = trigger_data;
    let mut filter = core::ptr::null_mut(), *tmp;
pub static mut ret: c_int = 0;
pub static mut s: *mut c_void = core::ptr::null_mut();
    if (!filter_str) /* clear the current filter */ {
// goto;
    }
    s = strsep(&filter_str, " \t");
    if (!strlen(s) || strcmp(s, "if") != 0) {
    return ret;
    }
    if (!filter_str) {
    return ret;
    }
// The filter is for the 'trigger' event, not the triggered event
    ret = create_event_filter(file.tr, file.event_call,
    filter_str, true, &filter);
// Only enabled set_str for error handling
    if (filter) {
    kfree(filter.filter_string);
    filter.filter_string = core::ptr::null_mut();
    }
//
// If create_event_filter() fails, filter still needs to be freed.
// Which the calling code will do with data->filter.
//
// label;
    tmp = rcu_access_pointer(data.filter);
    rcu_assign_pointer(data.filter, filter);
    if (tmp) {
//
// Make sure the call is done with the filter.
// It is possible that a filter could fail at boot up,
// and then this path will be called. Avoid the synchronization
// in that case.
//
    if (system_state != SYSTEM_BOOTING) {
    tracepoint_synchronize_unregister();
    }
    free_event_filter(tmp);
    }
    kfree(data.filter_str);
    data.filter_str = core::ptr::null_mut();
    if (filter_str) {
    data.filter_str = kstrdup(filter_str, GFP_KERNEL);
    if (!data.filter_str) {
    free_event_filter(rcu_access_pointer(data.filter));
    data.filter = core::ptr::null_mut();
    ret = -ENOMEM;
    }
    }
    return ret;
    }
pub static mut named_triggers: usize = 0;
//
// find_named_trigger - Find the common named trigger associated with @name
// @name: The name of the set of named triggers to find the common data for
//
// Named triggers are sets of triggers that share a common set of
// trigger data.  The first named trigger registered with a given name
// owns the common trigger data that the others subsequently
// registered with the same name will reference.  This function
// returns the common trigger data associated with that first
// registered instance.
//
// Return: the common trigger data for the given named trigger on
// success, NULL otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn find_named_trigger(name: *mut c_char) -> *mut c_void {
pub static mut data: *mut c_void = core::ptr::null_mut();
    if (!name) {
    return core::ptr::null_mut();
    }
    list_for_each_entry(data, &named_triggers, named_list) {
    if (data.named_data) {
    continue;
    }
    if (strcmp(data.name, name) == 0) {
    return data;
    }
    }
    return core::ptr::null_mut();
    }
//
// is_named_trigger - determine if a given trigger is a named trigger
// @test: The trigger data to test
//
// Return: true if 'test' is a named trigger, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn is_named_trigger(test: *mut event_trigger_data) -> bool {
pub static mut data: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(data, &named_triggers, named_list) {
    if (test == data) {
    return true;
    }
    }
    return false;
    }
//
// save_named_trigger - save the trigger in the named trigger list
// @name: The name of the named trigger set
// @data: The trigger data to save
//
// Return: 0 if successful, negative error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn save_named_trigger(name: *const c_char, data: *mut event_trigger_data) -> c_int {
    data.name = kstrdup(name, GFP_KERNEL);
    if (!data.name) {
    return -ENOMEM;
    }
    list_add(&data.named_list, &named_triggers);
    return 0;
    }
//
// del_named_trigger - delete a trigger from the named trigger list
// @data: The trigger data to delete
//
#[no_mangle]
pub unsafe extern "C" fn del_named_trigger(data: *mut event_trigger_data) {
    kfree(data.name);
    data.name = core::ptr::null_mut();
    list_del(&data.named_list);
    }
#[no_mangle]
unsafe extern "C" fn __pause_named_trigger(data: *mut event_trigger_data, pause: bool) {
pub static mut test: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(test, &named_triggers, named_list) {
    if (strcmp(test.name, data.name) == 0) {
    if (pause) {
    test.paused_tmp = test.paused;
    test.paused = true;
    } else {
    test.paused = test.paused_tmp;
    }
    }
    }
    }
//
// pause_named_trigger - Pause all named triggers with the same name
// @data: The trigger data of a named trigger to pause
//
// Pauses a named trigger along with all other triggers having the
// same name.  Because named triggers share a common set of data,
// pausing only one is meaningless, so pausing one named trigger needs
// to pause all triggers with the same name.
//
#[no_mangle]
pub unsafe extern "C" fn pause_named_trigger(data: *mut event_trigger_data) {
    __pause_named_trigger(data, true);
    }
//
// unpause_named_trigger - Un-pause all named triggers with the same name
// @data: The trigger data of a named trigger to unpause
//
// Un-pauses a named trigger along with all other triggers having the
// same name.  Because named triggers share a common set of data,
// unpausing only one is meaningless, so unpausing one named trigger
// needs to unpause all triggers with the same name.
//
#[no_mangle]
pub unsafe extern "C" fn unpause_named_trigger(data: *mut event_trigger_data) {
    __pause_named_trigger(data, false);
    }
//
// set_named_trigger_data - Associate common named trigger data
// @data: The trigger data to associate
// @named_data: The common named trigger to be associated
//
// Named triggers are sets of triggers that share a common set of
// trigger data.  The first named trigger registered with a given name
// owns the common trigger data that the others subsequently
// registered with the same name will reference.  This function
// associates the common trigger data from the first trigger with the
// given trigger.
//
#[no_mangle]
pub unsafe extern "C" fn set_named_trigger_data(data: *mut event_trigger_data, named_data: *mut event_trigger_data) {
    data.named_data = named_data;
    }
#[no_mangle]
pub unsafe extern "C" fn get_named_trigger_data(data: *mut event_trigger_data) -> *mut c_void {
    return data.named_data;
    }
#[no_mangle]
pub unsafe extern "C" fn traceon_trigger(data: *mut event_trigger_data, buffer: *mut trace_buffer, rec: *mut c_void, event: *mut ring_buffer_event) {
    let mut file = data.private_data;
    if (WARN_ON_ONCE!(!file)) {
    return;
    }
    if (tracer_tracing_is_on(file.tr)) {
    return;
    }
    tracer_tracing_on(file.tr);
    }
#[no_mangle]
pub unsafe extern "C" fn traceon_count_func(data: *mut event_trigger_data, buffer: *mut trace_buffer, rec: *mut c_void, event: *mut ring_buffer_event) -> bool {
    let mut file = data.private_data;
    if (WARN_ON_ONCE!(!file)) {
    return false;
    }
    if (tracer_tracing_is_on(file.tr)) {
    return false;
    }
    if (!data.count) {
    return false;
    }
    if (data.count != -1) {
    (data.count)--;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn traceoff_trigger(data: *mut event_trigger_data, buffer: *mut trace_buffer, rec: *mut c_void, event: *mut ring_buffer_event) {
    let mut file = data.private_data;
    if (WARN_ON_ONCE!(!file)) {
    return;
    }
    if (!tracer_tracing_is_on(file.tr)) {
    return;
    }
    tracer_tracing_off(file.tr);
    }
#[no_mangle]
pub unsafe extern "C" fn traceoff_count_func(data: *mut event_trigger_data, buffer: *mut trace_buffer, rec: *mut c_void, event: *mut ring_buffer_event) -> bool {
    let mut file = data.private_data;
    if (WARN_ON_ONCE!(!file)) {
    return false;
    }
    if (!tracer_tracing_is_on(file.tr)) {
    return false;
    }
    if (!data.count) {
    return false;
    }
    if (data.count != -1) {
    (data.count)--;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn traceon_trigger_print(m: *mut seq_file, data: *mut event_trigger_data) -> c_int {
    return event_trigger_print("traceon", m, data.count,
    data.filter_str);
    }
#[no_mangle]
pub unsafe extern "C" fn traceoff_trigger_print(m: *mut seq_file, data: *mut event_trigger_data) -> c_int {
    return event_trigger_print("traceoff", m, data.count,
    data.filter_str);
    }
pub static mut event_command: usize = 0;
pub static mut event_command: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn snapshot_trigger(data: *mut event_trigger_data, buffer: *mut trace_buffer, rec: *mut c_void, event: *mut ring_buffer_event) {
    let mut file = data.private_data;
    if (WARN_ON_ONCE!(!file)) {
    return;
    }
    tracing_snapshot_instance(file.tr);
    }
#[no_mangle]
pub unsafe extern "C" fn register_snapshot_trigger(glob: *mut c_char, data: *mut event_trigger_data, file: *mut trace_event_file) -> c_int {
pub static mut ret: c_int = 0;
    if (ret < 0) {
    return ret;
    }
    ret = register_trigger(glob, data, file);
    if (ret < 0) {
    tracing_disarm_snapshot(file.tr);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn unregister_snapshot_trigger(glob: *mut c_char, data: *mut event_trigger_data, file: *mut trace_event_file) {
    if (try_unregister_trigger(glob, data, file)) {
    tracing_disarm_snapshot(file.tr);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn snapshot_trigger_print(m: *mut seq_file, data: *mut event_trigger_data) -> c_int {
    return event_trigger_print("snapshot", m, data.count,
    data.filter_str);
    }
pub static mut event_command: usize = 0;
#[no_mangle]
unsafe extern "C" fn register_trigger_snapshot_cmd() -> __init int {
    let mut ret = 0;
    ret = register_event_command(&trigger_snapshot_cmd);
    WARN_ON!(ret < 0);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn register_trigger_snapshot_cmd() -> c_int { return 0; }

// Skip 2:
// event_triggers_post_call()
// trace_event_raw_event_xxx()
//

//
// Skip 4:
// stacktrace_trigger()
// event_triggers_post_call()
// trace_event_buffer_commit()
// trace_event_raw_event_xxx()
//
pub const STACK_SKIP: c_int = 4;

#[no_mangle]
pub unsafe extern "C" fn stacktrace_trigger(data: *mut event_trigger_data, buffer: *mut trace_buffer, rec: *mut c_void, event: *mut ring_buffer_event) {
    let mut file = data.private_data;
    if (WARN_ON_ONCE!(!file)) {
    return;
    }
    __trace_stack(file.tr, tracing_gen_ctx_dec(), STACK_SKIP);
    }
#[no_mangle]
pub unsafe extern "C" fn stacktrace_trigger_print(m: *mut seq_file, data: *mut event_trigger_data) -> c_int {
    return event_trigger_print("stacktrace", m, data.count,
    data.filter_str);
    }
pub static mut event_command: usize = 0;
#[no_mangle]
unsafe extern "C" fn register_trigger_stacktrace_cmd() -> __init int {
    let mut ret = 0;
    ret = register_event_command(&trigger_stacktrace_cmd);
    WARN_ON!(ret < 0);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn register_trigger_stacktrace_cmd() -> c_int { return 0; }

#[no_mangle]
unsafe extern "C" fn unregister_trigger_traceon_traceoff_cmds() -> __init void {
    unregister_event_command(&trigger_traceon_cmd);
    unregister_event_command(&trigger_traceoff_cmd);
    }
#[no_mangle]
pub unsafe extern "C" fn event_enable_trigger(data: *mut event_trigger_data, buffer: *mut trace_buffer, rec: *mut c_void, event: *mut ring_buffer_event) {
    let mut enable_data = data.private_data;
    if (enable_data.enable) {
    clear_bit(EVENT_FILE_FL_SOFT_DISABLED_BIT, &enable_data.file.flags);
    }
    else {
    set_bit(EVENT_FILE_FL_SOFT_DISABLED_BIT, &enable_data.file.flags);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn event_enable_count_func(data: *mut event_trigger_data, buffer: *mut trace_buffer, rec: *mut c_void, event: *mut ring_buffer_event) -> bool {
    let mut enable_data = data.private_data;
    if (!data.count) {
    return false;
    }
// Skip if the event is in a state we want to switch to
    if (enable_data.enable == !(enable_data.file.flags & EVENT_FILE_FL_SOFT_DISABLED)) {
    return false;
    }
    if (data.count != -1) {
    (data.count)--;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn event_enable_trigger_print(m: *mut seq_file, data: *mut event_trigger_data) -> c_int {
    let mut enable_data = data.private_data;
    seq_printf(m, "%s:%s:%s",
    enable_data.hist ?
    (enable_data.enable ? ENABLE_HIST_STR : DISABLE_HIST_STR) :
    (enable_data.enable ? ENABLE_EVENT_STR : DISABLE_EVENT_STR),
    enable_data.file.event_call.class.system,
    trace_event_name(enable_data.file.event_call));
    if (data.count == -1) {
    seq_puts(m, ":unlimited");
    }
    else {
    seq_printf(m, ":count=%ld", data.count);
    }
    if (data.filter_str) {
    seq_printf(m, " if %s\n", data.filter_str);
    }
    else {
    seq_putc(m, '\n');
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn enable_trigger_private_data_free(data: *mut event_trigger_data) {
    let mut enable_data = data.private_data;
    trace_event_put_ref(enable_data.file.event_call);
    kfree(enable_data);
    }
#[no_mangle]
pub unsafe extern "C" fn event_enable_trigger_free(data: *mut event_trigger_data) {
    let mut enable_data = data.private_data;
    if (WARN_ON_ONCE!(data.ref <= 0)) {
    return;
    }
    data.ref -= 1;
    if (!data.ref) {
// Remove the SOFT_MODE flag
    trace_event_enable_disable(enable_data.file, 0, 1);
    data.private_data_free = enable_trigger_private_data_free;
    trigger_data_free(data);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn event_enable_trigger_parse(cmd_ops: *mut event_command, file: *mut trace_event_file, glob: *mut c_char, cmd: *mut c_char, param_and_filter: *mut c_char) -> c_int {
pub static mut event_enable_file: *mut c_void = core::ptr::null_mut();
    struct enable_trigger_data *enable_data __free(kfree) = core::ptr::null_mut();
pub static mut trigger_data: *mut c_void = core::ptr::null_mut();
    let mut tr = file.tr;
    let mut param = core::ptr::null_mut();
    let mut filter = core::ptr::null_mut();
    let mut enable = 0;
    let mut remove = 0;
pub static mut system: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut hist: bool = false;
    let mut ret = 0;
    remove = event_trigger_check_remove(glob);
    if (event_trigger_empty_param(param_and_filter)) {
    return -EINVAL;
    }
    ret = event_trigger_separate_filter(param_and_filter, &param, &filter, true);
    if (ret) {
    return ret;
    }
    system = strsep(&param, ":");
    if (!param) {
    return -EINVAL;
    }
    event = strsep(&param, ":");
    ret = -EINVAL;
    event_enable_file = find_event_file(tr, system, event);
    if (!event_enable_file) {
    return ret;
    }

    hist = ((strcmp(cmd, ENABLE_HIST_STR) == 0) ||
    (strcmp(cmd, DISABLE_HIST_STR) == 0));
    enable = ((strcmp(cmd, ENABLE_EVENT_STR) == 0) ||
    (strcmp(cmd, ENABLE_HIST_STR) == 0));

    enable = strcmp(cmd, ENABLE_EVENT_STR) == 0;

    ret = -ENOMEM;
    enable_data = kzalloc_obj(*enable_data);
    if (!enable_data) {
    return ret;
    }
    enable_data.hist = hist;
    enable_data.enable = enable;
    enable_data.file = event_enable_file;
    trigger_data = trigger_data_alloc(cmd_ops, cmd, param, enable_data);
    if (!trigger_data) {
    return ret;
    }
    if (remove) {
    event_trigger_unregister(cmd_ops, file, glob+1, trigger_data);
    kfree(trigger_data);
    return 0;
    }
// Up the trigger_data count to make sure nothing frees it on failure
    event_trigger_init(trigger_data);
    ret = event_trigger_parse_num(param, trigger_data);
    if (ret) {
// goto;
    }
    ret = event_trigger_set_filter(cmd_ops, file, filter, trigger_data);
    if (ret < 0) {
// goto;
    }
// Don't let event modules unload while probe registered
    ret = trace_event_try_get_ref(event_enable_file.event_call);
    if (!ret) {
    ret = -EBUSY;
// goto;
    }
    ret = trace_event_enable_disable(event_enable_file, 1, 1);
    if (ret < 0) {
// goto;
    }
    ret = event_trigger_register(cmd_ops, file, glob, trigger_data);
    if (ret) {
// goto;
    }
// It's now safe to free the reference taken earlier
    event_trigger_free(trigger_data);
// The enabled_data is assigned to trigger_data->private_data
    retain_and_null_ptr(enable_data);
    return ret;
// label;
    trace_event_enable_disable(event_enable_file, 0, 1);
// label;
    trace_event_put_ref(event_enable_file.event_call);
// label;
    event_trigger_reset_filter(cmd_ops, trigger_data);
    event_trigger_free(trigger_data);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn event_enable_register_trigger(glob: *mut c_char, data: *mut event_trigger_data, file: *mut trace_event_file) -> c_int {
    let mut enable_data = data.private_data;
pub static mut test_enable_data: *mut c_void = core::ptr::null_mut();
pub static mut test: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    lockdep_assert_held(&event_mutex);
    list_for_each_entry(test, &file.triggers, list) {
    test_enable_data = test.private_data;
    if (test_enable_data &&
    (test.cmd_ops.trigger_type ==
    data.cmd_ops.trigger_type) &&
    (test_enable_data.file == enable_data.file)) {
    return -EEXIST;
    }
    }
    if (data.cmd_ops.init) {
    ret = data.cmd_ops.init(data);
    if (ret < 0) {
    return ret;
    }
    }
    list_add_rcu(&data.list, &file.triggers);
    update_cond_flag(file);
    ret = trace_event_trigger_enable_disable(file, 1);
    if (ret < 0) {
    list_del_rcu(&data.list);
    update_cond_flag(file);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn event_enable_unregister_trigger(glob: *mut c_char, test: *mut event_trigger_data, file: *mut trace_event_file) {
    let mut test_enable_data = test.private_data;
    let mut data = core::ptr::null_mut(), *iter;
pub static mut enable_data: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&event_mutex);
    list_for_each_entry(iter, &file.triggers, list) {
    enable_data = iter.private_data;
    if (enable_data &&
    (iter.cmd_ops.trigger_type ==
    test.cmd_ops.trigger_type) &&
    (enable_data.file == test_enable_data.file)) {
    data = iter;
    list_del_rcu(&data.list);
    trace_event_trigger_enable_disable(file, 0);
    update_cond_flag(file);
    break;
    }
    }
    if (data && data.cmd_ops.free) {
    data.cmd_ops.free(data);
    }
    }
pub static mut event_command: usize = 0;
pub static mut event_command: usize = 0;
#[no_mangle]
unsafe extern "C" fn unregister_trigger_enable_disable_cmds() -> __init void {
    unregister_event_command(&trigger_enable_cmd);
    unregister_event_command(&trigger_disable_cmd);
    }
#[no_mangle]
unsafe extern "C" fn register_trigger_enable_disable_cmds() -> __init int {
    let mut ret = 0;
    ret = register_event_command(&trigger_enable_cmd);
    if (WARN_ON!(ret < 0)) {
    return ret;
    }
    ret = register_event_command(&trigger_disable_cmd);
    if (WARN_ON!(ret < 0)) {
    unregister_trigger_enable_disable_cmds();
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn register_trigger_traceon_traceoff_cmds() -> __init int {
    let mut ret = 0;
    ret = register_event_command(&trigger_traceon_cmd);
    if (WARN_ON!(ret < 0)) {
    return ret;
    }
    ret = register_event_command(&trigger_traceoff_cmd);
    if (WARN_ON!(ret < 0)) {
    unregister_trigger_traceon_traceoff_cmds();
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn register_trigger_cmds() -> __init int {
    register_trigger_traceon_traceoff_cmds();
    register_trigger_snapshot_cmd();
    register_trigger_stacktrace_cmd();
    register_trigger_enable_disable_cmds();
    register_trigger_hist_enable_disable_cmds();
    register_trigger_hist_cmd();
    return 0;
    }
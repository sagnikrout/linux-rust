//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_event_perf.c
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
// trace event based perf event profiling/tracing
//
// Copyright (C) 2009 Red Hat Inc, Peter Zijlstra
// Copyright (C) 2009-2010 Frederic Weisbecker <fweisbec@gmail.com>
//

    static char  *perf_trace_buf[PERF_NR_CONTEXTS];
//
// Force it to be aligned to unsigned long to avoid misaligned accesses
// surprises
//
#[no_mangle]
pub unsafe extern "C" fn typeof(long)]: unsigned long [PERF_MAX_TRACE_SIZE / sizeof!(unsigned) -> typedef {
    typedef typeof(unsigned long [PERF_MAX_TRACE_SIZE / sizeof!(unsigned long)])
    perf_trace_t;
// Count the events in use (per event id, not per instance)
    static int	total_ref_count;
#[no_mangle]
pub unsafe extern "C" fn perf_trace_event_perm(tp_event: *mut trace_event_call, p_event: *mut perf_event) -> c_int {
    let mut ret = 0;
    if (tp_event.perf_perm) {
    ret = tp_event.perf_perm(tp_event, p_event);
    if (ret) {
    return ret;
    }
    }
//
// We checked and allowed to create parent,
// allow children without checking.
//
    if (p_event.parent) {
    return 0;
    }
//
// It's ok to check current process (owner) permissions in here,
// because code below is called only via perf_event_open syscall.
//
// The ftrace function trace is allowed only for root.
    if (ftrace_event_is_function(tp_event)) {
    ret = perf_allow_tracepoint();
    if (ret) {
    return ret;
    }
    if (!is_sampling_event(p_event)) {
    return 0;
    }
//
// We don't allow user space callchains for  function trace
// event, due to issues with page faults while tracing page
// fault handler and its overall trickiness nature.
//
    if (!p_event.attr.exclude_callchain_user) {
    return -EINVAL;
    }
//
// Same reason to disable user stack dump as for user space
// callchains above.
//
    if (p_event.attr.sample_type & PERF_SAMPLE_STACK_USER) {
    return -EINVAL;
    }
    }
// No tracing, just counting, so no obvious leak
    if (!(p_event.attr.sample_type & PERF_SAMPLE_RAW)) {
    return 0;
    }
// Some events are ok to be traced by non-root users...
    if (p_event.attach_state == PERF_ATTACH_TASK) {
    if (tp_event.flags & TRACE_EVENT_FL_CAP_ANY) {
    return 0;
    }
    }
//
// ...otherwise raw tracepoint data can be a severe data leak,
// only allow root to have these.
//
    ret = perf_allow_tracepoint();
    if (ret) {
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_trace_event_reg(tp_event: *mut trace_event_call, p_event: *mut perf_event) -> c_int {
    let mut list = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    let mut cpu = 0;
    p_event.tp_event = tp_event;
    if (tp_event.perf_refcount++ > 0) {
    return 0;
    }
    list = alloc_percpu(hlist_head);
    if (!list) {
// goto;
    }
    for_each_possible_cpu(cpu) {
    INIT_HLIST_HEAD(per_cpu_ptr(list, cpu));
    }
    tp_event.perf_events = list;
    if (!total_ref_count) {
    let mut buf = core::ptr::null_mut();
    let mut i = 0;
    while (i < PERF_NR_CONTEXTS) {
    buf = alloc_percpu(perf_trace_t);
    if (!buf) {
// goto;
    }
    perf_trace_buf[i] = buf;
    }
    }
    ret = tp_event.class.reg(tp_event, TRACE_REG_PERF_REGISTER, core::ptr::null_mut());
    if (ret) {
// goto;
    }
    total_ref_count += 1;
    return 0;
// label;
    if (!total_ref_count) {
    let mut i = 0;
    while (i < PERF_NR_CONTEXTS) {
    free_percpu(perf_trace_buf[i]);
    perf_trace_buf[i] = core::ptr::null_mut();
    }
    }
    if (!--tp_event.perf_refcount) {
    free_percpu(tp_event.perf_events);
    tp_event.perf_events = core::ptr::null_mut();
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn perf_trace_event_unreg(p_event: *mut perf_event) {
    let mut tp_event = p_event.tp_event;
    let mut i = 0;
    if (--tp_event.perf_refcount > 0) {
    return;
    }
    tp_event.class.reg(tp_event, TRACE_REG_PERF_UNREGISTER, core::ptr::null_mut());
//
// Ensure our callback won't be called anymore. The buffers
// will be freed after that.
//
    tracepoint_synchronize_unregister();
    free_percpu(tp_event.perf_events);
    tp_event.perf_events = core::ptr::null_mut();
    if (!--total_ref_count) {
    while (i < PERF_NR_CONTEXTS) {
    free_percpu(perf_trace_buf[i]);
    perf_trace_buf[i] = core::ptr::null_mut();
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn perf_trace_event_open(p_event: *mut perf_event) -> c_int {
    let mut tp_event = p_event.tp_event;
    return tp_event.class.reg(tp_event, TRACE_REG_PERF_OPEN, p_event);
    }
#[no_mangle]
unsafe extern "C" fn perf_trace_event_close(p_event: *mut perf_event) {
    let mut tp_event = p_event.tp_event;
    tp_event.class.reg(tp_event, TRACE_REG_PERF_CLOSE, p_event);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_trace_event_init(tp_event: *mut trace_event_call, p_event: *mut perf_event) -> c_int {
    let mut ret = 0;
    ret = perf_trace_event_perm(tp_event, p_event);
    if (ret) {
    return ret;
    }
    ret = perf_trace_event_reg(tp_event, p_event);
    if (ret) {
    return ret;
    }
    ret = perf_trace_event_open(p_event);
    if (ret) {
    perf_trace_event_unreg(p_event);
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_trace_init(p_event: *mut perf_event) -> c_int {
pub static mut tp_event: *mut c_void = core::ptr::null_mut();
pub static mut event_id: u64 = 0;
pub static mut ret: c_int = 0;
    mutex_lock(&event_mutex);
    list_for_each_entry(tp_event, &ftrace_events, list) {
    if (tp_event.event.type == event_id &&
    tp_event.class && tp_event.class.reg &&
    trace_event_try_get_ref(tp_event)) {
    ret = perf_trace_event_init(tp_event, p_event);
    if (ret) {
    trace_event_put_ref(tp_event);
    }
    break;
    }
    }
    mutex_unlock(&event_mutex);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_trace_destroy(p_event: *mut perf_event) {
    mutex_lock(&event_mutex);
    perf_trace_event_close(p_event);
    perf_trace_event_unreg(p_event);
    trace_event_put_ref(p_event.tp_event);
    mutex_unlock(&event_mutex);
    }

#[no_mangle]
pub unsafe extern "C" fn perf_kprobe_init(p_event: *mut perf_event, is_retprobe: bool) -> c_int {
    let mut ret = 0;
    let mut func = core::ptr::null_mut();
pub static mut tp_event: *mut c_void = core::ptr::null_mut();
    if (p_event.attr.kprobe_func) {
    func = strndup_user(u64_to_user_ptr(p_event.attr.kprobe_func),
    KSYM_NAME_LEN);
    if (IS_ERR(func)) {
    ret = PTR_ERR(func);
    return (ret == -EINVAL) ? -E2BIG : ret;
    }
    if (func[0] == '\0') {
    kfree(func);
    func = core::ptr::null_mut();
    }
    }
    tp_event = create_local_trace_kprobe(
    func, (unsigned long)(p_event.attr.kprobe_addr),
    p_event.attr.probe_offset, is_retprobe);
    if (IS_ERR(tp_event)) {
    ret = PTR_ERR(tp_event);
// goto;
    }
    mutex_lock(&event_mutex);
    ret = perf_trace_event_init(tp_event, p_event);
    if (ret) {
    destroy_local_trace_kprobe(tp_event);
    }
    mutex_unlock(&event_mutex);
// label;
    kfree(func);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_kprobe_destroy(p_event: *mut perf_event) {
    mutex_lock(&event_mutex);
    perf_trace_event_close(p_event);
    perf_trace_event_unreg(p_event);
    trace_event_put_ref(p_event.tp_event);
    mutex_unlock(&event_mutex);
    destroy_local_trace_kprobe(p_event.tp_event);
    }

#[no_mangle]
pub unsafe extern "C" fn perf_uprobe_init(p_event: *mut perf_event, ref_ctr_offset: c_ulong, is_retprobe: bool) -> c_int {
    let mut ret = 0;
    let mut path = core::ptr::null_mut();
pub static mut tp_event: *mut c_void = core::ptr::null_mut();
    if (!p_event.attr.uprobe_path) {
    return -EINVAL;
    }
    path = strndup_user(u64_to_user_ptr(p_event.attr.uprobe_path),
    PATH_MAX);
    if (IS_ERR(path)) {
    ret = PTR_ERR(path);
    return (ret == -EINVAL) ? -E2BIG : ret;
    }
    if (path[0] == '\0') {
    ret = -EINVAL;
// goto;
    }
    tp_event = create_local_trace_uprobe(path, p_event.attr.probe_offset,
    ref_ctr_offset, is_retprobe);
    if (IS_ERR(tp_event)) {
    ret = PTR_ERR(tp_event);
// goto;
    }
//
// local trace_uprobe need to hold event_mutex to call
// uprobe_buffer_enable() and uprobe_buffer_disable().
// event_mutex is not required for local trace_kprobes.
//
    mutex_lock(&event_mutex);
    ret = perf_trace_event_init(tp_event, p_event);
    if (ret) {
    destroy_local_trace_uprobe(tp_event);
    }
    mutex_unlock(&event_mutex);
// label;
    kfree(path);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_uprobe_destroy(p_event: *mut perf_event) {
    mutex_lock(&event_mutex);
    perf_trace_event_close(p_event);
    perf_trace_event_unreg(p_event);
    trace_event_put_ref(p_event.tp_event);
    mutex_unlock(&event_mutex);
    destroy_local_trace_uprobe(p_event.tp_event);
    }

#[no_mangle]
pub unsafe extern "C" fn perf_trace_add(p_event: *mut perf_event, flags: c_int) -> c_int {
    let mut tp_event = p_event.tp_event;
    let mut hwc = &p_event.hw;
    if (!(flags & PERF_EF_START)) {
    p_event.hw.state = PERF_HES_STOPPED;
    }
    if (is_sampling_event(p_event)) {
    hwc.last_period = hwc.sample_period;
    perf_swevent_set_period(p_event);
    }
//
// If TRACE_REG_PERF_ADD returns false; no custom action was performed
// and we need to take the default action of enqueueing our event on
// the right per-cpu hlist.
//
    if (!tp_event.class.reg(tp_event, TRACE_REG_PERF_ADD, p_event)) {
    let mut pcpu_list = core::ptr::null_mut();
pub static mut list: *mut c_void = core::ptr::null_mut();
    pcpu_list = tp_event.perf_events;
    if (WARN_ON_ONCE!(!pcpu_list)) {
    return -EINVAL;
    }
    list = this_cpu_ptr(pcpu_list);
    hlist_add_head_rcu(&p_event.hlist_entry, list);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_trace_del(p_event: *mut perf_event, flags: c_int) {
    let mut tp_event = p_event.tp_event;
//
// If TRACE_REG_PERF_DEL returns false; no custom action was performed
// and we need to take the default action of dequeueing our event from
// the right per-cpu hlist.
//
    if (!tp_event.class.reg(tp_event, TRACE_REG_PERF_DEL, p_event)) {
    hlist_del_rcu(&p_event.hlist_entry);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn perf_trace_buf_alloc(size: c_int, regs: *mut *mut pt_regs, rctxp: *mut c_int) -> *mut c_void {
pub static mut raw_data: *mut c_void = core::ptr::null_mut();
    let mut rctx = 0;
    BUILD_BUG_ON!(PERF_MAX_TRACE_SIZE % sizeof!(unsigned long));
    if (WARN_ONCE(size > PERF_MAX_TRACE_SIZE,
    "perf buffer not large enough, wanted %d, have %d",
    size, PERF_MAX_TRACE_SIZE)) {
    return core::ptr::null_mut();
    }
// rctxp = rctx = perf_swevent_get_recursion_context();
    if (rctx < 0) {
    return core::ptr::null_mut();
    }
    if (regs) {
// regs = this_cpu_ptr(&__perf_regs[rctx]);
    }
    raw_data = this_cpu_ptr(perf_trace_buf[rctx]);
// zero the dead bytes from align to not leak stack to user
    memset(&raw_data[size - sizeof!(u64)], 0, sizeof!(u64));
    return raw_data;
    }
    EXPORT_SYMBOL_GPL(perf_trace_buf_alloc);
    NOKPROBE_SYMBOL(perf_trace_buf_alloc);
#[no_mangle]
pub unsafe extern "C" fn perf_trace_buf_update(record: *mut c_void, type: u16) {
    let mut entry = record;
    tracing_generic_entry_update(entry, type, tracing_gen_ctx());
    }
    NOKPROBE_SYMBOL(perf_trace_buf_update);

#[no_mangle]
pub unsafe extern "C" fn perf_ftrace_function_call(ip: c_ulong, parent_ip: c_ulong, ops: *mut ftrace_ops, fregs: *mut ftrace_regs) {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut head: usize = 0;
pub static mut regs: usize = 0;
    let mut rctx = 0;
    let mut bit = 0;
    if (!rcu_is_watching()) {
    return;
    }
    bit = ftrace_test_recursion_trylock(ip, parent_ip);
    if (bit < 0) {
    return;
    }
    if ((unsigned long)ops.private != smp_processor_id()) {
// goto;
    }
    event = container_of!(ops, perf_event, ftrace_ops);
//
// @event->hlist entry is NULL (per INIT_HLIST_NODE), and all
// the perf code does is hlist_for_each_entry_rcu(), so we can
// get away with simply setting the @head.first pointer in order
// to create a singular list.
//
    head.first = &event.hlist_entry;

    sizeof!(u64)) - sizeof!(u32))
    BUILD_BUG_ON!(ENTRY_SIZE > PERF_MAX_TRACE_SIZE);
    memset(&regs, 0, sizeof!(regs));
    perf_fetch_caller_regs(&regs);
    entry = perf_trace_buf_alloc(ENTRY_SIZE, core::ptr::null_mut(), &rctx);
    if (!entry) {
// goto;
    }
    entry.ip = ip;
    entry.parent_ip = parent_ip;
    perf_trace_buf_submit(entry, ENTRY_SIZE, rctx, TRACE_FN,
    1, &regs, &head, core::ptr::null_mut());
// label;
    ftrace_test_recursion_unlock(bit);

    }
#[no_mangle]
unsafe extern "C" fn perf_ftrace_function_register(event: *mut perf_event) -> c_int {
    let mut ops = &event.ftrace_ops;
    ops.func    = perf_ftrace_function_call;
    ops.private = (unsigned long)nr_cpu_ids;
    return register_ftrace_function(ops);
    }
#[no_mangle]
unsafe extern "C" fn perf_ftrace_function_unregister(event: *mut perf_event) -> c_int {
    let mut ops = &event.ftrace_ops;
pub static mut ret: c_int = 0;
//
// Perf will call this unconditionally even if the ops is not
// enabled. The unregister_ftrace_function() will warn if called
// when not enabled. Just bypass the unregistering if ops isn't
// enabled here.
//
    if (ops.flags & FTRACE_OPS_FL_ENABLED) {
    ret = unregister_ftrace_function(ops);
    }
    ftrace_free_filter(ops);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_ftrace_event_register(call: *mut trace_event_call, type: trace_reg, data: *mut c_void) -> c_int {
    let mut event = data;
    match (type) {
    TRACE_REG_REGISTER => {
    }
    TRACE_REG_UNREGISTER => {
    // break;
    }
    TRACE_REG_PERF_REGISTER => {
    }
    TRACE_REG_PERF_UNREGISTER => {
    return 0;
    }
    TRACE_REG_PERF_OPEN => {
    return perf_ftrace_function_register(data);
    }
    TRACE_REG_PERF_CLOSE => {
    return perf_ftrace_function_unregister(data);
    }
    TRACE_REG_PERF_ADD => {
    event.ftrace_ops.private = (unsigned long)smp_processor_id();
    return 1;
    }
    TRACE_REG_PERF_DEL => {
    event.ftrace_ops.private = (unsigned long)nr_cpu_ids;
    return 1;
    }
    }
    return -EINVAL;
    }
}

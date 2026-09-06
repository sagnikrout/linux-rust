//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_remote.c
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
// Copyright (C) 2025 - Google LLC
// Author: Vincent Donnefort <vdonnefort@google.com>
//

pub const TRACEFS_MODE_WRITE: c_int = 0640;
pub const TRACEFS_MODE_READ: c_int = 0440;
    enum tri_type {
    TRI_CONSUMING,
    TRI_NONCONSUMING,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_remote_iterator {
    pub remote: *mut trace_remote,
    pub seq: trace_seq,
    pub poll_work: delayed_work,
    pub lost_events: c_ulong,
    pub ts: u64,
    pub rb_iter: *mut ring_buffer_iter,
    pub rb_iters: *mut ring_buffer_iter,
    pub evt: *mut remote_event_hdr,
    pub cpu: c_int,
    pub evt_cpu: c_int,
    pub pos: loff_t,
    pub type: tri_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_remote {
    pub cbs: *mut trace_remote_callbacks,
    pub priv: *mut c_void,
    pub trace_buffer: *mut trace_buffer,
    pub trace_buffer_desc: *mut trace_buffer_desc,
    pub dentry: *mut dentry,
    pub eventfs: *mut eventfs_inode,
    pub events: *mut remote_event,
    pub nr_events: c_ulong,
    pub trace_buffer_size: c_ulong,
    pub rb_remote: ring_buffer_remote,
    pub lock: mutex,
    pub reader_lock: rw_semaphore,
    pub pcpu_reader_locks: *mut rw_semaphore,
    pub nr_readers: c_uint,
    pub poll_ms: c_uint,
    pub tracing_on: bool,
}

#[no_mangle]
unsafe extern "C" fn trace_remote_loaded(remote: *mut trace_remote) -> bool {
    return !!remote.trace_buffer;
    }
#[no_mangle]
unsafe extern "C" fn trace_remote_load(remote: *mut trace_remote) -> c_int {
    let mut rb_remote = &remote.rb_remote;
pub static mut desc: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&remote.lock);
    if (trace_remote_loaded(remote)) {
    return 0;
    }
    desc = remote.cbs.load_trace_buffer(remote.trace_buffer_size, remote.priv);
    if (IS_ERR(desc)) {
    return PTR_ERR(desc);
    }
    rb_remote.desc = desc;
    rb_remote.swap_reader_page = remote.cbs.swap_reader_page;
    rb_remote.priv = remote.priv;
    rb_remote.reset = remote.cbs.reset;
    remote.trace_buffer = ring_buffer_alloc_remote(rb_remote);
    if (!remote.trace_buffer) {
    remote.cbs.unload_trace_buffer(desc, remote.priv);
    return -ENOMEM;
    }
    remote.trace_buffer_desc = desc;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_remote_try_unload(remote: *mut trace_remote) {
    lockdep_assert_held(&remote.lock);
    if (!trace_remote_loaded(remote)) {
    return;
    }
// The buffer is being read or writable
    if (remote.nr_readers || remote.tracing_on) {
    return;
    }
// The buffer has readable data
    if (!ring_buffer_empty(remote.trace_buffer)) {
    return;
    }
    ring_buffer_free(remote.trace_buffer);
    remote.trace_buffer = core::ptr::null_mut();
    remote.cbs.unload_trace_buffer(remote.trace_buffer_desc, remote.priv);
    }
#[no_mangle]
unsafe extern "C" fn trace_remote_enable_tracing(remote: *mut trace_remote) -> c_int {
    let mut ret = 0;
    lockdep_assert_held(&remote.lock);
    if (remote.tracing_on) {
    return 0;
    }
    ret = trace_remote_load(remote);
    if (ret) {
    return ret;
    }
    ret = remote.cbs.enable_tracing(true, remote.priv);
    if (ret) {
    trace_remote_try_unload(remote);
    return ret;
    }
    remote.tracing_on = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_remote_disable_tracing(remote: *mut trace_remote) -> c_int {
    let mut ret = 0;
    lockdep_assert_held(&remote.lock);
    if (!remote.tracing_on) {
    return 0;
    }
    ret = remote.cbs.enable_tracing(false, remote.priv);
    if (ret) {
    return ret;
    }
    ring_buffer_poll_remote(remote.trace_buffer, RING_BUFFER_ALL_CPUS);
    remote.tracing_on = false;
    trace_remote_try_unload(remote);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_remote_reset(remote: *mut trace_remote, cpu: c_int) {
    lockdep_assert_held(&remote.lock);
    if (!trace_remote_loaded(remote)) {
    return;
    }
    if (cpu == RING_BUFFER_ALL_CPUS) {
    ring_buffer_reset(remote.trace_buffer);
    }
    else {
    ring_buffer_reset_cpu(remote.trace_buffer, cpu);
    }
    trace_remote_try_unload(remote);
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_on_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut seq = filp.private_data;
    let mut remote = seq.private;
    let mut val = 0;
    let mut ret = 0;
    ret = kstrtoul_from_user(ubuf, cnt, 10, &val);
    if (ret) {
    return ret;
    }
    guard(mutex)(&remote.lock);
    ret = val ? trace_remote_enable_tracing(remote) : trace_remote_disable_tracing(remote);
    if (ret) {
    return ret;
    }
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn tracing_on_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    let mut remote = s.private;
    seq_printf(s, "%d\n", remote.tracing_on);
    return 0;
    }
pub static mut tracing_on: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn buffer_size_kb_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut seq = filp.private_data;
    let mut remote = seq.private;
    let mut val = 0;
    let mut ret = 0;
    ret = kstrtoul_from_user(ubuf, cnt, 10, &val);
    if (ret) {
    return ret;
    }
// KiB to Bytes
    if (!val || check_shl_overflow(val, 10, &val)) {
    return -EINVAL;
    }
    guard(mutex)(&remote.lock);
    if (trace_remote_loaded(remote)) {
    return -EBUSY;
    }
    remote.trace_buffer_size = val;
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn buffer_size_kb_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    let mut remote = s.private;
    seq_printf(s, "%lu (%s)\n", remote.trace_buffer_size >> 10,
    trace_remote_loaded(remote) ? "loaded" : "unloaded");
    return 0;
    }
pub static mut buffer_size_kb: usize = 0;
#[no_mangle]
unsafe extern "C" fn trace_remote_get(remote: *mut trace_remote, cpu: c_int) -> c_int {
    let mut ret = 0;
    if (remote.nr_readers == UINT_MAX) {
    return -EBUSY;
    }
    ret = trace_remote_load(remote);
    if (ret) {
    return ret;
    }
    if (cpu != RING_BUFFER_ALL_CPUS && !remote.pcpu_reader_locks) {
    let mut lock_cpu = 0;
    remote.pcpu_reader_locks = kzalloc_objs(*remote.pcpu_reader_locks,
    nr_cpu_ids);
    if (!remote.pcpu_reader_locks) {
    trace_remote_try_unload(remote);
    return -ENOMEM;
    }
    for_each_possible_cpu(lock_cpu) {
    init_rwsem(&remote.pcpu_reader_locks[lock_cpu]);
    }
    }
    remote.nr_readers += 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_remote_put(remote: *mut trace_remote) {
    if (WARN_ON!(!remote.nr_readers)) {
    return;
    }
    remote.nr_readers -= 1;
    if (remote.nr_readers) {
    return;
    }
    kfree(remote.pcpu_reader_locks);
    remote.pcpu_reader_locks = core::ptr::null_mut();
    trace_remote_try_unload(remote);
    }
#[no_mangle]
unsafe extern "C" fn trace_remote_has_cpu(remote: *mut trace_remote, cpu: c_int) -> bool {
    if (cpu == RING_BUFFER_ALL_CPUS) {
    return true;
    }
    return ring_buffer_poll_remote(remote.trace_buffer, cpu) == 0;
    }
#[no_mangle]
unsafe extern "C" fn __poll_remote(work: *mut work_struct) {
    let mut dwork = to_delayed_work(work);
pub static mut iter: *mut c_void = core::ptr::null_mut();
    iter = container_of!(dwork, trace_remote_iterator, poll_work);
    ring_buffer_poll_remote(iter.remote.trace_buffer, iter.cpu);
    schedule_delayed_work(work,
    msecs_to_jiffies(iter.remote.poll_ms));
    }
#[no_mangle]
unsafe extern "C" fn __free_ring_buffer_iter(iter: *mut trace_remote_iterator, cpu: c_int) {
    if (cpu != RING_BUFFER_ALL_CPUS) {
    ring_buffer_read_finish(iter.rb_iter);
    return;
    }
    for_each_possible_cpu(cpu) {
    if (iter.rb_iters[cpu]) {
    ring_buffer_read_finish(iter.rb_iters[cpu]);
    }
    }
    kfree(iter.rb_iters);
    }
#[no_mangle]
unsafe extern "C" fn __alloc_ring_buffer_iter(iter: *mut trace_remote_iterator, cpu: c_int) -> c_int {
    if (cpu != RING_BUFFER_ALL_CPUS) {
    iter.rb_iter = ring_buffer_read_start(iter.remote.trace_buffer, cpu, GFP_KERNEL);
    return iter.rb_iter ? 0 : -ENOMEM;
    }
    iter.rb_iters = kzalloc_objs(*iter.rb_iters, nr_cpu_ids);
    if (!iter.rb_iters) {
    return -ENOMEM;
    }
    for_each_possible_cpu(cpu) {
    iter.rb_iters[cpu] = ring_buffer_read_start(iter.remote.trace_buffer, cpu,
    GFP_KERNEL);
    if (!iter.rb_iters[cpu]) {
// This CPU isn't part of trace_buffer. Skip it
    if (!trace_remote_has_cpu(iter.remote, cpu)) {
    continue;
    }
    __free_ring_buffer_iter(iter, RING_BUFFER_ALL_CPUS);
    return -ENOMEM;
    }
    }
    return 0;
    }
    static struct trace_remote_iterator
// trace_remote_iter(trace_remote *remote, int cpu, enum tri_type type)
    {
    let mut iter = core::ptr::null_mut();
    let mut ret = 0;
    lockdep_assert_held(&remote.lock);
    if (type == TRI_NONCONSUMING && !trace_remote_loaded(remote)) {
    return core::ptr::null_mut();
    }
    ret = trace_remote_get(remote, cpu);
    if (ret) {
    return ERR_PTR(ret);
    }
    if (!trace_remote_has_cpu(remote, cpu)) {
    ret = -ENODEV;
// goto;
    }
    iter = kzalloc_obj(*iter);
    if (iter) {
    iter.remote = remote;
    iter.cpu = cpu;
    iter.type = type;
    trace_seq_init(&iter.seq);
    match (type) {
    TRI_CONSUMING => {
    ring_buffer_poll_remote(remote.trace_buffer, cpu);
    INIT_DELAYED_WORK(&iter.poll_work, __poll_remote);
    schedule_delayed_work(&iter.poll_work, msecs_to_jiffies(remote.poll_ms));
    // break;
    }
    TRI_NONCONSUMING => {
    ret = __alloc_ring_buffer_iter(iter, cpu);
    // break;
    }
    }
    if (ret) {
// goto;
    }
    return iter;
    }
    ret = -ENOMEM;
// label;
    kfree(iter);
    trace_remote_put(remote);
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn trace_remote_iter_free(iter: *mut trace_remote_iterator) {
pub static mut remote: *mut c_void = core::ptr::null_mut();
    if (!iter) {
    return;
    }
    remote = iter.remote;
    lockdep_assert_held(&remote.lock);
    match (iter.type) {
    TRI_CONSUMING => {
    cancel_delayed_work_sync(&iter.poll_work);
    // break;
    }
    TRI_NONCONSUMING => {
    __free_ring_buffer_iter(iter, iter.cpu);
    // break;
    }
    }
    kfree(iter);
    trace_remote_put(remote);
    }
#[no_mangle]
unsafe extern "C" fn trace_remote_iter_read_start(iter: *mut trace_remote_iterator) {
    let mut remote = iter.remote;
pub static mut cpu: c_int = 0;
// Acquire global reader lock
    if (cpu == RING_BUFFER_ALL_CPUS && iter.type == TRI_CONSUMING) {
    down_write(&remote.reader_lock);
    }
    else {
    down_read(&remote.reader_lock);
    }
    if (cpu == RING_BUFFER_ALL_CPUS) {
    return;
    }
//
// No need for the remote lock here, iter holds a reference on
// remote->nr_readers
//
// Get the per-CPU one
    if (WARN_ON_ONCE!(!remote.pcpu_reader_locks)) {
    return;
    }
    if (iter.type == TRI_CONSUMING) {
    down_write(&remote.pcpu_reader_locks[cpu]);
    }
    else {
    down_read(&remote.pcpu_reader_locks[cpu]);
    }
    }
#[no_mangle]
unsafe extern "C" fn trace_remote_iter_read_finished(iter: *mut trace_remote_iterator) {
    let mut remote = iter.remote;
pub static mut cpu: c_int = 0;
// Release per-CPU reader lock
    if (cpu != RING_BUFFER_ALL_CPUS) {
//
// No need for the remote lock here, iter holds a reference on
// remote->nr_readers
//
    if (iter.type == TRI_CONSUMING) {
    up_write(&remote.pcpu_reader_locks[cpu]);
    }
    else {
    up_read(&remote.pcpu_reader_locks[cpu]);
    }
    }
// Release global reader lock
    if (cpu == RING_BUFFER_ALL_CPUS && iter.type == TRI_CONSUMING) {
    up_write(&remote.reader_lock);
    }
    else {
    up_read(&remote.reader_lock);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __get_rb_iter(iter: *mut trace_remote_iterator, cpu: c_int) -> *mut c_void {
    return iter.cpu != RING_BUFFER_ALL_CPUS ? iter.rb_iter : iter.rb_iters[cpu];
    }
#[no_mangle]
pub unsafe extern "C" fn __peek_event(iter: *mut trace_remote_iterator, cpu: c_int, ts: *mut u64, lost_events: *mut c_ulong) -> *mut c_void {
pub static mut rb_evt: *mut c_void = core::ptr::null_mut();
pub static mut rb_iter: *mut c_void = core::ptr::null_mut();
    match (iter.type) {
    TRI_CONSUMING => {
    return ring_buffer_peek(iter.remote.trace_buffer, cpu, ts, lost_events);
    }
    TRI_NONCONSUMING => {
    rb_iter = __get_rb_iter(iter, cpu);
    if (!rb_iter) {
    return core::ptr::null_mut();
    }
    rb_evt = ring_buffer_iter_peek(rb_iter, ts);
    if (!rb_evt) {
    return core::ptr::null_mut();
    }
// lost_events = ring_buffer_iter_dropped(rb_iter);
    return rb_evt;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn trace_remote_iter_read_event(iter: *mut trace_remote_iterator) -> bool {
    let mut trace_buffer = iter.remote.trace_buffer;
pub static mut rb_evt: *mut c_void = core::ptr::null_mut();
pub static mut cpu: c_int = 0;
    if (cpu != RING_BUFFER_ALL_CPUS) {
    if (ring_buffer_empty_cpu(trace_buffer, cpu)) {
    return false;
    }
    rb_evt = __peek_event(iter, cpu, &iter.ts, &iter.lost_events);
    if (!rb_evt) {
    return false;
    }
    iter.evt_cpu = cpu;
    iter.evt = ring_buffer_event_data(rb_evt);
    return true;
    }
    iter.ts = U64_MAX;
    for_each_possible_cpu(cpu) {
    let mut lost_events = 0;
    let mut ts = 0;
    if (ring_buffer_empty_cpu(trace_buffer, cpu)) {
    continue;
    }
    rb_evt = __peek_event(iter, cpu, &ts, &lost_events);
    if (!rb_evt) {
    continue;
    }
    if (ts >= iter.ts) {
    continue;
    }
    iter.ts = ts;
    iter.evt_cpu = cpu;
    iter.evt = ring_buffer_event_data(rb_evt);
    iter.lost_events = lost_events;
    }
    return iter.ts != U64_MAX;
    }
#[no_mangle]
unsafe extern "C" fn trace_remote_iter_move(iter: *mut trace_remote_iterator) {
    let mut trace_buffer = iter.remote.trace_buffer;
    match (iter.type) {
    TRI_CONSUMING => {
    ring_buffer_consume(trace_buffer, iter.evt_cpu, core::ptr::null_mut(), core::ptr::null_mut());
    // break;
    }
    TRI_NONCONSUMING => {
    ring_buffer_iter_advance(__get_rb_iter(iter, iter.evt_cpu));
    // break;
    }
    }
    }
// forward_decl: trace_remote_find_event;
#[no_mangle]
unsafe extern "C" fn trace_remote_iter_print_event(iter: *mut trace_remote_iterator) -> c_int {
pub static mut evt: *mut c_void = core::ptr::null_mut();
    let mut usecs_rem = 0;
pub static mut ts: u64 = 0;
    if (iter.lost_events) {
    trace_seq_printf(&iter.seq, "CPU:%d [LOST %lu EVENTS]\n",
    iter.evt_cpu, iter.lost_events);
    }
    do_div(ts, 1000);
    usecs_rem = do_div(ts, USEC_PER_SEC);
    trace_seq_printf(&iter.seq, "[%03d]\t%5llu.%06lu: ", iter.evt_cpu,
    ts, usecs_rem);
    evt = trace_remote_find_event(iter.remote, iter.evt.id);
    if (!evt) {
    trace_seq_printf(&iter.seq, "UNKNOWN id=%d\n", iter.evt.id);
    }
    else {
    evt.print(iter.evt, &iter.seq);
    }
    return trace_seq_has_overflowed(&iter.seq) ? -EOVERFLOW : 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_pipe_open(inode: *mut inode, filp: *mut file) -> c_int {
    let mut remote = inode.i_private;
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut cpu: c_int = 0;
    guard(mutex)(&remote.lock);
    iter = trace_remote_iter(remote, cpu, TRI_CONSUMING);
    if (IS_ERR(iter)) {
    return PTR_ERR(iter);
    }
    filp.private_data = iter;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_pipe_release(inode: *mut inode, filp: *mut file) -> c_int {
    let mut iter = filp.private_data;
    let mut remote = iter.remote;
    guard(mutex)(&remote.lock);
    trace_remote_iter_free(iter);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_pipe_read(filp: *mut file, ubuf: *mut char , cnt: usize, ppos: *mut loff_t) -> isize {
    let mut iter = filp.private_data;
    let mut trace_buffer = iter.remote.trace_buffer;
    let mut ret = 0;
// label;
    ret = trace_seq_to_user(&iter.seq, ubuf, cnt);
    if (ret != -EBUSY) {
    return ret;
    }
    trace_seq_init(&iter.seq);
    ret = ring_buffer_wait(trace_buffer, iter.cpu, 0, core::ptr::null_mut(), core::ptr::null_mut());
    if (ret < 0) {
    return ret;
    }
    trace_remote_iter_read_start(iter);
    while (trace_remote_iter_read_event(iter)) {
pub static mut prev_len: c_int = 0;
    if (trace_remote_iter_print_event(iter)) {
    iter.seq.seq.len = prev_len;
    break;
    }
    trace_remote_iter_move(iter);
    }
    trace_remote_iter_read_finished(iter);
// goto;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn trace_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut iter = m.private;
    ++*pos;
    if (!iter || !trace_remote_iter_read_event(iter)) {
    return core::ptr::null_mut();
    }
    trace_remote_iter_move(iter);
    iter.pos += 1;
    return iter;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut iter = m.private;
    let mut i = 0;
    if (!iter) {
    return core::ptr::null_mut();
    }
    trace_remote_iter_read_start(iter);
    if (!*pos) {
    iter.pos = -1;
    return trace_next(m, core::ptr::null_mut(), &i);
    }
    i = iter.pos;
    while (i < *pos) {
    iter = trace_next(m, core::ptr::null_mut(), &i);
    if (!iter) {
    return core::ptr::null_mut();
    }
    }
    return iter;
    }
#[no_mangle]
unsafe extern "C" fn trace_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut iter = v;
    trace_seq_init(&iter.seq);
    if (trace_remote_iter_print_event(iter)) {
    seq_printf(m, "[EVENT %d PRINT TOO BIG]\n", iter.evt.id);
    return 0;
    }
    return trace_print_seq(m, &iter.seq);
    }
#[no_mangle]
unsafe extern "C" fn trace_stop(m: *mut seq_file, v: *mut c_void) {
    let mut iter = m.private;
    if (iter) {
    trace_remote_iter_read_finished(iter);
    }
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn trace_open(inode: *mut inode, filp: *mut file) -> c_int {
    let mut remote = inode.i_private;
    let mut iter = core::ptr::null_mut();
pub static mut cpu: c_int = 0;
    let mut ret = 0;
    if (!(filp.f_mode & FMODE_READ)) {
    return 0;
    }
    guard(mutex)(&remote.lock);
    iter = trace_remote_iter(remote, cpu, TRI_NONCONSUMING);
    if (IS_ERR(iter)) {
    return PTR_ERR(iter);
    }
    ret = seq_open(filp, &trace_sops);
    if (ret) {
    trace_remote_iter_free(iter);
    return ret;
    }
    (filp.private_data).private = iter;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_release(inode: *mut inode, filp: *mut file) -> c_int {
pub static mut iter: *mut c_void = core::ptr::null_mut();
    if (!(filp.f_mode & FMODE_READ)) {
    return 0;
    }
    iter = (filp.private_data).private;
    seq_release(inode, filp);
    if (!iter) {
    return 0;
    }
    guard(mutex)(&iter.remote.lock);
    trace_remote_iter_free(iter);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_write(filp: *mut file, ubuf: *const char , cnt: usize, ppos: *mut loff_t) -> isize {
    let mut inode = file_inode(filp);
    let mut remote = inode.i_private;
pub static mut cpu: c_int = 0;
    guard(mutex)(&remote.lock);
    trace_remote_reset(remote, cpu);
    return cnt;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn trace_remote_init_tracefs(name: *const c_char, remote: *mut trace_remote) -> c_int {
    let mut remote_d = core::ptr::null_mut();
    let mut percpu_d = core::ptr::null_mut();
    let mut d = core::ptr::null_mut();
pub static mut root: *mut c_void = core::ptr::null_mut();
pub static mut lock: usize = 0;
pub static mut root_inited: bool = false;
    let mut cpu = 0;
    guard(mutex)(&lock);
    if (!root) {
    root = tracefs_create_dir(TRACEFS_DIR, core::ptr::null_mut());
    if (!root) {
    pr_err!("Failed to create tracefs dir "TRACEFS_DIR"\n");
    return -ENOMEM;
    }
    root_inited = true;
    }
    remote_d = tracefs_create_dir(name, root);
    if (!remote_d) {
    pr_err!("Failed to create tracefs dir "TRACEFS_DIR"%s/\n", name);
// goto;
    }
    d = trace_create_file("tracing_on", TRACEFS_MODE_WRITE, remote_d, remote, &tracing_on_fops);
    if (!d) {
// goto;
    }
    d = trace_create_file("buffer_size_kb", TRACEFS_MODE_WRITE, remote_d, remote,
    &buffer_size_kb_fops);
    if (!d) {
// goto;
    }
    d = trace_create_file("trace_pipe", TRACEFS_MODE_READ, remote_d, remote, &trace_pipe_fops);
    if (!d) {
// goto;
    }
    d = trace_create_file("trace", TRACEFS_MODE_WRITE, remote_d, remote, &trace_fops);
    if (!d) {
// goto;
    }
    percpu_d = tracefs_create_dir("per_cpu", remote_d);
    if (!percpu_d) {
    pr_err!("Failed to create tracefs dir "TRACEFS_DIR"%s/per_cpu/\n", name);
// goto;
    }
    for_each_possible_cpu(cpu) {
pub static mut cpu_d: *mut c_void = core::ptr::null_mut();
    char cpu_name[16];
    snprintf(cpu_name, sizeof!(cpu_name), "cpu%d", cpu);
    cpu_d = tracefs_create_dir(cpu_name, percpu_d);
    if (!cpu_d) {
    pr_err!("Failed to create tracefs dir "TRACEFS_DIR"%s/percpu/cpu%d\n",
    name, cpu);
// goto;
    }
    d = trace_create_cpu_file("trace_pipe", TRACEFS_MODE_READ, cpu_d, remote, cpu,
    &trace_pipe_fops);
    if (!d) {
// goto;
    }
    d = trace_create_cpu_file("trace", TRACEFS_MODE_WRITE, cpu_d, remote, cpu,
    &trace_fops);
    if (!d) {
// goto;
    }
    }
    remote.dentry = remote_d;
    return 0;
// label;
    if (root_inited) {
    tracefs_remove(root);
    root = core::ptr::null_mut();
    } else {
    tracefs_remove(remote_d);
    }
    return -ENOMEM;
    }
// forward_decl: trace_remote_register_events;
//
// trace_remote_register() - Register a Tracefs remote
// @name:	Name of the remote, used for the Tracefs remotes/ directory.
// @cbs:	Set of callbacks used to control the remote.
// @priv:	Private data, passed to each callback from @cbs.
// @events:	Array of events. &remote_event.name and &remote_event.id must be
// filled by the caller.
// @nr_events:	Number of events in the @events array.
//
// A trace remote is an entity, outside of the kernel (most likely firmware or
// hypervisor) capable of writing events into a Tracefs compatible ring-buffer.
// The kernel would then act as a reader.
//
// The registered remote will be found under the Tracefs directory
// remotes/<name>.
//
// Return: 0 on success, negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn trace_remote_register(name: *mut c_char, cbs: *mut trace_remote_callbacks, priv: *mut c_void, events: *mut remote_event, nr_events: size_t) -> c_int {
pub static mut remote: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    remote = kzalloc_obj(*remote);
    if (!remote) {
    return -ENOMEM;
    }
    remote.cbs = cbs;
    remote.priv = priv;
    remote.trace_buffer_size = 7 << 10;
    remote.poll_ms = 100;
    mutex_init(&remote.lock);
    init_rwsem(&remote.reader_lock);
    if (trace_remote_init_tracefs(name, remote)) {
    kfree(remote);
    return -ENOMEM;
    }
    ret = trace_remote_register_events(name, remote, events, nr_events);
    if (ret) {
    pr_err!("Failed to register events for trace remote '%s' (%d)\n",
    name, ret);
    return ret;
    }
    ret = cbs.init ? cbs.init(remote.dentry, priv) : 0;
    if (ret) {
    pr_err!("Init failed for trace remote '%s' (%d)\n", name, ret);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(trace_remote_register);
//
// trace_remote_free_buffer() - Free trace buffer allocated with trace_remote_alloc_buffer()
// @desc:	Descriptor of the per-CPU ring-buffers, originally filled by
// trace_remote_alloc_buffer()
//
// Most likely called from &trace_remote_callbacks.unload_trace_buffer.
//
#[no_mangle]
pub unsafe extern "C" fn trace_remote_free_buffer(desc: *mut trace_buffer_desc) {
pub static mut rb_desc: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    for_each_ring_buffer_desc(rb_desc, cpu, desc) {
    let mut id = 0;
    free_page(rb_desc.meta_va);
    for (id = 0; id < rb_desc.nr_page_va; id++) {
    free_page(rb_desc.page_va[id]);
    }
    }
    }
    EXPORT_SYMBOL_GPL(trace_remote_free_buffer);
//
// trace_remote_alloc_buffer() - Dynamically allocate a trace buffer
// @desc:		Uninitialized trace_buffer_desc
// @desc_size:		Size of the trace_buffer_desc. Must be at least equal to
// trace_buffer_desc_size()
// @buffer_size:	Size in bytes of each per-CPU ring-buffer
// @cpumask:		CPUs to allocate a ring-buffer for
//
// Helper to dynamically allocate a set of pages (enough to cover @buffer_size)
// for each CPU from @cpumask and fill @desc. Most likely called from
// &trace_remote_callbacks.load_trace_buffer.
//
// Return: 0 on success, negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn trace_remote_alloc_buffer(desc: *mut trace_buffer_desc, desc_size: size_t, buffer_size: size_t, cpumask: *mut cpumask) -> c_int {
pub static mut min_desc_size: usize = 0;
pub static mut nr_pages: c_uint = 0;
pub static mut rb_desc: *mut c_void = core::ptr::null_mut();
    int cpu, ret = -ENOMEM;
    if (desc_size < min_desc_size) {
    return -EINVAL;
    }
    desc.nr_cpus = 0;
    desc.struct_len = min_desc_size;
    rb_desc = __first_ring_buffer_desc(desc);
    for_each_cpu(cpu, cpumask) {
    let mut id = 0;
    rb_desc.cpu = cpu;
    rb_desc.nr_page_va = 0;
    rb_desc.meta_va = (unsigned long)__get_free_page(GFP_KERNEL);
    if (!rb_desc.meta_va) {
// goto;
    }
    desc.nr_cpus += 1;
    while (id < nr_pages) {
    rb_desc.nr_page_va += 1;
    rb_desc.page_va[id] = (unsigned long)__get_free_page(GFP_KERNEL);
    if (!rb_desc.page_va[id]) {
// goto;
    }
    }
    rb_desc = __next_ring_buffer_desc(rb_desc);
    }
    return 0;
// label;
    trace_remote_free_buffer(desc);
    return ret;
    }
    EXPORT_SYMBOL_GPL(trace_remote_alloc_buffer);
#[no_mangle]
pub unsafe extern "C" fn trace_remote_enable_event(remote: *mut trace_remote, evt: *mut remote_event, enable: bool) -> c_int {
    let mut ret = 0;
    lockdep_assert_held(&remote.lock);
    if (evt.enabled == enable) {
    return 0;
    }
    ret = remote.cbs.enable_event(evt.id, enable, remote.priv);
    if (ret) {
    return ret;
    }
    evt.enabled = enable;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn remote_event_enable_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    let mut evt = s.private;
    seq_printf(s, "%d\n", evt.enabled);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn remote_event_enable_write(filp: *mut file, ubuf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut seq = filp.private_data;
    let mut evt = seq.private;
    let mut remote = evt.remote;
    let mut enable = 0;
    let mut ret = 0;
    ret = kstrtou8_from_user(ubuf, count, 10, &enable);
    if (ret) {
    return ret;
    }
    guard(mutex)(&remote.lock);
    ret = trace_remote_enable_event(remote, evt, enable);
    if (ret) {
    return ret;
    }
    return count;
    }
pub static mut remote_event_enable: usize = 0;
#[no_mangle]
unsafe extern "C" fn remote_event_id_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    let mut evt = s.private;
    seq_printf(s, "%d\n", evt.id);
    return 0;
    }
pub static mut remote_event_id: usize = 0;
#[no_mangle]
unsafe extern "C" fn remote_event_format_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
pub static mut offset: usize = 0;
    let mut evt = s.private;
pub static mut field: *mut c_void = core::ptr::null_mut();
    seq_printf(s, "name: %s\n", evt.name);
    seq_printf(s, "ID: %d\n", evt.id);
    seq_puts(s,
    "format:\n\tfield:unsigned short common_type;\toffset:0;\tsize:2;\tsigned:0;\n\n");
    field = &evt.fields[0];
    while (field.name) {
    seq_printf(s, "\tfield:%s %s;\toffset:%zu;\tsize:%u;\tsigned:%d;\n",
    field.type, field.name, offset, field.size,
    field.is_signed);
    offset += field.size;
    field += 1;
    }
    if (field != &evt.fields[0]) {
    seq_puts(s, "\n");
    }
    seq_printf(s, "print fmt: %s\n", evt.print_fmt);
    return 0;
    }
pub static mut remote_event_format: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn remote_event_callback(name: *mut c_char, mode: *mut umode_t, data: *mut *mut c_void, fops: *mut *mut file_operations) -> c_int {
    if (!strcmp(name, "enable")) {
// mode = TRACEFS_MODE_WRITE;
// fops = &remote_event_enable_fops;
    return 1;
    }
    if (!strcmp(name, "id")) {
// mode = TRACEFS_MODE_READ;
// fops = &remote_event_id_fops;
    return 1;
    }
    if (!strcmp(name, "format")) {
// mode = TRACEFS_MODE_READ;
// fops = &remote_event_format_fops;
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn remote_events_dir_enable_write(filp: *mut file, ubuf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut remote = file_inode(filp).i_private;
    let mut i = 0;
    let mut ret = 0;
    let mut enable = 0;
    ret = kstrtou8_from_user(ubuf, count, 10, &enable);
    if (ret) {
    return ret;
    }
    guard(mutex)(&remote.lock);
    while (i < remote.nr_events) {
    let mut evt = &remote.events[i];
    let mut eret = 0;
    eret = trace_remote_enable_event(remote, evt, enable);
//
// Save the first error and return that. Some events
// may still have been enabled, but let the user
// know that something went wrong.
//
    if (!ret && eret) {
    ret = eret;
    }
    }
    if (ret) {
    return ret;
    }
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn remote_events_dir_enable_read(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut remote = file_inode(filp).i_private;
    const char enabled_char[] = {'0', '1', 'X'};
    char enabled_str[] = " \n";
    int i, enabled = -1;
    guard(mutex)(&remote.lock);
    while (i < remote.nr_events) {
    let mut evt = &remote.events[i];
    if (enabled == -1) {
    enabled = evt.enabled;
    } else if (enabled != evt.enabled) {
    enabled = 2;
    break;
    }
    }
    enabled_str[0] = enabled_char[enabled == -1 ? 0 : enabled];
    return simple_read_from_buffer(ubuf, cnt, ppos, enabled_str, 2);
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn remote_events_dir_header_page_read(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut s: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    s = kmalloc_obj(*s);
    if (!s) {
    return -ENOMEM;
    }
    trace_seq_init(s);
    ring_buffer_print_page_header(core::ptr::null_mut(), s);
    ret = simple_read_from_buffer(ubuf, cnt, ppos, s.buffer, trace_seq_used(s));
    kfree(s);
    return ret;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn remote_events_dir_header_event_read(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
pub static mut s: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    s = kmalloc_obj(*s);
    if (!s) {
    return -ENOMEM;
    }
    trace_seq_init(s);
    ring_buffer_print_entry_header(s);
    ret = simple_read_from_buffer(ubuf, cnt, ppos, s.buffer, trace_seq_used(s));
    kfree(s);
    return ret;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn remote_events_dir_callback(name: *mut c_char, mode: *mut umode_t, data: *mut *mut c_void, fops: *mut *mut file_operations) -> c_int {
    if (!strcmp(name, "enable")) {
// mode = TRACEFS_MODE_WRITE;
// fops = &remote_events_dir_enable_fops;
    return 1;
    }
    if (!strcmp(name, "header_page")) {
// mode = TRACEFS_MODE_READ;
// fops = &remote_events_dir_header_page_fops;
    return 1;
    }
    if (!strcmp(name, "header_event")) {
// mode = TRACEFS_MODE_READ;
// fops = &remote_events_dir_header_event_fops;
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_remote_init_eventfs(remote_name: *mut c_char, remote: *mut trace_remote, evt: *mut remote_event) -> c_int {
    let mut eventfs = remote.eventfs;
pub static mut eventfs_entry: usize = 0;
pub static mut eventfs_entry: usize = 0;
pub static mut eventfs_create: bool = false;
    if (!eventfs) {
    eventfs = eventfs_create_events_dir("events", remote.dentry, dir_entries,
    ARRAY_SIZE!(dir_entries), remote);
    if (IS_ERR(eventfs)) {
    return PTR_ERR(eventfs);
    }
//
// Create similar hierarchy as local events even if a single system is supported at
// the moment
//
    eventfs = eventfs_create_dir(remote_name, eventfs, core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (IS_ERR(eventfs)) {
    return PTR_ERR(eventfs);
    }
    remote.eventfs = eventfs;
    eventfs_create = true;
    }
    eventfs = eventfs_create_dir(evt.name, eventfs, entries, ARRAY_SIZE!(entries), evt);
    if (IS_ERR(eventfs)) {
    if (eventfs_create) {
    eventfs_remove_events_dir(remote.eventfs);
    remote.eventfs = core::ptr::null_mut();
    }
    return PTR_ERR(eventfs);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_remote_attach_events(remote: *mut trace_remote, events: *mut remote_event, nr_events: size_t) -> c_int {
    let mut i = 0;
    while (i < nr_events) {
    let mut evt = &events[i];
    if (evt.remote) {
    return -EEXIST;
    }
    evt.remote = remote;
// We need events to be sorted for efficient lookup
    if (i && evt.id <= events[i - 1].id) {
    return -EINVAL;
    }
    }
    remote.events = events;
    remote.nr_events = nr_events;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_remote_register_events(remote_name: *mut c_char, remote: *mut trace_remote, events: *mut remote_event, nr_events: size_t) -> c_int {
    let mut i = 0;
    let mut ret = 0;
    ret = trace_remote_attach_events(remote, events, nr_events);
    if (ret) {
    return ret;
    }
    while (i < nr_events) {
    let mut evt = &events[i];
    ret = trace_remote_init_eventfs(remote_name, remote, evt);
    if (ret) {
    pr_warn!("Failed to init eventfs for event '%s' (%d)",
    evt.name, ret);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __cmp_events(key: *const c_void, data: *const c_void) -> c_int {
    let mut evt = data;
pub static mut id: c_int = 0;
    return id - (int)evt.id;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_remote_find_event(remote: *mut trace_remote, id: c_ushort) -> *mut c_void {
    return bsearch((unsigned long)id, remote.events, remote.nr_events,
    sizeof!(*remote.events), __cmp_events);
    }
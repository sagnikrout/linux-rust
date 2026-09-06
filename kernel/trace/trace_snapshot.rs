//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_snapshot.c
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

// Used if snapshot allocated at boot
    static bool allocate_snapshot;
    static bool snapshot_at_boot;
    static char boot_snapshot_info[COMMAND_LINE_SIZE] __initdata;
    static int boot_snapshot_index;
#[no_mangle]
unsafe extern "C" fn boot_alloc_snapshot(str: *mut c_char) -> c_int {
    let mut slot = boot_snapshot_info + boot_snapshot_index;
pub static mut left: c_int = 0;
    let mut ret = 0;
    if (str[0] == '=') {
    str += 1;
    if (strlen(str) >= left) {
    return -1;
    }
    ret = snprintf(slot, left, "%s\t", str);
    boot_snapshot_index += ret;
    } else {
    allocate_snapshot = true;
// We also need the main ring buffer expanded
    trace_set_ring_buffer_expanded(core::ptr::null_mut());
    }
    return 1;
    }
    __setup!("alloc_snapshot", boot_alloc_snapshot);
#[no_mangle]
unsafe extern "C" fn boot_snapshot(str: *mut c_char) -> c_int {
    snapshot_at_boot = true;
    boot_alloc_snapshot(str);
    return 1;
    }
    __setup!("ftrace_boot_snapshot", boot_snapshot);
#[no_mangle]
pub unsafe extern "C" fn tracing_snapshot_instance_cond(tr: *mut trace_array, cond_data: *mut c_void) {
    let mut flags = 0;
    if (in_nmi()) {
    trace_array_puts(tr, "*** SNAPSHOT CALLED FROM NMI CONTEXT ***\n");
    trace_array_puts(tr, "*** snapshot is being ignored        ***\n");
    return;
    }
    if (!tr.allocated_snapshot) {
    trace_array_puts(tr, "*** SNAPSHOT NOT ALLOCATED ***\n");
    trace_array_puts(tr, "*** stopping trace here!   ***\n");
    tracer_tracing_off(tr);
    return;
    }
    if (tr.mapped) {
    trace_array_puts(tr, "*** BUFFER MEMORY MAPPED ***\n");
    trace_array_puts(tr, "*** Can not use snapshot (sorry) ***\n");
    return;
    }
// Note, snapshot can not be used when the tracer uses it
    if (tracer_uses_snapshot(tr.current_trace)) {
    trace_array_puts(tr, "*** LATENCY TRACER ACTIVE ***\n");
    trace_array_puts(tr, "*** Can not use snapshot (sorry) ***\n");
    return;
    }
    local_irq_save(flags);
    update_max_tr(tr, current, smp_processor_id(), cond_data);
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_snapshot_instance(tr: *mut trace_array) {
    tracing_snapshot_instance_cond(tr, core::ptr::null_mut());
    }
//
// tracing_snapshot_cond - conditionally take a snapshot of the current buffer.
// @tr:		The tracing instance to snapshot
// @cond_data:	The data to be tested conditionally, and possibly saved
//
// This is the same as tracing_snapshot() except that the snapshot is
// conditional - the snapshot will only happen if the
// cond_snapshot.update() implementation receiving the cond_data
// returns true, which means that the trace array's cond_snapshot
// update() operation used the cond_data to determine whether the
// snapshot should be taken, and if it was, presumably saved it along
// with the snapshot.
//
#[no_mangle]
pub unsafe extern "C" fn tracing_snapshot_cond(tr: *mut trace_array, cond_data: *mut c_void) {
    tracing_snapshot_instance_cond(tr, cond_data);
    }
    EXPORT_SYMBOL_GPL(tracing_snapshot_cond);
//
// tracing_cond_snapshot_data - get the user data associated with a snapshot
// @tr:		The tracing instance
//
// When the user enables a conditional snapshot using
// tracing_snapshot_cond_enable(), the user-defined cond_data is saved
// with the snapshot.  This accessor is used to retrieve it.
//
// Should not be called from cond_snapshot.update(), since it takes
// the tr->max_lock lock, which the code calling
// cond_snapshot.update() has already done.
//
// Returns the cond_data associated with the trace array's snapshot.
//
#[no_mangle]
pub unsafe extern "C" fn tracing_cond_snapshot_data(tr: *mut trace_array) -> *mut c_void {
    let mut cond_data = core::ptr::null_mut();
    local_irq_disable();
    arch_spin_lock(&tr.max_lock);
    if (tr.cond_snapshot) {
    cond_data = tr.cond_snapshot.cond_data;
    }
    arch_spin_unlock(&tr.max_lock);
    local_irq_enable();
    return cond_data;
    }
    EXPORT_SYMBOL_GPL(tracing_cond_snapshot_data);
// resize @tr's buffer to the size of @size_tr's entries
#[no_mangle]
pub unsafe extern "C" fn resize_buffer_duplicate_size(trace_buf: *mut array_buffer, size_buf: *mut array_buffer, cpu_id: c_int) -> c_int {
    int cpu, ret = 0;
    if (cpu_id == RING_BUFFER_ALL_CPUS) {
    for_each_tracing_cpu(cpu) {
    ret = ring_buffer_resize(trace_buf.buffer,
    per_cpu_ptr(size_buf.data, cpu).entries, cpu);
    if (ret < 0) {
    break;
    }
    per_cpu_ptr(trace_buf.data, cpu).entries =
    per_cpu_ptr(size_buf.data, cpu).entries;
    }
    } else {
    ret = ring_buffer_resize(trace_buf.buffer,
    per_cpu_ptr(size_buf.data, cpu_id).entries, cpu_id);
    if (ret == 0) {
    per_cpu_ptr(trace_buf.data, cpu_id).entries =
    per_cpu_ptr(size_buf.data, cpu_id).entries;
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_alloc_snapshot_instance(tr: *mut trace_array) -> c_int {
    let mut order = 0;
    let mut ret = 0;
    if (!tr.allocated_snapshot) {
// Make the snapshot buffer have the same order as main buffer
    order = ring_buffer_subbuf_order_get(tr.array_buffer.buffer);
    ret = ring_buffer_subbuf_order_set(tr.snapshot_buffer.buffer, order);
    if (ret < 0) {
    return ret;
    }
// allocate spare buffer
    ret = resize_buffer_duplicate_size(&tr.snapshot_buffer,
    &tr.array_buffer, RING_BUFFER_ALL_CPUS);
    if (ret < 0) {
    return ret;
    }
    tr.allocated_snapshot = true;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn free_snapshot(tr: *mut trace_array) {
//
// We don't free the ring buffer. instead, resize it because
// The max_tr ring buffer has some state (e.g. ring->clock) and
// we want preserve it.
//
    ring_buffer_subbuf_order_set(tr.snapshot_buffer.buffer, 0);
    ring_buffer_resize(tr.snapshot_buffer.buffer, 1, RING_BUFFER_ALL_CPUS);
    trace_set_buffer_entries(&tr.snapshot_buffer, 1);
    tracing_reset_online_cpus(&tr.snapshot_buffer);
    tr.allocated_snapshot = false;
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_arm_snapshot_locked(tr: *mut trace_array) -> c_int {
    let mut ret = 0;
    lockdep_assert_held(&trace_types_lock);
    spin_lock(&tr.snapshot_trigger_lock);
    if (tr.snapshot == UINT_MAX || tr.mapped) {
    spin_unlock(&tr.snapshot_trigger_lock);
    return -EBUSY;
    }
    tr.snapshot += 1;
    spin_unlock(&tr.snapshot_trigger_lock);
    ret = tracing_alloc_snapshot_instance(tr);
    if (ret) {
    spin_lock(&tr.snapshot_trigger_lock);
    tr.snapshot -= 1;
    spin_unlock(&tr.snapshot_trigger_lock);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_arm_snapshot(tr: *mut trace_array) -> c_int {
    guard(mutex)(&trace_types_lock);
    return tracing_arm_snapshot_locked(tr);
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_disarm_snapshot(tr: *mut trace_array) {
    spin_lock(&tr.snapshot_trigger_lock);
    if (!WARN_ON!(!tr.snapshot)) {
    tr.snapshot -= 1;
    }
    spin_unlock(&tr.snapshot_trigger_lock);
    }
//
// tracing_snapshot_alloc - allocate and take a snapshot of the current buffer.
//
// This is similar to tracing_snapshot(), but it will allocate the
// snapshot buffer if it isn't already allocated. Use this only
// where it is safe to sleep, as the allocation may sleep.
//
// This causes a swap between the snapshot buffer and the current live
// tracing buffer. You can use this to take snapshots of the live
// trace when some condition is triggered, but continue to trace.
//
#[no_mangle]
pub unsafe extern "C" fn tracing_snapshot_alloc() {
    let mut ret = 0;
    ret = tracing_alloc_snapshot();
    if (ret < 0) {
    return;
    }
    tracing_snapshot();
    }
    EXPORT_SYMBOL_GPL(tracing_snapshot_alloc);
//
// tracing_snapshot_cond_enable - enable conditional snapshot for an instance
// @tr:		The tracing instance
// @cond_data:	User data to associate with the snapshot
// @update:	Implementation of the cond_snapshot update function
//
// Check whether the conditional snapshot for the given instance has
// already been enabled, or if the current tracer is already using a
// snapshot; if so, return -EBUSY, else create a cond_snapshot and
// save the cond_data and update function inside.
//
// Returns 0 if successful, error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn tracing_snapshot_cond_enable(tr: *mut trace_array, cond_data: *mut c_void, update: cond_update_fn_t) -> c_int {
    struct cond_snapshot *cond_snapshot __free(kfree) =
    kzalloc_obj(*cond_snapshot);
    let mut ret = 0;
    if (!cond_snapshot) {
    return -ENOMEM;
    }
    cond_snapshot.cond_data = cond_data;
    cond_snapshot.update = update;
    guard(mutex)(&trace_types_lock);
    if (tracer_uses_snapshot(tr.current_trace)) {
    return -EBUSY;
    }
//
// The cond_snapshot can only change to NULL without the
// trace_types_lock. We don't care if we race with it going
// to NULL, but we want to make sure that it's not set to
// something other than NULL when we get here, which we can
// do safely with only holding the trace_types_lock and not
// having to take the max_lock.
//
    if (tr.cond_snapshot) {
    return -EBUSY;
    }
    ret = tracing_arm_snapshot_locked(tr);
    if (ret) {
    return ret;
    }
    local_irq_disable();
    arch_spin_lock(&tr.max_lock);
    tr.cond_snapshot = no_free_ptr(cond_snapshot);
    arch_spin_unlock(&tr.max_lock);
    local_irq_enable();
    return 0;
    }
    EXPORT_SYMBOL_GPL(tracing_snapshot_cond_enable);
//
// tracing_snapshot_cond_disable - disable conditional snapshot for an instance
// @tr:		The tracing instance
//
// Check whether the conditional snapshot for the given instance is
// enabled; if so, free the cond_snapshot associated with it,
// otherwise return -EINVAL.
//
// Returns 0 if successful, error otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn tracing_snapshot_cond_disable(tr: *mut trace_array) -> c_int {
pub static mut ret: c_int = 0;
    local_irq_disable();
    arch_spin_lock(&tr.max_lock);
    if (!tr.cond_snapshot) {
    ret = -EINVAL;
    }
    else {
    kfree(tr.cond_snapshot);
    tr.cond_snapshot = core::ptr::null_mut();
    }
    arch_spin_unlock(&tr.max_lock);
    local_irq_enable();
    tracing_disarm_snapshot(tr);
    return ret;
    }
    EXPORT_SYMBOL_GPL(tracing_snapshot_cond_disable);

pub static mut fsnotify_wq: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn latency_fsnotify_workfn(work: *mut work_struct) {
    let mut tr = container_of!(work, trace_array,
    fsnotify_work);
    fsnotify_inode(tr.d_max_latency.d_inode, FS_MODIFY);
    }
#[no_mangle]
unsafe extern "C" fn latency_fsnotify_workfn_irq(iwork: *mut irq_work) {
    let mut tr = container_of!(iwork, trace_array,
    fsnotify_irqwork);
    queue_work(fsnotify_wq, &tr.fsnotify_work);
    }
#[no_mangle]
pub unsafe extern "C" fn latency_fsnotify_init() -> __init static int {
    fsnotify_wq = alloc_workqueue("tr_max_lat_wq",
    WQ_UNBOUND | WQ_HIGHPRI, 0);
    if (!fsnotify_wq) {
    pr_err!("Unable to allocate tr_max_lat_wq\n");
    return -ENOMEM;
    }
    return 0;
    }
    late_initcall_sync!(latency_fsnotify_init);
#[no_mangle]
pub unsafe extern "C" fn latency_fsnotify(tr: *mut trace_array) {
    if (!fsnotify_wq) {
    return;
    }
//
// We cannot call queue_work(&tr->fsnotify_work) from here because it's
// possible that we are called from __schedule() or do_idle(), which
// could cause a deadlock.
//
    irq_work_queue(&tr.fsnotify_irqwork);
    }

pub static mut tracing_max_lat_fops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn trace_create_maxlat_file(tr: *mut trace_array, d_tracer: *mut dentry) {

    INIT_WORK(&tr.fsnotify_work, latency_fsnotify_workfn);
    init_irq_work(&tr.fsnotify_irqwork, latency_fsnotify_workfn_irq);

    tr.d_max_latency = trace_create_file("tracing_max_latency",
    TRACE_MODE_WRITE,
    d_tracer, tr,
    &tracing_max_lat_fops);
    }
//
// Copy the new maximum trace into the separate maximum-trace
// structure. (this way the maximum trace is permanently saved,
for later retrieval via /sys/kernel/tracing/tracing_max_latency)
//
#[no_mangle]
pub unsafe extern "C" fn __update_max_tr(tr: *mut trace_array, tsk: *mut task_struct, cpu: c_int) {
    let mut trace_buf = &tr.array_buffer;
    let mut data = per_cpu_ptr(trace_buf.data, cpu);
    let mut max_buf = &tr.snapshot_buffer;
    let mut max_data = per_cpu_ptr(max_buf.data, cpu);
    max_buf.cpu = cpu;
    max_buf.time_start = data.preempt_timestamp;
    max_data.saved_latency = tr.max_latency;
    max_data.critical_start = data.critical_start;
    max_data.critical_end = data.critical_end;
    strscpy(max_data.comm, tsk.comm);
    max_data.pid = tsk.pid;
//
// If tsk == current, then use current_uid(), as that does not use
// RCU. The irq tracer can be called out of RCU scope.
//
    if (tsk == current)
    max_data.uid = current_uid();
    else
    max_data.uid = task_uid(tsk);
    max_data.nice = tsk.static_prio - 20 - MAX_RT_PRIO;
    max_data.policy = tsk.policy;
    max_data.rt_priority = tsk.rt_priority;
// record this tasks comm
    tracing_record_cmdline(tsk);
    latency_fsnotify(tr);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: __update_max_tr
pub unsafe extern "C" fn __update_max_tr_dup(tr: *mut trace_array, tsk: *mut task_struct, cpu: c_int) { }

//
// update_max_tr - snapshot all trace buffers from global_trace to max_tr
// @tr: tracer
// @tsk: the task with the latency
// @cpu: The cpu that initiated the trace.
// @cond_data: User data associated with a conditional snapshot
//
// Flip the buffers between the @tr and the max_tr and record information
// about which task was the cause of this latency.
//
#[no_mangle]
pub unsafe extern "C" fn update_max_tr(tr: *mut trace_array, tsk: *mut task_struct, cpu: c_int, cond_data: *mut c_void) {
    if (tr.stop_count)
    return;
    WARN_ON_ONCE!(!irqs_disabled());
    if (!tr.allocated_snapshot) {
// Only the nop tracer should hit this when disabling
    WARN_ON_ONCE!(tr.current_trace != &nop_trace);
    return;
    }
    arch_spin_lock(&tr.max_lock);
// Inherit the recordable setting from array_buffer
    if (ring_buffer_record_is_set_on(tr.array_buffer.buffer))
    ring_buffer_record_on(tr.snapshot_buffer.buffer);
    else
    ring_buffer_record_off(tr.snapshot_buffer.buffer);
    if (tr.cond_snapshot && !tr.cond_snapshot.update(tr, cond_data)) {
    arch_spin_unlock(&tr.max_lock);
    return;
    }
    swap(tr.array_buffer.buffer, tr.snapshot_buffer.buffer);
    __update_max_tr(tr, tsk, cpu);
    arch_spin_unlock(&tr.max_lock);
// Any waiters on the old snapshot buffer need to wake up
    ring_buffer_wake_waiters(tr.array_buffer.buffer, RING_BUFFER_ALL_CPUS);
    }
//
// update_max_tr_single - only copy one trace over, and reset the rest
// @tr: tracer
// @tsk: task with the latency
// @cpu: the cpu of the buffer to copy.
//
// Flip the trace of a single CPU buffer between the @tr and the max_tr.
//
#[no_mangle]
pub unsafe extern "C" fn update_max_tr_single(tr: *mut trace_array, tsk: *mut task_struct, cpu: c_int) {
    let mut ret = 0;
    if (tr.stop_count)
    return;
    WARN_ON_ONCE!(!irqs_disabled());
    if (!tr.allocated_snapshot) {
// Only the nop tracer should hit this when disabling
    WARN_ON_ONCE!(tr.current_trace != &nop_trace);
    return;
    }
    arch_spin_lock(&tr.max_lock);
    ret = ring_buffer_swap_cpu(tr.snapshot_buffer.buffer, tr.array_buffer.buffer, cpu);
    if (ret == -EBUSY) {
//
// We failed to swap the buffer due to a commit taking
// place on this CPU. We fail to record, but we reset
// the max trace buffer (no one writes directly to it)
// and flag that it failed.
// Another reason is resize is in progress.
//
    trace_array_printk_buf(tr.snapshot_buffer.buffer, _THIS_IP_, {
    "Failed to swap buffers due to commit or resize in progress\n");
}
    }
    WARN_ON_ONCE!(ret && ret != -EAGAIN && ret != -EBUSY);
    __update_max_tr(tr, tsk, cpu);
    arch_spin_unlock(&tr.max_lock);
    }
#[no_mangle]
unsafe extern "C" fn show_snapshot_main_help(m: *mut seq_file) {
    seq_puts(m, "# echo 0 > snapshot : Clears and frees snapshot buffer\n"
    "# echo 1 > snapshot : Allocates snapshot buffer, if not already allocated.\n"
    "#                      Takes a snapshot of the main buffer.\n"
    "# echo 2 > snapshot : Clears snapshot buffer (but does not allocate or free)\n"
    "#                      (Doesn't have to be '2' works with any number that\n"
    "#                       is not a '0' or '1')\n");
    }
#[no_mangle]
unsafe extern "C" fn show_snapshot_percpu_help(m: *mut seq_file) {
    seq_puts(m, "# echo 0 > snapshot : Invalid for per_cpu snapshot file.\n");

    seq_puts(m, "# echo 1 > snapshot : Allocates snapshot buffer, if not already allocated.\n"
    "#                      Takes a snapshot of the main buffer for this cpu.\n");

    seq_puts(m, "# echo 1 > snapshot : Not supported with this kernel.\n"
    "#                     Must use main snapshot file to allocate.\n");

    seq_puts(m, "# echo 2 > snapshot : Clears this cpu's snapshot buffer (but does not allocate)\n"
    "#                      (Doesn't have to be '2' works with any number that\n"
    "#                       is not a '0' or '1')\n");
    }
#[no_mangle]
pub unsafe extern "C" fn print_snapshot_help(m: *mut seq_file, iter: *mut trace_iterator) {
    if (iter.tr.allocated_snapshot) {
    seq_puts(m, "#\n# * Snapshot is allocated *\n#\n");
    }
    else {
    seq_puts(m, "#\n# * Snapshot is freed *\n#\n");
    }
    seq_puts(m, "# Snapshot commands:\n");
    if (iter.cpu_file == RING_BUFFER_ALL_CPUS) {
    show_snapshot_main_help(m);
    }
    else {
    show_snapshot_percpu_help(m);
    }
    }
#[no_mangle]
unsafe extern "C" fn tracing_snapshot_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut tr = inode.i_private;
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut m: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ret = tracing_check_open_get_tr(tr);
    if (ret) {
    return ret;
    }
    if (file.f_mode & FMODE_READ) {
    iter = __tracing_open(inode, file, true);
    if (IS_ERR(iter)) {
    ret = PTR_ERR(iter);
    }
    } else {
// Writes still need the seq_file to hold the private data
    ret = -ENOMEM;
    m = kzalloc_obj(*m);
    if (!m) {
// goto;
    }
    iter = kzalloc_obj(*iter);
    if (!iter) {
    kfree(m);
// goto;
    }
    ret = 0;
    iter.tr = tr;
    iter.array_buffer = &tr.snapshot_buffer;
    iter.cpu_file = tracing_get_cpu(inode);
    m.private = iter;
    file.private_data = m;
    }
// label;
    if (ret < 0) {
    trace_array_put(tr);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tracing_swap_cpu_buffer(tr: *mut c_void) {
    update_max_tr_single(tr, current, smp_processor_id());
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_snapshot_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut m = filp.private_data;
    let mut iter = m.private;
    let mut tr = iter.tr;
    let mut val = 0;
    let mut ret = 0;
    ret = tracing_update_buffers(tr);
    if (ret < 0) {
    return ret;
    }
    ret = kstrtoul_from_user(ubuf, cnt, 10, &val);
    if (ret) {
    return ret;
    }
    guard(mutex)(&trace_types_lock);
    if (tracer_uses_snapshot(tr.current_trace)) {
    return -EBUSY;
    }
    local_irq_disable();
    arch_spin_lock(&tr.max_lock);
    if (tr.cond_snapshot) {
    ret = -EBUSY;
    }
    arch_spin_unlock(&tr.max_lock);
    local_irq_enable();
    if (ret) {
    return ret;
    }
    match (val) {
    0 => {
    if (iter.cpu_file != RING_BUFFER_ALL_CPUS) {
    return -EINVAL;
    }
    if (tr.allocated_snapshot) {
    free_snapshot(tr);
    }
    // break;
    }
    1 => {
// Only allow per-cpu swap if the ring buffer supports it

    if (iter.cpu_file != RING_BUFFER_ALL_CPUS) {
    return -EINVAL;
    }

    if (tr.allocated_snapshot) {
    ret = resize_buffer_duplicate_size(&tr.snapshot_buffer,
    &tr.array_buffer, iter.cpu_file);
    }
    ret = tracing_arm_snapshot_locked(tr);
    if (ret) {
    return ret;
    }
// Now, we're going to swap
    if (iter.cpu_file == RING_BUFFER_ALL_CPUS) {
    local_irq_disable();
    update_max_tr(tr, current, smp_processor_id(), core::ptr::null_mut());
    local_irq_enable();
    } else {
    smp_call_function_single(iter.cpu_file, tracing_swap_cpu_buffer,
    tr, 1);
    }
    tracing_disarm_snapshot(tr);
    // break;
    }
    _ => {
    if (tr.allocated_snapshot) {
    if (iter.cpu_file == RING_BUFFER_ALL_CPUS) {
    tracing_reset_online_cpus(&tr.snapshot_buffer);
    }
    else {
    tracing_reset_cpu(&tr.snapshot_buffer, iter.cpu_file);
    }
    }
    // break;
    }
    }
    if (ret >= 0) {
// ppos += cnt;
    ret = cnt;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tracing_snapshot_release(inode: *mut inode, file: *mut file) -> c_int {
    let mut m = file.private_data;
    let mut ret = 0;
    ret = tracing_release(inode, file);
    if (file.f_mode & FMODE_READ) {
    return ret;
    }
// If write only, the seq_file is just a stub
    if (m) {
    kfree(m.private);
    }
    kfree(m);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snapshot_raw_open(inode: *mut inode, filp: *mut file) -> c_int {
pub static mut info: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
// The following checks for tracefs lockdown
    ret = tracing_buffers_open(inode, filp);
    if (ret < 0) {
    return ret;
    }
    info = filp.private_data;
    if (tracer_uses_snapshot(info.iter.trace)) {
    tracing_buffers_release(inode, filp);
    return -EBUSY;
    }
    info.iter.snapshot = true;
    info.iter.array_buffer = &info.iter.tr.snapshot_buffer;
    return ret;
    }
pub static mut file_operations: usize = 0;
pub static mut file_operations: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn tracing_max_lat_read(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut tr = filp.private_data;
    return tracing_nsecs_read(&tr.max_latency, ubuf, cnt, ppos);
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_max_lat_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut tr = filp.private_data;
    return tracing_nsecs_write(&tr.max_latency, ubuf, cnt, ppos);
    }
pub static mut file_operations: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn get_snapshot_map(tr: *mut trace_array) -> c_int {
pub static mut err: c_int = 0;
//
// Called with mmap_lock held. lockdep would be unhappy if we would now
// take trace_types_lock. Instead use the specific
// snapshot_trigger_lock.
//
    spin_lock(&tr.snapshot_trigger_lock);
    if (tr.snapshot || tr.mapped == UINT_MAX) {
    err = -EBUSY;
    }
    else {
    tr.mapped += 1;
    }
    spin_unlock(&tr.snapshot_trigger_lock);
// Wait for update_max_tr() to observe iter->tr->mapped
    if (tr.mapped == 1) {
    synchronize_rcu();
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn put_snapshot_map(tr: *mut trace_array) {
    spin_lock(&tr.snapshot_trigger_lock);
    if (!WARN_ON!(!tr.mapped)) {
    tr.mapped -= 1;
    }
    spin_unlock(&tr.snapshot_trigger_lock);
    }

#[no_mangle]
pub unsafe extern "C" fn ftrace_snapshot(ip: c_ulong, parent_ip: c_ulong, tr: *mut trace_array, ops: *mut ftrace_probe_ops, data: *mut c_void) {
    tracing_snapshot_instance(tr);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_count_snapshot(ip: c_ulong, parent_ip: c_ulong, tr: *mut trace_array, ops: *mut ftrace_probe_ops, data: *mut c_void) {
    let mut mapper = data;
    let mut count = core::ptr::null_mut();
    if (mapper) {
    count = ftrace_func_mapper_find_ip(mapper, ip);
    }
    if (count) {
    if (*count <= 0) {
    return;
    }
    (*count)--;
    }
    tracing_snapshot_instance(tr);
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_snapshot_print(m: *mut seq_file, ip: c_ulong, ops: *mut ftrace_probe_ops, data: *mut c_void) -> c_int {
    let mut mapper = data;
    let mut count = core::ptr::null_mut();
    seq_printf(m, "%ps:", ip);
    seq_puts(m, "snapshot");
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
pub unsafe extern "C" fn ftrace_snapshot_init(ops: *mut ftrace_probe_ops, tr: *mut trace_array, ip: c_ulong, init_data: *mut c_void, data: *mut *mut c_void) -> c_int {
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
pub unsafe extern "C" fn ftrace_snapshot_free(ops: *mut ftrace_probe_ops, tr: *mut trace_array, ip: c_ulong, data: *mut c_void) {
    let mut mapper = data;
    if (!ip) {
    if (!mapper) {
    return;
    }
    free_ftrace_func_mapper(mapper, core::ptr::null_mut());
    return;
    }
    ftrace_func_mapper_remove_ip(mapper, ip);
    }
pub static mut ftrace_probe_ops: usize = 0;
pub static mut ftrace_probe_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn ftrace_trace_snapshot_callback(tr: *mut trace_array, hash: *mut ftrace_hash, glob: *mut c_char, cmd: *mut c_char, param: *mut c_char, enable: c_int) -> c_int {
pub static mut ops: *mut c_void = core::ptr::null_mut();
    let mut count = -1;
pub static mut number: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!tr) {
    return -ENODEV;
    }
// hash funcs only work with set_ftrace_filter
    if (!enable) {
    return -EINVAL;
    }
    ops = param ? &snapshot_count_probe_ops :  &snapshot_probe_ops;
    if (glob[0] == '!') {
    ret = unregister_ftrace_function_probe_func(glob+1, tr, ops);
    if (!ret) {
    tracing_disarm_snapshot(tr);
    }
    return ret;
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
    ret = tracing_arm_snapshot(tr);
    if (ret < 0) {
    return ret;
    }
    ret = register_ftrace_function_probe(glob, tr, ops, count);
    if (ret < 0) {
    tracing_disarm_snapshot(tr);
    }
    return ret < 0 ? ret : 0;
    }
pub static mut ftrace_func_command: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn register_snapshot_cmd() -> __init int {
    return register_ftrace_command(&ftrace_snapshot_cmd);
    }

#[no_mangle]
pub unsafe extern "C" fn trace_allocate_snapshot(tr: *mut trace_array, size: c_int) -> c_int {
    let mut ret = 0;
// Fix mapped buffer trace arrays do not have snapshot buffers
    if (tr.range_addr_start) {
    return 0;
    }
// allocate_snapshot can only be true during system boot
    ret = allocate_trace_buffer(tr, &tr.snapshot_buffer,
    allocate_snapshot ? size : 1);
    if (ret < 0) {
    return -ENOMEM;
    }
    tr.allocated_snapshot = allocate_snapshot;
    allocate_snapshot = false;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tr_needs_alloc_snapshot(name: *const c_char) -> __init static bool {
pub static mut test: *mut c_void = core::ptr::null_mut();
pub static mut len: c_int = 0;
    let mut ret = 0;
    if (!boot_snapshot_index) {
    return false;
    }
    if (strncmp(name, boot_snapshot_info, len) == 0 &&
    boot_snapshot_info[len] == '\t') {
    return true;
    }
    test = kmalloc(strlen(name) + 3, GFP_KERNEL);
    if (!test) {
    return false;
    }
    sprintf(test, "\t%s\t", name);
    ret = strstr(boot_snapshot_info, test) == core::ptr::null_mut();
    kfree(test);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn do_allocate_snapshot(name: *const c_char) -> __init void {
    if (!tr_needs_alloc_snapshot(name)) {
    return;
    }
//
// When allocate_snapshot is set, the next call to
// allocate_trace_buffers() (called by trace_array_get_by_name())
// will allocate the snapshot buffer. That will also clear
// this flag.
//
    allocate_snapshot = true;
    }
#[no_mangle]
pub unsafe extern "C" fn ftrace_boot_snapshot()  {
pub static mut tr: *mut c_void = core::ptr::null_mut();
    if (!snapshot_at_boot) {
    return;
    }
    list_for_each_entry(tr, &ftrace_trace_arrays, list) {
    if (!tr.allocated_snapshot) {
    continue;
    }
    tracing_snapshot_instance(tr);
    trace_array_puts(tr, "** Boot snapshot taken **\n");
    }
    }
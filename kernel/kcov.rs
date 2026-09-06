//! Automatically rewritten from C to Rust
//! Source: kernel/kcov.c
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


























// SPDX-License-Identifier: GPL-2.0

// Macro flag: #define DISABLE_BRANCH_PROFILING

// Number of 64-bit words written per one comparison:
pub const KCOV_WORDS_PER_CMP: c_int = 4;
//
// kcov descriptor (one per opened debugfs file).
// State transitions of the descriptor:
// - initial state after open()
// - then there must be a single ioctl(KCOV_INIT_TRACE) call
// - then, mmap() call (several calls are allowed but not useful)
// - then, ioctl(KCOV_ENABLE, arg), where arg is
// KCOV_TRACE_PC - to trace only the PCs
// or
// KCOV_TRACE_CMP - to trace only the comparison operands
// - then, ioctl(KCOV_DISABLE) to disable the task.
// Enabling/disabling ioctls can be repeated (only one task a time allowed).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcov {
//
// Reference counter. We keep one for:
// - opened file descriptor
// - task with enabled coverage (we can't unwire it from another task)
// - each code section for remote coverage collection
//
    pub refcount: refcount_t,
// The lock protects mode, size, area and t.
    pub lock: spinlock_t,
    pub __guarded_by(&lock): kcov_mode mode,
// Size of arena (in long's).
    pub __guarded_by(&lock): unsigned int size,
// Coverage buffer shared with user space.
    pub __guarded_by(&lock): *mut *mut c_void area,
// Task for which we collect coverage, or NULL.
    pub __guarded_by(&lock): *mut *mut task_t,
// Collecting coverage from remote (background) threads.
    pub remote: bool,
// Size of remote area (in long's).
    pub remote_size: c_uint,
//
// Sequence is incremented each time kcov is reenabled, used by
// kcov_remote_stop(), see the comment there.
//
    pub sequence: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcov_remote_area {
    pub list: list_head,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcov_remote {
    pub handle: u64,
    pub kcov: *mut kcov,
    pub hnode: hlist_node,
}
// static DEFINE_SPINLOCK(kcov_remote_lock);
// static DEFINE_HASHTABLE(kcov_remote_map, 4);
pub static mut list_head: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcov_percpu_data {
    pub lock: local_lock_t,
}

    static DEFINE_PER_CPU(kcov_percpu_data, kcov_percpu_data) = {
    .lock = INIT_LOCAL_LOCK(lock),
    };
// Must be called with kcov_remote_lock locked.
#[no_mangle]
pub unsafe extern "C" fn kcov_remote_find() {
    let mut remote = core::ptr::null_mut();
    hash_for_each_possible(kcov_remote_map, remote, hnode, handle) {
    if (remote.handle == handle) {
    return remote;
    }
    }
    return core::ptr::null_mut();
    }
// Must be called with kcov_remote_lock locked.
#[no_mangle]
pub unsafe extern "C" fn kcov_remote_add() {
    let mut remote = core::ptr::null_mut();
    if (kcov_remote_find(handle)) {
    return ERR_PTR(-EEXIST);
    }
    remote = kmalloc_obj(*remote, GFP_ATOMIC);
    if (!remote) {
    return ERR_PTR(-ENOMEM);
    }
    remote.handle = handle;
    remote.kcov = kcov;
    hash_add(kcov_remote_map, &remote.hnode, handle);
    return remote;
    }
// Must be called with kcov_remote_lock locked.
#[no_mangle]
pub unsafe extern "C" fn kcov_remote_area_get() {
    let mut area = core::ptr::null_mut();
    let mut pos = core::ptr::null_mut();
    let mut list = &kcov_remote_areas[irq];
    list_for_each(pos, list) {
    area = list_entry(pos, kcov_remote_area, list);
    if (area.size == size) {
    list_del(&area.list);
    return area;
    }
    }
    return core::ptr::null_mut();
    }
// Must be called with kcov_remote_lock locked.
#[no_mangle]
pub unsafe extern "C" fn kcov_remote_area_put() {
// INIT_LIST_HEAD;
    area.size = size;
    list_add(&area.list, &kcov_remote_areas[irq]);
//
// KMSAN doesn't instrument this file, so it may not know area->list
// is initialized. Unpoison it explicitly to avoid reports in
// kcov_remote_area_get().
//
    kmsan_unpoison_memory(&area.list, sizeof!(area.list));
    }
//
// Unlike in_serving_softirq(), this function returns false when called during
// a hardirq or an NMI that happened in the softirq context.
//
#[no_mangle]
unsafe extern "C" fn in_softirq_really() -> __always_inline bool {
    return in_serving_softirq() && !in_hardirq() && !in_nmi();
    }
#[no_mangle]
unsafe extern "C" fn check_kcov_mode(needed_mode: kcov_mode, t: *mut task_struct) -> notrace bool {
    let mut mode = 0;
//
// We are interested in code coverage as a function of a syscall inputs,
// so we ignore code executed in interrupts, unless we are in a remote
// coverage collection section in a softirq.
//
    if (!in_task() && !(in_softirq_really() && t.kcov_softirq)) {
    return false;
    }
    mode = READ_ONCE(t.kcov_mode);
//
// There is some code that runs in interrupts but for which
// in_interrupt() returns false (e.g. preempt_schedule_irq()).
// READ_ONCE()/barrier() effectively provides load-acquire wrt
// interrupts, there are paired barrier()/WRITE_ONCE() in
// kcov_start().
//
    barrier();
pub static mut mode: return = 0;
    }
#[no_mangle]
unsafe extern "C" fn canonicalize_ip(ip: c_ulong) -> notrace unsigned long {

    ip -= kaslr_offset();

    return ip;
    }
//
// Entry point from instrumented code.
// This is called once per basic-block/edge.
//
#[no_mangle]
pub unsafe extern "C" fn __sanitizer_cov_trace_pc() -> void notrace {
    let mut t = core::ptr::null_mut();
pub static mut area: *mut c_void = core::ptr::null_mut();
pub static mut ip: c_ulong = 0;
    let mut pos = 0;
    t = current;
    if (!check_kcov_mode(KCOV_MODE_TRACE_PC, t)) {
    return;
    }
    area = t.kcov_area;
// The first 64-bit word is the number of subsequent PCs.
    pos = READ_ONCE(area[0]) + 1;
    if (likely(pos < t.kcov_size)) {
// Previously we write pc before updating pos. However, some
// early interrupt code could bypass check_kcov_mode() check
// and invoke __sanitizer_cov_trace_pc(). If such interrupt is
// raised between writing pc and updating pos, the pc could be
// overitten by the recursive __sanitizer_cov_trace_pc().
// Update pos before writing pc to avoid such interleaving.
//
// WRITE_ONCE;
    barrier();
    area[pos] = ip;
    }
    }
// EXPORT_SYMBOL;

#[no_mangle]
unsafe extern "C" fn write_comp_data(type: u64, arg1: u64, arg2: u64, ip: u64) -> void notrace {
    let mut t = core::ptr::null_mut();
    let mut area = core::ptr::null_mut();
    u64 count, start_index, end_pos, max_pos;
    t = current;
    if (!check_kcov_mode(KCOV_MODE_TRACE_CMP, t)) {
    return;
    }
    ip = canonicalize_ip(ip);
//
// We write all comparison arguments and types as u64.
// The buffer was allocated for t->kcov_size unsigned longs.
//
    area = t.kcov_area;
    max_pos = t.kcov_size * sizeof!(unsigned long);
    count = READ_ONCE(area[0]);
// Every record is KCOV_WORDS_PER_CMP 64-bit words.
    start_index = 1 + count * KCOV_WORDS_PER_CMP;
    end_pos = (start_index + KCOV_WORDS_PER_CMP) * sizeof!(u64);
    if (likely(end_pos <= max_pos)) {
// See comment in __sanitizer_cov_trace_pc().
// WRITE_ONCE;
    barrier();
    area[start_index] = type;
    area[start_index + 1] = arg1;
    area[start_index + 2] = arg2;
    area[start_index + 3] = ip;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __sanitizer_cov_trace_cmp1(arg1: u8, arg2: u8) -> void notrace {
    write_comp_data(KCOV_CMP_SIZE(0), arg1, arg2, _RET_IP_);
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn __sanitizer_cov_trace_cmp2(arg1: u16, arg2: u16) -> void notrace {
    write_comp_data(KCOV_CMP_SIZE(1), arg1, arg2, _RET_IP_);
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn __sanitizer_cov_trace_cmp4(arg1: u32, arg2: u32) -> void notrace {
    write_comp_data(KCOV_CMP_SIZE(2), arg1, arg2, _RET_IP_);
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn __sanitizer_cov_trace_cmp8(arg1: kcov_u64, arg2: kcov_u64) -> void notrace {
    write_comp_data(KCOV_CMP_SIZE(3), arg1, arg2, _RET_IP_);
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn __sanitizer_cov_trace_const_cmp1(arg1: u8, arg2: u8) -> void notrace {
    write_comp_data(KCOV_CMP_SIZE(0) | KCOV_CMP_CONST, arg1, arg2,
    _RET_IP_);
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn __sanitizer_cov_trace_const_cmp2(arg1: u16, arg2: u16) -> void notrace {
    write_comp_data(KCOV_CMP_SIZE(1) | KCOV_CMP_CONST, arg1, arg2,
    _RET_IP_);
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn __sanitizer_cov_trace_const_cmp4(arg1: u32, arg2: u32) -> void notrace {
    write_comp_data(KCOV_CMP_SIZE(2) | KCOV_CMP_CONST, arg1, arg2,
    _RET_IP_);
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn __sanitizer_cov_trace_const_cmp8(arg1: kcov_u64, arg2: kcov_u64) -> void notrace {
    write_comp_data(KCOV_CMP_SIZE(3) | KCOV_CMP_CONST, arg1, arg2,
    _RET_IP_);
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn __sanitizer_cov_trace_switch(val: kcov_u64, arg: *mut c_void) -> void notrace {
    let mut i = 0;
    let mut cases = arg;
pub static mut count: u64 = 0;
pub static mut size: u64 = 0;
pub static mut type: u64 = 0;
    match (size) {
    8 => {
    type |= KCOV_CMP_SIZE(0);
    // break;
    }
    16 => {
    type |= KCOV_CMP_SIZE(1);
    // break;
    }
    32 => {
    type |= KCOV_CMP_SIZE(2);
    // break;
    }
    64 => {
    type |= KCOV_CMP_SIZE(3);
    // break;
    }
    _ => {
    return;
    }
    }
    for (i = 0; i < count; i++) {
    write_comp_data(type, cases[i + 2], val, _RET_IP_);
    }
    }
// EXPORT_SYMBOL;

#[no_mangle]
pub unsafe extern "C" fn kcov_start() {
    kcov_debug("t = %px, size = %u, area = %px\n", t, size, area);
    t.kcov = kcov;
// Cache in task struct for performance.
    t.kcov_size = size;
    t.kcov_area = area;
    t.kcov_sequence = sequence;
// See comment in check_kcov_mode().
    barrier();
// WRITE_ONCE;
    }
// operates on coverage-generator-owned fields
#[no_mangle]
unsafe extern "C" fn kcov_stop(t: *mut task_struct) {
// WRITE_ONCE;
    barrier();
    t.kcov = core::ptr::null_mut();
    t.kcov_size = 0;
    t.kcov_area = core::ptr::null_mut();
    }
// operates on coverage-generator-owned fields
#[no_mangle]
unsafe extern "C" fn kcov_task_reset(t: *mut task_struct) {
    kcov_stop(t);
    t.kcov_sequence = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kcov_task_init(t: *mut task_struct) {
    kcov_task_reset(t);
    t.kcov_remote = core::ptr::null_mut();
    t.kcov_handle = current.kcov_handle;
    t.kcov_softirq = 0;
    t.kcov_saved_mode = 0;
    t.kcov_saved_size = 0;
    t.kcov_saved_area = core::ptr::null_mut();
    t.kcov_saved_kcov = core::ptr::null_mut();
    t.kcov_saved_sequence = 0;
    }
#[no_mangle]
unsafe extern "C" fn kcov_reset(kcov: *mut kcov) {
    kcov.t = core::ptr::null_mut();
    kcov.mode = KCOV_MODE_INIT;
    kcov.remote = false;
    kcov.remote_size = 0;
    kcov.sequence += 1;
    }
#[no_mangle]
unsafe extern "C" fn kcov_remote_reset(kcov: *mut kcov) {
    let mut bkt = 0;
    let mut remote = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    let mut flags = 0;
    spin_lock_irqsave(&kcov_remote_lock, flags);
    hash_for_each_safe(kcov_remote_map, bkt, tmp, remote, hnode) {
    if (remote.kcov != kcov) {
    continue;
    }
    hash_del(&remote.hnode);
    kfree(remote);
    }
// Do reset before unlock to prevent races with kcov_remote_start().
    kcov_reset(kcov);
    spin_unlock_irqrestore(&kcov_remote_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn kcov_disable(t: *mut task_struct, kcov: *mut kcov) {
    if (kcov.remote) {
    t.kcov_handle = 0;
    t.kcov_remote = core::ptr::null_mut();
    kcov_remote_reset(kcov);
    } else {
    kcov_task_reset(t);
    kcov_reset(kcov);
    }
    }
#[no_mangle]
unsafe extern "C" fn kcov_get(kcov: *mut kcov) {
    refcount_inc(&kcov.refcount);
    }
#[no_mangle]
unsafe extern "C" fn kcov_put(kcov: *mut kcov) {
    if (refcount_dec_and_test(&kcov.refcount)) {
// Context-safety: no references left, object being destroyed.
    context_unsafe(
    kcov_remote_reset(kcov);
    vfree(kcov.area);
    );
    kfree(kcov);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kcov_task_exit(t: *mut task_struct) {
    let mut kcov = core::ptr::null_mut();
    let mut flags = 0;
    kcov = t.kcov;
    if (kcov) {
    spin_lock_irqsave(&kcov.lock, flags);
    kcov_debug("t = %px, kcov.t = %px\n", t, kcov.t);
//
// This could be a remote task between kcov_remote_start() and
// kcov_remote_stop().
// In this case we should print a warning right away, since a
// task shouldn't be exiting when it's in a kcov coverage
// collection section.
//
// Otherwise, this should be a task that created a local
// kcov instance and hasn't called KCOV_DISABLE.
// Make sure that t->kcov->t is consistent.
//
    if (WARN_ON!(kcov.remote) || WARN_ON!(kcov.t != t)) {
    spin_unlock_irqrestore(&kcov.lock, flags);
    return;
    }
// Just to not leave dangling references behind.
    kcov_disable(t, kcov);
    spin_unlock_irqrestore(&kcov.lock, flags);
    kcov_put(kcov);
    }
    kcov = t.kcov_remote;
    if (kcov) {
    spin_lock_irqsave(&kcov.lock, flags);
    kcov_debug("t = %px, kcov.t = %px\n", t, kcov.t);
//
// This is a KCOV_REMOTE_ENABLE device, and the task is the
// user task which has requested remote coverage collection.
// Make sure that t->kcov->t is consistent.
//
    if (WARN_ON!(!kcov.remote) || WARN_ON!(kcov.t != t)) {
    spin_unlock_irqrestore(&kcov.lock, flags);
    return;
    }
// Just to not leave dangling references behind.
    kcov_disable(t, kcov);
    spin_unlock_irqrestore(&kcov.lock, flags);
    kcov_put(kcov);
    }
    }
#[no_mangle]
unsafe extern "C" fn kcov_mmap(filep: *mut file, vma: *mut vm_area_struct) -> c_int {
pub static mut res: c_int = 0;
    let mut kcov = vma.vm_file.private_data;
    unsigned long size, off;
    let mut page = core::ptr::null_mut();
    let mut flags = 0;
    let mut area = core::ptr::null_mut();
    spin_lock_irqsave(&kcov.lock, flags);
    size = kcov.size * sizeof!(unsigned long);
    if (kcov.area == core::ptr::null_mut() || vma_start_pgoff(vma) ||
    vma.vm_end - vma.vm_start != size) {
    res = -EINVAL;
// goto;
    }
    area = kcov.area;
    spin_unlock_irqrestore(&kcov.lock, flags);
    vm_flags_set(vma, VM_DONTEXPAND);
    while (off < size) {
    page = vmalloc_to_page(area + off);
    res = vm_insert_page(vma, vma.vm_start + off, page);
    if (res) {
    pr_warn_once("kcov: vm_insert_page() failed\n");
    return res;
    }
    }
    return 0;
// label;
    spin_unlock_irqrestore(&kcov.lock, flags);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn kcov_open(inode: *mut inode, filep: *mut file) -> c_int {
    let mut kcov = core::ptr::null_mut();
    kcov = kzalloc_obj(*kcov);
    if (!kcov) {
    return -ENOMEM;
    }
    guard(spinlock_init)(&kcov.lock);
    kcov.mode = KCOV_MODE_DISABLED;
    kcov.sequence = 1;
    refcount_set(&kcov.refcount, 1);
    filep.private_data = kcov;
    return nonseekable_open(inode, filep);
    }
#[no_mangle]
unsafe extern "C" fn kcov_close(inode: *mut inode, filep: *mut file) -> c_int {
    kcov_put(filep.private_data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kcov_get_mode(arg: c_ulong) -> c_int {
    if (arg == KCOV_TRACE_PC) {
    return KCOV_MODE_TRACE_PC;
    }

    else if (arg == KCOV_TRACE_CMP) {

    return KCOV_MODE_TRACE_CMP;
    }

    return -ENOTSUPP;

    else {
    return -EINVAL;
    }
    }
//
// Fault in a lazily-faulted vmalloc area before it can be used by
// __sanitizer_cov_trace_pc(), to avoid recursion issues if any code on the
// vmalloc fault handling path is instrumented.
//
#[no_mangle]
unsafe extern "C" fn kcov_fault_in_area(kcov: *mut kcov) {
pub static mut stride: c_ulong = 0;
    let mut area = kcov.area;
    let mut offset = 0;
    for (offset = 0; offset < kcov.size; offset += stride) {
// READ_ONCE;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kcov_check_handle() {
    if (handle & ~(KCOV_SUBSYSTEM_MASK | KCOV_INSTANCE_MASK)) {
    return false;
    }
    match (handle & KCOV_SUBSYSTEM_MASK) {
    KCOV_SUBSYSTEM_COMMON => {
    return (handle & KCOV_INSTANCE_MASK) ?
    common_valid : zero_valid;
    }
    KCOV_SUBSYSTEM_USB => {
    return uncommon_valid;
    }
    _ => {
    return false;
    }
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn kcov_ioctl_locked() {
    let mut t = core::ptr::null_mut();
    unsigned long flags, unused;
    let mut mode = 0;
    let mut i = 0;
    let mut remote_arg = core::ptr::null_mut();
    let mut remote = core::ptr::null_mut();
    match (cmd) {
    KCOV_ENABLE => {
//
// Enable coverage for the current task.
// At this point user must have been enabled trace mode,
// and mmapped the file. Coverage collection is disabled only
// at task exit or voluntary by KCOV_DISABLE. After that it can
// be enabled for another task.
//
    if (kcov.mode != KCOV_MODE_INIT || !kcov.area) {
    return -EINVAL;
    }
    t = current;
    if (kcov.t != core::ptr::null_mut() || t.kcov != core::ptr::null_mut()) {
    return -EBUSY;
    }
    mode = kcov_get_mode(arg);
    if (mode < 0) {
    return mode;
    }
    kcov_fault_in_area(kcov);
    kcov.mode = mode;
    kcov_start(t, kcov, kcov.size, kcov.area, kcov.mode,
    kcov.sequence);
    kcov.t = t;
// Put either in kcov_task_exit() or in KCOV_DISABLE.
    kcov_get(kcov);
    return 0;
    }
    KCOV_DISABLE => {
// Disable coverage for the current task.
    unused = arg;
    t = current;
    if (unused != 0 || (kcov != t.kcov && kcov != t.kcov_remote)) {
    return -EINVAL;
    }
    if (WARN_ON!(kcov.t != t)) {
    return -EINVAL;
    }
    kcov_disable(t, kcov);
    kcov_put(kcov);
    return 0;
    }
    KCOV_REMOTE_ENABLE => {
    if (kcov.mode != KCOV_MODE_INIT || !kcov.area) {
    return -EINVAL;
    }
    t = current;
    if (kcov.t != core::ptr::null_mut() || t.kcov_remote != core::ptr::null_mut()) {
    return -EBUSY;
    }
    remote_arg = arg;
    mode = kcov_get_mode(remote_arg.trace_mode);
    if (mode < 0) {
    return mode;
    }
    if ((unsigned long)remote_arg.area_size >
    LONG_MAX / sizeof!(unsigned long)) {
    return -EINVAL;
    }
    kcov.mode = mode;
    t.kcov_remote = kcov;
    kcov.t = t;
    kcov.remote = true;
    kcov.remote_size = remote_arg.area_size;
    spin_lock_irqsave(&kcov_remote_lock, flags);
    while (i < remote_arg.num_handles) {
    if (!kcov_check_handle(remote_arg.handles[i],
    false, true, false)) {
    spin_unlock_irqrestore(&kcov_remote_lock,
    flags);
    kcov_disable(t, kcov);
    return -EINVAL;
    }
    remote = kcov_remote_add(kcov, remote_arg.handles[i]);
    if (IS_ERR(remote)) {
    spin_unlock_irqrestore(&kcov_remote_lock,
    flags);
    kcov_disable(t, kcov);
    return PTR_ERR(remote);
    }
    }
    if (remote_arg.common_handle) {
    if (!kcov_check_handle(remote_arg.common_handle,
    true, false, false)) {
    spin_unlock_irqrestore(&kcov_remote_lock,
    flags);
    kcov_disable(t, kcov);
    return -EINVAL;
    }
    remote = kcov_remote_add(kcov,
    remote_arg.common_handle);
    if (IS_ERR(remote)) {
    spin_unlock_irqrestore(&kcov_remote_lock,
    flags);
    kcov_disable(t, kcov);
    return PTR_ERR(remote);
    }
    t.kcov_handle = remote_arg.common_handle;
    }
    spin_unlock_irqrestore(&kcov_remote_lock, flags);
// Put either in kcov_task_exit() or in KCOV_DISABLE.
    kcov_get(kcov);
    return 0;
    }
    _ => {
    return -ENOTTY;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn kcov_ioctl(filep: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let mut kcov = core::ptr::null_mut();
    let mut res = 0;
    let mut remote_arg = core::ptr::null_mut();
    let mut remote_num_handles = 0;
    let mut remote_arg_size = 0;
    unsigned long size, flags;
    let mut area = core::ptr::null_mut();
    kcov = filep.private_data;
    match (cmd) {
    KCOV_INIT_TRACE => {
//
// Enable kcov in trace mode and setup buffer size.
// Must happen before anything else.
//
// First check the size argument - it must be at least 2
// to hold the current position and one PC.
//
    size = arg;
    if (size < 2 || size > INT_MAX / sizeof!(unsigned long)) {
    return -EINVAL;
    }
    area = vmalloc_user(size * sizeof!(unsigned long));
    if (area == core::ptr::null_mut()) {
    return -ENOMEM;
    }
    spin_lock_irqsave(&kcov.lock, flags);
    if (kcov.mode != KCOV_MODE_DISABLED) {
    spin_unlock_irqrestore(&kcov.lock, flags);
    vfree(area);
    return -EBUSY;
    }
    kcov.area = area;
    kcov.size = size;
    kcov.mode = KCOV_MODE_INIT;
    spin_unlock_irqrestore(&kcov.lock, flags);
    return 0;
    }
    KCOV_REMOTE_ENABLE => {
    if (get_user(remote_num_handles, (arg +
    offsetof(kcov_remote_arg, num_handles)))) {
    return -EFAULT;
    }
    if (remote_num_handles > KCOV_REMOTE_MAX_HANDLES) {
    return -EINVAL;
    }
    remote_arg_size = struct_size(remote_arg, handles,
    remote_num_handles);
    remote_arg = memdup_user(arg, remote_arg_size);
    if (IS_ERR(remote_arg)) {
    return PTR_ERR(remote_arg);
    }
    if (remote_arg.num_handles != remote_num_handles) {
    kfree(remote_arg);
    return -EINVAL;
    }
    arg = (unsigned long)remote_arg;
    fallthrough;
    }
    _ => {
//
// All other commands can be normally executed under a spin lock, so we
// obtain and release it here in order to simplify kcov_ioctl_locked().
//
    spin_lock_irqsave(&kcov.lock, flags);
    res = kcov_ioctl_locked(kcov, cmd, arg);
    spin_unlock_irqrestore(&kcov.lock, flags);
    kfree(remote_arg);
    return res;
    }
    }
    }
pub static mut file_operations: usize = 0;
//
// kcov_remote_start() and kcov_remote_stop() can be used to annotate a section
// of code in a kernel background thread or in a softirq to allow kcov to be
// used to collect coverage from that part of code.
//
// The handle argument of kcov_remote_start() identifies a code section that is
// used for coverage collection. A userspace process passes this handle to
// KCOV_REMOTE_ENABLE ioctl to make the used kcov device start collecting
// coverage for the code section identified by this handle.
//
// The usage of these annotations in the kernel code is different depending on
// the type of the kernel thread whose code is being annotated.
//
// For global kernel threads that are spawned in a limited number of instances
// (e.g. one USB hub_event() worker thread is spawned per USB HCD) and for
// softirqs, each instance must be assigned a unique 4-byte instance id. The
// instance id is then combined with a 1-byte subsystem id to get a handle via
// kcov_remote_handle(subsystem_id, instance_id).
//
// For local kernel threads that are spawned from system calls handler when a
// user interacts with some kernel interface (e.g. vhost workers), a handle is
// passed from a userspace process as the common_handle field of the
// kcov_remote_arg struct (note, that the user must generate a handle by using
// kcov_remote_handle() with KCOV_SUBSYSTEM_COMMON as the subsystem id and an
// arbitrary 4-byte non-zero number as the instance id). This common handle
// then gets saved into the task_struct of the process that issued the
// KCOV_REMOTE_ENABLE ioctl. When this process issues system calls that spawn
// kernel threads, the common handle must be retrieved via kcov_common_handle()
// and passed to the spawned threads via custom annotations. Those kernel
// threads must in turn be annotated with kcov_remote_start(common_handle) and
// kcov_remote_stop(). All of the threads that are spawned by the same process
// obtain the same handle, hence the name "common".
//
// See Documentation/dev-tools/kcov.rst for more details.
//
// Internally, kcov_remote_start() looks up the kcov device associated with the
// provided handle, allocates an area for coverage collection, and saves the
// pointers to kcov and area into the current task_struct to allow coverage to
// be collected via __sanitizer_cov_trace_pc().
// In turns kcov_remote_stop() clears those pointers from task_struct to stop
// collecting coverage and copies all collected coverage into the kcov area.
//
#[no_mangle]
pub unsafe extern "C" fn kcov_mode_enabled(mode: c_uint) -> bool {
    return (mode & ~KCOV_IN_CTXSW) != KCOV_MODE_DISABLED;
    }
#[no_mangle]
unsafe extern "C" fn kcov_remote_softirq_start(t: *mut task_struct) {
    let mut mode = 0;
    mode = READ_ONCE(t.kcov_mode);
    barrier();
    if (kcov_mode_enabled(mode)) {
    t.kcov_saved_mode = mode;
    t.kcov_saved_size = t.kcov_size;
    t.kcov_saved_area = t.kcov_area;
    t.kcov_saved_sequence = t.kcov_sequence;
    t.kcov_saved_kcov = t.kcov;
    kcov_stop(t);
    }
    }
#[no_mangle]
unsafe extern "C" fn kcov_remote_softirq_stop(t: *mut task_struct) {
    if (t.kcov_saved_kcov) {
    kcov_start(t, t.kcov_saved_kcov, t.kcov_saved_size,
    t.kcov_saved_area, t.kcov_saved_mode,
    t.kcov_saved_sequence);
    t.kcov_saved_mode = 0;
    t.kcov_saved_size = 0;
    t.kcov_saved_area = core::ptr::null_mut();
    t.kcov_saved_sequence = 0;
    t.kcov_saved_kcov = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kcov_remote_start(handle: u64) {
    let mut t = current;
    let mut remote = core::ptr::null_mut();
    let mut kcov = core::ptr::null_mut();
    let mut mode = 0;
    let mut area = core::ptr::null_mut();
    let mut size = 0;
    let mut sequence = 0;
    let mut flags = 0;
    if (WARN_ON!(!kcov_check_handle(handle, true, true, true))) {
    return;
    }
    if (!in_task() && !in_softirq_really()) {
    return;
    }
    local_lock_irqsave(&kcov_percpu_data.lock, flags);
//
// Check that kcov_remote_start() is not called twice in background
// threads nor called by user tasks (with enabled kcov).
//
    mode = READ_ONCE(t.kcov_mode);
    if (WARN_ON!(in_task() && kcov_mode_enabled(mode))) {
    local_unlock_irqrestore(&kcov_percpu_data.lock, flags);
    return;
    }
//
// Check that kcov_remote_start() is not called twice in softirqs.
// Note, that kcov_remote_start() can be called from a softirq that
// happened while collecting coverage from a background thread.
//
    if (WARN_ON!(in_serving_softirq() && t.kcov_softirq)) {
    local_unlock_irqrestore(&kcov_percpu_data.lock, flags);
    return;
    }
    spin_lock(&kcov_remote_lock);
    remote = kcov_remote_find(handle);
    if (!remote) {
    spin_unlock(&kcov_remote_lock);
    local_unlock_irqrestore(&kcov_percpu_data.lock, flags);
    return;
    }
    kcov_debug("handle = %llx, context: %s\n", handle,
    in_task() ? "task" : "softirq");
    kcov = remote.kcov;
// Put in kcov_remote_stop().
    kcov_get(kcov);
//
// Read kcov fields before unlocking kcov_remote_lock to prevent races
// with KCOV_DISABLE and kcov_remote_reset(); cannot acquire kcov->lock
// here, because it might lead to deadlock given kcov_remote_lock is
// acquired _after_ kcov->lock elsewhere.
//
    mode = context_unsafe(kcov.mode);
    sequence = kcov.sequence;
    if (in_task()) {
    size = kcov.remote_size;
    area = kcov_remote_area_get(size, false);
    } else {
    size = CONFIG_KCOV_IRQ_AREA_SIZE;
    area = kcov_remote_area_get(size, true);
    }
    spin_unlock(&kcov_remote_lock);
// Allocate new buffer if we can sleep.
    if (!area) {
    local_unlock_irqrestore(&kcov_percpu_data.lock, flags);
    area = in_task() ? vmalloc(size * sizeof!(unsigned long)) : core::ptr::null_mut();
    if (!area) {
    kcov_put(kcov);
    return;
    }
    local_lock_irqsave(&kcov_percpu_data.lock, flags);
    }
// Reset coverage size.
// area = 0;
    if (in_serving_softirq()) {
    kcov_remote_softirq_start(t);
    t.kcov_softirq = 1;
    }
    kcov_start(t, kcov, size, area, mode, sequence);
    local_unlock_irqrestore(&kcov_percpu_data.lock, flags);
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn kcov_move_area() {
pub static mut word_size: u64 = 0;
    u64 count_size, entry_size_log;
    u64 dst_len, src_len;
    let mut dst_entries = core::ptr::null_mut();
    let mut src_entries = core::ptr::null_mut();
    u64 dst_occupied, dst_free, bytes_to_move, entries_moved;
    kcov_debug("%px %u <= %px %lu\n",
    dst_area, dst_area_size, src_area, *src_area);
    match (mode) {
    KCOV_MODE_TRACE_PC => {
    dst_len = READ_ONCE(*dst_area);
    src_len = *src_area;
    count_size = sizeof!(unsigned long);
    entry_size_log = __ilog2_u64(sizeof!(unsigned long));
    // break;
    }
    KCOV_MODE_TRACE_CMP => {
    dst_len = READ_ONCE(*dst_area);
    src_len = *src_area;
    count_size = sizeof!(u64);
// BUILD_BUG_ON;
    entry_size_log = __ilog2_u64(sizeof!(u64) * KCOV_WORDS_PER_CMP);
    // break;
    }
    _ => {
// WARN_ON;
    return;
    }
    }
// As arm can't divide u64 integers use log of entry size.
    if (dst_len > ((dst_area_size * word_size - count_size) >>
    entry_size_log)) {
    return;
    }
    dst_occupied = count_size + (dst_len << entry_size_log);
    dst_free = dst_area_size * word_size - dst_occupied;
    bytes_to_move = min(dst_free, src_len << entry_size_log);
    dst_entries = dst_area + dst_occupied;
    src_entries = src_area + count_size;
    memcpy(dst_entries, src_entries, bytes_to_move);
    entries_moved = bytes_to_move >> entry_size_log;
//
// A write memory barrier is required here, to ensure
// that the writes from the memcpy() are visible before
// the count is updated. Without this, it is possible for
// a user to observe a new count value but stale
// coverage data.
//
    smp_wmb();
    match (mode) {
    KCOV_MODE_TRACE_PC => {
// WRITE_ONCE;
    // break;
    }
    KCOV_MODE_TRACE_CMP => {
// WRITE_ONCE;
    // break;
    }
    _ => {
    // break;
    }
    }
    }
// See the comment before kcov_remote_start() for usage details.
#[no_mangle]
pub unsafe extern "C" fn kcov_remote_stop() {
    let mut t = current;
    let mut kcov = core::ptr::null_mut();
    let mut mode = 0;
    let mut area = core::ptr::null_mut();
    let mut size = 0;
    let mut sequence = 0;
    let mut flags = 0;
    if (!in_task() && !in_softirq_really()) {
    return;
    }
    local_lock_irqsave(&kcov_percpu_data.lock, flags);
    mode = READ_ONCE(t.kcov_mode);
    barrier();
    if (!kcov_mode_enabled(mode)) {
    local_unlock_irqrestore(&kcov_percpu_data.lock, flags);
    return;
    }
//
// When in softirq, check if the corresponding kcov_remote_start()
// actually found the remote handle and started collecting coverage.
//
    if (in_serving_softirq() && !t.kcov_softirq) {
    local_unlock_irqrestore(&kcov_percpu_data.lock, flags);
    return;
    }
// Make sure that kcov_softirq is only set when in softirq.
    if (WARN_ON!(!in_serving_softirq() && t.kcov_softirq)) {
    local_unlock_irqrestore(&kcov_percpu_data.lock, flags);
    return;
    }
    kcov = t.kcov;
    area = t.kcov_area;
    size = t.kcov_size;
    sequence = t.kcov_sequence;
    kcov_stop(t);
    if (in_serving_softirq()) {
    t.kcov_softirq = 0;
    kcov_remote_softirq_stop(t);
    }
    spin_lock(&kcov.lock);
//
// KCOV_DISABLE could have been called between kcov_remote_start()
// and kcov_remote_stop(), hence the sequence check.
//
    if (sequence == kcov.sequence && kcov.remote) {
    kcov_move_area(kcov.mode, kcov.area, kcov.size, area);
    }
    spin_unlock(&kcov.lock);
    spin_lock(&kcov_remote_lock);
    kcov_remote_area_put(area, size, !in_task());
    spin_unlock(&kcov_remote_lock);
    local_unlock_irqrestore(&kcov_percpu_data.lock, flags);
// Get in kcov_remote_start().
    kcov_put(kcov);
    }
// EXPORT_SYMBOL;
// See the comment before kcov_remote_start() for usage details.
#[no_mangle]
pub unsafe extern "C" fn kcov_common_handle() -> kcov_common_handle_id {
    if (!in_task()) {
    return (kcov_common_handle_id){ .val = 0 };
    }
    return (kcov_common_handle_id){ .val = current.kcov_handle };
    }
// EXPORT_SYMBOL;

#[no_mangle]
unsafe extern "C" fn selftest() -> c_int {
    let mut start = 0;
    pr_err!("running self test\n");
//
// Test that interrupts don't produce spurious coverage.
// The coverage callback filters out interrupt code, but only
// after the handler updates preempt count. Some code periodically
// leaks out of that section and leads to spurious coverage.
// It's hard to call the actual interrupt handler directly,
// so we just loop here for a bit waiting for a timer interrupt.
// We set kcov_mode to enable tracing, but don't setup the area,
// so any attempt to trace will crash. Note: we must not call any
// potentially traced functions in this region.
//
    start = jiffies;
// WRITE_ONCE;
    while ((jiffies - start) * MSEC_PER_SEC / HZ < 300) {
    ;
    }
// WRITE_ONCE;
    pr_err!("done running self test\n");
    }

#[no_mangle]
unsafe extern "C" fn kcov_init() -> c_int {
pub static mut cpu: c_int = 0;

// Allocate some extra buffers in order to prepare for softirq preemption.
    cpu = cpu >= 4 ? cpu * 2 : cpu + 4;

    while (cpu--) {
    let mut area = vmalloc(CONFIG_KCOV_IRQ_AREA_SIZE * sizeof!(unsigned long));
    let mut flags = 0;
    if (!area) {
    return -ENOMEM;
    }
    spin_lock_irqsave(&kcov_remote_lock, flags);
    kcov_remote_area_put(area, CONFIG_KCOV_IRQ_AREA_SIZE, true);
    spin_unlock_irqrestore(&kcov_remote_lock, flags);
    }
//
// The kcov debugfs file won't ever get removed and thus,
// there is no need to protect it against removal races. The
// use of debugfs_create_file_unsafe() is actually safe here.
//
    debugfs_create_file_unsafe("kcov", 0600, core::ptr::null_mut(), core::ptr::null_mut(), &kcov_fops);

    selftest();

    return 0;
    }
// device_initcall;
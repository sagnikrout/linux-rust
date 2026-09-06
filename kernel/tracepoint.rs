//! Automatically rewritten from C to Rust
//! Source: kernel/tracepoint.c
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



// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2008-2014 Mathieu Desnoyers
//

    enum tp_func_state {
    TP_FUNC_0,
    TP_FUNC_1,
    TP_FUNC_2,
    TP_FUNC_N,
    };
    extern tracepoint_ptr_t __start___tracepoints_ptrs[];
    extern tracepoint_ptr_t __stop___tracepoints_ptrs[];
    enum tp_transition_sync {
    TP_TRANSITION_SYNC_1_0_1,
    TP_TRANSITION_SYNC_N_2_1,
    _NR_TP_TRANSITION_SYNC,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tp_transition_snapshot {
    pub rcu: c_ulong,
    pub srcu_gp: c_ulong,
    pub ongoing: bool,
}

pub static mut tracepoint_srcu: usize = 0;
    EXPORT_SYMBOL_GPL(tracepoint_srcu);
// Protected by tracepoints_mutex
    static struct tp_transition_snapshot tp_transition_snapshot[_NR_TP_TRANSITION_SYNC];
#[no_mangle]
unsafe extern "C" fn tp_rcu_get_state(sync: tp_transition_sync) {
    let mut snapshot = &tp_transition_snapshot[sync];
// Keep the latest get_state snapshot.
    snapshot.rcu = get_state_synchronize_rcu();
    snapshot.srcu_gp = start_poll_synchronize_srcu(&tracepoint_srcu);
    snapshot.ongoing = true;
    }
#[no_mangle]
unsafe extern "C" fn tp_rcu_cond_sync(sync: tp_transition_sync) {
    let mut snapshot = &tp_transition_snapshot[sync];
    if (!snapshot.ongoing) {
    return;
    }
    cond_synchronize_rcu(snapshot.rcu);
    if (!poll_state_synchronize_srcu(&tracepoint_srcu, snapshot.srcu_gp)) {
    synchronize_srcu(&tracepoint_srcu);
    }
    snapshot.ongoing = false;
    }
// Set to 1 to enable tracepoint debug output
    static const int tracepoint_debug;

//
// Tracepoint module list mutex protects the local module list.
//
// static DEFINE_MUTEX(tracepoint_module_list_mutex);
// Local list of struct tp_module
// static LIST_HEAD(tracepoint_module_list);

//
// tracepoints_mutex protects the builtin and module tracepoints.
// tracepoints_mutex nests inside tracepoint_module_list_mutex.
//
// static DEFINE_MUTEX(tracepoints_mutex);
//
// Note about RCU :
// It is used to delay the free of multiple probes array until a quiescent
// state is reached.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tp_probes {
    pub rcu: rcu_head,
    pub probes: [tracepoint_func; 0],
}

// Called in removal of a func but failed to allocate a new tp_funcs
#[no_mangle]
unsafe extern "C" fn tp_stub_func() {
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn allocate_probes(count: c_int) -> *mut c_void {
    let mut p = kmalloc_flex(*p, probes, count);
pub static mut p: return = 0;
    }
#[no_mangle]
unsafe extern "C" fn rcu_free_old_probes(head: *mut rcu_head) {
    kfree(container_of!(head, tp_probes, rcu));
    }
#[no_mangle]
pub unsafe extern "C" fn release_probes(tp: *mut tracepoint, old: *mut tracepoint_func) {
    if (old) {
    let mut tp_probes = container_of!(old, tp_probes, probes[0]);
    if (tracepoint_is_faultable(tp)) {
    call_rcu_tasks_trace(&tp_probes.rcu,
    rcu_free_old_probes);
    } else {
    call_srcu(&tracepoint_srcu, &tp_probes.rcu,
    rcu_free_old_probes);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn debug_print_probes(funcs: *mut tracepoint_func) {
    let mut i = 0;
    if (!tracepoint_debug || !funcs) {
    return;
    }
    for (i = 0; funcs[i].func; i++) {
    printk("Probe %d : %pSb\n", i, funcs[i].func);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn func_add(funcs: *mut *mut tracepoint_func, tp_func: *mut tracepoint_func, prio: c_int) -> *mut c_void {
    let mut old = core::ptr::null_mut();
    let mut new = core::ptr::null_mut();
    let mut iter_probes = 0;	/* Iterate over old probe array. */
    let mut nr_probes = 0;	/* Counter for probes */
    let mut pos = -1;		/* Insertion position into new array */
    if (WARN_ON!(!tp_func.func)) {
    return ERR_PTR(-EINVAL);
    }
    debug_print_probes(*funcs);
    old = *funcs;
    if (old) {
// (N -> N+1), (N != 0, 1) probes
    while (old[iter_probes].func) {
    if (old[iter_probes].func == tp_stub_func) {
    continue;	/* Skip stub functions. */
    }
    if (old[iter_probes].func == tp_func.func &&
    old[iter_probes].data == tp_func.data) {
    return ERR_PTR(-EEXIST);
    }
    nr_probes += 1;
    }
    }
// + 2 : one for new probe, one for NULL func
    new = allocate_probes(nr_probes + 2);
    if (new == core::ptr::null_mut()) {
    return ERR_PTR(-ENOMEM);
    }
    if (old) {
    nr_probes = 0;
    while (old[iter_probes].func) {
    if (old[iter_probes].func == tp_stub_func) {
    continue;
    }
// Insert before probes of lower priority
    if (pos < 0 && old[iter_probes].prio < prio) {
    pos = nr_probes += 1;
    }
    new[nr_probes++] = old[iter_probes];
    }
    if (pos < 0) {
    pos = nr_probes += 1;
    }
// nr_probes now points to the end of the new array
    } else {
    pos = 0;
    nr_probes = 1; /* must point at end of array */
    }
    new[pos] = *tp_func;
    new[nr_probes].func = core::ptr::null_mut();
// funcs = new;
    debug_print_probes(*funcs);
    return old;
    }
#[no_mangle]
pub unsafe extern "C" fn func_remove(funcs: *mut *mut tracepoint_func, tp_func: *mut tracepoint_func) -> *mut c_void {
pub static mut nr_probes: c_int = 0;
    let mut old = core::ptr::null_mut();
    let mut new = core::ptr::null_mut();
    old = *funcs;
    if (!old) {
    return ERR_PTR(-ENOENT);
    }
    debug_print_probes(*funcs);
// (N -> M), (N > 1, M >= 0) probes
    if (tp_func.func) {
    while (old[nr_probes].func) {
    if ((old[nr_probes].func == tp_func.func &&
    old[nr_probes].data == tp_func.data) ||
    old[nr_probes].func == tp_stub_func) {
    nr_del += 1;
    }
    }
    }
//
// If probe is NULL, then nr_probes = nr_del = 0, and then the
// entire entry will be removed.
//
    if (nr_probes - nr_del == 0) {
// N -> 0, (N > 1)
// funcs = NULL;
    debug_print_probes(*funcs);
    return old;
    } else {
pub static mut j: c_int = 0;
// N -> M, (N > 1, M > 0)
// + 1 for NULL
    new = allocate_probes(nr_probes - nr_del + 1);
    if (new) {
    while (old[i].func) {
    if ((old[i].func != tp_func.func ||
    old[i].data != tp_func.data) &&
    old[i].func != tp_stub_func) {
    new[j++] = old[i];
    }
    }
    new[nr_probes - nr_del].func = core::ptr::null_mut();
// funcs = new;
    } else {
//
// Failed to allocate, replace the old function
// with calls to tp_stub_func.
//
    while (old[i].func) {
    if (old[i].func == tp_func.func &&
    old[i].data == tp_func.data) {
    WRITE_ONCE(old[i].func, tp_stub_func);
    }
    }
// funcs = old;
    }
    }
    debug_print_probes(*funcs);
    return old;
    }
//
// Count the number of functions (enum tp_func_state) in a tp_funcs array.
//
#[no_mangle]
unsafe extern "C" fn nr_func_state(tp_funcs: *const tracepoint_func) -> enum tp_func_state {
    if (!tp_funcs) {
    return TP_FUNC_0;
    }
    if (!tp_funcs[1].func) {
    return TP_FUNC_1;
    }
    if (!tp_funcs[2].func) {
    return TP_FUNC_2;
    }
    return TP_FUNC_N;	/* 3 or more */
    }
#[no_mangle]
unsafe extern "C" fn tracepoint_update_call(tp: *mut tracepoint, tp_funcs: *mut tracepoint_func) {
    let mut func = tp.iterator;
// Synthetic events do not have static call sites
    if (!tp.static_call_key) {
    return;
    }
    if (nr_func_state(tp_funcs) == TP_FUNC_1) {
    func = tp_funcs[0].func;
    }
    __static_call_update(tp.static_call_key, tp.static_call_tramp, func);
    }
//
// Add the probe function to a tracepoint.
//
#[no_mangle]
pub unsafe extern "C" fn tracepoint_add_func(tp: *mut tracepoint, func: *mut tracepoint_func, prio: c_int, warn: bool) -> c_int {
    let mut old = core::ptr::null_mut();
    let mut tp_funcs = core::ptr::null_mut();
    let mut ret = 0;
    if (tp.ext && tp.ext.regfunc && !static_key_enabled(&tp.key)) {
    ret = tp.ext.regfunc();
    if (ret < 0) {
    return ret;
    }
    }
    tp_funcs = rcu_dereference_protected(tp.funcs,
    lockdep_is_held(&tracepoints_mutex));
    old = func_add(&tp_funcs, func, prio);
    if (IS_ERR(old)) {
    if (tp.ext && tp.ext.unregfunc && !static_key_enabled(&tp.key)) {
    tp.ext.unregfunc();
    }
    WARN_ON_ONCE!(warn && PTR_ERR(old) != -ENOMEM);
    return PTR_ERR(old);
    }
//
// rcu_assign_pointer has as smp_store_release() which makes sure
// that the new probe callbacks array is consistent before setting
// a pointer to it.  This array is referenced by __DO_TRACE from
// include/linux/tracepoint.h using rcu_dereference_sched().
//
    switch (nr_func_state(tp_funcs)) {
    case TP_FUNC_1:		/* 0.1 */
//
// Make sure new static func never uses old data after a
// 1->0->1 transition sequence.
//
    tp_rcu_cond_sync(TP_TRANSITION_SYNC_1_0_1);
// Set static call to first function
    tracepoint_update_call(tp, tp_funcs);
// Both iterator and static call handle NULL tp->funcs
    rcu_assign_pointer(tp.funcs, tp_funcs);
    static_branch_enable(&tp.key);
    break;
    case TP_FUNC_2:		/* 1.2 */
// Set iterator static call
    tracepoint_update_call(tp, tp_funcs);
//
// Iterator callback installed before updating tp->funcs.
// Requires ordering between RCU assign/dereference and
// static call update/call.
//
    fallthrough;
    case TP_FUNC_N:		/* N.N+1 (N>1) */
    rcu_assign_pointer(tp.funcs, tp_funcs);
//
// Make sure static func never uses incorrect data after a
// N->...->2->1 (N>1) transition sequence.
//
    if (tp_funcs[0].data != old[0].data) {
    tp_rcu_get_state(TP_TRANSITION_SYNC_N_2_1);
    }
    break;
// label;
    WARN_ON_ONCE!(1);
    break;
    }
    release_probes(tp, old);
    return 0;
    }
//
// Remove a probe function from a tracepoint.
// Note: only waiting an RCU period after setting elem->call to the empty
// function insures that the original callback is not used anymore. This insured
// by preempt_disable around the call site.
//
#[no_mangle]
pub unsafe extern "C" fn tracepoint_remove_func(tp: *mut tracepoint, func: *mut tracepoint_func) -> c_int {
    let mut old = core::ptr::null_mut();
    let mut tp_funcs = core::ptr::null_mut();
    tp_funcs = rcu_dereference_protected(tp.funcs,
    lockdep_is_held(&tracepoints_mutex));
    old = func_remove(&tp_funcs, func);
    if (WARN_ON_ONCE!(IS_ERR(old))) {
    return PTR_ERR(old);
    }
    if (tp_funcs == old) {
// Failed allocating new tp_funcs, replaced func with stub
    return 0;
    }
    switch (nr_func_state(tp_funcs)) {
    case TP_FUNC_0:		/* 1.0 */
// Removed last function
    if (tp.ext && tp.ext.unregfunc && static_key_enabled(&tp.key)) {
    tp.ext.unregfunc();
    }
    static_branch_disable(&tp.key);
// Set iterator static call
    tracepoint_update_call(tp, tp_funcs);
// Both iterator and static call handle NULL tp->funcs
    rcu_assign_pointer(tp.funcs, core::ptr::null_mut());
//
// Make sure new static func never uses old data after a
// 1->0->1 transition sequence.
//
    tp_rcu_get_state(TP_TRANSITION_SYNC_1_0_1);
    break;
    case TP_FUNC_1:		/* 2.1 */
    rcu_assign_pointer(tp.funcs, tp_funcs);
//
// Make sure static func never uses incorrect data after a
// N->...->2->1 (N>2) transition sequence. If the first
// element's data has changed, then force the synchronization
// to prevent current readers that have loaded the old data
// from calling the new function.
//
    if (tp_funcs[0].data != old[0].data) {
    tp_rcu_get_state(TP_TRANSITION_SYNC_N_2_1);
    }
    tp_rcu_cond_sync(TP_TRANSITION_SYNC_N_2_1);
// Set static call to first function
    tracepoint_update_call(tp, tp_funcs);
    break;
    case TP_FUNC_2:		/* N.N-1 (N>2) */
    fallthrough;
    case TP_FUNC_N:
    rcu_assign_pointer(tp.funcs, tp_funcs);
//
// Make sure static func never uses incorrect data after a
// N->...->2->1 (N>2) transition sequence.
//
    if (tp_funcs[0].data != old[0].data) {
    tp_rcu_get_state(TP_TRANSITION_SYNC_N_2_1);
    }
    break;
// label;
    WARN_ON_ONCE!(1);
    break;
    }
    release_probes(tp, old);
    return 0;
    }
//
// tracepoint_probe_register_prio_may_exist -  Connect a probe to a tracepoint with priority
// @tp: tracepoint
// @probe: probe handler
// @data: tracepoint data
// @prio: priority of this function over other registered functions
//
// Same as tracepoint_probe_register_prio() except that it will not warn
// if the tracepoint is already registered.
//
#[no_mangle]
pub unsafe extern "C" fn tracepoint_probe_register_prio_may_exist(tp: *mut tracepoint, probe: *mut c_void, data: *mut c_void, prio: c_int) -> c_int {
pub static mut tp_func: usize = 0;
    let mut ret = 0;
    mutex_lock(&tracepoints_mutex);
    tp_func.func = probe;
    tp_func.data = data;
    tp_func.prio = prio;
    ret = tracepoint_add_func(tp, &tp_func, prio, false);
    mutex_unlock(&tracepoints_mutex);
    return ret;
    }
    EXPORT_SYMBOL_GPL(tracepoint_probe_register_prio_may_exist);
//
// tracepoint_probe_register_prio -  Connect a probe to a tracepoint with priority
// @tp: tracepoint
// @probe: probe handler
// @data: tracepoint data
// @prio: priority of this function over other registered functions
//
// Returns 0 if ok, error value on error.
// Note: if @tp is within a module, the caller is responsible for
// unregistering the probe before the module is gone. This can be
// performed either with a tracepoint module going notifier, or from
// within module exit functions.
//
#[no_mangle]
pub unsafe extern "C" fn tracepoint_probe_register_prio(tp: *mut tracepoint, probe: *mut c_void, data: *mut c_void, prio: c_int) -> c_int {
pub static mut tp_func: usize = 0;
    let mut ret = 0;
    mutex_lock(&tracepoints_mutex);
    tp_func.func = probe;
    tp_func.data = data;
    tp_func.prio = prio;
    ret = tracepoint_add_func(tp, &tp_func, prio, true);
    mutex_unlock(&tracepoints_mutex);
    return ret;
    }
    EXPORT_SYMBOL_GPL(tracepoint_probe_register_prio);
//
// tracepoint_probe_register -  Connect a probe to a tracepoint
// @tp: tracepoint
// @probe: probe handler
// @data: tracepoint data
//
// Returns 0 if ok, error value on error.
// Note: if @tp is within a module, the caller is responsible for
// unregistering the probe before the module is gone. This can be
// performed either with a tracepoint module going notifier, or from
// within module exit functions.
//
#[no_mangle]
pub unsafe extern "C" fn tracepoint_probe_register(tp: *mut tracepoint, probe: *mut c_void, data: *mut c_void) -> c_int {
    return tracepoint_probe_register_prio(tp, probe, data, TRACEPOINT_DEFAULT_PRIO);
    }
    EXPORT_SYMBOL_GPL(tracepoint_probe_register);
//
// tracepoint_probe_unregister -  Disconnect a probe from a tracepoint
// @tp: tracepoint
// @probe: probe function pointer
// @data: tracepoint data
//
// Returns 0 if ok, error value on error.
//
#[no_mangle]
pub unsafe extern "C" fn tracepoint_probe_unregister(tp: *mut tracepoint, probe: *mut c_void, data: *mut c_void) -> c_int {
pub static mut tp_func: usize = 0;
    let mut ret = 0;
    mutex_lock(&tracepoints_mutex);
    tp_func.func = probe;
    tp_func.data = data;
    ret = tracepoint_remove_func(tp, &tp_func);
    mutex_unlock(&tracepoints_mutex);
    return ret;
    }
    EXPORT_SYMBOL_GPL(tracepoint_probe_unregister);
#[no_mangle]
pub unsafe extern "C" fn for_each_tracepoint_range(begin: *mut tracepoint_ptr_t, end: *mut tracepoint_ptr_t, tp: *mut *mut c_void (fct)( tracepoint, priv: *mut c_void) {
pub static mut iter: *mut c_void = core::ptr::null_mut();
    if (!begin) {
    return;
    }
    for (iter = begin; iter < end; iter++) {
    fct(tracepoint_ptr_deref(iter), priv);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn trace_module_has_bad_taint(mod: *mut module) -> bool {
    return mod.taints & ~((1 << TAINT_OOT_MODULE) | (1 << TAINT_CRAP) |
    (1 << TAINT_UNSIGNED_MODULE) | (1 << TAINT_TEST) |
    (1 << TAINT_LIVEPATCH));
    }
// static BLOCKING_NOTIFIER_HEAD(tracepoint_notify_list);
//
// register_tracepoint_module_notifier - register tracepoint coming/going notifier
// @nb: notifier block
//
// Notifiers registered with this function are called on module
// coming/going with the tracepoint_module_list_mutex held.
// The notifier block callback should expect a "struct tp_module" data
// pointer.
//
#[no_mangle]
pub unsafe extern "C" fn register_tracepoint_module_notifier(nb: *mut notifier_block) -> c_int {
pub static mut tp_mod: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    mutex_lock(&tracepoint_module_list_mutex);
    ret = blocking_notifier_chain_register(&tracepoint_notify_list, nb);
    if (ret) {
// goto;
    }
    list_for_each_entry(tp_mod, &tracepoint_module_list, list) {
    (void) nb.notifier_call(nb, MODULE_STATE_COMING, tp_mod);
    }
// label;
    mutex_unlock(&tracepoint_module_list_mutex);
    return ret;
    }
    EXPORT_SYMBOL_GPL(register_tracepoint_module_notifier);
//
// unregister_tracepoint_module_notifier - unregister tracepoint coming/going notifier
// @nb: notifier block
//
// The notifier block callback should expect a "struct tp_module" data
// pointer.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_tracepoint_module_notifier(nb: *mut notifier_block) -> c_int {
pub static mut tp_mod: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    mutex_lock(&tracepoint_module_list_mutex);
    ret = blocking_notifier_chain_unregister(&tracepoint_notify_list, nb);
    if (ret) {
// goto;
    }
    list_for_each_entry(tp_mod, &tracepoint_module_list, list) {
    (void) nb.notifier_call(nb, MODULE_STATE_GOING, tp_mod);
    }
// label;
    mutex_unlock(&tracepoint_module_list_mutex);
    return ret;
    }
    EXPORT_SYMBOL_GPL(unregister_tracepoint_module_notifier);
//
// Ensure the tracer unregistered the module's probes before the module
// teardown is performed. Prevents leaks of probe and data pointers.
//
#[no_mangle]
unsafe extern "C" fn tp_module_going_check_quiescent(tp: *mut tracepoint, priv: *mut c_void) {
    WARN_ON_ONCE!(tp.funcs);
    }
#[no_mangle]
unsafe extern "C" fn tracepoint_module_coming(mod: *mut module) -> c_int {
pub static mut tp_mod: *mut c_void = core::ptr::null_mut();
    if (!mod.num_tracepoints) {
    return 0;
    }
//
// We skip modules that taint the kernel, especially those with different
// module headers (for forced load), to make sure we don't cause a crash.
// Staging, out-of-tree, unsigned GPL, and test modules are fine.
//
    if (trace_module_has_bad_taint(mod)) {
    return 0;
    }
    tp_mod = kmalloc_obj(tp_module);
    if (!tp_mod) {
    return -ENOMEM;
    }
    tp_mod.mod = mod;
    mutex_lock(&tracepoint_module_list_mutex);
    list_add_tail(&tp_mod.list, &tracepoint_module_list);
    blocking_notifier_call_chain(&tracepoint_notify_list,
    MODULE_STATE_COMING, tp_mod);
    mutex_unlock(&tracepoint_module_list_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tracepoint_module_going(mod: *mut module) {
pub static mut tp_mod: *mut c_void = core::ptr::null_mut();
    if (!mod.num_tracepoints) {
    return;
    }
    mutex_lock(&tracepoint_module_list_mutex);
    list_for_each_entry(tp_mod, &tracepoint_module_list, list) {
    if (tp_mod.mod == mod) {
    blocking_notifier_call_chain(&tracepoint_notify_list,
    MODULE_STATE_GOING, tp_mod);
    list_del(&tp_mod.list);
    kfree(tp_mod);
//
// Called the going notifier before checking for
// quiescence.
//
    for_each_tracepoint_range(mod.tracepoints_ptrs,
    mod.tracepoints_ptrs + mod.num_tracepoints,
    tp_module_going_check_quiescent, core::ptr::null_mut()); {
    break;
    }
    }
    }
//
// In the case of modules that were tainted at "coming", we'll simply
// walk through the list without finding it. We cannot use the "tainted"
// flag on "going", in case a module taints the kernel only after being
// loaded.
//
    mutex_unlock(&tracepoint_module_list_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn tracepoint_module_notify(self: *mut notifier_block, val: c_ulong, data: *mut c_void) -> c_int {
    let mut mod = data;
pub static mut ret: c_int = 0;
    match (val) {
    MODULE_STATE_COMING => {
    ret = tracepoint_module_coming(mod);
    // break;
    }
    MODULE_STATE_LIVE => {
    // break;
    }
    MODULE_STATE_GOING => {
    tracepoint_module_going(mod);
    // break;
    }
    MODULE_STATE_UNFORMED => {
    // break;
    }
    }
    return notifier_from_errno(ret);
    }
pub static mut notifier_block: usize = 0;
#[no_mangle]
unsafe extern "C" fn init_tracepoints() -> __init int {
    let mut ret = 0;
    ret = register_module_notifier(&tracepoint_module_nb);
    if (ret) {
    pr_warn!("Failed to register tracepoint module enter notifier\n");
    }
    return ret;
    }
    __initcall!(init_tracepoints);
//
// for_each_tracepoint_in_module - iteration on all tracepoints in a module
// @mod: module
// @fct: callback
// @priv: private data
//
#[no_mangle]
pub unsafe extern "C" fn for_each_tracepoint_in_module(mod: *mut module, tp: *mut *mut c_void (fct)( tracepoint, mod: *mut module, priv: *mut c_void) {
    let mut begin = core::ptr::null_mut();
    let mut end = core::ptr::null_mut();
    let mut iter = core::ptr::null_mut();
    lockdep_assert_held(&tracepoint_module_list_mutex);
    if (!mod) {
    return;
    }
    begin = mod.tracepoints_ptrs;
    end = mod.tracepoints_ptrs + mod.num_tracepoints;
    for (iter = begin; iter < end; iter++) {
    fct(tracepoint_ptr_deref(iter), mod, priv);
    }
    }
//
// for_each_module_tracepoint - iteration on all tracepoints in all modules
// @fct: callback
// @priv: private data
//
#[no_mangle]
pub unsafe extern "C" fn for_each_module_tracepoint(tp: *mut *mut c_void (fct)( tracepoint, mod: *mut module, priv: *mut c_void) {
pub static mut tp_mod: *mut c_void = core::ptr::null_mut();
    mutex_lock(&tracepoint_module_list_mutex);
    list_for_each_entry(tp_mod, &tracepoint_module_list, list) {
    for_each_tracepoint_in_module(tp_mod.mod, fct, priv);
    }
    mutex_unlock(&tracepoint_module_list_mutex);
    }

//
// for_each_kernel_tracepoint - iteration on all kernel tracepoints
// @fct: callback
// @priv: private data
//
#[no_mangle]
pub unsafe extern "C" fn for_each_kernel_tracepoint(tp: *mut *mut c_void (fct)( tracepoint, priv: *mut c_void) {
    for_each_tracepoint_range(__start___tracepoints_ptrs,
    __stop___tracepoints_ptrs, fct, priv); {
    }
    EXPORT_SYMBOL_GPL(for_each_kernel_tracepoint);
    }

// NB: reg/unreg are called while guarded with the tracepoints_mutex
    static int sys_tracepoint_refcount;
#[no_mangle]
pub unsafe extern "C" fn syscall_regfunc() -> c_int {
    let mut p = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    if (!sys_tracepoint_refcount) {
    read_lock(&tasklist_lock);
    for_each_process_thread(p, t) {
    set_task_syscall_work(t, SYSCALL_TRACEPOINT);
    }
    read_unlock(&tasklist_lock);
    }
    sys_tracepoint_refcount += 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_unregfunc() {
    let mut p = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    sys_tracepoint_refcount -= 1;
    if (!sys_tracepoint_refcount) {
    read_lock(&tasklist_lock);
    for_each_process_thread(p, t) {
    clear_task_syscall_work(t, SYSCALL_TRACEPOINT);
    }
    read_unlock(&tasklist_lock);
    }
    }
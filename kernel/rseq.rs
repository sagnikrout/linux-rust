//! Automatically rewritten from C to Rust
//! Source: kernel/rseq.c
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



// SPDX-License-Identifier: GPL-2.0+
//
// Restartable sequences system call
//
// Copyright (C) 2015, Google, Inc.,
// Paul Turner <pjt@google.com> and Andrew Hunter <ahh@google.com>
// Copyright (C) 2015-2018, EfficiOS Inc.,
// Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
//
// Restartable sequences are a lightweight interface that allows
// user-level code to be executed atomically relative to scheduler
// preemption and signal delivery. Typically used for implementing
// per-cpu operations.
//
// It allows user-space to perform update operations on per-cpu data
// without requiring heavy-weight atomic operations.
//
// Detailed algorithm of rseq user-space assembly sequences:
//
// init(rseq_cs)
// cpu = TLS->rseq::cpu_id_start
// [1]               TLS->rseq::rseq_cs = rseq_cs
// [start_ip]        ----------------------------
// [2]               if (cpu != TLS->rseq::cpu_id)
// goto abort_ip;
// [3]               <last_instruction_in_cs>
// [post_commit_ip]  ----------------------------
//
// The address of jump target abort_ip must be outside the critical
// region, i.e.:
//
// [abort_ip] < [start_ip]  || [abort_ip] >= [post_commit_ip]
//
// Steps [2]-[3] (inclusive) need to be a sequence of instructions in
// userspace that can handle being interrupted between any of those
// instructions, and then resumed to the abort_ip.
//
// 1.  Userspace stores the address of the struct rseq_cs assembly
// block descriptor into the rseq_cs field of the registered
// struct rseq TLS area. This update is performed through a single
// store within the inline assembly instruction sequence.
// [start_ip]
//
// 2.  Userspace tests to check whether the current cpu_id field match
// the cpu number loaded before start_ip, branching to abort_ip
// in case of a mismatch.
//
// If the sequence is preempted or interrupted by a signal
// at or after start_ip and before post_commit_ip, then the kernel
// clears TLS->__rseq_abi::rseq_cs, and sets the user-space return
// ip to abort_ip before returning to user-space, so the preempted
// execution resumes at abort_ip.
//
// 3.  Userspace critical section final instruction before
// post_commit_ip is the commit. The critical section is
// self-terminating.
// [post_commit_ip]
//
// 4.  <success>
//
// On failure at [2], or if interrupted by preempt or signal delivery
// between [1] and [3]:
//
// [abort_ip]
// F1. <failure>
//
// Required to select the proper per_cpu ops for rseq_stats_inc()
// Macro flag: #define RSEQ_BUILD_SLOW_PATH

// Macro flag: #define CREATE_TRACE_POINTS

pub static mut CONFIG_RSEQ_DEBUG_DEFAULT_ENABLE: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn rseq_control_debug(on: bool) {
    if (on) {
    static_branch_enable(&rseq_debug_enabled);
    }
    else {
    static_branch_disable(&rseq_debug_enabled);
    }
    }
#[no_mangle]
unsafe extern "C" fn rseq_setup_debug(str: *mut c_char) -> c_int {
    let mut on = 0;
    if (kstrtobool(str, &on)) {
    return -EINVAL;
    }
    rseq_control_debug(on);
    return 1;
    }
    __setup!("rseq_debug=", rseq_setup_debug);

//
// Out of line, so the actual update functions can be in a header to be
// inlined into the exit to user code.
//
#[no_mangle]
pub unsafe extern "C" fn __rseq_trace_update(t: *mut task_struct) {
    trace_rseq_update(t);
    }
#[no_mangle]
pub unsafe extern "C" fn __rseq_trace_ip_fixup(ip: c_ulong, start_ip: c_ulong, offset: c_ulong, abort_ip: c_ulong) {
    trace_rseq_ip_fixup(ip, start_ip, offset, abort_ip);
    }

pub static mut struct rseq_stats: usize = 0;
#[no_mangle]
unsafe extern "C" fn rseq_stats_show(m: *mut seq_file, p: *mut c_void) -> c_int {
pub static mut stats: rseq_stats = 0;
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    stats.exit	+= data_race(per_cpu(rseq_stats.exit, cpu));
    stats.signal	+= data_race(per_cpu(rseq_stats.signal, cpu));
    stats.slowpath	+= data_race(per_cpu(rseq_stats.slowpath, cpu));
    stats.fastpath	+= data_race(per_cpu(rseq_stats.fastpath, cpu));
    stats.ids	+= data_race(per_cpu(rseq_stats.ids, cpu));
    stats.cs	+= data_race(per_cpu(rseq_stats.cs, cpu));
    stats.clear	+= data_race(per_cpu(rseq_stats.clear, cpu));
    stats.fixup	+= data_race(per_cpu(rseq_stats.fixup, cpu));
    if (IS_ENABLED!(CONFIG_RSEQ_SLICE_EXTENSION)) {
    stats.s_granted	+= data_race(per_cpu(rseq_stats.s_granted, cpu));
    stats.s_expired	+= data_race(per_cpu(rseq_stats.s_expired, cpu));
    stats.s_revoked	+= data_race(per_cpu(rseq_stats.s_revoked, cpu));
    stats.s_yielded	+= data_race(per_cpu(rseq_stats.s_yielded, cpu));
    stats.s_aborted	+= data_race(per_cpu(rseq_stats.s_aborted, cpu));
    }
    }
    seq_printf(m, "exit:   %16lu\n", stats.exit);
    seq_printf(m, "signal: %16lu\n", stats.signal);
    seq_printf(m, "slowp:  %16lu\n", stats.slowpath);
    seq_printf(m, "fastp:  %16lu\n", stats.fastpath);
    seq_printf(m, "ids:    %16lu\n", stats.ids);
    seq_printf(m, "cs:     %16lu\n", stats.cs);
    seq_printf(m, "clear:  %16lu\n", stats.clear);
    seq_printf(m, "fixup:  %16lu\n", stats.fixup);
    if (IS_ENABLED!(CONFIG_RSEQ_SLICE_EXTENSION)) {
    seq_printf(m, "sgrant: %16lu\n", stats.s_granted);
    seq_printf(m, "sexpir: %16lu\n", stats.s_expired);
    seq_printf(m, "srevok: %16lu\n", stats.s_revoked);
    seq_printf(m, "syield: %16lu\n", stats.s_yielded);
    seq_printf(m, "sabort: %16lu\n", stats.s_aborted);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rseq_stats_open(inode: *mut inode, file: *mut file) -> c_int {
    return single_open(file, rseq_stats_show, inode.i_private);
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn rseq_stats_init(root_dir: *mut dentry) -> c_int {
    debugfs_create_file("stats", 0444, root_dir, core::ptr::null_mut(), &stat_ops);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn rseq_stats_init(root_dir: *mut dentry) { }

#[no_mangle]
unsafe extern "C" fn rseq_debug_show(m: *mut seq_file, p: *mut c_void) -> c_int {
pub static mut on: bool = false;
    seq_printf(m, "%d\n", on);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rseq_debug_write(file: *mut file, ubuf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut on = 0;
    if (kstrtobool_from_user(ubuf, count, &on)) {
    return -EINVAL;
    }
    rseq_control_debug(on);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn rseq_debug_open(inode: *mut inode, file: *mut file) -> c_int {
    return single_open(file, rseq_debug_show, inode.i_private);
    }
pub static mut file_operations: usize = 0;
// forward_decl: rseq_slice_ext_init;
#[no_mangle]
unsafe extern "C" fn rseq_debugfs_init() -> c_int {
    let mut root_dir = debugfs_create_dir("rseq", core::ptr::null_mut());
    debugfs_create_file("debug", 0644, root_dir, core::ptr::null_mut(), &debug_ops);
    rseq_stats_init(root_dir);
    if (IS_ENABLED!(CONFIG_RSEQ_SLICE_EXTENSION)) {
    rseq_slice_ext_init(root_dir);
    }
    return 0;
    }
    __initcall!(rseq_debugfs_init);
#[no_mangle]
unsafe extern "C" fn rseq_handle_cs(t: *mut task_struct, regs: *mut pt_regs) -> bool {
    let mut urseq = t.rseq.usrptr;
    let mut csaddr = 0;
    scoped_user_read_access(urseq, efault)
    unsafe_get_user(csaddr, &urseq.rseq_cs, efault);
    if (likely(!csaddr)) {
    return true;
    }
    return rseq_update_user_cs(t, regs, csaddr);
// label;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn rseq_slowpath_update_usr(regs: *mut pt_regs) {
//
// Preserve has_rseq and user_irq state. The generic entry code clears
// user_irq on the way out, the non-generic entry architectures are not
// setting user_irq.
//
pub static mut rseq_event: usize = 0;
    let mut t = current;
pub static mut ids: usize = 0;
    let mut event = 0;
    if (unlikely(t.flags & PF_EXITING)) {
    return;
    }
    rseq_stat_inc(rseq_stats.slowpath);
//
// Read and clear the event pending bit first. If the task
// was not preempted or migrated or a signal is on the way,
// there is no point in doing any of the heavy lifting here
// on production kernels. In that case TIF_NOTIFY_RESUME
// was raised by some other functionality.
//
// This is correct because the read/clear operation is
// guarded against scheduler preemption, which makes it CPU
// local atomic. If the task is preempted right after
// re-enabling preemption then TIF_NOTIFY_RESUME is set
// again and this function is invoked another time _before_
// the task is able to return to user mode.
//
// On a debug kernel, invoke the fixup code unconditionally
// with the result handed in to allow the detection of
// inconsistencies.
//
    scoped_guard(irq) {
    event = t.rseq.event.sched_switch;
    t.rseq.event.all &= evt_mask.all;
    ids.cpu_id = task_cpu(t);
    ids.mm_cid = task_mm_cid(t);
    }
    if (!event) {
    return;
    }
    ids.node_id = cpu_to_node(ids.cpu_id);
    if (unlikely(!rseq_update_usr(t, regs, &ids))) {
//
// Clear the errors just in case this might survive magically, but
// leave the rest intact.
//
    t.rseq.event.error = 0;
    force_sig(SIGSEGV);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __rseq_handle_slowpath(regs: *mut pt_regs) {
//
// If invoked from hypervisors before entering the guest via
// resume_user_mode_work(), then @regs is a NULL pointer.
//
// resume_user_mode_work() clears TIF_NOTIFY_RESUME and re-raises
// it before returning from the ioctl() to user space when
// rseq_event.sched_switch is set.
//
// So it's safe to ignore here instead of pointlessly updating it
// in the vcpu_run() loop.
//
    if (!regs) {
    return;
    }
    rseq_slowpath_update_usr(regs);
    }
#[no_mangle]
pub unsafe extern "C" fn __rseq_signal_deliver(sig: c_int, regs: *mut pt_regs) {
    rseq_stat_inc(rseq_stats.signal);
//
// Don't update IDs yet, they are handled on exit to user if
// necessary. The important thing is to abort a critical section of
// the interrupted context as after this point the instruction
// pointer in @regs points to the signal handler.
//
    if (unlikely(!rseq_handle_cs(current, regs))) {
//
// Clear the errors just in case this might survive
// magically, but leave the rest intact.
//
    current.rseq.event.error = 0;
    force_sigsegv(sig);
    }
//
// In legacy mode, force the update of IDs before returning to user
// space to stay compatible.
//
    if (!rseq_v2(current)) {
    rseq_force_update();
    }
    }
//
// Terminate the process if a syscall is issued within a restartable
// sequence.
//
#[no_mangle]
pub unsafe extern "C" fn __rseq_debug_syscall_return(regs: *mut pt_regs) {
    let mut t = current;
    let mut csaddr = 0;
    if (!t.rseq.event.has_rseq) {
    return;
    }
    if (get_user(csaddr, &t.rseq.usrptr.rseq_cs)) {
// goto;
    }
    if (likely(!csaddr)) {
    return;
    }
    if (unlikely(csaddr >= TASK_SIZE)) {
// goto;
    }
    if (rseq_debug_update_user_cs(t, regs, csaddr)) {
    return;
    }
// label;
    force_sig(SIGSEGV);
    }

// Kept around to keep GENERIC_ENTRY=n architectures supported.
#[no_mangle]
pub unsafe extern "C" fn rseq_syscall(regs: *mut pt_regs) {
    __rseq_debug_syscall_return(regs);
    }

#[no_mangle]
unsafe extern "C" fn rseq_reset_ids() -> bool {
    let mut rseq = current.rseq.usrptr;
//
// If this fails, terminate it because this leaves the kernel in
// stupid state as exit to user space will try to fixup the ids
// again.
//
    scoped_user_rw_access(rseq, efault) {
    unsafe_put_user(0, &rseq.cpu_id_start, efault);
    unsafe_put_user(RSEQ_CPU_ID_UNINITIALIZED, &rseq.cpu_id, efault);
    unsafe_put_user(0, &rseq.node_id, efault);
    unsafe_put_user(0, &rseq.mm_cid, efault);
    }
    return true;
// label;
    force_sig(SIGSEGV);
    return false;
    }
// The original rseq structure size (including padding) is 32 bytes.
pub const ORIG_RSEQ_SIZE: c_int = 32;
#[no_mangle]
unsafe extern "C" fn rseq_register(rseq: *mut *mut rseq , rseq_len: u32, flags: c_int, sig: u32) -> c_long {
pub static mut rseqfl: u32 = 0;
pub static mut version: u8 = 1;
    if (!access_ok(rseq, rseq_len)) {
    return -EFAULT;
    }
//
// Architectures, which use the generic IRQ entry code (at least) enable
// registrations with a size greater than the original v1 fixed sized
// @rseq_len, which has been validated already to utilize the optimized
// v2 ABI mode which also enables extended RSEQ features beyond MMCID.
//
    if (IS_ENABLED!(CONFIG_GENERIC_IRQ_ENTRY) && rseq_len > ORIG_RSEQ_SIZE) {
    version = 2;
    }
    if (IS_ENABLED!(CONFIG_RSEQ_SLICE_EXTENSION) && version > 1) {
    if (rseq_slice_extension_enabled()) {
    rseqfl |= RSEQ_CS_FLAG_SLICE_EXT_AVAILABLE;
    if (flags & RSEQ_FLAG_SLICE_EXT_DEFAULT_ON) {
    rseqfl |= RSEQ_CS_FLAG_SLICE_EXT_ENABLED;
    }
    }
    }
    scoped_user_write_access(rseq, efault) {
//
// If the rseq_cs pointer is non-NULL on registration, clear it to
// avoid a potential segfault on return to user-space. The proper thing
// to do would have been to fail the registration but this would break
// older libcs that reuse the rseq area for new threads without
// clearing the fields. Don't bother reading it, just reset it.
//
    unsafe_put_user(0UL, &rseq.rseq_cs, efault);
    unsafe_put_user(rseqfl, &rseq.flags, efault);
// Initialize IDs in user space
    unsafe_put_user(RSEQ_CPU_ID_UNINITIALIZED, &rseq.cpu_id_start, efault);
    unsafe_put_user(RSEQ_CPU_ID_UNINITIALIZED, &rseq.cpu_id, efault);
    unsafe_put_user(0U, &rseq.node_id, efault);
    unsafe_put_user(0U, &rseq.mm_cid, efault);
//
// All fields past mm_cid are only valid for non-legacy v2
// registrations.
//
    if (version > 1) {
    if (IS_ENABLED!(CONFIG_RSEQ_SLICE_EXTENSION)) {
    unsafe_put_user(0U, &rseq.slice_ctrl.all, efault);
    }
    }
    }
//
// Activate the registration by setting the rseq area address, length
// and signature in the task struct.
//
    current.rseq.usrptr = rseq;
    current.rseq.len = rseq_len;
    current.rseq.sig = sig;

    current.rseq.slice.state.enabled = !!(rseqfl & RSEQ_CS_FLAG_SLICE_EXT_ENABLED);

//
// Ensure the cpu_id_start and cpu_id fields are updated before
// returning to user-space.
//
    current.rseq.event.has_rseq = version;
    rseq_force_update();
    return 0;
// label;
    return -EFAULT;
    }
#[no_mangle]
unsafe extern "C" fn rseq_unregister(rseq: *mut *mut rseq , rseq_len: u32, flags: c_int, sig: u32) -> c_long {
    if (flags & ~RSEQ_FLAG_UNREGISTER) {
    return -EINVAL;
    }
    if (current.rseq.usrptr != rseq || !current.rseq.usrptr) {
    return -EINVAL;
    }
    if (rseq_len != current.rseq.len) {
    return -EINVAL;
    }
    if (current.rseq.sig != sig) {
    return -EPERM;
    }
    if (!rseq_reset_ids()) {
    return -EFAULT;
    }
    rseq_reset(current);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rseq_reregister(rseq: *mut *mut rseq , rseq_len: u32, sig: u32) -> c_long {
//
// If rseq is already registered, check whether the provided address
// differs from the prior one.
//
    if (current.rseq.usrptr != rseq || rseq_len != current.rseq.len) {
    return -EINVAL;
    }
    if (current.rseq.sig != sig) {
    return -EPERM;
    }
// Already registered.
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn rseq_length_valid(rseq: *mut rseq , rseq_len: c_uint) -> bool {
//
// Ensure the provided rseq is properly aligned, as communicated to
// user-space through the ELF auxiliary vector AT_RSEQ_ALIGN. If
// rseq_len is the original rseq size, the required alignment is the
// original struct rseq alignment.
//
// In order to be valid, rseq_len is either the original rseq size, or
// large enough to contain all supported fields, as communicated to
// user-space through the ELF auxiliary vector AT_RSEQ_FEATURE_SIZE.
//
    if (rseq_len < ORIG_RSEQ_SIZE) {
    return false;
    }
    if (rseq_len == ORIG_RSEQ_SIZE) {
    return IS_ALIGNED((unsigned long)rseq, ORIG_RSEQ_SIZE);
    }
    return IS_ALIGNED((unsigned long)rseq, rseq_alloc_align()) &&
    rseq_len >= offsetof(rseq, end);
    }

//
// sys_rseq - Register or unregister restartable sequences for the caller thread.
//
#[no_mangle]
pub unsafe extern "C" fn sys_rseq(rseq: usize, rseq_len: usize, flags: usize, sig: usize) -> c_long {
    if (flags & RSEQ_FLAG_UNREGISTER) {
    return rseq_unregister(rseq, rseq_len, flags, sig);
    }
    if (unlikely(flags & ~RSEQ_FLAGS_SUPPORTED)) {
    return -EINVAL;
    }
    if (current.rseq.usrptr) {
    return rseq_reregister(rseq, rseq_len, sig);
    }
    if (!rseq_length_valid(rseq, rseq_len)) {
    return -EINVAL;
    }
    return rseq_register(rseq, rseq_len, flags, sig);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slice_timer {
    pub timer: hrtimer,
    pub cookie: *mut c_void,
}

pub static mut rseq_slice_ext_nsecs_min: unsigned int = 0;
pub static mut rseq_slice_ext_nsecs_max: unsigned int = 0;
pub static mut : unsigned int rseq_slice_ext_nsecs = 0;
// static DEFINE_PER_CPU(slice_timer, slice_timer);
pub static mut rseq_slice_extension_key: usize = 0;
//
// When the timer expires and the task is still in user space, the return
// from interrupt will revoke the grant and schedule. If the task already
// entered the kernel via a syscall and the timer fires before the syscall
// work was able to cancel it, then depending on the preemption model this
// will either reschedule on return from interrupt or in the syscall work
// below.
//
#[no_mangle]
unsafe extern "C" fn rseq_slice_expired(tmr: *mut hrtimer) -> enum hrtimer_restart {
    let mut st = container_of!(tmr, slice_timer, timer);
//
// Validate that the task which armed the timer is still on the
// CPU. It could have been scheduled out without canceling the
// timer.
//
    if (st.cookie == current && current.rseq.slice.state.granted) {
    rseq_stat_inc(rseq_stats.s_expired);
    set_need_resched_current();
    }
    return HRTIMER_NORESTART;
    }
#[no_mangle]
pub unsafe extern "C" fn __rseq_arm_slice_extension_timer() -> bool {
    let mut st = this_cpu_ptr(&slice_timer);
    let mut curr = current;
    lockdep_assert_irqs_disabled();
//
// This check prevents a task, which got a time slice extension
// granted, from exceeding the maximum scheduling latency when the
// grant expired before going out to user space. Don't bother to
// clear the grant here, it will be cleaned up automatically before
// going out to user space after being scheduled back in.
//
    if ((unlikely(curr.rseq.slice.expires < ktime_get_mono_fast_ns()))) {
    set_need_resched_current();
    return true;
    }
//
// Store the task pointer as a cookie for comparison in the timer
// function. This is safe as the timer is CPU local and cannot be
// in the expiry function at this point.
//
    st.cookie = curr;
    hrtimer_start(&st.timer, curr.rseq.slice.expires, HRTIMER_MODE_ABS_PINNED_HARD);
// Arm the syscall entry work
    set_task_syscall_work(curr, SYSCALL_RSEQ_SLICE);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn rseq_cancel_slice_extension_timer() {
    let mut st = this_cpu_ptr(&slice_timer);
//
// st->cookie can be safely read as preemption is disabled and the
// timer is CPU local.
//
// As this is most probably the first expiring timer, the cancel is
// expensive as it has to reprogram the hardware, but that's less
// expensive than going through a full hrtimer_interrupt() cycle
// for nothing.
//
// hrtimer_try_to_cancel() is sufficient here as the timer is CPU
// local and once the hrtimer code disabled interrupts the timer
// callback cannot be running.
//
    if (st.cookie == current) {
    hrtimer_try_to_cancel(&st.timer);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rseq_slice_set_need_resched(curr: *mut task_struct) {
//
// The interrupt guard is required to prevent inconsistent state in
// this case:
//
// set_tsk_need_resched()
// --> Interrupt
// wakeup()
// set_tsk_need_resched()
// set_preempt_need_resched()
// schedule_on_return()
// clear_tsk_need_resched()
// clear_preempt_need_resched()
// set_preempt_need_resched()		<- Inconsistent state
//
// This is safe vs. a remote set of TIF_NEED_RESCHED because that
// only sets the already set bit and does not create inconsistent
// state.
//
    scoped_guard(irq)
    set_need_resched_current();
    }
#[no_mangle]
unsafe extern "C" fn rseq_slice_validate_ctrl(expected: u32) {
    let mut sctrl = &current.rseq.usrptr.slice_ctrl.all;
    let mut uval = 0;
    if (get_user(uval, sctrl) || uval != expected) {
    force_sig(SIGSEGV);
    }
    }
//
// Invoked from syscall entry if a time slice extension was granted and the
// kernel did not clear it before user space left the critical section.
//
// While the recommended way to relinquish the CPU side effect free is
// rseq_slice_yield(2), any syscall within a granted slice terminates the
// grant and immediately reschedules if required. This supports onion layer
// applications, where the code requesting the grant cannot control the
// code within the critical section.
//
#[no_mangle]
pub unsafe extern "C" fn rseq_syscall_enter_work(syscall: c_long) {
    let mut curr = current;
pub static mut ctrl: rseq_slice_ctrl = 0;
    clear_task_syscall_work(curr, SYSCALL_RSEQ_SLICE);
    if (static_branch_unlikely(&rseq_debug_enabled)) {
    rseq_slice_validate_ctrl(ctrl.all);
    }
//
// The kernel might have raced, revoked the grant and updated
// userspace, but kept the SLICE work set.
//
    if (!ctrl.granted) {
    return;
    }
//
// Required to stabilize the per CPU timer pointer and to make
// set_tsk_need_resched() correct on PREEMPT[RT] kernels.
//
// Leaving the scope will reschedule on preemption models FULL,
// LAZY and RT if necessary.
//
    scoped_guard(preempt) {
    rseq_cancel_slice_extension_timer();
//
// Now that preemption is disabled, quickly check whether
// the task was already rescheduled before arriving here.
//
    if (!curr.rseq.event.sched_switch) {
    rseq_slice_set_need_resched(curr);
    if (syscall == __NR_rseq_slice_yield) {
    rseq_stat_inc(rseq_stats.s_yielded);
// Update the yielded state for syscall return
    curr.rseq.slice.yielded = 1;
    } else {
    rseq_stat_inc(rseq_stats.s_aborted);
    }
    }
    }
// Reschedule on NONE/VOLUNTARY preemption models
    cond_resched();
// Clear the grant in kernel state and user space
    curr.rseq.slice.state.granted = false;
    if (put_user(0U, &curr.rseq.usrptr.slice_ctrl.all)) {
    force_sig(SIGSEGV);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rseq_slice_extension_prctl(arg2: c_ulong, arg3: c_ulong) -> c_int {
    match (arg2) {
    PR_RSEQ_SLICE_EXTENSION_GET => {
    if (arg3) {
    return -EINVAL;
    }
    return current.rseq.slice.state.enabled ? PR_RSEQ_SLICE_EXT_ENABLE : 0;
    }
    PR_RSEQ_SLICE_EXTENSION_SET => {
    u32 rflags, valid = RSEQ_CS_FLAG_SLICE_EXT_AVAILABLE;
pub static mut enable: bool = false;
    if (arg3 & ~PR_RSEQ_SLICE_EXT_ENABLE) {
    return -EINVAL;
    }
    if (!rseq_slice_extension_enabled()) {
    return -ENOTSUPP;
    }
    if (!current.rseq.usrptr) {
    return -ENXIO;
    }
    if (!rseq_v2(current)) {
    return -ENOTSUPP;
    }
// No change?
    if (enable == !!current.rseq.slice.state.enabled) {
    return 0;
    }
    if (get_user(rflags, &current.rseq.usrptr.flags)) {
// goto;
    }
    if (current.rseq.slice.state.enabled) {
    valid |= RSEQ_CS_FLAG_SLICE_EXT_ENABLED;
    }
    if ((rflags & valid) != valid) {
// goto;
    }
    rflags &= ~RSEQ_CS_FLAG_SLICE_EXT_ENABLED;
    rflags |= RSEQ_CS_FLAG_SLICE_EXT_AVAILABLE;
    if (enable) {
    rflags |= RSEQ_CS_FLAG_SLICE_EXT_ENABLED;
    }
    if (put_user(rflags, &current.rseq.usrptr.flags)) {
// goto;
    }
    current.rseq.slice.state.enabled = enable;
    return 0;
    }
    }
// label;
    return -EINVAL;
    }
// label;
    force_sig(SIGSEGV);
    return -EFAULT;
    }
//
// sys_rseq_slice_yield - yield the current processor side effect free if a
// task granted with a time slice extension is done with
// the critical work before being forced out.
//
// Return: 1 if the task successfully yielded the CPU within the granted slice.
// 0 if the slice extension was either never granted or was revoked by
// going over the granted extension, using a syscall other than this one
// or being scheduled out earlier due to a subsequent interrupt.
//
// The syscall does not schedule because the syscall entry work immediately
// relinquishes the CPU and schedules if required.
//
#[no_mangle]
pub unsafe extern "C" fn sys_rseq_slice_yield() -> c_long {
pub static mut yielded: c_int = 0;
    current.rseq.slice.yielded = 0;
    return yielded;
    }
#[no_mangle]
unsafe extern "C" fn rseq_slice_ext_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    seq_printf(m, "%d\n", rseq_slice_ext_nsecs);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rseq_slice_ext_write(file: *mut file, ubuf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut nsecs = 0;
    if (kstrtouint_from_user(ubuf, count, 10, &nsecs)) {
    return -EINVAL;
    }
    if (nsecs < rseq_slice_ext_nsecs_min) {
    return -ERANGE;
    }
    if (nsecs > rseq_slice_ext_nsecs_max) {
    return -ERANGE;
    }
    rseq_slice_ext_nsecs = nsecs;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn rseq_slice_ext_open(inode: *mut inode, file: *mut file) -> c_int {
    return single_open(file, rseq_slice_ext_show, inode.i_private);
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn rseq_slice_ext_init(root_dir: *mut dentry) {
    debugfs_create_file("slice_ext_nsec", 0644, root_dir, core::ptr::null_mut(), &slice_ext_ops);
    }
#[no_mangle]
unsafe extern "C" fn rseq_slice_cmdline(str: *mut c_char) -> c_int {
    let mut on = 0;
    if (kstrtobool(str, &on)) {
    return 0;
    }
    if (!on) {
    static_branch_disable(&rseq_slice_extension_key);
    }
    return 1;
    }
    __setup!("rseq_slice_ext=", rseq_slice_cmdline);
#[no_mangle]
unsafe extern "C" fn rseq_slice_init() -> c_int {
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    hrtimer_setup(per_cpu_ptr(&slice_timer.timer, cpu), rseq_slice_expired,
    CLOCK_MONOTONIC, HRTIMER_MODE_REL_PINNED_HARD);
    }
    return 0;
    }
    device_initcall!(rseq_slice_init);

#[no_mangle]
pub unsafe extern "C" fn rseq_slice_ext_init(root_dir: *mut dentry) {
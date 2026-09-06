//! Automatically rewritten from C to Rust
//! Source: kernel/sched/membarrier.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2010-2017 Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
//
// membarrier system call
//

//
// For documentation purposes, here are some membarrier ordering
// scenarios to keep in mind:
//
// A) Userspace thread execution after IPI vs membarrier's memory
// barrier before sending the IPI
//
// Userspace variables:
//
// int x = 0, y = 0;
//
// The memory barrier at the start of membarrier() on CPU0 is necessary in
// order to enforce the guarantee that any writes occurring on CPU0 before
// the membarrier() is executed will be visible to any code executing on
// CPU1 after the IPI-induced memory barrier:
//
// CPU0                              CPU1
//
// x = 1
// membarrier():
// a: smp_mb()
// b: send IPI                       IPI-induced mb
// c: smp_mb()
// r2 = y
// y = 1
// barrier()
// r1 = x
//
// BUG_ON!(r1 == 0 && r2 == 0)
//
// The write to y and load from x by CPU1 are unordered by the hardware,
// so it's possible to have "r1 = x" reordered before "y = 1" at any
// point after (b).  If the memory barrier at (a) is omitted, then "x = 1"
// can be reordered after (a) (although not after (c)), so we get r1 == 0
// and r2 == 0.  This violates the guarantee that membarrier() is
// supposed by provide.
//
// The timing of the memory barrier at (a) has to ensure that it executes
// before the IPI-induced memory barrier on CPU1.
//
// B) Userspace thread execution before IPI vs membarrier's memory
// barrier after completing the IPI
//
// Userspace variables:
//
// int x = 0, y = 0;
//
// The memory barrier at the end of membarrier() on CPU0 is necessary in
// order to enforce the guarantee that any writes occurring on CPU1 before
// the membarrier() is executed will be visible to any code executing on
// CPU0 after the membarrier():
//
// CPU0                              CPU1
//
// x = 1
// barrier()
// y = 1
// r2 = y
// membarrier():
// a: smp_mb()
// b: send IPI                       IPI-induced mb
// c: smp_mb()
// r1 = x
// BUG_ON!(r1 == 0 && r2 == 1)
//
// The writes to x and y are unordered by the hardware, so it's possible to
// have "r2 = 1" even though the write to x doesn't execute until (b).  If
// the memory barrier at (c) is omitted then "r1 = x" can be reordered
// before (b) (although not before (a)), so we get "r1 = 0".  This violates
// the guarantee that membarrier() is supposed to provide.
//
// The timing of the memory barrier at (c) has to ensure that it executes
// after the IPI-induced memory barrier on CPU1.
//
// C) Scheduling userspace thread -> kthread -> userspace thread vs membarrier
//
// CPU0                            CPU1
//
// membarrier():
// a: smp_mb()
// d: switch to kthread (includes mb)
// b: read rq->curr->mm == NULL
// e: switch to user (includes mb)
// c: smp_mb()
//
// Using the scenario from (A), we can show that (a) needs to be paired
// with (e). Using the scenario from (B), we can show that (c) needs to
// be paired with (d).
//
// D) exit_mm vs membarrier
//
// Two thread groups are created, A and B.  Thread group B is created by
// issuing clone from group A with flag CLONE_VM set, but not CLONE_THREAD.
// Let's assume we have a single thread within each thread group (Thread A
// and Thread B).  Thread A runs on CPU0, Thread B runs on CPU1.
//
// CPU0                            CPU1
//
// membarrier():
// a: smp_mb()
// exit_mm():
// d: smp_mb()
// e: current->mm = NULL
// b: read rq->curr->mm == NULL
// c: smp_mb()
//
// Using scenario (B), we can show that (c) needs to be paired with (d).
//
// E) kthread_{use,unuse}_mm vs membarrier
//
// CPU0                            CPU1
//
// membarrier():
// a: smp_mb()
// kthread_unuse_mm()
// d: smp_mb()
// e: current->mm = NULL
// b: read rq->curr->mm == NULL
// kthread_use_mm()
// f: current->mm = mm
// g: smp_mb()
// c: smp_mb()
//
// Using the scenario from (A), we can show that (a) needs to be paired
// with (g). Using the scenario from (B), we can show that (c) needs to
// be paired with (d).
//
// Bitmask made from a "or" of all commands within enum membarrier_cmd,
// except MEMBARRIER_CMD_QUERY.
//

    (MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE			
    | MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE)

pub const MEMBARRIER_PRIVATE_EXPEDITED_SYNC_CORE_BITMASK: c_int = 0;

    (MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ			
    | MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ)

pub const MEMBARRIER_PRIVATE_EXPEDITED_RSEQ_BITMASK: c_int = 0;

    (MEMBARRIER_CMD_GLOBAL | MEMBARRIER_CMD_GLOBAL_EXPEDITED	
    | MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED			
    | MEMBARRIER_CMD_PRIVATE_EXPEDITED				
    | MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED			
    | MEMBARRIER_PRIVATE_EXPEDITED_SYNC_CORE_BITMASK		
    | MEMBARRIER_PRIVATE_EXPEDITED_RSEQ_BITMASK			
    | MEMBARRIER_CMD_GET_REGISTRATIONS)
//
// Scoped guard for memory barriers on entry and exit.
// Matches memory barriers before & after rq->curr modification in scheduler.
//
    DEFINE_LOCK_GUARD_0(mb, smp_mb(), smp_mb())
pub static mut membarrier_ipi_mutex: usize = 0;
pub static mut struct mutex: usize = 0;

#[no_mangle]
unsafe extern "C" fn membarrier_init() -> c_int {
    let mut i = 0;
    for_each_possible_cpu(i) {
    mutex_init(&per_cpu(membarrier_cpu_mutexes, i));
    }
    return 0;
    }
    core_initcall!(membarrier_init);
#[no_mangle]
unsafe extern "C" fn ipi_mb(info: *mut c_void) {
    smp_mb();	/* IPIs should be serializing but paranoid. */
    }
#[no_mangle]
unsafe extern "C" fn ipi_sync_core(info: *mut c_void) {
//
// The smp_mb() in membarrier after all the IPIs is supposed to
// ensure that memory on remote CPUs that occur before the IPI
// become visible to membarrier()'s caller -- see scenario B in
// the big comment at the top of this file.
//
// A sync_core() would provide this guarantee, but
// sync_core_before_usermode() might end up being deferred until
// after membarrier()'s smp_mb().
//
    smp_mb();	/* IPIs should be serializing but paranoid. */
    sync_core_before_usermode();
    }
#[no_mangle]
unsafe extern "C" fn ipi_rseq(info: *mut c_void) {
//
// Ensure that all stores done by the calling thread are visible
// to the current task before the current task resumes.  We could
// probably optimize this away on most architectures, but by the
// time we've already sent an IPI, the cost of the extra smp_mb()
// is negligible.
//
    smp_mb();
//
// Legacy mode requires that IDs are written and the critical section is
// evaluated. V2 optimized mode handles the critical section and IDs are
// only updated if they change as a consequence of preemption after
// return from this IPI.
//
    if (rseq_v2(current)) {
    rseq_sched_switch_event(current);
    }
    else {
    rseq_force_update();
    }
    }
#[no_mangle]
unsafe extern "C" fn ipi_sync_rq_state(info: *mut c_void) {
    let mut mm =  info;
    if (current.mm != mm) {
    return;
    }
    this_cpu_write(runqueues.membarrier_state,
    atomic_read(&mm.membarrier_state));
//
// Issue a memory barrier after setting
// MEMBARRIER_STATE_GLOBAL_EXPEDITED in the current runqueue to
// guarantee that no memory access following registration is reordered
// before registration.
//
    smp_mb();
    }
#[no_mangle]
pub unsafe extern "C" fn membarrier_exec_mmap(mm: *mut mm_struct) {
//
// Issue a memory barrier before clearing membarrier_state to
// guarantee that no memory access prior to exec is reordered after
// clearing this state.
//
    smp_mb();
    atomic_set(&mm.membarrier_state, 0);
//
// Keep the runqueue membarrier_state in sync with this mm
// membarrier_state.
//
    this_cpu_write(runqueues.membarrier_state, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn membarrier_update_current_mm(next_mm: *mut mm_struct) {
    let mut rq = this_rq();
pub static mut membarrier_state: c_int = 0;
    if (next_mm) {
    membarrier_state = atomic_read(&next_mm.membarrier_state);
    }
    if (READ_ONCE(rq.membarrier_state) == membarrier_state) {
    return;
    }
    WRITE_ONCE(rq.membarrier_state, membarrier_state);
    }
#[no_mangle]
unsafe extern "C" fn membarrier_global_expedited() -> c_int {
    cpumask_var_t __free(free_cpumask_var) tmpmask = CPUMASK_VAR_NULL;
    let mut cpu = 0;
    if (num_online_cpus() == 1) {
    return 0;
    }
    if (!zalloc_cpumask_var(&tmpmask, GFP_KERNEL)) {
    return -ENOMEM;
    }
    guard(mb)();
    SERIALIZE_IPI();
    guard(cpus_read_lock)();
    rcu_read_lock();
    for_each_online_cpu(cpu) {
pub static mut p: *mut c_void = core::ptr::null_mut();
//
// Skipping the current CPU is OK even through we can be
// migrated at any point. The current CPU, at the point
// where we read raw_smp_processor_id(), is ensured to
// be in program order with respect to the caller
// thread. Therefore, we can skip this CPU from the
// iteration.
//
    if (cpu == raw_smp_processor_id()) {
    continue;
    }
    if (!(READ_ONCE(cpu_rq(cpu).membarrier_state) &
    MEMBARRIER_STATE_GLOBAL_EXPEDITED)) {
    continue;
    }
//
// Skip the CPU if it runs a kernel thread which is not using
// a task mm.
//
    p = rcu_dereference(cpu_rq(cpu).curr);
    if (!p.mm) {
    continue;
    }
    __cpumask_set_cpu(cpu, tmpmask);
    }
    rcu_read_unlock();
    preempt_disable();
    smp_call_function_many(tmpmask, ipi_mb, core::ptr::null_mut(), 1);
    preempt_enable();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn membarrier_private_expedited(flags: c_int, cpu_id: c_int) -> c_int {
    let mut mm = current.mm;
pub static mut ipi_func: smp_call_func_t = 0;
    if (flags == MEMBARRIER_FLAG_SYNC_CORE) {
    if (!IS_ENABLED!(CONFIG_ARCH_HAS_MEMBARRIER_SYNC_CORE)) {
    return -EINVAL;
    }
    if (!(atomic_read(&mm.membarrier_state) &
    MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE_READY)) {
    return -EPERM;
    }
    ipi_func = ipi_sync_core;
    prepare_sync_core_cmd(mm);
    } else if (flags == MEMBARRIER_FLAG_RSEQ) {
    if (!IS_ENABLED!(CONFIG_RSEQ)) {
    return -EINVAL;
    }
    if (!(atomic_read(&mm.membarrier_state) &
    MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ_READY)) {
    return -EPERM;
    }
    ipi_func = ipi_rseq;
    } else {
    WARN_ON_ONCE!(flags);
    if (!(atomic_read(&mm.membarrier_state) &
    MEMBARRIER_STATE_PRIVATE_EXPEDITED_READY)) {
    return -EPERM;
    }
    }
    if (flags != MEMBARRIER_FLAG_SYNC_CORE &&
    (atomic_read(&mm.mm_users) == 1 || num_online_cpus() == 1)) {
    return 0;
    }
//
// Matches memory barriers after rq->curr modification in
// scheduler.
//
// On RISC-V, this barrier pairing is also needed for the
// SYNC_CORE command when switching between processes, cf.
// the inline comments in membarrier_arch_switch_mm().
//
// Memory barrier on the caller thread _after_ we finished
// waiting for the last IPI. Matches memory barriers before
// rq->curr modification in scheduler.
//
    guard(mb)();
    if (cpu_id >= 0) {
    if (cpu_id >= nr_cpu_ids || !cpu_possible(cpu_id)) {
    return 0;
    }
    SERIALIZE_IPI_CPU(cpu_id);
    guard(cpus_read_lock)();
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (!cpu_online(cpu_id)) {
    return 0;
    }
    rcu_read_lock();
    p = rcu_dereference(cpu_rq(cpu_id).curr);
    if (!p || p.mm != mm) {
    rcu_read_unlock();
    return 0;
    }
    rcu_read_unlock();
//
// smp_call_function_single() will call ipi_func() if cpu_id
// is the calling CPU.
//
    smp_call_function_single(cpu_id, ipi_func, core::ptr::null_mut(), 1);
    } else {
    cpumask_var_t __free(free_cpumask_var) tmpmask = CPUMASK_VAR_NULL;
    let mut cpu = 0;
    if (!zalloc_cpumask_var(&tmpmask, GFP_KERNEL)) {
    return -ENOMEM;
    }
    SERIALIZE_IPI();
    guard(cpus_read_lock)();
    rcu_read_lock();
    for_each_online_cpu(cpu) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = rcu_dereference(cpu_rq(cpu).curr);
    if (p && p.mm == mm) {
    __cpumask_set_cpu(cpu, tmpmask);
    }
    }
    rcu_read_unlock();
//
// For regular membarrier, we can save a few cycles by
// skipping the current cpu -- we're about to do smp_mb()
// below, and if we migrate to a different cpu, this cpu
// and the new cpu will execute a full barrier in the
// scheduler.
//
// For SYNC_CORE, we do need a barrier on the current cpu --
// otherwise, if we are migrated and replaced by a different
// task in the same mm just before, during, or after
// membarrier, we will end up with some thread in the mm
// running without a core sync.
//
// For RSEQ, don't invoke rseq_sched_switch_event() on the
// caller.  User code is not supposed to issue syscalls at
// all from inside an rseq critical section.
//
    if (flags != MEMBARRIER_FLAG_SYNC_CORE) {
    preempt_disable();
    smp_call_function_many(tmpmask, ipi_func, core::ptr::null_mut(), true);
    preempt_enable();
    } else {
    on_each_cpu_mask(tmpmask, ipi_func, core::ptr::null_mut(), true);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sync_runqueues_membarrier_state(mm: *mut mm_struct) -> c_int {
pub static mut membarrier_state: c_int = 0;
    let mut tmpmask;
    let mut cpu = 0;
    if (atomic_read(&mm.mm_users) == 1 || num_online_cpus() == 1) {
    this_cpu_write(runqueues.membarrier_state, membarrier_state);
//
// For single mm user, we can simply issue a memory barrier
// after setting MEMBARRIER_STATE_GLOBAL_EXPEDITED in the
// mm and in the current runqueue to guarantee that no memory
// access following registration is reordered before
// registration.
//
    smp_mb();
    return 0;
    }
    if (!zalloc_cpumask_var(&tmpmask, GFP_KERNEL)) {
    return -ENOMEM;
    }
//
// For mm with multiple users, we need to ensure all future
// scheduler executions will observe @mm's new membarrier
// state.
//
    synchronize_rcu();
//
// For each cpu runqueue, if the task's mm match @mm, ensure that all
// @mm's membarrier state set bits are also set in the runqueue's
// membarrier state. This ensures that a runqueue scheduling
// between threads which are users of @mm has its membarrier state
// updated.
//
    SERIALIZE_IPI();
    cpus_read_lock();
    rcu_read_lock();
    for_each_online_cpu(cpu) {
    let mut rq = cpu_rq(cpu);
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = rcu_dereference(rq.curr);
    if (p && p.mm == mm) {
    __cpumask_set_cpu(cpu, tmpmask);
    }
    }
    rcu_read_unlock();
    on_each_cpu_mask(tmpmask, ipi_sync_rq_state, mm, true);
    free_cpumask_var(tmpmask);
    cpus_read_unlock();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn membarrier_register_global_expedited() -> c_int {
    let mut p = current;
    let mut mm = p.mm;
    let mut ret = 0;
    if (atomic_read(&mm.membarrier_state) &
    MEMBARRIER_STATE_GLOBAL_EXPEDITED_READY) {
    return 0;
    }
    atomic_or(MEMBARRIER_STATE_GLOBAL_EXPEDITED, &mm.membarrier_state);
    ret = sync_runqueues_membarrier_state(mm);
    if (ret) {
    return ret;
    }
    atomic_or(MEMBARRIER_STATE_GLOBAL_EXPEDITED_READY,
    &mm.membarrier_state);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn membarrier_register_private_expedited(flags: c_int) -> c_int {
    let mut p = current;
    let mut mm = p.mm;
    let mut ready_state = MEMBARRIER_STATE_PRIVATE_EXPEDITED_READY,
    set_state = MEMBARRIER_STATE_PRIVATE_EXPEDITED,
    ret;
    if (flags == MEMBARRIER_FLAG_SYNC_CORE) {
    if (!IS_ENABLED!(CONFIG_ARCH_HAS_MEMBARRIER_SYNC_CORE)) {
    return -EINVAL;
    }
    ready_state =
    MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE_READY;
    } else if (flags == MEMBARRIER_FLAG_RSEQ) {
    if (!IS_ENABLED!(CONFIG_RSEQ)) {
    return -EINVAL;
    }
    ready_state =
    MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ_READY;
    } else {
    WARN_ON_ONCE!(flags);
    }
//
// We need to consider threads belonging to different thread
// groups, which use the same mm. (CLONE_VM but not
// CLONE_THREAD).
//
    if ((atomic_read(&mm.membarrier_state) & ready_state) == ready_state) {
    return 0;
    }
    if (flags & MEMBARRIER_FLAG_SYNC_CORE) {
    set_state |= MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE;
    }
    if (flags & MEMBARRIER_FLAG_RSEQ) {
    set_state |= MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ;
    }
    atomic_or(set_state, &mm.membarrier_state);
    ret = sync_runqueues_membarrier_state(mm);
    if (ret) {
    return ret;
    }
    atomic_or(ready_state, &mm.membarrier_state);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn membarrier_get_registrations() -> c_int {
    let mut p = current;
    let mut mm = p.mm;
pub static mut registrations_mask: c_int = 0;
    static const int states[] = {
    MEMBARRIER_STATE_GLOBAL_EXPEDITED |
    MEMBARRIER_STATE_GLOBAL_EXPEDITED_READY,
    MEMBARRIER_STATE_PRIVATE_EXPEDITED |
    MEMBARRIER_STATE_PRIVATE_EXPEDITED_READY,
    MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE |
    MEMBARRIER_STATE_PRIVATE_EXPEDITED_SYNC_CORE_READY,
    MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ |
    MEMBARRIER_STATE_PRIVATE_EXPEDITED_RSEQ_READY
    };
    static const int registration_cmds[] = {
    MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED,
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED,
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE,
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ
    };
    BUILD_BUG_ON!(ARRAY_SIZE!(states) != ARRAY_SIZE!(registration_cmds));
    membarrier_state = atomic_read(&mm.membarrier_state);
    while (i < ARRAY_SIZE!(states)) {
    if (membarrier_state & states[i]) {
    registrations_mask |= registration_cmds[i];
    membarrier_state &= ~states[i];
    }
    }
    WARN_ON_ONCE!(membarrier_state != 0);
    return registrations_mask;
    }
//
// sys_membarrier - issue memory barriers on a set of threads
// @cmd:    Takes command values defined in enum membarrier_cmd.
// @flags:  Currently needs to be 0 for all commands other than
// MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ: in the latter
// case it can be MEMBARRIER_CMD_FLAG_CPU, indicating that @cpu_id
// contains the CPU on which to interrupt (= restart)
// the RSEQ critical section.
// @cpu_id: if @flags == MEMBARRIER_CMD_FLAG_CPU, indicates the cpu on which
// RSEQ CS should be interrupted (@cmd must be
// MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ).
//
// If this system call is not implemented, -ENOSYS is returned. If the
// command specified does not exist, not available on the running
// kernel, or if the command argument is invalid, this system call
// returns -EINVAL. For a given command, with flags argument set to 0,
// if this system call returns -ENOSYS or -EINVAL, it is guaranteed to
// always return the same value until reboot. In addition, it can return
// -ENOMEM if there is not enough memory available to perform the system
// call.
//
// All memory accesses performed in program order from each targeted thread
// is guaranteed to be ordered with respect to sys_membarrier(). If we use
// the semantic "barrier()" to represent a compiler barrier forcing memory
// accesses to be performed in program order across the barrier, and
// smp_mb() to represent explicit memory barriers forcing full memory
// ordering across the barrier, we have the following ordering table for
// each pair of barrier(), sys_membarrier() and smp_mb():
//
// The pair ordering is detailed as (O: ordered, X: not ordered):
//
// barrier()   smp_mb() sys_membarrier()
// barrier()          X           X            O
// smp_mb()           X           O            O
// sys_membarrier()   O           O            O
//
#[no_mangle]
pub unsafe extern "C" fn sys_membarrier(cmd: usize, flags: usize, cpu_id: usize) -> c_long {
    match (cmd) {
    MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ => {
    if (unlikely(flags && flags != MEMBARRIER_CMD_FLAG_CPU)) {
    return -EINVAL;
    }
    // break;
    }
    _ => {
    if (unlikely(flags)) {
    return -EINVAL;
    }
    }
    }
    if (!(flags & MEMBARRIER_CMD_FLAG_CPU)) {
    cpu_id = -1;
    }
    match (cmd) {
    MEMBARRIER_CMD_QUERY => {
    {
pub static mut cmd_mask: c_int = 0;
    if (tick_nohz_full_enabled()) {
    cmd_mask &= ~MEMBARRIER_CMD_GLOBAL;
    }
    return cmd_mask;
    }
    }
    MEMBARRIER_CMD_GLOBAL => {
// MEMBARRIER_CMD_GLOBAL is not compatible with nohz_full.
    if (tick_nohz_full_enabled()) {
    return -EINVAL;
    }
    if (num_online_cpus() > 1) {
    synchronize_rcu();
    }
    return 0;
    }
    MEMBARRIER_CMD_GLOBAL_EXPEDITED => {
    return membarrier_global_expedited();
    }
    MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED => {
    return membarrier_register_global_expedited();
    }
    MEMBARRIER_CMD_PRIVATE_EXPEDITED => {
    return membarrier_private_expedited(0, cpu_id);
    }
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED => {
    return membarrier_register_private_expedited(0);
    }
    MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE => {
    return membarrier_private_expedited(MEMBARRIER_FLAG_SYNC_CORE, cpu_id);
    }
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE => {
    return membarrier_register_private_expedited(MEMBARRIER_FLAG_SYNC_CORE);
    }
    MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ => {
    return membarrier_private_expedited(MEMBARRIER_FLAG_RSEQ, cpu_id);
    }
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ => {
    return membarrier_register_private_expedited(MEMBARRIER_FLAG_RSEQ);
    }
    MEMBARRIER_CMD_GET_REGISTRATIONS => {
    return membarrier_get_registrations();
    }
    _ => {
    return -EINVAL;
    }
    }
    }
//! Automatically rewritten from C to Rust
//! Source: kernel/sched/cputime.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Simple CPU accounting cgroup controller
//

pub static mut sched_clock_irqtime: usize = 0;
//
// There are no locks covering percpu hardirq/softirq time.
// They are only modified in vtime_account, on corresponding CPU
// with interrupts disabled. So, writes are safe.
// They are read and saved off onto struct rq in update_rq_clock().
// This may result in other CPU reading this CPU's IRQ time and can
// race with irq/vtime_account on this CPU. We would either get old
// or new value with a side effect of accounting a slice of IRQ time to wrong
// task when IRQ is in progress while we read rq->clock. That is a worthy
// compromise in place of having locks on each IRQ in account_system_time.
//
pub static mut struct irqtime: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn enable_sched_clock_irqtime() {
    static_branch_enable(&sched_clock_irqtime);
    }
#[no_mangle]
pub unsafe extern "C" fn disable_sched_clock_irqtime() {
    if (irqtime_enabled()) {
    static_branch_disable(&sched_clock_irqtime);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn irqtime_account_delta(irqtime: *mut irqtime, delta: u64, idx: cpu_usage_stat) {
    let mut cpustat = kcpustat_this_cpu.cpustat;
// forward_decl: _stats_update_begin;
    cpustat[idx] += delta;
    irqtime.total += delta;
    if (!kcpustat_idle_dyntick()) {
    irqtime.tick_delta += delta;
    }
// forward_decl: _stats_update_end;
    }
//
// Called after incrementing preempt_count on {soft,}irq_enter
// and before decrementing preempt_count on {soft,}irq_exit.
//
#[no_mangle]
pub unsafe extern "C" fn irqtime_account_irq(curr: *mut task_struct, offset: c_uint) {
    let mut irqtime = this_cpu_ptr(&cpu_irqtime);
    let mut pc = 0;
    let mut delta = 0;
    let mut cpu = 0;
    if (!irqtime_enabled()) {
    return;
    }
    cpu = smp_processor_id();
    delta = sched_clock_cpu(cpu) - irqtime.irq_start_time;
    irqtime.irq_start_time += delta;
    pc = irq_count() - offset;
//
// We do not account for softirq time from ksoftirqd here.
// We want to continue accounting softirq time to ksoftirqd thread
// in that case, so as not to confuse scheduler with a special task
// that do not consume any time, but still wants to run.
//
    if (pc & HARDIRQ_MASK) {
    irqtime_account_delta(irqtime, delta, CPUTIME_IRQ);
    }

    else if ((pc & SOFTIRQ_OFFSET) && curr != this_cpu_ksoftirqd()) {
    irqtime_account_delta(irqtime, delta, CPUTIME_SOFTIRQ);
    }
    }
#[no_mangle]
unsafe extern "C" fn irqtime_tick_accounted(maxtime: u64) -> u64 {
    let mut irqtime = this_cpu_ptr(&cpu_irqtime);
    let mut delta = 0;
    delta = min(irqtime.tick_delta, maxtime);
    irqtime.tick_delta -= delta;
    return delta;
    }

#[no_mangle]
unsafe extern "C" fn irqtime_tick_accounted(dummy: u64) -> u64 {
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn task_group_account_field(p: *mut task_struct, index: c_int, tmp: u64) {
//
// Since all updates are sure to touch the root cgroup, we
// get ourselves ahead and touch it first. If the root cgroup
// is the only cgroup, then nothing else should be necessary.
//
    __this_cpu_add(kernel_cpustat.cpustat[index], tmp);
    cgroup_account_cputime_field(p, index, tmp);
    }
//
// Account user CPU time to a process.
// @p: the process that the CPU time gets accounted to
// @cputime: the CPU time spent in user space since the last update
//
#[no_mangle]
pub unsafe extern "C" fn account_user_time(p: *mut task_struct, cputime: u64) {
    let mut index = 0;
// Add user time to process.
    p.utime += cputime;
    account_group_user_time(p, cputime);
    index = (task_nice(p) > 0) ? CPUTIME_NICE : CPUTIME_USER;
// Add user time to cpustat.
    task_group_account_field(p, index, cputime);
// Account for user time used
    acct_account_cputime(p);
    }
//
// Account guest CPU time to a process.
// @p: the process that the CPU time gets accounted to
// @cputime: the CPU time spent in virtual machine since the last update
//
#[no_mangle]
pub unsafe extern "C" fn account_guest_time(p: *mut task_struct, cputime: u64) {
    let mut cpustat = kcpustat_this_cpu.cpustat;
// Add guest time to process.
    p.utime += cputime;
    account_group_user_time(p, cputime);
    p.gtime += cputime;
// Add guest time to cpustat.
    if (task_nice(p) > 0) {
    task_group_account_field(p, CPUTIME_NICE, cputime);
    cpustat[CPUTIME_GUEST_NICE] += cputime;
    } else {
    task_group_account_field(p, CPUTIME_USER, cputime);
    cpustat[CPUTIME_GUEST] += cputime;
    }
    }
//
// Account system CPU time to a process and desired cpustat field
// @p: the process that the CPU time gets accounted to
// @cputime: the CPU time spent in kernel space since the last update
// @index: pointer to cpustat field that has to be updated
//
#[no_mangle]
pub unsafe extern "C" fn account_system_index_time(p: *mut task_struct, cputime: u64, index: cpu_usage_stat) {
// Add system time to process.
    p.stime += cputime;
    account_group_system_time(p, cputime);
// Add system time to cpustat.
    task_group_account_field(p, index, cputime);
// Account for system time used
    acct_account_cputime(p);
    }
//
// Account system CPU time to a process.
// @p: the process that the CPU time gets accounted to
// @hardirq_offset: the offset to subtract from hardirq_count()
// @cputime: the CPU time spent in kernel space since the last update
//
#[no_mangle]
pub unsafe extern "C" fn account_system_time(p: *mut task_struct, hardirq_offset: c_int, cputime: u64) {
    let mut index = 0;
    if ((p.flags & PF_VCPU) && (irq_count() - hardirq_offset == 0)) {
    account_guest_time(p, cputime);
    return;
    }
    if (hardirq_count() - hardirq_offset) {
    index = CPUTIME_IRQ;
    }

    else if (in_serving_softirq()) {
    index = CPUTIME_SOFTIRQ;
    }
    else {
    index = CPUTIME_SYSTEM;
    }
    account_system_index_time(p, cputime, index);
    }
//
// Account for involuntary wait time.
// @cputime: the CPU time spent in involuntary wait
//
#[no_mangle]
pub unsafe extern "C" fn account_steal_time(cputime: u64) {
    let mut cpustat = kcpustat_this_cpu.cpustat;
    cpustat[CPUTIME_STEAL] += cputime;
    }
//
// Account for idle time.
// @cputime: the CPU time spent in idle wait
//
#[no_mangle]
pub unsafe extern "C" fn account_idle_time(cputime: u64) {
    let mut cpustat = kcpustat_this_cpu.cpustat;
    let mut rq = this_rq();
    if (atomic_read(&rq.nr_iowait) > 0) {
    cpustat[CPUTIME_IOWAIT] += cputime;
    }
    else {
    cpustat[CPUTIME_IDLE] += cputime;
    }
    }

//
// Account for forceidle time due to core scheduling.
//
// REQUIRES: schedstat is enabled.
//
#[no_mangle]
pub unsafe extern "C" fn __account_forceidle_time(p: *mut task_struct, delta: u64) {
    __schedstat_add(p.stats.core_forceidle_sum, delta);
    task_group_account_field(p, CPUTIME_FORCEIDLE, delta);
    }

//
// When a guest is interrupted for a longer amount of time, missed clock
// ticks are not redelivered later. Due to that, this function may on
// occasion account more time than the calling functions think elapsed.
//

pub static mut paravirt_steal_enabled: usize = 0;

#[no_mangle]
unsafe extern "C" fn native_steal_clock(cpu: c_int) -> u64 {
    return 0;
    }
pub static mut pv_steal_clock: usize = 0;

#[no_mangle]
unsafe extern "C" fn steal_account_process_time(maxtime: u64) -> __always_inline u64 {

    if (static_key_false(&paravirt_steal_enabled)) {
    let mut steal = 0;
    steal = paravirt_steal_clock(smp_processor_id());
    steal -= this_rq().prev_steal_time;
    steal = min(steal, maxtime);
    account_steal_time(steal);
    this_rq().prev_steal_time += steal;
    return steal;
    }

    return 0;
    }
//
// Account how much elapsed time was spent in steal, IRQ, or softirq time.
//
#[no_mangle]
pub unsafe extern "C" fn account_other_time(max: u64) -> u64 {
    let mut accounted = 0;
    lockdep_assert_irqs_disabled();
    accounted = steal_account_process_time(max);
    if (accounted < max) {
    accounted += irqtime_tick_accounted(max - accounted);
    }
    return accounted;
    }

#[no_mangle]
pub unsafe extern "C" fn read_sum_exec_runtime(t: *mut task_struct) -> u64 {
    return t.se.sum_exec_runtime;
    }

#[no_mangle]
unsafe extern "C" fn read_sum_exec_runtime(t: *mut task_struct) -> u64 {
    let mut ns = 0;
pub static mut rf: usize = 0;
pub static mut rq: *mut c_void = core::ptr::null_mut();
    rq = task_rq_lock(t, &rf);
    ns = t.se.sum_exec_runtime;
    task_rq_unlock(rq, t, &rf);
    return ns;
    }

//
// Accumulate raw cputime values of dead tasks (sig->[us]time) and live
// tasks (sum on group iteration) belonging to @tsk's group.
//
#[no_mangle]
pub unsafe extern "C" fn thread_group_cputime(tsk: *mut task_struct, times: *mut task_cputime) {
    let mut sig = tsk.signal;
pub static mut t: *mut c_void = core::ptr::null_mut();
    u64 utime, stime;
//
// Update current task runtime to account pending time since last
// scheduler action or thread_group_cputime() call. This thread group
// might have other running tasks on different CPUs, but updating
// their runtime can affect syscall performance, so we skip account
// those pending times and rely only on values updated on tick or
// other scheduler action.
//
    if (same_thread_group(current, tsk)) {
    (void) task_sched_runtime(current);
    }
    guard(rcu)();
    scoped_seqlock_read (&sig.stats_lock, ss_lock_irqsave) {
    times.utime = sig.utime;
    times.stime = sig.stime;
    times.sum_exec_runtime = sig.sum_sched_runtime;
    __for_each_thread(sig, t) {
    task_cputime(t, &utime, &stime);
    times.utime += utime;
    times.stime += stime;
    times.sum_exec_runtime += read_sum_exec_runtime(t);
    }
    }
    }

//
// Account a tick to a process and cpustat
// @p: the process that the CPU time gets accounted to
// @user_tick: is the tick from userspace
// @rq: the pointer to rq
//
// Tick demultiplexing follows the order
// - pending hardirq update
// - pending softirq update
// - user_time
// - idle_time
// - system time
// - check for guest_time
// - else account as system_time
//
// Check for hardirq is done both for system and user time as there is
// no timer going off while we are on hardirq and hence we may never get an
// opportunity to update it solely in system time.
// p->stime and friends are only updated on system time and not on IRQ
// softirq as those do not count in task exec_runtime any more.
//
#[no_mangle]
pub unsafe extern "C" fn irqtime_account_process_tick(p: *mut task_struct, user_tick: c_int, ticks: c_int) {
    u64 other, cputime = TICK_NSEC * ticks;
//
// When returning from idle, many ticks can get accounted at
// once, including some ticks of steal, IRQ, and softirq time.
// Subtract those ticks from the amount of time accounted to
// idle, or potentially user or system time. Due to rounding,
// other time can exceed ticks occasionally.
//
    other = account_other_time(ULONG_MAX);
    if (other >= cputime) {
    return;
    }
    cputime -= other;
    if (this_cpu_ksoftirqd() == p) {
//
// ksoftirqd time do not get accounted in cpu_softirq_time.
// So, we have to handle it separately here.
// Also, p->stime needs to be updated for ksoftirqd.
//
    account_system_index_time(p, cputime, CPUTIME_SOFTIRQ);
    } else if (user_tick) {
    account_user_time(p, cputime);
    } else if (p == this_rq().idle) {
    account_idle_time(cputime);
    } else if (p.flags & PF_VCPU) { /* System time or guest time */ {
    account_guest_time(p, cputime);
    }
    } else {
    account_system_index_time(p, cputime, CPUTIME_SYSTEM);
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: irqtime_account_process_tick
pub unsafe extern "C" fn irqtime_account_process_tick_dup(p: *mut task_struct, user_tick: c_int, nr_ticks: c_int) { }

#[no_mangle]
unsafe extern "C" fn kcpustat_idle_stop(kc: *mut kernel_cpustat, now: u64) {
    let mut cpustat = kc.cpustat;
    u64 delta, steal, steal_delta;
    let mut iowait = 0;
    if (!kc.idle_elapse) {
    return;
    }
    iowait = nr_iowait_cpu(smp_processor_id()) > 0;
    delta = now - kc.idle_entrytime;
    steal = steal_account_process_time(delta);
//
// Record the idle time after substracting the steal time from
// previous update sequence. Don't substract the steal time from
// the current update sequence to avoid readers moving backward.
//
    write_seqcount_begin(&kc.idle_sleeptime_seq);
    steal_delta = min_t(u64, kc.idle_stealtime[iowait], delta);
    delta -= steal_delta;
    kc.idle_stealtime[iowait] -= steal_delta;
    if (iowait) {
    cpustat[CPUTIME_IOWAIT] += delta;
    }
    else {
    cpustat[CPUTIME_IDLE] += delta;
    }
    kc.idle_stealtime[iowait] += steal;
    kc.idle_entrytime = now;
    kc.idle_elapse = false;
    write_seqcount_end(&kc.idle_sleeptime_seq);
    }
#[no_mangle]
unsafe extern "C" fn kcpustat_idle_start(kc: *mut kernel_cpustat, now: u64) {
// Irqtime accounting might have been enabled in the middle of the IRQ
    if (kc.idle_elapse) {
    return;
    }
    write_seqcount_begin(&kc.idle_sleeptime_seq);
    kc.idle_entrytime = now;
    kc.idle_elapse = true;
    write_seqcount_end(&kc.idle_sleeptime_seq);
    }
#[no_mangle]
pub unsafe extern "C" fn kcpustat_dyntick_stop(now: u64) {
    let mut kc = kcpustat_this_cpu;
    if (!vtime_generic_enabled_this_cpu()) {
    WARN_ON_ONCE!(!kc.idle_dyntick);
    kcpustat_idle_stop(kc, now);
    kc.idle_dyntick = false;
    vtime_dyntick_stop();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kcpustat_dyntick_start(now: u64) {
    let mut kc = kcpustat_this_cpu;
    if (!vtime_generic_enabled_this_cpu()) {
    vtime_dyntick_start();
    kc.idle_dyntick = true;
    kcpustat_idle_start(kc, now);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kcpustat_irq_enter(now: u64) {
    let mut kc = kcpustat_this_cpu;
    if (!vtime_generic_enabled_this_cpu() &&
    (irqtime_enabled() || vtime_accounting_enabled_this_cpu())) {
    kcpustat_idle_stop(kc, now);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kcpustat_irq_exit(now: u64) {
    let mut kc = kcpustat_this_cpu;
//
// Generic vtime already does its own idle accounting.
// But irqtime accounting or arch vtime which also accounts IRQs
// need to pause nohz accounting. Resume nohz accounting as long
// as the irqtime config is enabled to handle case where irqtime
// accounting got runtime disabled in the middle of an IRQ.
//
    if (!vtime_generic_enabled_this_cpu() &&
    (IS_ENABLED!(CONFIG_IRQ_TIME_ACCOUNTING) || vtime_accounting_enabled_this_cpu())) {
    kcpustat_idle_start(kc, now);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kcpustat_field_dyntick(cpu: c_int, idx: cpu_usage_stat, compute_delta: bool, now: u64) -> u64 {
    let mut kc = &kcpustat_cpu(cpu);
pub static mut iowait: c_int = 0;
    let mut cpustat = kc.cpustat;
    let mut seq = 0;
    let mut idle = 0;
    do {
    seq = read_seqcount_begin(&kc.idle_sleeptime_seq);
    idle = cpustat[idx];
    if (kc.idle_elapse && compute_delta && now > kc.idle_entrytime) {
pub static mut delta: u64 = 0;
    delta -= min_t(u64, kc.idle_stealtime[iowait], delta);
    idle += delta;
    }
    } while (read_seqcount_retry(&kc.idle_sleeptime_seq, seq));
    return idle;
    }
#[no_mangle]
pub unsafe extern "C" fn kcpustat_field_idle(cpu: c_int) -> u64 {
    return kcpustat_field_dyntick(cpu, CPUTIME_IDLE,
    !nr_iowait_cpu(cpu), ktime_get());
    }
    EXPORT_SYMBOL_GPL(kcpustat_field_idle);
#[no_mangle]
pub unsafe extern "C" fn kcpustat_field_iowait(cpu: c_int) -> u64 {
    return kcpustat_field_dyntick(cpu, CPUTIME_IOWAIT,
    nr_iowait_cpu(cpu), ktime_get());
    }
    EXPORT_SYMBOL_GPL(kcpustat_field_iowait);

#[no_mangle]
#[no_mangle]
// duplicate fn: kcpustat_field_dyntick
pub unsafe extern "C" fn kcpustat_field_dyntick_dup(cpu: c_int, idx: cpu_usage_stat, compute_delta: bool, now: ktime_t) -> u64 {
    return kcpustat_cpu(cpu).cpustat[idx];
    }

#[no_mangle]
pub unsafe extern "C" fn get_cpu_sleep_time_us(cpu: c_int, idx: cpu_usage_stat, compute_delta: bool, last_update_time: *mut u64) -> u64 {
pub static mut now: ktime_t = 0;
    let mut res = 0;
    if (vtime_generic_enabled_cpu(cpu)) {
    res = kcpustat_field(idx, cpu);
    }
    else {
    res = kcpustat_field_dyntick(cpu, idx, compute_delta, now);
    }
    do_div(res, NSEC_PER_USEC);
    if (last_update_time) {
// last_update_time = ktime_to_us(now);
    }
    return res;
    }
//
// get_cpu_idle_time_us - get the total idle time of a CPU
// @cpu: CPU number to query
// @last_update_time: variable to store update time in. Do not update
// counters if NULL.
//
// Return the cumulative idle time (since boot) for a given
// CPU, in microseconds. Note that this is partially broken due to
// the counter of iowait tasks that can be remotely updated without
// any synchronization. Therefore it is possible to observe backward
// values within two consecutive reads.
//
// This time is measured via accounting rather than sampling,
// and is as accurate as ktime_get() is.
//
// Return: total idle time of the @cpu
//
#[no_mangle]
pub unsafe extern "C" fn get_cpu_idle_time_us(cpu: c_int, last_update_time: *mut u64) -> u64 {
    return get_cpu_sleep_time_us(cpu, CPUTIME_IDLE,
    !nr_iowait_cpu(cpu), last_update_time);
    }
    EXPORT_SYMBOL_GPL(get_cpu_idle_time_us);
//
// get_cpu_iowait_time_us - get the total iowait time of a CPU
// @cpu: CPU number to query
// @last_update_time: variable to store update time in. Do not update
// counters if NULL.
//
// Return the cumulative iowait time (since boot) for a given
// CPU, in microseconds. Note this is partially broken due to
// the counter of iowait tasks that can be remotely updated without
// any synchronization. Therefore it is possible to observe backward
// values within two consecutive reads.
//
// This time is measured via accounting rather than sampling,
// and is as accurate as ktime_get() is.
//
// Return: total iowait time of @cpu
//
#[no_mangle]
pub unsafe extern "C" fn get_cpu_iowait_time_us(cpu: c_int, last_update_time: *mut u64) -> u64 {
    return get_cpu_sleep_time_us(cpu, CPUTIME_IOWAIT,
    nr_iowait_cpu(cpu), last_update_time);
    }
    EXPORT_SYMBOL_GPL(get_cpu_iowait_time_us);
//
// Use precise platform statistics if available:
//

#[no_mangle]
pub unsafe extern "C" fn vtime_account_irq(tsk: *mut task_struct, offset: c_uint) {
pub static mut pc: c_uint = 0;
    if (pc & HARDIRQ_OFFSET) {
    vtime_account_hardirq(tsk);
    } else if (pc & SOFTIRQ_OFFSET) {
    vtime_account_softirq(tsk);
    } else if (!kcpustat_idle_dyntick()) {
    if (!IS_ENABLED!(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE) &&
    is_idle_task(tsk)) {
    vtime_account_idle(tsk);
    } else {
    vtime_account_kernel(tsk);
    }
    } else {
    vtime_reset();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cputime_adjust(curr: *mut task_cputime, prev: *mut prev_cputime, ut: *mut u64, st: *mut u64) {
// ut = curr->utime;
// st = curr->stime;
    }
#[no_mangle]
pub unsafe extern "C" fn task_cputime_adjusted(p: *mut task_struct, ut: *mut u64, st: *mut u64) {
// ut = p->utime;
// st = p->stime;
    }
    EXPORT_SYMBOL_GPL(task_cputime_adjusted);
#[no_mangle]
pub unsafe extern "C" fn thread_group_cputime_adjusted(p: *mut task_struct, ut: *mut u64, st: *mut u64) {
pub static mut cputime: usize = 0;
    thread_group_cputime(p, &cputime);
// ut = cputime.utime;
// st = cputime.stime;
    }

//
// Account a single tick of CPU time.
// @p: the process that the CPU time gets accounted to
// @user_tick: indicates if the tick is a user or a system tick
//
#[no_mangle]
pub unsafe extern "C" fn account_process_tick(p: *mut task_struct, user_tick: c_int) {
    u64 cputime, steal;
    if (vtime_accounting_enabled_this_cpu()) {
    return;
    }
    if (kcpustat_idle_dyntick()) {
    return;
    }
    if (irqtime_enabled()) {
    irqtime_account_process_tick(p, user_tick, 1);
    return;
    }
    cputime = TICK_NSEC;
    steal = steal_account_process_time(ULONG_MAX);
    if (steal >= cputime) {
    return;
    }
    cputime -= steal;
    if (user_tick) {
    account_user_time(p, cputime);
    }

    else if ((p != this_rq().idle) || (irq_count() != HARDIRQ_OFFSET)) {
    account_system_time(p, HARDIRQ_OFFSET, cputime);
    }
    else {
    account_idle_time(cputime);
    }
    }
//
// Adjust tick based cputime random precision against scheduler runtime
// accounting.
//
// Tick based cputime accounting depend on random scheduling timeslices of a
// task to be interrupted or not by the timer.  Depending on these
// circumstances, the number of these interrupts may be over or
// under-optimistic, matching the real user and system cputime with a variable
// precision.
//
// Fix this by scaling these tick based values against the total runtime
// accounted by the CFS scheduler.
//
// This code provides the following guarantees:
//
// stime + utime == rtime
// stime_i+1 >= stime_i, utime_i+1 >= utime_i
//
// Assuming that rtime_i+1 >= rtime_i.
//
#[no_mangle]
#[no_mangle]
// duplicate fn: cputime_adjust
pub unsafe extern "C" fn cputime_adjust_dup(curr: *mut task_cputime, prev: *mut prev_cputime, ut: *mut u64, st: *mut u64) {
    u64 rtime, stime, utime;
    let mut flags = 0;
// Serialize concurrent callers such that we can honour our guarantees
    raw_spin_lock_irqsave(&prev.lock, flags);
    rtime = curr.sum_exec_runtime;
//
// This is possible under two circumstances:
// - rtime isn't monotonic after all (a bug);
// - we got reordered by the lock.
//
// In both cases this acts as a filter such that the rest of the code
// can assume it is monotonic regardless of anything else.
//
    if (prev.stime + prev.utime >= rtime) {
// goto;
    }
    stime = curr.stime;
    utime = curr.utime;
//
// If either stime or utime are 0, assume all runtime is userspace.
// Once a task gets some ticks, the monotonicity code at 'update:'
// will ensure things converge to the observed ratio.
//
    if (stime == 0) {
    utime = rtime;
// goto;
    }
    if (utime == 0) {
    stime = rtime;
// goto;
    }
    stime = mul_u64_u64_div_u64(stime, rtime, stime + utime);
// label;
//
// Make sure stime doesn't go backwards; this preserves monotonicity
// for utime because rtime is monotonic.
//
// utime_i+1 = rtime_i+1 - stime_i
// = rtime_i+1 - (rtime_i - utime_i)
// = (rtime_i+1 - rtime_i) + utime_i
// >= utime_i
//
    if (stime < prev.stime) {
    stime = prev.stime;
    }
    utime = rtime - stime;
//
// Make sure utime doesn't go backwards; this still preserves
// monotonicity for stime, analogous argument to above.
//
    if (utime < prev.utime) {
    utime = prev.utime;
    stime = rtime - utime;
    }
    prev.stime = stime;
    prev.utime = utime;
// label;
// ut = prev->utime;
// st = prev->stime;
    raw_spin_unlock_irqrestore(&prev.lock, flags);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: task_cputime_adjusted
pub unsafe extern "C" fn task_cputime_adjusted_dup(p: *mut task_struct, ut: *mut u64, st: *mut u64) {
pub static mut task_cputime: usize = 0;
    if (task_cputime(p, &cputime.utime, &cputime.stime)) {
    cputime.sum_exec_runtime = task_sched_runtime(p);
    }
    cputime_adjust(&cputime, &p.prev_cputime, ut, st);
    }
    EXPORT_SYMBOL_GPL(task_cputime_adjusted);
#[no_mangle]
#[no_mangle]
// duplicate fn: thread_group_cputime_adjusted
pub unsafe extern "C" fn thread_group_cputime_adjusted_dup(p: *mut task_struct, ut: *mut u64, st: *mut u64) {
pub static mut cputime: usize = 0;
    thread_group_cputime(p, &cputime);
    cputime_adjust(&cputime, &p.signal.prev_cputime, ut, st);
    }

#[no_mangle]
unsafe extern "C" fn vtime_delta(vtime: *mut vtime) -> u64 {
    unsigned long long clock;
    clock = sched_clock();
    if (clock < vtime.starttime) {
    return 0;
    }
    return clock - vtime.starttime;
    }
#[no_mangle]
unsafe extern "C" fn get_vtime_delta(vtime: *mut vtime) -> u64 {
pub static mut delta: u64 = 0;
    let mut other = 0;
//
// Unlike tick based timing, vtime based timing never has lost
// ticks, and no need for steal time accounting to make up for
// lost ticks. Vtime accounts a rounded version of actual
// elapsed time. Limit account_other_time to prevent rounding
// errors from causing elapsed vtime to go negative.
//
    other = account_other_time(delta);
    WARN_ON_ONCE!(vtime.state == VTIME_INACTIVE);
    vtime.starttime += delta;
    return delta - other;
    }
#[no_mangle]
pub unsafe extern "C" fn vtime_account_system(tsk: *mut task_struct, vtime: *mut vtime) {
    vtime.stime += get_vtime_delta(vtime);
    if (vtime.stime >= TICK_NSEC) {
    account_system_time(tsk, irq_count(), vtime.stime);
    vtime.stime = 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn vtime_account_guest(tsk: *mut task_struct, vtime: *mut vtime) {
    vtime.gtime += get_vtime_delta(vtime);
    if (vtime.gtime >= TICK_NSEC) {
    account_guest_time(tsk, vtime.gtime);
    vtime.gtime = 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __vtime_account_kernel(tsk: *mut task_struct, vtime: *mut vtime) {
// We might have scheduled out from guest path
    if (vtime.state == VTIME_GUEST) {
    vtime_account_guest(tsk, vtime);
    }
    else {
    vtime_account_system(tsk, vtime);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn vtime_account_kernel(tsk: *mut task_struct) {
    let mut vtime = &tsk.vtime;
    if (!vtime_delta(vtime)) {
    return;
    }
    write_seqcount_begin(&vtime.seqcount);
    __vtime_account_kernel(tsk, vtime);
    write_seqcount_end(&vtime.seqcount);
    }
#[no_mangle]
pub unsafe extern "C" fn vtime_user_enter(tsk: *mut task_struct) {
    let mut vtime = &tsk.vtime;
    write_seqcount_begin(&vtime.seqcount);
    vtime_account_system(tsk, vtime);
    vtime.state = VTIME_USER;
    write_seqcount_end(&vtime.seqcount);
    }
#[no_mangle]
pub unsafe extern "C" fn vtime_user_exit(tsk: *mut task_struct) {
    let mut vtime = &tsk.vtime;
    write_seqcount_begin(&vtime.seqcount);
    vtime.utime += get_vtime_delta(vtime);
    if (vtime.utime >= TICK_NSEC) {
    account_user_time(tsk, vtime.utime);
    vtime.utime = 0;
    }
    vtime.state = VTIME_SYS;
    write_seqcount_end(&vtime.seqcount);
    }
#[no_mangle]
pub unsafe extern "C" fn vtime_guest_enter(tsk: *mut task_struct) {
    let mut vtime = &tsk.vtime;
//
// The flags must be updated under the lock with
// the vtime_starttime flush and update.
// That enforces a right ordering and update sequence
// synchronization against the reader (task_gtime())
// that can thus safely catch up with a tickless delta.
//
    write_seqcount_begin(&vtime.seqcount);
    vtime_account_system(tsk, vtime);
    tsk.flags |= PF_VCPU;
    vtime.state = VTIME_GUEST;
    write_seqcount_end(&vtime.seqcount);
    }
    EXPORT_SYMBOL_GPL(vtime_guest_enter);
#[no_mangle]
pub unsafe extern "C" fn vtime_guest_exit(tsk: *mut task_struct) {
    let mut vtime = &tsk.vtime;
    write_seqcount_begin(&vtime.seqcount);
    vtime_account_guest(tsk, vtime);
    tsk.flags &= ~PF_VCPU;
    vtime.state = VTIME_SYS;
    write_seqcount_end(&vtime.seqcount);
    }
    EXPORT_SYMBOL_GPL(vtime_guest_exit);
#[no_mangle]
unsafe extern "C" fn __vtime_account_idle(vtime: *mut vtime) {
    account_idle_time(get_vtime_delta(vtime));
    }
#[no_mangle]
pub unsafe extern "C" fn vtime_task_switch_generic(prev: *mut task_struct) {
    let mut vtime = &prev.vtime;
    write_seqcount_begin(&vtime.seqcount);
    if (vtime.state == VTIME_IDLE) {
    __vtime_account_idle(vtime);
    }
    else {
    __vtime_account_kernel(prev, vtime);
    }
    vtime.state = VTIME_INACTIVE;
    vtime.cpu = -1;
    write_seqcount_end(&vtime.seqcount);
    vtime = &current.vtime;
    write_seqcount_begin(&vtime.seqcount);
    if (is_idle_task(current)) {
    vtime.state = VTIME_IDLE;
    }

    else if (current.flags & PF_VCPU) {
    vtime.state = VTIME_GUEST;
    }
    else {
    vtime.state = VTIME_SYS;
    }
    vtime.starttime = sched_clock();
    vtime.cpu = smp_processor_id();
    write_seqcount_end(&vtime.seqcount);
    }
#[no_mangle]
pub unsafe extern "C" fn vtime_init_idle(t: *mut task_struct, cpu: c_int) {
    let mut vtime = &t.vtime;
    let mut flags = 0;
    local_irq_save(flags);
    write_seqcount_begin(&vtime.seqcount);
    vtime.state = VTIME_IDLE;
    vtime.starttime = sched_clock();
    vtime.cpu = cpu;
    write_seqcount_end(&vtime.seqcount);
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn task_gtime(t: *mut task_struct) -> u64 {
    let mut vtime = &t.vtime;
    let mut seq = 0;
    let mut gtime = 0;
    if (!vtime_accounting_enabled()) {
    return t.gtime;
    }
    do {
    seq = read_seqcount_begin(&vtime.seqcount);
    gtime = t.gtime;
    if (vtime.state == VTIME_GUEST) {
    gtime += vtime.gtime + vtime_delta(vtime);
    }
    } while (read_seqcount_retry(&vtime.seqcount, seq));
    return gtime;
    }
//
// Fetch cputime raw values from fields of task_struct and
// add up the pending nohz execution time since the last
// cputime snapshot.
//
#[no_mangle]
pub unsafe extern "C" fn task_cputime(t: *mut task_struct, utime: *mut u64, stime: *mut u64) -> bool {
    let mut vtime = &t.vtime;
    let mut seq = 0;
    let mut delta = 0;
    let mut ret = 0;
    if (!vtime_accounting_enabled()) {
// utime = t->utime;
// stime = t->stime;
    return false;
    }
    do {
    ret = false;
    seq = read_seqcount_begin(&vtime.seqcount);
// utime = t->utime;
// stime = t->stime;
// Task is sleeping or idle, nothing to add
    if (vtime.state < VTIME_SYS) {
    continue;
    }
    ret = true;
    delta = vtime_delta(vtime);
//
// Task runs either in user (including guest) or kernel space,
// add pending nohz time to the right place.
//
    if (vtime.state == VTIME_SYS) {
// stime += vtime->stime + delta;
    }
    else {
// utime += vtime->utime + delta;
    }
    } while (read_seqcount_retry(&vtime.seqcount, seq));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vtime_state_fetch(vtime: *mut vtime, cpu: c_int) -> c_int {
pub static mut state: c_int = 0;
//
// We raced against a context switch, fetch the
// kcpustat task again.
//
    if (vtime.cpu != cpu && vtime.cpu != -1) {
    return -EAGAIN;
    }
//
// Two possible things here:
// 1) We are seeing the scheduling out task (prev) or any past one.
// 2) We are seeing the scheduling in task (next) but it hasn't
// passed though vtime_task_switch() yet so the pending
// cputime of the prev task may not be flushed yet.
//
// Case 1) is ok but 2) is not. So wait for a safe VTIME state.
//
    if (state == VTIME_INACTIVE) {
    return -EAGAIN;
    }
    return state;
    }
#[no_mangle]
unsafe extern "C" fn kcpustat_user_vtime(vtime: *mut vtime) -> u64 {
    if (vtime.state == VTIME_USER) {
    return vtime.utime + vtime_delta(vtime);
    }

    else if (vtime.state == VTIME_GUEST) {
    return vtime.gtime + vtime_delta(vtime);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kcpustat_field_vtime(cpustat: *mut u64, tsk: *mut task_struct, usage: cpu_usage_stat, cpu: c_int, val: *mut u64) -> c_int {
    let mut vtime = &tsk.vtime;
    let mut rq = cpu_rq(cpu);
    let mut seq = 0;
    do {
    let mut state = 0;
    seq = read_seqcount_begin(&vtime.seqcount);
    state = vtime_state_fetch(vtime, cpu);
    if (state < 0) {
    return state;
    }
// val = cpustat[usage];
//
// Nice VS unnice cputime accounting may be inaccurate if
// the nice value has changed since the last vtime update.
// But proper fix would involve interrupting target on nice
// updates which is a no go on nohz_full (although the scheduler
// may still interrupt the target if rescheduling is needed...)
//
    match (usage) {
    CPUTIME_SYSTEM => {
    if (state == VTIME_SYS) {
// val += vtime->stime + vtime_delta(vtime);
    }
    // break;
    }
    CPUTIME_USER => {
    if (task_nice(tsk) <= 0) {
// val += kcpustat_user_vtime(vtime);
    }
    // break;
    }
    CPUTIME_NICE => {
    if (task_nice(tsk) > 0) {
// val += kcpustat_user_vtime(vtime);
    }
    // break;
    }
    CPUTIME_GUEST => {
    if (state == VTIME_GUEST && task_nice(tsk) <= 0) {
// val += vtime->gtime + vtime_delta(vtime);
    }
    // break;
    }
    CPUTIME_GUEST_NICE => {
    if (state == VTIME_GUEST && task_nice(tsk) > 0) {
// val += vtime->gtime + vtime_delta(vtime);
    }
    // break;
    }
    CPUTIME_IDLE => {
    if (state == VTIME_IDLE && !atomic_read(&rq.nr_iowait)) {
// val += vtime_delta(vtime);
    }
    // break;
    }
    CPUTIME_IOWAIT => {
    if (state == VTIME_IDLE && atomic_read(&rq.nr_iowait) > 0) {
// val += vtime_delta(vtime);
    }
    // break;
    }
    _ => {
    // break;
    }
    }
    } while (read_seqcount_retry(&vtime.seqcount, seq));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kcpustat_field(usage: cpu_usage_stat, cpu: c_int) -> u64 {
    let mut cpustat = kcpustat_cpu(cpu).cpustat;
pub static mut val: u64 = 0;
pub static mut rq: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (!vtime_generic_enabled_cpu(cpu)) {
    return kcpustat_field_default(usage, cpu);
    }
    rq = cpu_rq(cpu);
    for (;;) {
pub static mut curr: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    curr = rcu_dereference(rq.curr);
    if (WARN_ON_ONCE!(!curr)) {
    rcu_read_unlock();
    return cpustat[usage];
    }
    err = kcpustat_field_vtime(cpustat, curr, usage, cpu, &val);
    rcu_read_unlock();
    if (!err) {
    return val;
    }
    cpu_relax();
    }
    }
    EXPORT_SYMBOL_GPL(kcpustat_field);
#[no_mangle]
pub unsafe extern "C" fn kcpustat_cpu_fetch_vtime(dst: *mut kernel_cpustat, src: *mut kernel_cpustat, tsk: *mut task_struct, cpu: c_int) -> c_int {
    let mut vtime = &tsk.vtime;
    let mut seq = 0;
    do {
pub static mut cpustat: *mut c_void = core::ptr::null_mut();
    let mut delta = 0;
    let mut state = 0;
    seq = read_seqcount_begin(&vtime.seqcount);
    state = vtime_state_fetch(vtime, cpu);
    if (state < 0) {
    return state;
    }
// dst = *src;
    cpustat = dst.cpustat;
// Task is sleeping or dead, nothing to add
    if (state < VTIME_IDLE) {
    continue;
    }
    delta = vtime_delta(vtime);
//
// Task runs either in user (including guest) or kernel space,
// add pending nohz time to the right place.
//
    match (state) {
    VTIME_SYS => {
    cpustat[CPUTIME_SYSTEM] += vtime.stime + delta;
    // break;
    }
    VTIME_USER => {
    if (task_nice(tsk) > 0) {
    cpustat[CPUTIME_NICE] += vtime.utime + delta;
    }
    else {
    cpustat[CPUTIME_USER] += vtime.utime + delta;
    }
    // break;
    }
    VTIME_GUEST => {
    if (task_nice(tsk) > 0) {
    cpustat[CPUTIME_GUEST_NICE] += vtime.gtime + delta;
    cpustat[CPUTIME_NICE] += vtime.gtime + delta;
    } else {
    cpustat[CPUTIME_GUEST] += vtime.gtime + delta;
    cpustat[CPUTIME_USER] += vtime.gtime + delta;
    }
    // break;
    }
    VTIME_IDLE => {
    if (atomic_read(&cpu_rq(cpu).nr_iowait) > 0) {
    cpustat[CPUTIME_IOWAIT] += delta;
    }
    else {
    cpustat[CPUTIME_IDLE] += delta;
    }
    // break;
    }
    _ => {
    WARN_ON_ONCE!(1);
    }
    }
    } while (read_seqcount_retry(&vtime.seqcount, seq));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kcpustat_cpu_fetch(dst: *mut kernel_cpustat, cpu: c_int) {
    let mut src = &kcpustat_cpu(cpu);
pub static mut rq: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (!vtime_generic_enabled_cpu(cpu)) {
    kcpustat_cpu_fetch_default(dst, cpu);
    return;
    }
    rq = cpu_rq(cpu);
    for (;;) {
pub static mut curr: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    curr = rcu_dereference(rq.curr);
    if (WARN_ON_ONCE!(!curr)) {
    rcu_read_unlock();
    kcpustat_cpu_fetch_default(dst, cpu);
    return;
    }
    err = kcpustat_cpu_fetch_vtime(dst, src, curr, cpu);
    rcu_read_unlock();
    if (!err) {
    return;
    }
    cpu_relax();
    }
    }
    EXPORT_SYMBOL_GPL(kcpustat_cpu_fetch);
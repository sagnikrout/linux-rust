//! Automatically rewritten from C to Rust
//! Source: kernel/sched/psi.c
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
// Pressure stall information for CPU, memory and IO
//
// Copyright (c) 2018 Facebook, Inc.
// Author: Johannes Weiner <hannes@cmpxchg.org>
//
// Polling support by Suren Baghdasaryan <surenb@google.com>
// Copyright (c) 2018 Google, Inc.
//
// When CPU, memory and IO are contended, tasks experience delays that
// reduce throughput and introduce latencies into the workload. Memory
// and IO contention, in addition, can cause a full loss of forward
// progress in which the CPU goes idle.
//
// This code aggregates individual task delays into resource pressure
// metrics that indicate problems with both workload health and
// resource utilization.
//
// Model
//
// The time in which a task can execute on a CPU is our baseline for
// productivity. Pressure expresses the amount of time in which this
// potential cannot be realized due to resource contention.
//
// This concept of productivity has two components: the workload and
// the CPU. To measure the impact of pressure on both, we define two
// contention states for a resource: SOME and FULL.
//
// In the SOME state of a given resource, one or more tasks are
// delayed on that resource. This affects the workload's ability to
// perform work, but the CPU may still be executing other tasks.
//
// In the FULL state of a given resource, all non-idle tasks are
// delayed on that resource such that nobody is advancing and the CPU
// goes idle. This leaves both workload and CPU unproductive.
//
// SOME = nr_delayed_tasks != 0
// FULL = nr_delayed_tasks != 0 && nr_productive_tasks == 0
//
// What it means for a task to be productive is defined differently
// for each resource. For IO, productive means a running task. For
// memory, productive means a running task that isn't a reclaimer. For
// CPU, productive means an on-CPU task.
//
// Naturally, the FULL state doesn't exist for the CPU resource at the
// system level, but exist at the cgroup level. At the cgroup level,
// FULL means all non-idle tasks in the cgroup are delayed on the CPU
// resource which is being used by others outside of the cgroup or
// throttled by the cgroup cpu.max configuration.
//
// The percentage of wall clock time spent in those compound stall
// states gives pressure numbers between 0 and 100 for each resource,
// where the SOME percentage indicates workload slowdowns and the FULL
// percentage indicates reduced CPU utilization:
//
// %SOME = time(SOME) / period
// %FULL = time(FULL) / period
//
// Multiple CPUs
//
// The more tasks and available CPUs there are, the more work can be
// performed concurrently. This means that the potential that can go
// unrealized due to resource contention *also* scales with non-idle
// tasks and CPUs.
//
// Consider a scenario where 257 number crunching tasks are trying to
// run concurrently on 256 CPUs. If we simply aggregated the task
// states, we would have to conclude a CPU SOME pressure number of
// 100%, since *somebody* is waiting on a runqueue at all
// times. However, that is clearly not the amount of contention the
// workload is experiencing: only one out of 256 possible execution
// threads will be contended at any given time, or about 0.4%.
//
// Conversely, consider a scenario of 4 tasks and 4 CPUs where at any
// given time *one* of the tasks is delayed due to a lack of memory.
// Again, looking purely at the task state would yield a memory FULL
// pressure number of 0%, since *somebody* is always making forward
// progress. But again this wouldn't capture the amount of execution
// potential lost, which is 1 out of 4 CPUs, or 25%.
//
// To calculate wasted potential (pressure) with multiple processors,
// we have to base our calculation on the number of non-idle tasks in
// conjunction with the number of available CPUs, which is the number
// of potential execution threads. SOME becomes then the proportion of
// delayed tasks to possible threads, and FULL is the share of possible
// threads that are unproductive due to delays:
//
// threads = min(nr_nonidle_tasks, nr_cpus)
// SOME = min(nr_delayed_tasks / threads, 1)
// FULL = (threads - min(nr_productive_tasks, threads)) / threads
//
// For the 257 number crunchers on 256 CPUs, this yields:
//
// threads = min(257, 256)
// SOME = min(1 / 256, 1)             = 0.4%
// FULL = (256 - min(256, 256)) / 256 = 0%
//
// For the 1 out of 4 memory-delayed tasks, this yields:
//
// threads = min(4, 4)
// SOME = min(1 / 4, 1)               = 25%
// FULL = (4 - min(3, 4)) / 4         = 25%
//
// [ Substitute nr_cpus with 1, and you can see that it's a natural
// extension of the single-CPU model. ]
//
// Implementation
//
// To assess the precise time spent in each such state, we would have
// to freeze the system on task changes and start/stop the state
// clocks accordingly. Obviously that doesn't scale in practice.
//
// Because the scheduler aims to distribute the compute load evenly
// among the available CPUs, we can track task state locally to each
// CPU and, at much lower frequency, extrapolate the global state for
// the cumulative stall times and the running averages.
//
// For each runqueue, we track:
//
// tSOME[cpu] = time(nr_delayed_tasks[cpu] != 0)
// tFULL[cpu] = time(nr_delayed_tasks[cpu] && !nr_productive_tasks[cpu])
// tNONIDLE[cpu] = time(nr_nonidle_tasks[cpu] != 0)
//
// and then periodically aggregate:
//
// tNONIDLE = sum(tNONIDLE[i])
//
// tSOME = sum(tSOME[i] * tNONIDLE[i]) / tNONIDLE
// tFULL = sum(tFULL[i] * tNONIDLE[i]) / tNONIDLE
//
// %SOME = tSOME / period
// %FULL = tFULL / period
//
// This gives us an approximation of pressure that is practical
// cost-wise, yet way more sensitive and accurate than periodic
// sampling of the aggregate task states would be.
//

    static int psi_bug ;
pub static mut psi_disabled: usize = 0;
pub static mut psi_cgroups_enabled: usize = 0;

    static bool psi_enable;

pub static mut psi_enable: bool = true;

#[no_mangle]
unsafe extern "C" fn setup_psi(str: *mut c_char) -> c_int {
    return kstrtobool(str, &psi_enable) == 0;
    }
    __setup!("psi=", setup_psi);
// Running averages - we need to be higher-res than loadavg

// PSI trigger definitions

// Sampling frequency in nanoseconds
    static u64 psi_period ;
// System-level pressure and stall tracking
pub static mut struct psi_group_cpu: usize = 0;
pub static mut psi_group: usize = 0;
    static DEFINE_PER_CPU(seqcount_t, psi_seq) = SEQCNT_ZERO(psi_seq);
#[no_mangle]
pub unsafe extern "C" fn psi_write_begin(cpu: c_int) {
    write_seqcount_begin(per_cpu_ptr(&psi_seq, cpu));
    }
#[no_mangle]
pub unsafe extern "C" fn psi_write_end(cpu: c_int) {
    write_seqcount_end(per_cpu_ptr(&psi_seq, cpu));
    }
#[no_mangle]
pub unsafe extern "C" fn psi_read_begin(cpu: c_int) -> u32 {
    return read_seqcount_begin(per_cpu_ptr(&psi_seq, cpu));
    }
#[no_mangle]
pub unsafe extern "C" fn psi_read_retry(cpu: c_int, seq: u32) -> bool {
    return read_seqcount_retry(per_cpu_ptr(&psi_seq, cpu), seq);
    }
// forward_decl: psi_avgs_work;
// forward_decl: poll_timer_fn;
#[no_mangle]
unsafe extern "C" fn group_init(group: *mut psi_group) {
    group.enabled = true;
    group.avg_last_update = sched_clock();
    group.avg_next_update = group.avg_last_update + psi_period;
    mutex_init(&group.avgs_lock);
// Init avg trigger-related members
    INIT_LIST_HEAD(&group.avg_triggers);
    memset(group.avg_nr_triggers, 0, sizeof!(group.avg_nr_triggers));
    INIT_DELAYED_WORK(&group.avgs_work, psi_avgs_work);
// Init rtpoll trigger-related members
    atomic_set(&group.rtpoll_scheduled, 0);
    mutex_init(&group.rtpoll_trigger_lock);
    INIT_LIST_HEAD(&group.rtpoll_triggers);
    group.rtpoll_min_period = U32_MAX;
    group.rtpoll_next_update = ULLONG_MAX;
    init_waitqueue_head(&group.rtpoll_wait);
    timer_setup(&group.rtpoll_timer, poll_timer_fn, 0);
    rcu_assign_pointer(group.rtpoll_task, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn psi_init()  {
    if (!psi_enable) {
    static_branch_enable(&psi_disabled);
    static_branch_disable(&psi_cgroups_enabled);
    return;
    }
    if (!cgroup_psi_enabled()) {
    static_branch_disable(&psi_cgroups_enabled);
    }
    psi_period = jiffies_to_nsecs(PSI_FREQ);
    group_init(&psi_system);
    }
#[no_mangle]
unsafe extern "C" fn test_states(tasks: *mut c_uint, state_mask: u32) -> u32 {
pub static mut oncpu: bool = false;
    if (tasks[NR_IOWAIT]) {
    state_mask |= BIT(PSI_IO_SOME);
    if (!tasks[NR_RUNNING]) {
    state_mask |= BIT(PSI_IO_FULL);
    }
    }
    if (tasks[NR_MEMSTALL]) {
    state_mask |= BIT(PSI_MEM_SOME);
    if (tasks[NR_RUNNING] == tasks[NR_MEMSTALL_RUNNING]) {
    state_mask |= BIT(PSI_MEM_FULL);
    }
    }
    if (tasks[NR_RUNNING] > oncpu) {
    state_mask |= BIT(PSI_CPU_SOME);
    }
    if (tasks[NR_RUNNING] && !oncpu) {
    state_mask |= BIT(PSI_CPU_FULL);
    }
    if (tasks[NR_IOWAIT] || tasks[NR_MEMSTALL] || tasks[NR_RUNNING]) {
    state_mask |= BIT(PSI_NONIDLE);
    }
    return state_mask;
    }
#[no_mangle]
pub unsafe extern "C" fn get_recent_times(group: *mut psi_group, cpu: c_int, aggregator: psi_aggregators, times: *mut u32, pchanged_states: *mut u32) {
    let mut groupc = per_cpu_ptr(group.pcpu, cpu);
pub static mut current_cpu: c_int = 0;
    unsigned int tasks[NR_PSI_TASK_COUNTS];
    u64 now, state_start;
    enum psi_states s;
    let mut seq = 0;
    let mut state_mask = 0;
// pchanged_states = 0;
// Snapshot a coherent view of the CPU state
    do {
    seq = psi_read_begin(cpu);
    now = cpu_clock(cpu);
    memcpy(times, groupc.times, sizeof!(groupc.times));
    state_mask = groupc.state_mask;
    state_start = groupc.state_start;
    if (cpu == current_cpu) {
    memcpy(tasks, groupc.tasks, sizeof!(groupc.tasks));
    }
    } while (psi_read_retry(cpu, seq));
// Calculate state time deltas against the previous snapshot
    while (s < NR_PSI_STATES) {
    let mut delta = 0;
//
// In addition to already concluded states, we also
// incorporate currently active states on the CPU,
// since states may last for many sampling periods.
//
// This way we keep our delta sampling buckets small
// (u32) and our reported pressure close to what's
// actually happening.
//
    if (state_mask & (1 << s)) {
    times[s] += now - state_start;
    }
    delta = times[s] - groupc.times_prev[aggregator][s];
    groupc.times_prev[aggregator][s] = times[s];
    times[s] = delta;
    if (delta) {
// pchanged_states |= (1 << s);
    }
    }
//
// When collect_percpu_times() from the avgs_work, we don't want to
// re-arm avgs_work when all CPUs are IDLE. But the current CPU running
// this avgs_work is never IDLE, cause avgs_work can't be shut off.
// So for the current CPU, we need to re-arm avgs_work only when
// (NR_RUNNING > 1 || NR_IOWAIT > 0 || NR_MEMSTALL > 0), for other CPUs
// we can just check PSI_NONIDLE delta.
//
    if (current_work() == &group.avgs_work.work) {
    let mut reschedule = 0;
    if (cpu == current_cpu) {
    reschedule = tasks[NR_RUNNING] +
    tasks[NR_IOWAIT] +
    tasks[NR_MEMSTALL] > 1;
    }
    else {
    reschedule = *pchanged_states & (1 << PSI_NONIDLE);
    }
    if (reschedule) {
// pchanged_states |= PSI_STATE_RESCHEDULE;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn calc_avgs(missed_periods: c_int, time: u64, period: u64) {
    let mut pct = 0;
// Fill in zeroes for periods of no activity
    if (missed_periods) {
    avg[0] = calc_load_n(avg[0], EXP_10s, 0, missed_periods);
    avg[1] = calc_load_n(avg[1], EXP_60s, 0, missed_periods);
    avg[2] = calc_load_n(avg[2], EXP_300s, 0, missed_periods);
    }
// Sample the most recent active period
    pct = div_u64(time * 100, period);
    pct *= FIXED_1;
    avg[0] = calc_load(avg[0], EXP_10s, pct);
    avg[1] = calc_load(avg[1], EXP_60s, pct);
    avg[2] = calc_load(avg[2], EXP_300s, pct);
    }
#[no_mangle]
pub unsafe extern "C" fn collect_percpu_times(group: *mut psi_group, aggregator: psi_aggregators, pchanged_states: *mut u32) {
    u64 deltas[NR_PSI_STATES - 1] = { 0, };
pub static mut nonidle_total: c_ulong = 0;
pub static mut changed_states: u32 = 0;
    let mut cpu = 0;
    let mut s = 0;
//
// Collect the per-cpu time buckets and average them into a
// single time sample that is normalized to wall clock time.
//
// For averaging, each CPU is weighted by its non-idle time in
// the sampling period. This eliminates artifacts from uneven
// loading, or even entirely idle CPUs.
//
    for_each_possible_cpu(cpu) {
    u32 times[NR_PSI_STATES];
    let mut nonidle = 0;
    let mut cpu_changed_states = 0;
    get_recent_times(group, cpu, aggregator, times,
    &cpu_changed_states);
    changed_states |= cpu_changed_states;
    nonidle = nsecs_to_jiffies(times[PSI_NONIDLE]);
    nonidle_total += nonidle;
    for (s = 0; s < PSI_NONIDLE; s++) {
    deltas[s] += (u64)times[s] * nonidle;
    }
    }
//
// Integrate the sample into the running statistics that are
// reported to userspace: the cumulative stall times and the
// decaying averages.
//
// Pressure percentages are sampled at PSI_FREQ. We might be
// called more often when the user polls more frequently than
// that; we might be called less often when there is no task
// activity, thus no data, and clock ticks are sporadic. The
// below handles both.
//
// total=
    for (s = 0; s < NR_PSI_STATES - 1; s++) {
    group.total[aggregator][s] +=
    div_u64(deltas[s], max(nonidle_total, 1UL));
    }
    if (pchanged_states) {
// pchanged_states = changed_states;
    }
    }
// Trigger tracking window manipulations
#[no_mangle]
pub unsafe extern "C" fn window_reset(win: *mut psi_window, now: u64, value: u64, prev_growth: u64) {
    win.start_time = now;
    win.start_value = value;
    win.prev_growth = prev_growth;
    }
//
// PSI growth tracking window update and growth calculation routine.
//
// This approximates a sliding tracking window by interpolating
// partially elapsed windows using historical growth data from the
// previous intervals. This minimizes memory requirements (by not storing
// all the intermediate values in the previous window) and simplifies
// the calculations. It works well because PSI signal changes only in
// positive direction and over relatively small window sizes the growth
// is close to linear.
//
#[no_mangle]
unsafe extern "C" fn window_update(win: *mut psi_window, now: u64, value: u64) -> u64 {
    let mut elapsed = 0;
    let mut growth = 0;
    elapsed = now - win.start_time;
    growth = value - win.start_value;
//
// After each tracking window passes win->start_value and
// win->start_time get reset and win->prev_growth stores
// the average per-window growth of the previous window.
// win->prev_growth is then used to interpolate additional
// growth from the previous window assuming it was linear.
//
    if (elapsed > win.size) {
    window_reset(win, now, value, growth);
    }
    else {
    let mut remaining = 0;
    remaining = win.size - elapsed;
    growth += div64_u64(win.prev_growth * remaining, win.size);
    }
    return growth;
    }
#[no_mangle]
pub unsafe extern "C" fn update_triggers(group: *mut psi_group, now: u64, aggregator: psi_aggregators) {
pub static mut t: *mut c_void = core::ptr::null_mut();
    let mut total = group.total[aggregator];
pub static mut triggers: *mut c_void = core::ptr::null_mut();
pub static mut aggregator_total: *mut c_void = core::ptr::null_mut();
    if (aggregator == PSI_AVGS) {
    triggers = &group.avg_triggers;
    aggregator_total = group.avg_total;
    } else {
    triggers = &group.rtpoll_triggers;
    aggregator_total = group.rtpoll_total;
    }
//
// On subsequent updates, calculate growth deltas and let
// watchers know when their specified thresholds are exceeded.
//
    list_for_each_entry(t, triggers, node) {
    let mut growth = 0;
    let mut new_stall = 0;
    new_stall = aggregator_total[t.state] != total[t.state];
// Check for stall activity or a previous threshold breach
    if (!new_stall && !t.pending_event) {
    continue;
    }
//
// Check for new stall activity, as well as deferred
// events that occurred in the last window after the
// trigger had already fired (we want to ratelimit
// events without dropping any).
//
    if (new_stall) {
// Calculate growth since last update
    growth = window_update(&t.win, now, total[t.state]);
    if (!t.pending_event) {
    if (growth < t.threshold) {
    continue;
    }
    t.pending_event = true;
    }
    }
// Limit event signaling to once per window
    if (now < t.last_event_time + t.win.size) {
    continue;
    }
// Generate an event
    if (cmpxchg(&t.event, 0, 1) == 0) {
    if (t.of) {
    kernfs_notify(t.of.kn);
    }
    else {
    wake_up_interruptible(&t.event_wait);
    }
    }
    t.last_event_time = now;
// Reset threshold breach flag once event got generated
    t.pending_event = false;
    }
    }
#[no_mangle]
unsafe extern "C" fn update_averages(group: *mut psi_group, now: u64) -> u64 {
pub static mut missed_periods: c_ulong = 0;
    u64 expires, period;
    let mut avg_next_update = 0;
    let mut s = 0;
// avgX=
    expires = group.avg_next_update;
    if (now - expires >= psi_period) {
    missed_periods = div_u64(now - expires, psi_period);
    }
//
// The periodic clock tick can get delayed for various
// reasons, especially on loaded systems. To avoid clock
// drift, we schedule the clock in fixed psi_period intervals.
// But the deltas we sample out of the per-cpu buckets above
// are based on the actual time elapsing between clock ticks.
//
    avg_next_update = expires + ((1 + missed_periods) * psi_period);
    period = now - (group.avg_last_update + (missed_periods * psi_period));
    group.avg_last_update = now;
    while (s < NR_PSI_STATES - 1) {
    let mut sample = 0;
    sample = group.total[PSI_AVGS][s] - group.avg_total[s];
//
// Due to the lockless sampling of the time buckets,
// recorded time deltas can slip into the next period,
// which under full pressure can result in samples in
// excess of the period length.
//
// We don't want to report non-sensical pressures in
// excess of 100%, nor do we want to drop such events
// on the floor. Instead we punt any overage into the
// future until pressure subsides. By doing this we
// don't underreport the occurring pressure curve, we
// just report it delayed by one period length.
//
// The error isn't cumulative. As soon as another
// delta slips from a period P to P+1, by definition
// it frees up its time T in P.
//
    if (sample > period) {
    sample = period;
    }
    group.avg_total[s] += sample;
    calc_avgs(group.avg[s], missed_periods, sample, period);
    }
    return avg_next_update;
    }
#[no_mangle]
unsafe extern "C" fn psi_avgs_work(work: *mut work_struct) {
pub static mut dwork: *mut c_void = core::ptr::null_mut();
pub static mut group: *mut c_void = core::ptr::null_mut();
    let mut changed_states = 0;
    let mut now = 0;
    dwork = to_delayed_work(work);
    group = container_of!(dwork, psi_group, avgs_work);
    mutex_lock(&group.avgs_lock);
    now = sched_clock();
    collect_percpu_times(group, PSI_AVGS, &changed_states);
//
// If there is task activity, periodically fold the per-cpu
// times and feed samples into the running averages. If things
// are idle and there is no data to process, stop the clock.
// Once restarted, we'll catch up the running averages in one
// go - see calc_avgs() and missed_periods.
//
    if (now >= group.avg_next_update) {
    update_triggers(group, now, PSI_AVGS);
    group.avg_next_update = update_averages(group, now);
    }
    if (changed_states & PSI_STATE_RESCHEDULE) {
    schedule_delayed_work(dwork, nsecs_to_jiffies(
    group.avg_next_update - now) + 1);
    }
    mutex_unlock(&group.avgs_lock);
    }
#[no_mangle]
unsafe extern "C" fn init_rtpoll_triggers(group: *mut psi_group, now: u64) {
pub static mut t: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(t, &group.rtpoll_triggers, node) {
    window_reset(&t.win, now,
    group.total[PSI_POLL][t.state], 0);
    }
    memcpy(group.rtpoll_total, group.total[PSI_POLL],
    sizeof!(group.rtpoll_total));
    group.rtpoll_next_update = now + group.rtpoll_min_period;
    }
// Schedule rtpolling if it's not already scheduled or forced.
#[no_mangle]
pub unsafe extern "C" fn psi_schedule_rtpoll_work(group: *mut psi_group, delay: c_ulong, force: bool) {
pub static mut task: *mut c_void = core::ptr::null_mut();
//
// atomic_xchg should be called even when !force to provide a
// full memory barrier (see the comment inside psi_rtpoll_work).
//
    if (atomic_xchg(&group.rtpoll_scheduled, 1) && !force) {
    return;
    }
    rcu_read_lock();
    task = rcu_dereference(group.rtpoll_task);
//
// kworker might be NULL in case psi_trigger_destroy races with
// psi_task_change (hotpath) which can't use locks
//
    if (likely(task)) {
    mod_timer(&group.rtpoll_timer, jiffies + delay);
    }
    else {
    atomic_set(&group.rtpoll_scheduled, 0);
    }
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn psi_rtpoll_work(group: *mut psi_group) {
pub static mut force_reschedule: bool = false;
    let mut changed_states = 0;
    let mut now = 0;
    mutex_lock(&group.rtpoll_trigger_lock);
    now = sched_clock();
    if (now > group.rtpoll_until) {
//
// We are either about to start or might stop rtpolling if no
// state change was recorded. Resetting rtpoll_scheduled leaves
// a small window for psi_group_change to sneak in and schedule
// an immediate rtpoll_work before we get to rescheduling. One
// potential extra wakeup at the end of the rtpolling window
// should be negligible and rtpoll_next_update still keeps
// updates correctly on schedule.
//
    atomic_set(&group.rtpoll_scheduled, 0);
//
// A task change can race with the rtpoll worker that is supposed to
// report on it. To avoid missing events, ensure ordering between
// rtpoll_scheduled and the task state accesses, such that if the
// rtpoll worker misses the state update, the task change is
// guaranteed to reschedule the rtpoll worker:
//
// rtpoll worker:
// atomic_set(rtpoll_scheduled, 0)
// smp_mb()
// LOAD states
//
// task change:
// STORE states
// if atomic_xchg(rtpoll_scheduled, 1) == 0:
// schedule rtpoll worker
//
// The atomic_xchg() implies a full barrier.
//
    smp_mb();
    } else {
// The rtpolling window is not over, keep rescheduling
    force_reschedule = true;
    }
    collect_percpu_times(group, PSI_POLL, &changed_states);
    if (changed_states & group.rtpoll_states) {
// Initialize trigger windows when entering rtpolling mode
    if (now > group.rtpoll_until) {
    init_rtpoll_triggers(group, now);
    }
//
// Keep the monitor active for at least the duration of the
// minimum tracking window as long as monitor states are
// changing.
//
    group.rtpoll_until = now +
    group.rtpoll_min_period * UPDATES_PER_WINDOW;
    }
    if (now > group.rtpoll_until) {
    group.rtpoll_next_update = ULLONG_MAX;
// goto;
    }
    if (now >= group.rtpoll_next_update) {
    if (changed_states & group.rtpoll_states) {
    update_triggers(group, now, PSI_POLL);
    memcpy(group.rtpoll_total, group.total[PSI_POLL],
    sizeof!(group.rtpoll_total));
    }
    group.rtpoll_next_update = now + group.rtpoll_min_period;
    }
    psi_schedule_rtpoll_work(group,
    nsecs_to_jiffies(group.rtpoll_next_update - now) + 1,
    force_reschedule);
// label;
    mutex_unlock(&group.rtpoll_trigger_lock);
    }
#[no_mangle]
unsafe extern "C" fn psi_rtpoll_worker(data: *mut c_void) -> c_int {
    let mut group = data;
    sched_set_fifo_low(current);
    while (true) {
    wait_event_interruptible(group.rtpoll_wait,
    atomic_cmpxchg(&group.rtpoll_wakeup, 1, 0) ||
    kthread_should_stop());
    if (kthread_should_stop()) {
    break;
    }
    psi_rtpoll_work(group);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn poll_timer_fn(t: *mut timer_list) {
    let mut group = timer_container_of(group, t, rtpoll_timer);
    atomic_set(&group.rtpoll_wakeup, 1);
    wake_up_interruptible(&group.rtpoll_wait);
    }
#[no_mangle]
unsafe extern "C" fn record_times(groupc: *mut psi_group_cpu, now: u64) {
    let mut delta = 0;
    delta = now - groupc.state_start;
    groupc.state_start = now;
    if (groupc.state_mask & (1 << PSI_IO_SOME)) {
    groupc.times[PSI_IO_SOME] += delta;
    if (groupc.state_mask & (1 << PSI_IO_FULL)) {
    groupc.times[PSI_IO_FULL] += delta;
    }
    }
    if (groupc.state_mask & (1 << PSI_MEM_SOME)) {
    groupc.times[PSI_MEM_SOME] += delta;
    if (groupc.state_mask & (1 << PSI_MEM_FULL)) {
    groupc.times[PSI_MEM_FULL] += delta;
    }
    }
    if (groupc.state_mask & (1 << PSI_CPU_SOME)) {
    groupc.times[PSI_CPU_SOME] += delta;
    if (groupc.state_mask & (1 << PSI_CPU_FULL)) {
    groupc.times[PSI_CPU_FULL] += delta;
    }
    }
    if (groupc.state_mask & (1 << PSI_NONIDLE)) {
    groupc.times[PSI_NONIDLE] += delta;
    }
    }

    for (typeof(group) iter = group; iter; iter = iter.parent) {
#[no_mangle]
pub unsafe extern "C" fn psi_group_change(group: *mut psi_group, cpu: c_int, clear: c_uint, set: c_uint, now: u64, wake_clock: bool) {
    }
pub static mut groupc: *mut c_void = core::ptr::null_mut();
    let mut t = 0;
    let mut m = 0;
    let mut state_mask = 0;
    lockdep_assert_rq_held(cpu_rq(cpu));
    groupc = per_cpu_ptr(group.pcpu, cpu);
//
// Start with TSK_ONCPU, which doesn't have a corresponding
// task count - it's just a boolean flag directly encoded in
// the state mask. Clear, set, or carry the current state if
// no changes are requested.
//
    if (unlikely(clear & TSK_ONCPU)) {
    state_mask = 0;
    clear &= ~TSK_ONCPU;
    } else if (unlikely(set & TSK_ONCPU)) {
    state_mask = PSI_ONCPU;
    set &= ~TSK_ONCPU;
    } else {
    state_mask = groupc.state_mask & PSI_ONCPU;
    }
//
// The rest of the state mask is calculated based on the task
// counts. Update those first, then construct the mask.
//
    for (t = 0, m = clear; m; m &= ~(1 << t), t++) {
    if (!(m & (1 << t))) {
    continue;
    }
    if (groupc.tasks[t]) {
    groupc.tasks[t]--;
    } else if (!psi_bug) {
    printk_deferred("psi: task underflow! cpu=%d t=%d tasks=[%u %u %u %u] clear=%x set=%x\n",
    cpu, t, groupc.tasks[0],
    groupc.tasks[1], groupc.tasks[2],
    groupc.tasks[3], clear, set);
    psi_bug = 1;
    }
    }
    for (t = 0; set; set &= ~(1 << t), t++) {
    if (set & (1 << t))
    groupc.tasks[t]++;
    }
    if (!group.enabled) {
//
// On the first group change after disabling PSI, conclude
// the current state and flush its time. This is unlikely
// to matter to the user, but aggregation (get_recent_times)
// may have already incorporated the live state into times_prev;
// avoid a delta sample underflow when PSI is later re-enabled.
//
    if (unlikely(groupc.state_mask & (1 << PSI_NONIDLE))) {
    record_times(groupc, now);
    }
    groupc.state_mask = state_mask;
    return;
    }
    state_mask = test_states(groupc.tasks, state_mask);
//
// Since we care about lost potential, a memstall is FULL
// when there are no other working tasks, but also when
// the CPU is actively reclaiming and nothing productive
// could run even if it were runnable. So when the current
// task in a cgroup is in_memstall, the corresponding groupc
// on that cpu is in PSI_MEM_FULL state.
//
    if (unlikely((state_mask & PSI_ONCPU) && cpu_curr(cpu).in_memstall)) {
    state_mask |= (1 << PSI_MEM_FULL);
    }
    record_times(groupc, now);
    groupc.state_mask = state_mask;
    if (state_mask & group.rtpoll_states) {
    psi_schedule_rtpoll_work(group, 1, false);
    }
    if (wake_clock && !delayed_work_pending(&group.avgs_work)) {
    schedule_delayed_work(&group.avgs_work, PSI_FREQ);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn task_psi_group(task: *mut task_struct) -> *mut c_void {

    if (static_branch_likely(&psi_cgroups_enabled)) {
    return cgroup_psi(task_dfl_cgroup(task));
    }

    return &psi_system;
    }
#[no_mangle]
unsafe extern "C" fn psi_flags_change(task: *mut task_struct, clear: c_int, set: c_int) {
    if (((task.psi_flags & set) ||
    (task.psi_flags & clear) != clear) &&
    !psi_bug) {
    printk_deferred("psi: inconsistent task state! task=%d:%s cpu=%d psi_flags=%x clear=%x set=%x\n",
    task.pid, task.comm, task_cpu(task),
    task.psi_flags, clear, set);
    psi_bug = 1;
    }
    task.psi_flags &= ~clear;
    task.psi_flags |= set;
    }
#[no_mangle]
pub unsafe extern "C" fn psi_task_change(task: *mut task_struct, clear: c_int, set: c_int) {
pub static mut cpu: c_int = 0;
    let mut now = 0;
    if (!task.pid) {
    return;
    }
    psi_flags_change(task, clear, set);
    psi_write_begin(cpu);
    now = cpu_clock(cpu);
    for_each_group(group, task_psi_group(task)) {
    psi_group_change(group, cpu, clear, set, now, true);
    }
    psi_write_end(cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn psi_task_switch(prev: *mut task_struct, next: *mut task_struct, sleep: bool) {
    let mut common = core::ptr::null_mut();
pub static mut cpu: c_int = 0;
    let mut now = 0;
    psi_write_begin(cpu);
    now = cpu_clock(cpu);
    if (next.pid) {
    psi_flags_change(next, 0, TSK_ONCPU);
//
// Set TSK_ONCPU on @next's cgroups. If @next shares any
// ancestors with @prev, those will already have @prev's
// TSK_ONCPU bit set, and we can stop the iteration there.
//
    for_each_group(group, task_psi_group(next)) {
    let mut groupc = per_cpu_ptr(group.pcpu, cpu);
    if (groupc.state_mask & PSI_ONCPU) {
    common = group;
    break;
    }
    psi_group_change(group, cpu, 0, TSK_ONCPU, now, true);
    }
    }
    if (prev.pid) {
pub static mut clear: c_int = 0;
pub static mut wake_clock: bool = true;
//
// When we're going to sleep, psi_dequeue() lets us
// handle TSK_RUNNING, TSK_MEMSTALL_RUNNING and
// TSK_IOWAIT here, where we can combine it with
// TSK_ONCPU and save walking common ancestors twice.
//
    if (sleep) {
    clear |= TSK_RUNNING;
    if (prev.in_memstall) {
    clear |= TSK_MEMSTALL_RUNNING;
    }
    if (prev.in_iowait) {
    set |= TSK_IOWAIT;
    }
//
// Periodic aggregation shuts off if there is a period of no
// task changes, so we wake it back up if necessary. However,
// don't do this if the task change is the aggregation worker
// itself going to sleep, or we'll ping-pong forever.
//
    if (unlikely((prev.flags & PF_WQ_WORKER) &&
    wq_worker_last_func(prev) == psi_avgs_work)) {
    wake_clock = false;
    }
    }
    psi_flags_change(prev, clear, set);
    for_each_group(group, task_psi_group(prev)) {
    if (group == common) {
    break;
    }
    psi_group_change(group, cpu, clear, set, now, wake_clock);
    }
//
// TSK_ONCPU is handled up to the common ancestor. If there are
// any other differences between the two tasks (e.g. prev goes
// to sleep, or only one task is memstall), finish propagating
// those differences all the way up to the root.
//
    if ((prev.psi_flags ^ next.psi_flags) & ~TSK_ONCPU) {
    clear &= ~TSK_ONCPU;
    for_each_group(group, common) {
    psi_group_change(group, cpu, clear, set, now, wake_clock);
    }
    }
    }
    psi_write_end(cpu);
    }

#[no_mangle]
pub unsafe extern "C" fn psi_account_irqtime(rq: *mut rq, curr: *mut task_struct, prev: *mut task_struct) {
pub static mut cpu: c_int = 0;
pub static mut groupc: *mut c_void = core::ptr::null_mut();
    let mut delta = 0;
    let mut irq = 0;
    let mut now = 0;
    if (static_branch_likely(&psi_disabled) || !irqtime_enabled()) {
    return;
    }
    if (!curr.pid) {
    return;
    }
    lockdep_assert_rq_held(rq);
    if (prev && task_psi_group(prev) == task_psi_group(curr)) {
    return;
    }
    irq = irq_time_read(cpu);
    delta = (s64)(irq - rq.psi_irq_time);
    if (delta <= 0) {
    return;
    }
    rq.psi_irq_time = irq;
    psi_write_begin(cpu);
    now = cpu_clock(cpu);
    for_each_group(group, task_psi_group(curr)) {
    if (!group.enabled) {
    continue;
    }
    groupc = per_cpu_ptr(group.pcpu, cpu);
    record_times(groupc, now);
    groupc.times[PSI_IRQ_FULL] += delta;
    if (group.rtpoll_states & (1 << PSI_IRQ_FULL)) {
    psi_schedule_rtpoll_work(group, 1, false);
    }
    }
    psi_write_end(cpu);
    }

//
// psi_memstall_enter - mark the beginning of a memory stall section
// @flags: flags to handle nested sections
//
// Marks the calling task as being stalled due to a lack of memory,
// such as waiting for a refault or performing reclaim.
//
#[no_mangle]
pub unsafe extern "C" fn psi_memstall_enter(flags: *mut c_ulong) {
pub static mut rf: usize = 0;
pub static mut rq: *mut c_void = core::ptr::null_mut();
    if (static_branch_likely(&psi_disabled)) {
    return;
    }
// flags = current->in_memstall;
    if (*flags) {
    return;
    }
//
// in_memstall setting & accounting needs to be atomic wrt
// changes to the task's scheduling state, otherwise we can
// race with CPU migration.
//
    rq = this_rq_lock_irq(&rf);
    current.in_memstall = 1;
    psi_task_change(current, 0, TSK_MEMSTALL | TSK_MEMSTALL_RUNNING);
    rq_unlock_irq(rq, &rf);
    }
    EXPORT_SYMBOL_GPL(psi_memstall_enter);
//
// psi_memstall_leave - mark the end of an memory stall section
// @flags: flags to handle nested memdelay sections
//
// Marks the calling task as no longer stalled due to lack of memory.
//
#[no_mangle]
pub unsafe extern "C" fn psi_memstall_leave(flags: *mut c_ulong) {
pub static mut rf: usize = 0;
pub static mut rq: *mut c_void = core::ptr::null_mut();
    if (static_branch_likely(&psi_disabled)) {
    return;
    }
    if (*flags) {
    return;
    }
//
// in_memstall clearing & accounting needs to be atomic wrt
// changes to the task's scheduling state, otherwise we could
// race with CPU migration.
//
    rq = this_rq_lock_irq(&rf);
    current.in_memstall = 0;
    psi_task_change(current, TSK_MEMSTALL | TSK_MEMSTALL_RUNNING, 0);
    rq_unlock_irq(rq, &rf);
    }
    EXPORT_SYMBOL_GPL(psi_memstall_leave);

#[no_mangle]
pub unsafe extern "C" fn psi_cgroup_alloc(cgroup: *mut cgroup) -> c_int {
    if (!static_branch_likely(&psi_cgroups_enabled)) {
    return 0;
    }
    cgroup.psi = kzalloc_obj(psi_group);
    if (!cgroup.psi) {
    return -ENOMEM;
    }
    cgroup.psi.pcpu = alloc_percpu(psi_group_cpu);
    if (!cgroup.psi.pcpu) {
    kfree(cgroup.psi);
    return -ENOMEM;
    }
    group_init(cgroup.psi);
    cgroup.psi.parent = cgroup_psi(cgroup_parent(cgroup));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn psi_cgroup_free(cgroup: *mut cgroup) {
    if (!static_branch_likely(&psi_cgroups_enabled)) {
    return;
    }
    cancel_delayed_work_sync(&cgroup.psi.avgs_work);
//
// A psi_schedule_rtpoll_work() call racing the last trigger's
// destruction may have re-armed the timer after psi_trigger_destroy()
// deleted it. Spurious firing while the group is alive is harmless.
//
    timer_shutdown_sync(&cgroup.psi.rtpoll_timer);
    free_percpu(cgroup.psi.pcpu);
// All triggers must be removed by now
    WARN_ONCE(cgroup.psi.rtpoll_states, "psi: trigger leak\n");
    kfree(cgroup.psi);
    }
//
// cgroup_move_task - move task to a different cgroup
// @task: the task
// @to: the target css_set
//
// Move task to a new cgroup and safely migrate its associated stall
// state between the different groups.
//
// This function acquires the task's rq lock to lock out concurrent
// changes to the task's scheduling state and - in case the task is
// running - concurrent changes to its stall state.
//
#[no_mangle]
pub unsafe extern "C" fn cgroup_move_task(task: *mut task_struct, to: *mut css_set) {
    let mut task_flags = 0;
pub static mut rf: usize = 0;
pub static mut rq: *mut c_void = core::ptr::null_mut();
    if (!static_branch_likely(&psi_cgroups_enabled)) {
//
// Lame to do this here, but the scheduler cannot be locked
// from the outside, so we move cgroups from inside sched/.
//
    rcu_assign_pointer(task.cgroups, to);
    return;
    }
    rq = task_rq_lock(task, &rf);
//
// We may race with schedule() dropping the rq lock between
// deactivating prev and switching to next. Because the psi
// updates from the deactivation are deferred to the switch
// callback to save cgroup tree updates, the task's scheduling
// state here is not coherent with its psi state:
//
// schedule()                   cgroup_move_task()
// rq_lock()
// deactivate_task()
// p->on_rq = 0
// psi_dequeue() // defers TSK_RUNNING & TSK_IOWAIT updates
// pick_next_task()
// rq_unlock()
// rq_lock()
// psi_task_change() // old cgroup
// task->cgroups = to
// psi_task_change() // new cgroup
// rq_unlock()
// rq_lock()
// psi_sched_switch() // does deferred updates in new cgroup
//
// Don't rely on the scheduling state. Use psi_flags instead.
//
    task_flags = task.psi_flags;
    if (task_flags) {
    psi_task_change(task, task_flags, 0);
    }
// See comment above
    rcu_assign_pointer(task.cgroups, to);
    if (task_flags) {
    psi_task_change(task, 0, task_flags);
    }
    task_rq_unlock(rq, task, &rf);
    }
#[no_mangle]
pub unsafe extern "C" fn psi_cgroup_restart(group: *mut psi_group) {
    let mut cpu = 0;
//
// After we disable psi_group->enabled, we don't actually
// stop percpu tasks accounting in each psi_group_cpu,
instead only stop test_states() loop, record_times()
// and averaging worker, see psi_group_change() for details.
//
// When disable cgroup PSI, this function has nothing to sync
// since cgroup pressure files are hidden and percpu psi_group_cpu
// would see !psi_group->enabled and only do task accounting.
//
// When re-enable cgroup PSI, this function use psi_group_change()
// to get correct state mask from test_states() loop on tasks[],
// and restart groupc->state_start from now, use .clear = .set = 0
// here since no task status really changed.
//
    if (!group.enabled) {
    return;
    }
    for_each_possible_cpu(cpu) {
    let mut now = 0;
    guard(rq_lock_irq)(cpu_rq(cpu));
    psi_write_begin(cpu);
    now = cpu_clock(cpu);
    psi_group_change(group, cpu, 0, 0, now, true);
    psi_write_end(cpu);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn psi_show(m: *mut seq_file, group: *mut psi_group, res: psi_res) -> c_int {
pub static mut only_full: bool = false;
    let mut full = 0;
    let mut now = 0;
    if (static_branch_likely(&psi_disabled)) {
    return -EOPNOTSUPP;
    }

    if (!irqtime_enabled() && res == PSI_IRQ) {
    return -EOPNOTSUPP;
    }

// Update averages before reporting them
    mutex_lock(&group.avgs_lock);
    now = sched_clock();
    collect_percpu_times(group, PSI_AVGS, core::ptr::null_mut());
    if (now >= group.avg_next_update) {
    group.avg_next_update = update_averages(group, now);
    }
    mutex_unlock(&group.avgs_lock);

    only_full = res == PSI_IRQ;

    while (full < 2 - only_full) {
    unsigned long avg[3] = { 0, };
pub static mut total: u64 = 0;
    let mut w = 0;
// CPU FULL is undefined at the system level
    if (!(group == &psi_system && res == PSI_CPU && full)) {
    for (w = 0; w < 3; w++) {
    avg[w] = group.avg[res * 2 + full][w];
    }
    total = div_u64(group.total[PSI_AVGS][res * 2 + full],
    NSEC_PER_USEC);
    }
    seq_printf(m, "%s avg10=%lu.%02lu avg60=%lu.%02lu avg300=%lu.%02lu total=%llu\n",
    full || only_full ? "full" : "some",
    LOAD_INT(avg[0]), LOAD_FRAC(avg[0]),
    LOAD_INT(avg[1]), LOAD_FRAC(avg[1]),
    LOAD_INT(avg[2]), LOAD_FRAC(avg[2]),
    total);
    }
    return 0;
    }
//
// Create @group's rtpoll worker after psi_trigger_create() reported the need
// for one. kthread creation depends on the whole fork path and we don't want
// all of that nested inside cgroup_mutex, so the caller must drop it and any
// other lock that forks can wait behind. If two callers race, the loser stops
// its never-woken kthread.
//
#[no_mangle]
pub unsafe extern "C" fn psi_trigger_create_rtpoll_worker(group: *mut psi_group) -> c_int {
pub static mut task: *mut c_void = core::ptr::null_mut();
    task = kthread_create(psi_rtpoll_worker, group, "psimon");
    if (IS_ERR(task)) {
    return PTR_ERR(task);
    }
    scoped_guard(mutex, &group.rtpoll_trigger_lock) {
    if (!rcu_access_pointer(group.rtpoll_task)) {
    atomic_set(&group.rtpoll_wakeup, 0);
    wake_up_process(task);
    rcu_assign_pointer(group.rtpoll_task, task);
//
// Poll once to catch up on scheduling attempts dropped
// while there was no rtpoll worker.
//
    psi_schedule_rtpoll_work(group, 1, true);
    return 0;
    }
    }
    kthread_stop(task);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn psi_trigger_create(group: *mut psi_group, buf: *mut c_char, res: psi_res, file: *mut file, of: *mut kernfs_open_file, need_rtpoll_worker: *mut bool) -> *mut c_void {
pub static mut t: *mut c_void = core::ptr::null_mut();
    enum psi_states state;
    let mut threshold_us = 0;
    let mut privileged = 0;
    let mut window_us = 0;
// need_rtpoll_worker = false;
    if (static_branch_likely(&psi_disabled)) {
    return ERR_PTR(-EOPNOTSUPP);
    }
//
// Checking the privilege here on file->f_cred implies that a privileged user
// could open the file and delegate the write to an unprivileged one.
//
    privileged = cap_raised(file.f_cred.cap_effective, CAP_SYS_RESOURCE);
    if (sscanf(buf, "some %u %u", &threshold_us, &window_us) == 2) {
    state = PSI_IO_SOME + res * 2;
    }

    else if (sscanf(buf, "full %u %u", &threshold_us, &window_us) == 2) {
    state = PSI_IO_FULL + res * 2;
    }
    else {
    return ERR_PTR(-EINVAL);
    }

    if (res == PSI_IRQ && --state != PSI_IRQ_FULL) {
    return ERR_PTR(-EINVAL);
    }

    if (state >= PSI_NONIDLE) {
    return ERR_PTR(-EINVAL);
    }
    if (window_us == 0 || window_us > WINDOW_MAX_US) {
    return ERR_PTR(-EINVAL);
    }
//
// Unprivileged users can only use 2s windows so that averages aggregation
// work is used, and no RT threads need to be spawned.
//
    if (!privileged && window_us % 2000000) {
    return ERR_PTR(-EINVAL);
    }
// Check threshold
    if (threshold_us == 0 || threshold_us > window_us) {
    return ERR_PTR(-EINVAL);
    }
    t = kmalloc_obj(*t);
    if (!t) {
    return ERR_PTR(-ENOMEM);
    }
    t.group = group;
    t.state = state;
    t.threshold = threshold_us * NSEC_PER_USEC;
    t.win.size = window_us * NSEC_PER_USEC;
    window_reset(&t.win, sched_clock(),
    group.total[PSI_POLL][t.state], 0);
    t.event = 0;
    t.last_event_time = 0;
    t.of = of;
    if (!of) {
    init_waitqueue_head(&t.event_wait);
    }
    t.pending_event = false;
    t.aggregator = privileged ? PSI_POLL : PSI_AVGS;
    if (privileged) {
    mutex_lock(&group.rtpoll_trigger_lock);
    list_add(&t.node, &group.rtpoll_triggers);
    group.rtpoll_min_period = min(group.rtpoll_min_period,
    div_u64(t.win.size, UPDATES_PER_WINDOW));
    group.rtpoll_nr_triggers[t.state]++;
    group.rtpoll_states |= (1 << t.state);
// need_rtpoll_worker = !rcu_access_pointer(group->rtpoll_task);
    mutex_unlock(&group.rtpoll_trigger_lock);
    } else {
    mutex_lock(&group.avgs_lock);
    list_add(&t.node, &group.avg_triggers);
    group.avg_nr_triggers[t.state]++;
    mutex_unlock(&group.avgs_lock);
    }
    return t;
    }
#[no_mangle]
pub unsafe extern "C" fn psi_trigger_destroy(t: *mut psi_trigger) {
pub static mut group: *mut c_void = core::ptr::null_mut();
    let mut task_to_destroy = core::ptr::null_mut();
//
// We do not check psi_disabled since it might have been disabled after
// the trigger got created.
//
    if (!t) {
    return;
    }
    group = t.group;
//
// Wakeup waiters to stop polling and clear the queue to prevent it from
// being accessed later. Can happen if cgroup is deleted from under a
// polling process.
//
    if (t.of) {
    kernfs_notify(t.of.kn);
    }
    else {
    wake_up_interruptible(&t.event_wait);
    }
    if (t.aggregator == PSI_AVGS) {
    mutex_lock(&group.avgs_lock);
    if (!list_empty(&t.node)) {
    list_del(&t.node);
    group.avg_nr_triggers[t.state]--;
    }
    mutex_unlock(&group.avgs_lock);
    } else {
    mutex_lock(&group.rtpoll_trigger_lock);
    if (!list_empty(&t.node)) {
pub static mut tmp: *mut c_void = core::ptr::null_mut();
pub static mut period: u64 = 0;
    list_del(&t.node);
    group.rtpoll_nr_triggers[t.state]--;
    if (!group.rtpoll_nr_triggers[t.state]) {
    group.rtpoll_states &= ~(1 << t.state);
    }
//
// Reset min update period for the remaining triggers
// iff the destroying trigger had the min window size.
//
    if (group.rtpoll_min_period == div_u64(t.win.size, UPDATES_PER_WINDOW)) {
    list_for_each_entry(tmp, &group.rtpoll_triggers, node) {
    period = min(period, div_u64(tmp.win.size,
    UPDATES_PER_WINDOW));
    }
    group.rtpoll_min_period = period;
    }
// Destroy rtpoll_task when the last trigger is destroyed
    if (group.rtpoll_states == 0) {
    group.rtpoll_until = 0;
    task_to_destroy = rcu_dereference_protected(
    group.rtpoll_task,
    lockdep_is_held(&group.rtpoll_trigger_lock));
    rcu_assign_pointer(group.rtpoll_task, core::ptr::null_mut());
    timer_delete(&group.rtpoll_timer);
    }
    }
    mutex_unlock(&group.rtpoll_trigger_lock);
    }
//
// Wait for psi_schedule_rtpoll_work RCU to complete its read-side
// critical section before destroying the trigger and optionally the
// rtpoll_task.
//
    synchronize_rcu();
//
// Stop kthread 'psimon' after releasing rtpoll_trigger_lock to prevent
// a deadlock while waiting for psi_rtpoll_work to acquire
// rtpoll_trigger_lock
//
    if (task_to_destroy) {
//
// After the RCU grace period has expired, the worker
// can no longer be found through group->rtpoll_task.
//
    kthread_stop(task_to_destroy);
    atomic_set(&group.rtpoll_scheduled, 0);
    }
    kfree(t);
    }
    __poll_t psi_trigger_poll(void **trigger_ptr, file *file, poll_table *wait)
    {
pub static mut ret: __poll_t = 0;
pub static mut t: *mut c_void = core::ptr::null_mut();
    if (static_branch_likely(&psi_disabled)) {
    return DEFAULT_POLLMASK | EPOLLERR | EPOLLPRI;
    }
    t = smp_load_acquire(trigger_ptr);
    if (!t) {
    return DEFAULT_POLLMASK | EPOLLERR | EPOLLPRI;
    }
    if (t.of) {
    kernfs_generic_poll(t.of, wait);
    }
    else {
    poll_wait(file, &t.event_wait, wait);
    }
    if (cmpxchg(&t.event, 1, 0) == 1) {
    ret |= EPOLLPRI;
    }
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn psi_io_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    return psi_show(m, &psi_system, PSI_IO);
    }
#[no_mangle]
unsafe extern "C" fn psi_memory_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    return psi_show(m, &psi_system, PSI_MEM);
    }
#[no_mangle]
unsafe extern "C" fn psi_cpu_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    return psi_show(m, &psi_system, PSI_CPU);
    }
#[no_mangle]
unsafe extern "C" fn psi_io_open(inode: *mut inode, file: *mut file) -> c_int {
    return single_open(file, psi_io_show, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn psi_memory_open(inode: *mut inode, file: *mut file) -> c_int {
    return single_open(file, psi_memory_show, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn psi_cpu_open(inode: *mut inode, file: *mut file) -> c_int {
    return single_open(file, psi_cpu_show, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn psi_write(file: *mut file, user_buf: *mut c_char, nbytes: size_t, res: psi_res) -> ssize_t {
    char buf[32];
    let mut buf_size = 0;
pub static mut seq: *mut c_void = core::ptr::null_mut();
pub static mut new: *mut c_void = core::ptr::null_mut();
    let mut need_rtpoll_worker = 0;
    let mut ret = 0;
    if (static_branch_likely(&psi_disabled)) {
    return -EOPNOTSUPP;
    }
    if (!nbytes) {
    return -EINVAL;
    }
    buf_size = min(nbytes, sizeof!(buf));
    if (copy_from_user(buf, user_buf, buf_size)) {
    return -EFAULT;
    }
    buf[buf_size - 1] = '\0';
    seq = file.private_data;
// Take seq->lock to protect seq->private from concurrent writes
    mutex_lock(&seq.lock);
// Allow only one trigger per file descriptor
    if (seq.private) {
    mutex_unlock(&seq.lock);
    return -EBUSY;
    }
    new = psi_trigger_create(&psi_system, buf, res, file, core::ptr::null_mut(),
    &need_rtpoll_worker);
    if (IS_ERR(new)) {
    mutex_unlock(&seq.lock);
    return PTR_ERR(new);
    }
    if (need_rtpoll_worker) {
    ret = psi_trigger_create_rtpoll_worker(&psi_system);
    if (ret) {
    psi_trigger_destroy(new);
    mutex_unlock(&seq.lock);
    return ret;
    }
    }
    smp_store_release(&seq.private, new);
    mutex_unlock(&seq.lock);
    return nbytes;
    }
#[no_mangle]
pub unsafe extern "C" fn psi_io_write(file: *mut file, user_buf: *mut c_char, nbytes: size_t, ppos: *mut loff_t) -> ssize_t {
    return psi_write(file, user_buf, nbytes, PSI_IO);
    }
#[no_mangle]
pub unsafe extern "C" fn psi_memory_write(file: *mut file, user_buf: *mut c_char, nbytes: size_t, ppos: *mut loff_t) -> ssize_t {
    return psi_write(file, user_buf, nbytes, PSI_MEM);
    }
#[no_mangle]
pub unsafe extern "C" fn psi_cpu_write(file: *mut file, user_buf: *mut c_char, nbytes: size_t, ppos: *mut loff_t) -> ssize_t {
    return psi_write(file, user_buf, nbytes, PSI_CPU);
    }
#[no_mangle]
unsafe extern "C" fn psi_fop_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let mut seq = file.private_data;
    return psi_trigger_poll(&seq.private, file, wait);
    }
#[no_mangle]
unsafe extern "C" fn psi_fop_release(inode: *mut inode, file: *mut file) -> c_int {
    let mut seq = file.private_data;
    psi_trigger_destroy(seq.private);
    return single_release(inode, file);
    }
pub static mut proc_ops: usize = 0;
pub static mut proc_ops: usize = 0;
pub static mut proc_ops: usize = 0;

#[no_mangle]
unsafe extern "C" fn psi_irq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    return psi_show(m, &psi_system, PSI_IRQ);
    }
#[no_mangle]
unsafe extern "C" fn psi_irq_open(inode: *mut inode, file: *mut file) -> c_int {
    return single_open(file, psi_irq_show, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn psi_irq_write(file: *mut file, user_buf: *mut c_char, nbytes: size_t, ppos: *mut loff_t) -> ssize_t {
    return psi_write(file, user_buf, nbytes, PSI_IRQ);
    }
pub static mut proc_ops: usize = 0;

#[no_mangle]
unsafe extern "C" fn psi_proc_init() -> c_int {
    if (psi_enable) {
    proc_mkdir("pressure", core::ptr::null_mut());
    proc_create("pressure/io", 0666, core::ptr::null_mut(), &psi_io_proc_ops);
    proc_create("pressure/memory", 0666, core::ptr::null_mut(), &psi_memory_proc_ops);
    proc_create("pressure/cpu", 0666, core::ptr::null_mut(), &psi_cpu_proc_ops);

    proc_create("pressure/irq", 0666, core::ptr::null_mut(), &psi_irq_proc_ops);

    }
    return 0;
    }
    module_init!(psi_proc_init);
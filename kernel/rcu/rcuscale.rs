//! Automatically rewritten from C to Rust
//! Source: kernel/rcu/rcuscale.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Read-Copy Update module-based scalability-test facility
//
// Copyright (C) IBM Corporation, 2015
//
// Authors: Paul E. McKenney <paulmck@linux.ibm.com>
//

    MODULE_DESCRIPTION("Read-Copy Update module-based scalability-test facility");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Paul E. McKenney <paulmck@linux.ibm.com>");

    pr_alert("%s" SCALE_FLAG " %s\n", scale_type, s)

    do { if (verbose) pr_alert("%s" SCALE_FLAG " %s\n", scale_type, s); } while (0)

    pr_alert("%s" SCALE_FLAG "!!! %s\n", scale_type, s)
//
// The intended use cases for the nreaders and nwriters module parameters
// are as follows:
//
// 1.	Specify only the nr_cpus kernel boot parameter.  This will
// set both nreaders and nwriters to the value specified by
// nr_cpus for a mixed reader/writer test.
//
// 2.	Specify the nr_cpus kernel boot parameter, but set
// rcuscale.nreaders to zero.  This will set nwriters to the
// value specified by nr_cpus for an update-only test.
//
// 3.	Specify the nr_cpus kernel boot parameter, but set
// rcuscale.nwriters to zero.  This will set nreaders to the
// value specified by nr_cpus for a read-only test.
//
// Various other use cases may of course be specified.
//
// Note that this test's readers are intended only as a test load for
// the writers.  The reader scalability statistics will be overly
// pessimistic due to the per-critical-section interrupt disabling,
// test-end checks, and the pair of calls through pointers.
//
    torture_param(bool, gp_async, false, "Use asynchronous GP wait primitives");
    torture_param(int, gp_async_max, 1000, "Max # outstanding waits per writer");
    torture_param(bool, gp_exp, false, "Use expedited GP wait primitives");
    torture_param(int, holdoff, 10, "Holdoff time before test start (s)");
    torture_param(int, minruntime, 0, "Minimum run time (s)");
    torture_param(int, nreaders, -1, "Number of RCU reader threads");
    torture_param(int, nwriters, -1, "Number of RCU updater threads");
    torture_param(int, shutdown_secs, !IS_MODULE(CONFIG_RCU_SCALE_TEST) * 300,
    "Shutdown at end of scalability tests or at specified timeout (s).");
    torture_param(int, verbose, 1, "Enable verbose debugging printk()s");
    torture_param(int, writer_holdoff, 0, "Holdoff (us) between GPs, zero to disable");
    torture_param(int, writer_holdoff_jiffies, 0, "Holdoff (jiffies) between GPs, zero to disable");
    torture_param(int, nexp, 0, "Number of expedited GP threads to run concurrently");
    torture_param(int, exp_interval, 0, "Interval (us) between expedited GPs, zero to disable");
    torture_param(int, kfree_rcu_test, 0, "Do we run a kfree_rcu() scale test?");
    torture_param(int, kfree_mult, 1, "Multiple of kfree_obj size to allocate.");
    torture_param(int, kfree_by_call_rcu, 0, "Use call_rcu() to emulate kfree_rcu()?");
    static char *scale_type = "rcu";
    module_param(scale_type, charp, 0444);
    MODULE_PARM_DESC(scale_type, "Type of RCU to scalability-test (rcu, srcu, ...)");
// Structure definitions for custom fixed-per-task allocator.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct writer_mblock {
    pub wmb_rh: rcu_head,
    pub wmb_node: llist_node,
    pub wmb_wfl: *mut writer_freelist,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct writer_freelist {
    pub ws_lhg: llist_head,
    pub ws_inflight: core::sync::atomic::AtomicI32,
    pub ws_lhp: llist_head ____cacheline_internodealigned_in_smp,
    pub ws_mblocks: *mut writer_mblock,
}

    static int nrealreaders;
    static int nrealwriters;
    static int nrealexp;
    static struct task_struct **writer_tasks;
    static struct task_struct **reader_tasks;
    static struct task_struct **exp_tasks;
    static u64 **writer_durations;
    static bool *writer_done;
    static struct writer_freelist *writer_freelists;
    static int *writer_n_durations;
    static atomic_t n_rcu_scale_reader_started;
    static atomic_t n_rcu_scale_writer_started;
    static atomic_t n_rcu_scale_writer_finished;
    static u64 t_rcu_scale_writer_started;
    static u64 t_rcu_scale_writer_finished;
    static unsigned long b_rcu_gp_test_started;
    static unsigned long b_rcu_gp_test_finished;
pub const MAX_MEAS: c_int = 10000;
pub const MIN_MEAS: c_int = 100;
//
// Operations vector for selecting different types of tests.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_scale_ops {
    pub ptype: c_int,
    pub (*init)(void): *mut c_void,
    pub (*cleanup)(void): *mut c_void,
    pub (*readlock)(void): *mut c_int,
    pub idx): *mut *mut void (readunlock)(int,
    pub (*get_gp_seq)(void): *mut c_ulong,
    pub old): *mut *mut unsigned long (gp_diff)(unsigned long new, unsigned long,
    pub (*exp_completed)(void): *mut c_ulong,
    pub func): *mut *mut *mut void (async)(struct rcu_head head, rcu_callback_t,
    pub (*gp_barrier)(void): *mut c_void,
    pub (*sync)(void): *mut c_void,
    pub (*exp_sync)(void): *mut c_void,
    pub (*rso_gp_kthread)(void): *mut task_struct,
    pub (*stats)(void): *mut c_void,
    pub name: *const c_char,
}

    static struct rcu_scale_ops *cur_ops;
//
// Definitions for rcu scalability testing.
//
#[no_mangle]
unsafe extern "C" fn rcu_scale_read_lock(__acquires(RCU: void)) -> c_int {
    static int rcu_scale_read_lock(void) __acquires(RCU)
    {
    rcu_read_lock();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rcu_scale_read_unlock(__releases(RCU: int idx)) {
    static void rcu_scale_read_unlock(int idx) __releases(RCU)
    {
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn rcu_no_completed() -> unsigned long __maybe_unused {
    static unsigned long __maybe_unused rcu_no_completed(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rcu_sync_scale_init() {
    static void rcu_sync_scale_init(void)
    {
    }
    static struct rcu_scale_ops rcu_ops = {
    .ptype		= RCU_FLAVOR,
    .init		= rcu_sync_scale_init,
    .readlock	= rcu_scale_read_lock,
    .readunlock	= rcu_scale_read_unlock,
    .get_gp_seq	= rcu_get_gp_seq,
    .gp_diff	= rcu_seq_diff,
    .exp_completed	= rcu_exp_batches_completed,
    .async		= call_rcu_hurry,
    .gp_barrier	= rcu_barrier,
    .sync		= synchronize_rcu,
    .exp_sync	= synchronize_rcu_expedited,
    .name		= "rcu"
    };
//
// Definitions for srcu scalability testing.
//
    DEFINE_STATIC_SRCU(srcu_ctl_scale);
    static struct srcu_struct *srcu_ctlp = &srcu_ctl_scale;
#[no_mangle]
unsafe extern "C" fn srcu_scale_read_lock(__acquires(srcu_ctlp: void)) -> c_int {
    static int srcu_scale_read_lock(void) __acquires(srcu_ctlp)
    {
    return srcu_read_lock(srcu_ctlp);
    }
#[no_mangle]
unsafe extern "C" fn srcu_scale_read_unlock(__releases(srcu_ctlp: int idx)) {
    static void srcu_scale_read_unlock(int idx) __releases(srcu_ctlp)
    {
    srcu_read_unlock(srcu_ctlp, idx);
    }
#[no_mangle]
unsafe extern "C" fn srcu_scale_completed() -> c_ulong {
    static unsigned long srcu_scale_completed(void)
    {
    return srcu_batches_completed(srcu_ctlp);
    }
#[no_mangle]
unsafe extern "C" fn srcu_call_rcu(head: *mut rcu_head, func: rcu_callback_t) {
    static void srcu_call_rcu(struct rcu_head *head, rcu_callback_t func)
    {
    call_srcu(srcu_ctlp, head, func);
    }
#[no_mangle]
unsafe extern "C" fn srcu_rcu_barrier() {
    static void srcu_rcu_barrier(void)
    {
    srcu_barrier(srcu_ctlp);
    }
#[no_mangle]
unsafe extern "C" fn srcu_scale_synchronize() {
    static void srcu_scale_synchronize(void)
    {
    synchronize_srcu(srcu_ctlp);
    }
#[no_mangle]
unsafe extern "C" fn srcu_scale_stats() {
    static void srcu_scale_stats(void)
    {
    srcu_torture_stats_print(srcu_ctlp, scale_type, SCALE_FLAG);
    }
#[no_mangle]
unsafe extern "C" fn srcu_scale_synchronize_expedited() {
    static void srcu_scale_synchronize_expedited(void)
    {
    synchronize_srcu_expedited(srcu_ctlp);
    }
    static struct rcu_scale_ops srcu_ops = {
    .ptype		= SRCU_FLAVOR,
    .init		= rcu_sync_scale_init,
    .readlock	= srcu_scale_read_lock,
    .readunlock	= srcu_scale_read_unlock,
    .get_gp_seq	= srcu_scale_completed,
    .gp_diff	= rcu_seq_diff,
    .exp_completed	= srcu_scale_completed,
    .async		= srcu_call_rcu,
    .gp_barrier	= srcu_rcu_barrier,
    .sync		= srcu_scale_synchronize,
    .exp_sync	= srcu_scale_synchronize_expedited,
    .stats		= srcu_scale_stats,
    .name		= "srcu"
    };
    static struct srcu_struct srcud;
#[no_mangle]
unsafe extern "C" fn srcu_sync_scale_init() {
    static void srcu_sync_scale_init(void)
    {
    srcu_ctlp = &srcud;
    init_srcu_struct(srcu_ctlp);
    }
#[no_mangle]
unsafe extern "C" fn srcu_sync_scale_cleanup() {
    static void srcu_sync_scale_cleanup(void)
    {
    cleanup_srcu_struct(srcu_ctlp);
    }
    static struct rcu_scale_ops srcud_ops = {
    .ptype		= SRCU_FLAVOR,
    .init		= srcu_sync_scale_init,
    .cleanup	= srcu_sync_scale_cleanup,
    .readlock	= srcu_scale_read_lock,
    .readunlock	= srcu_scale_read_unlock,
    .get_gp_seq	= srcu_scale_completed,
    .gp_diff	= rcu_seq_diff,
    .exp_completed	= srcu_scale_completed,
    .async		= srcu_call_rcu,
    .gp_barrier	= srcu_rcu_barrier,
    .sync		= srcu_scale_synchronize,
    .exp_sync	= srcu_scale_synchronize_expedited,
    .stats		= srcu_scale_stats,
    .name		= "srcud"
    };

//
// Definitions for RCU-tasks scalability testing.
//
#[no_mangle]
unsafe extern "C" fn tasks_scale_read_lock() -> c_int {
    static int tasks_scale_read_lock(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tasks_scale_read_unlock(idx: c_int) {
    static void tasks_scale_read_unlock(int idx)
    {
    }
#[no_mangle]
unsafe extern "C" fn rcu_tasks_scale_stats() {
    static void rcu_tasks_scale_stats(void)
    {
    rcu_tasks_torture_stats_print(scale_type, SCALE_FLAG);
    }
    static struct rcu_scale_ops tasks_ops = {
    .ptype		= RCU_TASKS_FLAVOR,
    .init		= rcu_sync_scale_init,
    .readlock	= tasks_scale_read_lock,
    .readunlock	= tasks_scale_read_unlock,
    .get_gp_seq	= rcu_no_completed,
    .gp_diff	= rcu_seq_diff,
    .async		= call_rcu_tasks,
    .gp_barrier	= rcu_barrier_tasks,
    .sync		= synchronize_rcu_tasks,
    .exp_sync	= synchronize_rcu_tasks,
    .rso_gp_kthread	= get_rcu_tasks_gp_kthread,
    .stats		= IS_ENABLED(CONFIG_TINY_RCU) ? core::ptr::null_mut() : rcu_tasks_scale_stats,
    .name		= "tasks"
    };

// Macro flag: #define TASKS_OPS

//
// Definitions for RCU-tasks-rude scalability testing.
//
#[no_mangle]
unsafe extern "C" fn tasks_rude_scale_read_lock() -> c_int {
    static int tasks_rude_scale_read_lock(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tasks_rude_scale_read_unlock(idx: c_int) {
    static void tasks_rude_scale_read_unlock(int idx)
    {
    }
#[no_mangle]
unsafe extern "C" fn rcu_tasks_rude_scale_stats() {
    static void rcu_tasks_rude_scale_stats(void)
    {
    rcu_tasks_rude_torture_stats_print(scale_type, SCALE_FLAG);
    }
    static struct rcu_scale_ops tasks_rude_ops = {
    .ptype		= RCU_TASKS_RUDE_FLAVOR,
    .init		= rcu_sync_scale_init,
    .readlock	= tasks_rude_scale_read_lock,
    .readunlock	= tasks_rude_scale_read_unlock,
    .get_gp_seq	= rcu_no_completed,
    .gp_diff	= rcu_seq_diff,
    .sync		= synchronize_rcu_tasks_rude,
    .exp_sync	= synchronize_rcu_tasks_rude,
    .rso_gp_kthread	= get_rcu_tasks_rude_gp_kthread,
    .stats		= IS_ENABLED(CONFIG_TINY_RCU) ? core::ptr::null_mut() : rcu_tasks_rude_scale_stats,
    .name		= "tasks-rude"
    };

// Macro flag: #define TASKS_RUDE_OPS

//
// Definitions for RCU-tasks-trace scalability testing.
//
#[no_mangle]
unsafe extern "C" fn tasks_trace_scale_read_lock() -> c_int {
    static int tasks_trace_scale_read_lock(void)
    {
    rcu_read_lock_trace();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tasks_trace_scale_read_unlock(idx: c_int) {
    static void tasks_trace_scale_read_unlock(int idx)
    {
    rcu_read_unlock_trace();
    }
    static struct rcu_scale_ops tasks_tracing_ops = {
    .ptype		= RCU_TASKS_FLAVOR,
    .init		= rcu_sync_scale_init,
    .readlock	= tasks_trace_scale_read_lock,
    .readunlock	= tasks_trace_scale_read_unlock,
    .get_gp_seq	= rcu_no_completed,
    .gp_diff	= rcu_seq_diff,
    .async		= call_rcu_tasks_trace,
    .gp_barrier	= rcu_barrier_tasks_trace,
    .sync		= synchronize_rcu_tasks_trace,
    .exp_sync	= synchronize_rcu_tasks_trace,
    .name		= "tasks-tracing"
    };

// Macro flag: #define TASKS_TRACING_OPS

#[no_mangle]
unsafe extern "C" fn rcuscale_seq_diff(new: c_ulong, old: c_ulong) -> c_ulong {
    static unsigned long rcuscale_seq_diff(unsigned long new, unsigned long old)
    {
    if (!cur_ops.gp_diff)
    return new - old;
    return cur_ops.gp_diff(new, old);
    }
//
// If scalability tests complete, wait for shutdown to commence.
//
#[no_mangle]
unsafe extern "C" fn rcu_scale_wait_shutdown() {
    static void rcu_scale_wait_shutdown(void)
    {
    cond_resched_tasks_rcu_qs();
    if (atomic_read(&n_rcu_scale_writer_finished) < nrealwriters)
    return;
    while (!torture_must_stop())
    schedule_timeout_uninterruptible(1);
    }
//
// RCU scalability reader kthread.  Repeatedly does empty RCU read-side
// critical section, minimizing update-side interference.  However, the
// point of this test is not to evaluate reader scalability, but instead
// to serve as a test load for update-side scalability testing.
//
    static int
    rcu_scale_reader(void *arg)
    {
    unsigned long flags;
    int idx;
    let mut me: c_long = (long)arg;
    VERBOSE_SCALEOUT_STRING("rcu_scale_reader task started");
    set_cpus_allowed_ptr(current, cpumask_of(me % nr_cpu_ids));
    set_user_nice(current, MAX_NICE);
    atomic_inc(&n_rcu_scale_reader_started);
    do {
    local_irq_save(flags);
    idx = cur_ops.readlock();
    cur_ops.readunlock(idx);
    local_irq_restore(flags);
    rcu_scale_wait_shutdown();
    } while (!torture_must_stop());
    torture_kthread_stopping("rcu_scale_reader");
    return 0;
    }
//
// RCU expedited GP kthread.  Repeatedly invokes expedited grace periods
// to generate concurrent expedited GP load while the normal-GP writers
// are being measured.  This allows measuring the benefit of callbacks
// that can piggyback on expedited grace periods.
//
    static int
    rcu_scale_exp(void *arg)
    {
    let mut me: c_long = (long)arg;
    VERBOSE_SCALEOUT_STRING("rcu_scale_exp task started");
    set_cpus_allowed_ptr(current, cpumask_of(me % nr_cpu_ids));
    set_user_nice(current, MIN_NICE);
    if (holdoff)
    schedule_timeout_idle(holdoff * HZ);
    do {
    if (exp_interval)
    udelay(exp_interval);
    cur_ops.exp_sync();
    rcu_scale_wait_shutdown();
    } while (!torture_must_stop());
    torture_kthread_stopping("rcu_scale_exp");
    return 0;
    }
//
// Allocate a writer_mblock structure for the specified rcu_scale_writer
// task.
//
    static struct writer_mblock *rcu_scale_alloc(long me)
    {
    struct llist_node *llnp;
    struct writer_freelist *wflp;
    struct writer_mblock *wmbp;
    if (WARN_ON_ONCE(!writer_freelists))
    return core::ptr::null_mut();
    wflp = &writer_freelists[me];
    if (llist_empty(&wflp.ws_lhp)) {
// ->ws_lhp is private to its rcu_scale_writer task.
    wmbp = container_of(llist_del_all(&wflp.ws_lhg), struct writer_mblock, wmb_node);
    wflp.ws_lhp.first = &wmbp.wmb_node;
    }
    llnp = llist_del_first(&wflp.ws_lhp);
    if (!llnp)
    return core::ptr::null_mut();
    return container_of(llnp, struct writer_mblock, wmb_node);
    }
//
// Free a writer_mblock structure to its rcu_scale_writer task.
//
#[no_mangle]
unsafe extern "C" fn rcu_scale_free(wmbp: *mut writer_mblock) {
    static void rcu_scale_free(struct writer_mblock *wmbp)
    {
    struct writer_freelist *wflp;
    if (!wmbp)
    return;
    wflp = wmbp.wmb_wfl;
    llist_add(&wmbp.wmb_node, &wflp.ws_lhg);
    }
//
// Callback function for asynchronous grace periods from rcu_scale_writer().
//
#[no_mangle]
unsafe extern "C" fn rcu_scale_async_cb(rhp: *mut rcu_head) {
    static void rcu_scale_async_cb(struct rcu_head *rhp)
    {
    struct writer_mblock *wmbp = container_of(rhp, struct writer_mblock, wmb_rh);
    struct writer_freelist *wflp = wmbp.wmb_wfl;
    atomic_dec(&wflp.ws_inflight);
    rcu_scale_free(wmbp);
    }
    static void rcu_scale_cleanup(void);
//
// RCU scale writer kthread.  Repeatedly does a grace period.
//
    static int
    rcu_scale_writer(void *arg)
    {
    let mut i: c_int = 0;
    int i_max;
    unsigned long jdone;
    let mut me: c_long = (long)arg;
    let mut selfreport: bool = false;
    let mut started: bool = false, done = false, alldone = false;
    u64 t;
    DEFINE_TORTURE_RANDOM(tr);
    u64 *wdp;
    u64 *wdpp = writer_durations[me];
    struct writer_freelist *wflp = &writer_freelists[me];
    struct writer_mblock *wmbp = core::ptr::null_mut();
    VERBOSE_SCALEOUT_STRING("rcu_scale_writer task started");
    WARN_ON(!wdpp);
    set_cpus_allowed_ptr(current, cpumask_of(me % nr_cpu_ids));
    current.flags |= PF_NO_SETAFFINITY;
    sched_set_fifo_low(current);
    if (holdoff)
    schedule_timeout_idle(holdoff * HZ);
//
// Wait until rcu_end_inkernel_boot() is called for normal GP tests
// so that RCU is not always expedited for normal GP tests.
// The system_state test is approximate, but works well in practice.
//
    while (!gp_exp && system_state != SYSTEM_RUNNING)
    schedule_timeout_uninterruptible(1);
    t = ktime_get_mono_fast_ns();
    if (atomic_inc_return(&n_rcu_scale_writer_started) >= nrealwriters) {
    t_rcu_scale_writer_started = t;
    if (gp_exp) {
    b_rcu_gp_test_started =
    cur_ops.exp_completed() / 2;
    } else {
    b_rcu_gp_test_started = cur_ops.get_gp_seq();
    }
    }
    jdone = jiffies + minruntime * HZ;
    do {
    let mut gp_succeeded: bool = false;
    if (writer_holdoff)
    udelay(writer_holdoff);
    if (writer_holdoff_jiffies)
    schedule_timeout_idle(torture_random(&tr) % writer_holdoff_jiffies + 1);
    wdp = &wdpp[i];
// wdp = ktime_get_mono_fast_ns();
    if (gp_async && !WARN_ON_ONCE(!cur_ops.async)) {
    if (!wmbp)
    wmbp = rcu_scale_alloc(me);
    if (wmbp && atomic_read(&wflp.ws_inflight) < gp_async_max) {
    atomic_inc(&wflp.ws_inflight);
    cur_ops.async(&wmbp.wmb_rh, rcu_scale_async_cb);
    wmbp = core::ptr::null_mut();
    gp_succeeded = true;
    } else if (!kthread_should_stop()) {
    cur_ops.gp_barrier();
    } else {
    rcu_scale_free(wmbp); /* Because we are stopping. */
    wmbp = core::ptr::null_mut();
    }
    } else if (gp_exp) {
    cur_ops.exp_sync();
    gp_succeeded = true;
    } else {
    cur_ops.sync();
    gp_succeeded = true;
    }
    t = ktime_get_mono_fast_ns();
// wdp = t - *wdp;
    i_max = i;
    if (!started &&
    atomic_read(&n_rcu_scale_writer_started) >= nrealwriters)
    started = true;
    if (!done && i >= MIN_MEAS && time_after(jiffies, jdone)) {
    done = true;
    WRITE_ONCE(writer_done[me], true);
    sched_set_normal(current, 0);
    pr_alert("%s%s rcu_scale_writer %ld has %d measurements\n",
    scale_type, SCALE_FLAG, me, MIN_MEAS);
    if (atomic_inc_return(&n_rcu_scale_writer_finished) >=
    nrealwriters) {
    schedule_timeout_interruptible(10);
    rcu_ftrace_dump(DUMP_ALL);
    SCALEOUT_STRING("Test complete");
    t_rcu_scale_writer_finished = t;
    if (gp_exp) {
    b_rcu_gp_test_finished =
    cur_ops.exp_completed() / 2;
    } else {
    b_rcu_gp_test_finished =
    cur_ops.get_gp_seq();
    }
    if (shutdown_secs) {
    writer_tasks[me] = core::ptr::null_mut();
    smp_mb(); /* Assign before wake. */
    rcu_scale_cleanup();
    kernel_power_off();
    }
    }
    }
    if (done && !alldone &&
    atomic_read(&n_rcu_scale_writer_finished) >= nrealwriters)
    alldone = true;
    if (done && !alldone && time_after(jiffies, jdone + HZ * 60)) {
    static atomic_t dumped;
    int i;
    if (!atomic_xchg(&dumped, 1)) {
    for (i = 0; i < nrealwriters; i++) {
    if (writer_done[i])
    continue;
    pr_info("%s: Task %ld flags writer %d:\n", __func__, me, i);
    sched_show_task(writer_tasks[i]);
    }
    if (cur_ops.stats)
    cur_ops.stats();
    }
    }
    if (!selfreport && time_after(jiffies, jdone + HZ * (70 + me))) {
    pr_info("%s: Writer %ld self-report: started %d done %d/%d.%d i %d jdone %lu.\n",
    __func__, me, started, done, writer_done[me], atomic_read(&n_rcu_scale_writer_finished), i, jiffies - jdone);
    selfreport = true;
    }
    if (gp_succeeded && started && !alldone && i < MAX_MEAS - 1)
    i++;
    rcu_scale_wait_shutdown();
    } while (!torture_must_stop());
    if (gp_async && cur_ops.async) {
    rcu_scale_free(wmbp);
    cur_ops.gp_barrier();
    }
    writer_n_durations[me] = i_max + 1;
    torture_kthread_stopping("rcu_scale_writer");
    return 0;
    }
    static void
    rcu_scale_print_module_parms(struct rcu_scale_ops *cur_ops, const char *tag)
    {
    pr_alert("%s" SCALE_FLAG
    "--- %s: gp_async=%d gp_async_max=%d gp_exp=%d holdoff=%d minruntime=%d nreaders=%d nwriters=%d nexp=%d exp_interval=%d writer_holdoff=%d writer_holdoff_jiffies=%d verbose=%d shutdown_secs=%d\n",
    scale_type, tag, gp_async, gp_async_max, gp_exp, holdoff,
    minruntime, nrealreaders, nrealwriters, nrealexp, exp_interval,
    writer_holdoff, writer_holdoff_jiffies, verbose, shutdown_secs);
    }
//
// Return the number if non-negative.  If -1, the number of CPUs.
// If less than -1, that much less than the number of CPUs, but
// at least one.
//
#[no_mangle]
unsafe extern "C" fn compute_real(n: c_int) -> c_int {
    static int compute_real(int n)
    {
    int nr;
    if (n >= 0) {
    nr = n;
    } else {
    nr = num_online_cpus() + 1 + n;
    if (nr <= 0)
    nr = 1;
    }
    return nr;
    }
//
// kfree_rcu() scalability tests: Start a kfree_rcu() loop on all CPUs for number
// of iterations and measure total time and number of GP for all iterations to complete.
//
    torture_param(int, kfree_nthreads, -1, "Number of threads running loops of kfree_rcu().");
    torture_param(int, kfree_alloc_num, 8000, "Number of allocations and frees done in an iteration.");
    torture_param(int, kfree_loops, 10, "Number of loops doing kfree_alloc_num allocations and frees.");
    torture_param(bool, kfree_rcu_test_double, false, "Do we run a kfree_rcu() double-argument scale test?");
    torture_param(bool, kfree_rcu_test_single, false, "Do we run a kfree_rcu() single-argument scale test?");
    static struct task_struct **kfree_reader_tasks;
    static int kfree_nrealthreads;
    static atomic_t n_kfree_scale_thread_started;
    static atomic_t n_kfree_scale_thread_ended;
    static struct task_struct *kthread_tp;
    static u64 kthread_stime;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfree_obj {
    pub kfree_obj: [c_char; 8],
    pub rh: rcu_head,
}

// Used if doing RCU-kfree'ing via call_rcu().
#[no_mangle]
unsafe extern "C" fn kfree_call_rcu(rh: *mut rcu_head) {
    static void kfree_call_rcu(struct rcu_head *rh)
    {
    struct kfree_obj *obj = container_of(rh, struct kfree_obj, rh);
    kfree(obj);
    }
    static void kfree_scale_cleanup(void);
    static int
    kfree_scale_thread(void *arg)
    {
    int i, loop = 0;
    let mut me: c_long = (long)arg;
    struct kfree_obj *alloc_ptr;
    u64 start_time, end_time;
    long long mem_begin, mem_during = 0;
    bool kfree_rcu_test_both;
    DEFINE_TORTURE_RANDOM(tr);
    VERBOSE_SCALEOUT_STRING("kfree_scale_thread task started");
    set_cpus_allowed_ptr(current, cpumask_of(me % nr_cpu_ids));
    set_user_nice(current, MAX_NICE);
    kfree_rcu_test_both = (kfree_rcu_test_single == kfree_rcu_test_double);
    start_time = ktime_get_mono_fast_ns();
    if (atomic_inc_return(&n_kfree_scale_thread_started) >= kfree_nrealthreads) {
    if (gp_exp)
    b_rcu_gp_test_started = cur_ops.exp_completed() / 2;
    else
    b_rcu_gp_test_started = cur_ops.get_gp_seq();
    }
    do {
    if (!mem_during) {
    mem_during = mem_begin = si_mem_available();
    } else if (loop % (kfree_loops / 4) == 0) {
    mem_during = (mem_during + si_mem_available()) / 2;
    }
    for (i = 0; i < kfree_alloc_num; i++) {
    alloc_ptr = kzalloc_objs(struct kfree_obj, kfree_mult);
    if (!alloc_ptr)
    return -ENOMEM;
    if (kfree_by_call_rcu) {
    call_rcu(&(alloc_ptr.rh), kfree_call_rcu);
    continue;
    }
// By default kfree_rcu_test_single and kfree_rcu_test_double are
// initialized to false. If both have the same value (false or true)
// both are randomly tested, otherwise only the one with value true
// is tested.
    if ((kfree_rcu_test_single && !kfree_rcu_test_double) ||
    (kfree_rcu_test_both && torture_random(&tr) & 0x800))
    kfree_rcu_mightsleep(alloc_ptr);
    else
    kfree_rcu(alloc_ptr, rh);
    }
    cond_resched();
    } while (!torture_must_stop() && ++loop < kfree_loops);
    if (atomic_inc_return(&n_kfree_scale_thread_ended) >= kfree_nrealthreads) {
    end_time = ktime_get_mono_fast_ns();
    if (gp_exp)
    b_rcu_gp_test_finished = cur_ops.exp_completed() / 2;
    else
    b_rcu_gp_test_finished = cur_ops.get_gp_seq();
    pr_alert("Total time taken by all kfree'ers: %llu ns, loops: %d, batches: %ld, memory footprint: %lldMB\n",
    (unsigned long long)(end_time - start_time), kfree_loops,
    rcuscale_seq_diff(b_rcu_gp_test_finished, b_rcu_gp_test_started),
    PAGES_TO_MB(mem_begin - mem_during));
    if (shutdown_secs) {
    kfree_reader_tasks[me] = core::ptr::null_mut();
    smp_mb(); /* Assign before wake. */
    kfree_scale_cleanup();
    kernel_power_off();
    }
    }
    torture_kthread_stopping("kfree_scale_thread");
    return 0;
    }
    static void
    kfree_scale_cleanup(void)
    {
    int i;
    if (torture_cleanup_begin())
    return;
    if (exp_tasks) {
    for (i = 0; i < nrealexp; i++)
    torture_stop_kthread(rcu_scale_exp, exp_tasks[i]);
    kfree(exp_tasks);
    exp_tasks = core::ptr::null_mut();
    }
    if (kfree_reader_tasks) {
    for (i = 0; i < kfree_nrealthreads; i++)
    torture_stop_kthread(kfree_scale_thread,
    kfree_reader_tasks[i]);
    kfree(kfree_reader_tasks);
    kfree_reader_tasks = core::ptr::null_mut();
    }
    torture_cleanup_end();
    }
// Used if doing RCU-kfree'ing via call_rcu().
    static unsigned long jiffies_at_lazy_cb;
    static struct rcu_head lazy_test1_rh;
    static int rcu_lazy_test1_cb_called;
#[no_mangle]
unsafe extern "C" fn call_rcu_lazy_test1(rh: *mut rcu_head) {
    static void call_rcu_lazy_test1(struct rcu_head *rh)
    {
    jiffies_at_lazy_cb = jiffies;
    WRITE_ONCE(rcu_lazy_test1_cb_called, 1);
    }
    static int __init
    kfree_scale_init(void)
    {
    let mut firsterr: c_int = 0;
    long i;
    unsigned long jif_start;
    unsigned long orig_jif;
    pr_alert("%s" SCALE_FLAG
    "--- kfree_rcu_test: kfree_mult=%d kfree_by_call_rcu=%d kfree_nthreads=%d kfree_alloc_num=%d kfree_loops=%d kfree_rcu_test_double=%d kfree_rcu_test_single=%d\n",
    scale_type, kfree_mult, kfree_by_call_rcu, kfree_nthreads, kfree_alloc_num, kfree_loops, kfree_rcu_test_double, kfree_rcu_test_single);
// Also, do a quick self-test to ensure laziness is as much as
// expected.
    if (kfree_by_call_rcu && !IS_ENABLED(CONFIG_RCU_LAZY)) {
    pr_alert("CONFIG_RCU_LAZY is disabled, falling back to kfree_rcu() for delayed RCU kfree'ing\n");
    kfree_by_call_rcu = 0;
    }
    if (kfree_by_call_rcu) {
// do a test to check the timeout.
    orig_jif = rcu_get_jiffies_lazy_flush();
    rcu_set_jiffies_lazy_flush(2 * HZ);
    rcu_barrier();
    jif_start = jiffies;
    jiffies_at_lazy_cb = 0;
    call_rcu(&lazy_test1_rh, call_rcu_lazy_test1);
    smp_cond_load_relaxed(&rcu_lazy_test1_cb_called, VAL == 1);
    rcu_set_jiffies_lazy_flush(orig_jif);
    if (WARN_ON_ONCE(jiffies_at_lazy_cb - jif_start < 2 * HZ)) {
    pr_alert("ERROR: call_rcu() CBs are not being lazy as expected!\n");
    firsterr = -1;
    goto unwind;
    }
    if (WARN_ON_ONCE(jiffies_at_lazy_cb - jif_start > 3 * HZ)) {
    pr_alert("ERROR: call_rcu() CBs are being too lazy!\n");
    firsterr = -1;
    goto unwind;
    }
    }
    kfree_nrealthreads = compute_real(kfree_nthreads);
// Start up the kthreads.
    if (shutdown_secs) {
    firsterr = torture_shutdown_init(shutdown_secs, kfree_scale_cleanup);
    if (torture_init_error(firsterr))
    goto unwind;
    }
    pr_alert("kfree object size=%zu, kfree_by_call_rcu=%d\n",
    kfree_mult * sizeof(struct kfree_obj),
    kfree_by_call_rcu);
    kfree_reader_tasks = kzalloc_objs(kfree_reader_tasks[0],
    kfree_nrealthreads);
    if (kfree_reader_tasks == core::ptr::null_mut()) {
    firsterr = -ENOMEM;
    goto unwind;
    }
    for (i = 0; i < kfree_nrealthreads; i++) {
    firsterr = torture_create_kthread(kfree_scale_thread, (void *)i,
    kfree_reader_tasks[i]);
    if (torture_init_error(firsterr))
    goto unwind;
    }
    if (nrealexp > 0 && cur_ops.exp_sync) {
    exp_tasks = kzalloc_objs(exp_tasks[0], nrealexp);
    if (!exp_tasks) {
    SCALEOUT_ERRSTRING("out of memory");
    firsterr = -ENOMEM;
    goto unwind;
    }
    for (i = 0; i < nrealexp; i++) {
    firsterr = torture_create_kthread(rcu_scale_exp,
    (void *)i,
    exp_tasks[i]);
    if (torture_init_error(firsterr))
    goto unwind;
    }
    }
    while (atomic_read(&n_kfree_scale_thread_started) < kfree_nrealthreads)
    schedule_timeout_uninterruptible(1);
    torture_init_end();
    return 0;
    unwind:
    torture_init_end();
    kfree_scale_cleanup();
    return firsterr;
    }
    static void
    rcu_scale_cleanup(void)
    {
    int i;
    int j;
    let mut ngps: c_int = 0;
    u64 *wdp;
    u64 *wdpp;
//
// Would like warning at start, but everything is expedited
// during the mid-boot phase, so have to wait till the end.
//
    if (rcu_gp_is_expedited() && !rcu_gp_is_normal() && !gp_exp)
    SCALEOUT_ERRSTRING("All grace periods expedited, no normal ones to measure!");
    if (rcu_gp_is_normal() && gp_exp)
    SCALEOUT_ERRSTRING("All grace periods normal, no expedited ones to measure!");
    if (gp_exp && gp_async)
    SCALEOUT_ERRSTRING("No expedited async GPs, so went with async!");
// If built-in, just report all of the GP kthread's CPU time.
    if (IS_BUILTIN(CONFIG_RCU_SCALE_TEST) && !kthread_tp && cur_ops.rso_gp_kthread)
    kthread_tp = cur_ops.rso_gp_kthread();
    if (kthread_tp) {
    u32 ns;
    u64 us;
    kthread_stime = kthread_tp.stime - kthread_stime;
    us = div_u64_rem(kthread_stime, 1000, &ns);
    pr_info("rcu_scale: Grace-period kthread CPU time: %llu.%03u us\n", us, ns);
    show_rcu_gp_kthreads();
    }
    if (kfree_rcu_test) {
    kfree_scale_cleanup();
    return;
    }
    if (torture_cleanup_begin())
    return;
    if (!cur_ops) {
    torture_cleanup_end();
    return;
    }
    if (exp_tasks) {
    for (i = 0; i < nrealexp; i++)
    torture_stop_kthread(rcu_scale_exp, exp_tasks[i]);
    kfree(exp_tasks);
    exp_tasks = core::ptr::null_mut();
    }
    if (reader_tasks) {
    for (i = 0; i < nrealreaders; i++)
    torture_stop_kthread(rcu_scale_reader,
    reader_tasks[i]);
    kfree(reader_tasks);
    reader_tasks = core::ptr::null_mut();
    }
    if (writer_tasks) {
    for (i = 0; i < nrealwriters; i++) {
    torture_stop_kthread(rcu_scale_writer,
    writer_tasks[i]);
    if (!writer_n_durations)
    continue;
    j = writer_n_durations[i];
    pr_alert("%s%s writer %d gps: %d\n",
    scale_type, SCALE_FLAG, i, j);
    ngps += j;
    }
    pr_alert("%s%s start: %llu end: %llu duration: %llu gps: %d batches: %ld\n",
    scale_type, SCALE_FLAG,
    t_rcu_scale_writer_started, t_rcu_scale_writer_finished,
    t_rcu_scale_writer_finished -
    t_rcu_scale_writer_started,
    ngps,
    rcuscale_seq_diff(b_rcu_gp_test_finished,
    b_rcu_gp_test_started));
    for (i = 0; i < nrealwriters; i++) {
    if (!writer_durations)
    break;
    if (!writer_n_durations)
    continue;
    wdpp = writer_durations[i];
    if (!wdpp)
    continue;
    for (j = 0; j < writer_n_durations[i]; j++) {
    wdp = &wdpp[j];
    pr_alert("%s%s %4d writer-duration: %5d %llu\n",
    scale_type, SCALE_FLAG,
    i, j, *wdp);
    if (j % 100 == 0)
    schedule_timeout_uninterruptible(1);
    }
    kfree(writer_durations[i]);
    if (writer_freelists) {
    let mut ctr: c_int = 0;
    struct llist_node *llnp;
    struct writer_freelist *wflp = &writer_freelists[i];
    if (wflp.ws_mblocks) {
    llist_for_each(llnp, wflp.ws_lhg.first)
    ctr++;
    llist_for_each(llnp, wflp.ws_lhp.first)
    ctr++;
    WARN_ONCE(ctr != gp_async_max,
    "%s: ctr = %d gp_async_max = %d\n",
    __func__, ctr, gp_async_max);
    kfree(wflp.ws_mblocks);
    }
    }
    }
    kfree(writer_tasks);
    writer_tasks = core::ptr::null_mut();
    kfree(writer_durations);
    writer_durations = core::ptr::null_mut();
    kfree(writer_n_durations);
    writer_n_durations = core::ptr::null_mut();
    kfree(writer_done);
    writer_done = core::ptr::null_mut();
    kfree(writer_freelists);
    writer_freelists = core::ptr::null_mut();
    }
// Do torture-type-specific cleanup operations.
    if (cur_ops.cleanup != core::ptr::null_mut())
    cur_ops.cleanup();
    torture_cleanup_end();
    }
    static int __init
    rcu_scale_init(void)
    {
    let mut firsterr: c_int = 0;
    long i;
    long j;
    static struct rcu_scale_ops *scale_ops[] = {
    &rcu_ops, &srcu_ops, &srcud_ops, TASKS_OPS TASKS_RUDE_OPS TASKS_TRACING_OPS
    };
    if (!torture_init_begin(scale_type, verbose))
    return -EBUSY;
// Process args and announce that the scalability'er is on the job.
    for (i = 0; i < ARRAY_SIZE(scale_ops); i++) {
    cur_ops = scale_ops[i];
    if (strcmp(scale_type, cur_ops.name) == 0)
    break;
    }
    if (i == ARRAY_SIZE(scale_ops)) {
    pr_alert("rcu-scale: invalid scale type: \"%s\"\n", scale_type);
    pr_alert("rcu-scale types:");
    for (i = 0; i < ARRAY_SIZE(scale_ops); i++)
    pr_cont(" %s", scale_ops[i].name);
    pr_cont("\n");
    firsterr = -EINVAL;
    cur_ops = core::ptr::null_mut();
    goto unwind;
    }
    if (cur_ops.init)
    cur_ops.init();
    if (cur_ops.rso_gp_kthread) {
    kthread_tp = cur_ops.rso_gp_kthread();
    if (kthread_tp)
    kthread_stime = kthread_tp.stime;
    }
    nrealexp = nexp;
    if (kfree_rcu_test)
    return kfree_scale_init();
    nrealwriters = compute_real(nwriters);
    nrealreaders = compute_real(nreaders);
    atomic_set(&n_rcu_scale_reader_started, 0);
    atomic_set(&n_rcu_scale_writer_started, 0);
    atomic_set(&n_rcu_scale_writer_finished, 0);
    rcu_scale_print_module_parms(cur_ops, "Start of test");
// Start up the kthreads.
    if (shutdown_secs) {
    firsterr = torture_shutdown_init(shutdown_secs, rcu_scale_cleanup);
    if (torture_init_error(firsterr))
    goto unwind;
    }
    reader_tasks = kzalloc_objs(reader_tasks[0], nrealreaders);
    if (reader_tasks == core::ptr::null_mut()) {
    SCALEOUT_ERRSTRING("out of memory");
    firsterr = -ENOMEM;
    goto unwind;
    }
    for (i = 0; i < nrealreaders; i++) {
    firsterr = torture_create_kthread(rcu_scale_reader, (void *)i,
    reader_tasks[i]);
    if (torture_init_error(firsterr))
    goto unwind;
    }
    while (atomic_read(&n_rcu_scale_reader_started) < nrealreaders)
    schedule_timeout_uninterruptible(1);
    if (nrealexp > 0 && cur_ops.exp_sync) {
    exp_tasks = kzalloc_objs(exp_tasks[0], nrealexp);
    if (!exp_tasks) {
    SCALEOUT_ERRSTRING("out of memory");
    firsterr = -ENOMEM;
    goto unwind;
    }
    for (i = 0; i < nrealexp; i++) {
    firsterr = torture_create_kthread(rcu_scale_exp,
    (void *)i,
    exp_tasks[i]);
    if (torture_init_error(firsterr))
    goto unwind;
    }
    }
    writer_tasks = kzalloc_objs(writer_tasks[0], nrealwriters);
    writer_durations = kcalloc(nrealwriters, sizeof(*writer_durations), GFP_KERNEL);
    writer_n_durations = kzalloc_objs(*writer_n_durations, nrealwriters);
    writer_done = kzalloc_objs(writer_done[0], nrealwriters);
    if (gp_async) {
    if (gp_async_max <= 0) {
    pr_warn("%s: gp_async_max = %d must be greater than zero.\n",
    __func__, gp_async_max);
    WARN_ON_ONCE(IS_BUILTIN(CONFIG_RCU_TORTURE_TEST));
    firsterr = -EINVAL;
    goto unwind;
    }
    writer_freelists = kzalloc_objs(writer_freelists[0],
    nrealwriters);
    }
    if (!writer_tasks || !writer_durations || !writer_n_durations || !writer_done ||
    (gp_async && !writer_freelists)) {
    SCALEOUT_ERRSTRING("out of memory");
    firsterr = -ENOMEM;
    goto unwind;
    }
    for (i = 0; i < nrealwriters; i++) {
    writer_durations[i] =
    kcalloc(MAX_MEAS, sizeof(*writer_durations[i]),
    GFP_KERNEL);
    if (!writer_durations[i]) {
    firsterr = -ENOMEM;
    goto unwind;
    }
    if (writer_freelists) {
    struct writer_freelist *wflp = &writer_freelists[i];
    init_llist_head(&wflp.ws_lhg);
    init_llist_head(&wflp.ws_lhp);
    wflp.ws_mblocks = kzalloc_objs(wflp.ws_mblocks[0],
    gp_async_max);
    if (!wflp.ws_mblocks) {
    firsterr = -ENOMEM;
    goto unwind;
    }
    for (j = 0; j < gp_async_max; j++) {
    struct writer_mblock *wmbp = &wflp.ws_mblocks[j];
    wmbp.wmb_wfl = wflp;
    llist_add(&wmbp.wmb_node, &wflp.ws_lhp);
    }
    }
    firsterr = torture_create_kthread(rcu_scale_writer, (void *)i,
    writer_tasks[i]);
    if (torture_init_error(firsterr))
    goto unwind;
    }
    torture_init_end();
    return 0;
    unwind:
    torture_init_end();
    rcu_scale_cleanup();
    if (shutdown_secs) {
    WARN_ON(!IS_MODULE(CONFIG_RCU_SCALE_TEST));
    kernel_power_off();
    }
    return firsterr;
    }
    module_init(rcu_scale_init);
    module_exit(rcu_scale_cleanup);

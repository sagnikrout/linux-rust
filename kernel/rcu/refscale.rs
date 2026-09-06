//! Automatically rewritten from C to Rust
//! Source: kernel/rcu/refscale.c
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
// Scalability test comparing RCU vs other mechanisms
// for acquiring references on objects.
//
// Copyright (C) Google, 2020.
//
// Author: Joel Fernandes <joel@joelfernandes.org>

    pr_alert("%s" SCALE_FLAG s, scale_type, ## x)

    do { \
    if (verbose) \
    pr_alert("%s" SCALE_FLAG s "\n", scale_type, ## x); \
    } while (0)
    static atomic_t verbose_batch_ctr;

    do {											\
    if (verbose &&									\
    (verbose_batched <= 0 ||							\
    !(atomic_inc_return(&verbose_batch_ctr) % verbose_batched))) {		\
    schedule_timeout_uninterruptible(1);					\
    pr_alert("%s" SCALE_FLAG s "\n", scale_type, ## x);			\
    }										\
    } while (0)

    MODULE_DESCRIPTION("Scalability test for object reference mechanisms");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Joel Fernandes (Google) <joel@joelfernandes.org>");
    static char *scale_type = "rcu";
    module_param(scale_type, charp, 0444);
    MODULE_PARM_DESC(scale_type, "Type of test (rcu, srcu, refcnt, rwsem, rwlock.");
    torture_param(int, verbose, 0, "Enable verbose debugging printk()s");
    torture_param(int, verbose_batched, 0, "Batch verbose debugging printk()s");
// Number of seconds to extend warm-up and cool-down for multiple guest OSes
    torture_param(long, guest_os_delay, 0,
    "Number of seconds to extend warm-up/cool-down for multiple guest OSes.");
// Wait until there are multiple CPUs before starting test.
    torture_param(int, holdoff, IS_BUILTIN(CONFIG_RCU_REF_SCALE_TEST) ? 10 : 0,
    "Holdoff time before test start (s)");
// Number of typesafe_lookup structures, that is, the degree of concurrency.
    torture_param(long, lookup_instances, 0, "Number of typesafe_lookup structures.");
// Number of loops per experiment, all readers execute operations concurrently.
    torture_param(int, loops, 10000, "Number of loops per experiment.");
// Number of readers, with -1 defaulting to about 75% of the CPUs.
    torture_param(int, nreaders, -1, "Number of readers, -1 for 75% of CPUs.");
// Number of runs.
    torture_param(int, nruns, 30, "Number of experiments to run.");
// Reader delay in nanoseconds, 0 for no delay.
    torture_param(int, readdelay, 0, "Read-side delay in nanoseconds.");
// Maximum shutdown delay in seconds, or zero for no shutdown.
    torture_param(int, shutdown_secs, !IS_MODULE(CONFIG_REPRO_TEST) * 300,
    "Shutdown at end of scalability tests or at specified timeout (s).");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reader_task {
    pub task: *mut task_struct,
    pub start_reader: c_int,
    pub wq: wait_queue_head_t,
    pub last_duration_ns: u64,
}

    static struct task_struct *main_task;
    static wait_queue_head_t main_wq;
    static struct reader_task *reader_tasks;
// Number of readers that are part of the current experiment.
    static atomic_t nreaders_exp;
// Use to wait for all threads to start.
    static atomic_t n_init;
    static atomic_t n_started;
    static atomic_t n_warmedup;
    static atomic_t n_cooleddown;
// Track which experiment is currently running.
    static int exp_idx;
// Operations vector for selecting different types of tests.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ref_scale_ops {
    pub (*init)(void): *mut bool,
    pub (*cleanup)(void): *mut c_void,
    pub nloops): *const *const void (readsection)(int,
    pub ndl): *const *const void (delaysection)(int nloops, int udl, int,
    pub enable_irqs: bool,
    pub name: *const c_char,
}

    static const struct ref_scale_ops *cur_ops;
#[no_mangle]
unsafe extern "C" fn un_delay(udl: c_int, ndl: c_int) {
    static void un_delay(const int udl, const int ndl)
    {
    if (udl)
    udelay(udl);
    if (ndl)
    ndelay(ndl);
    }
#[no_mangle]
unsafe extern "C" fn ref_rcu_read_section(nloops: c_int) {
    static void ref_rcu_read_section(const int nloops)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    rcu_read_lock();
    rcu_read_unlock();
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_rcu_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_rcu_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    rcu_read_lock();
    un_delay(udl, ndl);
    rcu_read_unlock();
    }
    }
#[no_mangle]
unsafe extern "C" fn rcu_sync_scale_init() -> bool {
    static bool rcu_sync_scale_init(void)
    {
    return true;
    }
    static const struct ref_scale_ops rcu_ops = {
    .init		= rcu_sync_scale_init,
    .readsection	= ref_rcu_read_section,
    .delaysection	= ref_rcu_delay_section,
    .name		= "rcu"
    };
// Definitions for SRCU ref scale testing.
    DEFINE_STATIC_SRCU(srcu_refctl_scale);
    DEFINE_STATIC_SRCU_FAST(srcu_fast_refctl_scale);
    DEFINE_STATIC_SRCU_FAST_UPDOWN(srcu_fast_updown_refctl_scale);
    static struct srcu_struct *srcu_ctlp = &srcu_refctl_scale;
#[no_mangle]
unsafe extern "C" fn srcu_ref_scale_read_section(nloops: c_int) {
    static void srcu_ref_scale_read_section(const int nloops)
    {
    int i;
    int idx;
    for (i = nloops; i >= 0; i--) {
    idx = srcu_read_lock(srcu_ctlp);
    srcu_read_unlock(srcu_ctlp, idx);
    }
    }
#[no_mangle]
unsafe extern "C" fn srcu_ref_scale_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void srcu_ref_scale_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    int idx;
    for (i = nloops; i >= 0; i--) {
    idx = srcu_read_lock(srcu_ctlp);
    un_delay(udl, ndl);
    srcu_read_unlock(srcu_ctlp, idx);
    }
    }
    static const struct ref_scale_ops srcu_ops = {
    .init		= rcu_sync_scale_init,
    .readsection	= srcu_ref_scale_read_section,
    .delaysection	= srcu_ref_scale_delay_section,
    .name		= "srcu"
    };
#[no_mangle]
unsafe extern "C" fn srcu_fast_sync_scale_init() -> bool {
    static bool srcu_fast_sync_scale_init(void)
    {
    srcu_ctlp = &srcu_fast_refctl_scale;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn srcu_fast_ref_scale_read_section(nloops: c_int) {
    static void srcu_fast_ref_scale_read_section(const int nloops)
    {
    int i;
    struct srcu_ctr __percpu *scp;
    for (i = nloops; i >= 0; i--) {
    scp = srcu_read_lock_fast(srcu_ctlp);
    srcu_read_unlock_fast(srcu_ctlp, scp);
    }
    }
#[no_mangle]
unsafe extern "C" fn srcu_fast_ref_scale_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void srcu_fast_ref_scale_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    struct srcu_ctr __percpu *scp;
    for (i = nloops; i >= 0; i--) {
    scp = srcu_read_lock_fast(srcu_ctlp);
    un_delay(udl, ndl);
    srcu_read_unlock_fast(srcu_ctlp, scp);
    }
    }
    static const struct ref_scale_ops srcu_fast_ops = {
    .init		= srcu_fast_sync_scale_init,
    .readsection	= srcu_fast_ref_scale_read_section,
    .delaysection	= srcu_fast_ref_scale_delay_section,
    .name		= "srcu-fast"
    };
#[no_mangle]
unsafe extern "C" fn srcu_fast_updown_sync_scale_init() -> bool {
    static bool srcu_fast_updown_sync_scale_init(void)
    {
    srcu_ctlp = &srcu_fast_updown_refctl_scale;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn srcu_fast_updown_ref_scale_read_section(nloops: c_int) {
    static void srcu_fast_updown_ref_scale_read_section(const int nloops)
    {
    int i;
    struct srcu_ctr __percpu *scp;
    for (i = nloops; i >= 0; i--) {
    scp = srcu_read_lock_fast_updown(srcu_ctlp);
    srcu_read_unlock_fast_updown(srcu_ctlp, scp);
    }
    }
#[no_mangle]
unsafe extern "C" fn srcu_fast_updown_ref_scale_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void srcu_fast_updown_ref_scale_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    struct srcu_ctr __percpu *scp;
    for (i = nloops; i >= 0; i--) {
    scp = srcu_read_lock_fast_updown(srcu_ctlp);
    un_delay(udl, ndl);
    srcu_read_unlock_fast_updown(srcu_ctlp, scp);
    }
    }
    static const struct ref_scale_ops srcu_fast_updown_ops = {
    .init		= srcu_fast_updown_sync_scale_init,
    .readsection	= srcu_fast_updown_ref_scale_read_section,
    .delaysection	= srcu_fast_updown_ref_scale_delay_section,
    .name		= "srcu-fast-updown"
    };

// Definitions for RCU Tasks ref scale testing: Empty read markers.
// These definitions also work for RCU Rude readers.
#[no_mangle]
unsafe extern "C" fn rcu_tasks_ref_scale_read_section(nloops: c_int) {
    static void rcu_tasks_ref_scale_read_section(const int nloops)
    {
    int i;
    for (i = nloops; i >= 0; i--)
    continue;
    }
#[no_mangle]
unsafe extern "C" fn rcu_tasks_ref_scale_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void rcu_tasks_ref_scale_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    for (i = nloops; i >= 0; i--)
    un_delay(udl, ndl);
    }
    static const struct ref_scale_ops rcu_tasks_ops = {
    .init		= rcu_sync_scale_init,
    .readsection	= rcu_tasks_ref_scale_read_section,
    .delaysection	= rcu_tasks_ref_scale_delay_section,
    .name		= "rcu-tasks"
    };

// Macro flag: #define RCU_TASKS_OPS

// Definitions for RCU Tasks Trace ref scale testing.
#[no_mangle]
unsafe extern "C" fn rcu_trace_ref_scale_read_section(nloops: c_int) {
    static void rcu_trace_ref_scale_read_section(const int nloops)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    rcu_read_lock_trace();
    rcu_read_unlock_trace();
    }
    }
#[no_mangle]
unsafe extern "C" fn rcu_trace_ref_scale_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void rcu_trace_ref_scale_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    rcu_read_lock_trace();
    un_delay(udl, ndl);
    rcu_read_unlock_trace();
    }
    }
    static const struct ref_scale_ops rcu_trace_ops = {
    .init		= rcu_sync_scale_init,
    .readsection	= rcu_trace_ref_scale_read_section,
    .delaysection	= rcu_trace_ref_scale_delay_section,
    .name		= "rcu-trace"
    };

// Macro flag: #define RCU_TRACE_OPS

// Definitions for reference count
    static atomic_t refcnt;
// Definitions acquire-release.
    static DEFINE_PER_CPU(unsigned long, test_acqrel);
#[no_mangle]
unsafe extern "C" fn ref_refcnt_section(nloops: c_int) {
    static void ref_refcnt_section(const int nloops)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    atomic_inc(&refcnt);
    atomic_dec(&refcnt);
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_refcnt_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_refcnt_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    atomic_inc(&refcnt);
    un_delay(udl, ndl);
    atomic_dec(&refcnt);
    }
    }
    static const struct ref_scale_ops refcnt_ops = {
    .init		= rcu_sync_scale_init,
    .readsection	= ref_refcnt_section,
    .delaysection	= ref_refcnt_delay_section,
    .name		= "refcnt"
    };
#[no_mangle]
unsafe extern "C" fn ref_percpuinc_section(nloops: c_int) {
    static void ref_percpuinc_section(const int nloops)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    this_cpu_inc(test_acqrel);
    this_cpu_dec(test_acqrel);
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_percpuinc_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_percpuinc_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    this_cpu_inc(test_acqrel);
    un_delay(udl, ndl);
    this_cpu_dec(test_acqrel);
    }
    }
    static const struct ref_scale_ops percpuinc_ops = {
    .init		= rcu_sync_scale_init,
    .readsection	= ref_percpuinc_section,
    .delaysection	= ref_percpuinc_delay_section,
    .name		= "percpuinc"
    };
// Note that this can lose counts in preemptible kernels.
#[no_mangle]
unsafe extern "C" fn ref_incpercpu_section(nloops: c_int) {
    static void ref_incpercpu_section(const int nloops)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    unsigned long *tap = this_cpu_ptr(&test_acqrel);
    WRITE_ONCE(*tap, READ_ONCE(*tap) + 1);
    WRITE_ONCE(*tap, READ_ONCE(*tap) - 1);
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_incpercpu_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_incpercpu_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    unsigned long *tap = this_cpu_ptr(&test_acqrel);
    WRITE_ONCE(*tap, READ_ONCE(*tap) + 1);
    un_delay(udl, ndl);
    WRITE_ONCE(*tap, READ_ONCE(*tap) - 1);
    }
    }
    static const struct ref_scale_ops incpercpu_ops = {
    .init		= rcu_sync_scale_init,
    .readsection	= ref_incpercpu_section,
    .delaysection	= ref_incpercpu_delay_section,
    .name		= "incpercpu"
    };
#[no_mangle]
unsafe extern "C" fn ref_incpercpupreempt_section(nloops: c_int) {
    static void ref_incpercpupreempt_section(const int nloops)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    unsigned long *tap;
    preempt_disable();
    tap = this_cpu_ptr(&test_acqrel);
    WRITE_ONCE(*tap, READ_ONCE(*tap) + 1);
    WRITE_ONCE(*tap, READ_ONCE(*tap) - 1);
    preempt_enable();
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_incpercpupreempt_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_incpercpupreempt_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    unsigned long *tap;
    preempt_disable();
    tap = this_cpu_ptr(&test_acqrel);
    WRITE_ONCE(*tap, READ_ONCE(*tap) + 1);
    un_delay(udl, ndl);
    WRITE_ONCE(*tap, READ_ONCE(*tap) - 1);
    preempt_enable();
    }
    }
    static const struct ref_scale_ops incpercpupreempt_ops = {
    .init		= rcu_sync_scale_init,
    .readsection	= ref_incpercpupreempt_section,
    .delaysection	= ref_incpercpupreempt_delay_section,
    .name		= "incpercpupreempt"
    };
#[no_mangle]
unsafe extern "C" fn ref_incpercpubh_section(nloops: c_int) {
    static void ref_incpercpubh_section(const int nloops)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    unsigned long *tap;
    local_bh_disable();
    tap = this_cpu_ptr(&test_acqrel);
    WRITE_ONCE(*tap, READ_ONCE(*tap) + 1);
    WRITE_ONCE(*tap, READ_ONCE(*tap) - 1);
    local_bh_enable();
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_incpercpubh_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_incpercpubh_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    unsigned long *tap;
    local_bh_disable();
    tap = this_cpu_ptr(&test_acqrel);
    WRITE_ONCE(*tap, READ_ONCE(*tap) + 1);
    un_delay(udl, ndl);
    WRITE_ONCE(*tap, READ_ONCE(*tap) - 1);
    local_bh_enable();
    }
    }
    static const struct ref_scale_ops incpercpubh_ops = {
    .init		= rcu_sync_scale_init,
    .readsection	= ref_incpercpubh_section,
    .delaysection	= ref_incpercpubh_delay_section,
    .enable_irqs	= true,
    .name		= "incpercpubh"
    };
#[no_mangle]
unsafe extern "C" fn ref_incpercpuirqsave_section(nloops: c_int) {
    static void ref_incpercpuirqsave_section(const int nloops)
    {
    int i;
    unsigned long flags;
    for (i = nloops; i >= 0; i--) {
    unsigned long *tap;
    local_irq_save(flags);
    tap = this_cpu_ptr(&test_acqrel);
    WRITE_ONCE(*tap, READ_ONCE(*tap) + 1);
    WRITE_ONCE(*tap, READ_ONCE(*tap) - 1);
    local_irq_restore(flags);
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_incpercpuirqsave_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_incpercpuirqsave_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    unsigned long flags;
    for (i = nloops; i >= 0; i--) {
    unsigned long *tap;
    local_irq_save(flags);
    tap = this_cpu_ptr(&test_acqrel);
    WRITE_ONCE(*tap, READ_ONCE(*tap) + 1);
    un_delay(udl, ndl);
    WRITE_ONCE(*tap, READ_ONCE(*tap) - 1);
    local_irq_restore(flags);
    }
    }
    static const struct ref_scale_ops incpercpuirqsave_ops = {
    .init		= rcu_sync_scale_init,
    .readsection	= ref_incpercpuirqsave_section,
    .delaysection	= ref_incpercpuirqsave_delay_section,
    .name		= "incpercpuirqsave"
    };
// Definitions for rwlock
    static rwlock_t test_rwlock;
#[no_mangle]
unsafe extern "C" fn ref_rwlock_init() -> bool {
    static bool ref_rwlock_init(void)
    {
    rwlock_init(&test_rwlock);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ref_rwlock_section(nloops: c_int) {
    static void ref_rwlock_section(const int nloops)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    read_lock(&test_rwlock);
    read_unlock(&test_rwlock);
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_rwlock_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_rwlock_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    read_lock(&test_rwlock);
    un_delay(udl, ndl);
    read_unlock(&test_rwlock);
    }
    }
    static const struct ref_scale_ops rwlock_ops = {
    .init		= ref_rwlock_init,
    .readsection	= ref_rwlock_section,
    .delaysection	= ref_rwlock_delay_section,
    .name		= "rwlock"
    };
// Definitions for rwsem
    static struct rw_semaphore test_rwsem;
#[no_mangle]
unsafe extern "C" fn ref_rwsem_init() -> bool {
    static bool ref_rwsem_init(void)
    {
    init_rwsem(&test_rwsem);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ref_rwsem_section(nloops: c_int) {
    static void ref_rwsem_section(const int nloops)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    down_read(&test_rwsem);
    up_read(&test_rwsem);
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_rwsem_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_rwsem_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    for (i = nloops; i >= 0; i--) {
    down_read(&test_rwsem);
    un_delay(udl, ndl);
    up_read(&test_rwsem);
    }
    }
    static const struct ref_scale_ops rwsem_ops = {
    .init		= ref_rwsem_init,
    .readsection	= ref_rwsem_section,
    .delaysection	= ref_rwsem_delay_section,
    .name		= "rwsem"
    };
// Definitions for global spinlock
    static DEFINE_RAW_SPINLOCK(test_lock);
#[no_mangle]
unsafe extern "C" fn ref_lock_section(nloops: c_int) {
    static void ref_lock_section(const int nloops)
    {
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    raw_spin_lock(&test_lock);
    raw_spin_unlock(&test_lock);
    }
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn ref_lock_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_lock_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    raw_spin_lock(&test_lock);
    un_delay(udl, ndl);
    raw_spin_unlock(&test_lock);
    }
    preempt_enable();
    }
    static const struct ref_scale_ops lock_ops = {
    .readsection	= ref_lock_section,
    .delaysection	= ref_lock_delay_section,
    .name		= "lock"
    };
// Definitions for global irq-save spinlock
#[no_mangle]
unsafe extern "C" fn ref_lock_irq_section(nloops: c_int) {
    static void ref_lock_irq_section(const int nloops)
    {
    unsigned long flags;
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    raw_spin_lock_irqsave(&test_lock, flags);
    raw_spin_unlock_irqrestore(&test_lock, flags);
    }
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn ref_lock_irq_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_lock_irq_delay_section(const int nloops, const int udl, const int ndl)
    {
    unsigned long flags;
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    raw_spin_lock_irqsave(&test_lock, flags);
    un_delay(udl, ndl);
    raw_spin_unlock_irqrestore(&test_lock, flags);
    }
    preempt_enable();
    }
    static const struct ref_scale_ops lock_irq_ops = {
    .readsection	= ref_lock_irq_section,
    .delaysection	= ref_lock_irq_delay_section,
    .name		= "lock-irq"
    };
#[no_mangle]
unsafe extern "C" fn ref_acqrel_section(nloops: c_int) {
    static void ref_acqrel_section(const int nloops)
    {
    unsigned long x;
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    x = smp_load_acquire(this_cpu_ptr(&test_acqrel));
    smp_store_release(this_cpu_ptr(&test_acqrel), x + 1);
    }
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn ref_acqrel_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_acqrel_delay_section(const int nloops, const int udl, const int ndl)
    {
    unsigned long x;
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    x = smp_load_acquire(this_cpu_ptr(&test_acqrel));
    un_delay(udl, ndl);
    smp_store_release(this_cpu_ptr(&test_acqrel), x + 1);
    }
    preempt_enable();
    }
    static const struct ref_scale_ops acqrel_ops = {
    .readsection	= ref_acqrel_section,
    .delaysection	= ref_acqrel_delay_section,
    .name		= "acqrel"
    };
    static volatile u64 stopopts;
#[no_mangle]
unsafe extern "C" fn ref_sched_clock_section(nloops: c_int) {
    static void ref_sched_clock_section(const int nloops)
    {
    let mut x: u64 = 0;
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--)
    x += sched_clock();
    preempt_enable();
    stopopts = x;
    }
#[no_mangle]
unsafe extern "C" fn ref_sched_clock_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_sched_clock_delay_section(const int nloops, const int udl, const int ndl)
    {
    let mut x: u64 = 0;
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    x += sched_clock();
    un_delay(udl, ndl);
    }
    preempt_enable();
    stopopts = x;
    }
    static const struct ref_scale_ops sched_clock_ops = {
    .readsection	= ref_sched_clock_section,
    .delaysection	= ref_sched_clock_delay_section,
    .name		= "sched-clock"
    };
#[no_mangle]
unsafe extern "C" fn ref_clock_section(nloops: c_int) {
    static void ref_clock_section(const int nloops)
    {
    let mut x: u64 = 0;
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--)
    x += ktime_get_real_fast_ns();
    preempt_enable();
    stopopts = x;
    }
#[no_mangle]
unsafe extern "C" fn ref_clock_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_clock_delay_section(const int nloops, const int udl, const int ndl)
    {
    let mut x: u64 = 0;
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    x += ktime_get_real_fast_ns();
    un_delay(udl, ndl);
    }
    preempt_enable();
    stopopts = x;
    }
    static const struct ref_scale_ops clock_ops = {
    .readsection	= ref_clock_section,
    .delaysection	= ref_clock_delay_section,
    .name		= "clock"
    };
#[no_mangle]
unsafe extern "C" fn ref_jiffies_section(nloops: c_int) {
    static void ref_jiffies_section(const int nloops)
    {
    let mut x: u64 = 0;
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--)
    x += jiffies;
    preempt_enable();
    stopopts = x;
    }
#[no_mangle]
unsafe extern "C" fn ref_jiffies_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_jiffies_delay_section(const int nloops, const int udl, const int ndl)
    {
    let mut x: u64 = 0;
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    x += jiffies;
    un_delay(udl, ndl);
    }
    preempt_enable();
    stopopts = x;
    }
    static const struct ref_scale_ops jiffies_ops = {
    .readsection	= ref_jiffies_section,
    .delaysection	= ref_jiffies_delay_section,
    .name		= "jiffies"
    };
#[no_mangle]
unsafe extern "C" fn ref_preempt_section(nloops: c_int) {
    static void ref_preempt_section(const int nloops)
    {
    int i;
    migrate_disable();
    for (i = nloops; i >= 0; i--) {
    preempt_disable();
    preempt_enable();
    }
    migrate_enable();
    }
#[no_mangle]
unsafe extern "C" fn ref_preempt_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_preempt_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    migrate_disable();
    for (i = nloops; i >= 0; i--) {
    preempt_disable();
    un_delay(udl, ndl);
    preempt_enable();
    }
    migrate_enable();
    }
    static const struct ref_scale_ops preempt_ops = {
    .readsection	= ref_preempt_section,
    .delaysection	= ref_preempt_delay_section,
    .name		= "preempt"
    };
#[no_mangle]
unsafe extern "C" fn ref_bh_section(nloops: c_int) {
    static void ref_bh_section(const int nloops)
    {
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    local_bh_disable();
    local_bh_enable();
    }
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn ref_bh_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_bh_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    local_bh_disable();
    un_delay(udl, ndl);
    local_bh_enable();
    }
    preempt_enable();
    }
    static const struct ref_scale_ops bh_ops = {
    .readsection	= ref_bh_section,
    .delaysection	= ref_bh_delay_section,
    .enable_irqs	= true,
    .name		= "bh"
    };
#[no_mangle]
unsafe extern "C" fn ref_irq_section(nloops: c_int) {
    static void ref_irq_section(const int nloops)
    {
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    local_irq_disable();
    local_irq_enable();
    }
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn ref_irq_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_irq_delay_section(const int nloops, const int udl, const int ndl)
    {
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    local_irq_disable();
    un_delay(udl, ndl);
    local_irq_enable();
    }
    preempt_enable();
    }
    static const struct ref_scale_ops irq_ops = {
    .readsection	= ref_irq_section,
    .delaysection	= ref_irq_delay_section,
    .name		= "irq"
    };
#[no_mangle]
unsafe extern "C" fn ref_irqsave_section(nloops: c_int) {
    static void ref_irqsave_section(const int nloops)
    {
    unsigned long flags;
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    local_irq_save(flags);
    local_irq_restore(flags);
    }
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn ref_irqsave_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void ref_irqsave_delay_section(const int nloops, const int udl, const int ndl)
    {
    unsigned long flags;
    int i;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    local_irq_save(flags);
    un_delay(udl, ndl);
    local_irq_restore(flags);
    }
    preempt_enable();
    }
    static const struct ref_scale_ops irqsave_ops = {
    .readsection	= ref_irqsave_section,
    .delaysection	= ref_irqsave_delay_section,
    .name		= "irqsave"
    };
//
// Methods leveraging SLAB_TYPESAFE_BY_RCU.
//
// Item to look up in a typesafe manner.  Array of pointers to these.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct refscale_typesafe {
    pub flavors: atomic_t rts_refctr; // Used by all,
    pub rts_lock: spinlock_t,
    pub rts_seqlock: seqlock_t,
    pub a: c_uint,
    pub b: c_uint,
}

    static struct kmem_cache *typesafe_kmem_cachep;
    static struct refscale_typesafe **rtsarray;
    static long rtsarray_size;
    static DEFINE_TORTURE_RANDOM_PERCPU(refscale_rand);
    static bool (*rts_acquire)(struct refscale_typesafe *rtsp, unsigned int *start);
    static bool (*rts_release)(struct refscale_typesafe *rtsp, unsigned int start);
// Conditionally acquire an explicit in-structure reference count.
#[no_mangle]
unsafe extern "C" fn typesafe_ref_acquire(rtsp: *mut refscale_typesafe, start: *mut c_uint) -> bool {
    static bool typesafe_ref_acquire(struct refscale_typesafe *rtsp, unsigned int *start)
    {
    return atomic_inc_not_zero(&rtsp.rts_refctr);
    }
// Unconditionally release an explicit in-structure reference count.
#[no_mangle]
unsafe extern "C" fn typesafe_ref_release(rtsp: *mut refscale_typesafe, start: c_uint) -> bool {
    static bool typesafe_ref_release(struct refscale_typesafe *rtsp, unsigned int start)
    {
    if (!atomic_dec_return(&rtsp.rts_refctr)) {
    WRITE_ONCE(rtsp.a, rtsp.a + 1);
    kmem_cache_free(typesafe_kmem_cachep, rtsp);
    }
    return true;
    }
// Unconditionally acquire an explicit in-structure spinlock.
#[no_mangle]
unsafe extern "C" fn typesafe_lock_acquire(rtsp: *mut refscale_typesafe, start: *mut c_uint) -> bool {
    static bool typesafe_lock_acquire(struct refscale_typesafe *rtsp, unsigned int *start)
    {
    spin_lock(&rtsp.rts_lock);
    return true;
    }
// Unconditionally release an explicit in-structure spinlock.
#[no_mangle]
unsafe extern "C" fn typesafe_lock_release(rtsp: *mut refscale_typesafe, start: c_uint) -> bool {
    static bool typesafe_lock_release(struct refscale_typesafe *rtsp, unsigned int start)
    {
    spin_unlock(&rtsp.rts_lock);
    return true;
    }
// Unconditionally acquire an explicit in-structure sequence lock.
#[no_mangle]
unsafe extern "C" fn typesafe_seqlock_acquire(rtsp: *mut refscale_typesafe, start: *mut c_uint) -> bool {
    static bool typesafe_seqlock_acquire(struct refscale_typesafe *rtsp, unsigned int *start)
    {
// start = read_seqbegin(&rtsp->rts_seqlock);
    return true;
    }
// Conditionally release an explicit in-structure sequence lock.  Return
// true if this release was successful, that is, if no retry is required.
#[no_mangle]
unsafe extern "C" fn typesafe_seqlock_release(rtsp: *mut refscale_typesafe, start: c_uint) -> bool {
    static bool typesafe_seqlock_release(struct refscale_typesafe *rtsp, unsigned int start)
    {
    return !read_seqretry(&rtsp.rts_seqlock, start);
    }
// Do a read-side critical section with the specified delay in
// microseconds and nanoseconds inserted so as to increase probability
// of failure.
#[no_mangle]
unsafe extern "C" fn typesafe_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    static void typesafe_delay_section(const int nloops, const int udl, const int ndl)
    {
    unsigned int a;
    unsigned int b;
    int i;
    long idx;
    struct refscale_typesafe *rtsp;
    unsigned int start;
    for (i = nloops; i >= 0; i--) {
    preempt_disable();
    idx = torture_random(this_cpu_ptr(&refscale_rand)) % rtsarray_size;
    preempt_enable();
    retry:
    rcu_read_lock();
    rtsp = rcu_dereference(rtsarray[idx]);
    a = READ_ONCE(rtsp.a);
    if (!rts_acquire(rtsp, &start)) {
    rcu_read_unlock();
    goto retry;
    }
    if (a != READ_ONCE(rtsp.a)) {
    (void)rts_release(rtsp, start);
    rcu_read_unlock();
    goto retry;
    }
    un_delay(udl, ndl);
    b = READ_ONCE(rtsp.a);
// Remember, seqlock read-side release can fail.
    if (!rts_release(rtsp, start)) {
    rcu_read_unlock();
    goto retry;
    }
    WARN_ONCE(a != b, "Re-read of .a changed from %u to %u.\n", a, b);
    b = rtsp.b;
    rcu_read_unlock();
    WARN_ON_ONCE(a * a != b);
    }
    }
// Because the acquisition and release methods are expensive, there
// is no point in optimizing away the un_delay() function's two checks.
// Thus simply define typesafe_read_section() as a simple wrapper around
// typesafe_delay_section().
#[no_mangle]
unsafe extern "C" fn typesafe_read_section(nloops: c_int) {
    static void typesafe_read_section(const int nloops)
    {
    typesafe_delay_section(nloops, 0, 0);
    }
// Allocate and initialize one refscale_typesafe structure.
    static struct refscale_typesafe *typesafe_alloc_one(void)
    {
    struct refscale_typesafe *rtsp;
    rtsp = kmem_cache_alloc(typesafe_kmem_cachep, GFP_KERNEL);
    if (!rtsp)
    return core::ptr::null_mut();
    atomic_set(&rtsp.rts_refctr, 1);
    WRITE_ONCE(rtsp.a, rtsp.a + 1);
    WRITE_ONCE(rtsp.b, rtsp.a * rtsp.a);
    return rtsp;
    }
// Slab-allocator constructor for refscale_typesafe structures created
// out of a new slab of system memory.
#[no_mangle]
unsafe extern "C" fn refscale_typesafe_ctor(rtsp_in: *mut c_void) {
    static void refscale_typesafe_ctor(void *rtsp_in)
    {
    struct refscale_typesafe *rtsp = rtsp_in;
    spin_lock_init(&rtsp.rts_lock);
    seqlock_init(&rtsp.rts_seqlock);
    preempt_disable();
    rtsp.a = torture_random(this_cpu_ptr(&refscale_rand));
    preempt_enable();
    }
    static const struct ref_scale_ops typesafe_ref_ops;
    static const struct ref_scale_ops typesafe_lock_ops;
    static const struct ref_scale_ops typesafe_seqlock_ops;
// Initialize for a typesafe test.
#[no_mangle]
unsafe extern "C" fn typesafe_init() -> bool {
    static bool typesafe_init(void)
    {
    long idx;
    let mut si: c_long = lookup_instances;
    typesafe_kmem_cachep = kmem_cache_create("refscale_typesafe",
    sizeof(struct refscale_typesafe), sizeof(void *),
    SLAB_TYPESAFE_BY_RCU, refscale_typesafe_ctor);
    if (!typesafe_kmem_cachep)
    return false;
    if (si < 0)
    si = -si * nr_cpu_ids;
#[no_mangle]
pub unsafe extern "C" fn if(0: si ==) -> else {
    else if (si == 0)
    si = nr_cpu_ids;
    rtsarray_size = si;
    rtsarray = kzalloc_objs(*rtsarray, si);
    if (!rtsarray)
    return false;
    for (idx = 0; idx < rtsarray_size; idx++) {
    rtsarray[idx] = typesafe_alloc_one();
    if (!rtsarray[idx])
    return false;
    }
    if (cur_ops == &typesafe_ref_ops) {
    rts_acquire = typesafe_ref_acquire;
    rts_release = typesafe_ref_release;
    } else if (cur_ops == &typesafe_lock_ops) {
    rts_acquire = typesafe_lock_acquire;
    rts_release = typesafe_lock_release;
    } else if (cur_ops == &typesafe_seqlock_ops) {
    rts_acquire = typesafe_seqlock_acquire;
    rts_release = typesafe_seqlock_release;
    } else {
    WARN_ON_ONCE(1);
    return false;
    }
    return true;
    }
// Clean up after a typesafe test.
#[no_mangle]
unsafe extern "C" fn typesafe_cleanup() {
    static void typesafe_cleanup(void)
    {
    long idx;
    if (rtsarray) {
    for (idx = 0; idx < rtsarray_size; idx++)
    kmem_cache_free(typesafe_kmem_cachep, rtsarray[idx]);
    kfree(rtsarray);
    rtsarray = core::ptr::null_mut();
    rtsarray_size = 0;
    }
    kmem_cache_destroy(typesafe_kmem_cachep);
    typesafe_kmem_cachep = core::ptr::null_mut();
    rts_acquire = core::ptr::null_mut();
    rts_release = core::ptr::null_mut();
    }
// The typesafe_init() function distinguishes these structures by address.
    static const struct ref_scale_ops typesafe_ref_ops = {
    .init		= typesafe_init,
    .cleanup	= typesafe_cleanup,
    .readsection	= typesafe_read_section,
    .delaysection	= typesafe_delay_section,
    .name		= "typesafe_ref"
    };
    static const struct ref_scale_ops typesafe_lock_ops = {
    .init		= typesafe_init,
    .cleanup	= typesafe_cleanup,
    .readsection	= typesafe_read_section,
    .delaysection	= typesafe_delay_section,
    .name		= "typesafe_lock"
    };
    static const struct ref_scale_ops typesafe_seqlock_ops = {
    .init		= typesafe_init,
    .cleanup	= typesafe_cleanup,
    .readsection	= typesafe_read_section,
    .delaysection	= typesafe_delay_section,
    .name		= "typesafe_seqlock"
    };
#[no_mangle]
unsafe extern "C" fn rcu_scale_one_reader() {
    static void rcu_scale_one_reader(void)
    {
    if (readdelay <= 0)
    cur_ops.readsection(loops);
    else
    cur_ops.delaysection(loops, readdelay / 1000, readdelay % 1000);
    }
// Warm up cache, or, if needed run a series of rcu_scale_one_reader()
// to allow multiple rcuscale guest OSes to collect mutually valid data.
#[no_mangle]
unsafe extern "C" fn rcu_scale_warm_cool() {
    static void rcu_scale_warm_cool(void)
    {
    let mut jdone: c_ulong = jiffies + (guest_os_delay > 0 ? guest_os_delay * HZ : -1);
    do {
    rcu_scale_one_reader();
    cond_resched();
    } while (time_before(jiffies, jdone));
    }
// Reader kthread.  Repeatedly does empty RCU read-side
// critical section, minimizing update-side interference.
    static int
    ref_scale_reader(void *arg)
    {
    unsigned long flags;
    let mut me: c_long = (long)arg;
    struct reader_task *rt = &(reader_tasks[me]);
    u64 start;
    s64 duration;
    VERBOSE_SCALEOUT_BATCH("ref_scale_reader %ld: task started", me);
    WARN_ON_ONCE(set_cpus_allowed_ptr(current, cpumask_of(me % nr_cpu_ids)));
    set_user_nice(current, MAX_NICE);
    atomic_inc(&n_init);
    if (holdoff)
    schedule_timeout_interruptible(holdoff * HZ);
    repeat:
    VERBOSE_SCALEOUT_BATCH("ref_scale_reader %ld: waiting to start next experiment on cpu %d", me, raw_smp_processor_id());
// Wait for signal that this reader can start.
    wait_event(rt.wq, (atomic_read(&nreaders_exp) && smp_load_acquire(&rt.start_reader)) ||
    torture_must_stop());
    if (torture_must_stop())
    goto end;
// Make sure that the CPU is affinitized appropriately during testing.
    WARN_ON_ONCE(raw_smp_processor_id() != me % nr_cpu_ids);
    WRITE_ONCE(rt.start_reader, 0);
    if (!atomic_dec_return(&n_started))
    while (atomic_read_acquire(&n_started))
    cpu_relax();
    VERBOSE_SCALEOUT_BATCH("ref_scale_reader %ld: experiment %d started", me, exp_idx);
// To reduce noise, do an initial cache-warming invocation, check
// in, and then keep warming until everyone has checked in.
    rcu_scale_one_reader();
    if (!atomic_dec_return(&n_warmedup))
    while (atomic_read_acquire(&n_warmedup))
    rcu_scale_one_reader();
// Also keep interrupts disabled when it is safe to do so, which
// it is not for local_bh_enable().  This also has the effect of
// preventing entries into slow path for rcu_read_unlock().
    if (!cur_ops.enable_irqs)
    local_irq_save(flags);
    start = ktime_get_mono_fast_ns();
    rcu_scale_one_reader();
    duration = ktime_get_mono_fast_ns() - start;
    if (!cur_ops.enable_irqs)
    local_irq_restore(flags);
    rt.last_duration_ns = WARN_ON_ONCE(duration < 0) ? 0 : duration;
// To reduce runtime-skew noise, do maintain-load invocations until
// everyone is done.
    if (!atomic_dec_return(&n_cooleddown))
    while (atomic_read_acquire(&n_cooleddown))
    rcu_scale_one_reader();
    if (atomic_dec_and_test(&nreaders_exp))
    wake_up(&main_wq);
    VERBOSE_SCALEOUT_BATCH("ref_scale_reader %ld: experiment %d ended, (readers remaining=%d)",
    me, exp_idx, atomic_read(&nreaders_exp));
    if (!torture_must_stop())
    goto repeat;
    end:
    torture_kthread_stopping("ref_scale_reader");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn reset_readers() {
    static void reset_readers(void)
    {
    int i;
    struct reader_task *rt;
    for (i = 0; i < nreaders; i++) {
    rt = &(reader_tasks[i]);
    rt.last_duration_ns = 0;
    }
    }
// Print the results of each reader and return the sum of all their durations.
#[no_mangle]
unsafe extern "C" fn process_durations(n: c_int) -> u64 {
    static u64 process_durations(int n)
    {
    int i;
    struct reader_task *rt;
    struct seq_buf s;
    char *buf;
    let mut sum: u64 = 0;
    buf = kmalloc(800 + 64, GFP_KERNEL);
    if (!buf)
    return 0;
    seq_buf_init(&s, buf, 800 + 64);
    seq_buf_printf(&s, "Experiment #%d (Format: <THREAD-NUM>:<Total loop time in ns>)",
    exp_idx);
    for (i = 0; i < n && !torture_must_stop(); i++) {
    rt = &(reader_tasks[i]);
    if (i % 5 == 0)
    seq_buf_putc(&s, '\n');
    if (seq_buf_used(&s) >= 800) {
    pr_alert("%s", seq_buf_str(&s));
    seq_buf_clear(&s);
    }
    seq_buf_printf(&s, "%d: %llu\t", i, rt.last_duration_ns);
    sum += rt.last_duration_ns;
    }
    pr_alert("%s\n", seq_buf_str(&s));
    kfree(buf);
    return sum;
    }
    static void ref_scale_cleanup(void);
// The main_func is the main orchestrator, it performs a bunch of
// experiments.  For every experiment, it orders all the readers
// involved to start and waits for them to finish the experiment. It
// then reads their timestamps and starts the next experiment. Each
// experiment progresses from 1 concurrent reader to N of them at which
// point all the timestamps are printed.
#[no_mangle]
unsafe extern "C" fn main_func(arg: *mut c_void) -> c_int {
    static int main_func(void *arg)
    {
    int exp, r;
    char buf1[64];
    char *buf;
    u64 *result_avg;
    set_cpus_allowed_ptr(current, cpumask_of(nreaders % nr_cpu_ids));
    set_user_nice(current, MAX_NICE);
    VERBOSE_SCALEOUT("main_func task started");
    result_avg = kcalloc(nruns, sizeof(*result_avg), GFP_KERNEL);
    buf = kzalloc(800 + 64, GFP_KERNEL);
    if (!result_avg || !buf) {
    SCALEOUT_ERRSTRING("out of memory");
    goto oom_exit;
    }
    if (holdoff)
    schedule_timeout_interruptible(holdoff * HZ);
// Wait for all threads to start.
    atomic_inc(&n_init);
    while (atomic_read(&n_init) < nreaders + 1)
    schedule_timeout_uninterruptible(1);
// Start exp readers up per experiment
    rcu_scale_warm_cool();
    for (exp = 0; exp < nruns && !torture_must_stop(); exp++) {
    if (torture_must_stop())
    goto end;
    reset_readers();
    atomic_set(&nreaders_exp, nreaders);
    atomic_set(&n_started, nreaders);
    atomic_set(&n_warmedup, nreaders);
    atomic_set(&n_cooleddown, nreaders);
    exp_idx = exp;
    for (r = 0; r < nreaders; r++) {
    smp_store_release(&reader_tasks[r].start_reader, 1);
    wake_up(&reader_tasks[r].wq);
    }
    VERBOSE_SCALEOUT("main_func: experiment started, waiting for %d readers",
    nreaders);
    wait_event(main_wq,
    !atomic_read(&nreaders_exp) || torture_must_stop());
    VERBOSE_SCALEOUT("main_func: experiment ended");
    if (torture_must_stop())
    goto end;
    result_avg[exp] = div_u64(1000 * process_durations(nreaders), nreaders * loops);
    }
    rcu_scale_warm_cool();
// Print the average of all experiments
    SCALEOUT("END OF TEST. Calculating average duration per loop (nanoseconds)...\n");
    pr_alert("Runs\tTime(ns)\n");
    for (exp = 0; exp < nruns; exp++) {
    u64 avg;
    u32 rem;
    avg = div_u64_rem(result_avg[exp], 1000, &rem);
    sprintf(buf1, "%d\t%llu.%03u\n", exp + 1, avg, rem);
    strcat(buf, buf1);
    if (strlen(buf) >= 800) {
    pr_alert("%s", buf);
    buf[0] = 0;
    }
    }
    pr_alert("%s", buf);
    oom_exit:
// This will shutdown everything including us.
    if (shutdown_secs) {
    main_task = core::ptr::null_mut();  // Avoid self-kill deadlock.
    ref_scale_cleanup();
    kernel_power_off();
    }
// Wait for torture to stop us
    while (!torture_must_stop())
    schedule_timeout_uninterruptible(1);
    end:
    torture_kthread_stopping("main_func");
    kfree(result_avg);
    kfree(buf);
    return 0;
    }
    static void
    ref_scale_print_module_parms(const struct ref_scale_ops *cur_ops, const char *tag)
    {
    pr_alert("%s" SCALE_FLAG
    "--- %s:  verbose=%d verbose_batched=%d shutdown_secs=%d holdoff=%d lookup_instances=%ld loops=%d nreaders=%d nruns=%d readdelay=%d\n", scale_type, tag,
    verbose, verbose_batched, shutdown_secs, holdoff, lookup_instances, loops, nreaders, nruns, readdelay);
    }
    static void
    ref_scale_cleanup(void)
    {
    int i;
    if (torture_cleanup_begin())
    return;
    if (!cur_ops) {
    torture_cleanup_end();
    return;
    }
    if (reader_tasks) {
    for (i = 0; i < nreaders; i++)
    torture_stop_kthread("ref_scale_reader",
    reader_tasks[i].task);
    }
    kfree(reader_tasks);
    reader_tasks = core::ptr::null_mut();
    torture_stop_kthread("main_task", main_task);
// Do scale-type-specific cleanup operations.
    if (cur_ops.cleanup != core::ptr::null_mut())
    cur_ops.cleanup();
    torture_cleanup_end();
    }
    static int __init
    ref_scale_init(void)
    {
    long i;
    let mut firsterr: c_int = 0;
    static const struct ref_scale_ops *scale_ops[] = {
    &rcu_ops, &srcu_ops, &srcu_fast_ops, &srcu_fast_updown_ops,
    RCU_TRACE_OPS RCU_TASKS_OPS
    &refcnt_ops, &percpuinc_ops, &incpercpu_ops, &incpercpupreempt_ops,
    &incpercpubh_ops, &incpercpuirqsave_ops,
    &rwlock_ops, &rwsem_ops, &lock_ops, &lock_irq_ops, &acqrel_ops,
    &sched_clock_ops, &clock_ops, &jiffies_ops,
    &preempt_ops, &bh_ops, &irq_ops, &irqsave_ops,
    &typesafe_ref_ops, &typesafe_lock_ops, &typesafe_seqlock_ops,
    };
    if (!torture_init_begin(scale_type, verbose))
    return -EBUSY;
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
    if (!cur_ops.init()) {
    firsterr = -EUCLEAN;
    goto unwind;
    }
    ref_scale_print_module_parms(cur_ops, "Start of test");
// Shutdown task
    if (shutdown_secs) {
    firsterr = torture_shutdown_init(shutdown_secs, ref_scale_cleanup);
    if (torture_init_error(firsterr))
    goto unwind;
    }
// Reader tasks (default to ~75% of online CPUs).
    if (nreaders < 0)
    nreaders = (num_online_cpus() >> 1) + (num_online_cpus() >> 2);
    if (WARN_ONCE(loops <= 0, "%s: loops = %d, adjusted to 1\n", __func__, loops))
    loops = 1;
    if (WARN_ONCE(nreaders <= 0, "%s: nreaders = %d, adjusted to 1\n", __func__, nreaders))
    nreaders = 1;
    if (WARN_ONCE(nruns <= 0, "%s: nruns = %d, adjusted to 1\n", __func__, nruns))
    nruns = 1;
    if (WARN_ONCE(loops > INT_MAX / nreaders,
    "%s: nreaders * loops will overflow, adjusted loops to %d",
    __func__, INT_MAX / nreaders))
    loops = INT_MAX / nreaders;
    reader_tasks = kzalloc_objs(reader_tasks[0], nreaders);
    if (!reader_tasks) {
    SCALEOUT_ERRSTRING("out of memory");
    firsterr = -ENOMEM;
    goto unwind;
    }
    VERBOSE_SCALEOUT("Starting %d reader threads", nreaders);
    for (i = 0; i < nreaders; i++) {
    init_waitqueue_head(&reader_tasks[i].wq);
    firsterr = torture_create_kthread(ref_scale_reader, (void *)i,
    reader_tasks[i].task);
    if (torture_init_error(firsterr))
    goto unwind;
    }
// Main Task
    init_waitqueue_head(&main_wq);
    firsterr = torture_create_kthread(main_func, core::ptr::null_mut(), main_task);
    if (torture_init_error(firsterr))
    goto unwind;
    torture_init_end();
    return 0;
    unwind:
    torture_init_end();
    ref_scale_cleanup();
    if (shutdown_secs) {
    WARN_ON(!IS_MODULE(CONFIG_RCU_REF_SCALE_TEST));
    kernel_power_off();
    }
    return firsterr;
    }
    module_init(ref_scale_init);
    module_exit(ref_scale_cleanup);

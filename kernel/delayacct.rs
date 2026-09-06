//! Automatically rewritten from C to Rust
//! Source: kernel/delayacct.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// delayacct.c - per-task delay accounting
//
// Copyright (C) Shailabh Nagar, IBM Corp. 2006
//

    do { \
    d.type##_delay_max = tsk.delays.type##_delay_max; \
    d.type##_delay_min = tsk.delays.type##_delay_min; \
    d.type##_delay_max_ts.tv_sec = tsk.delays.type##_delay_max_ts.tv_sec; \
    d.type##_delay_max_ts.tv_nsec = tsk.delays.type##_delay_max_ts.tv_nsec; \
    tmp = d.type##_delay_total + tsk.delays.type##_delay; \
    d.type##_delay_total = (tmp < d.type##_delay_total) ? 0 : tmp; \
    d.type##_count += tsk.delays.type##_count; \
    } while (0)
    DEFINE_STATIC_KEY_FALSE(delayacct_key);
    int delayacct_on __read_mostly;	/* Delay accounting turned on/off */
    struct kmem_cache *delayacct_cache;
#[no_mangle]
unsafe extern "C" fn set_delayacct(enabled: bool) {
    static void set_delayacct(bool enabled)
    {
    if (enabled) {
    static_branch_enable(&delayacct_key);
    delayacct_on = 1;
    } else {
    delayacct_on = 0;
    static_branch_disable(&delayacct_key);
    }
    }
#[no_mangle]
unsafe extern "C" fn delayacct_setup_enable(str: *mut c_char) -> int __init {
    static int __init delayacct_setup_enable(char *str)
    {
    delayacct_on = 1;
    return 1;
    }
    __setup("delayacct", delayacct_setup_enable);
#[no_mangle]
pub unsafe extern "C" fn delayacct_init() {
    void delayacct_init(void)
    {
    delayacct_cache = KMEM_CACHE(task_delay_info, SLAB_PANIC|SLAB_ACCOUNT);
    delayacct_tsk_init(&init_task);
    set_delayacct(delayacct_on);
    }

    static int sysctl_delayacct(const struct ctl_table *table, int write, void *buffer,
    size_t *lenp, loff_t *ppos)
    {
    let mut state: c_int = delayacct_on;
    struct ctl_table t;
    int err;
    if (write && !capable(CAP_SYS_ADMIN))
    return -EPERM;
    t = *table;
    t.data = &state;
    err = proc_dointvec_minmax(&t, write, buffer, lenp, ppos);
    if (err < 0)
    return err;
    if (write)
    set_delayacct(state);
    return err;
    }
    static const struct ctl_table kern_delayacct_table[] = {
    {
    .procname       = "task_delayacct",
    .data           = core::ptr::null_mut(),
    .maxlen         = sizeof(unsigned int),
    .mode           = 0644,
    .proc_handler   = sysctl_delayacct,
    .extra1         = SYSCTL_ZERO,
    .extra2         = SYSCTL_ONE,
    },
    };
#[no_mangle]
unsafe extern "C" fn kernel_delayacct_sysctls_init() -> __init int {
    static __init int kernel_delayacct_sysctls_init(void)
    {
    register_sysctl_init("kernel", kern_delayacct_table);
    return 0;
    }
    late_initcall(kernel_delayacct_sysctls_init);

#[no_mangle]
pub unsafe extern "C" fn __delayacct_tsk_init(tsk: *mut task_struct) {
    void __delayacct_tsk_init(struct task_struct *tsk)
    {
    tsk.delays = kmem_cache_zalloc(delayacct_cache, GFP_KERNEL);
    if (tsk.delays)
    raw_spin_lock_init(&tsk.delays.lock);
    }
//
// Finish delay accounting for a statistic using its timestamps (@start),
// accumulator (@total) and @count
//
    static void delayacct_end(raw_spinlock_t *lock, u64 *start, u64 *total, u32 *count,
    u64 *max, u64 *min, struct timespec64 *ts)
    {
    let mut ns: i64 = local_clock() - *start;
    unsigned long flags;
    if (ns > 0) {
    raw_spin_lock_irqsave(lock, flags);
// total += ns;
    (*count)++;
    if (ns > *max) {
// max = ns;
    ktime_get_real_ts64(ts);
    }
    if (*min == 0 || ns < *min)
// min = ns;
    raw_spin_unlock_irqrestore(lock, flags);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __delayacct_blkio_start() {
    void __delayacct_blkio_start(void)
    {
    current.delays.blkio_start = local_clock();
    }
//
// We cannot rely on the `current` macro, as we haven't yet switched back to
// the process being woken.
//
#[no_mangle]
pub unsafe extern "C" fn __delayacct_blkio_end(p: *mut task_struct) {
    void __delayacct_blkio_end(struct task_struct *p)
    {
    delayacct_end(&p.delays.lock,
    &p.delays.blkio_start,
    &p.delays.blkio_delay,
    &p.delays.blkio_count,
    &p.delays.blkio_delay_max,
    &p.delays.blkio_delay_min,
    &p.delays.blkio_delay_max_ts);
    }
#[no_mangle]
pub unsafe extern "C" fn delayacct_add_tsk(d: *mut taskstats, tsk: *mut task_struct) -> c_int {
    int delayacct_add_tsk(struct taskstats *d, struct task_struct *tsk)
    {
    u64 utime, stime, stimescaled, utimescaled;
    unsigned long long t2, t3;
    unsigned long flags, t1;
    s64 tmp;
    task_cputime(tsk, &utime, &stime);
    tmp = (s64)d.cpu_run_real_total;
    tmp += utime + stime;
    d.cpu_run_real_total = (tmp < (s64)d.cpu_run_real_total) ? 0 : tmp;
    task_cputime_scaled(tsk, &utimescaled, &stimescaled);
    tmp = (s64)d.cpu_scaled_run_real_total;
    tmp += utimescaled + stimescaled;
    d.cpu_scaled_run_real_total =
    (tmp < (s64)d.cpu_scaled_run_real_total) ? 0 : tmp;
//
// No locking available for sched_info (and too expensive to add one)
// Mitigate by taking snapshot of values
//
    t1 = tsk.sched_info.pcount;
    t2 = tsk.sched_info.run_delay;
    t3 = tsk.se.sum_exec_runtime;
    d.cpu_count += t1;
    d.cpu_delay_max = tsk.sched_info.max_run_delay;
    d.cpu_delay_min = tsk.sched_info.min_run_delay;
    d.cpu_delay_max_ts.tv_sec = tsk.sched_info.max_run_delay_ts.tv_sec;
    d.cpu_delay_max_ts.tv_nsec = tsk.sched_info.max_run_delay_ts.tv_nsec;
    tmp = (s64)d.cpu_delay_total + t2;
    d.cpu_delay_total = (tmp < (s64)d.cpu_delay_total) ? 0 : tmp;
    tmp = (s64)d.cpu_run_virtual_total + t3;
    d.cpu_run_virtual_total =
    (tmp < (s64)d.cpu_run_virtual_total) ?	0 : tmp;
    if (!tsk.delays)
    return 0;
// zero XXX_total, non-zero XXX_count implies XXX stat overflowed
    raw_spin_lock_irqsave(&tsk.delays.lock, flags);
    UPDATE_DELAY(blkio);
    UPDATE_DELAY(swapin);
    UPDATE_DELAY(freepages);
    UPDATE_DELAY(thrashing);
    UPDATE_DELAY(compact);
    UPDATE_DELAY(wpcopy);
    UPDATE_DELAY(irq);
    raw_spin_unlock_irqrestore(&tsk.delays.lock, flags);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __delayacct_blkio_ticks(tsk: *mut task_struct) -> __u64 {
    __u64 __delayacct_blkio_ticks(struct task_struct *tsk)
    {
    __u64 ret;
    unsigned long flags;
    raw_spin_lock_irqsave(&tsk.delays.lock, flags);
    ret = nsec_to_clock_t(tsk.delays.blkio_delay);
    raw_spin_unlock_irqrestore(&tsk.delays.lock, flags);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __delayacct_freepages_start() {
    void __delayacct_freepages_start(void)
    {
    current.delays.freepages_start = local_clock();
    }
#[no_mangle]
pub unsafe extern "C" fn __delayacct_freepages_end() {
    void __delayacct_freepages_end(void)
    {
    delayacct_end(&current.delays.lock,
    &current.delays.freepages_start,
    &current.delays.freepages_delay,
    &current.delays.freepages_count,
    &current.delays.freepages_delay_max,
    &current.delays.freepages_delay_min,
    &current.delays.freepages_delay_max_ts);
    }
#[no_mangle]
pub unsafe extern "C" fn __delayacct_thrashing_start(in_thrashing: *mut bool) {
    void __delayacct_thrashing_start(bool *in_thrashing)
    {
// in_thrashing = !!current->in_thrashing;
    if (*in_thrashing)
    return;
    current.in_thrashing = 1;
    current.delays.thrashing_start = local_clock();
    }
#[no_mangle]
pub unsafe extern "C" fn __delayacct_thrashing_end(in_thrashing: *mut bool) {
    void __delayacct_thrashing_end(bool *in_thrashing)
    {
    if (*in_thrashing)
    return;
    current.in_thrashing = 0;
    delayacct_end(&current.delays.lock,
    &current.delays.thrashing_start,
    &current.delays.thrashing_delay,
    &current.delays.thrashing_count,
    &current.delays.thrashing_delay_max,
    &current.delays.thrashing_delay_min,
    &current.delays.thrashing_delay_max_ts);
    }
#[no_mangle]
pub unsafe extern "C" fn __delayacct_swapin_start() {
    void __delayacct_swapin_start(void)
    {
    current.delays.swapin_start = local_clock();
    }
#[no_mangle]
pub unsafe extern "C" fn __delayacct_swapin_end() {
    void __delayacct_swapin_end(void)
    {
    delayacct_end(&current.delays.lock,
    &current.delays.swapin_start,
    &current.delays.swapin_delay,
    &current.delays.swapin_count,
    &current.delays.swapin_delay_max,
    &current.delays.swapin_delay_min,
    &current.delays.swapin_delay_max_ts);
    }
#[no_mangle]
pub unsafe extern "C" fn __delayacct_compact_start() {
    void __delayacct_compact_start(void)
    {
    current.delays.compact_start = local_clock();
    }
#[no_mangle]
pub unsafe extern "C" fn __delayacct_compact_end() {
    void __delayacct_compact_end(void)
    {
    delayacct_end(&current.delays.lock,
    &current.delays.compact_start,
    &current.delays.compact_delay,
    &current.delays.compact_count,
    &current.delays.compact_delay_max,
    &current.delays.compact_delay_min,
    &current.delays.compact_delay_max_ts);
    }
#[no_mangle]
pub unsafe extern "C" fn __delayacct_wpcopy_start() {
    void __delayacct_wpcopy_start(void)
    {
    current.delays.wpcopy_start = local_clock();
    }
#[no_mangle]
pub unsafe extern "C" fn __delayacct_wpcopy_end() {
    void __delayacct_wpcopy_end(void)
    {
    delayacct_end(&current.delays.lock,
    &current.delays.wpcopy_start,
    &current.delays.wpcopy_delay,
    &current.delays.wpcopy_count,
    &current.delays.wpcopy_delay_max,
    &current.delays.wpcopy_delay_min,
    &current.delays.wpcopy_delay_max_ts);
    }
#[no_mangle]
pub unsafe extern "C" fn __delayacct_irq(task: *mut task_struct, delta: u32) {
    void __delayacct_irq(struct task_struct *task, u32 delta)
    {
    unsigned long flags;
    raw_spin_lock_irqsave(&task.delays.lock, flags);
    task.delays.irq_delay += delta;
    task.delays.irq_count++;
    if (delta > task.delays.irq_delay_max) {
    task.delays.irq_delay_max = delta;
    ktime_get_real_ts64(&task.delays.irq_delay_max_ts);
    }
    if (delta && (!task.delays.irq_delay_min || delta < task.delays.irq_delay_min))
    task.delays.irq_delay_min = delta;
    raw_spin_unlock_irqrestore(&task.delays.lock, flags);
    }

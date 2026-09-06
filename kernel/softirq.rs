//! Automatically rewritten from C to Rust
//! Source: kernel/softirq.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/kernel/softirq.c
//
// Copyright (C) 1992 Linus Torvalds
//
// Rewritten. Old one was good in 2.2, but in 2.3 it was immoral. --ANK (990903)
//

// Macro flag: #define INSTANTIATE_EXPORTED_INTERRUPT_DISABLE

// Macro flag: #define CREATE_TRACE_POINTS

//
    - No shared variables, all the data are CPU local.
    - If a softirq needs serialization, let it serialize itself
    by its own spinlocks.
    - Even if softirq is serialized, only local cpu is marked for
    execution. Hence, we get something sort of weak cpu binding.
    Though it is still not clear, will it result in better locality
    or will not.
    Examples:
    - NET RX softirq. It is multithreaded and does not require
    any global serialization.
    - NET TX softirq. It kicks software netdevice queues, hence
    it is logically serialized per device, but this serialization
    is invisible to common code.
    - Tasklets: serialized wrt itself.
//

    DEFINE_PER_CPU_ALIGNED(irq_cpustat_t, irq_stat);
    EXPORT_PER_CPU_SYMBOL(irq_stat);

    static struct softirq_action softirq_vec[NR_SOFTIRQS] __cacheline_aligned_in_smp;
    DEFINE_PER_CPU(struct task_struct *, ksoftirqd);
    const char * const softirq_to_name[NR_SOFTIRQS] = {
    "HI", "TIMER", "NET_TX", "NET_RX", "BLOCK", "IRQ_POLL",
    "TASKLET", "SCHED", "HRTIMER", "RCU"
    };
//
// we cannot loop indefinitely here to avoid userspace starvation,
// but we also don't want to introduce a worst case 1/HZ latency
// to the pending events, so lets the scheduler to balance
// the softirq load for us.
//
#[no_mangle]
unsafe extern "C" fn wakeup_softirqd() {
    static void wakeup_softirqd(void)
    {
// Interrupts are disabled: no need to stop preemption
    struct task_struct *tsk = __this_cpu_read(ksoftirqd);
    if (tsk)
    wake_up_process(tsk);
    }

    DEFINE_PER_CPU(int, hardirqs_enabled);
    DEFINE_PER_CPU(int, hardirq_context);
    EXPORT_PER_CPU_SYMBOL_GPL(hardirqs_enabled);
    EXPORT_PER_CPU_SYMBOL_GPL(hardirq_context);

    DEFINE_PER_CPU(unsigned long, local_interrupt_disable_state);
#[no_mangle]
pub unsafe extern "C" fn _local_interrupt_disable() {
    void _local_interrupt_disable(void)
    {
    __local_interrupt_disable();
    }
    EXPORT_SYMBOL(_local_interrupt_disable);
#[no_mangle]
pub unsafe extern "C" fn _local_interrupt_enable() {
    void _local_interrupt_enable(void)
    {
    __local_interrupt_enable();
    }
    EXPORT_SYMBOL(_local_interrupt_enable);

//
// Any 32bit architecture that still cares about performance should
// probably ensure this is near preempt_count.
//
    DEFINE_PER_CPU(unsigned int, nmi_nesting);

//
// SOFTIRQ_OFFSET usage:
//
// On !RT kernels 'count' is the preempt counter, on RT kernels this applies
// to a per CPU counter and to task::softirqs_disabled_cnt.
//
// - count is changed by SOFTIRQ_OFFSET on entering or leaving softirq
// processing.
//
// - count is changed by SOFTIRQ_DISABLE_OFFSET (= 2 * SOFTIRQ_OFFSET)
// on local_bh_disable or local_bh_enable.
//
// This lets us distinguish between whether we are currently processing
// softirq and whether we just have bh disabled.
//

//
// RT accounts for BH disabled sections in task::softirqs_disabled_cnt and
// also in per CPU softirq_ctrl::cnt. This is necessary to allow tasks in a
// softirq disabled section to be preempted.
//
// The per task counter is used for softirq_count(), in_softirq() and
// in_serving_softirqs() because these counts are only valid when the task
// holding softirq_ctrl::lock is running.
//
// The per CPU counter prevents pointless wakeups of ksoftirqd in case that
// the task which is in a softirq disabled section is preempted or blocks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct softirq_ctrl {
    pub lock: local_lock_t,
    pub cnt: c_int,
}

    static DEFINE_PER_CPU(struct softirq_ctrl, softirq_ctrl) = {
    .lock	= INIT_LOCAL_LOCK(softirq_ctrl.lock),
    };

    static struct lock_class_key bh_lock_key;
    struct lockdep_map bh_lock_map = {
    .name			= "local_bh",
    .key			= &bh_lock_key,
    .wait_type_outer	= LD_WAIT_FREE,
    .wait_type_inner	= LD_WAIT_CONFIG, /* PREEMPT_RT makes BH preemptible. */
    .lock_type		= LD_LOCK_PERCPU,
    };
    EXPORT_SYMBOL_GPL(bh_lock_map);

//
// local_bh_blocked() - Check for idle whether BH processing is blocked
//
// Returns false if the per CPU softirq::cnt is 0 otherwise true.
//
// This is invoked from the idle task to guard against false positive
// softirq pending warnings, which would happen when the task which holds
// softirq_ctrl::lock was the only running task on the CPU and blocks on
// some other lock.
//
#[no_mangle]
pub unsafe extern "C" fn local_bh_blocked() -> bool {
    bool local_bh_blocked(void)
    {
    return __this_cpu_read(softirq_ctrl.cnt) != 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __local_bh_disable_ip(ip: c_ulong, cnt: c_uint) {
    void __local_bh_disable_ip(unsigned long ip, unsigned int cnt)
    {
    unsigned long flags;
    int newcnt;
    WARN_ON_ONCE(in_hardirq());
    lock_map_acquire_read(&bh_lock_map);
// First entry of a task into a BH disabled section?
    if (!current.softirq_disable_cnt) {
    if (preemptible()) {
    if (IS_ENABLED(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK))
    local_lock(&softirq_ctrl.lock);
    else
    migrate_disable();
// Required to meet the RCU bottomhalf requirements.
    rcu_read_lock();
    } else {
    DEBUG_LOCKS_WARN_ON(this_cpu_read(softirq_ctrl.cnt));
    }
    }
//
// Track the per CPU softirq disabled state. On RT this is per CPU
// state to allow preemption of bottom half disabled sections.
//
    if (IS_ENABLED(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK)) {
    newcnt = this_cpu_add_return(softirq_ctrl.cnt, cnt);
//
// Reflect the result in the task state to prevent recursion on the
// local lock and to make softirq_count() & al work.
//
    current.softirq_disable_cnt = newcnt;
    if (IS_ENABLED(CONFIG_TRACE_IRQFLAGS) && newcnt == cnt) {
    raw_local_irq_save(flags);
    lockdep_softirqs_off(ip);
    raw_local_irq_restore(flags);
    }
    } else {
    let mut sirq_dis: bool = false;
    if (!current.softirq_disable_cnt)
    sirq_dis = true;
    this_cpu_add(softirq_ctrl.cnt, cnt);
    current.softirq_disable_cnt += cnt;
    WARN_ON_ONCE(current.softirq_disable_cnt < 0);
    if (IS_ENABLED(CONFIG_TRACE_IRQFLAGS) && sirq_dis) {
    raw_local_irq_save(flags);
    lockdep_softirqs_off(ip);
    raw_local_irq_restore(flags);
    }
    }
    }
    EXPORT_SYMBOL(__local_bh_disable_ip);
#[no_mangle]
unsafe extern "C" fn __local_bh_enable(cnt: c_uint, unlock: bool) {
    static void __local_bh_enable(unsigned int cnt, bool unlock)
    {
    unsigned long flags;
    let mut sirq_en: bool = false;
    int newcnt;
    if (IS_ENABLED(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK)) {
    DEBUG_LOCKS_WARN_ON(current.softirq_disable_cnt !=
    this_cpu_read(softirq_ctrl.cnt));
    if (softirq_count() == cnt)
    sirq_en = true;
    } else {
    if (current.softirq_disable_cnt == cnt)
    sirq_en = true;
    }
    if (IS_ENABLED(CONFIG_TRACE_IRQFLAGS) && sirq_en) {
    raw_local_irq_save(flags);
    lockdep_softirqs_on(_RET_IP_);
    raw_local_irq_restore(flags);
    }
    if (IS_ENABLED(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK)) {
    newcnt = this_cpu_sub_return(softirq_ctrl.cnt, cnt);
    current.softirq_disable_cnt = newcnt;
    if (!newcnt && unlock) {
    rcu_read_unlock();
    local_unlock(&softirq_ctrl.lock);
    }
    } else {
    current.softirq_disable_cnt -= cnt;
    this_cpu_sub(softirq_ctrl.cnt, cnt);
    if (unlock && !current.softirq_disable_cnt) {
    migrate_enable();
    rcu_read_unlock();
    } else {
    WARN_ON_ONCE(current.softirq_disable_cnt < 0);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __local_bh_enable_ip(ip: c_ulong, cnt: c_uint) {
    void __local_bh_enable_ip(unsigned long ip, unsigned int cnt)
    {
    let mut preempt_on: bool = preemptible();
    unsigned long flags;
    u32 pending;
    int curcnt;
    WARN_ON_ONCE(in_hardirq());
    lockdep_assert_irqs_enabled();
    lock_map_release(&bh_lock_map);
    local_irq_save(flags);
    if (IS_ENABLED(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK))
    curcnt = this_cpu_read(softirq_ctrl.cnt);
    else
    curcnt = current.softirq_disable_cnt;
//
// If this is not reenabling soft interrupts, no point in trying to
// run pending ones.
//
    if (curcnt != cnt)
    goto out;
    pending = local_softirq_pending();
    if (!pending)
    goto out;
//
// If this was called from non preemptible context, wake up the
// softirq daemon.
//
    if (!preempt_on) {
    wakeup_softirqd();
    goto out;
    }
//
// Adjust softirq count to SOFTIRQ_OFFSET which makes
// in_serving_softirq() become true.
//
    cnt = SOFTIRQ_OFFSET;
    __local_bh_enable(cnt, false);
    __do_softirq();
    out:
    __local_bh_enable(cnt, preempt_on);
    local_irq_restore(flags);
    }
    EXPORT_SYMBOL(__local_bh_enable_ip);
//
// Invoked from ksoftirqd_run() outside of the interrupt disabled section
// to acquire the per CPU local lock for reentrancy protection.
//
#[no_mangle]
pub unsafe extern "C" fn ksoftirqd_run_begin() {
    static inline void ksoftirqd_run_begin(void)
    {
    __local_bh_disable_ip(_RET_IP_, SOFTIRQ_OFFSET);
    local_irq_disable();
    }
// Counterpart to ksoftirqd_run_begin()
#[no_mangle]
pub unsafe extern "C" fn ksoftirqd_run_end() {
    static inline void ksoftirqd_run_end(void)
    {
// pairs with the lock_map_acquire_read() in ksoftirqd_run_begin()
    lock_map_release(&bh_lock_map);
    __local_bh_enable(SOFTIRQ_OFFSET, true);
    WARN_ON_ONCE(in_interrupt());
    local_irq_enable();
    }
    static inline void softirq_handle_begin(void) { }
    static inline void softirq_handle_end(void) { }
#[no_mangle]
pub unsafe extern "C" fn should_wake_ksoftirqd() -> bool {
    static inline bool should_wake_ksoftirqd(void)
    {
    return !this_cpu_read(softirq_ctrl.cnt);
    }
#[no_mangle]
pub unsafe extern "C" fn invoke_softirq() {
    static inline void invoke_softirq(void)
    {
    if (should_wake_ksoftirqd())
    wakeup_softirqd();
    }

//
// flush_smp_call_function_queue() can raise a soft interrupt in a function
// call. On RT kernels this is undesired and the only known functionalities
// are in the block layer which is disabled on RT, and in the scheduler for
// idle load balancing. If soft interrupts get raised which haven't been
// raised before the flush, warn if it is not a SCHED_SOFTIRQ so it can be
// investigated.
//
#[no_mangle]
pub unsafe extern "C" fn do_softirq_post_smp_call_flush(was_pending: c_uint) {
    void do_softirq_post_smp_call_flush(unsigned int was_pending)
    {
    let mut is_pending: c_uint = local_softirq_pending();
    if (unlikely(was_pending != is_pending)) {
    WARN_ON_ONCE(was_pending != (is_pending & ~SCHED_SOFTIRQ_MASK));
    invoke_softirq();
    }
    }

//
// This one is for softirq.c-internal use, where hardirqs are disabled
// legitimately:
//

#[no_mangle]
pub unsafe extern "C" fn __local_bh_disable_ip(ip: c_ulong, cnt: c_uint) {
    void __local_bh_disable_ip(unsigned long ip, unsigned int cnt)
    {
    unsigned long flags;
    WARN_ON_ONCE(in_hardirq());
    raw_local_irq_save(flags);
//
// The preempt tracer hooks into preempt_count_add and will break
// lockdep because it calls back into lockdep after SOFTIRQ_OFFSET
// is set and before current->softirq_enabled is cleared.
// We must manually increment preempt_count here and manually
// call the trace_preempt_off later.
//
    __preempt_count_add(cnt);
//
// Were softirqs turned off above:
//
    if (softirq_count() == (cnt & SOFTIRQ_MASK))
    lockdep_softirqs_off(ip);
    raw_local_irq_restore(flags);
    if (preempt_count() == cnt) {

    current.preempt_disable_ip = get_lock_parent_ip();

    trace_preempt_off(CALLER_ADDR0, get_lock_parent_ip());
    }
    }
    EXPORT_SYMBOL(__local_bh_disable_ip);

#[no_mangle]
unsafe extern "C" fn __local_bh_enable(cnt: c_uint) {
    static void __local_bh_enable(unsigned int cnt)
    {
    lockdep_assert_irqs_disabled();
    if (preempt_count() == cnt)
    trace_preempt_on(CALLER_ADDR0, get_lock_parent_ip());
    if (softirq_count() == (cnt & SOFTIRQ_MASK))
    lockdep_softirqs_on(_RET_IP_);
    __preempt_count_sub(cnt);
    }
//
// Special-case - softirqs can safely be enabled by __do_softirq(),
// without processing still-pending softirqs:
//
#[no_mangle]
pub unsafe extern "C" fn _local_bh_enable() {
    void _local_bh_enable(void)
    {
    WARN_ON_ONCE(in_hardirq());
    __local_bh_enable(SOFTIRQ_DISABLE_OFFSET);
    }
    EXPORT_SYMBOL(_local_bh_enable);
#[no_mangle]
pub unsafe extern "C" fn __local_bh_enable_ip(ip: c_ulong, cnt: c_uint) {
    void __local_bh_enable_ip(unsigned long ip, unsigned int cnt)
    {
    WARN_ON_ONCE(in_hardirq());
    lockdep_assert_irqs_enabled();

    local_irq_disable();

//
// Are softirqs going to be turned on now:
//
    if (softirq_count() == SOFTIRQ_DISABLE_OFFSET)
    lockdep_softirqs_on(ip);
//
// Keep preemption disabled until we are done with
// softirq processing:
//
    __preempt_count_sub(cnt - 1);
    if (unlikely(!in_interrupt() && local_softirq_pending())) {
//
// Run softirq if any pending. And do it in its own stack
// as we may be calling this deep in a task call stack already.
//
    do_softirq();
    }
    preempt_count_dec();

    local_irq_enable();

    preempt_check_resched();
    }
    EXPORT_SYMBOL(__local_bh_enable_ip);
#[no_mangle]
pub unsafe extern "C" fn softirq_handle_begin() {
    static inline void softirq_handle_begin(void)
    {
    __local_bh_disable_ip(_RET_IP_, SOFTIRQ_OFFSET);
    }
#[no_mangle]
pub unsafe extern "C" fn softirq_handle_end() {
    static inline void softirq_handle_end(void)
    {
    __local_bh_enable(SOFTIRQ_OFFSET);
    WARN_ON_ONCE(in_interrupt());
    }
#[no_mangle]
pub unsafe extern "C" fn ksoftirqd_run_begin() {
    static inline void ksoftirqd_run_begin(void)
    {
    local_irq_disable();
    }
#[no_mangle]
pub unsafe extern "C" fn ksoftirqd_run_end() {
    static inline void ksoftirqd_run_end(void)
    {
    local_irq_enable();
    }
#[no_mangle]
pub unsafe extern "C" fn should_wake_ksoftirqd() -> bool {
    static inline bool should_wake_ksoftirqd(void)
    {
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn invoke_softirq() {
    static inline void invoke_softirq(void)
    {
    if (!force_irqthreads() || !__this_cpu_read(ksoftirqd)) {

//
// We can safely execute softirq on the current stack if
// it is the irq stack, because it should be near empty
// at this stage.
//
    __do_softirq();

//
// Otherwise, irq_exit() is called on the task stack that can
// be potentially deep already. So call softirq in its own stack
// to prevent from any overrun.
//
    do_softirq_own_stack();

    } else {
    wakeup_softirqd();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn do_softirq() -> asmlinkage __visible void {
    asmlinkage __visible void do_softirq(void)
    {
    __u32 pending;
    unsigned long flags;
    if (in_interrupt())
    return;
    local_irq_save(flags);
    pending = local_softirq_pending();
    if (pending)
    do_softirq_own_stack();
    local_irq_restore(flags);
    }

//
// We restart softirq processing for at most MAX_SOFTIRQ_RESTART times,
// but break the loop if need_resched() is set or after 2 ms.
// The MAX_SOFTIRQ_TIME provides a nice upper bound in most cases, but in
// certain cases, such as stop_machine(), jiffies may cease to
// increment and so we need the MAX_SOFTIRQ_RESTART limit as
// well to make sure we eventually return from this method.
//
// These limits have been established via experimentation.
// The two things to balance is latency against fairness -
// we want to handle softirqs as soon as possible, but they
// should not be able to lock up the box.
//

pub const MAX_SOFTIRQ_RESTART: c_int = 10;

//
// When we run softirqs from irq_exit() and thus on the hardirq stack we need
// to keep the lockdep irq context tracking as tight as possible in order to
// not miss-qualify lock contexts and miss possible deadlocks.
//
#[no_mangle]
pub unsafe extern "C" fn lockdep_softirq_start() -> bool {
    static inline bool lockdep_softirq_start(void)
    {
    let mut in_hardirq: bool = false;
    if (lockdep_hardirq_context()) {
    in_hardirq = true;
    lockdep_hardirq_exit();
    }
    lockdep_softirq_enter();
    return in_hardirq;
    }
#[no_mangle]
pub unsafe extern "C" fn lockdep_softirq_end(in_hardirq: bool) {
    static inline void lockdep_softirq_end(bool in_hardirq)
    {
    lockdep_softirq_exit();
    if (in_hardirq)
    lockdep_hardirq_enter();
    }

    static inline bool lockdep_softirq_start(void) { return false; }
    static inline void lockdep_softirq_end(bool in_hardirq) { }

#[no_mangle]
unsafe extern "C" fn handle_softirqs(ksirqd: bool) {
    static void handle_softirqs(bool ksirqd)
    {
    let mut end: c_ulong = jiffies + MAX_SOFTIRQ_TIME;
    let mut old_flags: c_ulong = current.flags;
    let mut max_restart: c_int = MAX_SOFTIRQ_RESTART;
    struct softirq_action *h;
    bool in_hardirq;
    __u32 pending;
    int softirq_bit;
//
// Mask out PF_MEMALLOC as the current task context is borrowed for the
// softirq. A softirq handled, such as network RX, might set PF_MEMALLOC
// again if the socket is related to swapping.
//
    current.flags &= ~PF_MEMALLOC;
    pending = local_softirq_pending();
    softirq_handle_begin();
    in_hardirq = lockdep_softirq_start();
    account_softirq_enter(current);
    restart:
// Reset the pending bitmask before enabling irqs
    set_softirq_pending(0);
    local_irq_enable();
    h = softirq_vec;
    while ((softirq_bit = ffs(pending))) {
    unsigned int vec_nr;
    int prev_count;
    h += softirq_bit - 1;
    vec_nr = h - softirq_vec;
    prev_count = preempt_count();
    kstat_incr_softirqs_this_cpu(vec_nr);
    trace_softirq_entry(vec_nr);
    h.action();
    trace_softirq_exit(vec_nr);
    if (unlikely(prev_count != preempt_count())) {
    pr_err("huh, entered softirq %u %s %p with preempt_count %08x, exited with %08x?\n",
    vec_nr, softirq_to_name[vec_nr], h.action,
    prev_count, preempt_count());
    preempt_count_set(prev_count);
    }
    h++;
    pending >>= softirq_bit;
    }
    if (!IS_ENABLED(CONFIG_PREEMPT_RT) && ksirqd)
    rcu_softirq_qs();
    local_irq_disable();
    pending = local_softirq_pending();
    if (pending) {
    if (time_before(jiffies, end) && !need_resched() &&
    --max_restart)
    goto restart;
    wakeup_softirqd();
    }
    account_softirq_exit(current);
    lockdep_softirq_end(in_hardirq);
    softirq_handle_end();
    current_restore_flags(old_flags, PF_MEMALLOC);
    }
#[no_mangle]
pub unsafe extern "C" fn __do_softirq() -> asmlinkage __visible void __softirq_entry {
    asmlinkage __visible void __softirq_entry __do_softirq(void)
    {
    handle_softirqs(false);
    }
//
// irq_enter_rcu - Enter an interrupt context with RCU watching
//
#[no_mangle]
pub unsafe extern "C" fn irq_enter_rcu() {
    void irq_enter_rcu(void)
    {
    __irq_enter_raw();
//
// If this is a nested interrupt that hits the exit_to_user_mode_loop
// where it has enabled interrupts but before it has hit schedule() we
// could have hrtimers in an undefined state. Fix it up here.
//
    hrtimer_rearm_deferred();
    if (tick_nohz_full_cpu(smp_processor_id()) ||
    (is_idle_task(current) && (irq_count() == HARDIRQ_OFFSET)))
    tick_irq_enter();
    account_hardirq_enter(current);
    }
//
// irq_enter - Enter an interrupt context including RCU update
//
#[no_mangle]
pub unsafe extern "C" fn irq_enter() {
    void irq_enter(void)
    {
    ct_irq_enter();
    irq_enter_rcu();
    }
#[no_mangle]
pub unsafe extern "C" fn tick_irq_exit() {
    static inline void tick_irq_exit(void)
    {

    let mut cpu: c_int = smp_processor_id();
// Make sure that timer wheel updates are propagated
    if ((sched_core_idle_cpu(cpu) && !need_resched()) || tick_nohz_full_cpu(cpu)) {
    if (!in_hardirq())
    tick_nohz_irq_exit();
    }

    }

    DEFINE_PER_CPU(struct task_struct *, ktimerd);
    DEFINE_PER_CPU(unsigned long, pending_timer_softirq);
#[no_mangle]
unsafe extern "C" fn wake_timersd() {
    static void wake_timersd(void)
    {
    struct task_struct *tsk = __this_cpu_read(ktimerd);
    if (tsk)
    wake_up_process(tsk);
    }

    static inline void wake_timersd(void) { }

#[no_mangle]
pub unsafe extern "C" fn __irq_exit_rcu() {
    static inline void __irq_exit_rcu(void)
    {

    local_irq_disable();

    lockdep_assert_irqs_disabled();

    account_hardirq_exit(current);
    preempt_count_sub(HARDIRQ_OFFSET);
//
// Interrupts may happen between hardirq_disable_enter() and
// local_irq_save() in local_interrupt_disable(), if irq_exit() invokes
// softirq here, we may have a softirq handler calling
// local_interrupt_disable() but it won't disable the IRQ because
// hardirq disabling count is already 1, hence we need to prevent
// invoking softirq when a local_interrupt_disable() is ongoing.
//
    if (!in_interrupt() && !hardirq_disable_count() &&
    local_softirq_pending()) {
//
// If we left hrtimers unarmed, make sure to arm them now,
// before enabling interrupts to run softirq.
//
    hrtimer_rearm_deferred();
    invoke_softirq();
    }
    if (IS_ENABLED(CONFIG_IRQ_FORCED_THREADING) && force_irqthreads() &&
    local_timers_pending_force_th() && !(in_nmi() | in_hardirq()))
    wake_timersd();
    tick_irq_exit();
    }
//
// irq_exit_rcu() - Exit an interrupt context without updating RCU
//
// Also processes softirqs if needed and possible.
//
#[no_mangle]
pub unsafe extern "C" fn irq_exit_rcu() {
    void irq_exit_rcu(void)
    {
    __irq_exit_rcu();
// must be last!
    lockdep_hardirq_exit();
    }
//
// irq_exit - Exit an interrupt context, update RCU and lockdep
//
// Also processes softirqs if needed and possible.
//
#[no_mangle]
pub unsafe extern "C" fn irq_exit() {
    void irq_exit(void)
    {
    __irq_exit_rcu();
    ct_irq_exit();
// must be last!
    lockdep_hardirq_exit();
    }
//
// This function must run with irqs disabled!
//
#[no_mangle]
pub unsafe extern "C" fn raise_softirq_irqoff(nr: c_uint) {
    inline void raise_softirq_irqoff(unsigned int nr)
    {
    __raise_softirq_irqoff(nr);
//
// If we're in an interrupt or softirq, we're done
// (this also catches softirq-disabled code). We will
// actually run the softirq once we return from
// the irq or softirq.
//
// Otherwise we wake up ksoftirqd to make sure we
// schedule the softirq soon.
//
    if (!in_interrupt() && should_wake_ksoftirqd())
    wakeup_softirqd();
    }
#[no_mangle]
pub unsafe extern "C" fn raise_softirq(nr: c_uint) {
    void raise_softirq(unsigned int nr)
    {
    unsigned long flags;
    local_irq_save(flags);
    raise_softirq_irqoff(nr);
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn __raise_softirq_irqoff(nr: c_uint) {
    void __raise_softirq_irqoff(unsigned int nr)
    {
    lockdep_assert_irqs_disabled();
    trace_softirq_raise(nr);
    or_softirq_pending(1UL << nr);
    }
#[no_mangle]
pub unsafe extern "C" fn open_softirq(nr: c_int, (*action)(void): *mut c_void) {
    void open_softirq(int nr, void (*action)(void))
    {
    softirq_vec[nr].action = action;
    }
//
// Tasklets
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasklet_head {
    pub head: *mut tasklet_struct,
    pub tail: *mut tasklet_struct,
}

    static DEFINE_PER_CPU(struct tasklet_head, tasklet_vec);
    static DEFINE_PER_CPU(struct tasklet_head, tasklet_hi_vec);
    static void __tasklet_schedule_common(struct tasklet_struct *t,
    struct tasklet_head __percpu *headp,
    unsigned int softirq_nr)
    {
    struct tasklet_head *head;
    unsigned long flags;
    local_irq_save(flags);
    head = this_cpu_ptr(headp);
    t.next = core::ptr::null_mut();
// head->tail = t;
    head.tail = &(t.next);
    raise_softirq_irqoff(softirq_nr);
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn __tasklet_schedule(t: *mut tasklet_struct) {
    void __tasklet_schedule(struct tasklet_struct *t)
    {
    __tasklet_schedule_common(t, &tasklet_vec,
    TASKLET_SOFTIRQ);
    }
    EXPORT_SYMBOL(__tasklet_schedule);
#[no_mangle]
pub unsafe extern "C" fn __tasklet_hi_schedule(t: *mut tasklet_struct) {
    void __tasklet_hi_schedule(struct tasklet_struct *t)
    {
    __tasklet_schedule_common(t, &tasklet_hi_vec,
    HI_SOFTIRQ);
    }
    EXPORT_SYMBOL(__tasklet_hi_schedule);
#[no_mangle]
unsafe extern "C" fn tasklet_clear_sched(t: *mut tasklet_struct) -> bool {
    static bool tasklet_clear_sched(struct tasklet_struct *t)
    {
    if (test_and_clear_wake_up_bit(TASKLET_STATE_SCHED, &t.state))
    return true;
    WARN_ONCE(1, "tasklet SCHED state not set: %s %pS\n",
    t.use_callback ? "callback" : "func",
    t.use_callback ? (void *)t.callback : (void *)t.func);
    return false;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasklet_sync_callback {
    pub cb_lock: spinlock_t,
    pub cb_waiters: core::sync::atomic::AtomicI32,
}

    static DEFINE_PER_CPU(struct tasklet_sync_callback, tasklet_sync_callback) = {
    .cb_lock	= __SPIN_LOCK_UNLOCKED(tasklet_sync_callback.cb_lock),
    .cb_waiters	= ATOMIC_INIT(0),
    };
#[no_mangle]
unsafe extern "C" fn tasklet_lock_callback() {
    static void tasklet_lock_callback(void)
    {
    spin_lock(this_cpu_ptr(&tasklet_sync_callback.cb_lock));
    }
#[no_mangle]
unsafe extern "C" fn tasklet_unlock_callback() {
    static void tasklet_unlock_callback(void)
    {
    spin_unlock(this_cpu_ptr(&tasklet_sync_callback.cb_lock));
    }
#[no_mangle]
unsafe extern "C" fn tasklet_callback_cancel_wait_running() {
    static void tasklet_callback_cancel_wait_running(void)
    {
    struct tasklet_sync_callback *sync_cb = this_cpu_ptr(&tasklet_sync_callback);
    atomic_inc(&sync_cb.cb_waiters);
    spin_lock(&sync_cb.cb_lock);
    atomic_dec(&sync_cb.cb_waiters);
    spin_unlock(&sync_cb.cb_lock);
    }
#[no_mangle]
unsafe extern "C" fn tasklet_callback_sync_wait_running() {
    static void tasklet_callback_sync_wait_running(void)
    {
    struct tasklet_sync_callback *sync_cb = this_cpu_ptr(&tasklet_sync_callback);
    if (atomic_read(&sync_cb.cb_waiters)) {
    spin_unlock(&sync_cb.cb_lock);
    spin_lock(&sync_cb.cb_lock);
    }
    }

    static void tasklet_lock_callback(void) { }
    static void tasklet_unlock_callback(void) { }
    static void tasklet_callback_sync_wait_running(void) { }

    static void tasklet_callback_cancel_wait_running(void) { }

    static void tasklet_action_common(struct tasklet_head *tl_head,
    unsigned int softirq_nr)
    {
    struct tasklet_struct *list;
    local_irq_disable();
    list = tl_head.head;
    tl_head.head = core::ptr::null_mut();
    tl_head.tail = &tl_head.head;
    local_irq_enable();
    tasklet_lock_callback();
    while (list) {
    struct tasklet_struct *t = list;
    list = list.next;
    if (tasklet_trylock(t)) {
    if (!atomic_read(&t.count)) {
    if (tasklet_clear_sched(t)) {
    if (t.use_callback) {
    trace_tasklet_entry(t, t.callback);
    t.callback(t);
    trace_tasklet_exit(t, t.callback);
    } else {
    trace_tasklet_entry(t, t.func);
    t.func(t.data);
    trace_tasklet_exit(t, t.func);
    }
    }
    tasklet_unlock(t);
    tasklet_callback_sync_wait_running();
    continue;
    }
    tasklet_unlock(t);
    }
    local_irq_disable();
    t.next = core::ptr::null_mut();
// tl_head->tail = t;
    tl_head.tail = &t.next;
    __raise_softirq_irqoff(softirq_nr);
    local_irq_enable();
    }
    tasklet_unlock_callback();
    }
#[no_mangle]
unsafe extern "C" fn tasklet_action() -> __latent_entropy void {
    static __latent_entropy void tasklet_action(void)
    {
    workqueue_softirq_action(false);
    tasklet_action_common(this_cpu_ptr(&tasklet_vec), TASKLET_SOFTIRQ);
    }
#[no_mangle]
unsafe extern "C" fn tasklet_hi_action() -> __latent_entropy void {
    static __latent_entropy void tasklet_hi_action(void)
    {
    workqueue_softirq_action(true);
    tasklet_action_common(this_cpu_ptr(&tasklet_hi_vec), HI_SOFTIRQ);
    }
    void tasklet_setup(struct tasklet_struct *t,
    void (*callback)(struct tasklet_struct *))
    {
    t.next = core::ptr::null_mut();
    t.state = 0;
    atomic_set(&t.count, 0);
    t.callback = callback;
    t.use_callback = true;
    t.data = 0;
    }
    EXPORT_SYMBOL(tasklet_setup);
    void tasklet_init(struct tasklet_struct *t,
    void (*func)(unsigned long), unsigned long data)
    {
    t.next = core::ptr::null_mut();
    t.state = 0;
    atomic_set(&t.count, 0);
    t.func = func;
    t.use_callback = false;
    t.data = data;
    }
    EXPORT_SYMBOL(tasklet_init);

//
// Do not use in new code. Waiting for tasklets from atomic contexts is
// error prone and should be avoided.
//
#[no_mangle]
pub unsafe extern "C" fn tasklet_unlock_spin_wait(t: *mut tasklet_struct) {
    void tasklet_unlock_spin_wait(struct tasklet_struct *t)
    {
    while (test_bit(TASKLET_STATE_RUN, &(t).state)) {
    if (IS_ENABLED(CONFIG_PREEMPT_RT)) {
//
// Prevent a live lock when current preempted soft
// interrupt processing or prevents ksoftirqd from
// running.
//
    tasklet_callback_cancel_wait_running();
    } else {
    cpu_relax();
    }
    }
    }
    EXPORT_SYMBOL(tasklet_unlock_spin_wait);

#[no_mangle]
pub unsafe extern "C" fn tasklet_kill(t: *mut tasklet_struct) {
    void tasklet_kill(struct tasklet_struct *t)
    {
    if (in_interrupt())
    pr_notice("Attempt to kill tasklet from interrupt\n");
    wait_on_bit_lock(&t.state, TASKLET_STATE_SCHED, TASK_UNINTERRUPTIBLE);
    tasklet_unlock_wait(t);
    tasklet_clear_sched(t);
    }
    EXPORT_SYMBOL(tasklet_kill);

#[no_mangle]
pub unsafe extern "C" fn tasklet_unlock(t: *mut tasklet_struct) {
    void tasklet_unlock(struct tasklet_struct *t)
    {
    clear_and_wake_up_bit(TASKLET_STATE_RUN, &t.state);
    }
    EXPORT_SYMBOL_GPL(tasklet_unlock);
#[no_mangle]
pub unsafe extern "C" fn tasklet_unlock_wait(t: *mut tasklet_struct) {
    void tasklet_unlock_wait(struct tasklet_struct *t)
    {
    wait_on_bit(&t.state, TASKLET_STATE_RUN, TASK_UNINTERRUPTIBLE);
    }
    EXPORT_SYMBOL_GPL(tasklet_unlock_wait);

#[no_mangle]
pub unsafe extern "C" fn softirq_init() -> void __init {
    void __init softirq_init(void)
    {
    int cpu;
    for_each_possible_cpu(cpu) {
    per_cpu(tasklet_vec, cpu).tail =
    &per_cpu(tasklet_vec, cpu).head;
    per_cpu(tasklet_hi_vec, cpu).tail =
    &per_cpu(tasklet_hi_vec, cpu).head;
    }
    open_softirq(TASKLET_SOFTIRQ, tasklet_action);
    open_softirq(HI_SOFTIRQ, tasklet_hi_action);
    }
#[no_mangle]
unsafe extern "C" fn ksoftirqd_should_run(cpu: c_uint) -> c_int {
    static int ksoftirqd_should_run(unsigned int cpu)
    {
    return local_softirq_pending();
    }
#[no_mangle]
unsafe extern "C" fn run_ksoftirqd(cpu: c_uint) {
    static void run_ksoftirqd(unsigned int cpu)
    {
    ksoftirqd_run_begin();
    if (local_softirq_pending()) {
//
// We can safely run softirq on inline stack, as we are not deep
// in the task stack here.
//
    handle_softirqs(true);
    ksoftirqd_run_end();
    cond_resched();
    return;
    }
    ksoftirqd_run_end();
    }

#[no_mangle]
unsafe extern "C" fn takeover_tasklets(cpu: c_uint) -> c_int {
    static int takeover_tasklets(unsigned int cpu)
    {
    workqueue_softirq_dead(cpu);
// CPU is dead, so no lock needed.
    local_irq_disable();
// Find end, append list for that CPU.
    if (&per_cpu(tasklet_vec, cpu).head != per_cpu(tasklet_vec, cpu).tail) {
// __this_cpu_read(tasklet_vec.tail) = per_cpu(tasklet_vec, cpu).head;
    __this_cpu_write(tasklet_vec.tail, per_cpu(tasklet_vec, cpu).tail);
    per_cpu(tasklet_vec, cpu).head = core::ptr::null_mut();
    per_cpu(tasklet_vec, cpu).tail = &per_cpu(tasklet_vec, cpu).head;
    }
    raise_softirq_irqoff(TASKLET_SOFTIRQ);
    if (&per_cpu(tasklet_hi_vec, cpu).head != per_cpu(tasklet_hi_vec, cpu).tail) {
// __this_cpu_read(tasklet_hi_vec.tail) = per_cpu(tasklet_hi_vec, cpu).head;
    __this_cpu_write(tasklet_hi_vec.tail, per_cpu(tasklet_hi_vec, cpu).tail);
    per_cpu(tasklet_hi_vec, cpu).head = core::ptr::null_mut();
    per_cpu(tasklet_hi_vec, cpu).tail = &per_cpu(tasklet_hi_vec, cpu).head;
    }
    raise_softirq_irqoff(HI_SOFTIRQ);
    local_irq_enable();
    return 0;
    }

    static struct smp_hotplug_thread softirq_threads = {
    .store			= &ksoftirqd,
    .thread_should_run	= ksoftirqd_should_run,
    .thread_fn		= run_ksoftirqd,
    .thread_comm		= "ksoftirqd/%u",
    };

#[no_mangle]
unsafe extern "C" fn ktimerd_setup(cpu: c_uint) {
    static void ktimerd_setup(unsigned int cpu)
    {
// Above SCHED_NORMAL to handle timers before regular tasks.
    sched_set_fifo_low(current);
    }
#[no_mangle]
unsafe extern "C" fn ktimerd_should_run(cpu: c_uint) -> c_int {
    static int ktimerd_should_run(unsigned int cpu)
    {
    return local_timers_pending_force_th();
    }
#[no_mangle]
pub unsafe extern "C" fn raise_ktimers_thread(nr: c_uint) {
    void raise_ktimers_thread(unsigned int nr)
    {
    trace_softirq_raise(nr);
    __this_cpu_or(pending_timer_softirq, BIT(nr));
    }
#[no_mangle]
unsafe extern "C" fn run_ktimerd(cpu: c_uint) {
    static void run_ktimerd(unsigned int cpu)
    {
    unsigned int timer_si;
    ksoftirqd_run_begin();
    timer_si = local_timers_pending_force_th();
    __this_cpu_write(pending_timer_softirq, 0);
    or_softirq_pending(timer_si);
    __do_softirq();
    ksoftirqd_run_end();
    }
    static struct smp_hotplug_thread timer_thread = {
    .store			= &ktimerd,
    .setup			= ktimerd_setup,
    .thread_should_run	= ktimerd_should_run,
    .thread_fn		= run_ktimerd,
    .thread_comm		= "ktimers/%u",
    };

#[no_mangle]
unsafe extern "C" fn spawn_ksoftirqd() -> __init int {
    static __init int spawn_ksoftirqd(void)
    {
    cpuhp_setup_state_nocalls(CPUHP_SOFTIRQ_DEAD, "softirq:dead", core::ptr::null_mut(),
    takeover_tasklets);
    BUG_ON(smpboot_register_percpu_thread(&softirq_threads));

    if (force_irqthreads())
    BUG_ON(smpboot_register_percpu_thread(&timer_thread));

    return 0;
    }
    early_initcall(spawn_ksoftirqd);
//
// [ These __weak aliases are kept in a separate compilation unit, so that
// GCC does not inline them incorrectly. ]
//
#[no_mangle]
pub unsafe extern "C" fn early_irq_init() -> int __init __weak {
    int __init __weak early_irq_init(void)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_probe_nr_irqs() -> int __init __weak {
    int __init __weak arch_probe_nr_irqs(void)
    {
    return NR_IRQS_LEGACY;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_early_irq_init() -> int __init __weak {
    int __init __weak arch_early_irq_init(void)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_dynirq_lower_bound(from: c_uint) -> unsigned int __weak {
    unsigned int __weak arch_dynirq_lower_bound(unsigned int from)
    {
    return from;
    }

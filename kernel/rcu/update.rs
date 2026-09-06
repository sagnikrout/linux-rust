//! Automatically rewritten from C to Rust
//! Source: kernel/rcu/update.c
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
// Read-Copy Update mechanism for mutual exclusion
//
// Copyright IBM Corporation, 2001
//
// Authors: Dipankar Sarma <dipankar@in.ibm.com>
// Manfred Spraul <manfred@colorfullife.com>
//
// Based on the original work by Paul McKenney <paulmck@linux.ibm.com>
// and inputs from Rusty Russell, Andrea Arcangeli and Andi Kleen.
// Papers:
// http://www.rdrop.com/users/paulmck/paper/rclockpdcsproof.pdf
// http://lse.sourceforge.net/locking/rclock_OLS.2001.05.01c.sc.pdf (OLS2001)
//
// For detailed explanation of Read-Copy Update mechanism see -
// http://lse.sourceforge.net/locking/rcupdate.html
//

// Macro flag: #define CREATE_TRACE_POINTS

    module_param(rcu_expedited, int, 0444);
    module_param(rcu_normal, int, 0444);
    let mut rcu_normal_after_boot: static int = IS_ENABLED(CONFIG_PREEMPT_RT);

    module_param(rcu_normal_after_boot, int, 0444);

//
// rcu_read_lock_held_common() - might we be in RCU-sched read-side critical section?
// @ret:	Best guess answer if lockdep cannot be relied on
//
// Returns true if lockdep must be ignored, in which case ``*ret`` contains
// the best guess described below.  Otherwise returns false, in which
// case ``*ret`` tells the caller nothing and the caller should instead
// consult lockdep.
//
// If CONFIG_DEBUG_LOCK_ALLOC is selected, set ``*ret`` to nonzero iff in an
// RCU-sched read-side critical section.  In absence of
// CONFIG_DEBUG_LOCK_ALLOC, this assumes we are in an RCU-sched read-side
// critical section unless it can prove otherwise.  Note that disabling
// of preemption (including disabling irqs) counts as an RCU-sched
// read-side critical section.  This is useful for debug checks in functions
// that required that they be called within an RCU-sched read-side
// critical section.
//
// Check debug_lockdep_rcu_enabled() to prevent false positives during boot
// and while lockdep is disabled.
//
// Note that if the CPU is in the idle loop from an RCU point of view (ie:
// that we are in the section between ct_idle_enter() and ct_idle_exit())
// then rcu_read_lock_held() sets ``*ret`` to false even if the CPU did an
// rcu_read_lock().  The reason for this is that RCU ignores CPUs that are
// in such a section, considering these as in extended quiescent state,
// so such a CPU is effectively never in an RCU read-side critical section
// regardless of what RCU primitives it invokes.  This state of affairs is
// required --- we need to keep an RCU-free window in idle where the CPU may
// possibly enter into low power mode. This way we can notice an extended
// quiescent state to other CPUs that started a grace period. Otherwise
// we would delay any grace period as long as we run in the idle task.
//
// Similarly, we avoid claiming an RCU read lock held if the current
// CPU is offline.
//
#[no_mangle]
unsafe extern "C" fn rcu_read_lock_held_common(ret: *mut bool) -> bool {
    static bool rcu_read_lock_held_common(bool *ret)
    {
    if (!debug_lockdep_rcu_enabled()) {
// ret = true;
    return true;
    }
    if (!rcu_is_watching()) {
// ret = false;
    return true;
    }
    if (!rcu_lockdep_current_cpu_online()) {
// ret = false;
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_sched_held() -> int notrace {
    int notrace rcu_read_lock_sched_held(void)
    {
    bool ret;
    if (rcu_read_lock_held_common(&ret))
    return ret;
    return lock_is_held(&rcu_sched_lock_map) || !preemptible();
    }
    EXPORT_SYMBOL(rcu_read_lock_sched_held);

//
// Should expedited grace-period primitives always fall back to their
// non-expedited counterparts?  Intended for use within RCU.  Note
// that if the user specifies both rcu_expedited and rcu_normal, then
// rcu_normal wins.  (Except during the time period during boot from
// when the first task is spawned until the rcu_set_runtime_mode()
// core_initcall() is invoked, at which point everything is expedited.)
//
#[no_mangle]
pub unsafe extern "C" fn rcu_gp_is_normal() -> bool {
    bool rcu_gp_is_normal(void)
    {
    return READ_ONCE(rcu_normal) &&
    rcu_scheduler_active != RCU_SCHEDULER_INIT;
    }
    EXPORT_SYMBOL_GPL(rcu_gp_is_normal);
    let mut rcu_async_hurry_nesting: static atomic_t = ATOMIC_INIT(1);
//
// Should call_rcu() callbacks be processed with urgency or are
// they OK being executed with arbitrary delays?
//
#[no_mangle]
pub unsafe extern "C" fn rcu_async_should_hurry() -> bool {
    bool rcu_async_should_hurry(void)
    {
    return !IS_ENABLED(CONFIG_RCU_LAZY) ||
    atomic_read(&rcu_async_hurry_nesting);
    }
    EXPORT_SYMBOL_GPL(rcu_async_should_hurry);
//
// rcu_async_hurry - Make future async RCU callbacks not lazy.
//
// After a call to this function, future calls to call_rcu()
// will be processed in a timely fashion.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_async_hurry() {
    void rcu_async_hurry(void)
    {
    if (IS_ENABLED(CONFIG_RCU_LAZY))
    atomic_inc(&rcu_async_hurry_nesting);
    }
    EXPORT_SYMBOL_GPL(rcu_async_hurry);
//
// rcu_async_relax - Make future async RCU callbacks lazy.
//
// After a call to this function, future calls to call_rcu()
// will be processed in a lazy fashion.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_async_relax() {
    void rcu_async_relax(void)
    {
    if (IS_ENABLED(CONFIG_RCU_LAZY))
    atomic_dec(&rcu_async_hurry_nesting);
    }
    EXPORT_SYMBOL_GPL(rcu_async_relax);
    let mut rcu_expedited_nesting: static atomic_t = ATOMIC_INIT(1);
//
// Should normal grace-period primitives be expedited?  Intended for
// use within RCU.  Note that this function takes the rcu_expedited
// sysfs/boot variable and rcu_scheduler_active into account as well
// as the rcu_expedite_gp() nesting.  So looping on rcu_unexpedite_gp()
// until rcu_gp_is_expedited() returns false is a -really- bad idea.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_gp_is_expedited() -> bool {
    bool rcu_gp_is_expedited(void)
    {
    return rcu_expedited || atomic_read(&rcu_expedited_nesting);
    }
    EXPORT_SYMBOL_GPL(rcu_gp_is_expedited);
//
// rcu_expedite_gp - Expedite future RCU grace periods
//
// After a call to this function, future calls to synchronize_rcu() and
// friends act as the corresponding synchronize_rcu_expedited() function
// had instead been called.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_expedite_gp() {
    void rcu_expedite_gp(void)
    {
    atomic_inc(&rcu_expedited_nesting);
    }
    EXPORT_SYMBOL_GPL(rcu_expedite_gp);
//
// rcu_unexpedite_gp - Cancel prior rcu_expedite_gp() invocation
//
// Undo a prior call to rcu_expedite_gp().  If all prior calls to
// rcu_expedite_gp() are undone by a subsequent call to rcu_unexpedite_gp(),
// and if the rcu_expedited sysfs/boot parameter is not set, then all
// subsequent calls to synchronize_rcu() and friends will return to
// their normal non-expedited behavior.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_unexpedite_gp() {
    void rcu_unexpedite_gp(void)
    {
    atomic_dec(&rcu_expedited_nesting);
    }
    EXPORT_SYMBOL_GPL(rcu_unexpedite_gp);
    static bool rcu_boot_ended __read_mostly;
//
// Inform RCU of the end of the in-kernel boot sequence.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_end_inkernel_boot() {
    void rcu_end_inkernel_boot(void)
    {
    rcu_unexpedite_gp();
    rcu_async_relax();
    if (rcu_normal_after_boot)
    WRITE_ONCE(rcu_normal, 1);
    rcu_boot_ended = true;
    }
//
// Let rcutorture know when it is OK to turn it up to eleven.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_inkernel_boot_has_ended() -> bool {
    bool rcu_inkernel_boot_has_ended(void)
    {
    return rcu_boot_ended;
    }
    EXPORT_SYMBOL_GPL(rcu_inkernel_boot_has_ended);

//
// Test each non-SRCU synchronous grace-period wait API.  This is
// useful just after a change in mode for these primitives, and
// during early boot.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_test_sync_prims() {
    void rcu_test_sync_prims(void)
    {
    if (!IS_ENABLED(CONFIG_PROVE_RCU))
    return;
    pr_info("Running RCU synchronous self tests\n");
    synchronize_rcu();
    synchronize_rcu_expedited();
    }

//
// Switch to run-time mode once RCU has fully initialized.
//
#[no_mangle]
unsafe extern "C" fn rcu_set_runtime_mode() -> int __init {
    static int __init rcu_set_runtime_mode(void)
    {
    rcu_test_sync_prims();
    rcu_scheduler_active = RCU_SCHEDULER_RUNNING;
    kfree_rcu_scheduler_running();
    rcu_test_sync_prims();
    return 0;
    }
    core_initcall(rcu_set_runtime_mode);

    static struct lock_class_key rcu_lock_key;
    struct lockdep_map rcu_lock_map = {
    .name = "rcu_read_lock",
    .key = &rcu_lock_key,
    .wait_type_outer = LD_WAIT_FREE,
    .wait_type_inner = LD_WAIT_CONFIG, /* PREEMPT_RT implies PREEMPT_RCU */
    };
    EXPORT_SYMBOL_GPL(rcu_lock_map);
    static struct lock_class_key rcu_bh_lock_key;
    struct lockdep_map rcu_bh_lock_map = {
    .name = "rcu_read_lock_bh",
    .key = &rcu_bh_lock_key,
    .wait_type_outer = LD_WAIT_FREE,
    .wait_type_inner = LD_WAIT_CONFIG, /* PREEMPT_RT makes BH preemptible. */
    };
    EXPORT_SYMBOL_GPL(rcu_bh_lock_map);
    static struct lock_class_key rcu_sched_lock_key;
    struct lockdep_map rcu_sched_lock_map = {
    .name = "rcu_read_lock_sched",
    .key = &rcu_sched_lock_key,
    .wait_type_outer = LD_WAIT_FREE,
    .wait_type_inner = LD_WAIT_SPIN,
    };
    EXPORT_SYMBOL_GPL(rcu_sched_lock_map);
// Tell lockdep when RCU callbacks are being invoked.
    static struct lock_class_key rcu_callback_key;
    struct lockdep_map rcu_callback_map =
    STATIC_LOCKDEP_MAP_INIT("rcu_callback", &rcu_callback_key);
    EXPORT_SYMBOL_GPL(rcu_callback_map);
#[no_mangle]
pub unsafe extern "C" fn debug_lockdep_rcu_enabled() -> noinstr int notrace {
    noinstr int notrace debug_lockdep_rcu_enabled(void)
    {
    return rcu_scheduler_active != RCU_SCHEDULER_INACTIVE && READ_ONCE(debug_locks) &&
    current.lockdep_recursion == 0;
    }
    EXPORT_SYMBOL_GPL(debug_lockdep_rcu_enabled);
//
// rcu_read_lock_held() - might we be in RCU read-side critical section?
//
// If CONFIG_DEBUG_LOCK_ALLOC is selected, returns nonzero iff in an RCU
// read-side critical section.  In absence of CONFIG_DEBUG_LOCK_ALLOC,
// this assumes we are in an RCU read-side critical section unless it can
// prove otherwise.  This is useful for debug checks in functions that
// require that they be called within an RCU read-side critical section.
//
// Checks debug_lockdep_rcu_enabled() to prevent false positives during boot
// and while lockdep is disabled.
//
// Note that rcu_read_lock() and the matching rcu_read_unlock() must
// occur in the same context, for example, it is illegal to invoke
// rcu_read_unlock() in process context if the matching rcu_read_lock()
// was invoked from within an irq handler.
//
// Note that rcu_read_lock() is disallowed if the CPU is either idle or
// offline from an RCU perspective, so check for those as well.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_held() -> int notrace {
    int notrace rcu_read_lock_held(void)
    {
    bool ret;
    if (rcu_read_lock_held_common(&ret))
    return ret;
    return lock_is_held(&rcu_lock_map);
    }
    EXPORT_SYMBOL_GPL(rcu_read_lock_held);
//
// rcu_read_lock_bh_held() - might we be in RCU-bh read-side critical section?
//
// Check for bottom half being disabled, which covers both the
// CONFIG_PROVE_RCU and not cases.  Note that if someone uses
// rcu_read_lock_bh(), but then later enables BH, lockdep (if enabled)
// will show the situation.  This is useful for debug checks in functions
// that require that they be called within an RCU read-side critical
// section.
//
// Check debug_lockdep_rcu_enabled() to prevent false positives during boot.
//
// Note that rcu_read_lock_bh() is disallowed if the CPU is either idle or
// offline from an RCU perspective, so check for those as well.
//
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_bh_held() -> int notrace {
    int notrace rcu_read_lock_bh_held(void)
    {
    bool ret;
    if (rcu_read_lock_held_common(&ret))
    return ret;
    return in_softirq() || irqs_disabled();
    }
    EXPORT_SYMBOL_GPL(rcu_read_lock_bh_held);
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_any_held() -> int notrace {
    int notrace rcu_read_lock_any_held(void)
    {
    bool ret;
    if (rcu_read_lock_held_common(&ret))
    return ret;
    if (lock_is_held(&rcu_lock_map) ||
    lock_is_held(&rcu_bh_lock_map) ||
    lock_is_held(&rcu_sched_lock_map))
    return 1;
    return !preemptible();
    }
    EXPORT_SYMBOL_GPL(rcu_read_lock_any_held);

//
// wakeme_after_rcu() - Callback function to awaken a task after grace period
// @head: Pointer to rcu_head member within rcu_synchronize structure
//
// Awaken the corresponding task now that a grace period has elapsed.
//
#[no_mangle]
pub unsafe extern "C" fn wakeme_after_rcu(head: *mut rcu_head) {
    void wakeme_after_rcu(struct rcu_head *head)
    {
    struct rcu_synchronize *rcu;
    rcu = container_of(head, struct rcu_synchronize, head);
    complete(&rcu.completion);
    }
    EXPORT_SYMBOL_GPL(wakeme_after_rcu);
    void __wait_rcu_gp(bool checktiny, unsigned int state, int n, call_rcu_func_t *crcu_array,
    struct rcu_synchronize *rs_array)
    {
    int i;
    int j;
// Initialize and register callbacks for each crcu_array element.
    for (i = 0; i < n; i++) {
    if (checktiny &&
    (crcu_array[i] == call_rcu)) {
    might_sleep();
    continue;
    }
    for (j = 0; j < i; j++)
    if (crcu_array[j] == crcu_array[i])
    break;
    if (j == i) {
    init_rcu_head_on_stack(&rs_array[i].head);
    init_completion(&rs_array[i].completion);
    (crcu_array[i])(&rs_array[i].head, wakeme_after_rcu);
    }
    }
// Wait for all callbacks to be invoked.
    for (i = 0; i < n; i++) {
    if (checktiny &&
    (crcu_array[i] == call_rcu))
    continue;
    for (j = 0; j < i; j++)
    if (crcu_array[j] == crcu_array[i])
    break;
    if (j == i) {
    wait_for_completion_state(&rs_array[i].completion, state);
    destroy_rcu_head_on_stack(&rs_array[i].head);
    }
    }
    }
    EXPORT_SYMBOL_GPL(__wait_rcu_gp);
#[no_mangle]
pub unsafe extern "C" fn finish_rcuwait(w: *mut rcuwait) {
    void finish_rcuwait(struct rcuwait *w)
    {
    rcu_assign_pointer(w.task, core::ptr::null_mut());
    __set_current_state(TASK_RUNNING);
    }
    EXPORT_SYMBOL_GPL(finish_rcuwait);

#[no_mangle]
pub unsafe extern "C" fn init_rcu_head(head: *mut rcu_head) {
    void init_rcu_head(struct rcu_head *head)
    {
    debug_object_init(head, &rcuhead_debug_descr);
    }
    EXPORT_SYMBOL_GPL(init_rcu_head);
#[no_mangle]
pub unsafe extern "C" fn destroy_rcu_head(head: *mut rcu_head) {
    void destroy_rcu_head(struct rcu_head *head)
    {
    debug_object_free(head, &rcuhead_debug_descr);
    }
    EXPORT_SYMBOL_GPL(destroy_rcu_head);
#[no_mangle]
unsafe extern "C" fn rcuhead_is_static_object(addr: *mut c_void) -> bool {
    static bool rcuhead_is_static_object(void *addr)
    {
    return true;
    }
//
// init_rcu_head_on_stack() - initialize on-stack rcu_head for debugobjects
// @head: pointer to rcu_head structure to be initialized
//
// This function informs debugobjects of a new rcu_head structure that
// has been allocated as an auto variable on the stack.  This function
// is not required for rcu_head structures that are statically defined or
// that are dynamically allocated on the heap.  This function has no
// effect for !CONFIG_DEBUG_OBJECTS_RCU_HEAD kernel builds.
//
#[no_mangle]
pub unsafe extern "C" fn init_rcu_head_on_stack(head: *mut rcu_head) {
    void init_rcu_head_on_stack(struct rcu_head *head)
    {
    debug_object_init_on_stack(head, &rcuhead_debug_descr);
    }
    EXPORT_SYMBOL_GPL(init_rcu_head_on_stack);
//
// destroy_rcu_head_on_stack() - destroy on-stack rcu_head for debugobjects
// @head: pointer to rcu_head structure to be initialized
//
// This function informs debugobjects that an on-stack rcu_head structure
// is about to go out of scope.  As with init_rcu_head_on_stack(), this
// function is not required for rcu_head structures that are statically
// defined or that are dynamically allocated on the heap.  Also as with
// init_rcu_head_on_stack(), this function has no effect for
// !CONFIG_DEBUG_OBJECTS_RCU_HEAD kernel builds.
//
#[no_mangle]
pub unsafe extern "C" fn destroy_rcu_head_on_stack(head: *mut rcu_head) {
    void destroy_rcu_head_on_stack(struct rcu_head *head)
    {
    debug_object_free(head, &rcuhead_debug_descr);
    }
    EXPORT_SYMBOL_GPL(destroy_rcu_head_on_stack);
    const struct debug_obj_descr rcuhead_debug_descr = {
    .name = "rcu_head",
    .is_static_object = rcuhead_is_static_object,
    };
    EXPORT_SYMBOL_GPL(rcuhead_debug_descr);

    void do_trace_rcu_torture_read(const char *rcutorturename, struct rcu_head *rhp,
    unsigned long secs,
    unsigned long c_old, unsigned long c)
    {
    trace_rcu_torture_read(rcutorturename, rhp, secs, c_old, c);
    }
    EXPORT_SYMBOL_GPL(do_trace_rcu_torture_read);

    do { } while (0)

// Get rcutorture access to sched_setaffinity().
#[no_mangle]
pub unsafe extern "C" fn torture_sched_setaffinity(pid: pid_t, in_mask: *const cpumask, dowarn: bool) -> c_long {
    long torture_sched_setaffinity(pid_t pid, const struct cpumask *in_mask, bool dowarn)
    {
    int ret;
    ret = sched_setaffinity(pid, in_mask);
    WARN_ONCE(dowarn && ret, "%s: sched_setaffinity(%d) returned %d\n", __func__, pid, ret);
    return ret;
    }
    EXPORT_SYMBOL_GPL(torture_sched_setaffinity);

// Trivial and stupid grace-period wait.  Defined here so that lockdep
// kernels can find tasklist_lock.
#[no_mangle]
pub unsafe extern "C" fn synchronize_rcu_trivial_preempt() {
    void synchronize_rcu_trivial_preempt(void)
    {
    struct task_struct *g;
    struct task_struct *t;
    smp_mb(); // Order prior accesses before grace-period start.
    rcu_read_lock(); // Protect task list.
    for_each_process_thread(g, t) {
    if (t == current)
    continue;  // Don't deadlock on ourselves!
// Order later rcu_read_lock() on other tasks after QS.
    while (smp_load_acquire(&t.rcu_trivial_preempt_nesting))
    continue;
    }
    rcu_read_unlock();
    }
    EXPORT_SYMBOL_GPL(synchronize_rcu_trivial_preempt);

    int rcu_cpu_stall_notifiers __read_mostly; // !0 = provide stall notifiers (rarely useful)
    EXPORT_SYMBOL_GPL(rcu_cpu_stall_notifiers);

    int rcu_cpu_stall_ftrace_dump __read_mostly;
    module_param(rcu_cpu_stall_ftrace_dump, int, 0644);

    module_param(rcu_cpu_stall_notifiers, int, 0444);

    int rcu_cpu_stall_suppress __read_mostly; // !0 = suppress stall warnings.
    EXPORT_SYMBOL_GPL(rcu_cpu_stall_suppress);
    module_param(rcu_cpu_stall_suppress, int, 0644);
    let mut __read_mostly: int rcu_cpu_stall_timeout = CONFIG_RCU_CPU_STALL_TIMEOUT;
    module_param(rcu_cpu_stall_timeout, int, 0644);
    let mut __read_mostly: int rcu_exp_cpu_stall_timeout = CONFIG_RCU_EXP_CPU_STALL_TIMEOUT;
    module_param(rcu_exp_cpu_stall_timeout, int, 0644);
    let mut __read_mostly: int rcu_cpu_stall_cputime = IS_ENABLED(CONFIG_RCU_CPU_STALL_CPUTIME);
    module_param(rcu_cpu_stall_cputime, int, 0644);
    bool rcu_exp_stall_task_details __read_mostly;
    module_param(rcu_exp_stall_task_details, bool, 0644);

// Suppress boot-time RCU CPU stall warnings and rcutorture writer stall
// warnings.  Also used by rcutorture even if stall warnings are excluded.
    int rcu_cpu_stall_suppress_at_boot __read_mostly; // !0 = suppress boot stalls.
    EXPORT_SYMBOL_GPL(rcu_cpu_stall_suppress_at_boot);
    module_param(rcu_cpu_stall_suppress_at_boot, int, 0444);
//
// get_completed_synchronize_rcu - Return a pre-completed polled state cookie
//
// Returns a value that will always be treated by functions like
// poll_state_synchronize_rcu() as a cookie whose grace period has already
// completed.
//
#[no_mangle]
pub unsafe extern "C" fn get_completed_synchronize_rcu() -> c_ulong {
    unsigned long get_completed_synchronize_rcu(void)
    {
    return RCU_GET_STATE_COMPLETED;
    }
    EXPORT_SYMBOL_GPL(get_completed_synchronize_rcu);

//
// Early boot self test parameters.
//
    static bool rcu_self_test;
    module_param(rcu_self_test, bool, 0444);
    static int rcu_self_test_counter;
#[no_mangle]
unsafe extern "C" fn test_callback(r: *mut rcu_head) {
    static void test_callback(struct rcu_head *r)
    {
    rcu_self_test_counter++;
    pr_info("RCU test callback executed %d\n", rcu_self_test_counter);
    }
    DEFINE_STATIC_SRCU(early_srcu);
    static unsigned long early_srcu_cookie;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct early_boot_kfree_rcu {
    pub rh: rcu_head,
}

#[no_mangle]
unsafe extern "C" fn early_boot_test_call_rcu() {
    static void early_boot_test_call_rcu(void)
    {
    static struct rcu_head head;
    int idx;
    static struct rcu_head shead;
    struct early_boot_kfree_rcu *rhp;
    idx = srcu_down_read(&early_srcu);
    srcu_up_read(&early_srcu, idx);
    call_rcu(&head, test_callback);
    early_srcu_cookie = start_poll_synchronize_srcu(&early_srcu);
    call_srcu(&early_srcu, &shead, test_callback);
    rhp = kmalloc_obj(*rhp);
    if (!WARN_ON_ONCE(!rhp))
    kfree_rcu(rhp, rh);
    }
#[no_mangle]
pub unsafe extern "C" fn rcu_early_boot_tests() {
    void rcu_early_boot_tests(void)
    {
    pr_info("Running RCU self tests\n");
    if (rcu_self_test)
    early_boot_test_call_rcu();
    rcu_test_sync_prims();
    }
#[no_mangle]
unsafe extern "C" fn rcu_verify_early_boot_tests() -> c_int {
    static int rcu_verify_early_boot_tests(void)
    {
    let mut ret: c_int = 0;
    let mut early_boot_test_counter: c_int = 0;
    if (rcu_self_test) {
    early_boot_test_counter++;
    rcu_barrier();
    early_boot_test_counter++;
    srcu_barrier(&early_srcu);
    WARN_ON_ONCE(!poll_state_synchronize_srcu(&early_srcu, early_srcu_cookie));
    cleanup_srcu_struct(&early_srcu);
    }
    if (rcu_self_test_counter != early_boot_test_counter) {
    WARN_ON(1);
    ret = -1;
    }
    return ret;
    }
    late_initcall(rcu_verify_early_boot_tests);

    void rcu_early_boot_tests(void) {}

//
// Print any significant non-default boot-time settings.
//
#[no_mangle]
pub unsafe extern "C" fn rcupdate_announce_bootup_oddness() -> void __init {
    void __init rcupdate_announce_bootup_oddness(void)
    {
    if (rcu_normal)
    pr_info("\tNo expedited grace period (rcu_normal).\n");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: rcu_normal_after_boot) -> else {
    else if (rcu_normal_after_boot)
    pr_info("\tNo expedited grace period (rcu_normal_after_boot).\n");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: rcu_expedited) -> else {
    else if (rcu_expedited)
    pr_info("\tAll grace periods are expedited (rcu_expedited).\n");
    if (rcu_cpu_stall_suppress)
    pr_info("\tRCU CPU stall warnings suppressed (rcu_cpu_stall_suppress).\n");
    if (rcu_cpu_stall_timeout != CONFIG_RCU_CPU_STALL_TIMEOUT)
    pr_info("\tRCU CPU stall warnings timeout set to %d (rcu_cpu_stall_timeout).\n", rcu_cpu_stall_timeout);
    rcu_tasks_bootup_oddness();
    }

//! Automatically rewritten from C to Rust
//! Source: kernel/cpu.c
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
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0
// CPU control.
// (C) 2001, 2002, 2003, 2004 Rusty Russell
//

// Macro flag: #define CREATE_TRACE_POINTS

//
// struct cpuhp_cpu_state - Per cpu hotplug state storage
// @state:	The current cpu state
// @target:	The target state
// @fail:	Current CPU hotplug callback state
// @thread:	Pointer to the hotplug thread
// @should_run:	Thread should execute
// @rollback:	Perform a rollback
// @single:	Single callback invocation
// @bringup:	Single callback bringup or teardown selector
// @node:	Remote CPU node; for multi-instance, do a
// single entry callback for install/remove
// @last:	For multi-instance rollback, remember how far we got
// @cb_state:	The state for a single callback (install/uninstall)
// @result:	Result of the operation
// @ap_sync_state:	State for AP synchronization
// @done_up:	Signal completion to the issuer of the task for cpu-up
// @done_down:	Signal completion to the issuer of the task for cpu-down
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuhp_cpu_state {
    pub state: cpuhp_state,
    pub target: cpuhp_state,
    pub fail: cpuhp_state,

    pub thread: *mut task_struct,
    pub should_run: bool,
    pub rollback: bool,
    pub single: bool,
    pub bringup: bool,
    pub node: *mut hlist_node,
    pub last: *mut hlist_node,
    pub cb_state: cpuhp_state,
    pub result: c_int,
    pub ap_sync_state: core::sync::atomic::AtomicI32,
    pub done_up: completion,
    pub done_down: completion,

}

    static DEFINE_PER_CPU(struct cpuhp_cpu_state, cpuhp_state) = {
    .fail = CPUHP_INVALID,
    };

    let mut cpus_booted_once_mask;

    static struct lockdep_map cpuhp_state_up_map =
// STATIC_LOCKDEP_MAP_INIT;
    static struct lockdep_map cpuhp_state_down_map =
// STATIC_LOCKDEP_MAP_INIT;
#[no_mangle]
pub unsafe extern "C" fn cpuhp_lock_acquire(bringup: bool) {
    lock_map_acquire(bringup ? &cpuhp_state_up_map : &cpuhp_state_down_map);
    }
#[no_mangle]
pub unsafe extern "C" fn cpuhp_lock_release(bringup: bool) {
    lock_map_release(bringup ? &cpuhp_state_up_map : &cpuhp_state_down_map);
    }

#[no_mangle]
pub unsafe extern "C" fn cpuhp_lock_acquire() { }
#[no_mangle]
pub unsafe extern "C" fn cpuhp_lock_release() { }

//
// struct cpuhp_step - Hotplug state machine step
// @name:	Name of the step
// @startup:	Startup function of the step
// @teardown:	Teardown function of the step
// @cant_stop:	Bringup/teardown can't be stopped at this step
// @multi_instance:	State has multiple instances which get added afterwards
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuhp_step {
    pub name: *const c_char,
    union {
    pub cpu): *mut *mut int (single)(unsigned int,
    int		(*multi)(unsigned int cpu,
    pub node): *mut hlist_node,
    pub startup: },
    union {
    pub cpu): *mut *mut int (single)(unsigned int,
    int		(*multi)(unsigned int cpu,
    pub node): *mut hlist_node,
    pub teardown: },
// private:
    pub list: hlist_head,
// public:
    pub cant_stop: bool,
    pub multi_instance: bool,
}
// static DEFINE_MUTEX(cpuhp_state_mutex);
    static struct cpuhp_step cpuhp_hp_states[];
#[no_mangle]
pub unsafe extern "C" fn cpuhp_get_step() {
    return cpuhp_hp_states + state;
    }
#[no_mangle]
unsafe extern "C" fn cpuhp_step_empty(bringup: bool, step: *mut cpuhp_step) -> bool {
    return bringup ? !step.startup.single : !step.teardown.single;
    }
//
// cpuhp_invoke_callback - Invoke the callbacks for a given state
// @cpu:	The cpu for which the callback should be invoked
// @state:	The state to do callbacks for
// @bringup:	True if the bringup callback should be invoked
// @node:	For multi-instance, do a single entry callback for install/remove
// @lastp:	For multi-instance rollback, remember how far we got
//
// Called from cpu hotplug and from the state register machinery.
//
// Return: %0 on success or a negative errno code
//
#[no_mangle]
pub unsafe extern "C" fn cpuhp_invoke_callback() {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, cpu);
    struct cpuhp_step *step = cpuhp_get_step(state);
    int (*cbm)(unsigned int cpu, struct hlist_node *node);
    int (*cb)(unsigned int cpu);
    int ret, cnt, rollback_ret;
    if (st.fail == state) {
    st.fail = CPUHP_INVALID;
    return -EAGAIN;
    }
    if (cpuhp_step_empty(bringup, step)) {
// WARN_ON_ONCE;
    return 0;
    }
    if (!step.multi_instance) {
// WARN_ON_ONCE;
    cb = bringup ? step.startup.single : step.teardown.single;
    trace_cpuhp_enter(cpu, st.target, state, cb);
    ret = cb(cpu);
    trace_cpuhp_exit(cpu, st.state, state, ret);
    return ret;
    }
    cbm = bringup ? step.startup.multi : step.teardown.multi;
// Single invocation for instance add/remove
    if (node) {
// WARN_ON_ONCE;
    trace_cpuhp_multi_enter(cpu, st.target, state, cbm, node);
    ret = cbm(cpu, node);
    trace_cpuhp_exit(cpu, st.state, state, ret);
    return ret;
    }
// State transition. Invoke on all instances
    cnt = 0;
    hlist_for_each(node, &step.list) {
    if (lastp && node == *lastp) {
    break;
    }
    trace_cpuhp_multi_enter(cpu, st.target, state, cbm, node);
    ret = cbm(cpu, node);
    trace_cpuhp_exit(cpu, st.state, state, ret);
    if (ret) {
    if (!lastp) {
    goto err;
    }
// lastp = node;
    return ret;
    }
    cnt++;
    }
    if (lastp) {
// lastp = NULL;
    }
    return 0;
    err:
// Rollback the instances if one failed
    cbm = !bringup ? step.startup.multi : step.teardown.multi;
    if (!cbm) {
    return ret;
    }
    hlist_for_each(node, &step.list) {
    if (!cnt--) {
    break;
    }
    trace_cpuhp_multi_enter(cpu, st.target, state, cbm, node);
    rollback_ret = cbm(cpu, node);
    trace_cpuhp_exit(cpu, st.state, state, rollback_ret);
//
// Rollback must not fail,
//
// WARN_ON_ONCE;
    }
    return ret;
    }
//
// The former STARTING/DYING states, ran with IRQs disabled and must not fail.
//
#[no_mangle]
unsafe extern "C" fn cpuhp_is_atomic_state(state: cpuhp_state) -> bool {
    return CPUHP_AP_IDLE_DEAD <= state && state < CPUHP_AP_ONLINE;
    }

#[no_mangle]
unsafe extern "C" fn cpuhp_is_ap_state(state: cpuhp_state) -> bool {
//
// The extra check for CPUHP_TEARDOWN_CPU is only for documentation
// purposes as that state is handled explicitly in cpu_down.
//
    return state > CPUHP_BRINGUP_CPU && state != CPUHP_TEARDOWN_CPU;
    }
#[no_mangle]
pub unsafe extern "C" fn wait_for_ap_thread(st: *mut cpuhp_cpu_state, bringup: bool) {
    struct completion *done = bringup ? &st.done_up : &st.done_down;
    wait_for_completion(done);
    }
#[no_mangle]
pub unsafe extern "C" fn complete_ap_thread(st: *mut cpuhp_cpu_state, bringup: bool) {
    struct completion *done = bringup ? &st.done_up : &st.done_down;
    complete(done);
    }
// Synchronization state management
    enum cpuhp_sync_state {
    SYNC_STATE_DEAD,
    SYNC_STATE_KICKED,
    SYNC_STATE_SHOULD_DIE,
    SYNC_STATE_ALIVE,
    SYNC_STATE_SHOULD_ONLINE,
    SYNC_STATE_ONLINE,
    };

//
// cpuhp_ap_update_sync_state - Update synchronization state during bringup/teardown
// @state:	The synchronization state to set
//
// No synchronization point. Just update of the synchronization state, but implies
// a full barrier so that the AP changes are visible before the control CPU proceeds.
//
#[no_mangle]
pub unsafe extern "C" fn cpuhp_ap_update_sync_state(state: cpuhp_sync_state) {
    atomic_t *st = this_cpu_ptr(&cpuhp_state.ap_sync_state);
    (void)atomic_xchg(st, state);
    }
    void __weak arch_cpuhp_sync_state_poll(void) { cpu_relax(); }
#[no_mangle]
pub unsafe extern "C" fn cpuhp_wait_for_sync_state() {
    atomic_t *st = per_cpu_ptr(&cpuhp_state.ap_sync_state, cpu);
    ktime_t now, end, start = ktime_get();
    let mut sync = 0;
    end = start + 10ULL * NSEC_PER_SEC;
    sync = atomic_read(st);
    while (1) {
    if (sync == state) {
    if (!atomic_try_cmpxchg(st, &sync, next_state)) {
    continue;
    }
    return true;
    }
    now = ktime_get();
    if (now > end) {
// Timeout. Leave the state unchanged
    return false;
    } else if (now - start < NSEC_PER_MSEC) {
// Poll for one millisecond
    arch_cpuhp_sync_state_poll();
    } else {
    usleep_range(USEC_PER_MSEC, 2 * USEC_PER_MSEC);
    }
    sync = atomic_read(st);
    }
    return true;
    }

#[no_mangle]
pub unsafe extern "C" fn cpuhp_ap_update_sync_state() { }

//
// cpuhp_ap_report_dead - Update synchronization state to DEAD
//
// No synchronization point. Just update of the synchronization state.
//
#[no_mangle]
pub unsafe extern "C" fn cpuhp_ap_report_dead() {
    cpuhp_ap_update_sync_state(SYNC_STATE_DEAD);
    }
    void __weak arch_cpuhp_cleanup_dead_cpu(unsigned int cpu) { }
//
// Late CPU shutdown synchronization point. Cannot use cpuhp_state::done_down
// because the AP cannot issue complete() at this stage.
//
#[no_mangle]
unsafe extern "C" fn cpuhp_bp_sync_dead(cpu: c_uint) {
    atomic_t *st = per_cpu_ptr(&cpuhp_state.ap_sync_state, cpu);
pub static mut sync: c_int = atomic_read(st);
    do {
// CPU can have reported dead already. Don't overwrite that!
    if (sync == SYNC_STATE_DEAD) {
    break;
    }
    } while (!atomic_try_cmpxchg(st, &sync, SYNC_STATE_SHOULD_DIE));
    if (cpuhp_wait_for_sync_state(cpu, SYNC_STATE_DEAD, SYNC_STATE_DEAD)) {
// CPU reached dead state. Invoke the cleanup function
    arch_cpuhp_cleanup_dead_cpu(cpu);
    return;
    }
// No further action possible. Emit message and give up.
    pr_err("CPU%u failed to report dead state\n", cpu);
    }

#[no_mangle]
pub unsafe extern "C" fn cpuhp_bp_sync_dead() { }

//
// cpuhp_ap_sync_alive - Synchronize AP with the control CPU once it is alive
//
// Updates the AP synchronization state to SYNC_STATE_ALIVE and waits
// for the BP to release it.
//
#[no_mangle]
pub unsafe extern "C" fn cpuhp_ap_sync_alive() {
    atomic_t *st = this_cpu_ptr(&cpuhp_state.ap_sync_state);
    cpuhp_ap_update_sync_state(SYNC_STATE_ALIVE);
// Wait for the control CPU to release it.
    while (atomic_read(st) != SYNC_STATE_SHOULD_ONLINE)
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn cpuhp_can_boot_ap(cpu: c_uint) -> bool {
    atomic_t *st = per_cpu_ptr(&cpuhp_state.ap_sync_state, cpu);
pub static mut sync: c_int = atomic_read(st);
    again:
    match (sync) {
    SYNC_STATE_DEAD => {
// CPU is properly dead
    break;
    SYNC_STATE_KICKED => {
// CPU did not come up in previous attempt
    break;
    SYNC_STATE_ALIVE => {
// CPU is stuck cpuhp_ap_sync_alive().
    break;
    _ => {
// CPU failed to report online or dead and is in limbo state.
    return false;
    }
// Prepare for booting
    if (!atomic_try_cmpxchg(st, &sync, SYNC_STATE_KICKED)) {
    goto again;
    }
    return true;
    }
    void __weak arch_cpuhp_cleanup_kick_cpu(unsigned int cpu) { }
//
// Early CPU bringup synchronization point. Cannot use cpuhp_state::done_up
// because the AP cannot issue complete() so early in the bringup.
//
#[no_mangle]
unsafe extern "C" fn cpuhp_bp_sync_alive(cpu: c_uint) -> c_int {
pub static mut ret: c_int = 0;
    if (!IS_ENABLED(CONFIG_HOTPLUG_CORE_SYNC_FULL)) {
    return 0;
    }
    if (!cpuhp_wait_for_sync_state(cpu, SYNC_STATE_ALIVE, SYNC_STATE_SHOULD_ONLINE)) {
    pr_err("CPU%u failed to report alive state\n", cpu);
    ret = -EIO;
    }
// Let the architecture cleanup the kick alive mechanics.
    arch_cpuhp_cleanup_kick_cpu(cpu);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn cpuhp_bp_sync_alive() { return 0; }
#[no_mangle]
pub unsafe extern "C" fn cpuhp_can_boot_ap() { return true; }

// Serializes the updates to cpu_online_mask, cpu_present_mask
// static DEFINE_MUTEX(cpu_add_remove_lock);
    let mut cpuhp_tasks_frozen = 0;
// EXPORT_SYMBOL_GPL;
//
// The following two APIs (cpu_maps_update_begin/done) must be used when
// attempting to serialize the updates to cpu_online_mask & cpu_present_mask.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_maps_update_begin() {
    mutex_lock(&cpu_add_remove_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_maps_update_done() {
    mutex_unlock(&cpu_add_remove_lock);
    }
//
// If set, cpu_up and cpu_down will return -EBUSY and do nothing.
// Should always be manipulated under cpu_add_remove_lock
//
    static int cpu_hotplug_disabled;

// DEFINE_STATIC_PERCPU_RWSEM;
    static bool cpu_hotplug_offline_disabled __ro_after_init;
#[no_mangle]
pub unsafe extern "C" fn cpus_read_lock() {
    percpu_down_read(&cpu_hotplug_lock);
    }
// EXPORT_SYMBOL_GPL;
#[no_mangle]
pub unsafe extern "C" fn cpus_read_trylock() -> c_int {
    return percpu_down_read_trylock(&cpu_hotplug_lock);
    }
// EXPORT_SYMBOL_GPL;
#[no_mangle]
pub unsafe extern "C" fn cpus_read_unlock() {
    percpu_up_read(&cpu_hotplug_lock);
    }
// EXPORT_SYMBOL_GPL;
#[no_mangle]
pub unsafe extern "C" fn cpus_write_lock() {
    percpu_down_write(&cpu_hotplug_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn cpus_write_unlock() {
    percpu_up_write(&cpu_hotplug_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn lockdep_assert_cpus_held() {
//
// We can't have hotplug operations before userspace starts running,
// and some init codepaths will knowingly not take the hotplug lock.
// This is all valid, so mute lockdep until it makes sense to report
// unheld locks.
//
    if (system_state < SYSTEM_RUNNING) {
    return;
    }
    percpu_rwsem_assert_held(&cpu_hotplug_lock);
    }
// EXPORT_SYMBOL_GPL;

#[no_mangle]
pub unsafe extern "C" fn lockdep_is_cpus_held() -> c_int {
    return percpu_rwsem_is_held(&cpu_hotplug_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn lockdep_is_cpus_write_held() -> c_int {
    return percpu_rwsem_is_write_held(&cpu_hotplug_lock);
    }

#[no_mangle]
unsafe extern "C" fn lockdep_acquire_cpus_lock() {
    rwsem_acquire(&cpu_hotplug_lock.dep_map, 0, 0, _THIS_IP_);
    }
#[no_mangle]
unsafe extern "C" fn lockdep_release_cpus_lock() {
    rwsem_release(&cpu_hotplug_lock.dep_map, _THIS_IP_);
    }
// Declare CPU offlining not supported
#[no_mangle]
pub unsafe extern "C" fn cpu_hotplug_disable_offlining() {
    cpu_maps_update_begin();
    cpu_hotplug_offline_disabled = true;
    cpu_maps_update_done();
    }
//
// Wait for currently running CPU hotplug operations to complete (if any) and
// disable future CPU hotplug (from sysfs). The 'cpu_add_remove_lock' protects
// the 'cpu_hotplug_disabled' flag. The same lock is also acquired by the
// hotplug path before performing hotplug operations. So acquiring that lock
// guarantees mutual exclusion from any currently running hotplug operations.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_hotplug_disable() {
    cpu_maps_update_begin();
    cpu_hotplug_disabled++;
    cpu_maps_update_done();
    }
// EXPORT_SYMBOL_GPL;
#[no_mangle]
unsafe extern "C" fn __cpu_hotplug_enable() {
    if (WARN_ONCE(!cpu_hotplug_disabled, "Unbalanced cpu hotplug enable\n")) {
    return;
    }
    cpu_hotplug_disabled--;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_hotplug_enable() {
    cpu_maps_update_begin();
    __cpu_hotplug_enable();
    cpu_maps_update_done();
    }
// EXPORT_SYMBOL_GPL;

#[no_mangle]
unsafe extern "C" fn lockdep_acquire_cpus_lock() {
    }
#[no_mangle]
unsafe extern "C" fn lockdep_release_cpus_lock() {
    }

//
// Architectures that need SMT-specific errata handling during SMT hotplug
// should override this.
//
    void __weak arch_smt_update(void) { }

pub static mut __read_mostly: cpuhp_smt_control cpu_smt_control = CPU_SMT_ENABLED;
    static unsigned int cpu_smt_max_threads __ro_after_init;
pub static mut __read_mostly: unsigned int cpu_smt_num_threads = UINT_MAX;
#[no_mangle]
pub unsafe extern "C" fn cpu_smt_disable(force: bool) -> c_int {
    if (!cpu_smt_possible()) {
    return;
    }
    if (force) {
    pr_info("SMT: Force disabled\n");
    cpu_smt_control = CPU_SMT_FORCE_DISABLED;
    } else {
    pr_info("SMT: disabled\n");
    cpu_smt_control = CPU_SMT_DISABLED;
    }
    cpu_smt_num_threads = 1;
    }
//
// The decision whether SMT is supported can only be done after the full
// CPU identification. Called from architecture code.
//
    void __init cpu_smt_set_num_threads(unsigned int num_threads,
    unsigned int max_threads)
    {
// WARN_ON;
    if (max_threads == 1) {
    cpu_smt_control = CPU_SMT_NOT_SUPPORTED;
    }
    cpu_smt_max_threads = max_threads;
//
// If SMT has been disabled via the kernel command line or SMT is
// not supported, set cpu_smt_num_threads to 1 for consistency.
// If enabled, take the architecture requested number of threads
// to bring up into account.
//
    if (cpu_smt_control != CPU_SMT_ENABLED) {
    cpu_smt_num_threads = 1;
    }
#[no_mangle]
pub unsafe extern "C" fn if(cpu_smt_num_threads: num_threads <) -> else {
    else if (num_threads < cpu_smt_num_threads)
    cpu_smt_num_threads = num_threads;
    }
#[no_mangle]
unsafe extern "C" fn smt_cmdline_disable(str: *mut c_char) -> c_int {
    cpu_smt_disable(str && !strcmp(str, "force"));
    return 0;
    }
    early_param("nosmt", smt_cmdline_disable);
//
// For Archicture supporting partial SMT states check if the thread is allowed.
// Otherwise this has already been checked through cpu_smt_max_threads when
// setting the SMT level.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_smt_thread_allowed(cpu: c_uint) -> bool {

    return topology_smt_thread_allowed(cpu);

    return true;

    }
#[no_mangle]
pub unsafe extern "C" fn cpu_bootable(cpu: c_uint) -> bool {
    if (cpu_smt_control == CPU_SMT_ENABLED && cpu_smt_thread_allowed(cpu)) {
    return true;
    }
// All CPUs are bootable if controls are not configured
    if (cpu_smt_control == CPU_SMT_NOT_IMPLEMENTED) {
    return true;
    }
// All CPUs are bootable if CPU is not SMT capable
    if (cpu_smt_control == CPU_SMT_NOT_SUPPORTED) {
    return true;
    }
    if (topology_is_primary_thread(cpu)) {
    return true;
    }
//
// On x86 it's required to boot all logical CPUs at least once so
// that the init code can get a chance to set CR4.MCE on each
// CPU. Otherwise, a broadcasted MCE observing CR4.MCE=0b on any
// core will shutdown the machine.
//
    return !cpumask_test_cpu(cpu, &cpus_booted_once_mask);
    }
// Returns true if SMT is supported and not forcefully (irreversibly) disabled
#[no_mangle]
pub unsafe extern "C" fn cpu_smt_possible() -> bool {
    return cpu_smt_control != CPU_SMT_FORCE_DISABLED &&
    cpu_smt_control != CPU_SMT_NOT_SUPPORTED;
    }
// EXPORT_SYMBOL_GPL;

#[no_mangle]
pub unsafe extern "C" fn cpu_bootable() { return true; }

    static inline enum cpuhp_state
    cpuhp_set_state(int cpu, struct cpuhp_cpu_state *st, enum cpuhp_state target)
    {
pub static mut prev_state: cpuhp_state = st.state;
pub static mut bringup: bool = st.state < target;
    st.rollback = false;
    st.last = core::ptr::null_mut();
    st.target = target;
    st.single = false;
    st.bringup = bringup;
    if (cpu_dying(cpu) != !bringup) {
    set_cpu_dying(cpu, !bringup);
    }
    return prev_state;
    }
#[no_mangle]
pub unsafe extern "C" fn cpuhp_reset_state() {
pub static mut bringup: bool = !st.bringup;
    st.target = prev_state;
//
// Already rolling back. No need invert the bringup value or to change
// the current state.
//
    if (st.rollback) {
    return;
    }
    st.rollback = true;
//
// If we have st->last we need to undo partial multi_instance of this
// state first. Otherwise start undo at the previous state.
//
    if (!st.last) {
    if (st.bringup) {
    st.state--;
    }
    else {
    st.state++;
    }
    }
    st.bringup = bringup;
    if (cpu_dying(cpu) != !bringup) {
    set_cpu_dying(cpu, !bringup);
    }
    }
// Regular hotplug invocation of the AP hotplug thread
#[no_mangle]
unsafe extern "C" fn __cpuhp_kick_ap(st: *mut cpuhp_cpu_state) {
    if (!st.single && st.state == st.target) {
    return;
    }
    st.result = 0;
//
// Make sure the above stores are visible before should_run becomes
// true. Paired with the mb() above in cpuhp_thread_fun()
//
    smp_mb();
    st.should_run = true;
    wake_up_process(st.thread);
    wait_for_ap_thread(st, st.bringup);
    }
#[no_mangle]
pub unsafe extern "C" fn cpuhp_kick_ap() {
    enum cpuhp_state prev_state;
    let mut ret = 0;
    prev_state = cpuhp_set_state(cpu, st, target);
    __cpuhp_kick_ap(st);
    if ((ret = st.result)) {
    cpuhp_reset_state(cpu, st, prev_state);
    __cpuhp_kick_ap(st);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bringup_wait_for_ap_online(cpu: c_uint) -> c_int {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, cpu);
// Wait for the CPU to reach CPUHP_AP_ONLINE_IDLE
    wait_for_ap_thread(st, true);
    if (WARN_ON_ONCE((!cpu_online(cpu)))) {
    return -ECANCELED;
    }
// Unpark the hotplug thread of the target cpu
    kthread_unpark(st.thread);
//
// SMT soft disabling on X86 requires to bring the CPU out of the
// BIOS 'wait for SIPI' state in order to set the CR4.MCE bit.  The
// CPU marked itself as booted_once in notify_cpu_starting() so the
// cpu_bootable() check will now return false if this is not the
// primary sibling.
//
    if (!cpu_bootable(cpu)) {
    return -ECANCELED;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn cpuhp_kick_ap_alive(cpu: c_uint) -> c_int {
    if (!cpuhp_can_boot_ap(cpu)) {
    return -EAGAIN;
    }
    return arch_cpuhp_kick_ap_alive(cpu, idle_thread_get(cpu));
    }
#[no_mangle]
unsafe extern "C" fn cpuhp_bringup_ap(cpu: c_uint) -> c_int {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, cpu);
    let mut ret = 0;
//
// Some architectures have to walk the irq descriptors to
// setup the vector space for the cpu which comes online.
// Prevent irq alloc/free across the bringup.
//
    irq_lock_sparse();
    ret = cpuhp_bp_sync_alive(cpu);
    if (ret) {
    goto out_unlock;
    }
    ret = bringup_wait_for_ap_online(cpu);
    if (ret) {
    goto out_unlock;
    }
    irq_unlock_sparse();
    if (st.target <= CPUHP_AP_ONLINE_IDLE) {
    return 0;
    }
    return cpuhp_kick_ap(cpu, st, st.target);
    out_unlock:
    irq_unlock_sparse();
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn bringup_cpu(cpu: c_uint) -> c_int {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, cpu);
    struct task_struct *idle = idle_thread_get(cpu);
    let mut ret = 0;
    if (!cpuhp_can_boot_ap(cpu)) {
    return -EAGAIN;
    }
//
// Some architectures have to walk the irq descriptors to
// setup the vector space for the cpu which comes online.
//
// Prevent irq alloc/free across the bringup by acquiring the
// sparse irq lock. Hold it until the upcoming CPU completes the
// startup in cpuhp_online_idle() which allows to avoid
// intermediate synchronization points in the architecture code.
//
    irq_lock_sparse();
    ret = __cpu_up(cpu, idle);
    if (ret) {
    goto out_unlock;
    }
    ret = cpuhp_bp_sync_alive(cpu);
    if (ret) {
    goto out_unlock;
    }
    ret = bringup_wait_for_ap_online(cpu);
    if (ret) {
    goto out_unlock;
    }
    irq_unlock_sparse();
    if (st.target <= CPUHP_AP_ONLINE_IDLE) {
    return 0;
    }
    return cpuhp_kick_ap(cpu, st, st.target);
    out_unlock:
    irq_unlock_sparse();
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn finish_cpu(cpu: c_uint) -> c_int {
    struct task_struct *idle = idle_thread_get(cpu);
    struct mm_struct *mm = idle.active_mm;
//
// sched_force_init_mm() ensured the use of &init_mm,
// drop that refcount now that the CPU has stopped.
//
// WARN_ON;
    idle.active_mm = core::ptr::null_mut();
    mmdrop_lazy_tlb(mm);
    return 0;
    }
//
// Hotplug state machine related functions
//
// Get the next state to run. Empty ones will be skipped. Returns true if a
// state must be run.
//
// st->state will be modified ahead of time, to match state_to_run, as if it
// has already ran.
//
#[no_mangle]
pub unsafe extern "C" fn cpuhp_next_state() {
    do {
    if (bringup) {
    if (st.state >= target) {
    return false;
    }
// state_to_run = ++st->state;
    } else {
    if (st.state <= target) {
    return false;
    }
// state_to_run = st->state--;
    }
    if (!cpuhp_step_empty(bringup, cpuhp_get_step(*state_to_run))) {
    break;
    }
    } while (true);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn __cpuhp_invoke_callback_range() {
    enum cpuhp_state state;
pub static mut ret: c_int = 0;
    while (cpuhp_next_state(bringup, &state, st, target)) {
    let mut err = 0;
    err = cpuhp_invoke_callback(cpu, state, bringup, core::ptr::null_mut(), core::ptr::null_mut());
    if (!err) {
    continue;
    }
    if (nofail) {
    pr_warn("CPU %u %s state %s (%d) failed (%d)\n",
    cpu, bringup ? "UP" : "DOWN",
    cpuhp_get_step(st.state).name,
    st.state, err);
    ret = -1;
    } else {
    ret = err;
    break;
    }
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cpuhp_invoke_callback_range() {
    return __cpuhp_invoke_callback_range(bringup, cpu, st, target, false);
    }
#[no_mangle]
pub unsafe extern "C" fn cpuhp_invoke_callback_range_nofail() {
    __cpuhp_invoke_callback_range(bringup, cpu, st, target, true);
    }
#[no_mangle]
pub unsafe extern "C" fn can_rollback_cpu(st: *mut cpuhp_cpu_state) -> bool {
    if (IS_ENABLED(CONFIG_HOTPLUG_CPU)) {
    return true;
    }
//
// When CPU hotplug is disabled, then taking the CPU down is not
// possible because takedown_cpu() and the architecture and
// subsystem specific mechanisms are not available. So the CPU
// which would be completely unplugged again needs to stay around
// in the current state.
//
    return st.state <= CPUHP_BRINGUP_CPU;
    }
#[no_mangle]
pub unsafe extern "C" fn cpuhp_up_callbacks() {
pub static mut prev_state: cpuhp_state = st.state;
pub static mut ret: c_int = 0;
    ret = cpuhp_invoke_callback_range(true, cpu, st, target);
    if (ret) {
    pr_debug("CPU UP failed (%d) CPU %u state %s (%d)\n",
    ret, cpu, cpuhp_get_step(st.state).name,
    st.state);
    cpuhp_reset_state(cpu, st, prev_state);
    if (can_rollback_cpu(st)) {
    WARN_ON(cpuhp_invoke_callback_range(false, cpu, st,
    prev_state));
    }
    }
    return ret;
    }
//
// The cpu hotplug threads manage the bringup and teardown of the cpus
//
#[no_mangle]
unsafe extern "C" fn cpuhp_should_run(cpu: c_uint) -> c_int {
    struct cpuhp_cpu_state *st = this_cpu_ptr(&cpuhp_state);
    return st.should_run;
    }
//
// Execute teardown/startup callbacks on the plugged cpu. Also used to invoke
// callbacks when a state gets [un]installed at runtime.
//
// Each invocation of this function by the smpboot thread does a single AP
// state callback.
//
// It has 3 modes of operation:
// - single: runs st->cb_state
// - up:     runs ++st->state, while st->state < st->target
// - down:   runs st->state--, while st->state > st->target
//
// When complete or on error, should_run is cleared and the completion is fired.
//
#[no_mangle]
unsafe extern "C" fn cpuhp_thread_fun(cpu: c_uint) {
    struct cpuhp_cpu_state *st = this_cpu_ptr(&cpuhp_state);
pub static mut bringup: bool = st.bringup;
    enum cpuhp_state state;
    if (WARN_ON_ONCE(!st.should_run)) {
    return;
    }
//
// ACQUIRE for the cpuhp_should_run() load of ->should_run. Ensures
// that if we see ->should_run we also see the rest of the state.
//
    smp_mb();
//
// The BP holds the hotplug lock, but we're now running on the AP,
// ensure that anybody asserting the lock is held, will actually find
// it so.
//
    lockdep_acquire_cpus_lock();
    cpuhp_lock_acquire(bringup);
    if (st.single) {
    state = st.cb_state;
    st.should_run = false;
    } else {
    st.should_run = cpuhp_next_state(bringup, &state, st, st.target);
    if (!st.should_run) {
    goto end;
    }
    }
// WARN_ON_ONCE;
    if (cpuhp_is_atomic_state(state)) {
    local_irq_disable();
    st.result = cpuhp_invoke_callback(cpu, state, bringup, st.node, &st.last);
    local_irq_enable();
//
// STARTING/DYING must not fail!
//
// WARN_ON_ONCE;
    } else {
    st.result = cpuhp_invoke_callback(cpu, state, bringup, st.node, &st.last);
    }
    if (st.result) {
//
// If we fail on a rollback, we're up a creek without no
// paddle, no way forward, no way back. We loose, thanks for
// playing.
//
// WARN_ON_ONCE;
    st.should_run = false;
    }
    end:
    cpuhp_lock_release(bringup);
    lockdep_release_cpus_lock();
    if (!st.should_run) {
    complete_ap_thread(st, bringup);
    }
    }
// Invoke a single callback on a remote cpu
#[no_mangle]
pub unsafe extern "C" fn cpuhp_invoke_ap_callback() {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, cpu);
    let mut ret = 0;
    if (!cpu_online(cpu)) {
    return 0;
    }
    cpuhp_lock_acquire(false);
    cpuhp_lock_release(false);
    cpuhp_lock_acquire(true);
    cpuhp_lock_release(true);
//
// If we are up and running, use the hotplug thread. For early calls
// we invoke the thread function directly.
//
    if (!st.thread) {
    return cpuhp_invoke_callback(cpu, state, bringup, node, core::ptr::null_mut());
    }
    st.rollback = false;
    st.last = core::ptr::null_mut();
    st.node = node;
    st.bringup = bringup;
    st.cb_state = state;
    st.single = true;
    __cpuhp_kick_ap(st);
//
// If we failed and did a partial, do a rollback.
//
    if ((ret = st.result) && st.last) {
    st.rollback = true;
    st.bringup = !bringup;
    __cpuhp_kick_ap(st);
    }
//
// Clean up the leftovers so the next hotplug operation wont use stale
// data.
//
    st.node = st.last = core::ptr::null_mut();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cpuhp_kick_ap_work(cpu: c_uint) -> c_int {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, cpu);
pub static mut prev_state: cpuhp_state = st.state;
    let mut ret = 0;
    cpuhp_lock_acquire(false);
    cpuhp_lock_release(false);
    cpuhp_lock_acquire(true);
    cpuhp_lock_release(true);
    trace_cpuhp_enter(cpu, st.target, prev_state, cpuhp_kick_ap_work);
    ret = cpuhp_kick_ap(cpu, st, st.target);
    trace_cpuhp_exit(cpu, st.state, prev_state, ret);
    return ret;
    }
pub static mut smp_hotplug_thread: usize = 0;
#[no_mangle]
unsafe extern "C" fn cpuhp_init_state() -> __init void {
    let mut st = core::ptr::null_mut();
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    st = per_cpu_ptr(&cpuhp_state, cpu);
    init_completion(&st.done_up);
    init_completion(&st.done_down);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cpuhp_threads_init() -> c_int {
    cpuhp_init_state();
// BUG_ON;
    kthread_unpark(this_cpu_read(cpuhp_state.thread));
    }

//
// clear_tasks_mm_cpumask - Safely clear tasks' mm_cpumask for a CPU
// @cpu: a CPU id
//
// This function walks all processes, finds a valid mm struct for each one and
// then clears a corresponding bit in mm's cpumask.  While this all sounds
// trivial, there are various non-obvious corner cases, which this function
// tries to solve in a safe manner.
//
// Also note that the function uses a somewhat relaxed locking scheme, so it may
// be called only for an already offlined CPU.
//
#[no_mangle]
pub unsafe extern "C" fn clear_tasks_mm_cpumask(cpu: c_int) {
    let mut p = core::ptr::null_mut();
//
// This function is called after the cpu is taken down and marked
// offline, so its not like new tasks will ever get this cpu set in
// their mm mask. -- Peter Zijlstra
// Thus, we may use rcu_read_lock() here, instead of grabbing
// full-fledged tasklist_lock.
//
// WARN_ON;
    rcu_read_lock();
    for_each_process(p) {
    let mut t = core::ptr::null_mut();
//
// Main thread might exit, but other threads may still have
// a valid mm. Find one.
//
    t = find_lock_task_mm(p);
    if (!t) {
    continue;
    }
    arch_clear_mm_cpumask_cpu(cpu, t.mm);
    task_unlock(t);
    }
    rcu_read_unlock();
    }
// Take this CPU down.
#[no_mangle]
unsafe extern "C" fn take_cpu_down(_param: *mut c_void) -> c_int {
    struct cpuhp_cpu_state *st = this_cpu_ptr(&cpuhp_state);
pub static mut target: cpuhp_state = max((int)st.target, CPUHP_AP_OFFLINE);
    int err, cpu = smp_processor_id();
// Ensure this CPU doesn't handle any more interrupts.
    err = __cpu_disable();
    if (err < 0) {
    return err;
    }
//
// Must be called from CPUHP_TEARDOWN_CPU, which means, as we are going
// down, that the current state is CPUHP_TEARDOWN_CPU - 1.
//
// WARN_ON;
//
// Invoke the former CPU_DYING callbacks. DYING must not fail!
//
    cpuhp_invoke_callback_range_nofail(false, cpu, st, target);
// Park the stopper thread
    stop_machine_park(cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn takedown_cpu(cpu: c_uint) -> c_int {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, cpu);
    let mut err = 0;
// Park the smpboot threads
    kthread_park(st.thread);
//
// Prevent irq alloc/free while the dying cpu reorganizes the
// interrupt affinities.
//
    irq_lock_sparse();
    err = stop_machine_cpuslocked(take_cpu_down, core::ptr::null_mut(), cpumask_of(cpu));
    if (err) {
// CPU refused to die
    irq_unlock_sparse();
// Unpark the hotplug thread so we can rollback there
    kthread_unpark(st.thread);
    return err;
    }
// BUG_ON;
//
// The teardown callback for CPUHP_AP_SCHED_STARTING will have removed
// all runnable tasks from the CPU, there's only the idle task left now
// that the migration thread is done doing the stop_machine thing.
//
// Wait for the stop thread to go away.
//
    wait_for_ap_thread(st, false);
// BUG_ON;
// Interrupts are moved away from the dying cpu, reenable alloc/free
    irq_unlock_sparse();
    hotplug_cpu__broadcast_tick_pull(cpu);
// This actually kills the CPU.
    __cpu_die(cpu);
    cpuhp_bp_sync_dead(cpu);
    lockdep_cleanup_dead_cpu(cpu, idle_thread_get(cpu));
//
// Callbacks must be re-integrated right away to the RCU state machine.
// Otherwise an RCU callback could block a further teardown function
// waiting for its completion.
//
    rcutree_migrate_callbacks(cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpuhp_complete_idle_dead(arg: *mut c_void) {
    struct cpuhp_cpu_state *st = arg;
    complete_ap_thread(st, false);
    }
#[no_mangle]
pub unsafe extern "C" fn cpuhp_report_idle_dead() {
    struct cpuhp_cpu_state *st = this_cpu_ptr(&cpuhp_state);
// BUG_ON;
    tick_assert_timekeeping_handover();
    rcutree_report_cpu_dead();
    st.state = CPUHP_AP_IDLE_DEAD;
//
// We cannot call complete after rcutree_report_cpu_dead() so we delegate it
// to an online cpu.
//
    smp_call_function_single(cpumask_first(cpu_online_mask),
    cpuhp_complete_idle_dead, st, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn cpuhp_down_callbacks() {
pub static mut prev_state: cpuhp_state = st.state;
pub static mut ret: c_int = 0;
    ret = cpuhp_invoke_callback_range(false, cpu, st, target);
    if (ret) {
    pr_debug("CPU DOWN failed (%d) CPU %u state %s (%d)\n",
    ret, cpu, cpuhp_get_step(st.state).name,
    st.state);
    cpuhp_reset_state(cpu, st, prev_state);
    if (st.state < prev_state) {
    WARN_ON(cpuhp_invoke_callback_range(true, cpu, st,
    prev_state));
    }
    }
    return ret;
    }
// Requires cpu_add_remove_lock to be held
    static int __ref _cpu_down(unsigned int cpu, int tasks_frozen,
    enum cpuhp_state target)
    {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, cpu);
    int prev_state, ret = 0;
    if (num_online_cpus() == 1) {
    return -EBUSY;
    }
    if (!cpu_present(cpu)) {
    return -EINVAL;
    }
    cpus_write_lock();
//
// Keep at least one housekeeping cpu onlined to avoid generating
// an empty sched_domain span.
//
    if (cpumask_any_and(cpu_online_mask,
    housekeeping_cpumask(HK_TYPE_DOMAIN)) >= nr_cpu_ids) {
    ret = -EBUSY;
    goto out;
    }
    cpuhp_tasks_frozen = tasks_frozen;
    prev_state = cpuhp_set_state(cpu, st, target);
//
// If the current CPU state is in the range of the AP hotplug thread,
// then we need to kick the thread.
//
    if (st.state > CPUHP_TEARDOWN_CPU) {
    st.target = max((int)target, CPUHP_TEARDOWN_CPU);
    ret = cpuhp_kick_ap_work(cpu);
//
// The AP side has done the error rollback already. Just
// return the error code..
//
    if (ret) {
    goto out;
    }
//
// We might have stopped still in the range of the AP hotplug
// thread. Nothing to do anymore.
//
    if (st.state > CPUHP_TEARDOWN_CPU) {
    goto out;
    }
    st.target = target;
    }
//
// The AP brought itself down to CPUHP_TEARDOWN_CPU. So we need
// to do the further cleanups.
//
    ret = cpuhp_down_callbacks(cpu, st, target);
    if (ret && st.state < prev_state) {
    if (st.state == CPUHP_TEARDOWN_CPU) {
    cpuhp_reset_state(cpu, st, prev_state);
    __cpuhp_kick_ap(st);
    } else {
// WARN;
    }
    }
    out:
    cpus_write_unlock();
    arch_smt_update();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cpu_down_maps_locked(cpu: c_uint, target: cpuhp_state) -> c_int {
//
// If the platform does not support hotplug, report it explicitly to
// differentiate it from a transient offlining failure.
//
    if (cpu_hotplug_offline_disabled) {
    return -EOPNOTSUPP;
    }
    if (cpu_hotplug_disabled) {
    return -EBUSY;
    }
    return _cpu_down(cpu, 0, target);
    }
#[no_mangle]
unsafe extern "C" fn cpu_down(cpu: c_uint, target: cpuhp_state) -> c_int {
    let mut err = 0;
    cpu_maps_update_begin();
    err = cpu_down_maps_locked(cpu, target);
    cpu_maps_update_done();
    return err;
    }
//
// cpu_device_down - Bring down a cpu device
// @dev: Pointer to the cpu device to offline
//
// This function is meant to be used by device core cpu subsystem only.
//
// Other subsystems should use remove_cpu() instead.
//
// Return: %0 on success or a negative errno code
//
#[no_mangle]
pub unsafe extern "C" fn cpu_device_down(dev: *mut device) -> c_int {
    return cpu_down(dev.id, CPUHP_OFFLINE);
    }
#[no_mangle]
pub unsafe extern "C" fn remove_cpu(cpu: c_uint) -> c_int {
    let mut ret = 0;
    lock_device_hotplug();
    ret = device_offline(get_cpu_device(cpu));
    unlock_device_hotplug();
    return ret;
    }
// EXPORT_SYMBOL_GPL;
#[no_mangle]
pub unsafe extern "C" fn smp_shutdown_nonboot_cpus(primary_cpu: c_uint) {
    let mut cpu = 0;
    let mut error = 0;
    cpu_maps_update_begin();
//
// Make certain the cpu I'm about to reboot on is online.
//
// This is inline to what migrate_to_reboot_cpu() already do.
//
    if (!cpu_online(primary_cpu)) {
    primary_cpu = cpumask_first(cpu_online_mask);
    }
    for_each_online_cpu(cpu) {
    if (cpu == primary_cpu) {
    continue;
    }
    error = cpu_down_maps_locked(cpu, CPUHP_OFFLINE);
    if (error) {
    pr_err("Failed to offline CPU%d - error=%d",
    cpu, error);
    break;
    }
    }
//
// Ensure all but the reboot CPU are offline.
//
// BUG_ON;
//
// Make sure the CPUs won't be enabled by someone else after this
// point. Kexec will reboot to a new kernel shortly resetting
// everything along the way.
//
    cpu_hotplug_disabled++;
    cpu_maps_update_done();
    }

//
// notify_cpu_starting(cpu) - Invoke the callbacks on the starting CPU
// @cpu: cpu that just started
//
// It must be called by the arch code on the new cpu, before the new cpu
// enables interrupts and before the "boot" cpu returns from __cpu_up().
//
#[no_mangle]
pub unsafe extern "C" fn notify_cpu_starting(cpu: c_uint) {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, cpu);
pub static mut target: cpuhp_state = min((int)st.target, CPUHP_AP_ONLINE);
    rcutree_report_cpu_starting(cpu);	/* Enables RCU usage on this CPU. */
    cpumask_set_cpu(cpu, &cpus_booted_once_mask);
//
// STARTING must not fail!
//
    cpuhp_invoke_callback_range_nofail(true, cpu, st, target);
    }
//
// Called from the idle task. Wake up the controlling task which brings the
// hotplug thread of the upcoming CPU up and then delegates the rest of the
// online bringup to the hotplug thread.
//
#[no_mangle]
pub unsafe extern "C" fn cpuhp_online_idle(state: cpuhp_state) {
    struct cpuhp_cpu_state *st = this_cpu_ptr(&cpuhp_state);
// Happens for the boot cpu
    if (state != CPUHP_AP_ONLINE_IDLE) {
    return;
    }
    cpuhp_ap_update_sync_state(SYNC_STATE_ONLINE);
//
// Unpark the stopper thread before we start the idle loop (and start
// scheduling); this ensures the stopper task is always available.
//
    stop_machine_unpark(smp_processor_id());
    st.state = CPUHP_AP_ONLINE_IDLE;
    complete_ap_thread(st, true);
    }
// Requires cpu_add_remove_lock to be held
#[no_mangle]
unsafe extern "C" fn _cpu_up(cpu: c_uint, tasks_frozen: c_int, target: cpuhp_state) -> c_int {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, cpu);
    let mut idle = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    cpus_write_lock();
    if (!cpu_present(cpu)) {
    ret = -EINVAL;
    goto out;
    }
//
// The caller of cpu_up() might have raced with another
// caller. Nothing to do.
//
    if (st.state >= target) {
    goto out;
    }
    if (st.state == CPUHP_OFFLINE) {
// Let it fail before we try to bring the cpu up
    idle = idle_thread_get(cpu);
    if (IS_ERR(idle)) {
    ret = PTR_ERR(idle);
    goto out;
    }
//
// Reset stale stack state from the last time this CPU was online.
//
    scs_task_reset(idle);
    kasan_unpoison_task_stack(idle);
    }
    cpuhp_tasks_frozen = tasks_frozen;
    cpuhp_set_state(cpu, st, target);
//
// If the current CPU state is in the range of the AP hotplug thread,
// then we need to kick the thread once more.
//
    if (st.state > CPUHP_BRINGUP_CPU) {
    ret = cpuhp_kick_ap_work(cpu);
//
// The AP side has done the error rollback already. Just
// return the error code..
//
    if (ret) {
    goto out;
    }
    }
//
// Try to reach the target state. We max out on the BP at
// CPUHP_BRINGUP_CPU. After that the AP hotplug thread is
// responsible for bringing it up to the target state.
//
    target = min((int)target, CPUHP_BRINGUP_CPU);
    ret = cpuhp_up_callbacks(cpu, st, target);
    out:
    cpus_write_unlock();
    arch_smt_update();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cpu_up(cpu: c_uint, target: cpuhp_state) -> c_int {
pub static mut err: c_int = 0;
    if (!cpu_possible(cpu)) {
    pr_err("can't online cpu %d because it is not configured as may-hotadd at boot time\n",
    cpu);
    return -EINVAL;
    }
    err = try_online_node(cpu_to_node(cpu));
    if (err) {
    return err;
    }
    cpu_maps_update_begin();
    if (cpu_hotplug_disabled) {
    err = -EBUSY;
    goto out;
    }
    if (!cpu_bootable(cpu)) {
    err = -EPERM;
    goto out;
    }
    err = _cpu_up(cpu, 0, target);
    out:
    cpu_maps_update_done();
    return err;
    }
//
// cpu_device_up - Bring up a cpu device
// @dev: Pointer to the cpu device to online
//
// This function is meant to be used by device core cpu subsystem only.
//
// Other subsystems should use add_cpu() instead.
//
// Return: %0 on success or a negative errno code
//
#[no_mangle]
pub unsafe extern "C" fn cpu_device_up(dev: *mut device) -> c_int {
    return cpu_up(dev.id, CPUHP_ONLINE);
    }
#[no_mangle]
pub unsafe extern "C" fn add_cpu(cpu: c_uint) -> c_int {
    let mut ret = 0;
    lock_device_hotplug();
    ret = device_online(get_cpu_device(cpu));
    unlock_device_hotplug();
    return ret;
    }
// EXPORT_SYMBOL_GPL;
//
// bringup_hibernate_cpu - Bring up the CPU that we hibernated on
// @sleep_cpu: The cpu we hibernated on and should be brought up.
//
// On some architectures like arm64, we can hibernate on any CPU, but on
// wake up the CPU we hibernated on might be offline as a side effect of
// using maxcpus= for example.
//
// Return: %0 on success or a negative errno code
//
#[no_mangle]
pub unsafe extern "C" fn bringup_hibernate_cpu(sleep_cpu: c_uint) -> c_int {
    let mut ret = 0;
    if (!cpu_online(sleep_cpu)) {
    pr_info("Hibernated on a CPU that is offline! Bringing CPU up.\n");
    ret = cpu_up(sleep_cpu, CPUHP_ONLINE);
    if (ret) {
    pr_err("Failed to bring hibernate-CPU up!\n");
    return ret;
    }
    }
    return 0;
    }
    static void __init cpuhp_bringup_mask(const struct cpumask *mask, unsigned int ncpus,
    enum cpuhp_state target)
    {
    let mut cpu = 0;
    for_each_cpu(cpu, mask) {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, cpu);
    if (cpu_up(cpu, target) && can_rollback_cpu(st)) {
//
// If this failed then cpu_up() might have only
// rolled back to CPUHP_BP_KICK_AP for the final
// online. Clean it up. NOOP if already rolled back.
//
// WARN_ON;
    }
    if (!--ncpus) {
    break;
    }
    }
    }

pub static mut __ro_after_init: bool __cpuhp_parallel_bringup = true;
#[no_mangle]
unsafe extern "C" fn parallel_bringup_parse_param(arg: *mut c_char) -> c_int {
    return kstrtobool(arg, &__cpuhp_parallel_bringup);
    }
    early_param("cpuhp.parallel", parallel_bringup_parse_param);

#[no_mangle]
pub unsafe extern "C" fn cpuhp_smt_aware() -> bool {
    return cpu_smt_max_threads > 1;
    }
    static inline const struct cpumask *cpuhp_get_primary_thread_mask(void)
    {
    return cpu_primary_thread_mask;
    }

#[no_mangle]
pub unsafe extern "C" fn cpuhp_smt_aware() -> bool {
    return false;
    }
    static inline const struct cpumask *cpuhp_get_primary_thread_mask(void)
    {
    return cpu_none_mask;
    }

#[no_mangle]
pub unsafe extern "C" fn arch_cpuhp_init_parallel_bringup() -> bool __weak {
    return true;
    }
//
// On architectures which have enabled parallel bringup this invokes all BP
// prepare states for each of the to be onlined APs first. The last state
// sends the startup IPI to the APs. The APs proceed through the low level
// bringup code in parallel and then wait for the control CPU to release
// them one by one for the final onlining procedure.
//
// This avoids waiting for each AP to respond to the startup IPI in
// CPUHP_BRINGUP_CPU.
//
#[no_mangle]
unsafe extern "C" fn cpuhp_bringup_cpus_parallel(ncpus: c_uint) -> bool __init {
    const struct cpumask *mask = cpu_present_mask;
    if (__cpuhp_parallel_bringup) {
    __cpuhp_parallel_bringup = arch_cpuhp_init_parallel_bringup();
    }
    if (!__cpuhp_parallel_bringup) {
    return false;
    }
    if (cpuhp_smt_aware()) {
    const struct cpumask *pmask = cpuhp_get_primary_thread_mask();
    static struct cpumask tmp_mask __initdata;
//
// X86 requires to prevent that SMT siblings stopped while
// the primary thread does a microcode update for various
// reasons. Bring the primary threads up first.
//
    cpumask_and(&tmp_mask, mask, pmask);
    cpuhp_bringup_mask(&tmp_mask, ncpus, CPUHP_BP_KICK_AP);
    cpuhp_bringup_mask(&tmp_mask, ncpus, CPUHP_ONLINE);
// Account for the online CPUs
    ncpus -= num_online_cpus();
    if (!ncpus) {
    return true;
    }
// Create the mask for secondary CPUs
    cpumask_andnot(&tmp_mask, mask, pmask);
    mask = &tmp_mask;
    }
// Bring the not-yet started CPUs up
    cpuhp_bringup_mask(mask, ncpus, CPUHP_BP_KICK_AP);
    cpuhp_bringup_mask(mask, ncpus, CPUHP_ONLINE);
    return true;
    }

#[no_mangle]
pub unsafe extern "C" fn cpuhp_bringup_cpus_parallel() { return false; }

#[no_mangle]
pub unsafe extern "C" fn bringup_nonboot_cpus(max_cpus: c_uint) -> c_int {
    if (!max_cpus) {
    return;
    }
// Try parallel bringup optimization if enabled
    if (cpuhp_bringup_cpus_parallel(max_cpus)) {
    return;
    }
// Full per CPU serialized bringup
    cpuhp_bringup_mask(cpu_present_mask, max_cpus, CPUHP_ONLINE);
    }

    static cpumask_var_t frozen_cpus;
#[no_mangle]
pub unsafe extern "C" fn freeze_secondary_cpus(primary: c_int) -> c_int {
    int cpu, error = 0;
    cpu_maps_update_begin();
    if (primary == -1) {
    primary = cpumask_first(cpu_online_mask);
    if (!housekeeping_cpu(primary, HK_TYPE_TIMER)) {
    primary = housekeeping_any_cpu(HK_TYPE_TIMER);
    }
    } else {
    if (!cpu_online(primary)) {
    primary = cpumask_first(cpu_online_mask);
    }
    }
//
// We take down all of the non-boot CPUs in one shot to avoid races
// with the userspace trying to use the CPU hotplug at the same time
//
    cpumask_clear(frozen_cpus);
    pr_info("Disabling non-boot CPUs ...\n");
    for (cpu = nr_cpu_ids - 1; cpu >= 0; cpu--) {
    if (!cpu_online(cpu) || cpu == primary) {
    continue;
    }
    if (pm_wakeup_pending()) {
    pr_info("Wakeup pending. Abort CPU freeze\n");
    error = -EBUSY;
    break;
    }
    trace_suspend_resume(TPS("CPU_OFF"), cpu, true);
    error = _cpu_down(cpu, 1, CPUHP_OFFLINE);
    trace_suspend_resume(TPS("CPU_OFF"), cpu, false);
    if (!error) {
    cpumask_set_cpu(cpu, frozen_cpus);
    }
    else {
    pr_err("Error taking CPU%d down: %d\n", cpu, error);
    break;
    }
    }
    if (!error) {
// BUG_ON;
    }
    else {
    pr_err("Non-boot CPUs are not disabled\n");
    }
//
// Make sure the CPUs won't be enabled by someone else. We need to do
// this even in case of failure as all freeze_secondary_cpus() users are
// supposed to do thaw_secondary_cpus() on the failure path.
//
    cpu_hotplug_disabled++;
    cpu_maps_update_done();
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_thaw_secondary_cpus_begin() -> void __weak {
    }
#[no_mangle]
pub unsafe extern "C" fn arch_thaw_secondary_cpus_end() -> void __weak {
    }
#[no_mangle]
pub unsafe extern "C" fn thaw_secondary_cpus() {
    int cpu, error;
// Allow everyone to use the CPU hotplug again
    cpu_maps_update_begin();
    __cpu_hotplug_enable();
    if (cpumask_empty(frozen_cpus)) {
    goto out;
    }
    pr_info("Enabling non-boot CPUs ...\n");
    arch_thaw_secondary_cpus_begin();
    for_each_cpu(cpu, frozen_cpus) {
    trace_suspend_resume(TPS("CPU_ON"), cpu, true);
    error = _cpu_up(cpu, 1, CPUHP_ONLINE);
    trace_suspend_resume(TPS("CPU_ON"), cpu, false);
    if (!error) {
    pr_info("CPU%d is up\n", cpu);
    continue;
    }
    pr_warn("Error taking CPU%d up: %d\n", cpu, error);
    }
    arch_thaw_secondary_cpus_end();
    cpumask_clear(frozen_cpus);
    out:
    cpu_maps_update_done();
    }
#[no_mangle]
unsafe extern "C" fn alloc_frozen_cpus() -> c_int {
    if (!alloc_cpumask_var(&frozen_cpus, GFP_KERNEL|__GFP_ZERO)) {
    return -ENOMEM;
    }
    return 0;
    }
// core_initcall;
//
// When callbacks for CPU hotplug notifications are being executed, we must
// ensure that the state of the system with respect to the tasks being frozen
// or not, as reported by the notification, remains unchanged *throughout the
// duration* of the execution of the callbacks.
// Hence we need to prevent the freezer from racing with regular CPU hotplug.
//
// This synchronization is implemented by mutually excluding regular CPU
// hotplug and Suspend/Hibernate call paths by hooking onto the Suspend
// Hibernate notifications.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_hotplug_pm_callback() {
    match (action) {
    PM_SUSPEND_PREPARE => {
    PM_HIBERNATION_PREPARE => {
    cpu_hotplug_disable();
    break;
    PM_POST_SUSPEND => {
    PM_POST_HIBERNATION => {
    cpu_hotplug_enable();
    break;
    _ => {
    return NOTIFY_DONE;
    }
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn cpu_hotplug_pm_sync_init() -> c_int {
//
// cpu_hotplug_pm_callback has higher priority than x86
// bsp_pm_callback which depends on cpu_hotplug_pm_callback
// to disable cpu hotplug to avoid cpu hotplug race.
//
    pm_notifier(cpu_hotplug_pm_callback, 0);
    return 0;
    }
// core_initcall;

    let mut __boot_cpu_id = 0;

// Boot processor state steps
pub static mut cpuhp_step: usize = 0;
// Sanity check for callbacks
#[no_mangle]
unsafe extern "C" fn cpuhp_cb_check(state: cpuhp_state) -> c_int {
    if (state <= CPUHP_OFFLINE || state >= CPUHP_ONLINE) {
    return -EINVAL;
    }
    return 0;
    }
//
// Returns a free for dynamic slot assignment of the Online state. The states
// are protected by the cpuhp_slot_states mutex and an empty slot is identified
// by having no name assigned.
//
#[no_mangle]
unsafe extern "C" fn cpuhp_reserve_state(state: cpuhp_state) -> c_int {
    enum cpuhp_state i, end;
    let mut step = core::ptr::null_mut();
    match (state) {
    CPUHP_AP_ONLINE_DYN => {
    step = cpuhp_hp_states + CPUHP_AP_ONLINE_DYN;
    end = CPUHP_AP_ONLINE_DYN_END;
    break;
    CPUHP_BP_PREPARE_DYN => {
    step = cpuhp_hp_states + CPUHP_BP_PREPARE_DYN;
    end = CPUHP_BP_PREPARE_DYN_END;
    break;
    _ => {
    return -EINVAL;
    }
    for (i = state; i <= end; i++, step++) {
    if (!step.name) {
    return i;
    }
    }
// WARN;
    return -ENOSPC;
    }
#[no_mangle]
pub unsafe extern "C" fn cpuhp_store_callbacks() {
// (Un)Install the callbacks for further cpu hotplug operations
    let mut sp = core::ptr::null_mut();
pub static mut ret: c_int = 0;
//
// If name is NULL, then the state gets removed.
//
// CPUHP_AP_ONLINE_DYN and CPUHP_BP_PREPARE_DYN are handed out on
// the first allocation from these dynamic ranges, so the removal
// would trigger a new allocation and clear the wrong (already
// empty) state, leaving the callbacks of the to be cleared state
// dangling, which causes wreckage on the next hotplug operation.
//
    if (name && (state == CPUHP_AP_ONLINE_DYN ||
    state == CPUHP_BP_PREPARE_DYN)) {
    ret = cpuhp_reserve_state(state);
    if (ret < 0) {
    return ret;
    }
    state = ret;
    }
    sp = cpuhp_get_step(state);
    if (name && sp.name) {
    return -EBUSY;
    }
    sp.startup.single = startup;
    sp.teardown.single = teardown;
    sp.name = name;
    sp.multi_instance = multi_instance;
// INIT_HLIST_HEAD;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cpuhp_get_teardown_cb() {
    return cpuhp_get_step(state).teardown.single;
    }
//
// Call the startup/teardown function for a step either on the AP or
// on the current CPU.
//
#[no_mangle]
pub unsafe extern "C" fn cpuhp_issue_call() {
    struct cpuhp_step *sp = cpuhp_get_step(state);
    let mut ret = 0;
//
// If there's nothing to do, we done.
// Relies on the union for multi_instance.
//
    if (cpuhp_step_empty(bringup, sp)) {
    return 0;
    }
//
// The non AP bound callbacks can fail on bringup. On teardown
// e.g. module removal we crash for now.
//

    if (cpuhp_is_ap_state(state)) {
    ret = cpuhp_invoke_ap_callback(cpu, state, bringup, node);
    }
    else {
    ret = cpuhp_invoke_callback(cpu, state, bringup, node, core::ptr::null_mut());
    }

    if (cpuhp_is_atomic_state(state)) {
    guard(irqsave)();
    ret = cpuhp_invoke_callback(cpu, state, bringup, node, core::ptr::null_mut());
// STARTING/DYING must not fail!
// WARN_ON_ONCE;
    } else {
    ret = cpuhp_invoke_callback(cpu, state, bringup, node, core::ptr::null_mut());
    }

// BUG_ON;
    return ret;
    }
//
// Called from __cpuhp_setup_state on a recoverable failure.
//
// Note: The teardown callbacks for rollback are not allowed to fail!
//
#[no_mangle]
pub unsafe extern "C" fn cpuhp_rollback_install() {
    let mut cpu = 0;
// Roll back the already executed steps on the other cpus
    for_each_present_cpu(cpu) {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, cpu);
pub static mut cpustate: c_int = st.state;
    if (cpu >= failedcpu) {
    break;
    }
// Did we invoke the startup call on that cpu ?
    if (cpustate >= state) {
    cpuhp_issue_call(cpu, state, false, node);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __cpuhp_state_add_instance_cpuslocked() {
    let mut sp = core::ptr::null_mut();
    let mut cpu = 0;
    let mut ret = 0;
    lockdep_assert_cpus_held();
    sp = cpuhp_get_step(state);
    if (sp.multi_instance == false) {
    return -EINVAL;
    }
    mutex_lock(&cpuhp_state_mutex);
    if (!invoke || !sp.startup.multi) {
    goto add_node;
    }
//
// Try to call the startup callback for each present cpu
// depending on the hotplug state of the cpu.
//
    for_each_present_cpu(cpu) {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, cpu);
pub static mut cpustate: c_int = st.state;
    if (cpustate < state) {
    continue;
    }
    ret = cpuhp_issue_call(cpu, state, true, node);
    if (ret) {
    if (sp.teardown.multi) {
    cpuhp_rollback_install(cpu, state, node);
    }
    goto unlock;
    }
    }
    add_node:
    ret = 0;
    hlist_add_head(node, &sp.list);
    unlock:
    mutex_unlock(&cpuhp_state_mutex);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __cpuhp_state_add_instance() {
    let mut ret = 0;
    cpus_read_lock();
    ret = __cpuhp_state_add_instance_cpuslocked(state, node, invoke);
    cpus_read_unlock();
    return ret;
    }
// EXPORT_SYMBOL_GPL;
//
// __cpuhp_setup_state_cpuslocked - Setup the callbacks for an hotplug machine state
// @state:		The state to setup
// @name:		Name of the step
// @invoke:		If true, the startup function is invoked for cpus where
// cpu state >= @state
// @startup:		startup callback function
// @teardown:		teardown callback function
// @multi_instance:	State is set up for multiple instances which get
// added afterwards.
//
// The caller needs to hold cpus read locked while calling this function.
// Return:
// On success:
// Positive state number if @state is CPUHP_AP_ONLINE_DYN or CPUHP_BP_PREPARE_DYN;
// 0 for all other states
// On failure: proper (negative) error code
//
#[no_mangle]
pub unsafe extern "C" fn __cpuhp_setup_state_cpuslocked() {
    int cpu, ret = 0;
    let mut dynstate = 0;
    lockdep_assert_cpus_held();
    if (cpuhp_cb_check(state) || !name) {
    return -EINVAL;
    }
    mutex_lock(&cpuhp_state_mutex);
    ret = cpuhp_store_callbacks(state, name, startup, teardown,
    multi_instance);
    dynstate = state == CPUHP_AP_ONLINE_DYN || state == CPUHP_BP_PREPARE_DYN;
    if (ret > 0 && dynstate) {
    state = ret;
    ret = 0;
    }
    if (ret || !invoke || !startup) {
    goto out;
    }
//
// Try to call the startup callback for each present cpu
// depending on the hotplug state of the cpu.
//
    for_each_present_cpu(cpu) {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, cpu);
pub static mut cpustate: c_int = st.state;
    if (cpustate < state) {
    continue;
    }
    ret = cpuhp_issue_call(cpu, state, true, core::ptr::null_mut());
    if (ret) {
    if (teardown) {
    cpuhp_rollback_install(cpu, state, core::ptr::null_mut());
    }
    cpuhp_store_callbacks(state, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), false);
    goto out;
    }
    }
    out:
    mutex_unlock(&cpuhp_state_mutex);
//
// If the requested state is CPUHP_AP_ONLINE_DYN or CPUHP_BP_PREPARE_DYN,
// return the dynamically allocated state in case of success.
//
    if (!ret && dynstate) {
    return state;
    }
    return ret;
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn __cpuhp_setup_state() {
    let mut ret = 0;
    cpus_read_lock();
    ret = __cpuhp_setup_state_cpuslocked(state, name, invoke, startup,
    teardown, multi_instance);
    cpus_read_unlock();
    return ret;
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn __cpuhp_state_remove_instance() {
    struct cpuhp_step *sp = cpuhp_get_step(state);
    let mut cpu = 0;
// BUG_ON;
    if (!sp.multi_instance) {
    return -EINVAL;
    }
    cpus_read_lock();
    mutex_lock(&cpuhp_state_mutex);
    if (!invoke || !cpuhp_get_teardown_cb(state)) {
    goto remove;
    }
//
// Call the teardown callback for each present cpu depending
// on the hotplug state of the cpu. This function is not
// allowed to fail currently!
//
    for_each_present_cpu(cpu) {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, cpu);
pub static mut cpustate: c_int = st.state;
    if (cpustate >= state) {
    cpuhp_issue_call(cpu, state, false, node);
    }
    }
    remove:
    hlist_del(node);
    mutex_unlock(&cpuhp_state_mutex);
    cpus_read_unlock();
    return 0;
    }
// EXPORT_SYMBOL_GPL;
//
// __cpuhp_remove_state_cpuslocked - Remove the callbacks for an hotplug machine state
// @state:	The state to remove
// @invoke:	If true, the teardown function is invoked for cpus where
// cpu state >= @state
//
// The caller needs to hold cpus read locked while calling this function.
// The teardown callback is currently not allowed to fail. Think
// about module removal!
//
#[no_mangle]
pub unsafe extern "C" fn __cpuhp_remove_state_cpuslocked(state: cpuhp_state, invoke: bool) {
    struct cpuhp_step *sp = cpuhp_get_step(state);
    let mut cpu = 0;
// BUG_ON;
    lockdep_assert_cpus_held();
    mutex_lock(&cpuhp_state_mutex);
    if (sp.multi_instance) {
    WARN(!hlist_empty(&sp.list),
    "Error: Removing state %d which has instances left.\n",
    state);
    goto remove;
    }
    if (!invoke || !cpuhp_get_teardown_cb(state)) {
    goto remove;
    }
//
// Call the teardown callback for each present cpu depending
// on the hotplug state of the cpu. This function is not
// allowed to fail currently!
//
    for_each_present_cpu(cpu) {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, cpu);
pub static mut cpustate: c_int = st.state;
    if (cpustate >= state) {
    cpuhp_issue_call(cpu, state, false, core::ptr::null_mut());
    }
    }
    remove:
    cpuhp_store_callbacks(state, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), false);
    mutex_unlock(&cpuhp_state_mutex);
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn __cpuhp_remove_state(state: cpuhp_state, invoke: bool) {
    cpus_read_lock();
    __cpuhp_remove_state_cpuslocked(state, invoke);
    cpus_read_unlock();
    }
// EXPORT_SYMBOL;

#[no_mangle]
unsafe extern "C" fn cpuhp_offline_cpu_device(cpu: c_uint) {
    struct device *dev = get_cpu_device(cpu);
    dev_set_offline(dev);
// Tell user space about the state change
    kobject_uevent(&dev.kobj, KOBJ_OFFLINE);
    }
#[no_mangle]
unsafe extern "C" fn cpuhp_online_cpu_device(cpu: c_uint) {
    struct device *dev = get_cpu_device(cpu);
    dev_clear_offline(dev);
// Tell user space about the state change
    kobject_uevent(&dev.kobj, KOBJ_ONLINE);
    }
#[no_mangle]
pub unsafe extern "C" fn cpuhp_smt_disable(ctrlval: cpuhp_smt_control) -> c_int {
    int cpu, ret = 0;
    cpu_maps_update_begin();
    for_each_online_cpu(cpu) {
    if (topology_is_primary_thread(cpu)) {
    continue;
    }
//
// Disable can be called with CPU_SMT_ENABLED when changing
// from a higher to lower number of SMT threads per core.
//
    if (ctrlval == CPU_SMT_ENABLED && cpu_smt_thread_allowed(cpu)) {
    continue;
    }
    ret = cpu_down_maps_locked(cpu, CPUHP_OFFLINE);
    if (ret) {
    break;
    }
//
// As this needs to hold the cpu maps lock it's impossible
// to call device_offline() because that ends up calling
// cpu_down() which takes cpu maps lock. cpu maps lock
// needs to be held as this might race against in kernel
// abusers of the hotplug machinery (thermal management).
//
// So nothing would update device:offline state. That would
// leave the sysfs entry stale and prevent onlining after
// smt control has been changed to 'off' again. This is
// called under the sysfs hotplug lock, so it is properly
// serialized against the regular offline usage.
//
    cpuhp_offline_cpu_device(cpu);
    }
    if (!ret) {
    cpu_smt_control = ctrlval;
    }
    cpu_maps_update_done();
    return ret;
    }
// Check if the core a CPU belongs to is online

#[no_mangle]
pub unsafe extern "C" fn topology_is_core_online(cpu: c_uint) -> bool {
    return true;
    }

#[no_mangle]
pub unsafe extern "C" fn cpuhp_smt_enable() -> c_int {
    int cpu, ret = 0;
    cpu_maps_update_begin();
    cpu_smt_control = CPU_SMT_ENABLED;
    for_each_present_cpu(cpu) {
// Skip online CPUs and CPUs on offline nodes
    if (cpu_online(cpu) || !node_online(cpu_to_node(cpu))) {
    continue;
    }
    if (!cpu_smt_thread_allowed(cpu) || !topology_is_core_online(cpu)) {
    continue;
    }
    ret = _cpu_up(cpu, 0, CPUHP_ONLINE);
    if (ret) {
    break;
    }
// See comment in cpuhp_smt_disable()
    cpuhp_online_cpu_device(cpu);
    }
    cpu_maps_update_done();
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn state_show() {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, dev.id);
    return sprintf(buf, "%d\n", st.state);
    }
// static DEVICE_ATTR_RO(state);
#[no_mangle]
pub unsafe extern "C" fn target_store() {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, dev.id);
    let mut sp = core::ptr::null_mut();
    int target, ret;
    ret = kstrtoint(buf, 10, &target);
    if (ret) {
    return ret;
    }

    if (target < CPUHP_OFFLINE || target > CPUHP_ONLINE) {
    return -EINVAL;
    }

    if (target != CPUHP_OFFLINE && target != CPUHP_ONLINE) {
    return -EINVAL;
    }

    ret = lock_device_hotplug_sysfs();
    if (ret) {
    return ret;
    }
    mutex_lock(&cpuhp_state_mutex);
    sp = cpuhp_get_step(target);
    ret = !sp.name || sp.cant_stop ? -EINVAL : 0;
    mutex_unlock(&cpuhp_state_mutex);
    if (ret) {
    goto out;
    }
    if (st.state < target) {
    ret = cpu_up(dev.id, target);
    }
#[no_mangle]
pub unsafe extern "C" fn if(target: st->state >) -> else {
    else if (st.state > target)
    ret = cpu_down(dev.id, target);
#[no_mangle]
pub unsafe extern "C" fn if(target): WARN_ON(st->target !=) -> else {
    else if (WARN_ON(st.target != target))
    st.target = target;
    out:
    unlock_device_hotplug();
    return ret ? ret : count;
    }
#[no_mangle]
pub unsafe extern "C" fn target_show() {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, dev.id);
    return sprintf(buf, "%d\n", st.target);
    }
// static DEVICE_ATTR_RW(target);
#[no_mangle]
pub unsafe extern "C" fn fail_store() {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, dev.id);
    let mut sp = core::ptr::null_mut();
    int fail, ret;
    ret = kstrtoint(buf, 10, &fail);
    if (ret) {
    return ret;
    }
    if (fail == CPUHP_INVALID) {
    st.fail = fail;
    return count;
    }
    if (fail < CPUHP_OFFLINE || fail > CPUHP_ONLINE) {
    return -EINVAL;
    }
//
// Cannot fail STARTING/DYING callbacks.
//
    if (cpuhp_is_atomic_state(fail)) {
    return -EINVAL;
    }
//
// DEAD callbacks cannot fail...
// ... neither can CPUHP_BRINGUP_CPU during hotunplug. The latter
// triggering STARTING callbacks, a failure in this state would
// hinder rollback.
//
    if (fail <= CPUHP_BRINGUP_CPU && st.state > CPUHP_BRINGUP_CPU) {
    return -EINVAL;
    }
//
// Cannot fail anything that doesn't have callbacks.
//
    mutex_lock(&cpuhp_state_mutex);
    sp = cpuhp_get_step(fail);
    if (!sp.startup.single && !sp.teardown.single) {
    ret = -EINVAL;
    }
    mutex_unlock(&cpuhp_state_mutex);
    if (ret) {
    return ret;
    }
    st.fail = fail;
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn fail_show() {
    struct cpuhp_cpu_state *st = per_cpu_ptr(&cpuhp_state, dev.id);
    return sprintf(buf, "%d\n", st.fail);
    }
// static DEVICE_ATTR_RW(fail);
    static struct attribute *cpuhp_cpu_attrs[] = {
    &dev_attr_state.attr,
    &dev_attr_target.attr,
    &dev_attr_fail.attr,
    core::ptr::null_mut()
    };
pub static mut attribute_group: usize = 0;
#[no_mangle]
unsafe extern "C" fn states_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
pub static mut res: isize = 0;
    let mut i = 0;
    mutex_lock(&cpuhp_state_mutex);
    for (i = CPUHP_OFFLINE; i <= CPUHP_ONLINE; i++) {
    struct cpuhp_step *sp = cpuhp_get_step(i);
    if (sp.name) {
    res += sysfs_emit_at(buf, res, "%3d: %s\n", i, sp.name);
    }
    }
    mutex_unlock(&cpuhp_state_mutex);
    return res;
    }
// static DEVICE_ATTR_RO(states);
    static struct attribute *cpuhp_cpu_root_attrs[] = {
    &dev_attr_states.attr,
    core::ptr::null_mut()
    };
pub static mut attribute_group: usize = 0;

#[no_mangle]
unsafe extern "C" fn cpu_smt_num_threads_valid(threads: c_uint) -> bool {
    if (IS_ENABLED(CONFIG_SMT_NUM_THREADS_DYNAMIC)) {
    return threads >= 1 && threads <= cpu_smt_max_threads;
    }
pub static mut threads: return = = 1 || threads == cpu_smt_max_threads;
    }
#[no_mangle]
pub unsafe extern "C" fn __store_smt_control() {
    int ctrlval, ret, num_threads, orig_threads;
    let mut force_off = 0;
    if (cpu_smt_control == CPU_SMT_FORCE_DISABLED) {
    return -EPERM;
    }
    if (cpu_smt_control == CPU_SMT_NOT_SUPPORTED) {
    return -ENODEV;
    }
    if (sysfs_streq(buf, "on")) {
    ctrlval = CPU_SMT_ENABLED;
    num_threads = cpu_smt_max_threads;
    } else if (sysfs_streq(buf, "off")) {
    ctrlval = CPU_SMT_DISABLED;
    num_threads = 1;
    } else if (sysfs_streq(buf, "forceoff")) {
    ctrlval = CPU_SMT_FORCE_DISABLED;
    num_threads = 1;
    } else if (kstrtoint(buf, 10, &num_threads) == 0) {
    if (num_threads == 1) {
    ctrlval = CPU_SMT_DISABLED;
    }
#[no_mangle]
pub unsafe extern "C" fn if(_arg: cpu_smt_num_threads_valid(num_threads)) -> else {
    else if (cpu_smt_num_threads_valid(num_threads))
    ctrlval = CPU_SMT_ENABLED;
    else {
    return -EINVAL;
    }
    } else {
    return -EINVAL;
    }
    ret = lock_device_hotplug_sysfs();
    if (ret) {
    return ret;
    }
    orig_threads = cpu_smt_num_threads;
    cpu_smt_num_threads = num_threads;
    force_off = ctrlval != cpu_smt_control && ctrlval == CPU_SMT_FORCE_DISABLED;
    if (num_threads > orig_threads) {
    ret = cpuhp_smt_enable();
    }
#[no_mangle]
pub unsafe extern "C" fn if(force_off: num_threads < orig_threads ||) -> else {
    else if (num_threads < orig_threads || force_off)
    ret = cpuhp_smt_disable(ctrlval);
    unlock_device_hotplug();
    return ret ? ret : count;
    }

#[no_mangle]
pub unsafe extern "C" fn __store_smt_control() {
    return -ENODEV;
    }

    static const char *smt_states[] = {
    [CPU_SMT_ENABLED]		= "on",
    [CPU_SMT_DISABLED]		= "off",
    [CPU_SMT_FORCE_DISABLED]	= "forceoff",
    [CPU_SMT_NOT_SUPPORTED]		= "notsupported",
    [CPU_SMT_NOT_IMPLEMENTED]	= "notimplemented",
    };
#[no_mangle]
pub unsafe extern "C" fn control_show() {
    const char *state = smt_states[cpu_smt_control];

//
// If SMT is enabled but not all threads are enabled then show the
// number of threads. If all threads are enabled show "on". Otherwise
// show the state name.
//
    if (cpu_smt_control == CPU_SMT_ENABLED &&
    cpu_smt_num_threads != cpu_smt_max_threads)
    return sysfs_emit(buf, "%d\n", cpu_smt_num_threads);

    return sysfs_emit(buf, "%s\n", state);
    }
#[no_mangle]
pub unsafe extern "C" fn control_store() {
    return __store_smt_control(dev, attr, buf, count);
    }
// static DEVICE_ATTR_RW(control);
#[no_mangle]
pub unsafe extern "C" fn active_show() {
    return sysfs_emit(buf, "%d\n", sched_smt_active());
    }
// static DEVICE_ATTR_RO(active);
    static struct attribute *cpuhp_smt_attrs[] = {
    &dev_attr_control.attr,
    &dev_attr_active.attr,
    core::ptr::null_mut()
    };
pub static mut attribute_group: usize = 0;
#[no_mangle]
unsafe extern "C" fn cpu_smt_sysfs_init() -> c_int {
    let mut dev_root = core::ptr::null_mut();
pub static mut ret: c_int = -ENODEV;
    dev_root = bus_get_dev_root(&cpu_subsys);
    if (dev_root) {
    ret = sysfs_create_group(&dev_root.kobj, &cpuhp_smt_attr_group);
    put_device(dev_root);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cpuhp_sysfs_init() -> c_int {
    let mut dev_root = core::ptr::null_mut();
    int cpu, ret;
    ret = cpu_smt_sysfs_init();
    if (ret) {
    return ret;
    }
    dev_root = bus_get_dev_root(&cpu_subsys);
    if (dev_root) {
    ret = sysfs_create_group(&dev_root.kobj, &cpuhp_cpu_root_attr_group);
    put_device(dev_root);
    if (ret) {
    return ret;
    }
    }
    for_each_possible_cpu(cpu) {
    struct device *dev = get_cpu_device(cpu);
    if (!dev) {
    continue;
    }
    ret = sysfs_create_group(&dev.kobj, &cpuhp_cpu_attr_group);
    if (ret) {
    return ret;
    }
    }
    return 0;
    }
// device_initcall;

//
// cpu_bit_bitmap[] is a special, "compressed" data structure that
// represents all NR_CPUS bits binary values of 1<<nr.
//
// It is used by cpumask_of() to get a constant address to a CPU
// mask value that has a single bit set only.
//
// cpu_bit_bitmap[0] is empty - so we can back into it

    const unsigned long cpu_bit_bitmap[BITS_PER_LONG+1][BITS_TO_LONGS(NR_CPUS)] = {
    MASK_DECLARE_8(0),	MASK_DECLARE_8(8),
    MASK_DECLARE_8(16),	MASK_DECLARE_8(24),

    MASK_DECLARE_8(32),	MASK_DECLARE_8(40),
    MASK_DECLARE_8(48),	MASK_DECLARE_8(56),

    };
// EXPORT_SYMBOL_GPL;
    const DECLARE_BITMAP(cpu_all_bits, NR_CPUS) = CPU_BITS_ALL;
// EXPORT_SYMBOL;

    struct cpumask __cpu_possible_mask __ro_after_init
    = {CPU_BITS_ALL};
pub static mut __ro_after_init: unsigned int __num_possible_cpus = NR_CPUS;

    struct cpumask __cpu_possible_mask __ro_after_init;
    unsigned int __num_possible_cpus __ro_after_init;

// EXPORT_SYMBOL;
// EXPORT_SYMBOL;
    struct cpumask __cpu_online_mask __read_mostly;
// EXPORT_SYMBOL;
    struct cpumask __cpu_enabled_mask __read_mostly;
// EXPORT_SYMBOL;
    struct cpumask __cpu_present_mask __read_mostly;
// EXPORT_SYMBOL;
    struct cpumask __cpu_active_mask __read_mostly;
// EXPORT_SYMBOL;
    struct cpumask __cpu_dying_mask __read_mostly;
// EXPORT_SYMBOL;
    atomic_t __num_online_cpus __read_mostly;
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn init_cpu_present(src: *const cpumask) {
    cpumask_copy(&__cpu_present_mask, src);
    }
#[no_mangle]
pub unsafe extern "C" fn init_cpu_possible(src: *const cpumask) {
    cpumask_copy(&__cpu_possible_mask, src);
    __num_possible_cpus = cpumask_weight(&__cpu_possible_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn set_cpu_online(cpu: c_uint, online: bool) {
//
// atomic_inc/dec() is required to handle the horrid abuse of this
// function by the reboot and kexec code which invoke it from
// IPI/NMI broadcasts when shutting down CPUs. Invocation from
// regular CPU hotplug is properly serialized.
//
// Note, that the fact that __num_online_cpus is of type atomic_t
// does not protect readers which are not serialized against
// concurrent hotplug operations.
//
    if (online) {
    if (!cpumask_test_and_set_cpu(cpu, &__cpu_online_mask)) {
    atomic_inc(&__num_online_cpus);
    }
    } else {
    if (cpumask_test_and_clear_cpu(cpu, &__cpu_online_mask)) {
    atomic_dec(&__num_online_cpus);
    }
    }
    }
//
// This should be marked __init, but there is a boatload of call sites
// which need to be fixed up to do so. Sigh...
//
#[no_mangle]
pub unsafe extern "C" fn set_cpu_possible(cpu: c_uint, possible: bool) {
    if (possible) {
    if (!cpumask_test_and_set_cpu(cpu, &__cpu_possible_mask)) {
    __num_possible_cpus++;
    }
    } else {
    if (cpumask_test_and_clear_cpu(cpu, &__cpu_possible_mask)) {
    __num_possible_cpus--;
    }
    }
    }
//
// Activate the first processor.
//
#[no_mangle]
pub unsafe extern "C" fn boot_cpu_init() -> c_int {
pub static mut cpu: c_int = smp_processor_id();
// Mark the boot cpu "present", "online" etc for SMP and UP case
    set_cpu_online(cpu, true);
    set_cpu_active(cpu, true);
    set_cpu_present(cpu, true);
    set_cpu_possible(cpu, true);

    __boot_cpu_id = cpu;

    }
//
// Must be called _AFTER_ setting up the per_cpu areas
//
#[no_mangle]
pub unsafe extern "C" fn boot_cpu_hotplug_init() -> c_int {

    cpumask_set_cpu(smp_processor_id(), &cpus_booted_once_mask);
    atomic_set(this_cpu_ptr(&cpuhp_state.ap_sync_state), SYNC_STATE_ONLINE);

    this_cpu_write(cpuhp_state.state, CPUHP_ONLINE);
    this_cpu_write(cpuhp_state.target, CPUHP_ONLINE);
    }

//
// All except the cross-thread attack vector are mitigated by default.
// Cross-thread mitigation often requires disabling SMT which is expensive
// so cross-thread mitigations are only partially enabled by default.
//
// Guest-to-Host and Guest-to-Guest vectors are only needed if KVM support is
// present.
//
    static bool attack_vectors[NR_CPU_ATTACK_VECTORS] __ro_after_init = {
    [CPU_MITIGATE_USER_KERNEL] = true,
    [CPU_MITIGATE_USER_USER] = true,
    [CPU_MITIGATE_GUEST_HOST] = IS_ENABLED(CONFIG_KVM),
    [CPU_MITIGATE_GUEST_GUEST] = IS_ENABLED(CONFIG_KVM),
    };
#[no_mangle]
pub unsafe extern "C" fn cpu_attack_vector_mitigated(v: cpu_attack_vectors) -> bool {
    if (v < NR_CPU_ATTACK_VECTORS) {
    return attack_vectors[v];
    }
// WARN_ONCE;
    return false;
    }
//
// There are 3 global options, 'off', 'auto', 'auto,nosmt'. These may optionally
// be combined with attack-vector disables which follow them.
//
// Examples:
// mitigations=auto,no_user_kernel,no_user_user,no_cross_thread
// mitigations=auto,nosmt,no_guest_host,no_guest_guest
//
// mitigations=off is equivalent to disabling all attack vectors.
//
    enum cpu_mitigations {
    CPU_MITIGATIONS_OFF,
    CPU_MITIGATIONS_AUTO,
    CPU_MITIGATIONS_AUTO_NOSMT,
    };
    enum {
    NO_USER_KERNEL,
    NO_USER_USER,
    NO_GUEST_HOST,
    NO_GUEST_GUEST,
    NO_CROSS_THREAD,
    NR_VECTOR_PARAMS,
    };
pub static mut __ro_after_init: smt_mitigations smt_mitigations = SMT_MITIGATIONS_AUTO;
pub static mut __ro_after_init: cpu_mitigations cpu_mitigations = CPU_MITIGATIONS_AUTO;
    static const match_table_t global_mitigations = {
    { CPU_MITIGATIONS_AUTO_NOSMT,	"auto,nosmt"},
    { CPU_MITIGATIONS_AUTO,		"auto"},
    { CPU_MITIGATIONS_OFF,		"off"},
    };
    static const match_table_t vector_mitigations = {
    { NO_USER_KERNEL,	"no_user_kernel"},
    { NO_USER_USER,		"no_user_user"},
    { NO_GUEST_HOST,	"no_guest_host"},
    { NO_GUEST_GUEST,	"no_guest_guest"},
    { NO_CROSS_THREAD,	"no_cross_thread"},
    { NR_VECTOR_PARAMS,	core::ptr::null_mut()},
    };
#[no_mangle]
unsafe extern "C" fn mitigations_parse_global_opt(arg: *mut c_char) -> c_int {
    let mut i = 0;
    for (i = 0; i < ARRAY_SIZE(global_mitigations); i++) {
    const char *pattern = global_mitigations[i].pattern;
    if (!strncmp(arg, pattern, strlen(pattern))) {
    cpu_mitigations = global_mitigations[i].token;
    return strlen(pattern);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mitigations_parse_cmdline(arg: *mut c_char) -> c_int {
    char *s, *p;
    let mut len = 0;
    len = mitigations_parse_global_opt(arg);
    if (cpu_mitigations_off()) {
    memset(attack_vectors, 0, sizeof(attack_vectors));
    smt_mitigations = SMT_MITIGATIONS_OFF;
    } else if (cpu_mitigations_auto_nosmt()) {
    smt_mitigations = SMT_MITIGATIONS_ON;
    }
    p = arg + len;
    if (!*p) {
    return 0;
    }
// Attack vector controls may come after the ','
    if (*p++ != ',' || !IS_ENABLED(CONFIG_ARCH_HAS_CPU_ATTACK_VECTORS)) {
    pr_crit("Unsupported mitigations=%s, system may still be vulnerable\n",	arg);
    return 0;
    }
    while ((s = strsep(&p, ",")) != core::ptr::null_mut()) {
    switch (match_token(s, vector_mitigations, core::ptr::null_mut())) {
    NO_USER_KERNEL => {
    attack_vectors[CPU_MITIGATE_USER_KERNEL] = false;
    break;
    NO_USER_USER => {
    attack_vectors[CPU_MITIGATE_USER_USER] = false;
    break;
    NO_GUEST_HOST => {
    attack_vectors[CPU_MITIGATE_GUEST_HOST] = false;
    break;
    NO_GUEST_GUEST => {
    attack_vectors[CPU_MITIGATE_GUEST_GUEST] = false;
    break;
    NO_CROSS_THREAD => {
    smt_mitigations = SMT_MITIGATIONS_OFF;
    break;
    _ => {
    pr_crit("Unsupported mitigations options %s\n",	s);
    return 0;
    }
    }
    return 0;
    }
// mitigations=off
#[no_mangle]
pub unsafe extern "C" fn cpu_mitigations_off() -> bool {
pub static mut cpu_mitigations: return = = CPU_MITIGATIONS_OFF;
    }
// EXPORT_SYMBOL_GPL;
// mitigations=auto,nosmt
#[no_mangle]
pub unsafe extern "C" fn cpu_mitigations_auto_nosmt() -> bool {
pub static mut cpu_mitigations: return = = CPU_MITIGATIONS_AUTO_NOSMT;
    }
// EXPORT_SYMBOL_GPL;

#[no_mangle]
unsafe extern "C" fn mitigations_parse_cmdline(arg: *mut c_char) -> c_int {
    pr_crit("Kernel compiled without mitigations, ignoring 'mitigations'; system may still be vulnerable\n");
    return 0;
    }

    early_param("mitigations", mitigations_parse_cmdline);

}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
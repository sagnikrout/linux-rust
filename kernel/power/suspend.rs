//! Automatically rewritten from C to Rust
//! Source: kernel/power/suspend.c
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
// kernel/power/suspend.c - Suspend to RAM and standby functionality.
//
// Copyright (c) 2003 Patrick Mochel
// Copyright (c) 2003 Open Source Development Lab
// Copyright (c) 2009 Rafael J. Wysocki <rjw@sisk.pl>, Novell Inc.
//

    const char * const pm_labels[] = {
    [PM_SUSPEND_TO_IDLE] = "freeze",
    [PM_SUSPEND_STANDBY] = "standby",
    [PM_SUSPEND_MEM] = "mem",
    };
    const char *pm_states[PM_SUSPEND_MAX];
    static const char * const mem_sleep_labels[] = {
    [PM_SUSPEND_TO_IDLE] = "s2idle",
    [PM_SUSPEND_STANDBY] = "shallow",
    [PM_SUSPEND_MEM] = "deep",
    };
    const char *mem_sleep_states[PM_SUSPEND_MAX];
pub static mut mem_sleep_current: suspend_state_t = 0;
pub static mut mem_sleep_default: suspend_state_t = 0;
    let mut pm_suspend_target_state;
    EXPORT_SYMBOL_GPL(pm_suspend_target_state);
    let mut pm_suspend_global_flags = 0;
    EXPORT_SYMBOL_GPL(pm_suspend_global_flags);
pub static mut suspend_ops: *mut c_void = core::ptr::null_mut();
pub static mut s2idle_ops: *mut c_void = core::ptr::null_mut();
pub static mut s2idle_wait_head: usize = 0;
    enum s2idle_states  s2idle_state;
pub static mut s2idle_lock: usize = 0;
//
// pm_suspend_default_s2idle - Check if suspend-to-idle is the default suspend.
//
// Return 'true' if suspend-to-idle has been selected as the default system
// suspend method.
//
#[no_mangle]
pub unsafe extern "C" fn pm_suspend_default_s2idle() -> bool {
pub static mut mem_sleep_current: return = 0;
    }
    EXPORT_SYMBOL_GPL(pm_suspend_default_s2idle);
#[no_mangle]
pub unsafe extern "C" fn s2idle_set_ops(ops: *const platform_s2idle_ops) {
    let mut sleep_flags = 0;
    sleep_flags = lock_system_sleep();
    s2idle_ops = ops;
    unlock_system_sleep(sleep_flags);
    }
#[no_mangle]
unsafe extern "C" fn s2idle_begin() {
    s2idle_state = S2IDLE_STATE_NONE;
    }
#[no_mangle]
unsafe extern "C" fn s2idle_enter() {
    trace_suspend_resume(TPS("machine_suspend"), PM_SUSPEND_TO_IDLE, true);
//
// The correctness of the code below depends on the number of online
// CPUs being stable, but CPUs cannot be taken offline or put online
// while it is running.
//
// The s2idle_lock must be acquired before the pending wakeup check to
// prevent pm_system_wakeup() from running as a whole between that check
// and the subsequent s2idle_state update in which case a wakeup event
// would get lost.
//
    raw_spin_lock_irq(&s2idle_lock);
    if (pm_wakeup_pending()) {
// goto;
    }
    s2idle_state = S2IDLE_STATE_ENTER;
    raw_spin_unlock_irq(&s2idle_lock);
// Push all the CPUs into the idle loop.
    wake_up_all_idle_cpus();
// Make the current CPU wait so it can enter the idle loop too.
    swait_event_exclusive(s2idle_wait_head,
    s2idle_state == S2IDLE_STATE_WAKE);
//
// Kick all CPUs to ensure that they resume their timers and restore
// consistent system state.
//
    wake_up_all_idle_cpus();
    raw_spin_lock_irq(&s2idle_lock);
// label;
    s2idle_state = S2IDLE_STATE_NONE;
    raw_spin_unlock_irq(&s2idle_lock);
    trace_suspend_resume(TPS("machine_suspend"), PM_SUSPEND_TO_IDLE, false);
    }
#[no_mangle]
unsafe extern "C" fn s2idle_loop() {
    pm_pr_dbg("suspend-to-idle\n");
//
// Suspend-to-idle equals:
// frozen processes + suspended devices + idle processors.
// Thus s2idle_enter() should be called right after all devices have
// been suspended.
//
// Wakeups during the noirq suspend of devices may be spurious, so try
// to avoid them upfront.
//
    for (;;) {
    if (s2idle_ops && s2idle_ops.wake) {
    if (s2idle_ops.wake()) {
    break;
    }
    } else if (pm_wakeup_pending()) {
    break;
    }
    if (s2idle_ops && s2idle_ops.check) {
    s2idle_ops.check();
    }
    s2idle_enter();
    }
    pm_pr_dbg("resume from suspend-to-idle\n");
    }
#[no_mangle]
pub unsafe extern "C" fn s2idle_wake() {
    let mut flags = 0;
    raw_spin_lock_irqsave(&s2idle_lock, flags);
    if (s2idle_state > S2IDLE_STATE_NONE) {
    s2idle_state = S2IDLE_STATE_WAKE;
    swake_up_one(&s2idle_wait_head);
    }
    raw_spin_unlock_irqrestore(&s2idle_lock, flags);
    }
    EXPORT_SYMBOL_GPL(s2idle_wake);
#[no_mangle]
unsafe extern "C" fn valid_state(state: suspend_state_t) -> bool {
//
// The PM_SUSPEND_STANDBY and PM_SUSPEND_MEM states require low-level
// support and need to be valid to the low-level implementation.
//
// No ->valid() or ->enter() callback implies that none are valid.
//
    return suspend_ops && suspend_ops.valid && suspend_ops.valid(state) &&
    suspend_ops.enter;
    }
#[no_mangle]
pub unsafe extern "C" fn pm_states_init()  {
// "mem" and "freeze" are always present in /sys/power/state.
    pm_states[PM_SUSPEND_MEM] = pm_labels[PM_SUSPEND_MEM];
    pm_states[PM_SUSPEND_TO_IDLE] = pm_labels[PM_SUSPEND_TO_IDLE];
//
// Suspend-to-idle should be supported even without any suspend_ops,
// initialize mem_sleep_states[] accordingly here.
//
    mem_sleep_states[PM_SUSPEND_TO_IDLE] = mem_sleep_labels[PM_SUSPEND_TO_IDLE];
    }
#[no_mangle]
unsafe extern "C" fn mem_sleep_default_setup(str: *mut c_char) -> c_int {
    let mut state;
    for (state = PM_SUSPEND_TO_IDLE; state <= PM_SUSPEND_MEM; state++) {
    if (mem_sleep_labels[state] &&
    !strcmp(str, mem_sleep_labels[state])) {
    }
    mem_sleep_default = state;
    mem_sleep_current = state;
    break;
    }
    return 1;
    }
    __setup!("mem_sleep_default=", mem_sleep_default_setup);
//
// suspend_set_ops - Set the global suspend method table.
// @ops: Suspend operations to use.
//
#[no_mangle]
pub unsafe extern "C" fn suspend_set_ops(ops: *const platform_suspend_ops) {
    let mut sleep_flags = 0;
    sleep_flags = lock_system_sleep();
    suspend_ops = ops;
    if (valid_state(PM_SUSPEND_STANDBY)) {
    mem_sleep_states[PM_SUSPEND_STANDBY] = mem_sleep_labels[PM_SUSPEND_STANDBY];
    pm_states[PM_SUSPEND_STANDBY] = pm_labels[PM_SUSPEND_STANDBY];
    if (mem_sleep_default == PM_SUSPEND_STANDBY) {
    mem_sleep_current = PM_SUSPEND_STANDBY;
    }
    }
    if (valid_state(PM_SUSPEND_MEM)) {
    mem_sleep_states[PM_SUSPEND_MEM] = mem_sleep_labels[PM_SUSPEND_MEM];
    if (mem_sleep_default >= PM_SUSPEND_MEM) {
    mem_sleep_current = PM_SUSPEND_MEM;
    }
    }
    unlock_system_sleep(sleep_flags);
    }
    EXPORT_SYMBOL_GPL(suspend_set_ops);
//
// suspend_valid_only_mem - Generic memory-only valid callback.
// @state: Target system sleep state.
//
// Platform drivers that implement mem suspend only and only need to check for
// that in their .valid() callback can use this instead of rolling their own
// .valid() callback.
//
#[no_mangle]
pub unsafe extern "C" fn suspend_valid_only_mem(state: suspend_state_t) -> c_int {
pub static mut state: return = 0;
    }
    EXPORT_SYMBOL_GPL(suspend_valid_only_mem);
#[no_mangle]
unsafe extern "C" fn sleep_state_supported(state: suspend_state_t) -> bool {
    return state == PM_SUSPEND_TO_IDLE ||
    (valid_state(state) && !cxl_mem_active());
    }
#[no_mangle]
unsafe extern "C" fn platform_suspend_prepare(state: suspend_state_t) -> c_int {
    return state != PM_SUSPEND_TO_IDLE && suspend_ops.prepare ?
    suspend_ops.prepare() : 0;
    }
#[no_mangle]
unsafe extern "C" fn platform_suspend_prepare_late(state: suspend_state_t) -> c_int {
    return state == PM_SUSPEND_TO_IDLE && s2idle_ops && s2idle_ops.prepare ?
    s2idle_ops.prepare() : 0;
    }
#[no_mangle]
unsafe extern "C" fn platform_suspend_prepare_noirq(state: suspend_state_t) -> c_int {
    if (state == PM_SUSPEND_TO_IDLE) {
    return s2idle_ops && s2idle_ops.prepare_late ?
    s2idle_ops.prepare_late() : 0;
    }
    return suspend_ops.prepare_late ? suspend_ops.prepare_late() : 0;
    }
#[no_mangle]
unsafe extern "C" fn platform_resume_noirq(state: suspend_state_t) {
    if (state == PM_SUSPEND_TO_IDLE) {
    if (s2idle_ops && s2idle_ops.restore_early) {
    s2idle_ops.restore_early();
    }
    } else if (suspend_ops.wake) {
    suspend_ops.wake();
    }
    }
#[no_mangle]
unsafe extern "C" fn platform_resume_early(state: suspend_state_t) {
    if (state == PM_SUSPEND_TO_IDLE && s2idle_ops && s2idle_ops.restore) {
    s2idle_ops.restore();
    }
    }
#[no_mangle]
unsafe extern "C" fn platform_resume_finish(state: suspend_state_t) {
    if (state != PM_SUSPEND_TO_IDLE && suspend_ops.finish) {
    suspend_ops.finish();
    }
    }
#[no_mangle]
unsafe extern "C" fn platform_suspend_begin(state: suspend_state_t) -> c_int {
    if (state == PM_SUSPEND_TO_IDLE && s2idle_ops && s2idle_ops.begin) {
    return s2idle_ops.begin();
    }

    else if (suspend_ops && suspend_ops.begin) {
    return suspend_ops.begin(state);
    }
    else {
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn platform_resume_end(state: suspend_state_t) {
    if (state == PM_SUSPEND_TO_IDLE && s2idle_ops && s2idle_ops.end) {
    s2idle_ops.end();
    }

    else if (suspend_ops && suspend_ops.end) {
    suspend_ops.end();
    }
    }
#[no_mangle]
unsafe extern "C" fn platform_recover(state: suspend_state_t) {
    if (state != PM_SUSPEND_TO_IDLE && suspend_ops.recover) {
    suspend_ops.recover();
    }
    }
#[no_mangle]
unsafe extern "C" fn platform_suspend_again(state: suspend_state_t) -> bool {
    return state != PM_SUSPEND_TO_IDLE && suspend_ops.suspend_again ?
    suspend_ops.suspend_again() : false;
    }

pub static mut pm_test_delay: unsigned int = 5;
    module_param!(pm_test_delay, uint, 0644);
    MODULE_PARM_DESC(pm_test_delay,
    "Number of seconds to wait before resuming from suspend test");

#[no_mangle]
unsafe extern "C" fn suspend_test(level: c_int) -> c_int {

    let mut i = 0;
    if (pm_test_level == level) {
    pr_info!("suspend debug: Waiting for %d second(s).\n",
    pm_test_delay);
    while (i < pm_test_delay && !pm_wakeup_pending()) {
    if (level > TEST_CORE) {
    msleep(1000);
    }
    else {
    mdelay(1000);
    }
    }
    return 1;
    }

    return 0;
    }
//
// suspend_prepare - Prepare for entering system sleep state.
// @state: Target system sleep state.
//
// Common code run for every system sleep state that can be entered (except for
// hibernation).  Run suspend notifiers, allocate the "suspend" console and
// freeze processes.
//
#[no_mangle]
unsafe extern "C" fn suspend_prepare(state: suspend_state_t) -> c_int {
    let mut error = 0;
    if (!sleep_state_supported(state)) {
    return -EPERM;
    }
    pm_prepare_console();
    error = pm_notifier_call_chain_robust(PM_SUSPEND_PREPARE, PM_POST_SUSPEND);
    if (error) {
// goto;
    }
    filesystems_freeze(filesystem_freeze_enabled);
    trace_suspend_resume(TPS("freeze_processes"), 0, true);
    error = suspend_freeze_processes();
    trace_suspend_resume(TPS("freeze_processes"), 0, false);
    if (!error) {
    return 0;
    }
    dpm_save_failed_step(SUSPEND_FREEZE);
    filesystems_thaw();
    pm_notifier_call_chain(PM_POST_SUSPEND);
// label;
    pm_restore_console();
    return error;
    }
// default implementation
#[no_mangle]
pub unsafe extern "C" fn arch_suspend_disable_irqs() -> void __weak {
    local_irq_disable();
    }
// default implementation
#[no_mangle]
pub unsafe extern "C" fn arch_suspend_enable_irqs() -> void __weak {
    local_irq_enable();
    }
//
// suspend_enter - Make the system enter the given sleep state.
// @state: System sleep state to enter.
// @wakeup: Returns information that the sleep state should not be re-entered.
//
// This function should be called after devices have been suspended.
//
#[no_mangle]
unsafe extern "C" fn suspend_enter(state: suspend_state_t, wakeup: *mut bool) -> c_int {
    let mut error = 0;
    error = platform_suspend_prepare(state);
    if (error) {
// goto;
    }
    error = dpm_suspend_late(PMSG_SUSPEND);
    if (error) {
    pr_err!("late suspend of devices failed\n");
// goto;
    }
    error = platform_suspend_prepare_late(state);
    if (error) {
// goto;
    }
    error = dpm_suspend_noirq(PMSG_SUSPEND);
    if (error) {
    pr_err!("noirq suspend of devices failed\n");
// goto;
    }
    error = platform_suspend_prepare_noirq(state);
    if (error) {
// goto;
    }
    if (suspend_test(TEST_PLATFORM)) {
// goto;
    }
    if (state == PM_SUSPEND_TO_IDLE) {
    s2idle_loop();
// goto;
    }
    error = pm_sleep_disable_secondary_cpus();
    if (error || suspend_test(TEST_CPUS)) {
// goto;
    }
    arch_suspend_disable_irqs();
    BUG_ON!(!irqs_disabled());
    system_state = SYSTEM_SUSPEND;
    error = syscore_suspend();
    if (!error) {
// wakeup = pm_wakeup_pending();
    if (!(suspend_test(TEST_CORE) || *wakeup)) {
    trace_suspend_resume(TPS("machine_suspend"),
    state, true);
    error = suspend_ops.enter(state);
    trace_suspend_resume(TPS("machine_suspend"),
    state, false);
    } else if (*wakeup) {
    error = -EBUSY;
    }
    syscore_resume();
    }
    system_state = SYSTEM_RUNNING;
    arch_suspend_enable_irqs();
    BUG_ON!(irqs_disabled());
// label;
    pm_sleep_enable_secondary_cpus();
// label;
    platform_resume_noirq(state);
    dpm_resume_noirq(PMSG_RESUME);
// label;
    platform_resume_early(state);
// label;
    dpm_resume_early(PMSG_RESUME);
// label;
    platform_resume_finish(state);
    return error;
    }
//
// suspend_devices_and_enter - Suspend devices and enter system sleep state.
// @state: System sleep state to enter.
//
#[no_mangle]
pub unsafe extern "C" fn suspend_devices_and_enter(state: suspend_state_t) -> c_int {
    let mut error = 0;
pub static mut wakeup: bool = false;
    if (!sleep_state_supported(state)) {
    return -ENOSYS;
    }
    pm_suspend_target_state = state;
    if (state == PM_SUSPEND_TO_IDLE) {
    pm_set_suspend_no_platform();
    }
    error = platform_suspend_begin(state);
    if (error) {
// goto;
    }
    console_suspend_all();
    suspend_test_start();
    error = dpm_suspend_start(PMSG_SUSPEND);
    if (error) {
    pr_err!("Some devices failed to suspend, or early wake event detected\n");
// goto;
    }
    suspend_test_finish("suspend devices");
    if (suspend_test(TEST_DEVICES)) {
// goto;
    }
    do {
    error = suspend_enter(state, &wakeup);
    } while (!error && !wakeup && platform_suspend_again(state));
// label;
    suspend_test_start();
    dpm_resume_end(PMSG_RESUME);
    suspend_test_finish("resume devices");
    trace_suspend_resume(TPS("console_resume_all"), state, true);
    console_resume_all();
    trace_suspend_resume(TPS("console_resume_all"), state, false);
// label;
    platform_resume_end(state);
    pm_suspend_target_state = PM_SUSPEND_ON;
    return error;
// label;
    platform_recover(state);
// goto;
    }
//
// suspend_finish - Clean up before finishing the suspend sequence.
//
// Call platform code to clean up, restart processes, and free the console that
// we've allocated. This routine is not called for hibernation.
//
#[no_mangle]
unsafe extern "C" fn suspend_finish() {
    suspend_thaw_processes();
    filesystems_thaw();
    pm_notifier_call_chain(PM_POST_SUSPEND);
    pm_restore_console();
    }
//
// enter_state - Do common work needed to enter system sleep state.
// @state: System sleep state to enter.
//
// Make sure that no one else is trying to put the system into a sleep state.
// Fail if that's not the case.  Otherwise, prepare for system suspend, make the
// system enter the given sleep state and clean up after wakeup.
//
#[no_mangle]
unsafe extern "C" fn enter_state(state: suspend_state_t) -> c_int {
    let mut error = 0;
    trace_suspend_resume(TPS("suspend_enter"), state, true);
    if (state == PM_SUSPEND_TO_IDLE) {

    if (pm_test_level != TEST_NONE && pm_test_level <= TEST_CPUS) {
    pr_warn!("Unsupported test mode for suspend to idle, please choose none/freezer/devices/platform.\n");
    return -EAGAIN;
    }

    } else if (!valid_state(state)) {
    return -EINVAL;
    }
    if (!mutex_trylock(&system_transition_mutex)) {
    return -EBUSY;
    }
    if (state == PM_SUSPEND_TO_IDLE) {
    s2idle_begin();
    }
    if (sync_on_suspend_enabled) {
    trace_suspend_resume(TPS("sync_filesystems"), 0, true);
    error = pm_sleep_fs_sync();
    if (error) {
// goto;
    }
    trace_suspend_resume(TPS("sync_filesystems"), 0, false);
    }
    pm_pr_dbg("Preparing system for sleep (%s)\n", mem_sleep_labels[state]);
    pm_suspend_clear_flags();
    error = suspend_prepare(state);
    if (error) {
// goto;
    }
    if (suspend_test(TEST_FREEZER)) {
// goto;
    }
    trace_suspend_resume(TPS("suspend_enter"), state, false);
    pm_pr_dbg("Suspending system (%s)\n", mem_sleep_labels[state]);
    error = suspend_devices_and_enter(state);
// label;
    events_check_enabled = false;
    pm_pr_dbg("Finishing wakeup.\n");
    suspend_finish();
// label;
    mutex_unlock(&system_transition_mutex);
    return error;
    }
//
// pm_suspend - Externally visible function for suspending the system.
// @state: System sleep state to enter.
//
// Check if the value of @state represents one of the supported states,
// execute enter_state() and update system suspend statistics.
//
#[no_mangle]
pub unsafe extern "C" fn pm_suspend(state: suspend_state_t) -> c_int {
    let mut error = 0;
    if (state <= PM_SUSPEND_ON || state >= PM_SUSPEND_MAX) {
    return -EINVAL;
    }
    pr_info!("suspend entry (%s)\n", mem_sleep_labels[state]);
    error = enter_state(state);
    dpm_save_errno(error);
    pr_info!("suspend exit\n");
    return error;
    }
    EXPORT_SYMBOL(pm_suspend);
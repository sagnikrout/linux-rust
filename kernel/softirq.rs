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
// label;
    - NET RX softirq. It is multithreaded and does not require
    any global serialization.
    - NET TX softirq. It kicks software netdevice queues, hence
    it is logically serialized per device, but this serialization
    is invisible to common code.
    - Tasklets: serialized wrt itself.
//

pub static mut irq_cpustat_t: usize = 0;
    EXPORT_PER_CPU_SYMBOL(irq_stat);

    static struct softirq_action softirq_vec[NR_SOFTIRQS] __cacheline_aligned_in_smp;
pub static mut struct task_struct *: usize = 0;
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
// Interrupts are disabled: no need to stop preemption
    let mut tsk = __this_cpu_read(ksoftirqd);
    if (tsk) {
    wake_up_process(tsk);
    }
    }

pub static mut int: usize = 0;
pub static mut int: usize = 0;
    EXPORT_PER_CPU_SYMBOL_GPL(hardirqs_enabled);
    EXPORT_PER_CPU_SYMBOL_GPL(hardirq_context);

pub static mut unsigned long: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn _local_interrupt_disable() {
    __local_interrupt_disable();
    }
    EXPORT_SYMBOL(_local_interrupt_disable);
#[no_mangle]
pub unsafe extern "C" fn _local_interrupt_enable() {
    __local_interrupt_enable();
    }
    EXPORT_SYMBOL(_local_interrupt_enable);

//
// Any 32bit architecture that still cares about performance should
// probably ensure this is near preempt_count.
//
pub static mut unsigned int: usize = 0;

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

    static DEFINE_PER_CPU(softirq_ctrl, softirq_ctrl) = {
    .lock	= INIT_LOCAL_LOCK(softirq_ctrl.lock),
    };

pub static mut bh_lock_key: usize = 0;
pub static mut lockdep_map: usize = 0;
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
    return __this_cpu_read(softirq_ctrl.cnt) != 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __local_bh_disable_ip(ip: c_ulong, cnt: c_uint) {
    let mut flags = 0;
    let mut newcnt = 0;
    WARN_ON_ONCE!(in_hardirq());
    lock_map_acquire_read(&bh_lock_map);
// First entry of a task into a BH disabled section?
    if (!current.softirq_disable_cnt) {
    if (preemptible()) {
    if (IS_ENABLED!(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK)) {
    local_lock(&softirq_ctrl.lock);
    }
    else {
    migrate_disable();
    }
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
    if (IS_ENABLED!(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK)) {
    newcnt = this_cpu_add_return(softirq_ctrl.cnt, cnt);
//
// Reflect the result in the task state to prevent recursion on the
// local lock and to make softirq_count() & al work.
//
    current.softirq_disable_cnt = newcnt;
    if (IS_ENABLED!(CONFIG_TRACE_IRQFLAGS) && newcnt == cnt) {
    raw_local_irq_save(flags);
    lockdep_softirqs_off(ip);
    raw_local_irq_restore(flags);
    }
    } else {
pub static mut sirq_dis: bool = false;
    if (!current.softirq_disable_cnt) {
    sirq_dis = true;
    }
    this_cpu_add(softirq_ctrl.cnt, cnt);
    current.softirq_disable_cnt += cnt;
    WARN_ON_ONCE!(current.softirq_disable_cnt < 0);
    if (IS_ENABLED!(CONFIG_TRACE_IRQFLAGS) && sirq_dis) {
    raw_local_irq_save(flags);
    lockdep_softirqs_off(ip);
    raw_local_irq_restore(flags);
    }
    }
    }
    EXPORT_SYMBOL(__local_bh_disable_ip);
#[no_mangle]
unsafe extern "C" fn __local_bh_enable(cnt: c_uint, unlock: bool) {
    let mut flags = 0;
pub static mut sirq_en: bool = false;
    let mut newcnt = 0;
    if (IS_ENABLED!(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK)) {
    DEBUG_LOCKS_WARN_ON(current.softirq_disable_cnt !=
    this_cpu_read(softirq_ctrl.cnt));
    if (softirq_count() == cnt) {
    sirq_en = true;
    }
    } else {
    if (current.softirq_disable_cnt == cnt) {
    sirq_en = true;
    }
    }
    if (IS_ENABLED!(CONFIG_TRACE_IRQFLAGS) && sirq_en) {
    raw_local_irq_save(flags);
    lockdep_softirqs_on(_RET_IP_);
    raw_local_irq_restore(flags);
    }
    if (IS_ENABLED!(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK)) {
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
    WARN_ON_ONCE!(current.softirq_disable_cnt < 0);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __local_bh_enable_ip(ip: c_ulong, cnt: c_uint) {
pub static mut preempt_on: bool = false;
    let mut flags = 0;
    let mut pending = 0;
    let mut curcnt = 0;
    WARN_ON_ONCE!(in_hardirq());
    lockdep_assert_irqs_enabled();
    lock_map_release(&bh_lock_map);
    local_irq_save(flags);
    if (IS_ENABLED!(CONFIG_PREEMPT_RT_NEEDS_BH_LOCK)) {
    curcnt = this_cpu_read(softirq_ctrl.cnt);
    }
    else {
    curcnt = current.softirq_disable_cnt;
    }
//
// If this is not reenabling soft interrupts, no point in trying to
// run pending ones.
//
    if (curcnt != cnt) {
// goto;
    }
    pending = local_softirq_pending();
    if (!pending) {
// goto;
    }
//
// If this was called from non preemptible context, wake up the
// softirq daemon.
//
    if (!preempt_on) {
    wakeup_softirqd();
// goto;
    }
//
// Adjust softirq count to SOFTIRQ_OFFSET which makes
// in_serving_softirq() become true.
//
    cnt = SOFTIRQ_OFFSET;
    __local_bh_enable(cnt, false);
    __do_softirq();
// label;
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
    __local_bh_disable_ip(_RET_IP_, SOFTIRQ_OFFSET);
    local_irq_disable();
    }
// Counterpart to ksoftirqd_run_begin()
#[no_mangle]
pub unsafe extern "C" fn ksoftirqd_run_end() {
// pairs with the lock_map_acquire_read() in ksoftirqd_run_begin()
    lock_map_release(&bh_lock_map);
    __local_bh_enable(SOFTIRQ_OFFSET, true);
    WARN_ON_ONCE!(in_interrupt());
    local_irq_enable();
    }
#[no_mangle]
pub unsafe extern "C" fn softirq_handle_begin() { }
#[no_mangle]
pub unsafe extern "C" fn softirq_handle_end() { }
#[no_mangle]
pub unsafe extern "C" fn should_wake_ksoftirqd() -> bool {
    return !this_cpu_read(softirq_ctrl.cnt);
    }
#[no_mangle]
pub unsafe extern "C" fn invoke_softirq() {
    if (should_wake_ksoftirqd()) {
    wakeup_softirqd();
    }
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
pub static mut is_pending: c_uint = 0;
    if (unlikely(was_pending != is_pending)) {
    WARN_ON_ONCE!(was_pending != (is_pending & ~SCHED_SOFTIRQ_MASK));
    invoke_softirq();
    }
    }

//
// This one is for softirq.c-internal use, where hardirqs are disabled
// legitimately:
//

#[no_mangle]
#[no_mangle]
// duplicate fn: __local_bh_disable_ip
pub unsafe extern "C" fn __local_bh_disable_ip_dup(ip: c_ulong, cnt: c_uint) {
    let mut flags = 0;
    WARN_ON_ONCE!(in_hardirq());
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
    if (softirq_count() == (cnt & SOFTIRQ_MASK)) {
    lockdep_softirqs_off(ip);
    }
    raw_local_irq_restore(flags);
    if (preempt_count() == cnt) {

    current.preempt_disable_ip = get_lock_parent_ip();

    trace_preempt_off(CALLER_ADDR0, get_lock_parent_ip());
    }
    }
    EXPORT_SYMBOL(__local_bh_disable_ip);

#[no_mangle]
unsafe extern "C" fn __local_bh_enable(cnt: c_uint) {
    lockdep_assert_irqs_disabled();
    if (preempt_count() == cnt) {
    trace_preempt_on(CALLER_ADDR0, get_lock_parent_ip());
    }
    if (softirq_count() == (cnt & SOFTIRQ_MASK)) {
    lockdep_softirqs_on(_RET_IP_);
    }
    __preempt_count_sub(cnt);
    }
//
// Special-case - softirqs can safely be enabled by __do_softirq(),
// without processing still-pending softirqs:
//
#[no_mangle]
pub unsafe extern "C" fn _local_bh_enable() {
    WARN_ON_ONCE!(in_hardirq());
    __local_bh_enable(SOFTIRQ_DISABLE_OFFSET);
    }
    EXPORT_SYMBOL(_local_bh_enable);
#[no_mangle]
#[no_mangle]
// duplicate fn: __local_bh_enable_ip
pub unsafe extern "C" fn __local_bh_enable_ip_dup(ip: c_ulong, cnt: c_uint) {
    WARN_ON_ONCE!(in_hardirq());
    lockdep_assert_irqs_enabled();

    local_irq_disable();

//
// Are softirqs going to be turned on now:
//
    if (softirq_count() == SOFTIRQ_DISABLE_OFFSET) {
    lockdep_softirqs_on(ip);
    }
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
#[no_mangle]
// duplicate fn: softirq_handle_begin
pub unsafe extern "C" fn softirq_handle_begin_dup() {
    __local_bh_disable_ip(_RET_IP_, SOFTIRQ_OFFSET);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: softirq_handle_end
pub unsafe extern "C" fn softirq_handle_end_dup() {
    __local_bh_enable(SOFTIRQ_OFFSET);
    WARN_ON_ONCE!(in_interrupt());
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: ksoftirqd_run_begin
pub unsafe extern "C" fn ksoftirqd_run_begin_dup() {
    local_irq_disable();
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: ksoftirqd_run_end
pub unsafe extern "C" fn ksoftirqd_run_end_dup() {
    local_irq_enable();
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: should_wake_ksoftirqd
pub unsafe extern "C" fn should_wake_ksoftirqd_dup() -> bool {
    return true;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: invoke_softirq
pub unsafe extern "C" fn invoke_softirq_dup() {
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
    let mut pending = 0;
    let mut flags = 0;
    if (in_interrupt()) {
    return;
    }
    local_irq_save(flags);
    pending = local_softirq_pending();
    if (pending) {
    do_softirq_own_stack();
    }
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
pub static mut in_hardirq: bool = false;
    if (lockdep_hardirq_context()) {
    in_hardirq = true;
    lockdep_hardirq_exit();
    }
    lockdep_softirq_enter();
    return in_hardirq;
    }
#[no_mangle]
pub unsafe extern "C" fn lockdep_softirq_end(in_hardirq: bool) {
    lockdep_softirq_exit();
    if (in_hardirq) {
    lockdep_hardirq_enter();
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: lockdep_softirq_start
pub unsafe extern "C" fn lockdep_softirq_start_dup() -> bool { return false; }
#[no_mangle]
#[no_mangle]
// duplicate fn: lockdep_softirq_end
pub unsafe extern "C" fn lockdep_softirq_end_dup(in_hardirq: bool) { }

#[no_mangle]
unsafe extern "C" fn handle_softirqs(ksirqd: bool) {
pub static mut end: c_ulong = 0;
pub static mut old_flags: c_ulong = 0;
pub static mut max_restart: c_int = 0;
pub static mut h: *mut c_void = core::ptr::null_mut();
    let mut in_hardirq = 0;
    let mut pending = 0;
    let mut softirq_bit = 0;
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
// label;
// Reset the pending bitmask before enabling irqs
    set_softirq_pending(0);
    local_irq_enable();
    h = softirq_vec;
    while ((softirq_bit = ffs(pending))) {
    let mut vec_nr = 0;
    let mut prev_count = 0;
    h += softirq_bit - 1;
    vec_nr = h - softirq_vec;
    prev_count = preempt_count();
    kstat_incr_softirqs_this_cpu(vec_nr);
    trace_softirq_entry(vec_nr);
    h.action();
    trace_softirq_exit(vec_nr);
    if (unlikely(prev_count != preempt_count())) {
    pr_err!("huh, entered softirq %u %s %p with preempt_count %08x, exited with %08x?\n",
    vec_nr, softirq_to_name[vec_nr], h.action,
    prev_count, preempt_count());
    preempt_count_set(prev_count);
    }
    h += 1;
    pending >>= softirq_bit;
    }
    if (!IS_ENABLED!(CONFIG_PREEMPT_RT) && ksirqd) {
    rcu_softirq_qs();
    }
    local_irq_disable();
    pending = local_softirq_pending();
    if (pending) {
    if (time_before(jiffies, end) && !need_resched() &&
    --max_restart) {
// goto;
    }
    wakeup_softirqd();
    }
    account_softirq_exit(current);
    lockdep_softirq_end(in_hardirq);
    softirq_handle_end();
    current_restore_flags(old_flags, PF_MEMALLOC);
    }
#[no_mangle]
pub unsafe extern "C" fn __do_softirq() -> asmlinkage __visible void __softirq_entry {
    handle_softirqs(false);
    }
//
// irq_enter_rcu - Enter an interrupt context with RCU watching
//
#[no_mangle]
pub unsafe extern "C" fn irq_enter_rcu() {
    __irq_enter_raw();
//
// If this is a nested interrupt that hits the exit_to_user_mode_loop
// where it has enabled interrupts but before it has hit schedule() we
// could have hrtimers in an undefined state. Fix it up here.
//
    hrtimer_rearm_deferred();
    if (tick_nohz_full_cpu(smp_processor_id()) ||
    (is_idle_task(current) && (irq_count() == HARDIRQ_OFFSET))) {
    tick_irq_enter();
    }
    account_hardirq_enter(current);
    }
//
// irq_enter - Enter an interrupt context including RCU update
//
#[no_mangle]
pub unsafe extern "C" fn irq_enter() {
    ct_irq_enter();
    irq_enter_rcu();
    }
#[no_mangle]
pub unsafe extern "C" fn tick_irq_exit() {

pub static mut cpu: c_int = 0;
// Make sure that timer wheel updates are propagated
    if ((sched_core_idle_cpu(cpu) && !need_resched()) || tick_nohz_full_cpu(cpu)) {
    if (!in_hardirq()) {
    tick_nohz_irq_exit();
    }
    }

    }

pub static mut struct task_struct *: usize = 0;
pub static mut unsigned long: usize = 0;
#[no_mangle]
unsafe extern "C" fn wake_timersd() {
    let mut tsk = __this_cpu_read(ktimerd);
    if (tsk) {
    wake_up_process(tsk);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn wake_timersd() { }

#[no_mangle]
pub unsafe extern "C" fn __irq_exit_rcu() {

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
    if (IS_ENABLED!(CONFIG_IRQ_FORCED_THREADING) && force_irqthreads() &&
    local_timers_pending_force_th() && !(in_nmi() | in_hardirq())) {
    wake_timersd();
    }
    tick_irq_exit();
    }
//
// irq_exit_rcu() - Exit an interrupt context without updating RCU
//
// Also processes softirqs if needed and possible.
//
#[no_mangle]
pub unsafe extern "C" fn irq_exit_rcu() {
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
    if (!in_interrupt() && should_wake_ksoftirqd()) {
    wakeup_softirqd();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn raise_softirq(nr: c_uint) {
    let mut flags = 0;
    local_irq_save(flags);
    raise_softirq_irqoff(nr);
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn __raise_softirq_irqoff(nr: c_uint) {
    lockdep_assert_irqs_disabled();
    trace_softirq_raise(nr);
    or_softirq_pending(1UL << nr);
    }
#[no_mangle]
pub unsafe extern "C" fn open_softirq(nr: c_int, (*action)(void): *mut c_void) {
#[no_mangle]
#[no_mangle]
// duplicate fn: open_softirq
pub unsafe extern "C" fn open_softirq_dup(nr: c_int) {
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
// static DEFINE_PER_CPU(tasklet_head, tasklet_vec);
// static DEFINE_PER_CPU(tasklet_head, tasklet_hi_vec);
#[no_mangle]
pub unsafe extern "C" fn __tasklet_schedule_common(t: *mut tasklet_struct, headp: *mut tasklet_head, softirq_nr: c_uint) {
pub static mut head: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
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
    __tasklet_schedule_common(t, &tasklet_vec,
    TASKLET_SOFTIRQ);
    }
    EXPORT_SYMBOL(__tasklet_schedule);
#[no_mangle]
pub unsafe extern "C" fn __tasklet_hi_schedule(t: *mut tasklet_struct) {
    __tasklet_schedule_common(t, &tasklet_hi_vec,
    HI_SOFTIRQ);
    }
    EXPORT_SYMBOL(__tasklet_hi_schedule);
#[no_mangle]
unsafe extern "C" fn tasklet_clear_sched(t: *mut tasklet_struct) -> bool {
    if (test_and_clear_wake_up_bit(TASKLET_STATE_SCHED, &t.state)) {
    return true;
    }
    WARN_ONCE(1, "tasklet SCHED state not set: %s %pS\n",
    t.use_callback ? "callback" : "func",
    t.use_callback ? t.callback : t.func);
    return false;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasklet_sync_callback {
    pub cb_lock: spinlock_t,
    pub cb_waiters: core::sync::atomic::AtomicI32,
}

    static DEFINE_PER_CPU(tasklet_sync_callback, tasklet_sync_callback) = {
    .cb_lock	= __SPIN_LOCK_UNLOCKED(tasklet_sync_callback.cb_lock),
    .cb_waiters	= ATOMIC_INIT(0),
    };
#[no_mangle]
unsafe extern "C" fn tasklet_lock_callback() {
    spin_lock(this_cpu_ptr(&tasklet_sync_callback.cb_lock));
    }
#[no_mangle]
unsafe extern "C" fn tasklet_unlock_callback() {
    spin_unlock(this_cpu_ptr(&tasklet_sync_callback.cb_lock));
    }
#[no_mangle]
unsafe extern "C" fn tasklet_callback_cancel_wait_running() {
    let mut sync_cb = this_cpu_ptr(&tasklet_sync_callback);
    atomic_inc(&sync_cb.cb_waiters);
    spin_lock(&sync_cb.cb_lock);
    atomic_dec(&sync_cb.cb_waiters);
    spin_unlock(&sync_cb.cb_lock);
    }
#[no_mangle]
unsafe extern "C" fn tasklet_callback_sync_wait_running() {
    let mut sync_cb = this_cpu_ptr(&tasklet_sync_callback);
    if (atomic_read(&sync_cb.cb_waiters)) {
    spin_unlock(&sync_cb.cb_lock);
    spin_lock(&sync_cb.cb_lock);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn tasklet_lock_callback() { }
#[no_mangle]
pub unsafe extern "C" fn tasklet_unlock_callback() { }
#[no_mangle]
pub unsafe extern "C" fn tasklet_callback_sync_wait_running() { }

#[no_mangle]
pub unsafe extern "C" fn tasklet_callback_cancel_wait_running() { }

#[no_mangle]
pub unsafe extern "C" fn tasklet_action_common(tl_head: *mut tasklet_head, softirq_nr: c_uint) {
pub static mut list: *mut c_void = core::ptr::null_mut();
    local_irq_disable();
    list = tl_head.head;
    tl_head.head = core::ptr::null_mut();
    tl_head.tail = &tl_head.head;
    local_irq_enable();
    tasklet_lock_callback();
    while (list) {
    let mut t = list;
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
    workqueue_softirq_action(false);
    tasklet_action_common(this_cpu_ptr(&tasklet_vec), TASKLET_SOFTIRQ);
    }
#[no_mangle]
unsafe extern "C" fn tasklet_hi_action() -> __latent_entropy void {
    workqueue_softirq_action(true);
    tasklet_action_common(this_cpu_ptr(&tasklet_hi_vec), HI_SOFTIRQ);
    }
#[no_mangle]
pub unsafe extern "C" fn tasklet_setup(t: *mut tasklet_struct) {
    t.next = core::ptr::null_mut();
    t.state = 0;
    atomic_set(&t.count, 0);
    t.callback = callback;
    t.use_callback = true;
    t.data = 0;
    }
    EXPORT_SYMBOL(tasklet_setup);
#[no_mangle]
pub unsafe extern "C" fn tasklet_init(t: *mut tasklet_struct, data: c_ulong) {
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
    while (test_bit(TASKLET_STATE_RUN, &(t).state)) {
    if (IS_ENABLED!(CONFIG_PREEMPT_RT)) {
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
    if (in_interrupt()) {
    pr_notice("Attempt to kill tasklet from interrupt\n");
    }
    wait_on_bit_lock(&t.state, TASKLET_STATE_SCHED, TASK_UNINTERRUPTIBLE);
    tasklet_unlock_wait(t);
    tasklet_clear_sched(t);
    }
    EXPORT_SYMBOL(tasklet_kill);

#[no_mangle]
pub unsafe extern "C" fn tasklet_unlock(t: *mut tasklet_struct) {
    clear_and_wake_up_bit(TASKLET_STATE_RUN, &t.state);
    }
    EXPORT_SYMBOL_GPL(tasklet_unlock);
#[no_mangle]
pub unsafe extern "C" fn tasklet_unlock_wait(t: *mut tasklet_struct) {
    wait_on_bit(&t.state, TASKLET_STATE_RUN, TASK_UNINTERRUPTIBLE);
    }
    EXPORT_SYMBOL_GPL(tasklet_unlock_wait);

#[no_mangle]
pub unsafe extern "C" fn softirq_init() -> c_int {
    let mut cpu = 0;
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
    return local_softirq_pending();
    }
#[no_mangle]
unsafe extern "C" fn run_ksoftirqd(cpu: c_uint) {
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

pub static mut smp_hotplug_thread: usize = 0;

#[no_mangle]
unsafe extern "C" fn ktimerd_setup(cpu: c_uint) {
// Above SCHED_NORMAL to handle timers before regular tasks.
    sched_set_fifo_low(current);
    }
#[no_mangle]
unsafe extern "C" fn ktimerd_should_run(cpu: c_uint) -> c_int {
    return local_timers_pending_force_th();
    }
#[no_mangle]
pub unsafe extern "C" fn raise_ktimers_thread(nr: c_uint) {
    trace_softirq_raise(nr);
    __this_cpu_or(pending_timer_softirq, BIT(nr));
    }
#[no_mangle]
unsafe extern "C" fn run_ktimerd(cpu: c_uint) {
    let mut timer_si = 0;
    ksoftirqd_run_begin();
    timer_si = local_timers_pending_force_th();
    __this_cpu_write(pending_timer_softirq, 0);
    or_softirq_pending(timer_si);
    __do_softirq();
    ksoftirqd_run_end();
    }
pub static mut smp_hotplug_thread: usize = 0;

#[no_mangle]
unsafe extern "C" fn spawn_ksoftirqd() -> __init int {
    cpuhp_setup_state_nocalls(CPUHP_SOFTIRQ_DEAD, "softirq:dead", core::ptr::null_mut(),
    takeover_tasklets);
    BUG_ON!(smpboot_register_percpu_thread(&softirq_threads));

    if (force_irqthreads()) {
    BUG_ON!(smpboot_register_percpu_thread(&timer_thread));
    }

    return 0;
    }
    early_initcall!(spawn_ksoftirqd);
//
// [ These __weak aliases are kept in a separate compilation unit, so that
// GCC does not inline them incorrectly. ]
//
#[no_mangle]
pub unsafe extern "C" fn early_irq_init() -> c_int __weak {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_probe_nr_irqs() -> c_int __weak {
    return NR_IRQS_LEGACY;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_early_irq_init() -> c_int __weak {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_dynirq_lower_bound(from: c_uint) -> unsigned int __weak {
    return from;
    }

}
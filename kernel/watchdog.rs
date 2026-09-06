//! Automatically rewritten from C to Rust
//! Source: kernel/watchdog.c
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
// Detect hard and soft lockups on a system
//
// started by Don Zickus, Copyright (C) 2010 Red Hat, Inc.
//
// Note: Most of this code is borrowed heavily from the original softlockup
// detector, so thanks to Ingo for the initial implementation.
// Some chunks also taken from the old x86-specific nmi watchdog code, thanks
// to those contributors as well.
//

pub static mut watchdog_mutex: usize = 0;

pub const NUM_SAMPLE_PERIODS: c_int = 5;
    let mut watchdog_enabled = 0;
pub static mut watchdog_user_enabled: int  = 1;
pub static mut watchdog_hardlockup_user_enabled: int  = 0;
pub static mut watchdog_softlockup_user_enabled: int  = 1;
pub static mut watchdog_thresh: int  = 10;
    static int  watchdog_thresh_next;
    static int  watchdog_hardlockup_available;
    let mut watchdog_cpumask: cpumask = unsafe { core::mem::zeroed() };
    let mut watchdog_cpumask_bits = cpumask_bits(&watchdog_cpumask);

    let mut sysctl_hardlockup_all_cpu_backtrace = 0;

//
// Number of consecutive missed interrupts before declaring a lockup.
// Default to 1 (immediate) for NMI/Perf. Buddy will overwrite this to 3.
//
pub static mut watchdog_hardlockup_miss_thresh: int  = 1;
    EXPORT_SYMBOL_GPL(watchdog_hardlockup_miss_thresh);
//
// Should we panic when a soft-lockup or hard-lockup occurs:
//
    let mut hardlockup_panic = IS_ENABLED!(CONFIG_BOOTPARAM_HARDLOCKUP_PANIC);
//
// bitmasks to control what kinds of system info to be printed when
// hard lockup is detected, it could be task, memory, lock etc.
// Refer include/linux/sys_info.h for detailed bit definition.
//
    let mut hardlockup_si_mask = 0;

    static unsigned int hardlockup_count;
#[no_mangle]
pub unsafe extern "C" fn hardlockup_count_show(kobj: *mut kobject, attr: *mut kobj_attribute, page: *mut c_char) -> ssize_t {
    return sysfs_emit(page, "%u\n", hardlockup_count);
    }
pub static mut hardlockup_count_attr: kobj_attribute = 0;
#[no_mangle]
unsafe extern "C" fn kernel_hardlockup_sysfs_init() -> __init int {
    sysfs_add_file_to_group(kernel_kobj, &hardlockup_count_attr.attr, core::ptr::null_mut());
    return 0;
    }
    late_initcall!(kernel_hardlockup_sysfs_init);

//
// We may not want to enable hard lockup detection by default in all cases,
// for example when running the kernel as a guest on a hypervisor. In these
// cases this function can be called to disable hard lockup detection. This
// function should only be executed once by the boot processor before the
// kernel command line parameters are parsed, because otherwise it is not
// possible to override this in hardlockup_panic_setup().
//
#[no_mangle]
pub unsafe extern "C" fn hardlockup_detector_disable()  {
    watchdog_hardlockup_user_enabled = 0;
    }
#[no_mangle]
unsafe extern "C" fn hardlockup_panic_setup(str: *mut c_char) -> c_int {
// label;
    if (!strncmp(str, "panic", 5)) {
    hardlockup_panic = 1;
    }

    else if (!strncmp(str, "nopanic", 7)) {
    hardlockup_panic = 0;
    }

    else if (!strncmp(str, "0", 1)) {
    watchdog_hardlockup_user_enabled = 0;
    }

    else if (!strncmp(str, "1", 1)) {
    watchdog_hardlockup_user_enabled = 1;
    }

    else if (!strncmp(str, "r", 1)) {
    hardlockup_config_perf_event(str + 1);
    }
    while (*(str++)) {
    if (*str == ',') {
    str += 1;
// goto;
    }
    }
    return 1;
    }
    __setup!("nmi_watchdog=", hardlockup_panic_setup);

pub static mut atomic_t: usize = 0;
pub static mut int: usize = 0;
pub static mut int: usize = 0;
pub static mut bool: usize = 0;
pub static mut bool: usize = 0;
    static unsigned long hard_lockup_nmi_warn;
#[no_mangle]
pub unsafe extern "C" fn arch_touch_nmi_watchdog() -> notrace void {
//
// Using __raw here because some code paths have
// preemption enabled.  If preemption is enabled
// then interrupts should be enabled too, in which
// case we shouldn't have to worry about the watchdog
// going off.
//
    raw_cpu_write(watchdog_hardlockup_touched, true);
    }
    EXPORT_SYMBOL(arch_touch_nmi_watchdog);
#[no_mangle]
pub unsafe extern "C" fn watchdog_hardlockup_touch_cpu(cpu: c_uint) {
    per_cpu(watchdog_hardlockup_touched, cpu) = true;
    }
#[no_mangle]
unsafe extern "C" fn watchdog_hardlockup_update_reset(cpu: c_uint) {
pub static mut hrint: c_int = 0;
//
// NOTE: we don't need any fancy atomic_t or READ_ONCE/WRITE_ONCE
// for hrtimer_interrupts_saved. hrtimer_interrupts_saved is
// written/read by a single CPU.
//
    per_cpu(hrtimer_interrupts_saved, cpu) = hrint;
    per_cpu(hrtimer_interrupts_missed, cpu) = 0;
    }
#[no_mangle]
unsafe extern "C" fn is_hardlockup(cpu: c_uint) -> bool {
pub static mut hrint: c_int = 0;
    if (per_cpu(hrtimer_interrupts_saved, cpu) != hrint) {
    watchdog_hardlockup_update_reset(cpu);
    return false;
    }
    per_cpu(hrtimer_interrupts_missed, cpu)++;
    if (per_cpu(hrtimer_interrupts_missed, cpu) % watchdog_hardlockup_miss_thresh) {
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn watchdog_hardlockup_kick() {
    let mut new_interrupts = 0;
    new_interrupts = atomic_inc_return(this_cpu_ptr(&hrtimer_interrupts));
    watchdog_buddy_check_hardlockup(new_interrupts);
    }
#[no_mangle]
pub unsafe extern "C" fn watchdog_hardlockup_check(cpu: c_uint, regs: *mut pt_regs) {
    let mut hardlockup_all_cpu_backtrace = 0;
    let mut this_cpu = 0;
    let mut flags = 0;
    if (per_cpu(watchdog_hardlockup_touched, cpu)) {
    watchdog_hardlockup_update_reset(cpu);
    per_cpu(watchdog_hardlockup_touched, cpu) = false;
    return;
    }
    hardlockup_all_cpu_backtrace = (hardlockup_si_mask & SYS_INFO_ALL_BT) ?
    1 : sysctl_hardlockup_all_cpu_backtrace;
//
// Check for a hardlockup by making sure the CPU's timer
// interrupt is incrementing. The timer interrupt should have
// fired multiple times before we overflow'd. If it hasn't
// then this is a good indication the cpu is stuck
//
    if (!is_hardlockup(cpu)) {
    per_cpu(watchdog_hardlockup_warned, cpu) = false;
    return;
    }

    hardlockup_count += 1;

//
// A poorly behaving BPF scheduler can trigger hard lockup by
// e.g. putting numerous affinitized tasks in a single queue and
// directing all CPUs at it. The following call can return true
// only once when sched_ext is enabled and will immediately
// abort the BPF scheduler and print out a warning message.
//
    if (scx_hardlockup(cpu)) {
    return;
    }
// Only print hardlockups once.
    if (per_cpu(watchdog_hardlockup_warned, cpu)) {
    return;
    }
//
// Prevent multiple hard-lockup reports if one cpu is already
// engaged in dumping all cpu back traces.
//
    if (hardlockup_all_cpu_backtrace) {
    if (test_and_set_bit_lock(0, &hard_lockup_nmi_warn)) {
    return;
    }
    }
//
// NOTE: we call printk_cpu_sync_get_irqsave() after printing
// the lockup message. While it would be nice to serialize
// that printout, we really want to make sure that if some
// other CPU somehow locked up while holding the lock associated
// with printk_cpu_sync_get_irqsave() that we can still at least
// get the message about the lockup out.
//
    this_cpu = smp_processor_id();
    pr_emerg("CPU%u: Watchdog detected hard LOCKUP on cpu %u\n", this_cpu, cpu);
    printk_cpu_sync_get_irqsave(flags);
    print_modules();
    print_irqtrace_events(current);
    if (cpu == this_cpu) {
    if (regs) {
    show_regs(regs);
    }
    else {
    dump_stack();
    }
    printk_cpu_sync_put_irqrestore(flags);
    } else {
    printk_cpu_sync_put_irqrestore(flags);
    trigger_single_cpu_backtrace(cpu);
    }
    if (hardlockup_all_cpu_backtrace) {
    trigger_allbutcpu_cpu_backtrace(cpu);
    if (!hardlockup_panic) {
    clear_bit_unlock(0, &hard_lockup_nmi_warn);
    }
    }
    sys_info(hardlockup_si_mask & ~SYS_INFO_ALL_BT);
    if (hardlockup_panic) {
    nmi_panic(regs, "Hard LOCKUP");
    }
    per_cpu(watchdog_hardlockup_warned, cpu) = true;
    }

#[no_mangle]
pub unsafe extern "C" fn watchdog_hardlockup_kick() { }

//
// These functions can be overridden based on the configured hardlockdup detector.
//
// watchdog_hardlockup_enable/disable can be implemented to start and stop when
// softlockup watchdog start and stop. The detector must select the
// SOFTLOCKUP_DETECTOR Kconfig.
//
    void __weak watchdog_hardlockup_enable(unsigned int cpu) { }
    void __weak watchdog_hardlockup_disable(unsigned int cpu) { }
//
// Watchdog-detector specific API.
//
// Return 0 when hardlockup watchdog is available, negative value otherwise.
// Note that the negative value means that a delayed probe might
// succeed later.
//
#[no_mangle]
pub unsafe extern "C" fn watchdog_hardlockup_probe() -> int __weak __init {
    return -ENODEV;
    }
//
// watchdog_hardlockup_stop - Stop the watchdog for reconfiguration
//
// The reconfiguration steps are:
// watchdog_hardlockup_stop();
// update_variables();
// watchdog_hardlockup_start();
//
    void __weak watchdog_hardlockup_stop(void) { }
//
// watchdog_hardlockup_start - Start the watchdog after reconfiguration
//
// Counterpart to watchdog_hardlockup_stop().
//
// The following variables have been updated in update_variables() and
// contain the currently valid configuration:
// - watchdog_enabled
// - watchdog_thresh
// - watchdog_cpumask
//
    void __weak watchdog_hardlockup_start(void) { }
//
// lockup_detector_update_enable - Update the sysctl enable bit
//
// Caller needs to make sure that the hard watchdogs are off, so this
// can't race with watchdog_hardlockup_disable().
//
#[no_mangle]
unsafe extern "C" fn lockup_detector_update_enable() {
    watchdog_enabled = 0;
    if (!watchdog_user_enabled) {
    return;
    }
    if (watchdog_hardlockup_available && watchdog_hardlockup_user_enabled) {
    watchdog_enabled |= WATCHDOG_HARDLOCKUP_ENABLED;
    }
    if (watchdog_softlockup_user_enabled) {
    watchdog_enabled |= WATCHDOG_SOFTLOCKUP_ENABLED;
    }
    }

//
// Delay the softlockup report when running a known slow code.
// It does _not_ affect the timestamp of the last successful reschedule.
//

    let mut sysctl_softlockup_all_cpu_backtrace = 0;

//
// bitmasks to control what kinds of system info to be printed when
// soft lockup is detected, it could be task, memory, lock etc.
// Refer include/linux/sys_info.h for detailed bit definition.
//
    static unsigned long softlockup_si_mask;
    static struct cpumask watchdog_allowed_mask ;
// Global variables, exported for sysctl
    let mut softlockup_panic = CONFIG_BOOTPARAM_SOFTLOCKUP_PANIC;
    static bool softlockup_initialized ;
    static u64  sample_period;

    static unsigned int softlockup_count;
#[no_mangle]
pub unsafe extern "C" fn softlockup_count_show(kobj: *mut kobject, attr: *mut kobj_attribute, page: *mut c_char) -> ssize_t {
    return sysfs_emit(page, "%u\n", softlockup_count);
    }
pub static mut softlockup_count_attr: kobj_attribute = 0;
#[no_mangle]
unsafe extern "C" fn kernel_softlockup_sysfs_init() -> __init int {
    sysfs_add_file_to_group(kernel_kobj, &softlockup_count_attr.attr, core::ptr::null_mut());
    return 0;
    }
    late_initcall!(kernel_softlockup_sysfs_init);

// Timestamp taken after the last successful reschedule.
pub static mut unsigned long: usize = 0;
// Timestamp of the last softlockup report.
pub static mut unsigned long: usize = 0;
pub static mut struct hrtimer: usize = 0;
pub static mut bool: usize = 0;
    static unsigned long soft_lockup_nmi_warn;
#[no_mangle]
unsafe extern "C" fn softlockup_panic_setup(str: *mut c_char) -> c_int {
    softlockup_panic = simple_strtoul(str, core::ptr::null_mut(), 0);
    return 1;
    }
    __setup!("softlockup_panic=", softlockup_panic_setup);
#[no_mangle]
unsafe extern "C" fn nowatchdog_setup(str: *mut c_char) -> c_int {
    watchdog_user_enabled = 0;
    return 1;
    }
    __setup!("nowatchdog", nowatchdog_setup);
#[no_mangle]
unsafe extern "C" fn nosoftlockup_setup(str: *mut c_char) -> c_int {
    watchdog_softlockup_user_enabled = 0;
    return 1;
    }
    __setup!("nosoftlockup", nosoftlockup_setup);
#[no_mangle]
unsafe extern "C" fn watchdog_thresh_setup(str: *mut c_char) -> c_int {
    get_option(&str, &watchdog_thresh);
    return 1;
    }
    __setup!("watchdog_thresh=", watchdog_thresh_setup);

    enum stats_per_group {
    STATS_SYSTEM,
    STATS_SOFTIRQ,
    STATS_HARDIRQ,
    STATS_IDLE,
    NUM_STATS_PER_GROUP,
    };
    static const enum cpu_usage_stat tracked_stats[NUM_STATS_PER_GROUP] = {
    CPUTIME_SYSTEM,
    CPUTIME_SOFTIRQ,
    CPUTIME_IRQ,
    CPUTIME_IDLE,
    };
pub static mut u16: usize = 0;
pub static mut u8: usize = 0;
pub static mut u8: usize = 0;
//
// We don't need nanosecond resolution. A granularity of 16ms is
// sufficient for our precision, allowing us to use u16 to store
// cpustats, which will roll over roughly every ~1000 seconds.
// 2^24 ~= 16 * 10^6
//
#[no_mangle]
unsafe extern "C" fn get_16bit_precision(data_ns: u64) -> u16 {
//
// 2^24ns ~= 16.8ms
// Round to the nearest multiple of 16.8 milliseconds.
//
    return (data_ns + (1 << 23)) >> 24LL;
    }
#[no_mangle]
unsafe extern "C" fn update_cpustat() {
    let mut i = 0;
    let mut util = 0;
    u16 old_stat, new_stat;
pub static mut kcpustat: usize = 0;
    let mut cpustat = kcpustat.cpustat;
pub static mut tail: u8 = 0;
pub static mut sample_period_16: u16 = 0;
    kcpustat_cpu_fetch(&kcpustat, smp_processor_id());
    while (i < NUM_STATS_PER_GROUP) {
    old_stat = __this_cpu_read(cpustat_old[i]);
    new_stat = get_16bit_precision(cpustat[tracked_stats[i]]);
    util = DIV_ROUND_UP(100 * (new_stat - old_stat), sample_period_16);
//
// Since we use 16-bit precision, the raw data will undergo
// integer division, which may sometimes result in data loss,
// and then result might exceed 100%. To avoid confusion,
// we enforce a 100% display cap when calculations exceed this threshold.
//
    if (util > 100) {
    util = 100;
    }
    __this_cpu_write(cpustat_util[tail][i], util);
    __this_cpu_write(cpustat_old[i], new_stat);
    }
    __this_cpu_write(cpustat_tail, (tail + 1) % NUM_SAMPLE_PERIODS);
    }
#[no_mangle]
unsafe extern "C" fn print_cpustat() {
    let mut i = 0;
    let mut group = 0;
pub static mut tail: u8 = 0;
pub static mut sample_period_msecond: u64 = 0;
    do_div(sample_period_msecond, NSEC_PER_MSEC);
//
// Outputting the "watchdog" prefix on every line is redundant and not
// concise, and the original alarm information is sufficient for
// positioning in logs, hence here printk() is used instead of pr_crit().
//
    printk("CPU#%d Utilization every %llums during lockup:\n",
    smp_processor_id(), sample_period_msecond);
    while (i < NUM_SAMPLE_PERIODS) {
    group = (tail + i) % NUM_SAMPLE_PERIODS;
    printk("\t#%d: %3u%% system,\t%3u%% softirq,\t"
    "%3u%% hardirq,\t%3u%% idle\n", i + 1,
    __this_cpu_read(cpustat_util[group][STATS_SYSTEM]),
    __this_cpu_read(cpustat_util[group][STATS_SOFTIRQ]),
    __this_cpu_read(cpustat_util[group][STATS_HARDIRQ]),
    __this_cpu_read(cpustat_util[group][STATS_IDLE]));
    }
    }
pub const HARDIRQ_PERCENT_THRESH: c_int = 50;
pub const NUM_HARDIRQ_REPORT: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_counts {
    pub irq: c_int,
    pub counts: u32,
}

pub static mut bool: usize = 0;
// Tabulate the most frequent interrupts.
#[no_mangle]
unsafe extern "C" fn tabulate_irq_count(irq_counts: *mut irq_counts, irq: c_int, counts: u32, rank: c_int) {
    let mut i = 0;
pub static mut new_count: irq_counts = 0;
    while (i < rank) {
    if (counts > irq_counts[i].counts) {
    swap(new_count, irq_counts[i]);
    }
    }
    }
//
// If the hardirq time exceeds HARDIRQ_PERCENT_THRESH% of the sample_period,
// then the cause of softlockup might be interrupt storm. In this case, it
// would be useful to start interrupt counting.
//
#[no_mangle]
unsafe extern "C" fn need_counting_irqs() -> bool {
    let mut util = 0;
pub static mut tail: c_int = 0;
    tail = (tail + NUM_SAMPLE_PERIODS - 1) % NUM_SAMPLE_PERIODS;
    util = __this_cpu_read(cpustat_util[tail][STATS_HARDIRQ]);
    return util > HARDIRQ_PERCENT_THRESH;
    }
#[no_mangle]
unsafe extern "C" fn start_counting_irqs() {
    if (!__this_cpu_read(snapshot_taken)) {
    kstat_snapshot_irqs();
    __this_cpu_write(snapshot_taken, true);
    }
    }
#[no_mangle]
unsafe extern "C" fn stop_counting_irqs() {
    __this_cpu_write(snapshot_taken, false);
    }
#[no_mangle]
unsafe extern "C" fn print_irq_counts() {
    let mut i = 0;
    let mut count = 0;
pub static mut irq_counts: usize = 0;
    if (__this_cpu_read(snapshot_taken)) {
    for_each_active_irq(i) {
    count = kstat_get_irq_since_snapshot(i);
    tabulate_irq_count(irq_counts_sorted, i, count, NUM_HARDIRQ_REPORT);
    }
//
// Outputting the "watchdog" prefix on every line is redundant and not
// concise, and the original alarm information is sufficient for
// positioning in logs, hence here printk() is used instead of pr_crit().
//
    printk("CPU#%d Detect HardIRQ Time exceeds %d%%. Most frequent HardIRQs:\n",
    smp_processor_id(), HARDIRQ_PERCENT_THRESH);
    while (i < NUM_HARDIRQ_REPORT) {
    if (irq_counts_sorted[i].irq == -1) {
    break;
    }
    printk("\t#%u: %-10u\tirq#%d\n",
    i + 1, irq_counts_sorted[i].counts,
    irq_counts_sorted[i].irq);
    }
//
// If the hardirq time is less than HARDIRQ_PERCENT_THRESH% in the last
// sample_period, then we suspect the interrupt storm might be subsiding.
//
    if (!need_counting_irqs()) {
    stop_counting_irqs();
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn report_cpu_status() {
    print_cpustat();
    print_irq_counts();
    }

#[no_mangle]
pub unsafe extern "C" fn update_cpustat() { }
#[no_mangle]
pub unsafe extern "C" fn report_cpu_status() { }
#[no_mangle]
pub unsafe extern "C" fn need_counting_irqs() -> bool { return false; }
#[no_mangle]
pub unsafe extern "C" fn start_counting_irqs() { }
#[no_mangle]
pub unsafe extern "C" fn stop_counting_irqs() { }

//
// Hard-lockup warnings should be triggered after just a few seconds. Soft-
// lockups can have false positives under extreme conditions. So we generally
// want a higher threshold for soft lockups than for hard lockups. So we couple
// the thresholds with a factor: we make the soft threshold twice the amount of
// time the hard threshold is.
//
#[no_mangle]
unsafe extern "C" fn get_softlockup_thresh() -> c_int {
    return watchdog_thresh * 2;
    }
//
// Returns seconds, approximately.  We don't need nanosecond
// resolution, and we don't need to waste time with a big divide when
// 2^30ns == 1.074s.
//
#[no_mangle]
unsafe extern "C" fn get_timestamp() -> c_ulong {
    return running_clock() >> 30LL;  /* 2^30 ~= 10^9 */
    }
#[no_mangle]
unsafe extern "C" fn set_sample_period() {
//
// convert watchdog_thresh from seconds to ns
// the divide by 5 is to give hrtimer several chances (two
// or three with the current relation between the soft
// and hard thresholds) to increment before the
// hardlockup detector generates a warning
//
    sample_period = get_softlockup_thresh() * ((u64)NSEC_PER_SEC / NUM_SAMPLE_PERIODS);
    watchdog_update_hrtimer_threshold(sample_period);
    }
#[no_mangle]
unsafe extern "C" fn update_report_ts() {
    __this_cpu_write(watchdog_report_ts, get_timestamp());
    }
// Commands for resetting the watchdog
#[no_mangle]
unsafe extern "C" fn update_touch_ts() {
    __this_cpu_write(watchdog_touch_ts, get_timestamp());
    update_report_ts();
    }
//
// touch_softlockup_watchdog_sched - touch watchdog on scheduler stalls
//
// Call when the scheduler may have stalled for legitimate reasons
// preventing the watchdog task from executing - e.g. the scheduler
// entering idle state.  This should only be used for scheduler events.
// Use touch_softlockup_watchdog() for everything else.
//
#[no_mangle]
pub unsafe extern "C" fn touch_softlockup_watchdog_sched() -> notrace void {
//
// Preemption can be enabled.  It doesn't matter which CPU's watchdog
// report period gets restarted here, so use the raw_ operation.
//
    raw_cpu_write(watchdog_report_ts, SOFTLOCKUP_DELAY_REPORT);
    }
#[no_mangle]
pub unsafe extern "C" fn touch_softlockup_watchdog() -> notrace void {
    touch_softlockup_watchdog_sched();
    wq_watchdog_touch(raw_smp_processor_id());
    }
    EXPORT_SYMBOL(touch_softlockup_watchdog);
#[no_mangle]
pub unsafe extern "C" fn touch_all_softlockup_watchdogs() {
    let mut cpu = 0;
//
// watchdog_mutex cannpt be taken here, as this might be called
// from (soft)interrupt context, so the access to
// watchdog_allowed_cpumask might race with a concurrent update.
//
// The watchdog time stamp can race against a concurrent real
// update as well, the only side effect might be a cycle delay for
// the softlockup check.
//
    for_each_cpu(cpu, &watchdog_allowed_mask) {
    per_cpu(watchdog_report_ts, cpu) = SOFTLOCKUP_DELAY_REPORT;
    wq_watchdog_touch(cpu);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn touch_softlockup_watchdog_sync() {
    __this_cpu_write(softlockup_touch_sync, true);
    __this_cpu_write(watchdog_report_ts, SOFTLOCKUP_DELAY_REPORT);
    }
#[no_mangle]
pub unsafe extern "C" fn is_softlockup(touch_ts: c_ulong, period_ts: c_ulong, now: c_ulong) -> c_int {
    if ((watchdog_enabled & WATCHDOG_SOFTLOCKUP_ENABLED) && watchdog_thresh) {
//
// If period_ts has not been updated during a sample_period, then
// in the subsequent few sample_periods, period_ts might also not
// be updated, which could indicate a potential softlockup. In
// this case, if we suspect the cause of the potential softlockup
// might be interrupt storm, then we need to count the interrupts
// to find which interrupt is storming.
//
    if (time_after_eq(now, period_ts + get_softlockup_thresh() / NUM_SAMPLE_PERIODS) &&
    need_counting_irqs()) {
    start_counting_irqs();
    }
//
// A poorly behaving BPF scheduler can live-lock the system into
// soft lockups. Tell sched_ext to try ejecting the BPF
// scheduler when close to a soft lockup.
//
    if (time_after_eq(now, period_ts + get_softlockup_thresh() * 3 / 4)) {
    scx_softlockup(now - touch_ts);
    }
// Warn about unreasonable delays.
    if (time_after(now, period_ts + get_softlockup_thresh())) {
    return now - touch_ts;
    }
    }
    return 0;
    }
// watchdog detector functions
pub static mut struct completion: usize = 0;
pub static mut struct cpu_stop_work: usize = 0;
//
// The watchdog feed function - touches the timestamp.
//
// It only runs once every sample_period seconds (4 seconds by
// default) to reset the softlockup timestamp. If this gets delayed
// for more than 2*watchdog_thresh seconds then the debug-printout
// triggers in watchdog_timer_fn().
//
#[no_mangle]
unsafe extern "C" fn softlockup_fn(data: *mut c_void) -> c_int {
    update_touch_ts();
    stop_counting_irqs();
    complete(this_cpu_ptr(&softlockup_completion));
    return 0;
    }
// watchdog kicker functions
#[no_mangle]
unsafe extern "C" fn watchdog_timer_fn(hrtimer: *mut hrtimer) -> enum hrtimer_restart {
    unsigned long touch_ts, period_ts, now;
    let mut regs = get_irq_regs();
    let mut softlockup_all_cpu_backtrace = 0;
    let mut duration = 0;
    let mut thresh_count = 0;
    let mut flags = 0;
    if (!watchdog_enabled) {
    return HRTIMER_NORESTART;
    }
//
// pass the buddy check if a panic is in process
//
    if (panic_in_progress()) {
    return HRTIMER_NORESTART;
    }
    softlockup_all_cpu_backtrace = (softlockup_si_mask & SYS_INFO_ALL_BT) ?
    1 : sysctl_softlockup_all_cpu_backtrace;
    watchdog_hardlockup_kick();
// kick the softlockup detector
    if (completion_done(this_cpu_ptr(&softlockup_completion))) {
    reinit_completion(this_cpu_ptr(&softlockup_completion));
    stop_one_cpu_nowait(smp_processor_id(),
    softlockup_fn, core::ptr::null_mut(),
    this_cpu_ptr(&softlockup_stop_work));
    }
// .. and repeat
    hrtimer_forward_now(hrtimer, ns_to_ktime(sample_period));
//
// Read the current timestamp first. It might become invalid anytime
// when a virtual machine is stopped by the host or when the watchog
// is touched from NMI.
//
    now = get_timestamp();
//
// If a virtual machine is stopped by the host it can look to
// the watchdog like a soft lockup. This function touches the watchdog.
//
    kvm_check_and_clear_guest_paused();
//
// The stored timestamp is comparable with @now only when not touched.
// It might get touched anytime from NMI. Make sure that is_softlockup()
// uses the same (valid) value.
//
    period_ts = READ_ONCE(*this_cpu_ptr(&watchdog_report_ts));
    update_cpustat();
// Reset the interval when touched by known problematic code.
    if (period_ts == SOFTLOCKUP_DELAY_REPORT) {
    if (unlikely(__this_cpu_read(softlockup_touch_sync))) {
//
// If the time stamp was touched atomically
// make sure the scheduler tick is up to date.
//
    __this_cpu_write(softlockup_touch_sync, false);
    sched_clock_tick();
    }
    update_report_ts();
    return HRTIMER_RESTART;
    }
// Check for a softlockup.
    touch_ts = __this_cpu_read(watchdog_touch_ts);
    duration = is_softlockup(touch_ts, period_ts, now);
    if (unlikely(duration)) {

    softlockup_count += 1;

//
// Prevent multiple soft-lockup reports if one cpu is already
// engaged in dumping all cpu back traces.
//
    if (softlockup_all_cpu_backtrace) {
    if (test_and_set_bit_lock(0, &soft_lockup_nmi_warn)) {
    return HRTIMER_RESTART;
    }
    }
// Start period for the next softlockup warning.
    update_report_ts();
    printk_cpu_sync_get_irqsave(flags);
    pr_emerg("BUG: soft lockup - CPU#%d stuck for %us! [%s:%d]\n",
    smp_processor_id(), duration,
    current.comm, task_pid_nr(current));
    report_cpu_status();
    print_modules();
    print_irqtrace_events(current);
    if (regs) {
    show_regs(regs);
    }
    else {
    dump_stack();
    }
    printk_cpu_sync_put_irqrestore(flags);
    if (softlockup_all_cpu_backtrace) {
    trigger_allbutcpu_cpu_backtrace(smp_processor_id());
    if (!softlockup_panic) {
    clear_bit_unlock(0, &soft_lockup_nmi_warn);
    }
    }
    add_taint(TAINT_SOFTLOCKUP, LOCKDEP_STILL_OK);
    sys_info(softlockup_si_mask & ~SYS_INFO_ALL_BT);
    thresh_count = duration / get_softlockup_thresh();
    if (softlockup_panic && thresh_count >= softlockup_panic) {
    panic("softlockup: hung tasks");
    }
    }
    return HRTIMER_RESTART;
    }
#[no_mangle]
unsafe extern "C" fn watchdog_enable(cpu: c_uint) {
    let mut hrtimer = this_cpu_ptr(&watchdog_hrtimer);
    let mut done = this_cpu_ptr(&softlockup_completion);
    WARN_ON_ONCE!(cpu != smp_processor_id());
    init_completion(done);
    complete(done);
//
// Start the timer first to prevent the hardlockup watchdog triggering
// before the timer has a chance to fire.
//
    hrtimer_setup(hrtimer, watchdog_timer_fn, CLOCK_MONOTONIC, HRTIMER_MODE_REL_HARD);
    hrtimer_start(hrtimer, ns_to_ktime(sample_period),
    HRTIMER_MODE_REL_PINNED_HARD);
// Initialize timestamp
    update_touch_ts();
// Enable the hardlockup detector
    if (watchdog_enabled & WATCHDOG_HARDLOCKUP_ENABLED) {
    watchdog_hardlockup_enable(cpu);
    }
    }
#[no_mangle]
unsafe extern "C" fn watchdog_disable(cpu: c_uint) {
    let mut hrtimer = this_cpu_ptr(&watchdog_hrtimer);
    WARN_ON_ONCE!(cpu != smp_processor_id());
//
// Disable the hardlockup detector first. That prevents that a large
// delay between disabling the timer and disabling the hardlockup
// detector causes a false positive.
//
    watchdog_hardlockup_disable(cpu);
    hrtimer_cancel(hrtimer);
    wait_for_completion(this_cpu_ptr(&softlockup_completion));
    }
#[no_mangle]
unsafe extern "C" fn softlockup_stop_fn(data: *mut c_void) -> c_int {
    watchdog_disable(smp_processor_id());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn softlockup_stop_all() {
    let mut cpu = 0;
    if (!softlockup_initialized) {
    return;
    }
    for_each_cpu(cpu, &watchdog_allowed_mask) {
    smp_call_on_cpu(cpu, softlockup_stop_fn, core::ptr::null_mut(), false);
    }
    cpumask_clear(&watchdog_allowed_mask);
    }
#[no_mangle]
unsafe extern "C" fn softlockup_start_fn(data: *mut c_void) -> c_int {
    watchdog_enable(smp_processor_id());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn softlockup_start_all() {
    let mut cpu = 0;
    cpumask_copy(&watchdog_allowed_mask, &watchdog_cpumask);
    for_each_cpu(cpu, &watchdog_allowed_mask) {
    smp_call_on_cpu(cpu, softlockup_start_fn, core::ptr::null_mut(), false);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn lockup_detector_online_cpu(cpu: c_uint) -> c_int {
    if (cpumask_test_cpu(cpu, &watchdog_allowed_mask)) {
    watchdog_enable(cpu);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn lockup_detector_offline_cpu(cpu: c_uint) -> c_int {
    if (cpumask_test_cpu(cpu, &watchdog_allowed_mask)) {
    watchdog_disable(cpu);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __lockup_detector_reconfigure(thresh_changed: bool) {
    cpus_read_lock();
    watchdog_hardlockup_stop();
    softlockup_stop_all();
//
// To prevent watchdog_timer_fn from using the old interval and
// the new watchdog_thresh at the same time, which could lead to
// false softlockup reports, it is necessary to update the
// watchdog_thresh after the softlockup is completed.
//
    if (thresh_changed) {
    watchdog_thresh = READ_ONCE(watchdog_thresh_next);
    }
    set_sample_period();
    lockup_detector_update_enable();
    if (watchdog_enabled && watchdog_thresh) {
    softlockup_start_all();
    }
    watchdog_hardlockup_start();
    cpus_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn lockup_detector_reconfigure() {
    mutex_lock(&watchdog_mutex);
    __lockup_detector_reconfigure(false);
    mutex_unlock(&watchdog_mutex);
    }
//
// Create the watchdog infrastructure and configure the detector(s).
//
#[no_mangle]
unsafe extern "C" fn lockup_detector_setup() -> __init void {
//
// If sysctl is off and watchdog got disabled on the command line,
// nothing to do here.
//
    lockup_detector_update_enable();
    if (!IS_ENABLED!(CONFIG_SYSCTL) &&
    !(watchdog_enabled && watchdog_thresh)) {
    return;
    }
    mutex_lock(&watchdog_mutex);
    __lockup_detector_reconfigure(false);
    softlockup_initialized = true;
    mutex_unlock(&watchdog_mutex);
    }

#[no_mangle]
unsafe extern "C" fn __lockup_detector_reconfigure(thresh_changed: bool) {
    cpus_read_lock();
    watchdog_hardlockup_stop();
    if (thresh_changed) {
    watchdog_thresh = READ_ONCE(watchdog_thresh_next);
    }
    lockup_detector_update_enable();
    watchdog_hardlockup_start();
    cpus_read_unlock();
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: lockup_detector_reconfigure
pub unsafe extern "C" fn lockup_detector_reconfigure_dup() {
    __lockup_detector_reconfigure(false);
    }
#[no_mangle]
pub unsafe extern "C" fn lockup_detector_setup() {
    __lockup_detector_reconfigure(false);
    }

//
// lockup_detector_soft_poweroff - Interface to stop lockup detector(s)
//
// Special interface for parisc. It prevents lockup detector warnings from
// the default pm_poweroff() function which busy loops forever.
//
#[no_mangle]
pub unsafe extern "C" fn lockup_detector_soft_poweroff() {
    watchdog_enabled = 0;
    }

// Propagate any changes to the watchdog infrastructure
#[no_mangle]
unsafe extern "C" fn proc_watchdog_update(thresh_changed: bool) {
// Remove impossible cpus to keep sysctl output clean.
    cpumask_and(&watchdog_cpumask, &watchdog_cpumask, cpu_possible_mask);
    __lockup_detector_reconfigure(thresh_changed);
    }
//
// common function for watchdog, nmi_watchdog and soft_watchdog parameter
//
// caller             | table->data points to            | 'which'
// -------------------|----------------------------------|-------------------------------
// proc_watchdog      | watchdog_user_enabled            | WATCHDOG_HARDLOCKUP_ENABLED |
// |                                  | WATCHDOG_SOFTLOCKUP_ENABLED
// -------------------|----------------------------------|-------------------------------
// proc_nmi_watchdog  | watchdog_hardlockup_user_enabled | WATCHDOG_HARDLOCKUP_ENABLED
// -------------------|----------------------------------|-------------------------------
// proc_soft_watchdog | watchdog_softlockup_user_enabled | WATCHDOG_SOFTLOCKUP_ENABLED
//
#[no_mangle]
pub unsafe extern "C" fn proc_watchdog_common(which: c_int, table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    int err, old, *param = table.data;
    mutex_lock(&watchdog_mutex);
    old = *param;
    if (!write) {
//
// On read synchronize the userspace interface. This is a
// racy snapshot.
//
// param = (watchdog_enabled & which) != 0;
    err = proc_dointvec_minmax(table, write, buffer, lenp, ppos);
// param = old;
    } else {
    err = proc_dointvec_minmax(table, write, buffer, lenp, ppos);
    if (!err && old != READ_ONCE(*param)) {
    proc_watchdog_update(false);
    }
    }
    mutex_unlock(&watchdog_mutex);
    return err;
    }
//
// /proc/sys/kernel/watchdog
//
#[no_mangle]
pub unsafe extern "C" fn proc_watchdog(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return proc_watchdog_common(WATCHDOG_HARDLOCKUP_ENABLED |
    WATCHDOG_SOFTLOCKUP_ENABLED,
    table, write, buffer, lenp, ppos);
    }
//
// /proc/sys/kernel/nmi_watchdog
//
#[no_mangle]
pub unsafe extern "C" fn proc_nmi_watchdog(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    if (!watchdog_hardlockup_available && write) {
    return -ENOTSUPP;
    }
    return proc_watchdog_common(WATCHDOG_HARDLOCKUP_ENABLED,
    table, write, buffer, lenp, ppos);
    }

//
// /proc/sys/kernel/soft_watchdog
//
#[no_mangle]
pub unsafe extern "C" fn proc_soft_watchdog(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return proc_watchdog_common(WATCHDOG_SOFTLOCKUP_ENABLED,
    table, write, buffer, lenp, ppos);
    }

//
// /proc/sys/kernel/watchdog_thresh
//
#[no_mangle]
pub unsafe extern "C" fn proc_watchdog_thresh(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut err = 0;
    let mut old = 0;
    mutex_lock(&watchdog_mutex);
    watchdog_thresh_next = READ_ONCE(watchdog_thresh);
    old = watchdog_thresh_next;
    err = proc_dointvec_minmax(table, write, buffer, lenp, ppos);
    if (!err && write && old != READ_ONCE(watchdog_thresh_next)) {
    proc_watchdog_update(true);
    }
    mutex_unlock(&watchdog_mutex);
    return err;
    }
//
// The cpumask is the mask of possible cpus that the watchdog can run
// on, not the mask of cpus it is actually running on.  This allows the
// user to specify a mask that will include cpus that have not yet
// been brought online, if desired.
//
#[no_mangle]
pub unsafe extern "C" fn proc_watchdog_cpumask(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut err = 0;
    mutex_lock(&watchdog_mutex);
    err = proc_do_large_bitmap(table, write, buffer, lenp, ppos);
    if (!err && write) {
    proc_watchdog_update(false);
    }
    mutex_unlock(&watchdog_mutex);
    return err;
    }
pub static mut sixty: int = 60;
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn watchdog_sysctl_init()  {
    register_sysctl_init("kernel", watchdog_sysctls);
    }

    static void __init lockup_detector_delay_init(work_struct *work);
    static bool allow_lockup_detector_init_retry __initdata;
    static struct work_struct detector_work __initdata =
    __WORK_INITIALIZER(detector_work, lockup_detector_delay_init);
#[no_mangle]
unsafe extern "C" fn lockup_detector_delay_init(work: *mut work_struct)  {
    let mut ret = 0;
    ret = watchdog_hardlockup_probe();
    if (ret) {
    if (ret == -ENODEV) {
    pr_info!("NMI not fully supported\n");
    }
    else {
    pr_info!("Delayed init of the lockup detector failed: %d\n", ret);
    }
    pr_info!("Hard watchdog permanently disabled\n");
    return;
    }
    allow_lockup_detector_init_retry = false;
    watchdog_hardlockup_available = true;
    lockup_detector_setup();
    }
//
// lockup_detector_retry_init - retry init lockup detector if possible.
//
// Retry hardlockup detector init. It is useful when it requires some
// functionality that has to be initialized later on a particular
// platform.
//
#[no_mangle]
pub unsafe extern "C" fn lockup_detector_retry_init()  {
// Must be called before late init calls
    if (!allow_lockup_detector_init_retry) {
    return;
    }
    schedule_work(&detector_work);
    }
//
// Ensure that optional delayed hardlockup init is proceed before
// the init code and memory is freed.
//
#[no_mangle]
unsafe extern "C" fn lockup_detector_check() -> c_int {
// Prevent any later retry.
    allow_lockup_detector_init_retry = false;
// Make sure no work is pending.
    flush_work(&detector_work);
    watchdog_sysctl_init();
    return 0;
    }
    late_initcall_sync!(lockup_detector_check);
#[no_mangle]
pub unsafe extern "C" fn lockup_detector_init()  {
    if (tick_nohz_full_enabled()) {
    pr_info!("Disabling watchdog on nohz_full cores by default\n");
    }
    cpumask_copy(&watchdog_cpumask,
    housekeeping_cpumask(HK_TYPE_TIMER));
    if (!watchdog_hardlockup_probe()) {
    watchdog_hardlockup_available = true;
    }
    else {
    allow_lockup_detector_init_retry = true;
    }
    lockup_detector_setup();
    }
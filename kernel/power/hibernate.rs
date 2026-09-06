//! Automatically rewritten from C to Rust
//! Source: kernel/power/hibernate.c
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
// kernel/power/hibernate.c - Hibernation (a.k.a suspend-to-disk) support.
//
// Copyright (c) 2003 Patrick Mochel
// Copyright (c) 2003 Open Source Development Lab
// Copyright (c) 2004 Pavel Machek <pavel@ucw.cz>
// Copyright (c) 2009 Rafael J. Wysocki, Novell Inc.
// Copyright (C) 2012 Bojan Smojver <bojan@rexursive.com>
//

    static int nocompress;
    static int noresume;
    static int nohibernate;
    static int resume_wait;
    static unsigned int resume_delay;
    static char resume_file[256] = CONFIG_PM_STD_PARTITION;
    let mut swsusp_resume_device;
    let mut swsusp_resume_block;
    __visible int in_suspend __nosavedata;
    static char hibernate_compressor[CRYPTO_MAX_ALG_NAME] = CONFIG_HIBERNATION_DEF_COMP;
//
// Compression/decompression algorithm to be used while saving/loading
// image to/from disk. This would later be used in 'kernel/power/swap.c'
// to allocate comp streams.
//
    char hib_comp_algo[CRYPTO_MAX_ALG_NAME];
    enum {
    HIBERNATION_INVALID,
    HIBERNATION_PLATFORM,
    HIBERNATION_SHUTDOWN,
    HIBERNATION_REBOOT,

    HIBERNATION_SUSPEND,

    HIBERNATION_TEST_RESUME,
// keep last
    __HIBERNATION_AFTER_LAST
    };

pub static mut hibernation_mode: int = 0;
    let mut freezer_test_done = 0;
pub static mut hibernation_ops: *mut c_void = core::ptr::null_mut();
pub static mut hibernate_atomic: atomic_t = 0;

//
// pm_hibernation_mode_is_suspend - Check if hibernation has been set to suspend
//
#[no_mangle]
pub unsafe extern "C" fn pm_hibernation_mode_is_suspend() -> bool {
pub static mut hibernation_mode: return = 0;
    }
    EXPORT_SYMBOL_GPL(pm_hibernation_mode_is_suspend);

#[no_mangle]
pub unsafe extern "C" fn hibernate_acquire() -> bool {
    return atomic_add_unless(&hibernate_atomic, -1, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn hibernate_release() {
    atomic_inc(&hibernate_atomic);
    }
#[no_mangle]
pub unsafe extern "C" fn hibernation_in_progress() -> bool {
    return !atomic_read(&hibernate_atomic);
    }
#[no_mangle]
pub unsafe extern "C" fn hibernation_available() -> bool {
    return nohibernate == 0 &&
    !security_locked_down(LOCKDOWN_HIBERNATION) &&
    !secretmem_active() && !cxl_mem_active();
    }
//
// hibernation_set_ops - Set the global hibernate operations.
// @ops: Hibernation operations to use in subsequent hibernation transitions.
//
#[no_mangle]
pub unsafe extern "C" fn hibernation_set_ops(ops: *const platform_hibernation_ops) {
    let mut sleep_flags = 0;
    if (ops && !(ops.begin && ops.end &&  ops.pre_snapshot
    && ops.prepare && ops.finish && ops.enter && ops.pre_restore
    && ops.restore_cleanup && ops.leave)) {
    WARN_ON!(1);
    return;
    }
    sleep_flags = lock_system_sleep();
    hibernation_ops = ops;
    if (ops) {
    hibernation_mode = HIBERNATION_PLATFORM;
    }

    else if (hibernation_mode == HIBERNATION_PLATFORM) {
    hibernation_mode = HIBERNATION_SHUTDOWN;
    }
    unlock_system_sleep(sleep_flags);
    }
    EXPORT_SYMBOL_GPL(hibernation_set_ops);
    static bool entering_platform_hibernation;
#[no_mangle]
pub unsafe extern "C" fn system_entering_hibernation() -> bool {
    return entering_platform_hibernation;
    }
    EXPORT_SYMBOL(system_entering_hibernation);

pub static mut pm_test_delay: unsigned int = 5;
    module_param!(pm_test_delay, uint, 0644);
    MODULE_PARM_DESC(pm_test_delay,
    "Number of seconds to wait before resuming from hibernation test");
#[no_mangle]
unsafe extern "C" fn hibernation_debug_sleep() {
    pr_info!("hibernation debug: Waiting for %d second(s).\n",
    pm_test_delay);
    mdelay(pm_test_delay * 1000);
    }
#[no_mangle]
unsafe extern "C" fn hibernation_test(level: c_int) -> c_int {
    if (pm_test_level == level) {
    hibernation_debug_sleep();
    return 1;
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn hibernation_test(level: c_int) -> c_int { return 0; }

//
// platform_begin - Call platform to start hibernation.
// @platform_mode: Whether or not to use the platform driver.
//
#[no_mangle]
unsafe extern "C" fn platform_begin(platform_mode: c_int) -> c_int {
    return (platform_mode && hibernation_ops) ?
    hibernation_ops.begin(PMSG_FREEZE) : 0;
    }
//
// platform_end - Call platform to finish transition to the working state.
// @platform_mode: Whether or not to use the platform driver.
//
#[no_mangle]
unsafe extern "C" fn platform_end(platform_mode: c_int) {
    if (platform_mode && hibernation_ops) {
    hibernation_ops.end();
    }
    }
//
// platform_pre_snapshot - Call platform to prepare the machine for hibernation.
// @platform_mode: Whether or not to use the platform driver.
//
// Use the platform driver to prepare the system for creating a hibernate image,
// if so configured, and return an error code if that fails.
//
#[no_mangle]
unsafe extern "C" fn platform_pre_snapshot(platform_mode: c_int) -> c_int {
    return (platform_mode && hibernation_ops) ?
    hibernation_ops.pre_snapshot() : 0;
    }
//
// platform_leave - Call platform to prepare a transition to the working state.
// @platform_mode: Whether or not to use the platform driver.
//
// Use the platform driver prepare to prepare the machine for switching to the
// normal mode of operation.
//
// This routine is called on one CPU with interrupts disabled.
//
#[no_mangle]
unsafe extern "C" fn platform_leave(platform_mode: c_int) {
    if (platform_mode && hibernation_ops) {
    hibernation_ops.leave();
    }
    }
//
// platform_finish - Call platform to switch the system to the working state.
// @platform_mode: Whether or not to use the platform driver.
//
// Use the platform driver to switch the machine to the normal mode of
// operation.
//
// This routine must be called after platform_prepare().
//
#[no_mangle]
unsafe extern "C" fn platform_finish(platform_mode: c_int) {
    if (platform_mode && hibernation_ops) {
    hibernation_ops.finish();
    }
    }
//
// platform_pre_restore - Prepare for hibernate image restoration.
// @platform_mode: Whether or not to use the platform driver.
//
// Use the platform driver to prepare the system for resume from a hibernation
// image.
//
// If the restore fails after this function has been called,
// platform_restore_cleanup() must be called.
//
#[no_mangle]
unsafe extern "C" fn platform_pre_restore(platform_mode: c_int) -> c_int {
    return (platform_mode && hibernation_ops) ?
    hibernation_ops.pre_restore() : 0;
    }
//
// platform_restore_cleanup - Switch to the working state after failing restore.
// @platform_mode: Whether or not to use the platform driver.
//
// Use the platform driver to switch the system to the normal mode of operation
// after a failing restore.
//
// If platform_pre_restore() has been called before the failing restore, this
// function must be called too, regardless of the result of
// platform_pre_restore().
//
#[no_mangle]
unsafe extern "C" fn platform_restore_cleanup(platform_mode: c_int) {
    if (platform_mode && hibernation_ops) {
    hibernation_ops.restore_cleanup();
    }
    }
//
// platform_recover - Recover from a failure to suspend devices.
// @platform_mode: Whether or not to use the platform driver.
//
#[no_mangle]
unsafe extern "C" fn platform_recover(platform_mode: c_int) {
    if (platform_mode && hibernation_ops && hibernation_ops.recover) {
    hibernation_ops.recover();
    }
    }
//
// swsusp_show_speed - Print time elapsed between two events during hibernation.
// @start: Starting event.
// @stop: Final event.
// @nr_pages: Number of memory pages processed between @start and @stop.
// @msg: Additional diagnostic message to print.
//
#[no_mangle]
pub unsafe extern "C" fn swsusp_show_speed(start: ktime_t, stop: ktime_t, nr_pages: c_uint, msg: *mut c_char) {
    let mut diff;
    let mut elapsed_centisecs64 = 0;
    let mut centisecs = 0;
    let mut k = 0;
    let mut kps = 0;
    diff = ktime_sub(stop, start);
    elapsed_centisecs64 = ktime_divns(diff, 10*NSEC_PER_MSEC);
    centisecs = elapsed_centisecs64;
    if (centisecs == 0) {
    centisecs = 1;	/* avoid div-by-zero */
    }
    k = nr_pages * (PAGE_SIZE / 1024);
    kps = (k * 100) / centisecs;
    pr_info!("%s %u kbytes in %u.%02u seconds (%u.%02u MB/s)\n",
    msg, k, centisecs / 100, centisecs % 100, kps / 1000,
    (kps % 1000) / 10);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_resume_nosmt() -> __weak int {
    return 0;
    }
//
// create_image - Create a hibernation image.
// @platform_mode: Whether or not to use the platform driver.
//
// Execute device drivers' "late" and "noirq" freeze callbacks, create a
// hibernation image and run the drivers' "noirq" and "early" thaw callbacks.
//
// Control reappears in this routine after the subsequent restore.
//
#[no_mangle]
unsafe extern "C" fn create_image(platform_mode: c_int) -> c_int {
    let mut error = 0;
    error = dpm_suspend_end(PMSG_FREEZE);
    if (error) {
    pr_err!("Some devices failed to power down, aborting\n");
    return error;
    }
    error = platform_pre_snapshot(platform_mode);
    if (error || hibernation_test(TEST_PLATFORM)) {
// goto;
    }
    error = pm_sleep_disable_secondary_cpus();
    if (error || hibernation_test(TEST_CPUS)) {
// goto;
    }
    local_irq_disable();
    system_state = SYSTEM_SUSPEND;
    error = syscore_suspend();
    if (error) {
    pr_err!("Some system devices failed to power down, aborting\n");
// goto;
    }
    if (hibernation_test(TEST_CORE) || pm_wakeup_pending()) {
// goto;
    }
    in_suspend = 1;
    save_processor_state();
    trace_suspend_resume(TPS("machine_suspend"), PM_EVENT_HIBERNATE, true);
    error = swsusp_arch_suspend();
// Restore control flow magically appears here
    restore_processor_state();
    trace_suspend_resume(TPS("machine_suspend"), PM_EVENT_HIBERNATE, false);
    if (error) {
    pr_err!("Error %d creating image\n", error);
    }
    if (!in_suspend) {
    events_check_enabled = false;
    clear_or_poison_free_pages();
    }
    platform_leave(platform_mode);
// label;
    syscore_resume();
// label;
    system_state = SYSTEM_RUNNING;
    local_irq_enable();
// label;
    pm_sleep_enable_secondary_cpus();
// Allow architectures to do nosmt-specific post-resume dances
    if (!in_suspend) {
    error = arch_resume_nosmt();
    }
// label;
    platform_finish(platform_mode);
    dpm_resume_start(in_suspend ?
    (error ? PMSG_RECOVER : PMSG_THAW) : PMSG_RESTORE);
    return error;
    }
//
// hibernation_snapshot - Quiesce devices and create a hibernation image.
// @platform_mode: If set, use platform driver to prepare for the transition.
//
// This routine must be called with system_transition_mutex held.
//
#[no_mangle]
pub unsafe extern "C" fn hibernation_snapshot(platform_mode: c_int) -> c_int {
    let mut msg;
    let mut error = 0;
    pm_suspend_clear_flags();
    error = platform_begin(platform_mode);
    if (error) {
// goto;
    }
    error = freeze_kernel_threads();
    if (error) {
// goto;
    }
    if (hibernation_test(TEST_FREEZER)) {
//
// Indicate to the caller that we are returning due to a
// successful freezer test.
//
    freezer_test_done = true;
// goto;
    }
    error = dpm_prepare(PMSG_FREEZE);
    if (error) {
// goto;
    }
// Preallocate image memory before shutting down devices.
    error = hibernate_preallocate_memory();
    if (error) {
// goto;
    }
    console_suspend_all();
    pm_restrict_gfp_mask();
    error = dpm_suspend(PMSG_FREEZE);
    if (error || hibernation_test(TEST_DEVICES)) {
    platform_recover(platform_mode);
    }
    else {
    error = create_image(platform_mode);
    }
//
// In the case that we call create_image() above, the control
// returns here (1) after the image has been created or the
// image creation has failed and (2) after a successful restore.
//
// We may need to release the preallocated image pages here.
    if (error || !in_suspend) {
    swsusp_free();
    }
    msg = in_suspend ? (error ? PMSG_RECOVER : PMSG_THAW) : PMSG_RESTORE;
    dpm_resume(msg);
    if (error || !in_suspend) {
    pm_restore_gfp_mask();
    }
    console_resume_all();
    dpm_complete(msg);
// label;
    platform_end(platform_mode);
    return error;
// label;
    dpm_complete(PMSG_RECOVER);
// label;
    thaw_kernel_threads();
// goto;
    }
#[no_mangle]
pub unsafe extern "C" fn hibernate_resume_nonboot_cpu_disable() -> int __weak {
    return suspend_disable_secondary_cpus();
    }
//
// resume_target_kernel - Restore system state from a hibernation image.
// @platform_mode: Whether or not to use the platform driver.
//
// Execute device drivers' "noirq" and "late" freeze callbacks, restore the
// contents of highmem that have not been restored yet from the image and run
// the low-level code that will restore the remaining contents of memory and
// switch to the just restored target kernel.
//
#[no_mangle]
unsafe extern "C" fn resume_target_kernel(platform_mode: bool) -> c_int {
    let mut error = 0;
    error = dpm_suspend_end(PMSG_QUIESCE);
    if (error) {
    pr_err!("Some devices failed to power down, aborting resume\n");
    return error;
    }
    error = platform_pre_restore(platform_mode);
    if (error) {
// goto;
    }
    cpuidle_pause();
    error = hibernate_resume_nonboot_cpu_disable();
    if (error) {
// goto;
    }
    local_irq_disable();
    system_state = SYSTEM_SUSPEND;
    error = syscore_suspend();
    if (error) {
// goto;
    }
    save_processor_state();
    error = restore_highmem();
    if (!error) {
    error = swsusp_arch_resume();
//
// The code below is only ever reached in case of a failure.
// Otherwise, execution continues at the place where
// swsusp_arch_suspend() was called.
//
    BUG_ON!(!error);
//
// This call to restore_highmem() reverts the changes made by
// the previous one.
//
    restore_highmem();
    }
//
// The only reason why swsusp_arch_resume() can fail is memory being
// very tight, so we have to free it as soon as we can to avoid
// subsequent failures.
//
    swsusp_free();
    restore_processor_state();
    touch_softlockup_watchdog();
    syscore_resume();
// label;
    system_state = SYSTEM_RUNNING;
    local_irq_enable();
// label;
    pm_sleep_enable_secondary_cpus();
// label;
    platform_restore_cleanup(platform_mode);
    dpm_resume_start(PMSG_RECOVER);
    return error;
    }
//
// hibernation_restore - Quiesce devices and restore from a hibernation image.
// @platform_mode: If set, use platform driver to prepare for the transition.
//
// This routine must be called with system_transition_mutex held.  If it is
// successful, control reappears in the restored target kernel in
// hibernation_snapshot().
//
#[no_mangle]
pub unsafe extern "C" fn hibernation_restore(platform_mode: c_int) -> c_int {
    let mut error = 0;
    pm_prepare_console();
    console_suspend_all();
    error = dpm_suspend_start(PMSG_QUIESCE);
    if (!error) {
    error = resume_target_kernel(platform_mode);
//
// The above should either succeed and jump to the new kernel,
// or return with an error. Otherwise things are just
// undefined, so let's be paranoid.
//
    BUG_ON!(!error);
    }
    dpm_resume_end(PMSG_RECOVER);
    console_resume_all();
    pm_restore_console();
    return error;
    }
//
// hibernation_platform_enter - Power off the system using the platform driver.
//
#[no_mangle]
pub unsafe extern "C" fn hibernation_platform_enter() -> c_int {
    let mut error = 0;
    if (!hibernation_ops) {
    return -ENOSYS;
    }
//
// We have cancelled the power transition by running
// hibernation_ops->finish() before saving the image, so we should let
// the firmware know that we're going to enter the sleep state after all
//
    error = hibernation_ops.begin(PMSG_HIBERNATE);
    if (error) {
// goto;
    }
    entering_platform_hibernation = true;
    console_suspend_all();
    error = dpm_suspend_start(PMSG_HIBERNATE);
    if (error) {
    if (hibernation_ops.recover) {
    hibernation_ops.recover();
    }
// goto;
    }
    error = dpm_suspend_end(PMSG_HIBERNATE);
    if (error) {
// goto;
    }
    error = hibernation_ops.prepare();
    if (error) {
// goto;
    }
    error = pm_sleep_disable_secondary_cpus();
    if (error) {
// goto;
    }
    local_irq_disable();
    system_state = SYSTEM_SUSPEND;
    error = syscore_suspend();
    if (error) {
// goto;
    }
    if (pm_wakeup_pending()) {
    error = -EAGAIN;
// goto;
    }
    hibernation_ops.enter();
// We should never get here
    while (1); {
// label;
    }
    syscore_resume();
// label;
    system_state = SYSTEM_RUNNING;
    local_irq_enable();
// label;
    pm_sleep_enable_secondary_cpus();
// label;
    hibernation_ops.finish();
    dpm_resume_start(PMSG_RESTORE);
// label;
    entering_platform_hibernation = false;
    dpm_resume_end(PMSG_RESTORE);
    console_resume_all();
// label;
    hibernation_ops.end();
    return error;
    }
//
// power_down - Shut the machine down for hibernation.
//
// Use the platform driver, if configured, to put the system into the sleep
// state corresponding to hibernation, or try to power it off or reboot,
// depending on the value of hibernation_mode.
//
#[no_mangle]
unsafe extern "C" fn power_down() {
    let mut error = 0;

    if (hibernation_mode == HIBERNATION_SUSPEND) {
    error = suspend_devices_and_enter(mem_sleep_current);
    if (!error) {
// goto;
    }
    hibernation_mode = hibernation_ops ? HIBERNATION_PLATFORM :
    HIBERNATION_SHUTDOWN;
    }

    match (hibernation_mode) {
    HIBERNATION_REBOOT => {
    kernel_restart(core::ptr::null_mut());
    // break;
    }
    HIBERNATION_PLATFORM => {
    error = hibernation_platform_enter();
    if (error == -EAGAIN || error == -EBUSY) {
    events_check_enabled = false;
    pr_info!("Wakeup event detected during hibernation, rolling back.\n");
// goto;
    }
    fallthrough;
    }
    HIBERNATION_SHUTDOWN => {
    if (kernel_can_power_off()) {
    entering_platform_hibernation = true;
    kernel_power_off();
    entering_platform_hibernation = false;
    }
    // break;
    }
    }
    kernel_halt();
//
// Valid image is on the disk, if we continue we risk serious data
// corruption after resume.
//
    pr_crit("Power down manually\n");
    while (1) {
    cpu_relax();
    }
// label;
// Restore swap signature.
    error = swsusp_unmark();
    if (error) {
    pr_err!("Swap will be unusable! Try swapon -a.\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn load_image_and_restore() -> c_int {
    let mut error = 0;
    let mut flags = 0;
    pm_pr_dbg("Loading hibernation image.\n");
    lock_device_hotplug();
    error = create_basic_memory_bitmaps();
    if (error) {
    swsusp_close();
// goto;
    }
    error = swsusp_read(&flags);
    swsusp_close();
    if (!error) {
    error = hibernation_restore(flags & SF_PLATFORM_MODE);
    }
    pr_err!("Failed to load image, recovering.\n");
    swsusp_free();
    free_basic_memory_bitmaps();
// label;
    unlock_device_hotplug();
    return error;
    }

//
// hibernate - Carry out system hibernation, including saving the image.
//
#[no_mangle]
pub unsafe extern "C" fn hibernate() -> c_int {
pub static mut snapshot_test: bool = false;
    let mut sleep_flags = 0;
    let mut error = 0;
    if (!hibernation_available()) {
    pm_pr_dbg("Hibernation not available.\n");
    return -EPERM;
    }
//
// Query for the compression algorithm support if compression is enabled.
//
    if (!nocompress) {
    strscpy(hib_comp_algo, hibernate_compressor);
    if (!crypto_has_acomp(hib_comp_algo, 0, CRYPTO_ALG_ASYNC)) {
    pr_err!("%s compression is not available\n", hib_comp_algo);
    return -EOPNOTSUPP;
    }
    }
    sleep_flags = lock_system_sleep();
// The snapshot device should not be opened while we're running
    if (!hibernate_acquire()) {
    error = -EBUSY;
// goto;
    }
    pr_info!("hibernation entry\n");
    pm_prepare_console();
    error = pm_notifier_call_chain_robust(PM_HIBERNATION_PREPARE, PM_POST_HIBERNATION);
    if (error) {
// goto;
    }
    error = pm_sleep_fs_sync();
    if (error) {
// goto;
    }
    filesystems_freeze(filesystem_freeze_enabled);
    error = freeze_processes();
    if (error) {
// goto;
    }
    lock_device_hotplug();
// Allocate memory management structures
    error = create_basic_memory_bitmaps();
    if (error) {
// goto;
    }
    error = hibernation_snapshot(hibernation_mode == HIBERNATION_PLATFORM);
    if (error || freezer_test_done) {
// goto;
    }
    if (in_suspend) {
pub static mut flags: c_uint = 0;
    if (hibernation_mode == HIBERNATION_PLATFORM) {
    flags |= SF_PLATFORM_MODE;
    }
    if (nocompress) {
    flags |= SF_NOCOMPRESS_MODE;
    } else {
    flags |= SF_CRC32_MODE;
//
// By default, LZO compression is enabled. Use SF_COMPRESSION_ALG_LZ4
// to override this behaviour and use LZ4.
//
// Refer kernel/power/power.h for more details
//
    if (!strcmp(hib_comp_algo, COMPRESSION_ALGO_LZ4)) {
    flags |= SF_COMPRESSION_ALG_LZ4;
    }
    else {
    flags |= SF_COMPRESSION_ALG_LZO;
    }
    }
    pm_pr_dbg("Writing hibernation image.\n");
    error = swsusp_write(flags);
    swsusp_free();
    if (!error) {
    if (hibernation_mode == HIBERNATION_TEST_RESUME) {
    snapshot_test = true;
    }
    else {
    power_down();
    }
    }
    in_suspend = 0;
    pm_restore_gfp_mask();
    } else {
    pm_pr_dbg("Hibernation image restored successfully.\n");
    }
// label;
    free_basic_memory_bitmaps();
// label;
    unlock_device_hotplug();
    if (snapshot_test) {
    pm_pr_dbg("Checking hibernation image\n");
    error = swsusp_check(false);
    if (!error) {
    error = load_image_and_restore();
    }
    }
    thaw_processes();
// Don't bother checking whether freezer_test_done is true
    freezer_test_done = false;
// label;
    filesystems_thaw();
// label;
    pm_notifier_call_chain(PM_POST_HIBERNATION);
// label;
    pm_restore_console();
    hibernate_release();
// label;
    unlock_system_sleep(sleep_flags);
    pr_info!("hibernation exit\n");
    return error;
    }
//
// hibernate_quiet_exec - Execute a function with all devices frozen.
// @func: Function to execute.
// @data: Data pointer to pass to @func.
//
// Return the @func return value or an error code if it cannot be executed.
//
#[no_mangle]
pub unsafe extern "C" fn hibernate_quiet_exec(data): *mut *mut int (func)(void, data: *mut c_void) -> c_int {
#[no_mangle]
#[no_mangle]
// duplicate fn: hibernate_quiet_exec
pub unsafe extern "C" fn hibernate_quiet_exec_dup(data: *mut c_void) -> c_int {
    let mut sleep_flags = 0;
    let mut error = 0;
    sleep_flags = lock_system_sleep();
    if (!hibernate_acquire()) {
    error = -EBUSY;
// goto;
    }
    pm_prepare_console();
    error = pm_notifier_call_chain_robust(PM_HIBERNATION_PREPARE, PM_POST_HIBERNATION);
    if (error) {
// goto;
    }
    filesystems_freeze(filesystem_freeze_enabled);
    error = freeze_processes();
    if (error) {
// goto;
    }
    lock_device_hotplug();
    pm_suspend_clear_flags();
    error = platform_begin(true);
    if (error) {
// goto;
    }
    error = freeze_kernel_threads();
    if (error) {
// goto;
    }
    error = dpm_prepare(PMSG_FREEZE);
    if (error) {
// goto;
    }
    console_suspend_all();
    error = dpm_suspend(PMSG_FREEZE);
    if (error) {
// goto;
    }
    error = dpm_suspend_end(PMSG_FREEZE);
    if (error) {
// goto;
    }
    error = platform_pre_snapshot(true);
    if (error) {
// goto;
    }
    error = func(data);
// label;
    platform_finish(true);
    dpm_resume_start(PMSG_THAW);
// label;
    dpm_resume(PMSG_THAW);
    console_resume_all();
// label;
    dpm_complete(PMSG_THAW);
    thaw_kernel_threads();
// label;
    platform_end(true);
    unlock_device_hotplug();
    thaw_processes();
// label;
    filesystems_thaw();
    pm_notifier_call_chain(PM_POST_HIBERNATION);
// label;
    pm_restore_console();
    hibernate_release();
// label;
    unlock_system_sleep(sleep_flags);
    return error;
    }
    EXPORT_SYMBOL_GPL(hibernate_quiet_exec);
#[no_mangle]
unsafe extern "C" fn find_resume_device() -> c_int {
    if (!strlen(resume_file)) {
    return -ENOENT;
    }
    pm_pr_dbg("Checking hibernation image partition %s\n", resume_file);
    if (resume_delay) {
    pr_info!("Waiting %dsec before reading resume device ...\n",
    resume_delay);
    ssleep(resume_delay);
    }
// Check if the device is there
    if (!early_lookup_bdev(resume_file, &swsusp_resume_device)) {
    return 0;
    }
//
// Some device discovery might still be in progress; we need to wait for
// this to finish.
//
    wait_for_device_probe();
    if (resume_wait) {
    while (early_lookup_bdev(resume_file, &swsusp_resume_device)) {
    msleep(10);
    }
    async_synchronize_full();
    }
    return early_lookup_bdev(resume_file, &swsusp_resume_device);
    }
#[no_mangle]
unsafe extern "C" fn software_resume() -> c_int {
    let mut error = 0;
    pm_pr_dbg("Hibernation image partition %d:%d present\n",
    MAJOR(swsusp_resume_device), MINOR(swsusp_resume_device));
    pm_pr_dbg("Looking for hibernation image.\n");
    mutex_lock(&system_transition_mutex);
    error = swsusp_check(true);
    if (error) {
// goto;
    }
//
// Check if the hibernation image is compressed. If so, query for
// the algorithm support.
//
    if (!(swsusp_header_flags & SF_NOCOMPRESS_MODE)) {
    if (swsusp_header_flags & SF_COMPRESSION_ALG_LZ4) {
    strscpy(hib_comp_algo, COMPRESSION_ALGO_LZ4);
    }
    else {
    strscpy(hib_comp_algo, COMPRESSION_ALGO_LZO);
    }
    if (!crypto_has_acomp(hib_comp_algo, 0, CRYPTO_ALG_ASYNC)) {
    pr_err!("%s compression is not available\n", hib_comp_algo);
    error = -EOPNOTSUPP;
// goto;
    }
    }
// The snapshot device should not be opened while we're running
    if (!hibernate_acquire()) {
    error = -EBUSY;
    swsusp_close();
// goto;
    }
    pr_info!("resume from hibernation\n");
    pm_prepare_console();
    error = pm_notifier_call_chain_robust(PM_RESTORE_PREPARE, PM_POST_RESTORE);
    if (error) {
// goto;
    }
    filesystems_freeze(filesystem_freeze_enabled);
    pm_pr_dbg("Preparing processes for hibernation restore.\n");
    error = freeze_processes();
    if (error) {
    filesystems_thaw();
// goto;
    }
    error = freeze_kernel_threads();
    if (error) {
    thaw_processes();
    filesystems_thaw();
// goto;
    }
    error = load_image_and_restore();
    thaw_processes();
    filesystems_thaw();
// label;
    pm_notifier_call_chain(PM_POST_RESTORE);
// label;
    pm_restore_console();
    pr_info!("resume failed (%d)\n", error);
    hibernate_release();
// For success case, the suspend path will release the lock
// label;
    mutex_unlock(&system_transition_mutex);
    pm_pr_dbg("Hibernation image not present or could not be loaded.\n");
    return error;
// label;
    swsusp_close();
// goto;
    }
//
// software_resume_initcall - Resume from a saved hibernation image.
//
// This routine is called as a late initcall, when all devices have been
// discovered and initialized already.
//
// The image reading code is called to see if there is a hibernation image
// available for reading.  If that is the case, devices are quiesced and the
// contents of memory is restored from the saved image.
//
// If this is successful, control reappears in the restored target kernel in
// hibernation_snapshot() which returns to hibernate().  Otherwise, the routine
// attempts to recover gracefully and make the kernel return to the normal mode
// of operation.
//
#[no_mangle]
unsafe extern "C" fn software_resume_initcall!() -> c_int {
//
// If the user said "noresume".. bail out early.
//
    if (noresume || !hibernation_available()) {
    return 0;
    }
    if (!swsusp_resume_device) {
pub static mut error: c_int = 0;
    if (error) {
    return error;
    }
    }
    return software_resume();
    }
    late_initcall_sync!(software_resume_initcall);
    static const char * const hibernation_modes[] = {
    [HIBERNATION_PLATFORM]	= "platform",
    [HIBERNATION_SHUTDOWN]	= "shutdown",
    [HIBERNATION_REBOOT]	= "reboot",

    [HIBERNATION_SUSPEND]	= "suspend",

    [HIBERNATION_TEST_RESUME]	= "test_resume",
    };
//
// /sys/power/disk - Control hibernation mode.
//
// Hibernation can be handled in several ways.  There are a few different ways
// to put the system into the sleep state: using the platform driver (e.g. ACPI
// or other hibernation_ops), powering it off or rebooting it (for testing
// mostly).
//
// The sysfs file /sys/power/disk provides an interface for selecting the
// hibernation mode to use.  Reading from this file causes the available modes
// to be printed.  There are 3 modes that can be supported:
//
// 'platform'
// 'shutdown'
// 'reboot'
//
// If a platform hibernation driver is in use, 'platform' will be supported
// and will be used by default.  Otherwise, 'shutdown' will be used by default.
// The selected option (i.e. the one corresponding to the current value of
// hibernation_mode) is enclosed by a square bracket.
//
// To select a given hibernation mode it is necessary to write the mode's
// string representation (as returned by reading from /sys/power/disk) back
// into /sys/power/disk.
//
#[no_mangle]
pub unsafe extern "C" fn disk_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
pub static mut count: isize = 0;
    let mut i = 0;
    if (!hibernation_available()) {
    return sysfs_emit(buf, "[disabled]\n");
    }
    while (i <= HIBERNATION_MAX) {
    if (!hibernation_modes[i]) {
    continue;
    }
    match (i) {
    HIBERNATION_SHUTDOWN => {
    }
    HIBERNATION_REBOOT => {

    }
    HIBERNATION_SUSPEND => {

    }
    HIBERNATION_TEST_RESUME => {
    // break;
    }
    HIBERNATION_PLATFORM => {
    if (hibernation_ops) {
    // break;
    }
// not a valid mode, continue with loop
    continue;
    }
    }
    if (i == hibernation_mode) {
    count += sysfs_emit_at(buf, count, "[%s] ", hibernation_modes[i]);
    }
    else {
    count += sysfs_emit_at(buf, count, "%s ", hibernation_modes[i]);
    }
    }
// Convert the last space to a newline if needed.
    if (count > 0) {
    buf[count - 1] = '\n';
    }
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn disk_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
pub static mut mode: c_int = 0;
    let mut sleep_flags = 0;
pub static mut error: c_int = 0;
    let mut len = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (!hibernation_available()) {
    return -EPERM;
    }
    p = memchr(buf, '\n', n);
    len = p ? p - buf : n;
    sleep_flags = lock_system_sleep();
    while (i <= HIBERNATION_MAX) {
    if (len == strlen(hibernation_modes[i])
    && !strncmp(buf, hibernation_modes[i], len)) {
    mode = i;
    break;
    }
    }
    if (mode != HIBERNATION_INVALID) {
    match (mode) {
    HIBERNATION_SHUTDOWN => {
    }
    HIBERNATION_REBOOT => {

    }
    HIBERNATION_SUSPEND => {

    }
    HIBERNATION_TEST_RESUME => {
    hibernation_mode = mode;
    // break;
    }
    HIBERNATION_PLATFORM => {
    if (hibernation_ops) {
    hibernation_mode = mode;
    }
    else {
    error = -EINVAL;
    }
    }
    }
    } else {
    error = -EINVAL;
    }
    if (!error) {
    pm_pr_dbg("Hibernation mode set to '%s'\n",
    hibernation_modes[mode]);
    }
    unlock_system_sleep(sleep_flags);
    return error ? error : n;
    }
    power_attr(disk);
#[no_mangle]
pub unsafe extern "C" fn resume_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%d:%d\n", MAJOR(swsusp_resume_device),
    MINOR(swsusp_resume_device));
    }
#[no_mangle]
pub unsafe extern "C" fn resume_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
    let mut sleep_flags = 0;
pub static mut len: c_int = 0;
pub static mut name: *mut c_void = core::ptr::null_mut();
    let mut dev;
    let mut error = 0;
    if (!hibernation_available()) {
    return n;
    }
    if (len && buf[len-1] == '\n') {
    len -= 1;
    }
    name = kstrndup(buf, len, GFP_KERNEL);
    if (!name) {
    return -ENOMEM;
    }
    error = lookup_bdev(name, &dev);
    if (error) {
    let mut maj = 0;
    let mut min = 0;
    let mut offset = 0;
    char *p, dummy;
    error = 0;
    if (sscanf(name, "%u:%u%c", &maj, &min, &dummy) == 2 ||
    sscanf(name, "%u:%u:%u:%c", &maj, &min, &offset,
    &dummy) == 3) {
    dev = MKDEV(maj, min);
    if (maj != MAJOR(dev) || min != MINOR(dev)) {
    error = -EINVAL;
    }
    } else {
    dev = new_decode_dev(simple_strtoul(name, &p, 16));
    if (*p) {
    error = -EINVAL;
    }
    }
    }
    kfree(name);
    if (error) {
    return error;
    }
    sleep_flags = lock_system_sleep();
    swsusp_resume_device = dev;
    unlock_system_sleep(sleep_flags);
    pm_pr_dbg("Configured hibernation resume from disk to %u\n",
    swsusp_resume_device);
    noresume = 0;
    software_resume();
    return n;
    }
    power_attr(resume);
#[no_mangle]
pub unsafe extern "C" fn resume_offset_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%llu\n", (unsigned long long)swsusp_resume_block);
    }
#[no_mangle]
pub unsafe extern "C" fn resume_offset_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
    unsigned long long offset;
    let mut rc = 0;
    rc = kstrtoull(buf, 0, &offset);
    if (rc) {
    return rc;
    }
    swsusp_resume_block = offset;
    return n;
    }
    power_attr(resume_offset);
#[no_mangle]
pub unsafe extern "C" fn image_size_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%lu\n", image_size);
    }
#[no_mangle]
pub unsafe extern "C" fn image_size_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
    let mut size = 0;
    if (sscanf(buf, "%lu", &size) == 1) {
    image_size = size;
    return n;
    }
    return -EINVAL;
    }
    power_attr(image_size);
#[no_mangle]
pub unsafe extern "C" fn reserved_size_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%lu\n", reserved_size);
    }
#[no_mangle]
pub unsafe extern "C" fn reserved_size_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
    let mut size = 0;
    if (sscanf(buf, "%lu", &size) == 1) {
    reserved_size = size;
    return n;
    }
    return -EINVAL;
    }
    power_attr(reserved_size);
    static struct attribute *g[] = {
    &disk_attr.attr,
    &resume_offset_attr.attr,
    &resume_attr.attr,
    &image_size_attr.attr,
    &reserved_size_attr.attr,
    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
#[no_mangle]
unsafe extern "C" fn pm_disk_init() -> c_int {
    return sysfs_create_group(power_kobj, &attr_group);
    }
    core_initcall!(pm_disk_init);
#[no_mangle]
unsafe extern "C" fn resume_setup(str: *mut c_char) -> c_int {
    if (noresume) {
    return 1;
    }
    strscpy(resume_file, str);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn resume_offset_setup(str: *mut c_char) -> c_int {
    unsigned long long offset;
    if (noresume) {
    return 1;
    }
    if (sscanf(str, "%llu", &offset) == 1) {
    swsusp_resume_block = offset;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn hibernate_setup(str: *mut c_char) -> c_int {
    if (!strncmp(str, "noresume", 8)) {
    noresume = 1;
    } else if (!strncmp(str, "nocompress", 10)) {
    nocompress = 1;
    } else if (!strncmp(str, "no", 2)) {
    noresume = 1;
    nohibernate = 1;
    } else if (IS_ENABLED!(CONFIG_STRICT_KERNEL_RWX)
    && !strncmp(str, "protect_image", 13)) {
    enable_restore_image_protection();
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn noresume_setup(str: *mut c_char) -> c_int {
    noresume = 1;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn resumewait_setup(str: *mut c_char) -> c_int {
    resume_wait = 1;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn resumedelay_setup(str: *mut c_char) -> c_int {
pub static mut rc: c_int = 0;
    if (rc) {
    pr_warn!("resumedelay: bad option string '%s'\n", str);
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn nohibernate_setup(str: *mut c_char) -> c_int {
    noresume = 1;
    nohibernate = 1;
    return 1;
    }
    static const char * const comp_alg_enabled[] = {

    COMPRESSION_ALGO_LZO,

    COMPRESSION_ALGO_LZ4,

    };
#[no_mangle]
pub unsafe extern "C" fn hibernate_compressor_param_set(compressor: *mut c_char, kp: *mut kernel_param) -> c_int {
    let mut index = 0;
    let mut ret = 0;
    if (!mutex_trylock(&system_transition_mutex)) {
    return -EBUSY;
    }
    index = sysfs_match_string(comp_alg_enabled, compressor);
    if (index >= 0) {
    ret = param_set_copystring(comp_alg_enabled[index], kp);
    if (!ret) {
    strscpy(hib_comp_algo, comp_alg_enabled[index]);
    }
    } else {
    ret = index;
    }
    mutex_unlock(&system_transition_mutex);
    if (ret) {
    pr_debug!("Cannot set specified compressor %s\n",
    compressor);
    }
    return ret;
    }
pub static mut kernel_param_ops: usize = 0;
pub static mut kparam_string: usize = 0;
    module_param_cb!(compressor, &hibernate_compressor_param_ops,
    &hibernate_compressor_param_string, 0644);
    MODULE_PARM_DESC(compressor,
    "Compression algorithm to be used with hibernation");
    __setup!("noresume", noresume_setup);
    __setup!("resume_offset=", resume_offset_setup);
    __setup!("resume=", resume_setup);
    __setup!("hibernate=", hibernate_setup);
    __setup!("resumewait", resumewait_setup);
    __setup!("resumedelay=", resumedelay_setup);
    __setup!("nohibernate", nohibernate_setup);
}

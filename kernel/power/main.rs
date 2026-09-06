//! Automatically rewritten from C to Rust
//! Source: kernel/power/main.c
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
// kernel/power/main.c - PM subsystem core functionality.
//
// Copyright (c) 2003 Patrick Mochel
// Copyright (c) 2003 Open Source Development Lab
//

//
// The following functions are used by the suspend/hibernate code to temporarily
// change gfp_allowed_mask in order to avoid using I/O during memory allocations
// while devices are suspended.  To avoid races with the suspend/hibernate code,
// they should always be called with system_transition_mutex held
// (gfp_allowed_mask also should only be modified with system_transition_mutex
// held, unless the suspend/hibernate code is guaranteed not to run in parallel
// with that modification).
//
    static unsigned int saved_gfp_count;
    static gfp_t saved_gfp_mask;
#[no_mangle]
pub unsafe extern "C" fn pm_restore_gfp_mask() {
    WARN_ON!(!mutex_is_locked(&system_transition_mutex));
    if (!saved_gfp_count || --saved_gfp_count) {
    return;
    }
    gfp_allowed_mask = saved_gfp_mask;
    saved_gfp_mask = 0;
    pm_pr_dbg("GFP mask restored\n");
    }
#[no_mangle]
pub unsafe extern "C" fn pm_restrict_gfp_mask() {
    WARN_ON!(!mutex_is_locked(&system_transition_mutex));
    if (saved_gfp_count++) {
    WARN_ON!((saved_gfp_mask & ~(__GFP_IO | __GFP_FS)) != gfp_allowed_mask);
    return;
    }
    saved_gfp_mask = gfp_allowed_mask;
    gfp_allowed_mask &= ~(__GFP_IO | __GFP_FS);
    pm_pr_dbg("GFP mask restricted\n");
    }
#[no_mangle]
pub unsafe extern "C" fn lock_system_sleep() -> c_uint {
pub static mut flags: c_uint = 0;
    current.flags |= PF_NOFREEZE;
    mutex_lock(&system_transition_mutex);
    return flags;
    }
    EXPORT_SYMBOL_GPL(lock_system_sleep);
#[no_mangle]
pub unsafe extern "C" fn unlock_system_sleep(flags: c_uint) {
    if (!(flags & PF_NOFREEZE)) {
    current.flags &= ~PF_NOFREEZE;
    }
    mutex_unlock(&system_transition_mutex);
    }
    EXPORT_SYMBOL_GPL(unlock_system_sleep);
#[no_mangle]
pub unsafe extern "C" fn ksys_sync_helper() {
    let mut start;
    let mut elapsed_msecs = 0;
    start = ktime_get();
    ksys_sync();
    elapsed_msecs = ktime_to_ms(ktime_sub(ktime_get(), start));
    pr_info!("Filesystems sync: %ld.%03ld seconds\n",
    elapsed_msecs / MSEC_PER_SEC, elapsed_msecs % MSEC_PER_SEC);
    }
    EXPORT_SYMBOL_GPL(ksys_sync_helper);

// Wakeup events handling resolution while syncing file systems in jiffies
pub const PM_FS_SYNC_WAKEUP_RESOLUTION: c_int = 5;
pub static mut pm_fs_sync_count: atomic_t = 0;
pub static mut pm_fs_sync_wq: *mut c_void = core::ptr::null_mut();
pub static mut pm_fs_sync_wait: usize = 0;
#[no_mangle]
unsafe extern "C" fn pm_fs_sync_completed() -> bool {
    return atomic_read(&pm_fs_sync_count) == 0;
    }
#[no_mangle]
unsafe extern "C" fn pm_fs_sync_work_fn(work: *mut work_struct) {
    ksys_sync_helper();
    if (atomic_dec_and_test(&pm_fs_sync_count)) {
    wake_up(&pm_fs_sync_wait);
    }
    }
pub static mut pm_fs_sync_work: usize = 0;
//
// pm_sleep_fs_sync() - Sync file systems in an interruptible way
//
// Return: 0 on successful file system sync, or -EBUSY if the file system sync
// was aborted.
//
#[no_mangle]
pub unsafe extern "C" fn pm_sleep_fs_sync() -> c_int {
    pm_wakeup_clear(0);
//
// Take back-to-back sleeps into account by queuing a subsequent fs sync
// only if the previous fs sync is running or is not queued. Multiple fs
// syncs increase the likelihood of saving the latest files immediately
// before sleep.
//
    if (!work_pending(&pm_fs_sync_work)) {
    atomic_inc(&pm_fs_sync_count);
    queue_work(pm_fs_sync_wq, &pm_fs_sync_work);
    }
    while (!pm_fs_sync_completed()) {
    if (pm_wakeup_pending()) {
    return -EBUSY;
    }
    wait_event_timeout(pm_fs_sync_wait, pm_fs_sync_completed(),
    PM_FS_SYNC_WAKEUP_RESOLUTION);
    }
    return 0;
    }

// Routines for PM-transition notifications
    static BLOCKING_NOTIFIER_HEAD(pm_chain_head);
#[no_mangle]
pub unsafe extern "C" fn register_pm_notifier(nb: *mut notifier_block) -> c_int {
    return blocking_notifier_chain_register(&pm_chain_head, nb);
    }
    EXPORT_SYMBOL_GPL(register_pm_notifier);
#[no_mangle]
pub unsafe extern "C" fn unregister_pm_notifier(nb: *mut notifier_block) -> c_int {
    return blocking_notifier_chain_unregister(&pm_chain_head, nb);
    }
    EXPORT_SYMBOL_GPL(unregister_pm_notifier);
#[no_mangle]
pub unsafe extern "C" fn pm_notifier_call_chain_robust(val_up: c_ulong, val_down: c_ulong) -> c_int {
    let mut ret = 0;
    ret = blocking_notifier_call_chain_robust(&pm_chain_head, val_up, val_down, core::ptr::null_mut());
    return notifier_to_errno(ret);
    }
#[no_mangle]
pub unsafe extern "C" fn pm_notifier_call_chain(val: c_ulong) -> c_int {
    return blocking_notifier_call_chain(&pm_chain_head, val, core::ptr::null_mut());
    }
// If set, devices may be suspended and resumed asynchronously.
pub static mut pm_async_enabled: c_int = 1;
#[no_mangle]
unsafe extern "C" fn pm_async_setup(str: *mut c_char) -> c_int {
    if (!strcmp(str, "off")) {
    pm_async_enabled = 0;
    }
    return 1;
    }
    __setup!("pm_async=", pm_async_setup);
#[no_mangle]
pub unsafe extern "C" fn pm_async_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%d\n", pm_async_enabled);
    }
#[no_mangle]
pub unsafe extern "C" fn pm_async_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
    let mut val = 0;
    if (kstrtoul(buf, 10, &val)) {
    return -EINVAL;
    }
    if (val > 1) {
    return -EINVAL;
    }
    pm_async_enabled = val;
    return n;
    }
    power_attr(pm_async);

#[no_mangle]
pub unsafe extern "C" fn mem_sleep_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
pub static mut count: isize = 0;
    let mut i;
    while (i < PM_SUSPEND_MAX) {
    if (i >= PM_SUSPEND_MEM && cxl_mem_active()) {
    continue;
    }
    if (mem_sleep_states[i]) {
    let mut label = mem_sleep_states[i];
    if (mem_sleep_current == i) {
    count += sysfs_emit_at(buf, count, "[%s] ", label);
    }
    else {
    count += sysfs_emit_at(buf, count, "%s ", label);
    }
    }
    }
// Convert the last space to a newline if needed.
    if (count > 0) {
    buf[count - 1] = '\n';
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn decode_suspend_state(buf: *const c_char, n: usize) -> suspend_state_t {
    let mut state;
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    p = memchr(buf, '\n', n);
    len = p ? p - buf : n;
    while (state < PM_SUSPEND_MAX) {
    let mut label = mem_sleep_states[state];
    if (label && len == strlen(label) && !strncmp(buf, label, len)) {
    return state;
    }
    }
    return PM_SUSPEND_ON;
    }
#[no_mangle]
pub unsafe extern "C" fn mem_sleep_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
    let mut state;
    let mut error = 0;
    error = pm_autosleep_lock();
    if (error) {
    return error;
    }
    if (pm_autosleep_state() > PM_SUSPEND_ON) {
    error = -EBUSY;
// goto;
    }
    state = decode_suspend_state(buf, n);
    if (state < PM_SUSPEND_MAX && state > PM_SUSPEND_ON) {
    mem_sleep_current = state;
    }
    else {
    error = -EINVAL;
    }
// label;
    pm_autosleep_unlock();
    return error ? error : n;
    }
    power_attr(mem_sleep);
//
// sync_on_suspend: Sync file systems before suspend.
//
// show() returns whether file systems sync before suspend is enabled.
// store() accepts 0 or 1.  0 disables file systems sync and 1 enables it.
//
pub static mut sync_on_suspend_enabled: bool = false;
#[no_mangle]
pub unsafe extern "C" fn sync_on_suspend_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%d\n", sync_on_suspend_enabled);
    }
#[no_mangle]
pub unsafe extern "C" fn sync_on_suspend_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
    let mut val = 0;
    if (kstrtoul(buf, 10, &val)) {
    return -EINVAL;
    }
    if (val > 1) {
    return -EINVAL;
    }
    sync_on_suspend_enabled = !!val;
    return n;
    }
    power_attr(sync_on_suspend);

pub static mut pm_test_level: c_int = 0;
    static const char * const pm_tests[__TEST_AFTER_LAST] = {
    [TEST_NONE] = "none",
    [TEST_CORE] = "core",
    [TEST_CPUS] = "processors",
    [TEST_PLATFORM] = "platform",
    [TEST_DEVICES] = "devices",
    [TEST_FREEZER] = "freezer",
    };
#[no_mangle]
pub unsafe extern "C" fn pm_test_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
pub static mut count: isize = 0;
    let mut level = 0;
    for (level = TEST_FIRST; level <= TEST_MAX; level++) {
    if (pm_tests[level]) {
    }
    if (level == pm_test_level) {
    count += sysfs_emit_at(buf, count, "[%s] ", pm_tests[level]);
    }
    else {
    count += sysfs_emit_at(buf, count, "%s ", pm_tests[level]);
    }
    }
// Convert the last space to a newline if needed.
    if (count > 0) {
    buf[count - 1] = '\n';
    }
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn pm_test_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
    let mut sleep_flags = 0;
    const char * const *s;
pub static mut error: c_int = 0;
    let mut level = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    p = memchr(buf, '\n', n);
    len = p ? p - buf : n;
    sleep_flags = lock_system_sleep();
    level = TEST_FIRST;
    for (s = &pm_tests[level]; level <= TEST_MAX; s++, level++) {
    if (*s && len == strlen(*s) && !strncmp(buf, *s, len)) {
    }
    pm_test_level = level;
    error = 0;
    break;
    }
    unlock_system_sleep(sleep_flags);
    return error ? error : n;
    }
    power_attr(pm_test);

pub const REC_FAILED_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct suspend_stats {
    pub step_failures: [c_uint; SUSPEND_NR_STEPS],
    pub success: c_uint,
    pub fail: c_uint,
    pub last_failed_dev: c_int,
    pub failed_devs: [c_char; REC_FAILED_NUM][40],
    pub last_failed_errno: c_int,
    pub errno: [c_int; REC_FAILED_NUM],
    pub last_failed_step: c_int,
    pub last_hw_sleep: u64,
    pub total_hw_sleep: u64,
    pub max_hw_sleep: u64,
    pub failed_steps: [enum suspend_stat_step; REC_FAILED_NUM],
}

pub static mut suspend_stats: usize = 0;
pub static mut suspend_stats_lock: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn dpm_save_failed_dev(name: *const c_char) {
    mutex_lock(&suspend_stats_lock);
    strscpy(suspend_stats.failed_devs[suspend_stats.last_failed_dev],
    name, sizeof!(suspend_stats.failed_devs[0]));
    suspend_stats.last_failed_dev += 1;
    suspend_stats.last_failed_dev %= REC_FAILED_NUM;
    mutex_unlock(&suspend_stats_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn dpm_save_failed_step(step: suspend_stat_step) {
    suspend_stats.step_failures[step-1]++;
    suspend_stats.failed_steps[suspend_stats.last_failed_step] = step;
    suspend_stats.last_failed_step += 1;
    suspend_stats.last_failed_step %= REC_FAILED_NUM;
    }
#[no_mangle]
pub unsafe extern "C" fn dpm_save_errno(err: c_int) {
    if (!err) {
    suspend_stats.success += 1;
    return;
    }
    suspend_stats.fail += 1;
    suspend_stats.errno[suspend_stats.last_failed_errno] = err;
    suspend_stats.last_failed_errno += 1;
    suspend_stats.last_failed_errno %= REC_FAILED_NUM;
    }
#[no_mangle]
pub unsafe extern "C" fn pm_report_hw_sleep_time(t: u64) {
    suspend_stats.last_hw_sleep = t;
    suspend_stats.total_hw_sleep += t;
    }
    EXPORT_SYMBOL_GPL(pm_report_hw_sleep_time);
#[no_mangle]
pub unsafe extern "C" fn pm_report_max_hw_sleep(t: u64) {
    suspend_stats.max_hw_sleep = t;
    }
    EXPORT_SYMBOL_GPL(pm_report_max_hw_sleep);
    static const char * const suspend_step_names[] = {
    [SUSPEND_WORKING] = "",
    [SUSPEND_FREEZE] = "freeze",
    [SUSPEND_PREPARE] = "prepare",
    [SUSPEND_SUSPEND] = "suspend",
    [SUSPEND_SUSPEND_LATE] = "suspend_late",
    [SUSPEND_SUSPEND_NOIRQ] = "suspend_noirq",
    [SUSPEND_RESUME_NOIRQ] = "resume_noirq",
    [SUSPEND_RESUME_EARLY] = "resume_early",
    [SUSPEND_RESUME] = "resume",
    };

    static ssize_t _name##_show(kobject *kobj, kobj_attribute *attr, char *buf)		
    {								
    return sysfs_emit(buf, format_str, suspend_stats._name);
    }								
    static struct kobj_attribute _name = __ATTR_RO(_name)
    suspend_attr(success, "%u\n");
    suspend_attr(fail, "%u\n");
    suspend_attr(last_hw_sleep, "%llu\n");
    suspend_attr(total_hw_sleep, "%llu\n");
    suspend_attr(max_hw_sleep, "%llu\n");

    static ssize_t _name##_show(kobject *kobj, kobj_attribute *attr, char *buf)		
    {								
    return sysfs_emit(buf, "%u\n",				
    suspend_stats.step_failures[step-1]);	
    }								
    static struct kobj_attribute _name = __ATTR_RO(_name)
    suspend_step_attr(failed_freeze, SUSPEND_FREEZE);
    suspend_step_attr(failed_prepare, SUSPEND_PREPARE);
    suspend_step_attr(failed_suspend, SUSPEND_SUSPEND);
    suspend_step_attr(failed_suspend_late, SUSPEND_SUSPEND_LATE);
    suspend_step_attr(failed_suspend_noirq, SUSPEND_SUSPEND_NOIRQ);
    suspend_step_attr(failed_resume, SUSPEND_RESUME);
    suspend_step_attr(failed_resume_early, SUSPEND_RESUME_EARLY);
    suspend_step_attr(failed_resume_noirq, SUSPEND_RESUME_NOIRQ);
#[no_mangle]
pub unsafe extern "C" fn last_failed_dev_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut index = 0;
    let mut last_failed_dev = core::ptr::null_mut();
    index = suspend_stats.last_failed_dev + REC_FAILED_NUM - 1;
    index %= REC_FAILED_NUM;
    last_failed_dev = suspend_stats.failed_devs[index];
    return sysfs_emit(buf, "%s\n", last_failed_dev);
    }
pub static mut last_failed_dev: kobj_attribute = 0;
#[no_mangle]
pub unsafe extern "C" fn last_failed_errno_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut index = 0;
    let mut last_failed_errno = 0;
    index = suspend_stats.last_failed_errno + REC_FAILED_NUM - 1;
    index %= REC_FAILED_NUM;
    last_failed_errno = suspend_stats.errno[index];
    return sysfs_emit(buf, "%d\n", last_failed_errno);
    }
pub static mut last_failed_errno: kobj_attribute = 0;
#[no_mangle]
pub unsafe extern "C" fn last_failed_step_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    enum suspend_stat_step step;
    let mut index = 0;
    index = suspend_stats.last_failed_step + REC_FAILED_NUM - 1;
    index %= REC_FAILED_NUM;
    step = suspend_stats.failed_steps[index];
    return sysfs_emit(buf, "%s\n", suspend_step_names[step]);
    }
pub static mut last_failed_step: kobj_attribute = 0;
    static struct attribute *suspend_attrs[] = {
    &success.attr,
    &fail.attr,
    &failed_freeze.attr,
    &failed_prepare.attr,
    &failed_suspend.attr,
    &failed_suspend_late.attr,
    &failed_suspend_noirq.attr,
    &failed_resume.attr,
    &failed_resume_early.attr,
    &failed_resume_noirq.attr,
    &last_failed_dev.attr,
    &last_failed_errno.attr,
    &last_failed_step.attr,
    &last_hw_sleep.attr,
    &total_hw_sleep.attr,
    &max_hw_sleep.attr,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn suspend_attr_is_visible(kobj: *mut kobject, attr: *mut attribute, idx: c_int) -> umode_t {
    if (attr != &last_hw_sleep.attr &&
    attr != &total_hw_sleep.attr &&
    attr != &max_hw_sleep.attr) {
    return 0444;
    }

    if (acpi_gbl_FADT.flags & ACPI_FADT_LOW_POWER_S0) {
    return 0444;
    }

    return 0;
    }
pub static mut attribute_group: usize = 0;

#[no_mangle]
unsafe extern "C" fn suspend_stats_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    let mut i = 0;
    let mut index = 0;
    let mut last_dev = 0;
    let mut last_errno = 0;
    let mut last_step = 0;
    enum suspend_stat_step step;
    last_dev = suspend_stats.last_failed_dev + REC_FAILED_NUM - 1;
    last_dev %= REC_FAILED_NUM;
    last_errno = suspend_stats.last_failed_errno + REC_FAILED_NUM - 1;
    last_errno %= REC_FAILED_NUM;
    last_step = suspend_stats.last_failed_step + REC_FAILED_NUM - 1;
    last_step %= REC_FAILED_NUM;
    seq_printf(s, "success: %u\nfail: %u\n",
    suspend_stats.success, suspend_stats.fail);
    for (step = SUSPEND_FREEZE; step <= SUSPEND_NR_STEPS; step++) {
    seq_printf(s, "failed_%s: %u\n", suspend_step_names[step],
    suspend_stats.step_failures[step-1]);
    }
    seq_printf(s,	"failures:\n  last_failed_dev:\t%-s\n",
    suspend_stats.failed_devs[last_dev]);
    while (i < REC_FAILED_NUM) {
    index = last_dev + REC_FAILED_NUM - i;
    index %= REC_FAILED_NUM;
    seq_printf(s, "\t\t\t%-s\n", suspend_stats.failed_devs[index]);
    }
    seq_printf(s,	"  last_failed_errno:\t%-d\n",
    suspend_stats.errno[last_errno]);
    while (i < REC_FAILED_NUM) {
    index = last_errno + REC_FAILED_NUM - i;
    index %= REC_FAILED_NUM;
    seq_printf(s, "\t\t\t%-d\n", suspend_stats.errno[index]);
    }
    seq_printf(s,	"  last_failed_step:\t%-s\n",
    suspend_step_names[suspend_stats.failed_steps[last_step]]);
    while (i < REC_FAILED_NUM) {
    index = last_step + REC_FAILED_NUM - i;
    index %= REC_FAILED_NUM;
    seq_printf(s, "\t\t\t%-s\n",
    suspend_step_names[suspend_stats.failed_steps[index]]);
    }
    return 0;
    }
pub static mut suspend_stats: usize = 0;
#[no_mangle]
unsafe extern "C" fn pm_debugfs_init() -> c_int {
    debugfs_create_file("suspend_stats", S_IFREG | S_IRUGO,
    core::ptr::null_mut(), core::ptr::null_mut(), &suspend_stats_fops);
    return 0;
    }
    late_initcall!(pm_debugfs_init);

#[no_mangle]
pub unsafe extern "C" fn pm_sleep_transition_in_progress() -> bool {
    return pm_suspend_in_progress() || hibernation_in_progress();
    }

//
// pm_print_times: print time taken by devices to suspend and resume.
//
// show() returns whether printing of suspend and resume times is enabled.
// store() accepts 0 or 1.  0 disables printing and 1 enables it.
//
    let mut pm_print_times_enabled = 0;
#[no_mangle]
pub unsafe extern "C" fn pm_print_times_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%d\n", pm_print_times_enabled);
    }
#[no_mangle]
pub unsafe extern "C" fn pm_print_times_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
    let mut val = 0;
    if (kstrtoul(buf, 10, &val)) {
    return -EINVAL;
    }
    if (val > 1) {
    return -EINVAL;
    }
    pm_print_times_enabled = !!val;
    return n;
    }
    power_attr(pm_print_times);
#[no_mangle]
pub unsafe extern "C" fn pm_print_times_init() {
    pm_print_times_enabled = initcall_debug;
    }
#[no_mangle]
pub unsafe extern "C" fn pm_wakeup_irq_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    if (!pm_wakeup_irq()) {
    return -ENODATA;
    }
    return sysfs_emit(buf, "%u\n", pm_wakeup_irq());
    }
    power_attr_ro(pm_wakeup_irq);
    let mut pm_debug_messages_on = 0;
#[no_mangle]
pub unsafe extern "C" fn pm_debug_messages_should_print() -> bool {
    return pm_debug_messages_on && pm_sleep_transition_in_progress();
    }
    EXPORT_SYMBOL_GPL(pm_debug_messages_should_print);
#[no_mangle]
pub unsafe extern "C" fn pm_debug_messages_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%d\n", pm_debug_messages_on);
    }
#[no_mangle]
pub unsafe extern "C" fn pm_debug_messages_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
    let mut val = 0;
    if (kstrtoul(buf, 10, &val)) {
    return -EINVAL;
    }
    if (val > 1) {
    return -EINVAL;
    }
    pm_debug_messages_on = !!val;
    return n;
    }
    power_attr(pm_debug_messages);
#[no_mangle]
unsafe extern "C" fn pm_debug_messages_setup(str: *mut c_char) -> c_int {
    pm_debug_messages_on = true;
    return 1;
    }
    __setup!("pm_debug_messages", pm_debug_messages_setup);

#[no_mangle]
#[no_mangle]
// duplicate fn: pm_print_times_init
pub unsafe extern "C" fn pm_print_times_init_dup() {}

pub static mut power_kobj: *mut c_void = core::ptr::null_mut();
//
// state - control system sleep states.
//
// show() returns available sleep state labels, which may be "mem", "standby",
// "freeze" and "disk" (hibernation).
// See Documentation/admin-guide/pm/sleep-states.rst for a description of
// what they mean.
//
// store() accepts one of those strings, translates it into the proper
// enumerated value, and initiates a suspend transition.
//
#[no_mangle]
pub unsafe extern "C" fn state_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
pub static mut count: isize = 0;

    let mut i;
    for (i = PM_SUSPEND_MIN; i < PM_SUSPEND_MAX; i++) {
    if (pm_states[i])
    count += sysfs_emit_at(buf, count, "%s ", pm_states[i]);
    }

    if (hibernation_available()) {
    count += sysfs_emit_at(buf, count, "disk ");
    }
// Convert the last space to a newline if needed.
    if (count > 0) {
    buf[count - 1] = '\n';
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn decode_state(buf: *const c_char, n: usize) -> suspend_state_t {

    let mut state;

pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    p = memchr(buf, '\n', n);
    len = p ? p - buf : n;
// Check hibernation first.
    if (len == 4 && str_has_prefix(buf, "disk")) {
    return PM_SUSPEND_MAX;
    }

    while (state < PM_SUSPEND_MAX) {
    let mut label = pm_states[state];
    if (label && len == strlen(label) && !strncmp(buf, label, len)) {
    return state;
    }
    }

    return PM_SUSPEND_ON;
    }
#[no_mangle]
pub unsafe extern "C" fn state_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
    let mut state;
    let mut error = 0;
    error = pm_autosleep_lock();
    if (error) {
    return error;
    }
    if (pm_autosleep_state() > PM_SUSPEND_ON) {
    error = -EBUSY;
// goto;
    }
    state = decode_state(buf, n);
    if (state < PM_SUSPEND_MAX) {
    if (state == PM_SUSPEND_MEM) {
    state = mem_sleep_current;
    }
    error = pm_suspend(state);
    } else if (state == PM_SUSPEND_MAX) {
    error = hibernate();
    } else {
    error = -EINVAL;
    }
// label;
    pm_autosleep_unlock();
    return error ? error : n;
    }
    power_attr(state);

//
// The 'wakeup_count' attribute, along with the functions defined in
// drivers/base/power/wakeup.c, provides a means by which wakeup events can be
// handled in a non-racy way.
//
// If a wakeup event occurs when the system is in a sleep state, it simply is
// woken up.  In turn, if an event that would wake the system up from a sleep
// state occurs when it is undergoing a transition to that sleep state, the
// transition should be aborted.  Moreover, if such an event occurs when the
// system is in the working state, an attempt to start a transition to the
// given sleep state should fail during certain period after the detection of
// the event.  Using the 'state' attribute alone is not sufficient to satisfy
// these requirements, because a wakeup event may occur exactly when 'state'
// is being written to and may be delivered to user space right before it is
// frozen, so the event will remain only partially processed until the system is
// woken up by another event.  In particular, it won't cause the transition to
// a sleep state to be aborted.
//
// This difficulty may be overcome if user space uses 'wakeup_count' before
// writing to 'state'.  It first should read from 'wakeup_count' and store
// the read value.  Then, after carrying out its own preparations for the system
// transition to a sleep state, it should write the stored value to
// 'wakeup_count'.  If that fails, at least one wakeup event has occurred since
// 'wakeup_count' was read and 'state' should not be written to.  Otherwise, it
// is allowed to write to 'state', but the transition will be aborted if there
// are any wakeup events detected after 'wakeup_count' was written to.
//
#[no_mangle]
pub unsafe extern "C" fn wakeup_count_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    let mut val = 0;
    return pm_get_wakeup_count(&val, true) ?
    sysfs_emit(buf, "%u\n", val) : -EINTR;
    }
#[no_mangle]
pub unsafe extern "C" fn wakeup_count_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
    let mut val = 0;
    let mut error = 0;
    error = pm_autosleep_lock();
    if (error) {
    return error;
    }
    if (pm_autosleep_state() > PM_SUSPEND_ON) {
    error = -EBUSY;
// goto;
    }
    error = -EINVAL;
    if (sscanf(buf, "%u", &val) == 1) {
    if (pm_save_wakeup_count(val)) {
    error = n;
    }
    else {
    pm_print_active_wakeup_sources();
    }
    }
// label;
    pm_autosleep_unlock();
    return error;
    }
    power_attr(wakeup_count);

#[no_mangle]
pub unsafe extern "C" fn autosleep_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
pub static mut state: suspend_state_t = 0;
    if (state == PM_SUSPEND_ON) {
    return sysfs_emit(buf, "off\n");
    }

    if (state < PM_SUSPEND_MAX) {
    return sysfs_emit(buf, "%s\n", pm_states[state] ?
    pm_states[state] : "error");
    }

    return sysfs_emit(buf, "disk\n");

    return sysfs_emit(buf, "error\n");

    }
#[no_mangle]
pub unsafe extern "C" fn autosleep_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
pub static mut state: suspend_state_t = 0;
    let mut error = 0;
    if (state == PM_SUSPEND_ON
    && strcmp(buf, "off") && strcmp(buf, "off\n")) {
    return -EINVAL;
    }
    if (state == PM_SUSPEND_MEM) {
    state = mem_sleep_current;
    }
    error = pm_autosleep_set_state(state);
    return error ? error : n;
    }
    power_attr(autosleep);

#[no_mangle]
pub unsafe extern "C" fn wake_lock_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return pm_show_wakelocks(buf, true);
    }
#[no_mangle]
pub unsafe extern "C" fn wake_lock_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
pub static mut error: c_int = 0;
    return error ? error : n;
    }
    power_attr(wake_lock);
#[no_mangle]
pub unsafe extern "C" fn wake_unlock_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return pm_show_wakelocks(buf, false);
    }
#[no_mangle]
pub unsafe extern "C" fn wake_unlock_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
pub static mut error: c_int = 0;
    return error ? error : n;
    }
    power_attr(wake_unlock);

    let mut pm_trace_enabled = 0;
#[no_mangle]
pub unsafe extern "C" fn pm_trace_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%d\n", pm_trace_enabled);
    }
#[no_mangle]
pub unsafe extern "C" fn pm_trace_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
    let mut val = 0;
    if (sscanf(buf, "%d", &val) == 1) {
    pm_trace_enabled = !!val;
    if (pm_trace_enabled) {
    pr_warn!("PM: Enabling pm_trace changes system date and time during resume.\n"
    "PM: Correct system time has to be restored manually after resume.\n");
    }
    return n;
    }
    return -EINVAL;
    }
    power_attr(pm_trace);
#[no_mangle]
pub unsafe extern "C" fn pm_trace_dev_match_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return show_trace_dev_match(buf, PAGE_SIZE);
    }
    power_attr_ro(pm_trace_dev_match);

#[no_mangle]
pub unsafe extern "C" fn pm_freeze_timeout_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%u\n", freeze_timeout_msecs);
    }
#[no_mangle]
pub unsafe extern "C" fn pm_freeze_timeout_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
    let mut val = 0;
    if (kstrtoul(buf, 10, &val)) {
    return -EINVAL;
    }
    freeze_timeout_msecs = val;
    return n;
    }
    power_attr(pm_freeze_timeout);

pub static mut filesystem_freeze_enabled: bool = false;
#[no_mangle]
pub unsafe extern "C" fn freeze_filesystems_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%d\n", filesystem_freeze_enabled);
    }
#[no_mangle]
pub unsafe extern "C" fn freeze_filesystems_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, n: size_t) -> ssize_t {
    let mut val = 0;
    if (kstrtoul(buf, 10, &val)) {
    return -EINVAL;
    }
    if (val > 1) {
    return -EINVAL;
    }
    filesystem_freeze_enabled = !!val;
    return n;
    }
    power_attr(freeze_filesystems);

    static struct attribute * g[] = {
    &state_attr.attr,

    &pm_trace_attr.attr,
    &pm_trace_dev_match_attr.attr,

    &pm_async_attr.attr,
    &wakeup_count_attr.attr,

    &mem_sleep_attr.attr,
    &sync_on_suspend_attr.attr,

    &autosleep_attr.attr,

    &wake_lock_attr.attr,
    &wake_unlock_attr.attr,

    &pm_test_attr.attr,
    &pm_print_times_attr.attr,
    &pm_wakeup_irq_attr.attr,
    &pm_debug_messages_attr.attr,

    &pm_freeze_timeout_attr.attr,

    &freeze_filesystems_attr.attr,

    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
    static const struct attribute_group *attr_groups[] = {
    &attr_group,

    &suspend_attr_group,

    core::ptr::null_mut(),
    };
pub static mut pm_wq: *mut c_void = core::ptr::null_mut();
    EXPORT_SYMBOL_GPL(pm_wq);
#[no_mangle]
unsafe extern "C" fn pm_start_workqueues() -> c_int {
    pm_wq = alloc_workqueue("pm", WQ_UNBOUND, 0);
    if (!pm_wq) {
    return -ENOMEM;
    }

    pm_fs_sync_wq = alloc_ordered_workqueue("pm_fs_sync", 0);
    if (!pm_fs_sync_wq) {
    destroy_workqueue(pm_wq);
    return -ENOMEM;
    }

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm_init() -> c_int {
pub static mut error: c_int = 0;
    if (error) {
    return error;
    }
    hibernate_image_size_init();
    hibernate_reserved_size_init();
    pm_states_init();
    power_kobj = kobject_create_and_add("power", core::ptr::null_mut());
    if (!power_kobj) {
    return -ENOMEM;
    }
    error = sysfs_create_groups(power_kobj, attr_groups);
    if (error) {
    return error;
    }
    pm_print_times_init();
    return pm_autosleep_init();
    }
    core_initcall!(pm_init);
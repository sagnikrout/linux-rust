//! Automatically rewritten from C to Rust
//! Source: kernel/kcsan/debugfs.c
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
// KCSAN debugfs interface.
//
// Copyright (C) 2019, Google LLC.
//

    atomic_long_t kcsan_counters[KCSAN_COUNTER_COUNT];
    static const char *const counter_names[] = {
    [KCSAN_COUNTER_USED_WATCHPOINTS]		= "used_watchpoints",
    [KCSAN_COUNTER_SETUP_WATCHPOINTS]		= "setup_watchpoints",
    [KCSAN_COUNTER_DATA_RACES]			= "data_races",
    [KCSAN_COUNTER_ASSERT_FAILURES]			= "assert_failures",
    [KCSAN_COUNTER_NO_CAPACITY]			= "no_capacity",
    [KCSAN_COUNTER_REPORT_RACES]			= "report_races",
    [KCSAN_COUNTER_RACES_UNKNOWN_ORIGIN]		= "races_unknown_origin",
    [KCSAN_COUNTER_UNENCODABLE_ACCESSES]		= "unencodable_accesses",
    [KCSAN_COUNTER_ENCODING_FALSE_POSITIVES]	= "encoding_false_positives",
    };
    static_assert(ARRAY_SIZE!(counter_names) == KCSAN_COUNTER_COUNT);
//
// Addresses for filtering functions from reporting. This list can be used as a
// whitelist or blacklist.
//
    static struct {
pub static mut addrs: *mut c_void = core::ptr::null_mut();		/* array of addresses */
    let mut size = 0;		/* current size */
    let mut used = 0;		/* number of elements used */
    let mut sorted = 0;		/* if elements are sorted */
    let mut whitelist = 0;	/* if list is a blacklist or whitelist */
    } report_filterlist;
pub static mut report_filterlist_lock: usize = 0;
//
// The microbenchmark allows benchmarking KCSAN core runtime only. To run
// multiple threads, pipe 'microbench=<iters>' from multiple tasks into the
// debugfs file. This will not generate any conflicts, and tests fast-path only.
//
#[no_mangle]
unsafe extern "C" fn microbenchmark(iters: c_ulong) -> noinline void {
pub static mut ctx_save: kcsan_ctx = 0;
pub static mut was_enabled: bool = false;
    let mut cycles = 0;
// We may have been called from an atomic region; reset context.
    memset(&current.kcsan_ctx, 0, sizeof!(current.kcsan_ctx));
//
// Disable to benchmark fast-path for all accesses, and (expected
// negligible) call into slow-path, but never set up watchpoints.
//
    WRITE_ONCE(kcsan_enabled, false);
    pr_info!("%s begin | iters: %lu\n", __func__, iters);
    cycles = get_cycles();
    while (iters--) {
pub static mut addr: c_ulong = 0;
    let mut type = !(iters & 0x7f) ? KCSAN_ACCESS_ATOMIC :
    (!(iters & 0xf) ? KCSAN_ACCESS_WRITE : 0);
    __kcsan_check_access(addr, sizeof!(long), type);
    }
    cycles = get_cycles() - cycles;
    pr_info!("%s end   | cycles: %llu\n", __func__, cycles);
    WRITE_ONCE(kcsan_enabled, was_enabled);
// restore context
    current.kcsan_ctx = ctx_save;
    }
#[no_mangle]
unsafe extern "C" fn cmp_filterlist_addrs(rhs: *const c_void, lhs: *const c_void) -> c_int {
pub static mut a: c_ulong = 0;
pub static mut b: c_ulong = 0;
    return a < b ? -1 : a == b ? 0 : 1;
    }
#[no_mangle]
pub unsafe extern "C" fn kcsan_skip_report_debugfs(func_addr: c_ulong) -> bool {
    unsigned long symbolsize, offset;
    let mut flags = 0;
pub static mut ret: bool = false;
    if (!kallsyms_lookup_size_offset(func_addr, &symbolsize, &offset)) {
    return false;
    }
    func_addr -= offset; /* Get function start */
    raw_spin_lock_irqsave(&report_filterlist_lock, flags);
    if (report_filterlist.used == 0) {
// goto;
    }
// Sort array if it is unsorted, and then do a binary search.
    if (!report_filterlist.sorted) {
    sort(report_filterlist.addrs, report_filterlist.used,
    sizeof!(unsigned long), cmp_filterlist_addrs, core::ptr::null_mut());
    report_filterlist.sorted = true;
    }
    ret = !!bsearch(&func_addr, report_filterlist.addrs,
    report_filterlist.used, sizeof!(unsigned long),
    cmp_filterlist_addrs);
    if (report_filterlist.whitelist) {
    ret = !ret;
    }
// label;
    raw_spin_unlock_irqrestore(&report_filterlist_lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn set_report_filterlist_whitelist(whitelist: bool) {
    let mut flags = 0;
    raw_spin_lock_irqsave(&report_filterlist_lock, flags);
    report_filterlist.whitelist = whitelist;
    raw_spin_unlock_irqrestore(&report_filterlist_lock, flags);
    }
// Returns 0 on success, error-code otherwise.
#[no_mangle]
unsafe extern "C" fn insert_report_filterlist(func: *const c_char) -> isize {
    let mut flags = 0;
pub static mut addr: c_ulong = 0;
    let mut delay_free = core::ptr::null_mut();
    let mut new_addrs = core::ptr::null_mut();
pub static mut new_size: usize = 0;
pub static mut ret: isize = 0;
    if (!addr) {
    pr_err!("could not find function: '%s'\n", func);
    return -ENOENT;
    }
// label;
//
// Check if we need an allocation, and re-validate under the lock. Since
// the report_filterlist_lock is a raw, cannot allocate under the lock.
//
    if (data_race(report_filterlist.used == report_filterlist.size)) {
    new_size = (report_filterlist.size ?: 4) * 2;
    delay_free = new_addrs = kmalloc_array(new_size, sizeof!(unsigned long), GFP_KERNEL);
    if (!new_addrs) {
    return -ENOMEM;
    }
    }
    raw_spin_lock_irqsave(&report_filterlist_lock, flags);
    if (report_filterlist.used == report_filterlist.size) {
// Check we pre-allocated enough, and retry if not.
    if (report_filterlist.used >= new_size) {
    raw_spin_unlock_irqrestore(&report_filterlist_lock, flags);
    kfree(new_addrs); /* kfree(core::ptr::null_mut()) is safe */
    delay_free = new_addrs = core::ptr::null_mut();
// goto;
    }
    if (report_filterlist.used) {
    memcpy(new_addrs, report_filterlist.addrs, report_filterlist.used * sizeof!(unsigned long));
    }
    delay_free = report_filterlist.addrs; /* free the old list */
    report_filterlist.addrs = new_addrs;  /* switch to the new list */
    report_filterlist.size = new_size;
    }
// Note: deduplicating should be done in userspace.
    report_filterlist.addrs[report_filterlist.used++] = addr;
    report_filterlist.sorted = false;
    raw_spin_unlock_irqrestore(&report_filterlist_lock, flags);
    kfree(delay_free);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn show_info(file: *mut seq_file, v: *mut c_void) -> c_int {
    let mut i = 0;
    let mut flags = 0;
// show stats
    seq_printf(file, "enabled: %i\n", READ_ONCE(kcsan_enabled));
    while (i < KCSAN_COUNTER_COUNT) {
    seq_printf(file, "%s: %ld\n", counter_names[i],
    atomic_long_read(&kcsan_counters[i]));
    }
// show filter functions, and filter type
    raw_spin_lock_irqsave(&report_filterlist_lock, flags);
    seq_printf(file, "\n%s functions: %s\n",
    report_filterlist.whitelist ? "whitelisted" : "blacklisted",
    report_filterlist.used == 0 ? "none" : "");
    for (i = 0; i < report_filterlist.used; ++i) {
    seq_printf(file, " %ps\n", report_filterlist.addrs[i]);
    }
    raw_spin_unlock_irqrestore(&report_filterlist_lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn debugfs_open(inode: *mut inode, file: *mut file) -> c_int {
    return single_open(file, show_info, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn debugfs_write(file: *mut file, buf: *mut c_char, count: size_t, off: *mut loff_t) -> ssize_t {
    char kbuf[KSYM_NAME_LEN];
pub static mut arg: *mut c_void = core::ptr::null_mut();
pub static mut read_len: usize = 0;
    if (copy_from_user(kbuf, buf, read_len)) {
    return -EFAULT;
    }
    kbuf[read_len] = '\0';
    arg = strstrip(kbuf);
    if (!strcmp(arg, "on")) {
    WRITE_ONCE(kcsan_enabled, true);
    } else if (!strcmp(arg, "off")) {
    WRITE_ONCE(kcsan_enabled, false);
    } else if (str_has_prefix(arg, "microbench=")) {
    let mut iters = 0;
    if (kstrtoul(&arg[strlen("microbench=")], 0, &iters)) {
    return -EINVAL;
    }
    microbenchmark(iters);
    } else if (!strcmp(arg, "whitelist")) {
    set_report_filterlist_whitelist(true);
    } else if (!strcmp(arg, "blacklist")) {
    set_report_filterlist_whitelist(false);
    } else if (arg[0] == '!') {
pub static mut ret: isize = 0;
    if (ret < 0) {
    return ret;
    }
    } else {
    return -EINVAL;
    }
    return count;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn kcsan_debugfs_init() -> c_int {
    debugfs_create_file("kcsan", 0644, core::ptr::null_mut(), core::ptr::null_mut(), &debugfs_ops);
    return 0;
    }
    late_initcall!(kcsan_debugfs_init);
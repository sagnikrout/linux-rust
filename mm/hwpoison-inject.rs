//! Automatically rewritten from C to Rust
//! Source: mm/hwpoison-inject.c
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
// Inject a hwpoison memory failure on a arbitrary pfn

    static u32 hwpoison_filter_enable;
pub static mut hwpoison_filter_dev_major: u32 = 0;
pub static mut hwpoison_filter_dev_minor: u32 = 0;
    static u64 hwpoison_filter_flags_mask;
    static u64 hwpoison_filter_flags_value;
#[no_mangle]
unsafe extern "C" fn hwpoison_filter_dev(p: *mut page) -> c_int {
    let mut folio = page_folio(p);
pub static mut mapping: *mut c_void = core::ptr::null_mut();
    let mut dev;
    if (hwpoison_filter_dev_major == ~0U &&
    hwpoison_filter_dev_minor == ~0U) {
    return 0;
    }
    mapping = folio_mapping(folio);
    if (mapping == core::ptr::null_mut() || mapping.host == core::ptr::null_mut()) {
    return -EINVAL;
    }
    dev = mapping.host.i_sb.s_dev;
    if (hwpoison_filter_dev_major != ~0U &&
    hwpoison_filter_dev_major != MAJOR(dev)) {
    return -EINVAL;
    }
    if (hwpoison_filter_dev_minor != ~0U &&
    hwpoison_filter_dev_minor != MINOR(dev)) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hwpoison_filter_flags(p: *mut page) -> c_int {
    if (!hwpoison_filter_flags_mask) {
    return 0;
    }
    if ((stable_page_flags(p) & hwpoison_filter_flags_mask) ==
    hwpoison_filter_flags_value) {
    return 0;
    }
    else {
    return -EINVAL;
    }
    }
//
// This allows stress tests to limit test scope to a collection of tasks
// by putting them under some memcg. This prevents killing unrelated/important
// processes such as /sbin/init. Note that the target task may share clean
// pages with init (eg. libc text), which is harmless. If the target task
// share _dirty_ pages with another task B, the test scheme must make sure B
// is also included in the memcg. At last, due to race conditions this filter
// can only guarantee that the page either belongs to the memcg tasks, or is
// a freed page.
//

    static u64 hwpoison_filter_memcg;
#[no_mangle]
unsafe extern "C" fn hwpoison_filter_task(p: *mut page) -> c_int {
    if (!hwpoison_filter_memcg) {
    return 0;
    }
    if (page_cgroup_ino(p) != hwpoison_filter_memcg) {
    return -EINVAL;
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn hwpoison_filter_task(p: *mut page) -> c_int { return 0; }

#[no_mangle]
unsafe extern "C" fn hwpoison_filter(p: *mut page) -> c_int {
    if (!hwpoison_filter_enable) {
    return 0;
    }
    if (hwpoison_filter_dev(p)) {
    return -EINVAL;
    }
    if (hwpoison_filter_flags(p)) {
    return -EINVAL;
    }
    if (hwpoison_filter_task(p)) {
    return -EINVAL;
    }
    return 0;
    }
pub static mut hwpoison_dir: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn hwpoison_inject(data: *mut c_void, val: u64) -> c_int {
pub static mut pfn: c_ulong = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (!capable(CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    if (!pfn_valid(pfn)) {
    return -ENXIO;
    }
    p = pfn_to_page(pfn);
    folio = page_folio(p);
    if (!hwpoison_filter_enable) {
// goto;
    }
    shake_folio(folio);
//
// This implies unable to support non-LRU pages except free page.
//
    if (!folio_test_lru(folio) && !folio_test_hugetlb(folio) &&
    !is_free_buddy_page(p)) {
    return 0;
    }
//
// do a racy check to make sure PG_hwpoison will only be set for
// the targeted owner (or on a free page).
// memory_failure() will redo the check reliably inside page lock.
//
    err = hwpoison_filter(&folio.page);
    if (err) {
    return 0;
    }
// label;
    pr_info!("Injecting memory failure at pfn %#lx\n", pfn);
    err = memory_failure(pfn, MF_SW_SIMULATED);
    return (err == -EOPNOTSUPP) ? 0 : err;
    }
#[no_mangle]
unsafe extern "C" fn hwpoison_unpoison(data: *mut c_void, val: u64) -> c_int {
    if (!capable(CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    return unpoison_memory(val);
    }
    DEFINE_DEBUGFS_ATTRIBUTE(hwpoison_fops, core::ptr::null_mut(), hwpoison_inject, "%lli\n");
    DEFINE_DEBUGFS_ATTRIBUTE(unpoison_fops, core::ptr::null_mut(), hwpoison_unpoison, "%lli\n");
#[no_mangle]
unsafe extern "C" fn pfn_inject_exit()  {
    hwpoison_filter_enable = 0;
    hwpoison_filter_unregister();
    debugfs_remove_recursive(hwpoison_dir);
    }
#[no_mangle]
unsafe extern "C" fn pfn_inject_init() -> c_int {
    hwpoison_dir = debugfs_create_dir("hwpoison", core::ptr::null_mut());
//
// Note that the below poison/unpoison interfaces do not involve
// hardware status change, hence do not require hardware support.
// They are mainly for testing hwpoison in software level.
//
    debugfs_create_file("corrupt-pfn", 0200, hwpoison_dir, core::ptr::null_mut(),
    &hwpoison_fops);
    debugfs_create_file("unpoison-pfn", 0200, hwpoison_dir, core::ptr::null_mut(),
    &unpoison_fops);
    debugfs_create_u32("corrupt-filter-enable", 0600, hwpoison_dir,
    &hwpoison_filter_enable);
    debugfs_create_u32("corrupt-filter-dev-major", 0600, hwpoison_dir,
    &hwpoison_filter_dev_major);
    debugfs_create_u32("corrupt-filter-dev-minor", 0600, hwpoison_dir,
    &hwpoison_filter_dev_minor);
    debugfs_create_u64("corrupt-filter-flags-mask", 0600, hwpoison_dir,
    &hwpoison_filter_flags_mask);
    debugfs_create_u64("corrupt-filter-flags-value", 0600, hwpoison_dir,
    &hwpoison_filter_flags_value);

    debugfs_create_u64("corrupt-filter-memcg", 0600, hwpoison_dir,
    &hwpoison_filter_memcg);

    hwpoison_filter_register(hwpoison_filter);
    return 0;
    }
    module_init!(pfn_inject_init);
    module_exit!(pfn_inject_exit);
    MODULE_DESCRIPTION("HWPoison pages injector");
    MODULE_LICENSE("GPL");
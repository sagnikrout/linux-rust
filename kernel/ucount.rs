//! Automatically rewritten from C to Rust
//! Source: kernel/ucount.c
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

pub static mut ucounts: usize = 0;
pub const UCOUNTS_HASHTABLE_BITS: c_int = 10;

pub static mut hlist_nulls_head: usize = 0;
// static DEFINE_SPINLOCK(ucounts_lock);

    hash_long((unsigned long)__kuid_val(uid) + (unsigned long)(ns), 
    UCOUNTS_HASHTABLE_BITS)

    (ucounts_hashtable + ucounts_hashfn(ns, uid))

#[no_mangle]
pub unsafe extern "C" fn set_lookup(root: *mut ctl_table_root) -> *mut c_void {
    return &current_user_ns().set;
    }
#[no_mangle]
unsafe extern "C" fn set_is_seen(set: *mut ctl_table_set) -> c_int {
    return &current_user_ns().set == set;
    }
#[no_mangle]
pub unsafe extern "C" fn set_permissions(head: *mut ctl_table_header, table: *mut ctl_table) -> c_int {
    let mut user_ns = container_of!(head.set, user_namespace, set);
    let mut mode = 0;
// Allow users with CAP_SYS_RESOURCE unrestrained access
    if (ns_capable_noaudit(user_ns, CAP_SYS_RESOURCE)) {
    mode = (table.mode & S_IRWXU) >> 6;
    }
    else {
// Allow all others at most read-only access
    mode = table.mode & S_IROTH;
    }
    return (mode << 6) | (mode << 3) | mode;
    }
pub static mut ctl_table_root: usize = 0;
pub static mut ue_zero: long = 0;
pub static mut ue_int_max: long = 0;

    {							
    .procname	= name,				
    .maxlen		= sizeof!(long),			
    .mode		= 0644,				
    .proc_handler	= proc_doulongvec_minmax,	
    .extra1		= &ue_zero,			
    .extra2		= &ue_int_max,			
    }
pub static mut ctl_table: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn setup_userns_sysctls(ns: *mut user_namespace) -> bool {

pub static mut tbl: *mut c_void = core::ptr::null_mut();
    BUILD_BUG_ON!(ARRAY_SIZE!(user_table) != UCOUNT_COUNTS);
    setup_sysctl_set(&ns.set, &set_root, set_is_seen);
    tbl = kmemdup(user_table, sizeof!(user_table), GFP_KERNEL);
    if (tbl) {
    let mut i = 0;
    while (i < UCOUNT_COUNTS) {
    tbl[i].data = &ns.ucount_max[i];
    }
    ns.sysctls = __register_sysctl_table(&ns.set, "user", tbl,
    ARRAY_SIZE!(user_table));
    }
    if (!ns.sysctls) {
    kfree(tbl);
    retire_sysctl_set(&ns.set);
    return false;
    }

    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn retire_userns_sysctls(ns: *mut user_namespace) {

pub static mut tbl: *mut c_void = core::ptr::null_mut();
    tbl = ns.sysctls.ctl_table_arg;
    unregister_sysctl_table(ns.sysctls);
    retire_sysctl_set(&ns.set);
    kfree(tbl);

    }
#[no_mangle]
pub unsafe extern "C" fn find_ucounts(ns: *mut user_namespace, uid: kuid_t, hashent: *mut hlist_nulls_head) -> *mut c_void {
pub static mut ucounts: *mut c_void = core::ptr::null_mut();
pub static mut pos: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    hlist_nulls_for_each_entry_rcu(ucounts, pos, hashent, node) {
    if (uid_eq(ucounts.uid, uid) && (ucounts.ns == ns)) {
    if (rcuref_get(&ucounts.count)) {
    return ucounts;
    }
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn hlist_add_ucounts(ucounts: *mut ucounts) {
    let mut hashent = ucounts_hashentry(ucounts.ns, ucounts.uid);
    spin_lock_irq(&ucounts_lock);
    hlist_nulls_add_head_rcu(&ucounts.node, hashent);
    spin_unlock_irq(&ucounts_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_ucounts(ns: *mut user_namespace, uid: kuid_t) -> *mut c_void {
    let mut hashent = ucounts_hashentry(ns, uid);
    let mut ucounts = core::ptr::null_mut();
    let mut new = core::ptr::null_mut();
    ucounts = find_ucounts(ns, uid, hashent);
    if (ucounts) {
    return ucounts;
    }
    new = kzalloc_obj(*new);
    if (!new) {
    return core::ptr::null_mut();
    }
    new.ns = ns;
    new.uid = uid;
    rcuref_init(&new.count, 1);
    spin_lock_irq(&ucounts_lock);
    ucounts = find_ucounts(ns, uid, hashent);
    if (ucounts) {
    spin_unlock_irq(&ucounts_lock);
    kfree(new);
    return ucounts;
    }
    hlist_nulls_add_head_rcu(&new.node, hashent);
    get_user_ns(new.ns);
    spin_unlock_irq(&ucounts_lock);
    return new;
    }
#[no_mangle]
pub unsafe extern "C" fn put_ucounts(ucounts: *mut ucounts) {
    let mut flags = 0;
    if (rcuref_put(&ucounts.count)) {
    spin_lock_irqsave(&ucounts_lock, flags);
    hlist_nulls_del_rcu(&ucounts.node);
    spin_unlock_irqrestore(&ucounts_lock, flags);
    put_user_ns(ucounts.ns);
    kfree_rcu(ucounts, rcu);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn atomic_long_inc_below(v: *mut atomic_long_t, u: c_long) -> bool {
pub static mut c: c_long = 0;
    do {
    if (unlikely(c >= u)) {
    return false;
    }
    } while (!atomic_long_try_cmpxchg(v, &c, c+1));
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn inc_ucount(ns: *mut user_namespace, uid: kuid_t, type: ucount_type) -> *mut c_void {
    let mut ucounts = core::ptr::null_mut();
    let mut iter = core::ptr::null_mut();
    let mut bad = core::ptr::null_mut();
pub static mut tns: *mut c_void = core::ptr::null_mut();
    ucounts = alloc_ucounts(ns, uid);
    while (iter) {
    let mut max = 0;
    tns = iter.ns;
    max = READ_ONCE(tns.ucount_max[type]);
    if (!atomic_long_inc_below(&iter.ucount[type], max)) {
// goto;
    }
    }
    return ucounts;
// label;
    bad = iter;
    for (iter = ucounts; iter != bad; iter = iter.ns.ucounts) {
    atomic_long_dec(&iter.ucount[type]);
    }
    put_ucounts(ucounts);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_FOR_MODULES(inc_ucount, "binfmt_misc");
#[no_mangle]
pub unsafe extern "C" fn dec_ucount(ucounts: *mut ucounts, type: ucount_type) {
pub static mut iter: *mut c_void = core::ptr::null_mut();
    while (iter) {
pub static mut dec: c_long = 0;
    WARN_ON_ONCE!(dec < 0);
    }
    put_ucounts(ucounts);
    }
    EXPORT_SYMBOL_FOR_MODULES(dec_ucount, "binfmt_misc");
#[no_mangle]
pub unsafe extern "C" fn inc_rlimit_ucounts(ucounts: *mut ucounts, type: rlimit_type, v: c_long) -> c_long {
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut max: c_long = 0;
pub static mut ret: c_long = 0;
    while (iter) {
pub static mut new: c_long = 0;
    if (new < 0 || new > max) {
    ret = LONG_MAX;
    }

    else if (iter == ucounts) {
    ret = new;
    }
    max = get_userns_rlimit_max(iter.ns, type);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dec_rlimit_ucounts(ucounts: *mut ucounts, type: rlimit_type, v: c_long) -> bool {
pub static mut iter: *mut c_void = core::ptr::null_mut();
    let mut new = -1; /* Silence compiler warning */
    while (iter) {
pub static mut dec: c_long = 0;
    WARN_ON_ONCE!(dec < 0);
    if (iter == ucounts) {
    new = dec;
    }
    }
    return (new == 0);
    }
#[no_mangle]
pub unsafe extern "C" fn do_dec_rlimit_put_ucounts(ucounts: *mut ucounts, last: *mut ucounts, type: rlimit_type) {
    let mut iter = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    while (iter != last) {
pub static mut dec: c_long = 0;
    WARN_ON_ONCE!(dec < 0);
    next = iter.ns.ucounts;
    if (dec == 0) {
    put_ucounts(iter);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dec_rlimit_put_ucounts(ucounts: *mut ucounts, type: rlimit_type) {
    do_dec_rlimit_put_ucounts(ucounts, core::ptr::null_mut(), type);
    }
#[no_mangle]
pub unsafe extern "C" fn inc_rlimit_get_ucounts(ucounts: *mut ucounts, type: rlimit_type, override_rlimit: bool) -> c_long {
// Caller must hold a reference to ucounts
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut max: c_long = 0;
    long dec, ret = 0;
    while (iter) {
pub static mut new: c_long = 0;
    if (new < 0 || new > max) {
// goto;
    }
    if (iter == ucounts) {
    ret = new;
    }
    if (!override_rlimit) {
    max = get_userns_rlimit_max(iter.ns, type);
    }
//
// Grab an extra ucount reference for the caller when
// the rlimit count was previously 0.
//
    if (new != 1) {
    continue;
    }
    if (!get_ucounts(iter)) {
// goto;
    }
    }
    return ret;
// label;
    dec = atomic_long_sub_return(1, &iter.rlimit[type]);
    WARN_ON_ONCE!(dec < 0);
    do_dec_rlimit_put_ucounts(ucounts, iter, type);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn is_rlimit_overlimit(ucounts: *mut ucounts, type: rlimit_type, rlimit: c_ulong) -> bool {
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut max: c_long = 0;
    if (rlimit > LONG_MAX) {
    max = LONG_MAX;
    }
    while (iter) {
pub static mut val: c_long = 0;
    if (val < 0 || val > max) {
    return true;
    }
    max = get_userns_rlimit_max(iter.ns, type);
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn user_namespace_sysctl_init() -> __init int {

pub static mut user_header: *mut c_void = core::ptr::null_mut();
    static struct ctl_table empty[1];
//
// It is necessary to register the user directory in the
// default set so that registrations in the child sets work
// properly.
//
    user_header = register_sysctl_sz("user", empty, 0);
    kmemleak_ignore(user_header);
    BUG_ON!(!user_header);
    BUG_ON!(!setup_userns_sysctls(&init_user_ns));

    hlist_add_ucounts(&init_ucounts);
    inc_rlimit_ucounts(&init_ucounts, UCOUNT_RLIMIT_NPROC, 1);
    return 0;
    }
    subsys_initcall!(user_namespace_sysctl_init);
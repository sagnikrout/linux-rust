//! Automatically rewritten from C to Rust
//! Source: kernel/user.c
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
// The "user cache".
//
// (C) Copyright 1991-2000 Linus Torvalds
//
// We have a per-user structure to keep track of how many
// processes, files etc the user has claimed, in order to be
// able to have per-user limits for system resources.
//

pub static mut binfmt_misc: usize = 0;
    EXPORT_SYMBOL_GPL(init_binfmt_misc);

//
// userns count is 1 for root user, 1 for init_uts_ns,
// and 1 for... ?
//
pub static mut user_namespace: usize = 0;
    EXPORT_SYMBOL_GPL(init_user_ns);
//
// UID task count cache, to get fast user lookup in "alloc_uid"
// when changing user ID's (ie setuid() and friends).
//

pub static mut uid_cachep: *mut c_void = core::ptr::null_mut();
    static struct hlist_head uidhash_table[UIDHASH_SZ];
//
// The uidhash_lock is mostly taken from process context, but it is
// occasionally also taken from softirq/tasklet context, when
// task-structs get RCU-freed. Hence all locking must be softirq-safe.
// But free_uid() is also called with local interrupts disabled, and running
// local_bh_enable() with local interrupts disabled is an error - we'll run
// softirq callbacks, and they can unconditionally enable interrupts, and
// the caller of free_uid() didn't expect that..
//
// static DEFINE_SPINLOCK(uidhash_lock);
// root_user.__count is 1, for init task cred
pub static mut user_struct: usize = 0;
//
// These routines must be called with the uidhash spinlock held!
//
#[no_mangle]
unsafe extern "C" fn uid_hash_insert(up: *mut user_struct, hashent: *mut hlist_head) {
    hlist_add_head(&up.uidhash_node, hashent);
    }
#[no_mangle]
unsafe extern "C" fn uid_hash_remove(up: *mut user_struct) {
    hlist_del_init(&up.uidhash_node);
    }
#[no_mangle]
pub unsafe extern "C" fn uid_hash_find(uid: kuid_t, hashent: *mut hlist_head) -> *mut c_void {
pub static mut user: *mut c_void = core::ptr::null_mut();
    hlist_for_each_entry(user, hashent, uidhash_node) {
    if (uid_eq(user.uid, uid)) {
    refcount_inc(&user.__count);
    return user;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn user_epoll_alloc(up: *mut user_struct) -> c_int {

    return percpu_counter_init(&up.epoll_watches, 0, GFP_KERNEL);

    return 0;

    }
#[no_mangle]
unsafe extern "C" fn user_epoll_free(up: *mut user_struct) {

    percpu_counter_destroy(&up.epoll_watches);

    }
// IRQs are disabled and uidhash_lock is held upon function entry.
// IRQ state (as stored in flags) is restored and uidhash_lock released
// upon function exit.
//
#[no_mangle]
unsafe extern "C" fn free_user(up: *mut user_struct, flags: c_ulong) {
    uid_hash_remove(up);
    spin_unlock_irqrestore(&uidhash_lock, flags);
    user_epoll_free(up);
    kmem_cache_free(uid_cachep, up);
    }
//
// Locate the user_struct for the passed UID.  If found, take a ref on it.  The
// caller must undo that ref with free_uid().
//
// If the user_struct could not be found, return NULL.
//
#[no_mangle]
pub unsafe extern "C" fn find_user(uid: kuid_t) -> *mut c_void {
pub static mut ret: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    spin_lock_irqsave(&uidhash_lock, flags);
    ret = uid_hash_find(uid, uidhashentry(uid));
    spin_unlock_irqrestore(&uidhash_lock, flags);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn free_uid(up: *mut user_struct) {
    let mut flags = 0;
    if (!up) {
    return;
    }
    if (refcount_dec_and_lock_irqsave(&up.__count, &uidhash_lock, &flags)) {
    free_user(up, flags);
    }
    }
    EXPORT_SYMBOL_GPL(free_uid);
#[no_mangle]
pub unsafe extern "C" fn alloc_uid(uid: kuid_t) -> *mut c_void {
    let mut hashent = uidhashentry(uid);
    let mut up = core::ptr::null_mut();
    let mut new = core::ptr::null_mut();
    spin_lock_irq(&uidhash_lock);
    up = uid_hash_find(uid, hashent);
    spin_unlock_irq(&uidhash_lock);
    if (!up) {
    new = kmem_cache_zalloc(uid_cachep, GFP_KERNEL);
    if (!new) {
    return core::ptr::null_mut();
    }
    new.uid = uid;
    refcount_set(&new.__count, 1);
    if (user_epoll_alloc(new)) {
    kmem_cache_free(uid_cachep, new);
    return core::ptr::null_mut();
    }
    ratelimit_state_init(&new.ratelimit, HZ, 100);
    ratelimit_set_flags(&new.ratelimit, RATELIMIT_MSG_ON_RELEASE);
//
// Before adding this, check whether we raced
// on adding the same user already..
//
    spin_lock_irq(&uidhash_lock);
    up = uid_hash_find(uid, hashent);
    if (up) {
    user_epoll_free(new);
    kmem_cache_free(uid_cachep, new);
    } else {
    uid_hash_insert(new, hashent);
    up = new;
    }
    spin_unlock_irq(&uidhash_lock);
    }
    return up;
    }
#[no_mangle]
unsafe extern "C" fn uid_cache_init() -> c_int {
    let mut n = 0;
    uid_cachep = kmem_cache_create("uid_cache", sizeof!(user_struct),
    0, SLAB_HWCACHE_ALIGN|SLAB_PANIC, core::ptr::null_mut());
    for(n = 0; n < UIDHASH_SZ; ++n) {
    INIT_HLIST_HEAD(uidhash_table + n);
    }
    if (user_epoll_alloc(&root_user)) {
    panic("root_user epoll percpu counter alloc failed");
    }
// Insert the root user immediately (init already runs as root)
    spin_lock_irq(&uidhash_lock);
    uid_hash_insert(&root_user, uidhashentry(GLOBAL_ROOT_UID));
    spin_unlock_irq(&uidhash_lock);
    return 0;
    }
    subsys_initcall!(uid_cache_init);
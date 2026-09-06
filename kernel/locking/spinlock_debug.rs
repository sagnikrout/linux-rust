//! Automatically rewritten from C to Rust
//! Source: kernel/locking/spinlock_debug.c
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


//
// Copyright 2005, Red Hat, Inc., Ingo Molnar
// Released under the General Public License (GPL).
//
// This file contains the spinlock/rwlock implementations for
// DEBUG_SPINLOCK.
//

#[no_mangle]
pub unsafe extern "C" fn __raw_spin_lock_init(lock: *mut raw_spinlock_t, name: *mut c_char, key: *mut lock_class_key, inner: c_short) {

//
// Make sure we are not reinitializing a held lock:
//
    debug_check_no_locks_freed(lock, sizeof!(*lock));
    lockdep_init_map_wait(&lock.dep_map, name, key, 0, inner);

    lock.raw_lock = (arch_spinlock_t)__ARCH_SPIN_LOCK_UNLOCKED;
    lock.magic = SPINLOCK_MAGIC;
    lock.owner = SPINLOCK_OWNER_INIT;
    lock.owner_cpu = -1;
    }
    EXPORT_SYMBOL(__raw_spin_lock_init);

#[no_mangle]
pub unsafe extern "C" fn __rwlock_init(lock: *mut rwlock_t, name: *mut c_char, key: *mut lock_class_key) {

//
// Make sure we are not reinitializing a held lock:
//
    debug_check_no_locks_freed(lock, sizeof!(*lock));
    lockdep_init_map_wait(&lock.dep_map, name, key, 0, LD_WAIT_CONFIG);

    lock.raw_lock = (arch_rwlock_t) __ARCH_RW_LOCK_UNLOCKED;
    lock.magic = RWLOCK_MAGIC;
    lock.owner = SPINLOCK_OWNER_INIT;
    lock.owner_cpu = -1;
    }
    EXPORT_SYMBOL(__rwlock_init);

#[no_mangle]
unsafe extern "C" fn spin_dump(lock: *mut raw_spinlock_t, msg: *const c_char) {
    let mut owner = READ_ONCE(lock.owner);
    if (owner == SPINLOCK_OWNER_INIT) {
    owner = core::ptr::null_mut();
    }
    printk("BUG: spinlock %s on CPU#%d, %s/%d\n",
    msg, raw_smp_processor_id(),
    current.comm, task_pid_nr(current));
    printk(" lock: %pS, .magic: %08x, .owner: %s/%d, "
    ".owner_cpu: %d\n",
    lock, READ_ONCE(lock.magic),
    owner ? owner.comm : "<none>",
    owner ? task_pid_nr(owner) : -1,
    READ_ONCE(lock.owner_cpu));
    dump_stack();
    }
#[no_mangle]
unsafe extern "C" fn spin_bug(lock: *mut raw_spinlock_t, msg: *const c_char) {
    if (!debug_locks_off()) {
    return;
    }
    spin_dump(lock, msg);
    }

#[no_mangle]
pub unsafe extern "C" fn debug_spin_lock_before(lock: *mut raw_spinlock_t) {
    SPIN_BUG_ON(READ_ONCE(lock.magic) != SPINLOCK_MAGIC, lock, "bad magic");
    SPIN_BUG_ON(READ_ONCE(lock.owner) == current, lock, "recursion");
    SPIN_BUG_ON(READ_ONCE(lock.owner_cpu) == raw_smp_processor_id(),
    lock, "cpu recursion");
    }
#[no_mangle]
pub unsafe extern "C" fn debug_spin_lock_after(lock: *mut raw_spinlock_t) {
    WRITE_ONCE(lock.owner_cpu, raw_smp_processor_id());
    WRITE_ONCE(lock.owner, current);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_spin_unlock(lock: *mut raw_spinlock_t) {
    SPIN_BUG_ON(lock.magic != SPINLOCK_MAGIC, lock, "bad magic");
    SPIN_BUG_ON(!raw_spin_is_locked(lock), lock, "already unlocked");
    SPIN_BUG_ON(lock.owner != current, lock, "wrong owner");
    SPIN_BUG_ON(lock.owner_cpu != raw_smp_processor_id(),
    lock, "wrong CPU");
    WRITE_ONCE(lock.owner, SPINLOCK_OWNER_INIT);
    WRITE_ONCE(lock.owner_cpu, -1);
    }
//
// We are now relying on the NMI watchdog to detect lockup instead of doing
// the detection here with an unfair lock which can cause problem of its own.
//
#[no_mangle]
pub unsafe extern "C" fn do_raw_spin_lock(lock: *mut raw_spinlock_t) {
    debug_spin_lock_before(lock);
    arch_spin_lock(&lock.raw_lock);
    mmiowb_spin_lock();
    debug_spin_lock_after(lock);
    }
#[no_mangle]
pub unsafe extern "C" fn do_raw_spin_trylock(lock: *mut raw_spinlock_t) -> c_int {
pub static mut ret: c_int = 0;
    if (ret) {
    mmiowb_spin_lock();
    debug_spin_lock_after(lock);
    }

//
// Must not happen on UP:
//
    SPIN_BUG_ON(!ret, lock, "trylock failure on UP");

    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn do_raw_spin_unlock(lock: *mut raw_spinlock_t) {
    mmiowb_spin_unlock();
    debug_spin_unlock(lock);
    arch_spin_unlock(&lock.raw_lock);
    }

#[no_mangle]
unsafe extern "C" fn rwlock_bug(lock: *mut rwlock_t, msg: *const c_char) {
    if (!debug_locks_off()) {
    return;
    }
    printk("BUG: rwlock %s on CPU#%d, %s/%d, %p\n",
    msg, raw_smp_processor_id(), current.comm,
    task_pid_nr(current), lock);
    dump_stack();
    }

#[no_mangle]
pub unsafe extern "C" fn do_raw_read_lock(lock: *mut rwlock_t) {
    RWLOCK_BUG_ON(lock.magic != RWLOCK_MAGIC, lock, "bad magic");
    arch_read_lock(&lock.raw_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn do_raw_read_trylock(lock: *mut rwlock_t) -> c_int {
pub static mut ret: c_int = 0;

//
// Must not happen on UP:
//
    RWLOCK_BUG_ON(!ret, lock, "trylock failure on UP");

    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn do_raw_read_unlock(lock: *mut rwlock_t) {
    RWLOCK_BUG_ON(lock.magic != RWLOCK_MAGIC, lock, "bad magic");
    arch_read_unlock(&lock.raw_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_write_lock_before(lock: *mut rwlock_t) {
    RWLOCK_BUG_ON(lock.magic != RWLOCK_MAGIC, lock, "bad magic");
    RWLOCK_BUG_ON(READ_ONCE(lock.owner) == current, lock, "recursion");
    RWLOCK_BUG_ON(READ_ONCE(lock.owner_cpu) == raw_smp_processor_id(),
    lock, "cpu recursion");
    }
#[no_mangle]
pub unsafe extern "C" fn debug_write_lock_after(lock: *mut rwlock_t) {
    WRITE_ONCE(lock.owner_cpu, raw_smp_processor_id());
    WRITE_ONCE(lock.owner, current);
    }
#[no_mangle]
pub unsafe extern "C" fn debug_write_unlock(lock: *mut rwlock_t) {
    RWLOCK_BUG_ON(lock.magic != RWLOCK_MAGIC, lock, "bad magic");
    RWLOCK_BUG_ON(lock.owner != current, lock, "wrong owner");
    RWLOCK_BUG_ON(lock.owner_cpu != raw_smp_processor_id(),
    lock, "wrong CPU");
    WRITE_ONCE(lock.owner, SPINLOCK_OWNER_INIT);
    WRITE_ONCE(lock.owner_cpu, -1);
    }
#[no_mangle]
pub unsafe extern "C" fn do_raw_write_lock(lock: *mut rwlock_t) {
    debug_write_lock_before(lock);
    arch_write_lock(&lock.raw_lock);
    debug_write_lock_after(lock);
    }
#[no_mangle]
pub unsafe extern "C" fn do_raw_write_trylock(lock: *mut rwlock_t) -> c_int {
pub static mut ret: c_int = 0;
    if (ret) {
    debug_write_lock_after(lock);
    }

//
// Must not happen on UP:
//
    RWLOCK_BUG_ON(!ret, lock, "trylock failure on UP");

    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn do_raw_write_unlock(lock: *mut rwlock_t) {
    debug_write_unlock(lock);
    arch_write_unlock(&lock.raw_lock);
    }
//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/futex/futex.h
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
// Futex flags used to encode options to functions and preserve them across
// restarts.
//
pub const FLAGS_SIZE_8: c_uint = 0x0000;
pub const FLAGS_SIZE_16: c_uint = 0x0001;
pub const FLAGS_SIZE_32: c_uint = 0x0002;
pub const FLAGS_SIZE_64: c_uint = 0x0003;
pub const FLAGS_SIZE_MASK: c_uint = 0x0003;

//
// NOMMU does not have per process address space. Let the compiler optimize
// code away.
//

pub const FLAGS_CLOCKRT: c_uint = 0x0020;
pub const FLAGS_HAS_TIMEOUT: c_uint = 0x0040;
pub const FLAGS_NUMA: c_uint = 0x0080;
pub const FLAGS_STRICT: c_uint = 0x0100;
pub const FLAGS_MPOL: c_uint = 0x0200;
pub const FLAGS_ROBUST_UNLOCK: c_uint = 0x0400;
pub const FLAGS_ROBUST_LIST32: c_uint = 0x0800;
// FUTEX_ to FLAGS_

// FUTEX2_ to FLAGS_
// Only 64bit futexes for 64bit code
// Only 32bit futexes are implemented -- for now
//
// Must be able to represent both FUTEX_NO_NODE and every valid nodeid
// in a futex word.
//

extern "C" {
    pub fn should_fail_futex(fshared: bool) -> bool;
}

//
// Relies on get_futex_key() to set either bit for shared
// futexes -- see comment with union futex_key.
//
// Hash buckets are shared by all the futex_keys that hash to the same
// location.  Each key may have multiple futex_q structures, one for each task
// waiting on a futex.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_hash_bucket {
    pub waiters: core::sync::atomic::AtomicI32,
    pub lock: spinlock_t,
    pub chain: plist_head,
}
//
// Priority Inheritance state:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_pi_state {
//
// list of 'owned' pi_state instances - these have to be
// cleaned up in do_exit() if the task exits prematurely:
//
    pub list: list_head,
//
// The PI object:
//
    pub pi_mutex: rt_mutex_base,
    pub owner: *mut task_struct,
    pub refcount: refcount_t,
    pub key: futex_key,
}
    pub futex_q: struct,
    pub q): *mut *mut typedef void (futex_wake_fn)(wake_q_head wake_q, futex_q,
//
// struct futex_q - The hashed futex queue entry, one per waiting task
// @list:		priority-sorted list of tasks waiting on this futex
// @task:		the task waiting on the futex
// @lock_ptr:		the hash bucket lock
// @wake:		the wake handler for this queue
// @wake_data:		data associated with the wake handler
// @key:		the key the futex is hashed on
// @pi_state:		optional priority inheritance state
// @rt_waiter:		rt_waiter storage for use with requeue_pi
// @requeue_pi_key:	the requeue_pi target futex key
// @bitset:		bitset for the optional bitmasked wakeup
// @requeue_state:	State field for futex_requeue_pi()
// @drop_fph:		Waiter should drop the extra private hash reference when set
// @requeue_wait:	RCU wait for futex_requeue_pi() (RT only)
//
// We use this hashed waitqueue, instead of a normal wait_queue_entry_t, so
// we can wake only the relevant ones (hashed queues may be shared).
//
// A futex_q has a woken state, just like tasks have TASK_RUNNING.
// It is considered woken when plist_node_empty(&q->list) || q->lock_ptr == 0.
// The order of wakeup is always to make the first condition true, then
// the second.
//
// PI futexes are typically woken before they are removed from the hash list via
// the rt_mutex code. See futex_unqueue_pi().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_q {
    pub list: plist_node,
    pub task: *mut task_struct,
    pub lock_ptr: *mut spinlock_t,
    pub wake: *mut futex_wake_fn,
    pub wake_data: *mut c_void,
    pub key: futex_key,
    pub pi_state: *mut futex_pi_state,
    pub rt_waiter: *mut rt_mutex_waiter,
    pub requeue_pi_key: *mut futex_key,
    pub bitset: u32,
    pub requeue_state: core::sync::atomic::AtomicI32,
    pub drop_fph: *mut futex_private_hash,

    pub requeue_wait: rcuwait,

}
    pub futex_q_init: extern struct futex_q,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum futex_access {
    FUTEX_READ,
    FUTEX_WRITE
}

    pub rw): futex_access,
    pub __acquires(q->lock_ptr): *mut *mut extern void futex_q_lockptr_lock(futex_q q),
    pub range_ns): int flags, u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_bucket_ref {
    pub hb: *mut futex_hash_bucket,
    pub fph: *mut futex_private_hash,
}

extern "C" {
    pub fn futex_private_hash_put(fph: *mut futex_private_hash);
}

extern "C" {
    pub fn futex_hash(key: *mut futex_key) -> futex_bucket_ref;
}
//
// futex_match - Check whether two futex keys are equal
// @key1:	Pointer to key1
// @key2:	Pointer to key2
//
// Return 1 if two futex_keys are equal, 0 otherwise.
//
extern "C" {
    pub fn futex_do_wait(q: *mut futex_q, timeout: *mut hrtimer_sleeper);
}
extern "C" {
    pub fn __futex_wake_mark(q: *mut futex_q) -> bool;
}
extern "C" {
    pub fn futex_wake_mark(wake_q: *mut wake_q_head, q: *mut futex_q);
}
extern "C" {
    pub fn fault_in_user_writeable(uaddr: *mut u32 ) -> c_int;
}
// Read from user memory with pagefaults disabled
extern "C" {
    pub fn get_user_inline(_arg: *mut dest, _arg: from) -> return;
}
extern "C" {
    pub fn __futex_unqueue(q: *mut futex_q);
}
extern "C" {
    pub fn futex_unqueue(q: *mut futex_q) -> c_int;
}
//
// futex_queue() - Enqueue the futex_q on the futex_hash_bucket
// @q:	The futex_q to enqueue
// @hb:	The destination hash bucket
// @task: Task queueing this futex
//
// The hb->lock must be held by the caller, and is released here. A call to
// futex_queue() is typically paired with exactly one call to futex_unqueue().  The
// exceptions involve the PI related operations, which may use futex_unqueue_pi()
// or nothing if the unqueue is done as part of the wake process and the unqueue
// state is implicit in the state of woken task (see futex_wait_requeue_pi() for
// an example).
//
// Note that @task may be NULL, for async usage of futexes.
//
extern "C" {
    pub fn futex_unqueue_pi(q: *mut futex_q);
}
extern "C" {
    pub fn wait_for_owner_exiting(ret: c_int, exiting: *mut task_struct);
}
//
// Reflects a new waiter being added to the waitqueue.
//

//
// Full barrier (A), see the ordering comment above.
//

//
// Reflects a waiter being removed from the waitqueue by wakeup
// paths.
//

//
// Full barrier (B), see the ordering comment above.
//
extern "C" {
    pub fn atomic_read(_arg: &hb->waiters) -> return;
}

extern "C" {
    pub fn refill_pi_state_cache() -> c_int;
}
extern "C" {
    pub fn get_pi_state(pi_state: *mut futex_pi_state);
}
extern "C" {
    pub fn put_pi_state(pi_state: *mut futex_pi_state);
}
extern "C" {
    pub fn fixup_pi_owner(uaddr: *mut u32 , q: *mut futex_q, locked: c_int) -> c_int;
}
//
// Express the locking dependencies for lockdep:
//
// syscalls
// uaddr2);
//
// struct futex_vector - Auxiliary struct for futex_waitv()
// @w: Userspace provided data
// @q: Kernel side data
//
// Struct used to build an array with all data need for futex_waitv()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_vector {
    pub w: futex_waitv,
    pub q: futex_q,
}

extern "C" {
    pub fn futex_unqueue_multiple(v: *mut futex_vector, count: c_int) -> c_int;
}
extern "C" {
    pub fn futex_unlock_pi(uaddr: *mut u32 , flags: c_uint, pop: *mut c_void ) -> c_int;
}
extern "C" {
    pub fn futex_lock_pi(uaddr: *mut u32 , flags: c_uint, time: *mut ktime_t, trylock: c_int) -> c_int;
}
extern "C" {
    pub fn futex_robust_list_clear_pending(pop: *mut c_void , flags: c_uint) -> bool;
}
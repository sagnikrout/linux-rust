//! Automatically rewritten from C to Rust
//! Source: ipc/sem.c
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
macro_rules! printk { ($($tt:tt)*) => { 0 }; }
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
macro_rules! rootfs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! pure_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! min_t { ($($tt:tt)*) => { 0 }; }
macro_rules! max_t { ($($tt:tt)*) => { 0 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! MKDEV { ($($tt:tt)*) => { 0u32 }; }
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
macro_rules! pr_warn_once { ($($tt:tt)*) => {}; }
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
pub struct ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_ids { pub _opaque: [u8; 0] }

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
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

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
pub type compat_uptr_t = u32;
pub type compat_long_t = i32;
pub type compat_ulong_t = u32;
pub type compat_size_t = u32;
pub type __compat_uid_t = u32;
pub type __compat_gid_t = u32;
pub type compat_mode_t = u32;
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
pub const ENOSYS: c_int = 38;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;
pub const SHMLBA: usize = 4096;
pub const COMPAT_SHMLBA: usize = 4096;

// Standard File Mode Constants
pub const S_IFCHR: u32 = 0x2000;
pub const S_IFDIR: u32 = 0x4000;
pub const S_IFREG: u32 = 0x8000;
pub const S_IFBLK: u32 = 0x6000;
pub const S_IFIFO: u32 = 0x1000;
pub const S_IFLNK: u32 = 0xa000;
pub const S_IFSOCK: u32 = 0xc000;
pub const S_IRWXU: u32 = 0x01c0;
pub const S_IRUSR: u32 = 0x0100;
pub const S_IWUSR: u32 = 0x0080;
pub const S_IXUSR: u32 = 0x0040;
pub const S_IRUGO: u32 = 0x0124;
pub const S_IWUGO: u32 = 0x0092;
pub const S_IXUGO: u32 = 0x0049;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn usermodehelper_enable();
    pub fn new_encode_dev(dev: u32) -> u32;
}

pub unsafe fn init_mkdir<T>(_path: T, _mode: u32) -> c_int { 0 }
pub unsafe fn init_mknod<T>(_path: T, _mode: u32, _dev: u32) -> c_int { 0 }
// === KERNEL_MACRO_PRELUDE_END ===



// SPDX-License-Identifier: GPL-2.0
//
// linux/ipc/sem.c
// Copyright (C) 1992 Krishna Balasubramanian
// Copyright (C) 1995 Eric Schenk, Bruno Haible
//
// /proc/sysvipc/sem support (c) 1999 Dragos Acostachioaie <dragos@iname.com>
//
// SMP-threaded, sysctl's added
// (c) 1999 Manfred Spraul <manfred@colorfullife.com>
// Enforced range limit on SEM_UNDO
// (c) 2001 Red Hat Inc
// Lockless wakeup
// (c) 2003 Manfred Spraul <manfred@colorfullife.com>
// (c) 2016 Davidlohr Bueso <dave@stgolabs.net>
// Further wakeup optimizations, documentation
// (c) 2010 Manfred Spraul <manfred@colorfullife.com>
//
// support for audit of ipc object properties and permission changes
// Dustin Kirkland <dustin.kirkland@us.ibm.com>
//
// namespaces support
// OpenVZ, SWsoft Inc.
// Pavel Emelianov <xemul@openvz.org>
//
// Implementation notes: (May 2010)
// This file implements System V semaphores.
//
// User space visible behavior:
// - FIFO ordering for semop() operations (just FIFO, not starvation
// protection)
// - multiple semaphore operations that alter the same semaphore in
// one semop() are handled.
// - sem_ctime (time of last semctl()) is updated in the IPC_SET, SETVAL and
// SETALL calls.
// - two Linux specific semctl() commands: SEM_STAT, SEM_INFO.
// - undo adjustments at process exit are limited to 0..SEMVMX.
// - namespace are supported.
// - SEMMSL, SEMMNS, SEMOPM and SEMMNI can be configured at runtime by writing
// to /proc/sys/kernel/sem.
// - statistics about the usage are reported in /proc/sysvipc/sem.
//
// Internals:
// - scalability:
// - all global variables are read-mostly.
// - semop() calls and semctl(RMID) are synchronized by RCU.
// - most operations do write operations (actually: spin_lock calls) to
// the per-semaphore array structure.
// Thus: Perfect SMP scaling between independent semaphore arrays.
// If multiple semaphores in one array are used, then cache line
// trashing on the semaphore array spinlock will limit the scaling.
// - semncnt and semzcnt are calculated on demand in count_semcnt()
// - the task that performs a successful semop() scans the list of all
// sleeping tasks and completes any pending operations that can be fulfilled.
// Semaphores are actively given to waiting tasks (necessary for FIFO).
// (see update_queue())
// - To improve the scalability, the actual wake-up calls are performed after
// dropping all locks. (see wake_up_sem_queue_prepare())
// - All work is done by the waker, the woken up task does not have to do
// anything - not even acquiring a lock or dropping a refcount.
// - A woken up task may not even touch the semaphore array anymore, it may
// have been destroyed already by a semctl(RMID).
// - UNDO values are stored in an array (one per process and per
// semaphore array, lazily allocated). For backwards compatibility, multiple
// modes for the UNDO variables are supported (per process, per thread)
// (see copy_semundo, CLONE_SYSVSEM)
// - There are two lists of the pending operations: a per-array list
// and per-semaphore list (stored in the array). This allows to achieve FIFO
// ordering without always scanning all pending operations.
// The worst-case behavior is nevertheless O(N^2) for N wakeups.
//

// One semaphore structure for each semaphore in the system.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem {
//     pub /: *mut *mut int semval; / current value,
//
// PID of the process that last modified the semaphore. For
// Linux, specifically these are:
// - semop
// - semctl, via SETVAL and SETALL.
// - at task exit when performing undo adjustments (see exit_sem).
//
    pub sempid: *mut pid,
//     pub /: *mut *mut spinlock_t lock; / spinlock for fine-grained semtimedop,
//     pub /: *mut *mut list_head pending_alter; / pending single-sop operations,
// that alter the semaphore
//     pub /: *mut *mut list_head pending_const; / pending single-sop operations,
// that do not alter the semaphore
//     pub /: *mut *mut time64_t sem_otime; / candidate for sem_otime,
}
// One sem_array data structure for each set of semaphores in the system.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array {
//     pub /: *mut *mut kern_ipc_perm sem_perm; / permissions .. see ipc.h,
//     pub /: *mut *mut time64_t sem_ctime; / create/last semctl() time,
//     pub /: *mut *mut list_head pending_alter; / pending operations,
// that alter the array
//     pub /: *mut *mut list_head pending_const; / pending complex operations,
// that do not alter semvals
//     pub /: *mut *mut list_head list_id; / undo requests on this array,
//     pub /: *mut *mut int sem_nsems; / no. of semaphores in array,
//     pub /: *mut *mut int complex_count; / pending complex operations,
//     pub /: *mut *mut unsigned int use_global_lock;/ >0: global lock required,
    pub sems: [sem; 0],
}
// One queue for each sleeping process in the system.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_queue {
//     pub /: *mut *mut list_head list; / queue of pending operations,
//     pub /: *mut *mut *mut task_sleeper; / this process,
//     pub /: *mut *mut *mut sem_undo undo; / undo structure,
//     pub /: *mut *mut *mut pid pid; / process id of requesting process,
//     pub /: *mut *mut int status; / completion status of operation,
//     pub /: *mut *mut *mut sembuf sops; / array of pending operations,
//     pub /: *mut *mut *mut sembuf blocking; / the operation that blocked,
//     pub /: *mut *mut int nsops; / number of operations,
//     pub /: *mut *mut *mut bool alter; / does sops alter the array?,
//     pub /: *mut *mut bool dupsop; / sops on more than one sem_num,
}

// Each task has a list of undo requests. They are executed automatically
// when the process exits.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_undo {
// mangled field
// all undos from one process
// rcu protected
//     pub /: *mut *mut rcu_head rcu; / rcu for sem_undo,
//     pub /: *mut *mut *mut sem_undo_list ulp; / back ptr to sem_undo_list,
// mangled field
// all undos for one array
//     pub /: *mut *mut int semid; / semaphore set identifier,
//     pub /: *mut *mut short semadj[]; / array of adjustments,
// one per semaphore
}

// sem_undo_list controls shared access to the list of sem_undo structures
// that may be shared among all a CLONE_SYSVSEM task group.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_undo_list {
    pub refcnt: refcount_t,
    pub lock: spinlock_t,
    pub list_proc: list_head,
}

// forward_decl: newary;
// forward_decl: freeary;

// forward_decl: sysvipc_sem_proc_show;

//
// Switching from the mode suitable for simple ops
// to the mode for complex ops is costly. Therefore:
// use some hysteresis
//
pub const USE_GLOBAL_LOCK_HYSTERESIS: c_int = 10;
//
// Locking:
// a) global sem_lock() for read/write
// sem_undo.id_next,
// sem_array.complex_count,
// sem_array.pending{_alter,_const},
// sem_array.sem_undo
//
// b) global or semaphore sem_lock() for read/write:
// sem_array.sems[i].pending_{const,alter}:
//
// c) special:
// sem_undo_list.list_proc:
// * undo_list->lock for write
// * rcu for read
// use_global_lock:
// * global sem_lock() for write
// * either local or global sem_lock() for read.
//
// Memory ordering:
// Most ordering is enforced by using spin_lock() and spin_unlock().
//
// Exceptions:
// 1) use_global_lock: (SEM_BARRIER_1)
// Setting it from non-zero to 0 is a RELEASE, this is ensured by
// using smp_store_release(): Immediately after setting it to 0,
// a simple op can start.
// Testing if it is non-zero is an ACQUIRE, this is ensured by using
// smp_load_acquire().
// Setting it from 0 to non-zero must be ordered with regards to
// this smp_load_acquire(), this is guaranteed because the smp_load_acquire()
// is inside a spin_lock() and after a write from 0 to non-zero a
// spin_lock()+spin_unlock() is done.
// To prevent the compiler/cpu temporarily writing 0 to use_global_lock,
// READ_ONCE()/WRITE_ONCE() is used.
//
// 2) queue.status: (SEM_BARRIER_2)
// Initialization is done while holding sem_lock(), so no further barrier is
// required.
// Setting it to a result code is a RELEASE, this is ensured by both a
// smp_store_release() (for case a) and while holding sem_lock()
// (for case b).
// The ACQUIRE when reading the result code without holding sem_lock() is
// achieved by using READ_ONCE() + smp_acquire__after_ctrl_dep().
// (case a above).
// Reading the result code while holding sem_lock() needs no further barriers,
// the locks inside sem_lock() enforce ordering (case b above)
//
// 3) current->state:
// current->state is set to TASK_INTERRUPTIBLE while holding sem_lock().
// The wakeup is handled using the wake_q infrastructure. wake_q wakeups may
// happen immediately after calling wake_q_add. As wake_q_add_safe() is called
// when holding sem_lock(), no further barriers are required.
//
// See also ipc/mqueue.c for more details on the covered races.
//

#[no_mangle]
pub unsafe extern "C" fn sem_init_ns(ns: *mut ipc_namespace) {
    ns.sc_semmsl = SEMMSL;
    ns.sc_semmns = SEMMNS;
    ns.sc_semopm = SEMOPM;
    ns.sc_semmni = SEMMNI;
    ns.used_sems = 0;
    ipc_init_ids(&ns.ids[IPC_SEM_IDS]);
    }

#[no_mangle]
pub unsafe extern "C" fn sem_exit_ns(ns: *mut ipc_namespace) {
    free_ipcs(ns, &sem_ids(ns), freeary);
    idr_destroy(&ns.ids[IPC_SEM_IDS].ipcs_idr);
    rhashtable_destroy(&ns.ids[IPC_SEM_IDS].key_ht);
    }

#[no_mangle]
pub unsafe extern "C" fn sem_init()  {
    sem_init_ns(&init_ipc_ns);
    ipc_init_proc_interface("sysvipc/sem",
    "       key      semid perms      nsems   uid   gid  cuid  cgid      otime      ctime\n",
    IPC_SEM_IDS, sysvipc_sem_proc_show);
    }
//
// unmerge_queues - unmerge queues, if possible.
// @sma: semaphore array
//
// The function unmerges the wait queues if complex_count is 0.
// It must be called prior to dropping the global semaphore array lock.
//
#[no_mangle]
unsafe extern "C" fn unmerge_queues(sma: *mut sem_array) {
    let mut q = core::ptr::null_mut();
    let mut tq = core::ptr::null_mut();
// complex operations still around?
    if (sma.complex_count) {
    return;
    }
//
// We will switch back to simple mode.
// Move all pending operation back into the per-semaphore
// queues.
//
    if false {
pub static mut curr: *mut c_void = core::ptr::null_mut();
    curr = &sma.sems[q.sops[0].sem_num];
    list_add_tail(&q.list, &curr.pending_alter);
    }
    INIT_LIST_HEAD(&sma.pending_alter);
    }
//
// merge_queues - merge single semop queues into global queue
// @sma: semaphore array
//
// This function merges all per-semaphore queues into the global queue.
// It is necessary to achieve FIFO ordering for the pending single-sop
// operations when a multi-semop operation must sleep.
// Only the alter operations must be moved, the const operations can stay.
//
#[no_mangle]
unsafe extern "C" fn merge_queues(sma: *mut sem_array) {
    let mut i = 0;
    while (i < sma.sem_nsems) {
    let mut sem = core::ptr::null_mut();
    list_splice_init(&sem.pending_alter, &sma.pending_alter);
    }
    }
#[no_mangle]
unsafe extern "C" fn sem_rcu_free(head: *mut rcu_head) {
    let mut p = container_of!(head, kern_ipc_perm, rcu);
    let mut sma = container_of!(p, sem_array, sem_perm);
    security_sem_free(&sma.sem_perm);
    kvfree(sma);
    }
//
// Enter the mode suitable for non-simple operations:
// Caller must own sem_perm.lock.
//
#[no_mangle]
unsafe extern "C" fn complexmode_enter(sma: *mut sem_array) {
    let mut i = 0;
pub static mut sem: *mut c_void = core::ptr::null_mut();
    if (sma.use_global_lock > 0)  {
//
// We are already in global lock mode.
// Nothing to do, just reset the
// counter until we return to simple mode.
//
    WRITE_ONCE(sma.use_global_lock, USE_GLOBAL_LOCK_HYSTERESIS);
    return;
    }
    WRITE_ONCE(sma.use_global_lock, USE_GLOBAL_LOCK_HYSTERESIS);
    while (i < sma.sem_nsems) {
    sem = &sma.sems[i];
    spin_lock(&sem.lock);
    spin_unlock(&sem.lock);
    }
    }
//
// Try to leave the mode that disallows simple operations:
// Caller must own sem_perm.lock.
//
#[no_mangle]
unsafe extern "C" fn complexmode_tryleave(sma: *mut sem_array) {
    if (sma.complex_count)  {
// Complex ops are sleeping.
// We must stay in complex mode
//
    return;
    }
    if (sma.use_global_lock == 1) {
// See SEM_BARRIER_1 for purpose/pairing
    smp_store_release(&sma.use_global_lock, 0);
    } else {
    WRITE_ONCE(sma.use_global_lock,
    sma.use_global_lock-1);
    }
    }

//
// If the request contains only one semaphore operation, and there are
// no complex transactions pending, lock only the semaphore involved.
// Otherwise, lock the entire semaphore array, since we either have
// multiple semaphores in our own semops, or we need to look at
// semaphores from other pending complex operations.
//
#[no_mangle]
pub unsafe extern "C" fn sem_lock(sma: *mut sem_array, sops: *mut sembuf, nsops: c_int) -> c_int {
pub static mut sem: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    if (nsops != 1) {
// Complex operation - acquire a full lock
    ipc_lock_object(&sma.sem_perm);
// Prevent parallel simple ops
    complexmode_enter(sma);
    return SEM_GLOBAL_LOCK;
    }
//
// Only one semaphore affected - try to optimize locking.
// Optimized locking is possible if no complex operation
// is either enqueued or processed right now.
//
// Both facts are tracked by use_global_mode.
//
    idx = array_index_nospec(sops.sem_num, sma.sem_nsems);
    sem = &sma.sems[idx];
//
// Initial check for use_global_lock. Just an optimization,
// no locking, no memory barrier.
//
    if (!READ_ONCE(sma.use_global_lock)) {
//
// It appears that no complex operation is around.
// Acquire the per-semaphore lock.
//
    spin_lock(&sem.lock);
// see SEM_BARRIER_1 for purpose/pairing
    if (!smp_load_acquire(&sma.use_global_lock)) {
// fast path successful!
    return sops.sem_num;
    }
    spin_unlock(&sem.lock);
    }
// slow path: acquire the full lock
    ipc_lock_object(&sma.sem_perm);
    if (sma.use_global_lock == 0) {
//
// The use_global_lock mode ended while we waited for
// sma->sem_perm.lock. Thus we must switch to locking
// with sem->lock.
// Unlike in the fast path, there is no need to recheck
// sma->use_global_lock after we have acquired sem->lock:
// We own sma->sem_perm.lock, thus use_global_lock cannot
// change.
//
    spin_lock(&sem.lock);
    ipc_unlock_object(&sma.sem_perm);
    return sops.sem_num;
    } else {
//
// Not a false alarm, thus continue to use the global lock
// mode. No need for complexmode_enter(), this was done by
// the caller that has set use_global_mode to non-zero.
//
    return SEM_GLOBAL_LOCK;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sem_unlock(sma: *mut sem_array, locknum: c_int) {
    if (locknum == SEM_GLOBAL_LOCK) {
    unmerge_queues(sma);
    complexmode_tryleave(sma);
    ipc_unlock_object(&sma.sem_perm);
    } else {
    let mut sem = core::ptr::null_mut();
    spin_unlock(&sem.lock);
    }
    }
//
// sem_lock_(check_) routines are called in the paths where the rwsem
// is not held.
//
// The caller holds the RCU read lock.
//
#[no_mangle]
pub unsafe extern "C" fn sem_obtain_object(ns: *mut ipc_namespace, id: c_int) -> *mut c_void {
    let mut ipcp = ipc_obtain_object_idr(&sem_ids(ns), id);
    if (IS_ERR(ipcp)) {
    return ERR_CAST(ipcp);
    }
    return container_of!(ipcp, sem_array, sem_perm);
    }
#[no_mangle]
pub unsafe extern "C" fn sem_obtain_object_check(ns: *mut ipc_namespace, id: c_int) -> *mut c_void {
    let mut ipcp = ipc_obtain_object_check(&sem_ids(ns), id);
    if (IS_ERR(ipcp)) {
    return ERR_CAST(ipcp);
    }
    return container_of!(ipcp, sem_array, sem_perm);
    }
#[no_mangle]
pub unsafe extern "C" fn sem_lock_and_putref(sma: *mut sem_array) {
    sem_lock(sma, core::ptr::null_mut(), -1);
    ipc_rcu_putref(&sma.sem_perm, sem_rcu_free);
    }
#[no_mangle]
pub unsafe extern "C" fn sem_rmid(ns: *mut ipc_namespace, s: *mut sem_array) {
    ipc_rmid(&sem_ids(ns), &s.sem_perm);
    }
#[no_mangle]
pub unsafe extern "C" fn sem_alloc(nsems: size_t) -> *mut c_void {
pub static mut sma: *mut c_void = core::ptr::null_mut();
    if (nsems > (INT_MAX - sizeof!(*sma)) / sizeof!(sma.sems[0])) {
    return core::ptr::null_mut();
    }
    sma = kvzalloc_flex(*sma, sems, nsems, GFP_KERNEL_ACCOUNT);
    if (unlikely(!sma)) {
    return core::ptr::null_mut();
    }
    return sma;
    }
//
// newary - Create a new semaphore set
// @ns: namespace
// @params: ptr to the structure that contains key, semflg and nsems
//
// Called with sem_ids.rwsem held (as a writer)
//
#[no_mangle]
unsafe extern "C" fn newary(ns: *mut ipc_namespace, params: *mut ipc_params) -> c_int {
    let mut retval = 0;
pub static mut sma: *mut c_void = core::ptr::null_mut();
pub static mut key: key_t = 0;
pub static mut nsems: c_int = 0;
pub static mut semflg: c_int = 0;
    let mut i = 0;
    if (!nsems) {
    return -EINVAL;
    }
    if (ns.used_sems + nsems > ns.sc_semmns) {
    return -ENOSPC;
    }
    sma = sem_alloc(nsems);
    if (!sma) {
    return -ENOMEM;
    }
    sma.sem_perm.mode = (semflg & S_IRWXUGO);
    sma.sem_perm.key = key;
    sma.sem_perm.security = core::ptr::null_mut();
    retval = security_sem_alloc(&sma.sem_perm);
    if (retval) {
    kvfree(sma);
    return retval;
    }
    while (i < nsems) {
    INIT_LIST_HEAD(&sma.sems[i].pending_alter);
    INIT_LIST_HEAD(&sma.sems[i].pending_const);
    spin_lock_init(&sma.sems[i].lock);
    }
    sma.complex_count = 0;
    sma.use_global_lock = USE_GLOBAL_LOCK_HYSTERESIS;
    INIT_LIST_HEAD(&sma.pending_alter);
    INIT_LIST_HEAD(&sma.pending_const);
    INIT_LIST_HEAD(&sma.list_id);
    sma.sem_nsems = nsems;
    sma.sem_ctime = ktime_get_real_seconds();
// ipc_addid() locks sma upon success.
    retval = ipc_addid(&sem_ids(ns), &sma.sem_perm, ns.sc_semmni);
    if (retval < 0) {
    ipc_rcu_putref(&sma.sem_perm, sem_rcu_free);
    return retval;
    }
    ns.used_sems += nsems;
    sem_unlock(sma, -1);
    rcu_read_unlock();
    return sma.sem_perm.id;
    }
//
// Called with sem_ids.rwsem and ipcp locked.
//
#[no_mangle]
unsafe extern "C" fn sem_more_checks(ipcp: *mut kern_ipc_perm, params: *mut ipc_params) -> c_int {
pub static mut sma: *mut c_void = core::ptr::null_mut();
    sma = container_of!(ipcp, sem_array, sem_perm);
    if (params.u.nsems > sma.sem_nsems) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ksys_semget(key: key_t, nsems: c_int, semflg: c_int) -> c_long {
pub static mut ns: *mut c_void = core::ptr::null_mut();
pub static mut ipc_ops: usize = 0;
pub static mut sem_params: usize = 0;
    ns = current.nsproxy.ipc_ns;
    if (nsems < 0 || nsems > ns.sc_semmsl) {
    return -EINVAL;
    }
    sem_params.key = key;
    sem_params.flg = semflg;
    sem_params.u.nsems = nsems;
    return ipcget(ns, &sem_ids(ns), &sem_ops, &sem_params);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_semget(key: usize, nsems: usize, semflg: usize) -> c_long {
    return ksys_semget(key, nsems, semflg);
    }
//
// perform_atomic_semop[_slow] - Attempt to perform semaphore
// operations on a given array.
// @sma: semaphore array
// @q: sem_queue that describes the operation
//
// Caller blocking are as follows, based the value
// indicated by the semaphore operation (sem_op):
//
// (1) >0 never blocks.
// (2)  0 (wait-for-zero operation): semval is non-zero.
// (3) <0 attempting to decrement semval to a value smaller than zero.
//
// Returns 0 if the operation was possible.
// Returns 1 if the operation is impossible, the caller must sleep.
// Returns <0 for error codes.
//
#[no_mangle]
unsafe extern "C" fn perform_atomic_semop_slow(sma: *mut sem_array, q: *mut sem_queue) -> c_int {
    let mut result = 0;
    let mut sem_op = 0;
    let mut nsops = 0;
pub static mut pid: *mut c_void = core::ptr::null_mut();
pub static mut sop: *mut c_void = core::ptr::null_mut();
pub static mut curr: *mut c_void = core::ptr::null_mut();
pub static mut sops: *mut c_void = core::ptr::null_mut();
pub static mut un: *mut c_void = core::ptr::null_mut();
    sops = q.sops;
    nsops = q.nsops;
    un = q.undo;
    while (sop < sops + nsops) {
pub static mut idx: c_int = 0;
    curr = &sma.sems[idx];
    sem_op = sop.sem_op;
    result = curr.semval;
    if (!sem_op && result) {
// goto;
    }
    result += sem_op;
    if (result < 0) {
// goto;
    }
    if (result > SEMVMX) {
// goto;
    }
    if (sop.sem_flg & SEM_UNDO) {
pub static mut undo: c_int = 0;
// Exceeding the undo range is an error.
    if (undo < (-SEMAEM - 1) || undo > SEMAEM) {
// goto;
    }
    un.semadj[sop.sem_num] = undo;
    }
    curr.semval = result;
    }
    sop -= 1;
    pid = q.pid;
    while (sop >= sops) {
    ipc_update_pid(&sma.sems[sop.sem_num].sempid, pid);
    sop -= 1;
    }
    return 0;
    // label: out_of_range
    result = -ERANGE;
// goto;
    // label: would_block
    q.blocking = sop;
    if (sop.sem_flg & IPC_NOWAIT) {
    result = -EAGAIN;
    }
    else {
    result = 1;
    }
    // label: undo
    sop -= 1;
    while (sop >= sops) {
    sem_op = sop.sem_op;
    sma.sems[sop.sem_num].semval -= sem_op;
    if (sop.sem_flg & SEM_UNDO) {
    un.semadj[sop.sem_num] += sem_op;
    }
    sop -= 1;
    }
    return result;
    }
#[no_mangle]
unsafe extern "C" fn perform_atomic_semop(sma: *mut sem_array, q: *mut sem_queue) -> c_int {
    let mut result = 0;
    let mut sem_op = 0;
    let mut nsops = 0;
pub static mut sop: *mut c_void = core::ptr::null_mut();
pub static mut curr: *mut c_void = core::ptr::null_mut();
pub static mut sops: *mut c_void = core::ptr::null_mut();
pub static mut un: *mut c_void = core::ptr::null_mut();
    sops = q.sops;
    nsops = q.nsops;
    un = q.undo;
    if (unlikely(q.dupsop)) {
    return perform_atomic_semop_slow(sma, q);
    }
//
// We scan the semaphore set twice, first to ensure that the entire
// operation can succeed, therefore avoiding any pointless writes
// to shared memory and having to undo such changes in order to block
// until the operations can go through.
//
    while (sop < sops + nsops) {
pub static mut idx: c_int = 0;
    curr = &sma.sems[idx];
    sem_op = sop.sem_op;
    result = curr.semval;
    if (!sem_op && result) {
// goto; /* wait-for-zero */
    }
    result += sem_op;
    if (result < 0) {
// goto;
    }
    if (result > SEMVMX) {
    return -ERANGE;
    }
    if (sop.sem_flg & SEM_UNDO) {
pub static mut undo: c_int = 0;
// Exceeding the undo range is an error.
    if (undo < (-SEMAEM - 1) || undo > SEMAEM) {
    return -ERANGE;
    }
    }
    }
    while (sop < sops + nsops) {
    curr = &sma.sems[sop.sem_num];
    sem_op = sop.sem_op;
    if (sop.sem_flg & SEM_UNDO) {
pub static mut undo: c_int = 0;
    un.semadj[sop.sem_num] = undo;
    }
    curr.semval += sem_op;
    ipc_update_pid(&curr.sempid, q.pid);
    }
    return 0;
    // label: would_block
    q.blocking = sop;
    return if sop.sem_flg & IPC_NOWAIT { -EAGAIN } else { 1 };
    }
#[no_mangle]
pub unsafe extern "C" fn wake_up_sem_queue_prepare(q: *mut sem_queue, error: c_int, wake_q: *mut wake_q_head) {
pub static mut sleeper: *mut c_void = core::ptr::null_mut();
    sleeper = get_task_struct(q.sleeper);
// see SEM_BARRIER_2 for purpose/pairing
    smp_store_release(&q.status, error);
    wake_q_add_safe(wake_q, sleeper);
    }
#[no_mangle]
unsafe extern "C" fn unlink_queue(sma: *mut sem_array, q: *mut sem_queue) {
    list_del(&q.list);
    if (q.nsops > 1) {
    sma.complex_count -= 1;
    }
    }
// check_restart(sma, q)
// @sma: semaphore array
// @q: the operation that just completed
//
// update_queue is O(N^2) when it restarts scanning the whole queue of
// waiting operations. Therefore this function checks if the restart is
// really necessary. It is called after a previously waiting operation
// modified the array.
// Note that wait-for-zero operations are handled without restart.
//
#[no_mangle]
pub unsafe extern "C" fn check_restart(sma: *mut sem_array, q: *mut sem_queue) -> c_int {
// pending complex alter operations are too difficult to analyse
    if (!list_empty(&sma.pending_alter)) {
    return 1;
    }
// we were a sleeping complex operation. Too difficult
    if (q.nsops > 1) {
    return 1;
    }
// It is impossible that someone waits for the new value:
// - complex operations always restart.
// - wait-for-zero are handled separately.
// - q is a previously sleeping simple operation that
// altered the array. It must be a decrement, because
// simple increments never sleep.
// - If there are older (higher priority) decrements
// in the queue, then they have observed the original
// semval value and couldn't proceed. The operation
// decremented to value - thus they won't proceed either.
//
    return 0;
    }
//
// wake_const_ops - wake up non-alter tasks
// @sma: semaphore array.
// @semnum: semaphore that was modified.
// @wake_q: lockless wake-queue head.
//
// wake_const_ops must be called after a semaphore in a semaphore array
// was set to 0. If complex const operations are pending, wake_const_ops must
// be called with semnum = -1, as well as with the number of each modified
// semaphore.
// The tasks that must be woken up are added to @wake_q. The return code
// is stored in q->pid.
// The function returns 1 if at least one operation was completed successfully.
//
#[no_mangle]
pub unsafe extern "C" fn wake_const_ops(sma: *mut sem_array, semnum: c_int, wake_q: *mut wake_q_head) -> c_int {
    let mut q = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
pub static mut pending_list: *mut c_void = core::ptr::null_mut();
pub static mut semop_completed: c_int = 0;
    if (semnum == -1) {
    pending_list = &sma.pending_const;
    }
    else {
    pending_list = &sma.sems[semnum].pending_const;
    }
    if false {
pub static mut error: c_int = 0;
    if (error > 0) {
    continue;
    }
// operation completed, remove from queue & wakeup
    unlink_queue(sma, q);
    wake_up_sem_queue_prepare(q, error, wake_q);
    if (error == 0) {
    semop_completed = 1;
    }
    }
    return semop_completed;
    }
//
// do_smart_wakeup_zero - wakeup all wait for zero tasks
// @sma: semaphore array
// @sops: operations that were performed
// @nsops: number of operations
// @wake_q: lockless wake-queue head
//
// Checks all required queue for wait-for-zero operations, based
// on the actual changes that were performed on the semaphore array.
// The function returns 1 if at least one operation was completed successfully.
//
#[no_mangle]
pub unsafe extern "C" fn do_smart_wakeup_zero(sma: *mut sem_array, sops: *mut sembuf, nsops: c_int, wake_q: *mut wake_q_head) -> c_int {
    let mut i = 0;
pub static mut semop_completed: c_int = 0;
pub static mut got_zero: c_int = 0;
// first: the per-semaphore queues, if known
    if (sops) {
    while (i < nsops) {
pub static mut num: c_int = 0;
    if (sma.sems[num].semval == 0) {
    got_zero = 1;
    semop_completed |= wake_const_ops(sma, num, wake_q);
    }
    }
    } else {
//
// No sops means modified semaphores not known.
// Assume all were changed.
//
    while (i < sma.sem_nsems) {
    if (sma.sems[i].semval == 0) {
    got_zero = 1;
    semop_completed |= wake_const_ops(sma, i, wake_q);
    }
    }
    }
//
// If one of the modified semaphores got 0,
// then check the global queue, too.
//
    if (got_zero) {
    semop_completed |= wake_const_ops(sma, -1, wake_q);
    }
    return semop_completed;
    }
//
// update_queue - look for tasks that can be completed.
// @sma: semaphore array.
// @semnum: semaphore that was modified.
// @wake_q: lockless wake-queue head.
//
// update_queue must be called after a semaphore in a semaphore array
// was modified. If multiple semaphores were modified, update_queue must
// be called with semnum = -1, as well as with the number of each modified
// semaphore.
// The tasks that must be woken up are added to @wake_q. The return code
// is stored in q->pid.
// The function internally checks if const operations can now succeed.
//
// The function return 1 if at least one semop was completed successfully.
//
#[no_mangle]
unsafe extern "C" fn update_queue(sma: *mut sem_array, semnum: c_int, wake_q: *mut wake_q_head) -> c_int {
    let mut q = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
pub static mut pending_list: *mut c_void = core::ptr::null_mut();
pub static mut semop_completed: c_int = 0;
    if (semnum == -1) {
    pending_list = &sma.pending_alter;
    }
    else {
    pending_list = &sma.sems[semnum].pending_alter;
    }
    // label: again
    if false {
    let mut error = 0;
    let mut restart = 0;
// If we are scanning the single sop, per-semaphore list of
// one semaphore and that semaphore is 0, then it is not
// necessary to scan further: simple increments
// that affect only one entry succeed immediately and cannot
// be in the  per semaphore pending queue, and decrements
// cannot be successful if the value is already 0.
//
    if (semnum != -1 && sma.sems[semnum].semval == 0) {
    break;
    }
    error = perform_atomic_semop(sma, q);
// Does q->sleeper still need to sleep?
    if (error > 0) {
    continue;
    }
    unlink_queue(sma, q);
    if (error) {
    restart = 0;
    } else {
    semop_completed = 1;
    do_smart_wakeup_zero(sma, q.sops, q.nsops, wake_q);
    restart = check_restart(sma, q);
    }
    wake_up_sem_queue_prepare(q, error, wake_q);
    if (restart) {
// goto;
    }
    }
    return semop_completed;
    }
//
// set_semotime - set sem_otime
// @sma: semaphore array
// @sops: operations that modified the array, may be NULL
//
// sem_otime is replicated to avoid cache line trashing.
// This function sets one instance to the current time.
//
#[no_mangle]
unsafe extern "C" fn set_semotime(sma: *mut sem_array, sops: *mut sembuf) {
    if (sops == core::ptr::null_mut()) {
    sma.sems[0].sem_otime = ktime_get_real_seconds();
    } else {
    sma.sems[sops[0].sem_num].sem_otime =
    ktime_get_real_seconds();
    }
    }
//
// do_smart_update - optimized update_queue
// @sma: semaphore array
// @sops: operations that were performed
// @nsops: number of operations
// @otime: force setting otime
// @wake_q: lockless wake-queue head
//
// do_smart_update() does the required calls to update_queue and wakeup_zero,
// based on the actual changes that were performed on the semaphore array.
// Note that the function does not do the actual wake-up: the caller is
// responsible for calling wake_up_q().
// It is safe to perform this call after dropping all locks.
//
#[no_mangle]
pub unsafe extern "C" fn do_smart_update(sma: *mut sem_array, sops: *mut sembuf, nsops: c_int, otime: c_int, wake_q: *mut wake_q_head) {
    let mut i = 0;
    otime |= do_smart_wakeup_zero(sma, sops, nsops, wake_q);
    if (!list_empty(&sma.pending_alter)) {
// semaphore array uses the global queue - just process it.
    otime |= update_queue(sma, -1, wake_q);
    } else {
    if (!sops) {
//
// No sops, thus the modified semaphores are not
// known. Check all.
//
    while (i < sma.sem_nsems) {
    otime |= update_queue(sma, i, wake_q);
    }
    } else {
//
// Check the semaphores that were increased:
// - No complex ops, thus all sleeping ops are
// decrease.
// - if we decreased the value, then any sleeping
// semaphore ops won't be able to run: If the
// previous value was too small, then the new
// value will be too small, too.
//
    while (i < nsops) {
    if (sops[i].sem_op > 0) {
    otime |= update_queue(sma,
    sops[i].sem_num, wake_q);
    }
    }
    }
    }
    if (otime) {
    set_semotime(sma, sops);
    }
    }
//
// check_qop: Test if a queued operation sleeps on the semaphore semnum
//
#[no_mangle]
pub unsafe extern "C" fn check_qop(sma: *mut sem_array, semnum: c_int, q: *mut sem_queue, count_zero: bool) -> c_int {
    let mut sop = core::ptr::null_mut();
//
// Linux always (since 0.99.10) reported a task as sleeping on all
// semaphores. This violates SUS, therefore it was changed to the
// standard compliant behavior.
// Give the administrators a chance to notice that an application
// might misbehave because it relies on the Linux behavior.
//
    pr_info_once!("semctl(GETNCNT/GETZCNT) is since 3.16 Single Unix Specification compliant.\nThe task %s (%d) triggered the difference, watch for misbehavior.\n",
    current.comm, task_pid_nr(current));
    if (sop.sem_num != semnum) {
    return 0;
    }
    if (count_zero && sop.sem_op == 0) {
    return 1;
    }
    if (!count_zero && sop.sem_op < 0) {
    return 1;
    }
    return 0;
    }
// The following counts are associated to each semaphore:
// semncnt        number of tasks waiting on semval being nonzero
// semzcnt        number of tasks waiting on semval being zero
//
// Per definition, a task waits only on the semaphore of the first semop
// that cannot proceed, even if additional operation would block, too.
//
#[no_mangle]
pub unsafe extern "C" fn count_semcnt(sma: *mut sem_array, semnum: c_ushort, count_zero: bool) -> c_int {
pub static mut l: *mut c_void = core::ptr::null_mut();
pub static mut q: *mut c_void = core::ptr::null_mut();
    let mut semcnt = 0;
    semcnt = 0;
// First: check the simple operations. They are easy to evaluate
    if (count_zero) {
    l = &sma.sems[semnum].pending_const;
    }
    else {
    l = &sma.sems[semnum].pending_alter;
    }
    if false {
// all task on a per-semaphore list sleep on exactly
// that semaphore
//
    semcnt += 1;
    }
// Then: check the complex operations.
    if false {
    semcnt += check_qop(sma, semnum, q, count_zero);
    }
    if (count_zero) {
    if false {
    semcnt += check_qop(sma, semnum, q, count_zero);
    }
    }
    return semcnt;
    }
// Free a semaphore set. freeary() is called with sem_ids.rwsem locked
// as a writer and the spinlock for this semaphore set hold. sem_ids.rwsem
// remains locked on exit.
//
#[no_mangle]
unsafe extern "C" fn freeary(ns: *mut ipc_namespace, ipcp: *mut kern_ipc_perm) {
    let mut un = core::ptr::null_mut();
    let mut tu = core::ptr::null_mut();
    let mut q = core::ptr::null_mut();
    let mut tq = core::ptr::null_mut();
    let mut sma = container_of!(ipcp, sem_array, sem_perm);
    let mut i = 0;
pub static mut wake_q: usize = 0;
// Free the existing undo structures for this semaphore set.
    ipc_assert_locked_object(&sma.sem_perm);
    if false {
    list_del(&un.list_id);
    spin_lock(&un.ulp.lock);
    un.semid = -1;
    list_del_rcu(&un.list_proc);
    spin_unlock(&un.ulp.lock);
    kvfree_rcu(un, rcu);
    }
// Wake up all pending processes and let them fail with EIDRM.
    if false {
    unlink_queue(sma, q);
    wake_up_sem_queue_prepare(q, -EIDRM, &wake_q);
    }
    if false {
    unlink_queue(sma, q);
    wake_up_sem_queue_prepare(q, -EIDRM, &wake_q);
    }
    while (i < sma.sem_nsems) {
    let mut sem = core::ptr::null_mut();
    if false {
    unlink_queue(sma, q);
    wake_up_sem_queue_prepare(q, -EIDRM, &wake_q);
    }
    if false {
    unlink_queue(sma, q);
    wake_up_sem_queue_prepare(q, -EIDRM, &wake_q);
    }
    ipc_update_pid(&sem.sempid, core::ptr::null_mut());
    }
// Remove the semaphore set from the IDR
    sem_rmid(ns, sma);
    sem_unlock(sma, -1);
    rcu_read_unlock();
    wake_up_q(&wake_q);
    ns.used_sems -= sma.sem_nsems;
    ipc_rcu_putref(&sma.sem_perm, sem_rcu_free);
    }
#[no_mangle]
unsafe extern "C" fn copy_semid_to_user(buf: *mut c_void , r#in: *mut semid64_ds, version: c_int) -> c_ulong {
    match (version) {
    IPC_64 => {
    return copy_to_user(buf, r#in, sizeof!(*r#in));
    }
    IPC_OLD => {
    {
pub static mut out: usize = 0;
    memset(&out, 0, sizeof!(out));
    ipc64_perm_to_ipc_perm(&r#in.sem_perm, &out.sem_perm);
    out.sem_otime	= r#in.sem_otime;
    out.sem_ctime	= r#in.sem_ctime;
    out.sem_nsems	= r#in.sem_nsems;
    return copy_to_user(buf, &out, sizeof!(out));
    }
    }
    _ => {
    return -EINVAL;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn get_semotime(sma: *mut sem_array) -> time64_t {
    let mut i = 0;
    let mut res;
    res = sma.sems[0].sem_otime;
    while (i < sma.sem_nsems) {
pub static mut to: time64_t = 0;
    if (to > res) {
    res = to;
    }
    }
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn semctl_stat(ns: *mut ipc_namespace, semid: c_int, cmd: c_int, semid64: *mut semid64_ds) -> c_int {
pub static mut sma: *mut c_void = core::ptr::null_mut();
    let mut semotime;
    let mut err = 0;
    memset(semid64, 0, sizeof!(*semid64));
    rcu_read_lock();
    if (cmd == SEM_STAT || cmd == SEM_STAT_ANY) {
    sma = sem_obtain_object(ns, semid);
    if (IS_ERR(sma)) {
    err = PTR_ERR(sma);
// goto;
    }
    } else { /* IPC_STAT */
    sma = sem_obtain_object_check(ns, semid);
    if (IS_ERR(sma)) {
    err = PTR_ERR(sma);
// goto;
    }
    }
// see comment for SHM_STAT_ANY
    if (cmd == SEM_STAT_ANY) {
    audit_ipc_obj(&sma.sem_perm);
    }
    else {
    err = -EACCES;
    if (ipcperms(ns, &sma.sem_perm, S_IRUGO)) {
// goto;
    }
    }
    err = security_sem_semctl(&sma.sem_perm, cmd);
    if (err) {
// goto;
    }
    ipc_lock_object(&sma.sem_perm);
    if (!ipc_valid_object(&sma.sem_perm)) {
    ipc_unlock_object(&sma.sem_perm);
    err = -EIDRM;
// goto;
    }
    kernel_to_ipc64_perm(&sma.sem_perm, &semid64.sem_perm);
    semotime = get_semotime(sma);
    semid64.sem_otime = semotime;
    semid64.sem_ctime = sma.sem_ctime;

    semid64.sem_otime_high = semotime >> 32;
    semid64.sem_ctime_high = sma.sem_ctime >> 32;

    semid64.sem_nsems = sma.sem_nsems;
    if (cmd == IPC_STAT) {
//
// As defined in SUS:
// Return 0 on success
//
    err = 0;
    } else {
//
// SEM_STAT and SEM_STAT_ANY (both Linux specific)
// Return the full id, including the sequence number
//
    err = sma.sem_perm.id;
    }
    ipc_unlock_object(&sma.sem_perm);
    // label: out_unlock
    rcu_read_unlock();
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn semctl_info(ns: *mut ipc_namespace, semid: c_int, cmd: c_int, p: *mut c_void) -> c_int {
pub static mut seminfo: usize = 0;
    let mut max_idx = 0;
    let mut err = 0;
    err = security_sem_semctl(core::ptr::null_mut(), cmd);
    if (err) {
    return err;
    }
    memset(&seminfo, 0, sizeof!(seminfo));
    seminfo.semmni = ns.sc_semmni;
    seminfo.semmns = ns.sc_semmns;
    seminfo.semmsl = ns.sc_semmsl;
    seminfo.semopm = ns.sc_semopm;
    seminfo.semvmx = SEMVMX;
    seminfo.semmnu = SEMMNU;
    seminfo.semmap = SEMMAP;
    seminfo.semume = SEMUME;
    down_read(&sem_ids(ns).rwsem);
    if (cmd == SEM_INFO) {
    seminfo.semusz = sem_ids(ns).in_use;
    seminfo.semaem = ns.used_sems;
    } else {
    seminfo.semusz = SEMUSZ;
    seminfo.semaem = SEMAEM;
    }
    max_idx = ipc_get_maxidx(&sem_ids(ns));
    up_read(&sem_ids(ns).rwsem);
    if (copy_to_user(p, &seminfo, sizeof!(seminfo))) {
    return -EFAULT;
    }
    return if (max_idx < 0) { 0 } else { max_idx };
    }
#[no_mangle]
pub unsafe extern "C" fn semctl_setval(ns: *mut ipc_namespace, semid: c_int, semnum: c_int, val: c_int) -> c_int {
pub static mut un: *mut c_void = core::ptr::null_mut();
pub static mut sma: *mut c_void = core::ptr::null_mut();
pub static mut curr: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
pub static mut wake_q: usize = 0;
    if (val > SEMVMX || val < 0) {
    return -ERANGE;
    }
    rcu_read_lock();
    sma = sem_obtain_object_check(ns, semid);
    if (IS_ERR(sma)) {
    rcu_read_unlock();
    return PTR_ERR(sma);
    }
    if (semnum < 0 || semnum >= sma.sem_nsems) {
    rcu_read_unlock();
    return -EINVAL;
    }
    if (ipcperms(ns, &sma.sem_perm, S_IWUGO)) {
    rcu_read_unlock();
    return -EACCES;
    }
    err = security_sem_semctl(&sma.sem_perm, SETVAL);
    if (err) {
    rcu_read_unlock();
    return -EACCES;
    }
    sem_lock(sma, core::ptr::null_mut(), -1);
    if (!ipc_valid_object(&sma.sem_perm)) {
    sem_unlock(sma, -1);
    rcu_read_unlock();
    return -EIDRM;
    }
    semnum = array_index_nospec(semnum, sma.sem_nsems);
    curr = &sma.sems[semnum];
    ipc_assert_locked_object(&sma.sem_perm);
    if false {
    rcu_read_unlock();
    return PTR_ERR(sma);
    }
    nsems = sma.sem_nsems;
    err = -EACCES;
    if (ipcperms(ns, &sma.sem_perm, (if cmd == SETALL { S_IWUGO } else { S_IRUGO }))) {
// goto;
    }
    err = security_sem_semctl(&sma.sem_perm, cmd);
    if (err) {
// goto;
    }
    match (cmd) {
    GETALL => {
    {
    let mut array = core::ptr::null_mut();
    let mut i = 0;
    sem_lock(sma, core::ptr::null_mut(), -1);
    if (!ipc_valid_object(&sma.sem_perm)) {
    err = -EIDRM;
// goto;
    }
    if (nsems > SEMMSL_FAST) {
    if (!ipc_rcu_getref(&sma.sem_perm)) {
    err = -EIDRM;
// goto;
    }
    sem_unlock(sma, -1);
    rcu_read_unlock();
    sem_io = kvmalloc_array(nsems, sizeof!(ushort),
    GFP_KERNEL);
    if (sem_io == core::ptr::null_mut()) {
    ipc_rcu_putref(&sma.sem_perm, sem_rcu_free);
    return -ENOMEM;
    }
    rcu_read_lock();
    sem_lock_and_putref(sma);
    if (!ipc_valid_object(&sma.sem_perm)) {
    err = -EIDRM;
// goto;
    }
    }
    while (i < sma.sem_nsems) {
    sem_io[i] = sma.sems[i].semval;
    }
    sem_unlock(sma, -1);
    rcu_read_unlock();
    err = 0;
    if (copy_to_user(array, sem_io, nsems*sizeof!(ushort))) {
    err = -EFAULT;
    }
// goto;
    }
    }
    SETALL => {
    {
    let mut i = 0;
pub static mut un: *mut c_void = core::ptr::null_mut();
    if (!ipc_rcu_getref(&sma.sem_perm)) {
    err = -EIDRM;
// goto;
    }
    rcu_read_unlock();
    if (nsems > SEMMSL_FAST) {
    sem_io = kvmalloc_array(nsems, sizeof!(ushort),
    GFP_KERNEL);
    if (sem_io == core::ptr::null_mut()) {
    ipc_rcu_putref(&sma.sem_perm, sem_rcu_free);
    return -ENOMEM;
    }
    }
    if (copy_from_user(sem_io, p, nsems*sizeof!(ushort))) {
    ipc_rcu_putref(&sma.sem_perm, sem_rcu_free);
    err = -EFAULT;
// goto;
    }
    while (i < nsems) {
    if (sem_io[i] > SEMVMX) {
    ipc_rcu_putref(&sma.sem_perm, sem_rcu_free);
    err = -ERANGE;
// goto;
    }
    }
    rcu_read_lock();
    sem_lock_and_putref(sma);
    if (!ipc_valid_object(&sma.sem_perm)) {
    err = -EIDRM;
// goto;
    }
    while (i < nsems) {
    sma.sems[i].semval = sem_io[i];
    ipc_update_pid(&sma.sems[i].sempid, task_tgid(current));
    }
    ipc_assert_locked_object(&sma.sem_perm);
    if false {
    while (i < nsems) {
    un.semadj[i] = 0;
    }
    }
    sma.sem_ctime = ktime_get_real_seconds();
// maybe some queued-up processes were waiting for this
    do_smart_update(sma, core::ptr::null_mut(), 0, 0, &wake_q);
    err = 0;
// goto;
    }
// GETVAL, GETPID, GETNCTN, GETZCNT: fall-through
    }
    }
    err = -EINVAL;
    if (semnum < 0 || semnum >= nsems) {
// goto;
    }
    sem_lock(sma, core::ptr::null_mut(), -1);
    if (!ipc_valid_object(&sma.sem_perm)) {
    err = -EIDRM;
// goto;
    }
    semnum = array_index_nospec(semnum, nsems);
    curr = &sma.sems[semnum];
    match (cmd) {
    GETVAL => {
    err = curr.semval;
// goto;
    }
    GETPID => {
    err = pid_vnr(curr.sempid);
// goto;
    }
    GETNCNT => {
    err = count_semcnt(sma, semnum, 0);
// goto;
    }
    GETZCNT => {
    err = count_semcnt(sma, semnum, 1);
// goto;
    }
    }
    // label: out_unlock
    sem_unlock(sma, -1);
    // label: out_rcu_wakeup
    rcu_read_unlock();
    wake_up_q(&wake_q);
    // label: out_free
    if (sem_io != fast_sem_io) {
    kvfree(sem_io);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn copy_semid_from_user(out: *mut semid64_ds, buf: *mut c_void, version: c_int) -> c_ulong {
    match (version) {
    IPC_64 => {
    if (copy_from_user(out, buf, sizeof!(*out))) {
    return -EFAULT;
    }
    return 0;
    }
    IPC_OLD => {
    {
pub static mut tbuf_old: usize = 0;
    if (copy_from_user(&tbuf_old, buf, sizeof!(tbuf_old))) {
    return -EFAULT;
    }
    out.sem_perm.uid	= tbuf_old.sem_perm.uid;
    out.sem_perm.gid	= tbuf_old.sem_perm.gid;
    out.sem_perm.mode	= tbuf_old.sem_perm.mode;
    return 0;
    }
    }
    _ => {
    return -EINVAL;
    }
    }
    }
//
// This function handles some semctl commands which require the rwsem
// to be held in write mode.
// NOTE: no locks must be held, the rwsem is taken inside this function.
//
#[no_mangle]
pub unsafe extern "C" fn semctl_down(ns: *mut ipc_namespace, semid: c_int, cmd: c_int, semid64: *mut semid64_ds) -> c_int {
pub static mut sma: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
pub static mut ipcp: *mut c_void = core::ptr::null_mut();
    down_write(&sem_ids(ns).rwsem);
    rcu_read_lock();
    ipcp = ipcctl_obtain_check(ns, &sem_ids(ns), semid, cmd,
    &semid64.sem_perm, 0);
    if (IS_ERR(ipcp)) {
    err = PTR_ERR(ipcp);
// goto;
    }
    sma = container_of!(ipcp, sem_array, sem_perm);
    err = security_sem_semctl(&sma.sem_perm, cmd);
    if (err) {
// goto;
    }
    match (cmd) {
    IPC_RMID => {
    sem_lock(sma, core::ptr::null_mut(), -1);
// freeary unlocks the ipc object and rcu
    freeary(ns, ipcp);
// goto;
    }
    IPC_SET => {
    sem_lock(sma, core::ptr::null_mut(), -1);
    err = ipc_update_perm(&semid64.sem_perm, ipcp);
    if (err) {
// goto;
    }
    sma.sem_ctime = ktime_get_real_seconds();
    // break;
    }
    _ => {
    err = -EINVAL;
// goto;
    }
    }
    // label: out_unlock0
    sem_unlock(sma, -1);
    // label: out_unlock1
    rcu_read_unlock();
    // label: out_up
    up_write(&sem_ids(ns).rwsem);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ksys_semctl(semid: c_int, semnum: c_int, cmd: c_int, arg: c_ulong, version: c_int) -> c_long {
pub static mut ns: *mut c_void = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
pub static mut semid64: usize = 0;
    let mut err = 0;
    if (semid < 0) {
    return -EINVAL;
    }
    ns = current.nsproxy.ipc_ns;
    match (cmd) {
    IPC_INFO | SEM_INFO => {
    return semctl_info(ns, semid, cmd, p);
    }
    IPC_STAT | SEM_STAT | SEM_STAT_ANY => {
    err = semctl_stat(ns, semid, cmd, &semid64);
    if (err < 0) {
    return err;
    }
    if (copy_semid_to_user(p, &semid64, version)) {
    err = -EFAULT;
    }
    return err;
    }
    GETALL | GETVAL | GETPID | GETNCNT | GETZCNT | SETALL => {
    return semctl_main(ns, semid, semnum, cmd, p);
     }
     SETVAL => {
     {
    let mut val = 0;

// big-endian 64bit
    val = arg >> 32;

// 32bit or little-endian 64bit
    val = arg;

    return semctl_setval(ns, semid, semnum, val);
    }
    }
    IPC_SET => {
    if (copy_semid_from_user(&semid64, p, version)) {
    return -EFAULT;
    }
    fallthrough;
    }
    IPC_RMID => {
    return semctl_down(ns, semid, cmd, &semid64);
    }
    _ => {
    return -EINVAL;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn sys_semctl(semid: usize, semnum: usize, cmd: usize, arg: usize) -> c_long {
    return ksys_semctl(semid, semnum, cmd, arg, IPC_64);
    }

#[no_mangle]
pub unsafe extern "C" fn ksys_old_semctl(semid: c_int, semnum: c_int, cmd: c_int, arg: c_ulong) -> c_long {
pub static mut version: c_int = 0;
    return ksys_semctl(semid, semnum, cmd, arg, version);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_old_semctl(semid: usize, semnum: usize, cmd: usize, arg: usize) -> c_long {
    return ksys_old_semctl(semid, semnum, cmd, arg);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_semid_ds {
    pub sem_perm: compat_ipc_perm,
    pub sem_otime: old_time32_t,
    pub sem_ctime: old_time32_t,
    pub sem_base: compat_uptr_t,
    pub sem_pending: compat_uptr_t,
    pub sem_pending_last: compat_uptr_t,
    pub undo: compat_uptr_t,
    pub sem_nsems: c_ushort,
}

#[no_mangle]
pub unsafe extern "C" fn copy_compat_semid_from_user(out: *mut semid64_ds, buf: *mut c_void, version: c_int) -> c_int {
    memset(out, 0, sizeof!(*out));
    if (version == IPC_64) {
    let mut p = core::ptr::null_mut();
    return get_compat_ipc64_perm(&out.sem_perm, &p.sem_perm);
    } else {
    let mut p = core::ptr::null_mut();
    return get_compat_ipc_perm(&out.sem_perm, &p.sem_perm);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn copy_compat_semid_to_user(buf: *mut c_void, r#in: *mut semid64_ds, version: c_int) -> c_int {
    if (version == IPC_64) {
pub static mut v: usize = 0;
    memset(&v, 0, sizeof!(v));
    to_compat_ipc64_perm(&v.sem_perm, &r#in.sem_perm);
    v.sem_otime	 = lower_32_bits(r#in.sem_otime);
    v.sem_otime_high = upper_32_bits(r#in.sem_otime);
    v.sem_ctime	 = lower_32_bits(r#in.sem_ctime);
    v.sem_ctime_high = upper_32_bits(r#in.sem_ctime);
    v.sem_nsems = r#in.sem_nsems;
    return copy_to_user(buf, &v, sizeof!(v));
    } else {
pub static mut v: usize = 0;
    memset(&v, 0, sizeof!(v));
    to_compat_ipc_perm(&v.sem_perm, &r#in.sem_perm);
    v.sem_otime = r#in.sem_otime;
    v.sem_ctime = r#in.sem_ctime;
    v.sem_nsems = r#in.sem_nsems;
    return copy_to_user(buf, &v, sizeof!(v));
    }
    }
#[no_mangle]
unsafe extern "C" fn compat_ksys_semctl(semid: c_int, semnum: c_int, cmd: c_int, arg: c_int, version: c_int) -> c_long {
    let mut p = core::ptr::null_mut();
pub static mut ns: *mut c_void = core::ptr::null_mut();
pub static mut semid64: usize = 0;
    let mut err = 0;
    ns = current.nsproxy.ipc_ns;
    if (semid < 0) {
    return -EINVAL;
    }
    match (cmd & (!IPC_64)) {
    IPC_INFO | SEM_INFO => {
    return semctl_info(ns, semid, cmd, p);
    }
    IPC_STAT | SEM_STAT | SEM_STAT_ANY => {
    err = semctl_stat(ns, semid, cmd, &semid64);
    if (err < 0) {
    return err;
    }
    if (copy_compat_semid_to_user(p, &semid64, version)) {
    err = -EFAULT;
    }
    return err;
    }
    GETVAL | GETPID | GETNCNT | GETZCNT | GETALL | SETALL => {
    return semctl_main(ns, semid, semnum, cmd, p);
    }
    SETVAL => {
    return semctl_setval(ns, semid, semnum, arg);
    }
    IPC_SET => {
    if (copy_compat_semid_from_user(&semid64, p, version)) {
    return -EFAULT;
    }
    fallthrough;
    }
    IPC_RMID => {
    return semctl_down(ns, semid, cmd, &semid64);
    }
    _ => {
    return -EINVAL;
    }
    }
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sys_semctl
pub unsafe extern "C" fn sys_semctl_dup(semid: usize, semnum: usize, cmd: usize, arg: usize) -> c_long {
    return compat_ksys_semctl(semid, semnum, cmd, arg, IPC_64);
    }

#[no_mangle]
pub unsafe extern "C" fn compat_ksys_old_semctl(semid: c_int, semnum: c_int, cmd: c_int, arg: c_int) -> c_long {
pub static mut version: c_int = 0;
    return compat_ksys_semctl(semid, semnum, cmd, arg, version);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: sys_old_semctl
pub unsafe extern "C" fn sys_old_semctl_dup(semid: usize, semnum: usize, cmd: usize, arg: usize) -> c_long {
    return compat_ksys_old_semctl(semid, semnum, cmd, arg);
    }

// If the task doesn't already have a undo_list, then allocate one
// here.  We guarantee there is only one thread using this undo list,
// and current is THE ONE
//
// If this allocation and assignment succeeds, but later
// portions of this code fail, there is no need to free the sem_undo_list.
// Just let it stay associated with the task, and it'll be freed later
// at exit time.
//
// This can block, so callers must hold no locks.
//
#[no_mangle]
pub unsafe extern "C" fn get_undo_list(undo_listp: *mut sem_undo_list) -> c_int {
pub static mut undo_list: *mut c_void = core::ptr::null_mut();
    undo_list = current.sysvsem.undo_list;
    if (!undo_list) {
    undo_list = kzalloc_obj(*undo_list, GFP_KERNEL_ACCOUNT);
    if (undo_list == core::ptr::null_mut()) {
    return -ENOMEM;
    }
    spin_lock_init(&undo_list.lock);
    refcount_set(&undo_list.refcnt, 1);
    INIT_LIST_HEAD(&undo_list.list_proc);
    current.sysvsem.undo_list = undo_list;
    }
// undo_listp = undo_list;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __lookup_undo(ulp: *mut sem_undo_list, semid: c_int) -> *mut c_void {
pub static mut un: *mut c_void = core::ptr::null_mut();
    if false {
    if (un.semid == semid) {
    return un;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn lookup_undo(ulp: *mut sem_undo_list, semid: c_int) -> *mut c_void {
pub static mut un: *mut c_void = core::ptr::null_mut();
    assert_spin_locked(&ulp.lock);
    un = __lookup_undo(ulp, semid);
    if (un) {
    list_del_rcu(&un.list_proc);
    list_add_rcu(&un.list_proc, &ulp.list_proc);
    }
    return un;
    }
//
// find_alloc_undo - lookup (and if not present create) undo array
// @ns: namespace
// @semid: semaphore array id
//
// The function looks up (and if not present creates) the undo structure.
// The size of the undo structure depends on the size of the semaphore
// array, thus the alloc path is not that straightforward.
// Lifetime-rules: sem_undo is rcu-protected, on success, the function
// performs a rcu_read_lock().
//
#[no_mangle]
pub unsafe extern "C" fn find_alloc_undo(ns: *mut ipc_namespace, semid: c_int) -> *mut c_void {
pub static mut sma: *mut c_void = core::ptr::null_mut();
pub static mut ulp: *mut c_void = core::ptr::null_mut();
    let mut un = core::ptr::null_mut();
    let mut new = core::ptr::null_mut();
    let mut nsems = 0;
    let mut error = 0;
    error = get_undo_list(&ulp);
    if (error) {
    return ERR_PTR(error);
    }
    rcu_read_lock();
    spin_lock(&ulp.lock);
    un = lookup_undo(ulp, semid);
    spin_unlock(&ulp.lock);
    if (likely(un != core::ptr::null_mut())) {
// goto;
    }
// no undo structure around - allocate one.
// step 1: figure out the size of the semaphore array
    sma = sem_obtain_object_check(ns, semid);
    if (IS_ERR(sma)) {
    rcu_read_unlock();
    return ERR_CAST(sma);
    }
    nsems = sma.sem_nsems;
    if (!ipc_rcu_getref(&sma.sem_perm)) {
    rcu_read_unlock();
    un = ERR_PTR(-EIDRM);
// goto;
    }
    rcu_read_unlock();
// step 2: allocate new undo structure
    new = kvzalloc_flex(*new, semadj, nsems, GFP_KERNEL_ACCOUNT);
    if (!new) {
    ipc_rcu_putref(&sma.sem_perm, sem_rcu_free);
    return ERR_PTR(-ENOMEM);
    }
// step 3: Acquire the lock on semaphore array
    rcu_read_lock();
    sem_lock_and_putref(sma);
    if (!ipc_valid_object(&sma.sem_perm)) {
    sem_unlock(sma, -1);
    rcu_read_unlock();
    kvfree(new);
    un = ERR_PTR(-EIDRM);
// goto;
    }
    spin_lock(&ulp.lock);
//
// step 4: check for races: did someone else allocate the undo struct?
//
    un = lookup_undo(ulp, semid);
    if (un) {
    spin_unlock(&ulp.lock);
    kvfree(new);
// goto;
    }
// step 5: initialize & link new undo structure
    new.ulp = ulp;
    new.semid = semid;
    assert_spin_locked(&ulp.lock);
    list_add_rcu(&new.list_proc, &ulp.list_proc);
    ipc_assert_locked_object(&sma.sem_perm);
    list_add(&new.list_id, &sma.list_id);
    un = new;
    spin_unlock(&ulp.lock);
    // label: success
    sem_unlock(sma, -1);
    // label: out
    return un;
    }
#[no_mangle]
pub unsafe extern "C" fn __do_semtimedop(semid: c_int, sops: *mut sembuf, nsops: c_uint, timeout: *mut timespec64, ns: *mut ipc_namespace) -> c_long {
pub static mut error: c_int = 0;
pub static mut sma: *mut c_void = core::ptr::null_mut();
pub static mut sop: *mut c_void = core::ptr::null_mut();
pub static mut un: *mut c_void = core::ptr::null_mut();
    let mut max = 0;
    let mut locknum = 0;
pub static mut undos: bool = false;
pub static mut queue: usize = 0;
pub static mut dup: c_ulong = 0;
pub static mut expires: usize = 0;
    let mut exp = core::ptr::null_mut();
pub static mut timed_out: bool = false;
    if (nsops < 1 || semid < 0) {
    return -EINVAL;
    }
    if (nsops > ns.sc_semopm) {
    return -E2BIG;
    }
    if (timeout) {
    if (!timespec64_valid(timeout)) {
    return -EINVAL;
    }
    expires = ktime_add_safe(ktime_get(),
    timespec64_to_ktime(*timeout));
    exp = &expires;
    }
    max = 0;
    while (sop < sops + nsops) {
pub static mut mask: c_ulong = 0;
    if (sop.sem_num >= max) {
    max = sop.sem_num;
    }
    if (sop.sem_flg & SEM_UNDO) {
    undos = true;
    }
    if (dup & mask) {
//
// There was a previous alter access that appears
// to have accessed the same semaphore, thus use
// the dupsop logic. "appears", because the detection
// can only check % BITS_PER_LONG.
//
    dupsop = true;
    }
    if (sop.sem_op != 0) {
    alter = true;
    dup |= mask;
    }
    }
    if (undos) {
// On success, find_alloc_undo takes the rcu_read_lock
    un = find_alloc_undo(ns, semid);
    if (IS_ERR(un)) {
    error = PTR_ERR(un);
// goto;
    }
    } else {
    un = core::ptr::null_mut();
    rcu_read_lock();
    }
    sma = sem_obtain_object_check(ns, semid);
    if (IS_ERR(sma)) {
    rcu_read_unlock();
    error = PTR_ERR(sma);
// goto;
    }
    error = -EFBIG;
    if (max >= sma.sem_nsems) {
    rcu_read_unlock();
// goto;
    }
    error = -EACCES;
    if (ipcperms(ns, &sma.sem_perm, (if alter { S_IWUGO } else { S_IRUGO }))) {
    rcu_read_unlock();
// goto;
    }
    error = security_sem_semop(&sma.sem_perm, sops, nsops, alter);
    if (error) {
    rcu_read_unlock();
// goto;
    }
    error = -EIDRM;
    locknum = sem_lock(sma, sops, nsops);
//
// We eventually might perform the following check in a lockless
// fashion, considering ipc_valid_object() locking constraints.
// If nsops == 1 and there is no contention for sem_perm.lock, then
// only a per-semaphore lock is held and it's OK to proceed with the
// check below. More details on the fine grained locking scheme
// entangled here and why it's RMID race safe on comments at sem_lock()
//
    if (!ipc_valid_object(&sma.sem_perm)) {
// goto;
    }
//
// semid identifiers are not unique - find_alloc_undo may have
// allocated an undo structure, it was invalidated by an RMID
// and now a new array with received the same id. Check and fail.
// This case can be detected checking un->semid. The existence of
// "un" itself is guaranteed by rcu.
//
    if (un && un.semid == -1) {
// goto;
    }
    queue.sops = sops;
    queue.nsops = nsops;
    queue.undo = un;
    queue.pid = task_tgid(current);
    queue.alter = alter;
    queue.dupsop = dupsop;
    error = perform_atomic_semop(sma, &queue);
    if (error == 0) { /* non-blocking successful path */ {
pub static mut wake_q: usize = 0;
    }
//
// If the operation was successful, then do
// the required updates.
//
    if (alter) {
    do_smart_update(sma, sops, nsops, 1, &wake_q);
    }
    else {
    set_semotime(sma, sops);
    }
    sem_unlock(sma, locknum);
    rcu_read_unlock();
    wake_up_q(&wake_q);
// goto;
    }
    if (error < 0) /* non-blocking error path */ {
// goto;
    }
//
// We need to sleep on this operation, so we put the current
// task into the pending queue and go to sleep.
//
    if (nsops == 1) {
pub static mut curr: *mut c_void = core::ptr::null_mut();
pub static mut idx: c_int = 0;
    curr = &sma.sems[idx];
    if (alter) {
    if (sma.complex_count) {
    list_add_tail(&queue.list,
    &sma.pending_alter);
    } else {
    list_add_tail(&queue.list,
    &curr.pending_alter);
    }
    } else {
    list_add_tail(&queue.list, &curr.pending_const);
    }
    } else {
    if (!sma.complex_count) {
    merge_queues(sma);
    }
    if (alter) {
    list_add_tail(&queue.list, &sma.pending_alter);
    }
    else {
    list_add_tail(&queue.list, &sma.pending_const);
    }
    sma.complex_count += 1;
    }
    loop {
// memory ordering ensured by the lock in sem_lock()
    WRITE_ONCE(queue.status, -EINTR);
    queue.sleeper = current;
// memory ordering is ensured by the lock in sem_lock()
    __set_current_state(TASK_INTERRUPTIBLE);
    sem_unlock(sma, locknum);
    rcu_read_unlock();
    timed_out = !schedule_hrtimeout_range(exp,
    current.timer_slack_ns, HRTIMER_MODE_ABS);
//
// fastpath: the semop has completed, either successfully or
// not, from the syscall pov, is quite irrelevant to us at this
// point; we're done.
//
// We _do_ care, nonetheless, about being awoken by a signal or
// spuriously.  The queue.status is checked again in the
// slowpath (aka after taking sem_lock), such that we can detect
// scenarios where we were awakened externally, during the
// window between wake_q_add() and wake_up_q().
//
    rcu_read_lock();
    error = READ_ONCE(queue.status);
    if (error != -EINTR) {
// see SEM_BARRIER_2 for purpose/pairing
    smp_acquire__after_ctrl_dep();
    rcu_read_unlock();
// goto;
    }
    locknum = sem_lock(sma, sops, nsops);
    if (!ipc_valid_object(&sma.sem_perm)) {
// goto;
    }
//
// No necessity for any barrier: We are protect by sem_lock()
//
    error = READ_ONCE(queue.status);
//
// If queue.status != -EINTR we are woken up by another process.
// Leave without unlink_queue(), but with sem_unlock().
//
    if (error != -EINTR) {
// goto;
    }
//
// If an interrupt occurred we have to clean up the queue.
//
    if (timed_out) {
    error = -EAGAIN;
    }
    break; } /* spurious */
    unlink_queue(sma, &queue);
    // label: out_unlock
    sem_unlock(sma, locknum);
    rcu_read_unlock();
    // label: out
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn do_semtimedop(semid: c_int, tsops: *mut sembuf, nsops: c_uint, timeout: *mut timespec64) -> c_long {
    let mut fast_sops: [sembuf; 0] = [];
    let mut sops = core::ptr::null_mut();
pub static mut ns: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ns = current.nsproxy.ipc_ns;
    if (nsops > ns.sc_semopm) {
    return -E2BIG;
    }
    if (nsops < 1) {
    return -EINVAL;
    }
    if (nsops > SEMOPM_FAST) {
    sops = kvmalloc_objs(*sops, nsops);
    if (sops == core::ptr::null_mut()) {
    return -ENOMEM;
    }
    }
    if (copy_from_user(sops, tsops, nsops * sizeof!(*tsops))) {
    ret =  -EFAULT;
// goto;
    }
    ret = __do_semtimedop(semid, sops, nsops, timeout, ns);
    // label: out_free
    if (sops != fast_sops) {
    kvfree(sops);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ksys_semtimedop(semid: c_int, tsops: *mut sembuf, nsops: c_uint, timeout: *mut __kernel_timespec) -> c_long {
    if (timeout) {
pub static mut ts: usize = 0;
    if (get_timespec64(&ts, timeout)) {
    return -EFAULT;
    }
    return do_semtimedop(semid, tsops, nsops, &ts);
    }
    return do_semtimedop(semid, tsops, nsops, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn sys_semtimedop(semid: usize, tsops: usize, nsops: usize, timeout: usize) -> c_long {
    return ksys_semtimedop(semid, tsops, nsops, timeout);
    }

#[no_mangle]
pub unsafe extern "C" fn compat_ksys_semtimedop(semid: c_int, tsems: *mut sembuf, nsops: c_uint, timeout: *mut old_timespec32) -> c_long {
    if (timeout) {
pub static mut ts: usize = 0;
    if (get_old_timespec32(&ts, timeout)) {
    return -EFAULT;
    }
    return do_semtimedop(semid, tsems, nsops, &ts);
    }
    return do_semtimedop(semid, tsems, nsops, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn sys_semtimedop_time32(semid: usize, tsems: usize, nsops: usize, timeout: usize) -> c_long {
    return compat_ksys_semtimedop(semid, tsems, nsops, timeout);
    }

#[no_mangle]
pub unsafe extern "C" fn sys_semop(semid: usize, tsops: usize, nsops: usize) -> c_long {
    return do_semtimedop(semid, tsops, nsops, core::ptr::null_mut());
    }
// If CLONE_SYSVSEM is set, establish sharing of SEM_UNDO state between
// parent and child tasks.
//
#[no_mangle]
pub unsafe extern "C" fn copy_semundo(clone_flags: u64, tsk: *mut task_struct) -> c_int {
pub static mut undo_list: *mut c_void = core::ptr::null_mut();
    let mut error = 0;
    if (clone_flags & CLONE_SYSVSEM) {
    error = get_undo_list(&undo_list);
    if (error) {
    return error;
    }
    refcount_inc(&undo_list.refcnt);
    tsk.sysvsem.undo_list = undo_list;
    } else {
    tsk.sysvsem.undo_list = core::ptr::null_mut();
    }
    return 0;
    }
//
// add semadj values to semaphores, free undo structures.
// undo structures are not freed when semaphore arrays are destroyed
// so some of them may be out of date.
// IMPLEMENTATION NOTE: There is some confusion over whether the
// set of adjustments that needs to be done should be done in an atomic
// manner or not. That is, if we are attempting to decrement the semval
// should we queue up and wait until we can do so legally?
// The original implementation attempted to do this (queue and wait).
// The current implementation does not do so. The POSIX standard
// and SVID should be consulted to determine what behavior is mandated.
//
#[no_mangle]
pub unsafe extern "C" fn exit_sem(tsk: *mut task_struct) {
pub static mut ulp: *mut c_void = core::ptr::null_mut();
    ulp = tsk.sysvsem.undo_list;
    if (!ulp) {
    return;
    }
    tsk.sysvsem.undo_list = core::ptr::null_mut();
    if (!refcount_dec_and_test(&ulp.refcnt)) {
    return;
    }
    loop {
pub static mut sma: *mut c_void = core::ptr::null_mut();
pub static mut un: *mut c_void = core::ptr::null_mut();
    let mut semid = 0;
    let mut i = 0;
pub static mut wake_q: usize = 0;
    cond_resched();
    rcu_read_lock();
    un = list_entry_rcu(ulp.list_proc.next, sem_undo, list_proc);
    if (&un.list_proc == &ulp.list_proc) {
//
// We must wait for freeary() before freeing this ulp,
// in case we raced with last sem_undo. There is a small
// possibility where we exit while freeary() didn't
// finish unlocking sem_undo_list.
//
    spin_lock(&ulp.lock);
    spin_unlock(&ulp.lock);
    rcu_read_unlock();
    break;
    }
    spin_lock(&ulp.lock);
    semid = un.semid;
    spin_unlock(&ulp.lock);
// exit_sem raced with IPC_RMID, nothing to do
    if (semid == -1) {
    rcu_read_unlock();
    continue;
    }
    sma = sem_obtain_object_check(tsk.nsproxy.ipc_ns, semid);
// exit_sem raced with IPC_RMID, nothing to do
    if (IS_ERR(sma)) {
    rcu_read_unlock();
    continue;
    }
    sem_lock(sma, core::ptr::null_mut(), -1);
// exit_sem raced with IPC_RMID, nothing to do
    if (!ipc_valid_object(&sma.sem_perm)) {
    sem_unlock(sma, -1);
    rcu_read_unlock();
    continue;
    }
    un = __lookup_undo(ulp, semid);
    if (un == core::ptr::null_mut()) {
// exit_sem raced with IPC_RMID+semget() that created
// exactly the same semid. Nothing to do.
//
    sem_unlock(sma, -1);
    rcu_read_unlock();
    continue;
    }
// remove un from the linked lists
    ipc_assert_locked_object(&sma.sem_perm);
    list_del(&un.list_id);
    spin_lock(&ulp.lock);
    list_del_rcu(&un.list_proc);
    spin_unlock(&ulp.lock);
// perform adjustments registered in un
    while (i < sma.sem_nsems) {
    let mut semaphore = core::ptr::null_mut();
    if (un.semadj[i]) {
    semaphore.semval += un.semadj[i];
//
// Range checks of the new semaphore value,
// not defined by sus:
// - Some unices ignore the undo entirely
// (e.g. HP UX 11i 11.22, Tru64 V5.1)
// - some cap the value (e.g. FreeBSD caps
// at 0, but doesn't enforce SEMVMX)
//
// Linux caps the semaphore value, both at 0
// and at SEMVMX.
//
// Manfred <manfred@colorfullife.com>
//
    if (semaphore.semval < 0) {
    semaphore.semval = 0;
    }
    if (semaphore.semval > SEMVMX) {
    semaphore.semval = SEMVMX;
    }
    ipc_update_pid(&semaphore.sempid, task_tgid(current));
    }
    }
// maybe some queued-up processes were waiting for this
    do_smart_update(sma, core::ptr::null_mut(), 0, 1, &wake_q);
    sem_unlock(sma, -1);
    rcu_read_unlock();
    wake_up_q(&wake_q);
    kvfree_rcu(un, rcu);
    }
    kfree(ulp);
    }

#[no_mangle]
unsafe extern "C" fn sysvipc_sem_proc_show(s: *mut seq_file, it: *mut c_void) -> c_int {
    let mut user_ns = core::ptr::null_mut();
    let mut ipcp = core::ptr::null_mut();
    let mut sma = container_of!(ipcp, sem_array, sem_perm);
    let mut sem_otime;
//
// The proc interface isn't aware of sem_lock(), it calls
// ipc_lock_object(), i.e. spin_lock(&sma->sem_perm.lock).
// (in sysvipc_find_ipc)
// In order to stay compatible with sem_lock(), we must
// enter / leave complex_mode.
//
    complexmode_enter(sma);
    sem_otime = get_semotime(sma);
    seq_printf(s,
    "%10d %10d  %4o %10u %5u %5u %5u %5u %10llu %10llu\n",
    sma.sem_perm.key,
    sma.sem_perm.id,
    sma.sem_perm.mode,
    sma.sem_nsems,
    from_kuid_munged(user_ns, sma.sem_perm.uid),
    from_kgid_munged(user_ns, sma.sem_perm.gid),
    from_kuid_munged(user_ns, sma.sem_perm.cuid),
    from_kgid_munged(user_ns, sma.sem_perm.cgid),
    sem_otime,
    sma.sem_ctime);
    complexmode_tryleave(sma);
    return 0;
    }
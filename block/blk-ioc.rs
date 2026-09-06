//! Automatically rewritten from C to Rust
//! Source: block/blk-ioc.c
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
// Functions related to io context handling
//

//
// For io context allocations
//
pub static mut iocontext_cachep: *mut c_void = core::ptr::null_mut();

//
// get_io_context - increment reference count to io_context
// @ioc: io_context to get
//
// Increment reference count to @ioc.
//
#[no_mangle]
unsafe extern "C" fn get_io_context(ioc: *mut io_context) {
    BUG_ON!(atomic_long_read(&ioc.refcount) <= 0);
    atomic_long_inc(&ioc.refcount);
    }
//
// Exit an icq. Called with ioc locked for blk-mq, and with both ioc
// and queue locked for legacy.
//
#[no_mangle]
unsafe extern "C" fn ioc_exit_icq(icq: *mut io_cq) {
    let mut et = icq.q.elevator.type;
    if (icq.flags & ICQ_EXITED) {
    return;
    }
    if (et.ops.exit_icq) {
    et.ops.exit_icq(icq);
    }
    icq.flags |= ICQ_EXITED;
    }
#[no_mangle]
unsafe extern "C" fn ioc_exit_icqs(ioc: *mut io_context) {
pub static mut icq: *mut c_void = core::ptr::null_mut();
    spin_lock_irq(&ioc.lock);
    hlist_for_each_entry(icq, &ioc.icq_list, ioc_node)
    ioc_exit_icq(icq);
    spin_unlock_irq(&ioc.lock);
    }
//
// Release an icq. Called with ioc locked for blk-mq, and with both ioc
// and queue locked for legacy.
//
#[no_mangle]
unsafe extern "C" fn ioc_destroy_icq(icq: *mut io_cq) {
    let mut ioc = icq.ioc;
    let mut q = icq.q;
    let mut et = q.elevator.type;
    lockdep_assert_held(&ioc.lock);
    lockdep_assert_held(&q.queue_lock);
    if (icq.flags & ICQ_DESTROYED) {
    return;
    }
    radix_tree_delete(&ioc.icq_tree, icq.q.id);
    hlist_del_init(&icq.ioc_node);
    list_del_init(&icq.q_node);
//
// Both setting lookup hint to and clearing it from @icq are done
// under queue_lock.  If it's not pointing to @icq now, it never
// will.  Hint assignment itself can race safely.
//
    if (rcu_access_pointer(ioc.icq_hint) == icq) {
    rcu_assign_pointer(ioc.icq_hint, core::ptr::null_mut());
    }
    ioc_exit_icq(icq);
//
// @icq->q might have gone away by the time RCU callback runs
// making it impossible to determine icq_cache.  Record it in @icq.
//
    icq.__rcu_icq_cache = et.icq_cache;
    icq.flags |= ICQ_DESTROYED;
    kfree_rcu(icq, __rcu_head);
    }
//
// Slow path for ioc release in put_io_context().  Performs double-lock
// dancing to unlink all icq's and then frees ioc.
//
#[no_mangle]
unsafe extern "C" fn ioc_release_fn(work: *mut work_struct) {
    let mut ioc = container_of!(work, io_context,
    release_work);
    spin_lock_irq(&ioc.lock);
    while (!hlist_empty(&ioc.icq_list)) {
    let mut icq = hlist_entry(ioc.icq_list.first, io_cq, ioc_node);
    let mut q = icq.q;
    if (spin_trylock(&q.queue_lock)) {
    ioc_destroy_icq(icq);
    spin_unlock(&q.queue_lock);
    } else {
// Make sure q and icq cannot be freed.
    rcu_read_lock();
// Re-acquire the locks in the correct order.
    spin_unlock(&ioc.lock);
    spin_lock(&q.queue_lock);
    spin_lock(&ioc.lock);
    ioc_destroy_icq(icq);
    spin_unlock(&q.queue_lock);
    rcu_read_unlock();
    }
    }
    spin_unlock_irq(&ioc.lock);
    kmem_cache_free(iocontext_cachep, ioc);
    }
//
// Releasing icqs requires reverse order double locking and we may already be
// holding a queue_lock.  Do it asynchronously from a workqueue.
//
#[no_mangle]
unsafe extern "C" fn ioc_delay_free(ioc: *mut io_context) -> bool {
    let mut flags = 0;
    spin_lock_irqsave(&ioc.lock, flags);
    if (!hlist_empty(&ioc.icq_list)) {
    queue_work(system_power_efficient_wq, &ioc.release_work);
    spin_unlock_irqrestore(&ioc.lock, flags);
    return true;
    }
    spin_unlock_irqrestore(&ioc.lock, flags);
    return false;
    }
//
// ioc_clear_queue - break any ioc association with the specified queue
// @q: request_queue being cleared
//
// Walk @q->icq_list and exit all io_cq's.
//
#[no_mangle]
pub unsafe extern "C" fn ioc_clear_queue(q: *mut request_queue) {
    spin_lock_irq(&q.queue_lock);
    while (!list_empty(&q.icq_list)) {
    let mut icq = list_first_entry(&q.icq_list, io_cq, q_node);
//
// Other context won't hold ioc lock to wait for queue_lock, see
// details in ioc_release_fn().
//
    spin_lock(&icq.ioc.lock);
    ioc_destroy_icq(icq);
    spin_unlock(&icq.ioc.lock);
    }
    spin_unlock_irq(&q.queue_lock);
    }

#[no_mangle]
pub unsafe extern "C" fn ioc_exit_icqs(ioc: *mut io_context) {
    }
#[no_mangle]
pub unsafe extern "C" fn ioc_delay_free(ioc: *mut io_context) -> bool {
    return false;
    }

//
// put_io_context - put a reference of io_context
// @ioc: io_context to put
//
// Decrement reference count of @ioc and release it if the count reaches
// zero.
//
#[no_mangle]
pub unsafe extern "C" fn put_io_context(ioc: *mut io_context) {
    BUG_ON!(atomic_long_read(&ioc.refcount) <= 0);
    if (atomic_long_dec_and_test(&ioc.refcount) && !ioc_delay_free(ioc)) {
    kmem_cache_free(iocontext_cachep, ioc);
    }
    }
    EXPORT_SYMBOL_GPL(put_io_context);
// Called by the exiting task
#[no_mangle]
pub unsafe extern "C" fn exit_io_context(task: *mut task_struct) {
pub static mut ioc: *mut c_void = core::ptr::null_mut();
    task_lock(task);
    ioc = task.io_context;
    task.io_context = core::ptr::null_mut();
    task_unlock(task);
    if (atomic_dec_and_test(&ioc.active_ref)) {
    ioc_exit_icqs(ioc);
    put_io_context(ioc);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_io_context(gfp_flags: gfp_t, node: c_int) -> *mut c_void {
pub static mut ioc: *mut c_void = core::ptr::null_mut();
    ioc = kmem_cache_alloc_node(iocontext_cachep, gfp_flags | __GFP_ZERO,
    node);
    if (unlikely(!ioc)) {
    return core::ptr::null_mut();
    }
    atomic_long_set(&ioc.refcount, 1);
    atomic_set(&ioc.active_ref, 1);

    spin_lock_init(&ioc.lock);
    INIT_RADIX_TREE(&ioc.icq_tree, GFP_ATOMIC);
    INIT_HLIST_HEAD(&ioc.icq_list);
    INIT_WORK(&ioc.release_work, ioc_release_fn);

    ioc.ioprio = IOPRIO_DEFAULT;
    return ioc;
    }
#[no_mangle]
pub unsafe extern "C" fn set_task_ioprio(task: *mut task_struct, ioprio: c_int) -> c_int {
    let mut err = 0;
    let mut cred = current_cred(), *tcred;
    rcu_read_lock();
    tcred = __task_cred(task);
    if (!uid_eq(tcred.uid, cred.euid) &&
    !uid_eq(tcred.uid, cred.uid) && !capable(CAP_SYS_NICE)) {
    rcu_read_unlock();
    return -EPERM;
    }
    rcu_read_unlock();
    err = security_task_setioprio(task, ioprio);
    if (err) {
    return err;
    }
    task_lock(task);
    if (unlikely(!task.io_context)) {
pub static mut ioc: *mut c_void = core::ptr::null_mut();
    task_unlock(task);
    ioc = alloc_io_context(GFP_ATOMIC, NUMA_NO_NODE);
    if (!ioc) {
    return -ENOMEM;
    }
    task_lock(task);
    if (task.flags & PF_EXITING) {
    kmem_cache_free(iocontext_cachep, ioc);
// goto;
    }
    if (task.io_context) {
    kmem_cache_free(iocontext_cachep, ioc);
    }
    else {
    task.io_context = ioc;
    }
    }
    task.io_context.ioprio = ioprio;
// label;
    task_unlock(task);
    return 0;
    }
    EXPORT_SYMBOL_GPL(set_task_ioprio);
#[no_mangle]
pub unsafe extern "C" fn __copy_io(clone_flags: u64, tsk: *mut task_struct) -> c_int {
    let mut ioc = current.io_context;
//
// Share io context with parent, if CLONE_IO is set
//
    if (clone_flags & CLONE_IO) {
    atomic_inc(&ioc.active_ref);
    tsk.io_context = ioc;
    } else if (ioprio_valid(ioc.ioprio)) {
    tsk.io_context = alloc_io_context(GFP_KERNEL, NUMA_NO_NODE);
    if (!tsk.io_context) {
    return -ENOMEM;
    }
    tsk.io_context.ioprio = ioc.ioprio;
    }
    return 0;
    }

//
// ioc_lookup_icq - lookup io_cq from ioc in io issue path
// @q: the associated request_queue
//
// Look up io_cq associated with @ioc - @q pair from @ioc.  Must be called
// from io issue path, either return NULL if current issue io to @q for the
// first time, or return a valid icq.
//
#[no_mangle]
pub unsafe extern "C" fn ioc_lookup_icq(q: *mut request_queue) -> *mut c_void {
    let mut ioc = current.io_context;
pub static mut icq: *mut c_void = core::ptr::null_mut();
//
// icq's are indexed from @ioc using radix tree and hint pointer,
// both of which are protected with RCU, io issue path ensures that
// both request_queue and current task are valid, the found icq
// is guaranteed to be valid until the io is done.
//
    rcu_read_lock();
    icq = rcu_dereference(ioc.icq_hint);
    if (icq && icq.q == q) {
// goto;
    }
    icq = radix_tree_lookup(&ioc.icq_tree, q.id);
    if (icq && icq.q == q) {
    rcu_assign_pointer(ioc.icq_hint, icq);	/* allowed to race */
    }
    else {
    icq = core::ptr::null_mut();
    }
// label;
    rcu_read_unlock();
    return icq;
    }
    EXPORT_SYMBOL(ioc_lookup_icq);
//
// ioc_create_icq - create and link io_cq
// @q: request_queue of interest
//
// Make sure io_cq linking @ioc and @q exists.  If icq doesn't exist, they
// will be created using @gfp_mask.
//
// The caller is responsible for ensuring @ioc won't go away and @q is
// alive and will stay alive until this function returns.
//
#[no_mangle]
pub unsafe extern "C" fn ioc_create_icq(q: *mut request_queue) -> *mut c_void {
    let mut ioc = current.io_context;
    let mut et = q.elevator.type;
pub static mut icq: *mut c_void = core::ptr::null_mut();
// allocate stuff
    icq = kmem_cache_alloc_node(et.icq_cache, GFP_ATOMIC | __GFP_ZERO,
    q.node);
    if (!icq) {
    return core::ptr::null_mut();
    }
    if (radix_tree_maybe_preload(GFP_ATOMIC) < 0) {
    kmem_cache_free(et.icq_cache, icq);
    return core::ptr::null_mut();
    }
    icq.ioc = ioc;
    icq.q = q;
    INIT_LIST_HEAD(&icq.q_node);
    INIT_HLIST_NODE(&icq.ioc_node);
// lock both q and ioc and try to link @icq
    spin_lock_irq(&q.queue_lock);
    spin_lock(&ioc.lock);
    if (likely(!radix_tree_insert(&ioc.icq_tree, q.id, icq))) {
    hlist_add_head(&icq.ioc_node, &ioc.icq_list);
    list_add(&icq.q_node, &q.icq_list);
    if (et.ops.init_icq) {
    et.ops.init_icq(icq);
    }
    } else {
    kmem_cache_free(et.icq_cache, icq);
    icq = ioc_lookup_icq(q);
    if (!icq) {
    printk("cfq: icq link failed!\n");
    }
    }
    spin_unlock(&ioc.lock);
    spin_unlock_irq(&q.queue_lock);
    radix_tree_preload_end();
    return icq;
    }
#[no_mangle]
pub unsafe extern "C" fn ioc_find_get_icq(q: *mut request_queue) -> *mut c_void {
    let mut ioc = current.io_context;
    let mut icq = core::ptr::null_mut();
    if (unlikely(!ioc)) {
    ioc = alloc_io_context(GFP_ATOMIC, q.node);
    if (!ioc) {
    return core::ptr::null_mut();
    }
    task_lock(current);
    if (current.io_context) {
    kmem_cache_free(iocontext_cachep, ioc);
    ioc = current.io_context;
    } else {
    current.io_context = ioc;
    }
    get_io_context(ioc);
    task_unlock(current);
    } else {
    get_io_context(ioc);
    icq = ioc_lookup_icq(q);
    }
    if (!icq) {
    icq = ioc_create_icq(q);
    if (!icq) {
    put_io_context(ioc);
    return core::ptr::null_mut();
    }
    }
    return icq;
    }
    EXPORT_SYMBOL_GPL(ioc_find_get_icq);

#[no_mangle]
unsafe extern "C" fn blk_ioc_init() -> c_int {
    iocontext_cachep = kmem_cache_create("blkdev_ioc",
    sizeof!(io_context), 0, SLAB_PANIC, core::ptr::null_mut());
    return 0;
    }
    subsys_initcall!(blk_ioc_init);
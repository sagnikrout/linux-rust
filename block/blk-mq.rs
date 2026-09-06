//! Automatically rewritten from C Header to Rust Module
//! Source: block/blk-mq.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_mq_ctxs {
    pub kobj: kobject,
    pub queue_ctx: *mut blk_mq_ctx ,
}

//
// struct blk_mq_ctx - State for a software queue facing the submitting CPUs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_mq_ctx {
    pub lock: spinlock_t,
    pub rq_lists: [list_head; HCTX_MAX_TYPES],
}
    pub cpu: c_uint,
    pub index_hw: [c_ushort; HCTX_MAX_TYPES],
    pub hctxs: [*mut blk_mq_hw_ctx; HCTX_MAX_TYPES],
    pub queue: *mut request_queue,
    pub ctxs: *mut blk_mq_ctxs,
    pub kobj: kobject,
}
}

pub type blk_insert_t = u32;

extern "C" {
    pub fn blk_mq_submit_bio(bio: *mut bio);
}
extern "C" {
    pub fn blk_mq_exit_queue(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_wake_waiters(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_flush_busy_ctxs(hctx: *mut blk_mq_hw_ctx, list: *mut list_head);
}
extern "C" {
    pub fn blk_mq_put_rq_ref(rq: *mut request);
}
//
// Internal helpers for allocating/freeing the request map
//
extern "C" {
    pub fn blk_mq_free_rq_map(set: *mut blk_mq_tag_set, tags: *mut blk_mq_tags);
}
//
// CPU -> queue mappings
//
extern "C" {
    pub fn blk_mq_hw_queue_to_node(qmap: *mut blk_mq_queue_map, int: unsigned) -> c_int;
}
//
// blk_mq_map_queue_type() - map (hctx_type,cpu) to hardware queue
// @q: request queue
// @type: the hctx type index
// @cpu: CPU
//
extern "C" {
    pub fn queue_hctx(_arg: (q), _arg: (q->tag_set->map[type].mq_map[cpu])) -> return;
}
//
// The caller ensure that if REQ_POLLED, poll must be enabled.
//
// blk_mq_map_queue() - map (cmd_flags,type) to hardware queue
// @opf: operation type (REQ_OP_*) and flags (e.g. REQ_POLLED).
// @ctx: software queue cpu ctx
//
// Default to double of smaller one between hw queue_depth and
// 128, since we don't split into sync/async like the old code
// did. Additionally, this is a per-hw queue depth.
//
extern "C" {
    pub fn min_t(int: unsigned, _arg: set->queue_depth, _arg: BLKDEV_DEFAULT_RQ) -> *mut return 2;
}
//
// sysfs helpers
//
extern "C" {
    pub fn blk_mq_sysfs_init(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_sysfs_deinit(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_sysfs_register(disk: *mut gendisk) -> c_int;
}
extern "C" {
    pub fn blk_mq_sysfs_unregister(disk: *mut gendisk);
}
extern "C" {
    pub fn blk_mq_sysfs_register_hctxs(q: *mut request_queue) -> c_int;
}
extern "C" {
    pub fn blk_mq_sysfs_unregister_hctxs(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_hctx_kobj_init(hctx: *mut blk_mq_hw_ctx);
}
extern "C" {
    pub fn blk_mq_free_plug_rqs(plug: *mut blk_plug);
}
extern "C" {
    pub fn blk_mq_flush_plug_list(plug: *mut blk_plug, from_schedule: bool);
}
extern "C" {
    pub fn blk_mq_cancel_work_sync(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_release(q: *mut request_queue);
}
extern "C" {
    pub fn per_cpu_ptr(_arg: q->queue_ctx, _arg: cpu) -> return;
}
//
// This assumes per-cpu software queueing queues. They could be per-node
// as well, for instance. For now this is hardcoded as-is. Note that we don't
// care about preemption, since we know the ctx's are persistent. This does
// mean that we can't rely on ctx always matching the currently running CPU.
//
extern "C" {
    pub fn __blk_mq_get_ctx(_arg: q, _arg: raw_smp_processor_id()) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_mq_alloc_data {
// input parameter
    pub q: *mut request_queue,
    pub flags: blk_mq_req_flags_t,
    pub shallow_depth: c_uint,
    pub cmd_flags: blk_opf_t,
    pub rq_flags: req_flags_t,
// allocate multiple requests/tags in one go
    pub nr_tags: c_uint,
    pub cached_rqs: *mut rq_list,
// input & output parameter
    pub ctx: *mut blk_mq_ctx,
    pub hctx: *mut blk_mq_hw_ctx,
}

extern "C" {
    pub fn blk_mq_free_tags(set: *mut blk_mq_tag_set, tags: *mut blk_mq_tags);
}
extern "C" {
    pub fn blk_mq_get_tag(data: *mut blk_mq_alloc_data) -> c_uint;
}
extern "C" {
    pub fn blk_mq_put_tags(tags: *mut blk_mq_tags, tag_array: *mut c_int, nr_tags: c_int);
}
extern "C" {
    pub fn blk_mq_tag_wakeup_all(tags: *mut blk_mq_tags, _arg: bool);
}
extern "C" {
    pub fn sbq_wait_ptr(_arg: bt, _arg: &hctx->wait_index) -> return;
}
extern "C" {
    pub fn __blk_mq_tag_busy(: *mut blk_mq_hw_ctx);
}
extern "C" {
    pub fn __blk_mq_tag_idle(: *mut blk_mq_hw_ctx);
}
// Fast path: hardware queue is not stopped most of the time.
//
// This barrier is used to order adding of dispatch list before and
// the test of BLK_MQ_S_STOPPED below. Pairs with the memory barrier
// in blk_mq_start_stopped_hw_queue() so that dispatch code could
// either see BLK_MQ_S_STOPPED is cleared or dispatch list is not
// empty to avoid missing dispatching requests.
//
extern "C" {
    pub fn test_bit(_arg: BLK_MQ_S_STOPPED, _arg: &hctx->state) -> return;
}
extern "C" {
    pub fn blk_mq_in_driver_rw(part: *mut block_device, inflight[2]: c_uint);
}
extern "C" {
    pub fn atomic_read(_arg: &hctx->queue->nr_active_requests_shared_tags) -> return;
}
extern "C" {
    pub fn atomic_read(_arg: &hctx->nr_active) -> return;
}
extern "C" {
    pub fn __blk_mq_alloc_driver_tag(rq: *mut request) -> bool;
}
// Free all requests on the list
//
// For shared tag users, we track the number of currently active users
// and attempt to provide a fair share of the tag depth for each of them.
//
// Don't try dividing an ant
//
// Allow at least some tags
//
// run the code block in @dispatch_ops with rcu/srcu read lock held
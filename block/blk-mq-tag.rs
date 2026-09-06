//! Automatically rewritten from C to Rust
//! Source: block/blk-mq-tag.c
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
// Tag allocation using scalable bitmaps. Uses active queue tracking to support
// fairer distribution of tags between multiple submitters when a shared tag map
// is used.
//
// Copyright (C) 2013-2014 Jens Axboe
//

//
// Recalculate wakeup batch when tag is shared by hctx.
//
#[no_mangle]
pub unsafe extern "C" fn blk_mq_update_wake_batch(tags: *mut blk_mq_tags, users: c_uint) {
    if (!users) {
    return;
    }
    sbitmap_queue_recalculate_wake_batch(&tags.bitmap_tags,
    users);
    sbitmap_queue_recalculate_wake_batch(&tags.breserved_tags,
    users);
    }
//
// If a previously inactive queue goes active, bump the active user count.
// We need to do this before try to allocate driver tag, then even if fail
// to get tag when first time, the other shared-tag users could reserve
// budget for it.
//
#[no_mangle]
pub unsafe extern "C" fn __blk_mq_tag_busy(hctx: *mut blk_mq_hw_ctx) {
    let mut users = 0;
    let mut flags = 0;
    let mut tags = hctx.tags;
//
// calling test_bit() prior to test_and_set_bit() is intentional,
// it avoids dirtying the cacheline if the queue is already active.
//
    if (blk_mq_is_shared_tags(hctx.flags)) {
    let mut q = hctx.queue;
    if (test_bit(QUEUE_FLAG_HCTX_ACTIVE, &q.queue_flags) ||
    test_and_set_bit(QUEUE_FLAG_HCTX_ACTIVE, &q.queue_flags)) {
    return;
    }
    } else {
    if (test_bit(BLK_MQ_S_TAG_ACTIVE, &hctx.state) ||
    test_and_set_bit(BLK_MQ_S_TAG_ACTIVE, &hctx.state)) {
    return;
    }
    }
    spin_lock_irqsave(&tags.lock, flags);
    users = tags.active_queues + 1;
    WRITE_ONCE(tags.active_queues, users);
    blk_mq_update_wake_batch(tags, users);
    spin_unlock_irqrestore(&tags.lock, flags);
    }
//
// Wakeup all potentially sleeping on tags
//
#[no_mangle]
pub unsafe extern "C" fn blk_mq_tag_wakeup_all(tags: *mut blk_mq_tags, include_reserve: bool) {
    sbitmap_queue_wake_all(&tags.bitmap_tags);
    if (include_reserve) {
    sbitmap_queue_wake_all(&tags.breserved_tags);
    }
    }
//
// If a previously busy queue goes inactive, potential waiters could now
// be allowed to queue. Wake them up and check.
//
#[no_mangle]
pub unsafe extern "C" fn __blk_mq_tag_idle(hctx: *mut blk_mq_hw_ctx) {
    let mut tags = hctx.tags;
    let mut users = 0;
    if (blk_mq_is_shared_tags(hctx.flags)) {
    let mut q = hctx.queue;
    if (!test_and_clear_bit(QUEUE_FLAG_HCTX_ACTIVE,
    &q.queue_flags)) {
    return;
    }
    } else {
    if (!test_and_clear_bit(BLK_MQ_S_TAG_ACTIVE, &hctx.state)) {
    return;
    }
    }
    spin_lock_irq(&tags.lock);
    users = tags.active_queues - 1;
    WRITE_ONCE(tags.active_queues, users);
    blk_mq_update_wake_batch(tags, users);
    spin_unlock_irq(&tags.lock);
    blk_mq_tag_wakeup_all(tags, false);
    }
#[no_mangle]
pub unsafe extern "C" fn __blk_mq_get_tag(data: *mut blk_mq_alloc_data, bt: *mut sbitmap_queue) -> c_int {
    if (!data.q.elevator && !(data.flags & BLK_MQ_REQ_RESERVED) &&
    !hctx_may_queue(data.hctx, bt)) {
    return BLK_MQ_NO_TAG;
    }
    if (data.shallow_depth) {
    return sbitmap_queue_get_shallow(bt, data.shallow_depth);
    }
    else {
    return __sbitmap_queue_get(bt);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_get_tags(data: *mut blk_mq_alloc_data, nr_tags: c_int, offset: *mut c_uint) -> c_ulong {
    let mut tags = blk_mq_tags_from_data(data);
    let mut bt = &tags.bitmap_tags;
    let mut ret = 0;
    if (data.shallow_depth ||data.flags & BLK_MQ_REQ_RESERVED ||
    data.hctx.flags & BLK_MQ_F_TAG_QUEUE_SHARED) {
    return 0;
    }
    ret = __sbitmap_queue_get_batch(bt, nr_tags, offset);
// offset += tags->nr_reserved_tags;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_get_tag(data: *mut blk_mq_alloc_data) -> c_uint {
    let mut tags = blk_mq_tags_from_data(data);
pub static mut bt: *mut c_void = core::ptr::null_mut();
pub static mut ws: *mut c_void = core::ptr::null_mut();
pub static mut wait: usize = 0;
    let mut tag_offset = 0;
    let mut tag = 0;
    if (data.flags & BLK_MQ_REQ_RESERVED) {
    if (unlikely(!tags.nr_reserved_tags)) {
    WARN_ON_ONCE!(1);
    return BLK_MQ_NO_TAG;
    }
    bt = &tags.breserved_tags;
    tag_offset = 0;
    } else {
    bt = &tags.bitmap_tags;
    tag_offset = tags.nr_reserved_tags;
    }
    tag = __blk_mq_get_tag(data, bt);
    if (tag != BLK_MQ_NO_TAG) {
// goto;
    }
    if (data.flags & BLK_MQ_REQ_NOWAIT) {
    return BLK_MQ_NO_TAG;
    }
    ws = bt_wait_ptr(bt, data.hctx);
    do {
pub static mut bt_prev: *mut c_void = core::ptr::null_mut();
//
// We're out of tags on this hardware queue, kick any
// pending IO submits before going to sleep waiting for
// some to complete.
//
    blk_mq_run_hw_queue(data.hctx, false);
//
// Retry tag allocation after running the hardware queue,
// as running the queue may also have found completions.
//
    tag = __blk_mq_get_tag(data, bt);
    if (tag != BLK_MQ_NO_TAG) {
    break;
    }
// Log the starvation event before altering task state
    trace_block_rq_tag_wait(data.q, data.hctx,
    data.rq_flags & RQF_SCHED_TAGS,
    data.flags);
    sbitmap_prepare_to_wait(bt, ws, &wait, TASK_UNINTERRUPTIBLE);
    tag = __blk_mq_get_tag(data, bt);
    if (tag != BLK_MQ_NO_TAG) {
    break;
    }
    bt_prev = bt;
    io_schedule();
    sbitmap_finish_wait(bt, ws, &wait);
    data.ctx = blk_mq_get_ctx(data.q);
    data.hctx = blk_mq_map_queue(data.cmd_flags, data.ctx);
    tags = blk_mq_tags_from_data(data);
    if (data.flags & BLK_MQ_REQ_RESERVED) {
    bt = &tags.breserved_tags;
    }
    else {
    bt = &tags.bitmap_tags;
    }
//
// If destination hw queue is changed, fake wake up on
// previous queue for compensating the wake up miss, so
// other allocations on previous queue won't be starved.
//
    if (bt != bt_prev) {
    sbitmap_queue_wake_up(bt_prev, 1);
    }
    ws = bt_wait_ptr(bt, data.hctx);
    } while (1);
    sbitmap_finish_wait(bt, ws, &wait);
// label;
//
// Give up this allocation if the hctx is inactive.  The caller will
// retry on an active hctx.
//
    if (unlikely(test_bit(BLK_MQ_S_INACTIVE, &data.hctx.state))) {
    blk_mq_put_tag(tags, data.ctx, tag + tag_offset);
    return BLK_MQ_NO_TAG;
    }
    return tag + tag_offset;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_put_tag(tags: *mut blk_mq_tags, ctx: *mut blk_mq_ctx, tag: c_uint) {
    if (!blk_mq_tag_is_reserved(tags, tag)) {
pub static mut real_tag: c_int = 0;
    BUG_ON!(real_tag >= tags.nr_tags);
    sbitmap_queue_clear(&tags.bitmap_tags, real_tag, ctx.cpu);
    } else {
    sbitmap_queue_clear(&tags.breserved_tags, tag, ctx.cpu);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_put_tags(tags: *mut blk_mq_tags, tag_array: *mut c_int, nr_tags: c_int) {
    sbitmap_queue_clear_batch(&tags.bitmap_tags, tags.nr_reserved_tags,
    tag_array, nr_tags);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_iter_data {
    pub hctx: *mut blk_mq_hw_ctx,
    pub q: *mut request_queue,
    pub fn: *mut busy_tag_iter_fn,
    pub data: *mut c_void,
    pub reserved: bool,
}

#[no_mangle]
pub unsafe extern "C" fn blk_mq_find_and_get_req(tags: *mut blk_mq_tags, bitnr: c_uint) -> *mut c_void {
pub static mut rq: *mut c_void = core::ptr::null_mut();
    rq = tags.rqs[bitnr];
    if (!rq || rq.tag != bitnr || !req_ref_inc_not_zero(rq)) {
    rq = core::ptr::null_mut();
    }
    return rq;
    }
#[no_mangle]
unsafe extern "C" fn bt_iter(bitmap: *mut sbitmap, bitnr: c_uint, data: *mut c_void) -> bool {
    let mut iter_data = data;
    let mut hctx = iter_data.hctx;
    let mut q = iter_data.q;
    let mut set = q.tag_set;
pub static mut tags: *mut c_void = core::ptr::null_mut();
pub static mut rq: *mut c_void = core::ptr::null_mut();
pub static mut ret: bool = true;
    if (blk_mq_is_shared_tags(set.flags)) {
    tags = set.shared_tags;
    }
    else {
    tags = hctx.tags;
    }
    if (!iter_data.reserved) {
    bitnr += tags.nr_reserved_tags;
    }
//
// We can hit rq == NULL here, because the tagging functions
// test and set the bit before assigning ->rqs[].
//
    rq = blk_mq_find_and_get_req(tags, bitnr);
    if (!rq) {
    return true;
    }
    if (rq.q == q && (!hctx || rq.mq_hctx == hctx)) {
    ret = iter_data.fn(rq, iter_data.data);
    }
    blk_mq_put_rq_ref(rq);
    return ret;
    }
//
// bt_for_each - iterate over the requests associated with a hardware queue
// @hctx:	Hardware queue to examine.
// @q:		Request queue @hctx is associated with (@hctx->queue).
// @bt:		sbitmap to examine. This is either the breserved_tags member
// or the bitmap_tags member of struct blk_mq_tags.
// @fn:		Pointer to the function that will be called for each request
// associated with @hctx that has been assigned a driver tag.
// @fn will be called as follows: @fn(rq, @data) where rq is a
// pointer to a request. Return %true to continue iterating tags;
// %false to stop.
// @data:	Will be passed as second argument to @fn.
// @reserved:	Indicates whether @bt is the breserved_tags member or the
// bitmap_tags member of struct blk_mq_tags.
//
#[no_mangle]
pub unsafe extern "C" fn bt_for_each(hctx: *mut blk_mq_hw_ctx, q: *mut request_queue, bt: *mut sbitmap_queue, fn: *mut busy_tag_iter_fn, data: *mut c_void, reserved: bool) {
pub static mut bt_iter_data: usize = 0;
    sbitmap_for_each_set(&bt.sb, bt_iter, &iter_data);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_tags_iter_data {
    pub tags: *mut blk_mq_tags,
    pub fn: *mut busy_tag_iter_fn,
    pub data: *mut c_void,
    pub flags: c_uint,
}

#[no_mangle]
unsafe extern "C" fn bt_tags_iter(bitmap: *mut sbitmap, bitnr: c_uint, data: *mut c_void) -> bool {
    let mut iter_data = data;
    let mut tags = iter_data.tags;
pub static mut rq: *mut c_void = core::ptr::null_mut();
pub static mut ret: bool = true;
pub static mut iter_static_rqs: bool = false;
    if (!(iter_data.flags & BT_TAG_ITER_RESERVED)) {
    bitnr += tags.nr_reserved_tags;
    }
//
// We can hit rq == NULL here, because the tagging functions
// test and set the bit before assigning ->rqs[].
//
    if (iter_static_rqs) {
    rq = tags.static_rqs[bitnr];
    }
    else {
    rq = blk_mq_find_and_get_req(tags, bitnr);
    }
    if (!rq) {
    return true;
    }
    if (!(iter_data.flags & BT_TAG_ITER_STARTED) ||
    blk_mq_request_started(rq)) {
    ret = iter_data.fn(rq, iter_data.data);
    }
    if (!iter_static_rqs) {
    blk_mq_put_rq_ref(rq);
    }
    return ret;
    }
//
// bt_tags_for_each - iterate over the requests in a tag map
// @tags:	Tag map to iterate over.
// @bt:		sbitmap to examine. This is either the breserved_tags member
// or the bitmap_tags member of struct blk_mq_tags.
// @fn:		Pointer to the function that will be called for each started
// request. @fn will be called as follows: @fn(rq, @data) where rq
// is a pointer to a request. Return %true to continue iterating
// tags; %false to stop.
// @data:	Will be passed as second argument to @fn.
// @flags:	BT_TAG_ITER_
//
#[no_mangle]
pub unsafe extern "C" fn bt_tags_for_each(tags: *mut blk_mq_tags, bt: *mut sbitmap_queue, fn: *mut busy_tag_iter_fn, data: *mut c_void, flags: c_uint) {
pub static mut bt_tags_iter_data: usize = 0;
    if (tags.rqs) {
    sbitmap_for_each_set(&bt.sb, bt_tags_iter, &iter_data);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __blk_mq_all_tag_iter(tags: *mut blk_mq_tags, fn: *mut busy_tag_iter_fn, priv: *mut c_void, flags: c_uint) {
    WARN_ON_ONCE!(flags & BT_TAG_ITER_RESERVED);
    if (tags.nr_reserved_tags) {
    bt_tags_for_each(tags, &tags.breserved_tags, fn, priv,
    flags | BT_TAG_ITER_RESERVED);
    }
    bt_tags_for_each(tags, &tags.bitmap_tags, fn, priv, flags);
    }
//
// blk_mq_all_tag_iter - iterate over all requests in a tag map
// @tags:	Tag map to iterate over.
// @fn:		Pointer to the function that will be called for each
// request. @fn will be called as follows: @fn(rq, @priv) where rq
// is a pointer to a request. Return %true to continue iterating
// tags; %false to stop.
// @priv:	Will be passed as second argument to @fn.
//
// Caller has to pass the tag map from which requests are allocated.
//
#[no_mangle]
pub unsafe extern "C" fn blk_mq_all_tag_iter(tags: *mut blk_mq_tags, fn: *mut busy_tag_iter_fn, priv: *mut c_void) {
    __blk_mq_all_tag_iter(tags, fn, priv, BT_TAG_ITER_STATIC_RQS);
    }
//
// blk_mq_tagset_busy_iter - iterate over all started requests in a tag set
// @tagset:	Tag set to iterate over.
// @fn:		Pointer to the function that will be called for each started
// request. @fn will be called as follows: @fn(rq, @priv) where
// rq is a pointer to a request. Return true to continue iterating
// tags, false to stop.
// @priv:	Will be passed as second argument to @fn.
//
// We grab one request reference before calling @fn and release it after
// @fn returns.
//
#[no_mangle]
pub unsafe extern "C" fn blk_mq_tagset_busy_iter(tagset: *mut blk_mq_tag_set, fn: *mut busy_tag_iter_fn, priv: *mut c_void) {
pub static mut flags: c_uint = 0;
    let mut i = 0;
    let mut nr_tags = 0;
    let mut srcu_idx = 0;
    srcu_idx = srcu_read_lock(&tagset.tags_srcu);
    nr_tags = blk_mq_is_shared_tags(flags) ? 1 : tagset.nr_hw_queues;
    while (i < nr_tags) {
    if (tagset.tags && tagset.tags[i]) {
    __blk_mq_all_tag_iter(tagset.tags[i], fn, priv,
    BT_TAG_ITER_STARTED);
    }
    }
    srcu_read_unlock(&tagset.tags_srcu, srcu_idx);
    }
    EXPORT_SYMBOL(blk_mq_tagset_busy_iter);
#[no_mangle]
unsafe extern "C" fn blk_mq_tagset_count_completed_rqs(rq: *mut request, data: *mut c_void) -> bool {
    let mut count = data;
    if (blk_mq_request_completed(rq)) {
    (*count)++;
    }
    return true;
    }
//
// blk_mq_tagset_wait_completed_request - Wait until all scheduled request
// completions have finished.
// @tagset:	Tag set to drain completed request
//
// Note: This function has to be run after all IO queues are shutdown
//
#[no_mangle]
pub unsafe extern "C" fn blk_mq_tagset_wait_completed_request(tagset: *mut blk_mq_tag_set) {
    while (true) {
pub static mut count: unsigned = 0;
    blk_mq_tagset_busy_iter(tagset,
    blk_mq_tagset_count_completed_rqs, &count);
    if (!count) {
    break;
    }
    msleep(5);
    }
    }
    EXPORT_SYMBOL(blk_mq_tagset_wait_completed_request);
//
// blk_mq_queue_tag_busy_iter - iterate over all requests with a driver tag
// @q:		Request queue to examine.
// @fn:		Pointer to the function that will be called for each request
// on @q. @fn will be called as follows: @fn(rq, @priv) where rq
// is a pointer to a request and hctx points to the hardware queue
// associated with the request.
// @priv:	Will be passed as second argument to @fn.
//
// Note: if @q->tag_set is shared with other request queues then @fn will be
// called for all requests on all queues that share that tag set and not only
// for requests associated with @q.
//
#[no_mangle]
pub unsafe extern "C" fn blk_mq_queue_tag_busy_iter(q: *mut request_queue, fn: *mut busy_tag_iter_fn, priv: *mut c_void) {
    let mut srcu_idx = 0;
//
// __blk_mq_update_nr_hw_queues() updates nr_hw_queues and queue_hw_ctx
// while the queue is frozen. So we can use q_usage_counter to avoid
// racing with it.
//
    if (!percpu_ref_tryget(&q.q_usage_counter)) {
    return;
    }
    srcu_idx = srcu_read_lock(&q.tag_set.tags_srcu);
    if (blk_mq_is_shared_tags(q.tag_set.flags)) {
    let mut tags = q.tag_set.shared_tags;
    let mut bresv = &tags.breserved_tags;
    let mut btags = &tags.bitmap_tags;
    if (tags.nr_reserved_tags) {
    bt_for_each(core::ptr::null_mut(), q, bresv, fn, priv, true);
    }
    bt_for_each(core::ptr::null_mut(), q, btags, fn, priv, false);
    } else {
pub static mut hctx: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    queue_for_each_hw_ctx(q, hctx, i) {
    let mut tags = hctx.tags;
    let mut bresv = &tags.breserved_tags;
    let mut btags = &tags.bitmap_tags;
//
// If no software queues are currently mapped to this
// hardware queue, there's nothing to check
//
    if (!blk_mq_hw_queue_mapped(hctx)) {
    continue;
    }
    if (tags.nr_reserved_tags) {
    bt_for_each(hctx, q, bresv, fn, priv, true);
    }
    bt_for_each(hctx, q, btags, fn, priv, false);
    }
    }
    srcu_read_unlock(&q.tag_set.tags_srcu, srcu_idx);
    blk_queue_exit(q);
    }
#[no_mangle]
pub unsafe extern "C" fn bt_alloc(bt: *mut sbitmap_queue, depth: c_uint, round_robin: bool, node: c_int) -> c_int {
    return sbitmap_queue_init_node(bt, depth, -1, round_robin, GFP_KERNEL,
    node);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_init_tags(total_tags: c_uint, reserved_tags: c_uint, flags: c_uint, node: c_int) -> *mut c_void {
pub static mut depth: c_uint = 0;
pub static mut round_robin: bool = false;
pub static mut tags: *mut c_void = core::ptr::null_mut();
    if (total_tags > BLK_MQ_TAG_MAX) {
    pr_err!("blk-mq: tag depth too large\n");
    return core::ptr::null_mut();
    }
    tags = kzalloc_node(sizeof!(*tags), GFP_KERNEL, node);
    if (!tags) {
    return core::ptr::null_mut();
    }
    tags.nr_tags = total_tags;
    tags.nr_reserved_tags = reserved_tags;
    spin_lock_init(&tags.lock);
    INIT_LIST_HEAD(&tags.page_list);
    if (bt_alloc(&tags.bitmap_tags, depth, round_robin, node)) {
// goto;
    }
    if (bt_alloc(&tags.breserved_tags, reserved_tags, round_robin, node)) {
// goto;
    }
    return tags;
// label;
    sbitmap_queue_free(&tags.bitmap_tags);
// label;
    kfree(tags);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn blk_mq_free_tags_callback(head: *mut rcu_head) {
    let mut tags = container_of!(head, blk_mq_tags,
    rcu_head);
pub static mut page: *mut c_void = core::ptr::null_mut();
    while (!list_empty(&tags.page_list)) {
    page = list_first_entry(&tags.page_list, page, lru);
    list_del_init(&page.lru);
//
// Remove kmemleak object previously allocated in
// blk_mq_alloc_rqs().
//
    kmemleak_free(page_address(page));
    __free_pages(page, page.private);
    }
    kfree(tags);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_free_tags(set: *mut blk_mq_tag_set, tags: *mut blk_mq_tags) {
    sbitmap_queue_free(&tags.bitmap_tags);
    sbitmap_queue_free(&tags.breserved_tags);
// if tags pages is not allocated yet, free tags directly
    if (list_empty(&tags.page_list)) {
    kfree(tags);
    return;
    }
    call_srcu(&set.tags_srcu, &tags.rcu_head, blk_mq_free_tags_callback);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_tag_resize_shared_tags(set: *mut blk_mq_tag_set, size: c_uint) {
    let mut tags = set.shared_tags;
    sbitmap_queue_resize(&tags.bitmap_tags, size - set.reserved_tags);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_tag_update_sched_shared_tags(q: *mut request_queue, nr: c_uint) {
    sbitmap_queue_resize(&q.sched_shared_tags.bitmap_tags,
    nr - q.tag_set.reserved_tags);
    }
//
// blk_mq_unique_tag() - return a tag that is unique queue-wide
// @rq: request for which to compute a unique tag
//
// The tag field in struct request is unique per hardware queue but not over
// all hardware queues. Hence this function that returns a tag with the
// hardware context index in the upper bits and the per hardware queue tag in
// the lower bits.
//
// Note: When called for a request that is queued on a non-multiqueue request
// queue, the hardware context index is set to zero.
//
#[no_mangle]
pub unsafe extern "C" fn blk_mq_unique_tag(rq: *mut request) -> u32 {
    return (rq.mq_hctx.queue_num << BLK_MQ_UNIQUE_TAG_BITS) |
    (rq.tag & BLK_MQ_UNIQUE_TAG_MASK);
    }
    EXPORT_SYMBOL(blk_mq_unique_tag);
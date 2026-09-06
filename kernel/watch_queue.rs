//! Automatically rewritten from C to Rust
//! Source: kernel/watch_queue.c
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
// Watch queue and general notification mechanism, built on pipes
//
// Copyright (C) 2020 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// See Documentation/core-api/watch_queue.rst
//

    MODULE_DESCRIPTION("Watch queue");
    MODULE_AUTHOR("Red Hat, Inc.");
pub const WATCH_QUEUE_NOTE_SIZE: c_int = 128;

//
// This must be called under the RCU read-lock, which makes
// sure that the wqueue still exists. It can then take the lock,
// and check that the wqueue hasn't been destroyed, which in
// turn makes sure that the notification pipe still exists.
//
#[no_mangle]
pub unsafe extern "C" fn lock_wqueue(wqueue: *mut watch_queue) -> bool {
    spin_lock_bh(&wqueue.lock);
    if (unlikely(!wqueue.pipe)) {
    spin_unlock_bh(&wqueue.lock);
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn unlock_wqueue(wqueue: *mut watch_queue) {
    spin_unlock_bh(&wqueue.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn watch_queue_pipe_buf_release(pipe: *mut pipe_inode_info, buf: *mut pipe_buffer) {
    let mut wqueue = buf.private;
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut bit = 0;
// We need to work out which note within the page this refers to, but
// the note might have been maximum size, so merely ANDing the offset
// off doesn't work.  OTOH, the note must've been more than zero size.
//
    bit = buf.offset + buf.len;
    if ((bit & (WATCH_QUEUE_NOTE_SIZE - 1)) == 0) {
    bit -= WATCH_QUEUE_NOTE_SIZE;
    }
    bit /= WATCH_QUEUE_NOTE_SIZE;
    page = buf.page;
    bit += page.private;
    set_bit(bit, wqueue.notes_bitmap);
    generic_pipe_buf_release(pipe, buf);
    }
// No try_steal function => no stealing

// New data written to a pipe may be appended to a buffer with this type.
pub static mut pipe_buf_operations: usize = 0;
//
// Post a notification to a watch queue.
//
// Must be called with the RCU lock for reading, and the
// watch_queue lock held, which guarantees that the pipe
// hasn't been released.
//
#[no_mangle]
pub unsafe extern "C" fn post_one_notification(wqueue: *mut watch_queue, n: *mut watch_notification) -> bool {
pub static mut p: *mut c_void = core::ptr::null_mut();
    let mut pipe = wqueue.pipe;
pub static mut buf: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut head = 0;
    let mut tail = 0;
    let mut note = 0;
    let mut offset = 0;
    let mut len = 0;
pub static mut done: bool = false;
    spin_lock_irq(&pipe.rd_wait.lock);
    head = pipe.head;
    tail = pipe.tail;
    if (pipe_full(head, tail, pipe.ring_size)) {
// goto;
    }
    note = find_first_bit(wqueue.notes_bitmap, wqueue.nr_notes);
    if (note >= wqueue.nr_notes) {
// goto;
    }
    page = wqueue.notes[note / WATCH_QUEUE_NOTES_PER_PAGE];
    offset = note % WATCH_QUEUE_NOTES_PER_PAGE * WATCH_QUEUE_NOTE_SIZE;
    get_page(page);
    len = n.info & WATCH_INFO_LENGTH;
    p = kmap_local_page(page);
    memcpy(p + offset, n, len);
    kunmap_local(p);
    buf = pipe_buf(pipe, head);
    buf.page = page;
    buf.private = (unsigned long)wqueue;
    buf.ops = &watch_queue_pipe_buf_ops;
    buf.offset = offset;
    buf.len = len;
    buf.flags = PIPE_BUF_FLAG_WHOLE;
    smp_store_release(&pipe.head, head + 1); /* vs pipe_read() */
    if (!test_and_clear_bit(note, wqueue.notes_bitmap)) {
    spin_unlock_irq(&pipe.rd_wait.lock);
    BUG();
    }
    wake_up_interruptible_sync_poll_locked(&pipe.rd_wait, EPOLLIN | EPOLLRDNORM);
    done = true;
// label;
    spin_unlock_irq(&pipe.rd_wait.lock);
    if (done) {
    kill_fasync(&pipe.fasync_readers, SIGIO, POLL_IN);
    }
    return done;
// label;
    buf = pipe_buf(pipe, head - 1);
    buf.flags |= PIPE_BUF_FLAG_LOSS;
// goto;
    }
//
// Apply filter rules to a notification.
//
#[no_mangle]
pub unsafe extern "C" fn filter_watch_notification(wf: *mut watch_filter, n: *mut watch_notification) -> bool {
pub static mut wt: *mut c_void = core::ptr::null_mut();
pub static mut st_bits: c_uint = 0;
pub static mut st_index: c_uint = 0;
pub static mut st_bit: c_uint = 0;
    let mut i = 0;
    if (!test_bit(n.type, wf.type_filter)) {
    return false;
    }
    while (i < wf.nr_filters) {
    wt = &wf.filters[i];
    if (n.type == wt.type &&
    (wt.subtype_filter[st_index] & st_bit) &&
    (n.info & wt.info_mask) == wt.info_filter) {
    return true;
    }
    }
    return false; /* If there is a filter, the default is to reject. */
    }
//
// __post_watch_notification - Post an event notification
// @wlist: The watch list to post the event to.
// @n: The notification record to post.
// @cred: The creds of the process that triggered the notification.
// @id: The ID to match on the watch.
//
// Post a notification of an event into a set of watch queues and let the users
// know.
//
// The size of the notification should be set in n->info & WATCH_INFO_LENGTH and
// should be in units of sizeof!(*n).
//
#[no_mangle]
pub unsafe extern "C" fn __post_watch_notification(wlist: *mut watch_list, n: *mut watch_notification, cred: *mut cred, id: u64) {
pub static mut wf: *mut c_void = core::ptr::null_mut();
pub static mut wqueue: *mut c_void = core::ptr::null_mut();
pub static mut watch: *mut c_void = core::ptr::null_mut();
    if (((n.info & WATCH_INFO_LENGTH) >> WATCH_INFO_LENGTH__SHIFT) == 0) {
    WARN_ON!(1);
    return;
    }
    rcu_read_lock();
    hlist_for_each_entry_rcu(watch, &wlist.watchers, list_node) {
    if (watch.id != id) {
    continue;
    }
    n.info &= ~WATCH_INFO_ID;
    n.info |= watch.info_id;
    wqueue = rcu_dereference(watch.queue);
    wf = rcu_dereference(wqueue.filter);
    if (wf && !filter_watch_notification(wf, n)) {
    continue;
    }
    if (security_post_notification(watch.cred, cred, n) < 0) {
    continue;
    }
    if (lock_wqueue(wqueue)) {
    post_one_notification(wqueue, n);
    unlock_wqueue(wqueue);
    }
    }
    rcu_read_unlock();
    }
    EXPORT_SYMBOL(__post_watch_notification);
//
// Allocate sufficient pages to preallocation for the requested number of
// notifications.
//
#[no_mangle]
pub unsafe extern "C" fn watch_queue_set_size(pipe: *mut pipe_inode_info, nr_notes: c_uint) -> c_long {
    let mut wqueue = pipe.watch_queue;
pub static mut pages: *mut c_void = core::ptr::null_mut();
pub static mut bitmap: *mut c_void = core::ptr::null_mut();
    let mut user_bufs = 0;
    let mut ret = 0;
    let mut i = 0;
    let mut nr_pages = 0;
    if (!wqueue) {
    return -ENODEV;
    }
    if (wqueue.notes) {
    return -EBUSY;
    }
    if (nr_notes < 1 ||
    nr_notes > 512) /* TODO: choose a better hard limit */ {
    return -EINVAL;
    }
    nr_pages = (nr_notes + WATCH_QUEUE_NOTES_PER_PAGE - 1);
    nr_pages /= WATCH_QUEUE_NOTES_PER_PAGE;
    user_bufs = account_pipe_buffers(pipe.user, pipe.nr_accounted, nr_pages);
    if (nr_pages > pipe.max_usage &&
    (too_many_pipe_buffers_hard(user_bufs) ||
    too_many_pipe_buffers_soft(user_bufs)) &&
    pipe_is_unprivileged_user()) {
    ret = -EPERM;
// goto;
    }
    nr_notes = nr_pages * WATCH_QUEUE_NOTES_PER_PAGE;
    ret = pipe_resize_ring(pipe, roundup_pow_of_two(nr_notes));
    if (ret < 0) {
// goto;
    }
//
// pipe_resize_ring() does not update nr_accounted for watch_queue
// pipes, because the above vastly overprovisions. Set nr_accounted on
// and max_usage this pipe to the number that was actually charged to
// the user above via account_pipe_buffers.
//
    pipe.max_usage = nr_pages;
    pipe.nr_accounted = nr_pages;
    ret = -ENOMEM;
    pages = kzalloc_objs(page *, nr_pages);
    if (!pages) {
// goto;
    }
    while (i < nr_pages) {
    pages[i] = alloc_page(GFP_KERNEL);
    if (!pages[i]) {
// goto;
    }
    pages[i].private = i * WATCH_QUEUE_NOTES_PER_PAGE;
    }
    bitmap = bitmap_alloc(nr_notes, GFP_KERNEL);
    if (!bitmap) {
// goto;
    }
    bitmap_fill(bitmap, nr_notes);
    wqueue.notes = pages;
    wqueue.notes_bitmap = bitmap;
    wqueue.nr_pages = nr_pages;
    wqueue.nr_notes = nr_notes;
    return 0;
// label;
    while (--i >= 0) {
    __free_page(pages[i]);
    }
    kfree(pages);
// label;
    (void) account_pipe_buffers(pipe.user, nr_pages, pipe.nr_accounted);
    return ret;
    }
//
// Set the filter on a watch queue.
//
#[no_mangle]
pub unsafe extern "C" fn watch_queue_set_filter(pipe: *mut pipe_inode_info, _filter: *mut watch_notification_filter) -> c_long {
pub static mut tf: *mut c_void = core::ptr::null_mut();
pub static mut filter: usize = 0;
pub static mut q: *mut c_void = core::ptr::null_mut();
pub static mut wfilter: *mut c_void = core::ptr::null_mut();
    let mut wqueue = pipe.watch_queue;
    int ret, nr_filter = 0, i;
    if (!wqueue) {
    return -ENODEV;
    }
    if (!_filter) {
// Remove the old filter
    wfilter = core::ptr::null_mut();
// goto;
    }
// Grab the user's filter specification
    if (copy_from_user(&filter, _filter, sizeof!(filter)) != 0) {
    return -EFAULT;
    }
    if (filter.nr_filters == 0 ||
    filter.nr_filters > 16 ||
    filter.__reserved != 0) {
    return -EINVAL;
    }
    tf = memdup_array_user(_filter.filters, filter.nr_filters, sizeof!(*tf));
    if (IS_ERR(tf)) {
    return PTR_ERR(tf);
    }
    ret = -EINVAL;
    while (i < filter.nr_filters) {
    if ((tf[i].info_filter & ~tf[i].info_mask) ||
    tf[i].info_mask & WATCH_INFO_LENGTH) {
// goto;
    }
// Ignore any unknown types
    if (tf[i].type >= WATCH_TYPE__NR) {
    continue;
    }
    nr_filter += 1;
    }
// Now we need to build the internal filter from only the relevant
// user-specified filters.
//
    ret = -ENOMEM;
    wfilter = kzalloc_flex(*wfilter, filters, nr_filter);
    if (!wfilter) {
// goto;
    }
    wfilter.nr_filters = nr_filter;
    q = wfilter.filters;
    while (i < filter.nr_filters) {
    if (tf[i].type >= WATCH_TYPE__NR) {
    continue;
    }
    q.type			= tf[i].type;
    q.info_filter		= tf[i].info_filter;
    q.info_mask		= tf[i].info_mask;
    q.subtype_filter[0]	= tf[i].subtype_filter[0];
    __set_bit(q.type, wfilter.type_filter);
    q += 1;
    }
    kfree(tf);
// label;
    pipe_lock(pipe);
    wfilter = rcu_replace_pointer(wqueue.filter, wfilter,
    lockdep_is_held(&pipe.mutex));
    pipe_unlock(pipe);
    if (wfilter) {
    kfree_rcu(wfilter, rcu);
    }
    return 0;
// label;
    kfree(tf);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __put_watch_queue(kref: *mut kref) {
    let mut wqueue = container_of!(kref, watch_queue, usage);
pub static mut wfilter: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    for (i = 0; i < wqueue.nr_pages; i++) {
    __free_page(wqueue.notes[i]);
    }
    kfree(wqueue.notes);
    bitmap_free(wqueue.notes_bitmap);
    wfilter = rcu_access_pointer(wqueue.filter);
    if (wfilter) {
    kfree_rcu(wfilter, rcu);
    }
    kfree_rcu(wqueue, rcu);
    }
//
// put_watch_queue - Dispose of a ref on a watchqueue.
// @wqueue: The watch queue to unref.
//
#[no_mangle]
pub unsafe extern "C" fn put_watch_queue(wqueue: *mut watch_queue) {
    kref_put(&wqueue.usage, __put_watch_queue);
    }
    EXPORT_SYMBOL(put_watch_queue);
#[no_mangle]
unsafe extern "C" fn free_watch(rcu: *mut rcu_head) {
    let mut watch = container_of!(rcu, watch, rcu);
    put_watch_queue(rcu_access_pointer(watch.queue));
    atomic_dec(&watch.cred.user.nr_watches);
    put_cred(watch.cred);
    kfree(watch);
    }
#[no_mangle]
unsafe extern "C" fn __put_watch(kref: *mut kref) {
    let mut watch = container_of!(kref, watch, usage);
    call_rcu(&watch.rcu, free_watch);
    }
//
// Discard a watch.
//
#[no_mangle]
unsafe extern "C" fn put_watch(watch: *mut watch) {
    kref_put(&watch.usage, __put_watch);
    }
//
// init_watch - Initialise a watch
// @watch: The watch to initialise.
// @wqueue: The queue to assign.
//
// Initialise a watch and set the watch queue.
//
#[no_mangle]
pub unsafe extern "C" fn init_watch(watch: *mut watch, wqueue: *mut watch_queue) {
    kref_init(&watch.usage);
    INIT_HLIST_NODE(&watch.list_node);
    INIT_HLIST_NODE(&watch.queue_node);
    rcu_assign_pointer(watch.queue, wqueue);
    }
#[no_mangle]
unsafe extern "C" fn add_one_watch(watch: *mut watch, wlist: *mut watch_list, wqueue: *mut watch_queue) -> c_int {
pub static mut cred: *mut c_void = core::ptr::null_mut();
pub static mut w: *mut c_void = core::ptr::null_mut();
    hlist_for_each_entry(w, &wlist.watchers, list_node) {
    let mut wq = rcu_access_pointer(w.queue);
    if (wqueue == wq && watch.id == w.id) {
    return -EBUSY;
    }
    }
    cred = current_cred();
    if (atomic_inc_return(&cred.user.nr_watches) > task_rlimit(current, RLIMIT_NOFILE)) {
    atomic_dec(&cred.user.nr_watches);
    return -EAGAIN;
    }
    watch.cred = get_cred(cred);
    rcu_assign_pointer(watch.watch_list, wlist);
    kref_get(&wqueue.usage);
    kref_get(&watch.usage);
    hlist_add_head(&watch.queue_node, &wqueue.watches);
    hlist_add_head_rcu(&watch.list_node, &wlist.watchers);
    return 0;
    }
//
// add_watch_to_object - Add a watch on an object to a watch list
// @watch: The watch to add
// @wlist: The watch list to add to
//
// @watch->queue must have been set to point to the queue to post notifications
// to and the watch list of the object to be watched.  @watch->cred must also
// have been set to the appropriate credentials and a ref taken on them.
//
// The caller must pin the queue and the list both and must hold the list
// locked against racing watch additions/removals.
//
#[no_mangle]
pub unsafe extern "C" fn add_watch_to_object(watch: *mut watch, wlist: *mut watch_list) -> c_int {
pub static mut wqueue: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    rcu_read_lock();
    wqueue = rcu_access_pointer(watch.queue);
    if (lock_wqueue(wqueue)) {
    spin_lock(&wlist.lock);
    ret = add_one_watch(watch, wlist, wqueue);
    spin_unlock(&wlist.lock);
    unlock_wqueue(wqueue);
    }
    rcu_read_unlock();
    return ret;
    }
    EXPORT_SYMBOL(add_watch_to_object);
//
// remove_watch_from_object - Remove a watch or all watches from an object.
// @wlist: The watch list to remove from
// @wq: The watch queue of interest (ignored if @all is true)
// @id: The ID of the watch to remove (ignored if @all is true)
// @all: True to remove all objects
//
// Remove a specific watch or all watches from an object.  A notification is
// sent to the watcher to tell them that this happened.
//
#[no_mangle]
pub unsafe extern "C" fn remove_watch_from_object(wlist: *mut watch_list, wq: *mut watch_queue, id: u64, all: bool) -> c_int {
pub static mut n: usize = 0;
pub static mut wqueue: *mut c_void = core::ptr::null_mut();
pub static mut watch: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    rcu_read_lock();
// label;
    spin_lock(&wlist.lock);
    hlist_for_each_entry(watch, &wlist.watchers, list_node) {
    if (all ||
    (watch.id == id && rcu_access_pointer(watch.queue) == wq)) {
// goto;
    }
    }
    spin_unlock(&wlist.lock);
// goto;
// label;
    ret = 0;
    hlist_del_init_rcu(&watch.list_node);
    rcu_assign_pointer(watch.watch_list, core::ptr::null_mut());
    spin_unlock(&wlist.lock);
// We now own the reference on watch that used to belong to wlist.
    n.watch.type = WATCH_TYPE_META;
    n.watch.subtype = WATCH_META_REMOVAL_NOTIFICATION;
    n.watch.info = watch.info_id | watch_sizeof(n.watch);
    n.id = id;
    if (id != 0) {
    n.watch.info = watch.info_id | watch_sizeof(n);
    }
    wqueue = rcu_dereference(watch.queue);
    if (lock_wqueue(wqueue)) {
    post_one_notification(wqueue, &n.watch);
    if (!hlist_unhashed(&watch.queue_node)) {
    hlist_del_init_rcu(&watch.queue_node);
    put_watch(watch);
    }
    unlock_wqueue(wqueue);
    }
    if (wlist.release_watch) {
    void (*release_watch);
    release_watch = wlist.release_watch;
    rcu_read_unlock();
    (*release_watch)(watch);
    rcu_read_lock();
    }
    put_watch(watch);
    if (all && !hlist_empty(&wlist.watchers)) {
// goto;
    }
// label;
    rcu_read_unlock();
    return ret;
    }
    EXPORT_SYMBOL(remove_watch_from_object);
//
// Remove all the watches that are contributory to a queue.  This has the
// potential to race with removal of the watches by the destruction of the
// objects being watched or with the distribution of notifications.
//
#[no_mangle]
pub unsafe extern "C" fn watch_queue_clear(wqueue: *mut watch_queue) {
pub static mut wlist: *mut c_void = core::ptr::null_mut();
pub static mut watch: *mut c_void = core::ptr::null_mut();
    let mut release = 0;
    rcu_read_lock();
    spin_lock_bh(&wqueue.lock);
//
// This pipe can be freed by callers like free_pipe_info().
// Removing this reference also prevents new notifications.
//
    wqueue.pipe = core::ptr::null_mut();
    while (!hlist_empty(&wqueue.watches)) {
    watch = hlist_entry(wqueue.watches.first, watch, queue_node);
    hlist_del_init_rcu(&watch.queue_node);
// We now own a ref on the watch.
    spin_unlock_bh(&wqueue.lock);
// We can't do the next bit under the queue lock as we need to
// get the list lock - which would cause a deadlock if someone
// was removing from the opposite direction at the same time or
// posting a notification.
//
    wlist = rcu_dereference(watch.watch_list);
    if (wlist) {
    void (*release_watch);
    spin_lock(&wlist.lock);
    release = !hlist_unhashed(&watch.list_node);
    if (release) {
    hlist_del_init_rcu(&watch.list_node);
    rcu_assign_pointer(watch.watch_list, core::ptr::null_mut());
// We now own a second ref on the watch.
    }
    release_watch = wlist.release_watch;
    spin_unlock(&wlist.lock);
    if (release) {
    if (release_watch) {
    rcu_read_unlock();
// This might need to call dput(), so
// we have to drop all the locks.
//
    (*release_watch)(watch);
    rcu_read_lock();
    }
    put_watch(watch);
    }
    }
    put_watch(watch);
    spin_lock_bh(&wqueue.lock);
    }
    spin_unlock_bh(&wqueue.lock);
    rcu_read_unlock();
    }
//
// get_watch_queue - Get a watch queue from its file descriptor.
// @fd: The fd to query.
//
#[no_mangle]
pub unsafe extern "C" fn get_watch_queue(fd: c_int) -> *mut c_void {
pub static mut pipe: *mut c_void = core::ptr::null_mut();
    let mut wqueue = ERR_PTR(-EINVAL);
    CLASS(fd, f)(fd);
    if (!fd_empty(f)) {
    pipe = get_pipe_info(fd_file(f), false);
    if (pipe && pipe.watch_queue) {
    wqueue = pipe.watch_queue;
    kref_get(&wqueue.usage);
    }
    }
    return wqueue;
    }
    EXPORT_SYMBOL(get_watch_queue);
//
// Initialise a watch queue
//
#[no_mangle]
pub unsafe extern "C" fn watch_queue_init(pipe: *mut pipe_inode_info) -> c_int {
pub static mut wqueue: *mut c_void = core::ptr::null_mut();
    wqueue = kzalloc_obj(*wqueue);
    if (!wqueue) {
    return -ENOMEM;
    }
    wqueue.pipe = pipe;
    kref_init(&wqueue.usage);
    spin_lock_init(&wqueue.lock);
    INIT_HLIST_HEAD(&wqueue.watches);
    pipe.watch_queue = wqueue;
    return 0;
    }
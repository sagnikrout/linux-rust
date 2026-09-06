//! Automatically rewritten from C to Rust
//! Source: kernel/trace/simple_ring_buffer.c
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
// Copyright (C) 2025 - Google LLC
// Author: Vincent Donnefort <vdonnefort@google.com>
//

    enum simple_rb_link_type {
    SIMPLE_RB_LINK_NORMAL		= 0,
    SIMPLE_RB_LINK_HEAD		= 1,
    SIMPLE_RB_LINK_HEAD_MOVING
    };

#[no_mangle]
unsafe extern "C" fn simple_bpage_set_head_link(bpage: *mut simple_buffer_page) {
pub static mut link: c_ulong = 0;
    link &= SIMPLE_RB_LINK_MASK;
    link |= SIMPLE_RB_LINK_HEAD;
//
// Paired with simple_rb_find_head() to order access between the head
// link and overrun. It ensures we always report an up-to-date value
// after swapping the reader page.
//
    smp_store_release(&bpage.link.next, link);
    }
#[no_mangle]
pub unsafe extern "C" fn simple_bpage_unset_head_link(bpage: *mut simple_buffer_page, dst: *mut simple_buffer_page, new_type: simple_rb_link_type) -> bool {
    let mut link = (&bpage.link.next);
pub static mut old: c_ulong = 0;
pub static mut new: c_ulong = 0;
    return try_cmpxchg(link, &old, new);
    }
#[no_mangle]
unsafe extern "C" fn simple_bpage_set_normal_link(bpage: *mut simple_buffer_page) {
pub static mut link: c_ulong = 0;
    WRITE_ONCE(bpage.link.next, (link & SIMPLE_RB_LINK_MASK));
    }
#[no_mangle]
pub unsafe extern "C" fn simple_bpage_from_link(link: *mut list_head) -> *mut c_void {
pub static mut ptr: c_ulong = 0;
    return container_of!(ptr, simple_buffer_page, link);
    }
#[no_mangle]
pub unsafe extern "C" fn simple_bpage_next_page(bpage: *mut simple_buffer_page) -> *mut c_void {
    return simple_bpage_from_link(bpage.link.next);
    }
#[no_mangle]
unsafe extern "C" fn simple_bpage_reset(bpage: *mut simple_buffer_page) {
    bpage.write = 0;
    bpage.entries = 0;
    local_set(&bpage.page.commit, 0);
    }
#[no_mangle]
unsafe extern "C" fn simple_bpage_init(bpage: *mut simple_buffer_page, page: *mut c_void) {
    INIT_LIST_HEAD(&bpage.link);
    bpage.page = page;
    simple_bpage_reset(bpage);
    }

    WRITE_ONCE((__meta), (__meta + __inc))
#[no_mangle]
unsafe extern "C" fn simple_rb_loaded(cpu_buffer: *mut simple_rb_per_cpu) -> bool {
    return !!cpu_buffer.bpages;
    }
#[no_mangle]
unsafe extern "C" fn simple_rb_find_head(cpu_buffer: *mut simple_rb_per_cpu) -> c_int {
pub static mut retry: c_int = 0;
pub static mut head: *mut c_void = core::ptr::null_mut();
    head = cpu_buffer.head_page;
    while (retry--) {
    let mut link = 0;
// label;
// See smp_store_release in simple_bpage_set_head_link()
    link = (unsigned long)smp_load_acquire(&head.link.prev.next);
    match (link & ~SIMPLE_RB_LINK_MASK) {
// Found the head
    SIMPLE_RB_LINK_HEAD => {
    cpu_buffer.head_page = head;
    return 0;
// The writer caught the head, we can spin, that won't be long
    }
    SIMPLE_RB_LINK_HEAD_MOVING => {
// goto;
    }
    }
    head = simple_bpage_next_page(head);
    }
    return -EBUSY;
    }
//
// simple_ring_buffer_swap_reader_page - Swap ring-buffer head with the reader
// @cpu_buffer: A simple_rb_per_cpu
//
// This function enables consuming reading. It ensures the current head page will not be overwritten
// and can be safely read.
//
// Returns 0 on success, -ENODEV if @cpu_buffer was unloaded or -EBUSY if we failed to catch the
// head page.
//
#[no_mangle]
pub unsafe extern "C" fn simple_ring_buffer_swap_reader_page(cpu_buffer: *mut simple_rb_per_cpu) -> c_int {
    let mut last = core::ptr::null_mut();
    let mut head = core::ptr::null_mut();
    let mut reader = core::ptr::null_mut();
    let mut overrun = 0;
pub static mut retry: c_int = 8;
    let mut ret = 0;
    if (!simple_rb_loaded(cpu_buffer)) {
    return -ENODEV;
    }
    reader = cpu_buffer.reader_page;
    do {
// Run after the writer to find the head
    ret = simple_rb_find_head(cpu_buffer);
    if (ret) {
    return ret;
    }
    head = cpu_buffer.head_page;
// Connect the reader page around the header page
    reader.link.next = head.link.next;
    reader.link.prev = head.link.prev;
// The last page before the head
    last = simple_bpage_from_link(head.link.prev);
// The reader page points to the new header page
    simple_bpage_set_head_link(reader);
    overrun = cpu_buffer.meta.overrun;
    } while (!simple_bpage_unset_head_link(last, reader, SIMPLE_RB_LINK_NORMAL) && retry--);
    if (retry < 0) {
    return -EBUSY;
    }
    cpu_buffer.head_page = simple_bpage_from_link(reader.link.next);
    cpu_buffer.head_page.link.prev = &reader.link;
    cpu_buffer.reader_page = head;
    cpu_buffer.meta.reader.lost_events = overrun - cpu_buffer.last_overrun;
    cpu_buffer.meta.reader.id = cpu_buffer.reader_page.id;
    cpu_buffer.last_overrun = overrun;
    return 0;
    }
    EXPORT_SYMBOL_GPL(simple_ring_buffer_swap_reader_page);
#[no_mangle]
pub unsafe extern "C" fn simple_rb_move_tail(cpu_buffer: *mut simple_rb_per_cpu) -> *mut c_void {
    let mut tail = core::ptr::null_mut();
    let mut new_tail = core::ptr::null_mut();
    tail = cpu_buffer.tail_page;
    new_tail = simple_bpage_next_page(tail);
    if (simple_bpage_unset_head_link(tail, new_tail, SIMPLE_RB_LINK_HEAD_MOVING)) {
//
// Oh no! we've caught the head. There is none anymore and
// swap_reader will spin until we set the new one. Overrun must
// be written first, to make sure we report the correct number
// of lost events.
//
    simple_rb_meta_inc(cpu_buffer.meta.overrun, new_tail.entries);
    simple_rb_meta_inc(cpu_buffer.meta.pages_lost, 1);
    simple_bpage_set_head_link(new_tail);
    simple_bpage_set_normal_link(tail);
    }
    simple_bpage_reset(new_tail);
    cpu_buffer.tail_page = new_tail;
    simple_rb_meta_inc(cpu_buffer.meta.pages_touched, 1);
    return new_tail;
    }
#[no_mangle]
unsafe extern "C" fn rb_event_size(length: c_ulong) -> c_ulong {
pub static mut event: *mut c_void = core::ptr::null_mut();
    return length + RB_EVNT_HDR_SIZE + sizeof!(event.array[0]);
    }
#[no_mangle]
pub unsafe extern "C" fn rb_event_add_ts_extend(event: *mut ring_buffer_event, delta: u64) -> *mut c_void {
    event.type_len = RINGBUF_TYPE_TIME_EXTEND;
    event.time_delta = delta & TS_MASK;
    event.array[0] = delta >> TS_SHIFT;
    return ((unsigned long)event + 8);
    }
#[no_mangle]
pub unsafe extern "C" fn simple_rb_reserve_next(cpu_buffer: *mut simple_rb_per_cpu, length: c_ulong, timestamp: u64) -> *mut c_void {
pub static mut ts_ext_size: c_ulong = 0;
    let mut tail = cpu_buffer.tail_page;
pub static mut event: *mut c_void = core::ptr::null_mut();
    u32 write, prev_write;
    let mut time_delta = 0;
    time_delta = timestamp - cpu_buffer.write_stamp;
    if (test_time_stamp(time_delta)) {
    ts_ext_size = 8;
    }
    prev_write = tail.write;
    write = prev_write + event_size + ts_ext_size;
    if (unlikely(write > (PAGE_SIZE - BUF_PAGE_HDR_SIZE))) {
    tail = simple_rb_move_tail(cpu_buffer);
    }
    if (!tail.entries) {
    tail.page.time_stamp = timestamp;
    time_delta = 0;
    ts_ext_size = 0;
    write = event_size;
    prev_write = 0;
    }
    tail.write = write;
    tail.entries += 1;
    cpu_buffer.write_stamp = timestamp;
    event = (tail.page.data + prev_write);
    if (ts_ext_size) {
    event = rb_event_add_ts_extend(event, time_delta);
    time_delta = 0;
    }
    event.type_len = 0;
    event.time_delta = time_delta;
    event.array[0] = event_size - RB_EVNT_HDR_SIZE;
    return event;
    }
//
// simple_ring_buffer_reserve - Reserve an entry in @cpu_buffer
// @cpu_buffer:	A simple_rb_per_cpu
// @length:	Size of the entry in bytes
// @timestamp:	Timestamp of the entry
//
// Returns the address of the entry where to write data or NULL
//
#[no_mangle]
pub unsafe extern "C" fn simple_ring_buffer_reserve(cpu_buffer: *mut simple_rb_per_cpu, length: c_ulong, timestamp: u64) -> *mut c_void {
pub static mut rb_event: *mut c_void = core::ptr::null_mut();
    if (cmpxchg(&cpu_buffer.status, SIMPLE_RB_READY, SIMPLE_RB_WRITING) != SIMPLE_RB_READY) {
    return core::ptr::null_mut();
    }
    rb_event = simple_rb_reserve_next(cpu_buffer, length, timestamp);
    return &rb_event.array[1];
    }
    EXPORT_SYMBOL_GPL(simple_ring_buffer_reserve);
//
// simple_ring_buffer_commit - Commit the entry reserved with simple_ring_buffer_reserve()
// @cpu_buffer:	The simple_rb_per_cpu where the entry has been reserved
//
#[no_mangle]
pub unsafe extern "C" fn simple_ring_buffer_commit(cpu_buffer: *mut simple_rb_per_cpu) {
    local_set(&cpu_buffer.tail_page.page.commit,
    cpu_buffer.tail_page.write);
    simple_rb_meta_inc(cpu_buffer.meta.entries, 1);
//
// Paired with simple_rb_enable_tracing() to ensure data is
// written to the ring-buffer before teardown.
//
    smp_store_release(&cpu_buffer.status, SIMPLE_RB_READY);
    }
    EXPORT_SYMBOL_GPL(simple_ring_buffer_commit);
#[no_mangle]
unsafe extern "C" fn simple_rb_enable_tracing(cpu_buffer: *mut simple_rb_per_cpu, enable: bool) -> u32 {
    let mut prev_status = 0;
    if (enable) {
    return cmpxchg(&cpu_buffer.status, SIMPLE_RB_UNAVAILABLE, SIMPLE_RB_READY);
    }
// Wait for the buffer to be released
    do {
    prev_status = cmpxchg_acquire(&cpu_buffer.status,
    SIMPLE_RB_READY,
    SIMPLE_RB_UNAVAILABLE);
    } while (prev_status == SIMPLE_RB_WRITING);
    return prev_status;
    }
//
// simple_ring_buffer_reset - Reset @cpu_buffer
// @cpu_buffer: A simple_rb_per_cpu
//
// This will not clear the content of the data, only reset counters and pointers
//
// Returns 0 on success or -ENODEV if @cpu_buffer was unloaded.
//
#[no_mangle]
pub unsafe extern "C" fn simple_ring_buffer_reset(cpu_buffer: *mut simple_rb_per_cpu) -> c_int {
pub static mut bpage: *mut c_void = core::ptr::null_mut();
    let mut prev_status = 0;
    let mut ret = 0;
    if (!simple_rb_loaded(cpu_buffer)) {
    return -ENODEV;
    }
    prev_status = simple_rb_enable_tracing(cpu_buffer, false);
    ret = simple_rb_find_head(cpu_buffer);
    if (ret) {
    return ret;
    }
    bpage = cpu_buffer.tail_page = cpu_buffer.head_page;
    do {
    simple_bpage_reset(bpage);
    bpage = simple_bpage_next_page(bpage);
    } while (bpage != cpu_buffer.head_page);
    simple_bpage_reset(cpu_buffer.reader_page);
    cpu_buffer.last_overrun = 0;
    cpu_buffer.write_stamp = 0;
    cpu_buffer.meta.reader.read = 0;
    cpu_buffer.meta.reader.lost_events = 0;
    cpu_buffer.meta.entries = 0;
    cpu_buffer.meta.overrun = 0;
    cpu_buffer.meta.read = 0;
    cpu_buffer.meta.pages_lost = 0;
    cpu_buffer.meta.pages_touched = 0;
    if (prev_status == SIMPLE_RB_READY) {
    simple_rb_enable_tracing(cpu_buffer, true);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(simple_ring_buffer_reset);
#[no_mangle]
pub unsafe extern "C" fn simple_ring_buffer_init_mm(cpu_buffer: *mut simple_rb_per_cpu, bpages: *mut simple_buffer_page, desc: *mut ring_buffer_desc) -> c_int {
    let mut bpage = bpages;
pub static mut ret: c_int = 0;
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
// At least 1 reader page and two pages in the ring-buffer
    if (desc.nr_page_va < 3) {
    return -EINVAL;
    }
    memset(cpu_buffer, 0, sizeof!(*cpu_buffer));
    cpu_buffer.meta = load_page(desc.meta_va);
    if (!cpu_buffer.meta) {
    return -EINVAL;
    }
    memset(cpu_buffer.meta, 0, sizeof!(*cpu_buffer.meta));
    cpu_buffer.meta.meta_page_size = PAGE_SIZE;
// The reader page is not part of the ring initially
    page = load_page(desc.page_va[0]);
    if (!page) {
    unload_page(cpu_buffer.meta);
    return -EINVAL;
    }
    simple_bpage_init(bpage, page);
    bpage.id = 0;
    cpu_buffer.nr_pages = 1;
    cpu_buffer.reader_page = bpage;
    cpu_buffer.tail_page = bpage + 1;
    cpu_buffer.head_page = bpage + 1;
    while (i < desc.nr_page_va) {
    page = load_page(desc.page_va[i]);
    if (!page) {
    ret = -EINVAL;
    break;
    }
    simple_bpage_init(++bpage, page);
    bpage.link.next = &(bpage + 1).link;
    bpage.link.prev = &(bpage - 1).link;
    bpage.id = i;
    cpu_buffer.nr_pages = i + 1;
    }
    if (ret) {
    for (i -= 1; i >= 0; i--) {
    unload_page(bpages[i].page);
    }
    unload_page(cpu_buffer.meta);
    return ret;
    }
    cpu_buffer.meta.nr_subbufs = cpu_buffer.nr_pages;
// Close the ring
    bpage.link.next = &cpu_buffer.tail_page.link;
    cpu_buffer.tail_page.link.prev = &bpage.link;
// The last init'ed page points to the head page
    simple_bpage_set_head_link(bpage);
    cpu_buffer.bpages = bpages;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __load_page(page: c_ulong) -> *mut c_void {
    return page;
    }
#[no_mangle]
pub unsafe extern "C" fn __unload_page(page: *mut c_void) { }
//
// simple_ring_buffer_init - Init @cpu_buffer based on @desc
// @cpu_buffer:	A simple_rb_per_cpu buffer to init, allocated by the caller.
// @bpages:	Array of simple_buffer_pages, with as many elements as @desc->nr_page_va
// @desc:	A ring_buffer_desc
//
// Returns 0 on success or -EINVAL if the content of @desc is invalid
//
#[no_mangle]
pub unsafe extern "C" fn simple_ring_buffer_init(cpu_buffer: *mut simple_rb_per_cpu, bpages: *mut simple_buffer_page, desc: *mut ring_buffer_desc) -> c_int {
    return simple_ring_buffer_init_mm(cpu_buffer, bpages, desc, __load_page, __unload_page);
    }
    EXPORT_SYMBOL_GPL(simple_ring_buffer_init);
#[no_mangle]
pub unsafe extern "C" fn simple_ring_buffer_unload_mm(cpu_buffer: *mut simple_rb_per_cpu) {
    let mut p = 0;
    if (!simple_rb_loaded(cpu_buffer)) {
    return;
    }
    simple_rb_enable_tracing(cpu_buffer, false);
    unload_page(cpu_buffer.meta);
    for (p = 0; p < cpu_buffer.nr_pages; p++) {
    unload_page(cpu_buffer.bpages[p].page);
    }
    cpu_buffer.bpages = core::ptr::null_mut();
    }
//
// simple_ring_buffer_unload - Prepare @cpu_buffer for deletion
// @cpu_buffer:	A simple_rb_per_cpu that will be deleted.
//
#[no_mangle]
pub unsafe extern "C" fn simple_ring_buffer_unload(cpu_buffer: *mut simple_rb_per_cpu) {
    return simple_ring_buffer_unload_mm(cpu_buffer, __unload_page);
    }
    EXPORT_SYMBOL_GPL(simple_ring_buffer_unload);
//
// simple_ring_buffer_enable_tracing - Enable or disable writing to @cpu_buffer
// @cpu_buffer: A simple_rb_per_cpu
// @enable:	True to enable tracing, False to disable it
//
// Returns 0 on success or -ENODEV if @cpu_buffer was unloaded
//
#[no_mangle]
pub unsafe extern "C" fn simple_ring_buffer_enable_tracing(cpu_buffer: *mut simple_rb_per_cpu, enable: bool) -> c_int {
    if (!simple_rb_loaded(cpu_buffer)) {
    return -ENODEV;
    }
    simple_rb_enable_tracing(cpu_buffer, enable);
    return 0;
    }
    EXPORT_SYMBOL_GPL(simple_ring_buffer_enable_tracing);
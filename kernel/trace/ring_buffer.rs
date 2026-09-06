//! Automatically rewritten from C to Rust
//! Source: kernel/trace/ring_buffer.c
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
// Generic ring buffer
//
// Copyright (C) 2008 Steven Rostedt <srostedt@redhat.com>
//

//
// The "absolute" timestamp in the buffer is only 59 bits.
// If a clock has the 5 MSBs set, it needs to be saved and
// reinserted.
//

// forward_decl: update_pages_handler;
pub const RING_BUFFER_META_MAGIC: c_uint = 0xBADFEED;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_buffer_meta {
    pub magic: c_int,
    pub struct_sizes: c_int,
    pub total_size: c_ulong,
    pub buffers_offset: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_buffer_cpu_meta {
    pub first_buffer: c_ulong,
    pub head_buffer: c_ulong,
    pub commit_buffer: c_ulong,
    pub subbuf_size: __u32,
    pub nr_subbufs: __u32,

    pub nr_invalid: __u32,
    pub entry_bytes: __u32,
    pub buffers: [c_int; 0],
}

//
// The ring buffer header is special. We must manually up keep it.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_print_entry_header(s: *mut trace_seq) -> c_int {
    trace_seq_puts(s, "# compressed entry header\n");
    trace_seq_puts(s, "\ttype_len    :    5 bits\n");
    trace_seq_puts(s, "\ttime_delta  :   27 bits\n");
    trace_seq_puts(s, "\tarray       :   32 bits\n");
    trace_seq_putc(s, '\n');
    trace_seq_printf(s, "\tpadding     : type == %d\n",
    RINGBUF_TYPE_PADDING);
    trace_seq_printf(s, "\ttime_extend : type == %d\n",
    RINGBUF_TYPE_TIME_EXTEND);
    trace_seq_printf(s, "\ttime_stamp : type == %d\n",
    RINGBUF_TYPE_TIME_STAMP);
    trace_seq_printf(s, "\tdata max type_len  == %d\n",
    RINGBUF_TYPE_DATA_TYPE_LEN_MAX);
    return !trace_seq_has_overflowed(s);
    }
//
// The ring buffer is made up of a list of pages. A separate list of pages is
// allocated for each CPU. A writer may only write to a buffer that is
// associated with the CPU it is currently executing on.  A reader may read
// from any per cpu buffer.
//
// The reader is special. For each per cpu buffer, the reader has its own
// reader page. When a reader has read the entire reader page, this reader
// page is swapped with another page in the ring buffer.
//
// Now, as long as the writer is off the reader page, the reader can do what
// ever it wants with that page. The writer will never write to that page
// again (as long as it is out of the ring buffer).
//
// Here's some silly ASCII art.
//
// +------+
// |reader|          RING BUFFER
// |page  |
// +------+        +---+   +---+   +---+
// |   |-->|   |-->|   |
// +---+   +---+   +---+
// ^               |
// |               |
// +---------------+
//
// +------+
// |reader|          RING BUFFER
// |page  |------------------v
// +------+        +---+   +---+   +---+
// |   |-->|   |-->|   |
// +---+   +---+   +---+
// ^               |
// |               |
// +---------------+
//
// +------+
// |reader|          RING BUFFER
// |page  |------------------v
// +------+        +---+   +---+   +---+
// ^            |   |-->|   |-->|   |
// |            +---+   +---+   +---+
// |                              |
// +------------------------------+
//
// +------+
// |buffer|          RING BUFFER
// |page  |------------------v
// +------+        +---+   +---+   +---+
// ^            |   |   |   |-->|   |
// |   New      +---+   +---+   +---+
// |  Reader------^               |
// |   page                       |
// +------------------------------+
//
// After we make this swap, the reader can hand this page off to the splice
// code and be done with it. It can even allocate a new page if it needs to
// and swap that into the ring buffer.
//
// We will be using cmpxchg soon to make all this lockless.
//
// Used for individual buffers (after the counter)

// define RINGBUF_TYPE_DATA for 'case RINGBUF_TYPE_DATA:'

    enum {
    RB_LEN_TIME_EXTEND = 8,
    RB_LEN_TIME_STAMP =  8,
    };

    ((event + RB_LEN_TIME_EXTEND))

    (event.type_len >= RINGBUF_TYPE_TIME_EXTEND)
#[no_mangle]
pub unsafe extern "C" fn rb_null_event(event: *mut ring_buffer_event) -> bool {
    return event.type_len == RINGBUF_TYPE_PADDING && !event.time_delta;
    }
#[no_mangle]
unsafe extern "C" fn rb_event_set_padding(event: *mut ring_buffer_event) {
// padding has a NULL time_delta
    event.type_len = RINGBUF_TYPE_PADDING;
    event.time_delta = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rb_event_data_length(event: *mut ring_buffer_event) -> c_uint {
    let mut length: c_uint = 0;
    if (event.type_len) {
    length = event.type_len * RB_ALIGNMENT;
    }
    else {
    length = event.array[0];
    }
    return length + RB_EVNT_HDR_SIZE;
    }
//
// Return the length of the given event. Will return
// the length of the time extend if the event is a
// time extend.
//
#[no_mangle]
pub unsafe extern "C" fn rb_event_length(event: *mut ring_buffer_event) -> c_uint {
    match (event.type_len) {
    RINGBUF_TYPE_PADDING => {
    if (rb_null_event(event)) {
// undefined
    return -1;
    }
    return  event.array[0] + RB_EVNT_HDR_SIZE;
    }
    RINGBUF_TYPE_TIME_EXTEND => {
    return RB_LEN_TIME_EXTEND;
    }
    RINGBUF_TYPE_TIME_STAMP => {
    return RB_LEN_TIME_STAMP;
    }
    RINGBUF_TYPE_DATA => {
    return rb_event_data_length(event);
    }
    _ => {
    WARN_ON_ONCE!(1);
    }
    }
// not hit
    return 0;
    }
//
// Return total length of time extend and data,
// or just the event length for all other events.
//
#[no_mangle]
pub unsafe extern "C" fn rb_event_ts_length(event: *mut ring_buffer_event) -> c_uint {
pub static mut len: unsigned = 0;
    if (extended_time(event)) {
// time extends include the data event after it
    len = RB_LEN_TIME_EXTEND;
    event = skip_time_extend(event);
    }
    return len + rb_event_length(event);
    }
//
// ring_buffer_event_length - return the length of the event
// @event: the event to get the length of
//
// Returns the size of the data load of a data event.
// If the event is something other than a data event, it
// returns the size of the event itself. With the exception
// of a TIME EXTEND, where it still returns the size of the
// data load of the data event after it.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_event_length(event: *mut ring_buffer_event) -> unsigned {
    let mut length: c_uint = 0;
    if (extended_time(event)) {
    event = skip_time_extend(event);
    }
    length = rb_event_length(event);
    if (event.type_len > RINGBUF_TYPE_DATA_TYPE_LEN_MAX) {
    return length;
    }
    length -= RB_EVNT_HDR_SIZE;
    if (length > RB_MAX_SMALL_DATA + sizeof!(event.array[0]) ||
    RB_FORCE_8BYTE_ALIGNMENT) {
    length -= sizeof!(event.array[0]);
    }
    return length;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_event_length);
// inline for ring buffer fast paths
    static __always_inline void *
    rb_event_data(ring_buffer_event *event)
    {
    if (extended_time(event)) {
    event = skip_time_extend(event);
    }
    WARN_ON_ONCE!(event.type_len > RINGBUF_TYPE_DATA_TYPE_LEN_MAX);
// If length is in len field, then array[0] has the data
    if (event.type_len) {
    return &event.array[0];
    }
// Otherwise length is in array[0] and array[1] has the data
    return &event.array[1];
    }
//
// ring_buffer_event_data - return the data of the event
// @event: the event to get the data from
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_event_data(event: *mut ring_buffer_event) -> *mut c_void {
    return rb_event_data(event);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_event_data);

    for_each_cpu(cpu, buffer.cpumask) {

    for_each_cpu_and(cpu, buffer.cpumask, cpu_online_mask)
#[no_mangle]
unsafe extern "C" fn rb_event_time_stamp(event: *mut ring_buffer_event) -> u64 {
    }
    let mut ts = 0;
    ts = event.array[0];
    ts <<= TS_SHIFT;
    ts += event.time_delta;
    return ts;
    }
// Flag when events were overwritten

// Missed count stored at end

#[repr(C)]
#[derive(Copy, Clone)]
pub struct buffer_data_read_page {
//     pub /: *mut *mut unsigned order; / order of the page,
//     pub /: *mut *mut *mut buffer_data_page data; / actual data, stored in this page,
}

//
// Note, the buffer_page list must be first. The buffer pages
// are allocated in cache lines, which means that each buffer
// page will be at the beginning of a cache line, and thus
// the least significant bits will be zero. We use this to
// add flags in the list struct pointers, to make the ring buffer
// lockless.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct buffer_page {
//     pub /: *mut *mut list_head list; / list of buffer pages,
//     pub /: *mut *mut local_t write; / index for next write,
//     pub /: *mut *mut unsigned read; / index for next read,
//     pub /: *mut *mut local_t entries; / entries on this page,
//     pub /: *mut *mut unsigned long real_end; / real end of data,
//     pub /: *mut *mut unsigned order; / order of the page,
//     pub /: *mut *mut u32 id:30; / ID for external mapping,
//     pub /: *mut *mut u32 range:1; / Mapped via a range,
//     pub /: *mut *mut *mut buffer_data_page page; / Actual data page,
}

//
// The buffer page counters, write and entries, must be reset
// atomically when crossing page boundaries. To synchronize this
// update, two counters are inserted into the number. One is
// the actual counter for the write position or count on the page.
//
// The other is a counter of updaters. Before an update happens
// the update partition of the counter is incremented. This will
// allow the updater to update the counter atomically.
//
// The counter is 20 bits, and the state data is 12.
//
pub const RB_WRITE_MASK: c_uint = 0xfffff;

#[no_mangle]
unsafe extern "C" fn rb_init_data_page(bpage: *mut buffer_data_page) {
    local_set(&bpage.commit, 0);
    bpage.time_stamp = 0;
    }
#[no_mangle]
unsafe extern "C" fn rb_data_page_commit(dpage: *mut buffer_data_page) -> __always_inline long {
    return local_read(&dpage.commit);
    }
#[no_mangle]
unsafe extern "C" fn rb_data_page_size(dpage: *mut buffer_data_page) -> __always_inline long {
    return rb_data_page_commit(dpage) & ~RB_MISSED_MASK;
    }
#[no_mangle]
unsafe extern "C" fn rb_page_commit(bpage: *mut buffer_page) -> __always_inline unsigned int {
    return rb_data_page_commit(bpage.page);
    }
#[no_mangle]
unsafe extern "C" fn rb_page_size(bpage: *mut buffer_page) -> __always_inline unsigned int {
    return rb_data_page_size(bpage.page);
    }
//
// rb_page_capacity - Get the capacity of a buffer page
// @bpage:	The buffer page
//
// Return: The maximum size available for events in the given buffer page.
//
#[no_mangle]
unsafe extern "C" fn rb_page_capacity(bpage: *mut buffer_page) -> __always_inline unsigned int {
    return (PAGE_SIZE << bpage.order) - BUF_PAGE_HDR_SIZE;
    }
#[no_mangle]
unsafe extern "C" fn free_buffer_page(bpage: *mut buffer_page) {
// Range pages are not to be freed
    if (!bpage.range) {
    free_pages((unsigned long)bpage.page, bpage.order);
    }
    kfree(bpage);
    }
//
// For best performance, allocate cpu buffer data cache line sized
// and per CPU.
//

    kzalloc_node(ALIGN(sizeof!(ring_buffer_per_cpu),		
    cache_line_size()), GFP_KERNEL, cpu_to_node(cpu))

    kzalloc_node(ALIGN(sizeof!(buffer_page),			
    cache_line_size()), GFP_KERNEL, cpu_to_node(cpu))
#[no_mangle]
pub unsafe extern "C" fn alloc_cpu_data(cpu: c_int, order: c_int) -> *mut c_void {
pub static mut dpage: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut mflags;
//
// __GFP_RETRY_MAYFAIL flag makes sure that the allocation fails
// gracefully without invoking oom-killer and the system is not
// destabilized.
//
    mflags = GFP_KERNEL | __GFP_RETRY_MAYFAIL | __GFP_COMP | __GFP_ZERO;
    page = alloc_pages_node(cpu_to_node(cpu), mflags, order);
    if (!page) {
    return core::ptr::null_mut();
    }
    dpage = page_address(page);
    rb_init_data_page(dpage);
    return dpage;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_irq_work {
    pub work: irq_work,
    pub waiters: wait_queue_head_t,
    pub full_waiters: wait_queue_head_t,
    pub seq: core::sync::atomic::AtomicI32,
    pub waiters_pending: bool,
    pub full_waiters_pending: bool,
    pub wakeup_full: bool,
}

//
// Structure to hold event state and handle nested events.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_event_info {
    pub ts: u64,
    pub delta: u64,
    pub before: u64,
    pub after: u64,
    pub length: c_ulong,
    pub tail_page: *mut buffer_page,
    pub add_timestamp: c_int,
}

//
// Used for the add_timestamp
// NONE
// EXTEND - wants a time extend
// ABSOLUTE - the buffer requests all events to have absolute time stamps
// FORCE - force a full time stamp.
//
    enum {
    RB_ADD_STAMP_NONE		= 0,
    RB_ADD_STAMP_EXTEND		= BIT(1),
    RB_ADD_STAMP_ABSOLUTE		= BIT(2),
    RB_ADD_STAMP_FORCE		= BIT(3)
    };
//
// Used for which event context the event is in.
// TRANSITION = 0
// NMI     = 1
// IRQ     = 2
// SOFTIRQ = 3
// NORMAL  = 4
//
// See trace_recursive_lock() comment below for more details.
//
    enum {
    RB_CTX_TRANSITION,
    RB_CTX_NMI,
    RB_CTX_IRQ,
    RB_CTX_SOFTIRQ,
    RB_CTX_NORMAL,
    RB_CTX_MAX
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_time_struct {
    pub time: local64_t,
}

    typedef struct rb_time_struct rb_time_t;
pub const MAX_NEST: c_int = 5;
//
// head_page == tail_page && head == tail then buffer is empty.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_buffer_per_cpu {
    pub cpu: c_int,
    pub record_disabled: core::sync::atomic::AtomicI32,
    pub resize_disabled: core::sync::atomic::AtomicI32,
    pub buffer: *mut trace_buffer,
//     pub /: *mut *mut raw_spinlock_t reader_lock; / serialize readers,
    pub lock: arch_spinlock_t,
    pub lock_key: lock_class_key,
    pub free_page: buffer_data_read_page,
    pub nr_pages: c_ulong,
    pub current_context: c_uint,
    pub pages: *mut list_head,
// pages generation counter, incremented when the list changes
    pub cnt: c_ulong,
//     pub /: *mut *mut *mut buffer_page head_page; / read from head,
//     pub /: *mut *mut *mut buffer_page tail_page; / write to tail,
//     pub /: *mut *mut *mut buffer_page commit_page; / committed pages,
    pub reader_page: *mut buffer_page,
    pub lost_events: c_ulong,
    pub last_overrun: c_ulong,
    pub nest: c_ulong,
    pub entries_bytes: local_t,
    pub entries: local_t,
    pub overrun: local_t,
    pub commit_overrun: local_t,
    pub dropped_events: local_t,
    pub committing: local_t,
    pub commits: local_t,
    pub pages_touched: local_t,
    pub pages_lost: local_t,
    pub pages_read: local_t,
    pub last_pages_touch: c_long,
    pub shortest_full: usize,
    pub read: c_ulong,
    pub read_bytes: c_ulong,
    pub write_stamp: rb_time_t,
    pub before_stamp: rb_time_t,
    pub event_stamp: [u64; MAX_NEST],
    pub read_stamp: u64,
// pages removed since last reset
    pub pages_removed: c_ulong,
//     pub /: *mut *mut unsigned int user_mapped; / user space mapping,
    pub mapping_lock: mutex,
//     pub /: *mut *mut *mut *mut buffer_page subbuf_ids; / ID to subbuf VA,
    pub meta_page: *mut trace_buffer_meta,
    pub ring_meta: *mut ring_buffer_cpu_meta,
    pub remote: *mut ring_buffer_remote,
// ring buffer pages to update, > 0 to add, < 0 to remove
    pub nr_pages_to_update: c_long,
//     pub /: *mut *mut list_head new_pages; / new pages to add,
    pub update_pages_work: work_struct,
    pub update_done: completion,
    pub irq_work: rb_irq_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_buffer {
    pub flags: unsigned,
    pub record_disabled: core::sync::atomic::AtomicI32,
    pub resizing: core::sync::atomic::AtomicI32,
    pub cpumask: cpumask_var_t,
    pub reader_lock_key: *mut lock_class_key,
    pub mutex: mutex,
    pub buffers: *mut ring_buffer_per_cpu,
    pub remote: *mut ring_buffer_remote,
    pub node: hlist_node,
    pub (*clock)(void): *mut u64,
    pub irq_work: rb_irq_work,
    pub time_stamp_abs: bool,
    pub range_addr_start: c_ulong,
    pub range_addr_end: c_ulong,
    pub flush_nb: notifier_block,
    pub meta: *mut ring_buffer_meta,
    pub subbuf_order: c_uint,
}

#[no_mangle]
unsafe extern "C" fn rb_subbuf_size(buffer: *mut trace_buffer) -> __always_inline unsigned int {
    return PAGE_SIZE << buffer.subbuf_order;
    }
//
// rb_subbuf_capacity - Get the capacity of a subbuffer
// @buffer:	A trace buffer
//
// Unsafe to use without holding trace_buffer::mutex or with resizing enabled.
// Consider rb_page_capacity() instead.
//
// Return: The maximum size available for events in a trace buffer subbuffer.
//
#[no_mangle]
unsafe extern "C" fn rb_subbuf_capacity(buffer: *mut trace_buffer) -> __always_inline unsigned int {
    return rb_subbuf_size(buffer) - BUF_PAGE_HDR_SIZE;
    }
//
// rb_subbuf_max_data_size - Get the maximum payload size of a single event
// @buffer:	A trace buffer
//
// Return: The maximum data payload size that can be stored in a single event.
//
#[no_mangle]
unsafe extern "C" fn rb_subbuf_max_data_size(buffer: *mut trace_buffer) -> __always_inline unsigned int {
pub static mut event: *mut c_void = core::ptr::null_mut();
//
// surely rb_subbuf_capacity() is bigger than
// RINGBUF_TYPE_DATA_TYPE_LEN_MAX (see ring_buffer_event_length).
//
    return rb_subbuf_capacity(buffer) - RB_EVNT_HDR_SIZE - sizeof!(event.array[0]);
    }
//
// rb_subbuf_start - Get the start address of a subbuffer
// @buffer:	A trace buffer
// @addr:	An address of an event on a subbuffer
//
// Return: The start of the subbuffer for where @addr sits
//
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn rb_subbuf_start(buffer: *mut trace_buffer, addr: c_ulong) -> c_ulong {
    return addr & ~((unsigned long)(rb_subbuf_size(buffer) - 1));
    }
#[no_mangle]
unsafe extern "C" fn rb_is_static(cpu_buffer: *mut ring_buffer_per_cpu) -> bool {
    return cpu_buffer.user_mapped || cpu_buffer.remote || cpu_buffer.ring_meta;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_buffer_iter {
    pub cpu_buffer: *mut ring_buffer_per_cpu,
    pub head: c_ulong,
    pub next_event: c_ulong,
    pub head_page: *mut buffer_page,
    pub cache_reader_page: *mut buffer_page,
    pub cache_read: c_ulong,
    pub cache_pages_removed: c_ulong,
    pub read_stamp: u64,
    pub page_stamp: u64,
    pub event: *mut ring_buffer_event,
    pub event_size: usize,
    pub missed_events: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn ring_buffer_print_page_header(buffer: *mut trace_buffer, s: *mut trace_seq) -> c_int {
pub static mut field: usize = 0;
    trace_seq_printf(s, "\tfield: u64 timestamp;\t"
    "offset:0;\tsize:%u;\tsigned:%u;\n",
    (unsigned int)sizeof!(field.time_stamp),
    (unsigned int)is_signed_type(u64));
    trace_seq_printf(s, "\tfield: local_t commit;\t"
    "offset:%u;\tsize:%u;\tsigned:%u;\n",
    (unsigned int)offsetof(typeof(field), commit),
    (unsigned int)sizeof!(field.commit),
    (unsigned int)is_signed_type(long));
    trace_seq_printf(s, "\tfield: char overwrite;\t"
    "offset:%u;\tsize:%u;\tsigned:%u;\n",
    (unsigned int)offsetof(typeof(field), commit),
    1,
    (unsigned int)is_signed_type(char));
    trace_seq_printf(s, "\tfield: char data;\t"
    "offset:%u;\tsize:%u;\tsigned:%u;\n",
    (unsigned int)offsetof(typeof(field), data),
    (unsigned int)(buffer ? rb_subbuf_capacity(buffer) :
    PAGE_SIZE - BUF_PAGE_HDR_SIZE),
    (unsigned int)is_signed_type(char));
    return !trace_seq_has_overflowed(s);
    }
#[no_mangle]
pub unsafe extern "C" fn rb_time_read(t: *mut rb_time_t, ret: *mut u64) {
// ret = local64_read(&t->time);
    }
#[no_mangle]
unsafe extern "C" fn rb_time_set(t: *mut rb_time_t, val: u64) {
    local64_set(&t.time, val);
    }
//
// Enable this to make sure that the event passed to
// ring_buffer_event_time_stamp() is not committed and also
// is on the buffer that it passed in.
//
// #define RB_VERIFY_EVENT

// forward_decl: rb_list_head;
#[no_mangle]
pub unsafe extern "C" fn verify_event(cpu_buffer: *mut ring_buffer_per_cpu, event: *mut c_void) {
    let mut page = cpu_buffer.commit_page;
    let mut tail_page = READ_ONCE(cpu_buffer.tail_page);
pub static mut next: *mut c_void = core::ptr::null_mut();
    let mut commit = 0;
    let mut write = 0;
pub static mut addr: c_ulong = 0;
pub static mut done: bool = false;
pub static mut stop: c_int = 0;
// Make sure the event exists and is not committed yet
    do {
    if (page == tail_page || WARN_ON_ONCE!(stop++ > 100)) {
    done = true;
    }
    commit = rb_page_commit(page);
    write = local_read(&page.write);
    if (addr >= (unsigned long)&page.page.data[commit] &&
    addr < (unsigned long)&page.page.data[write]) {
    return;
    }
    next = rb_list_head(page.list.next);
    page = list_entry(next, buffer_page, list);
    } while (!done);
    WARN_ON_ONCE!(1);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: verify_event
pub unsafe extern "C" fn verify_event_dup(cpu_buffer: *mut ring_buffer_per_cpu, event: *mut c_void) {
    }

//
// The absolute time stamp drops the 5 MSBs and some clocks may
// require them. The rb_fix_abs_ts() will take a previous full
// time stamp, and add the 5 MSB of that time stamp on to the
// saved absolute time stamp. Then they are compared in case of
// the unlikely event that the latest time stamp incremented
// the 5 MSB.
//
#[no_mangle]
pub unsafe extern "C" fn rb_fix_abs_ts(abs: u64, save_ts: u64) -> u64 {
    if (save_ts & TS_MSB) {
    abs |= save_ts & TS_MSB;
// Check for overflow
    if (unlikely(abs < save_ts)) {
    abs += 1ULL << 59;
    }
    }
    return abs;
    }
// forward_decl: rb_time_stamp;
//
// ring_buffer_event_time_stamp - return the event's current time stamp
// @buffer: The buffer that the event is on
// @event: the event to get the time stamp of
//
// Note, this must be called after @event is reserved, and before it is
// committed to the ring buffer. And must be called from the same
// context where the event was reserved (normal, softirq, irq, etc).
//
// Returns the time stamp associated with the current event.
// If the event has an extended time stamp, then that is used as
// the time stamp to return.
// In the highly unlikely case that the event was nested more than
// the max nesting, then the write_stamp of the buffer is returned,
// otherwise  current time is returned, but that really neither of
// the last two cases should ever happen.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_event_time_stamp(buffer: *mut trace_buffer, event: *mut ring_buffer_event) -> u64 {
    let mut cpu_buffer = buffer.buffers[smp_processor_id()];
    let mut nest = 0;
    let mut ts = 0;
// If the event includes an absolute time, then just use that
    if (event.type_len == RINGBUF_TYPE_TIME_STAMP) {
    ts = rb_event_time_stamp(event);
    return rb_fix_abs_ts(ts, cpu_buffer.tail_page.page.time_stamp);
    }
    nest = local_read(&cpu_buffer.committing);
    verify_event(cpu_buffer, event);
    if (WARN_ON_ONCE!(!nest)) {
// goto;
    }
// Read the current saved nesting level time stamp
    if (likely(--nest < MAX_NEST)) {
    return cpu_buffer.event_stamp[nest];
    }
// Shouldn't happen, warn if it does
    WARN_ONCE(1, "nest (%d) greater than max", nest);
// label;
    rb_time_read(&cpu_buffer.write_stamp, &ts);
    return ts;
    }
//
// ring_buffer_nr_dirty_pages - get the number of used pages in the ring buffer
// @buffer: The ring_buffer to get the number of pages from
// @cpu: The cpu of the ring_buffer to get the number of pages from
//
// Returns the number of pages that have content in the ring buffer.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_nr_dirty_pages(buffer: *mut trace_buffer, cpu: c_int) -> usize {
    let mut read = 0;
    let mut lost = 0;
    let mut cnt = 0;
    read = local_read(&buffer.buffers[cpu].pages_read);
    lost = local_read(&buffer.buffers[cpu].pages_lost);
    cnt = local_read(&buffer.buffers[cpu].pages_touched);
    if (WARN_ON_ONCE!(cnt < lost)) {
    return 0;
    }
    cnt -= lost;
// The reader can read an empty page, but not more than that
    if (cnt < read) {
    WARN_ON_ONCE!(read > cnt + 1);
    return 0;
    }
    return cnt - read;
    }
#[no_mangle]
unsafe extern "C" fn full_hit(buffer: *mut trace_buffer, cpu: c_int, full: c_int) -> __always_inline bool {
    let mut cpu_buffer = buffer.buffers[cpu];
    let mut nr_pages = 0;
    let mut dirty = 0;
    nr_pages = cpu_buffer.nr_pages;
    if (!nr_pages || !full) {
    return true;
    }
//
// Add one as dirty will never equal nr_pages, as the sub-buffer
// that the writer is on is not counted as dirty.
// This is needed if "buffer_percent" is set to 100.
//
    dirty = ring_buffer_nr_dirty_pages(buffer, cpu) + 1;
    return (dirty * 100) >= (full * nr_pages);
    }
//
// rb_wake_up_waiters - wake up tasks waiting for ring buffer input
//
// Schedules a delayed work to wake up any task that is blocked on the
// ring buffer waiters queue.
//
#[no_mangle]
unsafe extern "C" fn rb_wake_up_waiters(work: *mut irq_work) {
    let mut rbwork = container_of!(work, rb_irq_work, work);
// For waiters waiting for the first wake up
    (void)atomic_fetch_inc_release(&rbwork.seq);
    wake_up_all(&rbwork.waiters);
    if (rbwork.full_waiters_pending || rbwork.wakeup_full) {
// Only cpu_buffer sets the above flags
    let mut cpu_buffer = container_of!(rbwork, ring_buffer_per_cpu, irq_work);
// Called from interrupt context
    raw_spin_lock(&cpu_buffer.reader_lock);
    rbwork.wakeup_full = false;
    rbwork.full_waiters_pending = false;
// Waking up all waiters, they will reset the shortest full
    cpu_buffer.shortest_full = 0;
    raw_spin_unlock(&cpu_buffer.reader_lock);
    wake_up_all(&rbwork.full_waiters);
    }
    }
//
// ring_buffer_wake_waiters - wake up any waiters on this ring buffer
// @buffer: The ring buffer to wake waiters on
// @cpu: The CPU buffer to wake waiters on
//
// In the case of a file that represents a ring buffer is closing,
// it is prudent to wake up any waiters that are on this.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_wake_waiters(buffer: *mut trace_buffer, cpu: c_int) {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
pub static mut rbwork: *mut c_void = core::ptr::null_mut();
    if (!buffer) {
    return;
    }
    if (cpu == RING_BUFFER_ALL_CPUS) {
// Wake up individual ones too. One level recursion
    for_each_buffer_cpu(buffer, cpu) {
    ring_buffer_wake_waiters(buffer, cpu);
    }
    rbwork = &buffer.irq_work;
    } else {
    if (WARN_ON_ONCE!(!buffer.buffers)) {
    return;
    }
    if (WARN_ON_ONCE!(cpu >= nr_cpu_ids)) {
    return;
    }
    cpu_buffer = buffer.buffers[cpu];
// The CPU buffer may not have been initialized yet
    if (!cpu_buffer) {
    return;
    }
    rbwork = &cpu_buffer.irq_work;
    }
// This can be called in any context
    irq_work_queue(&rbwork.work);
    }
#[no_mangle]
unsafe extern "C" fn rb_watermark_hit(buffer: *mut trace_buffer, cpu: c_int, full: c_int) -> bool {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
pub static mut ret: bool = false;
// Reads of all CPUs always waits for any data
    if (cpu == RING_BUFFER_ALL_CPUS) {
    return !ring_buffer_empty(buffer);
    }
    cpu_buffer = buffer.buffers[cpu];
    if (!ring_buffer_empty_cpu(buffer, cpu)) {
    let mut flags = 0;
    let mut pagebusy = 0;
    if (!full) {
    return true;
    }
    raw_spin_lock_irqsave(&cpu_buffer.reader_lock, flags);
    pagebusy = cpu_buffer.reader_page == cpu_buffer.commit_page;
    ret = !pagebusy && full_hit(buffer, cpu, full);
    if (!ret && (!cpu_buffer.shortest_full ||
    cpu_buffer.shortest_full > full)) {
    cpu_buffer.shortest_full = full;
    }
    raw_spin_unlock_irqrestore(&cpu_buffer.reader_lock, flags);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn rb_wait_cond(rbwork: *mut rb_irq_work, buffer: *mut trace_buffer, cpu: c_int, full: c_int, cond: ring_buffer_cond_fn, data: *mut c_void) -> bool {
    if (rb_watermark_hit(buffer, cpu, full)) {
    return true;
    }
    if (cond(data)) {
    return true;
    }
//
// The events can happen in critical sections where
// checking a work queue can cause deadlocks.
// After adding a task to the queue, this flag is set
// only to notify events to try to wake up the queue
// using irq_work.
//
// We don't clear it even if the buffer is no longer
// empty. The flag only causes the next event to run
// irq_work to do the work queue wake up. The worse
// that can happen if we race with !trace_empty() is that
// an event will cause an irq_work to try to wake up
// an empty queue.
//
// There's no reason to protect this flag either, as
// the work queue and irq_work logic will do the necessary
// synchronization for the wake ups. The only thing
// that is necessary is that the wake up happens after
// a task has been queued. It's OK for spurious wake ups.
//
    if (full) {
    rbwork.full_waiters_pending = true;
    }
    else {
    rbwork.waiters_pending = true;
    }
    return false;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_wait_data {
    pub irq_work: *mut rb_irq_work,
    pub seq: c_int,
}

//
// The default wait condition for ring_buffer_wait() is to just to exit the
// wait loop the first time it is woken up.
//
#[no_mangle]
unsafe extern "C" fn rb_wait_once(data: *mut c_void) -> bool {
    let mut rdata = data;
    let mut rbwork = rdata.irq_work;
    return atomic_read_acquire(&rbwork.seq) != rdata.seq;
    }
//
// ring_buffer_wait - wait for input to the ring buffer
// @buffer: buffer to wait on
// @cpu: the cpu buffer to wait on
// @full: wait until the percentage of pages are available, if @cpu != RING_BUFFER_ALL_CPUS
// @cond: condition function to break out of wait (NULL to run once)
// @data: the data to pass to @cond.
//
// If @cpu == RING_BUFFER_ALL_CPUS then the task will wake up as soon
// as data is added to any of the @buffer's cpu buffers. Otherwise
// it will wait for data to be added to a specific cpu buffer.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_wait(buffer: *mut trace_buffer, cpu: c_int, full: c_int, cond: ring_buffer_cond_fn, data: *mut c_void) -> c_int {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
pub static mut waitq: *mut c_void = core::ptr::null_mut();
pub static mut rbwork: *mut c_void = core::ptr::null_mut();
pub static mut rdata: usize = 0;
pub static mut ret: c_int = 0;
//
// Depending on what the caller is waiting for, either any
// data in any cpu buffer, or a specific buffer, put the
// caller on the appropriate wait queue.
//
    if (cpu == RING_BUFFER_ALL_CPUS) {
    rbwork = &buffer.irq_work;
// Full only makes sense on per cpu reads
    full = 0;
    } else {
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return -ENODEV;
    }
    cpu_buffer = buffer.buffers[cpu];
    rbwork = &cpu_buffer.irq_work;
    }
    if (full) {
    waitq = &rbwork.full_waiters;
    }
    else {
    waitq = &rbwork.waiters;
    }
// Set up to exit loop as soon as it is woken
    if (!cond) {
    cond = rb_wait_once;
    rdata.irq_work = rbwork;
    rdata.seq = atomic_read_acquire(&rbwork.seq);
    data = &rdata;
    }
    ret = wait_event_interruptible((*waitq),
    rb_wait_cond(rbwork, buffer, cpu, full, cond, data));
    return ret;
    }
//
// ring_buffer_poll_wait - poll on buffer input
// @buffer: buffer to wait on
// @cpu: the cpu buffer to wait on
// @filp: the file descriptor
// @poll_table: The poll descriptor
// @full: wait until the percentage of pages are available, if @cpu != RING_BUFFER_ALL_CPUS
//
// If @cpu == RING_BUFFER_ALL_CPUS then the task will wake up as soon
// as data is added to any of the @buffer's cpu buffers. Otherwise
// it will wait for data to be added to a specific cpu buffer.
//
// Returns EPOLLIN | EPOLLRDNORM if data exists in the buffers,
// zero otherwise.
//
    __poll_t ring_buffer_poll_wait(trace_buffer *buffer, int cpu, file *filp, poll_table *poll_table, int full)
    {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
pub static mut rbwork: *mut c_void = core::ptr::null_mut();
    if (cpu == RING_BUFFER_ALL_CPUS) {
    rbwork = &buffer.irq_work;
    full = 0;
    } else {
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return EPOLLERR;
    }
    cpu_buffer = buffer.buffers[cpu];
    rbwork = &cpu_buffer.irq_work;
    }
    if (full) {
    poll_wait(filp, &rbwork.full_waiters, poll_table);
    if (rb_watermark_hit(buffer, cpu, full)) {
    return EPOLLIN | EPOLLRDNORM;
    }
//
// Only allow full_waiters_pending update to be seen after
// the shortest_full is set (in rb_watermark_hit). If the
// writer sees the full_waiters_pending flag set, it will
// compare the amount in the ring buffer to shortest_full.
// If the amount in the ring buffer is greater than the
// shortest_full percent, it will call the irq_work handler
// to wake up this list. The irq_handler will reset shortest_full
// back to zero. That's done under the reader_lock, but
// the below smp_mb() makes sure that the update to
// full_waiters_pending doesn't leak up into the above.
//
    smp_mb();
    rbwork.full_waiters_pending = true;
    return 0;
    }
    poll_wait(filp, &rbwork.waiters, poll_table);
    rbwork.waiters_pending = true;
//
// There's a tight race between setting the waiters_pending and
// checking if the ring buffer is empty.  Once the waiters_pending bit
// is set, the next event will wake the task up, but we can get stuck
// if there's only a single event in.
//
// FIXME: Ideally, we need a memory barrier on the writer side as well,
// but adding a memory barrier to all events will cause too much of a
// performance hit in the fast path.  We only need a memory barrier when
// the buffer goes from empty to having content.  But as this race is
// extremely small, and it's not a problem if another event comes in, we
// will fix it later.
//
    smp_mb();
    if ((cpu == RING_BUFFER_ALL_CPUS && !ring_buffer_empty(buffer)) ||
    (cpu != RING_BUFFER_ALL_CPUS && !ring_buffer_empty_cpu(buffer, cpu))) {
    return EPOLLIN | EPOLLRDNORM;
    }
    return 0;
    }
// buffer may be either ring_buffer or ring_buffer_per_cpu

    ({								
    let mut _____ret = unlikely(cond);				
    if (_____ret) {						
    if (__same_type(*(b), ring_buffer_per_cpu)) { 
    let mut __b = b;			
    atomic_inc(&__b.buffer.record_disabled); 
    } else {
    atomic_inc(&b.record_disabled);	
    }
    WARN_ON!(1);					
    }							
    _____ret;						
    })
// Up this if you want to test the TIME_EXTENTS and normalization
pub const DEBUG_SHIFT: c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn rb_time_stamp(buffer: *mut trace_buffer) -> u64 {
    let mut ts = 0;
// Skip retpolines :-(
    if (IS_ENABLED!(CONFIG_MITIGATION_RETPOLINE) && likely(buffer.clock == trace_clock_local)) {
    ts = trace_clock_local();
    }
    else {
    ts = buffer.clock();
    }
// shift to debug/test normalization and TIME_EXTENTS
    return ts << DEBUG_SHIFT;
    }
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_time_stamp(buffer: *mut trace_buffer) -> u64 {
    let mut time = 0;
    preempt_disable_notrace();
    time = rb_time_stamp(buffer);
    preempt_enable_notrace();
    return time;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_time_stamp);
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_normalize_time_stamp(buffer: *mut trace_buffer, cpu: c_int, ts: *mut u64) {
// Just stupid testing the normalize function and deltas
// ts >>= DEBUG_SHIFT;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_normalize_time_stamp);
//
// Making the ring buffer lockless makes things tricky.
// Although writes only happen on the CPU that they are on,
// and they only need to worry about interrupts. Reads can
// happen on any CPU.
//
// The reader page is always off the ring buffer, but when the
// reader finishes with a page, it needs to swap its page with
// a new one from the buffer. The reader needs to take from
// the head (writes go to the tail). But if a writer is in overwrite
// mode and wraps, it must push the head page forward.
//
// Here lies the problem.
//
// The reader must be careful to replace only the head page, and
// not another one. As described at the top of the file in the
// ASCII art, the reader sets its old page to point to the next
// page after head. It then sets the page after head to point to
// the old reader page. But if the writer moves the head page
// during this operation, the reader could end up with the tail.
//
// We use cmpxchg to help prevent this race. We also do something
// special with the page before head. We set the LSB to 1.
//
// When the writer must push the page forward, it will clear the
// bit that points to the head page, move the head, and then set
// the bit that points to the new head page.
//
// We also don't want an interrupt coming in and moving the head
// page on another writer. Thus we use the second LSB to catch
// that too. Thus:
//
// head->list->prev->next        bit 1          bit 0
// -------        -------
// Normal page                     0              0
// Points to head page             0              1
// New head page                   1              0
//
// Note we can not trust the prev pointer of the head page, because:
//
// +----+       +-----+        +-----+
// |    |------>|  T  |---X--->|  N  |
// |    |<------|     |        |     |
// +----+       +-----+        +-----+
// ^                           ^ |
// |          +-----+          | |
// +----------|  R  |----------+ |
// |     |<-----------+
// +-----+
//
// Key:  ---X-->  HEAD flag set in pointer
// T      Tail page
// R      Reader page
// N      Next page
//
// (see __rb_reserve_next() to see where this happens)
//
// What the above shows is that the reader just swapped out
// the reader page with a page in the buffer, but before it
// could make the new header point back to the new page added
// it was preempted by a writer. The writer moved forward onto
// the new page added by the reader and is about to move forward
// again.
//
// You can see, it is legitimate for the previous pointer of
// the head (or any page) not to point back to itself. But only
// temporarily.
//

// PAGE_MOVED is not part of the mask

//
// rb_list_head - remove any bit
//
#[no_mangle]
pub unsafe extern "C" fn rb_list_head(list: *mut list_head) -> *mut c_void {
pub static mut val: c_ulong = 0;
    return (val & ~RB_FLAG_MASK);
    }
//
// rb_is_head_page - test if the given page is the head page
//
// Because the reader may move the head_page pointer, we can
// not trust what the head page is (it may be pointing to
// the reader page). But if the next page is a header page,
// its flags will be non zero.
//
#[no_mangle]
pub unsafe extern "C" fn rb_is_head_page(page: *mut buffer_page, list: *mut list_head) -> c_int {
    let mut val = 0;
    val = (unsigned long)list.next;
    if ((val & ~RB_FLAG_MASK) != (unsigned long)&page.list) {
    return RB_PAGE_MOVED;
    }
    return val & RB_FLAG_MASK;
    }
//
// rb_is_reader_page
//
// The unique thing about the reader page, is that, if the
// writer is ever on it, the previous pointer never points
// back to the reader page.
//
#[no_mangle]
unsafe extern "C" fn rb_is_reader_page(page: *mut buffer_page) -> bool {
    let mut list = page.list.prev;
    return rb_list_head(list.next) != &page.list;
    }
//
// rb_set_list_to_head - set a list_head to be pointing to head.
//
#[no_mangle]
unsafe extern "C" fn rb_set_list_to_head(list: *mut list_head) {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    ptr = &list.next;
// ptr |= RB_PAGE_HEAD;
// ptr &= ~RB_PAGE_UPDATE;
    }
//
// rb_head_page_activate - sets up head page
//
#[no_mangle]
unsafe extern "C" fn rb_head_page_activate(cpu_buffer: *mut ring_buffer_per_cpu) {
pub static mut head: *mut c_void = core::ptr::null_mut();
    head = cpu_buffer.head_page;
    if (!head) {
    return;
    }
//
// Set the previous list pointer to have the HEAD flag.
//
    rb_set_list_to_head(head.list.prev);
    if (cpu_buffer.ring_meta) {
    let mut meta = cpu_buffer.ring_meta;
    meta.head_buffer = (unsigned long)head.page;
    }
    }
#[no_mangle]
unsafe extern "C" fn rb_list_head_clear(list: *mut list_head) {
    let mut ptr = &list.next;
// ptr &= ~RB_FLAG_MASK;
    }
//
// rb_head_page_deactivate - clears head page ptr (for free list)
//
#[no_mangle]
pub unsafe extern "C" fn rb_head_page_deactivate(cpu_buffer: *mut ring_buffer_per_cpu) {
pub static mut hd: *mut c_void = core::ptr::null_mut();
// Go through the whole list and clear any pointers found.
    rb_list_head_clear(cpu_buffer.pages);
    list_for_each(hd, cpu_buffer.pages) {
    rb_list_head_clear(hd);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rb_head_page_set(cpu_buffer: *mut ring_buffer_per_cpu, head: *mut buffer_page, prev: *mut buffer_page, old_flag: c_int, new_flag: c_int) -> c_int {
pub static mut list: *mut c_void = core::ptr::null_mut();
pub static mut val: c_ulong = 0;
    let mut ret = 0;
    list = &prev.list;
    val &= ~RB_FLAG_MASK;
    ret = cmpxchg(&list.next,
    val | old_flag, val | new_flag);
// check if the reader took the page
    if ((ret & ~RB_FLAG_MASK) != val) {
    return RB_PAGE_MOVED;
    }
    return ret & RB_FLAG_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn rb_head_page_set_update(cpu_buffer: *mut ring_buffer_per_cpu, head: *mut buffer_page, prev: *mut buffer_page, old_flag: c_int) -> c_int {
    return rb_head_page_set(cpu_buffer, head, prev,
    old_flag, RB_PAGE_UPDATE);
    }
#[no_mangle]
pub unsafe extern "C" fn rb_head_page_set_head(cpu_buffer: *mut ring_buffer_per_cpu, head: *mut buffer_page, prev: *mut buffer_page, old_flag: c_int) -> c_int {
    return rb_head_page_set(cpu_buffer, head, prev,
    old_flag, RB_PAGE_HEAD);
    }
#[no_mangle]
pub unsafe extern "C" fn rb_head_page_set_normal(cpu_buffer: *mut ring_buffer_per_cpu, head: *mut buffer_page, prev: *mut buffer_page, old_flag: c_int) -> c_int {
    return rb_head_page_set(cpu_buffer, head, prev,
    old_flag, RB_PAGE_NORMAL);
    }
#[no_mangle]
pub unsafe extern "C" fn rb_inc_page(bpage: *mut buffer_page) {
    let mut p = rb_list_head((*bpage).list.next);
// bpage = list_entry(p, buffer_page, list);
    }
#[no_mangle]
pub unsafe extern "C" fn rb_dec_page(bpage: *mut buffer_page) {
    let mut p = rb_list_head((*bpage).list.prev);
// bpage = list_entry(p, buffer_page, list);
    }
#[no_mangle]
pub unsafe extern "C" fn rb_set_head_page(cpu_buffer: *mut ring_buffer_per_cpu) -> *mut c_void {
pub static mut head: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut list: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (RB_WARN_ON(cpu_buffer, !cpu_buffer.head_page)) {
    return core::ptr::null_mut();
    }
// sanity check
    list = cpu_buffer.pages;
    if (RB_WARN_ON(cpu_buffer, rb_list_head(list.prev.next) != list)) {
    return core::ptr::null_mut();
    }
    page = head = cpu_buffer.head_page;
//
// It is possible that the writer moves the header behind
// where we started, and we miss in one loop.
// A second loop should grab the header, but we'll do
// three loops just because I'm paranoid.
//
    while (i < 3) {
    do {
    if (rb_is_head_page(page, page.list.prev)) {
    cpu_buffer.head_page = page;
    return page;
    }
    rb_inc_page(&page);
    } while (page != head);
    }
    RB_WARN_ON(cpu_buffer, 1);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn rb_head_page_replace(old: *mut buffer_page, new: *mut buffer_page) -> bool {
    let mut ptr = &old.list.prev.next;
    let mut val = 0;
    val = *ptr & ~RB_FLAG_MASK;
    val |= RB_PAGE_HEAD;
    return try_cmpxchg(ptr, &val, (unsigned long)&new.list);
    }
//
// rb_tail_page_update - move the tail page forward
//
#[no_mangle]
pub unsafe extern "C" fn rb_tail_page_update(cpu_buffer: *mut ring_buffer_per_cpu, tail_page: *mut buffer_page, next_page: *mut buffer_page) {
    let mut old_entries = 0;
    let mut old_write = 0;
//
// The tail page now needs to be moved forward.
//
// We need to reset the tail page, but without messing
// with possible erasing of data brought in by interrupts
// that have moved the tail page and are currently on it.
//
// We add a counter to the write field to denote this.
//
    old_write = local_add_return(RB_WRITE_INTCNT, &next_page.write);
    old_entries = local_add_return(RB_WRITE_INTCNT, &next_page.entries);
//
// Just make sure we have seen our old_write and synchronize
// with any interrupts that come in.
//
    barrier();
//
// If the tail page is still the same as what we think
// it is, then it is up to us to update the tail
// pointer.
//
    if (tail_page == READ_ONCE(cpu_buffer.tail_page)) {
// Zero the write counter
pub static mut val: c_ulong = 0;
pub static mut eval: c_ulong = 0;
//
// This will only succeed if an interrupt did
// not come in and change it. In which case, we
// do not want to modify it.
//
// We add (void) to let the compiler know that we do not care
// about the return value of these functions. We use the
// cmpxchg to only update if an interrupt did not already
// do it for us. If the cmpxchg fails, we don't care.
//
    (void)local_cmpxchg(&next_page.write, old_write, val);
    (void)local_cmpxchg(&next_page.entries, old_entries, eval);
//
// No need to worry about races with clearing out the commit.
// it only can increment when a commit takes place. But that
// only happens in the outer most nested commit.
//
    local_set(&next_page.page.commit, 0);
// Either we update tail_page or an interrupt does
    if (try_cmpxchg(&cpu_buffer.tail_page, &tail_page, next_page)) {
    local_inc(&cpu_buffer.pages_touched);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rb_check_bpage(cpu_buffer: *mut ring_buffer_per_cpu, bpage: *mut buffer_page) {
pub static mut val: c_ulong = 0;
    RB_WARN_ON(cpu_buffer, val & RB_FLAG_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn rb_check_links(cpu_buffer: *mut ring_buffer_per_cpu, list: *mut list_head) -> bool {
    if (RB_WARN_ON(cpu_buffer,
    rb_list_head(rb_list_head(list.next).prev) != list)) {
    return false;
    }
    if (RB_WARN_ON(cpu_buffer,
    rb_list_head(rb_list_head(list.prev).next) != list)) {
    return false;
    }
    return true;
    }
//
// rb_check_pages - integrity check of buffer pages
// @cpu_buffer: CPU buffer with pages to test
//
// As a safety measure we check to make sure the data pages have not
// been corrupted.
//
#[no_mangle]
unsafe extern "C" fn rb_check_pages(cpu_buffer: *mut ring_buffer_per_cpu) {
    let mut head = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    let mut buffer_cnt = 0;
    let mut flags = 0;
pub static mut nr_loops: c_int = 0;
//
// Walk the linked list underpinning the ring buffer and validate all
// its next and prev links.
//
// The check acquires the reader_lock to avoid concurrent processing
// with code that could be modifying the list. However, the lock cannot
// be held for the entire duration of the walk, as this would make the
// time when interrupts are disabled non-deterministic, dependent on the
// ring buffer size. Therefore, the code releases and re-acquires the
// lock after checking each page. The ring_buffer_per_cpu.cnt variable
// is then used to detect if the list was modified while the lock was
// not held, in which case the check needs to be restarted.
//
// The code attempts to perform the check at most three times before
// giving up. This is acceptable because this is only a self-validation
// to detect problems early on. In practice, the list modification
// operations are fairly spaced, and so this check typically succeeds at
// most on the second try.
//
// label;
    if (++nr_loops > 3) {
    return;
    }
    raw_spin_lock_irqsave(&cpu_buffer.reader_lock, flags);
    head = rb_list_head(cpu_buffer.pages);
    if (!rb_check_links(cpu_buffer, head)) {
// goto;
    }
    buffer_cnt = cpu_buffer.cnt;
    tmp = head;
    raw_spin_unlock_irqrestore(&cpu_buffer.reader_lock, flags);
    while (true) {
    raw_spin_lock_irqsave(&cpu_buffer.reader_lock, flags);
    if (buffer_cnt != cpu_buffer.cnt) {
// The list was updated, try again.
    raw_spin_unlock_irqrestore(&cpu_buffer.reader_lock, flags);
// goto;
    }
    tmp = rb_list_head(tmp.next);
    if (tmp == head) {
// The iteration circled back, all is done.
// goto;
    }
    if (!rb_check_links(cpu_buffer, tmp)) {
// goto;
    }
    raw_spin_unlock_irqrestore(&cpu_buffer.reader_lock, flags);
    }
// label;
    raw_spin_unlock_irqrestore(&cpu_buffer.reader_lock, flags);
    }
//
// Take an address, add the meta data size as well as the array of
// array subbuffer indexes, then align it to a subbuffer size.
//
// This is used to help find the next per cpu subbuffer within a mapped range.
//
#[no_mangle]
pub unsafe extern "C" fn rb_range_align_subbuf(addr: c_ulong, subbuf_size: c_int, nr_subbufs: c_int) -> c_ulong {
    addr += sizeof!(ring_buffer_cpu_meta) +
    sizeof!(int) * nr_subbufs;
    return ALIGN(addr, subbuf_size);
    }
//
// Return the ring_buffer_meta for a given @cpu.
//
#[no_mangle]
pub unsafe extern "C" fn rb_range_meta(buffer: *mut trace_buffer, nr_pages: c_int, cpu: c_int) -> *mut c_void {
pub static mut subbuf_size: c_int = 0;
pub static mut meta: *mut c_void = core::ptr::null_mut();
pub static mut bmeta: *mut c_void = core::ptr::null_mut();
    let mut ptr = 0;
    let mut nr_subbufs = 0;
    bmeta = buffer.meta;
    if (!bmeta) {
    return core::ptr::null_mut();
    }
    ptr = (unsigned long)bmeta + bmeta.buffers_offset;
    meta = ptr;
// When nr_pages passed in is zero, the first meta has already been initialized
    if (!nr_pages) {
    nr_subbufs = meta.nr_subbufs;
    } else {
// Include the reader page
    nr_subbufs = nr_pages + 1;
    }
//
// The first chunk may not be subbuffer aligned, where as
// the rest of the chunks are.
//
    if (cpu) {
    ptr = rb_range_align_subbuf(ptr, subbuf_size, nr_subbufs);
    ptr += subbuf_size * nr_subbufs;
// We can use multiplication to find chunks greater than 1
    if (cpu > 1) {
    let mut size = 0;
    let mut p = 0;
// Save the beginning of this CPU chunk
    p = ptr;
    ptr = rb_range_align_subbuf(ptr, subbuf_size, nr_subbufs);
    ptr += subbuf_size * nr_subbufs;
// Now all chunks after this are the same size
    size = ptr - p;
    ptr += size * (cpu - 2);
    }
    }
    return ptr;
    }
// Return the start of subbufs given the meta pointer
#[no_mangle]
pub unsafe extern "C" fn rb_subbufs_from_meta(meta: *mut ring_buffer_cpu_meta) -> *mut c_void {
pub static mut subbuf_size: c_int = 0;
    let mut ptr = 0;
    ptr = (unsigned long)meta;
    ptr = rb_range_align_subbuf(ptr, subbuf_size, meta.nr_subbufs);
    return ptr;
    }
//
// Return a specific sub-buffer for a given @cpu defined by @idx.
//
#[no_mangle]
pub unsafe extern "C" fn rb_range_buffer(cpu_buffer: *mut ring_buffer_per_cpu, idx: c_int) -> *mut c_void {
pub static mut meta: *mut c_void = core::ptr::null_mut();
    let mut ptr = 0;
    let mut subbuf_size = 0;
    meta = rb_range_meta(cpu_buffer.buffer, 0, cpu_buffer.cpu);
    if (!meta) {
    return core::ptr::null_mut();
    }
    if (WARN_ON_ONCE!(idx >= meta.nr_subbufs)) {
    return core::ptr::null_mut();
    }
    subbuf_size = meta.subbuf_size;
// Map this buffer to the order that's in meta->buffers[]
    idx = meta.buffers[idx];
    ptr = (unsigned long)rb_subbufs_from_meta(meta);
    ptr += subbuf_size * idx;
    if (ptr + subbuf_size > cpu_buffer.buffer.range_addr_end) {
    return core::ptr::null_mut();
    }
    return ptr;
    }
//
// See if the existing memory contains a valid meta section.
// if so, use that, otherwise initialize it.
//
#[no_mangle]
unsafe extern "C" fn rb_meta_init(buffer: *mut trace_buffer, scratch_size: c_int) -> bool {
pub static mut ptr: c_ulong = 0;
pub static mut bmeta: *mut c_void = core::ptr::null_mut();
    let mut total_size = 0;
    let mut struct_sizes = 0;
    bmeta = ptr;
    buffer.meta = bmeta;
    total_size = buffer.range_addr_end - buffer.range_addr_start;
    struct_sizes = sizeof!(ring_buffer_cpu_meta);
    struct_sizes |= sizeof!(*bmeta) << 16;
// The first buffer will start word size after the meta page
    ptr += sizeof!(*bmeta);
    ptr = ALIGN(ptr, sizeof!(long));
    ptr += scratch_size;
    if (bmeta.magic != RING_BUFFER_META_MAGIC) {
    pr_info!("Ring buffer boot meta mismatch of magic\n");
// goto;
    }
    if (bmeta.struct_sizes != struct_sizes) {
    pr_info!("Ring buffer boot meta mismatch of struct size\n");
// goto;
    }
    if (bmeta.total_size != total_size) {
    pr_info!("Ring buffer boot meta mismatch of total size\n");
// goto;
    }
    if (bmeta.buffers_offset > bmeta.total_size) {
    pr_info!("Ring buffer boot meta mismatch of offset outside of total size\n");
// goto;
    }
    if (bmeta.buffers_offset != ptr - bmeta) {
    pr_info!("Ring buffer boot meta mismatch of first buffer offset\n");
// goto;
    }
    return true;
// label;
    bmeta.magic = RING_BUFFER_META_MAGIC;
    bmeta.struct_sizes = struct_sizes;
    bmeta.total_size = total_size;
    bmeta.buffers_offset = ptr - bmeta;
// Zero out the scratch pad
    memset(bmeta + sizeof!(*bmeta), 0, bmeta.buffers_offset - sizeof!(*bmeta));
    return false;
    }
//
// See if the existing memory contains valid ring buffer data.
// As the previous kernel must be the same as this kernel, all
// the calculations (size of buffers and number of buffers)
// must be the same.
//
#[no_mangle]
pub unsafe extern "C" fn rb_cpu_meta_valid(meta: *mut ring_buffer_cpu_meta, cpu: c_int, buffer: *mut trace_buffer, nr_pages: c_int, subbuf_mask: *mut c_ulong) -> bool {
pub static mut subbuf_size: c_int = 0;
    let mut buffers_start = 0;
    let mut buffers_end = 0;
    let mut i = 0;
    if (!subbuf_mask) {
    return false;
    }
    if (meta.subbuf_size != PAGE_SIZE) {
    pr_info!("Ring buffer boot meta [%d] invalid subbuf_size\n", cpu);
    return false;
    }
    buffers_start = meta.first_buffer;
    buffers_end = meta.first_buffer + (subbuf_size * meta.nr_subbufs);
// Is the head and commit buffers within the range of buffers?
    if (meta.head_buffer < buffers_start ||
    meta.head_buffer >= buffers_end) {
    pr_info!("Ring buffer boot meta [%d] head buffer out of range\n", cpu);
    return false;
    }
    if (meta.commit_buffer < buffers_start ||
    meta.commit_buffer >= buffers_end) {
    pr_info!("Ring buffer boot meta [%d] commit buffer out of range\n", cpu);
    return false;
    }
    bitmap_clear(subbuf_mask, 0, meta.nr_subbufs);
//
// Ensure the meta::buffers array has correct data. The data in each subbufs
// are checked later in rb_meta_validate_events().
//
    while (i < meta.nr_subbufs) {
    if (meta.buffers[i] < 0 ||
    meta.buffers[i] >= meta.nr_subbufs) {
    pr_info!("Ring buffer boot meta [%d] array out of range\n", cpu);
    return false;
    }
    if (test_bit(meta.buffers[i], subbuf_mask)) {
    pr_info!("Ring buffer boot meta [%d] array has duplicates\n", cpu);
    return false;
    }
    set_bit(meta.buffers[i], subbuf_mask);
    }
    return true;
    }
// forward_decl: rb_meta_subbuf_idx;
#[no_mangle]
pub unsafe extern "C" fn rb_read_data_buffer(dpage: *mut buffer_data_page, tail: c_int, cpu: c_int, timestamp: *mut unsigned long long, delta_ptr: *mut u64) -> c_int {
pub static mut event: *mut c_void = core::ptr::null_mut();
    u64 ts, delta;
pub static mut events: c_int = 0;
    let mut len = 0;
    let mut e = 0;
// delta_ptr = 0;
// timestamp = 0;
    ts = dpage.time_stamp;
    while (e < tail) {
    event = (dpage.data + e);
    len = rb_event_length(event);
    if (len <= 0 || len > tail - e) {
    return -1;
    }
    match (event.type_len) {
    RINGBUF_TYPE_TIME_EXTEND => {
    delta = rb_event_time_stamp(event);
    ts += delta;
    // break;
    }
    RINGBUF_TYPE_TIME_STAMP => {
    delta = rb_event_time_stamp(event);
    delta = rb_fix_abs_ts(delta, ts);
    if (delta < ts) {
// delta_ptr = delta;
// timestamp = ts;
    return -1;
    }
    ts = delta;
    // break;
    }
    RINGBUF_TYPE_PADDING => {
    if (event.time_delta == 1) {
    // break;
    }
    fallthrough;
    }
    RINGBUF_TYPE_DATA => {
    events += 1;
    ts += event.time_delta;
    // break;
    }
    _ => {
    return -1;
    }
    }
    }
// timestamp = ts;
    return events;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_validation_state {
    pub entries: c_ulong,
    pub entry_bytes: c_ulong,
    pub discarded: c_int,
    pub ts: u64,
}

#[no_mangle]
pub unsafe extern "C" fn __rb_validate_buffer(bpage: *mut buffer_page, cpu: c_int, meta: *mut ring_buffer_cpu_meta, prev_ts: u64, next_ts: u64) -> c_int {
    let mut dpage = bpage.page;
    unsigned long long ts;
    let mut tail = 0;
    let mut delta = 0;
    let mut ret = 0;
//
// When a sub-buffer is recovered from a read, the commit value may
// have RB_MISSED_* bits set, as these bits are reset on reuse.
// Even after clearing these bits, a commit value greater than the
// subbuf_size is considered invalid.
//
    tail = rb_data_page_commit(dpage);
    if (tail <= meta.subbuf_size - BUF_PAGE_HDR_SIZE) {
    ret = rb_read_data_buffer(dpage, tail, cpu, &ts, &delta);
    }
    else {
    ret = -1;
    }
//
// The timestamp must be greater than @prev_ts and smaller than @next_ts.
// Since this function works in both forward (verify) and reverse (unwind)
// loop, we don't know both @prev_ts and @next_ts at the same time.
// So use the known boundary as the boundary.
//
    if (ret < 0 || (prev_ts && prev_ts > ts) || (next_ts && ts > next_ts)) {
    local_set(&bpage.entries, 0);
//
// Note, the RB_MISSED_EVENTS is only set inside the main write
// buffer by this verification logic. The normal ring buffer
// has this bit set when the page is read and passed to the
// consumers.
//
    local_set(&dpage.commit, RB_MISSED_EVENTS);
    dpage.time_stamp = prev_ts ? prev_ts : next_ts;
    ret = -1;
    } else {
    local_set(&bpage.entries, ret);
    }
    return ret;
    }
//
// rb_validate_buffer - validates a single buffer page and updates the state.
// @bpage: buffer page to validate
// @cpu_buffer: cpu_buffer this page belongs to
// @meta: meta of the cpu_buffer
// @state: validation state
// @prev_ts: previous buffer's timestamp (optional)
// @next_ts: next buffer's timestamp (optional)
//
// If the page is invalid (wrong event length or timestamp), it increments the
// discarded counter and warns it. Otherwise, it updates the validation state.
//
#[no_mangle]
pub unsafe extern "C" fn rb_validate_buffer(bpage: *mut buffer_page, cpu_buffer: *mut ring_buffer_per_cpu, meta: *mut ring_buffer_cpu_meta, state: *mut rb_validation_state, prev_ts: u64, next_ts: u64) {
    let mut ret = 0;
    ret = __rb_validate_buffer(bpage, cpu_buffer.cpu, meta, prev_ts, next_ts);
    if (ret < 0) {
    if (!state.discarded) {
    pr_info!("Ring buffer meta [%d] invalid buffer page detected\n",
    cpu_buffer.cpu);
    }
    state.discarded += 1;
    } else {
// If the buffer has content, update pages_touched
    if (ret) {
    local_inc(&cpu_buffer.pages_touched);
    }
    state.entries += ret;
    state.entry_bytes += rb_page_size(bpage);
    state.ts = bpage.page.time_stamp;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rb_meta_inject_reader_page(cpu_buffer: *mut ring_buffer_per_cpu, meta: *mut ring_buffer_cpu_meta, orig_head: *mut buffer_page, head_page: *mut buffer_page) {
    let mut bpage = orig_head;
    let mut i = 0;
    rb_dec_page(&bpage);
//
// Insert the reader_page before the original head page.
// Since the list encode RB_PAGE flags, general list
// operations should be avoided.
//
    cpu_buffer.reader_page.list.next = &orig_head.list;
    cpu_buffer.reader_page.list.prev = orig_head.list.prev;
    orig_head.list.prev = &cpu_buffer.reader_page.list;
    bpage.list.next = &cpu_buffer.reader_page.list;
// Make the head_page the reader page
    cpu_buffer.reader_page = head_page;
    bpage = head_page;
    rb_inc_page(&head_page);
    head_page.list.prev = bpage.list.prev;
    rb_dec_page(&bpage);
    bpage.list.next = &head_page.list;
    rb_set_list_to_head(&bpage.list);
    cpu_buffer.pages = &head_page.list;
    cpu_buffer.head_page = head_page;
    meta.head_buffer = (unsigned long)head_page.page;
// Reset all the indexes
    bpage = cpu_buffer.reader_page;
    meta.buffers[0] = rb_meta_subbuf_idx(meta, bpage.page);
    bpage.id = 0;
    for (i = 1, bpage = head_page; i < meta.nr_subbufs;
    i++, rb_inc_page(&bpage)) {
    meta.buffers[i] = rb_meta_subbuf_idx(meta, bpage.page);
    bpage.id = i;
    }
    }
// If the meta data has been validated, now validate the events
#[no_mangle]
unsafe extern "C" fn rb_meta_validate_events(cpu_buffer: *mut ring_buffer_per_cpu) {
    let mut meta = cpu_buffer.ring_meta;
    let mut head_page = core::ptr::null_mut();
    let mut orig_head = core::ptr::null_mut();
    let mut orig_reader = core::ptr::null_mut();
pub static mut state: rb_validation_state = 0;
pub static mut skip: bool = false;
    let mut ret = 0;
    let mut i = 0;
    if (!meta || !meta.head_buffer) {
    return;
    }
    orig_head = head_page = cpu_buffer.head_page;
    orig_reader = cpu_buffer.reader_page;
// Do the head page first
    ret = __rb_validate_buffer(head_page, cpu_buffer.cpu, meta, 0, 0);
    if (ret < 0) {
    pr_info!("Ring buffer meta [%d] invalid head page detected\n",
    cpu_buffer.cpu);
// Don't bother rewinding
    skip = true;
    state.ts = 0;
    } else {
    state.ts = head_page.page.time_stamp;
    }
// Do the reader page - reader must be previous to head.
    rb_validate_buffer(orig_reader, cpu_buffer, meta, &state, 0, state.ts);
    if (skip) {
// goto;
    }
//
// Try to rewind the head so that we can read the pages which are already
// read in the previous boot.
//
    if (head_page == cpu_buffer.tail_page) {
// goto;
    }
    rb_dec_page(&head_page);
    for (i = 0; i < meta.nr_subbufs + 1; i++, rb_dec_page(&head_page)) {
// Rewind until tail (writer) page.
    if (head_page == cpu_buffer.tail_page) {
    break;
    }
// Rewind until unused page (no timestamp, no commit).
    if (!head_page.page.time_stamp && rb_page_commit(head_page) == 0) {
    break;
    }
//
// Skip if the page is invalid, or its timestamp is newer than the
// previous valid page.
//
    rb_validate_buffer(head_page, cpu_buffer, meta, &state, 0, state.ts);
    }
    if (i) {
    pr_info!("Ring buffer [%d] rewound %d pages\n", cpu_buffer.cpu, i);
    }
// The last rewound page must be skipped.
    if (head_page != orig_head) {
    rb_inc_page(&head_page);
    }
//
// If the ring buffer was rewound, then inject the reader page
// into the location just before the original head page.
//
    if (head_page != orig_head) {
    rb_meta_inject_reader_page(cpu_buffer, meta, orig_head, head_page);
// We'll restart verifying from orig_head
    head_page = orig_head;
    }
// label;
// If the commit_buffer is the reader page, update the commit page
    if (meta.commit_buffer == (unsigned long)cpu_buffer.reader_page.page) {
    cpu_buffer.commit_page = cpu_buffer.reader_page;
// Nothing more to do, the only page is the reader page
// goto;
    }
    state.ts = head_page.page.time_stamp;
// Iterate until finding the commit page
    for (i = 0; i < meta.nr_subbufs + 1; i++, rb_inc_page(&head_page)) {
// The original reader page has already been checked/counted.
    if (head_page == orig_reader) {
    continue;
    }
    rb_validate_buffer(head_page, cpu_buffer, meta, &state, state.ts, 0);
    if (head_page == cpu_buffer.commit_page) {
    break;
    }
    }
    if (head_page != cpu_buffer.commit_page) {
    pr_info!("Ring buffer meta [%d] commit page not found\n",
    cpu_buffer.cpu);
// goto;
    }
// label;
    local_set(&cpu_buffer.entries, state.entries);
    local_set(&cpu_buffer.entries_bytes, state.entry_bytes);
    pr_info!("Ring buffer meta [%d] is from previous boot!", cpu_buffer.cpu);
    if (state.discarded) {
    pr_cont(" (%d pages discarded)", state.discarded);
    }
    pr_cont("\n");

    if (meta.nr_invalid) {
    pr_warn!("Ring buffer testing [%d] invalid pages: %s (%d/%d)\n",
    cpu_buffer.cpu,
    (state.discarded == meta.nr_invalid) ? "PASSED" : "FAILED",
    state.discarded, meta.nr_invalid);
    }
    if (meta.entry_bytes) {
    pr_warn!("Ring buffer testing [%d] entry_bytes: %s (%ld/%ld)\n",
    cpu_buffer.cpu,
    (state.entry_bytes == meta.entry_bytes) ? "PASSED" : "FAILED",
    (long)state.entry_bytes, (long)meta.entry_bytes);
    }
    meta.nr_invalid = 0;
    meta.entry_bytes = 0;

    return;
// label;
// The content of the buffers are invalid, reset the meta data
    meta.head_buffer = 0;
    meta.commit_buffer = 0;
// Reset the reader page
    local_set(&cpu_buffer.reader_page.entries, 0);
    rb_init_data_page(cpu_buffer.reader_page.page);
// Reset all the subbuffers
    for (i = 0; i < meta.nr_subbufs - 1; i++, rb_inc_page(&head_page)) {
    local_set(&head_page.entries, 0);
    rb_init_data_page(head_page.page);
    }
    }
#[no_mangle]
unsafe extern "C" fn rb_range_meta_init(buffer: *mut trace_buffer, nr_pages: c_int, scratch_size: c_int) {
pub static mut meta: *mut c_void = core::ptr::null_mut();
pub static mut subbuf_mask: *mut c_void = core::ptr::null_mut();
    let mut delta = 0;
pub static mut subbuf: *mut c_void = core::ptr::null_mut();
pub static mut valid: bool = false;
    let mut cpu = 0;
    let mut i = 0;
// Create a mask to test the subbuf array
    subbuf_mask = bitmap_alloc(nr_pages + 1, GFP_KERNEL);
// If subbuf_mask fails to allocate, then rb_meta_valid() will return false
    if (rb_meta_init(buffer, scratch_size)) {
    valid = true;
    }
    while (cpu < nr_cpu_ids) {
pub static mut next_meta: *mut c_void = core::ptr::null_mut();
    meta = rb_range_meta(buffer, nr_pages, cpu);
    if (valid && rb_cpu_meta_valid(meta, cpu, buffer, nr_pages, subbuf_mask)) {
// Make the mappings match the current address
    subbuf = rb_subbufs_from_meta(meta);
    delta = (unsigned long)subbuf - meta.first_buffer;
    meta.first_buffer += delta;
    meta.head_buffer += delta;
    meta.commit_buffer += delta;
    continue;
    }
    if (cpu < nr_cpu_ids - 1) {
    next_meta = rb_range_meta(buffer, nr_pages, cpu + 1);
    }
    else {
    next_meta = buffer.range_addr_end;
    }
    memset(meta, 0, next_meta - meta);
    meta.nr_subbufs = nr_pages + 1;
    meta.subbuf_size = PAGE_SIZE;
    subbuf = rb_subbufs_from_meta(meta);
    meta.first_buffer = (unsigned long)subbuf;
//
// The buffers[] array holds the order of the sub-buffers
// that are after the meta data. The sub-buffers may
// be swapped out when read and inserted into a different
// location of the ring buffer. Although their addresses
// remain the same, the buffers[] array contains the
// index into the sub-buffers holding their actual order.
//
    while (i < meta.nr_subbufs) {
    meta.buffers[i] = i;
    rb_init_data_page(subbuf);
    subbuf += meta.subbuf_size;
    }
    }
    bitmap_free(subbuf_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn rbm_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    let mut cpu_buffer = m.private;
    let mut meta = cpu_buffer.ring_meta;
    let mut val = 0;
    if (!meta) {
    return core::ptr::null_mut();
    }
    if (*pos > meta.nr_subbufs) {
    return core::ptr::null_mut();
    }
    val = *pos;
    val += 1;
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn rbm_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    (*pos)++;
    return rbm_start(m, pos);
    }
#[no_mangle]
unsafe extern "C" fn rbm_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut cpu_buffer = m.private;
    let mut meta = cpu_buffer.ring_meta;
pub static mut val: c_ulong = 0;
pub static mut dpage: *mut c_void = core::ptr::null_mut();
    if (val == 1) {
    seq_printf(m, "head_buffer:   %d\n",
    rb_meta_subbuf_idx(meta, meta.head_buffer));
    seq_printf(m, "commit_buffer: %d\n",
    rb_meta_subbuf_idx(meta, meta.commit_buffer));
    seq_printf(m, "subbuf_size:   %d\n", meta.subbuf_size);
    seq_printf(m, "nr_subbufs:    %d\n", meta.nr_subbufs);
    return 0;
    }
    val -= 2;
    dpage = rb_range_buffer(cpu_buffer, val);
    seq_printf(m, "buffer[%ld]:    %d (commit: %ld)\n",
    val, meta.buffers[val], dpage ? rb_data_page_commit(dpage) : -1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rbm_stop(m: *mut seq_file, p: *mut c_void) {
    }
pub static mut seq_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_meta_seq_init(file: *mut file, buffer: *mut trace_buffer, cpu: c_int) -> c_int {
pub static mut m: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ret = seq_open(file, &rb_meta_seq_ops);
    if (ret) {
    return ret;
    }
    m = file.private_data;
    m.private = buffer.buffers[cpu];
    return 0;
    }
// Map the buffer_pages to the previous head and commit pages
#[no_mangle]
pub unsafe extern "C" fn rb_meta_buffer_update(cpu_buffer: *mut ring_buffer_per_cpu, bpage: *mut buffer_page) {
    let mut meta = cpu_buffer.ring_meta;
    if (meta.head_buffer == (unsigned long)bpage.page) {
    cpu_buffer.head_page = bpage;
    }
    if (meta.commit_buffer == (unsigned long)bpage.page) {
    cpu_buffer.commit_page = bpage;
    cpu_buffer.tail_page = bpage;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_desc(trace_desc: *mut trace_buffer_desc, cpu: c_int) -> *mut c_void {
    let mut desc = core::ptr::null_mut();
    let mut end = core::ptr::null_mut();
    let mut len = 0;
    let mut i = 0;
    if (!trace_desc || !trace_desc.nr_cpus) {
    return core::ptr::null_mut();
    }
    end = (trace_desc + trace_desc.struct_len);
    desc = __first_ring_buffer_desc(trace_desc);
    len = struct_size(desc, page_va, desc.nr_page_va);
    desc = (desc + (len * cpu));
    if (desc < end && desc.cpu == cpu) {
    return desc;
    }
// Missing CPUs, need to linear search
    for_each_ring_buffer_desc(desc, i, trace_desc) {
    if (desc.cpu == cpu) {
    return desc;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_desc_page(desc: *mut ring_buffer_desc, page_id: c_uint) -> *mut c_void {
    return page_id >= desc.nr_page_va ? core::ptr::null_mut() : desc.page_va[page_id];
    }
#[no_mangle]
pub unsafe extern "C" fn __rb_allocate_pages(cpu_buffer: *mut ring_buffer_per_cpu, nr_pages: c_long, pages: *mut list_head) -> c_int {
    let mut buffer = cpu_buffer.buffer;
    let mut meta = core::ptr::null_mut();
    let mut bpage = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
pub static mut user_thread: bool = false;
    let mut desc = core::ptr::null_mut();
    let mut i = 0;
//
// Check if the available memory is there first.
// Note, si_mem_available() only gives us a rough estimate of available
// memory. It may not be accurate. But we don't care, we just want
// to prevent doing any allocation when it is obvious that it is
// not going to succeed.
//
    i = si_mem_available();
    if (i < nr_pages) {
    return -ENOMEM;
    }
//
// If a user thread allocates too much, and si_mem_available()
// reports there's enough memory, even though there is not.
// Make sure the OOM killer kills this thread. This can happen
// even with RETRY_MAYFAIL because another task may be doing
// an allocation after this task has taken all memory.
// This is the task the OOM killer needs to take out during this
// loop, even if it was triggered by an allocation somewhere else.
//
    if (user_thread) {
    set_current_oom_origin();
    }
    if (buffer.range_addr_start) {
    meta = rb_range_meta(buffer, nr_pages, cpu_buffer.cpu);
    }
    if (buffer.remote) {
    desc = ring_buffer_desc(buffer.remote.desc, cpu_buffer.cpu);
    if (!desc || WARN_ON!(desc.nr_page_va != (nr_pages + 1))) {
    return -EINVAL;
    }
    }
    while (i < nr_pages) {
    bpage = alloc_cpu_page(cpu_buffer.cpu);
    if (!bpage) {
// goto;
    }
    rb_check_bpage(cpu_buffer, bpage);
//
// Append the pages as for mapped buffers we want to keep
// the order
//
    list_add_tail(&bpage.list, pages);
    if (meta) {
// A range was given. Use that for the buffer page
    bpage.page = rb_range_buffer(cpu_buffer, i + 1);
    if (!bpage.page) {
// goto;
    }
// If this is valid from a previous boot
    if (meta.head_buffer) {
    rb_meta_buffer_update(cpu_buffer, bpage);
    }
    bpage.range = 1;
    bpage.id = i + 1;
    } else if (desc) {
    let mut p = ring_buffer_desc_page(desc, i + 1);
    if (WARN_ON!(!p)) {
// goto;
    }
    bpage.page = p;
    bpage.range = 1; /* bpage.page can't be freed */
    bpage.id = i + 1;
    cpu_buffer.subbuf_ids[i + 1] = bpage;
    } else {
    bpage.page = alloc_cpu_data(cpu_buffer.cpu,
    cpu_buffer.buffer.subbuf_order);
    if (!bpage.page) {
// goto;
    }
    }
    bpage.order = cpu_buffer.buffer.subbuf_order;
    if (user_thread && fatal_signal_pending(current)) {
// goto;
    }
    }
    if (user_thread) {
    clear_current_oom_origin();
    }
    return 0;
// label;
    list_for_each_entry_safe(bpage, tmp, pages, list) {
    list_del_init(&bpage.list);
    free_buffer_page(bpage);
    }
    if (user_thread) {
    clear_current_oom_origin();
    }
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn rb_allocate_pages(cpu_buffer: *mut ring_buffer_per_cpu, nr_pages: c_ulong) -> c_int {
pub static mut pages: usize = 0;
    WARN_ON!(!nr_pages);
    if (__rb_allocate_pages(cpu_buffer, nr_pages, &pages)) {
    return -ENOMEM;
    }
//
// The ring buffer page list is a circular list that does not
// start and end with a list head. All page list items point to
// other pages.
//
    cpu_buffer.pages = pages.next;
    list_del(&pages);
    cpu_buffer.nr_pages = nr_pages;
    rb_check_pages(cpu_buffer);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rb_allocate_cpu_buffer(buffer: *mut trace_buffer, nr_pages: c_long, cpu: c_int) -> *mut c_void {
    struct ring_buffer_per_cpu *cpu_buffer __free(kfree) =
    alloc_cpu_buffer(cpu);
pub static mut meta: *mut c_void = core::ptr::null_mut();
pub static mut bpage: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!cpu_buffer) {
    return core::ptr::null_mut();
    }
    cpu_buffer.cpu = cpu;
    cpu_buffer.buffer = buffer;
    raw_spin_lock_init(&cpu_buffer.reader_lock);
    lockdep_set_class(&cpu_buffer.reader_lock, buffer.reader_lock_key);
    cpu_buffer.lock = (arch_spinlock_t)__ARCH_SPIN_LOCK_UNLOCKED;
    INIT_WORK(&cpu_buffer.update_pages_work, update_pages_handler);
    init_completion(&cpu_buffer.update_done);
    init_irq_work(&cpu_buffer.irq_work.work, rb_wake_up_waiters);
    init_waitqueue_head(&cpu_buffer.irq_work.waiters);
    init_waitqueue_head(&cpu_buffer.irq_work.full_waiters);
    mutex_init(&cpu_buffer.mapping_lock);
    bpage = alloc_cpu_page(cpu);
    if (!bpage) {
    return core::ptr::null_mut();
    }
    bpage.order = cpu_buffer.buffer.subbuf_order;
    rb_check_bpage(cpu_buffer, bpage);
    cpu_buffer.reader_page = bpage;
    if (buffer.range_addr_start) {
//
// Range mapped buffers have the same restrictions as memory
// mapped ones do.
//
    cpu_buffer.ring_meta = rb_range_meta(buffer, nr_pages, cpu);
    bpage.page = rb_range_buffer(cpu_buffer, 0);
    if (!bpage.page) {
// goto;
    }
    if (cpu_buffer.ring_meta.head_buffer) {
    rb_meta_buffer_update(cpu_buffer, bpage);
    }
    bpage.range = 1;
    atomic_inc(&cpu_buffer.resize_disabled);
    } else if (buffer.remote) {
    let mut desc = ring_buffer_desc(buffer.remote.desc, cpu);
    if (!desc) {
// goto;
    }
    cpu_buffer.remote = buffer.remote;
    cpu_buffer.meta_page = desc.meta_va;
    cpu_buffer.nr_pages = nr_pages;
    cpu_buffer.subbuf_ids = kzalloc_objs(*cpu_buffer.subbuf_ids,
    cpu_buffer.nr_pages + 1);
    if (!cpu_buffer.subbuf_ids) {
// goto;
    }
// Remote buffers are read-only and immutable
    atomic_inc(&cpu_buffer.record_disabled);
    atomic_inc(&cpu_buffer.resize_disabled);
    bpage.page = ring_buffer_desc_page(desc, cpu_buffer.meta_page.reader.id);
    if (!bpage.page) {
// goto;
    }
    bpage.range = 1;
    cpu_buffer.subbuf_ids[0] = bpage;
    } else {
    bpage.page = alloc_cpu_data(cpu, bpage.order);
    if (!bpage.page) {
// goto;
    }
    }
    INIT_LIST_HEAD(&cpu_buffer.reader_page.list);
    INIT_LIST_HEAD(&cpu_buffer.new_pages);
    ret = rb_allocate_pages(cpu_buffer, nr_pages);
    if (ret < 0) {
// goto;
    }
    rb_meta_validate_events(cpu_buffer);
// If the boot meta was valid then this has already been updated
    meta = cpu_buffer.ring_meta;
    if (!meta || !meta.head_buffer ||
    !cpu_buffer.head_page || !cpu_buffer.commit_page || !cpu_buffer.tail_page) {
    if (meta && meta.head_buffer &&
    (cpu_buffer.head_page || cpu_buffer.commit_page || cpu_buffer.tail_page)) {
    pr_warn!("Ring buffer meta buffers not all mapped\n");
    if (!cpu_buffer.head_page) {
    pr_warn!("   Missing head_page\n");
    }
    if (!cpu_buffer.commit_page) {
    pr_warn!("   Missing commit_page\n");
    }
    if (!cpu_buffer.tail_page) {
    pr_warn!("   Missing tail_page\n");
    }
    }
    cpu_buffer.head_page
    = list_entry(cpu_buffer.pages, buffer_page, list);
    cpu_buffer.tail_page = cpu_buffer.commit_page = cpu_buffer.head_page;
    rb_head_page_activate(cpu_buffer);
    if (cpu_buffer.ring_meta) {
    meta.commit_buffer = meta.head_buffer;
    }
    } else {
// The valid meta buffer still needs to activate the head page
    rb_head_page_activate(cpu_buffer);
    }
    return_ptr(cpu_buffer);
// label;
    kfree(cpu_buffer.subbuf_ids);
    free_buffer_page(cpu_buffer.reader_page);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn rb_free_cpu_buffer(cpu_buffer: *mut ring_buffer_per_cpu) {
    let mut head = cpu_buffer.pages;
    let mut bpage = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    irq_work_sync(&cpu_buffer.irq_work.work);
    if (cpu_buffer.remote) {
    kfree(cpu_buffer.subbuf_ids);
    }
    free_buffer_page(cpu_buffer.reader_page);
    if (head) {
    rb_head_page_deactivate(cpu_buffer);
    list_for_each_entry_safe(bpage, tmp, head, list) {
    list_del_init(&bpage.list);
    free_buffer_page(bpage);
    }
    bpage = list_entry(head, buffer_page, list);
    free_buffer_page(bpage);
    }
    free_pages((unsigned long)cpu_buffer.free_page.data, cpu_buffer.free_page.order);
    kfree(cpu_buffer);
    }

#[no_mangle]
unsafe extern "C" fn rb_test_inject_invalid_pages(buffer: *mut trace_buffer) {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
pub static mut meta: *mut c_void = core::ptr::null_mut();
pub static mut dpage: *mut c_void = core::ptr::null_mut();
pub static mut entry_bytes: c_ulong = 0;
    let mut ptr = 0;
    let mut subbuf_size = 0;
pub static mut invalid: c_int = 0;
    let mut cpu = 0;
    let mut i = 0;
    if (!(buffer.flags & RB_FL_TESTING)) {
    return;
    }
    guard(preempt)();
    cpu = smp_processor_id();
    cpu_buffer = buffer.buffers[cpu];
    if (!cpu_buffer) {
    return;
    }
    meta = cpu_buffer.ring_meta;
    if (!meta) {
    return;
    }
    ptr = (unsigned long)rb_subbufs_from_meta(meta);
    subbuf_size = meta.subbuf_size;
    while (i < meta.nr_subbufs) {
pub static mut idx: c_ulong = 0;
    dpage = (ptr + idx * subbuf_size);
// Skip unused pages
    if (!rb_data_page_commit(dpage)) {
    continue;
    }
//
// Invalidate even pages or multiples of 5. This will cause 3
// contiguous invalidated(empty) pages.
//
    if (!(i & 0x1) || !(i % 5)) {
    local_add(subbuf_size + 1, &dpage.commit);
    invalid += 1;
    } else {
// Count total commit bytes.
    entry_bytes += rb_data_page_size(dpage);
    }
    }
    pr_info!("Inject invalidated %d pages on CPU%d, total size: %ld\n",
    invalid, cpu, (long)entry_bytes);
    meta.nr_invalid = invalid;
    meta.entry_bytes = entry_bytes;
    }

// Stop recording on a persistent buffer and flush cache if needed.
#[no_mangle]
unsafe extern "C" fn rb_flush_buffer_cb(nb: *mut notifier_block, event: c_ulong, data: *mut c_void) -> c_int {
    let mut buffer = container_of!(nb, trace_buffer, flush_nb);
    ring_buffer_record_off(buffer);
    rb_test_inject_invalid_pages(buffer);
    arch_ring_buffer_flush_range(buffer.range_addr_start, buffer.range_addr_end);
    return NOTIFY_DONE;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_buffer(size: c_ulong, flags: c_uint, order: c_int, start: c_ulong, end: c_ulong, scratch_size: c_ulong, key: *mut lock_class_key, remote: *mut ring_buffer_remote) -> *mut c_void {
    struct trace_buffer *buffer __free(kfree) = core::ptr::null_mut();
    let mut nr_pages = 0;
    let mut subbuf_size = 0;
    let mut bsize = 0;
    let mut cpu = 0;
    let mut ret = 0;
// keep it in its own cache line
    buffer = kzalloc(ALIGN(sizeof!(*buffer), cache_line_size()),
    GFP_KERNEL);
    if (!buffer) {
    return core::ptr::null_mut();
    }
    if (!zalloc_cpumask_var(&buffer.cpumask, GFP_KERNEL)) {
    return core::ptr::null_mut();
    }
    buffer.subbuf_order = order;
    subbuf_size = (PAGE_SIZE << order);
    buffer.flags = flags;
    buffer.clock = trace_clock_local;
    buffer.reader_lock_key = key;
    init_irq_work(&buffer.irq_work.work, rb_wake_up_waiters);
    init_waitqueue_head(&buffer.irq_work.waiters);
    bsize = sizeof! * nr_cpu_ids;
    buffer.buffers = kzalloc(ALIGN(bsize, cache_line_size()),
    GFP_KERNEL);
    if (!buffer.buffers) {
// goto;
    }
    cpu = raw_smp_processor_id();
// If start/end are specified, then that overrides size
    if (start && end) {
    let mut buffers_start = 0;
    let mut ptr = 0;
    let mut n = 0;
// Make sure that start is word aligned
    start = ALIGN(start, sizeof!(long));
// scratch_size needs to be aligned too
    scratch_size = ALIGN(scratch_size, sizeof!(long));
// Subtract the buffer meta data and word aligned
    buffers_start = start + sizeof!(ring_buffer_cpu_meta);
    buffers_start = ALIGN(buffers_start, sizeof!(long));
    buffers_start += scratch_size;
// Calculate the size for the per CPU data
    size = end - buffers_start;
    size = size / nr_cpu_ids;
//
// The number of sub-buffers (nr_pages) is determined by the
// total size allocated minus the meta data size.
// Then that is divided by the number of per CPU buffers
// needed, plus account for the integer array index that
// will be appended to the meta data.
//
    nr_pages = (size - sizeof!(ring_buffer_cpu_meta)) /
    (subbuf_size + sizeof!(int));
// Need at least two pages plus the reader page
    if (nr_pages < 3) {
// goto;
    }
// label;
// Make sure that the size fits aligned
    while (n < nr_cpu_ids) {
    ptr += sizeof!(ring_buffer_cpu_meta) +
    sizeof!(int) * nr_pages;
    ptr = ALIGN(ptr, subbuf_size);
    ptr += subbuf_size * nr_pages;
    }
    if (ptr > end) {
    if (nr_pages <= 3) {
// goto;
    }
    nr_pages -= 1;
// goto;
    }
// nr_pages should not count the reader page
    nr_pages -= 1;
    buffer.range_addr_start = start;
    buffer.range_addr_end = end;
    rb_range_meta_init(buffer, nr_pages, scratch_size);
    } else if (remote) {
    let mut desc = ring_buffer_desc(remote.desc, cpu);
    buffer.remote = remote;
// The writer is remote. This ring-buffer is read-only
    atomic_inc(&buffer.record_disabled);
    nr_pages = desc.nr_page_va - 1;
    if (nr_pages < 2) {
// goto;
    }
    } else {
// need at least two pages
    nr_pages = DIV_ROUND_UP(size, rb_subbuf_capacity(buffer));
    if (nr_pages < 2) {
    nr_pages = 2;
    }
    }
    cpumask_set_cpu(cpu, buffer.cpumask);
    buffer.buffers[cpu] = rb_allocate_cpu_buffer(buffer, nr_pages, cpu);
    if (!buffer.buffers[cpu]) {
// goto;
    }
    ret = cpuhp_state_add_instance(CPUHP_TRACE_RB_PREPARE, &buffer.node);
    if (ret < 0) {
// goto;
    }
    mutex_init(&buffer.mutex);
// Persistent ring buffer needs to flush cache before reboot.
    if (start && end) {
    buffer.flush_nb.notifier_call = rb_flush_buffer_cb;
    atomic_notifier_chain_register(&panic_notifier_list, &buffer.flush_nb);
    }
    return_ptr(buffer);
// label;
    for_each_buffer_cpu(buffer, cpu) {
    if (buffer.buffers[cpu]) {
    rb_free_cpu_buffer(buffer.buffers[cpu]);
    }
    }
    kfree(buffer.buffers);
// label;
    free_cpumask_var(buffer.cpumask);
    return core::ptr::null_mut();
    }
//
// __ring_buffer_alloc - allocate a new ring_buffer
// @size: the size in bytes per cpu that is needed.
// @flags: attributes to set for the ring buffer.
// @key: ring buffer reader_lock_key.
//
// Currently the only flag that is available is the RB_FL_OVERWRITE
// flag. This flag means that the buffer will overwrite old data
// when the buffer wraps. If this flag is not set, the buffer will
// drop data when the tail hits the head.
//
#[no_mangle]
pub unsafe extern "C" fn __ring_buffer_alloc(size: c_ulong, flags: c_uint, key: *mut lock_class_key) -> *mut c_void {
// Default buffer page size - one system page
    return alloc_buffer(size, flags, 0, 0, 0, 0, key, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(__ring_buffer_alloc);
//
// __ring_buffer_alloc_range - allocate a new ring_buffer from existing memory
// @size: the size in bytes per cpu that is needed.
// @flags: attributes to set for the ring buffer.
// @order: sub-buffer order
// @start: start of allocated range
// @range_size: size of allocated range
// @scratch_size: size of scratch area (for preallocated memory buffers)
// @key: ring buffer reader_lock_key.
//
// Currently the only flag that is available is the RB_FL_OVERWRITE
// flag. This flag means that the buffer will overwrite old data
// when the buffer wraps. If this flag is not set, the buffer will
// drop data when the tail hits the head.
//
#[no_mangle]
pub unsafe extern "C" fn __ring_buffer_alloc_range(size: c_ulong, flags: c_uint, order: c_int, start: c_ulong, range_size: c_ulong, scratch_size: c_ulong, key: *mut lock_class_key) -> *mut c_void {
    return alloc_buffer(size, flags, order, start, start + range_size,
    scratch_size, key, core::ptr::null_mut());
    }
//
// __ring_buffer_alloc_remote - allocate a new ring_buffer from a remote
// @remote: Contains a description of the ring-buffer pages and remote callbacks.
// @key: ring buffer reader_lock_key.
//
#[no_mangle]
pub unsafe extern "C" fn __ring_buffer_alloc_remote(remote: *mut ring_buffer_remote, key: *mut lock_class_key) -> *mut c_void {
    return alloc_buffer(0, 0, 0, 0, 0, 0, key, remote);
    }
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_meta_scratch(buffer: *mut trace_buffer, size: *mut c_uint) -> *mut c_void {
pub static mut meta: *mut c_void = core::ptr::null_mut();
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    if (!buffer || !buffer.meta) {
    return core::ptr::null_mut();
    }
    meta = buffer.meta;
    ptr = ALIGN((unsigned long)meta + sizeof!(*meta), sizeof!(long));
    if (size) {
// size = meta + meta->buffers_offset - ptr;
    }
    return ptr;
    }
//
// ring_buffer_free - free a ring buffer.
// @buffer: the buffer to free.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_free(buffer: *mut trace_buffer) {
    let mut cpu = 0;
    if (buffer.range_addr_start && buffer.range_addr_end) {
    atomic_notifier_chain_unregister(&panic_notifier_list, &buffer.flush_nb);
    }
    cpuhp_state_remove_instance(CPUHP_TRACE_RB_PREPARE, &buffer.node);
    irq_work_sync(&buffer.irq_work.work);
    for_each_buffer_cpu(buffer, cpu) {
    rb_free_cpu_buffer(buffer.buffers[cpu]);
    }
    kfree(buffer.buffers);
    free_cpumask_var(buffer.cpumask);
    kfree(buffer);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_free);
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_set_clock(buffer: *mut trace_buffer) {
    buffer.clock = clock;
    }
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_set_time_stamp_abs(buffer: *mut trace_buffer, abs: bool) {
    buffer.time_stamp_abs = abs;
    }
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_time_stamp_abs(buffer: *mut trace_buffer) -> bool {
    return buffer.time_stamp_abs;
    }
#[no_mangle]
pub unsafe extern "C" fn rb_page_entries(bpage: *mut buffer_page) -> c_ulong {
    return local_read(&bpage.entries) & RB_WRITE_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn rb_page_write(bpage: *mut buffer_page) -> c_ulong {
    return local_read(&bpage.write) & RB_WRITE_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn rb_remove_pages(cpu_buffer: *mut ring_buffer_per_cpu, nr_pages: c_ulong) -> bool {
    let mut tail_page = core::ptr::null_mut();
    let mut to_remove = core::ptr::null_mut();
    let mut next_page = core::ptr::null_mut();
    let mut to_remove_page = core::ptr::null_mut();
    let mut tmp_iter_page = core::ptr::null_mut();
    let mut last_page = core::ptr::null_mut();
    let mut first_page = core::ptr::null_mut();
    let mut nr_removed = 0;
    let mut head_bit = 0;
    let mut page_entries = 0;
    head_bit = 0;
    raw_spin_lock_irq(&cpu_buffer.reader_lock);
    atomic_inc(&cpu_buffer.record_disabled);
//
// We don't race with the readers since we have acquired the reader
// lock. We also don't race with writers after disabling recording.
// This makes it easy to figure out the first and the last page to be
// removed from the list. We unlink all the pages in between including
// the first and last pages. This is done in a busy loop so that we
// lose the least number of traces.
// The pages are freed after we restart recording and unlock readers.
//
    tail_page = &cpu_buffer.tail_page.list;
//
// tail page might be on reader page, we remove the next page
// from the ring buffer
//
    if (cpu_buffer.tail_page == cpu_buffer.reader_page) {
    tail_page = rb_list_head(tail_page.next);
    }
    to_remove = tail_page;
// start of pages to remove
    first_page = list_entry(rb_list_head(to_remove.next), buffer_page, list);
    while (nr_removed < nr_pages) {
    to_remove = rb_list_head(to_remove).next;
    head_bit |= (unsigned long)to_remove & RB_PAGE_HEAD;
    }
// Read iterators need to reset themselves when some pages removed
    cpu_buffer.pages_removed += nr_removed;
    next_page = rb_list_head(to_remove).next;
//
// Now we remove all pages between tail_page and next_page.
// Make sure that we have head_bit value preserved for the
// next page
//
    tail_page.next = ((unsigned long)next_page |
    head_bit);
    next_page = rb_list_head(next_page);
    next_page.prev = tail_page;
// make sure pages points to a valid page in the ring buffer
    cpu_buffer.pages = next_page;
    cpu_buffer.cnt += 1;
// update head page
    if (head_bit) {
    cpu_buffer.head_page = list_entry(next_page, buffer_page, list);
    }
// pages are removed, resume tracing and then free the pages
    atomic_dec(&cpu_buffer.record_disabled);
    raw_spin_unlock_irq(&cpu_buffer.reader_lock);
    RB_WARN_ON(cpu_buffer, list_empty(cpu_buffer.pages));
// last buffer page to remove
    last_page = list_entry(rb_list_head(to_remove), buffer_page,
    list);
    tmp_iter_page = first_page;
    do {
    cond_resched();
    to_remove_page = tmp_iter_page;
    rb_inc_page(&tmp_iter_page);
// update the counters
    page_entries = rb_page_entries(to_remove_page);
    if (page_entries) {
//
// If something was added to this page, it was full
// since it is not the tail page. So we deduct the
// bytes consumed in ring buffer from here.
// Increment overrun to account for the lost events.
//
    local_add(page_entries, &cpu_buffer.overrun);
    local_sub(rb_page_commit(to_remove_page), &cpu_buffer.entries_bytes);
    local_inc(&cpu_buffer.pages_lost);
    }
//
// We have already removed references to this list item, just
// free up the buffer_page and its page
//
    free_buffer_page(to_remove_page);
    nr_removed -= 1;
    } while (to_remove_page != last_page);
    RB_WARN_ON(cpu_buffer, nr_removed);
pub static mut nr_removed: return = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rb_insert_pages(cpu_buffer: *mut ring_buffer_per_cpu) -> bool {
    let mut pages = &cpu_buffer.new_pages;
    let mut flags = 0;
    let mut success = 0;
    let mut retries = 0;
// Can be called at early boot up, where interrupts must not been enabled
    raw_spin_lock_irqsave(&cpu_buffer.reader_lock, flags);
//
// We are holding the reader lock, so the reader page won't be swapped
// in the ring buffer. Now we are racing with the writer trying to
// move head page and the tail page.
// We are going to adapt the reader page update process where:
// 1. We first splice the start and end of list of new pages between
// the head page and its previous page.
// 2. We cmpxchg the prev_page->next to point from head page to the
// start of new pages list.
// 3. Finally, we update the head->prev to the end of new list.
//
// We will try this process 10 times, to make sure that we don't keep
// spinning.
//
    retries = 10;
    success = false;
    while (retries--) {
    let mut head_page = core::ptr::null_mut();
    let mut prev_page = core::ptr::null_mut();
    let mut last_page = core::ptr::null_mut();
    let mut first_page = core::ptr::null_mut();
pub static mut head_page_with_bit: *mut c_void = core::ptr::null_mut();
    let mut hpage = rb_set_head_page(cpu_buffer);
    if (!hpage) {
    break;
    }
    head_page = &hpage.list;
    prev_page = head_page.prev;
    first_page = pages.next;
    last_page  = pages.prev;
    head_page_with_bit = 
    ((unsigned long)head_page | RB_PAGE_HEAD);
    last_page.next = head_page_with_bit;
    first_page.prev = prev_page;
// caution: head_page_with_bit gets updated on cmpxchg failure
    if (try_cmpxchg(&prev_page.next,
    &head_page_with_bit, first_page)) {
//
// yay, we replaced the page pointer to our new list,
// now, we just have to update to head page's prev
// pointer to point to end of list
//
    head_page.prev = last_page;
    cpu_buffer.cnt += 1;
    success = true;
    break;
    }
    }
    if (success) {
    INIT_LIST_HEAD(pages);
    }
//
// If we weren't successful in adding in new pages, warn and stop
// tracing
//
    RB_WARN_ON(cpu_buffer, !success);
    raw_spin_unlock_irqrestore(&cpu_buffer.reader_lock, flags);
// free pages if they weren't inserted
    if (!success) {
    let mut bpage = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    list_for_each_entry_safe(bpage, tmp, &cpu_buffer.new_pages,
    list) {
    list_del_init(&bpage.list);
    free_buffer_page(bpage);
    }
    }
    return success;
    }
#[no_mangle]
unsafe extern "C" fn rb_update_pages(cpu_buffer: *mut ring_buffer_per_cpu) {
    let mut success = 0;
    if (cpu_buffer.nr_pages_to_update > 0) {
    success = rb_insert_pages(cpu_buffer);
    }
    else {
    success = rb_remove_pages(cpu_buffer,
    -cpu_buffer.nr_pages_to_update);
    }
    if (success) {
    cpu_buffer.nr_pages += cpu_buffer.nr_pages_to_update;
    }
    }
#[no_mangle]
unsafe extern "C" fn update_pages_handler(work: *mut work_struct) {
    let mut cpu_buffer = container_of!(work, ring_buffer_per_cpu, update_pages_work);
    rb_update_pages(cpu_buffer);
    complete(&cpu_buffer.update_done);
    }
//
// ring_buffer_resize - resize the ring buffer
// @buffer: the buffer to resize.
// @size: the new size.
// @cpu_id: the cpu buffer to resize
//
// Minimum size is 2 * rb_subbuf_capacity(buffer).
//
// Returns 0 on success and < 0 on failure.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_resize(buffer: *mut trace_buffer, size: c_ulong, cpu_id: c_int) -> c_int {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut nr_pages = 0;
    let mut cpu = 0;
    let mut err = 0;
//
// Always succeed at resizing a non-existent buffer:
//
    if (!buffer) {
    return 0;
    }
// Make sure the requested buffer exists
    if (cpu_id != RING_BUFFER_ALL_CPUS &&
    !cpumask_test_cpu(cpu_id, buffer.cpumask)) {
    return 0;
    }
//
// Keep CPUs from coming online while resizing to synchronize
// with new per CPU buffers being created.
//
    guard(cpus_read_lock)();
// prevent another thread from changing buffer sizes
    mutex_lock(&buffer.mutex);
    atomic_inc(&buffer.resizing);
    nr_pages = DIV_ROUND_UP(size, rb_subbuf_capacity(buffer));
// we need a minimum of two pages
    if (nr_pages < 2) {
    nr_pages = 2;
    }
    if (cpu_id == RING_BUFFER_ALL_CPUS) {
//
// Don't succeed if resizing is disabled, as a reader might be
// manipulating the ring buffer and is expecting a sane state while
// this is true.
//
    for_each_buffer_cpu(buffer, cpu) {
    cpu_buffer = buffer.buffers[cpu];
    if (atomic_read(&cpu_buffer.resize_disabled)) {
    err = -EBUSY;
// goto;
    }
    }
// calculate the pages to update
    for_each_buffer_cpu(buffer, cpu) {
    cpu_buffer = buffer.buffers[cpu];
    cpu_buffer.nr_pages_to_update = nr_pages -
    cpu_buffer.nr_pages;
//
// nothing more to do for removing pages or no update
//
    if (cpu_buffer.nr_pages_to_update <= 0) {
    continue;
    }
//
// to add pages, make sure all new pages can be
// allocated without receiving ENOMEM
//
    INIT_LIST_HEAD(&cpu_buffer.new_pages);
    if (__rb_allocate_pages(cpu_buffer, cpu_buffer.nr_pages_to_update,
    &cpu_buffer.new_pages)) {
// not enough memory for new pages
    err = -ENOMEM;
// goto;
    }
    cond_resched();
    }
//
// Fire off all the required work handlers
// We can't schedule on offline CPUs, but it's not necessary
// since we can change their buffer sizes without any race.
//
    for_each_buffer_cpu(buffer, cpu) {
    cpu_buffer = buffer.buffers[cpu];
    if (!cpu_buffer.nr_pages_to_update) {
    continue;
    }
// Can't run something on an offline CPU.
    if (!cpu_online(cpu)) {
    rb_update_pages(cpu_buffer);
    cpu_buffer.nr_pages_to_update = 0;
    } else {
// Run directly if possible.
    migrate_disable();
    if (cpu != smp_processor_id()) {
    migrate_enable();
    schedule_work_on(cpu,
    &cpu_buffer.update_pages_work);
    } else {
    update_pages_handler(&cpu_buffer.update_pages_work);
    migrate_enable();
    }
    }
    }
// wait for all the updates to complete
    for_each_buffer_cpu(buffer, cpu) {
    cpu_buffer = buffer.buffers[cpu];
    if (!cpu_buffer.nr_pages_to_update) {
    continue;
    }
    if (cpu_online(cpu)) {
    wait_for_completion(&cpu_buffer.update_done);
    }
    cpu_buffer.nr_pages_to_update = 0;
    }
    } else {
    cpu_buffer = buffer.buffers[cpu_id];
    if (nr_pages == cpu_buffer.nr_pages) {
// goto;
    }
//
// Don't succeed if resizing is disabled, as a reader might be
// manipulating the ring buffer and is expecting a sane state while
// this is true.
//
    if (atomic_read(&cpu_buffer.resize_disabled)) {
    err = -EBUSY;
// goto;
    }
    cpu_buffer.nr_pages_to_update = nr_pages -
    cpu_buffer.nr_pages;
    INIT_LIST_HEAD(&cpu_buffer.new_pages);
    if (cpu_buffer.nr_pages_to_update > 0 &&
    __rb_allocate_pages(cpu_buffer, cpu_buffer.nr_pages_to_update,
    &cpu_buffer.new_pages)) {
    err = -ENOMEM;
// goto;
    }
// Can't run something on an offline CPU.
    if (!cpu_online(cpu_id)) {
    rb_update_pages(cpu_buffer);
    }
    else {
// Run directly if possible.
    migrate_disable();
    if (cpu_id == smp_processor_id()) {
    rb_update_pages(cpu_buffer);
    migrate_enable();
    } else {
    migrate_enable();
    schedule_work_on(cpu_id,
    &cpu_buffer.update_pages_work);
    wait_for_completion(&cpu_buffer.update_done);
    }
    }
    cpu_buffer.nr_pages_to_update = 0;
    }
// label;
//
// The ring buffer resize can happen with the ring buffer
// enabled, so that the update disturbs the tracing as little
// as possible. But if the buffer is disabled, we do not need
// to worry about that, and we can take the time to verify
// that the buffer is not corrupt.
//
    if (atomic_read(&buffer.record_disabled)) {
    atomic_inc(&buffer.record_disabled);
//
// Even though the buffer was disabled, we must make sure
// that it is truly disabled before calling rb_check_pages.
// There could have been a race between checking
// record_disable and incrementing it.
//
    synchronize_rcu();
    for_each_buffer_cpu(buffer, cpu) {
    cpu_buffer = buffer.buffers[cpu];
    rb_check_pages(cpu_buffer);
    }
    atomic_dec(&buffer.record_disabled);
    }
    atomic_dec(&buffer.resizing);
    mutex_unlock(&buffer.mutex);
    return 0;
// label;
    for_each_buffer_cpu(buffer, cpu) {
    let mut bpage = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    cpu_buffer = buffer.buffers[cpu];
    cpu_buffer.nr_pages_to_update = 0;
    if (list_empty(&cpu_buffer.new_pages)) {
    continue;
    }
    list_for_each_entry_safe(bpage, tmp, &cpu_buffer.new_pages,
    list) {
    list_del_init(&bpage.list);
    free_buffer_page(bpage);
    cond_resched();
    }
    }
// label;
    atomic_dec(&buffer.resizing);
    mutex_unlock(&buffer.mutex);
    return err;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_resize);
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_change_overwrite(buffer: *mut trace_buffer, val: c_int) {
    mutex_lock(&buffer.mutex);
    if (val) {
    buffer.flags |= RB_FL_OVERWRITE;
    }
    else {
    buffer.flags &= ~RB_FL_OVERWRITE;
    }
    mutex_unlock(&buffer.mutex);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_change_overwrite);
    static __always_inline void *__rb_page_index(buffer_page *bpage, unsigned index)
    {
    return bpage.page.data + index;
    }
    static __always_inline struct ring_buffer_event *
    rb_reader_event(ring_buffer_per_cpu *cpu_buffer)
    {
    return __rb_page_index(cpu_buffer.reader_page,
    cpu_buffer.reader_page.read);
    }
#[no_mangle]
pub unsafe extern "C" fn rb_iter_head_event(iter: *mut ring_buffer_iter) -> *mut c_void {
pub static mut event: *mut c_void = core::ptr::null_mut();
    let mut iter_head_page = iter.head_page;
    let mut commit = 0;
    let mut length: c_uint = 0;
    if (iter.head != iter.next_event) {
    return iter.event;
    }
//
// When the writer goes across pages, it issues a cmpxchg which
// is a mb(), which will synchronize with the rmb here.
// (see rb_tail_page_update() and __rb_reserve_next())
//
    commit = rb_page_size(iter_head_page);
    smp_rmb();
// An event needs to be at least 8 bytes in size
    if (iter.head > commit - 8) {
// goto;
    }
    event = __rb_page_index(iter_head_page, iter.head);
    length = rb_event_length(event);
//
// READ_ONCE() doesn't work on functions and we don't want the
// compiler doing any crazy optimizations with length.
//
    barrier();
    if ((iter.head + length) > commit || length > iter.event_size) {
// Writer corrupted the read?
// goto;
    }
    memcpy(iter.event, event, length);
//
// If the page stamp is still the same after this rmb() then the
// event was safely copied without the writer entering the page.
//
    smp_rmb();
// Make sure the page didn't change since we read this
    if (iter.page_stamp != iter_head_page.page.time_stamp ||
    commit > rb_page_size(iter_head_page)) {
// goto;
    }
    iter.next_event = iter.head + length;
    return iter.event;
// label;
// Reset to the beginning
    iter.page_stamp = iter.read_stamp = iter.head_page.page.time_stamp;
    iter.head = 0;
    iter.next_event = 0;
    iter.missed_events = 1;
    return core::ptr::null_mut();
    }
    static __always_inline unsigned
    rb_commit_index(ring_buffer_per_cpu *cpu_buffer)
    {
    return rb_page_commit(cpu_buffer.commit_page);
    }
    static __always_inline unsigned
    rb_event_index(ring_buffer_per_cpu *cpu_buffer, ring_buffer_event *event)
    {
pub static mut addr: c_ulong = 0;
    addr &= (unsigned long)rb_subbuf_size(cpu_buffer.buffer) - 1;
    return addr - BUF_PAGE_HDR_SIZE;
    }
#[no_mangle]
unsafe extern "C" fn rb_inc_iter(iter: *mut ring_buffer_iter) {
    let mut cpu_buffer = iter.cpu_buffer;
//
// The iterator could be on the reader page (it starts there).
// But the head could have moved, since the reader was
// found. Check for this case and assign the iterator
// to the head page instead of next.
//
    if (iter.head_page == cpu_buffer.reader_page) {
    iter.head_page = rb_set_head_page(cpu_buffer);
    }
    else {
    rb_inc_page(&iter.head_page);
    }
    if (rb_page_commit(iter.head_page) & RB_MISSED_EVENTS) {
    iter.missed_events = -1;
    }
    iter.page_stamp = iter.read_stamp = iter.head_page.page.time_stamp;
    iter.head = 0;
    iter.next_event = 0;
    }
// Return the index into the sub-buffers for a given sub-buffer
#[no_mangle]
unsafe extern "C" fn rb_meta_subbuf_idx(meta: *mut ring_buffer_cpu_meta, subbuf: *mut c_void) -> c_int {
pub static mut subbuf_array: *mut c_void = core::ptr::null_mut();
    subbuf_array = meta + sizeof!(int) * meta.nr_subbufs;
    subbuf_array = ALIGN((unsigned long)subbuf_array, meta.subbuf_size);
    return (subbuf - subbuf_array) / meta.subbuf_size;
    }
#[no_mangle]
pub unsafe extern "C" fn rb_update_meta_head(cpu_buffer: *mut ring_buffer_per_cpu, next_page: *mut buffer_page) {
    let mut meta = cpu_buffer.ring_meta;
pub static mut old_head: c_ulong = 0;
    let mut new_head = 0;
    rb_inc_page(&next_page);
    new_head = (unsigned long)next_page.page;
//
// Only move it forward once, if something else came in and
// moved it forward, then we don't want to touch it.
//
    (void)cmpxchg(&meta.head_buffer, old_head, new_head);
    }
#[no_mangle]
pub unsafe extern "C" fn rb_update_meta_reader(cpu_buffer: *mut ring_buffer_per_cpu, reader: *mut buffer_page) {
    let mut meta = cpu_buffer.ring_meta;
    let mut old_reader = cpu_buffer.reader_page.page;
    let mut new_reader = reader.page;
    let mut id = 0;
    id = reader.id;
    cpu_buffer.reader_page.id = id;
    reader.id = 0;
    meta.buffers[0] = rb_meta_subbuf_idx(meta, new_reader);
    meta.buffers[id] = rb_meta_subbuf_idx(meta, old_reader);
// The head pointer is the one after the reader
    rb_update_meta_head(cpu_buffer, reader);
    }
//
// rb_handle_head_page - writer hit the head page
//
// Returns: +1 to retry page
// 0 to continue
// -1 on error
//
#[no_mangle]
pub unsafe extern "C" fn rb_handle_head_page(cpu_buffer: *mut ring_buffer_per_cpu, tail_page: *mut buffer_page, next_page: *mut buffer_page) -> c_int {
pub static mut new_head: *mut c_void = core::ptr::null_mut();
    let mut entries = 0;
    let mut type = 0;
    let mut ret = 0;
    entries = rb_page_entries(next_page);
//
// The hard part is here. We need to move the head
// forward, and protect against both readers on
// other CPUs and writers coming in via interrupts.
//
    type = rb_head_page_set_update(cpu_buffer, next_page, tail_page,
    RB_PAGE_HEAD);
//
// type can be one of four:
// NORMAL - an interrupt already moved it for us
// HEAD   - we are the first to get here.
// UPDATE - we are the interrupt interrupting
// a current move.
// MOVED  - a reader on another CPU moved the next
// pointer to its reader page. Give up
// and try again.
//
    match (type) {
    RB_PAGE_HEAD => {
//
// We changed the head to UPDATE, thus
// it is our responsibility to update
// the counters.
//
    local_add(entries, &cpu_buffer.overrun);
    local_sub(rb_page_commit(next_page), &cpu_buffer.entries_bytes);
    local_inc(&cpu_buffer.pages_lost);
    if (cpu_buffer.ring_meta) {
    rb_update_meta_head(cpu_buffer, next_page);
    }
//
// The entries will be zeroed out when we move the
// tail page.
//
// still more to do
    // break;
    }
    RB_PAGE_UPDATE => {
//
// This is an interrupt that interrupt the
// previous update. Still more to do.
//
    // break;
    }
    RB_PAGE_NORMAL => {
//
// An interrupt came in before the update
// and processed this for us.
// Nothing left to do.
//
    return 1;
    }
    RB_PAGE_MOVED => {
//
// The reader is on another CPU and just did
// a swap with our next_page.
// Try again.
//
    return 1;
    }
    _ => {
    RB_WARN_ON(cpu_buffer, 1); /* WTF??? */
    return -1;
    }
    }
//
// Now that we are here, the old head pointer is
// set to UPDATE. This will keep the reader from
// swapping the head page with the reader page.
// The reader (on another CPU) will spin till
// we are finished.
//
// We just need to protect against interrupts
// doing the job. We will set the next pointer
// to HEAD. After that, we set the old pointer
// to NORMAL, but only if it was HEAD before.
// otherwise we are an interrupt, and only
// want the outer most commit to reset it.
//
    new_head = next_page;
    rb_inc_page(&new_head);
    ret = rb_head_page_set_head(cpu_buffer, new_head, next_page,
    RB_PAGE_NORMAL);
//
// Valid returns are:
// HEAD   - an interrupt came in and already set it.
// NORMAL - One of two things:
// 1) We really set it.
// 2) A bunch of interrupts came in and moved
// the page forward again.
//
    match (ret) {
    RB_PAGE_HEAD => {
    }
    RB_PAGE_NORMAL => {
// OK
    // break;
    }
    _ => {
    RB_WARN_ON(cpu_buffer, 1);
    return -1;
    }
    }
//
// It is possible that an interrupt came in,
// set the head up, then more interrupts came in
// and moved it again. When we get back here,
// the page would have been set to NORMAL but we
// just set it back to HEAD.
//
// How do you detect this? Well, if that happened
// the tail page would have moved.
//
    if (ret == RB_PAGE_NORMAL) {
pub static mut buffer_tail_page: *mut c_void = core::ptr::null_mut();
    buffer_tail_page = READ_ONCE(cpu_buffer.tail_page);
//
// If the tail had moved passed next, then we need
// to reset the pointer.
//
    if (buffer_tail_page != tail_page &&
    buffer_tail_page != next_page) {
    rb_head_page_set_normal(cpu_buffer, new_head,
    next_page,
    RB_PAGE_HEAD);
    }
    }
//
// If this was the outer most commit (the one that
// changed the original pointer from HEAD to UPDATE),
// then it is up to us to reset it to NORMAL.
//
    if (type == RB_PAGE_HEAD) {
    ret = rb_head_page_set_normal(cpu_buffer, next_page,
    tail_page,
    RB_PAGE_UPDATE);
    if (RB_WARN_ON(cpu_buffer,
    ret != RB_PAGE_UPDATE)) {
    return -1;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rb_reset_tail(cpu_buffer: *mut ring_buffer_per_cpu, tail: c_ulong, info: *mut rb_event_info) {
    let mut tail_page = info.tail_page;
pub static mut bsize: c_ulong = 0;
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut length: c_ulong = 0;
//
// Only the event that crossed the page boundary
// must fill the old tail_page with padding.
//
    if (tail >= bsize) {
//
// If the page was filled, then we still need
// to update the real_end. Reset it to zero
// and the reader will ignore it.
//
    if (tail == bsize) {
    tail_page.real_end = 0;
    }
    local_sub(length, &tail_page.write);
    return;
    }
    event = __rb_page_index(tail_page, tail);
//
// Save the original length to the meta data.
// This will be used by the reader to add lost event
// counter.
//
    tail_page.real_end = tail;
//
// If this event is bigger than the minimum size, then
// we need to be careful that we don't subtract the
// write counter enough to allow another writer to slip
// in on this page.
// We put in a discarded commit instead, to make sure
// that this space is not used again, and this space will
// not be accounted into 'entries_bytes'.
//
// If we are less than the minimum size, we don't need to
// worry about it.
//
    if (tail > (bsize - RB_EVNT_MIN_SIZE)) {
// No room for any events
// Mark the rest of the page with padding
    rb_event_set_padding(event);
// Make sure the padding is visible before the write update
    smp_wmb();
// Set the write back to the previous setting
    local_sub(length, &tail_page.write);
    return;
    }
// Put in a discarded event
    event.array[0] = (bsize - tail) - RB_EVNT_HDR_SIZE;
    event.type_len = RINGBUF_TYPE_PADDING;
// time delta must be non zero
    event.time_delta = 1;
// account for padding bytes
    local_add(bsize - tail, &cpu_buffer.entries_bytes);
// Make sure the padding is visible before the tail_page->write update
    smp_wmb();
// Set write to end of buffer
    length = (tail + length) - bsize;
    local_sub(length, &tail_page.write);
    }
// forward_decl: rb_end_commit;
//
// This is the slow path, force gcc not to inline it.
//
    static noinline struct ring_buffer_event *
    rb_move_tail(ring_buffer_per_cpu *cpu_buffer,
    unsigned long tail, rb_event_info *info)
    {
    let mut tail_page = info.tail_page;
    let mut commit_page = cpu_buffer.commit_page;
    let mut buffer = cpu_buffer.buffer;
pub static mut next_page: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    next_page = tail_page;
    rb_inc_page(&next_page);
//
// If for some reason, we had an interrupt storm that made
// it all the way around the buffer, bail, and warn
// about it.
//
    if (unlikely(next_page == commit_page)) {
    local_inc(&cpu_buffer.commit_overrun);
// goto;
    }
//
// This is where the fun begins!
//
// We are fighting against races between a reader that
// could be on another CPU trying to swap its reader
// page with the buffer head.
//
// We are also fighting against interrupts coming in and
// moving the head or tail on us as well.
//
// If the next page is the head page then we have filled
// the buffer, unless the commit page is still on the
// reader page.
//
    if (rb_is_head_page(next_page, &tail_page.list)) {
//
// If the commit is not on the reader page, then
// move the header page.
//
    if (!rb_is_reader_page(cpu_buffer.commit_page)) {
//
// If we are not in overwrite mode,
// this is easy, just stop here.
//
    if (!(buffer.flags & RB_FL_OVERWRITE)) {
    local_inc(&cpu_buffer.dropped_events);
// goto;
    }
    ret = rb_handle_head_page(cpu_buffer,
    tail_page,
    next_page);
    if (ret < 0) {
// goto;
    }
    if (ret) {
// goto;
    }
    } else {
//
// We need to be careful here too. The
// commit page could still be on the reader
// page. We could have a small buffer, and
// have filled up the buffer with events
// from interrupts and such, and wrapped.
//
// Note, if the tail page is also on the
// reader_page, we let it move out.
//
    if (unlikely((cpu_buffer.commit_page !=
    cpu_buffer.tail_page) &&
    (cpu_buffer.commit_page ==
    cpu_buffer.reader_page))) {
    local_inc(&cpu_buffer.commit_overrun);
// goto;
    }
    }
    }
    rb_tail_page_update(cpu_buffer, tail_page, next_page);
// label;
    rb_reset_tail(cpu_buffer, tail, info);
// Commit what we have for now.
    rb_end_commit(cpu_buffer);
// rb_end_commit() decs committing
    local_inc(&cpu_buffer.committing);
// fail and let the caller try again
    return ERR_PTR(-EAGAIN);
// label;
// reset write
    rb_reset_tail(cpu_buffer, tail, info);
    return core::ptr::null_mut();
    }
// Slow path
#[no_mangle]
pub unsafe extern "C" fn rb_add_time_stamp(cpu_buffer: *mut ring_buffer_per_cpu, event: *mut ring_buffer_event, delta: u64, abs: bool) -> *mut c_void {
    if (abs) {
    event.type_len = RINGBUF_TYPE_TIME_STAMP;
    }
    else {
    event.type_len = RINGBUF_TYPE_TIME_EXTEND;
    }
// Not the first event on the page, or not delta?
    if (abs || rb_event_index(cpu_buffer, event)) {
    event.time_delta = delta & TS_MASK;
    event.array[0] = delta >> TS_SHIFT;
    } else {
// nope, just zero it
    event.time_delta = 0;
    event.array[0] = 0;
    }
    return skip_time_extend(event);
    }
#[no_mangle]
pub unsafe extern "C" fn rb_check_timestamp(cpu_buffer: *mut ring_buffer_per_cpu, info: *mut rb_event_info) {
    let mut write_stamp = 0;
    WARN_ONCE(1, "Delta way too big! %llu ts=%llu before=%llu after=%llu write stamp=%llu\n%s",
    (unsigned long long)info.delta,
    (unsigned long long)info.ts,
    (unsigned long long)info.before,
    (unsigned long long)info.after,
    (unsigned long long)({rb_time_read(&cpu_buffer.write_stamp, &write_stamp); write_stamp;}),
    sched_clock_stable() ? "" :
    "If you just came from a suspend/resume,\n"
    "please switch to the trace global clock:\n"
    "  echo global > /sys/kernel/tracing/trace_clock\n"
    "or add trace_clock=global to the kernel command line\n");
    }
#[no_mangle]
pub unsafe extern "C" fn rb_add_timestamp(cpu_buffer: *mut ring_buffer_per_cpu, event: *mut *mut ring_buffer_event, info: *mut rb_event_info, delta: *mut u64, length: *mut c_uint) {
    let mut abs = info.add_timestamp &
    (RB_ADD_STAMP_FORCE | RB_ADD_STAMP_ABSOLUTE);
    if (unlikely(info.delta > (1ULL << 59))) {
//
// Some timers can use more than 59 bits, and when a timestamp
// is added to the buffer, it will lose those bits.
//
    if (abs && (info.ts & TS_MSB)) {
    info.delta &= ABS_TS_MASK;
// did the clock go backwards
    } else if (info.before == info.after && info.before > info.ts) {
// not interrupted
    static int once;
//
// This is possible with a recalibrating of the TSC.
// Do not produce a call stack, but just report it.
//
    if (!once) {
    once += 1;
    pr_warn!("Ring buffer clock went backwards: %llu . %llu\n",
    info.before, info.ts);
    }
    } else {
    rb_check_timestamp(cpu_buffer, info);
    }
    if (!abs) {
    info.delta = 0;
    }
    }
// event = rb_add_time_stamp(cpu_buffer, *event, info->delta, abs);
// length -= RB_LEN_TIME_EXTEND;
// delta = 0;
    }
//
// rb_update_event - update event type and data
// @cpu_buffer: The per cpu buffer of the @event
// @event: the event to update
// @info: The info to update the @event with (contains length and delta)
//
// Update the type and data fields of the @event. The length
// is the actual size that is written to the ring buffer,
// and with this, we can determine what to place into the
// data field.
//
#[no_mangle]
pub unsafe extern "C" fn rb_update_event(cpu_buffer: *mut ring_buffer_per_cpu, event: *mut ring_buffer_event, info: *mut rb_event_info) {
pub static mut length: unsigned = 0;
pub static mut delta: u64 = 0;
pub static mut nest: c_uint = 0;
    if (!WARN_ON_ONCE!(nest >= MAX_NEST)) {
    cpu_buffer.event_stamp[nest] = info.ts;
    }
//
// If we need to add a timestamp, then we
// add it to the start of the reserved space.
//
    if (unlikely(info.add_timestamp)) {
    rb_add_timestamp(cpu_buffer, &event, info, &delta, &length);
    }
    event.time_delta = delta;
    length -= RB_EVNT_HDR_SIZE;
    if (length > RB_MAX_SMALL_DATA || RB_FORCE_8BYTE_ALIGNMENT) {
    event.type_len = 0;
    event.array[0] = length;
    } else {
    event.type_len = DIV_ROUND_UP(length, RB_ALIGNMENT);
    }
    }
#[no_mangle]
unsafe extern "C" fn rb_calculate_event_length(length: unsigned) -> unsigned {
pub static mut event: usize = 0; /* Used only for sizeof array */
// zero length can cause confusions
    if (!length) {
    length += 1;
    }
    if (length > RB_MAX_SMALL_DATA || RB_FORCE_8BYTE_ALIGNMENT) {
    length += sizeof!(event.array[0]);
    }
    length += RB_EVNT_HDR_SIZE;
    length = ALIGN(length, RB_ARCH_ALIGNMENT);
//
// In case the time delta is larger than the 27 bits for it
// in the header, we need to add a timestamp. If another
// event comes in when trying to discard this one to increase
// the length, then the timestamp will be added in the allocated
// space of this event. If length is bigger than the size needed
// for the TIME_EXTEND, then padding has to be used. The events
// length must be either RB_LEN_TIME_EXTEND, or greater than or equal
// to RB_LEN_TIME_EXTEND + 8, as 8 is the minimum size for padding.
// As length is a multiple of 4, we only need to worry if it
// is 12 (RB_LEN_TIME_EXTEND + 4).
//
    if (length == RB_LEN_TIME_EXTEND + RB_ALIGNMENT) {
    length += RB_ALIGNMENT;
    }
    return length;
    }
#[no_mangle]
pub unsafe extern "C" fn rb_try_to_discard(cpu_buffer: *mut ring_buffer_per_cpu, event: *mut ring_buffer_event) -> bool {
    unsigned long new_index, old_index;
pub static mut bpage: *mut c_void = core::ptr::null_mut();
    let mut addr = 0;
    new_index = rb_event_index(cpu_buffer, event);
    old_index = new_index + rb_event_ts_length(event);
    addr = rb_subbuf_start(cpu_buffer.buffer, (unsigned long)event);
    bpage = READ_ONCE(cpu_buffer.tail_page);
//
// Make sure the tail_page is still the same and
// the next write location is the end of this event
//
    if (bpage.page == addr && rb_page_write(bpage) == old_index) {
    let mut write_mask = local_read(&bpage.write) & ~RB_WRITE_MASK;
pub static mut event_length: c_ulong = 0;
//
// For the before_stamp to be different than the write_stamp
// to make sure that the next event adds an absolute
// value and does not rely on the saved write stamp, which
// is now going to be bogus.
//
// By setting the before_stamp to zero, the next event
// is not going to use the write_stamp and will instead
// create an absolute timestamp. This means there's no
// reason to update the wirte_stamp!
//
    rb_time_set(&cpu_buffer.before_stamp, 0);
//
// If an event were to come in now, it would see that the
// write_stamp and the before_stamp are different, and assume
// that this event just added itself before updating
// the write stamp. The interrupting event will fix the
// write stamp for us, and use an absolute timestamp.
//
// This is on the tail page. It is possible that
// a write could come in and move the tail page
// and write to the next page. That is fine
// because we just shorten what is on this page.
//
    old_index += write_mask;
    new_index += write_mask;
// caution: old_index gets updated on cmpxchg failure
    if (local_try_cmpxchg(&bpage.write, &old_index, new_index)) {
// update counters
    local_sub(event_length, &cpu_buffer.entries_bytes);
    return true;
    }
    }
// could not discard
    return false;
    }
#[no_mangle]
unsafe extern "C" fn rb_start_commit(cpu_buffer: *mut ring_buffer_per_cpu) {
    local_inc(&cpu_buffer.committing);
    local_inc(&cpu_buffer.commits);
    }
    static __always_inline void
    rb_set_commit_to_write(ring_buffer_per_cpu *cpu_buffer)
    {
    let mut max_count = 0;
//
// We only race with interrupts and NMIs on this CPU.
// If we own the commit event, then we can commit
// all others that interrupted us, since the interruptions
// are in stack format (they finish before they come
// back to us). This allows us to do a simple loop to
// assign the commit to the tail.
//
// label;
    max_count = cpu_buffer.nr_pages * 100;
    while (cpu_buffer.commit_page != READ_ONCE(cpu_buffer.tail_page)) {
    if (RB_WARN_ON(cpu_buffer, !(--max_count))) {
    return;
    }
    if (RB_WARN_ON(cpu_buffer,
    rb_is_reader_page(cpu_buffer.tail_page))) {
    return;
    }
//
// No need for a memory barrier here, as the update
// of the tail_page did it for this page.
//
    local_set(&cpu_buffer.commit_page.page.commit,
    rb_page_write(cpu_buffer.commit_page));
    rb_inc_page(&cpu_buffer.commit_page);
    if (cpu_buffer.ring_meta) {
    let mut meta = cpu_buffer.ring_meta;
    meta.commit_buffer = (unsigned long)cpu_buffer.commit_page.page;
    }
// add barrier to keep gcc from optimizing too much
    barrier();
    }
    while (rb_commit_index(cpu_buffer) !=
    rb_page_write(cpu_buffer.commit_page)) {
// Make sure the readers see the content of what is committed.
    smp_wmb();
    local_set(&cpu_buffer.commit_page.page.commit,
    rb_page_write(cpu_buffer.commit_page));
    RB_WARN_ON(cpu_buffer,
    rb_page_commit(cpu_buffer.commit_page) & ~RB_WRITE_MASK);
    barrier();
    }
// again, keep gcc from optimizing
    barrier();
//
// If an interrupt came in just after the first while loop
// and pushed the tail page forward, we will be left with
// a dangling commit that will never go forward.
//
    if (unlikely(cpu_buffer.commit_page != READ_ONCE(cpu_buffer.tail_page))) {
// goto;
    }
    }
#[no_mangle]
unsafe extern "C" fn rb_end_commit(cpu_buffer: *mut ring_buffer_per_cpu) -> __always_inline void {
    let mut commits = 0;
    if (RB_WARN_ON(cpu_buffer,
    !local_read(&cpu_buffer.committing))) {
    return;
    }
// label;
    commits = local_read(&cpu_buffer.commits);
// synchronize with interrupts
    barrier();
    if (local_read(&cpu_buffer.committing) == 1) {
    rb_set_commit_to_write(cpu_buffer);
    }
    local_dec(&cpu_buffer.committing);
// synchronize with interrupts
    barrier();
//
// Need to account for interrupts coming in between the
// updating of the commit page and the clearing of the
// committing counter.
//
    if (unlikely(local_read(&cpu_buffer.commits) != commits) &&
    !local_read(&cpu_buffer.committing)) {
    local_inc(&cpu_buffer.committing);
// goto;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rb_event_discard(event: *mut ring_buffer_event) {
    if (extended_time(event)) {
    event = skip_time_extend(event);
    }
// array[0] holds the actual length for the discarded event
    event.array[0] = rb_event_data_length(event) - RB_EVNT_HDR_SIZE;
    event.type_len = RINGBUF_TYPE_PADDING;
// time delta must be non zero
    if (!event.time_delta) {
    event.time_delta = 1;
    }
    }
#[no_mangle]
unsafe extern "C" fn rb_commit(cpu_buffer: *mut ring_buffer_per_cpu) {
    local_inc(&cpu_buffer.entries);
    rb_end_commit(cpu_buffer);
    }
#[no_mangle]
pub unsafe extern "C" fn rb_irq_work_queue(irq_work: *mut rb_irq_work) -> bool {
    let mut cpu = 0;
// irq_work_queue_on() is not NMI-safe
    if (unlikely(in_nmi())) {
    return irq_work_queue(&irq_work.work);
    }
//
// If CPU isolation is not active, cpu is always the current
// CPU, and the following is equivallent to irq_work_queue().
//
    cpu = housekeeping_any_cpu(HK_TYPE_KERNEL_NOISE);
    return irq_work_queue_on(&irq_work.work, cpu);
    }
    static __always_inline void
    rb_wakeups(trace_buffer *buffer, ring_buffer_per_cpu *cpu_buffer)
    {
    if (buffer.irq_work.waiters_pending) {
    buffer.irq_work.waiters_pending = false;
// irq_work_queue() supplies it's own memory barriers
    rb_irq_work_queue(&buffer.irq_work);
    }
    if (cpu_buffer.irq_work.waiters_pending) {
    cpu_buffer.irq_work.waiters_pending = false;
// irq_work_queue() supplies it's own memory barriers
    rb_irq_work_queue(&cpu_buffer.irq_work);
    }
    if (cpu_buffer.last_pages_touch == local_read(&cpu_buffer.pages_touched)) {
    return;
    }
    if (cpu_buffer.reader_page == cpu_buffer.commit_page) {
    return;
    }
    if (!cpu_buffer.irq_work.full_waiters_pending) {
    return;
    }
    cpu_buffer.last_pages_touch = local_read(&cpu_buffer.pages_touched);
    if (!full_hit(buffer, cpu_buffer.cpu, cpu_buffer.shortest_full)) {
    return;
    }
    cpu_buffer.irq_work.wakeup_full = true;
    cpu_buffer.irq_work.full_waiters_pending = false;
// irq_work_queue() supplies it's own memory barriers
    rb_irq_work_queue(&cpu_buffer.irq_work);
    }

    do_ftrace_record_recursion(_THIS_IP_, _RET_IP_)

//
// The lock and unlock are done within a preempt disable section.
// The current_context per_cpu variable can only be modified
// by the current task between lock and unlock. But it can
// be modified more than once via an interrupt. To pass this
// information from the lock to the unlock without having to
// access the 'in_interrupt()' functions again (which do show
// a bit of overhead in something as critical as function tracing,
// we use a bitmask trick.
//
// bit 1 =  NMI context
// bit 2 =  IRQ context
// bit 3 =  SoftIRQ context
// bit 4 =  normal context.
//
// This works because this is the order of contexts that can
// preempt other contexts. A SoftIRQ never preempts an IRQ
// context.
//
// When the context is determined, the corresponding bit is
// checked and set (if it was set, then a recursion of that context
// happened).
//
// On unlock, we need to clear this bit. To do so, just subtract
// 1 from the current_context and AND it to itself.
//
// (binary)
// 101 - 1 = 100
// 101 & 100 = 100 (clearing bit zero)
//
// 1010 - 1 = 1001
// 1010 & 1001 = 1000 (clearing bit 1)
//
// The least significant bit can be cleared this way, and it
// just so happens that it is the same bit corresponding to
// the current context.
//
// Now the TRANSITION bit breaks the above slightly. The TRANSITION bit
// is set when a recursion is detected at the current context, and if
// the TRANSITION bit is already set, it will fail the recursion.
// This is needed because there's a lag between the changing of
// interrupt context and updating the preempt count. In this case,
// a false positive will be found. To handle this, one extra recursion
// is allowed, and this is done by the TRANSITION bit. If the TRANSITION
// bit is already set, then it is considered a recursion and the function
// ends. Otherwise, the TRANSITION bit is set, and that bit is returned.
//
// On the trace_recursive_unlock(), the TRANSITION bit will be the first
// to be cleared. Even if it wasn't the context that set it. That is,
// if an interrupt comes in while NORMAL bit is set and the ring buffer
// is called before preempt_count() is updated, since the check will
// be on the NORMAL bit, the TRANSITION bit will then be set. If an
// NMI then comes in, it will set the NMI bit, but when the NMI code
// does the trace_recursive_unlock() it will clear the TRANSITION bit
// and leave the NMI bit set. But this is fine, because the interrupt
// code that set the TRANSITION bit will then clear the NMI bit when it
// calls trace_recursive_unlock(). If another NMI comes in, it will
// set the TRANSITION bit and continue.
//
// Note: The TRANSITION bit only handles a single transition between context.
//
    static __always_inline bool
    trace_recursive_lock(ring_buffer_per_cpu *cpu_buffer)
    {
pub static mut val: c_uint = 0;
pub static mut bit: c_int = 0;
    bit = RB_CTX_NORMAL - bit;
    if (unlikely(val & (1 << (bit + cpu_buffer.nest)))) {
//
// It is possible that this was called by transitioning
// between interrupt context, and preempt_count() has not
// been updated yet. In this case, use the TRANSITION bit.
//
    bit = RB_CTX_TRANSITION;
    if (val & (1 << (bit + cpu_buffer.nest))) {
    do_ring_buffer_record_recursion();
    return true;
    }
    }
    val |= (1 << (bit + cpu_buffer.nest));
    cpu_buffer.current_context = val;
    return false;
    }
    static __always_inline void
    trace_recursive_unlock(ring_buffer_per_cpu *cpu_buffer)
    {
    cpu_buffer.current_context &=
    cpu_buffer.current_context - (1 << cpu_buffer.nest);
    }
// The recursive locking above uses 5 bits
pub const NESTED_BITS: c_int = 5;
//
// ring_buffer_nest_start - Allow to trace while nested
// @buffer: The ring buffer to modify
//
// The ring buffer has a safety mechanism to prevent recursion.
// But there may be a case where a trace needs to be done while
// tracing something else. In this case, calling this function
// will allow this function to nest within a currently active
// ring_buffer_lock_reserve().
//
// Call this function before calling another ring_buffer_lock_reserve() and
// call ring_buffer_nest_end() after the nested ring_buffer_unlock_commit().
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_nest_start(buffer: *mut trace_buffer) {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
// Enabled by ring_buffer_nest_end()
    preempt_disable_notrace();
    cpu = raw_smp_processor_id();
    cpu_buffer = buffer.buffers[cpu];
// This is the shift value for the above recursive locking
    cpu_buffer.nest += NESTED_BITS;
    }
//
// ring_buffer_nest_end - Allow to trace while nested
// @buffer: The ring buffer to modify
//
// Must be called after ring_buffer_nest_start() and after the
// ring_buffer_unlock_commit().
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_nest_end(buffer: *mut trace_buffer) {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
// disabled by ring_buffer_nest_start()
    cpu = raw_smp_processor_id();
    cpu_buffer = buffer.buffers[cpu];
// This is the shift value for the above recursive locking
    cpu_buffer.nest -= NESTED_BITS;
    preempt_enable_notrace();
    }
//
// ring_buffer_unlock_commit - commit a reserved
// @buffer: The buffer to commit to
//
// This commits the data to the ring buffer, and releases any locks held.
//
// Must be paired with ring_buffer_lock_reserve.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_unlock_commit(buffer: *mut trace_buffer) -> c_int {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
pub static mut cpu: c_int = 0;
    cpu_buffer = buffer.buffers[cpu];
    rb_commit(cpu_buffer);
    rb_wakeups(buffer, cpu_buffer);
    trace_recursive_unlock(cpu_buffer);
    preempt_enable_notrace();
    return 0;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_unlock_commit);
// Special value to validate all deltas on a page.

    static const char *show_irq_str(int bits)
    {
    static const char * type[] = {
    ".",	// 0
    "s",	// 1
    "h",	// 2
    "Hs",	// 3
    "n",	// 4
    "Ns",	// 5
    "Nh",	// 6
    "NHs",	// 7
    };
    return type[bits];
    }
// Assume this is a trace event
    static const char *show_flags(ring_buffer_event *event)
    {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut bits: c_int = 0;
    if (rb_event_data_length(event) - RB_EVNT_HDR_SIZE < sizeof!(*entry)) {
    return "X";
    }
    entry = ring_buffer_event_data(event);
    if (entry.flags & TRACE_FLAG_SOFTIRQ) {
    bits |= 1;
    }
    if (entry.flags & TRACE_FLAG_HARDIRQ) {
    bits |= 2;
    }
    if (entry.flags & TRACE_FLAG_NMI) {
    bits |= 4;
    }
    return show_irq_str(bits);
    }
    static const char *show_irq(ring_buffer_event *event)
    {
pub static mut entry: *mut c_void = core::ptr::null_mut();
    if (rb_event_data_length(event) - RB_EVNT_HDR_SIZE < sizeof!(*entry)) {
    return "";
    }
    entry = ring_buffer_event_data(event);
    if (entry.flags & TRACE_FLAG_IRQS_OFF) {
    return "d";
    }
    return "";
    }
    static const char *show_interrupt_level(void)
    {
pub static mut pc: c_ulong = 0;
pub static mut level: c_uchar = 0;
    if (pc & SOFTIRQ_OFFSET) {
    level |= 1;
    }
    if (pc & HARDIRQ_MASK) {
    level |= 2;
    }
    if (pc & NMI_MASK) {
    level |= 4;
    }
    return show_irq_str(level);
    }
#[no_mangle]
pub unsafe extern "C" fn dump_buffer_page(dpage: *mut buffer_data_page, info: *mut rb_event_info, tail: c_ulong) {
pub static mut event: *mut c_void = core::ptr::null_mut();
    u64 ts, delta;
    let mut e = 0;
    ts = dpage.time_stamp;
    pr_warn!("  [%lld] PAGE TIME STAMP\n", ts);
    for (e = 0; e < tail; e += rb_event_length(event)) {
    event = (dpage.data + e);
    match (event.type_len) {
    RINGBUF_TYPE_TIME_EXTEND => {
    delta = rb_event_time_stamp(event);
    ts += delta;
    pr_warn!(" 0x%x: [%lld] delta:%lld TIME EXTEND\n",
    e, ts, delta);
    // break;
    }
    RINGBUF_TYPE_TIME_STAMP => {
    delta = rb_event_time_stamp(event);
    ts = rb_fix_abs_ts(delta, ts);
    pr_warn!(" 0x%x:  [%lld] absolute:%lld TIME STAMP\n",
    e, ts, delta);
    // break;
    }
    RINGBUF_TYPE_PADDING => {
    ts += event.time_delta;
    pr_warn!(" 0x%x:  [%lld] delta:%d PADDING\n",
    e, ts, event.time_delta);
    // break;
    }
    RINGBUF_TYPE_DATA => {
    ts += event.time_delta;
    pr_warn!(" 0x%x:  [%lld] delta:%d %s%s\n",
    e, ts, event.time_delta,
    show_flags(event), show_irq(event));
    // break;
    }
    _ => {
    // break;
    }
    }
    }
    pr_warn!("expected end:0x%lx last event actually ended at:0x%x\n", tail, e);
    }
pub static mut atomic_t: usize = 0;
    static atomic_t ts_dump;

    do {								
// If another report is happening, ignore this one */	
    if (atomic_inc_return(&ts_dump) != 1) {			
    atomic_dec(&ts_dump);				
// goto;					
    }							
    atomic_inc(&cpu_buffer.record_disabled);		
    pr_warn!(fmt, ##__VA_ARGS__);				
    dump_buffer_page(dpage, info, tail);			
    atomic_dec(&ts_dump);					
// There's some cases in boot up that this can happen */ 
    if (WARN_ON_ONCE!(system_state != SYSTEM_BOOTING))	 {
// Do not re-enable checking */			
    return;						
    }
    } while (0)
//
// Check if the current event time stamp matches the deltas on
// the buffer page.
//
#[no_mangle]
pub unsafe extern "C" fn check_buffer(cpu_buffer: *mut ring_buffer_per_cpu, info: *mut rb_event_info, tail: c_ulong) {
pub static mut dpage: *mut c_void = core::ptr::null_mut();
    u64 ts, delta;
pub static mut full: bool = false;
    let mut ret = 0;
    dpage = info.tail_page.page;
    if (tail == CHECK_FULL_PAGE) {
    full = true;
    tail = rb_data_page_commit(dpage);
    } else if (info.add_timestamp &
    (RB_ADD_STAMP_FORCE | RB_ADD_STAMP_ABSOLUTE)) {
// Ignore events with absolute time stamps
    return;
    }
//
// Do not check the first event (skip possible extends too).
// Also do not check if previous events have not been committed.
//
    if (tail <= 8 || tail > rb_data_page_commit(dpage)) {
    return;
    }
//
// If this interrupted another event,
//
    if (atomic_inc_return(this_cpu_ptr(&checking)) != 1) {
// goto;
    }
    ret = rb_read_data_buffer(dpage, tail, cpu_buffer.cpu, &ts, &delta);
    if (ret < 0) {
    if (delta < ts) {
    buffer_warn_return("[CPU: %d]ABSOLUTE TIME WENT BACKWARDS: last ts: %lld absolute ts: %lld clock:%pS\n",
    cpu_buffer.cpu, ts, delta,
    cpu_buffer.buffer.clock);
// goto;
    }
    }
    if ((full && ts > info.ts) ||
    (!full && ts + info.delta != info.ts)) {
    buffer_warn_return("[CPU: %d]TIME DOES NOT MATCH expected:%lld actual:%lld delta:%lld before:%lld after:%lld%s context:%s\ntrace clock:%pS",
    cpu_buffer.cpu,
    ts + info.delta, info.ts, info.delta,
    info.before, info.after,
    full ? " (full)" : "", show_interrupt_level(),
    cpu_buffer.buffer.clock);
    }
// label;
    atomic_dec(this_cpu_ptr(&checking));
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: check_buffer
pub unsafe extern "C" fn check_buffer_dup(cpu_buffer: *mut ring_buffer_per_cpu, info: *mut rb_event_info, tail: c_ulong) {
    }

#[no_mangle]
pub unsafe extern "C" fn __rb_reserve_next(cpu_buffer: *mut ring_buffer_per_cpu, info: *mut rb_event_info) -> *mut c_void {
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut tail_page: *mut c_void = core::ptr::null_mut();
    unsigned long tail, write, w;
// Don't let the compiler play games with cpu_buffer->tail_page
    tail_page = info.tail_page = READ_ONCE(cpu_buffer.tail_page);
// A*/	w = local_read(&tail_page->write) & RB_WRITE_MASK;
    barrier();
    rb_time_read(&cpu_buffer.before_stamp, &info.before);
    rb_time_read(&cpu_buffer.write_stamp, &info.after);
    barrier();
    info.ts = rb_time_stamp(cpu_buffer.buffer);
    if ((info.add_timestamp & RB_ADD_STAMP_ABSOLUTE)) {
    info.delta = info.ts;
    } else {
//
// If interrupting an event time update, we may need an
// absolute timestamp.
// Don't bother if this is the start of a new page (w == 0).
//
    if (!w) {
// Use the sub-buffer timestamp
    info.delta = 0;
    } else if (unlikely(info.before != info.after)) {
    info.add_timestamp |= RB_ADD_STAMP_FORCE | RB_ADD_STAMP_EXTEND;
    info.length += RB_LEN_TIME_EXTEND;
    } else {
    info.delta = info.ts - info.after;
    if (unlikely(test_time_stamp(info.delta))) {
    info.add_timestamp |= RB_ADD_STAMP_EXTEND;
    info.length += RB_LEN_TIME_EXTEND;
    }
    }
    }
// B*/	rb_time_set(&cpu_buffer->before_stamp, info->ts);
// C*/	write = local_add_return(info->length, &tail_page->write);
// set write to only the index of the write
    write &= RB_WRITE_MASK;
    tail = write - info.length;
// See if we shot pass the end of this buffer page
    if (unlikely(write > rb_page_capacity(tail_page))) {
    check_buffer(cpu_buffer, info, CHECK_FULL_PAGE);
    return rb_move_tail(cpu_buffer, tail, info);
    }
    if (likely(tail == w)) {
// Nothing interrupted us between A and C
// D*/		rb_time_set(&cpu_buffer->write_stamp, info->ts);
//
// If something came in between C and D, the write stamp
// may now not be in sync. But that's fine as the before_stamp
// will be different and then next event will just be forced
// to use an absolute timestamp.
//
    if (likely(!(info.add_timestamp &
    (RB_ADD_STAMP_FORCE | RB_ADD_STAMP_ABSOLUTE)))) {
// This did not interrupt any time update
    info.delta = info.ts - info.after;
    }
    else {
// Just use full timestamp for interrupting event
    info.delta = info.ts;
    }
    check_buffer(cpu_buffer, info, tail);
    } else {
    let mut ts = 0;
// SLOW PATH - Interrupted between A and C
// Save the old before_stamp
    rb_time_read(&cpu_buffer.before_stamp, &info.before);
//
// Read a new timestamp and update the before_stamp to make
// the next event after this one force using an absolute
// timestamp. This is in case an interrupt were to come in
// between E and F.
//
    ts = rb_time_stamp(cpu_buffer.buffer);
    rb_time_set(&cpu_buffer.before_stamp, ts);
    barrier();
// E*/		rb_time_read(&cpu_buffer->write_stamp, &info->after);
    barrier();
// F*/		if (write == (local_read(&tail_page->write) & RB_WRITE_MASK) &&
    info.after == info.before && info.after < ts) {
//
// Nothing came after this event between C and F, it is
// safe to use info->after for the delta as it
// matched info->before and is still valid.
//
    info.delta = ts - info.after;
    } else {
//
// Interrupted between C and F:
// Lost the previous events time stamp. Just set the
// delta to zero, and this will be the same time as
// the event this event interrupted. And the events that
// came after this will still be correct (as they would
// have built their delta on the previous event.
//
    info.delta = 0;
    }
    info.ts = ts;
    info.add_timestamp &= ~RB_ADD_STAMP_FORCE;
    }
//
// If this is the first commit on the page, then it has the same
// timestamp as the page itself.
//
    if (unlikely(!tail && !(info.add_timestamp &
    (RB_ADD_STAMP_FORCE | RB_ADD_STAMP_ABSOLUTE)))) {
    info.delta = 0;
    }
// We reserved something on the buffer
    event = __rb_page_index(tail_page, tail);
    rb_update_event(cpu_buffer, event, info);
    local_inc(&tail_page.entries);
//
// If this is the first commit on the page, then update
// its timestamp.
//
    if (unlikely(!tail)) {
    tail_page.page.time_stamp = info.ts;
    }
// account for these added bytes
    local_add(info.length, &cpu_buffer.entries_bytes);
    return event;
    }
    static __always_inline struct ring_buffer_event *
    rb_reserve_next_event(trace_buffer *buffer, ring_buffer_per_cpu *cpu_buffer,
    unsigned long length)
    {
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut info: usize = 0;
pub static mut nr_loops: c_int = 0;
    let mut add_ts_default = 0;
//
// ring buffer does cmpxchg as well as atomic64 operations
// (which some archs use locking for atomic64), make sure this
// is safe in NMI context
//
    if ((!IS_ENABLED!(CONFIG_ARCH_HAVE_NMI_SAFE_CMPXCHG) ||
    IS_ENABLED!(CONFIG_GENERIC_ATOMIC64)) &&
    (unlikely(in_nmi()))) {
    return core::ptr::null_mut();
    }
    rb_start_commit(cpu_buffer);
// The commit page can not change after this

//
// Due to the ability to swap a cpu buffer from a buffer
// it is possible it was swapped before we committed.
// (committing stops a swap). We check for it here and
// if it happened, we have to fail the write.
//
    barrier();
    if (unlikely(READ_ONCE(cpu_buffer.buffer) != buffer)) {
    local_dec(&cpu_buffer.committing);
    local_dec(&cpu_buffer.commits);
    return core::ptr::null_mut();
    }

    info.length = rb_calculate_event_length(length);
    if (ring_buffer_time_stamp_abs(cpu_buffer.buffer)) {
    add_ts_default = RB_ADD_STAMP_ABSOLUTE;
    info.length += RB_LEN_TIME_EXTEND;
    if (info.length > rb_subbuf_max_data_size(cpu_buffer.buffer)) {
// goto;
    }
    } else {
    add_ts_default = RB_ADD_STAMP_NONE;
    }
// label;
    info.add_timestamp = add_ts_default;
    info.delta = 0;
//
// We allow for interrupts to reenter here and do a trace.
// If one does, it will cause this original code to loop
// back here. Even with heavy interrupts happening, this
// should only happen a few times in a row. If this happens
// 1000 times in a row, there must be either an interrupt
// storm or we have something buggy.
// Bail!
//
    if (RB_WARN_ON(cpu_buffer, ++nr_loops > 1000)) {
// goto;
    }
    event = __rb_reserve_next(cpu_buffer, &info);
    if (unlikely(PTR_ERR(event) == -EAGAIN)) {
    if (info.add_timestamp & (RB_ADD_STAMP_FORCE | RB_ADD_STAMP_EXTEND)) {
    info.length -= RB_LEN_TIME_EXTEND;
    }
// goto;
    }
    if (likely(event)) {
    return event;
    }
// label;
    rb_end_commit(cpu_buffer);
    return core::ptr::null_mut();
    }
//
// ring_buffer_lock_reserve - reserve a part of the buffer
// @buffer: the ring buffer to reserve from
// @length: the length of the data to reserve (excluding event header)
//
// Returns a reserved event on the ring buffer to copy directly to.
// The user of this interface will need to get the body to write into
// and can use the ring_buffer_event_data() interface.
//
// The length is the length of the data needed, not the event length
// which also includes the event header.
//
// Must be paired with ring_buffer_unlock_commit, unless NULL is returned.
// If NULL is returned, then nothing has been allocated or locked.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_lock_reserve(buffer: *mut trace_buffer, length: c_ulong) -> *mut c_void {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
// If we are tracing schedule, we don't want to recurse
    preempt_disable_notrace();
    if (unlikely(atomic_read(&buffer.record_disabled))) {
// goto;
    }
    cpu = raw_smp_processor_id();
    if (unlikely(!cpumask_test_cpu(cpu, buffer.cpumask))) {
// goto;
    }
    cpu_buffer = buffer.buffers[cpu];
    if (unlikely(atomic_read(&cpu_buffer.record_disabled))) {
// goto;
    }
    if (unlikely(length > rb_subbuf_max_data_size(buffer))) {
// goto;
    }
    if (unlikely(trace_recursive_lock(cpu_buffer))) {
// goto;
    }
    event = rb_reserve_next_event(buffer, cpu_buffer, length);
    if (!event) {
// goto;
    }
    return event;
// label;
    trace_recursive_unlock(cpu_buffer);
// label;
    preempt_enable_notrace();
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(ring_buffer_lock_reserve);
//
// Decrement the entries to the page that an event is on.
// The event does not even need to exist, only the pointer
// to the page it is on. This may only be called before the commit
// takes place.
//
#[no_mangle]
pub unsafe extern "C" fn rb_decrement_entry(cpu_buffer: *mut ring_buffer_per_cpu, event: *mut ring_buffer_event) {
pub static mut addr: c_ulong = 0;
    let mut bpage = cpu_buffer.commit_page;
pub static mut start: *mut c_void = core::ptr::null_mut();
    addr = rb_subbuf_start(cpu_buffer.buffer, addr);
// Do the likely case first
    if (likely(bpage.page == addr)) {
    local_dec(&bpage.entries);
    return;
    }
//
// Because the commit page may be on the reader page we
// start with the next page and check the end loop there.
//
    rb_inc_page(&bpage);
    start = bpage;
    do {
    if (bpage.page == addr) {
    local_dec(&bpage.entries);
    return;
    }
    rb_inc_page(&bpage);
    } while (bpage != start);
// commit not part of this buffer??
    RB_WARN_ON(cpu_buffer, 1);
    }
//
// ring_buffer_discard_commit - discard an event that has not been committed
// @buffer: the ring buffer
// @event: non committed event to discard
//
// Sometimes an event that is in the ring buffer needs to be ignored.
// This function lets the user discard an event in the ring buffer
// and then that event will not be read later.
//
// This function only works if it is called before the item has been
// committed. It will try to free the event from the ring buffer
// if another event has not been added behind it.
//
// If another event has been added behind it, it will set the event
// up as discarded, and perform the commit.
//
// If this function is called, do not call ring_buffer_unlock_commit on
// the event.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_discard_commit(buffer: *mut trace_buffer, event: *mut ring_buffer_event) {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
// The event is discarded regardless
    rb_event_discard(event);
    cpu = smp_processor_id();
    cpu_buffer = buffer.buffers[cpu];
//
// This must only be called if the event has not been
// committed yet. Thus we can assume that preemption
// is still disabled.
//
    RB_WARN_ON(buffer, !local_read(&cpu_buffer.committing));
    rb_decrement_entry(cpu_buffer, event);
    rb_try_to_discard(cpu_buffer, event);
    rb_end_commit(cpu_buffer);
    trace_recursive_unlock(cpu_buffer);
    preempt_enable_notrace();
    }
    EXPORT_SYMBOL_GPL(ring_buffer_discard_commit);
//
// ring_buffer_write - write data to the buffer without reserving
// @buffer: The ring buffer to write to.
// @length: The length of the data being written (excluding the event header)
// @data: The data to write to the buffer.
//
// This is like ring_buffer_lock_reserve and ring_buffer_unlock_commit as
// one function. If you already have the data to write to the buffer, it
// may be easier to simply call this function.
//
// Note, like ring_buffer_lock_reserve, the length is the length of the data
// and not the length of the event which would hold the header.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_write(buffer: *mut trace_buffer, length: c_ulong, data: *mut c_void) -> c_int {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut body: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    let mut cpu = 0;
    guard(preempt_notrace)();
    if (atomic_read(&buffer.record_disabled)) {
    return -EBUSY;
    }
    cpu = raw_smp_processor_id();
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return -EBUSY;
    }
    cpu_buffer = buffer.buffers[cpu];
    if (atomic_read(&cpu_buffer.record_disabled)) {
    return -EBUSY;
    }
    if (length > rb_subbuf_max_data_size(buffer)) {
    return -EBUSY;
    }
    if (unlikely(trace_recursive_lock(cpu_buffer))) {
    return -EBUSY;
    }
    event = rb_reserve_next_event(buffer, cpu_buffer, length);
    if (!event) {
// goto;
    }
    body = rb_event_data(event);
    memcpy(body, data, length);
    rb_commit(cpu_buffer);
    rb_wakeups(buffer, cpu_buffer);
    ret = 0;
// label;
    trace_recursive_unlock(cpu_buffer);
    return ret;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_write);
//
// The total entries in the ring buffer is the running counter
// of entries entered into the ring buffer, minus the sum of
// the entries read from the ring buffer and the number of
// entries that were overwritten.
//
#[no_mangle]
pub unsafe extern "C" fn rb_num_of_entries(cpu_buffer: *mut ring_buffer_per_cpu) -> c_ulong {
    return local_read(&cpu_buffer.entries) -
    (local_read(&cpu_buffer.overrun) + cpu_buffer.read);
    }
#[no_mangle]
unsafe extern "C" fn rb_per_cpu_empty(cpu_buffer: *mut ring_buffer_per_cpu) -> bool {
    return !rb_num_of_entries(cpu_buffer);
    }
//
// ring_buffer_record_disable - stop all writes into the buffer
// @buffer: The ring buffer to stop writes to.
//
// This prevents all writes to the buffer. Any attempt to write
// to the buffer after this will fail and return NULL.
//
// The caller should call synchronize_rcu() after this.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_record_disable(buffer: *mut trace_buffer) {
    atomic_inc(&buffer.record_disabled);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_record_disable);
//
// ring_buffer_record_enable - enable writes to the buffer
// @buffer: The ring buffer to enable writes
//
// Note, multiple disables will need the same number of enables
// to truly enable the writing (much like preempt_disable).
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_record_enable(buffer: *mut trace_buffer) {
    atomic_dec(&buffer.record_disabled);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_record_enable);
//
// ring_buffer_record_off - stop all writes into the buffer
// @buffer: The ring buffer to stop writes to.
//
// This prevents all writes to the buffer. Any attempt to write
// to the buffer after this will fail and return NULL.
//
// This is different than ring_buffer_record_disable() as
// it works like an on/off switch, where as the disable() version
// must be paired with a enable().
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_record_off(buffer: *mut trace_buffer) {
    let mut rd = 0;
    let mut new_rd = 0;
    rd = atomic_read(&buffer.record_disabled);
    do {
    new_rd = rd | RB_BUFFER_OFF;
    } while (!atomic_try_cmpxchg(&buffer.record_disabled, &rd, new_rd));
    }
    EXPORT_SYMBOL_GPL(ring_buffer_record_off);
//
// ring_buffer_record_on - restart writes into the buffer
// @buffer: The ring buffer to start writes to.
//
// This enables all writes to the buffer that was disabled by
// ring_buffer_record_off().
//
// This is different than ring_buffer_record_enable() as
// it works like an on/off switch, where as the enable() version
// must be paired with a disable().
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_record_on(buffer: *mut trace_buffer) {
    let mut rd = 0;
    let mut new_rd = 0;
    rd = atomic_read(&buffer.record_disabled);
    do {
    new_rd = rd & ~RB_BUFFER_OFF;
    } while (!atomic_try_cmpxchg(&buffer.record_disabled, &rd, new_rd));
    }
    EXPORT_SYMBOL_GPL(ring_buffer_record_on);
//
// ring_buffer_record_is_on - return true if the ring buffer can write
// @buffer: The ring buffer to see if write is enabled
//
// Returns true if the ring buffer is in a state that it accepts writes.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_record_is_on(buffer: *mut trace_buffer) -> bool {
    return !atomic_read(&buffer.record_disabled);
    }
//
// ring_buffer_record_is_set_on - return true if the ring buffer is set writable
// @buffer: The ring buffer to see if write is set enabled
//
// Returns true if the ring buffer is set writable by ring_buffer_record_on().
// Note that this does NOT mean it is in a writable state.
//
// It may return true when the ring buffer has been disabled by
// ring_buffer_record_disable(), as that is a temporary disabling of
// the ring buffer.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_record_is_set_on(buffer: *mut trace_buffer) -> bool {
    return !(atomic_read(&buffer.record_disabled) & RB_BUFFER_OFF);
    }
//
// ring_buffer_record_is_on_cpu - return true if the ring buffer can write
// @buffer: The ring buffer to see if write is enabled
// @cpu: The CPU to test if the ring buffer can write too
//
// Returns true if the ring buffer is in a state that it accepts writes
// for a particular CPU.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_record_is_on_cpu(buffer: *mut trace_buffer, cpu: c_int) -> bool {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    cpu_buffer = buffer.buffers[cpu];
    return ring_buffer_record_is_set_on(buffer) &&
    !atomic_read(&cpu_buffer.record_disabled);
    }
//
// ring_buffer_record_disable_cpu - stop all writes into the cpu_buffer
// @buffer: The ring buffer to stop writes to.
// @cpu: The CPU buffer to stop
//
// This prevents all writes to the buffer. Any attempt to write
// to the buffer after this will fail and return NULL.
//
// The caller should call synchronize_rcu() after this.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_record_disable_cpu(buffer: *mut trace_buffer, cpu: c_int) {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return;
    }
    cpu_buffer = buffer.buffers[cpu];
    atomic_inc(&cpu_buffer.record_disabled);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_record_disable_cpu);
//
// ring_buffer_record_enable_cpu - enable writes to the buffer
// @buffer: The ring buffer to enable writes
// @cpu: The CPU to enable.
//
// Note, multiple disables will need the same number of enables
// to truly enable the writing (much like preempt_disable).
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_record_enable_cpu(buffer: *mut trace_buffer, cpu: c_int) {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return;
    }
    cpu_buffer = buffer.buffers[cpu];
    atomic_dec(&cpu_buffer.record_disabled);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_record_enable_cpu);
//
// ring_buffer_oldest_event_ts - get the oldest event timestamp from the buffer
// @buffer: The ring buffer
// @cpu: The per CPU buffer to read from.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_oldest_event_ts(buffer: *mut trace_buffer, cpu: c_int) -> u64 {
    let mut flags = 0;
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
pub static mut bpage: *mut c_void = core::ptr::null_mut();
pub static mut ret: u64 = 0;
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return 0;
    }
    cpu_buffer = buffer.buffers[cpu];
    raw_spin_lock_irqsave(&cpu_buffer.reader_lock, flags);
//
// if the tail is on reader_page, oldest time stamp is on the reader
// page
//
    if (cpu_buffer.tail_page == cpu_buffer.reader_page) {
    bpage = cpu_buffer.reader_page;
    }
    else {
    bpage = rb_set_head_page(cpu_buffer);
    }
    if (bpage) {
    ret = bpage.page.time_stamp;
    }
    raw_spin_unlock_irqrestore(&cpu_buffer.reader_lock, flags);
    return ret;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_oldest_event_ts);
//
// ring_buffer_bytes_cpu - get the number of bytes unconsumed in a cpu buffer
// @buffer: The ring buffer
// @cpu: The per CPU buffer to read from.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_bytes_cpu(buffer: *mut trace_buffer, cpu: c_int) -> c_ulong {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return 0;
    }
    cpu_buffer = buffer.buffers[cpu];
    ret = local_read(&cpu_buffer.entries_bytes) - cpu_buffer.read_bytes;
    return ret;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_bytes_cpu);
//
// ring_buffer_entries_cpu - get the number of entries in a cpu buffer
// @buffer: The ring buffer
// @cpu: The per CPU buffer to get the entries from.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_entries_cpu(buffer: *mut trace_buffer, cpu: c_int) -> c_ulong {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return 0;
    }
    cpu_buffer = buffer.buffers[cpu];
    return rb_num_of_entries(cpu_buffer);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_entries_cpu);
//
// ring_buffer_overrun_cpu - get the number of overruns caused by the ring
// buffer wrapping around (only if RB_FL_OVERWRITE is on).
// @buffer: The ring buffer
// @cpu: The per CPU buffer to get the number of overruns from
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_overrun_cpu(buffer: *mut trace_buffer, cpu: c_int) -> c_ulong {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return 0;
    }
    cpu_buffer = buffer.buffers[cpu];
    ret = local_read(&cpu_buffer.overrun);
    return ret;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_overrun_cpu);
//
// ring_buffer_commit_overrun_cpu - get the number of overruns caused by
// commits failing due to the buffer wrapping around while there are uncommitted
// events, such as during an interrupt storm.
// @buffer: The ring buffer
// @cpu: The per CPU buffer to get the number of overruns from
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_commit_overrun_cpu(buffer: *mut trace_buffer, cpu: c_int) -> c_ulong {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return 0;
    }
    cpu_buffer = buffer.buffers[cpu];
    ret = local_read(&cpu_buffer.commit_overrun);
    return ret;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_commit_overrun_cpu);
//
// ring_buffer_dropped_events_cpu - get the number of dropped events caused by
// the ring buffer filling up (only if RB_FL_OVERWRITE is off).
// @buffer: The ring buffer
// @cpu: The per CPU buffer to get the number of overruns from
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_dropped_events_cpu(buffer: *mut trace_buffer, cpu: c_int) -> c_ulong {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return 0;
    }
    cpu_buffer = buffer.buffers[cpu];
    ret = local_read(&cpu_buffer.dropped_events);
    return ret;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_dropped_events_cpu);
//
// ring_buffer_read_events_cpu - get the number of events successfully read
// @buffer: The ring buffer
// @cpu: The per CPU buffer to get the number of events read
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_read_events_cpu(buffer: *mut trace_buffer, cpu: c_int) -> c_ulong {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return 0;
    }
    cpu_buffer = buffer.buffers[cpu];
    return cpu_buffer.read;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_read_events_cpu);
//
// ring_buffer_entries - get the number of entries in a buffer
// @buffer: The ring buffer
//
// Returns the total number of entries in the ring buffer
// (all CPU entries)
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_entries(buffer: *mut trace_buffer) -> c_ulong {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
pub static mut entries: c_ulong = 0;
    let mut cpu = 0;
// if you care about this being correct, lock the buffer
    for_each_buffer_cpu(buffer, cpu) {
    cpu_buffer = buffer.buffers[cpu];
    entries += rb_num_of_entries(cpu_buffer);
    }
    return entries;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_entries);
//
// ring_buffer_overruns - get the number of overruns in buffer
// @buffer: The ring buffer
//
// Returns the total number of overruns in the ring buffer
// (all CPU entries)
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_overruns(buffer: *mut trace_buffer) -> c_ulong {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
pub static mut overruns: c_ulong = 0;
    let mut cpu = 0;
// if you care about this being correct, lock the buffer
    for_each_buffer_cpu(buffer, cpu) {
    cpu_buffer = buffer.buffers[cpu];
    overruns += local_read(&cpu_buffer.overrun);
    }
    return overruns;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_overruns);
#[no_mangle]
unsafe extern "C" fn rb_read_remote_meta_page(cpu_buffer: *mut ring_buffer_per_cpu) -> bool {
    local_set(&cpu_buffer.entries, READ_ONCE(cpu_buffer.meta_page.entries));
    local_set(&cpu_buffer.overrun, READ_ONCE(cpu_buffer.meta_page.overrun));
    local_set(&cpu_buffer.pages_touched, READ_ONCE(cpu_buffer.meta_page.pages_touched));
    local_set(&cpu_buffer.pages_lost, READ_ONCE(cpu_buffer.meta_page.pages_lost));
    return rb_num_of_entries(cpu_buffer);
    }
#[no_mangle]
unsafe extern "C" fn rb_update_remote_head(cpu_buffer: *mut ring_buffer_per_cpu) {
    let mut next = core::ptr::null_mut();
    let mut orig = core::ptr::null_mut();
pub static mut retry: c_int = 3;
    orig = next = cpu_buffer.head_page;
    rb_inc_page(&next);
// Run after the writer
    while (cpu_buffer.head_page.page.time_stamp > next.page.time_stamp) {
    rb_inc_page(&next);
    rb_list_head_clear(cpu_buffer.head_page.list.prev);
    rb_inc_page(&cpu_buffer.head_page);
    rb_set_list_to_head(cpu_buffer.head_page.list.prev);
    if (cpu_buffer.head_page == orig) {
    if (WARN_ON_ONCE!(!(--retry))) {
    return;
    }
    }
    }
    orig = cpu_buffer.commit_page = cpu_buffer.head_page;
    retry = 3;
    while (cpu_buffer.commit_page.page.time_stamp < next.page.time_stamp) {
    rb_inc_page(&next);
    rb_inc_page(&cpu_buffer.commit_page);
    if (cpu_buffer.commit_page == orig) {
    if (WARN_ON_ONCE!(!(--retry))) {
    return;
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn rb_iter_reset(iter: *mut ring_buffer_iter) {
    let mut cpu_buffer = iter.cpu_buffer;
    if (cpu_buffer.remote) {
    rb_read_remote_meta_page(cpu_buffer);
    rb_update_remote_head(cpu_buffer);
    }
// Iterator usage is expected to have record disabled
    iter.head_page = cpu_buffer.reader_page;
    iter.head = cpu_buffer.reader_page.read;
    iter.next_event = iter.head;
    iter.missed_events = 0;
    iter.cache_reader_page = iter.head_page;
    iter.cache_read = cpu_buffer.read;
    iter.cache_pages_removed = cpu_buffer.pages_removed;
    if (iter.head) {
    iter.read_stamp = cpu_buffer.read_stamp;
    iter.page_stamp = cpu_buffer.reader_page.page.time_stamp;
    } else {
    iter.read_stamp = iter.head_page.page.time_stamp;
    iter.page_stamp = iter.read_stamp;
    }
    }
//
// ring_buffer_iter_reset - reset an iterator
// @iter: The iterator to reset
//
// Resets the iterator, so that it will start from the beginning
// again.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_iter_reset(iter: *mut ring_buffer_iter) {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    if (!iter) {
    return;
    }
    cpu_buffer = iter.cpu_buffer;
    raw_spin_lock_irqsave(&cpu_buffer.reader_lock, flags);
    rb_iter_reset(iter);
    raw_spin_unlock_irqrestore(&cpu_buffer.reader_lock, flags);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_iter_reset);
//
// ring_buffer_iter_empty - check if an iterator has no more to read
// @iter: The iterator to check
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_iter_empty(iter: *mut ring_buffer_iter) -> c_int {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
pub static mut reader: *mut c_void = core::ptr::null_mut();
pub static mut head_page: *mut c_void = core::ptr::null_mut();
pub static mut commit_page: *mut c_void = core::ptr::null_mut();
pub static mut curr_commit_page: *mut c_void = core::ptr::null_mut();
    let mut commit: c_uint = 0;
    let mut curr_commit_ts = 0;
    let mut commit_ts = 0;
    cpu_buffer = iter.cpu_buffer;
    reader = cpu_buffer.reader_page;
    head_page = cpu_buffer.head_page;
    commit_page = READ_ONCE(cpu_buffer.commit_page);
    commit_ts = commit_page.page.time_stamp;
//
// When the writer goes across pages, it issues a cmpxchg which
// is a mb(), which will synchronize with the rmb here.
// (see rb_tail_page_update())
//
    smp_rmb();
    commit = rb_page_size(commit_page);
// We want to make sure that the commit page doesn't change
    smp_rmb();
// Make sure commit page didn't change
    curr_commit_page = READ_ONCE(cpu_buffer.commit_page);
    curr_commit_ts = READ_ONCE(curr_commit_page.page.time_stamp);
// If the commit page changed, then there's more data
    if (curr_commit_page != commit_page ||
    curr_commit_ts != commit_ts) {
    return 0;
    }
// Still racy, as it may return a false positive, but that's OK
    return ((iter.head_page == commit_page && iter.head >= commit) ||
    (iter.head_page == reader && commit_page == head_page &&
    head_page.read == commit &&
    iter.head == rb_page_size(cpu_buffer.reader_page)));
    }
    EXPORT_SYMBOL_GPL(ring_buffer_iter_empty);
#[no_mangle]
pub unsafe extern "C" fn rb_update_read_stamp(cpu_buffer: *mut ring_buffer_per_cpu, event: *mut ring_buffer_event) {
    let mut delta = 0;
    match (event.type_len) {
    RINGBUF_TYPE_PADDING => {
    return;
    }
    RINGBUF_TYPE_TIME_EXTEND => {
    delta = rb_event_time_stamp(event);
    cpu_buffer.read_stamp += delta;
    return;
    }
    RINGBUF_TYPE_TIME_STAMP => {
    delta = rb_event_time_stamp(event);
    delta = rb_fix_abs_ts(delta, cpu_buffer.read_stamp);
    cpu_buffer.read_stamp = delta;
    return;
    }
    RINGBUF_TYPE_DATA => {
    cpu_buffer.read_stamp += event.time_delta;
    return;
    }
    _ => {
    RB_WARN_ON(cpu_buffer, 1);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rb_update_iter_read_stamp(iter: *mut ring_buffer_iter, event: *mut ring_buffer_event) {
    let mut delta = 0;
    match (event.type_len) {
    RINGBUF_TYPE_PADDING => {
    return;
    }
    RINGBUF_TYPE_TIME_EXTEND => {
    delta = rb_event_time_stamp(event);
    iter.read_stamp += delta;
    return;
    }
    RINGBUF_TYPE_TIME_STAMP => {
    delta = rb_event_time_stamp(event);
    delta = rb_fix_abs_ts(delta, iter.read_stamp);
    iter.read_stamp = delta;
    return;
    }
    RINGBUF_TYPE_DATA => {
    iter.read_stamp += event.time_delta;
    return;
    }
    _ => {
    RB_WARN_ON(iter.cpu_buffer, 1);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __rb_get_reader_page_from_remote(cpu_buffer: *mut ring_buffer_per_cpu) -> *mut c_void {
    let mut new_reader = core::ptr::null_mut();
    let mut prev_reader = core::ptr::null_mut();
    let mut prev_head = core::ptr::null_mut();
    let mut new_head = core::ptr::null_mut();
    let mut last = core::ptr::null_mut();
    if (!rb_read_remote_meta_page(cpu_buffer)) {
    return core::ptr::null_mut();
    }
// More to read on the reader page
    if (cpu_buffer.reader_page.read < rb_page_size(cpu_buffer.reader_page)) {
    if (!cpu_buffer.reader_page.read) {
    cpu_buffer.read_stamp = cpu_buffer.reader_page.page.time_stamp;
    }
    return cpu_buffer.reader_page;
    }
    prev_reader = cpu_buffer.subbuf_ids[cpu_buffer.meta_page.reader.id];
    if (cpu_buffer.remote.swap_reader_page(cpu_buffer.cpu,
    cpu_buffer.remote.priv)) {
    pr_warn_ratelimited("Remote reader page swap failed\n");
    return core::ptr::null_mut();
    }
// nr_pages doesn't include the reader page
    if (WARN_ON_ONCE!(cpu_buffer.meta_page.reader.id > cpu_buffer.nr_pages)) {
    return core::ptr::null_mut();
    }
    new_reader = cpu_buffer.subbuf_ids[cpu_buffer.meta_page.reader.id];
    WARN_ON_ONCE!(prev_reader == new_reader);
    prev_head = new_reader;  /* New reader was also the previous head */
    new_head = prev_head;
    rb_inc_page(&new_head);
    last = prev_head;
    rb_dec_page(&last);
// Clear the old HEAD flag
    rb_list_head_clear(cpu_buffer.head_page.list.prev);
    prev_reader.list.next = prev_head.list.next;
    prev_reader.list.prev = prev_head.list.prev;
// Swap prev_reader with new_reader
    last.list.next = &prev_reader.list;
    new_head.list.prev = &prev_reader.list;
    new_reader.list.prev = &new_reader.list;
    new_reader.list.next = &new_head.list;
// Reactivate the HEAD flag
    rb_set_list_to_head(&last.list);
    cpu_buffer.head_page = new_head;
    cpu_buffer.reader_page = new_reader;
    cpu_buffer.reader_page.read = 0;
    cpu_buffer.pages = &new_head.list;
    cpu_buffer.read_stamp = new_reader.page.time_stamp;
    cpu_buffer.lost_events = cpu_buffer.meta_page.reader.lost_events;
    return rb_page_size(cpu_buffer.reader_page) ? cpu_buffer.reader_page : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __rb_get_reader_page(cpu_buffer: *mut ring_buffer_per_cpu) -> *mut c_void {
pub static mut max_loops: c_int = 0;
    let mut reader = core::ptr::null_mut();
    let mut overwrite = 0;
    let mut flags = 0;
pub static mut missed_events: c_int = 0;
pub static mut nr_loops: c_int = 0;
    let mut ret = 0;
    local_irq_save(flags);
    arch_spin_lock(&cpu_buffer.lock);
// label;
//
// This should normally only loop twice. But because the
// start of the reader inserts an empty page, it causes a
// case where we will loop three times. There should be no
// reason to loop four times unless the ring buffer is a
// recovered persistent ring buffer. For persistent ring buffers,
// invalid pages are reset during recovery, so there may be more
// than 3 contiguous pages can be empty, but less than nr_pages.
//
    if (RB_WARN_ON(cpu_buffer, ++nr_loops > max_loops)) {
    reader = core::ptr::null_mut();
// goto;
    }
    reader = cpu_buffer.reader_page;
// If there's more to read, return this page
    if (cpu_buffer.reader_page.read < rb_page_size(reader)) {
// goto;
    }
// Never should we have an index greater than the size
    if (RB_WARN_ON(cpu_buffer,
    cpu_buffer.reader_page.read > rb_page_size(reader))) {
// goto;
    }
// check if we caught up to the tail
    reader = core::ptr::null_mut();
    if (cpu_buffer.commit_page == cpu_buffer.reader_page) {
// goto;
    }
// Don't bother swapping if the ring buffer is empty
    if (rb_num_of_entries(cpu_buffer) == 0) {
// goto;
    }
//
// Reset the reader page to size zero.
//
    local_set(&cpu_buffer.reader_page.write, 0);
    local_set(&cpu_buffer.reader_page.entries, 0);
    rb_init_data_page(cpu_buffer.reader_page.page);
    cpu_buffer.reader_page.real_end = 0;
// label;
//
// Splice the empty reader page into the list around the head.
//
    reader = rb_set_head_page(cpu_buffer);
    if (!reader) {
// goto;
    }
    cpu_buffer.reader_page.list.next = rb_list_head(reader.list.next);
    cpu_buffer.reader_page.list.prev = reader.list.prev;
//
// cpu_buffer->pages just needs to point to the buffer, it
// has no specific buffer page to point to. Lets move it out
// of our way so we don't accidentally swap it.
//
    cpu_buffer.pages = reader.list.prev;
// The reader page will be pointing to the new head
    rb_set_list_to_head(&cpu_buffer.reader_page.list);
//
// We want to make sure we read the overruns after we set up our
// pointers to the next object. The writer side does a
// cmpxchg to cross pages which acts as the mb on the writer
// side. Note, the reader will constantly fail the swap
// while the writer is updating the pointers, so this
// guarantees that the overwrite recorded here is the one we
// want to compare with the last_overrun.
//
    smp_mb();
    overwrite = local_read(&(cpu_buffer.overrun));
//
// Here's the tricky part.
//
// We need to move the pointer past the header page.
// But we can only do that if a writer is not currently
// moving it. The page before the header page has the
// flag bit '1' set if it is pointing to the page we want.
// but if the writer is in the process of moving it
// then it will be '2' or already moved '0'.
//
    ret = rb_head_page_replace(reader, cpu_buffer.reader_page);
//
// If we did not convert it, then we must try again.
//
    if (!ret) {
// goto;
    }
    if (rb_page_commit(reader) & RB_MISSED_EVENTS) {
    missed_events = -1;
    }
    if (cpu_buffer.ring_meta) {
    rb_update_meta_reader(cpu_buffer, reader);
    }
//
// Yay! We succeeded in replacing the page.
//
// Now make the new head point back to the reader page.
//
    rb_list_head(reader.list.next).prev = &cpu_buffer.reader_page.list;
    rb_inc_page(&cpu_buffer.head_page);
    cpu_buffer.cnt += 1;
    local_inc(&cpu_buffer.pages_read);
// Finally update the reader page to the new head
    cpu_buffer.reader_page = reader;
    cpu_buffer.reader_page.read = 0;
    if (overwrite != cpu_buffer.last_overrun) {
    cpu_buffer.lost_events = overwrite - cpu_buffer.last_overrun;
    cpu_buffer.last_overrun = overwrite;
    }
// goto;
// label;
// Update the read_stamp on the first event
    if (reader && reader.read == 0) {
    cpu_buffer.read_stamp = reader.page.time_stamp;
    }
    arch_spin_unlock(&cpu_buffer.lock);
    local_irq_restore(flags);
//
// The writer has preempt disable, wait for it. But not forever
// Although, 1 second is pretty much "forever"
//
pub const USECS_WAIT: c_int = 1000000;
    while (nr_loops < USECS_WAIT) {
// If the write is past the end of page, a writer is still updating it
    if (likely(!reader || rb_page_write(reader) <= rb_page_capacity(reader))) {
    break;
    }
    udelay(1);
// Get the latest version of the reader write value
    smp_rmb();
    }
// The writer is not moving forward? Something is wrong
    if (RB_WARN_ON(cpu_buffer, nr_loops == USECS_WAIT)) {
    reader = core::ptr::null_mut();
    }
//
// Make sure we see any padding after the write update
// (see rb_reset_tail()).
//
// In addition, a writer may be writing on the reader page
// if the page has not been fully filled, so the read barrier
// is also needed to make sure we see the content of what is
// committed by the writer (see rb_set_commit_to_write()).
//
    smp_rmb();
    if (!cpu_buffer.lost_events) {
    cpu_buffer.lost_events = missed_events;
    }
    return reader;
    }
#[no_mangle]
pub unsafe extern "C" fn rb_get_reader_page(cpu_buffer: *mut ring_buffer_per_cpu) -> *mut c_void {
    return cpu_buffer.remote ? __rb_get_reader_page_from_remote(cpu_buffer) :
    __rb_get_reader_page(cpu_buffer);
    }
#[no_mangle]
unsafe extern "C" fn rb_advance_reader(cpu_buffer: *mut ring_buffer_per_cpu) {
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut reader: *mut c_void = core::ptr::null_mut();
    let mut length: c_uint = 0;
    reader = rb_get_reader_page(cpu_buffer);
// This function should not be called when buffer is empty
    if (RB_WARN_ON(cpu_buffer, !reader)) {
    return;
    }
    event = rb_reader_event(cpu_buffer);
    if (event.type_len <= RINGBUF_TYPE_DATA_TYPE_LEN_MAX) {
    cpu_buffer.read += 1;
    }
    rb_update_read_stamp(cpu_buffer, event);
    length = rb_event_length(event);
    cpu_buffer.reader_page.read += length;
    cpu_buffer.read_bytes += length;
    }
#[no_mangle]
unsafe extern "C" fn rb_advance_iter(iter: *mut ring_buffer_iter) {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    cpu_buffer = iter.cpu_buffer;
// If head == next_event then we need to jump to the next event
    if (iter.head == iter.next_event) {
// If the event gets overwritten again, there's nothing to do
    if (rb_iter_head_event(iter) == core::ptr::null_mut()) {
    return;
    }
    }
    iter.head = iter.next_event;
//
// Check if we are at the end of the buffer.
//
    if (iter.next_event >= rb_page_size(iter.head_page)) {
// discarded commits can make the page empty
    if (iter.head_page == cpu_buffer.commit_page) {
    return;
    }
    rb_inc_iter(iter);
    return;
    }
    rb_update_iter_read_stamp(iter, iter.event);
    }
#[no_mangle]
unsafe extern "C" fn rb_lost_events(cpu_buffer: *mut ring_buffer_per_cpu) -> c_int {
    return cpu_buffer.lost_events;
    }
#[no_mangle]
pub unsafe extern "C" fn rb_buffer_peek(cpu_buffer: *mut ring_buffer_per_cpu, ts: *mut u64, lost_events: *mut c_ulong) -> *mut c_void {
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut reader: *mut c_void = core::ptr::null_mut();
pub static mut nr_loops: c_int = 0;
    if (ts) {
// ts = 0;
    }
// label;
//
// We repeat when a time extend is encountered.
// Since the time extend is always attached to a data event,
// we should never loop more than once.
// (We never hit the following condition more than twice).
//
    if (RB_WARN_ON(cpu_buffer, ++nr_loops > 2)) {
    return core::ptr::null_mut();
    }
    reader = rb_get_reader_page(cpu_buffer);
    if (!reader) {
    return core::ptr::null_mut();
    }
    event = rb_reader_event(cpu_buffer);
    match (event.type_len) {
    RINGBUF_TYPE_PADDING => {
    if (rb_null_event(event)) {
    RB_WARN_ON(cpu_buffer, 1);
    }
//
// Because the writer could be discarding every
// event it creates (which would probably be bad)
// if we were to go back to "again" then we may never
// catch up, and will trigger the warn on, or lock
// the box. Return the padding, and we will release
// the current locks, and try again.
//
    return event;
    }
    RINGBUF_TYPE_TIME_EXTEND => {
// Internal data, OK to advance
    rb_advance_reader(cpu_buffer);
// goto;
    }
    RINGBUF_TYPE_TIME_STAMP => {
    if (ts) {
// ts = rb_event_time_stamp(event);
// ts = rb_fix_abs_ts(*ts, reader->page->time_stamp);
    ring_buffer_normalize_time_stamp(cpu_buffer.buffer,
    cpu_buffer.cpu, ts);
    }
// Internal data, OK to advance
    rb_advance_reader(cpu_buffer);
// goto;
    }
    RINGBUF_TYPE_DATA => {
    if (ts && !(*ts)) {
// ts = cpu_buffer->read_stamp + event->time_delta;
    ring_buffer_normalize_time_stamp(cpu_buffer.buffer,
    cpu_buffer.cpu, ts);
    }
    if (lost_events) {
// lost_events = rb_lost_events(cpu_buffer);
    }
    return event;
    }
    _ => {
    RB_WARN_ON(cpu_buffer, 1);
    }
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(ring_buffer_peek);
#[no_mangle]
pub unsafe extern "C" fn rb_iter_peek(iter: *mut ring_buffer_iter, ts: *mut u64) -> *mut c_void {
pub static mut buffer: *mut c_void = core::ptr::null_mut();
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut nr_loops: c_int = 0;
    let mut max_loops = 0;
    if (ts) {
// ts = 0;
    }
    cpu_buffer = iter.cpu_buffer;
    buffer = cpu_buffer.buffer;
    max_loops = cpu_buffer.ring_meta ? cpu_buffer.nr_pages : 3;
//
// Check if someone performed a consuming read to the buffer
// or removed some pages from the buffer. In these cases,
// iterator was invalidated and we need to reset it.
//
    if (unlikely(iter.cache_read != cpu_buffer.read ||
    iter.cache_reader_page != cpu_buffer.reader_page ||
    iter.cache_pages_removed != cpu_buffer.pages_removed)) {
    rb_iter_reset(iter);
    }
// label;
    if (ring_buffer_iter_empty(iter)) {
    return core::ptr::null_mut();
    }
//
// As the writer can mess with what the iterator is trying
// to read, just give up if we fail to get an event after
// three tries. The iterator is not as reliable when reading
// the ring buffer with an active write as the consumer is.
// Do not warn if the three failures is reached.
//
    if (++nr_loops > max_loops) {
    return core::ptr::null_mut();
    }
    if (rb_per_cpu_empty(cpu_buffer)) {
    return core::ptr::null_mut();
    }
    if (iter.head >= rb_page_size(iter.head_page)) {
    rb_inc_iter(iter);
// goto;
    }
    event = rb_iter_head_event(iter);
    if (!event) {
// goto;
    }
    match (event.type_len) {
    RINGBUF_TYPE_PADDING => {
    if (rb_null_event(event)) {
    rb_inc_iter(iter);
// goto;
    }
    rb_advance_iter(iter);
    return event;
    }
    RINGBUF_TYPE_TIME_EXTEND => {
// Internal data, OK to advance
    rb_advance_iter(iter);
// goto;
    }
    RINGBUF_TYPE_TIME_STAMP => {
    if (ts) {
// ts = rb_event_time_stamp(event);
// ts = rb_fix_abs_ts(*ts, iter->head_page->page->time_stamp);
    ring_buffer_normalize_time_stamp(cpu_buffer.buffer,
    cpu_buffer.cpu, ts);
    }
// Internal data, OK to advance
    rb_advance_iter(iter);
// goto;
    }
    RINGBUF_TYPE_DATA => {
    if (ts && !(*ts)) {
// ts = iter->read_stamp + event->time_delta;
    ring_buffer_normalize_time_stamp(buffer,
    cpu_buffer.cpu, ts);
    }
    return event;
    }
    _ => {
    RB_WARN_ON(cpu_buffer, 1);
    }
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(ring_buffer_iter_peek);
#[no_mangle]
pub unsafe extern "C" fn rb_reader_lock(cpu_buffer: *mut ring_buffer_per_cpu) -> bool {
    if (likely(!in_nmi())) {
    raw_spin_lock(&cpu_buffer.reader_lock);
    return true;
    }
//
// If an NMI die dumps out the content of the ring buffer
// trylock must be used to prevent a deadlock if the NMI
// preempted a task that holds the ring buffer locks. If
// we get the lock then all is fine, if not, then continue
// to do the read, but this can corrupt the ring buffer,
// so it must be permanently disabled from future writes.
// Reading from NMI is a oneshot deal.
//
    if (raw_spin_trylock(&cpu_buffer.reader_lock)) {
    return true;
    }
// Continue without locking, but disable the ring buffer
    atomic_inc(&cpu_buffer.record_disabled);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn rb_reader_unlock(cpu_buffer: *mut ring_buffer_per_cpu, locked: bool) {
    if (likely(locked)) {
    raw_spin_unlock(&cpu_buffer.reader_lock);
    }
    }
//
// ring_buffer_peek - peek at the next event to be read
// @buffer: The ring buffer to read
// @cpu: The cpu to peak at
// @ts: The timestamp counter of this event.
// @lost_events: a variable to store if events were lost (may be NULL)
//
// This will return the event that will be read next, but does
// not consume the data.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_peek(buffer: *mut trace_buffer, cpu: c_int, ts: *mut u64, lost_events: *mut c_ulong) -> *mut c_void {
    let mut cpu_buffer = buffer.buffers[cpu];
pub static mut event: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut dolock = 0;
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return core::ptr::null_mut();
    }
// label;
    local_irq_save(flags);
    dolock = rb_reader_lock(cpu_buffer);
    event = rb_buffer_peek(cpu_buffer, ts, lost_events);
    if (event && event.type_len == RINGBUF_TYPE_PADDING) {
    rb_advance_reader(cpu_buffer);
    }
    rb_reader_unlock(cpu_buffer, dolock);
    local_irq_restore(flags);
    if (event && event.type_len == RINGBUF_TYPE_PADDING) {
// goto;
    }
    return event;
    }
// ring_buffer_iter_dropped - report if there are dropped events
// @iter: The ring buffer iterator
//
// Returns true if there was dropped events since the last peek.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_iter_dropped(iter: *mut ring_buffer_iter) -> bool {
    return iter.missed_events != 0;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_iter_dropped);
//
// ring_buffer_iter_peek - peek at the next event to be read
// @iter: The ring buffer iterator
// @ts: The timestamp counter of this event.
//
// This will return the event that will be read next, but does
// not increment the iterator.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_iter_peek(iter: *mut ring_buffer_iter, ts: *mut u64) -> *mut c_void {
    let mut cpu_buffer = iter.cpu_buffer;
pub static mut event: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
// label;
    raw_spin_lock_irqsave(&cpu_buffer.reader_lock, flags);
    event = rb_iter_peek(iter, ts);
    raw_spin_unlock_irqrestore(&cpu_buffer.reader_lock, flags);
    if (event && event.type_len == RINGBUF_TYPE_PADDING) {
// goto;
    }
    return event;
    }
//
// ring_buffer_consume - return an event and consume it
// @buffer: The ring buffer to get the next event from
// @cpu: the cpu to read the buffer from
// @ts: a variable to store the timestamp (may be NULL)
// @lost_events: a variable to store if events were lost (may be NULL)
//
// Returns the next event in the ring buffer, and that event is consumed.
// Meaning, that sequential reads will keep returning a different event,
// and eventually empty the ring buffer if the producer is slower.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_consume(buffer: *mut trace_buffer, cpu: c_int, ts: *mut u64, lost_events: *mut c_ulong) -> *mut c_void {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut event = core::ptr::null_mut();
    let mut flags = 0;
    let mut dolock = 0;
// label;
// might be called in atomic
    preempt_disable();
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
// goto;
    }
    cpu_buffer = buffer.buffers[cpu];
    local_irq_save(flags);
    dolock = rb_reader_lock(cpu_buffer);
    event = rb_buffer_peek(cpu_buffer, ts, lost_events);
    if (event) {
    cpu_buffer.lost_events = 0;
    rb_advance_reader(cpu_buffer);
    }
    rb_reader_unlock(cpu_buffer, dolock);
    local_irq_restore(flags);
// label;
    preempt_enable();
    if (event && event.type_len == RINGBUF_TYPE_PADDING) {
// goto;
    }
    return event;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_consume);
//
// ring_buffer_read_start - start a non consuming read of the buffer
// @buffer: The ring buffer to read from
// @cpu: The cpu buffer to iterate over
// @flags: gfp flags to use for memory allocation
//
// This creates an iterator to allow non-consuming iteration through
// the buffer. If the buffer is disabled for writing, it will produce
// the same information each time, but if the buffer is still writing
// then the first hit of a write will cause the iteration to stop.
//
// Must be paired with ring_buffer_read_finish.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_read_start(buffer: *mut trace_buffer, cpu: c_int, flags: gfp_t) -> *mut c_void {
    struct ring_buffer_iter *iter __free(kfree) = kzalloc_obj(*iter, flags);
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    if (!iter) {
    return core::ptr::null_mut();
    }
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return core::ptr::null_mut();
    }
    cpu_buffer = buffer.buffers[cpu];
//
// Only KDB is using GFP_ATOMIC, for the others, lock the buffer to
// prevent concurrent resizing.
//
    if (gfpflags_allow_blocking(flags)) {
    mutex_lock(&buffer.mutex);
    }
    atomic_inc(&cpu_buffer.resize_disabled);
    if (gfpflags_allow_blocking(flags)) {
    mutex_unlock(&buffer.mutex);
    }
// Holds the entire event: data and meta data.
    iter.event_size = rb_page_capacity(READ_ONCE(cpu_buffer.reader_page));
    iter.event = kmalloc(iter.event_size, flags);
    if (!iter.event) {
    atomic_dec(&cpu_buffer.resize_disabled);
    return core::ptr::null_mut();
    }
    iter.cpu_buffer = cpu_buffer;
    guard(raw_spinlock_irqsave)(&cpu_buffer.reader_lock);
    arch_spin_lock(&cpu_buffer.lock);
    rb_iter_reset(iter);
    arch_spin_unlock(&cpu_buffer.lock);
    return_ptr(iter);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_read_start);
//
// ring_buffer_read_finish - finish reading the iterator of the buffer
// @iter: The iterator retrieved by ring_buffer_start
//
// This re-enables resizing of the buffer, and frees the iterator.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_read_finish(iter: *mut ring_buffer_iter) {
    let mut cpu_buffer = iter.cpu_buffer;
// Use this opportunity to check the integrity of the ring buffer.
    rb_check_pages(cpu_buffer);
    atomic_dec(&cpu_buffer.resize_disabled);
    kfree(iter.event);
    kfree(iter);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_read_finish);
//
// ring_buffer_iter_advance - advance the iterator to the next location
// @iter: The ring buffer iterator
//
// Move the location of the iterator such that the next read will
// be the next location of the iterator.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_iter_advance(iter: *mut ring_buffer_iter) {
    let mut cpu_buffer = iter.cpu_buffer;
    let mut flags = 0;
    raw_spin_lock_irqsave(&cpu_buffer.reader_lock, flags);
    iter.missed_events = 0;
    rb_advance_iter(iter);
    raw_spin_unlock_irqrestore(&cpu_buffer.reader_lock, flags);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_iter_advance);
//
// ring_buffer_size - return the size of the ring buffer (in bytes)
// @buffer: The ring buffer.
// @cpu: The CPU to get ring buffer size from.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_size(buffer: *mut trace_buffer, cpu: c_int) -> c_ulong {
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return 0;
    }
    return rb_subbuf_capacity(buffer) * buffer.buffers[cpu].nr_pages;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_size);
//
// ring_buffer_max_event_size - return the max data size of an event
// @buffer: The ring buffer.
//
// Returns the maximum size an event can be.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_max_event_size(buffer: *mut trace_buffer) -> c_ulong {
// If abs timestamp is requested, events have a timestamp too
    if (ring_buffer_time_stamp_abs(buffer)) {
    return rb_subbuf_max_data_size(buffer) - RB_LEN_TIME_EXTEND;
    }
    return rb_subbuf_max_data_size(buffer);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_max_event_size);
#[no_mangle]
unsafe extern "C" fn rb_clear_buffer_page(page: *mut buffer_page) {
    local_set(&page.write, 0);
    local_set(&page.entries, 0);
    rb_init_data_page(page.page);
    page.read = 0;
    }
//
// When the buffer is memory mapped to user space, each sub buffer
// has a unique id that is used by the meta data to tell the user
// where the current reader page is.
//
// For a normal allocated ring buffer, the id is saved in the buffer page
// id field, and updated via this function.
//
// But for a fixed memory mapped buffer, the id is already assigned for
// fixed memory ordering in the memory layout and can not be used. Instead
// the index of where the page lies in the memory layout is used.
//
// For the normal pages, set the buffer page id with the passed in @id
// value and return that.
//
// For fixed memory mapped pages, get the page index in the memory layout
// and return that as the id.
//
#[no_mangle]
pub unsafe extern "C" fn rb_page_id(cpu_buffer: *mut ring_buffer_per_cpu, bpage: *mut buffer_page, id: c_int) -> c_int {
//
// For boot buffers, the id is the index,
// otherwise, set the buffer page with this id
//
    if (cpu_buffer.ring_meta) {
    id = rb_meta_subbuf_idx(cpu_buffer.ring_meta, bpage.page);
    }
    else {
    bpage.id = id;
    }
    return id;
    }
#[no_mangle]
unsafe extern "C" fn rb_update_meta_page(cpu_buffer: *mut ring_buffer_per_cpu) {
    let mut meta = cpu_buffer.meta_page;
    if (!meta) {
    return;
    }
    meta.reader.read = cpu_buffer.reader_page.read;
    meta.reader.id = rb_page_id(cpu_buffer, cpu_buffer.reader_page,
    cpu_buffer.reader_page.id);
    meta.reader.lost_events = cpu_buffer.lost_events;
    meta.entries = local_read(&cpu_buffer.entries);
    meta.overrun = local_read(&cpu_buffer.overrun);
    meta.read = cpu_buffer.read;
    meta.pages_lost = local_read(&cpu_buffer.pages_lost);
    meta.pages_touched = local_read(&cpu_buffer.pages_touched);
// Some archs do not have data cache coherency between kernel and user-space
    flush_kernel_vmap_range(cpu_buffer.meta_page, PAGE_SIZE);
    }
#[no_mangle]
pub unsafe extern "C" fn rb_reset_cpu(cpu_buffer: *mut ring_buffer_per_cpu) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (cpu_buffer.remote) {
    if (!cpu_buffer.remote.reset) {
    return;
    }
    cpu_buffer.remote.reset(cpu_buffer.cpu, cpu_buffer.remote.priv);
    rb_read_remote_meta_page(cpu_buffer);
// Read related values, not covered by the meta-page
    local_set(&cpu_buffer.pages_read, 0);
    cpu_buffer.read = 0;
    cpu_buffer.read_bytes = 0;
    cpu_buffer.last_overrun = 0;
    cpu_buffer.reader_page.read = 0;
    return;
    }
    rb_head_page_deactivate(cpu_buffer);
    cpu_buffer.head_page
    = list_entry(cpu_buffer.pages, buffer_page, list);
    rb_clear_buffer_page(cpu_buffer.head_page);
    list_for_each_entry(page, cpu_buffer.pages, list) {
    rb_clear_buffer_page(page);
    }
    cpu_buffer.tail_page = cpu_buffer.head_page;
    cpu_buffer.commit_page = cpu_buffer.head_page;
    INIT_LIST_HEAD(&cpu_buffer.reader_page.list);
    INIT_LIST_HEAD(&cpu_buffer.new_pages);
    rb_clear_buffer_page(cpu_buffer.reader_page);
    local_set(&cpu_buffer.entries_bytes, 0);
    local_set(&cpu_buffer.overrun, 0);
    local_set(&cpu_buffer.commit_overrun, 0);
    local_set(&cpu_buffer.dropped_events, 0);
    local_set(&cpu_buffer.entries, 0);
    local_set(&cpu_buffer.committing, 0);
    local_set(&cpu_buffer.commits, 0);
    local_set(&cpu_buffer.pages_touched, 0);
    local_set(&cpu_buffer.pages_lost, 0);
    local_set(&cpu_buffer.pages_read, 0);
    cpu_buffer.last_pages_touch = 0;
    cpu_buffer.shortest_full = 0;
    cpu_buffer.read = 0;
    cpu_buffer.read_bytes = 0;
    rb_time_set(&cpu_buffer.write_stamp, 0);
    rb_time_set(&cpu_buffer.before_stamp, 0);
    memset(cpu_buffer.event_stamp, 0, sizeof!(cpu_buffer.event_stamp));
    cpu_buffer.lost_events = 0;
    cpu_buffer.last_overrun = 0;
    rb_head_page_activate(cpu_buffer);
    cpu_buffer.pages_removed = 0;
    rb_update_meta_page(cpu_buffer);
    if (cpu_buffer.ring_meta) {
    let mut meta = cpu_buffer.ring_meta;
    meta.commit_buffer = meta.head_buffer;
    }
    }
// Must have disabled the cpu buffer then done a synchronize_rcu
#[no_mangle]
unsafe extern "C" fn reset_disabled_cpu_buffer(cpu_buffer: *mut ring_buffer_per_cpu) {
    guard(raw_spinlock_irqsave)(&cpu_buffer.reader_lock);
    if (RB_WARN_ON(cpu_buffer, local_read(&cpu_buffer.committing))) {
    return;
    }
    arch_spin_lock(&cpu_buffer.lock);
    rb_reset_cpu(cpu_buffer);
    arch_spin_unlock(&cpu_buffer.lock);
    }
//
// ring_buffer_reset_cpu - reset a ring buffer per CPU buffer
// @buffer: The ring buffer to reset a per cpu buffer of
// @cpu: The CPU buffer to be reset
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_reset_cpu(buffer: *mut trace_buffer, cpu: c_int) {
    let mut cpu_buffer = buffer.buffers[cpu];
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return;
    }
// prevent another thread from changing buffer sizes
    mutex_lock(&buffer.mutex);
    atomic_inc(&cpu_buffer.resize_disabled);
    atomic_inc(&cpu_buffer.record_disabled);
// Make sure all commits have finished
    synchronize_rcu();
    reset_disabled_cpu_buffer(cpu_buffer);
    atomic_dec(&cpu_buffer.record_disabled);
    atomic_dec(&cpu_buffer.resize_disabled);
    mutex_unlock(&buffer.mutex);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_reset_cpu);
// Flag to ensure proper resetting of atomic variables

//
// ring_buffer_reset_online_cpus - reset a ring buffer per CPU buffer
// @buffer: The ring buffer to reset a per cpu buffer of
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_reset_online_cpus(buffer: *mut trace_buffer) {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
// prevent another thread from changing buffer sizes
    mutex_lock(&buffer.mutex);
    for_each_online_buffer_cpu(buffer, cpu) {
    cpu_buffer = buffer.buffers[cpu];
    atomic_add(RESET_BIT, &cpu_buffer.resize_disabled);
    atomic_inc(&cpu_buffer.record_disabled);
    }
// Make sure all commits have finished
    synchronize_rcu();
    for_each_buffer_cpu(buffer, cpu) {
    cpu_buffer = buffer.buffers[cpu];
//
// If a CPU came online during the synchronize_rcu(), then
// ignore it.
//
    if (!(atomic_read(&cpu_buffer.resize_disabled) & RESET_BIT)) {
    continue;
    }
    reset_disabled_cpu_buffer(cpu_buffer);
    atomic_dec(&cpu_buffer.record_disabled);
    atomic_sub(RESET_BIT, &cpu_buffer.resize_disabled);
    }
    mutex_unlock(&buffer.mutex);
    }
//
// ring_buffer_reset - reset a ring buffer
// @buffer: The ring buffer to reset all cpu buffers
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_reset(buffer: *mut trace_buffer) {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
// prevent another thread from changing buffer sizes
    mutex_lock(&buffer.mutex);
    for_each_buffer_cpu(buffer, cpu) {
    cpu_buffer = buffer.buffers[cpu];
    atomic_inc(&cpu_buffer.resize_disabled);
    atomic_inc(&cpu_buffer.record_disabled);
    }
// Make sure all commits have finished
    synchronize_rcu();
    for_each_buffer_cpu(buffer, cpu) {
    cpu_buffer = buffer.buffers[cpu];
    reset_disabled_cpu_buffer(cpu_buffer);
    atomic_dec(&cpu_buffer.record_disabled);
    atomic_dec(&cpu_buffer.resize_disabled);
    }
    mutex_unlock(&buffer.mutex);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_reset);
//
// ring_buffer_empty - is the ring buffer empty?
// @buffer: The ring buffer to test
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_empty(buffer: *mut trace_buffer) -> bool {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut dolock = 0;
    let mut ret = 0;
    let mut cpu = 0;
// yes this is racy, but if you don't like the race, lock the buffer
    for_each_buffer_cpu(buffer, cpu) {
    cpu_buffer = buffer.buffers[cpu];
    local_irq_save(flags);
    dolock = rb_reader_lock(cpu_buffer);
    ret = rb_per_cpu_empty(cpu_buffer);
    rb_reader_unlock(cpu_buffer, dolock);
    local_irq_restore(flags);
    if (!ret) {
    return false;
    }
    }
    return true;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_empty);
//
// ring_buffer_empty_cpu - is a cpu buffer of a ring buffer empty?
// @buffer: The ring buffer
// @cpu: The CPU buffer to test
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_empty_cpu(buffer: *mut trace_buffer, cpu: c_int) -> bool {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut dolock = 0;
    let mut ret = 0;
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return true;
    }
    cpu_buffer = buffer.buffers[cpu];
    local_irq_save(flags);
    dolock = rb_reader_lock(cpu_buffer);
    ret = rb_per_cpu_empty(cpu_buffer);
    rb_reader_unlock(cpu_buffer, dolock);
    local_irq_restore(flags);
    return ret;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_empty_cpu);
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_poll_remote(buffer: *mut trace_buffer, cpu: c_int) -> c_int {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    if (cpu != RING_BUFFER_ALL_CPUS) {
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return -EINVAL;
    }
    cpu_buffer = buffer.buffers[cpu];
    guard(raw_spinlock)(&cpu_buffer.reader_lock);
    if (rb_read_remote_meta_page(cpu_buffer)) {
    rb_wakeups(buffer, cpu_buffer);
    }
    return 0;
    }
    guard(cpus_read_lock)();
//
// Make sure all the ring buffers are up to date before we start reading
// them.
//
    for_each_buffer_cpu(buffer, cpu) {
    cpu_buffer = buffer.buffers[cpu];
    guard(raw_spinlock)(&cpu_buffer.reader_lock);
    rb_read_remote_meta_page(cpu_buffer);
    }
    for_each_buffer_cpu(buffer, cpu) {
    cpu_buffer = buffer.buffers[cpu];
    if (rb_num_of_entries(cpu_buffer)) {
    rb_wakeups(buffer, cpu_buffer);
    }
    }
    return 0;
    }

//
// ring_buffer_swap_cpu - swap a CPU buffer between two ring buffers
// @buffer_a: One buffer to swap with
// @buffer_b: The other buffer to swap with
// @cpu: the CPU of the buffers to swap
//
// This function is useful for tracers that want to take a "snapshot"
// of a CPU buffer and has another back up buffer lying around.
// it is expected that the tracer handles the cpu buffer not being
// used at the moment.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_swap_cpu(buffer_a: *mut trace_buffer, buffer_b: *mut trace_buffer, cpu: c_int) -> c_int {
pub static mut cpu_buffer_a: *mut c_void = core::ptr::null_mut();
pub static mut cpu_buffer_b: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (!cpumask_test_cpu(cpu, buffer_a.cpumask) ||
    !cpumask_test_cpu(cpu, buffer_b.cpumask)) {
    return -EINVAL;
    }
    cpu_buffer_a = buffer_a.buffers[cpu];
    cpu_buffer_b = buffer_b.buffers[cpu];
// It's up to the callers to not try to swap static buffers
    if (WARN_ON_ONCE!(rb_is_static(cpu_buffer_a) || rb_is_static(cpu_buffer_b))) {
    return -EBUSY;
    }
// At least make sure the two buffers are somewhat the same
    if (cpu_buffer_a.nr_pages != cpu_buffer_b.nr_pages) {
    return -EINVAL;
    }
    if (buffer_a.subbuf_order != buffer_b.subbuf_order) {
    return -EINVAL;
    }
    if (atomic_read(&buffer_a.record_disabled)) {
    return -EAGAIN;
    }
    if (atomic_read(&buffer_b.record_disabled)) {
    return -EAGAIN;
    }
    if (atomic_read(&cpu_buffer_a.record_disabled)) {
    return -EAGAIN;
    }
    if (atomic_read(&cpu_buffer_b.record_disabled)) {
    return -EAGAIN;
    }
//
// We can't do a synchronize_rcu here because this
// function can be called in atomic context.
// Normally this will be called from the same CPU as cpu.
// If not it's up to the caller to protect this.
//
    atomic_inc(&cpu_buffer_a.record_disabled);
    atomic_inc(&cpu_buffer_b.record_disabled);
// Do not swap if either buffer is in the process of writing
    if (cpu_buffer_a.current_context) {
// goto;
    }
    if (cpu_buffer_b.current_context) {
// goto;
    }
//
// When resize is in progress, we cannot swap it because
// it will mess the state of the cpu buffer.
//
    if (atomic_read(&buffer_a.resizing)) {
// goto;
    }
    if (atomic_read(&buffer_b.resizing)) {
// goto;
    }
    buffer_a.buffers[cpu] = cpu_buffer_b;
    buffer_b.buffers[cpu] = cpu_buffer_a;
    cpu_buffer_b.buffer = buffer_a;
    cpu_buffer_a.buffer = buffer_b;
    ret = 0;
// label;
    atomic_dec(&cpu_buffer_a.record_disabled);
    atomic_dec(&cpu_buffer_b.record_disabled);
    return ret;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_swap_cpu);

//
// ring_buffer_alloc_read_page - allocate a page to read from buffer
// @buffer: the buffer to allocate for.
// @cpu: the cpu buffer to allocate.
//
// This function is used in conjunction with ring_buffer_read_page.
// When reading a full page from the ring buffer, these functions
// can be used to speed up the process. The calling function should
// allocate a few pages first with this function. Then when it
// needs to get pages from the ring buffer, it passes the result
// of this function into ring_buffer_read_page, which will swap
// the page that was allocated, with the read page of the buffer.
//
// Returns:
// The page allocated, or ERR_PTR
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_alloc_read_page(buffer: *mut trace_buffer, cpu: c_int) -> *mut c_void {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut bpage = core::ptr::null_mut();
    let mut flags = 0;
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return ERR_PTR(-ENODEV);
    }
    bpage = kzalloc_obj(*bpage);
    if (!bpage) {
    return ERR_PTR(-ENOMEM);
    }
    bpage.order = buffer.subbuf_order;
    cpu_buffer = buffer.buffers[cpu];
    local_irq_save(flags);
    arch_spin_lock(&cpu_buffer.lock);
    if (cpu_buffer.free_page.data) {
// bpage = cpu_buffer->free_page;
    cpu_buffer.free_page.data = core::ptr::null_mut();
    }
    arch_spin_unlock(&cpu_buffer.lock);
    local_irq_restore(flags);
    if (bpage.data) {
    rb_init_data_page(bpage.data);
    } else {
    bpage.data = alloc_cpu_data(cpu, bpage.order);
    if (!bpage.data) {
    kfree(bpage);
    return ERR_PTR(-ENOMEM);
    }
    }
    return bpage;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_alloc_read_page);
//
// ring_buffer_free_read_page - free an allocated read page
// @buffer: the buffer the page was allocate for
// @cpu: the cpu buffer the page came from
// @data_page: the page to free
//
// Free a page allocated from ring_buffer_alloc_read_page.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_free_read_page(buffer: *mut trace_buffer, cpu: c_int, data_page: *mut buffer_data_read_page) {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut dpage = data_page.data;
    let mut page = virt_to_page(dpage);
    let mut flags = 0;
    if (!buffer || !buffer.buffers || !buffer.buffers[cpu]) {
    return;
    }
    cpu_buffer = buffer.buffers[cpu];
//
// If the page is still in use someplace else, or order of the page
// is different from the subbuffer order of the buffer -
// we can't reuse it
//
    if (page_ref_count(page) > 1 || data_page.order != buffer.subbuf_order) {
// goto;
    }
    local_irq_save(flags);
    arch_spin_lock(&cpu_buffer.lock);
    if (!cpu_buffer.free_page.data) {
    cpu_buffer.free_page = *data_page;
    dpage = core::ptr::null_mut();
    }
    arch_spin_unlock(&cpu_buffer.lock);
    local_irq_restore(flags);
// label;
    free_pages((unsigned long)dpage, data_page.order);
    kfree(data_page);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_free_read_page);
//
// ring_buffer_read_page - extract a page from the ring buffer
// @buffer: buffer to extract from
// @data_page: the page to use allocated from ring_buffer_alloc_read_page
// @len: amount to extract
// @cpu: the cpu of the buffer to extract
// @full: should the extraction only happen when the page is full.
//
// This function will pull out a page from the ring buffer and consume it.
// @data_page must be the address of the variable that was returned
// from ring_buffer_alloc_read_page. This is because the page might be used
// to swap with a page in the ring buffer.
//
// for example:
// rpage = ring_buffer_alloc_read_page(buffer, cpu);
// if (IS_ERR(rpage))
// return PTR_ERR(rpage);
// ret = ring_buffer_read_page(buffer, rpage, len, cpu, 0);
// if (ret >= 0)
// process_page(ring_buffer_read_page_data(rpage), ret);
// ring_buffer_free_read_page(buffer, cpu, rpage);
//
// When @full is set, the function will not return true unless
// the writer is off the reader page.
//
// Note: it is up to the calling functions to handle sleeps and wakeups.
// The ring buffer can be used anywhere in the kernel and can not
// blindly call wake_up. The layer that uses the ring buffer must be
// responsible for that.
//
// Returns:
// >=0 if data has been transferred, returns the offset of consumed data.
// <0 if no data has been transferred.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_read_page(buffer: *mut trace_buffer, data_page: *mut buffer_data_read_page, len: size_t, cpu: c_int, full: c_int) -> c_int {
    let mut cpu_buffer = buffer.buffers[cpu];
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut dpage: *mut c_void = core::ptr::null_mut();
pub static mut reader: *mut c_void = core::ptr::null_mut();
    let mut missed_events = 0;
    let mut commit = 0;
    let mut size = 0;
    let mut read = 0;
    let mut save_timestamp = 0;
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return -1;
    }
//
// If len is not big enough to hold the page header, then
// we can not copy anything.
//
    if (len <= BUF_PAGE_HDR_SIZE) {
    return -1;
    }
    len -= BUF_PAGE_HDR_SIZE;
    if (!data_page || !data_page.data) {
    return -1;
    }
    dpage = data_page.data;
    if (!dpage) {
    return -1;
    }
    guard(raw_spinlock_irqsave)(&cpu_buffer.reader_lock);
    if (data_page.order != cpu_buffer.reader_page.order) {
    return -1;
    }
    reader = rb_get_reader_page(cpu_buffer);
    if (!reader) {
    return -1;
    }
    event = rb_reader_event(cpu_buffer);
    read = reader.read;
    commit = rb_page_commit(reader);
    size = rb_page_size(reader);
// Check if any events were dropped
    missed_events = cpu_buffer.lost_events;
//
// If this page has been partially read or
// if len is not big enough to read the rest of the page or
// a writer is still on the page, then
// we must copy the data from the page to the buffer.
// Otherwise, we can simply swap the page with the one passed in.
//
    if (read || (len < (size - read)) ||
    cpu_buffer.reader_page == cpu_buffer.commit_page ||
    rb_is_static(cpu_buffer)) {
    let mut rpage = cpu_buffer.reader_page.page;
pub static mut rpos: c_uint = 0;
pub static mut pos: c_uint = 0;
    let mut event_size = 0;
pub static mut flags: c_uint = 0;
//
// If a full page is expected, this can still be returned
// if there's been a previous partial read and the
// rest of the page can be read and the commit page is off
// the reader page.
//
    if (full &&
    (!read || (len < (size - read)) ||
    cpu_buffer.reader_page == cpu_buffer.commit_page)) {
    return -1;
    }
    if (len > (size - read)) {
    len = (size - read);
    }
// Always keep the time extend and data together
    event_size = rb_event_ts_length(event);
    if (len < event_size) {
    return -1;
    }
    if (commit & RB_MISSED_EVENTS) {
    flags = RB_MISSED_EVENTS;
    }
// save the current timestamp, since the user will need it
    save_timestamp = cpu_buffer.read_stamp;
// Need to copy one event at a time
    do {
// We need the size of one event, because
// rb_advance_reader only advances by one event,
// whereas rb_event_ts_length may include the size of
// one or two events.
// We have already ensured there's enough space if this
// is a time extend.
    event_size = rb_event_length(event);
    memcpy(dpage.data + pos, rpage.data + rpos, event_size);
    len -= event_size;
    rb_advance_reader(cpu_buffer);
    rpos = reader.read;
    pos += event_size;
    if (rpos >= size) {
    break;
    }
    event = rb_reader_event(cpu_buffer);
// Always keep the time extend and data together
    event_size = rb_event_ts_length(event);
    } while (len >= event_size);
// update dpage
    local_set(&dpage.commit, pos | flags);
    dpage.time_stamp = save_timestamp;
// we copied everything to the beginning
    read = 0;
    } else {
// update the entry counter
    cpu_buffer.read += rb_page_entries(reader);
    cpu_buffer.read_bytes += rb_page_size(reader);
// swap the pages
    rb_init_data_page(dpage);
    dpage = reader.page;
    reader.page = data_page.data;
    local_set(&reader.write, 0);
    local_set(&reader.entries, 0);
    reader.read = 0;
    data_page.data = dpage;
    if (!missed_events && rb_data_page_commit(dpage) & RB_MISSED_EVENTS) {
    missed_events = -1;
    }
//
// Use the real_end for the data size,
// This gives us a chance to store the lost events
// on the page.
//
    if (reader.real_end) {
    local_set(&dpage.commit, reader.real_end);
    }
    }
    cpu_buffer.lost_events = 0;
    size = rb_data_page_size(dpage);
//
// Set a flag in the commit field if we lost events
//
    if (missed_events) {
//
// If there is room at the end of the page to save the
// missed events, then record it there.
//
    if (missed_events > 0 &&
    rb_page_capacity(reader) - size >= sizeof!(missed_events)) {
    memcpy(&dpage.data[size], &missed_events,
    sizeof!(missed_events));
    local_add(RB_MISSED_STORED, &dpage.commit);
    size += sizeof!(missed_events);
    }
//
// Note, for the persistent ring buffer, the RB_MISSED_EVENTS
// may have been set in the main buffer via the verification code.
// But here, dpage is a copy of that page and has not yet had
// the RB_MISSED_EVENTS set. As for the normal buffers,
// the main write buffer does not set these bits and it needs
// to be set here.
//
    local_add(RB_MISSED_EVENTS, &dpage.commit);
    }
//
// This page may be off to user land. Zero it out here.
//
    if (size < rb_page_capacity(reader)) {
    memset(&dpage.data[size], 0, rb_page_capacity(reader) - size);
    }
    return read;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_read_page);
//
// ring_buffer_read_page_data - get pointer to the data in the page.
// @page:  the page to get the data from
//
// Returns pointer to the actual data in this page.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_read_page_data(page: *mut buffer_data_read_page) -> *mut c_void {
    return page.data;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_read_page_data);
//
// ring_buffer_subbuf_size_get - get size of the sub buffer.
// @buffer: the buffer to get the sub buffer size from
//
// Returns size of the sub buffer, in bytes.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_subbuf_size_get(buffer: *mut trace_buffer) -> c_int {
    return rb_subbuf_size(buffer);
    }
    EXPORT_SYMBOL_GPL(ring_buffer_subbuf_size_get);
//
// ring_buffer_subbuf_order_get - get order of system sub pages in one buffer page.
// @buffer: The ring_buffer to get the system sub page order from
//
// By default, one ring buffer sub page equals to one system page. This parameter
// is configurable, per ring buffer. The size of the ring buffer sub page can be
// extended, but must be an order of system page size.
//
// Returns the order of buffer sub page size, in system pages:
// 0 means the sub buffer size is 1 system page and so forth.
// In case of an error < 0 is returned.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_subbuf_order_get(buffer: *mut trace_buffer) -> c_int {
    if (!buffer) {
    return -EINVAL;
    }
    return buffer.subbuf_order;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_subbuf_order_get);
//
// ring_buffer_subbuf_order_set - set the size of ring buffer sub page.
// @buffer: The ring_buffer to set the new page size.
// @order: Order of the system pages in one sub buffer page
//
// By default, one ring buffer pages equals to one system page. This API can be
// used to set new size of the ring buffer page. The size must be order of
// system page size, that's why the input parameter @order is the order of
// system pages that are allocated for one ring buffer page:
// 0 - 1 system page
// 1 - 2 system pages
// 3 - 4 system pages
// ...
//
// Returns 0 on success or < 0 in case of an error.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_subbuf_order_set(buffer: *mut trace_buffer, order: c_int) -> c_int {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut bpage = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    let mut old_capacity = 0;
    let mut old_order = 0;
    let mut nr_pages = 0;
    let mut psize = 0;
    let mut err = 0;
    let mut cpu = 0;
    if (!buffer || order < 0) {
    return -EINVAL;
    }
    psize = (1 << order) * PAGE_SIZE;
    if (psize <= BUF_PAGE_HDR_SIZE) {
    return -EINVAL;
    }
// Size of a subbuf cannot be greater than the write counter
    if (psize > RB_WRITE_MASK + 1) {
    return -EINVAL;
    }
// prevent another thread from changing buffer sizes
    guard(mutex)(&buffer.mutex);
    old_order = buffer.subbuf_order;
    if (old_order == order) {
    return 0;
    }
    old_capacity = rb_subbuf_capacity(buffer);
    atomic_inc(&buffer.record_disabled);
// Make sure all commits have finished
    synchronize_rcu();
    buffer.subbuf_order = order;
// Make sure all new buffers are allocated, before deleting the old ones
    for_each_buffer_cpu(buffer, cpu) {
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    continue;
    }
    cpu_buffer = buffer.buffers[cpu];
    if (atomic_read(&cpu_buffer.resize_disabled)) {
    err = -EBUSY;
// goto;
    }
// Update the number of pages to match the new size
    nr_pages = old_capacity * buffer.buffers[cpu].nr_pages;
    nr_pages = DIV_ROUND_UP(nr_pages, rb_subbuf_capacity(buffer));
// we need a minimum of two pages
    if (nr_pages < 2) {
    nr_pages = 2;
    }
    cpu_buffer.nr_pages_to_update = nr_pages;
// Include the reader page
    nr_pages += 1;
// Allocate the new size buffer
    INIT_LIST_HEAD(&cpu_buffer.new_pages);
    if (__rb_allocate_pages(cpu_buffer, nr_pages,
    &cpu_buffer.new_pages)) {
// not enough memory for new pages
    err = -ENOMEM;
// goto;
    }
    }
    for_each_buffer_cpu(buffer, cpu) {
pub static mut old_free_data_page: usize = 0;
pub static mut old_pages: usize = 0;
    let mut flags = 0;
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    continue;
    }
    cpu_buffer = buffer.buffers[cpu];
    raw_spin_lock_irqsave(&cpu_buffer.reader_lock, flags);
// Clear the head bit to make the link list normal to read
    rb_head_page_deactivate(cpu_buffer);
//
// Collect buffers from the cpu_buffer pages list and the
// reader_page on old_pages, so they can be freed later when not
// under a spinlock. The pages list is a linked list with no
// head, adding old_pages turns it into a regular list with
// old_pages being the head.
//
    list_add(&old_pages, cpu_buffer.pages);
    list_add(&cpu_buffer.reader_page.list, &old_pages);
// One page was allocated for the reader page
    cpu_buffer.reader_page = list_entry(cpu_buffer.new_pages.next, buffer_page, list);
    list_del_init(&cpu_buffer.reader_page.list);
// Install the new pages, remove the head from the list
    cpu_buffer.pages = cpu_buffer.new_pages.next;
    list_del_init(&cpu_buffer.new_pages);
    cpu_buffer.cnt += 1;
    cpu_buffer.head_page
    = list_entry(cpu_buffer.pages, buffer_page, list);
    cpu_buffer.tail_page = cpu_buffer.commit_page = cpu_buffer.head_page;
    cpu_buffer.nr_pages = cpu_buffer.nr_pages_to_update;
    cpu_buffer.nr_pages_to_update = 0;
    arch_spin_lock(&cpu_buffer.lock);
    old_free_data_page = cpu_buffer.free_page;
    cpu_buffer.free_page.data = core::ptr::null_mut();
    arch_spin_unlock(&cpu_buffer.lock);
    rb_head_page_activate(cpu_buffer);
    raw_spin_unlock_irqrestore(&cpu_buffer.reader_lock, flags);
// Free old sub buffers
    list_for_each_entry_safe(bpage, tmp, &old_pages, list) {
    list_del_init(&bpage.list);
    free_buffer_page(bpage);
    }
    free_pages((unsigned long)old_free_data_page.data, old_free_data_page.order);
    rb_check_pages(cpu_buffer);
    }
    atomic_dec(&buffer.record_disabled);
    return 0;
// label;
    buffer.subbuf_order = old_order;
    atomic_dec(&buffer.record_disabled);
    for_each_buffer_cpu(buffer, cpu) {
    cpu_buffer = buffer.buffers[cpu];
    if (!cpu_buffer.nr_pages_to_update) {
    continue;
    }
    list_for_each_entry_safe(bpage, tmp, &cpu_buffer.new_pages, list) {
    list_del_init(&bpage.list);
    free_buffer_page(bpage);
    }
    }
    return err;
    }
    EXPORT_SYMBOL_GPL(ring_buffer_subbuf_order_set);
#[no_mangle]
unsafe extern "C" fn rb_alloc_meta_page(cpu_buffer: *mut ring_buffer_per_cpu) -> c_int {
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (cpu_buffer.meta_page) {
    return 0;
    }
    page = alloc_page(GFP_USER | __GFP_ZERO);
    if (!page) {
    return -ENOMEM;
    }
    cpu_buffer.meta_page = page_to_virt(page);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rb_free_meta_page(cpu_buffer: *mut ring_buffer_per_cpu) {
pub static mut addr: c_ulong = 0;
    free_page(addr);
    cpu_buffer.meta_page = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn rb_setup_ids_meta_page(cpu_buffer: *mut ring_buffer_per_cpu, subbuf_ids: *mut *mut buffer_page) {
    let mut meta = cpu_buffer.meta_page;
pub static mut nr_subbufs: c_uint = 0;
    let mut first_subbuf = core::ptr::null_mut();
    let mut subbuf = core::ptr::null_mut();
pub static mut cnt: c_int = 0;
pub static mut id: c_int = 0;
    id = rb_page_id(cpu_buffer, cpu_buffer.reader_page, id);
    subbuf_ids[id++] = cpu_buffer.reader_page;
    cnt += 1;
    first_subbuf = subbuf = rb_set_head_page(cpu_buffer);
    do {
    id = rb_page_id(cpu_buffer, subbuf, id);
    if (WARN_ON!(id >= nr_subbufs)) {
    break;
    }
    subbuf_ids[id] = subbuf;
    rb_inc_page(&subbuf);
    id += 1;
    cnt += 1;
    } while (subbuf != first_subbuf);
    WARN_ON!(cnt != nr_subbufs);
// install subbuf ID to bpage translation
    cpu_buffer.subbuf_ids = subbuf_ids;
    meta.meta_struct_len = sizeof!(*meta);
    meta.nr_subbufs = nr_subbufs;
    meta.subbuf_size = rb_subbuf_size(cpu_buffer.buffer);
    meta.meta_page_size = meta.subbuf_size;
    rb_update_meta_page(cpu_buffer);
    }
#[no_mangle]
pub unsafe extern "C" fn rb_get_mapped_buffer(buffer: *mut trace_buffer, cpu: c_int) -> *mut c_void {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return ERR_PTR(-EINVAL);
    }
    cpu_buffer = buffer.buffers[cpu];
    mutex_lock(&cpu_buffer.mapping_lock);
    if (!cpu_buffer.user_mapped) {
    mutex_unlock(&cpu_buffer.mapping_lock);
    return ERR_PTR(-ENODEV);
    }
    return cpu_buffer;
    }
#[no_mangle]
unsafe extern "C" fn rb_put_mapped_buffer(cpu_buffer: *mut ring_buffer_per_cpu) {
    mutex_unlock(&cpu_buffer.mapping_lock);
    }
//
// Fast-path for rb_buffer_(un)map(). Called whenever the meta-page doesn't need
// to be set-up or torn-down.
//
#[no_mangle]
pub unsafe extern "C" fn __rb_inc_dec_mapped(cpu_buffer: *mut ring_buffer_per_cpu, inc: bool) -> c_int {
    let mut flags = 0;
    lockdep_assert_held(&cpu_buffer.mapping_lock);
    if (inc && cpu_buffer.user_mapped == UINT_MAX) {
    return -EBUSY;
    }
    if (WARN_ON!(!inc && cpu_buffer.user_mapped == 0)) {
    return -EINVAL;
    }
    mutex_lock(&cpu_buffer.buffer.mutex);
    raw_spin_lock_irqsave(&cpu_buffer.reader_lock, flags);
    if (inc) {
    cpu_buffer.user_mapped += 1;
    }
    else {
    cpu_buffer.user_mapped -= 1;
    }
    raw_spin_unlock_irqrestore(&cpu_buffer.reader_lock, flags);
    mutex_unlock(&cpu_buffer.buffer.mutex);
    return 0;
    }
//
// +--------------+  pgoff == 0
// |   meta page  |
// +--------------+  pgoff == 1
// | subbuffer 0  |
// |              |
// +--------------+  pgoff == (1 + (1 << subbuf_order))
// | subbuffer 1  |
// |              |
// ...
//

#[no_mangle]
pub unsafe extern "C" fn __rb_map_vma(cpu_buffer: *mut ring_buffer_per_cpu, vma: *mut vm_area_struct) -> c_int {
    unsigned long nr_subbufs, nr_pages, nr_vma_pages;
pub static mut pgoff: pgoff_t = 0;
    let mut subbuf_pages = 0;
    let mut subbuf_order = 0;
    struct page **pages __free(kfree) = core::ptr::null_mut();
pub static mut p: c_int = 0;
    let mut err = 0;
// Refuse MP_PRIVATE or writable mappings
    if (vma.vm_flags & VM_WRITE || vma.vm_flags & VM_EXEC ||
    !(vma.vm_flags & VM_MAYSHARE)) {
    return -EPERM;
    }
    subbuf_order = cpu_buffer.buffer.subbuf_order;
    subbuf_pages = 1 << subbuf_order;
    if (subbuf_order && pgoff % subbuf_pages) {
    return -EINVAL;
    }
//
// Make sure the mapping cannot become writable later. Also tell the VM
// to not touch these pages (VM_DONTCOPY | VM_DONTEXPAND).
//
    vm_flags_mod(vma, VM_DONTCOPY | VM_DONTEXPAND | VM_DONTDUMP,
    VM_MAYWRITE);
    lockdep_assert_held(&cpu_buffer.mapping_lock);
    nr_subbufs = cpu_buffer.nr_pages + 1; /* + reader-subbuf */
    nr_pages = ((nr_subbufs + 1) << subbuf_order); /* + meta-page */
    if (nr_pages <= pgoff) {
    return -EINVAL;
    }
    nr_pages -= pgoff;
    nr_vma_pages = vma_pages(vma);
    if (!nr_vma_pages || nr_vma_pages > nr_pages) {
    return -EINVAL;
    }
    nr_pages = nr_vma_pages;
    pages = kzalloc_objs(*pages, nr_pages);
    if (!pages) {
    return -ENOMEM;
    }
    if (!pgoff) {
    let mut meta_page_padding = 0;
    pages[p++] = virt_to_page(cpu_buffer.meta_page);
//
// Pad with the zero-page to align the meta-page with the
// sub-buffers.
//
    meta_page_padding = subbuf_pages - 1;
    while (meta_page_padding-- && p < nr_pages) {
    unsigned long __maybe_unused zero_addr =
    vma.vm_start + (PAGE_SIZE * p);
    pages[p++] = ZERO_PAGE(zero_addr);
    }
    } else {
// Skip the meta-page
    pgoff -= subbuf_pages;
    s += pgoff / subbuf_pages;
    }
    while (p < nr_pages) {
pub static mut subbuf: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut off: c_int = 0;
    if (WARN_ON_ONCE!(s >= nr_subbufs)) {
    return -EINVAL;
    }
    subbuf = cpu_buffer.subbuf_ids[s];
    page = virt_to_page(subbuf.page);
    while (off < (1 << (subbuf_order))) {
    if (p >= nr_pages) {
    break;
    }
    pages[p++] = page;
    }
    s += 1;
    }
    err = vm_insert_pages(vma, vma.vm_start, pages, &nr_pages);
    return err;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: __rb_map_vma
pub unsafe extern "C" fn __rb_map_vma_dup(cpu_buffer: *mut ring_buffer_per_cpu, vma: *mut vm_area_struct) -> c_int {
    return -EOPNOTSUPP;
    }

#[no_mangle]
pub unsafe extern "C" fn ring_buffer_map(buffer: *mut trace_buffer, cpu: c_int, vma: *mut vm_area_struct) -> c_int {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
pub static mut subbuf_ids: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut err = 0;
    if (!cpumask_test_cpu(cpu, buffer.cpumask) || buffer.remote) {
    return -EINVAL;
    }
    cpu_buffer = buffer.buffers[cpu];
    guard(mutex)(&cpu_buffer.mapping_lock);
    if (cpu_buffer.user_mapped) {
    err = __rb_map_vma(cpu_buffer, vma);
    if (!err) {
    err = __rb_inc_dec_mapped(cpu_buffer, true);
    }
    return err;
    }
// prevent another thread from changing buffer/sub-buffer sizes
    guard(mutex)(&buffer.mutex);
    err = rb_alloc_meta_page(cpu_buffer);
    if (err) {
    return err;
    }
// subbuf_ids includes the reader while nr_pages does not
    subbuf_ids = kcalloc(cpu_buffer.nr_pages + 1, sizeof!(*subbuf_ids), GFP_KERNEL);
    if (!subbuf_ids) {
    rb_free_meta_page(cpu_buffer);
    return -ENOMEM;
    }
    atomic_inc(&cpu_buffer.resize_disabled);
//
// Lock all readers to block any subbuf swap until the subbuf IDs are
// assigned.
//
    raw_spin_lock_irqsave(&cpu_buffer.reader_lock, flags);
    rb_setup_ids_meta_page(cpu_buffer, subbuf_ids);
    raw_spin_unlock_irqrestore(&cpu_buffer.reader_lock, flags);
    err = __rb_map_vma(cpu_buffer, vma);
    if (!err) {
    raw_spin_lock_irqsave(&cpu_buffer.reader_lock, flags);
// This is the first time it is mapped by user
    cpu_buffer.user_mapped = 1;
    raw_spin_unlock_irqrestore(&cpu_buffer.reader_lock, flags);
    } else {
    kfree(cpu_buffer.subbuf_ids);
    cpu_buffer.subbuf_ids = core::ptr::null_mut();
    rb_free_meta_page(cpu_buffer);
    atomic_dec(&cpu_buffer.resize_disabled);
    }
    return err;
    }
//
// This is called when a VMA is duplicated (e.g., on fork()) to increment
// the user_mapped counter without remapping pages.
//
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_map_dup(buffer: *mut trace_buffer, cpu: c_int) {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    if (WARN_ON!(!cpumask_test_cpu(cpu, buffer.cpumask))) {
    return;
    }
    cpu_buffer = buffer.buffers[cpu];
    guard(mutex)(&cpu_buffer.mapping_lock);
    if (cpu_buffer.user_mapped) {
    __rb_inc_dec_mapped(cpu_buffer, true);
    }
    else {
    WARN(1, "Unexpected buffer stat, it should be mapped");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_unmap(buffer: *mut trace_buffer, cpu: c_int) -> c_int {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    if (!cpumask_test_cpu(cpu, buffer.cpumask)) {
    return -EINVAL;
    }
    cpu_buffer = buffer.buffers[cpu];
    guard(mutex)(&cpu_buffer.mapping_lock);
    if (!cpu_buffer.user_mapped) {
    return -ENODEV;
    } else if (cpu_buffer.user_mapped > 1) {
    __rb_inc_dec_mapped(cpu_buffer, false);
    return 0;
    }
    guard(mutex)(&buffer.mutex);
    raw_spin_lock_irqsave(&cpu_buffer.reader_lock, flags);
// This is the last user space mapping
    cpu_buffer.user_mapped = 0;
    raw_spin_unlock_irqrestore(&cpu_buffer.reader_lock, flags);
    kfree(cpu_buffer.subbuf_ids);
    cpu_buffer.subbuf_ids = core::ptr::null_mut();
    rb_free_meta_page(cpu_buffer);
    atomic_dec(&cpu_buffer.resize_disabled);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_map_get_reader(buffer: *mut trace_buffer, cpu: c_int) -> c_int {
pub static mut cpu_buffer: *mut c_void = core::ptr::null_mut();
pub static mut reader: *mut c_void = core::ptr::null_mut();
    let mut missed_events = 0;
    let mut reader_size = 0;
    let mut flags = 0;
    cpu_buffer = rb_get_mapped_buffer(buffer, cpu);
    if (IS_ERR(cpu_buffer)) {
    return (int)PTR_ERR(cpu_buffer);
    }
    raw_spin_lock_irqsave(&cpu_buffer.reader_lock, flags);
// label;
    if (rb_per_cpu_empty(cpu_buffer)) {
// goto;
    }
    reader_size = rb_page_size(cpu_buffer.reader_page);
//
// There are data to be read on the current reader page, we can
// return to the caller. But before that, we assume the latter will read
// everything. Let's update the kernel reader accordingly.
//
    if (cpu_buffer.reader_page.read < reader_size) {
    while (cpu_buffer.reader_page.read < reader_size) {
    rb_advance_reader(cpu_buffer);
    }
// goto;
    }
// Did the reader catch up with the writer?
    if (cpu_buffer.reader_page == cpu_buffer.commit_page) {
// goto;
    }
    reader = rb_get_reader_page(cpu_buffer);
    if (WARN_ON!(!reader)) {
// goto;
    }
// Check if any events were dropped
    missed_events = cpu_buffer.lost_events;
    if (missed_events) {
    if (cpu_buffer.reader_page != cpu_buffer.commit_page) {
    let mut dpage = reader.page;
    let mut commit = 0;
//
// Use the real_end for the data size,
// This gives us a chance to store the lost events
// on the page.
//
    if (reader.real_end) {
    local_set(&dpage.commit, reader.real_end);
    }
//
// If there is room at the end of the page to save the
// missed events, then record it there.
//
    commit = rb_page_size(reader);
    if (rb_page_capacity(reader) - commit >= sizeof!(missed_events)) {
    memcpy(&dpage.data[commit], &missed_events,
    sizeof!(missed_events));
    local_add(RB_MISSED_STORED, &dpage.commit);
    }
    local_add(RB_MISSED_EVENTS, &dpage.commit);
    } else if (!WARN_ONCE(cpu_buffer.reader_page == cpu_buffer.tail_page,
    "Reader on commit with %ld missed events",
    missed_events)) {
//
// There shouldn't be any missed events if the tail_page
// is on the reader page. But if the tail page is not on the
// reader page and the commit_page is, that would mean that
// there's a commit_overrun (an interrupt preempted an
// addition of an event and then filled the buffer
// with new events). In this case it's not an
// error, but it should still be reported.
//
// TODO: Add missed events to the page for user space to know.
//
    pr_info!("Ring buffer [%d] commit overrun lost %ld events at timestamp:%lld\n",
    cpu, missed_events, cpu_buffer.reader_page.page.time_stamp);
    }
    }
    cpu_buffer.lost_events = 0;
// goto;
// label;
// Some archs do not have data cache coherency between kernel and user-space
    flush_kernel_vmap_range(cpu_buffer.reader_page.page,
    rb_subbuf_size(buffer));
    rb_update_meta_page(cpu_buffer);
    raw_spin_unlock_irqrestore(&cpu_buffer.reader_lock, flags);
    rb_put_mapped_buffer(cpu_buffer);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rb_cpu_sync(data: *mut c_void) {
// Not really needed, but documents what is happening
    smp_rmb();
    }
//
// We only allocate new buffers, never free them if the CPU goes down.
// If we were to free the buffer, then the user would lose any trace that was in
// the buffer.
//
#[no_mangle]
pub unsafe extern "C" fn trace_rb_cpu_prepare(cpu: c_uint, node: *mut hlist_node) -> c_int {
pub static mut buffer: *mut c_void = core::ptr::null_mut();
    let mut nr_pages_same = 0;
    let mut cpu_i = 0;
    let mut nr_pages = 0;
    buffer = container_of!(node, trace_buffer, node);
    if (cpumask_test_cpu(cpu, buffer.cpumask)) {
    return 0;
    }
    nr_pages = 0;
    nr_pages_same = 1;
// check if all cpu sizes are same
    for_each_buffer_cpu(buffer, cpu_i) {
// fill in the size from first enabled cpu
    if (nr_pages == 0) {
    nr_pages = buffer.buffers[cpu_i].nr_pages;
    }
    if (nr_pages != buffer.buffers[cpu_i].nr_pages) {
    nr_pages_same = 0;
    break;
    }
    }
// allocate minimum pages, user can later expand it
    if (!nr_pages_same) {
    nr_pages = 2;
    }
    buffer.buffers[cpu] =
    rb_allocate_cpu_buffer(buffer, nr_pages, cpu);
    if (!buffer.buffers[cpu]) {
    WARN(1, "failed to allocate ring buffer on CPU %u\n",
    cpu);
    return -ENOMEM;
    }
//
// Ensure trace_buffer readers observe the newly allocated
// ring_buffer_per_cpu before they check the cpumask. Instead of using a
// read barrier for all readers, send an IPI.
//
    if (unlikely(system_state == SYSTEM_RUNNING)) {
    on_each_cpu(rb_cpu_sync, core::ptr::null_mut(), 1);
// Not really needed, but documents what is happening
    smp_wmb();
    }
    cpumask_set_cpu(cpu, buffer.cpumask);
    return 0;
    }

//
// This is a basic integrity check of the ring buffer.
// Late in the boot cycle this test will run when configured in.
// It will kick off a thread per CPU that will go into a loop
// writing to the per cpu ring buffer various sizes of data.
// Some of the data will be large items, some small.
//
// Another thread is created that goes into a spin, sending out
// IPIs to the other CPUs to also write into the ring buffer.
// this is to test the nesting ability of the buffer.
//
// Basic stats are recorded and reported. If something in the
// ring buffer should happen that's not expected, a big warning
// is displayed and all ring buffers are disabled.
//
    static struct task_struct *rb_threads[NR_CPUS] __initdata;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_test_data {
    pub buffer: *mut trace_buffer,
    pub events: c_ulong,
    pub bytes_written: c_ulong,
    pub bytes_alloc: c_ulong,
    pub bytes_dropped: c_ulong,
    pub events_nested: c_ulong,
    pub bytes_written_nested: c_ulong,
    pub bytes_alloc_nested: c_ulong,
    pub bytes_dropped_nested: c_ulong,
    pub min_size_nested: c_int,
    pub max_size_nested: c_int,
    pub max_size: c_int,
    pub min_size: c_int,
    pub cpu: c_int,
    pub cnt: c_int,
}

    static struct rb_test_data rb_data[NR_CPUS] __initdata;
// 1 meg per cpu
pub const RB_TEST_BUFFER_SIZE: c_int = 1048576;
    static char rb_string[] __initdata =
    "abcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()?+\\"
    "?+|:';\",.<>/?abcdefghijklmnopqrstuvwxyz1234567890"
    "!@#$%^&*()?+\\?+|:';\",.<>/?abcdefghijklmnopqrstuv";
    static bool rb_test_started __initdata;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_item {
    pub size: c_int,
    pub str: [c_char; 0],
}

#[no_mangle]
unsafe extern "C" fn rb_write_something(data: *mut rb_test_data, nested: bool) -> __init int {
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut item: *mut c_void = core::ptr::null_mut();
    let mut started = 0;
    let mut event_len = 0;
    let mut size = 0;
    let mut len = 0;
    let mut cnt = 0;
// Have nested writes different that what is written
    cnt = data.cnt + (nested ? 27 : 0);
// Multiply cnt by ~e, to make some unique increment
    size = (cnt * 68 / 25) % (sizeof!(rb_string) - 1);
    len = size + sizeof!(rb_item);
    started = rb_test_started;
// read rb_test_started before checking buffer enabled
    smp_rmb();
    event = ring_buffer_lock_reserve(data.buffer, len);
    if (!event) {
// Ignore dropped events before test starts.
    if (started) {
    if (nested) {
    data.bytes_dropped_nested += len;
    }
    else {
    data.bytes_dropped += len;
    }
    }
    return len;
    }
    event_len = ring_buffer_event_length(event);
    if (RB_WARN_ON(data.buffer, event_len < len)) {
// goto;
    }
    item = ring_buffer_event_data(event);
    item.size = size;
    memcpy(item.str, rb_string, size);
    if (nested) {
    data.bytes_alloc_nested += event_len;
    data.bytes_written_nested += len;
    data.events_nested += 1;
    if (!data.min_size_nested || len < data.min_size_nested) {
    data.min_size_nested = len;
    }
    if (len > data.max_size_nested) {
    data.max_size_nested = len;
    }
    } else {
    data.bytes_alloc += event_len;
    data.bytes_written += len;
    data.events += 1;
    if (!data.min_size || len < data.min_size) {
    data.max_size = len;
    }
    if (len > data.max_size) {
    data.max_size = len;
    }
    }
// label;
    ring_buffer_unlock_commit(data.buffer);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rb_test(arg: *mut c_void) -> __init int {
    let mut data = arg;
    while (!kthread_should_stop()) {
    rb_write_something(data, false);
    data.cnt += 1;
    set_current_state(TASK_INTERRUPTIBLE);
// Now sleep between a min of 100-300us and a max of 1ms
    usleep_range(((data.cnt % 3) + 1) * 100, 1000);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rb_ipi(ignore: *mut c_void) -> __init void {
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut cpu: c_int = 0;
    data = &rb_data[cpu];
    rb_write_something(data, true);
    }
#[no_mangle]
unsafe extern "C" fn rb_hammer_test(arg: *mut c_void) -> __init int {
    while (!kthread_should_stop()) {
// Send an IPI to all cpus to write data!
    smp_call_function(rb_ipi, core::ptr::null_mut(), 1);
// No sleep, but for non preempt, let others run
    schedule();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_ringbuffer() -> __init int {
pub static mut rb_hammer: *mut c_void = core::ptr::null_mut();
pub static mut buffer: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
pub static mut ret: c_int = 0;
    if (security_locked_down(LOCKDOWN_TRACEFS)) {
    pr_warn!("Lockdown is enabled, skipping ring buffer tests\n");
    return 0;
    }
    pr_info!("Running ring buffer tests...\n");
    buffer = ring_buffer_alloc(RB_TEST_BUFFER_SIZE, RB_FL_OVERWRITE);
    if (WARN_ON!(!buffer)) {
    return 0;
    }
// Disable buffer so that threads can't write to it yet
    ring_buffer_record_off(buffer);
    for_each_online_cpu(cpu) {
    rb_data[cpu].buffer = buffer;
    rb_data[cpu].cpu = cpu;
    rb_data[cpu].cnt = cpu;
    rb_threads[cpu] = kthread_run_on_cpu(rb_test, &rb_data[cpu],
    cpu, "rbtester/%u");
    if (WARN_ON!(IS_ERR(rb_threads[cpu]))) {
    pr_cont("FAILED\n");
    ret = PTR_ERR(rb_threads[cpu]);
// goto;
    }
    }
// Now create the rb hammer!
    rb_hammer = kthread_run(rb_hammer_test, core::ptr::null_mut(), "rbhammer");
    if (WARN_ON!(IS_ERR(rb_hammer))) {
    pr_cont("FAILED\n");
    ret = PTR_ERR(rb_hammer);
// goto;
    }
    ring_buffer_record_on(buffer);
//
// Show buffer is enabled before setting rb_test_started.
// Yes there's a small race window where events could be
// dropped and the thread won't catch it. But when a ring
// buffer gets enabled, there will always be some kind of
// delay before other CPUs see it. Thus, we don't care about
// those dropped events. We care about events dropped after
// the threads see that the buffer is active.
//
    smp_wmb();
    rb_test_started = true;
    set_current_state(TASK_INTERRUPTIBLE);
// Just run for 10 seconds
    schedule_timeout(10 * HZ);
    kthread_stop(rb_hammer);
// label;
    for_each_online_cpu(cpu) {
    if (IS_ERR_OR_NULL(rb_threads[cpu])) {
    break;
    }
    kthread_stop(rb_threads[cpu]);
    }
    if (ret) {
    ring_buffer_free(buffer);
    return ret;
    }
// Report!
    pr_info!("finished\n");
    for_each_online_cpu(cpu) {
pub static mut event: *mut c_void = core::ptr::null_mut();
    let mut data = &rb_data[cpu];
pub static mut item: *mut c_void = core::ptr::null_mut();
    let mut total_events = 0;
    let mut total_dropped = 0;
    let mut total_written = 0;
    let mut total_alloc = 0;
pub static mut total_read: c_ulong = 0;
pub static mut total_size: c_ulong = 0;
pub static mut total_len: c_ulong = 0;
pub static mut total_lost: c_ulong = 0;
    let mut lost = 0;
    let mut big_event_size = 0;
    let mut small_event_size = 0;
    ret = -1;
    total_events = data.events + data.events_nested;
    total_written = data.bytes_written + data.bytes_written_nested;
    total_alloc = data.bytes_alloc + data.bytes_alloc_nested;
    total_dropped = data.bytes_dropped + data.bytes_dropped_nested;
    big_event_size = data.max_size + data.max_size_nested;
    small_event_size = data.min_size + data.min_size_nested;
    pr_info!("CPU %d:\n", cpu);
    pr_info!("              events:    %ld\n", total_events);
    pr_info!("       dropped bytes:    %ld\n", total_dropped);
    pr_info!("       alloced bytes:    %ld\n", total_alloc);
    pr_info!("       written bytes:    %ld\n", total_written);
    pr_info!("       biggest event:    %d\n", big_event_size);
    pr_info!("      smallest event:    %d\n", small_event_size);
    if (RB_WARN_ON(buffer, total_dropped)) {
    break;
    }
    ret = 0;
    while ((event = ring_buffer_consume(buffer, cpu, core::ptr::null_mut(), &lost))) {
    total_lost += lost;
    item = ring_buffer_event_data(event);
    total_len += ring_buffer_event_length(event);
    total_size += item.size + sizeof!(rb_item);
    if (memcmp(&item.str[0], rb_string, item.size) != 0) {
    pr_info!("FAILED!\n");
    pr_info!("buffer had: %.*s\n", item.size, item.str);
    pr_info!("expected:   %.*s\n", item.size, rb_string);
    RB_WARN_ON(buffer, 1);
    ret = -1;
    break;
    }
    total_read += 1;
    }
    if (ret) {
    break;
    }
    ret = -1;
    pr_info!("         read events:   %ld\n", total_read);
    pr_info!("         lost events:   %ld\n", total_lost);
    pr_info!("        total events:   %ld\n", total_lost + total_read);
    pr_info!("  recorded len bytes:   %ld\n", total_len);
    pr_info!(" recorded size bytes:   %ld\n", total_size);
    if (total_lost) {
    pr_info!(" With dropped events, record len and size may not match\n"
    " alloced and written from above\n");
    } else {
    if (RB_WARN_ON(buffer, total_len != total_alloc ||
    total_size != total_written)) {
    break;
    }
    }
    if (RB_WARN_ON(buffer, total_lost + total_read != total_events)) {
    break;
    }
    ret = 0;
    }
    if (!ret) {
    pr_info!("Ring buffer PASSED!\n");
    }
    ring_buffer_free(buffer);
    return 0;
    }
    late_initcall!(test_ringbuffer);
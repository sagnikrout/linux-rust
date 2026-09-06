//! Automatically rewritten from C to Rust
//! Source: kernel/events/ring_buffer.c
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
// Performance events ring-buffer code:
//
// Copyright (C) 2008 Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
// Copyright (C) 2008-2011 Red Hat, Inc., Ingo Molnar
// Copyright (C) 2008-2011 Red Hat, Inc., Peter Zijlstra
// Copyright  ©  2009 Paul Mackerras, IBM Corp. <paulus@au1.ibm.com>
//

#[no_mangle]
unsafe extern "C" fn perf_output_wakeup(handle: *mut perf_output_handle) {
    atomic_set(&handle.rb.poll, EPOLLIN | EPOLLRDNORM);
    handle.event.pending_wakeup = 1;
    if (*perf_event_fasync(handle.event) && !handle.event.pending_kill) {
    handle.event.pending_kill = POLL_IN;
    }
    irq_work_queue(&handle.event.pending_irq);
    }
//
// We need to ensure a later event_id doesn't publish a head when a former
// event isn't done writing. However since we need to deal with NMIs we
// cannot fully serialize things.
//
// We only publish the head (and generate a wakeup) when the outer-most
// event completes.
//
#[no_mangle]
unsafe extern "C" fn perf_output_get_handle(handle: *mut perf_output_handle) {
    let mut rb = handle.rb;
    preempt_disable();
//
// Avoid an explicit LOAD/STORE such that architectures with memops
// can use them.
//
    (*&rb.nest)++;
    handle.wakeup = local_read(&rb.wakeup);
    }
#[no_mangle]
unsafe extern "C" fn perf_output_put_handle(handle: *mut perf_output_handle) {
    let mut rb = handle.rb;
    let mut head = 0;
    let mut nest = 0;
//
// If this isn't the outermost nesting, we don't have to update
// @rb->user_page->data_head.
//
    nest = READ_ONCE(rb.nest);
    if (nest > 1) {
    WRITE_ONCE(rb.nest, nest - 1);
// goto;
    }
// label;
//
// In order to avoid publishing a head value that goes backwards,
// we must ensure the load of @rb->head happens after we've
// incremented @rb->nest.
//
// Otherwise we can observe a @rb->head value before one published
// by an IRQ/NMI happening between the load and the increment.
//
    barrier();
    head = local_read(&rb.head);
//
// IRQ/NMI can happen here and advance @rb->head, causing our
// load above to be stale.
//
// Since the mmap() consumer (userspace) can run on a different CPU:
//
// kernel				user
//
// if (LOAD ->data_tail) {		LOAD ->data_head
// (A)		smp_rmb()	(C)
// STORE $data			LOAD $data
// smp_wmb()	(B)		smp_mb()	(D)
// STORE ->data_head		STORE ->data_tail
// }
//
// Where A pairs with D, and B pairs with C.
//
// In our case (A) is a control dependency that separates the load of
// the ->data_tail and the stores of $data. In case ->data_tail
// indicates there is no room in the buffer to store $data we do not.
//
// D needs to be a full barrier since it separates the data READ
// from the tail WRITE.
//
// For B a WMB is sufficient since it separates two WRITEs, and for C
// an RMB is sufficient since it separates two READs.
//
// See perf_output_begin().
//
    smp_wmb(); /* B, matches C */
    WRITE_ONCE(rb.user_page.data_head, head);
//
// We must publish the head before decrementing the nest count,
// otherwise an IRQ/NMI can publish a more recent head value and our
// write will (temporarily) publish a stale value.
//
    barrier();
    WRITE_ONCE(rb.nest, 0);
//
// Ensure we decrement @rb->nest before we validate the @rb->head.
// Otherwise we cannot be sure we caught the 'last' nested update.
//
    barrier();
    if (unlikely(head != local_read(&rb.head))) {
    WRITE_ONCE(rb.nest, 1);
// goto;
    }
    if (handle.wakeup != local_read(&rb.wakeup)) {
    perf_output_wakeup(handle);
    }
// label;
    preempt_enable();
    }
    static __always_inline bool
    ring_buffer_has_space(unsigned long head, unsigned long tail,
    unsigned long data_size, unsigned int size,
    bool backward)
    {
    if (!backward) {
    return CIRC_SPACE(head, tail, data_size) >= size;
    }
    else {
    return CIRC_SPACE(tail, head, data_size) >= size;
    }
    }
    static __always_inline int
    __perf_output_begin(perf_output_handle *handle, perf_sample_data *data, perf_event *event, unsigned int size,
    bool backward)
    {
pub static mut rb: *mut c_void = core::ptr::null_mut();
    unsigned long tail, offset, head;
    let mut have_lost = 0;
    let mut page_shift = 0;
    struct {
pub static mut header: usize = 0;
    let mut id = 0;
    let mut lost = 0;
    } lost_event;
    rcu_read_lock();
//
// For inherited events we send all the output towards the parent.
//
    if (event.parent) {
    event = event.parent;
    }
    rb = rcu_dereference(event.rb);
    if (unlikely(!rb)) {
// goto;
    }
    if (unlikely(rb.paused)) {
    if (rb.nr_pages) {
    local_inc(&rb.lost);
    atomic64_inc(&event.lost_samples);
    }
// goto;
    }
    handle.rb    = rb;
    handle.event = event;
    handle.flags = 0;
    have_lost = local_read(&rb.lost);
    if (unlikely(have_lost)) {
    size += sizeof!(lost_event);
    if (event.attr.sample_id_all) {
    size += event.id_header_size;
    }
    }
    perf_output_get_handle(handle);
    offset = local_read(&rb.head);
    do {
    head = offset;
    tail = READ_ONCE(rb.user_page.data_tail);
    if (!rb.overwrite) {
    if (unlikely(!ring_buffer_has_space(head, tail,
    perf_data_size(rb),
    size, backward))) {
// goto;
    }
    }
//
// The above forms a control dependency barrier separating the
// @tail load above from the data stores below. Since the @tail
// load is required to compute the branch to fail below.
//
// A, matches D; the full memory barrier userspace SHOULD issue
// after reading the data and before storing the new tail
// position.
//
// See perf_output_put_handle().
//
    if (!backward) {
    head += size;
    }
    else {
    head -= size;
    }
    } while (!local_try_cmpxchg(&rb.head, &offset, head));
    if (backward) {
    offset = head;
    head = (u64)(-head);
    }
//
// We rely on the implied barrier() by local_cmpxchg() to ensure
// none of the data stores below can be lifted up by the compiler.
//
    if (unlikely(head - local_read(&rb.wakeup) > rb.watermark)) {
    local_add(rb.watermark, &rb.wakeup);
    }
    page_shift = PAGE_SHIFT + page_order(rb);
    handle.page = (offset >> page_shift) & (rb.nr_pages - 1);
    offset &= (1UL << page_shift) - 1;
    handle.addr = rb.data_pages[handle.page] + offset;
    handle.size = (1UL << page_shift) - offset;
    if (unlikely(have_lost)) {
    lost_event.header.size = sizeof!(lost_event);
    lost_event.header.type = PERF_RECORD_LOST;
    lost_event.header.misc = 0;
    lost_event.id          = event.id;
    lost_event.lost        = local_xchg(&rb.lost, 0);
// XXX mostly redundant; @data is already fully initializes
    perf_event_header__init_id(&lost_event.header, data, event);
    perf_output_put(handle, lost_event);
    perf_event__output_id_sample(event, handle, data);
    }
    return 0;
// label;
    local_inc(&rb.lost);
    atomic64_inc(&event.lost_samples);
    perf_output_put_handle(handle);
// label;
    rcu_read_unlock();
    return -ENOSPC;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_output_begin_forward(handle: *mut perf_output_handle, data: *mut perf_sample_data, event: *mut perf_event, size: c_uint) -> c_int {
    return __perf_output_begin(handle, data, event, size, false);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_output_begin_backward(handle: *mut perf_output_handle, data: *mut perf_sample_data, event: *mut perf_event, size: c_uint) -> c_int {
    return __perf_output_begin(handle, data, event, size, true);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_output_begin(handle: *mut perf_output_handle, data: *mut perf_sample_data, event: *mut perf_event, size: c_uint) -> c_int {
    return __perf_output_begin(handle, data, event, size,
    unlikely(is_write_backward(event)));
    }
#[no_mangle]
pub unsafe extern "C" fn perf_output_copy(handle: *mut perf_output_handle, buf: *mut c_void, len: c_uint) -> c_uint {
    return __output_copy(handle, buf, len);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_output_skip(handle: *mut perf_output_handle, len: c_uint) -> c_uint {
    return __output_skip(handle, core::ptr::null_mut(), len);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_output_end(handle: *mut perf_output_handle) {
    perf_output_put_handle(handle);
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn ring_buffer_init(rb: *mut perf_buffer, watermark: c_long, flags: c_int) {
pub static mut max_size: c_long = 0;
    if (watermark) {
    rb.watermark = min(max_size, watermark);
    }
    if (!rb.watermark) {
    rb.watermark = max_size / 2;
    }
    if (flags & RING_BUFFER_WRITABLE) {
    rb.overwrite = 0;
    }
    else {
    rb.overwrite = 1;
    }
    refcount_set(&rb.refcount, 1);
    INIT_LIST_HEAD(&rb.event_list);
    spin_lock_init(&rb.event_lock);
//
// perf_output_begin() only checks rb->paused, therefore
// rb->paused must be true if we have no pages for output.
//
    if (!rb.nr_pages) {
    rb.paused = 1;
    }
    mutex_init(&rb.aux_mutex);
    rb.mmap_user = get_current_user();
    refcount_set(&rb.mmap_count, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_aux_output_flag(handle: *mut perf_output_handle, flags: u64) {
//
// OVERWRITE is determined by perf_aux_output_end() and can't
// be passed in directly.
//
    if (WARN_ON_ONCE!(flags & PERF_AUX_FLAG_OVERWRITE)) {
    return;
    }
    handle.aux_flags |= flags;
    }
    EXPORT_SYMBOL_GPL(perf_aux_output_flag);
//
// This is called before hardware starts writing to the AUX area to
// obtain an output handle and make sure there's room in the buffer.
// When the capture completes, call perf_aux_output_end() to commit
// the recorded data to the buffer.
//
// The ordering is similar to that of perf_output_{begin,end}, with
// the exception of (B), which should be taken care of by the pmu
// driver, since ordering rules will differ depending on hardware.
//
// Call this from pmu::start(); see the comment in perf_aux_output_end()
// about its use in pmu callbacks. Both can also be called from the PMI
// handler if needed.
//
#[no_mangle]
pub unsafe extern "C" fn perf_aux_output_begin(handle: *mut perf_output_handle, event: *mut perf_event) -> *mut c_void {
    let mut output_event = event;
    unsigned long aux_head, aux_tail;
pub static mut rb: *mut c_void = core::ptr::null_mut();
    let mut nest = 0;
    if (output_event.parent) {
    output_event = output_event.parent;
    }
//
// Since this will typically be open across pmu::add/pmu::del, we
// grab ring_buffer's refcount instead of holding rcu read lock
// to make sure it doesn't disappear under us.
//
    rb = ring_buffer_get(output_event);
    if (!rb) {
    return core::ptr::null_mut();
    }
    if (!rb_has_aux(rb)) {
// goto;
    }
//
// If aux_mmap_count is zero, the aux buffer is in perf_mmap_close(),
// about to get freed, so we leave immediately.
//
// Checking rb::aux_mmap_count and rb::refcount has to be done in
// the same order, see perf_mmap_close. Otherwise we end up freeing
// aux pages in this path, which is a bug, because in_atomic().
//
    if (!refcount_read(&rb.aux_mmap_count)) {
// goto;
    }
    if (!refcount_inc_not_zero(&rb.aux_refcount)) {
// goto;
    }
    nest = READ_ONCE(rb.aux_nest);
//
// Nesting is not supported for AUX area, make sure nested
// writers are caught early
//
    if (WARN_ON_ONCE!(nest)) {
// goto;
    }
    WRITE_ONCE(rb.aux_nest, nest + 1);
    aux_head = rb.aux_head;
    handle.rb = rb;
    handle.event = event;
    handle.head = aux_head;
    handle.size = 0;
    handle.aux_flags = 0;
//
// In overwrite mode, AUX data stores do not depend on aux_tail,
// therefore (A) control dependency barrier does not exist. The
// (B) <-> (C) ordering is still observed by the pmu driver.
//
    if (!rb.aux_overwrite) {
    aux_tail = READ_ONCE(rb.user_page.aux_tail);
    handle.wakeup = rb.aux_wakeup + rb.aux_watermark;
    if (aux_head - aux_tail < perf_aux_size(rb)) {
    handle.size = CIRC_SPACE(aux_head, aux_tail, perf_aux_size(rb));
    }
//
// handle->size computation depends on aux_tail load; this forms a
// control dependency barrier separating aux_tail load from aux data
// store that will be enabled on successful return
//
    if (!handle.size) { /* A, matches D */ {
    perf_event_disable_inatomic(handle.event);
    }
    perf_output_wakeup(handle);
    WRITE_ONCE(rb.aux_nest, 0);
// goto;
    }
    }
    return handle.rb.aux_priv;
// label;
// can't be last
    rb_free_aux(rb);
// label;
    ring_buffer_put(rb);
    handle.event = core::ptr::null_mut();
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(perf_aux_output_begin);
#[no_mangle]
unsafe extern "C" fn rb_need_aux_wakeup(rb: *mut perf_buffer) -> __always_inline bool {
    if (rb.aux_overwrite) {
    return false;
    }
    if (rb.aux_head - rb.aux_wakeup >= rb.aux_watermark) {
    rb.aux_wakeup = rounddown(rb.aux_head, rb.aux_watermark);
    return true;
    }
    return false;
    }
//
// Commit the data written by hardware into the ring buffer by adjusting
// aux_head and posting a PERF_RECORD_AUX into the perf buffer. It is the
// pmu driver's responsibility to observe ordering rules of the hardware,
// so that all the data is externally visible before this is called.
//
// Note: this has to be called from pmu::stop() callback, as the assumption
// of the AUX buffer management code is that after pmu::stop(), the AUX
// transaction must be stopped and therefore drop the AUX reference count.
//
#[no_mangle]
pub unsafe extern "C" fn perf_aux_output_end(handle: *mut perf_output_handle, size: c_ulong) {
pub static mut wakeup: bool = false;
    let mut rb = handle.rb;
    let mut aux_head = 0;
// in overwrite mode, driver provides aux_head via handle
    if (rb.aux_overwrite) {
    handle.aux_flags |= PERF_AUX_FLAG_OVERWRITE;
    aux_head = handle.head;
    rb.aux_head = aux_head;
    } else {
    handle.aux_flags &= ~PERF_AUX_FLAG_OVERWRITE;
    aux_head = rb.aux_head;
    rb.aux_head += size;
    }
//
// Only send RECORD_AUX if we have something useful to communicate
//
// Note: the OVERWRITE records by themselves are not considered
// useful, as they don't communicate any *new* information,
// aside from the short-lived offset, that becomes history at
// the next event sched-in and therefore isn't useful.
// The userspace that needs to copy out AUX data in overwrite
// mode should know to use user_page::aux_head for the actual
// offset. So, from now on we don't output AUX records that
// have *only* OVERWRITE flag set.
//
    if (size || (handle.aux_flags & ~(u64)PERF_AUX_FLAG_OVERWRITE)) {
    perf_event_aux_event(handle.event, aux_head, size,
    handle.aux_flags);
    }
    WRITE_ONCE(rb.user_page.aux_head, rb.aux_head);
    if (rb_need_aux_wakeup(rb)) {
    wakeup = true;
    }
    if (wakeup) {
    if (handle.aux_flags & PERF_AUX_FLAG_TRUNCATED) {
    perf_event_disable_inatomic(handle.event);
    }
    perf_output_wakeup(handle);
    }
    handle.event = core::ptr::null_mut();
    WRITE_ONCE(rb.aux_nest, 0);
// can't be last
    rb_free_aux(rb);
    ring_buffer_put(rb);
    }
    EXPORT_SYMBOL_GPL(perf_aux_output_end);
//
// Skip over a given number of bytes in the AUX buffer, due to, for example,
// hardware's alignment constraints.
//
#[no_mangle]
pub unsafe extern "C" fn perf_aux_output_skip(handle: *mut perf_output_handle, size: c_ulong) -> c_int {
    let mut rb = handle.rb;
    if (size > handle.size) {
    return -ENOSPC;
    }
    rb.aux_head += size;
    WRITE_ONCE(rb.user_page.aux_head, rb.aux_head);
    if (rb_need_aux_wakeup(rb)) {
    perf_output_wakeup(handle);
    handle.wakeup = rb.aux_wakeup + rb.aux_watermark;
    }
    handle.head = rb.aux_head;
    handle.size -= size;
    return 0;
    }
    EXPORT_SYMBOL_GPL(perf_aux_output_skip);
#[no_mangle]
pub unsafe extern "C" fn perf_get_aux(handle: *mut perf_output_handle) -> *mut c_void {
// this is only valid between perf_aux_output_begin and *_end
    if (!handle.event) {
    return core::ptr::null_mut();
    }
    return handle.rb.aux_priv;
    }
    EXPORT_SYMBOL_GPL(perf_get_aux);
//
// Copy out AUX data from an AUX handle.
//
#[no_mangle]
pub unsafe extern "C" fn perf_output_copy_aux(aux_handle: *mut perf_output_handle, handle: *mut perf_output_handle, from: c_ulong, to: c_ulong) -> c_long {
    let mut rb = aux_handle.rb;
    unsigned long tocopy, remainder, len = 0;
pub static mut addr: *mut c_void = core::ptr::null_mut();
    from &= (rb.aux_nr_pages << PAGE_SHIFT) - 1;
    to &= (rb.aux_nr_pages << PAGE_SHIFT) - 1;
    do {
    tocopy = PAGE_SIZE - offset_in_page(from);
    if (to > from) {
    tocopy = min(tocopy, to - from);
    }
    if (!tocopy) {
    break;
    }
    addr = rb.aux_pages[from >> PAGE_SHIFT];
    addr += offset_in_page(from);
    remainder = perf_output_copy(handle, addr, tocopy);
    if (remainder) {
    return -EFAULT;
    }
    len += tocopy;
    from += tocopy;
    from &= (rb.aux_nr_pages << PAGE_SHIFT) - 1;
    } while (to != from);
    return len;
    }

#[no_mangle]
pub unsafe extern "C" fn rb_alloc_aux_page(node: c_int, order: c_int) -> *mut c_void {
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (order > MAX_PAGE_ORDER) {
    order = MAX_PAGE_ORDER;
    }
    do {
    page = alloc_pages_node(node, PERF_AUX_GFP, order);
    } while (!page && order--);
    if (page && order) {
//
// Communicate the allocation size to the driver:
// if we managed to secure a high-order allocation,
// set its first page's private to this order;
// !PagePrivate(page) means it's just a normal page.
//
    split_page(page, order);
    SetPagePrivate(page);
    set_page_private(page, order);
    }
    return page;
    }
#[no_mangle]
unsafe extern "C" fn rb_free_aux_page(rb: *mut perf_buffer, idx: c_int) {
    let mut page = virt_to_page(rb.aux_pages[idx]);
    ClearPagePrivate(page);
    __free_page(page);
    }
#[no_mangle]
unsafe extern "C" fn __rb_free_aux(rb: *mut perf_buffer) {
    let mut pg = 0;
//
// Should never happen, the last reference should be dropped from
// perf_mmap_close() path, which first stops aux transactions (which
// in turn are the atomic holders of aux_refcount) and then does the
// last rb_free_aux().
//
    WARN_ON_ONCE!(in_atomic());
    if (rb.aux_priv) {
    rb.free_aux(rb.aux_priv);
    rb.free_aux = core::ptr::null_mut();
    rb.aux_priv = core::ptr::null_mut();
    }
    if (rb.aux_nr_pages) {
    for (pg = 0; pg < rb.aux_nr_pages; pg++) {
    rb_free_aux_page(rb, pg);
    }
    kfree(rb.aux_pages);
    rb.aux_nr_pages = 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rb_alloc_aux(rb: *mut perf_buffer, event: *mut perf_event, pgoff: pgoff_t, nr_pages: c_int, watermark: c_long, flags: c_int) -> c_int {
pub static mut overwrite: bool = false;
pub static mut node: c_int = 0;
    let mut use_contiguous_pages = event.pmu.capabilities & (
    PERF_PMU_CAP_AUX_NO_SG | PERF_PMU_CAP_AUX_PREFER_LARGE);
//
// Initialize max_order to 0 for page allocation. This allocates single
// pages to minimize memory fragmentation. This is overridden if the
// PMU needs or prefers contiguous pages (use_contiguous_pages = true).
//
pub static mut max_order: c_int = 0;
pub static mut ret: c_int = 0;
    if (!has_aux(event)) {
    return -EOPNOTSUPP;
    }
    if (nr_pages <= 0) {
    return -EINVAL;
    }
    if (!overwrite) {
//
// Watermark defaults to half the buffer, to aid PMU drivers
// in double buffering.
//
    if (!watermark) {
    watermark = min_t(unsigned long,
    U32_MAX,
    (unsigned long)nr_pages << (PAGE_SHIFT - 1));
    }
//
// If using contiguous pages, use aux_watermark as the basis
// for chunking to help PMU drivers honor the watermark.
//
    if (use_contiguous_pages) {
    max_order = get_order(watermark);
    }
    } else {
//
// If using contiguous pages, we need to start with the
// max_order that fits in nr_pages, not the other way around,
// hence ilog2() and not get_order.
//
    if (use_contiguous_pages) {
    max_order = ilog2(nr_pages);
    }
    watermark = 0;
    }
//
// kcalloc_node() is unable to allocate buffer if the size is larger
// than: PAGE_SIZE << MAX_PAGE_ORDER; directly bail out in this case.
//
    if (get_order((unsigned long)nr_pages * sizeof!) > MAX_PAGE_ORDER) {
    return -ENOMEM;
    }
    rb.aux_pages = kcalloc_node(nr_pages, sizeof!, GFP_KERNEL,
    node);
    if (!rb.aux_pages) {
    return -ENOMEM;
    }
    rb.free_aux = event.pmu.free_aux;
    while (rb.aux_nr_pages < nr_pages) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut last = 0;
    let mut order = 0;
    order = min(max_order, ilog2(nr_pages - rb.aux_nr_pages));
    page = rb_alloc_aux_page(node, order);
    if (!page) {
// goto;
    }
    for (last = rb.aux_nr_pages + (1 << page_private(page));
    last > rb.aux_nr_pages; rb.aux_nr_pages++) {
    rb.aux_pages[rb.aux_nr_pages] = page_address(page++);
    }
    }
//
// In overwrite mode, PMUs that don't support SG may not handle more
// than one contiguous allocation, since they rely on PMI to do double
// buffering. In this case, the entire buffer has to be one contiguous
// chunk.
//
    if ((event.pmu.capabilities & PERF_PMU_CAP_AUX_NO_SG) &&
    overwrite) {
    let mut page = virt_to_page(rb.aux_pages[0]);
    if (page_private(page) != max_order) {
// goto;
    }
    }
    rb.aux_priv = event.pmu.setup_aux(event, rb.aux_pages, nr_pages,
    overwrite);
    if (!rb.aux_priv) {
// goto;
    }
    ret = 0;
//
// aux_pages (and pmu driver's private data, aux_priv) will be
// referenced in both producer's and consumer's contexts, thus
// we keep a refcount here to make sure either of the two can
// reference them safely.
//
    refcount_set(&rb.aux_refcount, 1);
    rb.aux_overwrite = overwrite;
    rb.aux_watermark = watermark;
// label;
    if (!ret) {
    rb.aux_pgoff = pgoff;
    }
    else {
    __rb_free_aux(rb);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn rb_free_aux(rb: *mut perf_buffer) {
    if (refcount_dec_and_test(&rb.aux_refcount)) {
    __rb_free_aux(rb);
    }
    }

//
// Back perf_mmap() with regular GFP_KERNEL-0 pages.
//
#[no_mangle]
pub unsafe extern "C" fn __perf_mmap_to_page(rb: *mut perf_buffer, pgoff: c_ulong) -> *mut c_void {
    if (pgoff > rb.nr_pages) {
    return core::ptr::null_mut();
    }
    if (pgoff == 0) {
    return virt_to_page(rb.user_page);
    }
    return virt_to_page(rb.data_pages[pgoff - 1]);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_mmap_alloc_page(cpu: c_int) -> *mut c_void {
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut node = 0;
    node = (cpu == -1) ? cpu : cpu_to_node(cpu);
    page = alloc_pages_node(node, GFP_KERNEL | __GFP_ZERO, 0);
    if (!page) {
    return core::ptr::null_mut();
    }
    return page_address(page);
    }
#[no_mangle]
unsafe extern "C" fn perf_mmap_free_page(addr: *mut c_void) {
    let mut page = virt_to_page(addr);
    __free_page(page);
    }
#[no_mangle]
pub unsafe extern "C" fn rb_alloc(nr_pages: c_int, watermark: c_long, cpu: c_int, flags: c_int) -> *mut c_void {
pub static mut rb: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    let mut i = 0;
    let mut node = 0;
    size = sizeof!(perf_buffer);
    size += nr_pages * sizeof!;
    if (order_base_2(size) > PAGE_SHIFT+MAX_PAGE_ORDER) {
// goto;
    }
    node = (cpu == -1) ? cpu : cpu_to_node(cpu);
    rb = kzalloc_node(size, GFP_KERNEL, node);
    if (!rb) {
// goto;
    }
    rb.user_page = perf_mmap_alloc_page(cpu);
    if (!rb.user_page) {
// goto;
    }
    while (i < nr_pages) {
    rb.data_pages[i] = perf_mmap_alloc_page(cpu);
    if (!rb.data_pages[i]) {
// goto;
    }
    }
    rb.nr_pages = nr_pages;
    ring_buffer_init(rb, watermark, flags);
    return rb;
// label;
    for (i -= 1; i >= 0; i--) {
    perf_mmap_free_page(rb.data_pages[i]);
    }
    perf_mmap_free_page(rb.user_page);
// label;
    kfree(rb);
// label;
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn rb_free(rb: *mut perf_buffer) {
    let mut i = 0;
    perf_mmap_free_page(rb.user_page);
    for (i = 0; i < rb.nr_pages; i++) {
    perf_mmap_free_page(rb.data_pages[i]);
    }
    kfree(rb);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: __perf_mmap_to_page
pub unsafe extern "C" fn __perf_mmap_to_page_dup(rb: *mut perf_buffer, pgoff: c_ulong) -> *mut c_void {
// The '>' counts in the user page.
    if (pgoff > data_page_nr(rb)) {
    return core::ptr::null_mut();
    }
    return vmalloc_to_page(rb.user_page + pgoff * PAGE_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn rb_free_work(work: *mut work_struct) {
pub static mut rb: *mut c_void = core::ptr::null_mut();
    rb = container_of!(work, perf_buffer, work);
    vfree(rb.user_page);
    kfree(rb);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: rb_free
pub unsafe extern "C" fn rb_free_dup(rb: *mut perf_buffer) {
    schedule_work(&rb.work);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: rb_alloc
pub unsafe extern "C" fn rb_alloc_dup(nr_pages: c_int, watermark: c_long, cpu: c_int, flags: c_int) -> *mut c_void {
pub static mut rb: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
pub static mut all_buf: *mut c_void = core::ptr::null_mut();
    let mut node = 0;
    size = sizeof!(perf_buffer);
    size += sizeof!;
    node = (cpu == -1) ? cpu : cpu_to_node(cpu);
    rb = kzalloc_node(size, GFP_KERNEL, node);
    if (!rb) {
// goto;
    }
    INIT_WORK(&rb.work, rb_free_work);
    all_buf = vmalloc_user((nr_pages + 1) * PAGE_SIZE);
    if (!all_buf) {
// goto;
    }
    rb.user_page = all_buf;
    rb.data_pages[0] = all_buf + PAGE_SIZE;
    if (nr_pages) {
    rb.nr_pages = 1;
    rb.page_order = ilog2(nr_pages);
    }
    ring_buffer_init(rb, watermark, flags);
    return rb;
// label;
    kfree(rb);
// label;
    return core::ptr::null_mut();
    }

#[no_mangle]
pub unsafe extern "C" fn perf_mmap_to_page(rb: *mut perf_buffer, pgoff: c_ulong) -> *mut c_void {
    if (rb.aux_nr_pages) {
// above AUX space
    if (pgoff > rb.aux_pgoff + rb.aux_nr_pages) {
    return core::ptr::null_mut();
    }
// AUX space
    if (pgoff >= rb.aux_pgoff) {
pub static mut aux_pgoff: c_int = 0;
    return virt_to_page(rb.aux_pages[aux_pgoff]);
    }
    }
    return __perf_mmap_to_page(rb, pgoff);
    }
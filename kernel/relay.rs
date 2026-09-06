//! Automatically rewritten from C to Rust
//! Source: kernel/relay.c
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



// SPDX-License-Identifier: GPL-2.0
//
// Public API and common code for kernel->userspace relay file support.
//
// See Documentation/filesystems/relay.rst for an overview.
//
// Copyright (C) 2002-2005 - Tom Zanussi (zanussi@us.ibm.com), IBM Corp
// Copyright (C) 1999-2005 - Karim Yaghmour (karim@opersys.com)
//
// Moved to kernel/relay.c by Paul Mundt, 2006.
// November 2006 - CPU hotplug support by Mathieu Desnoyers
// (mathieu.desnoyers@polymtl.ca)
//

// list of open channels, for cpu hotplug
// static DEFINE_MUTEX(relay_channels_mutex);
// static LIST_HEAD(relay_channels);
//
// fault() vm_op implementation for relay file mapping.
//
#[no_mangle]
unsafe extern "C" fn relay_buf_fault(vmf: *mut vm_fault) -> vm_fault_t {
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut buf = vmf.vma.vm_private_data;
pub static mut pgoff: pgoff_t = 0;
    if (!buf) {
    return VM_FAULT_OOM;
    }
    page = vmalloc_to_page(buf.start + (pgoff << PAGE_SHIFT));
    if (!page) {
    return VM_FAULT_SIGBUS;
    }
    get_page(page);
    vmf.page = page;
    return 0;
    }
//
// vm_ops for relay file mappings.
//
pub static mut vm_operations_struct: usize = 0;
//
// allocate an array of pointers of struct page
//
    static struct page **relay_alloc_page_array(unsigned int n_pages)
    {
    return kvzalloc_objs(page *, n_pages);
    }
//
// free an array of pointers of struct page
//
#[no_mangle]
unsafe extern "C" fn relay_free_page_array(array: *mut page) {
    kvfree(array);
    }
//
// relay_mmap_prepare_buf: - mmap channel buffer to process address space
// @buf: the relay channel buffer
// @desc: describing what to map
//
// Returns 0 if ok, negative on error
//
// Caller should already have grabbed mmap_lock.
//
#[no_mangle]
pub unsafe extern "C" fn relay_mmap_prepare_buf(buf: *mut rchan_buf, desc: *mut vm_area_desc) -> c_int {
pub static mut length: c_ulong = 0;
    if (!buf) {
    return -EBADF;
    }
    if (length != (unsigned long)buf.chan.alloc_size) {
    return -EINVAL;
    }
    desc.vm_ops = &relay_file_mmap_ops;
    vma_desc_set_flags(desc, VMA_DONTEXPAND_BIT);
    desc.private_data = buf;
    return 0;
    }
//
// relay_alloc_buf - allocate a channel buffer
// @buf: the buffer struct
// @size: total size of the buffer
//
// Returns a pointer to the resulting buffer, %NULL if unsuccessful. The
// passed in size will get page aligned, if it isn't already.
//
#[no_mangle]
pub unsafe extern "C" fn relay_alloc_buf(buf: *mut rchan_buf, size: *mut size_t) -> *mut c_void {
pub static mut mem: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut j = 0;
    let mut n_pages = 0;
// size = PAGE_ALIGN(*size);
    n_pages = *size >> PAGE_SHIFT;
    buf.page_array = relay_alloc_page_array(n_pages);
    if (!buf.page_array) {
    return core::ptr::null_mut();
    }
    while (i < n_pages) {
    buf.page_array[i] = alloc_page(GFP_KERNEL | __GFP_ZERO);
    if (unlikely(!buf.page_array[i])) {
// goto;
    }
    set_page_private(buf.page_array[i], (unsigned long)buf);
    }
    mem = vmap(buf.page_array, n_pages, VM_MAP, PAGE_KERNEL);
    if (!mem) {
// goto;
    }
    buf.page_count = n_pages;
    return mem;
// label;
    for (j = 0; j < i; j++) {
    __free_page(buf.page_array[j]);
    }
    relay_free_page_array(buf.page_array);
    return core::ptr::null_mut();
    }
//
// relay_create_buf - allocate and initialize a channel buffer
// @chan: the relay channel
//
// Returns channel buffer if successful, %NULL otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn relay_create_buf(chan: *mut rchan) -> *mut c_void {
pub static mut buf: *mut c_void = core::ptr::null_mut();
    if (chan.n_subbufs > KMALLOC_MAX_SIZE / sizeof!(size_t)) {
    return core::ptr::null_mut();
    }
    buf = kzalloc_obj(rchan_buf);
    if (!buf) {
    return core::ptr::null_mut();
    }
    buf.padding = kmalloc_objs(size_t, chan.n_subbufs);
    if (!buf.padding) {
// goto;
    }
    buf.start = relay_alloc_buf(buf, &chan.alloc_size);
    if (!buf.start) {
// goto;
    }
    buf.chan = chan;
    kref_get(&buf.chan.kref);
    return buf;
// label;
    kfree(buf.padding);
    kfree(buf);
    return core::ptr::null_mut();
    }
//
// relay_destroy_channel - free the channel struct
// @kref: target kernel reference that contains the relay channel
//
// Should only be called from kref_put().
//
#[no_mangle]
unsafe extern "C" fn relay_destroy_channel(kref: *mut kref) {
    let mut chan = container_of!(kref, rchan, kref);
    free_percpu(chan.buf);
    kfree(chan);
    }
//
// relay_destroy_buf - destroy an rchan_buf struct and associated buffer
// @buf: the buffer struct
//
#[no_mangle]
unsafe extern "C" fn relay_destroy_buf(buf: *mut rchan_buf) {
    let mut chan = buf.chan;
    let mut i = 0;
    if (likely(buf.start)) {
    vunmap(buf.start);
    for (i = 0; i < buf.page_count; i++) {
    __free_page(buf.page_array[i]);
    }
    relay_free_page_array(buf.page_array);
    }
// per_cpu_ptr(chan->buf, buf->cpu) = NULL;
    kfree(buf.padding);
    kfree(buf);
    kref_put(&chan.kref, relay_destroy_channel);
    }
//
// relay_remove_buf - remove a channel buffer
// @kref: target kernel reference that contains the relay buffer
//
// Removes the file from the filesystem, which also frees the
// rchan_buf_struct and the channel buffer.  Should only be called from
// kref_put().
//
#[no_mangle]
unsafe extern "C" fn relay_remove_buf(kref: *mut kref) {
    let mut buf = container_of!(kref, rchan_buf, kref);
    relay_destroy_buf(buf);
    }
//
// relay_buf_empty - boolean, is the channel buffer empty?
// @buf: channel buffer
//
// Returns 1 if the buffer is empty, 0 otherwise.
//
#[no_mangle]
unsafe extern "C" fn relay_buf_empty(buf: *mut rchan_buf) -> c_int {
    return (buf.subbufs_produced - buf.subbufs_consumed) ? 0 : 1;
    }
//
// relay_buf_full - boolean, is the channel buffer full?
// @buf: channel buffer
//
// Returns 1 if the buffer is full, 0 otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn relay_buf_full(buf: *mut rchan_buf) -> c_int {
pub static mut ready: usize = 0;
    return (ready >= buf.chan.n_subbufs) ? 1 : 0;
    }
    EXPORT_SYMBOL_GPL(relay_buf_full);
//
// High-level relay kernel API and associated functions.
//
#[no_mangle]
pub unsafe extern "C" fn relay_subbuf_start(buf: *mut rchan_buf, subbuf: *mut c_void, prev_subbuf: *mut c_void) -> c_int {
pub static mut full: c_int = 0;
    if (full) {
    buf.stats.full_count += 1;
    }
    if (!buf.chan.cb.subbuf_start) {
    return !full;
    }
    return buf.chan.cb.subbuf_start(buf, subbuf,
    prev_subbuf);
    }
//
// wakeup_readers - wake up readers waiting on a channel
// @work: contains the channel buffer
//
// This is the function used to defer reader waking
//
#[no_mangle]
unsafe extern "C" fn wakeup_readers(work: *mut irq_work) {
pub static mut buf: *mut c_void = core::ptr::null_mut();
    buf = container_of!(work, rchan_buf, wakeup_work);
    wake_up_interruptible(&buf.read_wait);
    }
//
// __relay_reset - reset a channel buffer
// @buf: the channel buffer
// @init: 1 if this is a first-time initialization
//
// See relay_reset() for description of effect.
//
#[no_mangle]
unsafe extern "C" fn __relay_reset(buf: *mut rchan_buf, init: c_uint) {
    let mut i = 0;
    if (init) {
    init_waitqueue_head(&buf.read_wait);
    kref_init(&buf.kref);
    init_irq_work(&buf.wakeup_work, wakeup_readers);
    } else {
    irq_work_sync(&buf.wakeup_work);
    }
    buf.subbufs_produced = 0;
    buf.subbufs_consumed = 0;
    buf.bytes_consumed = 0;
    buf.finalized = 0;
    buf.data = buf.start;
    buf.offset = 0;
    buf.stats.full_count = 0;
    buf.stats.big_count = 0;
    for (i = 0; i < buf.chan.n_subbufs; i++) {
    buf.padding[i] = 0;
    }
    relay_subbuf_start(buf, buf.data, core::ptr::null_mut());
    }
//
// relay_reset - reset the channel
// @chan: the channel
//
// This has the effect of erasing all data from all channel buffers
// and restarting the channel in its initial state.  The buffers
// are not freed, so any mappings are still in effect.
//
// NOTE. Care should be taken that the channel isn't actually
// being used by anything when this call is made.
//
#[no_mangle]
pub unsafe extern "C" fn relay_reset(chan: *mut rchan) {
pub static mut buf: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (!chan) {
    return;
    }
    if (chan.is_global && (buf = *per_cpu_ptr(chan.buf, 0))) {
    __relay_reset(buf, 0);
    return;
    }
    mutex_lock(&relay_channels_mutex);
    for_each_possible_cpu(i) {
    if ((buf = *per_cpu_ptr(chan.buf, i)))
    __relay_reset(buf, 0);
    }
    mutex_unlock(&relay_channels_mutex);
    }
    EXPORT_SYMBOL_GPL(relay_reset);
#[no_mangle]
pub unsafe extern "C" fn relay_set_buf_dentry(buf: *mut rchan_buf, dentry: *mut dentry) {
    buf.dentry = dentry;
    d_inode(buf.dentry).i_size = buf.early_bytes;
    }
#[no_mangle]
pub unsafe extern "C" fn relay_create_buf_file(chan: *mut rchan, buf: *mut rchan_buf, cpu: c_uint) -> *mut c_void {
pub static mut dentry: *mut c_void = core::ptr::null_mut();
pub static mut tmpname: *mut c_void = core::ptr::null_mut();
    tmpname = kasprintf(GFP_KERNEL, "%s%d", chan.base_filename, cpu);
    if (!tmpname) {
    return core::ptr::null_mut();
    }
// Create file in fs
    dentry = chan.cb.create_buf_file(tmpname, chan.parent,
    S_IRUSR, buf,
    &chan.is_global);
    if (IS_ERR(dentry)) {
    dentry = core::ptr::null_mut();
    }
    kfree(tmpname);
    return dentry;
    }
//
// relay_open_buf - create a new relay channel buffer
//
// used by relay_open() and CPU hotplug.
//
#[no_mangle]
pub unsafe extern "C" fn relay_open_buf(chan: *mut rchan, cpu: c_uint) -> *mut c_void {
pub static mut buf: *mut c_void = core::ptr::null_mut();
pub static mut dentry: *mut c_void = core::ptr::null_mut();
    if (chan.is_global) {
    return *per_cpu_ptr(chan.buf, 0);
    }
    buf = relay_create_buf(chan);
    if (!buf) {
    return core::ptr::null_mut();
    }
    if (chan.has_base_filename) {
    dentry = relay_create_buf_file(chan, buf, cpu);
    if (!dentry) {
// goto;
    }
    relay_set_buf_dentry(buf, dentry);
    } else {
// Only retrieve global info, nothing more, nothing less
    dentry = chan.cb.create_buf_file(core::ptr::null_mut(), core::ptr::null_mut(),
    S_IRUSR, buf,
    &chan.is_global);
    if (IS_ERR_OR_NULL(dentry)) {
// goto;
    }
    }
    buf.cpu = cpu;
    __relay_reset(buf, 1);
    if(chan.is_global) {
// per_cpu_ptr(chan->buf, 0) = buf;
    buf.cpu = 0;
    }
    return buf;
// label;
    relay_destroy_buf(buf);
    return core::ptr::null_mut();
    }
//
// relay_close_buf - close a channel buffer
// @buf: channel buffer
//
// Marks the buffer finalized and restores the default callbacks.
// The channel buffer and channel buffer data structure are then freed
// automatically when the last reference is given up.
//
#[no_mangle]
unsafe extern "C" fn relay_close_buf(buf: *mut rchan_buf) {
    buf.finalized = 1;
    irq_work_sync(&buf.wakeup_work);
    buf.chan.cb.remove_buf_file(buf.dentry);
    kref_put(&buf.kref, relay_remove_buf);
    }
#[no_mangle]
pub unsafe extern "C" fn relay_prepare_cpu(cpu: c_uint) -> c_int {
pub static mut chan: *mut c_void = core::ptr::null_mut();
pub static mut buf: *mut c_void = core::ptr::null_mut();
    mutex_lock(&relay_channels_mutex);
    list_for_each_entry(chan, &relay_channels, list) {
    if (*per_cpu_ptr(chan.buf, cpu)) {
    continue;
    }
    buf = relay_open_buf(chan, cpu);
    if (!buf) {
    pr_err!("relay: cpu %d buffer creation failed\n", cpu);
    mutex_unlock(&relay_channels_mutex);
    return -ENOMEM;
    }
// per_cpu_ptr(chan->buf, cpu) = buf;
    }
    mutex_unlock(&relay_channels_mutex);
    return 0;
    }
//
// relay_open - create a new relay channel
// @base_filename: base name of files to create
// @parent: dentry of parent directory, %NULL for root directory or buffer
// @subbuf_size: size of sub-buffers
// @n_subbufs: number of sub-buffers
// @cb: client callback functions
// @private_data: user-defined data
//
// Returns channel pointer if successful, %NULL otherwise.
//
// Creates a channel buffer for each cpu using the sizes and
// attributes specified.  The created channel buffer files
// will be named base_filename0...base_filenameN-1.  File
// permissions will be %S_IRUSR.
//
#[no_mangle]
pub unsafe extern "C" fn relay_open(base_filename: *mut c_char, parent: *mut dentry, subbuf_size: size_t, n_subbufs: size_t, cb: *mut rchan_callbacks, private_data: *mut c_void) -> *mut c_void {
    let mut i = 0;
pub static mut chan: *mut c_void = core::ptr::null_mut();
pub static mut buf: *mut c_void = core::ptr::null_mut();
    if (!(subbuf_size && n_subbufs)) {
    return core::ptr::null_mut();
    }
    if (subbuf_size > UINT_MAX / n_subbufs) {
    return core::ptr::null_mut();
    }
    if (!cb || !cb.create_buf_file || !cb.remove_buf_file) {
    return core::ptr::null_mut();
    }
    chan = kzalloc_obj(rchan);
    if (!chan) {
    return core::ptr::null_mut();
    }
    chan.buf = alloc_percpu;
    if (!chan.buf) {
    kfree(chan);
    return core::ptr::null_mut();
    }
    chan.version = RELAYFS_CHANNEL_VERSION;
    chan.n_subbufs = n_subbufs;
    chan.subbuf_size = subbuf_size;
    chan.alloc_size = PAGE_ALIGN(subbuf_size * n_subbufs);
    chan.parent = parent;
    chan.private_data = private_data;
    if (base_filename) {
    chan.has_base_filename = 1;
    strscpy(chan.base_filename, base_filename, NAME_MAX);
    }
    chan.cb = cb;
    kref_init(&chan.kref);
    mutex_lock(&relay_channels_mutex);
    for_each_online_cpu(i) {
    buf = relay_open_buf(chan, i);
    if (!buf) {
// goto;
    }
// per_cpu_ptr(chan->buf, i) = buf;
    }
    list_add(&chan.list, &relay_channels);
    mutex_unlock(&relay_channels_mutex);
    return chan;
// label;
    for_each_possible_cpu(i) {
    if ((buf = *per_cpu_ptr(chan.buf, i))) {
    relay_close_buf(buf);
    }
    }
    kref_put(&chan.kref, relay_destroy_channel);
    mutex_unlock(&relay_channels_mutex);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(relay_open);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rchan_percpu_buf_dispatcher {
    pub buf: *mut rchan_buf,
    pub dentry: *mut dentry,
}

//
// relay_switch_subbuf - switch to a new sub-buffer
// @buf: channel buffer
// @length: size of current event
//
// Returns either the length passed in or 0 if full.
//
// Performs sub-buffer-switch tasks such as invoking callbacks,
// updating padding counts, waking up readers, etc.
//
#[no_mangle]
pub unsafe extern "C" fn relay_switch_subbuf(buf: *mut rchan_buf, length: usize) -> usize {
    let mut old = core::ptr::null_mut();
    let mut new = core::ptr::null_mut();
    size_t old_subbuf, new_subbuf;
    if (unlikely(length > buf.chan.subbuf_size)) {
// goto;
    }
    if (buf.offset != buf.chan.subbuf_size + 1) {
    let mut prev_padding = 0;
    prev_padding = buf.chan.subbuf_size - buf.offset;
    old_subbuf = buf.subbufs_produced % buf.chan.n_subbufs;
    buf.padding[old_subbuf] = prev_padding;
    buf.subbufs_produced += 1;
    if (buf.dentry) {
    d_inode(buf.dentry).i_size +=
    buf.chan.subbuf_size -
    buf.padding[old_subbuf];
    }
    else {
    buf.early_bytes += buf.chan.subbuf_size -
    buf.padding[old_subbuf];
    }
    smp_mb();
    if (waitqueue_active(&buf.read_wait)) {
//
// Calling wake_up_interruptible() from here
// will deadlock if we happen to be logging
// from the scheduler (trying to re-grab
// rq->lock), so defer it.
//
    irq_work_queue(&buf.wakeup_work);
    }
    }
    old = buf.data;
    new_subbuf = buf.subbufs_produced % buf.chan.n_subbufs;
    new = buf.start + new_subbuf * buf.chan.subbuf_size;
    buf.offset = 0;
    if (!relay_subbuf_start(buf, new, old)) {
    buf.offset = buf.chan.subbuf_size + 1;
    return 0;
    }
    buf.data = new;
    buf.padding[new_subbuf] = 0;
    if (unlikely(length + buf.offset > buf.chan.subbuf_size)) {
// goto;
    }
    return length;
// label;
    buf.stats.big_count += 1;
    return 0;
    }
    EXPORT_SYMBOL_GPL(relay_switch_subbuf);
//
// relay_subbufs_consumed - update the buffer's sub-buffers-consumed count
// @chan: the channel
// @cpu: the cpu associated with the channel buffer to update
// @subbufs_consumed: number of sub-buffers to add to current buf's count
//
// Adds to the channel buffer's consumed sub-buffer count.
// subbufs_consumed should be the number of sub-buffers newly consumed,
// not the total consumed.
//
// NOTE. Kernel clients don't need to call this function if the channel
// mode is 'overwrite'.
//
#[no_mangle]
pub unsafe extern "C" fn relay_subbufs_consumed(chan: *mut rchan, cpu: c_uint, subbufs_consumed: size_t) {
pub static mut buf: *mut c_void = core::ptr::null_mut();
    if (!chan || cpu >= NR_CPUS) {
    return;
    }
    buf = *per_cpu_ptr(chan.buf, cpu);
    if (!buf || subbufs_consumed > chan.n_subbufs) {
    return;
    }
    if (subbufs_consumed > buf.subbufs_produced - buf.subbufs_consumed) {
    buf.subbufs_consumed = buf.subbufs_produced;
    }
    else {
    buf.subbufs_consumed += subbufs_consumed;
    }
    }
    EXPORT_SYMBOL_GPL(relay_subbufs_consumed);
//
// relay_close - close the channel
// @chan: the channel
//
// Closes all channel buffers and frees the channel.
//
#[no_mangle]
pub unsafe extern "C" fn relay_close(chan: *mut rchan) {
pub static mut buf: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (!chan) {
    return;
    }
    mutex_lock(&relay_channels_mutex);
    if (chan.is_global && (buf = *per_cpu_ptr(chan.buf, 0))) {
    relay_close_buf(buf);
    }
    else {
    for_each_possible_cpu(i)
    if ((buf = *per_cpu_ptr(chan.buf, i)))
    relay_close_buf(buf);
    }
    list_del(&chan.list);
    kref_put(&chan.kref, relay_destroy_channel);
    mutex_unlock(&relay_channels_mutex);
    }
    EXPORT_SYMBOL_GPL(relay_close);
//
// relay_flush - close the channel
// @chan: the channel
//
// Flushes all channel buffers, i.e. forces buffer switch.
//
#[no_mangle]
pub unsafe extern "C" fn relay_flush(chan: *mut rchan) {
pub static mut buf: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    if (!chan) {
    return;
    }
    if (chan.is_global && (buf = *per_cpu_ptr(chan.buf, 0))) {
    relay_switch_subbuf(buf, 0);
    return;
    }
    mutex_lock(&relay_channels_mutex);
    for_each_possible_cpu(i) {
    if ((buf = *per_cpu_ptr(chan.buf, i)))
    relay_switch_subbuf(buf, 0);
    }
    mutex_unlock(&relay_channels_mutex);
    }
    EXPORT_SYMBOL_GPL(relay_flush);
//
// relay_stats - get channel buffer statistics
// @chan: the channel
// @flags: select particular information to get
//
// Returns the count of certain field that caller specifies.
//
#[no_mangle]
pub unsafe extern "C" fn relay_stats(chan: *mut rchan, flags: c_int) -> usize {
    unsigned int i, count = 0;
pub static mut rbuf: *mut c_void = core::ptr::null_mut();
    if (!chan || flags > RELAY_STATS_LAST) {
    return 0;
    }
    if (chan.is_global) {
    rbuf = *per_cpu_ptr(chan.buf, 0);
    if (flags & RELAY_STATS_BUF_FULL) {
    count = rbuf.stats.full_count;
    }

    else if (flags & RELAY_STATS_WRT_BIG) {
    count = rbuf.stats.big_count;
    }
    } else {
    for_each_online_cpu(i) {
    rbuf = *per_cpu_ptr(chan.buf, i);
    if (rbuf) {
    if (flags & RELAY_STATS_BUF_FULL) {
    count += rbuf.stats.full_count;
    }

    else if (flags & RELAY_STATS_WRT_BIG) {
    count += rbuf.stats.big_count;
    }
    }
    }
    }
    return count;
    }
//
// relay_file_open - open file op for relay files
// @inode: the inode
// @filp: the file
//
// Increments the channel buffer refcount.
//
#[no_mangle]
unsafe extern "C" fn relay_file_open(inode: *mut inode, filp: *mut file) -> c_int {
    let mut buf = inode.i_private;
    kref_get(&buf.kref);
    filp.private_data = buf;
    return nonseekable_open(inode, filp);
    }
//
// relay_file_mmap_prepare - mmap file op for relay files
// @desc: describing what to map
//
// Calls upon relay_mmap_prepare_buf() to map the file into user space.
//
#[no_mangle]
unsafe extern "C" fn relay_file_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    let mut buf = desc.file.private_data;
    return relay_mmap_prepare_buf(buf, desc);
    }
//
// relay_file_poll - poll file op for relay files
// @filp: the file
// @wait: poll table
//
// Poll implemention.
//
#[no_mangle]
unsafe extern "C" fn relay_file_poll(filp: *mut file, wait: *mut poll_table) -> __poll_t {
pub static mut mask: __poll_t = 0;
    let mut buf = filp.private_data;
    if (buf.finalized) {
    return EPOLLERR;
    }
    if (filp.f_mode & FMODE_READ) {
    poll_wait(filp, &buf.read_wait, wait);
    if (!relay_buf_empty(buf)) {
    mask |= EPOLLIN | EPOLLRDNORM;
    }
    }
    return mask;
    }
//
// relay_file_release - release file op for relay files
// @inode: the inode
// @filp: the file
//
// Decrements the channel refcount, as the filesystem is
// no longer using it.
//
#[no_mangle]
unsafe extern "C" fn relay_file_release(inode: *mut inode, filp: *mut file) -> c_int {
    let mut buf = filp.private_data;
    kref_put(&buf.kref, relay_remove_buf);
    return 0;
    }
//
// relay_file_read_consume - update the consumed count for the buffer
//
#[no_mangle]
pub unsafe extern "C" fn relay_file_read_consume(buf: *mut rchan_buf, read_pos: size_t, bytes_consumed: size_t) {
pub static mut subbuf_size: usize = 0;
pub static mut n_subbufs: usize = 0;
    let mut read_subbuf = 0;
    if (buf.subbufs_produced == buf.subbufs_consumed &&
    buf.offset == buf.bytes_consumed) {
    return;
    }
    if (buf.bytes_consumed + bytes_consumed > subbuf_size) {
    relay_subbufs_consumed(buf.chan, buf.cpu, 1);
    buf.bytes_consumed = 0;
    }
    buf.bytes_consumed += bytes_consumed;
    if (!read_pos) {
    read_subbuf = buf.subbufs_consumed % n_subbufs;
    }
    else {
    read_subbuf = read_pos / buf.chan.subbuf_size;
    }
    if (buf.bytes_consumed + buf.padding[read_subbuf] == subbuf_size) {
    if ((read_subbuf == buf.subbufs_produced % n_subbufs) &&
    (buf.offset == subbuf_size)) {
    return;
    }
    relay_subbufs_consumed(buf.chan, buf.cpu, 1);
    buf.bytes_consumed = 0;
    }
    }
//
// relay_file_read_avail - boolean, are there unconsumed bytes available?
//
#[no_mangle]
unsafe extern "C" fn relay_file_read_avail(buf: *mut rchan_buf) -> c_int {
pub static mut subbuf_size: usize = 0;
pub static mut n_subbufs: usize = 0;
pub static mut produced: usize = 0;
    let mut consumed = 0;
    relay_file_read_consume(buf, 0, 0);
    consumed = buf.subbufs_consumed;
    if (unlikely(buf.offset > subbuf_size)) {
    if (produced == consumed) {
    return 0;
    }
    return 1;
    }
    if (unlikely(produced - consumed >= n_subbufs)) {
    consumed = produced - n_subbufs + 1;
    buf.subbufs_consumed = consumed;
    buf.bytes_consumed = 0;
    }
    produced = (produced % n_subbufs) * subbuf_size + buf.offset;
    consumed = (consumed % n_subbufs) * subbuf_size + buf.bytes_consumed;
    if (consumed > produced) {
    produced += n_subbufs * subbuf_size;
    }
    if (consumed == produced) {
    if (buf.offset == subbuf_size &&
    buf.subbufs_produced > buf.subbufs_consumed) {
    return 1;
    }
    return 0;
    }
    return 1;
    }
//
// relay_file_read_subbuf_avail - return bytes available in sub-buffer
// @read_pos: file read position
// @buf: relay channel buffer
//
#[no_mangle]
pub unsafe extern "C" fn relay_file_read_subbuf_avail(read_pos: size_t, buf: *mut rchan_buf) -> size_t {
    size_t padding, avail = 0;
    size_t read_subbuf, read_offset, write_subbuf, write_offset;
pub static mut subbuf_size: usize = 0;
    write_subbuf = (buf.data - buf.start) / subbuf_size;
    write_offset = buf.offset > subbuf_size ? subbuf_size : buf.offset;
    read_subbuf = read_pos / subbuf_size;
    read_offset = read_pos % subbuf_size;
    padding = buf.padding[read_subbuf];
    if (read_subbuf == write_subbuf) {
    if (read_offset + padding < write_offset) {
    avail = write_offset - (read_offset + padding);
    }
    } else {
    avail = (subbuf_size - padding) - read_offset;
    }
    return avail;
    }
//
// relay_file_read_start_pos - find the first available byte to read
// @buf: relay channel buffer
//
// If the read_pos is in the middle of padding, return the
// position of the first actually available byte, otherwise
// return the original value.
//
#[no_mangle]
unsafe extern "C" fn relay_file_read_start_pos(buf: *mut rchan_buf) -> usize {
    size_t read_subbuf, padding, padding_start, padding_end;
pub static mut subbuf_size: usize = 0;
pub static mut n_subbufs: usize = 0;
pub static mut consumed: usize = 0;
    size_t read_pos = (consumed * subbuf_size + buf.bytes_consumed)
    % (n_subbufs * subbuf_size);
    read_subbuf = read_pos / subbuf_size;
    padding = buf.padding[read_subbuf];
    padding_start = (read_subbuf + 1) * subbuf_size - padding;
    padding_end = (read_subbuf + 1) * subbuf_size;
    if (read_pos >= padding_start && read_pos < padding_end) {
    read_subbuf = (read_subbuf + 1) % n_subbufs;
    read_pos = read_subbuf * subbuf_size;
    }
    return read_pos;
    }
//
// relay_file_read_end_pos - return the new read position
// @read_pos: file read position
// @buf: relay channel buffer
// @count: number of bytes to be read
//
#[no_mangle]
pub unsafe extern "C" fn relay_file_read_end_pos(buf: *mut rchan_buf, read_pos: size_t, count: size_t) -> size_t {
    size_t read_subbuf, padding, end_pos;
pub static mut subbuf_size: usize = 0;
pub static mut n_subbufs: usize = 0;
    read_subbuf = read_pos / subbuf_size;
    padding = buf.padding[read_subbuf];
    if (read_pos % subbuf_size + count + padding == subbuf_size) {
    end_pos = (read_subbuf + 1) * subbuf_size;
    }
    else {
    end_pos = read_pos + count;
    }
    if (end_pos >= subbuf_size * n_subbufs) {
    end_pos = 0;
    }
    return end_pos;
    }
#[no_mangle]
pub unsafe extern "C" fn relay_file_read(filp: *mut file, buffer: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut buf = filp.private_data;
    size_t read_start, avail;
pub static mut written: usize = 0;
    let mut ret = 0;
    if (!count) {
    return 0;
    }
    inode_lock(file_inode(filp));
    do {
pub static mut from: *mut c_void = core::ptr::null_mut();
    if (!relay_file_read_avail(buf)) {
    break;
    }
    read_start = relay_file_read_start_pos(buf);
    avail = relay_file_read_subbuf_avail(read_start, buf);
    if (!avail) {
    break;
    }
    avail = min(count, avail);
    from = buf.start + read_start;
    ret = avail;
    if (copy_to_user(buffer, from, avail)) {
    break;
    }
    buffer += ret;
    written += ret;
    count -= ret;
    relay_file_read_consume(buf, read_start, ret);
// ppos = relay_file_read_end_pos(buf, read_start, ret);
    } while (count);
    inode_unlock(file_inode(filp));
    return written;
    }
pub static mut file_operations: usize = 0;
    EXPORT_SYMBOL_GPL(relay_file_operations);
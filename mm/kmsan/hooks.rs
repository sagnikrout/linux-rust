//! Automatically rewritten from C to Rust
//! Source: mm/kmsan/hooks.c
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
// KMSAN hooks for kernel subsystems.
//
// These functions handle creation of KMSAN metadata for memory allocations.
//
// Copyright (C) 2018-2022 Google LLC
// Author: Alexander Potapenko <glider@google.com>
//

//
// Instrumented functions shouldn't be called under
// kmsan_enter_runtime()/kmsan_leave_runtime(), because this will lead to
// skipping effects of functions like memset() inside instrumented code.
//
#[no_mangle]
pub unsafe extern "C" fn kmsan_task_create(task: *mut task_struct) {
    kmsan_enter_runtime();
    kmsan_internal_task_create(task);
    kmsan_leave_runtime();
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_task_exit(task: *mut task_struct) {
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return;
    }
    kmsan_disable_current();
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_slab_alloc(s: *mut kmem_cache, object: *mut c_void, flags: gfp_t) {
    if (unlikely(object == core::ptr::null_mut())) {
    return;
    }
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return;
    }
//
// There's a ctor or this is an RCU cache - do nothing. The memory
// status hasn't changed since last use.
//
    if (s.ctor || (s.flags & SLAB_TYPESAFE_BY_RCU)) {
    return;
    }
    kmsan_enter_runtime();
    if (flags & __GFP_ZERO) {
    kmsan_internal_unpoison_memory(object, s.object_size,
    KMSAN_POISON_CHECK);
    }
    else {
    kmsan_internal_poison_memory(object, s.object_size, flags,
    KMSAN_POISON_CHECK);
    }
    kmsan_leave_runtime();
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_slab_free(s: *mut kmem_cache, object: *mut c_void) {
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return;
    }
// RCU slabs could be legally used after free within the RCU period
    if (unlikely(s.flags & SLAB_TYPESAFE_BY_RCU)) {
    return;
    }
//
// If there's a constructor, freed memory must remain in the same state
// until the next allocation. We cannot save its state to detect
// use-after-free bugs, instead we just keep it unpoisoned.
//
    if (s.ctor) {
    return;
    }
    kmsan_enter_runtime();
    kmsan_internal_poison_memory(object, s.object_size,
    GFP_KERNEL & ~(__GFP_RECLAIM),
    KMSAN_POISON_CHECK | KMSAN_POISON_FREE);
    kmsan_leave_runtime();
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_kmalloc_large(ptr: *const c_void, size: usize, flags: gfp_t) {
    if (unlikely(ptr == core::ptr::null_mut())) {
    return;
    }
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return;
    }
    kmsan_enter_runtime();
    if (flags & __GFP_ZERO) {
    kmsan_internal_unpoison_memory(ptr, size,
// checked*/ true);
    }
    else {
    kmsan_internal_poison_memory(ptr, size, flags,
    KMSAN_POISON_CHECK);
    }
    kmsan_leave_runtime();
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_kfree_large(ptr: *const c_void) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return;
    }
    kmsan_enter_runtime();
    page = virt_to_head_page(ptr);
    KMSAN_WARN_ON(ptr != page_address(page));
    kmsan_internal_poison_memory(ptr, page_size(page),
    GFP_KERNEL & ~(__GFP_RECLAIM),
    KMSAN_POISON_CHECK | KMSAN_POISON_FREE);
    kmsan_leave_runtime();
    }
#[no_mangle]
unsafe extern "C" fn vmalloc_shadow(addr: c_ulong) -> c_ulong {
    return (unsigned long)kmsan_get_metadata(addr,
    KMSAN_META_SHADOW);
    }
#[no_mangle]
unsafe extern "C" fn vmalloc_origin(addr: c_ulong) -> c_ulong {
    return (unsigned long)kmsan_get_metadata(addr,
    KMSAN_META_ORIGIN);
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_vunmap_range_noflush(start: c_ulong, end: c_ulong) {
    __vunmap_range_noflush(vmalloc_shadow(start), vmalloc_shadow(end));
    __vunmap_range_noflush(vmalloc_origin(start), vmalloc_origin(end));
    flush_cache_vmap(vmalloc_shadow(start), vmalloc_shadow(end));
    flush_cache_vmap(vmalloc_origin(start), vmalloc_origin(end));
    }
//
// This function creates new shadow/origin pages for the physical pages mapped
// into the virtual memory. If those physical pages already had shadow/origin,
// those are ignored.
//
#[no_mangle]
pub unsafe extern "C" fn kmsan_ioremap_page_range(start: c_ulong, end: c_ulong, phys_addr: phys_addr_t, prot: pgprot_t, page_shift: c_uint) -> c_int {
pub static mut gfp_mask: gfp_t = 0;
    let mut shadow = core::ptr::null_mut();
    let mut origin = core::ptr::null_mut();
pub static mut off: c_ulong = 0;
    int nr, err = 0, clean = 0, mapped;
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return 0;
    }
    nr = (end - start) / PAGE_SIZE;
    kmsan_enter_runtime();
    while (i < nr) {
    shadow = alloc_pages(gfp_mask, 1);
    origin = alloc_pages(gfp_mask, 1);
    if (!shadow || !origin) {
    err = -ENOMEM;
// goto;
    }
    mapped = __vmap_pages_range_noflush(
    vmalloc_shadow(start + off),
    vmalloc_shadow(start + off + PAGE_SIZE), prot, &shadow,
    PAGE_SHIFT);
    if (mapped) {
    err = mapped;
// goto;
    }
    shadow = core::ptr::null_mut();
    mapped = __vmap_pages_range_noflush(
    vmalloc_origin(start + off),
    vmalloc_origin(start + off + PAGE_SIZE), prot, &origin,
    PAGE_SHIFT);
    if (mapped) {
    __vunmap_range_noflush(
    vmalloc_shadow(start + off),
    vmalloc_shadow(start + off + PAGE_SIZE));
    err = mapped;
// goto;
    }
    origin = core::ptr::null_mut();
    }
// Page mapping loop finished normally, nothing to clean up.
    clean = 0;
// label;
    if (clean > 0) {
//
// Something went wrong. Clean up shadow/origin pages allocated
// on the last loop iteration, then delete mappings created
// during the previous iterations.
//
    if (shadow) {
    __free_pages(shadow, 1);
    }
    if (origin) {
    __free_pages(origin, 1);
    }
    __vunmap_range_noflush(
    vmalloc_shadow(start),
    vmalloc_shadow(start + clean * PAGE_SIZE));
    __vunmap_range_noflush(
    vmalloc_origin(start),
    vmalloc_origin(start + clean * PAGE_SIZE));
    }
    flush_cache_vmap(vmalloc_shadow(start), vmalloc_shadow(end));
    flush_cache_vmap(vmalloc_origin(start), vmalloc_origin(end));
    kmsan_leave_runtime();
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_iounmap_page_range(start: c_ulong, end: c_ulong) {
    unsigned long v_shadow, v_origin;
    let mut shadow = core::ptr::null_mut();
    let mut origin = core::ptr::null_mut();
    let mut nr = 0;
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return;
    }
    nr = (end - start) / PAGE_SIZE;
    kmsan_enter_runtime();
    v_shadow = (unsigned long)vmalloc_shadow(start);
    v_origin = (unsigned long)vmalloc_origin(start);
    while (i < nr) {
    shadow = kmsan_vmalloc_to_page_or_null(v_shadow);
    origin = kmsan_vmalloc_to_page_or_null(v_origin);
    __vunmap_range_noflush(v_shadow, vmalloc_shadow(end));
    __vunmap_range_noflush(v_origin, vmalloc_origin(end));
    if (shadow) {
    __free_pages(shadow, 1);
    }
    if (origin) {
    __free_pages(origin, 1);
    }
    }
    flush_cache_vmap(vmalloc_shadow(start), vmalloc_shadow(end));
    flush_cache_vmap(vmalloc_origin(start), vmalloc_origin(end));
    kmsan_leave_runtime();
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_copy_to_user(to: *mut c_void, from: *mut c_void, to_copy: size_t, left: size_t) {
    let mut ua_flags = 0;
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return;
    }
//
// At this point we've copied the memory already. It's hard to check it
// before copying, as the size of actually copied buffer is unknown.
//
// copy_to_user() may copy zero bytes. No need to check.
    if (!to_copy) {
    return;
    }
// Or maybe copy_to_user() failed to copy anything.
    if (to_copy <= left) {
    return;
    }
    ua_flags = user_access_save();
    if (!IS_ENABLED!(CONFIG_ARCH_HAS_NON_OVERLAPPING_ADDRESS_SPACE) ||
    (u64)to < TASK_SIZE) {
// This is a user memory access, check it.
    kmsan_internal_check_memory(from, to_copy - left, to,
    REASON_COPY_TO_USER);
    } else {
// Otherwise this is a kernel memory access. This happens when a
// compat syscall passes an argument allocated on the kernel
// stack to a real syscall.
// Don't check anything, just copy the shadow of the copied
// bytes.
//
    kmsan_enter_runtime();
    kmsan_internal_memmove_metadata(to, from,
    to_copy - left);
    kmsan_leave_runtime();
    }
    user_access_restore(ua_flags);
    }
    EXPORT_SYMBOL(kmsan_copy_to_user);
#[no_mangle]
pub unsafe extern "C" fn kmsan_memmove(to: *mut c_void, from: *const c_void, size: usize) {
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return;
    }
    kmsan_enter_runtime();
    kmsan_internal_memmove_metadata(to, from, size);
    kmsan_leave_runtime();
    }
    EXPORT_SYMBOL(kmsan_memmove);
// Helper function to check an URB.
#[no_mangle]
pub unsafe extern "C" fn kmsan_handle_urb(urb: *const urb, is_out: bool) {
    if (!urb) {
    return;
    }
    if (is_out) {
    kmsan_internal_check_memory(urb.transfer_buffer,
    urb.transfer_buffer_length,
// user_addr*/ NULL,
    REASON_SUBMIT_URB);
    }
    else {
    kmsan_internal_unpoison_memory(urb.transfer_buffer,
    urb.transfer_buffer_length,
// checked*/ false);
    }
    }
    EXPORT_SYMBOL_GPL(kmsan_handle_urb);
#[no_mangle]
pub unsafe extern "C" fn kmsan_handle_dma_page(addr: *mut c_void, size: size_t, dir: dma_data_direction) {
    match (dir) {
    DMA_BIDIRECTIONAL => {
    kmsan_internal_check_memory(addr, size,
// user_addr*/ NULL, REASON_ANY);
    kmsan_internal_unpoison_memory(addr, size,
// checked*/ false);
    // break;
    }
    DMA_TO_DEVICE => {
    kmsan_internal_check_memory(addr, size,
// user_addr*/ NULL, REASON_ANY);
    // break;
    }
    DMA_FROM_DEVICE => {
    kmsan_internal_unpoison_memory(addr, size,
// checked*/ false);
    // break;
    }
    DMA_NONE => {
    // break;
    }
    }
    }
// Helper function to handle DMA data transfers.
#[no_mangle]
pub unsafe extern "C" fn kmsan_handle_dma(phys: phys_addr_t, size: size_t, dir: dma_data_direction) {
    u64 page_offset, to_go;
pub static mut addr: *mut c_void = core::ptr::null_mut();
    if (PhysHighMem(phys)) {
    return;
    }
    addr = phys_to_virt(phys);
//
// The kernel may occasionally give us adjacent DMA pages not belonging
// to the same allocation. Process them separately to avoid triggering
// internal KMSAN checks.
//
    while (size > 0) {
    page_offset = offset_in_page(addr);
    to_go = min(PAGE_SIZE - page_offset, (u64)size);
    kmsan_handle_dma_page(addr, to_go, dir);
    addr += to_go;
    size -= to_go;
    }
    }
    EXPORT_SYMBOL_GPL(kmsan_handle_dma);
#[no_mangle]
pub unsafe extern "C" fn kmsan_handle_dma_sg(sg: *mut scatterlist, nents: c_int, dir: dma_data_direction) {
pub static mut item: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    for_each_sg(sg, item, nents, i) {
    kmsan_handle_dma(sg_phys(item), item.length, dir);
    }
    }
// Functions from kmsan-checks.h follow.
//
// To create an origin, kmsan_poison_memory() unwinds the stacks and stores it
// into the stack depot. This may cause deadlocks if done from within KMSAN
// runtime, therefore we bail out if kmsan_in_runtime().
//
#[no_mangle]
pub unsafe extern "C" fn kmsan_poison_memory(address: *const c_void, size: usize, flags: gfp_t) {
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return;
    }
    kmsan_enter_runtime();
// The users may want to poison/unpoison random memory.
    kmsan_internal_poison_memory(address, size, flags,
    KMSAN_POISON_NOCHECK);
    kmsan_leave_runtime();
    }
    EXPORT_SYMBOL(kmsan_poison_memory);
//
// Unlike kmsan_poison_memory(), this function can be used from within KMSAN
// runtime, because it does not trigger allocations or call instrumented code.
//
#[no_mangle]
pub unsafe extern "C" fn kmsan_unpoison_memory(address: *const c_void, size: usize) {
    let mut ua_flags = 0;
    if (!kmsan_enabled) {
    return;
    }
    ua_flags = user_access_save();
// The users may want to poison/unpoison random memory.
    kmsan_internal_unpoison_memory(address, size,
    KMSAN_POISON_NOCHECK);
    user_access_restore(ua_flags);
    }
    EXPORT_SYMBOL(kmsan_unpoison_memory);
//
// Version of kmsan_unpoison_memory() called from IRQ entry functions.
//
#[no_mangle]
pub unsafe extern "C" fn kmsan_unpoison_entry_regs(regs: *const pt_regs) {
    kmsan_unpoison_memory(regs, sizeof!(*regs));
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_check_memory(addr: *const c_void, size: usize) {
    if (!kmsan_enabled) {
    return;
    }
    return kmsan_internal_check_memory(addr, size,
// user_addr*/ NULL, REASON_ANY);
    }
    EXPORT_SYMBOL(kmsan_check_memory);
#[no_mangle]
pub unsafe extern "C" fn kmsan_enable_current() {
    KMSAN_WARN_ON(current.kmsan_ctx.depth == 0);
    current.kmsan_ctx.depth -= 1;
    }
    EXPORT_SYMBOL(kmsan_enable_current);
#[no_mangle]
pub unsafe extern "C" fn kmsan_disable_current() {
    current.kmsan_ctx.depth += 1;
    KMSAN_WARN_ON(current.kmsan_ctx.depth == 0);
    }
    EXPORT_SYMBOL(kmsan_disable_current);
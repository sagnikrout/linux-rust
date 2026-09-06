//! Automatically rewritten from C to Rust
//! Source: mm/kmsan/core.c
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
// KMSAN runtime library.
//
// Copyright (C) 2017-2022 Google LLC
// Author: Alexander Potapenko <glider@google.com>
//

    let mut kmsan_enabled = 0;
//
// Per-CPU KMSAN context to be used in interrupts, where current->kmsan is
// unavailable.
//
pub static mut struct kmsan_ctx: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn kmsan_internal_task_create(task: *mut task_struct) {
    let mut ctx = &task.kmsan_ctx;
    let mut info = current_thread_info();
    __memset(ctx, 0, sizeof!(*ctx));
    kmsan_internal_unpoison_memory(info, sizeof!(*info), false);
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_internal_poison_memory(address: *mut c_void, size: size_t, flags: gfp_t, poison_flags: c_uint) {
    u32 extra_bits =
    kmsan_extra_bits(/*depth*/ 0, poison_flags & KMSAN_POISON_FREE);
pub static mut checked: bool = false;
    let mut handle;
    handle = kmsan_save_stack_with_flags(flags, extra_bits);
    kmsan_internal_set_shadow_origin(address, size, -1, handle, checked);
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_internal_unpoison_memory(address: *mut c_void, size: usize, checked: bool) {
    kmsan_internal_set_shadow_origin(address, size, 0, 0, checked);
    }
    depot_stack_handle_t kmsan_save_stack_with_flags(gfp_t flags,
    unsigned int extra)
    {
    unsigned long entries[KMSAN_STACK_DEPTH];
    let mut nr_entries = 0;
    let mut handle;
    nr_entries = stack_trace_save(entries, KMSAN_STACK_DEPTH, 0);
    handle = stack_depot_save(entries, nr_entries, flags);
    return stack_depot_set_extra_bits(handle, extra);
    }
// Copy the metadata following the memmove() behavior.
#[no_mangle]
pub unsafe extern "C" fn kmsan_internal_memmove_metadata(dst: *mut c_void, src: *mut c_void, n: usize) {
pub static mut prev_old_origin: depot_stack_handle_t = 0;
    let mut i = 0;
    let mut iter = 0;
    let mut step = 0;
    let mut src_off = 0;
    let mut dst_off = 0;
    let mut oiter_src = 0;
    let mut oiter_dst = 0;
pub static mut old_origin: depot_stack_handle_t = 0;
    let mut origin_src = core::ptr::null_mut();
    let mut origin_dst = core::ptr::null_mut();
    let mut shadow_src = core::ptr::null_mut();
    let mut shadow_dst = core::ptr::null_mut();
pub static mut align_shadow_dst: *mut c_void = core::ptr::null_mut();
    let mut backwards = 0;
    shadow_dst = kmsan_get_metadata(dst, KMSAN_META_SHADOW);
    if (!shadow_dst) {
    return;
    }
    KMSAN_WARN_ON(!kmsan_metadata_is_contiguous(dst, n));
    align_shadow_dst =
    ALIGN_DOWN((u64)shadow_dst, KMSAN_ORIGIN_SIZE);
    shadow_src = kmsan_get_metadata(src, KMSAN_META_SHADOW);
    if (!shadow_src) {
// @src is untracked: mark @dst as initialized.
    kmsan_internal_unpoison_memory(dst, n, /*checked*/ false);
    return;
    }
    KMSAN_WARN_ON(!kmsan_metadata_is_contiguous(src, n));
    origin_dst = kmsan_get_metadata(dst, KMSAN_META_ORIGIN);
    origin_src = kmsan_get_metadata(src, KMSAN_META_ORIGIN);
    KMSAN_WARN_ON(!origin_dst || !origin_src);
    backwards = dst > src;
    step = backwards ? -1 : 1;
    iter = backwards ? n - 1 : 0;
    src_off = (u64)src % KMSAN_ORIGIN_SIZE;
    dst_off = (u64)dst % KMSAN_ORIGIN_SIZE;
// Copy shadow bytes one by one, updating the origins if necessary.
    while (i < n) {
    oiter_src = (iter + src_off) / KMSAN_ORIGIN_SIZE;
    oiter_dst = (iter + dst_off) / KMSAN_ORIGIN_SIZE;
    if (!shadow_src[iter]) {
    shadow_dst[iter] = 0;
    if (!align_shadow_dst[oiter_dst]) {
    origin_dst[oiter_dst] = 0;
    }
    continue;
    }
    shadow_dst[iter] = shadow_src[iter];
    old_origin = origin_src[oiter_src];
    if (old_origin == prev_old_origin) {
    new_origin = prev_new_origin;
    }
    else {
//
// kmsan_internal_chain_origin() may return
// NULL, but we don't want to lose the previous
// origin value.
//
    new_origin = kmsan_internal_chain_origin(old_origin);
    if (!new_origin) {
    new_origin = old_origin;
    }
    }
    origin_dst[oiter_dst] = new_origin;
    prev_new_origin = new_origin;
    prev_old_origin = old_origin;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_internal_chain_origin(id: depot_stack_handle_t) -> depot_stack_handle_t {
    unsigned long entries[3];
    let mut extra_bits = 0;
    let mut depth = 0;
    let mut uaf = 0;
    let mut handle;
    if (!id) {
    return id;
    }
//
// Make sure we have enough spare bits in @id to hold the UAF bit and
// the chain depth.
//
    BUILD_BUG_ON!((1 << STACK_DEPOT_EXTRA_BITS) <=
    (KMSAN_MAX_ORIGIN_DEPTH << 1));
    extra_bits = stack_depot_get_extra_bits(id);
    depth = kmsan_depth_from_eb(extra_bits);
    uaf = kmsan_uaf_from_eb(extra_bits);
//
// Stop chaining origins once the depth reached KMSAN_MAX_ORIGIN_DEPTH.
// This mostly happens in the case structures with uninitialized padding
// are copied around many times. Origin chains for such structures are
// usually periodic, and it does not make sense to fully store them.
//
    if (depth == KMSAN_MAX_ORIGIN_DEPTH) {
    return id;
    }
    depth += 1;
    extra_bits = kmsan_extra_bits(depth, uaf);
    entries[0] = KMSAN_CHAIN_MAGIC_ORIGIN;
    entries[1] = kmsan_save_stack_with_flags(__GFP_HIGH, 0);
    entries[2] = id;
//
// @entries is a local var in non-instrumented code, so KMSAN does not
// know it is initialized. Explicitly unpoison it to avoid false
// positives when stack_depot_save() passes it to instrumented code.
//
    kmsan_internal_unpoison_memory(entries, sizeof!(entries), false);
    handle = stack_depot_save(entries, ARRAY_SIZE!(entries), __GFP_HIGH);
    return stack_depot_set_extra_bits(handle, extra_bits);
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_internal_set_shadow_origin(addr: *mut c_void, size: size_t, b: c_int, origin: u32, checked: bool) {
pub static mut address: u64 = 0;
pub static mut shadow_start: *mut c_void = core::ptr::null_mut();
    let mut aligned_shadow = core::ptr::null_mut();
    let mut origin_start = core::ptr::null_mut();
pub static mut pad: usize = 0;
    KMSAN_WARN_ON(!kmsan_metadata_is_contiguous(addr, size));
    shadow_start = kmsan_get_metadata(addr, KMSAN_META_SHADOW);
    if (!shadow_start) {
//
// kmsan_metadata_is_contiguous() is true, so either all shadow
// and origin pages are NULL, or all are non-NULL.
//
    if (checked) {
    pr_err!("%s: not memsetting %ld bytes starting at %px, because the shadow is core::ptr::null_mut()\n",
    __func__, size, addr);
    KMSAN_WARN_ON(true);
    }
    return;
    }
    __memset(shadow_start, b, size);
    if (IS_ALIGNED(address, KMSAN_ORIGIN_SIZE)) {
    aligned_shadow = shadow_start;
    } else {
    pad = address % KMSAN_ORIGIN_SIZE;
    address -= pad;
    aligned_shadow = shadow_start - pad;
    size += pad;
    }
    size = ALIGN(size, KMSAN_ORIGIN_SIZE);
    origin_start =
    kmsan_get_metadata(address, KMSAN_META_ORIGIN);
//
// If the new origin is non-zero, assume that the shadow byte is also non-zero,
// and unconditionally overwrite the old origin slot.
// If the new origin is zero, overwrite the old origin slot iff the
// corresponding shadow slot is zero.
//
    while (i < size / KMSAN_ORIGIN_SIZE) {
    if (origin || !aligned_shadow[i]) {
    origin_start[i] = origin;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_vmalloc_to_page_or_null(vaddr: *mut c_void) -> *mut c_void {
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (!kmsan_internal_is_vmalloc_addr(vaddr) &&
    !kmsan_internal_is_module_addr(vaddr)) {
    return core::ptr::null_mut();
    }
    page = vmalloc_to_page(vaddr);
    if (pfn_valid(page_to_pfn(page))) {
    return page;
    }
    else {
    return core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_internal_check_memory(addr: *mut c_void, size: size_t, user_addr: *mut c_void, reason: c_int) {
pub static mut cur_origin: depot_stack_handle_t = 0;
pub static mut addr64: c_ulong = 0;
    let mut origin = core::ptr::null_mut();
    let mut shadow = core::ptr::null_mut();
pub static mut cur_off_start: c_int = 0;
    let mut chunk_size = 0;
pub static mut pos: usize = 0;
    if (!size) {
    return;
    }
    KMSAN_WARN_ON(!kmsan_metadata_is_contiguous(addr, size));
    while (pos < size) {
    chunk_size = min(size - pos,
    PAGE_SIZE - ((addr64 + pos) % PAGE_SIZE));
    shadow = kmsan_get_metadata((addr64 + pos),
    KMSAN_META_SHADOW);
    if (!shadow) {
//
// This page is untracked. If there were uninitialized
// bytes before, report them.
//
    if (cur_origin) {
    kmsan_report(cur_origin, addr, size,
    cur_off_start, pos - 1, user_addr,
    reason);
    }
    cur_origin = 0;
    cur_off_start = -1;
    pos += chunk_size;
    continue;
    }
    while (i < chunk_size) {
    if (!shadow[i]) {
//
// This byte is unpoisoned. If there were
// poisoned bytes before, report them.
//
    if (cur_origin) {
    kmsan_report(cur_origin, addr, size,
    cur_off_start, pos + i - 1,
    user_addr, reason);
    }
    cur_origin = 0;
    cur_off_start = -1;
    continue;
    }
    origin = kmsan_get_metadata((addr64 + pos + i),
    KMSAN_META_ORIGIN);
    KMSAN_WARN_ON(!origin);
    new_origin = *origin;
//
// Encountered new origin - report the previous
// uninitialized range.
//
    if (cur_origin != new_origin) {
    if (cur_origin) {
    kmsan_report(cur_origin, addr, size,
    cur_off_start, pos + i - 1,
    user_addr, reason);
    }
    cur_origin = new_origin;
    cur_off_start = pos + i;
    }
    }
    pos += chunk_size;
    }
    KMSAN_WARN_ON(pos != size);
    if (cur_origin) {
    kmsan_report(cur_origin, addr, size, cur_off_start, pos - 1,
    user_addr, reason);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_metadata_is_contiguous(addr: *mut c_void, size: usize) -> bool {
    let mut cur_shadow = core::ptr::null_mut(), *next_shadow = core::ptr::null_mut(), *cur_origin = core::ptr::null_mut(),
// next_origin = NULL;
pub static mut cur_addr: u64 = 0;
pub static mut origin_p: *mut c_void = core::ptr::null_mut();
pub static mut all_untracked: bool = false;
    if (!size) {
    return true;
    }
// The whole range belongs to the same page.
    if (ALIGN_DOWN(cur_addr + size - 1, PAGE_SIZE) ==
    ALIGN_DOWN(cur_addr, PAGE_SIZE)) {
    return true;
    }
    cur_shadow = kmsan_get_metadata(cur_addr, /*is_origin*/ false);
    if (!cur_shadow) {
    all_untracked = true;
    }
    cur_origin = kmsan_get_metadata(cur_addr, /*is_origin*/ true);
    if (all_untracked && cur_origin) {
// goto;
    }
    while (next_addr < (u64)addr + size) {
    next_shadow = kmsan_get_metadata(next_addr, false);
    next_origin = kmsan_get_metadata(next_addr, true);
    if (all_untracked) {
    if (next_shadow || next_origin) {
// goto;
    }
    if (!next_shadow && !next_origin) {
    continue;
    }
    }
    if (((u64)cur_shadow == ((u64)next_shadow - PAGE_SIZE)) &&
    ((u64)cur_origin == ((u64)next_origin - PAGE_SIZE))) {
    continue;
    }
// goto;
    }
    return true;
// label;
    pr_err!("%s: attempting to access two shadow page ranges.\n", __func__);
    pr_err!("Access of size %ld at %px.\n", size, addr);
    pr_err!("Addresses belonging to different ranges: %px and %px\n",
    cur_addr, next_addr);
    pr_err!("page[0].shadow: %px, page[1].shadow: %px\n", cur_shadow,
    next_shadow);
    pr_err!("page[0].origin: %px, page[1].origin: %px\n", cur_origin,
    next_origin);
    origin_p = kmsan_get_metadata(addr, KMSAN_META_ORIGIN);
    if (origin_p) {
    pr_err!("Origin: %08x\n", *origin_p);
    kmsan_print_origin(*origin_p);
    } else {
    pr_err!("Origin: unavailable\n");
    }
    return false;
    }
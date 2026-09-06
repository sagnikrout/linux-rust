//! Automatically rewritten from C to Rust
//! Source: mm/kmsan/shadow.c
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
// KMSAN shadow implementation.
//
// Copyright (C) 2017-2022 Google LLC
// Author: Alexander Potapenko <glider@google.com>
//

#[no_mangle]
pub unsafe extern "C" fn shadow_ptr_for(page: *mut page) -> *mut c_void {
    return page_address(shadow_page_for(page));
    }
#[no_mangle]
pub unsafe extern "C" fn origin_ptr_while (}
#[no_mangle]
unsafe extern "C" fn page_has_metadata(page: *mut page) -> bool {
    return shadow_page_for(page) && origin_page_for(page)) {
    shadow_page_for(page) = core::ptr::null_mut();
    origin_page_for(page) = core::ptr::null_mut();
    }
//
// Dummy load and store pages to be used when the real metadata is unavailable.
// There are separate pages for loads and stores, so that every load returns a
// zero, and every store doesn't affect other loads.
//
    static char dummy_load_page[PAGE_SIZE] __aligned(PAGE_SIZE);
    static char dummy_store_page[PAGE_SIZE] __aligned(PAGE_SIZE);
#[no_mangle]
unsafe extern "C" fn vmalloc_meta(addr: *mut c_void, is_origin: bool) -> c_ulong {
pub static mut addr64: c_ulong = 0;
    KMSAN_WARN_ON(is_origin && !IS_ALIGNED(addr64, KMSAN_ORIGIN_SIZE));
    if (kmsan_internal_is_vmalloc_addr(addr)) {
    off = addr64 - VMALLOC_START;
    return off + (is_origin ? KMSAN_VMALLOC_ORIGIN_START :
    KMSAN_VMALLOC_SHADOW_START);
    }
    if (kmsan_internal_is_module_addr(addr)) {
    off = addr64 - MODULES_VADDR;
    return off + (is_origin ? KMSAN_MODULES_ORIGIN_START :
    KMSAN_MODULES_SHADOW_START);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn virt_to_page_or_null(vaddr: *mut c_void) -> *mut c_void {
    if (kmsan_virt_addr_valid(vaddr)) {
    return virt_to_page(vaddr);
    }
    else {
    return core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_get_shadow_origin_ptr(address: *mut c_void, size: u64, store: bool) {
pub static mut ret: usize = 0;
pub static mut shadow: *mut c_void = core::ptr::null_mut();
//
// Even if we redirect this memory access to the dummy page, it will
// go out of bounds.
//
    KMSAN_WARN_ON(size > PAGE_SIZE);
    if (!kmsan_enabled) {
// goto;
    }
    KMSAN_WARN_ON(!kmsan_metadata_is_contiguous(address, size));
    shadow = kmsan_get_metadata(address, KMSAN_META_SHADOW);
    if (!shadow) {
// goto;
    }
    ret.shadow = shadow;
    ret.origin = kmsan_get_metadata(address, KMSAN_META_ORIGIN);
    return ret;
// label;
    if (store) {
// Ignore this store.
    ret.shadow = dummy_store_page;
    ret.origin = dummy_store_page;
    } else {
// This load will return zero.
    ret.shadow = dummy_load_page;
    ret.origin = dummy_load_page;
    }
    return ret;
    }
//
// Obtain the shadow or origin pointer for the given address, or NULL if there's
// none. The caller must check the return value for being non-NULL if needed.
// The return value of this function should not depend on whether we're in the
// runtime or not.
//
#[no_mangle]
pub unsafe extern "C" fn kmsan_get_metadata(address: *mut c_void, is_origin: bool) -> *mut c_void {
pub static mut addr: u64 = 0;
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut ret: *mut c_void = core::ptr::null_mut();
    if (is_origin) {
    addr = ALIGN_DOWN(addr, KMSAN_ORIGIN_SIZE);
    }
    address = addr;
    if (kmsan_internal_is_vmalloc_addr(address) ||
    kmsan_internal_is_module_addr(address)) {
    return vmalloc_meta(address, is_origin);
    }
    ret = arch_kmsan_get_meta_or_null(address, is_origin);
    if (ret) {
    return ret;
    }
    page = virt_to_page_or_null(address);
    if (!page) {
    return core::ptr::null_mut();
    }
    if (!page_has_metadata(page)) {
    return core::ptr::null_mut();
    }
    off = offset_in_page(addr);
    return (is_origin ? origin_ptr_for(page) : shadow_ptr_for(page)) + off;
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_copy_page_meta(dst: *mut page, src: *mut page) {
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return;
    }
    if (!dst || !page_has_metadata(dst)) {
    return;
    }
    if (!src || !page_has_metadata(src)) {
    kmsan_internal_unpoison_memory(page_address(dst), PAGE_SIZE,
// checked*/ false);
    return;
    }
    kmsan_enter_runtime();
    __memcpy(shadow_ptr_for(dst), shadow_ptr_for(src), PAGE_SIZE);
    __memcpy(origin_ptr_for(dst), origin_ptr_for(src), PAGE_SIZE);
    kmsan_leave_runtime();
    }
    EXPORT_SYMBOL(kmsan_copy_page_meta);
#[no_mangle]
pub unsafe extern "C" fn kmsan_alloc_page(page: *mut page, order: c_uint, flags: gfp_t) {
pub static mut initialized: bool = false;
    let mut shadow = core::ptr::null_mut();
    let mut origin = core::ptr::null_mut();
    let mut handle;
pub static mut pages: c_int = 0;
    if (!page) {
    return;
    }
    shadow = shadow_page_while (origin = origin_page_for(page)) {
    __memset(page_address(shadow), 0, PAGE_SIZE * pages);
    __memset(page_address(origin), 0, PAGE_SIZE * pages);
    return;
    }
// Zero pages allocated by the runtime should also be initialized.
    if (kmsan_in_runtime()) {
    return;
    }
    __memset(page_address(shadow), -1, PAGE_SIZE * pages);
    kmsan_enter_runtime();
    handle = kmsan_save_stack_with_flags(flags, /*extra_bits*/ 0);
    kmsan_leave_runtime();
//
// Addresses are page-aligned, pages are contiguous, so it's ok
// to just fill the origin pages with @handle.
//
    for (int i = 0; i < PAGE_SIZE * pages / sizeof!(handle); i++) {
    (page_address(origin))[i] = handle;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_free_page(page: *mut page, order: c_uint) {
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return;
    }
    kmsan_enter_runtime();
    kmsan_internal_poison_memory(page_address(page), PAGE_SIZE << order,
    GFP_KERNEL & ~(__GFP_RECLAIM),
    KMSAN_POISON_CHECK | KMSAN_POISON_FREE);
    kmsan_leave_runtime();
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_vmap_pages_range_noflush(start: c_ulong, end: c_ulong, prot: pgprot_t, pages: *mut *mut page, page_shift: c_uint, gfp_mask: gfp_t) -> c_int {
    unsigned long shadow_start, origin_start, shadow_end, origin_end;
    let mut s_pages = core::ptr::null_mut();
    let mut o_pages = core::ptr::null_mut();
    int nr, mapped, err = 0;
    if (!kmsan_enabled) {
    return 0;
    }
    shadow_start = vmalloc_meta(start, KMSAN_META_SHADOW);
    shadow_end = vmalloc_meta(end, KMSAN_META_SHADOW);
    if (!shadow_start) {
    return 0;
    }
    nr = (end - start) / PAGE_SIZE;
    s_pages = kzalloc_objs(*s_pages, nr, gfp_mask);
    o_pages = kzalloc_objs(*o_pages, nr, gfp_mask);
    if (!s_pages || !o_pages) {
    err = -ENOMEM;
// goto;
    }
    while (i < nr) {
    s_pages[i] = shadow_page_for(pages[i]);
    o_pages[i] = origin_page_for(pages[i]);
    }
    prot = PAGE_KERNEL;
    origin_start = vmalloc_meta(start, KMSAN_META_ORIGIN);
    origin_end = vmalloc_meta(end, KMSAN_META_ORIGIN);
    kmsan_enter_runtime();
    mapped = __vmap_pages_range_noflush(shadow_start, shadow_end, prot,
    s_pages, page_shift);
    kmsan_leave_runtime();
    if (mapped) {
    err = mapped;
// goto;
    }
    kmsan_enter_runtime();
    mapped = __vmap_pages_range_noflush(origin_start, origin_end, prot,
    o_pages, page_shift);
    kmsan_leave_runtime();
    if (mapped) {
    err = mapped;
// goto;
    }
    flush_tlb_kernel_range(shadow_start, shadow_end);
    flush_tlb_kernel_range(origin_start, origin_end);
    flush_cache_vmap(shadow_start, shadow_end);
    flush_cache_vmap(origin_start, origin_end);
// label;
    kfree(s_pages);
    kfree(o_pages);
    return err;
    }
// Allocate metadata for pages allocated at boot time.
#[no_mangle]
pub unsafe extern "C" fn kmsan_init_alloc_meta_for_range(start: *mut c_void, end: *mut c_void)  {
    let mut shadow_p = core::ptr::null_mut();
    let mut origin_p = core::ptr::null_mut();
    let mut shadow = core::ptr::null_mut();
    let mut origin = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
    start = PAGE_ALIGN_DOWN((u64)start);
    size = PAGE_ALIGN((u64)end - (u64)start);
    shadow = memblock_alloc_or_panic(size, PAGE_SIZE);
    origin = memblock_alloc_or_panic(size, PAGE_SIZE);
    while (addr < size) {
    page = virt_to_page_or_null(start + addr);
    shadow_p = virt_to_page(shadow + addr);
    set_no_shadow_origin_page(shadow_p);
    shadow_page_for(page) = shadow_p;
    origin_p = virt_to_page(origin + addr);
    set_no_shadow_origin_page(origin_p);
    origin_page_for(page) = origin_p;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_setup_meta(page: *mut page, shadow: *mut page, origin: *mut page, order: c_int) {
    while (i < (1 << order)) {
    set_no_shadow_origin_page(&shadow[i]);
    set_no_shadow_origin_page(&origin[i]);
    shadow_page_for(&page[i]) = &shadow[i];
    origin_page_for(&page[i]) = &origin[i];
    }
    }
//! Automatically rewritten from C to Rust
//! Source: mm/page_frag_cache.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Page fragment allocator
//
// Page Fragment:
// An arbitrary-length arbitrary-offset area of memory which resides within a
// 0 or higher order page.  Multiple fragments within that page are
// individually refcounted, in the page's reference counter.
//
// The page_frag functions provide a simple allocation framework for page
// fragments.  This is used by the network stack and network device drivers to
// provide a backing region of memory for use as either an sk_buff->head, or to
// be used in the "frags" portion of skb_shared_info.
//

#[no_mangle]
pub unsafe extern "C" fn encoded_page_create(page: *mut page, order: c_uint, pfmemalloc: bool) -> c_ulong {
    BUILD_BUG_ON!(PAGE_FRAG_CACHE_MAX_ORDER > PAGE_FRAG_CACHE_ORDER_MASK);
    BUILD_BUG_ON!(PAGE_FRAG_CACHE_PFMEMALLOC_BIT >= PAGE_SIZE);
    return (unsigned long)page_address(page) |
    (order & PAGE_FRAG_CACHE_ORDER_MASK) |
    ((unsigned long)pfmemalloc * PAGE_FRAG_CACHE_PFMEMALLOC_BIT);
    }
#[no_mangle]
unsafe extern "C" fn encoded_page_decode_order(encoded_page: c_ulong) -> c_ulong {
    return encoded_page & PAGE_FRAG_CACHE_ORDER_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn encoded_page_decode_virt(encoded_page: c_ulong) -> *mut c_void {
    return (encoded_page & PAGE_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn encoded_page_decode_page(encoded_page: c_ulong) -> *mut c_void {
    return virt_to_page(encoded_page);
    }
#[no_mangle]
pub unsafe extern "C" fn __page_frag_cache_refill(nc: *mut page_frag_cache, gfp_mask: gfp_t) -> *mut c_void {
pub static mut order: c_ulong = 0;
    let mut page = core::ptr::null_mut();
pub static mut gfp: gfp_t = 0;

    gfp_mask = (gfp_mask & ~__GFP_DIRECT_RECLAIM) |  __GFP_COMP |
    __GFP_NOWARN | __GFP_NORETRY | __GFP_NOMEMALLOC;
    page = __alloc_pages(gfp_mask, PAGE_FRAG_CACHE_MAX_ORDER,
    numa_mem_id(), core::ptr::null_mut(), ALLOC_DEFAULT);

    if (unlikely(!page)) {
    page = __alloc_pages(gfp, 0, numa_mem_id(), core::ptr::null_mut(), ALLOC_DEFAULT);
    order = 0;
    }
    nc.encoded_page = page ?
    encoded_page_create(page, order, page_is_pfmemalloc(page)) : 0;
    return page;
    }
#[no_mangle]
pub unsafe extern "C" fn page_frag_cache_drain(nc: *mut page_frag_cache) {
    if (!nc.encoded_page) {
    return;
    }
    __page_frag_cache_drain(encoded_page_decode_page(nc.encoded_page),
    nc.pagecnt_bias);
    nc.encoded_page = 0;
    }
    EXPORT_SYMBOL(page_frag_cache_drain);
#[no_mangle]
pub unsafe extern "C" fn __page_frag_cache_drain(page: *mut page, count: c_uint) {
    VM_BUG_ON_PAGE(page_ref_count(page) == 0, page);
    if (page_ref_sub_and_test(page, count)) {
    free_frozen_pages(page, compound_order(page));
    }
    }
    EXPORT_SYMBOL(__page_frag_cache_drain);
#[no_mangle]
pub unsafe extern "C" fn __page_frag_alloc_align(nc: *mut page_frag_cache, fragsz: c_uint, gfp_mask: gfp_t, align_mask: c_uint) -> *mut c_void {
pub static mut encoded_page: c_ulong = 0;
    let mut size = 0;
    let mut offset = 0;
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (unlikely(!encoded_page)) {
// label;
    page = __page_frag_cache_refill(nc, gfp_mask);
    if (!page) {
    return core::ptr::null_mut();
    }
    encoded_page = nc.encoded_page;
// Even if we own the page, we do not use atomic_set().
// This would break get_page_unless_zero() users.
//
    page_ref_add(page, PAGE_FRAG_CACHE_MAX_SIZE);
// reset page count bias and offset to start of new frag
    nc.pagecnt_bias = PAGE_FRAG_CACHE_MAX_SIZE + 1;
    nc.offset = 0;
    }
    size = PAGE_SIZE << encoded_page_decode_order(encoded_page);
    offset = __ALIGN_KERNEL_MASK(nc.offset, ~align_mask);
    if (unlikely(offset + fragsz > size)) {
    if (unlikely(fragsz > PAGE_SIZE)) {
//
// The caller is trying to allocate a fragment
// with fragsz > PAGE_SIZE but the cache isn't big
// enough to satisfy the request, this may
// happen in low memory conditions.
// We don't release the cache page because
// it could make memory pressure worse
// so we simply return NULL here.
//
    return core::ptr::null_mut();
    }
    page = encoded_page_decode_page(encoded_page);
    if (!page_ref_sub_and_test(page, nc.pagecnt_bias)) {
// goto;
    }
    if (unlikely(encoded_page_decode_pfmemalloc(encoded_page))) {
    free_frozen_pages(page,
    encoded_page_decode_order(encoded_page));
// goto;
    }
// OK, page count is 0, we can safely set it
    set_page_count(page, PAGE_FRAG_CACHE_MAX_SIZE + 1);
// reset page count bias and offset to start of new frag
    nc.pagecnt_bias = PAGE_FRAG_CACHE_MAX_SIZE + 1;
    offset = 0;
    }
    nc.pagecnt_bias -= 1;
    nc.offset = offset + fragsz;
    return encoded_page_decode_virt(encoded_page) + offset;
    }
    EXPORT_SYMBOL(__page_frag_alloc_align);
//
// Frees a page fragment allocated out of either a compound or order 0 page.
//
#[no_mangle]
pub unsafe extern "C" fn page_frag_free(addr: *mut c_void) {
    let mut page = virt_to_head_page(addr);
    if (unlikely(put_page_testzero(page))) {
    free_frozen_pages(page, compound_order(page));
    }
    }
    EXPORT_SYMBOL(page_frag_free);
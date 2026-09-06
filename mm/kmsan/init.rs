//! Automatically rewritten from C to Rust
//! Source: mm/kmsan/init.c
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
// KMSAN initialization routines.
//
// Copyright (C) 2017-2021 Google LLC
// Author: Alexander Potapenko <glider@google.com>
//

pub const NUM_FUTURE_RANGES: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct start_end_pair {
    pub end: u64 start,,
}

    static struct start_end_pair start_end_pairs[NUM_FUTURE_RANGES] __initdata;
    static int future_index __initdata;
//
// Record a range of memory for which the metadata pages will be created once
// the page allocator becomes available.
//
#[no_mangle]
unsafe extern "C" fn kmsan_record_future_shadow_range(start: *mut c_void, end: *mut c_void)  {
pub static mut nstart: u64 = 0;
pub static mut merged: bool = false;
    KMSAN_WARN_ON(future_index == NUM_FUTURE_RANGES);
    KMSAN_WARN_ON((nstart >= nend) ||
// Virtual address 0 is valid on s390.
    (!IS_ENABLED!(CONFIG_S390) && !nstart) || !nend);
    nstart = ALIGN_DOWN(nstart, PAGE_SIZE);
    nend = ALIGN(nend, PAGE_SIZE);
//
// Scan the existing ranges to see if any of them overlaps with
// [start, end). In that case, merge the two ranges instead of
// creating a new one.
// The number of ranges is less than 20, so there is no need to organize
// them into a more intelligent data structure.
//
    while (i < future_index) {
    cstart = start_end_pairs[i].start;
    cend = start_end_pairs[i].end;
    if ((cstart < nstart && cend < nstart) ||
    (cstart > nend && cend > nend)) {
// ranges are disjoint - do not merge
    continue;
    }
    start_end_pairs[i].start = min(nstart, cstart);
    start_end_pairs[i].end = max(nend, cend);
    merged = true;
    break;
    }
    if (merged) {
    return;
    }
    start_end_pairs[future_index].start = nstart;
    start_end_pairs[future_index].end = nend;
    future_index += 1;
    }
//
// Initialize the shadow for existing mappings during kernel initialization.
// These include kernel text/data sections, NODE_DATA and future ranges
// registered while creating other data (e.g. percpu).
//
// Allocations via memblock can be only done before slab is initialized.
//
#[no_mangle]
pub unsafe extern "C" fn kmsan_init_shadow()  {
pub static mut nd_size: usize = 0;
    phys_addr_t p_start, p_end;
    let mut loop = 0;
    let mut nid = 0;
    for_each_reserved_mem_range(loop, &p_start, &p_end) {
    kmsan_record_future_shadow_range(phys_to_virt(p_start),
    phys_to_virt(p_end));
    }
// Allocate shadow for .data
    kmsan_record_future_shadow_range(_sdata, _edata);
    for_each_online_node(nid) {
    kmsan_record_future_shadow_range(
    NODE_DATA(nid), NODE_DATA(nid) + nd_size);
    }
    for (int i = 0; i < future_index; i++) {
    kmsan_init_alloc_meta_for_range(
    start_end_pairs[i].start,
    start_end_pairs[i].end);
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct metadata_page_pair {
    pub origin: *mut *mut page shadow,,
}

    static struct metadata_page_pair held_back[NR_PAGE_ORDERS] __initdata;
//
// Eager metadata allocation. When the memblock allocator is freeing pages to
// pagealloc, we use 2/3 of them as metadata for the remaining 1/3.
// We store the pointers to the returned blocks of pages in held_back[] grouped
// by their order: when kmsan_memblock_free_pages() is called for the first
// time with a certain order, it is reserved as a shadow block, for the second
// time - as an origin block. On the third time the incoming block receives its
// shadow and origin ranges from the previously saved shadow and origin blocks,
// after which held_back[order] can be used again.
//
// At the very end there may be leftover blocks in held_back[]. They are
// collected later by kmsan_memblock_discard().
//
#[no_mangle]
pub unsafe extern "C" fn kmsan_memblock_free_pages(page: *mut page, order: c_uint) -> bool {
    let mut shadow = core::ptr::null_mut();
    let mut origin = core::ptr::null_mut();
    if (!held_back[order].shadow) {
    held_back[order].shadow = page;
    return false;
    }
    if (!held_back[order].origin) {
    held_back[order].origin = page;
    return false;
    }
    shadow = held_back[order].shadow;
    origin = held_back[order].origin;
    kmsan_setup_meta(page, shadow, origin, order);
    held_back[order].shadow = core::ptr::null_mut();
    held_back[order].origin = core::ptr::null_mut();
    return true;
    }
pub const MAX_BLOCKS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smallstack {
    pub items: [*mut page; MAX_BLOCKS],
    pub index: c_int,
    pub order: c_int,
}

pub static mut smallstack: usize = 0;
#[no_mangle]
unsafe extern "C" fn smallstack_push(stack: *mut smallstack, pages: *mut page) {
    KMSAN_WARN_ON(stack.index == MAX_BLOCKS);
    stack.items[stack.index] = pages;
    stack.index += 1;
    }

#[no_mangle]
pub unsafe extern "C" fn smallstack_pop(stack: *mut smallstack) -> *mut c_void {
pub static mut ret: *mut c_void = core::ptr::null_mut();
    KMSAN_WARN_ON(stack.index == 0);
    stack.index -= 1;
    ret = stack.items[stack.index];
    stack.items[stack.index] = core::ptr::null_mut();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn do_collection() {
    let mut page = core::ptr::null_mut();
    let mut shadow = core::ptr::null_mut();
    let mut origin = core::ptr::null_mut();
    while (collect.index >= 3) {
    page = smallstack_pop(&collect);
    shadow = smallstack_pop(&collect);
    origin = smallstack_pop(&collect);
    kmsan_setup_meta(page, shadow, origin, collect.order);
    __free_pages_core(page, collect.order, MEMINIT_EARLY);
    }
    }
#[no_mangle]
unsafe extern "C" fn collect_split() {
pub static mut smallstack: usize = 0;
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (!collect.order) {
    return;
    }
    while (collect.index) {
    page = smallstack_pop(&collect);
    smallstack_push(&tmp, &page[0]);
    smallstack_push(&tmp, &page[1 << tmp.order]);
    }
    __memcpy(&collect, &tmp, sizeof!(tmp));
    }
//
// Memblock is about to go away. Split the page blocks left over in held_back[]
// and return 1/3 of that memory to the system.
//
#[no_mangle]
unsafe extern "C" fn kmsan_memblock_discard() {
//
// For each order=N:
// - push held_back[N].shadow and .origin to @collect;
// - while there are >= 3 elements in @collect, do garbage collection:
// - pop 3 ranges from @collect;
// - use two of them as shadow and origin for the third one;
// - repeat;
// - split each remaining element from @collect into 2 ranges of
// order=N-1,
// - repeat.
//
    collect.order = MAX_PAGE_ORDER;
    while (i >= 0) {
    if (held_back[i].shadow) {
    smallstack_push(&collect, held_back[i].shadow);
    }
    if (held_back[i].origin) {
    smallstack_push(&collect, held_back[i].origin);
    }
    held_back[i].shadow = core::ptr::null_mut();
    held_back[i].origin = core::ptr::null_mut();
    do_collection();
    collect_split();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kmsan_init_runtime()  {
// Assuming current is init_task
    kmsan_internal_task_create(current);
    kmsan_memblock_discard();
    pr_info!("Starting KernelMemorySanitizer\n");
    pr_info!("ATTENTION: KMSAN is a debugging tool! Do not use it on production machines!\n");
    kmsan_enabled = true;
    }
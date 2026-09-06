//! Automatically rewritten from C to Rust
//! Source: kernel/power/snapshot.c
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
//
// linux/kernel/power/snapshot.c
//
// This file provides system snapshot/restore functionality for swsusp.
//
// Copyright (C) 1998-2005 Pavel Machek <pavel@ucw.cz>
// Copyright (C) 2006 Rafael J. Wysocki <rjw@sisk.pl>
//

    static bool hibernate_restore_protection;
    static bool hibernate_restore_protection_active;
#[no_mangle]
pub unsafe extern "C" fn enable_restore_image_protection() {
    hibernate_restore_protection = true;
    }
#[no_mangle]
pub unsafe extern "C" fn hibernate_restore_protection_begin() {
    hibernate_restore_protection_active = hibernate_restore_protection;
    }
#[no_mangle]
pub unsafe extern "C" fn hibernate_restore_protection_end() {
    hibernate_restore_protection_active = false;
    }
#[no_mangle]
pub unsafe extern "C" fn hibernate_restore_protect_page(page_address: *mut c_void) -> int __must_check {
    if (hibernate_restore_protection_active) {
    return set_memory_ro((unsigned long)page_address, 1);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hibernate_restore_unprotect_page(page_address: *mut c_void) -> c_int {
    if (hibernate_restore_protection_active) {
    return set_memory_rw((unsigned long)page_address, 1);
    }
    return 0;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: hibernate_restore_protection_begin
pub unsafe extern "C" fn hibernate_restore_protection_begin_dup() {}
#[no_mangle]
#[no_mangle]
// duplicate fn: hibernate_restore_protection_end
pub unsafe extern "C" fn hibernate_restore_protection_end_dup() {}
    static inline int __must_check hibernate_restore_protect_page(void *page_address) {return 0; }
#[no_mangle]
#[no_mangle]
// duplicate fn: hibernate_restore_unprotect_page
pub unsafe extern "C" fn hibernate_restore_unprotect_page_dup(page_address: *mut c_void) -> c_int {return 0; }

//
// The calls to set_direct_map_*() should not fail because remapping a page
// here means that we only update protection bits in an existing PTE.
// It is still worth to have a warning here if something changes and this
// will no longer be the case.
//
#[no_mangle]
pub unsafe extern "C" fn hibernate_map_page(page: *mut page) {
    if (IS_ENABLED!(CONFIG_ARCH_HAS_SET_DIRECT_MAP)) {
pub static mut ret: c_int = 0;
    if (ret) {
    pr_warn_once("Failed to remap page\n");
    }
    } else {
    debug_pagealloc_map_pages(page, 1);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hibernate_unmap_page(page: *mut page) {
    if (IS_ENABLED!(CONFIG_ARCH_HAS_SET_DIRECT_MAP)) {
pub static mut addr: c_ulong = 0;
pub static mut ret: c_int = 0;
    if (ret) {
    pr_warn_once("Failed to remap page\n");
    }
    flush_tlb_kernel_range(addr, addr + PAGE_SIZE);
    } else {
    debug_pagealloc_unmap_pages(page, 1);
    }
    }
// forward_decl: swsusp_page_is_free;
// forward_decl: swsusp_set_page_forbidden;
// forward_decl: swsusp_unset_page_forbidden;
//
// Number of bytes to reserve for memory allocations made by device drivers
// from their ->freeze() and ->freeze_noirq() callbacks so that they don't
// cause image creation to fail (tunable via /sys/power/reserved_size).
//
    let mut reserved_size = 0;
#[no_mangle]
pub unsafe extern "C" fn hibernate_reserved_size_init()  {
    reserved_size = SPARE_PAGES * PAGE_SIZE;
    }
//
// Preferred image size in bytes (tunable via /sys/power/image_size).
// When it is set to N, swsusp will do its best to ensure the image
// size will not exceed N bytes, but if that is impossible, it will
// try to create the smallest image possible.
//
    let mut image_size = 0;
#[no_mangle]
pub unsafe extern "C" fn hibernate_image_size_init()  {
    image_size = ((totalram_pages() * 2) / 5) * PAGE_SIZE;
    }
//
// List of PBEs needed for restoring the pages that were allocated before
// the suspend and included in the suspend image, but have also been
// allocated by the "resume" kernel, so their contents cannot be written
// directly to their "original" page frames.
//
pub static mut restore_pblist: *mut c_void = core::ptr::null_mut();
// struct linked_page is used to build chains of pages

#[repr(C)]
#[derive(Copy, Clone)]
pub struct linked_page {
    pub next: *mut linked_page,
    pub data: [c_char; LINKED_PAGE_DATA_SIZE],
    pub __packed: },
//
// List of "safe" pages (ie. pages that were not used by the image kernel
// before hibernation) that may be used as temporary storage for image kernel
// memory contents.
//
    pub safe_pages_list: *mut static struct linked_page,
Pointer to an auxiliary buffer (1 page)
    pub buffer: *mut static void,
pub const PG_ANY: c_int = 0;
pub const PG_SAFE: c_int = 1;
pub const PG_UNSAFE_CLEAR: c_int = 1;
pub const PG_UNSAFE_KEEP: c_int = 0;
    pub allocated_unsafe_pages: unsigned int,
//
// get_image_page - Allocate a page for a hibernation image.
// @gfp_mask: GFP mask for the allocation.
// @safe_needed: Get pages that were not used before hibernation (restore only)
//
// During image restoration, for storing the PBE list and the image data, we can
// only use memory pages that do not conflict with the pages used before
// hibernation.  The "unsafe" pages have PageNosaveFree set and we count them
// using allocated_unsafe_pages.
//
// Each allocated image page is marked as PageNosave and PageNosaveFree so that
// swsusp_free() can release it.
//
#[no_mangle]
pub unsafe extern "C" fn get_image_page(gfp_mask: gfp_t, safe_needed: c_int) -> *mut c_void {
    pub res: *mut c_void,
    pub )get_zeroed_page(gfp_mask): *mut res = (void,
    if (safe_needed) {
    while (res && swsusp_page_is_free(virt_to_page(res))) {
    }
// The page is unsafe, mark it for swsusp_free()
    pub )get_zeroed_page(gfp_mask): *mut res = (void,
    }
    if (res) {
    }
    pub res: return,
    }
#[no_mangle]
pub unsafe extern "C" fn __get_safe_page(gfp_mask: gfp_t) -> *mut c_void {
    if (safe_pages_list) {
    pub safe_pages_list: *mut *mut c_void ret =,
    pub safe_pages_list->next: safe_pages_list =,
    pub PAGE_SIZE): memset(ret, 0,,
    pub ret: return,
    }
    pub PG_SAFE): return get_image_page(gfp_mask,,
    }
#[no_mangle]
pub unsafe extern "C" fn get_safe_page(gfp_mask: gfp_t) -> c_ulong {
    pub long)__get_safe_page(gfp_mask): return (unsigned,
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_image_page(gfp_mask: gfp_t) -> *mut c_void {
    pub page: *mut page,
    pub alloc_page(gfp_mask): page =,
    if (page) {
    }
    pub page: return,
    }
#[no_mangle]
unsafe extern "C" fn recycle_safe_page(page_address: *mut c_void) {
    pub page_address: *mut *mut linked_page lp =,
    pub safe_pages_list: lp->next =,
    pub lp: safe_pages_list =,
    }
//
// free_image_page - Free a page allocated for hibernation image.
// @addr: Address of the page to free.
// @clear_nosave_free: If set, clear the PageNosaveFree bit for the page.
//
// The page to free should have been allocated by get_image_page() (page flags
// set by it are affected).
//
#[no_mangle]
pub unsafe extern "C" fn free_image_page(addr: *mut c_void, clear_nosave_free: c_int) {
    pub page: *mut page,
    pub virt_to_page(addr): page =,
    if (clear_nosave_free) {
    }
#[no_mangle]
pub unsafe extern "C" fn free_list_of_pages(list: *mut linked_page, clear_page_nosave: c_int) {
    }
    while (list) {
    pub list->next: *mut *mut linked_page lp =,
    pub clear_page_nosave): free_image_page(list,,
    pub lp: list =,
    }
    }
//
// struct chain_allocator is used for allocating small objects out of
// a linked list of pages called 'the chain'.
//
// The chain grows each time when there is no room for a new object in
// the current page.  The allocated objects cannot be freed individually.
// It is only possible to free them all at once, by freeing the entire
// chain.
//
// NOTE: The chain allocator may be inefficient if the allocated objects
// are not much smaller than PAGE_SIZE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chain_allocator {
//     pub /: *mut *mut *mut linked_page chain; / the chain,
    pub out: *mut *mut unsigned int used_space; / total size of objects allocated,
    of the current page */
//     pub /: *mut *mut gfp_t gfp_mask; / mask for allocating pages,
//     pub /: *mut *mut int safe_needed; / if set, only "safe" pages are allocated,
}

#[no_mangle]
pub unsafe extern "C" fn chain_init(ca: *mut chain_allocator, gfp_mask: gfp_t, safe_needed: c_int) {
    ca.chain = core::ptr::null_mut();
    ca.used_space = LINKED_PAGE_DATA_SIZE;
    ca.gfp_mask = gfp_mask;
    ca.safe_needed = safe_needed;
    }
#[no_mangle]
pub unsafe extern "C" fn chain_alloc(ca: *mut chain_allocator, size: c_uint) -> *mut c_void {
pub static mut ret: *mut c_void = core::ptr::null_mut();
    if (LINKED_PAGE_DATA_SIZE - ca.used_space < size) {
pub static mut lp: *mut c_void = core::ptr::null_mut();
    lp = ca.safe_needed ? __get_safe_page(ca.gfp_mask) :
    get_image_page(ca.gfp_mask, PG_ANY);
    if (!lp) {
    return core::ptr::null_mut();
    }
    lp.next = ca.chain;
    ca.chain = lp;
    ca.used_space = 0;
    }
    ret = ca.chain.data + ca.used_space;
    ca.used_space += size;
    return ret;
    }
//
// Data types related to memory bitmaps.
//
// Memory bitmap is a structure consisting of many linked lists of
// objects.  The main list's elements are of type struct zone_bitmap
// and each of them corresponds to one zone.  For each zone bitmap
// object there is a list of objects of type struct bm_block that
// represent each blocks of bitmap in which information is stored.
//
// struct memory_bitmap contains a pointer to the main list of zone
// bitmap objects, a struct bm_position used for browsing the bitmap,
// and a pointer to the list of pages used for allocating all of the
// zone bitmap objects and bitmap block objects.
//
// NOTE: It has to be possible to lay out the bitmap in memory
// using only allocations of order 0.  Additionally, the bitmap is
// designed to work with arbitrary number of zones (this is over the
// top for now, but let's avoid making unnecessary assumptions ;-).
//
// struct zone_bitmap contains a pointer to a list of bitmap block
// objects and a pointer to the bitmap block object that has been
// most recently used for setting bits.  Additionally, it contains the
// PFNs that correspond to the start and end of the represented zone.
//
// struct bm_block contains a pointer to the memory page in which
// information is stored (in the form of a block of bitmap)
// It also contains the pfns that correspond to the start and end of
// the represented memory area.
//
// The memory bitmap is organized as a radix tree to guarantee fast random
// access to the bits. There is one radix tree for each zone (as returned
// from create_mem_extents).
//
// One radix tree is represented by one struct mem_zone_bm_rtree. There are
// two linked lists for the nodes of the tree, one for the inner nodes and
// one for the leaf nodes. The linked leaf nodes are used for fast linear
// access of the memory bitmap.
//
// The struct rtree_node represents one node of the radix tree.
//

//
// struct rtree_node is a wrapper struct to link the nodes
// of the rtree together for easy linear iteration over
// bits and easy freeing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtree_node {
    pub list: list_head,
    pub data: *mut c_ulong,
}

//
// struct mem_zone_bm_rtree represents a bitmap used for one
// populated memory zone.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_zone_bm_rtree {
//     pub /: *mut *mut list_head list; / Link Zones together,
//     pub /: *mut *mut list_head nodes; / Radix Tree inner nodes,
//     pub /: *mut *mut list_head leaves; / Radix Tree leaves,
//     pub /: *mut *mut unsigned long start_pfn; / Zone start page frame,
//     pub /: *mut *mut unsigned long end_pfn; / Zone end page frame + 1,
//     pub /: *mut *mut *mut rtree_node rtree; / Radix Tree Root,
//     pub /: *mut *mut int levels; / Number of Radix Tree Levels,
//     pub /: *mut *mut unsigned int blocks; / Number of Bitmap Blocks,
}

// struct bm_position is used for browsing memory bitmaps
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bm_position {
    pub zone: *mut mem_zone_bm_rtree,
    pub node: *mut rtree_node,
    pub node_pfn: c_ulong,
    pub cur_pfn: c_ulong,
    pub node_bit: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memory_bitmap {
    pub zones: list_head,
    pub zone: *mut *mut *mut linked_page p_list; / list of pages used to store,
    bitmap objects and bitmap block
    objects */
//     pub /: *mut *mut bm_position cur; / most recently used bit position,
}

// Functions that operate on memory bitmaps

//
// alloc_rtree_node - Allocate a new node and add it to the radix tree.
// @gfp_mask: GFP mask for the allocation.
// @safe_needed: Get pages not used before hibernation (restore only)
// @ca: Pointer to a linked list of pages ("a chain") to allocate from
// @list: Radix Tree node to add.
//
// This function is used to allocate inner nodes as well as the
// leave nodes of the radix tree. It also adds the node to the
// corresponding linked list passed in by the *list parameter.
//
#[no_mangle]
pub unsafe extern "C" fn alloc_rtree_node(gfp_mask: gfp_t, safe_needed: c_int, ca: *mut chain_allocator, list: *mut list_head) -> *mut c_void {
pub static mut node: *mut c_void = core::ptr::null_mut();
    node = chain_alloc(ca, sizeof!(rtree_node));
    if (!node) {
    return core::ptr::null_mut();
    }
    node.data = get_image_page(gfp_mask, safe_needed);
    if (!node.data) {
    return core::ptr::null_mut();
    }
    list_add_tail(&node.list, list);
    return node;
    }
//
// add_rtree_block - Add a new leave node to the radix tree.
//
// The leave nodes need to be allocated in order to keep the leaves
// linked list in order. This is guaranteed by the zone->blocks
// counter.
//
#[no_mangle]
pub unsafe extern "C" fn add_rtree_block(zone: *mut mem_zone_bm_rtree, gfp_mask: gfp_t, safe_needed: c_int, ca: *mut chain_allocator) -> c_int {
    let mut node = core::ptr::null_mut();
    let mut block = core::ptr::null_mut();
    let mut dst = core::ptr::null_mut();
    let mut levels_needed = 0;
    let mut block_nr = 0;
    let mut i = 0;
    block_nr = zone.blocks;
    levels_needed = 0;
// How many levels do we need for this block nr?
    while (block_nr) {
    levels_needed += 1;
    block_nr >>= BM_RTREE_LEVEL_SHIFT;
    }
// Make sure the rtree has enough levels
    while (i < levels_needed) {
    node = alloc_rtree_node(gfp_mask, safe_needed, ca,
    &zone.nodes);
    if (!node) {
    return -ENOMEM;
    }
    node.data[0] = (unsigned long)zone.rtree;
    zone.rtree = node;
    zone.levels += 1;
    }
// Allocate new block
    block = alloc_rtree_node(gfp_mask, safe_needed, ca, &zone.leaves);
    if (!block) {
    return -ENOMEM;
    }
// Now walk the rtree to insert the block
    node = zone.rtree;
    dst = &zone.rtree;
    block_nr = zone.blocks;
    while (i > 0) {
    let mut index = 0;
    if (!node) {
    node = alloc_rtree_node(gfp_mask, safe_needed, ca,
    &zone.nodes);
    if (!node) {
    return -ENOMEM;
    }
// dst = node;
    }
    index = block_nr >> ((i - 1) * BM_RTREE_LEVEL_SHIFT);
    index &= BM_RTREE_LEVEL_MASK;
    dst = &((*dst).data[index]);
    node = *dst;
    }
    zone.blocks += 1;
// dst = block;
    return 0;
    }
// forward_decl: free_zone_bm_rtree;
//
// create_zone_bm_rtree - Create a radix tree for one zone.
//
// Allocated the mem_zone_bm_rtree structure and initializes it.
// This function also allocated and builds the radix tree for the
// zone.
//
#[no_mangle]
pub unsafe extern "C" fn create_zone_bm_rtree(gfp_mask: gfp_t, safe_needed: c_int, ca: *mut chain_allocator, start: c_ulong, end: c_ulong) -> *mut c_void {
pub static mut zone: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut nr_blocks = 0;
    let mut pages = 0;
    pages = end - start;
    zone  = chain_alloc(ca, sizeof!(mem_zone_bm_rtree));
    if (!zone) {
    return core::ptr::null_mut();
    }
    INIT_LIST_HEAD(&zone.nodes);
    INIT_LIST_HEAD(&zone.leaves);
    zone.start_pfn = start;
    zone.end_pfn = end;
    nr_blocks = DIV_ROUND_UP(pages, BM_BITS_PER_BLOCK);
    while (i < nr_blocks) {
    if (add_rtree_block(zone, gfp_mask, safe_needed, ca)) {
    free_zone_bm_rtree(zone, PG_UNSAFE_CLEAR);
    return core::ptr::null_mut();
    }
    }
    return zone;
    }
//
// free_zone_bm_rtree - Free the memory of the radix tree.
//
// Free all node pages of the radix tree. The mem_zone_bm_rtree
// structure itself is not freed here nor are the rtree_node
// structs.
//
#[no_mangle]
pub unsafe extern "C" fn free_zone_bm_rtree(zone: *mut mem_zone_bm_rtree, clear_nosave_free: c_int) {
pub static mut node: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(node, &zone.nodes, list) {
    free_image_page(node.data, clear_nosave_free);
    }
    list_for_each_entry(node, &zone.leaves, list) {
    free_image_page(node.data, clear_nosave_free);
    }
    }
#[no_mangle]
unsafe extern "C" fn memory_bm_position_reset(bm: *mut memory_bitmap) {
    bm.cur.zone = list_entry(bm.zones.next, mem_zone_bm_rtree,
    list);
    bm.cur.node = list_entry(bm.cur.zone.leaves.next, rtree_node, list);
    bm.cur.node_pfn = 0;
    bm.cur.cur_pfn = BM_END_OF_MAP;
    bm.cur.node_bit = 0;
    }
// forward_decl: memory_bm_free;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_extent {
    pub hook: list_head,
    pub start: c_ulong,
    pub end: c_ulong,
}

//
// free_mem_extents - Free a list of memory extents.
// @list: List of extents to free.
//
#[no_mangle]
unsafe extern "C" fn free_mem_extents(list: *mut list_head) {
    let mut ext = core::ptr::null_mut();
    let mut aux = core::ptr::null_mut();
    list_for_each_entry_safe(ext, aux, list, hook) {
    list_del(&ext.hook);
    kfree(ext);
    }
    }
//
// create_mem_extents - Create a list of memory extents.
// @list: List to put the extents into.
// @gfp_mask: Mask to use for memory allocations.
//
// The extents represent contiguous ranges of PFNs.
//
#[no_mangle]
unsafe extern "C" fn create_mem_extents(list: *mut list_head, gfp_mask: gfp_t) -> c_int {
pub static mut zone: *mut c_void = core::ptr::null_mut();
    INIT_LIST_HEAD(list);
    for_each_populated_zone(zone) {
    unsigned long zone_start, zone_end;
    let mut ext = core::ptr::null_mut();
    let mut cur = core::ptr::null_mut();
    let mut aux = core::ptr::null_mut();
    zone_start = zone.zone_start_pfn;
    zone_end = zone_end_pfn(zone);
    list_for_each_entry(ext, list, hook) {
    if (zone_start <= ext.end)
    break;
    }
    if (&ext.hook == list || zone_end < ext.start) {
// New extent is necessary
pub static mut new_ext: *mut c_void = core::ptr::null_mut();
    new_ext = kzalloc_obj(mem_extent, gfp_mask);
    if (!new_ext) {
    free_mem_extents(list);
    return -ENOMEM;
    }
    new_ext.start = zone_start;
    new_ext.end = zone_end;
    list_add_tail(&new_ext.hook, &ext.hook);
    continue;
    }
// Merge this zone's range of PFNs with the existing one
    if (zone_start < ext.start) {
    ext.start = zone_start;
    }
    if (zone_end > ext.end) {
    ext.end = zone_end;
    }
// More merging may be possible
    cur = ext;
    list_for_each_entry_safe_continue(cur, aux, list, hook) {
    if (zone_end < cur.start) {
    break;
    }
    if (zone_end < cur.end) {
    ext.end = cur.end;
    }
    list_del(&cur.hook);
    kfree(cur);
    }
    }
    return 0;
    }
//
// memory_bm_create - Allocate memory for a memory bitmap.
//
#[no_mangle]
pub unsafe extern "C" fn memory_bm_create(bm: *mut memory_bitmap, gfp_mask: gfp_t, safe_needed: c_int) -> c_int {
pub static mut ca: usize = 0;
pub static mut mem_extents: usize = 0;
pub static mut ext: *mut c_void = core::ptr::null_mut();
    let mut error = 0;
    chain_init(&ca, gfp_mask, safe_needed);
    INIT_LIST_HEAD(&bm.zones);
    error = create_mem_extents(&mem_extents, gfp_mask);
    if (error) {
    return error;
    }
    list_for_each_entry(ext, &mem_extents, hook) {
pub static mut zone: *mut c_void = core::ptr::null_mut();
    zone = create_zone_bm_rtree(gfp_mask, safe_needed, &ca,
    ext.start, ext.end);
    if (!zone) {
    error = -ENOMEM;
// goto;
    }
    list_add_tail(&zone.list, &bm.zones);
    }
    bm.p_list = ca.chain;
    memory_bm_position_reset(bm);
// label;
    free_mem_extents(&mem_extents);
    return error;
// label;
    bm.p_list = ca.chain;
    memory_bm_free(bm, PG_UNSAFE_CLEAR);
// goto;
    }
//
// memory_bm_free - Free memory occupied by the memory bitmap.
// @bm: Memory bitmap.
//
#[no_mangle]
unsafe extern "C" fn memory_bm_free(bm: *mut memory_bitmap, clear_nosave_free: c_int) {
pub static mut zone: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(zone, &bm.zones, list) {
    free_zone_bm_rtree(zone, clear_nosave_free);
    }
    free_list_of_pages(bm.p_list, clear_nosave_free);
    INIT_LIST_HEAD(&bm.zones);
    }
//
// memory_bm_find_bit - Find the bit for a given PFN in a memory bitmap.
//
// Find the bit in memory bitmap @bm that corresponds to the given PFN.
// The cur.zone, cur.block and cur.node_pfn members of @bm are updated.
//
// Walk the radix tree to find the page containing the bit that represents @pfn
// and return the position of the bit in @addr and @bit_nr.
//
#[no_mangle]
pub unsafe extern "C" fn memory_bm_find_bit(bm: *mut memory_bitmap, pfn: c_ulong, addr: *mut *mut c_void, bit_nr: *mut c_uint) -> c_int {
    let mut curr = core::ptr::null_mut();
    let mut zone = core::ptr::null_mut();
pub static mut node: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut block_nr = 0;
    zone = bm.cur.zone;
    if (pfn >= zone.start_pfn && pfn < zone.end_pfn) {
// goto;
    }
    zone = core::ptr::null_mut();
// Find the right zone
    list_for_each_entry(curr, &bm.zones, list) {
    if (pfn >= curr.start_pfn && pfn < curr.end_pfn) {
    zone = curr;
    break;
    }
    }
    if (!zone) {
    return -EFAULT;
    }
// label;
//
// We have found the zone. Now walk the radix tree to find the leaf node
// for our PFN.
//
// If the zone we wish to scan is the current zone and the
// pfn falls into the current node then we do not need to walk
// the tree.
//
    node = bm.cur.node;
    if (zone == bm.cur.zone &&
    ((pfn - zone.start_pfn) & ~BM_BLOCK_MASK) == bm.cur.node_pfn) {
// goto;
    }
    node      = zone.rtree;
    block_nr  = (pfn - zone.start_pfn) >> BM_BLOCK_SHIFT;
    while (i > 0) {
    let mut index = 0;
    index = block_nr >> ((i - 1) * BM_RTREE_LEVEL_SHIFT);
    index &= BM_RTREE_LEVEL_MASK;
    BUG_ON!(node.data[index] == 0);
    node = node.data[index];
    }
// label;
// Update last position
    bm.cur.zone = zone;
    bm.cur.node = node;
    bm.cur.node_pfn = (pfn - zone.start_pfn) & ~BM_BLOCK_MASK;
    bm.cur.cur_pfn = pfn;
// Set return values
// addr = node->data;
// bit_nr = (pfn - zone->start_pfn) & BM_BLOCK_MASK;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn memory_bm_set_bit(bm: *mut memory_bitmap, pfn: c_ulong) {
pub static mut addr: *mut c_void = core::ptr::null_mut();
    let mut bit = 0;
    let mut error = 0;
    error = memory_bm_find_bit(bm, pfn, &addr, &bit);
    BUG_ON!(error);
    set_bit(bit, addr);
    }
#[no_mangle]
unsafe extern "C" fn mem_bm_set_bit_check(bm: *mut memory_bitmap, pfn: c_ulong) -> c_int {
pub static mut addr: *mut c_void = core::ptr::null_mut();
    let mut bit = 0;
    let mut error = 0;
    error = memory_bm_find_bit(bm, pfn, &addr, &bit);
    if (!error) {
    set_bit(bit, addr);
    }
    return error;
    }
#[no_mangle]
unsafe extern "C" fn memory_bm_clear_bit(bm: *mut memory_bitmap, pfn: c_ulong) {
pub static mut addr: *mut c_void = core::ptr::null_mut();
    let mut bit = 0;
    let mut error = 0;
    error = memory_bm_find_bit(bm, pfn, &addr, &bit);
    BUG_ON!(error);
    clear_bit(bit, addr);
    }
#[no_mangle]
unsafe extern "C" fn memory_bm_clear_current(bm: *mut memory_bitmap) {
    let mut bit = 0;
    bit = max(bm.cur.node_bit - 1, 0);
    clear_bit(bit, bm.cur.node.data);
    }
#[no_mangle]
unsafe extern "C" fn memory_bm_get_current(bm: *mut memory_bitmap) -> c_ulong {
    return bm.cur.cur_pfn;
    }
#[no_mangle]
unsafe extern "C" fn memory_bm_test_bit(bm: *mut memory_bitmap, pfn: c_ulong) -> c_int {
pub static mut addr: *mut c_void = core::ptr::null_mut();
    let mut bit = 0;
    let mut error = 0;
    error = memory_bm_find_bit(bm, pfn, &addr, &bit);
    BUG_ON!(error);
    return test_bit(bit, addr);
    }
#[no_mangle]
unsafe extern "C" fn memory_bm_pfn_present(bm: *mut memory_bitmap, pfn: c_ulong) -> bool {
pub static mut addr: *mut c_void = core::ptr::null_mut();
    let mut bit = 0;
    return !memory_bm_find_bit(bm, pfn, &addr, &bit);
    }
//
// rtree_next_node - Jump to the next leaf node.
//
// Set the position to the beginning of the next node in the
// memory bitmap. This is either the next node in the current
// zone's radix tree or the first node in the radix tree of the
// next zone.
//
// Return true if there is a next node, false otherwise.
//
#[no_mangle]
unsafe extern "C" fn rtree_next_node(bm: *mut memory_bitmap) -> bool {
    if (!list_is_last(&bm.cur.node.list, &bm.cur.zone.leaves)) {
    bm.cur.node = list_entry(bm.cur.node.list.next, rtree_node, list);
    bm.cur.node_pfn += BM_BITS_PER_BLOCK;
    bm.cur.node_bit  = 0;
    touch_softlockup_watchdog();
    return true;
    }
// No more nodes, goto next zone
    if (!list_is_last(&bm.cur.zone.list, &bm.zones)) {
    bm.cur.zone = list_entry(bm.cur.zone.list.next, mem_zone_bm_rtree, list);
    bm.cur.node = list_entry(bm.cur.zone.leaves.next, rtree_node, list);
    bm.cur.node_pfn = 0;
    bm.cur.node_bit = 0;
    return true;
    }
// No more zones
    return false;
    }
//
// memory_bm_next_pfn - Find the next set bit in a memory bitmap.
// @bm: Memory bitmap.
//
// Starting from the last returned position this function searches for the next
// set bit in @bm and returns the PFN represented by it.  If no more bits are
// set, BM_END_OF_MAP is returned.
//
// It is required to run memory_bm_position_reset() before the first call to
// this function for the given memory bitmap.
//
#[no_mangle]
unsafe extern "C" fn memory_bm_next_pfn(bm: *mut memory_bitmap) -> c_ulong {
    unsigned long bits, pfn, pages;
    let mut bit = 0;
    do {
    pages	  = bm.cur.zone.end_pfn - bm.cur.zone.start_pfn;
    bits      = min(pages - bm.cur.node_pfn, BM_BITS_PER_BLOCK);
    bit	  = find_next_bit(bm.cur.node.data, bits,
    bm.cur.node_bit);
    if (bit < bits) {
    pfn = bm.cur.zone.start_pfn + bm.cur.node_pfn + bit;
    bm.cur.node_bit = bit + 1;
    bm.cur.cur_pfn = pfn;
    return pfn;
    }
    } while (rtree_next_node(bm));
    bm.cur.cur_pfn = BM_END_OF_MAP;
    return BM_END_OF_MAP;
    }
//
// This structure represents a range of page frames the contents of which
// should not be saved during hibernation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nosave_region {
    pub list: list_head,
    pub start_pfn: c_ulong,
    pub end_pfn: c_ulong,
}

pub static mut nosave_regions: usize = 0;
#[no_mangle]
unsafe extern "C" fn recycle_zone_bm_rtree(zone: *mut mem_zone_bm_rtree) {
pub static mut node: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(node, &zone.nodes, list) {
    recycle_safe_page(node.data);
    }
    list_for_each_entry(node, &zone.leaves, list) {
    recycle_safe_page(node.data);
    }
    }
#[no_mangle]
unsafe extern "C" fn memory_bm_recycle(bm: *mut memory_bitmap) {
pub static mut zone: *mut c_void = core::ptr::null_mut();
pub static mut p_list: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(zone, &bm.zones, list) {
    recycle_zone_bm_rtree(zone);
    }
    p_list = bm.p_list;
    while (p_list) {
    let mut lp = p_list;
    p_list = lp.next;
    recycle_safe_page(lp);
    }
    }
//
// register_nosave_region - Register a region of unsaveable memory.
//
// Register a range of page frames the contents of which should not be saved
// during hibernation (to be used in the early initialization code).
//
#[no_mangle]
pub unsafe extern "C" fn register_nosave_region(start_pfn: c_ulong, end_pfn: c_ulong)  {
pub static mut region: *mut c_void = core::ptr::null_mut();
    if (start_pfn >= end_pfn) {
    return;
    }
    if (!list_empty(&nosave_regions)) {
// Try to extend the previous region (they should be sorted)
    region = list_entry(nosave_regions.prev, nosave_region, list);
    if (region.end_pfn == start_pfn) {
    region.end_pfn = end_pfn;
// goto;
    }
    }
// This allocation cannot fail
    region = memblock_alloc_or_panic(sizeof!(nosave_region),
    SMP_CACHE_BYTES);
    region.start_pfn = start_pfn;
    region.end_pfn = end_pfn;
    list_add_tail(&region.list, &nosave_regions);
// label;
    pr_info!("Registered nosave memory: [mem %#010llx-%#010llx]\n",
    (unsigned long long) start_pfn << PAGE_SHIFT,
    ((unsigned long long) end_pfn << PAGE_SHIFT) - 1);
    }
//
// Set bits in this map correspond to the page frames the contents of which
// should not be saved during the suspend.
//
pub static mut forbidden_pages_map: *mut c_void = core::ptr::null_mut();
// Set bits in this map correspond to free page frames.
pub static mut free_pages_map: *mut c_void = core::ptr::null_mut();
//
// Each page frame allocated for creating the image is marked by setting the
// corresponding bits in forbidden_pages_map and free_pages_map simultaneously
//
#[no_mangle]
pub unsafe extern "C" fn swsusp_set_page_free(page: *mut page) {
    if (free_pages_map) {
    memory_bm_set_bit(free_pages_map, page_to_pfn(page));
    }
    }
#[no_mangle]
unsafe extern "C" fn swsusp_page_is_free(page: *mut page) -> c_int {
    return free_pages_map ?
    memory_bm_test_bit(free_pages_map, page_to_pfn(page)) : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn swsusp_unset_page_free(page: *mut page) {
    if (free_pages_map) {
    memory_bm_clear_bit(free_pages_map, page_to_pfn(page));
    }
    }
#[no_mangle]
unsafe extern "C" fn swsusp_set_page_forbidden(page: *mut page) {
    if (forbidden_pages_map) {
    memory_bm_set_bit(forbidden_pages_map, page_to_pfn(page));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn swsusp_page_is_forbidden(page: *mut page) -> c_int {
    return forbidden_pages_map ?
    memory_bm_test_bit(forbidden_pages_map, page_to_pfn(page)) : 0;
    }
#[no_mangle]
unsafe extern "C" fn swsusp_unset_page_forbidden(page: *mut page) {
    if (forbidden_pages_map) {
    memory_bm_clear_bit(forbidden_pages_map, page_to_pfn(page));
    }
    }
//
// mark_nosave_pages - Mark pages that should not be saved.
// @bm: Memory bitmap.
//
// Set the bits in @bm that correspond to the page frames the contents of which
// should not be saved.
//
#[no_mangle]
unsafe extern "C" fn mark_nosave_pages(bm: *mut memory_bitmap) {
pub static mut region: *mut c_void = core::ptr::null_mut();
    if (list_empty(&nosave_regions)) {
    return;
    }
    list_for_each_entry(region, &nosave_regions, list) {
    let mut pfn = 0;
    pr_debug!("Marking nosave pages: [mem %#010llx-%#010llx]\n",
    (unsigned long long) region.start_pfn << PAGE_SHIFT,
    ((unsigned long long) region.end_pfn << PAGE_SHIFT)
    - 1);
    for_each_valid_pfn(pfn, region.start_pfn, region.end_pfn) {
//
// It is safe to ignore the result of
// mem_bm_set_bit_check() here, since we won't
// touch the PFNs for which the error is
// returned anyway.
//
    mem_bm_set_bit_check(bm, pfn);
    }
    }
    }
//
// create_basic_memory_bitmaps - Create bitmaps to hold basic page information.
//
// Create bitmaps needed for marking page frames that should not be saved and
// free page frames.  The forbidden_pages_map and free_pages_map pointers are
// only modified if everything goes well, because we don't want the bits to be
// touched before both bitmaps are set up.
//
#[no_mangle]
pub unsafe extern "C" fn create_basic_memory_bitmaps() -> c_int {
    let mut bm1 = core::ptr::null_mut();
    let mut bm2 = core::ptr::null_mut();
    let mut error = 0;
    if (forbidden_pages_map && free_pages_map) {
    return 0;
    }
    else {
    BUG_ON!(forbidden_pages_map || free_pages_map);
    }
    bm1 = kzalloc_obj(memory_bitmap);
    if (!bm1) {
    return -ENOMEM;
    }
    error = memory_bm_create(bm1, GFP_KERNEL, PG_ANY);
    if (error) {
// goto;
    }
    bm2 = kzalloc_obj(memory_bitmap);
    if (!bm2) {
// goto;
    }
    error = memory_bm_create(bm2, GFP_KERNEL, PG_ANY);
    if (error) {
// goto;
    }
    forbidden_pages_map = bm1;
    free_pages_map = bm2;
    mark_nosave_pages(forbidden_pages_map);
    pr_debug!("Basic memory bitmaps created\n");
    return 0;
// label;
    kfree(bm2);
// label;
    memory_bm_free(bm1, PG_UNSAFE_CLEAR);
// label;
    kfree(bm1);
    return -ENOMEM;
    }
//
// free_basic_memory_bitmaps - Free memory bitmaps holding basic information.
//
// Free memory bitmaps allocated by create_basic_memory_bitmaps().  The
// auxiliary pointers are necessary so that the bitmaps themselves are not
// referred to while they are being freed.
//
#[no_mangle]
pub unsafe extern "C" fn free_basic_memory_bitmaps() {
    let mut bm1 = core::ptr::null_mut();
    let mut bm2 = core::ptr::null_mut();
    if (WARN_ON!(!(forbidden_pages_map && free_pages_map))) {
    return;
    }
    bm1 = forbidden_pages_map;
    bm2 = free_pages_map;
    forbidden_pages_map = core::ptr::null_mut();
    free_pages_map = core::ptr::null_mut();
    memory_bm_free(bm1, PG_UNSAFE_CLEAR);
    kfree(bm1);
    memory_bm_free(bm2, PG_UNSAFE_CLEAR);
    kfree(bm2);
    pr_debug!("Basic memory bitmaps freed\n");
    }
#[no_mangle]
unsafe extern "C" fn clear_or_poison_free_page(page: *mut page) {
    if (page_poisoning_enabled_static()) {
    __kernel_poison_pages(page, 1);
    }

    else if (want_init_on_free()) {
    clear_highpage(page);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn clear_or_poison_free_pages() {
    let mut bm = free_pages_map;
    let mut pfn = 0;
    if (WARN_ON!(!(free_pages_map))) {
    return;
    }
    if (page_poisoning_enabled() || want_init_on_free()) {
    memory_bm_position_reset(bm);
    pfn = memory_bm_next_pfn(bm);
    while (pfn != BM_END_OF_MAP) {
    if (pfn_valid(pfn)) {
    clear_or_poison_free_page(pfn_to_page(pfn));
    }
    pfn = memory_bm_next_pfn(bm);
    }
    memory_bm_position_reset(bm);
    pr_info!("free pages cleared after restore\n");
    }
    }
//
// snapshot_additional_pages - Estimate the number of extra pages needed.
// @zone: Memory zone to carry out the computation for.
//
// Estimate the number of additional pages needed for setting up a hibernation
// image data structures for @zone (usually, the returned value is greater than
// the exact number).
//
#[no_mangle]
pub unsafe extern "C" fn snapshot_additional_pages(zone: *mut zone) -> c_uint {
    let mut rtree = 0;
    let mut nodes = 0;
    rtree = nodes = DIV_ROUND_UP(zone.spanned_pages, BM_BITS_PER_BLOCK);
    rtree += DIV_ROUND_UP(rtree * sizeof!(rtree_node),
    LINKED_PAGE_DATA_SIZE);
    while (nodes > 1) {
    nodes = DIV_ROUND_UP(nodes, BM_ENTRIES_PER_LEVEL);
    rtree += nodes;
    }
    return 2 * rtree;
    }
//
// Touch the watchdog for every WD_PAGE_COUNT pages.
//

#[no_mangle]
unsafe extern "C" fn mark_free_pages(zone: *mut zone) {
    unsigned long pfn, max_zone_pfn, page_count = WD_PAGE_COUNT;
pub static mut free_list: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut order = 0;
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (zone_is_empty(zone)) {
    return;
    }
    spin_lock_irqsave(&zone.lock, flags);
    max_zone_pfn = zone_end_pfn(zone);
    for_each_valid_pfn(pfn, zone.zone_start_pfn, max_zone_pfn) {
    page = pfn_to_page(pfn);
    if (!--page_count) {
    touch_nmi_watchdog();
    page_count = WD_PAGE_COUNT;
    }
    if (page_zone(page) != zone) {
    continue;
    }
    if (!swsusp_page_is_forbidden(page)) {
    swsusp_unset_page_free(page);
    }
    }
    for_each_free_list(free_list, zone, order) {
    list_for_each_entry(page, free_list, buddy_list) {
    let mut i = 0;
    pfn = page_to_pfn(page);
    while (i < (1UL << order)) {
    if (!--page_count) {
    touch_nmi_watchdog();
    page_count = WD_PAGE_COUNT;
    }
    swsusp_set_page_free(pfn_to_page(pfn + i));
    }
    }
    }
    spin_unlock_irqrestore(&zone.lock, flags);
    }

//
// count_free_highmem_pages - Compute the total number of free highmem pages.
//
// The returned number is system-wide.
//
#[no_mangle]
unsafe extern "C" fn count_free_highmem_pages() -> c_uint {
pub static mut zone: *mut c_void = core::ptr::null_mut();
pub static mut cnt: c_uint = 0;
    for_each_populated_zone(zone) {
    if (is_highmem(zone))
    cnt += zone_page_state(zone, NR_FREE_PAGES);
    }
    return cnt;
    }
//
// saveable_highmem_page - Check if a highmem page is saveable.
//
// Determine whether a highmem page should be included in a hibernation image.
//
// We should save the page if it isn't Nosave or NosaveFree, or Reserved,
// and it isn't part of a free chunk of pages.
//
#[no_mangle]
pub unsafe extern "C" fn saveable_highmem_page(zone: *mut zone, pfn: c_ulong) -> *mut c_void {
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (!pfn_valid(pfn)) {
    return core::ptr::null_mut();
    }
    page = pfn_to_online_page(pfn);
    if (!page || page_zone(page) != zone) {
    return core::ptr::null_mut();
    }
    BUG_ON!(!PageHighMem(page));
    if (swsusp_page_is_forbidden(page) ||  swsusp_page_is_free(page)) {
    return core::ptr::null_mut();
    }
    if (PageReserved(page) || PageOffline(page)) {
    return core::ptr::null_mut();
    }
    if (page_is_guard(page)) {
    return core::ptr::null_mut();
    }
    return page;
    }
//
// count_highmem_pages - Compute the total number of saveable highmem pages.
//
#[no_mangle]
unsafe extern "C" fn count_highmem_pages() -> c_uint {
pub static mut zone: *mut c_void = core::ptr::null_mut();
pub static mut n: c_uint = 0;
    for_each_populated_zone(zone) {
    unsigned long pfn, max_zone_pfn;
    if (!is_highmem(zone)) {
    continue;
    }
    mark_free_pages(zone);
    max_zone_pfn = zone_end_pfn(zone);
    for (pfn = zone.zone_start_pfn; pfn < max_zone_pfn; pfn++) {
    if (saveable_highmem_page(zone, pfn))
    n += 1;
    }
    }
    return n;
    }

//
// saveable_page - Check if the given page is saveable.
//
// Determine whether a non-highmem page should be included in a hibernation
// image.
//
// We should save the page if it isn't Nosave, and is not in the range
// of pages statically defined as 'unsaveable', and it isn't part of
// a free chunk of pages.
//
#[no_mangle]
pub unsafe extern "C" fn saveable_page(zone: *mut zone, pfn: c_ulong) -> *mut c_void {
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (!pfn_valid(pfn)) {
    return core::ptr::null_mut();
    }
    page = pfn_to_online_page(pfn);
    if (!page || page_zone(page) != zone) {
    return core::ptr::null_mut();
    }
    BUG_ON!(PageHighMem(page));
    if (swsusp_page_is_forbidden(page) || swsusp_page_is_free(page)) {
    return core::ptr::null_mut();
    }
    if (PageOffline(page)) {
    return core::ptr::null_mut();
    }
    if (PageReserved(page)
    && (!kernel_page_present(page) || pfn_is_nosave(pfn))) {
    return core::ptr::null_mut();
    }
    if (page_is_guard(page)) {
    return core::ptr::null_mut();
    }
    return page;
    }
//
// count_data_pages - Compute the total number of saveable non-highmem pages.
//
#[no_mangle]
unsafe extern "C" fn count_data_pages() -> c_uint {
pub static mut zone: *mut c_void = core::ptr::null_mut();
    unsigned long pfn, max_zone_pfn;
pub static mut n: c_uint = 0;
    for_each_populated_zone(zone) {
    if (is_highmem(zone)) {
    continue;
    }
    mark_free_pages(zone);
    max_zone_pfn = zone_end_pfn(zone);
    for (pfn = zone.zone_start_pfn; pfn < max_zone_pfn; pfn++) {
    if (saveable_page(zone, pfn))
    n += 1;
    }
    }
    return n;
    }
//
// This is needed, because copy_page and memcpy are not usable for copying
// task structs. Returns true if the page was filled with only zeros,
// otherwise false.
//
#[no_mangle]
pub unsafe extern "C" fn do_copy_page(dst: *mut c_long, src: *mut c_long) -> bool {
pub static mut z: c_long = 0;
    let mut n = 0;
    while (n) {
    z |= *src;
// dst++ = *src += 1;
    }
    return !z;
    }
//
// safe_copy_page - Copy a page in a safe way.
//
// Check if the page we are going to copy is marked as present in the kernel
// page tables. This always is the case if CONFIG_DEBUG_PAGEALLOC or
// CONFIG_ARCH_HAS_SET_DIRECT_MAP is not set. In that case kernel_page_present()
// always returns 'true'. Returns true if the page was entirely composed of
// zeros, otherwise it will return false.
//
#[no_mangle]
unsafe extern "C" fn safe_copy_page(dst: *mut c_void, s_page: *mut page) -> bool {
    let mut zeros_only = 0;
    if (kernel_page_present(s_page)) {
    zeros_only = do_copy_page(dst, page_address(s_page));
    } else {
    hibernate_map_page(s_page);
    zeros_only = do_copy_page(dst, page_address(s_page));
    hibernate_unmap_page(s_page);
    }
    return zeros_only;
    }

#[no_mangle]
pub unsafe extern "C" fn page_is_saveable(zone: *mut zone, pfn: c_ulong) -> *mut c_void {
    return is_highmem(zone) ?
    saveable_highmem_page(zone, pfn) : saveable_page(zone, pfn);
    }
#[no_mangle]
unsafe extern "C" fn copy_data_page(dst_pfn: c_ulong, src_pfn: c_ulong) -> bool {
    let mut s_page = core::ptr::null_mut();
    let mut d_page = core::ptr::null_mut();
    let mut src = core::ptr::null_mut();
    let mut dst = core::ptr::null_mut();
    let mut zeros_only = 0;
    s_page = pfn_to_page(src_pfn);
    d_page = pfn_to_page(dst_pfn);
    if (PageHighMem(s_page)) {
    src = kmap_local_page(s_page);
    dst = kmap_local_page(d_page);
    zeros_only = do_copy_page(dst, src);
    kunmap_local(dst);
    kunmap_local(src);
    } else {
    if (PageHighMem(d_page)) {
//
// The page pointed to by src may contain some kernel
// data modified by kmap_atomic()
//
    zeros_only = safe_copy_page(buffer, s_page);
    dst = kmap_local_page(d_page);
    copy_page(dst, buffer);
    kunmap_local(dst);
    } else {
    zeros_only = safe_copy_page(page_address(d_page), s_page);
    }
    }
    return zeros_only;
    }

#[no_mangle]
pub unsafe extern "C" fn copy_data_page(dst_pfn: c_ulong, src_pfn: c_ulong) -> c_int {
    return safe_copy_page(page_address(pfn_to_page(dst_pfn)),
    pfn_to_page(src_pfn));
    }

//
// Copy data pages will copy all pages into pages pulled from the copy_bm.
// If a page was entirely filled with zeros it will be marked in the zero_bm.
//
// Returns the number of pages copied.
//
#[no_mangle]
pub unsafe extern "C" fn copy_data_pages(copy_bm: *mut memory_bitmap, orig_bm: *mut memory_bitmap, zero_bm: *mut memory_bitmap) -> c_ulong {
pub static mut copied_pages: c_ulong = 0;
pub static mut zone: *mut c_void = core::ptr::null_mut();
    unsigned long pfn, copy_pfn;
    for_each_populated_zone(zone) {
    let mut max_zone_pfn = 0;
    mark_free_pages(zone);
    max_zone_pfn = zone_end_pfn(zone);
    for (pfn = zone.zone_start_pfn; pfn < max_zone_pfn; pfn++) {
    if (page_is_saveable(zone, pfn))
    memory_bm_set_bit(orig_bm, pfn);
    }
    }
    memory_bm_position_reset(orig_bm);
    memory_bm_position_reset(copy_bm);
    copy_pfn = memory_bm_next_pfn(copy_bm);
    for (;;) {
    pfn = memory_bm_next_pfn(orig_bm);
    if (unlikely(pfn == BM_END_OF_MAP)) {
    break;
    }
    if (copy_data_page(copy_pfn, pfn)) {
    memory_bm_set_bit(zero_bm, pfn);
// Use this copy_pfn for a page that is not full of zeros
    continue;
    }
    copied_pages += 1;
    copy_pfn = memory_bm_next_pfn(copy_bm);
    }
    return copied_pages;
    }
// Total number of image pages
    static unsigned int nr_copy_pages;
// Number of pages needed for saving the original pfns of the image pages
    static unsigned int nr_meta_pages;
// Number of zero pages
    static unsigned int nr_zero_pages;
//
// Numbers of normal and highmem page frames allocated for hibernation image
// before suspending devices.
//
    static unsigned int alloc_normal, alloc_highmem;
//
// Memory bitmap used for marking saveable pages (during hibernation) or
// hibernation image pages (during restore)
//
pub static mut orig_bm: usize = 0;
//
// Memory bitmap used during hibernation for marking allocated page frames that
// will contain copies of saveable pages.  During restore it is initially used
// for marking hibernation image pages, but then the set bits from it are
// duplicated in @orig_bm and it is released.  On highmem systems it is next
// used for marking "safe" highmem pages, but it has to be reinitialized for
// this purpose.
//
pub static mut copy_bm: usize = 0;
// Memory bitmap which tracks which saveable pages were zero filled.
pub static mut zero_bm: usize = 0;
//
// swsusp_free - Free pages allocated for hibernation image.
//
// Image pages are allocated before snapshot creation, so they need to be
// released after resume.
//
#[no_mangle]
pub unsafe extern "C" fn swsusp_free() {
    unsigned long fb_pfn, fr_pfn;
    if (!forbidden_pages_map || !free_pages_map) {
// goto;
    }
    memory_bm_position_reset(forbidden_pages_map);
    memory_bm_position_reset(free_pages_map);
// label;
    fr_pfn = memory_bm_next_pfn(free_pages_map);
    fb_pfn = memory_bm_next_pfn(forbidden_pages_map);
//
// Find the next bit set in both bitmaps. This is guaranteed to
// terminate when fb_pfn == fr_pfn == BM_END_OF_MAP.
//
    do {
    if (fb_pfn < fr_pfn) {
    fb_pfn = memory_bm_next_pfn(forbidden_pages_map);
    }
    if (fr_pfn < fb_pfn) {
    fr_pfn = memory_bm_next_pfn(free_pages_map);
    }
    } while (fb_pfn != fr_pfn);
    if (fr_pfn != BM_END_OF_MAP && pfn_valid(fr_pfn)) {
    let mut page = pfn_to_page(fr_pfn);
    memory_bm_clear_current(forbidden_pages_map);
    memory_bm_clear_current(free_pages_map);
    hibernate_restore_unprotect_page(page_address(page));
    __free_page(page);
// goto;
    }
// label;
    nr_copy_pages = 0;
    nr_meta_pages = 0;
    nr_zero_pages = 0;
    restore_pblist = core::ptr::null_mut();
    buffer = core::ptr::null_mut();
    alloc_normal = 0;
    alloc_highmem = 0;
    hibernate_restore_protection_end();
    }
// Helper functions used for the shrinking of memory.

//
// preallocate_image_pages - Allocate a number of pages for hibernation image.
// @nr_pages: Number of page frames to allocate.
// @mask: GFP flags to use for the allocation.
//
// Return value: Number of page frames actually allocated
//
#[no_mangle]
unsafe extern "C" fn preallocate_image_pages(nr_pages: c_ulong, mask: gfp_t) -> c_ulong {
pub static mut nr_alloc: c_ulong = 0;
    while (nr_pages > 0) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    page = alloc_image_page(mask);
    if (!page) {
    break;
    }
    memory_bm_set_bit(&copy_bm, page_to_pfn(page));
    if (PageHighMem(page)) {
    alloc_highmem += 1;
    }
    else {
    alloc_normal += 1;
    }
    nr_pages -= 1;
    nr_alloc += 1;
    }
    return nr_alloc;
    }
#[no_mangle]
pub unsafe extern "C" fn preallocate_image_memory(nr_pages: c_ulong, avail_normal: c_ulong) -> c_ulong {
    let mut alloc = 0;
    if (avail_normal <= alloc_normal) {
    return 0;
    }
    alloc = avail_normal - alloc_normal;
    if (nr_pages < alloc) {
    alloc = nr_pages;
    }
    return preallocate_image_pages(alloc, GFP_IMAGE);
    }

#[no_mangle]
unsafe extern "C" fn preallocate_image_highmem(nr_pages: c_ulong) -> c_ulong {
    return preallocate_image_pages(nr_pages, GFP_IMAGE | __GFP_HIGHMEM);
    }
//
// __fraction - Compute (an approximation of) x * (multiplier / base).
//
#[no_mangle]
unsafe extern "C" fn __fraction(x: u64, multiplier: u64, base: u64) -> c_ulong {
    return div64_u64(x * multiplier, base);
    }
#[no_mangle]
pub unsafe extern "C" fn preallocate_highmem_fraction(nr_pages: c_ulong, highmem: c_ulong, total: c_ulong) -> c_ulong {
pub static mut alloc: c_ulong = 0;
    return preallocate_image_pages(alloc, GFP_IMAGE | __GFP_HIGHMEM);
    }

#[no_mangle]
pub unsafe extern "C" fn preallocate_image_highmem(nr_pages: c_ulong) -> c_ulong {
    return 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: preallocate_highmem_fraction
pub unsafe extern "C" fn preallocate_highmem_fraction_dup(nr_pages: c_ulong, highmem: c_ulong, total: c_ulong) -> c_ulong {
    return 0;
    }

//
// free_unnecessary_pages - Release preallocated pages not needed for the image.
//
#[no_mangle]
unsafe extern "C" fn free_unnecessary_pages() -> c_ulong {
    unsigned long save, to_free_normal, to_free_highmem, free;
    save = count_data_pages();
    if (alloc_normal >= save) {
    to_free_normal = alloc_normal - save;
    save = 0;
    } else {
    to_free_normal = 0;
    save -= alloc_normal;
    }
    save += count_highmem_pages();
    if (alloc_highmem >= save) {
    to_free_highmem = alloc_highmem - save;
    } else {
    to_free_highmem = 0;
    save -= alloc_highmem;
    if (to_free_normal > save) {
    to_free_normal -= save;
    }
    else {
    to_free_normal = 0;
    }
    }
    free = to_free_normal + to_free_highmem;
    memory_bm_position_reset(&copy_bm);
    while (to_free_normal > 0 || to_free_highmem > 0) {
pub static mut pfn: c_ulong = 0;
    let mut page = pfn_to_page(pfn);
    if (PageHighMem(page)) {
    if (!to_free_highmem) {
    continue;
    }
    to_free_highmem -= 1;
    alloc_highmem -= 1;
    } else {
    if (!to_free_normal) {
    continue;
    }
    to_free_normal -= 1;
    alloc_normal -= 1;
    }
    memory_bm_clear_bit(&copy_bm, pfn);
    swsusp_unset_page_forbidden(page);
    swsusp_unset_page_free(page);
    __free_page(page);
    }
    return free;
    }
//
// minimum_image_size - Estimate the minimum acceptable size of an image.
// @saveable: Number of saveable pages in the system.
//
// We want to avoid attempting to free too much memory too hard, so estimate the
// minimum acceptable size of a hibernation image to use as the lower limit for
// preallocating memory.
//
// We assume that the minimum image size should be proportional to
//
// [number of saveable pages] - [number of pages that can be freed in theory]
//
// where the second term is the sum of (1) reclaimable slab pages, (2) active
// and (3) inactive anonymous pages, (4) active and (5) inactive file pages.
//
#[no_mangle]
unsafe extern "C" fn minimum_image_size(saveable: c_ulong) -> c_ulong {
    let mut size = 0;
    size = global_node_page_state_pages(NR_SLAB_RECLAIMABLE_B)
    + global_node_page_state(NR_ACTIVE_ANON)
    + global_node_page_state(NR_INACTIVE_ANON)
    + global_node_page_state(NR_ACTIVE_FILE)
    + global_node_page_state(NR_INACTIVE_FILE);
    return saveable <= size ? 0 : saveable - size;
    }
//
// hibernate_preallocate_memory - Preallocate memory for hibernation image.
//
// To create a hibernation image it is necessary to make a copy of every page
// frame in use.  We also need a number of page frames to be free during
// hibernation for allocations made while saving the image and for device
// drivers, in case they need to allocate memory from their hibernation
// callbacks (these two numbers are given by PAGES_FOR_IO (which is a rough
// estimate) and reserved_size divided by PAGE_SIZE (which is tunable through
// /sys/power/reserved_size, respectively).  To make this happen, we compute the
// total number of available page frames and allocate at least
//
// ([page frames total] - PAGES_FOR_IO - [metadata pages]) / 2
// - 2 * DIV_ROUND_UP(reserved_size, PAGE_SIZE)
//
// of them, which corresponds to the maximum size of a hibernation image.
//
// If image_size is set below the number following from the above formula,
// the preallocation of memory is continued until the total number of saveable
// pages in the system is below the requested image size or the minimum
// acceptable image size returned by minimum_image_size(), whichever is greater.
//
#[no_mangle]
pub unsafe extern "C" fn hibernate_preallocate_memory() -> c_int {
pub static mut zone: *mut c_void = core::ptr::null_mut();
    unsigned long saveable, size, max_size, count, highmem, pages = 0;
    unsigned long alloc, save_highmem, pages_highmem, avail_normal;
    ktime_t start, stop;
    let mut error = 0;
    pr_info!("Preallocating image memory\n");
    start = ktime_get();
    error = memory_bm_create(&orig_bm, GFP_IMAGE, PG_ANY);
    if (error) {
    pr_err!("Cannot allocate original bitmap\n");
// goto;
    }
    error = memory_bm_create(&copy_bm, GFP_IMAGE, PG_ANY);
    if (error) {
    pr_err!("Cannot allocate copy bitmap\n");
// goto;
    }
    error = memory_bm_create(&zero_bm, GFP_IMAGE, PG_ANY);
    if (error) {
    pr_err!("Cannot allocate zero bitmap\n");
// goto;
    }
    alloc_normal = 0;
    alloc_highmem = 0;
    nr_zero_pages = 0;
// Count the number of saveable data pages.
    save_highmem = count_highmem_pages();
    saveable = count_data_pages();
//
// Compute the total number of page frames we can use (count) and the
// number of pages needed for image metadata (size).
//
    count = saveable;
    saveable += save_highmem;
    highmem = save_highmem;
    size = 0;
    for_each_populated_zone(zone) {
    size += snapshot_additional_pages(zone);
    if (is_highmem(zone)) {
    highmem += zone_page_state(zone, NR_FREE_PAGES);
    }
    else {
    count += zone_page_state(zone, NR_FREE_PAGES);
    }
    }
    avail_normal = count;
    count += highmem;
    count -= totalreserve_pages;
// Compute the maximum number of saveable pages to leave in memory.
    max_size = (count - (size + PAGES_FOR_IO)) / 2
    - 2 * DIV_ROUND_UP(reserved_size, PAGE_SIZE);
// Compute the desired number of image pages specified by image_size.
    size = DIV_ROUND_UP(image_size, PAGE_SIZE);
    if (size > max_size) {
    size = max_size;
    }
//
// If the desired number of image pages is at least as large as the
// current number of saveable pages in memory, allocate page frames for
// the image and we're done.
//
    if (size >= saveable) {
    pages = preallocate_image_highmem(save_highmem);
    pages += preallocate_image_memory(saveable - pages, avail_normal);
// goto;
    }
// Estimate the minimum size of the image.
    pages = minimum_image_size(saveable);
//
// To avoid excessive pressure on the normal zone, leave room in it to
// accommodate an image of the minimum size (unless it's already too
// small, in which case don't preallocate pages from it at all).
//
    if (avail_normal > pages) {
    avail_normal -= pages;
    }
    else {
    avail_normal = 0;
    }
    if (size < pages) {
    size = min_t(unsigned long, pages, max_size);
    }
//
// Let the memory management subsystem know that we're going to need a
// large number of page frames to allocate and make it free some memory.
// NOTE: If this is not done, performance will be hurt badly in some
// test cases.
//
    shrink_all_memory(saveable - size);
//
// The number of saveable pages in memory was too high, so apply some
// pressure to decrease it.  First, make room for the largest possible
// image and fail if that doesn't work.  Next, try to decrease the size
// of the image as much as indicated by 'size' using allocations from
// highmem and non-highmem zones separately.
//
    pages_highmem = preallocate_image_highmem(highmem / 2);
    alloc = count - max_size;
    if (alloc > pages_highmem) {
    alloc -= pages_highmem;
    }
    else {
    alloc = 0;
    }
    pages = preallocate_image_memory(alloc, avail_normal);
    if (pages < alloc) {
// We have exhausted non-highmem pages, try highmem.
    alloc -= pages;
    pages += pages_highmem;
    pages_highmem = preallocate_image_highmem(alloc);
    if (pages_highmem < alloc) {
    pr_err!("Image allocation is %lu pages short\n",
    alloc - pages_highmem);
// goto;
    }
    pages += pages_highmem;
//
// size is the desired number of saveable pages to leave in
// memory, so try to preallocate (all memory - size) pages.
//
    alloc = (count - pages) - size;
    pages += preallocate_image_highmem(alloc);
    } else {
//
// There are approximately max_size saveable pages at this point
// and we want to reduce this number down to size.
//
    alloc = max_size - size;
    size = preallocate_highmem_fraction(alloc, highmem, count);
    pages_highmem += size;
    alloc -= size;
    size = preallocate_image_memory(alloc, avail_normal);
    pages_highmem += preallocate_image_highmem(alloc - size);
    pages += pages_highmem + size;
    }
//
// We only need as many page frames for the image as there are saveable
// pages in memory, but we have allocated more.  Release the excessive
// ones now.
//
    pages -= free_unnecessary_pages();
// label;
    stop = ktime_get();
    pr_info!("Allocated %lu pages for snapshot\n", pages);
    swsusp_show_speed(start, stop, pages, "Allocated");
    return 0;
// label;
    swsusp_free();
    return -ENOMEM;
    }

//
// count_pages_for_highmem - Count non-highmem pages needed for copying highmem.
//
// Compute the number of non-highmem pages that will be necessary for creating
// copies of highmem pages.
//
#[no_mangle]
unsafe extern "C" fn count_pages_for_highmem(nr_highmem: c_uint) -> c_uint {
pub static mut free_highmem: c_uint = 0;
    if (free_highmem >= nr_highmem) {
    nr_highmem = 0;
    }
    else {
    nr_highmem -= free_highmem;
    }
    return nr_highmem;
    }

#[no_mangle]
pub unsafe extern "C" fn count_pages_for_highmem(nr_highmem: c_uint) -> c_uint { return 0; }

//
// enough_free_mem - Check if there is enough free memory for the image.
//
#[no_mangle]
unsafe extern "C" fn enough_free_mem(nr_pages: c_uint, nr_highmem: c_uint) -> c_int {
pub static mut zone: *mut c_void = core::ptr::null_mut();
pub static mut free: c_uint = 0;
    for_each_populated_zone(zone) {
    if (!is_highmem(zone))
    free += zone_page_state(zone, NR_FREE_PAGES);
    }
    nr_pages += count_pages_for_highmem(nr_highmem);
    pr_debug!("Normal pages needed: %u + %u, available pages: %u\n",
    nr_pages, PAGES_FOR_IO, free);
    return free > nr_pages + PAGES_FOR_IO;
    }

//
// get_highmem_buffer - Allocate a buffer for highmem pages.
//
// If there are some highmem pages in the hibernation image, we may need a
// buffer to copy them and/or load their data.
//
#[no_mangle]
pub unsafe extern "C" fn get_highmem_buffer(safe_needed: c_int) -> c_int {
    buffer = get_image_page(GFP_ATOMIC, safe_needed);
    return buffer ? 0 : -ENOMEM;
    }
//
// alloc_highmem_pages - Allocate some highmem pages for the image.
//
// Try to allocate as many pages as needed, but if the number of free highmem
// pages is less than that, allocate them all.
//
#[no_mangle]
pub unsafe extern "C" fn alloc_highmem_pages(bm: *mut memory_bitmap, nr_highmem: c_uint) -> c_uint {
pub static mut to_alloc: c_uint = 0;
    if (to_alloc > nr_highmem) {
    to_alloc = nr_highmem;
    }
    nr_highmem -= to_alloc;
    while (to_alloc-- > 0) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    page = alloc_image_page(__GFP_HIGHMEM|__GFP_KSWAPD_RECLAIM);
    memory_bm_set_bit(bm, page_to_pfn(page));
    }
    return nr_highmem;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: get_highmem_buffer
pub unsafe extern "C" fn get_highmem_buffer_dup(safe_needed: c_int) -> c_int { return 0; }
#[no_mangle]
#[no_mangle]
// duplicate fn: alloc_highmem_pages
pub unsafe extern "C" fn alloc_highmem_pages_dup(bm: *mut memory_bitmap, n: c_uint) -> c_uint { return 0; }

//
// swsusp_alloc - Allocate memory for hibernation image.
//
// We first try to allocate as many highmem pages as there are
// saveable highmem pages in the system.  If that fails, we allocate
// non-highmem pages for the copies of the remaining highmem ones.
//
// In this approach it is likely that the copies of highmem pages will
// also be located in the high memory, because of the way in which
// copy_data_pages() works.
//
#[no_mangle]
pub unsafe extern "C" fn swsusp_alloc(copy_bm: *mut memory_bitmap, nr_pages: c_uint, nr_highmem: c_uint) -> c_int {
    if (nr_highmem > 0) {
    if (get_highmem_buffer(PG_ANY)) {
// goto;
    }
    if (nr_highmem > alloc_highmem) {
    nr_highmem -= alloc_highmem;
    nr_pages += alloc_highmem_pages(copy_bm, nr_highmem);
    }
    }
    if (nr_pages > alloc_normal) {
    nr_pages -= alloc_normal;
    while (nr_pages-- > 0) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    page = alloc_image_page(GFP_ATOMIC);
    if (!page) {
// goto;
    }
    memory_bm_set_bit(copy_bm, page_to_pfn(page));
    }
    }
    return 0;
// label;
    swsusp_free();
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn swsusp_save() -> asmlinkage __visible int {
    let mut nr_pages = 0;
    let mut nr_highmem = 0;
    pm_deferred_pr_dbg("Creating image\n");
    drain_local_pages(core::ptr::null_mut());
    nr_pages = count_data_pages();
    nr_highmem = count_highmem_pages();
    pm_deferred_pr_dbg("Need to copy %u pages\n", nr_pages + nr_highmem);
    if (!enough_free_mem(nr_pages, nr_highmem)) {
    pm_deferred_pr_dbg("Not enough free memory for image creation\n");
    return -ENOMEM;
    }
    if (swsusp_alloc(&copy_bm, nr_pages, nr_highmem)) {
    return -ENOMEM;
    }
//
// During allocating of suspend pagedir, new cold pages may appear.
// Kill them.
//
    drain_local_pages(core::ptr::null_mut());
    nr_copy_pages = copy_data_pages(&copy_bm, &orig_bm, &zero_bm);
//
// End of critical section. From now on, we can write to memory,
// but we should not touch disk. This specially means we must _not_
// touch swap space! Except we must write out our image of course.
//
    nr_pages += nr_highmem;
// We don't actually copy the zero pages
    nr_zero_pages = nr_pages - nr_copy_pages;
    nr_meta_pages = DIV_ROUND_UP(nr_pages * sizeof!(long), PAGE_SIZE);
    pm_deferred_pr_dbg("Image created (%d pages copied, %d zero pages)\n",
    nr_copy_pages, nr_zero_pages);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn init_header_complete(info: *mut swsusp_info) -> c_int {
    memcpy(&info.uts, init_utsname(), sizeof!(new_utsname));
    info.version_code = LINUX_VERSION_CODE;
    return 0;
    }
    static const char *check_image_kernel(swsusp_info *info)
    {
    if (info.version_code != LINUX_VERSION_CODE) {
    return "kernel version";
    }
    if (strcmp(info.uts.sysname, init_utsname().sysname)) {
    return "system type";
    }
    if (strcmp(info.uts.release, init_utsname().release)) {
    return "kernel release";
    }
    if (strcmp(info.uts.version, init_utsname().version)) {
    return "version";
    }
    if (strcmp(info.uts.machine, init_utsname().machine)) {
    return "machine";
    }
    return core::ptr::null_mut();
    }

#[no_mangle]
pub unsafe extern "C" fn snapshot_get_image_size() -> c_ulong {
    return nr_copy_pages + nr_meta_pages + 1;
    }
#[no_mangle]
unsafe extern "C" fn init_header(info: *mut swsusp_info) -> c_int {
    memset(info, 0, sizeof!(swsusp_info));
    info.num_physpages = get_num_physpages();
    info.image_pages = nr_copy_pages;
    info.pages = snapshot_get_image_size();
    info.size = info.pages;
    info.size <<= PAGE_SHIFT;
    return init_header_complete(info);
    }

//
// pack_pfns - Prepare PFNs for saving.
// @bm: Memory bitmap.
// @buf: Memory buffer to store the PFNs in.
// @zero_bm: Memory bitmap containing PFNs of zero pages.
//
// PFNs corresponding to set bits in @bm are stored in the area of memory
// pointed to by @buf (1 page at a time). Pages which were filled with only
// zeros will have the highest bit set in the packed format to distinguish
// them from PFNs which will be contained in the image file.
//
#[no_mangle]
pub unsafe extern "C" fn pack_pfns(buf: *mut c_ulong, bm: *mut memory_bitmap, zero_bm: *mut memory_bitmap) {
    let mut j = 0;
    while (j < PAGE_SIZE / sizeof!(long)) {
    buf[j] = memory_bm_next_pfn(bm);
    if (unlikely(buf[j] == BM_END_OF_MAP)) {
    break;
    }
    if (memory_bm_test_bit(zero_bm, buf[j])) {
    buf[j] |= ENCODED_PFN_ZERO_FLAG;
    }
    }
    }
//
// snapshot_read_next - Get the address to read the next image page from.
// @handle: Snapshot handle to be used for the reading.
//
// On the first call, @handle should point to a zeroed snapshot_handle
// structure.  The structure gets populated then and a pointer to it should be
// passed to this function every next time.
//
// On success, the function returns a positive number.  Then, the caller
// is allowed to read up to the returned number of bytes from the memory
// location computed by the data_of() macro.
//
// The function returns 0 to indicate the end of the data stream condition,
// and negative numbers are returned on errors.  If that happens, the structure
// pointed to by @handle is not updated and should not be used any more.
//
#[no_mangle]
pub unsafe extern "C" fn snapshot_read_next(handle: *mut snapshot_handle) -> c_int {
    if (handle.cur > nr_meta_pages + nr_copy_pages) {
    return 0;
    }
    if (!buffer) {
// This makes the buffer be freed by swsusp_free()
    buffer = get_image_page(GFP_ATOMIC, PG_ANY);
    if (!buffer) {
    return -ENOMEM;
    }
    }
    if (!handle.cur) {
    let mut error = 0;
    error = init_header(buffer);
    if (error) {
    return error;
    }
    handle.buffer = buffer;
    memory_bm_position_reset(&orig_bm);
    memory_bm_position_reset(&copy_bm);
    } else if (handle.cur <= nr_meta_pages) {
    clear_page(buffer);
    pack_pfns(buffer, &orig_bm, &zero_bm);
    } else {
pub static mut page: *mut c_void = core::ptr::null_mut();
    page = pfn_to_page(memory_bm_next_pfn(&copy_bm));
    if (PageHighMem(page)) {
//
// Highmem pages are copied to the buffer,
// because we can't return with a kmapped
// highmem page (we may not be called again).
//
pub static mut kaddr: *mut c_void = core::ptr::null_mut();
    kaddr = kmap_local_page(page);
    copy_page(buffer, kaddr);
    kunmap_local(kaddr);
    handle.buffer = buffer;
    } else {
    handle.buffer = page_address(page);
    }
    }
    handle.cur += 1;
    return PAGE_SIZE;
    }
#[no_mangle]
pub unsafe extern "C" fn duplicate_memory_bitmap(dst: *mut memory_bitmap, src: *mut memory_bitmap) {
    let mut pfn = 0;
    memory_bm_position_reset(src);
    pfn = memory_bm_next_pfn(src);
    while (pfn != BM_END_OF_MAP) {
    memory_bm_set_bit(dst, pfn);
    pfn = memory_bm_next_pfn(src);
    }
    }
//
// mark_unsafe_pages - Mark pages that were used before hibernation.
//
// Mark the pages that cannot be used for storing the image during restoration,
// because they conflict with the pages that had been used before hibernation.
//
#[no_mangle]
unsafe extern "C" fn mark_unsafe_pages(bm: *mut memory_bitmap) {
    let mut pfn = 0;
// Clear the "free"/"unsafe" bit for all PFNs
    memory_bm_position_reset(free_pages_map);
    pfn = memory_bm_next_pfn(free_pages_map);
    while (pfn != BM_END_OF_MAP) {
    memory_bm_clear_current(free_pages_map);
    pfn = memory_bm_next_pfn(free_pages_map);
    }
// Mark pages that correspond to the "original" PFNs as "unsafe"
    duplicate_memory_bitmap(free_pages_map, bm);
    allocated_unsafe_pages = 0;
    }
#[no_mangle]
unsafe extern "C" fn check_header(info: *mut swsusp_info) -> c_int {
pub static mut reason: *mut c_void = core::ptr::null_mut();
    reason = check_image_kernel(info);
    if (!reason && info.num_physpages != get_num_physpages()) {
    reason = "memory size";
    }
    if (reason) {
    pr_err!("Image mismatch: %s\n", reason);
    return -EPERM;
    }
    return 0;
    }
//
// load_header - Check the image header and copy the data from it.
//
#[no_mangle]
unsafe extern "C" fn load_header(info: *mut swsusp_info) -> c_int {
    let mut error = 0;
    restore_pblist = core::ptr::null_mut();
    error = check_header(info);
    if (!error) {
    nr_copy_pages = info.image_pages;
    nr_meta_pages = info.pages - info.image_pages - 1;
    }
    return error;
    }
//
// unpack_orig_pfns - Set bits corresponding to given PFNs in a memory bitmap.
// @bm: Memory bitmap.
// @buf: Area of memory containing the PFNs.
// @zero_bm: Memory bitmap with the zero PFNs marked.
//
// For each element of the array pointed to by @buf (1 page at a time), set the
// corresponding bit in @bm. If the page was originally populated with only
// zeros then a corresponding bit will also be set in @zero_bm.
//
#[no_mangle]
pub unsafe extern "C" fn unpack_orig_pfns(buf: *mut c_ulong, bm: *mut memory_bitmap, zero_bm: *mut memory_bitmap) -> c_int {
    let mut decoded_pfn = 0;
    let mut zero = 0;
    let mut j = 0;
    while (j < PAGE_SIZE / sizeof!(long)) {
    if (unlikely(buf[j] == BM_END_OF_MAP)) {
    break;
    }
    zero = !!(buf[j] & ENCODED_PFN_ZERO_FLAG);
    decoded_pfn = buf[j] & ENCODED_PFN_MASK;
    if (pfn_valid(decoded_pfn) && memory_bm_pfn_present(bm, decoded_pfn)) {
    memory_bm_set_bit(bm, decoded_pfn);
    if (zero) {
    memory_bm_set_bit(zero_bm, decoded_pfn);
    nr_zero_pages += 1;
    }
    } else {
    if (!pfn_valid(decoded_pfn)) {
    pr_err!(FW_BUG "Memory map mismatch at 0x%llx after hibernation\n",
    (unsigned long long)PFN_PHYS(decoded_pfn));
    }
    return -EFAULT;
    }
    }
    return 0;
    }

//
// struct highmem_pbe is used for creating the list of highmem pages that
// should be restored atomically during the resume from disk, because the page
// frames they have occupied before the suspend are in use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct highmem_pbe {
//     pub /: *mut *mut *mut page copy_page; / data is here now,
//     pub /: *mut *mut *mut page orig_page; / data was here before the suspend,
    pub next: *mut highmem_pbe,
}

//
// List of highmem PBEs needed for restoring the highmem pages that were
// allocated before the suspend and included in the suspend image, but have
// also been allocated by the "resume" kernel, so their contents cannot be
// written directly to their "original" page frames.
//
pub static mut highmem_pblist: *mut c_void = core::ptr::null_mut();
//
// count_highmem_image_pages - Compute the number of highmem pages in the image.
// @bm: Memory bitmap.
//
// The bits in @bm that correspond to image pages are assumed to be set.
//
#[no_mangle]
unsafe extern "C" fn count_highmem_image_pages(bm: *mut memory_bitmap) -> c_uint {
    let mut pfn = 0;
pub static mut cnt: c_uint = 0;
    memory_bm_position_reset(bm);
    pfn = memory_bm_next_pfn(bm);
    while (pfn != BM_END_OF_MAP) {
    if (PageHighMem(pfn_to_page(pfn))) {
    cnt += 1;
    }
    pfn = memory_bm_next_pfn(bm);
    }
    return cnt;
    }
    static unsigned int safe_highmem_pages;
pub static mut safe_highmem_bm: *mut c_void = core::ptr::null_mut();
//
// prepare_highmem_image - Allocate memory for loading highmem data from image.
// @bm: Pointer to an uninitialized memory bitmap structure.
// @nr_highmem_p: Pointer to the number of highmem image pages.
//
// Try to allocate as many highmem pages as there are highmem image pages
// (@nr_highmem_p points to the variable containing the number of highmem image
// pages).  The pages that are "safe" (ie. will not be overwritten when the
// hibernation image is restored entirely) have the corresponding bits set in
// @bm (it must be uninitialized).
//
// NOTE: This function should not be called if there are no highmem image pages.
//
#[no_mangle]
pub unsafe extern "C" fn prepare_highmem_image(bm: *mut memory_bitmap, nr_highmem_p: *mut c_uint) -> c_int {
    let mut to_alloc = 0;
    if (memory_bm_create(bm, GFP_ATOMIC, PG_SAFE)) {
    return -ENOMEM;
    }
    if (get_highmem_buffer(PG_SAFE)) {
    return -ENOMEM;
    }
    to_alloc = count_free_highmem_pages();
    if (to_alloc > *nr_highmem_p) {
    to_alloc = *nr_highmem_p;
    }
    else {
// nr_highmem_p = to_alloc;
    }
    safe_highmem_pages = 0;
    while (to_alloc-- > 0) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    page = alloc_page(__GFP_HIGHMEM);
    if (!swsusp_page_is_free(page)) {
// The page is "safe", set its bit the bitmap
    memory_bm_set_bit(bm, page_to_pfn(page));
    safe_highmem_pages += 1;
    }
// Mark the page as allocated
    swsusp_set_page_forbidden(page);
    swsusp_set_page_free(page);
    }
    memory_bm_position_reset(bm);
    safe_highmem_bm = bm;
    return 0;
    }
pub static mut last_highmem_page: *mut c_void = core::ptr::null_mut();
//
// get_highmem_page_buffer - Prepare a buffer to store a highmem image page.
//
// For a given highmem image page get a buffer that suspend_write_next() should
// return to its caller to write to.
//
// If the page is to be saved to its "original" page frame or a copy of
// the page is to be made in the highmem, @buffer is returned.  Otherwise,
// the copy of the page is to be made in normal memory, so the address of
// the copy is returned.
//
// If @buffer is returned, the caller of suspend_write_next() will write
// the page's contents to @buffer, so they will have to be copied to the
// right location on the next call to suspend_write_next() and it is done
// with the help of copy_last_highmem_page().  For this purpose, if
// @buffer is returned, @last_highmem_page is set to the page to which
// the data will have to be copied from @buffer.
//
#[no_mangle]
pub unsafe extern "C" fn get_highmem_page_buffer(page: *mut page, ca: *mut chain_allocator) -> *mut c_void {
pub static mut pbe: *mut c_void = core::ptr::null_mut();
pub static mut kaddr: *mut c_void = core::ptr::null_mut();
    if (swsusp_page_is_forbidden(page) && swsusp_page_is_free(page)) {
//
// We have allocated the "original" page frame and we can
// use it directly to store the loaded page.
//
    last_highmem_page = page;
    return buffer;
    }
//
// The "original" page frame has not been allocated and we have to
// use a "safe" page frame to store the loaded page.
//
    pbe = chain_alloc(ca, sizeof!(highmem_pbe));
    if (!pbe) {
    swsusp_free();
    return ERR_PTR(-ENOMEM);
    }
    pbe.orig_page = page;
    if (safe_highmem_pages > 0) {
pub static mut tmp: *mut c_void = core::ptr::null_mut();
// Copy of the page will be stored in high memory
    kaddr = buffer;
    tmp = pfn_to_page(memory_bm_next_pfn(safe_highmem_bm));
    safe_highmem_pages -= 1;
    last_highmem_page = tmp;
    pbe.copy_page = tmp;
    } else {
// Copy of the page will be stored in normal memory
    kaddr = __get_safe_page(ca.gfp_mask);
    if (!kaddr) {
    return ERR_PTR(-ENOMEM);
    }
    pbe.copy_page = virt_to_page(kaddr);
    }
    pbe.next = highmem_pblist;
    highmem_pblist = pbe;
    return kaddr;
    }
//
// copy_last_highmem_page - Copy most the most recent highmem image page.
//
// Copy the contents of a highmem image from @buffer, where the caller of
// snapshot_write_next() has stored them, to the right location represented by
// @last_highmem_page .
//
#[no_mangle]
unsafe extern "C" fn copy_last_highmem_page() {
    if (last_highmem_page) {
pub static mut dst: *mut c_void = core::ptr::null_mut();
    dst = kmap_local_page(last_highmem_page);
    copy_page(dst, buffer);
    kunmap_local(dst);
    last_highmem_page = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn last_highmem_page_copied() -> c_int {
    return !last_highmem_page;
    }
#[no_mangle]
pub unsafe extern "C" fn free_highmem_data() {
    if (safe_highmem_bm) {
    memory_bm_free(safe_highmem_bm, PG_UNSAFE_CLEAR);
    }
    if (buffer) {
    free_image_page(buffer, PG_UNSAFE_CLEAR);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn count_highmem_image_pages(bm: *mut memory_bitmap) -> c_uint { return 0; }
#[no_mangle]
#[no_mangle]
// duplicate fn: prepare_highmem_image
pub unsafe extern "C" fn prepare_highmem_image_dup(bm: *mut memory_bitmap, nr_highmem_p: *mut c_uint) -> c_int { return 0; }
#[no_mangle]
#[no_mangle]
// duplicate fn: get_highmem_page_buffer
pub unsafe extern "C" fn get_highmem_page_buffer_dup(page: *mut page, ca: *mut chain_allocator) -> *mut c_void {
    return ERR_PTR(-EINVAL);
    }
#[no_mangle]
pub unsafe extern "C" fn copy_last_highmem_page() {}
#[no_mangle]
#[no_mangle]
// duplicate fn: last_highmem_page_copied
pub unsafe extern "C" fn last_highmem_page_copied_dup() -> c_int { return 1; }
#[no_mangle]
#[no_mangle]
// duplicate fn: free_highmem_data
pub unsafe extern "C" fn free_highmem_data_dup() {}

//
// prepare_image - Make room for loading hibernation image.
// @new_bm: Uninitialized memory bitmap structure.
// @bm: Memory bitmap with unsafe pages marked.
// @zero_bm: Memory bitmap containing the zero pages.
//
// Use @bm to mark the pages that will be overwritten in the process of
// restoring the system memory state from the suspend image ("unsafe" pages)
// and allocate memory for the image.
//
// The idea is to allocate a new memory bitmap first and then allocate
// as many pages as needed for image data, but without specifying what those
// pages will be used for just yet.  Instead, we mark them all as allocated and
// create a lists of "safe" pages to be used later.  On systems with high
// memory a list of "safe" highmem pages is created too.
//
// Because it was not known which pages were unsafe when @zero_bm was created,
// make a copy of it and recreate it within safe pages.
//
#[no_mangle]
pub unsafe extern "C" fn prepare_image(new_bm: *mut memory_bitmap, bm: *mut memory_bitmap, zero_bm: *mut memory_bitmap) -> c_int {
    let mut nr_pages = 0;
    let mut nr_highmem = 0;
pub static mut tmp: usize = 0;
pub static mut lp: *mut c_void = core::ptr::null_mut();
    let mut error = 0;
// If there is no highmem, the buffer will not be necessary
    free_image_page(buffer, PG_UNSAFE_CLEAR);
    buffer = core::ptr::null_mut();
    nr_highmem = count_highmem_image_pages(bm);
    mark_unsafe_pages(bm);
    error = memory_bm_create(new_bm, GFP_ATOMIC, PG_SAFE);
    if (error) {
// goto;
    }
    duplicate_memory_bitmap(new_bm, bm);
    memory_bm_free(bm, PG_UNSAFE_KEEP);
// Make a copy of zero_bm so it can be created in safe pages
    error = memory_bm_create(&tmp, GFP_ATOMIC, PG_SAFE);
    if (error) {
// goto;
    }
    duplicate_memory_bitmap(&tmp, zero_bm);
    memory_bm_free(zero_bm, PG_UNSAFE_KEEP);
// Recreate zero_bm in safe pages
    error = memory_bm_create(zero_bm, GFP_ATOMIC, PG_SAFE);
    if (error) {
// goto;
    }
    duplicate_memory_bitmap(zero_bm, &tmp);
    memory_bm_free(&tmp, PG_UNSAFE_CLEAR);
// At this point zero_bm is in safe pages and it can be used for restoring.
    if (nr_highmem > 0) {
    error = prepare_highmem_image(bm, &nr_highmem);
    if (error) {
// goto;
    }
    }
//
// Reserve some safe pages for potential later use.
//
// NOTE: This way we make sure there will be enough safe pages for the
// chain_alloc() in get_buffer().  It is a bit wasteful, but
// nr_copy_pages cannot be greater than 50% of the memory anyway.
//
// nr_copy_pages cannot be less than allocated_unsafe_pages too.
//
    nr_pages = (nr_zero_pages + nr_copy_pages) - nr_highmem - allocated_unsafe_pages;
    nr_pages = DIV_ROUND_UP(nr_pages, PBES_PER_LINKED_PAGE);
    while (nr_pages > 0) {
    lp = get_image_page(GFP_ATOMIC, PG_SAFE);
    if (!lp) {
    error = -ENOMEM;
// goto;
    }
    lp.next = safe_pages_list;
    safe_pages_list = lp;
    nr_pages -= 1;
    }
// Preallocate memory for the image
    nr_pages = (nr_zero_pages + nr_copy_pages) - nr_highmem - allocated_unsafe_pages;
    while (nr_pages > 0) {
    lp = get_zeroed_page(GFP_ATOMIC);
    if (!lp) {
    error = -ENOMEM;
// goto;
    }
    if (!swsusp_page_is_free(virt_to_page(lp))) {
// The page is "safe", add it to the list
    lp.next = safe_pages_list;
    safe_pages_list = lp;
    }
// Mark the page as allocated
    swsusp_set_page_forbidden(virt_to_page(lp));
    swsusp_set_page_free(virt_to_page(lp));
    nr_pages -= 1;
    }
    return 0;
// label;
    swsusp_free();
    return error;
    }
//
// get_buffer - Get the address to store the next image data page.
//
// Get the address that snapshot_write_next() should return to its caller to
// write to.
//
#[no_mangle]
pub unsafe extern "C" fn get_buffer(bm: *mut memory_bitmap, ca: *mut chain_allocator) -> *mut c_void {
pub static mut pbe: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut pfn: c_ulong = 0;
    if (pfn == BM_END_OF_MAP) {
    return ERR_PTR(-EFAULT);
    }
    page = pfn_to_page(pfn);
    if (PageHighMem(page)) {
    return get_highmem_page_buffer(page, ca);
    }
    if (swsusp_page_is_forbidden(page) && swsusp_page_is_free(page)) {
//
// We have allocated the "original" page frame and we can
// use it directly to store the loaded page.
//
    return page_address(page);
    }
//
// The "original" page frame has not been allocated and we have to
// use a "safe" page frame to store the loaded page.
//
    pbe = chain_alloc(ca, sizeof!(pbe));
    if (!pbe) {
    swsusp_free();
    return ERR_PTR(-ENOMEM);
    }
    pbe.orig_address = page_address(page);
    pbe.address = __get_safe_page(ca.gfp_mask);
    if (!pbe.address) {
    return ERR_PTR(-ENOMEM);
    }
    pbe.next = restore_pblist;
    restore_pblist = pbe;
    return pbe.address;
    }
//
// snapshot_write_next - Get the address to store the next image page.
// @handle: Snapshot handle structure to guide the writing.
//
// On the first call, @handle should point to a zeroed snapshot_handle
// structure.  The structure gets populated then and a pointer to it should be
// passed to this function every next time.
//
// On success, the function returns a positive number.  Then, the caller
// is allowed to write up to the returned number of bytes to the memory
// location computed by the data_of() macro.
//
// The function returns 0 to indicate the "end of file" condition.  Negative
// numbers are returned on errors, in which cases the structure pointed to by
// @handle is not updated and should not be used any more.
//
#[no_mangle]
pub unsafe extern "C" fn snapshot_write_next(handle: *mut snapshot_handle) -> c_int {
pub static mut ca: usize = 0;
    let mut error = 0;
// label;
// Check if we have already loaded the entire image
    if (handle.cur > 1 && handle.cur > nr_meta_pages + nr_copy_pages + nr_zero_pages) {
    return 0;
    }
    if (!handle.cur) {
    if (!buffer) {
// This makes the buffer be freed by swsusp_free()
    buffer = get_image_page(GFP_ATOMIC, PG_ANY);
    }
    if (!buffer) {
    return -ENOMEM;
    }
    handle.buffer = buffer;
    } else if (handle.cur == 1) {
    error = load_header(buffer);
    if (error) {
    return error;
    }
    safe_pages_list = core::ptr::null_mut();
    error = memory_bm_create(&copy_bm, GFP_ATOMIC, PG_ANY);
    if (error) {
    return error;
    }
    error = memory_bm_create(&zero_bm, GFP_ATOMIC, PG_ANY);
    if (error) {
    memory_bm_free(&copy_bm, PG_UNSAFE_CLEAR);
    return error;
    }
    nr_zero_pages = 0;
    hibernate_restore_protection_begin();
    } else if (handle.cur <= nr_meta_pages + 1) {
    error = unpack_orig_pfns(buffer, &copy_bm, &zero_bm);
    if (error) {
    return error;
    }
    if (handle.cur == nr_meta_pages + 1) {
    error = prepare_image(&orig_bm, &copy_bm, &zero_bm);
    if (error) {
    return error;
    }
    chain_init(&ca, GFP_ATOMIC, PG_SAFE);
    memory_bm_position_reset(&orig_bm);
    memory_bm_position_reset(&zero_bm);
    restore_pblist = core::ptr::null_mut();
    handle.buffer = get_buffer(&orig_bm, &ca);
    if (IS_ERR(handle.buffer)) {
    return PTR_ERR(handle.buffer);
    }
    }
    } else {
    copy_last_highmem_page();
    error = hibernate_restore_protect_page(handle.buffer);
    if (error) {
    return error;
    }
    handle.buffer = get_buffer(&orig_bm, &ca);
    if (IS_ERR(handle.buffer)) {
    return PTR_ERR(handle.buffer);
    }
    }
    handle.sync_read = (handle.buffer == buffer);
    handle.cur += 1;
// Zero pages were not included in the image, memset it and move on.
    if (handle.cur > nr_meta_pages + 1 &&
    memory_bm_test_bit(&zero_bm, memory_bm_get_current(&orig_bm))) {
    memset(handle.buffer, 0, PAGE_SIZE);
// goto;
    }
    return PAGE_SIZE;
    }
//
// snapshot_write_finalize - Complete the loading of a hibernation image.
//
// Must be called after the last call to snapshot_write_next() in case the last
// page in the image happens to be a highmem page and its contents should be
// stored in highmem.  Additionally, it recycles bitmap memory that's not
// necessary any more.
//
#[no_mangle]
pub unsafe extern "C" fn snapshot_write_finalize(handle: *mut snapshot_handle) -> c_int {
    let mut error = 0;
//
// Call snapshot_write_next() to drain any trailing zero pages,
// but make sure we're in the data page region first.
// This function can return PAGE_SIZE if the kernel was expecting
// another copy page. Return -ENODATA in that situation.
//
    if (handle.cur > nr_meta_pages + 1) {
    error = snapshot_write_next(handle);
    if (error) {
    return error > 0 ? -ENODATA : error;
    }
    }
    copy_last_highmem_page();
    error = hibernate_restore_protect_page(handle.buffer);
// Do that only if we have loaded the image entirely
    if (handle.cur > 1 && handle.cur > nr_meta_pages + nr_copy_pages + nr_zero_pages) {
    memory_bm_recycle(&orig_bm);
    free_highmem_data();
    }
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn snapshot_image_loaded(handle: *mut snapshot_handle) -> c_int {
    return !(!nr_copy_pages || !last_highmem_page_copied() ||
    handle.cur <= nr_meta_pages + nr_copy_pages + nr_zero_pages);
    }

// Assumes that @buf is ready and points to a "safe" page
#[no_mangle]
pub unsafe extern "C" fn swap_two_pages_data(p1: *mut page, p2: *mut page, buf: *mut c_void) {
    let mut kaddr1 = core::ptr::null_mut();
    let mut kaddr2 = core::ptr::null_mut();
    kaddr1 = kmap_local_page(p1);
    kaddr2 = kmap_local_page(p2);
    copy_page(buf, kaddr1);
    copy_page(kaddr1, kaddr2);
    copy_page(kaddr2, buf);
    kunmap_local(kaddr2);
    kunmap_local(kaddr1);
    }
//
// restore_highmem - Put highmem image pages into their original locations.
//
// For each highmem page that was in use before hibernation and is included in
// the image, and also has been allocated by the "restore" kernel, swap its
// current contents with the previous (ie. "before hibernation") ones.
//
// If the restore eventually fails, we can call this function once again and
// restore the highmem state as seen by the restore kernel.
//
#[no_mangle]
pub unsafe extern "C" fn restore_highmem() -> c_int {
    let mut pbe = highmem_pblist;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    if (!pbe) {
    return 0;
    }
    buf = get_image_page(GFP_ATOMIC, PG_SAFE);
    if (!buf) {
    return -ENOMEM;
    }
    while (pbe) {
    swap_two_pages_data(pbe.copy_page, pbe.orig_page, buf);
    pbe = pbe.next;
    }
    free_image_page(buf, PG_UNSAFE_CLEAR);
    return 0;
    }
//! Automatically rewritten from C to Rust
//! Source: mm/percpu-vm.c
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
// mm/percpu-vm.c - vmalloc area based chunk allocation
//
// Copyright (C) 2010		SUSE Linux Products GmbH
// Copyright (C) 2010		Tejun Heo <tj@kernel.org>
//
// Chunks are mapped into vmalloc areas and populated page by page.
// This is the default chunk allocator.
//

#[no_mangle]
pub unsafe extern "C" fn pcpu_chunk_page(chunk: *mut pcpu_chunk, cpu: c_uint, page_idx: c_int) -> *mut c_void {
// must not be used on pre-mapped chunk
    WARN_ON!(chunk.immutable);
    return vmalloc_to_page(pcpu_chunk_addr(chunk, cpu, page_idx));
    }
//
// pcpu_get_pages - get temp pages array
// @gfp: allocation flags passed to the underlying allocator, 0 to only
// return the cached array
//
// Returns pointer to array of pointers to struct page which can be indexed
// with pcpu_page_idx().  Note that there is only one array and accesses
// should be serialized by pcpu_alloc_mutex.
//
// RETURNS:
// Pointer to temp pages array on success.
//
    static struct page **pcpu_get_pages(gfp_t gfp)
    {
pub static mut pages: *mut c_void = core::ptr::null_mut();
pub static mut pages_size: usize = 0;
    lockdep_assert_held(&pcpu_alloc_mutex);
    if (!pages && gfp) {
    pages = pcpu_mem_zalloc(pages_size, gfp);
    }
    return pages;
    }
    static struct page **pcpu_get_pages_cached(void)
    {
    return pcpu_get_pages(0);
    }
//
// pcpu_free_pages - free pages which were allocated for @chunk
// @chunk: chunk pages were allocated for
// @pages: array of pages to be freed, indexed by pcpu_page_idx()
// @page_start: page index of the first page to be freed
// @page_end: page index of the last page to be freed + 1
//
// Free pages [@page_start and @page_end) in @pages for all units.
// The pages were allocated for @chunk.
//
#[no_mangle]
pub unsafe extern "C" fn pcpu_free_pages(chunk: *mut pcpu_chunk, pages: *mut *mut page, page_start: c_int, page_end: c_int) {
    let mut cpu = 0;
    let mut i = 0;
    for_each_possible_cpu(cpu) {
    while (i < page_end) {
    let mut page = pages[pcpu_page_idx(cpu, i)];
    if (page) {
    __free_page(page);
    }
    }
    }
    }
//
// pcpu_alloc_pages - allocates pages for @chunk
// @chunk: target chunk
// @pages: array to put the allocated pages into, indexed by pcpu_page_idx()
// @page_start: page index of the first page to be allocated
// @page_end: page index of the last page to be allocated + 1
// @gfp: allocation flags passed to the underlying allocator
//
// Allocate pages [@page_start,@page_end) into @pages for all units.
// The allocation is for @chunk.  Percpu core doesn't care about the
// content of @pages and will pass it verbatim to pcpu_map_pages().
//
#[no_mangle]
pub unsafe extern "C" fn pcpu_alloc_pages(chunk: *mut pcpu_chunk, pages: *mut *mut page, page_start: c_int, page_end: c_int, gfp: gfp_t) -> c_int {
    let mut cpu = 0;
    let mut tcpu = 0;
    let mut i = 0;
    gfp |= __GFP_HIGHMEM;
    for_each_possible_cpu(cpu) {
    while (i < page_end) {
    let mut pagep = &pages[pcpu_page_idx(cpu, i)];
// pagep = alloc_pages_node(cpu_to_node(cpu), gfp, 0);
    if (!*pagep) {
// goto;
    }
    }
    }
    return 0;
// label;
    while (--i >= page_start) {
    __free_page(pages[pcpu_page_idx(cpu, i)]);
    }
    for_each_possible_cpu(tcpu) {
    if (tcpu == cpu) {
    break;
    }
    for (i = page_start; i < page_end; i++) {
    __free_page(pages[pcpu_page_idx(tcpu, i)]);
    }
    }
    return -ENOMEM;
    }
//
// pcpu_pre_unmap_flush - flush cache prior to unmapping
// @chunk: chunk the regions to be flushed belongs to
// @page_start: page index of the first page to be flushed
// @page_end: page index of the last page to be flushed + 1
//
// Pages in [@page_start,@page_end) of @chunk are about to be
// unmapped.  Flush cache.  As each flushing trial can be very
// expensive, issue flush on the whole region at once rather than
// doing it for each cpu.  This could be an overkill but is more
// scalable.
//
#[no_mangle]
pub unsafe extern "C" fn pcpu_pre_unmap_flush(chunk: *mut pcpu_chunk, page_start: c_int, page_end: c_int) {
    flush_cache_vunmap(
    pcpu_chunk_addr(chunk, pcpu_low_unit_cpu, page_start),
    pcpu_chunk_addr(chunk, pcpu_high_unit_cpu, page_end));
    }
#[no_mangle]
unsafe extern "C" fn __pcpu_unmap_pages(addr: c_ulong, nr_pages: c_int) {
    vunmap_range_noflush(addr, addr + (nr_pages << PAGE_SHIFT));
    }
//
// pcpu_unmap_pages - unmap pages out of a pcpu_chunk
// @chunk: chunk of interest
// @pages: pages array which can be used to pass information to free
// @page_start: page index of the first page to unmap
// @page_end: page index of the last page to unmap + 1
//
// For each cpu, unmap pages [@page_start,@page_end) out of @chunk.
// Corresponding elements in @pages were cleared by the caller and can
// be used to carry information to pcpu_free_pages() which will be
// called after all unmaps are finished.  The caller should call
// proper pre/post flush functions.
//
#[no_mangle]
pub unsafe extern "C" fn pcpu_unmap_pages(chunk: *mut pcpu_chunk, pages: *mut *mut page, page_start: c_int, page_end: c_int) {
    let mut cpu = 0;
    let mut i = 0;
    for_each_possible_cpu(cpu) {
    while (i < page_end) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    page = pcpu_chunk_page(chunk, cpu, i);
    WARN_ON!(!page);
    pages[pcpu_page_idx(cpu, i)] = page;
    }
    __pcpu_unmap_pages(pcpu_chunk_addr(chunk, cpu, page_start),
    page_end - page_start);
    }
    }
//
// pcpu_post_unmap_tlb_flush - flush TLB after unmapping
// @chunk: pcpu_chunk the regions to be flushed belong to
// @page_start: page index of the first page to be flushed
// @page_end: page index of the last page to be flushed + 1
//
// Pages [@page_start,@page_end) of @chunk have been unmapped.  Flush
// TLB for the regions.  This can be skipped if the area is to be
// returned to vmalloc as vmalloc will handle TLB flushing lazily.
//
// As with pcpu_pre_unmap_flush(), TLB flushing also is done at once
// for the whole region.
//
#[no_mangle]
pub unsafe extern "C" fn pcpu_post_unmap_tlb_flush(chunk: *mut pcpu_chunk, page_start: c_int, page_end: c_int) {
    flush_tlb_kernel_range(
    pcpu_chunk_addr(chunk, pcpu_low_unit_cpu, page_start),
    pcpu_chunk_addr(chunk, pcpu_high_unit_cpu, page_end));
    }
#[no_mangle]
pub unsafe extern "C" fn __pcpu_map_pages(addr: c_ulong, pages: *mut *mut page, nr_pages: c_int, gfp: gfp_t) -> c_int {
    let mut flags = 0;
    let mut ret = 0;
//
// The vmalloc page table allocation path does not pass @gfp down
// explicitly.  Apply the corresponding memalloc scope so implicit
// page table allocations preserve NOFS/NOIO constraints.
//
    flags = memalloc_apply_gfp_scope(gfp);
    ret = vmap_pages_range_noflush(addr, addr + (nr_pages << PAGE_SHIFT),
    PAGE_KERNEL, pages, PAGE_SHIFT, gfp);
    memalloc_restore_scope(flags);
    return ret;
    }
//
// pcpu_map_pages - map pages into a pcpu_chunk
// @chunk: chunk of interest
// @pages: pages array containing pages to be mapped
// @page_start: page index of the first page to map
// @page_end: page index of the last page to map + 1
// @gfp: allocation flags passed to the underlying allocator
//
// For each cpu, map pages [@page_start,@page_end) into @chunk.  The
// caller is responsible for calling pcpu_post_map_flush() after all
// mappings are complete.
//
// This function is responsible for setting up whatever is necessary for
// reverse lookup (addr -> chunk).
//
#[no_mangle]
pub unsafe extern "C" fn pcpu_map_pages(chunk: *mut pcpu_chunk, pages: *mut *mut page, page_start: c_int, page_end: c_int, gfp: gfp_t) -> c_int {
    let mut cpu = 0;
    let mut tcpu = 0;
    let mut i = 0;
    let mut err = 0;
    for_each_possible_cpu(cpu) {
    err = __pcpu_map_pages(pcpu_chunk_addr(chunk, cpu, page_start),
    &pages[pcpu_page_idx(cpu, page_start)],
    page_end - page_start, gfp);
    if (err < 0) {
// goto;
    }
    for (i = page_start; i < page_end; i++) {
    pcpu_set_page_chunk(pages[pcpu_page_idx(cpu, i)],
    chunk);
    }
    }
    return 0;
// label;
    for_each_possible_cpu(tcpu) {
    __pcpu_unmap_pages(pcpu_chunk_addr(chunk, tcpu, page_start),
    page_end - page_start);
    if (tcpu == cpu) {
    break;
    }
    }
    pcpu_post_unmap_tlb_flush(chunk, page_start, page_end);
    return err;
    }
//
// pcpu_post_map_flush - flush cache after mapping
// @chunk: pcpu_chunk the regions to be flushed belong to
// @page_start: page index of the first page to be flushed
// @page_end: page index of the last page to be flushed + 1
//
// Pages [@page_start,@page_end) of @chunk have been mapped.  Flush
// cache.
//
// As with pcpu_pre_unmap_flush(), TLB flushing also is done at once
// for the whole region.
//
#[no_mangle]
pub unsafe extern "C" fn pcpu_post_map_flush(chunk: *mut pcpu_chunk, page_start: c_int, page_end: c_int) {
    flush_cache_vmap(
    pcpu_chunk_addr(chunk, pcpu_low_unit_cpu, page_start),
    pcpu_chunk_addr(chunk, pcpu_high_unit_cpu, page_end));
    }
//
// pcpu_populate_chunk - populate and map an area of a pcpu_chunk
// @chunk: chunk of interest
// @page_start: the start page
// @page_end: the end page
// @gfp: allocation flags passed to the underlying memory allocator
//
// For each cpu, populate and map pages [@page_start,@page_end) into
// @chunk.
//
// CONTEXT:
// pcpu_alloc_mutex, does @gfp allocation.
//
#[no_mangle]
pub unsafe extern "C" fn pcpu_populate_chunk(chunk: *mut pcpu_chunk, page_start: c_int, page_end: c_int, gfp: gfp_t) -> c_int {
pub static mut pages: *mut c_void = core::ptr::null_mut();
    pages = pcpu_get_pages(gfp);
    if (!pages) {
    return -ENOMEM;
    }
    if (pcpu_alloc_pages(chunk, pages, page_start, page_end, gfp)) {
    return -ENOMEM;
    }
    if (pcpu_map_pages(chunk, pages, page_start, page_end, gfp)) {
    pcpu_free_pages(chunk, pages, page_start, page_end);
    return -ENOMEM;
    }
    pcpu_post_map_flush(chunk, page_start, page_end);
    return 0;
    }
//
// pcpu_depopulate_chunk - depopulate and unmap an area of a pcpu_chunk
// @chunk: chunk to depopulate
// @page_start: the start page
// @page_end: the end page
//
// For each cpu, depopulate and unmap pages [@page_start,@page_end)
// from @chunk.
//
// Caller is required to call pcpu_post_unmap_tlb_flush() if not returning the
// region back to vmalloc() which will lazily flush the tlb.
//
// CONTEXT:
// pcpu_alloc_mutex.
//
#[no_mangle]
pub unsafe extern "C" fn pcpu_depopulate_chunk(chunk: *mut pcpu_chunk, page_start: c_int, page_end: c_int) {
pub static mut pages: *mut c_void = core::ptr::null_mut();
//
// If control reaches here, there must have been at least one
// successful population attempt so the temp pages array must
// be available now.
//
    pages = pcpu_get_pages_cached();
    BUG_ON!(!pages);
// unmap and free
    pcpu_pre_unmap_flush(chunk, page_start, page_end);
    pcpu_unmap_pages(chunk, pages, page_start, page_end);
    pcpu_free_pages(chunk, pages, page_start, page_end);
    }
#[no_mangle]
pub unsafe extern "C" fn pcpu_create_chunk(gfp: gfp_t) -> *mut c_void {
pub static mut chunk: *mut c_void = core::ptr::null_mut();
pub static mut vms: *mut c_void = core::ptr::null_mut();
    chunk = pcpu_alloc_chunk(gfp);
    if (!chunk) {
    return core::ptr::null_mut();
    }
    vms = pcpu_get_vm_areas(pcpu_group_offsets, pcpu_group_sizes,
    pcpu_nr_groups, pcpu_atom_size, gfp);
    if (!vms) {
    pcpu_free_chunk(chunk);
    return core::ptr::null_mut();
    }
    chunk.data = vms;
    chunk.base_addr = vms[0].addr - pcpu_group_offsets[0];
    pcpu_stats_chunk_alloc();
    trace_percpu_create_chunk(chunk.base_addr);
    return chunk;
    }
#[no_mangle]
unsafe extern "C" fn pcpu_destroy_chunk(chunk: *mut pcpu_chunk) {
    if (!chunk) {
    return;
    }
    pcpu_stats_chunk_dealloc();
    trace_percpu_destroy_chunk(chunk.base_addr);
    if (chunk.data) {
    pcpu_free_vm_areas(chunk.data, pcpu_nr_groups);
    }
    pcpu_free_chunk(chunk);
    }
#[no_mangle]
pub unsafe extern "C" fn pcpu_addr_to_page(addr: *mut c_void) -> *mut c_void {
    return vmalloc_to_page(addr);
    }
#[no_mangle]
unsafe extern "C" fn pcpu_verify_alloc_info(ai: *const pcpu_alloc_info) -> c_int {
// no extra restriction
    return 0;
    }
//
// pcpu_should_reclaim_chunk - determine if a chunk should go into reclaim
// @chunk: chunk of interest
//
// This is the entry point for percpu reclaim.  If a chunk qualifies, it is then
// isolated and managed in separate lists at the back of pcpu_slot: sidelined
// and to_depopulate respectively.  The to_depopulate list holds chunks slated
// for depopulation.  They no longer contribute to pcpu_nr_empty_pop_pages once
// they are on this list.  Once depopulated, they are moved onto the sidelined
// list which enables them to be pulled back in for allocation if no other chunk
// can suffice the allocation.
//
#[no_mangle]
unsafe extern "C" fn pcpu_should_reclaim_chunk(chunk: *mut pcpu_chunk) -> bool {
// do not reclaim either the first chunk or reserved chunk
    if (chunk == pcpu_first_chunk || chunk == pcpu_reserved_chunk) {
    return false;
    }
//
// If it is isolated, it may be on the sidelined list so move it back to
// the to_depopulate list.  If we hit at least 1/4 pages empty pages AND
// there is no system-wide shortage of empty pages aside from this
// chunk, move it to the to_depopulate list.
//
    return ((chunk.isolated && chunk.nr_empty_pop_pages) ||
    (pcpu_nr_empty_pop_pages >
    (PCPU_EMPTY_POP_PAGES_HIGH + chunk.nr_empty_pop_pages) &&
    chunk.nr_empty_pop_pages >= chunk.nr_pages / 4));
    }
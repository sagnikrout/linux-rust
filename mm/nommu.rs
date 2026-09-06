//! Automatically rewritten from C to Rust
//! Source: mm/nommu.c
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
// linux/mm/nommu.c
//
// Replacement code for mm functions to support CPU's that don't
// have any form of memory management unit (thus no virtual memory).
//
// See Documentation/admin-guide/mm/nommu-mmap.rst
//
// Copyright (c) 2004-2008 David Howells <dhowells@redhat.com>
// Copyright (c) 2000-2003 David McCullough <davidm@snapgear.com>
// Copyright (c) 2000-2001 D Jeff Dionne <jeff@uClinux.org>
// Copyright (c) 2002      Greg Ungerer <gerg@snapgear.com>
// Copyright (c) 2007-2010 Paul Mundt <lethal@linux-sh.org>
//

    let mut highest_memmap_pfn = 0;
pub static mut heap_stack_gap: c_int = 0;
    let mut mmap_pages_allocated;
// list of mapped, potentially shareable regions
pub static mut vm_region_jar: *mut c_void = core::ptr::null_mut();
pub static mut nommu_region_tree: rb_root = 0;
pub static mut nommu_region_sem: usize = 0;
pub static mut vm_operations_struct: usize = 0;
//
// Return the total memory allocated for this pointer, not
// just what the caller asked for.
//
// Doesn't have to be accurate, i.e. may have races.
//
#[no_mangle]
pub unsafe extern "C" fn kobjsize(objp: *const c_void) -> c_uint {
pub static mut folio: *mut c_void = core::ptr::null_mut();
//
// If the object we have should not have ksize performed on it,
// return size of 0
//
    if (!objp || !virt_addr_valid(objp)) {
    return 0;
    }
    folio = virt_to_folio(objp);
//
// If the allocator sets PageSlab, we know the pointer came from
// kmalloc().
//
    if (folio_test_slab(folio)) {
    return ksize(objp);
    }
//
// If it's not a large folio, see if we have a matching VMA
// region. This test is intentionally done in reverse order,
// so if there's no VMA, we still fall through and hand back
// PAGE_SIZE for 0-order folios.
//
    if (!folio_test_large(folio)) {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    vma = find_vma(current.mm, (unsigned long)objp);
    if (vma) {
    return vma.vm_end - vma.vm_start;
    }
    }
//
// The ksize() function is only guaranteed to work for pointers
// returned by kmalloc(). So handle arbitrary pointers here.
//
    return folio_size(folio);
    }
#[no_mangle]
pub unsafe extern "C" fn vfree(addr: *const c_void) {
    kfree(addr);
    }
    EXPORT_SYMBOL(vfree);
#[no_mangle]
pub unsafe extern "C" fn __vmalloc_noprof(size: c_ulong, gfp_mask: gfp_t) -> *mut c_void {
//
// You can't specify __GFP_HIGHMEM with kmalloc() since kmalloc()
// returns only a logical address.
//
    return kmalloc_noprof(size, (gfp_mask | __GFP_COMP) & ~__GFP_HIGHMEM);
    }
    EXPORT_SYMBOL(__vmalloc_noprof);
#[no_mangle]
pub unsafe extern "C" fn vrealloc_node_align_noprof(p: *mut c_void, size: size_t, align: c_ulong, flags: gfp_t, node: c_int) -> *mut c_void {
    return krealloc_noprof(p, size, (flags | __GFP_COMP) & ~__GFP_HIGHMEM);
    }
#[no_mangle]
pub unsafe extern "C" fn __vmalloc_node_range_noprof(size: c_ulong, align: c_ulong, start: c_ulong, end: c_ulong, gfp_mask: gfp_t, prot: pgprot_t, vm_flags: c_ulong, node: c_int, caller: *mut c_void) -> *mut c_void {
    return __vmalloc_noprof(size, gfp_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn __vmalloc_node_noprof(size: c_ulong, align: c_ulong, gfp_mask: gfp_t, node: c_int, caller: *mut c_void) -> *mut c_void {
    return __vmalloc_noprof(size, gfp_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn __vmalloc_user_flags(size: c_ulong, flags: gfp_t) -> *mut c_void {
pub static mut ret: *mut c_void = core::ptr::null_mut();
    ret = __vmalloc(size, flags);
    if (ret) {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    mmap_write_lock(current.mm);
    vma = find_vma(current.mm, (unsigned long)ret);
    if (vma) {
    vm_flags_set(vma, VM_USERMAP);
    }
    mmap_write_unlock(current.mm);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn vmalloc_user_noprof(size: c_ulong) -> *mut c_void {
    return __vmalloc_user_flags(size, GFP_KERNEL | __GFP_ZERO);
    }
    EXPORT_SYMBOL(vmalloc_user_noprof);
#[no_mangle]
pub unsafe extern "C" fn vmalloc_to_page(addr: *mut c_void) -> *mut c_void {
    return virt_to_page(addr);
    }
    EXPORT_SYMBOL(vmalloc_to_page);
#[no_mangle]
pub unsafe extern "C" fn vmalloc_to_pfn(addr: *const c_void) -> c_ulong {
    return page_to_pfn(virt_to_page(addr));
    }
    EXPORT_SYMBOL(vmalloc_to_pfn);
#[no_mangle]
pub unsafe extern "C" fn vread_iter(iter: *mut iov_iter, addr: *const c_char, count: usize) -> c_long {
// Don't allow overflow
    if ((unsigned long) addr + count < count) {
    count = -(unsigned long) addr;
    }
    return copy_to_iter(addr, count, iter);
    }
//
// vmalloc  -  allocate virtually contiguous memory
//
// @size:		allocation size
//
// Allocate enough pages to cover @size from the page level
// allocator and map them into contiguous kernel virtual space.
//
// For tight control over page level allocator and protection flags
// use __vmalloc() instead.
//
#[no_mangle]
pub unsafe extern "C" fn vmalloc_noprof(size: c_ulong) -> *mut c_void {
    return __vmalloc_noprof(size, GFP_KERNEL);
    }
    EXPORT_SYMBOL(vmalloc_noprof);
//
// vmalloc_huge_node  -  allocate virtually contiguous memory, on a node
//
// @size:		allocation size
// @gfp_mask:	flags for the page level allocator
// @node:          node to use for allocation or NUMA_NO_NODE
//
// Allocate enough pages to cover @size from the page level
// allocator and map them into contiguous kernel virtual space.
//
// Due to NOMMU implications the node argument and HUGE page attribute is
// ignored.
//
#[no_mangle]
pub unsafe extern "C" fn vmalloc_huge_node_noprof(size: c_ulong, gfp_mask: gfp_t, node: c_int) -> *mut c_void {
    return __vmalloc_noprof(size, gfp_mask);
    }
//
// vzalloc - allocate virtually contiguous memory with zero fill
//
// @size:		allocation size
//
// Allocate enough pages to cover @size from the page level
// allocator and map them into contiguous kernel virtual space.
// The memory allocated is set to zero.
//
// For tight control over page level allocator and protection flags
// use __vmalloc() instead.
//
#[no_mangle]
pub unsafe extern "C" fn vzalloc_noprof(size: c_ulong) -> *mut c_void {
    return __vmalloc_noprof(size, GFP_KERNEL | __GFP_ZERO);
    }
    EXPORT_SYMBOL(vzalloc_noprof);
//
// vmalloc_node - allocate memory on a specific node
// @size:	allocation size
// @node:	numa node
//
// Allocate enough pages to cover @size from the page level
// allocator and map them into contiguous kernel virtual space.
//
// For tight control over page level allocator and protection flags
// use __vmalloc() instead.
//
#[no_mangle]
pub unsafe extern "C" fn vmalloc_node_noprof(size: c_ulong, node: c_int) -> *mut c_void {
    return vmalloc_noprof(size);
    }
    EXPORT_SYMBOL(vmalloc_node_noprof);
//
// vzalloc_node - allocate memory on a specific node with zero fill
// @size:	allocation size
// @node:	numa node
//
// Allocate enough pages to cover @size from the page level
// allocator and map them into contiguous kernel virtual space.
// The memory allocated is set to zero.
//
// For tight control over page level allocator and protection flags
// use __vmalloc() instead.
//
#[no_mangle]
pub unsafe extern "C" fn vzalloc_node_noprof(size: c_ulong, node: c_int) -> *mut c_void {
    return vzalloc_noprof(size);
    }
    EXPORT_SYMBOL(vzalloc_node_noprof);
//
// vmalloc_32  -  allocate virtually contiguous memory (32bit addressable)
// @size:		allocation size
//
// Allocate enough 32bit PA addressable pages to cover @size from the
// page level allocator and map them into contiguous kernel virtual space.
//
#[no_mangle]
pub unsafe extern "C" fn vmalloc_32_noprof(size: c_ulong) -> *mut c_void {
    return __vmalloc_noprof(size, GFP_KERNEL);
    }
    EXPORT_SYMBOL(vmalloc_32_noprof);
//
// vmalloc_32_user - allocate zeroed virtually contiguous 32bit memory
// @size:		allocation size
//
// The resulting memory area is 32bit addressable and zeroed so it can be
// mapped to userspace without leaking data.
//
// VM_USERMAP is set on the corresponding VMA so that subsequent calls to
// remap_vmalloc_range() are permissible.
//
#[no_mangle]
pub unsafe extern "C" fn vmalloc_32_user_noprof(size: c_ulong) -> *mut c_void {
//
// We'll have to sort out the ZONE_DMA bits for 64-bit,
// but for now this can simply use vmalloc_user() directly.
//
    return vmalloc_user_noprof(size);
    }
    EXPORT_SYMBOL(vmalloc_32_user_noprof);
#[no_mangle]
pub unsafe extern "C" fn vmap(pages: *mut *mut page, count: c_uint, flags: c_ulong, prot: pgprot_t) -> *mut c_void {
    BUG();
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(vmap);
#[no_mangle]
pub unsafe extern "C" fn vunmap(addr: *const c_void) {
    BUG();
    }
    EXPORT_SYMBOL(vunmap);
#[no_mangle]
pub unsafe extern "C" fn vm_map_ram(pages: *mut *mut page, count: c_uint, node: c_int) -> *mut c_void {
    BUG();
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(vm_map_ram);
#[no_mangle]
pub unsafe extern "C" fn vm_unmap_ram(mem: *const c_void, count: c_uint) {
    BUG();
    }
    EXPORT_SYMBOL(vm_unmap_ram);
#[no_mangle]
pub unsafe extern "C" fn vm_unmap_aliases() {
    }
    EXPORT_SYMBOL_GPL(vm_unmap_aliases);
#[no_mangle]
pub unsafe extern "C" fn free_vm_area(area: *mut vm_struct) {
    BUG();
    }
    EXPORT_SYMBOL_GPL(free_vm_area);
#[no_mangle]
pub unsafe extern "C" fn vm_insert_page(vma: *mut vm_area_struct, addr: c_ulong, page: *mut page) -> c_int {
    return -EINVAL;
    }
    EXPORT_SYMBOL(vm_insert_page);
#[no_mangle]
pub unsafe extern "C" fn vm_insert_pages(vma: *mut vm_area_struct, addr: c_ulong, pages: *mut *mut page, num: *mut c_ulong) -> c_int {
    return -EINVAL;
    }
    EXPORT_SYMBOL(vm_insert_pages);
#[no_mangle]
pub unsafe extern "C" fn vm_map_pages(vma: *mut vm_area_struct, pages: *mut *mut page, num: c_ulong) -> c_int {
    return -EINVAL;
    }
    EXPORT_SYMBOL(vm_map_pages);
#[no_mangle]
pub unsafe extern "C" fn vm_map_pages_zero(vma: *mut vm_area_struct, pages: *mut *mut page, num: c_ulong) -> c_int {
    return -EINVAL;
    }
    EXPORT_SYMBOL(vm_map_pages_zero);
//
// sys_brk() for the most part doesn't need the global kernel
// lock, except when an application is doing something nasty
// like trying to un-brk an area that has already been mapped
// to a regular file.  in this case, the unmapping will need
// to invoke file system routines that need the global lock.
//
#[no_mangle]
pub unsafe extern "C" fn sys_brk(brk: usize) -> c_long {
    let mut mm = current.mm;
    if (brk < mm.start_brk || brk > mm.context.end_brk) {
    return mm.brk;
    }
    if (mm.brk == brk) {
    return mm.brk;
    }
//
// Always allow shrinking brk
//
    if (brk <= mm.brk) {
    mm.brk = brk;
    return brk;
    }
//
// Ok, looks good - let it rip.
//
    flush_icache_user_range(mm.brk, brk);
    return mm.brk = brk;
    }
pub static mut sysctl_nr_trim_pages: int = 0;
pub static mut ctl_table: usize = 0;
//
// initialise the percpu counter for VM and region record slabs, initialise VMA
// state.
//
#[no_mangle]
pub unsafe extern "C" fn mmap_init()  {
    let mut ret = 0;
    ret = percpu_counter_init(&vm_committed_as, 0, GFP_KERNEL);
    VM_BUG_ON(ret);
    vm_region_jar = KMEM_CACHE(vm_region, SLAB_PANIC|SLAB_ACCOUNT);
    register_sysctl_init("vm", nommu_table);
    vma_state_init();
    }
//
// validate the region tree
// - the caller must hold the region lock
//

#[no_mangle]
unsafe extern "C" fn validate_nommu_regions() -> noinline void {
    let mut region = core::ptr::null_mut();
    let mut last = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut lastp = core::ptr::null_mut();
    lastp = rb_first(&nommu_region_tree);
    if (!lastp) {
    return;
    }
    last = rb_entry(lastp, vm_region, vm_rb);
    BUG_ON!(last.vm_end <= last.vm_start);
    BUG_ON!(last.vm_top < last.vm_end);
    while ((p = rb_next(lastp))) {
    region = rb_entry(p, vm_region, vm_rb);
    last = rb_entry(lastp, vm_region, vm_rb);
    BUG_ON!(region.vm_end <= region.vm_start);
    BUG_ON!(region.vm_top < region.vm_end);
    BUG_ON!(region.vm_start < last.vm_top);
    lastp = p;
    }
    }

#[no_mangle]
unsafe extern "C" fn validate_nommu_regions() {
    }

//
// add a region into the global tree
//
#[no_mangle]
unsafe extern "C" fn add_nommu_region(region: *mut vm_region) {
pub static mut pregion: *mut c_void = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut parent = core::ptr::null_mut();
    validate_nommu_regions();
    parent = core::ptr::null_mut();
    p = &nommu_region_tree.rb_node;
    while (*p) {
    parent = *p;
    pregion = rb_entry(parent, vm_region, vm_rb);
    if (region.vm_start < pregion.vm_start) {
    p = &(*p).rb_left;
    }

    else if (region.vm_start > pregion.vm_start) {
    p = &(*p).rb_right;
    }

    else if (pregion == region) {
    return;
    }
    else {
    BUG();
    }
    }
    rb_link_node(&region.vm_rb, parent, p);
    rb_insert_color(&region.vm_rb, &nommu_region_tree);
    validate_nommu_regions();
    }
//
// delete a region from the global tree
//
#[no_mangle]
unsafe extern "C" fn delete_nommu_region(region: *mut vm_region) {
    BUG_ON!(!nommu_region_tree.rb_node);
    validate_nommu_regions();
    rb_erase(&region.vm_rb, &nommu_region_tree);
    validate_nommu_regions();
    }
//
// free a contiguous series of pages
//
#[no_mangle]
unsafe extern "C" fn free_page_series(from: c_ulong, to: c_ulong) {
    while (from < to) {
    let mut page = virt_to_page(from);
    atomic_long_dec(&mmap_pages_allocated);
    put_page(page);
    }
    }
//
// release a reference to a region
// - the caller must hold the region semaphore for writing, which this releases
// - the region may not have been added to the tree yet, in which case vm_top
// will equal vm_start
//
#[no_mangle]
unsafe extern "C" fn __put_nommu_region(region: *mut vm_region) {
    BUG_ON!(!nommu_region_tree.rb_node);
    if (--region.vm_usage == 0) {
    if (region.vm_top > region.vm_start) {
    delete_nommu_region(region);
    }
    up_write(&nommu_region_sem);
    if (region.vm_file) {
    fput(region.vm_file);
    }
// IO memory and memory shared directly out of the pagecache
// from ramfs/tmpfs mustn't be released here
    if (region.vm_flags & VM_MAPPED_COPY) {
    free_page_series(region.vm_start, region.vm_top);
    }
    kmem_cache_free(vm_region_jar, region);
    } else {
    up_write(&nommu_region_sem);
    }
    }
//
// release a reference to a region
//
#[no_mangle]
unsafe extern "C" fn put_nommu_region(region: *mut vm_region) {
    down_write(&nommu_region_sem);
    __put_nommu_region(region);
    }
#[no_mangle]
unsafe extern "C" fn setup_vma_to_mm(vma: *mut vm_area_struct, mm: *mut mm_struct) {
    vma.vm_mm = mm;
// add the VMA to the mapping
    if (vma.vm_file) {
    let mut mapping = vma.vm_file.f_mapping;
    i_mmap_lock_write(mapping);
    flush_dcache_mmap_lock(mapping);
    mapping_rmap_tree_insert(vma, mapping);
    flush_dcache_mmap_unlock(mapping);
    i_mmap_unlock_write(mapping);
    }
    }
#[no_mangle]
unsafe extern "C" fn cleanup_vma_from_mm(vma: *mut vm_area_struct) {
    vma.vm_mm.map_count -= 1;
// remove the VMA from the mapping
    if (vma.vm_file) {
pub static mut mapping: *mut c_void = core::ptr::null_mut();
    mapping = vma.vm_file.f_mapping;
    i_mmap_lock_write(mapping);
    flush_dcache_mmap_lock(mapping);
    mapping_rmap_tree_remove(vma, mapping);
    flush_dcache_mmap_unlock(mapping);
    i_mmap_unlock_write(mapping);
    }
    }
//
// delete a VMA from its owning mm_struct and address space
//
#[no_mangle]
unsafe extern "C" fn delete_vma_from_mm(vma: *mut vm_area_struct) -> c_int {
    VMA_ITERATOR(vmi, vma.vm_mm, vma.vm_start);
    vma_iter_config(&vmi, vma.vm_start, vma.vm_end);
    if (vma_iter_prealloc(&vmi, core::ptr::null_mut())) {
    pr_warn!("Allocation of vma tree for process %d failed\n",
    current.pid);
    return -ENOMEM;
    }
    cleanup_vma_from_mm(vma);
// remove from the MM's tree and list
    vma_iter_clear(&vmi);
    return 0;
    }
//
// destroy a VMA record
//
#[no_mangle]
unsafe extern "C" fn delete_vma(mm: *mut mm_struct, vma: *mut vm_area_struct) {
    vma_close(vma);
    if (vma.vm_file) {
    fput(vma.vm_file);
    }
    put_nommu_region(vma.vm_region);
    vm_area_free(vma);
    }
#[no_mangle]
pub unsafe extern "C" fn find_vma_intersection(mm: *mut mm_struct, start_addr: c_ulong, end_addr: c_ulong) -> *mut c_void {
pub static mut index: c_ulong = 0;
    mmap_assert_locked(mm);
    return mt_find(&mm.mm_mt, &index, end_addr - 1);
    }
    EXPORT_SYMBOL(find_vma_intersection);
//
// look up the first VMA in which addr resides, NULL if none
// - should be called with mm->mmap_lock at least held readlocked
//
#[no_mangle]
pub unsafe extern "C" fn find_vma(mm: *mut mm_struct, addr: c_ulong) -> *mut c_void {
    VMA_ITERATOR(vmi, mm, addr);
    return vma_iter_load(&vmi);
    }
    EXPORT_SYMBOL(find_vma);
//
// expand a stack to a given address
// - not supported under NOMMU conditions
//
#[no_mangle]
pub unsafe extern "C" fn expand_stack_locked(vma: *mut vm_area_struct, addr: c_ulong) -> c_int {
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn expand_stack(mm: *mut mm_struct, addr: c_ulong) -> *mut c_void {
    mmap_read_unlock(mm);
    return core::ptr::null_mut();
    }
//
// look up the first VMA exactly that exactly matches addr
// - should be called with mm->mmap_lock at least held readlocked
//
#[no_mangle]
pub unsafe extern "C" fn find_vma_exact(mm: *mut mm_struct, addr: c_ulong, len: c_ulong) -> *mut c_void {
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut end: c_ulong = 0;
    VMA_ITERATOR(vmi, mm, addr);
    vma = vma_iter_load(&vmi);
    if (!vma) {
    return core::ptr::null_mut();
    }
    if (vma.vm_start != addr) {
    return core::ptr::null_mut();
    }
    if (vma.vm_end != end) {
    return core::ptr::null_mut();
    }
    return vma;
    }
//
// determine whether a mapping should be permitted and, if so, what sort of
// mapping we're capable of supporting
//
#[no_mangle]
pub unsafe extern "C" fn validate_mmap_request(file: *mut file, addr: c_ulong, len: c_ulong, prot: c_ulong, flags: c_ulong, pgoff: c_ulong, _capabilities: *mut c_ulong) -> c_int {
    unsigned long capabilities, rlen;
    let mut ret = 0;
// do the simple checks first
    if (flags & MAP_FIXED) {
    return -EINVAL;
    }
    if ((flags & MAP_TYPE) != MAP_PRIVATE &&
    (flags & MAP_TYPE) != MAP_SHARED) {
    return -EINVAL;
    }
    if (!len) {
    return -EINVAL;
    }
// Careful about overflows..
    rlen = PAGE_ALIGN(len);
    if (!rlen || rlen > TASK_SIZE) {
    return -ENOMEM;
    }
// offset overflow?
    if ((pgoff + (rlen >> PAGE_SHIFT)) < pgoff) {
    return -EOVERFLOW;
    }
    if (file) {
// files must support mmap
    if (!can_mmap_file(file)) {
    return -ENODEV;
    }
// work out if what we've got could possibly be shared
// - we support chardevs that provide their own "memory"
// - we support files/blockdevs that are memory backed
//
    if (file.f_op.mmap_capabilities) {
    capabilities = file.f_op.mmap_capabilities(file);
    } else {
// no explicit capabilities set, so assume some
// defaults
    switch (file_inode(file).i_mode & S_IFMT) {
    case S_IFREG:
    case S_IFBLK:
    capabilities = NOMMU_MAP_COPY;
    break;
    case S_IFCHR:
    capabilities =
    NOMMU_MAP_DIRECT |
    NOMMU_MAP_READ |
    NOMMU_MAP_WRITE;
    break;
// label;
    return -EINVAL;
    }
    }
// eliminate any capabilities that we can't support on this
// device
    if (!file.f_op.get_unmapped_area) {
    capabilities &= ~NOMMU_MAP_DIRECT;
    }
    if (!(file.f_mode & FMODE_CAN_READ)) {
    capabilities &= ~NOMMU_MAP_COPY;
    }
// The file shall have been opened with read permission.
    if (!(file.f_mode & FMODE_READ)) {
    return -EACCES;
    }
    if (flags & MAP_SHARED) {
// do checks for writing, appending and locking
    if ((prot & PROT_WRITE) &&
    !(file.f_mode & FMODE_WRITE)) {
    return -EACCES;
    }
    if (IS_APPEND(file_inode(file)) &&
    (file.f_mode & FMODE_WRITE)) {
    return -EACCES;
    }
    if (!(capabilities & NOMMU_MAP_DIRECT)) {
    return -ENODEV;
    }
// we mustn't privatise shared mappings
    capabilities &= ~NOMMU_MAP_COPY;
    } else {
// we're going to read the file into private memory we
// allocate
    if (!(capabilities & NOMMU_MAP_COPY)) {
    return -ENODEV;
    }
// we don't permit a private writable mapping to be
// shared with the backing device
    if (prot & PROT_WRITE) {
    capabilities &= ~NOMMU_MAP_DIRECT;
    }
    }
    if (capabilities & NOMMU_MAP_DIRECT) {
    if (((prot & PROT_READ)  && !(capabilities & NOMMU_MAP_READ))  ||
    ((prot & PROT_WRITE) && !(capabilities & NOMMU_MAP_WRITE)) ||
    ((prot & PROT_EXEC)  && !(capabilities & NOMMU_MAP_EXEC))
    ) {
    capabilities &= ~NOMMU_MAP_DIRECT;
    if (flags & MAP_SHARED) {
    pr_warn!("MAP_SHARED not completely supported on !MMU\n");
    return -EINVAL;
    }
    }
    }
// handle executable mappings and implied executable
// mappings
    if (path_noexec(&file.f_path)) {
    if (prot & PROT_EXEC) {
    return -EPERM;
    }
    } else if ((prot & PROT_READ) && !(prot & PROT_EXEC)) {
// handle implication of PROT_EXEC by PROT_READ
    if (current.personality & READ_IMPLIES_EXEC) {
    if (capabilities & NOMMU_MAP_EXEC) {
    prot |= PROT_EXEC;
    }
    }
    } else if ((prot & PROT_READ) &&
    (prot & PROT_EXEC) &&
    !(capabilities & NOMMU_MAP_EXEC)
    ) {
// backing file is not executable, try to copy
    capabilities &= ~NOMMU_MAP_DIRECT;
    }
    } else {
// anonymous mappings are always memory backed and can be
// privately mapped
//
    capabilities = NOMMU_MAP_COPY;
// handle PROT_EXEC implication by PROT_READ
    if ((prot & PROT_READ) &&
    (current.personality & READ_IMPLIES_EXEC)) {
    prot |= PROT_EXEC;
    }
    }
// allow the security API to have its say
    ret = security_mmap_addr(addr);
    if (ret < 0) {
    return ret;
    }
// looks okay
// _capabilities = capabilities;
    return 0;
    }
//
// we've determined that we can make the mapping, now translate what we
// now know into VMA flags
//
    static vm_flags_t determine_vm_flags(file *file,
    unsigned long prot,
    unsigned long flags,
    unsigned long capabilities)
    {
    let mut vm_flags;
    vm_flags = calc_vm_prot_bits(prot, 0) | calc_vm_flag_bits(file, flags);
    if (!file) {
//
// MAP_ANONYMOUS. MAP_SHARED is mapped to MAP_PRIVATE, because
// there is no fork().
//
    vm_flags |= VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC;
    } else if (flags & MAP_PRIVATE) {
// MAP_PRIVATE file mapping
    if (capabilities & NOMMU_MAP_DIRECT) {
    vm_flags |= (capabilities & NOMMU_VMFLAGS);
    }
    else {
    vm_flags |= VM_MAYREAD | VM_MAYWRITE | VM_MAYEXEC;
    }
    if (!(prot & PROT_WRITE) && !current.ptrace) {
//
// R/O private file mapping which cannot be used to
// modify memory, especially also not via active ptrace
// (e.g., set breakpoints) or later by upgrading
// permissions (no mprotect()). We can try overlaying
// the file mapping, which will work e.g., on chardevs,
// ramfs/tmpfs/shmfs and romfs/cramf.
//
    vm_flags |= VM_MAYOVERLAY;
    }
    } else {
// MAP_SHARED file mapping: NOMMU_MAP_DIRECT is set.
    vm_flags |= VM_SHARED | VM_MAYSHARE |
    (capabilities & NOMMU_VMFLAGS);
    }
    return vm_flags;
    }
//
// set up a shared mapping on a file (the driver or filesystem provides and
// pins the storage)
//
#[no_mangle]
unsafe extern "C" fn do_mmap_shared_file(vma: *mut vm_area_struct) -> c_int {
    let mut ret = 0;
    ret = mmap_file(vma.vm_file, vma);
    if (ret == 0) {
    vma.vm_region.vm_top = vma.vm_region.vm_end;
    return 0;
    }
    if (ret != -ENOSYS) {
    return ret;
    }
// getting -ENOSYS indicates that direct mmap isn't possible (as
// opposed to tried but failed) so we can only give a suitable error as
// it's not possible to make a private copy if MAP_SHARED was given
    return -ENODEV;
    }
//
// set up a private mapping or an anonymous shared mapping
//
#[no_mangle]
pub unsafe extern "C" fn do_mmap_private(vma: *mut vm_area_struct, region: *mut vm_region, len: c_ulong, capabilities: c_ulong) -> c_int {
    unsigned long total, point;
pub static mut base: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    let mut order = 0;
//
// Invoke the file's mapping function so that it can keep track of
// shared mappings on devices or memory. VM_MAYOVERLAY will be set if
// it may attempt to share, which will make is_nommu_shared_mapping()
// happy.
//
    if (capabilities & NOMMU_MAP_DIRECT) {
    ret = mmap_file(vma.vm_file, vma);
// shouldn't return success if we're not sharing
    if (WARN_ON_ONCE!(!is_nommu_shared_mapping(vma.vm_flags))) {
    ret = -ENOSYS;
    }
    if (ret == 0) {
    vma.vm_region.vm_top = vma.vm_region.vm_end;
    return 0;
    }
    if (ret != -ENOSYS) {
    return ret;
    }
// getting an ENOSYS error indicates that direct mmap isn't
// possible (as opposed to tried but failed) so we'll try to
// make a private copy of the data and map that instead
    }
// allocate some memory to hold the mapping
// - note that this may not return a page-aligned address if the object
// we're allocating is smaller than a page
//
    order = get_order(len);
    total = 1 << order;
    point = len >> PAGE_SHIFT;
// we don't want to allocate a power-of-2 sized page set
    if (sysctl_nr_trim_pages && total - point >= sysctl_nr_trim_pages) {
    total = point;
    }
    base = alloc_pages_exact(total << PAGE_SHIFT, GFP_KERNEL);
    if (!base) {
// goto;
    }
    atomic_long_add(total, &mmap_pages_allocated);
    vm_flags_set(vma, VM_MAPPED_COPY);
    region.vm_flags = vma.vm_flags;
    region.vm_start = (unsigned long) base;
    region.vm_end   = region.vm_start + len;
    region.vm_top   = region.vm_start + (total << PAGE_SHIFT);
    vma.vm_start = region.vm_start;
    vma.vm_end   = region.vm_start + len;
    if (vma.vm_file) {
// read the contents of a file into the copy
    let mut fpos = 0;
    fpos = vma_start_pgoff(vma);
    fpos <<= PAGE_SHIFT;
    ret = kernel_read(vma.vm_file, base, len, &fpos);
    if (ret < 0) {
// goto;
    }
// clear the last little bit
    if (ret < len) {
    memset(base + ret, 0, len - ret);
    }
    } else {
    vma_set_anonymous(vma);
    }
    return 0;
// label;
    free_page_series(region.vm_start, region.vm_top);
    region.vm_start = vma.vm_start = 0;
    region.vm_end   = vma.vm_end = 0;
    region.vm_top   = 0;
    return ret;
// label;
    pr_err!("Allocation of length %lu from process %d (%s) failed\n",
    len, current.pid, current.comm);
    show_mem();
    return -ENOMEM;
    }
//
// handle mapping creation for uClinux
//
#[no_mangle]
pub unsafe extern "C" fn do_mmap(file: *mut file, addr: c_ulong, len: c_ulong, prot: c_ulong, flags: c_ulong, vma_flags: vma_flags_t, pgoff: c_ulong, populate: *mut c_ulong, uf: *mut list_head) -> c_ulong {
pub static mut vm_flags: vm_flags_t = 0;
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut region: *mut c_void = core::ptr::null_mut();
pub static mut rb: *mut c_void = core::ptr::null_mut();
    unsigned long capabilities, result;
    let mut ret = 0;
    VMA_ITERATOR(vmi, current.mm, 0);
// populate = 0;
// decide whether we should attempt the mapping, and if so what sort of
// mapping
    ret = validate_mmap_request(file, addr, len, prot, flags, pgoff,
    &capabilities);
    if (ret < 0) {
    return ret;
    }
    if (current.mm.map_count >= get_sysctl_max_map_count()) {
    return -ENOMEM;
    }
// we ignore the address hint
    addr = 0;
    len = PAGE_ALIGN(len);
// we've determined that we can make the mapping, now translate what we
// now know into VMA flags
    vm_flags |= determine_vm_flags(file, prot, flags, capabilities);
// we're going to need to record the mapping
    region = kmem_cache_zalloc(vm_region_jar, GFP_KERNEL);
    if (!region) {
// goto;
    }
    vma = vm_area_alloc(current.mm);
    if (!vma) {
// goto;
    }
    region.vm_usage = 1;
    region.vm_flags = vm_flags;
    region.vm_pgoff = pgoff;
    vm_flags_init(vma, vm_flags);
    vma_set_pgoff(vma, pgoff);
    if (file) {
    region.vm_file = get_file(file);
    vma.vm_file = get_file(file);
    }
    down_write(&nommu_region_sem);
// if we want to share, we need to check for regions created by other
// mmap() calls that overlap with our proposed mapping
// - we can only share with a superset match on most regular files
// - shared mappings on character devices and memory backed files are
// permitted to overlap inexactly as far as we are concerned for in
// these cases, sharing is handled in the driver or filesystem rather
// than here
//
    if (is_nommu_shared_mapping(vm_flags)) {
pub static mut pregion: *mut c_void = core::ptr::null_mut();
    unsigned long pglen, rpglen, pgend, rpgend, start;
    pglen = (len + PAGE_SIZE - 1) >> PAGE_SHIFT;
    pgend = pgoff + pglen;
    for (rb = rb_first(&nommu_region_tree); rb; rb = rb_next(rb)) {
    pregion = rb_entry(rb, vm_region, vm_rb);
    if (!is_nommu_shared_mapping(pregion.vm_flags)) {
    continue;
    }
// search for overlapping mappings on the same file
    if (file_inode(pregion.vm_file) !=
    file_inode(file)) {
    continue;
    }
    if (pregion.vm_pgoff >= pgend) {
    continue;
    }
    rpglen = pregion.vm_end - pregion.vm_start;
    rpglen = (rpglen + PAGE_SIZE - 1) >> PAGE_SHIFT;
    rpgend = pregion.vm_pgoff + rpglen;
    if (pgoff >= rpgend) {
    continue;
    }
// handle inexactly overlapping matches between
// mappings
    if ((pregion.vm_pgoff != pgoff || rpglen != pglen) &&
    !(pgoff >= pregion.vm_pgoff && pgend <= rpgend)) {
// new mapping is not a subset of the region
    if (!(capabilities & NOMMU_MAP_DIRECT)) {
// goto;
    }
    continue;
    }
// we've found a region we can share
    pregion.vm_usage += 1;
    vma.vm_region = pregion;
    start = pregion.vm_start;
    start += (pgoff - pregion.vm_pgoff) << PAGE_SHIFT;
    vma.vm_start = start;
    vma.vm_end = start + len;
    if (pregion.vm_flags & VM_MAPPED_COPY) {
    vm_flags_set(vma, VM_MAPPED_COPY);
    }
    else {
    ret = do_mmap_shared_file(vma);
    if (ret < 0) {
    vma.vm_region = core::ptr::null_mut();
    vma.vm_start = 0;
    vma.vm_end = 0;
    pregion.vm_usage -= 1;
    pregion = core::ptr::null_mut();
// goto;
    }
    }
    fput(region.vm_file);
    kmem_cache_free(vm_region_jar, region);
    region = pregion;
    result = start;
// goto;
    }
// obtain the address at which to make a shared mapping
// - this is the hook for quasi-memory character devices to
// tell us the location of a shared mapping
//
    if (capabilities & NOMMU_MAP_DIRECT) {
    addr = file.f_op.get_unmapped_area(file, addr, len,
    pgoff, flags);
    if (IS_ERR_VALUE(addr)) {
    ret = addr;
    if (ret != -ENOSYS) {
// goto;
    }
// the driver refused to tell us where to site
// the mapping so we'll have to attempt to copy
// it
    ret = -ENODEV;
    if (!(capabilities & NOMMU_MAP_COPY)) {
// goto;
    }
    capabilities &= ~NOMMU_MAP_DIRECT;
    } else {
    vma.vm_start = region.vm_start = addr;
    vma.vm_end = region.vm_end = addr + len;
    }
    }
    }
    vma.vm_region = region;
// set up the mapping
// - the region is filled in if NOMMU_MAP_DIRECT is still set
//
    if (file && vma.vm_flags & VM_SHARED) {
    ret = do_mmap_shared_file(vma);
    }
    else {
    ret = do_mmap_private(vma, region, len, capabilities);
    }
    if (ret < 0) {
// goto;
    }
// clear anonymous mappings that don't ask for uninitialized data
    if (!vma.vm_file &&
    (!IS_ENABLED!(CONFIG_MMAP_ALLOW_UNINITIALIZED) ||
    !(flags & MAP_UNINITIALIZED))) {
    memset(region.vm_start, 0,
    region.vm_end - region.vm_start);
    }
// okay... we have a mapping; now we have to register it
    result = vma.vm_start;
    current.mm.total_vm += len >> PAGE_SHIFT;
// label;
    BUG_ON!(!vma.vm_region);
    vma_iter_config(&vmi, vma.vm_start, vma.vm_end);
    if (vma_iter_prealloc(&vmi, vma)) {
// goto;
    }
    add_nommu_region(region);
    setup_vma_to_mm(vma, current.mm);
    current.mm.map_count += 1;
// add the VMA to the tree
    vma_iter_store_new(&vmi, vma);
// we flush the region from the icache only when the first executable
// mapping of it is made
    if (vma.vm_flags & VM_EXEC && !region.vm_icache_flushed) {
    flush_icache_user_range(region.vm_start, region.vm_end);
    region.vm_icache_flushed = true;
    }
    up_write(&nommu_region_sem);
    return result;
// label;
    vma_close(vma);
// if the error was from shared mapping/existing region, don't free the region.
// this has to be before releasing semaphore.
//
    if (region.vm_usage == 1) {
    if (region.vm_file) {
    fput(region.vm_file);
    }
    kmem_cache_free(vm_region_jar, region);
    } else {
    region.vm_usage -= 1;
    }
    up_write(&nommu_region_sem);
    vma_iter_free(&vmi);
    if (vma.vm_file) {
    fput(vma.vm_file);
    }
    vm_area_free(vma);
    return ret;
// label;
    pr_warn!("Attempt to share mismatched mappings\n");
    ret = -EINVAL;
// goto;
// label;
    pr_warn!("Allocation of vma iterator for process %d failed\n", current.pid);
    show_mem();
    ret = -ENOMEM;
// in case that the region is allocated via do_mmap_private()
    if ((region.vm_usage == 1) && (region.vm_flags & VM_MAPPED_COPY)) {
    free_page_series(region.vm_start, region.vm_top);
    }
// goto;
// label;
    kmem_cache_free(vm_region_jar, region);
    pr_warn!("Allocation of vma for %lu byte allocation from process %d failed\n",
    len, current.pid);
    show_mem();
    return -ENOMEM;
// label;
    pr_warn!("Allocation of vm region for %lu byte allocation from process %d failed\n",
    len, current.pid);
    show_mem();
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn ksys_mmap_pgoff(addr: c_ulong, len: c_ulong, prot: c_ulong, flags: c_ulong, fd: c_ulong, pgoff: c_ulong) -> c_ulong {
    let mut file = core::ptr::null_mut();
pub static mut retval: c_ulong = 0;
    audit_mmap_fd(fd, flags);
    if (!(flags & MAP_ANONYMOUS)) {
    file = fget(fd);
    if (!file) {
// goto;
    }
    }
    retval = vm_mmap_pgoff(file, addr, len, prot, flags, pgoff);
    if (file) {
    fput(file);
    }
// label;
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_mmap_pgoff(addr: usize, len: usize, prot: usize, flags: usize, fd: usize, pgoff: usize) -> c_long {
    return ksys_mmap_pgoff(addr, len, prot, flags, fd, pgoff);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmap_arg_struct {
    pub addr: c_ulong,
    pub len: c_ulong,
    pub prot: c_ulong,
    pub flags: c_ulong,
    pub fd: c_ulong,
    pub offset: c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn sys_old_mmap(arg: usize) -> c_long {
pub static mut a: usize = 0;
    if (copy_from_user(&a, arg, sizeof!(a))) {
    return -EFAULT;
    }
    if (offset_in_page(a.offset)) {
    return -EINVAL;
    }
    return ksys_mmap_pgoff(a.addr, a.len, a.prot, a.flags, a.fd,
    a.offset >> PAGE_SHIFT);
    }

//
// split a vma into two pieces at address 'addr', a new vma is allocated either
// for the first part or the tail.
//
#[no_mangle]
pub unsafe extern "C" fn split_vma(vmi: *mut vma_iterator, vma: *mut vm_area_struct, addr: c_ulong, new_below: c_int) -> c_int {
pub static mut new: *mut c_void = core::ptr::null_mut();
pub static mut region: *mut c_void = core::ptr::null_mut();
    let mut npages = 0;
pub static mut mm: *mut c_void = core::ptr::null_mut();
// we're only permitted to split anonymous regions (these should have
// only a single usage on the region)
    if (vma.vm_file) {
    return -ENOMEM;
    }
    mm = vma.vm_mm;
    if (mm.map_count >= get_sysctl_max_map_count()) {
    return -ENOMEM;
    }
    region = kmem_cache_alloc(vm_region_jar, GFP_KERNEL);
    if (!region) {
    return -ENOMEM;
    }
    new = vm_area_dup(vma);
    if (!new) {
// goto;
    }
// most fields are the same, copy all, and then fixup
// region = *vma->vm_region;
    new.vm_region = region;
    npages = linear_page_delta(vma, addr);
    if (new_below) {
    region.vm_top = region.vm_end = new.vm_end = addr;
    } else {
    region.vm_start = new.vm_start = addr;
    vma_add_pgoff(new, npages);
    region.vm_pgoff = vma_start_pgoff(new);
    }
    vma_iter_config(vmi, new.vm_start, new.vm_end);
    if (vma_iter_prealloc(vmi, vma)) {
    pr_warn!("Allocation of vma tree for process %d failed\n",
    current.pid);
// goto;
    }
    if (new.vm_ops && new.vm_ops.open) {
    new.vm_ops.open(new);
    }
    down_write(&nommu_region_sem);
    delete_nommu_region(vma.vm_region);
    if (new_below) {
    vma.vm_region.vm_start = vma.vm_start = addr;
    vma_add_pgoff(vma, npages);
    vma.vm_region.vm_pgoff = vma_start_pgoff(vma);
    } else {
    vma.vm_region.vm_end = vma.vm_end = addr;
    vma.vm_region.vm_top = addr;
    }
    add_nommu_region(vma.vm_region);
    add_nommu_region(new.vm_region);
    up_write(&nommu_region_sem);
    setup_vma_to_mm(vma, mm);
    setup_vma_to_mm(new, mm);
    vma_iter_store_new(vmi, new);
// vmi should point lower address
    if (new_below) {
    vma_next(vmi);
    }
    mm.map_count += 1;
    return 0;
// label;
    vm_area_free(new);
// label;
    kmem_cache_free(vm_region_jar, region);
    return -ENOMEM;
    }
//
// shrink a VMA by removing the specified chunk from either the beginning or
// the end
//
#[no_mangle]
pub unsafe extern "C" fn vmi_shrink_vma(vmi: *mut vma_iterator, vma: *mut vm_area_struct, from: c_ulong, to: c_ulong) -> c_int {
pub static mut region: *mut c_void = core::ptr::null_mut();
// adjust the VMA's pointers, which may reposition it in the MM's tree
// and list
    if (from > vma.vm_start) {
    if (vma_iter_clear_gfp(vmi, from, vma.vm_end, GFP_KERNEL)) {
    return -ENOMEM;
    }
    vma.vm_end = from;
    } else {
    if (vma_iter_clear_gfp(vmi, vma.vm_start, to, GFP_KERNEL)) {
    return -ENOMEM;
    }
    vma.vm_start = to;
    }
// cut the backing region down to size
    region = vma.vm_region;
    BUG_ON!(region.vm_usage != 1);
    down_write(&nommu_region_sem);
    delete_nommu_region(region);
    if (from > region.vm_start) {
    to = region.vm_top;
    region.vm_top = region.vm_end = from;
    } else {
    region.vm_start = to;
    }
    add_nommu_region(region);
    up_write(&nommu_region_sem);
    free_page_series(from, to);
    return 0;
    }
//
// release a mapping
// - under NOMMU conditions the chunk to be unmapped must be backed by a single
// VMA, though it need not cover the whole VMA
//
#[no_mangle]
pub unsafe extern "C" fn do_munmap(mm: *mut mm_struct, start: c_ulong, len: usize, uf: *mut list_head) -> c_int {
    VMA_ITERATOR(vmi, mm, start);
pub static mut vma: *mut c_void = core::ptr::null_mut();
    let mut end = 0;
pub static mut ret: c_int = 0;
    len = PAGE_ALIGN(len);
    if (len == 0) {
    return -EINVAL;
    }
    end = start + len;
// find the first potentially overlapping VMA
    vma = vma_find(&vmi, end);
    if (!vma) {
    static int limit;
    if (limit < 5) {
    pr_warn!("munmap of memory not mmapped by process %d (%s): 0x%lx-0x%lx\n",
    current.pid, current.comm,
    start, start + len - 1);
    limit += 1;
    }
    return -EINVAL;
    }
// we're allowed to split an anonymous VMA but not a file-backed one
    if (vma.vm_file) {
    do {
    if (start > vma.vm_start) {
    return -EINVAL;
    }
    if (end == vma.vm_end) {
// goto;
    }
    vma = vma_find(&vmi, end);
    } while (vma);
    return -EINVAL;
    } else {
// the chunk must be a subset of the VMA found
    if (start == vma.vm_start && end == vma.vm_end) {
// goto;
    }
    if (start < vma.vm_start || end > vma.vm_end) {
    return -EINVAL;
    }
    if (offset_in_page(start)) {
    return -EINVAL;
    }
    if (end != vma.vm_end && offset_in_page(end)) {
    return -EINVAL;
    }
    if (start != vma.vm_start && end != vma.vm_end) {
    ret = split_vma(&vmi, vma, start, 1);
    if (ret < 0) {
    return ret;
    }
    }
    return vmi_shrink_vma(&vmi, vma, start, end);
    }
// label;
    if (delete_vma_from_mm(vma)) {
    ret = -ENOMEM;
    }
    else {
    delete_vma(mm, vma);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn vm_munmap(addr: c_ulong, len: usize) -> c_int {
    let mut mm = current.mm;
    let mut ret = 0;
    mmap_write_lock(mm);
    ret = do_munmap(mm, addr, len, core::ptr::null_mut());
    mmap_write_unlock(mm);
    return ret;
    }
    EXPORT_SYMBOL(vm_munmap);
#[no_mangle]
pub unsafe extern "C" fn sys_munmap(addr: usize, len: usize) -> c_long {
    return vm_munmap(addr, len);
    }
//
// release all the mappings made in a process's VM space
//
#[no_mangle]
pub unsafe extern "C" fn exit_mmap(mm: *mut mm_struct) {
    VMA_ITERATOR(vmi, mm, 0);
pub static mut vma: *mut c_void = core::ptr::null_mut();
    if (!mm) {
    return;
    }
    mm.total_vm = 0;
//
// Lock the mm to avoid assert complaining even though this is the only
// user of the mm
//
    mmap_write_lock(mm);
    for_each_vma(vmi, vma) {
    cleanup_vma_from_mm(vma);
    delete_vma(mm, vma);
    cond_resched();
    }
    __mt_destroy(&mm.mm_mt);
    mmap_write_unlock(mm);
    }
//
// expand (or shrink) an existing mapping, potentially moving it at the same
// time (controlled by the MREMAP_MAYMOVE flag and available VM space)
//
// under NOMMU conditions, we only permit changing a mapping's size, and only
// as long as it stays within the region allocated by do_mmap_private() and the
// block is not shareable
//
// MREMAP_FIXED is not supported under NOMMU conditions
//
#[no_mangle]
pub unsafe extern "C" fn do_mremap(addr: c_ulong, old_len: c_ulong, new_len: c_ulong, flags: c_ulong, new_addr: c_ulong) -> c_ulong {
pub static mut vma: *mut c_void = core::ptr::null_mut();
// insanity checks first
    old_len = PAGE_ALIGN(old_len);
    new_len = PAGE_ALIGN(new_len);
    if (old_len == 0 || new_len == 0) {
    return (unsigned long) -EINVAL;
    }
    if (offset_in_page(addr)) {
    return -EINVAL;
    }
    if (flags & MREMAP_FIXED && new_addr != addr) {
    return (unsigned long) -EINVAL;
    }
    vma = find_vma_exact(current.mm, addr, old_len);
    if (!vma) {
    return (unsigned long) -EINVAL;
    }
    if (vma.vm_end != vma.vm_start + old_len) {
    return (unsigned long) -EFAULT;
    }
    if (is_nommu_shared_mapping(vma.vm_flags)) {
    return (unsigned long) -EPERM;
    }
    if (new_len > vma.vm_region.vm_end - vma.vm_region.vm_start) {
    return (unsigned long) -ENOMEM;
    }
// all checks complete - do it
    vma.vm_end = vma.vm_start + new_len;
    return vma.vm_start;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_mremap(addr: usize, old_len: usize, new_len: usize, flags: usize, new_addr: usize) -> c_long {
    let mut ret = 0;
    mmap_write_lock(current.mm);
    ret = do_mremap(addr, old_len, new_len, flags, new_addr);
    mmap_write_unlock(current.mm);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn remap_pfn_range(vma: *mut vm_area_struct, addr: c_ulong, pfn: c_ulong, size: c_ulong, prot: pgprot_t) -> c_int {
    if (addr != (pfn << PAGE_SHIFT)) {
    return -EINVAL;
    }
    vm_flags_set(vma, VM_IO | VM_PFNMAP | VM_DONTEXPAND | VM_DONTDUMP);
    return 0;
    }
    EXPORT_SYMBOL(remap_pfn_range);
#[no_mangle]
pub unsafe extern "C" fn vm_iomap_memory(vma: *mut vm_area_struct, start: phys_addr_t, len: c_ulong) -> c_int {
pub static mut pfn: c_ulong = 0;
pub static mut vm_len: c_ulong = 0;
    pfn += vma_start_pgoff(vma);
    return io_remap_pfn_range(vma, vma.vm_start, pfn, vm_len, vma.vm_page_prot);
    }
    EXPORT_SYMBOL(vm_iomap_memory);
#[no_mangle]
pub unsafe extern "C" fn remap_vmalloc_range(vma: *mut vm_area_struct, addr: *mut c_void, pgoff: c_ulong) -> c_int {
pub static mut size: c_uint = 0;
    if (!(vma.vm_flags & VM_USERMAP)) {
    return -EINVAL;
    }
    vma.vm_start = (unsigned long)(addr + (pgoff << PAGE_SHIFT));
    vma.vm_end = vma.vm_start + size;
    return 0;
    }
    EXPORT_SYMBOL(remap_vmalloc_range);
#[no_mangle]
pub unsafe extern "C" fn filemap_fault(vmf: *mut vm_fault) -> vm_fault_t {
    BUG();
    return 0;
    }
    EXPORT_SYMBOL(filemap_fault);
    vm_fault_t filemap_map_pages(vm_fault *vmf,
    pgoff_t start_pgoff, pgoff_t end_pgoff)
    {
    BUG();
    return 0;
    }
    EXPORT_SYMBOL(filemap_map_pages);
#[no_mangle]
pub unsafe extern "C" fn __access_remote_vm(mm: *mut mm_struct, addr: c_ulong, buf: *mut c_void, len: c_int, gup_flags: c_uint) -> c_int {
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut write: c_int = 0;
    if (mmap_read_lock_killable(mm)) {
    return 0;
    }
// the access must start within one of the target process's mappings
    vma = find_vma(mm, addr);
    if (vma) {
// don't overrun this mapping
    if (addr + len >= vma.vm_end) {
    len = vma.vm_end - addr;
    }
// only read or write mappings where it is permitted
    if (write && vma.vm_flags & VM_MAYWRITE) {
    copy_to_user_page(vma, core::ptr::null_mut(), addr,
     addr, buf, len);
    }

    else if (!write && vma.vm_flags & VM_MAYREAD) {
    copy_from_user_page(vma, core::ptr::null_mut(), addr,
    buf,  addr, len);
    }
    else {
    len = 0;
    }
    } else {
    len = 0;
    }
    mmap_read_unlock(mm);
    return len;
    }
//
// access_remote_vm - access another process' address space
// @mm:		the mm_struct of the target address space
// @addr:	start address to access
// @buf:	source or destination buffer
// @len:	number of bytes to transfer
// @gup_flags:	flags modifying lookup behaviour
//
// The caller must hold a reference on @mm.
//
#[no_mangle]
pub unsafe extern "C" fn access_remote_vm(mm: *mut mm_struct, addr: c_ulong, buf: *mut c_void, len: c_int, gup_flags: c_uint) -> c_int {
    return __access_remote_vm(mm, addr, buf, len, gup_flags);
    }
//
// Access another process' address space.
// - source/target buffer must be kernel space
//
#[no_mangle]
pub unsafe extern "C" fn access_process_vm(tsk: *mut task_struct, addr: c_ulong, buf: *mut c_void, len: c_int, gup_flags: c_uint) -> c_int {
pub static mut mm: *mut c_void = core::ptr::null_mut();
    if (addr + len < addr) {
    return 0;
    }
    mm = get_task_mm(tsk);
    if (!mm) {
    return 0;
    }
    len = __access_remote_vm(mm, addr, buf, len, gup_flags);
    mmput(mm);
    return len;
    }
    EXPORT_SYMBOL_GPL(access_process_vm);

//
// Copy a string from another process's address space as given in mm.
// If there is any error return -EFAULT.
//
#[no_mangle]
pub unsafe extern "C" fn __copy_remote_vm_str(mm: *mut mm_struct, addr: c_ulong, buf: *mut c_void, len: c_int) -> c_int {
    let mut addr_end = 0;
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
// buf = '\0';
    if (mmap_read_lock_killable(mm)) {
    return ret;
    }
// the access must start within one of the target process's mappings
    vma = find_vma(mm, addr);
    if (!vma) {
// goto;
    }
    if (check_add_overflow(addr, len, &addr_end)) {
// goto;
    }
// don't overrun this mapping
    if (addr_end > vma.vm_end) {
    len = vma.vm_end - addr;
    }
// only read mappings where it is permitted
    if (vma.vm_flags & VM_MAYREAD) {
    ret = strscpy(buf, addr, len);
    if (ret < 0) {
    ret = len - 1;
    }
    }
// label;
    mmap_read_unlock(mm);
    return ret;
    }
//
// copy_remote_vm_str - copy a string from another process's address space.
// @tsk:	the task of the target address space
// @addr:	start address to read from
// @buf:	destination buffer
// @len:	number of bytes to copy
// @gup_flags:	flags modifying lookup behaviour (unused)
//
// The caller must hold a reference on @mm.
//
// Return: number of bytes copied from @addr (source) to @buf (destination);
// not including the trailing NUL. Always guaranteed to leave NUL-terminated
// buffer. On any error, return -EFAULT.
//
#[no_mangle]
pub unsafe extern "C" fn copy_remote_vm_str(tsk: *mut task_struct, addr: c_ulong, buf: *mut c_void, len: c_int, gup_flags: c_uint) -> c_int {
pub static mut mm: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (unlikely(len == 0)) {
    return 0;
    }
    mm = get_task_mm(tsk);
    if (!mm) {
// buf = '\0';
    return -EFAULT;
    }
    ret = __copy_remote_vm_str(mm, addr, buf, len);
    mmput(mm);
    return ret;
    }
    EXPORT_SYMBOL_GPL(copy_remote_vm_str);

//
// nommu_shrink_inode_mappings - Shrink the shared mappings on an inode
// @inode: The inode to check
// @size: The current filesize of the inode
// @newsize: The proposed filesize of the inode
//
// Check the shared mappings on an inode on behalf of a shrinking truncate to
// make sure that any outstanding VMAs aren't broken and then shrink the
// vm_regions that extend beyond so that do_mmap() doesn't
// automatically grant mappings that are too large.
//
#[no_mangle]
pub unsafe extern "C" fn nommu_shrink_inode_mappings(inode: *mut inode, size: size_t, newsize: size_t) -> c_int {
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut region: *mut c_void = core::ptr::null_mut();
    pgoff_t low, high;
    size_t r_size, r_top;
    low = newsize >> PAGE_SHIFT;
    high = (size + PAGE_SIZE - 1) >> PAGE_SHIFT;
    down_write(&nommu_region_sem);
    i_mmap_lock_read(inode.i_mapping);
// search for VMAs that fall within the dead zone
    mapping_rmap_tree_foreach(vma, inode.i_mapping, low, high) {
// found one - only interested if it's shared out of the page
// cache
    if (vma.vm_flags & VM_SHARED) {
    i_mmap_unlock_read(inode.i_mapping);
    up_write(&nommu_region_sem);
    return -ETXTBSY; /* not quite true, but near enough */
    }
    }
// reduce any regions that overlap the dead zone - if in existence,
// these will be pointed to by VMAs that don't overlap the dead zone
//
// we don't check for any regions that start beyond the EOF as there
// shouldn't be any
//
    mapping_rmap_tree_foreach(vma, inode.i_mapping, 0, ULONG_MAX) {
    if (!(vma.vm_flags & VM_SHARED)) {
    continue;
    }
    region = vma.vm_region;
    r_size = region.vm_top - region.vm_start;
    r_top = (region.vm_pgoff << PAGE_SHIFT) + r_size;
    if (r_top > newsize) {
    region.vm_top -= r_top - newsize;
    if (region.vm_end > region.vm_top) {
    region.vm_end = region.vm_top;
    }
    }
    }
    i_mmap_unlock_read(inode.i_mapping);
    up_write(&nommu_region_sem);
    return 0;
    }
//
// Initialise sysctl_user_reserve_kbytes.
//
// This is intended to prevent a user from starting a single memory hogging
// process, such that they cannot recover (kill the hog) in OVERCOMMIT_NEVER
// mode.
//
// The default value is min(3% of free memory, 128MB)
// 128MB is enough to recover with sshd/login, bash, and top/kill.
//
#[no_mangle]
unsafe extern "C" fn init_user_reserve() -> int __meminit {
    let mut free_kbytes = 0;
    free_kbytes = K(global_zone_page_state(NR_FREE_PAGES));
    sysctl_user_reserve_kbytes = min(free_kbytes / 32, 1UL << 17);
    return 0;
    }
    subsys_initcall!(init_user_reserve);
//
// Initialise sysctl_admin_reserve_kbytes.
//
// The purpose of sysctl_admin_reserve_kbytes is to allow the sys admin
// to log in and kill a memory hogging process.
//
// Systems with more than 256MB will reserve 8MB, enough to recover
// with sshd, bash, and top in OVERCOMMIT_GUESS. Smaller systems will
// only reserve 3% of free pages by default.
//
#[no_mangle]
unsafe extern "C" fn init_admin_reserve() -> int __meminit {
    let mut free_kbytes = 0;
    free_kbytes = K(global_zone_page_state(NR_FREE_PAGES));
    sysctl_admin_reserve_kbytes = min(free_kbytes / 32, 1UL << 13);
    return 0;
    }
    subsys_initcall!(init_admin_reserve);
#[no_mangle]
pub unsafe extern "C" fn dup_mmap(mm: *mut mm_struct, oldmm: *mut mm_struct) -> c_int {
    mmap_write_lock(oldmm);
    dup_mm_exe_file(mm, oldmm);
    mmap_write_unlock(oldmm);
    return 0;
    }
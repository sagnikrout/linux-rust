//! Automatically rewritten from C to Rust
//! Source: mm/memblock.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Procedures for maintaining information about logical memory blocks.
//
// Peter Bergner, IBM Corp.	June 2001.
// Copyright (C) 2001 Peter Bergner.
//

pub const INIT_MEMBLOCK_REGIONS: c_int = 128;
pub const INIT_PHYSMEM_REGIONS: c_int = 4;

//
// DOC: memblock overview
//
// Memblock is a method of managing memory regions during the early
// boot period when the usual kernel memory allocators are not up and
// running.
//
// Memblock views the system memory as collections of contiguous
// regions. There are several types of these collections:
//
// * ``memory`` - describes the physical memory available to the
// kernel; this may differ from the actual physical memory installed
// in the system, for instance when the memory is restricted with
// ``mem=`` command line parameter
// * ``reserved`` - describes the regions that were allocated
// * ``physmem`` - describes the actual physical memory available during
// boot regardless of the possible restrictions and memory hot(un)plug;
// the ``physmem`` type is only available on some architectures.
//
// Each region is represented by struct memblock_region that
// defines the region extents, its attributes and NUMA node id on NUMA
// systems. Every memory type is described by the struct memblock_type
// which contains an array of memory regions along with
// the allocator metadata. The "memory" and "reserved" types are nicely
// wrapped with struct memblock. This structure is statically
// initialized at build time. The region arrays are initially sized to
// %INIT_MEMBLOCK_MEMORY_REGIONS for "memory" and
// %INIT_MEMBLOCK_RESERVED_REGIONS for "reserved". The region array
// for "physmem" is initially sized to %INIT_PHYSMEM_REGIONS.
// The memblock_allow_resize() enables automatic resizing of the region
// arrays during addition of new regions. This feature should be used
// with care so that memory allocated for the region array will not
// overlap with areas that should be reserved, for example initrd.
//
// The early architecture setup should tell memblock what the physical
// memory layout is by using memblock_add() or memblock_add_node()
// functions. The first function does not assign the region to a NUMA
// node and it is appropriate for UMA systems. Yet, it is possible to
// use it on NUMA systems as well and assign the region to a NUMA node
// later in the setup process using memblock_set_node(). The
// memblock_add_node() performs such an assignment directly.
//
// Once memblock is setup the memory can be allocated using one of the
// API variants:
//
// * memblock_phys_alloc*() - these functions return the **physical
// address of the allocated memory
// * memblock_alloc*() - these functions return the **virtual** address
// of the allocated memory.
//
// Note, that both API variants use implicit assumptions about allowed
// memory ranges and the fallback methods. Consult the documentation
// of memblock_alloc_internal() and memblock_alloc_range_nid()
// functions for more elaborate description.
//
// As the system boot progresses, the architecture specific mem_init()
// function frees all the memory to the buddy page allocator.
//
// Unless an architecture enables %CONFIG_ARCH_KEEP_MEMBLOCK, the
// memblock data structures (except "physmem") will be discarded after the
// system initialization completes.
//

    struct pglist_data __refdata contig_page_data;
    EXPORT_SYMBOL(contig_page_data);

    let mut max_low_pfn = 0;
    let mut min_low_pfn = 0;
    let mut max_pfn = 0;
    unsigned long long max_possible_pfn;

// When set to true, only allocate from MEMBLOCK_KHO_SCRATCH ranges
    static bool kho_scratch_only;

    static struct memblock_region memblock_memory_init_regions[INIT_MEMBLOCK_MEMORY_REGIONS] __initdata_memblock;
    static struct memblock_region memblock_reserved_init_regions[INIT_MEMBLOCK_RESERVED_REGIONS] __initdata_memblock;

    static struct memblock_region memblock_physmem_init_regions[INIT_PHYSMEM_REGIONS];

    struct memblock memblock __initdata_memblock = {
    .memory.regions		= memblock_memory_init_regions,
    .memory.max		= INIT_MEMBLOCK_MEMORY_REGIONS,
    .memory.name		= "memory",
    .reserved.regions	= memblock_reserved_init_regions,
    .reserved.max		= INIT_MEMBLOCK_RESERVED_REGIONS,
    .reserved.name		= "reserved",
    .bottom_up		= false,
    .current_limit		= MEMBLOCK_ALLOC_ANYWHERE,
    };

pub static mut memblock_type: usize = 0;

//
// keep a pointer to &memblock.memory in the text section to use it in
// __next_mem_range() and its helpers.
// For architectures that do not keep memblock data after init, this
// pointer will be reset to NULL at memblock_discard()
//
    static __refdata struct memblock_type *memblock_memory = &memblock.memory;

    for (i = 0, rgn = &memblock_type.regions[0];			
    i < memblock_type.cnt;					
    i++, rgn = &memblock_type.regions[i]) {

    do {								
    }
    if (memblock_debug)					 {
    pr_info!(fmt, ##__VA_ARGS__);			
    }
    } while (0)
    static int memblock_debug __initdata_memblock;
    static bool system_has_some_mirror __initdata_memblock;
    static int memblock_can_resize __initdata_memblock;
    static int memblock_memory_in_slab __initdata_memblock;
    static int memblock_reserved_in_slab __initdata_memblock;
#[no_mangle]
pub unsafe extern "C" fn memblock_has_mirror() -> bool __init_memblock {
    return system_has_some_mirror;
    }
#[no_mangle]
unsafe extern "C" fn choose_memblock_flags() -> enum memblock_flags __init_memblock {
// skip non-scratch memory for kho early boot allocations
    if (kho_scratch_only) {
    return MEMBLOCK_KHO_SCRATCH;
    }
    return system_has_some_mirror ? MEMBLOCK_MIRROR : MEMBLOCK_NONE;
    }
// adjust *@size so that (@base + *@size) doesn't overflow, return new size
#[no_mangle]
pub unsafe extern "C" fn memblock_cap_size(base: phys_addr_t, size: *mut phys_addr_t) -> phys_addr_t {
    let mut size = min(*size, PHYS_ADDR_MAX - base);
    }
//
// Address comparison utilities
//
    unsigned long __init_memblock
    memblock_addrs_overlap(phys_addr_t base1, phys_addr_t size1, phys_addr_t base2,
    phys_addr_t size2)
    {
    return ((base1 < (base2 + size2)) && (base2 < (base1 + size1)));
    }
    bool __init_memblock memblock_overlaps_region(memblock_type *type,
    phys_addr_t base, phys_addr_t size)
    {
    let mut i = 0;
    memblock_cap_size(base, &size);
    for (i = 0; i < type.cnt; i++) {
    if (memblock_addrs_overlap(base, size, type.regions[i].base,
    type.regions[i].size))
    return true;
    }
    return false;
    }
//
// __memblock_find_range_bottom_up - find free area utility in bottom-up
// @start: start of candidate range
// @end: end of candidate range, can be %MEMBLOCK_ALLOC_ANYWHERE or
// %MEMBLOCK_ALLOC_ACCESSIBLE
// @size: size of free area to find
// @align: alignment of free area to find
// @nid: nid of the free area to find, %NUMA_NO_NODE for any node
// @flags: pick from blocks based on memory attributes
//
// Utility called from memblock_find_in_range_node(), find free area bottom-up.
//
// Return:
// Found address on success, 0 on failure.
//
    static phys_addr_t __init_memblock
    __memblock_find_range_bottom_up(phys_addr_t start, phys_addr_t end,
    phys_addr_t size, phys_addr_t align, int nid,
    enum memblock_flags flags)
    {
    phys_addr_t this_start, this_end, cand;
    let mut i = 0;
    for_each_free_mem_range(i, nid, flags, &this_start, &this_end, core::ptr::null_mut()) {
    this_start = clamp(this_start, start, end);
    this_end = clamp(this_end, start, end);
    cand = round_up(this_start, align);
    if (cand < this_end && this_end - cand >= size) {
    return cand;
    }
    }
    return 0;
    }
//
// __memblock_find_range_top_down - find free area utility, in top-down
// @start: start of candidate range
// @end: end of candidate range, can be %MEMBLOCK_ALLOC_ANYWHERE or
// %MEMBLOCK_ALLOC_ACCESSIBLE
// @size: size of free area to find
// @align: alignment of free area to find
// @nid: nid of the free area to find, %NUMA_NO_NODE for any node
// @flags: pick from blocks based on memory attributes
//
// Utility called from memblock_find_in_range_node(), find free area top-down.
//
// Return:
// Found address on success, 0 on failure.
//
    static phys_addr_t __init_memblock
    __memblock_find_range_top_down(phys_addr_t start, phys_addr_t end,
    phys_addr_t size, phys_addr_t align, int nid,
    enum memblock_flags flags)
    {
    phys_addr_t this_start, this_end, cand;
    let mut i = 0;
    for_each_free_mem_range_reverse(i, nid, flags, &this_start, &this_end,
    core::ptr::null_mut()) {
    this_start = clamp(this_start, start, end);
    this_end = clamp(this_end, start, end);
    if (this_end < size) {
    continue;
    }
    cand = round_down(this_end - size, align);
    if (cand >= this_start) {
    return cand;
    }
    }
    return 0;
    }
//
// memblock_find_in_range_node - find free area in given range and node
// @size: size of free area to find
// @align: alignment of free area to find
// @start: start of candidate range
// @end: end of candidate range, can be %MEMBLOCK_ALLOC_ANYWHERE or
// %MEMBLOCK_ALLOC_ACCESSIBLE
// @nid: nid of the free area to find, %NUMA_NO_NODE for any node
// @flags: pick from blocks based on memory attributes
//
// Find @size free area aligned to @align in the specified range and node.
//
// Return:
// Found address on success, 0 on failure.
//
    static phys_addr_t __init_memblock memblock_find_in_range_node(phys_addr_t size,
    phys_addr_t align, phys_addr_t start,
    phys_addr_t end, int nid,
    enum memblock_flags flags)
    {
// pump up @end
    if (end == MEMBLOCK_ALLOC_ACCESSIBLE ||
    end == MEMBLOCK_ALLOC_NOLEAKTRACE) {
    end = memblock.current_limit;
    }
// avoid allocating the first page
    start = max_t(phys_addr_t, start, PAGE_SIZE);
    end = max(start, end);
    if (memblock_bottom_up()) {
    return __memblock_find_range_bottom_up(start, end, size, align,
    nid, flags);
    }
    else {
    return __memblock_find_range_top_down(start, end, size, align,
    nid, flags);
    }
    }
//
// memblock_find_in_range - find free area in given range
// @start: start of candidate range
// @end: end of candidate range, can be %MEMBLOCK_ALLOC_ANYWHERE or
// %MEMBLOCK_ALLOC_ACCESSIBLE
// @size: size of free area to find
// @align: alignment of free area to find
//
// Find @size free area aligned to @align in the specified range.
//
// Return:
// Found address on success, 0 on failure.
//
    static phys_addr_t __init_memblock memblock_find_in_range(phys_addr_t start,
    phys_addr_t end, phys_addr_t size,
    phys_addr_t align)
    {
    let mut ret;
pub static mut flags: memblock_flags = 0;
// label;
    ret = memblock_find_in_range_node(size, align, start, end,
    NUMA_NO_NODE, flags);
    if (!ret && (flags & MEMBLOCK_MIRROR)) {
    pr_warn_ratelimited("Could not allocate %pap bytes of mirrored memory\n",
    &size);
    flags &= ~MEMBLOCK_MIRROR;
// goto;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn memblock_remove_region(type: *mut memblock_type, r: c_ulong) _memblock {
    type.total_size -= type.regions[r].size;
    memmove(&type.regions[r], &type.regions[r + 1],
    (type.cnt - (r + 1)) * sizeof!(type.regions[r]));
    type.cnt -= 1;
// Special case for empty arrays
    if (type.cnt == 0) {
    WARN_ON!(type.total_size != 0);
    type.regions[0].base = 0;
    type.regions[0].size = 0;
    type.regions[0].flags = 0;
    memblock_set_region_node(&type.regions[0], MAX_NUMNODES);
    }
    }

//
// memblock_discard - discard memory and reserved arrays if they were allocated
//
#[no_mangle]
pub unsafe extern "C" fn memblock_discard()  {
    let mut size;
pub static mut addr: *mut c_void = core::ptr::null_mut();
    if (memblock.reserved.regions != memblock_reserved_init_regions) {
    addr = memblock.reserved.regions;
    size = PAGE_ALIGN(sizeof!(memblock_region) *
    memblock.reserved.max);
    if (memblock_reserved_in_slab) {
    kfree(addr);
    }
    else {
    memblock_free(addr, size);
    }
    }
    if (memblock.memory.regions != memblock_memory_init_regions) {
    addr = memblock.memory.regions;
    size = PAGE_ALIGN(sizeof!(memblock_region) *
    memblock.memory.max);
    if (memblock_memory_in_slab) {
    kfree(addr);
    }
    else {
    memblock_free(addr, size);
    }
    }
    memblock_memory = core::ptr::null_mut();
    }

//
// memblock_double_array - double the size of the memblock regions array
// @type: memblock type of the regions array being doubled
// @new_area_start: starting address of memory range to avoid overlap with
// @new_area_size: size of memory range to avoid overlap with
//
// Double the size of the @type regions array. If memblock is being used to
// allocate memory for a new reserved regions array and there is a previously
// allocated memory range [@new_area_start, @new_area_start + @new_area_size]
// waiting to be reserved, ensure the memory used by the new array does
// not overlap.
//
// Return:
// 0 on success, -1 on failure.
//
    static int __init_memblock memblock_double_array(memblock_type *type,
    phys_addr_t new_area_start,
    phys_addr_t new_area_size)
    {
    let mut new_array = core::ptr::null_mut();
    let mut old_array = core::ptr::null_mut();
    phys_addr_t old_alloc_size, new_alloc_size;
    phys_addr_t old_size, new_size, addr, new_end;
pub static mut use_slab: c_int = 0;
pub static mut in_slab: *mut c_void = core::ptr::null_mut();
// We don't allow resizing until we know about the reserved regions
// of memory that aren't suitable for allocation
//
    if (!memblock_can_resize) {
    panic("memblock: cannot resize %s array\n", type.name);
    }
// Calculate new doubled size
    old_size = type.max * sizeof!(memblock_region);
    new_size = old_size << 1;
//
// We need to allocated new one align to PAGE_SIZE,
// so we can free them completely later.
//
    old_alloc_size = PAGE_ALIGN(old_size);
    new_alloc_size = PAGE_ALIGN(new_size);
// Retrieve the slab flag
    if (type == &memblock.memory) {
    in_slab = &memblock_memory_in_slab;
    }
    else {
    in_slab = &memblock_reserved_in_slab;
    }
// Try to find some space for it
    if (use_slab) {
    new_array = kmalloc(new_size, GFP_KERNEL);
    addr = new_array ? __pa(new_array) : 0;
    } else {
// only exclude range when trying to double reserved.regions
    if (type != &memblock.reserved) {
    new_area_start = new_area_size = 0;
    }
    addr = memblock_find_in_range(new_area_start + new_area_size,
    memblock.current_limit,
    new_alloc_size, PAGE_SIZE);
    if (!addr && new_area_size) {
    addr = memblock_find_in_range(0,
    min(new_area_start, memblock.current_limit),
    new_alloc_size, PAGE_SIZE);
    }
    if (addr) {
// The memory may not have been accepted, yet.
    accept_memory(addr, new_alloc_size);
    new_array = __va(addr);
    } else {
    new_array = core::ptr::null_mut();
    }
    }
    if (!addr) {
    pr_err!("memblock: Failed to double %s array from %ld to %ld entries !\n",
    type.name, type.max, type.max * 2);
    return -1;
    }
    new_end = addr + new_size - 1;
    memblock_dbg("memblock: %s is doubled to %ld at [%pa-%pa]",
    type.name, type.max * 2, &addr, &new_end);
//
// Found space, we now need to move the array over before we add the
// reserved region since it may be our reserved array itself that is
// full.
//
    memcpy(new_array, type.regions, old_size);
    memset(new_array + type.max, 0, old_size);
    old_array = type.regions;
    type.regions = new_array;
    type.max <<= 1;
// Free old array. We needn't free it if the array is the static one
    if (*in_slab) {
    kfree(old_array);
    }
    else if (old_array != memblock_memory_init_regions &&
    old_array != memblock_reserved_init_regions) {
    memblock_free(old_array, old_alloc_size);
    }
//
// Reserve the new array if that comes from the memblock.  Otherwise, we
// needn't do it
//
    if (!use_slab) {
    BUG_ON!(memblock_reserve_kern(addr, new_alloc_size));
    }
// Update slab flag
// in_slab = use_slab;
    return 0;
    }
//
// memblock_merge_regions - merge neighboring compatible regions
// @type: memblock type to scan
// @start_rgn: start scanning from (@start_rgn - 1)
// @end_rgn: end scanning at (@end_rgn - 1)
// Scan @type and merge neighboring compatible regions in [@start_rgn - 1, @end_rgn)
//
    static void __init_memblock memblock_merge_regions(memblock_type *type,
    unsigned long start_rgn,
    unsigned long end_rgn)
    {
pub static mut i: c_int = 0;
    if (start_rgn) {
    i = start_rgn - 1;
    }
    end_rgn = min(end_rgn, type.cnt - 1);
    while (i < end_rgn) {
    let mut this = &type.regions[i];
    let mut next = &type.regions[i + 1];
    if (this.base + this.size != next.base ||
    memblock_get_region_node(this) !=
    memblock_get_region_node(next) ||
    this.flags != next.flags) {
    BUG_ON!(this.base + this.size > next.base);
    i += 1;
    continue;
    }
    this.size += next.size;
// move forward from next + 1, index of which is i + 2
    memmove(next, next + 1, (type.cnt - (i + 2)) * sizeof!(*next));
    type.cnt -= 1;
    end_rgn -= 1;
    }
    }
//
// memblock_insert_region - insert new memblock region
// @type:	memblock type to insert into
// @idx:	index for the insertion point
// @base:	base address of the new region
// @size:	size of the new region
// @nid:	node id of the new region
// @flags:	flags of the new region
//
// Insert new memblock region [@base, @base + @size) into @type at @idx.
// @type must already have extra room to accommodate the new region.
//
    static void __init_memblock memblock_insert_region(memblock_type *type,
    int idx, phys_addr_t base,
    phys_addr_t size,
    int nid,
    enum memblock_flags flags)
    {
    let mut rgn = &type.regions[idx];
    BUG_ON!(type.cnt >= type.max);
    memmove(rgn + 1, rgn, (type.cnt - idx) * sizeof!(*rgn));
    rgn.base = base;
    rgn.size = size;
    rgn.flags = flags;
    memblock_set_region_node(rgn, nid);
    type.cnt += 1;
    type.total_size += size;
    }
//
// memblock_add_range - add new memblock region
// @type: memblock type to add new region into
// @base: base address of the new region
// @size: size of the new region
// @nid: nid of the new region
// @flags: flags of the new region
//
// Add new memblock region [@base, @base + @size) into @type.  The new region
// is allowed to overlap with existing ones - overlaps don't affect already
// existing regions.  @type is guaranteed to be minimal (all neighbouring
// compatible regions are merged) after the addition.
//
// Return:
// 0 on success, -errno on failure.
//
    static int __init_memblock memblock_add_range(memblock_type *type,
    phys_addr_t base, phys_addr_t size,
    int nid, enum memblock_flags flags)
    {
pub static mut insert: bool = false;
pub static mut obase: phys_addr_t = 0;
pub static mut end: phys_addr_t = 0;
    int idx, nr_new, start_rgn = -1, end_rgn;
pub static mut rgn: *mut c_void = core::ptr::null_mut();
    if (!size) {
    return 0;
    }
// special case for empty array
    if (type.regions[0].size == 0) {
    WARN_ON!(type.cnt != 0 || type.total_size);
    type.regions[0].base = base;
    type.regions[0].size = size;
    type.regions[0].flags = flags;
    memblock_set_region_node(&type.regions[0], nid);
    type.total_size = size;
    type.cnt = 1;
    return 0;
    }
//
// The worst case is when new range overlaps all existing regions,
// then we'll need type->cnt + 1 empty regions in @type. So if
// type->cnt * 2 + 1 is less than or equal to type->max, we know
// that there is enough empty regions in @type, and we can insert
// regions directly.
//
    if (type.cnt * 2 + 1 <= type.max) {
    insert = true;
    }
// label;
//
// The following is executed twice.  Once with %false @insert and
// then with %true.  The first counts the number of regions needed
// to accommodate the new area.  The second actually inserts them.
//
    base = obase;
    nr_new = 0;
    for_each_memblock_type(idx, type, rgn) {
pub static mut rbase: phys_addr_t = 0;
pub static mut rend: phys_addr_t = 0;
    if (rbase >= end) {
    break;
    }
    if (rend <= base) {
    continue;
    }
//
// @rgn overlaps.  If it separates the lower part of new
// area, insert that portion.
//
    if (rbase > base) {

    WARN_ON!(nid != memblock_get_region_node(rgn));

    WARN_ON!(flags != MEMBLOCK_NONE && flags != rgn.flags);
    nr_new += 1;
    if (insert) {
    if (start_rgn == -1) {
    start_rgn = idx;
    }
    end_rgn = idx + 1;
    memblock_insert_region(type, idx++, base,
    rbase - base, nid,
    flags);
    }
    }
// area below @rend is dealt with, forget about it
    base = min(rend, end);
    }
// insert the remaining portion
    if (base < end) {
    nr_new += 1;
    if (insert) {
    if (start_rgn == -1) {
    start_rgn = idx;
    }
    end_rgn = idx + 1;
    memblock_insert_region(type, idx, base, end - base,
    nid, flags);
    }
    }
    if (!nr_new) {
    return 0;
    }
//
// If this was the first round, resize array and repeat for actual
// insertions; otherwise, merge and return.
//
    if (!insert) {
    while (type.cnt + nr_new > type.max) {
    if (memblock_double_array(type, obase, size) < 0)
    return -ENOMEM;
    }
    insert = true;
// goto;
    } else {
    memblock_merge_regions(type, start_rgn, end_rgn);
    return 0;
    }
    }
//
// memblock_add_node - add new memblock region within a NUMA node
// @base: base address of the new region
// @size: size of the new region
// @nid: nid of the new region
// @flags: flags of the new region
//
// Add new memblock region [@base, @base + @size) to the "memory"
// type. See memblock_add_range() description for mode details
//
// Return:
// 0 on success, -errno on failure.
//
    int __init_memblock memblock_add_node(phys_addr_t base, phys_addr_t size,
    int nid, enum memblock_flags flags)
    {
pub static mut end: phys_addr_t = 0;
    memblock_dbg("%s: [%pa-%pa] nid=%d flags=%x %pS\n", __func__,
    &base, &end, nid, flags, _RET_IP_);
    return memblock_add_range(&memblock.memory, base, size, nid, flags);
    }
//
// memblock_add - add new memblock region
// @base: base address of the new region
// @size: size of the new region
//
// Add new memblock region [@base, @base + @size) to the "memory"
// type. See memblock_add_range() description for mode details
//
// Return:
// 0 on success, -errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_add(base: phys_addr_t, size: phys_addr_t) -> c_int_memblock {
pub static mut end: phys_addr_t = 0;
    memblock_dbg("%s: [%pa-%pa] %pS\n", __func__,
    &base, &end, _RET_IP_);
    return memblock_add_range(&memblock.memory, base, size, MAX_NUMNODES, 0);
    }
//
// memblock_validate_numa_coverage - check if amount of memory with
// no node ID assigned is less than a threshold
// @threshold_bytes: maximal memory size that can have unassigned node
// ID (in bytes).
//
// A buggy firmware may report memory that does not belong to any node.
// Check if amount of such memory is below @threshold_bytes.
//
// Return: true on success, false on failure.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_validate_numa_coverage(threshold_bytes: c_ulong) -> bool __init_memblock {
pub static mut nr_pages: c_ulong = 0;
    unsigned long start_pfn, end_pfn, mem_size_mb;
    let mut nid = 0;
    let mut i = 0;
// calculate lost page
    for_each_mem_pfn_range(i, MAX_NUMNODES, &start_pfn, &end_pfn, &nid) {
    if (!numa_valid_node(nid)) {
    nr_pages += end_pfn - start_pfn;
    }
    }
    if ((nr_pages << PAGE_SHIFT) > threshold_bytes) {
    mem_size_mb = memblock_phys_mem_size() / SZ_1M;
    pr_err!("NUMA: no nodes coverage for %luMB of %luMB RAM\n",
    (nr_pages << PAGE_SHIFT) / SZ_1M, mem_size_mb);
    return false;
    }
    return true;
    }
//
// memblock_isolate_range - isolate given range into disjoint memblocks
// @type: memblock type to isolate range for
// @base: base of range to isolate
// @size: size of range to isolate
// @start_rgn: out parameter for the start of isolated region
// @end_rgn: out parameter for the end of isolated region
//
// Walk @type and ensure that regions don't cross the boundaries defined by
// [@base, @base + @size).  Crossing regions are split at the boundaries,
// which may create at most two more regions.  The index of the first
// region inside the range is returned in *@start_rgn and the index of the
// first region after the range is returned in *@end_rgn.
//
// Return:
// 0 on success, -errno on failure.
//
    static int __init_memblock memblock_isolate_range(memblock_type *type,
    phys_addr_t base, phys_addr_t size,
    int *start_rgn, int *end_rgn)
    {
pub static mut end: phys_addr_t = 0;
    let mut idx = 0;
pub static mut rgn: *mut c_void = core::ptr::null_mut();
// start_rgn = *end_rgn = 0;
    if (!size) {
    return 0;
    }
// we'll create at most two more regions
    while (type.cnt + 2 > type.max) {
    if (memblock_double_array(type, base, size) < 0)
    return -ENOMEM;
    }
    for_each_memblock_type(idx, type, rgn) {
pub static mut rbase: phys_addr_t = 0;
pub static mut rend: phys_addr_t = 0;
    if (rbase >= end) {
    break;
    }
    if (rend <= base) {
    continue;
    }
    if (rbase < base) {
//
// @rgn intersects from below.  Split and continue
// to process the next region - the new top half.
//
    rgn.base = base;
    rgn.size -= base - rbase;
    type.total_size -= base - rbase;
    memblock_insert_region(type, idx, rbase, base - rbase,
    memblock_get_region_node(rgn),
    rgn.flags);
    } else if (rend > end) {
//
// @rgn intersects from above.  Split and redo the
// current region - the new bottom half.
//
    rgn.base = end;
    rgn.size -= end - rbase;
    type.total_size -= end - rbase;
    memblock_insert_region(type, idx--, rbase, end - rbase,
    memblock_get_region_node(rgn),
    rgn.flags);
    } else {
// @rgn is fully contained, record it
    if (!*end_rgn) {
// start_rgn = idx;
    }
// end_rgn = idx + 1;
    }
    }
    return 0;
    }
    static int __init_memblock memblock_remove_range(memblock_type *type,
    phys_addr_t base, phys_addr_t size)
    {
    let mut start_rgn = 0;
    let mut end_rgn = 0;
    let mut i = 0;
    let mut ret = 0;
    ret = memblock_isolate_range(type, base, size, &start_rgn, &end_rgn);
    if (ret) {
    return ret;
    }
    for (i = end_rgn - 1; i >= start_rgn; i--) {
    memblock_remove_region(type, i);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_remove(base: phys_addr_t, size: phys_addr_t) -> c_int_memblock {
pub static mut end: phys_addr_t = 0;
    memblock_dbg("%s: [%pa-%pa] %pS\n", __func__,
    &base, &end, _RET_IP_);
    return memblock_remove_range(&memblock.memory, base, size);
    }
#[no_mangle]
pub unsafe extern "C" fn __free_reserved_area(start: phys_addr_t, end: phys_addr_t, poison: c_int) -> c_ulong {
pub static mut pages: c_ulong = 0;
    if (deferred_pages_enabled()) {
    WARN(1, "Cannot free reserved memory because of deferred initialization of the memory map");
    return 0;
    }
    for_each_valid_pfn(pfn, PFN_UP(start), PFN_DOWN(end)) {
    let mut page = pfn_to_page(pfn);
pub static mut direct_map_addr: *mut c_void = core::ptr::null_mut();
//
// 'direct_map_addr' might be different from the kernel virtual
// address because some architectures use aliases.
// Going via physical address, pfn_to_page() and page_address()
// ensures that we get a _writeable_ alias for the memset().
//
    direct_map_addr = page_address(page);
//
// Perform a kasan-unchecked memset() since this memory
// has not been initialized.
//
    direct_map_addr = kasan_reset_tag(direct_map_addr);
    if ((unsigned int)poison <= 0xFF) {
    memset(direct_map_addr, poison, PAGE_SIZE);
    }
    free_reserved_page(page);
    pages += 1;
    }
    return pages;
    }
#[no_mangle]
pub unsafe extern "C" fn free_reserved_area(start: *mut c_void, end: *mut c_void, poison: c_int, s: *const c_char) -> c_ulong {
    phys_addr_t start_pa, end_pa;
    let mut pages = 0;
//
// end is the first address past the region and it may be beyond what
// __pa() or __pa_symbol() can handle.
// Use the address included in the range for the conversion and add back
// 1 afterwards.
//
    if (__is_kernel((unsigned long)start)) {
    start_pa = __pa_symbol(start);
    end_pa = __pa_symbol(end - 1) + 1;
    } else {
    start_pa = __pa(start);
    end_pa = __pa(end - 1) + 1;
    }
    if (IS_ENABLED!(CONFIG_ARCH_KEEP_MEMBLOCK)) {
    if (start_pa < end_pa) {
    memblock_remove_range(&memblock.reserved,
    start_pa, end_pa - start_pa);
    }
    }
    pages = __free_reserved_area(start_pa, end_pa, poison);
    if (pages && s) {
    pr_info!("Freeing %s memory: %ldK\n", s, K(pages));
    }
    return pages;
    }
//
// memblock_free - free boot memory allocation
// @ptr: starting address of the  boot memory allocation
// @size: size of the boot memory block in bytes
//
// Free boot memory block previously allocated by memblock_alloc_xx() API.
// If called after the buddy allocator is available, the memory is released to
// the buddy allocator.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_free(ptr: *mut c_void, size: usize) _memblock {
    if (ptr) {
    memblock_phys_free(__pa(ptr), size);
    }
    }
//
// memblock_phys_free - free boot memory block
// @base: phys starting address of the  boot memory block
// @size: size of the boot memory block in bytes
//
// Free boot memory block previously allocated by memblock_phys_alloc_xx() API.
// If called after the buddy allocator is available, the memory is released to
// the buddy allocator.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_phys_free(base: phys_addr_t, size: phys_addr_t) -> c_int_memblock {
pub static mut end: phys_addr_t = 0;
pub static mut ret: c_int = 0;
    memblock_dbg("%s: [%pa-%pa] %pS\n", __func__,
    &base, &end, _RET_IP_);
    kmemleak_free_part_phys(base, size);
    if (!slab_is_available() || IS_ENABLED!(CONFIG_ARCH_KEEP_MEMBLOCK)) {
    ret = memblock_remove_range(&memblock.reserved, base, size);
    }
    if (slab_is_available()) {
    __free_reserved_area(base, base + size, -1);
    }
    return ret;
    }
    int __init_memblock __memblock_reserve(phys_addr_t base, phys_addr_t size,
    int nid, enum memblock_flags flags)
    {
pub static mut end: phys_addr_t = 0;
    memblock_dbg("%s: [%pa-%pa] nid=%d flags=%x %pS\n", __func__,
    &base, &end, nid, flags, _RET_IP_);
    return memblock_add_range(&memblock.reserved, base, size, nid, flags);
    }

#[no_mangle]
pub unsafe extern "C" fn memblock_physmem_add(base: phys_addr_t, size: phys_addr_t) -> c_int_memblock {
pub static mut end: phys_addr_t = 0;
    memblock_dbg("%s: [%pa-%pa] %pS\n", __func__,
    &base, &end, _RET_IP_);
    return memblock_add_range(&physmem, base, size, MAX_NUMNODES, 0);
    }

//
// memblock_setclr_flag - set or clear flag for a memory region
// @type: memblock type to set/clear flag for
// @base: base address of the region
// @size: size of the region
// @set: set or clear the flag
// @flag: the flag to update
//
// This function isolates region [@base, @base + @size), and sets/clears flag
//
// Return: 0 on success, -errno on failure.
//
    static int __init_memblock memblock_setclr_flag(memblock_type *type,
    phys_addr_t base, phys_addr_t size, int set, int flag)
    {
    let mut i = 0;
    let mut ret = 0;
    let mut start_rgn = 0;
    let mut end_rgn = 0;
    ret = memblock_isolate_range(type, base, size, &start_rgn, &end_rgn);
    if (ret) {
    return ret;
    }
    while (i < end_rgn) {
    let mut r = &type.regions[i];
    if (set) {
    r.flags |= flag;
    }
    else {
    r.flags &= ~flag;
    }
    }
    memblock_merge_regions(type, start_rgn, end_rgn);
    return 0;
    }
//
// memblock_mark_hotplug - Mark hotpluggable memory with flag MEMBLOCK_HOTPLUG.
// @base: the base phys addr of the region
// @size: the size of the region
//
// Return: 0 on success, -errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_mark_hotplug(base: phys_addr_t, size: phys_addr_t) -> c_int_memblock {
    return memblock_setclr_flag(&memblock.memory, base, size, 1, MEMBLOCK_HOTPLUG);
    }
//
// memblock_clear_hotplug - Clear flag MEMBLOCK_HOTPLUG for a specified region.
// @base: the base phys addr of the region
// @size: the size of the region
//
// Return: 0 on success, -errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_clear_hotplug(base: phys_addr_t, size: phys_addr_t) -> c_int_memblock {
    return memblock_setclr_flag(&memblock.memory, base, size, 0, MEMBLOCK_HOTPLUG);
    }
//
// memblock_mark_mirror - Mark mirrored memory with flag MEMBLOCK_MIRROR.
// @base: the base phys addr of the region
// @size: the size of the region
//
// Return: 0 on success, -errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_mark_mirror(base: phys_addr_t, size: phys_addr_t) -> c_int_memblock {
    if (!mirrored_kernelcore) {
    return 0;
    }
    system_has_some_mirror = true;
    return memblock_setclr_flag(&memblock.memory, base, size, 1, MEMBLOCK_MIRROR);
    }
//
// memblock_mark_nomap - Mark a memory region with flag MEMBLOCK_NOMAP.
// @base: the base phys addr of the region
// @size: the size of the region
//
// The memory regions marked with %MEMBLOCK_NOMAP will not be added to the
// direct mapping of the physical memory. These regions will still be
// covered by the memory map. The struct page representing NOMAP memory
// frames in the memory map will be PageReserved()
//
// Note: if the memory being marked %MEMBLOCK_NOMAP was allocated from
// memblock, the caller must inform kmemleak to ignore that memory
//
// Return: 0 on success, -errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_mark_nomap(base: phys_addr_t, size: phys_addr_t) -> c_int_memblock {
    return memblock_setclr_flag(&memblock.memory, base, size, 1, MEMBLOCK_NOMAP);
    }
//
// memblock_clear_nomap - Clear flag MEMBLOCK_NOMAP for a specified region.
// @base: the base phys addr of the region
// @size: the size of the region
//
// Return: 0 on success, -errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_clear_nomap(base: phys_addr_t, size: phys_addr_t) -> c_int_memblock {
    return memblock_setclr_flag(&memblock.memory, base, size, 0, MEMBLOCK_NOMAP);
    }
//
// memblock_reserved_mark_noinit - Mark a reserved memory region with flag
// MEMBLOCK_RSRV_NOINIT
//
// @base: the base phys addr of the region
// @size: the size of the region
//
// The struct pages for the reserved regions marked %MEMBLOCK_RSRV_NOINIT will
// not be fully initialized to allow the caller optimize their initialization.
//
// When %CONFIG_DEFERRED_STRUCT_PAGE_INIT is enabled, setting this flag
// completely bypasses the initialization of struct pages for such region.
//
// When %CONFIG_DEFERRED_STRUCT_PAGE_INIT is disabled, pages in this
// region will be initialized with default values but won't be marked as
// reserved.
//
// Return: 0 on success, -errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_reserved_mark_noinit(base: phys_addr_t, size: phys_addr_t) -> c_int_memblock {
    return memblock_setclr_flag(&memblock.reserved, base, size, 1,
    MEMBLOCK_RSRV_NOINIT);
    }
//
// memblock_reserved_mark_kern - Mark a reserved memory region with flag
// MEMBLOCK_RSRV_KERN
//
// @base: the base phys addr of the region
// @size: the size of the region
//
// Return: 0 on success, -errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_reserved_mark_kern(base: phys_addr_t, size: phys_addr_t) -> c_int_memblock {
    return memblock_setclr_flag(&memblock.reserved, base, size, 1,
    MEMBLOCK_RSRV_KERN);
    }
//
// memblock_mark_kho_scratch - Mark a memory region as MEMBLOCK_KHO_SCRATCH.
// @base: the base phys addr of the region
// @size: the size of the region
//
// Only memory regions marked with %MEMBLOCK_KHO_SCRATCH will be considered
// for allocations during early boot with kexec handover.
//
// Return: 0 on success, -errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_mark_kho_scratch(base: phys_addr_t, size: phys_addr_t) -> __init int {
    return memblock_setclr_flag(&memblock.memory, base, size, 1,
    MEMBLOCK_KHO_SCRATCH);
    }
//
// memblock_clear_kho_scratch - Clear MEMBLOCK_KHO_SCRATCH flag for a
// specified region.
// @base: the base phys addr of the region
// @size: the size of the region
//
// Return: 0 on success, -errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_clear_kho_scratch(base: phys_addr_t, size: phys_addr_t) -> __init int {
    return memblock_setclr_flag(&memblock.memory, base, size, 0,
    MEMBLOCK_KHO_SCRATCH);
    }
#[no_mangle]
pub unsafe extern "C" fn should_skip_region(type: *mut memblock_type, m: *mut memblock_region, nid: c_int, flags: c_int) -> bool {
pub static mut m_nid: c_int = 0;
// we never skip regions when iterating memblock.reserved or physmem
    if (type != memblock_memory) {
    return false;
    }
// only memory regions are associated with nodes, check it
    if (numa_valid_node(nid) && nid != m_nid) {
    return true;
    }
// skip hotpluggable memory regions if needed
    if (movable_node_is_enabled() && memblock_is_hotpluggable(m) &&
    !(flags & MEMBLOCK_HOTPLUG)) {
    return true;
    }
// if we want mirror memory skip non-mirror memory regions
    if ((flags & MEMBLOCK_MIRROR) && !memblock_is_mirror(m)) {
    return true;
    }
// skip nomap memory unless we were asked for it explicitly
    if (!(flags & MEMBLOCK_NOMAP) && memblock_is_nomap(m)) {
    return true;
    }
// skip driver-managed memory unless we were asked for it explicitly
    if (!(flags & MEMBLOCK_DRIVER_MANAGED) && memblock_is_driver_managed(m)) {
    return true;
    }
//
// In early alloc during kexec handover, we can only consider
// MEMBLOCK_KHO_SCRATCH regions for the allocations
//
    if ((flags & MEMBLOCK_KHO_SCRATCH) && !memblock_is_kho_scratch(m)) {
    return true;
    }
    return false;
    }
//
// __next_mem_range - next function for for_each_free_mem_range() etc.
// @idx: pointer to u64 loop variable
// @nid: node selector, %NUMA_NO_NODE for all nodes
// @flags: pick from blocks based on memory attributes
// @type_a: pointer to memblock_type from where the range is taken
// @type_b: pointer to memblock_type which excludes memory from being taken
// @out_start: ptr to phys_addr_t for start address of the range, can be %NULL
// @out_end: ptr to phys_addr_t for end address of the range, can be %NULL
// @out_nid: ptr to int for nid of the range, can be %NULL
//
// Find the first area from *@idx which matches @nid, fill the out
// parameters, and update *@idx for the next iteration.  The lower 32bit of
// *@idx contains index into type_a and the upper 32bit indexes the
// areas before each region in type_b.	For example, if type_b regions
// look like the following,
//
// 0:[0-16), 1:[32-48), 2:[128-130)
//
// The upper 32bit indexes the following regions.
//
// 0:[0-0), 1:[16-32), 2:[48-128), 3:[130-MAX)
//
// As both region arrays are sorted, the function advances the two indices
// in lockstep and returns each intersection.
//
#[no_mangle]
pub unsafe extern "C" fn __next_mem_range(idx: *mut u64, nid: c_int, flags: memblock_flags, type_a: *mut memblock_type, type_b: *mut memblock_type, out_start: *mut phys_addr_t, out_end: *mut phys_addr_t, out_nid: *mut c_int) {
pub static mut idx_a: c_int = 0;
pub static mut idx_b: c_int = 0;
    while (idx_a < type_a.cnt) {
    let mut m = &type_a.regions[idx_a];
pub static mut m_start: phys_addr_t = 0;
pub static mut m_end: phys_addr_t = 0;
pub static mut m_nid: c_int = 0;
    if (should_skip_region(type_a, m, nid, flags)) {
    continue;
    }
    if (!type_b) {
    if (out_start) {
// out_start = m_start;
    }
    if (out_end) {
// out_end = m_end;
    }
    if (out_nid) {
// out_nid = m_nid;
    }
    idx_a += 1;
// idx = (u32)idx_a | (u64)idx_b << 32;
    return;
    }
// scan areas before each reservation
    while (idx_b < type_b.cnt + 1) {
pub static mut r: *mut c_void = core::ptr::null_mut();
    let mut r_start;
    let mut r_end;
    r = &type_b.regions[idx_b];
    r_start = idx_b ? r[-1].base + r[-1].size : 0;
    r_end = idx_b < type_b.cnt ?
    r.base : PHYS_ADDR_MAX;
//
// if idx_b advanced past idx_a,
// break out to advance idx_a
//
    if (r_start >= m_end) {
    break;
    }
// if the two regions intersect, we're done
    if (m_start < r_end) {
    if (out_start) {
// out_start =
    max(m_start, r_start);
    }
    if (out_end) {
// out_end = min(m_end, r_end);
    }
    if (out_nid) {
// out_nid = m_nid;
    }
//
// The region which ends first is
// advanced for the next iteration.
//
    if (m_end <= r_end) {
    idx_a += 1;
    }
    else {
    idx_b += 1;
    }
// idx = (u32)idx_a | (u64)idx_b << 32;
    return;
    }
    }
    }
// signal end of iteration
// idx = ULLONG_MAX;
    }
//
// __next_mem_range_rev - generic next function for for_each_*_range_rev()
//
// @idx: pointer to u64 loop variable
// @nid: node selector, %NUMA_NO_NODE for all nodes
// @flags: pick from blocks based on memory attributes
// @type_a: pointer to memblock_type from where the range is taken
// @type_b: pointer to memblock_type which excludes memory from being taken
// @out_start: ptr to phys_addr_t for start address of the range, can be %NULL
// @out_end: ptr to phys_addr_t for end address of the range, can be %NULL
// @out_nid: ptr to int for nid of the range, can be %NULL
//
// Finds the next range from type_a which is not marked as unsuitable
// in type_b.
//
// Reverse of __next_mem_range().
//
    void __init_memblock __next_mem_range_rev(u64 *idx, int nid,
    enum memblock_flags flags, memblock_type *type_a, memblock_type *type_b,
    phys_addr_t *out_start,
    phys_addr_t *out_end, int *out_nid)
    {
pub static mut idx_a: c_int = 0;
pub static mut idx_b: c_int = 0;
    if (*idx == (u64)ULLONG_MAX) {
    idx_a = type_a.cnt - 1;
    if (type_b != core::ptr::null_mut()) {
    idx_b = type_b.cnt;
    }
    else {
    idx_b = 0;
    }
    }
    while (idx_a >= 0) {
    let mut m = &type_a.regions[idx_a];
pub static mut m_start: phys_addr_t = 0;
pub static mut m_end: phys_addr_t = 0;
pub static mut m_nid: c_int = 0;
    if (should_skip_region(type_a, m, nid, flags)) {
    continue;
    }
    if (!type_b) {
    if (out_start) {
// out_start = m_start;
    }
    if (out_end) {
// out_end = m_end;
    }
    if (out_nid) {
// out_nid = m_nid;
    }
    idx_a -= 1;
// idx = (u32)idx_a | (u64)idx_b << 32;
    return;
    }
// scan areas before each reservation
    while (idx_b >= 0) {
pub static mut r: *mut c_void = core::ptr::null_mut();
    let mut r_start;
    let mut r_end;
    r = &type_b.regions[idx_b];
    r_start = idx_b ? r[-1].base + r[-1].size : 0;
    r_end = idx_b < type_b.cnt ?
    r.base : PHYS_ADDR_MAX;
//
// if idx_b advanced past idx_a,
// break out to advance idx_a
//
    if (r_end <= m_start) {
    break;
    }
// if the two regions intersect, we're done
    if (m_end > r_start) {
    if (out_start) {
// out_start = max(m_start, r_start);
    }
    if (out_end) {
// out_end = min(m_end, r_end);
    }
    if (out_nid) {
// out_nid = m_nid;
    }
    if (m_start >= r_start) {
    idx_a -= 1;
    }
    else {
    idx_b -= 1;
    }
// idx = (u32)idx_a | (u64)idx_b << 32;
    return;
    }
    }
    }
// signal end of iteration
// idx = ULLONG_MAX;
    }
//
// Common iterator interface used to define for_each_mem_pfn_range().
//
    void __init_memblock __next_mem_pfn_range(int *idx, int nid,
    unsigned long *out_start_pfn,
    unsigned long *out_end_pfn, int *out_nid)
    {
    let mut type = &memblock.memory;
pub static mut r: *mut c_void = core::ptr::null_mut();
    let mut r_nid = 0;
    while (++*idx < type.cnt) {
    r = &type.regions[*idx];
    r_nid = memblock_get_region_node(r);
    if (PFN_UP(r.base) >= PFN_DOWN(r.base + r.size)) {
    continue;
    }
    if (!numa_valid_node(nid) || nid == r_nid) {
    break;
    }
    }
    if (*idx >= type.cnt) {
// idx = -1;
    return;
    }
    if (out_start_pfn) {
// out_start_pfn = PFN_UP(r->base);
    }
    if (out_end_pfn) {
// out_end_pfn = PFN_DOWN(r->base + r->size);
    }
    if (out_nid) {
// out_nid = r_nid;
    }
    }
//
// memblock_set_node - set node ID on memblock regions
// @base: base of area to set node ID for
// @size: size of area to set node ID for
// @type: memblock type to set node ID for
// @nid: node ID to set
//
// Set the nid of memblock @type regions in [@base, @base + @size) to @nid.
// Regions which cross the area boundaries are split as necessary.
//
// Return:
// 0 on success, -errno on failure.
//
    int __init_memblock memblock_set_node(phys_addr_t base, phys_addr_t size, memblock_type *type, int nid)
    {

    let mut start_rgn = 0;
    let mut end_rgn = 0;
    let mut i = 0;
    let mut ret = 0;
    ret = memblock_isolate_range(type, base, size, &start_rgn, &end_rgn);
    if (ret) {
    return ret;
    }
    for (i = start_rgn; i < end_rgn; i++) {
    memblock_set_region_node(&type.regions[i], nid);
    }
    memblock_merge_regions(type, start_rgn, end_rgn);

    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_prep_allocation(start: phys_addr_t, size: phys_addr_t, kmemleak_trace: bool) {
//
// Skip kmemleak for those places like kasan_init() and
// early_pgtable_alloc() due to high volume.
//
    if (kmemleak_trace) {
//
// Memblock allocated blocks are never reported as
// leaks. This is because many of these blocks are
// only referred via the physical address which is
// not looked up by kmemleak.
//
    kmemleak_alloc_phys(start, size, 0);
    }
//
// Some Virtual Machine platforms, such as Intel TDX or AMD SEV-SNP,
// require memory to be accepted before it can be used by the
// guest.
//
// Accept the memory of the allocated buffer.
//
    accept_memory(start, size);
    }
//
// memblock_alloc_range_nid - allocate boot memory block
// @size: size of memory block to be allocated in bytes
// @align: alignment of the region and block's size
// @start: the lower bound of the memory region to allocate (phys address)
// @end: the upper bound of the memory region to allocate (phys address)
// @nid: nid of the free area to find, %NUMA_NO_NODE for any node
// @exact_nid: control the allocation fall back to other nodes
//
// The allocation is performed from memory region limited by
// memblock.current_limit if @end == %MEMBLOCK_ALLOC_ACCESSIBLE.
//
// If the specified node can not hold the requested memory and @exact_nid
// is false, the allocation falls back to any node in the system.
//
// For systems with memory mirroring, the allocation is attempted first
// from the regions with mirroring enabled and then retried from any
// memory region.
//
// In addition, function using kmemleak_alloc_phys for allocated boot
// memory block, it is never reported as leaks.
//
// Return:
// Physical address of allocated memory block on success, %0 on failure.
//
    phys_addr_t __init memblock_alloc_range_nid(phys_addr_t size,
    phys_addr_t align, phys_addr_t start,
    phys_addr_t end, int nid,
    bool exact_nid)
    {
pub static mut flags: memblock_flags = 0;
    let mut found;
//
// Detect any accidental use of these APIs after slab is ready, as at
// this moment memblock may be deinitialized already and its
// internal data may be destroyed (after execution of memblock_free_all)
//
    if (WARN_ON_ONCE!(slab_is_available())) {
    let mut vaddr = kzalloc_node(size, GFP_NOWAIT, nid);
    return vaddr ? virt_to_phys(vaddr) : 0;
    }
    if (!align) {
// Can't use WARNs this early in boot on powerpc
    dump_stack();
    align = SMP_CACHE_BYTES;
    }
// label;
    found = memblock_find_in_range_node(size, align, start, end, nid,
    flags);
    if (found && !__memblock_reserve(found, size, nid, MEMBLOCK_RSRV_KERN)) {
// goto;
    }
    if (numa_valid_node(nid) && !exact_nid) {
    found = memblock_find_in_range_node(size, align, start,
    end, NUMA_NO_NODE,
    flags);
    if (found && !memblock_reserve_kern(found, size)) {
// goto;
    }
    }
    if (flags & MEMBLOCK_MIRROR) {
    flags &= ~MEMBLOCK_MIRROR;
    pr_warn_ratelimited("Could not allocate %pap bytes of mirrored memory\n",
    &size);
// goto;
    }
    return 0;
// label;
    memblock_prep_allocation(found, size, end != MEMBLOCK_ALLOC_NOLEAKTRACE);
    return found;
    }
//
// memblock_phys_alloc_range - allocate a memory block inside specified range
// @size: size of memory block to be allocated in bytes
// @align: alignment of the region and block's size
// @start: the lower bound of the memory region to allocate (physical address)
// @end: the upper bound of the memory region to allocate (physical address)
//
// Allocate @size bytes in the between @start and @end.
//
// Return: physical address of the allocated memory block on success,
// %0 on failure.
//
    phys_addr_t __init memblock_phys_alloc_range(phys_addr_t size,
    phys_addr_t align,
    phys_addr_t start,
    phys_addr_t end)
    {
    memblock_dbg("%s: %llu bytes align=0x%llx from=%pa max_addr=%pa %pS\n",
    __func__, (u64)size, (u64)align, &start, &end,
    _RET_IP_);
    return memblock_alloc_range_nid(size, align, start, end, NUMA_NO_NODE,
    false);
    }
//
// memblock_phys_alloc_try_nid - allocate a memory block from specified NUMA node
// @size: size of memory block to be allocated in bytes
// @align: alignment of the region and block's size
// @nid: nid of the free area to find, %NUMA_NO_NODE for any node
//
// Allocates memory block from the specified NUMA node. If the node
// has no available memory, attempts to allocated from any node in the
// system.
//
// Return: physical address of the allocated memory block on success,
// %0 on failure.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_phys_alloc_try_nid(size: phys_addr_t, align: phys_addr_t, nid: c_int) -> phys_addr_t __init {
    return memblock_alloc_range_nid(size, align, 0,
    MEMBLOCK_ALLOC_ACCESSIBLE, nid, false);
    }
//
// memblock_alloc_internal - allocate boot memory block
// @size: size of memory block to be allocated in bytes
// @align: alignment of the region and block's size
// @min_addr: the lower bound of the memory region to allocate (phys address)
// @max_addr: the upper bound of the memory region to allocate (phys address)
// @nid: nid of the free area to find, %NUMA_NO_NODE for any node
// @exact_nid: control the allocation fall back to other nodes
//
// Allocates memory block using memblock_alloc_range_nid() and
// converts the returned physical address to virtual.
//
// The @min_addr limit is dropped if it can not be satisfied and the allocation
// will fall back to memory below @min_addr. Other constraints, such
// as node and mirrored memory will be handled again in
// memblock_alloc_range_nid().
//
// Return:
// Virtual address of allocated memory block on success, NULL on failure.
//
    static void * __init memblock_alloc_internal(
    phys_addr_t size, phys_addr_t align,
    phys_addr_t min_addr, phys_addr_t max_addr,
    int nid, bool exact_nid)
    {
    let mut alloc;
    if (max_addr > memblock.current_limit) {
    max_addr = memblock.current_limit;
    }
    alloc = memblock_alloc_range_nid(size, align, min_addr, max_addr, nid,
    exact_nid);
// retry allocation without lower limit
    if (!alloc && min_addr) {
    alloc = memblock_alloc_range_nid(size, align, 0, max_addr, nid,
    exact_nid);
    }
    if (!alloc) {
    return core::ptr::null_mut();
    }
    return phys_to_virt(alloc);
    }
//
// memblock_alloc_exact_nid_raw - allocate boot memory block on the exact node
// without zeroing memory
// @size: size of memory block to be allocated in bytes
// @align: alignment of the region and block's size
// @min_addr: the lower bound of the memory region from where the allocation
// is preferred (phys address)
// @max_addr: the upper bound of the memory region from where the allocation
// is preferred (phys address), or %MEMBLOCK_ALLOC_ACCESSIBLE to
// allocate only from memory limited by memblock.current_limit value
// @nid: nid of the free area to find, %NUMA_NO_NODE for any node
//
// Public function, provides additional debug information (including caller
// info), if enabled. Does not zero allocated memory.
//
// Return:
// Virtual address of allocated memory block on success, NULL on failure.
//
    void * __init memblock_alloc_exact_nid_raw(
    phys_addr_t size, phys_addr_t align,
    phys_addr_t min_addr, phys_addr_t max_addr,
    int nid)
    {
    memblock_dbg("%s: %llu bytes align=0x%llx nid=%d from=%pa max_addr=%pa %pS\n",
    __func__, (u64)size, (u64)align, nid, &min_addr,
    &max_addr, _RET_IP_);
    return memblock_alloc_internal(size, align, min_addr, max_addr, nid,
    true);
    }
//
// memblock_alloc_try_nid_raw - allocate boot memory block without zeroing
// memory and without panicking
// @size: size of memory block to be allocated in bytes
// @align: alignment of the region and block's size
// @min_addr: the lower bound of the memory region from where the allocation
// is preferred (phys address)
// @max_addr: the upper bound of the memory region from where the allocation
// is preferred (phys address), or %MEMBLOCK_ALLOC_ACCESSIBLE to
// allocate only from memory limited by memblock.current_limit value
// @nid: nid of the free area to find, %NUMA_NO_NODE for any node
//
// Public function, provides additional debug information (including caller
// info), if enabled. Does not zero allocated memory, does not panic if request
// cannot be satisfied.
//
// Return:
// Virtual address of allocated memory block on success, NULL on failure.
//
    void * __init memblock_alloc_try_nid_raw(
    phys_addr_t size, phys_addr_t align,
    phys_addr_t min_addr, phys_addr_t max_addr,
    int nid)
    {
    memblock_dbg("%s: %llu bytes align=0x%llx nid=%d from=%pa max_addr=%pa %pS\n",
    __func__, (u64)size, (u64)align, nid, &min_addr,
    &max_addr, _RET_IP_);
    return memblock_alloc_internal(size, align, min_addr, max_addr, nid,
    false);
    }
//
// memblock_alloc_hugetlb - allocate boot memory for HugeTLB pages
// @size:      size of the memory to be allocated in bytes
// @nid:       nid of the free memory to find, %NUMA_NO_NODE for any node
// @exact_nid: only allocate from the specified nid. If %false, the specified
// nid is tried first, and then all nodes are tried as fallback.
//
// HugeTLB pages are always aligned by their size, so the alignment matches
// @size. Since the memory is for userspace, mirrored memory is not used. The
// memory is not zeroed. Does not panic if request cannot be satisfied.
//
// Return:
// Virtual address of allocated memory block on success, %NULL on failure.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_alloc_hugetlb(size: phys_addr_t, nid: c_int, exact_nid: bool) -> *mut c_void {
pub static mut flags: memblock_flags = 0;
    phys_addr_t addr, start = 0, end = MEMBLOCK_ALLOC_ACCESSIBLE;
    memblock_dbg("%s: %llu bytes, nid=%d, exact_nid=%d %pS\n", __func__,
    (u64)size, nid, exact_nid, _RET_IP_);
// Don't waste mirrored memory on HugeTLB pages.
    flags &= ~MEMBLOCK_MIRROR;
// label;
// HugeTLB pages are always aligned by their size.
    addr = memblock_find_in_range_node(size, size, start, end, nid, flags);
    if (addr) {
// goto;
    }
// Try all nodes if allowed.
    if (numa_valid_node(nid) && !exact_nid) {
    nid = NUMA_NO_NODE;
//
// If a previous candidate overlapped with KHO scratch, it would
// update start or end. Now that the search is opening to all
// nodes, reset them.
//
    start = 0;
    end = MEMBLOCK_ALLOC_ACCESSIBLE;
// goto;
    }
// Found nothing... :-(
    return core::ptr::null_mut();
// label;
//
// HugeTLB pages can be preserved with KHO and no preserved memory can
// be in scratch. So retry if found address overlaps with scratch.
//
// Scratch areas are normally not very large, so this shouldn't take too
// many retries.
//
    if (kho_scratch_overlap(addr, size)) {
    if (memblock_bottom_up()) {
    start = addr + size;
    }
    else {
    end = addr;
    }
// goto;
    }
    if (__memblock_reserve(addr, size, nid, MEMBLOCK_RSRV_KERN | MEMBLOCK_RSRV_HUGETLB)) {
    return core::ptr::null_mut();
    }
    memblock_prep_allocation(addr, size, true);
    return phys_to_virt(addr);
    }
//
// memblock_alloc_try_nid - allocate boot memory block
// @size: size of memory block to be allocated in bytes
// @align: alignment of the region and block's size
// @min_addr: the lower bound of the memory region from where the allocation
// is preferred (phys address)
// @max_addr: the upper bound of the memory region from where the allocation
// is preferred (phys address), or %MEMBLOCK_ALLOC_ACCESSIBLE to
// allocate only from memory limited by memblock.current_limit value
// @nid: nid of the free area to find, %NUMA_NO_NODE for any node
//
// Public function, provides additional debug information (including caller
// info), if enabled. This function zeroes the allocated memory.
//
// Return:
// Virtual address of allocated memory block on success, NULL on failure.
//
    void * __init memblock_alloc_try_nid(
    phys_addr_t size, phys_addr_t align,
    phys_addr_t min_addr, phys_addr_t max_addr,
    int nid)
    {
pub static mut ptr: *mut c_void = core::ptr::null_mut();
    memblock_dbg("%s: %llu bytes align=0x%llx nid=%d from=%pa max_addr=%pa %pS\n",
    __func__, (u64)size, (u64)align, nid, &min_addr,
    &max_addr, _RET_IP_);
    ptr = memblock_alloc_internal(size, align,
    min_addr, max_addr, nid, false);
    if (ptr) {
    memset(ptr, 0, size);
    }
    return ptr;
    }
//
// __memblock_alloc_or_panic - Try to allocate memory and panic on failure
// @size: size of memory block to be allocated in bytes
// @align: alignment of the region and block's size
// @func: caller func name
//
// This function attempts to allocate memory using memblock_alloc,
// and in case of failure, it calls panic with the formatted message.
// This function should not be used directly, please use the macro memblock_alloc_or_panic.
//
    void *__init __memblock_alloc_or_panic(phys_addr_t size, phys_addr_t align,
    const char *func)
    {
    let mut addr = memblock_alloc(size, align);
    if (unlikely(!addr)) {
    panic("%s: Failed to allocate %pap bytes\n", func, &size);
    }
    return addr;
    }
//
// Remaining API functions
//
#[no_mangle]
pub unsafe extern "C" fn memblock_phys_mem_size() -> phys_addr_t __init_memblock {
    return memblock.memory.total_size;
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_reserved_size() -> phys_addr_t __init_memblock {
    return memblock.reserved.total_size;
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_reserved_hugetlb_size(limit: phys_addr_t, nid: c_int) -> phys_addr_t __init_memblock {
pub static mut r: *mut c_void = core::ptr::null_mut();
pub static mut total: phys_addr_t = 0;
    for_each_reserved_mem_region(r) {
pub static mut size: phys_addr_t = 0;
    if (r.base > limit) {
    break;
    }
    if (r.base + r.size > limit) {
    size = limit - r.base;
    }
    if (nid == memblock_get_region_node(r) || !numa_valid_node(nid)) {
    if (r.flags & MEMBLOCK_RSRV_HUGETLB)
    total += size;
    }
    }
    return total;
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_reserved_kern_size(limit: phys_addr_t, nid: c_int) -> phys_addr_t __init_memblock {
pub static mut r: *mut c_void = core::ptr::null_mut();
pub static mut total: phys_addr_t = 0;
    for_each_reserved_mem_region(r) {
pub static mut size: phys_addr_t = 0;
    if (r.base > limit) {
    break;
    }
    if (r.base + r.size > limit) {
    size = limit - r.base;
    }
    if (nid == memblock_get_region_node(r) || !numa_valid_node(nid)) {
    if (r.flags & MEMBLOCK_RSRV_KERN)
    total += size;
    }
    }
    return total;
    }
//
// memblock_estimated_nr_free_pages - return estimated number of free pages
// from memblock point of view
//
// During bootup, subsystems might need a rough estimate of the number of free
// pages in the whole system, before precise numbers are available from the
// buddy. Especially with CONFIG_DEFERRED_STRUCT_PAGE_INIT, the numbers
// obtained from the buddy might be very imprecise during bootup.
//
// Return:
// An estimated number of free pages from memblock point of view.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_estimated_nr_free_pages() -> unsigned long __init {
    return PHYS_PFN(memblock_phys_mem_size() -
    memblock_reserved_kern_size(MEMBLOCK_ALLOC_ANYWHERE, NUMA_NO_NODE));
    }
// lowest address
#[no_mangle]
pub unsafe extern "C" fn memblock_start_of_DRAM() -> phys_addr_t __init_memblock {
    return memblock.memory.regions[0].base;
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_end_of_DRAM() -> phys_addr_t __init_memblock {
pub static mut idx: c_int = 0;
    return (memblock.memory.regions[idx].base + memblock.memory.regions[idx].size);
    }
#[no_mangle]
unsafe extern "C" fn __find_max_addr(limit: phys_addr_t) -> phys_addr_t __init_memblock {
pub static mut max_addr: phys_addr_t = 0;
pub static mut r: *mut c_void = core::ptr::null_mut();
//
// translate the memory @limit size into the max address within one of
// the memory memblock regions, if the @limit exceeds the total size
// of those regions, max_addr will keep original value PHYS_ADDR_MAX
//
    for_each_mem_region(r) {
    if (limit <= r.size) {
    max_addr = r.base + limit;
    break;
    }
    limit -= r.size;
    }
    return max_addr;
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_enforce_memory_limit(limit: phys_addr_t)  {
    let mut max_addr;
    if (!limit) {
    return;
    }
    max_addr = __find_max_addr(limit);
// @limit exceeds the total size of the memory, do nothing
    if (max_addr == PHYS_ADDR_MAX) {
    return;
    }
// truncate both memory and reserved regions
    memblock_remove_range(&memblock.memory, max_addr,
    PHYS_ADDR_MAX);
    memblock_remove_range(&memblock.reserved, max_addr,
    PHYS_ADDR_MAX);
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_cap_memory_range(base: phys_addr_t, size: phys_addr_t)  {
    let mut start_rgn = 0;
    let mut end_rgn = 0;
    let mut i = 0;
    let mut ret = 0;
    if (!size) {
    return;
    }
    if (!memblock_memory.total_size) {
    pr_warn!("%s: No memory registered yet\n", __func__);
    return;
    }
    ret = memblock_isolate_range(&memblock.memory, base, size,
    &start_rgn, &end_rgn);
    if (ret) {
    return;
    }
// remove all the MAP regions
    for (i = memblock.memory.cnt - 1; i >= end_rgn; i--) {
    if (!memblock_is_nomap(&memblock.memory.regions[i]))
    memblock_remove_region(&memblock.memory, i);
    }
    for (i = start_rgn - 1; i >= 0; i--) {
    if (!memblock_is_nomap(&memblock.memory.regions[i]))
    memblock_remove_region(&memblock.memory, i);
    }
// truncate the reserved regions
    memblock_remove_range(&memblock.reserved, 0, base);
    memblock_remove_range(&memblock.reserved,
    base + size, PHYS_ADDR_MAX);
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_mem_limit_remove_map(limit: phys_addr_t)  {
    let mut max_addr;
    if (!limit) {
    return;
    }
    max_addr = __find_max_addr(limit);
// @limit exceeds the total size of the memory, do nothing
    if (max_addr == PHYS_ADDR_MAX) {
    return;
    }
    memblock_cap_memory_range(0, max_addr);
    }
#[no_mangle]
unsafe extern "C" fn memblock_search(type: *mut memblock_type, addr: phys_addr_t) -> c_int_memblock {
pub static mut left: c_uint = 0;
    do {
pub static mut mid: c_uint = 0;
    if (addr < type.regions[mid].base) {
    right = mid;
    }
    else if (addr >= (type.regions[mid].base +
    type.regions[mid].size)) {
    left = mid + 1;
    }
    else {
    return mid;
    }
    } while (left < right);
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_is_reserved(addr: phys_addr_t) -> bool __init_memblock {
    return memblock_search(&memblock.reserved, addr) != -1;
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_is_memory(addr: phys_addr_t) -> bool __init_memblock {
    return memblock_search(&memblock.memory, addr) != -1;
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_is_map_memory(addr: phys_addr_t) -> bool __init_memblock {
pub static mut i: c_int = 0;
    if (i == -1) {
    return false;
    }
    return !memblock_is_nomap(&memblock.memory.regions[i]);
    }
    int __init_memblock memblock_search_pfn_nid(unsigned long pfn,
    unsigned long *start_pfn, unsigned long *end_pfn)
    {
    let mut type = &memblock.memory;
pub static mut mid: c_int = 0;
    if (mid == -1) {
    return NUMA_NO_NODE;
    }
// start_pfn = PFN_DOWN(type->regions[mid].base);
// end_pfn = PFN_DOWN(type->regions[mid].base + type->regions[mid].size);
    return memblock_get_region_node(&type.regions[mid]);
    }
//
// memblock_is_region_memory - check if a region is a subset of memory
// @base: base of region to check
// @size: size of region to check
//
// Check if the region [@base, @base + @size) is a subset of a memory block.
//
// Return:
// 0 if false, non-zero if true
//
#[no_mangle]
pub unsafe extern "C" fn memblock_is_region_memory(base: phys_addr_t, size: phys_addr_t) -> bool __init_memblock {
pub static mut idx: c_int = 0;
pub static mut end: phys_addr_t = 0;
    if (idx == -1) {
    return false;
    }
    return (memblock.memory.regions[idx].base +
    memblock.memory.regions[idx].size) >= end;
    }
//
// memblock_is_region_reserved - check if a region intersects reserved memory
// @base: base of region to check
// @size: size of region to check
//
// Check if the region [@base, @base + @size) intersects a reserved
// memory block.
//
// Return:
// True if they intersect, false if not.
//
#[no_mangle]
pub unsafe extern "C" fn memblock_is_region_reserved(base: phys_addr_t, size: phys_addr_t) -> bool __init_memblock {
    return memblock_overlaps_region(&memblock.reserved, base, size);
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_trim_memory(align: phys_addr_t) _memblock {
    phys_addr_t start, end, orig_start, orig_end;
pub static mut r: *mut c_void = core::ptr::null_mut();
    for_each_mem_region(r) {
    orig_start = r.base;
    orig_end = r.base + r.size;
    start = round_up(orig_start, align);
    end = round_down(orig_end, align);
    if (start == orig_start && end == orig_end) {
    continue;
    }
    if (start < end) {
    r.base = start;
    r.size = end - start;
    } else {
    memblock_remove_region(&memblock.memory,
    r - memblock.memory.regions);
    r -= 1;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_set_current_limit(limit: phys_addr_t) _memblock {
    memblock.current_limit = limit;
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_get_current_limit() -> phys_addr_t __init_memblock {
    return memblock.current_limit;
    }
#[no_mangle]
unsafe extern "C" fn memblock_dump(type: *mut memblock_type) _memblock {
    phys_addr_t base, end, size;
    enum memblock_flags flags;
    let mut idx = 0;
pub static mut rgn: *mut c_void = core::ptr::null_mut();
    pr_info!(" %s.cnt  = 0x%lx\n", type.name, type.cnt);
    for_each_memblock_type(idx, type, rgn) {
    char nid_buf[32] = "";
    base = rgn.base;
    size = rgn.size;
    end = base + size - 1;
    flags = rgn.flags;

    if (numa_valid_node(memblock_get_region_node(rgn))) {
    snprintf(nid_buf, sizeof!(nid_buf), " on node %d",
    memblock_get_region_node(rgn));
    }

    pr_info!(" %s[%#x]\t[%pa-%pa], %pa bytes%s flags: %#x\n",
    type.name, idx, &base, &end, &size, nid_buf, flags);
    }
    }
#[no_mangle]
unsafe extern "C" fn __memblock_dump_all() _memblock {
    pr_info!("MEMBLOCK configuration:\n");
    pr_info!(" memory size = %pa reserved size = %pa\n",
    &memblock.memory.total_size,
    &memblock.reserved.total_size);
    memblock_dump(&memblock.memory);
    memblock_dump(&memblock.reserved);

    memblock_dump(&physmem);

    }
#[no_mangle]
pub unsafe extern "C" fn memblock_dump_all() _memblock {
    if (memblock_debug) {
    __memblock_dump_all();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_allow_resize()  {
    memblock_can_resize = 1;
    }
#[no_mangle]
unsafe extern "C" fn early_memblock(p: *mut c_char) -> c_int {
    if (p && strstr(p, "debug")) {
    memblock_debug = 1;
    }
    return 0;
    }
    early_param!("memblock", early_memblock);
#[no_mangle]
unsafe extern "C" fn free_memmap(start_pfn: c_ulong, end_pfn: c_ulong)  {
    let mut start_pg = core::ptr::null_mut();
    let mut end_pg = core::ptr::null_mut();
    phys_addr_t pg, pgend;
//
// Convert start_pfn/end_pfn to a struct page pointer.
//
    start_pg = pfn_to_page(start_pfn - 1) + 1;
    end_pg = pfn_to_page(end_pfn - 1) + 1;
//
// Convert to physical addresses, and round start upwards and end
// downwards.
//
    pg = PAGE_ALIGN(__pa(start_pg));
    pgend = PAGE_ALIGN_DOWN(__pa(end_pg));
//
// If there are free pages between these, free the section of the
// memmap array.
//
    if (pg < pgend) {
    memblock_phys_free(pg, pgend - pg);
    }
    }
//
// The mem_map array can get very big.  Free the unused area of the memory map.
//
#[no_mangle]
unsafe extern "C" fn free_unused_memmap()  {
    unsigned long start, end, prev_end = 0;
    let mut i = 0;
    if (!IS_ENABLED!(CONFIG_HAVE_ARCH_PFN_VALID) ||
    IS_ENABLED!(CONFIG_SPARSEMEM_VMEMMAP)) {
    return;
    }
//
// This relies on each bank being in address order.
// The banks are sorted previously in bootmem_init().
//
    for_each_mem_pfn_range(i, MAX_NUMNODES, &start, &end, core::ptr::null_mut()) {

//
// Take care not to free memmap entries that don't exist
// due to SPARSEMEM sections which aren't present.
//
    start = min(start, ALIGN(prev_end, PAGES_PER_SECTION));

//
// Align down here since many operations in VM subsystem
// presume that there are no holes in the memory map inside
// a pageblock
//
    start = pageblock_start_pfn(start);
//
// If we had a previous bank, and there is a space
// between the current bank and the previous, free it.
//
    if (prev_end && prev_end < start) {
    free_memmap(prev_end, start);
    }
//
// Align up here since many operations in VM subsystem
// presume that there are no holes in the memory map inside
// a pageblock
//
    prev_end = pageblock_align(end);
    }

    if (!IS_ALIGNED(prev_end, PAGES_PER_SECTION)) {
    free_memmap(prev_end, ALIGN(prev_end, PAGES_PER_SECTION));
    }

    }
#[no_mangle]
unsafe extern "C" fn __free_pages_memory(start: c_ulong, end: c_ulong)  {
    let mut order = 0;
    while (start < end) {
//
// Free the pages in the largest chunks alignment allows.
//
// __ffs() behaviour is undefined for 0. start == 0 is
// MAX_PAGE_ORDER-aligned, set order to MAX_PAGE_ORDER for
// the case.
//
    if (start) {
    order = min_t(int, MAX_PAGE_ORDER, __ffs(start));
    }
    else {
    order = MAX_PAGE_ORDER;
    }
    while (start + (1UL << order) > end) {
    order -= 1;
    }
    memblock_free_pages(start, order);
    start += (1UL << order);
    }
    }
    static unsigned long __init __free_memory_core(phys_addr_t start,
    phys_addr_t end)
    {
pub static mut start_pfn: c_ulong = 0;
pub static mut end_pfn: c_ulong = 0;
    if (!IS_ENABLED!(CONFIG_HIGHMEM) && end_pfn > max_low_pfn) {
    end_pfn = max_low_pfn;
    }
    if (start_pfn >= end_pfn) {
    return 0;
    }
    __free_pages_memory(start_pfn, end_pfn);
    return end_pfn - start_pfn;
    }
//
// Initialised pages do not have PageReserved set. This function is called
// for each reserved range and marks the pages PageReserved.
// When deferred initialization of struct pages is enabled it also ensures
// that struct pages are properly initialised.
//
    static void __init memmap_init_reserved_range(phys_addr_t start,
    phys_addr_t end, int nid)
    {
    let mut pfn = 0;
    for_each_valid_pfn(pfn, PFN_DOWN(start), PFN_UP(end)) {
    let mut page = pfn_to_page(pfn);
    init_deferred_page(pfn, nid);
//
// no need for atomic set_bit because the struct
// page is not visible yet so nobody should
// access it yet.
//
    __SetPageReserved(page);
    }
    }
#[no_mangle]
unsafe extern "C" fn memmap_init_reserved_pages()  {
pub static mut region: *mut c_void = core::ptr::null_mut();
    phys_addr_t start, end;
    let mut nid = 0;
    let mut max_reserved = 0;
//
// set nid on all reserved pages and also treat struct
// pages for the NOMAP regions as PageReserved
//
// label;
    max_reserved = memblock.reserved.max;
    for_each_mem_region(region) {
    nid = memblock_get_region_node(region);
    start = region.base;
    end = start + region.size;
    if (memblock_is_nomap(region)) {
    memmap_init_reserved_range(start, end, nid);
    }
    memblock_set_node(start, region.size, &memblock.reserved, nid);
    }
//
// 'max' is changed means memblock.reserved has been doubled its
// array, which may result a new reserved region before current
// 'start'. Now we should repeat the procedure to set its node id.
//
    if (max_reserved != memblock.reserved.max) {
// goto;
    }
//
// initialize struct pages for reserved regions that don't have
// the MEMBLOCK_RSRV_NOINIT flag set
//
    for_each_reserved_mem_region(region) {
    if (!memblock_is_reserved_noinit(region)) {
    nid = memblock_get_region_node(region);
    start = region.base;
    end = start + region.size;
    if (!numa_valid_node(nid)) {
    nid = early_pfn_to_nid(PFN_DOWN(start));
    }
    memmap_init_reserved_range(start, end, nid);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn free_low_memory_core_early() -> unsigned long __init {
pub static mut count: c_ulong = 0;
    phys_addr_t start, end;
    let mut i = 0;
    memblock_clear_hotplug(0, -1);
    memmap_init_reserved_pages();
//
// We need to use NUMA_NO_NODE instead of NODE_DATA(0)->node_id
// because in some case like Node0 doesn't have RAM installed
// low ram will be on Node1
//
    for_each_free_mem_range(i, NUMA_NO_NODE, MEMBLOCK_NONE, &start, &end,
    core::ptr::null_mut()) {
    count += __free_memory_core(start, end);
    }
    return count;
    }
    static int reset_managed_pages_done __initdata;
#[no_mangle]
unsafe extern "C" fn reset_node_managed_pages(pgdat: *mut pg_data_t)  {
pub static mut z: *mut c_void = core::ptr::null_mut();
    for (z = pgdat.node_zones; z < pgdat.node_zones + MAX_NR_ZONES; z++) {
    atomic_long_set(&z.managed_pages, 0);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn reset_all_zones_managed_pages()  {
pub static mut pgdat: *mut c_void = core::ptr::null_mut();
    if (reset_managed_pages_done) {
    return;
    }
    for_each_online_pgdat(pgdat) {
    reset_node_managed_pages(pgdat);
    }
    reset_managed_pages_done = 1;
    }
//
// memblock_free_all - release free pages to the buddy allocator
//
#[no_mangle]
pub unsafe extern "C" fn memblock_free_all()  {
    let mut pages = 0;
    free_unused_memmap();
    reset_all_zones_managed_pages();
    memblock_clear_kho_scratch_only();
    pages = free_low_memory_core_early();
    totalram_pages_add(pages);
    }
// Keep a table to reserve named memory
pub const RESERVE_MEM_MAX_ENTRIES: c_int = 8;
pub const RESERVE_MEM_NAME_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reserve_mem_table {
    pub name: [c_char; RESERVE_MEM_NAME_SIZE],
    pub start: phys_addr_t,
    pub size: phys_addr_t,
}

    static struct reserve_mem_table reserved_mem_table[RESERVE_MEM_MAX_ENTRIES];
    static int reserved_mem_count;
pub static mut reserve_mem_lock: usize = 0;
// Add wildcard region with a lookup name
    static void __init reserved_mem_add(phys_addr_t start, phys_addr_t size,
    const char *name)
    {
pub static mut map: *mut c_void = core::ptr::null_mut();
    map = &reserved_mem_table[reserved_mem_count++];
    map.start = start;
    map.size = size;
    strscpy(map.name, name);
    }
#[no_mangle]
pub unsafe extern "C" fn reserve_mem_find_by_name_nolock(name: *mut c_char) -> *mut c_void {
pub static mut map: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < reserved_mem_count) {
    map = &reserved_mem_table[i];
    if (!map.size) {
    continue;
    }
    if (strcmp(name, map.name) == 0) {
    return map;
    }
    }
    return core::ptr::null_mut();
    }
//
// reserve_mem_find_by_name - Find reserved memory region with a given name
// @name: The name that is attached to a reserved memory region
// @start: If found, holds the start address
// @size: If found, holds the size of the address.
//
// @start and @size are only updated if @name is found.
//
// Returns: 1 if found or 0 if not found.
//
#[no_mangle]
pub unsafe extern "C" fn reserve_mem_find_by_name(name: *const c_char, start: *mut phys_addr_t, size: *mut phys_addr_t) -> c_int {
pub static mut map: *mut c_void = core::ptr::null_mut();
    guard(mutex)(&reserve_mem_lock);
    map = reserve_mem_find_by_name_nolock(name);
    if (!map) {
    return 0;
    }
// start = map->start;
// size = map->size;
    return 1;
    }
    EXPORT_SYMBOL_GPL(reserve_mem_find_by_name);
//
// reserve_mem_release_by_name - Release reserved memory region with a given name
// @name: The name that is attached to a reserved memory region
//
// Forcibly release the pages in the reserved memory region so that those memory
// can be used as free memory. After released the reserved region size becomes 0.
//
// Returns: 1 if released or 0 if not found.
//
#[no_mangle]
pub unsafe extern "C" fn reserve_mem_release_by_name(name: *const c_char) -> c_int {
    char buf[RESERVE_MEM_NAME_SIZE + 12];
pub static mut map: *mut c_void = core::ptr::null_mut();
    let mut start = core::ptr::null_mut();
    let mut end = core::ptr::null_mut();
    guard(mutex)(&reserve_mem_lock);
    map = reserve_mem_find_by_name_nolock(name);
    if (!map) {
    return 0;
    }
    start = phys_to_virt(map.start);
    end = start + map.size;
    snprintf(buf, sizeof!(buf), "reserve_mem:%s", name);
    free_reserved_area(start, end, 0, buf);
    map.size = 0;
    return 1;
    }

#[no_mangle]
pub unsafe extern "C" fn memblock_set_kho_scratch_only() -> __init void {
    kho_scratch_only = true;
    }
#[no_mangle]
pub unsafe extern "C" fn memblock_clear_kho_scratch_only() -> __init void {
    kho_scratch_only = false;
    }

#[no_mangle]
unsafe extern "C" fn reserved_mem_preserve() -> c_int {
pub static mut nr_preserved: c_uint = 0;
    let mut err = 0;
    while (i < reserved_mem_count) {
    let mut map = &reserved_mem_table[i];
    let mut page = phys_to_page(map.start);
pub static mut nr_pages: c_uint = 0;
    err = kho_preserve_pages(page, nr_pages);
    if (err) {
// goto;
    }
    }
    return 0;
// label;
    while (i < nr_preserved) {
    let mut map = &reserved_mem_table[i];
    let mut page = phys_to_page(map.start);
pub static mut nr_pages: c_uint = 0;
    kho_unpreserve_pages(page, nr_pages);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn prepare_kho_fdt() -> c_int {
pub static mut fdt_page: *mut c_void = core::ptr::null_mut();
pub static mut fdt: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    fdt_page = alloc_page(GFP_KERNEL);
    if (!fdt_page) {
    err = -ENOMEM;
// goto;
    }
    fdt = page_to_virt(fdt_page);
    err = kho_preserve_pages(fdt_page, 1);
    if (err) {
// goto;
    }
    err |= fdt_create(fdt, PAGE_SIZE);
    err |= fdt_finish_reservemap(fdt);
    err |= fdt_begin_node(fdt, "");
    err |= fdt_property_string(fdt, "compatible", MEMBLOCK_KHO_NODE_COMPATIBLE);
    while (!err && i < reserved_mem_count) {
    let mut map = &reserved_mem_table[i];
    err |= fdt_begin_node(fdt, map.name);
    err |= fdt_property_string(fdt, "compatible", RESERVE_MEM_KHO_NODE_COMPATIBLE);
    err |= fdt_property(fdt, "start", &map.start, sizeof!(map.start));
    err |= fdt_property(fdt, "size", &map.size, sizeof!(map.size));
    err |= fdt_end_node(fdt);
    }
    err |= fdt_end_node(fdt);
    err |= fdt_finish(fdt);
    if (err) {
// goto;
    }
    err = kho_add_subtree(MEMBLOCK_KHO_FDT, fdt, fdt_totalsize(fdt));
    if (err) {
// goto;
    }
    err = reserved_mem_preserve();
    if (err) {
// goto;
    }
    return 0;
// label;
    kho_remove_subtree(fdt);
// label;
    kho_unpreserve_pages(fdt_page, 1);
// label;
    put_page(fdt_page);
// label;
    pr_err!("failed to prepare memblock FDT for KHO: %d\n", err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn reserve_mem_init() -> c_int {
    let mut err = 0;
    if (!kho_is_enabled() || !reserved_mem_count) {
    return 0;
    }
    err = prepare_kho_fdt();
    if (err) {
    return err;
    }
    return err;
    }
    late_initcall!(reserve_mem_init);
#[no_mangle]
unsafe extern "C" fn reserve_mem_kho_retrieve_fdt() -> *mut c_void {
    let mut fdt_phys;
pub static mut fdt: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (fdt) {
    return fdt;
    }
    err = kho_retrieve_subtree(MEMBLOCK_KHO_FDT, &fdt_phys, core::ptr::null_mut());
    if (err) {
    if (err != -ENOENT) {
    pr_warn!("failed to retrieve FDT '%s' from KHO: %d\n",
    MEMBLOCK_KHO_FDT, err);
    }
    return core::ptr::null_mut();
    }
    fdt = phys_to_virt(fdt_phys);
    err = fdt_node_check_compatible(fdt, 0, MEMBLOCK_KHO_NODE_COMPATIBLE);
    if (err) {
    pr_warn!("FDT '%s' is incompatible with '%s': %d\n",
    MEMBLOCK_KHO_FDT, MEMBLOCK_KHO_NODE_COMPATIBLE, err);
    fdt = core::ptr::null_mut();
    }
    return fdt;
    }
    static bool __init reserve_mem_kho_revive(const char *name, phys_addr_t size,
    phys_addr_t align)
    {
    let mut err = 0;
    let mut len_start = 0;
    let mut len_size = 0;
    let mut offset = 0;
    let mut p_start = core::ptr::null_mut();
    let mut p_size = core::ptr::null_mut();
pub static mut fdt: *mut c_void = core::ptr::null_mut();
    fdt = reserve_mem_kho_retrieve_fdt();
    if (!fdt) {
    return false;
    }
    offset = fdt_subnode_offset(fdt, 0, name);
    if (offset < 0) {
    pr_warn!("FDT '%s' has no child '%s': %d\n",
    MEMBLOCK_KHO_FDT, name, offset);
    return false;
    }
    err = fdt_node_check_compatible(fdt, offset, RESERVE_MEM_KHO_NODE_COMPATIBLE);
    if (err) {
    pr_warn!("Node '%s' is incompatible with '%s': %d\n",
    name, RESERVE_MEM_KHO_NODE_COMPATIBLE, err);
    return false;
    }
    p_start = fdt_getprop(fdt, offset, "start", &len_start);
    p_size = fdt_getprop(fdt, offset, "size", &len_size);
    if (!p_start || len_start != sizeof!(*p_start) || !p_size ||
    len_size != sizeof!(*p_size)) {
    return false;
    }
    if (*p_start & (align - 1)) {
    pr_warn!("KHO reserve-mem '%s' has wrong alignment (0x%lx, 0x%lx)\n",
    name, (long)align, (long)*p_start);
    return false;
    }
    if (*p_size != size) {
    pr_warn!("KHO reserve-mem '%s' has wrong size (0x%lx != 0x%lx)\n",
    name, (long)*p_size, (long)size);
    return false;
    }
    reserved_mem_add(*p_start, size, name);
    pr_info!("Revived memory reservation '%s' from KHO\n", name);
    return true;
    }

    static bool __init reserve_mem_kho_revive(const char *name, phys_addr_t size,
    phys_addr_t align)
    {
    return false;
    }

//
// Parse reserve_mem=nn:align:name
//
#[no_mangle]
unsafe extern "C" fn reserve_mem(p: *mut c_char) -> c_int {
    phys_addr_t start, size, align, tmp;
pub static mut name: *mut c_void = core::ptr::null_mut();
pub static mut oldp: *mut c_void = core::ptr::null_mut();
    let mut len = 0;
    if (!p) {
// goto;
    }
// Check if there's room for more reserved memory
    if (reserved_mem_count >= RESERVE_MEM_MAX_ENTRIES) {
    pr_err!("reserve_mem: no more room for reserved memory\n");
    return -EBUSY;
    }
    oldp = p;
    size = memparse(p, &p);
    if (!size || p == oldp) {
// goto;
    }
    if (*p != ':') {
// goto;
    }
    align = memparse(p+1, &p);
    if (*p != ':') {
// goto;
    }
//
// memblock_phys_alloc() doesn't like a zero size align,
// but it is OK for this command to have it.
//
    if (align < SMP_CACHE_BYTES) {
    align = SMP_CACHE_BYTES;
    }
    name = p + 1;
    len = strlen(name);
// name needs to have length but not too big
    if (!len || len >= RESERVE_MEM_NAME_SIZE) {
// goto;
    }
// Make sure that name has text
    while (*p) {
    if (!isspace(*p)) {
    break;
    }
    }
    if (!*p) {
// goto;
    }
// Make sure the name is not already used
    if (reserve_mem_find_by_name(name, &start, &tmp)) {
    pr_err!("reserve_mem: name \"%s\" was already used\n", name);
    return -EBUSY;
    }
// Pick previous allocations up from KHO if available
    if (reserve_mem_kho_revive(name, size, align)) {
    return 1;
    }
// TODO: Allocation must be outside of scratch region
    start = memblock_phys_alloc(size, align);
    if (!start) {
    pr_err!("reserve_mem: memblock allocation failed\n");
    return -ENOMEM;
    }
    reserved_mem_add(start, size, name);
    return 1;
// label;
    pr_err!("reserve_mem: empty or malformed parameter\n");
    return -EINVAL;
    }
    __setup!("reserve_mem=", reserve_mem);

    static const char * const flagname[] = {
    [ilog2(MEMBLOCK_HOTPLUG)] = "HOTPLUG",
    [ilog2(MEMBLOCK_MIRROR)] = "MIRROR",
    [ilog2(MEMBLOCK_NOMAP)] = "NOMAP",
    [ilog2(MEMBLOCK_DRIVER_MANAGED)] = "DRV_MNG",
    [ilog2(MEMBLOCK_RSRV_NOINIT)] = "RSV_NIT",
    [ilog2(MEMBLOCK_RSRV_KERN)] = "RSV_KERN",
    [ilog2(MEMBLOCK_KHO_SCRATCH)] = "KHO_SCRATCH",
    };
#[no_mangle]
unsafe extern "C" fn memblock_debug_show(m: *mut seq_file, private: *mut c_void) -> c_int {
    let mut type = m.private;
pub static mut reg: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut j = 0;
    let mut nid = 0;
pub static mut count: c_uint = 0;
    let mut end;
    while (i < type.cnt) {
    reg = &type.regions[i];
    end = reg.base + reg.size - 1;
    nid = memblock_get_region_node(reg);
    seq_printf(m, "%4d: ", i);
    seq_printf(m, "%pa..%pa ", &reg.base, &end);
    if (numa_valid_node(nid)) {
    seq_printf(m, "%4d ", nid);
    }
    else {
    seq_printf(m, "%4c ", 'x');
    }
    if (reg.flags) {
    while (j < count) {
    if (reg.flags & (1U << j)) {
    seq_printf(m, "%s\n", flagname[j]);
    break;
    }
    }
    if (j == count) {
    seq_printf(m, "%s\n", "UNKNOWN");
    }
    } else {
    seq_printf(m, "%s\n", "NONE");
    }
    }
    return 0;
    }
pub static mut memblock_debug: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn memblock_debugfs_expose_arrays(root: *mut dentry) {
    debugfs_create_file("memory", 0444, root,
    &memblock.memory, &memblock_debug_fops);
    debugfs_create_file("reserved", 0444, root,
    &memblock.reserved, &memblock_debug_fops);

    debugfs_create_file("physmem", 0444, root, &physmem,
    &memblock_debug_fops);

    }

#[no_mangle]
#[no_mangle]
// duplicate fn: memblock_debugfs_expose_arrays
pub unsafe extern "C" fn memblock_debugfs_expose_arrays_dup(root: *mut dentry) { }

#[no_mangle]
unsafe extern "C" fn memblock_reserve_mem_show(m: *mut seq_file, private: *mut c_void) -> c_int {
pub static mut map: *mut c_void = core::ptr::null_mut();
    char txtsz[16];
    guard(mutex)(&reserve_mem_lock);
    while (i < reserved_mem_count) {
    map = &reserved_mem_table[i];
    if (!map.size) {
    continue;
    }
    memset(txtsz, 0, sizeof!(txtsz));
    string_get_size(map.size, 1, STRING_UNITS_2, txtsz, sizeof!(txtsz));
    seq_printf(m, "%s\t\t(%s)\n", map.name, txtsz);
    }
    return 0;
    }
pub static mut memblock_reserve_mem: usize = 0;
#[no_mangle]
unsafe extern "C" fn memblock_init_debugfs() -> c_int {
pub static mut root: *mut c_void = core::ptr::null_mut();
    if (!IS_ENABLED!(CONFIG_ARCH_KEEP_MEMBLOCK) && !reserved_mem_count) {
    return 0;
    }
    root = debugfs_create_dir("memblock", core::ptr::null_mut());
    if (reserved_mem_count) {
    debugfs_create_file("reserve_mem_param", 0444, root, core::ptr::null_mut(),
    &memblock_reserve_mem_fops);
    }
    memblock_debugfs_expose_arrays(root);
    return 0;
    }
    __initcall!(memblock_init_debugfs);
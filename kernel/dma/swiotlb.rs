//! Automatically rewritten from C to Rust
//! Source: kernel/dma/swiotlb.c
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
// Dynamic DMA mapping support.
//
// This implementation is a fallback for platforms that do not support
// I/O TLBs (aka DMA address translation hardware).
// Copyright (C) 2000 Asit Mallick <Asit.K.Mallick@intel.com>
// Copyright (C) 2000 Goutham Rao <goutham.rao@intel.com>
// Copyright (C) 2000, 2003 Hewlett-Packard Co
// David Mosberger-Tang <davidm@hpl.hp.com>
//
// 03/05/07 davidm	Switch from PCI-DMA to generic device DMA API.
// 00/12/13 davidm	Rename to swiotlb.c and add mark_clean() to avoid
// unnecessary i-cache flushing.
// 04/07/.. ak		Better overflow handling. Assorted fixes.
// 05/09/10 linville	Add support for syncing ranges, support syncing for
// DMA_BIDIRECTIONAL mappings, miscellaneous cleanup.
// 08/12/11 beckyb	Add highmem support
//

// Macro flag: #define CREATE_TRACE_POINTS

//
// Minimum IO TLB size to bother booting with.  Systems with mainly
// 64bit capable cards will only lightly use the swiotlb.  If we can't
// allocate a contiguous 1MB, we're probably in trouble anyway.
//

//
// struct io_tlb_slot - IO TLB slot descriptor
// @orig_addr:	The original address corresponding to a mapped entry.
// @alloc_size:	Size of the allocated buffer.
// @list:	The free list describing the number of free entries available
// from each index.
// @pad_slots:	Number of preceding padding slots. Valid only in the first
// allocated non-padding slot.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_tlb_slot {
    pub orig_addr: phys_addr_t,
    pub alloc_size: usize,
    pub list: c_ushort,
    pub pad_slots: c_ushort,
}

    static bool swiotlb_force_bounce;
    static bool swiotlb_force_disable;

// forward_decl: swiotlb_dyn_alloc;
pub static mut io_tlb_mem: usize = 0;

pub static mut io_tlb_default_mem: usize = 0;

pub static mut default_nslabs: unsigned long = 0;
    static unsigned long default_nareas;
//
// struct io_tlb_area - IO TLB memory area descriptor
//
// This is a single area with a single lock.
//
// @used:	The number of used IO TLB block.
// @index:	The slot index to start searching in this area for next round.
// @lock:	The lock to protect the above data structures in the map and
// unmap calls.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_tlb_area {
    pub used: c_ulong,
    pub index: c_uint,
    pub lock: spinlock_t,
}

//
// Round up number of slabs to the next power of 2. The last area is going
// be smaller than the rest if default_nslabs is not power of two.
// The number of slot in an area should be a multiple of IO_TLB_SEGSIZE,
// otherwise a segment may span two or more areas. It conflicts with free
// contiguous slots tracking: free slots are treated contiguous no matter
// whether they cross an area boundary.
//
// Return true if default_nslabs is rounded up.
//
#[no_mangle]
unsafe extern "C" fn round_up_default_nslabs() -> bool {
    if (!default_nareas) {
    return false;
    }
    if (default_nslabs < IO_TLB_SEGSIZE * default_nareas) {
    default_nslabs = IO_TLB_SEGSIZE * default_nareas;
    }

    else if (is_power_of_2(default_nslabs)) {
    return false;
    }
    default_nslabs = roundup_pow_of_two(default_nslabs);
    return true;
    }
//
// swiotlb_adjust_nareas() - adjust the number of areas and slots
// @nareas:	Desired number of areas. Zero is treated as 1.
//
// Adjust the default number of areas in a memory pool.
// The default size of the memory pool may also change to meet minimum area
// size requirements.
//
#[no_mangle]
unsafe extern "C" fn swiotlb_adjust_nareas(nareas: c_uint) {
    if (!nareas) {
    nareas = 1;
    }

    else if (!is_power_of_2(nareas)) {
    nareas = roundup_pow_of_two(nareas);
    }
    default_nareas = nareas;
    pr_info!("area num %d.\n", nareas);
    if (round_up_default_nslabs()) {
    pr_info!("SWIOTLB bounce buffer size roundup to %luMB",
    (default_nslabs << IO_TLB_SHIFT) >> 20);
    }
    }
//
// limit_nareas() - get the maximum number of areas for a given memory pool size
// @nareas:	Desired number of areas.
// @nslots:	Total number of slots in the memory pool.
//
// Limit the number of areas to the maximum possible number of areas in
// a memory pool of the given size.
//
// Return: Maximum possible number of areas.
//
#[no_mangle]
unsafe extern "C" fn limit_nareas(nareas: c_uint, nslots: c_ulong) -> c_uint {
    if (nslots < nareas * IO_TLB_SEGSIZE) {
    return nslots / IO_TLB_SEGSIZE;
    }
    return nareas;
    }

//
// Track the total used slots with a global atomic value in order to have
// correct information to determine the high water mark.
//
#[no_mangle]
pub unsafe extern "C" fn inc_used_and_hiwater_real(mem: *mut io_tlb_mem, nslots: c_uint) {
    unsigned long old_hiwater, new_used;
    new_used = atomic_long_add_return(nslots, &mem.total_used);
    old_hiwater = atomic_long_read(&mem.used_hiwater);
    do {
    if (new_used <= old_hiwater) {
    break;
    }
    } while (!atomic_long_try_cmpxchg(&mem.used_hiwater,
    &old_hiwater, new_used));
    }
#[no_mangle]
unsafe extern "C" fn dec_used_real(mem: *mut io_tlb_mem, nslots: c_uint) {
    atomic_long_sub(nslots, &mem.total_used);
    }
#[no_mangle]
pub unsafe extern "C" fn inc_used_and_hiwater_nop(mem: *mut io_tlb_mem, nslots: c_uint) {
    }
#[no_mangle]
unsafe extern "C" fn dec_used_nop(mem: *mut io_tlb_mem, nslots: c_uint) {
    }
pub static mut swiotlb_inc_used: usize = 0;
pub static mut swiotlb_dec_used: usize = 0;
    static __always_inline void inc_used_and_hiwater(io_tlb_mem *mem,
    unsigned int nslots)
    {
    static_call(swiotlb_inc_used)(mem, nslots);
    }
    static __always_inline void dec_used(io_tlb_mem *mem,
    unsigned int nslots)
    {
    static_call(swiotlb_dec_used)(mem, nslots);
    }
    static bool track_hiwater_enabled ;

    static __always_inline void inc_used_and_hiwater(io_tlb_mem *mem,
    unsigned int nslots)
    {
    }
    static __always_inline void dec_used(io_tlb_mem *mem,
    unsigned int nslots)
    {
    }

//
// The tracking of used slots high watermark can be enabled
// by appending "track_hiwater" to the swiotlb= boot parameter.
// When disabled the tracking functions are no-ops with near-zero
// overhead via static_call.
//
    static int __init
    setup_io_tlb_npages(char *str)
    {
    if (isdigit(*str)) {
// avoid tail segment of size < IO_TLB_SEGSIZE
    default_nslabs =
    ALIGN(simple_strtoul(str, &str, 0), IO_TLB_SEGSIZE);
    }
    if (*str == ',') {
    str += 1;
    }
    if (isdigit(*str)) {
    swiotlb_adjust_nareas(simple_strtoul(str, &str, 0));
    }
    if (*str == ',') {
    str += 1;
    }
    if (!strncmp(str, "force", 5)) {
    swiotlb_force_bounce = true;
    str += 5;
    } else if (!strncmp(str, "noforce", 7)) {
    swiotlb_force_disable = true;
    str += 7;
    }

    if (*str == ',') {
    str += 1;
    }
    if (!strncmp(str, "track_hiwater", 13)) {
    track_hiwater_enabled = true;
    static_call_update(swiotlb_inc_used,
    inc_used_and_hiwater_real);
    static_call_update(swiotlb_dec_used, dec_used_real);
    }

    return 0;
    }
    early_param!("swiotlb", setup_io_tlb_npages);
#[no_mangle]
pub unsafe extern "C" fn swiotlb_size_or_default() -> c_ulong {
    return default_nslabs << IO_TLB_SHIFT;
    }
#[no_mangle]
pub unsafe extern "C" fn swiotlb_adjust_size(size: c_ulong)  {
//
// If swiotlb parameter has not been specified, give a chance to
// architectures such as those supporting memory encryption to
// adjust/expand SWIOTLB size for their use.
//
    if (default_nslabs != IO_TLB_DEFAULT_SIZE >> IO_TLB_SHIFT) {
    return;
    }
    size = ALIGN(size, IO_TLB_SIZE);
    default_nslabs = ALIGN(size >> IO_TLB_SHIFT, IO_TLB_SEGSIZE);
    if (round_up_default_nslabs()) {
    size = default_nslabs << IO_TLB_SHIFT;
    }
    pr_info!("SWIOTLB bounce buffer size adjusted to %luMB", size >> 20);
    }
#[no_mangle]
pub unsafe extern "C" fn swiotlb_print_info() {
    let mut mem = &io_tlb_default_mem.defpool;
    if (!mem.nslabs) {
    pr_warn!("No low mem\n");
    return;
    }
    pr_info!("mapped [mem %pa-%pa] (%luMB)\n", &mem.start, &mem.end,
    (mem.nslabs << IO_TLB_SHIFT) >> 20);
    }
#[no_mangle]
pub unsafe extern "C" fn io_tlb_offset(val: c_ulong) -> c_ulong {
    return val & (IO_TLB_SEGSIZE - 1);
    }
#[no_mangle]
pub unsafe extern "C" fn nr_slots(val: u64) -> c_ulong {
    return DIV_ROUND_UP(val, IO_TLB_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn swiotlb_mark_pool_used(pool: *mut io_tlb_pool) {
    let mut i = 0;
    while (i < pool.nareas) {
    pool.areas[i].index = 0;
    pool.areas[i].used = pool.area_nslabs;
    }
    while (i < pool.nslabs) {
    pool.slots[i].list = 0;
    pool.slots[i].orig_addr = INVALID_PHYS_ADDR;
    pool.slots[i].alloc_size = 0;
    pool.slots[i].pad_slots = 0;
    }
    }
//
// Early SWIOTLB allocation may be too early to allow an architecture to
// perform the desired operations.  This function allows the architecture to
// call SWIOTLB when the operations are possible.  It needs to be called
// before the SWIOTLB memory is used.
//
#[no_mangle]
pub unsafe extern "C" fn swiotlb_update_mem_attributes()  {
    let mut mem = &io_tlb_default_mem.defpool;
    let mut bytes = 0;
//
// if platform support memory encryption, swiotlb buffers are
// shared by default.
//
    if (cc_platform_has(CC_ATTR_MEM_ENCRYPT)) {
    io_tlb_default_mem.cc_shared = true;
    }
    else {
    io_tlb_default_mem.cc_shared = false;
    }
    if (!mem.nslabs || mem.late_alloc) {
    return;
    }
    bytes = PAGE_ALIGN(mem.nslabs << IO_TLB_SHIFT);
    if (io_tlb_default_mem.cc_shared) {
    let mut ret = 0;
    ret = set_memory_decrypted((unsigned long)mem.vaddr,
    bytes >> PAGE_SHIFT);
    if (ret) {
    pr_warn!("Failed to decrypt default memory pool, disabling it\n");
    swiotlb_mark_pool_used(mem);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn swiotlb_init_io_tlb_pool(mem: *mut io_tlb_pool, start: phys_addr_t, vaddr: *mut c_void, nslabs: c_ulong, late_alloc: bool, nareas: c_uint) {
pub static mut bytes: c_ulong = 0;
    mem.nslabs = nslabs;
    mem.start = start;
    mem.end = mem.start + bytes;
    mem.late_alloc = late_alloc;
    mem.nareas = nareas;
    mem.area_nslabs = nslabs / mem.nareas;
    while (i < mem.nareas) {
    spin_lock_init(&mem.areas[i].lock);
    mem.areas[i].index = 0;
    mem.areas[i].used = 0;
    }
    while (i < mem.nslabs) {
    mem.slots[i].list = min(IO_TLB_SEGSIZE - io_tlb_offset(i),
    mem.nslabs - i);
    mem.slots[i].orig_addr = INVALID_PHYS_ADDR;
    mem.slots[i].alloc_size = 0;
    mem.slots[i].pad_slots = 0;
    }
    memset(vaddr, 0, bytes);
    mem.vaddr = vaddr;
    return;
    }
//
// add_mem_pool() - add a memory pool to the allocator
// @mem:	Software IO TLB allocator.
// @pool:	Memory pool to be added.
//
#[no_mangle]
unsafe extern "C" fn add_mem_pool(mem: *mut io_tlb_mem, pool: *mut io_tlb_pool) {

    spin_lock(&mem.lock);
    list_add_rcu(&pool.node, &mem.pools);
    mem.nslabs += pool.nslabs;
    spin_unlock(&mem.lock);

    mem.nslabs = pool.nslabs;

    }
    static void __init *swiotlb_memblock_alloc(unsigned long nslabs,
    unsigned int flags,
    int (*remap)(void *tlb, unsigned long nslabs))
    {
pub static mut bytes: usize = 0;
pub static mut tlb: *mut c_void = core::ptr::null_mut();
//
// By default allocate the bounce buffer memory from low memory, but
// allow to pick a location everywhere for hypervisors with guest
// memory encryption.
//
    if (flags & SWIOTLB_ANY) {
    tlb = memblock_alloc(bytes, PAGE_SIZE);
    }
    else {
    tlb = memblock_alloc_low(bytes, PAGE_SIZE);
    }
    if (!tlb) {
    pr_warn!("%s: Failed to allocate %zu bytes tlb structure\n",
    __func__, bytes);
    return core::ptr::null_mut();
    }
    if (remap && remap(tlb, nslabs) < 0) {
    memblock_free(tlb, PAGE_ALIGN(bytes));
    pr_warn!("%s: Failed to remap %zu bytes\n", __func__, bytes);
    return core::ptr::null_mut();
    }
    return tlb;
    }
//
// Statically reserve bounce buffer space and initialize bounce buffer data
// structures for the software IO TLB used to implement the DMA API.
//
    void __init swiotlb_init_remap(bool addressing_limit, unsigned int flags,
    int (*remap)(void *tlb, unsigned long nslabs))
    {
    let mut mem = &io_tlb_default_mem.defpool;
    let mut nslabs = 0;
    let mut nareas = 0;
    let mut alloc_size = 0;
pub static mut tlb: *mut c_void = core::ptr::null_mut();
    if (!addressing_limit && !swiotlb_force_bounce) {
    return;
    }
    if (swiotlb_force_disable) {
    return;
    }
    io_tlb_default_mem.force_bounce = swiotlb_force_bounce;

    if (!remap) {
    io_tlb_default_mem.can_grow = true;
    }
    if (flags & SWIOTLB_ANY) {
    io_tlb_default_mem.phys_limit = virt_to_phys(high_memory - 1);
    }
    else {
    io_tlb_default_mem.phys_limit = ARCH_LOW_ADDRESS_LIMIT;
    }

    if (!default_nareas) {
    swiotlb_adjust_nareas(num_possible_cpus());
    }
    nslabs = default_nslabs;
    nareas = limit_nareas(default_nareas, nslabs);
    while ((tlb = swiotlb_memblock_alloc(nslabs, flags, remap)) == core::ptr::null_mut()) {
    if (nslabs <= IO_TLB_MIN_SLABS) {
    return;
    }
    nslabs = ALIGN(nslabs >> 1, IO_TLB_SEGSIZE);
    nareas = limit_nareas(nareas, nslabs);
    }
    if (default_nslabs != nslabs) {
    pr_info!("SWIOTLB bounce buffer size adjusted %lu . %lu slabs",
    default_nslabs, nslabs);
    default_nslabs = nslabs;
    }
    alloc_size = PAGE_ALIGN(array_size(sizeof!(*mem.slots), nslabs));
    mem.slots = memblock_alloc(alloc_size, PAGE_SIZE);
    if (!mem.slots) {
    pr_warn!("%s: Failed to allocate %zu bytes align=0x%lx\n",
    __func__, alloc_size, PAGE_SIZE);
    return;
    }
    mem.areas = memblock_alloc(array_size(sizeof!(io_tlb_area),
    nareas), SMP_CACHE_BYTES);
    if (!mem.areas) {
    pr_warn!("%s: Failed to allocate mem.areas.\n", __func__);
    return;
    }
    swiotlb_init_io_tlb_pool(mem, __pa(tlb), tlb, nslabs, false, nareas);
    add_mem_pool(&io_tlb_default_mem, mem);
    if (flags & SWIOTLB_VERBOSE) {
    swiotlb_print_info();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn swiotlb_init(addressing_limit: bool, flags: c_uint)  {
    swiotlb_init_remap(addressing_limit, flags, core::ptr::null_mut());
    }
//
// Systems with larger DMA zones (those that don't support ISA) can
// initialize the swiotlb later using the slab allocator if needed.
// This should be just like above, but with some error catching.
//
#[no_mangle]
pub unsafe extern "C" fn swiotlb_init_late(size: size_t, gfp_mask: gfp_t, tlb: *mut *mut int (remap)(void) -> c_int {
    let mut mem = &io_tlb_default_mem.defpool;
pub static mut nslabs: c_ulong = 0;
    let mut order = 0;
    let mut area_order = 0;
    let mut slot_order = 0;
pub static mut leak_pages: bool = false;
    let mut nareas = 0;
    let mut vstart = core::ptr::null_mut();
pub static mut retried: bool = false;
pub static mut rc: c_int = 0;
    if (io_tlb_default_mem.nslabs) {
    return 0;
    }
    if (swiotlb_force_disable) {
    return 0;
    }
    io_tlb_default_mem.force_bounce = swiotlb_force_bounce;

    if (!remap) {
    io_tlb_default_mem.can_grow = true;
    }
    if (IS_ENABLED!(CONFIG_ZONE_DMA) && (gfp_mask & __GFP_DMA)) {
    io_tlb_default_mem.phys_limit = zone_dma_limit;
    }

    else if (IS_ENABLED!(CONFIG_ZONE_DMA32) && (gfp_mask & __GFP_DMA32)) {
    io_tlb_default_mem.phys_limit = max(DMA_BIT_MASK(32), zone_dma_limit);
    }
    else {
    io_tlb_default_mem.phys_limit = virt_to_phys(high_memory - 1);
    }

    if (!default_nareas) {
    swiotlb_adjust_nareas(num_possible_cpus());
    }
// label;
    order = get_order(nslabs << IO_TLB_SHIFT);
    nslabs = SLABS_PER_PAGE << order;
    while ((SLABS_PER_PAGE << order) > IO_TLB_MIN_SLABS) {
    vstart = __get_free_pages(gfp_mask | __GFP_NOWARN,
    order);
    if (vstart) {
    break;
    }
    order -= 1;
    nslabs = SLABS_PER_PAGE << order;
    retried = true;
    }
    if (!vstart) {
    return -ENOMEM;
    }
    if (remap) {
    rc = remap(vstart, nslabs);
    }
    if (rc) {
    free_pages((unsigned long)vstart, order);
    nslabs = ALIGN(nslabs >> 1, IO_TLB_SEGSIZE);
    if (nslabs < IO_TLB_MIN_SLABS) {
    return rc;
    }
    retried = true;
// goto;
    }
    if (retried) {
    pr_warn!("only able to allocate %ld MB\n",
    (PAGE_SIZE << order) >> 20);
    }
    rc = -ENOMEM;
    nareas = limit_nareas(default_nareas, nslabs);
    area_order = get_order(array_size(sizeof!(*mem.areas), nareas));
    mem.areas = 
    __get_free_pages(GFP_KERNEL | __GFP_ZERO, area_order);
    if (!mem.areas) {
// goto;
    }
    slot_order = get_order(array_size(sizeof!(*mem.slots), nslabs));
    mem.slots = __get_free_pages(GFP_KERNEL | __GFP_ZERO,
    slot_order);
    if (!mem.slots) {
// goto;
    }
    if (io_tlb_default_mem.cc_shared) {
    rc = set_memory_decrypted((unsigned long)vstart,
    (nslabs << IO_TLB_SHIFT) >> PAGE_SHIFT);
    if (rc) {
    leak_pages = true;
// goto;
    }
    }
    swiotlb_init_io_tlb_pool(mem, virt_to_phys(vstart), vstart, nslabs, true,
    nareas);
    add_mem_pool(&io_tlb_default_mem, mem);
    swiotlb_print_info();
    return 0;
// label;
    free_pages((unsigned long)mem.slots, slot_order);
// label;
    free_pages((unsigned long)mem.areas, area_order);
// label;
    if (!leak_pages) {
    free_pages((unsigned long)vstart, order);
    }
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn swiotlb_exit()  {
    let mut mem = &io_tlb_default_mem.defpool;
pub static mut leak_pages: bool = false;
    let mut tbl_vaddr = 0;
    size_t tbl_size, slots_size;
    let mut area_order = 0;
    if (swiotlb_force_bounce) {
    return;
    }
    if (!mem.nslabs) {
    return;
    }
    pr_info!("tearing down default memory pool\n");
    tbl_vaddr = (unsigned long)phys_to_virt(mem.start);
    tbl_size = PAGE_ALIGN(mem.end - mem.start);
    slots_size = PAGE_ALIGN(array_size(sizeof!(*mem.slots), mem.nslabs));
    if (io_tlb_default_mem.cc_shared) {
    if (set_memory_encrypted(tbl_vaddr, tbl_size >> PAGE_SHIFT)) {
    leak_pages = true;
    }
    }
    if (mem.late_alloc) {
    area_order = get_order(array_size(sizeof!(*mem.areas),
    mem.nareas));
    free_pages((unsigned long)mem.areas, area_order);
    if (!leak_pages) {
    free_pages(tbl_vaddr, get_order(tbl_size));
    }
    free_pages((unsigned long)mem.slots, get_order(slots_size));
    } else {
    memblock_free(mem.areas,
    array_size(sizeof!(*mem.areas), mem.nareas));
    if (!leak_pages) {
    memblock_phys_free(mem.start, tbl_size);
    }
    memblock_free(mem.slots, slots_size);
    }
    memset(mem, 0, sizeof!(*mem));
    }

//
// alloc_dma_pages() - allocate pages to be used for DMA
// @gfp:	GFP flags for the allocation.
// @bytes:	Size of the buffer.
// @phys_limit:	Maximum allowed physical address of the buffer.
// @attrs:	DMA attributes for the allocation.
//
// Allocate pages from the buddy allocator. If successful, make the allocated
// pages decrypted that they can be used for DMA.
//
// Return: Decrypted pages, %NULL on allocation failure, or ERR_PTR(-EAGAIN)
// if the allocated physical address was above @phys_limit.
//
#[no_mangle]
pub unsafe extern "C" fn alloc_dma_pages(gfp: gfp_t, bytes: size_t, phys_limit: u64, attrs: c_ulong) -> *mut c_void {
pub static mut order: c_uint = 0;
pub static mut cc_shared: bool = false;
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut paddr;
pub static mut vaddr: *mut c_void = core::ptr::null_mut();
    page = alloc_pages(gfp, order);
    if (!page) {
    return core::ptr::null_mut();
    }
    paddr = page_to_phys(page);
    if (paddr + bytes - 1 > phys_limit) {
    __free_pages(page, order);
    return ERR_PTR(-EAGAIN);
    }
    vaddr = phys_to_virt(paddr);
    if (cc_shared && set_memory_decrypted((unsigned long)vaddr, PFN_UP(bytes))) {
// goto;
    }
    return page;
// label;
// Intentional leak if pages cannot be encrypted again.
    if (cc_shared && !set_memory_encrypted((unsigned long)vaddr, PFN_UP(bytes))) {
    __free_pages(page, order);
    }
    return core::ptr::null_mut();
    }
//
// swiotlb_alloc_tlb() - allocate a dynamic IO TLB buffer
// @dev:	Device for which a memory pool is allocated.
// @mem:	SWIOTLB allocator for the pool.
// @bytes:	Size of the buffer.
// @phys_limit:	Maximum allowed physical address of the buffer.
// @gfp:	GFP flags for the allocation.
// @vaddr:	Receives the virtual address for the allocated buffer.
//
// Return: Allocated pages, or %NULL on allocation failure.
//
#[no_mangle]
pub unsafe extern "C" fn swiotlb_alloc_tlb(dev: *mut device, mem: *mut io_tlb_mem, bytes: size_t, phys_limit: u64, gfp: gfp_t, vaddr: *mut *mut c_void) -> *mut c_void {
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut attrs: c_ulong = 0;
// vaddr = NULL;
//
// Allocate from the atomic pools if memory is encrypted and
// the allocation is atomic, because decrypting may block.
//
    if (!gfpflags_allow_blocking(gfp) && dev && mem.cc_shared) {
    if (!IS_ENABLED!(CONFIG_DMA_COHERENT_POOL)) {
    return core::ptr::null_mut();
    }
    return dma_alloc_from_pool(dev, bytes, vaddr, gfp,
    attrs, dma_coherent_ok);
    }
    gfp &= ~GFP_ZONEMASK;
    if (phys_limit <= zone_dma_limit) {
    gfp |= __GFP_DMA;
    }

    else if (phys_limit <= DMA_BIT_MASK(32)) {
    gfp |= __GFP_DMA32;
    }
    while (IS_ERR(page = alloc_dma_pages(gfp, bytes, phys_limit, attrs))) {
    if (IS_ENABLED!(CONFIG_ZONE_DMA32) &&
    phys_limit < DMA_BIT_MASK(64) &&
    !(gfp & (__GFP_DMA32 | __GFP_DMA))) {
    gfp |= __GFP_DMA32;
    }
    else if (IS_ENABLED!(CONFIG_ZONE_DMA) &&
    !(gfp & __GFP_DMA)) {
    gfp = (gfp & ~__GFP_DMA32) | __GFP_DMA;
    }
    else {
    return core::ptr::null_mut();
    }
    }
    if (page) {
// vaddr = phys_to_virt(page_to_phys(page));
    }
    return page;
    }
//
// swiotlb_free_tlb() - free a dynamically allocated IO TLB buffer
// @vaddr:	Virtual address of the buffer.
// @bytes:	Size of the buffer.
// @cc_shared: true if @vaddr was allocated decrypted and must be
// re-encrypted before being freed
//
#[no_mangle]
unsafe extern "C" fn swiotlb_free_tlb(vaddr: *mut c_void, bytes: usize, cc_shared: bool) {
    if (IS_ENABLED!(CONFIG_DMA_COHERENT_POOL) &&
    dma_free_from_pool(core::ptr::null_mut(), vaddr, bytes)) {
    return;
    }
// Intentional leak if pages cannot be encrypted again.
    if (!cc_shared ||
    !set_memory_encrypted((unsigned long)vaddr, PFN_UP(bytes))) {
    __free_pages(virt_to_page(vaddr), get_order(bytes));
    }
    }
//
// swiotlb_alloc_pool() - allocate a new IO TLB memory pool
// @dev:	Device for which a memory pool is allocated.
// @mem:	SWIOTLB allocator for the pool.
// @minslabs:	Minimum number of slabs.
// @nslabs:	Desired (maximum) number of slabs.
// @nareas:	Number of areas.
// @phys_limit:	Maximum DMA buffer physical address.
// @gfp:	GFP flags for the allocations.
//
// Allocate and initialize a new IO TLB memory pool. The actual number of
// slabs may be reduced if allocation of @nslabs fails. If even
// @minslabs cannot be allocated, this function fails.
//
// Return: New memory pool, or %NULL on allocation failure.
//
#[no_mangle]
pub unsafe extern "C" fn swiotlb_alloc_pool(dev: *mut device, mem: *mut io_tlb_mem, minslabs: c_ulong, nslabs: c_ulong, nareas: c_uint, phys_limit: u64, gfp: gfp_t) -> *mut c_void {
pub static mut pool: *mut c_void = core::ptr::null_mut();
    let mut slot_order = 0;
pub static mut tlb_vaddr: *mut c_void = core::ptr::null_mut();
pub static mut tlb: *mut c_void = core::ptr::null_mut();
    let mut pool_size = 0;
    let mut tlb_size = 0;
    if (nslabs > SLABS_PER_PAGE << MAX_PAGE_ORDER) {
    nslabs = SLABS_PER_PAGE << MAX_PAGE_ORDER;
    nareas = limit_nareas(nareas, nslabs);
    }
    pool_size = sizeof!(*pool) + array_size(sizeof!(*pool.areas), nareas);
    pool = kzalloc(pool_size, gfp);
    if (!pool) {
// goto;
    }
    pool.areas = pool + sizeof!(*pool);
    pool.cc_shared = mem.cc_shared;
    tlb_size = nslabs << IO_TLB_SHIFT;
    while (!(tlb = swiotlb_alloc_tlb(dev, mem, tlb_size,
    phys_limit, gfp, &tlb_vaddr))) {
    if (nslabs <= minslabs) {
// goto;
    }
    nslabs = ALIGN(nslabs >> 1, IO_TLB_SEGSIZE);
    nareas = limit_nareas(nareas, nslabs);
    tlb_size = nslabs << IO_TLB_SHIFT;
    }
    slot_order = get_order(array_size(sizeof!(*pool.slots), nslabs));
    pool.slots = 
    __get_free_pages(gfp, slot_order);
    if (!pool.slots) {
// goto;
    }
    swiotlb_init_io_tlb_pool(pool, page_to_phys(tlb), tlb_vaddr, nslabs,
    true, nareas);
    return pool;
// label;
    swiotlb_free_tlb(tlb_vaddr, tlb_size, mem.cc_shared);
// label;
    kfree(pool);
// label;
    return core::ptr::null_mut();
    }
//
// swiotlb_dyn_alloc() - dynamic memory pool allocation worker
// @work:	Pointer to dyn_alloc in struct io_tlb_mem.
//
#[no_mangle]
unsafe extern "C" fn swiotlb_dyn_alloc(work: *mut work_struct) {
    let mut mem = container_of!(work, io_tlb_mem, dyn_alloc);
pub static mut pool: *mut c_void = core::ptr::null_mut();
    pool = swiotlb_alloc_pool(core::ptr::null_mut(), mem, IO_TLB_MIN_SLABS, default_nslabs,
    default_nareas, mem.phys_limit, GFP_KERNEL);
    if (!pool) {
    pr_warn_ratelimited("Failed to allocate new pool");
    return;
    }
    add_mem_pool(mem, pool);
    }
#[no_mangle]
unsafe extern "C" fn swiotlb_dyn_free_work(work: *mut work_struct) {
    let mut pool = container_of!(to_rcu_work(work), io_tlb_pool, dyn_free);
pub static mut slots_size: usize = 0;
pub static mut tlb_size: usize = 0;
    free_pages((unsigned long)pool.slots, get_order(slots_size));
    swiotlb_free_tlb(pool.vaddr, tlb_size, pool.cc_shared);
    kfree(pool);
    }
#[no_mangle]
unsafe extern "C" fn swiotlb_schedule_dyn_free(pool: *mut io_tlb_pool) {
    INIT_RCU_WORK(&pool.dyn_free, swiotlb_dyn_free_work);
    queue_rcu_work(system_wq, &pool.dyn_free);
    }
//
// __swiotlb_find_pool() - find the IO TLB pool for a physical address
// @dev:        Device which has mapped the DMA buffer.
// @paddr:      Physical address within the DMA buffer.
//
// Find the IO TLB memory pool descriptor which contains the given physical
// address, if any. This function is for use only when the dev is known to
// be using swiotlb. Use swiotlb_find_pool() for the more general case
// when this condition is not met.
//
// Return: Memory pool which contains @paddr, or %NULL if none.
//
#[no_mangle]
pub unsafe extern "C" fn __swiotlb_find_pool(dev: *mut device, paddr: phys_addr_t) -> *mut c_void {
    let mut mem = dev.dma_io_tlb_mem;
pub static mut pool: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    list_for_each_entry_rcu(pool, &mem.pools, node) {
    if (paddr >= pool.start && paddr < pool.end) {
// goto;
    }
    }
    list_for_each_entry_rcu(pool, &dev.dma_io_tlb_pools, node) {
    if (paddr >= pool.start && paddr < pool.end) {
// goto;
    }
    }
    pool = core::ptr::null_mut();
// label;
    rcu_read_unlock();
    return pool;
    }
//
// swiotlb_del_pool() - remove an IO TLB pool from a device
// @dev:	Owning device.
// @pool:	Memory pool to be removed.
//
#[no_mangle]
unsafe extern "C" fn swiotlb_del_pool(dev: *mut device, pool: *mut io_tlb_pool) {
    let mut flags = 0;
    spin_lock_irqsave(&dev.dma_io_tlb_lock, flags);
    list_del_rcu(&pool.node);
    spin_unlock_irqrestore(&dev.dma_io_tlb_lock, flags);
    swiotlb_schedule_dyn_free(pool);
    }

//
// swiotlb_dev_init() - initialize swiotlb fields in &struct device
// @dev:	Device to be initialized.
//
#[no_mangle]
pub unsafe extern "C" fn swiotlb_dev_init(dev: *mut device) {
    dev.dma_io_tlb_mem = &io_tlb_default_mem;

    INIT_LIST_HEAD(&dev.dma_io_tlb_pools);
    spin_lock_init(&dev.dma_io_tlb_lock);
    dev.dma_uses_io_tlb = false;

    }
//
// swiotlb_align_offset() - Get required offset into an IO TLB allocation.
// @dev:         Owning device.
// @align_mask:  Allocation alignment mask.
// @addr:        DMA address.
//
// Return the minimum offset from the start of an IO TLB allocation which is
// required for a given buffer address and allocation alignment to keep the
// device happy.
//
// First, the address bits covered by min_align_mask must be identical in the
// original address and the bounce buffer address. High bits are preserved by
// choosing a suitable IO TLB slot, but bits below IO_TLB_SHIFT require extra
// padding bytes before the bounce buffer.
//
// Second, @align_mask specifies which bits of the first allocated slot must
// be zero. This may require allocating additional padding slots, and then the
// offset (in bytes) from the first such padding slot is returned.
//
#[no_mangle]
pub unsafe extern "C" fn swiotlb_align_offset(dev: *mut device, align_mask: c_uint, addr: u64) -> c_uint {
    return addr & dma_get_min_align_mask(dev) &
    (align_mask | (IO_TLB_SIZE - 1));
    }
//
// Bounce: copy the swiotlb buffer from or back to the original dma location
//
#[no_mangle]
pub unsafe extern "C" fn swiotlb_bounce(dev: *mut device, tlb_addr: phys_addr_t, size: size_t, dir: dma_data_direction, mem: *mut io_tlb_pool) {
pub static mut index: c_int = 0;
pub static mut orig_addr: phys_addr_t = 0;
pub static mut alloc_size: usize = 0;
pub static mut pfn: c_ulong = 0;
    let mut vaddr = mem.vaddr + tlb_addr - mem.start;
    let mut tlb_offset = 0;
    if (orig_addr == INVALID_PHYS_ADDR) {
    return;
    }
    if (dir == DMA_FROM_DEVICE && !dev_is_dma_coherent(dev)) {
    arch_sync_dma_flush();
    }
//
// It's valid for tlb_offset to be negative. This can happen when the
// "offset" returned by swiotlb_align_offset() is non-zero, and the
// tlb_addr is pointing within the first "offset" bytes of the second
// or subsequent slots of the allocated swiotlb area. While it's not
// valid for tlb_addr to be pointing within the first "offset" bytes
// of the first slot, there's no way to check for such an error since
// this function can't distinguish the first slot from the second and
// subsequent slots.
//
    tlb_offset = (tlb_addr & (IO_TLB_SIZE - 1)) -
    swiotlb_align_offset(dev, 0, orig_addr);
    orig_addr += tlb_offset;
    alloc_size -= tlb_offset;
    if (size > alloc_size) {
    dev_WARN_ONCE(dev, 1,
    "Buffer overflow detected. Allocation size: %zu. Mapping size: %zu.\n",
    alloc_size, size);
    size = alloc_size;
    }
    if (PageHighMem(pfn_to_page(pfn))) {
pub static mut offset: c_uint = 0;
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut sz: c_uint = 0;
    let mut flags = 0;
    while (size) {
    sz = min_t(size_t, PAGE_SIZE - offset, size);
    local_irq_save(flags);
    page = pfn_to_page(pfn);
    if (dir == DMA_TO_DEVICE) {
//
// Ideally, kmsan_check_highmem_page()
// could be used here to detect infoleaks,
// but callers may map uninitialized buffers
// that will be written by the device,
// causing false positives.
//
    memcpy_from_page(vaddr, page, offset, sz);
    } else {
    kmsan_unpoison_memory(vaddr, sz);
    memcpy_to_page(page, offset, vaddr, sz);
    }
    local_irq_restore(flags);
    size -= sz;
    pfn += 1;
    vaddr += sz;
    offset = 0;
    }
    } else if (dir == DMA_TO_DEVICE) {
//
// Ideally, kmsan_check_memory() could be used here to detect
// infoleaks (uninitialized data being sent to device), but
// callers may map uninitialized buffers that will be written
// by the device, causing false positives.
//
    memcpy(vaddr, phys_to_virt(orig_addr), size);
    } else {
    kmsan_unpoison_memory(vaddr, size);
    memcpy(phys_to_virt(orig_addr), vaddr, size);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn slot_addr(start: phys_addr_t, idx: phys_addr_t) -> phys_addr_t {
    return start + (idx << IO_TLB_SHIFT);
    }
//
// Carefully handle integer overflow which can occur when boundary_mask == ~0UL.
//
#[no_mangle]
pub unsafe extern "C" fn get_max_slots(boundary_mask: c_ulong) -> c_ulong {
    return (boundary_mask >> IO_TLB_SHIFT) + 1;
    }
#[no_mangle]
unsafe extern "C" fn wrap_area_index(mem: *mut io_tlb_pool, index: c_uint) -> c_uint {
    if (index >= mem.area_nslabs) {
    return 0;
    }
    return index;
    }

#[no_mangle]
unsafe extern "C" fn inc_transient_used(mem: *mut io_tlb_mem, nslots: c_uint) {
    atomic_long_add(nslots, &mem.transient_nslabs);
    }
#[no_mangle]
unsafe extern "C" fn dec_transient_used(mem: *mut io_tlb_mem, nslots: c_uint) {
    atomic_long_sub(nslots, &mem.transient_nslabs);
    }

#[no_mangle]
unsafe extern "C" fn inc_transient_used(mem: *mut io_tlb_mem, nslots: c_uint) {
    }
#[no_mangle]
unsafe extern "C" fn dec_transient_used(mem: *mut io_tlb_mem, nslots: c_uint) {
    }

//
// swiotlb_search_pool_area() - search one memory area in one pool
// @dev:	Device which maps the buffer.
// @pool:	Memory pool to be searched.
// @area_index:	Index of the IO TLB memory area to be searched.
// @orig_addr:	Original (non-bounced) IO buffer address.
// @tbl_dma_addr: DMA address of the bounce buffer.
// @alloc_size: Total requested size of the bounce buffer,
// including initial alignment padding.
// @alloc_align_mask:	Required alignment of the allocated buffer.
//
// Find a suitable sequence of IO TLB entries for the request and allocate
// a buffer from the given IO TLB memory area.
// This function takes care of locking.
//
// Return: Index of the first allocated slot, or -1 on error.
//
#[no_mangle]
pub unsafe extern "C" fn swiotlb_search_pool_area(dev: *mut device, pool: *mut io_tlb_pool, area_index: c_int, orig_addr: phys_addr_t, tbl_dma_addr: dma_addr_t, alloc_size: size_t, alloc_align_mask: c_uint) -> c_int {
    let mut area = pool.areas + area_index;
pub static mut boundary_mask: c_ulong = 0;
pub static mut max_slots: c_ulong = 0;
pub static mut iotlb_align_mask: c_uint = 0;
pub static mut nslots: c_uint = 0;
pub static mut offset: c_uint = 0;
    unsigned int index, slots_checked, count = 0, i;
    let mut flags = 0;
    let mut slot_base = 0;
    let mut slot_index = 0;
    BUG_ON!(!nslots);
    BUG_ON!(area_index >= pool.nareas);
    tbl_dma_addr &= boundary_mask;
//
// Historically, swiotlb allocations >= PAGE_SIZE were guaranteed to be
// page-aligned in the absence of any other alignment requirements.
// 'alloc_align_mask' was later introduced to specify the alignment
// explicitly, however this is passed as zero for streaming mappings
// and so we preserve the old behaviour there in case any drivers are
// relying on it.
//
    if (!alloc_align_mask && !iotlb_align_mask && alloc_size >= PAGE_SIZE) {
    alloc_align_mask = PAGE_SIZE - 1;
    }
//
// Ensure that the allocation is at least slot-aligned and update
// 'iotlb_align_mask' to ignore bits that will be preserved when
// offsetting into the allocation.
//
    alloc_align_mask |= (IO_TLB_SIZE - 1);
    iotlb_align_mask &= ~alloc_align_mask;
//
// For mappings with an alignment requirement don't bother looping to
// unaligned slots once we found an aligned one.
//
    stride = get_max_slots(max(alloc_align_mask, iotlb_align_mask));
    spin_lock_irqsave(&area.lock, flags);
    if (unlikely(nslots > pool.area_nslabs - area.used)) {
// goto;
    }
    slot_base = area_index * pool.area_nslabs;
    index = area.index;
    while (slots_checked < pool.area_nslabs) {
    let mut tlb_addr;
    slot_index = slot_base + index;
    tlb_addr = slot_addr(tbl_dma_addr, slot_index);
    if ((tlb_addr & alloc_align_mask) ||
    (orig_addr && (tlb_addr & iotlb_align_mask) !=
    (orig_addr & iotlb_align_mask))) {
    index = wrap_area_index(pool, index + 1);
    slots_checked += 1;
    continue;
    }
    if (!iommu_is_span_boundary(slot_index, nslots,
    nr_slots(tbl_dma_addr),
    max_slots)) {
    if (pool.slots[slot_index].list >= nslots) {
// goto;
    }
    }
    index = wrap_area_index(pool, index + stride);
    slots_checked += stride;
    }
// label;
    spin_unlock_irqrestore(&area.lock, flags);
    return -1;
// label;
//
// If we find a slot that indicates we have 'nslots' number of
// contiguous buffers, we allocate the buffers from that slot onwards
// and set the list of free entries to '0' indicating unavailable.
//
    while (i < slot_index + nslots) {
    pool.slots[i].list = 0;
    pool.slots[i].alloc_size = alloc_size - (offset +
    ((i - slot_index) << IO_TLB_SHIFT));
    }
    for (i = slot_index - 1;
    io_tlb_offset(i) != IO_TLB_SEGSIZE - 1 &&
    pool.slots[i].list; i--) {
    pool.slots[i].list = count += 1;
    }
//
// Update the indices to avoid searching in the next round.
//
    area.index = wrap_area_index(pool, index + nslots);
    area.used += nslots;
    spin_unlock_irqrestore(&area.lock, flags);
    inc_used_and_hiwater(dev.dma_io_tlb_mem, nslots);
    return slot_index;
    }

//
// swiotlb_search_area() - search one memory area in all pools
// @dev:	Device which maps the buffer.
// @start_cpu:	Start CPU number.
// @cpu_offset:	Offset from @start_cpu.
// @orig_addr:	Original (non-bounced) IO buffer address.
// @alloc_size: Total requested size of the bounce buffer,
// including initial alignment padding.
// @alloc_align_mask:	Required alignment of the allocated buffer.
// @retpool:	Used memory pool, updated on return.
//
// Search one memory area in all pools for a sequence of slots that match the
// allocation constraints.
//
// Return: Index of the first allocated slot, or -1 on error.
//
#[no_mangle]
pub unsafe extern "C" fn swiotlb_search_area(dev: *mut device, start_cpu: c_int, cpu_offset: c_int, orig_addr: phys_addr_t, alloc_size: size_t, alloc_align_mask: c_uint, retpool: *mut *mut io_tlb_pool) -> c_int {
    let mut mem = dev.dma_io_tlb_mem;
pub static mut pool: *mut c_void = core::ptr::null_mut();
    let mut tbl_dma_addr;
    let mut area_index = 0;
pub static mut index: c_int = 0;
    rcu_read_lock();
    list_for_each_entry_rcu(pool, &mem.pools, node) {
    if (cpu_offset >= pool.nareas) {
    continue;
    }
    area_index = (start_cpu + cpu_offset) & (pool.nareas - 1);
    if (mem.cc_shared) {
    tbl_dma_addr = phys_to_dma_unencrypted(dev, pool.start);
    }
    else {
    tbl_dma_addr = phys_to_dma_encrypted(dev, pool.start);
    }
    index = swiotlb_search_pool_area(dev, pool, area_index,
    orig_addr, tbl_dma_addr,
    alloc_size, alloc_align_mask);
    if (index >= 0) {
// retpool = pool;
    break;
    }
    }
    rcu_read_unlock();
    return index;
    }
//
// swiotlb_find_slots() - search for slots in the whole swiotlb
// @dev:	Device which maps the buffer.
// @orig_addr:	Original (non-bounced) IO buffer address.
// @alloc_size: Total requested size of the bounce buffer,
// including initial alignment padding.
// @alloc_align_mask:	Required alignment of the allocated buffer.
// @retpool:	Used memory pool, updated on return.
//
// Search through the whole software IO TLB to find a sequence of slots that
// match the allocation constraints.
//
// Return: Index of the first allocated slot, or -1 on error.
//
#[no_mangle]
pub unsafe extern "C" fn swiotlb_find_slots(dev: *mut device, orig_addr: phys_addr_t, alloc_size: size_t, alloc_align_mask: c_uint, retpool: *mut *mut io_tlb_pool) -> c_int {
    let mut mem = dev.dma_io_tlb_mem;
pub static mut pool: *mut c_void = core::ptr::null_mut();
    let mut tbl_dma_addr;
    let mut nslabs = 0;
    let mut flags = 0;
    let mut phys_limit = 0;
    let mut cpu = 0;
    let mut i = 0;
    let mut index = 0;
    if (alloc_size > IO_TLB_SEGSIZE * IO_TLB_SIZE) {
    return -1;
    }
    cpu = raw_smp_processor_id();
    while (i < default_nareas) {
    index = swiotlb_search_area(dev, cpu, i, orig_addr, alloc_size,
    alloc_align_mask, &pool);
    if (index >= 0) {
// goto;
    }
    }
    if (!mem.can_grow) {
    return -1;
    }
    schedule_work(&mem.dyn_alloc);
    nslabs = nr_slots(alloc_size);
    phys_limit = min_not_zero(*dev.dma_mask, dev.bus_dma_limit);
    pool = swiotlb_alloc_pool(dev, mem, nslabs, nslabs, 1, phys_limit,
    GFP_NOWAIT);
    if (!pool) {
    return -1;
    }
    if (mem.cc_shared) {
    tbl_dma_addr = phys_to_dma_unencrypted(dev, pool.start);
    }
    else {
    tbl_dma_addr = phys_to_dma_encrypted(dev, pool.start);
    }
    index = swiotlb_search_pool_area(dev, pool, 0, orig_addr, tbl_dma_addr,
    alloc_size, alloc_align_mask);
    if (index < 0) {
    swiotlb_schedule_dyn_free(pool);
    return -1;
    }
    pool.transient = true;
    spin_lock_irqsave(&dev.dma_io_tlb_lock, flags);
    list_add_rcu(&pool.node, &dev.dma_io_tlb_pools);
    spin_unlock_irqrestore(&dev.dma_io_tlb_lock, flags);
    inc_transient_used(mem, pool.nslabs);
// label;
    WRITE_ONCE(dev.dma_uses_io_tlb, true);
//
// The general barrier orders reads and writes against a presumed store
// of the SWIOTLB buffer address by a device driver (to a driver private
// data structure). It serves two purposes.
//
// First, the store to dev->dma_uses_io_tlb must be ordered before the
// presumed store. This guarantees that the returned buffer address
// cannot be passed to another CPU before updating dev->dma_uses_io_tlb.
//
// Second, the load from mem->pools must be ordered before the same
// presumed store. This guarantees that the returned buffer address
// cannot be observed by another CPU before an update of the RCU list
// that was made by swiotlb_dyn_alloc() on a third CPU (cf. multicopy
// atomicity).
//
// See also the comment in swiotlb_find_pool().
//
    smp_mb();
// retpool = pool;
    return index;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: swiotlb_find_slots
pub unsafe extern "C" fn swiotlb_find_slots_dup(dev: *mut device, orig_addr: phys_addr_t, alloc_size: size_t, alloc_align_mask: c_uint, retpool: *mut *mut io_tlb_pool) -> c_int {
    let mut mem = dev.dma_io_tlb_mem;
pub static mut pool: *mut c_void = core::ptr::null_mut();
    let mut tbl_dma_addr;
    let mut start = 0;
    let mut i = 0;
    let mut index = 0;
// retpool = pool = &mem->defpool;
    if (mem.cc_shared) {
    tbl_dma_addr = phys_to_dma_unencrypted(dev, pool.start);
    }
    else {
    tbl_dma_addr = phys_to_dma_encrypted(dev, pool.start);
    }
    i = start = raw_smp_processor_id() & (pool.nareas - 1);
    do {
    index = swiotlb_search_pool_area(dev, pool, i, orig_addr,
    tbl_dma_addr, alloc_size,
    alloc_align_mask);
    if (index >= 0) {
    return index;
    }
    if (++i >= pool.nareas) {
    i = 0;
    }
    } while (i != start);
    return -1;
    }

//
// mem_pool_used() - get number of used slots in a memory pool
// @pool:	Software IO TLB memory pool.
//
// The result is not accurate, see mem_used().
//
// Return: Approximate number of used slots.
//
#[no_mangle]
unsafe extern "C" fn mem_pool_used(pool: *mut io_tlb_pool) -> c_ulong {
    let mut i = 0;
pub static mut used: c_ulong = 0;
    for (i = 0; i < pool.nareas; i++) {
    used += pool.areas[i].used;
    }
    return used;
    }
//
// mem_used() - get number of used slots in an allocator
// @mem:	Software IO TLB allocator.
//
// When trace_hiwater and CONFIG_DEBUG_FS is enabled, the result is accurate
// because the total number of used slots is tracked in mem->total_used.
// Otherwise, the result is an approximation, because there is no locking of
// individual areas.
//
// Return: Number of used slots.
//
#[no_mangle]
unsafe extern "C" fn mem_used(mem: *mut io_tlb_mem) -> c_ulong {

    if (track_hiwater_enabled) {
    return atomic_long_read(&mem.total_used);
    }

pub static mut pool: *mut c_void = core::ptr::null_mut();
pub static mut used: c_ulong = 0;
    rcu_read_lock();
    list_for_each_entry_rcu(pool, &mem.pools, node) {
    used += mem_pool_used(pool);
    }
    rcu_read_unlock();
    return used;

    return mem_pool_used(&mem.defpool);

    }
//
// swiotlb_tbl_map_single() - bounce buffer map a single contiguous physical area
// @dev:		Device which maps the buffer.
// @orig_addr:		Original (non-bounced) physical IO buffer address
// @mapping_size:	Requested size of the actual bounce buffer, excluding
// any pre- or post-padding for alignment
// @alloc_align_mask:	Required start and end alignment of the allocated buffer
// @dir:		DMA direction
// @attrs:		Optional DMA attributes for the map operation, updated
// to match the selected SWIOTLB pool
//
// Find and allocate a suitable sequence of IO TLB slots for the request.
// The device's SWIOTLB pool must match the device's current DMA encryption
// requirements. If the device requires decrypted DMA, bouncing is done through
// an unencrypted pool and the mapping is marked shared. If the device can DMA
// to encrypted memory, bouncing is done through an encrypted pool even when the
// original DMA address was unencrypted. Enabling encrypted DMA for a device is
// therefore expected to update its default io_tlb_mem to an encrypted pool, so
// later bounce mappings for both encrypted and decrypted original memory use
// that encrypted pool.
//
// The allocated space starts at an alignment specified by alloc_align_mask,
// and the size of the allocated space is rounded up so that the total amount
// of allocated space is a multiple of (alloc_align_mask + 1). If
// alloc_align_mask is zero, the allocated space may be at any alignment and
// the size is not rounded up.
//
// The returned address is within the allocated space and matches the bits
// of orig_addr that are specified in the DMA min_align_mask for the device. As
// such, this returned address may be offset from the beginning of the allocated
// space. The bounce buffer space starting at the returned address for
// mapping_size bytes is initialized to the contents of the original IO buffer
// area. Any pre-padding (due to an offset) and any post-padding (due to
// rounding-up the size) is not initialized.
//
    phys_addr_t swiotlb_tbl_map_single(device *dev, phys_addr_t orig_addr,
    size_t mapping_size, unsigned int alloc_align_mask,
    enum dma_data_direction dir, unsigned long *attrs)
    {
    let mut mem = dev.dma_io_tlb_mem;
    let mut offset = 0;
pub static mut pool: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut size = 0;
    let mut index = 0;
    let mut tlb_addr;
    let mut pad_slots = 0;
    if (!mem || !mem.nslabs) {
    dev_warn_ratelimited(dev,
    "Can not allocate SWIOTLB buffer earlier and can't now provide you with the DMA bounce buffer");
    return (phys_addr_t)DMA_MAPPING_ERROR;
    }
    if (cc_platform_has(CC_ATTR_MEM_ENCRYPT)) {
    pr_warn_once("Memory encryption is active and system is using DMA bounce buffers\n");
    }
    if (cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT)) {
// swiotlb pool is incorrect for this device
    if (unlikely(mem.cc_shared != force_dma_unencrypted(dev))) {
    return (phys_addr_t)DMA_MAPPING_ERROR;
    }
    } else if (cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT)) {
//
// On hosts with memory encryption, SWIOTLB-backed memory is
// unencrypted. DMA addresses returned for bounce buffers must
// therefore be marked unencrypted, even for devices that can
// address encrypted memory. This also preserves swiotlb=force
// behavior for those devices.
//
    if (unlikely(!mem.cc_shared)) {
    return (phys_addr_t)DMA_MAPPING_ERROR;
    }
    }
// Force attrs to match the kind of memory in the pool
    if (mem.cc_shared) {
// attrs |= DMA_ATTR_CC_SHARED;
    }
    else {
// attrs &= ~DMA_ATTR_CC_SHARED;
    }
//
// The default swiotlb memory pool is allocated with PAGE_SIZE
// alignment. If a mapping is requested with larger alignment,
// the mapping may be unable to use the initial slot(s) in all
// sets of IO_TLB_SEGSIZE slots. In such case, a mapping request
// of or near the maximum mapping size would always fail.
//
    dev_WARN_ONCE(dev, alloc_align_mask > ~PAGE_MASK,
    "Alloc alignment may prevent fulfilling requests with max mapping_size\n");
    offset = swiotlb_align_offset(dev, alloc_align_mask, orig_addr);
    size = ALIGN(mapping_size + offset, alloc_align_mask + 1);
    index = swiotlb_find_slots(dev, orig_addr, size, alloc_align_mask, &pool);
    if (index == -1) {
    if (!(*attrs & DMA_ATTR_NO_WARN)) {
    dev_warn_ratelimited(dev,
    "swiotlb buffer is full (sz: %zd bytes), total %lu (slots), used %lu (slots)\n",
    size, mem.nslabs, mem_used(mem));
    }
    return (phys_addr_t)DMA_MAPPING_ERROR;
    }
//
// If dma_skip_sync was set, reset it on first SWIOTLB buffer
// mapping to always sync SWIOTLB buffers.
//
    dma_reset_need_sync(dev);
//
// Save away the mapping from the original address to the DMA address.
// This is needed when we sync the memory.  Then we sync the buffer if
// needed.
//
    pad_slots = offset >> IO_TLB_SHIFT;
    offset &= (IO_TLB_SIZE - 1);
    index += pad_slots;
    pool.slots[index].pad_slots = pad_slots;
    for (i = 0; i < (nr_slots(size) - pad_slots); i++) {
    pool.slots[index + i].orig_addr = slot_addr(orig_addr, i);
    }
    tlb_addr = slot_addr(pool.start, index) + offset;
//
// When the device is writing memory, i.e. dir == DMA_FROM_DEVICE, copy
// the original buffer to the TLB buffer before initiating DMA in order
// to preserve the original's data if the device does a partial write,
// i.e. if the device doesn't overwrite the entire buffer.  Preserving
// the original data, even if it's garbage, is necessary to match
// hardware behavior.  Use of swiotlb is supposed to be transparent,
// i.e. swiotlb must not corrupt memory by clobbering unwritten bytes.
//
    swiotlb_bounce(dev, tlb_addr, mapping_size, DMA_TO_DEVICE, pool);
    return tlb_addr;
    }
#[no_mangle]
pub unsafe extern "C" fn swiotlb_release_slots(dev: *mut device, tlb_addr: phys_addr_t, mem: *mut io_tlb_pool) {
    let mut flags = 0;
pub static mut offset: c_uint = 0;
    let mut index = 0;
    let mut nslots = 0;
    let mut aindex = 0;
pub static mut area: *mut c_void = core::ptr::null_mut();
    let mut count = 0;
    let mut i = 0;
    index = (tlb_addr - offset - mem.start) >> IO_TLB_SHIFT;
    index -= mem.slots[index].pad_slots;
    nslots = nr_slots(mem.slots[index].alloc_size + offset);
    aindex = index / mem.area_nslabs;
    area = &mem.areas[aindex];
//
// Return the buffer to the free list by setting the corresponding
// entries to indicate the number of contiguous entries available.
// While returning the entries to the free list, we merge the entries
// with slots below and above the pool being returned.
//
    BUG_ON!(aindex >= mem.nareas);
    spin_lock_irqsave(&area.lock, flags);
    if (index + nslots < ALIGN(index + 1, IO_TLB_SEGSIZE)) {
    count = mem.slots[index + nslots].list;
    }
    else {
    count = 0;
    }
//
// Step 1: return the slots to the free list, merging the slots with
// superceeding slots
//
    while (i >= index) {
    mem.slots[i].list = count += 1;
    mem.slots[i].orig_addr = INVALID_PHYS_ADDR;
    mem.slots[i].alloc_size = 0;
    mem.slots[i].pad_slots = 0;
    }
//
// Step 2: merge the returned slots with the preceding slots, if
// available (non zero)
//
    for (i = index - 1;
    io_tlb_offset(i) != IO_TLB_SEGSIZE - 1 && mem.slots[i].list;
    i--) {
    mem.slots[i].list = count += 1;
    }
    area.used -= nslots;
    spin_unlock_irqrestore(&area.lock, flags);
    dec_used(dev.dma_io_tlb_mem, nslots);
    }

//
// swiotlb_del_transient() - delete a transient memory pool
// @dev:	Device which mapped the buffer.
// @tlb_addr:	Physical address within a bounce buffer.
// @pool:       Pointer to the transient memory pool to be checked and deleted.
//
// Check whether the address belongs to a transient SWIOTLB memory pool.
// If yes, then delete the pool.
//
// Return: %true if @tlb_addr belonged to a transient pool that was released.
//
#[no_mangle]
pub unsafe extern "C" fn swiotlb_del_transient(dev: *mut device, tlb_addr: phys_addr_t, pool: *mut io_tlb_pool) -> bool {
    if (!pool.transient) {
    return false;
    }
    dec_used(dev.dma_io_tlb_mem, pool.nslabs);
    swiotlb_del_pool(dev, pool);
    dec_transient_used(dev.dma_io_tlb_mem, pool.nslabs);
    return true;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: swiotlb_del_transient
pub unsafe extern "C" fn swiotlb_del_transient_dup(dev: *mut device, tlb_addr: phys_addr_t, pool: *mut io_tlb_pool) -> bool {
    return false;
    }

//
// tlb_addr is the physical address of the bounce buffer to unmap.
//
#[no_mangle]
pub unsafe extern "C" fn __swiotlb_tbl_unmap_single(dev: *mut device, tlb_addr: phys_addr_t, mapping_size: size_t, dir: dma_data_direction, attrs: c_ulong, pool: *mut io_tlb_pool) {
//
// First, sync the memory before unmapping the entry
//
    if (!(attrs & DMA_ATTR_SKIP_CPU_SYNC) &&
    (dir == DMA_FROM_DEVICE || dir == DMA_BIDIRECTIONAL)) {
    swiotlb_bounce(dev, tlb_addr, mapping_size,
    DMA_FROM_DEVICE, pool);
    }
    if (swiotlb_del_transient(dev, tlb_addr, pool)) {
    return;
    }
    swiotlb_release_slots(dev, tlb_addr, pool);
    }
#[no_mangle]
pub unsafe extern "C" fn __swiotlb_sync_single_for_device(dev: *mut device, tlb_addr: phys_addr_t, size: size_t, dir: dma_data_direction, pool: *mut io_tlb_pool) {
    if (dir == DMA_TO_DEVICE || dir == DMA_BIDIRECTIONAL) {
    swiotlb_bounce(dev, tlb_addr, size, DMA_TO_DEVICE, pool);
    }
    else {
    BUG_ON!(dir != DMA_FROM_DEVICE);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __swiotlb_sync_single_for_cpu(dev: *mut device, tlb_addr: phys_addr_t, size: size_t, dir: dma_data_direction, pool: *mut io_tlb_pool) {
    if (dir == DMA_FROM_DEVICE || dir == DMA_BIDIRECTIONAL) {
    swiotlb_bounce(dev, tlb_addr, size, DMA_FROM_DEVICE, pool);
    }
    else {
    BUG_ON!(dir != DMA_TO_DEVICE);
    }
    }
//
// Create a swiotlb mapping for the buffer at @paddr, and in case of DMAing
// to the device copy the data into it as well.
//
    dma_addr_t swiotlb_map(device *dev, phys_addr_t paddr, size_t size,
    enum dma_data_direction dir, unsigned long attrs)
    {
    let mut swiotlb_addr;
    let mut dma_addr;
    trace_swiotlb_bounced(dev, phys_to_dma(dev, paddr), size);
    swiotlb_addr = swiotlb_tbl_map_single(dev, paddr, size, 0, dir, &attrs);
    if (swiotlb_addr == (phys_addr_t)DMA_MAPPING_ERROR) {
    return DMA_MAPPING_ERROR;
    }
    if (attrs & DMA_ATTR_CC_SHARED) {
    dma_addr = phys_to_dma_unencrypted(dev, swiotlb_addr);
    }
    else {
    dma_addr = phys_to_dma_encrypted(dev, swiotlb_addr);
    }
    if (unlikely(!dma_capable(dev, dma_addr, size, true, attrs))) {
    __swiotlb_tbl_unmap_single(dev, swiotlb_addr, size, dir,
    attrs | DMA_ATTR_SKIP_CPU_SYNC,
    swiotlb_find_pool(dev, swiotlb_addr));
    dev_WARN_ONCE(dev, 1,
    "swiotlb addr %pad+%zu overflow (mask %llx, bus limit %llx).\n",
    &dma_addr, size, *dev.dma_mask, dev.bus_dma_limit);
    return DMA_MAPPING_ERROR;
    }
    if (!dev_is_dma_coherent(dev) && !(attrs & DMA_ATTR_SKIP_CPU_SYNC)) {
    arch_sync_dma_for_device(swiotlb_addr, size, dir);
    arch_sync_dma_flush();
    }
    return dma_addr;
    }
#[no_mangle]
pub unsafe extern "C" fn swiotlb_max_mapping_size(dev: *mut device) -> usize {
pub static mut min_align_mask: c_int = 0;
pub static mut min_align: c_int = 0;
//
// swiotlb_find_slots() skips slots according to
// min align mask. This affects max mapping size.
// Take it into acount here.
//
    if (min_align_mask) {
    min_align = roundup(min_align_mask, IO_TLB_SIZE);
    }
    return ((size_t)IO_TLB_SIZE) * IO_TLB_SEGSIZE - min_align;
    }
//
// is_swiotlb_allocated() - check if the default software IO TLB is initialized
//
#[no_mangle]
pub unsafe extern "C" fn is_swiotlb_allocated() -> bool {
    return io_tlb_default_mem.nslabs;
    }
#[no_mangle]
pub unsafe extern "C" fn is_swiotlb_active(dev: *mut device) -> bool {
    let mut mem = dev.dma_io_tlb_mem;
    return mem && mem.nslabs;
    }
//
// default_swiotlb_base() - get the base address of the default SWIOTLB
//
// Get the lowest physical address used by the default software IO TLB pool.
//
#[no_mangle]
pub unsafe extern "C" fn default_swiotlb_base() -> phys_addr_t {

    io_tlb_default_mem.can_grow = false;

    return io_tlb_default_mem.defpool.start;
    }
//
// default_swiotlb_limit() - get the address limit of the default SWIOTLB
//
// Get the highest physical address used by the default software IO TLB pool.
//
#[no_mangle]
pub unsafe extern "C" fn default_swiotlb_limit() -> phys_addr_t {

    return io_tlb_default_mem.phys_limit;

    return io_tlb_default_mem.defpool.end - 1;

    }

#[no_mangle]
unsafe extern "C" fn mem_transient_used(mem: *mut io_tlb_mem) -> c_ulong {
    return atomic_long_read(&mem.transient_nslabs);
    }
#[no_mangle]
unsafe extern "C" fn io_tlb_transient_used_get(data: *mut c_void, val: *mut u64) -> c_int {
    let mut mem = data;
// val = mem_transient_used(mem);
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(fops_io_tlb_transient_used, io_tlb_transient_used_get,
    core::ptr::null_mut(), "%llu\n");

#[no_mangle]
unsafe extern "C" fn io_tlb_used_get(data: *mut c_void, val: *mut u64) -> c_int {
    let mut mem = data;
// val = mem_used(mem);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn io_tlb_hiwater_get(data: *mut c_void, val: *mut u64) -> c_int {
    let mut mem = data;
// val = atomic_long_read(&mem->used_hiwater);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn io_tlb_hiwater_set(data: *mut c_void, val: u64) -> c_int {
    let mut mem = data;
// Only allow setting to zero
    if (val != 0) {
    return -EINVAL;
    }
    atomic_long_set(&mem.used_hiwater, val);
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(fops_io_tlb_used, io_tlb_used_get, core::ptr::null_mut(), "%llu\n");
pub static mut fops_io_tlb_hiwater: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn swiotlb_create_debugfs_files(mem: *mut io_tlb_mem, dirname: *mut c_char) {
    mem.debugfs = debugfs_create_dir(dirname, io_tlb_default_mem.debugfs);
    if (!mem.nslabs) {
    return;
    }
    debugfs_create_ulong("io_tlb_nslabs", 0400, mem.debugfs, &mem.nslabs);
    debugfs_create_file("io_tlb_used", 0400, mem.debugfs, mem,
    &fops_io_tlb_used);
    debugfs_create_file("io_tlb_used_hiwater", 0600, mem.debugfs, mem,
    &fops_io_tlb_hiwater);

    debugfs_create_file("io_tlb_transient_nslabs", 0400, mem.debugfs,
    mem, &fops_io_tlb_transient_used);

    }
#[no_mangle]
unsafe extern "C" fn swiotlb_create_default_debugfs() -> c_int {
    swiotlb_create_debugfs_files(&io_tlb_default_mem, "swiotlb");
    return 0;
    }
    late_initcall!(swiotlb_create_default_debugfs);

#[no_mangle]
#[no_mangle]
// duplicate fn: swiotlb_create_debugfs_files
pub unsafe extern "C" fn swiotlb_create_debugfs_files_dup(mem: *mut io_tlb_mem, dirname: *mut c_char) {
    }

#[no_mangle]
pub unsafe extern "C" fn swiotlb_alloc(dev: *mut device, size: size_t, attrs: c_ulong) -> *mut c_void {
    let mut mem = dev.dma_io_tlb_mem;
pub static mut pool: *mut c_void = core::ptr::null_mut();
    let mut tlb_addr;
    let mut align = 0;
    let mut index = 0;
    if (!mem) {
    return core::ptr::null_mut();
    }
    if (mem.cc_shared != !!(attrs & __DMA_ATTR_ALLOC_CC_SHARED)) {
    return core::ptr::null_mut();
    }
    align = (1 << (get_order(size) + PAGE_SHIFT)) - 1;
    index = swiotlb_find_slots(dev, 0, size, align, &pool);
    if (index == -1) {
    return core::ptr::null_mut();
    }
    tlb_addr = slot_addr(pool.start, index);
    if (unlikely(!PAGE_ALIGNED(tlb_addr))) {
    dev_WARN_ONCE(dev, 1, "Cannot allocate pages from non page-aligned swiotlb addr 0x%pa.\n",
    &tlb_addr);
    swiotlb_release_slots(dev, tlb_addr, pool);
    return core::ptr::null_mut();
    }
    return pfn_to_page(PFN_DOWN(tlb_addr));
    }
#[no_mangle]
pub unsafe extern "C" fn swiotlb_free(dev: *mut device, page: *mut page, size: usize) -> bool {
pub static mut tlb_addr: phys_addr_t = 0;
pub static mut pool: *mut c_void = core::ptr::null_mut();
    pool = swiotlb_find_pool(dev, tlb_addr);
    if (!pool) {
    return false;
    }
    swiotlb_release_slots(dev, tlb_addr, pool);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn swiotlb_free_from_pool(dev: *mut device, tlb_addr: phys_addr_t, pool: *mut io_tlb_pool) {
    swiotlb_release_slots(dev, tlb_addr, pool);
    }
#[no_mangle]
pub unsafe extern "C" fn rmem_swiotlb_device_init(rmem: *mut reserved_mem, dev: *mut device) -> c_int {
    let mut mem = rmem.priv;
pub static mut nslabs: c_ulong = 0;
// Set Per-device io tlb area to one
pub static mut nareas: c_uint = 1;
    if (PageHighMem(pfn_to_page(PHYS_PFN(rmem.base)))) {
    dev_err(dev, "Restricted DMA pool must be accessible within the linear mapping.");
    return -EINVAL;
    }
//
// Since multiple devices can share the same pool, the private data,
// io_tlb_mem struct, will be initialized by the first device attached
// to it.
//
    if (!mem) {
pub static mut pool: *mut c_void = core::ptr::null_mut();
    mem = kzalloc_obj(*mem);
    if (!mem) {
    return -ENOMEM;
    }
    pool = &mem.defpool;
    pool.slots = kzalloc_objs(*pool.slots, nslabs);
    if (!pool.slots) {
    kfree(mem);
    return -ENOMEM;
    }
    pool.areas = kzalloc_objs(*pool.areas, nareas);
    if (!pool.areas) {
    kfree(pool.slots);
    kfree(mem);
    return -ENOMEM;
    }
//
// if platform supports memory encryption,
// restricted mem pool is shared by default
//
    if (cc_platform_has(CC_ATTR_MEM_ENCRYPT)) {
    let mut ret = 0;
    mem.cc_shared = true;
    ret = set_memory_decrypted((unsigned long)phys_to_virt(rmem.base),
    rmem.size >> PAGE_SHIFT);
    if (ret) {
    dev_err(dev, "Failed to decrypt restricted DMA pool\n");
    kfree(pool.areas);
    kfree(pool.slots);
    kfree(mem);
    return ret;
    }
    } else {
    mem.cc_shared = false;
    }
    swiotlb_init_io_tlb_pool(pool, rmem.base, phys_to_virt(rmem.base),
    nslabs, false, nareas);
    mem.force_bounce = true;
    mem.for_alloc = true;

    spin_lock_init(&mem.lock);
    INIT_LIST_HEAD_RCU(&mem.pools);

    add_mem_pool(mem, pool);
    rmem.priv = mem;
    swiotlb_create_debugfs_files(mem, rmem.name);
    }
    dev.dma_io_tlb_mem = mem;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rmem_swiotlb_device_release(rmem: *mut reserved_mem, dev: *mut device) {
    dev.dma_io_tlb_mem = &io_tlb_default_mem;
    }
    static int __init rmem_swiotlb_setup(unsigned long node, reserved_mem *rmem)
    {
    if (of_get_flat_dt_prop(node, "reusable", core::ptr::null_mut()) ||
    of_get_flat_dt_prop(node, "linux,cma-default", core::ptr::null_mut()) ||
    of_get_flat_dt_prop(node, "linux,dma-default", core::ptr::null_mut()) ||
    of_get_flat_dt_prop(node, "no-map", core::ptr::null_mut())) {
    return -EINVAL;
    }
    pr_info!("Reserved memory: created restricted DMA pool at %pa, size %ld MiB\n",
    &rmem.base, (unsigned long)rmem.size / SZ_1M);
    return 0;
    }
pub static mut reserved_mem_ops: usize = 0;
    RESERVEDMEM_OF_DECLARE(dma, "restricted-dma-pool", &rmem_swiotlb_ops);
//! Automatically rewritten from C to Rust
//! Source: mm/damon/vaddr.c
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
// DAMON Code for Virtual Address Spaces
//

pub const DAMON_MIN_REGION_SZ: c_int = 1;

//
// 't->pid' should be the pointer to the relevant 'struct pid' having reference
// count.  Caller must put the returned task, unless it is NULL.
//
#[no_mangle]
pub unsafe extern "C" fn damon_get_task_struct(t: *mut damon_target) -> *mut c_void {
    return get_pid_task(t.pid, PIDTYPE_PID);
    }
//
// Get the mm_struct of the given target
//
// Caller _must_ put the mm_struct after use, unless it is NULL.
//
// Returns the mm_struct of the target on success, NULL on failure
//
#[no_mangle]
pub unsafe extern "C" fn damon_get_mm(t: *mut damon_target) -> *mut c_void {
pub static mut task: *mut c_void = core::ptr::null_mut();
pub static mut mm: *mut c_void = core::ptr::null_mut();
    task = damon_get_task_struct(t);
    if (!task) {
    return core::ptr::null_mut();
    }
    mm = get_task_mm(task);
    put_task_struct(task);
    return mm;
    }
#[no_mangle]
unsafe extern "C" fn sz_range(r: *mut damon_addr_range) -> c_ulong {
    return r.end - r.start;
    }
//
// Find three regions separated by two biggest unmapped regions
//
// vma		the head vma of the target address space
// regions	an array of three address ranges that results will be saved
//
// This function receives an address space and finds three regions in it which
// separated by the two biggest unmapped regions in the space.  Please refer to
// below comments of '__damon_va_init_regions()' function to know why this is
// necessary.
//
// Returns 0 if success, or negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn __damon_va_three_regions(mm: *mut mm_struct) -> c_int {
pub static mut first_gap: damon_addr_range = 0;
    VMA_ITERATOR(vmi, mm, 0);
    struct vm_area_struct *vma, *prev = core::ptr::null_mut();
    let mut start = 0;
//
// Find the two biggest gaps so that first_gap > second_gap > others.
// If this is too slow, it can be optimised to examine the maple
// tree gaps.
//
    rcu_read_lock();
    for_each_vma(vmi, vma) {
    let mut gap = 0;
    if (!prev) {
    start = vma.vm_start;
// goto;
    }
    gap = vma.vm_start - prev.vm_end;
    if (gap > sz_range(&first_gap)) {
    second_gap = first_gap;
    first_gap.start = prev.vm_end;
    first_gap.end = vma.vm_start;
    } else if (gap > sz_range(&second_gap)) {
    second_gap.start = prev.vm_end;
    second_gap.end = vma.vm_start;
    }
// label;
    prev = vma;
    }
    rcu_read_unlock();
    if (!sz_range(&second_gap) || !sz_range(&first_gap)) {
    return -EINVAL;
    }
// Sort the two biggest gaps by address
    if (first_gap.start > second_gap.start) {
    swap(first_gap, second_gap);
    }
// Store the result
    regions[0].start = ALIGN(start, DAMON_MIN_REGION_SZ);
    regions[0].end = ALIGN(first_gap.start, DAMON_MIN_REGION_SZ);
    regions[1].start = ALIGN(first_gap.end, DAMON_MIN_REGION_SZ);
    regions[1].end = ALIGN(second_gap.start, DAMON_MIN_REGION_SZ);
    regions[2].start = ALIGN(second_gap.end, DAMON_MIN_REGION_SZ);
    regions[2].end = ALIGN(prev.vm_end, DAMON_MIN_REGION_SZ);
    return 0;
    }
//
// Get the three regions in the given target (task)
//
// Returns 0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn damon_va_three_regions(t: *mut damon_target) -> c_int {
pub static mut mm: *mut c_void = core::ptr::null_mut();
    let mut rc = 0;
    mm = damon_get_mm(t);
    if (!mm) {
    return -EINVAL;
    }
    mmap_read_lock(mm);
    rc = __damon_va_three_regions(mm, regions);
    mmap_read_unlock(mm);
    mmput(mm);
    return rc;
    }
//
// Initialize the monitoring target regions for the given target (task)
//
// t	the given target
//
// Because only a number of small portions of the entire address space
// is actually mapped to the memory and accessed, monitoring the unmapped
// regions is wasteful.  That said, because we can deal with small noises,
// tracking every mapping is not strictly required but could even incur a high
// overhead if the mapping frequently changes or the number of mappings is
// high.  The adaptive regions adjustment mechanism will further help to deal
// with the noise by simply identifying the unmapped areas as a region that
// has no access.  Moreover, applying the real mappings that would have many
// unmapped areas inside will make the adaptive mechanism quite complex.  That
// said, too huge unmapped areas inside the monitoring target should be removed
// to not take the time for the adaptive mechanism.
//
// For the reason, we convert the complex mappings to three distinct regions
// that cover every mapped area of the address space.  Also the two gaps
// between the three regions are the two biggest unmapped areas in the given
// address space.  In detail, this function first identifies the start and the
// end of the mappings and the two biggest unmapped areas of the address space.
// Then, it constructs the three regions as below:
//
// [mappings[0]->start, big_two_unmapped_areas[0]->start)
// [big_two_unmapped_areas[0]->end, big_two_unmapped_areas[1]->start)
// [big_two_unmapped_areas[1]->end, mappings[nr_mappings - 1]->end)
//
// As usual memory map of processes is as below, the gap between the heap and
// the uppermost mmap()-ed region, and the gap between the lowermost mmap()-ed
// region and the stack will be two biggest unmapped regions.  Because these
// gaps are exceptionally huge areas in usual address space, excluding these
// two biggest unmapped regions will be sufficient to make a trade-off.
//
// <heap>
// <BIG UNMAPPED REGION 1>
// <uppermost mmap()-ed region>
// (other mmap()-ed regions and small unmapped regions)
// <lowermost mmap()-ed region>
// <BIG UNMAPPED REGION 2>
// <stack>
//
#[no_mangle]
pub unsafe extern "C" fn __damon_va_init_regions(ctx: *mut damon_ctx, t: *mut damon_target) {
pub static mut ti: *mut c_void = core::ptr::null_mut();
    struct damon_addr_range regions[3];
pub static mut tidx: c_int = 0;
    if (damon_va_three_regions(t, regions)) {
    damon_for_each_target(ti, ctx) {
    if (ti == t) {
    break;
    }
    tidx += 1;
    }
    pr_debug!("Failed to get three regions of %dth target\n", tidx);
    return;
    }
    damon_set_regions(t, regions, 3, DAMON_MIN_REGION_SZ);
    }
// Initialize '->regions_list' of every target (task)
#[no_mangle]
unsafe extern "C" fn damon_va_init(ctx: *mut damon_ctx) {
pub static mut t: *mut c_void = core::ptr::null_mut();
    damon_for_each_target(t, ctx) {
// the user may set the target regions as they want
    if (!damon_nr_regions(t)) {
    __damon_va_init_regions(ctx, t);
    }
    }
    }
//
// Update regions for current memory mappings
//
#[no_mangle]
unsafe extern "C" fn damon_va_update(ctx: *mut damon_ctx) {
    struct damon_addr_range three_regions[3];
pub static mut t: *mut c_void = core::ptr::null_mut();
    damon_for_each_target(t, ctx) {
    if (damon_va_three_regions(t, three_regions)) {
    continue;
    }
    damon_set_regions(t, three_regions, 3, DAMON_MIN_REGION_SZ);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn damon_va_walk_page_range(mm: *mut mm_struct, start: c_ulong, end: c_ulong, ops: *mut mm_walk_ops, private: *mut c_void) {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    vma = lock_vma_under_rcu(mm, start);
    if (!vma) {
// goto;
    }
    if (end > vma.vm_end) {
    vma_end_read(vma);
// goto;
    }
    if (!(vma.vm_flags & VM_PFNMAP)) {
    ops.walk_lock = PGWALK_VMA_RDLOCK_VERIFY;
    walk_page_range_vma(vma, start, end, ops, private);
    }
    vma_end_read(vma);
    return;
// label;
    mmap_read_lock(mm);
    ops.walk_lock = PGWALK_RDLOCK;
    walk_page_range(mm, start, end, ops, private);
    mmap_read_unlock(mm);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_mkold_pmd_entry(pmd: *mut pmd_t, addr: c_ulong, next: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut pte: *mut c_void = core::ptr::null_mut();
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    ptl = pmd_trans_huge_lock(pmd, walk.vma);
    if (ptl) {
pub static mut pmde: pmd_t = 0;
    if (pmd_present(pmde)) {
    damon_pmdp_mkold(pmd, walk.vma, addr);
    }
    spin_unlock(ptl);
    return 0;
    }
    pte = pte_offset_map_lock(walk.mm, pmd, addr, &ptl);
    if (!pte) {
    return 0;
    }
    if (!pte_present(ptep_get(pte))) {
// goto;
    }
    damon_ptep_mkold(pte, walk.vma, addr);
// label;
    pte_unmap_unlock(pte, ptl);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn damon_hugetlb_mkold(pte: *mut pte_t, mm: *mut mm_struct, vma: *mut vm_area_struct, addr: c_ulong) {
pub static mut referenced: bool = false;
pub static mut entry: pte_t = 0;
    let mut folio = pfn_folio(pte_pfn(entry));
pub static mut psize: c_ulong = 0;
    folio_get(folio);
    if (pte_young(entry)) {
    referenced = true;
    entry = pte_mkold(entry);
    set_huge_pte_at(mm, addr, pte, entry, psize);
    }
    if (mmu_notifier_clear_young(mm, addr,
    addr + huge_page_size(hstate_vma(vma)))) {
    referenced = true;
    }
    if (referenced) {
    folio_set_young(folio);
    }
    folio_set_idle(folio);
    folio_put(folio);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_mkold_hugetlb_entry(pte: *mut pte_t, hmask: c_ulong, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut h = hstate_vma(walk.vma);
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut entry;
    ptl = huge_pte_lock(h, walk.mm, pte);
    entry = huge_ptep_get(walk.mm, addr, pte);
    if (!pte_present(entry)) {
// goto;
    }
    damon_hugetlb_mkold(pte, walk.mm, walk.vma, addr);
// label;
    spin_unlock(ptl);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn damon_va_mkold(mm: *mut mm_struct, addr: c_ulong) {
pub static mut mm_walk_ops: usize = 0;
    damon_va_walk_page_range(mm, addr, addr + 1, &damon_mkold_ops, core::ptr::null_mut());
    }
//
// Functions for the access checking of the regions
//
#[no_mangle]
pub unsafe extern "C" fn __damon_va_prepare_access_check(mm: *mut mm_struct, r: *mut damon_region, ctx: *mut damon_ctx) {
    r.sampling_addr = damon_rand(ctx, r.ar.start, r.ar.end);
    damon_va_mkold(mm, r.sampling_addr);
    }
#[no_mangle]
unsafe extern "C" fn damon_va_prepare_access_checks(ctx: *mut damon_ctx) {
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut mm: *mut c_void = core::ptr::null_mut();
pub static mut r: *mut c_void = core::ptr::null_mut();
    damon_for_each_target(t, ctx) {
    mm = damon_get_mm(t);
    if (!mm) {
    continue;
    }
    damon_for_each_region(r, t)
    __damon_va_prepare_access_check(mm, r, ctx);
    mmput(mm);
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_young_walk_private {
    pub young: bool,
}

#[no_mangle]
pub unsafe extern "C" fn damon_young_pmd_entry(pmd: *mut pmd_t, addr: c_ulong, next: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut pte: *mut c_void = core::ptr::null_mut();
    let mut ptent;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut priv = walk.private;

    ptl = pmd_trans_huge_lock(pmd, walk.vma);
    if (ptl) {
pub static mut pmde: pmd_t = 0;
    if (!pmd_present(pmde)) {
// goto;
    }
    folio = vm_normal_folio_pmd(walk.vma, addr, pmde);
    if (!folio) {
// goto;
    }
    if (pmd_young(pmde) || !folio_test_idle(folio) ||
    mmu_notifier_test_young(walk.mm,
    addr)) {
    priv.young = true;
    }
// label;
    spin_unlock(ptl);
    return 0;
    }

    pte = pte_offset_map_lock(walk.mm, pmd, addr, &ptl);
    if (!pte) {
    return 0;
    }
    ptent = ptep_get(pte);
    if (!pte_present(ptent)) {
// goto;
    }
    folio = vm_normal_folio(walk.vma, addr, ptent);
    if (!folio) {
// goto;
    }
    if (pte_young(ptent) || !folio_test_idle(folio) ||
    mmu_notifier_test_young(walk.mm, addr)) {
    priv.young = true;
    }
// label;
    pte_unmap_unlock(pte, ptl);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn damon_young_hugetlb_entry(pte: *mut pte_t, hmask: c_ulong, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut priv = walk.private;
    let mut h = hstate_vma(walk.vma);
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut entry;
    ptl = huge_pte_lock(h, walk.mm, pte);
    entry = huge_ptep_get(walk.mm, addr, pte);
    if (!pte_present(entry)) {
// goto;
    }
    folio = pfn_folio(pte_pfn(entry));
    folio_get(folio);
    if (pte_young(entry) || !folio_test_idle(folio) ||
    mmu_notifier_test_young(walk.mm, addr)) {
    priv.young = true;
    }
    folio_put(folio);
// label;
    spin_unlock(ptl);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn damon_va_young(mm: *mut mm_struct, addr: c_ulong) -> bool {
pub static mut damon_young_walk_private: usize = 0;
pub static mut mm_walk_ops: usize = 0;
    damon_va_walk_page_range(mm, addr, addr + 1, &damon_young_ops, &arg);
    return arg.young;
    }
//
// Check whether the region was accessed after the last preparation
//
// mm	'mm_struct' for the given virtual address space
// r	the region to be checked
//
#[no_mangle]
pub unsafe extern "C" fn __damon_va_check_access(mm: *mut mm_struct, r: *mut damon_region) {
    let mut accessed = 0;
    if (!mm) {
    damon_update_region_access_rate(r, false);
    return;
    }
    accessed = damon_va_young(mm, r.sampling_addr);
    damon_update_region_access_rate(r, accessed);
    }
#[no_mangle]
unsafe extern "C" fn damon_va_check_accesses(ctx: *mut damon_ctx) -> c_uint {
pub static mut t: *mut c_void = core::ptr::null_mut();
pub static mut mm: *mut c_void = core::ptr::null_mut();
pub static mut r: *mut c_void = core::ptr::null_mut();
pub static mut max_nr_accesses: c_uint = 0;
    damon_for_each_target(t, ctx) {
    mm = damon_get_mm(t);
    damon_for_each_region(r, t) {
    __damon_va_check_access(mm, r);
    max_nr_accesses = max(r.nr_accesses, max_nr_accesses);
    }
    if (mm) {
    mmput(mm);
    }
    }
    return max_nr_accesses;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_va_filter_young_match(filter: *mut damos_filter, folio: *mut folio, vma: *mut vm_area_struct, addr: c_ulong, ptep: *mut pte_t, pmdp: *mut pmd_t) -> bool {
pub static mut young: bool = false;
    if (ptep) {
    young = pte_young(ptep_get(ptep));
    }

    else if (pmdp) {
    young = pmd_young(pmdp_get(pmdp));
    }
    young = young || !folio_test_idle(folio) ||
    mmu_notifier_test_young(vma.vm_mm, addr);
    if (young && ptep) {
    damon_ptep_mkold(ptep, vma, addr);
    }

    else if (young && pmdp) {
    damon_pmdp_mkold(pmdp, vma, addr);
    }
pub static mut young: return = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_va_filter_out(scheme: *mut damos, folio: *mut folio, vma: *mut vm_area_struct, addr: c_ulong, ptep: *mut pte_t, pmdp: *mut pmd_t) -> bool {
pub static mut filter: *mut c_void = core::ptr::null_mut();
    let mut matched = 0;
    if (scheme.core_filters_allowed) {
    return false;
    }
    damos_for_each_ops_filter(filter, scheme) {
//
// damos_folio_filter_match checks the young filter by doing an
// rmap on the folio to find its page table. However, being the
// vaddr scheme, we have direct access to the page tables, so
// use that instead.
//
    if (filter.type == DAMOS_FILTER_TYPE_YOUNG) {
    matched = damos_va_filter_young_match(filter, folio,
    vma, addr, ptep, pmdp);
    }
    else {
    matched = damos_folio_filter_match(filter, folio);
    }
    if (matched) {
    return !filter.allow;
    }
    }
    return scheme.ops_filters_default_reject;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_va_migrate_private {
    pub migration_lists: *mut list_head,
    pub scheme: *mut damos,
}

//
// Place the given folio in the migration_list corresponding to where the folio
// should be migrated.
//
// The algorithm used here is similar to weighted_interleave_nid()
//
#[no_mangle]
pub unsafe extern "C" fn damos_va_migrate_dests_add(folio: *mut folio, vma: *mut vm_area_struct, addr: c_ulong, dests: *mut damos_migrate_dests, migration_lists: *mut list_head) {
    let mut ilx;
    let mut order = 0;
    let mut target = 0;
pub static mut weight_total: c_uint = 0;
    let mut i = 0;
//
// If dests is empty, there is only one migration list corresponding
// to s->target_nid.
//
    if (!dests.nr_dests) {
    i = 0;
// goto;
    }
    order = folio_order(folio);
    ilx = vma_start_pgoff(vma) >> order;
    ilx += linear_page_delta(vma, addr) >> order;
    for (i = 0; i < dests.nr_dests; i++) {
    weight_total += dests.weight_arr[i];
    }
// If the total weights are somehow 0, don't migrate at all
    if (!weight_total) {
    return;
    }
    target = ilx % weight_total;
    while (i < dests.nr_dests) {
    if (target < dests.weight_arr[i]) {
    break;
    }
    target -= dests.weight_arr[i];
    }
// If the folio is already in the right node, don't do anything
    if (folio_nid(folio) == dests.node_id_arr[i]) {
    return;
    }
// label;
    if (!folio_isolate_lru(folio)) {
    return;
    }
    node_stat_add_folio(folio, NR_ISOLATED_ANON +
    folio_is_file_lru(folio));
    list_add(&folio.lru, &migration_lists[i]);
    }
#[no_mangle]
pub unsafe extern "C" fn damos_va_migrate_pmd_entry(pmd: *mut pmd_t, addr: c_ulong, next: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut priv = walk.private;
    let mut migration_lists = priv.migration_lists;
    let mut s = priv.scheme;
    let mut dests = &s.migrate_dests;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    pte_t *start_pte, *pte, ptent;
    let mut nr = 0;

    ptl = pmd_trans_huge_lock(pmd, walk.vma);
    if (ptl) {
pub static mut pmde: pmd_t = 0;
    if (!pmd_present(pmde)) {
// goto;
    }
    folio = vm_normal_folio_pmd(walk.vma, addr, pmde);
    if (!folio) {
// goto;
    }
    if (damos_va_filter_out(s, folio, walk.vma, addr, core::ptr::null_mut(), pmd)) {
// goto;
    }
    damos_va_migrate_dests_add(folio, walk.vma, addr, dests,
    migration_lists);
// label;
    spin_unlock(ptl);
    return 0;
    }

    start_pte = pte = pte_offset_map_lock(walk.mm, pmd, addr, &ptl);
    if (!pte) {
    return 0;
    }
    while (addr < next) {
    nr = 1;
    ptent = ptep_get(pte);
    if (pte_none(ptent) || !pte_present(ptent)) {
    continue;
    }
    folio = vm_normal_folio(walk.vma, addr, ptent);
    if (!folio) {
    continue;
    }
    if (damos_va_filter_out(s, folio, walk.vma, addr, pte, core::ptr::null_mut())) {
    continue;
    }
    damos_va_migrate_dests_add(folio, walk.vma, addr, dests,
    migration_lists);
    nr = folio_nr_pages(folio);
    }
    pte_unmap_unlock(start_pte, ptl);
    return 0;
    }
//
// Functions for the target validity check and cleanup
//
#[no_mangle]
unsafe extern "C" fn damon_va_target_valid(t: *mut damon_target) -> bool {
pub static mut task: *mut c_void = core::ptr::null_mut();
    task = damon_get_task_struct(t);
    if (task) {
    put_task_struct(task);
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn damon_va_cleanup_target(t: *mut damon_target) {
    put_pid(t.pid);
    }

#[no_mangle]
pub unsafe extern "C" fn damos_madvise(target: *mut damon_target, r: *mut damon_region, behavior: c_int) -> c_ulong {
    return 0;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: damos_madvise
pub unsafe extern "C" fn damos_madvise_dup(target: *mut damon_target, r: *mut damon_region, behavior: c_int) -> c_ulong {
pub static mut mm: *mut c_void = core::ptr::null_mut();
pub static mut start: c_ulong = 0;
pub static mut len: c_ulong = 0;
    let mut applied = 0;
    mm = damon_get_mm(target);
    if (!mm) {
    return 0;
    }
    applied = do_madvise(mm, start, len, behavior) ? 0 : len;
    mmput(mm);
    return applied;
    }

#[no_mangle]
pub unsafe extern "C" fn damos_va_migrate(target: *mut damon_target, r: *mut damon_region, s: *mut damos, sz_filter_passed: *mut c_ulong) -> c_ulong {
pub static mut folio_list: usize = 0;
pub static mut priv: usize = 0;
pub static mut mm: *mut c_void = core::ptr::null_mut();
    let mut nr_dests = 0;
    let mut nid = 0;
    let mut use_target_nid = 0;
pub static mut applied: c_ulong = 0;
    let mut dests = &s.migrate_dests;
pub static mut mm_walk_ops: usize = 0;
    use_target_nid = dests.nr_dests == 0;
    nr_dests = use_target_nid ? 1 : dests.nr_dests;
    priv.scheme = s;
    priv.migration_lists = kmalloc_objs(*priv.migration_lists, nr_dests);
    if (!priv.migration_lists) {
    return 0;
    }
    for (int i = 0; i < nr_dests; i++) {
    INIT_LIST_HEAD(&priv.migration_lists[i]);
    }
    mm = damon_get_mm(target);
    if (!mm) {
// goto;
    }
    damon_va_walk_page_range(mm, r.ar.start, r.ar.end, &walk_ops, &priv);
    mmput(mm);
    while (i < nr_dests) {
    nid = use_target_nid ? s.target_nid : dests.node_id_arr[i];
    applied += damon_migrate_pages(&priv.migration_lists[i], nid);
    cond_resched();
    }
// label;
    kfree(priv.migration_lists);
    return applied * PAGE_SIZE;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_va_stat_private {
    pub scheme: *mut damos,
    pub sz_filter_passed: *mut c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn damos_va_invalid_folio(folio: *mut folio, s: *mut damos) -> bool {
    return !folio || folio == s.last_applied;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_va_stat_pmd_entry(pmd: *mut pmd_t, addr: c_ulong, next: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut priv = walk.private;
    let mut s = priv.scheme;
    let mut sz_filter_passed = priv.sz_filter_passed;
    let mut vma = walk.vma;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    pte_t *start_pte, *pte, ptent;
    let mut nr = 0;

    ptl = pmd_trans_huge_lock(pmd, vma);
    if (ptl) {
pub static mut pmde: pmd_t = 0;
    if (!pmd_present(pmde)) {
// goto;
    }
    folio = vm_normal_folio_pmd(vma, addr, pmde);
    if (damos_va_invalid_folio(folio, s)) {
// goto;
    }
    if (!damos_va_filter_out(s, folio, vma, addr, core::ptr::null_mut(), pmd)) {
// sz_filter_passed += folio_size(folio);
    }
    s.last_applied = folio;
// label;
    spin_unlock(ptl);
    return 0;
    }

    start_pte = pte = pte_offset_map_lock(vma.vm_mm, pmd, addr, &ptl);
    if (!start_pte) {
    return 0;
    }
    while (addr < next) {
    nr = 1;
    ptent = ptep_get(pte);
    if (pte_none(ptent) || !pte_present(ptent)) {
    continue;
    }
    folio = vm_normal_folio(vma, addr, ptent);
    if (damos_va_invalid_folio(folio, s)) {
    continue;
    }
    if (!damos_va_filter_out(s, folio, vma, addr, pte, core::ptr::null_mut())) {
// sz_filter_passed += folio_size(folio);
    }
    nr = folio_nr_pages(folio);
    s.last_applied = folio;
    }
    pte_unmap_unlock(start_pte, ptl);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damos_va_stat(target: *mut damon_target, r: *mut damon_region, s: *mut damos, sz_filter_passed: *mut c_ulong) -> c_ulong {
pub static mut priv: usize = 0;
pub static mut mm: *mut c_void = core::ptr::null_mut();
pub static mut mm_walk_ops: usize = 0;
    priv.scheme = s;
    priv.sz_filter_passed = sz_filter_passed;
    if (!damos_ops_has_filter(s)) {
    return 0;
    }
    mm = damon_get_mm(target);
    if (!mm) {
    return 0;
    }
    damon_va_walk_page_range(mm, r.ar.start, r.ar.end, &walk_ops, &priv);
    mmput(mm);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn damon_va_apply_scheme(ctx: *mut damon_ctx, t: *mut damon_target, r: *mut damon_region, scheme: *mut damos, sz_filter_passed: *mut c_ulong) -> c_ulong {
    let mut madv_action = 0;
    match (scheme.action) {
    DAMOS_WILLNEED => {
    madv_action = MADV_WILLNEED;
    // break;
    }
    DAMOS_COLD => {
    madv_action = MADV_COLD;
    // break;
    }
    DAMOS_PAGEOUT => {
    madv_action = MADV_PAGEOUT;
    // break;
    }
    DAMOS_HUGEPAGE => {
    madv_action = MADV_HUGEPAGE;
    // break;
    }
    DAMOS_NOHUGEPAGE => {
    madv_action = MADV_NOHUGEPAGE;
    // break;
    }
    DAMOS_COLLAPSE => {
    madv_action = MADV_COLLAPSE;
    // break;
    }
    DAMOS_MIGRATE_HOT => {
    }
    DAMOS_MIGRATE_COLD => {
    return damos_va_migrate(t, r, scheme, sz_filter_passed);
    }
    DAMOS_STAT => {
    return damos_va_stat(t, r, scheme, sz_filter_passed);
    }
    _ => {
//
// DAMOS actions that are not yet supported by 'vaddr'.
//
    return 0;
    }
    }
    return damos_madvise(t, r, madv_action);
    }
#[no_mangle]
pub unsafe extern "C" fn damon_va_scheme_score(context: *mut damon_ctx, r: *mut damon_region, scheme: *mut damos) -> c_int {
    match (scheme.action) {
    DAMOS_PAGEOUT => {
    return damon_cold_score(context, r, scheme);
    }
    DAMOS_MIGRATE_HOT => {
    return damon_hot_score(context, r, scheme);
    }
    DAMOS_MIGRATE_COLD => {
    return damon_cold_score(context, r, scheme);
    }
    _ => {
    // break;
    }
    }
    return DAMOS_MAX_SCORE;
    }
#[no_mangle]
unsafe extern "C" fn damon_va_initcall!() -> c_int {
pub static mut damon_operations: usize = 0;
// ops for fixed virtual address ranges
pub static mut ops_fvaddr: damon_operations = 0;
    let mut err = 0;
// Don't set the monitoring target regions for the entire mapping
    ops_fvaddr.id = DAMON_OPS_FVADDR;
    ops_fvaddr.init = core::ptr::null_mut();
    ops_fvaddr.update = core::ptr::null_mut();
    err = damon_register_ops(&ops);
    if (err) {
    return err;
    }
    return damon_register_ops(&ops_fvaddr);
    }
    subsys_initcall!(damon_va_initcall);
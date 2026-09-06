//! Automatically rewritten from C to Rust
//! Source: mm/madvise.c
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
// linux/mm/madvise.c
//
// Copyright (C) 1999  Linus Torvalds
// Copyright (C) 2002  Christoph Hellwig
//

//
// Maximum number of attempts we make to install guard pages before we give up
// and return -ERESTARTNOINTR to have userspace try again.
//
pub const MAX_MADVISE_GUARD_RETRIES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct madvise_walk_private {
    pub tlb: *mut mmu_gather,
    pub pageout: bool,
}

    enum madvise_lock_mode {
    MADVISE_NO_LOCK,
    MADVISE_MMAP_READ_LOCK,
    MADVISE_MMAP_WRITE_LOCK,
    MADVISE_VMA_READ_LOCK,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct madvise_behavior_range {
    pub start: c_ulong,
    pub end: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct madvise_behavior {
    pub mm: *mut mm_struct,
    pub behavior: c_int,
    pub tlb: *mut mmu_gather,
    pub lock_mode: madvise_lock_mode,
    pub anon_name: *mut anon_vma_name,
//
// The range over which the behaviour is currently being applied. If
// traversing multiple VMAs, this is updated for each.
//
    pub range: madvise_behavior_range,
// The VMA and VMA preceding it (if applicable) currently targeted.
    pub prev: *mut vm_area_struct,
    pub vma: *mut vm_area_struct,
    pub lock_dropped: bool,
}

// forward_decl: madvise_walk_vmas;
#[no_mangle]
pub unsafe extern "C" fn anon_vma_name_alloc(name: *mut c_char) -> *mut c_void {
pub static mut anon_name: *mut c_void = core::ptr::null_mut();
    let mut count = 0;
// Add 1 for NUL terminator at the end of the anon_name->name
    count = strlen(name) + 1;
    anon_name = kmalloc_flex(*anon_name, name, count);
    if (anon_name) {
    kref_init(&anon_name.kref);
    memcpy(anon_name.name, name, count);
    }
    return anon_name;
    }
#[no_mangle]
pub unsafe extern "C" fn anon_vma_name_free(kref: *mut kref) {
    let mut anon_name = container_of!(kref, anon_vma_name, kref);
    kfree(anon_name);
    }
#[no_mangle]
pub unsafe extern "C" fn anon_vma_name(vma: *mut vm_area_struct) -> *mut c_void {
    vma_assert_stabilised(vma);
    return vma.anon_name;
    }
// mmap_lock should be write-locked
#[no_mangle]
pub unsafe extern "C" fn replace_anon_vma_name(vma: *mut vm_area_struct, anon_name: *mut anon_vma_name) -> c_int {
    let mut orig_name = anon_vma_name(vma);
    if (!anon_name) {
    vma.anon_name = core::ptr::null_mut();
    anon_vma_name_put(orig_name);
    return 0;
    }
    if (anon_vma_name_eq(orig_name, anon_name)) {
    return 0;
    }
    vma.anon_name = anon_vma_name_reuse(anon_name);
    anon_vma_name_put(orig_name);
    return 0;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: replace_anon_vma_name
pub unsafe extern "C" fn replace_anon_vma_name_dup(vma: *mut vm_area_struct, anon_name: *mut anon_vma_name) -> c_int {
    if (anon_name) {
    return -EINVAL;
    }
    return 0;
    }

//
// Update the vm_flags or anon_name on region of a vma, splitting it or merging
// it as necessary. Must be called with mmap_lock held for writing.
//
#[no_mangle]
pub unsafe extern "C" fn madvise_update_vma(new_flags: vm_flags_t, madv_behavior: *mut madvise_behavior) -> c_int {
    let mut vma = madv_behavior.vma;
pub static mut new_vma_flags: vma_flags_t = 0;
    let mut range = &madv_behavior.range;
    let mut anon_name = madv_behavior.anon_name;
pub static mut set_new_anon_name: bool = false;
    VMA_ITERATOR(vmi, madv_behavior.mm, range.start);
    if (vma_flags_same_mask(&vma.flags, new_vma_flags) &&
    (!set_new_anon_name ||
    anon_vma_name_eq(anon_vma_name(vma), anon_name))) {
    return 0;
    }
    if (set_new_anon_name) {
    vma = vma_modify_name(&vmi, madv_behavior.prev, vma,
    range.start, range.end, anon_name);
    }
    else {
    vma = vma_modify_flags(&vmi, madv_behavior.prev, vma,
    range.start, range.end, &new_vma_flags);
    }
    if (IS_ERR(vma)) {
    return PTR_ERR(vma);
    }
    madv_behavior.vma = vma;
// vm_flags is protected by the mmap_lock held in write mode.
    vma_start_write(vma);
    vma.flags = new_vma_flags;
//
// If the vma become good for khugepaged to scan,
// register it here without waiting a page fault that
// may not happen any time soon.
//
    if (vma_flags_test(&new_vma_flags, VMA_HUGEPAGE_BIT)) {
    khugepaged_enter_vma(vma, vma_flags_to_legacy(new_vma_flags));
    }
    if (set_new_anon_name) {
    return replace_anon_vma_name(vma, anon_name);
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn swapin_walk_pmd_entry(pmd: *mut pmd_t, start: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut vma = walk.private;
pub static mut ctx: swap_io_ctx = 0;
    let mut ptep = core::ptr::null_mut();
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut addr = 0;
    while (addr < end) {
    let mut pte;
    let mut entry;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    if (!ptep++) {
    ptep = pte_offset_map_lock(vma.vm_mm, pmd, addr, &ptl);
    if (!ptep) {
    break;
    }
    }
    pte = ptep_get(ptep);
    entry = softleaf_from_pte(pte);
    if (unlikely(!softleaf_is_swap(entry))) {
    continue;
    }
    pte_unmap_unlock(ptep, ptl);
    ptep = core::ptr::null_mut();
    folio = read_swap_cache_async(&ctx, entry, GFP_HIGHUSER_MOVABLE,
    vma, addr);
    if (folio) {
    folio_put(folio);
    }
    }
    if (ptep) {
    pte_unmap_unlock(ptep, ptl);
    }
    swap_read_submit(&ctx);
    cond_resched();
    return 0;
    }
pub static mut mm_walk_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn shmem_swapin_range(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong, mapping: *mut address_space) {
    XA_STATE(xas, &mapping.i_pages, linear_page_index(vma, start));
pub static mut end_index: pgoff_t = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut ctx: swap_io_ctx = 0;
    rcu_read_lock();
    xas_for_each(&xas, folio, end_index) {
    let mut addr = 0;
    let mut entry;
    if (!xa_is_value(folio)) {
    continue;
    }
    entry = radix_to_swp_entry(folio);
// There might be swapin error entries in shmem mapping.
    if (!softleaf_is_swap(entry)) {
    continue;
    }
    addr = vma.vm_start +
    ((xas.xa_index - vma_start_pgoff(vma)) << PAGE_SHIFT);
    xas_pause(&xas);
    rcu_read_unlock();
    folio = read_swap_cache_async(&ctx, entry,
    mapping_gfp_mask(mapping), vma, addr);
    if (folio) {
    folio_put(folio);
    }
    rcu_read_lock();
    }
    rcu_read_unlock();
    swap_read_submit(&ctx);
    }

#[no_mangle]
unsafe extern "C" fn mark_mmap_lock_dropped(madv_behavior: *mut madvise_behavior) {
    VM_WARN_ON_ONCE(madv_behavior.lock_mode == MADVISE_VMA_READ_LOCK);
    madv_behavior.lock_dropped = true;
    }
//
// Schedule all required I/O operations.  Do not wait for completion.
//
#[no_mangle]
unsafe extern "C" fn madvise_willneed(madv_behavior: *mut madvise_behavior) -> c_long {
    let mut vma = madv_behavior.vma;
    let mut mm = madv_behavior.mm;
    let mut file = vma.vm_file;
pub static mut start: c_ulong = 0;
pub static mut end: c_ulong = 0;
    let mut offset = 0;

    if (!file) {
    walk_page_range_vma(vma, start, end, &swapin_walk_ops, vma);
    lru_add_drain(); /* Push any new pages onto the LRU now */
    return 0;
    }
    if (shmem_mapping(file.f_mapping)) {
    shmem_swapin_range(vma, start, end, file.f_mapping);
    lru_add_drain(); /* Push any new pages onto the LRU now */
    return 0;
    }

    if (!file) {
    return -EBADF;
    }

    if (IS_DAX(file_inode(file))) {
// no bad return value, but ignore advice
    return 0;
    }
//
// Filesystem's fadvise may need to take various locks.  We need to
// explicitly grab a reference because the vma (and hence the
// vma's reference to the file) can go away as soon as we drop
// mmap_lock.
//
    mark_mmap_lock_dropped(madv_behavior);
    get_file(file);
    offset = (loff_t)(start - vma.vm_start)
    + ((loff_t)vma_start_pgoff(vma) << PAGE_SHIFT);
    mmap_read_unlock(mm);
    vfs_fadvise(file, offset, end - start, POSIX_FADV_WILLNEED);
    fput(file);
    mmap_read_lock(mm);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn can_do_file_pageout(vma: *mut vm_area_struct) -> bool {
    if (!vma.vm_file) {
    return false;
    }
//
// paging out pagecache only for non-anonymous mappings that correspond
// to the files the calling process could (if tried) open for writing;
// otherwise we'd be including shared non-exclusive mappings, which
// opens a side channel.
//
    return file_owner_or_capable(vma.vm_file) ||
    file_permission(vma.vm_file, MAY_WRITE) == 0;
    }
#[no_mangle]
pub unsafe extern "C" fn madvise_folio_pte_batch(addr: c_ulong, end: c_ulong, folio: *mut folio, ptep: *mut pte_t, ptentp: *mut pte_t) -> c_int {
pub static mut max_nr: c_int = 0;
    return folio_pte_batch_flags(folio, core::ptr::null_mut(), ptep, ptentp, max_nr,
    FPB_MERGE_YOUNG_DIRTY);
    }
#[no_mangle]
pub unsafe extern "C" fn madvise_cold_or_pageout_pte_range(pmd: *mut pmd_t, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut private = walk.private;
    let mut tlb = private.tlb;
pub static mut pageout: bool = false;
    let mut mm = tlb.mm;
    let mut vma = walk.vma;
    pte_t *start_pte, *pte, ptent;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut folio = core::ptr::null_mut();
pub static mut folio_list: usize = 0;
    let mut pageout_anon_only_filter = 0;
pub static mut batch_count: c_uint = 0;
    let mut nr = 0;
    if (fatal_signal_pending(current)) {
    return -EINTR;
    }
    pageout_anon_only_filter = pageout && !vma_is_anonymous(vma) &&
    !can_do_file_pageout(vma);

    if (pmd_trans_huge(*pmd)) {
    let mut orig_pmd;
pub static mut next: c_ulong = 0;
    tlb_change_page_size(tlb, HPAGE_PMD_SIZE);
    ptl = pmd_trans_huge_lock(pmd, vma);
    if (!ptl) {
    return 0;
    }
    orig_pmd = *pmd;
    if (is_huge_zero_pmd(orig_pmd)) {
// goto;
    }
    if (unlikely(!pmd_present(orig_pmd))) {
    VM_WARN_ON_ONCE(!pmd_is_migration_entry(orig_pmd) &&
    !pmd_is_device_private_entry(orig_pmd));
// goto;
    }
    folio = pmd_folio(orig_pmd);
// Do not interfere with other mappings of this folio
    if (folio_maybe_mapped_shared(folio)) {
// goto;
    }
    if (pageout_anon_only_filter && !folio_test_anon(folio)) {
// goto;
    }
    if (next - addr != HPAGE_PMD_SIZE) {
    let mut err = 0;
    folio_get(folio);
    spin_unlock(ptl);
    folio_lock(folio);
    err = split_folio(folio);
    folio_unlock(folio);
    folio_put(folio);
    if (!err) {
// goto;
    }
    return 0;
    }
    if (!pageout && pmd_young(orig_pmd)) {
    pmdp_invalidate(vma, addr, pmd);
    orig_pmd = pmd_mkold(orig_pmd);
    set_pmd_at(mm, addr, pmd, orig_pmd);
    tlb_remove_pmd_tlb_entry(tlb, pmd, addr);
    }
    folio_clear_referenced(folio);
    folio_test_clear_young(folio);
    if (folio_test_active(folio)) {
    folio_set_workingset(folio);
    }
    if (pageout) {
    if (folio_isolate_lru(folio)) {
    if (folio_test_unevictable(folio)) {
    folio_putback_lru(folio);
    }
    else {
    list_add(&folio.lru, &folio_list);
    }
    }
    } else {
    folio_deactivate(folio);
    }
// label;
    spin_unlock(ptl);
    if (pageout) {
    reclaim_pages(&folio_list);
    }
    return 0;
    }
// label;
    tlb_change_page_size(tlb, PAGE_SIZE);
// label;
    start_pte = pte = pte_offset_map_lock(vma.vm_mm, pmd, addr, &ptl);
    if (!start_pte) {
    return 0;
    }
    flush_tlb_batched_pending(mm);
    lazy_mmu_mode_enable();
    while (addr < end) {
    nr = 1;
    ptent = ptep_get(pte);
    if (++batch_count == SWAP_CLUSTER_MAX) {
    batch_count = 0;
    if (need_resched()) {
    lazy_mmu_mode_disable();
    pte_unmap_unlock(start_pte, ptl);
    cond_resched();
// goto;
    }
    }
    if (pte_none(ptent)) {
    continue;
    }
    if (!pte_present(ptent)) {
    continue;
    }
    folio = vm_normal_folio(vma, addr, ptent);
    if (!folio || folio_is_zone_device(folio)) {
    continue;
    }
//
// If we encounter a large folio, only split it if it is not
// fully mapped within the range we are operating on. Otherwise
// leave it as is so that it can be swapped out whole. If we
// fail to split a folio, leave it in place and advance to the
// next pte in the range.
//
    if (folio_test_large(folio)) {
    nr = madvise_folio_pte_batch(addr, end, folio, pte, &ptent);
    if (nr < folio_nr_pages(folio)) {
    let mut err = 0;
    if (folio_maybe_mapped_shared(folio)) {
    continue;
    }
    if (pageout_anon_only_filter && !folio_test_anon(folio)) {
    continue;
    }
    if (!folio_trylock(folio)) {
    continue;
    }
    folio_get(folio);
    lazy_mmu_mode_disable();
    pte_unmap_unlock(start_pte, ptl);
    start_pte = core::ptr::null_mut();
    err = split_folio(folio);
    folio_unlock(folio);
    folio_put(folio);
    start_pte = pte =
    pte_offset_map_lock(mm, pmd, addr, &ptl);
    if (!start_pte) {
    break;
    }
    flush_tlb_batched_pending(mm);
    lazy_mmu_mode_enable();
    if (!err) {
    nr = 0;
    }
    continue;
    }
    }
//
// Do not interfere with other mappings of this folio and
// non-LRU folio. If we have a large folio at this point, we
// know it is fully mapped so if its mapcount is the same as its
// number of pages, it must be exclusive.
//
    if (!folio_test_lru(folio) ||
    folio_mapcount(folio) != folio_nr_pages(folio)) {
    continue;
    }
    if (pageout_anon_only_filter && !folio_test_anon(folio)) {
    continue;
    }
    if (!pageout && pte_young(ptent)) {
    clear_young_dirty_ptes(vma, addr, pte, nr,
    CYDP_CLEAR_YOUNG);
    tlb_remove_tlb_entries(tlb, pte, nr, addr);
    }
//
// We are deactivating a folio for accelerating reclaiming.
// VM couldn't reclaim the folio unless we clear PG_young.
// As a side effect, it makes confuse idle-page tracking
// because they will miss recent referenced history.
//
    folio_clear_referenced(folio);
    folio_test_clear_young(folio);
    if (folio_test_active(folio)) {
    folio_set_workingset(folio);
    }
    if (pageout) {
    if (folio_isolate_lru(folio)) {
    if (folio_test_unevictable(folio)) {
    folio_putback_lru(folio);
    }
    else {
    list_add(&folio.lru, &folio_list);
    }
    }
    } else {
    folio_deactivate(folio);
    }
    }
    if (start_pte) {
    lazy_mmu_mode_disable();
    pte_unmap_unlock(start_pte, ptl);
    }
    if (pageout) {
    reclaim_pages(&folio_list);
    }
    cond_resched();
    return 0;
    }
pub static mut mm_walk_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn madvise_cold_page_range(tlb: *mut mmu_gather, madv_behavior: *mut madvise_behavior) {
    let mut vma = madv_behavior.vma;
    let mut range = &madv_behavior.range;
pub static mut madvise_walk_private: usize = 0;
    tlb_start_vma(tlb, vma);
    walk_page_range_vma(vma, range.start, range.end, &cold_walk_ops,
    &walk_private);
    tlb_end_vma(tlb, vma);
    }
#[no_mangle]
pub unsafe extern "C" fn can_madv_lru_vma(vma: *mut vm_area_struct) -> bool {
    return !(vma.vm_flags & (VM_LOCKED|VM_PFNMAP|VM_HUGETLB));
    }
#[no_mangle]
unsafe extern "C" fn madvise_cold(madv_behavior: *mut madvise_behavior) -> c_long {
    let mut vma = madv_behavior.vma;
pub static mut tlb: usize = 0;
    if (!can_madv_lru_vma(vma)) {
    return -EINVAL;
    }
    lru_add_drain();
    tlb_gather_mmu(&tlb, madv_behavior.mm);
    madvise_cold_page_range(&tlb, madv_behavior);
    tlb_finish_mmu(&tlb);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn madvise_pageout_page_range(tlb: *mut mmu_gather, vma: *mut vm_area_struct, range: *mut madvise_behavior_range) {
pub static mut madvise_walk_private: usize = 0;
    tlb_start_vma(tlb, vma);
    walk_page_range_vma(vma, range.start, range.end, &cold_walk_ops,
    &walk_private);
    tlb_end_vma(tlb, vma);
    }
#[no_mangle]
unsafe extern "C" fn madvise_pageout(madv_behavior: *mut madvise_behavior) -> c_long {
pub static mut tlb: usize = 0;
    let mut vma = madv_behavior.vma;
    if (!can_madv_lru_vma(vma)) {
    return -EINVAL;
    }
//
// If the VMA belongs to a private file mapping, there can be private
// dirty pages which can be paged out if even this process is neither
// owner nor write capable of the file. We allow private file mappings
// further to pageout dirty anon pages.
//
    if (!vma_is_anonymous(vma) && (!can_do_file_pageout(vma) &&
    (vma.vm_flags & VM_MAYSHARE))) {
    return 0;
    }
    lru_add_drain();
    tlb_gather_mmu(&tlb, madv_behavior.mm);
    madvise_pageout_page_range(&tlb, vma, &madv_behavior.range);
    tlb_finish_mmu(&tlb);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn madvise_free_pte_range(pmd: *mut pmd_t, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut cydp_flags: cydp_t = 0;
    let mut tlb = walk.private;
    let mut mm = tlb.mm;
    let mut vma = walk.vma;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    pte_t *start_pte, *pte, ptent;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut nr_swap: c_int = 0;
    let mut next = 0;
    let mut nr = 0;
    let mut max_nr = 0;
    next = pmd_addr_end(addr, end);
    if (pmd_trans_huge(*pmd)) {
    if (madvise_free_huge_pmd(tlb, vma, pmd, addr, next))
    return 0;
    }
    tlb_change_page_size(tlb, PAGE_SIZE);
    start_pte = pte = pte_offset_map_lock(mm, pmd, addr, &ptl);
    if (!start_pte) {
    return 0;
    }
    flush_tlb_batched_pending(mm);
    lazy_mmu_mode_enable();
    while (addr != end) {
    nr = 1;
    ptent = ptep_get(pte);
    if (pte_none(ptent)) {
    continue;
    }
//
// If the pte has swp_entry, just clear page table to
// prevent swap-in which is more expensive rather than
// (page allocation + zeroing).
//
    if (!pte_present(ptent)) {
pub static mut entry: softleaf_t = 0;
    if (softleaf_is_swap(entry)) {
    max_nr = (end - addr) / PAGE_SIZE;
    nr = swap_pte_batch(pte, max_nr, ptent);
    nr_swap -= nr;
    swap_put_entries_direct(entry, nr);
    clear_nonpresent_ptes(mm, addr, pte, nr);
    } else if (softleaf_is_hwpoison(entry) ||
    softleaf_is_poison_marker(entry)) {
    pte_clear(mm, addr, pte);
    }
    continue;
    }
    folio = vm_normal_folio(vma, addr, ptent);
    if (!folio || folio_is_zone_device(folio)) {
    continue;
    }
//
// If we encounter a large folio, only split it if it is not
// fully mapped within the range we are operating on. Otherwise
// leave it as is so that it can be marked as lazyfree. If we
// fail to split a folio, leave it in place and advance to the
// next pte in the range.
//
    if (folio_test_large(folio)) {
    nr = madvise_folio_pte_batch(addr, end, folio, pte, &ptent);
    if (nr < folio_nr_pages(folio)) {
    let mut err = 0;
    if (folio_maybe_mapped_shared(folio)) {
    continue;
    }
    if (!folio_trylock(folio)) {
    continue;
    }
    folio_get(folio);
    lazy_mmu_mode_disable();
    pte_unmap_unlock(start_pte, ptl);
    start_pte = core::ptr::null_mut();
    err = split_folio(folio);
    folio_unlock(folio);
    folio_put(folio);
    pte = pte_offset_map_lock(mm, pmd, addr, &ptl);
    start_pte = pte;
    if (!start_pte) {
    break;
    }
    flush_tlb_batched_pending(mm);
    lazy_mmu_mode_enable();
    if (!err) {
    nr = 0;
    }
    continue;
    }
    }
    if (folio_test_swapcache(folio) || folio_test_dirty(folio)) {
    if (!folio_trylock(folio)) {
    continue;
    }
//
// If we have a large folio at this point, we know it is
// fully mapped so if its mapcount is the same as its
// number of pages, it must be exclusive.
//
    if (folio_mapcount(folio) != folio_nr_pages(folio)) {
    folio_unlock(folio);
    continue;
    }
    if (folio_test_swapcache(folio) &&
    !folio_free_swap(folio)) {
    folio_unlock(folio);
    continue;
    }
    folio_clear_dirty(folio);
    folio_unlock(folio);
    }
    if (pte_young(ptent) || pte_dirty(ptent)) {
    clear_young_dirty_ptes(vma, addr, pte, nr, cydp_flags);
    tlb_remove_tlb_entries(tlb, pte, nr, addr);
    }
    folio_mark_lazyfree(folio);
    }
    if (nr_swap) {
    add_mm_counter(mm, MM_SWAPENTS, nr_swap);
    }
    if (start_pte) {
    lazy_mmu_mode_disable();
    pte_unmap_unlock(start_pte, ptl);
    }
    cond_resched();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn get_walk_lock(mode: madvise_lock_mode) -> enum page_walk_lock {
    match (mode) {
    MADVISE_VMA_READ_LOCK => {
    return PGWALK_VMA_RDLOCK_VERIFY;
    }
    MADVISE_MMAP_READ_LOCK => {
    return PGWALK_RDLOCK;
    }
    _ => {
// Other modes don't require fixing up the walk_lock
    WARN_ON_ONCE!(1);
    return PGWALK_RDLOCK;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn madvise_free_single_vma(madv_behavior: *mut madvise_behavior) -> c_int {
    let mut mm = madv_behavior.mm;
    let mut vma = madv_behavior.vma;
pub static mut mmu_notifier_range: usize = 0;
    let mut tlb = madv_behavior.tlb;
pub static mut mm_walk_ops: usize = 0;
// MADV_FREE works for only anon vma at the moment
    if (!vma_is_anonymous(vma)) {
    return -EINVAL;
    }
    mmu_notifier_range_init(&range, MMU_NOTIFY_CLEAR, 0, mm,
    range.start, range.end);
    lru_add_drain();
    update_hiwater_rss(mm);
    mmu_notifier_invalidate_range_start(&range);
    tlb_start_vma(tlb, vma);
    walk_ops.walk_lock = get_walk_lock(madv_behavior.lock_mode);
    walk_page_range_vma(vma, range.start, range.end,
    &walk_ops, tlb);
    tlb_end_vma(tlb, vma);
    mmu_notifier_invalidate_range_end(&range);
    return 0;
    }
//
// Application no longer needs these pages.  If the pages are dirty,
// it's OK to just throw them away.  The app will be more careful about
// data it wants to keep.  Be sure to free swap resources too.  The
// zap_vma_range call sets things up for shrink_active_list to actually
// free these pages later if no one else has touched them in the meantime,
// although we could add these pages to a global reuse list for
// shrink_active_list to pick up before reclaiming other pages.
//
// NB: This interface discards data rather than pushes it out to swap,
// as some implementations do.  This has performance implications for
// applications like large transactional databases which want to discard
// pages in anonymous maps after committing to backing store the data
// that was kept in them.  There is no reason to write this data out to
// the swap area if the application is discarding it.
//
// An interface that causes the system to free clean pages and flush
// dirty pages is already available as msync(MS_INVALIDATE).
//
#[no_mangle]
unsafe extern "C" fn madvise_dontneed_single_vma(madv_behavior: *mut madvise_behavior) -> c_long {
    let mut range = &madv_behavior.range;
pub static mut zap_details: usize = 0;
    zap_vma_range_batched(madv_behavior.tlb, madv_behavior.vma,
    range.start, range.end - range.start, &details);
    return 0;
    }
    static
#[no_mangle]
pub unsafe extern "C" fn madvise_dontneed_free_valid_vma(madv_behavior: *mut madvise_behavior) -> bool {
    let mut vma = madv_behavior.vma;
pub static mut behavior: c_int = 0;
    let mut range = &madv_behavior.range;
    if (!is_vm_hugetlb_page(vma)) {
pub static mut forbidden: c_uint = 0;
    if (behavior != MADV_DONTNEED_LOCKED) {
    forbidden |= VM_LOCKED;
    }
    return !(vma.vm_flags & forbidden);
    }
    if (behavior != MADV_DONTNEED && behavior != MADV_DONTNEED_LOCKED) {
    return false;
    }
    if (range.start & ~huge_page_mask(hstate_vma(vma))) {
    return false;
    }
//
// Madvise callers expect the length to be rounded up to PAGE_SIZE
// boundaries, and may be unaware that this VMA uses huge pages.
// Avoid unexpected data loss by rounding down the number of
// huge pages freed.
//
    range.end = ALIGN_DOWN(range.end, huge_page_size(hstate_vma(vma)));
    return true;
    }
#[no_mangle]
unsafe extern "C" fn madvise_dontneed_free(madv_behavior: *mut madvise_behavior) -> c_long {
    let mut mm = madv_behavior.mm;
    let mut range = &madv_behavior.range;
pub static mut behavior: c_int = 0;
    if (!madvise_dontneed_free_valid_vma(madv_behavior)) {
    return -EINVAL;
    }
    if (range.start == range.end) {
    return 0;
    }
    if (!userfaultfd_remove(madv_behavior.vma, range.start, range.end)) {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    mark_mmap_lock_dropped(madv_behavior);
    mmap_read_lock(mm);
    madv_behavior.vma = vma = vma_lookup(mm, range.start);
    if (!vma) {
    return -ENOMEM;
    }
//
// Potential end adjustment for hugetlb vma is OK as
// the check below keeps end within vma.
//
    if (!madvise_dontneed_free_valid_vma(madv_behavior)) {
    return -EINVAL;
    }
    if (range.end > vma.vm_end) {
//
// Don't fail if end > vma->vm_end. If the old
// vma was split while the mmap_lock was
// released the effect of the concurrent
// operation may not cause madvise() to
// have an undefined result. There may be an
// adjacent next vma that we'll walk
// next. userfaultfd_remove() will generate an
// UFFD_EVENT_REMOVE repetition on the
// end-vma->vm_end range, but the manager can
// handle a repetition fine.
//
    range.end = vma.vm_end;
    }
//
// If the memory region between start and end was
// originally backed by 4kB pages and then remapped to
// be backed by hugepages while mmap_lock was dropped,
// the adjustment for hugetlb vma above may have rounded
// end down to the start address.
//
    if (range.start == range.end) {
    return 0;
    }
    VM_WARN_ON(range.start > range.end);
    }
    if (behavior == MADV_DONTNEED || behavior == MADV_DONTNEED_LOCKED) {
    return madvise_dontneed_single_vma(madv_behavior);
    }

    else if (behavior == MADV_FREE) {
    return madvise_free_single_vma(madv_behavior);
    }
    else {
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn madvise_populate(madv_behavior: *mut madvise_behavior) -> c_long {
    let mut mm = madv_behavior.mm;
pub static mut write: bool = false;
pub static mut locked: c_int = 1;
pub static mut start: c_ulong = 0;
pub static mut end: c_ulong = 0;
    let mut pages = 0;
    while (start < end) {
// Populate (prefault) page tables readable/writable.
    pages = faultin_page_range(mm, start, end, write, &locked);
    if (!locked) {
    mmap_read_lock(mm);
    locked = 1;
    }
    if (pages < 0) {
    match (pages) {
    -EINTR => {
    return -EINTR;
    }
    -EINVAL => {
    return -EINVAL;
    }
    -EHWPOISON => {
    return -EHWPOISON;
    }
    -EFAULT => {
    return -EFAULT;
    }
    _ => {
    pr_warn_once("%s: unhandled return value: %ld\n",
    __func__, pages);
    fallthrough;
    }
    -ENOMEM => {
    return -ENOMEM;
    }
    }
    }
    start += pages * PAGE_SIZE;
    }
    return 0;
    }
//
// Application wants to free up the pages and associated backing store.
// This is effectively punching a hole into the middle of a file.
//
#[no_mangle]
unsafe extern "C" fn madvise_remove(madv_behavior: *mut madvise_behavior) -> c_long {
    let mut offset = 0;
    let mut error = 0;
pub static mut f: *mut c_void = core::ptr::null_mut();
    let mut mm = madv_behavior.mm;
    let mut vma = madv_behavior.vma;
pub static mut start: c_ulong = 0;
pub static mut end: c_ulong = 0;
    mark_mmap_lock_dropped(madv_behavior);
    if (vma.vm_flags & VM_LOCKED) {
    return -EINVAL;
    }
    f = vma.vm_file;
    if (!f || !f.f_mapping || !f.f_mapping.host) {
    return -EINVAL;
    }
    if (!vma_is_shared_maywrite(vma)) {
    return -EACCES;
    }
    offset = (loff_t)(start - vma.vm_start)
    + ((loff_t)vma_start_pgoff(vma) << PAGE_SHIFT);
//
// Filesystem's fallocate may need to take i_rwsem.  We need to
// explicitly grab a reference because the vma (and hence the
// vma's reference to the file) can go away as soon as we drop
// mmap_lock.
//
    get_file(f);
    if (userfaultfd_remove(vma, start, end)) {
// mmap_lock was not released by userfaultfd_remove()
    mmap_read_unlock(mm);
    }
    error = vfs_fallocate(f,
    FALLOC_FL_PUNCH_HOLE | FALLOC_FL_KEEP_SIZE,
    offset, end - start);
    fput(f);
    mmap_read_lock(mm);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn is_valid_guard_vma(vma: *mut vm_area_struct, allow_locked: bool) -> bool {
pub static mut disallowed: vm_flags_t = 0;
//
// A user could lock after setting a guard range but that's fine, as
// they'd not be able to fault in. The issue arises when we try to zap
// existing locked VMAs. We don't want to do that.
//
    if (!allow_locked) {
    disallowed |= VM_LOCKED;
    }
    return !(vma.vm_flags & disallowed);
    }
#[no_mangle]
unsafe extern "C" fn is_guard_pte_marker(ptent: pte_t) -> bool {
pub static mut entry: softleaf_t = 0;
    return softleaf_is_guard_marker(entry);
    }
#[no_mangle]
pub unsafe extern "C" fn guard_install_pud_entry(pud: *mut pud_t, addr: c_ulong, next: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut pudval: pud_t = 0;
// If huge return >0 so we abort the operation + zap.
    return pud_trans_huge(pudval);
    }
#[no_mangle]
pub unsafe extern "C" fn guard_install_pmd_entry(pmd: *mut pmd_t, addr: c_ulong, next: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut pmdval: pmd_t = 0;
// If huge return >0 so we abort the operation + zap.
    return pmd_trans_huge(pmdval);
    }
#[no_mangle]
pub unsafe extern "C" fn guard_install_pte_entry(pte: *mut pte_t, addr: c_ulong, next: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut pteval: pte_t = 0;
    let mut nr_pages = walk.private;
// If there is already a guard page marker, we have nothing to do.
    if (is_guard_pte_marker(pteval)) {
    (*nr_pages)++;
    return 0;
    }
// If populated return >0 so we abort the operation + zap.
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn guard_install_set_pte(addr: c_ulong, next: c_ulong, ptep: *mut pte_t, walk: *mut mm_walk) -> c_int {
    let mut nr_pages = walk.private;
// Simply install a PTE marker, this causes segfault on access.
// ptep = make_pte_marker(PTE_MARKER_GUARD);
    (*nr_pages)++;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn madvise_guard_install(madv_behavior: *mut madvise_behavior) -> c_long {
    let mut vma = madv_behavior.vma;
    let mut range = &madv_behavior.range;
pub static mut mm_walk_ops: usize = 0;
    let mut err = 0;
    let mut i = 0;
    if (!is_valid_guard_vma(vma, /* allow_locked = */false)) {
    return -EINVAL;
    }
//
// Set atomically under read lock. All pertinent readers will need to
// acquire an mmap/VMA write lock to read it. All remaining readers may
// or may not see the flag set, but we don't care.
//
    vma_set_atomic_flag(vma, VMA_MAYBE_GUARD_BIT);
//
// If anonymous and we are establishing page tables the VMA ought to
// have an anon_vma associated with it.
//
// We will hold an mmap read lock if this is necessary, this is checked
// as part of the VMA lock logic.
//
    if (vma_is_anonymous(vma)) {
    VM_WARN_ON_ONCE(!vma.anon_vma &&
    madv_behavior.lock_mode != MADVISE_MMAP_READ_LOCK);
    err = anon_vma_prepare(vma);
    if (err) {
    return err;
    }
    }
//
// Optimistically try to install the guard marker pages first. If any
// non-guard pages or THP huge pages are encountered, give up and zap
// the range before trying again.
//
// We try a few times before giving up and releasing back to userland to
// loop around, releasing locks in the process to avoid contention.
//
// This would only happen due to races with e.g. page faults or
// khugepaged.
//
// In most cases we should simply install the guard markers immediately
// with no zap or looping.
//
    while (i < MAX_MADVISE_GUARD_RETRIES) {
pub static mut nr_pages: c_ulong = 0;
// Returns < 0 on error, == 0 if success, > 0 if zap needed.
    if (madv_behavior.lock_mode == MADVISE_VMA_READ_LOCK) {
    err = walk_page_range_vma_unsafe(madv_behavior.vma,
    range.start, range.end, &walk_ops,
    &nr_pages);
    }
    else {
    err = walk_page_range_mm_unsafe(vma.vm_mm, range.start,
    range.end, &walk_ops, &nr_pages);
    }
    if (err < 0) {
    return err;
    }
    if (err == 0) {
    let mut nr_expected_pages = PHYS_PFN(range.end - range.start);
    VM_WARN_ON(nr_pages != nr_expected_pages);
    return 0;
    }
//
// OK some of the range have non-guard pages mapped, zap
// them. This leaves existing guard pages in place.
//
    zap_vma_range(vma, range.start, range.end - range.start);
    }
//
// We were unable to install the guard pages, return to userspace and
// immediately retry, relieving lock contention.
//
    return restart_syscall();
    }
#[no_mangle]
pub unsafe extern "C" fn guard_remove_pud_entry(pud: *mut pud_t, addr: c_ulong, next: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut pudval: pud_t = 0;
// If huge, cannot have guard pages present, so no-op - skip.
    if (pud_trans_huge(pudval)) {
    walk.action = ACTION_CONTINUE;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn guard_remove_pmd_entry(pmd: *mut pmd_t, addr: c_ulong, next: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut pmdval: pmd_t = 0;
// If huge, cannot have guard pages present, so no-op - skip.
    if (pmd_trans_huge(pmdval)) {
    walk.action = ACTION_CONTINUE;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn guard_remove_pte_entry(pte: *mut pte_t, addr: c_ulong, next: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut ptent: pte_t = 0;
    if (is_guard_pte_marker(ptent)) {
// Simply clear the PTE marker.
    pte_clear(walk.mm, addr, pte);
    update_mmu_cache(walk.vma, addr, pte);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn madvise_guard_remove(madv_behavior: *mut madvise_behavior) -> c_long {
    let mut vma = madv_behavior.vma;
    let mut range = &madv_behavior.range;
pub static mut mm_walk_ops: usize = 0;
//
// We're ok with removing guards in mlock()'d ranges, as this is a
// non-destructive action.
//
    if (!is_valid_guard_vma(vma, /* allow_locked = */true)) {
    return -EINVAL;
    }
    return walk_page_range_vma(vma, range.start, range.end,
    &wallk_ops, core::ptr::null_mut());
    }

// Does the madvise operation result in discarding of mapped data?
#[no_mangle]
unsafe extern "C" fn is_discard(behavior: c_int) -> bool {
    match (behavior) {
    MADV_FREE => {
    }
    MADV_DONTNEED => {
    }
    MADV_DONTNEED_LOCKED => {
    }
    MADV_REMOVE => {
    }
    MADV_DONTFORK => {
    }
    MADV_WIPEONFORK => {
    }
    MADV_GUARD_INSTALL => {
    return true;
    }
    }
    return false;
    }
//
// We are restricted from madvise()'ing mseal()'d VMAs only in very particular
// circumstances - discarding of data from read-only anonymous SEALED mappings.
//
// This is because users cannot trivally discard data from these VMAs, and may
// only do so via an appropriate madvise() call.
//
#[no_mangle]
unsafe extern "C" fn can_madvise_modify(madv_behavior: *mut madvise_behavior) -> bool {
    let mut vma = madv_behavior.vma;
// If the VMA isn't sealed we're good.
    if (!vma_is_sealed(vma)) {
    return true;
    }
// For a sealed VMA, we only care about discard operations.
    if (!is_discard(madv_behavior.behavior)) {
    return true;
    }
//
// We explicitly permit all file-backed mappings, whether MAP_SHARED or
// MAP_PRIVATE.
//
// The latter causes some complications. Because now, one can mmap()
// read/write a MAP_PRIVATE mapping, write to it, then mprotect()
// read-only, mseal() and a discard will be permitted.
//
// However, in order to avoid issues with potential use of madvise(...,
// MADV_DONTNEED) of mseal()'d .text mappings we, for the time being,
// permit this.
//
    if (!vma_is_anonymous(vma)) {
    return true;
    }
// If the user could write to the mapping anyway, then this is fine.
    if ((vma.vm_flags & VM_WRITE) &&
    arch_vma_access_permitted(vma, /* write= */ true,
execute= */ false, /* foreign= */ false)) {
    return true;
    }
// Otherwise, we are not permitted to perform this operation.
    return false;
    }

#[no_mangle]
unsafe extern "C" fn can_madvise_modify(madv_behavior: *mut madvise_behavior) -> bool {
    return true;
    }

//
// Apply an madvise behavior to a region of a vma.  madvise_update_vma
// will handle splitting a vm area into separate areas, each area with its own
// behavior.
//
#[no_mangle]
unsafe extern "C" fn madvise_vma_behavior(madv_behavior: *mut madvise_behavior) -> c_int {
pub static mut behavior: c_int = 0;
    let mut vma = madv_behavior.vma;
pub static mut new_flags: vm_flags_t = 0;
    let mut range = &madv_behavior.range;
    let mut error = 0;
    if (unlikely(!can_madvise_modify(madv_behavior))) {
    return -EPERM;
    }
    match (behavior) {
    MADV_REMOVE => {
    return madvise_remove(madv_behavior);
    }
    MADV_WILLNEED => {
    return madvise_willneed(madv_behavior);
    }
    MADV_COLD => {
    return madvise_cold(madv_behavior);
    }
    MADV_PAGEOUT => {
    return madvise_pageout(madv_behavior);
    }
    MADV_FREE => {
    }
    MADV_DONTNEED => {
    }
    MADV_DONTNEED_LOCKED => {
    return madvise_dontneed_free(madv_behavior);
    }
    MADV_COLLAPSE => {
    return madvise_collapse(vma, range.start, range.end,
    &madv_behavior.lock_dropped);
    }
    MADV_GUARD_INSTALL => {
    return madvise_guard_install(madv_behavior);
    }
    MADV_GUARD_REMOVE => {
    return madvise_guard_remove(madv_behavior);
// The below behaviours update VMAs via madvise_update_vma().
    }
    MADV_NORMAL => {
    new_flags = new_flags & ~VM_RAND_READ & ~VM_SEQ_READ;
    // break;
    }
    MADV_SEQUENTIAL => {
    new_flags = (new_flags & ~VM_RAND_READ) | VM_SEQ_READ;
    // break;
    }
    MADV_RANDOM => {
    new_flags = (new_flags & ~VM_SEQ_READ) | VM_RAND_READ;
    // break;
    }
    MADV_DONTFORK => {
    new_flags |= VM_DONTCOPY;
    // break;
    }
    MADV_DOFORK => {
    if (new_flags & VM_SPECIAL) {
    return -EINVAL;
    }
    new_flags &= ~VM_DONTCOPY;
    // break;
    }
    MADV_WIPEONFORK => {
// MADV_WIPEONFORK is only supported on anonymous memory.
    if (vma.vm_file || new_flags & VM_SHARED) {
    return -EINVAL;
    }
    new_flags |= VM_WIPEONFORK;
    // break;
    }
    MADV_KEEPONFORK => {
    if (new_flags & VM_DROPPABLE) {
    return -EINVAL;
    }
    new_flags &= ~VM_WIPEONFORK;
    // break;
    }
    MADV_DONTDUMP => {
    new_flags |= VM_DONTDUMP;
    // break;
    }
    MADV_DODUMP => {
    if ((!is_vm_hugetlb_page(vma) && (new_flags & VM_SPECIAL)) ||
    (new_flags & VM_DROPPABLE)) {
    return -EINVAL;
    }
    new_flags &= ~VM_DONTDUMP;
    // break;
    }
    MADV_MERGEABLE => {
    }
    MADV_UNMERGEABLE => {
    error = ksm_madvise(vma, range.start, range.end,
    behavior, &new_flags);
    if (error) {
// goto;
    }
    // break;
    }
    MADV_HUGEPAGE => {
    }
    MADV_NOHUGEPAGE => {
    error = hugepage_madvise(vma, &new_flags, behavior);
    if (error) {
// goto;
    }
    // break;
    }
    __MADV_SET_ANON_VMA_NAME => {
// Only anonymous mappings can be named
    if (vma.vm_file && !vma_is_anon_shmem(vma)) {
    return -EBADF;
    }
    // break;
    }
    }
// This is a write operation.
    VM_WARN_ON_ONCE(madv_behavior.lock_mode != MADVISE_MMAP_WRITE_LOCK);
    error = madvise_update_vma(new_flags, madv_behavior);
// label;
//
// madvise() returns EAGAIN if kernel resources, such as
// slab, are temporarily unavailable.
//
    if (error == -ENOMEM) {
    error = -EAGAIN;
    }
    return error;
    }

//
// Error injection support for memory error handling.
//
#[no_mangle]
unsafe extern "C" fn madvise_inject_error(madv_behavior: *mut madvise_behavior) -> c_int {
    let mut size = 0;
pub static mut start: c_ulong = 0;
pub static mut end: c_ulong = 0;
    if (!capable(CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    while (start < end) {
    let mut pfn = 0;
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    ret = get_user_pages_fast(start, 1, 0, &page);
    if (ret != 1) {
    return ret;
    }
    pfn = page_to_pfn(page);
//
// When soft offlining hugepages, after migrating the page
// we dissolve it, therefore in the second loop "page" will
// no longer be a compound page.
//
    size = page_size(compound_head(page));
    if (madv_behavior.behavior == MADV_SOFT_OFFLINE) {
    pr_info!("Soft offlining pfn %#lx at process virtual address %#lx\n",
    pfn, start);
    ret = soft_offline_page(pfn, MF_COUNT_INCREASED);
    } else {
    pr_info!("Injecting memory failure for pfn %#lx at process virtual address %#lx\n",
    pfn, start);
    ret = memory_failure(pfn, MF_ACTION_REQUIRED | MF_COUNT_INCREASED | MF_SW_SIMULATED);
    if (ret == -EOPNOTSUPP) {
    ret = 0;
    }
    }
    if (ret) {
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn is_memory_failure(madv_behavior: *mut madvise_behavior) -> bool {
    match (madv_behavior.behavior) {
    MADV_HWPOISON => {
    }
    MADV_SOFT_OFFLINE => {
    return true;
    }
    _ => {
    return false;
    }
    }
    }

#[no_mangle]
unsafe extern "C" fn madvise_inject_error(madv_behavior: *mut madvise_behavior) -> c_int {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn is_memory_failure(madv_behavior: *mut madvise_behavior) -> bool {
    return false;
    }

#[no_mangle]
pub unsafe extern "C" fn madvise_behavior_valid(behavior: c_int) -> bool {
    match (behavior) {
    MADV_DOFORK => {
    }
    MADV_DONTFORK => {
    }
    MADV_NORMAL => {
    }
    MADV_SEQUENTIAL => {
    }
    MADV_RANDOM => {
    }
    MADV_REMOVE => {
    }
    MADV_WILLNEED => {
    }
    MADV_DONTNEED => {
    }
    MADV_DONTNEED_LOCKED => {
    }
    MADV_FREE => {
    }
    MADV_COLD => {
    }
    MADV_PAGEOUT => {
    }
    MADV_POPULATE_READ => {
    }
    MADV_POPULATE_WRITE => {

    }
    MADV_MERGEABLE => {
    }
    MADV_UNMERGEABLE => {

    }
    MADV_HUGEPAGE => {
    }
    MADV_NOHUGEPAGE => {
    }
    MADV_COLLAPSE => {

    }
    MADV_DONTDUMP => {
    }
    MADV_DODUMP => {
    }
    MADV_WIPEONFORK => {
    }
    MADV_KEEPONFORK => {
    }
    MADV_GUARD_INSTALL => {
    }
    MADV_GUARD_REMOVE => {

    }
    MADV_SOFT_OFFLINE => {
    }
    MADV_HWPOISON => {

    return true;
    }
    _ => {
    return false;
    }
    }
    }
// Can we invoke process_madvise() on a remote mm for the specified behavior?
#[no_mangle]
unsafe extern "C" fn process_madvise_remote_valid(behavior: c_int) -> bool {
    match (behavior) {
    MADV_COLD => {
    }
    MADV_PAGEOUT => {
    }
    MADV_WILLNEED => {
    }
    MADV_COLLAPSE => {
    return true;
    }
    _ => {
    return false;
    }
    }
    }
// Does this operation invoke anon_vma_prepare()?
#[no_mangle]
unsafe extern "C" fn prepares_anon_vma(behavior: c_int) -> bool {
    match (behavior) {
    MADV_GUARD_INSTALL => {
    return true;
    }
    _ => {
    return false;
    }
    }
    }
//
// We have acquired a VMA read lock, is the VMA valid to be madvise'd under VMA
// read lock only now we have a VMA to examine?
//
#[no_mangle]
pub unsafe extern "C" fn is_vma_lock_sufficient(vma: *mut vm_area_struct, madv_behavior: *mut madvise_behavior) -> bool {
// Must span only a single VMA.
    if (madv_behavior.range.end > vma.vm_end) {
    return false;
    }
// Remote processes unsupported.
    if (current.mm != vma.vm_mm) {
    return false;
    }
// Userfaultfd unsupported.
    if (userfaultfd_armed(vma)) {
    return false;
    }
//
// anon_vma_prepare() explicitly requires an mmap lock for
// serialisation, so we cannot use a VMA lock in this case.
//
// Note we might race with anon_vma being set, however this makes this
// check overly paranoid which is safe.
//
    if (vma_is_anonymous(vma) &&
    prepares_anon_vma(madv_behavior.behavior) && !vma.anon_vma) {
    return false;
    }
    return true;
    }
//
// Try to acquire a VMA read lock if possible.
//
// We only support this lock over a single VMA, which the input range must
// span either partially or fully.
//
// This function always returns with an appropriate lock held. If a VMA read
// lock could be acquired, we return true and set madv_behavior state
// accordingly.
//
// If a VMA read lock could not be acquired, we return false and expect caller to
// fallback to mmap lock behaviour.
//
#[no_mangle]
unsafe extern "C" fn try_vma_read_lock(madv_behavior: *mut madvise_behavior) -> bool {
    let mut mm = madv_behavior.mm;
pub static mut vma: *mut c_void = core::ptr::null_mut();
    vma = lock_vma_under_rcu(mm, madv_behavior.range.start);
    if (!vma) {
// goto;
    }
    if (!is_vma_lock_sufficient(vma, madv_behavior)) {
    vma_end_read(vma);
// goto;
    }
    madv_behavior.vma = vma;
    return true;
// label;
    mmap_read_lock(mm);
    madv_behavior.lock_mode = MADVISE_MMAP_READ_LOCK;
    return false;
    }
//
// Walk the vmas in range [start,end), and call the madvise_vma_behavior
// function on each one.  The function will get start and end parameters that
// cover the overlap between the current vma and the original range.  Any
// unmapped regions in the original range will result in this function returning
// -ENOMEM while still calling the madvise_vma_behavior function on all of the
// existing vmas in the range.  Must be called with the mmap_lock held for
// reading or writing.
//
    static
#[no_mangle]
pub unsafe extern "C" fn madvise_walk_vmas(madv_behavior: *mut madvise_behavior) -> c_int {
    let mut mm = madv_behavior.mm;
    let mut range = &madv_behavior.range;
// range is updated to span each VMA, so store end of entire range.
pub static mut last_end: c_ulong = 0;
pub static mut unmapped_error: c_int = 0;
    let mut error = 0;
    let mut prev = core::ptr::null_mut();
    let mut vma = core::ptr::null_mut();
//
// If VMA read lock is supported, apply madvise to a single VMA
// tentatively, avoiding walking VMAs.
//
    if (madv_behavior.lock_mode == MADVISE_VMA_READ_LOCK &&
    try_vma_read_lock(madv_behavior)) {
    error = madvise_vma_behavior(madv_behavior);
    vma_end_read(madv_behavior.vma);
    return error;
    }
    vma = find_vma_prev(mm, range.start, &prev);
    if (vma && range.start > vma.vm_start) {
    prev = vma;
    }
    for (;;) {
// Still start < end.
    if (!vma) {
    return -ENOMEM;
    }
// Here start < (last_end|vma->vm_end).
    if (range.start < vma.vm_start) {
//
// This indicates a gap between VMAs in the input
// range. This does not cause the operation to abort,
// rather we simply return -ENOMEM to indicate that this
// has happened, but carry on.
//
    unmapped_error = -ENOMEM;
    range.start = vma.vm_start;
    if (range.start >= last_end) {
    break;
    }
    }
// Here vma->vm_start <= range->start < (last_end|vma->vm_end)
    range.end = min(vma.vm_end, last_end);
// Here vma->vm_start <= range->start < range->end <= (last_end|vma->vm_end).
    madv_behavior.prev = prev;
    madv_behavior.vma = vma;
    error = madvise_vma_behavior(madv_behavior);
    if (error) {
    return error;
    }
    if (madv_behavior.lock_dropped) {
// We dropped the mmap lock, we can't ref the VMA.
    prev = core::ptr::null_mut();
    vma = core::ptr::null_mut();
    madv_behavior.lock_dropped = false;
    } else {
    vma = madv_behavior.vma;
    prev = vma;
    }
    if (vma && range.end < vma.vm_end) {
    range.end = vma.vm_end;
    }
    if (range.end >= last_end) {
    break;
    }
    vma = find_vma(mm, vma ? vma.vm_end : range.end);
    range.start = range.end;
    }
    return unmapped_error;
    }
//
// Any behaviour which results in changes to the vma->vm_flags needs to
// take mmap_lock for writing. Others, which simply traverse vmas, need
// to only take it for reading.
//
#[no_mangle]
unsafe extern "C" fn get_lock_mode(madv_behavior: *mut madvise_behavior) -> enum madvise_lock_mode {
    if (is_memory_failure(madv_behavior)) {
    return MADVISE_NO_LOCK;
    }
    match (madv_behavior.behavior) {
    MADV_REMOVE => {
    }
    MADV_WILLNEED => {
    }
    MADV_COLD => {
    }
    MADV_PAGEOUT => {
    }
    MADV_POPULATE_READ => {
    }
    MADV_POPULATE_WRITE => {
    }
    MADV_COLLAPSE => {
    return MADVISE_MMAP_READ_LOCK;
    }
    MADV_GUARD_INSTALL => {
    }
    MADV_GUARD_REMOVE => {
    }
    MADV_DONTNEED => {
    }
    MADV_DONTNEED_LOCKED => {
    }
    MADV_FREE => {
    return MADVISE_VMA_READ_LOCK;
    }
    _ => {
    return MADVISE_MMAP_WRITE_LOCK;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn madvise_lock(madv_behavior: *mut madvise_behavior) -> c_int {
    let mut mm = madv_behavior.mm;
pub static mut lock_mode: madvise_lock_mode = 0;
    match (lock_mode) {
    MADVISE_NO_LOCK => {
    // break;
    }
    MADVISE_MMAP_WRITE_LOCK => {
    if (mmap_write_lock_killable(mm)) {
    return -EINTR;
    }
    // break;
    }
    MADVISE_MMAP_READ_LOCK => {
    mmap_read_lock(mm);
    // break;
    }
    MADVISE_VMA_READ_LOCK => {
// We will acquire the lock per-VMA in madvise_walk_vmas().
    // break;
    }
    }
    madv_behavior.lock_mode = lock_mode;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn madvise_unlock(madv_behavior: *mut madvise_behavior) {
    let mut mm = madv_behavior.mm;
    match (madv_behavior.lock_mode) {
    MADVISE_NO_LOCK => {
    return;
    }
    MADVISE_MMAP_WRITE_LOCK => {
    mmap_write_unlock(mm);
    // break;
    }
    MADVISE_MMAP_READ_LOCK => {
    mmap_read_unlock(mm);
    // break;
    }
    MADVISE_VMA_READ_LOCK => {
// We will drop the lock per-VMA in madvise_walk_vmas().
    // break;
    }
    }
    madv_behavior.lock_mode = MADVISE_NO_LOCK;
    }
#[no_mangle]
unsafe extern "C" fn madvise_batch_tlb_flush(behavior: c_int) -> bool {
    match (behavior) {
    MADV_DONTNEED => {
    }
    MADV_DONTNEED_LOCKED => {
    }
    MADV_FREE => {
    return true;
    }
    _ => {
    return false;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn madvise_init_tlb(madv_behavior: *mut madvise_behavior) {
    if (madvise_batch_tlb_flush(madv_behavior.behavior)) {
    tlb_gather_mmu(madv_behavior.tlb, madv_behavior.mm);
    }
    }
#[no_mangle]
unsafe extern "C" fn madvise_finish_tlb(madv_behavior: *mut madvise_behavior) {
    if (madvise_batch_tlb_flush(madv_behavior.behavior)) {
    tlb_finish_mmu(madv_behavior.tlb);
    }
    }
//
// check_input_range() - Check if the requested range is valid.
// @start:	Start address of madvise-requested address range.
// @len_in:	Length of madvise-requested address range.
//
// Returns: 0 if the input range is valid, otherwise an error code.
//
#[no_mangle]
unsafe extern "C" fn check_input_range(start: c_ulong, len_in: usize) -> c_int {
    let mut len = 0;
    if (!PAGE_ALIGNED(start)) {
    return -EINVAL;
    }
    len = PAGE_ALIGN(len_in);
// Check to see whether len was rounded up from small -ve to zero
    if (len_in && !len) {
    return -EINVAL;
    }
    if (start + len < start) {
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn is_madvise_populate(madv_behavior: *mut madvise_behavior) -> bool {
    match (madv_behavior.behavior) {
    MADV_POPULATE_READ => {
    }
    MADV_POPULATE_WRITE => {
    return true;
    }
    _ => {
    return false;
    }
    }
    }
//
// untagged_addr_remote() assumes mmap_lock is already held. On
// architectures like x86 and RISC-V, tagging is tricky because each
// mm may have a different tagging mask. However, we might only hold
// the per-VMA lock (currently only local processes are supported),
// so untagged_addr is used to avoid the mmap_lock assertion for
// local processes.
//
#[no_mangle]
pub unsafe extern "C" fn get_untagged_addr(mm: *mut mm_struct, start: c_ulong) -> c_ulong {
    return current.mm == mm ? untagged_addr(start) :
    untagged_addr_remote(mm, start);
    }
#[no_mangle]
pub unsafe extern "C" fn madvise_do_behavior(start: c_ulong, len_in: size_t, madv_behavior: *mut madvise_behavior) -> c_int {
pub static mut plug: usize = 0;
    let mut error = 0;
    let mut range = &madv_behavior.range;
    if (is_memory_failure(madv_behavior)) {
    range.start = start;
    range.end = start + len_in;
    return madvise_inject_error(madv_behavior);
    }
    range.start = get_untagged_addr(madv_behavior.mm, start);
    range.end = range.start + PAGE_ALIGN(len_in);
    blk_start_plug(&plug);
    if (is_madvise_populate(madv_behavior)) {
    error = madvise_populate(madv_behavior);
    }
    else {
    error = madvise_walk_vmas(madv_behavior);
    }
    blk_finish_plug(&plug);
    return error;
    }
//
// The madvise(2) system call.
//
// Applications can use madvise() to advise the kernel how it should
// handle paging I/O in this VM area.  The idea is to help the kernel
// use appropriate read-ahead and caching techniques.  The information
// provided is advisory only, and can be safely disregarded by the
// kernel without affecting the correct operation of the application.
//
// behavior values:
// MADV_NORMAL - the default behavior is to read clusters.  This
// results in some read-ahead and read-behind.
// MADV_RANDOM - the system should read the minimum amount of data
// on any access, since it is unlikely that the appli-
// cation will need more than what it asks for.
// MADV_SEQUENTIAL - pages in the given range will probably be accessed
// once, so they can be aggressively read ahead, and
// can be freed soon after they are accessed.
// MADV_WILLNEED - the application is notifying the system to read
// some pages ahead.
// MADV_DONTNEED - the application is finished with the given range,
// so the kernel can free resources associated with it.
// MADV_FREE - the application marks pages in the given range as lazy free,
// where actual purges are postponed until memory pressure happens.
// MADV_REMOVE - the application wants to free up the given range of
// pages and associated backing store.
// MADV_DONTFORK - omit this area from child's address space when forking:
// typically, to avoid COWing pages pinned by get_user_pages().
// MADV_DOFORK - cancel MADV_DONTFORK: no longer omit this area when forking.
// MADV_WIPEONFORK - present the child process with zero-filled memory in this
// range after a fork.
// MADV_KEEPONFORK - undo the effect of MADV_WIPEONFORK
// MADV_HWPOISON - trigger memory error handler as if the given memory range
// were corrupted by unrecoverable hardware memory failure.
// MADV_SOFT_OFFLINE - try to soft-offline the given range of memory.
// MADV_MERGEABLE - the application recommends that KSM try to merge pages in
// this area with pages of identical content from other such areas.
// MADV_UNMERGEABLE- cancel MADV_MERGEABLE: no longer merge pages with others.
// MADV_HUGEPAGE - the application wants to back the given range by transparent
// huge pages in the future. Existing pages might be coalesced and
// new pages might be allocated as THP.
// MADV_NOHUGEPAGE - mark the given range as not worth being backed by
// transparent huge pages so the existing pages will not be
// coalesced into THP and new pages will not be allocated as THP.
// MADV_COLLAPSE - synchronously coalesce pages into new THP.
// MADV_DONTDUMP - the application wants to prevent pages in the given range
// from being included in its core dump.
// MADV_DODUMP - cancel MADV_DONTDUMP: no longer exclude from core dump.
// MADV_COLD - the application is not expected to use this memory soon,
// deactivate pages in this range so that they can be reclaimed
// easily if memory pressure happens.
// MADV_PAGEOUT - the application is not expected to use this memory soon,
// page out the pages in this range immediately.
// MADV_POPULATE_READ - populate (prefault) page tables readable by
// triggering read faults if required
// MADV_POPULATE_WRITE - populate (prefault) page tables writable by
// triggering write faults if required
//
// return values:
// zero    - success
// -EINVAL - start + len < 0, start is not page-aligned,
// "behavior" is not a valid value, or application
// is attempting to release locked or shared pages,
// or the specified address range includes file, Huge TLB,
// MAP_SHARED or VMPFNMAP range.
// -ENOMEM - addresses in the specified range are not currently
// mapped, or are outside the AS of the process.
// -EIO    - an I/O error occurred while paging in data.
// -EBADF  - map exists, but area maps something that isn't a file.
// -EAGAIN - a kernel resource was temporarily unavailable.
// -EPERM  - memory is sealed.
//
#[no_mangle]
pub unsafe extern "C" fn do_madvise(mm: *mut mm_struct, start: c_ulong, len_in: usize, behavior: c_int) -> c_int {
    let mut error = 0;
pub static mut tlb: usize = 0;
pub static mut madvise_behavior: usize = 0;
    if (!madvise_behavior_valid(behavior)) {
    return -EINVAL;
    }
    error = check_input_range(start, len_in);
    if (error || !len_in) {
    return error;
    }
    error = madvise_lock(&madv_behavior);
    if (error) {
    return error;
    }
    madvise_init_tlb(&madv_behavior);
    error = madvise_do_behavior(start, len_in, &madv_behavior);
    madvise_finish_tlb(&madv_behavior);
    madvise_unlock(&madv_behavior);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_madvise(start: usize, len_in: usize, behavior: usize) -> c_long {
    return do_madvise(current.mm, start, len_in, behavior);
    }
// Perform an madvise operation over a vector of addresses and lengths.
#[no_mangle]
pub unsafe extern "C" fn vector_madvise(mm: *mut mm_struct, iter: *mut iov_iter, behavior: c_int) -> ssize_t {
pub static mut ret: isize = 0;
    let mut total_len = 0;
pub static mut tlb: usize = 0;
pub static mut madvise_behavior: usize = 0;
    total_len = iov_iter_count(iter);
    ret = madvise_lock(&madv_behavior);
    if (ret) {
    return ret;
    }
    madvise_init_tlb(&madv_behavior);
    while (iov_iter_count(iter)) {
pub static mut start: c_ulong = 0;
pub static mut len_in: usize = 0;
    let mut error = 0;
    error = check_input_range(start, len_in);
    if (error || !len_in) {
    ret = error;
    }
    else {
    ret = madvise_do_behavior(start, len_in, &madv_behavior);
    }
//
// An madvise operation is attempting to restart the syscall,
// but we cannot proceed as it would not be correct to repeat
// the operation in aggregate, and would be surprising to the
// user.
//
// We drop and reacquire locks so it is safe to just loop and
// try again. We check for fatal signals in case we need exit
// early anyway.
//
    if (ret == -ERESTARTNOINTR) {
    if (fatal_signal_pending(current)) {
    ret = -EINTR;
    break;
    }
// Drop and reacquire lock to unwind race.
    madvise_finish_tlb(&madv_behavior);
    madvise_unlock(&madv_behavior);
    ret = madvise_lock(&madv_behavior);
    if (ret) {
// goto;
    }
    madvise_init_tlb(&madv_behavior);
    continue;
    }
    if (ret < 0) {
    break;
    }
    iov_iter_advance(iter, iter_iov_len(iter));
    }
    madvise_finish_tlb(&madv_behavior);
    madvise_unlock(&madv_behavior);
// label;
    ret = (total_len - iov_iter_count(iter)) ? : ret;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_process_madvise(pidfd: usize, vec: usize, vlen: usize, behavior: usize, flags: usize) -> c_long {
    let mut ret = 0;
    struct iovec iovstack[UIO_FASTIOV];
    let mut iov = iovstack;
pub static mut iter: usize = 0;
pub static mut task: *mut c_void = core::ptr::null_mut();
pub static mut mm: *mut c_void = core::ptr::null_mut();
    let mut f_flags = 0;
    if (flags != 0) {
    ret = -EINVAL;
// goto;
    }
    ret = import_iovec(ITER_DEST, vec, vlen, ARRAY_SIZE!(iovstack), &iov, &iter);
    if (ret < 0) {
// goto;
    }
    task = pidfd_get_task(pidfd, &f_flags);
    if (IS_ERR(task)) {
    ret = PTR_ERR(task);
// goto;
    }
// Require PTRACE_MODE_READ to avoid leaking ASLR metadata.
    mm = mm_access(task, PTRACE_MODE_READ_FSCREDS);
    if (IS_ERR(mm)) {
    ret = PTR_ERR(mm);
// goto;
    }
    if (!madvise_behavior_valid(behavior)) {
    ret = -EINVAL;
// goto;
    }
//
// We need only perform this check if we are attempting to manipulate a
// remote process's address space.
//
    if (mm != current.mm && !process_madvise_remote_valid(behavior)) {
    ret = -EINVAL;
// goto;
    }
//
// Require CAP_SYS_NICE for influencing process performance. Note that
// only non-destructive hints are currently supported for remote
// processes.
//
    if (mm != current.mm && !capable(CAP_SYS_NICE)) {
    ret = -EPERM;
// goto;
    }
    ret = vector_madvise(mm, &iter, behavior);
// label;
    mmput(mm);
// label;
    put_task_struct(task);
// label;
    kfree(iov);
// label;
    return ret;
    }

pub const ANON_VMA_NAME_MAX_LEN: c_int = 80;

#[no_mangle]
pub unsafe extern "C" fn is_valid_name_char(ch: c_char) -> bool {
// printable ascii characters, excluding ANON_VMA_NAME_INVALID_CHARS
    return ch > 0x1f && ch < 0x7f &&
    !strchr(ANON_VMA_NAME_INVALID_CHARS, ch);
    }
#[no_mangle]
pub unsafe extern "C" fn madvise_set_anon_name(mm: *mut mm_struct, start: c_ulong, len_in: c_ulong, anon_name: *mut anon_vma_name) -> c_int {
    let mut end = 0;
    let mut len = 0;
    let mut error = 0;
pub static mut madvise_behavior: usize = 0;
    if (start & ~PAGE_MASK) {
    return -EINVAL;
    }
    len = (len_in + ~PAGE_MASK) & PAGE_MASK;
// Check to see whether len was rounded up from small -ve to zero
    if (len_in && !len) {
    return -EINVAL;
    }
    end = start + len;
    if (end < start) {
    return -EINVAL;
    }
    if (end == start) {
    return 0;
    }
    madv_behavior.range.start = start;
    madv_behavior.range.end = end;
    error = madvise_lock(&madv_behavior);
    if (error) {
    return error;
    }
    error = madvise_walk_vmas(&madv_behavior);
    madvise_unlock(&madv_behavior);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn set_anon_vma_name(addr: c_ulong, size: c_ulong, uname: *mut c_char) -> c_int {
    let mut anon_name = core::ptr::null_mut();
    let mut mm = current.mm;
    let mut error = 0;
    if (uname) {
    let mut name = core::ptr::null_mut();
    let mut pch = core::ptr::null_mut();
    name = strndup_user(uname, ANON_VMA_NAME_MAX_LEN);
    if (IS_ERR(name)) {
    return PTR_ERR(name);
    }
    while (*pch != '\0') {
    if (!is_valid_name_char(*pch)) {
    kfree(name);
    return -EINVAL;
    }
    }
// anon_vma has its own copy
    anon_name = anon_vma_name_alloc(name);
    kfree(name);
    if (!anon_name) {
    return -ENOMEM;
    }
    }
    error = madvise_set_anon_name(mm, addr, size, anon_name);
    anon_vma_name_put(anon_name);
    return error;
    }
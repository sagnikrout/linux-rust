//! Automatically rewritten from C to Rust
//! Source: mm/mprotect.c
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
// mm/mprotect.c
//
// (C) Copyright 1994 Linus Torvalds
// (C) Copyright 2002 Christoph Hellwig
//
// Address space accounting code	<alan@lxorguk.ukuu.org.uk>
// (C) Copyright 2002 Red Hat Inc, All Rights Reserved
//

#[no_mangle]
unsafe extern "C" fn maybe_change_pte_writable(vma: *mut vm_area_struct, pte: pte_t) -> bool {
    if (WARN_ON_ONCE!(!vma_test(vma, VMA_WRITE_BIT))) {
    return false;
    }
// Don't touch entries that are not even readable.
    if (pte_protnone(pte)) {
    return false;
    }
// Do we need write faults for softdirty tracking?
    if (pte_needs_soft_dirty_wp(vma, pte)) {
    return false;
    }
// Do we need write faults for uffd-wp tracking?
    if (userfaultfd_pte_wp(vma, pte)) {
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn can_change_private_pte_writable(vma: *mut vm_area_struct, addr: c_ulong, pte: pte_t) -> bool {
pub static mut page: *mut c_void = core::ptr::null_mut();
    if (!maybe_change_pte_writable(vma, pte)) {
    return false;
    }
//
// Writable MAP_PRIVATE mapping: We can only special-case on
// exclusive anonymous pages, because we know that our
// write-fault handler similarly would map them writable without
// any additional checks while holding the PT lock.
//
    page = vm_normal_page(vma, addr, pte);
    return page && PageAnon(page) && PageAnonExclusive(page);
    }
#[no_mangle]
pub unsafe extern "C" fn can_change_shared_pte_writable(vma: *mut vm_area_struct, pte: pte_t) -> bool {
    if (!maybe_change_pte_writable(vma, pte)) {
    return false;
    }
    VM_WARN_ON_ONCE(is_zero_pfn(pte_pfn(pte)) && pte_dirty(pte));
//
// Writable MAP_SHARED mapping: "clean" might indicate that the FS still
// needs a real write-fault for writenotify
// (see vma_wants_writenotify()). If "dirty", the assumption is that the
// FS was already notified and we can simply mark the PTE writable
// just like the write-fault handler would do.
//
    return pte_dirty(pte);
    }
#[no_mangle]
pub unsafe extern "C" fn can_change_pte_writable(vma: *mut vm_area_struct, addr: c_ulong, pte: pte_t) -> bool {
    if (!vma_test(vma, VMA_SHARED_BIT)) {
    return can_change_private_pte_writable(vma, addr, pte);
    }
    return can_change_shared_pte_writable(vma, pte);
    }
#[no_mangle]
pub unsafe extern "C" fn mprotect_folio_pte_batch(folio: *mut folio, ptep: *mut pte_t, pte: pte_t, max_nr_ptes: c_int, flags: fpb_t) -> c_int {
// No underlying folio, so cannot batch
    if (!folio) {
    return 1;
    }
    if (!folio_test_large(folio)) {
    return 1;
    }
    return folio_pte_batch_flags(folio, core::ptr::null_mut(), ptep, &pte, max_nr_ptes, flags);
    }
// Set nr_ptes number of ptes, starting from idx
    static __always_inline void prot_commit_flush_ptes(vm_area_struct *vma,
    unsigned long addr, pte_t *ptep, pte_t oldpte, pte_t ptent,
    int nr_ptes, int idx, bool set_write, mmu_gather *tlb)
    {
//
// Advance the position in the batch by idx; note that if idx > 0,
// then the nr_ptes passed here is <= batch size - idx.
//
    addr += idx * PAGE_SIZE;
    ptep += idx;
    oldpte = pte_advance_pfn(oldpte, idx);
    ptent = pte_advance_pfn(ptent, idx);
    if (set_write) {
    ptent = pte_mkwrite(ptent, vma);
    }
    modify_prot_commit_ptes(vma, addr, ptep, oldpte, ptent, nr_ptes);
    if (pte_needs_flush(oldpte, ptent)) {
    tlb_flush_pte_range(tlb, addr, nr_ptes * PAGE_SIZE);
    }
    }
//
// Get max length of consecutive ptes pointing to PageAnonExclusive() pages or
// !PageAnonExclusive() pages, starting from start_idx. Caller must enforce
// that the ptes point to consecutive pages of the same anon large folio.
//
    static __always_inline int page_anon_exclusive_batch(int start_idx, int max_len, page *first_page, bool expected_anon_exclusive)
    {
    let mut idx = 0;
    while (idx < start_idx + max_len) {
    if (expected_anon_exclusive != PageAnonExclusive(first_page + idx)) {
    break;
    }
    }
    return idx - start_idx;
    }
//
// This function is a result of trying our very best to retain the
// "avoid the write-fault handler" optimization. In can_change_pte_writable(),
// if the vma is a private vma, and we cannot determine whether to change
// the pte to writable just from the vma and the pte, we then need to look
// at the actual page pointed to by the pte. Unfortunately, if we have a
// batch of ptes pointing to consecutive pages of the same anon large folio,
// the anon-exclusivity (or the negation) of the first page does not guarantee
// the anon-exclusivity (or the negation) of the other pages corresponding to
// the pte batch; hence in this case it is incorrect to decide to change or
// not change the ptes to writable just by using information from the first
// pte of the batch. Therefore, we must individually check all pages and
// retrieve sub-batches.
//
    static __always_inline void commit_anon_folio_batch(vm_area_struct *vma, folio *folio, page *first_page, unsigned long addr, pte_t *ptep,
    pte_t oldpte, pte_t ptent, int nr_ptes, mmu_gather *tlb)
    {
    let mut expected_anon_exclusive = 0;
pub static mut batch_idx: c_int = 0;
    let mut len = 0;
    while (nr_ptes) {
    expected_anon_exclusive = PageAnonExclusive(first_page + batch_idx);
    len = page_anon_exclusive_batch(batch_idx, nr_ptes,
    first_page, expected_anon_exclusive);
    prot_commit_flush_ptes(vma, addr, ptep, oldpte, ptent, len,
    batch_idx, expected_anon_exclusive, tlb);
    batch_idx += len;
    nr_ptes -= len;
    }
    }
    static __always_inline void set_write_prot_commit_flush_ptes(vm_area_struct *vma, folio *folio, page *page, unsigned long addr, pte_t *ptep,
    pte_t oldpte, pte_t ptent, int nr_ptes, mmu_gather *tlb)
    {
    let mut set_write = 0;
    if (vma_test(vma, VMA_SHARED_BIT)) {
    set_write = can_change_shared_pte_writable(vma, ptent);
    prot_commit_flush_ptes(vma, addr, ptep, oldpte, ptent, nr_ptes,
// idx = */ 0, set_write, tlb);
    return;
    }
    set_write = maybe_change_pte_writable(vma, ptent) &&
    (folio && folio_test_anon(folio));
    if (!set_write) {
    prot_commit_flush_ptes(vma, addr, ptep, oldpte, ptent, nr_ptes,
// idx = */ 0, set_write, tlb);
    return;
    }
    commit_anon_folio_batch(vma, folio, page, addr, ptep, oldpte, ptent, nr_ptes, tlb);
    }
#[no_mangle]
pub unsafe extern "C" fn change_softleaf_pte(vma: *mut vm_area_struct, addr: c_ulong, pte: *mut pte_t, oldpte: pte_t, cp_flags: c_ulong) -> c_long {
pub static mut uffd_prot: bool = false;
    let mut uffd_prot_resolve = cp_flags &
    (MM_CP_UFFD_WP_RESOLVE | MM_CP_UFFD_RWP_RESOLVE);
pub static mut entry: softleaf_t = 0;
    let mut newpte;
    if (softleaf_is_migration_write(entry)) {
    let mut folio = softleaf_to_folio(entry);
//
// A protection check is difficult so
// just be safe and disable write
//
    if (folio_test_anon(folio)) {
    entry = make_readable_exclusive_migration_entry(swp_offset(entry));
    }
    else {
    entry = make_readable_migration_entry(swp_offset(entry));
    }
    newpte = swp_entry_to_pte(entry);
    if (pte_swp_soft_dirty(oldpte)) {
    newpte = pte_swp_mksoft_dirty(newpte);
    }
    } else if (softleaf_is_device_private_write(entry)) {
//
// We do not preserve soft-dirtiness. See
// copy_nonpresent_pte() for explanation.
//
    entry = make_readable_device_private_entry(swp_offset(entry));
    newpte = swp_entry_to_pte(entry);
    if (pte_swp_uffd(oldpte)) {
    newpte = pte_swp_mkuffd(newpte);
    }
    } else if (softleaf_is_marker(entry)) {
//
// Ignore error swap entries unconditionally,
// because any access should sigbus/sigsegv
// anyway.
//
    if (softleaf_is_poison_marker(entry) ||
    softleaf_is_guard_marker(entry)) {
    return 0;
    }
//
// If this is uffd-wp pte marker and we'd like
// to unprotect it, drop it; the next page
// fault will trigger without uffd trapping.
//
    if (uffd_prot_resolve) {
    pte_clear(vma.vm_mm, addr, pte);
    return 1;
    }
    return 0;
    } else {
    newpte = oldpte;
    }
    if (uffd_prot) {
    newpte = pte_swp_mkuffd(newpte);
    }

    else if (uffd_prot_resolve) {
    newpte = pte_swp_clear_uffd(newpte);
    }
    if (!pte_same(oldpte, newpte)) {
    set_pte_at(vma.vm_mm, addr, pte, newpte);
    return 1;
    }
    return 0;
    }
    static __always_inline void change_present_ptes(mmu_gather *tlb, vm_area_struct *vma, unsigned long addr, pte_t *ptep,
    int nr_ptes, unsigned long end, pgprot_t newprot, folio *folio, page *page, unsigned long cp_flags)
    {
pub static mut uffd_prot: bool = false;
    let mut uffd_prot_resolve = cp_flags &
    (MM_CP_UFFD_WP_RESOLVE | MM_CP_UFFD_RWP_RESOLVE);
    pte_t ptent, oldpte;
    oldpte = modify_prot_start_ptes(vma, addr, ptep, nr_ptes);
    ptent = pte_modify(oldpte, newprot);
    if (uffd_prot) {
    ptent = pte_mkuffd(ptent);
    }

    else if (uffd_prot_resolve) {
    ptent = pte_clear_uffd(ptent);
    }
//
// The uffd bit on a VM_UFFD_RWP VMA carries PROT_NONE
// semantics. If mprotect() or NUMA hinting changed the
// base protection, restore PAGE_NONE so the PTE still
// traps on any access. pte_modify() preserves
// _PAGE_UFFD.
//
    if (userfaultfd_rwp(vma) && pte_uffd(ptent)) {
    ptent = pte_modify(ptent, PAGE_NONE);
    }
//
// In some writable, shared mappings, we might want
// to catch actual write access -- see
// vma_wants_writenotify().
//
// In all writable, private mappings, we have to
// properly handle COW.
//
// In both cases, we can sometimes still change PTEs
// writable and avoid the write-fault handler, for
// example, if a PTE is already dirty and no other
// COW or special handling is required.
//
    if ((cp_flags & MM_CP_TRY_CHANGE_WRITABLE) &&
    !pte_write(ptent)) {
    set_write_prot_commit_flush_ptes(vma, folio, page,
    addr, ptep, oldpte, ptent, nr_ptes, tlb);
    }
    else {
    prot_commit_flush_ptes(vma, addr, ptep, oldpte, ptent,
    nr_ptes, /* idx = */ 0, /* set_write = */ false, tlb);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn change_pte_range(tlb: *mut mmu_gather, vma: *mut vm_area_struct, pmd: *mut pmd_t, addr: c_ulong, end: c_ulong, newprot: pgprot_t, cp_flags: c_ulong) -> c_long {
    pte_t *pte, oldpte;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
pub static mut pages: c_long = 0;
    let mut is_private_single_threaded = 0;
pub static mut prot_numa: bool = false;
pub static mut uffd_rwp: bool = false;
pub static mut uffd_wp: bool = false;
    let mut nr_ptes = 0;
    tlb_change_page_size(tlb, PAGE_SIZE);
    pte = pte_offset_map_lock(vma.vm_mm, pmd, addr, &ptl);
    if (!pte) {
    return -EAGAIN;
    }
    if (prot_numa) {
    is_private_single_threaded = vma_is_single_threaded_private(vma);
    }
    flush_tlb_batched_pending(vma.vm_mm);
    lazy_mmu_mode_enable();
    do {
    nr_ptes = 1;
    oldpte = ptep_get(pte);
    if (pte_present(oldpte)) {
pub static mut flags: fpb_t = 0;
pub static mut max_nr_ptes: c_int = 0;
    let mut folio = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
// Already in the desired state.
    if (prot_numa && pte_protnone(oldpte)) {
    continue;
    }
//
// RWP-protected PTEs carry _PAGE_UFFD as a marker on
// top of PROT_NONE. Skip only entries already in that
// exact state; plain PROT_NONE from mprotect() still needs
// to be promoted so future faults can be distinguished.
//
    if (uffd_rwp && pte_protnone(oldpte) && pte_uffd(oldpte)) {
    continue;
    }
    page = vm_normal_page(vma, addr, oldpte);
    if (page) {
    folio = page_folio(page);
    }
//
// Avoid trapping faults against the zero or KSM
// pages. See similar comment in change_huge_pmd.
// Skip this filter for uffd RWP which
// must set protnone regardless of NUMA placement.
//
    if (prot_numa &&
    !folio_can_map_prot_numa(folio, vma,
    is_private_single_threaded)) {
// determine batch to skip
    nr_ptes = mprotect_folio_pte_batch(folio,
    pte, oldpte, max_nr_ptes, /* flags = */ 0);
    continue;
    }
    nr_ptes = mprotect_folio_pte_batch(folio, pte, oldpte, max_nr_ptes, flags);
//
// Optimize for the small-folio common case by
// special-casing it here. Compiler constant propagation
// plus copious amounts of __always_inline does wonders.
//
    if (likely(nr_ptes == 1)) {
    change_present_ptes(tlb, vma, addr, pte, 1,
    end, newprot, folio, page, cp_flags);
    } else {
    change_present_ptes(tlb, vma, addr, pte,
    nr_ptes, end, newprot, folio, page,
    cp_flags);
    }
    pages += nr_ptes;
    } else if (pte_none(oldpte)) {
//
// Nobody plays with any none ptes besides
// userfaultfd when applying the protections.
//
    if (likely(!uffd_wp)) {
    continue;
    }
    if (userfaultfd_wp_use_markers(vma)) {
//
// For file-backed mem, we need to be able to
// wr-protect a none pte, because even if the
// pte is none, the page/swap cache could
// exist.  Doing that by install a marker.
//
    set_pte_at(vma.vm_mm, addr, pte,
    make_pte_marker(PTE_MARKER_UFFD_WP));
    pages += 1;
    }
    } else  {
    pages += change_softleaf_pte(vma, addr, pte, oldpte, cp_flags);
    }
    } while (pte += nr_ptes, addr += nr_ptes * PAGE_SIZE, addr != end);
    lazy_mmu_mode_disable();
    pte_unmap_unlock(pte - 1, ptl);
    return pages;
    }
//
// Return true if we want to split THPs into PTE mappings in change
// protection procedure, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn pgtable_split_needed(vma: *mut vm_area_struct, cp_flags: c_ulong) -> bool {
//
// pte markers only resides in pte level, if we need pte markers,
// we need to split.  For example, we cannot wr-protect a file thp
// (e.g. 2M shmem) because file thp is handled differently when
// split by erasing the pmd so far.
//
    return (cp_flags & (MM_CP_UFFD_WP | MM_CP_UFFD_RWP)) && !vma_is_anonymous(vma);
    }
//
// Return true if we want to populate pgtables in change protection
// procedure, false otherwise
//
#[no_mangle]
pub unsafe extern "C" fn pgtable_populate_needed(vma: *mut vm_area_struct, cp_flags: c_ulong) -> bool {
// If not within ioctl(UFFDIO_WRITEPROTECT), then don't bother
    if (!(cp_flags & MM_CP_UFFD_WP)) {
    return false;
    }
// Populate if the userfaultfd mode requires pte markers
    return userfaultfd_wp_use_markers(vma);
    }
//
// Populate the pgtable underneath for whatever reason if requested.
// When {pte|pmd|...}_alloc() failed we treat it the same way as pgtable
// allocation failures during page faults by kicking OOM and returning
// error.
//

    ({								
    let mut err = 0;						
    if (unlikely(pgtable_populate_needed(vma, cp_flags))) {	
    if (pte_alloc(vma.vm_mm, pmd))			 {
    err = -ENOMEM;				
    }
    }							
    err;							
    })
//
// This is the general pud/p4d/pgd version of change_pmd_prepare(). We need to
// have separate change_pmd_prepare() because pte_alloc() returns 0 on success,
// while {pmd|pud|p4d}_alloc() returns the valid pointer on success.
//

    ({								
    let mut err = 0;						
    if (unlikely(pgtable_populate_needed(vma, cp_flags))) {	
    low##_t *p = low##_alloc(vma.vm_mm, high, addr); 
    if (p == core::ptr::null_mut())					 {
    err = -ENOMEM;				
    }
    }							
    err;							
    })
#[no_mangle]
pub unsafe extern "C" fn change_pmd_range(tlb: *mut mmu_gather, vma: *mut vm_area_struct, pud: *mut pud_t, addr: c_ulong, end: c_ulong, newprot: pgprot_t, cp_flags: c_ulong) -> c_long {
pub static mut pmd: *mut c_void = core::ptr::null_mut();
    let mut next = 0;
pub static mut pages: c_long = 0;
pub static mut nr_huge_updates: c_ulong = 0;
    pmd = pmd_offset(pud, addr);
    do {
    let mut ret = 0;
    let mut _pmd;
// label;
    next = pmd_addr_end(addr, end);
    ret = change_pmd_prepare(vma, pmd, cp_flags);
    if (ret) {
    pages = ret;
    break;
    }
    if (pmd_none(*pmd)) {
// goto;
    }
    _pmd = pmdp_get_lockless(pmd);
    if (pmd_is_huge(_pmd)) {
    if ((next - addr != HPAGE_PMD_SIZE) ||
    pgtable_split_needed(vma, cp_flags)) {
    __split_huge_pmd(vma, pmd, addr, false);
//
// For file-backed, the pmd could have been
// cleared; make sure pmd populated if
// necessary, then fall-through to pte level.
//
    ret = change_pmd_prepare(vma, pmd, cp_flags);
    if (ret) {
    pages = ret;
    break;
    }
    } else {
    ret = change_huge_pmd(tlb, vma, pmd,
    addr, newprot, cp_flags);
    if (ret) {
    if (ret == HPAGE_PMD_NR) {
    pages += HPAGE_PMD_NR;
    nr_huge_updates += 1;
    }
// huge pmd was handled
// goto;
    }
    }
// fall through, the trans huge pmd just split
    }
    ret = change_pte_range(tlb, vma, pmd, addr, next, newprot,
    cp_flags);
    if (ret < 0) {
// goto;
    }
    pages += ret;
// label;
    cond_resched();
    } while (pmd++, addr = next, addr != end);
    if (nr_huge_updates) {
    count_vm_numa_events(NUMA_HUGE_PTE_UPDATES, nr_huge_updates);
    }
    return pages;
    }
#[no_mangle]
pub unsafe extern "C" fn change_pud_range(tlb: *mut mmu_gather, vma: *mut vm_area_struct, p4d: *mut p4d_t, addr: c_ulong, end: c_ulong, newprot: pgprot_t, cp_flags: c_ulong) -> c_long {
pub static mut range: usize = 0;
    pud_t *pudp, pud;
    let mut next = 0;
pub static mut pages: c_long = 0;
    range.start = 0;
    pudp = pud_offset(p4d, addr);
    do {
// label;
    next = pud_addr_end(addr, end);
    ret = change_prepare(vma, pudp, pmd, addr, cp_flags);
    if (ret) {
    pages = ret;
    break;
    }
    pud = pudp_get(pudp);
    if (pud_none(pud)) {
    continue;
    }
    if (!range.start) {
    mmu_notifier_range_init(&range,
    MMU_NOTIFY_PROTECTION_VMA, 0,
    vma.vm_mm, addr, end);
    mmu_notifier_invalidate_range_start(&range);
    }
    if (pud_leaf(pud)) {
    if ((next - addr != PUD_SIZE) ||
    pgtable_split_needed(vma, cp_flags)) {
    __split_huge_pud(vma, pudp, addr);
// goto;
    } else {
    ret = change_huge_pud(tlb, vma, pudp,
    addr, newprot, cp_flags);
    if (ret == 0) {
// goto;
    }
// huge pud was handled
    if (ret == HPAGE_PUD_NR) {
    pages += HPAGE_PUD_NR;
    }
    continue;
    }
    }
    pages += change_pmd_range(tlb, vma, pudp, addr, next, newprot,
    cp_flags);
    } while (pudp++, addr = next, addr != end);
    if (range.start) {
    mmu_notifier_invalidate_range_end(&range);
    }
    return pages;
    }
#[no_mangle]
pub unsafe extern "C" fn change_p4d_range(tlb: *mut mmu_gather, vma: *mut vm_area_struct, pgd: *mut pgd_t, addr: c_ulong, end: c_ulong, newprot: pgprot_t, cp_flags: c_ulong) -> c_long {
pub static mut p4d: *mut c_void = core::ptr::null_mut();
    let mut next = 0;
pub static mut pages: c_long = 0;
    p4d = p4d_offset(pgd, addr);
    do {
    next = p4d_addr_end(addr, end);
    ret = change_prepare(vma, p4d, pud, addr, cp_flags);
    if (ret) {
    return ret;
    }
    if (p4d_none_or_clear_bad(p4d)) {
    continue;
    }
    pages += change_pud_range(tlb, vma, p4d, addr, next, newprot,
    cp_flags);
    } while (p4d++, addr = next, addr != end);
    return pages;
    }
#[no_mangle]
pub unsafe extern "C" fn change_protection_range(tlb: *mut mmu_gather, vma: *mut vm_area_struct, addr: c_ulong, end: c_ulong, newprot: pgprot_t, cp_flags: c_ulong) -> c_long {
    let mut mm = vma.vm_mm;
pub static mut pgd: *mut c_void = core::ptr::null_mut();
    let mut next = 0;
pub static mut pages: c_long = 0;
    BUG_ON!(addr >= end);
    pgd = pgd_offset(mm, addr);
    tlb_start_vma(tlb, vma);
    do {
    next = pgd_addr_end(addr, end);
    ret = change_prepare(vma, pgd, p4d, addr, cp_flags);
    if (ret) {
    pages = ret;
    break;
    }
    if (pgd_none_or_clear_bad(pgd)) {
    continue;
    }
    pages += change_p4d_range(tlb, vma, pgd, addr, next, newprot,
    cp_flags);
    } while (pgd++, addr = next, addr != end);
    tlb_end_vma(tlb, vma);
    return pages;
    }
#[no_mangle]
pub unsafe extern "C" fn change_protection(tlb: *mut mmu_gather, vma: *mut vm_area_struct, start: c_ulong, end: c_ulong, cp_flags: c_ulong) -> c_long {
pub static mut newprot: pgprot_t = 0;
    let mut pages = 0;
//
// MM_CP_UFFD_{WP,RWP} and _RESOLVE are mutually exclusive within one
// change, and WP and RWP cannot mix. Miswired callers get a warn and
// a no-op; userspace cannot reach this state.
//
    if (WARN_ON_ONCE!((cp_flags & MM_CP_UFFD_WP_ALL) == MM_CP_UFFD_WP_ALL ||
    (cp_flags & MM_CP_UFFD_RWP_ALL) == MM_CP_UFFD_RWP_ALL ||
    ((cp_flags & MM_CP_UFFD_WP_ALL) &&
    (cp_flags & MM_CP_UFFD_RWP_ALL)))) {
    return 0;
    }

//
// Ordinary protection updates (mprotect, uffd-wp, softdirty tracking)
// are expected to reflect their requirements via VMA flags such that
// vma_set_page_prot() will adjust vma->vm_page_prot accordingly.
//
    if (cp_flags & MM_CP_PROT_NUMA) {
    newprot = PAGE_NONE;
    }

    WARN_ON_ONCE!(cp_flags & MM_CP_PROT_NUMA);

    if (IS_ENABLED!(CONFIG_ARCH_HAS_PTE_PROTNONE) &&
    (cp_flags & MM_CP_UFFD_RWP)) {
    newprot = PAGE_NONE;
    }
    if (is_vm_hugetlb_page(vma)) {
    pages = hugetlb_change_protection(vma, start, end, newprot,
    cp_flags);
    }
    else {
    pages = change_protection_range(tlb, vma, start, end, newprot,
    cp_flags);
    }
    return pages;
    }
#[no_mangle]
pub unsafe extern "C" fn prot_none_pte_entry(pte: *mut pte_t, addr: c_ulong, next: c_ulong, walk: *mut mm_walk) -> c_int {
    return pfn_modify_allowed(pte_pfn(ptep_get(pte)),
// (walk->private)) ?
    0 : -EACCES;
    }

#[no_mangle]
pub unsafe extern "C" fn prot_none_hugetlb_entry(pte: *mut pte_t, hmask: c_ulong, addr: c_ulong, next: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut entry: pte_t = 0;
    if (pfn_modify_allowed(pte_pfn(entry), *(walk.private))) {
    return 0;
    }
    return -EACCES;
    }

pub static mut mm_walk_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn mprotect_fixup(vmi: *mut vma_iterator, tlb: *mut mmu_gather, vma: *mut vm_area_struct, pprev: *mut *mut vm_area_struct, start: c_ulong, end: c_ulong, newflags: vm_flags_t) -> c_int {
    let mut mm = vma.vm_mm;
pub static mut old_vma_flags: vma_flags_t = 0;
pub static mut new_vma_flags: vma_flags_t = 0;
pub static mut nrpages: c_long = 0;
pub static mut mm_cp_flags: c_uint = 0;
pub static mut charged: c_ulong = 0;
    let mut error = 0;
    if (vma_is_sealed(vma)) {
    return -EPERM;
    }
    if (vma_flags_same_pair(&old_vma_flags, &new_vma_flags)) {
// pprev = vma;
    return 0;
    }
//
// Do PROT_NONE PFN permission checks here when we can still
// bail out without undoing a lot of state. This is a rather
// uncommon case, so doesn't need to be very optimized.
//
    if (arch_has_pfn_modify_check() &&
    vma_flags_test_any(&old_vma_flags, VMA_PFNMAP_BIT,
    VMA_MIXEDMAP_BIT) &&
    !vma_flags_test_any_mask(&new_vma_flags, VMA_ACCESS_FLAGS)) {
pub static mut new_pgprot: pgprot_t = 0;
    error = walk_page_range_vma(vma, start, end,
    &prot_none_walk_ops, &new_pgprot);
    if (error) {
    return error;
    }
    }
//
// If we make a private mapping writable we increase our commit;
// but (without finer accounting) cannot reduce our commit if we
// make it unwritable again except in the anonymous case where no
// anon_vma has yet to be assigned.
//
// hugetlb mapping were accounted for even if read-only so there is
// no need to account for them here.
//
    if (vma_flags_test(&new_vma_flags, VMA_WRITE_BIT)) {
// Check space limits when area turns into data.
    if (!may_expand_vm(mm, &new_vma_flags, nrpages) &&
    may_expand_vm(mm, &old_vma_flags, nrpages)) {
    return -ENOMEM;
    }
    if (!vma_flags_test_any(&old_vma_flags,
    VMA_ACCOUNT_BIT, VMA_WRITE_BIT, VMA_HUGETLB_BIT,
    VMA_SHARED_BIT, VMA_NORESERVE_BIT)) {
    charged = nrpages;
    if (security_vm_enough_memory_mm(mm, charged)) {
    return -ENOMEM;
    }
    vma_flags_set(&new_vma_flags, VMA_ACCOUNT_BIT);
    }
    } else if (vma_flags_test(&old_vma_flags, VMA_ACCOUNT_BIT) &&
    vma_is_anonymous(vma) && !vma.anon_vma) {
    vma_flags_clear(&new_vma_flags, VMA_ACCOUNT_BIT);
    }
    vma = vma_modify_flags(vmi, *pprev, vma, start, end, &new_vma_flags);
    if (IS_ERR(vma)) {
    error = PTR_ERR(vma);
// goto;
    }
// pprev = vma;
//
// vm_flags and vm_page_prot are protected by the mmap_lock
// held in write mode.
//
    vma_start_write(vma);
    vma_flags_reset_once(vma, &new_vma_flags);
    if (vma_wants_manual_pte_write_upgrade(vma)) {
    mm_cp_flags |= MM_CP_TRY_CHANGE_WRITABLE;
    }
    vma_set_page_prot(vma);
    change_protection(tlb, vma, start, end, mm_cp_flags);
    if (vma_flags_test(&old_vma_flags, VMA_ACCOUNT_BIT) &&
    !vma_flags_test(&new_vma_flags, VMA_ACCOUNT_BIT)) {
    vm_unacct_memory(nrpages);
    }
//
// Private VMA_LOCKED_BIT VMA becoming writable: trigger COW to avoid
// major fault on access.
//
    if (vma_flags_test(&new_vma_flags, VMA_WRITE_BIT) &&
    vma_flags_test(&old_vma_flags, VMA_LOCKED_BIT) &&
    !vma_flags_test_any(&old_vma_flags, VMA_WRITE_BIT, VMA_SHARED_BIT)) {
    populate_vma_page_range(vma, start, end, core::ptr::null_mut());
    }
    vm_stat_account(mm, vma_flags_to_legacy(old_vma_flags), -nrpages);
    newflags = vma_flags_to_legacy(new_vma_flags);
    vm_stat_account(mm, newflags, nrpages);
    perf_event_mmap(vma);
    return 0;
// label;
    vm_unacct_memory(charged);
    return error;
    }
//
// pkey==-1 when doing a legacy mprotect()
//
#[no_mangle]
pub unsafe extern "C" fn do_mprotect_pkey(start: c_ulong, len: size_t, prot: c_ulong, pkey: c_int) -> c_int {
    unsigned long nstart, end, tmp, reqprot;
    let mut vma = core::ptr::null_mut();
    let mut prev = core::ptr::null_mut();
    let mut error = 0;
pub static mut grows: c_int = 0;
    let mut rier = (current.personality & READ_IMPLIES_EXEC) &&
    (prot & PROT_READ);
pub static mut tlb: usize = 0;
pub static mut vmi: usize = 0;
    start = untagged_addr(start);
    prot &= ~(PROT_GROWSDOWN|PROT_GROWSUP);
    if (grows == (PROT_GROWSDOWN|PROT_GROWSUP)) /* can't be both */ {
    return -EINVAL;
    }
    if (start & ~PAGE_MASK) {
    return -EINVAL;
    }
    if (!len) {
    return 0;
    }
    len = PAGE_ALIGN(len);
    end = start + len;
    if (end <= start) {
    return -ENOMEM;
    }
    if (!arch_validate_prot(prot, start)) {
    return -EINVAL;
    }
    reqprot = prot;
    if (mmap_write_lock_killable(current.mm)) {
    return -EINTR;
    }
//
// If userspace did not allocate the pkey, do not let
// them use it here.
//
    error = -EINVAL;
    if ((pkey != -1) && !mm_pkey_is_allocated(current.mm, pkey)) {
// goto;
    }
    vma_iter_init(&vmi, current.mm, start);
    vma = vma_find(&vmi, end);
    error = -ENOMEM;
    if (!vma) {
// goto;
    }
    if (unlikely(grows & PROT_GROWSDOWN)) {
    if (vma.vm_start >= end) {
// goto;
    }
    start = vma.vm_start;
    error = -EINVAL;
    if (!vma_test(vma, VMA_GROWSDOWN_BIT)) {
// goto;
    }
    } else {
    if (vma.vm_start > start) {
// goto;
    }
    if (unlikely(grows & PROT_GROWSUP)) {
    end = vma.vm_end;
    error = -EINVAL;
    if (!vma_test_single_mask(vma, VMA_GROWSUP)) {
// goto;
    }
    }
    }
    prev = vma_prev(&vmi);
    if (start > vma.vm_start) {
    prev = vma;
    }
    tlb_gather_mmu(&tlb, current.mm);
    nstart = start;
    tmp = vma.vm_start;
    for_each_vma_range(vmi, vma, end) {
    let mut mask_off_old_flags;
    let mut new_vma_flags;
    let mut newflags;
    let mut new_vma_pkey = 0;
    if (vma.vm_start != tmp) {
    error = -ENOMEM;
    break;
    }
// Does the application expect PROT_READ to imply PROT_EXEC
    if (rier && vma_test(vma, VMA_MAYEXEC_BIT)) {
    prot |= PROT_EXEC;
    }
//
// Each mprotect() call explicitly passes r/w/x permissions.
// If a permission is not passed to mprotect(), it must be
// cleared from the VMA.
//
    mask_off_old_flags = VM_ACCESS_FLAGS | VM_FLAGS_CLEAR;
    new_vma_pkey = arch_override_mprotect_pkey(vma, prot, pkey);
    newflags = calc_vm_prot_bits(prot, new_vma_pkey);
    newflags |= (vma.vm_flags & ~mask_off_old_flags);
    new_vma_flags = legacy_to_vma_flags(newflags);
// newflags >> 4 shift VM_MAY% in place of VM_%
    if ((newflags & ~(newflags >> 4)) & VM_ACCESS_FLAGS) {
    error = -EACCES;
    break;
    }
    if (map_deny_write_exec(&vma.flags, &new_vma_flags)) {
    error = -EACCES;
    break;
    }
// Allow architectures to sanity-check the new flags
    if (!arch_validate_flags(newflags)) {
    error = -EINVAL;
    break;
    }
    error = security_file_mprotect(vma, reqprot, prot);
    if (error) {
    break;
    }
    tmp = vma.vm_end;
    if (tmp > end) {
    tmp = end;
    }
    if (vma.vm_ops && vma.vm_ops.mprotect) {
    error = vma.vm_ops.mprotect(vma, nstart, tmp, newflags);
    if (error) {
    break;
    }
    }
    error = mprotect_fixup(&vmi, &tlb, vma, &prev, nstart, tmp, newflags);
    if (error) {
    break;
    }
    tmp = vma_iter_end(&vmi);
    nstart = tmp;
    prot = reqprot;
    }
    tlb_finish_mmu(&tlb);
    if (!error && tmp < end) {
    error = -ENOMEM;
    }
// label;
    mmap_write_unlock(current.mm);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_mprotect(start: usize, len: usize, prot: usize) -> c_long {
    return do_mprotect_pkey(start, len, prot, -1);
    }

#[no_mangle]
pub unsafe extern "C" fn sys_pkey_mprotect(start: usize, len: usize, prot: usize, pkey: usize) -> c_long {
    return do_mprotect_pkey(start, len, prot, pkey);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_pkey_alloc(flags: usize, init_val: usize) -> c_long {
    let mut pkey = 0;
    let mut ret = 0;
// No flags supported yet.
    if (flags) {
    return -EINVAL;
    }
// check for unsupported init values
    if (init_val & ~PKEY_ACCESS_MASK) {
    return -EINVAL;
    }
    mmap_write_lock(current.mm);
    pkey = mm_pkey_alloc(current.mm);
    ret = -ENOSPC;
    if (pkey == -1) {
// goto;
    }
    ret = arch_set_user_pkey_access(pkey, init_val);
    if (ret) {
    mm_pkey_free(current.mm, pkey);
// goto;
    }
    ret = pkey;
// label;
    mmap_write_unlock(current.mm);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_pkey_free(pkey: usize) -> c_long {
    let mut ret = 0;
    mmap_write_lock(current.mm);
    ret = mm_pkey_free(current.mm, pkey);
    mmap_write_unlock(current.mm);
//
// We could provide warnings or errors if any VMA still
// has the pkey set here.
//
    return ret;
    }
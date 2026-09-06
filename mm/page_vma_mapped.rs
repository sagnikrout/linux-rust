//! Automatically rewritten from C to Rust
//! Source: mm/page_vma_mapped.c
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

#[no_mangle]
pub unsafe extern "C" fn not_found(pvmw: *mut page_vma_mapped_walk) -> bool {
    page_vma_mapped_walk_done(pvmw);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn map_pte(pvmw: *mut page_vma_mapped_walk, pmdvalp: *mut pmd_t, ptlp: *mut *mut spinlock_t) -> bool {
    let mut is_migration = 0;
    let mut ptent;
    if (pvmw.flags & PVMW_SYNC) {
// Use the stricter lookup
    pvmw.pte = pte_offset_map_lock(pvmw.vma.vm_mm, pvmw.pmd,
    pvmw.address, &pvmw.ptl);
// ptlp = pvmw->ptl;
    return !!pvmw.pte;
    }
    is_migration = pvmw.flags & PVMW_MIGRATION;
// label;
//
// It is important to return the ptl corresponding to pte,
// in case *pvmw->pmd changes underneath us; so we need to
// return it even when choosing not to lock, in case caller
// proceeds to loop over next ptes, and finds a match later.
// Though, in most cases, page lock already protects this.
//
    pvmw.pte = pte_offset_map_rw_nolock(pvmw.vma.vm_mm, pvmw.pmd,
    pvmw.address, pmdvalp, ptlp);
    if (!pvmw.pte) {
    return false;
    }
    ptent = ptep_get_lockless(pvmw.pte);
    if (pte_none(ptent)) {
    return false;
    } else if (pte_present(ptent)) {
    if (is_migration) {
    return false;
    }
    } else if (!is_migration) {
    let mut entry;
//
// Handle un-addressable ZONE_DEVICE memory.
//
// We get here when we are trying to unmap a private
// device page from the process address space. Such
// page is not CPU accessible and thus is mapped as
// a special swap entry, nonetheless it still does
// count as a valid regular mapping for the page
// (and is accounted as such in page maps count).
//
// So handle this special case as if it was a normal
// page mapping ie lock CPU page table and return true.
//
// For more details on device private memory see HMM
// (include/linux/hmm.h or mm/hmm.c).
//
    entry = softleaf_from_pte(ptent);
    if (!softleaf_is_device_private(entry) &&
    !softleaf_is_device_exclusive(entry)) {
    return false;
    }
    }
    spin_lock(*ptlp);
    if (unlikely(!pmd_same(*pmdvalp, pmdp_get_lockless(pvmw.pmd)))) {
    pte_unmap_unlock(pvmw.pte, *ptlp);
// goto;
    }
    pvmw.ptl = *ptlp;
    return true;
    }
//
// check_pte - check if [pvmw->pfn, @pvmw->pfn + @pvmw->nr_pages) is
// mapped at the @pvmw->pte
// @pvmw: page_vma_mapped_walk struct, includes a pair pte and pfn range
// for checking
// @pte_nr: the number of small pages described by @pvmw->pte.
//
// page_vma_mapped_walk() found a place where pfn range is *potentially
// mapped. check_pte() has to validate this.
//
// pvmw->pte may point to empty PTE, swap PTE or PTE pointing to
// arbitrary page.
//
// If PVMW_MIGRATION flag is set, returns true if @pvmw->pte contains migration
// entry that points to [pvmw->pfn, @pvmw->pfn + @pvmw->nr_pages)
//
// If PVMW_MIGRATION flag is not set, returns true if pvmw->pte points to
// [pvmw->pfn, @pvmw->pfn + @pvmw->nr_pages)
//
// Otherwise, return false.
//
#[no_mangle]
unsafe extern "C" fn check_pte(pvmw: *mut page_vma_mapped_walk, pte_nr: c_ulong) -> bool {
    let mut pfn = 0;
    let mut ptent;
    if (is_vm_hugetlb_page(pvmw.vma)) {
    ptent = huge_ptep_get(pvmw.vma.vm_mm, pvmw.address,
    pvmw.pte);
    }
    else {
    ptent = ptep_get(pvmw.pte);
    }
    if (pvmw.flags & PVMW_MIGRATION) {
pub static mut entry: softleaf_t = 0;
    if (!softleaf_is_migration(entry)) {
    return false;
    }
    pfn = softleaf_to_pfn(entry);
    } else if (pte_present(ptent)) {
    pfn = pte_pfn(ptent);
    } else {
pub static mut entry: softleaf_t = 0;
// Handle un-addressable ZONE_DEVICE memory
    if (!softleaf_is_device_private(entry) &&
    !softleaf_is_device_exclusive(entry)) {
    return false;
    }
    pfn = softleaf_to_pfn(entry);
    }
    if ((pfn + pte_nr - 1) < pvmw.pfn) {
    return false;
    }
    if (pfn > (pvmw.pfn + pvmw.nr_pages - 1)) {
    return false;
    }
    return true;
    }
// Returns true if the two ranges overlap.  Careful to not overflow.
#[no_mangle]
unsafe extern "C" fn check_pmd(pfn: c_ulong, pvmw: *mut page_vma_mapped_walk) -> bool {
    if ((pfn + HPAGE_PMD_NR - 1) < pvmw.pfn) {
    return false;
    }
    if (pfn > pvmw.pfn + pvmw.nr_pages - 1) {
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn step_forward(pvmw: *mut page_vma_mapped_walk, size: c_ulong) {
    pvmw.address = (pvmw.address + size) & ~(size - 1);
    if (!pvmw.address) {
    pvmw.address = ULONG_MAX;
    }
    }
//
// page_vma_mapped_walk - check if @pvmw->pfn is mapped in @pvmw->vma at
// @pvmw->address
// @pvmw: pointer to struct page_vma_mapped_walk. page, vma, address and flags
// must be set. pmd, pte and ptl must be NULL.
//
// Returns true if the page is mapped in the vma. @pvmw->pmd and @pvmw->pte point
// to relevant page table entries. @pvmw->ptl is locked. @pvmw->address is
// adjusted if needed (for PTE-mapped THPs).
//
// If @pvmw->pmd is set but @pvmw->pte is not, you have found PMD-mapped page
// (usually THP). For PTE-mapped THP, you should run page_vma_mapped_walk() in
// a loop to find all PTEs that map the THP.
//
// For HugeTLB pages, @pvmw->pte is set to the relevant page table entry
// regardless of which page table level the page is mapped at. @pvmw->pmd is
// NULL.
//
// Returns false if there are no more page table entries for the page in
// the vma. @pvmw->ptl is unlocked and @pvmw->pte is unmapped.
//
// If you need to stop the walk before page_vma_mapped_walk() returned false,
// use page_vma_mapped_walk_done(). It will do the housekeeping.
//
#[no_mangle]
pub unsafe extern "C" fn page_vma_mapped_walk(pvmw: *mut page_vma_mapped_walk) -> bool {
    let mut vma = pvmw.vma;
    let mut mm = vma.vm_mm;
    let mut end = 0;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut pteval;
pub static mut pgd: *mut c_void = core::ptr::null_mut();
pub static mut p4d: *mut c_void = core::ptr::null_mut();
pub static mut pud: *mut c_void = core::ptr::null_mut();
    let mut pmde;
// The only possible pmd mapping has been handled on last iteration
    if (pvmw.pmd && !pvmw.pte) {
    return not_found(pvmw);
    }
    if (unlikely(is_vm_hugetlb_page(vma))) {
    let mut hstate = hstate_vma(vma);
pub static mut size: c_ulong = 0;
// The only possible mapping was handled on last iteration
    if (pvmw.pte) {
    return not_found(pvmw);
    }
//
// All callers that get here will already hold the
// i_mmap_rwsem.  Therefore, no additional locks need to be
// taken before calling hugetlb_walk().
//
    pvmw.pte = hugetlb_walk(vma, pvmw.address, size);
    if (!pvmw.pte) {
    return false;
    }
    pvmw.ptl = huge_pte_lock(hstate, mm, pvmw.pte);
    if (!check_pte(pvmw, pages_per_huge_page(hstate))) {
    return not_found(pvmw);
    }
    return true;
    }
    end = vma_address_end(pvmw);
    if (pvmw.pte) {
// goto;
    }
// label;
    do {
    pgd = pgd_offset(mm, pvmw.address);
    if (!pgd_present(*pgd)) {
    step_forward(pvmw, PGDIR_SIZE);
    continue;
    }
    p4d = p4d_offset(pgd, pvmw.address);
    if (!p4d_present(*p4d)) {
    step_forward(pvmw, P4D_SIZE);
    continue;
    }
    pud = pud_offset(p4d, pvmw.address);
    if (!pud_present(*pud)) {
    step_forward(pvmw, PUD_SIZE);
    continue;
    }
    pvmw.pmd = pmd_offset(pud, pvmw.address);
//
// Make sure the pmd value isn't cached in a register by the
// compiler and used as a stale value after we've observed a
// subsequent update.
//
    pmde = pmdp_get_lockless(pvmw.pmd);
    if (IS_ENABLED!(CONFIG_TRANSPARENT_HUGEPAGE) &&
    (pmd_trans_huge(pmde) || pmd_is_migration_entry(pmde) ||
    pmd_is_device_private_entry(pmde))) {
    pvmw.ptl = pmd_lock(mm, pvmw.pmd);
    pmde = *pvmw.pmd;
    if (pmd_is_migration_entry(pmde)) {
    let mut entry;
    if (!(pvmw.flags & PVMW_MIGRATION)) {
    return not_found(pvmw);
    }
    entry = softleaf_from_pmd(pmde);
    if (!check_pmd(softleaf_to_pfn(entry), pvmw)) {
    return not_found(pvmw);
    }
    return true;
    } else if (pmd_is_device_private_entry(pmde)) {
    let mut entry;
    if (pvmw.flags & PVMW_MIGRATION) {
    return not_found(pvmw);
    }
    entry = softleaf_from_pmd(pmde);
    if (!check_pmd(softleaf_to_pfn(entry), pvmw)) {
    return not_found(pvmw);
    }
    return true;
    } else if (!pmd_present(pmde)) {
    return not_found(pvmw);
    }
    if (likely(pmd_trans_huge(pmde))) {
    if (pvmw.flags & PVMW_MIGRATION) {
    return not_found(pvmw);
    }
    if (!check_pmd(pmd_pfn(pmde), pvmw)) {
    return not_found(pvmw);
    }
    return true;
    }
// THP/device-private pmd was split under us: handle on pte level
    spin_unlock(pvmw.ptl);
    pvmw.ptl = core::ptr::null_mut();
    } else if (!pmd_present(pmde)) {
    if ((pvmw.flags & PVMW_SYNC) &&
    thp_vma_suitable_order(vma, pvmw.address,
    PMD_ORDER) &&
    (pvmw.nr_pages >= HPAGE_PMD_NR)) {
    sync_with_folio_pmd_zap(mm, pvmw.pmd);
    }
    step_forward(pvmw, PMD_SIZE);
    continue;
    }
    if (!map_pte(pvmw, &pmde, &ptl)) {
    if (!pvmw.pte) {
// goto;
    }
// goto;
    }
// label;
    if (check_pte(pvmw, 1)) {
    return true;
    }
// label;
    do {
    pvmw.address += PAGE_SIZE;
    if (pvmw.address >= end) {
    return not_found(pvmw);
    }
// Did we cross page table boundary?
    if ((pvmw.address & (PMD_SIZE - PAGE_SIZE)) == 0) {
    if (pvmw.ptl) {
    spin_unlock(pvmw.ptl);
    pvmw.ptl = core::ptr::null_mut();
    }
    pte_unmap(pvmw.pte);
    pvmw.pte = core::ptr::null_mut();
    pvmw.flags |= PVMW_PGTABLE_CROSSED;
// goto;
    }
    pvmw.pte += 1;
    if (!pvmw.ptl) {
    pteval = ptep_get_lockless(pvmw.pte);
    }
    else {
    pteval = ptep_get(pvmw.pte);
    }
    } while (pte_none(pteval));
    if (!pvmw.ptl) {
    spin_lock(ptl);
    if (unlikely(!pmd_same(pmde, pmdp_get_lockless(pvmw.pmd)))) {
    pte_unmap_unlock(pvmw.pte, ptl);
    pvmw.pte = core::ptr::null_mut();
// goto;
    }
    pvmw.ptl = ptl;
    }
// goto;
    } while (pvmw.address < end);
    return false;
    }

//
// page_mapped_in_vma - check whether a page is really mapped in a VMA
// @page: the page to test
// @vma: the VMA to test
//
// Return: The address the page is mapped at if the page is in the range
// covered by the VMA and present in the page table.  If the page is
// outside the VMA or not present, returns -EFAULT.
// Only valid for normal file or anonymous VMAs.
//
#[no_mangle]
pub unsafe extern "C" fn page_mapped_in_vma(page: *mut page, vma: *mut vm_area_struct) -> c_ulong {
    let mut folio = page_folio(page);
pub static mut pgoff: pgoff_t = 0;
pub static mut page_vma_mapped_walk: usize = 0;
    if (folio_test_anon(folio)) {
    pvmw.address = vma_anon_address(vma, pgoff, 1);
    }
    else {
    pvmw.address = vma_filebacked_address(vma, pgoff, 1);
    }
    if (pvmw.address == -EFAULT) {
// goto;
    }
    if (!page_vma_mapped_walk(&pvmw)) {
    return -EFAULT;
    }
    page_vma_mapped_walk_done(&pvmw);
// label;
    return pvmw.address;
    }
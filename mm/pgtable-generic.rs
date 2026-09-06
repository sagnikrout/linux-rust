//! Automatically rewritten from C to Rust
//! Source: mm/pgtable-generic.c
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
// mm/pgtable-generic.c
//
// Generic pgtable methods declared in linux/pgtable.h
//
// Copyright (C) 2010  Linus Torvalds
//

//
// If a p?d_bad entry is found while walking page tables, report
// the error, before resetting entry to p?d_none.  Usually (but
// very seldom) called out from the p?d_none_or_clear_bad macros.
//
#[no_mangle]
pub unsafe extern "C" fn pgd_clear_bad(pgd: *mut pgd_t) {
    pgd_ERROR(*pgd);
    pgd_clear(pgd);
    }

#[no_mangle]
pub unsafe extern "C" fn p4d_clear_bad(p4d: *mut p4d_t) {
    p4d_ERROR(*p4d);
    p4d_clear(p4d);
    }

#[no_mangle]
pub unsafe extern "C" fn pud_clear_bad(pud: *mut pud_t) {
    pud_ERROR(*pud);
    pud_clear(pud);
    }

//
// Note that the pmd variant below can't be stub'ed out just as for p4d/pud
// above. pmd folding is special and typically pmd_* macros refer to upper
// level even when folded
//
#[no_mangle]
pub unsafe extern "C" fn pmd_clear_bad(pmd: *mut pmd_t) {
    pmd_ERROR(*pmd);
    pmd_clear(pmd);
    }

//
// Only sets the access flags (dirty, accessed), as well as write
// permission. Furthermore, we know it always gets set to a "more
// permissive" setting, which allows most architectures to optimize
// this. We return whether the PTE actually changed, which in turn
// instructs the caller to do things like update__mmu_cache.  This
// used to be done in the caller, but sparc needs minor faults to
// force that call on sun4c so we changed this macro slightly
//
#[no_mangle]
pub unsafe extern "C" fn ptep_set_access_flags(vma: *mut vm_area_struct, address: c_ulong, ptep: *mut pte_t, entry: pte_t, dirty: c_int) -> c_int {
pub static mut changed: c_int = 0;
    if (changed) {
    set_pte_at(vma.vm_mm, address, ptep, entry);
    flush_tlb_fix_spurious_fault(vma, address, ptep);
    }
    return changed;
    }

#[no_mangle]
pub unsafe extern "C" fn ptep_clear_flush_young(vma: *mut vm_area_struct, address: c_ulong, ptep: *mut pte_t) -> bool {
    let mut young = 0;
    young = ptep_test_and_clear_young(vma, address, ptep);
    if (young) {
    flush_tlb_page(vma, address);
    }
    return young;
    }

    pte_t ptep_clear_flush(vm_area_struct *vma, unsigned long address,
    pte_t *ptep)
    {
    let mut mm = (vma).vm_mm;
    let mut pte;
    pte = ptep_get_and_clear(mm, address, ptep);
    if (pte_accessible(mm, pte)) {
    flush_tlb_page(vma, address);
    }
    return pte;
    }

#[no_mangle]
pub unsafe extern "C" fn pmdp_set_access_flags(vma: *mut vm_area_struct, address: c_ulong, pmdp: *mut pmd_t, entry: pmd_t, dirty: c_int) -> c_int {
pub static mut changed: c_int = 0;
    VM_BUG_ON(address & ~HPAGE_PMD_MASK);
    if (changed) {
    set_pmd_at(vma.vm_mm, address, pmdp, entry);
    flush_pmd_tlb_range(vma, address, address + HPAGE_PMD_SIZE);
    }
    return changed;
    }

#[no_mangle]
pub unsafe extern "C" fn pmdp_clear_flush_young(vma: *mut vm_area_struct, address: c_ulong, pmdp: *mut pmd_t) -> bool {
    let mut young = 0;
    VM_BUG_ON(address & ~HPAGE_PMD_MASK);
    young = pmdp_test_and_clear_young(vma, address, pmdp);
    if (young) {
    flush_pmd_tlb_range(vma, address, address + HPAGE_PMD_SIZE);
    }
    return young;
    }

    pmd_t pmdp_huge_clear_flush(vm_area_struct *vma, unsigned long address,
    pmd_t *pmdp)
    {
    let mut pmd;
    VM_BUG_ON(address & ~HPAGE_PMD_MASK);
    VM_BUG_ON(pmd_present(*pmdp) && !pmd_trans_huge(*pmdp));
    pmd = pmdp_huge_get_and_clear(vma.vm_mm, address, pmdp);
    flush_pmd_tlb_range(vma, address, address + HPAGE_PMD_SIZE);
    return pmd;
    }

    pud_t pudp_huge_clear_flush(vm_area_struct *vma, unsigned long address,
    pud_t *pudp)
    {
    let mut pud;
    VM_BUG_ON(address & ~HPAGE_PUD_MASK);
    VM_BUG_ON(!pud_trans_huge(*pudp));
    pud = pudp_huge_get_and_clear(vma.vm_mm, address, pudp);
    flush_pud_tlb_range(vma, address, address + HPAGE_PUD_SIZE);
    return pud;
    }

#[no_mangle]
pub unsafe extern "C" fn pgtable_trans_huge_deposit(mm: *mut mm_struct, pmdp: *mut pmd_t, pgtable: pgtable_t) {
    assert_spin_locked(pmd_lockptr(mm, pmdp));
// FIFO
    if (!pmd_huge_pte(mm, pmdp)) {
    INIT_LIST_HEAD(&pgtable.lru);
    }
    else {
    list_add(&pgtable.lru, &pmd_huge_pte(mm, pmdp).lru);
    }
    pmd_huge_pte(mm, pmdp) = pgtable;
    }

// no "address" argument so destroys page coloring of some arch
#[no_mangle]
pub unsafe extern "C" fn pgtable_trans_huge_withdraw(mm: *mut mm_struct, pmdp: *mut pmd_t) -> pgtable_t {
    let mut pgtable;
    assert_spin_locked(pmd_lockptr(mm, pmdp));
// FIFO
    pgtable = pmd_huge_pte(mm, pmdp);
    pmd_huge_pte(mm, pmdp) = list_first_entry_or_null(&pgtable.lru, page, lru);
    if (pmd_huge_pte(mm, pmdp)) {
    list_del(&pgtable.lru);
    }
    return pgtable;
    }

    pmd_t pmdp_invalidate(vm_area_struct *vma, unsigned long address,
    pmd_t *pmdp)
    {
    VM_WARN_ON_ONCE(!pmd_present(*pmdp));
pub static mut old: pmd_t = 0;
    flush_pmd_tlb_range(vma, address, address + HPAGE_PMD_SIZE);
    return old;
    }

    pmd_t pmdp_invalidate_ad(vm_area_struct *vma, unsigned long address,
    pmd_t *pmdp)
    {
    VM_WARN_ON_ONCE(!pmd_present(*pmdp));
    return pmdp_invalidate(vma, address, pmdp);
    }

    pmd_t pmdp_collapse_flush(vm_area_struct *vma, unsigned long address,
    pmd_t *pmdp)
    {
//
// pmd and hugepage pte format are same. So we could
// use the same function.
//
    let mut pmd;
    VM_BUG_ON(address & ~HPAGE_PMD_MASK);
    VM_BUG_ON(pmd_trans_huge(*pmdp));
    pmd = pmdp_huge_get_and_clear(vma.vm_mm, address, pmdp);
// collapse entails shooting down ptes not pmd
    flush_tlb_range(vma, address, address + HPAGE_PMD_SIZE);
    return pmd;
    }

// arch define pte_free_defer in asm/pgalloc.h for its own implementation

#[no_mangle]
unsafe extern "C" fn pte_free_now(head: *mut rcu_head) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    page = container_of!(head, page, rcu_head);
    pte_free(core::ptr::null_mut() /* mm not passed and not used */, (pgtable_t)page);
    }
#[no_mangle]
pub unsafe extern "C" fn pte_free_defer(mm: *mut mm_struct, pgtable: pgtable_t) {
pub static mut page: *mut c_void = core::ptr::null_mut();
    page = pgtable;
    call_rcu(&page.rcu_head, pte_free_now);
    }

    (defined(CONFIG_SMP) || defined(CONFIG_PREEMPT_RCU))
//
// See the comment above ptep_get_lockless() in include/linux/pgtable.h:
// the barriers in pmdp_get_lockless() cannot guarantee that the value in
// pmd_high actually belongs with the value in pmd_low; but holding interrupts
// off blocks the TLB flush between present updates, which guarantees that a
// successful __pte_offset_map() points to a page from matched halves.
//
#[no_mangle]
unsafe extern "C" fn pmdp_get_lockless_start() -> c_ulong {
    let mut irqflags = 0;
    local_irq_save(irqflags);
    return irqflags;
    }
#[no_mangle]
unsafe extern "C" fn pmdp_get_lockless_end(irqflags: c_ulong) {
    local_irq_restore(irqflags);
    }

#[no_mangle]
pub unsafe extern "C" fn pmdp_get_lockless_start() -> c_ulong { return 0; }
#[no_mangle]
pub unsafe extern "C" fn pmdp_get_lockless_end(irqflags: c_ulong) { }

    pte_t *__pte_offset_map(pmd_t *pmd, unsigned long addr, pmd_t *pmdvalp)
    {
    let mut irqflags = 0;
    let mut pmdval;
    rcu_read_lock();
    irqflags = pmdp_get_lockless_start();
    pmdval = pmdp_get_lockless(pmd);
    pmdp_get_lockless_end(irqflags);
    if (pmdvalp) {
// pmdvalp = pmdval;
    }
    if (unlikely(pmd_none(pmdval) || !pmd_present(pmdval))) {
// goto;
    }
    if (unlikely(pmd_trans_huge(pmdval))) {
// goto;
    }
    if (unlikely(pmd_bad(pmdval))) {
    pmd_clear_bad(pmd);
// goto;
    }
    return __pte_map(&pmdval, addr);
// label;
    rcu_read_unlock();
    return core::ptr::null_mut();
    }
    pte_t *pte_offset_map_ro_nolock(mm_struct *mm, pmd_t *pmd,
    unsigned long addr, spinlock_t **ptlp)
    {
    let mut pmdval;
pub static mut pte: *mut c_void = core::ptr::null_mut();
    pte = __pte_offset_map(pmd, addr, &pmdval);
    if (likely(pte)) {
// ptlp = pte_lockptr(mm, &pmdval);
    }
    return pte;
    }
    pte_t *pte_offset_map_rw_nolock(mm_struct *mm, pmd_t *pmd,
    unsigned long addr, pmd_t *pmdvalp,
    spinlock_t **ptlp)
    {
pub static mut pte: *mut c_void = core::ptr::null_mut();
    VM_WARN_ON_ONCE(!pmdvalp);
    pte = __pte_offset_map(pmd, addr, pmdvalp);
    if (likely(pte)) {
// ptlp = pte_lockptr(mm, pmdvalp);
    }
    return pte;
    }
//
// pte_offset_map_lock(mm, pmd, addr, ptlp) is usually called with the pmd
// pointer for addr, reached by walking down the mm's pgd, p4d, pud for addr:
// either while holding mmap_lock or vma lock for read or for write; or in
// truncate or rmap context, while holding file's i_mmap_lock or anon_vma lock
// for read (or for write). In a few cases, it may be used with pmd pointing to
// a pmd_t already copied to or constructed on the stack.
//
// When successful, it returns the pte pointer for addr, with its page table
// kmapped if necessary (when CONFIG_HIGHPTE), and locked against concurrent
// modification by software, with a pointer to that spinlock in ptlp (in some
// configs mm->page_table_lock, in SPLIT_PTLOCK configs a spinlock in table's
// struct page).  pte_unmap_unlock(pte, ptl) to unlock and unmap afterwards.
//
// But it is unsuccessful, returning NULL with *ptlp unchanged, if there is no
// page table at *pmd: if, for example, the page table has just been removed,
// or replaced by the huge pmd of a THP.  (When successful, *pmd is rechecked
// after acquiring the ptlock, and retried internally if it changed: so that a
// page table can be safely removed or replaced by THP while holding its lock.)
//
// pte_offset_map(pmd, addr), and its internal helper __pte_offset_map() above,
// just returns the pte pointer for addr, its page table kmapped if necessary;
// or NULL if there is no page table at *pmd.  It does not attempt to lock the
// page table, so cannot normally be used when the page table is to be updated,
// or when entries read must be stable.  But it does take rcu_read_lock(): so
// that even when page table is racily removed, it remains a valid though empty
// and disconnected table.  Until pte_unmap(pte) unmaps and rcu_read_unlock()s
// afterwards.
//
// pte_offset_map_ro_nolock(mm, pmd, addr, ptlp), above, is like pte_offset_map();
// but when successful, it also outputs a pointer to the spinlock in ptlp - as
// pte_offset_map_lock() does, but in this case without locking it.  This helps
// the caller to avoid a later pte_lockptr(mm, *pmd), which might by that time
// act on a changed *pmd: pte_offset_map_ro_nolock() provides the correct spinlock
// pointer for the page table that it returns. Even after grabbing the spinlock,
// we might be looking either at a page table that is still mapped or one that
// was unmapped and is about to get freed. But for R/O access this is sufficient.
// So it is only applicable for read-only cases where any modification operations
// to the page table are not allowed even if the corresponding spinlock is held
// afterwards.
//
// pte_offset_map_rw_nolock(mm, pmd, addr, pmdvalp, ptlp), above, is like
// pte_offset_map_ro_nolock(); but when successful, it also outputs the pdmval.
// It is applicable for may-write cases where any modification operations to the
// page table may happen after the corresponding spinlock is held afterwards.
// But the users should make sure the page table is stable like checking pte_same()
// or checking pmd_same() by using the output pmdval before performing the write
// operations.
//
// Note: "RO" / "RW" expresses the intended semantics, not that the *kmap* will
// be read-only/read-write protected.
//
// Note that free_pgtables(), used after unmapping detached vmas, or when
// exiting the whole mm, does not take page table lock before freeing a page
// table, and may not use RCU at all: "outsiders" like khugepaged should avoid
// pte_offset_map() and co once the vma is detached from mm or mm_users is zero.
//
    pte_t *pte_offset_map_lock(mm_struct *mm, pmd_t *pmd,
    unsigned long addr, spinlock_t **ptlp)
    {
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut pmdval;
pub static mut pte: *mut c_void = core::ptr::null_mut();
// label;
    pte = __pte_offset_map(pmd, addr, &pmdval);
    if (unlikely(!pte)) {
    return pte;
    }
    ptl = pte_lockptr(mm, &pmdval);
    spin_lock(ptl);
    if (likely(pmd_same(pmdval, pmdp_get_lockless(pmd)))) {
// ptlp = ptl;
    return pte;
    }
    pte_unmap_unlock(pte, ptl);
// goto;
    }

// forward_decl: kernel_pgtable_work_func;
pub static mut kernel_pgtable_work: usize = 0;
#[no_mangle]
unsafe extern "C" fn kernel_pgtable_work_func(work: *mut work_struct) {
    let mut pt = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
pub static mut page_list: usize = 0;
    spin_lock(&kernel_pgtable_work.lock);
    list_splice_tail_init(&kernel_pgtable_work.list, &page_list);
    spin_unlock(&kernel_pgtable_work.lock);
    iommu_sva_invalidate_kva_range(PAGE_OFFSET, TLB_FLUSH_ALL);
    list_for_each_entry_safe(pt, next, &page_list, pt_list) {
    __pagetable_free(pt);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pagetable_free_kernel(pt: *mut ptdesc) {
    spin_lock(&kernel_pgtable_work.lock);
    list_add(&pt.pt_list, &kernel_pgtable_work.list);
    spin_unlock(&kernel_pgtable_work.lock);
    schedule_work(&kernel_pgtable_work.work);
    }
//! Automatically rewritten from C to Rust
//! Source: mm/debug_vm_pgtable.c
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
// This kernel test validates architecture page table helpers and
// accessors and helps in verifying their continued compliance with
// expected generic MM semantics.
//
// Copyright (C) 2019 ARM Ltd.
//
// Author: Anshuman Khandual <anshuman.khandual@arm.com>
//

//
// Please refer Documentation/mm/arch_pgtable_helpers.rst for the semantics
// expectations that are being validated here. All future changes in here
// or the documentation need to be in sync.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgtable_debug_args {
    pub mm: *mut mm_struct,
    pub vma: *mut vm_area_struct,
    pub pgdp: *mut pgd_t,
    pub p4dp: *mut p4d_t,
    pub pudp: *mut pud_t,
    pub pmdp: *mut pmd_t,
    pub ptep: *mut pte_t,
    pub start_p4dp: *mut p4d_t,
    pub start_pudp: *mut pud_t,
    pub start_pmdp: *mut pmd_t,
    pub start_ptep: pgtable_t,
    pub vaddr: c_ulong,
    pub page_prot: pgprot_t,
    pub page_prot_none: pgprot_t,
    pub is_contiguous_page: bool,
    pub pud_pfn: c_ulong,
    pub pmd_pfn: c_ulong,
    pub pte_pfn: c_ulong,
    pub fixed_alignment: c_ulong,
    pub fixed_pgd_pfn: c_ulong,
    pub fixed_p4d_pfn: c_ulong,
    pub fixed_pud_pfn: c_ulong,
    pub fixed_pmd_pfn: c_ulong,
    pub fixed_pte_pfn: c_ulong,
    pub swp_entry: swp_entry_t,
    pub leaf_entry: swp_entry_t,
}

#[no_mangle]
unsafe extern "C" fn pte_basic_tests(args: *mut pgtable_debug_args, idx: c_int)  {
pub static mut prot: pgprot_t = 0;
pub static mut pte: pte_t = 0;
pub static mut val: c_ulong = 0;
    pr_debug!("Validating PTE basic (%pGv)\n", ptr);
//
// This test needs to be executed after the given page table entry
// is created with pfn_pte() to make sure that vm_get_page_prot(idx)
// does not have the dirty bit enabled from the beginning. This is
// important for platforms like arm64 where (!PTE_RDONLY) indicate
// dirty bit being set.
//
    WARN_ON!(pte_dirty(pte_wrprotect(pte)));
    WARN_ON!(!pte_same(pte, pte));
    WARN_ON!(!pte_young(pte_mkyoung(pte_mkold(pte))));
    WARN_ON!(!pte_dirty(pte_mkdirty(pte_mkclean(pte))));
    WARN_ON!(!pte_write(pte_mkwrite(pte_wrprotect(pte), args.vma)));
    WARN_ON!(pte_young(pte_mkold(pte_mkyoung(pte))));
    WARN_ON!(pte_dirty(pte_mkclean(pte_mkdirty(pte))));
    WARN_ON!(pte_write(pte_wrprotect(pte_mkwrite(pte, args.vma))));
    WARN_ON!(pte_dirty(pte_wrprotect(pte_mkclean(pte))));
    WARN_ON!(!pte_dirty(pte_wrprotect(pte_mkdirty(pte))));
    WARN_ON!(!pte_dirty(pte_mkwrite_novma(pte_mkdirty(pte))));
    WARN_ON!(pte_dirty(pte_mkwrite_novma(pte_mkclean(pte))));
    WARN_ON!(!pte_write(pte_mkdirty(pte_mkwrite_novma(pte))));
    WARN_ON!(!pte_write(pte_mkwrite_novma(pte_wrprotect(pte))));
    WARN_ON!(pte_write(pte_wrprotect(pte_mkwrite_novma(pte))));
    }
#[no_mangle]
unsafe extern "C" fn pte_advanced_tests(args: *mut pgtable_debug_args)  {
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut pte;
//
// Architectures optimize set_pte_at by avoiding TLB flush.
// This requires set_pte_at to be not used to update an
// existing pte entry. Clear pte before we do set_pte_at
//
// flush_dcache_page() is called after set_pte_at() to clear
// PG_arch_1 for the page on ARM64. The page flag isn't cleared
// when it's released and page allocation check will fail when
// the page is allocated again. For architectures other than ARM64,
// the unexpected overhead of cache flushing is acceptable.
//
    page = (args.pte_pfn != ULONG_MAX) ? pfn_to_page(args.pte_pfn) : core::ptr::null_mut();
    if (!page) {
    return;
    }
    pr_debug!("Validating PTE advanced\n");
    if (WARN_ON!(!args.ptep)) {
    return;
    }
    pte = pfn_pte(args.pte_pfn, args.page_prot);
    set_pte_at(args.mm, args.vaddr, args.ptep, pte);
    flush_dcache_page(page);
    ptep_set_wrprotect(args.mm, args.vaddr, args.ptep);
    pte = ptep_get(args.ptep);
    WARN_ON!(pte_write(pte));
    ptep_get_and_clear(args.mm, args.vaddr, args.ptep);
    pte = ptep_get(args.ptep);
    WARN_ON!(!pte_none(pte));
    pte = pfn_pte(args.pte_pfn, args.page_prot);
    pte = pte_wrprotect(pte);
    pte = pte_mkclean(pte);
    set_pte_at(args.mm, args.vaddr, args.ptep, pte);
    flush_dcache_page(page);
    pte = pte_mkwrite(pte, args.vma);
    pte = pte_mkdirty(pte);
    ptep_set_access_flags(args.vma, args.vaddr, args.ptep, pte, 1);
    pte = ptep_get(args.ptep);
    WARN_ON!(!(pte_write(pte) && pte_dirty(pte)));
    ptep_get_and_clear_full(args.mm, args.vaddr, args.ptep, 1);
    pte = ptep_get(args.ptep);
    WARN_ON!(!pte_none(pte));
    pte = pfn_pte(args.pte_pfn, args.page_prot);
    pte = pte_mkyoung(pte);
    set_pte_at(args.mm, args.vaddr, args.ptep, pte);
    flush_dcache_page(page);
    ptep_test_and_clear_young(args.vma, args.vaddr, args.ptep);
    pte = ptep_get(args.ptep);
    WARN_ON!(pte_young(pte));
    ptep_get_and_clear_full(args.mm, args.vaddr, args.ptep, 1);
    }

#[no_mangle]
unsafe extern "C" fn pmd_basic_tests(args: *mut pgtable_debug_args, idx: c_int)  {
pub static mut prot: pgprot_t = 0;
pub static mut val: c_ulong = 0;
    let mut pmd;
    if (!has_transparent_hugepage()) {
    return;
    }
    pr_debug!("Validating PMD basic (%pGv)\n", ptr);
    pmd = pfn_pmd(args.fixed_pmd_pfn, prot);
//
// This test needs to be executed after the given page table entry
// is created with pfn_pmd() to make sure that vm_get_page_prot(idx)
// does not have the dirty bit enabled from the beginning. This is
// important for platforms like arm64 where (!PTE_RDONLY) indicate
// dirty bit being set.
//
    WARN_ON!(pmd_dirty(pmd_wrprotect(pmd)));
    WARN_ON!(!pmd_same(pmd, pmd));
    WARN_ON!(!pmd_young(pmd_mkyoung(pmd_mkold(pmd))));
    WARN_ON!(!pmd_dirty(pmd_mkdirty(pmd_mkclean(pmd))));
    WARN_ON!(!pmd_write(pmd_mkwrite(pmd_wrprotect(pmd), args.vma)));
    WARN_ON!(pmd_young(pmd_mkold(pmd_mkyoung(pmd))));
    WARN_ON!(pmd_dirty(pmd_mkclean(pmd_mkdirty(pmd))));
    WARN_ON!(pmd_write(pmd_wrprotect(pmd_mkwrite(pmd, args.vma))));
    WARN_ON!(pmd_dirty(pmd_wrprotect(pmd_mkclean(pmd))));
    WARN_ON!(!pmd_dirty(pmd_wrprotect(pmd_mkdirty(pmd))));
    WARN_ON!(!pmd_dirty(pmd_mkwrite_novma(pmd_mkdirty(pmd))));
    WARN_ON!(pmd_dirty(pmd_mkwrite_novma(pmd_mkclean(pmd))));
    WARN_ON!(!pmd_write(pmd_mkdirty(pmd_mkwrite_novma(pmd))));
    WARN_ON!(!pmd_write(pmd_mkwrite_novma(pmd_wrprotect(pmd))));
    WARN_ON!(pmd_write(pmd_wrprotect(pmd_mkwrite_novma(pmd))));
//
// A huge page does not point to next level page table
// entry. Hence this must qualify as pmd_bad().
//
    WARN_ON!(!pmd_bad(pmd_mkhuge(pmd)));
    }
#[no_mangle]
unsafe extern "C" fn pmd_advanced_tests(args: *mut pgtable_debug_args)  {
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut pmd;
pub static mut vaddr: c_ulong = 0;
    if (!has_transparent_hugepage()) {
    return;
    }
    page = (args.pmd_pfn != ULONG_MAX) ? pfn_to_page(args.pmd_pfn) : core::ptr::null_mut();
    if (!page) {
    return;
    }
//
// flush_dcache_page() is called after set_pmd_at() to clear
// PG_arch_1 for the page on ARM64. The page flag isn't cleared
// when it's released and page allocation check will fail when
// the page is allocated again. For architectures other than ARM64,
// the unexpected overhead of cache flushing is acceptable.
//
    pr_debug!("Validating PMD advanced\n");
// Align the address wrt HPAGE_PMD_SIZE
    vaddr &= HPAGE_PMD_MASK;
    pgtable_trans_huge_deposit(args.mm, args.pmdp, args.start_ptep);
    pmd = pfn_pmd(args.pmd_pfn, args.page_prot);
    set_pmd_at(args.mm, vaddr, args.pmdp, pmd);
    flush_dcache_page(page);
    pmdp_set_wrprotect(args.mm, vaddr, args.pmdp);
    pmd = pmdp_get(args.pmdp);
    WARN_ON!(pmd_write(pmd));
    pmdp_huge_get_and_clear(args.mm, vaddr, args.pmdp);
    pmd = pmdp_get(args.pmdp);
    WARN_ON!(!pmd_none(pmd));
    pmd = pfn_pmd(args.pmd_pfn, args.page_prot);
    pmd = pmd_wrprotect(pmd);
    pmd = pmd_mkclean(pmd);
    set_pmd_at(args.mm, vaddr, args.pmdp, pmd);
    flush_dcache_page(page);
    pmd = pmd_mkwrite(pmd, args.vma);
    pmd = pmd_mkdirty(pmd);
    pmdp_set_access_flags(args.vma, vaddr, args.pmdp, pmd, 1);
    pmd = pmdp_get(args.pmdp);
    WARN_ON!(!(pmd_write(pmd) && pmd_dirty(pmd)));
    pmdp_huge_get_and_clear_full(args.vma, vaddr, args.pmdp, 1);
    pmd = pmdp_get(args.pmdp);
    WARN_ON!(!pmd_none(pmd));
    pmd = pmd_mkhuge(pfn_pmd(args.pmd_pfn, args.page_prot));
    pmd = pmd_mkyoung(pmd);
    set_pmd_at(args.mm, vaddr, args.pmdp, pmd);
    flush_dcache_page(page);
    pmdp_test_and_clear_young(args.vma, vaddr, args.pmdp);
    pmd = pmdp_get(args.pmdp);
    WARN_ON!(pmd_young(pmd));
// Clear the pte entries
    pmdp_huge_get_and_clear(args.mm, vaddr, args.pmdp);
    pgtable_trans_huge_withdraw(args.mm, args.pmdp);
    }
#[no_mangle]
unsafe extern "C" fn pmd_leaf_tests(args: *mut pgtable_debug_args)  {
    let mut pmd;
    if (!has_transparent_hugepage()) {
    return;
    }
    pr_debug!("Validating PMD leaf\n");
    pmd = pfn_pmd(args.fixed_pmd_pfn, args.page_prot);
//
// PMD based THP is a leaf entry.
//
    pmd = pmd_mkhuge(pmd);
    WARN_ON!(!pmd_leaf(pmd));
    }

#[no_mangle]
unsafe extern "C" fn pud_basic_tests(args: *mut pgtable_debug_args, idx: c_int)  {
pub static mut prot: pgprot_t = 0;
pub static mut val: c_ulong = 0;
    let mut pud;
    if (!has_transparent_pud_hugepage()) {
    return;
    }
    pr_debug!("Validating PUD basic (%pGv)\n", ptr);
    pud = pfn_pud(args.fixed_pud_pfn, prot);
//
// This test needs to be executed after the given page table entry
// is created with pfn_pud() to make sure that vm_get_page_prot(idx)
// does not have the dirty bit enabled from the beginning. This is
// important for platforms like arm64 where (!PTE_RDONLY) indicate
// dirty bit being set.
//
    WARN_ON!(pud_dirty(pud_wrprotect(pud)));
    WARN_ON!(!pud_same(pud, pud));
    WARN_ON!(!pud_young(pud_mkyoung(pud_mkold(pud))));
    WARN_ON!(!pud_dirty(pud_mkdirty(pud_mkclean(pud))));
    WARN_ON!(pud_dirty(pud_mkclean(pud_mkdirty(pud))));
    WARN_ON!(!pud_write(pud_mkwrite(pud_wrprotect(pud))));
    WARN_ON!(pud_write(pud_wrprotect(pud_mkwrite(pud))));
    WARN_ON!(pud_young(pud_mkold(pud_mkyoung(pud))));
    WARN_ON!(pud_dirty(pud_wrprotect(pud_mkclean(pud))));
    WARN_ON!(!pud_dirty(pud_wrprotect(pud_mkdirty(pud))));
    if (mm_pmd_folded(args.mm)) {
    return;
    }
//
// A huge page does not point to next level page table
// entry. Hence this must qualify as pud_bad().
//
    WARN_ON!(!pud_bad(pud_mkhuge(pud)));
    }
#[no_mangle]
unsafe extern "C" fn pud_advanced_tests(args: *mut pgtable_debug_args)  {
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut vaddr: c_ulong = 0;
    let mut pud;
    if (!has_transparent_pud_hugepage()) {
    return;
    }
    page = (args.pud_pfn != ULONG_MAX) ? pfn_to_page(args.pud_pfn) : core::ptr::null_mut();
    if (!page) {
    return;
    }
//
// flush_dcache_page() is called after set_pud_at() to clear
// PG_arch_1 for the page on ARM64. The page flag isn't cleared
// when it's released and page allocation check will fail when
// the page is allocated again. For architectures other than ARM64,
// the unexpected overhead of cache flushing is acceptable.
//
    pr_debug!("Validating PUD advanced\n");
// Align the address wrt HPAGE_PUD_SIZE
    vaddr &= HPAGE_PUD_MASK;
    pud = pfn_pud(args.pud_pfn, args.page_prot);
    set_pud_at(args.mm, vaddr, args.pudp, pud);
    flush_dcache_page(page);
    pudp_set_wrprotect(args.mm, vaddr, args.pudp);
    pud = pudp_get(args.pudp);
    WARN_ON!(pud_write(pud));

    pudp_huge_get_and_clear(args.mm, vaddr, args.pudp);
    pud = pudp_get(args.pudp);
    WARN_ON!(!pud_none(pud));

    pud = pfn_pud(args.pud_pfn, args.page_prot);
    pud = pud_wrprotect(pud);
    pud = pud_mkclean(pud);
    set_pud_at(args.mm, vaddr, args.pudp, pud);
    flush_dcache_page(page);
    pud = pud_mkwrite(pud);
    pud = pud_mkdirty(pud);
    pudp_set_access_flags(args.vma, vaddr, args.pudp, pud, 1);
    pud = pudp_get(args.pudp);
    WARN_ON!(!(pud_write(pud) && pud_dirty(pud)));

    pudp_huge_get_and_clear_full(args.vma, vaddr, args.pudp, 1);
    pud = pudp_get(args.pudp);
    WARN_ON!(!pud_none(pud));

    pud = pfn_pud(args.pud_pfn, args.page_prot);
    pud = pud_mkyoung(pud);
    set_pud_at(args.mm, vaddr, args.pudp, pud);
    flush_dcache_page(page);
    pudp_test_and_clear_young(args.vma, vaddr, args.pudp);
    pud = pudp_get(args.pudp);
    WARN_ON!(pud_young(pud));
    pudp_huge_get_and_clear(args.mm, vaddr, args.pudp);
    }
#[no_mangle]
unsafe extern "C" fn pud_leaf_tests(args: *mut pgtable_debug_args)  {
    let mut pud;
    if (!has_transparent_pud_hugepage()) {
    return;
    }
    pr_debug!("Validating PUD leaf\n");
    pud = pfn_pud(args.fixed_pud_pfn, args.page_prot);
//
// PUD based THP is a leaf entry.
//
    pud = pud_mkhuge(pud);
    WARN_ON!(!pud_leaf(pud));
    }

    static void __init pud_basic_tests(pgtable_debug_args *args, int idx) { }
    static void __init pud_advanced_tests(pgtable_debug_args *args) { }
    static void __init pud_leaf_tests(pgtable_debug_args *args) { }

    static void __init pmd_basic_tests(pgtable_debug_args *args, int idx) { }
    static void __init pud_basic_tests(pgtable_debug_args *args, int idx) { }
    static void __init pmd_advanced_tests(pgtable_debug_args *args) { }
    static void __init pud_advanced_tests(pgtable_debug_args *args) { }
    static void __init pmd_leaf_tests(pgtable_debug_args *args) { }
    static void __init pud_leaf_tests(pgtable_debug_args *args) { }

#[no_mangle]
unsafe extern "C" fn pmd_huge_tests(args: *mut pgtable_debug_args)  {
    let mut pmd;
    if (!arch_vmap_pmd_supported(args.page_prot) ||
    args.fixed_alignment < PMD_SIZE) {
    return;
    }
    pr_debug!("Validating PMD huge\n");
//
// X86 defined pmd_set_huge() verifies that the given
// PMD is not a populated non-leaf entry.
//
    pmd_clear(args.pmdp);
    WARN_ON!(!pmd_set_huge(args.pmdp, __pfn_to_phys(args.fixed_pmd_pfn), args.page_prot));
    WARN_ON!(!pmd_clear_huge(args.pmdp));
    pmd = pmdp_get(args.pmdp);
    WARN_ON!(!pmd_none(pmd));
    }
#[no_mangle]
unsafe extern "C" fn pud_huge_tests(args: *mut pgtable_debug_args)  {
    let mut pud;
    if (!arch_vmap_pud_supported(args.page_prot) ||
    args.fixed_alignment < PUD_SIZE) {
    return;
    }
    pr_debug!("Validating PUD huge\n");
//
// X86 defined pud_set_huge() verifies that the given
// PUD is not a populated non-leaf entry.
//
    pud_clear(args.pudp);
    WARN_ON!(!pud_set_huge(args.pudp, __pfn_to_phys(args.fixed_pud_pfn), args.page_prot));
    WARN_ON!(!pud_clear_huge(args.pudp));
    pud = pudp_get(args.pudp);
    WARN_ON!(!pud_none(pud));
    }

    static void __init pmd_huge_tests(pgtable_debug_args *args) { }
    static void __init pud_huge_tests(pgtable_debug_args *args) { }

#[no_mangle]
unsafe extern "C" fn p4d_basic_tests(args: *mut pgtable_debug_args)  {
    let mut p4d;
    pr_debug!("Validating P4D basic\n");
    memset(&p4d, RANDOM_NZVALUE, sizeof!(p4d_t));
    WARN_ON!(!p4d_same(p4d, p4d));
    }
#[no_mangle]
unsafe extern "C" fn pgd_basic_tests(args: *mut pgtable_debug_args)  {
    let mut pgd;
    pr_debug!("Validating PGD basic\n");
    memset(&pgd, RANDOM_NZVALUE, sizeof!(pgd_t));
    WARN_ON!(!pgd_same(pgd, pgd));
    }

#[no_mangle]
unsafe extern "C" fn pud_clear_tests(args: *mut pgtable_debug_args)  {
pub static mut pud: pud_t = 0;
    if (mm_pmd_folded(args.mm)) {
    return;
    }
    pr_debug!("Validating PUD clear\n");
    WARN_ON!(pud_none(pud));
    pud_clear(args.pudp);
    pud = pudp_get(args.pudp);
    WARN_ON!(!pud_none(pud));
    }
#[no_mangle]
unsafe extern "C" fn pud_populate_tests(args: *mut pgtable_debug_args)  {
    let mut pud;
    if (mm_pmd_folded(args.mm)) {
    return;
    }
    pr_debug!("Validating PUD populate\n");
//
// This entry points to next level page table page.
// Hence this must not qualify as pud_bad().
//
    pud_populate(args.mm, args.pudp, args.start_pmdp);
    pud = pudp_get(args.pudp);
    WARN_ON!(pud_bad(pud));
    }

    static void __init pud_clear_tests(pgtable_debug_args *args) { }
    static void __init pud_populate_tests(pgtable_debug_args *args) { }

#[no_mangle]
unsafe extern "C" fn p4d_clear_tests(args: *mut pgtable_debug_args)  {
pub static mut p4d: p4d_t = 0;
    if (mm_pud_folded(args.mm)) {
    return;
    }
    pr_debug!("Validating P4D clear\n");
    WARN_ON!(p4d_none(p4d));
    p4d_clear(args.p4dp);
    p4d = p4dp_get(args.p4dp);
    WARN_ON!(!p4d_none(p4d));
    }
#[no_mangle]
unsafe extern "C" fn p4d_populate_tests(args: *mut pgtable_debug_args)  {
    let mut p4d;
    if (mm_pud_folded(args.mm)) {
    return;
    }
    pr_debug!("Validating P4D populate\n");
//
// This entry points to next level page table page.
// Hence this must not qualify as p4d_bad().
//
    pud_clear(args.pudp);
    p4d_clear(args.p4dp);
    p4d_populate(args.mm, args.p4dp, args.start_pudp);
    p4d = p4dp_get(args.p4dp);
    WARN_ON!(p4d_bad(p4d));
    }
#[no_mangle]
unsafe extern "C" fn pgd_clear_tests(args: *mut pgtable_debug_args)  {
pub static mut pgd: pgd_t = 0;
    if (mm_p4d_folded(args.mm)) {
    return;
    }
    pr_debug!("Validating PGD clear\n");
    WARN_ON!(pgd_none(pgd));
    pgd_clear(args.pgdp);
    pgd = pgdp_get(args.pgdp);
    WARN_ON!(!pgd_none(pgd));
    }
#[no_mangle]
unsafe extern "C" fn pgd_populate_tests(args: *mut pgtable_debug_args)  {
    let mut pgd;
    if (mm_p4d_folded(args.mm)) {
    return;
    }
    pr_debug!("Validating PGD populate\n");
//
// This entry points to next level page table page.
// Hence this must not qualify as pgd_bad().
//
    p4d_clear(args.p4dp);
    pgd_clear(args.pgdp);
    pgd_populate(args.mm, args.pgdp, args.start_p4dp);
    pgd = pgdp_get(args.pgdp);
    WARN_ON!(pgd_bad(pgd));
    }

    static void __init p4d_clear_tests(pgtable_debug_args *args) { }
    static void __init pgd_clear_tests(pgtable_debug_args *args) { }
    static void __init p4d_populate_tests(pgtable_debug_args *args) { }
    static void __init pgd_populate_tests(pgtable_debug_args *args) { }

#[no_mangle]
unsafe extern "C" fn pte_clear_tests(args: *mut pgtable_debug_args)  {
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut pte: pte_t = 0;
    page = (args.pte_pfn != ULONG_MAX) ? pfn_to_page(args.pte_pfn) : core::ptr::null_mut();
    if (!page) {
    return;
    }
//
// flush_dcache_page() is called after set_pte_at() to clear
// PG_arch_1 for the page on ARM64. The page flag isn't cleared
// when it's released and page allocation check will fail when
// the page is allocated again. For architectures other than ARM64,
// the unexpected overhead of cache flushing is acceptable.
//
    pr_debug!("Validating PTE clear\n");
    if (WARN_ON!(!args.ptep)) {
    return;
    }
    set_pte_at(args.mm, args.vaddr, args.ptep, pte);
    WARN_ON!(pte_none(pte));
    flush_dcache_page(page);
    barrier();
    ptep_clear(args.mm, args.vaddr, args.ptep);
    pte = ptep_get(args.ptep);
    WARN_ON!(!pte_none(pte));
    }
#[no_mangle]
unsafe extern "C" fn pmd_clear_tests(args: *mut pgtable_debug_args)  {
pub static mut pmd: pmd_t = 0;
    pr_debug!("Validating PMD clear\n");
    WARN_ON!(pmd_none(pmd));
    pmd_clear(args.pmdp);
    pmd = pmdp_get(args.pmdp);
    WARN_ON!(!pmd_none(pmd));
    }
#[no_mangle]
unsafe extern "C" fn pmd_populate_tests(args: *mut pgtable_debug_args)  {
    let mut pmd;
    pr_debug!("Validating PMD populate\n");
//
// This entry points to next level page table page.
// Hence this must not qualify as pmd_bad().
//
    pmd_populate(args.mm, args.pmdp, args.start_ptep);
    pmd = pmdp_get(args.pmdp);
    WARN_ON!(pmd_bad(pmd));
    }
#[no_mangle]
unsafe extern "C" fn pte_special_tests(args: *mut pgtable_debug_args)  {
pub static mut pte: pte_t = 0;
    if (!IS_ENABLED!(CONFIG_ARCH_HAS_PTE_SPECIAL)) {
    return;
    }
    pr_debug!("Validating PTE special\n");
    WARN_ON!(!pte_special(pte_mkspecial(pte)));
    }
#[no_mangle]
unsafe extern "C" fn pte_protnone_tests(args: *mut pgtable_debug_args)  {
pub static mut pte: pte_t = 0;
    if (!IS_ENABLED!(CONFIG_ARCH_HAS_PTE_PROTNONE)) {
    return;
    }
    pr_debug!("Validating PTE protnone\n");
    WARN_ON!(!pte_protnone(pte));
    WARN_ON!(!pte_present(pte));
    }

#[no_mangle]
unsafe extern "C" fn pmd_protnone_tests(args: *mut pgtable_debug_args)  {
    let mut pmd;
    if (!IS_ENABLED!(CONFIG_ARCH_HAS_PTE_PROTNONE)) {
    return;
    }
    if (!has_transparent_hugepage()) {
    return;
    }
    pr_debug!("Validating PMD protnone\n");
    pmd = pmd_mkhuge(pfn_pmd(args.fixed_pmd_pfn, args.page_prot_none));
    WARN_ON!(!pmd_protnone(pmd));
    WARN_ON!(!pmd_present(pmd));
    }

    static void __init pmd_protnone_tests(pgtable_debug_args *args) { }

#[no_mangle]
unsafe extern "C" fn pte_soft_dirty_tests(args: *mut pgtable_debug_args)  {
pub static mut pte: pte_t = 0;
    if (!pgtable_supports_soft_dirty()) {
    return;
    }
    pr_debug!("Validating PTE soft dirty\n");
    WARN_ON!(!pte_soft_dirty(pte_mksoft_dirty(pte)));
    WARN_ON!(pte_soft_dirty(pte_clear_soft_dirty(pte)));
    }
#[no_mangle]
unsafe extern "C" fn pte_swap_soft_dirty_tests(args: *mut pgtable_debug_args)  {
    let mut pte;
    let mut entry;
    if (!pgtable_supports_soft_dirty()) {
    return;
    }
    pr_debug!("Validating PTE swap soft dirty\n");
    pte = swp_entry_to_pte(args.swp_entry);
    entry = softleaf_from_pte(pte);
    WARN_ON!(!softleaf_is_swap(entry));
    WARN_ON!(!pte_swp_soft_dirty(pte_swp_mksoft_dirty(pte)));
    WARN_ON!(pte_swp_soft_dirty(pte_swp_clear_soft_dirty(pte)));
    }

#[no_mangle]
unsafe extern "C" fn pmd_soft_dirty_tests(args: *mut pgtable_debug_args)  {
    let mut pmd;
    if (!pgtable_supports_soft_dirty()) {
    return;
    }
    if (!has_transparent_hugepage()) {
    return;
    }
    pr_debug!("Validating PMD soft dirty\n");
    pmd = pfn_pmd(args.fixed_pmd_pfn, args.page_prot);
    WARN_ON!(!pmd_soft_dirty(pmd_mksoft_dirty(pmd)));
    WARN_ON!(pmd_soft_dirty(pmd_clear_soft_dirty(pmd)));
    }
#[no_mangle]
unsafe extern "C" fn pmd_leaf_soft_dirty_tests(args: *mut pgtable_debug_args)  {
    let mut pmd;
    if (!pgtable_supports_soft_dirty() ||
    !IS_ENABLED!(CONFIG_ARCH_HAS_PMD_SOFTLEAVES)) {
    return;
    }
    if (!has_transparent_hugepage()) {
    return;
    }
    pr_debug!("Validating PMD swap soft dirty\n");
    pmd = softleaf_to_pmd(args.leaf_entry);
    WARN_ON!(!pmd_is_huge(pmd));
    WARN_ON!(!pmd_is_valid_softleaf(pmd));
    WARN_ON!(!pmd_swp_soft_dirty(pmd_swp_mksoft_dirty(pmd)));
    WARN_ON!(pmd_swp_soft_dirty(pmd_swp_clear_soft_dirty(pmd)));
    }

    static void __init pmd_soft_dirty_tests(pgtable_debug_args *args) { }
    static void __init pmd_leaf_soft_dirty_tests(pgtable_debug_args *args) { }

#[no_mangle]
unsafe extern "C" fn pte_swap_exclusive_tests(args: *mut pgtable_debug_args)  {
    let mut entry;
    let mut softleaf;
    let mut pte;
    pr_debug!("Validating PTE swap exclusive\n");
    entry = args.swp_entry;
    pte = swp_entry_to_pte(entry);
    softleaf = softleaf_from_pte(pte);
    WARN_ON!(pte_swp_exclusive(pte));
    WARN_ON!(!softleaf_is_swap(softleaf));
    WARN_ON!(memcmp(&entry, &softleaf, sizeof!(entry)));
    pte = pte_swp_mkexclusive(pte);
    softleaf = softleaf_from_pte(pte);
    WARN_ON!(!pte_swp_exclusive(pte));
    WARN_ON!(!softleaf_is_swap(softleaf));
    WARN_ON!(pte_swp_soft_dirty(pte));
    WARN_ON!(memcmp(&entry, &softleaf, sizeof!(entry)));
    pte = pte_swp_clear_exclusive(pte);
    softleaf = softleaf_from_pte(pte);
    WARN_ON!(pte_swp_exclusive(pte));
    WARN_ON!(!softleaf_is_swap(softleaf));
    WARN_ON!(memcmp(&entry, &softleaf, sizeof!(entry)));
    }
#[no_mangle]
unsafe extern "C" fn pte_swap_tests(args: *mut pgtable_debug_args)  {
    let mut arch_entry;
    let mut entry;
    pte_t pte1, pte2;
    pr_debug!("Validating PTE swap\n");
    pte1 = swp_entry_to_pte(args.swp_entry);
    entry = softleaf_from_pte(pte1);
    WARN_ON!(!softleaf_is_swap(entry));
    arch_entry = __pte_to_swp_entry(pte1);
    pte2 = __swp_entry_to_pte(arch_entry);
    WARN_ON!(memcmp(&pte1, &pte2, sizeof!(pte1)));
    }

#[no_mangle]
unsafe extern "C" fn pmd_softleaf_tests(args: *mut pgtable_debug_args)  {
    let mut arch_entry;
    pmd_t pmd1, pmd2;
    if (!has_transparent_hugepage()) {
    return;
    }
    pr_debug!("Validating PMD swap\n");
    pmd1 = softleaf_to_pmd(args.leaf_entry);
    WARN_ON!(!pmd_is_huge(pmd1));
    WARN_ON!(!pmd_is_valid_softleaf(pmd1));
    arch_entry = __pmd_to_swp_entry(pmd1);
    pmd2 = __swp_entry_to_pmd(arch_entry);
    WARN_ON!(memcmp(&pmd1, &pmd2, sizeof!(pmd1)));
    }

    static void __init pmd_softleaf_tests(pgtable_debug_args *args) { }

#[no_mangle]
unsafe extern "C" fn swap_migration_tests(args: *mut pgtable_debug_args)  {
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut entry;
    if (!IS_ENABLED!(CONFIG_MIGRATION)) {
    return;
    }
//
// swap_migration_tests() requires a dedicated page as it needs to
// be locked before creating a migration entry from it. Locking the
// page that actually maps kernel text ('start_kernel') can be real
// problematic. Lets use the allocated page explicitly for this
// purpose.
//
    page = (args.pte_pfn != ULONG_MAX) ? pfn_to_page(args.pte_pfn) : core::ptr::null_mut();
    if (!page) {
    return;
    }
    pr_debug!("Validating swap migration\n");
//
// make_[readable|writable]_migration_entry() expects given page to
// be locked, otherwise it stumbles upon a BUG_ON!().
//
    __SetPageLocked(page);
    entry = make_writable_migration_entry(page_to_pfn(page));
    WARN_ON!(!softleaf_is_migration(entry));
    WARN_ON!(!softleaf_is_migration_write(entry));
    entry = make_readable_migration_entry(swp_offset(entry));
    WARN_ON!(!softleaf_is_migration(entry));
    WARN_ON!(softleaf_is_migration_write(entry));
    entry = make_readable_migration_entry(page_to_pfn(page));
    WARN_ON!(!softleaf_is_migration(entry));
    WARN_ON!(softleaf_is_migration_write(entry));
    __ClearPageLocked(page);
    }

#[no_mangle]
unsafe extern "C" fn hugetlb_basic_tests(args: *mut pgtable_debug_args)  {
    let mut pte;
    pr_debug!("Validating HugeTLB basic\n");
    pte = pfn_pte(args.fixed_pmd_pfn, args.page_prot);
    pte = arch_make_huge_pte(pte, PMD_SHIFT, VM_ACCESS_FLAGS);

    WARN_ON!(!pte_huge(pte));

    WARN_ON!(!huge_pte_dirty(huge_pte_mkdirty(pte)));
    WARN_ON!(!huge_pte_write(huge_pte_mkwrite(huge_pte_wrprotect(pte))));
    WARN_ON!(huge_pte_write(huge_pte_wrprotect(huge_pte_mkwrite(pte))));
    }

    static void __init hugetlb_basic_tests(pgtable_debug_args *args) { }

#[no_mangle]
unsafe extern "C" fn pmd_thp_tests(args: *mut pgtable_debug_args)  {
    let mut pmd;
    if (!has_transparent_hugepage()) {
    return;
    }
    pr_debug!("Validating PMD based THP\n");
//
// pmd_trans_huge() and pmd_present() must return positive after
// MMU invalidation with pmd_mkinvalid(). This behavior is an
// optimization for transparent huge page. pmd_trans_huge() must
// be true if pmd_page() returns a valid THP to avoid taking the
// pmd_lock when others walk over non transhuge pmds (i.e. there
// are no THP allocated). Especially when splitting a THP and
// removing the present bit from the pmd, pmd_trans_huge() still
// needs to return true. pmd_present() should be true whenever
// pmd_trans_huge() returns true.
//
    pmd = pfn_pmd(args.fixed_pmd_pfn, args.page_prot);
    WARN_ON!(!pmd_trans_huge(pmd_mkhuge(pmd)));

    WARN_ON!(!pmd_trans_huge(pmd_mkinvalid(pmd_mkhuge(pmd))));
    WARN_ON!(!pmd_present(pmd_mkinvalid(pmd_mkhuge(pmd))));
    WARN_ON!(!pmd_leaf(pmd_mkinvalid(pmd_mkhuge(pmd))));

    }

#[no_mangle]
unsafe extern "C" fn pud_thp_tests(args: *mut pgtable_debug_args)  {
    let mut pud;
    if (!has_transparent_pud_hugepage()) {
    return;
    }
    pr_debug!("Validating PUD based THP\n");
    pud = pfn_pud(args.fixed_pud_pfn, args.page_prot);
    WARN_ON!(!pud_trans_huge(pud_mkhuge(pud)));
//
// pud_mkinvalid() has been dropped for now. Enable back
// these tests when it comes back with a modified pud_present().
//
// WARN_ON!(!pud_trans_huge(pud_mkinvalid(pud_mkhuge(pud))));
// WARN_ON!(!pud_present(pud_mkinvalid(pud_mkhuge(pud))));
//
    }

    static void __init pud_thp_tests(pgtable_debug_args *args) { }

    static void __init pmd_thp_tests(pgtable_debug_args *args) { }
    static void __init pud_thp_tests(pgtable_debug_args *args) { }

#[no_mangle]
unsafe extern "C" fn get_random_vaddr() -> unsigned long __init {
    unsigned long random_vaddr, random_pages, total_user_pages;
    total_user_pages = (TASK_SIZE - FIRST_USER_ADDRESS) / PAGE_SIZE;
    random_pages = get_random_long() % total_user_pages;
    random_vaddr = FIRST_USER_ADDRESS + random_pages * PAGE_SIZE;
    return random_vaddr;
    }
    static void __init
    debug_vm_pgtable_free_huge_page(pgtable_debug_args *args,
    unsigned long pfn, int order)
    {

    if (args.is_contiguous_page) {
    free_contig_range(pfn, 1 << order);
    return;
    }

    __free_pages(pfn_to_page(pfn), order);
    }
#[no_mangle]
unsafe extern "C" fn destroy_args(args: *mut pgtable_debug_args)  {
// Free (huge) page
    if (IS_ENABLED!(CONFIG_TRANSPARENT_HUGEPAGE) &&
    has_transparent_pud_hugepage() &&
    args.pud_pfn != ULONG_MAX) {
    debug_vm_pgtable_free_huge_page(args, args.pud_pfn, HPAGE_PUD_ORDER);
    args.pud_pfn = ULONG_MAX;
    args.pmd_pfn = ULONG_MAX;
    args.pte_pfn = ULONG_MAX;
    }
    if (IS_ENABLED!(CONFIG_TRANSPARENT_HUGEPAGE) &&
    has_transparent_hugepage() &&
    args.pmd_pfn != ULONG_MAX) {
    debug_vm_pgtable_free_huge_page(args, args.pmd_pfn, HPAGE_PMD_ORDER);
    args.pmd_pfn = ULONG_MAX;
    args.pte_pfn = ULONG_MAX;
    }
    if (args.pte_pfn != ULONG_MAX) {
    __free_page(pfn_to_page(args.pte_pfn));
    args.pte_pfn = ULONG_MAX;
    }
// Free page table entries
    if (args.start_ptep) {
    pmd_clear(args.pmdp);
    pte_free(args.mm, args.start_ptep);
    mm_dec_nr_ptes(args.mm);
    }
    if (args.start_pmdp) {
    pud_clear(args.pudp);
    pmd_free(args.mm, args.start_pmdp);
    mm_dec_nr_pmds(args.mm);
    }
    if (args.start_pudp) {
    p4d_clear(args.p4dp);
    pud_free(args.mm, args.start_pudp);
    mm_dec_nr_puds(args.mm);
    }
    if (args.start_p4dp) {
    pgd_clear(args.pgdp);
    p4d_free(args.mm, args.start_p4dp);
    }
// Free vma and mm struct
    if (args.vma) {
    vm_area_free(args.vma);
    }
    if (args.mm) {
    mmput(args.mm);
    }
    }
    static struct page * __init
    debug_vm_pgtable_alloc_huge_page(pgtable_debug_args *args, int order)
    {
    let mut page = core::ptr::null_mut();

    if (order > MAX_PAGE_ORDER) {
    page = alloc_contig_pages((1 << order), GFP_KERNEL,
    first_online_node, core::ptr::null_mut());
    if (page) {
    args.is_contiguous_page = true;
    return page;
    }
    }

    if (order <= MAX_PAGE_ORDER) {
    page = alloc_pages(GFP_KERNEL, order);
    }
    return page;
    }
//
// Check if a physical memory range described by <pstart, pend> contains
// an area that is of size psize, and aligned to psize.
//
// Don't use address 0, an all-zeroes physical address might mask bugs, and
// it's not used on x86.
//
    static void  __init phys_align_check(phys_addr_t pstart,
    phys_addr_t pend, unsigned long psize,
    phys_addr_t *physp, unsigned long *alignp)
    {
    phys_addr_t aligned_start, aligned_end;
    if (pstart == 0) {
    pstart = PAGE_SIZE;
    }
    aligned_start = ALIGN(pstart, psize);
    aligned_end = aligned_start + psize;
    if (aligned_end > aligned_start && aligned_end <= pend) {
// alignp = psize;
// physp = aligned_start;
    }
    }
#[no_mangle]
unsafe extern "C" fn init_fixed_pfns(args: *mut pgtable_debug_args)  {
    let mut idx = 0;
    phys_addr_t phys, pstart, pend;
//
// Initialize the fixed pfns. To do this, try to find a
// valid physical range, preferably aligned to PUD_SIZE,
// but settling for aligned to PMD_SIZE as a fallback. If
// neither of those is found, use the physical address of
// the start_kernel symbol.
//
// The memory doesn't need to be allocated, it just needs to exist
// as usable memory. It won't be touched.
//
// The alignment is recorded, and can be checked to see if we
// can run the tests that require an actual valid physical
// address range on some architectures ({pmd,pud}_huge_test
// on x86).
//
    phys = __pa_symbol(&start_kernel);
    args.fixed_alignment = PAGE_SIZE;
    for_each_mem_range(idx, &pstart, &pend) {
// First check for a PUD-aligned area
    phys_align_check(pstart, pend, PUD_SIZE, &phys,
    &args.fixed_alignment);
// If a PUD-aligned area is found, we're done
    if (args.fixed_alignment == PUD_SIZE) {
    break;
    }
//
// If no PMD-aligned area found yet, check for one,
// but continue the loop to look for a PUD-aligned area.
//
    if (args.fixed_alignment < PMD_SIZE) {
    phys_align_check(pstart, pend, PMD_SIZE, &phys,
    &args.fixed_alignment);
    }
    }
    args.fixed_pgd_pfn = __phys_to_pfn(phys & PGDIR_MASK);
    args.fixed_p4d_pfn = __phys_to_pfn(phys & P4D_MASK);
    args.fixed_pud_pfn = __phys_to_pfn(phys & PUD_MASK);
    args.fixed_pmd_pfn = __phys_to_pfn(phys & PMD_MASK);
    args.fixed_pte_pfn = __phys_to_pfn(phys & PAGE_MASK);
    WARN_ON!(!pfn_valid(args.fixed_pte_pfn));
    }
#[no_mangle]
unsafe extern "C" fn init_args(args: *mut pgtable_debug_args) -> c_int {
    let mut max_swap_offset = 0;
    let mut page = core::ptr::null_mut();
pub static mut ret: c_int = 0;
//
// Initialize the debugging data.
//
// vm_get_page_prot(VM_NONE) or vm_get_page_prot(VM_SHARED|VM_NONE)
// will help create page table entries with PROT_NONE permission as
// required for pxx_protnone_tests().
//
    memset(args, 0, sizeof!(*args));
    args.vaddr              = get_random_vaddr();
    args.page_prot          = vm_get_page_prot(VM_ACCESS_FLAGS);
    args.page_prot_none     = vm_get_page_prot(VM_NONE);
    args.is_contiguous_page = false;
    args.pud_pfn            = ULONG_MAX;
    args.pmd_pfn            = ULONG_MAX;
    args.pte_pfn            = ULONG_MAX;
    args.fixed_pgd_pfn      = ULONG_MAX;
    args.fixed_p4d_pfn      = ULONG_MAX;
    args.fixed_pud_pfn      = ULONG_MAX;
    args.fixed_pmd_pfn      = ULONG_MAX;
    args.fixed_pte_pfn      = ULONG_MAX;
// Allocate mm and vma
    args.mm = mm_alloc();
    if (!args.mm) {
    pr_err!("Failed to allocate mm struct\n");
    ret = -ENOMEM;
// goto;
    }
    args.vma = vm_area_alloc(args.mm);
    if (!args.vma) {
    pr_err!("Failed to allocate vma\n");
    ret = -ENOMEM;
// goto;
    }
//
// Allocate page table entries. They will be modified in the tests.
// Lets save the page table entries so that they can be released
// when the tests are completed.
//
    args.pgdp = pgd_offset(args.mm, args.vaddr);
    args.p4dp = p4d_alloc(args.mm, args.pgdp, args.vaddr);
    if (!args.p4dp) {
    pr_err!("Failed to allocate p4d entries\n");
    ret = -ENOMEM;
// goto;
    }
    args.start_p4dp = p4d_offset(args.pgdp, 0UL);
    WARN_ON!(!args.start_p4dp);
    args.pudp = pud_alloc(args.mm, args.p4dp, args.vaddr);
    if (!args.pudp) {
    pr_err!("Failed to allocate pud entries\n");
    ret = -ENOMEM;
// goto;
    }
    args.start_pudp = pud_offset(args.p4dp, 0UL);
    WARN_ON!(!args.start_pudp);
    args.pmdp = pmd_alloc(args.mm, args.pudp, args.vaddr);
    if (!args.pmdp) {
    pr_err!("Failed to allocate pmd entries\n");
    ret = -ENOMEM;
// goto;
    }
    args.start_pmdp = pmd_offset(args.pudp, 0UL);
    WARN_ON!(!args.start_pmdp);
    if (pte_alloc(args.mm, args.pmdp)) {
    pr_err!("Failed to allocate pte entries\n");
    ret = -ENOMEM;
// goto;
    }
    args.start_ptep = pmd_pgtable(pmdp_get(args.pmdp));
    WARN_ON!(!args.start_ptep);
    init_fixed_pfns(args);
// See generic_max_swapfile_size(): probe the maximum offset
    max_swap_offset = swp_offset(softleaf_from_pte(softleaf_to_pte(swp_entry(0, ~0UL))));
// Create a swp entry with all possible bits set while still being swap.
    args.swp_entry = swp_entry(MAX_SWAPFILES - 1, max_swap_offset);
// Create a non-present migration entry.
    args.leaf_entry = make_writable_migration_entry(~0UL);
//
// Allocate (huge) pages because some of the tests need to access
// the data in the pages. The corresponding tests will be skipped
// if we fail to allocate (huge) pages.
//
    if (IS_ENABLED!(CONFIG_TRANSPARENT_HUGEPAGE) &&
    has_transparent_pud_hugepage()) {
    page = debug_vm_pgtable_alloc_huge_page(args, HPAGE_PUD_ORDER);
    if (page) {
    args.pud_pfn = page_to_pfn(page);
    args.pmd_pfn = args.pud_pfn;
    args.pte_pfn = args.pud_pfn;
    return 0;
    }
    }
    if (IS_ENABLED!(CONFIG_TRANSPARENT_HUGEPAGE) &&
    has_transparent_hugepage()) {
    page = debug_vm_pgtable_alloc_huge_page(args, HPAGE_PMD_ORDER);
    if (page) {
    args.pmd_pfn = page_to_pfn(page);
    args.pte_pfn = args.pmd_pfn;
    return 0;
    }
    }
    page = alloc_page(GFP_KERNEL);
    if (page) {
    args.pte_pfn = page_to_pfn(page);
    }
    return 0;
// label;
    destroy_args(args);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn debug_vm_pgtable() -> c_int {
pub static mut args: usize = 0;
    let mut ptl = core::ptr::null_mut();
    let mut idx = 0;
    let mut ret = 0;
    pr_info!("Validating architecture page table helpers\n");
    ret = init_args(&args);
    if (ret) {
    return ret;
    }
//
// Iterate over each possible vm_flags to make sure that all
// the basic page table transformation validations just hold
// true irrespective of the starting protection value for a
// given page table entry.
//
// Protection based vm_flags combinations are always linear
// and increasing i.e starting from VM_NONE and going up to
// (VM_SHARED | READ | WRITE | EXEC).
//

    while (idx <= VM_FLAGS_END) {
    pte_basic_tests(&args, idx);
    pmd_basic_tests(&args, idx);
    pud_basic_tests(&args, idx);
    }
//
// Both P4D and PGD level tests are very basic which do not
// involve creating page table entries from the protection
// value and the given pfn. Hence just keep them out from
// the above iteration for now to save some test execution
// time.
//
    p4d_basic_tests(&args);
    pgd_basic_tests(&args);
    pmd_leaf_tests(&args);
    pud_leaf_tests(&args);
    pte_special_tests(&args);
    pte_protnone_tests(&args);
    pmd_protnone_tests(&args);
    pte_soft_dirty_tests(&args);
    pmd_soft_dirty_tests(&args);
    pte_swap_soft_dirty_tests(&args);
    pmd_leaf_soft_dirty_tests(&args);
    pte_swap_exclusive_tests(&args);
    pte_swap_tests(&args);
    pmd_softleaf_tests(&args);
    swap_migration_tests(&args);
    pmd_thp_tests(&args);
    pud_thp_tests(&args);
    hugetlb_basic_tests(&args);
//
// Page table modifying tests. They need to hold
// proper page table lock.
//
    args.ptep = pte_offset_map_lock(args.mm, args.pmdp, args.vaddr, &ptl);
    pte_clear_tests(&args);
    pte_advanced_tests(&args);
    if (args.ptep) {
    pte_unmap_unlock(args.ptep, ptl);
    }
    ptl = pmd_lock(args.mm, args.pmdp);
    pmd_clear_tests(&args);
    pmd_advanced_tests(&args);
    pmd_huge_tests(&args);
    pmd_populate_tests(&args);
    spin_unlock(ptl);
    ptl = pud_lock(args.mm, args.pudp);
    pud_clear_tests(&args);
    pud_advanced_tests(&args);
    pud_huge_tests(&args);
    pud_populate_tests(&args);
    spin_unlock(ptl);
    spin_lock(&(args.mm.page_table_lock));
    p4d_clear_tests(&args);
    pgd_clear_tests(&args);
    p4d_populate_tests(&args);
    pgd_populate_tests(&args);
    spin_unlock(&(args.mm.page_table_lock));
    destroy_args(&args);
    return 0;
    }
    late_initcall!(debug_vm_pgtable);
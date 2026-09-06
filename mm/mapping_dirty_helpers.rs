//! Automatically rewritten from C to Rust
//! Source: mm/mapping_dirty_helpers.c
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
// struct wp_walk - Private struct for pagetable walk callbacks
// @range: Range for mmu notifiers
// @tlbflush_start: Address of first modified pte
// @tlbflush_end: Address of last modified pte + 1
// @total: Total number of modified ptes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wp_walk {
    pub range: mmu_notifier_range,
    pub tlbflush_start: c_ulong,
    pub tlbflush_end: c_ulong,
    pub total: c_ulong,
}

//
// wp_pte - Write-protect a pte
// @pte: Pointer to the pte
// @addr: The start of protecting virtual address
// @end: The end of protecting virtual address
// @walk: pagetable walk callback argument
//
// The function write-protects a pte and records the range in
// virtual address space of touched ptes for efficient range TLB flushes.
//
#[no_mangle]
pub unsafe extern "C" fn wp_pte(pte: *mut pte_t, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut wpwalk = walk.private;
pub static mut ptent: pte_t = 0;
    if (pte_write(ptent)) {
pub static mut old_pte: pte_t = 0;
    ptent = pte_wrprotect(old_pte);
    ptep_modify_prot_commit(walk.vma, addr, pte, old_pte, ptent);
    wpwalk.total += 1;
    wpwalk.tlbflush_start = min(wpwalk.tlbflush_start, addr);
    wpwalk.tlbflush_end = max(wpwalk.tlbflush_end,
    addr + PAGE_SIZE);
    }
    return 0;
    }
//
// struct clean_walk - Private struct for the clean_record_pte function.
// @base: wp_walk we derive from
// @bitmap_pgoff: Address_space Page offset of the first bit in @bitmap
// @bitmap: Bitmap with one bit for each page offset in the address_space range
// covered.
// @start: Address_space page offset of first modified pte relative
// to @bitmap_pgoff
// @end: Address_space page offset of last modified pte relative
// to @bitmap_pgoff
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clean_walk {
    pub base: wp_walk,
    pub bitmap_pgoff: pgoff_t,
    pub bitmap: *mut c_ulong,
    pub start: pgoff_t,
    pub end: pgoff_t,
}

//
// clean_record_pte - Clean a pte and record its address space offset in a
// bitmap
// @pte: Pointer to the pte
// @addr: The start of virtual address to be clean
// @end: The end of virtual address to be clean
// @walk: pagetable walk callback argument
//
// The function cleans a pte and records the range in
// virtual address space of touched ptes for efficient TLB flushes.
// It also records dirty ptes in a bitmap representing page offsets
// in the address_space, as well as the first and last of the bits
// touched.
//
#[no_mangle]
pub unsafe extern "C" fn clean_record_pte(pte: *mut pte_t, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut wpwalk = walk.private;
    let mut cwalk = to_clean_walk(wpwalk);
pub static mut ptent: pte_t = 0;
    if (pte_dirty(ptent)) {
    pgoff_t pgoff = ((addr - walk.vma.vm_start) >> PAGE_SHIFT) +
    vma_start_pgoff(walk.vma) - cwalk.bitmap_pgoff;
pub static mut old_pte: pte_t = 0;
    ptent = pte_mkclean(old_pte);
    ptep_modify_prot_commit(walk.vma, addr, pte, old_pte, ptent);
    wpwalk.total += 1;
    wpwalk.tlbflush_start = min(wpwalk.tlbflush_start, addr);
    wpwalk.tlbflush_end = max(wpwalk.tlbflush_end,
    addr + PAGE_SIZE);
    __set_bit(pgoff, cwalk.bitmap);
    cwalk.start = min(cwalk.start, pgoff);
    cwalk.end = max(cwalk.end, pgoff + 1);
    }
    return 0;
    }
//
// wp_clean_pmd_entry - The pagewalk pmd callback.
//
// Dirty-tracking should take place on the PTE level, so
// WARN() if encountering a dirty huge pmd.
// Furthermore, never split huge pmds, since that currently
// causes dirty info loss. The pagefault handler should do
// that if needed.
//
#[no_mangle]
pub unsafe extern "C" fn wp_clean_pmd_entry(pmd: *mut pmd_t, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut pmdval: pmd_t = 0;
// Do not split a huge pmd, present or migrated
    if (pmd_trans_huge(pmdval)) {
    WARN_ON!(pmd_write(pmdval) || pmd_dirty(pmdval));
    walk.action = ACTION_CONTINUE;
    }
    return 0;
    }
//
// wp_clean_pud_entry - The pagewalk pud callback.
//
// Dirty-tracking should take place on the PTE level, so
// WARN() if encountering a dirty huge puds.
// Furthermore, never split huge puds, since that currently
// causes dirty info loss. The pagefault handler should do
// that if needed.
//
#[no_mangle]
pub unsafe extern "C" fn wp_clean_pud_entry(pud: *mut pud_t, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {

pub static mut pudval: pud_t = 0;
// Do not split a huge pud
    if (pud_trans_huge(pudval)) {
    WARN_ON!(pud_write(pudval) || pud_dirty(pudval));
    walk.action = ACTION_CONTINUE;
    }

    return 0;
    }
//
// wp_clean_pre_vma - The pagewalk pre_vma callback.
//
// The pre_vma callback performs the cache flush, stages the tlb flush
// and calls the necessary mmu notifiers.
//
#[no_mangle]
pub unsafe extern "C" fn wp_clean_pre_vma(start: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut wpwalk = walk.private;
    wpwalk.tlbflush_start = end;
    wpwalk.tlbflush_end = start;
    mmu_notifier_range_init(&wpwalk.range, MMU_NOTIFY_PROTECTION_PAGE, 0,
    walk.mm, start, end);
    mmu_notifier_invalidate_range_start(&wpwalk.range);
    flush_cache_range(walk.vma, start, end);
//
// We're not using tlb_gather_mmu() since typically
// only a small subrange of PTEs are affected, whereas
// tlb_gather_mmu() records the full range.
//
    inc_tlb_flush_pending(walk.mm);
    return 0;
    }
//
// wp_clean_post_vma - The pagewalk post_vma callback.
//
// The post_vma callback performs the tlb flush and calls necessary mmu
// notifiers.
//
#[no_mangle]
unsafe extern "C" fn wp_clean_post_vma(walk: *mut mm_walk) {
    let mut wpwalk = walk.private;
    if (mm_tlb_flush_nested(walk.mm)) {
    flush_tlb_range(walk.vma, wpwalk.range.start,
    wpwalk.range.end);
    }

    else if (wpwalk.tlbflush_end > wpwalk.tlbflush_start) {
    flush_tlb_range(walk.vma, wpwalk.tlbflush_start,
    wpwalk.tlbflush_end);
    }
    mmu_notifier_invalidate_range_end(&wpwalk.range);
    dec_tlb_flush_pending(walk.mm);
    }
//
// wp_clean_test_walk - The pagewalk test_walk callback.
//
// Won't perform dirty-tracking on COW, read-only or HUGETLB vmas.
//
#[no_mangle]
pub unsafe extern "C" fn wp_clean_test_walk(start: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut vm_flags: vm_flags_t = 0;
// Skip non-applicable VMAs
    if ((vm_flags & (VM_SHARED | VM_MAYWRITE | VM_HUGETLB)) !=
    (VM_SHARED | VM_MAYWRITE)) {
    return 1;
    }
    return 0;
    }
pub static mut mm_walk_ops: usize = 0;
pub static mut mm_walk_ops: usize = 0;
//
// wp_shared_mapping_range - Write-protect all ptes in an address space range
// @mapping: The address_space we want to write protect
// @first_index: The first page offset in the range
// @nr: Number of incremental page offsets to cover
//
// Note: This function currently skips transhuge page-table entries, since
// it's intended for dirty-tracking on the PTE level. It will warn on
// encountering transhuge write-enabled entries, though, and can easily be
// extended to handle them as well.
//
// Return: The number of ptes actually write-protected. Note that
// already write-protected ptes are not counted.
//
#[no_mangle]
pub unsafe extern "C" fn wp_shared_mapping_range(mapping: *mut address_space, first_index: pgoff_t, nr: pgoff_t) -> c_ulong {
pub static mut wpwalk: wp_walk = 0;
    i_mmap_lock_read(mapping);
    WARN_ON!(walk_page_mapping(mapping, first_index, nr, &wp_walk_ops,
    &wpwalk));
    i_mmap_unlock_read(mapping);
    return wpwalk.total;
    }
    EXPORT_SYMBOL_GPL(wp_shared_mapping_range);
//
// clean_record_shared_mapping_range - Clean and record all ptes in an
// address space range
// @mapping: The address_space we want to clean
// @first_index: The first page offset in the range
// @nr: Number of incremental page offsets to cover
// @bitmap_pgoff: The page offset of the first bit in @bitmap
// @bitmap: Pointer to a bitmap of at least @nr bits. The bitmap needs to
// cover the whole range @first_index..@first_index + @nr.
// @start: Pointer to number of the first set bit in @bitmap.
// is modified as new bits are set by the function.
// @end: Pointer to the number of the last set bit in @bitmap.
// none set. The value is modified as new bits are set by the function.
//
// When this function returns there is no guarantee that a CPU has
// not already dirtied new ptes. However it will not clean any ptes not
// reported in the bitmap. The guarantees are as follows:
//
// * All ptes dirty when the function starts executing will end up recorded
// in the bitmap.
// * All ptes dirtied after that will either remain dirty, be recorded in the
// bitmap or both.
//
// If a caller needs to make sure all dirty ptes are picked up and none
// additional are added, it first needs to write-protect the address-space
// range and make sure new writers are blocked in page_mkwrite() or
// pfn_mkwrite(). And then after a TLB flush following the write-protection
// pick up all dirty bits.
//
// This function currently skips transhuge page-table entries, since
// it's intended for dirty-tracking on the PTE level. It will warn on
// encountering transhuge dirty entries, though, and can easily be extended
// to handle them as well.
//
// Return: The number of dirty ptes actually cleaned.
//
#[no_mangle]
pub unsafe extern "C" fn clean_record_shared_mapping_range(mapping: *mut address_space, first_index: pgoff_t, nr: pgoff_t, bitmap_pgoff: pgoff_t, bitmap: *mut c_ulong, start: *mut pgoff_t, end: *mut pgoff_t) -> c_ulong {
pub static mut none_set: bool = false;
pub static mut clean_walk: usize = 0;
    i_mmap_lock_read(mapping);
    WARN_ON!(walk_page_mapping(mapping, first_index, nr, &clean_walk_ops,
    &cwalk.base));
    i_mmap_unlock_read(mapping);
// start = cwalk.start;
// end = cwalk.end;
    return cwalk.base.total;
    }
    EXPORT_SYMBOL_GPL(clean_record_shared_mapping_range);
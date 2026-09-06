//! Automatically rewritten from C to Rust
//! Source: mm/pagewalk.c
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
// We want to know the real level where a entry is located ignoring any
// folding of levels which may be happening. For example if p4d is folded then
// a missing entry found at level 1 (p4d) is actually at level 0 (pgd).
//
#[no_mangle]
unsafe extern "C" fn real_depth(depth: c_int) -> c_int {
    if (depth == 3 && PTRS_PER_PMD == 1) {
    depth = 2;
    }
    if (depth == 2 && PTRS_PER_PUD == 1) {
    depth = 1;
    }
    if (depth == 1 && PTRS_PER_P4D == 1) {
    depth = 0;
    }
    return depth;
    }
#[no_mangle]
pub unsafe extern "C" fn walk_pte_range_inner(pte: *mut pte_t, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut ops = walk.ops;
pub static mut err: c_int = 0;
    for (;;) {
    if (ops.install_pte && pte_none(ptep_get(pte))) {
    let mut new_pte;
    err = ops.install_pte(addr, addr + PAGE_SIZE, &new_pte,
    walk);
    if (err) {
    break;
    }
    set_pte_at(walk.mm, addr, pte, new_pte);
// Non-present before, so for arches that need it.
    if (!WARN_ON_ONCE!(walk.no_vma)) {
    update_mmu_cache(walk.vma, addr, pte);
    }
    } else {
    err = ops.pte_entry(pte, addr, addr + PAGE_SIZE, walk);
    if (err) {
    break;
    }
    }
    if (addr >= end - PAGE_SIZE) {
    break;
    }
    addr += PAGE_SIZE;
    pte += 1;
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn walk_pte_range(pmd: *mut pmd_t, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut pte: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    if (walk.no_vma) {
//
// pte_offset_map() might apply user-specific validation.
// Indeed, on x86_64 the pmd entries set up by init_espfix_ap()
// fit its pmd_bad() check (_PAGE_NX set and _PAGE_RW clear),
// and CONFIG_EFI_PGT_DUMP efi_mm goes so far as to walk them.
//
    if (walk.mm == &init_mm || addr >= TASK_SIZE) {
    pte = pte_offset_kernel(pmd, addr);
    }
    else {
    pte = pte_offset_map(pmd, addr);
    }
    if (pte) {
    err = walk_pte_range_inner(pte, addr, end, walk);
    if (walk.mm != &init_mm && addr < TASK_SIZE) {
    pte_unmap(pte);
    }
    }
    } else {
    pte = pte_offset_map_lock(walk.mm, pmd, addr, &ptl);
    if (pte) {
    err = walk_pte_range_inner(pte, addr, end, walk);
    pte_unmap_unlock(pte, ptl);
    }
    }
    if (!pte) {
    walk.action = ACTION_AGAIN;
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn walk_pmd_range(pud: *mut pud_t, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut pudval: pud_t = 0;
pub static mut pmd: *mut c_void = core::ptr::null_mut();
    let mut next = 0;
    let mut ops = walk.ops;
pub static mut has_handler: bool = false;
pub static mut has_install: bool = false;
pub static mut err: c_int = 0;
pub static mut depth: c_int = 0;
//
// For PTE handling, pte_offset_map_lock() takes care of checking
// whether there actually is a page table. But it also has to be
// very careful about concurrent page table reclaim.
//
// Similarly, we have to be careful here - a PUD entry that points
// to a PMD table cannot go away, so we can just walk it. But if
// it's something else, we need to ensure we didn't race something,
// so need to retry.
//
// A pertinent example of this is a PUD refault after PUD split -
// we will need to split again or risk accessing invalid memory.
//
    if (!pud_present(pudval) || pud_leaf(pudval)) {
    walk.action = ACTION_AGAIN;
    return 0;
    }
    pmd = pmd_offset(pud, addr);
    do {
// label;
    walk.action = ACTION_SUBTREE;
    next = pmd_addr_end(addr, end);
    if (pmd_none(*pmd)) {
    if (has_install) {
    err = __pte_alloc(walk.mm, pmd);
    }

    else if (ops.pte_hole) {
    err = ops.pte_hole(addr, next, depth, walk);
    }
    if (err) {
    break;
    }
    if (!has_install) {
    continue;
    }
    }
//
// This implies that each ->pmd_entry() handler
// needs to know about pmd_trans_huge() pmds
//
    if (ops.pmd_entry) {
    err = ops.pmd_entry(pmd, addr, next, walk);
    }
    if (err) {
    break;
    }
    if (walk.action == ACTION_AGAIN) {
// goto;
    }
    if (walk.action == ACTION_CONTINUE) {
    continue;
    }
    if (!has_handler) { /* No handlers for lower page tables. */ {
    if (!has_install)
    continue; /* Nothing to do. */
    }
//
// We are ONLY installing, so avoid unnecessarily
// splitting a present huge page.
//
    if (pmd_present(*pmd) && pmd_trans_huge(*pmd)) {
    continue;
    }
    }
    if (walk.vma) {
    split_huge_pmd(walk.vma, pmd, addr);
    }

    else if (pmd_leaf(*pmd) || !pmd_present(*pmd)) {
    continue; /* Nothing to do. */
    }
    err = walk_pte_range(pmd, addr, next, walk);
    if (err) {
    break;
    }
    if (walk.action == ACTION_AGAIN) {
// goto;
    }
    } while (pmd++, addr = next, addr != end);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn walk_pud_range(p4d: *mut p4d_t, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut pud: *mut c_void = core::ptr::null_mut();
    let mut next = 0;
    let mut ops = walk.ops;
pub static mut has_handler: bool = false;
pub static mut has_install: bool = false;
pub static mut err: c_int = 0;
pub static mut depth: c_int = 0;
    pud = pud_offset(p4d, addr);
    do {
// label;
    walk.action = ACTION_SUBTREE;
    next = pud_addr_end(addr, end);
    if (pud_none(*pud)) {
    if (has_install) {
    err = __pmd_alloc(walk.mm, pud, addr);
    }

    else if (ops.pte_hole) {
    err = ops.pte_hole(addr, next, depth, walk);
    }
    if (err) {
    break;
    }
    if (!has_install) {
    continue;
    }
    }
    if (ops.pud_entry) {
    err = ops.pud_entry(pud, addr, next, walk);
    }
    if (err) {
    break;
    }
    if (walk.action == ACTION_AGAIN) {
// goto;
    }
    if (walk.action == ACTION_CONTINUE) {
    continue;
    }
    if (!has_handler) { /* No handlers for lower page tables. */ {
    if (!has_install)
    continue; /* Nothing to do. */
    }
//
// We are ONLY installing, so avoid unnecessarily
// splitting a present huge page.
//
    if (pud_present(*pud) && pud_trans_huge(*pud)) {
    continue;
    }
    }
    if (walk.vma) {
    split_huge_pud(walk.vma, pud, addr);
    }

    else if (pud_leaf(*pud) || !pud_present(*pud)) {
    continue; /* Nothing to do. */
    }
    err = walk_pmd_range(pud, addr, next, walk);
    if (err) {
    break;
    }
    if (walk.action == ACTION_AGAIN) {
// goto;
    }
    } while (pud++, addr = next, addr != end);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn walk_p4d_range(pgd: *mut pgd_t, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut p4d: *mut c_void = core::ptr::null_mut();
    let mut next = 0;
    let mut ops = walk.ops;
pub static mut has_handler: bool = false;
pub static mut has_install: bool = false;
pub static mut err: c_int = 0;
pub static mut depth: c_int = 0;
    p4d = p4d_offset(pgd, addr);
    do {
    next = p4d_addr_end(addr, end);
    if (p4d_none_or_clear_bad(p4d)) {
    if (has_install) {
    err = __pud_alloc(walk.mm, p4d, addr);
    }

    else if (ops.pte_hole) {
    err = ops.pte_hole(addr, next, depth, walk);
    }
    if (err) {
    break;
    }
    if (!has_install) {
    continue;
    }
    }
    if (ops.p4d_entry) {
    err = ops.p4d_entry(p4d, addr, next, walk);
    if (err) {
    break;
    }
    }
    if (has_handler || has_install) {
    err = walk_pud_range(p4d, addr, next, walk);
    }
    if (err) {
    break;
    }
    } while (p4d++, addr = next, addr != end);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn walk_pgd_range(addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut pgd: *mut c_void = core::ptr::null_mut();
    let mut next = 0;
    let mut ops = walk.ops;
    let mut has_handler = ops.p4d_entry || ops.pud_entry || ops.pmd_entry ||
    ops.pte_entry;
pub static mut has_install: bool = false;
pub static mut err: c_int = 0;
    if (walk.pgd) {
    pgd = walk.pgd + pgd_index(addr);
    }
    else {
    pgd = pgd_offset(walk.mm, addr);
    }
    do {
    next = pgd_addr_end(addr, end);
    if (pgd_none_or_clear_bad(pgd)) {
    if (has_install) {
    err = __p4d_alloc(walk.mm, pgd, addr);
    }

    else if (ops.pte_hole) {
    err = ops.pte_hole(addr, next, 0, walk);
    }
    if (err) {
    break;
    }
    if (!has_install) {
    continue;
    }
    }
    if (ops.pgd_entry) {
    err = ops.pgd_entry(pgd, addr, next, walk);
    if (err) {
    break;
    }
    }
    if (has_handler || has_install) {
    err = walk_p4d_range(pgd, addr, next, walk);
    }
    if (err) {
    break;
    }
    } while (pgd++, addr = next, addr != end);
    return err;
    }

#[no_mangle]
pub unsafe extern "C" fn hugetlb_entry_end(h: *mut hstate, addr: c_ulong, end: c_ulong) -> c_ulong {
pub static mut boundary: c_ulong = 0;
    return min(boundary, end);
    }
#[no_mangle]
pub unsafe extern "C" fn walk_hugetlb_range(addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut vma = walk.vma;
    let mut h = hstate_vma(vma);
    let mut next = 0;
pub static mut hmask: c_ulong = 0;
pub static mut sz: c_ulong = 0;
pub static mut pte: *mut c_void = core::ptr::null_mut();
    let mut ops = walk.ops;
pub static mut err: c_int = 0;
    hugetlb_vma_lock_read(vma);
    do {
    next = hugetlb_entry_end(h, addr, end);
    pte = hugetlb_walk(vma, addr & hmask, sz);
    if (pte) {
    err = ops.hugetlb_entry(pte, hmask, addr, next, walk);
    }

    else if (ops.pte_hole) {
    err = ops.pte_hole(addr, next, -1, walk);
    }
    if (err) {
    break;
    }
    } while (addr = next, addr != end);
    hugetlb_vma_unlock_read(vma);
    return err;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: walk_hugetlb_range
pub unsafe extern "C" fn walk_hugetlb_range_dup(addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    return 0;
    }

//
// Decide whether we really walk over the current vma on [@start, @end)
// or skip it via the returned value. Return 0 if we do walk over the
// current vma, and return 1 if we skip the vma. Negative values means
// error, where we abort the current walk.
//
#[no_mangle]
pub unsafe extern "C" fn walk_page_test(start: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut vma = walk.vma;
    let mut ops = walk.ops;
    if (ops.test_walk) {
    return ops.test_walk(start, end, walk);
    }
//
// vma(VM_PFNMAP) doesn't have any valid struct pages behind VM_PFNMAP
// range, so we don't walk over it as we do for normal vmas. However,
// Some callers are interested in handling hole range and they don't
// want to just ignore any single address range. Such users certainly
// define their ->pte_hole() callbacks, so let's delegate them to handle
// vma(VM_PFNMAP).
//
    if (vma.vm_flags & VM_PFNMAP) {
pub static mut err: c_int = 1;
    if (ops.pte_hole) {
    err = ops.pte_hole(start, end, -1, walk);
    }
    return err ? err : 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __walk_page_range(start: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut err: c_int = 0;
    let mut vma = walk.vma;
    let mut ops = walk.ops;
pub static mut is_hugetlb: bool = false;
// We do not support hugetlb PTE installation.
    if (ops.install_pte && is_hugetlb) {
    return -EINVAL;
    }
    if (ops.pre_vma) {
    err = ops.pre_vma(start, end, walk);
    if (err) {
    return err;
    }
    }
    if (is_hugetlb) {
    if (ops.hugetlb_entry) {
    err = walk_hugetlb_range(start, end, walk);
    }
    } else {
    err = walk_pgd_range(start, end, walk);
    }
    if (ops.post_vma) {
    ops.post_vma(walk);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn process_mm_walk_lock(mm: *mut mm_struct, walk_lock: page_walk_lock) {
    if (walk_lock == PGWALK_RDLOCK) {
    mmap_assert_locked(mm);
    }

    else if (walk_lock != PGWALK_VMA_RDLOCK_VERIFY) {
    mmap_assert_write_locked(mm);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn process_vma_walk_lock(vma: *mut vm_area_struct, walk_lock: page_walk_lock) {

    match (walk_lock) {
    PGWALK_WRLOCK => {
    vma_start_write(vma);
    // break;
    }
    PGWALK_WRLOCK_VERIFY => {
    vma_assert_write_locked(vma);
    // break;
    }
    PGWALK_VMA_RDLOCK_VERIFY => {
    vma_assert_locked(vma);
    // break;
    }
    PGWALK_RDLOCK => {
// PGWALK_RDLOCK is handled by process_mm_walk_lock
    // break;
    }
    }

    }
//
// See the comment for walk_page_range(), this performs the heavy lifting of the
// operation, only sets no restrictions on how the walk proceeds.
//
// We usually restrict the ability to install PTEs, but this functionality is
// available to internal memory management code and provided in mm/internal.h.
//
#[no_mangle]
pub unsafe extern "C" fn walk_page_range_mm_unsafe(mm: *mut mm_struct, start: c_ulong, end: c_ulong, ops: *mut mm_walk_ops, private: *mut c_void) -> c_int {
pub static mut err: c_int = 0;
    let mut next = 0;
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut mm_walk: usize = 0;
    if (start >= end) {
    return -EINVAL;
    }
    if (!walk.mm) {
    return -EINVAL;
    }
    process_mm_walk_lock(walk.mm, ops.walk_lock);
    vma = find_vma(walk.mm, start);
    do {
    if (!vma) { /* after the last vma */ {
    walk.vma = core::ptr::null_mut();
    }
    next = end;
    if (ops.pte_hole) {
    err = ops.pte_hole(start, next, -1, &walk);
    }
    } else if (start < vma.vm_start) { /* outside vma */ {
    walk.vma = core::ptr::null_mut();
    }
    next = min(end, vma.vm_start);
    if (ops.pte_hole) {
    err = ops.pte_hole(start, next, -1, &walk);
    }
    } else { /* inside vma */
    process_vma_walk_lock(vma, ops.walk_lock);
    walk.vma = vma;
    next = min(end, vma.vm_end);
    vma = find_vma(mm, vma.vm_end);
    err = walk_page_test(start, next, &walk);
    if (err > 0) {
//
// positive return values are purely for
// controlling the pagewalk, so should never
// be passed to the callers.
//
    err = 0;
    continue;
    }
    if (err < 0) {
    break;
    }
    err = __walk_page_range(start, next, &walk);
    }
    if (err) {
    break;
    }
    } while (start = next, start < end);
    return err;
    }
//
// Determine if the walk operations specified are permitted to be used for a
// page table walk.
//
// This check is performed on all functions which are parameterised by walk
// operations and exposed in include/linux/pagewalk.h.
//
// Internal memory management code can use *_unsafe() functions to be able to
// use all page walking operations.
//
#[no_mangle]
unsafe extern "C" fn check_ops_safe(ops: *const mm_walk_ops) -> bool {
//
// The installation of PTEs is solely under the control of memory
// management logic and subject to many subtle locking, security and
// cache considerations so we cannot permit other users to do so, and
// certainly not for exported symbols.
//
    if (ops.install_pte) {
    return false;
    }
    return true;
    }
//
// walk_page_range - walk page table with caller specific callbacks
// @mm:		mm_struct representing the target process of page table walk
// @start:	start address of the virtual address range
// @end:	end address of the virtual address range
// @ops:	operation to call during the walk
// @private:	private data for callbacks' usage
//
// Recursively walk the page table tree of the process represented by @mm
// within the virtual address range [@start, @end). During walking, we can do
// some caller-specific works for each entry, by setting up pmd_entry(),
// pte_entry(), and/or hugetlb_entry(). If you don't set up for some of these
// callbacks, the associated entries/pages are just ignored.
// The return values of these callbacks are commonly defined like below:
//
// - 0  : succeeded to handle the current entry, and if you don't reach the
// end address yet, continue to walk.
// - >0 : succeeded to handle the current entry, and return to the caller
// with caller specific value.
// - <0 : failed to handle the current entry, and return to the caller
// with error code.
//
// Before starting to walk page table, some callers want to check whether
// they really want to walk over the current vma, typically by checking
// its vm_flags. walk_page_test() and @ops->test_walk() are used for this
// purpose.
//
// If operations need to be staged before and committed after a vma is walked,
// there are two callbacks, pre_vma() and post_vma(). Note that post_vma(),
// since it is intended to handle commit-type operations, can't return any
// errors.
//
// struct mm_walk keeps current values of some common data like vma and pmd,
// which are useful for the access from callbacks. If you want to pass some
// caller-specific data to callbacks, @private should be helpful.
//
// Locking:
// Callers of walk_page_range() and walk_page_vma() should hold @mm->mmap_lock,
// because these function traverse vma list and/or access to vma's data.
//
#[no_mangle]
pub unsafe extern "C" fn walk_page_range(mm: *mut mm_struct, start: c_ulong, end: c_ulong, ops: *mut mm_walk_ops, private: *mut c_void) -> c_int {
    if (!check_ops_safe(ops)) {
    return -EINVAL;
    }
    return walk_page_range_mm_unsafe(mm, start, end, ops, private);
    }
//
// walk_kernel_page_table_range - walk a range of kernel pagetables.
// @start:	start address of the virtual address range
// @end:	end address of the virtual address range
// @ops:	operation to call during the walk
// @pgd:	pgd to walk if different from mm->pgd
// @private:	private data for callbacks' usage
//
// Similar to walk_page_range() but can walk any page tables even if they are
// not backed by VMAs. Because 'unusual' entries may be walked this function
// will also not lock the PTEs for the pte_entry() callback. This is useful for
// walking kernel pages tables or page tables for firmware.
//
// Note: Be careful to walk the kernel pages tables, the caller may be need to
// take other effective approaches (mmap lock may be insufficient) to prevent
// the intermediate kernel page tables belonging to the specified address range
// from being freed (e.g. memory hot-remove).
//
#[no_mangle]
pub unsafe extern "C" fn walk_kernel_page_table_range(start: c_ulong, end: c_ulong, ops: *mut mm_walk_ops, pgd: *mut pgd_t, private: *mut c_void) -> c_int {
//
// Kernel intermediate page tables are usually not freed, so the mmap
// read lock is sufficient. But there are some exceptions.
// E.g. memory hot-remove. In which case, the mmap lock is insufficient
// to prevent the intermediate kernel pages tables belonging to the
// specified address range from being freed. The caller should take
// other actions to prevent this race.
//
    mmap_assert_locked(&init_mm);
    return walk_kernel_page_table_range_lockless(start, end, ops, pgd,
    private);
    }
//
// Use this function to walk the kernel page tables locklessly. It should be
// guaranteed that the caller has exclusive access over the range they are
// operating on - that there should be no concurrent access, for example,
// changing permissions for vmalloc objects.
//
#[no_mangle]
pub unsafe extern "C" fn walk_kernel_page_table_range_lockless(start: c_ulong, end: c_ulong, ops: *mut mm_walk_ops, pgd: *mut pgd_t, private: *mut c_void) -> c_int {
pub static mut mm_walk: usize = 0;
    if (start >= end) {
    return -EINVAL;
    }
    if (!check_ops_safe(ops)) {
    return -EINVAL;
    }
    return walk_pgd_range(start, end, &walk);
    }
//
// walk_page_range_debug - walk a range of pagetables not backed by a vma
// @mm:		mm_struct representing the target process of page table walk
// @start:	start address of the virtual address range
// @end:	end address of the virtual address range
// @ops:	operation to call during the walk
// @pgd:	pgd to walk if different from mm->pgd
// @private:	private data for callbacks' usage
//
// Similar to walk_page_range() but can walk any page tables even if they are
// not backed by VMAs. Because 'unusual' entries may be walked this function
// will also not lock the PTEs for the pte_entry() callback.
//
// This is for debugging purposes ONLY.
//
// The mmap write lock must be held.
//
#[no_mangle]
pub unsafe extern "C" fn walk_page_range_debug(mm: *mut mm_struct, start: c_ulong, end: c_ulong, ops: *mut mm_walk_ops, pgd: *mut pgd_t, private: *mut c_void) -> c_int {
pub static mut mm_walk: usize = 0;
//
// When walking userland page tables, an mmap write lock must be held to
// account for munmap() downgrading to an mmap read lock when tearing
// down page tables.
//
// When walking kernel page tables, an mmap write lock must also be held
// to account for page table freeing on vmap huge page mapping.
//
    mmap_assert_write_locked(mm);
//
// x86, arm64 ptdump allow walks of efi mm's and x86 ptdump allows walks
// of arbitrary mm's.
//
// However, they both must also hold the init_mm lock to account for
// concurrent kernel page table freeing.
//
    mmap_assert_write_locked(&init_mm);
    if (start >= end) {
    return -EINVAL;
    }
    if (!check_ops_safe(ops)) {
    return -EINVAL;
    }
    return walk_pgd_range(start, end, &walk);
    }
#[no_mangle]
pub unsafe extern "C" fn walk_page_range_vma_unsafe(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong, ops: *mut mm_walk_ops, private: *mut c_void) -> c_int {
pub static mut mm_walk: usize = 0;
    if (start >= end || !walk.mm) {
    return -EINVAL;
    }
    if (start < vma.vm_start || end > vma.vm_end) {
    return -EINVAL;
    }
    process_mm_walk_lock(walk.mm, ops.walk_lock);
    process_vma_walk_lock(vma, ops.walk_lock);
    return __walk_page_range(start, end, &walk);
    }
#[no_mangle]
pub unsafe extern "C" fn walk_page_range_vma(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong, ops: *mut mm_walk_ops, private: *mut c_void) -> c_int {
    if (!check_ops_safe(ops)) {
    return -EINVAL;
    }
    return walk_page_range_vma_unsafe(vma, start, end, ops, private);
    }
#[no_mangle]
pub unsafe extern "C" fn walk_page_vma(vma: *mut vm_area_struct, ops: *mut mm_walk_ops, private: *mut c_void) -> c_int {
pub static mut mm_walk: usize = 0;
    if (!walk.mm) {
    return -EINVAL;
    }
    if (!check_ops_safe(ops)) {
    return -EINVAL;
    }
    process_mm_walk_lock(walk.mm, ops.walk_lock);
    process_vma_walk_lock(vma, ops.walk_lock);
    return __walk_page_range(vma.vm_start, vma.vm_end, &walk);
    }
//
// walk_page_mapping - walk all memory areas mapped into a struct address_space.
// @mapping: Pointer to the struct address_space
// @first_index: First page offset in the address_space
// @nr: Number of incremental page offsets to cover
// @ops:	operation to call during the walk
// @private:	private data for callbacks' usage
//
// This function walks all memory areas mapped into a struct address_space.
// The walk is limited to only the given page-size index range, but if
// the index boundaries cross a huge page-table entry, that entry will be
// included.
//
// Also see walk_page_range() for additional information.
//
// Locking:
// This function can't require that the struct mm_struct::mmap_lock is held,
// since @mapping may be mapped by multiple processes. Instead
// @mapping->i_mmap_rwsem must be held. This might have implications in the
// callbacks, and it's up tho the caller to ensure that the
// struct mm_struct::mmap_lock is not needed.
//
// Also this means that a caller can't rely on the struct
// vm_area_struct::vm_flags to be constant across a call,
// except for immutable flags. Callers requiring this shouldn't use
// this function.
//
// Return: 0 on success, negative error code on failure, positive number on
// caller defined premature termination.
//
#[no_mangle]
pub unsafe extern "C" fn walk_page_mapping(mapping: *mut address_space, first_index: pgoff_t, nr: pgoff_t, ops: *mut mm_walk_ops, private: *mut c_void) -> c_int {
pub static mut mm_walk: usize = 0;
pub static mut vma: *mut c_void = core::ptr::null_mut();
    pgoff_t vba, vea, cba, cea;
    unsigned long start_addr, end_addr;
pub static mut err: c_int = 0;
    if (!check_ops_safe(ops)) {
    return -EINVAL;
    }
    lockdep_assert_held(&mapping.i_mmap_rwsem);
    mapping_rmap_tree_foreach(vma, mapping, first_index,
    first_index + nr - 1) {
// Clip to the vma
    vba = vma_start_pgoff(vma);
    vea = vba + vma_pages(vma);
    cba = first_index;
    cba = max(cba, vba);
    cea = first_index + nr;
    cea = min(cea, vea);
    start_addr = ((cba - vba) << PAGE_SHIFT) + vma.vm_start;
    end_addr = ((cea - vba) << PAGE_SHIFT) + vma.vm_start;
    if (start_addr >= end_addr) {
    continue;
    }
    walk.vma = vma;
    walk.mm = vma.vm_mm;
    err = walk_page_test(vma.vm_start, vma.vm_end, &walk);
    if (err > 0) {
    err = 0;
    break;
    } else if (err < 0) {
    break;
    }
    err = __walk_page_range(start_addr, end_addr, &walk);
    if (err) {
    break;
    }
    }
    return err;
    }
//
// folio_walk_start - walk the page tables to a folio
// @fw: filled with information on success.
// @vma: the VMA.
// @addr: the virtual address to use for the page table walk.
// @flags: flags modifying which folios to walk to.
//
// Walk the page tables using @addr in a given @vma to a mapped folio and
// return the folio, making sure that the page table entry referenced by
// @addr cannot change until folio_walk_end() was called.
//
// As default, this function returns only folios that are not special (e.g., not
// the zeropage) and never returns folios that are supposed to be ignored by the
// VM as documented by vm_normal_page(). If requested, zeropages will be
// returned as well.
//
// If this function returns NULL it might either indicate "there is nothing" or
// "there is nothing suitable".
//
// On success, @fw is filled and the function returns the folio while the PTL
// is still held and folio_walk_end() must be called to clean up,
// releasing any held locks. The returned folio must *not* be used after the
// call to folio_walk_end(), unless a short-term folio reference is taken before
// that call.
//
// @fw->page will correspond to the page that is effectively referenced by
// @addr. However, for shared zeropages @fw->page is set to NULL. Note that
// large folios might be mapped by multiple page table entries, and this
// function will always only lookup a single entry as specified by @addr, which
// might or might not cover more than a single page of the returned folio.
//
// This function must *not* be used as a naive replacement for
// get_user_pages() / pin_user_pages(), especially not to perform DMA or
// to carelessly modify page content. This function may *only* be used to grab
// short-term folio references, never to grab long-term folio references.
//
// Using the page table entry pointers in @fw for reading or modifying the
// entry should be avoided where possible: however, there might be valid
// use cases.
//
// WARNING: Modifying page table entries in hugetlb VMAs requires a lot of care.
// For example, PMD page table sharing might require prior unsharing. Also,
// logical hugetlb entries might span multiple physical page table entries,
// which *must* be modified in a single operation (set_huge_pte_at(),
// huge_ptep_set_*, ...). Note that the page table entry stored in @fw might
// not correspond to the first physical entry of a logical hugetlb entry.
//
// The mmap lock must be held in read mode.
//
// Return: folio pointer on success, otherwise NULL.
//
#[no_mangle]
pub unsafe extern "C" fn folio_walk_start(fw: *mut folio_walk, vma: *mut vm_area_struct, addr: c_ulong, flags: folio_walk_flags_t) -> *mut c_void {
    let mut entry_size = 0;
pub static mut zeropage: bool = false;
pub static mut page: *mut c_void = core::ptr::null_mut();
    pud_t *pudp, pud;
    pmd_t *pmdp, pmd;
    pte_t *ptep, pte;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
pub static mut pgdp: *mut c_void = core::ptr::null_mut();
pub static mut p4dp: *mut c_void = core::ptr::null_mut();
    mmap_assert_locked(vma.vm_mm);
    vma_pgtable_walk_begin(vma);
    if (WARN_ON_ONCE!(addr < vma.vm_start || addr >= vma.vm_end)) {
// goto;
    }
    pgdp = pgd_offset(vma.vm_mm, addr);
    if (pgd_none_or_clear_bad(pgdp)) {
// goto;
    }
    p4dp = p4d_offset(pgdp, addr);
    if (p4d_none_or_clear_bad(p4dp)) {
// goto;
    }
    pudp = pud_offset(p4dp, addr);
    pud = pudp_get(pudp);
    if (pud_none(pud)) {
// goto;
    }
    if (IS_ENABLED!(CONFIG_PGTABLE_HAS_HUGE_LEAVES) &&
    (!pud_present(pud) || pud_leaf(pud))) {
    ptl = pud_lock(vma.vm_mm, pudp);
    pud = pudp_get(pudp);
    entry_size = PUD_SIZE;
    fw.level = FW_LEVEL_PUD;
    fw.pudp = pudp;
    fw.pud = pud;
    if (pud_none(pud)) {
    spin_unlock(ptl);
// goto;
    } else if (pud_present(pud) && !pud_leaf(pud)) {
    spin_unlock(ptl);
// goto;
    } else if (pud_present(pud)) {
    page = vm_normal_page_pud(vma, addr, pud);
    if (page) {
// goto;
    }
    }
    spin_unlock(ptl);
// goto;
    }
// label;
    VM_WARN_ON_ONCE(!pud_present(pud) || pud_leaf(pud));
    pmdp = pmd_offset(pudp, addr);
    pmd = pmdp_get_lockless(pmdp);
    if (pmd_none(pmd)) {
// goto;
    }
    if (IS_ENABLED!(CONFIG_PGTABLE_HAS_HUGE_LEAVES) &&
    (!pmd_present(pmd) || pmd_leaf(pmd))) {
    ptl = pmd_lock(vma.vm_mm, pmdp);
    pmd = pmdp_get(pmdp);
    entry_size = PMD_SIZE;
    fw.level = FW_LEVEL_PMD;
    fw.pmdp = pmdp;
    fw.pmd = pmd;
    if (pmd_none(pmd)) {
    spin_unlock(ptl);
// goto;
    } else if (pmd_present(pmd) && !pmd_leaf(pmd)) {
    spin_unlock(ptl);
// goto;
    } else if (pmd_present(pmd)) {
    page = vm_normal_page_pmd(vma, addr, pmd);
    if (page) {
// goto;
    } else if ((flags & FW_ZEROPAGE) &&
    is_huge_zero_pmd(pmd)) {
    page = pfn_to_page(pmd_pfn(pmd));
    zeropage = true;
// goto;
    }
    }
    spin_unlock(ptl);
// goto;
    }
// label;
    VM_WARN_ON_ONCE(!pmd_present(pmd) || pmd_leaf(pmd));
    ptep = pte_offset_map_lock(vma.vm_mm, pmdp, addr, &ptl);
    if (!ptep) {
// goto;
    }
    pte = ptep_get(ptep);
    entry_size = PAGE_SIZE;
    fw.level = FW_LEVEL_PTE;
    fw.ptep = ptep;
    fw.pte = pte;
    if (pte_present(pte)) {
    page = vm_normal_page(vma, addr, pte);
    if (page) {
// goto;
    }
    if ((flags & FW_ZEROPAGE) &&
    is_zero_pfn(pte_pfn(pte))) {
    page = pfn_to_page(pte_pfn(pte));
    zeropage = true;
// goto;
    }
    }
    pte_unmap_unlock(ptep, ptl);
// label;
    vma_pgtable_walk_end(vma);
    return core::ptr::null_mut();
// label;
    if (!zeropage) {
// Note: Offset from the mapped page, not the folio start.
    fw.page = page + ((addr & (entry_size - 1)) >> PAGE_SHIFT);
    }
    else {
    fw.page = core::ptr::null_mut();
    }
    fw.ptl = ptl;
    return page_folio(page);
    }
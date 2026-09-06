//! Automatically rewritten from C to Rust
//! Source: mm/kasan/init.c
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
// This file contains KASAN shadow initialization code.
//
// Copyright (c) 2015 Samsung Electronics Co., Ltd.
// Author: Andrey Ryabinin <ryabinin.a.a@gmail.com>
//

//
// This page serves two purposes:
// - It used as early shadow memory. The entire shadow region populated
// with this page, before we will be able to setup normal shadow memory.
// - Latter it reused it as zero shadow to cover large ranges of memory
// that allowed to access, but not handled by kasan (vmalloc/vmemmap ...).
//
    unsigned char kasan_early_shadow_page[PAGE_SIZE] __bss_pgtbl;

    p4d_t kasan_early_shadow_p4d[MAX_PTRS_PER_P4D] __bss_pgtbl;
#[no_mangle]
pub unsafe extern "C" fn kasan_p4d_table(pgd: pgd_t) -> bool {
    return pgd_page(pgd) == virt_to_page(lm_alias(kasan_early_shadow_p4d));
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: kasan_p4d_table
pub unsafe extern "C" fn kasan_p4d_table_dup(pgd: pgd_t) -> bool {
    return false;
    }

    pud_t kasan_early_shadow_pud[MAX_PTRS_PER_PUD] __bss_pgtbl;
#[no_mangle]
pub unsafe extern "C" fn kasan_pud_table(p4d: p4d_t) -> bool {
    return p4d_page(p4d) == virt_to_page(lm_alias(kasan_early_shadow_pud));
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: kasan_pud_table
pub unsafe extern "C" fn kasan_pud_table_dup(p4d: p4d_t) -> bool {
    return false;
    }

    pmd_t kasan_early_shadow_pmd[MAX_PTRS_PER_PMD] __bss_pgtbl;
#[no_mangle]
pub unsafe extern "C" fn kasan_pmd_table(pud: pud_t) -> bool {
    return pud_page(pud) == virt_to_page(lm_alias(kasan_early_shadow_pmd));
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: kasan_pmd_table
pub unsafe extern "C" fn kasan_pmd_table_dup(pud: pud_t) -> bool {
    return false;
    }

    pte_t kasan_early_shadow_pte[MAX_PTRS_PER_PTE + PTE_HWTABLE_PTRS]
    __bss_pgtbl;
#[no_mangle]
pub unsafe extern "C" fn kasan_pte_table(pmd: pmd_t) -> bool {
    return pmd_page(pmd) == virt_to_page(lm_alias(kasan_early_shadow_pte));
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_early_shadow_page_entry(pte: pte_t) -> bool {
    return pte_page(pte) == virt_to_page(lm_alias(kasan_early_shadow_page));
    }
#[no_mangle]
pub unsafe extern "C" fn early_alloc(size: size_t, node: c_int) -> *mut c_void {
    let mut ptr = memblock_alloc_try_nid(size, size, __pa(MAX_DMA_ADDRESS),
    MEMBLOCK_ALLOC_ACCESSIBLE, node);
    if (!ptr) {
    panic("%s: Failed to allocate %zu bytes align=%zx nid=%d from=%llx\n",
    __func__, size, size, node, (u64)__pa(MAX_DMA_ADDRESS));
    }
    return ptr;
    }
    static void __ref zero_pte_populate(pmd_t *pmd, unsigned long addr,
    unsigned long end)
    {
    let mut pte = pte_offset_kernel(pmd, addr);
    let mut zero_pte;
    zero_pte = pfn_pte(PFN_DOWN(__pa_symbol(kasan_early_shadow_page)),
    PAGE_KERNEL);
    zero_pte = pte_wrprotect(zero_pte);
    while (addr + PAGE_SIZE <= end) {
    set_pte_at(&init_mm, addr, pte, zero_pte);
    addr += PAGE_SIZE;
    pte = pte_offset_kernel(pmd, addr);
    }
    }
    static int __ref zero_pmd_populate(pud_t *pud, unsigned long addr,
    unsigned long end)
    {
    let mut pmd = pmd_offset(pud, addr);
    let mut next = 0;
    do {
    next = pmd_addr_end(addr, end);
    if (IS_ALIGNED(addr, PMD_SIZE) && end - addr >= PMD_SIZE) {
    pmd_populate_kernel(&init_mm, pmd,
    lm_alias(kasan_early_shadow_pte));
    continue;
    }
    if (pmd_none(*pmd)) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (slab_is_available()) {
    p = pte_alloc_one_kernel(&init_mm);
    }
    else {
    p = early_alloc(PAGE_SIZE, NUMA_NO_NODE);
    kernel_pte_init(p);
    }
    if (!p) {
    return -ENOMEM;
    }
    pmd_populate_kernel(&init_mm, pmd, p);
    }
    zero_pte_populate(pmd, addr, next);
    } while (pmd++, addr = next, addr != end);
    return 0;
    }
    static int __ref zero_pud_populate(p4d_t *p4d, unsigned long addr,
    unsigned long end)
    {
    let mut pud = pud_offset(p4d, addr);
    let mut next = 0;
    do {
    next = pud_addr_end(addr, end);
    if (IS_ALIGNED(addr, PUD_SIZE) && end - addr >= PUD_SIZE) {
pub static mut pmd: *mut c_void = core::ptr::null_mut();
    pud_populate(&init_mm, pud,
    lm_alias(kasan_early_shadow_pmd));
    pmd = pmd_offset(pud, addr);
    pmd_populate_kernel(&init_mm, pmd,
    lm_alias(kasan_early_shadow_pte));
    continue;
    }
    if (pud_none(*pud)) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (slab_is_available()) {
    p = pmd_alloc(&init_mm, pud, addr);
    if (!p) {
    return -ENOMEM;
    }
    } else {
    p = early_alloc(PAGE_SIZE, NUMA_NO_NODE);
    pmd_init(p);
    pud_populate(&init_mm, pud, p);
    }
    }
    zero_pmd_populate(pud, addr, next);
    } while (pud++, addr = next, addr != end);
    return 0;
    }
    static int __ref zero_p4d_populate(pgd_t *pgd, unsigned long addr,
    unsigned long end)
    {
    let mut p4d = p4d_offset(pgd, addr);
    let mut next = 0;
    do {
    next = p4d_addr_end(addr, end);
    if (IS_ALIGNED(addr, P4D_SIZE) && end - addr >= P4D_SIZE) {
pub static mut pud: *mut c_void = core::ptr::null_mut();
pub static mut pmd: *mut c_void = core::ptr::null_mut();
    p4d_populate_kernel(addr, p4d,
    lm_alias(kasan_early_shadow_pud));
    pud = pud_offset(p4d, addr);
    pud_populate(&init_mm, pud,
    lm_alias(kasan_early_shadow_pmd));
    pmd = pmd_offset(pud, addr);
    pmd_populate_kernel(&init_mm, pmd,
    lm_alias(kasan_early_shadow_pte));
    continue;
    }
    if (p4d_none(*p4d)) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (slab_is_available()) {
    p = pud_alloc(&init_mm, p4d, addr);
    if (!p) {
    return -ENOMEM;
    }
    } else {
    p = early_alloc(PAGE_SIZE, NUMA_NO_NODE);
    pud_init(p);
    p4d_populate_kernel(addr, p4d, p);
    }
    }
    zero_pud_populate(p4d, addr, next);
    } while (p4d++, addr = next, addr != end);
    return 0;
    }
//
// kasan_populate_early_shadow - populate shadow memory region with
// kasan_early_shadow_page
// @shadow_start: start of the memory range to populate
// @shadow_end: end of the memory range to populate
//
    int __ref kasan_populate_early_shadow(const void *shadow_start,
    const void *shadow_end)
    {
pub static mut addr: c_ulong = 0;
pub static mut end: c_ulong = 0;
    let mut pgd = pgd_offset_k(addr);
    let mut next = 0;
    do {
    next = pgd_addr_end(addr, end);
    if (IS_ALIGNED(addr, PGDIR_SIZE) && end - addr >= PGDIR_SIZE) {
pub static mut p4d: *mut c_void = core::ptr::null_mut();
pub static mut pud: *mut c_void = core::ptr::null_mut();
pub static mut pmd: *mut c_void = core::ptr::null_mut();
//
// kasan_early_shadow_pud should be populated with pmds
// at this moment.
// [pud,pmd]_populate*() below needed only for
// 3,2 - level page tables where we don't have
// puds,pmds, so pgd_populate(), pud_populate()
// is noops.
//
    pgd_populate_kernel(addr, pgd,
    lm_alias(kasan_early_shadow_p4d));
    p4d = p4d_offset(pgd, addr);
    p4d_populate_kernel(addr, p4d,
    lm_alias(kasan_early_shadow_pud));
    pud = pud_offset(p4d, addr);
    pud_populate(&init_mm, pud,
    lm_alias(kasan_early_shadow_pmd));
    pmd = pmd_offset(pud, addr);
    pmd_populate_kernel(&init_mm, pmd,
    lm_alias(kasan_early_shadow_pte));
    continue;
    }
    if (pgd_none(*pgd)) {
    if (slab_is_available()) {
    if (!p4d_alloc(&init_mm, pgd, addr)) {
    return -ENOMEM;
    }
    } else {
    pgd_populate_kernel(addr, pgd,
    early_alloc(PAGE_SIZE, NUMA_NO_NODE));
    }
    }
    zero_p4d_populate(pgd, addr, next);
    } while (pgd++, addr = next, addr != end);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kasan_free_pte(pte_start: *mut pte_t, pmd: *mut pmd_t) {
pub static mut pte: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < PTRS_PER_PTE) {
    pte = pte_start + i;
    if (!pte_none(ptep_get(pte))) {
    return;
    }
    }
    pte_free_kernel(&init_mm, pte_start);
    pmd_clear(pmd);
    }
#[no_mangle]
unsafe extern "C" fn kasan_free_pmd(pmd_start: *mut pmd_t, pud: *mut pud_t) {
pub static mut pmd: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < PTRS_PER_PMD) {
    pmd = pmd_start + i;
    if (!pmd_none(*pmd)) {
    return;
    }
    }
    pmd_free(&init_mm, pmd_start);
    pud_clear(pud);
    }
#[no_mangle]
unsafe extern "C" fn kasan_free_pud(pud_start: *mut pud_t, p4d: *mut p4d_t) {
pub static mut pud: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < PTRS_PER_PUD) {
    pud = pud_start + i;
    if (!pud_none(*pud)) {
    return;
    }
    }
    pud_free(&init_mm, pud_start);
    p4d_clear(p4d);
    }
#[no_mangle]
unsafe extern "C" fn kasan_free_p4d(p4d_start: *mut p4d_t, pgd: *mut pgd_t) {
pub static mut p4d: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    while (i < PTRS_PER_P4D) {
    p4d = p4d_start + i;
    if (!p4d_none(*p4d)) {
    return;
    }
    }
    p4d_free(&init_mm, p4d_start);
    pgd_clear(pgd);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_remove_pte_table(pte: *mut pte_t, addr: c_ulong, end: c_ulong) {
    let mut next = 0;
    let mut ptent;
    while (addr < end) {
    next = (addr + PAGE_SIZE) & PAGE_MASK;
    if (next > end) {
    next = end;
    }
    ptent = ptep_get(pte);
    if (!pte_present(ptent)) {
    continue;
    }
    if (WARN_ON!(!kasan_early_shadow_page_entry(ptent))) {
    continue;
    }
    pte_clear(&init_mm, addr, pte);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_remove_pmd_table(pmd: *mut pmd_t, addr: c_ulong, end: c_ulong) {
    let mut next = 0;
    while (addr < end) {
pub static mut pte: *mut c_void = core::ptr::null_mut();
    next = pmd_addr_end(addr, end);
    if (!pmd_present(*pmd)) {
    continue;
    }
    if (kasan_pte_table(*pmd)) {
    if (IS_ALIGNED(addr, PMD_SIZE) &&
    IS_ALIGNED(next, PMD_SIZE)) {
    pmd_clear(pmd);
    continue;
    }
    }
    pte = pte_offset_kernel(pmd, addr);
    kasan_remove_pte_table(pte, addr, next);
    kasan_free_pte(pte_offset_kernel(pmd, 0), pmd);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_remove_pud_table(pud: *mut pud_t, addr: c_ulong, end: c_ulong) {
    let mut next = 0;
    while (addr < end) {
    let mut pmd = core::ptr::null_mut();
    let mut pmd_base = core::ptr::null_mut();
    next = pud_addr_end(addr, end);
    if (!pud_present(*pud)) {
    continue;
    }
    if (kasan_pmd_table(*pud)) {
    if (IS_ALIGNED(addr, PUD_SIZE) &&
    IS_ALIGNED(next, PUD_SIZE)) {
    pud_clear(pud);
    continue;
    }
    }
    pmd = pmd_offset(pud, addr);
    pmd_base = pmd_offset(pud, 0);
    kasan_remove_pmd_table(pmd, addr, next);
    kasan_free_pmd(pmd_base, pud);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_remove_p4d_table(p4d: *mut p4d_t, addr: c_ulong, end: c_ulong) {
    let mut next = 0;
    while (addr < end) {
pub static mut pud: *mut c_void = core::ptr::null_mut();
    next = p4d_addr_end(addr, end);
    if (!p4d_present(*p4d)) {
    continue;
    }
    if (kasan_pud_table(*p4d)) {
    if (IS_ALIGNED(addr, P4D_SIZE) &&
    IS_ALIGNED(next, P4D_SIZE)) {
    p4d_clear(p4d);
    continue;
    }
    }
    pud = pud_offset(p4d, addr);
    kasan_remove_pud_table(pud, addr, next);
    kasan_free_pud(pud_offset(p4d, 0), p4d);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_remove_zero_shadow(start: *mut c_void, size: c_ulong) {
    unsigned long addr, end, next;
pub static mut pgd: *mut c_void = core::ptr::null_mut();
    addr = (unsigned long)kasan_mem_to_shadow(start);
    end = addr + (size >> KASAN_SHADOW_SCALE_SHIFT);
    if (WARN_ON!((unsigned long)start % KASAN_MEMORY_PER_SHADOW_PAGE) ||
    WARN_ON!(size % KASAN_MEMORY_PER_SHADOW_PAGE)) {
    return;
    }
    while (addr < end) {
pub static mut p4d: *mut c_void = core::ptr::null_mut();
    next = pgd_addr_end(addr, end);
    pgd = pgd_offset_k(addr);
    if (!pgd_present(*pgd)) {
    continue;
    }
    if (kasan_p4d_table(*pgd)) {
    if (IS_ALIGNED(addr, PGDIR_SIZE) &&
    IS_ALIGNED(next, PGDIR_SIZE)) {
    pgd_clear(pgd);
    continue;
    }
    }
    p4d = p4d_offset(pgd, addr);
    kasan_remove_p4d_table(p4d, addr, next);
    kasan_free_p4d(p4d_offset(pgd, 0), pgd);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_add_zero_shadow(start: *mut c_void, size: c_ulong) -> c_int {
    let mut ret = 0;
    let mut shadow_start = core::ptr::null_mut();
    let mut shadow_end = core::ptr::null_mut();
    shadow_start = kasan_mem_to_shadow(start);
    shadow_end = shadow_start + (size >> KASAN_SHADOW_SCALE_SHIFT);
    if (WARN_ON!((unsigned long)start % KASAN_MEMORY_PER_SHADOW_PAGE) ||
    WARN_ON!(size % KASAN_MEMORY_PER_SHADOW_PAGE)) {
    return -EINVAL;
    }
    ret = kasan_populate_early_shadow(shadow_start, shadow_end);
    if (ret) {
    kasan_remove_zero_shadow(start, size);
    }
    return ret;
    }
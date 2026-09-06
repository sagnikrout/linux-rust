//! Automatically rewritten from C to Rust
//! Source: mm/kasan/shadow.c
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
// This file contains KASAN runtime code that manages shadow memory for
// generic and software tag-based KASAN modes.
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// Author: Andrey Ryabinin <ryabinin.a.a@gmail.com>
//
// Some code borrowed from https://github.com/xairy/kasan-prototype by
// Andrey Konovalov <andreyknvl@gmail.com>
//

#[no_mangle]
pub unsafe extern "C" fn __kasan_check_read(p: *const volatile void, size: c_uint) -> bool {
    return kasan_check_range(p, size, false, _RET_IP_);
    }
    EXPORT_SYMBOL(__kasan_check_read);
#[no_mangle]
pub unsafe extern "C" fn __kasan_check_write(p: *const volatile void, size: c_uint) -> bool {
    return kasan_check_range(p, size, true, _RET_IP_);
    }
    EXPORT_SYMBOL(__kasan_check_write);

//
// CONFIG_GENERIC_ENTRY relies on compiler emitted mem*() calls to not be
// instrumented. KASAN enabled toolchains should emit __asan_mem*() functions
// for the sites they want to instrument.
//
// If we have a compiler that can instrument meminstrinsics, never override
// these, so that non-instrumented files can safely consider them as builtins.
//

#[no_mangle]
pub unsafe extern "C" fn memset(addr: *mut c_void, c: c_int, len: size_t) -> *mut c_void {
    if (!kasan_check_range(addr, len, true, _RET_IP_)) {
    return core::ptr::null_mut();
    }
    return __memset(addr, c, len);
    }

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut c_void, src: *mut c_void, len: size_t) -> *mut c_void {
    if (!kasan_check_range(src, len, false, _RET_IP_) ||
    !kasan_check_range(dest, len, true, _RET_IP_)) {
    return core::ptr::null_mut();
    }
    return __memmove(dest, src, len);
    }

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *mut c_void, len: size_t) -> *mut c_void {
    if (!kasan_check_range(src, len, false, _RET_IP_) ||
    !kasan_check_range(dest, len, true, _RET_IP_)) {
    return core::ptr::null_mut();
    }
    return __memcpy(dest, src, len);
    }

#[no_mangle]
pub unsafe extern "C" fn __asan_memset(addr: *mut c_void, c: c_int, len: ssize_t) -> *mut c_void {
    if (!kasan_check_range(addr, len, true, _RET_IP_)) {
    return core::ptr::null_mut();
    }
    return __memset(addr, c, len);
    }
    EXPORT_SYMBOL(__asan_memset);

#[no_mangle]
pub unsafe extern "C" fn __asan_memmove(dest: *mut c_void, src: *mut c_void, len: ssize_t) -> *mut c_void {
    if (!kasan_check_range(src, len, false, _RET_IP_) ||
    !kasan_check_range(dest, len, true, _RET_IP_)) {
    return core::ptr::null_mut();
    }
    return __memmove(dest, src, len);
    }
    EXPORT_SYMBOL(__asan_memmove);

#[no_mangle]
pub unsafe extern "C" fn __asan_memcpy(dest: *mut c_void, src: *mut c_void, len: ssize_t) -> *mut c_void {
    if (!kasan_check_range(src, len, false, _RET_IP_) ||
    !kasan_check_range(dest, len, true, _RET_IP_)) {
    return core::ptr::null_mut();
    }
    return __memcpy(dest, src, len);
    }
    EXPORT_SYMBOL(__asan_memcpy);

// forward_decl: __hwasan_memset;
    EXPORT_SYMBOL(__hwasan_memset);

// forward_decl: __hwasan_memmove;
    EXPORT_SYMBOL(__hwasan_memmove);

// forward_decl: __hwasan_memcpy;
    EXPORT_SYMBOL(__hwasan_memcpy);

#[no_mangle]
pub unsafe extern "C" fn kasan_poison(addr: *const c_void, size: usize, value: u8, init: bool) {
    let mut shadow_start = core::ptr::null_mut();
    let mut shadow_end = core::ptr::null_mut();
    if (!kasan_enabled()) {
    return;
    }
//
// Perform shadow offset calculation based on untagged address, as
// some of the callers (e.g. kasan_poison_new_object) pass tagged
// addresses to this function.
//
    addr = kasan_reset_tag(addr);
    if (WARN_ON!((unsigned long)addr & KASAN_GRANULE_MASK)) {
    return;
    }
    if (WARN_ON!(size & KASAN_GRANULE_MASK)) {
    return;
    }
    shadow_start = kasan_mem_to_shadow(addr);
    shadow_end = kasan_mem_to_shadow(addr + size);
    __memset(shadow_start, value, shadow_end - shadow_start);
    }
    EXPORT_SYMBOL_GPL(kasan_poison);

#[no_mangle]
pub unsafe extern "C" fn kasan_poison_last_granule(addr: *const c_void, size: usize) {
    if (!kasan_enabled()) {
    return;
    }
    if (size & KASAN_GRANULE_MASK) {
    let mut shadow = kasan_mem_to_shadow(addr + size);
// shadow = size & KASAN_GRANULE_MASK;
    }
    }

#[no_mangle]
pub unsafe extern "C" fn kasan_unpoison(addr: *const c_void, size: usize, init: bool) {
pub static mut tag: u8 = 0;
//
// Perform shadow offset calculation based on untagged address, as
// some of the callers (e.g. kasan_unpoison_new_object) pass tagged
// addresses to this function.
//
    addr = kasan_reset_tag(addr);
    if (WARN_ON!((unsigned long)addr & KASAN_GRANULE_MASK)) {
    return;
    }
// Unpoison all granules that cover the object.
    kasan_poison(addr, round_up(size, KASAN_GRANULE_SIZE), tag, false);
// Partially poison the last granule for the generic mode.
    if (IS_ENABLED!(CONFIG_KASAN_GENERIC)) {
    kasan_poison_last_granule(addr, size);
    }
    }

#[no_mangle]
unsafe extern "C" fn shadow_mapped(addr: c_ulong) -> bool {
    let mut pgd = pgd_offset_k(addr);
pub static mut p4d: *mut c_void = core::ptr::null_mut();
pub static mut pud: *mut c_void = core::ptr::null_mut();
pub static mut pmd: *mut c_void = core::ptr::null_mut();
pub static mut pte: *mut c_void = core::ptr::null_mut();
    if (pgd_none(*pgd)) {
    return false;
    }
    p4d = p4d_offset(pgd, addr);
    if (p4d_none(*p4d)) {
    return false;
    }
    pud = pud_offset(p4d, addr);
    if (pud_none(*pud)) {
    return false;
    }
    if (pud_leaf(*pud)) {
    return true;
    }
    pmd = pmd_offset(pud, addr);
    if (pmd_none(*pmd)) {
    return false;
    }
    if (pmd_leaf(*pmd)) {
    return true;
    }
    pte = pte_offset_kernel(pmd, addr);
    return !pte_none(ptep_get(pte));
    }
    static int __meminit kasan_mem_notifier(notifier_block *nb,
    unsigned long action, void *data)
    {
    let mut mem_data = data;
    unsigned long nr_shadow_pages, start_kaddr, shadow_start;
    unsigned long shadow_end, shadow_size;
    nr_shadow_pages = mem_data.nr_pages >> KASAN_SHADOW_SCALE_SHIFT;
    start_kaddr = (unsigned long)pfn_to_kaddr(mem_data.start_pfn);
    shadow_start = (unsigned long)kasan_mem_to_shadow(start_kaddr);
    shadow_size = nr_shadow_pages << PAGE_SHIFT;
    shadow_end = shadow_start + shadow_size;
    if (WARN_ON!(mem_data.nr_pages % KASAN_GRANULE_SIZE) ||
    WARN_ON!(start_kaddr % KASAN_MEMORY_PER_SHADOW_PAGE)) {
    return NOTIFY_BAD;
    }
    match (action) {
    MEM_GOING_ONLINE => {
pub static mut ret: *mut c_void = core::ptr::null_mut();
//
// If shadow is mapped already than it must have been mapped
// during the boot. This could happen if we onlining previously
// offlined memory.
//
    if (shadow_mapped(shadow_start)) {
    return NOTIFY_OK;
    }
    ret = __vmalloc_node_range(shadow_size, PAGE_SIZE, shadow_start,
    shadow_end, GFP_KERNEL,
    PAGE_KERNEL, VM_NO_GUARD,
    pfn_to_nid(mem_data.start_pfn),
    __builtin_return_address(0));
    if (!ret) {
    return NOTIFY_BAD;
    }
    kmemleak_ignore(ret);
    return NOTIFY_OK;
    }
    }
    case MEM_CANCEL_ONLINE:
    case MEM_OFFLINE: {
pub static mut vm: *mut c_void = core::ptr::null_mut();
//
// shadow_start was either mapped during boot by kasan_init()
// or during memory online by __vmalloc_node_range().
// In the latter case we can use vfree() to free shadow.
// Non-NULL result of the find_vm_area() will tell us if
// that was the second case.
//
// Currently it's not possible to free shadow mapped
// during boot by kasan_init(). It's because the code
// to do that hasn't been written yet. So we'll just
// leak the memory.
//
    vm = find_vm_area(shadow_start);
    if (vm) {
    vfree(shadow_start);
    }
    }
    }
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn kasan_memhotplug_init() -> c_int {
    hotplug_memory_notifier(kasan_mem_notifier, DEFAULT_CALLBACK_PRI);
    return 0;
    }
    core_initcall!(kasan_memhotplug_init);

    void __init __weak kasan_populate_early_vm_area_shadow(void *start,
    unsigned long size)
    {
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmalloc_populate_data {
    pub start: c_ulong,
    pub pages: *mut page,
}

#[no_mangle]
pub unsafe extern "C" fn kasan_populate_vmalloc_pte(ptep: *mut pte_t, addr: c_ulong, _data: *mut c_void) -> c_int {
    let mut data = _data;
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut pte;
    let mut index = 0;
    lazy_mmu_mode_pause();
    index = PFN_DOWN(addr - data.start);
    page = data.pages[index];
    __memset(page_to_virt(page), KASAN_VMALLOC_INVALID, PAGE_SIZE);
    pte = pfn_pte(page_to_pfn(page), PAGE_KERNEL);
    spin_lock(&init_mm.page_table_lock);
    if (likely(pte_none(ptep_get(ptep)))) {
    set_pte_at(&init_mm, addr, ptep, pte);
    data.pages[index] = core::ptr::null_mut();
    }
    spin_unlock(&init_mm.page_table_lock);
    lazy_mmu_mode_resume();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ___free_pages_bulk(pages: *mut page, nr_pages: c_int) {
    let mut i = 0;
    while (i < nr_pages) {
    if (pages[i]) {
    __free_pages(pages[i], 0);
    pages[i] = core::ptr::null_mut();
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn ___alloc_pages_bulk(pages: *mut page, nr_pages: c_int, gfp_mask: gfp_t) -> c_int {
    unsigned long nr_populated, nr_total = nr_pages;
    let mut page_array = pages;
    while (nr_pages) {
    nr_populated = alloc_pages_bulk(gfp_mask, nr_pages, pages);
    if (!nr_populated) {
    ___free_pages_bulk(page_array, nr_total - nr_pages);
    return -ENOMEM;
    }
    pages += nr_populated;
    nr_pages -= nr_populated;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __kasan_populate_vmalloc_do(start: c_ulong, end: c_ulong, gfp_mask: gfp_t) -> c_int {
    unsigned long nr_pages, nr_total = PFN_UP(end - start);
pub static mut data: usize = 0;
    let mut flags = 0;
pub static mut ret: c_int = 0;
    data.pages = __get_free_page(gfp_mask | __GFP_ZERO);
    if (!data.pages) {
    return -ENOMEM;
    }
    while (nr_total) {
    nr_pages = min(nr_total, PAGE_SIZE / sizeof!(data.pages[0]));
    ret = ___alloc_pages_bulk(data.pages, nr_pages, gfp_mask);
    if (ret) {
    break;
    }
    data.start = start;
//
// page tables allocations ignore external gfp mask, enforce it
// by the scope API
//
    flags = memalloc_apply_gfp_scope(gfp_mask);
    ret = apply_to_page_range(&init_mm, start, nr_pages * PAGE_SIZE,
    kasan_populate_vmalloc_pte, &data);
    memalloc_restore_scope(flags);
    ___free_pages_bulk(data.pages, nr_pages);
    if (ret) {
    break;
    }
    start += nr_pages * PAGE_SIZE;
    nr_total -= nr_pages;
    }
    free_page((unsigned long)data.pages);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_populate_vmalloc(addr: c_ulong, size: c_ulong, gfp_mask: gfp_t) -> c_int {
    unsigned long shadow_start, shadow_end;
    let mut ret = 0;
    if (!is_vmalloc_or_module_addr(addr)) {
    return 0;
    }
    shadow_start = (unsigned long)kasan_mem_to_shadow(addr);
    shadow_end = (unsigned long)kasan_mem_to_shadow(addr + size);
//
// User Mode Linux maps enough shadow memory for all of virtual memory
// at boot, so doesn't need to allocate more on vmalloc, just clear it.
//
// The remaining CONFIG_UML checks in this file exist for the same
// reason.
//
    if (IS_ENABLED!(CONFIG_UML)) {
    __memset(shadow_start, KASAN_VMALLOC_INVALID, shadow_end - shadow_start);
    return 0;
    }
    shadow_start = PAGE_ALIGN_DOWN(shadow_start);
    shadow_end = PAGE_ALIGN(shadow_end);
    ret = __kasan_populate_vmalloc_do(shadow_start, shadow_end, gfp_mask);
    if (ret) {
    return ret;
    }
    flush_cache_vmap(shadow_start, shadow_end);
//
// We need to be careful about inter-cpu effects here. Consider:
//
// CPU#0				  CPU#1
// WRITE_ONCE(p, vmalloc(100));		while (x = READ_ONCE(p)) ;
// p[99] = 1;
//
// With compiler instrumentation, that ends up looking like this:
//
// CPU#0				  CPU#1
// // vmalloc() allocates memory
// // let a = area->addr
// // we reach kasan_populate_vmalloc
// // and call kasan_unpoison:
// STORE shadow(a), unpoison_val
// ...
// STORE shadow(a+99), unpoison_val	x = LOAD p
// // rest of vmalloc process		<data dependency>
// STORE p, a				LOAD shadow(x+99)
//
// If there is no barrier between the end of unpoisoning the shadow
// and the store of the result to p, the stores could be committed
// in a different order by CPU#0, and CPU#1 could erroneously observe
// poison in the shadow.
//
// We need some sort of barrier between the stores.
//
// In the vmalloc() case, this is provided by a smp_wmb() in
// clear_vm_uninitialized_flag(). In the per-cpu allocator and in
// get_vm_area() and friends, the caller gets shadow allocated but
// doesn't have any pages mapped into the virtual address space that
// has been reserved. Mapping those pages in will involve taking and
// releasing a page-table lock, which will provide the barrier.
//
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_depopulate_vmalloc_pte(ptep: *mut pte_t, addr: c_ulong, unused: *mut c_void) -> c_int {
    let mut pte;
    let mut none = 0;
    lazy_mmu_mode_pause();
    spin_lock(&init_mm.page_table_lock);
    pte = ptep_get(ptep);
    none = pte_none(pte);
    if (likely(!none)) {
    pte_clear(&init_mm, addr, ptep);
    }
    spin_unlock(&init_mm.page_table_lock);
    if (likely(!none)) {
    __free_page(pfn_to_page(pte_pfn(pte)));
    }
    lazy_mmu_mode_resume();
    return 0;
    }
//
// Release the backing for the vmalloc region [start, end), which
// lies within the free region [free_region_start, free_region_end).
//
// This can be run lazily, long after the region was freed. It runs
// under vmap_area_lock, so it's not safe to interact with the vmalloc/vmap
// infrastructure.
//
// How does this work?
// -------------------
//
// We have a region that is page aligned, labeled as A.
// That might not map onto the shadow in a way that is page-aligned:
//
// start                     end
// v                         v
// |????????|????????|AAAAAAAA|AA....AA|AAAAAAAA|????????| < vmalloc
// -------- -------- --------          -------- --------
// |        |       |                 |        |
// |        |       |         /-------/        |
// \-------\|/------/         |/---------------
// |||                ||
// |??AAAAAA|AAAAAAAA|AA??????|                < shadow
// (1)      (2)      (3)
//
// First we align the start upwards and the end downwards, so that the
// shadow of the region aligns with shadow page boundaries. In the
// example, this gives us the shadow page (2). This is the shadow entirely
// covered by this allocation.
//
// Then we have the tricky bits. We want to know if we can free the
// partially covered shadow pages - (1) and (3) in the example. For this,
// we are given the start and end of the free region that contains this
// allocation. Extending our previous example, we could have:
//
// free_region_start                                    free_region_end
// |                 start                     end      |
// v                 v                         v        v
// |FFFFFFFF|FFFFFFFF|AAAAAAAA|AA....AA|AAAAAAAA|FFFFFFFF| < vmalloc
// -------- -------- --------          -------- --------
// |        |       |                 |        |
// |        |       |         /-------/        |
// \-------\|/------/         |/---------------
// |||                ||
// |FFAAAAAA|AAAAAAAA|AAF?????|                < shadow
// (1)      (2)      (3)
//
// Once again, we align the start of the free region up, and the end of
// the free region down so that the shadow is page aligned. So we can free
// page (1) - we know no allocation currently uses anything in that page,
// because all of it is in the vmalloc free region. But we cannot free
// page (3), because we can't be sure that the rest of it is unused.
//
// We only consider pages that contain part of the original region for
// freeing: we don't try to free other pages from the free region or we'd
// end up trying to free huge chunks of virtual address space.
//
// Concurrency
// -----------
//
// How do we know that we're not freeing a page that is simultaneously
// being used for a fresh allocation in kasan_populate_vmalloc(_pte)?
//
// We _can_ have kasan_release_vmalloc and kasan_populate_vmalloc running
// at the same time. While we run under free_vmap_area_lock, the population
// code does not.
//
// free_vmap_area_lock instead operates to ensure that the larger range
// [free_region_start, free_region_end) is safe: because __alloc_vmap_area and
// the per-cpu region-finding algorithm both run under free_vmap_area_lock,
// no space identified as free will become used while we are running. This
// means that so long as we are careful with alignment and only free shadow
// pages entirely covered by the free region, we will not run in to any
// trouble - any simultaneous allocations will be for disjoint regions.
//
#[no_mangle]
pub unsafe extern "C" fn __kasan_release_vmalloc(start: c_ulong, end: c_ulong, free_region_start: c_ulong, free_region_end: c_ulong, flags: c_ulong) {
    let mut shadow_start = core::ptr::null_mut();
    let mut shadow_end = core::ptr::null_mut();
    unsigned long region_start, region_end;
    let mut size = 0;
    region_start = ALIGN(start, KASAN_MEMORY_PER_SHADOW_PAGE);
    region_end = ALIGN_DOWN(end, KASAN_MEMORY_PER_SHADOW_PAGE);
    free_region_start = ALIGN(free_region_start, KASAN_MEMORY_PER_SHADOW_PAGE);
    if (start != region_start &&
    free_region_start < region_start) {
    region_start -= KASAN_MEMORY_PER_SHADOW_PAGE;
    }
    free_region_end = ALIGN_DOWN(free_region_end, KASAN_MEMORY_PER_SHADOW_PAGE);
    if (end != region_end &&
    free_region_end > region_end) {
    region_end += KASAN_MEMORY_PER_SHADOW_PAGE;
    }
    shadow_start = kasan_mem_to_shadow(region_start);
    shadow_end = kasan_mem_to_shadow(region_end);
    if (shadow_end > shadow_start) {
    size = shadow_end - shadow_start;
    if (IS_ENABLED!(CONFIG_UML)) {
    __memset(shadow_start, KASAN_SHADOW_INIT, shadow_end - shadow_start);
    return;
    }
    if (flags & KASAN_VMALLOC_PAGE_RANGE) {
    apply_to_existing_page_range(&init_mm,
    (unsigned long)shadow_start,
    size, kasan_depopulate_vmalloc_pte,
    core::ptr::null_mut());
    }
    if (flags & KASAN_VMALLOC_TLB_FLUSH) {
    flush_tlb_kernel_range((unsigned long)shadow_start,
    (unsigned long)shadow_end);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_unpoison_vmalloc(start: *mut c_void, size: c_ulong, flags: kasan_vmalloc_flags_t) -> *mut c_void {
//
// Software KASAN modes unpoison both VM_ALLOC and non-VM_ALLOC
// mappings, so the KASAN_VMALLOC_VM_ALLOC flag is ignored.
// Software KASAN modes can't optimize zeroing memory by combining it
// with setting memory tags, so the KASAN_VMALLOC_INIT flag is ignored.
//
    if (!is_vmalloc_or_module_addr(start)) {
    return start;
    }
//
// Don't tag executable memory with the tag-based mode.
// The kernel doesn't tolerate having the PC register tagged.
//
    if (IS_ENABLED!(CONFIG_KASAN_SW_TAGS) &&
    !(flags & KASAN_VMALLOC_PROT_NORMAL)) {
    return start;
    }
    if (unlikely(!(flags & KASAN_VMALLOC_KEEP_TAG))) {
    start = set_tag(start, kasan_random_tag());
    }
    kasan_unpoison(start, size, false);
    return start;
    }
//
// Poison the shadow for a vmalloc region. Called as part of the
// freeing process at the time the region is freed.
//
#[no_mangle]
pub unsafe extern "C" fn __kasan_poison_vmalloc(start: *const c_void, size: c_ulong) {
    if (!is_vmalloc_or_module_addr(start)) {
    return;
    }
    size = round_up(size, KASAN_GRANULE_SIZE);
    kasan_poison(start, size, KASAN_VMALLOC_INVALID, false);
    }

#[no_mangle]
pub unsafe extern "C" fn kasan_alloc_module_shadow(addr: *mut c_void, size: usize, gfp_mask: gfp_t) -> c_int {
pub static mut ret: *mut c_void = core::ptr::null_mut();
    let mut scaled_size = 0;
    let mut shadow_size = 0;
    let mut shadow_start = 0;
    shadow_start = (unsigned long)kasan_mem_to_shadow(addr);
    scaled_size = (size + KASAN_GRANULE_SIZE - 1) >>
    KASAN_SHADOW_SCALE_SHIFT;
    shadow_size = round_up(scaled_size, PAGE_SIZE);
    if (WARN_ON!(!PAGE_ALIGNED(shadow_start))) {
    return -EINVAL;
    }
    if (IS_ENABLED!(CONFIG_UML)) {
    __memset(shadow_start, KASAN_SHADOW_INIT, shadow_size);
    return 0;
    }
    ret = __vmalloc_node_range(shadow_size, 1, shadow_start,
    shadow_start + shadow_size,
    GFP_KERNEL,
    PAGE_KERNEL, VM_NO_GUARD, NUMA_NO_NODE,
    __builtin_return_address(0));
    if (ret) {
    let mut vm = find_vm_area(addr);
    __memset(ret, KASAN_SHADOW_INIT, shadow_size);
    vm.flags |= VM_KASAN;
    kmemleak_ignore(ret);
    if (vm.flags & VM_DEFER_KMEMLEAK) {
    kmemleak_vmalloc(vm, size, gfp_mask);
    }
    return 0;
    }
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_free_module_shadow(vm: *const vm_struct) {
    if (IS_ENABLED!(CONFIG_UML)) {
    return;
    }
    if (vm.flags & VM_KASAN) {
    vfree(kasan_mem_to_shadow(vm.addr));
    }
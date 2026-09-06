//! Automatically rewritten from C to Rust
//! Source: mm/execmem.c
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
// Copyright (C) 2002 Richard Henderson
// Copyright (C) 2001 Rusty Russell, 2002, 2010 Rusty Russell IBM.
// Copyright (C) 2023 Luis Chamberlain <mcgrof@kernel.org>
// Copyright (C) 2024 Mike Rapoport IBM.
//

pub static mut execmem_info: *mut c_void = core::ptr::null_mut();
    static struct execmem_info default_execmem_info __ro_after_init;

#[no_mangle]
pub unsafe extern "C" fn execmem_vmalloc(range: *mut execmem_range, size: size_t, pgprot: pgprot_t, vm_flags: c_ulong) -> *mut c_void {
pub static mut kasan: bool = false;
pub static mut gfp_flags: gfp_t = 0;
pub static mut align: c_uint = 0;
pub static mut start: c_ulong = 0;
pub static mut end: c_ulong = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    if (kasan) {
    vm_flags |= VM_DEFER_KMEMLEAK;
    }
    p = __vmalloc_node_range(size, align, start, end, gfp_flags,
    pgprot, vm_flags, NUMA_NO_NODE,
    __builtin_return_address(0));
    if (!p && range.fallback_start) {
    start = range.fallback_start;
    end = range.fallback_end;
    p = __vmalloc_node_range(size, align, start, end, gfp_flags,
    pgprot, vm_flags, NUMA_NO_NODE,
    __builtin_return_address(0));
    }
    if (!p) {
    pr_warn_ratelimited("unable to allocate memory\n");
    return core::ptr::null_mut();
    }
    if (kasan && (kasan_alloc_module_shadow(p, size, GFP_KERNEL) < 0)) {
    vfree(p);
    return core::ptr::null_mut();
    }
    return p;
    }
#[no_mangle]
pub unsafe extern "C" fn execmem_vmap(size: size_t) -> *mut c_void {
    let mut range = &execmem_info.ranges[EXECMEM_MODULE_DATA];
pub static mut area: *mut c_void = core::ptr::null_mut();
    area = __get_vm_area_node(size, range.alignment, PAGE_SHIFT, VM_ALLOC,
    range.start, range.end, NUMA_NO_NODE,
    GFP_KERNEL, __builtin_return_address(0));
    if (!area && range.fallback_start) {
    area = __get_vm_area_node(size, range.alignment, PAGE_SHIFT, VM_ALLOC,
    range.fallback_start, range.fallback_end,
    NUMA_NO_NODE, GFP_KERNEL, __builtin_return_address(0));
    }
    return area;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: execmem_vmalloc
pub unsafe extern "C" fn execmem_vmalloc_dup(range: *mut execmem_range, size: size_t, pgprot: pgprot_t, vm_flags: c_ulong) -> *mut c_void {
    return vmalloc(size);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct execmem_cache {
    pub mutex: mutex,
    pub busy_areas: maple_tree,
    pub free_areas: maple_tree,
//     pub /: *mut *mut unsigned int pending_free_cnt; / protected by mutex,
}

// delay to schedule asynchronous free if fast path free fails

// mark entries in busy_areas that should be freed asynchronously

pub static mut execmem_cache: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn mas_range_len(mas: *mut ma_state) -> c_ulong {
    return mas.last - mas.index + 1;
    }
#[no_mangle]
unsafe extern "C" fn execmem_set_direct_map_valid(vm: *mut vm_struct, valid: bool) -> c_int {
pub static mut nr: c_uint = 0;
pub static mut updated: c_uint = 0;
pub static mut err: c_int = 0;
    while (i < vm.nr_pages) {
    err = set_direct_map_valid_noflush(vm.pages[i], nr, valid);
    if (err) {
// goto;
    }
    updated += nr;
    }
    return 0;
// label;
    for (int i = 0; i < updated; i += nr) {
    set_direct_map_valid_noflush(vm.pages[i], nr, !valid);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn execmem_force_rw(ptr: *mut c_void, size: usize) -> c_int {
pub static mut nr: c_uint = 0;
pub static mut addr: c_ulong = 0;
    let mut ret = 0;
    ret = set_memory_nx(addr, nr);
    if (ret) {
    return ret;
    }
    return set_memory_rw(addr, nr);
    }
#[no_mangle]
pub unsafe extern "C" fn execmem_restore_rox(ptr: *mut c_void, size: usize) -> c_int {
pub static mut nr: c_uint = 0;
pub static mut addr: c_ulong = 0;
    return set_memory_rox(addr, nr);
    }
#[no_mangle]
unsafe extern "C" fn execmem_cache_clean(work: *mut work_struct) {
    let mut free_areas = &execmem_cache.free_areas;
    let mut mutex = &execmem_cache.mutex;
    MA_STATE(mas, free_areas, 0, ULONG_MAX);
pub static mut area: *mut c_void = core::ptr::null_mut();
    mutex_lock(mutex);
    mas_for_each(&mas, area, ULONG_MAX) {
pub static mut size: usize = 0;
    if (IS_ALIGNED(size, PMD_SIZE) &&
    IS_ALIGNED(mas.index, PMD_SIZE)) {
    let mut vm = find_vm_area(area);
    execmem_set_direct_map_valid(vm, true);
    mas_store_gfp(&mas, core::ptr::null_mut(), GFP_KERNEL);
    vfree(area);
    }
    }
    mutex_unlock(mutex);
    }
pub static mut execmem_cache_clean_work: usize = 0;
#[no_mangle]
unsafe extern "C" fn execmem_cache_add_locked(ptr: *mut c_void, size: usize, gfp_mask: gfp_t) -> c_int {
    let mut free_areas = &execmem_cache.free_areas;
pub static mut addr: c_ulong = 0;
    MA_STATE(mas, free_areas, addr - 1, addr + 1);
    unsigned long lower, upper;
    let mut area = core::ptr::null_mut();
    lower = addr;
    upper = addr + size - 1;
    area = mas_walk(&mas);
    if (area && mas.last == addr - 1) {
    lower = mas.index;
    }
    area = mas_next(&mas, ULONG_MAX);
    if (area && mas.index == addr + size) {
    upper = mas.last;
    }
    mas_set_range(&mas, lower, upper);
    return mas_store_gfp(&mas, lower, gfp_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn within_range(range: *mut execmem_range, mas: *mut ma_state, size: size_t) -> bool {
pub static mut addr: c_ulong = 0;
    if (addr >= range.start && addr + size < range.end) {
    return true;
    }
    if (range.fallback_start &&
    addr >= range.fallback_start && addr + size < range.fallback_end) {
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn execmem_cache_alloc_locked(range: *mut execmem_range, size: size_t) -> *mut c_void {
    let mut free_areas = &execmem_cache.free_areas;
    let mut busy_areas = &execmem_cache.busy_areas;
    MA_STATE(mas_free, free_areas, 0, ULONG_MAX);
    MA_STATE(mas_busy, busy_areas, 0, ULONG_MAX);
    unsigned long addr, last, area_size = 0;
    void *area, *ptr = core::ptr::null_mut();
    let mut err = 0;
    mas_for_each(&mas_free, area, ULONG_MAX) {
    area_size = mas_range_len(&mas_free);
    if (area_size >= size && within_range(range, &mas_free, size)) {
    break;
    }
    }
    if (area_size < size) {
    return core::ptr::null_mut();
    }
    addr = mas_free.index;
    last = mas_free.last;
// insert allocated size to busy_areas at range [addr, addr + size)
    mas_set_range(&mas_busy, addr, addr + size - 1);
    err = mas_store_gfp(&mas_busy, addr, GFP_KERNEL);
    if (err) {
    return core::ptr::null_mut();
    }
    mas_store_gfp(&mas_free, core::ptr::null_mut(), GFP_KERNEL);
    if (area_size > size) {
    let mut ptr = (addr + size);
//
// re-insert remaining free size to free_areas at range
// [addr + size, last]
//
    mas_set_range(&mas_free, addr + size, last);
    err = mas_store_gfp(&mas_free, ptr, GFP_KERNEL);
    if (err) {
    mas_store_gfp(&mas_busy, core::ptr::null_mut(), GFP_KERNEL);
    return core::ptr::null_mut();
    }
    }
    ptr = addr;
    return ptr;
    }
#[no_mangle]
pub unsafe extern "C" fn __execmem_cache_alloc(range: *mut execmem_range, size: size_t) -> *mut c_void {
    guard(mutex)(&execmem_cache.mutex);
    return execmem_cache_alloc_locked(range, size);
    }
#[no_mangle]
pub unsafe extern "C" fn execmem_cache_populate_alloc(range: *mut execmem_range, size: size_t) -> *mut c_void {
pub static mut vm_flags: c_ulong = 0;
    let mut mutex = &execmem_cache.mutex;
pub static mut vm: *mut c_void = core::ptr::null_mut();
    let mut alloc_size = 0;
pub static mut err: c_int = 0;
pub static mut p: *mut c_void = core::ptr::null_mut();
    alloc_size = round_up(size, PMD_SIZE);
    p = execmem_vmalloc(range, alloc_size, PAGE_KERNEL, vm_flags);
    if (!p) {
    alloc_size = size;
    p = execmem_vmalloc(range, alloc_size, PAGE_KERNEL, vm_flags);
    }
    if (!p) {
    return core::ptr::null_mut();
    }
    vm = find_vm_area(p);
    if (!vm) {
// goto;
    }
// fill memory with instructions that will trap
    execmem_fill_trapping_insns(p, alloc_size);
    err = set_memory_rox((unsigned long)p, vm.nr_pages);
    if (err) {
// goto;
    }
//
// New memory blocks must be allocated and added to the cache
// as an atomic operation, otherwise they may be consumed
// by a parallel call to the execmem_cache_alloc function.
//
    mutex_lock(mutex);
    err = execmem_cache_add_locked(p, alloc_size, GFP_KERNEL);
    if (err) {
// goto;
    }
    p = execmem_cache_alloc_locked(range, size);
    mutex_unlock(mutex);
    return p;
// label;
    mutex_unlock(mutex);
    execmem_set_direct_map_valid(vm, true);
// label;
    vfree(p);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn execmem_cache_alloc(range: *mut execmem_range, size: size_t) -> *mut c_void {
pub static mut p: *mut c_void = core::ptr::null_mut();
    p = __execmem_cache_alloc(range, size);
    if (p) {
    return p;
    }
    return execmem_cache_populate_alloc(range, size);
    }
#[no_mangle]
pub unsafe extern "C" fn is_pending_free(ptr: *mut c_void) -> bool {
    return ((unsigned long)ptr & PENDING_FREE_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn pending_free_set(ptr: *mut c_void) -> *mut c_void {
    return ((unsigned long)ptr | PENDING_FREE_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn pending_free_clear(ptr: *mut c_void) -> *mut c_void {
    return ((unsigned long)ptr & ~PENDING_FREE_MASK);
    }
#[no_mangle]
unsafe extern "C" fn __execmem_cache_free(mas: *mut ma_state, ptr: *mut c_void, gfp_mask: gfp_t) -> c_int {
pub static mut size: usize = 0;
    let mut err = 0;
    err = execmem_force_rw(ptr, size);
    if (err) {
    return err;
    }
    execmem_fill_trapping_insns(ptr, size);
    execmem_restore_rox(ptr, size);
    err = execmem_cache_add_locked(ptr, size, gfp_mask);
    if (err) {
    return err;
    }
    mas_store_gfp(mas, core::ptr::null_mut(), gfp_mask);
    return 0;
    }
// forward_decl: execmem_cache_free_slow;
pub static mut execmem_cache_free_work: usize = 0;
#[no_mangle]
unsafe extern "C" fn execmem_cache_free_slow(work: *mut work_struct) {
    let mut busy_areas = &execmem_cache.busy_areas;
    MA_STATE(mas, busy_areas, 0, ULONG_MAX);
pub static mut area: *mut c_void = core::ptr::null_mut();
    guard(mutex)(&execmem_cache.mutex);
    if (!execmem_cache.pending_free_cnt) {
    return;
    }
    mas_for_each(&mas, area, ULONG_MAX) {
    if (!is_pending_free(area)) {
    continue;
    }
    area = pending_free_clear(area);
    if (__execmem_cache_free(&mas, area, GFP_KERNEL)) {
    continue;
    }
    execmem_cache.pending_free_cnt -= 1;
    }
    if (execmem_cache.pending_free_cnt) {
    schedule_delayed_work(&execmem_cache_free_work, FREE_DELAY);
    }
    else {
    schedule_work(&execmem_cache_clean_work);
    }
    }
#[no_mangle]
unsafe extern "C" fn execmem_cache_free(ptr: *mut c_void) -> bool {
    let mut busy_areas = &execmem_cache.busy_areas;
pub static mut addr: c_ulong = 0;
    MA_STATE(mas, busy_areas, addr, addr);
pub static mut area: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    guard(mutex)(&execmem_cache.mutex);
    area = mas_walk(&mas);
    if (!area) {
    return false;
    }
    err = __execmem_cache_free(&mas, area, GFP_KERNEL | __GFP_NORETRY);
    if (err) {
//
// mas points to exact slot we've got the area from, nothing
// else can modify the tree because of the mutex, so there
// won't be any allocations in mas_store_gfp() and it will just
// change the pointer.
//
    area = pending_free_set(area);
    mas_store_gfp(&mas, area, GFP_KERNEL);
    execmem_cache.pending_free_cnt += 1;
    schedule_delayed_work(&execmem_cache_free_work, FREE_DELAY);
    return true;
    }
    schedule_work(&execmem_cache_clean_work);
    return true;
    }

//
// when ROX cache is not used the permissions defined by architectures for
// execmem ranges that are updated before use (e.g. EXECMEM_MODULE_TEXT) must
// be writable anyway
//
#[no_mangle]
pub unsafe extern "C" fn execmem_force_rw(ptr: *mut c_void, size: usize) -> c_int {
    return 0;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: execmem_cache_alloc
pub unsafe extern "C" fn execmem_cache_alloc_dup(range: *mut execmem_range, size: size_t) -> *mut c_void {
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn execmem_cache_free(ptr: *mut c_void) -> bool {
    return false;
    }

#[no_mangle]
pub unsafe extern "C" fn execmem_alloc(type: execmem_type, size: size_t) -> *mut c_void {
    let mut range = &execmem_info.ranges[type];
pub static mut use_cache: bool = false;
pub static mut vm_flags: c_ulong = 0;
pub static mut pgprot: pgprot_t = 0;
    let mut p = core::ptr::null_mut();
    size = PAGE_ALIGN(size);
    if (use_cache) {
    p = execmem_cache_alloc(range, size);
    }
    else {
    p = execmem_vmalloc(range, size, pgprot, vm_flags);
    }
    return kasan_reset_tag(p);
    }
#[no_mangle]
pub unsafe extern "C" fn execmem_alloc_rw(type: execmem_type, size: size_t) -> *mut c_void {
    void *p __free(execmem) = execmem_alloc(type, size);
    let mut err = 0;
    if (!p) {
    return core::ptr::null_mut();
    }
    err = execmem_force_rw(p, size);
    if (err) {
    return core::ptr::null_mut();
    }
    return no_free_ptr(p);
    }
#[no_mangle]
pub unsafe extern "C" fn execmem_free(ptr: *mut c_void) {
//
// This memory may be RO, and freeing RO memory in an interrupt is not
// supported by vmalloc.
//
    WARN_ON!(in_interrupt());
    if (!execmem_cache_free(ptr)) {
    vfree(ptr);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn execmem_is_rox(type: execmem_type) -> bool {
    return !!(execmem_info.ranges[type].flags & EXECMEM_ROX_CACHE);
    }
#[no_mangle]
unsafe extern "C" fn execmem_validate(info: *mut execmem_info) -> bool {
    let mut r = &info.ranges[EXECMEM_DEFAULT];
    if (!r.alignment || !r.start || !r.end || !pgprot_val(r.pgprot)) {
    pr_crit("Invalid parameters for execmem allocator, module loading will fail");
    return false;
    }
    if (!IS_ENABLED!(CONFIG_ARCH_HAS_EXECMEM_ROX)) {
    while (i < EXECMEM_TYPE_MAX) {
    r = &info.ranges[i];
    if (r.flags & EXECMEM_ROX_CACHE) {
    pr_warn_once("ROX cache is not supported\n");
    r.flags &= ~EXECMEM_ROX_CACHE;
    }
    }
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn execmem_init_missing(info: *mut execmem_info) {
    let mut default_range = &info.ranges[EXECMEM_DEFAULT];
    while (i < EXECMEM_TYPE_MAX) {
    let mut r = &info.ranges[i];
    if (!r.start) {
    if (i == EXECMEM_MODULE_DATA) {
    r.pgprot = PAGE_KERNEL;
    }
    else {
    r.pgprot = default_range.pgprot;
    }
    r.alignment = default_range.alignment;
    r.start = default_range.start;
    r.end = default_range.end;
    r.flags = default_range.flags;
    r.fallback_start = default_range.fallback_start;
    r.fallback_end = default_range.fallback_end;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn execmem_arch_setup() -> *mut execmem_info  __weak {
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn __execmem_init()  {
    let mut info = execmem_arch_setup();
    if (!info) {
    info = execmem_info = &default_execmem_info;
    info.ranges[EXECMEM_DEFAULT].start = VMALLOC_START;
    info.ranges[EXECMEM_DEFAULT].end = VMALLOC_END;
    info.ranges[EXECMEM_DEFAULT].pgprot = PAGE_KERNEL_EXEC;
    info.ranges[EXECMEM_DEFAULT].alignment = 1;
    }
    if (!execmem_validate(info)) {
    return;
    }
    execmem_init_missing(info);
    execmem_info = info;
    }

#[no_mangle]
unsafe extern "C" fn execmem_late_init() -> c_int {
    __execmem_init();
    return 0;
    }
    core_initcall!(execmem_late_init);

#[no_mangle]
pub unsafe extern "C" fn execmem_init()  {
    __execmem_init();
    }
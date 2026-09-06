//! Automatically rewritten from C to Rust
//! Source: mm/kasan/generic.c
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
// This file contains core generic KASAN code.
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// Author: Andrey Ryabinin <ryabinin.a.a@gmail.com>
//
// Some code borrowed from https://github.com/xairy/kasan-prototype by
// Andrey Konovalov <andreyknvl@gmail.com>
//

//
// Initialize Generic KASAN and enable runtime checks.
// This should be called from arch kasan_init() once shadow memory is ready.
//
#[no_mangle]
pub unsafe extern "C" fn kasan_init_generic()  {
    kasan_enable();
    pr_info!("KernelAddressSanitizer initialized (generic)\n");
    }
//
// All functions below always inlined so compiler could
// perform better optimizations in each of __asan_loadX/__assn_storeX
// depending on memory access size X.
//
#[no_mangle]
unsafe extern "C" fn memory_is_poisoned_1(addr: *const c_void) -> __always_inline bool {
pub static mut shadow_value: i8 = 0;
    if (unlikely(shadow_value)) {
pub static mut last_accessible_byte: i8 = 0;
    return unlikely(last_accessible_byte >= shadow_value);
    }
    return false;
    }
    static __always_inline bool memory_is_poisoned_2_4_8(const void *addr,
    unsigned long size)
    {
    let mut shadow_addr = kasan_mem_to_shadow(addr);
//
// Access crosses 8(shadow size)-byte boundary. Such access maps
// into 2 shadow bytes, so we need to check them both.
//
    if (unlikely((((unsigned long)addr + size - 1) & KASAN_GRANULE_MASK) < size - 1)) {
    return *shadow_addr || memory_is_poisoned_1(addr + size - 1);
    }
    return memory_is_poisoned_1(addr + size - 1);
    }
#[no_mangle]
unsafe extern "C" fn memory_is_poisoned_16(addr: *const c_void) -> __always_inline bool {
    let mut shadow_addr = kasan_mem_to_shadow(addr);
// Unaligned 16-bytes access maps into 3 shadow bytes.
    if (unlikely(!IS_ALIGNED((unsigned long)addr, KASAN_GRANULE_SIZE))) {
    return *shadow_addr || memory_is_poisoned_1(addr + 15);
    }
pub static mut shadow_addr: *mut c_void = core::ptr::null_mut();
    }
    static __always_inline unsigned long bytes_is_nonzero(const u8 *start,
    size_t size)
    {
    while (size) {
    if (unlikely(*start)) {
    return (unsigned long)start;
    }
    start += 1;
    size -= 1;
    }
    return 0;
    }
    static __always_inline unsigned long memory_is_nonzero(const void *start,
    const void *end)
    {
    let mut words = 0;
    let mut ret = 0;
pub static mut prefix: c_uint = 0;
    if (end - start <= 16) {
    return bytes_is_nonzero(start, end - start);
    }
    if (prefix) {
    prefix = 8 - prefix;
    ret = bytes_is_nonzero(start, prefix);
    if (unlikely(ret)) {
    return ret;
    }
    start += prefix;
    }
    words = (end - start) / 8;
    while (words) {
    if (unlikely(*start)) {
    return bytes_is_nonzero(start, 8);
    }
    start += 8;
    words -= 1;
    }
    return bytes_is_nonzero(start, (end - start) % 8);
    }
#[no_mangle]
unsafe extern "C" fn memory_is_poisoned_n(addr: *const c_void, size: usize) -> __always_inline bool {
    let mut ret = 0;
    ret = memory_is_nonzero(kasan_mem_to_shadow(addr),
    kasan_mem_to_shadow(addr + size - 1) + 1);
    if (unlikely(ret)) {
    let mut last_byte = addr + size - 1;
    let mut last_shadow = kasan_mem_to_shadow(last_byte);
pub static mut last_accessible_byte: i8 = 0;
    if (unlikely(ret != (unsigned long)last_shadow ||
    last_accessible_byte >= *last_shadow)) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn memory_is_poisoned(addr: *const c_void, size: usize) -> __always_inline bool {
    if (__builtin_constant_p(size)) {
    match (size) {
    1 => {
    return memory_is_poisoned_1(addr);
    }
    2 => {
    }
    4 => {
    }
    8 => {
    return memory_is_poisoned_2_4_8(addr, size);
    }
    16 => {
    return memory_is_poisoned_16(addr);
    }
    _ => {
    BUILD_BUG();
    }
    }
    }
    return memory_is_poisoned_n(addr, size);
    }
    static __always_inline bool check_region_inline(const void *addr,
    size_t size, bool write,
    unsigned long ret_ip)
    {
    if (!kasan_enabled()) {
    return true;
    }
    if (unlikely(size == 0)) {
    return true;
    }
    if (unlikely(addr + size < addr)) {
    return !kasan_report(addr, size, write, ret_ip);
    }
    if (unlikely(!addr_has_metadata(addr))) {
    return !kasan_report(addr, size, write, ret_ip);
    }
    if (likely(!memory_is_poisoned(addr, size))) {
    return true;
    }
    return !kasan_report(addr, size, write, ret_ip);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_check_range(addr: *mut c_void, size: size_t, write: bool, ret_ip: c_ulong) -> bool {
    return check_region_inline(addr, size, write, ret_ip);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_byte_accessible(addr: *const c_void) -> bool {
    let mut shadow_byte = 0;
    if (!kasan_enabled()) {
    return true;
    }
    shadow_byte = READ_ONCE(*kasan_mem_to_shadow(addr));
    return shadow_byte >= 0 && shadow_byte < KASAN_GRANULE_SIZE;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_cache_shrink(cache: *mut kmem_cache) {
    kasan_quarantine_remove_cache(cache);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_cache_shutdown(cache: *mut kmem_cache) {
    if (!__kmem_cache_empty(cache)) {
    kasan_quarantine_remove_cache(cache);
    }
    }
#[no_mangle]
unsafe extern "C" fn register_global(global: *mut kasan_global) {
pub static mut aligned_size: usize = 0;
    kasan_unpoison(global.beg, global.size, false);
    kasan_poison(global.beg + aligned_size,
    global.size_with_redzone - aligned_size,
    KASAN_GLOBAL_REDZONE, false);
    }
#[no_mangle]
pub unsafe extern "C" fn __asan_register_globals(ptr: *mut c_void, size: isize) {
    let mut i = 0;
    let mut globals = ptr;
    for (i = 0; i < size; i++) {
    register_global(&globals[i]);
    }
    }
    EXPORT_SYMBOL(__asan_register_globals);
#[no_mangle]
pub unsafe extern "C" fn __asan_unregister_globals(ptr: *mut c_void, size: isize) {
    }
    EXPORT_SYMBOL(__asan_unregister_globals);

    void __asan_load##size(void *addr)				
    {								
    check_region_inline(addr, size, false, _RET_IP_);	
    }								
    EXPORT_SYMBOL(__asan_load##size);				
    __alias(__asan_load##size)					
    void __asan_load##size##_noabort;			
    EXPORT_SYMBOL(__asan_load##size##_noabort);			
    void __asan_store##size(void *addr)				
    {								
    check_region_inline(addr, size, true, _RET_IP_);	
    }								
    EXPORT_SYMBOL(__asan_store##size);				
    __alias(__asan_store##size)					
    void __asan_store##size##_noabort;			
    EXPORT_SYMBOL(__asan_store##size##_noabort)
pub static mut 1: usize = 0;
pub static mut 2: usize = 0;
pub static mut 4: usize = 0;
pub static mut 8: usize = 0;
pub static mut 16: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn __asan_loadN(addr: *mut c_void, size: isize) {
    kasan_check_range(addr, size, false, _RET_IP_);
    }
    EXPORT_SYMBOL(__asan_loadN);
    __alias(__asan_loadN)
// forward_decl: __asan_loadN_noabort;
    EXPORT_SYMBOL(__asan_loadN_noabort);
#[no_mangle]
pub unsafe extern "C" fn __asan_storeN(addr: *mut c_void, size: isize) {
    kasan_check_range(addr, size, true, _RET_IP_);
    }
    EXPORT_SYMBOL(__asan_storeN);
    __alias(__asan_storeN)
// forward_decl: __asan_storeN_noabort;
    EXPORT_SYMBOL(__asan_storeN_noabort);
// to shut up compiler complaints
#[no_mangle]
pub unsafe extern "C" fn __asan_handle_no_return() {}
    EXPORT_SYMBOL(__asan_handle_no_return);
// Emitted by compiler to poison alloca()ed objects.
#[no_mangle]
pub unsafe extern "C" fn __asan_alloca_poison(addr: *mut c_void, size: isize) {
pub static mut rounded_up_size: usize = 0;
    size_t padding_size = round_up(size, KASAN_ALLOCA_REDZONE_SIZE) -
    rounded_up_size;
pub static mut rounded_down_size: usize = 0;
    let mut left_redzone = (addr -
    KASAN_ALLOCA_REDZONE_SIZE);
    let mut right_redzone = (addr + rounded_up_size);
    WARN_ON!(!IS_ALIGNED((unsigned long)addr, KASAN_ALLOCA_REDZONE_SIZE));
    kasan_unpoison((addr + rounded_down_size),
    size - rounded_down_size, false);
    kasan_poison(left_redzone, KASAN_ALLOCA_REDZONE_SIZE,
    KASAN_ALLOCA_LEFT, false);
    kasan_poison(right_redzone, padding_size + KASAN_ALLOCA_REDZONE_SIZE,
    KASAN_ALLOCA_RIGHT, false);
    }
    EXPORT_SYMBOL(__asan_alloca_poison);
// Emitted by compiler to unpoison alloca()ed areas when the stack unwinds.
#[no_mangle]
pub unsafe extern "C" fn __asan_allocas_unpoison(stack_top: *mut c_void, stack_bottom: isize) {
    if (unlikely(!stack_top || stack_top > stack_bottom)) {
    return;
    }
    kasan_unpoison(stack_top, stack_bottom - stack_top, false);
    }
    EXPORT_SYMBOL(__asan_allocas_unpoison);
// Emitted by the compiler to [un]poison local variables.

    void __asan_set_shadow_##byte(const void *addr, ssize_t size)	
    {								
    __memset(addr, 0x##byte, size);			
    }								
    EXPORT_SYMBOL(__asan_set_shadow_##byte)
pub static mut 00: usize = 0;
pub static mut f1: usize = 0;
pub static mut f2: usize = 0;
pub static mut f3: usize = 0;
pub static mut f5: usize = 0;
pub static mut f8: usize = 0;
//
// Adaptive redzone policy taken from the userspace AddressSanitizer runtime.
// For larger allocations larger redzones are used.
//
#[no_mangle]
pub unsafe extern "C" fn optimal_redzone(object_size: c_uint) -> c_uint {
    return
    object_size <= 64        - 16   ? 16 :
    object_size <= 128       - 32   ? 32 :
    object_size <= 512       - 64   ? 64 :
    object_size <= 4096      - 128  ? 128 :
    object_size <= (1 << 14) - 256  ? 256 :
    object_size <= (1 << 15) - 512  ? 512 :
    object_size <= (1 << 16) - 1024 ? 1024 : 2048;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_cache_create(cache: *mut kmem_cache, size: *mut c_uint, flags: *mut slab_flags_t) {
    let mut ok_size = 0;
    let mut optimal_size = 0;
    let mut rem_free_meta_size = 0;
    let mut orig_alloc_meta_offset = 0;
    if (!kasan_requires_meta()) {
    return;
    }
//
// SLAB_KASAN is used to mark caches that are sanitized by KASAN and
// that thus have per-object metadata. Currently, this flag is used in
// slab_ksize() to account for per-object metadata when calculating the
// size of the accessible memory within the object. Additionally, we use
// SLAB_NO_MERGE to prevent merging of caches with per-object metadata.
//
// flags |= SLAB_KASAN | SLAB_NO_MERGE;
    ok_size = *size;
// Add alloc meta into the redzone.
    cache.kasan_info.alloc_meta_offset = *size;
// size += sizeof!(kasan_alloc_meta);
// If alloc meta doesn't fit, don't add it.
    if (*size > KMALLOC_MAX_SIZE) {
    cache.kasan_info.alloc_meta_offset = 0;
// size = ok_size;
// Continue, since free meta might still fit.
    }
    ok_size = *size;
    orig_alloc_meta_offset = cache.kasan_info.alloc_meta_offset;
//
// Store free meta in the redzone when it's not possible to store
// it in the object. This is the case when:
// 1. Object is SLAB_TYPESAFE_BY_RCU, which means that it can
// be touched after it was freed, or
// 2. Object has a constructor, which means it's expected to
// retain its content until the next allocation, or
// 3. It is from a kmalloc cache which enables the debug option
// to store original size.
//
    if ((cache.flags & SLAB_TYPESAFE_BY_RCU) || cache.ctor ||
    slub_debug_orig_size(cache)) {
    cache.kasan_info.free_meta_offset = *size;
// size += sizeof!(kasan_free_meta);
// goto;
    }
//
// Otherwise, if the object is large enough to contain free meta,
// store it within the object.
//
    if (sizeof!(kasan_free_meta) <= cache.object_size) {
// cache->kasan_info.free_meta_offset = 0 is implied.
// goto;
    }
//
// For smaller objects, store the beginning of free meta within the
// object and the end in the redzone. And thus shift the location of
// alloc meta to free up space for free meta.
// This is only possible when slub_debug is disabled, as otherwise
// the end of free meta will overlap with slub_debug metadata.
//
    if (!__slub_debug_enabled()) {
    rem_free_meta_size = sizeof!(kasan_free_meta) -
    cache.object_size;
// size += rem_free_meta_size;
    if (cache.kasan_info.alloc_meta_offset != 0) {
    cache.kasan_info.alloc_meta_offset += rem_free_meta_size;
    }
// goto;
    }
//
// If the object is small and slub_debug is enabled, store free meta
// in the redzone after alloc meta.
//
    cache.kasan_info.free_meta_offset = *size;
// size += sizeof!(kasan_free_meta);
// label;
// If free meta doesn't fit, don't add it.
    if (*size > KMALLOC_MAX_SIZE) {
    cache.kasan_info.free_meta_offset = KASAN_NO_FREE_META;
    cache.kasan_info.alloc_meta_offset = orig_alloc_meta_offset;
// size = ok_size;
    }
// Calculate size with optimal redzone.
    optimal_size = cache.object_size + optimal_redzone(cache.object_size);
// Limit it with KMALLOC_MAX_SIZE.
    if (optimal_size > KMALLOC_MAX_SIZE) {
    optimal_size = KMALLOC_MAX_SIZE;
    }
// Use optimal size if the size with added metas is not large enough.
    if (*size < optimal_size) {
// size = optimal_size;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_get_alloc_meta(cache: *mut kmem_cache, object: *mut c_void) -> *mut c_void {
    if (!cache.kasan_info.alloc_meta_offset) {
    return core::ptr::null_mut();
    }
    return object + cache.kasan_info.alloc_meta_offset;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_get_free_meta(cache: *mut kmem_cache, object: *mut c_void) -> *mut c_void {
    BUILD_BUG_ON!(sizeof!(kasan_free_meta) > 32);
    if (cache.kasan_info.free_meta_offset == KASAN_NO_FREE_META) {
    return core::ptr::null_mut();
    }
    return object + cache.kasan_info.free_meta_offset;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_init_object_meta(cache: *mut kmem_cache, object: *const c_void) {
pub static mut alloc_meta: *mut c_void = core::ptr::null_mut();
    alloc_meta = kasan_get_alloc_meta(cache, object);
    if (alloc_meta) {
// Zero out alloc meta to mark it as invalid.
    __memset(alloc_meta, 0, sizeof!(*alloc_meta));
    }
//
// Explicitly marking free meta as invalid is not required: the shadow
// value for the first 8 bytes of a newly allocated object is not
// KASAN_SLAB_FREE_META.
//
    }
#[no_mangle]
unsafe extern "C" fn release_alloc_meta(meta: *mut kasan_alloc_meta) {
// Zero out alloc meta to mark it as invalid.
    __memset(meta, 0, sizeof!(*meta));
    }
#[no_mangle]
unsafe extern "C" fn release_free_meta(object: *const c_void, meta: *mut kasan_free_meta) {
// Check if free meta is valid.
    if (*kasan_mem_to_shadow(object) != KASAN_SLAB_FREE_META) {
    return;
    }
// Mark free meta as invalid.
// kasan_mem_to_shadow(object) = KASAN_SLAB_FREE;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_metadata_size(cache: *mut kmem_cache, in_object: bool) -> usize {
    let mut info = &cache.kasan_info;
    if (!kasan_requires_meta()) {
    return 0;
    }
    if (in_object) {
    return (info.free_meta_offset ?
    0 : sizeof!(kasan_free_meta));
    }
    else {
    return (info.alloc_meta_offset ?
    sizeof!(kasan_alloc_meta) : 0) +
    ((info.free_meta_offset &&
    info.free_meta_offset != KASAN_NO_FREE_META) ?
    sizeof!(kasan_free_meta) : 0);
    }
    }
//
// This function avoids dynamic memory allocations and thus can be called from
// contexts that do not allow allocating memory.
//
#[no_mangle]
pub unsafe extern "C" fn kasan_record_aux_stack(addr: *mut c_void) {
    let mut slab = kasan_addr_to_slab(addr);
pub static mut cache: *mut c_void = core::ptr::null_mut();
pub static mut alloc_meta: *mut c_void = core::ptr::null_mut();
pub static mut object: *mut c_void = core::ptr::null_mut();
    if (is_kfence_address(addr) || !slab) {
    return;
    }
    cache = slab.slab_cache;
    object = nearest_obj(cache, slab, addr);
    alloc_meta = kasan_get_alloc_meta(cache, object);
    if (!alloc_meta) {
    return;
    }
    alloc_meta.aux_stack[1] = alloc_meta.aux_stack[0];
    alloc_meta.aux_stack[0] = kasan_save_stack(0, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_save_alloc_info(cache: *mut kmem_cache, object: *mut c_void, flags: gfp_t) {
pub static mut alloc_meta: *mut c_void = core::ptr::null_mut();
    alloc_meta = kasan_get_alloc_meta(cache, object);
    if (!alloc_meta) {
    return;
    }
// Invalidate previous stack traces (might exist for krealloc or mempool).
    release_alloc_meta(alloc_meta);
    kasan_save_track(&alloc_meta.alloc_track, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_save_free_info(cache: *mut kmem_cache, object: *mut c_void) {
pub static mut free_meta: *mut c_void = core::ptr::null_mut();
    free_meta = kasan_get_free_meta(cache, object);
    if (!free_meta) {
    return;
    }
// Invalidate previous stack trace (might exist for mempool).
    release_free_meta(object, free_meta);
    kasan_save_track(&free_meta.free_track, 0);
// Mark free meta as valid.
// kasan_mem_to_shadow(object) = KASAN_SLAB_FREE_META;
    }
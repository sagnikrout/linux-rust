//! Automatically rewritten from C to Rust
//! Source: mm/kmsan/instrumentation.c
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
// KMSAN compiler API.
//
// This file implements __msan_XXX hooks that Clang inserts into the code
// compiled with -fsanitize=kernel-memory.
// See Documentation/dev-tools/kmsan.rst for more information on how KMSAN
// instrumentation works.
//
// Copyright (C) 2017-2022 Google LLC
// Author: Alexander Potapenko <glider@google.com>
//

#[no_mangle]
pub unsafe extern "C" fn is_bad_asm_addr(addr: *mut c_void, size: uintptr_t, is_store: bool) -> bool {
    if (IS_ENABLED!(CONFIG_ARCH_HAS_NON_OVERLAPPING_ADDRESS_SPACE) &&
    (u64)addr < TASK_SIZE) {
    return true;
    }
    if (!kmsan_get_metadata(addr, KMSAN_META_SHADOW)) {
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn get_shadow_origin_ptr(addr: *mut c_void, size: u64, store: bool) {
pub static mut ua_flags: c_ulong = 0;
pub static mut ret: usize = 0;
    ret = kmsan_get_shadow_origin_ptr(addr, size, store);
    user_access_restore(ua_flags);
    return ret;
    }
//
// KMSAN instrumentation functions follow. They are not declared elsewhere in
// the kernel code, so they are preceded by prototypes, to silence
// -Wmissing-prototypes warnings.
//
// Get shadow and origin pointers for a memory load with non-standard size.
// forward_decl: __msan_metadata_ptr_for_load_n;
#[no_mangle]
pub unsafe extern "C" fn __msan_metadata_ptr_for_load_n(addr: *mut c_void, size: uintptr_t) {
    return get_shadow_origin_ptr(addr, size, /*store*/ false);
    }
    EXPORT_SYMBOL(__msan_metadata_ptr_for_load_n);
// Get shadow and origin pointers for a memory store with non-standard size.
// forward_decl: __msan_metadata_ptr_for_store_n;
#[no_mangle]
pub unsafe extern "C" fn __msan_metadata_ptr_for_store_n(addr: *mut c_void, size: uintptr_t) {
    return get_shadow_origin_ptr(addr, size, /*store*/ true);
    }
    EXPORT_SYMBOL(__msan_metadata_ptr_for_store_n);
//
// Declare functions that obtain shadow/origin pointers for loads and stores
// with fixed size.
//

    struct shadow_origin_ptr __msan_metadata_ptr_for_load_##size(      
    void *addr);                                               
    struct shadow_origin_ptr __msan_metadata_ptr_for_load_##size(      
    void *addr)                                                
    {                                                                  
    return get_shadow_origin_ptr(addr, size, /*store*/ false); 
    }                                                                  
    EXPORT_SYMBOL(__msan_metadata_ptr_for_load_##size);                
    struct shadow_origin_ptr __msan_metadata_ptr_for_store_##size(     
    void *addr);                                               
    struct shadow_origin_ptr __msan_metadata_ptr_for_store_##size(     
    void *addr)                                                
    {                                                                  
    return get_shadow_origin_ptr(addr, size, /*store*/ true);  
    }                                                                  
    EXPORT_SYMBOL(__msan_metadata_ptr_for_store_##size)
pub static mut 1: usize = 0;
pub static mut 2: usize = 0;
pub static mut 4: usize = 0;
pub static mut 8: usize = 0;
//
// Handle a memory store performed by inline assembly. KMSAN conservatively
// attempts to unpoison the outputs of asm() directives to prevent false
// positives caused by missed stores.
//
// __msan_instrument_asm_store() may be called for inline assembly code when
// entering or leaving IRQ. We omit the check for kmsan_in_runtime() to ensure
// the memory written to in these cases is also marked as initialized.
//
// forward_decl: __msan_instrument_asm_store;
#[no_mangle]
pub unsafe extern "C" fn __msan_instrument_asm_store(addr: *mut c_void, size: uintptr_t) {
    let mut ua_flags = 0;
    if (!kmsan_enabled) {
    return;
    }
    ua_flags = user_access_save();
//
// Most of the accesses are below 32 bytes. The exceptions so far are
// clwb() (64 bytes), FPU state (512 bytes) and chsc() (4096 bytes).
//
    if (size > 4096) {
    WARN_ONCE(1, "assembly store size too big: %ld\n", size);
    size = 8;
    }
    if (is_bad_asm_addr(addr, size, /*is_store*/ true)) {
    user_access_restore(ua_flags);
    return;
    }
// Unpoisoning the memory on best effort.
    kmsan_internal_unpoison_memory(addr, size, /*checked*/ false);
    user_access_restore(ua_flags);
    }
    EXPORT_SYMBOL(__msan_instrument_asm_store);
//
// KMSAN instrumentation pass replaces LLVM memcpy, memmove and memset
// intrinsics with calls to respective __msan_ functions. We use
// get_param0_metadata() and set_retval_metadata() to store the shadow/origin
// values for the destination argument of these functions and use them for the
// functions' return values.
//
#[no_mangle]
pub unsafe extern "C" fn get_param0_metadata(shadow: *mut u64, origin: *mut depot_stack_handle_t) {
    let mut ctx = kmsan_get_context();
// shadow = *(ctx->cstate.param_tls);
// origin = ctx->cstate.param_origin_tls[0];
    }
#[no_mangle]
pub unsafe extern "C" fn set_retval_metadata(shadow: u64, origin: depot_stack_handle_t) {
    let mut ctx = kmsan_get_context();
// (ctx->cstate.retval_tls) = shadow;
    ctx.cstate.retval_origin_tls = origin;
    }
// Handle llvm.memmove intrinsic.
// forward_decl: __msan_memmove;
#[no_mangle]
pub unsafe extern "C" fn __msan_memmove(dst: *mut c_void, src: *mut c_void, n: uintptr_t) -> *mut c_void {
    let mut origin;
pub static mut result: *mut c_void = core::ptr::null_mut();
    let mut shadow = 0;
    get_param0_metadata(&shadow, &origin);
    result = __memmove(dst, src, n);
    if (!n) {
// Some people call memmove() with zero length.
    return result;
    }
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return result;
    }
    kmsan_enter_runtime();
    kmsan_internal_memmove_metadata(dst, src, n);
    kmsan_leave_runtime();
    set_retval_metadata(shadow, origin);
    return result;
    }
    EXPORT_SYMBOL(__msan_memmove);
// Handle llvm.memcpy intrinsic.
// forward_decl: __msan_memcpy;
#[no_mangle]
pub unsafe extern "C" fn __msan_memcpy(dst: *mut c_void, src: *mut c_void, n: uintptr_t) -> *mut c_void {
    let mut origin;
pub static mut result: *mut c_void = core::ptr::null_mut();
    let mut shadow = 0;
    get_param0_metadata(&shadow, &origin);
    result = __memcpy(dst, src, n);
    if (!n) {
// Some people call memcpy() with zero length.
    return result;
    }
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return result;
    }
    kmsan_enter_runtime();
// Using memmove instead of memcpy doesn't affect correctness.
    kmsan_internal_memmove_metadata(dst, src, n);
    kmsan_leave_runtime();
    set_retval_metadata(shadow, origin);
    return result;
    }
    EXPORT_SYMBOL(__msan_memcpy);
// Handle llvm.memset intrinsic.
// forward_decl: __msan_memset;
#[no_mangle]
pub unsafe extern "C" fn __msan_memset(dst: *mut c_void, c: c_int, n: uintptr_t) -> *mut c_void {
    let mut origin;
pub static mut result: *mut c_void = core::ptr::null_mut();
    let mut shadow = 0;
    get_param0_metadata(&shadow, &origin);
    result = __memset(dst, c, n);
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return result;
    }
    kmsan_enter_runtime();
//
// Clang doesn't pass parameter metadata here, so it is impossible to
// use shadow of @c to set up the shadow for @dst.
//
    kmsan_internal_unpoison_memory(dst, n, /*checked*/ false);
    kmsan_leave_runtime();
    set_retval_metadata(shadow, origin);
    return result;
    }
    EXPORT_SYMBOL(__msan_memset);
//
// Create a new origin from an old one. This is done when storing an
// uninitialized value to memory. When reporting an error, KMSAN unrolls and
// prints the whole chain of stores that preceded the use of this value.
//
    depot_stack_handle_t __msan_chain_origin(depot_stack_handle_t origin);
#[no_mangle]
pub unsafe extern "C" fn __msan_chain_origin(origin: depot_stack_handle_t) -> depot_stack_handle_t {
pub static mut ret: depot_stack_handle_t = 0;
    let mut ua_flags = 0;
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return ret;
    }
    ua_flags = user_access_save();
// Creating new origins may allocate memory.
    kmsan_enter_runtime();
    ret = kmsan_internal_chain_origin(origin);
    kmsan_leave_runtime();
    user_access_restore(ua_flags);
    return ret;
    }
    EXPORT_SYMBOL(__msan_chain_origin);
// Poison a local variable when entering a function.
// forward_decl: __msan_poison_alloca;
#[no_mangle]
pub unsafe extern "C" fn __msan_poison_alloca(address: *mut c_void, size: uintptr_t, descr: *mut c_char) {
    let mut handle;
    unsigned long entries[4];
    let mut ua_flags = 0;
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return;
    }
    ua_flags = user_access_save();
    entries[0] = KMSAN_ALLOCA_MAGIC_ORIGIN;
    entries[1] = (u64)descr;
    entries[2] = (u64)__builtin_return_address(0);
//
// With frame pointers enabled, it is possible to quickly fetch the
// second frame of the caller stack without calling the unwinder.
// Without them, simply do not bother.
//
    if (IS_ENABLED!(CONFIG_UNWINDER_FRAME_POINTER)) {
    entries[3] = (u64)__builtin_return_address(1);
    }
    else {
    entries[3] = 0;
    }
// stack_depot_save() may allocate memory.
    kmsan_enter_runtime();
    handle = stack_depot_save(entries, ARRAY_SIZE!(entries), __GFP_HIGH);
    kmsan_leave_runtime();
    kmsan_internal_set_shadow_origin(address, size, -1, handle,
// checked*/ true);
    user_access_restore(ua_flags);
    }
    EXPORT_SYMBOL(__msan_poison_alloca);
// Unpoison a local variable.
// forward_decl: __msan_unpoison_alloca;
#[no_mangle]
pub unsafe extern "C" fn __msan_unpoison_alloca(address: *mut c_void, size: uintptr_t) {
    if (!kmsan_enabled || kmsan_in_runtime()) {
    return;
    }
    kmsan_enter_runtime();
    kmsan_internal_unpoison_memory(address, size, /*checked*/ true);
    kmsan_leave_runtime();
    }
    EXPORT_SYMBOL(__msan_unpoison_alloca);
//
// Report that an uninitialized value with the given origin was used in a way
// that constituted undefined behavior.
//
// forward_decl: __msan_warning;
#[no_mangle]
pub unsafe extern "C" fn __msan_warning(origin: u32) {
    kmsan_report(origin, /*address*/ core::ptr::null_mut(), /*size*/ 0,
// off_first*/ 0, /*off_last*/ 0, /*user_addr*/ NULL,
    REASON_ANY);
    }
    EXPORT_SYMBOL(__msan_warning);
//
// At the beginning of an instrumented function, obtain the pointer to
// `struct kmsan_context_state` holding the metadata for function parameters.
//
// forward_decl: __msan_get_context_state;
#[no_mangle]
pub unsafe extern "C" fn __msan_get_context_state() -> *mut c_void {
    return &kmsan_get_context().cstate;
    }
    EXPORT_SYMBOL(__msan_get_context_state);
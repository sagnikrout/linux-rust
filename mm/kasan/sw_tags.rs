//! Automatically rewritten from C to Rust
//! Source: mm/kasan/sw_tags.c
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
// This file contains core software tag-based KASAN code.
//
// Copyright (c) 2018 Google, Inc.
// Author: Andrey Konovalov <andreyknvl@google.com>
//

pub static mut u32: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn kasan_init_sw_tags()  {
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
    per_cpu(prng_state, cpu) = (u32)get_cycles();
    }
    kasan_init_tags();
    kasan_enable();
    pr_info!("KernelAddressSanitizer initialized (sw-tags, stacktrace=%s)\n",
    str_on_off(kasan_stack_collection_enabled()));
    }
//
// If a preemption happens between this_cpu_read and this_cpu_write, the only
// side effect is that we'll give a few allocated in different contexts objects
// the same tag. Since tag-based KASAN is meant to be used a probabilistic
// bug-detection debug feature, this doesn't have significant negative impact.
//
// Ideally the tags use strong randomness to prevent any attempts to predict
// them during explicit exploit attempts. But strong randomness is expensive,
// and we did an intentional trade-off to use a PRNG. This non-atomic RMW
// sequence has in fact positive effect, since interrupts that randomly skew
// PRNG at unpredictable points do only good.
//
#[no_mangle]
pub unsafe extern "C" fn kasan_random_tag() -> u8 {
pub static mut state: u32 = 0;
    state = 1664525 * state + 1013904223;
    this_cpu_write(prng_state, state);
    return (u8)(state % (KASAN_TAG_MAX + 1));
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_check_range(addr: *mut c_void, size: size_t, write: bool, ret_ip: c_ulong) -> bool {
    let mut tag = 0;
    let mut shadow_first = core::ptr::null_mut();
    let mut shadow_last = core::ptr::null_mut();
    let mut shadow = core::ptr::null_mut();
pub static mut untagged_addr: *mut c_void = core::ptr::null_mut();
    if (unlikely(size == 0)) {
    return true;
    }
    if (unlikely(addr + size < addr)) {
    return !kasan_report(addr, size, write, ret_ip);
    }
    tag = get_tag(addr);
//
// Ignore accesses for pointers tagged with 0xff (native kernel
// pointer tag) to suppress false positives caused by kmap.
//
// Some kernel code was written to account for archs that don't keep
// high memory mapped all the time, but rather map and unmap particular
// pages when needed. Instead of storing a pointer to the kernel memory,
// this code saves the address of the page structure and offset within
// that page for later use. Those pages are then mapped and unmapped
// with kmap/kunmap when necessary and virt_to_page is used to get the
// virtual address of the page. For arm64 (that keeps the high memory
// mapped all the time), kmap is turned into a page_address call.
// The issue is that with use of the page_address + virt_to_page
// sequence the top byte value of the original pointer gets lost (gets
// set to KASAN_TAG_KERNEL (0xFF)).
//
    if (tag == KASAN_TAG_KERNEL) {
    return true;
    }
    untagged_addr = kasan_reset_tag(addr);
    if (unlikely(!addr_has_metadata(untagged_addr))) {
    return !kasan_report(addr, size, write, ret_ip);
    }
    shadow_first = kasan_mem_to_shadow(untagged_addr);
    shadow_last = kasan_mem_to_shadow(untagged_addr + size - 1);
    while (shadow <= shadow_last) {
    if (*shadow != tag) {
    return !kasan_report(addr, size, write, ret_ip);
    }
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_byte_accessible(addr: *const c_void) -> bool {
pub static mut tag: u8 = 0;
    let mut untagged_addr = kasan_reset_tag(addr);
    let mut shadow_byte = 0;
    if (!addr_has_metadata(untagged_addr)) {
    return false;
    }
    shadow_byte = READ_ONCE(*kasan_mem_to_shadow(untagged_addr));
pub static mut tag: return = 0;
    }

    void __hwasan_load##size##_noabort(void *addr)			
    {								
    kasan_check_range(addr, size, false, _RET_IP_);		
    }								
    EXPORT_SYMBOL(__hwasan_load##size##_noabort);			
    void __hwasan_store##size##_noabort(void *addr)			
    {								
    kasan_check_range(addr, size, true, _RET_IP_);		
    }								
    EXPORT_SYMBOL(__hwasan_store##size##_noabort)
pub static mut 1: usize = 0;
pub static mut 2: usize = 0;
pub static mut 4: usize = 0;
pub static mut 8: usize = 0;
pub static mut 16: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn __hwasan_loadN_noabort(addr: *mut c_void, size: isize) {
    kasan_check_range(addr, size, false, _RET_IP_);
    }
    EXPORT_SYMBOL(__hwasan_loadN_noabort);
#[no_mangle]
pub unsafe extern "C" fn __hwasan_storeN_noabort(addr: *mut c_void, size: isize) {
    kasan_check_range(addr, size, true, _RET_IP_);
    }
    EXPORT_SYMBOL(__hwasan_storeN_noabort);
#[no_mangle]
pub unsafe extern "C" fn __hwasan_tag_memory(addr: *mut c_void, tag: u8, size: isize) {
    kasan_poison(addr, size, tag, false);
    }
    EXPORT_SYMBOL(__hwasan_tag_memory);
#[no_mangle]
pub unsafe extern "C" fn kasan_tag_mismatch(addr: *mut c_void, access_info: c_ulong, ret_ip: c_ulong) {
    kasan_report(addr, 1 << (access_info & 0xf), access_info & 0x10,
    ret_ip);
    }
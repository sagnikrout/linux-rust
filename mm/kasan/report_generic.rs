//! Automatically rewritten from C to Rust
//! Source: mm/kasan/report_generic.c
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
// This file contains generic KASAN specific error reporting code.
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// Author: Andrey Ryabinin <ryabinin.a.a@gmail.com>
//
// Some code borrowed from https://github.com/xairy/kasan-prototype by
// Andrey Konovalov <andreyknvl@gmail.com>
//

    const void *kasan_find_first_bad_addr(const void *addr, size_t size)
    {
    let mut p = addr;
    if (!addr_has_metadata(p)) {
    return p;
    }
    while (p < addr + size && !(*kasan_mem_to_shadow(p))) {
    p += KASAN_GRANULE_SIZE;
    }
    return p;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_get_alloc_size(object: *mut c_void, cache: *mut kmem_cache) -> usize {
pub static mut size: usize = 0;
pub static mut shadow: *mut c_void = core::ptr::null_mut();
//
// Skip the addr_has_metadata check, as this function only operates on
// slab memory, which must have metadata.
//
// The loop below returns 0 for freed objects, for which KASAN cannot
// calculate the allocation size based on the metadata.
//
    shadow = kasan_mem_to_shadow(object);
    while (size < cache.object_size) {
    if (*shadow == 0) {
    size += KASAN_GRANULE_SIZE;
    }

    else if (*shadow >= 1 && *shadow <= KASAN_GRANULE_SIZE - 1) {
    return size + *shadow;
    }
    else {
    return size;
    }
    shadow += 1;
    }
    return cache.object_size;
    }
    static const char *get_shadow_bug_type(kasan_report_info *info)
    {
    let mut bug_type = "unknown-crash";
pub static mut shadow_addr: *mut c_void = core::ptr::null_mut();
    shadow_addr = kasan_mem_to_shadow(info.first_bad_addr);
//
// If shadow byte value is in [0, KASAN_GRANULE_SIZE) we can look
// at the next shadow byte to determine the type of the bad access.
//
    if (*shadow_addr > 0 && *shadow_addr <= KASAN_GRANULE_SIZE - 1) {
    shadow_addr += 1;
    }
    match (*shadow_addr) {
    0 ... KASAN_GRANULE_SIZE - 1 => {
//
// In theory it's still possible to see these shadow values
// due to a data race in the kernel code.
//
    bug_type = "out-of-bounds";
    // break;
    }
    KASAN_PAGE_REDZONE => {
    }
    KASAN_SLAB_REDZONE => {
    bug_type = "slab-out-of-bounds";
    // break;
    }
    KASAN_GLOBAL_REDZONE => {
    bug_type = "global-out-of-bounds";
    // break;
    }
    KASAN_STACK_LEFT => {
    }
    KASAN_STACK_MID => {
    }
    KASAN_STACK_RIGHT => {
    }
    KASAN_STACK_PARTIAL => {
    bug_type = "stack-out-of-bounds";
    // break;
    }
    KASAN_PAGE_FREE => {
    bug_type = "use-after-free";
    // break;
    }
    KASAN_SLAB_FREE => {
    }
    KASAN_SLAB_FREE_META => {
    bug_type = "slab-use-after-free";
    // break;
    }
    KASAN_ALLOCA_LEFT => {
    }
    KASAN_ALLOCA_RIGHT => {
    bug_type = "alloca-out-of-bounds";
    // break;
    }
    KASAN_VMALLOC_INVALID => {
    bug_type = "vmalloc-out-of-bounds";
    // break;
    }
    }
    return bug_type;
    }
    static const char *get_wild_bug_type(kasan_report_info *info)
    {
    let mut bug_type = "unknown-crash";
    if ((unsigned long)info.access_addr < PAGE_SIZE) {
    bug_type = "null-ptr-deref";
    }

    else if ((unsigned long)info.access_addr < TASK_SIZE) {
    bug_type = "user-memory-access";
    }
    else {
    bug_type = "wild-memory-access";
    }
    return bug_type;
    }
    static const char *get_bug_type(kasan_report_info *info)
    {
//
// If access_size is a negative number, then it has reason to be
// defined as out-of-bounds bug type.
//
// Casting negative numbers to size_t would indeed turn up as
// a large size_t and its value will be larger than ULONG_MAX/2,
// so that this can qualify as out-of-bounds.
//
    if (info.access_addr + info.access_size < info.access_addr) {
    return "out-of-bounds";
    }
    if (addr_has_metadata(info.access_addr)) {
    return get_shadow_bug_type(info);
    }
    return get_wild_bug_type(info);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_complete_mode_report_info(info: *mut kasan_report_info) {
pub static mut alloc_meta: *mut c_void = core::ptr::null_mut();
pub static mut free_meta: *mut c_void = core::ptr::null_mut();
    if (!info.bug_type) {
    info.bug_type = get_bug_type(info);
    }
    if (!info.cache || !info.object) {
    return;
    }
    alloc_meta = kasan_get_alloc_meta(info.cache, info.object);
    if (alloc_meta) {
    memcpy(&info.alloc_track, &alloc_meta.alloc_track,
    sizeof!(info.alloc_track));
    }
    if (*kasan_mem_to_shadow(info.object) == KASAN_SLAB_FREE_META) {
// Free meta must be present with KASAN_SLAB_FREE_META.
    free_meta = kasan_get_free_meta(info.cache, info.object);
    memcpy(&info.free_track, &free_meta.free_track,
    sizeof!(info.free_track));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_metadata_fetch_row(buffer: *mut c_char, row: *mut c_void) {
    memcpy(buffer, kasan_mem_to_shadow(row), META_BYTES_PER_ROW);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_print_aux_stacks(cache: *mut kmem_cache, object: *const c_void) {
pub static mut alloc_meta: *mut c_void = core::ptr::null_mut();
    alloc_meta = kasan_get_alloc_meta(cache, object);
    if (!alloc_meta) {
    return;
    }
    if (alloc_meta.aux_stack[0]) {
    pr_err!("Last potentially related work creation:\n");
    stack_depot_print(alloc_meta.aux_stack[0]);
    pr_err!("\n");
    }
    if (alloc_meta.aux_stack[1]) {
    pr_err!("Second to last potentially related work creation:\n");
    stack_depot_print(alloc_meta.aux_stack[1]);
    pr_err!("\n");
    }
    }

    static bool __must_check tokenize_frame_descr(const char **frame_descr,
    char *token, size_t max_tok_len,
    unsigned long *value)
    {
    let mut sep = strchr(*frame_descr, ' ');
    if (sep == core::ptr::null_mut()) {
    sep = *frame_descr + strlen(*frame_descr);
    }
    if (token != core::ptr::null_mut()) {
pub static mut tok_len: usize = 0;
    if (tok_len + 1 > max_tok_len) {
    pr_err!("internal error: frame description too long: %s\n",
// frame_descr);
    return false;
    }
// Copy token (+ 1 byte for '\0').
    strscpy(token, *frame_descr, tok_len + 1);
    }
// Advance frame_descr past separator.
// frame_descr = sep + 1;
    if (value != core::ptr::null_mut() && kstrtoul(token, 10, value)) {
    pr_err!("internal error: not a valid number: %s\n", token);
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn print_decoded_frame_descr(frame_descr: *const c_char) {
//
// We need to parse the following string:
// "n alloc_1 alloc_2 ... alloc_n"
// where alloc_i looks like
// "offset size len name"
// or "offset size len name:line".
//
    char token[64];
    let mut num_objects = 0;
    if (!tokenize_frame_descr(&frame_descr, token, sizeof!(token),
    &num_objects)) {
    return;
    }
    pr_err!("\n");
    pr_err!("This frame has %lu %s:\n", num_objects,
    num_objects == 1 ? "object" : "objects");
    while (num_objects--) {
    let mut offset = 0;
    let mut size = 0;
// access offset
    if (!tokenize_frame_descr(&frame_descr, token, sizeof!(token),
    &offset)) {
    return;
    }
// access size
    if (!tokenize_frame_descr(&frame_descr, token, sizeof!(token),
    &size)) {
    return;
    }
// name length (unused)
    if (!tokenize_frame_descr(&frame_descr, core::ptr::null_mut(), 0, core::ptr::null_mut())) {
    return;
    }
// object name
    if (!tokenize_frame_descr(&frame_descr, token, sizeof!(token),
    core::ptr::null_mut())) {
    return;
    }
// Strip line number; without filename it's not very helpful.
    strreplace(token, ':', '\0');
// Finally, print object information.
    pr_err!(" [%lu, %lu) '%s'", offset, offset + size, token);
    }
    }
// Returns true only if the address is on the current task's stack.
    static bool __must_check get_address_stack_frame_info(const void *addr,
    unsigned long *offset,
    const char **frame_descr,
    const void **frame_pc)
    {
    let mut aligned_addr = 0;
    let mut mem_ptr = 0;
pub static mut shadow_bottom: *mut c_void = core::ptr::null_mut();
pub static mut shadow_ptr: *mut c_void = core::ptr::null_mut();
pub static mut frame: *mut c_void = core::ptr::null_mut();
    BUILD_BUG_ON!(IS_ENABLED!(CONFIG_STACK_GROWSUP));
    aligned_addr = round_down((unsigned long)addr, sizeof!(long));
    mem_ptr = round_down(aligned_addr, KASAN_GRANULE_SIZE);
    shadow_ptr = kasan_mem_to_shadow(aligned_addr);
    shadow_bottom = kasan_mem_to_shadow(end_of_stack(current));
    while (shadow_ptr >= shadow_bottom && *shadow_ptr != KASAN_STACK_LEFT) {
    shadow_ptr -= 1;
    mem_ptr -= KASAN_GRANULE_SIZE;
    }
    while (shadow_ptr >= shadow_bottom && *shadow_ptr == KASAN_STACK_LEFT) {
    shadow_ptr -= 1;
    mem_ptr -= KASAN_GRANULE_SIZE;
    }
    if (shadow_ptr < shadow_bottom) {
    return false;
    }
    frame = (mem_ptr + KASAN_GRANULE_SIZE);
    if (frame[0] != KASAN_CURRENT_STACK_FRAME_MAGIC) {
    pr_err!("internal error: frame has invalid marker: %lu\n",
    frame[0]);
    return false;
    }
// offset = (unsigned long)addr - (unsigned long)frame;
// frame_descr = frame[1];
// frame_pc = frame[2];
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_print_address_stack_frame(addr: *const c_void) {
    let mut offset = 0;
pub static mut frame_descr: *mut c_void = core::ptr::null_mut();
pub static mut frame_pc: *mut c_void = core::ptr::null_mut();
    if (WARN_ON!(!object_is_on_stack(addr))) {
    return;
    }
    pr_err!("The buggy address belongs to stack of task %s/%d\n",
    current.comm, task_pid_nr(current));
    if (!get_address_stack_frame_info(addr, &offset, &frame_descr,
    &frame_pc)) {
    return;
    }
    pr_err!(" and is located at offset %lu in frame:\n", offset);
    pr_err!(" %pS\n", frame_pc);
    if (!frame_descr) {
    return;
    }
    print_decoded_frame_descr(frame_descr);
    }

    void __asan_report_load##size##_noabort(void *addr) 
    {                                                         
    kasan_report(addr, size, false, _RET_IP_);	  
    }                                                         
    EXPORT_SYMBOL(__asan_report_load##size##_noabort)

    void __asan_report_store##size##_noabort(void *addr) 
    {                                                          
    kasan_report(addr, size, true, _RET_IP_);	   
    }                                                          
    EXPORT_SYMBOL(__asan_report_store##size##_noabort)
pub static mut 1: usize = 0;
pub static mut 2: usize = 0;
pub static mut 4: usize = 0;
pub static mut 8: usize = 0;
pub static mut 16: usize = 0;
pub static mut 1: usize = 0;
pub static mut 2: usize = 0;
pub static mut 4: usize = 0;
pub static mut 8: usize = 0;
pub static mut 16: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn __asan_report_load_n_noabort(addr: *mut c_void, size: isize) {
    kasan_report(addr, size, false, _RET_IP_);
    }
    EXPORT_SYMBOL(__asan_report_load_n_noabort);
#[no_mangle]
pub unsafe extern "C" fn __asan_report_store_n_noabort(addr: *mut c_void, size: isize) {
    kasan_report(addr, size, true, _RET_IP_);
    }
    EXPORT_SYMBOL(__asan_report_store_n_noabort);
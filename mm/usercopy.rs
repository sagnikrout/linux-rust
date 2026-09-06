//! Automatically rewritten from C to Rust
//! Source: mm/usercopy.c
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
// This implements the various checks for CONFIG_HARDENED_USERCOPY*,
// which are designed to protect kernel memory from needless exposure
// and overwrite under many unintended conditions. This code is based
// on PAX_USERCOPY, which is:
//
// Copyright (C) 2001-2016 PaX Team, Bradley Spengler, Open Source
// Security Inc.
//

//
// Checks if a given pointer and length is contained by the current
// stack frame (if possible).
//
// Returns:
// NOT_STACK: not at all on the stack
// GOOD_FRAME: fully within a valid stack frame
// GOOD_STACK: within the current stack (when can't frame-check exactly)
// BAD_STACK: error condition (invalid stack position or bad stack frame)
//
#[no_mangle]
unsafe extern "C" fn check_stack_object(obj: *const c_void, len: c_ulong) -> noinline int {
pub static mut stack: *const c_void  const = core::ptr::null_mut();
pub static mut stackend: *const c_void  const = core::ptr::null_mut();
    let mut ret = 0;
// Object is not on the stack at all.
    if (obj + len <= stack || stackend <= obj) {
    return NOT_STACK;
    }
//
// Reject: object partially overlaps the stack (passing the
// check above means at least one end is within the stack,
// so if this check fails, the other end is outside the stack).
//
    if (obj < stack || stackend < obj + len) {
    return BAD_STACK;
    }
// Check if object is safely within a valid frame.
    ret = arch_within_stack_frames(stack, stackend, obj, len);
    if (ret) {
    return ret;
    }
// Finally, check stack depth if possible.

    if (IS_ENABLED!(CONFIG_STACK_GROWSUP)) {
    if (current_stack_pointer < obj + len) {
    return BAD_STACK;
    }
    } else {
    if (obj < current_stack_pointer) {
    return BAD_STACK;
    }
    }

    return GOOD_STACK;
    }
//
// If these functions are reached, then CONFIG_HARDENED_USERCOPY has found
// an unexpected state during a copy_from_user() or copy_to_user() call.
// There are several checks being performed on the buffer by the
// __check_object_size() function. Normal stack buffer usage should never
// trip the checks, and kernel text addressing will always trip the check.
// For cache objects, it is checking that only the whitelisted range of
// bytes for a given cache is being accessed (via the cache's usersize and
// useroffset fields). To adjust a cache whitelist, use the usercopy-aware
// kmem_cache_create_usercopy() function to create the cache (and
// carefully audit the whitelist range).
//
    void __noreturn usercopy_abort(const char *name, const char *detail,
    bool to_user, unsigned long offset,
    unsigned long len)
    {
    pr_emerg("Kernel memory %s attempt detected %s %s%s%s%s (offset %lu, size %lu)!\n",
    to_user ? "exposure" : "overwrite",
    to_user ? "from" : "to",
    name ? : "unknown?!",
    detail ? " '" : "", detail ? : "", detail ? "'" : "",
    offset, len);
//
// For greater effect, it would be nice to do do_group_exit(),
// but BUG() actually hooks all the lock-breaking and per-arch
// Oops code, so that is used here instead.
//
    BUG();
    }
// Returns true if any portion of [ptr,ptr+n) over laps with [low,high).
#[no_mangle]
pub unsafe extern "C" fn overlaps(ptr: c_ulong, n: c_ulong, low: c_ulong, high: c_ulong) -> bool {
pub static mut check_low: c_ulong = 0;
pub static mut check_high: c_ulong = 0;
// Does not overlap if entirely above or entirely below.
    if (check_low >= high || check_high <= low) {
    return false;
    }
    return true;
    }
// Is this address range in the kernel text area?
#[no_mangle]
pub unsafe extern "C" fn check_kernel_text_object(ptr: c_ulong, n: c_ulong, to_user: bool) {
pub static mut textlow: c_ulong = 0;
pub static mut texthigh: c_ulong = 0;
    unsigned long textlow_linear, texthigh_linear;
    if (overlaps(ptr, n, textlow, texthigh)) {
    usercopy_abort("kernel text", core::ptr::null_mut(), to_user, ptr - textlow, n);
    }
//
// Some architectures have virtual memory mappings with a secondary
// mapping of the kernel text, i.e. there is more than one virtual
// kernel address that points to the kernel image. It is usually
// when there is a separate linear physical memory mapping, in that
// __pa() is not just the reverse of __va(). This can be detected
// and checked:
//
    textlow_linear = (unsigned long)lm_alias(textlow);
// No different mapping: we're done.
    if (textlow_linear == textlow) {
    return;
    }
// Check the secondary mapping...
    texthigh_linear = (unsigned long)lm_alias(texthigh);
    if (overlaps(ptr, n, textlow_linear, texthigh_linear)) {
    usercopy_abort("linear kernel text", core::ptr::null_mut(), to_user,
    ptr - textlow_linear, n);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn check_bogus_address(ptr: c_ulong, n: c_ulong, to_user: bool) {
// Reject if object wraps past end of memory.
    if (ptr + (n - 1) < ptr) {
    usercopy_abort("wrapped address", core::ptr::null_mut(), to_user, 0, ptr + n);
    }
// Reject if NULL or ZERO-allocation.
    if (ZERO_OR_NULL_PTR(ptr)) {
    usercopy_abort("null address", core::ptr::null_mut(), to_user, ptr, n);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn check_heap_object(ptr: *mut c_void, n: c_ulong, to_user: bool) {
pub static mut addr: c_ulong = 0;
    let mut offset = 0;
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut slab: *mut c_void = core::ptr::null_mut();
    if (is_kmap_addr(ptr)) {
    offset = offset_in_page(ptr);
    if (n > PAGE_SIZE - offset) {
    usercopy_abort("kmap", core::ptr::null_mut(), to_user, offset, n);
    }
    return;
    }
    if (is_vmalloc_addr(ptr) && !pagefault_disabled()) {
    let mut area = find_vmap_area(addr);
    if (!area) {
    usercopy_abort("vmalloc", "no area", to_user, 0, n);
    }
    if (n > area.va_end - addr) {
    offset = addr - area.va_start;
    usercopy_abort("vmalloc", core::ptr::null_mut(), to_user, offset, n);
    }
    return;
    }
    if (!virt_addr_valid(ptr)) {
    return;
    }
    page = virt_to_page(ptr);
    slab = page_slab(page);
    if (slab) {
// Check slab allocator for flags and size.
    __check_heap_object(ptr, n, slab, to_user);
    } else if (PageCompound(page)) {
    page = compound_head(page);
    offset = ptr - page_address(page);
    if (n > page_size(page) - offset) {
    usercopy_abort("page alloc", core::ptr::null_mut(), to_user, offset, n);
    }
    }
//
// We cannot check non-compound pages.  They might be part of
// a large allocation, in which case crossing a page boundary
// is fine.
//
    }
pub static mut CONFIG_HARDENED_USERCOPY_DEFAULT_ON: usize = 0;
    EXPORT_SYMBOL(validate_usercopy_range);
//
// Validates that the given object is:
// - not bogus address
// - fully contained by stack (or stack frame, when available)
// - fully within SLAB object (or object whitelist area, when available)
// - not in kernel text
//
#[no_mangle]
pub unsafe extern "C" fn __check_object_size(ptr: *const c_void, n: c_ulong, to_user: bool) {
// Skip all tests if size is zero.
    if (!n) {
    return;
    }
// Check for invalid addresses.
    check_bogus_address((const unsigned long)ptr, n, to_user);
// Check for bad stack object.
    switch (check_stack_object(ptr, n)) {
    case NOT_STACK:
// Object is not touching the current process stack.
    break;
    case GOOD_FRAME:
    case GOOD_STACK:
//
// Object is either in the correct frame (when it
// is possible to check) or just generally on the
// process stack (when frame checking not available).
//
    return;
// label;
    usercopy_abort("process stack", core::ptr::null_mut(), to_user,

    IS_ENABLED!(CONFIG_STACK_GROWSUP) ?
    ptr - current_stack_pointer :
    current_stack_pointer - ptr,

    0,

    n);
    }
// Check for bad heap object.
    check_heap_object(ptr, n, to_user);
// Check for object in kernel to avoid text exposure.
    check_kernel_text_object((const unsigned long)ptr, n, to_user);
    }
    EXPORT_SYMBOL(__check_object_size);
    static bool enable_checks __initdata =
    IS_ENABLED!(CONFIG_HARDENED_USERCOPY_DEFAULT_ON);
#[no_mangle]
unsafe extern "C" fn parse_hardened_usercopy(str: *mut c_char) -> c_int {
    if (kstrtobool(str, &enable_checks)) {
    pr_warn!("Invalid option string for hardened_usercopy: '%s'\n",
    str);
    }
    return 1;
    }
    __setup!("hardened_usercopy=", parse_hardened_usercopy);
#[no_mangle]
unsafe extern "C" fn set_hardened_usercopy() -> c_int {
    if (enable_checks) {
    static_branch_enable(&validate_usercopy_range);
    }
    else {
    static_branch_disable(&validate_usercopy_range);
    }
    return 1;
    }
    late_initcall!(set_hardened_usercopy);
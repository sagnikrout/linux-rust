//! Automatically rewritten from C to Rust
//! Source: mm/kasan/hw_tags.c
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
// This file contains core hardware tag-based KASAN code.
//
// Copyright (c) 2020 Google, Inc.
// Author: Andrey Konovalov <andreyknvl@google.com>
//

    enum kasan_arg {
    KASAN_ARG_DEFAULT,
    KASAN_ARG_OFF,
    KASAN_ARG_ON,
    };
    enum kasan_arg_mode {
    KASAN_ARG_MODE_DEFAULT,
    KASAN_ARG_MODE_SYNC,
    KASAN_ARG_MODE_ASYNC,
    KASAN_ARG_MODE_ASYMM,
    };
    enum kasan_arg_vmalloc {
    KASAN_ARG_VMALLOC_DEFAULT,
    KASAN_ARG_VMALLOC_OFF,
    KASAN_ARG_VMALLOC_ON,
    };
    static enum kasan_arg kasan_arg __ro_after_init;
    static enum kasan_arg_mode kasan_arg_mode __ro_after_init;
    static enum kasan_arg_vmalloc kasan_arg_vmalloc __initdata;
//
// Whether the selected mode is synchronous, asynchronous, or asymmetric.
// Defaults to KASAN_MODE_SYNC.
//
    enum kasan_mode kasan_mode __ro_after_init;
    EXPORT_SYMBOL_GPL(kasan_mode);
// Whether to enable vmalloc tagging.

pub static mut kasan_flag_vmalloc: usize = 0;

pub static mut kasan_flag_vmalloc: usize = 0;

    EXPORT_SYMBOL_GPL(kasan_flag_vmalloc);
// Whether to check write accesses only.
    static bool kasan_flag_write_only;
pub const PAGE_ALLOC_SAMPLE_DEFAULT: c_int = 1;
pub const PAGE_ALLOC_SAMPLE_ORDER_DEFAULT: c_int = 3;
//
// Sampling interval of page_alloc allocation (un)poisoning.
// Defaults to no sampling.
//
pub static mut kasan_page_alloc_sample: c_ulong = 0;
//
// Minimum order of page_alloc allocations to be affected by sampling.
// The default value is chosen to match both
// PAGE_ALLOC_COSTLY_ORDER and SKB_FRAG_PAGE_ORDER.
//
pub static mut kasan_page_alloc_sample_order: c_uint = 0;
pub static mut long: usize = 0;
// kasan=off/on
#[no_mangle]
unsafe extern "C" fn early_kasan_flag(arg: *mut c_char) -> c_int {
    if (!arg) {
    return -EINVAL;
    }
    if (!strcmp(arg, "off")) {
    kasan_arg = KASAN_ARG_OFF;
    }

    else if (!strcmp(arg, "on")) {
    kasan_arg = KASAN_ARG_ON;
    }
    else {
    return -EINVAL;
    }
    return 0;
    }
    early_param!("kasan", early_kasan_flag);
// kasan.mode=sync/async/asymm
#[no_mangle]
unsafe extern "C" fn early_kasan_mode(arg: *mut c_char) -> c_int {
    if (!arg) {
    return -EINVAL;
    }
    if (!strcmp(arg, "sync")) {
    kasan_arg_mode = KASAN_ARG_MODE_SYNC;
    }

    else if (!strcmp(arg, "async")) {
    kasan_arg_mode = KASAN_ARG_MODE_ASYNC;
    }

    else if (!strcmp(arg, "asymm")) {
    kasan_arg_mode = KASAN_ARG_MODE_ASYMM;
    }
    else {
    return -EINVAL;
    }
    return 0;
    }
    early_param!("kasan.mode", early_kasan_mode);
// kasan.vmalloc=off/on
#[no_mangle]
unsafe extern "C" fn early_kasan_flag_vmalloc(arg: *mut c_char) -> c_int {
    if (!arg) {
    return -EINVAL;
    }
    if (!IS_ENABLED!(CONFIG_KASAN_VMALLOC)) {
    return 0;
    }
    if (!strcmp(arg, "off")) {
    kasan_arg_vmalloc = KASAN_ARG_VMALLOC_OFF;
    }

    else if (!strcmp(arg, "on")) {
    kasan_arg_vmalloc = KASAN_ARG_VMALLOC_ON;
    }
    else {
    return -EINVAL;
    }
    return 0;
    }
    early_param!("kasan.vmalloc", early_kasan_flag_vmalloc);
// kasan.write_only=off/on
#[no_mangle]
unsafe extern "C" fn early_kasan_flag_write_only(arg: *mut c_char) -> c_int {
    if (!arg) {
    return -EINVAL;
    }
    if (!strcmp(arg, "off")) {
    kasan_flag_write_only = false;
    }

    else if (!strcmp(arg, "on")) {
    kasan_flag_write_only = true;
    }
    else {
    return -EINVAL;
    }
    return 0;
    }
    early_param!("kasan.write_only", early_kasan_flag_write_only);
    static inline const char *kasan_mode_info(void)
    {
    if (kasan_mode == KASAN_MODE_ASYNC) {
    return "async";
    }

    else if (kasan_mode == KASAN_MODE_ASYMM) {
    return "asymm";
    }
    else {
    return "sync";
    }
    }
// kasan.page_alloc.sample=<sampling interval>
#[no_mangle]
unsafe extern "C" fn early_kasan_flag_page_alloc_sample(arg: *mut c_char) -> c_int {
    let mut rv = 0;
    if (!arg) {
    return -EINVAL;
    }
    rv = kstrtoul(arg, 0, &kasan_page_alloc_sample);
    if (rv) {
    return rv;
    }
    if (!kasan_page_alloc_sample || kasan_page_alloc_sample > LONG_MAX) {
    kasan_page_alloc_sample = PAGE_ALLOC_SAMPLE_DEFAULT;
    return -EINVAL;
    }
    return 0;
    }
    early_param!("kasan.page_alloc.sample", early_kasan_flag_page_alloc_sample);
// kasan.page_alloc.sample.order=<minimum page order>
#[no_mangle]
unsafe extern "C" fn early_kasan_flag_page_alloc_sample_order(arg: *mut c_char) -> c_int {
    let mut rv = 0;
    if (!arg) {
    return -EINVAL;
    }
    rv = kstrtouint(arg, 0, &kasan_page_alloc_sample_order);
    if (rv) {
    return rv;
    }
    if (kasan_page_alloc_sample_order > INT_MAX) {
    kasan_page_alloc_sample_order = PAGE_ALLOC_SAMPLE_ORDER_DEFAULT;
    return -EINVAL;
    }
    return 0;
    }
    early_param!("kasan.page_alloc.sample.order", early_kasan_flag_page_alloc_sample_order);
//
// kasan_init_hw_tags_cpu() is called for each CPU.
// Not marked as __init as a CPU can be hot-plugged after boot.
//
#[no_mangle]
pub unsafe extern "C" fn kasan_init_hw_tags_cpu() {
//
// There's no need to check that the hardware is MTE-capable here,
// as this function is only called for MTE-capable hardware.
//
// If KASAN is disabled via command line, don't initialize it.
// When this function is called, kasan_flag_enabled is not yet
// set by kasan_init_hw_tags(). Thus, check kasan_arg instead.
//
    if (kasan_arg == KASAN_ARG_OFF) {
    return;
    }
//
// Enable async or asymm modes only when explicitly requested
// through the command line.
//
    kasan_enable_hw_tags();
    }
// kasan_init_hw_tags() is called once on boot CPU.
#[no_mangle]
pub unsafe extern "C" fn kasan_init_hw_tags()  {
// If hardware doesn't support MTE, don't initialize KASAN.
    if (!system_supports_mte()) {
    return;
    }
// If KASAN is disabled via command line, don't initialize it.
    if (kasan_arg == KASAN_ARG_OFF) {
    return;
    }
    match (kasan_arg_mode) {
    KASAN_ARG_MODE_DEFAULT => {
// Default is specified by kasan_mode definition.
    // break;
    }
    KASAN_ARG_MODE_SYNC => {
    kasan_mode = KASAN_MODE_SYNC;
    // break;
    }
    KASAN_ARG_MODE_ASYNC => {
    kasan_mode = KASAN_MODE_ASYNC;
    // break;
    }
    KASAN_ARG_MODE_ASYMM => {
    kasan_mode = KASAN_MODE_ASYMM;
    // break;
    }
    }
    match (kasan_arg_vmalloc) {
    KASAN_ARG_VMALLOC_DEFAULT => {
// Default is specified by kasan_flag_vmalloc definition.
    // break;
    }
    KASAN_ARG_VMALLOC_OFF => {
    static_branch_disable(&kasan_flag_vmalloc);
    // break;
    }
    KASAN_ARG_VMALLOC_ON => {
    static_branch_enable(&kasan_flag_vmalloc);
    // break;
    }
    }
    kasan_init_tags();
// KASAN is now initialized, enable it.
    kasan_enable();
    pr_info!("KernelAddressSanitizer initialized (hw-tags, mode=%s, vmalloc=%s, stacktrace=%s, write_only=%s)\n",
    kasan_mode_info(),
    str_on_off(kasan_vmalloc_enabled()),
    str_on_off(kasan_stack_collection_enabled()),
    str_on_off(kasan_flag_write_only));
    }

#[no_mangle]
unsafe extern "C" fn unpoison_vmalloc_pages(addr: *const c_void, tag: u8) {
pub static mut area: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
//
// As hardware tag-based KASAN only tags VM_ALLOC vmalloc allocations
// (see the comment in __kasan_unpoison_vmalloc), all of the pages
// should belong to a single area.
//
    area = find_vm_area(addr);
    if (WARN_ON!(!area)) {
    return;
    }
    while (i < area.nr_pages) {
    let mut page = area.pages[i];
    page_kasan_tag_set(page, tag);
    }
    }
#[no_mangle]
unsafe extern "C" fn init_vmalloc_pages(start: *const c_void, size: c_ulong) {
pub static mut addr: *mut c_void = core::ptr::null_mut();
    while (addr < start + size) {
    let mut page = vmalloc_to_page(addr);
    clear_highpage_kasan_tagged(page);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_unpoison_vmalloc(start: *mut c_void, size: c_ulong, flags: kasan_vmalloc_flags_t) -> *mut c_void {
    let mut tag = 0;
    unsigned long redzone_start, redzone_size;
    if (!kasan_vmalloc_enabled()) {
    if (flags & KASAN_VMALLOC_INIT) {
    init_vmalloc_pages(start, size);
    }
    return start;
    }
//
// Don't tag non-VM_ALLOC mappings, as:
//
// 1. Unlike the software KASAN modes, hardware tag-based KASAN only
// supports tagging physical memory. Therefore, it can only tag a
// single mapping of normal physical pages.
// 2. Hardware tag-based KASAN can only tag memory mapped with special
// mapping protection bits, see arch_vmap_pgprot_tagged().
// As non-VM_ALLOC mappings can be mapped outside of vmalloc code,
// providing these bits would require tracking all non-VM_ALLOC
// mappers.
//
// Thus, for VM_ALLOC mappings, hardware tag-based KASAN only tags
// the first virtual mapping, which is created by vmalloc().
// Tagging the page_alloc memory backing that vmalloc() allocation is
// skipped, see ___GFP_SKIP_KASAN.
//
// For non-VM_ALLOC allocations, page_alloc memory is tagged as usual.
//
    if (!(flags & KASAN_VMALLOC_VM_ALLOC)) {
    WARN_ON!(flags & KASAN_VMALLOC_INIT);
    return start;
    }
//
// Don't tag executable memory.
// The kernel doesn't tolerate having the PC register tagged.
//
    if (!(flags & KASAN_VMALLOC_PROT_NORMAL)) {
    WARN_ON!(flags & KASAN_VMALLOC_INIT);
    return start;
    }
    tag = (flags & KASAN_VMALLOC_KEEP_TAG) ? get_tag(start) : kasan_random_tag();
    start = set_tag(start, tag);
// Unpoison and initialize memory up to size.
    kasan_unpoison(start, size, flags & KASAN_VMALLOC_INIT);
//
// Explicitly poison and initialize the in-page vmalloc() redzone.
// Unlike software KASAN modes, hardware tag-based KASAN doesn't
// unpoison memory when populating shadow for vmalloc() space.
//
    redzone_start = round_up((unsigned long)start + size,
    KASAN_GRANULE_SIZE);
    redzone_size = round_up(redzone_start, PAGE_SIZE) - redzone_start;
    kasan_poison(redzone_start, redzone_size, KASAN_TAG_INVALID,
    flags & KASAN_VMALLOC_INIT);
//
// Set per-page tag flags to allow accessing physical memory for the
// vmalloc() mapping through page_address(vmalloc_to_page()).
//
    unpoison_vmalloc_pages(start, tag);
    return start;
    }
#[no_mangle]
pub unsafe extern "C" fn __kasan_poison_vmalloc(start: *const c_void, size: c_ulong) {
//
// No tagging here.
// The physical pages backing the vmalloc() allocation are poisoned
// through the usual page_alloc paths.
//
    }

#[no_mangle]
pub unsafe extern "C" fn kasan_enable_hw_tags() {
    if (kasan_arg_mode == KASAN_ARG_MODE_ASYNC) {
    hw_enable_tag_checks_async();
    }

    else if (kasan_arg_mode == KASAN_ARG_MODE_ASYMM) {
    hw_enable_tag_checks_asymm();
    }
    else {
    hw_enable_tag_checks_sync();
    }
//
// CPUs can only be in one of two states:
// - All CPUs support the write_only feature
// - No CPUs support the write_only feature
//
// If the first CPU attempts hw_enable_tag_checks_write_only() and
// finds the feature unsupported, kasan_flag_write_only is set to OFF
// to avoid further unnecessary calls on other CPUs.
//
    if (kasan_flag_write_only && hw_enable_tag_checks_write_only()) {
    kasan_flag_write_only = false;
    pr_err_once("write-only mode is not supported and thus not enabled\n");
    }
    }

    EXPORT_SYMBOL_IF_KUNIT(kasan_enable_hw_tags);
#[no_mangle]
pub unsafe extern "C" fn kasan_force_async_fault() -> VISIBLE_IF_KUNIT void {
    hw_force_async_tag_fault();
    }
    EXPORT_SYMBOL_IF_KUNIT(kasan_force_async_fault);
#[no_mangle]
pub unsafe extern "C" fn kasan_write_only_enabled() -> VISIBLE_IF_KUNIT bool {
    return kasan_flag_write_only;
    }
    EXPORT_SYMBOL_IF_KUNIT(kasan_write_only_enabled);
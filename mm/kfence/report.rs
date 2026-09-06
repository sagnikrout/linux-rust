//! Automatically rewritten from C to Rust
//! Source: mm/kfence/report.c
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
// KFENCE reporting.
//
// Copyright (C) 2020, Google LLC.
//

// May be overridden by <asm/kfence.h>.

pub static mut __ro_after_init: kfence_fault kfence_fault = 0;
#[no_mangle]
unsafe extern "C" fn early_kfence_fault(arg: *mut c_char) -> c_int {
    if (!arg) {
    return -EINVAL;
    }
    if (!strcmp(arg, "report")) {
    kfence_fault = KFENCE_FAULT_REPORT;
    }

    else if (!strcmp(arg, "oops")) {
    kfence_fault = KFENCE_FAULT_OOPS;
    }

    else if (!strcmp(arg, "panic")) {
    kfence_fault = KFENCE_FAULT_PANIC;
    }
    else {
    return -EINVAL;
    }
    return 0;
    }
    early_param!("kfence.fault", early_kfence_fault);
// Helper function to either print to a seq_file or to console.
    __printf(2, 3)
#[no_mangle]
unsafe extern "C" fn seq_con_printf(seq: *mut seq_file, fmt: *const c_char, ...) {
    let mut args;
    va_start(args, fmt);
    if (seq) {
    seq_vprintf(seq, fmt, args);
    }
    else {
    vprintk(fmt, args);
    }
    va_end(args);
    }
//
// Get the number of stack entries to skip to get out of MM internals. @type is
// optional, and if set to NULL, assumes an allocation or free stack.
//
#[no_mangle]
pub unsafe extern "C" fn get_stack_skipnr(num_entries: c_int, type: *mut kfence_error_type) -> c_int {
    char buf[64];
    int skipnr, fallback = 0;
    if (type) {
// Depending on error type, find different stack entries.
    match (*type) {
    KFENCE_ERROR_UAF => {
    }
    KFENCE_ERROR_OOB => {
    }
    KFENCE_ERROR_INVALID => {
//
// kfence_handle_page_fault() may be called with pt_regs
// set to NULL; in that case we'll simply show the full
// stack trace.
//
    return 0;
    }
    KFENCE_ERROR_CORRUPTION => {
    }
    KFENCE_ERROR_INVALID_FREE => {
    // break;
    }
    }
    }
    while (skipnr < num_entries) {
pub static mut len: c_int = 0;
    if (str_has_prefix(buf, ARCH_FUNC_PREFIX "kfence_") ||
    str_has_prefix(buf, ARCH_FUNC_PREFIX "__kfence_") ||
    str_has_prefix(buf, ARCH_FUNC_PREFIX "__kmem_cache_free") ||
    !strncmp(buf, ARCH_FUNC_PREFIX "__slab_free", len)) {
//
// In case of tail calls from any of the below to any of
// the above, optimized by the compiler such that the
// stack trace would omit the initial entry point below.
//
    fallback = skipnr + 1;
    }
//
// The below list should only include the initial entry points
// into the slab allocators. Includes the *_bulk() variants by
// checking prefixes.
//
    if (str_has_prefix(buf, ARCH_FUNC_PREFIX "kfree") ||
    str_has_prefix(buf, ARCH_FUNC_PREFIX "kmem_cache_free") ||
    str_has_prefix(buf, ARCH_FUNC_PREFIX "__kmalloc") ||
    str_has_prefix(buf, ARCH_FUNC_PREFIX "kmem_cache_alloc")) {
// goto;
    }
    }
    if (fallback < num_entries) {
    return fallback;
    }
// label;
    skipnr += 1;
    return skipnr < num_entries ? skipnr : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kfence_print_stack(seq: *mut seq_file, meta: *mut kfence_metadata, lock: bool show_alloc)
    __must_hold(&meta.) {
    let mut track = show_alloc ? &meta.alloc_track : &meta.free_track;
pub static mut ts_sec: u64 = 0;
pub static mut rem_nsec: c_ulong = 0;
pub static mut interval_nsec: u64 = 0;
pub static mut rem_interval_nsec: c_ulong = 0;
// Timestamp matches printk timestamp format.
    seq_con_printf(seq, "%s by task %d on cpu %d at %lu.%06lus (%lu.%06lus ago):\n",
    show_alloc ? "allocated" : meta.state == KFENCE_OBJECT_RCU_FREEING ?
    "rcu freeing" : "freed", track.pid,
    track.cpu, (unsigned long)ts_sec, rem_nsec / 1000,
    (unsigned long)interval_nsec, rem_interval_nsec / 1000);
    if (track.num_stack_entries) {
// Skip allocation/free internals stack.
pub static mut i: c_int = 0;
// stack_trace_seq_print() does not exist; open code our own.
    for (; i < track.num_stack_entries; i++) {
    seq_con_printf(seq, " %pS\n", track.stack_entries[i]);
    }
    } else {
    seq_con_printf(seq, " no %s stack\n", show_alloc ? "allocation" : "deallocation");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kfence_print_object(seq: *mut seq_file, meta: *const kfence_metadata) {
pub static mut size: c_int = 0;
pub static mut start: c_ulong = 0;
pub static mut cache: *const kmem_cache const = core::ptr::null_mut();
    lockdep_assert_held(&meta.lock);
    if (meta.state == KFENCE_OBJECT_UNUSED) {
    seq_con_printf(seq, "kfence-#%td unused\n", meta - kfence_metadata);
    return;
    }
    seq_con_printf(seq, "kfence-#%td: 0x%p-0x%p, size=%d, cache=%s\n\n",
    meta - kfence_metadata, start, (start + size - 1),
    size, (cache && cache.name) ? cache.name : "<destroyed>");
    kfence_print_stack(seq, meta, true);
    if (meta.state == KFENCE_OBJECT_FREED || meta.state == KFENCE_OBJECT_RCU_FREEING) {
    seq_con_printf(seq, "\n");
    kfence_print_stack(seq, meta, false);
    }
    }
//
// Show bytes at @addr that are different from the expected canary values, up to
// @max_bytes.
//
#[no_mangle]
pub unsafe extern "C" fn print_diff_canary(address: c_ulong, bytes_to_show: size_t, meta: *mut kfence_metadata) {
pub static mut show_until_addr: c_ulong = 0;
    let mut cur = core::ptr::null_mut();
    let mut end = core::ptr::null_mut();
// Do not show contents of object nor read into following guard page.
    end = (address < meta.addr ? min(show_until_addr, meta.addr)
    : min(show_until_addr, PAGE_ALIGN(address)));
    pr_cont("[");
    while (cur < end) {
    if (*cur == KFENCE_CANARY_PATTERN_U8(cur)) {
    pr_cont(" .");
    }

    else if (no_hash_pointers) {
    pr_cont(" 0x%02x", *cur);
    }
    else /* Do not leak kernel memory in non-debug builds. */
    pr_cont(" !");
    }
    pr_cont(" ]");
    }
    static const char *get_access_type(bool is_write)
    {
    return str_write_read(is_write);
    }
    enum kfence_fault
    kfence_report_error(unsigned long address, bool is_write, pt_regs *regs,
    const struct kfence_metadata *meta, enum kfence_error_type type)
    {
    unsigned long stack_entries[KFENCE_STACK_DEPTH] = { 0 };
pub static mut object_index: ptrdiff_t = 0;
    let mut num_stack_entries = 0;
pub static mut skipnr: c_int = 0;
    if (regs) {
    num_stack_entries = stack_trace_save_regs(regs, stack_entries, KFENCE_STACK_DEPTH, 0);
    } else {
    num_stack_entries = stack_trace_save(stack_entries, KFENCE_STACK_DEPTH, 1);
    skipnr = get_stack_skipnr(stack_entries, num_stack_entries, &type);
    }
// Require non-NULL meta, except if KFENCE_ERROR_INVALID.
    if (WARN_ON!(type != KFENCE_ERROR_INVALID && !meta)) {
    return KFENCE_FAULT_NONE;
    }
//
// Because we may generate reports in printk-unfriendly parts of the
// kernel, such as scheduler code, the use of printk() could deadlock.
// Until such time that all printing code here is safe in all parts of
// the kernel, accept the risk, and just get our message out (given the
// system might already behave unpredictably due to the memory error).
// As such, also disable lockdep to hide warnings, and avoid disabling
// lockdep for the rest of the kernel.
//
    lockdep_off();
    pr_err!("==================================================================\n");
// Print report header.
    match (type) {
    KFENCE_ERROR_OOB => {
pub static mut left_of_object: bool = false;
    pr_err!("BUG: KFENCE: out-of-bounds %s in %pS\n\n", get_access_type(is_write),
    stack_entries[skipnr]);
    pr_err!("Out-of-bounds %s at 0x%p (%luB %s of kfence-#%td):\n",
    get_access_type(is_write), address,
    left_of_object ? meta.addr - address : address - meta.addr,
    left_of_object ? "left" : "right", object_index);
    // break;
    }
    }
    case KFENCE_ERROR_UAF:
    pr_err!("BUG: KFENCE: use-after-free %s in %pS\n\n", get_access_type(is_write),
    stack_entries[skipnr]);
    pr_err!("Use-after-free %s at 0x%p (in kfence-#%td):\n",
    get_access_type(is_write), address, object_index);
    break;
    case KFENCE_ERROR_CORRUPTION:
    pr_err!("BUG: KFENCE: memory corruption in %pS\n\n", stack_entries[skipnr]);
    pr_err!("Corrupted memory at 0x%p ", address);
    print_diff_canary(address, 16, meta);
    pr_cont(" (in kfence-#%td):\n", object_index);
    break;
    case KFENCE_ERROR_INVALID:
    pr_err!("BUG: KFENCE: invalid %s in %pS\n\n", get_access_type(is_write),
    stack_entries[skipnr]);
    pr_err!("Invalid %s at 0x%p:\n", get_access_type(is_write),
    address);
    break;
    case KFENCE_ERROR_INVALID_FREE:
    pr_err!("BUG: KFENCE: invalid free in %pS\n\n", stack_entries[skipnr]);
    pr_err!("Invalid free of 0x%p (in kfence-#%td):\n", address,
    object_index);
    break;
    }
// Print stack trace and object info.
    stack_trace_print(stack_entries + skipnr, num_stack_entries - skipnr, 0);
    if (meta) {
    lockdep_assert_held(&meta.lock);
    pr_err!("\n");
    kfence_print_object(core::ptr::null_mut(), meta);
    }
// Print report footer.
    pr_err!("\n");
    if (no_hash_pointers && regs) {
    show_regs(regs);
    }
    else {
    dump_stack_print_info(KERN_ERR);
    }
    trace_error_report_end(ERROR_DETECTOR_KFENCE, address);
    pr_err!("==================================================================\n");
    lockdep_on();
    check_panic_on_warn("KFENCE");
// We encountered a memory safety error, taint the kernel!
    add_taint(TAINT_BAD_PAGE, LOCKDEP_STILL_OK);
    return kfence_fault;
    }
#[no_mangle]
pub unsafe extern "C" fn kfence_handle_fault(fault: kfence_fault) {
    match (fault) {
    KFENCE_FAULT_NONE => {
    }
    KFENCE_FAULT_REPORT => {
    // break;
    }
    KFENCE_FAULT_OOPS => {
    BUG();
    // break;
    }
    KFENCE_FAULT_PANIC => {
// Disable KFENCE to avoid recursion if check_on_panic is set.
    WRITE_ONCE(kfence_enabled, false);
    panic("kfence.fault=panic set ...\n");
    // break;
    }
    }
    }

#[no_mangle]
unsafe extern "C" fn kfence_to_kp_stack(track: *const kfence_track, kp_stack: *mut c_void) {
    let mut i = 0;
    let mut j = 0;
    i = get_stack_skipnr(track.stack_entries, track.num_stack_entries, core::ptr::null_mut());
    for (j = 0; i < track.num_stack_entries && j < KS_ADDRS_COUNT; ++i, ++j) {
    kp_stack[j] = track.stack_entries[i];
    }
    if (j < KS_ADDRS_COUNT) {
    kp_stack[j] = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __kfence_obj_info(kpp: *mut kmem_obj_info, object: *mut c_void, slab: *mut slab) -> bool {
    let mut meta = addr_to_metadata((unsigned long)object);
    let mut flags = 0;
    if (!meta) {
    return false;
    }
//
// If state is UNUSED at least show the pointer requested; the rest
// would be garbage data.
//
    kpp.kp_ptr = object;
// Requesting info an a never-used object is almost certainly a bug.
    if (WARN_ON!(meta.state == KFENCE_OBJECT_UNUSED)) {
    return true;
    }
    raw_spin_lock_irqsave(&meta.lock, flags);
    kpp.kp_slab = slab;
    kpp.kp_slab_cache = meta.cache;
    kpp.kp_objp = meta.addr;
    kfence_to_kp_stack(&meta.alloc_track, kpp.kp_stack);
    if (meta.state == KFENCE_OBJECT_FREED || meta.state == KFENCE_OBJECT_RCU_FREEING) {
    kfence_to_kp_stack(&meta.free_track, kpp.kp_free_stack);
    }
// get_stack_skipnr() ensures the first entry is outside allocator.
    kpp.kp_ret = kpp.kp_stack[0];
    raw_spin_unlock_irqrestore(&meta.lock, flags);
    return true;
//! Automatically rewritten from C to Rust
//! Source: mm/kasan/report.c
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
// This file contains common KASAN error reporting code.
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// Author: Andrey Ryabinin <ryabinin.a.a@gmail.com>
//
// Some code borrowed from https://github.com/xairy/kasan-prototype by
// Andrey Konovalov <andreyknvl@gmail.com>
//

    static unsigned long kasan_flags;
pub const KASAN_BIT_REPORTED: c_int = 0;
pub const KASAN_BIT_MULTI_SHOT: c_int = 1;
    enum kasan_arg_fault {
    KASAN_ARG_FAULT_DEFAULT,
    KASAN_ARG_FAULT_REPORT,
    KASAN_ARG_FAULT_PANIC,
    KASAN_ARG_FAULT_PANIC_ON_WRITE,
    };
pub static mut __ro_after_init: kasan_arg_fault kasan_arg_fault = 0;
// kasan.fault=report/panic
#[no_mangle]
unsafe extern "C" fn early_kasan_fault(arg: *mut c_char) -> c_int {
    if (!arg) {
    return -EINVAL;
    }
    if (!strcmp(arg, "report")) {
    kasan_arg_fault = KASAN_ARG_FAULT_REPORT;
    }

    else if (!strcmp(arg, "panic")) {
    kasan_arg_fault = KASAN_ARG_FAULT_PANIC;
    }

    else if (!strcmp(arg, "panic_on_write")) {
    kasan_arg_fault = KASAN_ARG_FAULT_PANIC_ON_WRITE;
    }
    else {
    return -EINVAL;
    }
    return 0;
    }
    early_param!("kasan.fault", early_kasan_fault);
#[no_mangle]
unsafe extern "C" fn kasan_set_multi_shot(str: *mut c_char) -> c_int {
    set_bit(KASAN_BIT_MULTI_SHOT, &kasan_flags);
    return 1;
    }
    __setup!("kasan_multi_shot", kasan_set_multi_shot);
//
// This function is used to check whether KASAN reports are suppressed for
// software KASAN modes via kasan_disable/enable_current() critical sections.
//
// This is done to avoid:
// 1. False-positive reports when accessing slab metadata,
// 2. Deadlocking when poisoned memory is accessed by the reporting code.
//
// Hardware Tag-Based KASAN instead relies on:
// For #1: Resetting tags via kasan_reset_tag().
// For #2: Suppression of tag checks via CPU, see report_suppress_start/end().
//
#[no_mangle]
unsafe extern "C" fn report_suppressed_sw() -> bool {

    if (current.kasan_depth) {
    return true;
    }

    return false;
    }
#[no_mangle]
unsafe extern "C" fn report_suppress_start() {

//
// Disable preemption for the duration of printing a KASAN report, as
// hw_suppress_tag_checks_start() disables checks on the current CPU.
//
    preempt_disable();
    hw_suppress_tag_checks_start();

    kasan_disable_current();

    }
#[no_mangle]
unsafe extern "C" fn report_suppress_stop() {

    hw_suppress_tag_checks_stop();
    preempt_enable();

    kasan_enable_current();

    }
//
// Used to avoid reporting more than one KASAN bug unless kasan_multi_shot
// is enabled. Note that KASAN tests effectively enable kasan_multi_shot
// for their duration.
//
#[no_mangle]
unsafe extern "C" fn report_enabled() -> bool {
    if (test_bit(KASAN_BIT_MULTI_SHOT, &kasan_flags)) {
    return true;
    }
    return !test_and_set_bit(KASAN_BIT_REPORTED, &kasan_flags);
    }

#[no_mangle]
pub unsafe extern "C" fn kasan_save_enable_multi_shot() -> VISIBLE_IF_KUNIT bool {
    return test_and_set_bit(KASAN_BIT_MULTI_SHOT, &kasan_flags);
    }
    EXPORT_SYMBOL_IF_KUNIT(kasan_save_enable_multi_shot);
#[no_mangle]
pub unsafe extern "C" fn kasan_restore_multi_shot(enabled: bool) -> VISIBLE_IF_KUNIT void {
    if (!enabled) {
    clear_bit(KASAN_BIT_MULTI_SHOT, &kasan_flags);
    }
    }
    EXPORT_SYMBOL_IF_KUNIT(kasan_restore_multi_shot);

//
// Whether the KASAN KUnit test suite is currently being executed.
// Updated in kasan_test.c.
//
    static bool kasan_kunit_executing;
#[no_mangle]
pub unsafe extern "C" fn kasan_kunit_test_suite_start() -> VISIBLE_IF_KUNIT void {
    WRITE_ONCE(kasan_kunit_executing, true);
    }
    EXPORT_SYMBOL_IF_KUNIT(kasan_kunit_test_suite_start);
#[no_mangle]
pub unsafe extern "C" fn kasan_kunit_test_suite_end() -> VISIBLE_IF_KUNIT void {
    WRITE_ONCE(kasan_kunit_executing, false);
    }
    EXPORT_SYMBOL_IF_KUNIT(kasan_kunit_test_suite_end);
#[no_mangle]
unsafe extern "C" fn kasan_kunit_test_suite_executing() -> bool {
    return READ_ONCE(kasan_kunit_executing);
    }

#[no_mangle]
pub unsafe extern "C" fn kasan_kunit_test_suite_executing() -> bool { return false; }

#[no_mangle]
unsafe extern "C" fn fail_non_kasan_kunit_test() {
pub static mut test: *mut c_void = core::ptr::null_mut();
    if (kasan_kunit_test_suite_executing()) {
    return;
    }
    test = current.kunit_test;
    if (test) {
    kunit_set_failure(test);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn fail_non_kasan_kunit_test() { }

pub static mut report_lock: usize = 0;
#[no_mangle]
unsafe extern "C" fn start_report(flags: *mut c_ulong) {
    fail_non_kasan_kunit_test();
// Respect the /proc/sys/kernel/traceoff_on_warning interface.
    disable_trace_on_warning();
// Do not allow LOCKDEP mangling KASAN reports.
    lockdep_off();
// Make sure we don't end up in loop.
    report_suppress_start();
    raw_spin_lock_irqsave(&report_lock, *flags);
    pr_err!("==================================================================\n");
    }
#[no_mangle]
unsafe extern "C" fn end_report(flags: *mut c_ulong, addr: *const c_void, is_write: bool) {
    if (addr) {
    trace_error_report_end(ERROR_DETECTOR_KASAN,
    (unsigned long)addr);
    }
    pr_err!("==================================================================\n");
    raw_spin_unlock_irqrestore(&report_lock, *flags);
    if (!test_bit(KASAN_BIT_MULTI_SHOT, &kasan_flags)) {
    check_panic_on_warn("KASAN");
    }
    match (kasan_arg_fault) {
    KASAN_ARG_FAULT_DEFAULT => {
    }
    KASAN_ARG_FAULT_REPORT => {
    // break;
    }
    KASAN_ARG_FAULT_PANIC => {
    panic("kasan.fault=panic set ...\n");
    // break;
    }
    KASAN_ARG_FAULT_PANIC_ON_WRITE => {
    if (is_write) {
    panic("kasan.fault=panic_on_write set ...\n");
    }
    // break;
    }
    }
    add_taint(TAINT_BAD_PAGE, LOCKDEP_NOW_UNRELIABLE);
    lockdep_on();
    report_suppress_stop();
    }
#[no_mangle]
unsafe extern "C" fn print_error_description(info: *mut kasan_report_info) {
    pr_err!("BUG: KASAN: %s in %pS\n", info.bug_type, info.ip);
    if (info.type != KASAN_REPORT_ACCESS) {
    pr_err!("Free of addr %px by task %s/%d\n",
    info.access_addr, current.comm, task_pid_nr(current));
    return;
    }
    if (info.access_size) {
    pr_err!("%s of size %zu at addr %px by task %s/%d\n",
    info.is_write ? "Write" : "Read", info.access_size,
    info.access_addr, current.comm, task_pid_nr(current));
    }
    else {
    pr_err!("%s at addr %px by task %s/%d\n",
    info.is_write ? "Write" : "Read",
    info.access_addr, current.comm, task_pid_nr(current));
    }
    }
#[no_mangle]
unsafe extern "C" fn print_track(track: *mut kasan_track, prefix: *const c_char) {

pub static mut ts_nsec: u64 = 0;
    let mut rem_usec = 0;
    ts_nsec <<= 9;
    rem_usec = do_div(ts_nsec, NSEC_PER_SEC) / 1000;
    pr_err!("%s by task %u on cpu %d at %lu.%06lus:\n",
    prefix, track.pid, track.cpu,
    (unsigned long)ts_nsec, rem_usec);

    pr_err!("%s by task %u:\n", prefix, track.pid);

    if (track.stack) {
    stack_depot_print(track.stack);
    }
    else {
    pr_err!("(stack is not available)\n");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn addr_to_page(addr: *mut c_void) -> *mut c_void {
    if (virt_addr_valid(addr)) {
    return virt_to_head_page(addr);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn describe_object_addr(addr: *const c_void, info: *mut kasan_report_info) {
pub static mut access_addr: c_ulong = 0;
pub static mut object_addr: c_ulong = 0;
    const char *rel_type, *region_state = "";
    let mut rel_bytes = 0;
    pr_err!("The buggy address belongs to the object at %px\n"
    " which belongs to the cache %s of size %d\n",
    info.object, info.cache.name, info.cache.object_size);
    if (access_addr < object_addr) {
    rel_type = "to the left";
    rel_bytes = object_addr - access_addr;
    } else if (access_addr >= object_addr + info.alloc_size) {
    rel_type = "to the right";
    rel_bytes = access_addr - (object_addr + info.alloc_size);
    } else {
    rel_type = "inside";
    rel_bytes = access_addr - object_addr;
    }
//
// Tag-Based modes use the stack ring to infer the bug type, but the
// memory region state description is generated based on the metadata.
// Thus, defining the region state as below can contradict the metadata.
// Fixing this requires further improvements, so only infer the state
// for the Generic mode.
//
    if (IS_ENABLED!(CONFIG_KASAN_GENERIC)) {
    if (strcmp(info.bug_type, "slab-out-of-bounds") == 0) {
    region_state = "allocated ";
    }

    else if (strcmp(info.bug_type, "slab-use-after-free") == 0) {
    region_state = "freed ";
    }
    }
    pr_err!("The buggy address is located %d bytes %s of\n"
    " %s%zu-byte region [%px, %px)\n",
    rel_bytes, rel_type, region_state, info.alloc_size,
    object_addr, (object_addr + info.alloc_size));
    }
#[no_mangle]
unsafe extern "C" fn describe_object_stacks(info: *mut kasan_report_info) {
    if (info.alloc_track.stack) {
    print_track(&info.alloc_track, "Allocated");
    pr_err!("\n");
    }
    if (info.free_track.stack) {
    print_track(&info.free_track, "Freed");
    pr_err!("\n");
    }
    kasan_print_aux_stacks(info.cache, info.object);
    }
#[no_mangle]
unsafe extern "C" fn describe_object(addr: *const c_void, info: *mut kasan_report_info) {
    if (kasan_stack_collection_enabled()) {
    describe_object_stacks(info);
    }
    describe_object_addr(addr, info);
    }
#[no_mangle]
pub unsafe extern "C" fn kernel_or_module_addr(addr: *const c_void) -> bool {
    if (is_kernel((unsigned long)addr)) {
    return true;
    }
    if (is_module_address((unsigned long)addr)) {
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn init_task_stack_addr(addr: *const c_void) -> bool {
    return addr >= &init_thread_union.stack &&
    (addr <= &init_thread_union.stack +
    sizeof!(init_thread_union.stack));
    }
#[no_mangle]
pub unsafe extern "C" fn print_address_description(addr: *mut c_void, tag: u8, info: *mut kasan_report_info) {
    let mut page = addr_to_page(addr);
    dump_stack_lvl(KERN_ERR);
    pr_err!("\n");
    if (info.cache && info.object) {
    describe_object(addr, info);
    pr_err!("\n");
    }
    if (kernel_or_module_addr(addr) && !init_task_stack_addr(addr)) {
    pr_err!("The buggy address belongs to the variable:\n");
    pr_err!(" %pS\n", addr);
    pr_err!("\n");
    }
    if (object_is_on_stack(addr)) {
//
// Currently, KASAN supports printing frame information only
// for accesses to the task's own stack.
//
    kasan_print_address_stack_frame(addr);
    pr_err!("\n");
    }
    if (is_vmalloc_addr(addr)) {
    pr_err!("The buggy address belongs to a");
    if (!vmalloc_dump_obj(addr)) {
    pr_cont(" vmalloc virtual mapping\n");
    }
    page = vmalloc_to_page(addr);
    }
    if (page) {
    pr_err!("The buggy address belongs to the physical page:\n");
    dump_page(page, "kasan: bad access detected");
    pr_err!("\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn meta_row_is_guilty(row: *const c_void, addr: *const c_void) -> bool {
    return (row <= addr) && (addr < row + META_MEM_BYTES_PER_ROW);
    }
#[no_mangle]
unsafe extern "C" fn meta_pointer_offset(row: *const c_void, addr: *const c_void) -> c_int {
//
// Memory state around the buggy address:
// ff00ff00ff00ff00: 00 00 00 05 fe fe fe fe fe fe fe fe fe fe fe fe
// ...
//
// The length of ">ff00ff00ff00ff00: " is
// 3 + (BITS_PER_LONG / 8) * 2 chars.
// The length of each granule metadata is 2 bytes
// plus 1 byte for space.
//
    return 3 + (BITS_PER_LONG / 8) * 2 +
    (addr - row) / KASAN_GRANULE_SIZE * 3 + 1;
    }
#[no_mangle]
unsafe extern "C" fn print_memory_metadata(addr: *const c_void) {
    let mut i = 0;
pub static mut row: *mut c_void = core::ptr::null_mut();
    row = round_down((unsigned long)addr, META_MEM_BYTES_PER_ROW)
    - META_ROWS_AROUND_ADDR * META_MEM_BYTES_PER_ROW;
    pr_err!("Memory state around the buggy address:\n");
    while (i <= META_ROWS_AROUND_ADDR) {
    char buffer[4 + (BITS_PER_LONG / 8) * 2];
    char metadata[META_BYTES_PER_ROW];
    snprintf(buffer, sizeof!(buffer),
    (i == 0) ? ">%px: " : " %px: ", row);
//
// We should not pass a shadow pointer to generic
// function, because generic functions may try to
// access kasan mapping for the passed address.
//
    kasan_metadata_fetch_row(&metadata[0], row);
    print_hex_dump(KERN_ERR, buffer,
    DUMP_PREFIX_NONE, META_BYTES_PER_ROW, 1,
    metadata, META_BYTES_PER_ROW, 0);
    if (meta_row_is_guilty(row, addr)) {
    pr_err!("%*c\n", meta_pointer_offset(row, addr), '^');
    }
    row += META_MEM_BYTES_PER_ROW;
    }
    }
#[no_mangle]
unsafe extern "C" fn print_report(info: *mut kasan_report_info) {
    let mut addr = kasan_reset_tag(info.access_addr);
pub static mut tag: u8 = 0;
    print_error_description(info);
    if (addr_has_metadata(addr)) {
    kasan_print_tags(tag, info.first_bad_addr);
    }
    pr_err!("\n");
    if (addr_has_metadata(addr)) {
    print_address_description(addr, tag, info);
    print_memory_metadata(info.first_bad_addr);
    } else {
    dump_stack_lvl(KERN_ERR);
    }
    }
#[no_mangle]
unsafe extern "C" fn complete_report_info(info: *mut kasan_report_info) {
    let mut addr = kasan_reset_tag(info.access_addr);
pub static mut slab: *mut c_void = core::ptr::null_mut();
    if (info.type == KASAN_REPORT_ACCESS) {
    info.first_bad_addr = kasan_find_first_bad_addr(
    info.access_addr, info.access_size);
    }
    else {
    info.first_bad_addr = addr;
    }
    slab = kasan_addr_to_slab(addr);
    if (slab) {
    info.cache = slab.slab_cache;
    info.object = nearest_obj(info.cache, slab, addr);
// Try to determine allocation size based on the metadata.
    info.alloc_size = kasan_get_alloc_size(info.object, info.cache);
// Fallback to the object size if failed.
    if (!info.alloc_size) {
    info.alloc_size = info.cache.object_size;
    }
    } else {
    info.cache = info.object = core::ptr::null_mut();
    }
    match (info.type) {
    KASAN_REPORT_INVALID_FREE => {
    info.bug_type = "invalid-free";
    // break;
    }
    KASAN_REPORT_DOUBLE_FREE => {
    info.bug_type = "double-free";
    // break;
    }
    _ => {
// bug_type filled in by kasan_complete_mode_report_info.
    // break;
    }
    }
// Fill in mode-specific report info fields.
    kasan_complete_mode_report_info(info);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_report_invalid_free(ptr: *mut c_void, ip: c_ulong, type: kasan_report_type) {
    let mut flags = 0;
pub static mut info: usize = 0;
//
// Do not check report_suppressed_sw(), as an invalid-free cannot be
// caused by accessing poisoned memory and thus should not be suppressed
// by kasan_disable/enable_current() critical sections.
//
// Note that for Hardware Tag-Based KASAN, kasan_report_invalid_free()
// is triggered by explicit tag checks and not by the ones performed by
// the CPU. Thus, reporting invalid-free is not suppressed as well.
//
    if (unlikely(!report_enabled())) {
    return;
    }
    start_report(&flags);
    __memset(&info, 0, sizeof!(info));
    info.type = type;
    info.access_addr = ptr;
    info.access_size = 0;
    info.is_write = false;
    info.ip = ip;
    complete_report_info(&info);
    print_report(&info);
//
// Invalid free is considered a "write" since the allocator's metadata
// updates involves writes.
//
    end_report(&flags, ptr, true);
    }
//
// kasan_report() is the only reporting function that uses
// user_access_save/restore(): kasan_report_invalid_free() cannot be called
// from a UACCESS region, and kasan_report_async() is not used on x86.
//
#[no_mangle]
pub unsafe extern "C" fn kasan_report(addr: *mut c_void, size: size_t, is_write: bool, ip: c_ulong) -> bool {
pub static mut ret: bool = true;
pub static mut ua_flags: c_ulong = 0;
    let mut irq_flags = 0;
pub static mut info: usize = 0;
    if (unlikely(report_suppressed_sw()) || unlikely(!report_enabled())) {
    ret = false;
// goto;
    }
    start_report(&irq_flags);
    __memset(&info, 0, sizeof!(info));
    info.type = KASAN_REPORT_ACCESS;
    info.access_addr = addr;
    info.access_size = size;
    info.is_write = is_write;
    info.ip = ip;
    complete_report_info(&info);
    print_report(&info);
    end_report(&irq_flags, addr, is_write);
// label;
    user_access_restore(ua_flags);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn kasan_report_async() {
    let mut flags = 0;
//
// Do not check report_suppressed_sw(), as
// kasan_disable/enable_current() critical sections do not affect
// Hardware Tag-Based KASAN.
//
    if (unlikely(!report_enabled())) {
    return;
    }
    start_report(&flags);
    pr_err!("BUG: KASAN: invalid-access\n");
    pr_err!("Asynchronous fault: no details available\n");
    pr_err!("\n");
    dump_stack_lvl(KERN_ERR);
//
// Conservatively set is_write=true, because no details are available.
// In this mode, kasan.fault=panic_on_write is like kasan.fault=panic.
//
    end_report(&flags, core::ptr::null_mut(), true);
    }

//
// With compiler-based KASAN modes, accesses to bogus pointers (outside of the
// mapped kernel address space regions) cause faults when KASAN tries to check
// the shadow memory before the actual memory access. This results in cryptic
// GPF reports, which are hard for users to interpret. This hook helps users to
// figure out what the original bogus pointer was.
//
#[no_mangle]
pub unsafe extern "C" fn kasan_non_canonical_hook(addr: c_ulong) {
    unsigned long orig_addr, user_orig_addr;
pub static mut bug_type: *mut c_void = core::ptr::null_mut();
//
// All addresses that came as a result of the memory-to-shadow mapping
// (even for bogus pointers) must be >= KASAN_SHADOW_OFFSET.
//
    if (addr < KASAN_SHADOW_OFFSET) {
    return;
    }
    orig_addr = (unsigned long)kasan_shadow_to_mem(addr);
// Strip pointer tag before comparing against userspace ranges
    user_orig_addr = (unsigned long)set_tag(orig_addr, 0);
//
// For faults near the shadow address for NULL, we can be fairly certain
// that this is a KASAN shadow memory access.
// For faults that correspond to the shadow for low or high canonical
// addresses, we can still be pretty sure: these shadow regions are a
// fairly narrow chunk of the address space.
// But the shadow for non-canonical addresses is a really large chunk
// of the address space. For this case, we still print the decoded
// address, but make it clear that this is not necessarily what's
// actually going on.
//
    if (user_orig_addr < PAGE_SIZE) {
    bug_type = "null-ptr-deref";
    orig_addr = user_orig_addr;
    } else if (user_orig_addr < TASK_SIZE) {
    bug_type = "probably user-memory-access";
    orig_addr = user_orig_addr;
    } else if (addr_in_shadow(addr)) {
    bug_type = "probably wild-memory-access";
    }
    else {
    bug_type = "maybe wild-memory-access";
    }
    pr_alert("KASAN: %s in range [0x%016lx-0x%016lx]\n", bug_type,
    orig_addr, orig_addr + KASAN_GRANULE_SIZE - 1);
    }
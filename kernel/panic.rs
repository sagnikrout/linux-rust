//! Automatically rewritten from C to Rust
//! Source: kernel/panic.c
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



// SPDX-License-Identifier: GPL-2.0-only
//
// linux/kernel/panic.c
//
// Copyright (C) 1991, 1992  Linus Torvalds
//
// This function is used through-out the kernel (including mm and fs)
// to indicate a major problem.
//

pub const PANIC_TIMER_STEP: c_int = 100;
pub const PANIC_BLINK_SPD: c_int = 18;
pub const PANIC_MSG_BUFSZ: c_int = 1024;

//
// Should we dump all CPUs backtraces in an oops event?
// Defaults to 0, can be changed via sysctl.
//
    static unsigned int  sysctl_oops_all_cpu_backtrace;

pub const sysctl_oops_all_cpu_backtrace: c_int = 0;

pub static mut panic_on_oops: c_int = 0;
    static unsigned long tainted_mask =
    IS_ENABLED!(CONFIG_RANDSTRUCT) ? (1 << TAINT_RANDSTRUCT) : 0;
    static int pause_on_oops;
    static int pause_on_oops_flag;
// static DEFINE_SPINLOCK(pause_on_oops_lock);
    let mut crash_kexec_post_notifiers = 0;
    let mut panic_on_warn = 0;
    let mut panic_on_taint = 0;
pub static mut panic_on_taint_nousertaint: bool = false;
    static unsigned int warn_limit ;
    static bool panic_console_replay;
    let mut panic_triggering_all_cpu_backtrace = 0;
    static bool panic_this_cpu_backtrace_printed;
pub static mut panic_timeout: c_int = 0;
    EXPORT_SYMBOL_GPL(panic_timeout);
    let mut panic_print = 0;
pub static mut panic_force_cpu: int = 0;
    ATOMIC_NOTIFIER_HEAD(panic_notifier_list);
    EXPORT_SYMBOL(panic_notifier_list);
#[no_mangle]
unsafe extern "C" fn panic_print_deprecated() {
    pr_info_once!("Kernel: The 'panic_print' parameter is now deprecated. Please use 'panic_sys_info' and 'panic_console_replay' instead.\n");
    }

//
// Taint values can only be increased
// This means we can safely use a temporary.
//
#[no_mangle]
pub unsafe extern "C" fn proc_taint(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
pub static mut t: usize = 0;
pub static mut tmptaint: c_ulong = 0;
    let mut err = 0;
    if (write && !capable(CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    t = *table;
    t.data = &tmptaint;
    err = proc_doulongvec_minmax(&t, write, buffer, lenp, ppos);
    if (err < 0) {
    return err;
    }
    if (write) {
    let mut i = 0;
//
// If we are relying on panic_on_taint not producing
// false positives due to userspace input, bail out
// before setting the requested taint flags.
//
    if (panic_on_taint_nousertaint && (tmptaint & panic_on_taint)) {
    return -EINVAL;
    }
//
// Poor man's atomic or. Not worth adding a primitive
// to everyone's atomic.h for this
//
    for (i = 0; i < TAINT_FLAGS_COUNT; i++) {
    if ((1UL << i) & tmptaint)
    add_taint(i, LOCKDEP_STILL_OK);
    }
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn sysctl_panic_print_handler(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    if (write) {
    panic_print_deprecated();
    }
    return proc_doulongvec_minmax(table, write, buffer, lenp, ppos);
    }
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn kernel_panic_sysctls_init() -> __init int {
    register_sysctl_init("kernel", kern_panic_table);
    return 0;
    }
    late_initcall!(kernel_panic_sysctls_init);

// The format is "panic_sys_info=tasks,mem,locks,ftrace,..."
#[no_mangle]
unsafe extern "C" fn setup_panic_sys_info(buf: *mut c_char) -> c_int {
// There is no risk of race in kernel boot phase
    panic_print = sys_info_parse_param(buf);
    return 1;
    }
    __setup!("panic_sys_info=", setup_panic_sys_info);
pub static mut warn_count: atomic_t = 0;

#[no_mangle]
pub unsafe extern "C" fn warn_count_show(kobj: *mut kobject, attr: *mut kobj_attribute, page: *mut c_char) -> ssize_t {
    return sysfs_emit(page, "%d\n", atomic_read(&warn_count));
    }
pub static mut warn_count_attr: kobj_attribute = 0;
#[no_mangle]
unsafe extern "C" fn kernel_panic_sysfs_init() -> __init int {
    sysfs_add_file_to_group(kernel_kobj, &warn_count_attr.attr, core::ptr::null_mut());
    return 0;
    }
    late_initcall!(kernel_panic_sysfs_init);

#[no_mangle]
unsafe extern "C" fn no_blink(state: c_int) -> c_long {
    return 0;
    }
// Returns how long it waited in ms
    long (*panic_blink)(int state);
    EXPORT_SYMBOL(panic_blink);
//
// Stop ourself in panic -- architecture code may override this
//
#[no_mangle]
pub unsafe extern "C" fn panic_smp_self_stop() -> void __weak __noreturn {
    while (1) {
    cpu_relax();
    }
    }
//
// Stop ourselves in NMI context if another CPU has already panicked. Arch code
// may override this to prepare for crash dumping, e.g. save regs info.
//
#[no_mangle]
pub unsafe extern "C" fn nmi_panic_self_stop(regs: *mut pt_regs) -> void __weak __noreturn {
    panic_smp_self_stop();
    }
//
// Stop other CPUs in panic.  Architecture dependent code may override this
// with more suitable version.  For example, if the architecture supports
// crash dump, it should save registers of each stopped CPU and disable
// per-CPU features such as virtualization extensions.
//
#[no_mangle]
pub unsafe extern "C" fn crash_smp_send_stop() -> void __weak {
    static int cpus_stopped;
//
// This function can be called twice in panic path, but obviously
// we execute this only once.
//
    if (cpus_stopped) {
    return;
    }
//
// Note smp_send_stop is the usual smp shutdown function, which
// unfortunately means it may not be hardened to work in a panic
// situation.
//
    smp_send_stop();
    cpus_stopped = 1;
    }
pub static mut panic_cpu: core::sync::atomic::AtomicI32 = 0;
pub static mut panic_redirect_cpu: core::sync::atomic::AtomicI32 = 0;

pub static mut panic_force_buf: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn panic_force_cpu_setup(str: *mut c_char) -> c_int {
    let mut cpu = 0;
    if (!str) {
    return -EINVAL;
    }
    if (kstrtoint(str, 0, &cpu) || cpu < 0 || cpu >= nr_cpu_ids) {
    pr_warn!("panic_force_cpu: invalid value '%s'\n", str);
    return -EINVAL;
    }
    panic_force_cpu = cpu;
    return 0;
    }
    early_param!("panic_force_cpu", panic_force_cpu_setup);
#[no_mangle]
unsafe extern "C" fn panic_force_cpu_late_init() -> c_int {
    if (panic_force_cpu < 0) {
    return 0;
    }
    panic_force_buf = kmalloc(PANIC_MSG_BUFSZ, GFP_KERNEL);
    return 0;
    }
    late_initcall!(panic_force_cpu_late_init);
#[no_mangle]
unsafe extern "C" fn do_panic_on_target_cpu(info: *mut c_void) {
    panic("%s", info);
    }
//
// panic_smp_redirect_cpu - Redirect panic to target CPU
// @target_cpu: CPU that should handle the panic
// @msg: formatted panic message
//
// Default implementation uses IPI. Architectures with NMI support
// can override this for more reliable delivery.
//
// Return: 0 on success, negative errno on failure
//
#[no_mangle]
pub unsafe extern "C" fn panic_smp_redirect_cpu(target_cpu: c_int, msg: *mut c_void) -> int __weak {
    static call_single_data_t panic_csd;
    panic_csd.func = do_panic_on_target_cpu;
    panic_csd.info = msg;
    return smp_call_function_single_async(target_cpu, &panic_csd);
    }
//
// panic_try_force_cpu - Redirect panic to a specific CPU for crash kernel
// @fmt: panic message format string
// @args: arguments for format string
//
// Some platforms require panic handling to occur on a specific CPU
// for the crash kernel to function correctly. This function redirects
// panic handling to the CPU specified via the panic_force_cpu= boot parameter.
//
// Returns false if panic should proceed on current CPU.
// Returns true if panic was redirected.
//
    __printf(1, 0)
#[no_mangle]
unsafe extern "C" fn panic_try_force_cpu(fmt: *const c_char, args: va_list) -> bool {
pub static mut this_cpu: c_int = 0;
pub static mut old_cpu: c_int = 0;
pub static mut msg: *mut c_void = core::ptr::null_mut();
// Feature not enabled via boot parameter
    if (panic_force_cpu < 0) {
    return false;
    }
// Already on target CPU - proceed normally
    if (this_cpu == panic_force_cpu) {
    return false;
    }
// Target CPU is offline, can't redirect
    if (!cpu_online(panic_force_cpu)) {
    pr_warn!("panic: target CPU %d is offline, continuing on CPU %d\n",
    panic_force_cpu, this_cpu);
    return false;
    }
// Another panic already in progress
    if (panic_in_progress()) {
    return false;
    }
//
// Only one CPU can do the redirect. Use atomic cmpxchg to ensure
// we don't race with another CPU also trying to redirect.
//
    if (!atomic_try_cmpxchg(&panic_redirect_cpu, &old_cpu, this_cpu)) {
    return false;
    }
//
// Use dynamically allocated buffer if available, otherwise
// fall back to static message for early boot panics or allocation failure.
//
    if (panic_force_buf) {
    vsnprintf(panic_force_buf, PANIC_MSG_BUFSZ, fmt, args);
    msg = panic_force_buf;
    } else {
    msg = "Redirected panic (buffer unavailable)";
    }
    console_verbose();
    bust_spinlocks(1);
    pr_emerg("panic: Redirecting from CPU %d to CPU %d for crash kernel.\n",
    this_cpu, panic_force_cpu);
// Dump original CPU before redirecting
    if (!test_taint(TAINT_DIE) &&
    oops_in_progress <= 1 &&
    IS_ENABLED!(CONFIG_DEBUG_BUGVERBOSE)) {
    dump_stack();
    }
    if (panic_smp_redirect_cpu(panic_force_cpu, msg) != 0) {
    atomic_set(&panic_redirect_cpu, PANIC_CPU_INVALID);
    pr_warn!("panic: failed to redirect to CPU %d, continuing on CPU %d\n",
    panic_force_cpu, this_cpu);
    return false;
    }
// IPI/NMI sent, this CPU should stop
    return true;
    }

    __printf(1, 0)
#[no_mangle]
pub unsafe extern "C" fn panic_try_force_cpu(fmt: *const c_char, args: va_list) -> bool {
    return false;
    }

#[no_mangle]
pub unsafe extern "C" fn panic_try_start() -> bool {
    let mut old_cpu = 0;
    let mut this_cpu = 0;
//
// Only one CPU is allowed to execute the crash_kexec() code as with
// panic().  Otherwise parallel calls of panic() and crash_kexec()
// may stop each other.  To exclude them, we use panic_cpu here too.
//
    old_cpu = PANIC_CPU_INVALID;
    this_cpu = raw_smp_processor_id();
    return atomic_try_cmpxchg(&panic_cpu, &old_cpu, this_cpu);
    }
    EXPORT_SYMBOL(panic_try_start);
#[no_mangle]
pub unsafe extern "C" fn panic_reset() {
    atomic_set(&panic_cpu, PANIC_CPU_INVALID);
    }
    EXPORT_SYMBOL(panic_reset);
#[no_mangle]
pub unsafe extern "C" fn panic_in_progress() -> bool {
    return unlikely(atomic_read(&panic_cpu) != PANIC_CPU_INVALID);
    }
    EXPORT_SYMBOL(panic_in_progress);
// Return true if a panic is in progress on the current CPU.
#[no_mangle]
pub unsafe extern "C" fn panic_on_this_cpu() -> bool {
//
// We can use raw_smp_processor_id() here because it is impossible for
// the task to be migrated to the panic_cpu, or away from it. If
// panic_cpu has already been set, and we're not currently executing on
// that CPU, then we never will be.
//
    return unlikely(atomic_read(&panic_cpu) == raw_smp_processor_id());
    }
    EXPORT_SYMBOL(panic_on_this_cpu);
//
// Return true if a panic is in progress on a remote CPU.
//
// On true, the local CPU should immediately release any printing resources
// that may be needed by the panic CPU.
//
#[no_mangle]
pub unsafe extern "C" fn panic_on_other_cpu() -> bool {
    return (panic_in_progress() && !panic_on_this_cpu());
    }
    EXPORT_SYMBOL(panic_on_other_cpu);
//
// A variant of panic() called from NMI context. We return if we've already
// panicked on this CPU. If another CPU already panicked, loop in
// nmi_panic_self_stop() which can provide architecture dependent code such
// as saving register state for crash dump.
//
#[no_mangle]
pub unsafe extern "C" fn nmi_panic(regs: *mut pt_regs, msg: *const c_char) {
    if (panic_try_start()) {
    panic("%s", msg);
    }

    else if (panic_on_other_cpu()) {
    nmi_panic_self_stop(regs);
    }
    }
    EXPORT_SYMBOL(nmi_panic);
#[no_mangle]
pub unsafe extern "C" fn check_panic_on_warn(origin: *const c_char) {
    let mut limit = 0;
    if (panic_on_warn) {
    panic("%s: panic_on_warn set ...\n", origin);
    }
    limit = READ_ONCE(warn_limit);
    if (atomic_inc_return(&warn_count) >= limit && limit) {
    panic("%s: system warned too often (kernel.warn_limit is %d)",
    origin, limit);
    }
    }
#[no_mangle]
unsafe extern "C" fn panic_trigger_all_cpu_backtrace() {
// Temporary allow non-panic CPUs to write their backtraces.
    panic_triggering_all_cpu_backtrace = true;
    if (panic_this_cpu_backtrace_printed) {
    trigger_allbutcpu_cpu_backtrace(raw_smp_processor_id());
    }
    else {
    trigger_all_cpu_backtrace();
    }
    panic_triggering_all_cpu_backtrace = false;
    }
//
// Helper that triggers the NMI backtrace (if set in panic_print)
// and then performs the secondary CPUs shutdown - we cannot have
// the NMI backtrace after the CPUs are off!
//
#[no_mangle]
unsafe extern "C" fn panic_other_cpus_shutdown(crash_kexec: bool) {
    if (panic_print & SYS_INFO_ALL_BT) {
    panic_trigger_all_cpu_backtrace();
    }
//
// Note that smp_send_stop() is the usual SMP shutdown function,
// which unfortunately may not be hardened to work in a panic
// situation. If we want to do crash dump after notifier calls
// and kmsg_dump, we will need architecture dependent extra
// bits in addition to stopping other CPUs, hence we rely on
// crash_smp_send_stop() for that.
//
    if (!crash_kexec) {
    smp_send_stop();
    }
    else {
    crash_smp_send_stop();
    }
    }
//
// vpanic - halt the system
// @fmt: The text string to print
// @args: Arguments for the format string
//
// Display a message, then perform cleanups. This function never returns.
//
#[no_mangle]
pub unsafe extern "C" fn vpanic(fmt: *const c_char, args: va_list) {
    static char buf[PANIC_MSG_BUFSZ];
    long i, i_next = 0, len;
pub static mut state: c_int = 0;
pub static mut _crash_kexec_post_notifiers: bool = false;
    if (panic_on_warn) {
//
// This thread may hit another WARN() in the panic path.
// Resetting this prevents additional WARN() from panicking the
// system on this thread.  Other threads are blocked by the
// panic_mutex in panic().
//
    panic_on_warn = 0;
    }
//
// Disable local interrupts. This will prevent panic_smp_self_stop
// from deadlocking the first cpu that invokes the panic, since
// there is nothing to prevent an interrupt handler (that runs
// after setting panic_cpu) from invoking panic() again.
//
    local_irq_disable();
    preempt_disable_notrace();
// Redirect panic to target CPU if configured via panic_force_cpu=.
    if (panic_try_force_cpu(fmt, args)) {
//
// Mark ourselves offline so panic_other_cpus_shutdown() won't wait
// for us on architectures that check num_online_cpus().
//
    set_cpu_online(smp_processor_id(), false);
    panic_smp_self_stop();
    }
//
// It's possible to come here directly from a panic-assertion and
// not have preempt disabled. Some functions called from here want
// preempt to be disabled. No point enabling it later though...
//
// Only one CPU is allowed to execute the panic code from here. For
// multiple parallel invocations of panic, all other CPUs either
// stop themself or will wait until they are stopped by the 1st CPU
// with smp_send_stop().
//
// cmpxchg success means this is the 1st CPU which comes here,
// so go ahead.
// `old_cpu == this_cpu' means we came from nmi_panic() which sets
// panic_cpu to this CPU.  In this case, this is also the 1st CPU.
//
// atomic_try_cmpxchg updates old_cpu on failure
    if (panic_try_start()) {
// go ahead
    } else if (panic_on_other_cpu()) {
    panic_smp_self_stop();
    }
    console_verbose();
    bust_spinlocks(1);
    len = vscnprintf(buf, sizeof!(buf), fmt, args);
    if (len && buf[len - 1] == '\n') {
    buf[len - 1] = '\0';
    }
    pr_emerg("Kernel panic - not syncing: %s\n", buf);
//
// Avoid nested stack-dumping if a panic occurs during oops processing
//
    if (atomic_read(&panic_redirect_cpu) != PANIC_CPU_INVALID &&
    panic_force_cpu == raw_smp_processor_id()) {
    pr_emerg("panic: Redirected from CPU %d, skipping stack dump.\n",
    atomic_read(&panic_redirect_cpu));
    } else if (test_taint(TAINT_DIE) || oops_in_progress > 1) {
    panic_this_cpu_backtrace_printed = true;
    } else if (IS_ENABLED!(CONFIG_DEBUG_BUGVERBOSE)) {
    dump_stack();
    panic_this_cpu_backtrace_printed = true;
    }
//
// If kgdb is enabled, give it a chance to run before we stop all
// the other CPUs or else we won't be able to debug processes left
// running on them.
//
    kgdb_panic(buf);
//
// If we have crashed and we have a crash kernel loaded let it handle
// everything else.
// If we want to run this after calling panic_notifiers, pass
// the "crash_kexec_post_notifiers" option to the kernel.
//
// Bypass the panic_cpu check and call __crash_kexec directly.
//
    if (!_crash_kexec_post_notifiers) {
    __crash_kexec(core::ptr::null_mut());
    }
    panic_other_cpus_shutdown(_crash_kexec_post_notifiers);
    printk_legacy_allow_panic_sync();
//
// Run any panic handlers, including those that might need to
// add information to the kmsg dump output.
//
    atomic_notifier_call_chain(&panic_notifier_list, 0, buf);
    sys_info(panic_print);
    kmsg_dump_desc(KMSG_DUMP_PANIC, buf);
//
// If you doubt kdump always works fine in any situation,
// "crash_kexec_post_notifiers" offers you a chance to run
// panic_notifiers and dumping kmsg before kdump.
// Note: since some panic_notifiers can make crashed kernel
// more unstable, it can increase risks of the kdump failure too.
//
// Bypass the panic_cpu check and call __crash_kexec directly.
//
    if (_crash_kexec_post_notifiers) {
    __crash_kexec(core::ptr::null_mut());
    }
    console_unblank();
//
// We may have ended up stopping the CPU holding the lock (in
// smp_send_stop()) while still having some valuable data in the console
// buffer.  Try to acquire the lock then release it regardless of the
// result.  The release will also print the buffers out.  Locks debug
// should be disabled to avoid reporting bad unlock balance when
// panic() is not being callled from OOPS.
//
    debug_locks_off();
    console_flush_on_panic(CONSOLE_FLUSH_PENDING);
    if ((panic_print & SYS_INFO_PANIC_CONSOLE_REPLAY) ||
    panic_console_replay) {
    console_flush_on_panic(CONSOLE_REPLAY_ALL);
    }
    if (!panic_blink) {
    panic_blink = no_blink;
    }
    if (panic_timeout > 0) {
//
// Delay timeout seconds before rebooting the machine.
// We can't use the "normal" timers since we just panicked.
//
    pr_emerg("Rebooting in %d seconds..\n", panic_timeout);
    while (i < panic_timeout * 1000) {
    touch_nmi_watchdog();
    if (i >= i_next) {
    i += panic_blink(state ^= 1);
    i_next = i + 3600 / PANIC_BLINK_SPD;
    }
    mdelay(PANIC_TIMER_STEP);
    }
    }
    if (panic_timeout != 0) {
//
// This will not be a clean reboot, with everything
// shutting down.  But if there is a chance of
// rebooting the system it will be rebooted.
//
    if (panic_reboot_mode != REBOOT_UNDEFINED) {
    reboot_mode = panic_reboot_mode;
    }
    emergency_restart();
    }

    {
extern "C" { pub static mut stop_a_enabled: usize; }
// Make sure the user can actually press Stop-A (L1-A)
    stop_a_enabled = 1;
    pr_emerg("Press Stop-A (L1-A) from sun keyboard or send break\n"
    "twice on console to return to the boot prom\n");
    }

    disabled_wait();

    pr_emerg("---[ end Kernel panic - not syncing: %s ]---\n", buf);
// Do not scroll important messages printed above
    suppress_printk = 1;
//
// The final messages may not have been printed if in a context that
// defers printing (such as NMI) and irq_work is not available.
// Explicitly flush the kernel log buffer one last time.
//
    console_flush_on_panic(CONSOLE_FLUSH_PENDING);
    nbcon_atomic_flush_unsafe();
    local_irq_enable();
    while ( ) {
    touch_softlockup_watchdog();
    if (i >= i_next) {
    i += panic_blink(state ^= 1);
    i_next = i + 3600 / PANIC_BLINK_SPD;
    }
    mdelay(PANIC_TIMER_STEP);
    }
    }
    EXPORT_SYMBOL(vpanic);
// Identical to vpanic(), except it takes variadic arguments instead of va_list
#[no_mangle]
pub unsafe extern "C" fn panic(fmt: *const c_char, ...) {
    let mut args;
    va_start(args, fmt);
    vpanic(fmt, args);
    va_end(args);
    }
    EXPORT_SYMBOL(panic);

    [ TAINT_##taint ] = {						
    .c_true = _c_true, .c_false = _c_false,			
    .desc = #taint,						
    }
//
// NOTE: if you modify the taint_flags or TAINT_FLAGS_COUNT,
// please also modify tools/debugging/kernel-chktaint and
// Documentation/admin-guide/tainted-kernels.rst, including its
// small shell script that prints the TAINT_FLAGS_COUNT bits of
// /proc/sys/kernel/tainted.
//
// Also, update INIT_TAINT_BUF_MAX below.
//
pub static mut taint_flag: usize = 0;

#[no_mangle]
unsafe extern "C" fn print_tainted_seq(s: *mut seq_buf, verbose: bool) {
    let mut sep = "";
    let mut i = 0;
    if (!tainted_mask) {
    seq_buf_puts(s, "Not tainted");
    return;
    }
    seq_buf_printf(s, "Tainted: ");
    while (i < TAINT_FLAGS_COUNT) {
    let mut t = &taint_flags[i];
pub static mut is_set: bool = false;
pub static mut c: c_char = 0;
    if (verbose) {
    if (is_set) {
    seq_buf_printf(s, "%s[%c]=%s", sep, c, t.desc);
    sep = ", ";
    }
    } else {
    seq_buf_putc(s, c);
    }
    }
    }
// The initial buffer can accommodate all taint flags in verbose
// mode, with some headroom. Once the allocator is available, the
// exact size is allocated dynamically; the initial buffer remains
// as a fallback if allocation fails.
//
// The verbose taint string currently requires up to 327 characters.
//
pub const INIT_TAINT_BUF_MAX: c_int = 350;
    static char init_taint_buf[INIT_TAINT_BUF_MAX] __initdata;
pub static mut __refdata: *mut static char taint_buf = core::ptr::null_mut();
pub static mut taint_buf_size: size_t = 0;
#[no_mangle]
unsafe extern "C" fn alloc_taint_buf() -> __init int {
    let mut i = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
pub static mut size: usize = 0;
    size += sizeof!("Tainted: ") - 1;
    while (i < TAINT_FLAGS_COUNT) {
    size += 2; /* For ", " */
    size += 4; /* For "[%c]=" */
    size += strlen(taint_flags[i].desc);
    }
    size += 1; /* For core::ptr::null_mut() terminator */
    buf = kmalloc(size, GFP_KERNEL);
    if (!buf) {
    panic("Failed to allocate taint string buffer");
    }
    taint_buf = buf;
    taint_buf_size = size;
    return 0;
    }
    postcore_initcall!(alloc_taint_buf);
    static const char *_print_tainted(bool verbose)
    {
pub static mut s: usize = 0;
    BUILD_BUG_ON!(ARRAY_SIZE!(taint_flags) != TAINT_FLAGS_COUNT);
    seq_buf_init(&s, taint_buf, taint_buf_size);
    print_tainted_seq(&s, verbose);
    return seq_buf_str(&s);
    }
//
// print_tainted - return a string to represent the kernel taint state.
//
// For individual taint flag meanings, see Documentation/admin-guide/sysctl/kernel.rst
//
// The string is overwritten by the next call to print_tainted(),
// but is always NULL terminated.
//
    const char *print_tainted(void)
    {
    return _print_tainted(false);
    }
//
// print_tainted_verbose - A more verbose version of print_tainted()
//
    const char *print_tainted_verbose(void)
    {
    return _print_tainted(true);
    }
#[no_mangle]
pub unsafe extern "C" fn test_taint(flag: unsigned) -> c_int {
    return test_bit(flag, &tainted_mask);
    }
    EXPORT_SYMBOL(test_taint);
#[no_mangle]
pub unsafe extern "C" fn get_taint() -> c_ulong {
    return tainted_mask;
    }
//
// add_taint: add a taint flag if not already set.
// @flag: one of the TAINT_* constants.
// @lockdep_ok: whether lock debugging is still OK.
//
// If something bad has gone wrong, you'll want @lockdebug_ok = false, but for
// some notewortht-but-not-corrupting cases, it can be set to true.
//
#[no_mangle]
pub unsafe extern "C" fn add_taint(flag: unsigned, lockdep_ok: lockdep_ok) {
    if (lockdep_ok == LOCKDEP_NOW_UNRELIABLE && __debug_locks_off()) {
    pr_warn!("Disabling lock debugging due to kernel taint\n");
    }
    set_bit(flag, &tainted_mask);
    if (tainted_mask & panic_on_taint) {
    panic_on_taint = 0;
    panic("panic_on_taint set ...");
    }
    }
    EXPORT_SYMBOL(add_taint);
#[no_mangle]
unsafe extern "C" fn spin_msec(msecs: c_int) {
    let mut i = 0;
    while (i < msecs) {
    touch_nmi_watchdog();
    mdelay(1);
    }
    }
//
// It just happens that oops_enter() and oops_exit() are identically
// implemented...
//
#[no_mangle]
unsafe extern "C" fn do_oops_enter_exit() {
    let mut flags = 0;
    static int spin_counter;
    if (!pause_on_oops) {
    return;
    }
    spin_lock_irqsave(&pause_on_oops_lock, flags);
    if (pause_on_oops_flag == 0) {
// This CPU may now print the oops message
    pause_on_oops_flag = 1;
    } else {
// We need to stall this CPU
    if (!spin_counter) {
// This CPU gets to do the counting
    spin_counter = pause_on_oops;
    do {
    spin_unlock(&pause_on_oops_lock);
    spin_msec(MSEC_PER_SEC);
    spin_lock(&pause_on_oops_lock);
    } while (--spin_counter);
    pause_on_oops_flag = 0;
    } else {
// This CPU waits for a different one
    while (spin_counter) {
    spin_unlock(&pause_on_oops_lock);
    spin_msec(1);
    spin_lock(&pause_on_oops_lock);
    }
    }
    }
    spin_unlock_irqrestore(&pause_on_oops_lock, flags);
    }
//
// Return true if the calling CPU is allowed to print oops-related info.
// This is a bit racy..
//
#[no_mangle]
pub unsafe extern "C" fn oops_may_print() -> bool {
pub static mut pause_on_oops_flag: return = 0;
    }
//
// Called when the architecture enters its oops handler, before it prints
// anything.  If this is the first CPU to oops, and it's oopsing the first
// time then let it proceed.
//
// This is all enabled by the pause_on_oops kernel boot option.  We do all
// this to ensure that oopses don't scroll off the screen.  It has the
// side-effect of preventing later-oopsing CPUs from mucking up the display,
// too.
//
// It turns out that the CPU which is allowed to print ends up pausing for
// the right duration, whereas all the other CPUs pause for twice as long:
// once in oops_enter(), once in oops_exit().
//
#[no_mangle]
pub unsafe extern "C" fn oops_enter() {
    nbcon_cpu_emergency_enter();
    tracing_off();
// can't trust the integrity of the kernel anymore:
    debug_locks_off();
    do_oops_enter_exit();
    if (sysctl_oops_all_cpu_backtrace) {
    trigger_all_cpu_backtrace();
    }
    }
#[no_mangle]
unsafe extern "C" fn print_oops_end_marker() {
    pr_warn!("---[ end trace %016llx ]---\n", 0ULL);
    }
//
// Called when the architecture exits its oops handler, after printing
// everything.
//
#[no_mangle]
pub unsafe extern "C" fn oops_exit() {
    do_oops_enter_exit();
    print_oops_end_marker();
    nbcon_cpu_emergency_exit();
    kmsg_dump(KMSG_DUMP_OOPS);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct warn_args {
    pub fmt: *const c_char,
    pub args: va_list,
}

#[no_mangle]
pub unsafe extern "C" fn __warn(file: *mut c_char, line: c_int, caller: *mut c_void, taint: c_uint, regs: *mut pt_regs, args: *mut warn_args) {
    nbcon_cpu_emergency_enter();
    disable_trace_on_warning();
    if (file) {
    pr_warn!("WARNING: %s:%d at %pS, CPU#%d: %s/%d\n",
    file, line, caller,
    raw_smp_processor_id(), current.comm, current.pid);
    } else {
    pr_warn!("WARNING: at %pS, CPU#%d: %s/%d\n",
    caller,
    raw_smp_processor_id(), current.comm, current.pid);
    }

    if (args) {
    vprintk(args.fmt, args.args);
    }

    print_modules();
    if (regs) {
    show_regs(regs);
    }
    check_panic_on_warn("kernel");
    if (!regs) {
    dump_stack();
    }
    print_irqtrace_events(current);
    print_oops_end_marker();
    trace_error_report_end(ERROR_DETECTOR_WARN, (unsigned long)caller);
// Just a warning, don't kill lockdep.
    add_taint(taint, LOCKDEP_STILL_OK);
    nbcon_cpu_emergency_exit();
    }

#[no_mangle]
pub unsafe extern "C" fn warn_slowpath_fmt(file: *mut c_char, line: c_int, taint: c_uint, fmt: *mut c_char) {
pub static mut rcu: bool = false;
pub static mut args: usize = 0;
    if (kunit_is_suppressed_warning(true)) {
    warn_rcu_exit(rcu);
    return;
    }
    pr_warn!(CUT_HERE);
    if (!fmt) {
    __warn(file, line, __builtin_return_address(0), taint,
    core::ptr::null_mut(), core::ptr::null_mut());
    warn_rcu_exit(rcu);
    return;
    }
    args.fmt = fmt;
    va_start(args.args, fmt);
    __warn(file, line, __builtin_return_address(0), taint, core::ptr::null_mut(), &args);
    va_end(args.args);
    warn_rcu_exit(rcu);
    }
    EXPORT_SYMBOL(warn_slowpath_fmt);

#[no_mangle]
pub unsafe extern "C" fn __warn_printk(fmt: *const c_char, ...) {
pub static mut rcu: bool = false;
    let mut args;
    if (kunit_is_suppressed_warning(false)) {
    warn_rcu_exit(rcu);
    return;
    }
    pr_warn!(CUT_HERE);
    va_start(args, fmt);
    vprintk(fmt, args);
    va_end(args);
    warn_rcu_exit(rcu);
    }
    EXPORT_SYMBOL(__warn_printk);

// Support resetting WARN*_ONCE state
#[no_mangle]
unsafe extern "C" fn clear_warn_once_set(data: *mut c_void, val: u64) -> c_int {
    generic_bug_clear_once();
    memset(__start_once, 0, __end_once - __start_once);
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(clear_warn_once_fops, core::ptr::null_mut(), clear_warn_once_set,
    "%lld\n");
#[no_mangle]
unsafe extern "C" fn register_warn_debugfs() -> __init int {
// Don't care about failure
    debugfs_create_file_unsafe("clear_warn_once", 0200, core::ptr::null_mut(), core::ptr::null_mut(),
    &clear_warn_once_fops);
    return 0;
    }
    device_initcall!(register_warn_debugfs);

//
// Called when gcc's -fstack-protector feature is used, and
// gcc detects corruption of the on-stack canary value
//
#[no_mangle]
pub unsafe extern "C" fn __stack_chk_fail() -> __visible noinstr void {
    let mut flags = 0;
    instrumentation_begin();
    flags = user_access_save();
    panic("stack-protector: Kernel stack is corrupted in: %pB",
    __builtin_return_address(0));
    user_access_restore(flags);
    instrumentation_end();
    }
    EXPORT_SYMBOL(__stack_chk_fail);

    core_param!(panic, panic_timeout, int, 0644);
    core_param!(pause_on_oops, pause_on_oops, int, 0644);
    core_param!(panic_on_warn, panic_on_warn, int, 0644);
    core_param!(crash_kexec_post_notifiers, crash_kexec_post_notifiers, bool, 0644);
    core_param!(panic_console_replay, panic_console_replay, bool, 0644);
#[no_mangle]
unsafe extern "C" fn panic_print_set(val: *const c_char, kp: *const kernel_param) -> c_int {
    panic_print_deprecated();
    return  param_set_ulong(val, kp);
    }
#[no_mangle]
unsafe extern "C" fn panic_print_get(val: *mut c_char, kp: *const kernel_param) -> c_int {
    return  param_get_ulong(val, kp);
    }
pub static mut kernel_param_ops: usize = 0;
    __core_param_cb(panic_print, &panic_print_ops, &panic_print, 0644);
#[no_mangle]
unsafe extern "C" fn oops_setup(s: *mut c_char) -> c_int {
    if (!s) {
    return -EINVAL;
    }
    if (!strcmp(s, "panic")) {
    panic_on_oops = 1;
    }
    return 0;
    }
    early_param!("oops", oops_setup);
#[no_mangle]
unsafe extern "C" fn panic_on_taint_setup(s: *mut c_char) -> c_int {
pub static mut taint_str: *mut c_void = core::ptr::null_mut();
    if (!s) {
    return -EINVAL;
    }
    taint_str = strsep(&s, ",");
    if (kstrtoul(taint_str, 16, &panic_on_taint)) {
    return -EINVAL;
    }
// make sure panic_on_taint doesn't hold out-of-range TAINT flags
    panic_on_taint &= TAINT_FLAGS_MAX;
    if (!panic_on_taint) {
    return -EINVAL;
    }
    if (s && !strcmp(s, "nousertaint")) {
    panic_on_taint_nousertaint = true;
    }
    pr_info!("panic_on_taint: bitmask=0x%lx nousertaint_mode=%s\n",
    panic_on_taint, str_enabled_disabled(panic_on_taint_nousertaint));
    return 0;
    }
    early_param!("panic_on_taint", panic_on_taint_setup);
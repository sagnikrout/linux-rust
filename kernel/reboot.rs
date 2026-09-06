//! Automatically rewritten from C to Rust
//! Source: kernel/reboot.c
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
// linux/kernel/reboot.c
//
// Copyright (C) 2013  Linus Torvalds
//

//
// this indicates whether you can reboot with ctrl-alt-del: the default is yes
//
pub static mut C_A_D: int = 1;
pub static mut cad_pid: *mut c_void = core::ptr::null_mut();
    EXPORT_SYMBOL(cad_pid);

// Macro flag: #define DEFAULT_REBOOT_MODE

    enum reboot_mode reboot_mode DEFAULT_REBOOT_MODE;
    EXPORT_SYMBOL_GPL(reboot_mode);
pub static mut panic_reboot_mode: reboot_mode = 0;
pub static mut hw_protection_action: hw_protection_action = 0;
//
// This variable is used privately to keep track of whether or not
// reboot_type is still set to its default value (i.e., reboot= hasn't
// been set on the command line).  This is needed so that we can
// suppress DMI scanning for reboot quirks.  Without it, it's
// impossible to override a faulty reboot quirk without recompiling.
//
pub static mut reboot_default: c_int = 1;
    let mut reboot_cpu = 0;
pub static mut reboot_type: reboot_type = 0;
    let mut reboot_force = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sys_off_handler {
    pub nb: notifier_block,
    pub data): *mut *mut int (sys_off_cb)(sys_off_data,
    pub cb_data: *mut c_void,
    pub mode: sys_off_mode,
    pub blocking: bool,
    pub list: *mut c_void,
    pub dev: *mut device,
}

//
// This variable is used to indicate if a halt was initiated instead of a
// reboot when the reboot call was invoked with LINUX_REBOOT_CMD_POWER_OFF, but
// the system cannot be powered off. This allowes kernel_halt() to notify users
// of that.
//
    static bool poweroff_fallback_to_halt;
//
// Temporary stub that prevents linkage failure while we're in process
// of removing all uses of legacy pm_power_off() around the kernel.
//
// forward_decl: __weak;
//
// Notifier list for kernel code which wants to be called
// at shutdown. This is used to stop any idling DMA operations
// and the like.
//
// static BLOCKING_NOTIFIER_HEAD(reboot_notifier_list);
//
// emergency_restart - reboot the system
//
// Without shutting down any hardware or taking any locks
// reboot the system.  This is called when we know we are in
// trouble so this is our best effort to reboot.  This is
// safe to call in interrupt context.
//
#[no_mangle]
pub unsafe extern "C" fn emergency_restart() {
    kmsg_dump(KMSG_DUMP_EMERG);
    system_state = SYSTEM_RESTART;
    machine_emergency_restart();
    }
    EXPORT_SYMBOL_GPL(emergency_restart);
#[no_mangle]
pub unsafe extern "C" fn kernel_restart_prepare(cmd: *mut c_char) {
    blocking_notifier_call_chain(&reboot_notifier_list, SYS_RESTART, cmd);
    system_state = SYSTEM_RESTART;
    usermodehelper_disable();
    device_shutdown();
    }
//
// register_reboot_notifier - Register function to be called at reboot time
// @nb: Info about notifier function to be called
//
// Registers a function with the list of functions
// to be called at reboot time.
//
// Currently always returns zero, as blocking_notifier_chain_register()
// always returns zero.
//
#[no_mangle]
pub unsafe extern "C" fn register_reboot_notifier(nb: *mut notifier_block) -> c_int {
    return blocking_notifier_chain_register(&reboot_notifier_list, nb);
    }
    EXPORT_SYMBOL(register_reboot_notifier);
//
// unregister_reboot_notifier - Unregister previously registered reboot notifier
// @nb: Hook to be unregistered
//
// Unregisters a previously registered reboot
// notifier function.
//
// Returns zero on success, or %-ENOENT on failure.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_reboot_notifier(nb: *mut notifier_block) -> c_int {
    return blocking_notifier_chain_unregister(&reboot_notifier_list, nb);
    }
    EXPORT_SYMBOL(unregister_reboot_notifier);
#[no_mangle]
unsafe extern "C" fn devm_unregister_reboot_notifier(dev: *mut device, res: *mut c_void) {
    WARN_ON!(unregister_reboot_notifier(*res));
    }
#[no_mangle]
pub unsafe extern "C" fn devm_register_reboot_notifier(dev: *mut device, nb: *mut notifier_block) -> c_int {
pub static mut rcnb: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    rcnb = devres_alloc(devm_unregister_reboot_notifier,
    sizeof!(*rcnb), GFP_KERNEL);
    if (!rcnb) {
    return -ENOMEM;
    }
    ret = register_reboot_notifier(nb);
    if (!ret) {
// rcnb = nb;
    devres_add(dev, rcnb);
    } else {
    devres_free(rcnb);
    }
    return ret;
    }
    EXPORT_SYMBOL(devm_register_reboot_notifier);
//
// Notifier list for kernel code which wants to be called
// to restart the system.
//
// static ATOMIC_NOTIFIER_HEAD(restart_handler_list);
//
// register_restart_handler - Register function to be called to reset
// the system
// @nb: Info about handler function to be called
// @nb->priority:	Handler priority. Handlers should follow the
// following guidelines for setting priorities.
// 0:	Restart handler of last resort,
// with limited restart capabilities
// 128:	Default restart handler; use if no other
// restart handler is expected to be available,
// and/or if restart functionality is
// sufficient to restart the entire system
// 255:	Highest priority restart handler, will
// preempt all other restart handlers
//
// Registers a function with code to be called to restart the
// system.
//
// Registered functions will be called from machine_restart as last
// step of the restart sequence (if the architecture specific
// machine_restart function calls do_kernel_restart - see below
// for details).
// Registered functions are expected to restart the system immediately.
// If more than one function is registered, the restart handler priority
// selects which function will be called first.
//
// Restart handlers are expected to be registered from non-architecture
// code, typically from drivers. A typical use case would be a system
// where restart functionality is provided through a watchdog. Multiple
// restart handlers may exist; for example, one restart handler might
// restart the entire system, while another only restarts the CPU.
// In such cases, the restart handler which only restarts part of the
// hardware is expected to register with low priority to ensure that
// it only runs if no other means to restart the system is available.
//
// Currently always returns zero, as atomic_notifier_chain_register()
// always returns zero.
//
#[no_mangle]
pub unsafe extern "C" fn register_restart_handler(nb: *mut notifier_block) -> c_int {
    return atomic_notifier_chain_register(&restart_handler_list, nb);
    }
    EXPORT_SYMBOL(register_restart_handler);
//
// unregister_restart_handler - Unregister previously registered
// restart handler
// @nb: Hook to be unregistered
//
// Unregisters a previously registered restart handler function.
//
// Returns zero on success, or %-ENOENT on failure.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_restart_handler(nb: *mut notifier_block) -> c_int {
    return atomic_notifier_chain_unregister(&restart_handler_list, nb);
    }
    EXPORT_SYMBOL(unregister_restart_handler);
//
// do_kernel_restart - Execute kernel restart handler call chain
//
// @cmd: pointer to buffer containing command to execute for restart
// or %NULL
//
// Calls functions registered with register_restart_handler.
//
// Expected to be called from machine_restart as last step of the restart
// sequence.
//
// Restarts the system immediately if a restart handler function has been
// registered. Otherwise does nothing.
//
#[no_mangle]
pub unsafe extern "C" fn do_kernel_restart(cmd: *mut c_char) {
    atomic_notifier_call_chain(&restart_handler_list, reboot_mode, cmd);
    }
#[no_mangle]
pub unsafe extern "C" fn migrate_to_reboot_cpu() {
// The boot cpu is always logical cpu 0
pub static mut cpu: c_int = 0;
    cpu_hotplug_disable();
// Make certain the cpu I'm about to reboot on is online
    if (!cpu_online(cpu)) {
    cpu = cpumask_first(cpu_online_mask);
    }
// Prevent races with other tasks migrating this task
    current.flags |= PF_NO_SETAFFINITY;
// Make certain I only run on the appropriate processor
    set_cpus_allowed_ptr(current, cpumask_of(cpu));
    }
//
// Notifier list for kernel code which wants to be called
// to prepare system for restart.
//
// static BLOCKING_NOTIFIER_HEAD(restart_prep_handler_list);
#[no_mangle]
unsafe extern "C" fn do_kernel_restart_prepare() {
    blocking_notifier_call_chain(&restart_prep_handler_list, 0, core::ptr::null_mut());
    }
//
// kernel_restart - reboot the system
// @cmd: pointer to buffer containing command to execute for restart
// or %NULL
//
// Shutdown everything and perform a clean reboot.
// This is not safe to call in interrupt context.
//
#[no_mangle]
pub unsafe extern "C" fn kernel_restart(cmd: *mut c_char) {
    kernel_restart_prepare(cmd);
    do_kernel_restart_prepare();
    migrate_to_reboot_cpu();
    syscore_shutdown();
    if (!cmd) {
    pr_emerg("Restarting system\n");
    }
    else {
    pr_emerg("Restarting system with command '%s'\n", cmd);
    }
    kmsg_dump(KMSG_DUMP_SHUTDOWN);
    machine_restart(cmd);
    }
    EXPORT_SYMBOL_GPL(kernel_restart);
#[no_mangle]
unsafe extern "C" fn kernel_shutdown_prepare(state: system_states) {
    blocking_notifier_call_chain(&reboot_notifier_list,
    (state == SYSTEM_HALT) ? SYS_HALT : SYS_POWER_OFF, core::ptr::null_mut());
    system_state = state;
    usermodehelper_disable();
    device_shutdown();
    }
//
// kernel_halt - halt the system
//
// Shutdown everything and perform a clean system halt.
//
#[no_mangle]
pub unsafe extern "C" fn kernel_halt() {
    kernel_shutdown_prepare(SYSTEM_HALT);
    migrate_to_reboot_cpu();
    syscore_shutdown();
    if (poweroff_fallback_to_halt) {
    pr_emerg("Power off not available: System halted instead\n");
    }
    else {
    pr_emerg("System halted\n");
    }
    kmsg_dump(KMSG_DUMP_SHUTDOWN);
    machine_halt();
    }
    EXPORT_SYMBOL_GPL(kernel_halt);
//
// Notifier list for kernel code which wants to be called
// to prepare system for power off.
//
// static BLOCKING_NOTIFIER_HEAD(power_off_prep_handler_list);
//
// Notifier list for kernel code which wants to be called
// to power off system.
//
// static ATOMIC_NOTIFIER_HEAD(power_off_handler_list);
#[no_mangle]
pub unsafe extern "C" fn sys_off_notify(nb: *mut notifier_block, mode: c_ulong, cmd: *mut c_void) -> c_int {
pub static mut handler: *mut c_void = core::ptr::null_mut();
pub static mut data: sys_off_data = 0;
    handler = container_of!(nb, sys_off_handler, nb);
    data.cb_data = handler.cb_data;
    data.mode = mode;
    data.cmd = cmd;
    data.dev = handler.dev;
    return handler.sys_off_cb(&data);
    }
pub static mut platform_sys_off_handler: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn alloc_sys_off_handler(priority: c_int) -> *mut c_void {
pub static mut handler: *mut c_void = core::ptr::null_mut();
    let mut flags;
//
// Platforms like m68k can't allocate sys_off handler dynamically
// at the early boot time because memory allocator isn't available yet.
//
    if (priority == SYS_OFF_PRIO_PLATFORM) {
    handler = &platform_sys_off_handler;
    if (handler.cb_data) {
    return ERR_PTR(-EBUSY);
    }
    } else {
    if (system_state > SYSTEM_RUNNING) {
    flags = GFP_ATOMIC;
    }
    else {
    flags = GFP_KERNEL;
    }
    handler = kzalloc_obj(*handler, flags);
    if (!handler) {
    return ERR_PTR(-ENOMEM);
    }
    }
    return handler;
    }
#[no_mangle]
unsafe extern "C" fn free_sys_off_handler(handler: *mut sys_off_handler) {
    if (handler == &platform_sys_off_handler) {
    memset(handler, 0, sizeof!(*handler));
    }
    else {
    kfree(handler);
    }
    }
//
// register_sys_off_handler - Register sys-off handler
// @mode: Sys-off mode
// @priority: Handler priority
// @callback: Callback function
// @cb_data: Callback argument
//
// Registers system power-off or restart handler that will be invoked
// at the step corresponding to the given sys-off mode. Handler's callback
// should return NOTIFY_DONE to permit execution of the next handler in
// the call chain or NOTIFY_STOP to break the chain (in error case for
// example).
//
// Multiple handlers can be registered at the default priority level.
//
// Only one handler can be registered at the non-default priority level,
// otherwise ERR_PTR(-EBUSY) is returned.
//
// Returns a new instance of struct sys_off_handler on success, or
// an ERR_PTR()-encoded error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn register_sys_off_handler(mode: sys_off_mode, priority: c_int, cb_data: *mut c_void) -> *mut c_void {
pub static mut handler: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    handler = alloc_sys_off_handler(priority);
    if (IS_ERR(handler)) {
    return handler;
    }
    match (mode) {
    SYS_OFF_MODE_POWER_OFF_PREPARE => {
    handler.list = &power_off_prep_handler_list;
    handler.blocking = true;
    // break;
    }
    SYS_OFF_MODE_POWER_OFF => {
    handler.list = &power_off_handler_list;
    // break;
    }
    SYS_OFF_MODE_RESTART_PREPARE => {
    handler.list = &restart_prep_handler_list;
    handler.blocking = true;
    // break;
    }
    SYS_OFF_MODE_RESTART => {
    handler.list = &restart_handler_list;
    // break;
    }
    _ => {
    free_sys_off_handler(handler);
    return ERR_PTR(-EINVAL);
    }
    }
    handler.nb.notifier_call = sys_off_notify;
    handler.nb.priority = priority;
    handler.sys_off_cb = callback;
    handler.cb_data = cb_data;
    handler.mode = mode;
    if (handler.blocking) {
    if (priority == SYS_OFF_PRIO_DEFAULT) {
    err = blocking_notifier_chain_register(handler.list,
    &handler.nb);
    }
    else {
    err = blocking_notifier_chain_register_unique_prio(handler.list,
    &handler.nb);
    }
    } else {
    if (priority == SYS_OFF_PRIO_DEFAULT) {
    err = atomic_notifier_chain_register(handler.list,
    &handler.nb);
    }
    else {
    err = atomic_notifier_chain_register_unique_prio(handler.list,
    &handler.nb);
    }
    }
    if (err) {
    free_sys_off_handler(handler);
    return ERR_PTR(err);
    }
    return handler;
    }
    EXPORT_SYMBOL_GPL(register_sys_off_handler);
//
// unregister_sys_off_handler - Unregister sys-off handler
// @handler: Sys-off handler
//
// Unregisters given sys-off handler.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_sys_off_handler(handler: *mut sys_off_handler) {
    let mut err = 0;
    if (IS_ERR_OR_NULL(handler)) {
    return;
    }
    if (handler.blocking) {
    err = blocking_notifier_chain_unregister(handler.list,
    &handler.nb);
    }
    else {
    err = atomic_notifier_chain_unregister(handler.list,
    &handler.nb);
    }
// sanity check, shall never happen
    WARN_ON!(err);
    free_sys_off_handler(handler);
    }
    EXPORT_SYMBOL_GPL(unregister_sys_off_handler);
#[no_mangle]
unsafe extern "C" fn devm_unregister_sys_off_handler(data: *mut c_void) {
    let mut handler = data;
    unregister_sys_off_handler(handler);
    }
//
// devm_register_sys_off_handler - Register sys-off handler
// @dev: Device that registers handler
// @mode: Sys-off mode
// @priority: Handler priority
// @callback: Callback function
// @cb_data: Callback argument
//
// Registers resource-managed sys-off handler.
//
// Returns zero on success, or error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn devm_register_sys_off_handler(dev: *mut device, mode: sys_off_mode, priority: c_int, cb_data: *mut c_void) -> c_int {
pub static mut handler: *mut c_void = core::ptr::null_mut();
    handler = register_sys_off_handler(mode, priority, callback, cb_data);
    if (IS_ERR(handler)) {
    return PTR_ERR(handler);
    }
    handler.dev = dev;
    return devm_add_action_or_reset(dev, devm_unregister_sys_off_handler,
    handler);
    }
    EXPORT_SYMBOL_GPL(devm_register_sys_off_handler);
//
// devm_register_power_off_handler - Register power-off handler
// @dev: Device that registers callback
// @callback: Callback function
// @cb_data: Callback's argument
//
// Registers resource-managed sys-off handler with a default priority
// and using power-off mode.
//
// Returns zero on success, or error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn devm_register_power_off_handler(dev: *mut device, cb_data: *mut c_void) -> c_int {
    return devm_register_sys_off_handler(dev,
    SYS_OFF_MODE_POWER_OFF,
    SYS_OFF_PRIO_DEFAULT,
    callback, cb_data);
    }
    EXPORT_SYMBOL_GPL(devm_register_power_off_handler);
//
// devm_register_restart_handler - Register restart handler
// @dev: Device that registers callback
// @callback: Callback function
// @cb_data: Callback's argument
//
// Registers resource-managed sys-off handler with a default priority
// and using restart mode.
//
// Returns zero on success, or error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn devm_register_restart_handler(dev: *mut device, cb_data: *mut c_void) -> c_int {
    return devm_register_sys_off_handler(dev,
    SYS_OFF_MODE_RESTART,
    SYS_OFF_PRIO_DEFAULT,
    callback, cb_data);
    }
    EXPORT_SYMBOL_GPL(devm_register_restart_handler);
pub static mut platform_power_off_handler: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn platform_power_off_notify(data: *mut sys_off_data) -> c_int {
    void (*platform_power_power_off_cb)(void) = data.cb_data;
    platform_power_power_off_cb();
    return NOTIFY_DONE;
    }
//
// register_platform_power_off - Register platform-level power-off callback
// @power_off: Power-off callback
//
// Registers power-off callback that will be called as last step
// of the power-off sequence. This callback is expected to be invoked
// for the last resort. Only one platform power-off callback is allowed
// to be registered at a time.
//
// Returns zero on success, or error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn register_platform_power_off((*power_off)(void): *mut c_void) -> c_int {
#[no_mangle]
#[no_mangle]
// duplicate fn: register_platform_power_off
pub unsafe extern "C" fn register_platform_power_off_dup() -> c_int {
pub static mut handler: *mut c_void = core::ptr::null_mut();
    handler = register_sys_off_handler(SYS_OFF_MODE_POWER_OFF,
    SYS_OFF_PRIO_PLATFORM,
    platform_power_off_notify,
    power_off);
    if (IS_ERR(handler)) {
    return PTR_ERR(handler);
    }
    platform_power_off_handler = handler;
    return 0;
    }
    EXPORT_SYMBOL_GPL(register_platform_power_off);
//
// unregister_platform_power_off - Unregister platform-level power-off callback
// @power_off: Power-off callback
//
// Unregisters previously registered platform power-off callback.
//
#[no_mangle]
pub unsafe extern "C" fn unregister_platform_power_off((*power_off)(void): *mut c_void) {
#[no_mangle]
#[no_mangle]
// duplicate fn: unregister_platform_power_off
pub unsafe extern "C" fn unregister_platform_power_off_dup() {
    if (platform_power_off_handler &&
    platform_power_off_handler.cb_data == power_off) {
    unregister_sys_off_handler(platform_power_off_handler);
    platform_power_off_handler = core::ptr::null_mut();
    }
    }
    EXPORT_SYMBOL_GPL(unregister_platform_power_off);
#[no_mangle]
unsafe extern "C" fn legacy_pm_power_off(data: *mut sys_off_data) -> c_int {
    if (pm_power_off) {
    pm_power_off();
    }
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn do_kernel_power_off_prepare() {
    blocking_notifier_call_chain(&power_off_prep_handler_list, 0, core::ptr::null_mut());
    }
//
// do_kernel_power_off - Execute kernel power-off handler call chain
//
// Expected to be called as last step of the power-off sequence.
//
// Powers off the system immediately if a power-off handler function has
// been registered. Otherwise does nothing.
//
#[no_mangle]
pub unsafe extern "C" fn do_kernel_power_off() {
    let mut sys_off = core::ptr::null_mut();
//
// Register sys-off handlers for legacy PM callback. This allows
// legacy PM callbacks temporary co-exist with the new sys-off API.
//
// TODO: Remove legacy handlers once all legacy PM users will be
// switched to the sys-off based APIs.
//
    if (pm_power_off) {
    sys_off = register_sys_off_handler(SYS_OFF_MODE_POWER_OFF,
    SYS_OFF_PRIO_DEFAULT,
    legacy_pm_power_off, core::ptr::null_mut());
    }
    atomic_notifier_call_chain(&power_off_handler_list, 0, core::ptr::null_mut());
    unregister_sys_off_handler(sys_off);
    }
//
// kernel_can_power_off - check whether system can be powered off
//
// Returns true if power-off handler is registered and system can be
// powered off, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn kernel_can_power_off() -> bool {
    return !atomic_notifier_call_chain_is_empty(&power_off_handler_list) ||
    pm_power_off;
    }
    EXPORT_SYMBOL_GPL(kernel_can_power_off);
//
// kernel_power_off - power_off the system
//
// Shutdown everything and perform a clean system power_off.
//
#[no_mangle]
pub unsafe extern "C" fn kernel_power_off() {
    kernel_shutdown_prepare(SYSTEM_POWER_OFF);
    do_kernel_power_off_prepare();
    migrate_to_reboot_cpu();
    syscore_shutdown();
    pr_emerg("Power down\n");
    pr_flush(1000, true);
    kmsg_dump(KMSG_DUMP_SHUTDOWN);
    machine_power_off();
    }
    EXPORT_SYMBOL_GPL(kernel_power_off);
pub static mut system_transition_mutex: usize = 0;
//
// Reboot system call: for obvious reasons only root may call it,
// and even root needs to set up some magic numbers in the registers
// so that some mistake won't make this reboot the whole machine.
// You can also set the meaning of the ctrl-alt-del-key here.
//
// reboot doesn't sync: do that yourself before calling this.
//
#[no_mangle]
pub unsafe extern "C" fn sys_reboot(magic1: usize, magic2: usize, cmd: usize, arg: usize) -> c_long {
    let mut pid_ns = task_active_pid_ns(current);
    char buffer[256];
pub static mut ret: c_int = 0;
// We only trust the superuser with rebooting the system.
    if (!ns_capable(pid_ns.user_ns, CAP_SYS_BOOT)) {
    return -EPERM;
    }
// For safety, we require "magic" arguments.
    if (magic1 != LINUX_REBOOT_MAGIC1 ||
    (magic2 != LINUX_REBOOT_MAGIC2 &&
    magic2 != LINUX_REBOOT_MAGIC2A &&
    magic2 != LINUX_REBOOT_MAGIC2B &&
    magic2 != LINUX_REBOOT_MAGIC2C)) {
    return -EINVAL;
    }
//
// If pid namespaces are enabled and the current task is in a child
// pid_namespace, the command is handled by reboot_pid_ns() which will
// call do_exit().
//
    ret = reboot_pid_ns(pid_ns, cmd);
    if (ret) {
    return ret;
    }
// Instead of trying to make the power_off code look like
// halt when pm_power_off is not set do it the easy way.
//
    if ((cmd == LINUX_REBOOT_CMD_POWER_OFF) && !kernel_can_power_off()) {
    poweroff_fallback_to_halt = true;
    cmd = LINUX_REBOOT_CMD_HALT;
    }
    mutex_lock(&system_transition_mutex);
    match (cmd) {
    LINUX_REBOOT_CMD_RESTART => {
    kernel_restart(core::ptr::null_mut());
    // break;
    }
    LINUX_REBOOT_CMD_CAD_ON => {
    C_A_D = 1;
    // break;
    }
    LINUX_REBOOT_CMD_CAD_OFF => {
    C_A_D = 0;
    // break;
    }
    LINUX_REBOOT_CMD_HALT => {
    kernel_halt();
    do_exit(0);
    }
    LINUX_REBOOT_CMD_POWER_OFF => {
    kernel_power_off();
    do_exit(0);
    // break;
    }
    LINUX_REBOOT_CMD_RESTART2 => {
    ret = strncpy_from_user(&buffer[0], arg, sizeof!(buffer) - 1);
    if (ret < 0) {
    ret = -EFAULT;
    // break;
    }
    buffer[sizeof!(buffer) - 1] = '\0';
    kernel_restart(buffer);
    // break;

    }
    LINUX_REBOOT_CMD_KEXEC => {
    ret = kernel_kexec();
    // break;

    }
    LINUX_REBOOT_CMD_SW_SUSPEND => {
    ret = hibernate();
    // break;

    }
    _ => {
    ret = -EINVAL;
    // break;
    }
    }
    mutex_unlock(&system_transition_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn deferred_cad(dummy: *mut work_struct) {
    kernel_restart(core::ptr::null_mut());
    }
//
// This function gets called by ctrl-alt-del - ie the keyboard interrupt.
// As it's called within an interrupt, it may NOT sync: the only choice
// is whether to reboot at once, or just ignore the ctrl-alt-del.
//
#[no_mangle]
pub unsafe extern "C" fn ctrl_alt_del() {
// static DECLARE_WORK(cad_work, deferred_cad);
    if (C_A_D) {
    schedule_work(&cad_work);
    }
    else {
    kill_cad_pid(SIGINT, 1);
    }
    }
pub const POWEROFF_CMD_PATH_LEN: c_int = 256;
    static char poweroff_cmd[POWEROFF_CMD_PATH_LEN] = "/sbin/poweroff";
    static const char reboot_cmd[] = "/sbin/reboot";
#[no_mangle]
unsafe extern "C" fn run_cmd(cmd: *const c_char) -> c_int {
pub static mut argv: *mut c_void = core::ptr::null_mut();
    static char *envp[] = {
    "HOME=/",
    "PATH=/sbin:/bin:/usr/sbin:/usr/bin",
    core::ptr::null_mut()
    };
    let mut ret = 0;
    argv = argv_split(GFP_KERNEL, cmd, core::ptr::null_mut());
    if (argv) {
    ret = call_usermodehelper(argv[0], argv, envp, UMH_WAIT_EXEC);
    argv_free(argv);
    } else {
    ret = -ENOMEM;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __orderly_reboot() -> c_int {
    let mut ret = 0;
    ret = run_cmd(reboot_cmd);
    if (ret) {
    pr_warn!("Failed to start orderly reboot: forcing the issue\n");
    emergency_sync();
    kernel_restart(core::ptr::null_mut());
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __orderly_poweroff(force: bool) -> c_int {
    let mut ret = 0;
    ret = run_cmd(poweroff_cmd);
    if (ret && force) {
    pr_warn!("Failed to start orderly shutdown: forcing the issue\n");
//
// I guess this should try to kick off some daemon to sync and
// poweroff asap.  Or not even bother syncing if we're doing an
// emergency shutdown?
//
    emergency_sync();
    kernel_power_off();
    }
    return ret;
    }
    static bool poweroff_force;
#[no_mangle]
unsafe extern "C" fn poweroff_work_func(work: *mut work_struct) {
    __orderly_poweroff(poweroff_force);
    }
// static DECLARE_WORK(poweroff_work, poweroff_work_func);
//
// orderly_poweroff - Trigger an orderly system poweroff
// @force: force poweroff if command execution fails
//
// This may be called from any context to trigger a system shutdown.
// If the orderly shutdown fails, it will force an immediate shutdown.
//
#[no_mangle]
pub unsafe extern "C" fn orderly_poweroff(force: bool) {
    if (force) /* do not override the pending "true" */ {
    poweroff_force = true;
    }
    schedule_work(&poweroff_work);
    }
    EXPORT_SYMBOL_GPL(orderly_poweroff);
#[no_mangle]
unsafe extern "C" fn reboot_work_func(work: *mut work_struct) {
    __orderly_reboot();
    }
// static DECLARE_WORK(reboot_work, reboot_work_func);
//
// orderly_reboot - Trigger an orderly system reboot
//
// This may be called from any context to trigger a system reboot.
// If the orderly reboot fails, it will force an immediate reboot.
//
#[no_mangle]
pub unsafe extern "C" fn orderly_reboot() {
    schedule_work(&reboot_work);
    }
    EXPORT_SYMBOL_GPL(orderly_reboot);
    static const char *hw_protection_action_str(enum hw_protection_action action)
    {
    match (action) {
    HWPROT_ACT_SHUTDOWN => {
    return "shutdown";
    }
    HWPROT_ACT_REBOOT => {
    return "reboot";
    }
    _ => {
    return "undefined";
    }
    }
    }
    static enum hw_protection_action hw_failure_emergency_action;
//
// hw_failure_emergency_action_func - emergency action work after a known delay
// @work: work_struct associated with the emergency action function
//
// This function is called in very critical situations to force
// a kernel poweroff or reboot after a configurable timeout value.
//
#[no_mangle]
unsafe extern "C" fn hw_failure_emergency_action_func(work: *mut work_struct) {
    let mut action_str = hw_protection_action_str(hw_failure_emergency_action);
    pr_emerg("Hardware protection timed-out. Trying forced %s\n",
    action_str);
//
// We have reached here after the emergency action waiting period has
// expired. This means orderly_poweroff/reboot has not been able to
// shut off the system for some reason.
//
// Try to shut off the system immediately if possible
//
    if (hw_failure_emergency_action == HWPROT_ACT_REBOOT) {
    kernel_restart(core::ptr::null_mut());
    }
    else {
    kernel_power_off();
    }
//
// Worst of the worst case trigger emergency restart
//
    pr_emerg("Hardware protection %s failed. Trying emergency restart\n",
    action_str);
    emergency_restart();
    }
pub static mut hw_failure_emergency_action_work: usize = 0;
//
// hw_failure_emergency_schedule - Schedule an emergency system shutdown or reboot
//
// @action:		The hardware protection action to be taken
// @action_delay_ms:	Time in milliseconds to elapse before triggering action
//
// This may be called from any critical situation to trigger a system shutdown
// or reboot after a given period of time.
// If time is negative this is not scheduled.
//
#[no_mangle]
pub unsafe extern "C" fn hw_failure_emergency_schedule(action: hw_protection_action, action_delay_ms: c_int) {
    if (action_delay_ms <= 0) {
    return;
    }
    hw_failure_emergency_action = action;
    schedule_delayed_work(&hw_failure_emergency_action_work,
    msecs_to_jiffies(action_delay_ms));
    }
//
// __hw_protection_trigger - Trigger an emergency system shutdown or reboot
//
// @reason:		Reason of emergency shutdown or reboot to be printed.
// @ms_until_forced:	Time to wait for orderly shutdown or reboot before
// triggering it. Negative value disables the forced
// shutdown or reboot.
// @action:		The hardware protection action to be taken.
//
// Initiate an emergency system shutdown or reboot in order to protect
// hardware from further damage. Usage examples include a thermal protection.
// NOTE: The request is ignored if protection shutdown or reboot is already
// pending even if the previous request has given a large timeout for forced
// shutdown/reboot.
//
#[no_mangle]
pub unsafe extern "C" fn __hw_protection_trigger(reason: *mut c_char, ms_until_forced: c_int, action: hw_protection_action) {
pub static mut allow_proceed: atomic_t = 0;
    if (action == HWPROT_ACT_DEFAULT) {
    action = hw_protection_action;
    }
    pr_emerg("HARDWARE PROTECTION %s (%s)\n",
    hw_protection_action_str(action), reason);
// Shutdown should be initiated only once.
    if (!atomic_dec_and_test(&allow_proceed)) {
    return;
    }
//
// Queue a backup emergency shutdown in the event of
// orderly_poweroff failure
//
    hw_failure_emergency_schedule(action, ms_until_forced);
    if (action == HWPROT_ACT_REBOOT) {
    orderly_reboot();
    }
    else {
    orderly_poweroff(true);
    }
    }
    EXPORT_SYMBOL_GPL(__hw_protection_trigger);
#[no_mangle]
pub unsafe extern "C" fn hw_protection_action_parse(str: *mut c_char, action: *mut hw_protection_action) -> bool {
    if (sysfs_streq(str, "shutdown")) {
// action = HWPROT_ACT_SHUTDOWN;
    }

    else if (sysfs_streq(str, "reboot")) {
// action = HWPROT_ACT_REBOOT;
    }
    else {
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn hw_protection_setup(str: *mut c_char) -> c_int {
    hw_protection_action_parse(str, &hw_protection_action);
    return 1;
    }
    __setup!("hw_protection=", hw_protection_setup);

#[no_mangle]
pub unsafe extern "C" fn hw_protection_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%s\n",
    hw_protection_action_str(hw_protection_action));
    }
#[no_mangle]
pub unsafe extern "C" fn hw_protection_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    if (!capable(CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    if (!hw_protection_action_parse(buf, &hw_protection_action)) {
    return -EINVAL;
    }
    return count;
    }
pub static mut hw_protection_attr: kobj_attribute = 0;

#[no_mangle]
unsafe extern "C" fn reboot_setup(str: *mut c_char) -> c_int {
    for (;;) {
    enum reboot_mode *mode;
//
// Having anything passed on the command line via
// reboot= will cause us to disable DMI checking
// below.
//
    reboot_default = 0;
    if (!strncmp(str, "panic_", 6)) {
    mode = &panic_reboot_mode;
    str += 6;
    } else {
    mode = &reboot_mode;
    }
    match (*str) {
    'w' => {
// mode = REBOOT_WARM;
    // break;
    }
    'c' => {
// mode = REBOOT_COLD;
    // break;
    }
    'h' => {
// mode = REBOOT_HARD;
    // break;
    }
    's' => {
//
// reboot_cpu is s[mp]#### with #### being the processor
// to be used for rebooting. Skip 's' or 'smp' prefix.
//
    str += str[1] == 'm' && str[2] == 'p' ? 3 : 1;
    if (isdigit(str[0])) {
pub static mut cpu: c_int = 0;
    if (cpu >= num_possible_cpus()) {
    pr_err!("Ignoring the CPU number in reboot= option. "
    "CPU %d exceeds possible cpu number %d\n",
    cpu, num_possible_cpus());
    // break;
    }
    reboot_cpu = cpu;
    } else {
// mode = REBOOT_SOFT;
    }
    // break;
    }
    'g' => {
// mode = REBOOT_GPIO;
    // break;
    }
    'b' => {
    }
    'a' => {
    }
    'k' => {
    }
    't' => {
    }
    'e' => {
    }
    'p' => {
    reboot_type = *str;
    // break;
    }
    'f' => {
    reboot_force = 1;
    // break;
    }
    }
    str = strchr(str, ',');
    if (str) {
    str += 1;
    }
    else {
    break;
    }
    }
    return 1;
    }
    __setup!("reboot=", reboot_setup);

#[no_mangle]
unsafe extern "C" fn mode_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
pub static mut val: *mut c_void = core::ptr::null_mut();
    match (reboot_mode) {
    REBOOT_COLD => {
    val = REBOOT_COLD_STR;
    // break;
    }
    REBOOT_WARM => {
    val = REBOOT_WARM_STR;
    // break;
    }
    REBOOT_HARD => {
    val = REBOOT_HARD_STR;
    // break;
    }
    REBOOT_SOFT => {
    val = REBOOT_SOFT_STR;
    // break;
    }
    REBOOT_GPIO => {
    val = REBOOT_GPIO_STR;
    // break;
    }
    _ => {
    val = REBOOT_UNDEFINED_STR;
    }
    }
    return sysfs_emit(buf, "%s\n", val);
    }
#[no_mangle]
pub unsafe extern "C" fn mode_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    if (!capable(CAP_SYS_BOOT)) {
    return -EPERM;
    }
    if (!strncmp(buf, REBOOT_COLD_STR, strlen(REBOOT_COLD_STR))) {
    reboot_mode = REBOOT_COLD;
    }

    else if (!strncmp(buf, REBOOT_WARM_STR, strlen(REBOOT_WARM_STR))) {
    reboot_mode = REBOOT_WARM;
    }

    else if (!strncmp(buf, REBOOT_HARD_STR, strlen(REBOOT_HARD_STR))) {
    reboot_mode = REBOOT_HARD;
    }

    else if (!strncmp(buf, REBOOT_SOFT_STR, strlen(REBOOT_SOFT_STR))) {
    reboot_mode = REBOOT_SOFT;
    }

    else if (!strncmp(buf, REBOOT_GPIO_STR, strlen(REBOOT_GPIO_STR))) {
    reboot_mode = REBOOT_GPIO;
    }
    else {
    return -EINVAL;
    }
    reboot_default = 0;
    return count;
    }
pub static mut reboot_mode_attr: kobj_attribute = 0;

#[no_mangle]
unsafe extern "C" fn force_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%d\n", reboot_force);
    }
#[no_mangle]
pub unsafe extern "C" fn force_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut res = 0;
    if (!capable(CAP_SYS_BOOT)) {
    return -EPERM;
    }
    if (kstrtobool(buf, &res)) {
    return -EINVAL;
    }
    reboot_default = 0;
    reboot_force = res;
    return count;
    }
pub static mut reboot_force_attr: kobj_attribute = 0;
#[no_mangle]
unsafe extern "C" fn type_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
pub static mut val: *mut c_void = core::ptr::null_mut();
    match (reboot_type) {
    BOOT_TRIPLE => {
    val = BOOT_TRIPLE_STR;
    // break;
    }
    BOOT_KBD => {
    val = BOOT_KBD_STR;
    // break;
    }
    BOOT_BIOS => {
    val = BOOT_BIOS_STR;
    // break;
    }
    BOOT_ACPI => {
    val = BOOT_ACPI_STR;
    // break;
    }
    BOOT_EFI => {
    val = BOOT_EFI_STR;
    // break;
    }
    BOOT_CF9_FORCE => {
    val = BOOT_PCI_STR;
    // break;
    }
    _ => {
    val = REBOOT_UNDEFINED_STR;
    }
    }
    return sysfs_emit(buf, "%s\n", val);
    }
#[no_mangle]
pub unsafe extern "C" fn type_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    if (!capable(CAP_SYS_BOOT)) {
    return -EPERM;
    }
    if (!strncmp(buf, BOOT_TRIPLE_STR, strlen(BOOT_TRIPLE_STR))) {
    reboot_type = BOOT_TRIPLE;
    }

    else if (!strncmp(buf, BOOT_KBD_STR, strlen(BOOT_KBD_STR))) {
    reboot_type = BOOT_KBD;
    }

    else if (!strncmp(buf, BOOT_BIOS_STR, strlen(BOOT_BIOS_STR))) {
    reboot_type = BOOT_BIOS;
    }

    else if (!strncmp(buf, BOOT_ACPI_STR, strlen(BOOT_ACPI_STR))) {
    reboot_type = BOOT_ACPI;
    }

    else if (!strncmp(buf, BOOT_EFI_STR, strlen(BOOT_EFI_STR))) {
    reboot_type = BOOT_EFI;
    }

    else if (!strncmp(buf, BOOT_PCI_STR, strlen(BOOT_PCI_STR))) {
    reboot_type = BOOT_CF9_FORCE;
    }
    else {
    return -EINVAL;
    }
    reboot_default = 0;
    return count;
    }
pub static mut reboot_type_attr: kobj_attribute = 0;

#[no_mangle]
unsafe extern "C" fn cpu_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    return sysfs_emit(buf, "%d\n", reboot_cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut cpunum = 0;
    let mut rc = 0;
    if (!capable(CAP_SYS_BOOT)) {
    return -EPERM;
    }
    rc = kstrtouint(buf, 0, &cpunum);
    if (rc) {
    return rc;
    }
    if (cpunum >= num_possible_cpus()) {
    return -ERANGE;
    }
    reboot_default = 0;
    reboot_cpu = cpunum;
    return count;
    }
pub static mut reboot_cpu_attr: kobj_attribute = 0;

    static struct attribute *reboot_attrs[] = {
    &hw_protection_attr.attr,
    &reboot_mode_attr.attr,

    &reboot_force_attr.attr,
    &reboot_type_attr.attr,

    &reboot_cpu_attr.attr,

    core::ptr::null_mut(),
    };

#[no_mangle]
pub unsafe extern "C" fn proc_do_cad_pid(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
pub static mut tmp_table: ctl_table = 0;
pub static mut new_pid: *mut c_void = core::ptr::null_mut();
    let mut tmp_pid = 0;
    let mut r = 0;
    tmp_pid = pid_vnr(cad_pid);
    tmp_table.data = &tmp_pid;
    r = proc_dointvec(&tmp_table, write, buffer, lenp, ppos);
    if (r || !write) {
    return r;
    }
    new_pid = find_get_pid(tmp_pid);
    if (!new_pid) {
    return -ESRCH;
    }
    put_pid(xchg(&cad_pid, new_pid));
    return 0;
    }
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn kernel_reboot_sysctls_init() -> c_int {
    register_sysctl_init("kernel", kern_reboot_table);
    }

pub static mut attribute_group: usize = 0;
#[no_mangle]
unsafe extern "C" fn reboot_ksysfs_init() -> c_int {
pub static mut reboot_kobj: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    reboot_kobj = kobject_create_and_add("reboot", kernel_kobj);
    if (!reboot_kobj) {
    return -ENOMEM;
    }
    ret = sysfs_create_group(reboot_kobj, &reboot_attr_group);
    if (ret) {
    kobject_put(reboot_kobj);
    return ret;
    }
    kernel_reboot_sysctls_init();
    return 0;
    }
    late_initcall!(reboot_ksysfs_init);

}
}
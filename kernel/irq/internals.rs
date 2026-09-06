//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/irq/internals.h
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
// IRQ subsystem internal functions and variables:
//
// Do not ever include this file from anything else than
// kernel/irq/. Do not even think about using any information outside
// of this file for your non core code.
//

//
// Bits used by threaded handlers:
// IRQTF_RUNTHREAD - signals that the interrupt handler thread should run
// IRQTF_WARNED    - warning "IRQ_WAKE_THREAD w/o thread_fn" has been printed
// IRQTF_AFFINITY  - irq thread is requested to adjust affinity
// IRQTF_FORCED_THREAD  - irq action is force threaded
// IRQTF_READY     - signals that irq thread is ready
//
// Bit masks for desc->core_internal_state__do_not_mess_with_it
//
// IRQS_AUTODETECT		- autodetection in progress
// IRQS_SPURIOUS_DISABLED	- was disabled due to spurious interrupt
// detection
// IRQS_POLL_INPROGRESS		- polling in progress
// IRQS_ONESHOT			- irq is not unmasked in primary handler
// IRQS_REPLAY			- irq has been resent and will not be resent
// again until the handler has run and cleared
// this flag.
// IRQS_WAITING			- irq is waiting
// IRQS_PENDING			- irq needs to be resent and should be resent
// at the next available opportunity.
// IRQS_SUSPENDED		- irq is suspended
// IRQS_NMI			- irq line is used to deliver NMIs
// IRQS_SYSFS			- descriptor has been added to sysfs
//

extern "C" {
    pub fn __irq_set_trigger(desc: *mut irq_desc, flags: c_ulong) -> c_int;
}
extern "C" {
    pub fn __disable_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn __enable_irq(desc: *mut irq_desc);
}

extern "C" {
    pub fn irq_activate(desc: *mut irq_desc) -> c_int;
}
extern "C" {
    pub fn irq_activate_and_startup(desc: *mut irq_desc, resend: bool) -> c_int;
}
extern "C" {
    pub fn irq_startup(desc: *mut irq_desc, resend: bool, force: bool) -> c_int;
}
extern "C" {
    pub fn irq_startup_managed(desc: *mut irq_desc);
}
extern "C" {
    pub fn irq_shutdown(desc: *mut irq_desc);
}
extern "C" {
    pub fn irq_shutdown_and_deactivate(desc: *mut irq_desc);
}
extern "C" {
    pub fn irq_disable(desc: *mut irq_desc);
}
extern "C" {
    pub fn irq_percpu_enable(desc: *mut irq_desc, cpu: c_uint);
}
extern "C" {
    pub fn irq_percpu_disable(desc: *mut irq_desc, cpu: c_uint);
}
extern "C" {
    pub fn mask_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn unmask_irq(desc: *mut irq_desc);
}
extern "C" {
    pub fn unmask_threaded_irq(desc: *mut irq_desc);
}

extern "C" {
    pub fn irq_desc_free_rcu(desc: *mut irq_desc);
}
extern "C" {
    pub fn rcuref_get(_arg: &desc->refcnt) -> return;
}

extern "C" {
    pub fn irq_mark_irq(irq: c_uint);
}

extern "C" {
    pub fn __handle_irq_event_percpu(desc: *mut irq_desc) -> irqreturn_t;
}
extern "C" {
    pub fn handle_irq_event_percpu(desc: *mut irq_desc) -> irqreturn_t;
}
extern "C" {
    pub fn handle_irq_event(desc: *mut irq_desc) -> irqreturn_t;
}
// Resending of interrupts :
extern "C" {
    pub fn check_irq_resend(desc: *mut irq_desc, inject: bool) -> c_int;
}
extern "C" {
    pub fn clear_irq_resend(desc: *mut irq_desc);
}
extern "C" {
    pub fn irq_resend_init(desc: *mut irq_desc);
}
extern "C" {
    pub fn __irq_wake_thread(desc: *mut irq_desc, action: *mut irqaction);
}
extern "C" {
    pub fn wake_threads_waitq(desc: *mut irq_desc);
}

extern "C" {
    pub fn register_irq_proc(irq: c_uint, desc: *mut irq_desc);
}
extern "C" {
    pub fn unregister_irq_proc(irq: c_uint, desc: *mut irq_desc);
}
extern "C" {
    pub fn register_handler_proc(irq: c_uint, action: *mut irqaction);
}
extern "C" {
    pub fn unregister_handler_proc(irq: c_uint, action: *mut irqaction);
}
extern "C" {
    pub fn irq_proc_update_valid(desc: *mut irq_desc);
}

extern "C" {
    pub fn irq_can_set_affinity_usr(irq: c_uint) -> bool;
}
extern "C" {
    pub fn irq_affinity_schedule_notify_work(desc: *mut irq_desc);
}

extern "C" {
    pub fn irq_setup_affinity(desc: *mut irq_desc) -> c_int;
}

// Inline functions for support of irq chips on slow busses

extern "C" {
    pub fn __irq_put_desc_unlock(desc: *mut irq_desc, flags: c_ulong, bus: bool);
}

extern "C" {
    pub fn __irqd_to_state(_arg: d) -> return;
}
//
// Manipulation functions for irq_data.state
//

extern "C" {
    pub fn irq_common_data_get_node(_arg: &desc->irq_common_data) -> return;
}

extern "C" {
    pub fn irq_pm_handle_wakeup(desc: *mut irq_desc);
}
extern "C" {
    pub fn irq_pm_install_action(desc: *mut irq_desc, action: *mut irqaction);
}
extern "C" {
    pub fn irq_pm_remove_action(desc: *mut irq_desc, action: *mut irqaction);
}

extern "C" {
    pub fn irqd_is_setaffinity_pending(_arg: data) -> return;
}
extern "C" {
    pub fn irq_fixup_move_pending(desc: *mut irq_desc, force_clear: bool) -> bool;
}
extern "C" {
    pub fn irq_force_complete_move(desc: *mut irq_desc);
}
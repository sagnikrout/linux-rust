//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/suspend.h
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


// SPDX-License-Identifier: GPL-2.0

extern "C" {
    pub fn pm_set_vt_switch(_arg: c_int);
}

extern "C" {
    pub fn pm_prepare_console();
}
extern "C" {
    pub fn pm_restore_console();
}

pub type suspend_state_t = int ;

//
// struct platform_suspend_ops - Callbacks for managing platform dependent
// system sleep states.
//
// @valid: Callback to determine if given system sleep state is supported by
// the platform.
// Valid (ie. supported) states are advertised in /sys/power/state.  Note
// that it still may be impossible to enter given system sleep state if the
// conditions aren't right.
// There is the %suspend_valid_only_mem function available that can be
// assigned to this if the platform only supports mem sleep.
//
// @begin: Initialise a transition to given system sleep state.
// @begin() is executed right prior to suspending devices.  The information
// conveyed to the platform code by @begin() should be disregarded by it as
// soon as @end() is executed.  If @begin() fails (ie. returns nonzero),
// @prepare(), @enter() and @finish() will not be called by the PM core.
// This callback is optional.  However, if it is implemented, the argument
// passed to @enter() is redundant and should be ignored.
//
// @prepare: Prepare the platform for entering the system sleep state indicated
// by @begin().
// @prepare() is called right after devices have been suspended (ie. the
// appropriate .suspend() method has been executed for each device) and
// before device drivers' late suspend callbacks are executed.  It returns
// 0 on success or a negative error code otherwise, in which case the
// system cannot enter the desired sleep state (@prepare_late(), @enter(),
// and @wake() will not be called in that case).
//
// @prepare_late: Finish preparing the platform for entering the system sleep
// state indicated by @begin().
// @prepare_late is called before disabling nonboot CPUs and after
// device drivers' late suspend callbacks have been executed.  It returns
// 0 on success or a negative error code otherwise, in which case the
// system cannot enter the desired sleep state (@enter() will not be
// executed).
//
// @enter: Enter the system sleep state indicated by @begin() or represented by
// the argument if @begin() is not implemented.
// This callback is mandatory.  It returns 0 on success or a negative
// error code otherwise, in which case the system cannot enter the desired
// sleep state.
//
// @wake: Called when the system has just left a sleep state, right after
// the nonboot CPUs have been enabled and before device drivers' early
// resume callbacks are executed.
// This callback is optional, but should be implemented by the platforms
// that implement @prepare_late().  If implemented, it is always called
// after @prepare_late and @enter(), even if one of them fails.
//
// @finish: Finish wake-up of the platform.
// @finish is called right prior to calling device drivers' regular suspend
// callbacks.
// This callback is optional, but should be implemented by the platforms
// that implement @prepare().  If implemented, it is always called after
// @enter() and @wake(), even if any of them fails.  It is executed after
// a failing @prepare.
//
// @suspend_again: Returns whether the system should suspend again (true) or
// not (false). If the platform wants to poll sensors or execute some
// code during suspended without invoking userspace and most of devices,
// suspend_again callback is the place assuming that periodic-wakeup or
// alarm-wakeup is already setup. This allows to execute some codes while
// being kept suspended in the view of userland and devices.
//
// @end: Called by the PM core right after resuming devices, to indicate to
// the platform that the system has returned to the working state or
// the transition to the sleep state has been aborted.
// This callback is optional, but should be implemented by the platforms
// that implement @begin().  Accordingly, platforms implementing @begin()
// should also provide a @end() which cleans up transitions aborted before
// @enter().
//
// @recover: Recover the platform from a suspend failure.
// Called by the PM core if the suspending of devices fails.
// This callback is optional and should only be implemented by platforms
// which require special recovery actions in that situation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_suspend_ops {
    pub state): *mut *mut int (valid)(suspend_state_t,
    pub state): *mut *mut int (begin)(suspend_state_t,
    pub (*prepare)(void): *mut c_int,
    pub (*prepare_late)(void): *mut c_int,
    pub state): *mut *mut int (enter)(suspend_state_t,
    pub (*wake)(void): *mut c_void,
    pub (*finish)(void): *mut c_void,
    pub (*suspend_again)(void): *mut bool,
    pub (*end)(void): *mut c_void,
    pub (*recover)(void): *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_s2idle_ops {
    pub (*begin)(void): *mut c_int,
    pub (*prepare)(void): *mut c_int,
    pub (*prepare_late)(void): *mut c_int,
    pub (*check)(void): *mut c_void,
    pub (*wake)(void): *mut bool,
    pub (*restore_early)(void): *mut c_void,
    pub (*restore)(void): *mut c_void,
    pub (*end)(void): *mut c_void,
}

//
// suspend_set_ops - set platform dependent suspend operations
// @ops: The new suspend operations to set.
//
extern "C" {
    pub fn suspend_set_ops(ops: *const platform_suspend_ops);
}
extern "C" {
    pub fn suspend_valid_only_mem(state: suspend_state_t) -> c_int;
}

//
// pm_suspend_via_firmware - Check if platform firmware will suspend the system.
//
// To be called during system-wide power management transitions to sleep states
// or during the subsequent system-wide transitions back to the working state.
//
// Return 'true' if the platform firmware is going to be invoked at the end of
// the system-wide power management transition (to a sleep state) in progress in
// order to complete it, or if the platform firmware has been invoked in order
// to complete the last (or preceding) transition of the system to a sleep
// state.
//
// This matters if the caller needs or wants to carry out some special actions
// depending on whether or not control will be passed to the platform firmware
// subsequently (for example, the device may need to be reset before letting the
// platform firmware manipulate it, which is not necessary when the platform
// firmware is not going to be invoked) or when such special actions may have
// been carried out during the preceding transition of the system to a sleep
// state (as they may need to be taken into account).
//
// pm_resume_via_firmware - Check if platform firmware has woken up the system.
//
// To be called during system-wide power management transitions from sleep
// states.
//
// Return 'true' if the platform firmware has passed control to the kernel at
// the beginning of the system-wide power management transition in progress, so
// the event that woke up the system from sleep has been handled by the platform
// firmware.
//
// pm_suspend_no_platform - Check if platform may change device power states.
//
// To be called during system-wide power management transitions to sleep states
// or during the subsequent system-wide transitions back to the working state.
//
// Return 'true' if the power states of devices remain under full control of the
// kernel throughout the system-wide suspend and resume cycle in progress (that
// is, if a device is put into a certain power state during suspend, it can be
// expected to remain in that state during resume).
//
// Suspend-to-idle state machnine.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s2idle_states {
    S2IDLE_STATE_NONE,      /* Not suspended/suspending. */
    S2IDLE_STATE_ENTER,     /* Enter suspend-to-idle. */
    S2IDLE_STATE_WAKE,      /* Wake up from suspend-to-idle. */
}

extern "C" {
    pub fn unlikely(S2IDLE_STATE_ENTER: s2idle_state ==) -> return;
}
extern "C" {
    pub fn pm_suspend_default_s2idle() -> bool;
}
extern "C" {
    pub fn pm_states_init() -> void __init;
}
extern "C" {
    pub fn s2idle_set_ops(ops: *const platform_s2idle_ops);
}
extern "C" {
    pub fn s2idle_wake();
}
//
// arch_suspend_disable_irqs - disable IRQs for suspend
//
// Disables IRQs (in the default case). This is a weak symbol in the common
// code and thus allows architectures to override it if more needs to be
// done. Not called for suspend to disk.
//
extern "C" {
    pub fn arch_suspend_disable_irqs();
}
//
// arch_suspend_enable_irqs - enable IRQs after suspend
//
// Enables IRQs (in the default case). This is a weak symbol in the common
// code and thus allows architectures to override it if more needs to be
// done. Not called for suspend to disk.
//
extern "C" {
    pub fn arch_suspend_enable_irqs();
}
extern "C" {
    pub fn pm_suspend(state: suspend_state_t) -> c_int;
}

// struct pbe is used for creating lists of pages that should be restored
// atomically during the resume from disk, because the page frames they have
// occupied before the suspend are in use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pbe {
    pub /: *mut *mut *mut void address; / address of the copy,
    pub /: *mut *mut *mut void orig_address; / original address of a page,
    pub next: *mut pbe,
}

//
// struct platform_hibernation_ops - hibernation platform support
//
// The methods in this structure allow a platform to carry out special
// operations required by it during a hibernation transition.
//
// All the methods below, except for @recover(), must be implemented.
//
// @begin: Tell the platform driver that we're starting hibernation.
// Called right after shrinking memory and before freezing devices.
//
// @end: Called by the PM core right after resuming devices, to indicate to
// the platform that the system has returned to the working state.
//
// @pre_snapshot: Prepare the platform for creating the hibernation image.
// Called right after devices have been frozen and before the nonboot
// CPUs are disabled (runs with IRQs on).
//
// @finish: Restore the previous state of the platform after the hibernation
// image has been created *or* put the platform into the normal operation
// mode after the hibernation (the same method is executed in both cases).
// Called right after the nonboot CPUs have been enabled and before
// thawing devices (runs with IRQs on).
//
// @prepare: Prepare the platform for entering the low power state.
// Called right after the hibernation image has been saved and before
// devices are prepared for entering the low power state.
//
// @enter: Put the system into the low power state after the hibernation image
// has been saved to disk.
// Called after the nonboot CPUs have been disabled and all of the low
// level devices have been shut down (runs with IRQs off).
//
// @leave: Perform the first stage of the cleanup after the system sleep state
// indicated by @set_target() has been left.
// Called right after the control has been passed from the boot kernel to
// the image kernel, before the nonboot CPUs are enabled and before devices
// are resumed.  Executed with interrupts disabled.
//
// @pre_restore: Prepare system for the restoration from a hibernation image.
// Called right after devices have been frozen and before the nonboot
// CPUs are disabled (runs with IRQs on).
//
// @restore_cleanup: Clean up after a failing image restoration.
// Called right after the nonboot CPUs have been enabled and before
// thawing devices (runs with IRQs on).
//
// @recover: Recover the platform from a failure to suspend devices.
// Called by the PM core if the suspending of devices during hibernation
// fails.  This callback is optional and should only be implemented by
// platforms which require special recovery actions in that situation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_hibernation_ops {
    pub stage): *mut *mut int (begin)(pm_message_t,
    pub (*end)(void): *mut c_void,
    pub (*pre_snapshot)(void): *mut c_int,
    pub (*finish)(void): *mut c_void,
    pub (*prepare)(void): *mut c_int,
    pub (*enter)(void): *mut c_int,
    pub (*leave)(void): *mut c_void,
    pub (*pre_restore)(void): *mut c_int,
    pub (*restore_cleanup)(void): *mut c_void,
    pub (*recover)(void): *mut c_void,
}

// kernel/power/snapshot.c
extern "C" {
    pub fn register_nosave_region(b: c_ulong, e: c_ulong);
}
extern "C" {
    pub fn swsusp_page_is_forbidden(: *mut page) -> c_int;
}
extern "C" {
    pub fn swsusp_set_page_free(: *mut page);
}
extern "C" {
    pub fn swsusp_unset_page_free(: *mut page);
}
extern "C" {
    pub fn get_safe_page(gfp_mask: gfp_t) -> c_ulong;
}
extern "C" {
    pub fn swsusp_arch_suspend() -> asmlinkage int;
}
extern "C" {
    pub fn swsusp_arch_resume() -> asmlinkage int;
}
extern "C" {
    pub fn hibernation_set_ops(ops: *const platform_hibernation_ops);
}
extern "C" {
    pub fn hibernate() -> c_int;
}
extern "C" {
    pub fn system_entering_hibernation() -> bool;
}
extern "C" {
    pub fn hibernation_available() -> bool;
}
extern "C" {
    pub fn swsusp_save() -> asmlinkage int;
}
extern "C" {
    pub fn pfn_is_nosave(pfn: c_ulong) -> c_int;
}
extern "C" {
    pub fn hibernate_quiet_exec(data): *mut *mut int (func)(void, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn hibernate_resume_nonboot_cpu_disable() -> c_int;
}
extern "C" {
    pub fn arch_hibernation_header_save(addr: *mut c_void, max_size: c_uint) -> c_int;
}
extern "C" {
    pub fn arch_hibernation_header_restore(addr: *mut c_void) -> c_int;
}

extern "C" {
    pub fn pm_hibernation_mode_is_suspend() -> bool;
}

extern "C" {
    pub fn arch_resume_nosmt() -> c_int;
}

extern "C" {
    pub fn is_hibernate_resume_dev(dev: dev_t) -> c_int;
}

// Hibernation and suspend events
pub const PM_HIBERNATION_PREPARE: c_uint = 0x0001 /* Going to hibernate */;
pub const PM_POST_HIBERNATION: c_uint = 0x0002 /* Hibernation finished */;
pub const PM_SUSPEND_PREPARE: c_uint = 0x0003 /* Going to suspend the system */;
pub const PM_POST_SUSPEND: c_uint = 0x0004 /* Suspend finished */;
pub const PM_RESTORE_PREPARE: c_uint = 0x0005 /* Going to restore a saved image */;
pub const PM_POST_RESTORE: c_uint = 0x0006 /* Restore failed */;

extern "C" {
    pub fn save_processor_state();
}
extern "C" {
    pub fn restore_processor_state();
}
// kernel/power/main.c
extern "C" {
    pub fn register_pm_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_pm_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn ksys_sync_helper();
}
extern "C" {
    pub fn pm_report_hw_sleep_time(t: u64);
}
extern "C" {
    pub fn pm_report_max_hw_sleep(t: u64);
}
extern "C" {
    pub fn pm_restrict_gfp_mask();
}
extern "C" {
    pub fn pm_restore_gfp_mask();
}

// drivers/base/power/wakeup.c
extern "C" {
    pub fn pm_wakeup_pending() -> bool;
}
extern "C" {
    pub fn pm_system_wakeup();
}
extern "C" {
    pub fn pm_system_cancel_wakeup();
}
extern "C" {
    pub fn pm_wakeup_clear(irq_number: c_uint);
}
extern "C" {
    pub fn pm_system_irq_wakeup(irq_number: c_uint);
}
extern "C" {
    pub fn pm_wakeup_irq() -> c_uint;
}
extern "C" {
    pub fn pm_get_wakeup_count(count: *mut c_uint, block: bool) -> bool;
}
extern "C" {
    pub fn pm_save_wakeup_count(count: c_uint) -> bool;
}
extern "C" {
    pub fn pm_wakep_autosleep_enabled(set: bool);
}
extern "C" {
    pub fn pm_print_active_wakeup_sources();
}
extern "C" {
    pub fn lock_system_sleep() -> c_uint;
}
extern "C" {
    pub fn unlock_system_sleep(int: unsigned);
}
extern "C" {
    pub fn pm_sleep_transition_in_progress() -> bool;
}
extern "C" {
    pub fn pm_hibernate_is_recovering() -> bool;
}

extern "C" {
    pub fn pm_debug_messages_should_print() -> bool;
}

//
// pm_pr_dbg - print pm sleep debug messages
//
// If pm_debug_messages_on is enabled and the system is entering/leaving
// suspend, print message.
// If pm_debug_messages_on is disabled and CONFIG_DYNAMIC_DEBUG is enabled,
// print message only from instances explicitly enabled on dynamic debug's
// control.
// If pm_debug_messages_on is disabled and CONFIG_DYNAMIC_DEBUG is disabled,
// don't print message.
//

// kernel/power/autosleep.c
extern "C" {
    pub fn queue_up_suspend_work();
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum suspend_stat_step {
    SUSPEND_WORKING = 0,
    SUSPEND_FREEZE,
    SUSPEND_PREPARE,
    SUSPEND_SUSPEND,
    SUSPEND_SUSPEND_LATE,
    SUSPEND_SUSPEND_NOIRQ,
    SUSPEND_RESUME_NOIRQ,
    SUSPEND_RESUME_EARLY,
    SUSPEND_RESUME
}

extern "C" {
    pub fn dpm_save_failed_dev(name: *const c_char);
}
extern "C" {
    pub fn dpm_save_failed_step(step: suspend_stat_step);
}

//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pm_runtime.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// pm_runtime.h - Device run-time power management helper functions.
//
// Copyright (C) 2009 Rafael J. Wysocki <rjw@sisk.pl>
//

// Runtime PM flag argument bits
pub const RPM_ASYNC: c_uint = 0x01	/* Request is asynchronous */;
pub const RPM_NOWAIT: c_uint = 0x02	/* Don't wait for concurrent;
pub const RPM_GET_PUT: c_uint = 0x04	/* Increment/decrement the;
pub const RPM_AUTO: c_uint = 0x08	/* Use autosuspend_delay */;
pub const RPM_TRANSPARENT: c_uint = 0x10	/* Succeed if runtime PM is disabled */;
//
// Use this for defining a set of PM operations to be used in all situations
// (system suspend, hibernation or runtime PM).
//
// Note that the behaviour differs from the deprecated UNIVERSAL_DEV_PM_OPS()
// macro, which uses the provided callbacks for both runtime PM and system
// sleep, while DEFINE_RUNTIME_DEV_PM_OPS() uses pm_runtime_force_suspend()
// and pm_runtime_force_resume() for its system sleep callbacks.
//
// If the underlying dev_pm_ops struct symbol has to be exported, use
// EXPORT_RUNTIME_DEV_PM_OPS() or EXPORT_GPL_RUNTIME_DEV_PM_OPS() instead.
//

extern "C" {
    pub fn queue_work(_arg: pm_wq, _arg: work) -> return;
}
extern "C" {
    pub fn pm_generic_runtime_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_generic_runtime_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn __pm_runtime_idle(dev: *mut device, rpmflags: c_int) -> c_int;
}
extern "C" {
    pub fn __pm_runtime_suspend(dev: *mut device, rpmflags: c_int) -> c_int;
}
extern "C" {
    pub fn __pm_runtime_resume(dev: *mut device, rpmflags: c_int) -> c_int;
}
extern "C" {
    pub fn pm_runtime_get_if_active(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_runtime_get_if_in_use(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_schedule_suspend(dev: *mut device, delay: c_uint) -> c_int;
}
extern "C" {
    pub fn __pm_runtime_set_status(dev: *mut device, status: c_uint) -> c_int;
}
extern "C" {
    pub fn pm_runtime_barrier(dev: *mut device);
}
extern "C" {
    pub fn pm_runtime_block_if_disabled(dev: *mut device) -> bool;
}
extern "C" {
    pub fn pm_runtime_unblock(dev: *mut device);
}
extern "C" {
    pub fn pm_runtime_enable(dev: *mut device);
}
extern "C" {
    pub fn __pm_runtime_disable(dev: *mut device, check_resume: bool);
}
extern "C" {
    pub fn pm_runtime_allow(dev: *mut device);
}
extern "C" {
    pub fn pm_runtime_forbid(dev: *mut device);
}
extern "C" {
    pub fn pm_runtime_no_callbacks(dev: *mut device);
}
extern "C" {
    pub fn pm_runtime_irq_safe(dev: *mut device);
}
extern "C" {
    pub fn __pm_runtime_use_autosuspend(dev: *mut device, use: bool);
}
extern "C" {
    pub fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
}
extern "C" {
    pub fn pm_runtime_autosuspend_expiration(dev: *mut device) -> u64;
}
extern "C" {
    pub fn pm_runtime_set_memalloc_noio(dev: *mut device, enable: bool);
}
extern "C" {
    pub fn pm_runtime_get_suppliers(dev: *mut device);
}
extern "C" {
    pub fn pm_runtime_put_suppliers(dev: *mut device);
}
extern "C" {
    pub fn pm_runtime_new_link(dev: *mut device);
}
extern "C" {
    pub fn pm_runtime_drop_link(link: *mut device_link);
}
extern "C" {
    pub fn pm_runtime_release_supplier(link: *mut device_link);
}
extern "C" {
    pub fn devm_pm_runtime_set_active_enabled(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn devm_pm_runtime_enable(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn devm_pm_runtime_get_noresume(dev: *mut device) -> c_int;
}
//
// pm_suspend_ignore_children - Set runtime PM behavior regarding children.
// @dev: Target device.
// @enable: Whether or not to ignore possible dependencies on children.
//
// The dependencies of @dev on its children will not be taken into account by
// the runtime PM framework going forward if @enable is %true, or they will
// be taken into account otherwise.
//
// pm_runtime_get_noresume - Bump up runtime PM usage counter of a device.
// @dev: Target device.
//
// pm_runtime_put_noidle - Drop runtime PM usage counter of a device.
// @dev: Target device.
//
// Decrement the runtime PM usage counter of @dev unless it is 0 already.
//
// pm_runtime_suspended - Check whether or not a device is runtime-suspended.
// @dev: Target device.
//
// Return %true if runtime PM is enabled for @dev and its runtime PM status is
// %RPM_SUSPENDED, or %false otherwise.
//
// Note that the return value of this function can only be trusted if it is
// called under the runtime PM lock of @dev or under conditions in which
// runtime PM cannot be either disabled or enabled for @dev and its runtime PM
// status cannot change.
//
// pm_runtime_active - Check whether or not a device is runtime-active.
// @dev: Target device.
//
// Return %true if runtime PM is disabled for @dev or its runtime PM status is
// %RPM_ACTIVE, or %false otherwise.
//
// Note that the return value of this function can only be trusted if it is
// called under the runtime PM lock of @dev or under conditions in which
// runtime PM cannot be either disabled or enabled for @dev and its runtime PM
// status cannot change.
//
// pm_runtime_status_suspended - Check if runtime PM status is "suspended".
// @dev: Target device.
//
// Return %true if the runtime PM status of @dev is %RPM_SUSPENDED, or %false
// otherwise, regardless of whether or not runtime PM has been enabled for @dev.
//
// Note that the return value of this function can only be trusted if it is
// called under the runtime PM lock of @dev or under conditions in which the
// runtime PM status of @dev cannot change.
//
// pm_runtime_enabled - Check if runtime PM is enabled.
// @dev: Target device.
//
// Return %true if runtime PM is enabled for @dev or %false otherwise.
//
// Note that the return value of this function can only be trusted if it is
// called under the runtime PM lock of @dev or under conditions in which
// runtime PM cannot be either disabled or enabled for @dev.
//
// pm_runtime_blocked - Check if runtime PM enabling is blocked.
// @dev: Target device.
//
// Do not call this function outside system suspend/resume code paths.
//
// pm_runtime_has_no_callbacks - Check if runtime PM callbacks may be present.
// @dev: Target device.
//
// Return %true if @dev is a special device without runtime PM callbacks or
// %false otherwise.
//
// pm_runtime_mark_last_busy - Update the last access time of a device.
// @dev: Target device.
//
// Update the last access time of @dev used by the runtime PM autosuspend
// mechanism to the current time as returned by ktime_get_mono_fast_ns().
//
// pm_runtime_is_irq_safe - Check if runtime PM can work in interrupt context.
// @dev: Target device.
//
// Return %true if @dev has been marked as an "IRQ-safe" device (with respect
// to runtime PM), in which case its runtime PM callabcks can be expected to
// work correctly when invoked from interrupt handlers.
//
extern "C" {
    pub fn pm_runtime_suspended_time(dev: *mut device) -> u64;
}

extern "C" {
    pub fn pm_runtime_need_not_resume(dev: *mut device) -> bool;
}
extern "C" {
    pub fn pm_runtime_force_resume(dev: *mut device) -> c_int;
}

//
// pm_runtime_idle - Conditionally set up autosuspend of a device or suspend it.
// @dev: Target device.
//
// Invoke the "idle check" callback of @dev and, depending on its return value,
// set up autosuspend of @dev or suspend it (depending on whether or not
// autosuspend has been enabled for it).
//
// Return:
// * 0: Success.
// * -EINVAL: Runtime PM error.
// * -EACCES: Runtime PM disabled.
// * -EAGAIN: Runtime PM usage counter non-zero, Runtime PM status change
// ongoing or device not in %RPM_ACTIVE state.
// * -EBUSY: Runtime PM child_count non-zero.
// * -EPERM: Device PM QoS resume latency 0.
// * -EINPROGRESS: Suspend already in progress.
// * -ENOSYS: CONFIG_PM not enabled.
// Other values and conditions for the above values are possible as returned by
// Runtime PM idle and suspend callbacks.
//
extern "C" {
    pub fn __pm_runtime_idle(_arg: dev, _arg: 0) -> return;
}
//
// pm_runtime_suspend - Suspend a device synchronously.
// @dev: Target device.
//
// Return:
// * 1: Success; device was already suspended.
// * 0: Success.
// * -EINVAL: Runtime PM error.
// * -EACCES: Runtime PM disabled.
// * -EAGAIN: Runtime PM usage counter non-zero or Runtime PM status change
// ongoing.
// * -EBUSY: Runtime PM child_count non-zero.
// * -EPERM: Device PM QoS resume latency 0.
// * -ENOSYS: CONFIG_PM not enabled.
// Other values and conditions for the above values are possible as returned by
// Runtime PM suspend callbacks.
//
extern "C" {
    pub fn __pm_runtime_suspend(_arg: dev, _arg: 0) -> return;
}
//
// pm_runtime_autosuspend - Update the last access time and set up autosuspend
// of a device.
// @dev: Target device.
//
// First update the last access time, then set up autosuspend of @dev or suspend
// it (depending on whether or not autosuspend is enabled for it) without
// engaging its "idle check" callback.
//
// Return:
// * 1: Success; device was already suspended.
// * 0: Success.
// * -EINVAL: Runtime PM error.
// * -EACCES: Runtime PM disabled.
// * -EAGAIN: Runtime PM usage counter non-zero or Runtime PM status change
// ongoing.
// * -EBUSY: Runtime PM child_count non-zero.
// * -EPERM: Device PM QoS resume latency 0.
// * -ENOSYS: CONFIG_PM not enabled.
// Other values and conditions for the above values are possible as returned by
// Runtime PM suspend callbacks.
//
extern "C" {
    pub fn __pm_runtime_suspend(_arg: dev, _arg: RPM_AUTO) -> return;
}
//
// pm_runtime_resume - Resume a device synchronously.
// @dev: Target device.
//
extern "C" {
    pub fn __pm_runtime_resume(_arg: dev, _arg: 0) -> return;
}
//
// pm_request_idle - Queue up "idle check" execution for a device.
// @dev: Target device.
//
// Queue up a work item to run an equivalent of pm_runtime_idle() for @dev
// asynchronously.
//
// Return:
// * 0: Success.
// * -EINVAL: Runtime PM error.
// * -EACCES: Runtime PM disabled.
// * -EAGAIN: Runtime PM usage counter non-zero, Runtime PM status change
// ongoing or device not in %RPM_ACTIVE state.
// * -EBUSY: Runtime PM child_count non-zero.
// * -EPERM: Device PM QoS resume latency 0.
// * -EINPROGRESS: Suspend already in progress.
// * -ENOSYS: CONFIG_PM not enabled.
//
extern "C" {
    pub fn __pm_runtime_idle(_arg: dev, _arg: RPM_ASYNC) -> return;
}
//
// pm_request_resume - Queue up runtime-resume of a device.
// @dev: Target device.
//
extern "C" {
    pub fn __pm_runtime_resume(_arg: dev, _arg: RPM_ASYNC) -> return;
}
//
// pm_request_autosuspend - Update the last access time and queue up autosuspend
// of a device.
// @dev: Target device.
//
// Update the last access time of a device and queue up a work item to run an
// equivalent pm_runtime_autosuspend() for @dev asynchronously.
//
// Return:
// * 1: Success; device was already suspended.
// * 0: Success.
// * -EINVAL: Runtime PM error.
// * -EACCES: Runtime PM disabled.
// * -EAGAIN: Runtime PM usage counter non-zero or Runtime PM status change
// ongoing.
// * -EBUSY: Runtime PM child_count non-zero.
// * -EPERM: Device PM QoS resume latency 0.
// * -EINPROGRESS: Suspend already in progress.
// * -ENOSYS: CONFIG_PM not enabled.
//
extern "C" {
    pub fn __pm_runtime_suspend(_arg: dev, RPM_AUTO: RPM_ASYNC |) -> return;
}
//
// pm_runtime_get - Bump up usage counter and queue up resume of a device.
// @dev: Target device.
//
// Bump up the runtime PM usage counter of @dev and queue up a work item to
// carry out runtime-resume of it.
//
extern "C" {
    pub fn __pm_runtime_resume(_arg: dev, RPM_ASYNC: RPM_GET_PUT |) -> return;
}
//
// pm_runtime_get_sync - Bump up usage counter of a device and resume it.
// @dev: Target device.
//
// Bump up the runtime PM usage counter of @dev and carry out runtime-resume of
// it synchronously.
//
// The possible return values of this function are the same as for
// pm_runtime_resume() and the runtime PM usage counter of @dev remains
// incremented in all cases, even if it returns an error code.
// Consider using pm_runtime_resume_and_get() instead of it, especially
// if its return value is checked by the caller, as this is likely to result
// in cleaner code.
//
extern "C" {
    pub fn __pm_runtime_resume(_arg: dev, _arg: RPM_GET_PUT) -> return;
}
//
// pm_runtime_resume_and_get - Bump up usage counter of a device and resume it.
// @dev: Target device.
//
// Resume @dev synchronously and if that is successful, increment its runtime
// PM usage counter. Return 0 if the runtime PM usage counter of @dev has been
// incremented or a negative error code otherwise.
//
extern "C" {
    pub fn pm_runtime_get_active(_arg: dev, _arg: 0) -> return;
}
//
// pm_runtime_put - Drop device usage counter and queue up "idle check" if 0.
// @dev: Target device.
//
// Decrement the runtime PM usage counter of @dev and if it turns out to be
// equal to 0, queue up a work item for @dev like in pm_request_idle().
//
// __pm_runtime_put_autosuspend - Drop device usage counter and queue autosuspend if 0.
// @dev: Target device.
//
// Decrement the runtime PM usage counter of @dev and if it turns out to be
// equal to 0, queue up a work item for @dev like in pm_request_autosuspend().
//
// Return:
// * 1: Success. Usage counter dropped to zero, but device was already suspended.
// * 0: Success.
// * -EINVAL: Runtime PM error.
// * -EACCES: Runtime PM disabled.
// * -EAGAIN: Runtime PM usage counter became non-zero or Runtime PM status
// change ongoing.
// * -EBUSY: Runtime PM child_count non-zero.
// * -EPERM: Device PM QoS resume latency 0.
// * -EINPROGRESS: Suspend already in progress.
// * -ENOSYS: CONFIG_PM not enabled.
//
extern "C" {
    pub fn __pm_runtime_suspend(_arg: dev, RPM_AUTO: RPM_GET_PUT | RPM_ASYNC |) -> return;
}
//
// pm_runtime_put_autosuspend - Update the last access time of a device, drop
// its usage counter and queue autosuspend if the usage counter becomes 0.
// @dev: Target device.
//
// Update the last access time of @dev, decrement runtime PM usage counter of
// @dev and if it turns out to be equal to 0, queue up a work item for @dev like
// in pm_request_autosuspend().
//
// Return:
// * 1: Success. Usage counter dropped to zero, but device was already suspended.
// * 0: Success.
// * -EINVAL: Runtime PM error.
// * -EACCES: Runtime PM disabled.
// * -EAGAIN: Runtime PM usage counter became non-zero or Runtime PM status
// change ongoing.
// * -EBUSY: Runtime PM child_count non-zero.
// * -EPERM: Device PM QoS resume latency 0.
// * -EINPROGRESS: Suspend already in progress.
// * -ENOSYS: CONFIG_PM not enabled.
//
extern "C" {
    pub fn __pm_runtime_put_autosuspend(_arg: dev) -> return;
}
//
// Use the following guards with ACQUIRE()/ACQUIRE_ERR().
//
// The difference between the "_try" and "_try_enabled" variants is that the
// former do not produce an error when runtime PM is disabled for the given
// device.
//
// ACQUIRE() wrapper macros for the guards defined above.

//
// ACQUIRE_ERR() wrapper macro for guard pm_runtime_active.
//
// Always check PM_RUNTIME_ACQUIRE_ERR() after using one of the
// PM_RUNTIME_ACQUIRE*() macros defined above (yes, it can be used with
// any of them) and if it is nonzero, avoid accessing the given device.
//

//
// pm_runtime_put_sync - Drop device usage counter and run "idle check" if 0.
// @dev: Target device.
//
// Decrement the runtime PM usage counter of @dev and if it turns out to be
// equal to 0, invoke the "idle check" callback of @dev and, depending on its
// return value, set up autosuspend of @dev or suspend it (depending on whether
// or not autosuspend has been enabled for it).
//
// The runtime PM usage counter of @dev remains decremented in all cases, even
// if it returns an error code.
//
// Return:
// * 1: Success. Usage counter dropped to zero, but device was already suspended.
// * 0: Success.
// * -EINVAL: Runtime PM error.
// * -EACCES: Runtime PM disabled.
// * -EAGAIN: Runtime PM usage counter became non-zero or Runtime PM status
// change ongoing.
// * -EBUSY: Runtime PM child_count non-zero.
// * -EPERM: Device PM QoS resume latency 0.
// * -ENOSYS: CONFIG_PM not enabled.
// Other values and conditions for the above values are possible as returned by
// Runtime PM suspend callbacks.
//
extern "C" {
    pub fn __pm_runtime_idle(_arg: dev, _arg: RPM_GET_PUT) -> return;
}
//
// pm_runtime_put_sync_suspend - Drop device usage counter and suspend if 0.
// @dev: Target device.
//
// Decrement the runtime PM usage counter of @dev and if it turns out to be
// equal to 0, carry out runtime-suspend of @dev synchronously.
//
// The runtime PM usage counter of @dev remains decremented in all cases, even
// if it returns an error code.
//
// Return:
// * 1: Success. Usage counter dropped to zero, but device was already suspended.
// * 0: Success.
// * -EINVAL: Runtime PM error.
// * -EACCES: Runtime PM disabled.
// * -EAGAIN: Runtime PM usage counter became non-zero or Runtime PM status
// change ongoing.
// * -EBUSY: Runtime PM child_count non-zero.
// * -EPERM: Device PM QoS resume latency 0.
// * -ENOSYS: CONFIG_PM not enabled.
// Other values and conditions for the above values are possible as returned by
// Runtime PM suspend callbacks.
//
extern "C" {
    pub fn __pm_runtime_suspend(_arg: dev, _arg: RPM_GET_PUT) -> return;
}
//
// pm_runtime_put_sync_autosuspend - Update the last access time of a device,
// drop device usage counter and autosuspend if 0.
// @dev: Target device.
//
// Update the last access time of @dev, decrement the runtime PM usage counter
// of @dev and if it turns out to be equal to 0, set up autosuspend of @dev or
// suspend it synchronously (depending on whether or not autosuspend has been
// enabled for it).
//
// The runtime PM usage counter of @dev remains decremented in all cases, even
// if it returns an error code.
//
// Return:
// * 1: Success. Usage counter dropped to zero, but device was already suspended.
// * 0: Success.
// * -EINVAL: Runtime PM error.
// * -EACCES: Runtime PM disabled.
// * -EAGAIN: Runtime PM usage counter became non-zero or Runtime PM status
// change ongoing.
// * -EBUSY: Runtime PM child_count non-zero.
// * -EPERM: Device PM QoS resume latency 0.
// * -EINPROGRESS: Suspend already in progress.
// * -ENOSYS: CONFIG_PM not enabled.
// Other values and conditions for the above values are possible as returned by
// Runtime PM suspend callbacks.
//
extern "C" {
    pub fn __pm_runtime_suspend(_arg: dev, RPM_AUTO: RPM_GET_PUT |) -> return;
}
//
// pm_runtime_set_active - Set runtime PM status to "active".
// @dev: Target device.
//
// Set the runtime PM status of @dev to %RPM_ACTIVE and ensure that dependencies
// of it will be taken into account.
//
// It is not valid to call this function for devices with runtime PM enabled.
//
extern "C" {
    pub fn __pm_runtime_set_status(_arg: dev, _arg: RPM_ACTIVE) -> return;
}
//
// pm_runtime_set_suspended - Set runtime PM status to "suspended".
// @dev: Target device.
//
// Set the runtime PM status of @dev to %RPM_SUSPENDED and ensure that
// dependencies of it will be taken into account.
//
// It is not valid to call this function for devices with runtime PM enabled.
//
extern "C" {
    pub fn __pm_runtime_set_status(_arg: dev, _arg: RPM_SUSPENDED) -> return;
}
//
// pm_runtime_disable - Disable runtime PM for a device.
// @dev: Target device.
//
// Prevent the runtime PM framework from working with @dev by incrementing its
// "disable" counter.
//
// If the counter is zero when this function runs and there is a pending runtime
// resume request for @dev, it will be resumed.  If the counter is still zero at
// that point, all of the pending runtime PM requests for @dev will be canceled
// and all runtime PM operations in progress involving it will be waited for to
// complete.
//
// For each invocation of this function for @dev, there must be a matching
// pm_runtime_enable() call, so that runtime PM is eventually enabled for it
// again.
//
// pm_runtime_use_autosuspend - Allow autosuspend to be used for a device.
// @dev: Target device.
//
// Allow the runtime PM autosuspend mechanism to be used for @dev whenever
// requested (or "autosuspend" will be handled as direct runtime-suspend for
// it).
//
// NOTE: It's important to undo this with pm_runtime_dont_use_autosuspend()
// at driver exit time unless your driver initially enabled pm_runtime
// with devm_pm_runtime_enable() (which handles it for you).
//
// pm_runtime_dont_use_autosuspend - Prevent autosuspend from being used.
// @dev: Target device.
//
// Prevent the runtime PM autosuspend mechanism from being used for @dev which
// means that "autosuspend" will be handled as direct runtime-suspend for it
// going forward.
//

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/intel_runtime_pm.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2019 Intel Corporation
//

//
// This struct helps tracking the state needed for runtime PM, which puts the
// device in PCI D3 state. Notice that when this happens, nothing on the
// graphics device works, even register access, so we don't get interrupts nor
// anything else.
//
// Every piece of our code that needs to actually touch the hardware needs to
// either call intel_runtime_pm_get or call intel_display_power_get with the
// appropriate power domain.
//
// Our driver uses the autosuspend delay feature, which means we'll only really
// suspend if we stay with zero refcount for a certain amount of time. The
// default value is currently very conservative (see intel_runtime_pm_enable), but
// it can be changed with the standard runtime PM files from sysfs.
//
// The irqs_disabled variable becomes true exactly after we disable the IRQs and
// goes back to false exactly before we re-enable the IRQs. We use this variable
// to check if someone is trying to enable/disable IRQs while they're supposed
// to be disabled. This shouldn't happen and we'll print some error messages in
// case it happens.
//
// For more, read the Documentation/power/runtime_pm.rst.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_runtime_pm {
    pub wakeref_count: core::sync::atomic::AtomicI32,
    pub /: *mut *mut *mut device kdev; / points to i915->drm.dev,
    pub available: bool,
    pub no_wakeref_tracking: bool,
//
// Protects access to lmem usefault list.
// It is required, if we are outside of the runtime suspend path,
// access to @lmem_userfault_list requires always first grabbing the
// runtime pm, to ensure we can't race against runtime suspend.
// Once we have that we also need to grab @lmem_userfault_lock,
// at which point we have exclusive access.
// The runtime suspend path is special since it doesn't really hold any locks,
// but instead has exclusive access by virtue of all other accesses requiring
// holding the runtime pm wakeref.
//
    pub lmem_userfault_lock: spinlock_t,
//
// Keep list of userfaulted gem obj, which require to release their
// mmap mappings at runtime suspend path.
//
    pub lmem_userfault_list: list_head,
// Manual runtime pm autosuspend delay for user GGTT/lmem mmaps
    pub userfault_wakeref: intel_wakeref_auto,

//
// To aide detection of wakeref leaks and general misuse, we
// track all wakeref holders. With manual markup (i.e. returning
// a cookie to each rpm_get caller which they then supply to their
// paired rpm_put) we can remove corresponding pairs of and keep
// the array trimmed to active wakerefs.
//
    pub debug: ref_tracker_dir,

}

extern "C" {
    pub fn pm_runtime_suspended(_arg: rpm->kdev) -> return;
}
//
// disable_rpm_wakeref_asserts - disable the RPM assert checks
// @rpm: the intel_runtime_pm structure
//
// This function disable asserts that check if we hold an RPM wakelock
// reference, while keeping the device-not-suspended checks still enabled.
// It's meant to be used only in special circumstances where our rule about
// the wakelock refcount wrt. the device power state doesn't hold. According
// to this rule at any point where we access the HW or want to keep the HW in
// an active state we must hold an RPM wakelock reference acquired via one of
// the intel_runtime_pm_get() helpers. Currently there are a few special spots
// where this rule doesn't hold: the IRQ and suspend/resume handlers, the
// forcewake release timer, and the GPU RPS and hangcheck works. All other
// users should avoid using this function.
//
// Any calls to this function must have a symmetric call to
// enable_rpm_wakeref_asserts().
//
// enable_rpm_wakeref_asserts - re-enable the RPM assert checks
// @rpm: the intel_runtime_pm structure
//
// This function re-enables the RPM assert checks after disabling them with
// disable_rpm_wakeref_asserts. It's meant to be used only in special
// circumstances otherwise its use should be avoided.
//
// Any calls to this function must have a symmetric call to
// disable_rpm_wakeref_asserts().
//
extern "C" {
    pub fn intel_runtime_pm_init_early(rpm: *mut intel_runtime_pm);
}
extern "C" {
    pub fn intel_runtime_pm_enable(rpm: *mut intel_runtime_pm);
}
extern "C" {
    pub fn intel_runtime_pm_disable(rpm: *mut intel_runtime_pm);
}
extern "C" {
    pub fn intel_runtime_pm_driver_release(rpm: *mut intel_runtime_pm);
}
extern "C" {
    pub fn intel_runtime_pm_driver_last_release(rpm: *mut intel_runtime_pm);
}
extern "C" {
    pub fn intel_runtime_pm_get(rpm: *mut intel_runtime_pm) -> intel_wakeref_t;
}
extern "C" {
    pub fn intel_runtime_pm_get_if_in_use(rpm: *mut intel_runtime_pm) -> intel_wakeref_t;
}
extern "C" {
    pub fn intel_runtime_pm_get_if_active(rpm: *mut intel_runtime_pm) -> intel_wakeref_t;
}
extern "C" {
    pub fn intel_runtime_pm_get_noresume(rpm: *mut intel_runtime_pm) -> intel_wakeref_t;
}
extern "C" {
    pub fn intel_runtime_pm_get_raw(rpm: *mut intel_runtime_pm) -> intel_wakeref_t;
}

extern "C" {
    pub fn intel_runtime_pm_put_unchecked(rpm: *mut intel_runtime_pm);
}

extern "C" {
    pub fn intel_runtime_pm_put(rpm: *mut intel_runtime_pm, wref: intel_wakeref_t);
}

extern "C" {
    pub fn intel_runtime_pm_put_raw(rpm: *mut intel_runtime_pm, wref: intel_wakeref_t);
}


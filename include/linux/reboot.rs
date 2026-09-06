//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/reboot.h
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

pub const SYS_DOWN: c_uint = 0x0001	/* Notify of system down */;

pub const SYS_HALT: c_uint = 0x0002	/* Notify of system halt */;
pub const SYS_POWER_OFF: c_uint = 0x0003	/* Notify of system power off */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reboot_mode {
    REBOOT_UNDEFINED = -1,
    REBOOT_COLD = 0,
    REBOOT_WARM,
    REBOOT_HARD,
    REBOOT_SOFT,
    REBOOT_GPIO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reboot_type {
    BOOT_TRIPLE	= 't',
    BOOT_KBD	= 'k',
    BOOT_BIOS	= 'b',
    BOOT_ACPI	= 'a',
    BOOT_EFI	= 'e',
    BOOT_CF9_FORCE	= 'p',
    BOOT_CF9_SAFE	= 'q',
}

extern "C" {
    pub fn register_reboot_notifier(: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_reboot_notifier(: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn devm_register_reboot_notifier(: *mut device, : *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn register_restart_handler(: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_restart_handler(: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn do_kernel_restart(cmd: *mut c_char);
}
//
// Architecture-specific implementations of sys_reboot commands.
//
extern "C" {
    pub fn migrate_to_reboot_cpu();
}
extern "C" {
    pub fn machine_restart(cmd: *mut c_char);
}
extern "C" {
    pub fn machine_halt();
}
extern "C" {
    pub fn machine_power_off();
}
extern "C" {
    pub fn machine_shutdown();
}
extern "C" {
    pub fn machine_crash_shutdown(: *mut pt_regs);
}
extern "C" {
    pub fn do_kernel_power_off();
}
//
// sys-off handler API.
//
// Standard sys-off priority levels. Users are expected to set priorities
// relative to the standard levels.
//
// SYS_OFF_PRIO_PLATFORM:	Use this for platform-level handlers.
//
// SYS_OFF_PRIO_LOW:		Use this for handler of last resort.
//
// SYS_OFF_PRIO_DEFAULT:	Use this for normal handlers.
//
// SYS_OFF_PRIO_HIGH:		Use this for higher priority handlers.
//
// SYS_OFF_PRIO_FIRMWARE:	Use this if handler uses firmware call.
//

pub const SYS_OFF_PRIO_DEFAULT: c_int = 0;
pub const SYS_OFF_PRIO_HIGH: c_int = 192;
pub const SYS_OFF_PRIO_FIRMWARE: c_int = 224;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sys_off_mode {
//
// @SYS_OFF_MODE_POWER_OFF_PREPARE:
//
// Handlers prepare system to be powered off. Handlers are
// allowed to sleep.
//
    SYS_OFF_MODE_POWER_OFF_PREPARE,

//
// @SYS_OFF_MODE_POWER_OFF:
//
// Handlers power-off system. Handlers are disallowed to sleep.
//
    SYS_OFF_MODE_POWER_OFF,

//
// @SYS_OFF_MODE_RESTART_PREPARE:
//
// Handlers prepare system to be restarted. Handlers are
// allowed to sleep.
//
    SYS_OFF_MODE_RESTART_PREPARE,

//
// @SYS_OFF_MODE_RESTART:
//
// Handlers restart system. Handlers are disallowed to sleep.
//
    SYS_OFF_MODE_RESTART,
}

//
// struct sys_off_data - sys-off callback argument
//
// @mode: Mode ID. Currently used only by the sys-off restart mode,
// see enum reboot_mode for the available modes.
// @cb_data: User's callback data.
// @cmd: Command string. Currently used only by the sys-off restart mode,
// NULL otherwise.
// @dev: Device of the sys-off handler. Only if known (devm_register_*),
// NULL otherwise.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sys_off_data {
    pub mode: c_int,
    pub cb_data: *mut c_void,
    pub cmd: *const c_char,
    pub dev: *mut device,
}

extern "C" {
    pub fn unregister_sys_off_handler(handler: *mut sys_off_handler);
}
extern "C" {
    pub fn register_platform_power_off((*power_off)(void): *mut c_void) -> c_int;
}
extern "C" {
    pub fn unregister_platform_power_off((*power_off)(void): *mut c_void);
}
//
// Architecture independent implemenations of sys_reboot commands.
//
extern "C" {
    pub fn kernel_restart_prepare(cmd: *mut c_char);
}
extern "C" {
    pub fn kernel_restart(cmd: *mut c_char);
}
extern "C" {
    pub fn kernel_halt();
}
extern "C" {
    pub fn kernel_power_off();
}
extern "C" {
    pub fn kernel_can_power_off() -> bool;
}
extern "C" {
    pub fn ctrl_alt_del();
}
extern "C" {
    pub fn orderly_poweroff(force: bool);
}
extern "C" {
    pub fn orderly_reboot();
}
//
// enum hw_protection_action - Hardware protection action
//
// @HWPROT_ACT_DEFAULT:
// The default action should be taken. This is HWPROT_ACT_SHUTDOWN
// by default, but can be overridden.
// @HWPROT_ACT_SHUTDOWN:
// The system should be shut down (powered off) for HW protection.
// @HWPROT_ACT_REBOOT:
// The system should be rebooted for HW protection.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_protection_action {

    void __hw_protection_trigger(const char *reason, int ms_until_forced,
    enum hw_protection_action action);

//
// hw_protection_trigger - Trigger default emergency system hardware protection action
//
// @reason:		Reason of emergency shutdown or reboot to be printed.
// @ms_until_forced:	Time to wait for orderly shutdown or reboot before
// triggering it. Negative value disables the forced
// shutdown or reboot.
//
// Initiate an emergency system shutdown or reboot in order to protect
// hardware from further damage. The exact action taken is controllable at
// runtime and defaults to shutdown.
//
    static inline void hw_protection_trigger(const char *reason, int ms_until_forced)
    {
    __hw_protection_trigger(reason, ms_until_forced, HWPROT_ACT_DEFAULT);
    }

//
// Emergency restart, callable from an interrupt handler.
//

    extern void emergency_restart(void);


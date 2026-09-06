//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/utglobal.c
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: utglobal - Global variables for the ACPI subsystem
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Macro flag: #define EXPORT_ACPI_INTERFACES
// Macro flag: #define DEFINE_ACPI_GLOBALS

    ACPI_MODULE_NAME("utglobal")
//
// Static global variable initialization.
//
// Various state name strings
    const char *acpi_gbl_sleep_state_names[ACPI_S_STATE_COUNT] = {
    "\\_S0_",
    "\\_S1_",
    "\\_S2_",
    "\\_S3_",
    "\\_S4_",
    "\\_S5_"
    };
    const char *acpi_gbl_lowest_dstate_names[ACPI_NUM_sx_w_METHODS] = {
    "_S0W",
    "_S1W",
    "_S2W",
    "_S3W",
    "_S4W"
    };
    const char *acpi_gbl_highest_dstate_names[ACPI_NUM_sx_d_METHODS] = {
    "_S1D",
    "_S2D",
    "_S3D",
    "_S4D"
    };
// Hex-to-ascii
    const char acpi_gbl_lower_hex_digits[] = "0123456789abcdef";
    const char acpi_gbl_upper_hex_digits[] = "0123456789ABCDEF";
//
// Namespace globals
//
// Predefined ACPI Names (Built-in to the Interpreter)
//
// NOTES:
// 1) _SB_ is defined to be a device to allow \_SB_._INI to be run
// during the initialization sequence.
// 2) _TZ_ is defined to be a thermal zone in order to allow ASL code to
// perform a Notify() operation on it. 09/2010: Changed to type Device.
// This still allows notifies, but does not confuse host code that
// searches for valid thermal_zone objects.
//
    const struct acpi_predefined_names acpi_gbl_pre_defined_names[] = {
    {"_GPE", ACPI_TYPE_LOCAL_SCOPE, core::ptr::null_mut()},
    {"_PR_", ACPI_TYPE_LOCAL_SCOPE, core::ptr::null_mut()},
    {"_SB_", ACPI_TYPE_DEVICE, core::ptr::null_mut()},
    {"_SI_", ACPI_TYPE_LOCAL_SCOPE, core::ptr::null_mut()},
    {"_TZ_", ACPI_TYPE_DEVICE, core::ptr::null_mut()},
//
// March, 2015:
// The _REV object is in the process of being deprecated, because
// other ACPI implementations permanently return 2. Thus, it
// has little or no value. Return 2 for compatibility with
// other ACPI implementations.
//
    {"_REV", ACPI_TYPE_INTEGER, ACPI_CAST_PTR(char, 2)},
    {"_OS_", ACPI_TYPE_STRING, ACPI_OS_NAME},
    {"_GL_", ACPI_TYPE_MUTEX, ACPI_CAST_PTR(char, 1)},
    {"_OSI", ACPI_TYPE_METHOD, ACPI_CAST_PTR(char, 1)},
// Table terminator
    {core::ptr::null_mut(), ACPI_TYPE_ANY, core::ptr::null_mut()}
    };

//
// Event and Hardware globals
//
    struct acpi_bit_register_info acpi_gbl_bit_register_info[ACPI_NUM_BITREG] = {
// Name                                     Parent Register             Register Bit Position                   Register Bit Mask
// ACPI_BITREG_TIMER_STATUS         */ {ACPI_REGISTER_PM1_STATUS,
    ACPI_BITPOSITION_TIMER_STATUS,
    ACPI_BITMASK_TIMER_STATUS},
// ACPI_BITREG_BUS_MASTER_STATUS    */ {ACPI_REGISTER_PM1_STATUS,
    ACPI_BITPOSITION_BUS_MASTER_STATUS,
    ACPI_BITMASK_BUS_MASTER_STATUS},
// ACPI_BITREG_GLOBAL_LOCK_STATUS   */ {ACPI_REGISTER_PM1_STATUS,
    ACPI_BITPOSITION_GLOBAL_LOCK_STATUS,
    ACPI_BITMASK_GLOBAL_LOCK_STATUS},
// ACPI_BITREG_POWER_BUTTON_STATUS  */ {ACPI_REGISTER_PM1_STATUS,
    ACPI_BITPOSITION_POWER_BUTTON_STATUS,
    ACPI_BITMASK_POWER_BUTTON_STATUS},
// ACPI_BITREG_SLEEP_BUTTON_STATUS  */ {ACPI_REGISTER_PM1_STATUS,
    ACPI_BITPOSITION_SLEEP_BUTTON_STATUS,
    ACPI_BITMASK_SLEEP_BUTTON_STATUS},
// ACPI_BITREG_RT_CLOCK_STATUS      */ {ACPI_REGISTER_PM1_STATUS,
    ACPI_BITPOSITION_RT_CLOCK_STATUS,
    ACPI_BITMASK_RT_CLOCK_STATUS},
// ACPI_BITREG_WAKE_STATUS          */ {ACPI_REGISTER_PM1_STATUS,
    ACPI_BITPOSITION_WAKE_STATUS,
    ACPI_BITMASK_WAKE_STATUS},
// ACPI_BITREG_PCIEXP_WAKE_STATUS   */ {ACPI_REGISTER_PM1_STATUS,
    ACPI_BITPOSITION_PCIEXP_WAKE_STATUS,
    ACPI_BITMASK_PCIEXP_WAKE_STATUS},
// ACPI_BITREG_TIMER_ENABLE         */ {ACPI_REGISTER_PM1_ENABLE,
    ACPI_BITPOSITION_TIMER_ENABLE,
    ACPI_BITMASK_TIMER_ENABLE},
// ACPI_BITREG_GLOBAL_LOCK_ENABLE   */ {ACPI_REGISTER_PM1_ENABLE,
    ACPI_BITPOSITION_GLOBAL_LOCK_ENABLE,
    ACPI_BITMASK_GLOBAL_LOCK_ENABLE},
// ACPI_BITREG_POWER_BUTTON_ENABLE  */ {ACPI_REGISTER_PM1_ENABLE,
    ACPI_BITPOSITION_POWER_BUTTON_ENABLE,
    ACPI_BITMASK_POWER_BUTTON_ENABLE},
// ACPI_BITREG_SLEEP_BUTTON_ENABLE  */ {ACPI_REGISTER_PM1_ENABLE,
    ACPI_BITPOSITION_SLEEP_BUTTON_ENABLE,
    ACPI_BITMASK_SLEEP_BUTTON_ENABLE},
// ACPI_BITREG_RT_CLOCK_ENABLE      */ {ACPI_REGISTER_PM1_ENABLE,
    ACPI_BITPOSITION_RT_CLOCK_ENABLE,
    ACPI_BITMASK_RT_CLOCK_ENABLE},
// ACPI_BITREG_PCIEXP_WAKE_DISABLE  */ {ACPI_REGISTER_PM1_ENABLE,
    ACPI_BITPOSITION_PCIEXP_WAKE_DISABLE,
    ACPI_BITMASK_PCIEXP_WAKE_DISABLE},
// ACPI_BITREG_SCI_ENABLE           */ {ACPI_REGISTER_PM1_CONTROL,
    ACPI_BITPOSITION_SCI_ENABLE,
    ACPI_BITMASK_SCI_ENABLE},
// ACPI_BITREG_BUS_MASTER_RLD       */ {ACPI_REGISTER_PM1_CONTROL,
    ACPI_BITPOSITION_BUS_MASTER_RLD,
    ACPI_BITMASK_BUS_MASTER_RLD},
// ACPI_BITREG_GLOBAL_LOCK_RELEASE  */ {ACPI_REGISTER_PM1_CONTROL,
    ACPI_BITPOSITION_GLOBAL_LOCK_RELEASE,
    ACPI_BITMASK_GLOBAL_LOCK_RELEASE},
// ACPI_BITREG_SLEEP_TYPE           */ {ACPI_REGISTER_PM1_CONTROL,
    ACPI_BITPOSITION_SLEEP_TYPE,
    ACPI_BITMASK_SLEEP_TYPE},
// ACPI_BITREG_SLEEP_ENABLE         */ {ACPI_REGISTER_PM1_CONTROL,
    ACPI_BITPOSITION_SLEEP_ENABLE,
    ACPI_BITMASK_SLEEP_ENABLE},
// ACPI_BITREG_ARB_DIS              */ {ACPI_REGISTER_PM2_CONTROL,
    ACPI_BITPOSITION_ARB_DISABLE,
    ACPI_BITMASK_ARB_DISABLE}
    };
    struct acpi_fixed_event_info acpi_gbl_fixed_event_info[ACPI_NUM_FIXED_EVENTS] = {
// ACPI_EVENT_PMTIMER       */ {ACPI_BITREG_TIMER_STATUS,
    ACPI_BITREG_TIMER_ENABLE,
    ACPI_BITMASK_TIMER_STATUS,
    ACPI_BITMASK_TIMER_ENABLE},
// ACPI_EVENT_GLOBAL        */ {ACPI_BITREG_GLOBAL_LOCK_STATUS,
    ACPI_BITREG_GLOBAL_LOCK_ENABLE,
    ACPI_BITMASK_GLOBAL_LOCK_STATUS,
    ACPI_BITMASK_GLOBAL_LOCK_ENABLE},
// ACPI_EVENT_POWER_BUTTON  */ {ACPI_BITREG_POWER_BUTTON_STATUS,
    ACPI_BITREG_POWER_BUTTON_ENABLE,
    ACPI_BITMASK_POWER_BUTTON_STATUS,
    ACPI_BITMASK_POWER_BUTTON_ENABLE},
// ACPI_EVENT_SLEEP_BUTTON  */ {ACPI_BITREG_SLEEP_BUTTON_STATUS,
    ACPI_BITREG_SLEEP_BUTTON_ENABLE,
    ACPI_BITMASK_SLEEP_BUTTON_STATUS,
    ACPI_BITMASK_SLEEP_BUTTON_ENABLE},
// ACPI_EVENT_RTC           */ {ACPI_BITREG_RT_CLOCK_STATUS,
    ACPI_BITREG_RT_CLOCK_ENABLE,
    ACPI_BITMASK_RT_CLOCK_STATUS,
    ACPI_BITMASK_RT_CLOCK_ENABLE},
    };

// to_pld macro: compile/disassemble strings
    const char *acpi_gbl_pld_panel_list[] = {
    "TOP",
    "BOTTOM",
    "LEFT",
    "RIGHT",
    "FRONT",
    "BACK",
    "UNKNOWN",
    core::ptr::null_mut()
    };
    const char *acpi_gbl_pld_vertical_position_list[] = {
    "UPPER",
    "CENTER",
    "LOWER",
    core::ptr::null_mut()
    };
    const char *acpi_gbl_pld_horizontal_position_list[] = {
    "LEFT",
    "CENTER",
    "RIGHT",
    core::ptr::null_mut()
    };
    const char *acpi_gbl_pld_shape_list[] = {
    "ROUND",
    "OVAL",
    "SQUARE",
    "VERTICALRECTANGLE",
    "HORIZONTALRECTANGLE",
    "VERTICALTRAPEZOID",
    "HORIZONTALTRAPEZOID",
    "UNKNOWN",
    "CHAMFERED",
    core::ptr::null_mut()
    };

// Public globals
    ACPI_EXPORT_SYMBOL(acpi_gbl_FADT)
    ACPI_EXPORT_SYMBOL(acpi_dbg_level)
    ACPI_EXPORT_SYMBOL(acpi_dbg_layer)
    ACPI_EXPORT_SYMBOL(acpi_gpe_count)
    ACPI_EXPORT_SYMBOL(acpi_current_gpe_count)

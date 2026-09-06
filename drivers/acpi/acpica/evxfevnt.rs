//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/evxfevnt.c
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
// Module Name: evxfevnt - External Interfaces, ACPI event disable/enable
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Macro flag: #define EXPORT_ACPI_INTERFACES

    ACPI_MODULE_NAME("evxfevnt")

//
// FUNCTION:    acpi_enable
//
// PARAMETERS:  None
//
// RETURN:      Status
//
// DESCRIPTION: Transfers the system into ACPI mode.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_enable() -> acpi_status {
    acpi_status acpi_enable(void)
    {
    acpi_status status;
    int retry;
    ACPI_FUNCTION_TRACE(acpi_enable);
// ACPI tables must be present
    if (acpi_gbl_fadt_index == ACPI_INVALID_TABLE_INDEX) {
    return_ACPI_STATUS(AE_NO_ACPI_TABLES);
    }
// If the Hardware Reduced flag is set, machine is always in acpi mode
    if (acpi_gbl_reduced_hardware) {
    return_ACPI_STATUS(AE_OK);
    }
// Check current mode
    if (acpi_hw_get_mode() == ACPI_SYS_MODE_ACPI) {
    ACPI_DEBUG_PRINT((ACPI_DB_INIT,
    "System is already in ACPI mode\n"));
    return_ACPI_STATUS(AE_OK);
    }
// Transition to ACPI mode
    status = acpi_hw_set_mode(ACPI_SYS_MODE_ACPI);
    if (ACPI_FAILURE(status)) {
    ACPI_ERROR((AE_INFO,
    "Could not transition to ACPI mode"));
    return_ACPI_STATUS(status);
    }
// Sanity check that transition succeeded
    for (retry = 0; retry < 30000; ++retry) {
    if (acpi_hw_get_mode() == ACPI_SYS_MODE_ACPI) {
    if (retry != 0)
    ACPI_WARNING((AE_INFO,
    "Platform took > %d00 usec to enter ACPI mode", retry));
    return_ACPI_STATUS(AE_OK);
    }
    acpi_os_stall(100);	/* 100 usec */
    }
    ACPI_ERROR((AE_INFO, "Hardware did not enter ACPI mode"));
    return_ACPI_STATUS(AE_NO_HARDWARE_RESPONSE);
    }
    ACPI_EXPORT_SYMBOL(acpi_enable)
//
// FUNCTION:    acpi_disable
//
// PARAMETERS:  None
//
// RETURN:      Status
//
// DESCRIPTION: Transfers the system into LEGACY (non-ACPI) mode.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_disable() -> acpi_status {
    acpi_status acpi_disable(void)
    {
    let mut status: acpi_status = AE_OK;
    ACPI_FUNCTION_TRACE(acpi_disable);
// If the Hardware Reduced flag is set, machine is always in acpi mode
    if (acpi_gbl_reduced_hardware) {
    return_ACPI_STATUS(AE_OK);
    }
    if (acpi_hw_get_mode() == ACPI_SYS_MODE_LEGACY) {
    ACPI_DEBUG_PRINT((ACPI_DB_INIT,
    "System is already in legacy (non-ACPI) mode\n"));
    } else {
// Transition to LEGACY mode
    status = acpi_hw_set_mode(ACPI_SYS_MODE_LEGACY);
    if (ACPI_FAILURE(status)) {
    ACPI_ERROR((AE_INFO,
    "Could not exit ACPI mode to legacy mode"));
    return_ACPI_STATUS(status);
    }
    ACPI_DEBUG_PRINT((ACPI_DB_INIT, "ACPI mode disabled\n"));
    }
    return_ACPI_STATUS(status);
    }
    ACPI_EXPORT_SYMBOL(acpi_disable)
//
// FUNCTION:    acpi_enable_event
//
// PARAMETERS:  event           - The fixed eventto be enabled
// flags           - Reserved
//
// RETURN:      Status
//
// DESCRIPTION: Enable an ACPI event (fixed)
//
#[no_mangle]
pub unsafe extern "C" fn acpi_enable_event(event: u32, flags: u32) -> acpi_status {
    acpi_status acpi_enable_event(u32 event, u32 flags)
    {
    let mut status: acpi_status = AE_OK;
    u32 value;
    ACPI_FUNCTION_TRACE(acpi_enable_event);
// If Hardware Reduced flag is set, there are no fixed events
    if (acpi_gbl_reduced_hardware) {
    return_ACPI_STATUS(AE_OK);
    }
// Decode the Fixed Event
    if (event > ACPI_EVENT_MAX) {
    return_ACPI_STATUS(AE_BAD_PARAMETER);
    }
//
// Enable the requested fixed event (by writing a one to the enable
// register bit)
//
    status =
    acpi_write_bit_register(acpi_gbl_fixed_event_info[event].
    enable_register_id, ACPI_ENABLE_EVENT);
    if (ACPI_FAILURE(status)) {
    return_ACPI_STATUS(status);
    }
// Make sure that the hardware responded
    status =
    acpi_read_bit_register(acpi_gbl_fixed_event_info[event].
    enable_register_id, &value);
    if (ACPI_FAILURE(status)) {
    return_ACPI_STATUS(status);
    }
    if (value != 1) {
    ACPI_ERROR((AE_INFO,
    "Could not enable %s event",
    acpi_ut_get_event_name(event)));
    return_ACPI_STATUS(AE_NO_HARDWARE_RESPONSE);
    }
    return_ACPI_STATUS(status);
    }
    ACPI_EXPORT_SYMBOL(acpi_enable_event)
//
// FUNCTION:    acpi_disable_event
//
// PARAMETERS:  event           - The fixed event to be disabled
// flags           - Reserved
//
// RETURN:      Status
//
// DESCRIPTION: Disable an ACPI event (fixed)
//
#[no_mangle]
pub unsafe extern "C" fn acpi_disable_event(event: u32, flags: u32) -> acpi_status {
    acpi_status acpi_disable_event(u32 event, u32 flags)
    {
    let mut status: acpi_status = AE_OK;
    u32 value;
    ACPI_FUNCTION_TRACE(acpi_disable_event);
// If Hardware Reduced flag is set, there are no fixed events
    if (acpi_gbl_reduced_hardware) {
    return_ACPI_STATUS(AE_OK);
    }
// Decode the Fixed Event
    if (event > ACPI_EVENT_MAX) {
    return_ACPI_STATUS(AE_BAD_PARAMETER);
    }
//
// Disable the requested fixed event (by writing a zero to the enable
// register bit)
//
    status =
    acpi_write_bit_register(acpi_gbl_fixed_event_info[event].
    enable_register_id, ACPI_DISABLE_EVENT);
    if (ACPI_FAILURE(status)) {
    return_ACPI_STATUS(status);
    }
    status =
    acpi_read_bit_register(acpi_gbl_fixed_event_info[event].
    enable_register_id, &value);
    if (ACPI_FAILURE(status)) {
    return_ACPI_STATUS(status);
    }
    if (value != 0) {
    ACPI_ERROR((AE_INFO,
    "Could not disable %s events",
    acpi_ut_get_event_name(event)));
    return_ACPI_STATUS(AE_NO_HARDWARE_RESPONSE);
    }
    return_ACPI_STATUS(status);
    }
    ACPI_EXPORT_SYMBOL(acpi_disable_event)
//
// FUNCTION:    acpi_clear_event
//
// PARAMETERS:  event           - The fixed event to be cleared
//
// RETURN:      Status
//
// DESCRIPTION: Clear an ACPI event (fixed)
//
#[no_mangle]
pub unsafe extern "C" fn acpi_clear_event(event: u32) -> acpi_status {
    acpi_status acpi_clear_event(u32 event)
    {
    let mut status: acpi_status = AE_OK;
    ACPI_FUNCTION_TRACE(acpi_clear_event);
// If Hardware Reduced flag is set, there are no fixed events
    if (acpi_gbl_reduced_hardware) {
    return_ACPI_STATUS(AE_OK);
    }
// Decode the Fixed Event
    if (event > ACPI_EVENT_MAX) {
    return_ACPI_STATUS(AE_BAD_PARAMETER);
    }
//
// Clear the requested fixed event (By writing a one to the status
// register bit)
//
    status =
    acpi_write_bit_register(acpi_gbl_fixed_event_info[event].
    status_register_id, ACPI_CLEAR_STATUS);
    return_ACPI_STATUS(status);
    }
    ACPI_EXPORT_SYMBOL(acpi_clear_event)
//
// FUNCTION:    acpi_get_event_status
//
// PARAMETERS:  event           - The fixed event
// event_status    - Where the current status of the event will
// be returned
//
// RETURN:      Status
//
// DESCRIPTION: Obtains and returns the current status of the event
//
#[no_mangle]
pub unsafe extern "C" fn acpi_get_event_status(event: u32, event_status: *mut *mut acpi_event_status) -> acpi_status {
    acpi_status acpi_get_event_status(u32 event, acpi_event_status * event_status)
    {
    acpi_status status;
    let mut local_event_status: acpi_event_status = 0;
    u32 in_byte;
    ACPI_FUNCTION_TRACE(acpi_get_event_status);
    if (!event_status) {
    return_ACPI_STATUS(AE_BAD_PARAMETER);
    }
// Decode the Fixed Event
    if (event > ACPI_EVENT_MAX) {
    return_ACPI_STATUS(AE_BAD_PARAMETER);
    }
// Fixed event currently can be dispatched?
    if (acpi_gbl_fixed_event_handlers[event].handler) {
    local_event_status |= ACPI_EVENT_FLAG_HAS_HANDLER;
    }
// Fixed event currently enabled?
    status =
    acpi_read_bit_register(acpi_gbl_fixed_event_info[event].
    enable_register_id, &in_byte);
    if (ACPI_FAILURE(status)) {
    return_ACPI_STATUS(status);
    }
    if (in_byte) {
    local_event_status |=
    (ACPI_EVENT_FLAG_ENABLED | ACPI_EVENT_FLAG_ENABLE_SET);
    }
// Fixed event currently active?
    status =
    acpi_read_bit_register(acpi_gbl_fixed_event_info[event].
    status_register_id, &in_byte);
    if (ACPI_FAILURE(status)) {
    return_ACPI_STATUS(status);
    }
    if (in_byte) {
    local_event_status |= ACPI_EVENT_FLAG_STATUS_SET;
    }
    (*event_status) = local_event_status;
    return_ACPI_STATUS(AE_OK);
    }
    ACPI_EXPORT_SYMBOL(acpi_get_event_status)

//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/evevent.c
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
// Module Name: evevent - Fixed Event handling and dispatch
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

    ACPI_MODULE_NAME("evevent")

// Local prototypes
    static acpi_status acpi_ev_fixed_event_initialize(void);
    static u32 acpi_ev_fixed_event_dispatch(u32 event);
//
// FUNCTION:    acpi_ev_initialize_events
//
// PARAMETERS:  None
//
// RETURN:      Status
//
// DESCRIPTION: Initialize global data structures for ACPI events (Fixed, GPE)
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ev_initialize_events() -> acpi_status {
    acpi_status acpi_ev_initialize_events(void)
    {
    acpi_status status;
    ACPI_FUNCTION_TRACE(ev_initialize_events);
// If Hardware Reduced flag is set, there are no fixed events
    if (acpi_gbl_reduced_hardware) {
    return_ACPI_STATUS(AE_OK);
    }
//
// Initialize the Fixed and General Purpose Events. This is done prior to
// enabling SCIs to prevent interrupts from occurring before the handlers
// are installed.
//
    status = acpi_ev_fixed_event_initialize();
    if (ACPI_FAILURE(status)) {
    ACPI_EXCEPTION((AE_INFO, status,
    "Unable to initialize fixed events"));
    return_ACPI_STATUS(status);
    }
    status = acpi_ev_gpe_initialize();
    if (ACPI_FAILURE(status)) {
    ACPI_EXCEPTION((AE_INFO, status,
    "Unable to initialize general purpose events"));
    return_ACPI_STATUS(status);
    }
    return_ACPI_STATUS(status);
    }
//
// FUNCTION:    acpi_ev_install_xrupt_handlers
//
// PARAMETERS:  None
//
// RETURN:      Status
//
// DESCRIPTION: Install interrupt handlers for the SCI and Global Lock
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ev_install_xrupt_handlers() -> acpi_status {
    acpi_status acpi_ev_install_xrupt_handlers(void)
    {
    acpi_status status;
    ACPI_FUNCTION_TRACE(ev_install_xrupt_handlers);
// If Hardware Reduced flag is set, there is no ACPI h/w
    if (acpi_gbl_reduced_hardware) {
    return_ACPI_STATUS(AE_OK);
    }
// Install the SCI handler
    status = acpi_ev_install_sci_handler();
    if (ACPI_FAILURE(status)) {
    ACPI_EXCEPTION((AE_INFO, status,
    "Unable to install System Control Interrupt handler"));
    return_ACPI_STATUS(status);
    }
// Install the handler for the Global Lock
    status = acpi_ev_init_global_lock_handler();
    if (ACPI_FAILURE(status)) {
    ACPI_EXCEPTION((AE_INFO, status,
    "Unable to initialize Global Lock handler"));
    return_ACPI_STATUS(status);
    }
    acpi_gbl_events_initialized = TRUE;
    return_ACPI_STATUS(status);
    }
//
// FUNCTION:    acpi_ev_fixed_event_initialize
//
// PARAMETERS:  None
//
// RETURN:      Status
//
// DESCRIPTION: Install the fixed event handlers and disable all fixed events.
//
#[no_mangle]
unsafe extern "C" fn acpi_ev_fixed_event_initialize() -> acpi_status {
    static acpi_status acpi_ev_fixed_event_initialize(void)
    {
    u32 i;
    acpi_status status;
//
// Initialize the structure that keeps track of fixed event handlers and
// disable all of the fixed events.
//
    for (i = 0; i < ACPI_NUM_FIXED_EVENTS; i++) {
    acpi_gbl_fixed_event_handlers[i].handler = core::ptr::null_mut();
    acpi_gbl_fixed_event_handlers[i].context = core::ptr::null_mut();
// Disable the fixed event
    if (acpi_gbl_fixed_event_info[i].enable_register_id != 0xFF) {
    status =
    acpi_write_bit_register(acpi_gbl_fixed_event_info
    [i].enable_register_id,
    ACPI_DISABLE_EVENT);
    if (ACPI_FAILURE(status)) {
    return (status);
    }
    }
    }
    return (AE_OK);
    }
//
// FUNCTION:    acpi_ev_fixed_event_detect
//
// PARAMETERS:  None
//
// RETURN:      INTERRUPT_HANDLED or INTERRUPT_NOT_HANDLED
//
// DESCRIPTION: Checks the PM status register for active fixed events
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ev_fixed_event_detect() -> u32 {
    u32 acpi_ev_fixed_event_detect(void)
    {
    let mut int_status: u32 = ACPI_INTERRUPT_NOT_HANDLED;
    u32 fixed_status;
    u32 fixed_enable;
    u32 i;
    acpi_status status;
    ACPI_FUNCTION_NAME(ev_fixed_event_detect);
//
// Read the fixed feature status and enable registers, as all the cases
// depend on their values. Ignore errors here.
//
    status = acpi_hw_register_read(ACPI_REGISTER_PM1_STATUS, &fixed_status);
    status |=
    acpi_hw_register_read(ACPI_REGISTER_PM1_ENABLE, &fixed_enable);
    if (ACPI_FAILURE(status)) {
    return (int_status);
    }
    ACPI_DEBUG_PRINT((ACPI_DB_INTERRUPTS,
    "Fixed Event Block: Enable %08X Status %08X\n",
    fixed_enable, fixed_status));
//
// Check for all possible Fixed Events and dispatch those that are active
//
    for (i = 0; i < ACPI_NUM_FIXED_EVENTS; i++) {
// Both the status and enable bits must be on for this event
    if ((fixed_status & acpi_gbl_fixed_event_info[i].
    status_bit_mask)
    && (fixed_enable & acpi_gbl_fixed_event_info[i].
    enable_bit_mask)) {
//
// Found an active (signalled) event. Invoke global event
// handler if present.
//
    acpi_fixed_event_count[i]++;
    if (acpi_gbl_global_event_handler) {
    acpi_gbl_global_event_handler
    (ACPI_EVENT_TYPE_FIXED, core::ptr::null_mut(), i,
    acpi_gbl_global_event_handler_context);
    }
    int_status |= acpi_ev_fixed_event_dispatch(i);
    }
    }
    return (int_status);
    }
//
// FUNCTION:    acpi_ev_fixed_event_dispatch
//
// PARAMETERS:  event               - Event type
//
// RETURN:      INTERRUPT_HANDLED or INTERRUPT_NOT_HANDLED
//
// DESCRIPTION: Clears the status bit for the requested event, calls the
// handler that previously registered for the event.
// NOTE: If there is no handler for the event, the event is
// disabled to prevent further interrupts.
//
#[no_mangle]
unsafe extern "C" fn acpi_ev_fixed_event_dispatch(event: u32) -> u32 {
    static u32 acpi_ev_fixed_event_dispatch(u32 event)
    {
    ACPI_FUNCTION_ENTRY();
// Clear the status bit
    (void)acpi_write_bit_register(acpi_gbl_fixed_event_info[event].
    status_register_id, ACPI_CLEAR_STATUS);
//
// Make sure that a handler exists. If not, report an error
// and disable the event to prevent further interrupts.
//
    if (!acpi_gbl_fixed_event_handlers[event].handler) {
    (void)acpi_write_bit_register(acpi_gbl_fixed_event_info[event].
    enable_register_id,
    ACPI_DISABLE_EVENT);
    ACPI_ERROR((AE_INFO,
    "No installed handler for fixed event - %s (%u), disabling",
    acpi_ut_get_event_name(event), event));
    return (ACPI_INTERRUPT_NOT_HANDLED);
    }
// Invoke the Fixed Event handler
    return ((acpi_gbl_fixed_event_handlers[event].
    handler) (acpi_gbl_fixed_event_handlers[event].context));
    }
//
// FUNCTION:    acpi_any_fixed_event_status_set
//
// PARAMETERS:  None
//
// RETURN:      TRUE or FALSE
//
// DESCRIPTION: Checks the PM status register for active fixed events
//
#[no_mangle]
pub unsafe extern "C" fn acpi_any_fixed_event_status_set() -> u32 {
    u32 acpi_any_fixed_event_status_set(void)
    {
    acpi_status status;
    u32 in_status;
    u32 in_enable;
    u32 i;
    status = acpi_hw_register_read(ACPI_REGISTER_PM1_ENABLE, &in_enable);
    if (ACPI_FAILURE(status)) {
    return (FALSE);
    }
    status = acpi_hw_register_read(ACPI_REGISTER_PM1_STATUS, &in_status);
    if (ACPI_FAILURE(status)) {
    return (FALSE);
    }
//
// Check for all possible Fixed Events and dispatch those that are active
//
    for (i = 0; i < ACPI_NUM_FIXED_EVENTS; i++) {
// Both the status and enable bits must be on for this event
    if ((in_status & acpi_gbl_fixed_event_info[i].status_bit_mask) &&
    (in_enable & acpi_gbl_fixed_event_info[i].enable_bit_mask)) {
    return (TRUE);
    }
    }
    return (FALSE);
    }

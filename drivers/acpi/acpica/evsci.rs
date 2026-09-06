//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/evsci.c
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
// Module Name: evsci - System Control Interrupt configuration and
// legacy to ACPI mode state transition functions
//

    ACPI_MODULE_NAME("evsci")

// Local prototypes
    static u32 ACPI_SYSTEM_XFACE acpi_ev_sci_xrupt_handler(void *context);
//
// FUNCTION:    acpi_ev_sci_dispatch
//
// PARAMETERS:  None
//
// RETURN:      Status code indicates whether interrupt was handled.
//
// DESCRIPTION: Dispatch the SCI to all host-installed SCI handlers.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ev_sci_dispatch() -> u32 {
    u32 acpi_ev_sci_dispatch(void)
    {
    struct acpi_sci_handler_info *sci_handler;
    acpi_cpu_flags flags;
    let mut int_status: u32 = ACPI_INTERRUPT_NOT_HANDLED;
    ACPI_FUNCTION_NAME(ev_sci_dispatch);
// Are there any host-installed SCI handlers?
    if (!acpi_gbl_sci_handler_list) {
    return (int_status);
    }
    flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);
// Invoke all host-installed SCI handlers
    sci_handler = acpi_gbl_sci_handler_list;
    while (sci_handler) {
// Invoke the installed handler (at interrupt level)
    int_status |= sci_handler.address(sci_handler.context);
    sci_handler = sci_handler.next;
    }
    acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
    return (int_status);
    }
//
// FUNCTION:    acpi_ev_sci_xrupt_handler
//
// PARAMETERS:  context   - Calling Context
//
// RETURN:      Status code indicates whether interrupt was handled.
//
// DESCRIPTION: Interrupt handler that will figure out what function or
// control method to call to deal with a SCI.
//
#[no_mangle]
unsafe extern "C" fn acpi_ev_sci_xrupt_handler(context: *mut c_void) -> u32 ACPI_SYSTEM_XFACE {
    static u32 ACPI_SYSTEM_XFACE acpi_ev_sci_xrupt_handler(void *context)
    {
    struct acpi_gpe_xrupt_info *gpe_xrupt_list = context;
    let mut interrupt_handled: u32 = ACPI_INTERRUPT_NOT_HANDLED;
    ACPI_FUNCTION_TRACE(ev_sci_xrupt_handler);
//
// We are guaranteed by the ACPICA initialization/shutdown code that
// if this interrupt handler is installed, ACPI is enabled.
//
// Fixed Events:
// Check for and dispatch any Fixed Events that have occurred
//
    interrupt_handled |= acpi_ev_fixed_event_detect();
//
// General Purpose Events:
// Check for and dispatch any GPEs that have occurred
//
    interrupt_handled |= acpi_ev_gpe_detect(gpe_xrupt_list);
// Invoke all host-installed SCI handlers
    interrupt_handled |= acpi_ev_sci_dispatch();
    acpi_sci_count++;
    return_UINT32(interrupt_handled);
    }
//
// FUNCTION:    acpi_ev_gpe_xrupt_handler
//
// PARAMETERS:  context   - Calling Context
//
// RETURN:      Status code indicates whether interrupt was handled.
//
// DESCRIPTION: Handler for GPE Block Device interrupts
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ev_gpe_xrupt_handler(context: *mut c_void) -> u32 ACPI_SYSTEM_XFACE {
    u32 ACPI_SYSTEM_XFACE acpi_ev_gpe_xrupt_handler(void *context)
    {
    struct acpi_gpe_xrupt_info *gpe_xrupt_list = context;
    let mut interrupt_handled: u32 = ACPI_INTERRUPT_NOT_HANDLED;
    ACPI_FUNCTION_TRACE(ev_gpe_xrupt_handler);
//
// We are guaranteed by the ACPICA initialization/shutdown code that
// if this interrupt handler is installed, ACPI is enabled.
//
// GPEs: Check for and dispatch any GPEs that have occurred
    interrupt_handled |= acpi_ev_gpe_detect(gpe_xrupt_list);
    return_UINT32(interrupt_handled);
    }
//
// FUNCTION:    acpi_ev_install_sci_handler
//
// PARAMETERS:  none
//
// RETURN:      Status
//
// DESCRIPTION: Installs SCI handler.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ev_install_sci_handler() -> u32 {
    u32 acpi_ev_install_sci_handler(void)
    {
    let mut status: u32 = AE_OK;
    ACPI_FUNCTION_TRACE(ev_install_sci_handler);
    status =
    acpi_os_install_interrupt_handler((u32) acpi_gbl_FADT.sci_interrupt,
    acpi_ev_sci_xrupt_handler,
    acpi_gbl_gpe_xrupt_list_head);
    return_ACPI_STATUS(status);
    }
//
// FUNCTION:    acpi_ev_remove_all_sci_handlers
//
// PARAMETERS:  none
//
// RETURN:      AE_OK if handler uninstalled, AE_ERROR if handler was not
// installed to begin with
//
// DESCRIPTION: Remove the SCI interrupt handler. No further SCIs will be
// taken. Remove all host-installed SCI handlers.
//
// Note:  It doesn't seem important to disable all events or set the event
// enable registers to their original values. The OS should disable
// the SCI interrupt level when the handler is removed, so no more
// events will come in.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ev_remove_all_sci_handlers() -> acpi_status {
    acpi_status acpi_ev_remove_all_sci_handlers(void)
    {
    struct acpi_sci_handler_info *sci_handler;
    acpi_cpu_flags flags;
    acpi_status status;
    ACPI_FUNCTION_TRACE(ev_remove_all_sci_handlers);
// Just let the OS remove the handler and disable the level
    status =
    acpi_os_remove_interrupt_handler((u32) acpi_gbl_FADT.sci_interrupt,
    acpi_ev_sci_xrupt_handler);
    if (!acpi_gbl_sci_handler_list) {
    return (status);
    }
    flags = acpi_os_acquire_lock(acpi_gbl_gpe_lock);
// Free all host-installed SCI handlers
    while (acpi_gbl_sci_handler_list) {
    sci_handler = acpi_gbl_sci_handler_list;
    acpi_gbl_sci_handler_list = sci_handler.next;
    ACPI_FREE(sci_handler);
    }
    acpi_os_release_lock(acpi_gbl_gpe_lock, flags);
    return_ACPI_STATUS(status);
    }

//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/evmisc.c
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
// Module Name: evmisc - Miscellaneous event manager support functions
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

    ACPI_MODULE_NAME("evmisc")
// Local prototypes
    static void ACPI_SYSTEM_XFACE acpi_ev_notify_dispatch(void *context);
//
// FUNCTION:    acpi_ev_is_notify_object
//
// PARAMETERS:  node            - Node to check
//
// RETURN:      TRUE if notifies allowed on this object
//
// DESCRIPTION: Check type of node for a object that supports notifies.
//
// TBD: This could be replaced by a flag bit in the node.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ev_is_notify_object(node: *mut acpi_namespace_node) -> u8 {
    u8 acpi_ev_is_notify_object(struct acpi_namespace_node *node)
    {
    switch (node.type) {
    case ACPI_TYPE_DEVICE:
    case ACPI_TYPE_PROCESSOR:
    case ACPI_TYPE_THERMAL:
//
// These are the ONLY objects that can receive ACPI notifications
//
    return (TRUE);
    default:
    return (FALSE);
    }
    }
//
// FUNCTION:    acpi_ev_queue_notify_request
//
// PARAMETERS:  node            - NS node for the notified object
// notify_value    - Value from the Notify() request
//
// RETURN:      Status
//
// DESCRIPTION: Dispatch a device notification event to a previously
// installed handler.
//
    acpi_status
    acpi_ev_queue_notify_request(struct acpi_namespace_node *node, u32 notify_value)
    {
    union acpi_operand_object *obj_desc;
    union acpi_operand_object *handler_list_head = core::ptr::null_mut();
    union acpi_generic_state *info;
    let mut handler_list_id: u8 = 0;
    let mut status: acpi_status = AE_OK;
    ACPI_FUNCTION_NAME(ev_queue_notify_request);
// Are Notifies allowed on this object?
    if (!acpi_ev_is_notify_object(node)) {
    return (AE_TYPE);
    }
// Get the correct notify list type (System or Device)
    if (notify_value <= ACPI_MAX_SYS_NOTIFY) {
    handler_list_id = ACPI_SYSTEM_HANDLER_LIST;
    } else {
    handler_list_id = ACPI_DEVICE_HANDLER_LIST;
    }
// Get the notify object attached to the namespace Node
    obj_desc = acpi_ns_get_attached_object(node);
    if (obj_desc) {
// We have an attached object, Get the correct handler list
    handler_list_head =
    obj_desc.common_notify.notify_list[handler_list_id];
    }
//
// If there is no notify handler (Global or Local)
// for this object, just ignore the notify
//
    if (!acpi_gbl_global_notify[handler_list_id].handler
    && !handler_list_head) {
    ACPI_DEBUG_PRINT((ACPI_DB_INFO,
    "No notify handler for Notify, ignoring (%4.4s, %X) node %p\n",
    acpi_ut_get_node_name(node), notify_value,
    node));
    return (AE_OK);
    }
// Setup notify info and schedule the notify dispatcher
    info = acpi_ut_create_generic_state();
    if (!info) {
    return (AE_NO_MEMORY);
    }
    info.common.descriptor_type = ACPI_DESC_TYPE_STATE_NOTIFY;
    info.notify.node = node;
    info.notify.value = (u16)notify_value;
    info.notify.handler_list_id = handler_list_id;
    info.notify.handler_list_head = handler_list_head;
    info.notify.global = &acpi_gbl_global_notify[handler_list_id];
    ACPI_DEBUG_PRINT((ACPI_DB_INFO,
    "Dispatching Notify on [%4.4s] (%s) Value 0x%2.2X (%s) Node %p\n",
    acpi_ut_get_node_name(node),
    acpi_ut_get_type_name(node.type), notify_value,
    acpi_ut_get_notify_name(notify_value, ACPI_TYPE_ANY),
    node));
    status = acpi_os_execute(OSL_NOTIFY_HANDLER,
    acpi_ev_notify_dispatch, info);
    if (ACPI_FAILURE(status)) {
    acpi_ut_delete_generic_state(info);
    }
    return (status);
    }
//
// FUNCTION:    acpi_ev_notify_dispatch
//
// PARAMETERS:  context         - To be passed to the notify handler
//
// RETURN:      None.
//
// DESCRIPTION: Dispatch a device notification event to a previously
// installed handler.
//
#[no_mangle]
unsafe extern "C" fn acpi_ev_notify_dispatch(context: *mut c_void) -> void ACPI_SYSTEM_XFACE {
    static void ACPI_SYSTEM_XFACE acpi_ev_notify_dispatch(void *context)
    {
    union acpi_generic_state *info = (union acpi_generic_state *)context;
    union acpi_operand_object *handler_obj;
    ACPI_FUNCTION_ENTRY();
// Invoke a global notify handler if installed
    if (info.notify.global.handler) {
    info.notify.global.handler(info.notify.node,
    info.notify.value,
    info.notify.global.context);
    }
// Now invoke the local notify handler(s) if any are installed
    handler_obj = info.notify.handler_list_head;
    while (handler_obj) {
    handler_obj.notify.handler(info.notify.node,
    info.notify.value,
    handler_obj.notify.context);
    handler_obj =
    handler_obj.notify.next[info.notify.handler_list_id];
    }
// All done with the info object
    acpi_ut_delete_generic_state(info);
    }

//
// FUNCTION:    acpi_ev_terminate
//
// PARAMETERS:  none
//
// RETURN:      none
//
// DESCRIPTION: Disable events and free memory allocated for table storage.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ev_terminate() {
    void acpi_ev_terminate(void)
    {
    u32 i;
    acpi_status status;
    ACPI_FUNCTION_TRACE(ev_terminate);
    if (acpi_gbl_events_initialized) {
//
// Disable all event-related functionality. In all cases, on error,
// print a message but obviously we don't abort.
//
// Disable all fixed events
    for (i = 0; i < ACPI_NUM_FIXED_EVENTS; i++) {
    status = acpi_disable_event(i, 0);
    if (ACPI_FAILURE(status)) {
    ACPI_ERROR((AE_INFO,
    "Could not disable fixed event %u",
    (u32) i));
    }
    }
// Disable all GPEs in all GPE blocks
    status = acpi_ev_walk_gpe_list(acpi_hw_disable_gpe_block, core::ptr::null_mut());
    if (ACPI_FAILURE(status)) {
    ACPI_EXCEPTION((AE_INFO, status,
    "Could not disable GPEs in GPE block"));
    }
    status = acpi_ev_remove_global_lock_handler();
    if (ACPI_FAILURE(status)) {
    ACPI_EXCEPTION((AE_INFO, status,
    "Could not remove Global Lock handler"));
    }
    acpi_gbl_events_initialized = FALSE;
    }
// Remove SCI handlers
    status = acpi_ev_remove_all_sci_handlers();
    if (ACPI_FAILURE(status)) {
    ACPI_ERROR((AE_INFO, "Could not remove SCI handler"));
    }
// Deallocate all handler objects installed within GPE info structs
    status = acpi_ev_walk_gpe_list(acpi_ev_delete_gpe_handlers, core::ptr::null_mut());
    if (ACPI_FAILURE(status)) {
    ACPI_EXCEPTION((AE_INFO, status,
    "Could not delete GPE handlers"));
    }
// Return to original mode if necessary
    if (acpi_gbl_original_mode == ACPI_SYS_MODE_LEGACY) {
    status = acpi_disable();
    if (ACPI_FAILURE(status)) {
    ACPI_WARNING((AE_INFO, "AcpiDisable failed"));
    }
    }
    return_VOID;
    }

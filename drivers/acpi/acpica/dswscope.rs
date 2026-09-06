//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/dswscope.c
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
// Module Name: dswscope - Scope stack manipulation
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

    ACPI_MODULE_NAME("dswscope")
//
// FUNCTION:    acpi_ds_scope_stack_clear
//
// PARAMETERS:  walk_state      - Current state
//
// RETURN:      None
//
// DESCRIPTION: Pop (and free) everything on the scope stack except the
// root scope object (which remains at the stack top.)
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ds_scope_stack_clear(walk_state: *mut acpi_walk_state) {
    void acpi_ds_scope_stack_clear(struct acpi_walk_state *walk_state)
    {
    union acpi_generic_state *scope_info;
    ACPI_FUNCTION_NAME(ds_scope_stack_clear);
    while (walk_state.scope_info) {
// Pop a scope off the stack
    scope_info = walk_state.scope_info;
    walk_state.scope_info = scope_info.scope.next;
    ACPI_DEBUG_PRINT((ACPI_DB_EXEC,
    "Popped object type (%s)\n",
    acpi_ut_get_type_name(scope_info.common.
    value)));
    acpi_ut_delete_generic_state(scope_info);
    }
    }
//
// FUNCTION:    acpi_ds_scope_stack_push
//
// PARAMETERS:  node            - Name to be made current
// type            - Type of frame being pushed
// walk_state      - Current state
//
// RETURN:      Status
//
// DESCRIPTION: Push the current scope on the scope stack, and make the
// passed Node current.
//
    acpi_status
    acpi_ds_scope_stack_push(struct acpi_namespace_node *node,
    acpi_object_type type,
    struct acpi_walk_state *walk_state)
    {
    union acpi_generic_state *scope_info;
    union acpi_generic_state *old_scope_info;
    ACPI_FUNCTION_TRACE(ds_scope_stack_push);
    if (!node) {
// Invalid scope
    ACPI_ERROR((AE_INFO, "Null scope parameter"));
    return_ACPI_STATUS(AE_BAD_PARAMETER);
    }
// Make sure object type is valid
    if (!acpi_ut_valid_object_type(type)) {
    ACPI_WARNING((AE_INFO, "Invalid object type: 0x%X", type));
    }
// Allocate a new scope object
    scope_info = acpi_ut_create_generic_state();
    if (!scope_info) {
    return_ACPI_STATUS(AE_NO_MEMORY);
    }
// Init new scope object
    scope_info.common.descriptor_type = ACPI_DESC_TYPE_STATE_WSCOPE;
    scope_info.scope.node = node;
    scope_info.common.value = (u16) type;
    walk_state.scope_depth++;
    ACPI_DEBUG_PRINT((ACPI_DB_EXEC,
    "[%.2d] Pushed scope ",
    (u32) walk_state.scope_depth));
    old_scope_info = walk_state.scope_info;
    if (old_scope_info) {
    ACPI_DEBUG_PRINT_RAW((ACPI_DB_EXEC,
    "[%4.4s] (%s)",
    acpi_ut_get_node_name(old_scope_info.
    scope.node),
    acpi_ut_get_type_name(old_scope_info.
    common.value)));
    } else {
    ACPI_DEBUG_PRINT_RAW((ACPI_DB_EXEC, ACPI_NAMESPACE_ROOT));
    }
    ACPI_DEBUG_PRINT_RAW((ACPI_DB_EXEC,
    ", New scope . [%4.4s] (%s)\n",
    acpi_ut_get_node_name(scope_info.scope.node),
    acpi_ut_get_type_name(scope_info.common.value)));
// Push new scope object onto stack
    acpi_ut_push_generic_state(&walk_state.scope_info, scope_info);
    return_ACPI_STATUS(AE_OK);
    }
//
// FUNCTION:    acpi_ds_scope_stack_pop
//
// PARAMETERS:  walk_state      - Current state
//
// RETURN:      Status
//
// DESCRIPTION: Pop the scope stack once.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ds_scope_stack_pop(walk_state: *mut acpi_walk_state) -> acpi_status {
    acpi_status acpi_ds_scope_stack_pop(struct acpi_walk_state *walk_state)
    {
    union acpi_generic_state *scope_info;
    union acpi_generic_state *new_scope_info;
    ACPI_FUNCTION_TRACE(ds_scope_stack_pop);
//
// Pop scope info object off the stack.
//
    scope_info = acpi_ut_pop_generic_state(&walk_state.scope_info);
    if (!scope_info) {
    return_ACPI_STATUS(AE_STACK_UNDERFLOW);
    }
    walk_state.scope_depth--;
    ACPI_DEBUG_PRINT((ACPI_DB_EXEC,
    "[%.2d] Popped scope [%4.4s] (%s), New scope . ",
    (u32) walk_state.scope_depth,
    acpi_ut_get_node_name(scope_info.scope.node),
    acpi_ut_get_type_name(scope_info.common.value)));
    new_scope_info = walk_state.scope_info;
    if (new_scope_info) {
    ACPI_DEBUG_PRINT_RAW((ACPI_DB_EXEC, "[%4.4s] (%s)\n",
    acpi_ut_get_node_name(new_scope_info.
    scope.node),
    acpi_ut_get_type_name(new_scope_info.
    common.value)));
    } else {
    ACPI_DEBUG_PRINT_RAW((ACPI_DB_EXEC, "%s\n",
    ACPI_NAMESPACE_ROOT));
    }
    acpi_ut_delete_generic_state(scope_info);
    return_ACPI_STATUS(AE_OK);
    }

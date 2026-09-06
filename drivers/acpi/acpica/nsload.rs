//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/nsload.c
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
// Module Name: nsload - namespace loading/expanding/contracting procedures
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

    ACPI_MODULE_NAME("nsload")
// Local prototypes

    acpi_status acpi_ns_unload_namespace(acpi_handle handle);
    static acpi_status acpi_ns_delete_subtree(acpi_handle start_handle);

//
// FUNCTION:    acpi_ns_load_table
//
// PARAMETERS:  table_index     - Index for table to be loaded
// node            - Owning NS node
//
// RETURN:      Status
//
// DESCRIPTION: Load one ACPI table into the namespace
//
    acpi_status
    acpi_ns_load_table(u32 table_index, struct acpi_namespace_node *node)
    {
    acpi_status status;
    ACPI_FUNCTION_TRACE(ns_load_table);
// If table already loaded into namespace, just return
    if (acpi_tb_is_table_loaded(table_index)) {
    status = AE_ALREADY_EXISTS;
    goto unlock;
    }
    ACPI_DEBUG_PRINT((ACPI_DB_INFO,
    "**** Loading table into namespace ****\n"));
    status = acpi_tb_allocate_owner_id(table_index);
    if (ACPI_FAILURE(status)) {
    goto unlock;
    }
//
// Parse the table and load the namespace with all named
// objects found within. Control methods are NOT parsed
// at this time. In fact, the control methods cannot be
// parsed until the entire namespace is loaded, because
// if a control method makes a forward reference (call)
// to another control method, we can't continue parsing
// because we don't know how many arguments to parse next!
//
    status = acpi_ns_parse_table(table_index, node);
    if (ACPI_SUCCESS(status)) {
    acpi_tb_set_table_loaded_flag(table_index, TRUE);
    } else {
//
// On error, delete any namespace objects created by this table.
// We cannot initialize these objects, so delete them. There are
// a couple of especially bad cases:
// AE_ALREADY_EXISTS - namespace collision.
// AE_NOT_FOUND - the target of a Scope operator does not
// exist. This target of Scope must already exist in the
// namespace, as per the ACPI specification.
//
    acpi_ns_delete_namespace_by_owner(acpi_gbl_root_table_list.
    tables[table_index].owner_id);
    acpi_tb_release_owner_id(table_index);
    return_ACPI_STATUS(status);
    }
    unlock:
    if (ACPI_FAILURE(status)) {
    return_ACPI_STATUS(status);
    }
//
// Now we can parse the control methods. We always parse
// them here for a sanity check, and if configured for
// just-in-time parsing, we delete the control method
// parse trees.
//
    ACPI_DEBUG_PRINT((ACPI_DB_INFO,
    "**** Begin Table Object Initialization\n"));
    acpi_ex_enter_interpreter();
    status = acpi_ds_initialize_objects(table_index, node);
    acpi_ex_exit_interpreter();
    ACPI_DEBUG_PRINT((ACPI_DB_INFO,
    "**** Completed Table Object Initialization\n"));
    return_ACPI_STATUS(status);
    }

//
// FUNCTION:    acpi_load_namespace
//
// PARAMETERS:  None
//
// RETURN:      Status
//
// DESCRIPTION: Load the name space from what ever is pointed to by DSDT.
// (DSDT points to either the BIOS or a buffer.)
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ns_load_namespace() -> acpi_status {
    acpi_status acpi_ns_load_namespace(void)
    {
    acpi_status status;
    ACPI_FUNCTION_TRACE(acpi_load_name_space);
// There must be at least a DSDT installed
    if (acpi_gbl_DSDT == core::ptr::null_mut()) {
    ACPI_ERROR((AE_INFO, "DSDT is not in memory"));
    return_ACPI_STATUS(AE_NO_ACPI_TABLES);
    }
//
// Load the namespace. The DSDT is required,
// but the SSDT and PSDT tables are optional.
//
    status = acpi_ns_load_table_by_type(ACPI_TABLE_ID_DSDT);
    if (ACPI_FAILURE(status)) {
    return_ACPI_STATUS(status);
    }
// Ignore exceptions from these
    (void)acpi_ns_load_table_by_type(ACPI_TABLE_ID_SSDT);
    (void)acpi_ns_load_table_by_type(ACPI_TABLE_ID_PSDT);
    ACPI_DEBUG_PRINT_RAW((ACPI_DB_INIT,
    "ACPI Namespace successfully loaded at root %p\n",
    acpi_gbl_root_node));
    return_ACPI_STATUS(status);
    }

//
// FUNCTION:    acpi_ns_delete_subtree
//
// PARAMETERS:  start_handle        - Handle in namespace where search begins
//
// RETURNS      Status
//
// DESCRIPTION: Walks the namespace starting at the given handle and deletes
// all objects, entries, and scopes in the entire subtree.
//
// Namespace/Interpreter should be locked or the subsystem should
// be in shutdown before this routine is called.
//
#[no_mangle]
unsafe extern "C" fn acpi_ns_delete_subtree(start_handle: acpi_handle) -> acpi_status {
    static acpi_status acpi_ns_delete_subtree(acpi_handle start_handle)
    {
    acpi_status status;
    acpi_handle child_handle;
    acpi_handle parent_handle;
    acpi_handle next_child_handle;
    acpi_handle dummy;
    u32 level;
    ACPI_FUNCTION_TRACE(ns_delete_subtree);
    parent_handle = start_handle;
    child_handle = core::ptr::null_mut();
    level = 1;
//
// Traverse the tree of objects until we bubble back up
// to where we started.
//
    while (level > 0) {
// Attempt to get the next object in this scope
    status = acpi_get_next_object(ACPI_TYPE_ANY, parent_handle,
    child_handle, &next_child_handle);
    child_handle = next_child_handle;
// Did we get a new object?
    if (ACPI_SUCCESS(status)) {
// Check if this object has any children
    if (ACPI_SUCCESS
    (acpi_get_next_object
    (ACPI_TYPE_ANY, child_handle, core::ptr::null_mut(), &dummy))) {
//
// There is at least one child of this object,
// visit the object
//
    level++;
    parent_handle = child_handle;
    child_handle = core::ptr::null_mut();
    }
    } else {
//
// No more children in this object, go back up to
// the object's parent
//
    level--;
// Delete all children now
    acpi_ns_delete_children(child_handle);
    child_handle = parent_handle;
    status = acpi_get_parent(parent_handle, &parent_handle);
    if (ACPI_FAILURE(status)) {
    return_ACPI_STATUS(status);
    }
    }
    }
// Now delete the starting object, and we are done
    acpi_ns_remove_node(child_handle);
    return_ACPI_STATUS(AE_OK);
    }
//
// FUNCTION:       acpi_ns_unload_name_space
//
// PARAMETERS:     handle          - Root of namespace subtree to be deleted
//
// RETURN:         Status
//
// DESCRIPTION:    Shrinks the namespace, typically in response to an undocking
// event. Deletes an entire subtree starting from (and
// including) the given handle.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ns_unload_namespace(handle: acpi_handle) -> acpi_status {
    acpi_status acpi_ns_unload_namespace(acpi_handle handle)
    {
    acpi_status status;
    ACPI_FUNCTION_TRACE(ns_unload_name_space);
// Parameter validation
    if (!acpi_gbl_root_node) {
    return_ACPI_STATUS(AE_NO_NAMESPACE);
    }
    if (!handle) {
    return_ACPI_STATUS(AE_BAD_PARAMETER);
    }
// This function does the real work
    status = acpi_ns_delete_subtree(handle);
    return_ACPI_STATUS(status);
    }

//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/utinit.c
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
// Module Name: utinit - Common ACPI subsystem initialization
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

    ACPI_MODULE_NAME("utinit")
// Local prototypes
    static void acpi_ut_terminate(void);

    static void acpi_ut_free_gpe_lists(void);

// Macro flag: #define acpi_ut_free_gpe_lists()

//
// FUNCTION:    acpi_ut_free_gpe_lists
//
// PARAMETERS:  none
//
// RETURN:      none
//
// DESCRIPTION: Free global GPE lists
//
#[no_mangle]
unsafe extern "C" fn acpi_ut_free_gpe_lists() {
    static void acpi_ut_free_gpe_lists(void)
    {
    struct acpi_gpe_block_info *gpe_block;
    struct acpi_gpe_block_info *next_gpe_block;
    struct acpi_gpe_xrupt_info *gpe_xrupt_info;
    struct acpi_gpe_xrupt_info *next_gpe_xrupt_info;
// Free global GPE blocks and related info structures
    gpe_xrupt_info = acpi_gbl_gpe_xrupt_list_head;
    while (gpe_xrupt_info) {
    gpe_block = gpe_xrupt_info.gpe_block_list_head;
    while (gpe_block) {
    next_gpe_block = gpe_block.next;
    ACPI_FREE(gpe_block.event_info);
    ACPI_FREE(gpe_block.register_info);
    ACPI_FREE(gpe_block);
    gpe_block = next_gpe_block;
    }
    next_gpe_xrupt_info = gpe_xrupt_info.next;
    ACPI_FREE(gpe_xrupt_info);
    gpe_xrupt_info = next_gpe_xrupt_info;
    }
    }

//
// FUNCTION:    acpi_ut_init_globals
//
// PARAMETERS:  None
//
// RETURN:      Status
//
// DESCRIPTION: Initialize ACPICA globals. All globals that require specific
// initialization should be initialized here. This allows for
// a warm restart.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_init_globals() -> acpi_status {
    acpi_status acpi_ut_init_globals(void)
    {
    acpi_status status;
    u32 i;
    ACPI_FUNCTION_TRACE(ut_init_globals);
// Create all memory caches
    status = acpi_ut_create_caches();
    if (ACPI_FAILURE(status)) {
    return_ACPI_STATUS(status);
    }
// Address Range lists
    for (i = 0; i < ACPI_ADDRESS_RANGE_MAX; i++) {
    acpi_gbl_address_range_list[i] = core::ptr::null_mut();
    }
// Mutex locked flags
    for (i = 0; i < ACPI_NUM_MUTEX; i++) {
    acpi_gbl_mutex_info[i].mutex = core::ptr::null_mut();
    acpi_gbl_mutex_info[i].thread_id = ACPI_MUTEX_NOT_ACQUIRED;
    acpi_gbl_mutex_info[i].use_count = 0;
    }
    for (i = 0; i < ACPI_NUM_OWNERID_MASKS; i++) {
    acpi_gbl_owner_id_mask[i] = 0;
    }
// Last owner_ID is never valid
    acpi_gbl_owner_id_mask[ACPI_NUM_OWNERID_MASKS - 1] = 0x80000000;
// Event counters
    acpi_method_count = 0;
    acpi_sci_count = 0;
    acpi_gpe_count = 0;
    for (i = 0; i < ACPI_NUM_FIXED_EVENTS; i++) {
    acpi_fixed_event_count[i] = 0;
    }

// GPE/SCI support
    acpi_gbl_all_gpes_initialized = FALSE;
    acpi_gbl_gpe_xrupt_list_head = core::ptr::null_mut();
    acpi_gbl_gpe_fadt_blocks[0] = core::ptr::null_mut();
    acpi_gbl_gpe_fadt_blocks[1] = core::ptr::null_mut();
    acpi_current_gpe_count = 0;
    acpi_gbl_global_event_handler = core::ptr::null_mut();
    acpi_gbl_sci_handler_list = core::ptr::null_mut();

// Global handlers
    acpi_gbl_global_notify[0].handler = core::ptr::null_mut();
    acpi_gbl_global_notify[1].handler = core::ptr::null_mut();
    acpi_gbl_exception_handler = core::ptr::null_mut();
    acpi_gbl_init_handler = core::ptr::null_mut();
    acpi_gbl_table_handler = core::ptr::null_mut();
    acpi_gbl_interface_handler = core::ptr::null_mut();
// Global Lock support
    acpi_gbl_global_lock_semaphore = ACPI_SEMAPHORE_NULL;
    acpi_gbl_global_lock_mutex = core::ptr::null_mut();
    acpi_gbl_global_lock_acquired = FALSE;
    acpi_gbl_global_lock_handle = 0;
    acpi_gbl_global_lock_present = FALSE;
// Miscellaneous variables
    acpi_gbl_DSDT = core::ptr::null_mut();
    acpi_gbl_cm_single_step = FALSE;
    acpi_gbl_shutdown = FALSE;
    acpi_gbl_ns_lookup_count = 0;
    acpi_gbl_ps_find_count = 0;
    acpi_gbl_acpi_hardware_present = TRUE;
    acpi_gbl_last_owner_id_index = 0;
    acpi_gbl_next_owner_id_offset = 0;
    acpi_gbl_debugger_configuration = DEBUGGER_THREADING;
    acpi_gbl_osi_mutex = core::ptr::null_mut();
// Hardware oriented
    acpi_gbl_events_initialized = FALSE;
    acpi_gbl_system_awake_and_running = TRUE;
// Namespace
    acpi_gbl_root_node = core::ptr::null_mut();
    acpi_gbl_root_node_struct.name.integer = ACPI_ROOT_NAME;
    acpi_gbl_root_node_struct.descriptor_type = ACPI_DESC_TYPE_NAMED;
    acpi_gbl_root_node_struct.type = ACPI_TYPE_DEVICE;
    acpi_gbl_root_node_struct.parent = core::ptr::null_mut();
    acpi_gbl_root_node_struct.child = core::ptr::null_mut();
    acpi_gbl_root_node_struct.peer = core::ptr::null_mut();
    acpi_gbl_root_node_struct.object = core::ptr::null_mut();

    acpi_gbl_external_list = core::ptr::null_mut();
    acpi_gbl_num_external_methods = 0;
    acpi_gbl_resolved_external_methods = 0;

    acpi_gbl_lowest_stack_pointer = ACPI_CAST_PTR(acpi_size, ACPI_SIZE_MAX);

    acpi_gbl_display_final_mem_stats = FALSE;
    acpi_gbl_disable_mem_tracking = FALSE;

    return_ACPI_STATUS(AE_OK);
    }
//
// FUNCTION:    acpi_ut_terminate
//
// PARAMETERS:  none
//
// RETURN:      none
//
// DESCRIPTION: Free global memory
//
#[no_mangle]
unsafe extern "C" fn acpi_ut_terminate() {
    static void acpi_ut_terminate(void)
    {
    ACPI_FUNCTION_TRACE(ut_terminate);
    acpi_ut_free_gpe_lists();
    acpi_ut_delete_address_lists();
    return_VOID;
    }
//
// FUNCTION:    acpi_ut_subsystem_shutdown
//
// PARAMETERS:  None
//
// RETURN:      None
//
// DESCRIPTION: Shutdown the various components. Do not delete the mutex
// objects here, because the AML debugger may be still running.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_subsystem_shutdown() {
    void acpi_ut_subsystem_shutdown(void)
    {
    ACPI_FUNCTION_TRACE(ut_subsystem_shutdown);
// Just exit if subsystem is already shutdown
    if (acpi_gbl_shutdown) {
    ACPI_ERROR((AE_INFO, "ACPI Subsystem is already terminated"));
    return_VOID;
    }
// Subsystem appears active, go ahead and shut it down
    acpi_gbl_shutdown = TRUE;
    acpi_gbl_startup_flags = 0;
    ACPI_DEBUG_PRINT((ACPI_DB_INFO, "Shutting down ACPI Subsystem\n"));

// Close the acpi_event Handling
    acpi_ev_terminate();
// Delete any dynamic _OSI interfaces
    acpi_ut_interface_terminate();

// Close the Namespace
    acpi_ns_terminate();
// Delete the ACPI tables
    acpi_tb_terminate();
// Close the globals
    acpi_ut_terminate();
// Purge the local caches
    (void)acpi_ut_delete_caches();
    return_VOID;
    }

//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/pswalk.c
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
// Module Name: pswalk - Parser routines to walk parsed op tree(s)
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

    ACPI_MODULE_NAME("pswalk")
//
// FUNCTION:    acpi_ps_delete_parse_tree
//
// PARAMETERS:  subtree_root        - Root of tree (or subtree) to delete
//
// RETURN:      None
//
// DESCRIPTION: Delete a portion of or an entire parse tree.
//

#[no_mangle]
pub unsafe extern "C" fn acpi_ps_delete_parse_tree(subtree_root: *mut union acpi_parse_object) {
    void acpi_ps_delete_parse_tree(union acpi_parse_object *subtree_root)
    {
    union acpi_parse_object *op = subtree_root;
    union acpi_parse_object *next = core::ptr::null_mut();
    union acpi_parse_object *parent = core::ptr::null_mut();
    let mut level: u32 = 0;
    ACPI_FUNCTION_TRACE_PTR(ps_delete_parse_tree, subtree_root);
    ACPI_DEBUG_PRINT((ACPI_DB_PARSE_TREES, " root %p\n", subtree_root));
// Visit all nodes in the subtree
    while (op) {
    if (op != parent) {
// This is the descending case
    if (ACPI_IS_DEBUG_ENABLED
    (ACPI_LV_PARSE_TREES, _COMPONENT)) {
// This debug option will print the entire parse tree
    acpi_os_printf("      %*s%s %p", (level * 4),
    "",
    acpi_ps_get_opcode_name(op.
    common.
    aml_opcode),
    op);
    if (op.named.aml_opcode == AML_INT_NAMEPATH_OP) {
    acpi_os_printf("  %4.4s",
    op.common.value.string);
    }
    if (op.named.aml_opcode == AML_STRING_OP) {
    acpi_os_printf("  %s",
    op.common.value.string);
    }
    acpi_os_printf("\n");
    }
// Look for an argument or child of the current op
    next = acpi_ps_get_arg(op, 0);
    if (next) {
// Still going downward in tree (Op is not completed yet)
    op = next;
    level++;
    continue;
    }
    }
// No more children, this Op is complete.
    next = op.common.next;
    parent = op.common.parent;
    acpi_ps_free_op(op);
// If we are back to the starting point, the walk is complete.
    if (op == subtree_root) {
    return_VOID;
    }
    if (next) {
    op = next;
    } else {
    level--;
    op = parent;
    }
    }
    return_VOID;
    }

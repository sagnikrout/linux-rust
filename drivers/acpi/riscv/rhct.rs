//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/riscv/rhct.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2022-2023, Ventana Micro Systems Inc
// Author: Sunil V L <sunilvl@ventanamicro.com>
//

    static struct acpi_table_rhct *acpi_get_rhct(void)
    {
    static struct acpi_table_header *rhct;
    acpi_status status;
//
// RHCT will be used at runtime on every CPU, so we
// don't need to call acpi_put_table() to release the table mapping.
//
    if (!rhct) {
    status = acpi_get_table(ACPI_SIG_RHCT, 0, &rhct);
    if (ACPI_FAILURE(status)) {
    pr_warn_once("No RHCT table found\n");
    return core::ptr::null_mut();
    }
    }
    return (struct acpi_table_rhct *)rhct;
    }
//
// During early boot, the caller should call acpi_get_table() and pass its pointer to
// these functions(and free up later). At run time, since this table can be used
// multiple times, NULL may be passed in order to use the cached table.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_get_riscv_isa(table: *mut acpi_table_header, cpu: c_uint, isa: *const c_char) -> c_int {
    int acpi_get_riscv_isa(struct acpi_table_header *table, unsigned int cpu, const char **isa)
    {
    struct acpi_rhct_node_header *node, *ref_node, *end;
    let mut size_hdr: u32 = sizeof(struct acpi_rhct_node_header);
    let mut size_hartinfo: u32 = sizeof(struct acpi_rhct_hart_info);
    struct acpi_rhct_hart_info *hart_info;
    struct acpi_rhct_isa_string *isa_node;
    struct acpi_table_rhct *rhct;
    u32 *hart_info_node_offset;
    u32 acpi_cpu_id;
    int ret;
    BUG_ON(acpi_disabled);
    ret = acpi_get_cpu_uid(cpu, &acpi_cpu_id);
    if (ret != 0)
    return ret;
    if (!table) {
    rhct = acpi_get_rhct();
    if (!rhct)
    return -ENOENT;
    } else {
    rhct = (struct acpi_table_rhct *)table;
    }
    end = ACPI_ADD_PTR(struct acpi_rhct_node_header, rhct, rhct.header.length);
    for (node = ACPI_ADD_PTR(struct acpi_rhct_node_header, rhct, rhct.node_offset);
    node < end;
    node = ACPI_ADD_PTR(struct acpi_rhct_node_header, node, node.length)) {
    if (node.type == ACPI_RHCT_NODE_TYPE_HART_INFO) {
    hart_info = ACPI_ADD_PTR(struct acpi_rhct_hart_info, node, size_hdr);
    hart_info_node_offset = ACPI_ADD_PTR(u32, hart_info, size_hartinfo);
    if (acpi_cpu_id != hart_info.uid)
    continue;
    for (int i = 0; i < hart_info.num_offsets; i++) {
    ref_node = ACPI_ADD_PTR(struct acpi_rhct_node_header,
    rhct, hart_info_node_offset[i]);
    if (ref_node.type == ACPI_RHCT_NODE_TYPE_ISA_STRING) {
    isa_node = ACPI_ADD_PTR(struct acpi_rhct_isa_string,
    ref_node, size_hdr);
// isa = isa_node->isa;
    return 0;
    }
    }
    }
    }
    return -1;
    }
    static void acpi_parse_hart_info_cmo_node(struct acpi_table_rhct *rhct,
    struct acpi_rhct_hart_info *hart_info,
    u32 *cbom_size, u32 *cboz_size, u32 *cbop_size)
    {
    let mut size_hartinfo: u32 = sizeof(struct acpi_rhct_hart_info);
    let mut size_hdr: u32 = sizeof(struct acpi_rhct_node_header);
    struct acpi_rhct_node_header *ref_node;
    struct acpi_rhct_cmo_node *cmo_node;
    u32 *hart_info_node_offset;
    hart_info_node_offset = ACPI_ADD_PTR(u32, hart_info, size_hartinfo);
    for (int i = 0; i < hart_info.num_offsets; i++) {
    ref_node = ACPI_ADD_PTR(struct acpi_rhct_node_header,
    rhct, hart_info_node_offset[i]);
    if (ref_node.type == ACPI_RHCT_NODE_TYPE_CMO) {
    cmo_node = ACPI_ADD_PTR(struct acpi_rhct_cmo_node,
    ref_node, size_hdr);
    if (cbom_size && cmo_node.cbom_size <= 30) {
    if (!*cbom_size)
// cbom_size = BIT(cmo_node->cbom_size);
#[no_mangle]
pub unsafe extern "C" fn if(BIT(cmo_node->cbom_size): *mut *mut cbom_size !=) -> else {
    else if (*cbom_size != BIT(cmo_node.cbom_size))
    pr_warn("CBOM size is not the same across harts\n");
    }
    if (cboz_size && cmo_node.cboz_size <= 30) {
    if (!*cboz_size)
// cboz_size = BIT(cmo_node->cboz_size);
#[no_mangle]
pub unsafe extern "C" fn if(BIT(cmo_node->cboz_size): *mut *mut cboz_size !=) -> else {
    else if (*cboz_size != BIT(cmo_node.cboz_size))
    pr_warn("CBOZ size is not the same across harts\n");
    }
    if (cbop_size && cmo_node.cbop_size <= 30) {
    if (!*cbop_size)
// cbop_size = BIT(cmo_node->cbop_size);
#[no_mangle]
pub unsafe extern "C" fn if(BIT(cmo_node->cbop_size): *mut *mut cbop_size !=) -> else {
    else if (*cbop_size != BIT(cmo_node.cbop_size))
    pr_warn("CBOP size is not the same across harts\n");
    }
    }
    }
    }
//
// During early boot, the caller should call acpi_get_table() and pass its pointer to
// these functions (and free up later). At run time, since this table can be used
// multiple times, pass NULL so that the table remains in memory.
//
    void acpi_get_cbo_block_size(struct acpi_table_header *table, u32 *cbom_size,
    u32 *cboz_size, u32 *cbop_size)
    {
    let mut size_hdr: u32 = sizeof(struct acpi_rhct_node_header);
    struct acpi_rhct_node_header *node, *end;
    struct acpi_rhct_hart_info *hart_info;
    struct acpi_table_rhct *rhct;
    if (acpi_disabled)
    return;
    if (table) {
    rhct = (struct acpi_table_rhct *)table;
    } else {
    rhct = acpi_get_rhct();
    if (!rhct)
    return;
    }
    if (cbom_size)
// cbom_size = 0;
    if (cboz_size)
// cboz_size = 0;
    if (cbop_size)
// cbop_size = 0;
    end = ACPI_ADD_PTR(struct acpi_rhct_node_header, rhct, rhct.header.length);
    for (node = ACPI_ADD_PTR(struct acpi_rhct_node_header, rhct, rhct.node_offset);
    node < end;
    node = ACPI_ADD_PTR(struct acpi_rhct_node_header, node, node.length)) {
    if (node.type == ACPI_RHCT_NODE_TYPE_HART_INFO) {
    hart_info = ACPI_ADD_PTR(struct acpi_rhct_hart_info, node, size_hdr);
    acpi_parse_hart_info_cmo_node(rhct, hart_info, cbom_size,
    cboz_size, cbop_size);
    }
    }
    }

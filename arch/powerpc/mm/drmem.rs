//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/drmem.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Dynamic reconfiguration memory support
//
// Copyright 2017 IBM Corporation
//

    static int n_root_addr_cells, n_root_size_cells;
    static struct drmem_lmb_info __drmem_info;
    struct drmem_lmb_info *drmem_info = &__drmem_info;
    static bool in_drmem_update;
#[no_mangle]
pub unsafe extern "C" fn drmem_lmb_memory_max() -> u64 {
    u64 drmem_lmb_memory_max(void)
    {
    struct drmem_lmb *last_lmb;
    last_lmb = &drmem_info.lmbs[drmem_info.n_lmbs - 1];
    return last_lmb.base_addr + drmem_lmb_size();
    }
#[no_mangle]
unsafe extern "C" fn drmem_lmb_flags(lmb: *mut drmem_lmb) -> u32 {
    static u32 drmem_lmb_flags(struct drmem_lmb *lmb)
    {
//
// Return the value of the lmb flags field minus the reserved
// bit used internally for hotplug processing.
//
    return lmb.flags & ~DRMEM_LMB_RESERVED;
    }
    static struct property *clone_property(struct property *prop, u32 prop_sz)
    {
    struct property *new_prop;
    new_prop = kzalloc_obj(*new_prop);
    if (!new_prop)
    return core::ptr::null_mut();
    new_prop.name = kstrdup(prop.name, GFP_KERNEL);
    new_prop.value = kzalloc(prop_sz, GFP_KERNEL);
    if (!new_prop.name || !new_prop.value) {
    kfree(new_prop.name);
    kfree(new_prop.value);
    kfree(new_prop);
    return core::ptr::null_mut();
    }
    new_prop.length = prop_sz;

    of_property_set_flag(new_prop, OF_DYNAMIC);

    return new_prop;
    }
    static int drmem_update_dt_v1(struct device_node *memory,
    struct property *prop)
    {
    struct property *new_prop;
    struct of_drconf_cell_v1 *dr_cell;
    struct drmem_lmb *lmb;
    __be32 *p;
    new_prop = clone_property(prop, prop.length);
    if (!new_prop)
    return -1;
    p = new_prop.value;
// p++ = cpu_to_be32(drmem_info->n_lmbs);
    dr_cell = (struct of_drconf_cell_v1 *)p;
    for_each_drmem_lmb(lmb) {
    dr_cell.base_addr = cpu_to_be64(lmb.base_addr);
    dr_cell.drc_index = cpu_to_be32(lmb.drc_index);
    dr_cell.aa_index = cpu_to_be32(lmb.aa_index);
    dr_cell.flags = cpu_to_be32(drmem_lmb_flags(lmb));
    dr_cell++;
    }
    of_update_property(memory, new_prop);
    return 0;
    }
    static void init_drconf_v2_cell(struct of_drconf_cell_v2 *dr_cell,
    struct drmem_lmb *lmb)
    {
    dr_cell.base_addr = cpu_to_be64(lmb.base_addr);
    dr_cell.drc_index = cpu_to_be32(lmb.drc_index);
    dr_cell.aa_index = cpu_to_be32(lmb.aa_index);
    dr_cell.flags = cpu_to_be32(drmem_lmb_flags(lmb));
    }
    static int drmem_update_dt_v2(struct device_node *memory,
    struct property *prop)
    {
    struct property *new_prop;
    struct of_drconf_cell_v2 *dr_cell;
    struct drmem_lmb *lmb, *prev_lmb;
    u32 lmb_sets, prop_sz, seq_lmbs;
    u32 *p;
// First pass, determine how many LMB sets are needed.
    lmb_sets = 0;
    prev_lmb = core::ptr::null_mut();
    for_each_drmem_lmb(lmb) {
    if (!prev_lmb) {
    prev_lmb = lmb;
    lmb_sets++;
    continue;
    }
    if (prev_lmb.aa_index != lmb.aa_index ||
    drmem_lmb_flags(prev_lmb) != drmem_lmb_flags(lmb))
    lmb_sets++;
    prev_lmb = lmb;
    }
    prop_sz = lmb_sets * sizeof(*dr_cell) + sizeof(__be32);
    new_prop = clone_property(prop, prop_sz);
    if (!new_prop)
    return -1;
    p = new_prop.value;
// p++ = cpu_to_be32(lmb_sets);
    dr_cell = (struct of_drconf_cell_v2 *)p;
// Second pass, populate the LMB set data
    prev_lmb = core::ptr::null_mut();
    seq_lmbs = 0;
    for_each_drmem_lmb(lmb) {
    if (prev_lmb == core::ptr::null_mut()) {
// Start of first LMB set
    prev_lmb = lmb;
    init_drconf_v2_cell(dr_cell, lmb);
    seq_lmbs++;
    continue;
    }
    if (prev_lmb.aa_index != lmb.aa_index ||
    drmem_lmb_flags(prev_lmb) != drmem_lmb_flags(lmb)) {
// end of one set, start of another
    dr_cell.seq_lmbs = cpu_to_be32(seq_lmbs);
    dr_cell++;
    init_drconf_v2_cell(dr_cell, lmb);
    seq_lmbs = 1;
    } else {
    seq_lmbs++;
    }
    prev_lmb = lmb;
    }
// close out last LMB set
    dr_cell.seq_lmbs = cpu_to_be32(seq_lmbs);
    of_update_property(memory, new_prop);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn drmem_update_dt() -> c_int {
    int drmem_update_dt(void)
    {
    struct device_node *memory;
    struct property *prop;
    let mut rc: c_int = -1;
    memory = of_find_node_by_path("/ibm,dynamic-reconfiguration-memory");
    if (!memory)
    return -1;
//
// Set in_drmem_update to prevent the notifier callback to process the
// DT property back since the change is coming from the LMB tree.
//
    in_drmem_update = true;
    prop = of_find_property(memory, "ibm,dynamic-memory", core::ptr::null_mut());
    if (prop) {
    rc = drmem_update_dt_v1(memory, prop);
    } else {
    prop = of_find_property(memory, "ibm,dynamic-memory-v2", core::ptr::null_mut());
    if (prop)
    rc = drmem_update_dt_v2(memory, prop);
    }
    in_drmem_update = false;
    of_node_put(memory);
    return rc;
    }
    static void read_drconf_v1_cell(struct drmem_lmb *lmb,
    const __be32 **prop)
    {
    const __be32 *p = *prop;
    lmb.base_addr = of_read_number(p, n_root_addr_cells);
    p += n_root_addr_cells;
    lmb.drc_index = of_read_number(p++, 1);
    p++; /* skip reserved field */
    lmb.aa_index = of_read_number(p++, 1);
    lmb.flags = of_read_number(p++, 1);
// prop = p;
    }
    static int
    __walk_drmem_v1_lmbs(const __be32 *prop, const __be32 *usm, void *data,
    int (*func)(struct drmem_lmb *, const __be32 **, void *))
    {
    struct drmem_lmb lmb;
    u32 i, n_lmbs;
    let mut ret: c_int = 0;
    n_lmbs = of_read_number(prop++, 1);
    for (i = 0; i < n_lmbs; i++) {
    read_drconf_v1_cell(&lmb, &prop);
    ret = func(&lmb, &usm, data);
    if (ret)
    break;
    }
    return ret;
    }
    static void read_drconf_v2_cell(struct of_drconf_cell_v2 *dr_cell,
    const __be32 **prop)
    {
    const __be32 *p = *prop;
    dr_cell.seq_lmbs = of_read_number(p++, 1);
    dr_cell.base_addr = of_read_number(p, n_root_addr_cells);
    p += n_root_addr_cells;
    dr_cell.drc_index = of_read_number(p++, 1);
    dr_cell.aa_index = of_read_number(p++, 1);
    dr_cell.flags = of_read_number(p++, 1);
// prop = p;
    }
    static int
    __walk_drmem_v2_lmbs(const __be32 *prop, const __be32 *usm, void *data,
    int (*func)(struct drmem_lmb *, const __be32 **, void *))
    {
    struct of_drconf_cell_v2 dr_cell;
    struct drmem_lmb lmb;
    u32 i, j, lmb_sets;
    let mut ret: c_int = 0;
    lmb_sets = of_read_number(prop++, 1);
    for (i = 0; i < lmb_sets; i++) {
    read_drconf_v2_cell(&dr_cell, &prop);
    for (j = 0; j < dr_cell.seq_lmbs; j++) {
    lmb.base_addr = dr_cell.base_addr;
    dr_cell.base_addr += drmem_lmb_size();
    lmb.drc_index = dr_cell.drc_index;
    dr_cell.drc_index++;
    lmb.aa_index = dr_cell.aa_index;
    lmb.flags = dr_cell.flags;
    ret = func(&lmb, &usm, data);
    if (ret)
    break;
    }
    }
    return ret;
    }

    int __init walk_drmem_lmbs_early(unsigned long node, void *data,
    int (*func)(struct drmem_lmb *, const __be32 **, void *))
    {
    const __be32 *prop, *usm;
    int len, ret = -ENODEV;
    prop = of_get_flat_dt_prop(node, "ibm,lmb-size", &len);
    if (!prop || len < dt_root_size_cells * sizeof(__be32))
    return ret;
// Get the address & size cells
    n_root_addr_cells = dt_root_addr_cells;
    n_root_size_cells = dt_root_size_cells;
    drmem_info.lmb_size = dt_mem_next_cell(dt_root_size_cells, &prop);
    usm = of_get_flat_dt_prop(node, "linux,drconf-usable-memory", &len);
    prop = of_get_flat_dt_prop(node, "ibm,dynamic-memory", &len);
    if (prop) {
    ret = __walk_drmem_v1_lmbs(prop, usm, data, func);
    } else {
    prop = of_get_flat_dt_prop(node, "ibm,dynamic-memory-v2",
    &len);
    if (prop)
    ret = __walk_drmem_v2_lmbs(prop, usm, data, func);
    }
    memblock_dump_all();
    return ret;
    }
//
// Update the LMB associativity index.
//
    static int update_lmb(struct drmem_lmb *updated_lmb,
    __maybe_unused const __be32 **usm,
    __maybe_unused void *data)
    {
    struct drmem_lmb *lmb;
    for_each_drmem_lmb(lmb) {
    if (lmb.drc_index != updated_lmb.drc_index)
    continue;
    lmb.aa_index = updated_lmb.aa_index;
    break;
    }
    return 0;
    }
//
// Update the LMB associativity index.
//
// This needs to be called when the hypervisor is updating the
// dynamic-reconfiguration-memory node property.
//
#[no_mangle]
pub unsafe extern "C" fn drmem_update_lmbs(prop: *mut property) {
    void drmem_update_lmbs(struct property *prop)
    {
//
// Don't update the LMBs if triggered by the update done in
// drmem_update_dt(), the LMB values have been used to the update the DT
// property in that case.
//
    if (in_drmem_update)
    return;
    if (!strcmp(prop.name, "ibm,dynamic-memory"))
    __walk_drmem_v1_lmbs(prop.value, core::ptr::null_mut(), core::ptr::null_mut(), update_lmb);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(prop->name, _arg: "ibm, _arg: dynamic-memory-v2")) -> else {
    else if (!strcmp(prop.name, "ibm,dynamic-memory-v2"))
    __walk_drmem_v2_lmbs(prop.value, core::ptr::null_mut(), core::ptr::null_mut(), update_lmb);
    }

#[no_mangle]
unsafe extern "C" fn init_drmem_lmb_size(dn: *mut device_node) -> c_int {
    static int init_drmem_lmb_size(struct device_node *dn)
    {
    const __be32 *prop;
    int len;
    if (drmem_info.lmb_size)
    return 0;
    prop = of_get_property(dn, "ibm,lmb-size", &len);
    if (!prop || len < n_root_size_cells * sizeof(__be32)) {
    pr_info("Could not determine LMB size\n");
    return -1;
    }
    drmem_info.lmb_size = of_read_number(prop, n_root_size_cells);
    return 0;
    }
//
// Returns the property linux,drconf-usable-memory if
// it exists (the property exists only in kexec/kdump kernels,
// added by kexec-tools)
//
    static const __be32 *of_get_usable_memory(struct device_node *dn)
    {
    const __be32 *prop;
    u32 len;
    prop = of_get_property(dn, "linux,drconf-usable-memory", &len);
    if (!prop || len < sizeof(unsigned int))
    return core::ptr::null_mut();
    return prop;
    }
    int walk_drmem_lmbs(struct device_node *dn, void *data,
    int (*func)(struct drmem_lmb *, const __be32 **, void *))
    {
    struct device_node *root = of_find_node_by_path("/");
    const __be32 *prop, *usm;
    let mut ret: c_int = -ENODEV;
    if (!root)
    return ret;
// Get the address & size cells
    n_root_addr_cells = of_n_addr_cells(root);
    n_root_size_cells = of_n_size_cells(root);
    of_node_put(root);
    if (init_drmem_lmb_size(dn))
    return ret;
    usm = of_get_usable_memory(dn);
    prop = of_get_property(dn, "ibm,dynamic-memory", core::ptr::null_mut());
    if (prop) {
    ret = __walk_drmem_v1_lmbs(prop, usm, data, func);
    } else {
    prop = of_get_property(dn, "ibm,dynamic-memory-v2", core::ptr::null_mut());
    if (prop)
    ret = __walk_drmem_v2_lmbs(prop, usm, data, func);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn init_drmem_v1_lmbs(prop: *const __be32) -> void __init {
    static void __init init_drmem_v1_lmbs(const __be32 *prop)
    {
    struct drmem_lmb *lmb;
    drmem_info.n_lmbs = of_read_number(prop++, 1);
    if (drmem_info.n_lmbs == 0)
    return;
    drmem_info.lmbs = kzalloc_objs(*lmb, drmem_info.n_lmbs);
    if (!drmem_info.lmbs)
    return;
    for_each_drmem_lmb(lmb)
    read_drconf_v1_cell(lmb, &prop);
    }
#[no_mangle]
unsafe extern "C" fn init_drmem_v2_lmbs(prop: *const __be32) -> void __init {
    static void __init init_drmem_v2_lmbs(const __be32 *prop)
    {
    struct drmem_lmb *lmb;
    struct of_drconf_cell_v2 dr_cell;
    const __be32 *p;
    u32 i, j, lmb_sets;
    int lmb_index;
    lmb_sets = of_read_number(prop++, 1);
    if (lmb_sets == 0)
    return;
// first pass, calculate the number of LMBs
    p = prop;
    for (i = 0; i < lmb_sets; i++) {
    read_drconf_v2_cell(&dr_cell, &p);
    drmem_info.n_lmbs += dr_cell.seq_lmbs;
    }
    drmem_info.lmbs = kzalloc_objs(*lmb, drmem_info.n_lmbs);
    if (!drmem_info.lmbs)
    return;
// second pass, read in the LMB information
    lmb_index = 0;
    p = prop;
    for (i = 0; i < lmb_sets; i++) {
    read_drconf_v2_cell(&dr_cell, &p);
    for (j = 0; j < dr_cell.seq_lmbs; j++) {
    lmb = &drmem_info.lmbs[lmb_index++];
    lmb.base_addr = dr_cell.base_addr;
    dr_cell.base_addr += drmem_info.lmb_size;
    lmb.drc_index = dr_cell.drc_index;
    dr_cell.drc_index++;
    lmb.aa_index = dr_cell.aa_index;
    lmb.flags = dr_cell.flags;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn drmem_init() -> int __init {
    static int __init drmem_init(void)
    {
    struct device_node *dn;
    const __be32 *prop;
    dn = of_find_node_by_path("/ibm,dynamic-reconfiguration-memory");
    if (!dn)
    return 0;
    if (init_drmem_lmb_size(dn)) {
    of_node_put(dn);
    return 0;
    }
    prop = of_get_property(dn, "ibm,dynamic-memory", core::ptr::null_mut());
    if (prop) {
    init_drmem_v1_lmbs(prop);
    } else {
    prop = of_get_property(dn, "ibm,dynamic-memory-v2", core::ptr::null_mut());
    if (prop)
    init_drmem_v2_lmbs(prop);
    }
    of_node_put(dn);
    return 0;
    }
    late_initcall(drmem_init);

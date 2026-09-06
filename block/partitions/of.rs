//! Automatically rewritten from C to Rust
//! Source: block/partitions/of.c
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


// SPDX-License-Identifier: GPL-2.0

#[no_mangle]
unsafe extern "C" fn validate_of_partition(np: *mut device_node, slot: c_int) -> c_int {
    static int validate_of_partition(struct device_node *np, int slot)
    {
    u64 offset, size;
    int len;
    const __be32 *reg = of_get_property(np, "reg", &len);
    let mut a_cells: c_int = of_n_addr_cells(np);
    let mut s_cells: c_int = of_n_size_cells(np);
// Make sure reg len match the expected addr and size cells
    if (len / sizeof(*reg) != a_cells + s_cells)
    return -EINVAL;
// Validate offset conversion from bytes to sectors
    offset = of_read_number(reg, a_cells);
    if (offset % SECTOR_SIZE)
    return -EINVAL;
// Validate size conversion from bytes to sectors
    size = of_read_number(reg + a_cells, s_cells);
    if (!size || size % SECTOR_SIZE)
    return -EINVAL;
    return 0;
    }
    static void add_of_partition(struct parsed_partitions *state, int slot,
    struct device_node *np)
    {
    struct partition_meta_info *info;
    const char *partname;
    int len;
    const __be32 *reg = of_get_property(np, "reg", &len);
    let mut a_cells: c_int = of_n_addr_cells(np);
    let mut s_cells: c_int = of_n_size_cells(np);
// Convert bytes to sector size
    let mut offset: u64 = of_read_number(reg, a_cells) / SECTOR_SIZE;
    let mut size: u64 = of_read_number(reg + a_cells, s_cells) / SECTOR_SIZE;
    put_partition(state, slot, offset, size);
    if (of_property_read_bool(np, "read-only"))
    state.parts[slot].flags |= ADDPART_FLAG_READONLY;
//
// Follow MTD label logic, search for label property,
// fallback to node name if not found.
//
    info = &state.parts[slot].info;
    partname = of_get_property(np, "label", &len);
    if (!partname)
    partname = of_get_property(np, "name", &len);
    strscpy(info.volname, partname, sizeof(info.volname));
    seq_buf_printf(&state.pp_buf, "(%s)", info.volname);
    }
#[no_mangle]
pub unsafe extern "C" fn of_partition(state: *mut parsed_partitions) -> c_int {
    int of_partition(struct parsed_partitions *state)
    {
    struct device *ddev = disk_to_dev(state.disk);
    struct device_node *np;
    int slot;
    struct device_node *partitions_np = of_node_get(ddev.of_node);
    if (!partitions_np ||
    !of_device_is_compatible(partitions_np, "fixed-partitions")) {
    of_node_put(partitions_np);
    return 0;
    }
    slot = 1;
// Validate parition offset and size
    for_each_child_of_node(partitions_np, np) {
    if (validate_of_partition(np, slot)) {
    of_node_put(np);
    of_node_put(partitions_np);
    return -1;
    }
    slot++;
    }
    slot = 1;
    for_each_child_of_node(partitions_np, np) {
    if (slot >= state.limit) {
    of_node_put(np);
    break;
    }
    add_of_partition(state, slot, np);
    slot++;
    }
    seq_buf_puts(&state.pp_buf, "\n");
    of_node_put(partitions_np);
    return 1;
    }

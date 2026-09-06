//! Automatically rewritten from C to Rust
//! Source: drivers/of/of_numa.c
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
//
// OF NUMA Parsing support.
//
// Copyright (C) 2015 - 2016 Cavium Inc.
//

//
// Even though we connect cpus to numa domains later in SMP
// init, we need to know the node ids now for all cpus.
//
#[no_mangle]
unsafe extern "C" fn of_numa_parse_cpu_nodes() -> void __init {
    static void __init of_numa_parse_cpu_nodes(void)
    {
    u32 nid;
    int r;
    struct device_node *np;
    for_each_of_cpu_node(np) {
    r = of_property_read_u32(np, "numa-node-id", &nid);
    if (r)
    continue;
    pr_debug("CPU on %u\n", nid);
    if (nid >= MAX_NUMNODES)
    pr_warn("Node id %u exceeds maximum value\n", nid);
    else
    node_set(nid, numa_nodes_parsed);
    }
    }
#[no_mangle]
unsafe extern "C" fn of_numa_parse_memory_nodes() -> int __init {
    static int __init of_numa_parse_memory_nodes(void)
    {
    struct device_node *np = core::ptr::null_mut();
    struct resource rsrc;
    u32 nid;
    int i, r = -EINVAL;
    for_each_node_by_type(np, "memory") {
    r = of_property_read_u32(np, "numa-node-id", &nid);
    if (r == -EINVAL)
//
// property doesn't exist if -EINVAL, continue
// looking for more memory nodes with
// "numa-node-id" property
//
    continue;
    if (nid >= MAX_NUMNODES) {
    pr_warn("Node id %u exceeds maximum value\n", nid);
    r = -EINVAL;
    }
    for (i = 0; !r && !of_address_to_resource(np, i, &rsrc); i++)
    r = numa_add_memblk(nid, rsrc.start, rsrc.end + 1);
    if (!i || r) {
    of_node_put(np);
    pr_err("bad property in memory node\n");
    return r ? : -EINVAL;
    }
    }
    return r;
    }
#[no_mangle]
unsafe extern "C" fn of_numa_parse_distance_map_v1(map: *mut device_node) -> int __init {
    static int __init of_numa_parse_distance_map_v1(struct device_node *map)
    {
    const __be32 *matrix;
    int entry_count;
    int i;
    pr_info("parsing numa-distance-map-v1\n");
    matrix = of_get_property(map, "distance-matrix", core::ptr::null_mut());
    if (!matrix) {
    pr_err("No distance-matrix property in distance-map\n");
    return -EINVAL;
    }
    entry_count = of_property_count_u32_elems(map, "distance-matrix");
    if (entry_count <= 0) {
    pr_err("Invalid distance-matrix\n");
    return -EINVAL;
    }
    for (i = 0; i + 2 < entry_count; i += 3) {
    u32 nodea, nodeb, distance;
    nodea = of_read_number(matrix, 1);
    matrix++;
    nodeb = of_read_number(matrix, 1);
    matrix++;
    distance = of_read_number(matrix, 1);
    matrix++;
    if ((nodea == nodeb && distance != LOCAL_DISTANCE) ||
    (nodea != nodeb && distance <= LOCAL_DISTANCE)) {
    pr_err("Invalid distance[node%d . node%d] = %d\n",
    nodea, nodeb, distance);
    return -EINVAL;
    }
    node_set(nodea, numa_nodes_parsed);
    numa_set_distance(nodea, nodeb, distance);
// Set default distance of node B->A same as A->B
    if (nodeb > nodea)
    numa_set_distance(nodeb, nodea, distance);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn of_numa_parse_distance_map() -> int __init {
    static int __init of_numa_parse_distance_map(void)
    {
    let mut ret: c_int = 0;
    struct device_node *np;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(),
    "numa-distance-map-v1");
    if (np)
    ret = of_numa_parse_distance_map_v1(np);
    of_node_put(np);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn of_node_to_nid(device: *mut device_node) -> c_int {
    int of_node_to_nid(struct device_node *device)
    {
    struct device_node *np;
    u32 nid;
    let mut r: c_int = -ENODATA;
    np = of_node_get(device);
    while (np) {
    r = of_property_read_u32(np, "numa-node-id", &nid);
//
// -EINVAL indicates the property was not found, and
// we walk up the tree trying to find a parent with a
// "numa-node-id".  Any other type of error indicates
// a bad device tree and we give up.
//
    if (r != -EINVAL)
    break;
    np = of_get_next_parent(np);
    }
    if (np && r)
    pr_warn("Invalid \"numa-node-id\" property in node %pOFn\n",
    np);
    of_node_put(np);
//
// If numa=off passed on command line, or with a defective
// device tree, the nid may not be in the set of possible
// nodes.  Check for this case and return NUMA_NO_NODE.
//
    if (!r && nid < MAX_NUMNODES && node_possible(nid))
    return nid;
    return NUMA_NO_NODE;
    }
#[no_mangle]
pub unsafe extern "C" fn of_numa_init() -> int __init {
    int __init of_numa_init(void)
    {
    int r;
    of_numa_parse_cpu_nodes();
    r = of_numa_parse_memory_nodes();
    if (r)
    return r;
    return of_numa_parse_distance_map();
    }

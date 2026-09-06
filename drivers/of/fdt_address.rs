//! Automatically rewritten from C to Rust
//! Source: drivers/of/fdt_address.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// FDT Address translation based on u-boot fdt_support.c which in turn was
// based on the kernel unflattened DT address translation code.
//
// (C) Copyright 2007
// Gerald Van Baren, Custom IDEAS, vanbaren@cideas.com
//
// Copyright 2010-2011 Freescale Semiconductor, Inc.
//

// Uncomment me to enable of_dump_addr() debugging output
// #define DEBUG

// Callbacks for bus specific translators
#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_bus {
    void		(*count_cells)(const void *blob, int parentoffset,
    pub sizec): *mut *mut int addrc, int,
    u64		(*map)(__be32 *addr, const __be32 *range,
    pub pna): int na, int ns, int,
    pub na): *mut *mut *mut int (translate)(__be32 addr, u64 offset, int,
}

// Default translator (generic bus)
    static void __init fdt_bus_default_count_cells(const void *blob, int parentoffset,
    int *addrc, int *sizec)
    {
    const __be32 *prop;
    if (addrc) {
    prop = fdt_getprop(blob, parentoffset, "#address-cells", core::ptr::null_mut());
    if (prop)
// addrc = be32_to_cpup(prop);
    else
// addrc = -1;
    }
    if (sizec) {
    prop = fdt_getprop(blob, parentoffset, "#size-cells", core::ptr::null_mut());
    if (prop)
// sizec = be32_to_cpup(prop);
    else
// sizec = -1;
    }
    }
    static u64 __init fdt_bus_default_map(__be32 *addr, const __be32 *range,
    int na, int ns, int pna)
    {
    u64 cp, s, da;
    cp = of_read_number(range, na);
    s  = of_read_number(range + na + pna, ns);
    da = of_read_number(addr, na);
    pr_debug("default map, cp=%llx, s=%llx, da=%llx\n",
    cp, s, da);
    if (da < cp || da >= (cp + s))
    return OF_BAD_ADDR;
    return da - cp;
    }
#[no_mangle]
unsafe extern "C" fn fdt_bus_default_translate(addr: *mut __be32, offset: u64, na: c_int) -> int __init {
    static int __init fdt_bus_default_translate(__be32 *addr, u64 offset, int na)
    {
    let mut a: u64 = of_read_number(addr, na);
    memset(addr, 0, na * 4);
    a += offset;
    if (na > 1)
    addr[na - 2] = cpu_to_fdt32(a >> 32);
    addr[na - 1] = cpu_to_fdt32(a & 0xffffffffu);
    return 0;
    }
// Array of bus specific translators
    static const struct of_bus of_busses[] __initconst = {
// Default
    {
    .count_cells = fdt_bus_default_count_cells,
    .map = fdt_bus_default_map,
    .translate = fdt_bus_default_translate,
    },
    };
    static int __init fdt_translate_one(const void *blob, int parent,
    const struct of_bus *bus,
    const struct of_bus *pbus, __be32 *addr,
    int na, int ns, int pna, const char *rprop)
    {
    const __be32 *ranges;
    int rlen;
    int rone;
    let mut offset: u64 = OF_BAD_ADDR;
    ranges = fdt_getprop(blob, parent, rprop, &rlen);
    if (!ranges)
    return 1;
    if (rlen == 0) {
    offset = of_read_number(addr, na);
    memset(addr, 0, pna * 4);
    pr_debug("empty ranges, 1:1 translation\n");
    goto finish;
    }
    pr_debug("walking ranges...\n");
// Now walk through the ranges
    rlen /= 4;
    rone = na + pna + ns;
    for (; rlen >= rone; rlen -= rone, ranges += rone) {
    offset = bus.map(addr, ranges, na, ns, pna);
    if (offset != OF_BAD_ADDR)
    break;
    }
    if (offset == OF_BAD_ADDR) {
    pr_debug("not found !\n");
    return 1;
    }
    memcpy(addr, ranges + na, 4 * pna);
    finish:
    of_dump_addr("parent translation for:", addr, pna);
    pr_debug("with offset: %llx\n", offset);
// Translate it into parent bus space
    return pbus.translate(addr, offset, pna);
    }
//
// Translate an address from the device-tree into a CPU physical address,
// this walks up the tree and applies the various bus mappings on the
// way.
//
// Note: We consider that crossing any level with #size-cells == 0 to mean
// that translation is impossible (that is we are not dealing with a value
// that can be mapped to a cpu physical address). This is not really specified
// that way, but this is traditionally the way IBM at least do things
//
#[no_mangle]
unsafe extern "C" fn fdt_translate_address(blob: *const c_void, node_offset: c_int) -> u64 __init {
    static u64 __init fdt_translate_address(const void *blob, int node_offset)
    {
    int parent, len;
    const struct of_bus *bus, *pbus;
    const __be32 *reg;
    __be32 addr[OF_MAX_ADDR_CELLS];
    int na, ns, pna, pns;
    let mut result: u64 = OF_BAD_ADDR;
    pr_debug("** translation for device %s **\n",
    fdt_get_name(blob, node_offset, core::ptr::null_mut()));
    reg = fdt_getprop(blob, node_offset, "reg", &len);
    if (!reg) {
    pr_err("warning: device tree node '%s' has no address.\n",
    fdt_get_name(blob, node_offset, core::ptr::null_mut()));
    goto bail;
    }
// Get parent & match bus type
    parent = fdt_parent_offset(blob, node_offset);
    if (parent < 0)
    goto bail;
    bus = &of_busses[0];
// Cound address cells & copy address locally
    bus.count_cells(blob, parent, &na, &ns);
    if (!OF_CHECK_COUNTS(na, ns)) {
    pr_err("Bad cell count for %s\n",
    fdt_get_name(blob, node_offset, core::ptr::null_mut()));
    goto bail;
    }
    memcpy(addr, reg, na * 4);
    pr_debug("bus (na=%d, ns=%d) on %s\n",
    na, ns, fdt_get_name(blob, parent, core::ptr::null_mut()));
    of_dump_addr("translating address:", addr, na);
// Translate
    for (;;) {
// Switch to parent bus
    node_offset = parent;
    parent = fdt_parent_offset(blob, node_offset);
// If root, we have finished
    if (parent < 0) {
    pr_debug("reached root node\n");
    result = of_read_number(addr, na);
    break;
    }
// Get new parent bus and counts
    pbus = &of_busses[0];
    pbus.count_cells(blob, parent, &pna, &pns);
    if (!OF_CHECK_COUNTS(pna, pns)) {
    pr_err("Bad cell count for %s\n",
    fdt_get_name(blob, node_offset, core::ptr::null_mut()));
    break;
    }
    pr_debug("parent bus (na=%d, ns=%d) on %s\n",
    pna, pns, fdt_get_name(blob, parent, core::ptr::null_mut()));
// Apply bus translation
    if (fdt_translate_one(blob, node_offset, bus, pbus,
    addr, na, ns, pna, "ranges"))
    break;
// Complete the move up one level
    na = pna;
    ns = pns;
    bus = pbus;
    of_dump_addr("one level translation:", addr, na);
    }
    bail:
    return result;
    }
//
// of_flat_dt_translate_address - translate DT addr into CPU phys addr
// @node: node in the flat blob
//
#[no_mangle]
pub unsafe extern "C" fn of_flat_dt_translate_address(node: c_ulong) -> u64 __init {
    u64 __init of_flat_dt_translate_address(unsigned long node)
    {
    return fdt_translate_address(initial_boot_params, node);
    }

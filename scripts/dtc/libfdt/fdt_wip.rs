//! Automatically rewritten from C to Rust
//! Source: scripts/dtc/libfdt/fdt_wip.c
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


// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//
// libfdt - Flat Device Tree manipulation
// Copyright (C) 2006 David Gibson, IBM Corporation.
//

    int fdt_setprop_inplace_namelen_partial(void *fdt, int nodeoffset,
    const char *name, int namelen,
    uint32_t idx, const void *val,
    int len)
    {
    void *propval;
    int proplen;
    propval = fdt_getprop_namelen_w(fdt, nodeoffset, name, namelen,
    &proplen);
    if (!propval)
    return proplen;
    if ((unsigned)proplen < (len + idx))
    return -FDT_ERR_NOSPACE;
    memcpy((char *)propval + idx, val, len);
    return 0;
    }
    int fdt_setprop_inplace(void *fdt, int nodeoffset, const char *name,
    const void *val, int len)
    {
    const void *propval;
    int proplen;
    propval = fdt_getprop(fdt, nodeoffset, name, &proplen);
    if (!propval)
    return proplen;
    if (proplen != len)
    return -FDT_ERR_NOSPACE;
    return fdt_setprop_inplace_namelen_partial(fdt, nodeoffset, name,
    strlen(name), 0,
    val, len);
    }
#[no_mangle]
unsafe extern "C" fn fdt_nop_region_(start: *mut c_void, len: c_int) {
    static void fdt_nop_region_(void *start, int len)
    {
    fdt32_t *p;
    for (p = start; (char *)p < ((char *)start + len); p++)
// p = cpu_to_fdt32(FDT_NOP);
    }
#[no_mangle]
pub unsafe extern "C" fn fdt_nop_property(fdt: *mut c_void, nodeoffset: c_int, name: *const c_char) -> c_int {
    int fdt_nop_property(void *fdt, int nodeoffset, const char *name)
    {
    struct fdt_property *prop;
    int len;
    prop = fdt_get_property_w(fdt, nodeoffset, name, &len);
    if (!prop)
    return len;
    fdt_nop_region_(prop, len + sizeof(*prop));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn fdt_node_end_offset_(fdt: *mut c_void, offset: c_int) -> c_int {
    int fdt_node_end_offset_(void *fdt, int offset)
    {
    let mut depth: c_int = 0;
    while ((offset >= 0) && (depth >= 0))
    offset = fdt_next_node(fdt, offset, &depth);
    return offset;
    }
#[no_mangle]
pub unsafe extern "C" fn fdt_nop_node(fdt: *mut c_void, nodeoffset: c_int) -> c_int {
    int fdt_nop_node(void *fdt, int nodeoffset)
    {
    int endoffset;
    endoffset = fdt_node_end_offset_(fdt, nodeoffset);
    if (endoffset < 0)
    return endoffset;
    fdt_nop_region_(fdt_offset_ptr_w(fdt, nodeoffset, 0),
    endoffset - nodeoffset);
    return 0;
    }

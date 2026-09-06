//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/ni_routing/tools/convert_c_to_py.c
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

    typedef uint8_t u8;
    typedef uint16_t u16;
    typedef int8_t  s8;
// Macro flag: #define __user

pub const NI_ROUTE_VALUE_EXTERNAL_CONVERSION: c_int = 1;

//
// write out
// {
// "family" : "<family-name>",
// "register_values": {
// <destination0>:[src0, src1, ...],
// ...
// }
//
#[no_mangle]
pub unsafe extern "C" fn family_write(rv: *const family_route_values, fp: *mut FILE) {
    void family_write(const struct family_route_values *rv, FILE *fp)
    {
    fprintf(fp,
    "  \"%s\" : {\n"
    "    # dest . {src0:val0, src1:val1, ...}\n"
    , rv.family);
    for (unsigned int dest = NI_NAMES_BASE;
    dest < (NI_NAMES_BASE + NI_NUM_NAMES);
    ++dest) {
    let mut src: c_uint = NI_NAMES_BASE;
    for (; src < (NI_NAMES_BASE + NI_NUM_NAMES) &&
    RVij(rv, B(src), B(dest)) == 0; ++src)
    ;
    if (src >= (NI_NAMES_BASE + NI_NUM_NAMES))
    continue; /* no data here */
    fprintf(fp, "    %u : {\n", dest);
    for (src = NI_NAMES_BASE; src < (NI_NAMES_BASE + NI_NUM_NAMES);
    ++src) {
    let mut r: register_type = RVij(rv, B(src), B(dest));
    const char *M;
    if (r == 0) {
    continue;
    } else if (MARKED_V(r)) {
    M = "V";
    } else if (MARKED_I(r)) {
    M = "I";
    } else if (MARKED_U(r)) {
    M = "U";
    } else {
    fprintf(stderr,
    "Invalid register marking %s[%u][%u] = %u\n",
    rv.family, dest, src, r);
    exit(1);
    }
    fprintf(fp, "      %u : \"%s(%u)\",\n",
    src, M, UNMARK(r));
    }
    fprintf(fp, "    },\n");
    }
    fprintf(fp, "  },\n\n");
    }
#[no_mangle]
pub unsafe extern "C" fn is_valid_ni_sig(sig: c_uint) -> bool {
    bool is_valid_ni_sig(unsigned int sig)
    {
    return (sig >= NI_NAMES_BASE) && (sig < (NI_NAMES_BASE + NI_NUM_NAMES));
    }
//
// write out
// {
// "family" : "<family-name>",
// "register_values": {
// <destination0>:[src0, src1, ...],
// ...
// }
//
#[no_mangle]
pub unsafe extern "C" fn device_write(dR: *const ni_device_routes, fp: *mut FILE) {
    void device_write(const struct ni_device_routes *dR, FILE *fp)
    {
    fprintf(fp,
    "  \"%s\" : {\n"
    "    # dest . [src0, src1, ...]\n"
    , dR.device);
    let mut i: c_uint = 0;
    while (dR.routes[i].dest != 0) {
    if (!is_valid_ni_sig(dR.routes[i].dest)) {
    fprintf(stderr,
    "Invalid NI signal value [%u] for destination %s.[%u]\n",
    dR.routes[i].dest, dR.device, i);
    exit(1);
    }
    fprintf(fp, "    %u : [", dR.routes[i].dest);
    let mut j: c_uint = 0;
    while (dR.routes[i].src[j] != 0) {
    if (!is_valid_ni_sig(dR.routes[i].src[j])) {
    fprintf(stderr,
    "Invalid NI signal value [%u] for source %s.[%u].[%u]\n",
    dR.routes[i].src[j], dR.device, i, j);
    exit(1);
    }
    fprintf(fp, "%u,", dR.routes[i].src[j]);
    ++j;
    }
    fprintf(fp, "],\n");
    ++i;
    }
    fprintf(fp, "  },\n\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    FILE *fp = fopen("ni_values.py", "w");
    if (fp == core::ptr::null_mut()) {
    fprintf(stderr, "Could not open file!");
    return -1;
    }
// write route register values
    fprintf(fp, "ni_route_values = {\n");
    for (int i = 0; ni_all_route_values[i]; ++i)
    family_write(ni_all_route_values[i], fp);
    fprintf(fp, "}\n\n");
// write valid device routes
    fprintf(fp, "ni_device_routes = {\n");
    for (int i = 0; ni_device_routes_list[i]; ++i)
    device_write(ni_device_routes_list[i], fp);
    fprintf(fp, "}\n");
// finish; close file
    fclose(fp);
    return 0;
    }

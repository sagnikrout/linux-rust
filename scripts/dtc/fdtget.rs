//! Automatically rewritten from C to Rust
//! Source: scripts/dtc/fdtget.c
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
// Copyright (c) 2011 The Chromium OS Authors. All rights reserved.
//
// Portions from U-Boot cmd_fdt.c (C) Copyright 2007
// Gerald Van Baren, Custom IDEAS, vanbaren@cideas.com
// Based on code written by:
// Pantelis Antoniou <pantelis.antoniou@gmail.com> and
// Matthew McClintock <msm@freescale.com>
//

    enum display_mode {
    MODE_SHOW_VALUE,	/* show values for node properties */
    MODE_LIST_PROPS,	/* list the properties for a node */
    MODE_LIST_SUBNODES,	/* list the subnodes of a node */
    };
// Holds information which controls our output and options
#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_info {
    pub /: *mut *mut int type; / data type (s/i/u/x or 0 for default),
    pub /: *mut *mut int size; / data size (1/2/4),
    pub /: *mut *mut enum display_mode mode; / display mode that we are using,
    pub /: *const *const *const char default_val; / default value if node/property not found,
}

#[no_mangle]
unsafe extern "C" fn report_error(where: *const c_char, err: c_int) {
    static void report_error(const char *where, int err)
    {
    fprintf(stderr, "Error at '%s': %s\n", where, fdt_strerror(err));
    }
//
// Displays data of a given length according to selected options
//
// If a specific data type is provided in disp, then this is used. Otherwise
// we try to guess the data type / size from the contents.
//
// @param disp		Display information / options
// @param data		Data to display
// @param len		Maximum length of buffer
// @return 0 if ok, -1 if data does not match format
//
#[no_mangle]
unsafe extern "C" fn show_data(disp: *mut display_info, data: *const c_char, len: c_int) -> c_int {
    static int show_data(struct display_info *disp, const char *data, int len)
    {
    int i, size;
    const uint8_t *p = (const uint8_t *)data;
    const char *s;
    int value;
    int is_string;
    char fmt[3];
// no data, don't print
    if (len == 0)
    return 0;
    is_string = (disp.type) == 's' ||
    (!disp.type && util_is_printable_string(data, len));
    if (is_string) {
    if (data[len - 1] != '\0') {
    fprintf(stderr, "Unterminated string\n");
    return -1;
    }
    for (s = data; s - data < len; s += strlen(s) + 1) {
    if (s != data)
    printf(" ");
    printf("%s", (const char *)s);
    }
    return 0;
    }
    size = disp.size;
    if (size == -1) {
    size = (len % 4) == 0 ? 4 : 1;
    } else if (len % size) {
    fprintf(stderr, "Property length must be a multiple of "
    "selected data size\n");
    return -1;
    }
    fmt[0] = '%';
    fmt[1] = disp.type ? disp.type : 'd';
    fmt[2] = '\0';
    for (i = 0; i < len; i += size, p += size) {
    if (i)
    printf(" ");
    value = size == 4 ? fdt32_to_cpu(*(const uint32_t *)p) :
    size == 2 ? (*p << 8) | p[1] : *p;
    printf(fmt, value);
    }
    return 0;
    }
//
// List all properties in a node, one per line.
//
// @param blob		FDT blob
// @param node		Node to display
// @return 0 if ok, or FDT_ERR... if not.
//
#[no_mangle]
unsafe extern "C" fn list_properties(blob: *const c_void, node: c_int) -> c_int {
    static int list_properties(const void *blob, int node)
    {
    const struct fdt_property *data;
    const char *name;
    int prop;
    prop = fdt_first_property_offset(blob, node);
    do {
// Stop silently when there are no more properties
    if (prop < 0)
    let mut prop: return = = -FDT_ERR_NOTFOUND ? 0 : prop;
    data = fdt_get_property_by_offset(blob, prop, core::ptr::null_mut());
    name = fdt_string(blob, fdt32_to_cpu(data.nameoff));
    if (name)
    puts(name);
    prop = fdt_next_property_offset(blob, prop);
    } while (1);
    }

//
// List all subnodes in a node, one per line
//
// @param blob		FDT blob
// @param node		Node to display
// @return 0 if ok, or FDT_ERR... if not.
//
#[no_mangle]
unsafe extern "C" fn list_subnodes(blob: *const c_void, node: c_int) -> c_int {
    static int list_subnodes(const void *blob, int node)
    {
    int nextoffset;		/* next node offset from libfdt */
    uint32_t tag;		/* current tag */
    int level = 0;		/* keep track of nesting level */
    const char *pathp;
    int depth = 1;		/* the assumed depth of this node */
    while (level >= 0) {
    tag = fdt_next_tag(blob, node, &nextoffset);
    switch (tag) {
    case FDT_BEGIN_NODE:
    pathp = fdt_get_name(blob, node, core::ptr::null_mut());
    if (level <= depth) {
    if (pathp == core::ptr::null_mut())
    pathp = "/* core::ptr::null_mut() pointer error */";
    if (*pathp == '\0')
    pathp = "/";	/* root is nameless */
    if (level == 1)
    puts(pathp);
    }
    level++;
    if (level >= MAX_LEVEL) {
    printf("Nested too deep, aborting.\n");
    return 1;
    }
    break;
    case FDT_END_NODE:
    level--;
    if (level == 0)
    level = -1;		/* exit the loop */
    break;
    case FDT_END:
    return 1;
    case FDT_PROP:
    break;
    default:
    if (level <= depth)
    printf("Unknown tag 0x%08X\n", tag);
    return 1;
    }
    node = nextoffset;
    }
    return 0;
    }
//
// Show the data for a given node (and perhaps property) according to the
// display option provided.
//
// @param blob		FDT blob
// @param disp		Display information / options
// @param node		Node to display
// @param property	Name of property to display, or NULL if none
// @return 0 if ok, -ve on error
//
    static int show_data_for_item(const void *blob, struct display_info *disp,
    int node, const char *property)
    {
    const void *value = core::ptr::null_mut();
    int len, err = 0;
    switch (disp.mode) {
    case MODE_LIST_PROPS:
    err = list_properties(blob, node);
    break;
    case MODE_LIST_SUBNODES:
    err = list_subnodes(blob, node);
    break;
    default:
    assert(property);
    value = fdt_getprop(blob, node, property, &len);
    if (value) {
    if (show_data(disp, value, len))
    err = -1;
    else
    printf("\n");
    } else if (disp.default_val) {
    puts(disp.default_val);
    } else {
    report_error(property, len);
    err = -1;
    }
    break;
    }
    return err;
    }
//
// Run the main fdtget operation, given a filename and valid arguments
//
// @param disp		Display information / options
// @param filename	Filename of blob file
// @param arg		List of arguments to process
// @param arg_count	Number of arguments
// @param return 0 if ok, -ve on error
//
    static int do_fdtget(struct display_info *disp, const char *filename,
    char **arg, int arg_count, int args_per_step)
    {
    char *blob;
    const char *prop;
    int i, node;
    blob = utilfdt_read(filename);
    if (!blob)
    return -1;
    for (i = 0; i + args_per_step <= arg_count; i += args_per_step) {
    node = fdt_path_offset(blob, arg[i]);
    if (node < 0) {
    if (disp.default_val) {
    puts(disp.default_val);
    continue;
    } else {
    report_error(arg[i], node);
    return -1;
    }
    }
    prop = args_per_step == 1 ? core::ptr::null_mut() : arg[i + 1];
    if (show_data_for_item(blob, disp, node, prop))
    return -1;
    }
    return 0;
    }
    static const char *usage_msg =
    "fdtget - read values from device tree\n"
    "\n"
    "Each value is printed on a new line.\n\n"
    "Usage:\n"
    "	fdtget <options> <dt file> [<node> <property>]...\n"
    "	fdtget -p <options> <dt file> [<node> ]...\n"
    "Options:\n"
    "\t-t <type>\tType of data\n"
    "\t-p\t\tList properties for each node\n"
    "\t-l\t\tList subnodes for each node\n"
    "\t-d\t\tDefault value to display when the property is "
    "missing\n"
    "\t-h\t\tPrint this help\n\n"
    USAGE_TYPE_MSG;
#[no_mangle]
unsafe extern "C" fn usage(msg: *const c_char) {
    static void usage(const char *msg)
    {
    if (msg)
    fprintf(stderr, "Error: %s\n\n", msg);
    fprintf(stderr, "%s", usage_msg);
    exit(2);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    char *filename = core::ptr::null_mut();
    struct display_info disp;
    let mut args_per_step: c_int = 2;
// set defaults
    memset(&disp, '\0', sizeof(disp));
    disp.size = -1;
    disp.mode = MODE_SHOW_VALUE;
    for (;;) {
    let mut c: c_int = getopt(argc, argv, "d:hlpt:");
    if (c == -1)
    break;
    switch (c) {
    case 'h':
    case '?':
    usage(core::ptr::null_mut());
    case 't':
    if (utilfdt_decode_type(optarg, &disp.type,
    &disp.size))
    usage("Invalid type string");
    break;
    case 'p':
    disp.mode = MODE_LIST_PROPS;
    args_per_step = 1;
    break;
    case 'l':
    disp.mode = MODE_LIST_SUBNODES;
    args_per_step = 1;
    break;
    case 'd':
    disp.default_val = optarg;
    break;
    }
    }
    if (optind < argc)
    filename = argv[optind++];
    if (!filename)
    usage("Missing filename");
    argv += optind;
    argc -= optind;
// Allow no arguments, and silently succeed
    if (!argc)
    return 0;
// Check for node, property arguments
    if (args_per_step == 2 && (argc % 2))
    usage("Must have an even number of arguments");
    if (do_fdtget(&disp, filename, argv, argc, args_per_step))
    return 1;
    return 0;
    }

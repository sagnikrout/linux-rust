//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/xe/xe_gen_wa_oob.c
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//
// Macro flag: #define _GNU_SOURCE

    "// SPDX-License-Identifier: MIT\n" \
    "\n" \
    "/*\n" \
    " * DO NOT MODIFY.\n" \
    " *\n" \
    " * This file was generated from rules: %s\n" \
    " */\n" \
    "#ifndef _GENERATED_%s_\n" \
    "#define _GENERATED_%s_\n" \
    "\n" \
    "enum {\n"

    "};\n" \
    "\n" \
    "#endif\n"
#[no_mangle]
unsafe extern "C" fn print_usage(f: *mut FILE, progname: *const c_char) {
    static void print_usage(FILE *f, const char *progname)
    {
    fprintf(f, "usage: %s <input-rule-file> <generated-c-source-file> <generated-c-header-file>\n",
    progname);
    }
    static void print_parse_error(const char *err_msg, const char *line,
    unsigned int lineno)
    {
    fprintf(stderr, "ERROR: %s\nERROR: %u: %.60s\n",
    err_msg, lineno, line);
    }
    static char *strip(char *line, size_t linelen)
    {
    while (isspace(*(line + linelen)))
    linelen--;
    line[linelen - 1] = '\0';
    return  line + strspn(line, " \f\n\r\t\v");
    }
pub const MAX_LINE_LEN: c_int = 4096;
#[no_mangle]
unsafe extern "C" fn parse(input: *mut FILE, csource: *mut FILE, cheader: *mut FILE, prefix: *mut c_char) -> c_int {
    static int parse(FILE *input, FILE *csource, FILE *cheader, char *prefix)
    {
    char line[MAX_LINE_LEN + 1];
    char *name, *prev_name = core::ptr::null_mut(), *rules;
    let mut lineno: c_uint = 0, idx = 0;
    while (fgets(line, sizeof(line), input)) {
    size_t linelen;
    bool is_continuation;
    if (line[0] == '\0' || line[0] == '#' || line[0] == '\n') {
    lineno++;
    continue;
    }
    linelen = strlen(line);
    if (linelen == MAX_LINE_LEN) {
    print_parse_error("line too long", line, lineno);
    return -EINVAL;
    }
    is_continuation = isspace(line[0]);
    name = strip(line, linelen);
    if (!is_continuation) {
    name = strtok(name, " \t");
    rules = strtok(core::ptr::null_mut(), "");
    } else {
    if (!prev_name) {
    print_parse_error("invalid rule continuation",
    line, lineno);
    return -EINVAL;
    }
    rules = name;
    name = core::ptr::null_mut();
    }
    if (rules[0] == '\0') {
    print_parse_error("invalid empty rule\n", line, lineno);
    return -EINVAL;
    }
    if (name) {
    fprintf(cheader, "\t%s_%s = %u,\n", prefix, name, idx);
// Close previous entry before starting a new one
    if (idx)
    fprintf(csource, ") },\n");
    fprintf(csource, "{ XE_RTP_NAME(\"%s\"),\n  XE_RTP_RULES(%s",
    name, rules);
    idx++;
    } else {
    fprintf(csource, ", OR,\n\t%s", rules);
    }
    lineno++;
    if (!is_continuation)
    prev_name = name;
    }
// Close last entry
    if (idx)
    fprintf(csource, ") },\n");
    fprintf(cheader, "\t_%s_COUNT = %u\n", prefix, idx);
    return 0;
    }
// Avoid GNU vs POSIX basename() discrepancy, just use our own
    static const char *xbasename(const char *s)
    {
    const char *p = strrchr(s, '/');
    return p ? p + 1 : s;
    }
#[no_mangle]
unsafe extern "C" fn fn_to_prefix(fn: *const c_char, prefix: *mut c_char, size: usize) -> c_int {
    static int fn_to_prefix(const char *fn, char *prefix, size_t size)
    {
    size_t len;
    fn = xbasename(fn);
    len = strlen(fn);
    if (len > size - 1)
    return -ENAMETOOLONG;
    memcpy(prefix, fn, len + 1);
    for (char *p = prefix; *p; p++) {
    switch (*p) {
    case '.':
// p = '\0';
    return 0;
    default:
// p = toupper(*p);
    break;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *const c_char) -> c_int {
    int main(int argc, const char *argv[])
    {
    enum {
    ARGS_INPUT,
    ARGS_CSOURCE,
    ARGS_CHEADER,
    _ARGS_COUNT
    };
    struct {
    const char *fn;
    const char *mode;
    FILE *f;
    } args[] = {
    [ARGS_INPUT] = { .fn = argv[1], .mode = "r" },
    [ARGS_CSOURCE] = { .fn = argv[2], .mode = "w" },
    [ARGS_CHEADER] = { .fn = argv[3], .mode = "w" },
    };
    let mut ret: c_int = 1;
    char prefix[128];
    if (argc < 3) {
    fprintf(stderr, "ERROR: wrong arguments\n");
    print_usage(stderr, argv[0]);
    return 1;
    }
    if (fn_to_prefix(args[ARGS_CHEADER].fn, prefix, sizeof(prefix)) < 0)
    return 1;
    for (int i = 0; i < _ARGS_COUNT; i++) {
    args[i].f = fopen(args[i].fn, args[i].mode);
    if (!args[i].f) {
    fprintf(stderr, "ERROR: Can't open %s: %m\n",
    args[i].fn);
    goto err;
    }
    }
    fprintf(args[ARGS_CHEADER].f, HEADER, xbasename(args[ARGS_INPUT].fn),
    prefix, prefix);
    ret = parse(args[ARGS_INPUT].f, args[ARGS_CSOURCE].f,
    args[ARGS_CHEADER].f, prefix);
    if (!ret)
    fprintf(args[ARGS_CHEADER].f, FOOTER);
    err:
    for (int i = 0; i < _ARGS_COUNT; i++) {
    if (args[i].f)
    fclose(args[i].f);
    }
    return ret;
    }

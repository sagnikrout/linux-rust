//! Automatically rewritten from C to Rust
//! Source: tools/bpf/bpftool/token.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2025 Didi Technology Co., Tao Chen

// Macro flag: #define _GNU_SOURCE

    static struct {
    const char *header;
    const char *key;
    } sets[] = {
    {"allowed_cmds", "delegate_cmds"},
    {"allowed_maps", "delegate_maps"},
    {"allowed_progs", "delegate_progs"},
    {"allowed_attachs", "delegate_attachs"},
    };
#[no_mangle]
unsafe extern "C" fn has_delegate_options(mnt_ops: *const c_char) -> bool {
    static bool has_delegate_options(const char *mnt_ops)
    {
    return strstr(mnt_ops, "delegate_cmds") ||
    strstr(mnt_ops, "delegate_maps") ||
    strstr(mnt_ops, "delegate_progs") ||
    strstr(mnt_ops, "delegate_attachs");
    }
    static char *get_delegate_value(char *opts, const char *key)
    {
    char *token, *rest, *ret = core::ptr::null_mut();
    if (!opts)
    return core::ptr::null_mut();
    for (token = strtok_r(opts, ",", &rest); token;
    token = strtok_r(core::ptr::null_mut(), ",", &rest)) {
    if (strncmp(token, key, strlen(key)) == 0 &&
    token[strlen(key)] == '=') {
    ret = token + strlen(key) + 1;
    break;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn print_items_per_line(input: *mut c_char, items_per_line: c_int) {
    static void print_items_per_line(char *input, int items_per_line)
    {
    char *str, *rest;
    let mut cnt: c_int = 0;
    if (!input)
    return;
    for (str = strtok_r(input, ":", &rest); str;
    str = strtok_r(core::ptr::null_mut(), ":", &rest)) {
    if (cnt % items_per_line == 0)
    printf("\n\t  ");
    printf("%-20s", str);
    cnt++;
    }
    }
pub const ITEMS_PER_LINE: c_int = 4;
#[no_mangle]
unsafe extern "C" fn show_token_info_plain(mntent: *mut mntent) {
    static void show_token_info_plain(struct mntent *mntent)
    {
    size_t i;
    printf("token_info  %s", mntent.mnt_dir);
    for (i = 0; i < ARRAY_SIZE(sets); i++) {
    char *opts, *value;
    printf("\n\t%s:", sets[i].header);
    opts = strdup(mntent.mnt_opts);
    value = get_delegate_value(opts, sets[i].key);
    print_items_per_line(value, ITEMS_PER_LINE);
    free(opts);
    }
    printf("\n");
    }
#[no_mangle]
unsafe extern "C" fn split_json_array_str(input: *mut c_char) {
    static void split_json_array_str(char *input)
    {
    char *str, *rest;
    if (!input) {
    jsonw_start_array(json_wtr);
    jsonw_end_array(json_wtr);
    return;
    }
    jsonw_start_array(json_wtr);
    for (str = strtok_r(input, ":", &rest); str;
    str = strtok_r(core::ptr::null_mut(), ":", &rest)) {
    jsonw_string(json_wtr, str);
    }
    jsonw_end_array(json_wtr);
    }
#[no_mangle]
unsafe extern "C" fn show_token_info_json(mntent: *mut mntent) {
    static void show_token_info_json(struct mntent *mntent)
    {
    size_t i;
    jsonw_start_object(json_wtr);
    jsonw_string_field(json_wtr, "token_info", mntent.mnt_dir);
    for (i = 0; i < ARRAY_SIZE(sets); i++) {
    char *opts, *value;
    jsonw_name(json_wtr, sets[i].header);
    opts = strdup(mntent.mnt_opts);
    value = get_delegate_value(opts, sets[i].key);
    split_json_array_str(value);
    free(opts);
    }
    jsonw_end_object(json_wtr);
    }
#[no_mangle]
unsafe extern "C" fn __show_token_info(mntent: *mut mntent) -> c_int {
    static int __show_token_info(struct mntent *mntent)
    {
    if (json_output)
    show_token_info_json(mntent);
    else
    show_token_info_plain(mntent);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn show_token_info() -> c_int {
    static int show_token_info(void)
    {
    FILE *fp;
    struct mntent *ent;
    fp = setmntent(MOUNTS_FILE, "r");
    if (!fp) {
    p_err("Failed to open: %s", MOUNTS_FILE);
    return -1;
    }
    if (json_output)
    jsonw_start_array(json_wtr);
    while ((ent = getmntent(fp)) != core::ptr::null_mut()) {
    if (strncmp(ent.mnt_type, "bpf", 3) == 0) {
    if (has_delegate_options(ent.mnt_opts))
    __show_token_info(ent);
    }
    }
    if (json_output)
    jsonw_end_array(json_wtr);
    endmntent(fp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_show(argc: c_int, argv: *mut c_char) -> c_int {
    static int do_show(int argc, char **argv)
    {
    if (argc)
    return BAD_ARG();
    return show_token_info();
    }
#[no_mangle]
unsafe extern "C" fn do_help(argc: c_int, argv: *mut c_char) -> c_int {
    static int do_help(int argc, char **argv)
    {
    if (json_output) {
    jsonw_null(json_wtr);
    return 0;
    }
    fprintf(stderr,
    "Usage: %1$s %2$s { show | list }\n"
    "       %1$s %2$s help\n"
    "       " HELP_SPEC_OPTIONS " }\n"
    "\n"
    "",
    bin_name, argv[-2]);
    return 0;
    }
    static const struct cmd cmds[] = {
    { "show",	do_show },
    { "list",	do_show },
    { "help",	do_help },
    { 0 }
    };
#[no_mangle]
pub unsafe extern "C" fn do_token(argc: c_int, argv: *mut c_char) -> c_int {
    int do_token(int argc, char **argv)
    {
    return cmd_select(cmds, argc, argv, do_help);
    }

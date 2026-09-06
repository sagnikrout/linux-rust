//! Automatically rewritten from C to Rust
//! Source: tools/perf/builtin-data.c
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

    typedef int (*data_cmd_fn_t)(int argc, const char **argv);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_cmd {
    pub name: *const c_char,
    pub summary: *const c_char,
    pub fn: data_cmd_fn_t,
}

    static struct data_cmd data_cmds[];

    for (cmd = data_cmds; cmd && cmd.name; cmd++)
    static const char * const data_subcommands[] = { "convert", core::ptr::null_mut() };
    static const char *data_usage[] = {
    "perf data convert [<options>]",
    core::ptr::null_mut()
    };
    static const char *to_json;
    static const char *to_ctf;
    static struct perf_data_convert_opts opts = {
    .force = false,
    .all = false,
    .time_str = core::ptr::null_mut(),
    };
    static const struct option data_options[] = {
    OPT_INCR('v', "verbose", &verbose, "be more verbose"),
    OPT_STRING('i', "input", &input_name, "file", "input file name"),
    OPT_STRING(0, "to-json", &to_json, core::ptr::null_mut(), "Convert to JSON format"),
    OPT_STRING(0, "to-ctf", &to_ctf, core::ptr::null_mut(), "Convert to CTF format"),
    OPT_BOOLEAN(0, "tod", &opts.tod, "Convert time to wall clock time"),
    OPT_BOOLEAN('f', "force", &opts.force, "don't complain, do it"),
    OPT_BOOLEAN(0, "all", &opts.all, "Convert all events"),
    OPT_STRING(0, "time", &opts.time_str, "str",
    "Time span of interest (start,stop)"),
    OPT_END()
    };
#[no_mangle]
unsafe extern "C" fn cmd_data_convert(argc: c_int, argv: *const c_char) -> c_int {
    static int cmd_data_convert(int argc, const char **argv)
    {
    argc = parse_options(argc, argv, data_options,
    data_usage, 0);
    if (argc) {
    usage_with_options(data_usage, data_options);
    return -1;
    }
    if (to_json && to_ctf) {
    pr_err("You cannot specify both --to-ctf and --to-json.\n");
    return -1;
    }
    if (!to_json && !to_ctf) {
    pr_err("You must specify one of --to-ctf or --to-json.\n");
    return -1;
    }
    if (to_json)
    return bt_convert__perf2json(input_name, to_json, &opts);
    if (to_ctf) {

    return bt_convert__perf2ctf(input_name, to_ctf, &opts);

    pr_err("The babeltrace2 ctf support is not compiled in. Ensure you have both\n"
    "libbabeltrace2-dev[el] and libtraceevent-dev[el] installed or set\n"
    "PKG_CONFIG_PATH to find a local installation of those libraries.\n");
    return -1;

    }
    return 0;
    }
    static struct data_cmd data_cmds[] = {
    { "convert", "converts data file between formats", cmd_data_convert },
    { .name = core::ptr::null_mut(), },
    };
#[no_mangle]
pub unsafe extern "C" fn cmd_data(argc: c_int, argv: *const c_char) -> c_int {
    int cmd_data(int argc, const char **argv)
    {
    struct data_cmd *cmd;
    const char *cmdstr;
    argc = parse_options_subcommand(argc, argv, data_options, data_subcommands, data_usage,
    PARSE_OPT_STOP_AT_NON_OPTION);
    if (!argc) {
    usage_with_options(data_usage, data_options);
    return -1;
    }
    cmdstr = argv[0];
    for_each_cmd(cmd) {
    if (strcmp(cmd.name, cmdstr))
    continue;
    return cmd.fn(argc, argv);
    }
    pr_err("Unknown command: %s\n", cmdstr);
    usage_with_options(data_usage, data_options);
    return -1;
    }

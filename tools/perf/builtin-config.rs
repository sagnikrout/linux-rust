//! Automatically rewritten from C to Rust
//! Source: tools/perf/builtin-config.c
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
// builtin-config.c
//
// Copyright (C) 2015, Taeung Song <treeze.taeung@gmail.com>
//

    static bool use_system_config, use_user_config;
    static const char * const config_usage[] = {
    "perf config [<file-option>] [options] [section.name[=value] ...]",
    core::ptr::null_mut()
    };
    static enum actions {
    ACTION_LIST = 1
    } actions;
    static struct option config_options[] = {
    OPT_SET_UINT('l', "list", &actions,
    "show current config variables", ACTION_LIST),
    OPT_BOOLEAN(0, "system", &use_system_config, "use system config file"),
    OPT_BOOLEAN(0, "user", &use_user_config, "use user config file"),
    OPT_END()
    };
#[no_mangle]
unsafe extern "C" fn set_config(set: *mut perf_config_set, file_name: *const c_char) -> c_int {
    static int set_config(struct perf_config_set *set, const char *file_name)
    {
    struct perf_config_section *section = core::ptr::null_mut();
    struct perf_config_item *item = core::ptr::null_mut();
    const char *first_line = "# this file is auto-generated.";
    FILE *fp;
    if (set == core::ptr::null_mut())
    return -1;
    fp = fopen(file_name, "w");
    if (!fp)
    return -1;
    fprintf(fp, "%s\n", first_line);
// overwrite configvariables
    perf_config_items__for_each_entry(&set.sections, section) {
    if (!use_system_config && section.from_system_config)
    continue;
    fprintf(fp, "[%s]\n", section.name);
    perf_config_items__for_each_entry(&section.items, item) {
    if (!use_system_config && item.from_system_config)
    continue;
    if (item.value)
    fprintf(fp, "\t%s = %s\n",
    item.name, item.value);
    }
    }
    fclose(fp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn show_spec_config(set: *mut perf_config_set, var: *const c_char) -> c_int {
    static int show_spec_config(struct perf_config_set *set, const char *var)
    {
    struct perf_config_section *section;
    struct perf_config_item *item;
    if (set == core::ptr::null_mut())
    return -1;
    perf_config_items__for_each_entry(&set.sections, section) {
    if (!strstarts(var, section.name))
    continue;
    perf_config_items__for_each_entry(&section.items, item) {
    const char *name = var + strlen(section.name) + 1;
    if (strcmp(name, item.name) == 0) {
    char *value = item.value;
    if (value) {
    printf("%s=%s\n", var, value);
    return 0;
    }
    }
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn show_config(set: *mut perf_config_set) -> c_int {
    static int show_config(struct perf_config_set *set)
    {
    struct perf_config_section *section;
    struct perf_config_item *item;
    if (set == core::ptr::null_mut())
    return -1;
    perf_config_set__for_each_entry(set, section, item) {
    char *value = item.value;
    if (value)
    printf("%s.%s=%s\n", section.name,
    item.name, value);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn parse_config_arg(arg: *mut c_char, var: *mut c_char, value: *mut c_char) -> c_int {
    static int parse_config_arg(char *arg, char **var, char **value)
    {
    const char *last_dot = strchr(arg, '.');
//
// Since "var" actually contains the section name and the real
// config variable name separated by a dot, we have to know where the dot is.
//
    if (last_dot == core::ptr::null_mut() || last_dot == arg) {
    pr_err("The config variable does not contain a section name: %s\n", arg);
    return -1;
    }
    if (!last_dot[1]) {
    pr_err("The config variable does not contain a variable name: %s\n", arg);
    return -1;
    }
// value = strchr(arg, '=');
    if (*value == core::ptr::null_mut())
// var = arg;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: *mut !strcmp(value, _arg: "=")) -> else {
    pr_err("The config variable does not contain a value: %s\n", arg);
    return -1;
    } else {
// value = *value + 1; /* excluding a first character '='
// var = strsep(&arg, "=");
    if (*var[0] == '\0') {
    pr_err("invalid config variable: %s\n", arg);
    return -1;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_config__set_variable(var: *const c_char, value: *const c_char) -> c_int {
    int perf_config__set_variable(const char *var, const char *value)
    {
    char path[PATH_MAX];
    char *user_config = mkpath(path, sizeof(path), "%s/.perfconfig", getenv("HOME"));
    const char *config_filename;
    struct perf_config_set *set;
    let mut ret: c_int = -1;
    if (use_system_config)
    config_exclusive_filename = perf_etc_perfconfig();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: use_user_config) -> else {
    else if (use_user_config)
    config_exclusive_filename = user_config;
    if (!config_exclusive_filename)
    config_filename = user_config;
    else
    config_filename = config_exclusive_filename;
    set = perf_config_set__new();
    if (!set)
    goto out_err;
    if (perf_config_set__collect(set, config_filename, var, value) < 0) {
    pr_err("Failed to add '%s=%s'\n", var, value);
    goto out_err;
    }
    if (set_config(set, config_filename) < 0) {
    pr_err("Failed to set the configs on %s\n", config_filename);
    goto out_err;
    }
    ret = 0;
    out_err:
    perf_config_set__delete(set);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cmd_config(argc: c_int, argv: *const c_char) -> c_int {
    int cmd_config(int argc, const char **argv)
    {
    int i, ret = -1;
    struct perf_config_set *set;
    char path[PATH_MAX];
    char *user_config = mkpath(path, sizeof(path), "%s/.perfconfig", getenv("HOME"));
    const char *config_filename;
    let mut changed: bool = false;
    argc = parse_options(argc, argv, config_options, config_usage,
    PARSE_OPT_STOP_AT_NON_OPTION);
    if (use_system_config && use_user_config) {
    pr_err("Error: only one config file at a time\n");
    parse_options_usage(config_usage, config_options, "user", 0);
    parse_options_usage(core::ptr::null_mut(), config_options, "system", 0);
    return -1;
    }
    if (use_system_config)
    config_exclusive_filename = perf_etc_perfconfig();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: use_user_config) -> else {
    else if (use_user_config)
    config_exclusive_filename = user_config;
    if (!config_exclusive_filename)
    config_filename = user_config;
    else
    config_filename = config_exclusive_filename;
//
// At only 'config' sub-command, individually use the config set
// because of reinitializing with options config file location.
//
    set = perf_config_set__new();
    if (!set)
    goto out_err;
    switch (actions) {
    case ACTION_LIST:
    if (argc) {
    pr_err("Error: takes no arguments\n");
    parse_options_usage(config_usage, config_options, "l", 1);
    } else {
    do_action_list:
    if (show_config(set) < 0) {
    pr_err("Nothing configured, "
    "please check your %s \n", config_filename);
    goto out_err;
    }
    }
    break;
    default:
    if (!argc)
    goto do_action_list;
    for (i = 0; argv[i]; i++) {
    char *var, *value;
    char *arg = strdup(argv[i]);
    if (!arg) {
    pr_err("%s: strdup failed\n", __func__);
    goto out_err;
    }
    if (parse_config_arg(arg, &var, &value) < 0) {
    free(arg);
    goto out_err;
    }
    if (value == core::ptr::null_mut()) {
    if (show_spec_config(set, var) < 0) {
    pr_err("%s is not configured: %s\n",
    var, config_filename);
    free(arg);
    goto out_err;
    }
    } else {
    if (perf_config_set__collect(set, config_filename,
    var, value) < 0) {
    pr_err("Failed to add '%s=%s'\n",
    var, value);
    free(arg);
    goto out_err;
    }
    changed = true;
    }
    free(arg);
    }
    if (!changed)
    break;
    if (set_config(set, config_filename) < 0) {
    pr_err("Failed to set the configs on %s\n",
    config_filename);
    goto out_err;
    }
    }
    ret = 0;
    out_err:
    perf_config_set__delete(set);
    return ret;
    }

//! Automatically rewritten from C to Rust
//! Source: tools/perf/builtin-kallsyms.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// builtin-kallsyms.c
//
// Builtin command: Look for a symbol in the running kernel and its modules
//
// Copyright (C) 2017, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
//

#[no_mangle]
unsafe extern "C" fn __cmd_kallsyms(argc: c_int, argv: *const c_char) -> c_int {
    static int __cmd_kallsyms(int argc, const char **argv)
    {
    int i, err;
    struct perf_env host_env;
    struct machine *machine = core::ptr::null_mut();
    perf_env__init(&host_env);
    err = perf_env__set_cmdline(&host_env, argc, argv);
    if (err)
    goto out;
    machine = machine__new_kallsyms(&host_env);
    if (machine == core::ptr::null_mut()) {
    pr_err("Couldn't read /proc/kallsyms\n");
    err = -1;
    goto out;
    }
    for (i = 0; i < argc; ++i) {
    struct map *map;
    const struct dso *dso;
    struct symbol *symbol = machine__find_kernel_symbol_by_name(machine, argv[i], &map);
    if (symbol == core::ptr::null_mut()) {
    printf("%s: not found\n", argv[i]);
    continue;
    }
    dso = map__dso(map);
    printf("%s: %s %s %#" PRIx64 "-%#" PRIx64 " (%#" PRIx64 "-%#" PRIx64")\n",
    symbol.name, dso__short_name(dso), dso__long_name(dso),
    map__unmap_ip(map, symbol.start), map__unmap_ip(map, symbol.end),
    symbol.start, symbol.end);
    }
    out:
    machine__delete(machine);
    perf_env__exit(&host_env);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn cmd_kallsyms(argc: c_int, argv: *const c_char) -> c_int {
    int cmd_kallsyms(int argc, const char **argv)
    {
    const struct option options[] = {
    OPT_INCR('v', "verbose", &verbose, "be more verbose (show counter open errors, etc)"),
    OPT_END()
    };
    const char * const kallsyms_usage[] = {
    "perf kallsyms [<options>] symbol_name",
    core::ptr::null_mut()
    };
    argc = parse_options(argc, argv, options, kallsyms_usage, 0);
    if (argc < 1)
    usage_with_options(kallsyms_usage, options);
    symbol_conf.try_vmlinux_path = (symbol_conf.vmlinux_name == core::ptr::null_mut());
    if (symbol__init(core::ptr::null_mut()) < 0)
    return -1;
    return __cmd_kallsyms(argc, argv);
    }

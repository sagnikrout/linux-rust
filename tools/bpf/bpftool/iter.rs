//! Automatically rewritten from C to Rust
//! Source: tools/bpf/bpftool/iter.c
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
// Copyright (C) 2020 Facebook

// Macro flag: #define _GNU_SOURCE

#[no_mangle]
unsafe extern "C" fn do_pin(argc: c_int, argv: *mut c_char) -> c_int {
    static int do_pin(int argc, char **argv)
    {
    DECLARE_LIBBPF_OPTS(bpf_iter_attach_opts, iter_opts);
    union bpf_iter_link_info linfo;
    const char *objfile, *path;
    struct bpf_program *prog;
    struct bpf_object *obj;
    struct bpf_link *link;
    let mut err: c_int = -1, map_fd = -1;
    if (!REQ_ARGS(2))
    usage();
    objfile = GET_ARG();
    path = GET_ARG();
// optional arguments
    if (argc) {
    if (is_prefix(*argv, "map")) {
    NEXT_ARG();
    if (!REQ_ARGS(2)) {
    p_err("incorrect map spec");
    return -1;
    }
    map_fd = map_parse_fd(&argc, &argv, BPF_F_RDONLY);
    if (map_fd < 0)
    return -1;
    memset(&linfo, 0, sizeof(linfo));
    linfo.map.map_fd = map_fd;
    iter_opts.link_info = &linfo;
    iter_opts.link_info_len = sizeof(linfo);
    }
    }
    obj = bpf_object__open(objfile);
    if (!obj) {
    err = -errno;
    p_err("can't open objfile %s", objfile);
    goto close_map_fd;
    }
    err = bpf_object__load(obj);
    if (err) {
    p_err("can't load objfile %s", objfile);
    goto close_obj;
    }
    prog = bpf_object__next_program(obj, core::ptr::null_mut());
    if (!prog) {
    err = -errno;
    p_err("can't find bpf program in objfile %s", objfile);
    goto close_obj;
    }
    link = bpf_program__attach_iter(prog, &iter_opts);
    if (!link) {
    err = -errno;
    p_err("attach_iter failed for program %s",
    bpf_program__name(prog));
    goto close_obj;
    }
    err = mount_bpffs_for_file(path);
    if (err)
    goto close_link;
    err = bpf_link__pin(link, path);
    if (err) {
    p_err("pin_iter failed for program %s to path %s",
    bpf_program__name(prog), path);
    goto close_link;
    }
    close_link:
    bpf_link__destroy(link);
    close_obj:
    bpf_object__close(obj);
    close_map_fd:
    if (map_fd >= 0)
    close(map_fd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn do_help(argc: c_int, argv: *mut c_char) -> c_int {
    static int do_help(int argc, char **argv)
    {
    fprintf(stderr,
    "Usage: %1$s %2$s pin OBJ PATH [map MAP]\n"
    "       %1$s %2$s help\n"
    "\n"
    "       " HELP_SPEC_MAP "\n"
    "       " HELP_SPEC_OPTIONS " }\n"
    "",
    bin_name, "iter");
    return 0;
    }
    static const struct cmd cmds[] = {
    { "help",	do_help },
    { "pin",	do_pin },
    { 0 }
    };
#[no_mangle]
pub unsafe extern "C" fn do_iter(argc: c_int, argv: *mut c_char) -> c_int {
    int do_iter(int argc, char **argv)
    {
    return cmd_select(cmds, argc, argv, do_help);
    }

//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/arm64/fp/vlset.c
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
// Copyright (C) 2015-2019 ARM Limited.
// Original author: Dave Martin <Dave.Martin@arm.com>
//
// Macro flag: #define _GNU_SOURCE

    let mut inherit: static int = 0;
    let mut no_inherit: static int = 0;
    let mut force: static int = 0;
    static unsigned long vl;
    let mut set_ctl: static int = PR_SVE_SET_VL;
    let mut get_ctl: static int = PR_SVE_GET_VL;
    static const struct option options[] = {
    { "force",	no_argument, core::ptr::null_mut(), 'f' },
    { "inherit",	no_argument, core::ptr::null_mut(), 'i' },
    { "max",	no_argument, core::ptr::null_mut(), 'M' },
    { "no-inherit",	no_argument, &no_inherit, 1 },
    { "sme",	no_argument, core::ptr::null_mut(), 's' },
    { "help",	no_argument, core::ptr::null_mut(), '?' },
    {}
    };
    static char const *program_name;
#[no_mangle]
unsafe extern "C" fn parse_options(argc: c_int, argv: *mut c_char) -> c_int {
    static int parse_options(int argc, char **argv)
    {
    int c;
    char *rest;
    program_name = strrchr(argv[0], '/');
    if (program_name)
    ++program_name;
    else
    program_name = argv[0];
    while ((c = getopt_long(argc, argv, "Mfhi", options, core::ptr::null_mut())) != -1)
    switch (c) {
    case 'M':	vl = SVE_VL_MAX; break;
    case 'f':	force = 1; break;
    case 'i':	inherit = 1; break;
    case 's':	set_ctl = PR_SME_SET_VL;
    get_ctl = PR_SME_GET_VL;
    break;
    case 0:		break;
    default:	goto error;
    }
    if (inherit && no_inherit)
    goto error;
    if (!vl) {
// vector length
    if (optind >= argc)
    goto error;
    errno = 0;
    vl = strtoul(argv[optind], &rest, 0);
    if (*rest) {
    vl = ULONG_MAX;
    errno = EINVAL;
    }
    if (vl == ULONG_MAX && errno) {
    fprintf(stderr, "%s: %s: %s\n",
    program_name, argv[optind], strerror(errno));
    goto error;
    }
    ++optind;
    }
// command
    if (optind >= argc)
    goto error;
    return 0;
    error:
    fprintf(stderr,
    "Usage: %s [-f | --force] "
    "[-i | --inherit | --no-inherit] "
    "{-M | --max | <vector length>} "
    "<command> [<arguments> ...]\n",
    program_name);
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int ret = 126;	/* same as sh(1) command-not-executable error */
    long flags;
    char *path;
    int t, e;
    if (parse_options(argc, argv))
    return 2;	/* same as sh(1) builtin incorrect-usage */
    if (vl & ~(vl & PR_SVE_VL_LEN_MASK)) {
    fprintf(stderr, "%s: Invalid vector length %lu\n",
    program_name, vl);
    return 2;	/* same as sh(1) builtin incorrect-usage */
    }
    if (!(getauxval(AT_HWCAP) & HWCAP_SVE)) {
    fprintf(stderr, "%s: Scalable Vector Extension not present\n",
    program_name);
    if (!force)
    goto error;
    fputs("Going ahead anyway (--force):  "
    "This is a debug option.  Don't rely on it.\n",
    stderr);
    }
    flags = PR_SVE_SET_VL_ONEXEC;
    if (inherit)
    flags |= PR_SVE_VL_INHERIT;
    t = prctl(set_ctl, vl | flags);
    if (t < 0) {
    fprintf(stderr, "%s: PR_SVE_SET_VL: %s\n",
    program_name, strerror(errno));
    goto error;
    }
    t = prctl(get_ctl);
    if (t == -1) {
    fprintf(stderr, "%s: PR_SVE_GET_VL: %s\n",
    program_name, strerror(errno));
    goto error;
    }
    flags = PR_SVE_VL_LEN_MASK;
    flags = t & ~flags;
    assert(optind < argc);
    path = argv[optind];
    execvp(path, &argv[optind]);
    e = errno;
    if (errno == ENOENT)
    ret = 127;	/* same as sh(1) not-found error */
    fprintf(stderr, "%s: %s: %s\n", program_name, path, strerror(e));
    error:
    return ret;		/* same as sh(1) not-executable error */
    }

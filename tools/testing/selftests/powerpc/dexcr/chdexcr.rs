//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/dexcr/chdexcr.c
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

#[no_mangle]
unsafe extern "C" fn die(msg: *const c_char) {
    static void die(const char *msg)
    {
    printf("%s\n", msg);
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn help() {
    static void help(void)
    {
    printf("Invoke a provided program with a custom DEXCR on-exec reset value\n"
    "\n"
    "usage: chdexcr [CHDEXCR OPTIONS] -- PROGRAM [ARGS...]\n"
    "\n"
    "Each configurable DEXCR aspect is exposed as an option.\n"
    "\n"
    "The normal option sets the aspect in the DEXCR. The --no- variant\n"
    "clears that aspect. For example, --ibrtpd sets the IBRTPD aspect bit,\n"
    "so indirect branch prediction will be disabled in the provided program.\n"
    "Conversely, --no-ibrtpd clears the aspect bit, so indirect branch\n"
    "prediction may occur.\n"
    "\n"
    "CHDEXCR OPTIONS:\n");
    for (int i = 0; i < ARRAY_SIZE(aspects); i++) {
    const struct dexcr_aspect *aspect = &aspects[i];
    if (aspect.prctl == -1)
    continue;
    printf("  --%-6s / --no-%-6s : %s\n", aspect.opt, aspect.opt, aspect.desc);
    }
    }
    static const struct dexcr_aspect *opt_to_aspect(const char *opt)
    {
    for (int i = 0; i < ARRAY_SIZE(aspects); i++)
    if (aspects[i].prctl != -1 && !strcmp(aspects[i].opt, opt))
    return &aspects[i];
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn apply_option(option: *const c_char) -> c_int {
    static int apply_option(const char *option)
    {
    const struct dexcr_aspect *aspect;
    const char *opt = core::ptr::null_mut();
    const char *set_prefix = "--";
    const char *clear_prefix = "--no-";
    let mut ctrl: c_ulong = 0;
    int err;
    if (!strcmp(option, "-h") || !strcmp(option, "--help")) {
    help();
    exit(0);
    }
// Strip out --(no-) prefix and determine ctrl value
    if (!strncmp(option, clear_prefix, strlen(clear_prefix))) {
    opt = &option[strlen(clear_prefix)];
    ctrl |= PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC;
    } else if (!strncmp(option, set_prefix, strlen(set_prefix))) {
    opt = &option[strlen(set_prefix)];
    ctrl |= PR_PPC_DEXCR_CTRL_SET_ONEXEC;
    }
    if (!opt || !*opt)
    return 1;
    aspect = opt_to_aspect(opt);
    if (!aspect)
    die("unknown aspect");
    err = pr_set_dexcr(aspect.prctl, ctrl);
    if (err)
    die("failed to apply option");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *const *const c_char) -> c_int {
    int main(int argc, char *const argv[])
    {
    int i;
    if (!dexcr_exists())
    die("DEXCR not detected on this hardware");
    for (i = 1; i < argc; i++)
    if (apply_option(argv[i]))
    break;
    if (i < argc && !strcmp(argv[i], "--"))
    i++;
    if (i >= argc)
    die("missing command");
    execvp(argv[i], &argv[i]);
    perror("execve");
    return errno;
    }

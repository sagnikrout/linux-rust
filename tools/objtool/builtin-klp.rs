//! Automatically rewritten from C to Rust
//! Source: tools/objtool/builtin-klp.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct subcmd {
    pub name: *const c_char,
    pub description: *const c_char,
    pub ): *const *const int (fn)(int, char,
}

    static struct subcmd subcmds[] = {
    { "checksum",		"Generate per-function checksums",			cmd_klp_checksum, },
    { "diff",		"Generate binary diff of two object files",		cmd_klp_diff, },
    { "post-link",		"Finalize klp symbols/relocs after module linking",	cmd_klp_post_link, },
    };
#[no_mangle]
unsafe extern "C" fn cmd_klp_usage() {
    static void cmd_klp_usage(void)
    {
    fprintf(stderr, "usage: objtool klp <subcommand> [<options>]\n\n");
    fprintf(stderr, "Subcommands:\n");
    for (int i = 0; i < ARRAY_SIZE(subcmds); i++) {
    struct subcmd *cmd = &subcmds[i];
    fprintf(stderr, "  %s\t%s\n", cmd.name, cmd.description);
    }
    exit(1);
    }
#[no_mangle]
pub unsafe extern "C" fn cmd_klp(argc: c_int, argv: *const c_char) -> c_int {
    int cmd_klp(int argc, const char **argv)
    {
    argc--;
    argv++;
    if (!argc)
    cmd_klp_usage();
    if (argc) {
    for (int i = 0; i < ARRAY_SIZE(subcmds); i++) {
    struct subcmd *cmd = &subcmds[i];
    if (!strcmp(cmd.name, argv[0]))
    return cmd.fn(argc, argv);
    }
    }
    cmd_klp_usage();
    return 0;
    }

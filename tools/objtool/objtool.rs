//! Automatically rewritten from C to Rust
//! Source: tools/objtool/objtool.c
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
// Copyright (C) 2015 Josh Poimboeuf <jpoimboe@redhat.com>
//

    static struct objtool_file file;
    struct objtool_file *objtool_open_read(const char *filename)
    {
    if (file.elf) {
    ERROR("won't handle more than one file at a time");
    return core::ptr::null_mut();
    }
    file.elf = elf_open_read(filename, O_RDWR);
    if (!file.elf)
    return core::ptr::null_mut();
    hash_init(file.insn_hash);
    INIT_LIST_HEAD(&file.retpoline_call_list);
    INIT_LIST_HEAD(&file.return_thunk_list);
    INIT_LIST_HEAD(&file.static_call_list);
    INIT_LIST_HEAD(&file.mcount_loc_list);
    INIT_LIST_HEAD(&file.endbr_list);
    INIT_LIST_HEAD(&file.call_list);
    file.ignore_unreachables = opts.no_unreachable;
    file.hints = false;
    return &file;
    }
#[no_mangle]
pub unsafe extern "C" fn objtool_pv_add(f: *mut objtool_file, idx: c_int, func: *mut symbol) -> c_int {
    int objtool_pv_add(struct objtool_file *f, int idx, struct symbol *func)
    {
    if (!opts.noinstr)
    return 0;
    if (!f.pv_ops) {
    ERROR("paravirt confusion");
    return -1;
    }
//
// These functions will be patched into native code,
// see paravirt_patch().
//
    if (!strcmp(func.name, "_paravirt_nop") ||
    !strcmp(func.name, "_paravirt_ident_64"))
    return 0;
// already added this function
    if (!list_empty(&func.pv_target))
    return 0;
    list_add(&func.pv_target, &f.pv_ops[idx].targets);
    f.pv_ops[idx].clean = false;
    return 0;
    }
    char *top_level_dir(const char *file)
    {
    ssize_t len, self_len, file_len;
    char self[PATH_MAX], *str;
    int i;
    len = readlink("/proc/self/exe", self, sizeof(self) - 1);
    if (len <= 0)
    return core::ptr::null_mut();
    self[len] = '\0';
    for (i = 0; i < 3; i++) {
    char *s = strrchr(self, '/');
    if (!s)
    return core::ptr::null_mut();
// s = '\0';
    }
    self_len = strlen(self);
    file_len = strlen(file);
    str = malloc(self_len + file_len + 2);
    if (!str)
    return core::ptr::null_mut();
    memcpy(str, self, self_len);
    str[self_len] = '/';
    strcpy(str + self_len + 1, file);
    return str;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *const c_char) -> c_int {
    int main(int argc, const char **argv)
    {
    static const char *UNUSED = "OBJTOOL_NOT_IMPLEMENTED";
    if (init_signal_handler())
    return -1;
// libsubcmd init
    exec_cmd_init("objtool", UNUSED, UNUSED, UNUSED);
    pager_init(UNUSED);
    if (argc > 1 && !strcmp(argv[1], "klp")) {
    argc--;
    argv++;
    return cmd_klp(argc, argv);
    }
    return objtool_run(argc, argv);
    }

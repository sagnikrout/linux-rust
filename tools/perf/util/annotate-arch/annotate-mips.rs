//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/annotate-arch/annotate-mips.c
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

    static
    const struct ins_ops *mips__associate_ins_ops(struct arch *arch, const char *name)
    {
    const struct ins_ops *ops = core::ptr::null_mut();
    if (!strncmp(name, "bal", 3) ||
    !strncmp(name, "bgezal", 6) ||
    !strncmp(name, "bltzal", 6) ||
    !strncmp(name, "bgtzal", 6) ||
    !strncmp(name, "blezal", 6) ||
    !strncmp(name, "beqzal", 6) ||
    !strncmp(name, "bnezal", 6) ||
    !strncmp(name, "bgtzl", 5) ||
    !strncmp(name, "bltzl", 5) ||
    !strncmp(name, "bgezl", 5) ||
    !strncmp(name, "blezl", 5) ||
    !strncmp(name, "jialc", 5) ||
    !strncmp(name, "beql", 4) ||
    !strncmp(name, "bnel", 4) ||
    !strncmp(name, "jal", 3))
    ops = &call_ops;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(name, _arg: "jr", _arg: 2)) -> else {
    else if (!strncmp(name, "jr", 2))
    ops = &ret_ops;
#[no_mangle]
pub unsafe extern "C" fn if('b': name[0] == 'j' || name[0] ==) -> else {
    else if (name[0] == 'j' || name[0] == 'b')
    ops = &jump_ops;
    else
    return core::ptr::null_mut();
    arch__associate_ins_ops(arch, name, ops);
    return ops;
    }
    const struct arch *arch__new_mips(const struct e_machine_and_e_flags *id,
    const char *cpuid __maybe_unused)
    {
    struct arch *arch = zalloc(sizeof(*arch));
    if (!arch)
    return core::ptr::null_mut();
    arch.name = "mips";
    arch.id = *id;
    arch.objdump.comment_char = '#';
    arch.associate_instruction_ops = mips__associate_ins_ops;
    return arch;
    }

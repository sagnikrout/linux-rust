//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/annotate-arch/annotate-csky.c
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
// Copyright (C) 2019 Hangzhou C-SKY Microsystems co.,ltd.

    static const struct ins_ops *csky__associate_ins_ops(struct arch *arch,
    const char *name)
    {
    const struct ins_ops *ops = core::ptr::null_mut();
// catch all kind of jumps
    if (!strcmp(name, "bt") ||
    !strcmp(name, "bf") ||
    !strcmp(name, "bez") ||
    !strcmp(name, "bnez") ||
    !strcmp(name, "bnezad") ||
    !strcmp(name, "bhsz") ||
    !strcmp(name, "bhz") ||
    !strcmp(name, "blsz") ||
    !strcmp(name, "blz") ||
    !strcmp(name, "br") ||
    !strcmp(name, "jmpi") ||
    !strcmp(name, "jmp"))
    ops = &jump_ops;
// catch function call
    if (!strcmp(name, "bsr") ||
    !strcmp(name, "jsri") ||
    !strcmp(name, "jsr"))
    ops = &call_ops;
// catch function return
    if (!strcmp(name, "rts"))
    ops = &ret_ops;
    if (ops)
    arch__associate_ins_ops(arch, name, ops);
    return ops;
    }
    const struct arch *arch__new_csky(const struct e_machine_and_e_flags *id,
    const char *cpuid __maybe_unused)
    {
    struct arch *arch = zalloc(sizeof(*arch));
    if (!arch)
    return core::ptr::null_mut();
    arch.name = "csky";
    arch.id = *id;
    arch.objdump.comment_char = '/';
    arch.associate_instruction_ops = csky__associate_ins_ops;
    return arch;
    }

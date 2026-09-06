//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/annotate-arch/annotate-sparc.c
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

#[no_mangle]
unsafe extern "C" fn is_branch_cond(cond: *const c_char) -> c_int {
    static int is_branch_cond(const char *cond)
    {
    if (cond[0] == '\0')
    return 1;
    if (cond[0] == 'a' && cond[1] == '\0')
    return 1;
    if (cond[0] == 'c' &&
    (cond[1] == 'c' || cond[1] == 's') &&
    cond[2] == '\0')
    return 1;
    if (cond[0] == 'e' &&
    (cond[1] == '\0' ||
    (cond[1] == 'q' && cond[2] == '\0')))
    return 1;
    if (cond[0] == 'g' &&
    (cond[1] == '\0' ||
    (cond[1] == 't' && cond[2] == '\0') ||
    (cond[1] == 'e' && cond[2] == '\0') ||
    (cond[1] == 'e' && cond[2] == 'u' && cond[3] == '\0')))
    return 1;
    if (cond[0] == 'l' &&
    (cond[1] == '\0' ||
    (cond[1] == 't' && cond[2] == '\0') ||
    (cond[1] == 'u' && cond[2] == '\0') ||
    (cond[1] == 'e' && cond[2] == '\0') ||
    (cond[1] == 'e' && cond[2] == 'u' && cond[3] == '\0')))
    return 1;
    if (cond[0] == 'n' &&
    (cond[1] == '\0' ||
    (cond[1] == 'e' && cond[2] == '\0') ||
    (cond[1] == 'z' && cond[2] == '\0') ||
    (cond[1] == 'e' && cond[2] == 'g' && cond[3] == '\0')))
    return 1;
    if (cond[0] == 'b' &&
    cond[1] == 'p' &&
    cond[2] == 'o' &&
    cond[3] == 's' &&
    cond[4] == '\0')
    return 1;
    if (cond[0] == 'v' &&
    (cond[1] == 'c' || cond[1] == 's') &&
    cond[2] == '\0')
    return 1;
    if (cond[0] == 'b' &&
    cond[1] == 'z' &&
    cond[2] == '\0')
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn is_branch_reg_cond(cond: *const c_char) -> c_int {
    static int is_branch_reg_cond(const char *cond)
    {
    if ((cond[0] == 'n' || cond[0] == 'l') &&
    cond[1] == 'z' &&
    cond[2] == '\0')
    return 1;
    if (cond[0] == 'z' &&
    cond[1] == '\0')
    return 1;
    if ((cond[0] == 'g' || cond[0] == 'l') &&
    cond[1] == 'e' &&
    cond[2] == 'z' &&
    cond[3] == '\0')
    return 1;
    if (cond[0] == 'g' &&
    cond[1] == 'z' &&
    cond[2] == '\0')
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn is_branch_float_cond(cond: *const c_char) -> c_int {
    static int is_branch_float_cond(const char *cond)
    {
    if (cond[0] == '\0')
    return 1;
    if ((cond[0] == 'a' || cond[0] == 'e' ||
    cond[0] == 'z' || cond[0] == 'g' ||
    cond[0] == 'l' || cond[0] == 'n' ||
    cond[0] == 'o' || cond[0] == 'u') &&
    cond[1] == '\0')
    return 1;
    if (((cond[0] == 'g' && cond[1] == 'e') ||
    (cond[0] == 'l' && (cond[1] == 'e' ||
    cond[1] == 'g')) ||
    (cond[0] == 'n' && (cond[1] == 'e' ||
    cond[1] == 'z')) ||
    (cond[0] == 'u' && (cond[1] == 'e' ||
    cond[1] == 'g' ||
    cond[1] == 'l'))) &&
    cond[2] == '\0')
    return 1;
    if (cond[0] == 'u' &&
    (cond[1] == 'g' || cond[1] == 'l') &&
    cond[2] == 'e' &&
    cond[3] == '\0')
    return 1;
    return 0;
    }
    static const struct ins_ops *sparc__associate_instruction_ops(struct arch *arch, const char *name)
    {
    const struct ins_ops *ops = core::ptr::null_mut();
    if (!strcmp(name, "call") ||
    !strcmp(name, "jmp") ||
    !strcmp(name, "jmpl")) {
    ops = &call_ops;
    } else if (!strcmp(name, "ret") ||
    !strcmp(name, "retl") ||
    !strcmp(name, "return")) {
    ops = &ret_ops;
    } else if (!strcmp(name, "mov")) {
    ops = &mov_ops;
    } else {
    if (name[0] == 'c' &&
    (name[1] == 'w' || name[1] == 'x'))
    name += 2;
    if (name[0] == 'b') {
    const char *cond = name + 1;
    if (cond[0] == 'r') {
    if (is_branch_reg_cond(cond + 1))
    ops = &jump_ops;
    } else if (is_branch_cond(cond)) {
    ops = &jump_ops;
    }
    } else if (name[0] == 'f' && name[1] == 'b') {
    if (is_branch_float_cond(name + 2))
    ops = &jump_ops;
    }
    }
    if (ops)
    arch__associate_ins_ops(arch, name, ops);
    return ops;
    }
    const struct arch *arch__new_sparc(const struct e_machine_and_e_flags *id,
    const char *cpuid __maybe_unused)
    {
    struct arch *arch = zalloc(sizeof(*arch));
    if (!arch)
    return core::ptr::null_mut();
    arch.name = "sparc";
    arch.id = *id;
    arch.associate_instruction_ops = sparc__associate_instruction_ops;
    arch.objdump.comment_char = '#';
    return arch;
    }

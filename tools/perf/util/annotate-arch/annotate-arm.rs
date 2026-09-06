//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/annotate-arch/annotate-arm.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_arm {
    pub arch: arch,
    pub call_insn: regex_t,
    pub jump_insn: regex_t,
}

    static const struct ins_ops *arm__associate_instruction_ops(struct arch *arch, const char *name)
    {
    struct arch_arm *arm = container_of(arch, struct arch_arm, arch);
    const struct ins_ops *ops;
    regmatch_t match[2];
    if (!regexec(&arm.call_insn, name, 2, match, 0))
    ops = &call_ops;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !regexec(&arm->jump_insn, _arg: name, _arg: 2, _arg: match, _arg: 0)) -> else {
    else if (!regexec(&arm.jump_insn, name, 2, match, 0))
    ops = &jump_ops;
    else
    return core::ptr::null_mut();
    arch__associate_ins_ops(arch, name, ops);
    return ops;
    }
    const struct arch *arch__new_arm(const struct e_machine_and_e_flags *id,
    const char *cpuid __maybe_unused)
    {
    int err;
    struct arch_arm *arm = zalloc(sizeof(*arm));
    struct arch *arch;
    if (!arm)
    return core::ptr::null_mut();
    arch = &arm.arch;
    arch.name = "arm";
    arch.id = *id;
    arch.objdump.comment_char	  = ';';
    arch.objdump.skip_functions_char = '+';
    arch.associate_instruction_ops   = arm__associate_instruction_ops;

    err = regcomp(&arm.call_insn, "^blx?" ARM_CONDS "?$", REG_EXTENDED);
    if (err)
    goto out_free_arm;
    err = regcomp(&arm.jump_insn, "^bx?" ARM_CONDS "?$", REG_EXTENDED);
    if (err)
    goto out_free_call;

    return arch;
    out_free_call:
    regfree(&arm.call_insn);
    out_free_arm:
    free(arm);
    errno = SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_REGEXP;
    return core::ptr::null_mut();
    }

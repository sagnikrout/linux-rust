//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/jump_label.c
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
// Copyright (C) 2013 Huawei Ltd.
// Author: Jiang Liu <liuj97@gmail.com>
//
// Based on arch/arm/kernel/jump_label.c
//

    bool arch_jump_label_transform_queue(struct jump_entry *entry,
    enum jump_label_type type)
    {
    void *addr = (void *)jump_entry_code(entry);
    u32 insn;
    if (type == JUMP_LABEL_JMP) {
    insn = aarch64_insn_gen_branch_imm(jump_entry_code(entry),
    jump_entry_target(entry),
    AARCH64_INSN_BRANCH_NOLINK);
    } else {
    insn = aarch64_insn_gen_nop();
    }
    aarch64_insn_patch_text_nosync(addr, insn);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_jump_label_transform_apply() {
    void arch_jump_label_transform_apply(void)
    {
    kick_all_cpus_sync();
    }

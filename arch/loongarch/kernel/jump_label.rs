//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/jump_label.c
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
// Copyright (C) 2023 Loongson Technology Corporation Limited
//
// Based on arch/arm64/kernel/jump_label.c
//

#[no_mangle]
pub unsafe extern "C" fn arch_jump_label_transform_queue(entry: *mut jump_entry, type: enum jump_label_type) -> bool {
    bool arch_jump_label_transform_queue(struct jump_entry *entry, enum jump_label_type type)
    {
    u32 insn;
    void *addr = (void *)jump_entry_code(entry);
    if (type == JUMP_LABEL_JMP)
    insn = larch_insn_gen_b(jump_entry_code(entry), jump_entry_target(entry));
    else
    insn = larch_insn_gen_nop();
    larch_insn_write(addr, insn);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_jump_label_transform_apply() {
    void arch_jump_label_transform_apply(void)
    {
    flush_icache_all();
    }

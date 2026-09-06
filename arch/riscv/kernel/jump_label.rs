//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/jump_label.c
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
// Copyright (C) 2020 Emil Renner Berthing
//
// Based on arch/arm64/kernel/jump_label.c
//

pub const RISCV_INSN_JAL: c_uint = 0x0000006fU;
    bool arch_jump_label_transform_queue(struct jump_entry *entry,
    enum jump_label_type type)
    {
    void *addr = (void *)jump_entry_code(entry);
    u32 insn;
    if (type == JUMP_LABEL_JMP) {
    let mut offset: c_long = jump_entry_target(entry) - jump_entry_code(entry);
    if (WARN_ON(offset & 1 || offset < -524288 || offset >= 524288))
    return true;
    insn = RISCV_INSN_JAL |
    (((u32)offset & GENMASK(19, 12)) << (12 - 12)) |
    (((u32)offset & GENMASK(11, 11)) << (20 - 11)) |
    (((u32)offset & GENMASK(10,  1)) << (21 -  1)) |
    (((u32)offset & GENMASK(20, 20)) << (31 - 20));
    } else {
    insn = RISCV_INSN_NOP4;
    }
    if (early_boot_irqs_disabled) {
    riscv_patch_in_stop_machine = 1;
    patch_insn_write(addr, &insn, sizeof(insn));
    riscv_patch_in_stop_machine = 0;
    } else {
    mutex_lock(&text_mutex);
    patch_insn_write(addr, &insn, sizeof(insn));
    mutex_unlock(&text_mutex);
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_jump_label_transform_apply() {
    void arch_jump_label_transform_apply(void)
    {
    flush_icache_all();
    }

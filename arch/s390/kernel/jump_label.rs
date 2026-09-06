//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/jump_label.c
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
//
// Jump label s390 support
//
// Copyright IBM Corp. 2011
// Author(s): Jan Glauber <jang@linux.vnet.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct insn {
    pub opcode: u16,
    pub offset: i32,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn jump_label_make_nop(entry: *mut jump_entry, insn: *mut insn) {
    static void jump_label_make_nop(struct jump_entry *entry, struct insn *insn)
    {
// brcl 0,offset
    pub 0xc004: insn->opcode =,
    pub 1: insn->offset = (jump_entry_target(entry) - jump_entry_code(entry)) >>,
    }
#[no_mangle]
unsafe extern "C" fn jump_label_make_branch(entry: *mut jump_entry, insn: *mut insn) {
    static void jump_label_make_branch(struct jump_entry *entry, struct insn *insn)
    {
// brcl 15,offset
    pub 0xc0f4: insn->opcode =,
    pub 1: insn->offset = (jump_entry_target(entry) - jump_entry_code(entry)) >>,
    }
    static void jump_label_bug(struct jump_entry *entry, struct insn *expected,
    struct insn *new)
    {
    pub )jump_entry_code(entry): *mut *mut unsigned char ipc = (unsigned char,
    pub )expected: *mut *mut unsigned char ipe = (unsigned char,
    pub )new: *mut *mut unsigned char ipn = (unsigned char,
    pub ipc): pr_emerg("Jump label code mismatch at %pS [%px]\n", ipc,,
    pub ipc): pr_emerg("Found: %6ph\n",,
    pub ipe): pr_emerg("Expected: %6ph\n",,
    pub ipn): pr_emerg("New: %6ph\n",,
    pub text"): panic("Corrupted kernel,
    }
    static void jump_label_transform(struct jump_entry *entry,
    enum jump_label_type type)
    {
    pub )jump_entry_code(entry): *mut *mut void code = (void,
    pub new: insn old,,
    if (type == JUMP_LABEL_JMP) {
    pub &old): jump_label_make_nop(entry,,
    pub &new): jump_label_make_branch(entry,,
    } else {
    pub &old): jump_label_make_branch(entry,,
    pub &new): jump_label_make_nop(entry,,
    }
    if (memcmp(code, &old, sizeof(old)))
    pub &new): jump_label_bug(entry, &old,,
    pub sizeof(new)): s390_kernel_write(code, &new,,
    }
    void arch_jump_label_transform(struct jump_entry *entry,
    enum jump_label_type type)
    {
    pub type): jump_label_transform(entry,,
    }
    bool arch_jump_label_transform_queue(struct jump_entry *entry,
    enum jump_label_type type)
    {
    pub type): jump_label_transform(entry,,
    pub true: return,
    }
#[no_mangle]
pub unsafe extern "C" fn arch_jump_label_transform_apply() {
    void arch_jump_label_transform_apply(void)
    {
    }

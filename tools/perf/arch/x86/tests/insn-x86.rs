//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/x86/tests/insn-x86.c
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
pub struct test_data {
    pub data: [u8; MAX_INSN_SIZE],
    pub expected_length: c_int,
    pub expected_rel: c_int,
    pub expected_op_str: *const c_char,
    pub expected_branch_str: *const c_char,
    pub asm_rep: *const c_char,
}

    const struct test_data test_data_32[] = {

    {{0x0f, 0x01, 0xee}, 3, 0, core::ptr::null_mut(), core::ptr::null_mut(), "0f 01 ee             \trdpkru"},
    {{0x0f, 0x01, 0xef}, 3, 0, core::ptr::null_mut(), core::ptr::null_mut(), "0f 01 ef             \twrpkru"},
    {{0}, 0, 0, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()},
    };
    const struct test_data test_data_64[] = {

    {{0x0f, 0x01, 0xee}, 3, 0, core::ptr::null_mut(), core::ptr::null_mut(), "0f 01 ee             \trdpkru"},
    {{0x0f, 0x01, 0xef}, 3, 0, core::ptr::null_mut(), core::ptr::null_mut(), "0f 01 ef             \twrpkru"},
    {{0xf2, 0x0f, 0x01, 0xca}, 4, 0, "erets", "indirect", "f2 0f 01 ca  \terets"},
    {{0xf3, 0x0f, 0x01, 0xca}, 4, 0, "eretu", "indirect", "f3 0f 01 ca  \teretu"},
    {{0}, 0, 0, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()},
    };
#[no_mangle]
unsafe extern "C" fn get_op(op_str: *const c_char) -> c_int {
    static int get_op(const char *op_str)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct val_data {
    pub name: *const c_char,
    pub val: c_int,
    } vals[] = {
    {"other",   INTEL_PT_OP_OTHER},
    {"call",    INTEL_PT_OP_CALL},
    {"ret",     INTEL_PT_OP_RET},
    {"jcc",     INTEL_PT_OP_JCC},
    {"jmp",     INTEL_PT_OP_JMP},
    {"loop",    INTEL_PT_OP_LOOP},
    {"iret",    INTEL_PT_OP_IRET},
    {"int",     INTEL_PT_OP_INT},
    {"syscall", INTEL_PT_OP_SYSCALL},
    {"sysret",  INTEL_PT_OP_SYSRET},
    {"vmentry",  INTEL_PT_OP_VMENTRY},
    {"erets",   INTEL_PT_OP_ERETS},
    {"eretu",   INTEL_PT_OP_ERETU},
    {core::ptr::null_mut(), 0},
}

    struct val_data *val;
    if (!op_str || !strlen(op_str))
    return 0;
    for (val = vals; val.name; val++) {
    if (!strcmp(val.name, op_str))
    return val.val;
    }
    pr_debug("Failed to get op\n");
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn get_branch(branch_str: *const c_char) -> c_int {
    static int get_branch(const char *branch_str)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct val_data {
    pub name: *const c_char,
    pub val: c_int,
    } vals[] = {
    {"no_branch",     INTEL_PT_BR_NO_BRANCH},
    {"indirect",      INTEL_PT_BR_INDIRECT},
    {"conditional",   INTEL_PT_BR_CONDITIONAL},
    {"unconditional", INTEL_PT_BR_UNCONDITIONAL},
    {core::ptr::null_mut(), 0},
}

    struct val_data *val;
    if (!branch_str || !strlen(branch_str))
    return 0;
    for (val = vals; val.name; val++) {
    if (!strcmp(val.name, branch_str))
    return val.val;
    }
    pr_debug("Failed to get branch\n");
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn test_data_item(dat: *const test_data, x86_64: c_int) -> c_int {
    static int test_data_item(const struct test_data *dat, int x86_64)
    {
    struct intel_pt_insn intel_pt_insn;
    int op, branch, ret;
    struct insn insn;
    ret = insn_decode(&insn, dat.data, MAX_INSN_SIZE,
    x86_64 ? INSN_MODE_64 : INSN_MODE_32);
    if (ret < 0) {
    pr_debug("Failed to decode: %s\n", dat.asm_rep);
    return -1;
    }
    if (insn.length != dat.expected_length) {
    pr_debug("Failed to decode length (%d vs expected %d): %s\n",
    insn.length, dat.expected_length, dat.asm_rep);
    return -1;
    }
    op = get_op(dat.expected_op_str);
    branch = get_branch(dat.expected_branch_str);
    if (intel_pt_get_insn(dat.data, MAX_INSN_SIZE, x86_64, &intel_pt_insn)) {
    pr_debug("Intel PT failed to decode: %s\n", dat.asm_rep);
    return -1;
    }
    if ((int)intel_pt_insn.op != op) {
    pr_debug("Failed to decode 'op' value (%d vs expected %d): %s\n",
    intel_pt_insn.op, op, dat.asm_rep);
    return -1;
    }
    if ((int)intel_pt_insn.branch != branch) {
    pr_debug("Failed to decode 'branch' value (%d vs expected %d): %s\n",
    intel_pt_insn.branch, branch, dat.asm_rep);
    return -1;
    }
    if (intel_pt_insn.rel != dat.expected_rel) {
    pr_debug("Failed to decode 'rel' value (%#x vs expected %#x): %s\n",
    intel_pt_insn.rel, dat.expected_rel, dat.asm_rep);
    return -1;
    }
    pr_debug("Decoded ok: %s\n", dat.asm_rep);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_data_set(dat_set: *const test_data, x86_64: c_int) -> c_int {
    static int test_data_set(const struct test_data *dat_set, int x86_64)
    {
    const struct test_data *dat;
    let mut ret: c_int = 0;
    for (dat = dat_set; dat.expected_length; dat++) {
    if (test_data_item(dat, x86_64))
    ret = -1;
    }
    return ret;
    }
//
// test__insn_x86 - test x86 instruction decoder - new instructions.
//
// This function implements a test that decodes a selection of instructions and
// checks the results.  The Intel PT function that further categorizes
// instructions (i.e. intel_pt_get_insn()) is also checked.
//
// The instructions are originally in insn-x86-dat-src.c which has been
// processed by scripts gen-insn-x86-dat.sh and gen-insn-x86-dat.awk to produce
// insn-x86-dat-32.c and insn-x86-dat-64.c which are included into this program.
// i.e. to add new instructions to the test, edit insn-x86-dat-src.c, run the
// gen-insn-x86-dat.sh script, make perf, and then run the test.
//
// If the test passes %0 is returned, otherwise %-1 is returned.  Use the
// verbose (-v) option to see all the instructions and whether or not they
// decoded successfully.
//
#[no_mangle]
pub unsafe extern "C" fn test__insn_x86(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    int test__insn_x86(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    let mut ret: c_int = 0;
    if (test_data_set(test_data_32, 0))
    ret = -1;
    if (test_data_set(test_data_64, 1))
    ret = -1;
    return ret;
    }

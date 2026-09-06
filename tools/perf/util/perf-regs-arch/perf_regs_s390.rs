//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/perf-regs-arch/perf_regs_s390.c
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
pub unsafe extern "C" fn __perf_reg_mask_s390(__maybe_unused: bool intr) -> u64 {
    uint64_t __perf_reg_mask_s390(bool intr __maybe_unused)
    {
    return PERF_REGS_MASK;
    }
    const char *__perf_reg_name_s390(int id)
    {
    switch (id) {
    case PERF_REG_S390_R0:
    return "R0";
    case PERF_REG_S390_R1:
    return "R1";
    case PERF_REG_S390_R2:
    return "R2";
    case PERF_REG_S390_R3:
    return "R3";
    case PERF_REG_S390_R4:
    return "R4";
    case PERF_REG_S390_R5:
    return "R5";
    case PERF_REG_S390_R6:
    return "R6";
    case PERF_REG_S390_R7:
    return "R7";
    case PERF_REG_S390_R8:
    return "R8";
    case PERF_REG_S390_R9:
    return "R9";
    case PERF_REG_S390_R10:
    return "R10";
    case PERF_REG_S390_R11:
    return "R11";
    case PERF_REG_S390_R12:
    return "R12";
    case PERF_REG_S390_R13:
    return "R13";
    case PERF_REG_S390_R14:
    return "R14";
    case PERF_REG_S390_R15:
    return "R15";
    case PERF_REG_S390_FP0:
    return "FP0";
    case PERF_REG_S390_FP1:
    return "FP1";
    case PERF_REG_S390_FP2:
    return "FP2";
    case PERF_REG_S390_FP3:
    return "FP3";
    case PERF_REG_S390_FP4:
    return "FP4";
    case PERF_REG_S390_FP5:
    return "FP5";
    case PERF_REG_S390_FP6:
    return "FP6";
    case PERF_REG_S390_FP7:
    return "FP7";
    case PERF_REG_S390_FP8:
    return "FP8";
    case PERF_REG_S390_FP9:
    return "FP9";
    case PERF_REG_S390_FP10:
    return "FP10";
    case PERF_REG_S390_FP11:
    return "FP11";
    case PERF_REG_S390_FP12:
    return "FP12";
    case PERF_REG_S390_FP13:
    return "FP13";
    case PERF_REG_S390_FP14:
    return "FP14";
    case PERF_REG_S390_FP15:
    return "FP15";
    case PERF_REG_S390_MASK:
    return "MASK";
    case PERF_REG_S390_PC:
    return "PC";
    default:
    return core::ptr::null_mut();
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __perf_reg_ip_s390() -> u64 {
    uint64_t __perf_reg_ip_s390(void)
    {
    return PERF_REG_S390_PC;
    }
#[no_mangle]
pub unsafe extern "C" fn __perf_reg_sp_s390() -> u64 {
    uint64_t __perf_reg_sp_s390(void)
    {
    return PERF_REG_S390_R15;
    }
// %rXX

// +-###(%rXX)

    static regex_t sdt_op_regex1, sdt_op_regex2;
#[no_mangle]
unsafe extern "C" fn sdt_init_op_regex() -> c_int {
    static int sdt_init_op_regex(void)
    {
    static int initialized;
    let mut ret: c_int = 0;
    if (initialized)
    return 0;
    ret = regcomp(&sdt_op_regex1, SDT_OP_REGEX1, REG_EXTENDED);
    if (ret)
    goto error;
    initialized = 1;
    ret = regcomp(&sdt_op_regex2, SDT_OP_REGEX2, REG_EXTENDED);
    if (ret)
    goto free_regex1;
    initialized = 2;
    return 0;
    free_regex1:
    regfree(&sdt_op_regex1);
    error:
    pr_debug4("Regex compilation error, initialized %d\n", initialized);
    initialized = 0;
    return ret;
    }
//
// Parse OP and convert it into uprobe format, which is, +/-NUM(%gprREG).
// Possible variants of OP are:
// Format		Example
// -------------------------
// NUM(%rREG)	48(%r1)
// -NUM(%rREG)	-48(%r1)
// +NUM(%rREG)	+48(%r1)
// %rREG		%r1
//
#[no_mangle]
pub unsafe extern "C" fn __perf_sdt_arg_parse_op_s390(old_op: *mut c_char, new_op: *mut c_char) -> c_int {
    int __perf_sdt_arg_parse_op_s390(char *old_op, char **new_op)
    {
    int ret, new_len;
    regmatch_t rm[6];
// new_op = NULL;
    ret = sdt_init_op_regex();
    if (ret)
    return -EINVAL;
    if (!regexec(&sdt_op_regex1, old_op, ARRAY_SIZE(rm), rm, 0) ||
    !regexec(&sdt_op_regex2, old_op, ARRAY_SIZE(rm), rm, 0)) {
    new_len = 1;    /* core::ptr::null_mut() byte */
    new_len += (int)(rm[1].rm_eo - rm[1].rm_so);
// new_op = zalloc(new_len);
    if (!*new_op)
    return -ENOMEM;
    scnprintf(*new_op, new_len, "%.*s",
    (int)(rm[1].rm_eo - rm[1].rm_so), old_op + rm[1].rm_so);
    } else {
    pr_debug4("Skipping unsupported SDT argument: %s\n", old_op);
    return SDT_ARG_SKIP;
    }
    return SDT_ARG_VALID;
    }

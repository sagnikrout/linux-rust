//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/perf-regs-arch/perf_regs_aarch64.c
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

// %xNUM

// [sp], [sp, NUM]

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
    ret = regcomp(&sdt_op_regex2, SDT_OP_REGEX2, REG_EXTENDED);
    if (ret)
    goto free_regex1;
    initialized = 1;
    return 0;
    free_regex1:
    regfree(&sdt_op_regex1);
    error:
    pr_debug4("Regex compilation error.\n");
    return ret;
    }
//
// SDT marker arguments on Arm64 uses %xREG or [sp, NUM], currently
// support these two formats.
//
#[no_mangle]
pub unsafe extern "C" fn __perf_sdt_arg_parse_op_arm64(old_op: *mut c_char, new_op: *mut c_char) -> c_int {
    int __perf_sdt_arg_parse_op_arm64(char *old_op, char **new_op)
    {
    int ret, new_len;
    regmatch_t rm[5];
    ret = sdt_init_op_regex();
    if (ret < 0)
    return ret;
    if (!regexec(&sdt_op_regex1, old_op, 3, rm, 0)) {
// Extract xNUM
    new_len = 2;	/* % core::ptr::null_mut() */
    new_len += (int)(rm[1].rm_eo - rm[1].rm_so);
// new_op = zalloc(new_len);
    if (!*new_op)
    return -ENOMEM;
    scnprintf(*new_op, new_len, "%%%.*s",
    (int)(rm[1].rm_eo - rm[1].rm_so), old_op + rm[1].rm_so);
    } else if (!regexec(&sdt_op_regex2, old_op, 5, rm, 0)) {
// [sp], [sp, NUM] or [sp,NUM]
    new_len = 7;	/* + ( % s p ) core::ptr::null_mut() */
// If the argument is [sp], need to fill offset '0'
    if (rm[2].rm_so == -1)
    new_len += 1;
    else
    new_len += (int)(rm[2].rm_eo - rm[2].rm_so);
// new_op = zalloc(new_len);
    if (!*new_op)
    return -ENOMEM;
    if (rm[2].rm_so == -1)
    scnprintf(*new_op, new_len, "+0(%%sp)");
    else
    scnprintf(*new_op, new_len, "+%.*s(%%sp)",
    (int)(rm[2].rm_eo - rm[2].rm_so),
    old_op + rm[2].rm_so);
    } else {
    pr_debug4("Skipping unsupported SDT argument: %s\n", old_op);
    return SDT_ARG_SKIP;
    }
    return SDT_ARG_VALID;
    }
#[no_mangle]
pub unsafe extern "C" fn __perf_reg_mask_arm64(intr: bool) -> u64 {
    uint64_t __perf_reg_mask_arm64(bool intr)
    {
    struct perf_event_attr attr = {
    .type                   = PERF_TYPE_HARDWARE,
    .config                 = PERF_COUNT_HW_CPU_CYCLES,
    .sample_type            = PERF_SAMPLE_REGS_USER,
    .disabled               = 1,
    .exclude_kernel         = 1,
    .sample_period		= 1,
    .sample_regs_user	= PERF_REGS_MASK
    };
    int fd;
    if (intr)
    return PERF_REGS_MASK;
    if (getauxval(AT_HWCAP) & HWCAP_SVE)
    attr.sample_regs_user |= SMPL_REG_MASK(PERF_REG_ARM64_VG);
//
// Check if the pmu supports perf extended regs, before
// returning the register mask to sample. Open the event
// on the perf process to check this.
//
    if (attr.sample_regs_user != PERF_REGS_MASK) {
    event_attr_init(&attr);
    fd = sys_perf_event_open(&attr, /*pid=*/0, /*cpu=*/-1,
// group_fd=*/-1, /*flags=*/0);
    if (fd != -1) {
    close(fd);
    return attr.sample_regs_user;
    }
    }
    return PERF_REGS_MASK;
    }
    const char *__perf_reg_name_arm64(int id)
    {
    switch (id) {
    case PERF_REG_ARM64_X0:
    return "x0";
    case PERF_REG_ARM64_X1:
    return "x1";
    case PERF_REG_ARM64_X2:
    return "x2";
    case PERF_REG_ARM64_X3:
    return "x3";
    case PERF_REG_ARM64_X4:
    return "x4";
    case PERF_REG_ARM64_X5:
    return "x5";
    case PERF_REG_ARM64_X6:
    return "x6";
    case PERF_REG_ARM64_X7:
    return "x7";
    case PERF_REG_ARM64_X8:
    return "x8";
    case PERF_REG_ARM64_X9:
    return "x9";
    case PERF_REG_ARM64_X10:
    return "x10";
    case PERF_REG_ARM64_X11:
    return "x11";
    case PERF_REG_ARM64_X12:
    return "x12";
    case PERF_REG_ARM64_X13:
    return "x13";
    case PERF_REG_ARM64_X14:
    return "x14";
    case PERF_REG_ARM64_X15:
    return "x15";
    case PERF_REG_ARM64_X16:
    return "x16";
    case PERF_REG_ARM64_X17:
    return "x17";
    case PERF_REG_ARM64_X18:
    return "x18";
    case PERF_REG_ARM64_X19:
    return "x19";
    case PERF_REG_ARM64_X20:
    return "x20";
    case PERF_REG_ARM64_X21:
    return "x21";
    case PERF_REG_ARM64_X22:
    return "x22";
    case PERF_REG_ARM64_X23:
    return "x23";
    case PERF_REG_ARM64_X24:
    return "x24";
    case PERF_REG_ARM64_X25:
    return "x25";
    case PERF_REG_ARM64_X26:
    return "x26";
    case PERF_REG_ARM64_X27:
    return "x27";
    case PERF_REG_ARM64_X28:
    return "x28";
    case PERF_REG_ARM64_X29:
    return "x29";
    case PERF_REG_ARM64_SP:
    return "sp";
    case PERF_REG_ARM64_LR:
    return "lr";
    case PERF_REG_ARM64_PC:
    return "pc";
    case PERF_REG_ARM64_VG:
    return "vg";
    default:
    return core::ptr::null_mut();
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __perf_reg_ip_arm64() -> u64 {
    uint64_t __perf_reg_ip_arm64(void)
    {
    return PERF_REG_ARM64_PC;
    }
#[no_mangle]
pub unsafe extern "C" fn __perf_reg_sp_arm64() -> u64 {
    uint64_t __perf_reg_sp_arm64(void)
    {
    return PERF_REG_ARM64_SP;
    }

//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/arm64/fp/sve-ptrace.c
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
// Copyright (C) 2015-2021 ARM Limited.
// Original author: Dave Martin <Dave.Martin@arm.com>
//

// <linux/elf.h> and <sys/auxv.h> don't like each other, so:

pub const NT_ARM_SVE: c_uint = 0x405;

pub const NT_ARM_SSVE: c_uint = 0x40b;

//
// The architecture defines the maximum VQ as 16 but for extensibility
// the kernel specifies the SVE_VQ_MAX as 512 resulting in us running
// a *lot* more tests than are useful if we use it.  Until the
// architecture is extended let's limit our coverage to what is
// currently allowed, plus one extra to ensure we cover constraining
// the VL as expected.
//
pub const TEST_VQ_MAX: c_int = 17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vec_type {
    pub name: *const c_char,
    pub hwcap_type: c_ulong,
    pub hwcap: c_ulong,
    pub regset: c_int,
    pub prctl_set: c_int,
}

    static const struct vec_type vec_types[] = {
    {
    .name = "SVE",
    .hwcap_type = AT_HWCAP,
    .hwcap = HWCAP_SVE,
    .regset = NT_ARM_SVE,
    .prctl_set = PR_SVE_SET_VL,
    },
    {
    .name = "Streaming SVE",
    .hwcap_type = AT_HWCAP2,
    .hwcap = HWCAP2_SME,
    .regset = NT_ARM_SSVE,
    .prctl_set = PR_SME_SET_VL,
    },
    };

pub const FLAG_TESTS: c_int = 4;
pub const FPSIMD_TESTS: c_int = 2;

#[no_mangle]
unsafe extern "C" fn fill_buf(buf: *mut c_char, size: usize) {
    static void fill_buf(char *buf, size_t size)
    {
    int i;
    for (i = 0; i < size; i++)
    buf[i] = random();
    }
#[no_mangle]
unsafe extern "C" fn do_child() -> c_int {
    static int do_child(void)
    {
    if (ptrace(PTRACE_TRACEME, -1, core::ptr::null_mut(), core::ptr::null_mut()))
    ksft_exit_fail_msg("ptrace(PTRACE_TRACEME) failed: %s (%d)\n",
    strerror(errno), errno);
    if (raise(SIGSTOP))
    ksft_exit_fail_msg("raise(SIGSTOP) failed: %s (%d)\n",
    strerror(errno), errno);
    return EXIT_SUCCESS;
    }
#[no_mangle]
unsafe extern "C" fn get_fpsimd(pid: pid_t, fpsimd: *mut user_fpsimd_state) -> c_int {
    static int get_fpsimd(pid_t pid, struct user_fpsimd_state *fpsimd)
    {
    struct iovec iov;
    int ret;
    iov.iov_base = fpsimd;
    iov.iov_len = sizeof(*fpsimd);
    ret = ptrace(PTRACE_GETREGSET, pid, NT_PRFPREG, &iov);
    if (ret == -1)
    ksft_perror("ptrace(PTRACE_GETREGSET)");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn set_fpsimd(pid: pid_t, fpsimd: *mut user_fpsimd_state) -> c_int {
    static int set_fpsimd(pid_t pid, struct user_fpsimd_state *fpsimd)
    {
    struct iovec iov;
    int ret;
    iov.iov_base = fpsimd;
    iov.iov_len = sizeof(*fpsimd);
    ret = ptrace(PTRACE_SETREGSET, pid, NT_PRFPREG, &iov);
    if (ret == -1)
    ksft_perror("ptrace(PTRACE_SETREGSET)");
    return ret;
    }
    static struct user_sve_header *get_sve(pid_t pid, const struct vec_type *type,
    void **buf, size_t *size)
    {
    struct user_sve_header *sve;
    void *p;
    let mut sz: usize = sizeof(*sve);
    struct iovec iov;
    int ret;
    while (1) {
    if (*size < sz) {
    p = realloc(*buf, sz);
    if (!p) {
    errno = ENOMEM;
    goto error;
    }
// buf = p;
// size = sz;
    }
    iov.iov_base = *buf;
    iov.iov_len = sz;
    ret = ptrace(PTRACE_GETREGSET, pid, type.regset, &iov);
    if (ret) {
    ksft_perror("ptrace(PTRACE_GETREGSET)");
    goto error;
    }
    sve = *buf;
    if (sve.size <= sz)
    break;
    sz = sve.size;
    }
    return sve;
    error:
    return core::ptr::null_mut();
    }
    static int set_sve(pid_t pid, const struct vec_type *type,
    const struct user_sve_header *sve)
    {
    struct iovec iov;
    int ret;
    iov.iov_base = (void *)sve;
    iov.iov_len = sve.size;
    ret = ptrace(PTRACE_SETREGSET, pid, type.regset, &iov);
    if (ret == -1)
    ksft_perror("ptrace(PTRACE_SETREGSET)");
    return ret;
    }
// A read operation fails
#[no_mangle]
unsafe extern "C" fn read_fails(child: pid_t, type: *const vec_type) {
    static void read_fails(pid_t child, const struct vec_type *type)
    {
    struct user_sve_header *new_sve = core::ptr::null_mut();
    let mut new_sve_size: usize = 0;
    void *ret;
    ret = get_sve(child, type, (void **)&new_sve, &new_sve_size);
    ksft_test_result(ret == core::ptr::null_mut(), "%s unsupported read fails\n",
    type.name);
    free(new_sve);
    }
// A write operation fails
#[no_mangle]
unsafe extern "C" fn write_fails(child: pid_t, type: *const vec_type) {
    static void write_fails(pid_t child, const struct vec_type *type)
    {
    struct user_sve_header sve;
    int ret;
// Just the header, no data
    memset(&sve, 0, sizeof(sve));
    sve.size = sizeof(sve);
    sve.flags = SVE_PT_REGS_SVE;
    sve.vl = SVE_VL_MIN;
    ret = set_sve(child, type, &sve);
    ksft_test_result(ret != 0, "%s unsupported write fails\n",
    type.name);
    }
// Validate setting and getting the inherit flag
#[no_mangle]
unsafe extern "C" fn ptrace_set_get_inherit(child: pid_t, type: *const vec_type) {
    static void ptrace_set_get_inherit(pid_t child, const struct vec_type *type)
    {
    struct user_sve_header sve;
    struct user_sve_header *new_sve = core::ptr::null_mut();
    let mut new_sve_size: usize = 0;
    int ret;
// First set the flag
    memset(&sve, 0, sizeof(sve));
    sve.size = sizeof(sve);
    sve.vl = sve_vl_from_vq(SVE_VQ_MIN);
    sve.flags = SVE_PT_VL_INHERIT | SVE_PT_REGS_SVE;
    ret = set_sve(child, type, &sve);
    if (ret != 0) {
    ksft_test_result_fail("Failed to set %s SVE_PT_VL_INHERIT\n",
    type.name);
    return;
    }
//
// Read back the new register state and verify that we have
// set the flags we expected.
//
    if (!get_sve(child, type, (void **)&new_sve, &new_sve_size)) {
    ksft_test_result_fail("Failed to read %s SVE flags\n",
    type.name);
    return;
    }
    ksft_test_result(new_sve.flags & SVE_PT_VL_INHERIT,
    "%s SVE_PT_VL_INHERIT set\n", type.name);
// Now clear
    sve.flags &= ~SVE_PT_VL_INHERIT;
    ret = set_sve(child, type, &sve);
    if (ret != 0) {
    ksft_test_result_fail("Failed to clear %s SVE_PT_VL_INHERIT\n",
    type.name);
    return;
    }
    if (!get_sve(child, type, (void **)&new_sve, &new_sve_size)) {
    ksft_test_result_fail("Failed to read %s SVE flags\n",
    type.name);
    return;
    }
    ksft_test_result(!(new_sve.flags & SVE_PT_VL_INHERIT),
    "%s SVE_PT_VL_INHERIT cleared\n", type.name);
    free(new_sve);
    }
// Validate attempting to set the specfied VL via ptrace
    static void ptrace_set_get_vl(pid_t child, const struct vec_type *type,
    unsigned int vl, bool *supported)
    {
    struct user_sve_header sve;
    struct user_sve_header *new_sve = core::ptr::null_mut();
    let mut new_sve_size: usize = 0;
    int ret, prctl_vl;
// supported = false;
// Check if the VL is supported in this process
    prctl_vl = prctl(type.prctl_set, vl);
    if (prctl_vl == -1)
    ksft_exit_fail_msg("prctl(PR_%s_SET_VL) failed: %s (%d)\n",
    type.name, strerror(errno), errno);
// If the VL is not supported then a supported VL will be returned
// supported = (prctl_vl == vl);
// Set the VL by doing a set with no register payload
    memset(&sve, 0, sizeof(sve));
    sve.size = sizeof(sve);
    sve.flags = SVE_PT_REGS_SVE;
    sve.vl = vl;
    ret = set_sve(child, type, &sve);
    if (ret != 0) {
    ksft_test_result_fail("Failed to set %s VL %u\n",
    type.name, vl);
    return;
    }
//
// Read back the new register state and verify that we have the
// same VL that we got from prctl() on ourselves.
//
    if (!get_sve(child, type, (void **)&new_sve, &new_sve_size)) {
    ksft_test_result_fail("Failed to read %s VL %u\n",
    type.name, vl);
    return;
    }
    ksft_test_result(new_sve.vl == prctl_vl, "Set %s VL %u\n",
    type.name, vl);
    free(new_sve);
    }
    static void check_u32(unsigned int vl, const char *reg,
    uint32_t *in, uint32_t *out, int *errors)
    {
    if (*in != *out) {
    printf("# VL %d %s wrote %x read %x\n",
    vl, reg, *in, *out);
    (*errors)++;
    }
    }
// Set out of range VLs
#[no_mangle]
unsafe extern "C" fn ptrace_set_vl_ranges(child: pid_t, type: *const vec_type) {
    static void ptrace_set_vl_ranges(pid_t child, const struct vec_type *type)
    {
    struct user_sve_header sve;
    int ret;
    memset(&sve, 0, sizeof(sve));
    sve.flags = SVE_PT_REGS_SVE;
    sve.size = sizeof(sve);
    ret = set_sve(child, type, &sve);
    ksft_test_result(ret != 0, "%s Set invalid VL 0\n", type.name);
    sve.vl = SVE_VL_MAX + SVE_VQ_BYTES;
    ret = set_sve(child, type, &sve);
    ksft_test_result(ret != 0, "%s Set invalid VL %d\n", type.name,
    SVE_VL_MAX + SVE_VQ_BYTES);
    }
// Access the FPSIMD registers via the SVE regset
#[no_mangle]
unsafe extern "C" fn ptrace_sve_fpsimd(child: pid_t, type: *const vec_type) {
    static void ptrace_sve_fpsimd(pid_t child, const struct vec_type *type)
    {
    void *svebuf;
    struct user_sve_header *sve;
    struct user_fpsimd_state *fpsimd, new_fpsimd;
    unsigned int i, j;
    unsigned char *p;
    int ret;
    svebuf = malloc(SVE_PT_SIZE(0, SVE_PT_REGS_FPSIMD));
    if (!svebuf) {
    ksft_test_result_fail("Failed to allocate FPSIMD buffer\n");
    return;
    }
    memset(svebuf, 0, SVE_PT_SIZE(0, SVE_PT_REGS_FPSIMD));
    sve = svebuf;
    sve.flags = SVE_PT_REGS_FPSIMD;
    sve.size = SVE_PT_SIZE(0, SVE_PT_REGS_FPSIMD);
    sve.vl = 16;  /* We don't care what the VL is */
// Try to set a known FPSIMD state via PT_REGS_SVE
    fpsimd = (struct user_fpsimd_state *)((char *)sve +
    SVE_PT_FPSIMD_OFFSET);
    for (i = 0; i < 32; ++i) {
    p = (unsigned char *)&fpsimd.vregs[i];
    for (j = 0; j < sizeof(fpsimd.vregs[i]); ++j)
    p[j] = j;
    }
// This should only succeed for SVE
    ret = set_sve(child, type, sve);
    ksft_test_result((type.regset == NT_ARM_SVE) == (ret == 0),
    "%s FPSIMD set via SVE: %d\n",
    type.name, ret);
    if (ret)
    goto out;
// Verify via the FPSIMD regset
    if (get_fpsimd(child, &new_fpsimd)) {
    ksft_test_result_fail("get_fpsimd(): %s\n",
    strerror(errno));
    goto out;
    }
    if (memcmp(fpsimd, &new_fpsimd, sizeof(*fpsimd)) == 0)
    ksft_test_result_pass("%s get_fpsimd() gave same state\n",
    type.name);
    else
    ksft_test_result_fail("%s get_fpsimd() gave different state\n",
    type.name);
    out:
    free(svebuf);
    }
// Write the FPSIMD registers via the SVE regset when SVE is not supported
#[no_mangle]
unsafe extern "C" fn ptrace_sve_fpsimd_no_sve(child: pid_t) {
    static void ptrace_sve_fpsimd_no_sve(pid_t child)
    {
    void *svebuf;
    struct user_sve_header *sve;
    struct user_fpsimd_state *fpsimd, new_fpsimd;
    unsigned int i, j;
    unsigned char *p;
    int ret;
    svebuf = malloc(SVE_PT_SIZE(0, SVE_PT_REGS_FPSIMD));
    if (!svebuf) {
    ksft_test_result_fail("Failed to allocate FPSIMD buffer\n");
    return;
    }
// On a system without SVE the VL should be set to 0
    memset(svebuf, 0, SVE_PT_SIZE(0, SVE_PT_REGS_FPSIMD));
    sve = svebuf;
    sve.flags = SVE_PT_REGS_FPSIMD;
    sve.size = SVE_PT_SIZE(0, SVE_PT_REGS_FPSIMD);
    sve.vl = 0;
// Try to set a known FPSIMD state via PT_REGS_SVE
    fpsimd = (struct user_fpsimd_state *)((char *)sve +
    SVE_PT_FPSIMD_OFFSET);
    for (i = 0; i < 32; ++i) {
    p = (unsigned char *)&fpsimd.vregs[i];
    for (j = 0; j < sizeof(fpsimd.vregs[i]); ++j)
    p[j] = j;
    }
    ret = set_sve(child, &vec_types[0], sve);
    ksft_test_result(ret == 0, "FPSIMD write via SVE\n");
    if (ret) {
    ksft_test_result_skip("Verify FPSIMD write via SVE\n");
    goto out;
    }
// Verify via the FPSIMD regset
    if (get_fpsimd(child, &new_fpsimd)) {
    ksft_test_result_skip("Verify FPSIMD write via SVE\n");
    goto out;
    }
    ksft_test_result(memcmp(fpsimd, &new_fpsimd, sizeof(*fpsimd)) == 0,
    "Verify FPSIMD write via SVE\n");
    out:
    free(svebuf);
    }
// Validate attempting to set SVE data and read SVE data
    static void ptrace_set_sve_get_sve_data(pid_t child,
    const struct vec_type *type,
    unsigned int vl)
    {
    void *write_buf;
    void *read_buf = core::ptr::null_mut();
    struct user_sve_header *write_sve;
    struct user_sve_header *read_sve;
    let mut read_sve_size: usize = 0;
    let mut vq: c_uint = sve_vq_from_vl(vl);
    int ret, i;
    size_t data_size;
    let mut errors: c_int = 0;
    data_size = SVE_PT_SVE_OFFSET + SVE_PT_SVE_SIZE(vq, SVE_PT_REGS_SVE);
    write_buf = malloc(data_size);
    if (!write_buf) {
    ksft_test_result_fail("Error allocating %ld byte buffer for %s VL %u\n",
    data_size, type.name, vl);
    return;
    }
    write_sve = write_buf;
// Set up some data and write it out
    memset(write_sve, 0, data_size);
    write_sve.size = data_size;
    write_sve.vl = vl;
    write_sve.flags = SVE_PT_REGS_SVE;
    for (i = 0; i < __SVE_NUM_ZREGS; i++)
    fill_buf(write_buf + SVE_PT_SVE_ZREG_OFFSET(vq, i),
    SVE_PT_SVE_ZREG_SIZE(vq));
    for (i = 0; i < __SVE_NUM_PREGS; i++)
    fill_buf(write_buf + SVE_PT_SVE_PREG_OFFSET(vq, i),
    SVE_PT_SVE_PREG_SIZE(vq));
    fill_buf(write_buf + SVE_PT_SVE_FPSR_OFFSET(vq), SVE_PT_SVE_FPSR_SIZE);
    fill_buf(write_buf + SVE_PT_SVE_FPCR_OFFSET(vq), SVE_PT_SVE_FPCR_SIZE);
// TODO: Generate a valid FFR pattern
    ret = set_sve(child, type, write_sve);
    if (ret != 0) {
    ksft_test_result_fail("Failed to set %s VL %u data\n",
    type.name, vl);
    goto out;
    }
// Read the data back
    if (!get_sve(child, type, (void **)&read_buf, &read_sve_size)) {
    ksft_test_result_fail("Failed to read %s VL %u data\n",
    type.name, vl);
    goto out;
    }
    read_sve = read_buf;
// We might read more data if there's extensions we don't know
    if (read_sve.size < write_sve.size) {
    ksft_test_result_fail("%s wrote %d bytes, only read %d\n",
    type.name, write_sve.size,
    read_sve.size);
    goto out_read;
    }
    for (i = 0; i < __SVE_NUM_ZREGS; i++) {
    if (memcmp(write_buf + SVE_PT_SVE_ZREG_OFFSET(vq, i),
    read_buf + SVE_PT_SVE_ZREG_OFFSET(vq, i),
    SVE_PT_SVE_ZREG_SIZE(vq)) != 0) {
    printf("# Mismatch in %u Z%d\n", vl, i);
    errors++;
    }
    }
    for (i = 0; i < __SVE_NUM_PREGS; i++) {
    if (memcmp(write_buf + SVE_PT_SVE_PREG_OFFSET(vq, i),
    read_buf + SVE_PT_SVE_PREG_OFFSET(vq, i),
    SVE_PT_SVE_PREG_SIZE(vq)) != 0) {
    printf("# Mismatch in %u P%d\n", vl, i);
    errors++;
    }
    }
    check_u32(vl, "FPSR", write_buf + SVE_PT_SVE_FPSR_OFFSET(vq),
    read_buf + SVE_PT_SVE_FPSR_OFFSET(vq), &errors);
    check_u32(vl, "FPCR", write_buf + SVE_PT_SVE_FPCR_OFFSET(vq),
    read_buf + SVE_PT_SVE_FPCR_OFFSET(vq), &errors);
    ksft_test_result(errors == 0, "Set and get %s data for VL %u\n",
    type.name, vl);
    out_read:
    free(read_buf);
    out:
    free(write_buf);
    }
// Validate attempting to set SVE data and read it via the FPSIMD regset
    static void ptrace_set_sve_get_fpsimd_data(pid_t child,
    const struct vec_type *type,
    unsigned int vl)
    {
    void *write_buf;
    struct user_sve_header *write_sve;
    let mut vq: c_uint = sve_vq_from_vl(vl);
    struct user_fpsimd_state fpsimd_state;
    int ret, i;
    size_t data_size;
    let mut errors: c_int = 0;
    if (__BYTE_ORDER == __BIG_ENDIAN) {
    ksft_test_result_skip("Big endian not supported\n");
    return;
    }
    data_size = SVE_PT_SVE_OFFSET + SVE_PT_SVE_SIZE(vq, SVE_PT_REGS_SVE);
    write_buf = malloc(data_size);
    if (!write_buf) {
    ksft_test_result_fail("Error allocating %ld byte buffer for %s VL %u\n",
    data_size, type.name, vl);
    return;
    }
    write_sve = write_buf;
// Set up some data and write it out
    memset(write_sve, 0, data_size);
    write_sve.size = data_size;
    write_sve.vl = vl;
    write_sve.flags = SVE_PT_REGS_SVE;
    for (i = 0; i < __SVE_NUM_ZREGS; i++)
    fill_buf(write_buf + SVE_PT_SVE_ZREG_OFFSET(vq, i),
    SVE_PT_SVE_ZREG_SIZE(vq));
    fill_buf(write_buf + SVE_PT_SVE_FPSR_OFFSET(vq), SVE_PT_SVE_FPSR_SIZE);
    fill_buf(write_buf + SVE_PT_SVE_FPCR_OFFSET(vq), SVE_PT_SVE_FPCR_SIZE);
    ret = set_sve(child, type, write_sve);
    if (ret != 0) {
    ksft_test_result_fail("Failed to set %s VL %u data\n",
    type.name, vl);
    goto out;
    }
// Read the data back
    if (get_fpsimd(child, &fpsimd_state)) {
    ksft_test_result_fail("Failed to read %s VL %u FPSIMD data\n",
    type.name, vl);
    goto out;
    }
    for (i = 0; i < __SVE_NUM_ZREGS; i++) {
    let mut tmp: __uint128_t = 0;
//
// Z regs are stored endianness invariant, this won't
// work for big endian
//
    memcpy(&tmp, write_buf + SVE_PT_SVE_ZREG_OFFSET(vq, i),
    sizeof(tmp));
    if (tmp != fpsimd_state.vregs[i]) {
    printf("# Mismatch in FPSIMD for %s VL %u Z%d\n",
    type.name, vl, i);
    errors++;
    }
    }
    check_u32(vl, "FPSR", write_buf + SVE_PT_SVE_FPSR_OFFSET(vq),
    &fpsimd_state.fpsr, &errors);
    check_u32(vl, "FPCR", write_buf + SVE_PT_SVE_FPCR_OFFSET(vq),
    &fpsimd_state.fpcr, &errors);
    ksft_test_result(errors == 0, "Set and get FPSIMD data for %s VL %u\n",
    type.name, vl);
    out:
    free(write_buf);
    }
// Validate attempting to set FPSIMD data and read it via the SVE regset
    static void ptrace_set_fpsimd_get_sve_data(pid_t child,
    const struct vec_type *type,
    unsigned int vl)
    {
    void *read_buf = core::ptr::null_mut();
    unsigned char *p;
    struct user_sve_header *read_sve;
    let mut vq: c_uint = sve_vq_from_vl(vl);
    struct user_fpsimd_state write_fpsimd;
    int ret, i, j;
    let mut read_sve_size: usize = 0;
    size_t expected_size;
    let mut errors: c_int = 0;
    if (__BYTE_ORDER == __BIG_ENDIAN) {
    ksft_test_result_skip("Big endian not supported\n");
    return;
    }
    for (i = 0; i < 32; ++i) {
    p = (unsigned char *)&write_fpsimd.vregs[i];
    for (j = 0; j < sizeof(write_fpsimd.vregs[i]); ++j)
    p[j] = j;
    }
    ret = set_fpsimd(child, &write_fpsimd);
    if (ret != 0) {
    ksft_test_result_fail("Failed to set FPSIMD state: %d\n)",
    ret);
    return;
    }
    if (!get_sve(child, type, (void **)&read_buf, &read_sve_size)) {
    ksft_test_result_fail("Failed to read %s VL %u data\n",
    type.name, vl);
    return;
    }
    read_sve = read_buf;
    if (read_sve.vl != vl) {
    ksft_test_result_fail("Child VL != expected VL: %u != %u\n",
    read_sve.vl, vl);
    goto out;
    }
// The kernel may return either SVE or FPSIMD format
    switch (read_sve.flags & SVE_PT_REGS_MASK) {
    case SVE_PT_REGS_FPSIMD:
    expected_size = SVE_PT_FPSIMD_SIZE(vq, SVE_PT_REGS_FPSIMD);
    if (read_sve_size < expected_size) {
    ksft_test_result_fail("Read %ld bytes, expected %ld\n",
    read_sve_size, expected_size);
    goto out;
    }
    ret = memcmp(&write_fpsimd, read_buf + SVE_PT_FPSIMD_OFFSET,
    sizeof(write_fpsimd));
    if (ret != 0) {
    ksft_print_msg("Read FPSIMD data mismatch\n");
    errors++;
    }
    break;
    case SVE_PT_REGS_SVE:
    expected_size = SVE_PT_SVE_SIZE(vq, SVE_PT_REGS_SVE);
    if (read_sve_size < expected_size) {
    ksft_test_result_fail("Read %ld bytes, expected %ld\n",
    read_sve_size, expected_size);
    goto out;
    }
    for (i = 0; i < __SVE_NUM_ZREGS; i++) {
    let mut tmp: __uint128_t = 0;
//
// Z regs are stored endianness invariant, this won't
// work for big endian
//
    memcpy(&tmp, read_buf + SVE_PT_SVE_ZREG_OFFSET(vq, i),
    sizeof(tmp));
    if (tmp != write_fpsimd.vregs[i]) {
    ksft_print_msg("Mismatch in FPSIMD for %s VL %u Z%d/V%d\n",
    type.name, vl, i, i);
    errors++;
    }
    }
    check_u32(vl, "FPSR", &write_fpsimd.fpsr,
    read_buf + SVE_PT_SVE_FPSR_OFFSET(vq), &errors);
    check_u32(vl, "FPCR", &write_fpsimd.fpcr,
    read_buf + SVE_PT_SVE_FPCR_OFFSET(vq), &errors);
    break;
    default:
    ksft_print_msg("Unexpected regs type %d\n",
    read_sve.flags & SVE_PT_REGS_MASK);
    errors++;
    break;
    }
    ksft_test_result(errors == 0, "Set FPSIMD, read via SVE for %s VL %u\n",
    type.name, vl);
    out:
    free(read_buf);
    }
#[no_mangle]
unsafe extern "C" fn do_parent(child: pid_t) -> c_int {
    static int do_parent(pid_t child)
    {
    let mut ret: c_int = EXIT_FAILURE;
    pid_t pid;
    int status, i;
    siginfo_t si;
    unsigned int vq, vl;
    bool vl_supported;
    ksft_print_msg("Parent is %d, child is %d\n", getpid(), child);
// Attach to the child
    while (1) {
    int sig;
    pid = wait(&status);
    if (pid == -1) {
    perror("wait");
    goto error;
    }
//
// This should never happen but it's hard to flag in
// the framework.
//
    if (pid != child)
    continue;
    if (WIFEXITED(status) || WIFSIGNALED(status))
    ksft_exit_fail_msg("Child died unexpectedly\n");
    if (!WIFSTOPPED(status))
    goto error;
    sig = WSTOPSIG(status);
    if (ptrace(PTRACE_GETSIGINFO, pid, core::ptr::null_mut(), &si)) {
    if (errno == ESRCH)
    goto disappeared;
    if (errno == EINVAL) {
    sig = 0; /* bust group-stop */
    goto cont;
    }
    ksft_test_result_fail("PTRACE_GETSIGINFO: %s\n",
    strerror(errno));
    goto error;
    }
    if (sig == SIGSTOP && si.si_code == SI_TKILL &&
    si.si_pid == pid)
    break;
    cont:
    if (ptrace(PTRACE_CONT, pid, core::ptr::null_mut(), sig)) {
    if (errno == ESRCH)
    goto disappeared;
    ksft_test_result_fail("PTRACE_CONT: %s\n",
    strerror(errno));
    goto error;
    }
    }
    for (i = 0; i < ARRAY_SIZE(vec_types); i++) {
//
// If the vector type isn't supported reads and writes
// should fail.
//
    if (!(getauxval(vec_types[i].hwcap_type) & vec_types[i].hwcap)) {
    read_fails(child, &vec_types[i]);
    write_fails(child, &vec_types[i]);
    } else {
    ksft_test_result_skip("%s unsupported read fails\n",
    vec_types[i].name);
    ksft_test_result_skip("%s unsupported write fails\n",
    vec_types[i].name);
    }
// FPSIMD via SVE regset
    if (getauxval(vec_types[i].hwcap_type) & vec_types[i].hwcap) {
    ptrace_sve_fpsimd(child, &vec_types[i]);
    } else {
    ksft_test_result_skip("%s FPSIMD set via SVE\n",
    vec_types[i].name);
    ksft_test_result_skip("%s FPSIMD read\n",
    vec_types[i].name);
    }
// prctl() flags
    if (getauxval(vec_types[i].hwcap_type) & vec_types[i].hwcap) {
    ptrace_set_get_inherit(child, &vec_types[i]);
    } else {
    ksft_test_result_skip("%s SVE_PT_VL_INHERIT set\n",
    vec_types[i].name);
    ksft_test_result_skip("%s SVE_PT_VL_INHERIT cleared\n",
    vec_types[i].name);
    }
// Setting out of bounds VLs should fail
    if (getauxval(vec_types[i].hwcap_type) & vec_types[i].hwcap) {
    ptrace_set_vl_ranges(child, &vec_types[i]);
    } else {
    ksft_test_result_skip("%s Set invalid VL 0\n",
    vec_types[i].name);
    ksft_test_result_skip("%s Set invalid VL %d\n",
    vec_types[i].name,
    SVE_VL_MAX + SVE_VQ_BYTES);
    }
// Step through every possible VQ
    for (vq = SVE_VQ_MIN; vq <= TEST_VQ_MAX; vq++) {
    vl = sve_vl_from_vq(vq);
// First, try to set this vector length
    if (getauxval(vec_types[i].hwcap_type) &
    vec_types[i].hwcap) {
    ptrace_set_get_vl(child, &vec_types[i], vl,
    &vl_supported);
    } else {
    ksft_test_result_skip("%s get/set VL %d\n",
    vec_types[i].name, vl);
    vl_supported = false;
    }
// If the VL is supported validate data set/get
    if (vl_supported) {
    ptrace_set_sve_get_sve_data(child, &vec_types[i], vl);
    ptrace_set_sve_get_fpsimd_data(child, &vec_types[i], vl);
    ptrace_set_fpsimd_get_sve_data(child, &vec_types[i], vl);
    } else {
    ksft_test_result_skip("%s set SVE get SVE for VL %d\n",
    vec_types[i].name, vl);
    ksft_test_result_skip("%s set SVE get FPSIMD for VL %d\n",
    vec_types[i].name, vl);
    ksft_test_result_skip("%s set FPSIMD get SVE for VL %d\n",
    vec_types[i].name, vl);
    }
    }
    }
// We support SVE writes of FPSMID format on SME only systems
    if (!(getauxval(AT_HWCAP) & HWCAP_SVE) &&
    (getauxval(AT_HWCAP2) & HWCAP2_SME)) {
    ptrace_sve_fpsimd_no_sve(child);
    } else {
    ksft_test_result_skip("FPSIMD write via SVE\n");
    ksft_test_result_skip("Verify FPSIMD write via SVE\n");
    }
    ret = EXIT_SUCCESS;
    error:
    kill(child, SIGKILL);
    disappeared:
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    let mut ret: c_int = EXIT_SUCCESS;
    pid_t child;
    srandom(getpid());
    ksft_print_header();
    ksft_set_plan(EXPECTED_TESTS);
    child = fork();
    if (!child)
    return do_child();
    if (do_parent(child))
    ret = EXIT_FAILURE;
    ksft_print_cnts();
    return ret;
    }

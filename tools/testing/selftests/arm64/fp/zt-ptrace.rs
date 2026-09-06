//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/arm64/fp/zt-ptrace.c
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
// Copyright (C) 2021 ARM Limited.
//

// <linux/elf.h> and <sys/auxv.h> don't like each other, so:

pub const NT_ARM_ZA: c_uint = 0x40c;

pub const NT_ARM_ZT: c_uint = 0x40d;

pub const EXPECTED_TESTS: c_int = 3;
    static int sme_vl;
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
    static struct user_za_header *get_za(pid_t pid, void **buf, size_t *size)
    {
    struct user_za_header *za;
    void *p;
    let mut sz: usize = sizeof(*za);
    struct iovec iov;
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
    if (ptrace(PTRACE_GETREGSET, pid, NT_ARM_ZA, &iov))
    goto error;
    za = *buf;
    if (za.size <= sz)
    break;
    sz = za.size;
    }
    return za;
    error:
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn set_za(pid: pid_t, za: *const user_za_header) -> c_int {
    static int set_za(pid_t pid, const struct user_za_header *za)
    {
    struct iovec iov;
    iov.iov_base = (void *)za;
    iov.iov_len = za.size;
    return ptrace(PTRACE_SETREGSET, pid, NT_ARM_ZA, &iov);
    }
#[no_mangle]
unsafe extern "C" fn get_zt(pid: pid_t, zt[ZT_SIG_REG_BYTES]: c_char) -> c_int {
    static int get_zt(pid_t pid, char zt[ZT_SIG_REG_BYTES])
    {
    struct iovec iov;
    iov.iov_base = zt;
    iov.iov_len = ZT_SIG_REG_BYTES;
    return ptrace(PTRACE_GETREGSET, pid, NT_ARM_ZT, &iov);
    }
#[no_mangle]
unsafe extern "C" fn set_zt(pid: pid_t, zt[ZT_SIG_REG_BYTES]: c_char) -> c_int {
    static int set_zt(pid_t pid, const char zt[ZT_SIG_REG_BYTES])
    {
    struct iovec iov;
    iov.iov_base = (void *)zt;
    iov.iov_len = ZT_SIG_REG_BYTES;
    return ptrace(PTRACE_SETREGSET, pid, NT_ARM_ZT, &iov);
    }
// Reading with ZA disabled returns all zeros
#[no_mangle]
unsafe extern "C" fn ptrace_za_disabled_read_zt(child: pid_t) {
    static void ptrace_za_disabled_read_zt(pid_t child)
    {
    struct user_za_header za;
    char zt[ZT_SIG_REG_BYTES];
    int ret, i;
    let mut fail: bool = false;
// Disable PSTATE.ZA using the ZA interface
    memset(&za, 0, sizeof(za));
    za.vl = sme_vl;
    za.size = sizeof(za);
    ret = set_za(child, &za);
    if (ret != 0) {
    ksft_print_msg("Failed to disable ZA\n");
    fail = true;
    }
// Read back ZT
    ret = get_zt(child, zt);
    if (ret != 0) {
    ksft_print_msg("Failed to read ZT\n");
    fail = true;
    }
    for (i = 0; i < ARRAY_SIZE(zt); i++) {
    if (zt[i]) {
    ksft_print_msg("zt[%d]: 0x%x != 0\n", i, zt[i]);
    fail = true;
    }
    }
    ksft_test_result(!fail, "ptrace_za_disabled_read_zt\n");
    }
// Writing then reading ZT should return the data written
#[no_mangle]
unsafe extern "C" fn ptrace_set_get_zt(child: pid_t) {
    static void ptrace_set_get_zt(pid_t child)
    {
    char zt_in[ZT_SIG_REG_BYTES];
    char zt_out[ZT_SIG_REG_BYTES];
    int ret, i;
    let mut fail: bool = false;
    fill_buf(zt_in, sizeof(zt_in));
    ret = set_zt(child, zt_in);
    if (ret != 0) {
    ksft_print_msg("Failed to set ZT\n");
    fail = true;
    }
    ret = get_zt(child, zt_out);
    if (ret != 0) {
    ksft_print_msg("Failed to read ZT\n");
    fail = true;
    }
    for (i = 0; i < ARRAY_SIZE(zt_in); i++) {
    if (zt_in[i] != zt_out[i]) {
    ksft_print_msg("zt[%d]: 0x%x != 0x%x\n", i,
    zt_in[i], zt_out[i]);
    fail = true;
    }
    }
    ksft_test_result(!fail, "ptrace_set_get_zt\n");
    }
// Writing ZT should set PSTATE.ZA
#[no_mangle]
unsafe extern "C" fn ptrace_enable_za_via_zt(child: pid_t) {
    static void ptrace_enable_za_via_zt(pid_t child)
    {
    struct user_za_header za_in;
    struct user_za_header *za_out;
    char zt[ZT_SIG_REG_BYTES];
    char *za_data;
    size_t za_out_size;
    int ret, i, vq;
    let mut fail: bool = false;
// Disable PSTATE.ZA using the ZA interface
    memset(&za_in, 0, sizeof(za_in));
    za_in.vl = sme_vl;
    za_in.size = sizeof(za_in);
    ret = set_za(child, &za_in);
    if (ret != 0) {
    ksft_print_msg("Failed to disable ZA\n");
    fail = true;
    }
// Write ZT
    fill_buf(zt, sizeof(zt));
    ret = set_zt(child, zt);
    if (ret != 0) {
    ksft_print_msg("Failed to set ZT\n");
    fail = true;
    }
// Read back ZA and check for register data
    za_out = core::ptr::null_mut();
    za_out_size = 0;
    if (get_za(child, (void **)&za_out, &za_out_size)) {
// Should have an unchanged VL
    if (za_out.vl != sme_vl) {
    ksft_print_msg("VL changed from %d to %d\n",
    sme_vl, za_out.vl);
    fail = true;
    }
    vq = __sve_vq_from_vl(za_out.vl);
    za_data = (char *)za_out + ZA_PT_ZA_OFFSET;
// Should have register data
    if (za_out.size < ZA_PT_SIZE(vq)) {
    ksft_print_msg("ZA data less than expected: %u < %u\n",
    za_out.size, (unsigned int)ZA_PT_SIZE(vq));
    fail = true;
    vq = 0;
    }
// That register data should be non-zero
    for (i = 0; i < ZA_PT_ZA_SIZE(vq); i++) {
    if (za_data[i]) {
    ksft_print_msg("ZA byte %d is %x\n",
    i, za_data[i]);
    fail = true;
    }
    }
    } else {
    ksft_print_msg("Failed to read ZA\n");
    fail = true;
    }
    ksft_test_result(!fail, "ptrace_enable_za_via_zt\n");
    }
#[no_mangle]
unsafe extern "C" fn do_parent(child: pid_t) -> c_int {
    static int do_parent(pid_t child)
    {
    let mut ret: c_int = EXIT_FAILURE;
    pid_t pid;
    int status;
    siginfo_t si;
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
    ksft_print_msg("Parent is %d, child is %d\n", getpid(), child);
    ptrace_za_disabled_read_zt(child);
    ptrace_set_get_zt(child);
    ptrace_enable_za_via_zt(child);
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
    if (!(getauxval(AT_HWCAP2) & HWCAP2_SME2)) {
    ksft_set_plan(1);
    ksft_exit_skip("SME2 not available\n");
    }
// We need a valid SME VL to enable/disable ZA
    sme_vl = prctl(PR_SME_GET_VL);
    if (sme_vl == -1) {
    ksft_set_plan(1);
    ksft_exit_skip("Failed to read SME VL: %d (%s)\n",
    errno, strerror(errno));
    }
    ksft_set_plan(EXPECTED_TESTS);
    child = fork();
    if (!child)
    return do_child();
    if (do_parent(child))
    ret = EXIT_FAILURE;
    ksft_print_cnts();
    return ret;
    }

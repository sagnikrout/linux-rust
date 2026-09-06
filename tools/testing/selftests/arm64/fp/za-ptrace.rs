//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/arm64/fp/za-ptrace.c
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

//
// The architecture defines the maximum VQ as 16 but for extensibility
// the kernel specifies the SVE_VQ_MAX as 512 resulting in us running
// a *lot* more tests than are useful if we use it.  Until the
// architecture is extended let's limit our coverage to what is
// currently allowed, plus one extra to ensure we cover constraining
// the VL as expected.
//
pub const TEST_VQ_MAX: c_int = 17;

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
    ksft_exit_fail_msg("ptrace(PTRACE_TRACEME) failed: %s (%d)",
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
// Validate attempting to set the specfied VL via ptrace
#[no_mangle]
unsafe extern "C" fn ptrace_set_get_vl(child: pid_t, vl: c_uint, supported: *mut bool) {
    static void ptrace_set_get_vl(pid_t child, unsigned int vl, bool *supported)
    {
    struct user_za_header za;
    struct user_za_header *new_za = core::ptr::null_mut();
    let mut new_za_size: usize = 0;
    int ret, prctl_vl;
// supported = false;
// Check if the VL is supported in this process
    prctl_vl = prctl(PR_SME_SET_VL, vl);
    if (prctl_vl == -1)
    ksft_exit_fail_msg("prctl(PR_SME_SET_VL) failed: %s (%d)\n",
    strerror(errno), errno);
// If the VL is not supported then a supported VL will be returned
// supported = (prctl_vl == vl);
// Set the VL by doing a set with no register payload
    memset(&za, 0, sizeof(za));
    za.size = sizeof(za);
    za.vl = vl;
    ret = set_za(child, &za);
    if (ret != 0) {
    ksft_test_result_fail("Failed to set VL %u\n", vl);
    return;
    }
//
// Read back the new register state and verify that we have the
// same VL that we got from prctl() on ourselves.
//
    if (!get_za(child, (void **)&new_za, &new_za_size)) {
    ksft_test_result_fail("Failed to read VL %u\n", vl);
    return;
    }
    ksft_test_result(new_za.vl = prctl_vl, "Set VL %u\n", vl);
    free(new_za);
    }
// Validate attempting to set no ZA data and read it back
#[no_mangle]
unsafe extern "C" fn ptrace_set_no_data(child: pid_t, vl: c_uint) {
    static void ptrace_set_no_data(pid_t child, unsigned int vl)
    {
    void *read_buf = core::ptr::null_mut();
    struct user_za_header write_za;
    struct user_za_header *read_za;
    let mut read_za_size: usize = 0;
    int ret;
// Set up some data and write it out
    memset(&write_za, 0, sizeof(write_za));
    write_za.size = ZA_PT_ZA_OFFSET;
    write_za.vl = vl;
    ret = set_za(child, &write_za);
    if (ret != 0) {
    ksft_test_result_fail("Failed to set VL %u no data\n", vl);
    return;
    }
// Read the data back
    if (!get_za(child, (void **)&read_buf, &read_za_size)) {
    ksft_test_result_fail("Failed to read VL %u no data\n", vl);
    return;
    }
    read_za = read_buf;
// We might read more data if there's extensions we don't know
    if (read_za.size < write_za.size) {
    ksft_test_result_fail("VL %u wrote %d bytes, only read %d\n",
    vl, write_za.size, read_za.size);
    goto out_read;
    }
    ksft_test_result(read_za.size == write_za.size,
    "Disabled ZA for VL %u\n", vl);
    out_read:
    free(read_buf);
    }
// Validate attempting to set data and read it back
#[no_mangle]
unsafe extern "C" fn ptrace_set_get_data(child: pid_t, vl: c_uint) {
    static void ptrace_set_get_data(pid_t child, unsigned int vl)
    {
    void *write_buf;
    void *read_buf = core::ptr::null_mut();
    struct user_za_header *write_za;
    struct user_za_header *read_za;
    let mut read_za_size: usize = 0;
    let mut vq: c_uint = sve_vq_from_vl(vl);
    int ret;
    size_t data_size;
    data_size = ZA_PT_SIZE(vq);
    write_buf = malloc(data_size);
    if (!write_buf) {
    ksft_test_result_fail("Error allocating %ld byte buffer for VL %u\n",
    data_size, vl);
    return;
    }
    write_za = write_buf;
// Set up some data and write it out
    memset(write_za, 0, data_size);
    write_za.size = data_size;
    write_za.vl = vl;
    fill_buf(write_buf + ZA_PT_ZA_OFFSET, ZA_PT_ZA_SIZE(vq));
    ret = set_za(child, write_za);
    if (ret != 0) {
    ksft_test_result_fail("Failed to set VL %u data\n", vl);
    goto out;
    }
// Read the data back
    if (!get_za(child, (void **)&read_buf, &read_za_size)) {
    ksft_test_result_fail("Failed to read VL %u data\n", vl);
    goto out;
    }
    read_za = read_buf;
// We might read more data if there's extensions we don't know
    if (read_za.size < write_za.size) {
    ksft_test_result_fail("VL %u wrote %d bytes, only read %d\n",
    vl, write_za.size, read_za.size);
    goto out_read;
    }
    ksft_test_result(memcmp(write_buf + ZA_PT_ZA_OFFSET,
    read_buf + ZA_PT_ZA_OFFSET,
    ZA_PT_ZA_SIZE(vq)) == 0,
    "Data match for VL %u\n", vl);
    out_read:
    free(read_buf);
    out:
    free(write_buf);
    }
#[no_mangle]
unsafe extern "C" fn do_parent(child: pid_t) -> c_int {
    static int do_parent(pid_t child)
    {
    let mut ret: c_int = EXIT_FAILURE;
    pid_t pid;
    int status;
    siginfo_t si;
    unsigned int vq, vl;
    bool vl_supported;
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
// Step through every possible VQ
    for (vq = SVE_VQ_MIN; vq <= TEST_VQ_MAX; vq++) {
    vl = sve_vl_from_vq(vq);
// First, try to set this vector length
    ptrace_set_get_vl(child, vl, &vl_supported);
// If the VL is supported validate data set/get
    if (vl_supported) {
    ptrace_set_no_data(child, vl);
    ptrace_set_get_data(child, vl);
    } else {
    ksft_test_result_skip("Disabled ZA for VL %u\n", vl);
    ksft_test_result_skip("Get and set data for VL %u\n",
    vl);
    }
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
    if (!(getauxval(AT_HWCAP2) & HWCAP2_SME)) {
    ksft_set_plan(1);
    ksft_exit_skip("SME not available\n");
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

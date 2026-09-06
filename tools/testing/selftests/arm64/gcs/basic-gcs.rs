//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/arm64/gcs/basic-gcs.c
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
// Copyright (C) 2023 ARM Limited.
//

// nolibc doesn't have sysconf(), just hard code the maximum
    let mut page_size: static size_t = 65536;
#[no_mangle]
pub unsafe extern "C" fn __attribute__(valid_gcs_function(void: (noinline)) void) -> static {
    static  __attribute__((noinline)) void valid_gcs_function(void)
    {
// Do something the compiler can't optimise out
    syscall(__NR_prctl, PR_SVE_GET_VL);
    }
#[no_mangle]
pub unsafe extern "C" fn gcs_set_status(mode: c_ulong) -> c_int {
    static inline int gcs_set_status(unsigned long mode)
    {
    let mut enabling: bool = mode & PR_SHADOW_STACK_ENABLE;
    int ret;
    unsigned long new_mode;
//
// The prctl takes 1 argument but we need to ensure that the
// other 3 values passed in registers to the syscall are zero
// since the kernel validates them.
//
    ret = syscall(__NR_prctl, PR_SET_SHADOW_STACK_STATUS, mode, 0, 0, 0);
    if (ret == 0) {
    ret = syscall(__NR_prctl, PR_GET_SHADOW_STACK_STATUS, &new_mode, 0, 0, 0);
    if (ret == 0) {
    if (new_mode != mode) {
    ksft_print_msg("Mode set to %lx not %lx\n",
    new_mode, mode);
    ret = -EINVAL;
    }
    } else {
    ksft_print_msg("Failed to validate mode: %d\n", errno);
    }
    if (enabling != chkfeat_gcs()) {
    ksft_print_msg("%senabled by prctl but %senabled in CHKFEAT\n",
    enabling ? "" : "not ",
    chkfeat_gcs() ? "" : "not ");
    ret = -EINVAL;
    }
    }
    return ret;
    }
// Try to read the status
#[no_mangle]
unsafe extern "C" fn read_status() -> bool {
    static bool read_status(void)
    {
    unsigned long state;
    int ret;
    ret = syscall(__NR_prctl, PR_GET_SHADOW_STACK_STATUS, &state, 0, 0, 0);
    if (ret != 0) {
    ksft_print_msg("Failed to read state: %d\n", errno);
    return false;
    }
    return state & PR_SHADOW_STACK_ENABLE;
    }
// Just a straight enable
#[no_mangle]
unsafe extern "C" fn base_enable() -> bool {
    static bool base_enable(void)
    {
    int ret;
    ret = gcs_set_status(PR_SHADOW_STACK_ENABLE);
    if (ret) {
    ksft_print_msg("PR_SHADOW_STACK_ENABLE failed %d\n", ret);
    return false;
    }
    return true;
    }
// Check we can read GCSPR_EL0 when GCS is enabled
#[no_mangle]
unsafe extern "C" fn read_gcspr_el0() -> bool {
    static bool read_gcspr_el0(void)
    {
    unsigned long *gcspr_el0;
    ksft_print_msg("GET GCSPR\n");
    gcspr_el0 = get_gcspr();
    ksft_print_msg("GCSPR_EL0 is %p\n", gcspr_el0);
    return true;
    }
// Also allow writes to stack
#[no_mangle]
unsafe extern "C" fn enable_writeable() -> bool {
    static bool enable_writeable(void)
    {
    int ret;
    ret = gcs_set_status(PR_SHADOW_STACK_ENABLE | PR_SHADOW_STACK_WRITE);
    if (ret) {
    ksft_print_msg("PR_SHADOW_STACK_ENABLE writeable failed: %d\n", ret);
    return false;
    }
    ret = gcs_set_status(PR_SHADOW_STACK_ENABLE);
    if (ret) {
    ksft_print_msg("failed to restore plain enable %d\n", ret);
    return false;
    }
    return true;
    }
// Also allow writes to stack
#[no_mangle]
unsafe extern "C" fn enable_push_pop() -> bool {
    static bool enable_push_pop(void)
    {
    int ret;
    ret = gcs_set_status(PR_SHADOW_STACK_ENABLE | PR_SHADOW_STACK_PUSH);
    if (ret) {
    ksft_print_msg("PR_SHADOW_STACK_ENABLE with push failed: %d\n",
    ret);
    return false;
    }
    ret = gcs_set_status(PR_SHADOW_STACK_ENABLE);
    if (ret) {
    ksft_print_msg("failed to restore plain enable %d\n", ret);
    return false;
    }
    return true;
    }
// Enable GCS and allow everything
#[no_mangle]
unsafe extern "C" fn enable_all() -> bool {
    static bool enable_all(void)
    {
    int ret;
    ret = gcs_set_status(PR_SHADOW_STACK_ENABLE | PR_SHADOW_STACK_PUSH |
    PR_SHADOW_STACK_WRITE);
    if (ret) {
    ksft_print_msg("PR_SHADOW_STACK_ENABLE with everything failed: %d\n",
    ret);
    return false;
    }
    ret = gcs_set_status(PR_SHADOW_STACK_ENABLE);
    if (ret) {
    ksft_print_msg("failed to restore plain enable %d\n", ret);
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn enable_invalid() -> bool {
    static bool enable_invalid(void)
    {
    let mut ret: c_int = gcs_set_status(ULONG_MAX);
    if (ret == 0) {
    ksft_print_msg("GCS_SET_STATUS %lx succeeded\n", ULONG_MAX);
    return false;
    }
    return true;
    }
// Map a GCS
#[no_mangle]
unsafe extern "C" fn map_guarded_stack() -> bool {
    static bool map_guarded_stack(void)
    {
    int ret;
    uint64_t *buf;
    uint64_t expected_cap;
    int elem;
    let mut pass: bool = true;
    buf = (void *)syscall(__NR_map_shadow_stack, 0, page_size,
    SHADOW_STACK_SET_MARKER | SHADOW_STACK_SET_TOKEN);
    if (buf == MAP_FAILED) {
    ksft_print_msg("Failed to map %lu byte GCS: %d\n",
    page_size, errno);
    return false;
    }
    ksft_print_msg("Mapped GCS at %p-%p\n", buf,
    (void *)((uint64_t)buf + page_size));
// The top of the newly allocated region should be 0
    elem = (page_size / sizeof(uint64_t)) - 1;
    if (buf[elem]) {
    ksft_print_msg("Last entry is 0x%llx not 0x0\n", buf[elem]);
    pass = false;
    }
// Then a valid cap token
    elem--;
    expected_cap = ((uint64_t)buf + page_size - 16);
    expected_cap &= GCS_CAP_ADDR_MASK;
    expected_cap |= GCS_CAP_VALID_TOKEN;
    if (buf[elem] != expected_cap) {
    ksft_print_msg("Cap entry is 0x%llx not 0x%llx\n",
    buf[elem], expected_cap);
    pass = false;
    }
    ksft_print_msg("cap token is 0x%llx\n", buf[elem]);
// The rest should be zeros
    for (elem = 0; elem < page_size / sizeof(uint64_t) - 2; elem++) {
    if (!buf[elem])
    continue;
    ksft_print_msg("GCS slot %d is 0x%llx not 0x0\n",
    elem, buf[elem]);
    pass = false;
    }
    ret = munmap(buf, page_size);
    if (ret != 0) {
    ksft_print_msg("Failed to unmap %ld byte GCS: %d\n",
    page_size, errno);
    pass = false;
    }
    return pass;
    }
// A fork()ed process can run
#[no_mangle]
unsafe extern "C" fn test_fork() -> bool {
    static bool test_fork(void)
    {
    unsigned long child_mode;
    int ret, status;
    pid_t pid;
    let mut pass: bool = true;
    pid = fork();
    if (pid == -1) {
    ksft_print_msg("fork() failed: %d\n", errno);
    pass = false;
    goto out;
    }
    if (pid == 0) {
// In child, make sure we can call a function, read
// the GCS pointer and status and then exit
    valid_gcs_function();
    get_gcspr();
    ret = syscall(__NR_prctl, PR_GET_SHADOW_STACK_STATUS, &child_mode, 0, 0, 0);
    if (ret == 0 && !(child_mode & PR_SHADOW_STACK_ENABLE)) {
    ksft_print_msg("GCS not enabled in child\n");
    ret = -EINVAL;
    }
    exit(ret);
    }
//
// In parent, check we can still do function calls then block
// for the child.
//
    valid_gcs_function();
    ksft_print_msg("Waiting for child %d\n", pid);
    ret = waitpid(pid, &status, 0);
    if (ret == -1) {
    ksft_print_msg("Failed to wait for child: %d\n",
    errno);
    return false;
    }
    if (!WIFEXITED(status)) {
    ksft_print_msg("Child exited due to signal %d\n",
    WTERMSIG(status));
    pass = false;
    } else {
    if (WEXITSTATUS(status)) {
    ksft_print_msg("Child exited with status %d\n",
    WEXITSTATUS(status));
    pass = false;
    }
    }
    out:
    return pass;
    }
// A vfork()ed process can run and exit
#[no_mangle]
unsafe extern "C" fn test_vfork() -> bool {
    static bool test_vfork(void)
    {
    unsigned long child_mode;
    int ret, status;
    pid_t pid;
    let mut pass: bool = true;
    pid = vfork();
    if (pid == -1) {
    ksft_print_msg("vfork() failed: %d\n", errno);
    pass = false;
    goto out;
    }
    if (pid == 0) {
//
// In child, make sure we can call a function, read
// the GCS pointer and status and then exit.
//
    valid_gcs_function();
    get_gcspr();
    ret = syscall(__NR_prctl, PR_GET_SHADOW_STACK_STATUS, &child_mode, 0, 0, 0);
    if (ret == 0 && !(child_mode & PR_SHADOW_STACK_ENABLE)) {
    ksft_print_msg("GCS not enabled in child\n");
    ret = EXIT_FAILURE;
    }
    _exit(ret);
    }
//
// In parent, check we can still do function calls then check
// on the child.
//
    valid_gcs_function();
    ksft_print_msg("Waiting for child %d\n", pid);
    ret = waitpid(pid, &status, 0);
    if (ret == -1) {
    ksft_print_msg("Failed to wait for child: %d\n",
    errno);
    return false;
    }
    if (!WIFEXITED(status)) {
    ksft_print_msg("Child exited due to signal %d\n",
    WTERMSIG(status));
    pass = false;
    } else if (WEXITSTATUS(status)) {
    ksft_print_msg("Child exited with status %d\n",
    WEXITSTATUS(status));
    pass = false;
    }
    out:
    return pass;
    }
    typedef bool (*gcs_test)(void);
    static struct {
    char *name;
    gcs_test test;
    bool needs_enable;
    } tests[] = {
    { "read_status", read_status },
    { "base_enable", base_enable, true },
    { "read_gcspr_el0", read_gcspr_el0 },
    { "enable_writeable", enable_writeable, true },
    { "enable_push_pop", enable_push_pop, true },
    { "enable_all", enable_all, true },
    { "enable_invalid", enable_invalid, true },
    { "map_guarded_stack", map_guarded_stack },
    { "fork", test_fork },
    { "vfork", test_vfork },
    };
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    int i, ret;
    unsigned long gcs_mode;
    ksft_print_header();
    if (!(getauxval(AT_HWCAP) & HWCAP_GCS))
    ksft_exit_skip("SKIP GCS not supported\n");
    ret = syscall(__NR_prctl, PR_GET_SHADOW_STACK_STATUS, &gcs_mode, 0, 0, 0);
    if (ret != 0)
    ksft_exit_fail_msg("Failed to read GCS state: %d\n", errno);
    if (!(gcs_mode & PR_SHADOW_STACK_ENABLE)) {
    gcs_mode = PR_SHADOW_STACK_ENABLE;
    ret = syscall(__NR_prctl, PR_SET_SHADOW_STACK_STATUS, gcs_mode, 0, 0, 0);
    if (ret != 0)
    ksft_exit_fail_msg("Failed to enable GCS: %d\n", errno);
    }
    ksft_set_plan(ARRAY_SIZE(tests));
    for (i = 0; i < ARRAY_SIZE(tests); i++) {
    ksft_test_result((*tests[i].test)(), "%s\n", tests[i].name);
    }
// One last test: disable GCS, we can do this one time
    ret = syscall(__NR_prctl, PR_SET_SHADOW_STACK_STATUS, 0, 0, 0, 0);
    if (ret != 0)
    ksft_print_msg("Failed to disable GCS: %d\n", errno);
    ksft_finished();
    return 0;
    }

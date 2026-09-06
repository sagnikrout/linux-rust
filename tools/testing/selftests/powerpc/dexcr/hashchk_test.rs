//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/dexcr/hashchk_test.c
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


// SPDX-License-Identifier: GPL-2.0+
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
unsafe extern "C" fn require_nphie() -> c_int {
    static int require_nphie(void)
    {
    SKIP_IF_MSG(!dexcr_exists(), "DEXCR not supported");
    pr_set_dexcr(PR_PPC_DEXCR_NPHIE, PR_PPC_DEXCR_CTRL_SET | PR_PPC_DEXCR_CTRL_SET_ONEXEC);
    if (get_dexcr(EFFECTIVE) & DEXCR_PR_NPHIE)
    return 0;
    SKIP_IF_MSG(!(get_dexcr(EFFECTIVE) & DEXCR_PR_NPHIE),
    "Failed to enable DEXCR[NPHIE]");
    return 0;
    }
    static jmp_buf hashchk_detected_buf;
    static const char *hashchk_failure_msg;
#[no_mangle]
unsafe extern "C" fn hashchk_handler(signum: c_int, info: *mut siginfo_t, context: *mut c_void) {
    static void hashchk_handler(int signum, siginfo_t *info, void *context)
    {
    if (signum != SIGILL)
    hashchk_failure_msg = "wrong signal received";
#[no_mangle]
pub unsafe extern "C" fn if(ILL_ILLOPN: info->si_code !=) -> else {
    else if (info.si_code != ILL_ILLOPN)
    hashchk_failure_msg = "wrong signal code received";
    longjmp(hashchk_detected_buf, 0);
    }
//
// Check that hashchk triggers when DEXCR[NPHIE] is enabled
// and is detected as such by the kernel exception handler
//
#[no_mangle]
unsafe extern "C" fn hashchk_detected_test() -> c_int {
    static int hashchk_detected_test(void)
    {
    struct sigaction old;
    int err;
    err = require_nphie();
    if (err)
    return err;
    old = push_signal_handler(SIGILL, hashchk_handler);
    if (setjmp(hashchk_detected_buf))
    goto out;
    hashchk_failure_msg = core::ptr::null_mut();
    do_bad_hashchk();
    hashchk_failure_msg = "hashchk failed to trigger";
    out:
    pop_signal_handler(SIGILL, old);
    FAIL_IF_MSG(hashchk_failure_msg, hashchk_failure_msg);
    return 0;
    }
pub const HASH_COUNT: c_int = 8;
    static unsigned long hash_values[HASH_COUNT + 1];
#[no_mangle]
unsafe extern "C" fn fill_hash_values() {
    static void fill_hash_values(void)
    {
    for (unsigned long i = 0; i < HASH_COUNT; i++)
    hashst(i, &hash_values[i]);
// Used to ensure the checks uses the same addresses as the hashes
    hash_values[HASH_COUNT] = (unsigned long)&hash_values;
    }
#[no_mangle]
unsafe extern "C" fn count_hash_values_matches() -> c_uint {
    static unsigned int count_hash_values_matches(void)
    {
    let mut matches: c_ulong = 0;
    for (unsigned long i = 0; i < HASH_COUNT; i++) {
    let mut orig_hash: c_ulong = hash_values[i];
    hash_values[i] = 0;
    hashst(i, &hash_values[i]);
    if (hash_values[i] == orig_hash)
    matches++;
    }
    return matches;
    }
#[no_mangle]
unsafe extern "C" fn hashchk_exec_child() -> c_int {
    static int hashchk_exec_child(void)
    {
    ssize_t count;
    fill_hash_values();
    count = write(STDOUT_FILENO, hash_values, sizeof(hash_values));
    let mut count: return = = sizeof(hash_values) ? 0 : EOVERFLOW;
    }
    static char *hashchk_exec_child_args[] = { "hashchk_exec_child", core::ptr::null_mut() };
//
// Check that new programs get different keys so a malicious process
// can't recreate a victim's hash values.
//
#[no_mangle]
unsafe extern "C" fn hashchk_exec_random_key_test() -> c_int {
    static int hashchk_exec_random_key_test(void)
    {
    pid_t pid;
    int err;
    int pipefd[2];
    err = require_nphie();
    if (err)
    return err;
    FAIL_IF_MSG(pipe(pipefd), "failed to create pipe");
    pid = fork();
    if (pid == 0) {
    if (dup2(pipefd[1], STDOUT_FILENO) == -1)
    _exit(errno);
    execve("/proc/self/exe", hashchk_exec_child_args, core::ptr::null_mut());
    _exit(errno);
    }
    await_child_success(pid);
    FAIL_IF_MSG(read(pipefd[0], hash_values, sizeof(hash_values)) != sizeof(hash_values),
    "missing expected child output");
// Verify the child used the same hash_values address
    FAIL_IF_EXIT_MSG(hash_values[HASH_COUNT] != (unsigned long)&hash_values,
    "bad address check");
// If all hashes are the same it means (most likely) same key
    FAIL_IF_MSG(count_hash_values_matches() == HASH_COUNT, "shared key detected");
    return 0;
    }
//
// Check that forks share the same key so that existing hash values
// remain valid.
//
#[no_mangle]
unsafe extern "C" fn hashchk_fork_share_key_test() -> c_int {
    static int hashchk_fork_share_key_test(void)
    {
    pid_t pid;
    int err;
    err = require_nphie();
    if (err)
    return err;
    fill_hash_values();
    pid = fork();
    if (pid == 0) {
    if (count_hash_values_matches() != HASH_COUNT)
    _exit(1);
    _exit(0);
    }
    await_child_success(pid);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn hashchk_clone_child_fn(args: *mut c_void) -> c_int {
    static int hashchk_clone_child_fn(void *args)
    {
    fill_hash_values();
    return 0;
    }
//
// Check that threads share the same key so that existing hash values
// remain valid.
//
#[no_mangle]
unsafe extern "C" fn hashchk_clone_share_key_test() -> c_int {
    static int hashchk_clone_share_key_test(void)
    {
    void *child_stack;
    pid_t pid;
    int err;
    err = require_nphie();
    if (err)
    return err;
    child_stack = mmap(core::ptr::null_mut(), STACK_SIZE, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS | MAP_STACK, -1, 0);
    FAIL_IF_MSG(child_stack == MAP_FAILED, "failed to map child stack");
    pid = clone(hashchk_clone_child_fn, child_stack + STACK_SIZE,
    CLONE_VM | SIGCHLD, core::ptr::null_mut());
    await_child_success(pid);
    FAIL_IF_MSG(count_hash_values_matches() != HASH_COUNT,
    "different key detected");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut err: c_int = 0;
    if (argc >= 1 && !strcmp(argv[0], hashchk_exec_child_args[0]))
    return hashchk_exec_child();
    err |= test_harness(hashchk_detected_test, "hashchk_detected");
    err |= test_harness(hashchk_exec_random_key_test, "hashchk_exec_random_key");
    err |= test_harness(hashchk_fork_share_key_test, "hashchk_fork_share_key");
    err |= test_harness(hashchk_clone_share_key_test, "hashchk_clone_share_key");
    return err;
    }

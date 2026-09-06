//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/dexcr/dexcr_test.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

//
// Helper function for testing the behaviour of a newly exec-ed process
//
#[no_mangle]
unsafe extern "C" fn dexcr_prctl_onexec_test_child(which: c_ulong, status: *const c_char) -> c_int {
    static int dexcr_prctl_onexec_test_child(unsigned long which, const char *status)
    {
    let mut dexcr: c_ulong = mfspr(SPRN_DEXCR_RO);
    let mut aspect: c_ulong = pr_which_to_aspect(which);
    let mut ctrl: c_int = pr_get_dexcr(which);
    if (!strcmp(status, "set")) {
    FAIL_IF_EXIT_MSG(!(ctrl & PR_PPC_DEXCR_CTRL_SET),
    "setting aspect across exec not applied");
    FAIL_IF_EXIT_MSG(!(ctrl & PR_PPC_DEXCR_CTRL_SET_ONEXEC),
    "setting aspect across exec not inherited");
    FAIL_IF_EXIT_MSG(!(aspect & dexcr), "setting aspect across exec did not take effect");
    } else if (!strcmp(status, "clear")) {
    FAIL_IF_EXIT_MSG(!(ctrl & PR_PPC_DEXCR_CTRL_CLEAR),
    "clearing aspect across exec not applied");
    FAIL_IF_EXIT_MSG(!(ctrl & PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC),
    "clearing aspect across exec not inherited");
    FAIL_IF_EXIT_MSG(aspect & dexcr, "clearing aspect across exec did not take effect");
    } else {
    FAIL_IF_EXIT_MSG(true, "unknown expected status");
    }
    return 0;
    }
//
// Test that the given prctl value can be manipulated freely
//
#[no_mangle]
unsafe extern "C" fn dexcr_prctl_aspect_test(which: c_ulong) -> c_int {
    static int dexcr_prctl_aspect_test(unsigned long which)
    {
    let mut aspect: c_ulong = pr_which_to_aspect(which);
    pid_t pid;
    int ctrl;
    int err;
    int errno_save;
    SKIP_IF_MSG(!dexcr_exists(), "DEXCR not supported");
    SKIP_IF_MSG(!pr_dexcr_aspect_supported(which), "DEXCR aspect not supported");
    SKIP_IF_MSG(!pr_dexcr_aspect_editable(which), "DEXCR aspect not editable with prctl");
// We reject invalid combinations of arguments
    err = pr_set_dexcr(which, PR_PPC_DEXCR_CTRL_SET | PR_PPC_DEXCR_CTRL_CLEAR);
    errno_save = errno;
    FAIL_IF_MSG(err != -1, "simultaneous set and clear should be rejected");
    FAIL_IF_MSG(errno_save != EINVAL, "simultaneous set and clear should be rejected with EINVAL");
    err = pr_set_dexcr(which, PR_PPC_DEXCR_CTRL_SET_ONEXEC | PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC);
    errno_save = errno;
    FAIL_IF_MSG(err != -1, "simultaneous set and clear on exec should be rejected");
    FAIL_IF_MSG(errno_save != EINVAL, "simultaneous set and clear on exec should be rejected with EINVAL");
// We set the aspect
    err = pr_set_dexcr(which, PR_PPC_DEXCR_CTRL_SET);
    FAIL_IF_MSG(err, "PR_PPC_DEXCR_CTRL_SET failed");
    ctrl = pr_get_dexcr(which);
    FAIL_IF_MSG(!(ctrl & PR_PPC_DEXCR_CTRL_SET), "config value not PR_PPC_DEXCR_CTRL_SET");
    FAIL_IF_MSG(ctrl & PR_PPC_DEXCR_CTRL_CLEAR, "config value unexpected clear flag");
    FAIL_IF_MSG(!(aspect & mfspr(SPRN_DEXCR_RO)), "setting aspect did not take effect");
// We clear the aspect
    err = pr_set_dexcr(which, PR_PPC_DEXCR_CTRL_CLEAR);
    FAIL_IF_MSG(err, "PR_PPC_DEXCR_CTRL_CLEAR failed");
    ctrl = pr_get_dexcr(which);
    FAIL_IF_MSG(!(ctrl & PR_PPC_DEXCR_CTRL_CLEAR), "config value not PR_PPC_DEXCR_CTRL_CLEAR");
    FAIL_IF_MSG(ctrl & PR_PPC_DEXCR_CTRL_SET, "config value unexpected set flag");
    FAIL_IF_MSG(aspect & mfspr(SPRN_DEXCR_RO), "clearing aspect did not take effect");
// We make it set on exec (doesn't change our current value)
    err = pr_set_dexcr(which, PR_PPC_DEXCR_CTRL_SET_ONEXEC);
    FAIL_IF_MSG(err, "PR_PPC_DEXCR_CTRL_SET_ONEXEC failed");
    ctrl = pr_get_dexcr(which);
    FAIL_IF_MSG(!(ctrl & PR_PPC_DEXCR_CTRL_CLEAR), "process aspect should still be cleared");
    FAIL_IF_MSG(!(ctrl & PR_PPC_DEXCR_CTRL_SET_ONEXEC), "config value not PR_PPC_DEXCR_CTRL_SET_ONEXEC");
    FAIL_IF_MSG(ctrl & PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC, "config value unexpected clear on exec flag");
    FAIL_IF_MSG(aspect & mfspr(SPRN_DEXCR_RO), "scheduling aspect to set on exec should not change it now");
// We make it clear on exec (doesn't change our current value)
    err = pr_set_dexcr(which, PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC);
    FAIL_IF_MSG(err, "PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC failed");
    ctrl = pr_get_dexcr(which);
    FAIL_IF_MSG(!(ctrl & PR_PPC_DEXCR_CTRL_CLEAR), "process aspect config should still be cleared");
    FAIL_IF_MSG(!(ctrl & PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC), "config value not PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC");
    FAIL_IF_MSG(ctrl & PR_PPC_DEXCR_CTRL_SET_ONEXEC, "config value unexpected set on exec flag");
    FAIL_IF_MSG(aspect & mfspr(SPRN_DEXCR_RO), "process aspect should still be cleared");
// We allow setting the current and on-exec value in a single call
    err = pr_set_dexcr(which, PR_PPC_DEXCR_CTRL_SET | PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC);
    FAIL_IF_MSG(err, "PR_PPC_DEXCR_CTRL_SET | PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC failed");
    ctrl = pr_get_dexcr(which);
    FAIL_IF_MSG(!(ctrl & PR_PPC_DEXCR_CTRL_SET), "config value not PR_PPC_DEXCR_CTRL_SET");
    FAIL_IF_MSG(!(ctrl & PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC), "config value not PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC");
    FAIL_IF_MSG(!(aspect & mfspr(SPRN_DEXCR_RO)), "process aspect should be set");
    err = pr_set_dexcr(which, PR_PPC_DEXCR_CTRL_CLEAR | PR_PPC_DEXCR_CTRL_SET_ONEXEC);
    FAIL_IF_MSG(err, "PR_PPC_DEXCR_CTRL_CLEAR | PR_PPC_DEXCR_CTRL_SET_ONEXEC failed");
    ctrl = pr_get_dexcr(which);
    FAIL_IF_MSG(!(ctrl & PR_PPC_DEXCR_CTRL_CLEAR), "config value not PR_PPC_DEXCR_CTRL_CLEAR");
    FAIL_IF_MSG(!(ctrl & PR_PPC_DEXCR_CTRL_SET_ONEXEC), "config value not PR_PPC_DEXCR_CTRL_SET_ONEXEC");
    FAIL_IF_MSG(aspect & mfspr(SPRN_DEXCR_RO), "process aspect should be clear");
// Verify the onexec value is applied across exec
    pid = fork();
    if (!pid) {
    char which_str[32] = {};
    char *args[] = { "dexcr_prctl_onexec_test_child", which_str, "set", core::ptr::null_mut() };
    let mut ctrl: c_uint = pr_get_dexcr(which);
    sprintf(which_str, "%lu", which);
    FAIL_IF_EXIT_MSG(!(ctrl & PR_PPC_DEXCR_CTRL_SET_ONEXEC),
    "setting aspect on exec not copied across fork");
    FAIL_IF_EXIT_MSG(mfspr(SPRN_DEXCR_RO) & aspect,
    "setting aspect on exec wrongly applied to fork");
    execve("/proc/self/exe", args, core::ptr::null_mut());
    _exit(errno);
    }
    await_child_success(pid);
    err = pr_set_dexcr(which, PR_PPC_DEXCR_CTRL_SET | PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC);
    FAIL_IF_MSG(err, "PR_PPC_DEXCR_CTRL_SET | PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC failed");
    pid = fork();
    if (!pid) {
    char which_str[32] = {};
    char *args[] = { "dexcr_prctl_onexec_test_child", which_str, "clear", core::ptr::null_mut() };
    let mut ctrl: c_uint = pr_get_dexcr(which);
    sprintf(which_str, "%lu", which);
    FAIL_IF_EXIT_MSG(!(ctrl & PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC),
    "clearing aspect on exec not copied across fork");
    FAIL_IF_EXIT_MSG(!(mfspr(SPRN_DEXCR_RO) & aspect),
    "clearing aspect on exec wrongly applied to fork");
    execve("/proc/self/exe", args, core::ptr::null_mut());
    _exit(errno);
    }
    await_child_success(pid);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dexcr_prctl_ibrtpd_test() -> c_int {
    static int dexcr_prctl_ibrtpd_test(void)
    {
    return dexcr_prctl_aspect_test(PR_PPC_DEXCR_IBRTPD);
    }
#[no_mangle]
unsafe extern "C" fn dexcr_prctl_srapd_test() -> c_int {
    static int dexcr_prctl_srapd_test(void)
    {
    return dexcr_prctl_aspect_test(PR_PPC_DEXCR_SRAPD);
    }
#[no_mangle]
unsafe extern "C" fn dexcr_prctl_nphie_test() -> c_int {
    static int dexcr_prctl_nphie_test(void)
    {
    return dexcr_prctl_aspect_test(PR_PPC_DEXCR_NPHIE);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut err: c_int = 0;
//
// Some tests require checking what happens across exec, so we may be
// invoked as the child of a particular test
//
    if (argc > 1) {
    if (argc == 3 && !strcmp(argv[0], "dexcr_prctl_onexec_test_child")) {
    unsigned long which;
    err = parse_ulong(argv[1], strlen(argv[1]), &which, 10);
    FAIL_IF_MSG(err, "failed to parse which value for child");
    return dexcr_prctl_onexec_test_child(which, argv[2]);
    }
    FAIL_IF_MSG(true, "unknown test case");
    }
//
// Otherwise we are the main test invocation and run the full suite
//
    err |= test_harness(dexcr_prctl_ibrtpd_test, "dexcr_prctl_ibrtpd");
    err |= test_harness(dexcr_prctl_srapd_test, "dexcr_prctl_srapd");
    err |= test_harness(dexcr_prctl_nphie_test, "dexcr_prctl_nphie");
    return err;
    }

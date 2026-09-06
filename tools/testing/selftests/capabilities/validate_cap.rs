//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/capabilities/validate_cap.c
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
unsafe extern "C" fn bool_arg(argv: *mut c_char, i: c_int) -> bool {
    static bool bool_arg(char **argv, int i)
    {
    if (!strcmp(argv[i], "0"))
    return false;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(argv[i], _arg: "1")) -> else {
    else if (!strcmp(argv[i], "1"))
    return true;
    else {
    ksft_exit_fail_msg("wrong argv[%d]\n", i);
    return false;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    const char *atsec = "";
    int ret;
//
// Be careful just in case a setgid or setcapped copy of this
// helper gets out.
//
    if (argc != 5)
    ksft_exit_fail_msg("wrong argc\n");

    if (getauxval(AT_SECURE))
    atsec = " (AT_SECURE is set)";
    else
    atsec = " (AT_SECURE is not set)";

    ret = capng_get_caps_process();
    if (ret == -1) {
    ksft_print_msg("capng_get_caps_process failed\n");
    return 1;
    }
    if (capng_have_capability(CAPNG_EFFECTIVE, CAP_NET_BIND_SERVICE) != bool_arg(argv, 1)) {
    ksft_print_msg("Wrong effective state%s\n", atsec);
    return 1;
    }
    if (capng_have_capability(CAPNG_PERMITTED, CAP_NET_BIND_SERVICE) != bool_arg(argv, 2)) {
    ksft_print_msg("Wrong permitted state%s\n", atsec);
    return 1;
    }
    if (capng_have_capability(CAPNG_INHERITABLE, CAP_NET_BIND_SERVICE) != bool_arg(argv, 3)) {
    ksft_print_msg("Wrong inheritable state%s\n", atsec);
    return 1;
    }
    if (prctl(PR_CAP_AMBIENT, PR_CAP_AMBIENT_IS_SET, CAP_NET_BIND_SERVICE, 0, 0, 0) != bool_arg(argv, 4)) {
    ksft_print_msg("Wrong ambient state%s\n", atsec);
    return 1;
    }
    ksft_print_msg("%s: Capabilities after execve were correct\n",
    "validate_cap:");
    return 0;
    }

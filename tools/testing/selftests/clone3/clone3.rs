//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/clone3/clone3.c
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
// Based on Christian Brauner's clone3() example
// Macro flag: #define _GNU_SOURCE

    enum test_mode {
    CLONE3_ARGS_NO_TEST,
    CLONE3_ARGS_ALL_0,
    CLONE3_ARGS_INVAL_EXIT_SIGNAL_BIG,
    CLONE3_ARGS_INVAL_EXIT_SIGNAL_NEG,
    CLONE3_ARGS_INVAL_EXIT_SIGNAL_CSIG,
    CLONE3_ARGS_INVAL_EXIT_SIGNAL_NSIG,
    };
#[no_mangle]
unsafe extern "C" fn call_clone3(flags: u64, size: usize, test_mode: enum test_mode) -> c_int {
    static int call_clone3(uint64_t flags, size_t size, enum test_mode test_mode)
    {
    struct __clone_args args = {
    .flags = flags,
    .exit_signal = SIGCHLD,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clone_args_extended {
    pub args: __clone_args,
    pub excess_space: [__aligned_u64; 2],
    pub args_ext: },
    pub -1: pid_t pid =,
    pub status: c_int,
    pub sizeof(args_ext)): memset(&args_ext, 0,,
    if (size > sizeof(struct __clone_args))
    pub 1: args_ext.excess_space[1] =,
    if (size == 0)
    pub __clone_args): size = sizeof(struct,
    switch (test_mode) {
    case CLONE3_ARGS_NO_TEST:
//
// Uses default 'flags' and 'SIGCHLD'
// assignment.
//
    case CLONE3_ARGS_ALL_0:
    pub 0: args.flags =,
    pub 0: args.exit_signal =,
    case CLONE3_ARGS_INVAL_EXIT_SIGNAL_BIG:
    pub 0xbadc0ded00000000ULL: args.exit_signal =,
    case CLONE3_ARGS_INVAL_EXIT_SIGNAL_NEG:
    pub 0x0000000080000000ULL: args.exit_signal =,
    case CLONE3_ARGS_INVAL_EXIT_SIGNAL_CSIG:
    pub 0x0000000000000100ULL: args.exit_signal =,
    case CLONE3_ARGS_INVAL_EXIT_SIGNAL_NSIG:
    pub 0x00000000000000f0ULL: args.exit_signal =,
    }
    pub __clone_args)): memcpy(&args_ext.args, &args, sizeof(struct,
    pub size): *mut *mut pid = sys_clone3((struct __clone_args )&args_ext,,
    if (pid < 0) {
    ksft_print_msg("%s - Failed to create new process\n",
    pub -errno: return,
    }
    if (pid == 0) {
    pub getpid()): ksft_print_msg("I am the child, my PID is %d\n",,
    }
    ksft_print_msg("I am the parent (%d). My child's pid is %d\n",
    pub pid): getpid(),,
    if (waitpid(-1, &status, __WALL) < 0) {
    pub strerror(errno)): ksft_print_msg("waitpid() returned %s\n",,
    pub -errno: return,
    }
    if (!WIFEXITED(status)) {
    ksft_print_msg("Child did not exit normally, status 0x%x\n",
    pub EXIT_FAILURE: return,
    }
    if (WEXITSTATUS(status))
    pub WEXITSTATUS(status): return,
    pub 0: return,
    }
    static bool test_clone3(uint64_t flags, size_t size, int expected,
    enum test_mode test_mode)
    {
    pub ret: c_int,
    ksft_print_msg(
    "[%d] Trying clone3() with flags %#" PRIx64 " (size %zu)\n",
    pub size): getpid(), flags,,
    pub test_mode): ret = call_clone3(flags, size,,
    ksft_print_msg("[%d] clone3() with flags says: %d expected %d\n",
    pub expected): getpid(), ret,,
    if (ret != expected) {
    ksft_print_msg(
    "[%d] Result (%d) is different than expected (%d)\n",
    pub expected): getpid(), ret,,
    pub false: return,
    }
    pub true: return,
    }
    pub (*filter_function)(void): *mut typedef bool,
    pub (*size_function)(void): *mut typedef size_t,
#[no_mangle]
unsafe extern "C" fn not_root() -> bool {
    static bool not_root(void)
    {
    if (getuid() != 0) {
    pub root\n"): ksft_print_msg("Not running as,
    pub true: return,
    }
    pub false: return,
    }
#[no_mangle]
unsafe extern "C" fn no_timenamespace() -> bool {
    static bool no_timenamespace(void)
    {
    if (not_root())
    pub true: return,
    if (!access("/proc/self/ns/time", F_OK))
    pub false: return,
    pub supported\n"): ksft_print_msg("Time namespaces are not,
    pub true: return,
    }
#[no_mangle]
unsafe extern "C" fn page_size_plus_8() -> usize {
    static size_t page_size_plus_8(void)
    {
    pub 8: return getpagesize() +,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test {
    pub name: *const c_char,
    pub flags: u64,
    pub size: usize,
    pub size_function: size_function,
    pub expected: c_int,
    pub test_mode: enum test_mode,
    pub filter: filter_function,
}

    static const struct test tests[] = {
    {
    .name = "simple clone3()",
    .flags = 0,
    .size = 0,
    .expected = 0,
    .test_mode = CLONE3_ARGS_NO_TEST,
    },
    {
    .name = "clone3() in a new PID_NS",
    .flags = CLONE_NEWPID,
    .size = 0,
    .expected = 0,
    .test_mode = CLONE3_ARGS_NO_TEST,
    .filter = not_root,
    },
    {
    .name = "CLONE_ARGS_SIZE_VER0",
    .flags = 0,
    .size = CLONE_ARGS_SIZE_VER0,
    .expected = 0,
    .test_mode = CLONE3_ARGS_NO_TEST,
    },
    {
    .name = "CLONE_ARGS_SIZE_VER0 - 8",
    .flags = 0,
    .size = CLONE_ARGS_SIZE_VER0 - 8,
    .expected = -EINVAL,
    .test_mode = CLONE3_ARGS_NO_TEST,
    },
    {
    .name = "sizeof(struct clone_args) + 8",
    .flags = 0,
    .size = sizeof(struct __clone_args) + 8,
    .expected = 0,
    .test_mode = CLONE3_ARGS_NO_TEST,
    },
    {
    .name = "exit_signal with highest 32 bits non-zero",
    .flags = 0,
    .size = 0,
    .expected = -EINVAL,
    .test_mode = CLONE3_ARGS_INVAL_EXIT_SIGNAL_BIG,
    },
    {
    .name = "negative 32-bit exit_signal",
    .flags = 0,
    .size = 0,
    .expected = -EINVAL,
    .test_mode = CLONE3_ARGS_INVAL_EXIT_SIGNAL_NEG,
    },
    {
    .name = "exit_signal not fitting into CSIGNAL mask",
    .flags = 0,
    .size = 0,
    .expected = -EINVAL,
    .test_mode = CLONE3_ARGS_INVAL_EXIT_SIGNAL_CSIG,
    },
    {
    .name = "NSIG < exit_signal < CSIG",
    .flags = 0,
    .size = 0,
    .expected = -EINVAL,
    .test_mode = CLONE3_ARGS_INVAL_EXIT_SIGNAL_NSIG,
    },
    {
    .name = "Arguments sizeof(struct clone_args) + 8",
    .flags = 0,
    .size = sizeof(struct __clone_args) + 8,
    .expected = 0,
    .test_mode = CLONE3_ARGS_ALL_0,
    },
    {
    .name = "Arguments sizeof(struct clone_args) + 16",
    .flags = 0,
    .size = sizeof(struct __clone_args) + 16,
    .expected = -E2BIG,
    .test_mode = CLONE3_ARGS_ALL_0,
    },
    {
    .name = "Arguments sizeof(struct clone_arg) * 2",
    .flags = 0,
    .size = sizeof(struct __clone_args) + 16,
    .expected = -E2BIG,
    .test_mode = CLONE3_ARGS_ALL_0,
    },
    {
    .name = "Arguments > page size",
    .flags = 0,
    .size_function = page_size_plus_8,
    .expected = -E2BIG,
    .test_mode = CLONE3_ARGS_NO_TEST,
    },
    {
    .name = "CLONE_ARGS_SIZE_VER0 in a new PID NS",
    .flags = CLONE_NEWPID,
    .size = CLONE_ARGS_SIZE_VER0,
    .expected = 0,
    .test_mode = CLONE3_ARGS_NO_TEST,
    .filter = not_root,
    },
    {
    .name = "CLONE_ARGS_SIZE_VER0 - 8 in a new PID NS",
    .flags = CLONE_NEWPID,
    .size = CLONE_ARGS_SIZE_VER0 - 8,
    .expected = -EINVAL,
    .test_mode = CLONE3_ARGS_NO_TEST,
    },
    {
    .name = "sizeof(struct clone_args) + 8 in a new PID NS",
    .flags = CLONE_NEWPID,
    .size = sizeof(struct __clone_args) + 8,
    .expected = 0,
    .test_mode = CLONE3_ARGS_NO_TEST,
    .filter = not_root,
    },
    {
    .name = "Arguments > page size in a new PID NS",
    .flags = CLONE_NEWPID,
    .size_function = page_size_plus_8,
    .expected = -E2BIG,
    .test_mode = CLONE3_ARGS_NO_TEST,
    },
    {
    .name = "New time NS",
    .flags = CLONE_NEWTIME,
    .size = 0,
    .expected = 0,
    .test_mode = CLONE3_ARGS_NO_TEST,
    .filter = no_timenamespace,
    },
    {
    .name = "exit signal (SIGCHLD) in flags",
    .flags = SIGCHLD,
    .size = 0,
    .expected = -EINVAL,
    .test_mode = CLONE3_ARGS_NO_TEST,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    size_t size;
    int i;
    ksft_print_header();
    ksft_set_plan(ARRAY_SIZE(tests));
    test_clone3_supported();
    for (i = 0; i < ARRAY_SIZE(tests); i++) {
    if (tests[i].filter && tests[i].filter()) {
    ksft_test_result_skip("%s\n", tests[i].name);
    continue;
    }
    if (tests[i].size_function)
    size = tests[i].size_function();
    else
    size = tests[i].size;
    ksft_print_msg("Running test '%s'\n", tests[i].name);
    ksft_test_result(test_clone3(tests[i].flags, size,
    tests[i].expected,
    tests[i].test_mode),
    "%s\n", tests[i].name);
    }
    ksft_finished();
    }

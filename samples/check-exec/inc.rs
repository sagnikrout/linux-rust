//! Automatically rewritten from C to Rust
//! Source: samples/check-exec/inc.c
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Very simple script interpreter that can evaluate two different commands (one
// per line):
// - "?" to initialize a counter from user's input;
// - "+" to increment the counter (which is set to 0 by default).
//
// See tools/testing/selftests/exec/check-exec-tests.sh and
// Documentation/userspace-api/check_exec.rst
//
// Copyright © 2024 Microsoft Corporation
//
// Macro flag: #define _GNU_SOURCE

    static int sys_execveat(int dirfd, const char *pathname, char *const argv[],
    char *const envp[], int flags)
    {
    return syscall(__NR_execveat, dirfd, pathname, argv, envp, flags);
    }
// Returns 1 on error, 0 otherwise.
#[no_mangle]
unsafe extern "C" fn interpret_buffer(buffer: *mut c_char, buffer_size: usize) -> c_int {
    static int interpret_buffer(char *buffer, size_t buffer_size)
    {
    char *line, *saveptr = core::ptr::null_mut();
    let mut number: c_longlong = 0;
// Each command is the first character of a line.
    saveptr = core::ptr::null_mut();
    line = strtok_r(buffer, "\n", &saveptr);
    while (line) {
    if (*line != '#' && strlen(line) != 1) {
    fprintf(stderr, "# ERROR: Unknown string\n");
    return 1;
    }
    switch (*line) {
    case '#':
// Skips shebang and comments.
    break;
    case '+':
// Increments and prints the number.
    number++;
    printf("%lld\n", number);
    break;
    case '?':
// Reads integer from stdin.
    fprintf(stderr, "> Enter new number: \n");
    if (scanf("%lld", &number) != 1) {
    fprintf(stderr,
    "# WARNING: Failed to read number from stdin\n");
    }
    break;
    default:
    fprintf(stderr, "# ERROR: Unknown character '%c'\n",
// line);
    return 1;
    }
    line = strtok_r(core::ptr::null_mut(), "\n", &saveptr);
    }
    return 0;
    }
// Returns 1 on error, 0 otherwise.
    static int interpret_stream(FILE *script, char *const script_name,
    char *const *const envp, const bool restrict_stream)
    {
    int err;
    char *const script_argv[] = { script_name, core::ptr::null_mut() };
    char buf[128] = {};
    let mut buf_size: usize = sizeof(buf);
//
// We pass a valid argv and envp to the kernel to emulate a native
// script execution.  We must use the script file descriptor instead of
// the script path name to avoid race conditions.
//
    err = sys_execveat(fileno(script), "", script_argv, envp,
    AT_EMPTY_PATH | AT_EXECVE_CHECK);
    if (err && restrict_stream) {
    perror("ERROR: Script execution check");
    return 1;
    }
// Reads script.
    buf_size = fread(buf, 1, buf_size - 1, script);
    return interpret_buffer(buf, buf_size);
    }
#[no_mangle]
unsafe extern "C" fn print_usage(argv0: *const c_char) {
    static void print_usage(const char *argv0)
    {
    fprintf(stderr, "usage: %s <script.inc> | -i | -c <command>\n\n",
    argv0);
    fprintf(stderr, "Example:\n");
    fprintf(stderr, "  ./set-exec -fi -- ./inc -i < script-exec.inc\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *const *const c_char, envp: *const *const *const c_char) -> c_int {
    int main(const int argc, char *const argv[], char *const *const envp)
    {
    int opt;
    char *cmd = core::ptr::null_mut();
    char *script_name = core::ptr::null_mut();
    let mut interpret_stdin: bool = false;
    FILE *script_file = core::ptr::null_mut();
    int secbits;
    bool deny_interactive, restrict_file;
    size_t arg_nb;
    secbits = prctl(PR_GET_SECUREBITS);
    if (secbits == -1) {
//
// This should never happen, except with a buggy seccomp
// filter.
//
    perror("ERROR: Failed to get securebits");
    return 1;
    }
    deny_interactive = !!(secbits & SECBIT_EXEC_DENY_INTERACTIVE);
    restrict_file = !!(secbits & SECBIT_EXEC_RESTRICT_FILE);
    while ((opt = getopt(argc, argv, "c:i")) != -1) {
    switch (opt) {
    case 'c':
    if (cmd) {
    fprintf(stderr, "ERROR: Command already set");
    return 1;
    }
    cmd = optarg;
    break;
    case 'i':
    interpret_stdin = true;
    break;
    default:
    print_usage(argv[0]);
    return 1;
    }
    }
// Checks that only one argument is used, or read stdin.
    arg_nb = !!cmd + !!interpret_stdin;
    if (arg_nb == 0 && argc == 2) {
    script_name = argv[1];
    } else if (arg_nb != 1) {
    print_usage(argv[0]);
    return 1;
    }
    if (cmd) {
//
// Other kind of interactive interpretations should be denied
// as well (e.g. CLI arguments passing script snippets,
// environment variables interpreted as script).  However, any
// way to pass script files should only be restricted according
// to restrict_file.
//
    if (deny_interactive) {
    fprintf(stderr,
    "ERROR: Interactive interpretation denied.\n");
    return 1;
    }
    return interpret_buffer(cmd, strlen(cmd));
    }
    if (interpret_stdin && !script_name) {
    script_file = stdin;
//
// As for any execve(2) call, this path may be logged by the
// kernel.
//
    script_name = "/proc/self/fd/0";
//
// When stdin is used, it can point to a regular file or a
// pipe.  Restrict stdin execution according to
// SECBIT_EXEC_DENY_INTERACTIVE but always allow executable
// files (which are not considered as interactive inputs).
//
    return interpret_stream(script_file, script_name, envp,
    deny_interactive);
    } else if (script_name && !interpret_stdin) {
//
// In this sample, we don't pass any argument to scripts, but
// otherwise we would have to forge an argv with such
// arguments.
//
    script_file = fopen(script_name, "r");
    if (!script_file) {
    perror("ERROR: Failed to open script");
    return 1;
    }
//
// Restricts file execution according to
// SECBIT_EXEC_RESTRICT_FILE.
//
    return interpret_stream(script_file, script_name, envp,
    restrict_file);
    }
    print_usage(argv[0]);
    return 1;
    }

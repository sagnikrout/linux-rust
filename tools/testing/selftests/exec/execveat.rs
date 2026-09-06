//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/exec/execveat.c
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
// Copyright (c) 2014 Google, Inc.
//
// Selftests for execveat(2).
//

pub const TESTS_EXPECTED: c_int = 54;

    static char longpath[2 * PATH_MAX] = "";
    static char *envp[] = { "IN_TEST=yes", core::ptr::null_mut(), core::ptr::null_mut() };
    static char *argv[] = { "execveat", "99", core::ptr::null_mut() };
    static int execveat_(int fd, const char *path, char **argv, char **envp,
    int flags)
    {

    return syscall(__NR_execveat, fd, path, argv, envp, flags);

    errno = ENOSYS;
    return -1;

    }

    _check_execveat_fail(fd, path, flags, errno, #errno)
    static int _check_execveat_fail(int fd, const char *path, int flags,
    int expected_errno, const char *errno_str)
    {
    char test_name[TEST_NAME_LEN];
    int rc;
    errno = 0;
    snprintf(test_name, sizeof(test_name),
    "Check failure of execveat(%d, '%s', %d) with %s",
    fd, path?:"(null)", flags, errno_str);
    rc = execveat_(fd, path, argv, envp, flags);
    if (rc > 0) {
    ksft_print_msg("unexpected success from execveat(2)\n");
    ksft_test_result_fail("%s\n", test_name);
    return 1;
    }
    if (errno != expected_errno) {
    ksft_print_msg("expected errno %d (%s) not %d (%s)\n",
    expected_errno, strerror(expected_errno),
    errno, strerror(errno));
    ksft_test_result_fail("%s\n", test_name);
    return 1;
    }
    ksft_test_result_pass("%s\n", test_name);
    return 0;
    }
    static int check_execveat_invoked_rc(int fd, const char *path, int flags,
    int expected_rc, int expected_rc2)
    {
    char test_name[TEST_NAME_LEN];
    int status;
    int rc;
    pid_t child;
    let mut pathlen: c_int = path ? strlen(path) : 0;
    if (pathlen > 40)
    snprintf(test_name, sizeof(test_name),
    "Check success of execveat(%d, '%.20s...%s', %d)... ",
    fd, path, (path + pathlen - 20), flags);
    else
    snprintf(test_name, sizeof(test_name),
    "Check success of execveat(%d, '%s', %d)... ",
    fd, path?:"(null)", flags);
    child = fork();
    if (child < 0) {
    ksft_perror("fork() failed");
    ksft_test_result_fail("%s\n", test_name);
    return 1;
    }
    if (child == 0) {
// Child: do execveat().
    rc = execveat_(fd, path, argv, envp, flags);
    ksft_print_msg("child execveat() failed, rc=%d errno=%d (%s)\n",
    rc, errno, strerror(errno));
    exit(errno);
    }
// Parent: wait for & check child's exit status.
    rc = waitpid(child, &status, 0);
    if (rc != child) {
    ksft_print_msg("waitpid(%d,...) returned %d\n", child, rc);
    ksft_test_result_fail("%s\n", test_name);
    return 1;
    }
    if (!WIFEXITED(status)) {
    ksft_print_msg("child %d did not exit cleanly, status=%08x\n",
    child, status);
    ksft_test_result_fail("%s\n", test_name);
    return 1;
    }
    if ((WEXITSTATUS(status) != expected_rc) &&
    (WEXITSTATUS(status) != expected_rc2)) {
    ksft_print_msg("child %d exited with %d neither %d nor %d\n",
    child, WEXITSTATUS(status), expected_rc,
    expected_rc2);
    ksft_test_result_fail("%s\n", test_name);
    return 1;
    }
    ksft_test_result_pass("%s\n", test_name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_execveat(fd: c_int, path: *const c_char, flags: c_int) -> c_int {
    static int check_execveat(int fd, const char *path, int flags)
    {
    return check_execveat_invoked_rc(fd, path, flags, 99, 99);
    }
    static char *concat(const char *left, const char *right)
    {
    char *result = malloc(strlen(left) + strlen(right) + 1);
    strcpy(result, left);
    strcat(result, right);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn open_or_die(filename: *const c_char, flags: c_int) -> c_int {
    static int open_or_die(const char *filename, int flags)
    {
    let mut fd: c_int = open(filename, flags);
    if (fd < 0)
    ksft_exit_fail_msg("Failed to open '%s'; "
    "check prerequisites are available\n", filename);
    return fd;
    }
#[no_mangle]
unsafe extern "C" fn exe_cp(src: *const c_char, dest: *const c_char) {
    static void exe_cp(const char *src, const char *dest)
    {
    let mut in_fd: c_int = open_or_die(src, O_RDONLY);
    let mut out_fd: c_int = open(dest, O_RDWR|O_CREAT|O_TRUNC, 0755);
    struct stat info;
    fstat(in_fd, &info);
    sendfile(out_fd, in_fd, core::ptr::null_mut(), info.st_size);
    close(in_fd);
    close(out_fd);
    }
pub const XX_DIR_LEN: c_int = 200;
#[no_mangle]
unsafe extern "C" fn check_execveat_pathmax(root_dfd: c_int, src: *const c_char, is_script: c_int) -> c_int {
    static int check_execveat_pathmax(int root_dfd, const char *src, int is_script)
    {
    let mut fail: c_int = 0;
    int ii, count, len;
    char longname[XX_DIR_LEN + 1];
    int fd;
    if (*longpath == '\0') {
// Create a filename close to PATH_MAX in length
    char *cwd = getcwd(core::ptr::null_mut(), 0);
    if (!cwd) {
    ksft_perror("Failed to getcwd()");
    return 2;
    }
    strcpy(longpath, cwd);
    strcat(longpath, "/");
    memset(longname, 'x', XX_DIR_LEN - 1);
    longname[XX_DIR_LEN - 1] = '/';
    longname[XX_DIR_LEN] = '\0';
    count = (PATH_MAX - 3 - strlen(cwd)) / XX_DIR_LEN;
    for (ii = 0; ii < count; ii++) {
    strcat(longpath, longname);
    mkdir(longpath, 0755);
    }
    len = (PATH_MAX - 3 - strlen(cwd)) - (count * XX_DIR_LEN);
    if (len <= 0)
    len = 1;
    memset(longname, 'y', len);
    longname[len] = '\0';
    strcat(longpath, longname);
    free(cwd);
    }
    exe_cp(src, longpath);
//
// Execute as a pre-opened file descriptor, which works whether this is
// a script or not (because the interpreter sees a filename like
// "/dev/fd/20").
//
    fd = open(longpath, O_RDONLY);
    if (fd > 0) {
    ksft_print_msg("Invoke copy of '%s' via filename of length %zu:\n",
    src, strlen(longpath));
    fail += check_execveat(fd, "", AT_EMPTY_PATH);
    } else {
    ksft_print_msg("Failed to open length %zu filename, errno=%d (%s)\n",
    strlen(longpath), errno, strerror(errno));
    fail++;
    }
//
// Execute as a long pathname relative to "/".  If this is a script,
// the interpreter will launch but fail to open the script because its
// name ("/dev/fd/5/xxx....") is bigger than PATH_MAX.
//
// The failure code is usually 127 (POSIX: "If a command is not found,
// the exit status shall be 127."), but some systems give 126 (POSIX:
// "If the command name is found, but it is not an executable utility,
// the exit status shall be 126."), so allow either.
//
    if (is_script) {
    ksft_print_msg("Invoke script via root_dfd and relative filename\n");
    fail += check_execveat_invoked_rc(root_dfd, longpath + 1, 0,
    127, 126);
    } else {
    ksft_print_msg("Invoke exec via root_dfd and relative filename\n");
    fail += check_execveat(root_dfd, longpath + 1, 0);
    }
    return fail;
    }
#[no_mangle]
unsafe extern "C" fn check_execveat_comm(fd: c_int, argv0: *mut c_char, expected: *mut c_char) -> c_int {
    static int check_execveat_comm(int fd, char *argv0, char *expected)
    {
    char buf[128], *old_env, *old_argv0;
    int ret;
    snprintf(buf, sizeof(buf), CHECK_COMM "=%s", expected);
    old_env = envp[1];
    envp[1] = buf;
    old_argv0 = argv[0];
    argv[0] = argv0;
    ksft_print_msg("Check execveat(AT_EMPTY_PATH)'s comm is %s\n",
    expected);
    ret = check_execveat_invoked_rc(fd, "", AT_EMPTY_PATH, 0, 0);
    envp[1] = old_env;
    argv[0] = old_argv0;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn run_tests() -> c_int {
    static int run_tests(void)
    {
    let mut fail: c_int = 0;
    char *fullname = realpath("execveat", core::ptr::null_mut());
    char *fullname_script = realpath("script", core::ptr::null_mut());
    char *fullname_symlink = concat(fullname, ".symlink");
    let mut subdir_dfd: c_int = open_or_die("subdir", O_DIRECTORY|O_RDONLY);
    int subdir_dfd_ephemeral = open_or_die("subdir.ephemeral",
    O_DIRECTORY|O_RDONLY);
    let mut dot_dfd: c_int = open_or_die(".", O_DIRECTORY|O_RDONLY);
    let mut root_dfd: c_int = open_or_die("/", O_DIRECTORY|O_RDONLY);
    let mut dot_dfd_path: c_int = open_or_die(".", O_DIRECTORY|O_RDONLY|O_PATH);
    let mut dot_dfd_cloexec: c_int = open_or_die(".", O_DIRECTORY|O_RDONLY|O_CLOEXEC);
    let mut fd: c_int = open_or_die("execveat", O_RDONLY);
    let mut fd_path: c_int = open_or_die("execveat", O_RDONLY|O_PATH);
    let mut fd_symlink: c_int = open_or_die("execveat.symlink", O_RDONLY);
    let mut fd_denatured: c_int = open_or_die("execveat.denatured", O_RDONLY);
    int fd_denatured_path = open_or_die("execveat.denatured",
    O_RDONLY|O_PATH);
    let mut fd_script: c_int = open_or_die("script", O_RDONLY);
    let mut fd_ephemeral: c_int = open_or_die("execveat.ephemeral", O_RDONLY);
    int fd_ephemeral_path = open_or_die("execveat.path.ephemeral",
    O_RDONLY|O_PATH);
    let mut fd_script_ephemeral: c_int = open_or_die("script.ephemeral", O_RDONLY);
    let mut fd_cloexec: c_int = open_or_die("execveat", O_RDONLY|O_CLOEXEC);
    let mut fd_script_cloexec: c_int = open_or_die("script", O_RDONLY|O_CLOEXEC);
// Check if we have execveat at all, and bail early if not
    errno = 0;
    execveat_(-1, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), 0);
    if (errno == ENOSYS) {
    ksft_exit_skip(
    "ENOSYS calling execveat - no kernel support?\n");
    }
// Change file position to confirm it doesn't affect anything
    lseek(fd, 10, SEEK_SET);
// Normal executable file:
// dfd + path
    fail += check_execveat(subdir_dfd, "../execveat", 0);
    fail += check_execveat(dot_dfd, "execveat", 0);
    fail += check_execveat(dot_dfd_path, "execveat", 0);
// absolute path
    fail += check_execveat(AT_FDCWD, fullname, 0);
// absolute path with nonsense dfd
    fail += check_execveat(99, fullname, 0);
// fd + no path
    fail += check_execveat(fd, "", AT_EMPTY_PATH);
// O_CLOEXEC fd + no path
    fail += check_execveat(fd_cloexec, "", AT_EMPTY_PATH);
// O_PATH fd
    fail += check_execveat(fd_path, "", AT_EMPTY_PATH);
// Mess with executable file that's already open:
// fd + no path to a file that's been renamed
    rename("execveat.ephemeral", "execveat.moved");
    fail += check_execveat(fd_ephemeral, "", AT_EMPTY_PATH);
// fd + no path to a file that's been deleted
    unlink("execveat.moved"); /* remove the file now fd open */
    fail += check_execveat(fd_ephemeral, "", AT_EMPTY_PATH);
// Mess with executable file that's already open with O_PATH
// fd + no path to a file that's been deleted
    unlink("execveat.path.ephemeral");
    fail += check_execveat(fd_ephemeral_path, "", AT_EMPTY_PATH);
// Invalid argument failures
    fail += check_execveat_fail(fd, "", 0, ENOENT);
    fail += check_execveat_fail(fd, core::ptr::null_mut(), AT_EMPTY_PATH, EFAULT);
// Symlink to executable file:
// dfd + path
    fail += check_execveat(dot_dfd, "execveat.symlink", 0);
    fail += check_execveat(dot_dfd_path, "execveat.symlink", 0);
// absolute path
    fail += check_execveat(AT_FDCWD, fullname_symlink, 0);
// fd + no path, even with AT_SYMLINK_NOFOLLOW (already followed)
    fail += check_execveat(fd_symlink, "", AT_EMPTY_PATH);
    fail += check_execveat(fd_symlink, "",
    AT_EMPTY_PATH|AT_SYMLINK_NOFOLLOW);
// Symlink fails when AT_SYMLINK_NOFOLLOW set:
// dfd + path
    fail += check_execveat_fail(dot_dfd, "execveat.symlink",
    AT_SYMLINK_NOFOLLOW, ELOOP);
    fail += check_execveat_fail(dot_dfd_path, "execveat.symlink",
    AT_SYMLINK_NOFOLLOW, ELOOP);
// absolute path
    fail += check_execveat_fail(AT_FDCWD, fullname_symlink,
    AT_SYMLINK_NOFOLLOW, ELOOP);
// Non-regular file failure
    fail += check_execveat_fail(dot_dfd, "pipe", 0, EACCES);
    unlink("pipe");
// Shell script wrapping executable file:
// dfd + path
    fail += check_execveat(subdir_dfd, "../script", 0);
    fail += check_execveat(dot_dfd, "script", 0);
    fail += check_execveat(dot_dfd_path, "script", 0);
// absolute path
    fail += check_execveat(AT_FDCWD, fullname_script, 0);
// fd + no path
    fail += check_execveat(fd_script, "", AT_EMPTY_PATH);
    fail += check_execveat(fd_script, "",
    AT_EMPTY_PATH|AT_SYMLINK_NOFOLLOW);
// O_CLOEXEC fd fails for a script (as script file inaccessible)
    fail += check_execveat_fail(fd_script_cloexec, "", AT_EMPTY_PATH,
    ENOENT);
    fail += check_execveat_fail(dot_dfd_cloexec, "script", 0, ENOENT);
// Mess with script file that's already open:
// fd + no path to a file that's been renamed
    rename("script.ephemeral", "script.moved");
    fail += check_execveat(fd_script_ephemeral, "", AT_EMPTY_PATH);
// fd + no path to a file that's been deleted
    unlink("script.moved"); /* remove the file while fd open */
    fail += check_execveat(fd_script_ephemeral, "", AT_EMPTY_PATH);
// Rename a subdirectory in the path:
    rename("subdir.ephemeral", "subdir.moved");
    fail += check_execveat(subdir_dfd_ephemeral, "../script", 0);
    fail += check_execveat(subdir_dfd_ephemeral, "script", 0);
// Remove the subdir and its contents
    unlink("subdir.moved/script");
    unlink("subdir.moved");
// Shell loads via deleted subdir OK because name starts with ..
    fail += check_execveat(subdir_dfd_ephemeral, "../script", 0);
    fail += check_execveat_fail(subdir_dfd_ephemeral, "script", 0, ENOENT);
// Flag values other than AT_SYMLINK_NOFOLLOW => EINVAL
    fail += check_execveat_fail(dot_dfd, "execveat", 0xFFFF, EINVAL);
// Invalid path => ENOENT
    fail += check_execveat_fail(dot_dfd, "no-such-file", 0, ENOENT);
    fail += check_execveat_fail(dot_dfd_path, "no-such-file", 0, ENOENT);
    fail += check_execveat_fail(AT_FDCWD, "no-such-file", 0, ENOENT);
// Attempt to execute directory => EACCES
    fail += check_execveat_fail(dot_dfd, "", AT_EMPTY_PATH, EACCES);
// Attempt to execute non-executable => EACCES
    fail += check_execveat_fail(dot_dfd, "Makefile", 0, EACCES);
    fail += check_execveat_fail(fd_denatured, "", AT_EMPTY_PATH, EACCES);
    fail += check_execveat_fail(fd_denatured_path, "", AT_EMPTY_PATH,
    EACCES);
// Attempt to execute nonsense FD => EBADF
    fail += check_execveat_fail(99, "", AT_EMPTY_PATH, EBADF);
    fail += check_execveat_fail(99, "execveat", 0, EBADF);
// Attempt to execute relative to non-directory => ENOTDIR
    fail += check_execveat_fail(fd, "execveat", 0, ENOTDIR);
    fail += check_execveat_pathmax(root_dfd, "execveat", 0);
    fail += check_execveat_pathmax(root_dfd, "script", 1);
// /proc/pid/comm gives filename by default
    fail += check_execveat_comm(fd, "sentinel", "execveat");
// /proc/pid/comm gives argv[0] when invoked via link
    fail += check_execveat_comm(fd_symlink, "sentinel", "execveat");
// /proc/pid/comm gives filename if NULL is passed
    fail += check_execveat_comm(fd, core::ptr::null_mut(), "execveat");
    return fail;
    }
#[no_mangle]
unsafe extern "C" fn prerequisites() {
    static void prerequisites(void)
    {
    int fd;
    const char *script = "#!/bin/bash\nexit $*\n";
// Create ephemeral copies of files
    exe_cp("execveat", "execveat.ephemeral");
    exe_cp("execveat", "execveat.path.ephemeral");
    exe_cp("script", "script.ephemeral");
    mkdir("subdir.ephemeral", 0755);
    fd = open("subdir.ephemeral/script", O_RDWR|O_CREAT|O_TRUNC, 0755);
    write(fd, script, strlen(script));
    close(fd);
    mkfifo("pipe", 0755);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int ii;
    int rc;
    const char *verbose = getenv("VERBOSE");
    const char *check_comm = getenv(CHECK_COMM);
    if (argc >= 2 || check_comm) {
//
// If we are invoked with an argument, or no arguments but a
// command to check, don't run tests.
//
    const char *in_test = getenv("IN_TEST");
    if (verbose) {
    ksft_print_msg("invoked with:\n");
    for (ii = 0; ii < argc; ii++)
    ksft_print_msg("\t[%d]='%s\n'", ii, argv[ii]);
    }
// If the tests wanted us to check the command, do so.
    if (check_comm) {
// TASK_COMM_LEN == 16
    char buf[32];
    int fd, ret;
    fd = open("/proc/self/comm", O_RDONLY);
    if (fd < 0) {
    ksft_perror("open() comm failed");
    exit(1);
    }
    ret = read(fd, buf, sizeof(buf));
    if (ret < 0) {
    ksft_perror("read() comm failed");
    close(fd);
    exit(1);
    }
    close(fd);
// trim off the \n
    buf[ret-1] = 0;
    if (strcmp(buf, check_comm)) {
    ksft_print_msg("bad comm, got: %s expected: %s\n",
    buf, check_comm);
    exit(1);
    }
    exit(0);
    }
// Check expected environment transferred.
    if (!in_test || strcmp(in_test, "yes") != 0) {
    ksft_print_msg("no IN_TEST=yes in env\n");
    return 1;
    }
// Use the final argument as an exit code.
    rc = atoi(argv[argc - 1]);
    exit(rc);
    } else {
    ksft_print_header();
    ksft_set_plan(TESTS_EXPECTED);
    prerequisites();
    if (verbose)
    envp[1] = "VERBOSE=1";
    rc = run_tests();
    if (rc > 0)
    printf("%d tests failed\n", rc);
    ksft_finished();
    }
    return rc;
    }

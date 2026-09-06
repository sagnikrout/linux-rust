//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/tty/tty_tiocsti_test.c
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
//
// TTY Tests - TIOCSTI
//
// Copyright © 2025 Abhinav Saxena <xandfury@gmail.com>
//

    enum test_type {
    TEST_PTY_TIOCSTI_BASIC,
    TEST_PTY_TIOCSTI_FD_PASSING,
// other tests cases such as serial may be added.
    };
//
// Test Strategy:
// - Basic tests: Use PTY with/without TIOCSCTTY (controlling terminal for
// current process)
// - FD passing tests: Child creates PTY, parent receives FD (demonstrates
// security issue)
//
// SECURITY VULNERABILITY DEMONSTRATION:
// FD passing tests show that TIOCSTI uses CURRENT process credentials, not
// opener credentials. This means privileged processes can be given FDs from
// unprivileged processes and successfully perform TIOCSTI operations that the
// unprivileged process couldn't do directly.
//
// Attack scenario:
// 1. Unprivileged process opens TTY (direct TIOCSTI fails due to lack of
// privileges)
// 2. Unprivileged process passes FD to privileged process via SCM_RIGHTS
// 3. Privileged process can use TIOCSTI on the FD (succeeds due to its
// privileges)
// 4. Result: Effective privilege escalation via file descriptor passing
//
// This matches the kernel logic in tiocsti():
// 1. if (!tty_legacy_tiocsti && !capable(CAP_SYS_ADMIN)) return -EIO;
// 2. if ((current->signal->tty != tty) && !capable(CAP_SYS_ADMIN))
// return -EPERM;
// Note: Both checks use capable() on CURRENT process, not FD opener!
//
// If the file credentials were also checked along with the capable() checks
// then the results for FD pass tests would be consistent with the basic tests.
//
    FIXTURE(tiocsti)
    {
    int pty_master_fd; /* PTY - for basic tests */
    int pty_slave_fd;
    bool has_pty;
    bool initial_cap_sys_admin;
    int original_legacy_tiocsti_setting;
    bool can_modify_sysctl;
    };
    FIXTURE_VARIANT(tiocsti)
    {
    const enum test_type test_type;
    const bool controlling_tty; /* true=current.signal.tty == tty */
    const int legacy_tiocsti; /* 0=restricted, 1=permissive */
    const bool requires_cap; /* true=with CAP_SYS_ADMIN, false=without */
    const int expected_success; /* 0=success, -EIO/-EPERM=specific error */
    };
//
// Tests Controlling Terminal Variants (current->signal->tty == tty)
//
// TIOCSTI Test Matrix:
//
// | legacy_tiocsti | CAP_SYS_ADMIN | Expected Result | Error |
// |----------------|---------------|-----------------|-------|
// | 1 (permissive) | true          | SUCCESS         | -     |
// | 1 (permissive) | false         | SUCCESS         | -     |
// | 0 (restricted) | true          | SUCCESS         | -     |
// | 0 (restricted) | false         | FAILURE         | -EIO  |
//
// clang-format off
    FIXTURE_VARIANT_ADD(tiocsti, basic_pty_permissive_withcap) {
    .test_type = TEST_PTY_TIOCSTI_BASIC,
    .controlling_tty = true,
    .legacy_tiocsti = 1,
    .requires_cap = true,
    .expected_success = 0,
    };
    FIXTURE_VARIANT_ADD(tiocsti, basic_pty_permissive_nocap) {
    .test_type = TEST_PTY_TIOCSTI_BASIC,
    .controlling_tty = true,
    .legacy_tiocsti = 1,
    .requires_cap = false,
    .expected_success = 0,
    };
    FIXTURE_VARIANT_ADD(tiocsti, basic_pty_restricted_withcap) {
    .test_type = TEST_PTY_TIOCSTI_BASIC,
    .controlling_tty = true,
    .legacy_tiocsti = 0,
    .requires_cap = true,
    .expected_success = 0,
    };
    FIXTURE_VARIANT_ADD(tiocsti, basic_pty_restricted_nocap) {
    .test_type = TEST_PTY_TIOCSTI_BASIC,
    .controlling_tty = true,
    .legacy_tiocsti = 0,
    .requires_cap = false,
    .expected_success = -EIO, /* FAILURE: legacy restriction */
    }; /* clang-format on */
//
// Note for FD Passing Test Variants
// Since we're testing the scenario where an unprivileged process pass an FD
// to a privileged one, .requires_cap here means the caps of the child process.
// Not the parent; parent would always be privileged.
//
// clang-format off
    FIXTURE_VARIANT_ADD(tiocsti, fdpass_pty_permissive_withcap) {
    .test_type = TEST_PTY_TIOCSTI_FD_PASSING,
    .controlling_tty = true,
    .legacy_tiocsti = 1,
    .requires_cap = true,
    .expected_success = 0,
    };
    FIXTURE_VARIANT_ADD(tiocsti, fdpass_pty_permissive_nocap) {
    .test_type = TEST_PTY_TIOCSTI_FD_PASSING,
    .controlling_tty = true,
    .legacy_tiocsti = 1,
    .requires_cap = false,
    .expected_success = 0,
    };
    FIXTURE_VARIANT_ADD(tiocsti, fdpass_pty_restricted_withcap) {
    .test_type = TEST_PTY_TIOCSTI_FD_PASSING,
    .controlling_tty = true,
    .legacy_tiocsti = 0,
    .requires_cap = true,
    .expected_success = 0,
    };
    FIXTURE_VARIANT_ADD(tiocsti, fdpass_pty_restricted_nocap) {
    .test_type = TEST_PTY_TIOCSTI_FD_PASSING,
    .controlling_tty = true,
    .legacy_tiocsti = 0,
    .requires_cap = false,
    .expected_success = -EIO,
    }; /* clang-format on */
//
// Non-Controlling Terminal Variants (current->signal->tty != tty)
//
// TIOCSTI Test Matrix:
//
// | legacy_tiocsti | CAP_SYS_ADMIN | Expected Result | Error |
// |----------------|---------------|-----------------|-------|
// | 1 (permissive) | true          | SUCCESS         | -     |
// | 1 (permissive) | false         | FAILURE         | -EPERM|
// | 0 (restricted) | true          | SUCCESS         | -     |
// | 0 (restricted) | false         | FAILURE         | -EIO  |
//
// clang-format off
    FIXTURE_VARIANT_ADD(tiocsti, basic_nopty_permissive_withcap) {
    .test_type = TEST_PTY_TIOCSTI_BASIC,
    .controlling_tty = false,
    .legacy_tiocsti = 1,
    .requires_cap = true,
    .expected_success = 0,
    };
    FIXTURE_VARIANT_ADD(tiocsti, basic_nopty_permissive_nocap) {
    .test_type = TEST_PTY_TIOCSTI_BASIC,
    .controlling_tty = false,
    .legacy_tiocsti = 1,
    .requires_cap = false,
    .expected_success = -EPERM,
    };
    FIXTURE_VARIANT_ADD(tiocsti, basic_nopty_restricted_withcap) {
    .test_type = TEST_PTY_TIOCSTI_BASIC,
    .controlling_tty = false,
    .legacy_tiocsti = 0,
    .requires_cap = true,
    .expected_success = 0,
    };
    FIXTURE_VARIANT_ADD(tiocsti, basic_nopty_restricted_nocap) {
    .test_type = TEST_PTY_TIOCSTI_BASIC,
    .controlling_tty = false,
    .legacy_tiocsti = 0,
    .requires_cap = false,
    .expected_success = -EIO,
    };
    FIXTURE_VARIANT_ADD(tiocsti, fdpass_nopty_permissive_withcap) {
    .test_type = TEST_PTY_TIOCSTI_FD_PASSING,
    .controlling_tty = false,
    .legacy_tiocsti = 1,
    .requires_cap = true,
    .expected_success = 0,
    };
    FIXTURE_VARIANT_ADD(tiocsti, fdpass_nopty_permissive_nocap) {
    .test_type = TEST_PTY_TIOCSTI_FD_PASSING,
    .controlling_tty = false,
    .legacy_tiocsti = 1,
    .requires_cap = false,
    .expected_success = -EPERM,
    };
    FIXTURE_VARIANT_ADD(tiocsti, fdpass_nopty_restricted_withcap) {
    .test_type = TEST_PTY_TIOCSTI_FD_PASSING,
    .controlling_tty = false,
    .legacy_tiocsti = 0,
    .requires_cap = true,
    .expected_success = 0,
    };
    FIXTURE_VARIANT_ADD(tiocsti, fdpass_nopty_restricted_nocap) {
    .test_type = TEST_PTY_TIOCSTI_FD_PASSING,
    .controlling_tty = false,
    .legacy_tiocsti = 0,
    .requires_cap = false,
    .expected_success = -EIO,
    }; /* clang-format on */
// Helper function to send FD via SCM_RIGHTS
#[no_mangle]
unsafe extern "C" fn send_fd_via_socket(socket_fd: c_int, fd_to_send: c_int) -> c_int {
    static int send_fd_via_socket(int socket_fd, int fd_to_send)
    {
    let mut msg: msghdr = { 0 };
    struct cmsghdr *cmsg;
    char cmsg_buf[CMSG_SPACE(sizeof(int))];
    let mut dummy_data: c_char = 'F';
    let mut iov: iovec = { .iov_base = &dummy_data, .iov_len = 1 };
    msg.msg_iov = &iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf;
    msg.msg_controllen = sizeof(cmsg_buf);
    cmsg = CMSG_FIRSTHDR(&msg);
    cmsg.cmsg_level = SOL_SOCKET;
    cmsg.cmsg_type = SCM_RIGHTS;
    cmsg.cmsg_len = CMSG_LEN(sizeof(int));
    memcpy(CMSG_DATA(cmsg), &fd_to_send, sizeof(int));
    return sendmsg(socket_fd, &msg, 0) < 0 ? -1 : 0;
    }
// Helper function to receive FD via SCM_RIGHTS
#[no_mangle]
unsafe extern "C" fn recv_fd_via_socket(socket_fd: c_int) -> c_int {
    static int recv_fd_via_socket(int socket_fd)
    {
    let mut msg: msghdr = { 0 };
    struct cmsghdr *cmsg;
    char cmsg_buf[CMSG_SPACE(sizeof(int))];
    char dummy_data;
    let mut iov: iovec = { .iov_base = &dummy_data, .iov_len = 1 };
    let mut received_fd: c_int = -1;
    msg.msg_iov = &iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf;
    msg.msg_controllen = sizeof(cmsg_buf);
    if (recvmsg(socket_fd, &msg, 0) < 0)
    return -1;
    for (cmsg = CMSG_FIRSTHDR(&msg); cmsg; cmsg = CMSG_NXTHDR(&msg, cmsg)) {
    if (cmsg.cmsg_level == SOL_SOCKET &&
    cmsg.cmsg_type == SCM_RIGHTS) {
    memcpy(&received_fd, CMSG_DATA(cmsg), sizeof(int));
    break;
    }
    }
    return received_fd;
    }
#[no_mangle]
pub unsafe extern "C" fn has_cap_sys_admin() -> bool {
    static inline bool has_cap_sys_admin(void)
    {
    let mut caps: cap_t = cap_get_proc();
    if (!caps)
    return false;
    cap_flag_value_t cap_val;
    bool has_cap = (cap_get_flag(caps, CAP_SYS_ADMIN, CAP_EFFECTIVE,
    &cap_val) == 0) &&
    (cap_val == CAP_SET);
    cap_free(caps);
    return has_cap;
    }
//
// Switch to non-root user and clear all capabilities
//
#[no_mangle]
pub unsafe extern "C" fn drop_all_privs(_metadata: *mut __test_metadata) -> bool {
    static inline bool drop_all_privs(struct __test_metadata *_metadata)
    {
// Drop supplementary groups
    ASSERT_EQ(setgroups(0, core::ptr::null_mut()), 0);
// Switch to non-root user
    ASSERT_EQ(setgid(1000), 0);
    ASSERT_EQ(setuid(1000), 0);
// Clear all capabilities
    let mut empty: cap_t = cap_init();
    ASSERT_NE(empty, core::ptr::null_mut());
    ASSERT_EQ(cap_set_proc(empty), 0);
    cap_free(empty);
// Prevent privilege regain
    ASSERT_EQ(prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0), 0);
// Verify privilege drop
    ASSERT_FALSE(has_cap_sys_admin());
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn get_legacy_tiocsti_setting(_metadata: *mut __test_metadata) -> c_int {
    static inline int get_legacy_tiocsti_setting(struct __test_metadata *_metadata)
    {
    FILE *fp;
    let mut value: c_int = -1;
    fp = fopen("/proc/sys/dev/tty/legacy_tiocsti", "r");
    if (!fp) {
// legacy_tiocsti sysctl not available (kernel < 6.2)
    return -1;
    }
    if (fscanf(fp, "%d", &value) == 1 && fclose(fp) == 0) {
    if (value < 0 || value > 1)
    value = -1; /* Invalid value */
    } else {
    value = -1; /* Failed to parse */
    }
    return value;
    }
    static inline bool set_legacy_tiocsti_setting(struct __test_metadata *_metadata,
    int value)
    {
    FILE *fp;
    let mut success: bool = false;
// Sanity-check the value
    ASSERT_GE(value, 0);
    ASSERT_LE(value, 1);
//
// Try to open for writing; if we lack permission, return false so
// the test harness will skip variants that need to change it
//
    fp = fopen("/proc/sys/dev/tty/legacy_tiocsti", "w");
    if (!fp)
    return false;
// Write the new setting
    if (fprintf(fp, "%d\n", value) > 0 && fclose(fp) == 0)
    success = true;
    else
    TH_LOG("Failed to write legacy_tiocsti: %s", strerror(errno));
    return success;
    }
//
// TIOCSTI injection test function
// @tty_fd: TTY slave file descriptor to test TIOCSTI on
// Returns: 0 on success, -errno on failure
//
    static inline int test_tiocsti_injection(struct __test_metadata *_metadata,
    int tty_fd)
    {
    int ret;
    let mut inject_char: c_char = 'V';
    errno = 0;
    ret = ioctl(tty_fd, TIOCSTI, &inject_char);
    let mut ret: return = = 0 ? 0 : -errno;
    }
//
// Child process: test TIOCSTI directly with capability/controlling
// terminal setup
//
    static void run_basic_tiocsti_test(struct __test_metadata *_metadata,
    FIXTURE_DATA(tiocsti) * self,
#[no_mangle]
pub unsafe extern "C" fn FIXTURE_VARIANT(variant: *mut *mut tiocsti)) -> const {
    const FIXTURE_VARIANT(tiocsti) * variant)
    {
// Handle capability requirements
    if (self.initial_cap_sys_admin && !variant.requires_cap)
    ASSERT_TRUE(drop_all_privs(_metadata));
    if (variant.controlling_tty) {
//
// Create new session and set PTY as
// controlling terminal
//
    let mut sid: pid_t = setsid();
    ASSERT_GE(sid, 0);
    ASSERT_EQ(ioctl(self.pty_slave_fd, TIOCSCTTY, 0), 0);
    }
//
// Validate test environment setup and verify final
// capability state matches expectation
// after potential drop.
//
    ASSERT_TRUE(self.has_pty);
    ASSERT_EQ(has_cap_sys_admin(), variant.requires_cap);
// Test TIOCSTI and validate result
    let mut result: c_int = test_tiocsti_injection(_metadata, self.pty_slave_fd);
// Check against expected result from variant
    EXPECT_EQ(result, variant.expected_success);
    _exit(0);
    }
//
// Child process: create PTY and then pass FD to parent via SCM_RIGHTS
//
    static void run_fdpass_tiocsti_test(struct __test_metadata *_metadata,
    const FIXTURE_VARIANT(tiocsti) * variant,
    int sockfd)
    {
    signal(SIGHUP, SIG_IGN);
// Handle privilege dropping
    if (!variant.requires_cap && has_cap_sys_admin())
    ASSERT_TRUE(drop_all_privs(_metadata));
// Create child's PTY
    int child_master_fd, child_slave_fd;
    ASSERT_EQ(openpty(&child_master_fd, &child_slave_fd, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()),
    0);
    if (variant.controlling_tty) {
    let mut sid: pid_t = setsid();
    ASSERT_GE(sid, 0);
    ASSERT_EQ(ioctl(child_slave_fd, TIOCSCTTY, 0), 0);
    }
// Test child's direct TIOCSTI for reference
    let mut direct_result: c_int = test_tiocsti_injection(_metadata, child_slave_fd);
    EXPECT_EQ(direct_result, variant.expected_success);
// Send FD to parent
    ASSERT_EQ(send_fd_via_socket(sockfd, child_slave_fd), 0);
// Wait for parent completion signal
    char sync_byte;
    let mut bytes_read: isize = read(sockfd, &sync_byte, 1);
    ASSERT_EQ(bytes_read, 1);
    close(child_master_fd);
    close(child_slave_fd);
    close(sockfd);
    _exit(0);
    }
    FIXTURE_SETUP(tiocsti)
    {
// Create PTY pair for basic tests
    self.has_pty = (openpty(&self.pty_master_fd, &self.pty_slave_fd,
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()) == 0);
    if (!self.has_pty) {
    self.pty_master_fd = -1;
    self.pty_slave_fd = -1;
    }
    self.initial_cap_sys_admin = has_cap_sys_admin();
    self.original_legacy_tiocsti_setting =
    get_legacy_tiocsti_setting(_metadata);
    if (self.original_legacy_tiocsti_setting < 0)
    SKIP(return,
    "legacy_tiocsti sysctl not available (kernel < 6.2)");
// Common skip conditions
    if (variant.test_type == TEST_PTY_TIOCSTI_BASIC && !self.has_pty)
    SKIP(return, "PTY not available for controlling terminal test");
    if (variant.test_type == TEST_PTY_TIOCSTI_FD_PASSING &&
    !self.initial_cap_sys_admin)
    SKIP(return, "FD Pass tests require CAP_SYS_ADMIN");
    if (variant.requires_cap && !self.initial_cap_sys_admin)
    SKIP(return, "Test requires initial CAP_SYS_ADMIN");
// Test if we can modify the sysctl (requires appropriate privileges)
    self.can_modify_sysctl = set_legacy_tiocsti_setting(
    _metadata, self.original_legacy_tiocsti_setting);
// Sysctl setup based on variant
    if (self.can_modify_sysctl &&
    self.original_legacy_tiocsti_setting != variant.legacy_tiocsti) {
    if (!set_legacy_tiocsti_setting(_metadata,
    variant.legacy_tiocsti))
    SKIP(return, "Failed to set legacy_tiocsti sysctl");
    } else if (!self.can_modify_sysctl &&
    self.original_legacy_tiocsti_setting !=
    variant.legacy_tiocsti)
    SKIP(return, "legacy_tiocsti setting mismatch");
    }
    FIXTURE_TEARDOWN(tiocsti)
    {
//
// Backup restoration -
// each test should restore its own sysctl changes
//
    if (self.can_modify_sysctl) {
    let mut current_value: c_int = get_legacy_tiocsti_setting(_metadata);
    if (current_value != self.original_legacy_tiocsti_setting) {
    TH_LOG("Backup: Restoring legacy_tiocsti from %d to %d",
    current_value,
    self.original_legacy_tiocsti_setting);
    set_legacy_tiocsti_setting(
    _metadata,
    self.original_legacy_tiocsti_setting);
    }
    }
    if (self.has_pty) {
    if (self.pty_master_fd >= 0)
    close(self.pty_master_fd);
    if (self.pty_slave_fd >= 0)
    close(self.pty_slave_fd);
    }
    }
    TEST_F(tiocsti, test)
    {
    int status;
    pid_t child_pid;
    if (variant.test_type == TEST_PTY_TIOCSTI_BASIC) {
// ===== BASIC TIOCSTI TEST =====
    child_pid = fork();
    ASSERT_GE(child_pid, 0);
// Perform the actual test in the child process
    if (child_pid == 0)
    run_basic_tiocsti_test(_metadata, self, variant);
    } else {
// ===== FD PASSING SECURITY TEST =====
    int sockpair[2];
    ASSERT_EQ(socketpair(AF_UNIX, SOCK_STREAM, 0, sockpair), 0);
    child_pid = fork();
    ASSERT_GE(child_pid, 0);
    if (child_pid == 0) {
// Child process - create PTY and send FD
    close(sockpair[0]);
    run_fdpass_tiocsti_test(_metadata, variant,
    sockpair[1]);
    }
// Parent process - receive FD and test TIOCSTI
    close(sockpair[1]);
    let mut received_fd: c_int = recv_fd_via_socket(sockpair[0]);
    ASSERT_GE(received_fd, 0);
    let mut parent_has_cap: bool = self.initial_cap_sys_admin;
    TH_LOG("=== TIOCSTI FD Passing Test Context ===");
    TH_LOG("legacy_tiocsti: %d, Parent CAP_SYS_ADMIN: %s, Child: %s",
    variant.legacy_tiocsti, parent_has_cap ? "yes" : "no",
    variant.requires_cap ? "kept" : "dropped");
// SECURITY TEST: Try TIOCSTI with FD opened by child
    let mut result: c_int = test_tiocsti_injection(_metadata, received_fd);
// Log security concern if demonstrated
    if (result == 0 && !variant.requires_cap) {
    TH_LOG("*** SECURITY CONCERN DEMONSTRATED ***");
    TH_LOG("Privileged parent can use TIOCSTI on FD from unprivileged child");
    TH_LOG("This shows current process credentials are used, not opener credentials");
    }
    EXPECT_EQ(result, variant.expected_success)
    {
    TH_LOG("FD passing: expected error %d, got %d",
    variant.expected_success, result);
    }
// Signal child completion
    let mut sync_byte: c_char = 'D';
    let mut bytes_written: isize = write(sockpair[0], &sync_byte, 1);
    ASSERT_EQ(bytes_written, 1);
    close(received_fd);
    close(sockpair[0]);
    }
// Common child process cleanup for both test types
    ASSERT_EQ(waitpid(child_pid, &status, 0), child_pid);
    if (WIFSIGNALED(status)) {
    TH_LOG("Child terminated by signal %d", WTERMSIG(status));
    ASSERT_FALSE(WIFSIGNALED(status))
    {
    TH_LOG("Child process failed assertion");
    }
    } else {
    EXPECT_EQ(WEXITSTATUS(status), 0);
    }
    }
    TEST_HARNESS_MAIN

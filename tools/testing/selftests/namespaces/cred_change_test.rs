//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/namespaces/cred_change_test.c
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
// Macro flag: #define _GNU_SOURCE

//
// Test credential changes and their impact on namespace active references.
//
// Test setuid() in a user namespace properly swaps active references.
// Create a user namespace with multiple UIDs mapped, then setuid() between them.
// Verify that the user namespace remains active throughout.
//
    TEST(setuid_preserves_active_refs)
    {
    pid_t pid;
    int status;
    __u64 userns_id;
    struct ns_id_req req = {
    .size = sizeof(req),
    .spare = 0,
    .ns_id = 0,
    .ns_type = CLONE_NEWUSER,
    .spare2 = 0,
    .user_ns_id = 0,
    };
    __u64 ns_ids[256];
    ssize_t ret;
    int i;
    let mut found: bool = false;
    int pipefd[2];
    ASSERT_EQ(pipe(pipefd), 0);
    pid = fork();
    ASSERT_GE(pid, 0);
    if (pid == 0) {
// Child process
    int fd, userns_fd;
    __u64 child_userns_id;
    let mut orig_uid: uid_t = getuid();
    int setuid_count;
    close(pipefd[0]);
// Create new user namespace with multiple UIDs mapped (0-9)
    userns_fd = get_userns_fd(0, orig_uid, 10);
    if (userns_fd < 0) {
    close(pipefd[1]);
    exit(1);
    }
    if (setns(userns_fd, CLONE_NEWUSER) < 0) {
    close(userns_fd);
    close(pipefd[1]);
    exit(1);
    }
    close(userns_fd);
// Get user namespace ID
    fd = open("/proc/self/ns/user", O_RDONLY);
    if (fd < 0) {
    close(pipefd[1]);
    exit(1);
    }
    if (ioctl(fd, NS_GET_ID, &child_userns_id) < 0) {
    close(fd);
    close(pipefd[1]);
    exit(1);
    }
    close(fd);
// Send namespace ID to parent
    write(pipefd[1], &child_userns_id, sizeof(child_userns_id));
//
// Perform multiple setuid() calls.
// Each setuid() triggers commit_creds() which should properly
// swap active references via switch_cred_namespaces().
//
    for (setuid_count = 0; setuid_count < 50; setuid_count++) {
    let mut target_uid: uid_t = (setuid_count % 10);
    if (setuid(target_uid) < 0) {
    if (errno != EPERM) {
    close(pipefd[1]);
    exit(1);
    }
    }
    }
    close(pipefd[1]);
    exit(0);
    }
// Parent process
    close(pipefd[1]);
    if (read(pipefd[0], &userns_id, sizeof(userns_id)) != sizeof(userns_id)) {
    close(pipefd[0]);
    kill(pid, SIGKILL);
    waitpid(pid, core::ptr::null_mut(), 0);
    SKIP(return, "Failed to get namespace ID from child");
    }
    close(pipefd[0]);
    TH_LOG("Child user namespace ID: %llu", (unsigned long long)userns_id);
// Verify namespace is active while child is running
    ret = sys_listns(&req, ns_ids, ARRAY_SIZE(ns_ids), 0);
    if (ret < 0) {
    kill(pid, SIGKILL);
    waitpid(pid, core::ptr::null_mut(), 0);
    if (errno == ENOSYS)
    SKIP(return, "listns() not supported");
    ASSERT_GE(ret, 0);
    }
    for (i = 0; i < ret; i++) {
    if (ns_ids[i] == userns_id) {
    found = true;
    break;
    }
    }
    ASSERT_TRUE(found);
    waitpid(pid, &status, 0);
    ASSERT_TRUE(WIFEXITED(status));
    ASSERT_EQ(WEXITSTATUS(status), 0);
// Verify namespace becomes inactive after child exits
    ret = sys_listns(&req, ns_ids, ARRAY_SIZE(ns_ids), 0);
    ASSERT_GE(ret, 0);
    found = false;
    for (i = 0; i < ret; i++) {
    if (ns_ids[i] == userns_id) {
    found = true;
    break;
    }
    }
    ASSERT_FALSE(found);
    TH_LOG("setuid() correctly preserved active references (no leak)");
    }
//
// Test setgid() in a user namespace properly handles active references.
//
    TEST(setgid_preserves_active_refs)
    {
    pid_t pid;
    int status;
    __u64 userns_id;
    struct ns_id_req req = {
    .size = sizeof(req),
    .spare = 0,
    .ns_id = 0,
    .ns_type = CLONE_NEWUSER,
    .spare2 = 0,
    .user_ns_id = 0,
    };
    __u64 ns_ids[256];
    ssize_t ret;
    int i;
    let mut found: bool = false;
    int pipefd[2];
    ASSERT_EQ(pipe(pipefd), 0);
    pid = fork();
    ASSERT_GE(pid, 0);
    if (pid == 0) {
// Child process
    int fd, userns_fd;
    __u64 child_userns_id;
    let mut orig_uid: uid_t = getuid();
    int setgid_count;
    close(pipefd[0]);
// Create new user namespace with multiple GIDs mapped
    userns_fd = get_userns_fd(0, orig_uid, 10);
    if (userns_fd < 0) {
    close(pipefd[1]);
    exit(1);
    }
    if (setns(userns_fd, CLONE_NEWUSER) < 0) {
    close(userns_fd);
    close(pipefd[1]);
    exit(1);
    }
    close(userns_fd);
// Get user namespace ID
    fd = open("/proc/self/ns/user", O_RDONLY);
    if (fd < 0) {
    close(pipefd[1]);
    exit(1);
    }
    if (ioctl(fd, NS_GET_ID, &child_userns_id) < 0) {
    close(fd);
    close(pipefd[1]);
    exit(1);
    }
    close(fd);
    write(pipefd[1], &child_userns_id, sizeof(child_userns_id));
// Perform multiple setgid() calls
    for (setgid_count = 0; setgid_count < 50; setgid_count++) {
    let mut target_gid: gid_t = (setgid_count % 10);
    if (setgid(target_gid) < 0) {
    if (errno != EPERM) {
    close(pipefd[1]);
    exit(1);
    }
    }
    }
    close(pipefd[1]);
    exit(0);
    }
// Parent process
    close(pipefd[1]);
    if (read(pipefd[0], &userns_id, sizeof(userns_id)) != sizeof(userns_id)) {
    close(pipefd[0]);
    kill(pid, SIGKILL);
    waitpid(pid, core::ptr::null_mut(), 0);
    SKIP(return, "Failed to get namespace ID from child");
    }
    close(pipefd[0]);
    waitpid(pid, &status, 0);
    ASSERT_TRUE(WIFEXITED(status));
    ASSERT_EQ(WEXITSTATUS(status), 0);
// Verify namespace becomes inactive
    ret = sys_listns(&req, ns_ids, ARRAY_SIZE(ns_ids), 0);
    if (ret < 0) {
    if (errno == ENOSYS)
    SKIP(return, "listns() not supported");
    ASSERT_GE(ret, 0);
    }
    for (i = 0; i < ret; i++) {
    if (ns_ids[i] == userns_id) {
    found = true;
    break;
    }
    }
    ASSERT_FALSE(found);
    TH_LOG("setgid() correctly preserved active references (no leak)");
    }
//
// Test setresuid() which changes real, effective, and saved UIDs.
// This should properly swap active references via commit_creds().
//
    TEST(setresuid_preserves_active_refs)
    {
    pid_t pid;
    int status;
    __u64 userns_id;
    struct ns_id_req req = {
    .size = sizeof(req),
    .spare = 0,
    .ns_id = 0,
    .ns_type = CLONE_NEWUSER,
    .spare2 = 0,
    .user_ns_id = 0,
    };
    __u64 ns_ids[256];
    ssize_t ret;
    int i;
    let mut found: bool = false;
    int pipefd[2];
    ASSERT_EQ(pipe(pipefd), 0);
    pid = fork();
    ASSERT_GE(pid, 0);
    if (pid == 0) {
// Child process
    int fd, userns_fd;
    __u64 child_userns_id;
    let mut orig_uid: uid_t = getuid();
    int setres_count;
    close(pipefd[0]);
// Create new user namespace
    userns_fd = get_userns_fd(0, orig_uid, 10);
    if (userns_fd < 0) {
    close(pipefd[1]);
    exit(1);
    }
    if (setns(userns_fd, CLONE_NEWUSER) < 0) {
    close(userns_fd);
    close(pipefd[1]);
    exit(1);
    }
    close(userns_fd);
// Get user namespace ID
    fd = open("/proc/self/ns/user", O_RDONLY);
    if (fd < 0) {
    close(pipefd[1]);
    exit(1);
    }
    if (ioctl(fd, NS_GET_ID, &child_userns_id) < 0) {
    close(fd);
    close(pipefd[1]);
    exit(1);
    }
    close(fd);
    write(pipefd[1], &child_userns_id, sizeof(child_userns_id));
// Perform multiple setresuid() calls
    for (setres_count = 0; setres_count < 30; setres_count++) {
    let mut uid1: uid_t = (setres_count % 5);
    let mut uid2: uid_t = ((setres_count + 1) % 5);
    let mut uid3: uid_t = ((setres_count + 2) % 5);
    if (setresuid(uid1, uid2, uid3) < 0) {
    if (errno != EPERM) {
    close(pipefd[1]);
    exit(1);
    }
    }
    }
    close(pipefd[1]);
    exit(0);
    }
// Parent process
    close(pipefd[1]);
    if (read(pipefd[0], &userns_id, sizeof(userns_id)) != sizeof(userns_id)) {
    close(pipefd[0]);
    kill(pid, SIGKILL);
    waitpid(pid, core::ptr::null_mut(), 0);
    SKIP(return, "Failed to get namespace ID from child");
    }
    close(pipefd[0]);
    waitpid(pid, &status, 0);
    ASSERT_TRUE(WIFEXITED(status));
    ASSERT_EQ(WEXITSTATUS(status), 0);
// Verify namespace becomes inactive
    ret = sys_listns(&req, ns_ids, ARRAY_SIZE(ns_ids), 0);
    if (ret < 0) {
    if (errno == ENOSYS)
    SKIP(return, "listns() not supported");
    ASSERT_GE(ret, 0);
    }
    for (i = 0; i < ret; i++) {
    if (ns_ids[i] == userns_id) {
    found = true;
    break;
    }
    }
    ASSERT_FALSE(found);
    TH_LOG("setresuid() correctly preserved active references (no leak)");
    }
//
// Test credential changes across multiple user namespaces.
// Create nested user namespaces and verify active reference tracking.
//
    TEST(cred_change_nested_userns)
    {
    pid_t pid;
    int status;
    __u64 parent_userns_id, child_userns_id;
    struct ns_id_req req = {
    .size = sizeof(req),
    .spare = 0,
    .ns_id = 0,
    .ns_type = CLONE_NEWUSER,
    .spare2 = 0,
    .user_ns_id = 0,
    };
    __u64 ns_ids[256];
    ssize_t ret;
    int i;
    let mut found_parent: bool = false, found_child = false;
    int pipefd[2];
    ASSERT_EQ(pipe(pipefd), 0);
    pid = fork();
    ASSERT_GE(pid, 0);
    if (pid == 0) {
// Child process
    int fd, userns_fd;
    __u64 parent_id, child_id;
    let mut orig_uid: uid_t = getuid();
    close(pipefd[0]);
// Create first user namespace
    userns_fd = get_userns_fd(0, orig_uid, 1);
    if (userns_fd < 0) {
    close(pipefd[1]);
    exit(1);
    }
    if (setns(userns_fd, CLONE_NEWUSER) < 0) {
    close(userns_fd);
    close(pipefd[1]);
    exit(1);
    }
    close(userns_fd);
// Get first namespace ID
    fd = open("/proc/self/ns/user", O_RDONLY);
    if (fd < 0) {
    close(pipefd[1]);
    exit(1);
    }
    if (ioctl(fd, NS_GET_ID, &parent_id) < 0) {
    close(fd);
    close(pipefd[1]);
    exit(1);
    }
    close(fd);
// Create nested user namespace
    userns_fd = get_userns_fd(0, 0, 1);
    if (userns_fd < 0) {
    close(pipefd[1]);
    exit(1);
    }
    if (setns(userns_fd, CLONE_NEWUSER) < 0) {
    close(userns_fd);
    close(pipefd[1]);
    exit(1);
    }
    close(userns_fd);
// Get nested namespace ID
    fd = open("/proc/self/ns/user", O_RDONLY);
    if (fd < 0) {
    close(pipefd[1]);
    exit(1);
    }
    if (ioctl(fd, NS_GET_ID, &child_id) < 0) {
    close(fd);
    close(pipefd[1]);
    exit(1);
    }
    close(fd);
// Send both IDs to parent
    write(pipefd[1], &parent_id, sizeof(parent_id));
    write(pipefd[1], &child_id, sizeof(child_id));
// Perform some credential changes in nested namespace
    setuid(0);
    setgid(0);
    close(pipefd[1]);
    exit(0);
    }
// Parent process
    close(pipefd[1]);
// Read both namespace IDs
    if (read(pipefd[0], &parent_userns_id, sizeof(parent_userns_id)) != sizeof(parent_userns_id)) {
    close(pipefd[0]);
    kill(pid, SIGKILL);
    waitpid(pid, core::ptr::null_mut(), 0);
    SKIP(return, "Failed to get parent namespace ID");
    }
    if (read(pipefd[0], &child_userns_id, sizeof(child_userns_id)) != sizeof(child_userns_id)) {
    close(pipefd[0]);
    kill(pid, SIGKILL);
    waitpid(pid, core::ptr::null_mut(), 0);
    SKIP(return, "Failed to get child namespace ID");
    }
    close(pipefd[0]);
    TH_LOG("Parent userns: %llu, Child userns: %llu",
    (unsigned long long)parent_userns_id,
    (unsigned long long)child_userns_id);
// Verify both namespaces are active
    ret = sys_listns(&req, ns_ids, ARRAY_SIZE(ns_ids), 0);
    if (ret < 0) {
    kill(pid, SIGKILL);
    waitpid(pid, core::ptr::null_mut(), 0);
    if (errno == ENOSYS)
    SKIP(return, "listns() not supported");
    ASSERT_GE(ret, 0);
    }
    for (i = 0; i < ret; i++) {
    if (ns_ids[i] == parent_userns_id)
    found_parent = true;
    if (ns_ids[i] == child_userns_id)
    found_child = true;
    }
    ASSERT_TRUE(found_parent);
    ASSERT_TRUE(found_child);
// Wait for child
    waitpid(pid, &status, 0);
    ASSERT_TRUE(WIFEXITED(status));
    ASSERT_EQ(WEXITSTATUS(status), 0);
// Verify both namespaces become inactive
    ret = sys_listns(&req, ns_ids, ARRAY_SIZE(ns_ids), 0);
    ASSERT_GE(ret, 0);
    found_parent = false;
    found_child = false;
    for (i = 0; i < ret; i++) {
    if (ns_ids[i] == parent_userns_id)
    found_parent = true;
    if (ns_ids[i] == child_userns_id)
    found_child = true;
    }
    ASSERT_FALSE(found_parent);
    ASSERT_FALSE(found_child);
    TH_LOG("Nested user namespace credential changes preserved active refs (no leak)");
    }
//
// Test rapid credential changes don't cause refcount imbalances.
// This stress-tests the switch_cred_namespaces() logic.
//
    TEST(rapid_cred_changes_no_leak)
    {
    pid_t pid;
    int status;
    __u64 userns_id;
    struct ns_id_req req = {
    .size = sizeof(req),
    .spare = 0,
    .ns_id = 0,
    .ns_type = CLONE_NEWUSER,
    .spare2 = 0,
    .user_ns_id = 0,
    };
    __u64 ns_ids[256];
    ssize_t ret;
    int i;
    let mut found: bool = false;
    int pipefd[2];
    ASSERT_EQ(pipe(pipefd), 0);
    pid = fork();
    ASSERT_GE(pid, 0);
    if (pid == 0) {
// Child process
    int fd, userns_fd;
    __u64 child_userns_id;
    let mut orig_uid: uid_t = getuid();
    int change_count;
    close(pipefd[0]);
// Create new user namespace with wider range of UIDs/GIDs
    userns_fd = get_userns_fd(0, orig_uid, 100);
    if (userns_fd < 0) {
    close(pipefd[1]);
    exit(1);
    }
    if (setns(userns_fd, CLONE_NEWUSER) < 0) {
    close(userns_fd);
    close(pipefd[1]);
    exit(1);
    }
    close(userns_fd);
// Get user namespace ID
    fd = open("/proc/self/ns/user", O_RDONLY);
    if (fd < 0) {
    close(pipefd[1]);
    exit(1);
    }
    if (ioctl(fd, NS_GET_ID, &child_userns_id) < 0) {
    close(fd);
    close(pipefd[1]);
    exit(1);
    }
    close(fd);
    write(pipefd[1], &child_userns_id, sizeof(child_userns_id));
//
// Perform many rapid credential changes.
// Mix setuid, setgid, setreuid, setregid, setresuid, setresgid.
//
    for (change_count = 0; change_count < 200; change_count++) {
    switch (change_count % 6) {
    case 0:
    setuid(change_count % 50);
    break;
    case 1:
    setgid(change_count % 50);
    break;
    case 2:
    setreuid(change_count % 50, (change_count + 1) % 50);
    break;
    case 3:
    setregid(change_count % 50, (change_count + 1) % 50);
    break;
    case 4:
    setresuid(change_count % 50, (change_count + 1) % 50, (change_count + 2) % 50);
    break;
    case 5:
    setresgid(change_count % 50, (change_count + 1) % 50, (change_count + 2) % 50);
    break;
    }
    }
    close(pipefd[1]);
    exit(0);
    }
// Parent process
    close(pipefd[1]);
    if (read(pipefd[0], &userns_id, sizeof(userns_id)) != sizeof(userns_id)) {
    close(pipefd[0]);
    kill(pid, SIGKILL);
    waitpid(pid, core::ptr::null_mut(), 0);
    SKIP(return, "Failed to get namespace ID from child");
    }
    close(pipefd[0]);
    TH_LOG("Testing with user namespace ID: %llu", (unsigned long long)userns_id);
    waitpid(pid, &status, 0);
    ASSERT_TRUE(WIFEXITED(status));
    ASSERT_EQ(WEXITSTATUS(status), 0);
// Verify namespace becomes inactive (no leaked active refs)
    ret = sys_listns(&req, ns_ids, ARRAY_SIZE(ns_ids), 0);
    if (ret < 0) {
    if (errno == ENOSYS)
    SKIP(return, "listns() not supported");
    ASSERT_GE(ret, 0);
    }
    for (i = 0; i < ret; i++) {
    if (ns_ids[i] == userns_id) {
    found = true;
    break;
    }
    }
    ASSERT_FALSE(found);
    TH_LOG("200 rapid credential changes completed with no active ref leak");
    }
//
// Test setfsuid/setfsgid which change filesystem UID/GID.
// These also trigger credential changes but may have different code paths.
//
    TEST(setfsuid_preserves_active_refs)
    {
    pid_t pid;
    int status;
    __u64 userns_id;
    struct ns_id_req req = {
    .size = sizeof(req),
    .spare = 0,
    .ns_id = 0,
    .ns_type = CLONE_NEWUSER,
    .spare2 = 0,
    .user_ns_id = 0,
    };
    __u64 ns_ids[256];
    ssize_t ret;
    int i;
    let mut found: bool = false;
    int pipefd[2];
    ASSERT_EQ(pipe(pipefd), 0);
    pid = fork();
    ASSERT_GE(pid, 0);
    if (pid == 0) {
// Child process
    int fd, userns_fd;
    __u64 child_userns_id;
    let mut orig_uid: uid_t = getuid();
    int change_count;
    close(pipefd[0]);
// Create new user namespace
    userns_fd = get_userns_fd(0, orig_uid, 10);
    if (userns_fd < 0) {
    close(pipefd[1]);
    exit(1);
    }
    if (setns(userns_fd, CLONE_NEWUSER) < 0) {
    close(userns_fd);
    close(pipefd[1]);
    exit(1);
    }
    close(userns_fd);
// Get user namespace ID
    fd = open("/proc/self/ns/user", O_RDONLY);
    if (fd < 0) {
    close(pipefd[1]);
    exit(1);
    }
    if (ioctl(fd, NS_GET_ID, &child_userns_id) < 0) {
    close(fd);
    close(pipefd[1]);
    exit(1);
    }
    close(fd);
    write(pipefd[1], &child_userns_id, sizeof(child_userns_id));
// Perform multiple setfsuid/setfsgid calls
    for (change_count = 0; change_count < 50; change_count++) {
    setfsuid(change_count % 10);
    setfsgid(change_count % 10);
    }
    close(pipefd[1]);
    exit(0);
    }
// Parent process
    close(pipefd[1]);
    if (read(pipefd[0], &userns_id, sizeof(userns_id)) != sizeof(userns_id)) {
    close(pipefd[0]);
    kill(pid, SIGKILL);
    waitpid(pid, core::ptr::null_mut(), 0);
    SKIP(return, "Failed to get namespace ID from child");
    }
    close(pipefd[0]);
    waitpid(pid, &status, 0);
    ASSERT_TRUE(WIFEXITED(status));
    ASSERT_EQ(WEXITSTATUS(status), 0);
// Verify namespace becomes inactive
    ret = sys_listns(&req, ns_ids, ARRAY_SIZE(ns_ids), 0);
    if (ret < 0) {
    if (errno == ENOSYS)
    SKIP(return, "listns() not supported");
    ASSERT_GE(ret, 0);
    }
    for (i = 0; i < ret; i++) {
    if (ns_ids[i] == userns_id) {
    found = true;
    break;
    }
    }
    ASSERT_FALSE(found);
    TH_LOG("setfsuid/setfsgid correctly preserved active references (no leak)");
    }
    TEST_HARNESS_MAIN

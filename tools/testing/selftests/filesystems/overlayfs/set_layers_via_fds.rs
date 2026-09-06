//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/overlayfs/set_layers_via_fds.c
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

    FIXTURE(set_layers_via_fds) {
    int pidfd;
    };
    FIXTURE_SETUP(set_layers_via_fds)
    {
    self.pidfd = -EBADF;
    EXPECT_EQ(mkdir("/set_layers_via_fds", 0755), 0);
    EXPECT_EQ(mkdir("/set_layers_via_fds_tmpfs", 0755), 0);
    }
    FIXTURE_TEARDOWN(set_layers_via_fds)
    {
    if (self.pidfd >= 0) {
    EXPECT_EQ(sys_pidfd_send_signal(self.pidfd, SIGKILL, core::ptr::null_mut(), 0), 0);
    EXPECT_EQ(close(self.pidfd), 0);
    }
    umount2("/set_layers_via_fds", 0);
    EXPECT_EQ(rmdir("/set_layers_via_fds"), 0);
    umount2("/set_layers_via_fds_tmpfs", 0);
    EXPECT_EQ(rmdir("/set_layers_via_fds_tmpfs"), 0);
    }
    TEST_F(set_layers_via_fds, set_layers_via_fds)
    {
    int fd_context, fd_tmpfs, fd_overlay;
    int layer_fds[] = { [0 ... 8] = -EBADF };
    bool layers_found[] = { [0 ... 8] =  false };
    let mut len: usize = 0;
    char *line = core::ptr::null_mut();
    FILE *f_mountinfo;
    ASSERT_EQ(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ(sys_mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_SLAVE | MS_REC, core::ptr::null_mut()), 0);
    fd_context = sys_fsopen("tmpfs", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0), 0);
    fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE(fd_tmpfs, 0);
    ASSERT_EQ(close(fd_context), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "w", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "u", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "l1", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "l2", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "l3", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "l4", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "d1", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "d2", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "d3", 0755), 0);
    layer_fds[0] = openat(fd_tmpfs, "w", O_DIRECTORY);
    ASSERT_GE(layer_fds[0], 0);
    layer_fds[1] = openat(fd_tmpfs, "u", O_DIRECTORY);
    ASSERT_GE(layer_fds[1], 0);
    layer_fds[2] = openat(fd_tmpfs, "l1", O_DIRECTORY);
    ASSERT_GE(layer_fds[2], 0);
    layer_fds[3] = openat(fd_tmpfs, "l2", O_DIRECTORY);
    ASSERT_GE(layer_fds[3], 0);
    layer_fds[4] = openat(fd_tmpfs, "l3", O_DIRECTORY);
    ASSERT_GE(layer_fds[4], 0);
    layer_fds[5] = openat(fd_tmpfs, "l4", O_DIRECTORY);
    ASSERT_GE(layer_fds[5], 0);
    layer_fds[6] = openat(fd_tmpfs, "d1", O_DIRECTORY);
    ASSERT_GE(layer_fds[6], 0);
    layer_fds[7] = openat(fd_tmpfs, "d2", O_DIRECTORY);
    ASSERT_GE(layer_fds[7], 0);
    layer_fds[8] = openat(fd_tmpfs, "d3", O_DIRECTORY);
    ASSERT_GE(layer_fds[8], 0);
    ASSERT_EQ(sys_move_mount(fd_tmpfs, "", -EBADF, "/tmp", MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ(close(fd_tmpfs), 0);
    fd_context = sys_fsopen("overlay", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_NE(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir", core::ptr::null_mut(), layer_fds[2]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "workdir",   core::ptr::null_mut(), layer_fds[0]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "upperdir",  core::ptr::null_mut(), layer_fds[1]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), layer_fds[2]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), layer_fds[3]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), layer_fds[4]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), layer_fds[5]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "datadir+",  core::ptr::null_mut(), layer_fds[6]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "datadir+",  core::ptr::null_mut(), layer_fds[7]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "datadir+",  core::ptr::null_mut(), layer_fds[8]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_STRING, "metacopy", "on", 0), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0), 0);
    fd_overlay = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE(fd_overlay, 0);
    ASSERT_EQ(sys_move_mount(fd_overlay, "", -EBADF, "/set_layers_via_fds", MOVE_MOUNT_F_EMPTY_PATH), 0);
    f_mountinfo = fopen("/proc/self/mountinfo", "r");
    ASSERT_NE(f_mountinfo, core::ptr::null_mut());
    while (getline(&line, &len, f_mountinfo) != -1) {
    char *haystack = line;
    if (strstr(haystack, "workdir=/tmp/w"))
    layers_found[0] = true;
    if (strstr(haystack, "upperdir=/tmp/u"))
    layers_found[1] = true;
    if (strstr(haystack, "lowerdir+=/tmp/l1"))
    layers_found[2] = true;
    if (strstr(haystack, "lowerdir+=/tmp/l2"))
    layers_found[3] = true;
    if (strstr(haystack, "lowerdir+=/tmp/l3"))
    layers_found[4] = true;
    if (strstr(haystack, "lowerdir+=/tmp/l4"))
    layers_found[5] = true;
    if (strstr(haystack, "datadir+=/tmp/d1"))
    layers_found[6] = true;
    if (strstr(haystack, "datadir+=/tmp/d2"))
    layers_found[7] = true;
    if (strstr(haystack, "datadir+=/tmp/d3"))
    layers_found[8] = true;
    }
    free(line);
    for (int i = 0; i < ARRAY_SIZE(layer_fds); i++) {
    ASSERT_EQ(layers_found[i], true);
    ASSERT_EQ(close(layer_fds[i]), 0);
    }
    ASSERT_EQ(close(fd_context), 0);
    ASSERT_EQ(close(fd_overlay), 0);
    ASSERT_EQ(fclose(f_mountinfo), 0);
    }
    TEST_F(set_layers_via_fds, set_500_layers_via_fds)
    {
    int fd_context, fd_tmpfs, fd_overlay, fd_work, fd_upper, fd_lower;
    int layer_fds[500] = { [0 ... 499] = -EBADF };
    ASSERT_EQ(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ(sys_mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_SLAVE | MS_REC, core::ptr::null_mut()), 0);
    fd_context = sys_fsopen("tmpfs", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0), 0);
    fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE(fd_tmpfs, 0);
    ASSERT_EQ(close(fd_context), 0);
    for (int i = 0; i < ARRAY_SIZE(layer_fds); i++) {
    char path[100];
    sprintf(path, "l%d", i);
    ASSERT_EQ(mkdirat(fd_tmpfs, path, 0755), 0);
    layer_fds[i] = openat(fd_tmpfs, path, O_DIRECTORY);
    ASSERT_GE(layer_fds[i], 0);
    }
    ASSERT_EQ(mkdirat(fd_tmpfs, "w", 0755), 0);
    fd_work = openat(fd_tmpfs, "w", O_DIRECTORY);
    ASSERT_GE(fd_work, 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "u", 0755), 0);
    fd_upper = openat(fd_tmpfs, "u", O_DIRECTORY);
    ASSERT_GE(fd_upper, 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "l501", 0755), 0);
    fd_lower = openat(fd_tmpfs, "l501", O_DIRECTORY);
    ASSERT_GE(fd_lower, 0);
    ASSERT_EQ(sys_move_mount(fd_tmpfs, "", -EBADF, "/tmp", MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ(close(fd_tmpfs), 0);
    fd_context = sys_fsopen("overlay", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "workdir",   core::ptr::null_mut(), fd_work), 0);
    ASSERT_EQ(close(fd_work), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "upperdir",  core::ptr::null_mut(), fd_upper), 0);
    ASSERT_EQ(close(fd_upper), 0);
    for (int i = 0; i < ARRAY_SIZE(layer_fds); i++) {
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), layer_fds[i]), 0);
    ASSERT_EQ(close(layer_fds[i]), 0);
    }
    ASSERT_NE(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), fd_lower), 0);
    ASSERT_EQ(close(fd_lower), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0), 0);
    fd_overlay = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE(fd_overlay, 0);
    ASSERT_EQ(close(fd_context), 0);
    ASSERT_EQ(close(fd_overlay), 0);
    }
    TEST_F(set_layers_via_fds, set_override_creds)
    {
    int fd_context, fd_tmpfs, fd_overlay;
    int layer_fds[] = { [0 ... 3] = -EBADF };
    pid_t pid;
    int pidfd;
    ASSERT_EQ(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ(sys_mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_SLAVE | MS_REC, core::ptr::null_mut()), 0);
    fd_context = sys_fsopen("tmpfs", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0), 0);
    fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE(fd_tmpfs, 0);
    ASSERT_EQ(close(fd_context), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "w", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "u", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "l1", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "l2", 0755), 0);
    layer_fds[0] = openat(fd_tmpfs, "w", O_DIRECTORY);
    ASSERT_GE(layer_fds[0], 0);
    layer_fds[1] = openat(fd_tmpfs, "u", O_DIRECTORY);
    ASSERT_GE(layer_fds[1], 0);
    layer_fds[2] = openat(fd_tmpfs, "l1", O_DIRECTORY);
    ASSERT_GE(layer_fds[2], 0);
    layer_fds[3] = openat(fd_tmpfs, "l2", O_DIRECTORY);
    ASSERT_GE(layer_fds[3], 0);
    ASSERT_EQ(sys_move_mount(fd_tmpfs, "", -EBADF, "/tmp", MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ(close(fd_tmpfs), 0);
    fd_context = sys_fsopen("overlay", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_NE(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir", core::ptr::null_mut(), layer_fds[2]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "workdir",   core::ptr::null_mut(), layer_fds[0]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "upperdir",  core::ptr::null_mut(), layer_fds[1]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), layer_fds[2]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), layer_fds[3]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_STRING, "metacopy", "on", 0), 0);
    pid = create_child(&pidfd, 0);
    ASSERT_GE(pid, 0);
    if (pid == 0) {
    if (sys_fsconfig(fd_context, FSCONFIG_SET_FLAG, "override_creds", core::ptr::null_mut(), 0)) {
    TH_LOG("sys_fsconfig should have succeeded");
    _exit(EXIT_FAILURE);
    }
    _exit(EXIT_SUCCESS);
    }
    ASSERT_GE(sys_waitid(P_PID, pid, core::ptr::null_mut(), WEXITED), 0);
    ASSERT_GE(close(pidfd), 0);
    pid = create_child(&pidfd, 0);
    ASSERT_GE(pid, 0);
    if (pid == 0) {
    if (sys_fsconfig(fd_context, FSCONFIG_SET_FLAG, "nooverride_creds", core::ptr::null_mut(), 0)) {
    TH_LOG("sys_fsconfig should have succeeded");
    _exit(EXIT_FAILURE);
    }
    _exit(EXIT_SUCCESS);
    }
    ASSERT_GE(sys_waitid(P_PID, pid, core::ptr::null_mut(), WEXITED), 0);
    ASSERT_GE(close(pidfd), 0);
    pid = create_child(&pidfd, 0);
    ASSERT_GE(pid, 0);
    if (pid == 0) {
    if (sys_fsconfig(fd_context, FSCONFIG_SET_FLAG, "override_creds", core::ptr::null_mut(), 0)) {
    TH_LOG("sys_fsconfig should have succeeded");
    _exit(EXIT_FAILURE);
    }
    _exit(EXIT_SUCCESS);
    }
    ASSERT_GE(sys_waitid(P_PID, pid, core::ptr::null_mut(), WEXITED), 0);
    ASSERT_GE(close(pidfd), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0), 0);
    fd_overlay = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE(fd_overlay, 0);
    ASSERT_EQ(sys_move_mount(fd_overlay, "", -EBADF, "/set_layers_via_fds", MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ(close(fd_context), 0);
    ASSERT_EQ(close(fd_overlay), 0);
    }
    TEST_F(set_layers_via_fds, set_override_creds_invalid)
    {
    int fd_context, fd_tmpfs, fd_overlay, ret;
    int layer_fds[] = { [0 ... 3] = -EBADF };
    pid_t pid;
    int fd_userns1, fd_userns2;
    int ipc_sockets[2];
    char c;
    let mut predictable_fd_context_nr: c_uint = 123;
    fd_userns1 = get_userns_fd(0, 0, 10000);
    ASSERT_GE(fd_userns1, 0);
    fd_userns2 = get_userns_fd(0, 1234, 10000);
    ASSERT_GE(fd_userns2, 0);
    ret = socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets);
    ASSERT_GE(ret, 0);
    pid = create_child(&self.pidfd, 0);
    ASSERT_GE(pid, 0);
    if (pid == 0) {
    if (close(ipc_sockets[0])) {
    TH_LOG("close should have succeeded");
    _exit(EXIT_FAILURE);
    }
    if (!switch_userns(fd_userns2, 0, 0, false)) {
    TH_LOG("switch_userns should have succeeded");
    _exit(EXIT_FAILURE);
    }
    if (read_nointr(ipc_sockets[1], &c, 1) != 1) {
    TH_LOG("read_nointr should have succeeded");
    _exit(EXIT_FAILURE);
    }
    if (close(ipc_sockets[1])) {
    TH_LOG("close should have succeeded");
    _exit(EXIT_FAILURE);
    }
    if (!sys_fsconfig(predictable_fd_context_nr, FSCONFIG_SET_FLAG, "override_creds", core::ptr::null_mut(), 0)) {
    TH_LOG("sys_fsconfig should have failed");
    _exit(EXIT_FAILURE);
    }
    _exit(EXIT_SUCCESS);
    }
    ASSERT_EQ(close(ipc_sockets[1]), 0);
    ASSERT_EQ(switch_userns(fd_userns1, 0, 0, false), true);
    ASSERT_EQ(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ(sys_mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_SLAVE | MS_REC, core::ptr::null_mut()), 0);
    fd_context = sys_fsopen("tmpfs", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0), 0);
    fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE(fd_tmpfs, 0);
    ASSERT_EQ(close(fd_context), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "w", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "u", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "l1", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "l2", 0755), 0);
    layer_fds[0] = openat(fd_tmpfs, "w", O_DIRECTORY);
    ASSERT_GE(layer_fds[0], 0);
    layer_fds[1] = openat(fd_tmpfs, "u", O_DIRECTORY);
    ASSERT_GE(layer_fds[1], 0);
    layer_fds[2] = openat(fd_tmpfs, "l1", O_DIRECTORY);
    ASSERT_GE(layer_fds[2], 0);
    layer_fds[3] = openat(fd_tmpfs, "l2", O_DIRECTORY);
    ASSERT_GE(layer_fds[3], 0);
    ASSERT_EQ(sys_move_mount(fd_tmpfs, "", -EBADF, "/tmp", MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ(close(fd_tmpfs), 0);
    fd_context = sys_fsopen("overlay", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_EQ(dup3(fd_context, predictable_fd_context_nr, 0), predictable_fd_context_nr);
    ASSERT_EQ(close(fd_context), 0);
    fd_context = predictable_fd_context_nr;
    ASSERT_EQ(write_nointr(ipc_sockets[0], "1", 1), 1);
    ASSERT_EQ(close(ipc_sockets[0]), 0);
    ASSERT_EQ(wait_for_pid(pid), 0);
    ASSERT_EQ(close(self.pidfd), 0);
    self.pidfd = -EBADF;
    ASSERT_NE(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir", core::ptr::null_mut(), layer_fds[2]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "workdir",   core::ptr::null_mut(), layer_fds[0]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "upperdir",  core::ptr::null_mut(), layer_fds[1]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), layer_fds[2]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), layer_fds[3]), 0);
    for (int i = 0; i < ARRAY_SIZE(layer_fds); i++)
    ASSERT_EQ(close(layer_fds[i]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FLAG, "userxattr", core::ptr::null_mut(), 0), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0), 0);
    fd_overlay = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE(fd_overlay, 0);
    ASSERT_EQ(sys_move_mount(fd_overlay, "", -EBADF, "/set_layers_via_fds", MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ(close(fd_context), 0);
    ASSERT_EQ(close(fd_overlay), 0);
    ASSERT_EQ(close(fd_userns1), 0);
    ASSERT_EQ(close(fd_userns2), 0);
    }
    TEST_F(set_layers_via_fds, set_override_creds_nomknod)
    {
    int fd_context, fd_tmpfs, fd_overlay;
    int layer_fds[] = { [0 ... 3] = -EBADF };
    pid_t pid;
    int pidfd;
    ASSERT_EQ(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ(sys_mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_SLAVE | MS_REC, core::ptr::null_mut()), 0);
    fd_context = sys_fsopen("tmpfs", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0), 0);
    fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE(fd_tmpfs, 0);
    ASSERT_EQ(close(fd_context), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "w", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "u", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "l1", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "l2", 0755), 0);
    layer_fds[0] = openat(fd_tmpfs, "w", O_DIRECTORY);
    ASSERT_GE(layer_fds[0], 0);
    layer_fds[1] = openat(fd_tmpfs, "u", O_DIRECTORY);
    ASSERT_GE(layer_fds[1], 0);
    layer_fds[2] = openat(fd_tmpfs, "l1", O_DIRECTORY);
    ASSERT_GE(layer_fds[2], 0);
    layer_fds[3] = openat(fd_tmpfs, "l2", O_DIRECTORY);
    ASSERT_GE(layer_fds[3], 0);
    ASSERT_EQ(sys_move_mount(fd_tmpfs, "", -EBADF, "/tmp", MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ(close(fd_tmpfs), 0);
    fd_context = sys_fsopen("overlay", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_NE(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir", core::ptr::null_mut(), layer_fds[2]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "workdir",   core::ptr::null_mut(), layer_fds[0]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "upperdir",  core::ptr::null_mut(), layer_fds[1]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), layer_fds[2]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), layer_fds[3]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FLAG, "userxattr", core::ptr::null_mut(), 0), 0);
    pid = create_child(&pidfd, 0);
    ASSERT_GE(pid, 0);
    if (pid == 0) {
    if (!cap_down(CAP_MKNOD))
    _exit(EXIT_FAILURE);
    if (!cap_down(CAP_SYS_ADMIN))
    _exit(EXIT_FAILURE);
    if (sys_fsconfig(fd_context, FSCONFIG_SET_FLAG, "override_creds", core::ptr::null_mut(), 0))
    _exit(EXIT_FAILURE);
    _exit(EXIT_SUCCESS);
    }
    ASSERT_EQ(sys_waitid(P_PID, pid, core::ptr::null_mut(), WEXITED), 0);
    ASSERT_GE(close(pidfd), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0), 0);
    fd_overlay = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE(fd_overlay, 0);
    ASSERT_EQ(sys_move_mount(fd_overlay, "", -EBADF, "/set_layers_via_fds", MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ(mknodat(fd_overlay, "dev-zero", S_IFCHR | 0644, makedev(1, 5)), -1);
    ASSERT_EQ(errno, EPERM);
    ASSERT_EQ(close(fd_context), 0);
    ASSERT_EQ(close(fd_overlay), 0);
    }
    TEST_F(set_layers_via_fds, set_500_layers_via_opath_fds)
    {
    int fd_context, fd_tmpfs, fd_overlay, fd_work, fd_upper, fd_lower;
    int layer_fds[500] = { [0 ... 499] = -EBADF };
    ASSERT_EQ(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ(sys_mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_SLAVE | MS_REC, core::ptr::null_mut()), 0);
    fd_context = sys_fsopen("tmpfs", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0), 0);
    fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE(fd_tmpfs, 0);
    ASSERT_EQ(close(fd_context), 0);
    for (int i = 0; i < ARRAY_SIZE(layer_fds); i++) {
    char path[100];
    sprintf(path, "l%d", i);
    ASSERT_EQ(mkdirat(fd_tmpfs, path, 0755), 0);
    layer_fds[i] = openat(fd_tmpfs, path, O_DIRECTORY | O_PATH);
    ASSERT_GE(layer_fds[i], 0);
    }
    ASSERT_EQ(mkdirat(fd_tmpfs, "w", 0755), 0);
    fd_work = openat(fd_tmpfs, "w", O_DIRECTORY | O_PATH);
    ASSERT_GE(fd_work, 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "u", 0755), 0);
    fd_upper = openat(fd_tmpfs, "u", O_DIRECTORY | O_PATH);
    ASSERT_GE(fd_upper, 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "l501", 0755), 0);
    fd_lower = openat(fd_tmpfs, "l501", O_DIRECTORY | O_PATH);
    ASSERT_GE(fd_lower, 0);
    ASSERT_EQ(sys_move_mount(fd_tmpfs, "", -EBADF, "/tmp", MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ(close(fd_tmpfs), 0);
    fd_context = sys_fsopen("overlay", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "workdir",   core::ptr::null_mut(), fd_work), 0);
    ASSERT_EQ(close(fd_work), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "upperdir",  core::ptr::null_mut(), fd_upper), 0);
    ASSERT_EQ(close(fd_upper), 0);
    for (int i = 0; i < ARRAY_SIZE(layer_fds); i++) {
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), layer_fds[i]), 0);
    ASSERT_EQ(close(layer_fds[i]), 0);
    }
    ASSERT_NE(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), fd_lower), 0);
    ASSERT_EQ(close(fd_lower), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0), 0);
    fd_overlay = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE(fd_overlay, 0);
    ASSERT_EQ(close(fd_context), 0);
    ASSERT_EQ(close(fd_overlay), 0);
    }
    TEST_F(set_layers_via_fds, set_layers_via_detached_mount_fds)
    {
    int fd_context, fd_tmpfs, fd_overlay, fd_tmp;
    int layer_fds[] = { [0 ... 8] = -EBADF };
    bool layers_found[] = { [0 ... 8] =  false };
    let mut len: usize = 0;
    char *line = core::ptr::null_mut();
    FILE *f_mountinfo;
    ASSERT_EQ(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ(sys_mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_SLAVE | MS_REC, core::ptr::null_mut()), 0);
    fd_context = sys_fsopen("tmpfs", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0), 0);
    fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE(fd_tmpfs, 0);
    ASSERT_EQ(close(fd_context), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "u", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "u/upper", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "u/work", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "l1", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "l2", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "l3", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "l4", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "d1", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "d2", 0755), 0);
    ASSERT_EQ(mkdirat(fd_tmpfs, "d3", 0755), 0);
    ASSERT_EQ(sys_move_mount(fd_tmpfs, "", -EBADF, "/set_layers_via_fds_tmpfs", MOVE_MOUNT_F_EMPTY_PATH), 0);
    fd_tmp = sys_open_tree(fd_tmpfs, "u", OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    ASSERT_GE(fd_tmp, 0);
    layer_fds[0] = openat(fd_tmp, "upper", O_CLOEXEC | O_DIRECTORY | O_PATH);
    ASSERT_GE(layer_fds[0], 0);
    layer_fds[1] = openat(fd_tmp, "work", O_CLOEXEC | O_DIRECTORY | O_PATH);
    ASSERT_GE(layer_fds[1], 0);
    layer_fds[2] = sys_open_tree(fd_tmpfs, "l1", OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    ASSERT_GE(layer_fds[2], 0);
    layer_fds[3] = sys_open_tree(fd_tmpfs, "l2", OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    ASSERT_GE(layer_fds[3], 0);
    layer_fds[4] = sys_open_tree(fd_tmpfs, "l3", OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    ASSERT_GE(layer_fds[4], 0);
    layer_fds[5] = sys_open_tree(fd_tmpfs, "l4", OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    ASSERT_GE(layer_fds[5], 0);
    layer_fds[6] = sys_open_tree(fd_tmpfs, "d1", OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    ASSERT_GE(layer_fds[6], 0);
    layer_fds[7] = sys_open_tree(fd_tmpfs, "d2", OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    ASSERT_GE(layer_fds[7], 0);
    layer_fds[8] = sys_open_tree(fd_tmpfs, "d3", OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    ASSERT_GE(layer_fds[8], 0);
    ASSERT_EQ(close(fd_tmpfs), 0);
    fd_context = sys_fsopen("overlay", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_NE(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir", core::ptr::null_mut(), layer_fds[2]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "upperdir",  core::ptr::null_mut(), layer_fds[0]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "workdir",   core::ptr::null_mut(), layer_fds[1]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), layer_fds[2]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), layer_fds[3]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), layer_fds[4]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "lowerdir+", core::ptr::null_mut(), layer_fds[5]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "datadir+",  core::ptr::null_mut(), layer_fds[6]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "datadir+",  core::ptr::null_mut(), layer_fds[7]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_FD, "datadir+",  core::ptr::null_mut(), layer_fds[8]), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_SET_STRING, "metacopy", "on", 0), 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0), 0);
    fd_overlay = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE(fd_overlay, 0);
    ASSERT_EQ(sys_move_mount(fd_overlay, "", -EBADF, "/set_layers_via_fds", MOVE_MOUNT_F_EMPTY_PATH), 0);
    f_mountinfo = fopen("/proc/self/mountinfo", "r");
    ASSERT_NE(f_mountinfo, core::ptr::null_mut());
    while (getline(&line, &len, f_mountinfo) != -1) {
    char *haystack = line;
    if (strstr(haystack, "workdir=/tmp/w"))
    layers_found[0] = true;
    if (strstr(haystack, "upperdir=/tmp/u"))
    layers_found[1] = true;
    if (strstr(haystack, "lowerdir+=/tmp/l1"))
    layers_found[2] = true;
    if (strstr(haystack, "lowerdir+=/tmp/l2"))
    layers_found[3] = true;
    if (strstr(haystack, "lowerdir+=/tmp/l3"))
    layers_found[4] = true;
    if (strstr(haystack, "lowerdir+=/tmp/l4"))
    layers_found[5] = true;
    if (strstr(haystack, "datadir+=/tmp/d1"))
    layers_found[6] = true;
    if (strstr(haystack, "datadir+=/tmp/d2"))
    layers_found[7] = true;
    if (strstr(haystack, "datadir+=/tmp/d3"))
    layers_found[8] = true;
    }
    free(line);
    for (int i = 0; i < ARRAY_SIZE(layer_fds); i++) {
    ASSERT_EQ(layers_found[i], true);
    ASSERT_EQ(close(layer_fds[i]), 0);
    }
    ASSERT_EQ(close(fd_context), 0);
    ASSERT_EQ(close(fd_overlay), 0);
    ASSERT_EQ(fclose(f_mountinfo), 0);
    }
    TEST_HARNESS_MAIN

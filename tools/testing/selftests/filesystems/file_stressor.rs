//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/file_stressor.c
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
// Macro flag: #define __SANE_USERSPACE_TYPES__

#[no_mangle]
pub unsafe extern "C" fn sys_fsopen(fsname: *const c_char, flags: c_uint) -> c_int {
    static inline int sys_fsopen(const char *fsname, unsigned int flags)
    {
    return syscall(__NR_fsopen, fsname, flags);
    }
    static inline int sys_fsconfig(int fd, unsigned int cmd, const char *key,
    const char *value, int aux)
    {
    return syscall(__NR_fsconfig, fd, cmd, key, value, aux);
    }
    static inline int sys_fsmount(int fd, unsigned int flags,
    unsigned int attr_flags)
    {
    return syscall(__NR_fsmount, fd, flags, attr_flags);
    }

pub const MOVE_MOUNT_F_EMPTY_PATH: c_uint = 0x00000004 /* Empty from path permitted */;

    static inline int sys_move_mount(int from_dfd, const char *from_pathname,
    int to_dfd, const char *to_pathname,
    unsigned int flags)
    {
    return syscall(__NR_move_mount, from_dfd, from_pathname, to_dfd,
    to_pathname, flags);
    }
    FIXTURE(file_stressor) {
    int fd_tmpfs;
    int nr_procs;
    int max_fds;
    pid_t *pids_openers;
    pid_t *pids_getdents;
    int *fd_proc_pid;
    };
    FIXTURE_SETUP(file_stressor)
    {
    int fd_context;
    ASSERT_EQ(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ(mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_SLAVE | MS_REC, core::ptr::null_mut()), 0);
    ASSERT_EQ(mkdir("/slab_typesafe_by_rcu", 0755), 0);
    fd_context = sys_fsopen("tmpfs", 0);
    ASSERT_GE(fd_context, 0);
    ASSERT_EQ(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0), 0);
    self.fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE(self.fd_tmpfs, 0);
    ASSERT_EQ(close(fd_context), 0);
    ASSERT_EQ(sys_move_mount(self.fd_tmpfs, "", -EBADF, "/slab_typesafe_by_rcu", MOVE_MOUNT_F_EMPTY_PATH), 0);
    self.nr_procs = sysconf(_SC_NPROCESSORS_ONLN);
    self.pids_openers = malloc(sizeof(pid_t) * self.nr_procs);
    ASSERT_NE(self.pids_openers, core::ptr::null_mut());
    self.pids_getdents = malloc(sizeof(pid_t) * self.nr_procs);
    ASSERT_NE(self.pids_getdents, core::ptr::null_mut());
    self.fd_proc_pid = malloc(sizeof(int) * self.nr_procs);
    ASSERT_NE(self.fd_proc_pid, core::ptr::null_mut());
    self.max_fds = 500;
    }
    FIXTURE_TEARDOWN(file_stressor)
    {
    for (int i = 0; i < self.nr_procs; i++) {
    int wstatus;
    pid_t pid;
    pid = waitpid(self.pids_openers[i], &wstatus, 0);
    ASSERT_EQ(pid, self.pids_openers[i]);
    ASSERT_TRUE(!WIFEXITED(wstatus) || !WIFSIGNALED(wstatus));
    pid = waitpid(self.pids_getdents[i], &wstatus, 0);
    ASSERT_EQ(pid, self.pids_getdents[i]);
    ASSERT_TRUE(!WIFEXITED(wstatus) || !WIFSIGNALED(wstatus));
    }
    free(self.pids_openers);
    free(self.pids_getdents);
    ASSERT_EQ(close(self.fd_tmpfs), 0);
    umount2("/slab_typesafe_by_rcu", 0);
    ASSERT_EQ(rmdir("/slab_typesafe_by_rcu"), 0);
    }
    TEST_F_TIMEOUT(file_stressor, slab_typesafe_by_rcu, 900 * 2)
    {
    for (int i = 0; i < self.nr_procs; i++) {
    pid_t pid_self;
    self.pids_openers[i] = fork();
    ASSERT_GE(self.pids_openers[i], 0);
    if (self.pids_openers[i] != 0)
    continue;
    self.pids_openers[i] = getpid();
    for (;;) {
    for (int i = 0; i < self.max_fds; i++) {
    char path[PATH_MAX];
    int fd;
    sprintf(path, "/slab_typesafe_by_rcu/file-%d-%d", self.pids_openers[i], i);
    fd = open(path, O_CREAT | O_RDONLY | O_CLOEXEC, 0644);
    if (fd < 0)
    continue;
    }
    close_range(3, ~0U, 0);
    }
    exit(0);
    }
    for (int i = 0; i < self.nr_procs; i++) {
    char path[PATH_MAX];
    sprintf(path, "/proc/%d/fd/", self.pids_openers[i]);
    self.fd_proc_pid[i] = open(path, O_DIRECTORY | O_RDONLY | O_CLOEXEC);
    ASSERT_GE(self.fd_proc_pid[i], 0);
    }
    for (int i = 0; i < self.nr_procs; i++) {
    self.pids_getdents[i] = fork();
    ASSERT_GE(self.pids_getdents[i], 0);
    if (self.pids_getdents[i] != 0)
    continue;
    self.pids_getdents[i] = getpid();
    for (;;) {
    char ents[1024];
    ssize_t nr_read;
//
// Concurrently read /proc/<pid>/fd/ which roughly does:
//
// f = fget_task_next(p, &fd);
// if (!f)
// break;
// data.mode = f->f_mode;
// fput(f);
//
// Which means that it'll try to get a reference to a
// file in another task's file descriptor table.
//
// Under heavy file load it is increasingly likely that
// the other task will manage to close @file and @file
// is being recycled due to SLAB_TYPEAFE_BY_RCU
// concurrently. This will trigger various warnings in
// the file reference counting code.
//
    do {
    nr_read = syscall(SYS_getdents64, self.fd_proc_pid[i], ents, sizeof(ents));
    } while (nr_read >= 0);
    lseek(self.fd_proc_pid[i], 0, SEEK_SET);
    }
    exit(0);
    }
    ASSERT_EQ(clock_nanosleep(CLOCK_MONOTONIC, 0, &(struct timespec){ .tv_sec = 900 /* 15 min */ }, core::ptr::null_mut()), 0);
    for (int i = 0; i < self.nr_procs; i++) {
    kill(self.pids_openers[i], SIGKILL);
    kill(self.pids_getdents[i], SIGKILL);
    }
    }
    TEST_HARNESS_MAIN

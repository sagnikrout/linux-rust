//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/move_mount_set_group/move_mount_set_group_test.c
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

pub const CLONE_NEWNS: c_uint = 0x00020000;

pub const CLONE_NEWUSER: c_uint = 0x10000000;

pub const MOVE_MOUNT_SET_GROUP: c_uint = 0x00000100;

pub const MOVE_MOUNT_F_EMPTY_PATH: c_uint = 0x00000004;

pub const MOVE_MOUNT_T_EMPTY_PATH: c_uint = 0x00000040;

#[no_mangle]
unsafe extern "C" fn write_nointr(fd: c_int, buf: *const c_void, count: usize) -> isize {
    static ssize_t write_nointr(int fd, const void *buf, size_t count)
    {
    ssize_t ret;
    do {
    ret = write(fd, buf, count);
    } while (ret < 0 && errno == EINTR);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn write_file(path: *const c_char, buf: *const c_void, count: usize) -> c_int {
    static int write_file(const char *path, const void *buf, size_t count)
    {
    int fd;
    ssize_t ret;
    fd = open(path, O_WRONLY | O_CLOEXEC | O_NOCTTY | O_NOFOLLOW);
    if (fd < 0)
    return -1;
    ret = write_nointr(fd, buf, count);
    close(fd);
    if (ret < 0 || (size_t)ret != count)
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn create_and_enter_userns() -> c_int {
    static int create_and_enter_userns(void)
    {
    uid_t uid;
    gid_t gid;
    char map[100];
    uid = getuid();
    gid = getgid();
    if (unshare(CLONE_NEWUSER))
    return -1;
    if (write_file("/proc/self/setgroups", "deny", sizeof("deny") - 1) &&
    errno != ENOENT)
    return -1;
    snprintf(map, sizeof(map), "0 %d 1", uid);
    if (write_file("/proc/self/uid_map", map, strlen(map)))
    return -1;
    snprintf(map, sizeof(map), "0 %d 1", gid);
    if (write_file("/proc/self/gid_map", map, strlen(map)))
    return -1;
    if (setgid(0))
    return -1;
    if (setuid(0))
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn prepare_unpriv_mountns() -> c_int {
    static int prepare_unpriv_mountns(void)
    {
    if (create_and_enter_userns())
    return -1;
    if (unshare(CLONE_NEWNS))
    return -1;
    if (mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_REC | MS_PRIVATE, 0))
    return -1;
    return 0;
    }
    static char *get_field(char *src, int nfields)
    {
    int i;
    char *p = src;
    for (i = 0; i < nfields; i++) {
    while (*p && *p != ' ' && *p != '\t')
    p++;
    if (!*p)
    break;
    p++;
    }
    return p;
    }
#[no_mangle]
unsafe extern "C" fn null_endofword(word: *mut c_char) {
    static void null_endofword(char *word)
    {
    while (*word && *word != ' ' && *word != '\t')
    word++;
// word = '\0';
    }
#[no_mangle]
unsafe extern "C" fn is_shared_mount(path: *const c_char) -> bool {
    static bool is_shared_mount(const char *path)
    {
    let mut len: usize = 0;
    char *line = core::ptr::null_mut();
    FILE *f = core::ptr::null_mut();
    f = fopen("/proc/self/mountinfo", "re");
    if (!f)
    return false;
    while (getline(&line, &len, f) != -1) {
    char *opts, *target;
    target = get_field(line, 4);
    if (!target)
    continue;
    opts = get_field(target, 2);
    if (!opts)
    continue;
    null_endofword(target);
    if (strcmp(target, path) != 0)
    continue;
    null_endofword(opts);
    if (strstr(opts, "shared:"))
    return true;
    }
    free(line);
    fclose(f);
    return false;
    }
// Attempt to de-conflict with the selftests tree.

#[no_mangle]
unsafe extern "C" fn move_mount_set_group_supported() -> bool {
    static bool move_mount_set_group_supported(void)
    {
    int ret;
    if (mount("testing", "/tmp", "tmpfs", MS_NOATIME | MS_NODEV,
    "size=100000,mode=700"))
    return -1;
    if (mount(core::ptr::null_mut(), "/tmp", core::ptr::null_mut(), MS_PRIVATE, 0))
    return -1;
    if (mkdir(SET_GROUP_FROM, 0777))
    return -1;
    if (mkdir(SET_GROUP_TO, 0777))
    return -1;
    if (mount("testing", SET_GROUP_FROM, "tmpfs", MS_NOATIME | MS_NODEV,
    "size=100000,mode=700"))
    return -1;
    if (mount(SET_GROUP_FROM, SET_GROUP_TO, core::ptr::null_mut(), MS_BIND, core::ptr::null_mut()))
    return -1;
    if (mount(core::ptr::null_mut(), SET_GROUP_FROM, core::ptr::null_mut(), MS_SHARED, 0))
    return -1;
    ret = syscall(__NR_move_mount, AT_FDCWD, SET_GROUP_FROM,
    AT_FDCWD, SET_GROUP_TO, MOVE_MOUNT_SET_GROUP);
    umount2("/tmp", MNT_DETACH);
    return ret >= 0;
    }
    FIXTURE(move_mount_set_group) {
    };

    FIXTURE_SETUP(move_mount_set_group)
    {
    bool ret;
    ASSERT_EQ(prepare_unpriv_mountns(), 0);
    ret = move_mount_set_group_supported();
    ASSERT_GE(ret, 0);
    if (!ret)
    SKIP(return, "move_mount(MOVE_MOUNT_SET_GROUP) is not supported");
    umount2("/tmp", MNT_DETACH);
    ASSERT_EQ(mount("testing", "/tmp", "tmpfs", MS_NOATIME | MS_NODEV,
    "size=100000,mode=700"), 0);
    ASSERT_EQ(mkdir(SET_GROUP_A, 0777), 0);
    ASSERT_EQ(mount("testing", SET_GROUP_A, "tmpfs", MS_NOATIME | MS_NODEV,
    "size=100000,mode=700"), 0);
    }
    FIXTURE_TEARDOWN(move_mount_set_group)
    {
    bool ret;
    ret = move_mount_set_group_supported();
    ASSERT_GE(ret, 0);
    if (!ret)
    SKIP(return, "move_mount(MOVE_MOUNT_SET_GROUP) is not supported");
    umount2("/tmp", MNT_DETACH);
    }

#[no_mangle]
unsafe extern "C" fn do_clone(): *mut *mut int (fn)(void, arg: *mut c_void, flags: c_int) -> pid_t {
    static pid_t do_clone(int (*fn)(void *), void *arg, int flags)
    {
    void *stack;
    stack = malloc(__STACK_SIZE);
    if (!stack)
    return -ENOMEM;

    return __clone2(fn, stack, __STACK_SIZE, flags | SIGCHLD, arg, core::ptr::null_mut());

    return clone(fn, stack + __STACK_SIZE, flags | SIGCHLD, arg, core::ptr::null_mut());

    }
#[no_mangle]
unsafe extern "C" fn wait_for_pid(pid: pid_t) -> c_int {
    static int wait_for_pid(pid_t pid)
    {
    int status, ret;
    again:
    ret = waitpid(pid, &status, 0);
    if (ret == -1) {
    if (errno == EINTR)
    goto again;
    return -1;
    }
    if (!WIFEXITED(status))
    return -1;
    return WEXITSTATUS(status);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct child_args {
    pub unsfd: c_int,
    pub mntnsfd: c_int,
    pub shared: bool,
    pub mntfd: c_int,
}

#[no_mangle]
unsafe extern "C" fn get_nestedns_mount_cb(data: *mut c_void) -> c_int {
    static int get_nestedns_mount_cb(void *data)
    {
    struct child_args *ca = (struct child_args *)data;
    int ret;
    ret = prepare_unpriv_mountns();
    if (ret)
    return 1;
    if (ca.shared) {
    ret = mount(core::ptr::null_mut(), SET_GROUP_A, core::ptr::null_mut(), MS_SHARED, 0);
    if (ret)
    return 1;
    }
    ret = open("/proc/self/ns/user", O_RDONLY);
    if (ret < 0)
    return 1;
    ca.unsfd = ret;
    ret = open("/proc/self/ns/mnt", O_RDONLY);
    if (ret < 0)
    return 1;
    ca.mntnsfd = ret;
    ret = open(SET_GROUP_A, O_RDONLY);
    if (ret < 0)
    return 1;
    ca.mntfd = ret;
    return 0;
    }
    TEST_F(move_mount_set_group, complex_sharing_copying)
    {
    struct child_args ca_from = {
    .shared = true,
    };
    struct child_args ca_to = {
    .shared = false,
    };
    pid_t pid;
    bool ret;
    ret = move_mount_set_group_supported();
    ASSERT_GE(ret, 0);
    if (!ret)
    SKIP(return, "move_mount(MOVE_MOUNT_SET_GROUP) is not supported");
    pid = do_clone(get_nestedns_mount_cb, (void *)&ca_from, CLONE_VFORK |
    CLONE_VM | CLONE_FILES); ASSERT_GT(pid, 0);
    ASSERT_EQ(wait_for_pid(pid), 0);
    pid = do_clone(get_nestedns_mount_cb, (void *)&ca_to, CLONE_VFORK |
    CLONE_VM | CLONE_FILES); ASSERT_GT(pid, 0);
    ASSERT_EQ(wait_for_pid(pid), 0);
    ASSERT_EQ(syscall(__NR_move_mount, ca_from.mntfd, "",
    ca_to.mntfd, "", MOVE_MOUNT_SET_GROUP
    | MOVE_MOUNT_F_EMPTY_PATH | MOVE_MOUNT_T_EMPTY_PATH),
    0);
    ASSERT_EQ(setns(ca_to.mntnsfd, CLONE_NEWNS), 0);
    ASSERT_EQ(is_shared_mount(SET_GROUP_A), 1);
    }
    TEST_HARNESS_MAIN

//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/binderfs/binderfs_test.c
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

pub const DEFAULT_THREADS: c_int = 4;

    if (fd >= 0) {              \
    int _e_ = errno;    \
    close(fd);          \
    errno = _e_;        \
    fd = -EBADF;        \
    }
#[no_mangle]
unsafe extern "C" fn change_mountns(_metadata: *mut __test_metadata) {
    static void change_mountns(struct __test_metadata *_metadata)
    {
    int ret;
    ret = unshare(CLONE_NEWNS);
    ASSERT_EQ(ret, 0) {
    TH_LOG("%s - Failed to unshare mount namespace",
    strerror(errno));
    }
    ret = mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_REC | MS_PRIVATE, 0);
    ASSERT_EQ(ret, 0) {
    TH_LOG("%s - Failed to mount / as private",
    strerror(errno));
    }
    }
#[no_mangle]
unsafe extern "C" fn __do_binderfs_test(_metadata: *mut __test_metadata) -> c_int {
    static int __do_binderfs_test(struct __test_metadata *_metadata)
    {
    int fd, ret, saved_errno, result = 1;
    size_t len;
    let mut device: binderfs_device = { 0 };
    let mut version: binder_version = { 0 };
    char binderfs_mntpt[] = P_tmpdir "/binderfs_XXXXXX",
    device_path[sizeof(P_tmpdir "/binderfs_XXXXXX/") + BINDERFS_MAX_NAME];
    static const char * const binder_features[] = {
    "oneway_spam_detection",
    "extended_error",
    "freeze_notification",
    "transaction_report",
    };
    change_mountns(_metadata);
    EXPECT_NE(mkdtemp(binderfs_mntpt), core::ptr::null_mut()) {
    TH_LOG("%s - Failed to create binderfs mountpoint",
    strerror(errno));
    goto out;
    }
    ret = mount(core::ptr::null_mut(), binderfs_mntpt, "binder", 0, 0);
    EXPECT_EQ(ret, 0) {
    if (errno == ENODEV)
    SKIP(goto out, "binderfs missing");
    TH_LOG("%s - Failed to mount binderfs", strerror(errno));
    goto rmdir;
    }
// success: binderfs mounted
    memcpy(device.name, "my-binder", strlen("my-binder"));
    snprintf(device_path, sizeof(device_path), "%s/binder-control", binderfs_mntpt);
    fd = open(device_path, O_RDONLY | O_CLOEXEC);
    EXPECT_GE(fd, 0) {
    TH_LOG("%s - Failed to open binder-control device",
    strerror(errno));
    goto umount;
    }
    ret = ioctl(fd, BINDER_CTL_ADD, &device);
    saved_errno = errno;
    close(fd);
    errno = saved_errno;
    EXPECT_GE(ret, 0) {
    TH_LOG("%s - Failed to allocate new binder device",
    strerror(errno));
    goto umount;
    }
    TH_LOG("Allocated new binder device with major %d, minor %d, and name %s",
    device.major, device.minor, device.name);
// success: binder device allocation
    snprintf(device_path, sizeof(device_path), "%s/my-binder", binderfs_mntpt);
    fd = open(device_path, O_CLOEXEC | O_RDONLY);
    EXPECT_GE(fd, 0) {
    TH_LOG("%s - Failed to open my-binder device",
    strerror(errno));
    goto umount;
    }
    ret = ioctl(fd, BINDER_VERSION, &version);
    saved_errno = errno;
    close(fd);
    errno = saved_errno;
    EXPECT_GE(ret, 0) {
    TH_LOG("%s - Failed to open perform BINDER_VERSION request",
    strerror(errno));
    goto umount;
    }
    TH_LOG("Detected binder version: %d", version.protocol_version);
// success: binder transaction with binderfs binder device
    ret = unlink(device_path);
    EXPECT_EQ(ret, 0) {
    TH_LOG("%s - Failed to delete binder device",
    strerror(errno));
    goto umount;
    }
// success: binder device removal
    snprintf(device_path, sizeof(device_path), "%s/binder-control", binderfs_mntpt);
    ret = unlink(device_path);
    EXPECT_NE(ret, 0) {
    TH_LOG("Managed to delete binder-control device");
    goto umount;
    }
    EXPECT_EQ(errno, EPERM) {
    TH_LOG("%s - Failed to delete binder-control device but exited with unexpected error code",
    strerror(errno));
    goto umount;
    }
// success: binder-control device removal failed as expected
    for (int i = 0; i < ARRAY_SIZE(binder_features); i++) {
    snprintf(device_path, sizeof(device_path), "%s/features/%s",
    binderfs_mntpt, binder_features[i]);
    fd = open(device_path, O_CLOEXEC | O_RDONLY);
    EXPECT_GE(fd, 0) {
    TH_LOG("%s - Failed to open binder feature: %s",
    strerror(errno), binder_features[i]);
    goto umount;
    }
    close(fd);
    }
// success: binder feature files found
    result = 0;
    umount:
    ret = umount2(binderfs_mntpt, MNT_DETACH);
    EXPECT_EQ(ret, 0) {
    TH_LOG("%s - Failed to unmount binderfs", strerror(errno));
    }
    rmdir:
    ret = rmdir(binderfs_mntpt);
    EXPECT_EQ(ret, 0) {
    TH_LOG("%s - Failed to rmdir binderfs mount", strerror(errno));
    }
    out:
    return result;
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
#[no_mangle]
unsafe extern "C" fn setid_userns_root() -> c_int {
    static int setid_userns_root(void)
    {
    if (setuid(0))
    return -1;
    if (setgid(0))
    return -1;
    setfsuid(0);
    setfsgid(0);
    return 0;
    }
    enum idmap_type {
    UID_MAP,
    GID_MAP,
    };
#[no_mangle]
unsafe extern "C" fn read_nointr(fd: c_int, buf: *mut c_void, count: usize) -> isize {
    static ssize_t read_nointr(int fd, void *buf, size_t count)
    {
    ssize_t ret;
    again:
    ret = read(fd, buf, count);
    if (ret < 0 && errno == EINTR)
    goto again;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn write_nointr(fd: c_int, buf: *const c_void, count: usize) -> isize {
    static ssize_t write_nointr(int fd, const void *buf, size_t count)
    {
    ssize_t ret;
    again:
    ret = write(fd, buf, count);
    if (ret < 0 && errno == EINTR)
    goto again;
    return ret;
    }
    static int write_id_mapping(enum idmap_type type, pid_t pid, const char *buf,
    size_t buf_size)
    {
    int fd;
    int ret;
    char path[4096];
    if (type == GID_MAP) {
    int setgroups_fd;
    snprintf(path, sizeof(path), "/proc/%d/setgroups", pid);
    setgroups_fd = open(path, O_WRONLY | O_CLOEXEC | O_NOFOLLOW);
    if (setgroups_fd < 0 && errno != ENOENT)
    return -1;
    if (setgroups_fd >= 0) {
    ret = write_nointr(setgroups_fd, "deny", sizeof("deny") - 1);
    close_prot_errno_disarm(setgroups_fd);
    if (ret != sizeof("deny") - 1)
    return -1;
    }
    }
    switch (type) {
    case UID_MAP:
    ret = snprintf(path, sizeof(path), "/proc/%d/uid_map", pid);
    break;
    case GID_MAP:
    ret = snprintf(path, sizeof(path), "/proc/%d/gid_map", pid);
    break;
    default:
    return -1;
    }
    if (ret < 0 || ret >= sizeof(path))
    return -E2BIG;
    fd = open(path, O_WRONLY | O_CLOEXEC | O_NOFOLLOW);
    if (fd < 0)
    return -1;
    ret = write_nointr(fd, buf, buf_size);
    close_prot_errno_disarm(fd);
    if (ret != buf_size)
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn change_userns(_metadata: *mut __test_metadata, syncfds[2]: c_int) {
    static void change_userns(struct __test_metadata *_metadata, int syncfds[2])
    {
    int ret;
    char buf;
    close_prot_errno_disarm(syncfds[1]);
    ret = unshare(CLONE_NEWUSER);
    ASSERT_EQ(ret, 0) {
    TH_LOG("%s - Failed to unshare user namespace",
    strerror(errno));
    }
    ret = write_nointr(syncfds[0], "1", 1);
    ASSERT_EQ(ret, 1) {
    TH_LOG("write_nointr() failed");
    }
    ret = read_nointr(syncfds[0], &buf, 1);
    ASSERT_EQ(ret, 1) {
    TH_LOG("read_nointr() failed");
    }
    close_prot_errno_disarm(syncfds[0]);
    ASSERT_EQ(setid_userns_root(), 0) {
    TH_LOG("setid_userns_root() failed");
    }
    }
#[no_mangle]
unsafe extern "C" fn change_idmaps(_metadata: *mut __test_metadata, syncfds[2]: c_int, pid: pid_t) {
    static void change_idmaps(struct __test_metadata *_metadata, int syncfds[2], pid_t pid)
    {
    int ret;
    char buf;
    char id_map[4096];
    close_prot_errno_disarm(syncfds[0]);
    ret = read_nointr(syncfds[1], &buf, 1);
    ASSERT_EQ(ret, 1) {
    TH_LOG("read_nointr() failed");
    }
    snprintf(id_map, sizeof(id_map), "0 %d 1\n", getuid());
    ret = write_id_mapping(UID_MAP, pid, id_map, strlen(id_map));
    ASSERT_EQ(ret, 0) {
    TH_LOG("write_id_mapping(UID_MAP) failed");
    }
    snprintf(id_map, sizeof(id_map), "0 %d 1\n", getgid());
    ret = write_id_mapping(GID_MAP, pid, id_map, strlen(id_map));
    ASSERT_EQ(ret, 0) {
    TH_LOG("write_id_mapping(GID_MAP) failed");
    }
    ret = write_nointr(syncfds[1], "1", 1);
    ASSERT_EQ(ret, 1) {
    TH_LOG("write_nointr() failed");
    }
    close_prot_errno_disarm(syncfds[1]);
    }
    struct __test_metadata *_thread_metadata;
    static void *binder_version_thread(void *data)
    {
    struct __test_metadata *_metadata = _thread_metadata;
    let mut fd: c_int = PTR_TO_INT(data);
    let mut version: binder_version = { 0 };
    int ret;
    ret = ioctl(fd, BINDER_VERSION, &version);
    if (ret < 0)
    TH_LOG("%s - Failed to open perform BINDER_VERSION request\n",
    strerror(errno));
    pthread_exit(data);
    }
//
// Regression test:
// 2669b8b0c798 ("binder: prevent UAF for binderfs devices")
// f0fe2c0f050d ("binder: prevent UAF for binderfs devices II")
// 211b64e4b5b6 ("binderfs: use refcount for binder control devices too")
//
    TEST(binderfs_stress)
    {
    int fds[1000];
    int syncfds[2];
    pid_t pid;
    int fd, ret;
    size_t len;
    let mut device: binderfs_device = { 0 };
    char binderfs_mntpt[] = P_tmpdir "/binderfs_XXXXXX",
    device_path[sizeof(P_tmpdir "/binderfs_XXXXXX/") + BINDERFS_MAX_NAME];
    ret = socketpair(PF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, syncfds);
    ASSERT_EQ(ret, 0) {
    TH_LOG("%s - Failed to create socket pair", strerror(errno));
    }
    pid = fork();
    ASSERT_GE(pid, 0) {
    TH_LOG("%s - Failed to fork", strerror(errno));
    close_prot_errno_disarm(syncfds[0]);
    close_prot_errno_disarm(syncfds[1]);
    }
    if (pid == 0) {
    int i, j, k, nthreads;
    pthread_attr_t attr;
    pthread_t threads[DEFAULT_THREADS];
    change_userns(_metadata, syncfds);
    change_mountns(_metadata);
    ASSERT_NE(mkdtemp(binderfs_mntpt), core::ptr::null_mut()) {
    TH_LOG("%s - Failed to create binderfs mountpoint",
    strerror(errno));
    }
    ret = mount(core::ptr::null_mut(), binderfs_mntpt, "binder", 0, 0);
    ASSERT_EQ(ret, 0) {
    TH_LOG("%s - Failed to mount binderfs, check if CONFIG_ANDROID_BINDERFS is enabled in the running kernel",
    strerror(errno));
    }
    for (int i = 0; i < ARRAY_SIZE(fds); i++) {
    snprintf(device_path, sizeof(device_path),
    "%s/binder-control", binderfs_mntpt);
    fd = open(device_path, O_RDONLY | O_CLOEXEC);
    ASSERT_GE(fd, 0) {
    TH_LOG("%s - Failed to open binder-control device",
    strerror(errno));
    }
    memset(&device, 0, sizeof(device));
    snprintf(device.name, sizeof(device.name), "%d", i);
    ret = ioctl(fd, BINDER_CTL_ADD, &device);
    close_prot_errno_disarm(fd);
    ASSERT_EQ(ret, 0) {
    TH_LOG("%s - Failed to allocate new binder device",
    strerror(errno));
    }
    snprintf(device_path, sizeof(device_path), "%s/%d",
    binderfs_mntpt, i);
    fds[i] = open(device_path, O_RDONLY | O_CLOEXEC);
    ASSERT_GE(fds[i], 0) {
    TH_LOG("%s - Failed to open binder device", strerror(errno));
    }
    }
    ret = umount2(binderfs_mntpt, MNT_DETACH);
    ASSERT_EQ(ret, 0) {
    TH_LOG("%s - Failed to unmount binderfs", strerror(errno));
    rmdir(binderfs_mntpt);
    }
    nthreads = get_nprocs_conf();
    if (nthreads > DEFAULT_THREADS)
    nthreads = DEFAULT_THREADS;
    _thread_metadata = _metadata;
    pthread_attr_init(&attr);
    for (k = 0; k < ARRAY_SIZE(fds); k++) {
    for (i = 0; i < nthreads; i++) {
    ret = pthread_create(&threads[i], &attr, binder_version_thread, INT_TO_PTR(fds[k]));
    if (ret) {
    TH_LOG("%s - Failed to create thread %d",
    strerror(errno), i);
    break;
    }
    }
    for (j = 0; j < i; j++) {
    void *fdptr = core::ptr::null_mut();
    ret = pthread_join(threads[j], &fdptr);
    if (ret)
    TH_LOG("%s - Failed to join thread %d for fd %d",
    strerror(errno), j, PTR_TO_INT(fdptr));
    }
    }
    pthread_attr_destroy(&attr);
    for (k = 0; k < ARRAY_SIZE(fds); k++)
    close(fds[k]);
    exit(EXIT_SUCCESS);
    }
    change_idmaps(_metadata, syncfds, pid);
    ret = wait_for_pid(pid);
    ASSERT_EQ(ret, 0) {
    TH_LOG("wait_for_pid() failed");
    }
    }
    TEST(binderfs_test_privileged)
    {
    if (geteuid() != 0)
    SKIP(return, "Tests are not run as root. Skipping privileged tests");
    if (__do_binderfs_test(_metadata))
    SKIP(return, "The Android binderfs filesystem is not available");
    }
    TEST(binderfs_test_unprivileged)
    {
    int ret;
    int syncfds[2];
    pid_t pid;
    ret = socketpair(PF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, syncfds);
    ASSERT_EQ(ret, 0) {
    TH_LOG("%s - Failed to create socket pair", strerror(errno));
    }
    pid = fork();
    ASSERT_GE(pid, 0) {
    close_prot_errno_disarm(syncfds[0]);
    close_prot_errno_disarm(syncfds[1]);
    TH_LOG("%s - Failed to fork", strerror(errno));
    }
    if (pid == 0) {
    change_userns(_metadata, syncfds);
    if (__do_binderfs_test(_metadata))
    exit(2);
    exit(EXIT_SUCCESS);
    }
    change_idmaps(_metadata, syncfds, pid);
    ret = wait_for_pid(pid);
    if (ret) {
    if (ret == 2)
    SKIP(return, "The Android binderfs filesystem is not available");
    ASSERT_EQ(ret, 0) {
    TH_LOG("wait_for_pid() failed");
    }
    }
    }
    TEST_HARNESS_MAIN

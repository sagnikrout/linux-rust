//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/cgroup/lib/cgroup_util.c
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

    bool cg_test_v1_named;
// Returns read len on success, or -errno on failure.
#[no_mangle]
pub unsafe extern "C" fn read_text(path: *const c_char, buf: *mut c_char, max_len: usize) -> isize {
    ssize_t read_text(const char *path, char *buf, size_t max_len)
    {
    ssize_t len;
    int fd;
    fd = open(path, O_RDONLY);
    if (fd < 0)
    return -errno;
    len = read(fd, buf, max_len - 1);
    if (len >= 0)
    buf[len] = 0;
    close(fd);
    return len < 0 ? -errno : len;
    }
// Returns written len on success, or -errno on failure.
#[no_mangle]
pub unsafe extern "C" fn write_text(path: *const c_char, buf: *mut c_char, len: isize) -> isize {
    ssize_t write_text(const char *path, char *buf, ssize_t len)
    {
    int fd;
    fd = open(path, O_WRONLY | O_APPEND);
    if (fd < 0)
    return -errno;
    len = write(fd, buf, len);
    close(fd);
    return len < 0 ? -errno : len;
    }
    char *cg_name(const char *root, const char *name)
    {
    let mut len: usize = strlen(root) + strlen(name) + 2;
    char *ret = malloc(len);
    if (ret)
    snprintf(ret, len, "%s/%s", root, name);
    return ret;
    }
    char *cg_name_indexed(const char *root, const char *name, int index)
    {
    let mut len: usize = strlen(root) + strlen(name) + 10;
    char *ret = malloc(len);
    if (ret)
    snprintf(ret, len, "%s/%s_%d", root, name, index);
    return ret;
    }
    char *cg_control(const char *cgroup, const char *control)
    {
    let mut len: usize = strlen(cgroup) + strlen(control) + 2;
    char *ret = malloc(len);
    if (ret)
    snprintf(ret, len, "%s/%s", cgroup, control);
    return ret;
    }
// Returns 0 on success, or -errno on failure.
#[no_mangle]
pub unsafe extern "C" fn cg_read(cgroup: *const c_char, control: *const c_char, buf: *mut c_char, len: usize) -> c_int {
    int cg_read(const char *cgroup, const char *control, char *buf, size_t len)
    {
    char path[PATH_MAX];
    ssize_t ret;
    snprintf(path, sizeof(path), "%s/%s", cgroup, control);
    ret = read_text(path, buf, len);
    return ret >= 0 ? 0 : ret;
    }
    int cg_read_strcmp(const char *cgroup, const char *control,
    const char *expected)
    {
    size_t size;
    char *buf;
    int ret;
// Handle the case of comparing against empty string
    if (!expected)
    return -1;
// needs size > 1, otherwise cg_read() reads 0 bytes
    size = (expected[0] == '\0') ? 2 : strlen(expected) + 1;
    buf = malloc(size);
    if (!buf)
    return -1;
    if (cg_read(cgroup, control, buf, size)) {
    free(buf);
    return -1;
    }
    ret = strcmp(expected, buf);
    free(buf);
    return ret;
    }
    int cg_read_strcmp_wait(const char *cgroup, const char *control,
    const char *expected)
    {
    int i, ret;
    for (i = 0; i < 100; i++) {
    ret = cg_read_strcmp(cgroup, control, expected);
    if (!ret)
    return ret;
    usleep(10000);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cg_read_strstr(cgroup: *const c_char, control: *const c_char, needle: *const c_char) -> c_int {
    int cg_read_strstr(const char *cgroup, const char *control, const char *needle)
    {
    char buf[BUF_SIZE];
    if (cg_read(cgroup, control, buf, sizeof(buf)))
    return -1;
    return strstr(buf, needle) ? 0 : -1;
    }
#[no_mangle]
pub unsafe extern "C" fn cg_read_long(cgroup: *const c_char, control: *const c_char) -> c_long {
    long cg_read_long(const char *cgroup, const char *control)
    {
    char buf[128];
    if (cg_read(cgroup, control, buf, sizeof(buf)))
    return -1;
    return atol(buf);
    }
#[no_mangle]
pub unsafe extern "C" fn cg_read_long_fd(fd: c_int) -> c_long {
    long cg_read_long_fd(int fd)
    {
    char buf[128];
    if (pread(fd, buf, sizeof(buf), 0) <= 0)
    return -1;
    return atol(buf);
    }
#[no_mangle]
pub unsafe extern "C" fn cg_read_key_long(cgroup: *const c_char, control: *const c_char, key: *const c_char) -> c_long {
    long cg_read_key_long(const char *cgroup, const char *control, const char *key)
    {
    char buf[BUF_SIZE];
    char *ptr;
    if (cg_read(cgroup, control, buf, sizeof(buf)))
    return -1;
    ptr = strstr(buf, key);
    if (!ptr)
    return -1;
    return atol(ptr + strlen(key));
    }
    long cg_read_key_long_poll(const char *cgroup, const char *control,
    const char *key, long expected, int retries,
    useconds_t wait_interval_us)
    {
    let mut val: c_long = -1;
    int i;
    for (i = 0; i < retries; i++) {
    val = cg_read_key_long(cgroup, control, key);
    if (val < 0)
    return val;
    if (val == expected)
    break;
    usleep(wait_interval_us);
    }
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn cg_read_lc(cgroup: *const c_char, control: *const c_char) -> c_long {
    long cg_read_lc(const char *cgroup, const char *control)
    {
    char buf[BUF_SIZE];
    const char delim[] = "\n";
    char *line;
    let mut cnt: c_long = 0;
    if (cg_read(cgroup, control, buf, sizeof(buf)))
    return -1;
    for (line = strtok(buf, delim); line; line = strtok(core::ptr::null_mut(), delim))
    cnt++;
    return cnt;
    }
// Returns 0 on success, or -errno on failure.
#[no_mangle]
pub unsafe extern "C" fn cg_write(cgroup: *const c_char, control: *const c_char, buf: *mut c_char) -> c_int {
    int cg_write(const char *cgroup, const char *control, char *buf)
    {
    char path[PATH_MAX];
    let mut len: isize = strlen(buf), ret;
    snprintf(path, sizeof(path), "%s/%s", cgroup, control);
    ret = write_text(path, buf, len);
    let mut ret: return = = len ? 0 : ret;
    }
//
// Returns fd on success, or -1 on failure.
// (fd should be closed with close() as usual)
//
#[no_mangle]
pub unsafe extern "C" fn cg_open(cgroup: *const c_char, control: *const c_char, flags: c_int) -> c_int {
    int cg_open(const char *cgroup, const char *control, int flags)
    {
    char path[PATH_MAX];
    snprintf(path, sizeof(path), "%s/%s", cgroup, control);
    return open(path, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn cg_write_numeric(cgroup: *const c_char, control: *const c_char, value: c_long) -> c_int {
    int cg_write_numeric(const char *cgroup, const char *control, long value)
    {
    char buf[64];
    int ret;
    ret = sprintf(buf, "%lu", value);
    if (ret < 0)
    return ret;
    return cg_write(cgroup, control, buf);
    }
    static int cg_find_root(char *root, size_t len, const char *controller,
    bool *nsdelegate)
    {
    char buf[10 * BUF_SIZE];
    char *fs, *mount, *type, *options;
    const char delim[] = "\n\t ";
    if (read_text("/proc/self/mounts", buf, sizeof(buf)) <= 0)
    return -1;
//
// Example:
// cgroup /sys/fs/cgroup cgroup2 rw,seclabel,noexec,relatime 0 0
//
    for (fs = strtok(buf, delim); fs; fs = strtok(core::ptr::null_mut(), delim)) {
    mount = strtok(core::ptr::null_mut(), delim);
    type = strtok(core::ptr::null_mut(), delim);
    options = strtok(core::ptr::null_mut(), delim);
    strtok(core::ptr::null_mut(), delim);
    strtok(core::ptr::null_mut(), delim);
    if (strcmp(type, "cgroup") == 0) {
    if (!controller || !strstr(options, controller))
    continue;
    } else if (strcmp(type, "cgroup2") == 0) {
    if (controller &&
    cg_read_strstr(mount, "cgroup.controllers", controller))
    continue;
    } else {
    continue;
    }
    strncpy(root, mount, len);
    if (nsdelegate)
// nsdelegate = !!strstr(options, "nsdelegate");
    return 0;
    }
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn cg_find_controller_root(root: *mut c_char, len: usize, controller: *const c_char) -> c_int {
    int cg_find_controller_root(char *root, size_t len, const char *controller)
    {
    return cg_find_root(root, len, controller, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn cg_find_unified_root(root: *mut c_char, len: usize, nsdelegate: *mut bool) -> c_int {
    int cg_find_unified_root(char *root, size_t len, bool *nsdelegate)
    {
    return cg_find_root(root, len, core::ptr::null_mut(), nsdelegate);
    }
#[no_mangle]
pub unsafe extern "C" fn cg_create(cgroup: *const c_char) -> c_int {
    int cg_create(const char *cgroup)
    {
    return mkdir(cgroup, 0755);
    }
#[no_mangle]
pub unsafe extern "C" fn cg_wait_for_proc_count(cgroup: *const c_char, count: c_int) -> c_int {
    int cg_wait_for_proc_count(const char *cgroup, int count)
    {
    char buf[10 * BUF_SIZE] = {0};
    int attempts;
    char *ptr;
    for (attempts = 10; attempts >= 0; attempts--) {
    let mut nr: c_int = 0;
    if (cg_read(cgroup, "cgroup.procs", buf, sizeof(buf)))
    break;
    for (ptr = buf; *ptr; ptr++)
    if (*ptr == '\n')
    nr++;
    if (nr >= count)
    return 0;
    usleep(100000);
    }
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn cg_killall(cgroup: *const c_char) -> c_int {
    int cg_killall(const char *cgroup)
    {
    char buf[BUF_SIZE];
    char *ptr = buf;
// If cgroup.kill exists use it.
    if (!cg_write(cgroup, "cgroup.kill", "1"))
    return 0;
    if (cg_read(cgroup, "cgroup.procs", buf, sizeof(buf)))
    return -1;
    while (ptr < buf + sizeof(buf)) {
    let mut pid: c_int = strtol(ptr, &ptr, 10);
    if (pid == 0)
    break;
    if (*ptr)
    ptr++;
    else
    break;
    if (kill(pid, SIGKILL))
    return -1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cg_destroy(cgroup: *const c_char) -> c_int {
    int cg_destroy(const char *cgroup)
    {
    int ret;
    if (!cgroup)
    return 0;
    retry:
    ret = rmdir(cgroup);
    if (ret && errno == EBUSY) {
    cg_killall(cgroup);
    usleep(100);
    goto retry;
    }
    if (ret && errno == ENOENT)
    ret = 0;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cg_enter(cgroup: *const c_char, pid: c_int) -> c_int {
    int cg_enter(const char *cgroup, int pid)
    {
    char pidbuf[64];
    snprintf(pidbuf, sizeof(pidbuf), "%d", pid);
    return cg_write(cgroup, "cgroup.procs", pidbuf);
    }
#[no_mangle]
pub unsafe extern "C" fn cg_enter_current(cgroup: *const c_char) -> c_int {
    int cg_enter_current(const char *cgroup)
    {
    return cg_write(cgroup, "cgroup.procs", "0");
    }
#[no_mangle]
pub unsafe extern "C" fn cg_enter_current_thread(cgroup: *const c_char) -> c_int {
    int cg_enter_current_thread(const char *cgroup)
    {
    return cg_write(cgroup, CG_THREADS_FILE, "0");
    }
    int cg_run(const char *cgroup,
    int (*fn)(const char *cgroup, void *arg),
    void *arg)
    {
    int pid, retcode;
    pid = fork();
    if (pid < 0) {
    return pid;
    } else if (pid == 0) {
    char buf[64];
    snprintf(buf, sizeof(buf), "%d", getpid());
    if (cg_write(cgroup, "cgroup.procs", buf))
    exit(EXIT_FAILURE);
    exit(fn(cgroup, arg));
    } else {
    waitpid(pid, &retcode, 0);
    if (WIFEXITED(retcode))
    return WEXITSTATUS(retcode);
    else
    return -1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn clone_into_cgroup(cgroup_fd: c_int) -> pid_t {
    pid_t clone_into_cgroup(int cgroup_fd)
    {

    pid_t pid;
    struct __clone_args args = {
    .flags = CLONE_INTO_CGROUP,
    .exit_signal = SIGCHLD,
    .cgroup = cgroup_fd,
    };
    pid = sys_clone3(&args, sizeof(struct __clone_args));
//
// Verify that this is a genuine test failure:
// ENOSYS -> clone3() not available
// E2BIG  -> CLONE_INTO_CGROUP not available
//
    if (pid < 0 && (errno == ENOSYS || errno == E2BIG))
    goto pretend_enosys;
    return pid;
    pretend_enosys:

    errno = ENOSYS;
    return -ENOSYS;
    }
#[no_mangle]
pub unsafe extern "C" fn clone_reap(pid: pid_t, options: c_int) -> c_int {
    int clone_reap(pid_t pid, int options)
    {
    int ret;
    siginfo_t info = {
    .si_signo = 0,
    };
    again:
    ret = waitid(P_PID, pid, &info, options | __WALL | __WNOTHREAD);
    if (ret < 0) {
    if (errno == EINTR)
    goto again;
    return -1;
    }
    if (options & WEXITED) {
    if (WIFEXITED(info.si_status))
    return WEXITSTATUS(info.si_status);
    }
    if (options & WSTOPPED) {
    if (WIFSTOPPED(info.si_status))
    return WSTOPSIG(info.si_status);
    }
    if (options & WCONTINUED) {
    if (WIFCONTINUED(info.si_status))
    return 0;
    }
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn dirfd_open_opath(dir: *const c_char) -> c_int {
    int dirfd_open_opath(const char *dir)
    {
    return open(dir, O_DIRECTORY | O_CLOEXEC | O_NOFOLLOW | O_PATH);
    }

    if (fd >= 0) {                                                         \
    int _e_ = errno;                                               \
    close(fd);                                                     \
    errno = _e_;                                                   \
    }
    static int clone_into_cgroup_run_nowait(const char *cgroup,
    int (*fn)(const char *cgroup, void *arg),
    void *arg)
    {
    int cgroup_fd;
    pid_t pid;
    cgroup_fd =  dirfd_open_opath(cgroup);
    if (cgroup_fd < 0)
    return -1;
    pid = clone_into_cgroup(cgroup_fd);
    close_prot_errno(cgroup_fd);
    if (pid == 0)
    exit(fn(cgroup, arg));
    return pid;
    }
    int cg_run_nowait(const char *cgroup,
    int (*fn)(const char *cgroup, void *arg),
    void *arg)
    {
    int pid;
    pid = clone_into_cgroup_run_nowait(cgroup, fn, arg);
    if (pid > 0)
    return pid;
// Genuine test failure.
    if (pid < 0 && errno != ENOSYS)
    return -1;
    pid = fork();
    if (pid == 0) {
    char buf[64];
    snprintf(buf, sizeof(buf), "%d", getpid());
    if (cg_write(cgroup, "cgroup.procs", buf))
    exit(EXIT_FAILURE);
    exit(fn(cgroup, arg));
    }
    return pid;
    }
#[no_mangle]
pub unsafe extern "C" fn proc_mount_contains(option: *const c_char) -> c_int {
    int proc_mount_contains(const char *option)
    {
    char buf[4 * BUF_SIZE];
    ssize_t read;
    read = read_text("/proc/mounts", buf, sizeof(buf));
    if (read < 0)
    return read;
    return strstr(buf, option) != core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn cgroup_feature(feature: *const c_char) -> c_int {
    int cgroup_feature(const char *feature)
    {
    char buf[BUF_SIZE];
    ssize_t read;
    read = read_text("/sys/kernel/cgroup/features", buf, sizeof(buf));
    if (read < 0)
    return read;
    return strstr(buf, feature) != core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn proc_read_text(pid: c_int, thread: bool, item: *const c_char, buf: *mut c_char, size: usize) -> isize {
    ssize_t proc_read_text(int pid, bool thread, const char *item, char *buf, size_t size)
    {
    char path[PATH_MAX];
    ssize_t ret;
    if (!pid)
    snprintf(path, sizeof(path), "/proc/%s/%s",
    thread ? "thread-self" : "self", item);
    else
    snprintf(path, sizeof(path), "/proc/%d/%s", pid, item);
    ret = read_text(path, buf, size);
    return ret < 0 ? -1 : ret;
    }
#[no_mangle]
pub unsafe extern "C" fn proc_read_strstr(pid: c_int, thread: bool, item: *const c_char, needle: *const c_char) -> c_int {
    int proc_read_strstr(int pid, bool thread, const char *item, const char *needle)
    {
    char buf[BUF_SIZE];
    if (proc_read_text(pid, thread, item, buf, sizeof(buf)) < 0)
    return -1;
    return strstr(buf, needle) ? 0 : -1;
    }
#[no_mangle]
pub unsafe extern "C" fn clone_into_cgroup_run_wait(cgroup: *const c_char) -> c_int {
    int clone_into_cgroup_run_wait(const char *cgroup)
    {
    int cgroup_fd;
    pid_t pid;
    cgroup_fd =  dirfd_open_opath(cgroup);
    if (cgroup_fd < 0)
    return -1;
    pid = clone_into_cgroup(cgroup_fd);
    close_prot_errno(cgroup_fd);
    if (pid < 0)
    return -1;
    if (pid == 0)
    exit(EXIT_SUCCESS);
//
// We don't care whether this fails. We only care whether the initial
// clone succeeded.
//
    (void)clone_reap(pid, WEXITED);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __prepare_for_wait(cgroup: *const c_char, filename: *const c_char) -> c_int {
    static int __prepare_for_wait(const char *cgroup, const char *filename)
    {
    int fd, ret = -1;
    fd = inotify_init1(0);
    if (fd == -1)
    return fd;
    ret = inotify_add_watch(fd, cg_control(cgroup, filename), IN_MODIFY);
    if (ret == -1) {
    close(fd);
    fd = -1;
    }
    return fd;
    }
#[no_mangle]
pub unsafe extern "C" fn cg_prepare_for_wait(cgroup: *const c_char) -> c_int {
    int cg_prepare_for_wait(const char *cgroup)
    {
    return __prepare_for_wait(cgroup, "cgroup.events");
    }
#[no_mangle]
pub unsafe extern "C" fn memcg_prepare_for_wait(cgroup: *const c_char) -> c_int {
    int memcg_prepare_for_wait(const char *cgroup)
    {
    return __prepare_for_wait(cgroup, "memory.events");
    }
#[no_mangle]
pub unsafe extern "C" fn cg_wait_for(fd: c_int) -> c_int {
    int cg_wait_for(int fd)
    {
    let mut ret: c_int = -1;
    struct pollfd fds = {
    .fd = fd,
    .events = POLLIN,
    };
    while (true) {
    ret = poll(&fds, 1, 10000);
    if (ret == -1) {
    if (errno == EINTR)
    continue;
    break;
    }
    if (ret > 0 && fds.revents & POLLIN) {
    ret = 0;
    break;
    }
    }
    return ret;
    }

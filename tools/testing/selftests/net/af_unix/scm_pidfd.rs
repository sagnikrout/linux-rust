//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/af_unix/scm_pidfd.c
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
// Macro flag: #define _GNU_SOURCE

    fprintf(stderr, "(%s:%d: errno: %s) " MSG "\n", __FILE__, __LINE__, \
    clean_errno(), ##__VA_ARGS__)

pub const SCM_PIDFD: c_uint = 0x04;

pub const CHILD_EXIT_CODE_OK: c_int = 123;
#[no_mangle]
unsafe extern "C" fn child_die() {
    static void child_die()
    {
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn safe_int(numstr: *const c_char, converted: *mut c_int) -> c_int {
    static int safe_int(const char *numstr, int *converted)
    {
    char *err = core::ptr::null_mut();
    long sli;
    errno = 0;
    sli = strtol(numstr, &err, 0);
    if (errno == ERANGE && (sli == LONG_MAX || sli == LONG_MIN))
    return -ERANGE;
    if (errno != 0 && sli == 0)
    return -EINVAL;
    if (err == numstr || *err != '\0')
    return -EINVAL;
    if (sli > INT_MAX || sli < INT_MIN)
    return -ERANGE;
// converted = (int)sli;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn char_left_gc(buffer: *const c_char, len: usize) -> c_int {
    static int char_left_gc(const char *buffer, size_t len)
    {
    size_t i;
    for (i = 0; i < len; i++) {
    if (buffer[i] == ' ' || buffer[i] == '\t')
    continue;
    return i;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn char_right_gc(buffer: *const c_char, len: usize) -> c_int {
    static int char_right_gc(const char *buffer, size_t len)
    {
    int i;
    for (i = len - 1; i >= 0; i--) {
    if (buffer[i] == ' ' || buffer[i] == '\t' ||
    buffer[i] == '\n' || buffer[i] == '\0')
    continue;
    return i + 1;
    }
    return 0;
    }
    static char *trim_whitespace_in_place(char *buffer)
    {
    buffer += char_left_gc(buffer, strlen(buffer));
    buffer[char_right_gc(buffer, strlen(buffer))] = '\0';
    return buffer;
    }
// borrowed (with all helpers) from pidfd/pidfd_open_test.c
#[no_mangle]
unsafe extern "C" fn get_pid_from_fdinfo_file(pidfd: c_int, key: *const c_char, keylen: usize) -> pid_t {
    static pid_t get_pid_from_fdinfo_file(int pidfd, const char *key, size_t keylen)
    {
    int ret;
    char path[512];
    FILE *f;
    let mut n: usize = 0;
    let mut result: pid_t = -1;
    char *line = core::ptr::null_mut();
    snprintf(path, sizeof(path), "/proc/self/fdinfo/%d", pidfd);
    f = fopen(path, "re");
    if (!f)
    return -1;
    while (getline(&line, &n, f) != -1) {
    char *numstr;
    if (strncmp(line, key, keylen))
    continue;
    numstr = trim_whitespace_in_place(line + 4);
    ret = safe_int(numstr, &result);
    if (ret < 0)
    goto out;
    break;
    }
    out:
    free(line);
    fclose(f);
    return result;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmsg_data {
    pub ucred: *mut ucred,
    pub pidfd: *mut c_int,
}

#[no_mangle]
unsafe extern "C" fn parse_cmsg(msg: *mut msghdr, res: *mut cmsg_data) -> c_int {
    static int parse_cmsg(struct msghdr *msg, struct cmsg_data *res)
    {
    struct cmsghdr *cmsg;
    if (msg.msg_flags & (MSG_TRUNC | MSG_CTRUNC)) {
    log_err("recvmsg: truncated");
    return 1;
    }
    for (cmsg = CMSG_FIRSTHDR(msg); cmsg != core::ptr::null_mut();
    cmsg = CMSG_NXTHDR(msg, cmsg)) {
    if (cmsg.cmsg_level == SOL_SOCKET &&
    cmsg.cmsg_type == SCM_PIDFD) {
    if (cmsg.cmsg_len < sizeof(*res.pidfd)) {
    log_err("CMSG parse: SCM_PIDFD wrong len");
    return 1;
    }
    res.pidfd = (void *)CMSG_DATA(cmsg);
    }
    if (cmsg.cmsg_level == SOL_SOCKET &&
    cmsg.cmsg_type == SCM_CREDENTIALS) {
    if (cmsg.cmsg_len < sizeof(*res.ucred)) {
    log_err("CMSG parse: SCM_CREDENTIALS wrong len");
    return 1;
    }
    res.ucred = (void *)CMSG_DATA(cmsg);
    }
    }
    if (!res.pidfd) {
    log_err("CMSG parse: SCM_PIDFD not found");
    return 1;
    }
    if (!res.ucred) {
    log_err("CMSG parse: SCM_CREDENTIALS not found");
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmsg_check(fd: c_int) -> c_int {
    static int cmsg_check(int fd)
    {
    let mut msg: msghdr = { 0 };
    struct cmsg_data res;
    struct iovec iov;
    let mut data: c_int = 0;
    char control[CMSG_SPACE(sizeof(struct ucred)) +
    CMSG_SPACE(sizeof(int))] = { 0 };
    pid_t parent_pid;
    int err;
    iov.iov_base = &data;
    iov.iov_len = sizeof(data);
    msg.msg_iov = &iov;
    msg.msg_iovlen = 1;
    msg.msg_control = control;
    msg.msg_controllen = sizeof(control);
    err = recvmsg(fd, &msg, 0);
    if (err < 0) {
    log_err("recvmsg");
    return 1;
    }
    if (msg.msg_flags & (MSG_TRUNC | MSG_CTRUNC)) {
    log_err("recvmsg: truncated");
    return 1;
    }
// send(pfd, "x", sizeof(char), 0)
    if (data != 'x') {
    log_err("recvmsg: data corruption");
    return 1;
    }
    if (parse_cmsg(&msg, &res)) {
    log_err("CMSG parse: parse_cmsg() failed");
    return 1;
    }
// pidfd from SCM_PIDFD should point to the parent process PID
    parent_pid =
    get_pid_from_fdinfo_file(*res.pidfd, "Pid:", sizeof("Pid:") - 1);
    if (parent_pid != getppid()) {
    log_err("wrong SCM_PIDFD %d != %d", parent_pid, getppid());
    close(*res.pidfd);
    return 1;
    }
    close(*res.pidfd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmsg_check_dead(fd: c_int, expected_pid: c_int) -> c_int {
    static int cmsg_check_dead(int fd, int expected_pid)
    {
    int err;
    let mut msg: msghdr = { 0 };
    struct cmsg_data res;
    struct iovec iov;
    let mut data: c_int = 0;
    char control[CMSG_SPACE(sizeof(struct ucred)) +
    CMSG_SPACE(sizeof(int))] = { 0 };
    struct pidfd_info info = {
    .mask = PIDFD_INFO_EXIT,
    };
    iov.iov_base = &data;
    iov.iov_len = sizeof(data);
    msg.msg_iov = &iov;
    msg.msg_iovlen = 1;
    msg.msg_control = control;
    msg.msg_controllen = sizeof(control);
    err = recvmsg(fd, &msg, 0);
    if (err < 0) {
    log_err("recvmsg");
    return 1;
    }
    if (msg.msg_flags & (MSG_TRUNC | MSG_CTRUNC)) {
    log_err("recvmsg: truncated");
    return 1;
    }
// send(cfd, "y", sizeof(char), 0)
    if (data != 'y') {
    log_err("recvmsg: data corruption");
    return 1;
    }
    if (parse_cmsg(&msg, &res)) {
    log_err("CMSG parse: parse_cmsg() failed");
    return 1;
    }
//
// pidfd from SCM_PIDFD should point to the client_pid.
// Let's read exit information and check if it's what
// we expect to see.
//
    if (ioctl(*res.pidfd, PIDFD_GET_INFO, &info)) {
    log_err("%s: ioctl(PIDFD_GET_INFO) failed", __func__);
    close(*res.pidfd);
    return 1;
    }
    if (!(info.mask & PIDFD_INFO_EXIT)) {
    log_err("%s: No exit information from ioctl(PIDFD_GET_INFO)", __func__);
    close(*res.pidfd);
    return 1;
    }
    err = WIFEXITED(info.exit_code) ? WEXITSTATUS(info.exit_code) : 1;
    if (err != CHILD_EXIT_CODE_OK) {
    log_err("%s: wrong exit_code %d != %d", __func__, err, CHILD_EXIT_CODE_OK);
    close(*res.pidfd);
    return 1;
    }
    close(*res.pidfd);
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_addr {
    pub sock_name: [c_char; 32],
    pub listen_addr: sockaddr_un,
    pub addrlen: socklen_t,
}

    FIXTURE(scm_pidfd)
    {
    int server;
    pid_t client_pid;
    int startup_pipe[2];
    struct sock_addr server_addr;
    struct sock_addr *client_addr;
    };
    FIXTURE_VARIANT(scm_pidfd)
    {
    int type;
    bool abstract;
    };
    FIXTURE_VARIANT_ADD(scm_pidfd, stream_pathname)
    {
    .type = SOCK_STREAM,
    .abstract = 0,
    };
    FIXTURE_VARIANT_ADD(scm_pidfd, stream_abstract)
    {
    .type = SOCK_STREAM,
    .abstract = 1,
    };
    FIXTURE_VARIANT_ADD(scm_pidfd, dgram_pathname)
    {
    .type = SOCK_DGRAM,
    .abstract = 0,
    };
    FIXTURE_VARIANT_ADD(scm_pidfd, dgram_abstract)
    {
    .type = SOCK_DGRAM,
    .abstract = 1,
    };
    FIXTURE_SETUP(scm_pidfd)
    {
    self.client_addr = mmap(core::ptr::null_mut(), sizeof(*self.client_addr), PROT_READ | PROT_WRITE,
    MAP_SHARED | MAP_ANONYMOUS, -1, 0);
    ASSERT_NE(MAP_FAILED, self.client_addr);
    }
    FIXTURE_TEARDOWN(scm_pidfd)
    {
    close(self.server);
    kill(self.client_pid, SIGKILL);
    waitpid(self.client_pid, core::ptr::null_mut(), 0);
    if (!variant.abstract) {
    unlink(self.server_addr.sock_name);
    unlink(self.client_addr.sock_name);
    }
    }
#[no_mangle]
unsafe extern "C" fn fill_sockaddr(addr: *mut sock_addr, abstract: bool) {
    static void fill_sockaddr(struct sock_addr *addr, bool abstract)
    {
    char *sun_path_buf = (char *)&addr.listen_addr.sun_path;
    addr.listen_addr.sun_family = AF_UNIX;
    addr.addrlen = offsetof(struct sockaddr_un, sun_path);
    snprintf(addr.sock_name, sizeof(addr.sock_name), "scm_pidfd_%d", getpid());
    addr.addrlen += strlen(addr.sock_name);
    if (abstract) {
// sun_path_buf = '\0';
    addr.addrlen++;
    sun_path_buf++;
    } else {
    unlink(addr.sock_name);
    }
    memcpy(sun_path_buf, addr.sock_name, strlen(addr.sock_name));
    }
#[no_mangle]
unsafe extern "C" fn sk_enable_cred_pass(sk: c_int) -> c_int {
    static int sk_enable_cred_pass(int sk)
    {
    let mut on: c_int = 0;
    on = 1;
    if (setsockopt(sk, SOL_SOCKET, SO_PASSCRED, &on, sizeof(on))) {
    log_err("Failed to set SO_PASSCRED");
    return 1;
    }
    if (setsockopt(sk, SOL_SOCKET, SO_PASSPIDFD, &on, sizeof(on))) {
    log_err("Failed to set SO_PASSPIDFD");
    return 1;
    }
    return 0;
    }
    static void client(FIXTURE_DATA(scm_pidfd) *self,
#[no_mangle]
pub unsafe extern "C" fn FIXTURE_VARIANT(variant: *mut scm_pidfd)) -> const {
    const FIXTURE_VARIANT(scm_pidfd) *variant)
    {
    int cfd;
    socklen_t len;
    struct ucred peer_cred;
    int peer_pidfd;
    pid_t peer_pid;
    cfd = socket(AF_UNIX, variant.type, 0);
    if (cfd < 0) {
    log_err("socket");
    child_die();
    }
    if (variant.type == SOCK_DGRAM) {
    fill_sockaddr(self.client_addr, variant.abstract);
    if (bind(cfd, (struct sockaddr *)&self.client_addr.listen_addr, self.client_addr.addrlen)) {
    log_err("bind");
    child_die();
    }
    }
    if (connect(cfd, (struct sockaddr *)&self.server_addr.listen_addr,
    self.server_addr.addrlen) != 0) {
    log_err("connect");
    child_die();
    }
    if (sk_enable_cred_pass(cfd)) {
    log_err("sk_enable_cred_pass() failed");
    child_die();
    }
    close(self.startup_pipe[1]);
    if (cmsg_check(cfd)) {
    log_err("cmsg_check failed");
    child_die();
    }
// send something to the parent so it can receive SCM_PIDFD too and validate it
    if (send(cfd, "y", sizeof(char), 0) == -1) {
    log_err("Failed to send(cfd, \"y\", sizeof(char), 0)");
    child_die();
    }
// skip further for SOCK_DGRAM as it's not applicable
    if (variant.type == SOCK_DGRAM)
    return;
    len = sizeof(peer_cred);
    if (getsockopt(cfd, SOL_SOCKET, SO_PEERCRED, &peer_cred, &len)) {
    log_err("Failed to get SO_PEERCRED");
    child_die();
    }
    len = sizeof(peer_pidfd);
    if (getsockopt(cfd, SOL_SOCKET, SO_PEERPIDFD, &peer_pidfd, &len)) {
    log_err("Failed to get SO_PEERPIDFD");
    child_die();
    }
// pid from SO_PEERCRED should point to the parent process PID
    if (peer_cred.pid != getppid()) {
    log_err("peer_cred.pid != getppid(): %d != %d", peer_cred.pid, getppid());
    child_die();
    }
    peer_pid = get_pid_from_fdinfo_file(peer_pidfd,
    "Pid:", sizeof("Pid:") - 1);
    if (peer_pid != peer_cred.pid) {
    log_err("peer_pid != peer_cred.pid: %d != %d", peer_pid, peer_cred.pid);
    child_die();
    }
    }
    TEST_F(scm_pidfd, test)
    {
    int err;
    int pfd;
    let mut child_status: c_int = 0;
    self.server = socket(AF_UNIX, variant.type, 0);
    ASSERT_NE(-1, self.server);
    fill_sockaddr(&self.server_addr, variant.abstract);
    err = bind(self.server, (struct sockaddr *)&self.server_addr.listen_addr, self.server_addr.addrlen);
    ASSERT_EQ(0, err);
    if (variant.type == SOCK_STREAM) {
    err = listen(self.server, 1);
    ASSERT_EQ(0, err);
    }
    err = pipe(self.startup_pipe);
    ASSERT_NE(-1, err);
    self.client_pid = fork();
    ASSERT_NE(-1, self.client_pid);
    if (self.client_pid == 0) {
    close(self.server);
    close(self.startup_pipe[0]);
    client(self, variant);
//
// It's a bit unusual, but in case of success we return non-zero
// exit code (CHILD_EXIT_CODE_OK) and then we expect to read it
// from ioctl(PIDFD_GET_INFO) in cmsg_check_dead().
//
    exit(CHILD_EXIT_CODE_OK);
    }
    close(self.startup_pipe[1]);
    if (variant.type == SOCK_STREAM) {
    pfd = accept(self.server, core::ptr::null_mut(), core::ptr::null_mut());
    ASSERT_NE(-1, pfd);
    } else {
    pfd = self.server;
    }
// wait until the child arrives at checkpoint
    read(self.startup_pipe[0], &err, sizeof(int));
    close(self.startup_pipe[0]);
    if (variant.type == SOCK_DGRAM) {
    err = sendto(pfd, "x", sizeof(char), 0, (struct sockaddr *)&self.client_addr.listen_addr, self.client_addr.addrlen);
    ASSERT_NE(-1, err);
    } else {
    err = send(pfd, "x", sizeof(char), 0);
    ASSERT_NE(-1, err);
    }
    waitpid(self.client_pid, &child_status, 0);
// see comment before exit(CHILD_EXIT_CODE_OK)
    ASSERT_EQ(CHILD_EXIT_CODE_OK, WIFEXITED(child_status) ? WEXITSTATUS(child_status) : 1);
    err = sk_enable_cred_pass(pfd);
    ASSERT_EQ(0, err);
    err = cmsg_check_dead(pfd, self.client_pid);
    ASSERT_EQ(0, err);
    close(pfd);
    }
    TEST_HARNESS_MAIN

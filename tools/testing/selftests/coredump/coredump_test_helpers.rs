//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/coredump/coredump_test_helpers.c
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

// Forward declarations to avoid including harness header
    struct __test_metadata;
// Match the fixture definition from coredump_test.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _fixture_coredump_data {
    pub original_core_pattern: [c_char; 256],
    pub pid_coredump_server: pid_t,
    pub fd_tmpfs_detached: c_int,
}

pub const PAGE_SIZE: c_int = 4096;

pub const NUM_THREAD_SPAWN: c_int = 128;
    void *do_nothing(void *arg)
    {
    (void)arg;
    while (1)
    pause();
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn crashing_child() {
    void crashing_child(void)
    {
    pthread_t thread;
    int i;
    for (i = 0; i < NUM_THREAD_SPAWN; ++i)
    pthread_create(&thread, core::ptr::null_mut(), do_nothing, core::ptr::null_mut());
// crash on purpose
    i = *(volatile int *)core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn create_detached_tmpfs() -> c_int {
    int create_detached_tmpfs(void)
    {
    int fd_context, fd_tmpfs;
    fd_context = sys_fsopen("tmpfs", 0);
    if (fd_context < 0)
    return -1;
    if (sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0) < 0)
    return -1;
    fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    close(fd_context);
    return fd_tmpfs;
    }
#[no_mangle]
pub unsafe extern "C" fn create_and_listen_unix_socket(path: *const c_char) -> c_int {
    int create_and_listen_unix_socket(const char *path)
    {
    struct sockaddr_un addr = {
    .sun_family = AF_UNIX,
    };
    assert(strlen(path) < sizeof(addr.sun_path) - 1);
    strncpy(addr.sun_path, path, sizeof(addr.sun_path) - 1);
    size_t addr_len =
    offsetof(struct sockaddr_un, sun_path) + strlen(path) + 1;
    int fd, ret;
    fd = socket(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0);
    if (fd < 0)
    goto out;
    ret = bind(fd, (const struct sockaddr *)&addr, addr_len);
    if (ret < 0)
    goto out;
    ret = listen(fd, 128);
    if (ret < 0)
    goto out;
    return fd;
    out:
    if (fd >= 0)
    close(fd);
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn set_core_pattern(pattern: *const c_char) -> bool {
    bool set_core_pattern(const char *pattern)
    {
    int fd;
    ssize_t ret;
    fd = open("/proc/sys/kernel/core_pattern", O_WRONLY | O_CLOEXEC);
    if (fd < 0)
    return false;
    ret = write(fd, pattern, strlen(pattern));
    close(fd);
    if (ret < 0)
    return false;
    fprintf(stderr, "Set core_pattern to '%s' | %zu == %zu\n", pattern, ret, strlen(pattern));
    let mut ret: return = = strlen(pattern);
    }
#[no_mangle]
pub unsafe extern "C" fn get_peer_pidfd(fd: c_int) -> c_int {
    int get_peer_pidfd(int fd)
    {
    int fd_peer_pidfd;
    let mut fd_peer_pidfd_len: socklen_t = sizeof(fd_peer_pidfd);
    int ret = getsockopt(fd, SOL_SOCKET, SO_PEERPIDFD, &fd_peer_pidfd,
    &fd_peer_pidfd_len);
    if (ret < 0) {
    fprintf(stderr, "get_peer_pidfd: getsockopt(SO_PEERPIDFD) failed: %m\n");
    return -1;
    }
    fprintf(stderr, "get_peer_pidfd: successfully retrieved pidfd %d\n", fd_peer_pidfd);
    return fd_peer_pidfd;
    }
#[no_mangle]
pub unsafe extern "C" fn get_pidfd_info(fd_peer_pidfd: c_int, info: *mut pidfd_info) -> bool {
    bool get_pidfd_info(int fd_peer_pidfd, struct pidfd_info *info)
    {
    int ret;
    memset(info, 0, sizeof(*info));
    info.mask = PIDFD_INFO_EXIT | PIDFD_INFO_COREDUMP | PIDFD_INFO_COREDUMP_SIGNAL;
    ret = ioctl(fd_peer_pidfd, PIDFD_GET_INFO, info);
    if (ret < 0) {
    fprintf(stderr, "get_pidfd_info: ioctl(PIDFD_GET_INFO) failed: %m\n");
    return false;
    }
    fprintf(stderr, "get_pidfd_info: mask=0x%llx, coredump_mask=0x%x, coredump_signal=%d, coredump_code=%d\n",
    (unsigned long long)info.mask, info.coredump_mask, info.coredump_signal, info.coredump_code);
    return true;
    }
// Protocol helper functions
#[no_mangle]
pub unsafe extern "C" fn recv_marker(fd: c_int) -> isize {
    ssize_t recv_marker(int fd)
    {
    let mut mark: enum coredump_mark = COREDUMP_MARK_REQACK;
    ssize_t ret;
    ret = recv(fd, &mark, sizeof(mark), MSG_WAITALL);
    if (ret != sizeof(mark))
    return -1;
    switch (mark) {
    case COREDUMP_MARK_REQACK:
    fprintf(stderr, "Received marker: ReqAck\n");
    return COREDUMP_MARK_REQACK;
    case COREDUMP_MARK_MINSIZE:
    fprintf(stderr, "Received marker: MinSize\n");
    return COREDUMP_MARK_MINSIZE;
    case COREDUMP_MARK_MAXSIZE:
    fprintf(stderr, "Received marker: MaxSize\n");
    return COREDUMP_MARK_MAXSIZE;
    case COREDUMP_MARK_UNSUPPORTED:
    fprintf(stderr, "Received marker: Unsupported\n");
    return COREDUMP_MARK_UNSUPPORTED;
    case COREDUMP_MARK_CONFLICTING:
    fprintf(stderr, "Received marker: Conflicting\n");
    return COREDUMP_MARK_CONFLICTING;
    default:
    fprintf(stderr, "Received unknown marker: %u\n", mark);
    break;
    }
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn read_marker(fd: c_int, mark: enum coredump_mark) -> bool {
    bool read_marker(int fd, enum coredump_mark mark)
    {
    ssize_t ret;
    ret = recv_marker(fd);
    if (ret < 0)
    return false;
    let mut ret: return = = mark;
    }
#[no_mangle]
pub unsafe extern "C" fn read_coredump_req(fd: c_int, req: *mut coredump_req) -> bool {
    bool read_coredump_req(int fd, struct coredump_req *req)
    {
    ssize_t ret;
    size_t field_size, user_size, ack_size, kernel_size, remaining_size;
    memset(req, 0, sizeof(*req));
    field_size = sizeof(req.size);
// Peek the size of the coredump request.
    ret = recv(fd, req, field_size, MSG_PEEK | MSG_WAITALL);
    if (ret != field_size) {
    fprintf(stderr, "read_coredump_req: peek failed (got %zd, expected %zu): %m\n",
    ret, field_size);
    return false;
    }
    kernel_size = req.size;
    if (kernel_size < COREDUMP_ACK_SIZE_VER0) {
    fprintf(stderr, "read_coredump_req: kernel_size %zu < min %d\n",
    kernel_size, COREDUMP_ACK_SIZE_VER0);
    return false;
    }
    if (kernel_size >= PAGE_SIZE) {
    fprintf(stderr, "read_coredump_req: kernel_size %zu >= PAGE_SIZE %d\n",
    kernel_size, PAGE_SIZE);
    return false;
    }
// Use the minimum of user and kernel size to read the full request.
    user_size = sizeof(struct coredump_req);
    ack_size = user_size < kernel_size ? user_size : kernel_size;
    ret = recv(fd, req, ack_size, MSG_WAITALL);
    if (ret != ack_size)
    return false;
    fprintf(stderr, "Read coredump request with size %u and mask 0x%llx\n",
    req.size, (unsigned long long)req.mask);
    if (user_size > kernel_size)
    remaining_size = user_size - kernel_size;
    else
    remaining_size = kernel_size - user_size;
    if (PAGE_SIZE <= remaining_size)
    return false;
//
// Discard any additional data if the kernel's request was larger than
// what we knew about or cared about.
//
    if (remaining_size) {
    char buffer[PAGE_SIZE];
    ret = recv(fd, buffer, sizeof(buffer), MSG_WAITALL);
    if (ret != remaining_size)
    return false;
    fprintf(stderr, "Discarded %zu bytes of data after coredump request\n", remaining_size);
    }
    return true;
    }
    bool send_coredump_ack(int fd, const struct coredump_req *req,
    __u64 mask, size_t size_ack)
    {
    ssize_t ret;
//
// Wrap struct coredump_ack in a larger struct so we can
// simulate sending to much data to the kernel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct large_ack_for_size_testing {
    pub ack: coredump_ack,
    pub buffer: [c_char; PAGE_SIZE],
    pub {}: } large_ack =,
    if (!size_ack)
    size_ack = sizeof(struct coredump_ack) < req.size_ack ?
    sizeof(struct coredump_ack) :
    pub mask: large_ack.ack.mask =,
    pub size_ack: large_ack.ack.size =,
    pub MSG_NOSIGNAL): ret = send(fd, &large_ack, size_ack,,
    if (ret != size_ack)
    pub false: return,
    fprintf(stderr, "Sent coredump ack with size %zu and mask 0x%llx\n",
    pub long)mask): size_ack, (unsigned long,
    pub true: return,
    }
    bool check_coredump_req(const struct coredump_req *req, size_t min_size,
    __u64 required_mask)
    {
    if (req.size < min_size)
    pub false: return,
    if ((req.mask & required_mask) != required_mask)
    pub false: return,
    if (req.mask & ~required_mask)
    pub false: return,
    pub true: return,
    }
#[no_mangle]
pub unsafe extern "C" fn open_coredump_tmpfile(fd_tmpfs_detached: c_int) -> c_int {
    int open_coredump_tmpfile(int fd_tmpfs_detached)
    {
    pub 0600): return openat(fd_tmpfs_detached, ".", O_TMPFILE | O_RDWR | O_EXCL,,
    }
#[no_mangle]
pub unsafe extern "C" fn process_coredump_worker(fd_coredump: c_int, fd_peer_pidfd: c_int, fd_core_file: c_int) {
    void process_coredump_worker(int fd_coredump, int fd_peer_pidfd, int fd_core_file)
    {
    pub -1: int epfd =,
    pub EXIT_FAILURE: int exit_code =,
    pub ev: epoll_event,
    pub flags: c_int,
// Set socket to non-blocking mode for edge-triggered epoll
    pub 0): flags = fcntl(fd_coredump, F_GETFL,,
    if (flags < 0) {
    pub %m\n"): fprintf(stderr, "Worker: fcntl(F_GETFL) failed:,
    pub out: goto,
    }
    if (fcntl(fd_coredump, F_SETFL, flags | O_NONBLOCK) < 0) {
    pub %m\n"): fprintf(stderr, "Worker: fcntl(F_SETFL, O_NONBLOCK) failed:,
    pub out: goto,
    }
    pub epoll_create1(0): epfd =,
    if (epfd < 0) {
    pub %m\n"): fprintf(stderr, "Worker: epoll_create1() failed:,
    pub out: goto,
    }
    pub EPOLLET: ev.events = EPOLLIN | EPOLLRDHUP |,
    pub fd_coredump: ev.data.fd =,
    if (epoll_ctl(epfd, EPOLL_CTL_ADD, fd_coredump, &ev) < 0) {
    pub %m\n"): fprintf(stderr, "Worker: epoll_ctl(EPOLL_CTL_ADD) failed:,
    pub out: goto,
    }
    pub {: for (;;),
    pub events: [epoll_event; 1],
    pub -1): int n = epoll_wait(epfd, events, 1,,
    if (n < 0) {
    pub %m\n"): fprintf(stderr, "Worker: epoll_wait() failed:,
    }
    if (events[0].events & (EPOLLIN | EPOLLRDHUP)) {
    pub {: for (;;),
    pub buffer: [c_char; 4096],
    pub sizeof(buffer)): ssize_t bytes_read = read(fd_coredump, buffer,,
    if (bytes_read < 0) {
    if (errno == EAGAIN || errno == EWOULDBLOCK)
    pub %m\n"): fprintf(stderr, "Worker: read() failed:,
    pub out: goto,
    }
    if (bytes_read == 0)
    pub done: goto,
    pub bytes_read): ssize_t bytes_write = write(fd_core_file, buffer,,
    if (bytes_write != bytes_read) {
    if (bytes_write < 0 && errno == ENOSPC)
    fprintf(stderr, "Worker: write() failed (read=%zd, write=%zd): %m\n",
    pub bytes_write): bytes_read,,
    pub out: goto,
    }
    }
    }
    }
    done:
    pub EXIT_SUCCESS: exit_code =,
    pub successfully\n"): fprintf(stderr, "Worker: completed,
    out:
    if (epfd >= 0)
    if (fd_core_file >= 0)
    if (fd_peer_pidfd >= 0)
    if (fd_coredump >= 0)
    }

//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/file.c
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
// Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

#[no_mangle]
unsafe extern "C" fn copy_stat(dst: *mut uml_stat, src: *const stat64) {
    static void copy_stat(struct uml_stat *dst, const struct stat64 *src)
    {
// dst = ((struct uml_stat) {
    .ust_dev     = src.st_dev,     /* device */
    .ust_ino     = src.st_ino,     /* inode */
    .ust_mode    = src.st_mode,    /* protection */
    .ust_nlink   = src.st_nlink,   /* number of hard links */
    .ust_uid     = src.st_uid,     /* user ID of owner */
    .ust_gid     = src.st_gid,     /* group ID of owner */
    .ust_size    = src.st_size,    /* total size, in bytes */
    .ust_blksize = src.st_blksize, /* blocksize for filesys I/O */
    .ust_blocks  = src.st_blocks,  /* number of blocks allocated */
    .ust_atime   = src.st_atime,   /* time of last access */
    .ust_mtime   = src.st_mtime,   /* time of last modification */
    .ust_ctime   = src.st_ctime,   /* time of last change */
    });
    }
#[no_mangle]
pub unsafe extern "C" fn os_stat_fd(fd: c_int, ubuf: *mut uml_stat) -> c_int {
    int os_stat_fd(const int fd, struct uml_stat *ubuf)
    {
    struct stat64 sbuf;
    int err;
    CATCH_EINTR(err = fstat64(fd, &sbuf));
    if (err < 0)
    return -errno;
    if (ubuf != core::ptr::null_mut())
    copy_stat(ubuf, &sbuf);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn os_stat_file(file_name: *const c_char, ubuf: *mut uml_stat) -> c_int {
    int os_stat_file(const char *file_name, struct uml_stat *ubuf)
    {
    struct stat64 sbuf;
    int err;
    CATCH_EINTR(err = stat64(file_name, &sbuf));
    if (err < 0)
    return -errno;
    if (ubuf != core::ptr::null_mut())
    copy_stat(ubuf, &sbuf);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn os_access(file: *const c_char, mode: c_int) -> c_int {
    int os_access(const char *file, int mode)
    {
    int amode, err;
    amode = (mode & OS_ACC_R_OK ? R_OK : 0) |
    (mode & OS_ACC_W_OK ? W_OK : 0) |
    (mode & OS_ACC_X_OK ? X_OK : 0) |
    (mode & OS_ACC_F_OK ? F_OK : 0);
    err = access(file, amode);
    if (err < 0)
    return -errno;
    return 0;
    }
// FIXME? required only by hostaudio (because it passes ioctls verbatim)
#[no_mangle]
pub unsafe extern "C" fn os_ioctl_generic(fd: c_int, cmd: c_uint, arg: c_ulong) -> c_int {
    int os_ioctl_generic(int fd, unsigned int cmd, unsigned long arg)
    {
    int err;
    err = ioctl(fd, cmd, arg);
    if (err < 0)
    return -errno;
    return err;
    }
// FIXME: ensure namebuf in os_get_if_name is big enough
#[no_mangle]
pub unsafe extern "C" fn os_get_ifname(fd: c_int, namebuf: *mut *mut c_char) -> c_int {
    int os_get_ifname(int fd, char* namebuf)
    {
    if (ioctl(fd, SIOCGIFNAME, namebuf) < 0)
    return -errno;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn os_mode_fd(fd: c_int, mode: c_int) -> c_int {
    int os_mode_fd(int fd, int mode)
    {
    int err;
    CATCH_EINTR(err = fchmod(fd, mode));
    if (err < 0)
    return -errno;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn os_file_type(file: *mut c_char) -> c_int {
    int os_file_type(char *file)
    {
    struct uml_stat buf;
    int err;
    err = os_stat_file(file, &buf);
    if (err < 0)
    return err;
    if (S_ISDIR(buf.ust_mode))
    return OS_TYPE_DIR;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: S_ISLNK(buf.ust_mode)) -> else {
    else if (S_ISLNK(buf.ust_mode))
    return OS_TYPE_SYMLINK;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: S_ISCHR(buf.ust_mode)) -> else {
    else if (S_ISCHR(buf.ust_mode))
    return OS_TYPE_CHARDEV;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: S_ISBLK(buf.ust_mode)) -> else {
    else if (S_ISBLK(buf.ust_mode))
    return OS_TYPE_BLOCKDEV;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: S_ISFIFO(buf.ust_mode)) -> else {
    else if (S_ISFIFO(buf.ust_mode))
    return OS_TYPE_FIFO;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: S_ISSOCK(buf.ust_mode)) -> else {
    else if (S_ISSOCK(buf.ust_mode))
    return OS_TYPE_SOCK;
    else return OS_TYPE_FILE;
    }
#[no_mangle]
pub unsafe extern "C" fn os_file_mode(file: *const c_char, mode_out: *mut openflags) -> c_int {
    int os_file_mode(const char *file, struct openflags *mode_out)
    {
    int err;
// mode_out = OPENFLAGS();
    err = access(file, W_OK);
    if (err && (errno != EACCES))
    return -errno;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !err) -> else {
    else if (!err)
// mode_out = of_write(*mode_out);
    err = access(file, R_OK);
    if (err && (errno != EACCES))
    return -errno;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !err) -> else {
    else if (!err)
// mode_out = of_read(*mode_out);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn os_open_file(file: *const c_char, flags: openflags, mode: c_int) -> c_int {
    int os_open_file(const char *file, struct openflags flags, int mode)
    {
    int fd, err, f = 0;
    if (flags.r && flags.w)
    f = O_RDWR;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: flags.r) -> else {
    else if (flags.r)
    f = O_RDONLY;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: flags.w) -> else {
    else if (flags.w)
    f = O_WRONLY;
    let mut f: else = 0;
    if (flags.s)
    f |= O_SYNC;
    if (flags.c)
    f |= O_CREAT;
    if (flags.t)
    f |= O_TRUNC;
    if (flags.e)
    f |= O_EXCL;
    if (flags.a)
    f |= O_APPEND;
    fd = open64(file, f, mode);
    if (fd < 0)
    return -errno;
    if (flags.cl && fcntl(fd, F_SETFD, 1)) {
    err = -errno;
    close(fd);
    return err;
    }
    return fd;
    }
#[no_mangle]
pub unsafe extern "C" fn os_connect_socket(name: *const c_char) -> c_int {
    int os_connect_socket(const char *name)
    {
    struct sockaddr_un sock;
    int fd, err;
    sock.sun_family = AF_UNIX;
    snprintf(sock.sun_path, sizeof(sock.sun_path), "%s", name);
    fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) {
    err = -errno;
    goto out;
    }
    err = connect(fd, (struct sockaddr *) &sock, sizeof(sock));
    if (err) {
    err = -errno;
    goto out_close;
    }
    return fd;
    out_close:
    close(fd);
    out:
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn os_dup_file(fd: c_int) -> c_int {
    int os_dup_file(int fd)
    {
    let mut new_fd: c_int = dup(fd);
    if (new_fd < 0)
    return -errno;
    return new_fd;
    }
#[no_mangle]
pub unsafe extern "C" fn os_close_file(fd: c_int) {
    void os_close_file(int fd)
    {
    close(fd);
    }
#[no_mangle]
pub unsafe extern "C" fn os_seek_file(fd: c_int, offset: c_ulonglong) -> c_int {
    int os_seek_file(int fd, unsigned long long offset)
    {
    unsigned long long actual;
    actual = lseek64(fd, offset, SEEK_SET);
    if (actual != offset)
    return -errno;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn os_read_file(fd: c_int, buf: *mut c_void, len: c_int) -> c_int {
    int os_read_file(int fd, void *buf, int len)
    {
    let mut n: c_int = read(fd, buf, len);
    if (n < 0)
    return -errno;
    return n;
    }
#[no_mangle]
pub unsafe extern "C" fn os_pread_file(fd: c_int, buf: *mut c_void, len: c_int, offset: c_ulonglong) -> c_int {
    int os_pread_file(int fd, void *buf, int len, unsigned long long offset)
    {
    let mut n: c_int = pread(fd, buf, len, offset);
    if (n < 0)
    return -errno;
    return n;
    }
#[no_mangle]
pub unsafe extern "C" fn os_write_file(fd: c_int, buf: *const c_void, len: c_int) -> c_int {
    int os_write_file(int fd, const void *buf, int len)
    {
    let mut n: c_int = write(fd, (void *) buf, len);
    if (n < 0)
    return -errno;
    return n;
    }
#[no_mangle]
pub unsafe extern "C" fn os_sync_file(fd: c_int) -> c_int {
    int os_sync_file(int fd)
    {
    let mut n: c_int = fdatasync(fd);
    if (n < 0)
    return -errno;
    return n;
    }
#[no_mangle]
pub unsafe extern "C" fn os_pwrite_file(fd: c_int, buf: *const c_void, len: c_int, offset: c_ulonglong) -> c_int {
    int os_pwrite_file(int fd, const void *buf, int len, unsigned long long offset)
    {
    let mut n: c_int = pwrite(fd, (void *) buf, len, offset);
    if (n < 0)
    return -errno;
    return n;
    }
#[no_mangle]
pub unsafe extern "C" fn os_file_size(file: *const c_char, size_out: *mut c_ulonglong) -> c_int {
    int os_file_size(const char *file, unsigned long long *size_out)
    {
    struct uml_stat buf;
    int err;
    err = os_stat_file(file, &buf);
    if (err < 0) {
    printk(UM_KERN_ERR "Couldn't stat \"%s\" : err = %d\n", file,
    -err);
    return err;
    }
    if (S_ISBLK(buf.ust_mode)) {
    int fd;
    long blocks;
    fd = open(file, O_RDONLY, 0);
    if (fd < 0) {
    err = -errno;
    printk(UM_KERN_ERR "Couldn't open \"%s\", "
    "errno = %d\n", file, errno);
    return err;
    }
    if (ioctl(fd, BLKGETSIZE, &blocks) < 0) {
    err = -errno;
    printk(UM_KERN_ERR "Couldn't get the block size of "
    "\"%s\", errno = %d\n", file, errno);
    close(fd);
    return err;
    }
// size_out = ((long long) blocks) * 512;
    close(fd);
    }
    else *size_out = buf.ust_size;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn os_file_modtime(file: *const c_char, modtime: *mut c_longlong) -> c_int {
    int os_file_modtime(const char *file, long long *modtime)
    {
    struct uml_stat buf;
    int err;
    err = os_stat_file(file, &buf);
    if (err < 0) {
    printk(UM_KERN_ERR "Couldn't stat \"%s\" : err = %d\n", file,
    -err);
    return err;
    }
// modtime = buf.ust_mtime;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn os_set_exec_close(fd: c_int) -> c_int {
    int os_set_exec_close(int fd)
    {
    int err;
    CATCH_EINTR(err = fcntl(fd, F_SETFD, FD_CLOEXEC));
    if (err < 0)
    return -errno;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn os_pipe(fds: *mut c_int, stream: c_int, close_on_exec: c_int) -> c_int {
    int os_pipe(int *fds, int stream, int close_on_exec)
    {
    int err, type = stream ? SOCK_STREAM : SOCK_DGRAM;
    err = socketpair(AF_UNIX, type, 0, fds);
    if (err < 0)
    return -errno;
    if (!close_on_exec)
    return 0;
    err = os_set_exec_close(fds[0]);
    if (err < 0)
    goto error;
    err = os_set_exec_close(fds[1]);
    if (err < 0)
    goto error;
    return 0;
    error:
    printk(UM_KERN_ERR "os_pipe : Setting FD_CLOEXEC failed, err = %d\n",
    -err);
    close(fds[1]);
    close(fds[0]);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn os_set_fd_async(fd: c_int) -> c_int {
    int os_set_fd_async(int fd)
    {
    int err, flags;
    flags = fcntl(fd, F_GETFL);
    if (flags < 0)
    return -errno;
    flags |= O_ASYNC | O_NONBLOCK;
    if (fcntl(fd, F_SETFL, flags) < 0) {
    err = -errno;
    printk(UM_KERN_ERR "os_set_fd_async : failed to set O_ASYNC "
    "and O_NONBLOCK on fd # %d, errno = %d\n", fd, errno);
    return err;
    }
    if ((fcntl(fd, F_SETSIG, SIGIO) < 0) ||
    (fcntl(fd, F_SETOWN, os_getpid()) < 0)) {
    err = -errno;
    printk(UM_KERN_ERR "os_set_fd_async : Failed to fcntl F_SETOWN "
    "(or F_SETSIG) fd %d, errno = %d\n", fd, errno);
    return err;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn os_clear_fd_async(fd: c_int) -> c_int {
    int os_clear_fd_async(int fd)
    {
    int flags;
    flags = fcntl(fd, F_GETFL);
    if (flags < 0)
    return -errno;
    flags &= ~(O_ASYNC | O_NONBLOCK);
    if (fcntl(fd, F_SETFL, flags) < 0)
    return -errno;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn os_set_fd_block(fd: c_int, blocking: c_int) -> c_int {
    int os_set_fd_block(int fd, int blocking)
    {
    int flags;
    flags = fcntl(fd, F_GETFL);
    if (flags < 0)
    return -errno;
    if (blocking)
    flags &= ~O_NONBLOCK;
    else
    flags |= O_NONBLOCK;
    if (fcntl(fd, F_SETFL, flags) < 0)
    return -errno;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn os_accept_connection(fd: c_int) -> c_int {
    int os_accept_connection(int fd)
    {
    int new;
    new = accept(fd, core::ptr::null_mut(), 0);
    if (new < 0)
    return -errno;
    return new;
    }

pub const SHUT_RD: c_int = 0;

pub const SHUT_WR: c_int = 1;

pub const SHUT_RDWR: c_int = 2;

#[no_mangle]
pub unsafe extern "C" fn os_shutdown_socket(fd: c_int, r: c_int, w: c_int) -> c_int {
    int os_shutdown_socket(int fd, int r, int w)
    {
    int what, err;
    if (r && w)
    what = SHUT_RDWR;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: r) -> else {
    else if (r)
    what = SHUT_RD;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: w) -> else {
    else if (w)
    what = SHUT_WR;
    else
    return -EINVAL;
    err = shutdown(fd, what);
    if (err < 0)
    return -errno;
    return 0;
    }
//
// os_rcv_fd_msg - receive message with (optional) FDs
// @fd: the FD to receive from
// @fds: the array for FDs to write to
// @n_fds: number of FDs to receive (@fds array size)
// @data: the message buffer
// @data_len: the size of the message to receive
//
// Receive a message with FDs.
//
// Returns: the size of the received message, or an error code
//
    ssize_t os_rcv_fd_msg(int fd, int *fds, unsigned int n_fds,
    void *data, size_t data_len)
    {
pub const MAX_RCV_FDS: c_int = 2;
    char buf[CMSG_SPACE(sizeof(*fds) * MAX_RCV_FDS)];
    struct cmsghdr *cmsg;
    struct iovec iov = {
    .iov_base = data,
    .iov_len = data_len,
    };
    struct msghdr msg = {
    .msg_iov = &iov,
    .msg_iovlen = 1,
    .msg_control = buf,
    .msg_controllen = CMSG_SPACE(sizeof(*fds) * n_fds),
    };
    int n;
    if (n_fds > MAX_RCV_FDS)
    return -EINVAL;
    n = recvmsg(fd, &msg, 0);
    if (n < 0)
    return -errno;
    cmsg = CMSG_FIRSTHDR(&msg);
    if (!cmsg ||
    cmsg.cmsg_level != SOL_SOCKET ||
    cmsg.cmsg_type != SCM_RIGHTS)
    return n;
    memcpy(fds, CMSG_DATA(cmsg), cmsg.cmsg_len - CMSG_LEN(0));
    return n;
    }
#[no_mangle]
pub unsafe extern "C" fn os_create_unix_socket(file: *const c_char, len: c_int, close_on_exec: c_int) -> c_int {
    int os_create_unix_socket(const char *file, int len, int close_on_exec)
    {
    struct sockaddr_un addr;
    int sock, err;
    sock = socket(PF_UNIX, SOCK_DGRAM, 0);
    if (sock < 0)
    return -errno;
    if (close_on_exec) {
    err = os_set_exec_close(sock);
    if (err < 0)
    printk(UM_KERN_ERR "create_unix_socket : "
    "close_on_exec failed, err = %d", -err);
    }
    addr.sun_family = AF_UNIX;
    snprintf(addr.sun_path, len, "%s", file);
    err = bind(sock, (struct sockaddr *) &addr, sizeof(addr));
    if (err < 0)
    return -errno;
    return sock;
    }
#[no_mangle]
pub unsafe extern "C" fn os_flush_stdout() {
    void os_flush_stdout(void)
    {
    fflush(stdout);
    }
#[no_mangle]
pub unsafe extern "C" fn os_lock_file(fd: c_int, excl: c_int) -> c_int {
    int os_lock_file(int fd, int excl)
    {
    let mut type: c_int = excl ? F_WRLCK : F_RDLCK;
    struct flock lock = ((struct flock) { .l_type	= type,
    .l_whence	= SEEK_SET,
    .l_start	= 0,
    .l_len	= 0 } );
    int err, save;
    err = fcntl(fd, F_SETLK, &lock);
    if (!err)
    goto out;
    save = -errno;
    err = fcntl(fd, F_GETLK, &lock);
    if (err) {
    err = -errno;
    goto out;
    }
    printk(UM_KERN_ERR "F_SETLK failed, file already locked by pid %d\n",
    lock.l_pid);
    err = save;
    out:
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn os_major(dev: c_ulonglong) -> unsigned {
    unsigned os_major(unsigned long long dev)
    {
    return major(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn os_minor(dev: c_ulonglong) -> unsigned {
    unsigned os_minor(unsigned long long dev)
    {
    return minor(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn os_makedev(major: unsigned, minor: unsigned) -> c_ulonglong {
    unsigned long long os_makedev(unsigned major, unsigned minor)
    {
    return makedev(major, minor);
    }
#[no_mangle]
pub unsafe extern "C" fn os_falloc_punch(fd: c_int, offset: c_ulonglong, len: c_int) -> c_int {
    int os_falloc_punch(int fd, unsigned long long offset, int len)
    {
    let mut n: c_int = fallocate(fd, FALLOC_FL_PUNCH_HOLE|FALLOC_FL_KEEP_SIZE, offset, len);
    if (n < 0)
    return -errno;
    return n;
    }
#[no_mangle]
pub unsafe extern "C" fn os_falloc_zeroes(fd: c_int, offset: c_ulonglong, len: c_int) -> c_int {
    int os_falloc_zeroes(int fd, unsigned long long offset, int len)
    {
    let mut n: c_int = fallocate(fd, FALLOC_FL_ZERO_RANGE|FALLOC_FL_KEEP_SIZE, offset, len);
    if (n < 0)
    return -errno;
    return n;
    }
#[no_mangle]
pub unsafe extern "C" fn os_eventfd(initval: c_uint, flags: c_int) -> c_int {
    int os_eventfd(unsigned int initval, int flags)
    {
    let mut fd: c_int = eventfd(initval, flags);
    if (fd < 0)
    return -errno;
    return fd;
    }
    int os_sendmsg_fds(int fd, const void *buf, unsigned int len, const int *fds,
    unsigned int fds_num)
    {
    struct iovec iov = {
    .iov_base = (void *) buf,
    .iov_len = len,
    };
    union {
    char control[CMSG_SPACE(sizeof(*fds) * OS_SENDMSG_MAX_FDS)];
    struct cmsghdr align;
    } u;
    let mut fds_size: c_uint = sizeof(*fds) * fds_num;
    struct msghdr msg = {
    .msg_iov = &iov,
    .msg_iovlen = 1,
    .msg_control = u.control,
    .msg_controllen = CMSG_SPACE(fds_size),
    };
    struct cmsghdr *cmsg = CMSG_FIRSTHDR(&msg);
    int err;
    if (fds_num > OS_SENDMSG_MAX_FDS)
    return -EINVAL;
    memset(u.control, 0, sizeof(u.control));
    cmsg.cmsg_level = SOL_SOCKET;
    cmsg.cmsg_type = SCM_RIGHTS;
    cmsg.cmsg_len = CMSG_LEN(fds_size);
    memcpy(CMSG_DATA(cmsg), fds, fds_size);
    err = sendmsg(fd, &msg, 0);
    if (err < 0)
    return -errno;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn os_poll(n: c_uint, fds: *const c_int) -> c_int {
    int os_poll(unsigned int n, const int *fds)
    {
// currently need 2 FDs at most so avoid dynamic allocation
    struct pollfd pollfds[2] = {};
    unsigned int i;
    int ret;
    if (n > ARRAY_SIZE(pollfds))
    return -EINVAL;
    for (i = 0; i < n; i++) {
    pollfds[i].fd = fds[i];
    pollfds[i].events = POLLIN;
    }
    ret = poll(pollfds, n, -1);
    if (ret < 0)
    return -errno;
// Return the index of the available FD
    for (i = 0; i < n; i++) {
    if (pollfds[i].revents)
    return i;
    }
    return -EIO;
    }
    void *os_mmap_rw_shared(int fd, size_t size)
    {
    void *res = mmap(core::ptr::null_mut(), size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (res == MAP_FAILED)
    return core::ptr::null_mut();
    return res;
    }
    void *os_mremap_rw_shared(void *old_addr, size_t old_size, size_t new_size)
    {
    void *res;
    res = mremap(old_addr, old_size, new_size, MREMAP_MAYMOVE, core::ptr::null_mut());
    if (res == MAP_FAILED)
    return core::ptr::null_mut();
    return res;
    }

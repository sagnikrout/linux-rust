//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/failfs/failfs_test.c
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

pub const __NR_fchroot: c_int = 472;

pub const NOBODY_UID: c_int = 65534;
// Child sentinel exit code: the exec was blocked as expected.
pub const FAILFS_EXEC_BLOCKED: c_int = 99;
// Stack for the CLONE_FS helper in fchroot_sentinel_shared_fs_struct.

#[no_mangle]
unsafe extern "C" fn sys_fchroot(fd: c_int, flags: c_uint) -> c_int {
    static int sys_fchroot(int fd, unsigned int flags)
    {
    return syscall(__NR_fchroot, fd, flags);
    }
//
// Raw syscall: glibc's getcwd() rejects the kernel's "(unreachable)"
// result and falls back to a generic implementation.
//
#[no_mangle]
unsafe extern "C" fn sys_getcwd(buf: *mut c_char, size: usize) -> c_long {
    static long sys_getcwd(char *buf, size_t size)
    {
    return syscall(__NR_getcwd, buf, size);
    }
#[no_mangle]
unsafe extern "C" fn drop_to_nobody() -> c_int {
    static int drop_to_nobody(void)
    {
    return setresuid(NOBODY_UID, NOBODY_UID, NOBODY_UID);
    }
// Parked CLONE_FS child; dies with its parent so it never leaks.
#[no_mangle]
unsafe extern "C" fn failfs_park(arg: *mut c_void) -> c_int {
    static int failfs_park(void *arg)
    {
    let mut parent: pid_t = (pid_t)(long)arg;
    prctl(PR_SET_PDEATHSIG, SIGKILL);
// The parent may have died before the death signal was armed.
    if (getppid() != parent)
    _exit(0);
    pause();
    return 0;
    }
// Is fd a dynamically linked ELF with an absolute PT_INTERP interpreter?
#[no_mangle]
unsafe extern "C" fn elf_has_absolute_interp(fd: c_int) -> c_int {
    static int elf_has_absolute_interp(int fd)
    {
    ElfW(Ehdr) ehdr;
    ElfW(Phdr) phdr;
    char interp;
    int i;
    if (pread(fd, &ehdr, sizeof(ehdr), 0) != sizeof(ehdr))
    return 0;
    if (memcmp(ehdr.e_ident, ELFMAG, SELFMAG) != 0)
    return 0;
    for (i = 0; i < ehdr.e_phnum; i++) {
    if (pread(fd, &phdr, sizeof(phdr),
    ehdr.e_phoff + i * sizeof(phdr)) != sizeof(phdr))
    return 0;
    if (phdr.p_type != PT_INTERP)
    continue;
    if (pread(fd, &interp, 1, phdr.p_offset) != 1)
    return 0;
    let mut interp: return = = '/';
    }
    return 0;
    }
    TEST(fchdir_sentinel)
    {
    char buf[PATH_MAX];
    int fd;
    ASSERT_EQ(fchdir(FD_FAILFS_ROOT), 0);
// The working directory is unreachable from the process root.
    ASSERT_GT(sys_getcwd(buf, sizeof(buf)), 0);
    ASSERT_EQ(strncmp(buf, "(unreachable)", 13), 0);
// Every AT_FDCWD-relative lookup fails.
    ASSERT_EQ(openat(AT_FDCWD, "foo", O_RDONLY), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
    ASSERT_EQ(openat(AT_FDCWD, ".", O_RDONLY), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
    ASSERT_EQ(openat(AT_FDCWD, "..", O_RDONLY), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
    ASSERT_EQ(openat(AT_FDCWD, "foo", O_WRONLY | O_CREAT, 0600), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
// The cwd cannot be pinned by following /proc/self/cwd into it.
    ASSERT_EQ(open("/proc/self/cwd", O_PATH), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
// The root is untouched so absolute lookups keep working...
    fd = open("/", O_RDONLY | O_DIRECTORY);
    ASSERT_GE(fd, 0);
    ASSERT_EQ(close(fd), 0);
// ... and the working directory can be recovered.
    ASSERT_EQ(chdir("/"), 0);
    ASSERT_GT(sys_getcwd(buf, sizeof(buf)), 0);
    ASSERT_EQ(strcmp(buf, "/"), 0);
    }
    TEST(fchdir_rejects_other_sentinels)
    {
    ASSERT_EQ(fchdir(FD_PIDFS_ROOT), -1);
    ASSERT_EQ(errno, EBADF);
    ASSERT_EQ(fchdir(FD_NSFS_ROOT), -1);
    ASSERT_EQ(errno, EBADF);
    ASSERT_EQ(fchdir(-10009), -1);
    ASSERT_EQ(errno, EBADF);
    }
    TEST(fchroot_flags)
    {
    int fd;
    ASSERT_EQ(sys_fchroot(FD_FAILFS_ROOT, 1), -1);
    ASSERT_EQ(errno, EINVAL);
    fd = open("/", O_PATH | O_DIRECTORY);
    ASSERT_GE(fd, 0);
    ASSERT_EQ(sys_fchroot(fd, 1), -1);
    ASSERT_EQ(errno, EINVAL);
    ASSERT_EQ(close(fd), 0);
    }
    TEST(fchroot_bad_fd)
    {
    ASSERT_EQ(sys_fchroot(-1, 0), -1);
    ASSERT_EQ(errno, EBADF);
// Only FD_FAILFS_ROOT is a valid sentinel.
    ASSERT_EQ(sys_fchroot(FD_PIDFS_ROOT, 0), -1);
    ASSERT_EQ(errno, EBADF);
    ASSERT_EQ(sys_fchroot(FD_NSFS_ROOT, 0), -1);
    ASSERT_EQ(errno, EBADF);
    }
    TEST(fchroot_notdir)
    {
    int fd;
    fd = open("/proc/self/status", O_RDONLY);
    ASSERT_GE(fd, 0);
    ASSERT_EQ(sys_fchroot(fd, 0), -1);
    ASSERT_EQ(errno, ENOTDIR);
    ASSERT_EQ(close(fd), 0);
    }
    TEST(fchroot_realfd_requires_cap)
    {
    int fd;
    if (geteuid() == 0)
    ASSERT_EQ(drop_to_nobody(), 0);
    fd = open("/", O_PATH | O_DIRECTORY);
    ASSERT_GE(fd, 0);
    ASSERT_EQ(sys_fchroot(fd, 0), -1);
    ASSERT_EQ(errno, EPERM);
    ASSERT_EQ(close(fd), 0);
    }
    TEST(fchroot_realfd)
    {
    char template[] = "/tmp/failfs_test.XXXXXX";
    char path[PATH_MAX];
    struct stat st;
    int tmpfd, dfd, fd;
    if (geteuid() != 0)
    SKIP(return, "fchroot() with a regular fd requires CAP_SYS_CHROOT");
    tmpfd = open("/tmp", O_PATH | O_DIRECTORY);
    ASSERT_GE(tmpfd, 0);
    ASSERT_NE(mkdtemp(template), core::ptr::null_mut());
    snprintf(path, sizeof(path), "%s/canary", template);
    fd = open(path, O_WRONLY | O_CREAT, 0600);
    ASSERT_GE(fd, 0);
    ASSERT_EQ(close(fd), 0);
    dfd = open(template, O_PATH | O_DIRECTORY);
    ASSERT_GE(dfd, 0);
    ASSERT_EQ(sys_fchroot(dfd, 0), 0);
    ASSERT_EQ(close(dfd), 0);
    ASSERT_EQ(stat("/canary", &st), 0);
// Best-effort cleanup: dirfd-anchored I/O works with the new root.
    snprintf(path, sizeof(path), "%s/canary", template + strlen("/tmp/"));
    unlinkat(tmpfd, path, 0);
    unlinkat(tmpfd, template + strlen("/tmp/"), AT_REMOVEDIR);
    }
    TEST(fchroot_sentinel)
    {
    char template[] = "/tmp/failfs_test.XXXXXX";
    struct stat realroot, st;
    struct statfs sfs;
    char buf[PATH_MAX];
    int procfd, tmpfd, dfd, fd;
    struct {
    struct file_handle handle;
    unsigned char f_handle[MAX_HANDLE_SZ];
    } fh;
    int mntid;
    ssize_t ret;
    if (geteuid() != 0)
    SKIP(return, "privileged fchroot(FD_FAILFS_ROOT) requires CAP_SYS_CHROOT");
    ASSERT_EQ(stat("/", &realroot), 0);
    procfd = open("/proc", O_PATH | O_DIRECTORY);
    ASSERT_GE(procfd, 0);
    tmpfd = open("/tmp", O_PATH | O_DIRECTORY);
    ASSERT_GE(tmpfd, 0);
    ASSERT_NE(mkdtemp(template), core::ptr::null_mut());
    dfd = open(template, O_RDONLY | O_DIRECTORY);
    ASSERT_GE(dfd, 0);
    ASSERT_EQ(sys_fchroot(FD_FAILFS_ROOT, 0), 0);
// Absolute lookups fail.
    ASSERT_EQ(open("/etc/passwd", O_RDONLY), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
    ASSERT_EQ(mkdir("/foo", 0700), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
//
// The root cannot be referenced at all - not even an O_PATH open,
// which skips ->permission(), because it lands on the root as a
// jumped walk terminal that ->d_weak_revalidate() refuses.
//
    ASSERT_EQ(open("/", O_RDONLY | O_DIRECTORY), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
    ASSERT_EQ(open("/", O_PATH), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
    ASSERT_EQ(statfs("/", &sfs), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
//
// It cannot be pinned by following /proc/self/root into it either
// (only the root is in failfs here, so self/cwd is still real).
//
    ASSERT_EQ(openat(procfd, "self/root", O_PATH), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
// Nor encoded into a file handle.
    fh.handle.handle_bytes = MAX_HANDLE_SZ;
    ASSERT_EQ(name_to_handle_at(AT_FDCWD, "/", &fh.handle, &mntid, 0), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
// The working directory is now unreachable from the root.
    ASSERT_GT(sys_getcwd(buf, sizeof(buf)), 0);
    ASSERT_EQ(strncmp(buf, "(unreachable)", 13), 0);
// Lookups anchored at real directories keep working.
    fd = openat(AT_FDCWD, ".", O_RDONLY | O_DIRECTORY);
    ASSERT_GE(fd, 0);
    ASSERT_EQ(close(fd), 0);
    fd = openat(dfd, "canary", O_WRONLY | O_CREAT, 0600);
    ASSERT_GE(fd, 0);
    ASSERT_EQ(write(fd, "x", 1), 1);
    ASSERT_EQ(close(fd), 0);
    fd = openat(dfd, "canary", O_RDONLY);
    ASSERT_GE(fd, 0);
    ASSERT_EQ(close(fd), 0);
// ".." walks clamp at the top of the mount tree, not at failfs.
    fd = openat(AT_FDCWD, "../../../../../../../../../..", O_PATH);
    ASSERT_GE(fd, 0);
    ASSERT_EQ(fstat(fd, &st), 0);
    ASSERT_EQ(st.st_dev, realroot.st_dev);
    ASSERT_EQ(st.st_ino, realroot.st_ino);
    ASSERT_EQ(close(fd), 0);
// readlink of the magic link still works: it does not follow.
    ret = readlinkat(procfd, "self/root", buf, sizeof(buf) - 1);
    ASSERT_GT(ret, 0);
    buf[ret] = '\0';
    TH_LOG("/proc/self/root points to '%s'", buf);
// d_path() names the failfs root synthetically, never as a real path.
    ASSERT_EQ(strcmp(buf, "failfs:/"), 0);
// But following it into failfs is refused.
    ASSERT_EQ(fstatat(procfd, "self/root", &st, 0), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
// Best-effort cleanup via the pre-opened dirfds.
    unlinkat(dfd, "canary", 0);
    unlinkat(tmpfd, template + strlen("/tmp/"), AT_REMOVEDIR);
    }
    TEST(fchroot_sentinel_absolute_symlink)
    {
    char template[] = "/tmp/failfs_test.XXXXXX";
    int tmpfd, dfd, fd;
    if (geteuid() != 0)
    SKIP(return, "privileged fchroot(FD_FAILFS_ROOT) requires CAP_SYS_CHROOT");
    tmpfd = open("/tmp", O_PATH | O_DIRECTORY);
    ASSERT_GE(tmpfd, 0);
    ASSERT_NE(mkdtemp(template), core::ptr::null_mut());
    dfd = open(template, O_RDONLY | O_DIRECTORY);
    ASSERT_GE(dfd, 0);
    fd = openat(dfd, "target", O_WRONLY | O_CREAT, 0600);
    ASSERT_GE(fd, 0);
    ASSERT_EQ(close(fd), 0);
    ASSERT_EQ(symlinkat("target", dfd, "rel"), 0);
    ASSERT_EQ(symlinkat("/etc", dfd, "abs"), 0);
    ASSERT_EQ(sys_fchroot(FD_FAILFS_ROOT, 0), 0);
// Relative symlinks keep resolving within the dirfd-anchored walk...
    fd = openat(dfd, "rel", O_RDONLY);
    ASSERT_GE(fd, 0);
    ASSERT_EQ(close(fd), 0);
// ... absolute symlinks restart the walk at the failfs root.
    ASSERT_EQ(openat(dfd, "abs", O_RDONLY), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
// Best-effort cleanup via the pre-opened dirfds.
    unlinkat(dfd, "abs", 0);
    unlinkat(dfd, "rel", 0);
    unlinkat(dfd, "target", 0);
    unlinkat(tmpfd, template + strlen("/tmp/"), AT_REMOVEDIR);
    }
    TEST(fchroot_sentinel_unprivileged)
    {
    char buf[PATH_MAX];
    if (geteuid() == 0)
    ASSERT_EQ(drop_to_nobody(), 0);
// Without no_new_privs entering failfs is not allowed...
    ASSERT_EQ(sys_fchroot(FD_FAILFS_ROOT, 0), -1);
    ASSERT_EQ(errno, EPERM);
// ... with no_new_privs set it is allowed.
    ASSERT_EQ(prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0), 0);
    ASSERT_EQ(sys_fchroot(FD_FAILFS_ROOT, 0), 0);
    ASSERT_EQ(open("/etc/passwd", O_RDONLY), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
// The task counts as chrooted: no user namespaces anymore.
    ASSERT_EQ(unshare(CLONE_NEWUSER), -1);
    ASSERT_EQ(errno, EPERM);
// With both root and cwd in failfs getcwd() reports "/".
    ASSERT_EQ(fchdir(FD_FAILFS_ROOT), 0);
    ASSERT_GT(sys_getcwd(buf, sizeof(buf)), 0);
    ASSERT_EQ(strcmp(buf, "/"), 0);
    }
    TEST(fchroot_sentinel_rejected_when_chrooted)
    {
    char template[] = "/tmp/failfs_test.XXXXXX";
    int tmpfd;
    if (geteuid() != 0)
    SKIP(return, "chroot() requires CAP_SYS_CHROOT");
    tmpfd = open("/tmp", O_PATH | O_DIRECTORY);
    ASSERT_GE(tmpfd, 0);
    ASSERT_NE(mkdtemp(template), core::ptr::null_mut());
    ASSERT_EQ(chroot(template), 0);
    ASSERT_EQ(chdir("/"), 0);
// Remove the jail while still privileged; sticky /tmp blocks nobody.
    unlinkat(tmpfd, template + strlen("/tmp/"), AT_REMOVEDIR);
    ASSERT_EQ(drop_to_nobody(), 0);
    ASSERT_EQ(prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0), 0);
// An unprivileged chrooted task must not lift its ".." barrier.
    ASSERT_EQ(sys_fchroot(FD_FAILFS_ROOT, 0), -1);
    ASSERT_EQ(errno, EPERM);
    }
    TEST(fchroot_sentinel_shared_fs_struct)
    {
    char stack[FAILFS_CLONE_STACK];
    pid_t pid;
    if (geteuid() == 0)
    ASSERT_EQ(drop_to_nobody(), 0);
// A CLONE_FS sibling shares the fs_struct: bump fs->users to 2.
    pid = clone(failfs_park, stack + sizeof(stack), CLONE_FS | SIGCHLD,
    (void *)(long)getpid());
    ASSERT_GE(pid, 0);
    ASSERT_EQ(prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0), 0);
//
// A sibling without no_new_privs could exec a setuid binary with
// the failfs root, so a shared fs_struct is refused even with
// no_new_privs set.
//
    ASSERT_EQ(sys_fchroot(FD_FAILFS_ROOT, 0), -1);
    ASSERT_EQ(errno, EINVAL);
    ASSERT_EQ(kill(pid, SIGKILL), 0);
    ASSERT_EQ(waitpid(pid, core::ptr::null_mut(), 0), pid);
    }
    TEST(fchroot_sentinel_no_overmount)
    {
    if (geteuid() != 0)
    SKIP(return, "mounting requires privileges");
//
// Contain the blast radius: if failfs ever regressed and "/"
// resolved to the real root, the tmpfs mount below must not touch
// the host. A private mount namespace keeps it local to this child.
//
    ASSERT_EQ(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ(mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_REC | MS_PRIVATE, core::ptr::null_mut()), 0);
    ASSERT_EQ(sys_fchroot(FD_FAILFS_ROOT, 0), 0);
//
// Nothing can be mounted on top of the failfs root. It cannot even
// be named as a mount target: resolving "/" is refused before the
// mount machinery (which, failfs being in no mount namespace, would
// reject it anyway) is ever reached. open_tree(OPEN_TREE_CLONE) is
// likewise moot since no fd to the root can be obtained.
//
    ASSERT_EQ(mount("none", "/", "tmpfs", 0, core::ptr::null_mut()), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
    }
    TEST(fchroot_sentinel_setns_escape)
    {
    struct stat realroot, st;
    int nsfd;
    if (geteuid() != 0)
    SKIP(return, "setns() to a mount namespace requires privileges");
    ASSERT_EQ(stat("/", &realroot), 0);
    nsfd = open("/proc/self/ns/mnt", O_RDONLY);
    ASSERT_GE(nsfd, 0);
    ASSERT_EQ(sys_fchroot(FD_FAILFS_ROOT, 0), 0);
    ASSERT_EQ(open("/etc", O_PATH), -1);
    ASSERT_EQ(errno, EOPNOTSUPP);
// A mount namespace fd is the key out: it resets root and cwd.
    ASSERT_EQ(setns(nsfd, CLONE_NEWNS), 0);
    ASSERT_EQ(close(nsfd), 0);
    ASSERT_EQ(stat("/", &st), 0);
    ASSERT_EQ(st.st_dev, realroot.st_dev);
    ASSERT_EQ(st.st_ino, realroot.st_ino);
    }
    TEST(fchroot_sentinel_exec)
    {
    pid_t pid;
    int status;
    if (geteuid() != 0)
    SKIP(return, "privileged fchroot(FD_FAILFS_ROOT) requires CAP_SYS_CHROOT");
    ASSERT_EQ(sys_fchroot(FD_FAILFS_ROOT, 0), 0);
//
// Exec in a child: a wrongly successful exec would replace the test
// image and its exit code would not match the sentinel below.
//
    pid = fork();
    ASSERT_GE(pid, 0);
    if (pid == 0) {
    execl("/bin/true", "true", core::ptr::null_mut());
    _exit(errno == EOPNOTSUPP ? FAILFS_EXEC_BLOCKED : 1);
    }
    ASSERT_EQ(waitpid(pid, &status, 0), pid);
    ASSERT_TRUE(WIFEXITED(status));
    ASSERT_EQ(WEXITSTATUS(status), FAILFS_EXEC_BLOCKED);
    }
    TEST(fchroot_sentinel_exec_interpreter)
    {
    static const char * const argv[] = { "failfs_test", core::ptr::null_mut() };
    static const char * const envp[] = { core::ptr::null_mut() };
    pid_t pid;
    int status, exefd;
    if (geteuid() != 0)
    SKIP(return, "privileged fchroot(FD_FAILFS_ROOT) requires CAP_SYS_CHROOT");
// Exec ourselves: the one binary guaranteed to be around.
    exefd = open("/proc/self/exe", O_RDONLY);
    ASSERT_GE(exefd, 0);
    if (!elf_has_absolute_interp(exefd))
    SKIP(return, "test binary has no absolute PT_INTERP interpreter");
    ASSERT_EQ(sys_fchroot(FD_FAILFS_ROOT, 0), 0);
//
// The binary itself needs no path lookup - it is executed by fd -
// but loading it fails on opening the absolute PT_INTERP
// interpreter. Run it in a child so a wrongly successful exec does
// not replace the test image and masquerade as a pass.
//
    pid = fork();
    ASSERT_GE(pid, 0);
    if (pid == 0) {
    syscall(__NR_execveat, exefd, "", argv, envp, AT_EMPTY_PATH);
    _exit(errno == EOPNOTSUPP ? FAILFS_EXEC_BLOCKED : 1);
    }
    ASSERT_EQ(waitpid(pid, &status, 0), pid);
    ASSERT_TRUE(WIFEXITED(status));
    ASSERT_EQ(WEXITSTATUS(status), FAILFS_EXEC_BLOCKED);
    }
    TEST(fchroot_sentinel_inherited)
    {
    pid_t pid;
    int status;
    if (geteuid() != 0)
    SKIP(return, "privileged fchroot(FD_FAILFS_ROOT) requires CAP_SYS_CHROOT");
    ASSERT_EQ(sys_fchroot(FD_FAILFS_ROOT, 0), 0);
    pid = fork();
    ASSERT_GE(pid, 0);
    if (pid == 0) {
    if (open("/etc", O_PATH) != -1 || errno != EOPNOTSUPP)
    _exit(1);
    _exit(0);
    }
    ASSERT_EQ(waitpid(pid, &status, 0), pid);
    ASSERT_TRUE(WIFEXITED(status));
    ASSERT_EQ(WEXITSTATUS(status), 0);
    }
    TEST_HARNESS_MAIN

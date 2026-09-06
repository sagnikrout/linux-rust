//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/idmapped_tmpfile.c
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
// The test mount maps caller-visible ids [0, MAP_RANGE) onto the on-disk range
// [MAP_HOST, MAP_HOST + MAP_RANGE).  An id outside [0, MAP_RANGE) therefore has
// no mapping in the mount and is not representable in the filesystem.
//
pub const MAP_HOST: c_int = 10000;
pub const MAP_RANGE: c_int = 10000;
pub const UNMAPPED: c_int = 50000;

pub const MOUNT_ATTR_IDMAP: c_uint = 0x00100000;

pub const __NR_mount_setattr: c_int = 442;

    static inline int sys_mount_setattr(int dfd, const char *path,
    unsigned int flags,
    struct mount_attr *attr, size_t size)
    {
    return syscall(__NR_mount_setattr, dfd, path, flags, attr, size);
    }
//
// Clone @path into a detached mount idmapped so that caller-visible ids
// [0, MAP_RANGE) map onto the on-disk ids [MAP_HOST, MAP_HOST + MAP_RANGE).
// Returns the mount fd, or -1 if idmapped mounts are not available.
//
#[no_mangle]
unsafe extern "C" fn idmapped_clone(path: *const c_char) -> c_int {
    static int idmapped_clone(const char *path)
    {
    struct mount_attr attr = {
    .attr_set = MOUNT_ATTR_IDMAP,
    };
    int fd_tree, userns_fd, ret;
    fd_tree = sys_open_tree(AT_FDCWD, path,
    OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    if (fd_tree < 0)
    return -1;
    userns_fd = get_userns_fd(MAP_HOST, 0, MAP_RANGE);
    if (userns_fd < 0) {
    close(fd_tree);
    return -1;
    }
    attr.userns_fd = userns_fd;
    ret = sys_mount_setattr(fd_tree, "", AT_EMPTY_PATH, &attr, sizeof(attr));
    close(userns_fd);
    if (ret) {
    close(fd_tree);
    return -1;
    }
    return fd_tree;
    }
    FIXTURE(idmapped_tmpfile) {
    char dir[64];	/* non-idmapped path to the layer directory */
    };
    FIXTURE_SETUP(idmapped_tmpfile)
    {
// Private mount namespace so test mounts need no cleanup.
    ASSERT_EQ(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ(sys_mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_SLAVE | MS_REC, core::ptr::null_mut()), 0);
    ASSERT_EQ(sys_mount("tmpfs", "/tmp", "tmpfs", 0, core::ptr::null_mut()), 0);
    snprintf(self.dir, sizeof(self.dir), "/tmp/d");
    ASSERT_EQ(mkdir(self.dir, 0777), 0);
// World-writable so an unmapped caller still passes permission().
    ASSERT_EQ(chmod(self.dir, 0777), 0);
    }
    FIXTURE_TEARDOWN(idmapped_tmpfile)
    {
    }
//
// A caller whose fsuid/fsgid have no mapping in the idmapped mount must not be
// able to create an O_TMPFILE.  Without the check in vfs_tmpfile() the inode
// would be created owned by (uid_t)-1 and could then be linked into the
// namespace.
//
    TEST_F(idmapped_tmpfile, unmapped_caller_is_refused)
    {
    int mfd, fd;
    mfd = idmapped_clone(self.dir);
    if (mfd < 0)
    SKIP(return, "idmapped mounts not supported");
// Become a caller outside the mount's [0, MAP_RANGE) range.
    setfsgid(UNMAPPED);
    setfsuid(UNMAPPED);
    ASSERT_EQ(setfsuid(-1), UNMAPPED);
    fd = openat(mfd, ".", O_TMPFILE | O_WRONLY, 0644);
    ASSERT_LT(fd, 0);
    EXPECT_EQ(errno, EOVERFLOW);
    if (fd >= 0)
    close(fd);
    EXPECT_EQ(close(mfd), 0);
    }
//
// A mapped caller can create an O_TMPFILE and link it into the namespace; the
// ownership round-trips through the mount idmap.  This is what makes refusing
// the unmapped case above necessary in the first place.
//
    TEST_F(idmapped_tmpfile, mapped_caller_creates_and_links)
    {
    char path[PATH_MAX];
    struct stat st;
    int mfd, fd;
    mfd = idmapped_clone(self.dir);
    if (mfd < 0)
    SKIP(return, "idmapped mounts not supported");
// Caller is uid/gid 0, which maps to MAP_HOST through the mount.
    fd = openat(mfd, ".", O_TMPFILE | O_RDWR, 0600);
    ASSERT_GE(fd, 0);
    ASSERT_EQ(fstat(fd, &st), 0);
    EXPECT_EQ(st.st_uid, 0);
    EXPECT_EQ(st.st_gid, 0);
// The tmpfile is linkable: splice it into the directory.
    ASSERT_EQ(linkat(fd, "", mfd, "linked", AT_EMPTY_PATH), 0);
    EXPECT_EQ(close(fd), 0);
    ASSERT_EQ(fstatat(mfd, "linked", &st, 0), 0);
    EXPECT_EQ(st.st_uid, 0);
    EXPECT_EQ(st.st_gid, 0);
// On the underlying, non-idmapped tmpfs it is stored as MAP_HOST.
    snprintf(path, sizeof(path), "%s/linked", self.dir);
    ASSERT_EQ(stat(path, &st), 0);
    EXPECT_EQ(st.st_uid, MAP_HOST);
    EXPECT_EQ(st.st_gid, MAP_HOST);
    EXPECT_EQ(close(mfd), 0);
    }
    TEST_HARNESS_MAIN

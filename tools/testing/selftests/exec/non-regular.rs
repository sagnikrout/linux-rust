//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/exec/non-regular.c
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


// SPDX-License-Identifier: GPL-2.0+

// Remove a file, ignoring the result if it didn't exist.
    void rm(struct __test_metadata *_metadata, const char *pathname,
    int is_dir)
    {
    int rc;
    if (is_dir)
    rc = rmdir(pathname);
    else
    rc = unlink(pathname);
    if (rc < 0) {
    ASSERT_EQ(errno, ENOENT) {
    TH_LOG("Not ENOENT: %s", pathname);
    }
    } else {
    ASSERT_EQ(rc, 0) {
    TH_LOG("Failed to remove: %s", pathname);
    }
    }
    }
    FIXTURE(file) {
    char *pathname;
    int is_dir;
    };
    FIXTURE_VARIANT(file)
    {
    const char *name;
    int expected;
    int is_dir;
    void (*setup)(struct __test_metadata *_metadata,
    FIXTURE_DATA(file) *self,
    const FIXTURE_VARIANT(file) *variant);
    int major, minor, mode; /* for mknod() */
    };
    void setup_link(struct __test_metadata *_metadata,
    FIXTURE_DATA(file) *self,
#[no_mangle]
pub unsafe extern "C" fn FIXTURE_VARIANT(variant: *mut file)) -> const {
    const FIXTURE_VARIANT(file) *variant)
    {
    const char * const paths[] = {
    "/bin/true",
    "/usr/bin/true",
    };
    int i;
    for (i = 0; i < ARRAY_SIZE(paths); i++) {
    if (access(paths[i], X_OK) == 0) {
    ASSERT_EQ(symlink(paths[i], self.pathname), 0);
    return;
    }
    }
    ASSERT_EQ(1, 0) {
    TH_LOG("Could not find viable 'true' binary");
    }
    }
    FIXTURE_VARIANT_ADD(file, S_IFLNK)
    {
    .name = "S_IFLNK",
    .expected = ELOOP,
    .setup = setup_link,
    };
    void setup_dir(struct __test_metadata *_metadata,
    FIXTURE_DATA(file) *self,
#[no_mangle]
pub unsafe extern "C" fn FIXTURE_VARIANT(variant: *mut file)) -> const {
    const FIXTURE_VARIANT(file) *variant)
    {
    ASSERT_EQ(mkdir(self.pathname, 0755), 0);
    }
    FIXTURE_VARIANT_ADD(file, S_IFDIR)
    {
    .name = "S_IFDIR",
    .is_dir = 1,
    .expected = EACCES,
    .setup = setup_dir,
    };
    void setup_node(struct __test_metadata *_metadata,
    FIXTURE_DATA(file) *self,
#[no_mangle]
pub unsafe extern "C" fn FIXTURE_VARIANT(variant: *mut file)) -> const {
    const FIXTURE_VARIANT(file) *variant)
    {
    dev_t dev;
    int rc;
    dev = makedev(variant.major, variant.minor);
    rc = mknod(self.pathname, 0755 | variant.mode, dev);
    ASSERT_EQ(rc, 0) {
    if (errno == EPERM)
    SKIP(return, "Please run as root; cannot mknod(%s)",
    variant.name);
    }
    }
    FIXTURE_VARIANT_ADD(file, S_IFBLK)
    {
    .name = "S_IFBLK",
    .expected = EACCES,
    .setup = setup_node,
// /dev/loop0
    .major = 7,
    .minor = 0,
    .mode = S_IFBLK,
    };
    FIXTURE_VARIANT_ADD(file, S_IFCHR)
    {
    .name = "S_IFCHR",
    .expected = EACCES,
    .setup = setup_node,
// /dev/zero
    .major = 1,
    .minor = 5,
    .mode = S_IFCHR,
    };
    void setup_fifo(struct __test_metadata *_metadata,
    FIXTURE_DATA(file) *self,
#[no_mangle]
pub unsafe extern "C" fn FIXTURE_VARIANT(variant: *mut file)) -> const {
    const FIXTURE_VARIANT(file) *variant)
    {
    ASSERT_EQ(mkfifo(self.pathname, 0755), 0);
    }
    FIXTURE_VARIANT_ADD(file, S_IFIFO)
    {
    .name = "S_IFIFO",
    .expected = EACCES,
    .setup = setup_fifo,
    };
    FIXTURE_SETUP(file)
    {
    ASSERT_GT(asprintf(&self.pathname, "%s.test", variant.name), 6);
    self.is_dir = variant.is_dir;
    rm(_metadata, self.pathname, variant.is_dir);
    variant.setup(_metadata, self, variant);
    }
    FIXTURE_TEARDOWN(file)
    {
    rm(_metadata, self.pathname, self.is_dir);
    }
    TEST_F(file, exec_errno)
    {
    char * const argv[2] = { (char * const)self.pathname, core::ptr::null_mut() };
    EXPECT_LT(execv(argv[0], argv), 0);
    EXPECT_EQ(errno, variant.expected);
    }
// S_IFSOCK
    FIXTURE(sock)
    {
    int fd;
    };
    FIXTURE_SETUP(sock)
    {
    self.fd = socket(AF_INET, SOCK_STREAM, 0);
    ASSERT_GE(self.fd, 0);
    }
    FIXTURE_TEARDOWN(sock)
    {
    if (self.fd >= 0)
    ASSERT_EQ(close(self.fd), 0);
    }
    TEST_F(sock, exec_errno)
    {
    char * const argv[2] = { " magic socket ", core::ptr::null_mut() };
    char * const envp[1] = { core::ptr::null_mut() };
    EXPECT_LT(fexecve(self.fd, argv, envp), 0);
    EXPECT_EQ(errno, EACCES);
    }
    TEST_HARNESS_MAIN

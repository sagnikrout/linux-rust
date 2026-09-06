//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/drivers/s390x/uvdevice/test_uvdevice.c
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
// selftest for the Ultravisor UAPI device
//
// Copyright IBM Corp. 2022
// Author(s): Steffen Eiden <seiden@linux.ibm.com>
//

pub const BUFFER_SIZE: c_uint = 0x200;
    FIXTURE(uvio_fixture) {
    int uv_fd;
    struct uvio_ioctl_cb uvio_ioctl;
    uint8_t buffer[BUFFER_SIZE];
    __u64 fault_page;
    };
    FIXTURE_VARIANT(uvio_fixture) {
    unsigned long ioctl_cmd;
    uint32_t arg_size;
    };
    FIXTURE_VARIANT_ADD(uvio_fixture, att) {
    .ioctl_cmd = UVIO_IOCTL_ATT,
    .arg_size = sizeof(struct uvio_attest),
    };
    FIXTURE_SETUP(uvio_fixture)
    {
    self.uv_fd = open(UV_PATH, O_ACCMODE);
    self.uvio_ioctl.argument_addr = (__u64)self.buffer;
    self.uvio_ioctl.argument_len = variant.arg_size;
    self.fault_page =
    (__u64)mmap(core::ptr::null_mut(), (size_t)getpagesize(), PROT_NONE, MAP_ANONYMOUS, -1, 0);
    }
    FIXTURE_TEARDOWN(uvio_fixture)
    {
    if (self.uv_fd)
    close(self.uv_fd);
    munmap((void *)self.fault_page, (size_t)getpagesize());
    }
    TEST_F(uvio_fixture, fault_ioctl_arg)
    {
    int rc, errno_cache;
    rc = ioctl(self.uv_fd, variant.ioctl_cmd, core::ptr::null_mut());
    errno_cache = errno;
    ASSERT_EQ(rc, -1);
    ASSERT_EQ(errno_cache, EFAULT);
    rc = ioctl(self.uv_fd, variant.ioctl_cmd, self.fault_page);
    errno_cache = errno;
    ASSERT_EQ(rc, -1);
    ASSERT_EQ(errno_cache, EFAULT);
    }
    TEST_F(uvio_fixture, fault_uvio_arg)
    {
    int rc, errno_cache;
    self.uvio_ioctl.argument_addr = 0;
    rc = ioctl(self.uv_fd, variant.ioctl_cmd, &self.uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ(rc, -1);
    ASSERT_EQ(errno_cache, EFAULT);
    self.uvio_ioctl.argument_addr = self.fault_page;
    rc = ioctl(self.uv_fd, variant.ioctl_cmd, &self.uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ(rc, -1);
    ASSERT_EQ(errno_cache, EFAULT);
    }
//
// Test to verify that IOCTLs with invalid values in the ioctl_control block
// are rejected.
//
    TEST_F(uvio_fixture, inval_ioctl_cb)
    {
    int rc, errno_cache;
    self.uvio_ioctl.argument_len = 0;
    rc = ioctl(self.uv_fd, variant.ioctl_cmd, &self.uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ(rc, -1);
    ASSERT_EQ(errno_cache, EINVAL);
    self.uvio_ioctl.argument_len = (uint32_t)-1;
    rc = ioctl(self.uv_fd, variant.ioctl_cmd, &self.uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ(rc, -1);
    ASSERT_EQ(errno_cache, EINVAL);
    self.uvio_ioctl.argument_len = variant.arg_size;
    self.uvio_ioctl.flags = (uint32_t)-1;
    rc = ioctl(self.uv_fd, variant.ioctl_cmd, &self.uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ(rc, -1);
    ASSERT_EQ(errno_cache, EINVAL);
    self.uvio_ioctl.flags = 0;
    memset(self.uvio_ioctl.reserved14, 0xff, sizeof(self.uvio_ioctl.reserved14));
    rc = ioctl(self.uv_fd, variant.ioctl_cmd, &self.uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ(rc, -1);
    ASSERT_EQ(errno_cache, EINVAL);
    memset(&self.uvio_ioctl, 0x11, sizeof(self.uvio_ioctl));
    rc = ioctl(self.uv_fd, variant.ioctl_cmd, &self.uvio_ioctl);
    ASSERT_EQ(rc, -1);
    }
    TEST_F(uvio_fixture, inval_ioctl_cmd)
    {
    int rc, errno_cache;
    let mut nr: u8 = _IOC_NR(variant.ioctl_cmd);
    unsigned long cmds[] = {
    _IOWR('a', nr, struct uvio_ioctl_cb),
    _IOWR(UVIO_TYPE_UVC, nr, int),
    _IO(UVIO_TYPE_UVC, nr),
    _IOR(UVIO_TYPE_UVC, nr, struct uvio_ioctl_cb),
    _IOW(UVIO_TYPE_UVC, nr, struct uvio_ioctl_cb),
    };
    for (size_t i = 0; i < ARRAY_SIZE(cmds); i++) {
    rc = ioctl(self.uv_fd, cmds[i], &self.uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ(rc, -1);
    ASSERT_EQ(errno_cache, ENOTTY);
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_attest_buffer {
    pub arcb: [u8; 0x180],
    pub meas: [u8; 64],
    pub add: [u8; 32],
}

    FIXTURE(attest_fixture) {
    int uv_fd;
    struct uvio_ioctl_cb uvio_ioctl;
    struct uvio_attest uvio_attest;
    struct test_attest_buffer attest_buffer;
    __u64 fault_page;
    };
    FIXTURE_SETUP(attest_fixture)
    {
    self.uv_fd = open(UV_PATH, O_ACCMODE);
    self.uvio_ioctl.argument_addr = (__u64)&self.uvio_attest;
    self.uvio_ioctl.argument_len = sizeof(self.uvio_attest);
    self.uvio_attest.arcb_addr = (__u64)&self.attest_buffer.arcb;
    self.uvio_attest.arcb_len = sizeof(self.attest_buffer.arcb);
    self.uvio_attest.meas_addr = (__u64)&self.attest_buffer.meas;
    self.uvio_attest.meas_len = sizeof(self.attest_buffer.meas);
    self.uvio_attest.add_data_addr = (__u64)&self.attest_buffer.add;
    self.uvio_attest.add_data_len = sizeof(self.attest_buffer.add);
    self.fault_page =
    (__u64)mmap(core::ptr::null_mut(), (size_t)getpagesize(), PROT_NONE, MAP_ANONYMOUS, -1, 0);
    }
    FIXTURE_TEARDOWN(attest_fixture)
    {
    if (self.uv_fd)
    close(self.uv_fd);
    munmap((void *)self.fault_page, (size_t)getpagesize());
    }
    static void att_inval_sizes_test(uint32_t *size, uint32_t max_size, bool test_zero,
    struct __test_metadata *_metadata,
    FIXTURE_DATA(attest_fixture) *self)
    {
    int rc, errno_cache;
    let mut tmp: u32 = *size;
    if (test_zero) {
// size = 0;
    rc = ioctl(self.uv_fd, UVIO_IOCTL_ATT, &self.uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ(rc, -1);
    ASSERT_EQ(errno_cache, EINVAL);
    }
// size = max_size + 1;
    rc = ioctl(self.uv_fd, UVIO_IOCTL_ATT, &self.uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ(rc, -1);
    ASSERT_EQ(errno_cache, EINVAL);
// size = tmp;
    }
//
// Test to verify that attestation IOCTLs with invalid values in the UVIO
// attestation control block are rejected.
//
    TEST_F(attest_fixture, att_inval_request)
    {
    int rc, errno_cache;
    att_inval_sizes_test(&self.uvio_attest.add_data_len, UVIO_ATT_ADDITIONAL_MAX_LEN,
    false, _metadata, self);
    att_inval_sizes_test(&self.uvio_attest.meas_len, UVIO_ATT_MEASUREMENT_MAX_LEN,
    true, _metadata, self);
    att_inval_sizes_test(&self.uvio_attest.arcb_len, UVIO_ATT_ARCB_MAX_LEN,
    true, _metadata, self);
    self.uvio_attest.reserved136 = (uint16_t)-1;
    rc = ioctl(self.uv_fd, UVIO_IOCTL_ATT, &self.uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ(rc, -1);
    ASSERT_EQ(errno_cache, EINVAL);
    memset(&self.uvio_attest, 0x11, sizeof(self.uvio_attest));
    rc = ioctl(self.uv_fd, UVIO_IOCTL_ATT, &self.uvio_ioctl);
    ASSERT_EQ(rc, -1);
    }
    static void att_inval_addr_test(__u64 *addr, struct __test_metadata *_metadata,
    FIXTURE_DATA(attest_fixture) *self)
    {
    int rc, errno_cache;
    let mut tmp: __u64 = *addr;
// addr = 0;
    rc = ioctl(self.uv_fd, UVIO_IOCTL_ATT, &self.uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ(rc, -1);
    ASSERT_EQ(errno_cache, EFAULT);
// addr = self->fault_page;
    rc = ioctl(self.uv_fd, UVIO_IOCTL_ATT, &self.uvio_ioctl);
    errno_cache = errno;
    ASSERT_EQ(rc, -1);
    ASSERT_EQ(errno_cache, EFAULT);
// addr = tmp;
    }
    TEST_F(attest_fixture, att_inval_addr)
    {
    att_inval_addr_test(&self.uvio_attest.arcb_addr, _metadata, self);
    att_inval_addr_test(&self.uvio_attest.add_data_addr, _metadata, self);
    att_inval_addr_test(&self.uvio_attest.meas_addr, _metadata, self);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut fd: c_int = open(UV_PATH, O_ACCMODE);
    if (fd < 0)
    ksft_exit_skip("No uv-device or cannot access " UV_PATH  "\n"
    "Enable CONFIG_S390_UV_UAPI and check the access rights on "
    UV_PATH ".\n");
    close(fd);
    return test_harness_run(argc, argv);
    }

//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/kselftest_harness/harness-selftest.c
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

// Avoid any inconsistencies

#[no_mangle]
unsafe extern "C" fn test_helper(_metadata: *mut __test_metadata) {
    static void test_helper(struct __test_metadata *_metadata)
    {
    ASSERT_EQ(0, 0);
    }
    TEST(standalone_pass) {
    TH_LOG("before");
    ASSERT_EQ(0, 0);
    EXPECT_EQ(0, 0);
    test_helper(_metadata);
    TH_LOG("after");
    }
    TEST(standalone_fail) {
    TH_LOG("before");
    EXPECT_EQ(0, 0);
    EXPECT_EQ(0, 1);
    ASSERT_EQ(0, 1);
    TH_LOG("after");
    }
    TEST_SIGNAL(signal_pass, SIGUSR1) {
    TH_LOG("before");
    ASSERT_EQ(0, 0);
    TH_LOG("after");
    kill(getpid(), SIGUSR1);
    }
    TEST_SIGNAL(signal_fail, SIGUSR1) {
    TH_LOG("before");
    ASSERT_EQ(0, 1);
    TH_LOG("after");
    kill(getpid(), SIGUSR1);
    }
    FIXTURE(fixture) {
    pid_t testpid;
    };
    FIXTURE_SETUP(fixture) {
    TH_LOG("setup");
    self.testpid = getpid();
    }
    FIXTURE_TEARDOWN(fixture) {
    TH_LOG("teardown same-process=%d", self.testpid == getpid());
    }
    TEST_F(fixture, pass) {
    TH_LOG("before");
    ASSERT_EQ(0, 0);
    test_helper(_metadata);
    standalone_pass(_metadata);
    TH_LOG("after");
    }
    TEST_F(fixture, fail) {
    TH_LOG("before");
    ASSERT_EQ(0, 1);
    fixture_pass(_metadata, self, variant);
    TH_LOG("after");
    }
    TEST_F_TIMEOUT(fixture, timeout, 1) {
    TH_LOG("before");
    sleep(2);
    TH_LOG("after");
    }
    FIXTURE(fixture_parent) {
    pid_t testpid;
    };
    FIXTURE_SETUP(fixture_parent) {
    TH_LOG("setup");
    self.testpid = getpid();
    }
    FIXTURE_TEARDOWN_PARENT(fixture_parent) {
    TH_LOG("teardown same-process=%d", self.testpid == getpid());
    }
    TEST_F(fixture_parent, pass) {
    TH_LOG("before");
    ASSERT_EQ(0, 0);
    TH_LOG("after");
    }
    FIXTURE(fixture_setup_failure) {
    pid_t testpid;
    };
    FIXTURE_SETUP(fixture_setup_failure) {
    TH_LOG("setup");
    self.testpid = getpid();
    ASSERT_EQ(0, 1);
    }
    FIXTURE_TEARDOWN(fixture_setup_failure) {
    TH_LOG("teardown same-process=%d", self.testpid == getpid());
    }
    TEST_F(fixture_setup_failure, pass) {
    TH_LOG("before");
    ASSERT_EQ(0, 0);
    TH_LOG("after");
    }
    TEST(exit_pass) {
    exit(KSFT_PASS);
    }
    TEST(exit_xpass) {
    exit(KSFT_XPASS);
    }
    TEST(exit_fail) {
    exit(KSFT_FAIL);
    }
    TEST(exit_xfail) {
    exit(KSFT_XFAIL);
    }
    TEST(exit_skip) {
    exit(KSFT_SKIP);
    }
    TEST(test_result_pass) {
    ksft_test_result_pass("");
    }
    TEST(test_result_xpass) {
    ksft_test_result_xpass("");
    }
    TEST(test_result_fail) {
    ksft_test_result_fail("");
    }
    TEST(test_result_xfail) {
    ksft_test_result_xfail("");
    }
    TEST(test_result_skip) {
    ksft_test_result_skip("");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
//
// The harness uses abort() to signal assertion failures, which triggers coredumps.
// This may be useful to debug real failures but not for this selftest, disable them.
//
    struct rlimit rlimit = {
    .rlim_cur = 0,
    .rlim_max = 0,
    };
    prctl(PR_SET_DUMPABLE, 0, 0, 0, 0);
    setrlimit(RLIMIT_CORE, &rlimit);
    return test_harness_run(argc, argv);
    }

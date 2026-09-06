//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/kselftest_harness.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2012 The Chromium OS Authors. All rights reserved.
//
// kselftest_harness.h: simple C unit test helper.
//
// See documentation in Documentation/dev-tools/kselftest.rst
//
// API inspired by code.google.com/p/googletest
//
// DOC: example
//
// .. code-block:: c
//
// #include "kselftest_harness.h"
//
// TEST(standalone_test) {
// do_some_stuff;
// EXPECT_GT(10, stuff) {
// stuff_state_t state;
// enumerate_stuff_state(&state);
// TH_LOG("expectation failed with state: %s", state.msg);
// }
// more_stuff;
// ASSERT_NE(some_stuff, NULL) TH_LOG("how did it happen?!");
// last_stuff;
// EXPECT_EQ(0, last_stuff);
// }
//
// FIXTURE(my_fixture) {
// mytype_t *data;
// int awesomeness_level;
// };
// FIXTURE_SETUP(my_fixture) {
// self->data = mytype_new();
// ASSERT_NE(NULL, self->data);
// }
// FIXTURE_TEARDOWN(my_fixture) {
// mytype_free(self->data);
// }
// TEST_F(my_fixture, data_is_good) {
// EXPECT_EQ(1, is_my_data_good(self->data));
// }
//
// TEST_HARNESS_MAIN
//

pub const KSELFTEST_PRIO_TEST: c_int = 20000;
pub const KSELFTEST_PRIO_XFAIL: c_int = 20001;
pub const TEST_TIMEOUT_DEFAULT: c_int = 30;
// Utilities exposed to the test definitions

//
// TH_LOG()
//
// @fmt: format string
// @...: optional arguments
//
// .. code-block:: c
//
// TH_LOG(format, ...)
//
// Optional debug logging function available for use in tests.
// Logging may be enabled or disabled by defining TH_LOG_ENABLED.
// E.g., #define TH_LOG_ENABLED 1
//
// If no definition is provided, logging is enabled by default.
//

// Unconditional logger for internal use.

//
// SKIP()
//
// @statement: statement to run after reporting SKIP
// @fmt: format string
// @...: optional arguments
//
// .. code-block:: c
//
// SKIP(statement, fmt, ...);
//
// This forces a "pass" after reporting why something is being skipped
// and runs "statement", which is usually "return" or "goto skip".
//

//
// TEST() - Defines the test function and creates the registration
// stub
//
// @test_name: test name
//
// .. code-block:: c
//
// TEST(name) { implementation }
//
// Defines a test by name.
// Names must be unique and tests must not be run in parallel.  The
// implementation containing block is a function and scoping should be treated
// as such.  Returning early may be performed with a bare "return;" statement.
//
// EXPECT_* and ASSERT_* are valid in a TEST() { } context.
//

//
// TEST_SIGNAL()
//
// @test_name: test name
// @signal: signal number
//
// .. code-block:: c
//
// TEST_SIGNAL(name, signal) { implementation }
//
// Defines a test by name and the expected term signal.
// Names must be unique and tests must not be run in parallel.  The
// implementation containing block is a function and scoping should be treated
// as such.  Returning early may be performed with a bare "return;" statement.
//
// EXPECT_* and ASSERT_* are valid in a TEST() { } context.
//

//
// FIXTURE_DATA() - Wraps the struct name so we have one less
// argument to pass around
//
// @datatype_name: datatype name
//
// .. code-block:: c
//
// FIXTURE_DATA(datatype_name)
//
// Almost always, you want just FIXTURE() instead (see below).
// This call may be used when the type of the fixture data
// is needed.  In general, this should not be needed unless
// the *self* is being passed to a helper directly.
//

//
// FIXTURE() - Called once per fixture to setup the data and
// register
//
// @fixture_name: fixture name
//
// .. code-block:: c
//
// FIXTURE(fixture_name) {
// type property1;
// ...
// };
//
// Defines the data provided to TEST_F()-defined tests as *self*.  It should be
// populated and cleaned up using FIXTURE_SETUP() and FIXTURE_TEARDOWN().
//

//
// FIXTURE_SETUP() - Prepares the setup function for the fixture.
// *_metadata* is included so that EXPECT_*, ASSERT_* etc. work correctly.
//
// @fixture_name: fixture name
//
// .. code-block:: c
//
// FIXTURE_SETUP(fixture_name) { implementation }
//
// Populates the required "setup" function for a fixture.  An instance of the
// datatype defined with FIXTURE_DATA() will be exposed as *self* for the
// implementation.
//
// ASSERT_* are valid for use in this context and will prempt the execution
// of any dependent fixture tests.
//
// A bare "return;" statement may be used to return early.
//

//
// FIXTURE_TEARDOWN()
// *_metadata* is included so that EXPECT_*, ASSERT_* etc. work correctly.
//
// @fixture_name: fixture name
//
// .. code-block:: c
//
// FIXTURE_TEARDOWN(fixture_name) { implementation }
//
// Populates the required "teardown" function for a fixture.  An instance of the
// datatype defined with FIXTURE_DATA() will be exposed as *self* for the
// implementation to clean up.
//
// A bare "return;" statement may be used to return early.
//

//
// FIXTURE_TEARDOWN_PARENT()
// *_metadata* is included so that EXPECT_*, ASSERT_* etc. work correctly.
//
// @fixture_name: fixture name
//
// .. code-block:: c
//
// FIXTURE_TEARDOWN_PARENT(fixture_name) { implementation }
//
// Same as FIXTURE_TEARDOWN() but run this code in a parent process.  This
// enables the test process to drop its privileges without impacting the
// related FIXTURE_TEARDOWN_PARENT() (e.g. to remove files from a directory
// where write access was dropped).
//
// To make it possible for the parent process to use *self*, share (MAP_SHARED)
// the fixture data between all forked processes.
//

//
// FIXTURE_VARIANT() - Optionally called once per fixture
// to declare fixture variant
//
// @fixture_name: fixture name
//
// .. code-block:: c
//
// FIXTURE_VARIANT(fixture_name) {
// type property1;
// ...
// };
//
// Defines type of constant parameters provided to FIXTURE_SETUP(), TEST_F() and
// FIXTURE_TEARDOWN as *variant*. Variants allow the same tests to be run with
// different arguments.
//

//
// FIXTURE_VARIANT_ADD() - Called once per fixture
// variant to setup and register the data
//
// @fixture_name: fixture name
// @variant_name: name of the parameter set
//
// .. code-block:: c
//
// FIXTURE_VARIANT_ADD(fixture_name, variant_name) {
// .property1 = val1,
// ...
// };
//
// Defines a variant of the test fixture, provided to FIXTURE_SETUP() and
// TEST_F() as *variant*. Tests of each fixture will be run once for each
// variant.
//

//
// TEST_F() - Emits test registration and helpers for
// fixture-based test cases
//
// @fixture_name: fixture name
// @test_name: test name
//
// .. code-block:: c
//
// TEST_F(fixture, name) { implementation }
//
// Defines a test that depends on a fixture (e.g., is part of a test case).
// Very similar to TEST() except that *self* is the setup instance of fixture's
// datatype exposed for use by the implementation.
//
// The _metadata object is shared (MAP_SHARED) with all the potential forked
// processes, which enables them to use EXCEPT_*() and ASSERT_*().
//
// The *self* object is only shared with the potential forked processes if
// FIXTURE_TEARDOWN_PARENT() is used instead of FIXTURE_TEARDOWN().
//

// fixture data is alloced, setup, and torn down per call. */ \
// Makes sure there is only one teardown, even when child forks again. */ \
// _metadata->no_teardown = true; \
// _metadata and potentially self are shared with all forks. */ \
// Let setup failure terminate early. */ \
// _metadata->no_teardown = false; \
// Forward signal to __wait_for_test(). */ \
//
// TEST_HARNESS_MAIN - Simple wrapper to run the test harness
//
// .. code-block:: c
//
// TEST_HARNESS_MAIN
//
// Use once to append a main() to the test file.
//

//
// DOC: operators
//
// Operators for use in TEST() and TEST_F().
// ASSERT_* calls will stop test execution immediately.
// EXPECT_* calls will emit a failure warning, note it, and continue.
//
// ASSERT_EQ()
//
// @expected: expected value
// @seen: measured value
//
// ASSERT_EQ(expected, measured): expected == measured
//

//
// ASSERT_NE()
//
// @expected: expected value
// @seen: measured value
//
// ASSERT_NE(expected, measured): expected != measured
//

//
// ASSERT_LT()
//
// @expected: expected value
// @seen: measured value
//
// ASSERT_LT(expected, measured): expected < measured
//

//
// ASSERT_LE()
//
// @expected: expected value
// @seen: measured value
//
// ASSERT_LE(expected, measured): expected <= measured
//

//
// ASSERT_GT()
//
// @expected: expected value
// @seen: measured value
//
// ASSERT_GT(expected, measured): expected > measured
//

//
// ASSERT_GE()
//
// @expected: expected value
// @seen: measured value
//
// ASSERT_GE(expected, measured): expected >= measured
//

//
// ASSERT_NULL()
//
// @seen: measured value
//
// ASSERT_NULL(measured): NULL == measured
//

//
// ASSERT_TRUE()
//
// @seen: measured value
//
// ASSERT_TRUE(measured): measured != 0
//

//
// ASSERT_FALSE()
//
// @seen: measured value
//
// ASSERT_FALSE(measured): measured == 0
//

//
// ASSERT_STREQ()
//
// @expected: expected value
// @seen: measured value
//
// ASSERT_STREQ(expected, measured): !strcmp(expected, measured)
//

//
// ASSERT_STRNE()
//
// @expected: expected value
// @seen: measured value
//
// ASSERT_STRNE(expected, measured): strcmp(expected, measured)
//

//
// EXPECT_EQ()
//
// @expected: expected value
// @seen: measured value
//
// EXPECT_EQ(expected, measured): expected == measured
//

//
// EXPECT_NE()
//
// @expected: expected value
// @seen: measured value
//
// EXPECT_NE(expected, measured): expected != measured
//

//
// EXPECT_LT()
//
// @expected: expected value
// @seen: measured value
//
// EXPECT_LT(expected, measured): expected < measured
//

//
// EXPECT_LE()
//
// @expected: expected value
// @seen: measured value
//
// EXPECT_LE(expected, measured): expected <= measured
//

//
// EXPECT_GT()
//
// @expected: expected value
// @seen: measured value
//
// EXPECT_GT(expected, measured): expected > measured
//

//
// EXPECT_GE()
//
// @expected: expected value
// @seen: measured value
//
// EXPECT_GE(expected, measured): expected >= measured
//

//
// EXPECT_NULL()
//
// @seen: measured value
//
// EXPECT_NULL(measured): NULL == measured
//

//
// EXPECT_TRUE()
//
// @seen: measured value
//
// EXPECT_TRUE(measured): 0 != measured
//

//
// EXPECT_FALSE()
//
// @seen: measured value
//
// EXPECT_FALSE(measured): 0 == measured
//

//
// EXPECT_STREQ()
//
// @expected: expected value
// @seen: measured value
//
// EXPECT_STREQ(expected, measured): !strcmp(expected, measured)
//

//
// EXPECT_STRNE()
//
// @expected: expected value
// @seen: measured value
//
// EXPECT_STRNE(expected, measured): strcmp(expected, measured)
//

// Support an optional handler after and ASSERT_* or EXPECT_*.  The approach is
// not thread-safe, but it should be fine in most sane test scenarios.
//
// Using __bail(), which optionally abort()s, is the easiest way to early
// return while still providing an optional block to the API consumer.
//

// Avoid multiple evaluation of the cases */ \
// Report with actual signedness to avoid weird output. */ \
// Ensure the optional handler is triggered */ \

// List helpers

// Circular linked list where only prev is circular. */ \
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __test_results {
    pub /: *mut *mut char reason[1024]; / Reason for test result,
}

// Contains all the information about a fixture.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __fixture_metadata {
    pub name: *const c_char,
    pub tests: *mut __test_metadata,
    pub variant: *mut __fixture_variant_metadata,
    pub next: *mut *mut __fixture_metadata prev,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __test_xfail {
    pub fixture: *mut __fixture_metadata,
    pub variant: *mut __fixture_variant_metadata,
    pub test: *mut __test_metadata,
    pub next: *mut *mut __test_xfail prev,,
}

//
// XFAIL_ADD() - mark variant + test case combination as expected to fail
// @fixture_name: name of the fixture
// @variant_name: name of the variant
// @test_name: name of the test case
//
// Mark a combination of variant + test case for a given fixture as expected
// to fail. Tests marked this way will report XPASS / XFAIL return codes,
// instead of PASS / FAIL,and use respective counters.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __fixture_variant_metadata {
    pub name: *const c_char,
    pub data: *const c_void,
    pub xfails: *mut __test_xfail,
    pub next: *mut *mut __fixture_variant_metadata prev,,
}

// Contains all the information for test execution and status checking.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __test_metadata {
    pub name: *const c_char,
    pub ): *mut __fixture_variant_metadata,
    pub /: *mut *mut pid_t pid; / pid of test when being run,
    pub fixture: *mut __fixture_metadata,
    pub variant): *const *const void self, void,
    pub termsig: c_int,
    pub exit_code: c_int,
    pub /: *mut *mut int trigger; / extra handler after the evaluation,
    pub /: *mut *mut int timeout; / seconds to wait for test timeout,
    pub /: *mut *mut bool aborted; / stopped test due to failed ASSERT,
    pub /: *mut *mut *mut bool no_teardown; / fixture needs teardown,
    pub self: *mut c_void,
    pub variant: *const c_void,
    pub results: *mut __test_results,
    pub next: *mut *mut __test_metadata prev,,
}

//
// Since constructors are called in reverse order, reverse the test
// list so tests are run in source declaration order.
// https://gcc.gnu.org/onlinedocs/gccint/Initialization.html
// However, it seems not all toolchains do this correctly, so use
// __constructor_order_foward to detect which direction is called first
// and adjust list building logic to get things running in the right
// direction.
//
// if this is ASSERT, return immediately.
// otherwise, end the for loop and continue.
//
// Sets status so that WIFEXITED(status) returns true and
// WEXITSTATUS(status) returns KSFT_FAIL.  This safe default value
// should never be evaluated because of the waitpid(2) check and
// timeout handling.
//
// signal process group
// Success
// Failure
extern "C" {
    pub fn islower(_arg: opt) -> return;
}
extern "C" {
    pub fn islower(_arg: opt) -> return;
}
extern "C" {
    pub fn islower(_arg: opt) -> return;
}
//
// If there are no positive tests then we assume user just wants
// exclusions and everything else is a pass.
//
// reset test struct
// Make sure output buffers are flushed before fork
// Reset state inherited from the harness
// Check if we're expecting this test to fail
// unreachable

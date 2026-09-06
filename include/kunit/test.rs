//! Automatically rewritten from C Header to Rust Module
//! Source: include/kunit/test.h
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
// Base unit test (KUnit) API.
//
// Copyright (C) 2019, Google LLC.
// Author: Brendan Higgins <brendanhiggins@google.com>
//

// Static key: true if any KUnit tests are currently running
// Maximum size of parameter description string.
pub const KUNIT_PARAM_DESC_SIZE: c_int = 128;
// Maximum size of a status comment.
pub const KUNIT_STATUS_COMMENT_SIZE: c_int = 256;
//
// TAP specifies subtest stream indentation of 4 spaces, 8 spaces for a
// sub-subtest.  See the "Subtests" section in
// https://node-tap.org/tap-protocol
//
pub const KUNIT_INDENT_LEN: c_int = 4;

//
// enum kunit_status - Type of result for a test or test suite
// @KUNIT_SUCCESS: Denotes the test suite has not failed nor been skipped
// @KUNIT_FAILURE: Denotes the test has failed.
// @KUNIT_SKIPPED: Denotes the test has been skipped.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kunit_status {
    KUNIT_SUCCESS,
    KUNIT_FAILURE,
    KUNIT_SKIPPED,
}

// Attribute struct/enum definitions
//
// Speed Attribute is stored as an enum and separated into categories of
// speed: very_slow, slow, and normal. These speeds are relative to
// other KUnit tests.
//
// Note: unset speed attribute acts as default of KUNIT_SPEED_NORMAL.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kunit_speed {
    KUNIT_SPEED_UNSET,
    KUNIT_SPEED_VERY_SLOW,
    KUNIT_SPEED_SLOW,
    KUNIT_SPEED_NORMAL,
    KUNIT_SPEED_MAX = KUNIT_SPEED_NORMAL,
}

// Holds attributes for each test case and suite
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_attributes {
    pub speed: kunit_speed,
}

//
// struct kunit_case - represents an individual test case.
//
// @run_case: the function representing the actual test case.
// @name:     the name of the test case.
// @generate_params: the generator function for parameterized tests.
// @attr:     the attributes associated with the test
// @param_init: The init function to run before a parameterized test.
// @param_exit: The exit function to run after a parameterized test.
//
// A test case is a function with the signature,
// ``void (*)(struct kunit *)``
// that makes expectations and assertions (see KUNIT_EXPECT_TRUE() and
// KUNIT_ASSERT_TRUE()) about code under test. Each test case is associated
// with a &struct kunit_suite and will be run after the suite's init
// function and followed by the suite's exit function.
//
// A test case should be static and should only be created with the
// KUNIT_CASE() macro; additionally, every array of test cases should be
// terminated with an empty test case.
//
// Example:
//
// .. code-block:: c
//
// void add_test_basic(struct kunit *test)
// {
// KUNIT_EXPECT_EQ(test, 1, add(1, 0));
// KUNIT_EXPECT_EQ(test, 2, add(1, 1));
// KUNIT_EXPECT_EQ(test, 0, add(-1, 1));
// KUNIT_EXPECT_EQ(test, INT_MAX, add(0, INT_MAX));
// KUNIT_EXPECT_EQ(test, -1, add(INT_MAX, INT_MIN));
// }
//
// static struct kunit_case example_test_cases[] = {
// KUNIT_CASE(add_test_basic),
// {}
// };
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_case {
    pub test): *mut *mut void (run_case)(struct kunit,
    pub name: *const c_char,
    pub desc): *const *const void prev, char,
    pub attr: kunit_attributes,
    pub test): *mut *mut int (param_init)(struct kunit,
    pub test): *mut *mut void (param_exit)(struct kunit,
// private: internal use only.
    pub status: kunit_status,
    pub module_name: *mut c_char,
    pub log: *mut string_stream,
}

//
// KUNIT_CASE - A helper for creating a &struct kunit_case
//
// @test_name: a reference to a test case function.
//
// Takes a symbol for a function representing a test case and creates a
// &struct kunit_case object from it. See the documentation for
// &struct kunit_case for an example on how to use it.
//

//
// KUNIT_CASE_ATTR - A helper for creating a &struct kunit_case
// with attributes
//
// @test_name: a reference to a test case function.
// @attributes: a reference to a struct kunit_attributes object containing
// test attributes
//

//
// KUNIT_CASE_SLOW - A helper for creating a &struct kunit_case
// with the slow attribute
//
// @test_name: a reference to a test case function.
//

//
// KUNIT_CASE_PARAM - A helper for creation a parameterized &struct kunit_case
//
// @test_name: a reference to a test case function.
// @gen_params: a reference to a parameter generator function.
//
// The generator function::
//
// const void* gen_params(const void *prev, char *desc)
//
// is used to lazily generate a series of arbitrarily typed values that fit into
// a void*. The argument @prev is the previously returned value, which should be
// used to derive the next value; @prev is set to NULL on the initial generator
// call. When no more values are available, the generator must return NULL.
// Optionally write a string into @desc (size of KUNIT_PARAM_DESC_SIZE)
// describing the parameter.
//

//
// KUNIT_CASE_PARAM_ATTR - A helper for creating a parameterized &struct
// kunit_case with attributes
//
// @test_name: a reference to a test case function.
// @gen_params: a reference to a parameter generator function.
// @attributes: a reference to a struct kunit_attributes object containing
// test attributes
//

//
// KUNIT_CASE_PARAM_WITH_INIT - Define a parameterized KUnit test case with custom
// param_init() and param_exit() functions.
// @test_name: The function implementing the test case.
// @gen_params: The function to generate parameters for the test case.
// @init: A reference to the param_init() function to run before a parameterized test.
// @exit: A reference to the param_exit() function to run after a parameterized test.
//
// Provides the option to register param_init() and param_exit() functions.
// param_init/exit will be passed the parameterized test context and run once
// before and once after the parameterized test. The init function can be used
// to add resources to share between parameter runs, pass parameter arrays,
// and any other setup logic. The exit function can be used to clean up resources
// that were not managed by the parameterized test, and any other teardown logic.
//
// Note: If you are registering a parameter array in param_init() with
// kunit_register_param_array() then you need to pass kunit_array_gen_params()
// to this as the generator function.
//

//
// struct kunit_suite - describes a related collection of &struct kunit_case
//
// @name:	the name of the test. Purely informational.
// @suite_init:	called once per test suite before the test cases.
// @suite_exit:	called once per test suite after all test cases.
// @init:	called before every test case.
// @exit:	called after every test case.
// @test_cases:	a null terminated array of test cases.
// @attr:	the attributes associated with the test suite
//
// A kunit_suite is a collection of related &struct kunit_case s, such that
// @init is called before every test case and @exit is called after every
// test case, similar to the notion of a *test fixture* or a *test class
// in other unit testing frameworks like JUnit or Googletest.
//
// Note that @exit and @suite_exit will run even if @init or @suite_init
// fail: make sure they can handle any inconsistent state which may result.
//
// Every &struct kunit_case must be associated with a kunit_suite for KUnit
// to run it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_suite {
    pub name: [c_char; 256],
    pub suite): *mut *mut int (suite_init)(struct kunit_suite,
    pub suite): *mut *mut void (suite_exit)(struct kunit_suite,
    pub test): *mut *mut int (init)(struct kunit,
    pub test): *mut *mut void (exit)(struct kunit,
    pub test_cases: *mut kunit_case,
    pub attr: kunit_attributes,
// private: internal use only
    pub status_comment: [c_char; KUNIT_STATUS_COMMENT_SIZE],
    pub debugfs: *mut dentry,
    pub log: *mut string_stream,
    pub suite_init_err: c_int,
    pub is_init: bool,
    pub status: kunit_status,
}

// Stores an array of suites, end points one past the end
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_suite_set {
    pub start: *const *const kunit_suite,
    pub end: *const *const kunit_suite,
}

// Stores the pointer to the parameter array and its metadata.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_params {
//
// Reference to the parameter array for a parameterized test. This
// is NULL if a parameter array wasn't directly passed to the
// parameterized test context struct kunit via kunit_register_params_array().
//
    pub params: *const c_void,
// Reference to a function that gets the description of a parameter.
    pub desc): *const *const *const *const void (get_description)(struct kunit test, void param, char,
    pub num_params: usize,
    pub elem_size: usize,
}

//
// struct kunit - represents a running instance of a test.
//
// @priv: for user to store arbitrary data. Commonly used to pass data
// created in the init function (see &struct kunit_suite).
// @parent: reference to the parent context of type struct kunit that can
// be used for storing shared resources.
// @params_array: for storing the parameter array.
//
// Used to store information about the current context under which the test
// is running. Most of this data is private and should only be accessed
// indirectly via public functions; the exceptions are @priv, @parent and
// @params_array which can be used by the test writer to store arbitrary data,
// access the parent context, and to store the parameter array, respectively.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit {
    pub priv: *mut c_void,
    pub parent: *mut kunit,
    pub params_array: kunit_params,
// private: internal use only.
    pub /: *const *const *const char name; / Read only after initialization!,
    pub /: *mut *mut *mut string_stream log; / Points at case log after initialization,
    pub try_catch: kunit_try_catch,
// param_value is the current parameter value for a test case.
    pub param_value: *const c_void,
// param_index stores the index of the parameter in parameterized tests.
    pub param_index: c_int,
//
// success starts as true, and may only be set to false during a
// test case; thus, it is safe to update this across multiple
// threads using WRITE_ONCE; however, as a consequence, it may only
// be read after the test case finishes once all threads associated
// with the test case have terminated.
//
    pub /: *mut *mut spinlock_t lock; / Guards all mutable test state.,
    pub /: *mut *mut kunit_status status; / Read only after test_case finishes!,
//
// Because resources is a list that may be updated multiple times (with
// new resources) from any thread associated with a test case, we must
// protect it with some type of lock.
//
    pub /: *mut *mut list_head resources; / Protected by lock.,
    pub status_comment: [c_char; KUNIT_STATUS_COMMENT_SIZE],
// Saves the last seen test. Useful to help with faults.
    pub last_seen: kunit_loc,
}

extern "C" {
    pub fn kunit_enabled() -> bool;
}
extern "C" {
    pub fn kunit_autorun() -> bool;
}
extern "C" {
    pub fn kunit_init_test(test: *mut kunit, name: *const c_char, log: *mut string_stream);
}
extern "C" {
    pub fn kunit_run_tests(suite: *mut kunit_suite) -> c_int;
}
extern "C" {
    pub fn kunit_suite_num_test_cases(suite: *mut kunit_suite) -> usize;
}
extern "C" {
    pub fn kunit_free_suite_set(suite_set: kunit_suite_set);
}
extern "C" {
    pub fn __kunit_test_suites_exit(suites: *mut kunit_suite, num_suites: c_int);
}
extern "C" {
    pub fn kunit_exec_run_tests(suite_set: *mut kunit_suite_set, builtin: bool);
}
extern "C" {
    pub fn kunit_exec_list_tests(suite_set: *mut kunit_suite_set, include_attr: bool);
}

extern "C" {
    pub fn kunit_run_all_tests() -> c_int;
}

//
// kunit_test_suites() - used to register one or more &struct kunit_suite
// with KUnit.
//
// @__suites: a statically allocated list of &struct kunit_suite.
//
// Registers @suites with the test framework.
// This is done by placing the array of struct kunit_suite * in the
// .kunit_test_suites ELF section.
//
// When builtin, KUnit tests are all run via the executor at boot, and when
// built as a module, they run on module load.
//

//
// kunit_test_init_section_suites() - used to register one or more &struct
// kunit_suite containing init functions or
// init data.
//
// @__suites: a statically allocated list of &struct kunit_suite.
//
// This functions similar to kunit_test_suites() except that it compiles the
// list of suites during init phase.
//
// This macro also suffixes the array and suite declarations it makes with
// _probe; so that modpost suppresses warnings about referencing init data
// for symbols named in this manner.
//
// Note: these init tests are not able to be run after boot so there is no
// "run" debugfs file generated for these tests.
//
// Also, do not mark the suite or test case structs with __initdata because
// they will be used after the init phase with debugfs.
//

extern "C" {
    pub fn kunit_suite_has_succeeded(suite: *mut kunit_suite) -> kunit_status;
}
//
// kunit_kmalloc_array() - Like kmalloc_array() except the allocation is *test managed*.
// @test: The test context object.
// @n: number of elements.
// @size: The size in bytes of the desired memory.
// @gfp: flags passed to underlying kmalloc().
//
// Just like `kmalloc_array(...)`, except the allocation is managed by the test case
// and is automatically cleaned up after the test case concludes. See kunit_add_action()
// for more information.
//
// Note that some internal context data is also allocated with GFP_KERNEL,
// regardless of the gfp passed in.
//
// kunit_kmalloc() - Like kmalloc() except the allocation is *test managed*.
// @test: The test context object.
// @size: The size in bytes of the desired memory.
// @gfp: flags passed to underlying kmalloc().
//
// See kmalloc() and kunit_kmalloc_array() for more information.
//
// Note that some internal context data is also allocated with GFP_KERNEL,
// regardless of the gfp passed in.
//
extern "C" {
    pub fn kunit_kmalloc_array(_arg: test, _arg: 1, _arg: size, _arg: gfp) -> return;
}
//
// kunit_kfree() - Like kfree except for allocations managed by KUnit.
// @test: The test case to which the resource belongs.
// @ptr: The memory allocation to free.
//
extern "C" {
    pub fn kunit_kfree(test: *mut kunit, ptr: *const c_void);
}
//
// kunit_kzalloc() - Just like kunit_kmalloc(), but zeroes the allocation.
// @test: The test context object.
// @size: The size in bytes of the desired memory.
// @gfp: flags passed to underlying kmalloc().
//
// See kzalloc() and kunit_kmalloc_array() for more information.
//
extern "C" {
    pub fn kunit_kmalloc(_arg: test, _arg: size, __GFP_ZERO: gfp |) -> return;
}
//
// kunit_kcalloc() - Just like kunit_kmalloc_array(), but zeroes the allocation.
// @test: The test context object.
// @n: number of elements.
// @size: The size in bytes of the desired memory.
// @gfp: flags passed to underlying kmalloc().
//
// See kcalloc() and kunit_kmalloc_array() for more information.
//
extern "C" {
    pub fn kunit_kmalloc_array(_arg: test, _arg: n, _arg: size, __GFP_ZERO: gfp |) -> return;
}
//
// kunit_kfree_const() - conditionally free test managed memory
// @test: The test context object.
// @x: pointer to the memory
//
// Calls kunit_kfree() only if @x is not in .rodata section.
// See kunit_kstrdup_const() for more information.
//
extern "C" {
    pub fn kunit_kfree_const(test: *mut kunit, x: *const c_void);
}
//
// kunit_kstrdup() - Duplicates a string into a test managed allocation.
//
// @test: The test context object.
// @str: The NULL-terminated string to duplicate.
// @gfp: flags passed to underlying kmalloc().
//
// See kstrdup() and kunit_kmalloc_array() for more information.
//
// kunit_kstrdup_const() - Conditionally duplicates a string into a test managed allocation.
//
// @test: The test context object.
// @str: The NULL-terminated string to duplicate.
// @gfp: flags passed to underlying kmalloc().
//
// Calls kunit_kstrdup() only if @str is not in the rodata section. Must be freed with
// kunit_kfree_const() -- not kunit_kfree().
// See kstrdup_const() and kunit_kmalloc_array() for more information.
//
// kunit_attach_mm() - Create and attach a new mm if it doesn't already exist.
//
// Allocates a &struct mm_struct and attaches it to @current. In most cases, call
// kunit_vm_mmap() without calling kunit_attach_mm() directly. Only necessary when
// code under test accesses the mm before executing the mmap (e.g., to perform
// additional initialization beforehand).
//
// Return: 0 on success, -errno on failure.
//
extern "C" {
    pub fn kunit_attach_mm() -> c_int;
}
//
// kunit_vm_mmap() - Allocate KUnit-tracked vm_mmap() area
// @test: The test context object.
// @file: struct file pointer to map from, if any
// @addr: desired address, if any
// @len: how many bytes to allocate
// @prot: mmap PROT_* bits
// @flag: mmap flags
// @offset: offset into @file to start mapping from.
//
// See vm_mmap() for more information.
//
extern "C" {
    pub fn kunit_cleanup(test: *mut kunit);
}
extern "C" {
    pub fn kunit_free_boot_suites();
}
extern "C" {
    pub fn __printf(_arg: 2, log: *mut 3) kunit_log_append(struct string_stream, fmt: *const c_char, ...);
}
//
// kunit_mark_skipped() - Marks @test as skipped
//
// @test: The test context object.
// @fmt:  A printk() style format string.
//
// Marks the test as skipped. @fmt is given output as the test status
// comment, typically the reason the test was skipped.
//
// Test execution continues after kunit_mark_skipped() is called.
//

//
// kunit_skip() - Marks @test as skipped
//
// @test: The test context object.
// @fmt:  A printk() style format string.
//
// Skips the test. @fmt is given output as the test status
// comment, typically the reason the test was skipped.
//
// Test execution is halted after kunit_skip() is called.
//

//
// printk and log to per-test or per-suite log buffer.  Logging only done
// if CONFIG_KUNIT_DEBUGFS is 'y'; if it is 'n', no log is allocated/used.
//

//
// kunit_info() - Prints an INFO level message associated with @test.
//
// @test: The test context object.
// @fmt:  A printk() style format string.
//
// Prints an info level message associated with the test suite being run.
// Takes a variable number of format parameters just like printk().
//

//
// kunit_warn() - Prints a WARN level message associated with @test.
//
// @test: The test context object.
// @fmt:  A printk() style format string.
//
// Prints a warning level message.
//

//
// kunit_err() - Prints an ERROR level message associated with @test.
//
// @test: The test context object.
// @fmt:  A printk() style format string.
//
// Prints an error level message.
//

//
// Must be called at the beginning of each KUNIT_*_ASSERTION().
// Cf. KUNIT_CURRENT_LOC.
//

//
// KUNIT_SUCCEED() - A no-op expectation. Only exists for code clarity.
// @test: The test context object.
//
// The opposite of KUNIT_FAIL(), it is an expectation that cannot fail. In other
// words, it does nothing and only exists for code clarity. See
// KUNIT_EXPECT_TRUE() for more information.
//

extern "C" {
    pub fn __kunit_abort(test: *mut kunit) -> void __noreturn;
}

//
// KUNIT_FAIL() - Always causes a test to fail when evaluated.
// @test: The test context object.
// @fmt: an informational message to be printed when the assertion is made.
// @...: string format arguments.
//
// The opposite of KUNIT_SUCCEED(), it is an expectation that always fails. In
// other words, it always results in a failed expectation, and consequently
// always causes the test case to fail when evaluated. See KUNIT_EXPECT_TRUE()
// for more information.
//

// Helper to safely pass around an initializer list to other macros.

//
// A factory macro for defining the assertions and expectations for the basic
// comparisons defined for the built in types.
//
// Unfortunately, there is no common type that all types can be promoted to for
// which all the binary operators behave the same way as for the actual types
// (for example, there is no type that long long and unsigned long long can
// both be cast to where the comparison result is preserved for all values). So
// the best we can do is do the comparison in the original types and then coerce
// everything to long long for printing; this way, the comparison behaves
// correctly and the printed out value usually makes sense without
// interpretation, but can always be interpreted to figure out the actual
// value.
//

//
// KUNIT_EXPECT_TRUE() - Causes a test failure when the expression is not true.
// @test: The test context object.
// @condition: an arbitrary boolean expression. The test fails when this does
// not evaluate to true.
//
// This and expectations of the form `KUNIT_EXPECT_*` will cause the test case
// to fail when the specified condition is not met; however, it will not prevent
// the test case from continuing to run; this is otherwise known as an
// *expectation failure*.
//

//
// KUNIT_EXPECT_FALSE() - Makes a test failure when the expression is not false.
// @test: The test context object.
// @condition: an arbitrary boolean expression. The test fails when this does
// not evaluate to false.
//
// Sets an expectation that @condition evaluates to false. See
// KUNIT_EXPECT_TRUE() for more information.
//

//
// KUNIT_EXPECT_EQ() - Sets an expectation that @left and @right are equal.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a primitive C type.
// @right: an arbitrary expression that evaluates to a primitive C type.
//
// Sets an expectation that the values that @left and @right evaluate to are
// equal. This is semantically equivalent to
// KUNIT_EXPECT_TRUE(@test, (@left) == (@right)). See KUNIT_EXPECT_TRUE() for
// more information.
//

//
// KUNIT_EXPECT_PTR_EQ() - Expects that pointers @left and @right are equal.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a pointer.
// @right: an arbitrary expression that evaluates to a pointer.
//
// Sets an expectation that the values that @left and @right evaluate to are
// equal. This is semantically equivalent to
// KUNIT_EXPECT_TRUE(@test, (@left) == (@right)). See KUNIT_EXPECT_TRUE() for
// more information.
//

//
// KUNIT_EXPECT_NE() - An expectation that @left and @right are not equal.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a primitive C type.
// @right: an arbitrary expression that evaluates to a primitive C type.
//
// Sets an expectation that the values that @left and @right evaluate to are not
// equal. This is semantically equivalent to
// KUNIT_EXPECT_TRUE(@test, (@left) != (@right)). See KUNIT_EXPECT_TRUE() for
// more information.
//

//
// KUNIT_EXPECT_PTR_NE() - Expects that pointers @left and @right are not equal.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a pointer.
// @right: an arbitrary expression that evaluates to a pointer.
//
// Sets an expectation that the values that @left and @right evaluate to are not
// equal. This is semantically equivalent to
// KUNIT_EXPECT_TRUE(@test, (@left) != (@right)). See KUNIT_EXPECT_TRUE() for
// more information.
//

//
// KUNIT_EXPECT_LT() - An expectation that @left is less than @right.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a primitive C type.
// @right: an arbitrary expression that evaluates to a primitive C type.
//
// Sets an expectation that the value that @left evaluates to is less than the
// value that @right evaluates to. This is semantically equivalent to
// KUNIT_EXPECT_TRUE(@test, (@left) < (@right)). See KUNIT_EXPECT_TRUE() for
// more information.
//

//
// KUNIT_EXPECT_LE() - Expects that @left is less than or equal to @right.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a primitive C type.
// @right: an arbitrary expression that evaluates to a primitive C type.
//
// Sets an expectation that the value that @left evaluates to is less than or
// equal to the value that @right evaluates to. Semantically this is equivalent
// to KUNIT_EXPECT_TRUE(@test, (@left) <= (@right)). See KUNIT_EXPECT_TRUE() for
// more information.
//

//
// KUNIT_EXPECT_GT() - An expectation that @left is greater than @right.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a primitive C type.
// @right: an arbitrary expression that evaluates to a primitive C type.
//
// Sets an expectation that the value that @left evaluates to is greater than
// the value that @right evaluates to. This is semantically equivalent to
// KUNIT_EXPECT_TRUE(@test, (@left) > (@right)). See KUNIT_EXPECT_TRUE() for
// more information.
//

//
// KUNIT_EXPECT_GE() - Expects that @left is greater than or equal to @right.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a primitive C type.
// @right: an arbitrary expression that evaluates to a primitive C type.
//
// Sets an expectation that the value that @left evaluates to is greater than
// the value that @right evaluates to. This is semantically equivalent to
// KUNIT_EXPECT_TRUE(@test, (@left) >= (@right)). See KUNIT_EXPECT_TRUE() for
// more information.
//

//
// KUNIT_EXPECT_STREQ() - Expects that strings @left and @right are equal.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a null terminated string.
// @right: an arbitrary expression that evaluates to a null terminated string.
//
// Sets an expectation that the values that @left and @right evaluate to are
// equal. This is semantically equivalent to
// KUNIT_EXPECT_TRUE(@test, !strcmp((@left), (@right))). See KUNIT_EXPECT_TRUE()
// for more information.
//

//
// KUNIT_EXPECT_STRNEQ() - Expects that strings @left and @right are not equal.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a null terminated string.
// @right: an arbitrary expression that evaluates to a null terminated string.
//
// Sets an expectation that the values that @left and @right evaluate to are
// not equal. This is semantically equivalent to
// KUNIT_EXPECT_TRUE(@test, strcmp((@left), (@right))). See KUNIT_EXPECT_TRUE()
// for more information.
//

//
// KUNIT_EXPECT_MEMEQ() - Expects that the first @size bytes of @left and @right are equal.
// @test: The test context object.
// @left: An arbitrary expression that evaluates to the specified size.
// @right: An arbitrary expression that evaluates to the specified size.
// @size: Number of bytes compared.
//
// Sets an expectation that the values that @left and @right evaluate to are
// equal. This is semantically equivalent to
// KUNIT_EXPECT_TRUE(@test, !memcmp((@left), (@right), (@size))). See
// KUNIT_EXPECT_TRUE() for more information.
//
// Although this expectation works for any memory block, it is not recommended
// for comparing more structured data, such as structs. This expectation is
// recommended for comparing, for example, data arrays.
//

//
// KUNIT_EXPECT_MEMNEQ() - Expects that the first @size bytes of @left and @right are not equal.
// @test: The test context object.
// @left: An arbitrary expression that evaluates to the specified size.
// @right: An arbitrary expression that evaluates to the specified size.
// @size: Number of bytes compared.
//
// Sets an expectation that the values that @left and @right evaluate to are
// not equal. This is semantically equivalent to
// KUNIT_EXPECT_TRUE(@test, memcmp((@left), (@right), (@size))). See
// KUNIT_EXPECT_TRUE() for more information.
//
// Although this expectation works for any memory block, it is not recommended
// for comparing more structured data, such as structs. This expectation is
// recommended for comparing, for example, data arrays.
//

//
// KUNIT_EXPECT_NULL() - Expects that @ptr is null.
// @test: The test context object.
// @ptr: an arbitrary pointer.
//
// Sets an expectation that the value that @ptr evaluates to is null. This is
// semantically equivalent to KUNIT_EXPECT_PTR_EQ(@test, ptr, NULL).
// See KUNIT_EXPECT_TRUE() for more information.
//

//
// KUNIT_EXPECT_NOT_NULL() - Expects that @ptr is not null.
// @test: The test context object.
// @ptr: an arbitrary pointer.
//
// Sets an expectation that the value that @ptr evaluates to is not null. This
// is semantically equivalent to KUNIT_EXPECT_PTR_NE(@test, ptr, NULL).
// See KUNIT_EXPECT_TRUE() for more information.
//

//
// KUNIT_EXPECT_NOT_ERR_OR_NULL() - Expects that @ptr is not null and not err.
// @test: The test context object.
// @ptr: an arbitrary pointer.
//
// Sets an expectation that the value that @ptr evaluates to is not null and not
// an errno stored in a pointer. This is semantically equivalent to
// KUNIT_EXPECT_TRUE(@test, !IS_ERR_OR_NULL(@ptr)). See KUNIT_EXPECT_TRUE() for
// more information.
//

//
// KUNIT_FAIL_AND_ABORT() - Always causes a test to fail and abort when evaluated.
// @test: The test context object.
// @fmt: an informational message to be printed when the assertion is made.
// @...: string format arguments.
//
// The opposite of KUNIT_SUCCEED(), it is an assertion that always fails. In
// other words, it always results in a failed assertion, and consequently
// always causes the test case to fail and abort when evaluated.
// See KUNIT_ASSERT_TRUE() for more information.
//

//
// KUNIT_ASSERT_TRUE() - Sets an assertion that @condition is true.
// @test: The test context object.
// @condition: an arbitrary boolean expression. The test fails and aborts when
// this does not evaluate to true.
//
// This and assertions of the form `KUNIT_ASSERT_*` will cause the test case to
// fail *and immediately abort* when the specified condition is not met. Unlike
// an expectation failure, it will prevent the test case from continuing to run;
// this is otherwise known as an *assertion failure*.
//

//
// KUNIT_ASSERT_FALSE() - Sets an assertion that @condition is false.
// @test: The test context object.
// @condition: an arbitrary boolean expression.
//
// Sets an assertion that the value that @condition evaluates to is false. This
// is the same as KUNIT_EXPECT_FALSE(), except it causes an assertion failure
// (see KUNIT_ASSERT_TRUE()) when the assertion is not met.
//

//
// KUNIT_ASSERT_EQ() - Sets an assertion that @left and @right are equal.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a primitive C type.
// @right: an arbitrary expression that evaluates to a primitive C type.
//
// Sets an assertion that the values that @left and @right evaluate to are
// equal. This is the same as KUNIT_EXPECT_EQ(), except it causes an assertion
// failure (see KUNIT_ASSERT_TRUE()) when the assertion is not met.
//

//
// KUNIT_ASSERT_PTR_EQ() - Asserts that pointers @left and @right are equal.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a pointer.
// @right: an arbitrary expression that evaluates to a pointer.
//
// Sets an assertion that the values that @left and @right evaluate to are
// equal. This is the same as KUNIT_EXPECT_EQ(), except it causes an assertion
// failure (see KUNIT_ASSERT_TRUE()) when the assertion is not met.
//

//
// KUNIT_ASSERT_NE() - An assertion that @left and @right are not equal.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a primitive C type.
// @right: an arbitrary expression that evaluates to a primitive C type.
//
// Sets an assertion that the values that @left and @right evaluate to are not
// equal. This is the same as KUNIT_EXPECT_NE(), except it causes an assertion
// failure (see KUNIT_ASSERT_TRUE()) when the assertion is not met.
//

//
// KUNIT_ASSERT_PTR_NE() - Asserts that pointers @left and @right are not equal.
// KUNIT_ASSERT_PTR_EQ() - Asserts that pointers @left and @right are equal.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a pointer.
// @right: an arbitrary expression that evaluates to a pointer.
//
// Sets an assertion that the values that @left and @right evaluate to are not
// equal. This is the same as KUNIT_EXPECT_NE(), except it causes an assertion
// failure (see KUNIT_ASSERT_TRUE()) when the assertion is not met.
//

//
// KUNIT_ASSERT_LT() - An assertion that @left is less than @right.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a primitive C type.
// @right: an arbitrary expression that evaluates to a primitive C type.
//
// Sets an assertion that the value that @left evaluates to is less than the
// value that @right evaluates to. This is the same as KUNIT_EXPECT_LT(), except
// it causes an assertion failure (see KUNIT_ASSERT_TRUE()) when the assertion
// is not met.
//

//
// KUNIT_ASSERT_LE() - An assertion that @left is less than or equal to @right.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a primitive C type.
// @right: an arbitrary expression that evaluates to a primitive C type.
//
// Sets an assertion that the value that @left evaluates to is less than or
// equal to the value that @right evaluates to. This is the same as
// KUNIT_EXPECT_LE(), except it causes an assertion failure (see
// KUNIT_ASSERT_TRUE()) when the assertion is not met.
//

//
// KUNIT_ASSERT_GT() - An assertion that @left is greater than @right.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a primitive C type.
// @right: an arbitrary expression that evaluates to a primitive C type.
//
// Sets an assertion that the value that @left evaluates to is greater than the
// value that @right evaluates to. This is the same as KUNIT_EXPECT_GT(), except
// it causes an assertion failure (see KUNIT_ASSERT_TRUE()) when the assertion
// is not met.
//

//
// KUNIT_ASSERT_GE() - Assertion that @left is greater than or equal to @right.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a primitive C type.
// @right: an arbitrary expression that evaluates to a primitive C type.
//
// Sets an assertion that the value that @left evaluates to is greater than the
// value that @right evaluates to. This is the same as KUNIT_EXPECT_GE(), except
// it causes an assertion failure (see KUNIT_ASSERT_TRUE()) when the assertion
// is not met.
//

//
// KUNIT_ASSERT_STREQ() - An assertion that strings @left and @right are equal.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a null terminated string.
// @right: an arbitrary expression that evaluates to a null terminated string.
//
// Sets an assertion that the values that @left and @right evaluate to are
// equal. This is the same as KUNIT_EXPECT_STREQ(), except it causes an
// assertion failure (see KUNIT_ASSERT_TRUE()) when the assertion is not met.
//

//
// KUNIT_ASSERT_STRNEQ() - An assertion that strings @left and @right are not equal.
// @test: The test context object.
// @left: an arbitrary expression that evaluates to a null terminated string.
// @right: an arbitrary expression that evaluates to a null terminated string.
//
// Sets an assertion that the values that @left and @right evaluate to are
// not equal. This is semantically equivalent to
// KUNIT_ASSERT_TRUE(@test, strcmp((@left), (@right))). See KUNIT_ASSERT_TRUE()
// for more information.
//

//
// KUNIT_ASSERT_MEMEQ() - Asserts that the first @size bytes of @left and @right are equal.
// @test: The test context object.
// @left: An arbitrary expression that evaluates to the specified size.
// @right: An arbitrary expression that evaluates to the specified size.
// @size: Number of bytes compared.
//
// Sets an assertion that the values that @left and @right evaluate to are
// equal. This is semantically equivalent to
// KUNIT_ASSERT_TRUE(@test, !memcmp((@left), (@right), (@size))). See
// KUNIT_ASSERT_TRUE() for more information.
//
// Although this assertion works for any memory block, it is not recommended
// for comparing more structured data, such as structs. This assertion is
// recommended for comparing, for example, data arrays.
//

//
// KUNIT_ASSERT_MEMNEQ() - Asserts that the first @size bytes of @left and @right are not equal.
// @test: The test context object.
// @left: An arbitrary expression that evaluates to the specified size.
// @right: An arbitrary expression that evaluates to the specified size.
// @size: Number of bytes compared.
//
// Sets an assertion that the values that @left and @right evaluate to are
// not equal. This is semantically equivalent to
// KUNIT_ASSERT_TRUE(@test, memcmp((@left), (@right), (@size))). See
// KUNIT_ASSERT_TRUE() for more information.
//
// Although this assertion works for any memory block, it is not recommended
// for comparing more structured data, such as structs. This assertion is
// recommended for comparing, for example, data arrays.
//

//
// KUNIT_ASSERT_NULL() - Asserts that pointers @ptr is null.
// @test: The test context object.
// @ptr: an arbitrary pointer.
//
// Sets an assertion that the values that @ptr evaluates to is null. This is
// the same as KUNIT_EXPECT_NULL(), except it causes an assertion
// failure (see KUNIT_ASSERT_TRUE()) when the assertion is not met.
//

//
// KUNIT_ASSERT_NOT_NULL() - Asserts that pointers @ptr is not null.
// @test: The test context object.
// @ptr: an arbitrary pointer.
//
// Sets an assertion that the values that @ptr evaluates to is not null. This
// is the same as KUNIT_EXPECT_NOT_NULL(), except it causes an assertion
// failure (see KUNIT_ASSERT_TRUE()) when the assertion is not met.
//

//
// KUNIT_ASSERT_NOT_ERR_OR_NULL() - Assertion that @ptr is not null and not err.
// @test: The test context object.
// @ptr: an arbitrary pointer.
//
// Sets an assertion that the value that @ptr evaluates to is not null and not
// an errno stored in a pointer. This is the same as
// KUNIT_EXPECT_NOT_ERR_OR_NULL(), except it causes an assertion failure (see
// KUNIT_ASSERT_TRUE()) when the assertion is not met.
//

//
// KUNIT_ARRAY_PARAM() - Define test parameter generator from an array.
// @name:  prefix for the test parameter generator function.
// @array: array of test parameters.
// @get_desc: function to convert param to description; NULL to use default
//
// Define function @name_gen_params which uses @array to generate parameters.
//

//
// KUNIT_ARRAY_PARAM_DESC() - Define test parameter generator from an array.
// @name:  prefix for the test parameter generator function.
// @array: array of test parameters.
// @desc_member: structure member from array element to use as description
//
// Define function @name_gen_params which uses @array to generate parameters.
//

//
// kunit_register_params_array() - Register parameter array for a KUnit test.
// @test: The KUnit test structure to which parameters will be added.
// @array: An array of test parameters.
// @param_count: Number of parameters.
// @get_desc: Function that generates a string description for a given parameter
// element.
//
// This macro initializes the @test's parameter array data, storing information
// including the parameter array, its count, the element size, and the parameter
// description function within `test->params_array`.
//
// Note: If using this macro in param_init(), kunit_array_gen_params()
// will then need to be manually provided as the parameter generator function to
// KUNIT_CASE_PARAM_WITH_INIT(). kunit_array_gen_params() is a KUnit
// function that uses the registered array to generate parameters
//

// TODO(dlatypov@google.com): consider eventually migrating users to explicitly
// include resource.h themselves if they need it.

//
// Warning backtrace suppression API.
//
// Suppresses WARN*() backtraces on the current task while active. Two forms
// are provided:
//
// - Scoped: kunit_warning_suppress(test) { ... }
// Suppression is active for the duration of the block. On normal exit,
// the for-loop increment deactivates suppression. On early exit (break,
// return, goto), the __cleanup attribute fires. On kthread_exit() (e.g.,
// a failed KUnit assertion), kunit_add_action() cleans up at test
// teardown. The suppression handle is only accessible inside the block,
// so warning counts must be checked before the block exits.
//
// - Direct: kunit_start_suppress_warning() / kunit_end_suppress_warning()
// The underlying functions, returning an explicit handle pointer. Use
// when the handle needs to be retained (e.g., for post-suppression
// count checks) or passed across helper functions.
//
extern "C" {
    pub fn kunit_suppressed_warning_count(w: *mut kunit_suppressed_warning) -> c_int;
}
extern "C" {
    pub fn __kunit_suppress_auto_cleanup(wp: *mut kunit_suppressed_warning);
}
extern "C" {
    pub fn kunit_has_active_suppress_warning() -> bool;
}
//
// kunit_warning_suppress() - Suppress WARN*() backtraces for the duration
// of a block.
// @test: The test context object.
//
// Scoped form of the suppression API. Suppression starts when the block is
// entered and ends automatically when the block exits through any path. See
// the section comment above for the cleanup guarantees on each exit path.
// Fails the test if suppression is already active; nesting is not supported.
//
// The warning count can be checked inside the block via
// KUNIT_EXPECT_SUPPRESSED_WARNING_COUNT(). The handle is not accessible
// after the block exits.
//
// Example::
//
// kunit_warning_suppress(test) {
// trigger_warning();
// KUNIT_EXPECT_SUPPRESSED_WARNING_COUNT(test, 1);
// }
//

//
// KUNIT_SUPPRESSED_WARNING_COUNT() - Returns the suppressed warning count.
//
// Returns the number of WARN*() calls suppressed since the current
// suppression block started, or 0 if the handle is NULL. Usable inside a
// kunit_warning_suppress() block.
//

//
// KUNIT_EXPECT_SUPPRESSED_WARNING_COUNT() - Sets an expectation that the
// suppressed warning count equals
// @expected.
// @test: The test context object.
// @expected: an expression that evaluates to the expected warning count.
//
// Sets an expectation that the number of suppressed WARN*() calls equals
// @expected. This is semantically equivalent to
// KUNIT_EXPECT_EQ(@test, KUNIT_SUPPRESSED_WARNING_COUNT(), @expected).
// See KUNIT_EXPECT_EQ() for more information.
//

//
// KUNIT_ASSERT_SUPPRESSED_WARNING_COUNT() - Sets an assertion that the
// suppressed warning count equals
// @expected.
// @test: The test context object.
// @expected: an expression that evaluates to the expected warning count.
//
// Sets an assertion that the number of suppressed WARN*() calls equals
// @expected. This is the same as KUNIT_EXPECT_SUPPRESSED_WARNING_COUNT(),
// except it causes an assertion failure (see KUNIT_ASSERT_TRUE()) when the
// assertion is not met.
//


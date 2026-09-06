//! Automatically rewritten from C to Rust
//! Source: lib/tests/stackinit_kunit.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Test cases for compiler-based stack variable zeroing via
// -ftrivial-auto-var-init={zero,pattern}.
// For example, see:
// "Running tests with kunit_tool" at Documentation/dev-tools/kunit/start.rst
// ./tools/testing/kunit/kunit.py run stackinit [--raw_output] \
// --make_option LLVM=1 \
// --kconfig_add CONFIG_INIT_STACK_ALL_ZERO=y
//

// Exfiltration buffer.
pub const MAX_VAR_SIZE: c_int = 128;
    static u8 check_buf[MAX_VAR_SIZE];
// Character array to trigger stack protector in all functions.
pub const VAR_BUFFER: c_int = 32;
// Volatile mask to convince compiler to copy memory with 0xff.
    let mut forced_mask: static volatile u8 = 0xff;
// Location and size tracking to validate fill and test are colocated.
    static void *fill_start, *target_start;
    static size_t fill_size, target_size;
    static bool stackinit_range_contains(char *haystack_start, size_t haystack_size,
    char *needle_start, size_t needle_size)
    {
    if (needle_start >= haystack_start &&
    needle_start + needle_size <= haystack_start + haystack_size)
    return true;
    return false;
    }
// Whether the test is expected to fail.
pub const WANT_SUCCESS: c_int = 0;
pub const XFAIL: c_int = 1;

    (var) = do_nothing_ ## name(&(var))

    do_nothing_ ## name(var)

    do_nothing_ ## name(&(var))

    do_nothing_ ## name(&(var))

//
// On m68k, if the leaf function test variable is longer than 8 bytes,
// the start of the stack frame moves. 8 is sufficiently large to
// test m68k char arrays, but leave it at 16 for other architectures.
//

pub const FILL_SIZE_STRING: c_int = 8;
pub const FILL_SIZE_ARRAY: c_int = 2;

pub const FILL_SIZE_STRING: c_int = 16;
pub const FILL_SIZE_ARRAY: c_int = 8;

//
// For the struct, intentionally poison padding to see if it gets
// copied out in direct assignments.
//

    do {						\
    memset(&(zero), 0xFF, sizeof(zero));	\
    zero.one = 0;				\
    zero.two = 0;				\
    zero.three = 0;				\
    zero.four = 0;				\
    } while (0)

    .two = 0,			\
    .three = 0,			\
    .four = 0,			\
    }

    .two = arg.two,		\
    .three = arg.three,		\
    .four = arg.four,		\
    }

    var.two = 0;			\
    var.three = 0;			\
    var.four = 0

    = __static_partial

    = __static_all

    = __dynamic_partial

    = __dynamic_all

    ; __runtime_partial

    ; __runtime_all

    ; var = (var_type)__static_partial

    ; var = (var_type)__static_all

    ; var = (var_type)__dynamic_partial

    ; var = (var_type)__dynamic_all

    ; var = *(arg)
// Union initialization is the same as structs.

    INIT_STRUCT_static_partial(var_type)

    INIT_STRUCT_static_all(var_type)

    INIT_STRUCT_dynamic_partial(var_type)

    INIT_STRUCT_dynamic_all(var_type)

    INIT_STRUCT_runtime_partial(var_type)

    INIT_STRUCT_runtime_all(var_type)

    INIT_STRUCT_assigned_static_partial(var_type)

    INIT_STRUCT_assigned_static_all(var_type)

    INIT_STRUCT_assigned_dynamic_partial(var_type)

    INIT_STRUCT_assigned_dynamic_all(var_type)

    INIT_STRUCT_assigned_copy(var_type)
//
// The "did we actually fill the stack?" check value needs
// to be neither 0 nor any of the "pattern" bytes. The
// pattern bytes are compiler, architecture, and type based,
// so we have to pick a value that never appears for those
// combinations. Use 0x99 which is not 0xFF, 0xFE, nor 0xAA.
//
pub const FILL_BYTE: c_uint = 0x99;
//
// @name: unique string name for the test
// @var_type: type to be tested for zeroing initialization
// @which: is this a SCALAR, STRING, or STRUCT type?
// @init_level: what kind of initialization is performed
// @xfail: is this test expected to fail?
//

// Returns 0 on success, 1 on failure. */			\
    static noinline void test_ ## name (struct kunit *test)		\
    {								\
    var_type zero INIT_CLONE_ ## which;			\
    int ignored;						\
    u8 sum = 0, i;						\
    \
// Notice when a new test is larger than expected. */	\
    BUILD_BUG_ON(sizeof(zero) > MAX_VAR_SIZE);		\
    \
// Fill clone type with zero for per-field init. */	\
    ZERO_CLONE_ ## which(zero);				\
// Clear entire check buffer for 0xFF overlap test. */	\
    memset(check_buf, 0x00, sizeof(check_buf));		\
// Fill stack with FILL_BYTE. */			\
    ignored = leaf_ ##name((unsigned long)&ignored, 1,	\
    FETCH_ARG_ ## which(zero));	\
// Verify all bytes overwritten with FILL_BYTE. */	\
    for (sum = 0, i = 0; i < target_size; i++)		\
    sum += (check_buf[i] != FILL_BYTE);		\
// Clear entire check buffer for later bit tests. */	\
    memset(check_buf, 0x00, sizeof(check_buf));		\
// Extract stack-defined variable contents. */		\
    ignored = leaf_ ##name((unsigned long)&ignored, 0,	\
    FETCH_ARG_ ## which(zero));	\
// \
// Delay the sum test to here to do as little as	\
// possible between the two leaf function calls.	\
// \
    KUNIT_ASSERT_EQ_MSG(test, sum, 0,			\
    "leaf fill was not 0x%02X!?\n",	\
    FILL_BYTE);				\
    \
// Validate that compiler lined up fill and target. */	\
    KUNIT_ASSERT_TRUE_MSG(test,				\
    stackinit_range_contains(fill_start, fill_size,	\
    target_start, target_size),		\
    "stackframe was not the same between calls!? "	\
    "(fill %zu wide, target offset by %d)\n",	\
    fill_size,					\
    (int)((ssize_t)(uintptr_t)fill_start -		\
    (ssize_t)(uintptr_t)target_start));	\
    \
// Validate check region has no FILL_BYTE bytes. */	\
    for (sum = 0, i = 0; i < target_size; i++)		\
    sum += (check_buf[i] == FILL_BYTE);		\
    \
    if (sum != 0 && xfail)					\
    kunit_skip(test,				\
    "XFAIL uninit bytes: %d\n",		\
    sum);				\
    KUNIT_ASSERT_EQ_MSG(test, sum, 0,			\
    "uninit bytes: %d\n", sum);			\
    }

// no-op to force compiler into ignoring "uninitialized" vars */\
    static noinline DO_NOTHING_TYPE_ ## which(var_type)		\
    do_nothing_ ## name(var_type *ptr)				\
    {								\
    OPTIMIZER_HIDE_VAR(ptr);				\
// Will always be true, but compiler doesn't know. */	\
    if ((unsigned long)ptr > 0x2)				\
    return DO_NOTHING_RETURN_ ## which(ptr);	\
    else							\
    return DO_NOTHING_RETURN_ ## which(ptr + 1);	\
    }								\
    static noinline int leaf_ ## name(unsigned long sp, bool fill,	\
    var_type *arg)		\
    {								\
    char buf[VAR_BUFFER];					\
    var_type var						\
    INIT_ ## which ## _ ## init_level(var_type);	\
    \
    target_start = &var;					\
    target_size = sizeof(var);				\
// \
// Keep this buffer around to make sure we've got a	\
// stack frame of SOME kind...				\
// \
    memset(buf, (char)(sp & 0xff), sizeof(buf));		\
// Fill variable with FILL_BYTE. */			\
    if (fill) {						\
    fill_start = &var;				\
    fill_size = sizeof(var);			\
    memset(fill_start,				\
    FILL_BYTE & forced_mask,			\
    fill_size);				\
    }							\
    \
// Silence "never initialized" warnings. */		\
    DO_NOTHING_CALL_ ## which(var, name);			\
    \
// Exfiltrate "var". */					\
    memcpy(check_buf, target_start, target_size);		\
    \
    return (int)buf[0] | (int)buf[sizeof(buf) - 1];		\
    }								\
    DEFINE_TEST_DRIVER(name, var_type, which, xfail)
// Structure with no padding.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_packed {
    pub one: c_ulong,
    pub two: c_ulong,
    pub three: c_ulong,
    pub four: c_ulong,
}

// Simple structure with padding likely to be covered by compiler.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_small_hole {
    pub one: usize,
    pub two: c_char,
// 3 byte padding hole here.
    pub three: c_int,
    pub four: c_ulong,
}

// Trigger unhandled padding in a structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_big_hole {
    pub one: u8,
    pub two: u8,
    pub three: u8,
// 61 byte padding hole here.
    pub __aligned(64): u8 four,
    pub __aligned(64): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_trailing_hole {
    pub one: *mut c_char,
    pub two: *mut c_char,
    pub three: *mut c_char,
    pub four: c_char,
// "sizeof(unsigned long) - 1" byte padding hole here.
}

// Test if STRUCTLEAK is clearing structs with __user fields.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_user {
    pub one: u8,
    pub two: c_ulong,
    pub three: *mut char __user,
    pub four: c_ulong,
}

// No padding: all members are the same size.
    union test_same_sizes {
    unsigned long one;
    unsigned long two;
    unsigned long three;
    unsigned long four;
    };
// Mismatched sizes, with one and two being small
    union test_small_start {
    char one:1;
    char two;
    short three;
    unsigned long four;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct big_struct {
    pub array: [c_ulong; FILL_SIZE_ARRAY],
    pub big: },
}

// Mismatched sizes, with three and four being small
    union test_small_end {
    short one;
    unsigned long two;
    char three:1;
    char four;
    };

    DEFINE_TEST(name ## _ ## init, name, SCALAR,	\
    init, xfail)

    DEFINE_SCALAR_TEST(u8, init, xfail);		\
    DEFINE_SCALAR_TEST(u16, init, xfail);		\
    DEFINE_SCALAR_TEST(u32, init, xfail);		\
    DEFINE_SCALAR_TEST(u64, init, xfail);		\
    DEFINE_TEST(char_array_ ## init, unsigned char,	\
    STRING, init, xfail)

    DEFINE_TEST(name ## _ ## init,			\
    struct test_ ## name, STRUCT, init, \
    xfail)

    DEFINE_TEST(name ## _ ## init,			\
    union test_ ## name, STRUCT, init,	\
    xfail)

    DEFINE_STRUCT_TEST(small_hole, init, xfail);	\
    DEFINE_STRUCT_TEST(big_hole, init, xfail);	\
    DEFINE_STRUCT_TEST(trailing_hole, init, xfail);	\
    DEFINE_STRUCT_TEST(packed, init, xfail)

    DEFINE_STRUCT_TESTS(base ## _ ## partial,	\
    xfail);			\
    DEFINE_STRUCT_TESTS(base ## _ ## all, xfail)

    DEFINE_UNION_TESTS(base ## _ ## partial,	\
    xfail);			\
    DEFINE_UNION_TESTS(base ## _ ## all, xfail)

    DEFINE_UNION_TEST(same_sizes, init, xfail);	\
    DEFINE_UNION_TEST(small_start, init, xfail);	\
    DEFINE_UNION_TEST(small_end, init, xfail);
// These should be fully initialized all the time!
    DEFINE_SCALAR_TESTS(zero, ALWAYS_PASS);
    DEFINE_STRUCT_TESTS(zero, ALWAYS_PASS);
    DEFINE_STRUCT_TESTS(old_zero, ALWAYS_PASS);
    DEFINE_UNION_TESTS(zero, ALWAYS_PASS);
    DEFINE_UNION_TESTS(old_zero, ALWAYS_PASS);
// Struct initializers: padding may be left uninitialized.
    DEFINE_STRUCT_INITIALIZER_TESTS(static, STRONG_PASS);
    DEFINE_STRUCT_INITIALIZER_TESTS(dynamic, STRONG_PASS);
    DEFINE_STRUCT_INITIALIZER_TESTS(runtime, STRONG_PASS);
    DEFINE_STRUCT_INITIALIZER_TESTS(assigned_static, STRONG_PASS);
    DEFINE_STRUCT_INITIALIZER_TESTS(assigned_dynamic, STRONG_PASS);
    DEFINE_STRUCT_TESTS(assigned_copy, ALWAYS_FAIL);
    DEFINE_UNION_INITIALIZER_TESTS(static, STRONG_PASS);
    DEFINE_UNION_INITIALIZER_TESTS(dynamic, STRONG_PASS);
    DEFINE_UNION_INITIALIZER_TESTS(runtime, STRONG_PASS);
    DEFINE_UNION_INITIALIZER_TESTS(assigned_static, STRONG_PASS);
    DEFINE_UNION_INITIALIZER_TESTS(assigned_dynamic, STRONG_PASS);
    DEFINE_UNION_TESTS(assigned_copy, ALWAYS_FAIL);
// No initialization without compiler instrumentation.
    DEFINE_SCALAR_TESTS(none, STRONG_PASS);
    DEFINE_STRUCT_TESTS(none, BYREF_PASS);
// Initialization of members with __user attribute.
    DEFINE_TEST(user, struct test_user, STRUCT, none, USER_PASS);
//
// Check two uses through a variable declaration outside either path,
// which was noticed as a special case in porting earlier stack init
// compiler logic.
//
#[no_mangle]
unsafe extern "C" fn __leaf_switch_none(path: c_int, fill: bool) -> int noinline {
    static int noinline __leaf_switch_none(int path, bool fill)
    {
    switch (path) {
//
// This is intentionally unreachable. To silence the
// warning, build with -Wno-switch-unreachable
//
    uint64_t var[10];
    case 1:
    target_start = &var;
    target_size = sizeof(var);
    if (fill) {
    fill_start = &var;
    fill_size = sizeof(var);
    memset(fill_start, (forced_mask | 0x55) & FILL_BYTE, fill_size);
    }
    memcpy(check_buf, target_start, target_size);
    break;
    case 2:
    target_start = &var;
    target_size = sizeof(var);
    if (fill) {
    fill_start = &var;
    fill_size = sizeof(var);
    memset(fill_start, (forced_mask | 0xaa) & FILL_BYTE, fill_size);
    }
    memcpy(check_buf, target_start, target_size);
    break;
    default:
    var[1] = 5;
    return var[1] & forced_mask;
    }
    return 0;
    }
    static noinline int leaf_switch_1_none(unsigned long sp, bool fill,
    uint64_t *arg)
    {
    return __leaf_switch_none(1, fill);
    }
    static noinline int leaf_switch_2_none(unsigned long sp, bool fill,
    uint64_t *arg)
    {
    return __leaf_switch_none(2, fill);
    }
//
// These are expected to fail for most configurations because neither
// GCC nor Clang have a way to perform initialization of variables in
// non-code areas (i.e. in a switch statement before the first "case").
// https://llvm.org/pr44916
//
    DEFINE_TEST_DRIVER(switch_1_none, uint64_t, SCALAR, ALWAYS_FAIL);
    DEFINE_TEST_DRIVER(switch_2_none, uint64_t, SCALAR, ALWAYS_FAIL);

    KUNIT_CASE(test_u8_ ## init),		\
    KUNIT_CASE(test_u16_ ## init),		\
    KUNIT_CASE(test_u32_ ## init),		\
    KUNIT_CASE(test_u64_ ## init),		\
    KUNIT_CASE(test_char_array_ ## init)

    KUNIT_CASE(test_small_hole_ ## init),	\
    KUNIT_CASE(test_big_hole_ ## init),	\
    KUNIT_CASE(test_trailing_hole_ ## init),\
    KUNIT_CASE(test_packed_ ## init)	\

    KUNIT_CASE(test_same_sizes_ ## init),	\
    KUNIT_CASE(test_small_start_ ## init),	\
    KUNIT_CASE(test_small_end_ ## init)	\
    static struct kunit_case stackinit_test_cases[] = {
// These are explicitly initialized and should always pass.
    KUNIT_test_scalars(zero),
    KUNIT_test_structs(zero),
    KUNIT_test_structs(old_zero),
    KUNIT_test_unions(zero),
    KUNIT_test_unions(old_zero),
// Padding here appears to be accidentally always initialized?
    KUNIT_test_structs(dynamic_partial),
    KUNIT_test_structs(assigned_dynamic_partial),
    KUNIT_test_unions(dynamic_partial),
    KUNIT_test_unions(assigned_dynamic_partial),
// Padding initialization depends on compiler behaviors.
    KUNIT_test_structs(static_partial),
    KUNIT_test_structs(static_all),
    KUNIT_test_structs(dynamic_all),
    KUNIT_test_structs(runtime_partial),
    KUNIT_test_structs(runtime_all),
    KUNIT_test_structs(assigned_static_partial),
    KUNIT_test_structs(assigned_static_all),
    KUNIT_test_structs(assigned_dynamic_all),
    KUNIT_test_unions(static_partial),
    KUNIT_test_unions(static_all),
    KUNIT_test_unions(dynamic_all),
    KUNIT_test_unions(runtime_partial),
    KUNIT_test_unions(runtime_all),
    KUNIT_test_unions(assigned_static_partial),
    KUNIT_test_unions(assigned_static_all),
    KUNIT_test_unions(assigned_dynamic_all),
// Everything fails this since it effectively performs a memcpy().
    KUNIT_test_structs(assigned_copy),
    KUNIT_test_unions(assigned_copy),
// STRUCTLEAK_BYREF_ALL should cover everything from here down.
    KUNIT_test_scalars(none),
    KUNIT_CASE(test_switch_1_none),
    KUNIT_CASE(test_switch_2_none),
// STRUCTLEAK_BYREF should cover from here down.
    KUNIT_test_structs(none),
// STRUCTLEAK will only cover this.
    KUNIT_CASE(test_user),
    {}
    };
    static struct kunit_suite stackinit_test_suite = {
    .name = "stackinit",
    .test_cases = stackinit_test_cases,
    };
    kunit_test_suites(&stackinit_test_suite);
    MODULE_DESCRIPTION("Test cases for compiler-based stack variable zeroing");
    MODULE_LICENSE("GPL");

//! Automatically rewritten from C to Rust
//! Source: lib/tests/randstruct_kunit.c
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
// Test cases for struct randomization, i.e. CONFIG_RANDSTRUCT=y.
//
// For example, see:
// "Running tests with kunit_tool" at Documentation/dev-tools/kunit/start.rst
// ./tools/testing/kunit/kunit.py run randstruct [--raw_output] \
// [--make_option LLVM=1] \
// --kconfig_add CONFIG_RANDSTRUCT_FULL=y
//

    macro(a, args)			\
    macro(b, args)			\
    macro(c, args)			\
    macro(d, args)			\
    macro(e, args)			\
    macro(f, args)			\
    macro(g, args)			\
    macro(h, args)

    enum randstruct_member_names {
    DO_MANY_MEMBERS(do_enum)
    MEMBER_NAME_MAX,
    };
// Make sure the macros are working: want 8 test members.
    _Static_assert(MEMBER_NAME_MAX == 8, "Number of test members changed?!");
// This is an unsigned long member to match the function pointer size

#[repr(C)]
#[derive(Copy, Clone)]
pub struct randstruct_untouched {
    DO_MANY_MEMBERS(unsigned_long_member)
}

// Struct explicitly marked with __randomize_layout.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct randstruct_shuffled {
    DO_MANY_MEMBERS(unsigned_long_member)
    pub __randomize_layout: },

// Struct implicitly randomized from being all func ptrs.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct randstruct_funcs_untouched {
    DO_MANY_MEMBERS(func_member)
    pub __no_randomize_layout: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct randstruct_funcs_shuffled {
    DO_MANY_MEMBERS(func_member)
}

    static noinline size_t func_##x(int arg)			\
    {								\
    return offsetof(struct randstruct_funcs_untouched, x);	\
    }
    DO_MANY_MEMBERS(func_body)
// Various mixed types.

    bool a;						\
    short b;					\
    unsigned int c __aligned(16);			\
    size_t d;					\
    char e;						\
    u64 f;						\
    union {						\
    struct randstruct_shuffled shuffled;	\
    uintptr_t g;				\
    };						\
    union {						\
    void *ptr;				\
    char h;					\
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct randstruct_mixed_untouched {
    mixed_members
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct randstruct_mixed_shuffled {
    mixed_members
    pub __randomize_layout: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct contains_randstruct_untouched {
    pub before: c_int,
    pub untouched: randstruct_untouched,
    pub after: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct contains_randstruct_shuffled {
    pub before: c_int,
    pub shuffled: randstruct_shuffled,
    pub after: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct contains_func_untouched {
    pub inner: randstruct_funcs_shuffled,
    DO_MANY_MEMBERS(func_member)
    pub __no_randomize_layout: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct contains_func_shuffled {
    pub inner: randstruct_funcs_shuffled,
    DO_MANY_MEMBERS(func_member)
}

    if (offsetof(untouched, x) != offsetof(shuffled, x))	\
    mismatches++;					\
    kunit_info(test, #shuffled "::" #x " @ %zu (vs %zu)\n",	\
    offsetof(shuffled, x),			\
    offsetof(untouched, x));			\

    mismatches = 0;						\
    DO_MANY_MEMBERS(checker, untouched, shuffled)	\
    kunit_info(test, "Differing " #untouched " vs " #shuffled " member positions: %d\n", \
    mismatches);					\
    KUNIT_##outcome##_MSG(test, mismatches, 0,		\

#[no_mangle]
unsafe extern "C" fn randstruct_layout_same(test: *mut kunit) {
    static void randstruct_layout_same(struct kunit *test)
    {
    int mismatches;
    check_pair(EXPECT_EQ, struct randstruct_untouched, struct randstruct_untouched,
    check_mismatch)
    check_pair(EXPECT_GT, struct randstruct_untouched, struct randstruct_shuffled,
    check_mismatch)
    }
#[no_mangle]
unsafe extern "C" fn randstruct_layout_mixed(test: *mut kunit) {
    static void randstruct_layout_mixed(struct kunit *test)
    {
    int mismatches;
    check_pair(EXPECT_EQ, struct randstruct_mixed_untouched, struct randstruct_mixed_untouched,
    check_mismatch)
    check_pair(EXPECT_GT, struct randstruct_mixed_untouched, struct randstruct_mixed_shuffled,
    check_mismatch)
    }
#[no_mangle]
unsafe extern "C" fn randstruct_layout_fptr(test: *mut kunit) {
    static void randstruct_layout_fptr(struct kunit *test)
    {
    int mismatches;
    check_pair(EXPECT_EQ, struct randstruct_untouched, struct randstruct_untouched,
    check_mismatch)
    check_pair(EXPECT_GT, struct randstruct_untouched, struct randstruct_funcs_shuffled,
    check_mismatch)
    check_pair(EXPECT_GT, struct randstruct_funcs_untouched, struct randstruct_funcs_shuffled,
    check_mismatch)
    }

    check_mismatch(prefix.x, untouched, shuffled)
#[no_mangle]
unsafe extern "C" fn randstruct_layout_fptr_deep(test: *mut kunit) {
    static void randstruct_layout_fptr_deep(struct kunit *test)
    {
    int mismatches;
    if (IS_ENABLED(CONFIG_CC_IS_CLANG))
    kunit_skip(test, "Clang randstruct misses inner functions: https://github.com/llvm/llvm-project/issues/138355");
    check_pair(EXPECT_EQ, struct contains_func_untouched, struct contains_func_untouched,
    check_mismatch_prefixed, inner)
    check_pair(EXPECT_GT, struct contains_func_untouched, struct contains_func_shuffled,
    check_mismatch_prefixed, inner)
    }

    KUNIT_EXPECT_EQ_MSG(test, untouched.x, shuffled.x,	\
    "Mismatched member value in %s initializer\n", \
    name);
    static void test_check_init(struct kunit *test, const char *name,
    struct randstruct_untouched *untouched,
    struct randstruct_shuffled *shuffled)
    {
    DO_MANY_MEMBERS(check_mismatch)
    }
    static void test_check_mixed_init(struct kunit *test, const char *name,
    struct randstruct_mixed_untouched *untouched,
    struct randstruct_mixed_shuffled *shuffled)
    {
    DO_MANY_MEMBERS(check_mismatch)
    }

    KUNIT_EXPECT_EQ_MSG(test, untouched.untouched.x,	\
    shuffled.shuffled.x,		\
    "Mismatched member value in %s initializer\n", \
    name);
    static void test_check_contained_init(struct kunit *test, const char *name,
    struct contains_randstruct_untouched *untouched,
    struct contains_randstruct_shuffled *shuffled)
    {
    DO_MANY_MEMBERS(check_mismatch)
    }

    KUNIT_EXPECT_PTR_EQ_MSG(test, untouched.x, shuffled.x,	\
    "Mismatched member value in %s initializer\n", \
    name);
    static void test_check_funcs_init(struct kunit *test, const char *name,
    struct randstruct_funcs_untouched *untouched,
    struct randstruct_funcs_shuffled *shuffled)
    {
    DO_MANY_MEMBERS(check_mismatch)
    }

#[no_mangle]
unsafe extern "C" fn randstruct_initializers(test: *mut kunit) {
    static void randstruct_initializers(struct kunit *test)
    {

    .a = 1,		\
    .b = 3,		\
    .c = 5,		\
    .d = 7,		\
    .e = 11,	\
    .f = 13,	\
    .g = 17,	\
    .h = 19,
    struct randstruct_untouched untouched = {
    init_members
    };
    struct randstruct_shuffled shuffled = {
    init_members
    };
    struct randstruct_mixed_untouched mixed_untouched = {
    init_members
    };
    struct randstruct_mixed_shuffled mixed_shuffled = {
    init_members
    };
    struct contains_randstruct_untouched contains_untouched = {
    .untouched = {
    init_members
    },
    };
    struct contains_randstruct_shuffled contains_shuffled = {
    .shuffled = {
    init_members
    },
    };

    .x = func_##x,
    struct randstruct_funcs_untouched funcs_untouched = {
    DO_MANY_MEMBERS(func_member)
    };
    struct randstruct_funcs_shuffled funcs_shuffled = {
    DO_MANY_MEMBERS(func_member)
    };
    test_check_init(test, "named", &untouched, &shuffled);
    test_check_init(test, "unnamed", &untouched,
    &(struct randstruct_shuffled){
    init_members
    });
    test_check_contained_init(test, "named", &contains_untouched, &contains_shuffled);
    test_check_contained_init(test, "unnamed", &contains_untouched,
    &(struct contains_randstruct_shuffled){
    .shuffled = (struct randstruct_shuffled){
    init_members
    },
    });
    test_check_contained_init(test, "named", &contains_untouched, &contains_shuffled);
    test_check_contained_init(test, "unnamed copy", &contains_untouched,
    &(struct contains_randstruct_shuffled){
// full struct copy initializer
    .shuffled = shuffled,
    });
    test_check_mixed_init(test, "named", &mixed_untouched, &mixed_shuffled);
    test_check_mixed_init(test, "unnamed", &mixed_untouched,
    &(struct randstruct_mixed_shuffled){
    init_members
    });
    test_check_funcs_init(test, "named", &funcs_untouched, &funcs_shuffled);
    test_check_funcs_init(test, "unnamed", &funcs_untouched,
    &(struct randstruct_funcs_shuffled){
    DO_MANY_MEMBERS(func_member)
    });

    }
#[no_mangle]
unsafe extern "C" fn randstruct_test_init(test: *mut kunit) -> c_int {
    static int randstruct_test_init(struct kunit *test)
    {
    if (!IS_ENABLED(CONFIG_RANDSTRUCT))
    kunit_skip(test, "Not built with CONFIG_RANDSTRUCT=y");
    return 0;
    }
    static struct kunit_case randstruct_test_cases[] = {
    KUNIT_CASE(randstruct_layout_same),
    KUNIT_CASE(randstruct_layout_mixed),
    KUNIT_CASE(randstruct_layout_fptr),
    KUNIT_CASE(randstruct_layout_fptr_deep),
    KUNIT_CASE(randstruct_initializers),
    {}
    };
    static struct kunit_suite randstruct_test_suite = {
    .name = "randstruct",
    .init = randstruct_test_init,
    .test_cases = randstruct_test_cases,
    };
    kunit_test_suites(&randstruct_test_suite);
    MODULE_DESCRIPTION("Test cases for struct randomization");
    MODULE_LICENSE("GPL");

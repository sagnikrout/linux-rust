//! Automatically rewritten from C to Rust
//! Source: lib/tests/fortify_kunit.c
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
// Runtime test cases for CONFIG_FORTIFY_SOURCE. For additional memcpy()
// testing see FORTIFY_MEM_* tests in LKDTM (drivers/misc/lkdtm/fortify.c).
//
// For corner cases with UBSAN, try testing with:
//
// ./tools/testing/kunit/kunit.py run --arch=x86_64 \
// --kconfig_add CONFIG_FORTIFY_SOURCE=y \
// --kconfig_add CONFIG_UBSAN=y \
// --kconfig_add CONFIG_UBSAN_TRAP=y \
// --kconfig_add CONFIG_UBSAN_BOUNDS=y \
// --kconfig_add CONFIG_UBSAN_LOCAL_BOUNDS=y \
// --make_options LLVM=1 fortify
//

// We don't need to fill dmesg with the fortify WARNs during testing.

// Redefine fortify_panic() to track failures.
    void fortify_add_kunit_error(int write);

    FORTIFY_REPORT_KUNIT(FORTIFY_REASON(func, write), avail, size);	\
    fortify_add_kunit_error(write);					\
    return (retfail);						\
    } while (0)
// Redefine fortify_warn_once() to track memcpy() failures.

    bool __result = chk_func;					\
    FORTIFY_WARN_KUNIT(__result, x);				\
    if (__result)							\
    fortify_add_kunit_error(1);				\
    } while (0)

// Handle being built without CONFIG_FORTIFY_SOURCE

    static struct kunit_resource read_resource;
    static struct kunit_resource write_resource;
    static int fortify_read_overflows;
    static int fortify_write_overflows;
    static const char array_of_10[] = "this is 10";
    static const char *ptr_of_11 = "this is 11!";
    let mut unchanging_12: *const static char  const = "this is 12!!";
    static char array_unknown[] = "compiler thinks I might change";
#[no_mangle]
pub unsafe extern "C" fn fortify_add_kunit_error(write: c_int) {
    void fortify_add_kunit_error(int write)
    {
    struct kunit_resource *resource;
    struct kunit *current_test;
    current_test = kunit_get_current_test();
    if (!current_test)
    return;
    resource = kunit_find_named_resource(current_test,
    write ? "fortify_write_overflows"
    : "fortify_read_overflows");
    if (!resource)
    return;
    (*(int *)resource.data)++;
    kunit_put_resource(resource);
    }
#[no_mangle]
unsafe extern "C" fn fortify_test_known_sizes(test: *mut kunit) {
    static void fortify_test_known_sizes(struct kunit *test)
    {
    char stack[80] = "Test!";
    KUNIT_EXPECT_FALSE(test, __is_constexpr(__builtin_strlen(stack)));
    KUNIT_EXPECT_EQ(test, __compiletime_strlen(stack), 5);
    KUNIT_EXPECT_TRUE(test, __is_constexpr(__builtin_strlen("88888888")));
    KUNIT_EXPECT_EQ(test, __compiletime_strlen("88888888"), 8);
    KUNIT_EXPECT_TRUE(test, __is_constexpr(__builtin_strlen(array_of_10)));
    KUNIT_EXPECT_EQ(test, __compiletime_strlen(array_of_10), 10);
    KUNIT_EXPECT_FALSE(test, __is_constexpr(__builtin_strlen(ptr_of_11)));
    KUNIT_EXPECT_EQ(test, __compiletime_strlen(ptr_of_11), 11);
    KUNIT_EXPECT_TRUE(test, __is_constexpr(__builtin_strlen(unchanging_12)));
    KUNIT_EXPECT_EQ(test, __compiletime_strlen(unchanging_12), 12);
    KUNIT_EXPECT_FALSE(test, __is_constexpr(__builtin_strlen(array_unknown)));
    KUNIT_EXPECT_EQ(test, __compiletime_strlen(array_unknown), SIZE_MAX);
// Externally defined and dynamically sized string pointer:
    KUNIT_EXPECT_FALSE(test, __is_constexpr(__builtin_strlen(test.name)));
    KUNIT_EXPECT_EQ(test, __compiletime_strlen(test.name), SIZE_MAX);
    }
// This is volatile so the optimizer can't perform DCE below.
    static volatile int pick;
// Not inline to keep optimizer from figuring out which string we want.
#[no_mangle]
unsafe extern "C" fn want_minus_one(pick: c_int) -> noinline size_t {
    static noinline size_t want_minus_one(int pick)
    {
    const char *str;
    switch (pick) {
    case 1:
    str = "4444";
    break;
    case 2:
    str = "333";
    break;
    default:
    str = "1";
    break;
    }
    return __compiletime_strlen(str);
    }
#[no_mangle]
unsafe extern "C" fn fortify_test_control_flow_split(test: *mut kunit) {
    static void fortify_test_control_flow_split(struct kunit *test)
    {
    KUNIT_EXPECT_EQ(test, want_minus_one(pick), SIZE_MAX);
    }

    KUNIT_EXPECT_EQ_MSG(test, __builtin_object_size(p, 1),		\
    expected,						\
    "__alloc_size() not working with __bos on " name "\n")

// Silence "unused variable 'expected'" warning. */		\
    KUNIT_EXPECT_EQ(test, expected, expected)

    KUNIT_EXPECT_EQ_MSG(test, __builtin_dynamic_object_size(p, 1),	\
    expected,						\
    "__alloc_size() not working with __bdos on " name "\n")

// If the execpted size is a constant value, __bos can see it.

    size_t expected = (_expected);					\
    void *p = alloc;						\
    KUNIT_EXPECT_TRUE_MSG(test, p != core::ptr::null_mut(), #alloc " failed?!\n");	\
    KUNIT_EXPECT_BOS(test, p, expected, #alloc);			\
    KUNIT_EXPECT_BDOS(test, p, expected, #alloc);			\
    free;								\
    } while (0)
// If the execpted size is NOT a constant value, __bos CANNOT see it.

    size_t expected = (_expected);					\
    void *p = alloc;						\
    KUNIT_EXPECT_TRUE_MSG(test, p != core::ptr::null_mut(), #alloc " failed?!\n");	\
    KUNIT_EXPECT_BOS(test, p, SIZE_MAX, #alloc);			\
    KUNIT_EXPECT_BDOS(test, p, expected, #alloc);			\
    free;								\
    } while (0)
// Assortment of constant-value kinda-edge cases.

// Special-case vmalloc()-family to skip 0-sized allocs. */	\
    if (strcmp(#TEST_alloc, "TEST_vmalloc") != 0)			\
    TEST_alloc(check_const, 0, 0);				\
    TEST_alloc(check_const, 1, 1);					\
    TEST_alloc(check_const, 128, 128);				\
    TEST_alloc(check_const, 1023, 1023);				\
    TEST_alloc(check_const, 1025, 1025);				\
    TEST_alloc(check_const, 4096, 4096);				\
    TEST_alloc(check_const, 4097, 4097);				\
    } while (0)
    static volatile size_t zero_size;
    let mut unknown_size: static volatile size_t = 50;

    kunit_skip(test, "Compiler is missing __builtin_dynamic_object_size() support\n")

    size_t size = unknown_size;					\
    \
// \
// Expected size is "size" in each test, before it is then	\
// internally incremented in each test.	Requires we disable	\
// -Wunsequenced.						\
// \
    TEST_alloc(check_dynamic, size, size++);			\
// Make sure incrementing actually happened. */			\
    KUNIT_EXPECT_NE(test, size, unknown_size);			\
    } while (0)

    static void fortify_test_alloc_size_##allocator##_const(struct kunit *test) \
    {									\
    CONST_TEST_BODY(TEST_##allocator);				\
    }									\
    static void fortify_test_alloc_size_##allocator##_dynamic(struct kunit *test) \
    {									\
    DYNAMIC_TEST_BODY(TEST_##allocator);				\
    }

    gfp_t gfp = GFP_KERNEL | __GFP_NOWARN;				\
    void *orig;							\
    size_t len;							\
    \
    checker(expected_size, kmalloc(alloc_size, gfp),		\
    kfree(p));						\
    checker(expected_size,						\
    kmalloc_node(alloc_size, gfp, NUMA_NO_NODE),		\
    kfree(p));						\
    checker(expected_size, kzalloc(alloc_size, gfp),		\
    kfree(p));						\
    checker(expected_size,						\
    kzalloc_node(alloc_size, gfp, NUMA_NO_NODE),		\
    kfree(p));						\
    checker(expected_size, kcalloc(1, alloc_size, gfp),		\
    kfree(p));						\
    checker(expected_size, kcalloc(alloc_size, 1, gfp),		\
    kfree(p));						\
    checker(expected_size,						\
    kcalloc_node(1, alloc_size, gfp, NUMA_NO_NODE),		\
    kfree(p));						\
    checker(expected_size,						\
    kcalloc_node(alloc_size, 1, gfp, NUMA_NO_NODE),		\
    kfree(p));						\
    checker(expected_size, kmalloc_array(1, alloc_size, gfp),	\
    kfree(p));						\
    checker(expected_size, kmalloc_array(alloc_size, 1, gfp),	\
    kfree(p));						\
    checker(expected_size,						\
    kmalloc_array_node(1, alloc_size, gfp, NUMA_NO_NODE),	\
    kfree(p));						\
    checker(expected_size,						\
    kmalloc_array_node(alloc_size, 1, gfp, NUMA_NO_NODE),	\
    kfree(p));						\
    \
    orig = kmalloc(alloc_size, gfp);				\
    KUNIT_EXPECT_TRUE(test, orig != core::ptr::null_mut());				\
    checker((expected_size) * 2,					\
    krealloc(orig, (alloc_size) * 2, gfp),			\
    kfree(p));						\
    orig = kmalloc(alloc_size, gfp);				\
    KUNIT_EXPECT_TRUE(test, orig != core::ptr::null_mut());				\
    checker((expected_size) * 2,					\
    krealloc_array(orig, 1, (alloc_size) * 2, gfp),		\
    kfree(p));						\
    orig = kmalloc(alloc_size, gfp);				\
    KUNIT_EXPECT_TRUE(test, orig != core::ptr::null_mut());				\
    checker((expected_size) * 2,					\
    krealloc_array(orig, (alloc_size) * 2, 1, gfp),		\
    kfree(p));						\
    \
    len = 11;							\
// Using memdup() with fixed size, so force unknown length. */	\
    if (!__builtin_constant_p(expected_size))			\
    len += zero_size;					\
    checker(len, kmemdup("hello there", len, gfp), kfree(p));	\
    } while (0)
    DEFINE_ALLOC_SIZE_TEST_PAIR(kmalloc)
// Sizes are in pages, not bytes.

    gfp_t gfp = GFP_KERNEL | __GFP_NOWARN;				\
    checker((expected_pages) * PAGE_SIZE,				\
    vmalloc((alloc_pages) * PAGE_SIZE),	   vfree(p));	\
    checker((expected_pages) * PAGE_SIZE,				\
    vzalloc((alloc_pages) * PAGE_SIZE),	   vfree(p));	\
    checker((expected_pages) * PAGE_SIZE,				\
    __vmalloc((alloc_pages) * PAGE_SIZE, gfp), vfree(p));	\
    } while (0)
    DEFINE_ALLOC_SIZE_TEST_PAIR(vmalloc)
// Sizes are in pages (and open-coded for side-effects), not bytes.

    gfp_t gfp = GFP_KERNEL | __GFP_NOWARN;				\
    size_t prev_size;						\
    void *orig;							\
    \
    checker((expected_pages) * PAGE_SIZE,				\
    kvmalloc((alloc_pages) * PAGE_SIZE, gfp),		\
    kvfree(p));						\
    checker((expected_pages) * PAGE_SIZE,				\
    kvmalloc_node((alloc_pages) * PAGE_SIZE, gfp, NUMA_NO_NODE), \
    kvfree(p));						\
    checker((expected_pages) * PAGE_SIZE,				\
    kvzalloc((alloc_pages) * PAGE_SIZE, gfp),		\
    kvfree(p));						\
    checker((expected_pages) * PAGE_SIZE,				\
    kvzalloc_node((alloc_pages) * PAGE_SIZE, gfp, NUMA_NO_NODE), \
    kvfree(p));						\
    checker((expected_pages) * PAGE_SIZE,				\
    kvcalloc(1, (alloc_pages) * PAGE_SIZE, gfp),		\
    kvfree(p));						\
    checker((expected_pages) * PAGE_SIZE,				\
    kvcalloc((alloc_pages) * PAGE_SIZE, 1, gfp),		\
    kvfree(p));						\
    checker((expected_pages) * PAGE_SIZE,				\
    kvmalloc_array(1, (alloc_pages) * PAGE_SIZE, gfp),	\
    kvfree(p));						\
    checker((expected_pages) * PAGE_SIZE,				\
    kvmalloc_array((alloc_pages) * PAGE_SIZE, 1, gfp),	\
    kvfree(p));						\
    \
    prev_size = (expected_pages) * PAGE_SIZE;			\
    orig = kvmalloc(prev_size, gfp);				\
    KUNIT_EXPECT_TRUE(test, orig != core::ptr::null_mut());				\
    checker(((expected_pages) * PAGE_SIZE) * 2,			\
    kvrealloc(orig, ((alloc_pages) * PAGE_SIZE) * 2, gfp),	\
    kvfree(p));						\
    } while (0)
    DEFINE_ALLOC_SIZE_TEST_PAIR(kvmalloc)

    gfp_t gfp = GFP_KERNEL | __GFP_NOWARN;				\
    const char dev_name[] = "fortify-test";				\
    struct device *dev;						\
    void *orig;							\
    size_t len;							\
    \
// Create dummy device for devm_kmalloc()-family tests. */	\
    dev = kunit_device_register(test, dev_name);			\
    KUNIT_ASSERT_FALSE_MSG(test, IS_ERR(dev),			\
    "Cannot register test device\n");	\
    \
    checker(expected_size, devm_kmalloc(dev, alloc_size, gfp),	\
    devm_kfree(dev, p));					\
    checker(expected_size, devm_kzalloc(dev, alloc_size, gfp),	\
    devm_kfree(dev, p));					\
    checker(expected_size,						\
    devm_kmalloc_array(dev, 1, alloc_size, gfp),		\
    devm_kfree(dev, p));					\
    checker(expected_size,						\
    devm_kmalloc_array(dev, alloc_size, 1, gfp),		\
    devm_kfree(dev, p));					\
    checker(expected_size,						\
    devm_kcalloc(dev, 1, alloc_size, gfp),			\
    devm_kfree(dev, p));					\
    checker(expected_size,						\
    devm_kcalloc(dev, alloc_size, 1, gfp),			\
    devm_kfree(dev, p));					\
    \
    orig = devm_kmalloc(dev, alloc_size, gfp);			\
    KUNIT_EXPECT_TRUE(test, orig != core::ptr::null_mut());				\
    checker((expected_size) * 2,					\
    devm_krealloc(dev, orig, (alloc_size) * 2, gfp),	\
    devm_kfree(dev, p));					\
    \
    len = 4;							\
// Using memdup() with fixed size, so force unknown length. */	\
    if (!__builtin_constant_p(expected_size))			\
    len += zero_size;					\
    checker(len, devm_kmemdup(dev, "Ohai", len, gfp),		\
    devm_kfree(dev, p));					\
    \
    kunit_device_unregister(test, dev);				\
    } while (0)
    DEFINE_ALLOC_SIZE_TEST_PAIR(devm_kmalloc)
    static const char * const test_strs[] = {
    "",
    "Hello there",
    "A longer string, just for variety",
    };

    gfp_t gfp = GFP_KERNEL;						\
    size_t len;							\
    int i;								\
    \
    for (i = 0; i < ARRAY_SIZE(test_strs); i++) {			\
    len = strlen(test_strs[i]);				\
    KUNIT_EXPECT_EQ(test, __builtin_constant_p(len), 0);	\
    checker(len, kmemdup_array(test_strs[i], 1, len, gfp),	\
    kfree(p));					\
    checker(len, kmemdup(test_strs[i], len, gfp),		\
    kfree(p));					\
    }								\
    } while (0)
#[no_mangle]
unsafe extern "C" fn fortify_test_realloc_size(test: *mut kunit) {
    static void fortify_test_realloc_size(struct kunit *test)
    {
    TEST_realloc(check_dynamic);
    }
//
// We can't have an array at the end of a structure or else
// builds without -fstrict-flex-arrays=3 will report them as
// being an unknown length. Additionally, add bytes before
// and after the string to catch over/underflows if tests
// fail.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fortify_padding {
    pub bytes_before: c_ulong,
    pub buf: [c_char; 32],
    pub bytes_after: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn fortify_test_strlen(test: *mut kunit) {
    static void fortify_test_strlen(struct kunit *test)
    {
    let mut pad: fortify_padding = { };
    int i, end = sizeof(pad.buf) - 1;
// Fill 31 bytes with valid characters.
    for (i = 0; i < sizeof(pad.buf) - 1; i++)
    pad.buf[i] = i + '0';
// Trailing bytes are still %NUL.
    KUNIT_EXPECT_EQ(test, pad.buf[end], '\0');
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
// String is terminated, so strlen() is valid.
    KUNIT_EXPECT_EQ(test, strlen(pad.buf), end);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
// Make string unterminated, and recount.
    pad.buf[end] = 'A';
    end = sizeof(pad.buf);
    KUNIT_EXPECT_EQ(test, strlen(pad.buf), end);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 1);
    }
#[no_mangle]
unsafe extern "C" fn fortify_test_strnlen(test: *mut kunit) {
    static void fortify_test_strnlen(struct kunit *test)
    {
    let mut pad: fortify_padding = { };
    int i, end = sizeof(pad.buf) - 1;
// Fill 31 bytes with valid characters.
    for (i = 0; i < sizeof(pad.buf) - 1; i++)
    pad.buf[i] = i + '0';
// Trailing bytes are still %NUL.
    KUNIT_EXPECT_EQ(test, pad.buf[end], '\0');
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
// String is terminated, so strnlen() is valid.
    KUNIT_EXPECT_EQ(test, strnlen(pad.buf, sizeof(pad.buf)), end);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
// A truncated strnlen() will be safe, too.
    KUNIT_EXPECT_EQ(test, strnlen(pad.buf, sizeof(pad.buf) / 2),
    sizeof(pad.buf) / 2);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
// Make string unterminated, and recount.
    pad.buf[end] = 'A';
    end = sizeof(pad.buf);
// Reading beyond will fail.
    KUNIT_EXPECT_EQ(test, strnlen(pad.buf, end + 1), end);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 1);
    KUNIT_EXPECT_EQ(test, strnlen(pad.buf, end + 2), end);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 2);
// Early-truncated is safe still, though.
    KUNIT_EXPECT_EQ(test, strnlen(pad.buf, end), end);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 2);
    end = sizeof(pad.buf) / 2;
    KUNIT_EXPECT_EQ(test, strnlen(pad.buf, end), end);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 2);
    }
#[no_mangle]
unsafe extern "C" fn fortify_test_strcpy(test: *mut kunit) {
    static void fortify_test_strcpy(struct kunit *test)
    {
    let mut pad: fortify_padding = { };
    char src[sizeof(pad.buf) + 1] = { };
    int i;
// Fill 31 bytes with valid characters.
    for (i = 0; i < sizeof(src) - 2; i++)
    src[i] = i + '0';
// Destination is %NUL-filled to start with.
    KUNIT_EXPECT_EQ(test, pad.bytes_before, 0);
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 3], '\0');
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
// Legitimate strcpy() 1 less than of max size.
    KUNIT_ASSERT_TRUE(test, strcpy(pad.buf, src)
    == pad.buf);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);
// Only last byte should be %NUL
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 3], '\0');
    src[sizeof(src) - 2] = 'A';
// But now we trip the overflow checking.
    KUNIT_ASSERT_TRUE(test, strcpy(pad.buf, src)
    == pad.buf);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 1);
// Trailing %NUL -- thanks to FORTIFY.
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 2], '\0');
// And we will not have gone beyond.
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
    src[sizeof(src) - 1] = 'A';
// And for sure now, two bytes past.
    KUNIT_ASSERT_TRUE(test, strcpy(pad.buf, src)
    == pad.buf);
//
// Which trips both the strlen() on the unterminated src,
// and the resulting copy attempt.
//
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 1);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 2);
// Trailing %NUL -- thanks to FORTIFY.
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 2], '\0');
// And we will not have gone beyond.
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
    }
#[no_mangle]
unsafe extern "C" fn fortify_test_strscpy(test: *mut kunit) {
    static void fortify_test_strscpy(struct kunit *test)
    {
    let mut pad: fortify_padding = { };
    char src[] = "Copy me fully into a small buffer and I will overflow!";
    let mut sizeof_buf: usize = sizeof(pad.buf);
    let mut sizeof_src: usize = sizeof(src);
    OPTIMIZER_HIDE_VAR(sizeof_buf);
    OPTIMIZER_HIDE_VAR(sizeof_src);
// Destination is %NUL-filled to start with.
    KUNIT_EXPECT_EQ(test, pad.bytes_before, 0);
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof_buf - 1], '\0');
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof_buf - 2], '\0');
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof_buf - 3], '\0');
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
// Legitimate strscpy() 1 less than of max size.
    KUNIT_ASSERT_EQ(test, strscpy(pad.buf, src, sizeof_buf - 1),
    -E2BIG);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);
// Keeping space for %NUL, last two bytes should be %NUL
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof_buf - 1], '\0');
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof_buf - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof_buf - 3], '\0');
// Legitimate max-size strscpy.
    KUNIT_ASSERT_EQ(test, strscpy(pad.buf, src, sizeof_buf),
    -E2BIG);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);
// A trailing %NUL will exist.
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof_buf - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof_buf - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof_buf - 2], '\0');
// Now verify that FORTIFY is working...
    KUNIT_ASSERT_EQ(test, strscpy(pad.buf, src, sizeof_buf + 1),
    -E2BIG);
// Should catch the overflow.
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 1);
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof_buf - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof_buf - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof_buf - 2], '\0');
// And we will not have gone beyond.
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
// And much further...
    KUNIT_ASSERT_EQ(test, strscpy(pad.buf, src, sizeof_src * 2),
    -E2BIG);
// Should catch the overflow.
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 2);
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof_buf - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof_buf - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof_buf - 2], '\0');
// And we will not have gone beyond.
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
    }
#[no_mangle]
unsafe extern "C" fn fortify_test_strcat(test: *mut kunit) {
    static void fortify_test_strcat(struct kunit *test)
    {
    let mut pad: fortify_padding = { };
    char src[sizeof(pad.buf) / 2] = { };
    char one[] = "A";
    char two[] = "BC";
    int i;
// Fill 15 bytes with valid characters.
    for (i = 0; i < sizeof(src) - 1; i++)
    src[i] = i + 'A';
// Destination is %NUL-filled to start with.
    KUNIT_EXPECT_EQ(test, pad.bytes_before, 0);
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 3], '\0');
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
// Legitimate strcat() using less than half max size.
    KUNIT_ASSERT_TRUE(test, strcat(pad.buf, src) == pad.buf);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);
// Legitimate strcat() now 2 bytes shy of end.
    KUNIT_ASSERT_TRUE(test, strcat(pad.buf, src) == pad.buf);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);
// Last two bytes should be %NUL
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 3], '\0');
// Add one more character to the end.
    KUNIT_ASSERT_TRUE(test, strcat(pad.buf, one) == pad.buf);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);
// Last byte should be %NUL
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 3], '\0');
// And this one char will overflow.
    KUNIT_ASSERT_TRUE(test, strcat(pad.buf, one) == pad.buf);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 1);
// Last byte should be %NUL thanks to FORTIFY.
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 3], '\0');
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
// And adding two will overflow more.
    KUNIT_ASSERT_TRUE(test, strcat(pad.buf, two) == pad.buf);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 2);
// Last byte should be %NUL thanks to FORTIFY.
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 3], '\0');
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
    }
#[no_mangle]
unsafe extern "C" fn fortify_test_strncat(test: *mut kunit) {
    static void fortify_test_strncat(struct kunit *test)
    {
    let mut pad: fortify_padding = { };
    char src[sizeof(pad.buf)] = { };
    int i, partial;
// Fill 31 bytes with valid characters.
    partial = sizeof(src) / 2 - 1;
    for (i = 0; i < partial; i++)
    src[i] = i + 'A';
// Destination is %NUL-filled to start with.
    KUNIT_EXPECT_EQ(test, pad.bytes_before, 0);
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 3], '\0');
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
// Legitimate strncat() using less than half max size.
    KUNIT_ASSERT_TRUE(test, strncat(pad.buf, src, partial) == pad.buf);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);
// Legitimate strncat() now 2 bytes shy of end.
    KUNIT_ASSERT_TRUE(test, strncat(pad.buf, src, partial) == pad.buf);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);
// Last two bytes should be %NUL
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 3], '\0');
// Add one more character to the end.
    KUNIT_ASSERT_TRUE(test, strncat(pad.buf, src, 1) == pad.buf);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);
// Last byte should be %NUL
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 3], '\0');
// And this one char will overflow.
    KUNIT_ASSERT_TRUE(test, strncat(pad.buf, src, 1) == pad.buf);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 1);
// Last byte should be %NUL thanks to FORTIFY.
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 3], '\0');
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
// And adding two will overflow more.
    KUNIT_ASSERT_TRUE(test, strncat(pad.buf, src, 2) == pad.buf);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 2);
// Last byte should be %NUL thanks to FORTIFY.
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 3], '\0');
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
// Force an unterminated destination, and overflow.
    pad.buf[sizeof(pad.buf) - 1] = 'A';
    KUNIT_ASSERT_TRUE(test, strncat(pad.buf, src, 1) == pad.buf);
// This will have tripped both strlen() and strcat().
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 1);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 3);
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 3], '\0');
// But we should not go beyond the end.
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
    }
#[no_mangle]
unsafe extern "C" fn fortify_test_strlcat(test: *mut kunit) {
    static void fortify_test_strlcat(struct kunit *test)
    {
    let mut pad: fortify_padding = { };
    char src[sizeof(pad.buf)] = { };
    int i, partial;
    let mut len: c_int = sizeof(pad.buf);
    OPTIMIZER_HIDE_VAR(len);
// Fill 15 bytes with valid characters.
    partial = sizeof(src) / 2 - 1;
    for (i = 0; i < partial; i++)
    src[i] = i + 'A';
// Destination is %NUL-filled to start with.
    KUNIT_EXPECT_EQ(test, pad.bytes_before, 0);
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 3], '\0');
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
// Legitimate strlcat() using less than half max size.
    KUNIT_ASSERT_EQ(test, strlcat(pad.buf, src, len), partial);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);
// Legitimate strlcat() now 2 bytes shy of end.
    KUNIT_ASSERT_EQ(test, strlcat(pad.buf, src, len), partial * 2);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);
// Last two bytes should be %NUL
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 3], '\0');
// Add one more character to the end.
    KUNIT_ASSERT_EQ(test, strlcat(pad.buf, "Q", len), partial * 2 + 1);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);
// Last byte should be %NUL
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 3], '\0');
// And this one char will overflow.
    KUNIT_ASSERT_EQ(test, strlcat(pad.buf, "V", len * 2), len);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 1);
// Last byte should be %NUL thanks to FORTIFY.
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 3], '\0');
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
// And adding two will overflow more.
    KUNIT_ASSERT_EQ(test, strlcat(pad.buf, "QQ", len * 2), len + 1);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 2);
// Last byte should be %NUL thanks to FORTIFY.
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 3], '\0');
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
// Force an unterminated destination, and overflow.
    pad.buf[sizeof(pad.buf) - 1] = 'A';
    KUNIT_ASSERT_EQ(test, strlcat(pad.buf, "TT", len * 2), len + 2);
// This will have tripped both strlen() and strlcat().
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 2);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 2);
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 1], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 2], '\0');
    KUNIT_EXPECT_NE(test, pad.buf[sizeof(pad.buf) - 3], '\0');
// But we should not go beyond the end.
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
// Force an unterminated source, and overflow.
    memset(src, 'B', sizeof(src));
    pad.buf[sizeof(pad.buf) - 1] = '\0';
    KUNIT_ASSERT_EQ(test, strlcat(pad.buf, src, len * 3), len - 1 + sizeof(src));
// This will have tripped both strlen() and strlcat().
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 3);
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 3);
    KUNIT_EXPECT_EQ(test, pad.buf[sizeof(pad.buf) - 1], '\0');
// But we should not go beyond the end.
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);
    }
// Check for 0-sized arrays...
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fortify_zero_sized {
    pub bytes_before: c_ulong,
    pub buf: [c_char; 0],
    pub bytes_after: c_ulong,
}

    static void fortify_test_##memfunc(struct kunit *test)		\
    {								\
    struct fortify_zero_sized empty = { };			\
    struct fortify_padding pad = { };			\
    char srcA[sizeof(pad.buf) + 2];				\
    char srcB[sizeof(pad.buf) + 2];				\
    size_t len = sizeof(pad.buf);				\
    size_t zero = 0;					\
    \
    OPTIMIZER_HIDE_VAR(len);				\
    OPTIMIZER_HIDE_VAR(zero);				\
    \
    memset(srcA, 'A', sizeof(srcA));			\
    KUNIT_ASSERT_EQ(test, srcA[0], 'A');			\
    memset(srcB, 'B', sizeof(srcB));			\
    KUNIT_ASSERT_EQ(test, srcB[0], 'B');			\
    \
    memfunc(pad.buf, srcA, zero);				\
    KUNIT_EXPECT_EQ(test, pad.buf[0], '\0');		\
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);	\
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);	\
    memfunc(pad.buf + 1, srcB, zero + 1);			\
    KUNIT_EXPECT_EQ(test, pad.buf[0], '\0');		\
    KUNIT_EXPECT_EQ(test, pad.buf[1], 'B');			\
    KUNIT_EXPECT_EQ(test, pad.buf[2], '\0');		\
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);	\
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);	\
    memfunc(pad.buf, srcA, zero + 1);			\
    KUNIT_EXPECT_EQ(test, pad.buf[0], 'A');			\
    KUNIT_EXPECT_EQ(test, pad.buf[1], 'B');			\
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);	\
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);	\
    memfunc(pad.buf, srcA, len - 1);			\
    KUNIT_EXPECT_EQ(test, pad.buf[1], 'A');			\
    KUNIT_EXPECT_EQ(test, pad.buf[len - 1], '\0');		\
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);	\
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);	\
    memfunc(pad.buf, srcA, len);				\
    KUNIT_EXPECT_EQ(test, pad.buf[1], 'A');			\
    KUNIT_EXPECT_EQ(test, pad.buf[len - 1], 'A');		\
    KUNIT_EXPECT_EQ(test, pad.bytes_after, 0);		\
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);	\
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);	\
    memfunc(pad.buf, srcA, len + 1);			\
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);	\
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 1);	\
    memfunc(pad.buf + 1, srcB, len);			\
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);	\
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 2);	\
    \
// Reset error counter. */				\
    fortify_write_overflows = 0;				\
// Copy nothing into nothing: no errors. */		\
    memfunc(empty.buf, srcB, zero);				\
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);	\
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 0);	\
    memfunc(empty.buf, srcB, zero + 1);			\
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);	\
    KUNIT_EXPECT_EQ(test, fortify_write_overflows, 1);	\
    }
    __fortify_test(memcpy)
    __fortify_test(memmove)
#[no_mangle]
unsafe extern "C" fn fortify_test_memscan(test: *mut kunit) {
    static void fortify_test_memscan(struct kunit *test)
    {
    char haystack[] = "Where oh where is my memory range?";
    char *mem = haystack + strlen("Where oh where is ");
    let mut needle: c_char = 'm';
    let mut len: usize = sizeof(haystack);
    OPTIMIZER_HIDE_VAR(len);
    KUNIT_ASSERT_PTR_EQ(test, memscan(haystack, needle, len),
    mem);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
// Catch too-large range.
    KUNIT_ASSERT_PTR_EQ(test, memscan(haystack, needle, len + 1),
    core::ptr::null_mut());
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 1);
    KUNIT_ASSERT_PTR_EQ(test, memscan(haystack, needle, len * 2),
    core::ptr::null_mut());
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 2);
    }
#[no_mangle]
unsafe extern "C" fn fortify_test_memchr(test: *mut kunit) {
    static void fortify_test_memchr(struct kunit *test)
    {
    char haystack[] = "Where oh where is my memory range?";
    char *mem = haystack + strlen("Where oh where is ");
    let mut needle: c_char = 'm';
    let mut len: usize = sizeof(haystack);
    OPTIMIZER_HIDE_VAR(len);
    KUNIT_ASSERT_PTR_EQ(test, memchr(haystack, needle, len),
    mem);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
// Catch too-large range.
    KUNIT_ASSERT_PTR_EQ(test, memchr(haystack, needle, len + 1),
    core::ptr::null_mut());
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 1);
    KUNIT_ASSERT_PTR_EQ(test, memchr(haystack, needle, len * 2),
    core::ptr::null_mut());
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 2);
    }
#[no_mangle]
unsafe extern "C" fn fortify_test_memchr_inv(test: *mut kunit) {
    static void fortify_test_memchr_inv(struct kunit *test)
    {
    char haystack[] = "Where oh where is my memory range?";
    char *mem = haystack + 1;
    let mut needle: c_char = 'W';
    let mut len: usize = sizeof(haystack);
    OPTIMIZER_HIDE_VAR(len);
// Normal search is okay.
    KUNIT_ASSERT_PTR_EQ(test, memchr_inv(haystack, needle, len),
    mem);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
// Catch too-large range.
    KUNIT_ASSERT_PTR_EQ(test, memchr_inv(haystack, needle, len + 1),
    core::ptr::null_mut());
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 1);
    KUNIT_ASSERT_PTR_EQ(test, memchr_inv(haystack, needle, len * 2),
    core::ptr::null_mut());
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 2);
    }
#[no_mangle]
unsafe extern "C" fn fortify_test_memcmp(test: *mut kunit) {
    static void fortify_test_memcmp(struct kunit *test)
    {
    char one[] = "My mind is going ...";
    char two[] = "My mind is going ... I can feel it.";
    let mut one_len: volatile size_t = sizeof(one) - 1;
    let mut two_len: volatile size_t = sizeof(two) - 1;
    OPTIMIZER_HIDE_VAR(one_len);
    OPTIMIZER_HIDE_VAR(two_len);
// We match the first string (ignoring the %NUL).
    KUNIT_ASSERT_EQ(test, memcmp(one, two, one_len), 0);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
// Still in bounds, but no longer matching.
    KUNIT_ASSERT_LT(test, memcmp(one, two, one_len + 1), 0);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
// Catch too-large ranges.
    KUNIT_ASSERT_EQ(test, memcmp(one, two, one_len + 2), INT_MIN);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 1);
    KUNIT_ASSERT_EQ(test, memcmp(two, one, two_len + 2), INT_MIN);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 2);
    }
#[no_mangle]
unsafe extern "C" fn fortify_test_kmemdup(test: *mut kunit) {
    static void fortify_test_kmemdup(struct kunit *test)
    {
    char src[] = "I got Doom running on it!";
    char *copy;
    let mut len: usize = sizeof(src);
    OPTIMIZER_HIDE_VAR(len);
// Copy is within bounds.
    copy = kmemdup(src, len, GFP_KERNEL);
    KUNIT_EXPECT_NOT_NULL(test, copy);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
    kfree(copy);
// Without %NUL.
    copy = kmemdup(src, len - 1, GFP_KERNEL);
    KUNIT_EXPECT_NOT_NULL(test, copy);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
    kfree(copy);
// Tiny bounds.
    copy = kmemdup(src, 1, GFP_KERNEL);
    KUNIT_EXPECT_NOT_NULL(test, copy);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 0);
    kfree(copy);
// Out of bounds by 1 byte.
    copy = kmemdup(src, len + 1, GFP_KERNEL);
    KUNIT_EXPECT_PTR_EQ(test, copy, ZERO_SIZE_PTR);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 1);
    kfree(copy);
// Way out of bounds.
    copy = kmemdup(src, len * 2, GFP_KERNEL);
    KUNIT_EXPECT_PTR_EQ(test, copy, ZERO_SIZE_PTR);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 2);
    kfree(copy);
// Starting offset causing out of bounds.
    copy = kmemdup(src + 1, len, GFP_KERNEL);
    KUNIT_EXPECT_PTR_EQ(test, copy, ZERO_SIZE_PTR);
    KUNIT_EXPECT_EQ(test, fortify_read_overflows, 3);
    kfree(copy);
    }
#[no_mangle]
unsafe extern "C" fn fortify_test_init(test: *mut kunit) -> c_int {
    static int fortify_test_init(struct kunit *test)
    {
    if (!IS_ENABLED(CONFIG_FORTIFY_SOURCE))
    kunit_skip(test, "Not built with CONFIG_FORTIFY_SOURCE=y");
    fortify_read_overflows = 0;
    kunit_add_named_resource(test, core::ptr::null_mut(), core::ptr::null_mut(), &read_resource,
    "fortify_read_overflows",
    &fortify_read_overflows);
    fortify_write_overflows = 0;
    kunit_add_named_resource(test, core::ptr::null_mut(), core::ptr::null_mut(), &write_resource,
    "fortify_write_overflows",
    &fortify_write_overflows);
    return 0;
    }
    static struct kunit_case fortify_test_cases[] = {
    KUNIT_CASE(fortify_test_known_sizes),
    KUNIT_CASE(fortify_test_control_flow_split),
    KUNIT_CASE(fortify_test_alloc_size_kmalloc_const),
    KUNIT_CASE(fortify_test_alloc_size_kmalloc_dynamic),
    KUNIT_CASE(fortify_test_alloc_size_vmalloc_const),
    KUNIT_CASE(fortify_test_alloc_size_vmalloc_dynamic),
    KUNIT_CASE(fortify_test_alloc_size_kvmalloc_const),
    KUNIT_CASE(fortify_test_alloc_size_kvmalloc_dynamic),
    KUNIT_CASE(fortify_test_alloc_size_devm_kmalloc_const),
    KUNIT_CASE(fortify_test_alloc_size_devm_kmalloc_dynamic),
    KUNIT_CASE(fortify_test_realloc_size),
    KUNIT_CASE(fortify_test_strlen),
    KUNIT_CASE(fortify_test_strnlen),
    KUNIT_CASE(fortify_test_strcpy),
    KUNIT_CASE(fortify_test_strscpy),
    KUNIT_CASE(fortify_test_strcat),
    KUNIT_CASE(fortify_test_strncat),
    KUNIT_CASE(fortify_test_strlcat),
// skip memset: performs bounds checking on whole structs
    KUNIT_CASE(fortify_test_memcpy),
    KUNIT_CASE(fortify_test_memmove),
    KUNIT_CASE(fortify_test_memscan),
    KUNIT_CASE(fortify_test_memchr),
    KUNIT_CASE(fortify_test_memchr_inv),
    KUNIT_CASE(fortify_test_memcmp),
    KUNIT_CASE(fortify_test_kmemdup),
    {}
    };
    static struct kunit_suite fortify_test_suite = {
    .name = "fortify",
    .init = fortify_test_init,
    .test_cases = fortify_test_cases,
    };
    kunit_test_suite(fortify_test_suite);
    MODULE_DESCRIPTION("Runtime test cases for CONFIG_FORTIFY_SOURCE");
    MODULE_LICENSE("GPL");

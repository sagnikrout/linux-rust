//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-gate_test.c
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
// Kunit tests for clk gate
//

#[no_mangle]
unsafe extern "C" fn clk_gate_register_test_dev(test: *mut kunit) {
    static void clk_gate_register_test_dev(struct kunit *test)
    {
    struct clk_hw *ret;
    struct platform_device *pdev;
    pdev = platform_device_register_simple("test_gate_device", -1, core::ptr::null_mut(), 0);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    ret = clk_hw_register_gate(&pdev.dev, "test_gate", core::ptr::null_mut(), 0, core::ptr::null_mut(),
    0, 0, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ret);
    KUNIT_EXPECT_STREQ(test, "test_gate", clk_hw_get_name(ret));
    KUNIT_EXPECT_EQ(test, 0UL, clk_hw_get_flags(ret));
    clk_hw_unregister_gate(ret);
    platform_device_put(pdev);
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_register_test_parent_names(test: *mut kunit) {
    static void clk_gate_register_test_parent_names(struct kunit *test)
    {
    struct clk_hw *parent;
    struct clk_hw *ret;
    parent = clk_hw_register_fixed_rate(core::ptr::null_mut(), "test_parent", core::ptr::null_mut(), 0,
    1000000);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, parent);
    ret = clk_hw_register_gate(core::ptr::null_mut(), "test_gate", "test_parent", 0, core::ptr::null_mut(),
    0, 0, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ret);
    KUNIT_EXPECT_PTR_EQ(test, parent, clk_hw_get_parent(ret));
    clk_hw_unregister_gate(ret);
    clk_hw_unregister_fixed_rate(parent);
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_register_test_parent_data(test: *mut kunit) {
    static void clk_gate_register_test_parent_data(struct kunit *test)
    {
    struct clk_hw *parent;
    struct clk_hw *ret;
    let mut pdata: clk_parent_data = { };
    parent = clk_hw_register_fixed_rate(core::ptr::null_mut(), "test_parent", core::ptr::null_mut(), 0,
    1000000);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, parent);
    pdata.hw = parent;
    ret = clk_hw_register_gate_parent_data(core::ptr::null_mut(), "test_gate", &pdata, 0,
    core::ptr::null_mut(), 0, 0, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ret);
    KUNIT_EXPECT_PTR_EQ(test, parent, clk_hw_get_parent(ret));
    clk_hw_unregister_gate(ret);
    clk_hw_unregister_fixed_rate(parent);
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_register_test_parent_data_legacy(test: *mut kunit) {
    static void clk_gate_register_test_parent_data_legacy(struct kunit *test)
    {
    struct clk_hw *parent;
    struct clk_hw *ret;
    let mut pdata: clk_parent_data = { };
    parent = clk_hw_register_fixed_rate(core::ptr::null_mut(), "test_parent", core::ptr::null_mut(), 0,
    1000000);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, parent);
    pdata.name = "test_parent";
    ret = clk_hw_register_gate_parent_data(core::ptr::null_mut(), "test_gate", &pdata, 0,
    core::ptr::null_mut(), 0, 0, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ret);
    KUNIT_EXPECT_PTR_EQ(test, parent, clk_hw_get_parent(ret));
    clk_hw_unregister_gate(ret);
    clk_hw_unregister_fixed_rate(parent);
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_register_test_parent_hw(test: *mut kunit) {
    static void clk_gate_register_test_parent_hw(struct kunit *test)
    {
    struct clk_hw *parent;
    struct clk_hw *ret;
    parent = clk_hw_register_fixed_rate(core::ptr::null_mut(), "test_parent", core::ptr::null_mut(), 0,
    1000000);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, parent);
    ret = clk_hw_register_gate_parent_hw(core::ptr::null_mut(), "test_gate", parent, 0, core::ptr::null_mut(),
    0, 0, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ret);
    KUNIT_EXPECT_PTR_EQ(test, parent, clk_hw_get_parent(ret));
    clk_hw_unregister_gate(ret);
    clk_hw_unregister_fixed_rate(parent);
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_register_test_hiword_invalid(test: *mut kunit) {
    static void clk_gate_register_test_hiword_invalid(struct kunit *test)
    {
    struct clk_hw *ret;
    ret = clk_hw_register_gate(core::ptr::null_mut(), "test_gate", core::ptr::null_mut(), 0, core::ptr::null_mut(),
    20, CLK_GATE_HIWORD_MASK, core::ptr::null_mut());
    KUNIT_EXPECT_TRUE(test, IS_ERR(ret));
    }
    static struct kunit_case clk_gate_register_test_cases[] = {
    KUNIT_CASE(clk_gate_register_test_dev),
    KUNIT_CASE(clk_gate_register_test_parent_names),
    KUNIT_CASE(clk_gate_register_test_parent_data),
    KUNIT_CASE(clk_gate_register_test_parent_data_legacy),
    KUNIT_CASE(clk_gate_register_test_parent_hw),
    KUNIT_CASE(clk_gate_register_test_hiword_invalid),
    {}
    };
    static struct kunit_suite clk_gate_register_test_suite = {
    .name = "clk-gate-register-test",
    .test_cases = clk_gate_register_test_cases,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_gate_test_context {
    pub fake_mem: *mut void __iomem,
    pub hw: *mut clk_hw,
    pub parent: *mut clk_hw,
    pub /: *mut *mut __le32 fake_reg; / Keep at end, KASAN can detect out of bounds,
}

    static struct clk_gate_test_context *clk_gate_test_alloc_ctx(struct kunit *test)
    {
    struct clk_gate_test_context *ctx;
    test.priv = ctx = kunit_kzalloc(test, sizeof(*ctx), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx);
    ctx.fake_mem = (void  __iomem *)&ctx.fake_reg;
    return ctx;
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_test_parent_rate(test: *mut kunit) {
    static void clk_gate_test_parent_rate(struct kunit *test)
    {
    struct clk_gate_test_context *ctx = test.priv;
    struct clk_hw *parent = ctx.parent;
    struct clk_hw *hw = ctx.hw;
    let mut prate: c_ulong = clk_hw_get_rate(parent);
    let mut rate: c_ulong = clk_hw_get_rate(hw);
    KUNIT_EXPECT_EQ(test, prate, rate);
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_test_enable(test: *mut kunit) {
    static void clk_gate_test_enable(struct kunit *test)
    {
    struct clk_gate_test_context *ctx = test.priv;
    struct clk_hw *parent = ctx.parent;
    struct clk_hw *hw = ctx.hw;
    struct clk *clk = hw.clk;
    let mut enable_val: u32 = BIT(5);
    KUNIT_ASSERT_EQ(test, clk_prepare_enable(clk), 0);
    KUNIT_EXPECT_EQ(test, enable_val, le32_to_cpu(ctx.fake_reg));
    KUNIT_EXPECT_TRUE(test, clk_hw_is_enabled(hw));
    KUNIT_EXPECT_TRUE(test, clk_hw_is_prepared(hw));
    KUNIT_EXPECT_TRUE(test, clk_hw_is_enabled(parent));
    KUNIT_EXPECT_TRUE(test, clk_hw_is_prepared(parent));
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_test_disable(test: *mut kunit) {
    static void clk_gate_test_disable(struct kunit *test)
    {
    struct clk_gate_test_context *ctx = test.priv;
    struct clk_hw *parent = ctx.parent;
    struct clk_hw *hw = ctx.hw;
    struct clk *clk = hw.clk;
    let mut enable_val: u32 = BIT(5);
    let mut disable_val: u32 = 0;
    KUNIT_ASSERT_EQ(test, clk_prepare_enable(clk), 0);
    KUNIT_ASSERT_EQ(test, enable_val, le32_to_cpu(ctx.fake_reg));
    clk_disable_unprepare(clk);
    KUNIT_EXPECT_EQ(test, disable_val, le32_to_cpu(ctx.fake_reg));
    KUNIT_EXPECT_FALSE(test, clk_hw_is_enabled(hw));
    KUNIT_EXPECT_FALSE(test, clk_hw_is_prepared(hw));
    KUNIT_EXPECT_FALSE(test, clk_hw_is_enabled(parent));
    KUNIT_EXPECT_FALSE(test, clk_hw_is_prepared(parent));
    }
    static struct kunit_case clk_gate_test_cases[] = {
    KUNIT_CASE(clk_gate_test_parent_rate),
    KUNIT_CASE(clk_gate_test_enable),
    KUNIT_CASE(clk_gate_test_disable),
    {}
    };
#[no_mangle]
unsafe extern "C" fn clk_gate_test_init(test: *mut kunit) -> c_int {
    static int clk_gate_test_init(struct kunit *test)
    {
    struct clk_hw *parent;
    struct clk_hw *hw;
    struct clk_gate_test_context *ctx;
    ctx = clk_gate_test_alloc_ctx(test);
    parent = clk_hw_register_fixed_rate(core::ptr::null_mut(), "test_parent", core::ptr::null_mut(), 0,
    2000000);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, parent);
    hw = clk_hw_register_gate_parent_hw(core::ptr::null_mut(), "test_gate", parent, 0,
    ctx.fake_mem, 5, 0, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, hw);
    ctx.hw = hw;
    ctx.parent = parent;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_test_exit(test: *mut kunit) {
    static void clk_gate_test_exit(struct kunit *test)
    {
    struct clk_gate_test_context *ctx = test.priv;
    clk_hw_unregister_gate(ctx.hw);
    clk_hw_unregister_fixed_rate(ctx.parent);
    }
    static struct kunit_suite clk_gate_test_suite = {
    .name = "clk-gate-test",
    .init = clk_gate_test_init,
    .exit = clk_gate_test_exit,
    .test_cases = clk_gate_test_cases,
    };
#[no_mangle]
unsafe extern "C" fn clk_gate_test_invert_enable(test: *mut kunit) {
    static void clk_gate_test_invert_enable(struct kunit *test)
    {
    struct clk_gate_test_context *ctx = test.priv;
    struct clk_hw *parent = ctx.parent;
    struct clk_hw *hw = ctx.hw;
    struct clk *clk = hw.clk;
    let mut enable_val: u32 = 0;
    KUNIT_ASSERT_EQ(test, clk_prepare_enable(clk), 0);
    KUNIT_EXPECT_EQ(test, enable_val, le32_to_cpu(ctx.fake_reg));
    KUNIT_EXPECT_TRUE(test, clk_hw_is_enabled(hw));
    KUNIT_EXPECT_TRUE(test, clk_hw_is_prepared(hw));
    KUNIT_EXPECT_TRUE(test, clk_hw_is_enabled(parent));
    KUNIT_EXPECT_TRUE(test, clk_hw_is_prepared(parent));
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_test_invert_disable(test: *mut kunit) {
    static void clk_gate_test_invert_disable(struct kunit *test)
    {
    struct clk_gate_test_context *ctx = test.priv;
    struct clk_hw *parent = ctx.parent;
    struct clk_hw *hw = ctx.hw;
    struct clk *clk = hw.clk;
    let mut enable_val: u32 = 0;
    let mut disable_val: u32 = BIT(15);
    KUNIT_ASSERT_EQ(test, clk_prepare_enable(clk), 0);
    KUNIT_ASSERT_EQ(test, enable_val, le32_to_cpu(ctx.fake_reg));
    clk_disable_unprepare(clk);
    KUNIT_EXPECT_EQ(test, disable_val, le32_to_cpu(ctx.fake_reg));
    KUNIT_EXPECT_FALSE(test, clk_hw_is_enabled(hw));
    KUNIT_EXPECT_FALSE(test, clk_hw_is_prepared(hw));
    KUNIT_EXPECT_FALSE(test, clk_hw_is_enabled(parent));
    KUNIT_EXPECT_FALSE(test, clk_hw_is_prepared(parent));
    }
    static struct kunit_case clk_gate_test_invert_cases[] = {
    KUNIT_CASE(clk_gate_test_invert_enable),
    KUNIT_CASE(clk_gate_test_invert_disable),
    {}
    };
#[no_mangle]
unsafe extern "C" fn clk_gate_test_invert_init(test: *mut kunit) -> c_int {
    static int clk_gate_test_invert_init(struct kunit *test)
    {
    struct clk_hw *parent;
    struct clk_hw *hw;
    struct clk_gate_test_context *ctx;
    ctx = clk_gate_test_alloc_ctx(test);
    parent = clk_hw_register_fixed_rate(core::ptr::null_mut(), "test_parent", core::ptr::null_mut(), 0,
    2000000);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, parent);
    ctx.fake_reg = cpu_to_le32(BIT(15)); /* Default to off */
    hw = clk_hw_register_gate_parent_hw(core::ptr::null_mut(), "test_gate", parent, 0,
    ctx.fake_mem, 15,
    CLK_GATE_SET_TO_DISABLE, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, hw);
    ctx.hw = hw;
    ctx.parent = parent;
    return 0;
    }
    static struct kunit_suite clk_gate_test_invert_suite = {
    .name = "clk-gate-invert-test",
    .init = clk_gate_test_invert_init,
    .exit = clk_gate_test_exit,
    .test_cases = clk_gate_test_invert_cases,
    };
#[no_mangle]
unsafe extern "C" fn clk_gate_test_hiword_enable(test: *mut kunit) {
    static void clk_gate_test_hiword_enable(struct kunit *test)
    {
    struct clk_gate_test_context *ctx = test.priv;
    struct clk_hw *parent = ctx.parent;
    struct clk_hw *hw = ctx.hw;
    struct clk *clk = hw.clk;
    let mut enable_val: u32 = BIT(9) | BIT(9 + 16);
    KUNIT_ASSERT_EQ(test, clk_prepare_enable(clk), 0);
    KUNIT_EXPECT_EQ(test, enable_val, le32_to_cpu(ctx.fake_reg));
    KUNIT_EXPECT_TRUE(test, clk_hw_is_enabled(hw));
    KUNIT_EXPECT_TRUE(test, clk_hw_is_prepared(hw));
    KUNIT_EXPECT_TRUE(test, clk_hw_is_enabled(parent));
    KUNIT_EXPECT_TRUE(test, clk_hw_is_prepared(parent));
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_test_hiword_disable(test: *mut kunit) {
    static void clk_gate_test_hiword_disable(struct kunit *test)
    {
    struct clk_gate_test_context *ctx = test.priv;
    struct clk_hw *parent = ctx.parent;
    struct clk_hw *hw = ctx.hw;
    struct clk *clk = hw.clk;
    let mut enable_val: u32 = BIT(9) | BIT(9 + 16);
    let mut disable_val: u32 = BIT(9 + 16);
    KUNIT_ASSERT_EQ(test, clk_prepare_enable(clk), 0);
    KUNIT_ASSERT_EQ(test, enable_val, le32_to_cpu(ctx.fake_reg));
    clk_disable_unprepare(clk);
    KUNIT_EXPECT_EQ(test, disable_val, le32_to_cpu(ctx.fake_reg));
    KUNIT_EXPECT_FALSE(test, clk_hw_is_enabled(hw));
    KUNIT_EXPECT_FALSE(test, clk_hw_is_prepared(hw));
    KUNIT_EXPECT_FALSE(test, clk_hw_is_enabled(parent));
    KUNIT_EXPECT_FALSE(test, clk_hw_is_prepared(parent));
    }
    static struct kunit_case clk_gate_test_hiword_cases[] = {
    KUNIT_CASE(clk_gate_test_hiword_enable),
    KUNIT_CASE(clk_gate_test_hiword_disable),
    {}
    };
#[no_mangle]
unsafe extern "C" fn clk_gate_test_hiword_init(test: *mut kunit) -> c_int {
    static int clk_gate_test_hiword_init(struct kunit *test)
    {
    struct clk_hw *parent;
    struct clk_hw *hw;
    struct clk_gate_test_context *ctx;
    ctx = clk_gate_test_alloc_ctx(test);
    parent = clk_hw_register_fixed_rate(core::ptr::null_mut(), "test_parent", core::ptr::null_mut(), 0,
    2000000);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, parent);
    hw = clk_hw_register_gate_parent_hw(core::ptr::null_mut(), "test_gate", parent, 0,
    ctx.fake_mem, 9,
    CLK_GATE_HIWORD_MASK, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, hw);
    ctx.hw = hw;
    ctx.parent = parent;
    return 0;
    }
    static struct kunit_suite clk_gate_test_hiword_suite = {
    .name = "clk-gate-hiword-test",
    .init = clk_gate_test_hiword_init,
    .exit = clk_gate_test_exit,
    .test_cases = clk_gate_test_hiword_cases,
    };
#[no_mangle]
unsafe extern "C" fn clk_gate_test_is_enabled(test: *mut kunit) {
    static void clk_gate_test_is_enabled(struct kunit *test)
    {
    struct clk_hw *hw;
    struct clk_gate_test_context *ctx;
    ctx = clk_gate_test_alloc_ctx(test);
    ctx.fake_reg = cpu_to_le32(BIT(7));
    hw = clk_hw_register_gate(core::ptr::null_mut(), "test_gate", core::ptr::null_mut(), 0, ctx.fake_mem, 7,
    0, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, hw);
    KUNIT_ASSERT_TRUE(test, clk_hw_is_enabled(hw));
    clk_hw_unregister_gate(hw);
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_test_is_disabled(test: *mut kunit) {
    static void clk_gate_test_is_disabled(struct kunit *test)
    {
    struct clk_hw *hw;
    struct clk_gate_test_context *ctx;
    ctx = clk_gate_test_alloc_ctx(test);
    ctx.fake_reg = cpu_to_le32(BIT(4));
    hw = clk_hw_register_gate(core::ptr::null_mut(), "test_gate", core::ptr::null_mut(), 0, ctx.fake_mem, 7,
    0, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, hw);
    KUNIT_ASSERT_FALSE(test, clk_hw_is_enabled(hw));
    clk_hw_unregister_gate(hw);
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_test_is_enabled_inverted(test: *mut kunit) {
    static void clk_gate_test_is_enabled_inverted(struct kunit *test)
    {
    struct clk_hw *hw;
    struct clk_gate_test_context *ctx;
    ctx = clk_gate_test_alloc_ctx(test);
    ctx.fake_reg = cpu_to_le32(BIT(31));
    hw = clk_hw_register_gate(core::ptr::null_mut(), "test_gate", core::ptr::null_mut(), 0, ctx.fake_mem, 2,
    CLK_GATE_SET_TO_DISABLE, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, hw);
    KUNIT_ASSERT_TRUE(test, clk_hw_is_enabled(hw));
    clk_hw_unregister_gate(hw);
    }
#[no_mangle]
unsafe extern "C" fn clk_gate_test_is_disabled_inverted(test: *mut kunit) {
    static void clk_gate_test_is_disabled_inverted(struct kunit *test)
    {
    struct clk_hw *hw;
    struct clk_gate_test_context *ctx;
    ctx = clk_gate_test_alloc_ctx(test);
    ctx.fake_reg = cpu_to_le32(BIT(29));
    hw = clk_hw_register_gate(core::ptr::null_mut(), "test_gate", core::ptr::null_mut(), 0, ctx.fake_mem, 29,
    CLK_GATE_SET_TO_DISABLE, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, hw);
    KUNIT_ASSERT_FALSE(test, clk_hw_is_enabled(hw));
    clk_hw_unregister_gate(hw);
    }
    static struct kunit_case clk_gate_test_enabled_cases[] = {
    KUNIT_CASE(clk_gate_test_is_enabled),
    KUNIT_CASE(clk_gate_test_is_disabled),
    KUNIT_CASE(clk_gate_test_is_enabled_inverted),
    KUNIT_CASE(clk_gate_test_is_disabled_inverted),
    {}
    };
    static struct kunit_suite clk_gate_test_enabled_suite = {
    .name = "clk-gate-is_enabled-test",
    .test_cases = clk_gate_test_enabled_cases,
    };
    kunit_test_suites(
    &clk_gate_register_test_suite,
    &clk_gate_test_suite,
    &clk_gate_test_invert_suite,
    &clk_gate_test_hiword_suite,
    &clk_gate_test_enabled_suite
    );
    MODULE_DESCRIPTION("Kunit tests for clk gate");
    MODULE_LICENSE("GPL v2");

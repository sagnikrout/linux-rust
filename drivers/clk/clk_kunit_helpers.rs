//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk_kunit_helpers.c
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
// KUnit helpers for clk providers and consumers
//

    KUNIT_DEFINE_ACTION_WRAPPER(clk_disable_unprepare_wrapper,
    clk_disable_unprepare, struct clk *);
//
// clk_prepare_enable_kunit() - Test managed clk_prepare_enable()
// @test: The test context
// @clk: clk to prepare and enable
//
// Return: 0 on success, or negative errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn clk_prepare_enable_kunit(test: *mut kunit, clk: *mut clk) -> c_int {
    int clk_prepare_enable_kunit(struct kunit *test, struct clk *clk)
    {
    int ret;
    ret = clk_prepare_enable(clk);
    if (ret)
    return ret;
    return kunit_add_action_or_reset(test, clk_disable_unprepare_wrapper,
    clk);
    }
    EXPORT_SYMBOL_GPL(clk_prepare_enable_kunit);
    KUNIT_DEFINE_ACTION_WRAPPER(clk_put_wrapper, clk_put, struct clk *);
    static struct clk *__clk_get_kunit(struct kunit *test, struct clk *clk)
    {
    int ret;
    if (IS_ERR(clk))
    return clk;
    ret = kunit_add_action_or_reset(test, clk_put_wrapper, clk);
    if (ret)
    return ERR_PTR(ret);
    return clk;
    }
//
// clk_get_kunit() - Test managed clk_get()
// @test: The test context
// @dev: device for clock "consumer"
// @con_id: clock consumer ID
//
// Just like clk_get(), except the clk is managed by the test case and is
// automatically put with clk_put() after the test case concludes.
//
// Return: new clk consumer or ERR_PTR on failure.
//
    struct clk *
    clk_get_kunit(struct kunit *test, struct device *dev, const char *con_id)
    {
    struct clk *clk;
    clk = clk_get(dev, con_id);
    return __clk_get_kunit(test, clk);
    }
    EXPORT_SYMBOL_GPL(clk_get_kunit);
//
// of_clk_get_kunit() - Test managed of_clk_get()
// @test: The test context
// @np: device_node for clock "consumer"
// @index: index in 'clocks' property of @np
//
// Just like of_clk_get(), except the clk is managed by the test case and is
// automatically put with clk_put() after the test case concludes.
//
// Return: new clk consumer or ERR_PTR on failure.
//
    struct clk *
    of_clk_get_kunit(struct kunit *test, struct device_node *np, int index)
    {
    struct clk *clk;
    clk = of_clk_get(np, index);
    return __clk_get_kunit(test, clk);
    }
    EXPORT_SYMBOL_GPL(of_clk_get_kunit);
//
// clk_hw_get_clk_kunit() - Test managed clk_hw_get_clk()
// @test: The test context
// @hw: clk_hw associated with the clk being consumed
// @con_id: connection ID string on device
//
// Just like clk_hw_get_clk(), except the clk is managed by the test case and
// is automatically put with clk_put() after the test case concludes.
//
// Return: new clk consumer or ERR_PTR on failure.
//
    struct clk *
    clk_hw_get_clk_kunit(struct kunit *test, struct clk_hw *hw, const char *con_id)
    {
    struct clk *clk;
    clk = clk_hw_get_clk(hw, con_id);
    return __clk_get_kunit(test, clk);
    }
    EXPORT_SYMBOL_GPL(clk_hw_get_clk_kunit);
//
// clk_hw_get_clk_prepared_enabled_kunit() - Test managed clk_hw_get_clk() + clk_prepare_enable()
// @test: The test context
// @hw: clk_hw associated with the clk being consumed
// @con_id: connection ID string on device
//
// Just like
//
// .. code-block:: c
//
// struct clk *clk = clk_hw_get_clk(...);
// clk_prepare_enable(clk);
//
// except the clk is managed by the test case and is automatically disabled and
// unprepared with clk_disable_unprepare() and put with clk_put() after the
// test case concludes.
//
// Return: new clk consumer that is prepared and enabled or ERR_PTR on failure.
//
    struct clk *
    clk_hw_get_clk_prepared_enabled_kunit(struct kunit *test, struct clk_hw *hw,
    const char *con_id)
    {
    int ret;
    struct clk *clk;
    clk = clk_hw_get_clk_kunit(test, hw, con_id);
    if (IS_ERR(clk))
    return clk;
    ret = clk_prepare_enable_kunit(test, clk);
    if (ret)
    return ERR_PTR(ret);
    return clk;
    }
    EXPORT_SYMBOL_GPL(clk_hw_get_clk_prepared_enabled_kunit);
    KUNIT_DEFINE_ACTION_WRAPPER(clk_hw_unregister_wrapper,
    clk_hw_unregister, struct clk_hw *);
//
// clk_hw_register_kunit() - Test managed clk_hw_register()
// @test: The test context
// @dev: device that is registering this clock
// @hw: link to hardware-specific clock data
//
// Just like clk_hw_register(), except the clk registration is managed by the
// test case and is automatically unregistered after the test case concludes.
//
// Return: 0 on success or a negative errno value on failure.
//
#[no_mangle]
pub unsafe extern "C" fn clk_hw_register_kunit(test: *mut kunit, dev: *mut device, hw: *mut clk_hw) -> c_int {
    int clk_hw_register_kunit(struct kunit *test, struct device *dev, struct clk_hw *hw)
    {
    int ret;
    ret = clk_hw_register(dev, hw);
    if (ret)
    return ret;
    return kunit_add_action_or_reset(test, clk_hw_unregister_wrapper, hw);
    }
    EXPORT_SYMBOL_GPL(clk_hw_register_kunit);
//
// of_clk_hw_register_kunit() - Test managed of_clk_hw_register()
// @test: The test context
// @node: device_node of device that is registering this clock
// @hw: link to hardware-specific clock data
//
// Just like of_clk_hw_register(), except the clk registration is managed by
// the test case and is automatically unregistered after the test case
// concludes.
//
// Return: 0 on success or a negative errno value on failure.
//
#[no_mangle]
pub unsafe extern "C" fn of_clk_hw_register_kunit(test: *mut kunit, node: *mut device_node, hw: *mut clk_hw) -> c_int {
    int of_clk_hw_register_kunit(struct kunit *test, struct device_node *node, struct clk_hw *hw)
    {
    int ret;
    ret = of_clk_hw_register(node, hw);
    if (ret)
    return ret;
    return kunit_add_action_or_reset(test, clk_hw_unregister_wrapper, hw);
    }
    EXPORT_SYMBOL_GPL(of_clk_hw_register_kunit);
    KUNIT_DEFINE_ACTION_WRAPPER(of_clk_del_provider_wrapper,
    of_clk_del_provider, struct device_node *);
//
// of_clk_add_hw_provider_kunit() - Test managed of_clk_add_hw_provider()
// @test: The test context
// @np: Device node pointer associated with clock provider
// @get: Callback for decoding clk_hw
// @data: Context pointer for @get callback.
//
// Just like of_clk_add_hw_provider(), except the clk_hw provider is managed by
// the test case and is automatically unregistered after the test case
// concludes.
//
// Return: 0 on success or a negative errno value on failure.
//
    int of_clk_add_hw_provider_kunit(struct kunit *test, struct device_node *np,
    struct clk_hw *(*get)(struct of_phandle_args *clkspec, void *data),
    void *data)
    {
    int ret;
    ret = of_clk_add_hw_provider(np, get, data);
    if (ret)
    return ret;
    return kunit_add_action_or_reset(test, of_clk_del_provider_wrapper, np);
    }
    EXPORT_SYMBOL_GPL(of_clk_add_hw_provider_kunit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("KUnit helpers for clk providers and consumers");

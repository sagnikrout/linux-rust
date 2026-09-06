//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/qcom/tlmm-test.c
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
// Copyright (c) 2025, Qualcomm Innovation Center, Inc. All rights reserved.
//

//
// This TLMM test module serves the purpose of validating that the TLMM driver
// (pinctrl-msm) delivers expected number of interrupts in response to changing
// GPIO state.
//
// To achieve this without external equipment the test takes a module parameter
// "gpio", which the tester is expected to specify an unused and non-connected
// pin. The GPIO state is then driven by adjusting the bias of the pin, at
// suitable times through the different test cases.
//
// Upon execution, the test initialization will find the TLMM node (subject to
// tlmm_of_match[] allow listing) and create the necessary references
// dynamically, rather then relying on e.g. Devicetree and phandles.
//

pub const MSM_PULL_DOWN: c_int = 1;
pub const MSM_PULL_UP: c_int = 3;
pub const TLMM_REG_SIZE: c_uint = 0x1000;
    let mut tlmm_test_gpio: static int = -1;
    static char *tlmm_reg_name = "default_region";
    module_param_named(gpio, tlmm_test_gpio, int, 0600);
    module_param_named(name, tlmm_reg_name, charp, 0600);
    static struct {
    void __iomem *base;
    void __iomem *reg;
    int irq;
    u32 low_val;
    u32 high_val;
    } tlmm_suite;
//
// struct tlmm_test_priv - Per-test context
// @intr_count:		number of times hard handler was hit with TLMM_TEST_COUNT op set
// @thread_count:	number of times thread handler was hit with TLMM_TEST_COUNT op set
// @intr_op:		operations to be performed by the hard IRQ handler
// @intr_op_remain:	number of times the TLMM_TEST_THEN_* operations should be
// performed by the hard IRQ handler
// @thread_op:		operations to be performed by the threaded IRQ handler
// @thread_op_remain:	number of times the TLMM_TEST_THEN_* operations should
// be performed by the threaded IRQ handler
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlmm_test_priv {
    pub intr_count: core::sync::atomic::AtomicI32,
    pub thread_count: core::sync::atomic::AtomicI32,
    pub intr_op: c_uint,
    pub intr_op_remain: core::sync::atomic::AtomicI32,
    pub thread_op: c_uint,
    pub thread_op_remain: core::sync::atomic::AtomicI32,
}

// Operation masks for @intr_op and @thread_op

#[no_mangle]
unsafe extern "C" fn tlmm_output_low() {
    static void tlmm_output_low(void)
    {
    writel(tlmm_suite.low_val, tlmm_suite.reg);
    readl(tlmm_suite.reg);
    }
#[no_mangle]
unsafe extern "C" fn tlmm_output_high() {
    static void tlmm_output_high(void)
    {
    writel(tlmm_suite.high_val, tlmm_suite.reg);
    readl(tlmm_suite.reg);
    }
#[no_mangle]
unsafe extern "C" fn tlmm_test_intr_fn(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t tlmm_test_intr_fn(int irq, void *dev_id)
    {
    struct tlmm_test_priv *priv = dev_id;
    if (priv.intr_op & TLMM_TEST_COUNT)
    atomic_inc(&priv.intr_count);
    if (priv.intr_op & TLMM_TEST_OUTPUT_LOW)
    tlmm_output_low();
    if (priv.intr_op & TLMM_TEST_OUTPUT_HIGH)
    tlmm_output_high();
    if (atomic_dec_if_positive(&priv.intr_op_remain) > 0) {
    udelay(1);
    if (priv.intr_op & TLMM_TEST_THEN_LOW)
    tlmm_output_low();
    if (priv.intr_op & TLMM_TEST_THEN_HIGH)
    tlmm_output_high();
    }
    return priv.intr_op & TLMM_TEST_WAKE_THREAD ? IRQ_WAKE_THREAD : IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tlmm_test_intr_thread_fn(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t tlmm_test_intr_thread_fn(int irq, void *dev_id)
    {
    struct tlmm_test_priv *priv = dev_id;
    if (priv.thread_op & TLMM_TEST_COUNT)
    atomic_inc(&priv.thread_count);
    if (priv.thread_op & TLMM_TEST_OUTPUT_LOW)
    tlmm_output_low();
    if (priv.thread_op & TLMM_TEST_OUTPUT_HIGH)
    tlmm_output_high();
    if (atomic_dec_if_positive(&priv.thread_op_remain) > 0) {
    udelay(1);
    if (priv.thread_op & TLMM_TEST_THEN_LOW)
    tlmm_output_low();
    if (priv.thread_op & TLMM_TEST_THEN_HIGH)
    tlmm_output_high();
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tlmm_test_request_hard_irq(test: *mut kunit, irqflags: c_ulong) {
    static void tlmm_test_request_hard_irq(struct kunit *test, unsigned long irqflags)
    {
    struct tlmm_test_priv *priv = test.priv;
    int ret;
    ret = request_irq(tlmm_suite.irq, tlmm_test_intr_fn, irqflags, test.name, priv);
    KUNIT_EXPECT_EQ(test, ret, 0);
    }
#[no_mangle]
unsafe extern "C" fn tlmm_test_request_threaded_irq(test: *mut kunit, irqflags: c_ulong) {
    static void tlmm_test_request_threaded_irq(struct kunit *test, unsigned long irqflags)
    {
    struct tlmm_test_priv *priv = test.priv;
    int ret;
    ret = request_threaded_irq(tlmm_suite.irq,
    tlmm_test_intr_fn, tlmm_test_intr_thread_fn,
    irqflags, test.name, priv);
    KUNIT_EXPECT_EQ(test, ret, 0);
    }
#[no_mangle]
unsafe extern "C" fn tlmm_test_silent(test: *mut kunit, irqflags: c_ulong) {
    static void tlmm_test_silent(struct kunit *test, unsigned long irqflags)
    {
    struct tlmm_test_priv *priv = test.priv;
    priv.intr_op = TLMM_TEST_COUNT;
// GPIO line at non-triggering level
    if (irqflags == IRQF_TRIGGER_LOW || irqflags == IRQF_TRIGGER_FALLING)
    tlmm_output_high();
    else
    tlmm_output_low();
    tlmm_test_request_hard_irq(test, irqflags);
    msleep(100);
    free_irq(tlmm_suite.irq, priv);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.intr_count), 0);
    }
//
// Test that no RISING interrupts are triggered on a silent pin
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_silent_rising(test: *mut kunit) {
    static void tlmm_test_silent_rising(struct kunit *test)
    {
    tlmm_test_silent(test, IRQF_TRIGGER_RISING);
    }
//
// Test that no FALLING interrupts are triggered on a silent pin
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_silent_falling(test: *mut kunit) {
    static void tlmm_test_silent_falling(struct kunit *test)
    {
    tlmm_test_silent(test, IRQF_TRIGGER_FALLING);
    }
//
// Test that no LOW interrupts are triggered on a silent pin
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_silent_low(test: *mut kunit) {
    static void tlmm_test_silent_low(struct kunit *test)
    {
    tlmm_test_silent(test, IRQF_TRIGGER_LOW);
    }
//
// Test that no HIGH interrupts are triggered on a silent pin
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_silent_high(test: *mut kunit) {
    static void tlmm_test_silent_high(struct kunit *test)
    {
    tlmm_test_silent(test, IRQF_TRIGGER_HIGH);
    }
//
// Square wave with 10 high pulses, assert that we get 10 rising interrupts
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_rising(test: *mut kunit) {
    static void tlmm_test_rising(struct kunit *test)
    {
    struct tlmm_test_priv *priv = test.priv;
    int i;
    priv.intr_op = TLMM_TEST_COUNT;
    tlmm_output_low();
    tlmm_test_request_hard_irq(test, IRQF_TRIGGER_RISING);
    for (i = 0; i < 10; i++) {
    tlmm_output_low();
    msleep(20);
    tlmm_output_high();
    msleep(20);
    }
    free_irq(tlmm_suite.irq, priv);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.intr_count), 10);
    }
//
// Square wave with 10 low pulses, assert that we get 10 falling interrupts
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_falling(test: *mut kunit) {
    static void tlmm_test_falling(struct kunit *test)
    {
    struct tlmm_test_priv *priv = test.priv;
    int i;
    priv.intr_op = TLMM_TEST_COUNT;
    tlmm_output_high();
    tlmm_test_request_hard_irq(test, IRQF_TRIGGER_FALLING);
    for (i = 0; i < 10; i++) {
    tlmm_output_high();
    msleep(20);
    tlmm_output_low();
    msleep(20);
    }
    free_irq(tlmm_suite.irq, priv);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.intr_count), 10);
    }
//
// Drive line low 10 times, handler drives it high to "clear the interrupt
// source", assert we get 10 interrupts
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_low(test: *mut kunit) {
    static void tlmm_test_low(struct kunit *test)
    {
    struct tlmm_test_priv *priv = test.priv;
    int i;
    priv.intr_op = TLMM_TEST_COUNT | TLMM_TEST_OUTPUT_HIGH;
    tlmm_output_high();
    tlmm_test_request_hard_irq(test, IRQF_TRIGGER_LOW);
    for (i = 0; i < 10; i++) {
    msleep(20);
    tlmm_output_low();
    }
    msleep(100);
    free_irq(tlmm_suite.irq, priv);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.intr_count), 10);
    }
//
// Drive line high 10 times, handler drives it low to "clear the interrupt
// source", assert we get 10 interrupts
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_high(test: *mut kunit) {
    static void tlmm_test_high(struct kunit *test)
    {
    struct tlmm_test_priv *priv = test.priv;
    int i;
    priv.intr_op = TLMM_TEST_COUNT | TLMM_TEST_OUTPUT_LOW;
    tlmm_output_low();
    tlmm_test_request_hard_irq(test, IRQF_TRIGGER_HIGH);
    for (i = 0; i < 10; i++) {
    msleep(20);
    tlmm_output_high();
    }
    msleep(100);
    free_irq(tlmm_suite.irq, priv);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.intr_count), 10);
    }
//
// Handler drives GPIO high to "clear the interrupt source", then low to
// simulate a new interrupt, repeated 10 times, assert we get 10 interrupts
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_falling_in_handler(test: *mut kunit) {
    static void tlmm_test_falling_in_handler(struct kunit *test)
    {
    struct tlmm_test_priv *priv = test.priv;
    priv.intr_op = TLMM_TEST_COUNT | TLMM_TEST_OUTPUT_HIGH | TLMM_TEST_THEN_LOW;
    atomic_set(&priv.intr_op_remain, 10);
    tlmm_output_high();
    tlmm_test_request_hard_irq(test, IRQF_TRIGGER_FALLING);
    msleep(20);
    tlmm_output_low();
    msleep(100);
    free_irq(tlmm_suite.irq, priv);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.intr_count), 10);
    }
//
// Handler drives GPIO low to "clear the interrupt source", then high to
// simulate a new interrupt, repeated 10 times, assert we get 10 interrupts
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_rising_in_handler(test: *mut kunit) {
    static void tlmm_test_rising_in_handler(struct kunit *test)
    {
    struct tlmm_test_priv *priv = test.priv;
    priv.intr_op = TLMM_TEST_COUNT | TLMM_TEST_OUTPUT_LOW | TLMM_TEST_THEN_HIGH;
    atomic_set(&priv.intr_op_remain, 10);
    tlmm_output_low();
    tlmm_test_request_hard_irq(test, IRQF_TRIGGER_RISING);
    msleep(20);
    tlmm_output_high();
    msleep(100);
    free_irq(tlmm_suite.irq, priv);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.intr_count), 10);
    }
//
// Square wave with 10 high pulses, assert that we get 10 rising hard and
// 10 threaded interrupts
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_thread_rising(test: *mut kunit) {
    static void tlmm_test_thread_rising(struct kunit *test)
    {
    struct tlmm_test_priv *priv = test.priv;
    int i;
    priv.intr_op = TLMM_TEST_COUNT | TLMM_TEST_WAKE_THREAD;
    priv.thread_op = TLMM_TEST_COUNT;
    tlmm_output_low();
    tlmm_test_request_threaded_irq(test, IRQF_TRIGGER_RISING);
    for (i = 0; i < 10; i++) {
    tlmm_output_low();
    msleep(20);
    tlmm_output_high();
    msleep(20);
    }
    free_irq(tlmm_suite.irq, priv);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.intr_count), 10);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.thread_count), 10);
    }
//
// Square wave with 10 low pulses, assert that we get 10 falling interrupts
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_thread_falling(test: *mut kunit) {
    static void tlmm_test_thread_falling(struct kunit *test)
    {
    struct tlmm_test_priv *priv = test.priv;
    int i;
    priv.intr_op = TLMM_TEST_COUNT | TLMM_TEST_WAKE_THREAD;
    priv.thread_op = TLMM_TEST_COUNT;
    tlmm_output_high();
    tlmm_test_request_threaded_irq(test, IRQF_TRIGGER_FALLING);
    for (i = 0; i < 10; i++) {
    tlmm_output_high();
    msleep(20);
    tlmm_output_low();
    msleep(20);
    }
    free_irq(tlmm_suite.irq, priv);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.intr_count), 10);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.thread_count), 10);
    }
//
// Drive line high 10 times, threaded handler drives it low to "clear the
// interrupt source", assert we get 10 interrupts
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_thread_high(test: *mut kunit) {
    static void tlmm_test_thread_high(struct kunit *test)
    {
    struct tlmm_test_priv *priv = test.priv;
    int i;
    priv.intr_op = TLMM_TEST_COUNT | TLMM_TEST_WAKE_THREAD;
    priv.thread_op = TLMM_TEST_COUNT | TLMM_TEST_OUTPUT_LOW;
    tlmm_output_low();
    tlmm_test_request_threaded_irq(test, IRQF_TRIGGER_HIGH | IRQF_ONESHOT);
    for (i = 0; i < 10; i++) {
    tlmm_output_high();
    msleep(20);
    }
    free_irq(tlmm_suite.irq, priv);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.intr_count), 10);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.thread_count), 10);
    }
//
// Drive line low 10 times, threaded handler drives it high to "clear the
// interrupt source", assert we get 10 interrupts
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_thread_low(test: *mut kunit) {
    static void tlmm_test_thread_low(struct kunit *test)
    {
    struct tlmm_test_priv *priv = test.priv;
    int i;
    priv.intr_op = TLMM_TEST_COUNT | TLMM_TEST_WAKE_THREAD;
    priv.thread_op = TLMM_TEST_COUNT | TLMM_TEST_OUTPUT_HIGH;
    tlmm_output_high();
    tlmm_test_request_threaded_irq(test, IRQF_TRIGGER_LOW | IRQF_ONESHOT);
    for (i = 0; i < 10; i++) {
    tlmm_output_low();
    msleep(20);
    }
    free_irq(tlmm_suite.irq, priv);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.intr_count), 10);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.thread_count), 10);
    }
//
// Handler drives GPIO low to "clear the interrupt source", then high in the
// threaded handler to simulate a new interrupt, repeated 10 times, assert we
// get 10 interrupts
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_thread_rising_in_handler(test: *mut kunit) {
    static void tlmm_test_thread_rising_in_handler(struct kunit *test)
    {
    struct tlmm_test_priv *priv = test.priv;
    priv.intr_op = TLMM_TEST_COUNT | TLMM_TEST_OUTPUT_LOW | TLMM_TEST_WAKE_THREAD;
    priv.thread_op = TLMM_TEST_COUNT | TLMM_TEST_THEN_HIGH;
    atomic_set(&priv.thread_op_remain, 10);
    tlmm_output_low();
    tlmm_test_request_threaded_irq(test, IRQF_TRIGGER_RISING);
    msleep(20);
    tlmm_output_high();
    msleep(100);
    free_irq(tlmm_suite.irq, priv);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.intr_count), 10);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.thread_count), 10);
    }
//
// Handler drives GPIO high to "clear the interrupt source", then low in the
// threaded handler to simulate a new interrupt, repeated 10 times, assert we
// get 10 interrupts
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_thread_falling_in_handler(test: *mut kunit) {
    static void tlmm_test_thread_falling_in_handler(struct kunit *test)
    {
    struct tlmm_test_priv *priv = test.priv;
    priv.intr_op = TLMM_TEST_COUNT | TLMM_TEST_OUTPUT_HIGH | TLMM_TEST_WAKE_THREAD;
    priv.thread_op = TLMM_TEST_COUNT | TLMM_TEST_THEN_LOW;
    atomic_set(&priv.thread_op_remain, 10);
    tlmm_output_high();
    tlmm_test_request_threaded_irq(test, IRQF_TRIGGER_FALLING);
    msleep(20);
    tlmm_output_low();
    msleep(100);
    free_irq(tlmm_suite.irq, priv);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.intr_count), 10);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.thread_count), 10);
    }
//
// Validate that edge interrupts occurring while irq is disabled is delivered
// once the interrupt is reenabled.
//
#[no_mangle]
unsafe extern "C" fn tlmm_test_rising_while_disabled(test: *mut kunit) {
    static void tlmm_test_rising_while_disabled(struct kunit *test)
    {
    struct tlmm_test_priv *priv = test.priv;
    unsigned int after_edge;
    unsigned int before_edge;
    priv.intr_op = TLMM_TEST_COUNT;
    tlmm_output_low();
    tlmm_test_request_hard_irq(test, IRQF_TRIGGER_RISING);
    msleep(20);
    disable_irq(tlmm_suite.irq);
    before_edge = atomic_read(&priv.intr_count);
    tlmm_output_high();
    msleep(20);
    after_edge = atomic_read(&priv.intr_count);
    msleep(20);
    enable_irq(tlmm_suite.irq);
    msleep(20);
    free_irq(tlmm_suite.irq, priv);
    KUNIT_ASSERT_EQ(test, before_edge, 0);
    KUNIT_ASSERT_EQ(test, after_edge, 0);
    KUNIT_ASSERT_EQ(test, atomic_read(&priv.intr_count), 1);
    }
#[no_mangle]
unsafe extern "C" fn tlmm_test_init(test: *mut kunit) -> c_int {
    static int tlmm_test_init(struct kunit *test)
    {
    struct tlmm_test_priv *priv;
    priv = kunit_kzalloc(test, sizeof(*priv), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv);
    atomic_set(&priv.intr_count, 0);
    atomic_set(&priv.thread_count, 0);
    atomic_set(&priv.intr_op_remain, 0);
    atomic_set(&priv.thread_op_remain, 0);
    test.priv = priv;
    return 0;
    }
//
// NOTE: When adding compatibles to this list, ensure that TLMM_REG_SIZE and
// pull configuration values are supported and correct.
//
    static const struct of_device_id tlmm_of_match[] = {
    { .compatible = "qcom,sc8280xp-tlmm" },
    { .compatible = "qcom,x1e80100-tlmm" },
    {}
    };
#[no_mangle]
unsafe extern "C" fn tlmm_reg_base(tlmm: *mut device_node, res: *mut resource) -> c_int {
    static int tlmm_reg_base(struct device_node *tlmm, struct resource *res)
    {
    const char **reg_names;
    int count;
    int ret;
    int i;
    if (!strcmp(tlmm_reg_name, "default_region"))
    return of_address_to_resource(tlmm, 0, res);
    count = of_property_count_strings(tlmm, "reg-names");
    if (count <= 0) {
    pr_err("failed to find tlmm reg name\n");
    return count;
    }
    reg_names = kcalloc(count, sizeof(char *), GFP_KERNEL);
    if (!reg_names)
    return -ENOMEM;
    ret = of_property_read_string_array(tlmm, "reg-names", reg_names, count);
    if (ret != count) {
    kfree(reg_names);
    return -EINVAL;
    }
    for (i = 0; i < count; i++) {
    if (!strcmp(reg_names[i], tlmm_reg_name)) {
    ret = of_address_to_resource(tlmm, i, res);
    break;
    }
    }
    if (i == count)
    ret = -EINVAL;
    kfree(reg_names);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tlmm_test_init_suite(suite: *mut kunit_suite) -> c_int {
    static int tlmm_test_init_suite(struct kunit_suite *suite)
    {
    let mut args: of_phandle_args = {};
    struct resource res;
    int ret;
    u32 val;
    if (tlmm_test_gpio < 0) {
    pr_err("use the tlmm-test.gpio module parameter to specify which GPIO to use\n");
    return -EINVAL;
    }
    struct device_node *tlmm __free(device_node) = of_find_matching_node(core::ptr::null_mut(), tlmm_of_match);
    if (!tlmm) {
    pr_err("failed to find tlmm node\n");
    return -EINVAL;
    }
    ret = tlmm_reg_base(tlmm, &res);
    if (ret < 0)
    return ret;
    tlmm_suite.base = ioremap(res.start, resource_size(&res));
    if (!tlmm_suite.base)
    return -ENOMEM;
    args.np = tlmm;
    args.args_count = 2;
    args.args[0] = tlmm_test_gpio;
    args.args[1] = 0;
    tlmm_suite.irq = irq_create_of_mapping(&args);
    if (!tlmm_suite.irq) {
    pr_err("failed to map TLMM irq %d\n", args.args[0]);
    goto err_unmap;
    }
    tlmm_suite.reg = tlmm_suite.base + tlmm_test_gpio * TLMM_REG_SIZE;
    val = readl(tlmm_suite.reg) & ~MSM_PULL_MASK;
    tlmm_suite.low_val = val | MSM_PULL_DOWN;
    tlmm_suite.high_val = val | MSM_PULL_UP;
    return 0;
    err_unmap:
    iounmap(tlmm_suite.base);
    tlmm_suite.base = core::ptr::null_mut();
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn tlmm_test_exit_suite(suite: *mut kunit_suite) {
    static void tlmm_test_exit_suite(struct kunit_suite *suite)
    {
    irq_dispose_mapping(tlmm_suite.irq);
    iounmap(tlmm_suite.base);
    tlmm_suite.base = core::ptr::null_mut();
    tlmm_suite.irq = -1;
    }
    static struct kunit_case tlmm_test_cases[] = {
    KUNIT_CASE(tlmm_test_silent_rising),
    KUNIT_CASE(tlmm_test_silent_falling),
    KUNIT_CASE(tlmm_test_silent_low),
    KUNIT_CASE(tlmm_test_silent_high),
    KUNIT_CASE(tlmm_test_rising),
    KUNIT_CASE(tlmm_test_falling),
    KUNIT_CASE(tlmm_test_high),
    KUNIT_CASE(tlmm_test_low),
    KUNIT_CASE(tlmm_test_rising_in_handler),
    KUNIT_CASE(tlmm_test_falling_in_handler),
    KUNIT_CASE(tlmm_test_thread_rising),
    KUNIT_CASE(tlmm_test_thread_falling),
    KUNIT_CASE(tlmm_test_thread_high),
    KUNIT_CASE(tlmm_test_thread_low),
    KUNIT_CASE(tlmm_test_thread_rising_in_handler),
    KUNIT_CASE(tlmm_test_thread_falling_in_handler),
    KUNIT_CASE(tlmm_test_rising_while_disabled),
    {}
    };
    static struct kunit_suite tlmm_test_suite = {
    .name = "tlmm-test",
    .init = tlmm_test_init,
    .suite_init = tlmm_test_init_suite,
    .suite_exit = tlmm_test_exit_suite,
    .test_cases = tlmm_test_cases,
    };
    kunit_test_suites(&tlmm_test_suite);
    MODULE_DESCRIPTION("Qualcomm TLMM test");
    MODULE_LICENSE("GPL");

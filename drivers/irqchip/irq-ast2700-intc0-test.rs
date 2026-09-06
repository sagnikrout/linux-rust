//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-ast2700-intc0-test.c
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
// Copyright (C) 2026 Code Construct
//

#[no_mangle]
unsafe extern "C" fn aspeed_intc0_resolve_route_bad_args(test: *mut kunit) {
    static void aspeed_intc0_resolve_route_bad_args(struct kunit *test)
    {
    static const struct aspeed_intc_interrupt_range c1ranges[] = { 0 };
    static const u32 c1outs[] = { 0 };
    struct aspeed_intc_interrupt_range resolved;
    let mut c0domain: irq_domain = { 0 };
    int rc;
    rc = aspeed_intc0_resolve_route(core::ptr::null_mut(), 0, c1outs, 0, c1ranges, core::ptr::null_mut());
    KUNIT_EXPECT_EQ(test, rc, -EINVAL);
    rc = aspeed_intc0_resolve_route(&c0domain, 0, c1outs,
    ARRAY_SIZE(c1ranges), c1ranges,
    &resolved);
    KUNIT_EXPECT_EQ(test, rc, -ENOENT);
    rc = aspeed_intc0_resolve_route(&c0domain, ARRAY_SIZE(c1outs), c1outs,
    0, c1ranges, &resolved);
    KUNIT_EXPECT_EQ(test, rc, -ENOENT);
    }
    static int gicv3_fwnode_read_string_array(const struct fwnode_handle *fwnode,
    const char *propname, const char **val, size_t nval)
    {
    if (!propname)
    return -EINVAL;
    if (!val)
    return 1;
    if (WARN_ON(nval != 1))
    return -EOVERFLOW;
// val = "arm,gic-v3";
    return 1;
    }
    static const struct fwnode_operations arm_gicv3_fwnode_ops = {
    .property_read_string_array = gicv3_fwnode_read_string_array,
    };
#[no_mangle]
unsafe extern "C" fn aspeed_intc_resolve_route_invalid_c0domain(test: *mut kunit) {
    static void aspeed_intc_resolve_route_invalid_c0domain(struct kunit *test)
    {
    struct device_node intc0_node = {
    .fwnode = { .ops = &arm_gicv3_fwnode_ops },
    };
    let mut c0domain: irq_domain = { .fwnode = &intc0_node.fwnode };
    static const struct aspeed_intc_interrupt_range c1ranges[] = { 0 };
    static const u32 c1outs[] = { 0 };
    struct aspeed_intc_interrupt_range resolved;
    int rc;
    rc = aspeed_intc0_resolve_route(&c0domain, ARRAY_SIZE(c1outs), c1outs,
    ARRAY_SIZE(c1ranges), c1ranges,
    &resolved);
    KUNIT_EXPECT_NE(test, rc, 0);
    }
    static int
    aspeed_intc0_fwnode_read_string_array(const struct fwnode_handle *fwnode_handle,
    const char *propname, const char **val,
    size_t nval)
    {
    if (!propname)
    return -EINVAL;
    if (!val)
    return 1;
    if (WARN_ON(nval != 1))
    return -EOVERFLOW;
// val = "aspeed,ast2700-intc0";
    return nval;
    }
    static const struct fwnode_operations intc0_fwnode_ops = {
    .property_read_string_array = aspeed_intc0_fwnode_read_string_array,
    };
    static void
    aspeed_intc0_resolve_route_c1i1o1c0i1o1_connected(struct kunit *test)
    {
    struct device_node intc0_node = {
    .fwnode = { .ops = &intc0_fwnode_ops },
    };
    struct aspeed_intc_interrupt_range c1ranges[] = {
    {
    .start = 0,
    .count = 1,
    .upstream = {
    .fwnode = &intc0_node.fwnode,
    .param_count = 1,
    .param = { 128 }
    }
    }
    };
    static const u32 c1outs[] = { 0 };
    struct aspeed_intc_interrupt_range resolved;
    struct aspeed_intc_interrupt_range intc0_ranges[] = {
    {
    .start = 128,
    .count = 1,
    .upstream = {
    .fwnode = core::ptr::null_mut(),
    .param_count = 0,
    .param = { 0 },
    }
    }
    };
    struct aspeed_intc0 intc0 = {
    .ranges = { .ranges = intc0_ranges, .nranges = ARRAY_SIZE(intc0_ranges), }
    };
    const struct irq_domain c0domain = {
    .host_data = &intc0,
    .fwnode = &intc0_node.fwnode
    };
    int rc;
    rc = aspeed_intc0_resolve_route(&c0domain, ARRAY_SIZE(c1outs), c1outs,
    ARRAY_SIZE(c1ranges), c1ranges,
    &resolved);
    KUNIT_EXPECT_EQ(test, rc, 0);
    KUNIT_EXPECT_EQ(test, resolved.start, 0);
    KUNIT_EXPECT_EQ(test, resolved.count, 1);
    KUNIT_EXPECT_EQ(test, resolved.upstream.param[0], 128);
    }
    static void
    aspeed_intc0_resolve_route_c1i1o1c0i1o1_disconnected(struct kunit *test)
    {
    struct device_node intc0_node = {
    .fwnode = { .ops = &intc0_fwnode_ops },
    };
    struct aspeed_intc_interrupt_range c1ranges[] = {
    {
    .start = 0,
    .count = 1,
    .upstream = {
    .fwnode = &intc0_node.fwnode,
    .param_count = 1,
    .param = { 128 }
    }
    }
    };
    static const u32 c1outs[] = { 0 };
    struct aspeed_intc_interrupt_range resolved;
    struct aspeed_intc_interrupt_range intc0_ranges[] = {
    {
    .start = 129,
    .count = 1,
    .upstream = {
    .fwnode = core::ptr::null_mut(),
    .param_count = 0,
    .param = { 0 },
    }
    }
    };
    struct aspeed_intc0 intc0 = {
    .ranges = {
    .ranges = intc0_ranges,
    .nranges = ARRAY_SIZE(intc0_ranges),
    }
    };
    const struct irq_domain c0domain = {
    .host_data = &intc0,
    .fwnode = &intc0_node.fwnode
    };
    int rc;
    rc = aspeed_intc0_resolve_route(&c0domain, ARRAY_SIZE(c1outs), c1outs,
    ARRAY_SIZE(c1ranges), c1ranges,
    &resolved);
    KUNIT_EXPECT_NE(test, rc, 0);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_intc0_resolve_route_c1i1o1mc0i1o1(test: *mut kunit) {
    static void aspeed_intc0_resolve_route_c1i1o1mc0i1o1(struct kunit *test)
    {
    struct device_node intc0_node = {
    .fwnode = { .ops = &intc0_fwnode_ops },
    };
    struct aspeed_intc_interrupt_range c1ranges[] = {
    {
    .start = 0,
    .count = 1,
    .upstream = {
    .fwnode = &intc0_node.fwnode,
    .param_count = 1,
    .param = { 480 }
    }
    }
    };
    static const u32 c1outs[] = { 0 };
    struct aspeed_intc_interrupt_range resolved;
    struct aspeed_intc_interrupt_range intc0_ranges[] = {
    {
    .start = 192,
    .count = 1,
    .upstream = {
    .fwnode = core::ptr::null_mut(),
    .param_count = 0,
    .param = { 0 },
    }
    }
    };
    struct aspeed_intc0 intc0 = {
    .ranges = {
    .ranges = intc0_ranges,
    .nranges = ARRAY_SIZE(intc0_ranges),
    }
    };
    const struct irq_domain c0domain = {
    .host_data = &intc0,
    .fwnode = &intc0_node.fwnode
    };
    int rc;
    rc = aspeed_intc0_resolve_route(&c0domain, ARRAY_SIZE(c1outs), c1outs,
    ARRAY_SIZE(c1ranges), c1ranges,
    &resolved);
    KUNIT_EXPECT_EQ(test, rc, 0);
    KUNIT_EXPECT_EQ(test, resolved.start, 0);
    KUNIT_EXPECT_EQ(test, resolved.count, 1);
    KUNIT_EXPECT_EQ(test, resolved.upstream.param[0], 480);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_intc0_resolve_route_c1i2o2mc0i1o1(test: *mut kunit) {
    static void aspeed_intc0_resolve_route_c1i2o2mc0i1o1(struct kunit *test)
    {
    struct device_node intc0_node = {
    .fwnode = { .ops = &intc0_fwnode_ops },
    };
    struct aspeed_intc_interrupt_range c1ranges[] = {
    {
    .start = 0,
    .count = 1,
    .upstream = {
    .fwnode = &intc0_node.fwnode,
    .param_count = 1,
    .param = { 480 }
    }
    },
    {
    .start = 1,
    .count = 1,
    .upstream = {
    .fwnode = &intc0_node.fwnode,
    .param_count = 1,
    .param = { 510 }
    }
    }
    };
    static const u32 c1outs[] = { 1 };
    struct aspeed_intc_interrupt_range resolved;
    static struct aspeed_intc_interrupt_range intc0_ranges[] = {
    {
    .start = 208,
    .count = 1,
    .upstream = {
    .fwnode = core::ptr::null_mut(),
    .param_count = 0,
    .param = { 0 },
    }
    }
    };
    struct aspeed_intc0 intc0 = {
    .ranges = {
    .ranges = intc0_ranges,
    .nranges = ARRAY_SIZE(intc0_ranges),
    }
    };
    const struct irq_domain c0domain = {
    .host_data = &intc0,
    .fwnode = &intc0_node.fwnode
    };
    int rc;
    rc = aspeed_intc0_resolve_route(&c0domain, ARRAY_SIZE(c1outs), c1outs,
    ARRAY_SIZE(c1ranges), c1ranges,
    &resolved);
    KUNIT_EXPECT_EQ(test, rc, 0);
    KUNIT_EXPECT_EQ(test, resolved.start, 1);
    KUNIT_EXPECT_EQ(test, resolved.count, 1);
    KUNIT_EXPECT_EQ(test, resolved.upstream.param[0], 510);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_intc0_resolve_route_c1i1o1mc0i2o1(test: *mut kunit) {
    static void aspeed_intc0_resolve_route_c1i1o1mc0i2o1(struct kunit *test)
    {
    struct device_node intc0_node = {
    .fwnode = { .ops = &intc0_fwnode_ops },
    };
    struct aspeed_intc_interrupt_range c1ranges[] = {
    {
    .start = 0,
    .count = 1,
    .upstream = {
    .fwnode = &intc0_node.fwnode,
    .param_count = 1,
    .param = { 510 }
    }
    },
    };
    static const u32 c1outs[] = { 0 };
    struct aspeed_intc_interrupt_range resolved;
    static struct aspeed_intc_interrupt_range intc0_ranges[] = {
    {
    .start = 192,
    .count = 1,
    .upstream = {
    .fwnode = core::ptr::null_mut(),
    .param_count = 0,
    .param = {0},
    }
    },
    {
    .start = 208,
    .count = 1,
    .upstream = {
    .fwnode = core::ptr::null_mut(),
    .param_count = 0,
    .param = {0},
    }
    }
    };
    struct aspeed_intc0 intc0 = {
    .ranges = {
    .ranges = intc0_ranges,
    .nranges = ARRAY_SIZE(intc0_ranges),
    }
    };
    const struct irq_domain c0domain = {
    .host_data = &intc0,
    .fwnode = &intc0_node.fwnode
    };
    int rc;
    rc = aspeed_intc0_resolve_route(&c0domain, ARRAY_SIZE(c1outs), c1outs,
    ARRAY_SIZE(c1ranges), c1ranges,
    &resolved);
    KUNIT_EXPECT_EQ(test, rc, 0);
    KUNIT_EXPECT_EQ(test, resolved.start, 0);
    KUNIT_EXPECT_EQ(test, resolved.count, 1);
    KUNIT_EXPECT_EQ(test, resolved.upstream.param[0], 510);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_intc0_resolve_route_c1i1o2mc0i1o1_invalid(test: *mut kunit) {
    static void aspeed_intc0_resolve_route_c1i1o2mc0i1o1_invalid(struct kunit *test)
    {
    struct device_node intc0_node = {
    .fwnode = { .ops = &intc0_fwnode_ops },
    };
    struct aspeed_intc_interrupt_range c1ranges[] = {
    {
    .start = 0,
    .count = 1,
    .upstream = {
    .fwnode = &intc0_node.fwnode,
    .param_count = 1,
    .param = { 480 }
    }
    }
    };
    static const u32 c1outs[] = {
    AST2700_INTC_INVALID_ROUTE, 0
    };
    struct aspeed_intc_interrupt_range resolved;
    struct aspeed_intc_interrupt_range intc0_ranges[] = {
    {
    .start = 192,
    .count = 1,
    .upstream = {
    .fwnode = core::ptr::null_mut(),
    .param_count = 0,
    .param = { 0 },
    }
    }
    };
    struct aspeed_intc0 intc0 = {
    .ranges = {
    .ranges = intc0_ranges,
    .nranges = ARRAY_SIZE(intc0_ranges),
    }
    };
    const struct irq_domain c0domain = {
    .host_data = &intc0,
    .fwnode = &intc0_node.fwnode
    };
    int rc;
    rc = aspeed_intc0_resolve_route(&c0domain, ARRAY_SIZE(c1outs), c1outs,
    ARRAY_SIZE(c1ranges), c1ranges,
    &resolved);
    KUNIT_EXPECT_EQ(test, rc, 1);
    KUNIT_EXPECT_EQ(test, resolved.start, 0);
    KUNIT_EXPECT_EQ(test, resolved.count, 1);
    KUNIT_EXPECT_EQ(test, resolved.upstream.param[0], 480);
    }
    static void
    aspeed_intc0_resolve_route_c1i1o1mc0i1o1_bad_range_upstream(struct kunit *test)
    {
    struct device_node intc0_node = {
    .fwnode = { .ops = &intc0_fwnode_ops },
    };
    struct aspeed_intc_interrupt_range c1ranges[] = {
    {
    .start = 0,
    .count = 1,
    .upstream = {
    .fwnode = &intc0_node.fwnode,
    .param_count = 0,
    .param = { 0 }
    }
    }
    };
    static const u32 c1outs[] = { 0 };
    struct aspeed_intc_interrupt_range resolved;
    struct aspeed_intc_interrupt_range intc0_ranges[] = {
    {
    .start = 0,
    .count = 0,
    .upstream = {
    .fwnode = core::ptr::null_mut(),
    .param_count = 0,
    .param = { 0 },
    }
    }
    };
    struct aspeed_intc0 intc0 = {
    .ranges = {
    .ranges = intc0_ranges,
    .nranges = ARRAY_SIZE(intc0_ranges),
    }
    };
    const struct irq_domain c0domain = {
    .host_data = &intc0,
    .fwnode = &intc0_node.fwnode
    };
    int rc;
    rc = aspeed_intc0_resolve_route(&c0domain, ARRAY_SIZE(c1outs), c1outs,
    ARRAY_SIZE(c1ranges), c1ranges,
    &resolved);
    KUNIT_EXPECT_NE(test, rc, 0);
    }
    static struct kunit_case ast2700_intc0_test_cases[] = {
    KUNIT_CASE(aspeed_intc0_resolve_route_bad_args),
    KUNIT_CASE(aspeed_intc_resolve_route_invalid_c0domain),
    KUNIT_CASE(aspeed_intc0_resolve_route_c1i1o1c0i1o1_connected),
    KUNIT_CASE(aspeed_intc0_resolve_route_c1i1o1c0i1o1_disconnected),
    KUNIT_CASE(aspeed_intc0_resolve_route_c1i1o1mc0i1o1),
    KUNIT_CASE(aspeed_intc0_resolve_route_c1i2o2mc0i1o1),
    KUNIT_CASE(aspeed_intc0_resolve_route_c1i1o1mc0i2o1),
    KUNIT_CASE(aspeed_intc0_resolve_route_c1i1o2mc0i1o1_invalid),
    KUNIT_CASE(aspeed_intc0_resolve_route_c1i1o1mc0i1o1_bad_range_upstream),
    {},
    };
    static struct kunit_suite ast2700_intc0_test_suite = {
    .name = "ast2700-intc0",
    .test_cases = ast2700_intc0_test_cases,
    };
    kunit_test_suite(ast2700_intc0_test_suite);
    MODULE_LICENSE("GPL");

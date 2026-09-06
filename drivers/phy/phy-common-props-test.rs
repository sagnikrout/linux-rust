//! Automatically rewritten from C to Rust
//! Source: drivers/phy/phy-common-props-test.c
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
// phy-common-props-test.c  --  Unit tests for PHY common properties API
//
// Copyright 2025-2026 NXP
//

// Test: rx-polarity property is missing
#[no_mangle]
unsafe extern "C" fn phy_test_rx_polarity_is_missing(test: *mut kunit) {
    static void phy_test_rx_polarity_is_missing(struct kunit *test)
    {
    static const struct property_entry entries[] = {
    {}
    };
    struct fwnode_handle *node;
    unsigned int val;
    int ret;
    node = fwnode_create_software_node(entries, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, node);
    ret = phy_get_manual_rx_polarity(node, "sgmii", &val);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, val, PHY_POL_NORMAL);
    fwnode_remove_software_node(node);
    }
// Test: rx-polarity has more values than rx-polarity-names
#[no_mangle]
unsafe extern "C" fn phy_test_rx_polarity_more_values_than_names(test: *mut kunit) {
    static void phy_test_rx_polarity_more_values_than_names(struct kunit *test)
    {
    static const u32 rx_pol[] = { PHY_POL_NORMAL, PHY_POL_INVERT, PHY_POL_NORMAL };
    static const char * const rx_pol_names[] = { "sgmii", "2500base-x" };
    static const struct property_entry entries[] = {
    PROPERTY_ENTRY_U32_ARRAY("rx-polarity", rx_pol),
    PROPERTY_ENTRY_STRING_ARRAY("rx-polarity-names", rx_pol_names),
    {}
    };
    struct fwnode_handle *node;
    unsigned int val;
    int ret;
    node = fwnode_create_software_node(entries, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, node);
    ret = phy_get_manual_rx_polarity(node, "sgmii", &val);
    KUNIT_EXPECT_EQ(test, ret, -EINVAL);
    fwnode_remove_software_node(node);
    }
// Test: rx-polarity has 1 value and rx-polarity-names does not exist
#[no_mangle]
unsafe extern "C" fn phy_test_rx_polarity_single_value_no_names(test: *mut kunit) {
    static void phy_test_rx_polarity_single_value_no_names(struct kunit *test)
    {
    static const u32 rx_pol[] = { PHY_POL_INVERT };
    static const struct property_entry entries[] = {
    PROPERTY_ENTRY_U32_ARRAY("rx-polarity", rx_pol),
    {}
    };
    struct fwnode_handle *node;
    unsigned int val;
    int ret;
    node = fwnode_create_software_node(entries, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, node);
    ret = phy_get_manual_rx_polarity(node, "sgmii", &val);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, val, PHY_POL_INVERT);
    fwnode_remove_software_node(node);
    }
// Test: rx-polarity-names has more values than rx-polarity
#[no_mangle]
unsafe extern "C" fn phy_test_rx_polarity_more_names_than_values(test: *mut kunit) {
    static void phy_test_rx_polarity_more_names_than_values(struct kunit *test)
    {
    static const u32 rx_pol[] = { PHY_POL_NORMAL, PHY_POL_INVERT };
    static const char * const rx_pol_names[] = { "sgmii", "2500base-x", "1000base-x" };
    static const struct property_entry entries[] = {
    PROPERTY_ENTRY_U32_ARRAY("rx-polarity", rx_pol),
    PROPERTY_ENTRY_STRING_ARRAY("rx-polarity-names", rx_pol_names),
    {}
    };
    struct fwnode_handle *node;
    unsigned int val;
    int ret;
    node = fwnode_create_software_node(entries, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, node);
    ret = phy_get_manual_rx_polarity(node, "sgmii", &val);
    KUNIT_EXPECT_EQ(test, ret, -EINVAL);
    fwnode_remove_software_node(node);
    }
// Test: rx-polarity and rx-polarity-names have same length, find the name
#[no_mangle]
unsafe extern "C" fn phy_test_rx_polarity_find_by_name(test: *mut kunit) {
    static void phy_test_rx_polarity_find_by_name(struct kunit *test)
    {
    static const u32 rx_pol[] = { PHY_POL_NORMAL, PHY_POL_INVERT, PHY_POL_AUTO };
    static const char * const rx_pol_names[] = { "sgmii", "2500base-x", "usb-ss" };
    static const struct property_entry entries[] = {
    PROPERTY_ENTRY_U32_ARRAY("rx-polarity", rx_pol),
    PROPERTY_ENTRY_STRING_ARRAY("rx-polarity-names", rx_pol_names),
    {}
    };
    struct fwnode_handle *node;
    unsigned int val;
    int ret;
    node = fwnode_create_software_node(entries, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, node);
    ret = phy_get_manual_rx_polarity(node, "sgmii", &val);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, val, PHY_POL_NORMAL);
    ret = phy_get_manual_rx_polarity(node, "2500base-x", &val);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, val, PHY_POL_INVERT);
    ret = phy_get_rx_polarity(node, "usb-ss", BIT(PHY_POL_AUTO),
    PHY_POL_AUTO, &val);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, val, PHY_POL_AUTO);
    fwnode_remove_software_node(node);
    }
// Test: same length, name not found, no "default" - error
#[no_mangle]
unsafe extern "C" fn phy_test_rx_polarity_name_not_found_no_default(test: *mut kunit) {
    static void phy_test_rx_polarity_name_not_found_no_default(struct kunit *test)
    {
    static const u32 rx_pol[] = { PHY_POL_NORMAL, PHY_POL_INVERT };
    static const char * const rx_pol_names[] = { "2500base-x", "1000base-x" };
    static const struct property_entry entries[] = {
    PROPERTY_ENTRY_U32_ARRAY("rx-polarity", rx_pol),
    PROPERTY_ENTRY_STRING_ARRAY("rx-polarity-names", rx_pol_names),
    {}
    };
    struct fwnode_handle *node;
    unsigned int val;
    int ret;
    node = fwnode_create_software_node(entries, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, node);
    ret = phy_get_manual_rx_polarity(node, "sgmii", &val);
    KUNIT_EXPECT_EQ(test, ret, -EINVAL);
    fwnode_remove_software_node(node);
    }
// Test: same length, name not found, but "default" exists
#[no_mangle]
unsafe extern "C" fn phy_test_rx_polarity_name_not_found_with_default(test: *mut kunit) {
    static void phy_test_rx_polarity_name_not_found_with_default(struct kunit *test)
    {
    static const u32 rx_pol[] = { PHY_POL_NORMAL, PHY_POL_INVERT };
    static const char * const rx_pol_names[] = { "2500base-x", "default" };
    static const struct property_entry entries[] = {
    PROPERTY_ENTRY_U32_ARRAY("rx-polarity", rx_pol),
    PROPERTY_ENTRY_STRING_ARRAY("rx-polarity-names", rx_pol_names),
    {}
    };
    struct fwnode_handle *node;
    unsigned int val;
    int ret;
    node = fwnode_create_software_node(entries, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, node);
    ret = phy_get_manual_rx_polarity(node, "sgmii", &val);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, val, PHY_POL_INVERT);
    fwnode_remove_software_node(node);
    }
// Test: polarity found but value is unsupported
#[no_mangle]
unsafe extern "C" fn phy_test_rx_polarity_unsupported_value(test: *mut kunit) {
    static void phy_test_rx_polarity_unsupported_value(struct kunit *test)
    {
    static const u32 rx_pol[] = { PHY_POL_AUTO };
    static const char * const rx_pol_names[] = { "sgmii" };
    static const struct property_entry entries[] = {
    PROPERTY_ENTRY_U32_ARRAY("rx-polarity", rx_pol),
    PROPERTY_ENTRY_STRING_ARRAY("rx-polarity-names", rx_pol_names),
    {}
    };
    struct fwnode_handle *node;
    unsigned int val;
    int ret;
    node = fwnode_create_software_node(entries, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, node);
    ret = phy_get_manual_rx_polarity(node, "sgmii", &val);
    KUNIT_EXPECT_EQ(test, ret, -EOPNOTSUPP);
    fwnode_remove_software_node(node);
    }
// Test: tx-polarity property is missing
#[no_mangle]
unsafe extern "C" fn phy_test_tx_polarity_is_missing(test: *mut kunit) {
    static void phy_test_tx_polarity_is_missing(struct kunit *test)
    {
    static const struct property_entry entries[] = {
    {}
    };
    struct fwnode_handle *node;
    unsigned int val;
    int ret;
    node = fwnode_create_software_node(entries, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, node);
    ret = phy_get_manual_tx_polarity(node, "sgmii", &val);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, val, PHY_POL_NORMAL);
    fwnode_remove_software_node(node);
    }
// Test: tx-polarity has more values than tx-polarity-names
#[no_mangle]
unsafe extern "C" fn phy_test_tx_polarity_more_values_than_names(test: *mut kunit) {
    static void phy_test_tx_polarity_more_values_than_names(struct kunit *test)
    {
    static const u32 tx_pol[] = { PHY_POL_NORMAL, PHY_POL_INVERT, PHY_POL_NORMAL };
    static const char * const tx_pol_names[] = { "sgmii", "2500base-x" };
    static const struct property_entry entries[] = {
    PROPERTY_ENTRY_U32_ARRAY("tx-polarity", tx_pol),
    PROPERTY_ENTRY_STRING_ARRAY("tx-polarity-names", tx_pol_names),
    {}
    };
    struct fwnode_handle *node;
    unsigned int val;
    int ret;
    node = fwnode_create_software_node(entries, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, node);
    ret = phy_get_manual_tx_polarity(node, "sgmii", &val);
    KUNIT_EXPECT_EQ(test, ret, -EINVAL);
    fwnode_remove_software_node(node);
    }
// Test: tx-polarity has 1 value and tx-polarity-names does not exist
#[no_mangle]
unsafe extern "C" fn phy_test_tx_polarity_single_value_no_names(test: *mut kunit) {
    static void phy_test_tx_polarity_single_value_no_names(struct kunit *test)
    {
    static const u32 tx_pol[] = { PHY_POL_INVERT };
    static const struct property_entry entries[] = {
    PROPERTY_ENTRY_U32_ARRAY("tx-polarity", tx_pol),
    {}
    };
    struct fwnode_handle *node;
    unsigned int val;
    int ret;
    node = fwnode_create_software_node(entries, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, node);
    ret = phy_get_manual_tx_polarity(node, "sgmii", &val);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, val, PHY_POL_INVERT);
    fwnode_remove_software_node(node);
    }
// Test: tx-polarity-names has more values than tx-polarity
#[no_mangle]
unsafe extern "C" fn phy_test_tx_polarity_more_names_than_values(test: *mut kunit) {
    static void phy_test_tx_polarity_more_names_than_values(struct kunit *test)
    {
    static const u32 tx_pol[] = { PHY_POL_NORMAL, PHY_POL_INVERT };
    static const char * const tx_pol_names[] = { "sgmii", "2500base-x", "1000base-x" };
    static const struct property_entry entries[] = {
    PROPERTY_ENTRY_U32_ARRAY("tx-polarity", tx_pol),
    PROPERTY_ENTRY_STRING_ARRAY("tx-polarity-names", tx_pol_names),
    {}
    };
    struct fwnode_handle *node;
    unsigned int val;
    int ret;
    node = fwnode_create_software_node(entries, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, node);
    ret = phy_get_manual_tx_polarity(node, "sgmii", &val);
    KUNIT_EXPECT_EQ(test, ret, -EINVAL);
    fwnode_remove_software_node(node);
    }
// Test: tx-polarity and tx-polarity-names have same length, find the name
#[no_mangle]
unsafe extern "C" fn phy_test_tx_polarity_find_by_name(test: *mut kunit) {
    static void phy_test_tx_polarity_find_by_name(struct kunit *test)
    {
    static const u32 tx_pol[] = { PHY_POL_NORMAL, PHY_POL_INVERT, PHY_POL_NORMAL };
    static const char * const tx_pol_names[] = { "sgmii", "2500base-x", "1000base-x" };
    static const struct property_entry entries[] = {
    PROPERTY_ENTRY_U32_ARRAY("tx-polarity", tx_pol),
    PROPERTY_ENTRY_STRING_ARRAY("tx-polarity-names", tx_pol_names),
    {}
    };
    struct fwnode_handle *node;
    unsigned int val;
    int ret;
    node = fwnode_create_software_node(entries, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, node);
    ret = phy_get_manual_tx_polarity(node, "sgmii", &val);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, val, PHY_POL_NORMAL);
    ret = phy_get_manual_tx_polarity(node, "2500base-x", &val);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, val, PHY_POL_INVERT);
    ret = phy_get_manual_tx_polarity(node, "1000base-x", &val);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, val, PHY_POL_NORMAL);
    fwnode_remove_software_node(node);
    }
// Test: same length, name not found, no "default" - error
#[no_mangle]
unsafe extern "C" fn phy_test_tx_polarity_name_not_found_no_default(test: *mut kunit) {
    static void phy_test_tx_polarity_name_not_found_no_default(struct kunit *test)
    {
    static const u32 tx_pol[] = { PHY_POL_NORMAL, PHY_POL_INVERT };
    static const char * const tx_pol_names[] = { "2500base-x", "1000base-x" };
    static const struct property_entry entries[] = {
    PROPERTY_ENTRY_U32_ARRAY("tx-polarity", tx_pol),
    PROPERTY_ENTRY_STRING_ARRAY("tx-polarity-names", tx_pol_names),
    {}
    };
    struct fwnode_handle *node;
    unsigned int val;
    int ret;
    node = fwnode_create_software_node(entries, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, node);
    ret = phy_get_manual_tx_polarity(node, "sgmii", &val);
    KUNIT_EXPECT_EQ(test, ret, -EINVAL);
    fwnode_remove_software_node(node);
    }
// Test: same length, name not found, but "default" exists
#[no_mangle]
unsafe extern "C" fn phy_test_tx_polarity_name_not_found_with_default(test: *mut kunit) {
    static void phy_test_tx_polarity_name_not_found_with_default(struct kunit *test)
    {
    static const u32 tx_pol[] = { PHY_POL_NORMAL, PHY_POL_INVERT };
    static const char * const tx_pol_names[] = { "2500base-x", "default" };
    static const struct property_entry entries[] = {
    PROPERTY_ENTRY_U32_ARRAY("tx-polarity", tx_pol),
    PROPERTY_ENTRY_STRING_ARRAY("tx-polarity-names", tx_pol_names),
    {}
    };
    struct fwnode_handle *node;
    unsigned int val;
    int ret;
    node = fwnode_create_software_node(entries, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, node);
    ret = phy_get_manual_tx_polarity(node, "sgmii", &val);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, val, PHY_POL_INVERT);
    fwnode_remove_software_node(node);
    }
// Test: polarity found but value is unsupported (AUTO for TX)
#[no_mangle]
unsafe extern "C" fn phy_test_tx_polarity_unsupported_value(test: *mut kunit) {
    static void phy_test_tx_polarity_unsupported_value(struct kunit *test)
    {
    static const u32 tx_pol[] = { PHY_POL_AUTO };
    static const char * const tx_pol_names[] = { "sgmii" };
    static const struct property_entry entries[] = {
    PROPERTY_ENTRY_U32_ARRAY("tx-polarity", tx_pol),
    PROPERTY_ENTRY_STRING_ARRAY("tx-polarity-names", tx_pol_names),
    {}
    };
    struct fwnode_handle *node;
    unsigned int val;
    int ret;
    node = fwnode_create_software_node(entries, core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, node);
    ret = phy_get_manual_tx_polarity(node, "sgmii", &val);
    KUNIT_EXPECT_EQ(test, ret, -EOPNOTSUPP);
    fwnode_remove_software_node(node);
    }
    static struct kunit_case phy_common_props_test_cases[] = {
    KUNIT_CASE(phy_test_rx_polarity_is_missing),
    KUNIT_CASE(phy_test_rx_polarity_more_values_than_names),
    KUNIT_CASE(phy_test_rx_polarity_single_value_no_names),
    KUNIT_CASE(phy_test_rx_polarity_more_names_than_values),
    KUNIT_CASE(phy_test_rx_polarity_find_by_name),
    KUNIT_CASE(phy_test_rx_polarity_name_not_found_no_default),
    KUNIT_CASE(phy_test_rx_polarity_name_not_found_with_default),
    KUNIT_CASE(phy_test_rx_polarity_unsupported_value),
    KUNIT_CASE(phy_test_tx_polarity_is_missing),
    KUNIT_CASE(phy_test_tx_polarity_more_values_than_names),
    KUNIT_CASE(phy_test_tx_polarity_single_value_no_names),
    KUNIT_CASE(phy_test_tx_polarity_more_names_than_values),
    KUNIT_CASE(phy_test_tx_polarity_find_by_name),
    KUNIT_CASE(phy_test_tx_polarity_name_not_found_no_default),
    KUNIT_CASE(phy_test_tx_polarity_name_not_found_with_default),
    KUNIT_CASE(phy_test_tx_polarity_unsupported_value),
    {}
    };
    static struct kunit_suite phy_common_props_test_suite = {
    .name = "phy-common-props",
    .test_cases = phy_common_props_test_cases,
    };
    kunit_test_suite(phy_common_props_test_suite);
    MODULE_DESCRIPTION("Test module for PHY common properties API");
    MODULE_AUTHOR("Vladimir Oltean <vladimir.oltean@nxp.com>");
    MODULE_LICENSE("GPL");

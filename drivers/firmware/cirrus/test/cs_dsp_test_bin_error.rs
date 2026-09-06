//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/cirrus/test/cs_dsp_test_bin_error.c
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
// KUnit tests for cs_dsp.
//
// Copyright (C) 2024 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

    KUNIT_DEFINE_ACTION_WRAPPER(_put_device_wrapper, put_device, struct device *);
    KUNIT_DEFINE_ACTION_WRAPPER(_cs_dsp_remove_wrapper, cs_dsp_remove, struct cs_dsp *);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_dsp_test_local {
    pub bin_builder: *mut cs_dsp_mock_bin_builder,
    pub xm_header: *mut cs_dsp_mock_xm_header,
    pub wmfw_builder: *mut cs_dsp_mock_wmfw_builder,
    pub wmfw: *mut firmware,
    pub wmfw_version: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_dsp_bin_test_param {
    pub block_type: c_int,
}

    static const struct cs_dsp_mock_alg_def cs_dsp_bin_err_test_mock_algs[] = {
    {
    .id = 0xfafa,
    .ver = 0x100000,
    .xm_size_words = 164,
    .ym_size_words = 164,
    .zm_size_words = 164,
    },
    };
// Load a bin containing unknown blocks. They should be skipped.
#[no_mangle]
unsafe extern "C" fn bin_load_with_unknown_blocks(test: *mut kunit) {
    static void bin_load_with_unknown_blocks(struct kunit *test)
    {
    struct cs_dsp_test *priv = test.priv;
    struct cs_dsp_test_local *local = priv.local;
    struct firmware *bin;
    unsigned int reg_addr;
    u8 *payload_data, *readback;
    u8 random_data[8];
    let mut payload_size_bytes: c_uint = 64;
    payload_data = kunit_kmalloc(test, payload_size_bytes, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, payload_data);
    get_random_bytes(payload_data, payload_size_bytes);
    readback = kunit_kzalloc(test, payload_size_bytes, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, readback);
// Add some unknown blocks at the start of the bin
    get_random_bytes(random_data, sizeof(random_data));
    cs_dsp_mock_bin_add_raw_block(local.bin_builder,
    cs_dsp_bin_err_test_mock_algs[0].id,
    cs_dsp_bin_err_test_mock_algs[0].ver,
    0xf5, 0, 0,
    random_data, sizeof(random_data));
    cs_dsp_mock_bin_add_raw_block(local.bin_builder,
    cs_dsp_bin_err_test_mock_algs[0].id,
    cs_dsp_bin_err_test_mock_algs[0].ver,
    0xf500, 0, 0,
    random_data, sizeof(random_data));
    cs_dsp_mock_bin_add_raw_block(local.bin_builder,
    cs_dsp_bin_err_test_mock_algs[0].id,
    cs_dsp_bin_err_test_mock_algs[0].ver,
    0xc300, 0, 0,
    random_data, sizeof(random_data));
// Add a single payload to be written to DSP memory
    cs_dsp_mock_bin_add_raw_block(local.bin_builder,
    cs_dsp_bin_err_test_mock_algs[0].id,
    cs_dsp_bin_err_test_mock_algs[0].ver,
    WMFW_ADSP2_YM, 0, 0,
    payload_data, payload_size_bytes);
    bin = cs_dsp_mock_bin_get_firmware(local.bin_builder);
    KUNIT_EXPECT_EQ(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
// Check that the payload was written to memory
    reg_addr = cs_dsp_mock_base_addr_for_mem(priv, WMFW_ADSP2_YM);
    KUNIT_EXPECT_EQ(test,
    regmap_raw_read(priv.dsp.regmap, reg_addr, readback, payload_size_bytes),
    0);
    KUNIT_EXPECT_MEMEQ(test, readback, payload_data, payload_size_bytes);
    }
// Load a bin that doesn't have a valid magic marker.
#[no_mangle]
unsafe extern "C" fn bin_err_wrong_magic(test: *mut kunit) {
    static void bin_err_wrong_magic(struct kunit *test)
    {
    struct cs_dsp_test *priv = test.priv;
    struct cs_dsp_test_local *local = priv.local;
    struct firmware *bin;
// Sanity-check that the wmfw loads ok without the bin
    KUNIT_EXPECT_EQ(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", core::ptr::null_mut(), core::ptr::null_mut(), "misc"),
    0);
    cs_dsp_power_down(priv.dsp);
    bin = cs_dsp_mock_bin_get_firmware(local.bin_builder);
    memcpy((void *)bin.data, "WMFW", 4);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    memcpy((void *)bin.data, "xMDR", 4);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    memcpy((void *)bin.data, "WxDR", 4);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    memcpy((void *)bin.data, "WMxR", 4);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    memcpy((void *)bin.data, "WMDx", 4);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    memset((void *)bin.data, 0, 4);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    }
// Load a bin that is too short for a valid header.
#[no_mangle]
unsafe extern "C" fn bin_err_too_short_for_header(test: *mut kunit) {
    static void bin_err_too_short_for_header(struct kunit *test)
    {
    struct cs_dsp_test *priv = test.priv;
    struct cs_dsp_test_local *local = priv.local;
    struct firmware *bin;
// Sanity-check that the wmfw loads ok without the bin
    KUNIT_EXPECT_EQ(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", core::ptr::null_mut(), core::ptr::null_mut(), "misc"),
    0);
    cs_dsp_power_down(priv.dsp);
    bin = cs_dsp_mock_bin_get_firmware(local.bin_builder);
    do {
    bin.size--;
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    } while (bin.size > 0);
    }
// Header length field isn't a valid header length.
#[no_mangle]
unsafe extern "C" fn bin_err_bad_header_length(test: *mut kunit) {
    static void bin_err_bad_header_length(struct kunit *test)
    {
    struct cs_dsp_test *priv = test.priv;
    struct cs_dsp_test_local *local = priv.local;
    struct firmware *bin;
    struct wmfw_coeff_hdr *header;
    unsigned int real_len, len;
// Sanity-check that the wmfw loads ok without the bin
    KUNIT_EXPECT_EQ(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", core::ptr::null_mut(), core::ptr::null_mut(), "misc"),
    0);
    cs_dsp_power_down(priv.dsp);
    bin = cs_dsp_mock_bin_get_firmware(local.bin_builder);
    header = (struct wmfw_coeff_hdr *)bin.data;
    real_len = le32_to_cpu(header.len);
    for (len = 0; len < real_len; len++) {
    header.len = cpu_to_le32(len);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    }
    for (len = real_len + 1; len < real_len + 7; len++) {
    header.len = cpu_to_le32(len);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    }
    header.len = cpu_to_le32(0xffffffff);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    header.len = cpu_to_le32(0x80000000);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    header.len = cpu_to_le32(0x7fffffff);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    }
// Wrong core type in header.
#[no_mangle]
unsafe extern "C" fn bin_err_bad_core_type(test: *mut kunit) {
    static void bin_err_bad_core_type(struct kunit *test)
    {
    struct cs_dsp_test *priv = test.priv;
    struct cs_dsp_test_local *local = priv.local;
    struct firmware *bin;
    struct wmfw_coeff_hdr *header;
// Sanity-check that the wmfw loads ok without the bin
    KUNIT_EXPECT_EQ(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", core::ptr::null_mut(), core::ptr::null_mut(), "misc"),
    0);
    cs_dsp_power_down(priv.dsp);
    bin = cs_dsp_mock_bin_get_firmware(local.bin_builder);
    header = (struct wmfw_coeff_hdr *)bin.data;
    header.core_ver = cpu_to_le32(0);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    header.core_ver = cpu_to_le32(1);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    header.core_ver = cpu_to_le32(priv.dsp.type + 1);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    header.core_ver = cpu_to_le32(0xff);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    }
// File too short to contain a full block header
#[no_mangle]
unsafe extern "C" fn bin_too_short_for_block_header(test: *mut kunit) {
    static void bin_too_short_for_block_header(struct kunit *test)
    {
    const struct cs_dsp_bin_test_param *param = test.param_value;
    struct cs_dsp_test *priv = test.priv;
    struct cs_dsp_test_local *local = priv.local;
    struct firmware *bin;
    unsigned int header_length;
// Sanity-check that the wmfw loads ok without the bin
    KUNIT_EXPECT_EQ(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", core::ptr::null_mut(), core::ptr::null_mut(), "misc"),
    0);
    cs_dsp_power_down(priv.dsp);
    bin = cs_dsp_mock_bin_get_firmware(local.bin_builder);
    header_length = bin.size;
    kunit_kfree(test, bin);
    cs_dsp_mock_bin_add_raw_block(local.bin_builder,
    cs_dsp_bin_err_test_mock_algs[0].id,
    cs_dsp_bin_err_test_mock_algs[0].ver,
    param.block_type, 0, 0,
    core::ptr::null_mut(), 0);
    bin = cs_dsp_mock_bin_get_firmware(local.bin_builder);
    KUNIT_ASSERT_GT(test, bin.size, header_length);
    for (bin.size--; bin.size > header_length; bin.size--) {
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    }
    }
// File too short to contain the block payload
#[no_mangle]
unsafe extern "C" fn bin_too_short_for_block_payload(test: *mut kunit) {
    static void bin_too_short_for_block_payload(struct kunit *test)
    {
    const struct cs_dsp_bin_test_param *param = test.param_value;
    struct cs_dsp_test *priv = test.priv;
    struct cs_dsp_test_local *local = priv.local;
    struct firmware *bin;
    static const u8 payload[256] = { };
    int i;
// Sanity-check that the wmfw loads ok without the bin
    KUNIT_EXPECT_EQ(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", core::ptr::null_mut(), core::ptr::null_mut(), "misc"),
    0);
    cs_dsp_power_down(priv.dsp);
    cs_dsp_mock_bin_add_raw_block(local.bin_builder,
    cs_dsp_bin_err_test_mock_algs[0].id,
    cs_dsp_bin_err_test_mock_algs[0].ver,
    param.block_type, 0, 0,
    payload, sizeof(payload));
    bin = cs_dsp_mock_bin_get_firmware(local.bin_builder);
    for (i = 0; i < sizeof(payload); i++) {
    bin.size--;
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    }
    }
// Block payload length is a garbage value
#[no_mangle]
unsafe extern "C" fn bin_block_payload_len_garbage(test: *mut kunit) {
    static void bin_block_payload_len_garbage(struct kunit *test)
    {
    const struct cs_dsp_bin_test_param *param = test.param_value;
    struct cs_dsp_test *priv = test.priv;
    struct cs_dsp_test_local *local = priv.local;
    struct firmware *bin;
    struct wmfw_coeff_hdr *header;
    struct wmfw_coeff_item *block;
    let mut payload: u32 = 0;
// Sanity-check that the wmfw loads ok without the bin
    KUNIT_EXPECT_EQ(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", core::ptr::null_mut(), core::ptr::null_mut(), "misc"),
    0);
    cs_dsp_power_down(priv.dsp);
    cs_dsp_mock_bin_add_raw_block(local.bin_builder,
    cs_dsp_bin_err_test_mock_algs[0].id,
    cs_dsp_bin_err_test_mock_algs[0].ver,
    param.block_type, 0, 0,
    &payload, sizeof(payload));
    bin = cs_dsp_mock_bin_get_firmware(local.bin_builder);
    header = (struct wmfw_coeff_hdr *)bin.data;
    block = (struct wmfw_coeff_item *)&bin.data[le32_to_cpu(header.len)];
// Sanity check that we're looking at the correct part of the bin
    KUNIT_ASSERT_EQ(test, le16_to_cpu(block.type), param.block_type);
    KUNIT_ASSERT_EQ(test, le32_to_cpu(block.len), sizeof(payload));
    block.len = cpu_to_le32(0x8000);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    block.len = cpu_to_le32(0xffff);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    block.len = cpu_to_le32(0x7fffffff);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    block.len = cpu_to_le32(0x80000000);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    block.len = cpu_to_le32(0xffffffff);
    KUNIT_EXPECT_LT(test,
    cs_dsp_power_up(priv.dsp, local.wmfw, "wmfw", bin, "bin", "misc"),
    0);
    }
#[no_mangle]
unsafe extern "C" fn cs_dsp_bin_err_test_can_emit_message_hook() -> bool {
    static bool cs_dsp_bin_err_test_can_emit_message_hook(void)
    {

    return true;

    return false;

    }
    static int cs_dsp_bin_err_test_common_init(struct kunit *test, struct cs_dsp *dsp,
    int wmfw_version)
    {
    struct cs_dsp_test *priv;
    struct cs_dsp_test_local *local;
    struct device *test_dev;
    int ret;
    priv = kunit_kzalloc(test, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    local = kunit_kzalloc(test, sizeof(struct cs_dsp_test_local), GFP_KERNEL);
    if (!local)
    return -ENOMEM;
    priv.test = test;
    priv.dsp = dsp;
    test.priv = priv;
    priv.local = local;
    priv.local.wmfw_version = wmfw_version;
// Create dummy struct device
    test_dev = kunit_device_register(test, "cs_dsp_test_drv");
    if (IS_ERR(test_dev))
    return PTR_ERR(test_dev);
    dsp.dev = get_device(test_dev);
    if (!dsp.dev)
    return -ENODEV;
    ret = kunit_add_action_or_reset(test, _put_device_wrapper, dsp.dev);
    if (ret)
    return ret;
    dev_set_drvdata(dsp.dev, priv);
// Allocate regmap
    ret = cs_dsp_mock_regmap_init(priv);
    if (ret)
    return ret;
//
// There must always be a XM header with at least 1 algorithm, so create
// a dummy one that tests can use and extract it to a data payload.
//
    local.xm_header = cs_dsp_create_mock_xm_header(priv,
    cs_dsp_bin_err_test_mock_algs,
    ARRAY_SIZE(cs_dsp_bin_err_test_mock_algs));
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, local.xm_header);
    local.wmfw_builder = cs_dsp_mock_wmfw_init(priv, priv.local.wmfw_version);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, local.wmfw_builder);
// Add dummy XM header payload to wmfw
    cs_dsp_mock_wmfw_add_data_block(local.wmfw_builder,
    WMFW_ADSP2_XM, 0,
    local.xm_header.blob_data,
    local.xm_header.blob_size_bytes);
    local.wmfw = cs_dsp_mock_wmfw_get_firmware(priv.local.wmfw_builder);
    local.bin_builder =
    cs_dsp_mock_bin_init(priv, 1,
    cs_dsp_mock_xm_header_get_fw_version(local.xm_header));
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, local.bin_builder);
// Init cs_dsp
    dsp.client_ops = kunit_kzalloc(test, sizeof(*dsp.client_ops), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, dsp.client_ops);
    switch (dsp.type) {
    case WMFW_ADSP2:
    ret = cs_dsp_adsp2_init(dsp);
    break;
    case WMFW_HALO:
    ret = cs_dsp_halo_init(dsp);
    break;
    default:
    KUNIT_FAIL(test, "Untested DSP type %d\n", dsp.type);
    return -EINVAL;
    }
    if (ret)
    return ret;
// Automatically call cs_dsp_remove() when test case ends
    ret = kunit_add_action_or_reset(priv.test, _cs_dsp_remove_wrapper, dsp);
    if (ret)
    return ret;
//
// Testing error conditions can produce a lot of log output
// from cs_dsp error messages, so suppress messages.
//
    kunit_activate_static_stub(test, cs_dsp_can_emit_message,
    cs_dsp_bin_err_test_can_emit_message_hook);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cs_dsp_bin_err_test_halo_init(test: *mut kunit) -> c_int {
    static int cs_dsp_bin_err_test_halo_init(struct kunit *test)
    {
    struct cs_dsp *dsp;
// Fill in cs_dsp and initialize
    dsp = kunit_kzalloc(test, sizeof(*dsp), GFP_KERNEL);
    if (!dsp)
    return -ENOMEM;
    dsp.num = 1;
    dsp.type = WMFW_HALO;
    dsp.mem = cs_dsp_mock_halo_dsp1_regions;
    dsp.num_mems = cs_dsp_mock_count_regions(cs_dsp_mock_halo_dsp1_region_sizes);
    dsp.base = cs_dsp_mock_halo_core_base;
    dsp.base_sysinfo = cs_dsp_mock_halo_sysinfo_base;
    return cs_dsp_bin_err_test_common_init(test, dsp, 3);
    }
#[no_mangle]
unsafe extern "C" fn cs_dsp_bin_err_test_adsp2_32bit_init(test: *mut kunit) -> c_int {
    static int cs_dsp_bin_err_test_adsp2_32bit_init(struct kunit *test)
    {
    struct cs_dsp *dsp;
// Fill in cs_dsp and initialize
    dsp = kunit_kzalloc(test, sizeof(*dsp), GFP_KERNEL);
    if (!dsp)
    return -ENOMEM;
    dsp.num = 1;
    dsp.type = WMFW_ADSP2;
    dsp.rev = 1;
    dsp.mem = cs_dsp_mock_adsp2_32bit_dsp1_regions;
    dsp.num_mems = cs_dsp_mock_count_regions(cs_dsp_mock_adsp2_32bit_dsp1_region_sizes);
    dsp.base = cs_dsp_mock_adsp2_32bit_sysbase;
    return cs_dsp_bin_err_test_common_init(test, dsp, 2);
    }
#[no_mangle]
unsafe extern "C" fn cs_dsp_bin_err_test_adsp2_16bit_init(test: *mut kunit) -> c_int {
    static int cs_dsp_bin_err_test_adsp2_16bit_init(struct kunit *test)
    {
    struct cs_dsp *dsp;
// Fill in cs_dsp and initialize
    dsp = kunit_kzalloc(test, sizeof(*dsp), GFP_KERNEL);
    if (!dsp)
    return -ENOMEM;
    dsp.num = 1;
    dsp.type = WMFW_ADSP2;
    dsp.rev = 0;
    dsp.mem = cs_dsp_mock_adsp2_16bit_dsp1_regions;
    dsp.num_mems = cs_dsp_mock_count_regions(cs_dsp_mock_adsp2_16bit_dsp1_region_sizes);
    dsp.base = cs_dsp_mock_adsp2_16bit_sysbase;
    return cs_dsp_bin_err_test_common_init(test, dsp, 1);
    }
    static void cs_dsp_bin_err_block_types_desc(const struct cs_dsp_bin_test_param *param,
    char *desc)
    {
    snprintf(desc, KUNIT_PARAM_DESC_SIZE, "block_type:%#x", param.block_type);
    }
// Some block types to test against, including illegal types
    static const struct cs_dsp_bin_test_param bin_test_block_types_cases[] = {
    { .block_type = WMFW_INFO_TEXT << 8 },
    { .block_type = WMFW_METADATA << 8 },
    { .block_type = WMFW_ADSP2_PM },
    { .block_type = WMFW_ADSP2_XM },
    { .block_type = 0x33 },
    { .block_type = 0xf500 },
    { .block_type = 0xc000 },
    };
    KUNIT_ARRAY_PARAM(bin_test_block_types,
    bin_test_block_types_cases,
    cs_dsp_bin_err_block_types_desc);
    static struct kunit_case cs_dsp_bin_err_test_cases[] = {
    KUNIT_CASE(bin_load_with_unknown_blocks),
    KUNIT_CASE(bin_err_wrong_magic),
    KUNIT_CASE(bin_err_too_short_for_header),
    KUNIT_CASE(bin_err_bad_header_length),
    KUNIT_CASE(bin_err_bad_core_type),
    KUNIT_CASE_PARAM(bin_too_short_for_block_header, bin_test_block_types_gen_params),
    KUNIT_CASE_PARAM(bin_too_short_for_block_payload, bin_test_block_types_gen_params),
    KUNIT_CASE_PARAM(bin_block_payload_len_garbage, bin_test_block_types_gen_params),
    { } /* terminator */
    };
    static struct kunit_suite cs_dsp_bin_err_test_halo = {
    .name = "cs_dsp_bin_err_halo",
    .init = cs_dsp_bin_err_test_halo_init,
    .test_cases = cs_dsp_bin_err_test_cases,
    .attr.speed = KUNIT_SPEED_SLOW,
    };
    static struct kunit_suite cs_dsp_bin_err_test_adsp2_32bit = {
    .name = "cs_dsp_bin_err_adsp2_32bit",
    .init = cs_dsp_bin_err_test_adsp2_32bit_init,
    .test_cases = cs_dsp_bin_err_test_cases,
    .attr.speed = KUNIT_SPEED_SLOW,
    };
    static struct kunit_suite cs_dsp_bin_err_test_adsp2_16bit = {
    .name = "cs_dsp_bin_err_adsp2_16bit",
    .init = cs_dsp_bin_err_test_adsp2_16bit_init,
    .test_cases = cs_dsp_bin_err_test_cases,
    .attr.speed = KUNIT_SPEED_SLOW,
    };
    kunit_test_suites(&cs_dsp_bin_err_test_halo,
    &cs_dsp_bin_err_test_adsp2_32bit,
    &cs_dsp_bin_err_test_adsp2_16bit);

//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-uclogic-rdesc-test.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// HID driver for UC-Logic devices not fully compliant with HID standard
//
// Copyright (c) 2022 José Expósito <jose.exposito89@gmail.com>
//

    MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uclogic_template_case {
    pub name: *const c_char,
    pub template: *const __u8,
    pub template_size: usize,
    pub param_list: *const i32,
    pub param_num: usize,
    pub expected: *const __u8,
}

    static const s32 params_pen_all[UCLOGIC_RDESC_PH_ID_NUM] = {
    [UCLOGIC_RDESC_PEN_PH_ID_X_LM] = 0xAA,
    [UCLOGIC_RDESC_PEN_PH_ID_X_PM] = 0xBB,
    [UCLOGIC_RDESC_PEN_PH_ID_Y_LM] = 0xCC,
    [UCLOGIC_RDESC_PEN_PH_ID_Y_PM] = 0xDD,
    [UCLOGIC_RDESC_PEN_PH_ID_PRESSURE_LM] = 0xEE,
    };
    static const s32 params_pen_some[] = {
    [UCLOGIC_RDESC_PEN_PH_ID_X_LM] = 0xAA,
    [UCLOGIC_RDESC_PEN_PH_ID_X_PM] = 0xBB,
    };
    static const s32 params_frame_all[UCLOGIC_RDESC_PH_ID_NUM] = {
    [UCLOGIC_RDESC_FRAME_PH_ID_UM] = 0xFF,
    };
    static const __u8 template_empty[] = { };
    static const __u8 template_small[] = { 0x00 };
    static const __u8 template_no_ph[] = { 0xAA, 0xFE, 0xAA, 0xED, 0x1D };
    static const __u8 template_pen_ph_end[] = {
    0xAA, 0xBB, UCLOGIC_RDESC_PEN_PH_HEAD
    };
    static const __u8 template_btn_ph_end[] = {
    0xAA, 0xBB, UCLOGIC_RDESC_FRAME_PH_BTN_HEAD
    };
    static const __u8 template_pen_all_params[] = {
    UCLOGIC_RDESC_PEN_PH(X_LM),
    0x47, UCLOGIC_RDESC_PEN_PH(X_PM),
    0x27, UCLOGIC_RDESC_PEN_PH(Y_LM),
    UCLOGIC_RDESC_PEN_PH(Y_PM),
    0x00, UCLOGIC_RDESC_PEN_PH(PRESSURE_LM),
    };
    static const __u8 expected_pen_all_params[] = {
    0xAA, 0x00, 0x00, 0x00,
    0x47, 0xBB, 0x00, 0x00, 0x00,
    0x27, 0xCC, 0x00, 0x00, 0x00,
    0xDD, 0x00, 0x00, 0x00,
    0x00, 0xEE, 0x00, 0x00, 0x00,
    };
    static const __u8 template_frame_all_params[] = {
    0x01, 0x02,
    UCLOGIC_RDESC_FRAME_PH_BTN,
    0x99,
    };
    static const __u8 expected_frame_all_params[] = {
    0x01, 0x02,
    0x2A, 0xFF, 0x00,
    0x99,
    };
    static const __u8 template_pen_some_params[] = {
    0x01, 0x02,
    UCLOGIC_RDESC_PEN_PH(X_LM),
    0x03, UCLOGIC_RDESC_PEN_PH(X_PM),
    0x04, 0x05,
    };
    static const __u8 expected_pen_some_params[] = {
    0x01, 0x02,
    0xAA, 0x00, 0x00, 0x00,
    0x03, 0xBB, 0x00, 0x00, 0x00,
    0x04, 0x05,
    };
    static const __u8 template_params_none[] = {
    0x27, UCLOGIC_RDESC_PEN_PH(Y_LM),
    UCLOGIC_RDESC_PEN_PH(Y_PM),
    0x00, UCLOGIC_RDESC_PEN_PH(PRESSURE_LM),
    };
    static struct uclogic_template_case uclogic_template_cases[] = {
    {
    .name = "empty_template",
    .template = template_empty,
    .template_size = sizeof(template_empty),
    .param_list = params_pen_all,
    .param_num = ARRAY_SIZE(params_pen_all),
    .expected = template_empty,
    },
    {
    .name = "template_smaller_than_the_placeholder",
    .template = template_small,
    .template_size = sizeof(template_small),
    .param_list = params_pen_all,
    .param_num = ARRAY_SIZE(params_pen_all),
    .expected = template_small,
    },
    {
    .name = "no_placeholder",
    .template = template_no_ph,
    .template_size = sizeof(template_no_ph),
    .param_list = params_pen_all,
    .param_num = ARRAY_SIZE(params_pen_all),
    .expected = template_no_ph,
    },
    {
    .name = "pen_placeholder_at_the_end_without_id",
    .template = template_pen_ph_end,
    .template_size = sizeof(template_pen_ph_end),
    .param_list = params_pen_all,
    .param_num = ARRAY_SIZE(params_pen_all),
    .expected = template_pen_ph_end,
    },
    {
    .name = "frame_button_placeholder_at_the_end_without_id",
    .template = template_btn_ph_end,
    .template_size = sizeof(template_btn_ph_end),
    .param_list = params_frame_all,
    .param_num = ARRAY_SIZE(params_frame_all),
    .expected = template_btn_ph_end,
    },
    {
    .name = "all_params_present_in_the_pen_template",
    .template = template_pen_all_params,
    .template_size = sizeof(template_pen_all_params),
    .param_list = params_pen_all,
    .param_num = ARRAY_SIZE(params_pen_all),
    .expected = expected_pen_all_params,
    },
    {
    .name = "all_params_present_in_the_frame_template",
    .template = template_frame_all_params,
    .template_size = sizeof(template_frame_all_params),
    .param_list = params_frame_all,
    .param_num = ARRAY_SIZE(params_frame_all),
    .expected = expected_frame_all_params,
    },
    {
    .name = "some_params_present_in_the_pen_template_with_complete_param_list",
    .template = template_pen_some_params,
    .template_size = sizeof(template_pen_some_params),
    .param_list = params_pen_all,
    .param_num = ARRAY_SIZE(params_pen_all),
    .expected = expected_pen_some_params,
    },
    {
    .name = "some_params_present_in_the_pen_template_with_incomplete_param_list",
    .template = template_pen_some_params,
    .template_size = sizeof(template_pen_some_params),
    .param_list = params_pen_some,
    .param_num = ARRAY_SIZE(params_pen_some),
    .expected = expected_pen_some_params,
    },
    {
    .name = "no_params_present_in_the_template",
    .template = template_params_none,
    .template_size = sizeof(template_params_none),
    .param_list = params_pen_some,
    .param_num = ARRAY_SIZE(params_pen_some),
    .expected = template_params_none,
    },
    };
    static void uclogic_template_case_desc(struct uclogic_template_case *t,
    char *desc)
    {
    strscpy(desc, t.name, KUNIT_PARAM_DESC_SIZE);
    }
    KUNIT_ARRAY_PARAM(uclogic_template, uclogic_template_cases,
    uclogic_template_case_desc);
#[no_mangle]
unsafe extern "C" fn hid_test_uclogic_template(test: *mut kunit) {
    static void hid_test_uclogic_template(struct kunit *test)
    {
    __u8 *res;
    const struct uclogic_template_case *params = test.param_value;
    res = uclogic_rdesc_template_apply(params.template,
    params.template_size,
    params.param_list,
    params.param_num);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, res);
    KUNIT_EXPECT_MEMEQ(test, res, params.expected, params.template_size);
    kfree(res);
    }
    static struct kunit_case hid_uclogic_rdesc_test_cases[] = {
    KUNIT_CASE_PARAM(hid_test_uclogic_template, uclogic_template_gen_params),
    {}
    };
    static struct kunit_suite hid_uclogic_rdesc_test_suite = {
    .name = "hid_uclogic_rdesc_test",
    .test_cases = hid_uclogic_rdesc_test_cases,
    };
    kunit_test_suite(hid_uclogic_rdesc_test_suite);
    MODULE_DESCRIPTION("KUnit tests for the UC-Logic driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("José Expósito <jose.exposito89@gmail.com>");

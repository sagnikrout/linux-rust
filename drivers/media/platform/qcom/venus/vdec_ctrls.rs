//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/venus/vdec_ctrls.c
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
// Copyright (c) 2012-2016, The Linux Foundation. All rights reserved.
// Copyright (C) 2017 Linaro Ltd.
//

#[no_mangle]
unsafe extern "C" fn vdec_op_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int vdec_op_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct venus_inst *inst = ctrl_to_inst(ctrl);
    struct vdec_controls *ctr = &inst.controls.dec;
    switch (ctrl.id) {
    case V4L2_CID_MPEG_VIDEO_DECODER_MPEG4_DEBLOCK_FILTER:
    ctr.post_loop_deb_mode = ctrl.val;
    break;
    case V4L2_CID_MPEG_VIDEO_H264_PROFILE:
    case V4L2_CID_MPEG_VIDEO_MPEG4_PROFILE:
    case V4L2_CID_MPEG_VIDEO_VP8_PROFILE:
    case V4L2_CID_MPEG_VIDEO_VP9_PROFILE:
    ctr.profile = ctrl.val;
    break;
    case V4L2_CID_MPEG_VIDEO_H264_LEVEL:
    case V4L2_CID_MPEG_VIDEO_MPEG4_LEVEL:
    case V4L2_CID_MPEG_VIDEO_VP9_LEVEL:
    ctr.level = ctrl.val;
    break;
    case V4L2_CID_MPEG_VIDEO_DEC_DISPLAY_DELAY:
    ctr.display_delay = ctrl.val;
    break;
    case V4L2_CID_MPEG_VIDEO_DEC_DISPLAY_DELAY_ENABLE:
    ctr.display_delay_enable = ctrl.val;
    break;
    case V4L2_CID_MPEG_VIDEO_DEC_CONCEAL_COLOR:
    ctr.conceal_color = *ctrl.p_new.p_s64;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vdec_op_g_volatile_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int vdec_op_g_volatile_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct venus_inst *inst = ctrl_to_inst(ctrl);
    struct vdec_controls *ctr = &inst.controls.dec;
    struct hfi_buffer_requirements bufreq;
    let mut ver: enum hfi_version = inst.core.res.hfi_version;
    u32 profile, level;
    int ret;
    switch (ctrl.id) {
    case V4L2_CID_MPEG_VIDEO_H264_PROFILE:
    case V4L2_CID_MPEG_VIDEO_MPEG4_PROFILE:
    case V4L2_CID_MPEG_VIDEO_VP8_PROFILE:
    case V4L2_CID_MPEG_VIDEO_VP9_PROFILE:
    ret = venus_helper_get_profile_level(inst, &profile, &level);
    if (!ret)
    ctr.profile = profile;
    ctrl.val = ctr.profile;
    break;
    case V4L2_CID_MPEG_VIDEO_H264_LEVEL:
    case V4L2_CID_MPEG_VIDEO_MPEG4_LEVEL:
    case V4L2_CID_MPEG_VIDEO_VP9_LEVEL:
    ret = venus_helper_get_profile_level(inst, &profile, &level);
    if (!ret)
    ctr.level = level;
    ctrl.val = ctr.level;
    break;
    case V4L2_CID_MPEG_VIDEO_DECODER_MPEG4_DEBLOCK_FILTER:
    ctrl.val = ctr.post_loop_deb_mode;
    break;
    case V4L2_CID_MIN_BUFFERS_FOR_CAPTURE:
    ret = venus_helper_get_bufreq(inst, HFI_BUFFER_OUTPUT, &bufreq);
    if (!ret)
    ctrl.val = hfi_bufreq_get_count_min(&bufreq, ver);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct v4l2_ctrl_ops vdec_ctrl_ops = {
    .s_ctrl = vdec_op_s_ctrl,
    .g_volatile_ctrl = vdec_op_g_volatile_ctrl,
    };
#[no_mangle]
pub unsafe extern "C" fn vdec_ctrl_init(inst: *mut venus_inst) -> c_int {
    int vdec_ctrl_init(struct venus_inst *inst)
    {
    struct v4l2_ctrl *ctrl;
    int ret;
    ret = v4l2_ctrl_handler_init(&inst.ctrl_handler, 12);
    if (ret)
    return ret;
    ctrl = v4l2_ctrl_new_std_menu(&inst.ctrl_handler, &vdec_ctrl_ops,
    V4L2_CID_MPEG_VIDEO_MPEG4_PROFILE,
    V4L2_MPEG_VIDEO_MPEG4_PROFILE_ADVANCED_CODING_EFFICIENCY,
    ~((1 << V4L2_MPEG_VIDEO_MPEG4_PROFILE_SIMPLE) |
    (1 << V4L2_MPEG_VIDEO_MPEG4_PROFILE_ADVANCED_SIMPLE)),
    V4L2_MPEG_VIDEO_MPEG4_PROFILE_SIMPLE);
    if (ctrl)
    ctrl.flags |= V4L2_CTRL_FLAG_VOLATILE;
    ctrl = v4l2_ctrl_new_std_menu(&inst.ctrl_handler, &vdec_ctrl_ops,
    V4L2_CID_MPEG_VIDEO_MPEG4_LEVEL,
    V4L2_MPEG_VIDEO_MPEG4_LEVEL_5,
    0, V4L2_MPEG_VIDEO_MPEG4_LEVEL_0);
    if (ctrl)
    ctrl.flags |= V4L2_CTRL_FLAG_VOLATILE;
    ctrl = v4l2_ctrl_new_std_menu(&inst.ctrl_handler, &vdec_ctrl_ops,
    V4L2_CID_MPEG_VIDEO_H264_PROFILE,
    V4L2_MPEG_VIDEO_H264_PROFILE_MULTIVIEW_HIGH,
    ~((1 << V4L2_MPEG_VIDEO_H264_PROFILE_BASELINE) |
    (1 << V4L2_MPEG_VIDEO_H264_PROFILE_CONSTRAINED_BASELINE) |
    (1 << V4L2_MPEG_VIDEO_H264_PROFILE_MAIN) |
    (1 << V4L2_MPEG_VIDEO_H264_PROFILE_HIGH) |
    (1 << V4L2_MPEG_VIDEO_H264_PROFILE_STEREO_HIGH) |
    (1 << V4L2_MPEG_VIDEO_H264_PROFILE_MULTIVIEW_HIGH)),
    V4L2_MPEG_VIDEO_H264_PROFILE_BASELINE);
    if (ctrl)
    ctrl.flags |= V4L2_CTRL_FLAG_VOLATILE;
    ctrl = v4l2_ctrl_new_std_menu(&inst.ctrl_handler, &vdec_ctrl_ops,
    V4L2_CID_MPEG_VIDEO_H264_LEVEL,
    V4L2_MPEG_VIDEO_H264_LEVEL_5_1,
    0, V4L2_MPEG_VIDEO_H264_LEVEL_1_0);
    if (ctrl)
    ctrl.flags |= V4L2_CTRL_FLAG_VOLATILE;
    ctrl = v4l2_ctrl_new_std_menu(&inst.ctrl_handler, &vdec_ctrl_ops,
    V4L2_CID_MPEG_VIDEO_VP8_PROFILE,
    V4L2_MPEG_VIDEO_VP8_PROFILE_3,
    0, V4L2_MPEG_VIDEO_VP8_PROFILE_0);
    if (ctrl)
    ctrl.flags |= V4L2_CTRL_FLAG_VOLATILE;
    ctrl = v4l2_ctrl_new_std_menu(&inst.ctrl_handler, &vdec_ctrl_ops,
    V4L2_CID_MPEG_VIDEO_VP9_PROFILE,
    V4L2_MPEG_VIDEO_VP9_PROFILE_3,
    0, V4L2_MPEG_VIDEO_VP9_PROFILE_0);
    if (ctrl)
    ctrl.flags |= V4L2_CTRL_FLAG_VOLATILE;
    ctrl = v4l2_ctrl_new_std_menu(&inst.ctrl_handler, &vdec_ctrl_ops,
    V4L2_CID_MPEG_VIDEO_VP9_LEVEL,
    V4L2_MPEG_VIDEO_VP9_LEVEL_6_2,
    0, V4L2_MPEG_VIDEO_VP9_LEVEL_1_0);
    if (ctrl)
    ctrl.flags |= V4L2_CTRL_FLAG_VOLATILE;
    v4l2_ctrl_new_std(&inst.ctrl_handler, &vdec_ctrl_ops,
    V4L2_CID_MPEG_VIDEO_DECODER_MPEG4_DEBLOCK_FILTER, 0, 1, 1, 0);
    ctrl = v4l2_ctrl_new_std(&inst.ctrl_handler, &vdec_ctrl_ops,
    V4L2_CID_MIN_BUFFERS_FOR_CAPTURE, 1, 32, 1, 1);
    if (ctrl)
    ctrl.flags |= V4L2_CTRL_FLAG_VOLATILE;
    v4l2_ctrl_new_std(&inst.ctrl_handler, &vdec_ctrl_ops,
    V4L2_CID_MPEG_VIDEO_DEC_DISPLAY_DELAY,
    0, 16383, 1, 0);
    v4l2_ctrl_new_std(&inst.ctrl_handler, &vdec_ctrl_ops,
    V4L2_CID_MPEG_VIDEO_DEC_DISPLAY_DELAY_ENABLE,
    0, 1, 1, 0);
    v4l2_ctrl_new_std(&inst.ctrl_handler, &vdec_ctrl_ops,
    V4L2_CID_MPEG_VIDEO_DEC_CONCEAL_COLOR, 0,
    0xffffffffffffLL, 1, 0x8000800010LL);
    ret = inst.ctrl_handler.error;
    if (ret) {
    v4l2_ctrl_handler_free(&inst.ctrl_handler);
    return ret;
    }
    return 0;
    }

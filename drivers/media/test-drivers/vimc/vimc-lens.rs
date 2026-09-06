//! Automatically rewritten from C to Rust
//! Source: drivers/media/test-drivers/vimc/vimc-lens.c
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
// vimc-lens.c Virtual Media Controller Driver
// Copyright (C) 2022 Google, Inc
// Author: yunkec@google.com (Yunke Cao)
//

pub const VIMC_LENS_MAX_FOCUS_POS: c_int = 1023;
pub const VIMC_LENS_MAX_FOCUS_STEP: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vimc_lens_device {
    pub ved: vimc_ent_device,
    pub sd: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
    pub focus_absolute: u32,
}

    static const struct v4l2_subdev_core_ops vimc_lens_core_ops = {
    .log_status = v4l2_ctrl_subdev_log_status,
    .subscribe_event = v4l2_ctrl_subdev_subscribe_event,
    .unsubscribe_event = v4l2_event_subdev_unsubscribe,
    };
    static const struct v4l2_subdev_ops vimc_lens_ops = {
    .core = &vimc_lens_core_ops
    };
#[no_mangle]
unsafe extern "C" fn vimc_lens_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int vimc_lens_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct vimc_lens_device *vlens =
    container_of(ctrl.handler, struct vimc_lens_device, hdl);
    if (ctrl.id == V4L2_CID_FOCUS_ABSOLUTE) {
    vlens.focus_absolute = ctrl.val;
    return 0;
    }
    return -EINVAL;
    }
    static const struct v4l2_ctrl_ops vimc_lens_ctrl_ops = {
    .s_ctrl = vimc_lens_s_ctrl,
    };
    static struct vimc_ent_device *vimc_lens_add(struct vimc_device *vimc,
    const char *vcfg_name)
    {
    struct v4l2_device *v4l2_dev = &vimc.v4l2_dev;
    struct vimc_lens_device *vlens;
    int ret;
// Allocate the vlens struct
    vlens = kzalloc_obj(*vlens);
    if (!vlens)
    return ERR_PTR(-ENOMEM);
    v4l2_ctrl_handler_init(&vlens.hdl, 1);
    v4l2_ctrl_new_std(&vlens.hdl, &vimc_lens_ctrl_ops,
    V4L2_CID_FOCUS_ABSOLUTE, 0,
    VIMC_LENS_MAX_FOCUS_POS, VIMC_LENS_MAX_FOCUS_STEP, 0);
    vlens.sd.ctrl_handler = &vlens.hdl;
    if (vlens.hdl.error) {
    ret = vlens.hdl.error;
    goto err_free_vlens;
    }
    vlens.ved.dev = vimc.mdev.dev;
    ret = vimc_ent_sd_register(&vlens.ved, &vlens.sd, v4l2_dev,
    vcfg_name, MEDIA_ENT_F_LENS, 0,
    core::ptr::null_mut(), core::ptr::null_mut(), &vimc_lens_ops);
    if (ret)
    goto err_free_hdl;
    return &vlens.ved;
    err_free_hdl:
    v4l2_ctrl_handler_free(&vlens.hdl);
    err_free_vlens:
    kfree(vlens);
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn vimc_lens_release(ved: *mut vimc_ent_device) {
    static void vimc_lens_release(struct vimc_ent_device *ved)
    {
    struct vimc_lens_device *vlens =
    container_of(ved, struct vimc_lens_device, ved);
    v4l2_ctrl_handler_free(&vlens.hdl);
    v4l2_subdev_cleanup(&vlens.sd);
    media_entity_cleanup(vlens.ved.ent);
    kfree(vlens);
    }
    const struct vimc_ent_type vimc_lens_type = {
    .add = vimc_lens_add,
    .release = vimc_lens_release
    };

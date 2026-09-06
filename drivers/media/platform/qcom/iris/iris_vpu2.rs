//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/qcom/iris/iris_vpu2.c
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
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//

#[no_mangle]
unsafe extern "C" fn iris_vpu2_calc_freq(inst: *mut iris_inst, data_size: usize) -> u64 {
    static u64 iris_vpu2_calc_freq(struct iris_inst *inst, size_t data_size)
    {
    struct platform_inst_caps *caps = inst.core.iris_platform_data.inst_caps;
    struct v4l2_format *inp_f = inst.fmt_src;
    u32 mbs_per_second, mbpf, height, width;
    unsigned long vpp_freq, vsp_freq;
    let mut fps: u32 = inst.frame_rate;
    width = max(inp_f.fmt.pix_mp.width, inst.crop.width);
    height = max(inp_f.fmt.pix_mp.height, inst.crop.height);
    mbpf = NUM_MBS_PER_FRAME(height, width);
    mbs_per_second = mbpf * fps;
    vpp_freq = mbs_per_second * caps.mb_cycles_vpp;
// 21 / 20 is overhead factor
    vpp_freq += vpp_freq / 20;
    vsp_freq = mbs_per_second * caps.mb_cycles_vsp;
// 10 / 7 is overhead factor
    vsp_freq += ((fps * data_size * 8) * 10) / 7;
    return max(vpp_freq, vsp_freq);
    }
    const struct vpu_ops iris_vpu2_ops = {
    .power_off_hw = iris_vpu_power_off_hw,
    .power_on_hw = iris_vpu_power_on_hw,
    .power_off_controller = iris_vpu_power_off_controller,
    .power_on_controller = iris_vpu_power_on_controller,
    .calc_freq = iris_vpu2_calc_freq,
    .set_hwmode = iris_vpu_set_hwmode,
    };

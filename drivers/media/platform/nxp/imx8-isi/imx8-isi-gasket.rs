//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/nxp/imx8-isi/imx8-isi-gasket.c
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
// Copyright 2019-2023 NXP
//

// -----------------------------------------------------------------------------
// i.MX8MN and i.MX8MP gasket
//

pub const GASKET_CTRL: c_uint = 0x0000;

pub const GASKET_HSIZE: c_uint = 0x0004;
pub const GASKET_VSIZE: c_uint = 0x0008;
    static void mxc_imx8_gasket_enable(struct mxc_isi_dev *isi,
    const struct v4l2_mbus_frame_desc *fd,
    const struct v4l2_mbus_framefmt *fmt,
    const unsigned int port)
    {
    u32 val;
    regmap_write(isi.gasket, GASKET_BASE(port) + GASKET_HSIZE, fmt.width);
    regmap_write(isi.gasket, GASKET_BASE(port) + GASKET_VSIZE, fmt.height);
    val = GASKET_CTRL_DATA_TYPE(fd.entry[0].bus.csi2.dt);
    if (fd.entry[0].bus.csi2.dt == MIPI_CSI2_DT_YUV422_8B)
    val |= GASKET_CTRL_DUAL_COMP_ENABLE;
    val |= GASKET_CTRL_ENABLE;
    regmap_write(isi.gasket, GASKET_BASE(port) + GASKET_CTRL, val);
    }
    static void mxc_imx8_gasket_disable(struct mxc_isi_dev *isi,
    const unsigned int port)
    {
    regmap_write(isi.gasket, GASKET_BASE(port) + GASKET_CTRL, 0);
    }
    const struct mxc_gasket_ops mxc_imx8_gasket_ops = {
    .enable = mxc_imx8_gasket_enable,
    .disable = mxc_imx8_gasket_disable,
    };
// -----------------------------------------------------------------------------
// i.MX93 gasket
//
pub const DISP_MIX_CAMERA_MUX: c_uint = 0x30;

    static void mxc_imx93_gasket_enable(struct mxc_isi_dev *isi,
    const struct v4l2_mbus_frame_desc *fd,
    const struct v4l2_mbus_framefmt *fmt,
    const unsigned int port)
    {
    u32 val;
    val = DISP_MIX_CAMERA_MUX_DATA_TYPE(fd.entry[0].bus.csi2.dt);
    val |= DISP_MIX_CAMERA_MUX_GASKET_ENABLE;
//
// CAMERA MUX
// - [17]:	Selects source input to gasket
// 0: Data from MIPI CSI
// 1: Data from parallel camera
//
    if (fd.type == V4L2_MBUS_FRAME_DESC_TYPE_PARALLEL)
    val |= DISP_MIX_CAMERA_MUX_GASKET_SOURCE_TYPE;
    regmap_write(isi.gasket, DISP_MIX_CAMERA_MUX, val);
    }
    static void mxc_imx93_gasket_disable(struct mxc_isi_dev *isi,
    unsigned int port)
    {
    regmap_write(isi.gasket, DISP_MIX_CAMERA_MUX, 0);
    }
    const struct mxc_gasket_ops mxc_imx93_gasket_ops = {
    .enable = mxc_imx93_gasket_enable,
    .disable = mxc_imx93_gasket_disable,
    };

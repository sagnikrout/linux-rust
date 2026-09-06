//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/ipu-v3/ipu-vdi.c
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
// Copyright (C) 2012-2016 Mentor Graphics Inc.
// Copyright (C) 2005-2009 Freescale Semiconductor, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_vdi {
    pub base: *mut void __iomem,
    pub module: u32,
    pub lock: spinlock_t,
    pub use_count: c_int,
    pub ipu: *mut ipu_soc,
}

// VDI Register Offsets
pub const VDI_FSIZE: c_uint = 0x0000;
pub const VDI_C: c_uint = 0x0004;
// VDI Register Fields

pub const VDI_C_BURST_SIZE_MASK: c_uint = 0xF;
pub const VDI_C_BURST_SIZE1_OFFSET: c_int = 4;
pub const VDI_C_BURST_SIZE2_OFFSET: c_int = 8;
pub const VDI_C_BURST_SIZE3_OFFSET: c_int = 12;

#[no_mangle]
pub unsafe extern "C" fn ipu_vdi_read(vdi: *mut ipu_vdi, offset: c_uint) -> u32 {
    static inline u32 ipu_vdi_read(struct ipu_vdi *vdi, unsigned int offset)
    {
    return readl(vdi.base + offset);
    }
    static inline void ipu_vdi_write(struct ipu_vdi *vdi, u32 value,
    unsigned int offset)
    {
    writel(value, vdi.base + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_vdi_set_field_order(vdi: *mut ipu_vdi, std: v4l2_std_id, field: u32) {
    void ipu_vdi_set_field_order(struct ipu_vdi *vdi, v4l2_std_id std, u32 field)
    {
    let mut top_field_0: bool = false;
    unsigned long flags;
    u32 reg;
    switch (field) {
    case V4L2_FIELD_INTERLACED_TB:
    case V4L2_FIELD_SEQ_TB:
    case V4L2_FIELD_TOP:
    top_field_0 = true;
    break;
    case V4L2_FIELD_INTERLACED_BT:
    case V4L2_FIELD_SEQ_BT:
    case V4L2_FIELD_BOTTOM:
    top_field_0 = false;
    break;
    default:
    top_field_0 = (std & V4L2_STD_525_60) ? true : false;
    break;
    }
    spin_lock_irqsave(&vdi.lock, flags);
    reg = ipu_vdi_read(vdi, VDI_C);
    if (top_field_0)
    reg &= ~(VDI_C_TOP_FIELD_MAN_1 | VDI_C_TOP_FIELD_AUTO_1);
    else
    reg |= VDI_C_TOP_FIELD_MAN_1 | VDI_C_TOP_FIELD_AUTO_1;
    ipu_vdi_write(vdi, reg, VDI_C);
    spin_unlock_irqrestore(&vdi.lock, flags);
    }
    EXPORT_SYMBOL_GPL(ipu_vdi_set_field_order);
#[no_mangle]
pub unsafe extern "C" fn ipu_vdi_set_motion(vdi: *mut ipu_vdi, motion_sel: enum ipu_motion_sel) {
    void ipu_vdi_set_motion(struct ipu_vdi *vdi, enum ipu_motion_sel motion_sel)
    {
    unsigned long flags;
    u32 reg;
    spin_lock_irqsave(&vdi.lock, flags);
    reg = ipu_vdi_read(vdi, VDI_C);
    reg &= ~VDI_C_MOT_SEL_MASK;
    switch (motion_sel) {
    case MED_MOTION:
    reg |= VDI_C_MOT_SEL_MED;
    break;
    case HIGH_MOTION:
    reg |= VDI_C_MOT_SEL_FULL;
    break;
    default:
    reg |= VDI_C_MOT_SEL_LOW;
    break;
    }
    ipu_vdi_write(vdi, reg, VDI_C);
    spin_unlock_irqrestore(&vdi.lock, flags);
    }
    EXPORT_SYMBOL_GPL(ipu_vdi_set_motion);
#[no_mangle]
pub unsafe extern "C" fn ipu_vdi_setup(vdi: *mut ipu_vdi, code: u32, xres: c_int, yres: c_int) {
    void ipu_vdi_setup(struct ipu_vdi *vdi, u32 code, int xres, int yres)
    {
    unsigned long flags;
    u32 pixel_fmt, reg;
    spin_lock_irqsave(&vdi.lock, flags);
    reg = ((yres - 1) << 16) | (xres - 1);
    ipu_vdi_write(vdi, reg, VDI_FSIZE);
//
// Full motion, only vertical filter is used.
// Burst size is 4 accesses
//
    if (code == MEDIA_BUS_FMT_UYVY8_2X8 ||
    code == MEDIA_BUS_FMT_UYVY8_1X16 ||
    code == MEDIA_BUS_FMT_YUYV8_2X8 ||
    code == MEDIA_BUS_FMT_YUYV8_1X16)
    pixel_fmt = VDI_C_CH_422;
    else
    pixel_fmt = VDI_C_CH_420;
    reg = ipu_vdi_read(vdi, VDI_C);
    reg |= pixel_fmt;
    reg |= VDI_C_BURST_SIZE2_4;
    reg |= VDI_C_BURST_SIZE1_4 | VDI_C_VWM1_CLR_2;
    reg |= VDI_C_BURST_SIZE3_4 | VDI_C_VWM3_CLR_2;
    ipu_vdi_write(vdi, reg, VDI_C);
    spin_unlock_irqrestore(&vdi.lock, flags);
    }
    EXPORT_SYMBOL_GPL(ipu_vdi_setup);
#[no_mangle]
pub unsafe extern "C" fn ipu_vdi_enable(vdi: *mut ipu_vdi) -> c_int {
    int ipu_vdi_enable(struct ipu_vdi *vdi)
    {
    unsigned long flags;
    spin_lock_irqsave(&vdi.lock, flags);
    if (!vdi.use_count)
    ipu_module_enable(vdi.ipu, vdi.module);
    vdi.use_count++;
    spin_unlock_irqrestore(&vdi.lock, flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_vdi_enable);
#[no_mangle]
pub unsafe extern "C" fn ipu_vdi_disable(vdi: *mut ipu_vdi) -> c_int {
    int ipu_vdi_disable(struct ipu_vdi *vdi)
    {
    unsigned long flags;
    spin_lock_irqsave(&vdi.lock, flags);
    if (vdi.use_count) {
    if (!--vdi.use_count)
    ipu_module_disable(vdi.ipu, vdi.module);
    }
    spin_unlock_irqrestore(&vdi.lock, flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_vdi_disable);
    struct ipu_vdi *ipu_vdi_get(struct ipu_soc *ipu)
    {
    return ipu.vdi_priv;
    }
    EXPORT_SYMBOL_GPL(ipu_vdi_get);
#[no_mangle]
pub unsafe extern "C" fn ipu_vdi_put(vdi: *mut ipu_vdi) {
    void ipu_vdi_put(struct ipu_vdi *vdi)
    {
    }
    EXPORT_SYMBOL_GPL(ipu_vdi_put);
    int ipu_vdi_init(struct ipu_soc *ipu, struct device *dev,
    unsigned long base, u32 module)
    {
    struct ipu_vdi *vdi;
    vdi = devm_kzalloc(dev, sizeof(*vdi), GFP_KERNEL);
    if (!vdi)
    return -ENOMEM;
    ipu.vdi_priv = vdi;
    spin_lock_init(&vdi.lock);
    vdi.module = module;
    vdi.base = devm_ioremap(dev, base, PAGE_SIZE);
    if (!vdi.base)
    return -ENOMEM;
    dev_dbg(dev, "VDI base: 0x%08lx remapped to %p\n", base, vdi.base);
    vdi.ipu = ipu;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_vdi_exit(ipu: *mut ipu_soc) {
    void ipu_vdi_exit(struct ipu_soc *ipu)
    {
    }

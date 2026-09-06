//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/ipu-v3/ipu-ic.c
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
// Copyright (C) 2012-2014 Mentor Graphics Inc.
// Copyright 2005-2012 Freescale Semiconductor, Inc. All Rights Reserved.
//

// IC Register Offsets
pub const IC_CONF: c_uint = 0x0000;
pub const IC_PRP_ENC_RSC: c_uint = 0x0004;
pub const IC_PRP_VF_RSC: c_uint = 0x0008;
pub const IC_PP_RSC: c_uint = 0x000C;
pub const IC_CMBP_1: c_uint = 0x0010;
pub const IC_CMBP_2: c_uint = 0x0014;
pub const IC_IDMAC_1: c_uint = 0x0018;
pub const IC_IDMAC_2: c_uint = 0x001C;
pub const IC_IDMAC_3: c_uint = 0x0020;
pub const IC_IDMAC_4: c_uint = 0x0024;
// IC Register Fields

pub const IC_IDMAC_1_PRPENC_ROT_OFFSET: c_int = 11;

pub const IC_IDMAC_1_PRPVF_ROT_OFFSET: c_int = 14;

pub const IC_IDMAC_1_PP_ROT_OFFSET: c_int = 17;

pub const IC_IDMAC_2_PRPENC_HEIGHT_OFFSET: c_int = 0;

pub const IC_IDMAC_2_PRPVF_HEIGHT_OFFSET: c_int = 10;

pub const IC_IDMAC_2_PP_HEIGHT_OFFSET: c_int = 20;

pub const IC_IDMAC_3_PRPENC_WIDTH_OFFSET: c_int = 0;

pub const IC_IDMAC_3_PRPVF_WIDTH_OFFSET: c_int = 10;

pub const IC_IDMAC_3_PP_WIDTH_OFFSET: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ic_task_regoffs {
    pub rsc: u32,
    pub tpmem_csc: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ic_task_bitfields {
    pub ic_conf_en: u32,
    pub ic_conf_rot_en: u32,
    pub ic_conf_cmb_en: u32,
    pub ic_conf_csc1_en: u32,
    pub ic_conf_csc2_en: u32,
    pub ic_cmb_galpha_bit: u32,
}

    static const struct ic_task_regoffs ic_task_reg[IC_NUM_TASKS] = {
    [IC_TASK_ENCODER] = {
    .rsc = IC_PRP_ENC_RSC,
    .tpmem_csc = {0x2008, 0},
    },
    [IC_TASK_VIEWFINDER] = {
    .rsc = IC_PRP_VF_RSC,
    .tpmem_csc = {0x4028, 0x4040},
    },
    [IC_TASK_POST_PROCESSOR] = {
    .rsc = IC_PP_RSC,
    .tpmem_csc = {0x6060, 0x6078},
    },
    };
    static const struct ic_task_bitfields ic_task_bit[IC_NUM_TASKS] = {
    [IC_TASK_ENCODER] = {
    .ic_conf_en = IC_CONF_PRPENC_EN,
    .ic_conf_rot_en = IC_CONF_PRPENC_ROT_EN,
    .ic_conf_cmb_en = 0,    /* NA */
    .ic_conf_csc1_en = IC_CONF_PRPENC_CSC1,
    .ic_conf_csc2_en = 0,   /* NA */
    .ic_cmb_galpha_bit = 0, /* NA */
    },
    [IC_TASK_VIEWFINDER] = {
    .ic_conf_en = IC_CONF_PRPVF_EN,
    .ic_conf_rot_en = IC_CONF_PRPVF_ROT_EN,
    .ic_conf_cmb_en = IC_CONF_PRPVF_CMB,
    .ic_conf_csc1_en = IC_CONF_PRPVF_CSC1,
    .ic_conf_csc2_en = IC_CONF_PRPVF_CSC2,
    .ic_cmb_galpha_bit = 0,
    },
    [IC_TASK_POST_PROCESSOR] = {
    .ic_conf_en = IC_CONF_PP_EN,
    .ic_conf_rot_en = IC_CONF_PP_ROT_EN,
    .ic_conf_cmb_en = IC_CONF_PP_CMB,
    .ic_conf_csc1_en = IC_CONF_PP_CSC1,
    .ic_conf_csc2_en = IC_CONF_PP_CSC2,
    .ic_cmb_galpha_bit = 8,
    },
    };
    struct ipu_ic_priv;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_ic {
    pub task: enum ipu_ic_task,
    pub reg: *const ic_task_regoffs,
    pub bit: *const ic_task_bitfields,
    pub in_cs: ipu_ic_colorspace,
    pub g_in_cs: ipu_ic_colorspace,
    pub out_cs: ipu_ic_colorspace,
    pub graphics: bool,
    pub rotation: bool,
    pub in_use: bool,
    pub priv: *mut ipu_ic_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_ic_priv {
    pub base: *mut void __iomem,
    pub tpmem_base: *mut void __iomem,
    pub lock: spinlock_t,
    pub ipu: *mut ipu_soc,
    pub use_count: c_int,
    pub irt_use_count: c_int,
    pub task: [ipu_ic; IC_NUM_TASKS],
}

#[no_mangle]
pub unsafe extern "C" fn ipu_ic_read(ic: *mut ipu_ic, offset: unsigned) -> u32 {
    static inline u32 ipu_ic_read(struct ipu_ic *ic, unsigned offset)
    {
    return readl(ic.priv.base + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_ic_write(ic: *mut ipu_ic, value: u32, offset: unsigned) {
    static inline void ipu_ic_write(struct ipu_ic *ic, u32 value, unsigned offset)
    {
    writel(value, ic.priv.base + offset);
    }
    static int init_csc(struct ipu_ic *ic,
    const struct ipu_ic_csc *csc,
    int csc_index)
    {
    struct ipu_ic_priv *priv = ic.priv;
    u32 __iomem *base;
    const u16 (*c)[3];
    const u16 *a;
    u32 param;
    base = (u32 __iomem *)
    (priv.tpmem_base + ic.reg.tpmem_csc[csc_index]);
// Cast to unsigned
    c = (const u16 (*)[3])csc.params.coeff;
    a = (const u16 *)csc.params.offset;
    param = ((a[0] & 0x1f) << 27) | ((c[0][0] & 0x1ff) << 18) |
    ((c[1][1] & 0x1ff) << 9) | (c[2][2] & 0x1ff);
    writel(param, base++);
    param = ((a[0] & 0x1fe0) >> 5) | (csc.params.scale << 8) |
    (csc.params.sat << 10);
    writel(param, base++);
    param = ((a[1] & 0x1f) << 27) | ((c[0][1] & 0x1ff) << 18) |
    ((c[1][0] & 0x1ff) << 9) | (c[2][0] & 0x1ff);
    writel(param, base++);
    param = ((a[1] & 0x1fe0) >> 5);
    writel(param, base++);
    param = ((a[2] & 0x1f) << 27) | ((c[0][2] & 0x1ff) << 18) |
    ((c[1][2] & 0x1ff) << 9) | (c[2][1] & 0x1ff);
    writel(param, base++);
    param = ((a[2] & 0x1fe0) >> 5);
    writel(param, base++);
    return 0;
    }
    static int calc_resize_coeffs(struct ipu_ic *ic,
    u32 in_size, u32 out_size,
    u32 *resize_coeff,
    u32 *downsize_coeff)
    {
    struct ipu_ic_priv *priv = ic.priv;
    struct ipu_soc *ipu = priv.ipu;
    u32 temp_size, temp_downsize;
//
// Input size cannot be more than 4096, and output size cannot
// be more than 1024
//
    if (in_size > 4096) {
    dev_err(ipu.dev, "Unsupported resize (in_size > 4096)\n");
    return -EINVAL;
    }
    if (out_size > 1024) {
    dev_err(ipu.dev, "Unsupported resize (out_size > 1024)\n");
    return -EINVAL;
    }
// Cannot downsize more than 4:1
    if ((out_size << 2) < in_size) {
    dev_err(ipu.dev, "Unsupported downsize\n");
    return -EINVAL;
    }
// Compute downsizing coefficient
    temp_downsize = 0;
    temp_size = in_size;
    while (((temp_size > 1024) || (temp_size >= out_size * 2)) &&
    (temp_downsize < 2)) {
    temp_size >>= 1;
    temp_downsize++;
    }
// downsize_coeff = temp_downsize;
//
// compute resizing coefficient using the following equation:
// resize_coeff = M * (SI - 1) / (SO - 1)
// where M = 2^13, SI = input size, SO = output size
//
// resize_coeff = (8192L * (temp_size - 1)) / (out_size - 1);
    if (*resize_coeff >= 16384L) {
    dev_err(ipu.dev, "Warning! Overflow on resize coeff.\n");
// resize_coeff = 0x3FFF;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_ic_task_enable(ic: *mut ipu_ic) {
    void ipu_ic_task_enable(struct ipu_ic *ic)
    {
    struct ipu_ic_priv *priv = ic.priv;
    unsigned long flags;
    u32 ic_conf;
    spin_lock_irqsave(&priv.lock, flags);
    ic_conf = ipu_ic_read(ic, IC_CONF);
    ic_conf |= ic.bit.ic_conf_en;
    if (ic.rotation)
    ic_conf |= ic.bit.ic_conf_rot_en;
    if (ic.in_cs.cs != ic.out_cs.cs)
    ic_conf |= ic.bit.ic_conf_csc1_en;
    if (ic.graphics) {
    ic_conf |= ic.bit.ic_conf_cmb_en;
    ic_conf |= ic.bit.ic_conf_csc1_en;
    if (ic.g_in_cs.cs != ic.out_cs.cs)
    ic_conf |= ic.bit.ic_conf_csc2_en;
    }
    ipu_ic_write(ic, ic_conf, IC_CONF);
    spin_unlock_irqrestore(&priv.lock, flags);
    }
    EXPORT_SYMBOL_GPL(ipu_ic_task_enable);
#[no_mangle]
pub unsafe extern "C" fn ipu_ic_task_disable(ic: *mut ipu_ic) {
    void ipu_ic_task_disable(struct ipu_ic *ic)
    {
    struct ipu_ic_priv *priv = ic.priv;
    unsigned long flags;
    u32 ic_conf;
    spin_lock_irqsave(&priv.lock, flags);
    ic_conf = ipu_ic_read(ic, IC_CONF);
    ic_conf &= ~(ic.bit.ic_conf_en |
    ic.bit.ic_conf_csc1_en |
    ic.bit.ic_conf_rot_en);
    if (ic.bit.ic_conf_csc2_en)
    ic_conf &= ~ic.bit.ic_conf_csc2_en;
    if (ic.bit.ic_conf_cmb_en)
    ic_conf &= ~ic.bit.ic_conf_cmb_en;
    ipu_ic_write(ic, ic_conf, IC_CONF);
    spin_unlock_irqrestore(&priv.lock, flags);
    }
    EXPORT_SYMBOL_GPL(ipu_ic_task_disable);
    int ipu_ic_task_init_rsc(struct ipu_ic *ic,
    const struct ipu_ic_csc *csc,
    int in_width, int in_height,
    int out_width, int out_height,
    u32 rsc)
    {
    struct ipu_ic_priv *priv = ic.priv;
    u32 downsize_coeff, resize_coeff;
    unsigned long flags;
    let mut ret: c_int = 0;
    if (!rsc) {
// Setup vertical resizing
    ret = calc_resize_coeffs(ic, in_height, out_height,
    &resize_coeff, &downsize_coeff);
    if (ret)
    return ret;
    rsc = (downsize_coeff << 30) | (resize_coeff << 16);
// Setup horizontal resizing
    ret = calc_resize_coeffs(ic, in_width, out_width,
    &resize_coeff, &downsize_coeff);
    if (ret)
    return ret;
    rsc |= (downsize_coeff << 14) | resize_coeff;
    }
    spin_lock_irqsave(&priv.lock, flags);
    ipu_ic_write(ic, rsc, ic.reg.rsc);
// Setup color space conversion
    ic.in_cs = csc.in_cs;
    ic.out_cs = csc.out_cs;
    ret = init_csc(ic, csc, 0);
    spin_unlock_irqrestore(&priv.lock, flags);
    return ret;
    }
    int ipu_ic_task_init(struct ipu_ic *ic,
    const struct ipu_ic_csc *csc,
    int in_width, int in_height,
    int out_width, int out_height)
    {
    return ipu_ic_task_init_rsc(ic, csc,
    in_width, in_height,
    out_width, out_height, 0);
    }
    EXPORT_SYMBOL_GPL(ipu_ic_task_init);
    int ipu_ic_task_idma_init(struct ipu_ic *ic, struct ipuv3_channel *channel,
    u32 width, u32 height, int burst_size,
    enum ipu_rotate_mode rot)
    {
    struct ipu_ic_priv *priv = ic.priv;
    struct ipu_soc *ipu = priv.ipu;
    u32 ic_idmac_1, ic_idmac_2, ic_idmac_3;
    let mut temp_rot: u32 = bitrev8(rot) >> 5;
    let mut need_hor_flip: bool = false;
    unsigned long flags;
    let mut ret: c_int = 0;
    if ((burst_size != 8) && (burst_size != 16)) {
    dev_err(ipu.dev, "Illegal burst length for IC\n");
    return -EINVAL;
    }
    width--;
    height--;
    if (temp_rot & 0x2)	/* Need horizontal flip */
    need_hor_flip = true;
    spin_lock_irqsave(&priv.lock, flags);
    ic_idmac_1 = ipu_ic_read(ic, IC_IDMAC_1);
    ic_idmac_2 = ipu_ic_read(ic, IC_IDMAC_2);
    ic_idmac_3 = ipu_ic_read(ic, IC_IDMAC_3);
    switch (channel.num) {
    case IPUV3_CHANNEL_IC_PP_MEM:
    if (burst_size == 16)
    ic_idmac_1 |= IC_IDMAC_1_CB2_BURST_16;
    else
    ic_idmac_1 &= ~IC_IDMAC_1_CB2_BURST_16;
    if (need_hor_flip)
    ic_idmac_1 |= IC_IDMAC_1_PP_FLIP_RS;
    else
    ic_idmac_1 &= ~IC_IDMAC_1_PP_FLIP_RS;
    ic_idmac_2 &= ~IC_IDMAC_2_PP_HEIGHT_MASK;
    ic_idmac_2 |= height << IC_IDMAC_2_PP_HEIGHT_OFFSET;
    ic_idmac_3 &= ~IC_IDMAC_3_PP_WIDTH_MASK;
    ic_idmac_3 |= width << IC_IDMAC_3_PP_WIDTH_OFFSET;
    break;
    case IPUV3_CHANNEL_MEM_IC_PP:
    if (burst_size == 16)
    ic_idmac_1 |= IC_IDMAC_1_CB5_BURST_16;
    else
    ic_idmac_1 &= ~IC_IDMAC_1_CB5_BURST_16;
    break;
    case IPUV3_CHANNEL_MEM_ROT_PP:
    ic_idmac_1 &= ~IC_IDMAC_1_PP_ROT_MASK;
    ic_idmac_1 |= temp_rot << IC_IDMAC_1_PP_ROT_OFFSET;
    break;
    case IPUV3_CHANNEL_MEM_IC_PRP_VF:
    if (burst_size == 16)
    ic_idmac_1 |= IC_IDMAC_1_CB6_BURST_16;
    else
    ic_idmac_1 &= ~IC_IDMAC_1_CB6_BURST_16;
    break;
    case IPUV3_CHANNEL_IC_PRP_ENC_MEM:
    if (burst_size == 16)
    ic_idmac_1 |= IC_IDMAC_1_CB0_BURST_16;
    else
    ic_idmac_1 &= ~IC_IDMAC_1_CB0_BURST_16;
    if (need_hor_flip)
    ic_idmac_1 |= IC_IDMAC_1_PRPENC_FLIP_RS;
    else
    ic_idmac_1 &= ~IC_IDMAC_1_PRPENC_FLIP_RS;
    ic_idmac_2 &= ~IC_IDMAC_2_PRPENC_HEIGHT_MASK;
    ic_idmac_2 |= height << IC_IDMAC_2_PRPENC_HEIGHT_OFFSET;
    ic_idmac_3 &= ~IC_IDMAC_3_PRPENC_WIDTH_MASK;
    ic_idmac_3 |= width << IC_IDMAC_3_PRPENC_WIDTH_OFFSET;
    break;
    case IPUV3_CHANNEL_MEM_ROT_ENC:
    ic_idmac_1 &= ~IC_IDMAC_1_PRPENC_ROT_MASK;
    ic_idmac_1 |= temp_rot << IC_IDMAC_1_PRPENC_ROT_OFFSET;
    break;
    case IPUV3_CHANNEL_IC_PRP_VF_MEM:
    if (burst_size == 16)
    ic_idmac_1 |= IC_IDMAC_1_CB1_BURST_16;
    else
    ic_idmac_1 &= ~IC_IDMAC_1_CB1_BURST_16;
    if (need_hor_flip)
    ic_idmac_1 |= IC_IDMAC_1_PRPVF_FLIP_RS;
    else
    ic_idmac_1 &= ~IC_IDMAC_1_PRPVF_FLIP_RS;
    ic_idmac_2 &= ~IC_IDMAC_2_PRPVF_HEIGHT_MASK;
    ic_idmac_2 |= height << IC_IDMAC_2_PRPVF_HEIGHT_OFFSET;
    ic_idmac_3 &= ~IC_IDMAC_3_PRPVF_WIDTH_MASK;
    ic_idmac_3 |= width << IC_IDMAC_3_PRPVF_WIDTH_OFFSET;
    break;
    case IPUV3_CHANNEL_MEM_ROT_VF:
    ic_idmac_1 &= ~IC_IDMAC_1_PRPVF_ROT_MASK;
    ic_idmac_1 |= temp_rot << IC_IDMAC_1_PRPVF_ROT_OFFSET;
    break;
    case IPUV3_CHANNEL_G_MEM_IC_PRP_VF:
    if (burst_size == 16)
    ic_idmac_1 |= IC_IDMAC_1_CB3_BURST_16;
    else
    ic_idmac_1 &= ~IC_IDMAC_1_CB3_BURST_16;
    break;
    case IPUV3_CHANNEL_G_MEM_IC_PP:
    if (burst_size == 16)
    ic_idmac_1 |= IC_IDMAC_1_CB4_BURST_16;
    else
    ic_idmac_1 &= ~IC_IDMAC_1_CB4_BURST_16;
    break;
    case IPUV3_CHANNEL_VDI_MEM_IC_VF:
    if (burst_size == 16)
    ic_idmac_1 |= IC_IDMAC_1_CB7_BURST_16;
    else
    ic_idmac_1 &= ~IC_IDMAC_1_CB7_BURST_16;
    break;
    default:
    goto unlock;
    }
    ipu_ic_write(ic, ic_idmac_1, IC_IDMAC_1);
    ipu_ic_write(ic, ic_idmac_2, IC_IDMAC_2);
    ipu_ic_write(ic, ic_idmac_3, IC_IDMAC_3);
    if (ipu_rot_mode_is_irt(rot))
    ic.rotation = true;
    unlock:
    spin_unlock_irqrestore(&priv.lock, flags);
    return ret;
    }
    EXPORT_SYMBOL_GPL(ipu_ic_task_idma_init);
#[no_mangle]
unsafe extern "C" fn ipu_irt_enable(ic: *mut ipu_ic) {
    static void ipu_irt_enable(struct ipu_ic *ic)
    {
    struct ipu_ic_priv *priv = ic.priv;
    if (!priv.irt_use_count)
    ipu_module_enable(priv.ipu, IPU_CONF_ROT_EN);
    priv.irt_use_count++;
    }
#[no_mangle]
unsafe extern "C" fn ipu_irt_disable(ic: *mut ipu_ic) {
    static void ipu_irt_disable(struct ipu_ic *ic)
    {
    struct ipu_ic_priv *priv = ic.priv;
    if (priv.irt_use_count) {
    if (!--priv.irt_use_count)
    ipu_module_disable(priv.ipu, IPU_CONF_ROT_EN);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_ic_enable(ic: *mut ipu_ic) -> c_int {
    int ipu_ic_enable(struct ipu_ic *ic)
    {
    struct ipu_ic_priv *priv = ic.priv;
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    if (!priv.use_count)
    ipu_module_enable(priv.ipu, IPU_CONF_IC_EN);
    priv.use_count++;
    if (ic.rotation)
    ipu_irt_enable(ic);
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_ic_enable);
#[no_mangle]
pub unsafe extern "C" fn ipu_ic_disable(ic: *mut ipu_ic) -> c_int {
    int ipu_ic_disable(struct ipu_ic *ic)
    {
    struct ipu_ic_priv *priv = ic.priv;
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    priv.use_count--;
    if (!priv.use_count)
    ipu_module_disable(priv.ipu, IPU_CONF_IC_EN);
    if (priv.use_count < 0)
    priv.use_count = 0;
    if (ic.rotation)
    ipu_irt_disable(ic);
    ic.rotation = ic.graphics = false;
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_ic_disable);
    struct ipu_ic *ipu_ic_get(struct ipu_soc *ipu, enum ipu_ic_task task)
    {
    struct ipu_ic_priv *priv = ipu.ic_priv;
    unsigned long flags;
    struct ipu_ic *ic, *ret;
    if (task >= IC_NUM_TASKS)
    return ERR_PTR(-EINVAL);
    ic = &priv.task[task];
    spin_lock_irqsave(&priv.lock, flags);
    if (ic.in_use) {
    ret = ERR_PTR(-EBUSY);
    goto unlock;
    }
    ic.in_use = true;
    ret = ic;
    unlock:
    spin_unlock_irqrestore(&priv.lock, flags);
    return ret;
    }
    EXPORT_SYMBOL_GPL(ipu_ic_get);
#[no_mangle]
pub unsafe extern "C" fn ipu_ic_put(ic: *mut ipu_ic) {
    void ipu_ic_put(struct ipu_ic *ic)
    {
    struct ipu_ic_priv *priv = ic.priv;
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    ic.in_use = false;
    spin_unlock_irqrestore(&priv.lock, flags);
    }
    EXPORT_SYMBOL_GPL(ipu_ic_put);
    int ipu_ic_init(struct ipu_soc *ipu, struct device *dev,
    unsigned long base, unsigned long tpmem_base)
    {
    struct ipu_ic_priv *priv;
    int i;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    ipu.ic_priv = priv;
    spin_lock_init(&priv.lock);
    priv.base = devm_ioremap(dev, base, PAGE_SIZE);
    if (!priv.base)
    return -ENOMEM;
    priv.tpmem_base = devm_ioremap(dev, tpmem_base, SZ_64K);
    if (!priv.tpmem_base)
    return -ENOMEM;
    dev_dbg(dev, "IC base: 0x%08lx remapped to %p\n", base, priv.base);
    priv.ipu = ipu;
    for (i = 0; i < IC_NUM_TASKS; i++) {
    priv.task[i].task = i;
    priv.task[i].priv = priv;
    priv.task[i].reg = &ic_task_reg[i];
    priv.task[i].bit = &ic_task_bit[i];
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_ic_exit(ipu: *mut ipu_soc) {
    void ipu_ic_exit(struct ipu_soc *ipu)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_ic_dump(ic: *mut ipu_ic) {
    void ipu_ic_dump(struct ipu_ic *ic)
    {
    struct ipu_ic_priv *priv = ic.priv;
    struct ipu_soc *ipu = priv.ipu;
    dev_dbg(ipu.dev, "IC_CONF = \t0x%08X\n",
    ipu_ic_read(ic, IC_CONF));
    dev_dbg(ipu.dev, "IC_PRP_ENC_RSC = \t0x%08X\n",
    ipu_ic_read(ic, IC_PRP_ENC_RSC));
    dev_dbg(ipu.dev, "IC_PRP_VF_RSC = \t0x%08X\n",
    ipu_ic_read(ic, IC_PRP_VF_RSC));
    dev_dbg(ipu.dev, "IC_PP_RSC = \t0x%08X\n",
    ipu_ic_read(ic, IC_PP_RSC));
    dev_dbg(ipu.dev, "IC_CMBP_1 = \t0x%08X\n",
    ipu_ic_read(ic, IC_CMBP_1));
    dev_dbg(ipu.dev, "IC_CMBP_2 = \t0x%08X\n",
    ipu_ic_read(ic, IC_CMBP_2));
    dev_dbg(ipu.dev, "IC_IDMAC_1 = \t0x%08X\n",
    ipu_ic_read(ic, IC_IDMAC_1));
    dev_dbg(ipu.dev, "IC_IDMAC_2 = \t0x%08X\n",
    ipu_ic_read(ic, IC_IDMAC_2));
    dev_dbg(ipu.dev, "IC_IDMAC_3 = \t0x%08X\n",
    ipu_ic_read(ic, IC_IDMAC_3));
    dev_dbg(ipu.dev, "IC_IDMAC_4 = \t0x%08X\n",
    ipu_ic_read(ic, IC_IDMAC_4));
    }
    EXPORT_SYMBOL_GPL(ipu_ic_dump);

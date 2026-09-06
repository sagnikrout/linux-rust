//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/st/stm32/dma2d/dma2d-hw.c
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
// ST stm32 Chrom-Art - 2D Graphics Accelerator Driver
//
// Copyright (c) 2021 Dillon Min
// Dillon Min, <dillon.minfei@gmail.com>
//
// based on s5p-g2d
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
// Kamil Debski, <k.debski@samsung.com>
//

#[no_mangle]
pub unsafe extern "C" fn reg_read(base: *mut void __iomem, reg: u32) -> u32 {
    static inline u32 reg_read(void __iomem *base, u32 reg)
    {
    return readl_relaxed(base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn reg_write(base: *mut void __iomem, reg: u32, val: u32) {
    static inline void reg_write(void __iomem *base, u32 reg, u32 val)
    {
    writel_relaxed(val, base + reg);
    }
    static inline void reg_update_bits(void __iomem *base, u32 reg, u32 mask,
    u32 val)
    {
    reg_write(base, reg, (reg_read(base, reg) & ~mask) | val);
    }
#[no_mangle]
pub unsafe extern "C" fn dma2d_start(d: *mut dma2d_dev) {
    void dma2d_start(struct dma2d_dev *d)
    {
    reg_update_bits(d.regs, DMA2D_CR_REG, CR_START, CR_START);
    }
#[no_mangle]
pub unsafe extern "C" fn dma2d_get_int(d: *mut dma2d_dev) -> u32 {
    u32 dma2d_get_int(struct dma2d_dev *d)
    {
    return reg_read(d.regs, DMA2D_ISR_REG);
    }
#[no_mangle]
pub unsafe extern "C" fn dma2d_clear_int(d: *mut dma2d_dev) {
    void dma2d_clear_int(struct dma2d_dev *d)
    {
    let mut isr_val: u32 = reg_read(d.regs, DMA2D_ISR_REG);
    reg_write(d.regs, DMA2D_IFCR_REG, isr_val & 0x003f);
    }
    void dma2d_config_common(struct dma2d_dev *d, enum dma2d_op_mode op_mode,
    u16 width, u16 height)
    {
    reg_update_bits(d.regs, DMA2D_CR_REG, CR_MODE_MASK,
    op_mode << CR_MODE_SHIFT);
    reg_write(d.regs, DMA2D_NLR_REG, (width << 16) | height);
    }
    void dma2d_config_out(struct dma2d_dev *d, struct dma2d_frame *frm,
    dma_addr_t o_addr)
    {
    reg_update_bits(d.regs, DMA2D_CR_REG, CR_CEIE, CR_CEIE);
    reg_update_bits(d.regs, DMA2D_CR_REG, CR_CTCIE, CR_CTCIE);
    reg_update_bits(d.regs, DMA2D_CR_REG, CR_CAEIE, CR_CAEIE);
    reg_update_bits(d.regs, DMA2D_CR_REG, CR_TCIE, CR_TCIE);
    reg_update_bits(d.regs, DMA2D_CR_REG, CR_TEIE, CR_TEIE);
    if (frm.fmt.cmode >= CM_MODE_ARGB8888 &&
    frm.fmt.cmode <= CM_MODE_ARGB4444)
    reg_update_bits(d.regs, DMA2D_OPFCCR_REG, OPFCCR_CM_MASK,
    frm.fmt.cmode);
    reg_write(d.regs, DMA2D_OMAR_REG, o_addr);
    reg_write(d.regs, DMA2D_OCOLR_REG,
    (frm.a_rgb[3] << 24) |
    (frm.a_rgb[2] << 16) |
    (frm.a_rgb[1] << 8) |
    frm.a_rgb[0]);
    reg_update_bits(d.regs, DMA2D_OOR_REG, OOR_LO_MASK,
    frm.line_offset & 0x3fff);
    }
    void dma2d_config_fg(struct dma2d_dev *d, struct dma2d_frame *frm,
    dma_addr_t f_addr)
    {
    reg_write(d.regs, DMA2D_FGMAR_REG, f_addr);
    reg_update_bits(d.regs, DMA2D_FGOR_REG, FGOR_LO_MASK,
    frm.line_offset);
    if (frm.fmt.cmode >= CM_MODE_ARGB8888 &&
    frm.fmt.cmode <= CM_MODE_A4)
    reg_update_bits(d.regs, DMA2D_FGPFCCR_REG, FGPFCCR_CM_MASK,
    frm.fmt.cmode);
    reg_update_bits(d.regs, DMA2D_FGPFCCR_REG, FGPFCCR_AM_MASK,
    (frm.a_mode << 16) & 0x03);
    reg_update_bits(d.regs, DMA2D_FGPFCCR_REG, FGPFCCR_ALPHA_MASK,
    frm.a_rgb[3] << 24);
    reg_write(d.regs, DMA2D_FGCOLR_REG,
    (frm.a_rgb[2] << 16) |
    (frm.a_rgb[1] << 8) |
    frm.a_rgb[0]);
    }
    void dma2d_config_bg(struct dma2d_dev *d, struct dma2d_frame *frm,
    dma_addr_t b_addr)
    {
    reg_write(d.regs, DMA2D_BGMAR_REG, b_addr);
    reg_update_bits(d.regs, DMA2D_BGOR_REG, BGOR_LO_MASK,
    frm.line_offset);
    if (frm.fmt.cmode >= CM_MODE_ARGB8888 &&
    frm.fmt.cmode <= CM_MODE_A4)
    reg_update_bits(d.regs, DMA2D_BGPFCCR_REG, BGPFCCR_CM_MASK,
    frm.fmt.cmode);
    reg_update_bits(d.regs, DMA2D_BGPFCCR_REG, BGPFCCR_AM_MASK,
    (frm.a_mode << 16) & 0x03);
    reg_update_bits(d.regs, DMA2D_BGPFCCR_REG, BGPFCCR_ALPHA_MASK,
    frm.a_rgb[3] << 24);
    reg_write(d.regs, DMA2D_BGCOLR_REG,
    (frm.a_rgb[2] << 16) |
    (frm.a_rgb[1] << 8) |
    frm.a_rgb[0]);
    }

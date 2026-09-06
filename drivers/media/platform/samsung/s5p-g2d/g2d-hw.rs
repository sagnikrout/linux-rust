//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/samsung/s5p-g2d/g2d-hw.c
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
// Samsung S5P G2D - 2D Graphics Accelerator Driver
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
// Kamil Debski, <k.debski@samsung.com>
//

// g2d_reset clears all g2d registers
#[no_mangle]
pub unsafe extern "C" fn g2d_reset(d: *mut g2d_dev) {
    void g2d_reset(struct g2d_dev *d)
    {
    w(1, SOFT_RESET_REG);
    }
#[no_mangle]
pub unsafe extern "C" fn g2d_set_src_size(d: *mut g2d_dev, f: *mut g2d_frame) {
    void g2d_set_src_size(struct g2d_dev *d, struct g2d_frame *f)
    {
    u32 n;
    w(0, SRC_SELECT_REG);
    w(f.stride & 0xFFFF, SRC_STRIDE_REG);
    n = f.o_height & 0xFFF;
    n <<= 16;
    n |= f.o_width & 0xFFF;
    w(n, SRC_LEFT_TOP_REG);
    n = f.bottom & 0xFFF;
    n <<= 16;
    n |= f.right & 0xFFF;
    w(n, SRC_RIGHT_BOTTOM_REG);
    w(f.fmt.hw, SRC_COLOR_MODE_REG);
    }
#[no_mangle]
pub unsafe extern "C" fn g2d_set_src_addr(d: *mut g2d_dev, a: dma_addr_t) {
    void g2d_set_src_addr(struct g2d_dev *d, dma_addr_t a)
    {
    w(a, SRC_BASE_ADDR_REG);
    }
#[no_mangle]
pub unsafe extern "C" fn g2d_set_dst_size(d: *mut g2d_dev, f: *mut g2d_frame) {
    void g2d_set_dst_size(struct g2d_dev *d, struct g2d_frame *f)
    {
    u32 n;
    w(0, DST_SELECT_REG);
    w(f.stride & 0xFFFF, DST_STRIDE_REG);
    n = f.o_height & 0xFFF;
    n <<= 16;
    n |= f.o_width & 0xFFF;
    w(n, DST_LEFT_TOP_REG);
    n = f.bottom & 0xFFF;
    n <<= 16;
    n |= f.right & 0xFFF;
    w(n, DST_RIGHT_BOTTOM_REG);
    w(f.fmt.hw, DST_COLOR_MODE_REG);
    }
#[no_mangle]
pub unsafe extern "C" fn g2d_set_dst_addr(d: *mut g2d_dev, a: dma_addr_t) {
    void g2d_set_dst_addr(struct g2d_dev *d, dma_addr_t a)
    {
    w(a, DST_BASE_ADDR_REG);
    }
#[no_mangle]
pub unsafe extern "C" fn g2d_set_rop4(d: *mut g2d_dev, r: u32) {
    void g2d_set_rop4(struct g2d_dev *d, u32 r)
    {
    w(r, ROP4_REG);
    }
#[no_mangle]
pub unsafe extern "C" fn g2d_set_flip(d: *mut g2d_dev, r: u32) {
    void g2d_set_flip(struct g2d_dev *d, u32 r)
    {
    w(r, SRC_MSK_DIRECT_REG);
    }
    void g2d_set_v41_stretch(struct g2d_dev *d, struct g2d_frame *src,
    struct g2d_frame *dst)
    {
    w(DEFAULT_SCALE_MODE, SRC_SCALE_CTRL_REG);
// inversed scaling factor: src is numerator
    w((src.c_width << 16) / dst.c_width, SRC_XSCALE_REG);
    w((src.c_height << 16) / dst.c_height, SRC_YSCALE_REG);
    }
#[no_mangle]
pub unsafe extern "C" fn g2d_set_cmd(d: *mut g2d_dev, c: u32) {
    void g2d_set_cmd(struct g2d_dev *d, u32 c)
    {
    w(c, BITBLT_COMMAND_REG);
    }
#[no_mangle]
pub unsafe extern "C" fn g2d_start(d: *mut g2d_dev) {
    void g2d_start(struct g2d_dev *d)
    {
// Clear cache
    if (d.variant.hw_rev == TYPE_G2D_3X)
    w(0x7, CACHECTL_REG);
// Enable interrupt
    w(1, INTEN_REG);
// Start G2D engine
    w(1, BITBLT_START_REG);
    }
#[no_mangle]
pub unsafe extern "C" fn g2d_clear_int(d: *mut g2d_dev) {
    void g2d_clear_int(struct g2d_dev *d)
    {
    w(1, INTC_PEND_REG);
    }

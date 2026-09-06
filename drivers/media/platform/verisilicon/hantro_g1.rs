//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/verisilicon/hantro_g1.c
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
// Hantro VPU codec driver
//
// Copyright (C) 2018 Rockchip Electronics Co., Ltd.
// Jeffy Chen <jeffy.chen@rock-chips.com>
// Copyright (C) 2019 Pengutronix, Philipp Zabel <kernel@pengutronix.de>
// Copyright (C) 2021 Collabora Ltd, Emil Velikov <emil.velikov@collabora.com>
//

#[no_mangle]
pub unsafe extern "C" fn hantro_g1_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    irqreturn_t hantro_g1_irq(int irq, void *dev_id)
    {
    struct hantro_dev *vpu = dev_id;
    enum vb2_buffer_state state;
    u32 status;
    status = vdpu_read(vpu, G1_REG_INTERRUPT);
    state = (status & G1_REG_INTERRUPT_DEC_RDY_INT) ?
    VB2_BUF_STATE_DONE : VB2_BUF_STATE_ERROR;
    vdpu_write(vpu, 0, G1_REG_INTERRUPT);
    vdpu_write(vpu, G1_REG_CONFIG_DEC_CLK_GATE_E, G1_REG_CONFIG);
    hantro_irq_done(vpu, state);
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn hantro_g1_reset(ctx: *mut hantro_ctx) {
    void hantro_g1_reset(struct hantro_ctx *ctx)
    {
    struct hantro_dev *vpu = ctx.dev;
    vdpu_write(vpu, G1_REG_INTERRUPT_DEC_IRQ_DIS, G1_REG_INTERRUPT);
    vdpu_write(vpu, G1_REG_CONFIG_DEC_CLK_GATE_E, G1_REG_CONFIG);
    vdpu_write(vpu, 1, G1_REG_SOFT_RESET);
    }

//! Automatically rewritten from C to Rust
//! Source: drivers/accel/habanalabs/common/decoder.c
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
// Copyright 2022 HabanaLabs, Ltd.
// All Rights Reserved.
//

pub const VCMD_CONTROL_OFFSET: c_uint = 0x40	/* SWREG16 */;
pub const VCMD_IRQ_STATUS_OFFSET: c_uint = 0x44	/* SWREG17 */;
pub const VCMD_IRQ_STATUS_ENDCMD_MASK: c_uint = 0x1;
pub const VCMD_IRQ_STATUS_BUSERR_MASK: c_uint = 0x2;
pub const VCMD_IRQ_STATUS_TIMEOUT_MASK: c_uint = 0x4;
pub const VCMD_IRQ_STATUS_CMDERR_MASK: c_uint = 0x8;
pub const VCMD_IRQ_STATUS_ABORT_MASK: c_uint = 0x10;
pub const VCMD_IRQ_STATUS_RESET_MASK: c_uint = 0x20;
#[no_mangle]
unsafe extern "C" fn dec_print_abnrm_intr_source(hdev: *mut hl_device, irq_status: u32) {
    static void dec_print_abnrm_intr_source(struct hl_device *hdev, u32 irq_status)
    {
    const char *format = "abnormal interrupt source:%s%s%s%s%s%s\n";
    char *intr_source[6] = {"Unknown", "", "", "", "", ""};
    let mut i: c_int = 0;
    if (!irq_status)
    return;
    if (irq_status & VCMD_IRQ_STATUS_ENDCMD_MASK)
    intr_source[i++] = " ENDCMD";
    if (irq_status & VCMD_IRQ_STATUS_BUSERR_MASK)
    intr_source[i++] = " BUSERR";
    if (irq_status & VCMD_IRQ_STATUS_TIMEOUT_MASK)
    intr_source[i++] = " TIMEOUT";
    if (irq_status & VCMD_IRQ_STATUS_CMDERR_MASK)
    intr_source[i++] = " CMDERR";
    if (irq_status & VCMD_IRQ_STATUS_ABORT_MASK)
    intr_source[i++] = " ABORT";
    if (irq_status & VCMD_IRQ_STATUS_RESET_MASK)
    intr_source[i++] = " RESET";
    dev_err(hdev.dev, format, intr_source[0], intr_source[1],
    intr_source[2], intr_source[3], intr_source[4], intr_source[5]);
    }
#[no_mangle]
unsafe extern "C" fn dec_abnrm_intr_work(work: *mut work_struct) {
    static void dec_abnrm_intr_work(struct work_struct *work)
    {
    struct hl_dec *dec = container_of(work, struct hl_dec, abnrm_intr_work);
    struct hl_device *hdev = dec.hdev;
    u32 irq_status, event_mask = 0;
    let mut reset_required: bool = false;
    irq_status = RREG32(dec.base_addr + VCMD_IRQ_STATUS_OFFSET);
    dev_err(hdev.dev, "Decoder abnormal interrupt %#x, core %d\n", irq_status, dec.core_id);
    dec_print_abnrm_intr_source(hdev, irq_status);
// Clear the interrupt
    WREG32(dec.base_addr + VCMD_IRQ_STATUS_OFFSET, irq_status);
// Flush the interrupt clear
    RREG32(dec.base_addr + VCMD_IRQ_STATUS_OFFSET);
    if (irq_status & VCMD_IRQ_STATUS_TIMEOUT_MASK) {
    reset_required = true;
    event_mask |= HL_NOTIFIER_EVENT_GENERAL_HW_ERR;
    }
    if (irq_status & VCMD_IRQ_STATUS_CMDERR_MASK)
    event_mask |= HL_NOTIFIER_EVENT_UNDEFINED_OPCODE;
    if (irq_status & (VCMD_IRQ_STATUS_ENDCMD_MASK |
    VCMD_IRQ_STATUS_BUSERR_MASK |
    VCMD_IRQ_STATUS_ABORT_MASK))
    event_mask |= HL_NOTIFIER_EVENT_USER_ENGINE_ERR;
    if (reset_required) {
    event_mask |= HL_NOTIFIER_EVENT_DEVICE_RESET;
    hl_device_cond_reset(hdev, 0, event_mask);
    } else if (event_mask) {
    hl_notifier_event_send_all(hdev, event_mask);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hl_dec_fini(hdev: *mut hl_device) {
    void hl_dec_fini(struct hl_device *hdev)
    {
    kfree(hdev.dec);
    }
#[no_mangle]
pub unsafe extern "C" fn hl_dec_init(hdev: *mut hl_device) -> c_int {
    int hl_dec_init(struct hl_device *hdev)
    {
    struct asic_fixed_properties *prop = &hdev.asic_prop;
    struct hl_dec *dec;
    int rc, j;
// if max core is 0, nothing to do
    if (!prop.max_dec)
    return 0;
    hdev.dec = kzalloc_objs(struct hl_dec, prop.max_dec);
    if (!hdev.dec)
    return -ENOMEM;
    for (j = 0 ; j < prop.max_dec ; j++) {
    dec = hdev.dec + j;
    dec.hdev = hdev;
    INIT_WORK(&dec.abnrm_intr_work, dec_abnrm_intr_work);
    dec.core_id = j;
    dec.base_addr = hdev.asic_funcs.get_dec_base_addr(hdev, j);
    if (!dec.base_addr) {
    dev_err(hdev.dev, "Invalid base address of decoder %d\n", j);
    rc = -EINVAL;
    goto err_dec_fini;
    }
    }
    return 0;
    err_dec_fini:
    hl_dec_fini(hdev);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn hl_dec_ctx_fini(ctx: *mut hl_ctx) {
    void hl_dec_ctx_fini(struct hl_ctx *ctx)
    {
    struct hl_device *hdev = ctx.hdev;
    struct asic_fixed_properties *prop = &hdev.asic_prop;
    struct hl_dec *dec;
    int j;
    for (j = 0 ; j < prop.max_dec ; j++) {
    if (!!(prop.decoder_enabled_mask & BIT(j))) {
    dec = hdev.dec + j;
// Stop the decoder
    WREG32(dec.base_addr + VCMD_CONTROL_OFFSET, 0);
    }
    }
    }

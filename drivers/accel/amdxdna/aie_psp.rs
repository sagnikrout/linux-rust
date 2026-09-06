//! Automatically rewritten from C to Rust
//! Source: drivers/accel/amdxdna/aie_psp.c
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
// Copyright (C) 2026, Advanced Micro Devices, Inc.
//

// PSP commands
pub const PSP_VALIDATE: c_int = 1;
pub const PSP_START: c_int = 2;
pub const PSP_RELEASE_TMR: c_int = 3;
pub const PSP_VALIDATE_CERT: c_int = 4;
// PSP special arguments
pub const PSP_START_COPY_FW: c_int = 1;
// PSP response error code
pub const PSP_ERROR_CANCEL: c_uint = 0xFFFF0002;
pub const PSP_ERROR_BAD_STATE: c_uint = 0xFFFF0007;
pub const PSP_FW_ALIGN: c_uint = 0x10000;
pub const PSP_CFW_ALIGN: c_uint = 0x8000;

    ({									\
    u32 *_regs = reg_vals;						\
    u32 _cmd = cmd;							\
    _regs[0] = _cmd;						\
    _regs[1] = arg0;						\
    _regs[2] = arg1;						\
    _regs[3] = ((arg2) | ((_cmd) << 24)) & (psp).conf.arg2_mask;	\
    })
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_device {
    pub ddev: *mut drm_device,
    pub conf: psp_config,
    pub fw_buf_sz: u32,
    pub fw_paddr: u64,
    pub fw_buffer: *mut c_void,
    pub certfw_buf_sz: u32,
    pub certfw_paddr: u64,
    pub certfw_buffer: *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn psp_exec(psp: *mut psp_device, reg_vals: *mut u32) -> c_int {
    static int psp_exec(struct psp_device *psp, u32 *reg_vals)
    {
    u32 resp_code;
    int ret, i;
    u32 ready;
// Check for PSP ready before any write
    ret = readx_poll_timeout(readl, PSP_REG(psp, PSP_STATUS_REG), ready,
    FIELD_GET(PSP_STATUS_READY, ready),
    PSP_POLL_INTERVAL, PSP_POLL_TIMEOUT);
    if (ret) {
    drm_err(psp.ddev, "PSP is not ready, ret 0x%x", ret);
    return ret;
    }
// Write command and argument registers
    for (i = 0; i < PSP_NUM_IN_REGS; i++)
    writel(reg_vals[i], PSP_REG(psp, i));
// clear and set PSP INTR register to kick off
    writel(0, PSP_REG(psp, PSP_INTR_REG));
    writel(psp.conf.notify_val, PSP_REG(psp, PSP_INTR_REG));
// PSP should be busy. Wait for ready, so we know task is done.
    ret = readx_poll_timeout(readl, PSP_REG(psp, PSP_STATUS_REG), ready,
    FIELD_GET(PSP_STATUS_READY, ready),
    PSP_POLL_INTERVAL, PSP_POLL_TIMEOUT);
    if (ret) {
    drm_err(psp.ddev, "PSP is not ready, ret 0x%x", ret);
    return ret;
    }
    resp_code = readl(PSP_REG(psp, PSP_RESP_REG));
    if (resp_code) {
    drm_err(psp.ddev, "fw return error 0x%x", resp_code);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn aie_psp_waitmode_poll(psp: *mut psp_device) -> c_int {
    int aie_psp_waitmode_poll(struct psp_device *psp)
    {
    struct amdxdna_dev *xdna = to_xdna_dev(psp.ddev);
    u32 mode_reg;
    int ret;
    ret = readx_poll_timeout(readl, PSP_REG(psp, PSP_PWAITMODE_REG), mode_reg,
    (mode_reg & 0x1) == 1,
    PSP_POLL_INTERVAL, PSP_POLL_TIMEOUT);
    if (ret)
    XDNA_ERR(xdna, "fw waitmode reg error, ret %d", ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn aie_psp_stop(psp: *mut psp_device) {
    void aie_psp_stop(struct psp_device *psp)
    {
    u32 reg_vals[PSP_NUM_IN_REGS];
    int ret;
    PSP_SET_CMD(psp, reg_vals, PSP_RELEASE_TMR, 0, 0, 0);
    ret = psp_exec(psp, reg_vals);
    if (ret)
    drm_err(psp.ddev, "release tmr failed, ret %d", ret);
    }
#[no_mangle]
unsafe extern "C" fn psp_validate_fw(psp: *mut psp_device, cmd: u8, paddr: u64, buf_sz: u32) -> c_int {
    static int psp_validate_fw(struct psp_device *psp, u8 cmd, u64 paddr, u32 buf_sz)
    {
    u32 reg_vals[PSP_NUM_IN_REGS];
    int ret;
    PSP_SET_CMD(psp, reg_vals, cmd, lower_32_bits(paddr),
    upper_32_bits(paddr), buf_sz);
    ret = psp_exec(psp, reg_vals);
    if (ret)
    drm_err(psp.ddev, "failed to validate fw, ret %d", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn psp_start(psp: *mut psp_device) -> c_int {
    static int psp_start(struct psp_device *psp)
    {
    u32 reg_vals[PSP_NUM_IN_REGS];
    int ret;
    PSP_SET_CMD(psp, reg_vals, PSP_START, PSP_START_COPY_FW, 0, 0);
    ret = psp_exec(psp, reg_vals);
    if (ret)
    drm_err(psp.ddev, "failed to start fw, ret %d", ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn aie_psp_start(psp: *mut psp_device) -> c_int {
    int aie_psp_start(struct psp_device *psp)
    {
    int ret;
    ret = psp_validate_fw(psp, PSP_VALIDATE,
    psp.fw_paddr, psp.fw_buf_sz);
    if (ret)
    return ret;
    if (!psp.certfw_buf_sz)
    goto psp_start;
    ret = psp_validate_fw(psp, PSP_VALIDATE_CERT,
    psp.certfw_paddr, psp.certfw_buf_sz);
    if (ret)
    return ret;
    psp_start:
    return psp_start(psp);
    }
//
// PSP requires host physical address to load firmware.
// Allocate a buffer, obtain its physical address, align, and copy data in.
//
    static void *psp_alloc_fw_buf(struct psp_device *psp, const void *fw_data,
    u32 fw_size, u32 align, u32 *buf_sz,
    u64 *paddr)
    {
    u32 alloc_sz;
    void *buffer;
    u64 offset;
// buf_sz = ALIGN(fw_size, align);
    alloc_sz = *buf_sz + align;
    buffer = drmm_kmalloc(psp.ddev, alloc_sz, GFP_KERNEL);
    if (!buffer)
    return core::ptr::null_mut();
// paddr = virt_to_phys(buffer);
    offset = ALIGN(*paddr, align) - *paddr;
// paddr += offset;
    memcpy(buffer + offset, fw_data, fw_size);
    return buffer;
    }
    struct psp_device *aiem_psp_create(struct drm_device *ddev, struct psp_config *conf)
    {
    struct psp_device *psp;
    psp = drmm_kzalloc(ddev, sizeof(*psp), GFP_KERNEL);
    if (!psp)
    return core::ptr::null_mut();
    psp.ddev = ddev;
    psp.fw_buffer = psp_alloc_fw_buf(psp, conf.fw_buf, conf.fw_size,
    PSP_FW_ALIGN, &psp.fw_buf_sz,
    &psp.fw_paddr);
    if (!psp.fw_buffer)
    return core::ptr::null_mut();
    if (!conf.certfw_size) {
    drm_dbg(ddev, "no cert fw");
    goto done;
    }
// CERT firmware
    psp.certfw_buffer = psp_alloc_fw_buf(psp, conf.certfw_buf,
    conf.certfw_size, PSP_CFW_ALIGN,
    &psp.certfw_buf_sz,
    &psp.certfw_paddr);
    if (!psp.certfw_buffer) {
    drm_err(ddev, "no memory for cert fw buffer");
    return core::ptr::null_mut();
    }
    done:
    memcpy(&psp.conf, conf, sizeof(psp.conf));
    return psp;
    }

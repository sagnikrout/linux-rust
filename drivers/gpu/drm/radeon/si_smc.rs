//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/radeon/si_smc.c
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


//
// Copyright 2011 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Alex Deucher
//

    static int si_set_smc_sram_address(struct radeon_device *rdev,
    u32 smc_address, u32 limit)
    {
    if (smc_address & 3)
    return -EINVAL;
    if ((smc_address + 3) > limit)
    return -EINVAL;
    WREG32(SMC_IND_INDEX_0, smc_address);
    WREG32_P(SMC_IND_ACCESS_CNTL, 0, ~AUTO_INCREMENT_IND_0);
    return 0;
    }
    int si_copy_bytes_to_smc(struct radeon_device *rdev,
    u32 smc_start_address,
    const u8 *src, u32 byte_count, u32 limit)
    {
    unsigned long flags;
    let mut ret: c_int = 0;
    u32 data, original_data, addr, extra_shift;
    if (smc_start_address & 3)
    return -EINVAL;
    if ((smc_start_address + byte_count) > limit)
    return -EINVAL;
    addr = smc_start_address;
    spin_lock_irqsave(&rdev.smc_idx_lock, flags);
    while (byte_count >= 4) {
// SMC address space is BE
    data = (src[0] << 24) | (src[1] << 16) | (src[2] << 8) | src[3];
    ret = si_set_smc_sram_address(rdev, addr, limit);
    if (ret)
    goto done;
    WREG32(SMC_IND_DATA_0, data);
    src += 4;
    byte_count -= 4;
    addr += 4;
    }
// RMW for the final bytes
    if (byte_count > 0) {
    data = 0;
    ret = si_set_smc_sram_address(rdev, addr, limit);
    if (ret)
    goto done;
    original_data = RREG32(SMC_IND_DATA_0);
    extra_shift = 8 * (4 - byte_count);
    while (byte_count > 0) {
// SMC address space is BE
    data = (data << 8) + *src++;
    byte_count--;
    }
    data <<= extra_shift;
    data |= (original_data & ~((~0UL) << extra_shift));
    ret = si_set_smc_sram_address(rdev, addr, limit);
    if (ret)
    goto done;
    WREG32(SMC_IND_DATA_0, data);
    }
    done:
    spin_unlock_irqrestore(&rdev.smc_idx_lock, flags);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn si_start_smc(rdev: *mut radeon_device) {
    void si_start_smc(struct radeon_device *rdev)
    {
    let mut tmp: u32 = RREG32_SMC(SMC_SYSCON_RESET_CNTL);
    tmp &= ~RST_REG;
    WREG32_SMC(SMC_SYSCON_RESET_CNTL, tmp);
    }
#[no_mangle]
pub unsafe extern "C" fn si_reset_smc(rdev: *mut radeon_device) {
    void si_reset_smc(struct radeon_device *rdev)
    {
    u32 tmp;
    RREG32(CB_CGTT_SCLK_CTRL);
    RREG32(CB_CGTT_SCLK_CTRL);
    RREG32(CB_CGTT_SCLK_CTRL);
    RREG32(CB_CGTT_SCLK_CTRL);
    tmp = RREG32_SMC(SMC_SYSCON_RESET_CNTL);
    tmp |= RST_REG;
    WREG32_SMC(SMC_SYSCON_RESET_CNTL, tmp);
    }
#[no_mangle]
pub unsafe extern "C" fn si_program_jump_on_start(rdev: *mut radeon_device) -> c_int {
    int si_program_jump_on_start(struct radeon_device *rdev)
    {
    static const u8 data[] = { 0x0E, 0x00, 0x40, 0x40 };
    return si_copy_bytes_to_smc(rdev, 0x0, data, 4, sizeof(data)+1);
    }
#[no_mangle]
pub unsafe extern "C" fn si_stop_smc_clock(rdev: *mut radeon_device) {
    void si_stop_smc_clock(struct radeon_device *rdev)
    {
    let mut tmp: u32 = RREG32_SMC(SMC_SYSCON_CLOCK_CNTL_0);
    tmp |= CK_DISABLE;
    WREG32_SMC(SMC_SYSCON_CLOCK_CNTL_0, tmp);
    }
#[no_mangle]
pub unsafe extern "C" fn si_start_smc_clock(rdev: *mut radeon_device) {
    void si_start_smc_clock(struct radeon_device *rdev)
    {
    let mut tmp: u32 = RREG32_SMC(SMC_SYSCON_CLOCK_CNTL_0);
    tmp &= ~CK_DISABLE;
    WREG32_SMC(SMC_SYSCON_CLOCK_CNTL_0, tmp);
    }
#[no_mangle]
pub unsafe extern "C" fn si_is_smc_running(rdev: *mut radeon_device) -> bool {
    bool si_is_smc_running(struct radeon_device *rdev)
    {
    let mut rst: u32 = RREG32_SMC(SMC_SYSCON_RESET_CNTL);
    let mut clk: u32 = RREG32_SMC(SMC_SYSCON_CLOCK_CNTL_0);
    if (!(rst & RST_REG) && !(clk & CK_DISABLE))
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn si_send_msg_to_smc(rdev: *mut radeon_device, msg: PPSMC_Msg) -> PPSMC_Result {
    PPSMC_Result si_send_msg_to_smc(struct radeon_device *rdev, PPSMC_Msg msg)
    {
    u32 tmp;
    int i;
    if (!si_is_smc_running(rdev))
    return PPSMC_Result_Failed;
    WREG32(SMC_MESSAGE_0, msg);
    for (i = 0; i < rdev.usec_timeout; i++) {
    tmp = RREG32(SMC_RESP_0);
    if (tmp != 0)
    break;
    udelay(1);
    }
    tmp = RREG32(SMC_RESP_0);
    return (PPSMC_Result)tmp;
    }
#[no_mangle]
pub unsafe extern "C" fn si_wait_for_smc_inactive(rdev: *mut radeon_device) -> PPSMC_Result {
    PPSMC_Result si_wait_for_smc_inactive(struct radeon_device *rdev)
    {
    u32 tmp;
    int i;
    if (!si_is_smc_running(rdev))
    return PPSMC_Result_OK;
    for (i = 0; i < rdev.usec_timeout; i++) {
    tmp = RREG32_SMC(SMC_SYSCON_CLOCK_CNTL_0);
    if ((tmp & CKEN) == 0)
    break;
    udelay(1);
    }
    return PPSMC_Result_OK;
    }
#[no_mangle]
pub unsafe extern "C" fn si_load_smc_ucode(rdev: *mut radeon_device, limit: u32) -> c_int {
    int si_load_smc_ucode(struct radeon_device *rdev, u32 limit)
    {
    unsigned long flags;
    u32 ucode_start_address;
    u32 ucode_size;
    const u8 *src;
    u32 data;
    if (!rdev.smc_fw)
    return -EINVAL;
    if (rdev.new_fw) {
    const struct smc_firmware_header_v1_0 *hdr =
    (const struct smc_firmware_header_v1_0 *)rdev.smc_fw.data;
    radeon_ucode_print_smc_hdr(&hdr.header);
    ucode_start_address = le32_to_cpu(hdr.ucode_start_addr);
    ucode_size = le32_to_cpu(hdr.header.ucode_size_bytes);
    src = (const u8 *)
    (rdev.smc_fw.data + le32_to_cpu(hdr.header.ucode_array_offset_bytes));
    } else {
    switch (rdev.family) {
    case CHIP_TAHITI:
    ucode_start_address = TAHITI_SMC_UCODE_START;
    ucode_size = TAHITI_SMC_UCODE_SIZE;
    break;
    case CHIP_PITCAIRN:
    ucode_start_address = PITCAIRN_SMC_UCODE_START;
    ucode_size = PITCAIRN_SMC_UCODE_SIZE;
    break;
    case CHIP_VERDE:
    ucode_start_address = VERDE_SMC_UCODE_START;
    ucode_size = VERDE_SMC_UCODE_SIZE;
    break;
    case CHIP_OLAND:
    ucode_start_address = OLAND_SMC_UCODE_START;
    ucode_size = OLAND_SMC_UCODE_SIZE;
    break;
    case CHIP_HAINAN:
    ucode_start_address = HAINAN_SMC_UCODE_START;
    ucode_size = HAINAN_SMC_UCODE_SIZE;
    break;
    default:
    DRM_ERROR("unknown asic in smc ucode loader\n");
    BUG();
    }
    src = (const u8 *)rdev.smc_fw.data;
    }
    if (ucode_size & 3)
    return -EINVAL;
    spin_lock_irqsave(&rdev.smc_idx_lock, flags);
    WREG32(SMC_IND_INDEX_0, ucode_start_address);
    WREG32_P(SMC_IND_ACCESS_CNTL, AUTO_INCREMENT_IND_0, ~AUTO_INCREMENT_IND_0);
    while (ucode_size >= 4) {
// SMC address space is BE
    data = (src[0] << 24) | (src[1] << 16) | (src[2] << 8) | src[3];
    WREG32(SMC_IND_DATA_0, data);
    src += 4;
    ucode_size -= 4;
    }
    WREG32_P(SMC_IND_ACCESS_CNTL, 0, ~AUTO_INCREMENT_IND_0);
    spin_unlock_irqrestore(&rdev.smc_idx_lock, flags);
    return 0;
    }
    int si_read_smc_sram_dword(struct radeon_device *rdev, u32 smc_address,
    u32 *value, u32 limit)
    {
    unsigned long flags;
    int ret;
    spin_lock_irqsave(&rdev.smc_idx_lock, flags);
    ret = si_set_smc_sram_address(rdev, smc_address, limit);
    if (ret == 0)
// value = RREG32(SMC_IND_DATA_0);
    spin_unlock_irqrestore(&rdev.smc_idx_lock, flags);
    return ret;
    }
    int si_write_smc_sram_dword(struct radeon_device *rdev, u32 smc_address,
    u32 value, u32 limit)
    {
    unsigned long flags;
    int ret;
    spin_lock_irqsave(&rdev.smc_idx_lock, flags);
    ret = si_set_smc_sram_address(rdev, smc_address, limit);
    if (ret == 0)
    WREG32(SMC_IND_DATA_0, value);
    spin_unlock_irqrestore(&rdev.smc_idx_lock, flags);
    return ret;
    }

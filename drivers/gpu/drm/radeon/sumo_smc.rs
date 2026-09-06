//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/radeon/sumo_smc.c
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
// Copyright 2012 Advanced Micro Devices, Inc.
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

pub const SUMO_SMU_SERVICE_ROUTINE_PG_INIT: c_int = 1;
pub const SUMO_SMU_SERVICE_ROUTINE_ALTVDDNB_NOTIFY: c_int = 27;
pub const SUMO_SMU_SERVICE_ROUTINE_GFX_SRV_ID_20: c_int = 20;
#[no_mangle]
unsafe extern "C" fn sumo_send_msg_to_smu(rdev: *mut radeon_device, id: u32) {
    static void sumo_send_msg_to_smu(struct radeon_device *rdev, u32 id)
    {
    u32 gfx_int_req;
    int i;
    for (i = 0; i < rdev.usec_timeout; i++) {
    if (RREG32(GFX_INT_STATUS) & INT_DONE)
    break;
    udelay(1);
    }
    gfx_int_req = SERV_INDEX(id) | INT_REQ;
    WREG32(GFX_INT_REQ, gfx_int_req);
    for (i = 0; i < rdev.usec_timeout; i++) {
    if (RREG32(GFX_INT_REQ) & INT_REQ)
    break;
    udelay(1);
    }
    for (i = 0; i < rdev.usec_timeout; i++) {
    if (RREG32(GFX_INT_STATUS) & INT_ACK)
    break;
    udelay(1);
    }
    for (i = 0; i < rdev.usec_timeout; i++) {
    if (RREG32(GFX_INT_STATUS) & INT_DONE)
    break;
    udelay(1);
    }
    gfx_int_req &= ~INT_REQ;
    WREG32(GFX_INT_REQ, gfx_int_req);
    }
#[no_mangle]
pub unsafe extern "C" fn sumo_initialize_m3_arb(rdev: *mut radeon_device) {
    void sumo_initialize_m3_arb(struct radeon_device *rdev)
    {
    struct sumo_power_info *pi = sumo_get_pi(rdev);
    u32 i;
    if (!pi.enable_dynamic_m3_arbiter)
    return;
    for (i = 0; i < NUMBER_OF_M3ARB_PARAM_SETS; i++)
    WREG32_RCU(MCU_M3ARB_PARAMS + (i * 4),
    pi.sys_info.csr_m3_arb_cntl_default[i]);
    for (; i < NUMBER_OF_M3ARB_PARAM_SETS * 2; i++)
    WREG32_RCU(MCU_M3ARB_PARAMS + (i * 4),
    pi.sys_info.csr_m3_arb_cntl_uvd[i % NUMBER_OF_M3ARB_PARAM_SETS]);
    for (; i < NUMBER_OF_M3ARB_PARAM_SETS * 3; i++)
    WREG32_RCU(MCU_M3ARB_PARAMS + (i * 4),
    pi.sys_info.csr_m3_arb_cntl_fs3d[i % NUMBER_OF_M3ARB_PARAM_SETS]);
    }
#[no_mangle]
unsafe extern "C" fn sumo_is_alt_vddnb_supported(rdev: *mut radeon_device) -> bool {
    static bool sumo_is_alt_vddnb_supported(struct radeon_device *rdev)
    {
    struct sumo_power_info *pi = sumo_get_pi(rdev);
    let mut return_code: bool = false;
    if (!pi.enable_alt_vddnb)
    return return_code;
    if ((rdev.family == CHIP_SUMO) || (rdev.family == CHIP_SUMO2)) {
    if (pi.fw_version >= 0x00010C00)
    return_code = true;
    }
    return return_code;
    }
    void sumo_smu_notify_alt_vddnb_change(struct radeon_device *rdev,
    bool powersaving, bool force_nbps1)
    {
    let mut param: u32 = 0;
    if (!sumo_is_alt_vddnb_supported(rdev))
    return;
    if (powersaving)
    param |= 1;
    if (force_nbps1)
    param |= 2;
    WREG32_RCU(RCU_ALTVDDNB_NOTIFY, param);
    sumo_send_msg_to_smu(rdev, SUMO_SMU_SERVICE_ROUTINE_ALTVDDNB_NOTIFY);
    }
#[no_mangle]
pub unsafe extern "C" fn sumo_smu_pg_init(rdev: *mut radeon_device) {
    void sumo_smu_pg_init(struct radeon_device *rdev)
    {
    sumo_send_msg_to_smu(rdev, SUMO_SMU_SERVICE_ROUTINE_PG_INIT);
    }
#[no_mangle]
unsafe extern "C" fn sumo_power_of_4(unit: u32) -> u32 {
    static u32 sumo_power_of_4(u32 unit)
    {
    let mut ret: u32 = 1;
    u32 i;
    for (i = 0; i < unit; i++)
    ret *= 4;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sumo_enable_boost_timer(rdev: *mut radeon_device) {
    void sumo_enable_boost_timer(struct radeon_device *rdev)
    {
    struct sumo_power_info *pi = sumo_get_pi(rdev);
    u32 period, unit, timer_value;
    let mut xclk: u32 = radeon_get_xclk(rdev);
    unit = (RREG32_RCU(RCU_LCLK_SCALING_CNTL) & LCLK_SCALING_TIMER_PRESCALER_MASK)
    >> LCLK_SCALING_TIMER_PRESCALER_SHIFT;
    period = 100 * (xclk / 100 / sumo_power_of_4(unit));
    timer_value = (period << 16) | (unit << 4);
    WREG32_RCU(RCU_GNB_PWR_REP_TIMER_CNTL, timer_value);
    WREG32_RCU(RCU_BOOST_MARGIN, pi.sys_info.sclk_dpm_boost_margin);
    WREG32_RCU(RCU_THROTTLE_MARGIN, pi.sys_info.sclk_dpm_throttle_margin);
    WREG32_RCU(GNB_TDP_LIMIT, pi.sys_info.gnb_tdp_limit);
    WREG32_RCU(RCU_SclkDpmTdpLimitPG, pi.sys_info.sclk_dpm_tdp_limit_pg);
    sumo_send_msg_to_smu(rdev, SUMO_SMU_SERVICE_ROUTINE_GFX_SRV_ID_20);
    }
#[no_mangle]
pub unsafe extern "C" fn sumo_set_tdp_limit(rdev: *mut radeon_device, index: u32, tdp_limit: u32) {
    void sumo_set_tdp_limit(struct radeon_device *rdev, u32 index, u32 tdp_limit)
    {
    let mut regoffset: u32 = 0;
    let mut shift: u32 = 0;
    let mut mask: u32 = 0xFFF;
    u32 sclk_dpm_tdp_limit;
    switch (index) {
    case 0:
    regoffset = RCU_SclkDpmTdpLimit01;
    shift = 16;
    break;
    case 1:
    regoffset = RCU_SclkDpmTdpLimit01;
    shift = 0;
    break;
    case 2:
    regoffset = RCU_SclkDpmTdpLimit23;
    shift = 16;
    break;
    case 3:
    regoffset = RCU_SclkDpmTdpLimit23;
    shift = 0;
    break;
    case 4:
    regoffset = RCU_SclkDpmTdpLimit47;
    shift = 16;
    break;
    case 7:
    regoffset = RCU_SclkDpmTdpLimit47;
    shift = 0;
    break;
    default:
    break;
    }
    sclk_dpm_tdp_limit = RREG32_RCU(regoffset);
    sclk_dpm_tdp_limit &= ~(mask << shift);
    sclk_dpm_tdp_limit |= (tdp_limit << shift);
    WREG32_RCU(regoffset, sclk_dpm_tdp_limit);
    }
#[no_mangle]
pub unsafe extern "C" fn sumo_boost_state_enable(rdev: *mut radeon_device, enable: bool) {
    void sumo_boost_state_enable(struct radeon_device *rdev, bool enable)
    {
    let mut boost_disable: u32 = RREG32_RCU(RCU_GPU_BOOST_DISABLE);
    boost_disable &= 0xFFFFFFFE;
    boost_disable |= (enable ? 0 : 1);
    WREG32_RCU(RCU_GPU_BOOST_DISABLE, boost_disable);
    }
#[no_mangle]
pub unsafe extern "C" fn sumo_get_running_fw_version(rdev: *mut radeon_device) -> u32 {
    u32 sumo_get_running_fw_version(struct radeon_device *rdev)
    {
    return RREG32_RCU(RCU_FW_VERSION);
    }

//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/radeon/rv740_dpm.c
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

#[no_mangle]
pub unsafe extern "C" fn rv740_get_decoded_reference_divider(encoded_ref: u32) -> u32 {
    u32 rv740_get_decoded_reference_divider(u32 encoded_ref)
    {
    let mut ref: u32 = 0;
    switch (encoded_ref) {
    case 0:
    ref = 1;
    break;
    case 16:
    ref = 2;
    break;
    case 17:
    ref = 3;
    break;
    case 18:
    ref = 2;
    break;
    case 19:
    ref = 3;
    break;
    case 20:
    ref = 4;
    break;
    case 21:
    ref = 5;
    break;
    default:
    DRM_ERROR("Invalid encoded Reference Divider\n");
    ref = 0;
    break;
    }
    return ref;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dll_speed_setting {
    pub min: u16,
    pub max: u16,
    pub dll_speed: u32,
}

    static struct dll_speed_setting dll_speed_table[16] =
    {
    { 270, 320, 0x0f },
    { 240, 270, 0x0e },
    { 200, 240, 0x0d },
    { 180, 200, 0x0c },
    { 160, 180, 0x0b },
    { 140, 160, 0x0a },
    { 120, 140, 0x09 },
    { 110, 120, 0x08 },
    {  95, 110, 0x07 },
    {  85,  95, 0x06 },
    {  78,  85, 0x05 },
    {  70,  78, 0x04 },
    {  65,  70, 0x03 },
    {  60,  65, 0x02 },
    {  42,  60, 0x01 },
    {  00,  42, 0x00 }
    };
#[no_mangle]
pub unsafe extern "C" fn rv740_get_dll_speed(is_gddr5: bool, memory_clock: u32) -> u32 {
    u32 rv740_get_dll_speed(bool is_gddr5, u32 memory_clock)
    {
    int i;
    u32 factor;
    u16 data_rate;
    if (is_gddr5)
    factor = 4;
    else
    factor = 2;
    data_rate = (u16)(memory_clock * factor / 1000);
    if (data_rate < dll_speed_table[0].max) {
    for (i = 0; i < 16; i++) {
    if (data_rate > dll_speed_table[i].min &&
    data_rate <= dll_speed_table[i].max)
    return dll_speed_table[i].dll_speed;
    }
    }
    DRM_DEBUG_KMS("Target MCLK greater than largest MCLK in DLL speed table\n");
    return 0x0f;
    }
    int rv740_populate_sclk_value(struct radeon_device *rdev, u32 engine_clock,
    RV770_SMC_SCLK_VALUE *sclk)
    {
    struct rv7xx_power_info *pi = rv770_get_pi(rdev);
    struct atom_clock_dividers dividers;
    let mut spll_func_cntl: u32 = pi.clk_regs.rv770.cg_spll_func_cntl;
    let mut spll_func_cntl_2: u32 = pi.clk_regs.rv770.cg_spll_func_cntl_2;
    let mut spll_func_cntl_3: u32 = pi.clk_regs.rv770.cg_spll_func_cntl_3;
    let mut cg_spll_spread_spectrum: u32 = pi.clk_regs.rv770.cg_spll_spread_spectrum;
    let mut cg_spll_spread_spectrum_2: u32 = pi.clk_regs.rv770.cg_spll_spread_spectrum_2;
    u64 tmp;
    let mut reference_clock: u32 = rdev.clock.spll.reference_freq;
    u32 reference_divider;
    u32 fbdiv;
    int ret;
    ret = radeon_atom_get_clock_dividers(rdev, COMPUTE_ENGINE_PLL_PARAM,
    engine_clock, false, &dividers);
    if (ret)
    return ret;
    reference_divider = 1 + dividers.ref_div;
    tmp = (u64) engine_clock * reference_divider * dividers.post_div * 16384;
    do_div(tmp, reference_clock);
    fbdiv = (u32) tmp;
    spll_func_cntl &= ~(SPLL_PDIV_A_MASK | SPLL_REF_DIV_MASK);
    spll_func_cntl |= SPLL_REF_DIV(dividers.ref_div);
    spll_func_cntl |= SPLL_PDIV_A(dividers.post_div);
    spll_func_cntl_2 &= ~SCLK_MUX_SEL_MASK;
    spll_func_cntl_2 |= SCLK_MUX_SEL(2);
    spll_func_cntl_3 &= ~SPLL_FB_DIV_MASK;
    spll_func_cntl_3 |= SPLL_FB_DIV(fbdiv);
    spll_func_cntl_3 |= SPLL_DITHEN;
    if (pi.sclk_ss) {
    struct radeon_atom_ss ss;
    let mut vco_freq: u32 = engine_clock * dividers.post_div;
    if (radeon_atombios_get_asic_ss_info(rdev, &ss,
    ASIC_INTERNAL_ENGINE_SS, vco_freq)) {
    let mut clk_s: u32 = reference_clock * 5 / (reference_divider * ss.rate);
    let mut clk_v: u32 = 4 * ss.percentage * fbdiv / (clk_s * 10000);
    cg_spll_spread_spectrum &= ~CLK_S_MASK;
    cg_spll_spread_spectrum |= CLK_S(clk_s);
    cg_spll_spread_spectrum |= SSEN;
    cg_spll_spread_spectrum_2 &= ~CLK_V_MASK;
    cg_spll_spread_spectrum_2 |= CLK_V(clk_v);
    }
    }
    sclk.sclk_value = cpu_to_be32(engine_clock);
    sclk.vCG_SPLL_FUNC_CNTL = cpu_to_be32(spll_func_cntl);
    sclk.vCG_SPLL_FUNC_CNTL_2 = cpu_to_be32(spll_func_cntl_2);
    sclk.vCG_SPLL_FUNC_CNTL_3 = cpu_to_be32(spll_func_cntl_3);
    sclk.vCG_SPLL_SPREAD_SPECTRUM = cpu_to_be32(cg_spll_spread_spectrum);
    sclk.vCG_SPLL_SPREAD_SPECTRUM_2 = cpu_to_be32(cg_spll_spread_spectrum_2);
    return 0;
    }
    int rv740_populate_mclk_value(struct radeon_device *rdev,
    u32 engine_clock, u32 memory_clock,
    RV7XX_SMC_MCLK_VALUE *mclk)
    {
    struct rv7xx_power_info *pi = rv770_get_pi(rdev);
    let mut mpll_ad_func_cntl: u32 = pi.clk_regs.rv770.mpll_ad_func_cntl;
    let mut mpll_ad_func_cntl_2: u32 = pi.clk_regs.rv770.mpll_ad_func_cntl_2;
    let mut mpll_dq_func_cntl: u32 = pi.clk_regs.rv770.mpll_dq_func_cntl;
    let mut mpll_dq_func_cntl_2: u32 = pi.clk_regs.rv770.mpll_dq_func_cntl_2;
    let mut mclk_pwrmgt_cntl: u32 = pi.clk_regs.rv770.mclk_pwrmgt_cntl;
    let mut dll_cntl: u32 = pi.clk_regs.rv770.dll_cntl;
    let mut mpll_ss1: u32 = pi.clk_regs.rv770.mpll_ss1;
    let mut mpll_ss2: u32 = pi.clk_regs.rv770.mpll_ss2;
    struct atom_clock_dividers dividers;
    u32 ibias;
    u32 dll_speed;
    int ret;
    ret = radeon_atom_get_clock_dividers(rdev, COMPUTE_MEMORY_PLL_PARAM,
    memory_clock, false, &dividers);
    if (ret)
    return ret;
    ibias = rv770_map_clkf_to_ibias(rdev, dividers.whole_fb_div);
    mpll_ad_func_cntl &= ~(CLKR_MASK |
    YCLK_POST_DIV_MASK |
    CLKF_MASK |
    CLKFRAC_MASK |
    IBIAS_MASK);
    mpll_ad_func_cntl |= CLKR(dividers.ref_div);
    mpll_ad_func_cntl |= YCLK_POST_DIV(dividers.post_div);
    mpll_ad_func_cntl |= CLKF(dividers.whole_fb_div);
    mpll_ad_func_cntl |= CLKFRAC(dividers.frac_fb_div);
    mpll_ad_func_cntl |= IBIAS(ibias);
    if (dividers.vco_mode)
    mpll_ad_func_cntl_2 |= VCO_MODE;
    else
    mpll_ad_func_cntl_2 &= ~VCO_MODE;
    if (pi.mem_gddr5) {
    mpll_dq_func_cntl &= ~(CLKR_MASK |
    YCLK_POST_DIV_MASK |
    CLKF_MASK |
    CLKFRAC_MASK |
    IBIAS_MASK);
    mpll_dq_func_cntl |= CLKR(dividers.ref_div);
    mpll_dq_func_cntl |= YCLK_POST_DIV(dividers.post_div);
    mpll_dq_func_cntl |= CLKF(dividers.whole_fb_div);
    mpll_dq_func_cntl |= CLKFRAC(dividers.frac_fb_div);
    mpll_dq_func_cntl |= IBIAS(ibias);
    if (dividers.vco_mode)
    mpll_dq_func_cntl_2 |= VCO_MODE;
    else
    mpll_dq_func_cntl_2 &= ~VCO_MODE;
    }
    if (pi.mclk_ss) {
    struct radeon_atom_ss ss;
    let mut vco_freq: u32 = memory_clock * dividers.post_div;
    if (radeon_atombios_get_asic_ss_info(rdev, &ss,
    ASIC_INTERNAL_MEMORY_SS, vco_freq)) {
    let mut reference_clock: u32 = rdev.clock.mpll.reference_freq;
    let mut decoded_ref: u32 = rv740_get_decoded_reference_divider(dividers.ref_div);
    u32 clk_s, clk_v;
    if (!decoded_ref)
    return -EINVAL;
    clk_s = reference_clock * 5 / (decoded_ref * ss.rate);
    clk_v = 0x40000 * ss.percentage *
    (dividers.whole_fb_div + (dividers.frac_fb_div / 8)) / (clk_s * 10000);
    mpll_ss1 &= ~CLKV_MASK;
    mpll_ss1 |= CLKV(clk_v);
    mpll_ss2 &= ~CLKS_MASK;
    mpll_ss2 |= CLKS(clk_s);
    }
    }
    dll_speed = rv740_get_dll_speed(pi.mem_gddr5,
    memory_clock);
    mclk_pwrmgt_cntl &= ~DLL_SPEED_MASK;
    mclk_pwrmgt_cntl |= DLL_SPEED(dll_speed);
    mclk.mclk770.mclk_value = cpu_to_be32(memory_clock);
    mclk.mclk770.vMPLL_AD_FUNC_CNTL = cpu_to_be32(mpll_ad_func_cntl);
    mclk.mclk770.vMPLL_AD_FUNC_CNTL_2 = cpu_to_be32(mpll_ad_func_cntl_2);
    mclk.mclk770.vMPLL_DQ_FUNC_CNTL = cpu_to_be32(mpll_dq_func_cntl);
    mclk.mclk770.vMPLL_DQ_FUNC_CNTL_2 = cpu_to_be32(mpll_dq_func_cntl_2);
    mclk.mclk770.vMCLK_PWRMGT_CNTL = cpu_to_be32(mclk_pwrmgt_cntl);
    mclk.mclk770.vDLL_CNTL = cpu_to_be32(dll_cntl);
    mclk.mclk770.vMPLL_SS = cpu_to_be32(mpll_ss1);
    mclk.mclk770.vMPLL_SS2 = cpu_to_be32(mpll_ss2);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rv740_read_clock_registers(rdev: *mut radeon_device) {
    void rv740_read_clock_registers(struct radeon_device *rdev)
    {
    struct rv7xx_power_info *pi = rv770_get_pi(rdev);
    pi.clk_regs.rv770.cg_spll_func_cntl =
    RREG32(CG_SPLL_FUNC_CNTL);
    pi.clk_regs.rv770.cg_spll_func_cntl_2 =
    RREG32(CG_SPLL_FUNC_CNTL_2);
    pi.clk_regs.rv770.cg_spll_func_cntl_3 =
    RREG32(CG_SPLL_FUNC_CNTL_3);
    pi.clk_regs.rv770.cg_spll_spread_spectrum =
    RREG32(CG_SPLL_SPREAD_SPECTRUM);
    pi.clk_regs.rv770.cg_spll_spread_spectrum_2 =
    RREG32(CG_SPLL_SPREAD_SPECTRUM_2);
    pi.clk_regs.rv770.mpll_ad_func_cntl =
    RREG32(MPLL_AD_FUNC_CNTL);
    pi.clk_regs.rv770.mpll_ad_func_cntl_2 =
    RREG32(MPLL_AD_FUNC_CNTL_2);
    pi.clk_regs.rv770.mpll_dq_func_cntl =
    RREG32(MPLL_DQ_FUNC_CNTL);
    pi.clk_regs.rv770.mpll_dq_func_cntl_2 =
    RREG32(MPLL_DQ_FUNC_CNTL_2);
    pi.clk_regs.rv770.mclk_pwrmgt_cntl =
    RREG32(MCLK_PWRMGT_CNTL);
    pi.clk_regs.rv770.dll_cntl = RREG32(DLL_CNTL);
    pi.clk_regs.rv770.mpll_ss1 = RREG32(MPLL_SS1);
    pi.clk_regs.rv770.mpll_ss2 = RREG32(MPLL_SS2);
    }
    int rv740_populate_smc_acpi_state(struct radeon_device *rdev,
    RV770_SMC_STATETABLE *table)
    {
    struct rv7xx_power_info *pi = rv770_get_pi(rdev);
    let mut mpll_ad_func_cntl: u32 = pi.clk_regs.rv770.mpll_ad_func_cntl;
    let mut mpll_ad_func_cntl_2: u32 = pi.clk_regs.rv770.mpll_ad_func_cntl_2;
    let mut mpll_dq_func_cntl: u32 = pi.clk_regs.rv770.mpll_dq_func_cntl;
    let mut mpll_dq_func_cntl_2: u32 = pi.clk_regs.rv770.mpll_dq_func_cntl_2;
    let mut spll_func_cntl: u32 = pi.clk_regs.rv770.cg_spll_func_cntl;
    let mut spll_func_cntl_2: u32 = pi.clk_regs.rv770.cg_spll_func_cntl_2;
    let mut spll_func_cntl_3: u32 = pi.clk_regs.rv770.cg_spll_func_cntl_3;
    let mut mclk_pwrmgt_cntl: u32 = pi.clk_regs.rv770.mclk_pwrmgt_cntl;
    let mut dll_cntl: u32 = pi.clk_regs.rv770.dll_cntl;
    table.ACPIState = table.initialState;
    table.ACPIState.flags &= ~PPSMC_SWSTATE_FLAG_DC;
    if (pi.acpi_vddc) {
    rv770_populate_vddc_value(rdev, pi.acpi_vddc,
    &table.ACPIState.levels[0].vddc);
    table.ACPIState.levels[0].gen2PCIE =
    pi.pcie_gen2 ?
    pi.acpi_pcie_gen2 : 0;
    table.ACPIState.levels[0].gen2XSP =
    pi.acpi_pcie_gen2;
    } else {
    rv770_populate_vddc_value(rdev, pi.min_vddc_in_table,
    &table.ACPIState.levels[0].vddc);
    table.ACPIState.levels[0].gen2PCIE = 0;
    }
    mpll_ad_func_cntl_2 |= BIAS_GEN_PDNB | RESET_EN;
    mpll_dq_func_cntl_2 |= BYPASS | BIAS_GEN_PDNB | RESET_EN;
    mclk_pwrmgt_cntl |= (MRDCKA0_RESET |
    MRDCKA1_RESET |
    MRDCKB0_RESET |
    MRDCKB1_RESET |
    MRDCKC0_RESET |
    MRDCKC1_RESET |
    MRDCKD0_RESET |
    MRDCKD1_RESET);
    dll_cntl |= (MRDCKA0_BYPASS |
    MRDCKA1_BYPASS |
    MRDCKB0_BYPASS |
    MRDCKB1_BYPASS |
    MRDCKC0_BYPASS |
    MRDCKC1_BYPASS |
    MRDCKD0_BYPASS |
    MRDCKD1_BYPASS);
    spll_func_cntl |= SPLL_RESET | SPLL_SLEEP | SPLL_BYPASS_EN;
    spll_func_cntl_2 &= ~SCLK_MUX_SEL_MASK;
    spll_func_cntl_2 |= SCLK_MUX_SEL(4);
    table.ACPIState.levels[0].mclk.mclk770.vMPLL_AD_FUNC_CNTL = cpu_to_be32(mpll_ad_func_cntl);
    table.ACPIState.levels[0].mclk.mclk770.vMPLL_AD_FUNC_CNTL_2 = cpu_to_be32(mpll_ad_func_cntl_2);
    table.ACPIState.levels[0].mclk.mclk770.vMPLL_DQ_FUNC_CNTL = cpu_to_be32(mpll_dq_func_cntl);
    table.ACPIState.levels[0].mclk.mclk770.vMPLL_DQ_FUNC_CNTL_2 = cpu_to_be32(mpll_dq_func_cntl_2);
    table.ACPIState.levels[0].mclk.mclk770.vMCLK_PWRMGT_CNTL = cpu_to_be32(mclk_pwrmgt_cntl);
    table.ACPIState.levels[0].mclk.mclk770.vDLL_CNTL = cpu_to_be32(dll_cntl);
    table.ACPIState.levels[0].mclk.mclk770.mclk_value = 0;
    table.ACPIState.levels[0].sclk.vCG_SPLL_FUNC_CNTL = cpu_to_be32(spll_func_cntl);
    table.ACPIState.levels[0].sclk.vCG_SPLL_FUNC_CNTL_2 = cpu_to_be32(spll_func_cntl_2);
    table.ACPIState.levels[0].sclk.vCG_SPLL_FUNC_CNTL_3 = cpu_to_be32(spll_func_cntl_3);
    table.ACPIState.levels[0].sclk.sclk_value = 0;
    table.ACPIState.levels[1] = table.ACPIState.levels[0];
    table.ACPIState.levels[2] = table.ACPIState.levels[0];
    rv770_populate_mvdd_value(rdev, 0, &table.ACPIState.levels[0].mvdd);
    return 0;
    }
    void rv740_enable_mclk_spread_spectrum(struct radeon_device *rdev,
    bool enable)
    {
    if (enable)
    WREG32_P(MPLL_CNTL_MODE, SS_SSEN, ~SS_SSEN);
    else
    WREG32_P(MPLL_CNTL_MODE, 0, ~SS_SSEN);
    }
#[no_mangle]
pub unsafe extern "C" fn rv740_get_mclk_frequency_ratio(memory_clock: u32) -> u8 {
    u8 rv740_get_mclk_frequency_ratio(u32 memory_clock)
    {
    u8 mc_para_index;
    if ((memory_clock < 10000) || (memory_clock > 47500))
    mc_para_index = 0x00;
    else
    mc_para_index = (u8)((memory_clock - 10000) / 2500);
    return mc_para_index;
    }

//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/dc/clk_mgr/dcn35/dcn351_clk_mgr.c
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
// Copyright 2024 Advanced Micro Devices, Inc.
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
// Authors: AMD
//

pub const DCN_BASE__INST0_SEG1: c_uint = 0x000000C0;
pub const mmCLK1_CLK_PLL_REQ: c_uint = 0x16E37;
pub const mmCLK1_CLK0_DFS_CNTL: c_uint = 0x16E69;
pub const mmCLK1_CLK1_DFS_CNTL: c_uint = 0x16E6C;
pub const mmCLK1_CLK2_DFS_CNTL: c_uint = 0x16E6F;
pub const mmCLK1_CLK3_DFS_CNTL: c_uint = 0x16E72;
pub const mmCLK1_CLK4_DFS_CNTL: c_uint = 0x16E75;
pub const mmCLK1_CLK5_DFS_CNTL: c_uint = 0x16E78;
pub const mmCLK1_CLK0_CURRENT_CNT: c_uint = 0x16EFC;
pub const mmCLK1_CLK1_CURRENT_CNT: c_uint = 0x16EFD;
pub const mmCLK1_CLK2_CURRENT_CNT: c_uint = 0x16EFE;
pub const mmCLK1_CLK3_CURRENT_CNT: c_uint = 0x16EFF;
pub const mmCLK1_CLK4_CURRENT_CNT: c_uint = 0x16F00;
pub const mmCLK1_CLK5_CURRENT_CNT: c_uint = 0x16F01;
pub const mmCLK1_CLK0_BYPASS_CNTL: c_uint = 0x16E8A;
pub const mmCLK1_CLK1_BYPASS_CNTL: c_uint = 0x16E93;
pub const mmCLK1_CLK2_BYPASS_CNTL: c_uint = 0x16E9C;
pub const mmCLK1_CLK3_BYPASS_CNTL: c_uint = 0x16EA5;
pub const mmCLK1_CLK4_BYPASS_CNTL: c_uint = 0x16EAE;
pub const mmCLK1_CLK5_BYPASS_CNTL: c_uint = 0x16EB7;
pub const mmCLK1_CLK0_DS_CNTL: c_uint = 0x16E83;
pub const mmCLK1_CLK1_DS_CNTL: c_uint = 0x16E8C;
pub const mmCLK1_CLK2_DS_CNTL: c_uint = 0x16E95;
pub const mmCLK1_CLK3_DS_CNTL: c_uint = 0x16E9E;
pub const mmCLK1_CLK4_DS_CNTL: c_uint = 0x16EA7;
pub const mmCLK1_CLK5_DS_CNTL: c_uint = 0x16EB0;
pub const mmCLK1_CLK0_ALLOW_DS: c_uint = 0x16E84;
pub const mmCLK1_CLK1_ALLOW_DS: c_uint = 0x16E8D;
pub const mmCLK1_CLK2_ALLOW_DS: c_uint = 0x16E96;
pub const mmCLK1_CLK3_ALLOW_DS: c_uint = 0x16E9F;
pub const mmCLK1_CLK4_ALLOW_DS: c_uint = 0x16EA8;
pub const mmCLK1_CLK5_ALLOW_DS: c_uint = 0x16EB1;
pub const mmCLK5_spll_field_8: c_uint = 0x1B04B;
pub const mmCLK6_spll_field_8: c_uint = 0x1B24B;
pub const mmDENTIST_DISPCLK_CNTL: c_uint = 0x0124;
pub const regDENTIST_DISPCLK_CNTL: c_uint = 0x0064;
pub const regDENTIST_DISPCLK_CNTL_BASE_IDX: c_int = 1;
pub const CLK1_CLK_PLL_REQ__FbMult_int__SHIFT: c_uint = 0x0;
pub const CLK1_CLK_PLL_REQ__PllSpineDiv__SHIFT: c_uint = 0xc;
pub const CLK1_CLK_PLL_REQ__FbMult_frac__SHIFT: c_uint = 0x10;
pub const CLK1_CLK_PLL_REQ__FbMult_int_MASK: c_uint = 0x000001FFL;
pub const CLK1_CLK_PLL_REQ__PllSpineDiv_MASK: c_uint = 0x0000F000L;
pub const CLK1_CLK_PLL_REQ__FbMult_frac_MASK: c_uint = 0xFFFF0000L;
pub const CLK1_CLK2_BYPASS_CNTL__CLK2_BYPASS_SEL_MASK: c_uint = 0x00000007L;
// DENTIST_DISPCLK_CNTL
pub const DENTIST_DISPCLK_CNTL__DENTIST_DISPCLK_WDIVIDER__SHIFT: c_uint = 0x0;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DISPCLK_RDIVIDER__SHIFT: c_uint = 0x8;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DISPCLK_CHG_DONE__SHIFT: c_uint = 0x13;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DPPCLK_CHG_DONE__SHIFT: c_uint = 0x14;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DPPCLK_WDIVIDER__SHIFT: c_uint = 0x18;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DISPCLK_WDIVIDER_MASK: c_uint = 0x0000007FL;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DISPCLK_RDIVIDER_MASK: c_uint = 0x00007F00L;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DISPCLK_CHG_DONE_MASK: c_uint = 0x00080000L;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DPPCLK_CHG_DONE_MASK: c_uint = 0x00100000L;
pub const DENTIST_DISPCLK_CNTL__DENTIST_DPPCLK_WDIVIDER_MASK: c_uint = 0x7F000000L;
pub const CLK5_spll_field_8__spll_ssc_en_MASK: c_uint = 0x00002000L;

    (clk_mgr.regs.reg)

// Macro flag: #define SR(reg_name)\
    .reg_name = BASE(reg ## reg_name ## _BASE_IDX) +  \
    reg ## reg_name
// Macro flag: #define CLK_SR_DCN35(reg_name)\
    .reg_name = mm ## reg_name
    static const struct clk_mgr_registers clk_mgr_regs_dcn351 = {
    CLK_REG_LIST_DCN35()
    };
    static const struct clk_mgr_shift clk_mgr_shift_dcn351 = {
    CLK_COMMON_MASK_SH_LIST_DCN32(__SHIFT)
    };
    static const struct clk_mgr_mask clk_mgr_mask_dcn351 = {
    CLK_COMMON_MASK_SH_LIST_DCN32(_MASK)
    };
// Macro flag: #define TO_CLK_MGR_DCN35(clk_mgr)\
    container_of(clk_mgr, struct clk_mgr_dcn35, base)
    void dcn351_clk_mgr_construct(
    struct dc_context *ctx,
    struct clk_mgr_dcn35 *clk_mgr,
    struct pp_smu_funcs *pp_smu,
    struct dccg *dccg)
    {
// register offset changed
    clk_mgr.base.regs = &clk_mgr_regs_dcn351;
    clk_mgr.base.clk_mgr_shift = &clk_mgr_shift_dcn351;
    clk_mgr.base.clk_mgr_mask = &clk_mgr_mask_dcn351;
    dcn35_clk_mgr_construct(ctx,  clk_mgr, pp_smu, dccg);
    }

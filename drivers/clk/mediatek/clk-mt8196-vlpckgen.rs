//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8196-vlpckgen.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2025 MediaTek Inc.
// Guangjie Song <guangjie.song@mediatek.com>
// Copyright (c) 2025 Collabora Ltd.
// Laura Nao <laura.nao@collabora.com>
//

// MUX SEL REG
pub const VLP_CLK_CFG_UPDATE: c_uint = 0x0004;
pub const VLP_CLK_CFG_UPDATE1: c_uint = 0x0008;
pub const VLP_CLK_CFG_0: c_uint = 0x0010;
pub const VLP_CLK_CFG_0_SET: c_uint = 0x0014;
pub const VLP_CLK_CFG_0_CLR: c_uint = 0x0018;
pub const VLP_CLK_CFG_1: c_uint = 0x0020;
pub const VLP_CLK_CFG_1_SET: c_uint = 0x0024;
pub const VLP_CLK_CFG_1_CLR: c_uint = 0x0028;
pub const VLP_CLK_CFG_2: c_uint = 0x0030;
pub const VLP_CLK_CFG_2_SET: c_uint = 0x0034;
pub const VLP_CLK_CFG_2_CLR: c_uint = 0x0038;
pub const VLP_CLK_CFG_3: c_uint = 0x0040;
pub const VLP_CLK_CFG_3_SET: c_uint = 0x0044;
pub const VLP_CLK_CFG_3_CLR: c_uint = 0x0048;
pub const VLP_CLK_CFG_4: c_uint = 0x0050;
pub const VLP_CLK_CFG_4_SET: c_uint = 0x0054;
pub const VLP_CLK_CFG_4_CLR: c_uint = 0x0058;
pub const VLP_CLK_CFG_5: c_uint = 0x0060;
pub const VLP_CLK_CFG_5_SET: c_uint = 0x0064;
pub const VLP_CLK_CFG_5_CLR: c_uint = 0x0068;
pub const VLP_CLK_CFG_6: c_uint = 0x0070;
pub const VLP_CLK_CFG_6_SET: c_uint = 0x0074;
pub const VLP_CLK_CFG_6_CLR: c_uint = 0x0078;
pub const VLP_CLK_CFG_7: c_uint = 0x0080;
pub const VLP_CLK_CFG_7_SET: c_uint = 0x0084;
pub const VLP_CLK_CFG_7_CLR: c_uint = 0x0088;
pub const VLP_CLK_CFG_8: c_uint = 0x0090;
pub const VLP_CLK_CFG_8_SET: c_uint = 0x0094;
pub const VLP_CLK_CFG_8_CLR: c_uint = 0x0098;
pub const VLP_CLK_CFG_9: c_uint = 0x00a0;
pub const VLP_CLK_CFG_9_SET: c_uint = 0x00a4;
pub const VLP_CLK_CFG_9_CLR: c_uint = 0x00a8;
pub const VLP_CLK_CFG_10: c_uint = 0x00b0;
pub const VLP_CLK_CFG_10_SET: c_uint = 0x00b4;
pub const VLP_CLK_CFG_10_CLR: c_uint = 0x00b8;
pub const VLP_OCIC_FENC_STATUS_MON_0: c_uint = 0x039c;
pub const VLP_OCIC_FENC_STATUS_MON_1: c_uint = 0x03a0;
// MUX SHIFT
pub const TOP_MUX_SCP_SHIFT: c_int = 0;
pub const TOP_MUX_SCP_SPI_SHIFT: c_int = 1;
pub const TOP_MUX_SCP_IIC_SHIFT: c_int = 2;
pub const TOP_MUX_SCP_IIC_HS_SHIFT: c_int = 3;
pub const TOP_MUX_PWRAP_ULPOSC_SHIFT: c_int = 4;
pub const TOP_MUX_SPMI_M_TIA_32K_SHIFT: c_int = 5;
pub const TOP_MUX_APXGPT_26M_B_SHIFT: c_int = 6;
pub const TOP_MUX_DPSW_SHIFT: c_int = 7;
pub const TOP_MUX_DPSW_CENTRAL_SHIFT: c_int = 8;
pub const TOP_MUX_SPMI_M_MST_SHIFT: c_int = 9;
pub const TOP_MUX_DVFSRC_SHIFT: c_int = 10;
pub const TOP_MUX_PWM_VLP_SHIFT: c_int = 11;
pub const TOP_MUX_AXI_VLP_SHIFT: c_int = 12;
pub const TOP_MUX_SYSTIMER_26M_SHIFT: c_int = 13;
pub const TOP_MUX_SSPM_SHIFT: c_int = 14;
pub const TOP_MUX_SRCK_SHIFT: c_int = 15;
pub const TOP_MUX_CAMTG0_SHIFT: c_int = 16;
pub const TOP_MUX_CAMTG1_SHIFT: c_int = 17;
pub const TOP_MUX_CAMTG2_SHIFT: c_int = 18;
pub const TOP_MUX_CAMTG3_SHIFT: c_int = 19;
pub const TOP_MUX_CAMTG4_SHIFT: c_int = 20;
pub const TOP_MUX_CAMTG5_SHIFT: c_int = 21;
pub const TOP_MUX_CAMTG6_SHIFT: c_int = 22;
pub const TOP_MUX_CAMTG7_SHIFT: c_int = 23;
pub const TOP_MUX_SSPM_26M_SHIFT: c_int = 25;
pub const TOP_MUX_ULPOSC_SSPM_SHIFT: c_int = 26;
pub const TOP_MUX_VLP_PBUS_26M_SHIFT: c_int = 27;
pub const TOP_MUX_DEBUG_ERR_FLAG_VLP_26M_SHIFT: c_int = 28;
pub const TOP_MUX_DPMSRDMA_SHIFT: c_int = 29;
pub const TOP_MUX_VLP_PBUS_156M_SHIFT: c_int = 30;
pub const TOP_MUX_SPM_SHIFT: c_int = 0;
pub const TOP_MUX_MMINFRA_VLP_SHIFT: c_int = 1;
pub const TOP_MUX_USB_TOP_SHIFT: c_int = 2;
pub const TOP_MUX_SSUSB_XHCI_SHIFT: c_int = 3;
pub const TOP_MUX_NOC_VLP_SHIFT: c_int = 4;
pub const TOP_MUX_AUDIO_H_SHIFT: c_int = 5;
pub const TOP_MUX_AUD_ENGEN1_SHIFT: c_int = 6;
pub const TOP_MUX_AUD_ENGEN2_SHIFT: c_int = 7;
pub const TOP_MUX_AUD_INTBUS_SHIFT: c_int = 8;
pub const TOP_MUX_SPU_VLP_26M_SHIFT: c_int = 9;
pub const TOP_MUX_SPU0_VLP_SHIFT: c_int = 10;
pub const TOP_MUX_SPU1_VLP_SHIFT: c_int = 11;
// CKSTA REG
pub const VLP_CKSTA_REG0: c_uint = 0x0250;
pub const VLP_CKSTA_REG1: c_uint = 0x0254;
// HW Voter REG
pub const HWV_CG_9_SET: c_uint = 0x0048;
pub const HWV_CG_9_CLR: c_uint = 0x004c;
pub const HWV_CG_9_DONE: c_uint = 0x2c24;
pub const HWV_CG_10_SET: c_uint = 0x0050;
pub const HWV_CG_10_CLR: c_uint = 0x0054;
pub const HWV_CG_10_DONE: c_uint = 0x2c28;
// PLL REG
pub const VLP_AP_PLL_CON3: c_uint = 0x264;
pub const VLP_APLL1_TUNER_CON0: c_uint = 0x2a4;
pub const VLP_APLL2_TUNER_CON0: c_uint = 0x2a8;
pub const VLP_APLL1_CON0: c_uint = 0x274;
pub const VLP_APLL1_CON1: c_uint = 0x278;
pub const VLP_APLL1_CON2: c_uint = 0x27c;
pub const VLP_APLL1_CON3: c_uint = 0x280;
pub const VLP_APLL2_CON0: c_uint = 0x28c;
pub const VLP_APLL2_CON1: c_uint = 0x290;
pub const VLP_APLL2_CON2: c_uint = 0x294;
pub const VLP_APLL2_CON3: c_uint = 0x298;
// vlp apll1 tuner default value
pub const VLP_APLL1_TUNER_CON0_VALUE: c_uint = 0x6f28bd4d;
// vlp apll2 tuner default value + 1
pub const VLP_APLL2_TUNER_CON0_VALUE: c_uint = 0x78fd5265;
pub const VLP_PLLEN_ALL: c_uint = 0x080;
pub const VLP_PLLEN_ALL_SET: c_uint = 0x084;
pub const VLP_PLLEN_ALL_CLR: c_uint = 0x088;

pub const MT8196_INTEGER_BITS: c_int = 8;

    _flags, _pd_reg, _pd_shift,			\
    _pcw_reg, _pcw_shift, _pcwbits,		\
    _pll_en_bit) {					\
    .id = _id,					\
    .name = _name,					\
    .reg = _reg,					\
    .fenc_sta_ofs = _fenc_sta_ofs,			\
    .fenc_sta_bit = _fenc_sta_bit,			\
    .flags = _flags,				\
    .fmax = MT8196_PLL_FMAX,			\
    .fmin = MT8196_PLL_FMIN,			\
    .pd_reg = _pd_reg,				\
    .pd_shift = _pd_shift,				\
    .pcw_reg = _pcw_reg,				\
    .pcw_shift = _pcw_shift,			\
    .pcwbits = _pcwbits,				\
    .pcwibits = MT8196_INTEGER_BITS,		\
    .en_reg = VLP_PLLEN_ALL,			\
    .en_set_reg = VLP_PLLEN_ALL_SET,		\
    .en_clr_reg = VLP_PLLEN_ALL_CLR,		\
    .pll_en_bit = _pll_en_bit,			\
    .ops = &mtk_pll_fenc_clr_set_ops,		\
    }
    static DEFINE_SPINLOCK(mt8196_clk_vlp_lock);
    static const struct mtk_fixed_factor vlp_divs[] = {
    FACTOR(CLK_VLP_CLK26M, "vlp_clk26m", "clk26m", 1, 1),
    FACTOR(CLK_VLP_APLL1_D4, "apll1_d4", "vlp_apll1", 1, 4),
    FACTOR(CLK_VLP_APLL1_D8, "apll1_d8", "vlp_apll1", 1, 8),
    FACTOR(CLK_VLP_APLL2_D4, "apll2_d4", "vlp_apll2", 1, 4),
    FACTOR(CLK_VLP_APLL2_D8, "apll2_d8", "vlp_apll2", 1, 8),
    };
    static const char * const vlp_scp_parents[] = {
    "clk26m",
    "osc_d20",
    "mainpll_d6",
    "mainpll_d4",
    "mainpll_d3",
    "vlp_apll1"
    };
    static const char * const vlp_scp_spi_parents[] = {
    "clk26m",
    "osc_d20",
    "mainpll_d7_d2",
    "mainpll_d5_d2"
    };
    static const char * const vlp_scp_iic_parents[] = {
    "clk26m",
    "osc_d20",
    "mainpll_d5_d4",
    "mainpll_d7_d2"
    };
    static const char * const vlp_scp_iic_hs_parents[] = {
    "clk26m",
    "osc_d20",
    "mainpll_d5_d4",
    "mainpll_d7_d2",
    "mainpll_d7"
    };
    static const char * const vlp_pwrap_ulposc_parents[] = {
    "clk26m",
    "osc_d20",
    "osc_d14",
    "osc_d10"
    };
    static const char * const vlp_spmi_32k_parents[] = {
    "clk26m",
    "clk32k",
    "osc_d20",
    "osc_d14",
    "osc_d10"
    };
    static const char * const vlp_apxgpt_26m_b_parents[] = {
    "clk26m",
    "osc_d20"
    };
    static const char * const vlp_dpsw_parents[] = {
    "clk26m",
    "osc_d10",
    "osc_d7",
    "mainpll_d7_d4"
    };
    static const char * const vlp_dpsw_central_parents[] = {
    "clk26m",
    "osc_d10",
    "osc_d7",
    "mainpll_d7_d4"
    };
    static const char * const vlp_spmi_m_parents[] = {
    "clk26m",
    "osc_d20",
    "osc_d14",
    "osc_d10"
    };
    static const char * const vlp_dvfsrc_parents[] = {
    "clk26m",
    "osc_d20"
    };
    static const char * const vlp_pwm_vlp_parents[] = {
    "clk26m",
    "clk32k",
    "osc_d20",
    "osc_d8",
    "mainpll_d4_d8"
    };
    static const char * const vlp_axi_vlp_parents[] = {
    "clk26m",
    "osc_d20",
    "mainpll_d7_d4",
    "osc_d4",
    "mainpll_d7_d2"
    };
    static const char * const vlp_systimer_26m_parents[] = {
    "clk26m",
    "osc_d20"
    };
    static const char * const vlp_sspm_parents[] = {
    "clk26m",
    "osc_d20",
    "mainpll_d5_d2",
    "osc_d2",
    "mainpll_d6"
    };
    static const char * const vlp_srck_parents[] = {
    "clk26m",
    "osc_d20"
    };
    static const char * const vlp_camtg0_1_parents[] = {
    "clk26m",
    "univpll_192m_d32",
    "univpll_192m_d16",
    "clk13m",
    "osc_d40",
    "osc_d32",
    "univpll_192m_d10",
    "univpll_192m_d8",
    "univpll_d6_d16",
    "ulposc3",
    "osc_d20",
    "ck2_tvdpll1_d16",
    "univpll_d6_d8"
    };
    static const char * const vlp_camtg2_7_parents[] = {
    "clk26m",
    "univpll_192m_d32",
    "univpll_192m_d16",
    "clk13m",
    "osc_d40",
    "osc_d32",
    "univpll_192m_d10",
    "univpll_192m_d8",
    "univpll_d6_d16",
    "osc_d20",
    "ck2_tvdpll1_d16",
    "univpll_d6_d8"
    };
    static const char * const vlp_sspm_26m_parents[] = {
    "clk26m",
    "osc_d20"
    };
    static const char * const vlp_ulposc_sspm_parents[] = {
    "clk26m",
    "osc_d2",
    "mainpll_d4_d2"
    };
    static const char * const vlp_vlp_pbus_26m_parents[] = {
    "clk26m",
    "osc_d20"
    };
    static const char * const vlp_debug_err_flag_parents[] = {
    "clk26m",
    "osc_d20"
    };
    static const char * const vlp_dpmsrdma_parents[] = {
    "clk26m",
    "mainpll_d7_d2"
    };
    static const char * const vlp_vlp_pbus_156m_parents[] = {
    "clk26m",
    "osc_d2",
    "mainpll_d7_d2",
    "mainpll_d7"
    };
    static const char * const vlp_spm_parents[] = {
    "clk26m",
    "mainpll_d7_d4"
    };
    static const char * const vlp_mminfra_parents[] = {
    "clk26m",
    "osc_d4",
    "mainpll_d3"
    };
    static const char * const vlp_usb_parents[] = {
    "clk26m",
    "mainpll_d9"
    };
    static const char * const vlp_noc_vlp_parents[] = {
    "clk26m",
    "osc_d20",
    "mainpll_d9"
    };
    static const char * const vlp_audio_h_parents[] = {
    "vlp_clk26m",
    "vlp_apll1",
    "vlp_apll2"
    };
    static const char * const vlp_aud_engen1_parents[] = {
    "vlp_clk26m",
    "apll1_d8",
    "apll1_d4"
    };
    static const char * const vlp_aud_engen2_parents[] = {
    "vlp_clk26m",
    "apll2_d8",
    "apll2_d4"
    };
    static const char * const vlp_aud_intbus_parents[] = {
    "vlp_clk26m",
    "mainpll_d7_d4",
    "mainpll_d4_d4"
    };
    static const u8 vlp_aud_parent_index[] = { 1, 2, 3 };
    static const char * const vlp_spvlp_26m_parents[] = {
    "clk26m",
    "osc_d20"
    };
    static const char * const vlp_spu0_vlp_parents[] = {
    "clk26m",
    "osc_d20",
    "mainpll_d4_d4",
    "mainpll_d4_d2",
    "mainpll_d7",
    "mainpll_d6",
    "mainpll_d5"
    };
    static const char * const vlp_spu1_vlp_parents[] = {
    "clk26m",
    "osc_d20",
    "mainpll_d4_d4",
    "mainpll_d4_d2",
    "mainpll_d7",
    "mainpll_d6",
    "mainpll_d5"
    };
    static const struct mtk_mux vlp_muxes[] = {
// VLP_CLK_CFG_0
    MUX_GATE_FENC_CLR_SET_UPD(CLK_VLP_SCP, "vlp_scp", vlp_scp_parents,
    VLP_CLK_CFG_0, VLP_CLK_CFG_0_SET, VLP_CLK_CFG_0_CLR,
    0, 3, 7, VLP_CLK_CFG_UPDATE, TOP_MUX_SCP_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_0, 31),
    MUX_CLR_SET_UPD(CLK_VLP_SCP_SPI, "vlp_scp_spi",
    vlp_scp_spi_parents, VLP_CLK_CFG_0, VLP_CLK_CFG_0_SET,
    VLP_CLK_CFG_0_CLR, 8, 2,
    VLP_CLK_CFG_UPDATE, TOP_MUX_SCP_SPI_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_SCP_IIC, "vlp_scp_iic",
    vlp_scp_iic_parents, VLP_CLK_CFG_0, VLP_CLK_CFG_0_SET,
    VLP_CLK_CFG_0_CLR, 16, 2,
    VLP_CLK_CFG_UPDATE, TOP_MUX_SCP_IIC_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_SCP_IIC_HS, "vlp_scp_iic_hs",
    vlp_scp_iic_hs_parents, VLP_CLK_CFG_0, VLP_CLK_CFG_0_SET,
    VLP_CLK_CFG_0_CLR, 24, 3,
    VLP_CLK_CFG_UPDATE, TOP_MUX_SCP_IIC_HS_SHIFT),
// VLP_CLK_CFG_1
    MUX_CLR_SET_UPD(CLK_VLP_PWRAP_ULPOSC, "vlp_pwrap_ulposc",
    vlp_pwrap_ulposc_parents, VLP_CLK_CFG_1, VLP_CLK_CFG_1_SET,
    VLP_CLK_CFG_1_CLR, 0, 2,
    VLP_CLK_CFG_UPDATE, TOP_MUX_PWRAP_ULPOSC_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_SPMI_M_TIA_32K, "vlp_spmi_32k",
    vlp_spmi_32k_parents, VLP_CLK_CFG_1, VLP_CLK_CFG_1_SET,
    VLP_CLK_CFG_1_CLR, 8, 3,
    VLP_CLK_CFG_UPDATE, TOP_MUX_SPMI_M_TIA_32K_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_APXGPT_26M_B, "vlp_apxgpt_26m_b",
    vlp_apxgpt_26m_b_parents, VLP_CLK_CFG_1, VLP_CLK_CFG_1_SET,
    VLP_CLK_CFG_1_CLR, 16, 1,
    VLP_CLK_CFG_UPDATE, TOP_MUX_APXGPT_26M_B_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_DPSW, "vlp_dpsw",
    vlp_dpsw_parents, VLP_CLK_CFG_1, VLP_CLK_CFG_1_SET,
    VLP_CLK_CFG_1_CLR, 24, 2,
    VLP_CLK_CFG_UPDATE, TOP_MUX_DPSW_SHIFT),
// VLP_CLK_CFG_2
    MUX_CLR_SET_UPD(CLK_VLP_DPSW_CENTRAL, "vlp_dpsw_central",
    vlp_dpsw_central_parents, VLP_CLK_CFG_2, VLP_CLK_CFG_2_SET,
    VLP_CLK_CFG_2_CLR, 0, 2,
    VLP_CLK_CFG_UPDATE, TOP_MUX_DPSW_CENTRAL_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_SPMI_M_MST, "vlp_spmi_m",
    vlp_spmi_m_parents, VLP_CLK_CFG_2, VLP_CLK_CFG_2_SET,
    VLP_CLK_CFG_2_CLR, 8, 2,
    VLP_CLK_CFG_UPDATE, TOP_MUX_SPMI_M_MST_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_DVFSRC, "vlp_dvfsrc",
    vlp_dvfsrc_parents, VLP_CLK_CFG_2, VLP_CLK_CFG_2_SET,
    VLP_CLK_CFG_2_CLR, 16, 1,
    VLP_CLK_CFG_UPDATE, TOP_MUX_DVFSRC_SHIFT),
    MUX_GATE_FENC_CLR_SET_UPD(CLK_VLP_PWM_VLP, "vlp_pwm_vlp", vlp_pwm_vlp_parents,
    VLP_CLK_CFG_2, VLP_CLK_CFG_2_SET, VLP_CLK_CFG_2_CLR,
    24, 3, 31, VLP_CLK_CFG_UPDATE, TOP_MUX_PWM_VLP_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_0, 20),
// VLP_CLK_CFG_3
    MUX_CLR_SET_UPD(CLK_VLP_AXI_VLP, "vlp_axi_vlp",
    vlp_axi_vlp_parents, VLP_CLK_CFG_3, VLP_CLK_CFG_3_SET,
    VLP_CLK_CFG_3_CLR, 0, 3,
    VLP_CLK_CFG_UPDATE, TOP_MUX_AXI_VLP_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_SYSTIMER_26M, "vlp_systimer_26m",
    vlp_systimer_26m_parents, VLP_CLK_CFG_3, VLP_CLK_CFG_3_SET,
    VLP_CLK_CFG_3_CLR, 8, 1,
    VLP_CLK_CFG_UPDATE, TOP_MUX_SYSTIMER_26M_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_SSPM, "vlp_sspm",
    vlp_sspm_parents, VLP_CLK_CFG_3, VLP_CLK_CFG_3_SET,
    VLP_CLK_CFG_3_CLR, 16, 3,
    VLP_CLK_CFG_UPDATE, TOP_MUX_SSPM_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_SRCK, "vlp_srck",
    vlp_srck_parents, VLP_CLK_CFG_3, VLP_CLK_CFG_3_SET,
    VLP_CLK_CFG_3_CLR, 24, 1,
    VLP_CLK_CFG_UPDATE, TOP_MUX_SRCK_SHIFT),
// VLP_CLK_CFG_4
    MUX_GATE_HWV_FENC_CLR_SET_UPD(CLK_VLP_CAMTG0, "vlp_camtg0", vlp_camtg0_1_parents,
    VLP_CLK_CFG_4, VLP_CLK_CFG_4_SET, VLP_CLK_CFG_4_CLR,
    HWV_CG_9_DONE, HWV_CG_9_SET, HWV_CG_9_CLR,
    0, 4, 7, VLP_CLK_CFG_UPDATE, TOP_MUX_CAMTG0_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_0, 15),
    MUX_GATE_HWV_FENC_CLR_SET_UPD(CLK_VLP_CAMTG1, "vlp_camtg1", vlp_camtg0_1_parents,
    VLP_CLK_CFG_4, VLP_CLK_CFG_4_SET, VLP_CLK_CFG_4_CLR,
    HWV_CG_9_DONE, HWV_CG_9_SET, HWV_CG_9_CLR,
    8, 4, 15, VLP_CLK_CFG_UPDATE, TOP_MUX_CAMTG1_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_0, 14),
    MUX_GATE_HWV_FENC_CLR_SET_UPD(CLK_VLP_CAMTG2, "vlp_camtg2", vlp_camtg2_7_parents,
    VLP_CLK_CFG_4, VLP_CLK_CFG_4_SET, VLP_CLK_CFG_4_CLR,
    HWV_CG_9_DONE, HWV_CG_9_SET, HWV_CG_9_CLR,
    16, 4, 23, VLP_CLK_CFG_UPDATE, TOP_MUX_CAMTG2_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_0, 13),
    MUX_GATE_HWV_FENC_CLR_SET_UPD(CLK_VLP_CAMTG3, "vlp_camtg3", vlp_camtg2_7_parents,
    VLP_CLK_CFG_4, VLP_CLK_CFG_4_SET, VLP_CLK_CFG_4_CLR,
    HWV_CG_9_DONE, HWV_CG_9_SET, HWV_CG_9_CLR,
    24, 4, 31, VLP_CLK_CFG_UPDATE, TOP_MUX_CAMTG3_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_0, 12),
// VLP_CLK_CFG_5
    MUX_GATE_HWV_FENC_CLR_SET_UPD(CLK_VLP_CAMTG4, "vlp_camtg4", vlp_camtg2_7_parents,
    VLP_CLK_CFG_5, VLP_CLK_CFG_5_SET, VLP_CLK_CFG_5_CLR,
    HWV_CG_10_DONE, HWV_CG_10_SET, HWV_CG_10_CLR,
    0, 4, 7, VLP_CLK_CFG_UPDATE, TOP_MUX_CAMTG4_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_0, 11),
    MUX_GATE_HWV_FENC_CLR_SET_UPD(CLK_VLP_CAMTG5, "vlp_camtg5", vlp_camtg2_7_parents,
    VLP_CLK_CFG_5, VLP_CLK_CFG_5_SET, VLP_CLK_CFG_5_CLR,
    HWV_CG_10_DONE, HWV_CG_10_SET, HWV_CG_10_CLR,
    8, 4, 15, VLP_CLK_CFG_UPDATE, TOP_MUX_CAMTG5_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_0, 10),
    MUX_GATE_HWV_FENC_CLR_SET_UPD(CLK_VLP_CAMTG6, "vlp_camtg6", vlp_camtg2_7_parents,
    VLP_CLK_CFG_5, VLP_CLK_CFG_5_SET, VLP_CLK_CFG_5_CLR,
    HWV_CG_10_DONE, HWV_CG_10_SET, HWV_CG_10_CLR,
    16, 4, 23, VLP_CLK_CFG_UPDATE, TOP_MUX_CAMTG6_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_0, 9),
    MUX_GATE_HWV_FENC_CLR_SET_UPD(CLK_VLP_CAMTG7, "vlp_camtg7", vlp_camtg2_7_parents,
    VLP_CLK_CFG_5, VLP_CLK_CFG_5_SET, VLP_CLK_CFG_5_CLR,
    HWV_CG_10_DONE, HWV_CG_10_SET, HWV_CG_10_CLR,
    24, 4, 31, VLP_CLK_CFG_UPDATE, TOP_MUX_CAMTG7_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_0, 8),
// VLP_CLK_CFG_6
    MUX_CLR_SET_UPD(CLK_VLP_SSPM_26M, "vlp_sspm_26m",
    vlp_sspm_26m_parents, VLP_CLK_CFG_6, VLP_CLK_CFG_6_SET,
    VLP_CLK_CFG_6_CLR, 8, 1,
    VLP_CLK_CFG_UPDATE, TOP_MUX_SSPM_26M_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_ULPOSC_SSPM, "vlp_ulposc_sspm",
    vlp_ulposc_sspm_parents, VLP_CLK_CFG_6, VLP_CLK_CFG_6_SET,
    VLP_CLK_CFG_6_CLR, 16, 2,
    VLP_CLK_CFG_UPDATE, TOP_MUX_ULPOSC_SSPM_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_VLP_PBUS_26M, "vlp_vlp_pbus_26m",
    vlp_vlp_pbus_26m_parents, VLP_CLK_CFG_6, VLP_CLK_CFG_6_SET,
    VLP_CLK_CFG_6_CLR, 24, 1,
    VLP_CLK_CFG_UPDATE, TOP_MUX_VLP_PBUS_26M_SHIFT),
// VLP_CLK_CFG_7
    MUX_CLR_SET_UPD(CLK_VLP_DEBUG_ERR_FLAG, "vlp_debug_err_flag",
    vlp_debug_err_flag_parents, VLP_CLK_CFG_7, VLP_CLK_CFG_7_SET,
    VLP_CLK_CFG_7_CLR, 0, 1,
    VLP_CLK_CFG_UPDATE, TOP_MUX_DEBUG_ERR_FLAG_VLP_26M_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_DPMSRDMA, "vlp_dpmsrdma",
    vlp_dpmsrdma_parents, VLP_CLK_CFG_7, VLP_CLK_CFG_7_SET,
    VLP_CLK_CFG_7_CLR, 8, 1,
    VLP_CLK_CFG_UPDATE, TOP_MUX_DPMSRDMA_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_VLP_PBUS_156M, "vlp_vlp_pbus_156m",
    vlp_vlp_pbus_156m_parents, VLP_CLK_CFG_7, VLP_CLK_CFG_7_SET,
    VLP_CLK_CFG_7_CLR, 16, 2,
    VLP_CLK_CFG_UPDATE, TOP_MUX_VLP_PBUS_156M_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_SPM, "vlp_spm",
    vlp_spm_parents, VLP_CLK_CFG_7, VLP_CLK_CFG_7_SET,
    VLP_CLK_CFG_7_CLR, 24, 1,
    VLP_CLK_CFG_UPDATE1, TOP_MUX_SPM_SHIFT),
// VLP_CLK_CFG_8
    MUX_GATE_FENC_CLR_SET_UPD(CLK_VLP_MMINFRA, "vlp_mminfra", vlp_mminfra_parents,
    VLP_CLK_CFG_8, VLP_CLK_CFG_8_SET, VLP_CLK_CFG_8_CLR,
    0, 2, 7, VLP_CLK_CFG_UPDATE1, TOP_MUX_MMINFRA_VLP_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_1, 31),
    MUX_GATE_FENC_CLR_SET_UPD(CLK_VLP_USB_TOP, "vlp_usb", vlp_usb_parents,
    VLP_CLK_CFG_8, VLP_CLK_CFG_8_SET, VLP_CLK_CFG_8_CLR,
    8, 1, 15, VLP_CLK_CFG_UPDATE1, TOP_MUX_USB_TOP_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_1, 30),
    MUX_GATE_FENC_CLR_SET_UPD(CLK_VLP_USB_XHCI, "vlp_usb_xhci", vlp_usb_parents,
    VLP_CLK_CFG_8, VLP_CLK_CFG_8_SET, VLP_CLK_CFG_8_CLR,
    16, 1, 23, VLP_CLK_CFG_UPDATE1, TOP_MUX_SSUSB_XHCI_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_1, 29),
    MUX_CLR_SET_UPD(CLK_VLP_NOC_VLP, "vlp_noc_vlp",
    vlp_noc_vlp_parents, VLP_CLK_CFG_8, VLP_CLK_CFG_8_SET,
    VLP_CLK_CFG_8_CLR, 24, 2,
    VLP_CLK_CFG_UPDATE1, TOP_MUX_NOC_VLP_SHIFT),
// VLP_CLK_CFG_9
    MUX_GATE_FENC_CLR_SET_UPD_INDEXED(CLK_VLP_AUDIO_H, "vlp_audio_h",
    vlp_audio_h_parents, vlp_aud_parent_index,
    VLP_CLK_CFG_9, VLP_CLK_CFG_9_SET, VLP_CLK_CFG_9_CLR,
    0, 2, 7, VLP_CLK_CFG_UPDATE1, TOP_MUX_AUDIO_H_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_1, 27),
    MUX_GATE_FENC_CLR_SET_UPD_INDEXED(CLK_VLP_AUD_ENGEN1, "vlp_aud_engen1",
    vlp_aud_engen1_parents, vlp_aud_parent_index,
    VLP_CLK_CFG_9, VLP_CLK_CFG_9_SET, VLP_CLK_CFG_9_CLR,
    8, 2, 15, VLP_CLK_CFG_UPDATE1, TOP_MUX_AUD_ENGEN1_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_1, 26),
    MUX_GATE_FENC_CLR_SET_UPD_INDEXED(CLK_VLP_AUD_ENGEN2, "vlp_aud_engen2",
    vlp_aud_engen2_parents, vlp_aud_parent_index,
    VLP_CLK_CFG_9, VLP_CLK_CFG_9_SET, VLP_CLK_CFG_9_CLR,
    16, 2, 23, VLP_CLK_CFG_UPDATE1, TOP_MUX_AUD_ENGEN2_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_1, 25),
    MUX_GATE_FENC_CLR_SET_UPD_INDEXED(CLK_VLP_AUD_INTBUS, "vlp_aud_intbus",
    vlp_aud_intbus_parents, vlp_aud_parent_index,
    VLP_CLK_CFG_9, VLP_CLK_CFG_9_SET, VLP_CLK_CFG_9_CLR,
    24, 2, 31, VLP_CLK_CFG_UPDATE1, TOP_MUX_AUD_INTBUS_SHIFT,
    VLP_OCIC_FENC_STATUS_MON_1, 24),
// VLP_CLK_CFG_10
    MUX_CLR_SET_UPD(CLK_VLP_SPVLP_26M, "vlp_spvlp_26m",
    vlp_spvlp_26m_parents, VLP_CLK_CFG_10, VLP_CLK_CFG_10_SET,
    VLP_CLK_CFG_10_CLR, 0, 1,
    VLP_CLK_CFG_UPDATE1, TOP_MUX_SPU_VLP_26M_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_SPU0_VLP, "vlp_spu0_vlp",
    vlp_spu0_vlp_parents, VLP_CLK_CFG_10, VLP_CLK_CFG_10_SET,
    VLP_CLK_CFG_10_CLR, 8, 3,
    VLP_CLK_CFG_UPDATE1, TOP_MUX_SPU0_VLP_SHIFT),
    MUX_CLR_SET_UPD(CLK_VLP_SPU1_VLP, "vlp_spu1_vlp",
    vlp_spu1_vlp_parents, VLP_CLK_CFG_10, VLP_CLK_CFG_10_SET,
    VLP_CLK_CFG_10_CLR, 16, 3,
    VLP_CLK_CFG_UPDATE1, TOP_MUX_SPU1_VLP_SHIFT),
    };
    static const struct mtk_pll_data vlp_plls[] = {
    PLL_FENC(CLK_VLP_APLL1, "vlp_apll1", VLP_APLL1_CON0, 0x0358, 1, 0,
    VLP_APLL1_CON1, 24, VLP_APLL1_CON2, 0, 32, 0),
    PLL_FENC(CLK_VLP_APLL2, "vlp_apll2", VLP_APLL2_CON0, 0x0358, 0, 0,
    VLP_APLL2_CON1, 24, VLP_APLL2_CON2, 0, 32, 1),
    };
    static const struct regmap_config vlpckgen_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = 0x1000,
    };
#[no_mangle]
unsafe extern "C" fn clk_mt8196_vlp_probe(pdev: *mut platform_device) -> c_int {
    static int clk_mt8196_vlp_probe(struct platform_device *pdev)
    {
    static void __iomem *base;
    struct clk_hw_onecell_data *clk_data;
    int r;
    struct device_node *node = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    struct regmap *regmap;
    clk_data = mtk_alloc_clk_data(ARRAY_SIZE(vlp_muxes) +
    ARRAY_SIZE(vlp_plls) +
    ARRAY_SIZE(vlp_divs));
    if (!clk_data)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    regmap = devm_regmap_init_mmio(dev, base, &vlpckgen_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    r = mtk_clk_register_factors(vlp_divs, ARRAY_SIZE(vlp_divs), clk_data);
    if (r)
    goto free_clk_data;
    r = mtk_clk_register_muxes(&pdev.dev, vlp_muxes, ARRAY_SIZE(vlp_muxes),
    node, &mt8196_clk_vlp_lock, clk_data);
    if (r)
    goto unregister_factors;
    r = mtk_clk_register_plls(dev, vlp_plls, ARRAY_SIZE(vlp_plls),
    clk_data);
    if (r)
    goto unregister_muxes;
    r = of_clk_add_hw_provider(node, of_clk_hw_onecell_get, clk_data);
    if (r)
    goto unregister_plls;
    platform_set_drvdata(pdev, clk_data);
// Initialize APLL tuner registers
    regmap_write(regmap, VLP_APLL1_TUNER_CON0, VLP_APLL1_TUNER_CON0_VALUE);
    regmap_write(regmap, VLP_APLL2_TUNER_CON0, VLP_APLL2_TUNER_CON0_VALUE);
    return r;
    unregister_plls:
    mtk_clk_unregister_plls(vlp_plls, ARRAY_SIZE(vlp_plls), clk_data);
    unregister_muxes:
    mtk_clk_unregister_muxes(vlp_muxes, ARRAY_SIZE(vlp_muxes), clk_data);
    unregister_factors:
    mtk_clk_unregister_factors(vlp_divs, ARRAY_SIZE(vlp_divs), clk_data);
    free_clk_data:
    mtk_free_clk_data(clk_data);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn clk_mt8196_vlp_remove(pdev: *mut platform_device) {
    static void clk_mt8196_vlp_remove(struct platform_device *pdev)
    {
    struct clk_hw_onecell_data *clk_data = platform_get_drvdata(pdev);
    struct device_node *node = pdev.dev.of_node;
    of_clk_del_provider(node);
    mtk_clk_unregister_plls(vlp_plls, ARRAY_SIZE(vlp_plls), clk_data);
    mtk_clk_unregister_muxes(vlp_muxes, ARRAY_SIZE(vlp_muxes), clk_data);
    mtk_clk_unregister_factors(vlp_divs, ARRAY_SIZE(vlp_divs), clk_data);
    mtk_free_clk_data(clk_data);
    }
    static const struct of_device_id of_match_clk_mt8196_vlp_ck[] = {
    { .compatible = "mediatek,mt8196-vlpckgen" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8196_vlp_ck);
    static struct platform_driver clk_mt8196_vlp_drv = {
    .probe = clk_mt8196_vlp_probe,
    .remove = clk_mt8196_vlp_remove,
    .driver = {
    .name = "clk-mt8196-vlpck",
    .of_match_table = of_match_clk_mt8196_vlp_ck,
    },
    };
    MODULE_DESCRIPTION("MediaTek MT8196 VLP clock generator driver");
    module_platform_driver(clk_mt8196_vlp_drv);
    MODULE_LICENSE("GPL");

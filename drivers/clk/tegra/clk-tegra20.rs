//! Automatically rewritten from C to Rust
//! Source: drivers/clk/tegra/clk-tegra20.c
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
// Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
//

pub const MISC_CLK_ENB: c_uint = 0x48;
pub const OSC_CTRL: c_uint = 0x50;

pub const OSC_FREQ_DET: c_uint = 0x58;

pub const OSC_FREQ_DET_STATUS: c_uint = 0x5c;

pub const OSC_FREQ_DET_CNT_MASK: c_uint = 0xFFFFu;
pub const TEGRA20_CLK_PERIPH_BANKS: c_int = 3;
pub const PLLS_BASE: c_uint = 0xf0;
pub const PLLS_MISC: c_uint = 0xf4;
pub const PLLC_BASE: c_uint = 0x80;
pub const PLLC_MISC: c_uint = 0x8c;
pub const PLLM_BASE: c_uint = 0x90;
pub const PLLM_MISC: c_uint = 0x9c;
pub const PLLP_BASE: c_uint = 0xa0;
pub const PLLP_MISC: c_uint = 0xac;
pub const PLLA_BASE: c_uint = 0xb0;
pub const PLLA_MISC: c_uint = 0xbc;
pub const PLLU_BASE: c_uint = 0xc0;
pub const PLLU_MISC: c_uint = 0xcc;
pub const PLLD_BASE: c_uint = 0xd0;
pub const PLLD_MISC: c_uint = 0xdc;
pub const PLLX_BASE: c_uint = 0xe0;
pub const PLLX_MISC: c_uint = 0xe4;
pub const PLLE_BASE: c_uint = 0xe8;
pub const PLLE_MISC: c_uint = 0xec;

pub const PLL_MISC_LOCK_ENABLE: c_int = 18;
pub const PLLDU_MISC_LOCK_ENABLE: c_int = 22;
pub const PLLE_MISC_LOCK_ENABLE: c_int = 9;
pub const PLLC_OUT: c_uint = 0x84;
pub const PLLM_OUT: c_uint = 0x94;
pub const PLLP_OUTA: c_uint = 0xa4;
pub const PLLP_OUTB: c_uint = 0xa8;
pub const PLLA_OUT: c_uint = 0xb4;
pub const CCLK_BURST_POLICY: c_uint = 0x20;
pub const SUPER_CCLK_DIVIDER: c_uint = 0x24;
pub const SCLK_BURST_POLICY: c_uint = 0x28;
pub const SUPER_SCLK_DIVIDER: c_uint = 0x2c;
pub const CLK_SYSTEM_RATE: c_uint = 0x30;
pub const CCLK_BURST_POLICY_SHIFT: c_int = 28;
pub const CCLK_RUN_POLICY_SHIFT: c_int = 4;
pub const CCLK_IDLE_POLICY_SHIFT: c_int = 0;
pub const CCLK_IDLE_POLICY: c_int = 1;
pub const CCLK_RUN_POLICY: c_int = 2;
pub const CCLK_BURST_POLICY_PLLX: c_int = 8;
pub const CLK_SOURCE_I2S1: c_uint = 0x100;
pub const CLK_SOURCE_I2S2: c_uint = 0x104;
pub const CLK_SOURCE_PWM: c_uint = 0x110;
pub const CLK_SOURCE_SPI: c_uint = 0x114;
pub const CLK_SOURCE_XIO: c_uint = 0x120;
pub const CLK_SOURCE_TWC: c_uint = 0x12c;
pub const CLK_SOURCE_IDE: c_uint = 0x144;
pub const CLK_SOURCE_HDMI: c_uint = 0x18c;
pub const CLK_SOURCE_DISP1: c_uint = 0x138;
pub const CLK_SOURCE_DISP2: c_uint = 0x13c;
pub const CLK_SOURCE_CSITE: c_uint = 0x1d4;
pub const CLK_SOURCE_I2C1: c_uint = 0x124;
pub const CLK_SOURCE_I2C2: c_uint = 0x198;
pub const CLK_SOURCE_I2C3: c_uint = 0x1b8;
pub const CLK_SOURCE_DVC: c_uint = 0x128;
pub const CLK_SOURCE_UARTA: c_uint = 0x178;
pub const CLK_SOURCE_UARTB: c_uint = 0x17c;
pub const CLK_SOURCE_UARTC: c_uint = 0x1a0;
pub const CLK_SOURCE_UARTD: c_uint = 0x1c0;
pub const CLK_SOURCE_UARTE: c_uint = 0x1c4;
pub const CLK_SOURCE_EMC: c_uint = 0x19c;
pub const AUDIO_SYNC_CLK: c_uint = 0x38;
// Tegra CPU clock and reset control regs
pub const TEGRA_CLK_RST_CONTROLLER_CLK_CPU_CMPLX: c_uint = 0x4c;
pub const TEGRA_CLK_RST_CONTROLLER_RST_CPU_CMPLX_SET: c_uint = 0x340;
pub const TEGRA_CLK_RST_CONTROLLER_RST_CPU_CMPLX_CLR: c_uint = 0x344;

    static struct cpu_clk_suspend_context {
    u32 pllx_misc;
    u32 pllx_base;
    u32 cpu_burst;
    u32 clk_csite_src;
    u32 cclk_divider;
    } tegra20_cpu_clk_sctx;

    static void __iomem *clk_base;
    static void __iomem *pmc_base;

    _clk_num, _gate_flags, _clk_id)	\
    TEGRA_INIT_DATA(_name, core::ptr::null_mut(), core::ptr::null_mut(), _parents, _offset,	\
    30, 2, 0, 0, 8, 1, TEGRA_DIVIDER_ROUND_UP,	\
    _clk_num, \
    _gate_flags, _clk_id)

    _clk_num, _gate_flags, _clk_id)	\
    TEGRA_INIT_DATA(_name, core::ptr::null_mut(), core::ptr::null_mut(), _parents, _offset,	\
    30, 2, 0, 0, 16, 0, TEGRA_DIVIDER_ROUND_UP, \
    _clk_num, _gate_flags,	\
    _clk_id)

    _mux_shift, _mux_width, _clk_num, \
    _gate_flags, _clk_id)			\
    TEGRA_INIT_DATA(_name, core::ptr::null_mut(), core::ptr::null_mut(), _parents, _offset,	\
    _mux_shift, _mux_width, 0, 0, 0, 0, 0, \
    _clk_num, _gate_flags,	\
    _clk_id)
    static struct clk **clks;
    static struct tegra_clk_pll_freq_table pll_c_freq_table[] = {
    { 12000000, 600000000, 600, 12, 1, 8 },
    { 13000000, 600000000, 600, 13, 1, 8 },
    { 19200000, 600000000, 500, 16, 1, 6 },
    { 26000000, 600000000, 600, 26, 1, 8 },
    {        0,         0,   0,  0, 0, 0 },
    };
    static struct tegra_clk_pll_freq_table pll_m_freq_table[] = {
    { 12000000, 666000000, 666, 12, 1, 8 },
    { 13000000, 666000000, 666, 13, 1, 8 },
    { 19200000, 666000000, 555, 16, 1, 8 },
    { 26000000, 666000000, 666, 26, 1, 8 },
    { 12000000, 600000000, 600, 12, 1, 8 },
    { 13000000, 600000000, 600, 13, 1, 8 },
    { 19200000, 600000000, 375, 12, 1, 6 },
    { 26000000, 600000000, 600, 26, 1, 8 },
    {        0,         0,   0,  0, 0, 0 },
    };
    static struct tegra_clk_pll_freq_table pll_p_freq_table[] = {
    { 12000000, 216000000, 432, 12, 2, 8 },
    { 13000000, 216000000, 432, 13, 2, 8 },
    { 19200000, 216000000,  90,  4, 2, 1 },
    { 26000000, 216000000, 432, 26, 2, 8 },
    { 12000000, 432000000, 432, 12, 1, 8 },
    { 13000000, 432000000, 432, 13, 1, 8 },
    { 19200000, 432000000,  90,  4, 1, 1 },
    { 26000000, 432000000, 432, 26, 1, 8 },
    {        0,         0,   0,  0, 0, 0 },
    };
    static struct tegra_clk_pll_freq_table pll_a_freq_table[] = {
    { 28800000, 56448000, 49, 25, 1, 1 },
    { 28800000, 73728000, 64, 25, 1, 1 },
    { 28800000, 24000000,  5,  6, 1, 1 },
    {        0,        0,  0,  0, 0, 0 },
    };
    static struct tegra_clk_pll_freq_table pll_d_freq_table[] = {
    { 12000000,  216000000,  216, 12, 1,  4 },
    { 13000000,  216000000,  216, 13, 1,  4 },
    { 19200000,  216000000,  135, 12, 1,  3 },
    { 26000000,  216000000,  216, 26, 1,  4 },
    { 12000000,  594000000,  594, 12, 1,  8 },
    { 13000000,  594000000,  594, 13, 1,  8 },
    { 19200000,  594000000,  495, 16, 1,  8 },
    { 26000000,  594000000,  594, 26, 1,  8 },
    { 12000000, 1000000000, 1000, 12, 1, 12 },
    { 13000000, 1000000000, 1000, 13, 1, 12 },
    { 19200000, 1000000000,  625, 12, 1,  8 },
    { 26000000, 1000000000, 1000, 26, 1, 12 },
    {        0,          0,    0,  0, 0,  0 },
    };
    static struct tegra_clk_pll_freq_table pll_u_freq_table[] = {
    { 12000000, 480000000, 960, 12, 1, 0 },
    { 13000000, 480000000, 960, 13, 1, 0 },
    { 19200000, 480000000, 200,  4, 1, 0 },
    { 26000000, 480000000, 960, 26, 1, 0 },
    {        0,         0,   0,  0, 0, 0 },
    };
    static struct tegra_clk_pll_freq_table pll_x_freq_table[] = {
// 1 GHz
    { 12000000, 1000000000, 1000, 12, 1, 12 },
    { 13000000, 1000000000, 1000, 13, 1, 12 },
    { 19200000, 1000000000,  625, 12, 1,  8 },
    { 26000000, 1000000000, 1000, 26, 1, 12 },
// 912 MHz
    { 12000000,  912000000,  912, 12, 1, 12 },
    { 13000000,  912000000,  912, 13, 1, 12 },
    { 19200000,  912000000,  760, 16, 1,  8 },
    { 26000000,  912000000,  912, 26, 1, 12 },
// 816 MHz
    { 12000000,  816000000,  816, 12, 1, 12 },
    { 13000000,  816000000,  816, 13, 1, 12 },
    { 19200000,  816000000,  680, 16, 1,  8 },
    { 26000000,  816000000,  816, 26, 1, 12 },
// 760 MHz
    { 12000000,  760000000,  760, 12, 1, 12 },
    { 13000000,  760000000,  760, 13, 1, 12 },
    { 19200000,  760000000,  950, 24, 1,  8 },
    { 26000000,  760000000,  760, 26, 1, 12 },
// 750 MHz
    { 12000000,  750000000,  750, 12, 1, 12 },
    { 13000000,  750000000,  750, 13, 1, 12 },
    { 19200000,  750000000,  625, 16, 1,  8 },
    { 26000000,  750000000,  750, 26, 1, 12 },
// 608 MHz
    { 12000000,  608000000,  608, 12, 1, 12 },
    { 13000000,  608000000,  608, 13, 1, 12 },
    { 19200000,  608000000,  380, 12, 1,  8 },
    { 26000000,  608000000,  608, 26, 1, 12 },
// 456 MHz
    { 12000000,  456000000,  456, 12, 1, 12 },
    { 13000000,  456000000,  456, 13, 1, 12 },
    { 19200000,  456000000,  380, 16, 1,  8 },
    { 26000000,  456000000,  456, 26, 1, 12 },
// 312 MHz
    { 12000000,  312000000,  312, 12, 1, 12 },
    { 13000000,  312000000,  312, 13, 1, 12 },
    { 19200000,  312000000,  260, 16, 1,  8 },
    { 26000000,  312000000,  312, 26, 1, 12 },
    {        0,          0,    0,  0, 0,  0 },
    };
    static const struct pdiv_map plle_p[] = {
    { .pdiv = 1, .hw_val = 1 },
    { .pdiv = 0, .hw_val = 0 },
    };
    static struct tegra_clk_pll_freq_table pll_e_freq_table[] = {
    { 12000000, 100000000, 200, 24, 1, 0 },
    {        0,         0,   0,  0, 0, 0 },
    };
// PLL parameters
    static struct tegra_clk_pll_params pll_c_params = {
    .input_min = 2000000,
    .input_max = 31000000,
    .cf_min = 1000000,
    .cf_max = 6000000,
    .vco_min = 20000000,
    .vco_max = 1400000000,
    .base_reg = PLLC_BASE,
    .misc_reg = PLLC_MISC,
    .lock_mask = PLL_BASE_LOCK,
    .lock_enable_bit_idx = PLL_MISC_LOCK_ENABLE,
    .lock_delay = 300,
    .freq_table = pll_c_freq_table,
    .flags = TEGRA_PLL_HAS_CPCON | TEGRA_PLL_HAS_LOCK_ENABLE,
    };
    static struct tegra_clk_pll_params pll_m_params = {
    .input_min = 2000000,
    .input_max = 31000000,
    .cf_min = 1000000,
    .cf_max = 6000000,
    .vco_min = 20000000,
    .vco_max = 1200000000,
    .base_reg = PLLM_BASE,
    .misc_reg = PLLM_MISC,
    .lock_mask = PLL_BASE_LOCK,
    .lock_enable_bit_idx = PLL_MISC_LOCK_ENABLE,
    .lock_delay = 300,
    .freq_table = pll_m_freq_table,
    .flags = TEGRA_PLL_HAS_CPCON | TEGRA_PLL_HAS_LOCK_ENABLE,
    };
    static struct tegra_clk_pll_params pll_p_params = {
    .input_min = 2000000,
    .input_max = 31000000,
    .cf_min = 1000000,
    .cf_max = 6000000,
    .vco_min = 20000000,
    .vco_max = 1400000000,
    .base_reg = PLLP_BASE,
    .misc_reg = PLLP_MISC,
    .lock_mask = PLL_BASE_LOCK,
    .lock_enable_bit_idx = PLL_MISC_LOCK_ENABLE,
    .lock_delay = 300,
    .freq_table = pll_p_freq_table,
    .flags = TEGRA_PLL_FIXED | TEGRA_PLL_HAS_CPCON |
    TEGRA_PLL_HAS_LOCK_ENABLE,
    .fixed_rate =  216000000,
    };
    static struct tegra_clk_pll_params pll_a_params = {
    .input_min = 2000000,
    .input_max = 31000000,
    .cf_min = 1000000,
    .cf_max = 6000000,
    .vco_min = 20000000,
    .vco_max = 1400000000,
    .base_reg = PLLA_BASE,
    .misc_reg = PLLA_MISC,
    .lock_mask = PLL_BASE_LOCK,
    .lock_enable_bit_idx = PLL_MISC_LOCK_ENABLE,
    .lock_delay = 300,
    .freq_table = pll_a_freq_table,
    .flags = TEGRA_PLL_HAS_CPCON | TEGRA_PLL_HAS_LOCK_ENABLE,
    };
    static struct tegra_clk_pll_params pll_d_params = {
    .input_min = 2000000,
    .input_max = 40000000,
    .cf_min = 1000000,
    .cf_max = 6000000,
    .vco_min = 40000000,
    .vco_max = 1000000000,
    .base_reg = PLLD_BASE,
    .misc_reg = PLLD_MISC,
    .lock_mask = PLL_BASE_LOCK,
    .lock_enable_bit_idx = PLLDU_MISC_LOCK_ENABLE,
    .lock_delay = 1000,
    .freq_table = pll_d_freq_table,
    .flags = TEGRA_PLL_HAS_CPCON | TEGRA_PLL_HAS_LOCK_ENABLE,
    };
    static const struct pdiv_map pllu_p[] = {
    { .pdiv = 1, .hw_val = 1 },
    { .pdiv = 2, .hw_val = 0 },
    { .pdiv = 0, .hw_val = 0 },
    };
    static struct tegra_clk_pll_params pll_u_params = {
    .input_min = 2000000,
    .input_max = 40000000,
    .cf_min = 1000000,
    .cf_max = 6000000,
    .vco_min = 48000000,
    .vco_max = 960000000,
    .base_reg = PLLU_BASE,
    .misc_reg = PLLU_MISC,
    .lock_mask = PLL_BASE_LOCK,
    .lock_enable_bit_idx = PLLDU_MISC_LOCK_ENABLE,
    .lock_delay = 1000,
    .pdiv_tohw = pllu_p,
    .freq_table = pll_u_freq_table,
    .flags = TEGRA_PLLU | TEGRA_PLL_HAS_CPCON | TEGRA_PLL_HAS_LOCK_ENABLE,
    };
    static struct tegra_clk_pll_params pll_x_params = {
    .input_min = 2000000,
    .input_max = 31000000,
    .cf_min = 1000000,
    .cf_max = 6000000,
    .vco_min = 20000000,
    .vco_max = 1200000000,
    .base_reg = PLLX_BASE,
    .misc_reg = PLLX_MISC,
    .lock_mask = PLL_BASE_LOCK,
    .lock_enable_bit_idx = PLL_MISC_LOCK_ENABLE,
    .lock_delay = 300,
    .freq_table = pll_x_freq_table,
    .flags = TEGRA_PLL_HAS_CPCON | TEGRA_PLL_HAS_LOCK_ENABLE,
    .pre_rate_change = tegra_cclk_pre_pllx_rate_change,
    .post_rate_change = tegra_cclk_post_pllx_rate_change,
    };
    static struct tegra_clk_pll_params pll_e_params = {
    .input_min = 12000000,
    .input_max = 12000000,
    .cf_min = 0,
    .cf_max = 0,
    .vco_min = 0,
    .vco_max = 0,
    .base_reg = PLLE_BASE,
    .misc_reg = PLLE_MISC,
    .lock_mask = PLLE_MISC_LOCK,
    .lock_enable_bit_idx = PLLE_MISC_LOCK_ENABLE,
    .lock_delay = 0,
    .pdiv_tohw = plle_p,
    .freq_table = pll_e_freq_table,
    .flags = TEGRA_PLL_FIXED | TEGRA_PLL_LOCK_MISC |
    TEGRA_PLL_HAS_LOCK_ENABLE,
    .fixed_rate = 100000000,
    };
    static struct tegra_devclk devclks[] = {
    { .con_id = "pll_c", .dt_id = TEGRA20_CLK_PLL_C },
    { .con_id = "pll_c_out1", .dt_id = TEGRA20_CLK_PLL_C_OUT1 },
    { .con_id = "pll_p", .dt_id = TEGRA20_CLK_PLL_P },
    { .con_id = "pll_p_out1", .dt_id = TEGRA20_CLK_PLL_P_OUT1 },
    { .con_id = "pll_p_out2", .dt_id = TEGRA20_CLK_PLL_P_OUT2 },
    { .con_id = "pll_p_out3", .dt_id = TEGRA20_CLK_PLL_P_OUT3 },
    { .con_id = "pll_p_out4", .dt_id = TEGRA20_CLK_PLL_P_OUT4 },
    { .con_id = "pll_m", .dt_id = TEGRA20_CLK_PLL_M },
    { .con_id = "pll_m_out1", .dt_id = TEGRA20_CLK_PLL_M_OUT1 },
    { .con_id = "pll_x", .dt_id = TEGRA20_CLK_PLL_X },
    { .con_id = "pll_u", .dt_id = TEGRA20_CLK_PLL_U },
    { .con_id = "pll_d", .dt_id = TEGRA20_CLK_PLL_D },
    { .con_id = "pll_d_out0", .dt_id = TEGRA20_CLK_PLL_D_OUT0 },
    { .con_id = "pll_a", .dt_id = TEGRA20_CLK_PLL_A },
    { .con_id = "pll_a_out0", .dt_id = TEGRA20_CLK_PLL_A_OUT0 },
    { .con_id = "pll_e", .dt_id = TEGRA20_CLK_PLL_E },
    { .con_id = "cclk", .dt_id = TEGRA20_CLK_CCLK },
    { .con_id = "sclk", .dt_id = TEGRA20_CLK_SCLK },
    { .con_id = "hclk", .dt_id = TEGRA20_CLK_HCLK },
    { .con_id = "pclk", .dt_id = TEGRA20_CLK_PCLK },
    { .con_id = "fuse", .dt_id = TEGRA20_CLK_FUSE },
    { .con_id = "twd", .dt_id = TEGRA20_CLK_TWD },
    { .con_id = "audio", .dt_id = TEGRA20_CLK_AUDIO },
    { .con_id = "audio_2x", .dt_id = TEGRA20_CLK_AUDIO_2X },
    { .dev_id = "tegra20-ac97", .dt_id = TEGRA20_CLK_AC97 },
    { .dev_id = "tegra-apbdma", .dt_id = TEGRA20_CLK_APBDMA },
    { .dev_id = "rtc-tegra", .dt_id = TEGRA20_CLK_RTC },
    { .dev_id = "timer", .dt_id = TEGRA20_CLK_TIMER },
    { .dev_id = "tegra-kbc", .dt_id = TEGRA20_CLK_KBC },
    { .con_id = "csus", .dev_id =  "tegra_camera", .dt_id = TEGRA20_CLK_CSUS },
    { .con_id = "vcp", .dev_id = "tegra-avp", .dt_id = TEGRA20_CLK_VCP },
    { .con_id = "bsea", .dev_id = "tegra-avp", .dt_id = TEGRA20_CLK_BSEA },
    { .con_id = "bsev", .dev_id = "tegra-aes", .dt_id = TEGRA20_CLK_BSEV },
    { .con_id = "emc", .dt_id = TEGRA20_CLK_EMC },
    { .dev_id = "fsl-tegra-udc", .dt_id = TEGRA20_CLK_USBD },
    { .dev_id = "tegra-ehci.1", .dt_id = TEGRA20_CLK_USB2 },
    { .dev_id = "tegra-ehci.2", .dt_id = TEGRA20_CLK_USB3 },
    { .dev_id = "dsi", .dt_id = TEGRA20_CLK_DSI },
    { .con_id = "csi", .dev_id = "tegra_camera", .dt_id = TEGRA20_CLK_CSI },
    { .con_id = "isp", .dev_id = "tegra_camera", .dt_id = TEGRA20_CLK_ISP },
    { .con_id = "pex", .dt_id = TEGRA20_CLK_PEX },
    { .con_id = "afi", .dt_id = TEGRA20_CLK_AFI },
    { .con_id = "cdev1", .dt_id = TEGRA20_CLK_CDEV1 },
    { .con_id = "cdev2", .dt_id = TEGRA20_CLK_CDEV2 },
    { .con_id = "clk_32k", .dt_id = TEGRA20_CLK_CLK_32K },
    { .con_id = "clk_m", .dt_id = TEGRA20_CLK_CLK_M },
    { .con_id = "pll_ref", .dt_id = TEGRA20_CLK_PLL_REF },
    { .dev_id = "tegra20-i2s.0", .dt_id = TEGRA20_CLK_I2S1 },
    { .dev_id = "tegra20-i2s.1", .dt_id = TEGRA20_CLK_I2S2 },
    { .con_id = "spdif_out", .dev_id = "tegra20-spdif", .dt_id = TEGRA20_CLK_SPDIF_OUT },
    { .con_id = "spdif_in", .dev_id = "tegra20-spdif", .dt_id = TEGRA20_CLK_SPDIF_IN },
    { .dev_id = "spi_tegra.0", .dt_id = TEGRA20_CLK_SBC1 },
    { .dev_id = "spi_tegra.1", .dt_id = TEGRA20_CLK_SBC2 },
    { .dev_id = "spi_tegra.2", .dt_id = TEGRA20_CLK_SBC3 },
    { .dev_id = "spi_tegra.3", .dt_id = TEGRA20_CLK_SBC4 },
    { .dev_id = "spi", .dt_id = TEGRA20_CLK_SPI },
    { .dev_id = "xio", .dt_id = TEGRA20_CLK_XIO },
    { .dev_id = "twc", .dt_id = TEGRA20_CLK_TWC },
    { .dev_id = "ide", .dt_id = TEGRA20_CLK_IDE },
    { .dev_id = "tegra_nand", .dt_id = TEGRA20_CLK_NDFLASH },
    { .dev_id = "vfir", .dt_id = TEGRA20_CLK_VFIR },
    { .dev_id = "csite", .dt_id = TEGRA20_CLK_CSITE },
    { .dev_id = "la", .dt_id = TEGRA20_CLK_LA },
    { .dev_id = "tegra_w1", .dt_id = TEGRA20_CLK_OWR },
    { .dev_id = "mipi", .dt_id = TEGRA20_CLK_MIPI },
    { .dev_id = "vde", .dt_id = TEGRA20_CLK_VDE },
    { .con_id = "vi", .dev_id =  "tegra_camera", .dt_id = TEGRA20_CLK_VI },
    { .dev_id = "epp", .dt_id = TEGRA20_CLK_EPP },
    { .dev_id = "mpe", .dt_id = TEGRA20_CLK_MPE },
    { .dev_id = "host1x", .dt_id = TEGRA20_CLK_HOST1X },
    { .dev_id = "3d", .dt_id = TEGRA20_CLK_GR3D },
    { .dev_id = "2d", .dt_id = TEGRA20_CLK_GR2D },
    { .dev_id = "tegra-nor", .dt_id = TEGRA20_CLK_NOR },
    { .dev_id = "sdhci-tegra.0", .dt_id = TEGRA20_CLK_SDMMC1 },
    { .dev_id = "sdhci-tegra.1", .dt_id = TEGRA20_CLK_SDMMC2 },
    { .dev_id = "sdhci-tegra.2", .dt_id = TEGRA20_CLK_SDMMC3 },
    { .dev_id = "sdhci-tegra.3", .dt_id = TEGRA20_CLK_SDMMC4 },
    { .dev_id = "cve", .dt_id = TEGRA20_CLK_CVE },
    { .dev_id = "tvo", .dt_id = TEGRA20_CLK_TVO },
    { .dev_id = "tvdac", .dt_id = TEGRA20_CLK_TVDAC },
    { .con_id = "vi_sensor", .dev_id = "tegra_camera", .dt_id = TEGRA20_CLK_VI_SENSOR },
    { .dev_id = "hdmi", .dt_id = TEGRA20_CLK_HDMI },
    { .con_id = "div-clk", .dev_id = "tegra-i2c.0", .dt_id = TEGRA20_CLK_I2C1 },
    { .con_id = "div-clk", .dev_id = "tegra-i2c.1", .dt_id = TEGRA20_CLK_I2C2 },
    { .con_id = "div-clk", .dev_id = "tegra-i2c.2", .dt_id = TEGRA20_CLK_I2C3 },
    { .con_id = "div-clk", .dev_id = "tegra-i2c.3", .dt_id = TEGRA20_CLK_DVC },
    { .dev_id = "tegra-pwm", .dt_id = TEGRA20_CLK_PWM },
    { .dev_id = "tegra_uart.0", .dt_id = TEGRA20_CLK_UARTA },
    { .dev_id = "tegra_uart.1", .dt_id = TEGRA20_CLK_UARTB },
    { .dev_id = "tegra_uart.2", .dt_id = TEGRA20_CLK_UARTC },
    { .dev_id = "tegra_uart.3", .dt_id = TEGRA20_CLK_UARTD },
    { .dev_id = "tegra_uart.4", .dt_id = TEGRA20_CLK_UARTE },
    { .dev_id = "tegradc.0", .dt_id = TEGRA20_CLK_DISP1 },
    { .dev_id = "tegradc.1", .dt_id = TEGRA20_CLK_DISP2 },
    };
    static struct tegra_clk tegra20_clks[tegra_clk_max] __initdata = {
    [tegra_clk_ahbdma] = { .dt_id = TEGRA20_CLK_AHBDMA, .present = true },
    [tegra_clk_apbdma] = { .dt_id = TEGRA20_CLK_APBDMA, .present = true },
    [tegra_clk_spdif_out] = { .dt_id = TEGRA20_CLK_SPDIF_OUT, .present = true },
    [tegra_clk_spdif_in] = { .dt_id = TEGRA20_CLK_SPDIF_IN, .present = true },
    [tegra_clk_sdmmc1] = { .dt_id = TEGRA20_CLK_SDMMC1, .present = true },
    [tegra_clk_sdmmc2] = { .dt_id = TEGRA20_CLK_SDMMC2, .present = true },
    [tegra_clk_sdmmc3] = { .dt_id = TEGRA20_CLK_SDMMC3, .present = true },
    [tegra_clk_sdmmc4] = { .dt_id = TEGRA20_CLK_SDMMC4, .present = true },
    [tegra_clk_la] = { .dt_id = TEGRA20_CLK_LA, .present = true },
    [tegra_clk_csite] = { .dt_id = TEGRA20_CLK_CSITE, .present = true },
    [tegra_clk_vfir] = { .dt_id = TEGRA20_CLK_VFIR, .present = true },
    [tegra_clk_mipi] = { .dt_id = TEGRA20_CLK_MIPI, .present = true },
    [tegra_clk_nor] = { .dt_id = TEGRA20_CLK_NOR, .present = true },
    [tegra_clk_rtc] = { .dt_id = TEGRA20_CLK_RTC, .present = true },
    [tegra_clk_timer] = { .dt_id = TEGRA20_CLK_TIMER, .present = true },
    [tegra_clk_kbc] = { .dt_id = TEGRA20_CLK_KBC, .present = true },
    [tegra_clk_vcp] = { .dt_id = TEGRA20_CLK_VCP, .present = true },
    [tegra_clk_bsea] = { .dt_id = TEGRA20_CLK_BSEA, .present = true },
    [tegra_clk_bsev] = { .dt_id = TEGRA20_CLK_BSEV, .present = true },
    [tegra_clk_usbd] = { .dt_id = TEGRA20_CLK_USBD, .present = true },
    [tegra_clk_usb2] = { .dt_id = TEGRA20_CLK_USB2, .present = true },
    [tegra_clk_usb3] = { .dt_id = TEGRA20_CLK_USB3, .present = true },
    [tegra_clk_csi] = { .dt_id = TEGRA20_CLK_CSI, .present = true },
    [tegra_clk_isp] = { .dt_id = TEGRA20_CLK_ISP, .present = true },
    [tegra_clk_clk_32k] = { .dt_id = TEGRA20_CLK_CLK_32K, .present = true },
    [tegra_clk_hclk] = { .dt_id = TEGRA20_CLK_HCLK, .present = true },
    [tegra_clk_pclk] = { .dt_id = TEGRA20_CLK_PCLK, .present = true },
    [tegra_clk_pll_p_out1] = { .dt_id = TEGRA20_CLK_PLL_P_OUT1, .present = true },
    [tegra_clk_pll_p_out2] = { .dt_id = TEGRA20_CLK_PLL_P_OUT2, .present = true },
    [tegra_clk_pll_p_out3] = { .dt_id = TEGRA20_CLK_PLL_P_OUT3, .present = true },
    [tegra_clk_pll_p_out4] = { .dt_id = TEGRA20_CLK_PLL_P_OUT4, .present = true },
    [tegra_clk_pll_p] = { .dt_id = TEGRA20_CLK_PLL_P, .present = true },
    [tegra_clk_owr] = { .dt_id = TEGRA20_CLK_OWR, .present = true },
    [tegra_clk_sbc1] = { .dt_id = TEGRA20_CLK_SBC1, .present = true },
    [tegra_clk_sbc2] = { .dt_id = TEGRA20_CLK_SBC2, .present = true },
    [tegra_clk_sbc3] = { .dt_id = TEGRA20_CLK_SBC3, .present = true },
    [tegra_clk_sbc4] = { .dt_id = TEGRA20_CLK_SBC4, .present = true },
    [tegra_clk_vde] = { .dt_id = TEGRA20_CLK_VDE, .present = true },
    [tegra_clk_vi] = { .dt_id = TEGRA20_CLK_VI, .present = true },
    [tegra_clk_epp] = { .dt_id = TEGRA20_CLK_EPP, .present = true },
    [tegra_clk_mpe] = { .dt_id = TEGRA20_CLK_MPE, .present = true },
    [tegra_clk_host1x] = { .dt_id = TEGRA20_CLK_HOST1X, .present = true },
    [tegra_clk_gr2d] = { .dt_id = TEGRA20_CLK_GR2D, .present = true },
    [tegra_clk_gr3d] = { .dt_id = TEGRA20_CLK_GR3D, .present = true },
    [tegra_clk_ndflash] = { .dt_id = TEGRA20_CLK_NDFLASH, .present = true },
    [tegra_clk_cve] = { .dt_id = TEGRA20_CLK_CVE, .present = true },
    [tegra_clk_tvo] = { .dt_id = TEGRA20_CLK_TVO, .present = true },
    [tegra_clk_tvdac] = { .dt_id = TEGRA20_CLK_TVDAC, .present = true },
    [tegra_clk_vi_sensor] = { .dt_id = TEGRA20_CLK_VI_SENSOR, .present = true },
    [tegra_clk_afi] = { .dt_id = TEGRA20_CLK_AFI, .present = true },
    [tegra_clk_fuse] = { .dt_id = TEGRA20_CLK_FUSE, .present = true },
    [tegra_clk_kfuse] = { .dt_id = TEGRA20_CLK_KFUSE, .present = true },
    };
#[no_mangle]
unsafe extern "C" fn tegra20_clk_measure_input_freq() -> c_ulong {
    static unsigned long tegra20_clk_measure_input_freq(void)
    {
    let mut osc_ctrl: u32 = readl_relaxed(clk_base + OSC_CTRL);
    let mut auto_clk_control: u32 = osc_ctrl & OSC_CTRL_OSC_FREQ_MASK;
    let mut pll_ref_div: u32 = osc_ctrl & OSC_CTRL_PLL_REF_DIV_MASK;
    unsigned long input_freq;
    switch (auto_clk_control) {
    case OSC_CTRL_OSC_FREQ_12MHZ:
    BUG_ON(pll_ref_div != OSC_CTRL_PLL_REF_DIV_1);
    input_freq = 12000000;
    break;
    case OSC_CTRL_OSC_FREQ_13MHZ:
    BUG_ON(pll_ref_div != OSC_CTRL_PLL_REF_DIV_1);
    input_freq = 13000000;
    break;
    case OSC_CTRL_OSC_FREQ_19_2MHZ:
    BUG_ON(pll_ref_div != OSC_CTRL_PLL_REF_DIV_1);
    input_freq = 19200000;
    break;
    case OSC_CTRL_OSC_FREQ_26MHZ:
    BUG_ON(pll_ref_div != OSC_CTRL_PLL_REF_DIV_1);
    input_freq = 26000000;
    break;
    default:
    pr_err("Unexpected clock autodetect value %d",
    auto_clk_control);
    BUG();
    return 0;
    }
    return input_freq;
    }
#[no_mangle]
unsafe extern "C" fn tegra20_get_pll_ref_div() -> c_uint {
    static unsigned int tegra20_get_pll_ref_div(void)
    {
    u32 pll_ref_div = readl_relaxed(clk_base + OSC_CTRL) &
    OSC_CTRL_PLL_REF_DIV_MASK;
    switch (pll_ref_div) {
    case OSC_CTRL_PLL_REF_DIV_1:
    return 1;
    case OSC_CTRL_PLL_REF_DIV_2:
    return 2;
    case OSC_CTRL_PLL_REF_DIV_4:
    return 4;
    default:
    pr_err("Invalid pll ref divider %d\n", pll_ref_div);
    BUG();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra20_pll_init() {
    static void tegra20_pll_init(void)
    {
    struct clk *clk;
// PLLC
    clk = tegra_clk_register_pll("pll_c", "pll_ref", clk_base, core::ptr::null_mut(), 0,
    &pll_c_params, core::ptr::null_mut());
    clks[TEGRA20_CLK_PLL_C] = clk;
// PLLC_OUT1
    clk = tegra_clk_register_divider("pll_c_out1_div", "pll_c",
    clk_base + PLLC_OUT, 0, TEGRA_DIVIDER_ROUND_UP,
    8, 8, 1, core::ptr::null_mut());
    clk = tegra_clk_register_pll_out("pll_c_out1", "pll_c_out1_div",
    clk_base + PLLC_OUT, 1, 0, CLK_SET_RATE_PARENT,
    0, core::ptr::null_mut());
    clks[TEGRA20_CLK_PLL_C_OUT1] = clk;
// PLLM
    clk = tegra_clk_register_pll("pll_m", "pll_ref", clk_base, core::ptr::null_mut(),
    CLK_SET_RATE_GATE, &pll_m_params, core::ptr::null_mut());
    clks[TEGRA20_CLK_PLL_M] = clk;
// PLLM_OUT1
    clk = tegra_clk_register_divider("pll_m_out1_div", "pll_m",
    clk_base + PLLM_OUT, 0, TEGRA_DIVIDER_ROUND_UP,
    8, 8, 1, core::ptr::null_mut());
    clk = tegra_clk_register_pll_out("pll_m_out1", "pll_m_out1_div",
    clk_base + PLLM_OUT, 1, 0,
    CLK_SET_RATE_PARENT, 0, core::ptr::null_mut());
    clks[TEGRA20_CLK_PLL_M_OUT1] = clk;
// PLLX
    clk = tegra_clk_register_pll("pll_x", "pll_ref", clk_base, core::ptr::null_mut(), 0,
    &pll_x_params, core::ptr::null_mut());
    clks[TEGRA20_CLK_PLL_X] = clk;
// PLLU
    clk = tegra_clk_register_pll("pll_u", "pll_ref", clk_base, core::ptr::null_mut(), 0,
    &pll_u_params, core::ptr::null_mut());
    clks[TEGRA20_CLK_PLL_U] = clk;
// PLLD
    clk = tegra_clk_register_pll("pll_d", "pll_ref", clk_base, core::ptr::null_mut(), 0,
    &pll_d_params, core::ptr::null_mut());
    clks[TEGRA20_CLK_PLL_D] = clk;
// PLLD_OUT0
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "pll_d_out0", "pll_d",
    CLK_SET_RATE_PARENT, 1, 2);
    clks[TEGRA20_CLK_PLL_D_OUT0] = clk;
// PLLA
    clk = tegra_clk_register_pll("pll_a", "pll_p_out1", clk_base, core::ptr::null_mut(), 0,
    &pll_a_params, core::ptr::null_mut());
    clks[TEGRA20_CLK_PLL_A] = clk;
// PLLA_OUT0
    clk = tegra_clk_register_divider("pll_a_out0_div", "pll_a",
    clk_base + PLLA_OUT, 0, TEGRA_DIVIDER_ROUND_UP,
    8, 8, 1, core::ptr::null_mut());
    clk = tegra_clk_register_pll_out("pll_a_out0", "pll_a_out0_div",
    clk_base + PLLA_OUT, 1, 0, CLK_IGNORE_UNUSED |
    CLK_SET_RATE_PARENT, 0, core::ptr::null_mut());
    clks[TEGRA20_CLK_PLL_A_OUT0] = clk;
// PLLE
    clk = tegra_clk_register_plle("pll_e", "pll_ref", clk_base, pmc_base,
    0, &pll_e_params, core::ptr::null_mut());
    clks[TEGRA20_CLK_PLL_E] = clk;
    }
    static const char *cclk_parents[] = { "clk_m", "pll_c", "clk_32k", "pll_m",
    "pll_p", "pll_p_out4",
    "pll_p_out3", "clk_d", "pll_x" };
    static const char *sclk_parents[] = { "clk_m", "pll_c_out1", "pll_p_out4",
    "pll_p_out3", "pll_p_out2", "clk_d",
    "clk_32k", "pll_m_out1" };
#[no_mangle]
unsafe extern "C" fn tegra20_super_clk_init() {
    static void tegra20_super_clk_init(void)
    {
    struct clk *clk;
// CCLK
    clk = tegra_clk_register_super_cclk("cclk", cclk_parents,
    ARRAY_SIZE(cclk_parents), CLK_SET_RATE_PARENT,
    clk_base + CCLK_BURST_POLICY, TEGRA20_SUPER_CLK,
    core::ptr::null_mut());
    clks[TEGRA20_CLK_CCLK] = clk;
// twd
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "twd", "cclk", 0, 1, 4);
    clks[TEGRA20_CLK_TWD] = clk;
    }
    static const char *audio_parents[] = { "spdif_in", "i2s1", "i2s2", "unused",
    "pll_a_out0", "unused", "unused",
    "unused" };
#[no_mangle]
unsafe extern "C" fn tegra20_audio_clk_init() -> void __init {
    static void __init tegra20_audio_clk_init(void)
    {
    struct clk *clk;
// audio
    clk = clk_register_mux(core::ptr::null_mut(), "audio_mux", audio_parents,
    ARRAY_SIZE(audio_parents),
    CLK_SET_RATE_NO_REPARENT,
    clk_base + AUDIO_SYNC_CLK, 0, 3, 0, core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "audio", "audio_mux", 0,
    clk_base + AUDIO_SYNC_CLK, 4,
    CLK_GATE_SET_TO_DISABLE, core::ptr::null_mut());
    clks[TEGRA20_CLK_AUDIO] = clk;
// audio_2x
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "audio_doubler", "audio",
    CLK_SET_RATE_PARENT, 2, 1);
    clk = tegra_clk_register_periph_gate("audio_2x", "audio_doubler",
    TEGRA_PERIPH_NO_RESET, clk_base,
    CLK_SET_RATE_PARENT, 89,
    periph_clk_enb_refcnt);
    clks[TEGRA20_CLK_AUDIO_2X] = clk;
    }
    static const char *i2s1_parents[] = { "pll_a_out0", "audio_2x", "pll_p",
    "clk_m" };
    static const char *i2s2_parents[] = { "pll_a_out0", "audio_2x", "pll_p",
    "clk_m" };
    static const char *pwm_parents[] = { "pll_p", "pll_c", "audio", "clk_m",
    "clk_32k" };
    static const char *mux_pllpcm_clkm[] = { "pll_p", "pll_c", "pll_m", "clk_m" };
    static const char *mux_pllpdc_clkm[] = { "pll_p", "pll_d_out0", "pll_c",
    "clk_m" };
    static struct tegra_periph_init_data tegra_periph_clk_list[] = {
    TEGRA_INIT_DATA_MUX("i2s1", i2s1_parents,     CLK_SOURCE_I2S1,   11, TEGRA_PERIPH_ON_APB, TEGRA20_CLK_I2S1),
    TEGRA_INIT_DATA_MUX("i2s2", i2s2_parents,     CLK_SOURCE_I2S2,   18, TEGRA_PERIPH_ON_APB, TEGRA20_CLK_I2S2),
    TEGRA_INIT_DATA_MUX("spi",   mux_pllpcm_clkm,   CLK_SOURCE_SPI,   43, TEGRA_PERIPH_ON_APB, TEGRA20_CLK_SPI),
    TEGRA_INIT_DATA_MUX("xio",   mux_pllpcm_clkm,   CLK_SOURCE_XIO,   45, 0, TEGRA20_CLK_XIO),
    TEGRA_INIT_DATA_MUX("twc",   mux_pllpcm_clkm,   CLK_SOURCE_TWC,   16, TEGRA_PERIPH_ON_APB, TEGRA20_CLK_TWC),
    TEGRA_INIT_DATA_MUX("ide",   mux_pllpcm_clkm,   CLK_SOURCE_XIO,   25, 0, TEGRA20_CLK_IDE),
    TEGRA_INIT_DATA_DIV16("dvc", mux_pllpcm_clkm,   CLK_SOURCE_DVC,   47, TEGRA_PERIPH_ON_APB, TEGRA20_CLK_DVC),
    TEGRA_INIT_DATA_DIV16("i2c1", mux_pllpcm_clkm,   CLK_SOURCE_I2C1,   12, TEGRA_PERIPH_ON_APB, TEGRA20_CLK_I2C1),
    TEGRA_INIT_DATA_DIV16("i2c2", mux_pllpcm_clkm,   CLK_SOURCE_I2C2,   54, TEGRA_PERIPH_ON_APB, TEGRA20_CLK_I2C2),
    TEGRA_INIT_DATA_DIV16("i2c3", mux_pllpcm_clkm,   CLK_SOURCE_I2C3,   67, TEGRA_PERIPH_ON_APB, TEGRA20_CLK_I2C3),
    TEGRA_INIT_DATA_MUX("hdmi", mux_pllpdc_clkm,   CLK_SOURCE_HDMI,   51, 0, TEGRA20_CLK_HDMI),
    TEGRA_INIT_DATA("pwm", core::ptr::null_mut(), core::ptr::null_mut(), pwm_parents,     CLK_SOURCE_PWM,   28, 3, 0, 0, 8, 1, 0, 17, TEGRA_PERIPH_ON_APB, TEGRA20_CLK_PWM),
    };
    static struct tegra_periph_init_data tegra_periph_nodiv_clk_list[] = {
    TEGRA_INIT_DATA_NODIV("uarta",	mux_pllpcm_clkm, CLK_SOURCE_UARTA, 30, 2, 6,   TEGRA_PERIPH_ON_APB, TEGRA20_CLK_UARTA),
    TEGRA_INIT_DATA_NODIV("uartb",	mux_pllpcm_clkm, CLK_SOURCE_UARTB, 30, 2, 7,   TEGRA_PERIPH_ON_APB, TEGRA20_CLK_UARTB),
    TEGRA_INIT_DATA_NODIV("uartc",	mux_pllpcm_clkm, CLK_SOURCE_UARTC, 30, 2, 55,  TEGRA_PERIPH_ON_APB, TEGRA20_CLK_UARTC),
    TEGRA_INIT_DATA_NODIV("uartd",	mux_pllpcm_clkm, CLK_SOURCE_UARTD, 30, 2, 65,  TEGRA_PERIPH_ON_APB, TEGRA20_CLK_UARTD),
    TEGRA_INIT_DATA_NODIV("uarte",	mux_pllpcm_clkm, CLK_SOURCE_UARTE, 30, 2, 66,  TEGRA_PERIPH_ON_APB, TEGRA20_CLK_UARTE),
    TEGRA_INIT_DATA_NODIV("disp1",	mux_pllpdc_clkm, CLK_SOURCE_DISP1, 30, 2, 27,  0, TEGRA20_CLK_DISP1),
    TEGRA_INIT_DATA_NODIV("disp2",	mux_pllpdc_clkm, CLK_SOURCE_DISP2, 30, 2, 26,  0, TEGRA20_CLK_DISP2),
    };
#[no_mangle]
unsafe extern "C" fn tegra20_periph_clk_init() -> void __init {
    static void __init tegra20_periph_clk_init(void)
    {
    struct tegra_periph_init_data *data;
    struct clk *clk;
    unsigned int i;
// ac97
    clk = tegra_clk_register_periph_gate("ac97", "pll_a_out0",
    TEGRA_PERIPH_ON_APB,
    clk_base, 0, 3, periph_clk_enb_refcnt);
    clks[TEGRA20_CLK_AC97] = clk;
// emc
    clk = tegra20_clk_register_emc(clk_base + CLK_SOURCE_EMC, false);
    clks[TEGRA20_CLK_EMC] = clk;
    clk = tegra_clk_register_mc("mc", "emc", clk_base + CLK_SOURCE_EMC,
    core::ptr::null_mut());
    clks[TEGRA20_CLK_MC] = clk;
// dsi
    clk = tegra_clk_register_periph_gate("dsi", "pll_d_out0", 0,
    clk_base, 0, TEGRA20_CLK_DSI,
    periph_clk_enb_refcnt);
    clks[TEGRA20_CLK_DSI] = clk;
// pex
    clk = tegra_clk_register_periph_gate("pex", "clk_m", 0, clk_base, 0, 70,
    periph_clk_enb_refcnt);
    clks[TEGRA20_CLK_PEX] = clk;
// dev1 OSC divider
    clk_register_divider(core::ptr::null_mut(), "dev1_osc_div", "clk_m",
    0, clk_base + MISC_CLK_ENB, 22, 2,
    CLK_DIVIDER_POWER_OF_TWO | CLK_DIVIDER_READ_ONLY,
    core::ptr::null_mut());
// dev2 OSC divider
    clk_register_divider(core::ptr::null_mut(), "dev2_osc_div", "clk_m",
    0, clk_base + MISC_CLK_ENB, 20, 2,
    CLK_DIVIDER_POWER_OF_TWO | CLK_DIVIDER_READ_ONLY,
    core::ptr::null_mut());
// cdev1
    clk = tegra_clk_register_periph_gate("cdev1", "cdev1_mux", 0,
    clk_base, 0, 94, periph_clk_enb_refcnt);
    clks[TEGRA20_CLK_CDEV1] = clk;
// cdev2
    clk = tegra_clk_register_periph_gate("cdev2", "cdev2_mux", 0,
    clk_base, 0, 93, periph_clk_enb_refcnt);
    clks[TEGRA20_CLK_CDEV2] = clk;
// csus
    clk = tegra_clk_register_periph_gate("csus", "csus_mux", 0,
    clk_base, 0, TEGRA20_CLK_CSUS,
    periph_clk_enb_refcnt);
    clks[TEGRA20_CLK_CSUS] = clk;
    for (i = 0; i < ARRAY_SIZE(tegra_periph_clk_list); i++) {
    data = &tegra_periph_clk_list[i];
    clk = tegra_clk_register_periph_data(clk_base, data);
    clks[data.clk_id] = clk;
    }
    for (i = 0; i < ARRAY_SIZE(tegra_periph_nodiv_clk_list); i++) {
    data = &tegra_periph_nodiv_clk_list[i];
    clk = tegra_clk_register_periph_nodiv(data.name,
    data.p.parent_names,
    data.num_parents, &data.periph,
    clk_base, data.offset);
    clks[data.clk_id] = clk;
    }
    tegra_periph_clk_init(clk_base, pmc_base, tegra20_clks, &pll_p_params);
    }
#[no_mangle]
unsafe extern "C" fn tegra20_osc_clk_init() -> void __init {
    static void __init tegra20_osc_clk_init(void)
    {
    struct clk *clk;
    unsigned long input_freq;
    unsigned int pll_ref_div;
    input_freq = tegra20_clk_measure_input_freq();
// clk_m
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "clk_m", core::ptr::null_mut(), CLK_IGNORE_UNUSED,
    input_freq);
    clks[TEGRA20_CLK_CLK_M] = clk;
// pll_ref
    pll_ref_div = tegra20_get_pll_ref_div();
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "pll_ref", "clk_m",
    CLK_SET_RATE_PARENT, 1, pll_ref_div);
    clks[TEGRA20_CLK_PLL_REF] = clk;
    }
// Tegra20 CPU clock and reset control functions
#[no_mangle]
unsafe extern "C" fn tegra20_wait_cpu_in_reset(cpu: u32) {
    static void tegra20_wait_cpu_in_reset(u32 cpu)
    {
    unsigned int reg;
    do {
    reg = readl(clk_base +
    TEGRA_CLK_RST_CONTROLLER_RST_CPU_CMPLX_SET);
    cpu_relax();
    } while (!(reg & (1 << cpu)));	/* check CPU been reset or not */
    return;
    }
#[no_mangle]
unsafe extern "C" fn tegra20_put_cpu_in_reset(cpu: u32) {
    static void tegra20_put_cpu_in_reset(u32 cpu)
    {
    writel(CPU_RESET(cpu),
    clk_base + TEGRA_CLK_RST_CONTROLLER_RST_CPU_CMPLX_SET);
    dmb();
    }
#[no_mangle]
unsafe extern "C" fn tegra20_cpu_out_of_reset(cpu: u32) {
    static void tegra20_cpu_out_of_reset(u32 cpu)
    {
    writel(CPU_RESET(cpu),
    clk_base + TEGRA_CLK_RST_CONTROLLER_RST_CPU_CMPLX_CLR);
    wmb();
    }
#[no_mangle]
unsafe extern "C" fn tegra20_enable_cpu_clock(cpu: u32) {
    static void tegra20_enable_cpu_clock(u32 cpu)
    {
    unsigned int reg;
    reg = readl(clk_base + TEGRA_CLK_RST_CONTROLLER_CLK_CPU_CMPLX);
    writel(reg & ~CPU_CLOCK(cpu),
    clk_base + TEGRA_CLK_RST_CONTROLLER_CLK_CPU_CMPLX);
    barrier();
    reg = readl(clk_base + TEGRA_CLK_RST_CONTROLLER_CLK_CPU_CMPLX);
    }
#[no_mangle]
unsafe extern "C" fn tegra20_disable_cpu_clock(cpu: u32) {
    static void tegra20_disable_cpu_clock(u32 cpu)
    {
    unsigned int reg;
    reg = readl(clk_base + TEGRA_CLK_RST_CONTROLLER_CLK_CPU_CMPLX);
    writel(reg | CPU_CLOCK(cpu),
    clk_base + TEGRA_CLK_RST_CONTROLLER_CLK_CPU_CMPLX);
    }

#[no_mangle]
unsafe extern "C" fn tegra20_cpu_rail_off_ready() -> bool {
    static bool tegra20_cpu_rail_off_ready(void)
    {
    unsigned int cpu_rst_status;
    cpu_rst_status = readl(clk_base +
    TEGRA_CLK_RST_CONTROLLER_RST_CPU_CMPLX_SET);
    return !!(cpu_rst_status & 0x2);
    }
#[no_mangle]
unsafe extern "C" fn tegra20_cpu_clock_suspend() {
    static void tegra20_cpu_clock_suspend(void)
    {
// switch coresite to clk_m, save off original source
    tegra20_cpu_clk_sctx.clk_csite_src =
    readl(clk_base + CLK_SOURCE_CSITE);
    writel(3<<30, clk_base + CLK_SOURCE_CSITE);
    tegra20_cpu_clk_sctx.cpu_burst =
    readl(clk_base + CCLK_BURST_POLICY);
    tegra20_cpu_clk_sctx.pllx_base =
    readl(clk_base + PLLX_BASE);
    tegra20_cpu_clk_sctx.pllx_misc =
    readl(clk_base + PLLX_MISC);
    tegra20_cpu_clk_sctx.cclk_divider =
    readl(clk_base + SUPER_CCLK_DIVIDER);
    }
#[no_mangle]
unsafe extern "C" fn tegra20_cpu_clock_resume() {
    static void tegra20_cpu_clock_resume(void)
    {
    unsigned int reg, policy;
    u32 misc, base;
// Is CPU complex already running on PLLX?
    reg = readl(clk_base + CCLK_BURST_POLICY);
    policy = (reg >> CCLK_BURST_POLICY_SHIFT) & 0xF;
    if (policy == CCLK_IDLE_POLICY)
    reg = (reg >> CCLK_IDLE_POLICY_SHIFT) & 0xF;
#[no_mangle]
pub unsafe extern "C" fn if(CCLK_RUN_POLICY: policy ==) -> else {
    else if (policy == CCLK_RUN_POLICY)
    reg = (reg >> CCLK_RUN_POLICY_SHIFT) & 0xF;
    else
    BUG();
    if (reg != CCLK_BURST_POLICY_PLLX) {
    misc = readl_relaxed(clk_base + PLLX_MISC);
    base = readl_relaxed(clk_base + PLLX_BASE);
    if (misc != tegra20_cpu_clk_sctx.pllx_misc ||
    base != tegra20_cpu_clk_sctx.pllx_base) {
// restore PLLX settings if CPU is on different PLL
    writel(tegra20_cpu_clk_sctx.pllx_misc,
    clk_base + PLLX_MISC);
    writel(tegra20_cpu_clk_sctx.pllx_base,
    clk_base + PLLX_BASE);
// wait for PLL stabilization if PLLX was enabled
    if (tegra20_cpu_clk_sctx.pllx_base & (1 << 30))
    udelay(300);
    }
    }
//
// Restore original burst policy setting for calls resulting from CPU
// LP2 in idle or system suspend.
//
    writel(tegra20_cpu_clk_sctx.cclk_divider,
    clk_base + SUPER_CCLK_DIVIDER);
    writel(tegra20_cpu_clk_sctx.cpu_burst,
    clk_base + CCLK_BURST_POLICY);
    writel(tegra20_cpu_clk_sctx.clk_csite_src,
    clk_base + CLK_SOURCE_CSITE);
    }

    static struct tegra_cpu_car_ops tegra20_cpu_car_ops = {
    .wait_for_reset	= tegra20_wait_cpu_in_reset,
    .put_in_reset	= tegra20_put_cpu_in_reset,
    .out_of_reset	= tegra20_cpu_out_of_reset,
    .enable_clock	= tegra20_enable_cpu_clock,
    .disable_clock	= tegra20_disable_cpu_clock,

    .rail_off_ready = tegra20_cpu_rail_off_ready,
    .suspend	= tegra20_cpu_clock_suspend,
    .resume		= tegra20_cpu_clock_resume,

    };
    static struct tegra_clk_init_table init_table[] = {
    { TEGRA20_CLK_PLL_P, TEGRA20_CLK_CLK_MAX, 216000000, 1 },
    { TEGRA20_CLK_PLL_P_OUT1, TEGRA20_CLK_CLK_MAX, 28800000, 1 },
    { TEGRA20_CLK_PLL_P_OUT2, TEGRA20_CLK_CLK_MAX, 48000000, 1 },
    { TEGRA20_CLK_PLL_P_OUT3, TEGRA20_CLK_CLK_MAX, 72000000, 1 },
    { TEGRA20_CLK_PLL_P_OUT4, TEGRA20_CLK_CLK_MAX, 24000000, 1 },
    { TEGRA20_CLK_PLL_C, TEGRA20_CLK_CLK_MAX, 600000000, 0 },
    { TEGRA20_CLK_PLL_C_OUT1, TEGRA20_CLK_CLK_MAX, 120000000, 0 },
    { TEGRA20_CLK_SCLK, TEGRA20_CLK_PLL_C_OUT1, 120000000, 0 },
    { TEGRA20_CLK_HCLK, TEGRA20_CLK_CLK_MAX, 120000000, 0 },
    { TEGRA20_CLK_PCLK, TEGRA20_CLK_CLK_MAX, 60000000, 0 },
    { TEGRA20_CLK_CSITE, TEGRA20_CLK_CLK_MAX, 0, 1 },
    { TEGRA20_CLK_CCLK, TEGRA20_CLK_CLK_MAX, 0, 1 },
    { TEGRA20_CLK_UARTA, TEGRA20_CLK_PLL_P, 0, 0 },
    { TEGRA20_CLK_UARTB, TEGRA20_CLK_PLL_P, 0, 0 },
    { TEGRA20_CLK_UARTC, TEGRA20_CLK_PLL_P, 0, 0 },
    { TEGRA20_CLK_UARTD, TEGRA20_CLK_PLL_P, 0, 0 },
    { TEGRA20_CLK_UARTE, TEGRA20_CLK_PLL_P, 0, 0 },
    { TEGRA20_CLK_PLL_A, TEGRA20_CLK_CLK_MAX, 56448000, 0 },
    { TEGRA20_CLK_PLL_A_OUT0, TEGRA20_CLK_CLK_MAX, 11289600, 0 },
    { TEGRA20_CLK_I2S1, TEGRA20_CLK_PLL_A_OUT0, 11289600, 0 },
    { TEGRA20_CLK_I2S2, TEGRA20_CLK_PLL_A_OUT0, 11289600, 0 },
    { TEGRA20_CLK_SDMMC1, TEGRA20_CLK_PLL_P, 48000000, 0 },
    { TEGRA20_CLK_SDMMC3, TEGRA20_CLK_PLL_P, 48000000, 0 },
    { TEGRA20_CLK_SDMMC4, TEGRA20_CLK_PLL_P, 48000000, 0 },
    { TEGRA20_CLK_SPI, TEGRA20_CLK_PLL_P, 20000000, 0 },
    { TEGRA20_CLK_SBC1, TEGRA20_CLK_PLL_P, 100000000, 0 },
    { TEGRA20_CLK_SBC2, TEGRA20_CLK_PLL_P, 100000000, 0 },
    { TEGRA20_CLK_SBC3, TEGRA20_CLK_PLL_P, 100000000, 0 },
    { TEGRA20_CLK_SBC4, TEGRA20_CLK_PLL_P, 100000000, 0 },
    { TEGRA20_CLK_HOST1X, TEGRA20_CLK_PLL_C, 150000000, 0 },
    { TEGRA20_CLK_GR2D, TEGRA20_CLK_PLL_C, 300000000, 0 },
    { TEGRA20_CLK_GR3D, TEGRA20_CLK_PLL_C, 300000000, 0 },
    { TEGRA20_CLK_VDE, TEGRA20_CLK_PLL_C, 300000000, 0 },
    { TEGRA20_CLK_PWM, TEGRA20_CLK_PLL_P, 48000000, 0 },
// must be the last entry
    { TEGRA20_CLK_CLK_MAX, TEGRA20_CLK_CLK_MAX, 0, 0 },
    };
//
// Some clocks may be used by different drivers depending on the board
// configuration.  List those here to register them twice in the clock lookup
// table under two names.
//
    static struct tegra_clk_duplicate tegra_clk_duplicates[] = {
    TEGRA_CLK_DUPLICATE(TEGRA20_CLK_USBD,    "utmip-pad",     core::ptr::null_mut()),
    TEGRA_CLK_DUPLICATE(TEGRA20_CLK_USBD,    "tegra-ehci.0",  core::ptr::null_mut()),
    TEGRA_CLK_DUPLICATE(TEGRA20_CLK_USBD,    "tegra-otg",     core::ptr::null_mut()),
    TEGRA_CLK_DUPLICATE(TEGRA20_CLK_CCLK,    core::ptr::null_mut(),           "cpu"),
// must be the last entry
    TEGRA_CLK_DUPLICATE(TEGRA20_CLK_CLK_MAX, core::ptr::null_mut(),            core::ptr::null_mut()),
    };
    static const struct of_device_id pmc_match[] __initconst = {
    { .compatible = "nvidia,tegra20-pmc" },
    { },
    };
    static bool tegra20_car_initialized;
    static struct clk *tegra20_clk_src_onecell_get(struct of_phandle_args *clkspec,
    void *data)
    {
    struct clk_hw *parent_hw;
    struct clk_hw *hw;
    struct clk *clk;
//
// Timer clocks are needed early, the rest of the clocks shouldn't be
// available to device drivers until clock tree is fully initialized.
//
    if (clkspec.args[0] != TEGRA20_CLK_RTC &&
    clkspec.args[0] != TEGRA20_CLK_TWD &&
    clkspec.args[0] != TEGRA20_CLK_TIMER &&
    !tegra20_car_initialized)
    return ERR_PTR(-EPROBE_DEFER);
    clk = of_clk_src_onecell_get(clkspec, data);
    if (IS_ERR(clk))
    return clk;
    hw = __clk_get_hw(clk);
//
// Tegra20 CDEV1, CDEV2 and CSUS clocks are a bit special case, their
// parent clock is created by the pinctrl driver. It is possible for
// clk user to request these clocks before pinctrl driver got probed
// and hence user will get an orphaned clock. That might be undesirable
// because user may expect parent clock to be enabled by the child.
//
    if (clkspec.args[0] == TEGRA20_CLK_CDEV1 ||
    clkspec.args[0] == TEGRA20_CLK_CDEV2 ||
    clkspec.args[0] == TEGRA20_CLK_CSUS) {
    parent_hw = clk_hw_get_parent(hw);
    if (!parent_hw)
    return ERR_PTR(-EPROBE_DEFER);
    }
    if (clkspec.args[0] == TEGRA20_CLK_EMC) {
    if (!tegra20_clk_emc_driver_available(hw))
    return ERR_PTR(-EPROBE_DEFER);
    }
    return clk;
    }
#[no_mangle]
unsafe extern "C" fn tegra20_clock_init(np: *mut device_node) -> void __init {
    static void __init tegra20_clock_init(struct device_node *np)
    {
    struct device_node *node;
    clk_base = of_iomap(np, 0);
    if (!clk_base) {
    pr_err("Can't map CAR registers\n");
    BUG();
    }
    node = of_find_matching_node(core::ptr::null_mut(), pmc_match);
    if (!node) {
    pr_err("Failed to find pmc node\n");
    BUG();
    }
    pmc_base = of_iomap(node, 0);
    of_node_put(node);
    if (!pmc_base) {
    pr_err("Can't map pmc registers\n");
    BUG();
    }
    clks = tegra_clk_init(clk_base, TEGRA20_CLK_CLK_MAX,
    TEGRA20_CLK_PERIPH_BANKS);
    if (!clks)
    return;
    tegra20_osc_clk_init();
    tegra_fixed_clk_init(tegra20_clks);
    tegra20_pll_init();
    tegra20_super_clk_init();
    tegra_super_clk_gen4_init(clk_base, pmc_base, tegra20_clks, core::ptr::null_mut());
    tegra20_periph_clk_init();
    tegra20_audio_clk_init();
    tegra_init_dup_clks(tegra_clk_duplicates, clks, TEGRA20_CLK_CLK_MAX);
    tegra_add_of_provider(np, tegra20_clk_src_onecell_get);
    tegra_cpu_car_ops = &tegra20_cpu_car_ops;
    }
    CLK_OF_DECLARE_DRIVER(tegra20, "nvidia,tegra20-car", tegra20_clock_init);
//
// Clocks that use runtime PM can't be created at the tegra20_clock_init
// time because drivers' base isn't initialized yet, and thus platform
// devices can't be created for the clocks.  Hence we need to split the
// registration of the clocks into two phases.  The first phase registers
// essential clocks which don't require RPM and are actually used during
// early boot.  The second phase registers clocks which use RPM and this
// is done when device drivers' core API is ready.
//
#[no_mangle]
unsafe extern "C" fn tegra20_car_probe(pdev: *mut platform_device) -> c_int {
    static int tegra20_car_probe(struct platform_device *pdev)
    {
    struct clk *clk;
    clk = tegra_clk_register_super_mux("sclk", sclk_parents,
    ARRAY_SIZE(sclk_parents),
    CLK_SET_RATE_PARENT | CLK_IS_CRITICAL,
    clk_base + SCLK_BURST_POLICY, 0, 4, 0, 0, core::ptr::null_mut());
    clks[TEGRA20_CLK_SCLK] = clk;
    tegra_register_devclks(devclks, ARRAY_SIZE(devclks));
    tegra_init_from_table(init_table, clks, TEGRA20_CLK_CLK_MAX);
    tegra20_car_initialized = true;
    return 0;
    }
    static const struct of_device_id tegra20_car_match[] = {
    { .compatible = "nvidia,tegra20-car" },
    { }
    };
    static struct platform_driver tegra20_car_driver = {
    .driver = {
    .name = "tegra20-car",
    .of_match_table = tegra20_car_match,
    .suppress_bind_attrs = true,
    },
    .probe = tegra20_car_probe,
    };
    builtin_platform_driver(tegra20_car_driver);

//! Automatically rewritten from C to Rust
//! Source: drivers/clk/tegra/clk-tegra-periph.c
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
// Copyright (c) 2012, 2013, NVIDIA CORPORATION.  All rights reserved.
//

pub const CLK_SOURCE_I2S0: c_uint = 0x1d8;
pub const CLK_SOURCE_I2S1: c_uint = 0x100;
pub const CLK_SOURCE_I2S2: c_uint = 0x104;
pub const CLK_SOURCE_NDFLASH: c_uint = 0x160;
pub const CLK_SOURCE_I2S3: c_uint = 0x3bc;
pub const CLK_SOURCE_I2S4: c_uint = 0x3c0;
pub const CLK_SOURCE_SPDIF_OUT: c_uint = 0x108;
pub const CLK_SOURCE_SPDIF_IN: c_uint = 0x10c;
pub const CLK_SOURCE_PWM: c_uint = 0x110;
pub const CLK_SOURCE_ADX: c_uint = 0x638;
pub const CLK_SOURCE_ADX1: c_uint = 0x670;
pub const CLK_SOURCE_AMX: c_uint = 0x63c;
pub const CLK_SOURCE_AMX1: c_uint = 0x674;
pub const CLK_SOURCE_HDA: c_uint = 0x428;
pub const CLK_SOURCE_HDA2CODEC_2X: c_uint = 0x3e4;
pub const CLK_SOURCE_SBC1: c_uint = 0x134;
pub const CLK_SOURCE_SBC2: c_uint = 0x118;
pub const CLK_SOURCE_SBC3: c_uint = 0x11c;
pub const CLK_SOURCE_SBC4: c_uint = 0x1b4;
pub const CLK_SOURCE_SBC5: c_uint = 0x3c8;
pub const CLK_SOURCE_SBC6: c_uint = 0x3cc;
pub const CLK_SOURCE_SATA_OOB: c_uint = 0x420;
pub const CLK_SOURCE_SATA: c_uint = 0x424;
pub const CLK_SOURCE_NDSPEED: c_uint = 0x3f8;
pub const CLK_SOURCE_VFIR: c_uint = 0x168;
pub const CLK_SOURCE_SDMMC1: c_uint = 0x150;
pub const CLK_SOURCE_SDMMC2: c_uint = 0x154;
pub const CLK_SOURCE_SDMMC3: c_uint = 0x1bc;
pub const CLK_SOURCE_SDMMC4: c_uint = 0x164;
pub const CLK_SOURCE_CVE: c_uint = 0x140;
pub const CLK_SOURCE_TVO: c_uint = 0x188;
pub const CLK_SOURCE_TVDAC: c_uint = 0x194;
pub const CLK_SOURCE_VDE: c_uint = 0x1c8;
pub const CLK_SOURCE_CSITE: c_uint = 0x1d4;
pub const CLK_SOURCE_LA: c_uint = 0x1f8;
pub const CLK_SOURCE_TRACE: c_uint = 0x634;
pub const CLK_SOURCE_OWR: c_uint = 0x1cc;
pub const CLK_SOURCE_NOR: c_uint = 0x1d0;
pub const CLK_SOURCE_MIPI: c_uint = 0x174;
pub const CLK_SOURCE_I2C1: c_uint = 0x124;
pub const CLK_SOURCE_I2C2: c_uint = 0x198;
pub const CLK_SOURCE_I2C3: c_uint = 0x1b8;
pub const CLK_SOURCE_I2C4: c_uint = 0x3c4;
pub const CLK_SOURCE_I2C5: c_uint = 0x128;
pub const CLK_SOURCE_I2C6: c_uint = 0x65c;
pub const CLK_SOURCE_UARTA: c_uint = 0x178;
pub const CLK_SOURCE_UARTB: c_uint = 0x17c;
pub const CLK_SOURCE_UARTC: c_uint = 0x1a0;
pub const CLK_SOURCE_UARTD: c_uint = 0x1c0;
pub const CLK_SOURCE_UARTE: c_uint = 0x1c4;
pub const CLK_SOURCE_3D: c_uint = 0x158;
pub const CLK_SOURCE_2D: c_uint = 0x15c;
pub const CLK_SOURCE_MPE: c_uint = 0x170;
pub const CLK_SOURCE_VI_SENSOR: c_uint = 0x1a8;
pub const CLK_SOURCE_VI: c_uint = 0x148;
pub const CLK_SOURCE_EPP: c_uint = 0x16c;
pub const CLK_SOURCE_MSENC: c_uint = 0x1f0;
pub const CLK_SOURCE_TSEC: c_uint = 0x1f4;
pub const CLK_SOURCE_HOST1X: c_uint = 0x180;
pub const CLK_SOURCE_HDMI: c_uint = 0x18c;
pub const CLK_SOURCE_DISP1: c_uint = 0x138;
pub const CLK_SOURCE_DISP2: c_uint = 0x13c;
pub const CLK_SOURCE_CILAB: c_uint = 0x614;
pub const CLK_SOURCE_CILCD: c_uint = 0x618;
pub const CLK_SOURCE_CILE: c_uint = 0x61c;
pub const CLK_SOURCE_DSIALP: c_uint = 0x620;
pub const CLK_SOURCE_DSIBLP: c_uint = 0x624;
pub const CLK_SOURCE_TSENSOR: c_uint = 0x3b8;
pub const CLK_SOURCE_D_AUDIO: c_uint = 0x3d0;
pub const CLK_SOURCE_DAM0: c_uint = 0x3d8;
pub const CLK_SOURCE_DAM1: c_uint = 0x3dc;
pub const CLK_SOURCE_DAM2: c_uint = 0x3e0;
pub const CLK_SOURCE_ACTMON: c_uint = 0x3e8;
pub const CLK_SOURCE_EXTERN1: c_uint = 0x3ec;
pub const CLK_SOURCE_EXTERN2: c_uint = 0x3f0;
pub const CLK_SOURCE_EXTERN3: c_uint = 0x3f4;
pub const CLK_SOURCE_I2CSLOW: c_uint = 0x3fc;
pub const CLK_SOURCE_SE: c_uint = 0x42c;
pub const CLK_SOURCE_MSELECT: c_uint = 0x3b4;
pub const CLK_SOURCE_DFLL_REF: c_uint = 0x62c;
pub const CLK_SOURCE_DFLL_SOC: c_uint = 0x630;
pub const CLK_SOURCE_SOC_THERM: c_uint = 0x644;
pub const CLK_SOURCE_XUSB_HOST_SRC: c_uint = 0x600;
pub const CLK_SOURCE_XUSB_FALCON_SRC: c_uint = 0x604;
pub const CLK_SOURCE_XUSB_FS_SRC: c_uint = 0x608;
pub const CLK_SOURCE_XUSB_SS_SRC: c_uint = 0x610;
pub const CLK_SOURCE_XUSB_DEV_SRC: c_uint = 0x60c;
pub const CLK_SOURCE_ISP: c_uint = 0x144;
pub const CLK_SOURCE_SOR0: c_uint = 0x414;
pub const CLK_SOURCE_DPAUX: c_uint = 0x418;
pub const CLK_SOURCE_ENTROPY: c_uint = 0x628;
pub const CLK_SOURCE_VI_SENSOR2: c_uint = 0x658;
pub const CLK_SOURCE_HDMI_AUDIO: c_uint = 0x668;
pub const CLK_SOURCE_VIC03: c_uint = 0x678;
pub const CLK_SOURCE_CLK72MHZ: c_uint = 0x66c;
pub const CLK_SOURCE_DBGAPB: c_uint = 0x718;
pub const CLK_SOURCE_NVENC: c_uint = 0x6a0;
pub const CLK_SOURCE_NVDEC: c_uint = 0x698;
pub const CLK_SOURCE_NVJPG: c_uint = 0x69c;
pub const CLK_SOURCE_APE: c_uint = 0x6c0;
pub const CLK_SOURCE_SDMMC_LEGACY: c_uint = 0x694;
pub const CLK_SOURCE_QSPI: c_uint = 0x6c4;
pub const CLK_SOURCE_VI_I2C: c_uint = 0x6c8;
pub const CLK_SOURCE_MIPIBIF: c_uint = 0x660;
pub const CLK_SOURCE_UARTAPE: c_uint = 0x710;
pub const CLK_SOURCE_TSECB: c_uint = 0x6d8;
pub const CLK_SOURCE_MAUD: c_uint = 0x6d4;
pub const CLK_SOURCE_USB2_HSIC_TRK: c_uint = 0x6cc;
pub const CLK_SOURCE_DMIC1: c_uint = 0x64c;
pub const CLK_SOURCE_DMIC2: c_uint = 0x650;
pub const CLK_SOURCE_DMIC3: c_uint = 0x6bc;

    _clk_num, _gate_flags, _clk_id)	\
    TEGRA_INIT_DATA_TABLE(_name, core::ptr::null_mut(), core::ptr::null_mut(), _parents, _offset,\
    30, MASK(2), 0, 0, 8, 1, TEGRA_DIVIDER_ROUND_UP, \
    _clk_num,  _gate_flags, _clk_id, _parents##_idx, 0,\
    core::ptr::null_mut())

    _clk_num, _gate_flags, _clk_id, flags)\
    TEGRA_INIT_DATA_TABLE(_name, core::ptr::null_mut(), core::ptr::null_mut(), _parents, _offset,\
    30, MASK(2), 0, 0, 8, 1, TEGRA_DIVIDER_ROUND_UP,\
    _clk_num, _gate_flags, _clk_id, _parents##_idx, flags,\
    core::ptr::null_mut())

    _clk_num, _gate_flags, _clk_id)	\
    TEGRA_INIT_DATA_TABLE(_name, core::ptr::null_mut(), core::ptr::null_mut(), _parents, _offset,\
    29, MASK(3), 0, 0, 8, 1, TEGRA_DIVIDER_ROUND_UP,\
    _clk_num, _gate_flags, _clk_id, _parents##_idx, 0,\
    core::ptr::null_mut())

    TEGRA_INIT_DATA_TABLE(_name, core::ptr::null_mut(), core::ptr::null_mut(), _parents, _offset,	\
    29, MASK(3), 0, 0, 8, 1, TEGRA_DIVIDER_ROUND_UP,\
    0, TEGRA_PERIPH_NO_GATE, _clk_id,\
    _parents##_idx, 0, _lock)

    TEGRA_INIT_DATA_TABLE(_name, core::ptr::null_mut(), core::ptr::null_mut(), _parents, _offset,	\
    29, MASK(3), 0, 0, 8, 1, TEGRA_DIVIDER_ROUND_UP,\
    0, TEGRA_PERIPH_NO_GATE, _clk_id,\
    _parents##_idx, 0, core::ptr::null_mut())

    _clk_num, _gate_flags, _clk_id)	\
    TEGRA_INIT_DATA_TABLE(_name, core::ptr::null_mut(), core::ptr::null_mut(), _parents, _offset,\
    30, MASK(2), 0, 0, 8, 1, TEGRA_DIVIDER_INT| \
    TEGRA_DIVIDER_ROUND_UP, _clk_num, _gate_flags,\
    _clk_id, _parents##_idx, 0, core::ptr::null_mut())

    _clk_num, _gate_flags, _clk_id, flags)\
    TEGRA_INIT_DATA_TABLE(_name, core::ptr::null_mut(), core::ptr::null_mut(), _parents, _offset,\
    30, MASK(2), 0, 0, 8, 1, TEGRA_DIVIDER_INT| \
    TEGRA_DIVIDER_ROUND_UP, _clk_num,  _gate_flags,\
    _clk_id, _parents##_idx, flags, core::ptr::null_mut())

    _clk_num, _gate_flags, _clk_id)	\
    TEGRA_INIT_DATA_TABLE(_name, core::ptr::null_mut(), core::ptr::null_mut(), _parents, _offset,\
    29, MASK(3), 0, 0, 8, 1, TEGRA_DIVIDER_INT| \
    TEGRA_DIVIDER_ROUND_UP, _clk_num, _gate_flags,\
    _clk_id, _parents##_idx, 0, core::ptr::null_mut())

    _clk_num, _clk_id)			\
    TEGRA_INIT_DATA_TABLE(_name, core::ptr::null_mut(), core::ptr::null_mut(), _parents, _offset,\
    30, MASK(2), 0, 0, 16, 1, TEGRA_DIVIDER_UART| \
    TEGRA_DIVIDER_ROUND_UP, _clk_num, 0, _clk_id,\
    _parents##_idx, 0, core::ptr::null_mut())

    _clk_num, _clk_id)			\
    TEGRA_INIT_DATA_TABLE(_name, core::ptr::null_mut(), core::ptr::null_mut(), _parents, _offset,\
    29, MASK(3), 0, 0, 16, 1, TEGRA_DIVIDER_UART| \
    TEGRA_DIVIDER_ROUND_UP, _clk_num, 0, _clk_id,\
    _parents##_idx, 0, core::ptr::null_mut())

    _clk_num, _clk_id)			\
    TEGRA_INIT_DATA_TABLE(_name, core::ptr::null_mut(), core::ptr::null_mut(), _parents, _offset,\
    30, MASK(2), 0, 0, 16, 0, TEGRA_DIVIDER_ROUND_UP,\
    _clk_num, TEGRA_PERIPH_ON_APB, _clk_id, \
    _parents##_idx, 0, core::ptr::null_mut())

    _clk_num, _gate_flags, _clk_id)	 \
    TEGRA_INIT_DATA_TABLE(_name, core::ptr::null_mut(), core::ptr::null_mut(), _parents, _offset, \
    29, MASK(3), 0, 0, 8, 1, TEGRA_DIVIDER_INT| \
    TEGRA_DIVIDER_ROUND_UP, _clk_num, _gate_flags,\
    _clk_id, _parents##_idx, 0, core::ptr::null_mut())

    _gate_flags, _clk_id)		\
    TEGRA_INIT_DATA_TABLE(_name, core::ptr::null_mut(), core::ptr::null_mut(), mux_d_audio_clk,	\
    _offset, 16, 0xE01F, 0, 0, 8, 1,		\
    TEGRA_DIVIDER_ROUND_UP, _clk_num, _gate_flags,	\
    _clk_id, mux_d_audio_clk_idx, 0, core::ptr::null_mut())

    _mux_shift, _mux_mask, _clk_num, \
    _gate_flags, _clk_id, _lock)		\
    TEGRA_INIT_DATA_TABLE(_name, core::ptr::null_mut(), core::ptr::null_mut(), _parents, _offset,\
    _mux_shift, _mux_mask, 0, 0, 0, 0, 0,\
    _clk_num, (_gate_flags) | TEGRA_PERIPH_NO_DIV,\
    _clk_id, _parents##_idx, 0, _lock)

    _clk_num, _gate_flags,  _clk_id, _flags)	\
    {								\
    .name = _name,						\
    .clk_id = _clk_id,					\
    .p.parent_name = _parent_name,				\
    .periph = TEGRA_CLK_PERIPH(0, 0, 0, 0, 0, 0, 0,		\
    _clk_num, _gate_flags, core::ptr::null_mut(), core::ptr::null_mut()),	\
    .flags = _flags						\
    }

    {								\
    .name = _name,						\
    .clk_id = _clk_id,					\
    .p.parent_name = _parent_name,				\
    .periph = TEGRA_CLK_PERIPH(0, 0, 0, 0, 8, 1,		\
    TEGRA_DIVIDER_ROUND_UP, 0, 0,		\
    core::ptr::null_mut(), core::ptr::null_mut()),				\
    .offset = _offset,					\
    .flags = _flags,					\
    }
pub const PLLP_BASE: c_uint = 0xa0;
pub const PLLP_MISC: c_uint = 0xac;
pub const PLLP_MISC1: c_uint = 0x680;
pub const PLLP_OUTA: c_uint = 0xa4;
pub const PLLP_OUTB: c_uint = 0xa8;
pub const PLLP_OUTC: c_uint = 0x67c;

pub const PLL_MISC_LOCK_ENABLE: c_int = 18;
    static DEFINE_SPINLOCK(PLLP_OUTA_lock);
    static DEFINE_SPINLOCK(PLLP_OUTB_lock);
    static DEFINE_SPINLOCK(PLLP_OUTC_lock);

    static const char *mux_pllaout0_##_id##_2x_pllp_clkm[] = { "pll_a_out0", \

    "clk_m"};
    MUX_I2S_SPDIF(audio0)
    MUX_I2S_SPDIF(audio1)
    MUX_I2S_SPDIF(audio2)
    MUX_I2S_SPDIF(audio3)
    MUX_I2S_SPDIF(audio4)
    MUX_I2S_SPDIF(audio)

    static const char *mux_pllp_pllc_pllm_clkm[] = {
    "pll_p", "pll_c", "pll_m", "clk_m"
    };

    static const char *mux_pllp_pllc_pllm[] = { "pll_p", "pll_c", "pll_m" };

    static const char *mux_pllp_pllc_clk32_clkm[] = {
    "pll_p", "pll_c", "clk_32k", "clk_m"
    };

    static const char *mux_plla_pllc_pllp_clkm[] = {
    "pll_a_out0", "pll_c", "pll_p", "clk_m"
    };

    static const char *mux_pllp_pllc2_c_c3_pllm_clkm[] = {
    "pll_p", "pll_c2", "pll_c", "pll_c3", "pll_m", "clk_m"
    };
    static u32 mux_pllp_pllc2_c_c3_pllm_clkm_idx[] = {
    [0] = 0, [1] = 1, [2] = 2, [3] = 3, [4] = 4, [5] = 6,
    };
    static const char *mux_pllp_clkm[] = {
    "pll_p", "clk_m"
    };
    static u32 mux_pllp_clkm_idx[] = {
    [0] = 0, [1] = 3,
    };
    static const char *mux_pllp_clkm_2[] = {
    "pll_p", "clk_m"
    };
    static u32 mux_pllp_clkm_2_idx[] = {
    [0] = 2, [1] = 6,
    };
    static const char *mux_pllc2_c_c3_pllp_plla1_clkm[] = {
    "pll_c2", "pll_c", "pll_c3", "pll_p", "pll_a1", "clk_m"
    };
    static u32 mux_pllc2_c_c3_pllp_plla1_clkm_idx[] = {
    [0] = 1, [1] = 2, [2] = 3, [3] = 4, [4] = 6, [5] = 7,
    };
    static const char *
    mux_pllc4_out1_pllc_pllc4_out2_pllp_clkm_plla_pllc4_out0[] = {
    "pll_c4_out1", "pll_c", "pll_c4_out2", "pll_p", "clk_m",
    "pll_a_out0", "pll_c4_out0"
    };
    static u32 mux_pllc4_out1_pllc_pllc4_out2_pllp_clkm_plla_pllc4_out0_idx[] = {
    [0] = 0, [1] = 2, [2] = 3, [3] = 4, [4] = 5, [5] = 6, [6] = 7,
    };
    static const char *mux_pllc_pllp_plla[] = {
    "pll_c", "pll_p", "pll_a_out0"
    };
    static u32 mux_pllc_pllp_plla_idx[] = {
    [0] = 1, [1] = 2, [2] = 3,
    };
    static const char *mux_clkm_pllc_pllp_plla[] = {
    "clk_m", "pll_c", "pll_p", "pll_a_out0"
    };

    static const char *mux_pllc_pllp_plla1_pllc2_c3_clkm[] = {
    "pll_c", "pll_p", "pll_a1", "pll_c2", "pll_c3", "clk_m"
    };
    static u32 mux_pllc_pllp_plla1_pllc2_c3_clkm_idx[] = {
    [0] = 1, [1] = 2, [2] = 3, [3] = 4, [4] = 5, [5] = 6,
    };
    static const char *mux_pllc2_c_c3_pllp_clkm_plla1_pllc4[] = {
    "pll_c2", "pll_c", "pll_c3", "pll_p", "clk_m", "pll_a1", "pll_c4_out0",
    };
    static u32 mux_pllc2_c_c3_pllp_clkm_plla1_pllc4_idx[] = {
    [0] = 1, [1] = 2, [2] = 3, [3] = 4, [4] = 5, [5] = 6, [6] = 7,
    };
    static const char *mux_pllc_pllp_plla1_pllc2_c3_clkm_pllc4[] = {
    "pll_c", "pll_p", "pll_a1", "pll_c2", "pll_c3", "clk_m", "pll_c4_out0",
    };

    mux_pllc2_c_c3_pllp_clkm_plla1_pllc4_idx
    static const char *
    mux_plla_pllc4_out0_pllc_pllc4_out1_pllp_pllc4_out2_clkm[] = {
    "pll_a_out0", "pll_c4_out0", "pll_c", "pll_c4_out1", "pll_p",
    "pll_c4_out2", "clk_m"
    };

    static const char *mux_pllm_pllc2_c_c3_pllp_plla[] = {
    "pll_m", "pll_c2", "pll_c", "pll_c3", "pll_p", "pll_a_out0"
    };

    static const char *mux_pllp_pllm_plld_plla_pllc_plld2_clkm[] = {
    "pll_p", "pll_m", "pll_d_out0", "pll_a_out0", "pll_c",
    "pll_d2_out0", "clk_m"
    };

    static const char *mux_pllm_pllc_pllp_plla[] = {
    "pll_m", "pll_c", "pll_p", "pll_a_out0"
    };

    static const char *mux_pllp_pllc_clkm[] = {
    "pll_p", "pll_c", "clk_m"
    };
    static u32 mux_pllp_pllc_clkm_idx[] = {
    [0] = 0, [1] = 1, [2] = 3,
    };
    static const char *mux_pllp_pllc_clkm_1[] = {
    "pll_p", "pll_c", "clk_m"
    };
    static u32 mux_pllp_pllc_clkm_1_idx[] = {
    [0] = 0, [1] = 2, [2] = 5,
    };
    static const char *mux_pllp_pllc_plla_clkm[] = {
    "pll_p", "pll_c", "pll_a_out0", "clk_m"
    };
    static u32 mux_pllp_pllc_plla_clkm_idx[] = {
    [0] = 0, [1] = 2, [2] = 4, [3] = 6,
    };
    static const char *mux_pllp_pllc_pllc4_out0_pllc4_out1_clkm_pllc4_out2[] = {
    "pll_p", "pll_c", "pll_c4_out0", "pll_c4_out1", "clk_m", "pll_c4_out2"
    };
    static u32 mux_pllp_pllc_pllc4_out0_pllc4_out1_clkm_pllc4_out2_idx[] = {
    [0] = 0, [1] = 2, [2] = 3, [3] = 5, [4] = 6, [5] = 7,
    };
    static const char *
    mux_pllp_pllc_pllc_out1_pllc4_out2_pllc4_out1_clkm_pllc4_out0[] = {
    "pll_p", "pll_c_out1", "pll_c", "pll_c4_out2", "pll_c4_out1",
    "clk_m", "pll_c4_out0"
    };
    static u32
    mux_pllp_pllc_pllc_out1_pllc4_out2_pllc4_out1_clkm_pllc4_out0_idx[] = {
    [0] = 0, [1] = 1, [2] = 2, [3] = 4, [4] = 5, [5] = 6, [6] = 7,
    };
    static const char *mux_pllp_pllc4_out2_pllc4_out1_clkm_pllc4_out0[] = {
    "pll_p", "pll_c4_out2", "pll_c4_out1", "clk_m", "pll_c4_out0"
    };
    static u32 mux_pllp_pllc4_out2_pllc4_out1_clkm_pllc4_out0_idx[] = {
    [0] = 0, [1] = 3, [2] = 4, [3] = 6, [4] = 7,
    };
    static const char *mux_pllp_pllc2_c_c3_clkm[] = {
    "pll_p", "pll_c2", "pll_c", "pll_c3", "clk_m"
    };
    static u32 mux_pllp_pllc2_c_c3_clkm_idx[] = {
    [0] = 0, [1] = 1, [2] = 2, [3] = 3, [4] = 6,
    };
    static const char *mux_pllp_clkm_clk32_plle[] = {
    "pll_p", "clk_m", "clk_32k", "pll_e"
    };
    static u32 mux_pllp_clkm_clk32_plle_idx[] = {
    [0] = 0, [1] = 2, [2] = 4, [3] = 6,
    };
    static const char *mux_pllp_pllp_out3_clkm_clk32k_plla[] = {
    "pll_p", "pll_p_out3", "clk_m", "clk_32k", "pll_a_out0"
    };

    static const char *mux_pllp_out3_clkm_pllp_pllc4[] = {
    "pll_p_out3", "clk_m", "pll_p", "pll_c4_out0", "pll_c4_out1",
    "pll_c4_out2"
    };
    static u32 mux_pllp_out3_clkm_pllp_pllc4_idx[] = {
    [0] = 0, [1] = 3, [2] = 4, [3] = 5, [4] = 6, [5] = 7,
    };
    static const char *mux_clkm_pllp_pllre[] = {
    "clk_m", "pll_p_out_xusb", "pll_re_out"
    };
    static u32 mux_clkm_pllp_pllre_idx[] = {
    [0] = 0, [1] = 1, [2] = 5,
    };
    static const char *mux_pllp_pllc_clkm_clk32[] = {
    "pll_p", "pll_c", "clk_m", "clk_32k"
    };

    static const char *mux_plla_clk32_pllp_clkm_plle[] = {
    "pll_a_out0", "clk_32k", "pll_p", "clk_m", "pll_e_out0"
    };

    static const char *mux_clkm_pllp_pllc_pllre[] = {
    "clk_m", "pll_p", "pll_c", "pll_re_out"
    };
    static u32 mux_clkm_pllp_pllc_pllre_idx[] = {
    [0] = 0, [1] = 1, [2] = 3, [3] = 5,
    };
    static const char *mux_clkm_48M_pllp_480M[] = {
    "clk_m", "pll_u_48M", "pll_p", "pll_u_480M"
    };
    static u32 mux_clkm_48M_pllp_480M_idx[] = {
    [0] = 0, [1] = 2, [2] = 4, [3] = 6,
    };
    static const char *mux_clkm_pllre_clk32_480M[] = {
    "clk_m", "pll_re_out", "clk_32k", "pll_u_480M"
    };

    static const char *mux_clkm_pllre_clk32_480M_pllc_ref[] = {
    "clk_m", "pll_re_out", "clk_32k", "pll_u_480M", "pll_c", "pll_ref"
    };
    static u32 mux_clkm_pllre_clk32_480M_pllc_ref_idx[] = {
    [0] = 0, [1] = 1, [2] = 3, [3] = 3, [4] = 4, [5] = 7,
    };
    static const char *mux_pllp_out3_pllp_pllc_clkm[] = {
    "pll_p_out3", "pll_p", "pll_c", "clk_m"
    };
    static u32 mux_pllp_out3_pllp_pllc_clkm_idx[] = {
    [0] = 0, [1] = 1, [2] = 2, [3] = 6,
    };
    static const char *mux_ss_div2_60M[] = {
    "xusb_ss_div2", "pll_u_60M"
    };

    static const char *mux_ss_div2_60M_ss[] = {
    "xusb_ss_div2", "pll_u_60M", "xusb_ss_src"
    };

    static const char *mux_ss_clkm[] = {
    "xusb_ss_src", "clk_m"
    };

    static const char *mux_d_audio_clk[] = {
    "pll_a_out0", "pll_p", "clk_m", "spdif_in_sync", "i2s0_sync",
    "i2s1_sync", "i2s2_sync", "i2s3_sync", "i2s4_sync", "vimclk_sync",
    };
    static u32 mux_d_audio_clk_idx[] = {
    [0] = 0, [1] = 0x8000, [2] = 0xc000, [3] = 0xE000, [4] = 0xE001,
    [5] = 0xE002, [6] = 0xE003, [7] = 0xE004, [8] = 0xE005, [9] = 0xE007,
    };
    static const char *mux_pllp_plld_pllc_clkm[] = {
    "pll_p", "pll_d_out0", "pll_c", "clk_m"
    };

    static const char *mux_pllm_pllc_pllp_plla_clkm_pllc4[] = {
    "pll_m", "pll_c", "pll_p", "pll_a_out0", "clk_m", "pll_c4",
    };
    static u32 mux_pllm_pllc_pllp_plla_clkm_pllc4_idx[] = {
    [0] = 0, [1] = 1, [2] = 3, [3] = 3, [4] = 6, [5] = 7,
    };
    static const char *mux_pllp_clkm1[] = {
    "pll_p", "clk_m",
    };

    static const char *mux_pllp3_pllc_clkm[] = {
    "pll_p_out3", "pll_c", "pll_c2", "clk_m",
    };

    static const char *mux_pllm_pllc_pllp_plla_pllc2_c3_clkm[] = {
    "pll_m", "pll_c", "pll_p", "pll_a", "pll_c2", "pll_c3", "clk_m"
    };

    static const char *mux_pllm_pllc2_c_c3_pllp_plla_pllc4[] = {
    "pll_m", "pll_c2", "pll_c", "pll_c3", "pll_p", "pll_a_out0", "pll_c4",
    };
    static u32 mux_pllm_pllc2_c_c3_pllp_plla_pllc4_idx[] = {
    [0] = 0, [1] = 1, [2] = 2, [3] = 3, [4] = 4, [5] = 6, [6] = 7,
    };
// SOR1 mux'es
    static const char *mux_pllp_plld_plld2_clkm[] = {
    "pll_p", "pll_d_out0", "pll_d2_out0", "clk_m"
    };
    static u32 mux_pllp_plld_plld2_clkm_idx[] = {
    [0] = 0, [1] = 2, [2] = 5, [3] = 6
    };
    static const char *mux_pllp_pllre_clkm[] = {
    "pll_p", "pll_re_out1", "clk_m"
    };
    static u32 mux_pllp_pllre_clkm_idx[] = {
    [0] = 0, [1] = 2, [2] = 3,
    };
    static const char * const mux_dmic1[] = {
    "pll_a_out0", "dmic1_sync_clk", "pll_p", "clk_m"
    };

    static const char * const mux_dmic2[] = {
    "pll_a_out0", "dmic2_sync_clk", "pll_p", "clk_m"
    };

    static const char * const mux_dmic3[] = {
    "pll_a_out0", "dmic3_sync_clk", "pll_p", "clk_m"
    };

    static struct tegra_periph_init_data periph_clks[] = {
    AUDIO("d_audio", CLK_SOURCE_D_AUDIO, 106, TEGRA_PERIPH_ON_APB, tegra_clk_d_audio),
    AUDIO("dam0", CLK_SOURCE_DAM0, 108, TEGRA_PERIPH_ON_APB, tegra_clk_dam0),
    AUDIO("dam1", CLK_SOURCE_DAM1, 109, TEGRA_PERIPH_ON_APB, tegra_clk_dam1),
    AUDIO("dam2", CLK_SOURCE_DAM2, 110, TEGRA_PERIPH_ON_APB, tegra_clk_dam2),
    I2C("i2c1", mux_pllp_clkm, CLK_SOURCE_I2C1, 12, tegra_clk_i2c1),
    I2C("i2c2", mux_pllp_clkm, CLK_SOURCE_I2C2, 54, tegra_clk_i2c2),
    I2C("i2c3", mux_pllp_clkm, CLK_SOURCE_I2C3, 67, tegra_clk_i2c3),
    I2C("i2c4", mux_pllp_clkm, CLK_SOURCE_I2C4, 103, tegra_clk_i2c4),
    I2C("i2c5", mux_pllp_clkm, CLK_SOURCE_I2C5, 47, tegra_clk_i2c5),
    I2C("i2c6", mux_pllp_clkm, CLK_SOURCE_I2C6, 166, tegra_clk_i2c6),
    INT("vde", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_VDE, 61, 0, tegra_clk_vde),
    INT("vi", mux_pllm_pllc_pllp_plla, CLK_SOURCE_VI, 20, 0, tegra_clk_vi),
    INT("epp", mux_pllm_pllc_pllp_plla, CLK_SOURCE_EPP, 19, 0, tegra_clk_epp),
    INT("host1x", mux_pllm_pllc_pllp_plla, CLK_SOURCE_HOST1X, 28, 0, tegra_clk_host1x),
    INT("mpe", mux_pllm_pllc_pllp_plla, CLK_SOURCE_MPE, 60, 0, tegra_clk_mpe),
    INT("2d", mux_pllm_pllc_pllp_plla, CLK_SOURCE_2D, 21, 0, tegra_clk_gr2d),
    INT("3d", mux_pllm_pllc_pllp_plla, CLK_SOURCE_3D, 24, 0, tegra_clk_gr3d),
    INT8("vde", mux_pllp_pllc2_c_c3_pllm_clkm, CLK_SOURCE_VDE, 61, 0, tegra_clk_vde_8),
    INT8("vi", mux_pllm_pllc2_c_c3_pllp_plla, CLK_SOURCE_VI, 20, 0, tegra_clk_vi_8),
    INT8("vi", mux_pllm_pllc2_c_c3_pllp_plla_pllc4, CLK_SOURCE_VI, 20, 0, tegra_clk_vi_9),
    INT8("vi", mux_pllc2_c_c3_pllp_clkm_plla1_pllc4, CLK_SOURCE_VI, 20, 0, tegra_clk_vi_10),
    INT8("epp", mux_pllm_pllc2_c_c3_pllp_plla, CLK_SOURCE_EPP, 19, 0, tegra_clk_epp_8),
    INT8("msenc", mux_pllm_pllc2_c_c3_pllp_plla, CLK_SOURCE_MSENC, 91, TEGRA_PERIPH_WAR_1005168, tegra_clk_msenc),
    INT8("tsec", mux_pllp_pllc2_c_c3_pllm_clkm, CLK_SOURCE_TSEC, 83, 0, tegra_clk_tsec),
    INT("tsec", mux_pllp_pllc_clkm, CLK_SOURCE_TSEC, 83, 0, tegra_clk_tsec_8),
    INT8("host1x", mux_pllm_pllc2_c_c3_pllp_plla, CLK_SOURCE_HOST1X, 28, 0, tegra_clk_host1x_8),
    INT8("host1x", mux_pllc4_out1_pllc_pllc4_out2_pllp_clkm_plla_pllc4_out0, CLK_SOURCE_HOST1X, 28, 0, tegra_clk_host1x_9),
    INT8("se", mux_pllp_pllc2_c_c3_pllm_clkm, CLK_SOURCE_SE, 127, TEGRA_PERIPH_ON_APB, tegra_clk_se),
    INT8("se", mux_pllp_pllc2_c_c3_clkm, CLK_SOURCE_SE, 127, TEGRA_PERIPH_ON_APB, tegra_clk_se_10),
    INT8("2d", mux_pllm_pllc2_c_c3_pllp_plla, CLK_SOURCE_2D, 21, 0, tegra_clk_gr2d_8),
    INT8("3d", mux_pllm_pllc2_c_c3_pllp_plla, CLK_SOURCE_3D, 24, 0, tegra_clk_gr3d_8),
    INT8("vic03", mux_pllm_pllc_pllp_plla_pllc2_c3_clkm, CLK_SOURCE_VIC03, 178, 0, tegra_clk_vic03),
    INT8("vic03", mux_pllc_pllp_plla1_pllc2_c3_clkm, CLK_SOURCE_VIC03, 178, 0, tegra_clk_vic03_8),
    INT_FLAGS("mselect", mux_pllp_clkm, CLK_SOURCE_MSELECT, 99, 0, tegra_clk_mselect, CLK_IGNORE_UNUSED),
    MUX("i2s0", mux_pllaout0_audio0_2x_pllp_clkm, CLK_SOURCE_I2S0, 30, TEGRA_PERIPH_ON_APB, tegra_clk_i2s0),
    MUX("i2s1", mux_pllaout0_audio1_2x_pllp_clkm, CLK_SOURCE_I2S1, 11, TEGRA_PERIPH_ON_APB, tegra_clk_i2s1),
    MUX("i2s2", mux_pllaout0_audio2_2x_pllp_clkm, CLK_SOURCE_I2S2, 18, TEGRA_PERIPH_ON_APB, tegra_clk_i2s2),
    MUX("i2s3", mux_pllaout0_audio3_2x_pllp_clkm, CLK_SOURCE_I2S3, 101, TEGRA_PERIPH_ON_APB, tegra_clk_i2s3),
    MUX("i2s4", mux_pllaout0_audio4_2x_pllp_clkm, CLK_SOURCE_I2S4, 102, TEGRA_PERIPH_ON_APB, tegra_clk_i2s4),
    MUX("spdif_out", mux_pllaout0_audio_2x_pllp_clkm, CLK_SOURCE_SPDIF_OUT, 10, TEGRA_PERIPH_ON_APB, tegra_clk_spdif_out),
    MUX("spdif_in", mux_pllp_pllc_pllm, CLK_SOURCE_SPDIF_IN, 10, TEGRA_PERIPH_ON_APB, tegra_clk_spdif_in),
    MUX8("spdif_in", mux_pllp_pllc_clkm_1, CLK_SOURCE_SPDIF_IN, 10, TEGRA_PERIPH_ON_APB, tegra_clk_spdif_in_8),
    MUX("pwm", mux_pllp_pllc_clk32_clkm, CLK_SOURCE_PWM, 17, TEGRA_PERIPH_ON_APB, tegra_clk_pwm),
    MUX("adx", mux_plla_pllc_pllp_clkm, CLK_SOURCE_ADX, 154, TEGRA_PERIPH_ON_APB, tegra_clk_adx),
    MUX("amx", mux_plla_pllc_pllp_clkm, CLK_SOURCE_AMX, 153, TEGRA_PERIPH_ON_APB, tegra_clk_amx),
    MUX("hda", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_HDA, 125, TEGRA_PERIPH_ON_APB, tegra_clk_hda),
    MUX("hda", mux_pllp_pllc_clkm, CLK_SOURCE_HDA, 125, TEGRA_PERIPH_ON_APB, tegra_clk_hda_8),
    MUX("hda2codec_2x", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_HDA2CODEC_2X, 111, TEGRA_PERIPH_ON_APB, tegra_clk_hda2codec_2x),
    MUX8("hda2codec_2x", mux_pllp_pllc_plla_clkm, CLK_SOURCE_HDA2CODEC_2X, 111, TEGRA_PERIPH_ON_APB, tegra_clk_hda2codec_2x_8),
    MUX("vfir", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_VFIR, 7, TEGRA_PERIPH_ON_APB, tegra_clk_vfir),
    MUX("sdmmc1", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_SDMMC1, 14, TEGRA_PERIPH_ON_APB, tegra_clk_sdmmc1),
    MUX("sdmmc2", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_SDMMC2, 9, TEGRA_PERIPH_ON_APB, tegra_clk_sdmmc2),
    MUX("sdmmc3", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_SDMMC3, 69, TEGRA_PERIPH_ON_APB, tegra_clk_sdmmc3),
    MUX("sdmmc4", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_SDMMC4, 15, TEGRA_PERIPH_ON_APB, tegra_clk_sdmmc4),
    MUX8("sdmmc1", mux_pllp_pllc4_out2_pllc4_out1_clkm_pllc4_out0, CLK_SOURCE_SDMMC1, 14, TEGRA_PERIPH_ON_APB, tegra_clk_sdmmc1_9),
    MUX8("sdmmc3", mux_pllp_pllc4_out2_pllc4_out1_clkm_pllc4_out0, CLK_SOURCE_SDMMC3, 69, TEGRA_PERIPH_ON_APB, tegra_clk_sdmmc3_9),
    MUX("la", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_LA, 76, TEGRA_PERIPH_ON_APB, tegra_clk_la),
    MUX("trace", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_TRACE, 77, TEGRA_PERIPH_ON_APB, tegra_clk_trace),
    MUX("owr", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_OWR, 71, TEGRA_PERIPH_ON_APB, tegra_clk_owr),
    MUX("owr", mux_pllp_pllc_clkm, CLK_SOURCE_OWR, 71, TEGRA_PERIPH_ON_APB, tegra_clk_owr_8),
    MUX("nor", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_NOR, 42, 0, tegra_clk_nor),
    MUX("mipi", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_MIPI, 50, TEGRA_PERIPH_ON_APB, tegra_clk_mipi),
    MUX("vi_sensor", mux_pllm_pllc_pllp_plla, CLK_SOURCE_VI_SENSOR, 20, TEGRA_PERIPH_NO_RESET, tegra_clk_vi_sensor),
    MUX("vi_sensor", mux_pllc_pllp_plla, CLK_SOURCE_VI_SENSOR, 20, TEGRA_PERIPH_NO_RESET, tegra_clk_vi_sensor_9),
    MUX("cilab", mux_pllp_pllc_clkm, CLK_SOURCE_CILAB, 144, 0, tegra_clk_cilab),
    MUX("cilcd", mux_pllp_pllc_clkm, CLK_SOURCE_CILCD, 145, 0, tegra_clk_cilcd),
    MUX("cile", mux_pllp_pllc_clkm, CLK_SOURCE_CILE, 146, 0, tegra_clk_cile),
    MUX("dsialp", mux_pllp_pllc_clkm, CLK_SOURCE_DSIALP, 147, 0, tegra_clk_dsialp),
    MUX("dsiblp", mux_pllp_pllc_clkm, CLK_SOURCE_DSIBLP, 148, 0, tegra_clk_dsiblp),
    MUX("tsensor", mux_pllp_pllc_clkm_clk32, CLK_SOURCE_TSENSOR, 100, TEGRA_PERIPH_ON_APB, tegra_clk_tsensor),
    MUX("actmon", mux_pllp_pllc_clk32_clkm, CLK_SOURCE_ACTMON, 119, 0, tegra_clk_actmon),
    MUX("dfll_ref", mux_pllp_clkm, CLK_SOURCE_DFLL_REF, 155, TEGRA_PERIPH_ON_APB, tegra_clk_dfll_ref),
    MUX("dfll_soc", mux_pllp_clkm, CLK_SOURCE_DFLL_SOC, 155, TEGRA_PERIPH_ON_APB, tegra_clk_dfll_soc),
    MUX("i2cslow", mux_pllp_pllc_clk32_clkm, CLK_SOURCE_I2CSLOW, 81, TEGRA_PERIPH_ON_APB, tegra_clk_i2cslow),
    MUX("sbc1", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_SBC1, 41, TEGRA_PERIPH_ON_APB, tegra_clk_sbc1),
    MUX("sbc2", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_SBC2, 44, TEGRA_PERIPH_ON_APB, tegra_clk_sbc2),
    MUX("sbc3", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_SBC3, 46, TEGRA_PERIPH_ON_APB, tegra_clk_sbc3),
    MUX("sbc4", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_SBC4, 68, TEGRA_PERIPH_ON_APB, tegra_clk_sbc4),
    MUX("sbc5", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_SBC5, 104, TEGRA_PERIPH_ON_APB, tegra_clk_sbc5),
    MUX("sbc6", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_SBC6, 105, TEGRA_PERIPH_ON_APB, tegra_clk_sbc6),
    MUX("cve", mux_pllp_plld_pllc_clkm, CLK_SOURCE_CVE, 49, 0, tegra_clk_cve),
    MUX("tvo", mux_pllp_plld_pllc_clkm, CLK_SOURCE_TVO, 49, 0, tegra_clk_tvo),
    MUX("tvdac", mux_pllp_plld_pllc_clkm, CLK_SOURCE_TVDAC, 53, 0, tegra_clk_tvdac),
    MUX("ndflash", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_NDFLASH, 13, TEGRA_PERIPH_ON_APB, tegra_clk_ndflash),
    MUX("ndspeed", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_NDSPEED, 80, TEGRA_PERIPH_ON_APB, tegra_clk_ndspeed),
    MUX("sata_oob", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_SATA_OOB, 123, TEGRA_PERIPH_ON_APB, tegra_clk_sata_oob),
    MUX("sata_oob", mux_pllp_pllc_clkm, CLK_SOURCE_SATA_OOB, 123, TEGRA_PERIPH_ON_APB, tegra_clk_sata_oob_8),
    MUX("sata", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_SATA, 124, TEGRA_PERIPH_ON_APB, tegra_clk_sata),
    MUX("sata", mux_pllp_pllc_clkm, CLK_SOURCE_SATA, 124, TEGRA_PERIPH_ON_APB, tegra_clk_sata_8),
    MUX("adx1", mux_plla_pllc_pllp_clkm, CLK_SOURCE_ADX1, 180, TEGRA_PERIPH_ON_APB, tegra_clk_adx1),
    MUX("amx1", mux_plla_pllc_pllp_clkm, CLK_SOURCE_AMX1, 185, TEGRA_PERIPH_ON_APB, tegra_clk_amx1),
    MUX("vi_sensor2", mux_pllm_pllc2_c_c3_pllp_plla, CLK_SOURCE_VI_SENSOR2, 165, TEGRA_PERIPH_NO_RESET, tegra_clk_vi_sensor2),
    MUX("vi_sensor2", mux_pllc_pllp_plla, CLK_SOURCE_VI_SENSOR2, 165, TEGRA_PERIPH_NO_RESET, tegra_clk_vi_sensor2_8),
    MUX8("sdmmc1", mux_pllp_pllc2_c_c3_pllm_clkm, CLK_SOURCE_SDMMC1, 14, TEGRA_PERIPH_ON_APB, tegra_clk_sdmmc1_8),
    MUX8("sdmmc2", mux_pllp_pllc2_c_c3_pllm_clkm, CLK_SOURCE_SDMMC2, 9, TEGRA_PERIPH_ON_APB, tegra_clk_sdmmc2_8),
    MUX8("sdmmc3", mux_pllp_pllc2_c_c3_pllm_clkm, CLK_SOURCE_SDMMC3, 69, TEGRA_PERIPH_ON_APB, tegra_clk_sdmmc3_8),
    MUX8("sdmmc4", mux_pllp_pllc2_c_c3_pllm_clkm, CLK_SOURCE_SDMMC4, 15, TEGRA_PERIPH_ON_APB, tegra_clk_sdmmc4_8),
    MUX8("sbc1", mux_pllp_pllc2_c_c3_pllm_clkm, CLK_SOURCE_SBC1, 41, TEGRA_PERIPH_ON_APB, tegra_clk_sbc1_8),
    MUX8("sbc2", mux_pllp_pllc2_c_c3_pllm_clkm, CLK_SOURCE_SBC2, 44, TEGRA_PERIPH_ON_APB, tegra_clk_sbc2_8),
    MUX8("sbc3", mux_pllp_pllc2_c_c3_pllm_clkm, CLK_SOURCE_SBC3, 46, TEGRA_PERIPH_ON_APB, tegra_clk_sbc3_8),
    MUX8("sbc4", mux_pllp_pllc2_c_c3_pllm_clkm, CLK_SOURCE_SBC4, 68, TEGRA_PERIPH_ON_APB, tegra_clk_sbc4_8),
    MUX8("sbc5", mux_pllp_pllc2_c_c3_pllm_clkm, CLK_SOURCE_SBC5, 104, TEGRA_PERIPH_ON_APB, tegra_clk_sbc5_8),
    MUX8("sbc6", mux_pllp_pllc2_c_c3_pllm_clkm, CLK_SOURCE_SBC6, 105, TEGRA_PERIPH_ON_APB, tegra_clk_sbc6_8),
    MUX("sbc1", mux_pllp_pllc_clkm, CLK_SOURCE_SBC1, 41, TEGRA_PERIPH_ON_APB, tegra_clk_sbc1_9),
    MUX("sbc2", mux_pllp_pllc_clkm, CLK_SOURCE_SBC2, 44, TEGRA_PERIPH_ON_APB, tegra_clk_sbc2_9),
    MUX("sbc3", mux_pllp_pllc_clkm, CLK_SOURCE_SBC3, 46, TEGRA_PERIPH_ON_APB, tegra_clk_sbc3_9),
    MUX("sbc4", mux_pllp_pllc_clkm, CLK_SOURCE_SBC4, 68, TEGRA_PERIPH_ON_APB, tegra_clk_sbc4_9),
    MUX8("ndflash", mux_pllp_pllc2_c_c3_pllm_clkm, CLK_SOURCE_NDFLASH, 13, TEGRA_PERIPH_ON_APB, tegra_clk_ndflash_8),
    MUX8("ndspeed", mux_pllp_pllc2_c_c3_pllm_clkm, CLK_SOURCE_NDSPEED, 80, TEGRA_PERIPH_ON_APB, tegra_clk_ndspeed_8),
    MUX8("hdmi", mux_pllp_pllm_plld_plla_pllc_plld2_clkm, CLK_SOURCE_HDMI, 51, 0, tegra_clk_hdmi),
    MUX8("extern1", mux_plla_clk32_pllp_clkm_plle, CLK_SOURCE_EXTERN1, 120, TEGRA_PERIPH_NO_RESET, tegra_clk_extern1),
    MUX8("extern2", mux_plla_clk32_pllp_clkm_plle, CLK_SOURCE_EXTERN2, 121, TEGRA_PERIPH_NO_RESET, tegra_clk_extern2),
    MUX8("extern3", mux_plla_clk32_pllp_clkm_plle, CLK_SOURCE_EXTERN3, 122, TEGRA_PERIPH_NO_RESET, tegra_clk_extern3),
    MUX8("soc_therm", mux_pllm_pllc_pllp_plla, CLK_SOURCE_SOC_THERM, 78, TEGRA_PERIPH_ON_APB, tegra_clk_soc_therm),
    MUX8("soc_therm", mux_clkm_pllc_pllp_plla, CLK_SOURCE_SOC_THERM, 78, TEGRA_PERIPH_ON_APB, tegra_clk_soc_therm_8),
    MUX8("vi_sensor", mux_pllm_pllc2_c_c3_pllp_plla, CLK_SOURCE_VI_SENSOR, 164, TEGRA_PERIPH_NO_RESET, tegra_clk_vi_sensor_8),
    MUX8("isp", mux_pllm_pllc_pllp_plla_clkm_pllc4, CLK_SOURCE_ISP, 23, TEGRA_PERIPH_ON_APB, tegra_clk_isp_8),
    MUX8_NOGATE("isp", mux_pllc_pllp_plla1_pllc2_c3_clkm_pllc4, CLK_SOURCE_ISP, tegra_clk_isp_9),
    MUX8("entropy", mux_pllp_clkm1, CLK_SOURCE_ENTROPY, 149,  0, tegra_clk_entropy),
    MUX8("entropy", mux_pllp_clkm_clk32_plle, CLK_SOURCE_ENTROPY, 149,  0, tegra_clk_entropy_8),
    MUX8("hdmi_audio", mux_pllp3_pllc_clkm, CLK_SOURCE_HDMI_AUDIO, 176, TEGRA_PERIPH_NO_RESET, tegra_clk_hdmi_audio),
    MUX8("clk72mhz", mux_pllp3_pllc_clkm, CLK_SOURCE_CLK72MHZ, 177, TEGRA_PERIPH_NO_RESET, tegra_clk_clk72Mhz),
    MUX8("clk72mhz", mux_pllp_out3_pllp_pllc_clkm, CLK_SOURCE_CLK72MHZ, 177, TEGRA_PERIPH_NO_RESET, tegra_clk_clk72Mhz_8),
    MUX_FLAGS("csite", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_CSITE, 73, TEGRA_PERIPH_ON_APB, tegra_clk_csite, CLK_IGNORE_UNUSED),
    MUX_FLAGS("csite", mux_pllp_pllre_clkm, CLK_SOURCE_CSITE, 73, TEGRA_PERIPH_ON_APB, tegra_clk_csite_8, CLK_IGNORE_UNUSED),
    NODIV("disp1", mux_pllp_pllm_plld_plla_pllc_plld2_clkm, CLK_SOURCE_DISP1, 29, 7, 27, 0, tegra_clk_disp1, core::ptr::null_mut()),
    NODIV("disp1", mux_pllp_plld_plld2_clkm, CLK_SOURCE_DISP1, 29, 7, 27, 0, tegra_clk_disp1_8, core::ptr::null_mut()),
    NODIV("disp2", mux_pllp_pllm_plld_plla_pllc_plld2_clkm, CLK_SOURCE_DISP2, 29, 7, 26, 0, tegra_clk_disp2, core::ptr::null_mut()),
    NODIV("disp2", mux_pllp_plld_plld2_clkm, CLK_SOURCE_DISP2, 29, 7, 26, 0, tegra_clk_disp2_8, core::ptr::null_mut()),
    UART("uarta", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_UARTA, 6, tegra_clk_uarta),
    UART("uartb", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_UARTB, 7, tegra_clk_uartb),
    UART("uartc", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_UARTC, 55, tegra_clk_uartc),
    UART("uartd", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_UARTD, 65, tegra_clk_uartd),
    UART("uarte", mux_pllp_pllc_pllm_clkm, CLK_SOURCE_UARTE, 66, tegra_clk_uarte),
    UART8("uarta", mux_pllp_pllc_pllc4_out0_pllc4_out1_clkm_pllc4_out2, CLK_SOURCE_UARTA, 6, tegra_clk_uarta_8),
    UART8("uartb", mux_pllp_pllc_pllc4_out0_pllc4_out1_clkm_pllc4_out2, CLK_SOURCE_UARTB, 7, tegra_clk_uartb_8),
    UART8("uartc", mux_pllp_pllc_pllc4_out0_pllc4_out1_clkm_pllc4_out2, CLK_SOURCE_UARTC, 55, tegra_clk_uartc_8),
    UART8("uartd", mux_pllp_pllc_pllc4_out0_pllc4_out1_clkm_pllc4_out2, CLK_SOURCE_UARTD, 65, tegra_clk_uartd_8),
    XUSB("xusb_host_src", mux_clkm_pllp_pllc_pllre, CLK_SOURCE_XUSB_HOST_SRC, 143, TEGRA_PERIPH_ON_APB | TEGRA_PERIPH_NO_RESET, tegra_clk_xusb_host_src),
    XUSB("xusb_host_src", mux_clkm_pllp_pllre, CLK_SOURCE_XUSB_HOST_SRC, 143, TEGRA_PERIPH_ON_APB | TEGRA_PERIPH_NO_RESET, tegra_clk_xusb_host_src_8),
    XUSB("xusb_falcon_src", mux_clkm_pllp_pllc_pllre, CLK_SOURCE_XUSB_FALCON_SRC, 143, TEGRA_PERIPH_NO_RESET, tegra_clk_xusb_falcon_src),
    XUSB("xusb_falcon_src", mux_clkm_pllp_pllre, CLK_SOURCE_XUSB_FALCON_SRC, 143, TEGRA_PERIPH_NO_RESET, tegra_clk_xusb_falcon_src_8),
    XUSB("xusb_fs_src", mux_clkm_48M_pllp_480M, CLK_SOURCE_XUSB_FS_SRC, 143, TEGRA_PERIPH_NO_RESET, tegra_clk_xusb_fs_src),
    XUSB("xusb_ss_src", mux_clkm_pllre_clk32_480M_pllc_ref, CLK_SOURCE_XUSB_SS_SRC, 143, TEGRA_PERIPH_NO_RESET, tegra_clk_xusb_ss_src),
    XUSB("xusb_ss_src", mux_clkm_pllre_clk32_480M, CLK_SOURCE_XUSB_SS_SRC, 143, TEGRA_PERIPH_NO_RESET, tegra_clk_xusb_ss_src_8),
    NODIV("xusb_hs_src", mux_ss_div2_60M, CLK_SOURCE_XUSB_SS_SRC, 25, MASK(1), 143, TEGRA_PERIPH_NO_RESET, tegra_clk_xusb_hs_src, core::ptr::null_mut()),
    NODIV("xusb_hs_src", mux_ss_div2_60M_ss, CLK_SOURCE_XUSB_SS_SRC, 25, MASK(2), 143, TEGRA_PERIPH_NO_RESET, tegra_clk_xusb_hs_src_4, core::ptr::null_mut()),
    NODIV("xusb_ssp_src", mux_ss_clkm, CLK_SOURCE_XUSB_SS_SRC, 24, MASK(1), 143, TEGRA_PERIPH_NO_RESET, tegra_clk_xusb_ssp_src, core::ptr::null_mut()),
    XUSB("xusb_dev_src", mux_clkm_pllp_pllc_pllre, CLK_SOURCE_XUSB_DEV_SRC, 95, TEGRA_PERIPH_ON_APB | TEGRA_PERIPH_NO_RESET, tegra_clk_xusb_dev_src),
    XUSB("xusb_dev_src", mux_clkm_pllp_pllre, CLK_SOURCE_XUSB_DEV_SRC, 95, TEGRA_PERIPH_ON_APB | TEGRA_PERIPH_NO_RESET, tegra_clk_xusb_dev_src_8),
    MUX8("dbgapb", mux_pllp_clkm_2, CLK_SOURCE_DBGAPB, 185, TEGRA_PERIPH_NO_RESET, tegra_clk_dbgapb),
    MUX8("nvenc", mux_pllc2_c_c3_pllp_plla1_clkm, CLK_SOURCE_NVENC, 219, 0, tegra_clk_nvenc),
    MUX8("nvdec", mux_pllc2_c_c3_pllp_plla1_clkm, CLK_SOURCE_NVDEC, 194, 0, tegra_clk_nvdec),
    MUX8("nvjpg", mux_pllc2_c_c3_pllp_plla1_clkm, CLK_SOURCE_NVJPG, 195, 0, tegra_clk_nvjpg),
    MUX8("ape", mux_plla_pllc4_out0_pllc_pllc4_out1_pllp_pllc4_out2_clkm, CLK_SOURCE_APE, 198, TEGRA_PERIPH_ON_APB, tegra_clk_ape),
    MUX8("sdmmc_legacy", mux_pllp_out3_clkm_pllp_pllc4, CLK_SOURCE_SDMMC_LEGACY, 193, TEGRA_PERIPH_ON_APB | TEGRA_PERIPH_NO_RESET, tegra_clk_sdmmc_legacy),
    MUX8("qspi", mux_pllp_pllc_pllc_out1_pllc4_out2_pllc4_out1_clkm_pllc4_out0, CLK_SOURCE_QSPI, 211, TEGRA_PERIPH_ON_APB, tegra_clk_qspi),
    I2C("vii2c", mux_pllp_pllc_clkm, CLK_SOURCE_VI_I2C, 208, tegra_clk_vi_i2c),
    MUX("mipibif", mux_pllp_clkm, CLK_SOURCE_MIPIBIF, 173, TEGRA_PERIPH_ON_APB, tegra_clk_mipibif),
    MUX("uartape", mux_pllp_pllc_clkm, CLK_SOURCE_UARTAPE, 212, TEGRA_PERIPH_ON_APB | TEGRA_PERIPH_NO_RESET, tegra_clk_uartape),
    MUX8("tsecb", mux_pllp_pllc2_c_c3_clkm, CLK_SOURCE_TSECB, 206, 0, tegra_clk_tsecb),
    MUX8("maud", mux_pllp_pllp_out3_clkm_clk32k_plla, CLK_SOURCE_MAUD, 202, TEGRA_PERIPH_ON_APB | TEGRA_PERIPH_NO_RESET, tegra_clk_maud),
    MUX8("dmic1", mux_dmic1, CLK_SOURCE_DMIC1, 161, TEGRA_PERIPH_ON_APB | TEGRA_PERIPH_NO_RESET, tegra_clk_dmic1),
    MUX8("dmic2", mux_dmic2, CLK_SOURCE_DMIC2, 162, TEGRA_PERIPH_ON_APB | TEGRA_PERIPH_NO_RESET, tegra_clk_dmic2),
    MUX8("dmic3", mux_dmic3, CLK_SOURCE_DMIC3, 197, TEGRA_PERIPH_ON_APB | TEGRA_PERIPH_NO_RESET, tegra_clk_dmic3),
    };
    static struct tegra_periph_init_data gate_clks[] = {
    GATE("rtc", "clk_32k", 4, TEGRA_PERIPH_ON_APB | TEGRA_PERIPH_NO_RESET, tegra_clk_rtc, 0),
    GATE("timer", "clk_m", 5, 0, tegra_clk_timer, CLK_IS_CRITICAL),
    GATE("isp", "clk_m", 23, 0, tegra_clk_isp, 0),
    GATE("vcp", "clk_m", 29, 0, tegra_clk_vcp, 0),
    GATE("ahbdma", "hclk", 33, 0, tegra_clk_ahbdma, 0),
    GATE("apbdma", "pclk", 34, 0, tegra_clk_apbdma, 0),
    GATE("kbc", "clk_32k", 36, TEGRA_PERIPH_ON_APB | TEGRA_PERIPH_NO_RESET, tegra_clk_kbc, 0),
    GATE("fuse", "clk_m", 39, TEGRA_PERIPH_ON_APB, tegra_clk_fuse, 0),
    GATE("fuse_burn", "clk_m", 39, TEGRA_PERIPH_ON_APB, tegra_clk_fuse_burn, 0),
    GATE("kfuse", "clk_m", 40, TEGRA_PERIPH_ON_APB, tegra_clk_kfuse, 0),
    GATE("apbif", "clk_m", 107, TEGRA_PERIPH_ON_APB, tegra_clk_apbif, 0),
    GATE("hda2hdmi", "clk_m", 128, TEGRA_PERIPH_ON_APB, tegra_clk_hda2hdmi, 0),
    GATE("bsea", "clk_m", 62, 0, tegra_clk_bsea, 0),
    GATE("bsev", "clk_m", 63, 0, tegra_clk_bsev, 0),
    GATE("mipi-cal", "clk72mhz", 56, 0, tegra_clk_mipi_cal, 0),
    GATE("usbd", "clk_m", 22, 0, tegra_clk_usbd, 0),
    GATE("usb2", "clk_m", 58, 0, tegra_clk_usb2, 0),
    GATE("usb3", "clk_m", 59, 0, tegra_clk_usb3, 0),
    GATE("csi", "pll_p_out3", 52, 0, tegra_clk_csi, 0),
    GATE("afi", "mselect", 72, 0, tegra_clk_afi, 0),
    GATE("csus", "clk_m", 92, TEGRA_PERIPH_NO_RESET, tegra_clk_csus, 0),
    GATE("dds", "clk_m", 150, TEGRA_PERIPH_ON_APB, tegra_clk_dds, 0),
    GATE("dp2", "clk_m", 152, TEGRA_PERIPH_ON_APB, tegra_clk_dp2, 0),
    GATE("dtv", "clk_m", 79, TEGRA_PERIPH_ON_APB, tegra_clk_dtv, 0),
    GATE("xusb_host", "xusb_host_src", 89, 0, tegra_clk_xusb_host, 0),
    GATE("xusb_ss", "xusb_ss_src", 156, 0, tegra_clk_xusb_ss, 0),
    GATE("xusb_dev", "xusb_dev_src", 95, 0, tegra_clk_xusb_dev, 0),
    GATE("emc", "emc_mux", 57, 0, tegra_clk_emc, CLK_IS_CRITICAL),
    GATE("sata_cold", "clk_m", 129, TEGRA_PERIPH_ON_APB, tegra_clk_sata_cold, 0),
    GATE("ispa", "isp", 23, 0, tegra_clk_ispa, 0),
    GATE("ispb", "isp", 3, 0, tegra_clk_ispb, 0),
    GATE("vim2_clk", "clk_m", 11, 0, tegra_clk_vim2_clk, 0),
    GATE("pcie", "clk_m", 70, 0, tegra_clk_pcie, 0),
    GATE("gpu", "pll_ref", 184, 0, tegra_clk_gpu, 0),
    GATE("pllg_ref", "pll_ref", 189, 0, tegra_clk_pll_g_ref, 0),
    GATE("hsic_trk", "usb2_hsic_trk", 209, TEGRA_PERIPH_NO_RESET, tegra_clk_hsic_trk, 0),
    GATE("usb2_trk", "usb2_hsic_trk", 210, TEGRA_PERIPH_NO_RESET, tegra_clk_usb2_trk, 0),
    GATE("xusb_gate", "osc", 143, 0, tegra_clk_xusb_gate, 0),
    GATE("pll_p_out_cpu", "pll_p", 223, 0, tegra_clk_pll_p_out_cpu, 0),
    GATE("pll_p_out_adsp", "pll_p", 187, 0, tegra_clk_pll_p_out_adsp, 0),
    GATE("apb2ape", "clk_m", 107, 0, tegra_clk_apb2ape, 0),
    GATE("cec", "pclk", 136, 0, tegra_clk_cec, 0),
    GATE("iqc1", "clk_m", 221, 0, tegra_clk_iqc1, 0),
    GATE("iqc2", "clk_m", 220, 0, tegra_clk_iqc1, 0),
    GATE("pll_a_out_adsp", "pll_a", 188, 0, tegra_clk_pll_a_out_adsp, 0),
    GATE("pll_a_out0_out_adsp", "pll_a", 188, 0, tegra_clk_pll_a_out0_out_adsp, 0),
    GATE("adsp", "aclk", 199, 0, tegra_clk_adsp, 0),
    GATE("adsp_neon", "aclk", 218, 0, tegra_clk_adsp_neon, 0),
    };
    static struct tegra_periph_init_data div_clks[] = {
    DIV8("usb2_hsic_trk", "osc", CLK_SOURCE_USB2_HSIC_TRK, tegra_clk_usb2_hsic_trk, 0),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_out_data {
    pub div_name: *mut c_char,
    pub pll_out_name: *mut c_char,
    pub offset: u32,
    pub clk_id: c_int,
    pub div_shift: u8,
    pub div_flags: u8,
    pub rst_shift: u8,
    pub lock: *mut spinlock_t,
}

    {\
    .div_name = "pll_p_out" #_num "_div",\
    .pll_out_name = "pll_p_out" #_num,\
    .offset = _offset,\
    .div_shift = _div_shift,\
    .div_flags = _div_flags | TEGRA_DIVIDER_FIXED |\
    TEGRA_DIVIDER_ROUND_UP,\
    .rst_shift = _rst_shift,\
    .clk_id = tegra_clk_ ## _id,\
    .lock = &_offset ##_lock,\
    }
    static struct pll_out_data pllp_out_clks[] = {
    PLL_OUT(1, PLLP_OUTA, 8, 0, 0, pll_p_out1),
    PLL_OUT(2, PLLP_OUTA, 24, 0, 16, pll_p_out2),
    PLL_OUT(2, PLLP_OUTA, 24, TEGRA_DIVIDER_INT, 16, pll_p_out2_int),
    PLL_OUT(3, PLLP_OUTB, 8, 0, 0, pll_p_out3),
    PLL_OUT(4, PLLP_OUTB, 24, 0, 16, pll_p_out4),
    PLL_OUT(5, PLLP_OUTC, 24, 0, 16, pll_p_out5),
    };
    static void __init periph_clk_init(void __iomem *clk_base,
    struct tegra_clk *tegra_clks)
    {
    int i;
    struct clk *clk;
    struct clk **dt_clk;
    for (i = 0; i < ARRAY_SIZE(periph_clks); i++) {
    const struct tegra_clk_periph_regs *bank;
    struct tegra_periph_init_data *data;
    data = periph_clks + i;
    dt_clk = tegra_lookup_dt_id(data.clk_id, tegra_clks);
    if (!dt_clk)
    continue;
    bank = get_reg_bank(data.periph.gate.clk_num);
    if (!bank)
    continue;
    data.periph.gate.regs = bank;
    clk = tegra_clk_register_periph_data(clk_base, data);
// dt_clk = clk;
    }
    }
    static void __init gate_clk_init(void __iomem *clk_base,
    struct tegra_clk *tegra_clks)
    {
    int i;
    struct clk *clk;
    struct clk **dt_clk;
    for (i = 0; i < ARRAY_SIZE(gate_clks); i++) {
    struct tegra_periph_init_data *data;
    data = gate_clks + i;
    dt_clk = tegra_lookup_dt_id(data.clk_id, tegra_clks);
    if (!dt_clk)
    continue;
    clk = tegra_clk_register_periph_gate(data.name,
    data.p.parent_name, data.periph.gate.flags,
    clk_base, data.flags,
    data.periph.gate.clk_num,
    periph_clk_enb_refcnt);
// dt_clk = clk;
    }
    }
    static void __init div_clk_init(void __iomem *clk_base,
    struct tegra_clk *tegra_clks)
    {
    int i;
    struct clk *clk;
    struct clk **dt_clk;
    for (i = 0; i < ARRAY_SIZE(div_clks); i++) {
    struct tegra_periph_init_data *data;
    data = div_clks + i;
    dt_clk = tegra_lookup_dt_id(data.clk_id, tegra_clks);
    if (!dt_clk)
    continue;
    clk = tegra_clk_register_divider(data.name,
    data.p.parent_name, clk_base + data.offset,
    data.flags, data.periph.divider.flags,
    data.periph.divider.shift,
    data.periph.divider.width,
    data.periph.divider.frac_width,
    data.periph.divider.lock);
// dt_clk = clk;
    }
    }
    static void __init init_pllp(void __iomem *clk_base, void __iomem *pmc_base,
    struct tegra_clk *tegra_clks,
    struct tegra_clk_pll_params *pll_params)
    {
    struct clk *clk;
    struct clk **dt_clk;
    int i;
    dt_clk = tegra_lookup_dt_id(tegra_clk_pll_p, tegra_clks);
    if (dt_clk) {
// PLLP
    clk = tegra_clk_register_pll("pll_p", "pll_ref", clk_base,
    pmc_base, 0, pll_params, core::ptr::null_mut());
    clk_register_clkdev(clk, "pll_p", core::ptr::null_mut());
// dt_clk = clk;
    }
    for (i = 0; i < ARRAY_SIZE(pllp_out_clks); i++) {
    struct pll_out_data *data;
    data = pllp_out_clks + i;
    dt_clk = tegra_lookup_dt_id(data.clk_id, tegra_clks);
    if (!dt_clk)
    continue;
    clk = tegra_clk_register_divider(data.div_name, "pll_p",
    clk_base + data.offset, 0, data.div_flags,
    data.div_shift, 8, 1, data.lock);
    clk = tegra_clk_register_pll_out(data.pll_out_name,
    data.div_name, clk_base + data.offset,
    data.rst_shift + 1, data.rst_shift,
    CLK_IGNORE_UNUSED | CLK_SET_RATE_PARENT, 0,
    data.lock);
// dt_clk = clk;
    }
    dt_clk = tegra_lookup_dt_id(tegra_clk_pll_p_out_cpu,
    tegra_clks);
    if (dt_clk) {
//
// Tegra210 has control on enabling/disabling PLLP branches to
// CPU, register a gate clock "pll_p_out_cpu" for this gating
// function and parent "pll_p_out4" to it, so when we are
// re-parenting CPU off from "pll_p_out4" the PLLP branching to
// CPU can be disabled automatically.
//
    clk = tegra_clk_register_divider("pll_p_out4_div",
    "pll_p_out_cpu", clk_base + PLLP_OUTB, 0, 0, 24,
    8, 1, &PLLP_OUTB_lock);
    dt_clk = tegra_lookup_dt_id(tegra_clk_pll_p_out4_cpu, tegra_clks);
    if (dt_clk) {
    clk = tegra_clk_register_pll_out("pll_p_out4",
    "pll_p_out4_div", clk_base + PLLP_OUTB,
    17, 16, CLK_IGNORE_UNUSED |
    CLK_SET_RATE_PARENT, 0,
    &PLLP_OUTB_lock);
// dt_clk = clk;
    }
    }
    dt_clk = tegra_lookup_dt_id(tegra_clk_pll_p_out_hsio, tegra_clks);
    if (dt_clk) {
// PLLP_OUT_HSIO
    clk = clk_register_gate(core::ptr::null_mut(), "pll_p_out_hsio", "pll_p",
    CLK_SET_RATE_PARENT | CLK_IGNORE_UNUSED,
    clk_base + PLLP_MISC1, 29, 0, core::ptr::null_mut());
// dt_clk = clk;
    }
    dt_clk = tegra_lookup_dt_id(tegra_clk_pll_p_out_xusb, tegra_clks);
    if (dt_clk) {
// PLLP_OUT_XUSB
    clk = clk_register_gate(core::ptr::null_mut(), "pll_p_out_xusb",
    "pll_p_out_hsio", CLK_SET_RATE_PARENT |
    CLK_IGNORE_UNUSED, clk_base + PLLP_MISC1, 28, 0,
    core::ptr::null_mut());
    clk_register_clkdev(clk, "pll_p_out_xusb", core::ptr::null_mut());
// dt_clk = clk;
    }
    }
    void __init tegra_periph_clk_init(void __iomem *clk_base,
    void __iomem *pmc_base, struct tegra_clk *tegra_clks,
    struct tegra_clk_pll_params *pll_params)
    {
    init_pllp(clk_base, pmc_base, tegra_clks, pll_params);
    periph_clk_init(clk_base, tegra_clks);
    gate_clk_init(clk_base, tegra_clks);
    div_clk_init(clk_base, tegra_clks);
    }

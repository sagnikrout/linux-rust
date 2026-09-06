//! Automatically rewritten from C to Rust
//! Source: drivers/clk/tegra/clk-tegra-audio.c
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

pub const AUDIO_SYNC_CLK_I2S0: c_uint = 0x4a0;
pub const AUDIO_SYNC_CLK_I2S1: c_uint = 0x4a4;
pub const AUDIO_SYNC_CLK_I2S2: c_uint = 0x4a8;
pub const AUDIO_SYNC_CLK_I2S3: c_uint = 0x4ac;
pub const AUDIO_SYNC_CLK_I2S4: c_uint = 0x4b0;
pub const AUDIO_SYNC_CLK_SPDIF: c_uint = 0x4b4;
pub const AUDIO_SYNC_CLK_DMIC1: c_uint = 0x560;
pub const AUDIO_SYNC_CLK_DMIC2: c_uint = 0x564;
pub const AUDIO_SYNC_CLK_DMIC3: c_uint = 0x6b8;
pub const AUDIO_SYNC_DOUBLER: c_uint = 0x49c;
pub const PLLA_OUT: c_uint = 0xb4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_sync_source_initdata {
    pub name: *mut c_char,
    pub rate: c_ulong,
    pub max_rate: c_ulong,
    pub clk_id: c_int,
}

    {\
    .name		= #_name,\
    .clk_id		= tegra_clk_ ## _name,\
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_audio_clk_initdata {
    pub gate_name: *mut c_char,
    pub mux_name: *mut c_char,
    pub offset: u32,
    pub gate_clk_id: c_int,
    pub mux_clk_id: c_int,
}

    {\
    .gate_name	= #_name,\
    .mux_name	= #_name"_mux",\
    .offset		= _offset,\
    .gate_clk_id	= tegra_clk_ ## _name,\
    .mux_clk_id	= tegra_clk_ ## _name ## _mux,\
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_audio2x_clk_initdata {
    pub parent: *mut c_char,
    pub gate_name: *mut c_char,
    pub name_2x: *mut c_char,
    pub div_name: *mut c_char,
    pub clk_id: c_int,
    pub clk_num: c_int,
    pub div_offset: u8,
}

    {\
    .parent		= #_name,\
    .gate_name	= #_name"_2x",\
    .name_2x	= #_name"_doubler",\
    .div_name	= #_name"_div",\
    .clk_id		= tegra_clk_ ## _name ## _2x,\
    .clk_num	= _num,\
    .div_offset	= _offset,\
    }
    static DEFINE_SPINLOCK(clk_doubler_lock);
    static const char * const mux_audio_sync_clk[] = { "spdif_in_sync",
    "i2s0_sync", "i2s1_sync", "i2s2_sync", "i2s3_sync", "i2s4_sync",
    "pll_a_out0", "vimclk_sync",
    };
    static const char * const mux_dmic_sync_clk[] = { "unused", "i2s0_sync",
    "i2s1_sync", "i2s2_sync", "i2s3_sync", "i2s4_sync", "pll_a_out0",
    "vimclk_sync",
    };
    static struct tegra_sync_source_initdata sync_source_clks[] __initdata = {
    SYNC(spdif_in_sync),
    SYNC(i2s0_sync),
    SYNC(i2s1_sync),
    SYNC(i2s2_sync),
    SYNC(i2s3_sync),
    SYNC(i2s4_sync),
    SYNC(vimclk_sync),
    };
    static struct tegra_audio_clk_initdata audio_clks[] = {
    AUDIO(audio0, AUDIO_SYNC_CLK_I2S0),
    AUDIO(audio1, AUDIO_SYNC_CLK_I2S1),
    AUDIO(audio2, AUDIO_SYNC_CLK_I2S2),
    AUDIO(audio3, AUDIO_SYNC_CLK_I2S3),
    AUDIO(audio4, AUDIO_SYNC_CLK_I2S4),
    AUDIO(spdif, AUDIO_SYNC_CLK_SPDIF),
    };
    static struct tegra_audio_clk_initdata dmic_clks[] = {
    AUDIO(dmic1_sync_clk, AUDIO_SYNC_CLK_DMIC1),
    AUDIO(dmic2_sync_clk, AUDIO_SYNC_CLK_DMIC2),
    AUDIO(dmic3_sync_clk, AUDIO_SYNC_CLK_DMIC3),
    };
    static struct tegra_audio2x_clk_initdata audio2x_clks[] = {
    AUDIO2X(audio0, 113, 24),
    AUDIO2X(audio1, 114, 25),
    AUDIO2X(audio2, 115, 26),
    AUDIO2X(audio3, 116, 27),
    AUDIO2X(audio4, 117, 28),
    AUDIO2X(spdif, 118, 29),
    };
    static void __init tegra_audio_sync_clk_init(void __iomem *clk_base,
    struct tegra_clk *tegra_clks,
    struct tegra_audio_clk_initdata *sync,
    int num_sync_clks,
    const char * const *mux_names,
    int num_mux_inputs)
    {
    struct clk *clk;
    struct clk **dt_clk;
    struct tegra_audio_clk_initdata *data;
    int i;
    for (i = 0, data = sync; i < num_sync_clks; i++, data++) {
    dt_clk = tegra_lookup_dt_id(data.mux_clk_id, tegra_clks);
    if (!dt_clk)
    continue;
    clk = clk_register_mux(core::ptr::null_mut(), data.mux_name, mux_names,
    num_mux_inputs,
    CLK_SET_RATE_NO_REPARENT,
    clk_base + data.offset, 0, 3, 0,
    core::ptr::null_mut());
// dt_clk = clk;
    dt_clk = tegra_lookup_dt_id(data.gate_clk_id, tegra_clks);
    if (!dt_clk)
    continue;
    clk = clk_register_gate(core::ptr::null_mut(), data.gate_name, data.mux_name,
    0, clk_base + data.offset, 4,
    CLK_GATE_SET_TO_DISABLE, core::ptr::null_mut());
// dt_clk = clk;
    }
    }
    void __init tegra_audio_clk_init(void __iomem *clk_base,
    void __iomem *pmc_base, struct tegra_clk *tegra_clks,
    struct tegra_audio_clk_info *audio_info,
    unsigned int num_plls, unsigned long sync_max_rate)
    {
    struct clk *clk;
    struct clk **dt_clk;
    int i;
    if (!audio_info || num_plls < 1) {
    pr_err("No audio data passed to tegra_audio_clk_init\n");
    WARN_ON(1);
    return;
    }
    for (i = 0; i < num_plls; i++) {
    struct tegra_audio_clk_info *info = &audio_info[i];
    dt_clk = tegra_lookup_dt_id(info.clk_id, tegra_clks);
    if (dt_clk) {
    clk = tegra_clk_register_pll(info.name, info.parent,
    clk_base, pmc_base, 0, info.pll_params,
    core::ptr::null_mut());
// dt_clk = clk;
    }
    }
// PLLA_OUT0
    dt_clk = tegra_lookup_dt_id(tegra_clk_pll_a_out0, tegra_clks);
    if (dt_clk) {
    clk = tegra_clk_register_divider("pll_a_out0_div", "pll_a",
    clk_base + PLLA_OUT, 0, TEGRA_DIVIDER_ROUND_UP,
    8, 8, 1, core::ptr::null_mut());
    clk = tegra_clk_register_pll_out("pll_a_out0", "pll_a_out0_div",
    clk_base + PLLA_OUT, 1, 0, CLK_IGNORE_UNUSED |
    CLK_SET_RATE_PARENT, 0, core::ptr::null_mut());
// dt_clk = clk;
    }
    for (i = 0; i < ARRAY_SIZE(sync_source_clks); i++) {
    struct tegra_sync_source_initdata *data;
    data = &sync_source_clks[i];
    dt_clk = tegra_lookup_dt_id(data.clk_id, tegra_clks);
    if (!dt_clk)
    continue;
    clk = tegra_clk_register_sync_source(data.name, sync_max_rate);
// dt_clk = clk;
    }
    tegra_audio_sync_clk_init(clk_base, tegra_clks, audio_clks,
    ARRAY_SIZE(audio_clks), mux_audio_sync_clk,
    ARRAY_SIZE(mux_audio_sync_clk));
// make sure the DMIC sync clocks have a valid parent
    for (i = 0; i < ARRAY_SIZE(dmic_clks); i++)
    writel_relaxed(1, clk_base + dmic_clks[i].offset);
    tegra_audio_sync_clk_init(clk_base, tegra_clks, dmic_clks,
    ARRAY_SIZE(dmic_clks), mux_dmic_sync_clk,
    ARRAY_SIZE(mux_dmic_sync_clk));
    for (i = 0; i < ARRAY_SIZE(audio2x_clks); i++) {
    struct tegra_audio2x_clk_initdata *data;
    data = &audio2x_clks[i];
    dt_clk = tegra_lookup_dt_id(data.clk_id, tegra_clks);
    if (!dt_clk)
    continue;
    clk = clk_register_fixed_factor(core::ptr::null_mut(), data.name_2x,
    data.parent, CLK_SET_RATE_PARENT, 2, 1);
    clk = tegra_clk_register_divider(data.div_name,
    data.name_2x, clk_base + AUDIO_SYNC_DOUBLER,
    0, 0, data.div_offset, 1, 0,
    &clk_doubler_lock);
    clk = tegra_clk_register_periph_gate(data.gate_name,
    data.div_name, TEGRA_PERIPH_NO_RESET,
    clk_base, CLK_SET_RATE_PARENT, data.clk_num,
    periph_clk_enb_refcnt);
// dt_clk = clk;
    }
    }

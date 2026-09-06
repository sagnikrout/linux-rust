//! Automatically rewritten from C to Rust
//! Source: drivers/clk/cix/clk-sky1-audss.c
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
// Copyright 2026 Cix Technology Group Co., Ltd.

pub const INFO_HIFI0: c_uint = 0x00;
pub const INFO_CLK_GATE: c_uint = 0x10;
pub const INFO_CLK_DIV: c_uint = 0x14;
pub const INFO_CLK_MUX: c_uint = 0x18;
pub const INFO_MCLK: c_uint = 0x70;
pub const SKY1_AUDSS_CLK_PARENTS_CNT: c_int = 4;

    static u32 sky1_reg_save[][2] = {
    { INFO_HIFI0,  0 },
    { INFO_CLK_GATE,  0 },
    { INFO_CLK_DIV, 0 },
    { INFO_CLK_MUX, 0 },
    { INFO_MCLK, 0 },
    };
    static const char * const sky1_audss_clk_names[SKY1_AUDSS_CLK_PARENTS_CNT] = {
    "x8k", "x11k", "sys", "48m",
    };
    static const u32 sky1_clk_rate_default[SKY1_AUDSS_CLK_PARENTS_CNT] = {
    294912000,
    270950400,
    800000000,
    48000000,
    };
    static const char * const dsp_clk_parent[] = {
    "audio_clk4"
    };
    static const char * const dsp_bclk_parent[] = {
    "audio_clk4_div2"
    };
    static const char * const dsp_pbclk_parent[] = {
    "audio_clk4_div4"
    };
    static const char * const sram_axi_parent[] = {
    "audio_clk4_div2"
    };
    static const char * const hda_sys_parent[] = {
    "audio_clk4_div2"
    };
    static const char * const hda_hda_parent[] = {
    "audio_clk5"
    };
    static const char * const dmac_axi_parent[] = {
    "audio_clk4_div2"
    };
    static const char * const wdg_apb_parent[] = {
    "audio_clk5_div2"
    };
    static const char * const wdg_wdg_parent[] = {
    "audio_clk5_div2"
    };
    static const char * const timer_apb_parent[] = {
    "audio_clk4_div4"
    };
    static const char * const timer_timer_parent[] = {
    "audio_clk5_div2"
    };
    static const char * const mailbox_apb_parent[] = {
    "audio_clk4_div4"
    };
    static const char * const i2s_apb_parent[] = {
    "audio_clk4_div4"
    };
    static const char * const i2s0_parents[] = {
    "audio_clk0", "audio_clk2"
    };
    static const char * const i2s1_parents[] = {
    "audio_clk0", "audio_clk2"
    };
    static const char * const i2s2_parents[] = {
    "audio_clk0", "audio_clk2"
    };
    static const char * const i2s3_parents[] = {
    "audio_clk0", "audio_clk2"
    };
    static const char * const i2s4_parents[] = {
    "audio_clk0", "audio_clk2"
    };
    static const char * const i2s5_parents[] = {
    "audio_clk0", "audio_clk2"
    };
    static const char * const i2s6_parents[] = {
    "audio_clk0", "audio_clk2"
    };
    static const char * const i2s7_parents[] = {
    "audio_clk0", "audio_clk2"
    };
    static const char * const i2s8_parents[] = {
    "audio_clk0", "audio_clk2"
    };
    static const char * const i2s9_parents[] = {
    "audio_clk0", "audio_clk2"
    };
    static const char * const mclk_parents[] = {
    "audio_clk0", "audio_clk2"
    };
    static const u32 i2s3_mux_table[] = { 0, 2 };
    static const u32 i2s4_mux_table[] = { 0, 2 };
//
// audss composite clock definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct muxdiv_cfg {
    pub offset: c_int,
    pub shift: u8,
    pub width: u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gate_cfg {
    pub offset: c_int,
    pub shift: u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct composite_clk_cfg {
    pub id: u32,
    pub name: *const *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: c_int,
    pub mux_table: *const u32,
    pub mux_cfg: *mut muxdiv_cfg,
    pub div_cfg: *mut muxdiv_cfg,
    pub gate_cfg: *mut gate_cfg,
    pub flags: c_ulong,
}

// Macro flag: #define CFG(_id,\
    _name,\
    _parent_names,\
    _mux_table,\
    _mux_offset, _mux_shift, _mux_width, _mux_flags,\
    _div_offset, _div_shift, _div_width, _div_flags,\
    _gate_offset, _gate_shift, _gate_flags,\
    _flags)\
    {\
    .id = _id,\
    .name = _name,\
    .parent_names = _parent_names,\
    .num_parents = ARRAY_SIZE(_parent_names),\
    .mux_table = _mux_table,\
    .mux_cfg = &(struct muxdiv_cfg) { _mux_offset, _mux_shift, _mux_width, _mux_flags },\
    .div_cfg = &(struct muxdiv_cfg) { _div_offset, _div_shift, _div_width, _div_flags },\
    .gate_cfg = &(struct gate_cfg) { _gate_offset, _gate_shift, _gate_flags },\
    .flags = _flags,\
    }
    static const struct composite_clk_cfg sky1_audss_clks[] = {
// dsp
    CFG(CLK_DSP_CLK,
    "audss_dsp_clk",
    dsp_clk_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    INFO_HIFI0, 0, 0,
    0),
    CFG(CLK_DSP_BCLK,
    "audss_dsp_bclk",
    dsp_bclk_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    -1, 0, 0,
    0),
    CFG(CLK_DSP_PBCLK,
    "audss_dsp_pbclk",
    dsp_pbclk_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    -1, 0, 0,
    0),
// sram
    CFG(CLK_SRAM_AXI,
    "audss_sram_axi",
    sram_axi_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    INFO_CLK_GATE, 16, 0,
    0),
// hda
    CFG(CLK_HDA_SYS,
    "audss_hda_sys",
    hda_sys_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    INFO_CLK_GATE, 14, 0,
    0),
    CFG(CLK_HDA_HDA,
    "audss_hda_hda",
    hda_hda_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    -1, 0, 0, 0,
    INFO_CLK_GATE, 14, 0,
    0),
// dmac
    CFG(CLK_DMAC_AXI,
    "audss_dmac_axi",
    dmac_axi_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    INFO_CLK_GATE, 15, 0,
    0),
// wdg
    CFG(CLK_WDG_APB,
    "audss_wdg_apb",
    wdg_apb_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    -1, 0, 0, 0,
    INFO_CLK_GATE, 10, 0,
    0),
    CFG(CLK_WDG_WDG,
    "audss_wdg_wdg",
    wdg_wdg_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    -1, 0, 0, 0,
    INFO_CLK_GATE, 10, 0,
    0),
// timer
    CFG(CLK_TIMER_APB,
    "audss_timer_apb",
    timer_apb_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    INFO_CLK_GATE, 11, 0,
    0),
    CFG(CLK_TIMER_TIMER,
    "audss_timer_timer",
    timer_timer_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    -1, 0, 0, 0,
    INFO_CLK_GATE, 11, 0,
    0),
// mailbox: mb0(ap->dsp), mb1(dsp->ap)
    CFG(CLK_MB_0_APB,
    "audss_mb_0_apb",
    mailbox_apb_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    -1, 0, 0, 0,
    INFO_CLK_GATE, 12, 0,
    0),
    CFG(CLK_MB_1_APB,
    "audss_mb_1_apb",
    mailbox_apb_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    -1, 0, 0, 0,
    INFO_CLK_GATE, 13, 0,
    0),
// i2s
    CFG(CLK_I2S0_APB,
    "audss_i2s0_apb",
    i2s_apb_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    INFO_CLK_GATE, 0, 0,
    0),
    CFG(CLK_I2S1_APB,
    "audss_i2s1_apb",
    i2s_apb_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    INFO_CLK_GATE, 1, 0,
    0),
    CFG(CLK_I2S2_APB,
    "audss_i2s2_apb",
    i2s_apb_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    INFO_CLK_GATE, 2, 0,
    0),
    CFG(CLK_I2S3_APB,
    "audss_i2s3_apb",
    i2s_apb_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    INFO_CLK_GATE, 3, 0,
    0),
    CFG(CLK_I2S4_APB,
    "audss_i2s4_apb",
    i2s_apb_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    INFO_CLK_GATE, 4, 0,
    0),
    CFG(CLK_I2S5_APB,
    "audss_i2s5_apb",
    i2s_apb_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    INFO_CLK_GATE, 5, 0,
    0),
    CFG(CLK_I2S6_APB,
    "audss_i2s6_apb",
    i2s_apb_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    INFO_CLK_GATE, 6, 0,
    0),
    CFG(CLK_I2S7_APB,
    "audss_i2s7_apb",
    i2s_apb_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    INFO_CLK_GATE, 7, 0,
    0),
    CFG(CLK_I2S8_APB,
    "audss_i2s8_apb",
    i2s_apb_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    INFO_CLK_GATE, 8, 0,
    0),
    CFG(CLK_I2S9_APB,
    "audss_i2s9_apb",
    i2s_apb_parent,
    core::ptr::null_mut(),
    -1, 0, 0, 0,
    INFO_CLK_DIV, 0, 2, 0,
    INFO_CLK_GATE, 9, 0,
    0),
    CFG(CLK_I2S0,
    "audss_i2s0",
    i2s0_parents,
    core::ptr::null_mut(),
    INFO_CLK_MUX, 0, 2, 0,
    INFO_CLK_DIV, 2, 2, 0,
    INFO_CLK_GATE, 0, 0,
    0),
    CFG(CLK_I2S1,
    "audss_i2s1",
    i2s1_parents,
    core::ptr::null_mut(),
    INFO_CLK_MUX, 2, 2, 0,
    INFO_CLK_DIV, 4, 2, 0,
    INFO_CLK_GATE, 1, 0,
    0),
    CFG(CLK_I2S2,
    "audss_i2s2",
    i2s2_parents,
    core::ptr::null_mut(),
    INFO_CLK_MUX, 4, 2, 0,
    INFO_CLK_DIV, 6, 2, 0,
    INFO_CLK_GATE, 2, 0,
    0),
    CFG(CLK_I2S3,
    "audss_i2s3",
    i2s3_parents,
    i2s3_mux_table,
    INFO_CLK_MUX, 6, 2, 0,
    INFO_CLK_DIV, 8, 2, 0,
    INFO_CLK_GATE, 3, 0,
    0),
    CFG(CLK_I2S4,
    "audss_i2s4",
    i2s4_parents,
    i2s4_mux_table,
    INFO_CLK_MUX, 8, 2, 0,
    INFO_CLK_DIV, 10, 2, 0,
    INFO_CLK_GATE, 4, 0,
    0),
    CFG(CLK_I2S5,
    "audss_i2s5",
    i2s5_parents,
    core::ptr::null_mut(),
    INFO_CLK_MUX, 10, 2, 0,
    INFO_CLK_DIV, 12, 2, 0,
    INFO_CLK_GATE, 5, 0,
    0),
    CFG(CLK_I2S6,
    "audss_i2s6",
    i2s6_parents,
    core::ptr::null_mut(),
    INFO_CLK_MUX, 12, 2, 0,
    INFO_CLK_DIV, 14, 2, 0,
    INFO_CLK_GATE, 6, 0,
    0),
    CFG(CLK_I2S7,
    "audss_i2s7",
    i2s7_parents,
    core::ptr::null_mut(),
    INFO_CLK_MUX, 14, 2, 0,
    INFO_CLK_DIV, 16, 2, 0,
    INFO_CLK_GATE, 7, 0,
    0),
    CFG(CLK_I2S8,
    "audss_i2s8",
    i2s8_parents,
    core::ptr::null_mut(),
    INFO_CLK_MUX, 16, 2, 0,
    INFO_CLK_DIV, 18, 2, 0,
    INFO_CLK_GATE, 8, 0,
    0),
    CFG(CLK_I2S9,
    "audss_i2s9",
    i2s9_parents,
    core::ptr::null_mut(),
    INFO_CLK_MUX, 18, 2, 0,
    INFO_CLK_DIV, 20, 2, 0,
    INFO_CLK_GATE, 9, 0,
    0),
// mclk
    CFG(CLK_MCLK0,
    "audss_mclk0",
    mclk_parents,
    core::ptr::null_mut(),
    INFO_MCLK, 5, 1, 0,
    -1, 0, 0, 0,
    INFO_MCLK, 0, 0,
    0),
    CFG(CLK_MCLK1,
    "audss_mclk1",
    mclk_parents,
    core::ptr::null_mut(),
    INFO_MCLK, 6, 1, 0,
    -1, 0, 0, 0,
    INFO_MCLK, 1, 0,
    0),
    CFG(CLK_MCLK2,
    "audss_mclk2",
    mclk_parents,
    core::ptr::null_mut(),
    INFO_MCLK, 7, 1, 0,
    -1, 0, 0, 0,
    INFO_MCLK, 2, 0,
    0),
    CFG(CLK_MCLK3,
    "audss_mclk3",
    mclk_parents,
    core::ptr::null_mut(),
    INFO_MCLK, 8, 1, 0,
    -1, 0, 0, 0,
    INFO_MCLK, 3, 0,
    0),
    CFG(CLK_MCLK4,
    "audss_mclk4",
    mclk_parents,
    core::ptr::null_mut(),
    INFO_MCLK, 9, 1, 0,
    -1, 0, 0, 0,
    INFO_MCLK, 4, 0,
    0),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky1_audss_clks_devtype_data {
    pub (*reg_save)[2]: *mut u32,
    pub reg_save_size: usize,
    pub clk_names: *const *const c_char,
    pub clk_num: usize,
    pub clk_rate_default: *const u32,
    pub clk_cfg: *const composite_clk_cfg,
    pub clk_cfg_size: usize,
}

    static const struct regmap_config sky1_audss_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky1_audss_clks_priv {
    pub dev: *mut device,
    pub regmap_cru: *mut regmap,
    pub rst_noc: *mut reset_control,
    pub clks: [*mut clk; SKY1_AUDSS_CLK_PARENTS_CNT],
    pub devtype_data: *const sky1_audss_clks_devtype_data,
    pub lock: spinlock_t,
    pub clk_data: *mut clk_hw_onecell_data,
}

#[no_mangle]
unsafe extern "C" fn sky1_audss_reset_controller_register(dev: *mut device) -> c_int {
    static int sky1_audss_reset_controller_register(struct device *dev)
    {
    struct auxiliary_device *adev;
    if (!of_property_present(dev.of_node, "#reset-cells"))
    return 0;
    adev = devm_auxiliary_device_create(dev, "reset", core::ptr::null_mut());
    if (!adev)
    return -ENODEV;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn sky1_audss_reset_controller_register(dev: *mut device) -> c_int {
    static int sky1_audss_reset_controller_register(struct device *dev)
    {
    return 0;
    }

//
// clk_ops for audss clock mux/divider/gate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky1_clk_divider {
    pub div: clk_divider,
    pub regmap: *mut regmap,
    pub offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky1_clk_gate {
    pub gate: clk_gate,
    pub regmap: *mut regmap,
    pub offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky1_clk_mux {
    pub mux: clk_mux,
    pub regmap: *mut regmap,
    pub offset: c_int,
}

    static inline struct sky1_clk_mux *to_sky1_clk_mux(struct clk_mux *mux)
    {
    return container_of(mux, struct sky1_clk_mux, mux);
    }
#[no_mangle]
unsafe extern "C" fn sky1_audss_clk_mux_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 sky1_audss_clk_mux_get_parent(struct clk_hw *hw)
    {
    struct clk_mux *mux = to_clk_mux(hw);
    struct sky1_clk_mux *sky1_mux = to_sky1_clk_mux(mux);
    u32 val;
    regmap_read(sky1_mux.regmap, sky1_mux.offset, &val);
    val = val >> mux.shift;
    val &= mux.mask;
    return clk_mux_val_to_index(hw, mux.table, mux.flags, val);
    }
#[no_mangle]
unsafe extern "C" fn sky1_audss_clk_mux_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int sky1_audss_clk_mux_set_parent(struct clk_hw *hw, u8 index)
    {
    struct clk_mux *mux = to_clk_mux(hw);
    let mut val: u32 = clk_mux_index_to_val(mux.table, mux.flags, index);
    struct sky1_clk_mux *sky1_mux = to_sky1_clk_mux(mux);
    let mut flags: c_ulong = 0;
    u32 reg;
    if (mux.lock)
    spin_lock_irqsave(mux.lock, flags);
    else
    __acquire(mux.lock);
    if (mux.flags & CLK_MUX_HIWORD_MASK) {
    reg = mux.mask << (mux.shift + 16);
    } else {
    regmap_read(sky1_mux.regmap, sky1_mux.offset, &reg);
    reg &= ~(mux.mask << mux.shift);
    }
    val = val << mux.shift;
    reg |= val;
    regmap_write(sky1_mux.regmap, sky1_mux.offset, reg);
    if (mux.lock)
    spin_unlock_irqrestore(mux.lock, flags);
    else
    __release(mux.lock);
    return 0;
    }
    static int sky1_audss_clk_mux_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_mux *mux = to_clk_mux(hw);
    return clk_mux_determine_rate_flags(hw, req, mux.flags);
    }
    static const struct clk_ops sky1_audss_clk_mux_ops = {
    .get_parent = sky1_audss_clk_mux_get_parent,
    .set_parent = sky1_audss_clk_mux_set_parent,
    .determine_rate = sky1_audss_clk_mux_determine_rate,
    };
    static inline struct sky1_clk_divider *to_sky1_clk_divider(struct clk_divider *div)
    {
    return container_of(div, struct sky1_clk_divider, div);
    }
    static unsigned long sky1_audss_clk_divider_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_divider *divider = to_clk_divider(hw);
    struct sky1_clk_divider *sky1_div = to_sky1_clk_divider(divider);
    unsigned int val;
    regmap_read(sky1_div.regmap, sky1_div.offset, &val);
    val = val >> divider.shift;
    val &= clk_div_mask(divider.width);
    return divider_recalc_rate(hw, parent_rate, val, divider.table,
    divider.flags, divider.width);
    }
    static int sky1_audss_clk_divider_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_divider *divider = to_clk_divider(hw);
    struct sky1_clk_divider *sky1_div = to_sky1_clk_divider(divider);
// if read only, just return current value
    if (divider.flags & CLK_DIVIDER_READ_ONLY) {
    u32 val;
    regmap_read(sky1_div.regmap, sky1_div.offset, &val);
    val = val >> divider.shift;
    val &= clk_div_mask(divider.width);
    return divider_ro_determine_rate(hw, req, divider.table,
    divider.width,
    divider.flags, val);
    }
    return divider_determine_rate(hw, req, divider.table, divider.width,
    divider.flags);
    }
    static int sky1_audss_clk_divider_set_rate(struct clk_hw *hw,
    unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_divider *divider = to_clk_divider(hw);
    struct sky1_clk_divider *sky1_div = to_sky1_clk_divider(divider);
    int value;
    let mut flags: c_ulong = 0;
    u32 val;
    value = divider_get_val(rate, parent_rate, divider.table,
    divider.width, divider.flags);
    if (value < 0)
    return value;
    if (divider.lock)
    spin_lock_irqsave(divider.lock, flags);
    else
    __acquire(divider.lock);
    if (divider.flags & CLK_DIVIDER_HIWORD_MASK) {
    val = clk_div_mask(divider.width) << (divider.shift + 16);
    } else {
    regmap_read(sky1_div.regmap, sky1_div.offset, &val);
    val &= ~(clk_div_mask(divider.width) << divider.shift);
    }
    val |= (u32)value << divider.shift;
    regmap_write(sky1_div.regmap, sky1_div.offset, val);
    if (divider.lock)
    spin_unlock_irqrestore(divider.lock, flags);
    else
    __release(divider.lock);
    return 0;
    }
    static const struct clk_ops sky1_audss_clk_divider_ops = {
    .recalc_rate = sky1_audss_clk_divider_recalc_rate,
    .determine_rate = sky1_audss_clk_divider_determine_rate,
    .set_rate = sky1_audss_clk_divider_set_rate,
    };
    static inline struct sky1_clk_gate *to_sky1_clk_gate(struct clk_gate *gate)
    {
    return container_of(gate, struct sky1_clk_gate, gate);
    }
#[no_mangle]
unsafe extern "C" fn sky1_audss_clk_gate_endisable(hw: *mut clk_hw, enable: c_int) {
    static void sky1_audss_clk_gate_endisable(struct clk_hw *hw, int enable)
    {
    struct clk_gate *gate = to_clk_gate(hw);
    struct sky1_clk_gate *sky1_gate = to_sky1_clk_gate(gate);
    let mut set: c_int = gate.flags & CLK_GATE_SET_TO_DISABLE ? 1 : 0;
    let mut flags: c_ulong = 0;
    u32 reg;
    set ^= enable;
    if (gate.lock)
    spin_lock_irqsave(gate.lock, flags);
    else
    __acquire(gate.lock);
    if (gate.flags & CLK_GATE_HIWORD_MASK) {
    reg = BIT(gate.bit_idx + 16);
    if (set)
    reg |= BIT(gate.bit_idx);
    } else {
    regmap_read(sky1_gate.regmap, sky1_gate.offset, &reg);
    if (set)
    reg |= BIT(gate.bit_idx);
    else
    reg &= ~BIT(gate.bit_idx);
    }
    regmap_write(sky1_gate.regmap, sky1_gate.offset, reg);
    if (gate.lock)
    spin_unlock_irqrestore(gate.lock, flags);
    else
    __release(gate.lock);
    }
#[no_mangle]
unsafe extern "C" fn sky1_audss_clk_gate_enable(hw: *mut clk_hw) -> c_int {
    static int sky1_audss_clk_gate_enable(struct clk_hw *hw)
    {
    sky1_audss_clk_gate_endisable(hw, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sky1_audss_clk_gate_disable(hw: *mut clk_hw) {
    static void sky1_audss_clk_gate_disable(struct clk_hw *hw)
    {
    sky1_audss_clk_gate_endisable(hw, 0);
    }
#[no_mangle]
unsafe extern "C" fn sky1_audss_clk_gate_is_enabled(hw: *mut clk_hw) -> c_int {
    static int sky1_audss_clk_gate_is_enabled(struct clk_hw *hw)
    {
    struct clk_gate *gate = to_clk_gate(hw);
    struct sky1_clk_gate *sky1_gate = to_sky1_clk_gate(gate);
    u32 reg;
    regmap_read(sky1_gate.regmap, sky1_gate.offset, &reg);
// if a set bit disables this clk, flip it before masking
    if (gate.flags & CLK_GATE_SET_TO_DISABLE)
    reg ^= BIT(gate.bit_idx);
    reg &= BIT(gate.bit_idx);
    return !!reg;
    }
    static const struct clk_ops sky1_audss_clk_gate_ops = {
    .enable = sky1_audss_clk_gate_enable,
    .disable = sky1_audss_clk_gate_disable,
    .is_enabled = sky1_audss_clk_gate_is_enabled,
    };
    static struct clk_hw *sky1_audss_clk_register(struct device *dev,
    const char *name,
    const char * const *parent_names,
    int num_parents,
    struct regmap *regmap,
    const u32 *mux_table,
    struct muxdiv_cfg *mux_cfg,
    struct muxdiv_cfg *div_cfg,
    struct gate_cfg *gate_cfg,
    unsigned long flags,
    spinlock_t *lock)
    {
    const struct clk_ops *sky1_gate_ops = core::ptr::null_mut();
    const struct clk_ops *sky1_mux_ops = core::ptr::null_mut();
    const struct clk_ops *sky1_div_ops = core::ptr::null_mut();
    struct sky1_clk_divider *sky1_div = core::ptr::null_mut();
    struct sky1_clk_gate *sky1_gate = core::ptr::null_mut();
    struct sky1_clk_mux *sky1_mux = core::ptr::null_mut();
    struct clk_hw *hw = ERR_PTR(-ENOMEM);
    struct clk_parent_data *parent_data;
    int i;
    parent_data = devm_kcalloc(dev, num_parents, sizeof(*parent_data), GFP_KERNEL);
    if (!parent_data)
    return ERR_PTR(-ENOMEM);
    for (i = 0; i < num_parents; i++)
    parent_data[i].name = parent_names[i];
    if (mux_cfg.offset >= 0) {
    sky1_mux = devm_kzalloc(dev, sizeof(*sky1_mux), GFP_KERNEL);
    if (!sky1_mux)
    return ERR_PTR(-ENOMEM);
    sky1_mux.mux.reg = core::ptr::null_mut();
    sky1_mux.mux.shift = mux_cfg.shift;
    sky1_mux.mux.mask = BIT(mux_cfg.width) - 1;
    sky1_mux.mux.flags = mux_cfg.flags;
    sky1_mux.mux.table = mux_table;
    sky1_mux.mux.lock = lock;
    sky1_mux_ops = &sky1_audss_clk_mux_ops;
    sky1_mux.regmap = regmap;
    sky1_mux.offset = mux_cfg.offset;
    }
    if (div_cfg.offset >= 0) {
    sky1_div = devm_kzalloc(dev, sizeof(*sky1_div), GFP_KERNEL);
    if (!sky1_div)
    return ERR_PTR(-ENOMEM);
    sky1_div.div.reg = core::ptr::null_mut();
    sky1_div.div.shift = div_cfg.shift;
    sky1_div.div.width = div_cfg.width;
    sky1_div.div.flags = div_cfg.flags | CLK_DIVIDER_POWER_OF_TWO;
    sky1_div.div.lock = lock;
    sky1_div_ops = &sky1_audss_clk_divider_ops;
    sky1_div.regmap = regmap;
    sky1_div.offset = div_cfg.offset;
    }
    if (gate_cfg.offset >= 0) {
    sky1_gate = devm_kzalloc(dev, sizeof(*sky1_gate), GFP_KERNEL);
    if (!sky1_gate)
    return ERR_PTR(-ENOMEM);
    sky1_gate.gate.reg = core::ptr::null_mut();
    sky1_gate.gate.bit_idx = gate_cfg.shift;
    sky1_gate.gate.flags = gate_cfg.flags;
    sky1_gate.gate.lock = lock;
    sky1_gate_ops = &sky1_audss_clk_gate_ops;
    sky1_gate.regmap = regmap;
    sky1_gate.offset = gate_cfg.offset;
    }
    hw = devm_clk_hw_register_composite_pdata(dev, name, parent_data, num_parents,
    sky1_mux ? &sky1_mux.mux.hw : core::ptr::null_mut(), sky1_mux_ops,
    sky1_div ? &sky1_div.div.hw : core::ptr::null_mut(), sky1_div_ops,
    sky1_gate ? &sky1_gate.gate.hw : core::ptr::null_mut(), sky1_gate_ops,
    flags);
    if (IS_ERR(hw)) {
    dev_err(dev, "register %s clock failed with err = %ld\n",
    name, PTR_ERR(hw));
    return hw;
    }
    return hw;
    }
#[no_mangle]
unsafe extern "C" fn sky1_audss_clks_get(priv: *mut sky1_audss_clks_priv) -> c_int {
    static int sky1_audss_clks_get(struct sky1_audss_clks_priv *priv)
    {
    const struct sky1_audss_clks_devtype_data *devtype_data = priv.devtype_data;
    int i;
    for (i = 0; i < devtype_data.clk_num; i++) {
    priv.clks[i] = devm_clk_get(priv.dev, devtype_data.clk_names[i]);
    if (IS_ERR(priv.clks[i]))
    return dev_err_probe(priv.dev, PTR_ERR(priv.clks[i]),
    "failed to get clock %s", devtype_data.clk_names[i]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sky1_audss_clks_enable(priv: *mut sky1_audss_clks_priv) -> c_int {
    static int sky1_audss_clks_enable(struct sky1_audss_clks_priv *priv)
    {
    const struct sky1_audss_clks_devtype_data *devtype_data = priv.devtype_data;
    int i, err;
    for (i = 0; i < devtype_data.clk_num; i++) {
    err = clk_prepare_enable(priv.clks[i]);
    if (err) {
    dev_err(priv.dev, "failed to enable clock %s\n",
    devtype_data.clk_names[i]);
    goto err_clks;
    }
    }
    return 0;
    err_clks:
    while (--i >= 0)
    clk_disable_unprepare(priv.clks[i]);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sky1_audss_clks_disable(priv: *mut sky1_audss_clks_priv) {
    static void sky1_audss_clks_disable(struct sky1_audss_clks_priv *priv)
    {
    const struct sky1_audss_clks_devtype_data *devtype_data = priv.devtype_data;
    int i;
    for (i = 0; i < devtype_data.clk_num; i++)
    clk_disable_unprepare(priv.clks[i]);
    }
#[no_mangle]
unsafe extern "C" fn sky1_audss_clks_set_rate(priv: *mut sky1_audss_clks_priv) -> c_int {
    static int sky1_audss_clks_set_rate(struct sky1_audss_clks_priv *priv)
    {
    const struct sky1_audss_clks_devtype_data *devtype_data = priv.devtype_data;
    int i, err;
    for (i = 0; i < devtype_data.clk_num; i++) {
    err = clk_set_rate(priv.clks[i], devtype_data.clk_rate_default[i]);
    if (err) {
    dev_err(priv.dev, "failed to set clock rate %s\n",
    devtype_data.clk_names[i]);
    return err;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sky1_audss_clk_rpm_cleanup(data: *mut c_void) {
    static void sky1_audss_clk_rpm_cleanup(void *data)
    {
    pm_runtime_force_suspend(data);
    }
// register sky1 audio subsystem clocks
#[no_mangle]
unsafe extern "C" fn sky1_audss_clk_probe(pdev: *mut platform_device) -> c_int {
    static int sky1_audss_clk_probe(struct platform_device *pdev)
    {
    const struct sky1_audss_clks_devtype_data *devtype_data;
    struct sky1_audss_clks_priv *priv;
    struct device *dev = &pdev.dev;
    struct clk_hw **clk_table;
    void __iomem *base;
    int i, ret;
    devtype_data = device_get_match_data(dev);
    if (!devtype_data)
    return -ENODEV;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    spin_lock_init(&priv.lock);
    priv.clk_data = devm_kzalloc(dev,
    struct_size(priv.clk_data, hws, SKY1_AUDSS_NUM_CLKS),
    GFP_KERNEL);
    if (!priv.clk_data)
    return -ENOMEM;
    priv.clk_data.num = SKY1_AUDSS_NUM_CLKS;
    clk_table = priv.clk_data.hws;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    priv.regmap_cru = devm_regmap_init_mmio(dev, base, &sky1_audss_regmap_config);
    if (IS_ERR(priv.regmap_cru))
    return dev_err_probe(dev, PTR_ERR(priv.regmap_cru),
    "failed to initialize regmap\n");
    priv.dev = dev;
    priv.devtype_data = devtype_data;
    priv.rst_noc = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(priv.rst_noc))
    return dev_err_probe(dev, PTR_ERR(priv.rst_noc),
    "failed to get audss noc reset");
    platform_set_drvdata(pdev, priv);
    reset_control_assert(priv.rst_noc);
    ret = sky1_audss_clks_get(priv);
    if (ret)
    return ret;
    ret = sky1_audss_clks_enable(priv);
    if (ret)
    return ret;
// release noc reset after clock on
    ret = reset_control_deassert(priv.rst_noc);
    if (ret) {
    sky1_audss_clks_disable(priv);
    return ret;
    }
    pm_runtime_get_noresume(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
//
// Register cleanup action before clocks/provider (released last under LIFO).
//
    ret = devm_add_action_or_reset(dev, sky1_audss_clk_rpm_cleanup, dev);
    if (ret)
    return ret;
    ret = sky1_audss_clks_set_rate(priv);
    if (ret)
    goto fail_pm;
// audio_clk4 clock fixed divider
    clk_table[CLK_AUD_CLK4_DIV2] =
    devm_clk_hw_register_fixed_factor(dev,
    "audio_clk4_div2",
    "audio_clk4",
    0,
    1, 2);
    if (IS_ERR(clk_table[CLK_AUD_CLK4_DIV2])) {
    ret = PTR_ERR(clk_table[CLK_AUD_CLK4_DIV2]);
    dev_err(dev, "failed to register clock %d, ret:%d\n", CLK_AUD_CLK4_DIV2, ret);
    goto fail_pm;
    }
    clk_table[CLK_AUD_CLK4_DIV4] =
    devm_clk_hw_register_fixed_factor(dev,
    "audio_clk4_div4",
    "audio_clk4",
    0,
    1, 4);
    if (IS_ERR(clk_table[CLK_AUD_CLK4_DIV4])) {
    ret = PTR_ERR(clk_table[CLK_AUD_CLK4_DIV4]);
    dev_err(dev, "failed to register clock %d, ret:%d\n", CLK_AUD_CLK4_DIV4, ret);
    goto fail_pm;
    }
// audio_clk5 clock fixed divider
    clk_table[CLK_AUD_CLK5_DIV2] =
    devm_clk_hw_register_fixed_factor(dev,
    "audio_clk5_div2",
    "audio_clk5",
    0,
    1, 2);
    if (IS_ERR(clk_table[CLK_AUD_CLK5_DIV2])) {
    ret = PTR_ERR(clk_table[CLK_AUD_CLK5_DIV2]);
    dev_err(dev, "failed to register clock %d, ret:%d\n", CLK_AUD_CLK5_DIV2, ret);
    goto fail_pm;
    }
    for (i = 0; i < devtype_data.clk_cfg_size; i++) {
    clk_table[devtype_data.clk_cfg[i].id] =
    sky1_audss_clk_register(dev,
    devtype_data.clk_cfg[i].name,
    devtype_data.clk_cfg[i].parent_names,
    devtype_data.clk_cfg[i].num_parents,
    priv.regmap_cru,
    devtype_data.clk_cfg[i].mux_table,
    devtype_data.clk_cfg[i].mux_cfg,
    devtype_data.clk_cfg[i].div_cfg,
    devtype_data.clk_cfg[i].gate_cfg,
    devtype_data.clk_cfg[i].flags,
    &priv.lock);
    if (IS_ERR(clk_table[devtype_data.clk_cfg[i].id])) {
    ret = PTR_ERR(clk_table[devtype_data.clk_cfg[i].id]);
    dev_err(dev, "failed to register clock %d, ret:%d\n",
    devtype_data.clk_cfg[i].id, ret);
    goto fail_pm;
    }
    }
    ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get, priv.clk_data);
    if (ret) {
    dev_err(dev, "failed to add clock provider: %d\n", ret);
    goto fail_pm;
    }
    ret = sky1_audss_reset_controller_register(dev);
    if (ret) {
    dev_err(dev, "failed to register reset controller: %d\n", ret);
    goto fail_pm;
    }
    pm_runtime_put_sync(dev);
    return 0;
    fail_pm:
    pm_runtime_put_sync(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sky1_audss_clk_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused sky1_audss_clk_runtime_suspend(struct device *dev)
    {
    struct sky1_audss_clks_priv *priv = dev_get_drvdata(dev);
    const struct sky1_audss_clks_devtype_data *devtype_data = priv.devtype_data;
    unsigned long flags;
    int i;
    spin_lock_irqsave(&priv.lock, flags);
    for (i = 0; i < devtype_data.reg_save_size; i++)
    regmap_read(priv.regmap_cru,
    devtype_data.reg_save[i][0], &devtype_data.reg_save[i][1]);
    spin_unlock_irqrestore(&priv.lock, flags);
    reset_control_assert(priv.rst_noc);
    sky1_audss_clks_disable(priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sky1_audss_clk_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused sky1_audss_clk_runtime_resume(struct device *dev)
    {
    struct sky1_audss_clks_priv *priv = dev_get_drvdata(dev);
    const struct sky1_audss_clks_devtype_data *devtype_data = priv.devtype_data;
    unsigned long flags;
    int i, ret;
    ret = sky1_audss_clks_enable(priv);
    if (ret) {
    dev_err(dev, "failed to enable clocks\n");
    return ret;
    }
    ret = reset_control_deassert(priv.rst_noc);
    if (ret) {
    sky1_audss_clks_disable(priv);
    return ret;
    }
    spin_lock_irqsave(&priv.lock, flags);
    for (i = 0; i < devtype_data.reg_save_size; i++)
    regmap_write(priv.regmap_cru,
    devtype_data.reg_save[i][0], devtype_data.reg_save[i][1]);
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
    static const struct dev_pm_ops sky1_audss_clk_pm_ops = {
    SET_RUNTIME_PM_OPS(sky1_audss_clk_runtime_suspend,
    sky1_audss_clk_runtime_resume, core::ptr::null_mut())
    SET_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    };
    static const struct sky1_audss_clks_devtype_data sky1_devtype_data = {
    .reg_save = sky1_reg_save,
    .reg_save_size = ARRAY_SIZE(sky1_reg_save),
    .clk_names = sky1_audss_clk_names,
    .clk_num = ARRAY_SIZE(sky1_audss_clk_names),
    .clk_rate_default = sky1_clk_rate_default,
    .clk_cfg = sky1_audss_clks,
    .clk_cfg_size = ARRAY_SIZE(sky1_audss_clks),
    };
    static const struct of_device_id sky1_audss_clk_of_match[] = {
    { .compatible = "cix,sky1-audss-cru", .data = &sky1_devtype_data, },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, sky1_audss_clk_of_match);
    static struct platform_driver sky1_audss_clk_driver = {
    .probe = sky1_audss_clk_probe,
    .driver = {
    .name = "sky1-audss-clk",
    .suppress_bind_attrs = true,
    .of_match_table = sky1_audss_clk_of_match,
    .pm = pm_ptr(&sky1_audss_clk_pm_ops),
    },
    };
    module_platform_driver(sky1_audss_clk_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Joakim Zhang <joakim.zhang@cixtech.com>");
    MODULE_DESCRIPTION("Cixtech Sky1 Audio Subsystem Clock Controller Driver");

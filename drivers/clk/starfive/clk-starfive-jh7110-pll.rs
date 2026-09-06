//! Automatically rewritten from C to Rust
//! Source: drivers/clk/starfive/clk-starfive-jh7110-pll.c
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
// StarFive JH7110 PLL Clock Generator Driver
//
// Copyright (C) 2023 StarFive Technology Co., Ltd.
// Copyright (C) 2023 Emil Renner Berthing <emil.renner.berthing@canonical.com>
//
// This driver is about to register JH7110 PLL clock generator and support ops.
// The JH7110 have three PLL clock, PLL0, PLL1 and PLL2.
// Each PLL clocks work in integer mode or fraction mode by some dividers,
// and the configuration registers and dividers are set in several syscon registers.
// The formula for calculating frequency is:
// Fvco = Fref * (NI + NF) / M / Q1
// Fref: OSC source clock rate
// NI: integer frequency dividing ratio of feedback divider, set by fbdiv[11:0].
// NF: fractional frequency dividing ratio, set by frac[23:0]. NF = frac[23:0] / 2^24 = 0 ~ 0.999.
// M: frequency dividing ratio of pre-divider, set by prediv[5:0].
// Q1: frequency dividing ratio of post divider, set by 2^postdiv1[1:0], eg. 1, 2, 4 or 8.
//

// this driver expects a 24MHz input frequency from the oscillator

pub const JH7110_PLL0_PD_OFFSET: c_uint = 0x18;
pub const JH7110_PLL0_DACPD_SHIFT: c_int = 24;

pub const JH7110_PLL0_DSMPD_SHIFT: c_int = 25;

pub const JH7110_PLL0_FBDIV_OFFSET: c_uint = 0x1c;
pub const JH7110_PLL0_FBDIV_SHIFT: c_int = 0;

pub const JH7110_PLL0_FRAC_OFFSET: c_uint = 0x20;
pub const JH7110_PLL0_PREDIV_OFFSET: c_uint = 0x24;
pub const JH7110_PLL1_PD_OFFSET: c_uint = 0x24;
pub const JH7110_PLL1_DACPD_SHIFT: c_int = 15;

pub const JH7110_PLL1_DSMPD_SHIFT: c_int = 16;

pub const JH7110_PLL1_FBDIV_OFFSET: c_uint = 0x24;
pub const JH7110_PLL1_FBDIV_SHIFT: c_int = 17;

pub const JH7110_PLL1_FRAC_OFFSET: c_uint = 0x28;
pub const JH7110_PLL1_PREDIV_OFFSET: c_uint = 0x2c;
pub const JH7110_PLL2_PD_OFFSET: c_uint = 0x2c;
pub const JH7110_PLL2_DACPD_SHIFT: c_int = 15;

pub const JH7110_PLL2_DSMPD_SHIFT: c_int = 16;

pub const JH7110_PLL2_FBDIV_OFFSET: c_uint = 0x2c;
pub const JH7110_PLL2_FBDIV_SHIFT: c_int = 17;

pub const JH7110_PLL2_FRAC_OFFSET: c_uint = 0x30;
pub const JH7110_PLL2_PREDIV_OFFSET: c_uint = 0x34;
pub const JH7110_PLL_FRAC_SHIFT: c_int = 0;

pub const JH7110_PLL_POSTDIV1_SHIFT: c_int = 28;

pub const JH7110_PLL_PREDIV_SHIFT: c_int = 0;

    enum jh7110_pll_mode {
    JH7110_PLL_MODE_FRACTION,
    JH7110_PLL_MODE_INTEGER,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh7110_pll_preset {
    pub freq: c_ulong,
    pub /: *mut *mut u32 frac; / frac value should be decimals multiplied by 2^24,
    pub /: *mut *mut unsigned fbdiv : 12; / fbdiv value should be 8 to 4095,
    pub 6: unsigned prediv :,
    pub 2: unsigned postdiv1 :,
    pub 1: unsigned mode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh7110_pll_info {
    pub name: *mut c_char,
    pub presets: *const jh7110_pll_preset,
    pub npresets: c_uint,
    struct {
    pub pd: c_uint,
    pub fbdiv: c_uint,
    pub frac: c_uint,
    pub prediv: c_uint,
    pub offsets: },
    struct {
    pub dacpd: u32,
    pub dsmpd: u32,
    pub fbdiv: u32,
    pub masks: },
    struct {
    pub dacpd: c_char,
    pub dsmpd: c_char,
    pub fbdiv: c_char,
    pub shifts: },
}

    [_idx] = {							\
    .name = _name,						\
    .presets = _presets,					\
    .npresets = ARRAY_SIZE(_presets),			\
    .offsets = {						\
    .pd = JH7110_PLL##_idx##_PD_OFFSET,		\
    .fbdiv = JH7110_PLL##_idx##_FBDIV_OFFSET,	\
    .frac = JH7110_PLL##_idx##_FRAC_OFFSET,		\
    .prediv = JH7110_PLL##_idx##_PREDIV_OFFSET,	\
    },							\
    .masks = {						\
    .dacpd = JH7110_PLL##_idx##_DACPD_MASK,		\
    .dsmpd = JH7110_PLL##_idx##_DSMPD_MASK,		\
    .fbdiv = JH7110_PLL##_idx##_FBDIV_MASK,		\
    },							\
    .shifts = {						\
    .dacpd = JH7110_PLL##_idx##_DACPD_SHIFT,	\
    .dsmpd = JH7110_PLL##_idx##_DSMPD_SHIFT,	\
    .fbdiv = JH7110_PLL##_idx##_FBDIV_SHIFT,	\
    },							\
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh7110_pll_data {
    pub hw: clk_hw,
    pub idx: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh7110_pll_priv {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub pll: [jh7110_pll_data; JH7110_PLLCLK_END],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh7110_pll_regvals {
    pub dacpd: u32,
    pub dsmpd: u32,
    pub fbdiv: u32,
    pub frac: u32,
    pub postdiv1: u32,
    pub prediv: u32,
}

//
// Because the pll frequency is relatively fixed,
// it cannot be set arbitrarily, so it needs a specific configuration.
// PLL0 frequency should be multiple of 125MHz (USB frequency).
//
    static const struct jh7110_pll_preset jh7110_pll0_presets[] = {
    {
    .freq = 375000000,
    .fbdiv = 125,
    .prediv = 8,
    .postdiv1 = 0,
    .mode = JH7110_PLL_MODE_INTEGER,
    }, {
    .freq = 500000000,
    .fbdiv = 125,
    .prediv = 6,
    .postdiv1 = 0,
    .mode = JH7110_PLL_MODE_INTEGER,
    }, {
    .freq = 625000000,
    .fbdiv = 625,
    .prediv = 24,
    .postdiv1 = 0,
    .mode = JH7110_PLL_MODE_INTEGER,
    }, {
    .freq = 750000000,
    .fbdiv = 125,
    .prediv = 4,
    .postdiv1 = 0,
    .mode = JH7110_PLL_MODE_INTEGER,
    }, {
    .freq = 875000000,
    .fbdiv = 875,
    .prediv = 24,
    .postdiv1 = 0,
    .mode = JH7110_PLL_MODE_INTEGER,
    }, {
    .freq = 1000000000,
    .fbdiv = 125,
    .prediv = 3,
    .postdiv1 = 0,
    .mode = JH7110_PLL_MODE_INTEGER,
    }, {
    .freq = 1250000000,
    .fbdiv = 625,
    .prediv = 12,
    .postdiv1 = 0,
    .mode = JH7110_PLL_MODE_INTEGER,
    }, {
    .freq = 1375000000,
    .fbdiv = 1375,
    .prediv = 24,
    .postdiv1 = 0,
    .mode = JH7110_PLL_MODE_INTEGER,
    }, {
    .freq = 1500000000,
    .fbdiv = 125,
    .prediv = 2,
    .postdiv1 = 0,
    .mode = JH7110_PLL_MODE_INTEGER,
    },
    };
    static const struct jh7110_pll_preset jh7110_pll1_presets[] = {
    {
    .freq = 1066000000,
    .fbdiv = 533,
    .prediv = 12,
    .postdiv1 = 0,
    .mode = JH7110_PLL_MODE_INTEGER,
    }, {
    .freq = 1200000000,
    .fbdiv = 50,
    .prediv = 1,
    .postdiv1 = 0,
    .mode = JH7110_PLL_MODE_INTEGER,
    }, {
    .freq = 1400000000,
    .fbdiv = 350,
    .prediv = 6,
    .postdiv1 = 0,
    .mode = JH7110_PLL_MODE_INTEGER,
    }, {
    .freq = 1600000000,
    .fbdiv = 200,
    .prediv = 3,
    .postdiv1 = 0,
    .mode = JH7110_PLL_MODE_INTEGER,
    },
    };
    static const struct jh7110_pll_preset jh7110_pll2_presets[] = {
    {
    .freq = 1188000000,
    .fbdiv = 99,
    .prediv = 2,
    .postdiv1 = 0,
    .mode = JH7110_PLL_MODE_INTEGER,
    }, {
    .freq = 1228800000,
    .fbdiv = 256,
    .prediv = 5,
    .postdiv1 = 0,
    .mode = JH7110_PLL_MODE_INTEGER,
    },
    };
    static const struct jh7110_pll_info jh7110_plls[JH7110_PLLCLK_END] = {
    JH7110_PLL(JH7110_PLLCLK_PLL0_OUT, "pll0_out", jh7110_pll0_presets),
    JH7110_PLL(JH7110_PLLCLK_PLL1_OUT, "pll1_out", jh7110_pll1_presets),
    JH7110_PLL(JH7110_PLLCLK_PLL2_OUT, "pll2_out", jh7110_pll2_presets),
    };
    static struct jh7110_pll_data *jh7110_pll_data_from(struct clk_hw *hw)
    {
    return container_of(hw, struct jh7110_pll_data, hw);
    }
    static struct jh7110_pll_priv *jh7110_pll_priv_from(struct jh7110_pll_data *pll)
    {
    return container_of(pll, struct jh7110_pll_priv, pll[pll.idx]);
    }
    static void jh7110_pll_regvals_get(struct regmap *regmap,
    const struct jh7110_pll_info *info,
    struct jh7110_pll_regvals *ret)
    {
    u32 val;
    regmap_read(regmap, info.offsets.pd, &val);
    ret.dacpd = (val & info.masks.dacpd) >> info.shifts.dacpd;
    ret.dsmpd = (val & info.masks.dsmpd) >> info.shifts.dsmpd;
    regmap_read(regmap, info.offsets.fbdiv, &val);
    ret.fbdiv = (val & info.masks.fbdiv) >> info.shifts.fbdiv;
    regmap_read(regmap, info.offsets.frac, &val);
    ret.frac = (val & JH7110_PLL_FRAC_MASK) >> JH7110_PLL_FRAC_SHIFT;
    ret.postdiv1 = (val & JH7110_PLL_POSTDIV1_MASK) >> JH7110_PLL_POSTDIV1_SHIFT;
    regmap_read(regmap, info.offsets.prediv, &val);
    ret.prediv = (val & JH7110_PLL_PREDIV_MASK) >> JH7110_PLL_PREDIV_SHIFT;
    }
#[no_mangle]
unsafe extern "C" fn jh7110_pll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    static unsigned long jh7110_pll_recalc_rate(struct clk_hw *hw, unsigned long parent_rate)
    {
    struct jh7110_pll_data *pll = jh7110_pll_data_from(hw);
    struct jh7110_pll_priv *priv = jh7110_pll_priv_from(pll);
    struct jh7110_pll_regvals val;
    unsigned long rate;
    jh7110_pll_regvals_get(priv.regmap, &jh7110_plls[pll.idx], &val);
//
// dacpd = dsmpd = 0: fraction mode
// dacpd = dsmpd = 1: integer mode, frac value ignored
//
// rate = parent * (fbdiv + frac/2^24) / prediv / 2^postdiv1
// = (parent * fbdiv + parent * frac / 2^24) / (prediv * 2^postdiv1)
//
    if (val.dacpd == 0 && val.dsmpd == 0)
    rate = parent_rate * val.frac / (1UL << 24);
#[no_mangle]
pub unsafe extern "C" fn if(1: val.dacpd == 1 && val.dsmpd ==) -> else {
    else if (val.dacpd == 1 && val.dsmpd == 1)
    rate = 0;
    else
    return 0;
    rate += parent_rate * val.fbdiv;
    rate /= val.prediv << val.postdiv1;
    return rate;
    }
#[no_mangle]
unsafe extern "C" fn jh7110_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    static int jh7110_pll_determine_rate(struct clk_hw *hw, struct clk_rate_request *req)
    {
    struct jh7110_pll_data *pll = jh7110_pll_data_from(hw);
    const struct jh7110_pll_info *info = &jh7110_plls[pll.idx];
    const struct jh7110_pll_preset *selected = &info.presets[0];
    unsigned int idx;
// if the parent rate doesn't match our expectations the presets won't work
    if (req.best_parent_rate != JH7110_PLL_OSC_RATE) {
    req.rate = jh7110_pll_recalc_rate(hw, req.best_parent_rate);
    return 0;
    }
// find highest rate lower or equal to the requested rate
    for (idx = 1; idx < info.npresets; idx++) {
    const struct jh7110_pll_preset *val = &info.presets[idx];
    if (req.rate < val.freq)
    break;
    selected = val;
    }
    req.rate = selected.freq;
    return 0;
    }
    static int jh7110_pll_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct jh7110_pll_data *pll = jh7110_pll_data_from(hw);
    struct jh7110_pll_priv *priv = jh7110_pll_priv_from(pll);
    const struct jh7110_pll_info *info = &jh7110_plls[pll.idx];
    const struct jh7110_pll_preset *val;
    unsigned int idx;
// if the parent rate doesn't match our expectations the presets won't work
    if (parent_rate != JH7110_PLL_OSC_RATE)
    return -EINVAL;
    for (idx = 0, val = &info.presets[0]; idx < info.npresets; idx++, val++) {
    if (val.freq == rate)
    goto found;
    }
    return -EINVAL;
    found:
    if (val.mode == JH7110_PLL_MODE_FRACTION)
    regmap_update_bits(priv.regmap, info.offsets.frac, JH7110_PLL_FRAC_MASK,
    val.frac << JH7110_PLL_FRAC_SHIFT);
    regmap_update_bits(priv.regmap, info.offsets.pd, info.masks.dacpd,
    (u32)val.mode << info.shifts.dacpd);
    regmap_update_bits(priv.regmap, info.offsets.pd, info.masks.dsmpd,
    (u32)val.mode << info.shifts.dsmpd);
    regmap_update_bits(priv.regmap, info.offsets.prediv, JH7110_PLL_PREDIV_MASK,
    (u32)val.prediv << JH7110_PLL_PREDIV_SHIFT);
    regmap_update_bits(priv.regmap, info.offsets.fbdiv, info.masks.fbdiv,
    val.fbdiv << info.shifts.fbdiv);
    regmap_update_bits(priv.regmap, info.offsets.frac, JH7110_PLL_POSTDIV1_MASK,
    (u32)val.postdiv1 << JH7110_PLL_POSTDIV1_SHIFT);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn jh7110_pll_registers_read(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int jh7110_pll_registers_read(struct seq_file *s, void *unused)
    {
    struct jh7110_pll_data *pll = s.private;
    struct jh7110_pll_priv *priv = jh7110_pll_priv_from(pll);
    struct jh7110_pll_regvals val;
    jh7110_pll_regvals_get(priv.regmap, &jh7110_plls[pll.idx], &val);
    seq_printf(s, "fbdiv=%u\n"
    "frac=%u\n"
    "prediv=%u\n"
    "postdiv1=%u\n"
    "dacpd=%u\n"
    "dsmpd=%u\n",
    val.fbdiv, val.frac, val.prediv, val.postdiv1,
    val.dacpd, val.dsmpd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jh7110_pll_registers_open(inode: *mut inode, f: *mut file) -> c_int {
    static int jh7110_pll_registers_open(struct inode *inode, struct file *f)
    {
    return single_open(f, jh7110_pll_registers_read, inode.i_private);
    }
    static const struct file_operations jh7110_pll_registers_ops = {
    .owner = THIS_MODULE,
    .open = jh7110_pll_registers_open,
    .release = single_release,
    .read = seq_read,
    .llseek = seq_lseek
    };
#[no_mangle]
unsafe extern "C" fn jh7110_pll_debug_init(hw: *mut clk_hw, dentry: *mut dentry) {
    static void jh7110_pll_debug_init(struct clk_hw *hw, struct dentry *dentry)
    {
    struct jh7110_pll_data *pll = jh7110_pll_data_from(hw);
    debugfs_create_file("registers", 0400, dentry, pll,
    &jh7110_pll_registers_ops);
    }

    static const struct clk_ops jh7110_pll_ops = {
    .recalc_rate = jh7110_pll_recalc_rate,
    .determine_rate = jh7110_pll_determine_rate,
    .set_rate = jh7110_pll_set_rate,
    .debug_init = jh7110_pll_debug_init,
    };
    static struct clk_hw *jh7110_pll_get(struct of_phandle_args *clkspec, void *data)
    {
    struct jh7110_pll_priv *priv = data;
    let mut idx: c_uint = clkspec.args[0];
    if (idx < JH7110_PLLCLK_END)
    return &priv.pll[idx].hw;
    return ERR_PTR(-EINVAL);
    }
#[no_mangle]
unsafe extern "C" fn jh7110_pll_probe(pdev: *mut platform_device) -> int __init {
    static int __init jh7110_pll_probe(struct platform_device *pdev)
    {
    struct jh7110_pll_priv *priv;
    unsigned int idx;
    int ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = &pdev.dev;
    priv.regmap = syscon_node_to_regmap(priv.dev.of_node.parent);
    if (IS_ERR(priv.regmap))
    return PTR_ERR(priv.regmap);
    for (idx = 0; idx < JH7110_PLLCLK_END; idx++) {
    struct clk_parent_data parents = {
    .index = 0,
    };
    struct clk_init_data init = {
    .name = jh7110_plls[idx].name,
    .ops = &jh7110_pll_ops,
    .parent_data = &parents,
    .num_parents = 1,
    .flags = 0,
    };
    struct jh7110_pll_data *pll = &priv.pll[idx];
    pll.hw.init = &init;
    pll.idx = idx;
    ret = devm_clk_hw_register(&pdev.dev, &pll.hw);
    if (ret)
    return ret;
    }
    return devm_of_clk_add_hw_provider(&pdev.dev, jh7110_pll_get, priv);
    }
    static const struct of_device_id jh7110_pll_match[] = {
    { .compatible = "starfive,jh7110-pll" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, jh7110_pll_match);
    static struct platform_driver jh7110_pll_driver = {
    .driver = {
    .name = "clk-starfive-jh7110-pll",
    .of_match_table = jh7110_pll_match,
    },
    };
    builtin_platform_driver_probe(jh7110_pll_driver, jh7110_pll_probe);

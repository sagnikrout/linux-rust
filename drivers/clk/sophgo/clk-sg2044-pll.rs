//! Automatically rewritten from C to Rust
//! Source: drivers/clk/sophgo/clk-sg2044-pll.c
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
// Sophgo SG2044 PLL clock controller driver
//
// Copyright (C) 2025 Inochi Amaoto <inochiama@gmail.com>
//

// Low Control part

// High Control part

    (PLL_FBDIV_MASK | PLL_REFDIV_MASK | \
    PLL_POSTDIV1_MASK | PLL_POSTDIV2_MASK | \
    PLL_CALIBRATE_EN | PLL_CALIBRATE_MASK | \
    PLL_UPDATE_EN)
pub const PLL_HIGH_CTRL_OFFSET: c_int = 4;
pub const PLL_VCOSEL_1G6: c_uint = 0x2;
pub const PLL_VCOSEL_2G4: c_uint = 0x3;
pub const PLL_LIMIT_FOUTVCO: c_int = 0;
pub const PLL_LIMIT_FOUT: c_int = 1;
pub const PLL_LIMIT_REFDIV: c_int = 2;
pub const PLL_LIMIT_FBDIV: c_int = 3;
pub const PLL_LIMIT_POSTDIV1: c_int = 4;
pub const PLL_LIMIT_POSTDIV2: c_int = 5;

    for (_var = (_limit).min; _var <= (_limit).max; _var++)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg2044_pll_limit {
    pub min: u64,
    pub max: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg2044_pll_internal {
    pub ctrl_offset: u32,
    pub status_offset: u32,
    pub enable_offset: u32,
    pub status_lock_bit: u8,
    pub status_updating_bit: u8,
    pub enable_bit: u8,
    pub limits: *const sg2044_pll_limit,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg2044_clk_common {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub lock: *mut spinlock_t,
    pub id: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg2044_pll {
    pub common: sg2044_clk_common,
    pub pll: sg2044_pll_internal,
    pub syscon_offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg2044_pll_desc_data {
    pub pll: *const *const sg2044_clk_common,
    pub num_pll: u16,
}

pub const SG2044_SYSCON_PLL_OFFSET: c_uint = 0x98;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg2044_pll_ctrl {
    pub lock: spinlock_t,
    pub data: clk_hw_onecell_data,
}

    container_of((_hw), struct sg2044_clk_common, hw)
    static inline bool sg2044_clk_fit_limit(u64 value,
    const struct sg2044_pll_limit *limit)
    {
    return value >= limit.min && value <= limit.max;
    }
    static inline struct sg2044_pll *hw_to_sg2044_pll(struct clk_hw *hw)
    {
    return container_of(hw_to_sg2044_clk_common(hw),
    struct sg2044_pll, common);
    }
    static unsigned long sg2044_pll_calc_vco_rate(unsigned long parent_rate,
    unsigned long refdiv,
    unsigned long fbdiv)
    {
    let mut numerator: u64 = parent_rate * fbdiv;
    return div64_ul(numerator, refdiv);
    }
    static unsigned long sg2044_pll_calc_rate(unsigned long parent_rate,
    unsigned long refdiv,
    unsigned long fbdiv,
    unsigned long postdiv1,
    unsigned long postdiv2)
    {
    u64 numerator, denominator;
    numerator = parent_rate * fbdiv;
    denominator = refdiv * (postdiv1 + 1) * (postdiv2 + 1);
    return div64_u64(numerator, denominator);
    }
    static unsigned long sg2044_pll_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct sg2044_pll *pll = hw_to_sg2044_pll(hw);
    u32 value;
    int ret;
    ret = regmap_read(pll.common.regmap,
    pll.syscon_offset + pll.pll.ctrl_offset + PLL_HIGH_CTRL_OFFSET,
    &value);
    if (ret < 0)
    return 0;
    return sg2044_pll_calc_rate(parent_rate,
    FIELD_GET(PLL_REFDIV_MASK, value),
    FIELD_GET(PLL_FBDIV_MASK, value),
    FIELD_GET(PLL_POSTDIV1_MASK, value),
    FIELD_GET(PLL_POSTDIV2_MASK, value));
    }
    static bool pll_is_better_rate(unsigned long target, unsigned long now,
    unsigned long best)
    {
    return abs_diff(target, now) < abs_diff(target, best);
    }
    static int sg2042_pll_compute_postdiv(const struct sg2044_pll_limit *limits,
    unsigned long target,
    unsigned long parent_rate,
    unsigned int refdiv,
    unsigned int fbdiv,
    unsigned int *postdiv1,
    unsigned int *postdiv2)
    {
    unsigned int div1, div2;
    unsigned long tmp, best_rate = 0;
    let mut best_div1: c_uint = 0, best_div2 = 0;
    for_each_pll_limit_range(div2, &limits[PLL_LIMIT_POSTDIV2]) {
    for_each_pll_limit_range(div1, &limits[PLL_LIMIT_POSTDIV1]) {
    tmp = sg2044_pll_calc_rate(parent_rate,
    refdiv, fbdiv,
    div1, div2);
    if (tmp > target)
    continue;
    if (pll_is_better_rate(target, tmp, best_rate)) {
    best_div1 = div1;
    best_div2 = div2;
    best_rate = tmp;
    if (tmp == target)
    goto find;
    }
    }
    }
    find:
    if (best_rate) {
// postdiv1 = best_div1;
// postdiv2 = best_div2;
    return 0;
    }
    return -EINVAL;
    }
    static int sg2044_compute_pll_setting(const struct sg2044_pll_limit *limits,
    unsigned long req_rate,
    unsigned long parent_rate,
    unsigned int *value)
    {
    unsigned int refdiv, fbdiv, postdiv1, postdiv2;
    unsigned int best_refdiv, best_fbdiv, best_postdiv1, best_postdiv2;
    unsigned long tmp, best_rate = 0;
    int ret;
    for_each_pll_limit_range(fbdiv, &limits[PLL_LIMIT_FBDIV]) {
    for_each_pll_limit_range(refdiv, &limits[PLL_LIMIT_REFDIV]) {
    u64 vco = sg2044_pll_calc_vco_rate(parent_rate,
    refdiv, fbdiv);
    if (!sg2044_clk_fit_limit(vco, &limits[PLL_LIMIT_FOUTVCO]))
    continue;
    ret = sg2042_pll_compute_postdiv(limits,
    req_rate, parent_rate,
    refdiv, fbdiv,
    &postdiv1, &postdiv2);
    if (ret)
    continue;
    tmp = sg2044_pll_calc_rate(parent_rate,
    refdiv, fbdiv,
    postdiv1, postdiv2);
    if (pll_is_better_rate(req_rate, tmp, best_rate)) {
    best_refdiv = refdiv;
    best_fbdiv = fbdiv;
    best_postdiv1 = postdiv1;
    best_postdiv2 = postdiv2;
    best_rate = tmp;
    if (tmp == req_rate)
    goto find;
    }
    }
    }
    find:
    if (best_rate) {
// value = FIELD_PREP(PLL_REFDIV_MASK, best_refdiv) |
    FIELD_PREP(PLL_FBDIV_MASK, best_fbdiv) |
    FIELD_PREP(PLL_POSTDIV1_MASK, best_postdiv1) |
    FIELD_PREP(PLL_POSTDIV2_MASK, best_postdiv2);
    return 0;
    }
    return -EINVAL;
    }
    static int sg2044_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct sg2044_pll *pll = hw_to_sg2044_pll(hw);
    unsigned int value;
    u64 target;
    int ret;
    target = clamp(req.rate, pll.pll.limits[PLL_LIMIT_FOUT].min,
    pll.pll.limits[PLL_LIMIT_FOUT].max);
    ret = sg2044_compute_pll_setting(pll.pll.limits, target,
    req.best_parent_rate, &value);
    if (ret < 0)
    return ret;
    req.rate = sg2044_pll_calc_rate(req.best_parent_rate,
    FIELD_GET(PLL_REFDIV_MASK, value),
    FIELD_GET(PLL_FBDIV_MASK, value),
    FIELD_GET(PLL_POSTDIV1_MASK, value),
    FIELD_GET(PLL_POSTDIV2_MASK, value));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sg2044_pll_poll_update(pll: *mut sg2044_pll) -> c_int {
    static int sg2044_pll_poll_update(struct sg2044_pll *pll)
    {
    int ret;
    unsigned int value;
    ret = regmap_read_poll_timeout_atomic(pll.common.regmap,
    pll.syscon_offset + pll.pll.status_offset,
    value,
    (value & BIT(pll.pll.status_lock_bit)),
    1, 100000);
    if (ret)
    return ret;
    return regmap_read_poll_timeout_atomic(pll.common.regmap,
    pll.syscon_offset + pll.pll.status_offset,
    value,
    (!(value & BIT(pll.pll.status_updating_bit))),
    1, 100000);
    }
#[no_mangle]
unsafe extern "C" fn sg2044_pll_enable(pll: *mut sg2044_pll, en: bool) -> c_int {
    static int sg2044_pll_enable(struct sg2044_pll *pll, bool en)
    {
    if (en) {
    if (sg2044_pll_poll_update(pll) < 0)
    pr_warn("%s: fail to lock pll\n", clk_hw_get_name(&pll.common.hw));
    return regmap_set_bits(pll.common.regmap,
    pll.syscon_offset + pll.pll.enable_offset,
    BIT(pll.pll.enable_bit));
    }
    return regmap_clear_bits(pll.common.regmap,
    pll.syscon_offset + pll.pll.enable_offset,
    BIT(pll.pll.enable_bit));
    }
#[no_mangle]
unsafe extern "C" fn sg2044_pll_update_vcosel(pll: *mut sg2044_pll, rate: u64) -> c_int {
    static int sg2044_pll_update_vcosel(struct sg2044_pll *pll, u64 rate)
    {
    unsigned int sel;
    if (rate < U64_C(2400000000))
    sel = PLL_VCOSEL_1G6;
    else
    sel = PLL_VCOSEL_2G4;
    return regmap_write_bits(pll.common.regmap,
    pll.syscon_offset + pll.pll.ctrl_offset,
    PLL_VCOSEL_MASK,
    FIELD_PREP(PLL_VCOSEL_MASK, sel));
    }
    static int sg2044_pll_set_rate(struct clk_hw *hw,
    unsigned long rate, unsigned long parent_rate)
    {
    struct sg2044_pll *pll = hw_to_sg2044_pll(hw);
    unsigned int value;
    u64 vco;
    int ret;
    ret = sg2044_compute_pll_setting(pll.pll.limits, rate,
    parent_rate, &value);
    if (ret < 0)
    return ret;
    vco = sg2044_pll_calc_vco_rate(parent_rate,
    FIELD_GET(PLL_REFDIV_MASK, value),
    FIELD_GET(PLL_FBDIV_MASK, value));
    value |= PLL_CALIBRATE_EN;
    value |= PLL_CALIBRATE_DEFAULT;
    value |= PLL_UPDATE_EN;
    guard(spinlock_irqsave)(pll.common.lock);
    ret = sg2044_pll_enable(pll, false);
    if (ret)
    return ret;
    sg2044_pll_update_vcosel(pll, vco);
    regmap_write_bits(pll.common.regmap,
    pll.syscon_offset + pll.pll.ctrl_offset +
    PLL_HIGH_CTRL_OFFSET,
    PLL_HIGH_CTRL_MASK, value);
    sg2044_pll_enable(pll, true);
    return ret;
    }
    static const struct clk_ops sg2044_pll_ops = {
    .recalc_rate = sg2044_pll_recalc_rate,
    .determine_rate = sg2044_pll_determine_rate,
    .set_rate = sg2044_pll_set_rate,
    };
    static const struct clk_ops sg2044_pll_ro_ops = {
    .recalc_rate = sg2044_pll_recalc_rate,
    };

    {								\
    .hw.init = CLK_HW_INIT_PARENTS_DATA(_name, _parents,	\
    _op, (_flags)),	\
    .id = (_id),						\
    }

    _ctrl_offset,					\
    _status_offset, _status_lock_bit,		\
    _status_updating_bit,				\
    _enable_offset, _enable_bit,			\
    _limits)					\
    struct sg2044_pll _name = {					\
    .common	= SG2044_CLK_COMMON_PDATA(_id, #_name, _parent,	\
    &sg2044_pll_ops,	\
    (_flags)),		\
    .pll = {						\
    .ctrl_offset = (_ctrl_offset),			\
    .status_offset = (_status_offset),		\
    .enable_offset = (_enable_offset),		\
    .status_lock_bit = (_status_lock_bit),		\
    .status_updating_bit = (_status_updating_bit),	\
    .enable_bit = (_enable_bit),			\
    .limits = (_limits),				\
    },							\
    }

    _ctrl_offset,				\
    _status_offset, _status_lock_bit,		\
    _status_updating_bit,			\
    _enable_offset, _enable_bit,		\
    _limits)					\
    struct sg2044_pll _name = {					\
    .common	= SG2044_CLK_COMMON_PDATA(_id, #_name, _parent,	\
    &sg2044_pll_ro_ops,	\
    (_flags)),		\
    .pll = {						\
    .ctrl_offset = (_ctrl_offset),			\
    .status_offset = (_status_offset),		\
    .enable_offset = (_enable_offset),		\
    .status_lock_bit = (_status_lock_bit),		\
    .status_updating_bit = (_status_updating_bit),	\
    .enable_bit = (_enable_bit),			\
    .limits = (_limits),				\
    },							\
    }
    static const struct clk_parent_data osc_parents[] = {
    { .index = 0 },
    };
    static const struct sg2044_pll_limit pll_limits[] = {
    [PLL_LIMIT_FOUTVCO] = {
    .min = U64_C(1600000000),
    .max = U64_C(3200000000),
    },
    [PLL_LIMIT_FOUT] = {
    .min = U64_C(25000),
    .max = U64_C(3200000000),
    },
    [PLL_LIMIT_REFDIV] = {
    .min = U64_C(1),
    .max = U64_C(63),
    },
    [PLL_LIMIT_FBDIV] = {
    .min = U64_C(8),
    .max = U64_C(1066),
    },
    [PLL_LIMIT_POSTDIV1] = {
    .min = U64_C(0),
    .max = U64_C(7),
    },
    [PLL_LIMIT_POSTDIV2] = {
    .min = U64_C(0),
    .max = U64_C(7),
    },
    };
    static DEFINE_SG2044_PLL_RO(CLK_FPLL0, clk_fpll0, osc_parents, CLK_IS_CRITICAL,
    0x58, 0x00, 22, 6,
    0x04, 6, pll_limits);
    static DEFINE_SG2044_PLL_RO(CLK_FPLL1, clk_fpll1, osc_parents, CLK_IS_CRITICAL,
    0x60, 0x00, 23, 7,
    0x04, 7, pll_limits);
    static DEFINE_SG2044_PLL_RO(CLK_FPLL2, clk_fpll2, osc_parents, CLK_IS_CRITICAL,
    0x20, 0x08, 16, 0,
    0x0c, 0, pll_limits);
    static DEFINE_SG2044_PLL_RO(CLK_DPLL0, clk_dpll0, osc_parents, CLK_IS_CRITICAL,
    0x68, 0x00, 24, 8,
    0x04, 8, pll_limits);
    static DEFINE_SG2044_PLL_RO(CLK_DPLL1, clk_dpll1, osc_parents, CLK_IS_CRITICAL,
    0x70, 0x00, 25, 9,
    0x04, 9, pll_limits);
    static DEFINE_SG2044_PLL_RO(CLK_DPLL2, clk_dpll2, osc_parents, CLK_IS_CRITICAL,
    0x78, 0x00, 26, 10,
    0x04, 10, pll_limits);
    static DEFINE_SG2044_PLL_RO(CLK_DPLL3, clk_dpll3, osc_parents, CLK_IS_CRITICAL,
    0x80, 0x00, 27, 11,
    0x04, 11, pll_limits);
    static DEFINE_SG2044_PLL_RO(CLK_DPLL4, clk_dpll4, osc_parents, CLK_IS_CRITICAL,
    0x88, 0x00, 28, 12,
    0x04, 12, pll_limits);
    static DEFINE_SG2044_PLL_RO(CLK_DPLL5, clk_dpll5, osc_parents, CLK_IS_CRITICAL,
    0x90, 0x00, 29, 13,
    0x04, 13, pll_limits);
    static DEFINE_SG2044_PLL_RO(CLK_DPLL6, clk_dpll6, osc_parents, CLK_IS_CRITICAL,
    0x98, 0x00, 30, 14,
    0x04, 14, pll_limits);
    static DEFINE_SG2044_PLL_RO(CLK_DPLL7, clk_dpll7, osc_parents, CLK_IS_CRITICAL,
    0xa0, 0x00, 31, 15,
    0x04, 15, pll_limits);
    static DEFINE_SG2044_PLL(CLK_MPLL0, clk_mpll0, osc_parents, CLK_IS_CRITICAL,
    0x28, 0x00, 16, 0,
    0x04, 0, pll_limits);
    static DEFINE_SG2044_PLL(CLK_MPLL1, clk_mpll1, osc_parents, CLK_IS_CRITICAL,
    0x30, 0x00, 17, 1,
    0x04, 1, pll_limits);
    static DEFINE_SG2044_PLL(CLK_MPLL2, clk_mpll2, osc_parents, CLK_IS_CRITICAL,
    0x38, 0x00, 18, 2,
    0x04, 2, pll_limits);
    static DEFINE_SG2044_PLL(CLK_MPLL3, clk_mpll3, osc_parents, CLK_IS_CRITICAL,
    0x40, 0x00, 19, 3,
    0x04, 3, pll_limits);
    static DEFINE_SG2044_PLL(CLK_MPLL4, clk_mpll4, osc_parents, CLK_IS_CRITICAL,
    0x48, 0x00, 20, 4,
    0x04, 4, pll_limits);
    static DEFINE_SG2044_PLL(CLK_MPLL5, clk_mpll5, osc_parents, CLK_IS_CRITICAL,
    0x50, 0x00, 21, 5,
    0x04, 5, pll_limits);
    static struct sg2044_clk_common * const sg2044_pll_commons[] = {
    &clk_fpll0.common,
    &clk_fpll1.common,
    &clk_fpll2.common,
    &clk_dpll0.common,
    &clk_dpll1.common,
    &clk_dpll2.common,
    &clk_dpll3.common,
    &clk_dpll4.common,
    &clk_dpll5.common,
    &clk_dpll6.common,
    &clk_dpll7.common,
    &clk_mpll0.common,
    &clk_mpll1.common,
    &clk_mpll2.common,
    &clk_mpll3.common,
    &clk_mpll4.common,
    &clk_mpll5.common,
    };
    static int sg2044_pll_init_ctrl(struct device *dev, struct regmap *regmap,
    struct sg2044_pll_ctrl *ctrl,
    const struct sg2044_pll_desc_data *desc)
    {
    int ret, i;
    spin_lock_init(&ctrl.lock);
    for (i = 0; i < desc.num_pll; i++) {
    struct sg2044_clk_common *common = desc.pll[i];
    struct sg2044_pll *pll = hw_to_sg2044_pll(&common.hw);
    common.lock = &ctrl.lock;
    common.regmap = regmap;
    pll.syscon_offset = SG2044_SYSCON_PLL_OFFSET;
    ret = devm_clk_hw_register(dev, &common.hw);
    if (ret)
    return ret;
    ctrl.data.hws[common.id] = &common.hw;
    }
    return devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get,
    &ctrl.data);
    }
#[no_mangle]
unsafe extern "C" fn sg2044_pll_probe(pdev: *mut platform_device) -> c_int {
    static int sg2044_pll_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct sg2044_pll_ctrl *ctrl;
    const struct sg2044_pll_desc_data *desc;
    struct regmap *regmap;
    regmap = device_node_to_regmap(pdev.dev.parent.of_node);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap),
    "fail to get the regmap for PLL\n");
    desc = (const struct sg2044_pll_desc_data *)platform_get_device_id(pdev).driver_data;
    if (!desc)
    return dev_err_probe(dev, -EINVAL, "no match data for platform\n");
    ctrl = devm_kzalloc(dev, struct_size(ctrl, data.hws, desc.num_pll), GFP_KERNEL);
    if (!ctrl)
    return -ENOMEM;
    ctrl.data.num = desc.num_pll;
    return sg2044_pll_init_ctrl(dev, regmap, ctrl, desc);
    }
    static const struct sg2044_pll_desc_data sg2044_pll_desc_data = {
    .pll = sg2044_pll_commons,
    .num_pll = ARRAY_SIZE(sg2044_pll_commons),
    };
    static const struct platform_device_id sg2044_pll_match[] = {
    { .name = "sg2044-pll",
    .driver_data = (unsigned long)&sg2044_pll_desc_data },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, sg2044_pll_match);
    static struct platform_driver sg2044_clk_driver = {
    .probe = sg2044_pll_probe,
    .driver = {
    .name = "sg2044-pll",
    },
    .id_table = sg2044_pll_match,
    };
    module_platform_driver(sg2044_clk_driver);
    MODULE_AUTHOR("Inochi Amaoto <inochiama@gmail.com>");
    MODULE_DESCRIPTION("Sophgo SG2044 pll clock driver");
    MODULE_LICENSE("GPL");

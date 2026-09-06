//! Automatically rewritten from C to Rust
//! Source: drivers/clk/microchip/clk-mpfs.c
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
// PolarFire SoC MSS/core complex clock control
//
// Copyright (C) 2020-2022 Microchip Technology Inc. All rights reserved.
//

// address offset of control registers
pub const REG_MSSPLL_REF_CR: c_uint = 0x08u;
pub const REG_MSSPLL_POSTDIV01_CR: c_uint = 0x10u;
pub const REG_MSSPLL_POSTDIV23_CR: c_uint = 0x14u;
pub const REG_MSSPLL_SSCG_2_CR: c_uint = 0x2Cu;
pub const REG_CLOCK_CONFIG_CR: c_uint = 0x08u;
pub const REG_RTC_CLOCK_CR: c_uint = 0x0Cu;
pub const REG_SUBBLK_CLOCK_CR: c_uint = 0x84u;
pub const REG_SUBBLK_RESET_CR: c_uint = 0x88u;
pub const MSSPLL_FBDIV_SHIFT: c_uint = 0x00u;
pub const MSSPLL_FBDIV_WIDTH: c_uint = 0x0Cu;
pub const MSSPLL_REFDIV_SHIFT: c_uint = 0x08u;
pub const MSSPLL_REFDIV_WIDTH: c_uint = 0x06u;
pub const MSSPLL_POSTDIV02_SHIFT: c_uint = 0x08u;
pub const MSSPLL_POSTDIV13_SHIFT: c_uint = 0x18u;
pub const MSSPLL_POSTDIV_WIDTH: c_uint = 0x07u;

    static const struct regmap_config mpfs_clk_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .val_format_endian = REGMAP_ENDIAN_LITTLE,
    .max_register = REG_SUBBLK_RESET_CR,
    };
//
// This clock ID is defined here, rather than the binding headers, as it is an
// internal clock only, and therefore has no consumers in other peripheral
// blocks.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpfs_clock_data {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub base: *mut void __iomem,
    pub msspll_base: *mut void __iomem,
    pub hw_data: clk_hw_onecell_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpfs_msspll_hw_clock {
    pub base: *mut void __iomem,
    pub hw: clk_hw,
    pub init: clk_init_data,
    pub id: c_uint,
    pub reg_offset: u32,
    pub shift: u32,
    pub width: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpfs_msspll_out_hw_clock {
    pub base: *mut void __iomem,
    pub output: clk_divider,
    pub init: clk_init_data,
    pub id: c_uint,
    pub reg_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpfs_cfg_clock {
    pub map: *mut regmap,
    pub table: *const clk_div_table,
    pub map_offset: u8,
    pub shift: u8,
    pub width: u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpfs_cfg_hw_clock {
    pub hw: clk_hw,
    pub cfg: mpfs_cfg_clock,
    pub id: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpfs_periph_clock {
    pub map: *mut regmap,
    pub map_offset: u8,
    pub shift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpfs_periph_hw_clock {
    pub hw: clk_hw,
    pub periph: mpfs_periph_clock,
    pub id: c_uint,
}

//
// Protects MSSPLL outputs, since there's two to a register
//
    static DEFINE_SPINLOCK(mpfs_clk_lock);
    static const struct clk_parent_data mpfs_ext_ref[] = {
    { .index = 0 },
    };
    static const struct clk_div_table mpfs_div_cpu_axi_table[] = {
    { 0, 1 }, { 1, 2 }, { 2, 4 }, { 3, 8 },
    { 0, 0 }
    };
    static const struct clk_div_table mpfs_div_ahb_table[] = {
    { 1, 2 }, { 2, 4}, { 3, 8 },
    { 0, 0 }
    };
//
// The only two supported reference clock frequencies for the PolarFire SoC are
// 100 and 125 MHz, as the rtc reference is required to be 1 MHz.
// It therefore only needs to have divider table entries corresponding to
// divide by 100 and 125.
//
    static const struct clk_div_table mpfs_div_rtcref_table[] = {
    { 100, 100 }, { 125, 125 },
    { 0, 0 }
    };
//
// MSS PLL internal clock
//
#[no_mangle]
unsafe extern "C" fn mpfs_clk_msspll_recalc_rate(hw: *mut clk_hw, prate: c_ulong) -> c_ulong {
    static unsigned long mpfs_clk_msspll_recalc_rate(struct clk_hw *hw, unsigned long prate)
    {
    struct mpfs_msspll_hw_clock *msspll_hw = to_mpfs_msspll_clk(hw);
    void __iomem *mult_addr = msspll_hw.base + msspll_hw.reg_offset;
    void __iomem *ref_div_addr = msspll_hw.base + REG_MSSPLL_REF_CR;
    u32 mult, ref_div;
    mult = readl_relaxed(mult_addr) >> MSSPLL_FBDIV_SHIFT;
    mult &= clk_div_mask(MSSPLL_FBDIV_WIDTH);
    ref_div = readl_relaxed(ref_div_addr) >> MSSPLL_REFDIV_SHIFT;
    ref_div &= clk_div_mask(MSSPLL_REFDIV_WIDTH);
    return prate * mult / (ref_div * MSSPLL_FIXED_DIV);
    }
    static const struct clk_ops mpfs_clk_msspll_ops = {
    .recalc_rate = mpfs_clk_msspll_recalc_rate,
    };

    .id = _id,									\
    .flags = _flags,								\
    .shift = _shift,								\
    .width = _width,								\
    .reg_offset = _offset,								\
    .hw.init = CLK_HW_INIT_PARENTS_DATA(_name, _parent, &mpfs_clk_msspll_ops, 0),	\
    }
    static struct mpfs_msspll_hw_clock mpfs_msspll_clks[] = {
    CLK_PLL(CLK_MSSPLL_INTERNAL, "clk_msspll_internal", mpfs_ext_ref, MSSPLL_FBDIV_SHIFT,
    MSSPLL_FBDIV_WIDTH, 0, REG_MSSPLL_SSCG_2_CR),
    };
    static int mpfs_clk_register_mssplls(struct device *dev, struct mpfs_msspll_hw_clock *msspll_hws,
    unsigned int num_clks, struct mpfs_clock_data *data)
    {
    unsigned int i;
    int ret;
    for (i = 0; i < num_clks; i++) {
    struct mpfs_msspll_hw_clock *msspll_hw = &msspll_hws[i];
    msspll_hw.base = data.msspll_base;
    ret = devm_clk_hw_register(dev, &msspll_hw.hw);
    if (ret)
    return dev_err_probe(dev, ret, "failed to register msspll id: %d\n",
    CLK_MSSPLL_INTERNAL);
    data.hw_data.hws[msspll_hw.id] = &msspll_hw.hw;
    }
    return 0;
    }
//
// MSS PLL output clocks
//

    .id = _id,								\
    .output.shift = _shift,							\
    .output.width = _width,							\
    .output.table = core::ptr::null_mut(),							\
    .reg_offset = _offset,							\
    .output.flags = _flags,							\
    .output.hw.init = CLK_HW_INIT(_name, _parent, &clk_divider_ops, 0),	\
    .output.lock = &mpfs_clk_lock,						\
    }
    static struct mpfs_msspll_out_hw_clock mpfs_msspll_out_clks[] = {
    CLK_PLL_OUT(CLK_MSSPLL0, "clk_msspll", "clk_msspll_internal", CLK_DIVIDER_ONE_BASED,
    MSSPLL_POSTDIV02_SHIFT, MSSPLL_POSTDIV_WIDTH, REG_MSSPLL_POSTDIV01_CR),
    CLK_PLL_OUT(CLK_MSSPLL1, "clk_msspll1", "clk_msspll_internal", CLK_DIVIDER_ONE_BASED,
    MSSPLL_POSTDIV13_SHIFT, MSSPLL_POSTDIV_WIDTH, REG_MSSPLL_POSTDIV01_CR),
    CLK_PLL_OUT(CLK_MSSPLL2, "clk_msspll2", "clk_msspll_internal", CLK_DIVIDER_ONE_BASED,
    MSSPLL_POSTDIV02_SHIFT, MSSPLL_POSTDIV_WIDTH, REG_MSSPLL_POSTDIV23_CR),
    CLK_PLL_OUT(CLK_MSSPLL3, "clk_msspll3", "clk_msspll_internal", CLK_DIVIDER_ONE_BASED,
    MSSPLL_POSTDIV13_SHIFT, MSSPLL_POSTDIV_WIDTH, REG_MSSPLL_POSTDIV23_CR),
    };
    static int mpfs_clk_register_msspll_outs(struct device *dev,
    struct mpfs_msspll_out_hw_clock *msspll_out_hws,
    unsigned int num_clks, struct mpfs_clock_data *data)
    {
    unsigned int i;
    int ret;
    for (i = 0; i < num_clks; i++) {
    struct mpfs_msspll_out_hw_clock *msspll_out_hw = &msspll_out_hws[i];
    msspll_out_hw.output.reg = data.msspll_base + msspll_out_hw.reg_offset;
    ret = devm_clk_hw_register(dev, &msspll_out_hw.output.hw);
    if (ret)
    return dev_err_probe(dev, ret, "failed to register msspll out id: %d\n",
    msspll_out_hw.id);
    data.hw_data.hws[msspll_out_hw.id] = &msspll_out_hw.output.hw;
    }
    return 0;
    }
//
// "CFG" clocks
//
#[no_mangle]
unsafe extern "C" fn mpfs_cfg_clk_recalc_rate(hw: *mut clk_hw, prate: c_ulong) -> c_ulong {
    static unsigned long mpfs_cfg_clk_recalc_rate(struct clk_hw *hw, unsigned long prate)
    {
    struct mpfs_cfg_hw_clock *cfg_hw = to_mpfs_cfg_clk(hw);
    struct mpfs_cfg_clock *cfg = &cfg_hw.cfg;
    u32 val;
    regmap_read(cfg.map, cfg.map_offset, &val);
    val >>= cfg.shift;
    val &= clk_div_mask(cfg.width);
    return divider_recalc_rate(hw, prate, val, cfg.table, cfg.flags, cfg.width);
    }
#[no_mangle]
unsafe extern "C" fn mpfs_cfg_clk_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    static int mpfs_cfg_clk_determine_rate(struct clk_hw *hw, struct clk_rate_request *req)
    {
    struct mpfs_cfg_hw_clock *cfg_hw = to_mpfs_cfg_clk(hw);
    struct mpfs_cfg_clock *cfg = &cfg_hw.cfg;
    return divider_determine_rate(hw, req, cfg.table, cfg.width, 0);
    }
#[no_mangle]
unsafe extern "C" fn mpfs_cfg_clk_set_rate(hw: *mut clk_hw, rate: c_ulong, prate: c_ulong) -> c_int {
    static int mpfs_cfg_clk_set_rate(struct clk_hw *hw, unsigned long rate, unsigned long prate)
    {
    struct mpfs_cfg_hw_clock *cfg_hw = to_mpfs_cfg_clk(hw);
    struct mpfs_cfg_clock *cfg = &cfg_hw.cfg;
    int divider_setting;
    u32 val;
    u32 mask;
    divider_setting = divider_get_val(rate, prate, cfg.table, cfg.width, 0);
    if (divider_setting < 0)
    return divider_setting;
    mask = clk_div_mask(cfg.width) << cfg.shift;
    val = divider_setting << cfg.shift;
    regmap_update_bits(cfg.map, cfg.map_offset, mask, val);
    return 0;
    }
    static const struct clk_ops mpfs_clk_cfg_ops = {
    .recalc_rate = mpfs_cfg_clk_recalc_rate,
    .determine_rate = mpfs_cfg_clk_determine_rate,
    .set_rate = mpfs_cfg_clk_set_rate,
    };

    .id = _id,								\
    .cfg.shift = _shift,							\
    .cfg.width = _width,							\
    .cfg.table = _table,							\
    .cfg.map_offset = _offset,						\
    .cfg.flags = _flags,							\
    .hw.init = CLK_HW_INIT(_name, _parent, &mpfs_clk_cfg_ops, 0),		\
    }

    static struct mpfs_cfg_hw_clock mpfs_cfg_clks[] = {
    CLK_CFG(CLK_CPU, "clk_cpu", "clk_msspll", 0, 2, mpfs_div_cpu_axi_table, 0,
    REG_CLOCK_CONFIG_CR),
    CLK_CFG(CLK_AXI, "clk_axi", "clk_msspll", 2, 2, mpfs_div_cpu_axi_table, 0,
    REG_CLOCK_CONFIG_CR),
    CLK_CFG(CLK_AHB, "clk_ahb", "clk_msspll", 4, 2, mpfs_div_ahb_table, 0,
    REG_CLOCK_CONFIG_CR),
    {
    .id = CLK_RTCREF,
    .cfg.shift = 0,
    .cfg.width = 12,
    .cfg.table = mpfs_div_rtcref_table,
    .cfg.map_offset = REG_RTC_CLOCK_CR,
    .cfg.flags = CLK_DIVIDER_ONE_BASED,
    .hw.init =
    CLK_HW_INIT_PARENTS_DATA("clk_rtcref", mpfs_ext_ref, &mpfs_clk_cfg_ops, 0),
    }
    };
    static int mpfs_clk_register_cfgs(struct device *dev, struct mpfs_cfg_hw_clock *cfg_hws,
    unsigned int num_clks, struct mpfs_clock_data *data)
    {
    unsigned int i, id;
    int ret;
    for (i = 0; i < num_clks; i++) {
    struct mpfs_cfg_hw_clock *cfg_hw = &cfg_hws[i];
    cfg_hw.cfg.map = data.regmap;
    ret = devm_clk_hw_register(dev, &cfg_hw.hw);
    if (ret)
    return dev_err_probe(dev, ret, "failed to register clock id: %d\n",
    cfg_hw.id);
    id = cfg_hw.id;
    data.hw_data.hws[id] = &cfg_hw.hw;
    }
    return 0;
    }
//
// peripheral clocks - devices connected to axi or ahb buses.
//
#[no_mangle]
unsafe extern "C" fn mpfs_periph_clk_enable(hw: *mut clk_hw) -> c_int {
    static int mpfs_periph_clk_enable(struct clk_hw *hw)
    {
    struct mpfs_periph_hw_clock *periph_hw = to_mpfs_periph_clk(hw);
    struct mpfs_periph_clock *periph = &periph_hw.periph;
    regmap_update_bits(periph.map, periph.map_offset,
    BIT(periph.shift), BIT(periph.shift));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpfs_periph_clk_disable(hw: *mut clk_hw) {
    static void mpfs_periph_clk_disable(struct clk_hw *hw)
    {
    struct mpfs_periph_hw_clock *periph_hw = to_mpfs_periph_clk(hw);
    struct mpfs_periph_clock *periph = &periph_hw.periph;
    regmap_update_bits(periph.map, periph.map_offset, BIT(periph.shift), 0);
    }
#[no_mangle]
unsafe extern "C" fn mpfs_periph_clk_is_enabled(hw: *mut clk_hw) -> c_int {
    static int mpfs_periph_clk_is_enabled(struct clk_hw *hw)
    {
    struct mpfs_periph_hw_clock *periph_hw = to_mpfs_periph_clk(hw);
    struct mpfs_periph_clock *periph = &periph_hw.periph;
    u32 val;
    regmap_read(periph.map, periph.map_offset, &val);
    return !!(val & BIT(periph.shift));
    }
    static const struct clk_ops mpfs_periph_clk_ops = {
    .enable = mpfs_periph_clk_enable,
    .disable = mpfs_periph_clk_disable,
    .is_enabled = mpfs_periph_clk_is_enabled,
    };

    .id = _id,									\
    .periph.map_offset = REG_SUBBLK_CLOCK_CR,					\
    .periph.shift = _shift,								\
    .hw.init = CLK_HW_INIT_HW(_name, _parent, &mpfs_periph_clk_ops, _flags),	\
    }

//
// Critical clocks:
// - CLK_ENVM: reserved by hart software services (hss) superloop monitor/m mode interrupt
// trap handler
// - CLK_MMUART0: reserved by the hss
// - CLK_DDRC: provides clock to the ddr subsystem
// - CLK_RTC: the onboard RTC's AHB bus clock must be kept running as the rtc will stop
// if the AHB interface clock is disabled
// - CLK_FICx: these provide the processor side clocks to the "FIC" (Fabric InterConnect)
// clock domain crossers which provide the interface to the FPGA fabric. Disabling them
// causes the FPGA fabric to go into reset.
// - CLK_ATHENA: The athena clock is FIC4, which is reserved for the Athena TeraFire.
//
    static struct mpfs_periph_hw_clock mpfs_periph_clks[] = {
    CLK_PERIPH(CLK_ENVM, "clk_periph_envm", PARENT_CLK(AHB), 0, CLK_IS_CRITICAL),
    CLK_PERIPH(CLK_MAC0, "clk_periph_mac0", PARENT_CLK(AHB), 1, 0),
    CLK_PERIPH(CLK_MAC1, "clk_periph_mac1", PARENT_CLK(AHB), 2, 0),
    CLK_PERIPH(CLK_MMC, "clk_periph_mmc", PARENT_CLK(AHB), 3, 0),
    CLK_PERIPH(CLK_TIMER, "clk_periph_timer", PARENT_CLK(RTCREF), 4, 0),
    CLK_PERIPH(CLK_MMUART0, "clk_periph_mmuart0", PARENT_CLK(AHB), 5, CLK_IS_CRITICAL),
    CLK_PERIPH(CLK_MMUART1, "clk_periph_mmuart1", PARENT_CLK(AHB), 6, 0),
    CLK_PERIPH(CLK_MMUART2, "clk_periph_mmuart2", PARENT_CLK(AHB), 7, 0),
    CLK_PERIPH(CLK_MMUART3, "clk_periph_mmuart3", PARENT_CLK(AHB), 8, 0),
    CLK_PERIPH(CLK_MMUART4, "clk_periph_mmuart4", PARENT_CLK(AHB), 9, 0),
    CLK_PERIPH(CLK_SPI0, "clk_periph_spi0", PARENT_CLK(AHB), 10, 0),
    CLK_PERIPH(CLK_SPI1, "clk_periph_spi1", PARENT_CLK(AHB), 11, 0),
    CLK_PERIPH(CLK_I2C0, "clk_periph_i2c0", PARENT_CLK(AHB), 12, 0),
    CLK_PERIPH(CLK_I2C1, "clk_periph_i2c1", PARENT_CLK(AHB), 13, 0),
    CLK_PERIPH(CLK_CAN0, "clk_periph_can0", PARENT_CLK(AHB), 14, 0),
    CLK_PERIPH(CLK_CAN1, "clk_periph_can1", PARENT_CLK(AHB), 15, 0),
    CLK_PERIPH(CLK_USB, "clk_periph_usb", PARENT_CLK(AHB), 16, 0),
    CLK_PERIPH(CLK_RTC, "clk_periph_rtc", PARENT_CLK(AHB), 18, CLK_IS_CRITICAL),
    CLK_PERIPH(CLK_QSPI, "clk_periph_qspi", PARENT_CLK(AHB), 19, 0),
    CLK_PERIPH(CLK_GPIO0, "clk_periph_gpio0", PARENT_CLK(AHB), 20, 0),
    CLK_PERIPH(CLK_GPIO1, "clk_periph_gpio1", PARENT_CLK(AHB), 21, 0),
    CLK_PERIPH(CLK_GPIO2, "clk_periph_gpio2", PARENT_CLK(AHB), 22, 0),
    CLK_PERIPH(CLK_DDRC, "clk_periph_ddrc", PARENT_CLK(AHB), 23, CLK_IS_CRITICAL),
    CLK_PERIPH(CLK_FIC0, "clk_periph_fic0", PARENT_CLK(AXI), 24, CLK_IS_CRITICAL),
    CLK_PERIPH(CLK_FIC1, "clk_periph_fic1", PARENT_CLK(AXI), 25, CLK_IS_CRITICAL),
    CLK_PERIPH(CLK_FIC2, "clk_periph_fic2", PARENT_CLK(AXI), 26, CLK_IS_CRITICAL),
    CLK_PERIPH(CLK_FIC3, "clk_periph_fic3", PARENT_CLK(AXI), 27, CLK_IS_CRITICAL),
    CLK_PERIPH(CLK_ATHENA, "clk_periph_athena", PARENT_CLK(AXI), 28, CLK_IS_CRITICAL),
    CLK_PERIPH(CLK_CFM, "clk_periph_cfm", PARENT_CLK(AHB), 29, 0),
    };
    static int mpfs_clk_register_periphs(struct device *dev, struct mpfs_periph_hw_clock *periph_hws,
    int num_clks, struct mpfs_clock_data *data)
    {
    unsigned int i, id;
    int ret;
    for (i = 0; i < num_clks; i++) {
    struct mpfs_periph_hw_clock *periph_hw = &periph_hws[i];
    periph_hw.periph.map = data.regmap;
    ret = devm_clk_hw_register(dev, &periph_hw.hw);
    if (ret)
    return dev_err_probe(dev, ret, "failed to register clock id: %d\n",
    periph_hw.id);
    id = periph_hws[i].id;
    data.hw_data.hws[id] = &periph_hw.hw;
    }
    return 0;
    }
    static inline int mpfs_clk_syscon_probe(struct mpfs_clock_data *clk_data,
    struct platform_device *pdev)
    {
    clk_data.regmap = syscon_regmap_lookup_by_compatible("microchip,mpfs-mss-top-sysreg");
    if (IS_ERR(clk_data.regmap))
    return PTR_ERR(clk_data.regmap);
    clk_data.msspll_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(clk_data.msspll_base))
    return PTR_ERR(clk_data.msspll_base);
    return 0;
    }
    static inline int mpfs_clk_old_format_probe(struct mpfs_clock_data *clk_data,
    struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    dev_warn(&pdev.dev, "falling back to old devicetree format");
    clk_data.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(clk_data.base))
    return PTR_ERR(clk_data.base);
    clk_data.msspll_base = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(clk_data.msspll_base))
    return PTR_ERR(clk_data.msspll_base);
    clk_data.regmap = devm_regmap_init_mmio(dev, clk_data.base, &mpfs_clk_regmap_config);
    if (IS_ERR(clk_data.regmap))
    return PTR_ERR(clk_data.regmap);
    return mpfs_reset_controller_register(dev, clk_data.regmap);
    }
#[no_mangle]
unsafe extern "C" fn mpfs_clk_probe(pdev: *mut platform_device) -> c_int {
    static int mpfs_clk_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mpfs_clock_data *clk_data;
    unsigned int num_clks;
    int ret;
// CLK_RESERVED is not part of clock arrays, so add 1
    num_clks = ARRAY_SIZE(mpfs_msspll_clks) + ARRAY_SIZE(mpfs_msspll_out_clks)
    + ARRAY_SIZE(mpfs_cfg_clks)  + ARRAY_SIZE(mpfs_periph_clks) + 1;
    clk_data = devm_kzalloc(dev, struct_size(clk_data, hw_data.hws, num_clks), GFP_KERNEL);
    if (!clk_data)
    return -ENOMEM;
    ret = mpfs_clk_syscon_probe(clk_data, pdev);
    if (ret) {
    ret = mpfs_clk_old_format_probe(clk_data, pdev);
    if (ret)
    return ret;
    }
    clk_data.hw_data.num = num_clks;
    clk_data.dev = dev;
    dev_set_drvdata(dev, clk_data);
    ret = mpfs_clk_register_mssplls(dev, mpfs_msspll_clks, ARRAY_SIZE(mpfs_msspll_clks),
    clk_data);
    if (ret)
    return ret;
    ret = mpfs_clk_register_msspll_outs(dev, mpfs_msspll_out_clks,
    ARRAY_SIZE(mpfs_msspll_out_clks),
    clk_data);
    if (ret)
    return ret;
    ret = mpfs_clk_register_cfgs(dev, mpfs_cfg_clks, ARRAY_SIZE(mpfs_cfg_clks), clk_data);
    if (ret)
    return ret;
    ret = mpfs_clk_register_periphs(dev, mpfs_periph_clks, ARRAY_SIZE(mpfs_periph_clks),
    clk_data);
    if (ret)
    return ret;
    return devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get, &clk_data.hw_data);
    }
    static const struct of_device_id mpfs_clk_of_match_table[] = {
    { .compatible = "microchip,mpfs-clkcfg", },
    {}
    };
    MODULE_DEVICE_TABLE(of, mpfs_clk_of_match_table);
    static struct platform_driver mpfs_clk_driver = {
    .probe = mpfs_clk_probe,
    .driver	= {
    .name = "microchip-mpfs-clkcfg",
    .of_match_table = mpfs_clk_of_match_table,
    },
    };
#[no_mangle]
unsafe extern "C" fn clk_mpfs_init() -> int __init {
    static int __init clk_mpfs_init(void)
    {
    return platform_driver_register(&mpfs_clk_driver);
    }
    core_initcall(clk_mpfs_init);
#[no_mangle]
unsafe extern "C" fn clk_mpfs_exit() -> void __exit {
    static void __exit clk_mpfs_exit(void)
    {
    platform_driver_unregister(&mpfs_clk_driver);
    }
    module_exit(clk_mpfs_exit);
    MODULE_DESCRIPTION("Microchip PolarFire SoC Clock Driver");
    MODULE_AUTHOR("Padmarao Begari <padmarao.begari@microchip.com>");
    MODULE_AUTHOR("Daire McNamara <daire.mcnamara@microchip.com>");
    MODULE_AUTHOR("Conor Dooley <conor.dooley@microchip.com>");
    MODULE_IMPORT_NS("MCHP_CLK_MPFS");

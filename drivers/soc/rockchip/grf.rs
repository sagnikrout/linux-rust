//! Automatically rewritten from C to Rust
//! Source: drivers/soc/rockchip/grf.c
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
// Rockchip Generic Register Files setup
//
// Copyright (c) 2016 Heiko Stuebner <heiko@sntech.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_grf_value {
    pub desc: *const c_char,
    pub reg: u32,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_grf_info {
    pub values: *const rockchip_grf_value,
    pub num_values: c_int,
}

pub const RK3036_GRF_SOC_CON0: c_uint = 0x140;
    static const struct rockchip_grf_value rk3036_defaults[] __initconst = {
//
// Disable auto jtag/sdmmc switching that causes issues with the
// clock-framework and the mmc controllers making them unreliable.
//
    { "jtag switching", RK3036_GRF_SOC_CON0, FIELD_PREP_WM16_CONST(BIT(11), 0) },
    };
    static const struct rockchip_grf_info rk3036_grf __initconst = {
    .values = rk3036_defaults,
    .num_values = ARRAY_SIZE(rk3036_defaults),
    };
pub const RK3128_GRF_SOC_CON0: c_uint = 0x140;
pub const RK3128_GRF_SOC_CON1: c_uint = 0x144;
    static const struct rockchip_grf_value rk3128_defaults[] __initconst = {
    { "jtag switching", RK3128_GRF_SOC_CON0, FIELD_PREP_WM16_CONST(BIT(8), 0) },
    { "vpu main clock", RK3128_GRF_SOC_CON1, FIELD_PREP_WM16_CONST(BIT(10), 0) },
    };
    static const struct rockchip_grf_info rk3128_grf __initconst = {
    .values = rk3128_defaults,
    .num_values = ARRAY_SIZE(rk3128_defaults),
    };
pub const RK3228_GRF_SOC_CON6: c_uint = 0x418;
    static const struct rockchip_grf_value rk3228_defaults[] __initconst = {
    { "jtag switching", RK3228_GRF_SOC_CON6, FIELD_PREP_WM16_CONST(BIT(8), 0) },
    };
    static const struct rockchip_grf_info rk3228_grf __initconst = {
    .values = rk3228_defaults,
    .num_values = ARRAY_SIZE(rk3228_defaults),
    };
pub const RK3288_GRF_SOC_CON0: c_uint = 0x244;
pub const RK3288_GRF_SOC_CON2: c_uint = 0x24c;
    static const struct rockchip_grf_value rk3288_defaults[] __initconst = {
    { "jtag switching", RK3288_GRF_SOC_CON0, FIELD_PREP_WM16_CONST(BIT(12), 0) },
    { "pwm select", RK3288_GRF_SOC_CON2, FIELD_PREP_WM16_CONST(BIT(0), 1) },
    };
    static const struct rockchip_grf_info rk3288_grf __initconst = {
    .values = rk3288_defaults,
    .num_values = ARRAY_SIZE(rk3288_defaults),
    };
pub const RK3328_GRF_SOC_CON4: c_uint = 0x410;
    static const struct rockchip_grf_value rk3328_defaults[] __initconst = {
    { "jtag switching", RK3328_GRF_SOC_CON4, FIELD_PREP_WM16_CONST(BIT(12), 0) },
    };
    static const struct rockchip_grf_info rk3328_grf __initconst = {
    .values = rk3328_defaults,
    .num_values = ARRAY_SIZE(rk3328_defaults),
    };
pub const RK3368_GRF_SOC_CON15: c_uint = 0x43c;
    static const struct rockchip_grf_value rk3368_defaults[] __initconst = {
    { "jtag switching", RK3368_GRF_SOC_CON15, FIELD_PREP_WM16_CONST(BIT(13), 0) },
    { "pwm select", RK3368_GRF_SOC_CON15, FIELD_PREP_WM16_CONST(BIT(12), 1) },
    };
    static const struct rockchip_grf_info rk3368_grf __initconst = {
    .values = rk3368_defaults,
    .num_values = ARRAY_SIZE(rk3368_defaults),
    };
pub const RK3368_PMUGRF_SOC_CON0: c_uint = 0x100;
    static const struct rockchip_grf_value rk3368_pmugrf_defaults[] __initconst = {
    { "pwm2 select", RK3368_PMUGRF_SOC_CON0, FIELD_PREP_WM16_CONST(BIT(7), 0) },
    };
    static const struct rockchip_grf_info rk3368_pmugrf __initconst = {
    .values = rk3368_pmugrf_defaults,
    .num_values = ARRAY_SIZE(rk3368_pmugrf_defaults),
    };
pub const RK3399_GRF_SOC_CON7: c_uint = 0xe21c;
    static const struct rockchip_grf_value rk3399_defaults[] __initconst = {
    { "jtag switching", RK3399_GRF_SOC_CON7, FIELD_PREP_WM16_CONST(BIT(12), 0) },
    };
    static const struct rockchip_grf_info rk3399_grf __initconst = {
    .values = rk3399_defaults,
    .num_values = ARRAY_SIZE(rk3399_defaults),
    };
pub const RK3566_GRF_USB3OTG0_CON1: c_uint = 0x0104;
    static const struct rockchip_grf_value rk3566_defaults[] __initconst = {
    { "usb3otg port switch", RK3566_GRF_USB3OTG0_CON1, FIELD_PREP_WM16_CONST(BIT(12), 0) },
    { "usb3otg clock switch", RK3566_GRF_USB3OTG0_CON1, FIELD_PREP_WM16_CONST(BIT(7), 1) },
    { "usb3otg disable usb3", RK3566_GRF_USB3OTG0_CON1, FIELD_PREP_WM16_CONST(BIT(0), 1) },
    };
    static const struct rockchip_grf_info rk3566_pipegrf __initconst = {
    .values = rk3566_defaults,
    .num_values = ARRAY_SIZE(rk3566_defaults),
    };
pub const RK3576_SYSGRF_SOC_CON1: c_uint = 0x0004;
    static const struct rockchip_grf_value rk3576_defaults_sys_grf[] __initconst = {
    { "i3c0 weakpull", RK3576_SYSGRF_SOC_CON1, FIELD_PREP_WM16_CONST(GENMASK(7, 6), 3) },
    { "i3c1 weakpull", RK3576_SYSGRF_SOC_CON1, FIELD_PREP_WM16_CONST(GENMASK(9, 8), 3) },
    };
    static const struct rockchip_grf_info rk3576_sysgrf __initconst = {
    .values = rk3576_defaults_sys_grf,
    .num_values = ARRAY_SIZE(rk3576_defaults_sys_grf),
    };
pub const RK3576_IOCGRF_MISC_CON: c_uint = 0x40F0;
    static const struct rockchip_grf_value rk3576_defaults_ioc_grf[] __initconst = {
    { "jtag switching", RK3576_IOCGRF_MISC_CON, FIELD_PREP_WM16_CONST(BIT(1), 0) },
    };
    static const struct rockchip_grf_info rk3576_iocgrf __initconst = {
    .values = rk3576_defaults_ioc_grf,
    .num_values = ARRAY_SIZE(rk3576_defaults_ioc_grf),
    };
pub const RK3588_GRF_SOC_CON6: c_uint = 0x0318;
    static const struct rockchip_grf_value rk3588_defaults[] __initconst = {
    { "jtag switching", RK3588_GRF_SOC_CON6, FIELD_PREP_WM16_CONST(BIT(14), 0) },
    };
    static const struct rockchip_grf_info rk3588_sysgrf __initconst = {
    .values = rk3588_defaults,
    .num_values = ARRAY_SIZE(rk3588_defaults),
    };
    static const struct of_device_id rockchip_grf_dt_match[] __initconst = {
    {
    .compatible = "rockchip,rk3036-grf",
    .data = (void *)&rk3036_grf,
    }, {
    .compatible = "rockchip,rk3128-grf",
    .data = (void *)&rk3128_grf,
    }, {
    .compatible = "rockchip,rk3228-grf",
    .data = (void *)&rk3228_grf,
    }, {
    .compatible = "rockchip,rk3288-grf",
    .data = (void *)&rk3288_grf,
    }, {
    .compatible = "rockchip,rk3328-grf",
    .data = (void *)&rk3328_grf,
    }, {
    .compatible = "rockchip,rk3368-grf",
    .data = (void *)&rk3368_grf,
    }, {
    .compatible = "rockchip,rk3368-pmugrf",
    .data = (void *)&rk3368_pmugrf,
    }, {
    .compatible = "rockchip,rk3399-grf",
    .data = (void *)&rk3399_grf,
    }, {
    .compatible = "rockchip,rk3566-pipe-grf",
    .data = (void *)&rk3566_pipegrf,
    }, {
    .compatible = "rockchip,rk3576-sys-grf",
    .data = (void *)&rk3576_sysgrf,
    }, {
    .compatible = "rockchip,rk3576-ioc-grf",
    .data = (void *)&rk3576_iocgrf,
    }, {
    .compatible = "rockchip,rk3588-sys-grf",
    .data = (void *)&rk3588_sysgrf,
    },
    { /* sentinel */ },
    };
#[no_mangle]
unsafe extern "C" fn rockchip_grf_init() -> int __init {
    static int __init rockchip_grf_init(void)
    {
    const struct rockchip_grf_info *grf_info;
    const struct of_device_id *match;
    struct device_node *np;
    struct regmap *grf;
    int ret, i;
    for_each_matching_node_and_match(np, rockchip_grf_dt_match, &match) {
    if (!of_device_is_available(np))
    continue;
    if (!match || !match.data) {
    pr_err("%s: missing grf data\n", __func__);
    of_node_put(np);
    return -EINVAL;
    }
    grf_info = match.data;
    grf = syscon_node_to_regmap(np);
    if (IS_ERR(grf)) {
    pr_err("%s: could not get grf syscon\n", __func__);
    of_node_put(np);
    return PTR_ERR(grf);
    }
    for (i = 0; i < grf_info.num_values; i++) {
    const struct rockchip_grf_value *val = &grf_info.values[i];
    pr_debug("%s: adjusting %s in %#6x to %#10x\n", __func__,
    val.desc, val.reg, val.val);
    ret = regmap_write(grf, val.reg, val.val);
    if (ret < 0)
    pr_err("%s: write to %#6x failed with %d\n",
    __func__, val.reg, ret);
    }
    }
    return 0;
    }
    postcore_initcall(rockchip_grf_init);

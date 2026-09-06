//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/actions/owl-sps.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Actions Semi Owl Smart Power System (SPS)
//
// Copyright 2012 Actions Semi Inc.
// Author: Actions Semi, Inc.
//
// Copyright (c) 2017 Andreas Färber
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_sps_domain_info {
    pub name: *const c_char,
    pub pwr_bit: c_int,
    pub ack_bit: c_int,
    pub genpd_flags: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_sps_info {
    pub num_domains: unsigned,
    pub domains: *const owl_sps_domain_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_sps {
    pub dev: *mut device,
    pub info: *const owl_sps_info,
    pub base: *mut void __iomem,
    pub genpd_data: genpd_onecell_data,
    pub domains: [*mut generic_pm_domain; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_sps_domain {
    pub genpd: generic_pm_domain,
    pub info: *const owl_sps_domain_info,
    pub sps: *mut owl_sps,
}

#[no_mangle]
unsafe extern "C" fn owl_sps_set_power(pd: *mut owl_sps_domain, enable: bool) -> c_int {
    static int owl_sps_set_power(struct owl_sps_domain *pd, bool enable)
    {
    u32 pwr_mask, ack_mask;
    ack_mask = BIT(pd.info.ack_bit);
    pwr_mask = BIT(pd.info.pwr_bit);
    return owl_sps_set_pg(pd.sps.base, pwr_mask, ack_mask, enable);
    }
#[no_mangle]
unsafe extern "C" fn owl_sps_power_on(domain: *mut generic_pm_domain) -> c_int {
    static int owl_sps_power_on(struct generic_pm_domain *domain)
    {
    struct owl_sps_domain *pd = to_owl_pd(domain);
    dev_dbg(pd.sps.dev, "%s power on", pd.info.name);
    return owl_sps_set_power(pd, true);
    }
#[no_mangle]
unsafe extern "C" fn owl_sps_power_off(domain: *mut generic_pm_domain) -> c_int {
    static int owl_sps_power_off(struct generic_pm_domain *domain)
    {
    struct owl_sps_domain *pd = to_owl_pd(domain);
    dev_dbg(pd.sps.dev, "%s power off", pd.info.name);
    return owl_sps_set_power(pd, false);
    }
#[no_mangle]
unsafe extern "C" fn owl_sps_init_domain(sps: *mut owl_sps, index: c_int) -> c_int {
    static int owl_sps_init_domain(struct owl_sps *sps, int index)
    {
    struct owl_sps_domain *pd;
    pd = devm_kzalloc(sps.dev, sizeof(*pd), GFP_KERNEL);
    if (!pd)
    return -ENOMEM;
    pd.info = &sps.info.domains[index];
    pd.sps = sps;
    pd.genpd.name = pd.info.name;
    pd.genpd.power_on = owl_sps_power_on;
    pd.genpd.power_off = owl_sps_power_off;
    pd.genpd.flags = pd.info.genpd_flags;
    pm_genpd_init(&pd.genpd, core::ptr::null_mut(), false);
    sps.genpd_data.domains[index] = &pd.genpd;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn owl_sps_probe(pdev: *mut platform_device) -> c_int {
    static int owl_sps_probe(struct platform_device *pdev)
    {
    const struct owl_sps_info *sps_info;
    struct owl_sps *sps;
    int i, ret;
    sps_info = device_get_match_data(&pdev.dev);
    if (!sps_info) {
    dev_err(&pdev.dev, "unknown compatible or missing data\n");
    return -EINVAL;
    }
    sps = devm_kzalloc(&pdev.dev,
    struct_size(sps, domains, sps_info.num_domains),
    GFP_KERNEL);
    if (!sps)
    return -ENOMEM;
    sps.base = of_io_request_and_map(pdev.dev.of_node, 0, "owl-sps");
    if (IS_ERR(sps.base)) {
    dev_err(&pdev.dev, "failed to map sps registers\n");
    return PTR_ERR(sps.base);
    }
    sps.dev = &pdev.dev;
    sps.info = sps_info;
    sps.genpd_data.domains = sps.domains;
    sps.genpd_data.num_domains = sps_info.num_domains;
    for (i = 0; i < sps_info.num_domains; i++) {
    ret = owl_sps_init_domain(sps, i);
    if (ret)
    return ret;
    }
    ret = of_genpd_add_provider_onecell(pdev.dev.of_node, &sps.genpd_data);
    if (ret) {
    dev_err(&pdev.dev, "failed to add provider (%d)", ret);
    return ret;
    }
    return 0;
    }
    static const struct owl_sps_domain_info s500_sps_domains[] = {
    [S500_PD_VDE] = {
    .name = "VDE",
    .pwr_bit = 0,
    .ack_bit = 16,
    },
    [S500_PD_VCE_SI] = {
    .name = "VCE_SI",
    .pwr_bit = 1,
    .ack_bit = 17,
    },
    [S500_PD_USB2_1] = {
    .name = "USB2_1",
    .pwr_bit = 2,
    .ack_bit = 18,
    },
    [S500_PD_CPU2] = {
    .name = "CPU2",
    .pwr_bit = 5,
    .ack_bit = 21,
    .genpd_flags = GENPD_FLAG_ALWAYS_ON,
    },
    [S500_PD_CPU3] = {
    .name = "CPU3",
    .pwr_bit = 6,
    .ack_bit = 22,
    .genpd_flags = GENPD_FLAG_ALWAYS_ON,
    },
    [S500_PD_DMA] = {
    .name = "DMA",
    .pwr_bit = 8,
    .ack_bit = 12,
    },
    [S500_PD_DS] = {
    .name = "DS",
    .pwr_bit = 9,
    .ack_bit = 13,
    },
    [S500_PD_USB3] = {
    .name = "USB3",
    .pwr_bit = 10,
    .ack_bit = 14,
    },
    [S500_PD_USB2_0] = {
    .name = "USB2_0",
    .pwr_bit = 11,
    .ack_bit = 15,
    },
    };
    static const struct owl_sps_info s500_sps_info = {
    .num_domains = ARRAY_SIZE(s500_sps_domains),
    .domains = s500_sps_domains,
    };
    static const struct owl_sps_domain_info s700_sps_domains[] = {
    [S700_PD_VDE] = {
    .name = "VDE",
    .pwr_bit = 0,
    },
    [S700_PD_VCE_SI] = {
    .name = "VCE_SI",
    .pwr_bit = 1,
    },
    [S700_PD_USB2_1] = {
    .name = "USB2_1",
    .pwr_bit = 2,
    },
    [S700_PD_HDE] = {
    .name = "HDE",
    .pwr_bit = 7,
    },
    [S700_PD_DMA] = {
    .name = "DMA",
    .pwr_bit = 8,
    },
    [S700_PD_DS] = {
    .name = "DS",
    .pwr_bit = 9,
    },
    [S700_PD_USB3] = {
    .name = "USB3",
    .pwr_bit = 10,
    },
    [S700_PD_USB2_0] = {
    .name = "USB2_0",
    .pwr_bit = 11,
    },
    };
    static const struct owl_sps_info s700_sps_info = {
    .num_domains = ARRAY_SIZE(s700_sps_domains),
    .domains = s700_sps_domains,
    };
    static const struct owl_sps_domain_info s900_sps_domains[] = {
    [S900_PD_GPU_B] = {
    .name = "GPU_B",
    .pwr_bit = 3,
    },
    [S900_PD_VCE] = {
    .name = "VCE",
    .pwr_bit = 4,
    },
    [S900_PD_SENSOR] = {
    .name = "SENSOR",
    .pwr_bit = 5,
    },
    [S900_PD_VDE] = {
    .name = "VDE",
    .pwr_bit = 6,
    },
    [S900_PD_HDE] = {
    .name = "HDE",
    .pwr_bit = 7,
    },
    [S900_PD_USB3] = {
    .name = "USB3",
    .pwr_bit = 8,
    },
    [S900_PD_DDR0] = {
    .name = "DDR0",
    .pwr_bit = 9,
    },
    [S900_PD_DDR1] = {
    .name = "DDR1",
    .pwr_bit = 10,
    },
    [S900_PD_DE] = {
    .name = "DE",
    .pwr_bit = 13,
    },
    [S900_PD_NAND] = {
    .name = "NAND",
    .pwr_bit = 14,
    },
    [S900_PD_USB2_H0] = {
    .name = "USB2_H0",
    .pwr_bit = 15,
    },
    [S900_PD_USB2_H1] = {
    .name = "USB2_H1",
    .pwr_bit = 16,
    },
    };
    static const struct owl_sps_info s900_sps_info = {
    .num_domains = ARRAY_SIZE(s900_sps_domains),
    .domains = s900_sps_domains,
    };
    static const struct of_device_id owl_sps_of_matches[] = {
    { .compatible = "actions,s500-sps", .data = &s500_sps_info },
    { .compatible = "actions,s700-sps", .data = &s700_sps_info },
    { .compatible = "actions,s900-sps", .data = &s900_sps_info },
    { }
    };
    static struct platform_driver owl_sps_platform_driver = {
    .probe = owl_sps_probe,
    .driver = {
    .name = "owl-sps",
    .of_match_table = owl_sps_of_matches,
    .suppress_bind_attrs = true,
    },
    };
#[no_mangle]
unsafe extern "C" fn owl_sps_init() -> int __init {
    static int __init owl_sps_init(void)
    {
    return platform_driver_register(&owl_sps_platform_driver);
    }
    postcore_initcall(owl_sps_init);

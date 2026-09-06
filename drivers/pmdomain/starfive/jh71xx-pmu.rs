//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/starfive/jh71xx-pmu.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// StarFive JH71XX PMU (Power Management Unit) Controller Driver
//
// Copyright (C) 2022-2023 StarFive Technology Co., Ltd.
//

// register offset
pub const JH71XX_PMU_SW_TURN_ON_POWER: c_uint = 0x0C;
pub const JH71XX_PMU_SW_TURN_OFF_POWER: c_uint = 0x10;
pub const JH71XX_PMU_SW_ENCOURAGE: c_uint = 0x44;
pub const JH71XX_PMU_TIMER_INT_MASK: c_uint = 0x48;
pub const JH71XX_PMU_CURR_POWER_MODE: c_uint = 0x80;
pub const JH71XX_PMU_EVENT_STATUS: c_uint = 0x88;
pub const JH71XX_PMU_INT_STATUS: c_uint = 0x8C;
// aon pmu register offset
pub const JH71XX_AON_PMU_SWITCH: c_uint = 0x00;
// sw encourage cfg
pub const JH71XX_PMU_SW_ENCOURAGE_EN_LO: c_uint = 0x05;
pub const JH71XX_PMU_SW_ENCOURAGE_EN_HI: c_uint = 0x50;
pub const JH71XX_PMU_SW_ENCOURAGE_DIS_LO: c_uint = 0x0A;
pub const JH71XX_PMU_SW_ENCOURAGE_DIS_HI: c_uint = 0xA0;
pub const JH71XX_PMU_SW_ENCOURAGE_ON: c_uint = 0xFF;
// pmu int status

//
// The time required for switching power status is based on the time
// to turn on the largest domain's power, which is at microsecond level
//
pub const JH71XX_PMU_TIMEOUT_US: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh71xx_domain_info {
    pub name: *const *const c_char,
    pub flags: c_uint,
    pub bit: u8,
}

    struct jh71xx_pmu;
    struct jh71xx_pmu_dev;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh71xx_pmu_match_data {
    pub domain_info: *const jh71xx_domain_info,
    pub num_domains: c_int,
    pub pmu_status: c_uint,
    int (*pmu_parse_irq)(struct platform_device *pdev,
    pub pmu): *mut jh71xx_pmu,
    int (*pmu_set_state)(struct jh71xx_pmu_dev *pmd,
    pub on): u32 mask, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh71xx_pmu {
    pub dev: *mut device,
    pub match_data: *const jh71xx_pmu_match_data,
    pub base: *mut void __iomem,
    pub genpd: *mut generic_pm_domain,
    pub genpd_data: genpd_onecell_data,
    pub irq: c_int,
    pub /: *mut *mut spinlock_t lock; / protects pmu reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jh71xx_pmu_dev {
    pub domain_info: *const jh71xx_domain_info,
    pub pmu: *mut jh71xx_pmu,
    pub genpd: generic_pm_domain,
}

#[no_mangle]
unsafe extern "C" fn jh71xx_pmu_get_state(pmd: *mut jh71xx_pmu_dev, mask: u32, is_on: *mut bool) -> c_int {
    static int jh71xx_pmu_get_state(struct jh71xx_pmu_dev *pmd, u32 mask, bool *is_on)
    {
    struct jh71xx_pmu *pmu = pmd.pmu;
    if (!mask)
    return -EINVAL;
// is_on = readl(pmu->base + pmu->match_data->pmu_status) & mask;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jh7110_pmu_set_state(pmd: *mut jh71xx_pmu_dev, mask: u32, on: bool) -> c_int {
    static int jh7110_pmu_set_state(struct jh71xx_pmu_dev *pmd, u32 mask, bool on)
    {
    struct jh71xx_pmu *pmu = pmd.pmu;
    unsigned long flags;
    u32 val;
    u32 mode;
    u32 encourage_lo;
    u32 encourage_hi;
    int ret;
    spin_lock_irqsave(&pmu.lock, flags);
//
// The PMU accepts software encourage to switch power mode in the following 2 steps:
//
// 1.Configure the register SW_TURN_ON_POWER (offset 0x0c) by writing 1 to
// the bit corresponding to the power domain that will be turned on
// and writing 0 to the others.
// Likewise, configure the register SW_TURN_OFF_POWER (offset 0x10) by
// writing 1 to the bit corresponding to the power domain that will be
// turned off and writing 0 to the others.
//
    if (on) {
    mode = JH71XX_PMU_SW_TURN_ON_POWER;
    encourage_lo = JH71XX_PMU_SW_ENCOURAGE_EN_LO;
    encourage_hi = JH71XX_PMU_SW_ENCOURAGE_EN_HI;
    } else {
    mode = JH71XX_PMU_SW_TURN_OFF_POWER;
    encourage_lo = JH71XX_PMU_SW_ENCOURAGE_DIS_LO;
    encourage_hi = JH71XX_PMU_SW_ENCOURAGE_DIS_HI;
    }
    writel(mask, pmu.base + mode);
//
// 2.Write SW encourage command sequence to the Software Encourage Reg (offset 0x44)
// First write SW_MODE_ENCOURAGE_ON to JH71XX_PMU_SW_ENCOURAGE. This will reset
// the state machine which parses the command sequence. This register must be
// written every time software wants to power on/off a domain.
// Then write the lower bits of the command sequence, followed by the upper
// bits. The sequence differs between powering on & off a domain.
//
    writel(JH71XX_PMU_SW_ENCOURAGE_ON, pmu.base + JH71XX_PMU_SW_ENCOURAGE);
    writel(encourage_lo, pmu.base + JH71XX_PMU_SW_ENCOURAGE);
    writel(encourage_hi, pmu.base + JH71XX_PMU_SW_ENCOURAGE);
    spin_unlock_irqrestore(&pmu.lock, flags);
// Wait for the power domain bit to be enabled / disabled
    if (on) {
    ret = readl_poll_timeout_atomic(pmu.base + JH71XX_PMU_CURR_POWER_MODE,
    val, val & mask,
    1, JH71XX_PMU_TIMEOUT_US);
    } else {
    ret = readl_poll_timeout_atomic(pmu.base + JH71XX_PMU_CURR_POWER_MODE,
    val, !(val & mask),
    1, JH71XX_PMU_TIMEOUT_US);
    }
    if (ret) {
    dev_err(pmu.dev, "%s: failed to power %s\n",
    pmd.genpd.name, on ? "on" : "off");
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jh7110_aon_pmu_set_state(pmd: *mut jh71xx_pmu_dev, mask: u32, on: bool) -> c_int {
    static int jh7110_aon_pmu_set_state(struct jh71xx_pmu_dev *pmd, u32 mask, bool on)
    {
    struct jh71xx_pmu *pmu = pmd.pmu;
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(&pmu.lock, flags);
    val = readl(pmu.base + JH71XX_AON_PMU_SWITCH);
    if (on)
    val |= mask;
    else
    val &= ~mask;
    writel(val, pmu.base + JH71XX_AON_PMU_SWITCH);
    spin_unlock_irqrestore(&pmu.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jh71xx_pmu_set_state(pmd: *mut jh71xx_pmu_dev, mask: u32, on: bool) -> c_int {
    static int jh71xx_pmu_set_state(struct jh71xx_pmu_dev *pmd, u32 mask, bool on)
    {
    struct jh71xx_pmu *pmu = pmd.pmu;
    const struct jh71xx_pmu_match_data *match_data = pmu.match_data;
    bool is_on;
    int ret;
    ret = jh71xx_pmu_get_state(pmd, mask, &is_on);
    if (ret) {
    dev_dbg(pmu.dev, "unable to get current state for %s\n",
    pmd.genpd.name);
    return ret;
    }
    if (is_on == on) {
    dev_dbg(pmu.dev, "pm domain [%s] is already %sable status.\n",
    pmd.genpd.name, on ? "en" : "dis");
    return 0;
    }
    return match_data.pmu_set_state(pmd, mask, on);
    }
#[no_mangle]
unsafe extern "C" fn jh71xx_pmu_on(genpd: *mut generic_pm_domain) -> c_int {
    static int jh71xx_pmu_on(struct generic_pm_domain *genpd)
    {
    struct jh71xx_pmu_dev *pmd = container_of(genpd,
    struct jh71xx_pmu_dev, genpd);
    let mut pwr_mask: u32 = BIT(pmd.domain_info.bit);
    return jh71xx_pmu_set_state(pmd, pwr_mask, true);
    }
#[no_mangle]
unsafe extern "C" fn jh71xx_pmu_off(genpd: *mut generic_pm_domain) -> c_int {
    static int jh71xx_pmu_off(struct generic_pm_domain *genpd)
    {
    struct jh71xx_pmu_dev *pmd = container_of(genpd,
    struct jh71xx_pmu_dev, genpd);
    let mut pwr_mask: u32 = BIT(pmd.domain_info.bit);
    return jh71xx_pmu_set_state(pmd, pwr_mask, false);
    }
#[no_mangle]
unsafe extern "C" fn jh71xx_pmu_int_enable(pmu: *mut jh71xx_pmu, mask: u32, enable: bool) {
    static void jh71xx_pmu_int_enable(struct jh71xx_pmu *pmu, u32 mask, bool enable)
    {
    u32 val;
    unsigned long flags;
    spin_lock_irqsave(&pmu.lock, flags);
    val = readl(pmu.base + JH71XX_PMU_TIMER_INT_MASK);
    if (enable)
    val &= ~mask;
    else
    val |= mask;
    writel(val, pmu.base + JH71XX_PMU_TIMER_INT_MASK);
    spin_unlock_irqrestore(&pmu.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn jh71xx_pmu_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t jh71xx_pmu_interrupt(int irq, void *data)
    {
    struct jh71xx_pmu *pmu = data;
    u32 val;
    val = readl(pmu.base + JH71XX_PMU_INT_STATUS);
    if (val & JH71XX_PMU_INT_SEQ_DONE)
    dev_dbg(pmu.dev, "sequence done.\n");
    if (val & JH71XX_PMU_INT_HW_REQ)
    dev_dbg(pmu.dev, "hardware encourage requestion.\n");
    if (val & JH71XX_PMU_INT_SW_FAIL)
    dev_err(pmu.dev, "software encourage fail.\n");
    if (val & JH71XX_PMU_INT_HW_FAIL)
    dev_err(pmu.dev, "hardware encourage fail.\n");
    if (val & JH71XX_PMU_INT_PCH_FAIL)
    dev_err(pmu.dev, "p-channel fail event.\n");
// clear interrupts
    writel(val, pmu.base + JH71XX_PMU_INT_STATUS);
    writel(val, pmu.base + JH71XX_PMU_EVENT_STATUS);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn jh7110_pmu_parse_irq(pdev: *mut platform_device, pmu: *mut jh71xx_pmu) -> c_int {
    static int jh7110_pmu_parse_irq(struct platform_device *pdev, struct jh71xx_pmu *pmu)
    {
    struct device *dev = &pdev.dev;
    int ret;
    pmu.irq = platform_get_irq(pdev, 0);
    if (pmu.irq < 0)
    return pmu.irq;
    ret = devm_request_irq(dev, pmu.irq, jh71xx_pmu_interrupt,
    0, pdev.name, pmu);
    if (ret)
    dev_err(dev, "failed to request irq\n");
    jh71xx_pmu_int_enable(pmu, JH71XX_PMU_INT_ALL_MASK & ~JH71XX_PMU_INT_PCH_FAIL, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jh71xx_pmu_init_domain(pmu: *mut jh71xx_pmu, index: c_int) -> c_int {
    static int jh71xx_pmu_init_domain(struct jh71xx_pmu *pmu, int index)
    {
    struct jh71xx_pmu_dev *pmd;
    u32 pwr_mask;
    int ret;
    let mut is_on: bool = false;
    pmd = devm_kzalloc(pmu.dev, sizeof(*pmd), GFP_KERNEL);
    if (!pmd)
    return -ENOMEM;
    pmd.domain_info = &pmu.match_data.domain_info[index];
    pmd.pmu = pmu;
    pwr_mask = BIT(pmd.domain_info.bit);
    pmd.genpd.name = pmd.domain_info.name;
    pmd.genpd.flags = pmd.domain_info.flags;
    ret = jh71xx_pmu_get_state(pmd, pwr_mask, &is_on);
    if (ret)
    dev_warn(pmu.dev, "unable to get current state for %s\n",
    pmd.genpd.name);
    pmd.genpd.power_on = jh71xx_pmu_on;
    pmd.genpd.power_off = jh71xx_pmu_off;
    pm_genpd_init(&pmd.genpd, core::ptr::null_mut(), !is_on);
    pmu.genpd_data.domains[index] = &pmd.genpd;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jh71xx_pmu_probe(pdev: *mut platform_device) -> c_int {
    static int jh71xx_pmu_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    const struct jh71xx_pmu_match_data *match_data;
    struct jh71xx_pmu *pmu;
    unsigned int i;
    int ret;
    pmu = devm_kzalloc(dev, sizeof(*pmu), GFP_KERNEL);
    if (!pmu)
    return -ENOMEM;
    pmu.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pmu.base))
    return PTR_ERR(pmu.base);
    spin_lock_init(&pmu.lock);
    match_data = of_device_get_match_data(dev);
    if (!match_data)
    return -EINVAL;
    if (match_data.pmu_parse_irq) {
    ret = match_data.pmu_parse_irq(pdev, pmu);
    if (ret) {
    dev_err(dev, "failed to parse irq\n");
    return ret;
    }
    }
    pmu.genpd = devm_kcalloc(dev, match_data.num_domains,
    sizeof(struct generic_pm_domain *),
    GFP_KERNEL);
    if (!pmu.genpd)
    return -ENOMEM;
    pmu.dev = dev;
    pmu.match_data = match_data;
    pmu.genpd_data.domains = pmu.genpd;
    pmu.genpd_data.num_domains = match_data.num_domains;
    for (i = 0; i < match_data.num_domains; i++) {
    ret = jh71xx_pmu_init_domain(pmu, i);
    if (ret) {
    dev_err(dev, "failed to initialize power domain\n");
    return ret;
    }
    }
    ret = of_genpd_add_provider_onecell(np, &pmu.genpd_data);
    if (ret) {
    dev_err(dev, "failed to register genpd driver: %d\n", ret);
    return ret;
    }
    dev_dbg(dev, "registered %u power domains\n", i);
    return 0;
    }
    static const struct jh71xx_domain_info jh7110_power_domains[] = {
    [JH7110_PD_SYSTOP] = {
    .name = "SYSTOP",
    .bit = 0,
    .flags = GENPD_FLAG_ALWAYS_ON,
    },
    [JH7110_PD_CPU] = {
    .name = "CPU",
    .bit = 1,
    .flags = GENPD_FLAG_ALWAYS_ON,
    },
    [JH7110_PD_GPUA] = {
    .name = "GPUA",
    .bit = 2,
    },
    [JH7110_PD_VDEC] = {
    .name = "VDEC",
    .bit = 3,
    },
    [JH7110_PD_VOUT] = {
    .name = "VOUT",
    .bit = 4,
    },
    [JH7110_PD_ISP] = {
    .name = "ISP",
    .bit = 5,
    },
    [JH7110_PD_VENC] = {
    .name = "VENC",
    .bit = 6,
    },
    };
    static const struct jh71xx_pmu_match_data jh7110_pmu = {
    .num_domains = ARRAY_SIZE(jh7110_power_domains),
    .domain_info = jh7110_power_domains,
    .pmu_status = JH71XX_PMU_CURR_POWER_MODE,
    .pmu_parse_irq = jh7110_pmu_parse_irq,
    .pmu_set_state = jh7110_pmu_set_state,
    };
    static const struct jh71xx_domain_info jh7110_aon_power_domains[] = {
    [JH7110_AON_PD_DPHY_TX] = {
    .name = "DPHY-TX",
    .bit = 30,
    },
    [JH7110_AON_PD_DPHY_RX] = {
    .name = "DPHY-RX",
    .bit = 31,
    },
    };
    static const struct jh71xx_pmu_match_data jh7110_aon_pmu = {
    .num_domains = ARRAY_SIZE(jh7110_aon_power_domains),
    .domain_info = jh7110_aon_power_domains,
    .pmu_status = JH71XX_AON_PMU_SWITCH,
    .pmu_set_state = jh7110_aon_pmu_set_state,
    };
    static const struct of_device_id jh71xx_pmu_of_match[] = {
    {
    .compatible = "starfive,jh7110-pmu",
    .data = (void *)&jh7110_pmu,
    }, {
    .compatible = "starfive,jh7110-aon-syscon",
    .data = (void *)&jh7110_aon_pmu,
    }, {
// sentinel
    }
    };
    static struct platform_driver jh71xx_pmu_driver = {
    .probe = jh71xx_pmu_probe,
    .driver = {
    .name = "jh71xx-pmu",
    .of_match_table = jh71xx_pmu_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(jh71xx_pmu_driver);
    MODULE_AUTHOR("Walker Chen <walker.chen@starfivetech.com>");
    MODULE_AUTHOR("Changhuang Liang <changhuang.liang@starfivetech.com>");
    MODULE_DESCRIPTION("StarFive JH71XX PMU Driver");
    MODULE_LICENSE("GPL");

//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-pxa.c
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
// drivers/pwm/pwm-pxa.c
//
// simple driver for PWM (Pulse Width Modulator) controller
//
// 2008-02-13	initial version
// eric miao <eric.miao@marvell.com>
//
// Links to reference manuals for some of the supported PWM chips can be found
// in Documentation/arch/arm/marvell.rst.
//
// Limitations:
// - When PWM is stopped, the current PWM period stops abruptly at the next
// input clock (PWMCR_SD is set) and the output is driven to inactive.
//

pub const HAS_SECONDARY_PWM: c_uint = 0x10;
    static const struct platform_device_id pwm_id_table[] = {
// PWM            has_secondary_pwm?
    { .name = "pxa25x-pwm", .driver_data = 0 },
    { .name = "pxa27x-pwm", .driver_data = HAS_SECONDARY_PWM },
    { .name = "pxa168-pwm", .driver_data = 0 },
    { .name = "pxa910-pwm", .driver_data = 0 },
    { }
    };
    MODULE_DEVICE_TABLE(platform, pwm_id_table);
// PWM registers and bits definitions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa_pwm_chip {
    pub dev: *mut device,
    pub clk: *mut clk,
    pub mmio_base: *mut void __iomem,
}

    static inline struct pxa_pwm_chip *to_pxa_pwm_chip(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
//
// period_ns = 10^9 * (PRESCALE + 1) * (PV + 1) / PWM_CLK_RATE
// duty_ns   = 10^9 * (PRESCALE + 1) * DC / PWM_CLK_RATE
//
    static int pxa_pwm_config(struct pwm_chip *chip, struct pwm_device *pwm,
    u64 duty_ns, u64 period_ns)
    {
    struct pxa_pwm_chip *pc = to_pxa_pwm_chip(chip);
    unsigned long long c;
    unsigned long period_cycles, prescale, pv, dc;
    unsigned long offset;
    offset = pwm.hwpwm ? 0x10 : 0;
    c = clk_get_rate(pc.clk);
    c = c * period_ns;
    do_div(c, 1000000000);
    period_cycles = c;
    if (period_cycles < 1)
    period_cycles = 1;
    prescale = (period_cycles - 1) / 1024;
    pv = period_cycles / (prescale + 1) - 1;
    if (prescale > 63)
    return -EINVAL;
    if (duty_ns == period_ns)
    dc = PWMDCR_FD;
    else
    dc = mul_u64_u64_div_u64(pv + 1, duty_ns, period_ns);
    writel(prescale | PWMCR_SD, pc.mmio_base + offset + PWMCR);
    writel(dc, pc.mmio_base + offset + PWMDCR);
    writel(pv, pc.mmio_base + offset + PWMPCR);
    return 0;
    }
    static int pxa_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct pxa_pwm_chip *pc = to_pxa_pwm_chip(chip);
    u64 duty_cycle;
    int err;
    if (state.polarity != PWM_POLARITY_NORMAL)
    return -EINVAL;
    err = clk_prepare_enable(pc.clk);
    if (err)
    return err;
    duty_cycle = state.enabled ? state.duty_cycle : 0;
    err = pxa_pwm_config(chip, pwm, duty_cycle, state.period);
    if (err) {
    clk_disable_unprepare(pc.clk);
    return err;
    }
    if (state.enabled && !pwm.state.enabled)
    return 0;
    clk_disable_unprepare(pc.clk);
    if (!state.enabled && pwm.state.enabled)
    clk_disable_unprepare(pc.clk);
    return 0;
    }
    static const struct pwm_ops pxa_pwm_ops = {
    .apply = pxa_pwm_apply,
    };
//
// Device tree users must create one device instance for each PWM channel.
// Hence we dispense with the HAS_SECONDARY_PWM and "tell" the original driver
// code that this is a single channel pxa25x-pwm.  Currently all devices are
// supported identically.
//
    static const struct of_device_id pwm_of_match[] = {
    { .compatible = "marvell,pxa250-pwm", .data = &pwm_id_table[0] },
    { .compatible = "marvell,pxa270-pwm", .data = &pwm_id_table[0] },
    { .compatible = "marvell,pxa168-pwm", .data = &pwm_id_table[0] },
    { .compatible = "marvell,pxa910-pwm", .data = &pwm_id_table[0] },
    { }
    };
    MODULE_DEVICE_TABLE(of, pwm_of_match);
#[no_mangle]
unsafe extern "C" fn pwm_probe(pdev: *mut platform_device) -> c_int {
    static int pwm_probe(struct platform_device *pdev)
    {
    const struct platform_device_id *id = platform_get_device_id(pdev);
    struct pwm_chip *chip;
    struct pxa_pwm_chip *pc;
    struct clk *bus_clk;
    struct device *dev = &pdev.dev;
    struct reset_control *rst;
    let mut ret: c_int = 0;
    if (id == core::ptr::null_mut())
    id = of_device_get_match_data(dev);
    if (id == core::ptr::null_mut())
    return -EINVAL;
    chip = devm_pwmchip_alloc(dev, (id.driver_data & HAS_SECONDARY_PWM) ? 2 : 1,
    sizeof(*pc));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    pc = to_pxa_pwm_chip(chip);
    bus_clk = devm_clk_get_optional_enabled(dev, "bus");
    if (IS_ERR(bus_clk))
    return dev_err_probe(dev, PTR_ERR(bus_clk), "Failed to get bus clock\n");
// Get named func clk if bus clock is valid
    pc.clk = devm_clk_get(dev, bus_clk ? "func" : core::ptr::null_mut());
    if (IS_ERR(pc.clk))
    return dev_err_probe(dev, PTR_ERR(pc.clk), "Failed to get clock\n");
    rst = devm_reset_control_get_optional_exclusive_deasserted(dev, core::ptr::null_mut());
    if (IS_ERR(rst))
    return PTR_ERR(rst);
    chip.ops = &pxa_pwm_ops;
    chip.of_xlate = of_pwm_single_xlate;
    pc.mmio_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pc.mmio_base))
    return PTR_ERR(pc.mmio_base);
    ret = devm_pwmchip_add(dev, chip);
    if (ret < 0)
    return dev_err_probe(dev, ret, "pwmchip_add() failed\n");
    return 0;
    }
    static struct platform_driver pwm_driver = {
    .driver		= {
    .name	= "pxa25x-pwm",
    .of_match_table = pwm_of_match,
    },
    .probe		= pwm_probe,
    .id_table	= pwm_id_table,
    };
    module_platform_driver(pwm_driver);
    MODULE_DESCRIPTION("PXA Pulse Width Modulator driver");
    MODULE_LICENSE("GPL v2");

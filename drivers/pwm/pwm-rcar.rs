//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-rcar.c
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
// R-Car PWM Timer driver
//
// Copyright (C) 2015 Renesas Electronics Corporation
//
// Limitations:
// - The hardware cannot generate a 0% duty cycle.
//

pub const RCAR_PWM_MAX_DIVISION: c_int = 24;
pub const RCAR_PWM_MAX_CYCLE: c_int = 1023;
pub const RCAR_PWMCR: c_uint = 0x00;
pub const RCAR_PWMCR_CC0_MASK: c_uint = 0x000f0000;
pub const RCAR_PWMCR_CC0_SHIFT: c_int = 16;

pub const RCAR_PWMCNT: c_uint = 0x04;
pub const RCAR_PWMCNT_CYC0_MASK: c_uint = 0x03ff0000;
pub const RCAR_PWMCNT_CYC0_SHIFT: c_int = 16;
pub const RCAR_PWMCNT_PH0_MASK: c_uint = 0x000003ff;
pub const RCAR_PWMCNT_PH0_SHIFT: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_pwm_chip {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
}

    static inline struct rcar_pwm_chip *to_rcar_pwm_chip(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
    static void rcar_pwm_write(struct rcar_pwm_chip *rp, u32 data,
    unsigned int offset)
    {
    writel(data, rp.base + offset);
    }
#[no_mangle]
unsafe extern "C" fn rcar_pwm_read(rp: *mut rcar_pwm_chip, offset: c_uint) -> u32 {
    static u32 rcar_pwm_read(struct rcar_pwm_chip *rp, unsigned int offset)
    {
    return readl(rp.base + offset);
    }
    static void rcar_pwm_update(struct rcar_pwm_chip *rp, u32 mask, u32 data,
    unsigned int offset)
    {
    u32 value;
    value = rcar_pwm_read(rp, offset);
    value &= ~mask;
    value |= data & mask;
    rcar_pwm_write(rp, value, offset);
    }
#[no_mangle]
unsafe extern "C" fn rcar_pwm_get_clock_division(rp: *mut rcar_pwm_chip, period_ns: c_int) -> c_int {
    static int rcar_pwm_get_clock_division(struct rcar_pwm_chip *rp, int period_ns)
    {
    let mut clk_rate: c_ulong = clk_get_rate(rp.clk);
    u64 div, tmp;
    if (clk_rate == 0)
    return -EINVAL;
    div = (u64)NSEC_PER_SEC * RCAR_PWM_MAX_CYCLE;
    tmp = (u64)period_ns * clk_rate + div - 1;
    tmp = div64_u64(tmp, div);
    div = ilog2(tmp - 1) + 1;
    return (div <= RCAR_PWM_MAX_DIVISION) ? div : -ERANGE;
    }
    static void rcar_pwm_set_clock_control(struct rcar_pwm_chip *rp,
    unsigned int div)
    {
    u32 value;
    value = rcar_pwm_read(rp, RCAR_PWMCR);
    value &= ~(RCAR_PWMCR_CCMD | RCAR_PWMCR_CC0_MASK);
    if (div & 1)
    value |= RCAR_PWMCR_CCMD;
    div >>= 1;
    value |= div << RCAR_PWMCR_CC0_SHIFT;
    rcar_pwm_write(rp, value, RCAR_PWMCR);
    }
    static int rcar_pwm_set_counter(struct rcar_pwm_chip *rp, int div, u64 duty_ns,
    u64 period_ns)
    {
    unsigned long long tmp;
    let mut clk_rate: c_ulong = clk_get_rate(rp.clk);
    u32 cyc, ph;
// div <= 24 == RCAR_PWM_MAX_DIVISION, so the shift doesn't overflow.
    tmp = mul_u64_u64_div_u64(period_ns, clk_rate, (u64)NSEC_PER_SEC << div);
    if (tmp > FIELD_MAX(RCAR_PWMCNT_CYC0_MASK))
    tmp = FIELD_MAX(RCAR_PWMCNT_CYC0_MASK);
    cyc = FIELD_PREP(RCAR_PWMCNT_CYC0_MASK, tmp);
    tmp = mul_u64_u64_div_u64(duty_ns, clk_rate, (u64)NSEC_PER_SEC << div);
    if (tmp > FIELD_MAX(RCAR_PWMCNT_PH0_MASK))
    tmp = FIELD_MAX(RCAR_PWMCNT_PH0_MASK);
    ph = FIELD_PREP(RCAR_PWMCNT_PH0_MASK, tmp);
// Avoid prohibited setting
    if (cyc == 0 || ph == 0)
    return -EINVAL;
    rcar_pwm_write(rp, cyc | ph, RCAR_PWMCNT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rcar_pwm_request(chip: *mut pwm_chip, pwm: *mut pwm_device) -> c_int {
    static int rcar_pwm_request(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    return pm_runtime_get_sync(pwmchip_parent(chip));
    }
#[no_mangle]
unsafe extern "C" fn rcar_pwm_free(chip: *mut pwm_chip, pwm: *mut pwm_device) {
    static void rcar_pwm_free(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    pm_runtime_put(pwmchip_parent(chip));
    }
#[no_mangle]
unsafe extern "C" fn rcar_pwm_enable(rp: *mut rcar_pwm_chip) -> c_int {
    static int rcar_pwm_enable(struct rcar_pwm_chip *rp)
    {
    u32 value;
// Don't enable the PWM device if CYC0 or PH0 is 0
    value = rcar_pwm_read(rp, RCAR_PWMCNT);
    if ((value & RCAR_PWMCNT_CYC0_MASK) == 0 ||
    (value & RCAR_PWMCNT_PH0_MASK) == 0)
    return -EINVAL;
    rcar_pwm_update(rp, RCAR_PWMCR_EN0, RCAR_PWMCR_EN0, RCAR_PWMCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rcar_pwm_disable(rp: *mut rcar_pwm_chip) {
    static void rcar_pwm_disable(struct rcar_pwm_chip *rp)
    {
    rcar_pwm_update(rp, RCAR_PWMCR_EN0, 0, RCAR_PWMCR);
    }
    static int rcar_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct rcar_pwm_chip *rp = to_rcar_pwm_chip(chip);
    int div, ret;
// This HW/driver only supports normal polarity
    if (state.polarity != PWM_POLARITY_NORMAL)
    return -EINVAL;
    if (!state.enabled) {
    rcar_pwm_disable(rp);
    return 0;
    }
    div = rcar_pwm_get_clock_division(rp, state.period);
    if (div < 0)
    return div;
    rcar_pwm_update(rp, RCAR_PWMCR_SYNC, RCAR_PWMCR_SYNC, RCAR_PWMCR);
    ret = rcar_pwm_set_counter(rp, div, state.duty_cycle, state.period);
    if (!ret)
    rcar_pwm_set_clock_control(rp, div);
// The SYNC should be set to 0 even if rcar_pwm_set_counter failed
    rcar_pwm_update(rp, RCAR_PWMCR_SYNC, 0, RCAR_PWMCR);
    if (!ret)
    ret = rcar_pwm_enable(rp);
    return ret;
    }
    static const struct pwm_ops rcar_pwm_ops = {
    .request = rcar_pwm_request,
    .free = rcar_pwm_free,
    .apply = rcar_pwm_apply,
    };
#[no_mangle]
unsafe extern "C" fn rcar_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int rcar_pwm_probe(struct platform_device *pdev)
    {
    struct pwm_chip *chip;
    struct rcar_pwm_chip *rcar_pwm;
    int ret;
    chip = devm_pwmchip_alloc(&pdev.dev, 1, sizeof(*rcar_pwm));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    rcar_pwm = to_rcar_pwm_chip(chip);
    rcar_pwm.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rcar_pwm.base))
    return PTR_ERR(rcar_pwm.base);
    rcar_pwm.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(rcar_pwm.clk)) {
    dev_err(&pdev.dev, "cannot get clock\n");
    return PTR_ERR(rcar_pwm.clk);
    }
    chip.ops = &rcar_pwm_ops;
    platform_set_drvdata(pdev, chip);
    pm_runtime_enable(&pdev.dev);
    ret = pwmchip_add(chip);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to register PWM chip: %d\n", ret);
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rcar_pwm_remove(pdev: *mut platform_device) {
    static void rcar_pwm_remove(struct platform_device *pdev)
    {
    struct pwm_chip *chip = platform_get_drvdata(pdev);
    pwmchip_remove(chip);
    pm_runtime_disable(&pdev.dev);
    }
    static const struct of_device_id rcar_pwm_of_table[] = {
    { .compatible = "renesas,pwm-rcar" },
    { }
    };
    MODULE_DEVICE_TABLE(of, rcar_pwm_of_table);
    static struct platform_driver rcar_pwm_driver = {
    .probe = rcar_pwm_probe,
    .remove = rcar_pwm_remove,
    .driver = {
    .name = "pwm-rcar",
    .of_match_table = rcar_pwm_of_table,
    }
    };
    module_platform_driver(rcar_pwm_driver);
    MODULE_AUTHOR("Yoshihiro Shimoda <yoshihiro.shimoda.uh@renesas.com>");
    MODULE_DESCRIPTION("Renesas PWM Timer Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:pwm-rcar");

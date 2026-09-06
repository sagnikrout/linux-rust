//! Automatically rewritten from C to Rust
//! Source: drivers/pwm/pwm-samsung.c
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
// Copyright (c) 2007 Ben Dooks
// Copyright (c) 2008 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>, <ben-linux@fluff.org>
// Copyright (c) 2013 Tomasz Figa <tomasz.figa@gmail.com>
// Copyright (c) 2017 Samsung Electronics Co., Ltd.
//
// PWM driver for Samsung SoCs
//

// For struct samsung_timer_variant and samsung_pwm_lock.

pub const REG_TCFG0: c_uint = 0x00;
pub const REG_TCFG1: c_uint = 0x04;
pub const REG_TCON: c_uint = 0x08;

pub const TCFG0_PRESCALER_MASK: c_uint = 0xff;
pub const TCFG0_PRESCALER1_SHIFT: c_int = 8;
pub const TCFG1_MUX_MASK: c_uint = 0xf;

//
// Each channel occupies 4 bits in TCON register, but there is a gap of 4
// bits (one channel) after channel 0, so channels have different numbering
// when accessing TCON register. See to_tcon_channel() function.
//
// In addition, the location of autoreload bit for channel 4 (TCON channel 5)
// in its set of bits is 2 as opposed to 3 for other channels.
//

    ((chan < 5) ? _TCON_AUTORELOAD(chan) : _TCON_AUTORELOAD4(chan))
//
// struct samsung_pwm_channel - private data of PWM channel
// @period_ns:	current period in nanoseconds programmed to the hardware
// @duty_ns:	current duty time in nanoseconds programmed to the hardware
// @tin_ns:	time of one timer tick in nanoseconds with current timer rate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_pwm_channel {
    pub period_ns: u32,
    pub duty_ns: u32,
    pub tin_ns: u32,
}

//
// struct samsung_pwm_chip - private data of PWM chip
// @variant:		local copy of hardware variant data
// @inverter_mask:	inverter status for all channels - one bit per channel
// @disabled_mask:	disabled status for all channels - one bit per channel
// @base:		base address of mapped PWM registers
// @base_clk:		base clock used to drive the timers
// @tclk0:		external clock 0 (can be ERR_PTR if not present)
// @tclk1:		external clock 1 (can be ERR_PTR if not present)
// @channel:		per channel driver data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_pwm_chip {
    pub variant: samsung_pwm_variant,
    pub inverter_mask: u8,
    pub disabled_mask: u8,
    pub base: *mut void __iomem,
    pub base_clk: *mut clk,
    pub tclk0: *mut clk,
    pub tclk1: *mut clk,
    pub channel: [samsung_pwm_channel; SAMSUNG_PWM_NUM],
}

//
// PWM block is shared between pwm-samsung and samsung_pwm_timer drivers
// and some registers need access synchronization. If both drivers are
// compiled in, the spinlock is defined in the clocksource driver,
// otherwise following definition is used.
//
// Currently we do not need any more complex synchronization method
// because all the supported SoCs contain only one instance of the PWM
// IP. Should this change, both drivers will need to be modified to
// properly synchronize accesses to particular instances.
//
    static DEFINE_RAW_SPINLOCK(samsung_pwm_lock);

    static inline
    struct samsung_pwm_chip *to_samsung_pwm_chip(struct pwm_chip *chip)
    {
    return pwmchip_get_drvdata(chip);
    }
#[no_mangle]
pub unsafe extern "C" fn to_tcon_channel(channel: c_uint) -> c_uint {
    static inline unsigned int to_tcon_channel(unsigned int channel)
    {
// TCON register has a gap of 4 bits (1 channel) after channel 0
    return (channel == 0) ? 0 : (channel + 1);
    }
    static void __pwm_samsung_manual_update(struct samsung_pwm_chip *our_chip,
    struct pwm_device *pwm)
    {
    let mut tcon_chan: c_uint = to_tcon_channel(pwm.hwpwm);
    u32 tcon;
    tcon = readl(our_chip.base + REG_TCON);
    tcon |= TCON_MANUALUPDATE(tcon_chan);
    writel(tcon, our_chip.base + REG_TCON);
    tcon &= ~TCON_MANUALUPDATE(tcon_chan);
    writel(tcon, our_chip.base + REG_TCON);
    }
    static void pwm_samsung_set_divisor(struct samsung_pwm_chip *our_chip,
    unsigned int channel, u8 divisor)
    {
    let mut shift: u8 = TCFG1_SHIFT(channel);
    unsigned long flags;
    u32 reg;
    u8 bits;
    bits = (fls(divisor) - 1) - our_chip.variant.div_base;
    raw_spin_lock_irqsave(&samsung_pwm_lock, flags);
    reg = readl(our_chip.base + REG_TCFG1);
    reg &= ~(TCFG1_MUX_MASK << shift);
    reg |= bits << shift;
    writel(reg, our_chip.base + REG_TCFG1);
    raw_spin_unlock_irqrestore(&samsung_pwm_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn pwm_samsung_is_tdiv(our_chip: *mut samsung_pwm_chip, chan: c_uint) -> c_int {
    static int pwm_samsung_is_tdiv(struct samsung_pwm_chip *our_chip, unsigned int chan)
    {
    struct samsung_pwm_variant *variant = &our_chip.variant;
    u32 reg;
    reg = readl(our_chip.base + REG_TCFG1);
    reg >>= TCFG1_SHIFT(chan);
    reg &= TCFG1_MUX_MASK;
    return (BIT(reg) & variant.tclk_mask) == 0;
    }
    static unsigned long pwm_samsung_get_tin_rate(struct samsung_pwm_chip *our_chip,
    unsigned int chan)
    {
    unsigned long rate;
    u32 reg;
    rate = clk_get_rate(our_chip.base_clk);
    reg = readl(our_chip.base + REG_TCFG0);
    if (chan >= 2)
    reg >>= TCFG0_PRESCALER1_SHIFT;
    reg &= TCFG0_PRESCALER_MASK;
    return rate / (reg + 1);
    }
    static unsigned long pwm_samsung_calc_tin(struct pwm_chip *chip,
    unsigned int chan, unsigned long freq)
    {
    struct samsung_pwm_chip *our_chip = to_samsung_pwm_chip(chip);
    struct samsung_pwm_variant *variant = &our_chip.variant;
    unsigned long rate;
    struct clk *clk;
    u8 div;
    if (!pwm_samsung_is_tdiv(our_chip, chan)) {
    clk = (chan < 2) ? our_chip.tclk0 : our_chip.tclk1;
    if (!IS_ERR(clk)) {
    rate = clk_get_rate(clk);
    if (rate)
    return rate;
    }
    dev_warn(pwmchip_parent(chip),
    "tclk of PWM %d is inoperational, using tdiv\n", chan);
    }
    rate = pwm_samsung_get_tin_rate(our_chip, chan);
    dev_dbg(pwmchip_parent(chip), "tin parent at %lu\n", rate);
//
// Compare minimum PWM frequency that can be achieved with possible
// divider settings and choose the lowest divisor that can generate
// frequencies lower than requested.
//
    if (variant.bits < 32) {
// Only for s3c24xx
    for (div = variant.div_base; div < 4; ++div)
    if ((rate >> (variant.bits + div)) < freq)
    break;
    } else {
//
// Other variants have enough counter bits to generate any
// requested rate, so no need to check higher divisors.
//
    div = variant.div_base;
    }
    pwm_samsung_set_divisor(our_chip, chan, BIT(div));
    return rate >> div;
    }
#[no_mangle]
unsafe extern "C" fn pwm_samsung_request(chip: *mut pwm_chip, pwm: *mut pwm_device) -> c_int {
    static int pwm_samsung_request(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct samsung_pwm_chip *our_chip = to_samsung_pwm_chip(chip);
    if (!(our_chip.variant.output_mask & BIT(pwm.hwpwm))) {
    dev_warn(pwmchip_parent(chip),
    "tried to request PWM channel %d without output\n",
    pwm.hwpwm);
    return -EINVAL;
    }
    memset(&our_chip.channel[pwm.hwpwm], 0, sizeof(our_chip.channel[pwm.hwpwm]));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwm_samsung_enable(chip: *mut pwm_chip, pwm: *mut pwm_device) -> c_int {
    static int pwm_samsung_enable(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct samsung_pwm_chip *our_chip = to_samsung_pwm_chip(chip);
    let mut tcon_chan: c_uint = to_tcon_channel(pwm.hwpwm);
    unsigned long flags;
    u32 tcon;
    raw_spin_lock_irqsave(&samsung_pwm_lock, flags);
    tcon = readl(our_chip.base + REG_TCON);
    tcon &= ~TCON_START(tcon_chan);
    tcon |= TCON_MANUALUPDATE(tcon_chan);
    writel(tcon, our_chip.base + REG_TCON);
    tcon &= ~TCON_MANUALUPDATE(tcon_chan);
    tcon |= TCON_START(tcon_chan) | TCON_AUTORELOAD(tcon_chan);
    writel(tcon, our_chip.base + REG_TCON);
    our_chip.disabled_mask &= ~BIT(pwm.hwpwm);
    raw_spin_unlock_irqrestore(&samsung_pwm_lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwm_samsung_disable(chip: *mut pwm_chip, pwm: *mut pwm_device) {
    static void pwm_samsung_disable(struct pwm_chip *chip, struct pwm_device *pwm)
    {
    struct samsung_pwm_chip *our_chip = to_samsung_pwm_chip(chip);
    let mut tcon_chan: c_uint = to_tcon_channel(pwm.hwpwm);
    unsigned long flags;
    u32 tcon;
    raw_spin_lock_irqsave(&samsung_pwm_lock, flags);
    tcon = readl(our_chip.base + REG_TCON);
    tcon &= ~TCON_AUTORELOAD(tcon_chan);
    writel(tcon, our_chip.base + REG_TCON);
//
// In case the PWM is at 100% duty cycle, force a manual
// update to prevent the signal from staying high.
//
    if (readl(our_chip.base + REG_TCMPB(pwm.hwpwm)) == (u32)-1U)
    __pwm_samsung_manual_update(our_chip, pwm);
    our_chip.disabled_mask |= BIT(pwm.hwpwm);
    raw_spin_unlock_irqrestore(&samsung_pwm_lock, flags);
    }
    static void pwm_samsung_manual_update(struct samsung_pwm_chip *our_chip,
    struct pwm_device *pwm)
    {
    unsigned long flags;
    raw_spin_lock_irqsave(&samsung_pwm_lock, flags);
    __pwm_samsung_manual_update(our_chip, pwm);
    raw_spin_unlock_irqrestore(&samsung_pwm_lock, flags);
    }
    static int __pwm_samsung_config(struct pwm_chip *chip, struct pwm_device *pwm,
    int duty_ns, int period_ns, bool force_period)
    {
    struct samsung_pwm_chip *our_chip = to_samsung_pwm_chip(chip);
    struct samsung_pwm_channel *chan = &our_chip.channel[pwm.hwpwm];
    let mut tin_ns: u32 = chan.tin_ns, tcnt, tcmp, oldtcmp;
    tcnt = readl(our_chip.base + REG_TCNTB(pwm.hwpwm));
    oldtcmp = readl(our_chip.base + REG_TCMPB(pwm.hwpwm));
// We need tick count for calculation, not last tick.
    ++tcnt;
// Check to see if we are changing the clock rate of the PWM.
    if (chan.period_ns != period_ns || force_period) {
    unsigned long tin_rate;
    u32 period;
    period = NSEC_PER_SEC / period_ns;
    dev_dbg(pwmchip_parent(chip), "duty_ns=%d, period_ns=%d (%u)\n",
    duty_ns, period_ns, period);
    tin_rate = pwm_samsung_calc_tin(chip, pwm.hwpwm, period);
    dev_dbg(pwmchip_parent(chip), "tin_rate=%lu\n", tin_rate);
    tin_ns = NSEC_PER_SEC / tin_rate;
    tcnt = period_ns / tin_ns;
    }
// Period is too short.
    if (tcnt <= 1)
    return -ERANGE;
// Note that counters count down.
    tcmp = duty_ns / tin_ns;
// 0% duty is not available
    if (!tcmp)
    ++tcmp;
    tcmp = tcnt - tcmp;
// Decrement to get tick numbers, instead of tick counts.
    --tcnt;
// -1UL will give 100% duty.
    --tcmp;
    dev_dbg(pwmchip_parent(chip), "tin_ns=%u, tcmp=%u/%u\n", tin_ns, tcmp, tcnt);
// Update PWM registers.
    writel(tcnt, our_chip.base + REG_TCNTB(pwm.hwpwm));
    writel(tcmp, our_chip.base + REG_TCMPB(pwm.hwpwm));
//
// In case the PWM is currently at 100% duty cycle, force a manual
// update to prevent the signal staying high if the PWM is disabled
// shortly afer this update (before it autoreloaded the new values).
//
    if (oldtcmp == (u32) -1) {
    dev_dbg(pwmchip_parent(chip), "Forcing manual update");
    pwm_samsung_manual_update(our_chip, pwm);
    }
    chan.period_ns = period_ns;
    chan.tin_ns = tin_ns;
    chan.duty_ns = duty_ns;
    return 0;
    }
    static int pwm_samsung_config(struct pwm_chip *chip, struct pwm_device *pwm,
    int duty_ns, int period_ns)
    {
    return __pwm_samsung_config(chip, pwm, duty_ns, period_ns, false);
    }
    static void pwm_samsung_set_invert(struct samsung_pwm_chip *our_chip,
    unsigned int channel, bool invert)
    {
    let mut tcon_chan: c_uint = to_tcon_channel(channel);
    unsigned long flags;
    u32 tcon;
    raw_spin_lock_irqsave(&samsung_pwm_lock, flags);
    tcon = readl(our_chip.base + REG_TCON);
    if (invert) {
    our_chip.inverter_mask |= BIT(channel);
    tcon |= TCON_INVERT(tcon_chan);
    } else {
    our_chip.inverter_mask &= ~BIT(channel);
    tcon &= ~TCON_INVERT(tcon_chan);
    }
    writel(tcon, our_chip.base + REG_TCON);
    raw_spin_unlock_irqrestore(&samsung_pwm_lock, flags);
    }
    static int pwm_samsung_set_polarity(struct pwm_chip *chip,
    struct pwm_device *pwm,
    enum pwm_polarity polarity)
    {
    struct samsung_pwm_chip *our_chip = to_samsung_pwm_chip(chip);
    let mut invert: bool = (polarity == PWM_POLARITY_NORMAL);
// Inverted means normal in the hardware.
    pwm_samsung_set_invert(our_chip, pwm.hwpwm, invert);
    return 0;
    }
    static int pwm_samsung_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    int err, enabled = pwm.state.enabled;
    if (state.polarity != pwm.state.polarity) {
    if (enabled) {
    pwm_samsung_disable(chip, pwm);
    enabled = false;
    }
    err = pwm_samsung_set_polarity(chip, pwm, state.polarity);
    if (err)
    return err;
    }
    if (!state.enabled) {
    if (enabled)
    pwm_samsung_disable(chip, pwm);
    return 0;
    }
//
// We currently avoid using 64bit arithmetic by using the
// fact that anything faster than 1Hz is easily representable
// by 32bits.
//
    if (state.period > NSEC_PER_SEC)
    return -ERANGE;
    err = pwm_samsung_config(chip, pwm, state.duty_cycle, state.period);
    if (err)
    return err;
    if (!pwm.state.enabled)
    err = pwm_samsung_enable(chip, pwm);
    return err;
    }
    static const struct pwm_ops pwm_samsung_ops = {
    .request	= pwm_samsung_request,
    .apply		= pwm_samsung_apply,
    };

    static const struct samsung_pwm_variant s3c24xx_variant = {
    .bits		= 16,
    .div_base	= 1,
    .has_tint_cstat	= false,
    .tclk_mask	= BIT(4),
    };
    static const struct samsung_pwm_variant s3c64xx_variant = {
    .bits		= 32,
    .div_base	= 0,
    .has_tint_cstat	= true,
    .tclk_mask	= BIT(7) | BIT(6) | BIT(5),
    };
    static const struct samsung_pwm_variant s5p64x0_variant = {
    .bits		= 32,
    .div_base	= 0,
    .has_tint_cstat	= true,
    .tclk_mask	= 0,
    };
    static const struct samsung_pwm_variant s5pc100_variant = {
    .bits		= 32,
    .div_base	= 0,
    .has_tint_cstat	= true,
    .tclk_mask	= BIT(5),
    };
    static const struct of_device_id samsung_pwm_matches[] = {
    { .compatible = "samsung,s3c2410-pwm", .data = &s3c24xx_variant },
    { .compatible = "samsung,s3c6400-pwm", .data = &s3c64xx_variant },
    { .compatible = "samsung,s5p6440-pwm", .data = &s5p64x0_variant },
    { .compatible = "samsung,s5pc100-pwm", .data = &s5pc100_variant },
    { .compatible = "samsung,exynos4210-pwm", .data = &s5p64x0_variant },
    { }
    };
    MODULE_DEVICE_TABLE(of, samsung_pwm_matches);
#[no_mangle]
unsafe extern "C" fn pwm_samsung_parse_dt(chip: *mut pwm_chip) -> c_int {
    static int pwm_samsung_parse_dt(struct pwm_chip *chip)
    {
    struct samsung_pwm_chip *our_chip = to_samsung_pwm_chip(chip);
    struct device_node *np = pwmchip_parent(chip).of_node;
    const struct of_device_id *match;
    u32 val;
    match = of_match_node(samsung_pwm_matches, np);
    if (!match)
    return -ENODEV;
    memcpy(&our_chip.variant, match.data, sizeof(our_chip.variant));
    of_property_for_each_u32(np, "samsung,pwm-outputs", val) {
    if (val >= SAMSUNG_PWM_NUM) {
    dev_err(pwmchip_parent(chip),
    "%s: invalid channel index in samsung,pwm-outputs property\n",
    __func__);
    continue;
    }
    our_chip.variant.output_mask |= BIT(val);
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn pwm_samsung_parse_dt(chip: *mut pwm_chip) -> c_int {
    static int pwm_samsung_parse_dt(struct pwm_chip *chip)
    {
    return -ENODEV;
    }

#[no_mangle]
unsafe extern "C" fn pwm_samsung_probe(pdev: *mut platform_device) -> c_int {
    static int pwm_samsung_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct samsung_pwm_chip *our_chip;
    struct pwm_chip *chip;
    unsigned int chan;
    int ret;
    chip = devm_pwmchip_alloc(&pdev.dev, SAMSUNG_PWM_NUM, sizeof(*our_chip));
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    our_chip = to_samsung_pwm_chip(chip);
    chip.ops = &pwm_samsung_ops;
    our_chip.inverter_mask = BIT(SAMSUNG_PWM_NUM) - 1;
    if (IS_ENABLED(CONFIG_OF) && pdev.dev.of_node) {
    ret = pwm_samsung_parse_dt(chip);
    if (ret)
    return ret;
    } else {
    if (!pdev.dev.platform_data)
    return dev_err_probe(&pdev.dev, -EINVAL,
    "no platform data specified\n");
    memcpy(&our_chip.variant, pdev.dev.platform_data,
    sizeof(our_chip.variant));
    }
    our_chip.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(our_chip.base))
    return PTR_ERR(our_chip.base);
    our_chip.base_clk = devm_clk_get_enabled(&pdev.dev, "timers");
    if (IS_ERR(our_chip.base_clk))
    return dev_err_probe(dev, PTR_ERR(our_chip.base_clk),
    "failed to get timer base clk\n");
    for (chan = 0; chan < SAMSUNG_PWM_NUM; ++chan)
    if (our_chip.variant.output_mask & BIT(chan))
    pwm_samsung_set_invert(our_chip, chan, true);
// Following clocks are optional.
    our_chip.tclk0 = devm_clk_get(&pdev.dev, "pwm-tclk0");
    our_chip.tclk1 = devm_clk_get(&pdev.dev, "pwm-tclk1");
    platform_set_drvdata(pdev, chip);
    ret = devm_pwmchip_add(&pdev.dev, chip);
    if (ret < 0)
    return dev_err_probe(dev, ret, "failed to register PWM chip\n");
    dev_dbg(dev, "base_clk at %lu, tclk0 at %lu, tclk1 at %lu\n",
    clk_get_rate(our_chip.base_clk),
    !IS_ERR(our_chip.tclk0) ? clk_get_rate(our_chip.tclk0) : 0,
    !IS_ERR(our_chip.tclk1) ? clk_get_rate(our_chip.tclk1) : 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwm_samsung_resume(dev: *mut device) -> c_int {
    static int pwm_samsung_resume(struct device *dev)
    {
    struct pwm_chip *chip = dev_get_drvdata(dev);
    struct samsung_pwm_chip *our_chip = to_samsung_pwm_chip(chip);
    unsigned int i;
    for (i = 0; i < SAMSUNG_PWM_NUM; i++) {
    struct pwm_device *pwm = &chip.pwms[i];
    struct samsung_pwm_channel *chan = &our_chip.channel[i];
    if (!test_bit(PWMF_REQUESTED, &pwm.flags))
    continue;
    if (our_chip.variant.output_mask & BIT(i))
    pwm_samsung_set_invert(our_chip, i,
    our_chip.inverter_mask & BIT(i));
    if (chan.period_ns) {
    __pwm_samsung_config(chip, pwm, chan.duty_ns,
    chan.period_ns, true);
// needed to make PWM disable work on Odroid-XU3
    pwm_samsung_manual_update(our_chip, pwm);
    }
    if (our_chip.disabled_mask & BIT(i))
    pwm_samsung_disable(chip, pwm);
    else
    pwm_samsung_enable(chip, pwm);
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(pwm_samsung_pm_ops, core::ptr::null_mut(), pwm_samsung_resume);
    static struct platform_driver pwm_samsung_driver = {
    .driver		= {
    .name	= "samsung-pwm",
    .pm	= pm_ptr(&pwm_samsung_pm_ops),
    .of_match_table = of_match_ptr(samsung_pwm_matches),
    },
    .probe		= pwm_samsung_probe,
    };
    module_platform_driver(pwm_samsung_driver);
    MODULE_DESCRIPTION("Samsung Pulse Width Modulator driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Tomasz Figa <tomasz.figa@gmail.com>");
    MODULE_ALIAS("platform:samsung-pwm");

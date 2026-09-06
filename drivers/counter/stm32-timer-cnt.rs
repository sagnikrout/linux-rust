//! Automatically rewritten from C to Rust
//! Source: drivers/counter/stm32-timer-cnt.c
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
// STM32 Timer Encoder and Counter driver
//
// Copyright (C) STMicroelectronics 2018
//
// Author: Benjamin Gaignard <benjamin.gaignard@st.com>
//

    TIM_CCMR_IC1F | TIM_CCMR_IC2F)

    TIM_CCER_CC2P | TIM_CCER_CC2NP)
pub const STM32_CH1_SIG: c_int = 0;
pub const STM32_CH2_SIG: c_int = 1;
pub const STM32_CLOCK_SIG: c_int = 2;
pub const STM32_CH3_SIG: c_int = 3;
pub const STM32_CH4_SIG: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_timer_regs {
    pub cr1: u32,
    pub cnt: u32,
    pub smcr: u32,
    pub arr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_timer_cnt {
    pub regmap: *mut regmap,
    pub clk: *mut clk,
    pub max_arr: u32,
    pub enabled: bool,
    pub bak: stm32_timer_regs,
    pub has_encoder: bool,
    pub nchannels: c_uint,
    pub nr_irqs: c_uint,
    pub /: *mut *mut spinlock_t lock; / protects nb_ovf,
    pub nb_ovf: u64,
}

    static const enum counter_function stm32_count_functions[] = {
    COUNTER_FUNCTION_INCREASE,
    COUNTER_FUNCTION_QUADRATURE_X2_A,
    COUNTER_FUNCTION_QUADRATURE_X2_B,
    COUNTER_FUNCTION_QUADRATURE_X4,
    };
    static int stm32_count_read(struct counter_device *counter,
    struct counter_count *count, u64 *val)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    u32 cnt;
    regmap_read(priv.regmap, TIM_CNT, &cnt);
// val = cnt;
    return 0;
    }
    static int stm32_count_write(struct counter_device *counter,
    struct counter_count *count, const u64 val)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    u32 ceiling;
    regmap_read(priv.regmap, TIM_ARR, &ceiling);
    if (val > ceiling)
    return -EINVAL;
    return regmap_write(priv.regmap, TIM_CNT, val);
    }
    static int stm32_count_function_read(struct counter_device *counter,
    struct counter_count *count,
    enum counter_function *function)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    u32 smcr;
    regmap_read(priv.regmap, TIM_SMCR, &smcr);
    switch (smcr & TIM_SMCR_SMS) {
    case TIM_SMCR_SMS_SLAVE_MODE_DISABLED:
// function = COUNTER_FUNCTION_INCREASE;
    return 0;
    case TIM_SMCR_SMS_ENCODER_MODE_1:
// function = COUNTER_FUNCTION_QUADRATURE_X2_A;
    return 0;
    case TIM_SMCR_SMS_ENCODER_MODE_2:
// function = COUNTER_FUNCTION_QUADRATURE_X2_B;
    return 0;
    case TIM_SMCR_SMS_ENCODER_MODE_3:
// function = COUNTER_FUNCTION_QUADRATURE_X4;
    return 0;
    default:
    return -EINVAL;
    }
    }
    static int stm32_count_function_write(struct counter_device *counter,
    struct counter_count *count,
    enum counter_function function)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    u32 cr1, sms;
    switch (function) {
    case COUNTER_FUNCTION_INCREASE:
    sms = TIM_SMCR_SMS_SLAVE_MODE_DISABLED;
    break;
    case COUNTER_FUNCTION_QUADRATURE_X2_A:
    if (!priv.has_encoder)
    return -EOPNOTSUPP;
    sms = TIM_SMCR_SMS_ENCODER_MODE_1;
    break;
    case COUNTER_FUNCTION_QUADRATURE_X2_B:
    if (!priv.has_encoder)
    return -EOPNOTSUPP;
    sms = TIM_SMCR_SMS_ENCODER_MODE_2;
    break;
    case COUNTER_FUNCTION_QUADRATURE_X4:
    if (!priv.has_encoder)
    return -EOPNOTSUPP;
    sms = TIM_SMCR_SMS_ENCODER_MODE_3;
    break;
    default:
    return -EINVAL;
    }
// Store enable status
    regmap_read(priv.regmap, TIM_CR1, &cr1);
    regmap_update_bits(priv.regmap, TIM_CR1, TIM_CR1_CEN, 0);
    regmap_update_bits(priv.regmap, TIM_SMCR, TIM_SMCR_SMS, sms);
// Make sure that registers are updated
    regmap_update_bits(priv.regmap, TIM_EGR, TIM_EGR_UG, TIM_EGR_UG);
// Restore the enable status
    regmap_update_bits(priv.regmap, TIM_CR1, TIM_CR1_CEN, cr1);
    return 0;
    }
    static int stm32_count_direction_read(struct counter_device *counter,
    struct counter_count *count,
    enum counter_count_direction *direction)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    u32 cr1;
    regmap_read(priv.regmap, TIM_CR1, &cr1);
// direction = (cr1 & TIM_CR1_DIR) ? COUNTER_COUNT_DIRECTION_BACKWARD :
    COUNTER_COUNT_DIRECTION_FORWARD;
    return 0;
    }
    static int stm32_count_ceiling_read(struct counter_device *counter,
    struct counter_count *count, u64 *ceiling)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    u32 arr;
    regmap_read(priv.regmap, TIM_ARR, &arr);
// ceiling = arr;
    return 0;
    }
    static int stm32_count_ceiling_write(struct counter_device *counter,
    struct counter_count *count, u64 ceiling)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    if (ceiling > priv.max_arr)
    return -ERANGE;
// TIMx_ARR register shouldn't be buffered (ARPE=0)
    regmap_update_bits(priv.regmap, TIM_CR1, TIM_CR1_ARPE, 0);
    regmap_write(priv.regmap, TIM_ARR, ceiling);
    return 0;
    }
    static int stm32_count_enable_read(struct counter_device *counter,
    struct counter_count *count, u8 *enable)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    u32 cr1;
    regmap_read(priv.regmap, TIM_CR1, &cr1);
// enable = cr1 & TIM_CR1_CEN;
    return 0;
    }
    static int stm32_count_enable_write(struct counter_device *counter,
    struct counter_count *count, u8 enable)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    u32 cr1;
    int ret;
    if (enable) {
    regmap_read(priv.regmap, TIM_CR1, &cr1);
    if (!(cr1 & TIM_CR1_CEN)) {
    ret = clk_enable(priv.clk);
    if (ret) {
    dev_err(counter.parent, "Cannot enable clock %d\n", ret);
    return ret;
    }
    }
    regmap_update_bits(priv.regmap, TIM_CR1, TIM_CR1_CEN,
    TIM_CR1_CEN);
    } else {
    regmap_read(priv.regmap, TIM_CR1, &cr1);
    regmap_update_bits(priv.regmap, TIM_CR1, TIM_CR1_CEN, 0);
    if (cr1 & TIM_CR1_CEN)
    clk_disable(priv.clk);
    }
// Keep enabled state to properly handle low power states
    priv.enabled = enable;
    return 0;
    }
    static int stm32_count_prescaler_read(struct counter_device *counter,
    struct counter_count *count, u64 *prescaler)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    u32 psc;
    regmap_read(priv.regmap, TIM_PSC, &psc);
// prescaler = psc + 1;
    return 0;
    }
    static int stm32_count_prescaler_write(struct counter_device *counter,
    struct counter_count *count, u64 prescaler)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    u32 psc;
    if (!prescaler || prescaler > MAX_TIM_PSC + 1)
    return -ERANGE;
    psc = prescaler - 1;
    return regmap_write(priv.regmap, TIM_PSC, psc);
    }
    static int stm32_count_cap_read(struct counter_device *counter,
    struct counter_count *count,
    size_t ch, u64 *cap)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    u32 ccrx;
    if (ch >= priv.nchannels)
    return -EOPNOTSUPP;
    switch (ch) {
    case 0:
    regmap_read(priv.regmap, TIM_CCR1, &ccrx);
    break;
    case 1:
    regmap_read(priv.regmap, TIM_CCR2, &ccrx);
    break;
    case 2:
    regmap_read(priv.regmap, TIM_CCR3, &ccrx);
    break;
    case 3:
    regmap_read(priv.regmap, TIM_CCR4, &ccrx);
    break;
    default:
    return -EINVAL;
    }
    dev_dbg(counter.parent, "CCR%zu: 0x%08x\n", ch + 1, ccrx);
// cap = ccrx;
    return 0;
    }
    static int stm32_count_nb_ovf_read(struct counter_device *counter,
    struct counter_count *count, u64 *val)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    unsigned long irqflags;
    spin_lock_irqsave(&priv.lock, irqflags);
// val = priv->nb_ovf;
    spin_unlock_irqrestore(&priv.lock, irqflags);
    return 0;
    }
    static int stm32_count_nb_ovf_write(struct counter_device *counter,
    struct counter_count *count, u64 val)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    unsigned long irqflags;
    spin_lock_irqsave(&priv.lock, irqflags);
    priv.nb_ovf = val;
    spin_unlock_irqrestore(&priv.lock, irqflags);
    return 0;
    }
    static DEFINE_COUNTER_ARRAY_CAPTURE(stm32_count_cap_array, 4);
    static struct counter_comp stm32_count_ext[] = {
    COUNTER_COMP_DIRECTION(stm32_count_direction_read),
    COUNTER_COMP_ENABLE(stm32_count_enable_read, stm32_count_enable_write),
    COUNTER_COMP_CEILING(stm32_count_ceiling_read,
    stm32_count_ceiling_write),
    COUNTER_COMP_COUNT_U64("prescaler", stm32_count_prescaler_read,
    stm32_count_prescaler_write),
    COUNTER_COMP_ARRAY_CAPTURE(stm32_count_cap_read, core::ptr::null_mut(), stm32_count_cap_array),
    COUNTER_COMP_COUNT_U64("num_overflows", stm32_count_nb_ovf_read, stm32_count_nb_ovf_write),
    };
    static const enum counter_synapse_action stm32_clock_synapse_actions[] = {
    COUNTER_SYNAPSE_ACTION_RISING_EDGE,
    };
    static const enum counter_synapse_action stm32_synapse_actions[] = {
    COUNTER_SYNAPSE_ACTION_NONE,
    COUNTER_SYNAPSE_ACTION_BOTH_EDGES
    };
    static int stm32_action_read(struct counter_device *counter,
    struct counter_count *count,
    struct counter_synapse *synapse,
    enum counter_synapse_action *action)
    {
    enum counter_function function;
    int err;
    err = stm32_count_function_read(counter, count, &function);
    if (err)
    return err;
    switch (function) {
    case COUNTER_FUNCTION_INCREASE:
// counts on internal clock when CEN=1
    if (synapse.signal.id == STM32_CLOCK_SIG)
// action = COUNTER_SYNAPSE_ACTION_RISING_EDGE;
    else
// action = COUNTER_SYNAPSE_ACTION_NONE;
    return 0;
    case COUNTER_FUNCTION_QUADRATURE_X2_A:
// counts up/down on TI1FP1 edge depending on TI2FP2 level
    if (synapse.signal.id == STM32_CH1_SIG)
// action = COUNTER_SYNAPSE_ACTION_BOTH_EDGES;
    else
// action = COUNTER_SYNAPSE_ACTION_NONE;
    return 0;
    case COUNTER_FUNCTION_QUADRATURE_X2_B:
// counts up/down on TI2FP2 edge depending on TI1FP1 level
    if (synapse.signal.id == STM32_CH2_SIG)
// action = COUNTER_SYNAPSE_ACTION_BOTH_EDGES;
    else
// action = COUNTER_SYNAPSE_ACTION_NONE;
    return 0;
    case COUNTER_FUNCTION_QUADRATURE_X4:
// counts up/down on both TI1FP1 and TI2FP2 edges
    if (synapse.signal.id == STM32_CH1_SIG || synapse.signal.id == STM32_CH2_SIG)
// action = COUNTER_SYNAPSE_ACTION_BOTH_EDGES;
    else
// action = COUNTER_SYNAPSE_ACTION_NONE;
    return 0;
    default:
    return -EINVAL;
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_count_cc_regs {
    pub ccmr_reg: u32,
    pub ccmr_mask: u32,
    pub ccmr_bits: u32,
    pub ccer_bits: u32,
}

    static const struct stm32_count_cc_regs stm32_cc[] = {
    { TIM_CCMR1, TIM_CCMR_CC1S, TIM_CCMR_CC1S_TI1,
    TIM_CCER_CC1E | TIM_CCER_CC1P | TIM_CCER_CC1NP },
    { TIM_CCMR1, TIM_CCMR_CC2S, TIM_CCMR_CC2S_TI2,
    TIM_CCER_CC2E | TIM_CCER_CC2P | TIM_CCER_CC2NP },
    { TIM_CCMR2, TIM_CCMR_CC3S, TIM_CCMR_CC3S_TI3,
    TIM_CCER_CC3E | TIM_CCER_CC3P | TIM_CCER_CC3NP },
    { TIM_CCMR2, TIM_CCMR_CC4S, TIM_CCMR_CC4S_TI4,
    TIM_CCER_CC4E | TIM_CCER_CC4P | TIM_CCER_CC4NP },
    };
    static int stm32_count_capture_configure(struct counter_device *counter, unsigned int ch,
    bool enable)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    const struct stm32_count_cc_regs *cc;
    u32 ccmr, ccer;
    if (ch >= ARRAY_SIZE(stm32_cc) || ch >= priv.nchannels) {
    dev_err(counter.parent, "invalid ch: %d\n", ch);
    return -EINVAL;
    }
    cc = &stm32_cc[ch];
//
// configure channel in input capture mode, map channel 1 on TI1, channel2 on TI2...
// Select both edges / non-inverted to trigger a capture.
//
    if (enable) {
// first clear possibly latched capture flag upon enabling
    if (!regmap_test_bits(priv.regmap, TIM_CCER, cc.ccer_bits))
    regmap_write(priv.regmap, TIM_SR, ~TIM_SR_CC_IF(ch));
    regmap_update_bits(priv.regmap, cc.ccmr_reg, cc.ccmr_mask,
    cc.ccmr_bits);
    regmap_set_bits(priv.regmap, TIM_CCER, cc.ccer_bits);
    } else {
    regmap_clear_bits(priv.regmap, TIM_CCER, cc.ccer_bits);
    regmap_clear_bits(priv.regmap, cc.ccmr_reg, cc.ccmr_mask);
    }
    regmap_read(priv.regmap, cc.ccmr_reg, &ccmr);
    regmap_read(priv.regmap, TIM_CCER, &ccer);
    dev_dbg(counter.parent, "%s(%s) ch%d 0x%08x 0x%08x\n", __func__, enable ? "ena" : "dis",
    ch, ccmr, ccer);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_count_events_configure(counter: *mut counter_device) -> c_int {
    static int stm32_count_events_configure(struct counter_device *counter)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    struct counter_event_node *event_node;
    let mut dier: u32 = 0;
    int i, ret;
    list_for_each_entry(event_node, &counter.events_list, l) {
    switch (event_node.event) {
    case COUNTER_EVENT_OVERFLOW_UNDERFLOW:
// first clear possibly latched UIF before enabling
    if (!regmap_test_bits(priv.regmap, TIM_DIER, TIM_DIER_UIE))
    regmap_write(priv.regmap, TIM_SR, (u32)~TIM_SR_UIF);
    dier |= TIM_DIER_UIE;
    break;
    case COUNTER_EVENT_CAPTURE:
    ret = stm32_count_capture_configure(counter, event_node.channel, true);
    if (ret)
    return ret;
    dier |= TIM_DIER_CCxIE(event_node.channel + 1);
    break;
    default:
// should never reach this path
    return -EINVAL;
    }
    }
// Enable / disable all events at once, from events_list, so write all DIER bits
    regmap_write(priv.regmap, TIM_DIER, dier);
// check for disabled capture events
    for (i = 0 ; i < priv.nchannels; i++) {
    if (!(dier & TIM_DIER_CCxIE(i + 1))) {
    ret = stm32_count_capture_configure(counter, i, false);
    if (ret)
    return ret;
    }
    }
    return 0;
    }
    static int stm32_count_watch_validate(struct counter_device *counter,
    const struct counter_watch *watch)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
// Interrupts are optional
    if (!priv.nr_irqs)
    return -EOPNOTSUPP;
    switch (watch.event) {
    case COUNTER_EVENT_CAPTURE:
    if (watch.channel >= priv.nchannels) {
    dev_err(counter.parent, "Invalid channel %d\n", watch.channel);
    return -EINVAL;
    }
    return 0;
    case COUNTER_EVENT_OVERFLOW_UNDERFLOW:
    return 0;
    default:
    return -EINVAL;
    }
    }
    static const struct counter_ops stm32_timer_cnt_ops = {
    .count_read = stm32_count_read,
    .count_write = stm32_count_write,
    .function_read = stm32_count_function_read,
    .function_write = stm32_count_function_write,
    .action_read = stm32_action_read,
    .events_configure = stm32_count_events_configure,
    .watch_validate = stm32_count_watch_validate,
    };
    static int stm32_count_clk_get_freq(struct counter_device *counter,
    struct counter_signal *signal, u64 *freq)
    {
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
// freq = clk_get_rate(priv->clk);
    return 0;
    }
    static struct counter_comp stm32_count_clock_ext[] = {
    COUNTER_COMP_FREQUENCY(stm32_count_clk_get_freq),
    };
    static struct counter_signal stm32_signals[] = {
//
// Need to declare all the signals as a static array, and keep the signals order here,
// even if they're unused or unexisting on some timer instances. It's an abstraction,
// e.g. high level view of the counter features.
//
// Userspace programs may rely on signal0 to be "Channel 1", signal1 to be "Channel 2",
// and so on. When a signal is unexisting, the COUNTER_SYNAPSE_ACTION_NONE can be used,
// to indicate that a signal doesn't affect the counter.
//
    {
    .id = STM32_CH1_SIG,
    .name = "Channel 1"
    },
    {
    .id = STM32_CH2_SIG,
    .name = "Channel 2"
    },
    {
    .id = STM32_CLOCK_SIG,
    .name = "Clock",
    .ext = stm32_count_clock_ext,
    .num_ext = ARRAY_SIZE(stm32_count_clock_ext),
    },
    {
    .id = STM32_CH3_SIG,
    .name = "Channel 3"
    },
    {
    .id = STM32_CH4_SIG,
    .name = "Channel 4"
    },
    };
    static struct counter_synapse stm32_count_synapses[] = {
    {
    .actions_list = stm32_synapse_actions,
    .num_actions = ARRAY_SIZE(stm32_synapse_actions),
    .signal = &stm32_signals[STM32_CH1_SIG]
    },
    {
    .actions_list = stm32_synapse_actions,
    .num_actions = ARRAY_SIZE(stm32_synapse_actions),
    .signal = &stm32_signals[STM32_CH2_SIG]
    },
    {
    .actions_list = stm32_clock_synapse_actions,
    .num_actions = ARRAY_SIZE(stm32_clock_synapse_actions),
    .signal = &stm32_signals[STM32_CLOCK_SIG]
    },
    {
    .actions_list = stm32_synapse_actions,
    .num_actions = ARRAY_SIZE(stm32_synapse_actions),
    .signal = &stm32_signals[STM32_CH3_SIG]
    },
    {
    .actions_list = stm32_synapse_actions,
    .num_actions = ARRAY_SIZE(stm32_synapse_actions),
    .signal = &stm32_signals[STM32_CH4_SIG]
    },
    };
    static struct counter_count stm32_counts = {
    .id = 0,
    .name = "STM32 Timer Counter",
    .functions_list = stm32_count_functions,
    .num_functions = ARRAY_SIZE(stm32_count_functions),
    .synapses = stm32_count_synapses,
    .num_synapses = ARRAY_SIZE(stm32_count_synapses),
    .ext = stm32_count_ext,
    .num_ext = ARRAY_SIZE(stm32_count_ext)
    };
#[no_mangle]
unsafe extern "C" fn stm32_timer_cnt_isr(irq: c_int, ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t stm32_timer_cnt_isr(int irq, void *ptr)
    {
    struct counter_device *counter = ptr;
    let mut priv: *mut stm32_timer_cnt const = counter_priv(counter);
    u32 clr = GENMASK(31, 0); /* SR flags can be cleared by writing 0 (wr 1 has no effect) */
    u32 sr, dier;
    int i;
    regmap_read(priv.regmap, TIM_SR, &sr);
    regmap_read(priv.regmap, TIM_DIER, &dier);
//
// Some status bits in SR don't match with the enable bits in DIER. Only take care of
// the possibly enabled bits in DIER (that matches in between SR and DIER).
//
    dier &= (TIM_DIER_UIE | TIM_DIER_CC1IE | TIM_DIER_CC2IE | TIM_DIER_CC3IE | TIM_DIER_CC4IE);
    sr &= dier;
    if (sr & TIM_SR_UIF) {
    spin_lock(&priv.lock);
    priv.nb_ovf++;
    spin_unlock(&priv.lock);
    counter_push_event(counter, COUNTER_EVENT_OVERFLOW_UNDERFLOW, 0);
    dev_dbg(counter.parent, "COUNTER_EVENT_OVERFLOW_UNDERFLOW\n");
// SR flags can be cleared by writing 0, only clear relevant flag
    clr &= ~TIM_SR_UIF;
    }
// Check capture events
    for (i = 0 ; i < priv.nchannels; i++) {
    if (sr & TIM_SR_CC_IF(i)) {
    counter_push_event(counter, COUNTER_EVENT_CAPTURE, i);
    clr &= ~TIM_SR_CC_IF(i);
    dev_dbg(counter.parent, "COUNTER_EVENT_CAPTURE, %d\n", i);
    }
    }
    regmap_write(priv.regmap, TIM_SR, clr);
    return IRQ_HANDLED;
    };
    static void stm32_timer_cnt_detect_channels(struct device *dev,
    struct stm32_timer_cnt *priv)
    {
    u32 ccer, ccer_backup;
    regmap_read(priv.regmap, TIM_CCER, &ccer_backup);
    regmap_set_bits(priv.regmap, TIM_CCER, TIM_CCER_CCXE);
    regmap_read(priv.regmap, TIM_CCER, &ccer);
    regmap_write(priv.regmap, TIM_CCER, ccer_backup);
    priv.nchannels = hweight32(ccer & TIM_CCER_CCXE);
    dev_dbg(dev, "has %d cc channels\n", priv.nchannels);
    }
// encoder supported on TIM1 TIM2 TIM3 TIM4 TIM5 TIM8 TIM20

    BIT(19))
    static const char * const stm32_timer_trigger_compat[] = {
    "st,stm32-timer-trigger",
    "st,stm32h7-timer-trigger",
    "st,stm32mp25-timer-trigger",
    };
    static int stm32_timer_cnt_probe_encoder(struct device *dev,
    struct stm32_timer_cnt *priv)
    {
    struct device *parent = dev.parent;
    struct device_node *tnode = core::ptr::null_mut(), *pnode = parent.of_node;
    int i, ret;
    u32 idx;
//
// Need to retrieve the trigger node index from DT, to be able
// to determine if the counter supports encoder mode. It also
// enforce backward compatibility, and allow to support other
// counter modes in this driver (when the timer doesn't support
// encoder).
//
    for (i = 0; i < ARRAY_SIZE(stm32_timer_trigger_compat) && !tnode; i++)
    tnode = of_get_compatible_child(pnode, stm32_timer_trigger_compat[i]);
    if (!tnode) {
    dev_err(dev, "Can't find trigger node\n");
    return -ENODATA;
    }
    ret = of_property_read_u32(tnode, "reg", &idx);
    of_node_put(tnode);
    if (ret) {
    dev_err(dev, "Can't get index (%d)\n", ret);
    return ret;
    }
    priv.has_encoder = !!(STM32_TIM_ENCODER_SUPPORTED & BIT(idx));
    dev_dbg(dev, "encoder support: %s\n", priv.has_encoder ? "yes" : "no");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_timer_cnt_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_timer_cnt_probe(struct platform_device *pdev)
    {
    struct stm32_timers *ddata = dev_get_drvdata(pdev.dev.parent);
    struct device *dev = &pdev.dev;
    struct stm32_timer_cnt *priv;
    struct counter_device *counter;
    int i, ret;
    if (IS_ERR_OR_NULL(ddata))
    return -EINVAL;
    counter = devm_counter_alloc(dev, sizeof(*priv));
    if (!counter)
    return -ENOMEM;
    priv = counter_priv(counter);
    priv.regmap = ddata.regmap;
    priv.clk = ddata.clk;
    priv.max_arr = ddata.max_arr;
    priv.nr_irqs = ddata.nr_irqs;
    ret = stm32_timer_cnt_probe_encoder(dev, priv);
    if (ret)
    return ret;
    stm32_timer_cnt_detect_channels(dev, priv);
    counter.name = dev_name(dev);
    counter.parent = dev;
    counter.ops = &stm32_timer_cnt_ops;
    counter.counts = &stm32_counts;
    counter.num_counts = 1;
    counter.signals = stm32_signals;
    counter.num_signals = ARRAY_SIZE(stm32_signals);
    spin_lock_init(&priv.lock);
    platform_set_drvdata(pdev, priv);
// STM32 Timers can have either 1 global, or 4 dedicated interrupts (optional)
    if (priv.nr_irqs == 1) {
// All events reported through the global interrupt
    ret = devm_request_irq(&pdev.dev, ddata.irq[0], stm32_timer_cnt_isr,
    0, dev_name(dev), counter);
    if (ret)
    return ret;
    } else {
    for (i = 0; i < priv.nr_irqs; i++) {
//
// Only take care of update IRQ for overflow events, and cc for
// capture events.
//
    if (i != STM32_TIMERS_IRQ_UP && i != STM32_TIMERS_IRQ_CC)
    continue;
    ret = devm_request_irq(&pdev.dev, ddata.irq[i], stm32_timer_cnt_isr,
    0, dev_name(dev), counter);
    if (ret)
    return ret;
    }
    }
// Reset input selector to its default input
    regmap_write(priv.regmap, TIM_TISEL, 0x0);
// Register Counter device
    ret = devm_counter_add(dev, counter);
    if (ret < 0)
    dev_err_probe(dev, ret, "Failed to add counter\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm32_timer_cnt_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused stm32_timer_cnt_suspend(struct device *dev)
    {
    struct stm32_timer_cnt *priv = dev_get_drvdata(dev);
// Only take care of enabled counter: don't disturb other MFD child
    if (priv.enabled) {
// Backup registers that may get lost in low power mode
    regmap_read(priv.regmap, TIM_SMCR, &priv.bak.smcr);
    regmap_read(priv.regmap, TIM_ARR, &priv.bak.arr);
    regmap_read(priv.regmap, TIM_CNT, &priv.bak.cnt);
    regmap_read(priv.regmap, TIM_CR1, &priv.bak.cr1);
// Disable the counter
    regmap_update_bits(priv.regmap, TIM_CR1, TIM_CR1_CEN, 0);
    clk_disable(priv.clk);
    }
    return pinctrl_pm_select_sleep_state(dev);
    }
#[no_mangle]
unsafe extern "C" fn stm32_timer_cnt_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused stm32_timer_cnt_resume(struct device *dev)
    {
    struct stm32_timer_cnt *priv = dev_get_drvdata(dev);
    int ret;
    ret = pinctrl_pm_select_default_state(dev);
    if (ret)
    return ret;
    if (priv.enabled) {
    ret = clk_enable(priv.clk);
    if (ret) {
    dev_err(dev, "Cannot enable clock %d\n", ret);
    return ret;
    }
// Restore registers that may have been lost
    regmap_write(priv.regmap, TIM_SMCR, priv.bak.smcr);
    regmap_write(priv.regmap, TIM_ARR, priv.bak.arr);
    regmap_write(priv.regmap, TIM_CNT, priv.bak.cnt);
// Also re-enables the counter
    regmap_write(priv.regmap, TIM_CR1, priv.bak.cr1);
    }
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(stm32_timer_cnt_pm_ops, stm32_timer_cnt_suspend,
    stm32_timer_cnt_resume);
    static const struct of_device_id stm32_timer_cnt_of_match[] = {
    { .compatible = "st,stm32-timer-counter", },
    { .compatible = "st,stm32mp25-timer-counter", },
    {},
    };
    MODULE_DEVICE_TABLE(of, stm32_timer_cnt_of_match);
    static struct platform_driver stm32_timer_cnt_driver = {
    .probe = stm32_timer_cnt_probe,
    .driver = {
    .name = "stm32-timer-counter",
    .of_match_table = stm32_timer_cnt_of_match,
    .pm = &stm32_timer_cnt_pm_ops,
    },
    };
    module_platform_driver(stm32_timer_cnt_driver);
    MODULE_AUTHOR("Benjamin Gaignard <benjamin.gaignard@st.com>");
    MODULE_ALIAS("platform:stm32-timer-counter");
    MODULE_DESCRIPTION("STMicroelectronics STM32 TIMER counter driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("COUNTER");

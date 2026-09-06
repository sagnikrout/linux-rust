//! Automatically rewritten from C to Rust
//! Source: drivers/counter/ti-eqep.c
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
// Copyright (C) 2019 David Lechner <david@lechnology.com>
//
// Counter driver for Texas Instruments Enhanced Quadrature Encoder Pulse (eQEP)
//

// 32-bit registers
pub const QPOSCNT: c_uint = 0x0;
pub const QPOSINIT: c_uint = 0x4;
pub const QPOSMAX: c_uint = 0x8;
pub const QPOSCMP: c_uint = 0xc;
pub const QPOSILAT: c_uint = 0x10;
pub const QPOSSLAT: c_uint = 0x14;
pub const QPOSLAT: c_uint = 0x18;
pub const QUTMR: c_uint = 0x1c;
pub const QUPRD: c_uint = 0x20;
// 16-bit registers
pub const QWDTMR: c_uint = 0x0	/* 0x24 */;
pub const QWDPRD: c_uint = 0x2	/* 0x26 */;
pub const QDECCTL: c_uint = 0x4	/* 0x28 */;
pub const QEPCTL: c_uint = 0x6	/* 0x2a */;
pub const QCAPCTL: c_uint = 0x8	/* 0x2c */;
pub const QPOSCTL: c_uint = 0xa	/* 0x2e */;
pub const QEINT: c_uint = 0xc	/* 0x30 */;
pub const QFLG: c_uint = 0xe	/* 0x32 */;
pub const QCLR: c_uint = 0x10	/* 0x34 */;
pub const QFRC: c_uint = 0x12	/* 0x36 */;
pub const QEPSTS: c_uint = 0x14	/* 0x38 */;
pub const QCTMR: c_uint = 0x16	/* 0x3a */;
pub const QCPRD: c_uint = 0x18	/* 0x3c */;
pub const QCTMRLAT: c_uint = 0x1a	/* 0x3e */;
pub const QCPRDLAT: c_uint = 0x1c	/* 0x40 */;
pub const QDECCTL_QSRC_SHIFT: c_int = 14;

// EQEP Inputs
    enum {
    TI_EQEP_SIGNAL_QEPA,	/* QEPA/XCLK */
    TI_EQEP_SIGNAL_QEPB,	/* QEPB/XDIR */
    };
// Position Counter Input Modes
    enum ti_eqep_count_func {
    TI_EQEP_COUNT_FUNC_QUAD_COUNT,
    TI_EQEP_COUNT_FUNC_DIR_COUNT,
    TI_EQEP_COUNT_FUNC_UP_COUNT,
    TI_EQEP_COUNT_FUNC_DOWN_COUNT,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_eqep_cnt {
    pub regmap32: *mut regmap,
    pub regmap16: *mut regmap,
}

    static int ti_eqep_count_read(struct counter_device *counter,
    struct counter_count *count, u64 *val)
    {
    struct ti_eqep_cnt *priv = counter_priv(counter);
    u32 cnt;
    regmap_read(priv.regmap32, QPOSCNT, &cnt);
// val = cnt;
    return 0;
    }
    static int ti_eqep_count_write(struct counter_device *counter,
    struct counter_count *count, u64 val)
    {
    struct ti_eqep_cnt *priv = counter_priv(counter);
    u32 max;
    regmap_read(priv.regmap32, QPOSMAX, &max);
    if (val > max)
    return -EINVAL;
    return regmap_write(priv.regmap32, QPOSCNT, val);
    }
    static int ti_eqep_function_read(struct counter_device *counter,
    struct counter_count *count,
    enum counter_function *function)
    {
    struct ti_eqep_cnt *priv = counter_priv(counter);
    u32 qdecctl;
    regmap_read(priv.regmap16, QDECCTL, &qdecctl);
    switch ((qdecctl & QDECCTL_QSRC) >> QDECCTL_QSRC_SHIFT) {
    case TI_EQEP_COUNT_FUNC_QUAD_COUNT:
// function = COUNTER_FUNCTION_QUADRATURE_X4;
    break;
    case TI_EQEP_COUNT_FUNC_DIR_COUNT:
// function = COUNTER_FUNCTION_PULSE_DIRECTION;
    break;
    case TI_EQEP_COUNT_FUNC_UP_COUNT:
// function = COUNTER_FUNCTION_INCREASE;
    break;
    case TI_EQEP_COUNT_FUNC_DOWN_COUNT:
// function = COUNTER_FUNCTION_DECREASE;
    break;
    }
    return 0;
    }
    static int ti_eqep_function_write(struct counter_device *counter,
    struct counter_count *count,
    enum counter_function function)
    {
    struct ti_eqep_cnt *priv = counter_priv(counter);
    enum ti_eqep_count_func qsrc;
    switch (function) {
    case COUNTER_FUNCTION_QUADRATURE_X4:
    qsrc = TI_EQEP_COUNT_FUNC_QUAD_COUNT;
    break;
    case COUNTER_FUNCTION_PULSE_DIRECTION:
    qsrc = TI_EQEP_COUNT_FUNC_DIR_COUNT;
    break;
    case COUNTER_FUNCTION_INCREASE:
    qsrc = TI_EQEP_COUNT_FUNC_UP_COUNT;
    break;
    case COUNTER_FUNCTION_DECREASE:
    qsrc = TI_EQEP_COUNT_FUNC_DOWN_COUNT;
    break;
    default:
// should never reach this path
    return -EINVAL;
    }
    return regmap_write_bits(priv.regmap16, QDECCTL, QDECCTL_QSRC,
    qsrc << QDECCTL_QSRC_SHIFT);
    }
    static int ti_eqep_action_read(struct counter_device *counter,
    struct counter_count *count,
    struct counter_synapse *synapse,
    enum counter_synapse_action *action)
    {
    struct ti_eqep_cnt *priv = counter_priv(counter);
    enum counter_function function;
    u32 qdecctl;
    int err;
    err = ti_eqep_function_read(counter, count, &function);
    if (err)
    return err;
    switch (function) {
    case COUNTER_FUNCTION_QUADRATURE_X4:
// In quadrature mode, the rising and falling edge of both
// QEPA and QEPB trigger QCLK.
//
// action = COUNTER_SYNAPSE_ACTION_BOTH_EDGES;
    return 0;
    case COUNTER_FUNCTION_PULSE_DIRECTION:
// In direction-count mode only rising edge of QEPA is counted
// and QEPB gives direction.
//
    switch (synapse.signal.id) {
    case TI_EQEP_SIGNAL_QEPA:
// action = COUNTER_SYNAPSE_ACTION_RISING_EDGE;
    return 0;
    case TI_EQEP_SIGNAL_QEPB:
// action = COUNTER_SYNAPSE_ACTION_NONE;
    return 0;
    default:
// should never reach this path
    return -EINVAL;
    }
    case COUNTER_FUNCTION_INCREASE:
    case COUNTER_FUNCTION_DECREASE:
// In up/down-count modes only QEPA is counted and QEPB is not
// used.
//
    switch (synapse.signal.id) {
    case TI_EQEP_SIGNAL_QEPA:
    err = regmap_read(priv.regmap16, QDECCTL, &qdecctl);
    if (err)
    return err;
    if (qdecctl & QDECCTL_XCR)
// action = COUNTER_SYNAPSE_ACTION_BOTH_EDGES;
    else
// action = COUNTER_SYNAPSE_ACTION_RISING_EDGE;
    return 0;
    case TI_EQEP_SIGNAL_QEPB:
// action = COUNTER_SYNAPSE_ACTION_NONE;
    return 0;
    default:
// should never reach this path
    return -EINVAL;
    }
    default:
// should never reach this path
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn ti_eqep_events_configure(counter: *mut counter_device) -> c_int {
    static int ti_eqep_events_configure(struct counter_device *counter)
    {
    struct ti_eqep_cnt *priv = counter_priv(counter);
    struct counter_event_node *event_node;
    let mut qeint: u32 = 0;
    list_for_each_entry(event_node, &counter.events_list, l) {
    switch (event_node.event) {
    case COUNTER_EVENT_OVERFLOW:
    qeint |= QEINT_PCO;
    break;
    case COUNTER_EVENT_UNDERFLOW:
    qeint |= QEINT_PCU;
    break;
    case COUNTER_EVENT_DIRECTION_CHANGE:
    qeint |= QEINT_QDC;
    break;
    }
    }
    return regmap_write(priv.regmap16, QEINT, qeint);
    }
    static int ti_eqep_watch_validate(struct counter_device *counter,
    const struct counter_watch *watch)
    {
    switch (watch.event) {
    case COUNTER_EVENT_OVERFLOW:
    case COUNTER_EVENT_UNDERFLOW:
    case COUNTER_EVENT_DIRECTION_CHANGE:
    if (watch.channel != 0)
    return -EINVAL;
    return 0;
    default:
    return -EINVAL;
    }
    }
    static const struct counter_ops ti_eqep_counter_ops = {
    .count_read	= ti_eqep_count_read,
    .count_write	= ti_eqep_count_write,
    .function_read	= ti_eqep_function_read,
    .function_write	= ti_eqep_function_write,
    .action_read	= ti_eqep_action_read,
    .events_configure = ti_eqep_events_configure,
    .watch_validate	= ti_eqep_watch_validate,
    };
    static int ti_eqep_position_ceiling_read(struct counter_device *counter,
    struct counter_count *count,
    u64 *ceiling)
    {
    struct ti_eqep_cnt *priv = counter_priv(counter);
    u32 qposmax;
    regmap_read(priv.regmap32, QPOSMAX, &qposmax);
// ceiling = qposmax;
    return 0;
    }
    static int ti_eqep_position_ceiling_write(struct counter_device *counter,
    struct counter_count *count,
    u64 ceiling)
    {
    struct ti_eqep_cnt *priv = counter_priv(counter);
    if (ceiling != (u32)ceiling)
    return -ERANGE;
    regmap_write(priv.regmap32, QPOSMAX, ceiling);
    return 0;
    }
    static int ti_eqep_position_enable_read(struct counter_device *counter,
    struct counter_count *count, u8 *enable)
    {
    struct ti_eqep_cnt *priv = counter_priv(counter);
    u32 qepctl;
    regmap_read(priv.regmap16, QEPCTL, &qepctl);
// enable = !!(qepctl & QEPCTL_PHEN);
    return 0;
    }
    static int ti_eqep_position_enable_write(struct counter_device *counter,
    struct counter_count *count, u8 enable)
    {
    struct ti_eqep_cnt *priv = counter_priv(counter);
    regmap_write_bits(priv.regmap16, QEPCTL, QEPCTL_PHEN, enable ? -1 : 0);
    return 0;
    }
    static int ti_eqep_direction_read(struct counter_device *counter,
    struct counter_count *count,
    enum counter_count_direction *direction)
    {
    struct ti_eqep_cnt *priv = counter_priv(counter);
    u32 qepsts;
    regmap_read(priv.regmap16, QEPSTS, &qepsts);
// direction = (qepsts & QEPSTS_QDF) ? COUNTER_COUNT_DIRECTION_FORWARD
    : COUNTER_COUNT_DIRECTION_BACKWARD;
    return 0;
    }
    static struct counter_comp ti_eqep_position_ext[] = {
    COUNTER_COMP_CEILING(ti_eqep_position_ceiling_read,
    ti_eqep_position_ceiling_write),
    COUNTER_COMP_ENABLE(ti_eqep_position_enable_read,
    ti_eqep_position_enable_write),
    COUNTER_COMP_DIRECTION(ti_eqep_direction_read),
    };
    static struct counter_signal ti_eqep_signals[] = {
    [TI_EQEP_SIGNAL_QEPA] = {
    .id = TI_EQEP_SIGNAL_QEPA,
    .name = "QEPA"
    },
    [TI_EQEP_SIGNAL_QEPB] = {
    .id = TI_EQEP_SIGNAL_QEPB,
    .name = "QEPB"
    },
    };
    static const enum counter_function ti_eqep_position_functions[] = {
    COUNTER_FUNCTION_QUADRATURE_X4,
    COUNTER_FUNCTION_PULSE_DIRECTION,
    COUNTER_FUNCTION_INCREASE,
    COUNTER_FUNCTION_DECREASE,
    };
    static const enum counter_synapse_action ti_eqep_position_synapse_actions[] = {
    COUNTER_SYNAPSE_ACTION_BOTH_EDGES,
    COUNTER_SYNAPSE_ACTION_RISING_EDGE,
    COUNTER_SYNAPSE_ACTION_NONE,
    };
    static struct counter_synapse ti_eqep_position_synapses[] = {
    {
    .actions_list	= ti_eqep_position_synapse_actions,
    .num_actions	= ARRAY_SIZE(ti_eqep_position_synapse_actions),
    .signal		= &ti_eqep_signals[TI_EQEP_SIGNAL_QEPA],
    },
    {
    .actions_list	= ti_eqep_position_synapse_actions,
    .num_actions	= ARRAY_SIZE(ti_eqep_position_synapse_actions),
    .signal		= &ti_eqep_signals[TI_EQEP_SIGNAL_QEPB],
    },
    };
    static struct counter_count ti_eqep_counts[] = {
    {
    .id		= 0,
    .name		= "QPOSCNT",
    .functions_list	= ti_eqep_position_functions,
    .num_functions	= ARRAY_SIZE(ti_eqep_position_functions),
    .synapses	= ti_eqep_position_synapses,
    .num_synapses	= ARRAY_SIZE(ti_eqep_position_synapses),
    .ext		= ti_eqep_position_ext,
    .num_ext	= ARRAY_SIZE(ti_eqep_position_ext),
    },
    };
#[no_mangle]
unsafe extern "C" fn ti_eqep_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ti_eqep_irq_handler(int irq, void *dev_id)
    {
    struct counter_device *counter = dev_id;
    struct ti_eqep_cnt *priv = counter_priv(counter);
    u32 qflg;
    regmap_read(priv.regmap16, QFLG, &qflg);
    if (qflg & QFLG_PCO)
    counter_push_event(counter, COUNTER_EVENT_OVERFLOW, 0);
    if (qflg & QFLG_PCU)
    counter_push_event(counter, COUNTER_EVENT_UNDERFLOW, 0);
    if (qflg & QFLG_QDC)
    counter_push_event(counter, COUNTER_EVENT_DIRECTION_CHANGE, 0);
    regmap_write(priv.regmap16, QCLR, qflg);
    return IRQ_HANDLED;
    }
    static const struct regmap_config ti_eqep_regmap32_config = {
    .name = "32-bit",
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = QUPRD,
    };
    static const struct regmap_config ti_eqep_regmap16_config = {
    .name = "16-bit",
    .reg_bits = 16,
    .val_bits = 16,
    .reg_stride = 2,
    .max_register = QCPRDLAT,
    };
#[no_mangle]
unsafe extern "C" fn ti_eqep_probe(pdev: *mut platform_device) -> c_int {
    static int ti_eqep_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct counter_device *counter;
    struct ti_eqep_cnt *priv;
    void __iomem *base;
    struct clk *clk;
    int err, irq;
    counter = devm_counter_alloc(dev, sizeof(*priv));
    if (!counter)
    return -ENOMEM;
    priv = counter_priv(counter);
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    priv.regmap32 = devm_regmap_init_mmio(dev, base,
    &ti_eqep_regmap32_config);
    if (IS_ERR(priv.regmap32))
    return PTR_ERR(priv.regmap32);
    priv.regmap16 = devm_regmap_init_mmio(dev, base + 0x24,
    &ti_eqep_regmap16_config);
    if (IS_ERR(priv.regmap16))
    return PTR_ERR(priv.regmap16);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    err = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(), ti_eqep_irq_handler,
    IRQF_ONESHOT, dev_name(dev), counter);
    if (err < 0)
    return err;
    counter.name = dev_name(dev);
    counter.parent = dev;
    counter.ops = &ti_eqep_counter_ops;
    counter.counts = ti_eqep_counts;
    counter.num_counts = ARRAY_SIZE(ti_eqep_counts);
    counter.signals = ti_eqep_signals;
    counter.num_signals = ARRAY_SIZE(ti_eqep_signals);
    platform_set_drvdata(pdev, counter);
//
// Need to make sure power is turned on. On AM33xx, this comes from the
// parent PWMSS bus driver. On AM17xx, this comes from the PSC power
// domain.
//
    pm_runtime_enable(dev);
    pm_runtime_get_sync(dev);
    clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk), "failed to enable clock\n");
    err = counter_add(counter);
    if (err < 0) {
    pm_runtime_put_sync(dev);
    pm_runtime_disable(dev);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ti_eqep_remove(pdev: *mut platform_device) {
    static void ti_eqep_remove(struct platform_device *pdev)
    {
    struct counter_device *counter = platform_get_drvdata(pdev);
    struct device *dev = &pdev.dev;
    counter_unregister(counter);
    pm_runtime_put_sync(dev);
    pm_runtime_disable(dev);
    }
    static const struct of_device_id ti_eqep_of_match[] = {
    { .compatible = "ti,am3352-eqep", },
    { .compatible = "ti,am62-eqep", },
    { },
    };
    MODULE_DEVICE_TABLE(of, ti_eqep_of_match);
    static struct platform_driver ti_eqep_driver = {
    .probe = ti_eqep_probe,
    .remove = ti_eqep_remove,
    .driver = {
    .name = "ti-eqep-cnt",
    .of_match_table = ti_eqep_of_match,
    },
    };
    module_platform_driver(ti_eqep_driver);
    MODULE_AUTHOR("David Lechner <david@lechnology.com>");
    MODULE_DESCRIPTION("TI eQEP counter driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("COUNTER");

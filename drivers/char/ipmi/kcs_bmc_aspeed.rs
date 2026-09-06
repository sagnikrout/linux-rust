//! Automatically rewritten from C to Rust
//! Source: drivers/char/ipmi/kcs_bmc_aspeed.c
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
// Copyright (c) 2015-2018, Intel Corporation.
//

pub const KCS_CHANNEL_MAX: c_int = 4;
//
// Field class descriptions
//
// LPCyE	Enable LPC channel y
// IBFIEy	Input Buffer Full IRQ Enable for LPC channel y
// IRQxEy	Assert SerIRQ x for LPC channel y (Deprecated, use IDyIRQX, IRQXEy)
// IDyIRQX	Use the specified 4-bit SerIRQ for LPC channel y
// SELyIRQX	SerIRQ polarity for LPC channel y (low: 0, high: 1)
// IRQXEy	Assert the SerIRQ specified in IDyIRQX for LPC channel y
//

pub const LPC_HICR0: c_uint = 0x000;

pub const LPC_HICR2: c_uint = 0x008;

pub const LPC_HICR4: c_uint = 0x010;

pub const LPC_SIRQCR0: c_uint = 0x070;
// IRQ{12,1}E1 are deprecated as of AST2600 A3 but necessary for prior chips

pub const LPC_HICR5: c_uint = 0x080;

pub const LPC_HICR5_ID3IRQX_SHIFT: c_int = 20;

pub const LPC_HICR5_ID2IRQX_SHIFT: c_int = 16;

pub const LPC_LADR3H: c_uint = 0x014;
pub const LPC_LADR3L: c_uint = 0x018;
pub const LPC_LADR12H: c_uint = 0x01C;
pub const LPC_LADR12L: c_uint = 0x020;
pub const LPC_IDR1: c_uint = 0x024;
pub const LPC_IDR2: c_uint = 0x028;
pub const LPC_IDR3: c_uint = 0x02C;
pub const LPC_ODR1: c_uint = 0x030;
pub const LPC_ODR2: c_uint = 0x034;
pub const LPC_ODR3: c_uint = 0x038;
pub const LPC_STR1: c_uint = 0x03C;
pub const LPC_STR2: c_uint = 0x040;
pub const LPC_STR3: c_uint = 0x044;
pub const LPC_HICRB: c_uint = 0x100;

pub const LPC_HICRC: c_uint = 0x104;

pub const LPC_HICRC_ID4IRQX_SHIFT: c_int = 4;

pub const LPC_HICRC_TY4IRQX_SHIFT: c_int = 2;

pub const LPC_LADR4: c_uint = 0x110;
pub const LPC_IDR4: c_uint = 0x114;
pub const LPC_ODR4: c_uint = 0x118;
pub const LPC_STR4: c_uint = 0x11C;
pub const LPC_LSADR12: c_uint = 0x120;

pub const LPC_LSADR12_LSADR2_SHIFT: c_int = 16;

pub const LPC_LSADR12_LSADR1_SHIFT: c_int = 0;

    enum aspeed_kcs_irq_mode {
    aspeed_kcs_irq_none,
    aspeed_kcs_irq_serirq,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_kcs_bmc {
    pub kcs_bmc: kcs_bmc_device,
    pub map: *mut regmap,
    struct {
    pub mode: enum aspeed_kcs_irq_mode,
    pub id: c_int,
    pub upstream_irq: },
    struct {
    pub lock: spinlock_t,
    pub remove: bool,
    pub timer: timer_list,
    pub obe: },
}

    static inline struct aspeed_kcs_bmc *to_aspeed_kcs_bmc(struct kcs_bmc_device *kcs_bmc)
    {
    return container_of(kcs_bmc, struct aspeed_kcs_bmc, kcs_bmc);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_kcs_inb(kcs_bmc: *mut kcs_bmc_device, reg: u32) -> u8 {
    static u8 aspeed_kcs_inb(struct kcs_bmc_device *kcs_bmc, u32 reg)
    {
    struct aspeed_kcs_bmc *priv = to_aspeed_kcs_bmc(kcs_bmc);
    let mut val: u32 = 0;
    int rc;
    rc = regmap_read(priv.map, reg, &val);
    WARN(rc != 0, "regmap_read() failed: %d\n", rc);
    let mut rc: return = = 0 ? (u8) val : 0;
    }
#[no_mangle]
unsafe extern "C" fn aspeed_kcs_outb(kcs_bmc: *mut kcs_bmc_device, reg: u32, data: u8) {
    static void aspeed_kcs_outb(struct kcs_bmc_device *kcs_bmc, u32 reg, u8 data)
    {
    struct aspeed_kcs_bmc *priv = to_aspeed_kcs_bmc(kcs_bmc);
    int rc;
    rc = regmap_write(priv.map, reg, data);
    WARN(rc != 0, "regmap_write() failed: %d\n", rc);
// Trigger the upstream IRQ on ODR writes, if enabled
    switch (reg) {
    case LPC_ODR1:
    case LPC_ODR2:
    case LPC_ODR3:
    case LPC_ODR4:
    break;
    default:
    return;
    }
    if (priv.upstream_irq.mode != aspeed_kcs_irq_serirq)
    return;
    switch (kcs_bmc.channel) {
    case 1:
    switch (priv.upstream_irq.id) {
    case 12:
    regmap_update_bits(priv.map, LPC_SIRQCR0, LPC_SIRQCR0_IRQ12E1,
    LPC_SIRQCR0_IRQ12E1);
    break;
    case 1:
    regmap_update_bits(priv.map, LPC_SIRQCR0, LPC_SIRQCR0_IRQ1E1,
    LPC_SIRQCR0_IRQ1E1);
    break;
    default:
    break;
    }
    break;
    case 2:
    regmap_update_bits(priv.map, LPC_HICR5, LPC_HICR5_IRQXE2, LPC_HICR5_IRQXE2);
    break;
    case 3:
    regmap_update_bits(priv.map, LPC_HICR5, LPC_HICR5_IRQXE3, LPC_HICR5_IRQXE3);
    break;
    case 4:
    regmap_update_bits(priv.map, LPC_HICRC, LPC_HICRC_IRQXE4, LPC_HICRC_IRQXE4);
    break;
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn aspeed_kcs_updateb(kcs_bmc: *mut kcs_bmc_device, reg: u32, mask: u8, val: u8) {
    static void aspeed_kcs_updateb(struct kcs_bmc_device *kcs_bmc, u32 reg, u8 mask, u8 val)
    {
    struct aspeed_kcs_bmc *priv = to_aspeed_kcs_bmc(kcs_bmc);
    int rc;
    rc = regmap_update_bits(priv.map, reg, mask, val);
    WARN(rc != 0, "regmap_update_bits() failed: %d\n", rc);
    }
//
// We note D for Data, and C for Cmd/Status, default rules are
//
// 1. Only the D address is given:
// A. KCS1/KCS2 (D/C: X/X+4)
// D/C: CA0h/CA4h
// D/C: CA8h/CACh
// B. KCS3 (D/C: XX2/XX3h)
// D/C: CA2h/CA3h
// C. KCS4 (D/C: X/X+1)
// D/C: CA4h/CA5h
//
// 2. Both the D/C addresses are given:
// A. KCS1/KCS2/KCS4 (D/C: X/Y)
// D/C: CA0h/CA1h
// D/C: CA8h/CA9h
// D/C: CA4h/CA5h
// B. KCS3 (D/C: XX2/XX3h)
// D/C: CA2h/CA3h
//
#[no_mangle]
unsafe extern "C" fn aspeed_kcs_set_address(kcs_bmc: *mut kcs_bmc_device, addrs[2]: u32, nr_addrs: c_int) -> c_int {
    static int aspeed_kcs_set_address(struct kcs_bmc_device *kcs_bmc, u32 addrs[2], int nr_addrs)
    {
    struct aspeed_kcs_bmc *priv = to_aspeed_kcs_bmc(kcs_bmc);
    if (WARN_ON(nr_addrs < 1 || nr_addrs > 2))
    return -EINVAL;
    switch (priv.kcs_bmc.channel) {
    case 1:
    regmap_update_bits(priv.map, LPC_HICR4, LPC_HICR4_LADR12AS, 0);
    regmap_write(priv.map, LPC_LADR12H, addrs[0] >> 8);
    regmap_write(priv.map, LPC_LADR12L, addrs[0] & 0xFF);
    if (nr_addrs == 2) {
    regmap_update_bits(priv.map, LPC_LSADR12, LPC_LSADR12_LSADR1_MASK,
    addrs[1] << LPC_LSADR12_LSADR1_SHIFT);
    regmap_update_bits(priv.map, LPC_HICRB, LPC_HICRB_EN16LADR1,
    LPC_HICRB_EN16LADR1);
    }
    break;
    case 2:
    regmap_update_bits(priv.map, LPC_HICR4, LPC_HICR4_LADR12AS, LPC_HICR4_LADR12AS);
    regmap_write(priv.map, LPC_LADR12H, addrs[0] >> 8);
    regmap_write(priv.map, LPC_LADR12L, addrs[0] & 0xFF);
    if (nr_addrs == 2) {
    regmap_update_bits(priv.map, LPC_LSADR12, LPC_LSADR12_LSADR2_MASK,
    addrs[1] << LPC_LSADR12_LSADR2_SHIFT);
    regmap_update_bits(priv.map, LPC_HICRB, LPC_HICRB_EN16LADR2,
    LPC_HICRB_EN16LADR2);
    }
    break;
    case 3:
    if (nr_addrs == 2) {
    dev_err(priv.kcs_bmc.dev,
    "Channel 3 only supports inferred status IO address\n");
    return -EINVAL;
    }
    regmap_write(priv.map, LPC_LADR3H, addrs[0] >> 8);
    regmap_write(priv.map, LPC_LADR3L, addrs[0] & 0xFF);
    break;
    case 4:
    if (nr_addrs == 1)
    regmap_write(priv.map, LPC_LADR4, ((addrs[0] + 1) << 16) | addrs[0]);
    else
    regmap_write(priv.map, LPC_LADR4, (addrs[1] << 16) | addrs[0]);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn aspeed_kcs_map_serirq_type(dt_type: u32) -> c_int {
    static inline int aspeed_kcs_map_serirq_type(u32 dt_type)
    {
    switch (dt_type) {
    case IRQ_TYPE_EDGE_RISING:
    return LPC_TYIRQX_RISING;
    case IRQ_TYPE_LEVEL_HIGH:
    return LPC_TYIRQX_HIGH;
    case IRQ_TYPE_LEVEL_LOW:
    return LPC_TYIRQX_LOW;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn aspeed_kcs_config_upstream_irq(priv: *mut aspeed_kcs_bmc, id: u32, dt_type: u32) -> c_int {
    static int aspeed_kcs_config_upstream_irq(struct aspeed_kcs_bmc *priv, u32 id, u32 dt_type)
    {
    unsigned int mask, val, hw_type;
    int ret;
    if (id > 15)
    return -EINVAL;
    ret = aspeed_kcs_map_serirq_type(dt_type);
    if (ret < 0)
    return ret;
    hw_type = ret;
    priv.upstream_irq.mode = aspeed_kcs_irq_serirq;
    priv.upstream_irq.id = id;
    switch (priv.kcs_bmc.channel) {
    case 1:
// Needs IRQxE1 rather than (ID1IRQX, SEL1IRQX, IRQXE1) before AST2600 A3
    break;
    case 2:
    if (!(hw_type == LPC_TYIRQX_LOW || hw_type == LPC_TYIRQX_HIGH))
    return -EINVAL;
    mask = LPC_HICR5_SEL2IRQX | LPC_HICR5_ID2IRQX_MASK;
    val = (id << LPC_HICR5_ID2IRQX_SHIFT);
    val |= (hw_type == LPC_TYIRQX_HIGH) ? LPC_HICR5_SEL2IRQX : 0;
    regmap_update_bits(priv.map, LPC_HICR5, mask, val);
    break;
    case 3:
    if (!(hw_type == LPC_TYIRQX_LOW || hw_type == LPC_TYIRQX_HIGH))
    return -EINVAL;
    mask = LPC_HICR5_SEL3IRQX | LPC_HICR5_ID3IRQX_MASK;
    val = (id << LPC_HICR5_ID3IRQX_SHIFT);
    val |= (hw_type == LPC_TYIRQX_HIGH) ? LPC_HICR5_SEL3IRQX : 0;
    regmap_update_bits(priv.map, LPC_HICR5, mask, val);
    break;
    case 4:
    mask = LPC_HICRC_ID4IRQX_MASK | LPC_HICRC_TY4IRQX_MASK | LPC_HICRC_OBF4_AUTO_CLR;
    val = (id << LPC_HICRC_ID4IRQX_SHIFT) | (hw_type << LPC_HICRC_TY4IRQX_SHIFT);
    regmap_update_bits(priv.map, LPC_HICRC, mask, val);
    break;
    default:
    dev_warn(priv.kcs_bmc.dev,
    "SerIRQ configuration not supported on KCS channel %d\n",
    priv.kcs_bmc.channel);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aspeed_kcs_enable_channel(kcs_bmc: *mut kcs_bmc_device, enable: bool) {
    static void aspeed_kcs_enable_channel(struct kcs_bmc_device *kcs_bmc, bool enable)
    {
    struct aspeed_kcs_bmc *priv = to_aspeed_kcs_bmc(kcs_bmc);
    switch (kcs_bmc.channel) {
    case 1:
    regmap_update_bits(priv.map, LPC_HICR0, LPC_HICR0_LPC1E, enable * LPC_HICR0_LPC1E);
    return;
    case 2:
    regmap_update_bits(priv.map, LPC_HICR0, LPC_HICR0_LPC2E, enable * LPC_HICR0_LPC2E);
    return;
    case 3:
    regmap_update_bits(priv.map, LPC_HICR0, LPC_HICR0_LPC3E, enable * LPC_HICR0_LPC3E);
    regmap_update_bits(priv.map, LPC_HICR4,
    LPC_HICR4_KCSENBL, enable * LPC_HICR4_KCSENBL);
    return;
    case 4:
    regmap_update_bits(priv.map, LPC_HICRB, LPC_HICRB_LPC4E, enable * LPC_HICRB_LPC4E);
    return;
    default:
    pr_warn("%s: Unsupported channel: %d", __func__, kcs_bmc.channel);
    return;
    }
    }
#[no_mangle]
unsafe extern "C" fn aspeed_kcs_check_obe(timer: *mut timer_list) {
    static void aspeed_kcs_check_obe(struct timer_list *timer)
    {
    struct aspeed_kcs_bmc *priv = container_of(timer, struct aspeed_kcs_bmc, obe.timer);
    unsigned long flags;
    u8 str;
    spin_lock_irqsave(&priv.obe.lock, flags);
    if (priv.obe.remove) {
    spin_unlock_irqrestore(&priv.obe.lock, flags);
    return;
    }
    str = aspeed_kcs_inb(&priv.kcs_bmc, priv.kcs_bmc.ioreg.str);
    if (str & KCS_BMC_STR_OBF) {
    mod_timer(timer, jiffies + OBE_POLL_PERIOD);
    spin_unlock_irqrestore(&priv.obe.lock, flags);
    return;
    }
    spin_unlock_irqrestore(&priv.obe.lock, flags);
    kcs_bmc_handle_event(&priv.kcs_bmc);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_kcs_irq_mask_update(kcs_bmc: *mut kcs_bmc_device, mask: u8, state: u8) {
    static void aspeed_kcs_irq_mask_update(struct kcs_bmc_device *kcs_bmc, u8 mask, u8 state)
    {
    struct aspeed_kcs_bmc *priv = to_aspeed_kcs_bmc(kcs_bmc);
    int rc;
    u8 str;
// We don't have an OBE IRQ, emulate it
    if (mask & KCS_BMC_EVENT_TYPE_OBE) {
    if (KCS_BMC_EVENT_TYPE_OBE & state) {
//
// Given we don't have an OBE IRQ, delay by polling briefly to see if we can
// observe such an event before returning to the caller. This is not
// incorrect because OBF may have already become clear before enabling the
// IRQ if we had one, under which circumstance no event will be propagated
// anyway.
//
// The onus is on the client to perform a race-free check that it hasn't
// missed the event.
//
    rc = read_poll_timeout_atomic(aspeed_kcs_inb, str,
    !(str & KCS_BMC_STR_OBF), 1, 100, false,
    &priv.kcs_bmc, priv.kcs_bmc.ioreg.str);
// Time for the slow path?
    if (rc == -ETIMEDOUT)
    mod_timer(&priv.obe.timer, jiffies + OBE_POLL_PERIOD);
    } else {
    timer_delete(&priv.obe.timer);
    }
    }
    if (mask & KCS_BMC_EVENT_TYPE_IBF) {
    let mut enable: bool = !!(state & KCS_BMC_EVENT_TYPE_IBF);
    switch (kcs_bmc.channel) {
    case 1:
    regmap_update_bits(priv.map, LPC_HICR2, LPC_HICR2_IBFIE1,
    enable * LPC_HICR2_IBFIE1);
    return;
    case 2:
    regmap_update_bits(priv.map, LPC_HICR2, LPC_HICR2_IBFIE2,
    enable * LPC_HICR2_IBFIE2);
    return;
    case 3:
    regmap_update_bits(priv.map, LPC_HICR2, LPC_HICR2_IBFIE3,
    enable * LPC_HICR2_IBFIE3);
    return;
    case 4:
    regmap_update_bits(priv.map, LPC_HICRB, LPC_HICRB_IBFIE4,
    enable * LPC_HICRB_IBFIE4);
    return;
    default:
    pr_warn("%s: Unsupported channel: %d", __func__, kcs_bmc.channel);
    return;
    }
    }
    }
    static const struct kcs_bmc_device_ops aspeed_kcs_ops = {
    .irq_mask_update = aspeed_kcs_irq_mask_update,
    .io_inputb = aspeed_kcs_inb,
    .io_outputb = aspeed_kcs_outb,
    .io_updateb = aspeed_kcs_updateb,
    };
#[no_mangle]
unsafe extern "C" fn aspeed_kcs_irq(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t aspeed_kcs_irq(int irq, void *arg)
    {
    struct kcs_bmc_device *kcs_bmc = arg;
    return kcs_bmc_handle_event(kcs_bmc);
    }
    static int aspeed_kcs_config_downstream_irq(struct kcs_bmc_device *kcs_bmc,
    struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int irq;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    return devm_request_irq(dev, irq, aspeed_kcs_irq, IRQF_SHARED,
    dev_name(dev), kcs_bmc);
    }
    static const struct kcs_ioreg ast_kcs_bmc_ioregs[KCS_CHANNEL_MAX] = {
    { .idr = LPC_IDR1, .odr = LPC_ODR1, .str = LPC_STR1 },
    { .idr = LPC_IDR2, .odr = LPC_ODR2, .str = LPC_STR2 },
    { .idr = LPC_IDR3, .odr = LPC_ODR3, .str = LPC_STR3 },
    { .idr = LPC_IDR4, .odr = LPC_ODR4, .str = LPC_STR4 },
    };
#[no_mangle]
unsafe extern "C" fn aspeed_kcs_of_get_channel(pdev: *mut platform_device) -> c_int {
    static int aspeed_kcs_of_get_channel(struct platform_device *pdev)
    {
    struct device_node *np;
    struct kcs_ioreg ioreg;
    const __be32 *reg;
    int i;
    np = pdev.dev.of_node;
// Don't translate addresses, we want offsets for the regmaps
    reg = of_get_address(np, 0, core::ptr::null_mut(), core::ptr::null_mut());
    if (!reg)
    return -EINVAL;
    ioreg.idr = be32_to_cpup(reg);
    reg = of_get_address(np, 1, core::ptr::null_mut(), core::ptr::null_mut());
    if (!reg)
    return -EINVAL;
    ioreg.odr = be32_to_cpup(reg);
    reg = of_get_address(np, 2, core::ptr::null_mut(), core::ptr::null_mut());
    if (!reg)
    return -EINVAL;
    ioreg.str = be32_to_cpup(reg);
    for (i = 0; i < ARRAY_SIZE(ast_kcs_bmc_ioregs); i++) {
    if (!memcmp(&ast_kcs_bmc_ioregs[i], &ioreg, sizeof(ioreg)))
    return i + 1;
    }
    return -EINVAL;
    }
    static int
    aspeed_kcs_of_get_io_address(struct platform_device *pdev, u32 addrs[2])
    {
    int rc;
    rc = of_property_read_variable_u32_array(pdev.dev.of_node,
    "aspeed,lpc-io-reg",
    addrs, 1, 2);
    if (rc < 0) {
    dev_err(&pdev.dev, "No valid 'aspeed,lpc-io-reg' configured\n");
    return rc;
    }
    if (addrs[0] > 0xffff) {
    dev_err(&pdev.dev, "Invalid data address in 'aspeed,lpc-io-reg'\n");
    return -EINVAL;
    }
    if (rc == 2 && addrs[1] > 0xffff) {
    dev_err(&pdev.dev, "Invalid status address in 'aspeed,lpc-io-reg'\n");
    return -EINVAL;
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn aspeed_kcs_probe(pdev: *mut platform_device) -> c_int {
    static int aspeed_kcs_probe(struct platform_device *pdev)
    {
    struct kcs_bmc_device *kcs_bmc;
    struct aspeed_kcs_bmc *priv;
    struct device_node *np;
    bool have_upstream_irq;
    u32 upstream_irq[2];
    int rc, channel;
    int nr_addrs;
    u32 addrs[2];
    np = pdev.dev.of_node.parent;
    if (!of_device_is_compatible(np, "aspeed,ast2400-lpc-v2") &&
    !of_device_is_compatible(np, "aspeed,ast2500-lpc-v2") &&
    !of_device_is_compatible(np, "aspeed,ast2600-lpc-v2")) {
    dev_err(&pdev.dev, "unsupported LPC device binding\n");
    return -ENODEV;
    }
    channel = aspeed_kcs_of_get_channel(pdev);
    if (channel < 0)
    return channel;
    nr_addrs = aspeed_kcs_of_get_io_address(pdev, addrs);
    if (nr_addrs < 0)
    return nr_addrs;
    np = pdev.dev.of_node;
    rc = of_property_read_u32_array(np, "aspeed,lpc-interrupts", upstream_irq, 2);
    if (rc && rc != -EINVAL)
    return -EINVAL;
    have_upstream_irq = !rc;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    kcs_bmc = &priv.kcs_bmc;
    kcs_bmc.dev = &pdev.dev;
    kcs_bmc.channel = channel;
    kcs_bmc.ioreg = ast_kcs_bmc_ioregs[channel - 1];
    kcs_bmc.ops = &aspeed_kcs_ops;
    priv.map = syscon_node_to_regmap(pdev.dev.parent.of_node);
    if (IS_ERR(priv.map)) {
    dev_err(&pdev.dev, "Couldn't get regmap\n");
    return -ENODEV;
    }
    spin_lock_init(&priv.obe.lock);
    priv.obe.remove = false;
    timer_setup(&priv.obe.timer, aspeed_kcs_check_obe, 0);
    rc = aspeed_kcs_set_address(kcs_bmc, addrs, nr_addrs);
    if (rc)
    return rc;
// Host to BMC IRQ
    rc = aspeed_kcs_config_downstream_irq(kcs_bmc, pdev);
    if (rc)
    return rc;
// BMC to Host IRQ
    if (have_upstream_irq) {
    rc = aspeed_kcs_config_upstream_irq(priv, upstream_irq[0], upstream_irq[1]);
    if (rc < 0)
    return rc;
    } else {
    priv.upstream_irq.mode = aspeed_kcs_irq_none;
    }
    platform_set_drvdata(pdev, priv);
    aspeed_kcs_irq_mask_update(kcs_bmc, (KCS_BMC_EVENT_TYPE_IBF | KCS_BMC_EVENT_TYPE_OBE), 0);
    aspeed_kcs_enable_channel(kcs_bmc, true);
    rc = kcs_bmc_add_device(&priv.kcs_bmc);
    if (rc) {
    dev_warn(&pdev.dev, "Failed to register channel %d: %d\n", kcs_bmc.channel, rc);
    return rc;
    }
    dev_info(&pdev.dev, "Initialised channel %d at 0x%x\n",
    kcs_bmc.channel, addrs[0]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aspeed_kcs_remove(pdev: *mut platform_device) {
    static void aspeed_kcs_remove(struct platform_device *pdev)
    {
    struct aspeed_kcs_bmc *priv = platform_get_drvdata(pdev);
    struct kcs_bmc_device *kcs_bmc = &priv.kcs_bmc;
    kcs_bmc_remove_device(kcs_bmc);
    aspeed_kcs_enable_channel(kcs_bmc, false);
    aspeed_kcs_irq_mask_update(kcs_bmc, (KCS_BMC_EVENT_TYPE_IBF | KCS_BMC_EVENT_TYPE_OBE), 0);
// Make sure it's proper dead
    spin_lock_irq(&priv.obe.lock);
    priv.obe.remove = true;
    spin_unlock_irq(&priv.obe.lock);
    timer_delete_sync(&priv.obe.timer);
    }
    static const struct of_device_id ast_kcs_bmc_match[] = {
    { .compatible = "aspeed,ast2400-kcs-bmc-v2" },
    { .compatible = "aspeed,ast2500-kcs-bmc-v2" },
    { .compatible = "aspeed,ast2600-kcs-bmc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ast_kcs_bmc_match);
    static struct platform_driver ast_kcs_bmc_driver = {
    .driver = {
    .name           = DEVICE_NAME,
    .of_match_table = ast_kcs_bmc_match,
    },
    .probe  = aspeed_kcs_probe,
    .remove = aspeed_kcs_remove,
    };
    module_platform_driver(ast_kcs_bmc_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Haiyue Wang <haiyue.wang@linux.intel.com>");
    MODULE_AUTHOR("Andrew Jeffery <andrew@aj.id.au>");
    MODULE_DESCRIPTION("Aspeed device interface to the KCS BMC device");

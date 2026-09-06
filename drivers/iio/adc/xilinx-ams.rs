//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/xilinx-ams.c
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
// Xilinx AMS driver
//
// Copyright (C) 2021 Xilinx, Inc.
//
// Manish Narani <mnarani@xilinx.com>
// Rajnikant Bhojani <rajnikant.bhojani@xilinx.com>
//

// AMS registers definitions
pub const AMS_ISR_0: c_uint = 0x010;
pub const AMS_ISR_1: c_uint = 0x014;
pub const AMS_IER_0: c_uint = 0x020;
pub const AMS_IER_1: c_uint = 0x024;
pub const AMS_IDR_0: c_uint = 0x028;
pub const AMS_IDR_1: c_uint = 0x02C;
pub const AMS_PS_CSTS: c_uint = 0x040;
pub const AMS_PL_CSTS: c_uint = 0x044;
pub const AMS_VCC_PSPLL0: c_uint = 0x060;
pub const AMS_VCC_PSPLL3: c_uint = 0x06C;
pub const AMS_VCCINT: c_uint = 0x078;
pub const AMS_VCCBRAM: c_uint = 0x07C;
pub const AMS_VCCAUX: c_uint = 0x080;
pub const AMS_PSDDRPLL: c_uint = 0x084;
pub const AMS_PSINTFPDDR: c_uint = 0x09C;
pub const AMS_VCC_PSPLL0_CH: c_int = 48;
pub const AMS_VCC_PSPLL3_CH: c_int = 51;
pub const AMS_VCCINT_CH: c_int = 54;
pub const AMS_VCCBRAM_CH: c_int = 55;
pub const AMS_VCCAUX_CH: c_int = 56;
pub const AMS_PSDDRPLL_CH: c_int = 57;
pub const AMS_PSINTFPDDR_CH: c_int = 63;
pub const AMS_REG_CONFIG0: c_uint = 0x100;
pub const AMS_REG_CONFIG1: c_uint = 0x104;
pub const AMS_REG_CONFIG3: c_uint = 0x10C;
pub const AMS_REG_CONFIG4: c_uint = 0x110;
pub const AMS_REG_SEQ_CH0: c_uint = 0x120;
pub const AMS_REG_SEQ_CH1: c_uint = 0x124;
pub const AMS_REG_SEQ_CH2: c_uint = 0x118;

pub const AMS_TEMP: c_uint = 0x000;
pub const AMS_SUPPLY1: c_uint = 0x004;
pub const AMS_SUPPLY2: c_uint = 0x008;
pub const AMS_VP_VN: c_uint = 0x00C;
pub const AMS_VREFP: c_uint = 0x010;
pub const AMS_VREFN: c_uint = 0x014;
pub const AMS_SUPPLY3: c_uint = 0x018;
pub const AMS_SUPPLY4: c_uint = 0x034;
pub const AMS_SUPPLY5: c_uint = 0x038;
pub const AMS_SUPPLY6: c_uint = 0x03C;
pub const AMS_SUPPLY7: c_uint = 0x200;
pub const AMS_SUPPLY8: c_uint = 0x204;
pub const AMS_SUPPLY9: c_uint = 0x208;
pub const AMS_SUPPLY10: c_uint = 0x20C;
pub const AMS_VCCAMS: c_uint = 0x210;
pub const AMS_TEMP_REMOTE: c_uint = 0x214;

pub const AMS_PS_RESET_VALUE: c_uint = 0xFFFF;
pub const AMS_PL_RESET_VALUE: c_uint = 0xFFFF;

pub const AMS_ALARM_NONE: c_uint = 0x000 /* not a real offset */;
pub const AMS_ALARM_TEMP: c_uint = 0x140;
pub const AMS_ALARM_SUPPLY1: c_uint = 0x144;
pub const AMS_ALARM_SUPPLY2: c_uint = 0x148;
pub const AMS_ALARM_SUPPLY3: c_uint = 0x160;
pub const AMS_ALARM_SUPPLY4: c_uint = 0x164;
pub const AMS_ALARM_SUPPLY5: c_uint = 0x168;
pub const AMS_ALARM_SUPPLY6: c_uint = 0x16C;
pub const AMS_ALARM_SUPPLY7: c_uint = 0x180;
pub const AMS_ALARM_SUPPLY8: c_uint = 0x184;
pub const AMS_ALARM_SUPPLY9: c_uint = 0x188;
pub const AMS_ALARM_SUPPLY10: c_uint = 0x18C;
pub const AMS_ALARM_VCCAMS: c_uint = 0x190;
pub const AMS_ALARM_TEMP_REMOTE: c_uint = 0x194;
pub const AMS_ALARM_THRESHOLD_OFF_10: c_uint = 0x10;
pub const AMS_ALARM_THRESHOLD_OFF_20: c_uint = 0x20;

pub const AMS_ALARM_THR_MIN: c_uint = 0x0000;

pub const AMS_NO_OF_ALARMS: c_int = 32;
pub const AMS_PL_ALARM_START: c_int = 16;

    (AMS_CONF1_ALARM_2_TO_0_MASK | AMS_CONF1_ALARM_6_TO_3_MASK | BIT(0))

pub const AMS_PL_MAX_FIXED_CHANNEL: c_int = 10;
pub const AMS_PL_MAX_EXT_CHANNEL: c_int = 20;
pub const AMS_INIT_POLL_TIME_US: c_int = 200;
pub const AMS_INIT_TIMEOUT_US: c_int = 10000;
pub const AMS_UNMASK_TIMEOUT_MS: c_int = 500;
//
// Following scale and offset value is derived from
// UG580 (v1.7) December 20, 2016
//
pub const AMS_SUPPLY_SCALE_1VOLT_mV: c_int = 1000;
pub const AMS_SUPPLY_SCALE_3VOLT_mV: c_int = 3000;
pub const AMS_SUPPLY_SCALE_6VOLT_mV: c_int = 6000;
pub const AMS_SUPPLY_SCALE_DIV_BIT: c_int = 16;
pub const AMS_TEMP_SCALE: c_int = 509314;
pub const AMS_TEMP_SCALE_DIV_BIT: c_int = 16;

    enum ams_alarm_bit {
    AMS_ALARM_BIT_TEMP = 0,
    AMS_ALARM_BIT_SUPPLY1 = 1,
    AMS_ALARM_BIT_SUPPLY2 = 2,
    AMS_ALARM_BIT_SUPPLY3 = 3,
    AMS_ALARM_BIT_SUPPLY4 = 4,
    AMS_ALARM_BIT_SUPPLY5 = 5,
    AMS_ALARM_BIT_SUPPLY6 = 6,
    AMS_ALARM_BIT_RESERVED = 7,
    AMS_ALARM_BIT_SUPPLY7 = 8,
    AMS_ALARM_BIT_SUPPLY8 = 9,
    AMS_ALARM_BIT_SUPPLY9 = 10,
    AMS_ALARM_BIT_SUPPLY10 = 11,
    AMS_ALARM_BIT_VCCAMS = 12,
    AMS_ALARM_BIT_TEMP_REMOTE = 13,
    };
    enum ams_seq {
    AMS_SEQ_VCC_PSPLL = 0,
    AMS_SEQ_VCC_PSBATT = 1,
    AMS_SEQ_VCCINT = 2,
    AMS_SEQ_VCCBRAM = 3,
    AMS_SEQ_VCCAUX = 4,
    AMS_SEQ_PSDDRPLL = 5,
    AMS_SEQ_INTDDR = 6,
    };
    enum ams_ps_pl_seq {
    AMS_SEQ_CALIB = 0,
    AMS_SEQ_RSVD_1 = 1,
    AMS_SEQ_RSVD_2 = 2,
    AMS_SEQ_TEST = 3,
    AMS_SEQ_RSVD_4 = 4,
    AMS_SEQ_SUPPLY4 = 5,
    AMS_SEQ_SUPPLY5 = 6,
    AMS_SEQ_SUPPLY6 = 7,
    AMS_SEQ_TEMP = 8,
    AMS_SEQ_SUPPLY2 = 9,
    AMS_SEQ_SUPPLY1 = 10,
    AMS_SEQ_VP_VN = 11,
    AMS_SEQ_VREFP = 12,
    AMS_SEQ_VREFN = 13,
    AMS_SEQ_SUPPLY3 = 14,
    AMS_SEQ_CURRENT_MON = 15,
    AMS_SEQ_SUPPLY7 = 16,
    AMS_SEQ_SUPPLY8 = 17,
    AMS_SEQ_SUPPLY9 = 18,
    AMS_SEQ_SUPPLY10 = 19,
    AMS_SEQ_VCCAMS = 20,
    AMS_SEQ_TEMP_REMOTE = 21,
    AMS_SEQ_MAX = 22
    };

    .type = IIO_TEMP, \
    .indexed = 1, \
    .address = (_addr), \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) | \
    BIT(IIO_CHAN_INFO_SCALE) | \
    BIT(IIO_CHAN_INFO_OFFSET), \
    .event_spec = ams_temp_events, \
    .scan_index = _scan_index, \
    .num_event_specs = ARRAY_SIZE(ams_temp_events), \
    .datasheet_name = _name, \
    }

    .type = IIO_VOLTAGE, \
    .indexed = 1, \
    .address = (_addr), \
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) | \
    BIT(IIO_CHAN_INFO_SCALE), \
    .event_spec = (_alarm) ? ams_voltage_events : core::ptr::null_mut(), \
    .scan_index = _scan_index, \
    .num_event_specs = (_alarm) ? ARRAY_SIZE(ams_voltage_events) : 0, \
    .datasheet_name = _name, \
    }

    AMS_CHAN_TEMP(PS_SEQ(_scan_index), _addr, _name)

    AMS_CHAN_VOLTAGE(PS_SEQ(_scan_index), _addr, true, _name)

    AMS_CHAN_TEMP(PL_SEQ(_scan_index), _addr, _name)

    AMS_CHAN_VOLTAGE(PL_SEQ(_scan_index), _addr, _alarm, _name)

    AMS_CHAN_VOLTAGE(PL_SEQ(AMS_SEQ(_auxno)), AMS_REG_VAUX(_auxno), false, \
    "VAUX" #_auxno)

    AMS_CHAN_VOLTAGE(PL_SEQ(AMS_SEQ(AMS_SEQ(_scan_index))), _addr, false, \
    _name)
//
// struct ams - This structure contains necessary state for xilinx-ams to operate
// @base: physical base address of device
// @ps_base: physical base address of PS device
// @pl_base: physical base address of PL device
// @clk: clocks associated with the device
// @dev: pointer to device struct
// @lock: to handle multiple user interaction
// @intr_lock: to protect interrupt mask values
// @alarm_mask: alarm configuration
// @current_masked_alarm: currently masked due to alarm
// @intr_mask: interrupt configuration
// @ams_unmask_work: re-enables event once the event condition disappears
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ams {
    pub base: *mut void __iomem,
    pub ps_base: *mut void __iomem,
    pub pl_base: *mut void __iomem,
    pub clk: *mut clk,
    pub dev: *mut device,
    pub lock: mutex,
    pub intr_lock: spinlock_t,
    pub alarm_mask: c_uint,
    pub current_masked_alarm: c_uint,
    pub intr_mask: u64,
    pub ams_unmask_work: delayed_work,
}

    static inline void ams_ps_update_reg(struct ams *ams, unsigned int offset,
    u32 mask, u32 data)
    {
    u32 val, regval;
    val = readl(ams.ps_base + offset);
    regval = (val & ~mask) | (data & mask);
    writel(regval, ams.ps_base + offset);
    }
    static inline void ams_pl_update_reg(struct ams *ams, unsigned int offset,
    u32 mask, u32 data)
    {
    u32 val, regval;
    val = readl(ams.pl_base + offset);
    regval = (val & ~mask) | (data & mask);
    writel(regval, ams.pl_base + offset);
    }
#[no_mangle]
unsafe extern "C" fn ams_update_intrmask(ams: *mut ams, mask: u64, val: u64) {
    static void ams_update_intrmask(struct ams *ams, u64 mask, u64 val)
    {
    u32 regval;
    ams.intr_mask = (ams.intr_mask & ~mask) | (val & mask);
    regval = ~(ams.intr_mask | ams.current_masked_alarm);
    writel(regval, ams.base + AMS_IER_0);
    regval = ~(FIELD_GET(AMS_ISR1_INTR_MASK, ams.intr_mask));
    writel(regval, ams.base + AMS_IER_1);
    regval = ams.intr_mask | ams.current_masked_alarm;
    writel(regval, ams.base + AMS_IDR_0);
    regval = FIELD_GET(AMS_ISR1_INTR_MASK, ams.intr_mask);
    writel(regval, ams.base + AMS_IDR_1);
    }
#[no_mangle]
unsafe extern "C" fn ams_disable_all_alarms(ams: *mut ams) {
    static void ams_disable_all_alarms(struct ams *ams)
    {
// disable PS module alarm
    if (ams.ps_base) {
    ams_ps_update_reg(ams, AMS_REG_CONFIG1, AMS_REGCFG1_ALARM_MASK,
    AMS_REGCFG1_ALARM_MASK);
    ams_ps_update_reg(ams, AMS_REG_CONFIG3, AMS_REGCFG3_ALARM_MASK,
    AMS_REGCFG3_ALARM_MASK);
    }
// disable PL module alarm
    if (ams.pl_base) {
    ams_pl_update_reg(ams, AMS_REG_CONFIG1, AMS_REGCFG1_ALARM_MASK,
    AMS_REGCFG1_ALARM_MASK);
    ams_pl_update_reg(ams, AMS_REG_CONFIG3, AMS_REGCFG3_ALARM_MASK,
    AMS_REGCFG3_ALARM_MASK);
    }
    }
#[no_mangle]
unsafe extern "C" fn ams_update_ps_alarm(ams: *mut ams, alarm_mask: c_ulong) {
    static void ams_update_ps_alarm(struct ams *ams, unsigned long alarm_mask)
    {
    u32 cfg;
    u32 val;
    val = FIELD_GET(AMS_ISR0_ALARM_2_TO_0_MASK, alarm_mask);
    cfg = ~(FIELD_PREP(AMS_CONF1_ALARM_2_TO_0_MASK, val));
    val = FIELD_GET(AMS_ISR0_ALARM_6_TO_3_MASK, alarm_mask);
    cfg &= ~(FIELD_PREP(AMS_CONF1_ALARM_6_TO_3_MASK, val));
    ams_ps_update_reg(ams, AMS_REG_CONFIG1, AMS_REGCFG1_ALARM_MASK, cfg);
    val = FIELD_GET(AMS_ISR0_ALARM_12_TO_7_MASK, alarm_mask);
    cfg = ~(FIELD_PREP(AMS_CONF1_ALARM_12_TO_7_MASK, val));
    ams_ps_update_reg(ams, AMS_REG_CONFIG3, AMS_REGCFG3_ALARM_MASK, cfg);
    }
#[no_mangle]
unsafe extern "C" fn ams_update_pl_alarm(ams: *mut ams, alarm_mask: c_ulong) {
    static void ams_update_pl_alarm(struct ams *ams, unsigned long alarm_mask)
    {
    unsigned long pl_alarm_mask;
    u32 cfg;
    u32 val;
    pl_alarm_mask = FIELD_GET(AMS_PL_ALARM_MASK, alarm_mask);
    val = FIELD_GET(AMS_ISR0_ALARM_2_TO_0_MASK, pl_alarm_mask);
    cfg = ~(FIELD_PREP(AMS_CONF1_ALARM_2_TO_0_MASK, val));
    val = FIELD_GET(AMS_ISR0_ALARM_6_TO_3_MASK, pl_alarm_mask);
    cfg &= ~(FIELD_PREP(AMS_CONF1_ALARM_6_TO_3_MASK, val));
    ams_pl_update_reg(ams, AMS_REG_CONFIG1, AMS_REGCFG1_ALARM_MASK, cfg);
    val = FIELD_GET(AMS_ISR0_ALARM_12_TO_7_MASK, pl_alarm_mask);
    cfg = ~(FIELD_PREP(AMS_CONF1_ALARM_12_TO_7_MASK, val));
    ams_pl_update_reg(ams, AMS_REG_CONFIG3, AMS_REGCFG3_ALARM_MASK, cfg);
    }
#[no_mangle]
unsafe extern "C" fn ams_unmask(ams: *mut ams) {
    static void ams_unmask(struct ams *ams)
    {
    unsigned int status, unmask;
    status = readl(ams.base + AMS_ISR_0);
// Clear those bits which are not active anymore
    unmask = (ams.current_masked_alarm ^ status) & ams.current_masked_alarm;
// Clear status of disabled alarm
    unmask |= ams.intr_mask;
    ams.current_masked_alarm &= status;
// Also clear those which are masked out anyway
    ams.current_masked_alarm &= ~ams.intr_mask;
// Clear the interrupts before we unmask them
    writel(unmask, ams.base + AMS_ISR_0);
    ams_update_intrmask(ams, ~AMS_ALARM_MASK, ~AMS_ALARM_MASK);
    }
#[no_mangle]
unsafe extern "C" fn ams_update_alarm(ams: *mut ams, alarm_mask: c_ulong) {
    static void ams_update_alarm(struct ams *ams, unsigned long alarm_mask)
    {
    unsigned long flags;
    if (ams.ps_base)
    ams_update_ps_alarm(ams, alarm_mask);
    if (ams.pl_base)
    ams_update_pl_alarm(ams, alarm_mask);
    spin_lock_irqsave(&ams.intr_lock, flags);
    ams_update_intrmask(ams, AMS_ISR0_ALARM_MASK, ~alarm_mask);
    ams_unmask(ams);
    spin_unlock_irqrestore(&ams.intr_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ams_enable_channel_sequence(indio_dev: *mut iio_dev) {
    static void ams_enable_channel_sequence(struct iio_dev *indio_dev)
    {
    struct ams *ams = iio_priv(indio_dev);
    unsigned long long scan_mask;
    int i;
    u32 regval;
//
// Enable channel sequence. First 22 bits of scan_mask represent
// PS channels, and next remaining bits represent PL channels.
//
// Run calibration of PS & PL as part of the sequence
    scan_mask = BIT(0) | BIT(AMS_PS_SEQ_MAX);
    for (i = 0; i < indio_dev.num_channels; i++) {
    const struct iio_chan_spec *chan = &indio_dev.channels[i];
    if (chan.scan_index < AMS_CTRL_SEQ_BASE)
    scan_mask |= BIT_ULL(chan.scan_index);
    }
    if (ams.ps_base) {
// put sysmon in a soft reset to change the sequence
    ams_ps_update_reg(ams, AMS_REG_CONFIG1, AMS_CONF1_SEQ_MASK,
    AMS_CONF1_SEQ_DEFAULT);
// configure basic channels
    regval = FIELD_GET(AMS_REG_SEQ0_MASK, scan_mask);
    writel(regval, ams.ps_base + AMS_REG_SEQ_CH0);
    regval = FIELD_GET(AMS_REG_SEQ2_MASK, scan_mask);
    writel(regval, ams.ps_base + AMS_REG_SEQ_CH2);
// set continuous sequence mode
    ams_ps_update_reg(ams, AMS_REG_CONFIG1, AMS_CONF1_SEQ_MASK,
    AMS_CONF1_SEQ_CONTINUOUS);
    }
    if (ams.pl_base) {
// put sysmon in a soft reset to change the sequence
    ams_pl_update_reg(ams, AMS_REG_CONFIG1, AMS_CONF1_SEQ_MASK,
    AMS_CONF1_SEQ_DEFAULT);
// configure basic channels
    scan_mask = FIELD_GET(AMS_PL_SEQ_MASK, scan_mask);
    regval = FIELD_GET(AMS_REG_SEQ0_MASK, scan_mask);
    writel(regval, ams.pl_base + AMS_REG_SEQ_CH0);
    regval = FIELD_GET(AMS_REG_SEQ1_MASK, scan_mask);
    writel(regval, ams.pl_base + AMS_REG_SEQ_CH1);
    regval = FIELD_GET(AMS_REG_SEQ2_MASK, scan_mask);
    writel(regval, ams.pl_base + AMS_REG_SEQ_CH2);
// set continuous sequence mode
    ams_pl_update_reg(ams, AMS_REG_CONFIG1, AMS_CONF1_SEQ_MASK,
    AMS_CONF1_SEQ_CONTINUOUS);
    }
    }
#[no_mangle]
unsafe extern "C" fn ams_init_device(ams: *mut ams) -> c_int {
    static int ams_init_device(struct ams *ams)
    {
    let mut expect: u32 = AMS_PS_CSTS_PS_READY;
    u32 reg, value;
    int ret;
// reset AMS
    if (ams.ps_base) {
    writel(AMS_PS_RESET_VALUE, ams.ps_base + AMS_VP_VN);
    ret = readl_poll_timeout(ams.base + AMS_PS_CSTS, reg, (reg & expect),
    AMS_INIT_POLL_TIME_US, AMS_INIT_TIMEOUT_US);
    if (ret)
    return ret;
// put sysmon in a default state
    ams_ps_update_reg(ams, AMS_REG_CONFIG1, AMS_CONF1_SEQ_MASK,
    AMS_CONF1_SEQ_DEFAULT);
    }
    if (ams.pl_base) {
    value = readl(ams.base + AMS_PL_CSTS);
    if (value == 0)
    return 0;
    writel(AMS_PL_RESET_VALUE, ams.pl_base + AMS_VP_VN);
// put sysmon in a default state
    ams_pl_update_reg(ams, AMS_REG_CONFIG1, AMS_CONF1_SEQ_MASK,
    AMS_CONF1_SEQ_DEFAULT);
    }
    ams_disable_all_alarms(ams);
// Disable interrupt
    ams_update_intrmask(ams, AMS_ALARM_MASK, AMS_ALARM_MASK);
// Clear any pending interrupt
    writel(AMS_ISR0_ALARM_MASK, ams.base + AMS_ISR_0);
    writel(AMS_ISR1_ALARM_MASK, ams.base + AMS_ISR_1);
    return 0;
    }
    static int ams_read_label(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, char *label)
    {
    return sysfs_emit(label, "%s\n", chan.datasheet_name);
    }
#[no_mangle]
unsafe extern "C" fn ams_enable_single_channel(ams: *mut ams, offset: c_uint) -> c_int {
    static int ams_enable_single_channel(struct ams *ams, unsigned int offset)
    {
    u8 channel_num;
    switch (offset) {
    case AMS_VCC_PSPLL0:
    channel_num = AMS_VCC_PSPLL0_CH;
    break;
    case AMS_VCC_PSPLL3:
    channel_num = AMS_VCC_PSPLL3_CH;
    break;
    case AMS_VCCINT:
    channel_num = AMS_VCCINT_CH;
    break;
    case AMS_VCCBRAM:
    channel_num = AMS_VCCBRAM_CH;
    break;
    case AMS_VCCAUX:
    channel_num = AMS_VCCAUX_CH;
    break;
    case AMS_PSDDRPLL:
    channel_num = AMS_PSDDRPLL_CH;
    break;
    case AMS_PSINTFPDDR:
    channel_num = AMS_PSINTFPDDR_CH;
    break;
    default:
    return -EINVAL;
    }
// put sysmon in a soft reset to change the sequence
    ams_ps_update_reg(ams, AMS_REG_CONFIG1, AMS_CONF1_SEQ_MASK,
    AMS_CONF1_SEQ_DEFAULT);
// write the channel number
    ams_ps_update_reg(ams, AMS_REG_CONFIG0, AMS_CONF0_CHANNEL_NUM_MASK,
    channel_num);
// set single channel, sequencer off mode
    ams_ps_update_reg(ams, AMS_REG_CONFIG1, AMS_CONF1_SEQ_MASK,
    AMS_CONF1_SEQ_SINGLE_CHANNEL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ams_read_vcc_reg(ams: *mut ams, offset: c_uint, data: *mut u32) -> c_int {
    static int ams_read_vcc_reg(struct ams *ams, unsigned int offset, u32 *data)
    {
    let mut expect: u32 = AMS_ISR1_EOC_MASK;
    u32 reg;
    int ret;
    ret = ams_enable_single_channel(ams, offset);
    if (ret)
    return ret;
// clear end-of-conversion flag, wait for next conversion to complete
    writel(expect, ams.base + AMS_ISR_1);
    ret = readl_poll_timeout(ams.base + AMS_ISR_1, reg, (reg & expect),
    AMS_INIT_POLL_TIME_US, AMS_INIT_TIMEOUT_US);
    if (ret)
    return ret;
// data = readl(ams->base + offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ams_get_ps_scale(address: c_int) -> c_int {
    static int ams_get_ps_scale(int address)
    {
    int val;
    switch (address) {
    case AMS_SUPPLY1:
    case AMS_SUPPLY2:
    case AMS_SUPPLY3:
    case AMS_SUPPLY4:
    case AMS_SUPPLY9:
    case AMS_SUPPLY10:
    case AMS_VCCAMS:
    val = AMS_SUPPLY_SCALE_3VOLT_mV;
    break;
    case AMS_SUPPLY5:
    case AMS_SUPPLY6:
    case AMS_SUPPLY7:
    case AMS_SUPPLY8:
    val = AMS_SUPPLY_SCALE_6VOLT_mV;
    break;
    default:
    val = AMS_SUPPLY_SCALE_1VOLT_mV;
    break;
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn ams_get_pl_scale(ams: *mut ams, address: c_int) -> c_int {
    static int ams_get_pl_scale(struct ams *ams, int address)
    {
    int val, regval;
    switch (address) {
    case AMS_SUPPLY1:
    case AMS_SUPPLY2:
    case AMS_SUPPLY3:
    case AMS_SUPPLY4:
    case AMS_SUPPLY5:
    case AMS_SUPPLY6:
    case AMS_VCCAMS:
    case AMS_VREFP:
    case AMS_VREFN:
    val = AMS_SUPPLY_SCALE_3VOLT_mV;
    break;
    case AMS_SUPPLY7:
    regval = readl(ams.pl_base + AMS_REG_CONFIG4);
    if (FIELD_GET(AMS_VUSER0_MASK, regval))
    val = AMS_SUPPLY_SCALE_6VOLT_mV;
    else
    val = AMS_SUPPLY_SCALE_3VOLT_mV;
    break;
    case AMS_SUPPLY8:
    regval = readl(ams.pl_base + AMS_REG_CONFIG4);
    if (FIELD_GET(AMS_VUSER1_MASK, regval))
    val = AMS_SUPPLY_SCALE_6VOLT_mV;
    else
    val = AMS_SUPPLY_SCALE_3VOLT_mV;
    break;
    case AMS_SUPPLY9:
    regval = readl(ams.pl_base + AMS_REG_CONFIG4);
    if (FIELD_GET(AMS_VUSER2_MASK, regval))
    val = AMS_SUPPLY_SCALE_6VOLT_mV;
    else
    val = AMS_SUPPLY_SCALE_3VOLT_mV;
    break;
    case AMS_SUPPLY10:
    regval = readl(ams.pl_base + AMS_REG_CONFIG4);
    if (FIELD_GET(AMS_VUSER3_MASK, regval))
    val = AMS_SUPPLY_SCALE_6VOLT_mV;
    else
    val = AMS_SUPPLY_SCALE_3VOLT_mV;
    break;
    case AMS_VP_VN:
    case AMS_REG_VAUX(0) ... AMS_REG_VAUX(15):
    val = AMS_SUPPLY_SCALE_1VOLT_mV;
    break;
    default:
    val = AMS_SUPPLY_SCALE_1VOLT_mV;
    break;
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn ams_get_ctrl_scale(address: c_int) -> c_int {
    static int ams_get_ctrl_scale(int address)
    {
    int val;
    switch (address) {
    case AMS_VCC_PSPLL0:
    case AMS_VCC_PSPLL3:
    case AMS_VCCINT:
    case AMS_VCCBRAM:
    case AMS_VCCAUX:
    case AMS_PSDDRPLL:
    case AMS_PSINTFPDDR:
    val = AMS_SUPPLY_SCALE_3VOLT_mV;
    break;
    default:
    val = AMS_SUPPLY_SCALE_1VOLT_mV;
    break;
    }
    return val;
    }
    static int ams_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct ams *ams = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW: {
    guard(mutex)(&ams.lock);
    if (chan.scan_index >= AMS_CTRL_SEQ_BASE) {
    ret = ams_read_vcc_reg(ams, chan.address, val);
    if (ret)
    return ret;
    ams_enable_channel_sequence(indio_dev);
    } else if (chan.scan_index >= AMS_PS_SEQ_MAX)
// val = readl(ams->pl_base + chan->address);
    else
// val = readl(ams->ps_base + chan->address);
    return IIO_VAL_INT;
    }
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_VOLTAGE:
    if (chan.scan_index < AMS_PS_SEQ_MAX)
// val = ams_get_ps_scale(chan->address);
    else if (chan.scan_index >= AMS_PS_SEQ_MAX &&
    chan.scan_index < AMS_CTRL_SEQ_BASE)
// val = ams_get_pl_scale(ams, chan->address);
    else
// val = ams_get_ctrl_scale(chan->address);
// val2 = AMS_SUPPLY_SCALE_DIV_BIT;
    return IIO_VAL_FRACTIONAL_LOG2;
    case IIO_TEMP:
// val = AMS_TEMP_SCALE;
// val2 = AMS_TEMP_SCALE_DIV_BIT;
    return IIO_VAL_FRACTIONAL_LOG2;
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_OFFSET:
// Only the temperature channel has an offset
// val = AMS_TEMP_OFFSET;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ams_alarm_map {
    pub scan_index: enum ams_ps_pl_seq,
    pub base_offset: c_uint,
}

//
// Array index matches enum ams_alarm_bit.
// Entries with base_offset == AMS_ALARM_NONE are unused/invalid
// (e.g. RESERVED) and must be skipped.
//
    static const struct ams_alarm_map alarm_map[] = {
    [AMS_ALARM_BIT_TEMP] = { AMS_SEQ_TEMP, AMS_ALARM_TEMP },
    [AMS_ALARM_BIT_SUPPLY1] = { AMS_SEQ_SUPPLY1, AMS_ALARM_SUPPLY1 },
    [AMS_ALARM_BIT_SUPPLY2] = { AMS_SEQ_SUPPLY2, AMS_ALARM_SUPPLY2 },
    [AMS_ALARM_BIT_SUPPLY3] = { AMS_SEQ_SUPPLY3, AMS_ALARM_SUPPLY3 },
    [AMS_ALARM_BIT_SUPPLY4] = { AMS_SEQ_SUPPLY4, AMS_ALARM_SUPPLY4 },
    [AMS_ALARM_BIT_SUPPLY5] = { AMS_SEQ_SUPPLY5, AMS_ALARM_SUPPLY5 },
    [AMS_ALARM_BIT_SUPPLY6] = { AMS_SEQ_SUPPLY6, AMS_ALARM_SUPPLY6 },
    [AMS_ALARM_BIT_RESERVED] = { 0, AMS_ALARM_NONE },
    [AMS_ALARM_BIT_SUPPLY7] = { AMS_SEQ_SUPPLY7, AMS_ALARM_SUPPLY7 },
    [AMS_ALARM_BIT_SUPPLY8] = { AMS_SEQ_SUPPLY8, AMS_ALARM_SUPPLY8 },
    [AMS_ALARM_BIT_SUPPLY9] = { AMS_SEQ_SUPPLY9, AMS_ALARM_SUPPLY9 },
    [AMS_ALARM_BIT_SUPPLY10] = { AMS_SEQ_SUPPLY10, AMS_ALARM_SUPPLY10 },
    [AMS_ALARM_BIT_VCCAMS] = { AMS_SEQ_VCCAMS, AMS_ALARM_VCCAMS },
    [AMS_ALARM_BIT_TEMP_REMOTE] = { AMS_SEQ_TEMP_REMOTE, AMS_ALARM_TEMP_REMOTE },
    };
#[no_mangle]
unsafe extern "C" fn ams_scan_index_to_event(scan_index: c_int) -> c_int {
    static int ams_scan_index_to_event(int scan_index)
    {
    for (unsigned int i = 0; i < ARRAY_SIZE(alarm_map); i++) {
    if (alarm_map[i].base_offset == AMS_ALARM_NONE)
    continue;
    if (alarm_map[i].scan_index == scan_index)
    return i;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn ams_get_alarm_offset(scan_index: c_int, dir: enum iio_event_direction) -> c_int {
    static int ams_get_alarm_offset(int scan_index, enum iio_event_direction dir)
    {
    int offset, event;
    if (scan_index >= AMS_PS_SEQ_MAX)
    scan_index -= AMS_PS_SEQ_MAX;
    if (dir == IIO_EV_DIR_FALLING) {
    if (scan_index < AMS_SEQ_SUPPLY7)
    offset = AMS_ALARM_THRESHOLD_OFF_10;
    else
    offset = AMS_ALARM_THRESHOLD_OFF_20;
    } else {
    offset = 0;
    }
    event = ams_scan_index_to_event(scan_index);
    if (event < 0 || alarm_map[event].base_offset == AMS_ALARM_NONE)
    return 0;
    return alarm_map[event].base_offset + offset;
    }
    static const struct iio_chan_spec *ams_event_to_channel(struct iio_dev *dev,
    u32 event)
    {
    let mut scan_index: c_int = 0, i;
    if (event >= AMS_PL_ALARM_START) {
    event -= AMS_PL_ALARM_START;
    scan_index = AMS_PS_SEQ_MAX;
    }
    if (event >= ARRAY_SIZE(alarm_map))
    return core::ptr::null_mut();
    if (alarm_map[event].base_offset == AMS_ALARM_NONE)
    return core::ptr::null_mut();
    scan_index += alarm_map[event].scan_index;
    for (i = 0; i < dev.num_channels; i++)
    if (dev.channels[i].scan_index == scan_index)
    break;
    if (i == dev.num_channels)
    return core::ptr::null_mut();
    return &dev.channels[i];
    }
#[no_mangle]
unsafe extern "C" fn ams_get_alarm_mask(scan_index: c_int) -> c_int {
    static int ams_get_alarm_mask(int scan_index)
    {
    let mut bit: c_int = 0, event;
    if (scan_index >= AMS_PS_SEQ_MAX) {
    bit = AMS_PL_ALARM_START;
    scan_index -= AMS_PS_SEQ_MAX;
    }
    event = ams_scan_index_to_event(scan_index);
    if (event < 0)
    return 0;
    return BIT(event + bit);
    }
    static int ams_read_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir)
    {
    struct ams *ams = iio_priv(indio_dev);
    return !!(ams.alarm_mask & ams_get_alarm_mask(chan.scan_index));
    }
    static int ams_write_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    bool state)
    {
    struct ams *ams = iio_priv(indio_dev);
    unsigned int alarm;
    alarm = ams_get_alarm_mask(chan.scan_index);
    guard(mutex)(&ams.lock);
    if (state)
    ams.alarm_mask |= alarm;
    else
    ams.alarm_mask &= ~alarm;
    ams_update_alarm(ams, ams.alarm_mask);
    return 0;
    }
    static int ams_read_event_value(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info, int *val, int *val2)
    {
    struct ams *ams = iio_priv(indio_dev);
    let mut offset: c_uint = ams_get_alarm_offset(chan.scan_index, dir);
    guard(mutex)(&ams.lock);
    if (chan.scan_index >= AMS_PS_SEQ_MAX)
// val = readl(ams->pl_base + offset);
    else
// val = readl(ams->ps_base + offset);
    return IIO_VAL_INT;
    }
    static int ams_write_event_value(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info, int val, int val2)
    {
    struct ams *ams = iio_priv(indio_dev);
    unsigned int offset;
    guard(mutex)(&ams.lock);
// Set temperature channel threshold to direct threshold
    if (chan.type == IIO_TEMP) {
    offset = ams_get_alarm_offset(chan.scan_index, IIO_EV_DIR_FALLING);
    if (chan.scan_index >= AMS_PS_SEQ_MAX)
    ams_pl_update_reg(ams, offset,
    AMS_ALARM_THR_DIRECT_MASK,
    AMS_ALARM_THR_DIRECT_MASK);
    else
    ams_ps_update_reg(ams, offset,
    AMS_ALARM_THR_DIRECT_MASK,
    AMS_ALARM_THR_DIRECT_MASK);
    }
    offset = ams_get_alarm_offset(chan.scan_index, dir);
    if (chan.scan_index >= AMS_PS_SEQ_MAX)
    writel(val, ams.pl_base + offset);
    else
    writel(val, ams.ps_base + offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ams_handle_event(indio_dev: *mut iio_dev, event: u32) {
    static void ams_handle_event(struct iio_dev *indio_dev, u32 event)
    {
    const struct iio_chan_spec *chan;
    chan = ams_event_to_channel(indio_dev, event);
    if (!chan)
    return;
    if (chan.type == IIO_TEMP) {
//
// The temperature channel only supports over-temperature
// events.
//
    iio_push_event(indio_dev,
    IIO_UNMOD_EVENT_CODE(chan.type, chan.channel,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_RISING),
    iio_get_time_ns(indio_dev));
    } else {
//
// For other channels we don't know whether it is a upper or
// lower threshold event. Userspace will have to check the
// channel value if it wants to know.
//
    iio_push_event(indio_dev,
    IIO_UNMOD_EVENT_CODE(chan.type, chan.channel,
    IIO_EV_TYPE_THRESH,
    IIO_EV_DIR_EITHER),
    iio_get_time_ns(indio_dev));
    }
    }
#[no_mangle]
unsafe extern "C" fn ams_handle_events(indio_dev: *mut iio_dev, events: c_ulong) {
    static void ams_handle_events(struct iio_dev *indio_dev, unsigned long events)
    {
    unsigned int bit;
    for_each_set_bit(bit, &events, AMS_NO_OF_ALARMS)
    ams_handle_event(indio_dev, bit);
    }
//
// ams_unmask_worker - ams alarm interrupt unmask worker
// @work: work to be done
//
// The ZynqMP threshold interrupts are level sensitive. Since we can't make the
// threshold condition go way from within the interrupt handler, this means as
// soon as a threshold condition is present we would enter the interrupt handler
// again and again. To work around this we mask all active threshold interrupts
// in the interrupt handler and start a timer. In this timer we poll the
// interrupt status and only if the interrupt is inactive we unmask it again.
//
#[no_mangle]
unsafe extern "C" fn ams_unmask_worker(work: *mut work_struct) {
    static void ams_unmask_worker(struct work_struct *work)
    {
    struct ams *ams = container_of(work, struct ams, ams_unmask_work.work);
    spin_lock_irq(&ams.intr_lock);
    ams_unmask(ams);
    spin_unlock_irq(&ams.intr_lock);
// If still pending some alarm re-trigger the timer
    if (ams.current_masked_alarm)
    schedule_delayed_work(&ams.ams_unmask_work,
    msecs_to_jiffies(AMS_UNMASK_TIMEOUT_MS));
    }
#[no_mangle]
unsafe extern "C" fn ams_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ams_irq(int irq, void *data)
    {
    struct iio_dev *indio_dev = data;
    struct ams *ams = iio_priv(indio_dev);
    u32 isr0;
    spin_lock(&ams.intr_lock);
    isr0 = readl(ams.base + AMS_ISR_0);
// Only process alarms that are not masked
    isr0 &= ~((ams.intr_mask & AMS_ISR0_ALARM_MASK) | ams.current_masked_alarm);
    if (!isr0) {
    spin_unlock(&ams.intr_lock);
    return IRQ_NONE;
    }
// Clear interrupt
    writel(isr0, ams.base + AMS_ISR_0);
// Mask the alarm interrupts until cleared
    ams.current_masked_alarm |= isr0;
    ams_update_intrmask(ams, ~AMS_ALARM_MASK, ~AMS_ALARM_MASK);
    ams_handle_events(indio_dev, isr0);
    schedule_delayed_work(&ams.ams_unmask_work,
    msecs_to_jiffies(AMS_UNMASK_TIMEOUT_MS));
    spin_unlock(&ams.intr_lock);
    return IRQ_HANDLED;
    }
    static const struct iio_event_spec ams_temp_events[] = {
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_RISING,
    .mask_separate = BIT(IIO_EV_INFO_ENABLE) | BIT(IIO_EV_INFO_VALUE),
    },
    };
    static const struct iio_event_spec ams_voltage_events[] = {
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_RISING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE),
    },
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_FALLING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE),
    },
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_EITHER,
    .mask_separate = BIT(IIO_EV_INFO_ENABLE),
    },
    };
    static const struct iio_chan_spec ams_ps_channels[] = {
    AMS_PS_CHAN_TEMP(AMS_SEQ_TEMP, AMS_TEMP, "Temp_LPD"),
    AMS_PS_CHAN_TEMP(AMS_SEQ_TEMP_REMOTE, AMS_TEMP_REMOTE, "Temp_FPD"),
    AMS_PS_CHAN_VOLTAGE(AMS_SEQ_SUPPLY1, AMS_SUPPLY1, "VCC_PSINTLP"),
    AMS_PS_CHAN_VOLTAGE(AMS_SEQ_SUPPLY2, AMS_SUPPLY2, "VCC_PSINTFP"),
    AMS_PS_CHAN_VOLTAGE(AMS_SEQ_SUPPLY3, AMS_SUPPLY3, "VCC_PSAUX"),
    AMS_PS_CHAN_VOLTAGE(AMS_SEQ_SUPPLY4, AMS_SUPPLY4, "VCC_PSDDR"),
    AMS_PS_CHAN_VOLTAGE(AMS_SEQ_SUPPLY5, AMS_SUPPLY5, "VCC_PSIO3"),
    AMS_PS_CHAN_VOLTAGE(AMS_SEQ_SUPPLY6, AMS_SUPPLY6, "VCC_PSIO0"),
    AMS_PS_CHAN_VOLTAGE(AMS_SEQ_SUPPLY7, AMS_SUPPLY7, "VCC_PSIO1"),
    AMS_PS_CHAN_VOLTAGE(AMS_SEQ_SUPPLY8, AMS_SUPPLY8, "VCC_PSIO2"),
    AMS_PS_CHAN_VOLTAGE(AMS_SEQ_SUPPLY9, AMS_SUPPLY9, "PS_MGTRAVCC"),
    AMS_PS_CHAN_VOLTAGE(AMS_SEQ_SUPPLY10, AMS_SUPPLY10, "PS_MGTRAVTT"),
    AMS_PS_CHAN_VOLTAGE(AMS_SEQ_VCCAMS, AMS_VCCAMS, "VCC_PSADC"),
    };
    static const struct iio_chan_spec ams_pl_channels[] = {
    AMS_PL_CHAN_TEMP(AMS_SEQ_TEMP, AMS_TEMP, "Temp_PL"),
    AMS_PL_CHAN_VOLTAGE(AMS_SEQ_SUPPLY1, AMS_SUPPLY1, true, "VCCINT"),
    AMS_PL_CHAN_VOLTAGE(AMS_SEQ_SUPPLY2, AMS_SUPPLY2, true, "VCCAUX"),
    AMS_PL_CHAN_VOLTAGE(AMS_SEQ_VREFP, AMS_VREFP, false, "VREFP"),
    AMS_PL_CHAN_VOLTAGE(AMS_SEQ_VREFN, AMS_VREFN, false, "VREFN"),
    AMS_PL_CHAN_VOLTAGE(AMS_SEQ_SUPPLY3, AMS_SUPPLY3, true, "VCCBRAM"),
    AMS_PL_CHAN_VOLTAGE(AMS_SEQ_SUPPLY4, AMS_SUPPLY4, true, "VCC_PSINTLP"),
    AMS_PL_CHAN_VOLTAGE(AMS_SEQ_SUPPLY5, AMS_SUPPLY5, true, "VCC_PSINTFP"),
    AMS_PL_CHAN_VOLTAGE(AMS_SEQ_SUPPLY6, AMS_SUPPLY6, true, "VCC_PSAUX"),
    AMS_PL_CHAN_VOLTAGE(AMS_SEQ_VCCAMS, AMS_VCCAMS, true, "VCCAMS"),
    AMS_PL_CHAN_VOLTAGE(AMS_SEQ_VP_VN, AMS_VP_VN, false, "VP_VN"),
    AMS_PL_CHAN_VOLTAGE(AMS_SEQ_SUPPLY7, AMS_SUPPLY7, true, "VUser0"),
    AMS_PL_CHAN_VOLTAGE(AMS_SEQ_SUPPLY8, AMS_SUPPLY8, true, "VUser1"),
    AMS_PL_CHAN_VOLTAGE(AMS_SEQ_SUPPLY9, AMS_SUPPLY9, true, "VUser2"),
    AMS_PL_CHAN_VOLTAGE(AMS_SEQ_SUPPLY10, AMS_SUPPLY10, true, "VUser3"),
    AMS_PL_AUX_CHAN_VOLTAGE(0),
    AMS_PL_AUX_CHAN_VOLTAGE(1),
    AMS_PL_AUX_CHAN_VOLTAGE(2),
    AMS_PL_AUX_CHAN_VOLTAGE(3),
    AMS_PL_AUX_CHAN_VOLTAGE(4),
    AMS_PL_AUX_CHAN_VOLTAGE(5),
    AMS_PL_AUX_CHAN_VOLTAGE(6),
    AMS_PL_AUX_CHAN_VOLTAGE(7),
    AMS_PL_AUX_CHAN_VOLTAGE(8),
    AMS_PL_AUX_CHAN_VOLTAGE(9),
    AMS_PL_AUX_CHAN_VOLTAGE(10),
    AMS_PL_AUX_CHAN_VOLTAGE(11),
    AMS_PL_AUX_CHAN_VOLTAGE(12),
    AMS_PL_AUX_CHAN_VOLTAGE(13),
    AMS_PL_AUX_CHAN_VOLTAGE(14),
    AMS_PL_AUX_CHAN_VOLTAGE(15),
    };
    static const struct iio_chan_spec ams_ctrl_channels[] = {
    AMS_CTRL_CHAN_VOLTAGE(AMS_SEQ_VCC_PSPLL, AMS_VCC_PSPLL0, "VCC_PSPLL"),
    AMS_CTRL_CHAN_VOLTAGE(AMS_SEQ_VCC_PSBATT, AMS_VCC_PSPLL3, "VCC_PSBATT"),
    AMS_CTRL_CHAN_VOLTAGE(AMS_SEQ_VCCINT, AMS_VCCINT, "VCCINT"),
    AMS_CTRL_CHAN_VOLTAGE(AMS_SEQ_VCCBRAM, AMS_VCCBRAM, "VCCBRAM"),
    AMS_CTRL_CHAN_VOLTAGE(AMS_SEQ_VCCAUX, AMS_VCCAUX, "VCCAUX"),
    AMS_CTRL_CHAN_VOLTAGE(AMS_SEQ_PSDDRPLL, AMS_PSDDRPLL, "VCC_PSDDR_PLL"),
    AMS_CTRL_CHAN_VOLTAGE(AMS_SEQ_INTDDR, AMS_PSINTFPDDR, "VCC_PSINTFP_DDR"),
    };
    static int ams_get_ext_chan(struct fwnode_handle *chan_node,
    struct iio_chan_spec *channels, int num_channels)
    {
    struct iio_chan_spec *chan;
    struct fwnode_handle *child;
    unsigned int reg, ext_chan;
    int ret;
    fwnode_for_each_child_node(chan_node, child) {
    ret = fwnode_property_read_u32(child, "reg", &reg);
    if (ret || reg > AMS_PL_MAX_EXT_CHANNEL + 30)
    continue;
    chan = &channels[num_channels];
    ext_chan = reg + AMS_PL_MAX_FIXED_CHANNEL - 30;
    memcpy(chan, &ams_pl_channels[ext_chan], sizeof(*channels));
    if (fwnode_property_read_bool(child, "xlnx,bipolar"))
    chan.scan_type.sign = 's';
    num_channels++;
    }
    return num_channels;
    }
#[no_mangle]
unsafe extern "C" fn ams_iounmap_ps(data: *mut c_void) {
    static void ams_iounmap_ps(void *data)
    {
    struct ams *ams = data;
    iounmap(ams.ps_base);
    }
#[no_mangle]
unsafe extern "C" fn ams_iounmap_pl(data: *mut c_void) {
    static void ams_iounmap_pl(void *data)
    {
    struct ams *ams = data;
    iounmap(ams.pl_base);
    }
    static int ams_init_module(struct iio_dev *indio_dev,
    struct fwnode_handle *fwnode,
    struct iio_chan_spec *channels)
    {
    struct device *dev = indio_dev.dev.parent;
    struct ams *ams = iio_priv(indio_dev);
    let mut num_channels: c_int = 0;
    int ret;
    if (fwnode_device_is_compatible(fwnode, "xlnx,zynqmp-ams-ps")) {
    ams.ps_base = fwnode_iomap(fwnode, 0);
    if (!ams.ps_base)
    return -ENXIO;
    ret = devm_add_action_or_reset(dev, ams_iounmap_ps, ams);
    if (ret < 0)
    return ret;
// add PS channels to iio device channels
    memcpy(channels, ams_ps_channels, sizeof(ams_ps_channels));
    num_channels = ARRAY_SIZE(ams_ps_channels);
    } else if (fwnode_device_is_compatible(fwnode, "xlnx,zynqmp-ams-pl")) {
    ams.pl_base = fwnode_iomap(fwnode, 0);
    if (!ams.pl_base)
    return -ENXIO;
    ret = devm_add_action_or_reset(dev, ams_iounmap_pl, ams);
    if (ret < 0)
    return ret;
// Copy only first 10 fix channels
    memcpy(channels, ams_pl_channels, AMS_PL_MAX_FIXED_CHANNEL * sizeof(*channels));
    num_channels += AMS_PL_MAX_FIXED_CHANNEL;
    num_channels = ams_get_ext_chan(fwnode, channels,
    num_channels);
    } else if (fwnode_device_is_compatible(fwnode, "xlnx,zynqmp-ams")) {
// add AMS channels to iio device channels
    memcpy(channels, ams_ctrl_channels, sizeof(ams_ctrl_channels));
    num_channels += ARRAY_SIZE(ams_ctrl_channels);
    } else {
    return -EINVAL;
    }
    return num_channels;
    }
#[no_mangle]
unsafe extern "C" fn ams_parse_firmware(indio_dev: *mut iio_dev) -> c_int {
    static int ams_parse_firmware(struct iio_dev *indio_dev)
    {
    struct ams *ams = iio_priv(indio_dev);
    struct iio_chan_spec *ams_channels, *dev_channels;
    struct device *dev = indio_dev.dev.parent;
    struct fwnode_handle *fwnode = dev_fwnode(dev);
    size_t ams_size;
    int ret, ch_cnt = 0, i, rising_off, falling_off;
    let mut num_channels: c_uint = 0;
    ams_size = ARRAY_SIZE(ams_ps_channels) + ARRAY_SIZE(ams_pl_channels) +
    ARRAY_SIZE(ams_ctrl_channels);
// Initialize buffer for channel specification
    ams_channels = devm_kcalloc(dev, ams_size, sizeof(*ams_channels), GFP_KERNEL);
    if (!ams_channels)
    return -ENOMEM;
    if (fwnode_device_is_available(fwnode)) {
    ret = ams_init_module(indio_dev, fwnode, ams_channels);
    if (ret < 0)
    return ret;
    num_channels += ret;
    }
    device_for_each_child_node_scoped(dev, child) {
    ret = ams_init_module(indio_dev, child, ams_channels + num_channels);
    if (ret < 0)
    return ret;
    num_channels += ret;
    }
    for (i = 0; i < num_channels; i++) {
    ams_channels[i].channel = ch_cnt++;
    if (ams_channels[i].scan_index < AMS_CTRL_SEQ_BASE) {
// set threshold to max and min for each channel
    falling_off =
    ams_get_alarm_offset(ams_channels[i].scan_index,
    IIO_EV_DIR_FALLING);
    rising_off =
    ams_get_alarm_offset(ams_channels[i].scan_index,
    IIO_EV_DIR_RISING);
    if (ams_channels[i].scan_index >= AMS_PS_SEQ_MAX) {
    writel(AMS_ALARM_THR_MIN,
    ams.pl_base + falling_off);
    writel(AMS_ALARM_THR_MAX,
    ams.pl_base + rising_off);
    } else {
    writel(AMS_ALARM_THR_MIN,
    ams.ps_base + falling_off);
    writel(AMS_ALARM_THR_MAX,
    ams.ps_base + rising_off);
    }
    }
    }
    dev_channels = devm_krealloc_array(dev, ams_channels, num_channels,
    sizeof(*dev_channels), GFP_KERNEL);
    if (!dev_channels)
    return -ENOMEM;
    indio_dev.channels = dev_channels;
    indio_dev.num_channels = num_channels;
    return 0;
    }
    static const struct iio_info iio_ams_info = {
    .read_label = ams_read_label,
    .read_raw = &ams_read_raw,
    .read_event_config = &ams_read_event_config,
    .write_event_config = &ams_write_event_config,
    .read_event_value = &ams_read_event_value,
    .write_event_value = &ams_write_event_value,
    };
    static const struct of_device_id ams_of_match_table[] = {
    { .compatible = "xlnx,zynqmp-ams" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ams_of_match_table);
#[no_mangle]
unsafe extern "C" fn ams_probe(pdev: *mut platform_device) -> c_int {
    static int ams_probe(struct platform_device *pdev)
    {
    struct iio_dev *indio_dev;
    struct ams *ams;
    int ret;
    int irq;
    indio_dev = devm_iio_device_alloc(&pdev.dev, sizeof(*ams));
    if (!indio_dev)
    return -ENOMEM;
    ams = iio_priv(indio_dev);
    mutex_init(&ams.lock);
    spin_lock_init(&ams.intr_lock);
    indio_dev.name = "xilinx-ams";
    indio_dev.info = &iio_ams_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    ams.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ams.base))
    return PTR_ERR(ams.base);
    ams.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(ams.clk))
    return PTR_ERR(ams.clk);
    ret = devm_delayed_work_autocancel(&pdev.dev, &ams.ams_unmask_work,
    ams_unmask_worker);
    if (ret < 0)
    return ret;
    ret = ams_parse_firmware(indio_dev);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "failure in parsing DT\n");
    ret = ams_init_device(ams);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "failed to initialize AMS\n");
    ams_enable_channel_sequence(indio_dev);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(&pdev.dev, irq, &ams_irq, 0, "ams-irq",
    indio_dev);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, ret, "failed to register interrupt\n");
    platform_set_drvdata(pdev, indio_dev);
    return devm_iio_device_register(&pdev.dev, indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn ams_suspend(dev: *mut device) -> c_int {
    static int ams_suspend(struct device *dev)
    {
    struct ams *ams = iio_priv(dev_get_drvdata(dev));
    clk_disable_unprepare(ams.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ams_resume(dev: *mut device) -> c_int {
    static int ams_resume(struct device *dev)
    {
    struct ams *ams = iio_priv(dev_get_drvdata(dev));
    return clk_prepare_enable(ams.clk);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(ams_pm_ops, ams_suspend, ams_resume);
    static struct platform_driver ams_driver = {
    .probe = ams_probe,
    .driver = {
    .name = "xilinx-ams",
    .pm = pm_sleep_ptr(&ams_pm_ops),
    .of_match_table = ams_of_match_table,
    },
    };
    module_platform_driver(ams_driver);
    MODULE_DESCRIPTION("Xilinx AMS driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Xilinx, Inc.");

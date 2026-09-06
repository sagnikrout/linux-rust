//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ade9000.c
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
// ADE9000 driver
//
// Copyright 2025 Analog Devices Inc.
//

// Address of ADE9000 registers
pub const ADE9000_REG_AIGAIN: c_uint = 0x000;
pub const ADE9000_REG_AVGAIN: c_uint = 0x00B;
pub const ADE9000_REG_AIRMSOS: c_uint = 0x00C;
pub const ADE9000_REG_AVRMSOS: c_uint = 0x00D;
pub const ADE9000_REG_APGAIN: c_uint = 0x00E;
pub const ADE9000_REG_AWATTOS: c_uint = 0x00F;
pub const ADE9000_REG_AVAROS: c_uint = 0x010;
pub const ADE9000_REG_AFVAROS: c_uint = 0x012;
pub const ADE9000_REG_CONFIG0: c_uint = 0x060;
pub const ADE9000_REG_DICOEFF: c_uint = 0x072;
pub const ADE9000_REG_AI_PCF: c_uint = 0x20A;
pub const ADE9000_REG_AV_PCF: c_uint = 0x20B;
pub const ADE9000_REG_AIRMS: c_uint = 0x20C;
pub const ADE9000_REG_AVRMS: c_uint = 0x20D;
pub const ADE9000_REG_AWATT: c_uint = 0x210;
pub const ADE9000_REG_AVAR: c_uint = 0x211;
pub const ADE9000_REG_AVA: c_uint = 0x212;
pub const ADE9000_REG_AFVAR: c_uint = 0x214;
pub const ADE9000_REG_APF: c_uint = 0x216;
pub const ADE9000_REG_BI_PCF: c_uint = 0x22A;
pub const ADE9000_REG_BV_PCF: c_uint = 0x22B;
pub const ADE9000_REG_BIRMS: c_uint = 0x22C;
pub const ADE9000_REG_BVRMS: c_uint = 0x22D;
pub const ADE9000_REG_CI_PCF: c_uint = 0x24A;
pub const ADE9000_REG_CV_PCF: c_uint = 0x24B;
pub const ADE9000_REG_CIRMS: c_uint = 0x24C;
pub const ADE9000_REG_CVRMS: c_uint = 0x24D;
pub const ADE9000_REG_AWATT_ACC: c_uint = 0x2E5;
pub const ADE9000_REG_AWATTHR_LO: c_uint = 0x2E6;
pub const ADE9000_REG_AVAHR_LO: c_uint = 0x2FA;
pub const ADE9000_REG_AFVARHR_LO: c_uint = 0x30E;
pub const ADE9000_REG_BWATTHR_LO: c_uint = 0x322;
pub const ADE9000_REG_BVAHR_LO: c_uint = 0x336;
pub const ADE9000_REG_BFVARHR_LO: c_uint = 0x34A;
pub const ADE9000_REG_CWATTHR_LO: c_uint = 0x35E;
pub const ADE9000_REG_CVAHR_LO: c_uint = 0x372;
pub const ADE9000_REG_CFVARHR_LO: c_uint = 0x386;
pub const ADE9000_REG_STATUS0: c_uint = 0x402;
pub const ADE9000_REG_STATUS1: c_uint = 0x403;
pub const ADE9000_REG_MASK0: c_uint = 0x405;
pub const ADE9000_REG_MASK1: c_uint = 0x406;
pub const ADE9000_REG_EVENT_MASK: c_uint = 0x407;
pub const ADE9000_REG_VLEVEL: c_uint = 0x40F;
pub const ADE9000_REG_DIP_LVL: c_uint = 0x410;
pub const ADE9000_REG_DIPA: c_uint = 0x411;
pub const ADE9000_REG_DIPB: c_uint = 0x412;
pub const ADE9000_REG_DIPC: c_uint = 0x413;
pub const ADE9000_REG_SWELL_LVL: c_uint = 0x414;
pub const ADE9000_REG_SWELLA: c_uint = 0x415;
pub const ADE9000_REG_SWELLB: c_uint = 0x416;
pub const ADE9000_REG_SWELLC: c_uint = 0x417;
pub const ADE9000_REG_APERIOD: c_uint = 0x418;
pub const ADE9000_REG_BPERIOD: c_uint = 0x419;
pub const ADE9000_REG_CPERIOD: c_uint = 0x41A;
pub const ADE9000_REG_RUN: c_uint = 0x480;
pub const ADE9000_REG_CONFIG1: c_uint = 0x481;
pub const ADE9000_REG_ACCMODE: c_uint = 0x492;
pub const ADE9000_REG_CONFIG3: c_uint = 0x493;
pub const ADE9000_REG_ZXTOUT: c_uint = 0x498;
pub const ADE9000_REG_ZX_LP_SEL: c_uint = 0x49A;
pub const ADE9000_REG_WFB_CFG: c_uint = 0x4A0;
pub const ADE9000_REG_WFB_PG_IRQEN: c_uint = 0x4A1;
pub const ADE9000_REG_WFB_TRG_CFG: c_uint = 0x4A2;
pub const ADE9000_REG_WFB_TRG_STAT: c_uint = 0x4A3;
pub const ADE9000_REG_CONFIG2: c_uint = 0x4AF;
pub const ADE9000_REG_EP_CFG: c_uint = 0x4B0;
pub const ADE9000_REG_EGY_TIME: c_uint = 0x4B2;
pub const ADE9000_REG_PGA_GAIN: c_uint = 0x4B9;
pub const ADE9000_REG_VERSION: c_uint = 0x4FE;
pub const ADE9000_REG_WF_BUFF: c_uint = 0x800;
pub const ADE9000_REG_WF_HALF_BUFF: c_uint = 0xC00;

// External reference selection bit in CONFIG1

//
// Configuration registers
//
pub const ADE9000_PGA_GAIN: c_uint = 0x0000;
// Default configuration
pub const ADE9000_CONFIG0: c_uint = 0x00000000;
// CF3/ZX pin outputs Zero crossing, CF4 = DREADY
pub const ADE9000_CONFIG1: c_uint = 0x000E;
// Default High pass corner frequency of 1.25Hz
pub const ADE9000_CONFIG2: c_uint = 0x0A00;
// Peak and overcurrent detection disabled
pub const ADE9000_CONFIG3: c_uint = 0x0000;
//
// 50Hz operation, 3P4W Wye configuration, signed accumulation
// 3P4W Wye = 3-Phase 4-Wire star configuration (3 phases + neutral wire)
// Clear bit 8 i.e. ACCMODE=0x00xx for 50Hz operation
// ACCMODE=0x0x9x for 3Wire delta when phase B is used as reference
// 3Wire delta = 3-Phase 3-Wire triangle configuration (3 phases, no neutral)
//
pub const ADE9000_ACCMODE: c_uint = 0x0000;
pub const ADE9000_ACCMODE_60HZ: c_uint = 0x0100;
// Line period and zero crossing obtained from VA
pub const ADE9000_ZX_LP_SEL: c_uint = 0x0000;
// Interrupt mask values for initialization
pub const ADE9000_MASK0_ALL_INT_DIS: c_int = 0;
pub const ADE9000_MASK1_ALL_INT_DIS: c_uint = 0x00000000;
// Events disabled
pub const ADE9000_EVENT_DISABLE: c_uint = 0x00000000;
//
// Assuming Vnom=1/2 of full scale.
// Refer to Technical reference manual for detailed calculations.
//
pub const ADE9000_VLEVEL: c_uint = 0x0022EA28;
// Set DICOEFF= 0xFFFFE000 when integrator is enabled
pub const ADE9000_DICOEFF: c_uint = 0x00000000;
// DSP ON
pub const ADE9000_RUN_ON: c_uint = 0xFFFFFFFF;
//
// Energy Accumulation Settings
// Enable energy accumulation, accumulate samples at 8ksps
// latch energy accumulation after EGYRDY
// If accumulation is changed to half line cycle mode, change EGY_TIME
//
pub const ADE9000_EP_CFG: c_uint = 0x0011;
// Accumulate 4000 samples
pub const ADE9000_EGY_TIME: c_int = 7999;
//
// Constant Definitions
// ADE9000 FDSP: 8000sps, ADE9000 FDSP: 4000sps
//
pub const ADE9000_FDSP: c_int = 4000;
pub const ADE9000_DEFAULT_CLK_FREQ_HZ: c_int = 24576000;
pub const ADE9000_WFB_CFG: c_uint = 0x03E9;
pub const ADE9000_WFB_PAGE_SIZE: c_int = 128;
pub const ADE9000_WFB_NR_OF_PAGES: c_int = 16;
pub const ADE9000_WFB_MAX_CHANNELS: c_int = 8;
pub const ADE9000_WFB_BYTES_IN_SAMPLE: c_int = 4;

    (ADE9000_WFB_PAGE_SIZE / ADE9000_WFB_MAX_CHANNELS)

    (ADE9000_WFB_SAMPLES_IN_PAGE * ADE9000_WFB_NR_OF_PAGES)

    (ADE9000_WFB_PAGE_SIZE * ADE9000_WFB_NR_OF_PAGES)

    (ADE9000_WFB_FULL_BUFF_NR_SAMPLES * ADE9000_WFB_BYTES_IN_SAMPLE)

// Status and Mask register bits

pub const ADE9000_ST1_CROSSING_FIRST: c_int = 6;
pub const ADE9000_ST1_CROSSING_DEPTH: c_int = 25;

// Stop when waveform buffer is full
pub const ADE9000_WFB_FULL_MODE: c_uint = 0x0;
// Continuous fill—stop only on enabled trigger events
pub const ADE9000_WFB_EN_TRIG_MODE: c_uint = 0x1;
// Continuous filling—center capture around enabled trigger events
pub const ADE9000_WFB_C_EN_TRIG_MODE: c_uint = 0x2;
// Continuous fill—used as streaming mode for continuous data output
pub const ADE9000_WFB_STREAMING_MODE: c_uint = 0x3;

//
// Full scale Codes referred from Datasheet. Respective digital codes are
// produced when ADC inputs are at full scale.
//
pub const ADE9000_RMS_FULL_SCALE_CODES: c_int = 52866837;
pub const ADE9000_WATT_FULL_SCALE_CODES: c_int = 20694066;
pub const ADE9000_PCF_FULL_SCALE_CODES: c_int = 74770000;
// Phase and channel definitions
pub const ADE9000_PHASE_A_NR: c_int = 0;
pub const ADE9000_PHASE_B_NR: c_int = 1;
pub const ADE9000_PHASE_C_NR: c_int = 2;

// Waveform buffer configuration values
    enum ade9000_wfb_cfg {
    ADE9000_WFB_CFG_ALL_CHAN = 0x0,
    ADE9000_WFB_CFG_IA_VA = 0x1,
    ADE9000_WFB_CFG_IB_VB = 0x2,
    ADE9000_WFB_CFG_IC_VC = 0x3,
    ADE9000_WFB_CFG_IA = 0x8,
    ADE9000_WFB_CFG_VA = 0x9,
    ADE9000_WFB_CFG_IB = 0xA,
    ADE9000_WFB_CFG_VB = 0xB,
    ADE9000_WFB_CFG_IC = 0xC,
    ADE9000_WFB_CFG_VC = 0xD,
    };

pub const ADE9000_MAX_PHASE_NR: c_int = 3;
//
// Calculate register address for multi-phase device.
// Phase A (chan 0): base address + 0x00
// Phase B (chan 1): base address + 0x20
// Phase C (chan 2): base address + 0x40
//

    (((chan) == 0 ? 0 : (chan) == 1 ? 2 : 4) << 4 | (addr))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ade9000_state {
    pub reset_completion: completion,
    pub /: *mut *mut mutex lock; / Protects SPI transactions,
    pub wf_src: u8,
    pub wfb_trg: u32,
    pub wfb_nr_activ_chan: u8,
    pub wfb_nr_samples: u32,
    pub spi: *mut spi_device,
    pub clkin: *mut clk,
    pub xfer: [spi_transfer; 2],
    pub spi_msg: spi_message,
    pub regmap: *mut regmap,
    union{
    pub byte: [u8; ADE9000_WFB_FULL_BUFF_SIZE],
    pub word: [__be32; ADE9000_WFB_FULL_BUFF_NR_SAMPLES],
    pub __aligned(IIO_DMA_MINALIGN): } rx_buff,
    pub __aligned(IIO_DMA_MINALIGN): u8 tx_buff[2],
    pub bulk_read_buf: [c_uint; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ade9000_irq1_event {
    pub bit_mask: u32,
    pub chan_type: enum iio_chan_type,
    pub channel: u32,
    pub event_type: enum iio_event_type,
    pub event_dir: enum iio_event_direction,
}

    static const struct ade9000_irq1_event ade9000_irq1_events[] = {
    { ADE9000_ST1_ZXVA_BIT, IIO_VOLTAGE, ADE9000_PHASE_A_NR, IIO_EV_TYPE_THRESH, IIO_EV_DIR_EITHER },
    { ADE9000_ST1_ZXIA_BIT, IIO_CURRENT, ADE9000_PHASE_A_NR, IIO_EV_TYPE_THRESH, IIO_EV_DIR_EITHER },
    { ADE9000_ST1_ZXVB_BIT, IIO_VOLTAGE, ADE9000_PHASE_B_NR, IIO_EV_TYPE_THRESH, IIO_EV_DIR_EITHER },
    { ADE9000_ST1_ZXIB_BIT, IIO_CURRENT, ADE9000_PHASE_B_NR, IIO_EV_TYPE_THRESH, IIO_EV_DIR_EITHER },
    { ADE9000_ST1_ZXVC_BIT, IIO_VOLTAGE, ADE9000_PHASE_C_NR, IIO_EV_TYPE_THRESH, IIO_EV_DIR_EITHER },
    { ADE9000_ST1_ZXIC_BIT, IIO_CURRENT, ADE9000_PHASE_C_NR, IIO_EV_TYPE_THRESH, IIO_EV_DIR_EITHER },
    { ADE9000_ST1_SWELLA_BIT, IIO_ALTVOLTAGE, ADE9000_PHASE_A_NR, IIO_EV_TYPE_THRESH, IIO_EV_DIR_RISING },
    { ADE9000_ST1_SWELLB_BIT, IIO_ALTVOLTAGE, ADE9000_PHASE_B_NR, IIO_EV_TYPE_THRESH, IIO_EV_DIR_RISING },
    { ADE9000_ST1_SWELLC_BIT, IIO_ALTVOLTAGE, ADE9000_PHASE_C_NR, IIO_EV_TYPE_THRESH, IIO_EV_DIR_RISING },
    { ADE9000_ST1_DIPA_BIT, IIO_ALTVOLTAGE, ADE9000_PHASE_A_NR, IIO_EV_TYPE_THRESH, IIO_EV_DIR_FALLING },
    { ADE9000_ST1_DIPB_BIT, IIO_ALTVOLTAGE, ADE9000_PHASE_B_NR, IIO_EV_TYPE_THRESH, IIO_EV_DIR_FALLING },
    { ADE9000_ST1_DIPC_BIT, IIO_ALTVOLTAGE, ADE9000_PHASE_C_NR, IIO_EV_TYPE_THRESH, IIO_EV_DIR_FALLING },
    };
// Voltage events (zero crossing on instantaneous voltage)
    static const struct iio_event_spec ade9000_voltage_events[] = {
    {
// Zero crossing detection - datasheet: ZXV interrupts
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_EITHER,
    .mask_separate = BIT(IIO_EV_INFO_ENABLE),
    },
    };
// Current events (zero crossing on instantaneous current)
    static const struct iio_event_spec ade9000_current_events[] = {
    {
// Zero crossing detection - datasheet: ZXI interrupts
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_EITHER,
    .mask_separate = BIT(IIO_EV_INFO_ENABLE),
    },
    };
// RMS voltage events (swell/sag detection on RMS values)
    static const struct iio_event_spec ade9000_rms_voltage_events[] = {
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_RISING, /* RMS swell detection */
    .mask_separate = BIT(IIO_EV_INFO_ENABLE) | BIT(IIO_EV_INFO_VALUE),
    },
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_FALLING, /* RMS sag/dip detection */
    .mask_separate = BIT(IIO_EV_INFO_ENABLE) | BIT(IIO_EV_INFO_VALUE),
    },
    };
    static const char * const ade9000_filter_type_items[] = {
    "sinc4", "sinc4+lp",
    };
    static const int ade9000_filter_type_values[] = {
    0, 2,
    };
    static int ade9000_filter_type_get(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan)
    {
    struct ade9000_state *st = iio_priv(indio_dev);
    u32 val;
    int ret;
    unsigned int i;
    ret = regmap_read(st.regmap, ADE9000_REG_WFB_CFG, &val);
    if (ret)
    return ret;
    val = FIELD_GET(ADE9000_WF_SRC_MASK, val);
    for (i = 0; i < ARRAY_SIZE(ade9000_filter_type_values); i++) {
    if (ade9000_filter_type_values[i] == val)
    return i;
    }
    return -EINVAL;
    }
    static int ade9000_filter_type_set(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    unsigned int index)
    {
    struct ade9000_state *st = iio_priv(indio_dev);
    int ret, val;
    if (index >= ARRAY_SIZE(ade9000_filter_type_values))
    return -EINVAL;
    val = ade9000_filter_type_values[index];
// Update the WFB_CFG register with the new filter type
    ret = regmap_update_bits(st.regmap, ADE9000_REG_WFB_CFG,
    ADE9000_WF_SRC_MASK,
    FIELD_PREP(ADE9000_WF_SRC_MASK, val));
    if (ret)
    return ret;
// Update cached value
    st.wf_src = val;
    return 0;
    }
    static const struct iio_enum ade9000_filter_type_enum = {
    .items = ade9000_filter_type_items,
    .num_items = ARRAY_SIZE(ade9000_filter_type_items),
    .get = ade9000_filter_type_get,
    .set = ade9000_filter_type_set,
    };
    static const struct iio_chan_spec_ext_info ade9000_ext_info[] = {
    IIO_ENUM("filter_type", IIO_SHARED_BY_ALL, &ade9000_filter_type_enum),
    IIO_ENUM_AVAILABLE("filter_type", IIO_SHARED_BY_ALL, &ade9000_filter_type_enum),
    { }
    };

    .type = IIO_CURRENT,						\
    .channel = num,							\
    .address = ADE9000_ADDR_ADJUST(ADE9000_REG_AI_PCF, num),	\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |			\
    BIT(IIO_CHAN_INFO_SCALE) |		\
    BIT(IIO_CHAN_INFO_CALIBSCALE),		\
    .event_spec = ade9000_current_events,				\
    .num_event_specs = ARRAY_SIZE(ade9000_current_events),		\
    .scan_index = num,						\
    .indexed = 1,							\
    .scan_type = {							\
    .sign = 's',						\
    .realbits = 32,						\
    .storagebits = 32,					\
    .endianness = IIO_BE,					\
    },								\
    }

    .type = IIO_VOLTAGE,						\
    .channel = num,							\
    .address = ADE9000_ADDR_ADJUST(ADE9000_REG_AV_PCF, num),	\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |			\
    BIT(IIO_CHAN_INFO_SCALE) |		\
    BIT(IIO_CHAN_INFO_CALIBSCALE) |		\
    BIT(IIO_CHAN_INFO_FREQUENCY),		\
    .event_spec = ade9000_voltage_events,				\
    .num_event_specs = ARRAY_SIZE(ade9000_voltage_events),		\
    .scan_index = num + 1,	/* interleave with current channels */	\
    .indexed = 1,							\
    .scan_type = {							\
    .sign = 's',						\
    .realbits = 32,						\
    .storagebits = 32,					\
    .endianness = IIO_BE,					\
    },								\
    .ext_info = ade9000_ext_info,					\
    }

    .type = IIO_ALTCURRENT,						\
    .channel = num,							\
    .address = ADE9000_ADDR_ADJUST(ADE9000_REG_AIRMS, num),		\
    .channel2 = IIO_MOD_RMS,					\
    .modified = 1,							\
    .indexed = 1,							\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |			\
    BIT(IIO_CHAN_INFO_SCALE) |		\
    BIT(IIO_CHAN_INFO_CALIBBIAS),		\
    .scan_index = -1						\
    }

    .type = IIO_ALTVOLTAGE,						\
    .channel = num,							\
    .address = ADE9000_ADDR_ADJUST(ADE9000_REG_AVRMS, num),		\
    .channel2 = IIO_MOD_RMS,					\
    .modified = 1,							\
    .indexed = 1,							\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |			\
    BIT(IIO_CHAN_INFO_SCALE) |		\
    BIT(IIO_CHAN_INFO_CALIBBIAS),		\
    .event_spec = ade9000_rms_voltage_events,			\
    .num_event_specs = ARRAY_SIZE(ade9000_rms_voltage_events),	\
    .scan_index = -1						\
    }

    .type = IIO_POWER,						\
    .channel = num,							\
    .address = ADE9000_ADDR_ADJUST(ADE9000_REG_AWATT, num),		\
    .channel2 = IIO_MOD_ACTIVE,					\
    .modified = 1,							\
    .indexed = 1,							\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |			\
    BIT(IIO_CHAN_INFO_SCALE) |		\
    BIT(IIO_CHAN_INFO_CALIBBIAS) |		\
    BIT(IIO_CHAN_INFO_CALIBSCALE),		\
    .scan_index = -1						\
    }

    .type = IIO_POWER,						\
    .channel = num,							\
    .address = ADE9000_ADDR_ADJUST(ADE9000_REG_AVAR, num),		\
    .channel2 = IIO_MOD_REACTIVE,					\
    .modified = 1,							\
    .indexed = 1,							\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |			\
    BIT(IIO_CHAN_INFO_SCALE) |		\
    BIT(IIO_CHAN_INFO_CALIBBIAS),		\
    .scan_index = -1						\
    }

    .type = IIO_POWER,						\
    .channel = num,							\
    .address = ADE9000_ADDR_ADJUST(ADE9000_REG_AVA, num),		\
    .channel2 = IIO_MOD_APPARENT,					\
    .modified = 1,							\
    .indexed = 1,							\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |			\
    BIT(IIO_CHAN_INFO_SCALE),			\
    .scan_index = -1						\
    }

    .type = IIO_ENERGY,						\
    .channel = num,							\
    .address = addr,						\
    .channel2 = IIO_MOD_ACTIVE,					\
    .modified = 1,							\
    .indexed = 1,							\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),			\
    .scan_index = -1						\
    }

    .type = IIO_ENERGY,						\
    .channel = num,							\
    .address = addr,						\
    .channel2 = IIO_MOD_APPARENT,					\
    .modified = 1,							\
    .indexed = 1,							\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),			\
    .scan_index = -1						\
    }

    .type = IIO_ENERGY,						\
    .channel = num,							\
    .address = addr,						\
    .channel2 = IIO_MOD_REACTIVE,					\
    .modified = 1,							\
    .indexed = 1,							\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),			\
    .scan_index = -1						\
    }

    .type = IIO_POWER,						\
    .channel = num,							\
    .address = ADE9000_ADDR_ADJUST(ADE9000_REG_APF, num),		\
    .indexed = 1,							\
    .info_mask_separate = BIT(IIO_CHAN_INFO_POWERFACTOR),		\
    .scan_index = -1						\
    }
    static const struct iio_chan_spec ade9000_channels[] = {
// Phase A channels
    ADE9000_CURRENT_CHANNEL(ADE9000_PHASE_A_NR),
    ADE9000_VOLTAGE_CHANNEL(ADE9000_PHASE_A_NR),
    ADE9000_ALTCURRENT_RMS_CHANNEL(ADE9000_PHASE_A_NR),
    ADE9000_ALTVOLTAGE_RMS_CHANNEL(ADE9000_PHASE_A_NR),
    ADE9000_POWER_ACTIVE_CHANNEL(ADE9000_PHASE_A_NR),
    ADE9000_POWER_REACTIVE_CHANNEL(ADE9000_PHASE_A_NR),
    ADE9000_POWER_APPARENT_CHANNEL(ADE9000_PHASE_A_NR),
    ADE9000_ENERGY_ACTIVE_CHANNEL(ADE9000_PHASE_A_NR, ADE9000_REG_AWATTHR_LO),
    ADE9000_ENERGY_APPARENT_CHANNEL(ADE9000_PHASE_A_NR, ADE9000_REG_AVAHR_LO),
    ADE9000_ENERGY_REACTIVE_CHANNEL(ADE9000_PHASE_A_NR, ADE9000_REG_AFVARHR_LO),
    ADE9000_POWER_FACTOR_CHANNEL(ADE9000_PHASE_A_NR),
// Phase B channels
    ADE9000_CURRENT_CHANNEL(ADE9000_PHASE_B_NR),
    ADE9000_VOLTAGE_CHANNEL(ADE9000_PHASE_B_NR),
    ADE9000_ALTCURRENT_RMS_CHANNEL(ADE9000_PHASE_B_NR),
    ADE9000_ALTVOLTAGE_RMS_CHANNEL(ADE9000_PHASE_B_NR),
    ADE9000_POWER_ACTIVE_CHANNEL(ADE9000_PHASE_B_NR),
    ADE9000_POWER_REACTIVE_CHANNEL(ADE9000_PHASE_B_NR),
    ADE9000_POWER_APPARENT_CHANNEL(ADE9000_PHASE_B_NR),
    ADE9000_ENERGY_ACTIVE_CHANNEL(ADE9000_PHASE_B_NR, ADE9000_REG_BWATTHR_LO),
    ADE9000_ENERGY_APPARENT_CHANNEL(ADE9000_PHASE_B_NR, ADE9000_REG_BVAHR_LO),
    ADE9000_ENERGY_REACTIVE_CHANNEL(ADE9000_PHASE_B_NR, ADE9000_REG_BFVARHR_LO),
    ADE9000_POWER_FACTOR_CHANNEL(ADE9000_PHASE_B_NR),
// Phase C channels
    ADE9000_CURRENT_CHANNEL(ADE9000_PHASE_C_NR),
    ADE9000_VOLTAGE_CHANNEL(ADE9000_PHASE_C_NR),
    ADE9000_ALTCURRENT_RMS_CHANNEL(ADE9000_PHASE_C_NR),
    ADE9000_ALTVOLTAGE_RMS_CHANNEL(ADE9000_PHASE_C_NR),
    ADE9000_POWER_ACTIVE_CHANNEL(ADE9000_PHASE_C_NR),
    ADE9000_POWER_REACTIVE_CHANNEL(ADE9000_PHASE_C_NR),
    ADE9000_POWER_APPARENT_CHANNEL(ADE9000_PHASE_C_NR),
    ADE9000_ENERGY_ACTIVE_CHANNEL(ADE9000_PHASE_C_NR, ADE9000_REG_CWATTHR_LO),
    ADE9000_ENERGY_APPARENT_CHANNEL(ADE9000_PHASE_C_NR, ADE9000_REG_CVAHR_LO),
    ADE9000_ENERGY_REACTIVE_CHANNEL(ADE9000_PHASE_C_NR, ADE9000_REG_CFVARHR_LO),
    ADE9000_POWER_FACTOR_CHANNEL(ADE9000_PHASE_C_NR),
    };
    static const struct reg_sequence ade9000_initialization_sequence[] = {
    { ADE9000_REG_PGA_GAIN, ADE9000_PGA_GAIN },
    { ADE9000_REG_CONFIG0, ADE9000_CONFIG0 },
    { ADE9000_REG_CONFIG1, ADE9000_CONFIG1 },
    { ADE9000_REG_CONFIG2, ADE9000_CONFIG2 },
    { ADE9000_REG_CONFIG3, ADE9000_CONFIG3 },
    { ADE9000_REG_ACCMODE, ADE9000_ACCMODE },
    { ADE9000_REG_ZX_LP_SEL, ADE9000_ZX_LP_SEL },
    { ADE9000_REG_MASK0, ADE9000_MASK0_ALL_INT_DIS },
    { ADE9000_REG_MASK1, ADE9000_MASK1_ALL_INT_DIS },
    { ADE9000_REG_EVENT_MASK, ADE9000_EVENT_DISABLE },
    { ADE9000_REG_WFB_CFG, ADE9000_WFB_CFG },
    { ADE9000_REG_VLEVEL, ADE9000_VLEVEL },
    { ADE9000_REG_DICOEFF, ADE9000_DICOEFF },
    { ADE9000_REG_EGY_TIME, ADE9000_EGY_TIME },
    { ADE9000_REG_EP_CFG, ADE9000_EP_CFG },
// Clear all pending status bits by writing 1s
    { ADE9000_REG_STATUS0, GENMASK(31, 0) },
    { ADE9000_REG_STATUS1, GENMASK(31, 0) },
    { ADE9000_REG_RUN, ADE9000_RUN_ON }
    };
    static int ade9000_spi_write_reg(void *context, unsigned int reg,
    unsigned int val)
    {
    struct ade9000_state *st = context;
    u8 tx_buf[6];
    u16 addr;
    int ret, len;
    guard(mutex)(&st.lock);
    addr = FIELD_PREP(ADE9000_REG_ADDR_MASK, reg);
    put_unaligned_be16(addr, tx_buf);
    if (reg > ADE9000_REG_RUN && reg < ADE9000_REG_VERSION) {
    put_unaligned_be16(val, &tx_buf[2]);
    len = 4;
    } else {
    put_unaligned_be32(val, &tx_buf[2]);
    len = 6;
    }
    ret = spi_write_then_read(st.spi, tx_buf, len, core::ptr::null_mut(), 0);
    if (ret)
    dev_err(&st.spi.dev, "problem when writing register 0x%x\n", reg);
    return ret;
    }
    static int ade9000_spi_read_reg(void *context, unsigned int reg,
    unsigned int *val)
    {
    struct ade9000_state *st = context;
    u8 tx_buf[2];
    u8 rx_buf[4];
    u16 addr;
    int ret, rx_len;
    guard(mutex)(&st.lock);
    addr = FIELD_PREP(ADE9000_REG_ADDR_MASK, reg) |
    ADE9000_REG_READ_BIT_MASK;
    put_unaligned_be16(addr, tx_buf);
// Skip CRC bytes - only read actual data
    if (reg > ADE9000_REG_RUN && reg < ADE9000_REG_VERSION)
    rx_len = 2;
    else
    rx_len = 4;
    ret = spi_write_then_read(st.spi, tx_buf, 2, rx_buf, rx_len);
    if (ret) {
    dev_err(&st.spi.dev, "error reading register 0x%x\n", reg);
    return ret;
    }
    if (reg > ADE9000_REG_RUN && reg < ADE9000_REG_VERSION)
// val = get_unaligned_be16(rx_buf);
    else
// val = get_unaligned_be32(rx_buf);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ade9000_is_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool ade9000_is_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
// Interrupt/error status registers - volatile
    case ADE9000_REG_STATUS0:
    case ADE9000_REG_STATUS1:
    return true;
    default:
// All other registers are non-volatile
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn ade9000_configure_scan(indio_dev: *mut iio_dev, wfb_addr: u32) {
    static void ade9000_configure_scan(struct iio_dev *indio_dev, u32 wfb_addr)
    {
    struct ade9000_state *st = iio_priv(indio_dev);
    u16 addr;
    addr = FIELD_PREP(ADE9000_REG_ADDR_MASK, wfb_addr) |
    ADE9000_REG_READ_BIT_MASK;
    put_unaligned_be16(addr, st.tx_buff);
    st.xfer[0].tx_buf = &st.tx_buff[0];
    st.xfer[0].len = 2;
    st.xfer[1].rx_buf = st.rx_buff.byte;
// Always use streaming mode
    st.xfer[1].len = (st.wfb_nr_samples / 2) * 4;
    spi_message_init_with_transfers(&st.spi_msg, st.xfer, ARRAY_SIZE(st.xfer));
    }
#[no_mangle]
unsafe extern "C" fn ade9000_iio_push_streaming(indio_dev: *mut iio_dev) -> c_int {
    static int ade9000_iio_push_streaming(struct iio_dev *indio_dev)
    {
    struct ade9000_state *st = iio_priv(indio_dev);
    struct device *dev = &st.spi.dev;
    u32 current_page, i;
    int ret;
    guard(mutex)(&st.lock);
    ret = spi_sync(st.spi, &st.spi_msg);
    if (ret) {
    dev_err_ratelimited(dev, "SPI fail in trigger handler\n");
    return ret;
    }
// In streaming mode, only half the buffer is filled per interrupt
    for (i = 0; i < st.wfb_nr_samples / 2; i += st.wfb_nr_activ_chan)
    iio_push_to_buffers(indio_dev, &st.rx_buff.word[i]);
    ret = regmap_read(st.regmap, ADE9000_REG_WFB_PG_IRQEN, &current_page);
    if (ret) {
    dev_err_ratelimited(dev, "IRQ0 WFB read fail\n");
    return ret;
    }
    if (current_page & ADE9000_MIDDLE_PAGE_BIT) {
    ret = regmap_write(st.regmap, ADE9000_REG_WFB_PG_IRQEN,
    ADE9000_LAST_PAGE_BIT);
    if (ret) {
    dev_err_ratelimited(dev, "IRQ0 WFB write fail\n");
    return ret;
    }
    ade9000_configure_scan(indio_dev,
    ADE9000_REG_WF_HALF_BUFF);
    } else {
    ret = regmap_write(st.regmap, ADE9000_REG_WFB_PG_IRQEN,
    ADE9000_MIDDLE_PAGE_BIT);
    if (ret) {
    dev_err_ratelimited(dev, "IRQ0 WFB write fail");
    return ret;
    }
    ade9000_configure_scan(indio_dev, ADE9000_REG_WF_BUFF);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ade9000_iio_push_buffer(indio_dev: *mut iio_dev) -> c_int {
    static int ade9000_iio_push_buffer(struct iio_dev *indio_dev)
    {
    struct ade9000_state *st = iio_priv(indio_dev);
    int ret;
    u32 i;
    guard(mutex)(&st.lock);
    ret = spi_sync(st.spi, &st.spi_msg);
    if (ret) {
    dev_err_ratelimited(&st.spi.dev,
    "SPI fail in trigger handler\n");
    return ret;
    }
    for (i = 0; i < st.wfb_nr_samples; i += st.wfb_nr_activ_chan)
    iio_push_to_buffers(indio_dev, &st.rx_buff.word[i]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ade9000_irq0_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ade9000_irq0_thread(int irq, void *data)
    {
    struct iio_dev *indio_dev = data;
    struct ade9000_state *st = iio_priv(indio_dev);
    struct device *dev = &st.spi.dev;
    let mut handled_irq: u32 = 0;
    u32 interrupts, status;
    int ret;
    ret = regmap_read(st.regmap, ADE9000_REG_STATUS0, &status);
    if (ret) {
    dev_err_ratelimited(dev, "IRQ0 read status fail\n");
    return IRQ_HANDLED;
    }
    ret = regmap_read(st.regmap, ADE9000_REG_MASK0, &interrupts);
    if (ret) {
    dev_err_ratelimited(dev, "IRQ0 read mask fail\n");
    return IRQ_HANDLED;
    }
    if ((status & ADE9000_ST0_PAGE_FULL_BIT) &&
    (interrupts & ADE9000_ST0_PAGE_FULL_BIT)) {
// Always use streaming mode
    ret = ade9000_iio_push_streaming(indio_dev);
    if (ret) {
    dev_err_ratelimited(dev, "IRQ0 IIO push fail\n");
    return IRQ_HANDLED;
    }
    handled_irq |= ADE9000_ST0_PAGE_FULL_BIT;
    }
    if ((status & ADE9000_ST0_WFB_TRIG_BIT) &&
    (interrupts & ADE9000_ST0_WFB_TRIG_BIT)) {
    ret = regmap_update_bits(st.regmap, ADE9000_REG_WFB_CFG,
    ADE9000_WF_CAP_EN_MASK, 0);
    if (ret) {
    dev_err_ratelimited(dev, "IRQ0 WFB fail\n");
    return IRQ_HANDLED;
    }
    if (iio_buffer_enabled(indio_dev)) {
    ret = ade9000_iio_push_buffer(indio_dev);
    if (ret) {
    dev_err_ratelimited(dev,
    "IRQ0 IIO push fail @ WFB TRIG\n");
    return IRQ_HANDLED;
    }
    }
    handled_irq |= ADE9000_ST0_WFB_TRIG_BIT;
    }
    ret = regmap_write(st.regmap, ADE9000_REG_STATUS0, handled_irq);
    if (ret)
    dev_err_ratelimited(dev, "IRQ0 write status fail\n");
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ade9000_irq1_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ade9000_irq1_thread(int irq, void *data)
    {
    struct iio_dev *indio_dev = data;
    struct ade9000_state *st = iio_priv(indio_dev);
    let mut bit: c_uint = ADE9000_ST1_CROSSING_FIRST;
    let mut timestamp: i64 = iio_get_time_ns(indio_dev);
    let mut handled_irq: u32 = 0;
    u32 interrupts, result, status, tmp;
    DECLARE_BITMAP(interrupt_bits, ADE9000_ST1_CROSSING_DEPTH);
    const struct ade9000_irq1_event *event;
    int ret, i;
    if (!completion_done(&st.reset_completion)) {
    ret = regmap_read(st.regmap, ADE9000_REG_STATUS1, &result);
    if (ret) {
    dev_err_ratelimited(&st.spi.dev, "IRQ1 read status fail\n");
    return IRQ_HANDLED;
    }
    if (result & ADE9000_ST1_RSTDONE_BIT) {
    complete(&st.reset_completion);
// Clear the reset done status bit
    ret = regmap_write(st.regmap, ADE9000_REG_STATUS1, ADE9000_ST1_RSTDONE_BIT);
    if (ret)
    dev_err_ratelimited(&st.spi.dev,
    "IRQ1 clear reset status fail\n");
    } else {
    dev_err_ratelimited(&st.spi.dev,
    "Error testing reset done\n");
    }
    return IRQ_HANDLED;
    }
    ret = regmap_read(st.regmap, ADE9000_REG_STATUS1, &status);
    if (ret) {
    dev_err_ratelimited(&st.spi.dev, "IRQ1 read status fail\n");
    return IRQ_HANDLED;
    }
    ret = regmap_read(st.regmap, ADE9000_REG_MASK1, &interrupts);
    if (ret) {
    dev_err_ratelimited(&st.spi.dev, "IRQ1 read mask fail\n");
    return IRQ_HANDLED;
    }
    bitmap_from_arr32(interrupt_bits, &interrupts, ADE9000_ST1_CROSSING_DEPTH);
    for_each_set_bit_from(bit, interrupt_bits,
    ADE9000_ST1_CROSSING_DEPTH) {
    tmp = status & BIT(bit);
    if (!tmp)
    continue;
    event = core::ptr::null_mut();
// Find corresponding event in lookup table
    for (i = 0; i < ARRAY_SIZE(ade9000_irq1_events); i++) {
    if (ade9000_irq1_events[i].bit_mask == tmp) {
    event = &ade9000_irq1_events[i];
    break;
    }
    }
    if (event) {
    iio_push_event(indio_dev,
    IIO_UNMOD_EVENT_CODE(event.chan_type,
    event.channel,
    event.event_type,
    event.event_dir),
    timestamp);
    }
    handled_irq |= tmp;
    }
    ret = regmap_write(st.regmap, ADE9000_REG_STATUS1, handled_irq);
    if (ret)
    dev_err_ratelimited(&st.spi.dev, "IRQ1 write status fail\n");
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ade9000_dready_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ade9000_dready_thread(int irq, void *data)
    {
    struct iio_dev *indio_dev = data;
// Handle data ready interrupt from C4/EVENT/DREADY pin
    if (iio_device_try_claim_buffer_mode(indio_dev)) {
    ade9000_iio_push_buffer(indio_dev);
    iio_device_release_buffer_mode(indio_dev);
    }
    return IRQ_HANDLED;
    }
    static int ade9000_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val,
    int *val2,
    long mask)
    {
    struct ade9000_state *st = iio_priv(indio_dev);
    unsigned int measured;
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_FREQUENCY:
    if (chan.type == IIO_VOLTAGE) {
    int period_reg;
    int period;
    switch (chan.channel) {
    case ADE9000_PHASE_A_NR:
    period_reg = ADE9000_REG_APERIOD;
    break;
    case ADE9000_PHASE_B_NR:
    period_reg = ADE9000_REG_BPERIOD;
    break;
    case ADE9000_PHASE_C_NR:
    period_reg = ADE9000_REG_CPERIOD;
    break;
    default:
    return -EINVAL;
    }
    ret = regmap_read(st.regmap, period_reg, &period);
    if (ret)
    return ret;
//
// Frequency = (4MHz * 65536) / (PERIOD + 1)
// 4MHz = ADC sample rate, 65536 = 2^16 period register scaling
// See ADE9000 datasheet section on period measurement
//
// val = 4000 * 65536;
// val2 = period + 1;
    return IIO_VAL_FRACTIONAL;
    }
    return -EINVAL;
    case IIO_CHAN_INFO_RAW:
    if (chan.type == IIO_ENERGY) {
    let mut lo_reg: u16 = chan.address;
    ret = regmap_bulk_read(st.regmap, lo_reg,
    st.bulk_read_buf, 2);
    if (ret)
    return ret;
// val = st->bulk_read_buf[0];  /* Lower 32 bits
// val2 = st->bulk_read_buf[1]; /* Upper 32 bits
    return IIO_VAL_INT_64;
    }
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = regmap_read(st.regmap, chan.address, &measured);
    iio_device_release_direct(indio_dev);
    if (ret)
    return ret;
// val = measured;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_POWERFACTOR:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = regmap_read(st.regmap, chan.address, &measured);
    iio_device_release_direct(indio_dev);
    if (ret)
    return ret;
// val = measured;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    switch (chan.type) {
    case IIO_CURRENT:
    case IIO_VOLTAGE:
    case IIO_ALTVOLTAGE:
    case IIO_ALTCURRENT:
    switch (chan.address) {
    case ADE9000_REG_AI_PCF:
    case ADE9000_REG_AV_PCF:
    case ADE9000_REG_BI_PCF:
    case ADE9000_REG_BV_PCF:
    case ADE9000_REG_CI_PCF:
    case ADE9000_REG_CV_PCF:
// val = 1;
// val2 = ADE9000_PCF_FULL_SCALE_CODES;
    return IIO_VAL_FRACTIONAL;
    case ADE9000_REG_AIRMS:
    case ADE9000_REG_AVRMS:
    case ADE9000_REG_BIRMS:
    case ADE9000_REG_BVRMS:
    case ADE9000_REG_CIRMS:
    case ADE9000_REG_CVRMS:
// val = 1;
// val2 = ADE9000_RMS_FULL_SCALE_CODES;
    return IIO_VAL_FRACTIONAL;
    default:
    return -EINVAL;
    }
    case IIO_POWER:
// val = 1;
// val2 = ADE9000_WATT_FULL_SCALE_CODES;
    return IIO_VAL_FRACTIONAL;
    default:
    break;
    }
    return -EINVAL;
    default:
    return -EINVAL;
    }
    }
    static int ade9000_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val,
    int val2,
    long mask)
    {
    struct ade9000_state *st = iio_priv(indio_dev);
    u32 tmp;
    switch (mask) {
    case IIO_CHAN_INFO_CALIBBIAS:
    switch (chan.type) {
    case IIO_CURRENT:
    return regmap_write(st.regmap,
    ADE9000_ADDR_ADJUST(ADE9000_REG_AIRMSOS,
    chan.channel), val);
    case IIO_VOLTAGE:
    case IIO_ALTVOLTAGE:
    return regmap_write(st.regmap,
    ADE9000_ADDR_ADJUST(ADE9000_REG_AVRMSOS,
    chan.channel), val);
    case IIO_POWER:
    tmp = chan.address;
    tmp &= ~ADE9000_PHASE_B_POS_BIT;
    tmp &= ~ADE9000_PHASE_C_POS_BIT;
    switch (tmp) {
    case ADE9000_REG_AWATT:
    return regmap_write(st.regmap,
    ADE9000_ADDR_ADJUST(ADE9000_REG_AWATTOS,
    chan.channel), val);
    case ADE9000_REG_AVAR:
    return regmap_write(st.regmap,
    ADE9000_ADDR_ADJUST(ADE9000_REG_AVAROS,
    chan.channel), val);
    case ADE9000_REG_AFVAR:
    return regmap_write(st.regmap,
    ADE9000_ADDR_ADJUST(ADE9000_REG_AFVAROS,
    chan.channel), val);
    default:
    return -EINVAL;
    }
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_CALIBSCALE:
//
// Calibration gain registers for fine-tuning measurements.
// These are separate from PGA gain and applied in the digital domain.
//
    switch (chan.type) {
    case IIO_CURRENT:
    return regmap_write(st.regmap,
    ADE9000_ADDR_ADJUST(ADE9000_REG_AIGAIN,
    chan.channel), val);
    case IIO_VOLTAGE:
    return regmap_write(st.regmap,
    ADE9000_ADDR_ADJUST(ADE9000_REG_AVGAIN,
    chan.channel), val);
    case IIO_POWER:
    return regmap_write(st.regmap,
    ADE9000_ADDR_ADJUST(ADE9000_REG_APGAIN,
    chan.channel), val);
    default:
    return -EINVAL;
    }
    case IIO_CHAN_INFO_SCALE:
// Per-channel scales are read-only
    return -EINVAL;
    default:
    return -EINVAL;
    }
    }
    static int ade9000_reg_access(struct iio_dev *indio_dev,
    unsigned int reg,
    unsigned int tx_val,
    unsigned int *rx_val)
    {
    struct ade9000_state *st = iio_priv(indio_dev);
    if (rx_val)
    return regmap_read(st.regmap, reg, rx_val);
    return regmap_write(st.regmap, reg, tx_val);
    }
    static int ade9000_read_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir)
    {
    struct ade9000_state *st = iio_priv(indio_dev);
    u32 interrupts1;
    int ret;
// All events use MASK1 register
    ret = regmap_read(st.regmap, ADE9000_REG_MASK1, &interrupts1);
    if (ret)
    return ret;
    switch (chan.channel) {
    case ADE9000_PHASE_A_NR:
    if (chan.type == IIO_VOLTAGE && dir == IIO_EV_DIR_EITHER)
    return !!(interrupts1 & ADE9000_ST1_ZXVA_BIT);
#[no_mangle]
pub unsafe extern "C" fn if(IIO_EV_DIR_EITHER: chan->type == IIO_CURRENT && dir ==) -> else {
    else if (chan.type == IIO_CURRENT && dir == IIO_EV_DIR_EITHER)
    return !!(interrupts1 & ADE9000_ST1_ZXIA_BIT);
#[no_mangle]
pub unsafe extern "C" fn if(IIO_EV_DIR_RISING: chan->type == IIO_ALTVOLTAGE && dir ==) -> else {
    else if (chan.type == IIO_ALTVOLTAGE && dir == IIO_EV_DIR_RISING)
    return !!(interrupts1 & ADE9000_ST1_SWELLA_BIT);
#[no_mangle]
pub unsafe extern "C" fn if(IIO_EV_DIR_FALLING: chan->type == IIO_ALTVOLTAGE && dir ==) -> else {
    else if (chan.type == IIO_ALTVOLTAGE && dir == IIO_EV_DIR_FALLING)
    return !!(interrupts1 & ADE9000_ST1_DIPA_BIT);
    dev_err_ratelimited(&indio_dev.dev,
    "Invalid channel type %d or direction %d for phase A\n", chan.type, dir);
    return -EINVAL;
    case ADE9000_PHASE_B_NR:
    if (chan.type == IIO_VOLTAGE && dir == IIO_EV_DIR_EITHER)
    return !!(interrupts1 & ADE9000_ST1_ZXVB_BIT);
#[no_mangle]
pub unsafe extern "C" fn if(IIO_EV_DIR_EITHER: chan->type == IIO_CURRENT && dir ==) -> else {
    else if (chan.type == IIO_CURRENT && dir == IIO_EV_DIR_EITHER)
    return !!(interrupts1 & ADE9000_ST1_ZXIB_BIT);
#[no_mangle]
pub unsafe extern "C" fn if(IIO_EV_DIR_RISING: chan->type == IIO_ALTVOLTAGE && dir ==) -> else {
    else if (chan.type == IIO_ALTVOLTAGE && dir == IIO_EV_DIR_RISING)
    return !!(interrupts1 & ADE9000_ST1_SWELLB_BIT);
#[no_mangle]
pub unsafe extern "C" fn if(IIO_EV_DIR_FALLING: chan->type == IIO_ALTVOLTAGE && dir ==) -> else {
    else if (chan.type == IIO_ALTVOLTAGE && dir == IIO_EV_DIR_FALLING)
    return !!(interrupts1 & ADE9000_ST1_DIPB_BIT);
    dev_err_ratelimited(&indio_dev.dev,
    "Invalid channel type %d or direction %d for phase B\n", chan.type, dir);
    return -EINVAL;
    case ADE9000_PHASE_C_NR:
    if (chan.type == IIO_VOLTAGE && dir == IIO_EV_DIR_EITHER)
    return !!(interrupts1 & ADE9000_ST1_ZXVC_BIT);
#[no_mangle]
pub unsafe extern "C" fn if(IIO_EV_DIR_EITHER: chan->type == IIO_CURRENT && dir ==) -> else {
    else if (chan.type == IIO_CURRENT && dir == IIO_EV_DIR_EITHER)
    return !!(interrupts1 & ADE9000_ST1_ZXIC_BIT);
#[no_mangle]
pub unsafe extern "C" fn if(IIO_EV_DIR_RISING: chan->type == IIO_ALTVOLTAGE && dir ==) -> else {
    else if (chan.type == IIO_ALTVOLTAGE && dir == IIO_EV_DIR_RISING)
    return !!(interrupts1 & ADE9000_ST1_SWELLC_BIT);
#[no_mangle]
pub unsafe extern "C" fn if(IIO_EV_DIR_FALLING: chan->type == IIO_ALTVOLTAGE && dir ==) -> else {
    else if (chan.type == IIO_ALTVOLTAGE && dir == IIO_EV_DIR_FALLING)
    return !!(interrupts1 & ADE9000_ST1_DIPC_BIT);
    dev_err_ratelimited(&indio_dev.dev,
    "Invalid channel type %d or direction %d for phase C\n", chan.type, dir);
    return -EINVAL;
    default:
    return -EINVAL;
    }
    }
    static int ade9000_write_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    bool state)
    {
    struct ade9000_state *st = iio_priv(indio_dev);
    u32 bit_mask;
    int ret;
// Clear all pending events in STATUS1 register (write 1 to clear)
    ret = regmap_write(st.regmap, ADE9000_REG_STATUS1, GENMASK(31, 0));
    if (ret)
    return ret;
// Determine which interrupt bit to enable/disable
    switch (chan.channel) {
    case ADE9000_PHASE_A_NR:
    if (chan.type == IIO_VOLTAGE && dir == IIO_EV_DIR_EITHER) {
    bit_mask = ADE9000_ST1_ZXVA_BIT;
    if (state)
    st.wfb_trg |= ADE9000_WFB_TRG_ZXVA_BIT;
    else
    st.wfb_trg &= ~ADE9000_WFB_TRG_ZXVA_BIT;
    } else if (chan.type == IIO_CURRENT && dir == IIO_EV_DIR_EITHER) {
    bit_mask = ADE9000_ST1_ZXIA_BIT;
    if (state)
    st.wfb_trg |= ADE9000_WFB_TRG_ZXIA_BIT;
    else
    st.wfb_trg &= ~ADE9000_WFB_TRG_ZXIA_BIT;
    } else if (chan.type == IIO_ALTVOLTAGE && dir == IIO_EV_DIR_RISING) {
    bit_mask = ADE9000_ST1_SWELLA_BIT;
    if (state)
    st.wfb_trg |= ADE9000_WFB_TRG_SWELL_BIT;
    else
    st.wfb_trg &= ~ADE9000_WFB_TRG_SWELL_BIT;
    } else if (chan.type == IIO_ALTVOLTAGE && dir == IIO_EV_DIR_FALLING) {
    bit_mask = ADE9000_ST1_DIPA_BIT;
    if (state)
    st.wfb_trg |= ADE9000_WFB_TRG_DIP_BIT;
    else
    st.wfb_trg &= ~ADE9000_WFB_TRG_DIP_BIT;
    } else {
    dev_err_ratelimited(&indio_dev.dev, "Invalid channel type %d or direction %d for phase A\n",
    chan.type, dir);
    return -EINVAL;
    }
    break;
    case ADE9000_PHASE_B_NR:
    if (chan.type == IIO_VOLTAGE && dir == IIO_EV_DIR_EITHER) {
    bit_mask = ADE9000_ST1_ZXVB_BIT;
    if (state)
    st.wfb_trg |= ADE9000_WFB_TRG_ZXVB_BIT;
    else
    st.wfb_trg &= ~ADE9000_WFB_TRG_ZXVB_BIT;
    } else if (chan.type == IIO_CURRENT && dir == IIO_EV_DIR_EITHER) {
    bit_mask = ADE9000_ST1_ZXIB_BIT;
    if (state)
    st.wfb_trg |= ADE9000_WFB_TRG_ZXIB_BIT;
    else
    st.wfb_trg &= ~ADE9000_WFB_TRG_ZXIB_BIT;
    } else if (chan.type == IIO_ALTVOLTAGE && dir == IIO_EV_DIR_RISING) {
    bit_mask = ADE9000_ST1_SWELLB_BIT;
    if (state)
    st.wfb_trg |= ADE9000_WFB_TRG_SWELL_BIT;
    else
    st.wfb_trg &= ~ADE9000_WFB_TRG_SWELL_BIT;
    } else if (chan.type == IIO_ALTVOLTAGE && dir == IIO_EV_DIR_FALLING) {
    bit_mask = ADE9000_ST1_DIPB_BIT;
    if (state)
    st.wfb_trg |= ADE9000_WFB_TRG_DIP_BIT;
    else
    st.wfb_trg &= ~ADE9000_WFB_TRG_DIP_BIT;
    } else {
    dev_err_ratelimited(&indio_dev.dev,
    "Invalid channel type %d or direction %d for phase B\n",
    chan.type, dir);
    return -EINVAL;
    }
    break;
    case ADE9000_PHASE_C_NR:
    if (chan.type == IIO_VOLTAGE && dir == IIO_EV_DIR_EITHER) {
    bit_mask = ADE9000_ST1_ZXVC_BIT;
    if (state)
    st.wfb_trg |= ADE9000_WFB_TRG_ZXVC_BIT;
    else
    st.wfb_trg &= ~ADE9000_WFB_TRG_ZXVC_BIT;
    } else if (chan.type == IIO_CURRENT && dir == IIO_EV_DIR_EITHER) {
    bit_mask = ADE9000_ST1_ZXIC_BIT;
    if (state)
    st.wfb_trg |= ADE9000_WFB_TRG_ZXIC_BIT;
    else
    st.wfb_trg &= ~ADE9000_WFB_TRG_ZXIC_BIT;
    } else if (chan.type == IIO_ALTVOLTAGE && dir == IIO_EV_DIR_RISING) {
    bit_mask = ADE9000_ST1_SWELLC_BIT;
    if (state)
    st.wfb_trg |= ADE9000_WFB_TRG_SWELL_BIT;
    else
    st.wfb_trg &= ~ADE9000_WFB_TRG_SWELL_BIT;
    } else if (chan.type == IIO_ALTVOLTAGE && dir == IIO_EV_DIR_FALLING) {
    bit_mask = ADE9000_ST1_DIPC_BIT;
    if (state)
    st.wfb_trg |= ADE9000_WFB_TRG_DIP_BIT;
    else
    st.wfb_trg &= ~ADE9000_WFB_TRG_DIP_BIT;
    } else {
    dev_err_ratelimited(&indio_dev.dev,
    "Invalid channel type %d or direction %d for phase C\n",
    chan.type, dir);
    return -EINVAL;
    }
    break;
    default:
    return -EINVAL;
    }
// Set bits if enabling event, clear bits if disabling
    return regmap_assign_bits(st.regmap, ADE9000_REG_MASK1, bit_mask, state ? bit_mask : 0);
    }
    static int ade9000_write_event_value(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info,
    int val, int val2)
    {
    struct ade9000_state *st = iio_priv(indio_dev);
    switch (info) {
    case IIO_EV_INFO_VALUE:
    switch (dir) {
    case IIO_EV_DIR_FALLING:
    return regmap_write(st.regmap, ADE9000_REG_DIP_LVL, val);
    case IIO_EV_DIR_RISING:
    return regmap_write(st.regmap, ADE9000_REG_SWELL_LVL, val);
    default:
    return -EINVAL;
    }
    default:
    return -EINVAL;
    }
    }
    static int ade9000_read_event_value(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info,
    int *val, int *val2)
    {
    struct ade9000_state *st = iio_priv(indio_dev);
    unsigned int data;
    int ret;
    switch (info) {
    case IIO_EV_INFO_VALUE:
    switch (dir) {
    case IIO_EV_DIR_FALLING:
    ret = regmap_read(st.regmap, ADE9000_REG_DIP_LVL, &data);
    if (ret)
    return ret;
// val = data;
    return IIO_VAL_INT;
    case IIO_EV_DIR_RISING:
    ret = regmap_read(st.regmap, ADE9000_REG_SWELL_LVL, &data);
    if (ret)
    return ret;
// val = data;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn ade9000_waveform_buffer_config(indio_dev: *mut iio_dev) -> c_int {
    static int ade9000_waveform_buffer_config(struct iio_dev *indio_dev)
    {
    struct ade9000_state *st = iio_priv(indio_dev);
    u32 wfb_cfg_val;
    u32 active_scans;
    bitmap_to_arr32(&active_scans, indio_dev.active_scan_mask,
    iio_get_masklength(indio_dev));
    switch (active_scans) {
    case ADE9000_SCAN_POS_IA | ADE9000_SCAN_POS_VA:
    wfb_cfg_val = ADE9000_WFB_CFG_IA_VA;
    st.wfb_nr_activ_chan = 2;
    break;
    case ADE9000_SCAN_POS_IB | ADE9000_SCAN_POS_VB:
    wfb_cfg_val = ADE9000_WFB_CFG_IB_VB;
    st.wfb_nr_activ_chan = 2;
    break;
    case ADE9000_SCAN_POS_IC | ADE9000_SCAN_POS_VC:
    wfb_cfg_val = ADE9000_WFB_CFG_IC_VC;
    st.wfb_nr_activ_chan = 2;
    break;
    case ADE9000_SCAN_POS_IA:
    wfb_cfg_val = ADE9000_WFB_CFG_IA;
    st.wfb_nr_activ_chan = 1;
    break;
    case ADE9000_SCAN_POS_VA:
    wfb_cfg_val = ADE9000_WFB_CFG_VA;
    st.wfb_nr_activ_chan = 1;
    break;
    case ADE9000_SCAN_POS_IB:
    wfb_cfg_val = ADE9000_WFB_CFG_IB;
    st.wfb_nr_activ_chan = 1;
    break;
    case ADE9000_SCAN_POS_VB:
    wfb_cfg_val = ADE9000_WFB_CFG_VB;
    st.wfb_nr_activ_chan = 1;
    break;
    case ADE9000_SCAN_POS_IC:
    wfb_cfg_val = ADE9000_WFB_CFG_IC;
    st.wfb_nr_activ_chan = 1;
    break;
    case ADE9000_SCAN_POS_VC:
    wfb_cfg_val = ADE9000_WFB_CFG_VC;
    st.wfb_nr_activ_chan = 1;
    break;
    case (ADE9000_SCAN_POS_IA | ADE9000_SCAN_POS_VA | ADE9000_SCAN_POS_IB |
    ADE9000_SCAN_POS_VB | ADE9000_SCAN_POS_IC | ADE9000_SCAN_POS_VC):
    wfb_cfg_val = ADE9000_WFB_CFG_ALL_CHAN;
    st.wfb_nr_activ_chan = 6;
    break;
    default:
    dev_err(&st.spi.dev, "Unsupported combination of scans\n");
    return -EINVAL;
    }
    wfb_cfg_val |= FIELD_PREP(ADE9000_WF_SRC_MASK, st.wf_src);
    return regmap_write(st.regmap, ADE9000_REG_WFB_CFG, wfb_cfg_val);
    }
#[no_mangle]
unsafe extern "C" fn ade9000_waveform_buffer_interrupt_setup(st: *mut ade9000_state) -> c_int {
    static int ade9000_waveform_buffer_interrupt_setup(struct ade9000_state *st)
    {
    int ret;
    ret = regmap_write(st.regmap, ADE9000_REG_WFB_TRG_CFG, 0x0);
    if (ret)
    return ret;
// Always use streaming mode setup
    ret = regmap_write(st.regmap, ADE9000_REG_WFB_PG_IRQEN,
    ADE9000_MIDDLE_PAGE_BIT);
    if (ret)
    return ret;
    ret = regmap_write(st.regmap, ADE9000_REG_STATUS0, GENMASK(31, 0));
    if (ret)
    return ret;
    return regmap_set_bits(st.regmap, ADE9000_REG_MASK0,
    ADE9000_ST0_PAGE_FULL_BIT);
    }
#[no_mangle]
unsafe extern "C" fn ade9000_buffer_preenable(indio_dev: *mut iio_dev) -> c_int {
    static int ade9000_buffer_preenable(struct iio_dev *indio_dev)
    {
    struct ade9000_state *st = iio_priv(indio_dev);
    int ret;
    ret = ade9000_waveform_buffer_config(indio_dev);
    if (ret)
    return ret;
    st.wfb_nr_samples = ADE9000_WFB_MAX_SAMPLES_CHAN * st.wfb_nr_activ_chan;
    ade9000_configure_scan(indio_dev, ADE9000_REG_WF_BUFF);
    ret = ade9000_waveform_buffer_interrupt_setup(st);
    if (ret)
    return ret;
    ret = regmap_set_bits(st.regmap, ADE9000_REG_WFB_CFG,
    ADE9000_WF_CAP_EN_MASK);
    if (ret) {
    dev_err(&st.spi.dev, "Post-enable waveform buffer enable fail\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ade9000_buffer_postdisable(indio_dev: *mut iio_dev) -> c_int {
    static int ade9000_buffer_postdisable(struct iio_dev *indio_dev)
    {
    struct ade9000_state *st = iio_priv(indio_dev);
    struct device *dev = &st.spi.dev;
    u32 interrupts;
    int ret;
    ret = regmap_clear_bits(st.regmap, ADE9000_REG_WFB_CFG,
    ADE9000_WF_CAP_EN_MASK);
    if (ret) {
    dev_err(dev, "Post-disable waveform buffer disable fail\n");
    return ret;
    }
    ret = regmap_write(st.regmap, ADE9000_REG_WFB_TRG_CFG, 0x0);
    if (ret)
    return ret;
    interrupts = ADE9000_ST0_WFB_TRIG_BIT | ADE9000_ST0_PAGE_FULL_BIT;
    ret = regmap_clear_bits(st.regmap, ADE9000_REG_MASK0, interrupts);
    if (ret) {
    dev_err(dev, "Post-disable update mask0 fail\n");
    return ret;
    }
    return regmap_write(st.regmap, ADE9000_REG_STATUS0, GENMASK(31, 0));
    }
    static const struct iio_buffer_setup_ops ade9000_buffer_ops = {
    .preenable = &ade9000_buffer_preenable,
    .postdisable = &ade9000_buffer_postdisable,
    };
#[no_mangle]
unsafe extern "C" fn ade9000_reset(st: *mut ade9000_state) -> c_int {
    static int ade9000_reset(struct ade9000_state *st)
    {
    struct device *dev = &st.spi.dev;
    struct gpio_desc *gpio_reset;
    int ret;
    gpio_reset = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(gpio_reset))
    return PTR_ERR(gpio_reset);
// Software reset via register if no GPIO available
    if (!gpio_reset) {
    ret = regmap_set_bits(st.regmap, ADE9000_REG_CONFIG1,
    ADE9000_SWRST_BIT);
    if (ret)
    return ret;
    fsleep(90);
    return 0;
    }
// Hardware reset via GPIO
    fsleep(10);
    gpiod_set_value_cansleep(gpio_reset, 0);
    fsleep(50000);
// Only wait for completion if IRQ1 is available to signal reset done
    if (fwnode_irq_get_byname(dev_fwnode(dev), "irq1") >= 0) {
    if (!wait_for_completion_timeout(&st.reset_completion,
    msecs_to_jiffies(1000)))
    return dev_err_probe(dev, -ETIMEDOUT,
    "Reset timeout after 1s\n");
    }
// If no IRQ available, reset is already complete after the 50ms delay above
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ade9000_setup(st: *mut ade9000_state) -> c_int {
    static int ade9000_setup(struct ade9000_state *st)
    {
    struct device *dev = &st.spi.dev;
    int ret;
    ret = regmap_multi_reg_write(st.regmap, ade9000_initialization_sequence,
    ARRAY_SIZE(ade9000_initialization_sequence));
    if (ret)
    return dev_err_probe(dev, ret, "Failed to write register sequence");
    fsleep(2000);
    return 0;
    }
    static const struct iio_info ade9000_info = {
    .read_raw = ade9000_read_raw,
    .write_raw = ade9000_write_raw,
    .debugfs_reg_access = ade9000_reg_access,
    .write_event_config = ade9000_write_event_config,
    .read_event_config = ade9000_read_event_config,
    .write_event_value = ade9000_write_event_value,
    .read_event_value = ade9000_read_event_value,
    };
    static const struct regmap_config ade9000_regmap_config = {
    .reg_bits = 16,
    .val_bits = 32,
    .max_register = 0x6bc,
    .zero_flag_mask = true,
    .cache_type = REGCACHE_MAPLE,
    .reg_read = ade9000_spi_read_reg,
    .reg_write = ade9000_spi_write_reg,
    .volatile_reg = ade9000_is_volatile_reg,
    };
#[no_mangle]
unsafe extern "C" fn ade9000_setup_clkout(dev: *mut device, st: *mut ade9000_state) -> c_int {
    static int ade9000_setup_clkout(struct device *dev, struct ade9000_state *st)
    {
    struct clk_hw *clkout_hw;
    int ret;
    if (!IS_ENABLED(CONFIG_COMMON_CLK))
    return 0;
//
// Only provide clock output when using external CMOS clock.
// When using crystal, CLKOUT is connected to crystal and shouldn't
// be used as clock provider for other devices.
//
    if (!device_property_present(dev, "#clock-cells") || !st.clkin)
    return 0;
// CLKOUT passes through CLKIN with divider of 1
    clkout_hw = devm_clk_hw_register_divider(dev, "clkout", __clk_get_name(st.clkin),
    CLK_SET_RATE_PARENT, core::ptr::null_mut(), 0, 1, 0, core::ptr::null_mut());
    if (IS_ERR(clkout_hw))
    return dev_err_probe(dev, PTR_ERR(clkout_hw), "Failed to register clkout");
    ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get, clkout_hw);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to add clock provider");
    return 0;
    }
    static int ade9000_request_irq(struct device *dev, const char *name,
    irq_handler_t handler, void *dev_id)
    {
    int irq, ret;
    irq = fwnode_irq_get_byname(dev_fwnode(dev), name);
    if (irq == -EINVAL)
    return 0; /* interrupts are optional */
    if (irq < 0)
    return dev_err_probe(dev, irq, "Failed to get %s irq", name);
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(), handler,
    IRQF_ONESHOT, KBUILD_MODNAME, dev_id);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to request %s irq", name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ade9000_probe(spi: *mut spi_device) -> c_int {
    static int ade9000_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct iio_dev *indio_dev;
    struct ade9000_state *st;
    struct regmap *regmap;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    regmap = devm_regmap_init(dev, core::ptr::null_mut(), st, &ade9000_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap), "Unable to allocate ADE9000 regmap");
    st.regmap = regmap;
    st.spi = spi;
    init_completion(&st.reset_completion);
    ret = devm_mutex_init(dev, &st.lock);
    if (ret)
    return ret;
    ret = ade9000_request_irq(dev, "irq0", ade9000_irq0_thread, indio_dev);
    if (ret)
    return ret;
    ret = ade9000_request_irq(dev, "irq1", ade9000_irq1_thread, indio_dev);
    if (ret)
    return ret;
    ret = ade9000_request_irq(dev, "dready", ade9000_dready_thread, indio_dev);
    if (ret)
    return ret;
// External CMOS clock input (optional - crystal can be used instead)
    st.clkin = devm_clk_get_optional_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(st.clkin))
    return dev_err_probe(dev, PTR_ERR(st.clkin), "Failed to get and enable clkin");
    ret = ade9000_setup_clkout(dev, st);
    if (ret)
    return ret;
    indio_dev.name = "ade9000";
    indio_dev.info = &ade9000_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.setup_ops = &ade9000_buffer_ops;
    ret = devm_regulator_get_enable(&spi.dev, "vdd");
    if (ret)
    return dev_err_probe(&spi.dev, ret,
    "Failed to get and enable vdd regulator\n");
    indio_dev.channels = ade9000_channels;
    indio_dev.num_channels = ARRAY_SIZE(ade9000_channels);
    ret = devm_iio_kfifo_buffer_setup(dev, indio_dev,
    &ade9000_buffer_ops);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to setup IIO buffer");
    ret = ade9000_reset(st);
    if (ret)
    return ret;
// Configure reference selection if vref regulator is available
    ret = devm_regulator_get_enable_optional(dev, "vref");
    if (ret != -ENODEV && ret >= 0) {
    ret = regmap_set_bits(st.regmap, ADE9000_REG_CONFIG1,
    ADE9000_EXT_REF_MASK);
    if (ret)
    return ret;
    } else if (ret < 0 && ret != -ENODEV) {
    return dev_err_probe(dev, ret,
    "Failed to get and enable vref regulator\n");
    }
    ret = ade9000_setup(st);
    if (ret)
    return ret;
    return devm_iio_device_register(dev, indio_dev);
    };
    static const struct spi_device_id ade9000_id[] = {
    { .name = "ade9000" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ade9000_id);
    static const struct of_device_id ade9000_of_match[] = {
    { .compatible = "adi,ade9000" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ade9000_of_match);
    static struct spi_driver ade9000_driver = {
    .driver = {
    .name = "ade9000",
    .of_match_table = ade9000_of_match,
    },
    .probe = ade9000_probe,
    .id_table = ade9000_id,
    };
    module_spi_driver(ade9000_driver);
    MODULE_AUTHOR("Antoniu Miclaus <antoniu.miclaus@analog.com>");
    MODULE_DESCRIPTION("Analog Devices ADE9000");
    MODULE_LICENSE("GPL");

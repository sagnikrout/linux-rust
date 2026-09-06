//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ti-ads124s08.c
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
// TI ADS124S0X chip family driver
// Copyright (C) 2018 Texas Instruments Incorporated - https://www.ti.com
//

// Commands
pub const ADS124S08_CMD_NOP: c_uint = 0x00;
pub const ADS124S08_CMD_WAKEUP: c_uint = 0x02;
pub const ADS124S08_CMD_PWRDWN: c_uint = 0x04;
pub const ADS124S08_CMD_RESET: c_uint = 0x06;
pub const ADS124S08_CMD_START: c_uint = 0x08;
pub const ADS124S08_CMD_STOP: c_uint = 0x0a;
pub const ADS124S08_CMD_SYOCAL: c_uint = 0x16;
pub const ADS124S08_CMD_SYGCAL: c_uint = 0x17;
pub const ADS124S08_CMD_SFOCAL: c_uint = 0x19;
pub const ADS124S08_CMD_RDATA: c_uint = 0x12;
pub const ADS124S08_CMD_RREG: c_uint = 0x20;
pub const ADS124S08_CMD_WREG: c_uint = 0x40;
// Registers
pub const ADS124S08_ID_REG: c_uint = 0x00;
pub const ADS124S08_STATUS: c_uint = 0x01;
pub const ADS124S08_INPUT_MUX: c_uint = 0x02;
pub const ADS124S08_PGA: c_uint = 0x03;
pub const ADS124S08_DATA_RATE: c_uint = 0x04;
pub const ADS124S08_REF: c_uint = 0x05;
pub const ADS124S08_IDACMAG: c_uint = 0x06;
pub const ADS124S08_IDACMUX: c_uint = 0x07;
pub const ADS124S08_VBIAS: c_uint = 0x08;
pub const ADS124S08_SYS: c_uint = 0x09;
pub const ADS124S08_OFCAL0: c_uint = 0x0a;
pub const ADS124S08_OFCAL1: c_uint = 0x0b;
pub const ADS124S08_OFCAL2: c_uint = 0x0c;
pub const ADS124S08_FSCAL0: c_uint = 0x0d;
pub const ADS124S08_FSCAL1: c_uint = 0x0e;
pub const ADS124S08_FSCAL2: c_uint = 0x0f;
pub const ADS124S08_GPIODAT: c_uint = 0x10;
pub const ADS124S08_GPIOCON: c_uint = 0x11;
// ADS124S0x common channels
pub const ADS124S08_AIN0: c_uint = 0x00;
pub const ADS124S08_AIN1: c_uint = 0x01;
pub const ADS124S08_AIN2: c_uint = 0x02;
pub const ADS124S08_AIN3: c_uint = 0x03;
pub const ADS124S08_AIN4: c_uint = 0x04;
pub const ADS124S08_AIN5: c_uint = 0x05;
pub const ADS124S08_AINCOM: c_uint = 0x0c;
// ADS124S08 only channels
pub const ADS124S08_AIN6: c_uint = 0x06;
pub const ADS124S08_AIN7: c_uint = 0x07;
pub const ADS124S08_AIN8: c_uint = 0x08;
pub const ADS124S08_AIN9: c_uint = 0x09;
pub const ADS124S08_AIN10: c_uint = 0x0a;
pub const ADS124S08_AIN11: c_uint = 0x0b;
pub const ADS124S08_MAX_CHANNELS: c_int = 12;
pub const ADS124S08_POS_MUX_SHIFT: c_uint = 0x04;
pub const ADS124S08_INT_REF: c_uint = 0x09;
pub const ADS124S08_START_REG_MASK: c_uint = 0x1f;
pub const ADS124S08_NUM_BYTES_MASK: c_uint = 0x1f;
pub const ADS124S08_START_CONV: c_uint = 0x01;
pub const ADS124S08_STOP_CONV: c_uint = 0x00;
    enum ads124s_id {
    ADS124S08_ID,
    ADS124S06_ID,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ads124s_chip_info {
    pub channels: *const iio_chan_spec,
    pub num_channels: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ads124s_private {
    pub chip_info: *const ads124s_chip_info,
    pub reset_gpio: *mut gpio_desc,
    pub spi: *mut spi_device,
    pub lock: mutex,
//
// Used to correctly align data.
// Ensure timestamp is naturally aligned.
// Note that the full buffer length may not be needed if not
// all channels are enabled, as long as the alignment of the
// timestamp is maintained.
//
    pub __aligned(8): u32 buffer[ADS124S08_MAX_CHANNELS + sizeof(s64)/sizeof(u32)],
    pub __aligned(IIO_DMA_MINALIGN): u8 data[5],
}

    {								\
    .type = IIO_VOLTAGE,					\
    .indexed = 1,						\
    .channel = index,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .scan_index = index,					\
    .scan_type = {						\
    .sign = 'u',					\
    .realbits = 32,					\
    .storagebits = 32,				\
    },							\
    }
    static const struct iio_chan_spec ads124s06_channels[] = {
    ADS124S08_CHAN(0),
    ADS124S08_CHAN(1),
    ADS124S08_CHAN(2),
    ADS124S08_CHAN(3),
    ADS124S08_CHAN(4),
    ADS124S08_CHAN(5),
    };
    static const struct iio_chan_spec ads124s08_channels[] = {
    ADS124S08_CHAN(0),
    ADS124S08_CHAN(1),
    ADS124S08_CHAN(2),
    ADS124S08_CHAN(3),
    ADS124S08_CHAN(4),
    ADS124S08_CHAN(5),
    ADS124S08_CHAN(6),
    ADS124S08_CHAN(7),
    ADS124S08_CHAN(8),
    ADS124S08_CHAN(9),
    ADS124S08_CHAN(10),
    ADS124S08_CHAN(11),
    };
    static const struct ads124s_chip_info ads124s_chip_info_tbl[] = {
    [ADS124S08_ID] = {
    .channels = ads124s08_channels,
    .num_channels = ARRAY_SIZE(ads124s08_channels),
    },
    [ADS124S06_ID] = {
    .channels = ads124s06_channels,
    .num_channels = ARRAY_SIZE(ads124s06_channels),
    },
    };
#[no_mangle]
unsafe extern "C" fn ads124s_write_cmd(indio_dev: *mut iio_dev, command: u8) -> c_int {
    static int ads124s_write_cmd(struct iio_dev *indio_dev, u8 command)
    {
    struct ads124s_private *priv = iio_priv(indio_dev);
    priv.data[0] = command;
    return spi_write(priv.spi, &priv.data[0], 1);
    }
#[no_mangle]
unsafe extern "C" fn ads124s_write_reg(indio_dev: *mut iio_dev, reg: u8, data: u8) -> c_int {
    static int ads124s_write_reg(struct iio_dev *indio_dev, u8 reg, u8 data)
    {
    struct ads124s_private *priv = iio_priv(indio_dev);
    priv.data[0] = ADS124S08_CMD_WREG | reg;
    priv.data[1] = 0x0;
    priv.data[2] = data;
    return spi_write(priv.spi, &priv.data[0], 3);
    }
#[no_mangle]
unsafe extern "C" fn ads124s_reset(indio_dev: *mut iio_dev) -> c_int {
    static int ads124s_reset(struct iio_dev *indio_dev)
    {
    struct ads124s_private *priv = iio_priv(indio_dev);
    if (priv.reset_gpio) {
    gpiod_set_value_cansleep(priv.reset_gpio, 0);
    fsleep(200);
    gpiod_set_value_cansleep(priv.reset_gpio, 1);
    } else {
    return ads124s_write_cmd(indio_dev, ADS124S08_CMD_RESET);
    }
    return 0;
    };
#[no_mangle]
unsafe extern "C" fn ads124s_read(indio_dev: *mut iio_dev) -> c_int {
    static int ads124s_read(struct iio_dev *indio_dev)
    {
    struct ads124s_private *priv = iio_priv(indio_dev);
    int ret;
    struct spi_transfer t[] = {
    {
    .tx_buf = &priv.data[0],
    .len = 4,
    .cs_change = 1,
    }, {
    .tx_buf = &priv.data[1],
    .rx_buf = &priv.data[1],
    .len = 4,
    },
    };
    priv.data[0] = ADS124S08_CMD_RDATA;
    memset(&priv.data[1], ADS124S08_CMD_NOP, sizeof(priv.data) - 1);
    ret = spi_sync_transfer(priv.spi, t, ARRAY_SIZE(t));
    if (ret < 0)
    return ret;
    return get_unaligned_be24(&priv.data[2]);
    }
    static int ads124s_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long m)
    {
    struct ads124s_private *priv = iio_priv(indio_dev);
    int ret;
    mutex_lock(&priv.lock);
    switch (m) {
    case IIO_CHAN_INFO_RAW:
    ret = ads124s_write_reg(indio_dev, ADS124S08_INPUT_MUX,
    chan.channel);
    if (ret) {
    dev_err(&priv.spi.dev, "Set ADC CH failed\n");
    goto out;
    }
    ret = ads124s_write_cmd(indio_dev, ADS124S08_START_CONV);
    if (ret) {
    dev_err(&priv.spi.dev, "Start conversions failed\n");
    goto out;
    }
    ret = ads124s_read(indio_dev);
    if (ret < 0) {
    dev_err(&priv.spi.dev, "Read ADC failed\n");
    goto out;
    }
// val = ret;
    ret = ads124s_write_cmd(indio_dev, ADS124S08_STOP_CONV);
    if (ret) {
    dev_err(&priv.spi.dev, "Stop conversions failed\n");
    goto out;
    }
    ret = IIO_VAL_INT;
    break;
    default:
    ret = -EINVAL;
    break;
    }
    out:
    mutex_unlock(&priv.lock);
    return ret;
    }
    static const struct iio_info ads124s_info = {
    .read_raw = &ads124s_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn ads124s_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t ads124s_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct ads124s_private *priv = iio_priv(indio_dev);
    int scan_index, j = 0;
    int ret;
    iio_for_each_active_channel(indio_dev, scan_index) {
    ret = ads124s_write_reg(indio_dev, ADS124S08_INPUT_MUX,
    scan_index);
    if (ret)
    dev_err(&priv.spi.dev, "Set ADC CH failed\n");
    ret = ads124s_write_cmd(indio_dev, ADS124S08_START_CONV);
    if (ret)
    dev_err(&priv.spi.dev, "Start ADC conversions failed\n");
    priv.buffer[j] = ads124s_read(indio_dev);
    ret = ads124s_write_cmd(indio_dev, ADS124S08_STOP_CONV);
    if (ret)
    dev_err(&priv.spi.dev, "Stop ADC conversions failed\n");
    j++;
    }
    iio_push_to_buffers_with_ts(indio_dev, priv.buffer, sizeof(priv.buffer),
    pf.timestamp);
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ads124s_probe(spi: *mut spi_device) -> c_int {
    static int ads124s_probe(struct spi_device *spi)
    {
    struct ads124s_private *ads124s_priv;
    struct iio_dev *indio_dev;
    const struct spi_device_id *spi_id = spi_get_device_id(spi);
    int ret;
    indio_dev = devm_iio_device_alloc(&spi.dev, sizeof(*ads124s_priv));
    if (indio_dev == core::ptr::null_mut())
    return -ENOMEM;
    ads124s_priv = iio_priv(indio_dev);
    ads124s_priv.reset_gpio = devm_gpiod_get_optional(&spi.dev,
    "reset", GPIOD_OUT_LOW);
    if (IS_ERR(ads124s_priv.reset_gpio))
    return dev_err_probe(&spi.dev, PTR_ERR(ads124s_priv.reset_gpio),
    "Failed to get reset GPIO\n");
    ads124s_priv.chip_info = &ads124s_chip_info_tbl[spi_id.driver_data];
    ads124s_priv.spi = spi;
    indio_dev.name = spi_id.name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = ads124s_priv.chip_info.channels;
    indio_dev.num_channels = ads124s_priv.chip_info.num_channels;
    indio_dev.info = &ads124s_info;
    mutex_init(&ads124s_priv.lock);
    ret = devm_iio_triggered_buffer_setup(&spi.dev, indio_dev, core::ptr::null_mut(),
    ads124s_trigger_handler, core::ptr::null_mut());
    if (ret) {
    dev_err(&spi.dev, "iio triggered buffer setup failed\n");
    return ret;
    }
    ads124s_reset(indio_dev);
    return devm_iio_device_register(&spi.dev, indio_dev);
    }
    static const struct spi_device_id ads124s_id[] = {
    { .name = "ads124s06", .driver_data = ADS124S06_ID },
    { .name = "ads124s08", .driver_data = ADS124S08_ID },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ads124s_id);
    static const struct of_device_id ads124s_of_table[] = {
    { .compatible = "ti,ads124s06" },
    { .compatible = "ti,ads124s08" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ads124s_of_table);
    static struct spi_driver ads124s_driver = {
    .driver = {
    .name	= "ads124s08",
    .of_match_table = ads124s_of_table,
    },
    .probe		= ads124s_probe,
    .id_table	= ads124s_id,
    };
    module_spi_driver(ads124s_driver);
    MODULE_AUTHOR("Dan Murphy <dmuprhy@ti.com>");
    MODULE_DESCRIPTION("TI TI_ADS12S0X ADC");
    MODULE_LICENSE("GPL v2");

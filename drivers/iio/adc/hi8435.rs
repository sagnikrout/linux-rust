//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/hi8435.c
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
// Holt Integrated Circuits HI-8435 threshold detector driver
//
// Copyright (C) 2015 Zodiac Inflight Innovations
// Copyright (C) 2015 Cogent Embedded, Inc.
//

// Register offsets for HI-8435
pub const HI8435_CTRL_REG: c_uint = 0x02;
pub const HI8435_PSEN_REG: c_uint = 0x04;
pub const HI8435_TMDATA_REG: c_uint = 0x1E;
pub const HI8435_GOCENHYS_REG: c_uint = 0x3A;
pub const HI8435_SOCENHYS_REG: c_uint = 0x3C;
pub const HI8435_SO7_0_REG: c_uint = 0x10;
pub const HI8435_SO15_8_REG: c_uint = 0x12;
pub const HI8435_SO23_16_REG: c_uint = 0x14;
pub const HI8435_SO31_24_REG: c_uint = 0x16;
pub const HI8435_SO31_0_REG: c_uint = 0x78;
pub const HI8435_WRITE_OPCODE: c_uint = 0x00;
pub const HI8435_READ_OPCODE: c_uint = 0x80;
// CTRL register bits
pub const HI8435_CTRL_TEST: c_uint = 0x01;
pub const HI8435_CTRL_SRST: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi8435_priv {
    pub spi: *mut spi_device,
    pub lock: mutex,
    pub /: *mut *mut unsigned long event_scan_mask; / soft mask/unmask channels events,
    pub event_prev_val: c_uint,
    pub /: *mut *mut unsigned threshold_lo[2]; / GND-Open and Supply-Open thresholds,
    pub /: *mut *mut unsigned threshold_hi[2]; / GND-Open and Supply-Open thresholds,
    pub __aligned(IIO_DMA_MINALIGN): u8 reg_buffer[3],
}

#[no_mangle]
unsafe extern "C" fn hi8435_readb(priv: *mut hi8435_priv, reg: u8, val: *mut u8) -> c_int {
    static int hi8435_readb(struct hi8435_priv *priv, u8 reg, u8 *val)
    {
    reg |= HI8435_READ_OPCODE;
    return spi_write_then_read(priv.spi, &reg, 1, val, 1);
    }
#[no_mangle]
unsafe extern "C" fn hi8435_readw(priv: *mut hi8435_priv, reg: u8, val: *mut u16) -> c_int {
    static int hi8435_readw(struct hi8435_priv *priv, u8 reg, u16 *val)
    {
    int ret;
    __be16 be_val;
    reg |= HI8435_READ_OPCODE;
    ret = spi_write_then_read(priv.spi, &reg, 1, &be_val, 2);
// val = be16_to_cpu(be_val);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hi8435_readl(priv: *mut hi8435_priv, reg: u8, val: *mut u32) -> c_int {
    static int hi8435_readl(struct hi8435_priv *priv, u8 reg, u32 *val)
    {
    int ret;
    __be32 be_val;
    reg |= HI8435_READ_OPCODE;
    ret = spi_write_then_read(priv.spi, &reg, 1, &be_val, 4);
// val = be32_to_cpu(be_val);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hi8435_writeb(priv: *mut hi8435_priv, reg: u8, val: u8) -> c_int {
    static int hi8435_writeb(struct hi8435_priv *priv, u8 reg, u8 val)
    {
    priv.reg_buffer[0] = reg | HI8435_WRITE_OPCODE;
    priv.reg_buffer[1] = val;
    return spi_write(priv.spi, priv.reg_buffer, 2);
    }
#[no_mangle]
unsafe extern "C" fn hi8435_writew(priv: *mut hi8435_priv, reg: u8, val: u16) -> c_int {
    static int hi8435_writew(struct hi8435_priv *priv, u8 reg, u16 val)
    {
    priv.reg_buffer[0] = reg | HI8435_WRITE_OPCODE;
    priv.reg_buffer[1] = (val >> 8) & 0xff;
    priv.reg_buffer[2] = val & 0xff;
    return spi_write(priv.spi, priv.reg_buffer, 3);
    }
    static int hi8435_read_raw(struct iio_dev *idev,
    const struct iio_chan_spec *chan,
    int *val, int *val2, long mask)
    {
    struct hi8435_priv *priv = iio_priv(idev);
    u32 tmp;
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = hi8435_readl(priv, HI8435_SO31_0_REG, &tmp);
    if (ret < 0)
    return ret;
// val = !!(tmp & BIT(chan->channel));
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
    static int hi8435_read_event_config(struct iio_dev *idev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir)
    {
    struct hi8435_priv *priv = iio_priv(idev);
    return !!(priv.event_scan_mask & BIT(chan.channel));
    }
    static int hi8435_write_event_config(struct iio_dev *idev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir, bool state)
    {
    struct hi8435_priv *priv = iio_priv(idev);
    int ret;
    u32 tmp;
    if (state) {
    ret = hi8435_readl(priv, HI8435_SO31_0_REG, &tmp);
    if (ret < 0)
    return ret;
    if (tmp & BIT(chan.channel))
    priv.event_prev_val |= BIT(chan.channel);
    else
    priv.event_prev_val &= ~BIT(chan.channel);
    priv.event_scan_mask |= BIT(chan.channel);
    } else
    priv.event_scan_mask &= ~BIT(chan.channel);
    return 0;
    }
    static int hi8435_read_event_value(struct iio_dev *idev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info,
    int *val, int *val2)
    {
    struct hi8435_priv *priv = iio_priv(idev);
    int ret;
    u8 mode, psen;
    u16 reg;
    ret = hi8435_readb(priv, HI8435_PSEN_REG, &psen);
    if (ret < 0)
    return ret;
// Supply-Open or GND-Open sensing mode
    mode = !!(psen & BIT(chan.channel / 8));
    ret = hi8435_readw(priv, mode ? HI8435_SOCENHYS_REG :
    HI8435_GOCENHYS_REG, &reg);
    if (ret < 0)
    return ret;
    if (dir == IIO_EV_DIR_FALLING)
// val = ((reg & 0xff) - (reg >> 8)) / 2;
#[no_mangle]
pub unsafe extern "C" fn if(IIO_EV_DIR_RISING: dir ==) -> else {
    else if (dir == IIO_EV_DIR_RISING)
// val = ((reg & 0xff) + (reg >> 8)) / 2;
    return IIO_VAL_INT;
    }
    static int hi8435_write_event_value(struct iio_dev *idev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info,
    int val, int val2)
    {
    struct hi8435_priv *priv = iio_priv(idev);
    int ret;
    u8 mode, psen;
    u16 reg;
    ret = hi8435_readb(priv, HI8435_PSEN_REG, &psen);
    if (ret < 0)
    return ret;
// Supply-Open or GND-Open sensing mode
    mode = !!(psen & BIT(chan.channel / 8));
    ret = hi8435_readw(priv, mode ? HI8435_SOCENHYS_REG :
    HI8435_GOCENHYS_REG, &reg);
    if (ret < 0)
    return ret;
    if (dir == IIO_EV_DIR_FALLING) {
// falling threshold range 2..21V, hysteresis minimum 2V
    if (val < 2 || val > 21 || (val + 2) > priv.threshold_hi[mode])
    return -EINVAL;
    if (val == priv.threshold_lo[mode])
    return 0;
    priv.threshold_lo[mode] = val;
// hysteresis must not be odd
    if ((priv.threshold_hi[mode] - priv.threshold_lo[mode]) % 2)
    priv.threshold_hi[mode]--;
    } else if (dir == IIO_EV_DIR_RISING) {
// rising threshold range 3..22V, hysteresis minimum 2V
    if (val < 3 || val > 22 || val < (priv.threshold_lo[mode] + 2))
    return -EINVAL;
    if (val == priv.threshold_hi[mode])
    return 0;
    priv.threshold_hi[mode] = val;
// hysteresis must not be odd
    if ((priv.threshold_hi[mode] - priv.threshold_lo[mode]) % 2)
    priv.threshold_lo[mode]++;
    }
// program thresholds
    mutex_lock(&priv.lock);
    ret = hi8435_readw(priv, mode ? HI8435_SOCENHYS_REG :
    HI8435_GOCENHYS_REG, &reg);
    if (ret < 0) {
    mutex_unlock(&priv.lock);
    return ret;
    }
// hysteresis
    reg = priv.threshold_hi[mode] - priv.threshold_lo[mode];
    reg <<= 8;
// threshold center
    reg |= (priv.threshold_hi[mode] + priv.threshold_lo[mode]);
    ret = hi8435_writew(priv, mode ? HI8435_SOCENHYS_REG :
    HI8435_GOCENHYS_REG, reg);
    mutex_unlock(&priv.lock);
    return ret;
    }
    static int hi8435_debugfs_reg_access(struct iio_dev *idev,
    unsigned reg, unsigned writeval,
    unsigned *readval)
    {
    struct hi8435_priv *priv = iio_priv(idev);
    int ret;
    u8 val;
    if (readval != core::ptr::null_mut()) {
    ret = hi8435_readb(priv, reg, &val);
// readval = val;
    } else {
    val = (u8)writeval;
    ret = hi8435_writeb(priv, reg, val);
    }
    return ret;
    }
    static const struct iio_event_spec hi8435_events[] = {
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_RISING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE),
    }, {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_FALLING,
    .mask_separate = BIT(IIO_EV_INFO_VALUE),
    }, {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_EITHER,
    .mask_separate = BIT(IIO_EV_INFO_ENABLE),
    },
    };
    static int hi8435_get_sensing_mode(struct iio_dev *idev,
    const struct iio_chan_spec *chan)
    {
    struct hi8435_priv *priv = iio_priv(idev);
    int ret;
    u8 reg;
    ret = hi8435_readb(priv, HI8435_PSEN_REG, &reg);
    if (ret < 0)
    return ret;
    return !!(reg & BIT(chan.channel / 8));
    }
    static int hi8435_set_sensing_mode(struct iio_dev *idev,
    const struct iio_chan_spec *chan,
    unsigned int mode)
    {
    struct hi8435_priv *priv = iio_priv(idev);
    int ret;
    u8 reg;
    mutex_lock(&priv.lock);
    ret = hi8435_readb(priv, HI8435_PSEN_REG, &reg);
    if (ret < 0) {
    mutex_unlock(&priv.lock);
    return ret;
    }
    reg &= ~BIT(chan.channel / 8);
    if (mode)
    reg |= BIT(chan.channel / 8);
    ret = hi8435_writeb(priv, HI8435_PSEN_REG, reg);
    mutex_unlock(&priv.lock);
    return ret;
    }
    static const char * const hi8435_sensing_modes[] = { "GND-Open",
    "Supply-Open" };
    static const struct iio_enum hi8435_sensing_mode = {
    .items = hi8435_sensing_modes,
    .num_items = ARRAY_SIZE(hi8435_sensing_modes),
    .get = hi8435_get_sensing_mode,
    .set = hi8435_set_sensing_mode,
    };
    static const struct iio_chan_spec_ext_info hi8435_ext_info[] = {
    IIO_ENUM("sensing_mode", IIO_SEPARATE, &hi8435_sensing_mode),
    IIO_ENUM_AVAILABLE("sensing_mode", IIO_SHARED_BY_TYPE, &hi8435_sensing_mode),
    { }
    };

    {							\
    .type = IIO_VOLTAGE,				\
    .indexed = 1,					\
    .channel = num,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),	\
    .event_spec = hi8435_events,			\
    .num_event_specs = ARRAY_SIZE(hi8435_events),	\
    .ext_info = hi8435_ext_info,			\
    }
    static const struct iio_chan_spec hi8435_channels[] = {
    HI8435_VOLTAGE_CHANNEL(0),
    HI8435_VOLTAGE_CHANNEL(1),
    HI8435_VOLTAGE_CHANNEL(2),
    HI8435_VOLTAGE_CHANNEL(3),
    HI8435_VOLTAGE_CHANNEL(4),
    HI8435_VOLTAGE_CHANNEL(5),
    HI8435_VOLTAGE_CHANNEL(6),
    HI8435_VOLTAGE_CHANNEL(7),
    HI8435_VOLTAGE_CHANNEL(8),
    HI8435_VOLTAGE_CHANNEL(9),
    HI8435_VOLTAGE_CHANNEL(10),
    HI8435_VOLTAGE_CHANNEL(11),
    HI8435_VOLTAGE_CHANNEL(12),
    HI8435_VOLTAGE_CHANNEL(13),
    HI8435_VOLTAGE_CHANNEL(14),
    HI8435_VOLTAGE_CHANNEL(15),
    HI8435_VOLTAGE_CHANNEL(16),
    HI8435_VOLTAGE_CHANNEL(17),
    HI8435_VOLTAGE_CHANNEL(18),
    HI8435_VOLTAGE_CHANNEL(19),
    HI8435_VOLTAGE_CHANNEL(20),
    HI8435_VOLTAGE_CHANNEL(21),
    HI8435_VOLTAGE_CHANNEL(22),
    HI8435_VOLTAGE_CHANNEL(23),
    HI8435_VOLTAGE_CHANNEL(24),
    HI8435_VOLTAGE_CHANNEL(25),
    HI8435_VOLTAGE_CHANNEL(26),
    HI8435_VOLTAGE_CHANNEL(27),
    HI8435_VOLTAGE_CHANNEL(28),
    HI8435_VOLTAGE_CHANNEL(29),
    HI8435_VOLTAGE_CHANNEL(30),
    HI8435_VOLTAGE_CHANNEL(31),
    IIO_CHAN_SOFT_TIMESTAMP(32),
    };
    static const struct iio_info hi8435_info = {
    .read_raw = hi8435_read_raw,
    .read_event_config = hi8435_read_event_config,
    .write_event_config = hi8435_write_event_config,
    .read_event_value = hi8435_read_event_value,
    .write_event_value = hi8435_write_event_value,
    .debugfs_reg_access = hi8435_debugfs_reg_access,
    };
#[no_mangle]
unsafe extern "C" fn hi8435_iio_push_event(idev: *mut iio_dev, val: c_uint) {
    static void hi8435_iio_push_event(struct iio_dev *idev, unsigned int val)
    {
    struct hi8435_priv *priv = iio_priv(idev);
    enum iio_event_direction dir;
    unsigned int i;
    let mut status: c_uint = priv.event_prev_val ^ val;
    if (!status)
    return;
    for_each_set_bit(i, &priv.event_scan_mask, 32) {
    if (status & BIT(i)) {
    dir = val & BIT(i) ? IIO_EV_DIR_RISING :
    IIO_EV_DIR_FALLING;
    iio_push_event(idev,
    IIO_UNMOD_EVENT_CODE(IIO_VOLTAGE, i,
    IIO_EV_TYPE_THRESH, dir),
    iio_get_time_ns(idev));
    }
    }
    priv.event_prev_val = val;
    }
#[no_mangle]
unsafe extern "C" fn hi8435_trigger_handler(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t hi8435_trigger_handler(int irq, void *private)
    {
    struct iio_poll_func *pf = private;
    struct iio_dev *idev = pf.indio_dev;
    struct hi8435_priv *priv = iio_priv(idev);
    u32 val;
    int ret;
    ret = hi8435_readl(priv, HI8435_SO31_0_REG, &val);
    if (ret < 0)
    goto err_read;
    hi8435_iio_push_event(idev, val);
    err_read:
    iio_trigger_notify_done(idev.trig);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hi8435_triggered_event_cleanup(data: *mut c_void) {
    static void hi8435_triggered_event_cleanup(void *data)
    {
    iio_triggered_event_cleanup(data);
    }
#[no_mangle]
unsafe extern "C" fn hi8435_probe(spi: *mut spi_device) -> c_int {
    static int hi8435_probe(struct spi_device *spi)
    {
    struct iio_dev *idev;
    struct hi8435_priv *priv;
    struct gpio_desc *reset_gpio;
    int ret;
    idev = devm_iio_device_alloc(&spi.dev, sizeof(*priv));
    if (!idev)
    return -ENOMEM;
    priv = iio_priv(idev);
    priv.spi = spi;
    reset_gpio = devm_gpiod_get(&spi.dev, core::ptr::null_mut(), GPIOD_OUT_LOW);
    if (IS_ERR(reset_gpio)) {
// chip s/w reset if h/w reset failed
    hi8435_writeb(priv, HI8435_CTRL_REG, HI8435_CTRL_SRST);
    hi8435_writeb(priv, HI8435_CTRL_REG, 0);
    } else {
    udelay(5);
    gpiod_set_value_cansleep(reset_gpio, 1);
    }
    mutex_init(&priv.lock);
    idev.name		= spi_get_device_id(spi).name;
    idev.modes		= INDIO_DIRECT_MODE;
    idev.info		= &hi8435_info;
    idev.channels		= hi8435_channels;
    idev.num_channels	= ARRAY_SIZE(hi8435_channels);
// unmask all events
    priv.event_scan_mask = ~(0);
//
// There is a restriction in the chip - the hysteresis can not be odd.
// If the hysteresis is set to odd value then chip gets into lock state
// and not functional anymore.
// After chip reset the thresholds are in undefined state, so we need to
// initialize thresholds to some initial values and then prevent
// userspace setting odd hysteresis.
//
// Set threshold low voltage to 2V, threshold high voltage to 4V
// for both GND-Open and Supply-Open sensing modes.
//
    priv.threshold_lo[0] = priv.threshold_lo[1] = 2;
    priv.threshold_hi[0] = priv.threshold_hi[1] = 4;
    hi8435_writew(priv, HI8435_GOCENHYS_REG, 0x206);
    hi8435_writew(priv, HI8435_SOCENHYS_REG, 0x206);
    ret = iio_triggered_event_setup(idev, core::ptr::null_mut(), hi8435_trigger_handler);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(&spi.dev,
    hi8435_triggered_event_cleanup,
    idev);
    if (ret)
    return ret;
    return devm_iio_device_register(&spi.dev, idev);
    }
    static const struct of_device_id hi8435_dt_ids[] = {
    { .compatible = "holt,hi8435" },
    { }
    };
    MODULE_DEVICE_TABLE(of, hi8435_dt_ids);
    static const struct spi_device_id hi8435_id[] = {
    { .name = "hi8435" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, hi8435_id);
    static struct spi_driver hi8435_driver = {
    .driver	= {
    .name		= "hi8435",
    .of_match_table	= hi8435_dt_ids,
    },
    .probe		= hi8435_probe,
    .id_table	= hi8435_id,
    };
    module_spi_driver(hi8435_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Vladimir Barinov");
    MODULE_DESCRIPTION("HI-8435 threshold detector");

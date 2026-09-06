//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/cm36651.c
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
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// Author: Beomho Seo <beomho.seo@samsung.com>
//

// Slave address 0x19 for PS of 7 bit addressing protocol for I2C
pub const CM36651_I2C_ADDR_PS: c_uint = 0x19;
// Alert Response Address
pub const CM36651_ARA: c_uint = 0x0C;
// Ambient light sensor
pub const CM36651_CS_CONF1: c_uint = 0x00;
pub const CM36651_CS_CONF2: c_uint = 0x01;
pub const CM36651_ALS_WH_M: c_uint = 0x02;
pub const CM36651_ALS_WH_L: c_uint = 0x03;
pub const CM36651_ALS_WL_M: c_uint = 0x04;
pub const CM36651_ALS_WL_L: c_uint = 0x05;
pub const CM36651_CS_CONF3: c_uint = 0x06;
pub const CM36651_CS_CONF_REG_NUM: c_uint = 0x02;
// Proximity sensor
pub const CM36651_PS_CONF1: c_uint = 0x00;
pub const CM36651_PS_THD: c_uint = 0x01;
pub const CM36651_PS_CANC: c_uint = 0x02;
pub const CM36651_PS_CONF2: c_uint = 0x03;
pub const CM36651_PS_REG_NUM: c_uint = 0x04;
// CS_CONF1 command code
pub const CM36651_ALS_ENABLE: c_uint = 0x00;
pub const CM36651_ALS_DISABLE: c_uint = 0x01;
pub const CM36651_ALS_INT_EN: c_uint = 0x02;
pub const CM36651_ALS_THRES: c_uint = 0x04;
// CS_CONF2 command code
pub const CM36651_CS_CONF2_DEFAULT_BIT: c_uint = 0x08;
// CS_CONF3 channel integration time
pub const CM36651_CS_IT1: c_uint = 0x00 /* Integration time 80 msec */;
pub const CM36651_CS_IT2: c_uint = 0x40 /* Integration time 160 msec */;
pub const CM36651_CS_IT3: c_uint = 0x80 /* Integration time 320 msec */;
pub const CM36651_CS_IT4: c_uint = 0xC0 /* Integration time 640 msec */;
// PS_CONF1 command code
pub const CM36651_PS_ENABLE: c_uint = 0x00;
pub const CM36651_PS_DISABLE: c_uint = 0x01;
pub const CM36651_PS_INT_EN: c_uint = 0x02;
pub const CM36651_PS_PERS2: c_uint = 0x04;
pub const CM36651_PS_PERS3: c_uint = 0x08;
pub const CM36651_PS_PERS4: c_uint = 0x0C;
// PS_CONF1 command code: integration time
pub const CM36651_PS_IT1: c_uint = 0x00 /* Integration time 0.32 msec */;
pub const CM36651_PS_IT2: c_uint = 0x10 /* Integration time 0.42 msec */;
pub const CM36651_PS_IT3: c_uint = 0x20 /* Integration time 0.52 msec */;
pub const CM36651_PS_IT4: c_uint = 0x30 /* Integration time 0.64 msec */;
// PS_CONF1 command code: duty ratio
pub const CM36651_PS_DR1: c_uint = 0x00 /* Duty ratio 1/80 */;
pub const CM36651_PS_DR2: c_uint = 0x40 /* Duty ratio 1/160 */;
pub const CM36651_PS_DR3: c_uint = 0x80 /* Duty ratio 1/320 */;
pub const CM36651_PS_DR4: c_uint = 0xC0 /* Duty ratio 1/640 */;
// PS_THD command code
pub const CM36651_PS_INITIAL_THD: c_uint = 0x05;
// PS_CANC command code
pub const CM36651_PS_CANC_DEFAULT: c_uint = 0x00;
// PS_CONF2 command code
pub const CM36651_PS_HYS1: c_uint = 0x00;
pub const CM36651_PS_HYS2: c_uint = 0x01;
pub const CM36651_PS_SMART_PERS_EN: c_uint = 0x02;
pub const CM36651_PS_DIR_INT: c_uint = 0x04;
pub const CM36651_PS_MS: c_uint = 0x10;
pub const CM36651_CS_COLOR_NUM: c_int = 4;
pub const CM36651_CLOSE_PROXIMITY: c_uint = 0x32;
pub const CM36651_FAR_PROXIMITY: c_uint = 0x33;

    enum cm36651_operation_mode {
    CM36651_LIGHT_EN,
    CM36651_PROXIMITY_EN,
    CM36651_PROXIMITY_EV_EN,
    };
    enum cm36651_light_channel_idx {
    CM36651_LIGHT_CHANNEL_IDX_RED,
    CM36651_LIGHT_CHANNEL_IDX_GREEN,
    CM36651_LIGHT_CHANNEL_IDX_BLUE,
    CM36651_LIGHT_CHANNEL_IDX_CLEAR,
    };
    enum cm36651_command {
    CM36651_CMD_READ_RAW_LIGHT,
    CM36651_CMD_READ_RAW_PROXIMITY,
    CM36651_CMD_PROX_EV_EN,
    CM36651_CMD_PROX_EV_DIS,
    };
    static const u8 cm36651_cs_reg[CM36651_CS_CONF_REG_NUM] = {
    CM36651_CS_CONF1,
    CM36651_CS_CONF2,
    };
    static const u8 cm36651_ps_reg[CM36651_PS_REG_NUM] = {
    CM36651_PS_CONF1,
    CM36651_PS_THD,
    CM36651_PS_CANC,
    CM36651_PS_CONF2,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cm36651_data {
    pub pdata: *const cm36651_platform_data,
    pub client: *mut i2c_client,
    pub ps_client: *mut i2c_client,
    pub ara_client: *mut i2c_client,
    pub lock: mutex,
    pub vled_reg: *mut regulator,
    pub flags: c_ulong,
    pub cs_int_time: [c_int; CM36651_CS_COLOR_NUM],
    pub ps_int_time: c_int,
    pub cs_ctrl_regs: [u8; CM36651_CS_CONF_REG_NUM],
    pub ps_ctrl_regs: [u8; CM36651_PS_REG_NUM],
    pub color: [u16; CM36651_CS_COLOR_NUM],
}

#[no_mangle]
unsafe extern "C" fn cm36651_setup_reg(cm36651: *mut cm36651_data) -> c_int {
    static int cm36651_setup_reg(struct cm36651_data *cm36651)
    {
    struct i2c_client *client = cm36651.client;
    struct i2c_client *ps_client = cm36651.ps_client;
    int i, ret;
// CS initialization
    cm36651.cs_ctrl_regs[CM36651_CS_CONF1] = CM36651_ALS_ENABLE |
    CM36651_ALS_THRES;
    cm36651.cs_ctrl_regs[CM36651_CS_CONF2] = CM36651_CS_CONF2_DEFAULT_BIT;
    for (i = 0; i < CM36651_CS_CONF_REG_NUM; i++) {
    ret = i2c_smbus_write_byte_data(client, cm36651_cs_reg[i],
    cm36651.cs_ctrl_regs[i]);
    if (ret < 0)
    return ret;
    }
// PS initialization
    cm36651.ps_ctrl_regs[CM36651_PS_CONF1] = CM36651_PS_ENABLE |
    CM36651_PS_IT2;
    cm36651.ps_ctrl_regs[CM36651_PS_THD] = CM36651_PS_INITIAL_THD;
    cm36651.ps_ctrl_regs[CM36651_PS_CANC] = CM36651_PS_CANC_DEFAULT;
    cm36651.ps_ctrl_regs[CM36651_PS_CONF2] = CM36651_PS_HYS2 |
    CM36651_PS_DIR_INT | CM36651_PS_SMART_PERS_EN;
    for (i = 0; i < CM36651_PS_REG_NUM; i++) {
    ret = i2c_smbus_write_byte_data(ps_client, cm36651_ps_reg[i],
    cm36651.ps_ctrl_regs[i]);
    if (ret < 0)
    return ret;
    }
// Set shutdown mode
    ret = i2c_smbus_write_byte_data(client, CM36651_CS_CONF1,
    CM36651_ALS_DISABLE);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(cm36651.ps_client,
    CM36651_PS_CONF1, CM36651_PS_DISABLE);
    if (ret < 0)
    return ret;
    return 0;
    }
    static int cm36651_read_output(struct cm36651_data *cm36651,
    struct iio_chan_spec const *chan, int *val)
    {
    struct i2c_client *client = cm36651.client;
    let mut ret: c_int = -EINVAL;
    switch (chan.type) {
    case IIO_LIGHT:
// val = i2c_smbus_read_word_data(client, chan->address);
    if (*val < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(client, CM36651_CS_CONF1,
    CM36651_ALS_DISABLE);
    if (ret < 0)
    return ret;
    ret = IIO_VAL_INT;
    break;
    case IIO_PROXIMITY:
// val = i2c_smbus_read_byte(cm36651->ps_client);
    if (*val < 0)
    return ret;
    if (!test_bit(CM36651_PROXIMITY_EV_EN, &cm36651.flags)) {
    ret = i2c_smbus_write_byte_data(cm36651.ps_client,
    CM36651_PS_CONF1, CM36651_PS_DISABLE);
    if (ret < 0)
    return ret;
    }
    ret = IIO_VAL_INT;
    break;
    default:
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cm36651_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t cm36651_irq_handler(int irq, void *data)
    {
    struct iio_dev *indio_dev = data;
    struct cm36651_data *cm36651 = iio_priv(indio_dev);
    struct i2c_client *client = cm36651.client;
    int ev_dir, ret;
    u64 ev_code;
//
// The PS INT pin is an active low signal that PS INT move logic low
// when the object is detect. Once the MCU host received the PS INT
// "LOW" signal, the Host needs to read the data at Alert Response
// Address(ARA) to clear the PS INT signal. After clearing the PS
// INT pin, the PS INT signal toggles from low to high.
//
    ret = i2c_smbus_read_byte(cm36651.ara_client);
    if (ret < 0) {
    dev_err(&client.dev,
    "%s: Data read failed: %d\n", __func__, ret);
    return IRQ_HANDLED;
    }
    switch (ret) {
    case CM36651_CLOSE_PROXIMITY:
    ev_dir = IIO_EV_DIR_RISING;
    break;
    case CM36651_FAR_PROXIMITY:
    ev_dir = IIO_EV_DIR_FALLING;
    break;
    default:
    dev_err(&client.dev,
    "%s: Data read wrong: %d\n", __func__, ret);
    return IRQ_HANDLED;
    }
    ev_code = IIO_UNMOD_EVENT_CODE(IIO_PROXIMITY,
    CM36651_CMD_READ_RAW_PROXIMITY,
    IIO_EV_TYPE_THRESH, ev_dir);
    iio_push_event(indio_dev, ev_code, iio_get_time_ns(indio_dev));
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cm36651_set_operation_mode(cm36651: *mut cm36651_data, cmd: c_int) -> c_int {
    static int cm36651_set_operation_mode(struct cm36651_data *cm36651, int cmd)
    {
    struct i2c_client *client = cm36651.client;
    struct i2c_client *ps_client = cm36651.ps_client;
    let mut ret: c_int = -EINVAL;
    switch (cmd) {
    case CM36651_CMD_READ_RAW_LIGHT:
    ret = i2c_smbus_write_byte_data(client, CM36651_CS_CONF1,
    cm36651.cs_ctrl_regs[CM36651_CS_CONF1]);
    break;
    case CM36651_CMD_READ_RAW_PROXIMITY:
    if (test_bit(CM36651_PROXIMITY_EV_EN, &cm36651.flags))
    return CM36651_PROXIMITY_EV_EN;
    ret = i2c_smbus_write_byte_data(ps_client, CM36651_PS_CONF1,
    cm36651.ps_ctrl_regs[CM36651_PS_CONF1]);
    break;
    case CM36651_CMD_PROX_EV_EN:
    if (test_bit(CM36651_PROXIMITY_EV_EN, &cm36651.flags)) {
    dev_err(&client.dev,
    "Already proximity event enable state\n");
    return ret;
    }
    set_bit(CM36651_PROXIMITY_EV_EN, &cm36651.flags);
    ret = i2c_smbus_write_byte_data(ps_client,
    cm36651_ps_reg[CM36651_PS_CONF1],
    CM36651_PS_INT_EN | CM36651_PS_PERS2 | CM36651_PS_IT2);
    if (ret < 0) {
    dev_err(&client.dev, "Proximity enable event failed\n");
    return ret;
    }
    break;
    case CM36651_CMD_PROX_EV_DIS:
    if (!test_bit(CM36651_PROXIMITY_EV_EN, &cm36651.flags)) {
    dev_err(&client.dev,
    "Already proximity event disable state\n");
    return ret;
    }
    clear_bit(CM36651_PROXIMITY_EV_EN, &cm36651.flags);
    ret = i2c_smbus_write_byte_data(ps_client,
    CM36651_PS_CONF1, CM36651_PS_DISABLE);
    break;
    }
    if (ret < 0)
    dev_err(&client.dev, "Write register failed\n");
    return ret;
    }
    static int cm36651_read_channel(struct cm36651_data *cm36651,
    struct iio_chan_spec const *chan, int *val)
    {
    struct i2c_client *client = cm36651.client;
    int cmd, ret;
    if (chan.type == IIO_LIGHT)
    cmd = CM36651_CMD_READ_RAW_LIGHT;
#[no_mangle]
pub unsafe extern "C" fn if(IIO_PROXIMITY: chan->type ==) -> else {
    else if (chan.type == IIO_PROXIMITY)
    cmd = CM36651_CMD_READ_RAW_PROXIMITY;
    else
    return -EINVAL;
    ret = cm36651_set_operation_mode(cm36651, cmd);
    if (ret < 0) {
    dev_err(&client.dev, "CM36651 set operation mode failed\n");
    return ret;
    }
// Delay for work after enable operation
    msleep(50);
    ret = cm36651_read_output(cm36651, chan, val);
    if (ret < 0) {
    dev_err(&client.dev, "CM36651 read output failed\n");
    return ret;
    }
    return ret;
    }
    static int cm36651_read_int_time(struct cm36651_data *cm36651,
    struct iio_chan_spec const *chan, int *val2)
    {
    switch (chan.type) {
    case IIO_LIGHT:
    if (cm36651.cs_int_time[chan.address] == CM36651_CS_IT1)
// val2 = 80000;
#[no_mangle]
pub unsafe extern "C" fn if(CM36651_CS_IT2: cm36651->cs_int_time[chan->address] ==) -> else {
    else if (cm36651.cs_int_time[chan.address] == CM36651_CS_IT2)
// val2 = 160000;
#[no_mangle]
pub unsafe extern "C" fn if(CM36651_CS_IT3: cm36651->cs_int_time[chan->address] ==) -> else {
    else if (cm36651.cs_int_time[chan.address] == CM36651_CS_IT3)
// val2 = 320000;
#[no_mangle]
pub unsafe extern "C" fn if(CM36651_CS_IT4: cm36651->cs_int_time[chan->address] ==) -> else {
    else if (cm36651.cs_int_time[chan.address] == CM36651_CS_IT4)
// val2 = 640000;
    else
    return -EINVAL;
    break;
    case IIO_PROXIMITY:
    if (cm36651.ps_int_time == CM36651_PS_IT1)
// val2 = 320;
#[no_mangle]
pub unsafe extern "C" fn if(CM36651_PS_IT2: cm36651->ps_int_time ==) -> else {
    else if (cm36651.ps_int_time == CM36651_PS_IT2)
// val2 = 420;
#[no_mangle]
pub unsafe extern "C" fn if(CM36651_PS_IT3: cm36651->ps_int_time ==) -> else {
    else if (cm36651.ps_int_time == CM36651_PS_IT3)
// val2 = 520;
#[no_mangle]
pub unsafe extern "C" fn if(CM36651_PS_IT4: cm36651->ps_int_time ==) -> else {
    else if (cm36651.ps_int_time == CM36651_PS_IT4)
// val2 = 640;
    else
    return -EINVAL;
    break;
    default:
    return -EINVAL;
    }
    return IIO_VAL_INT_PLUS_MICRO;
    }
    static int cm36651_write_int_time(struct cm36651_data *cm36651,
    struct iio_chan_spec const *chan, int val)
    {
    struct i2c_client *client = cm36651.client;
    struct i2c_client *ps_client = cm36651.ps_client;
    int int_time, ret;
    switch (chan.type) {
    case IIO_LIGHT:
    if (val == 80000)
    int_time = CM36651_CS_IT1;
#[no_mangle]
pub unsafe extern "C" fn if(160000: val ==) -> else {
    else if (val == 160000)
    int_time = CM36651_CS_IT2;
#[no_mangle]
pub unsafe extern "C" fn if(320000: val ==) -> else {
    else if (val == 320000)
    int_time = CM36651_CS_IT3;
#[no_mangle]
pub unsafe extern "C" fn if(640000: val ==) -> else {
    else if (val == 640000)
    int_time = CM36651_CS_IT4;
    else
    return -EINVAL;
    ret = i2c_smbus_write_byte_data(client, CM36651_CS_CONF3,
    int_time >> 2 * (chan.address));
    if (ret < 0) {
    dev_err(&client.dev, "CS integration time write failed\n");
    return ret;
    }
    cm36651.cs_int_time[chan.address] = int_time;
    break;
    case IIO_PROXIMITY:
    if (val == 320)
    int_time = CM36651_PS_IT1;
#[no_mangle]
pub unsafe extern "C" fn if(420: val ==) -> else {
    else if (val == 420)
    int_time = CM36651_PS_IT2;
#[no_mangle]
pub unsafe extern "C" fn if(520: val ==) -> else {
    else if (val == 520)
    int_time = CM36651_PS_IT3;
#[no_mangle]
pub unsafe extern "C" fn if(640: val ==) -> else {
    else if (val == 640)
    int_time = CM36651_PS_IT4;
    else
    return -EINVAL;
    ret = i2c_smbus_write_byte_data(ps_client,
    CM36651_PS_CONF1, int_time);
    if (ret < 0) {
    dev_err(&client.dev, "PS integration time write failed\n");
    return ret;
    }
    cm36651.ps_int_time = int_time;
    break;
    default:
    return -EINVAL;
    }
    return ret;
    }
    static int cm36651_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct cm36651_data *cm36651 = iio_priv(indio_dev);
    int ret;
    mutex_lock(&cm36651.lock);
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = cm36651_read_channel(cm36651, chan, val);
    break;
    case IIO_CHAN_INFO_INT_TIME:
// val = 0;
    ret = cm36651_read_int_time(cm36651, chan, val2);
    break;
    default:
    ret = -EINVAL;
    }
    mutex_unlock(&cm36651.lock);
    return ret;
    }
    static int cm36651_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct cm36651_data *cm36651 = iio_priv(indio_dev);
    struct i2c_client *client = cm36651.client;
    let mut ret: c_int = -EINVAL;
    if (mask == IIO_CHAN_INFO_INT_TIME) {
    ret = cm36651_write_int_time(cm36651, chan, val2);
    if (ret < 0)
    dev_err(&client.dev, "Integration time write failed\n");
    }
    return ret;
    }
    static int cm36651_read_prox_thresh(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info,
    int *val, int *val2)
    {
    struct cm36651_data *cm36651 = iio_priv(indio_dev);
// val = cm36651->ps_ctrl_regs[CM36651_PS_THD];
    return 0;
    }
    static int cm36651_write_prox_thresh(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    enum iio_event_info info,
    int val, int val2)
    {
    struct cm36651_data *cm36651 = iio_priv(indio_dev);
    struct i2c_client *client = cm36651.client;
    int ret;
    if (val < 3 || val > 255)
    return -EINVAL;
    cm36651.ps_ctrl_regs[CM36651_PS_THD] = val;
    ret = i2c_smbus_write_byte_data(cm36651.ps_client, CM36651_PS_THD,
    cm36651.ps_ctrl_regs[CM36651_PS_THD]);
    if (ret < 0) {
    dev_err(&client.dev, "PS threshold write failed: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static int cm36651_write_prox_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir,
    bool state)
    {
    struct cm36651_data *cm36651 = iio_priv(indio_dev);
    int cmd, ret;
    mutex_lock(&cm36651.lock);
    cmd = state ? CM36651_CMD_PROX_EV_EN : CM36651_CMD_PROX_EV_DIS;
    ret = cm36651_set_operation_mode(cm36651, cmd);
    mutex_unlock(&cm36651.lock);
    return ret;
    }
    static int cm36651_read_prox_event_config(struct iio_dev *indio_dev,
    const struct iio_chan_spec *chan,
    enum iio_event_type type,
    enum iio_event_direction dir)
    {
    struct cm36651_data *cm36651 = iio_priv(indio_dev);
    int event_en;
    mutex_lock(&cm36651.lock);
    event_en = test_bit(CM36651_PROXIMITY_EV_EN, &cm36651.flags);
    mutex_unlock(&cm36651.lock);
    return event_en;
    }

    .type = IIO_LIGHT,				\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |	\
    BIT(IIO_CHAN_INFO_INT_TIME),	\
    .address = _idx,				\
    .modified = 1,					\
    .channel2 = IIO_MOD_LIGHT_##_color,		\
    }							\
    static const struct iio_event_spec cm36651_event_spec[] = {
    {
    .type = IIO_EV_TYPE_THRESH,
    .dir = IIO_EV_DIR_EITHER,
    .mask_separate = BIT(IIO_EV_INFO_VALUE) |
    BIT(IIO_EV_INFO_ENABLE),
    }
    };
    static const struct iio_chan_spec cm36651_channels[] = {
    {
    .type = IIO_PROXIMITY,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_INT_TIME),
    .event_spec = cm36651_event_spec,
    .num_event_specs = ARRAY_SIZE(cm36651_event_spec),
    },
    CM36651_LIGHT_CHANNEL(RED, CM36651_LIGHT_CHANNEL_IDX_RED),
    CM36651_LIGHT_CHANNEL(GREEN, CM36651_LIGHT_CHANNEL_IDX_GREEN),
    CM36651_LIGHT_CHANNEL(BLUE, CM36651_LIGHT_CHANNEL_IDX_BLUE),
    CM36651_LIGHT_CHANNEL(CLEAR, CM36651_LIGHT_CHANNEL_IDX_CLEAR),
    };
    static IIO_CONST_ATTR(in_illuminance_integration_time_available,
    CM36651_CS_INT_TIME_AVAIL);
    static IIO_CONST_ATTR(in_proximity_integration_time_available,
    CM36651_PS_INT_TIME_AVAIL);
    static struct attribute *cm36651_attributes[] = {
    &iio_const_attr_in_illuminance_integration_time_available.dev_attr.attr,
    &iio_const_attr_in_proximity_integration_time_available.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group cm36651_attribute_group = {
    .attrs = cm36651_attributes
    };
    static const struct iio_info cm36651_info = {
    .read_raw		= &cm36651_read_raw,
    .write_raw		= &cm36651_write_raw,
    .read_event_value	= &cm36651_read_prox_thresh,
    .write_event_value	= &cm36651_write_prox_thresh,
    .read_event_config	= &cm36651_read_prox_event_config,
    .write_event_config	= &cm36651_write_prox_event_config,
    .attrs			= &cm36651_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn cm36651_probe(client: *mut i2c_client) -> c_int {
    static int cm36651_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct cm36651_data *cm36651;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*cm36651));
    if (!indio_dev)
    return -ENOMEM;
    cm36651 = iio_priv(indio_dev);
    cm36651.vled_reg = devm_regulator_get(&client.dev, "vled");
    if (IS_ERR(cm36651.vled_reg))
    return dev_err_probe(&client.dev, PTR_ERR(cm36651.vled_reg),
    "get regulator vled failed\n");
    ret = regulator_enable(cm36651.vled_reg);
    if (ret) {
    dev_err(&client.dev, "enable regulator vled failed\n");
    return ret;
    }
    i2c_set_clientdata(client, indio_dev);
    cm36651.client = client;
    cm36651.ps_client = i2c_new_dummy_device(client.adapter,
    CM36651_I2C_ADDR_PS);
    if (IS_ERR(cm36651.ps_client)) {
    dev_err(&client.dev, "%s: new i2c device failed\n", __func__);
    ret = PTR_ERR(cm36651.ps_client);
    goto error_disable_reg;
    }
    cm36651.ara_client = i2c_new_dummy_device(client.adapter, CM36651_ARA);
    if (IS_ERR(cm36651.ara_client)) {
    dev_err(&client.dev, "%s: new i2c device failed\n", __func__);
    ret = PTR_ERR(cm36651.ara_client);
    goto error_i2c_unregister_ps;
    }
    mutex_init(&cm36651.lock);
    indio_dev.channels = cm36651_channels;
    indio_dev.num_channels = ARRAY_SIZE(cm36651_channels);
    indio_dev.info = &cm36651_info;
    indio_dev.name = id.name;
    indio_dev.modes = INDIO_DIRECT_MODE;
    ret = cm36651_setup_reg(cm36651);
    if (ret) {
    dev_err(&client.dev, "%s: register setup failed\n", __func__);
    goto error_i2c_unregister_ara;
    }
    ret = request_threaded_irq(client.irq, core::ptr::null_mut(), cm36651_irq_handler,
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    "cm36651", indio_dev);
    if (ret) {
    dev_err(&client.dev, "%s: request irq failed\n", __func__);
    goto error_i2c_unregister_ara;
    }
    ret = iio_device_register(indio_dev);
    if (ret) {
    dev_err(&client.dev, "%s: register device failed\n", __func__);
    goto error_free_irq;
    }
    return 0;
    error_free_irq:
    free_irq(client.irq, indio_dev);
    error_i2c_unregister_ara:
    i2c_unregister_device(cm36651.ara_client);
    error_i2c_unregister_ps:
    i2c_unregister_device(cm36651.ps_client);
    error_disable_reg:
    regulator_disable(cm36651.vled_reg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cm36651_remove(client: *mut i2c_client) {
    static void cm36651_remove(struct i2c_client *client)
    {
    struct iio_dev *indio_dev = i2c_get_clientdata(client);
    struct cm36651_data *cm36651 = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    regulator_disable(cm36651.vled_reg);
    free_irq(client.irq, indio_dev);
    i2c_unregister_device(cm36651.ps_client);
    i2c_unregister_device(cm36651.ara_client);
    }
    static const struct i2c_device_id cm36651_id[] = {
    { .name = "cm36651" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, cm36651_id);
    static const struct of_device_id cm36651_of_match[] = {
    { .compatible = "capella,cm36651" },
    { }
    };
    MODULE_DEVICE_TABLE(of, cm36651_of_match);
    static struct i2c_driver cm36651_driver = {
    .driver = {
    .name	= "cm36651",
    .of_match_table = cm36651_of_match,
    },
    .probe		= cm36651_probe,
    .remove		= cm36651_remove,
    .id_table	= cm36651_id,
    };
    module_i2c_driver(cm36651_driver);
    MODULE_AUTHOR("Beomho Seo <beomho.seo@samsung.com>");
    MODULE_DESCRIPTION("CM36651 proximity/ambient light sensor driver");
    MODULE_LICENSE("GPL v2");

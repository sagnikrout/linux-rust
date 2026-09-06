//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/iqs62x.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Azoteq IQS620A/621/622/624/625 Multi-Function Sensors
//
// Copyright (C) 2019 Jeff LaBundy <jeff@labundy.com>
//
// These devices rely on application-specific register settings and calibration
// data developed in and exported from a suite of GUIs offered by the vendor. A
// separate tool converts the GUIs' ASCII-based output into a standard firmware
// file parsed by the driver.
//
// Link to datasheets and GUIs: https://www.azoteq.com
//
// Link to conversion tool: https://github.com/jlabundy/iqs62x-h2bin.git
//

pub const IQS62X_PROD_NUM: c_uint = 0x00;
pub const IQS62X_SYS_FLAGS: c_uint = 0x10;
pub const IQS620_HALL_FLAGS: c_uint = 0x16;
pub const IQS621_HALL_FLAGS: c_uint = 0x19;

pub const IQS624_INTERVAL_NUM: c_uint = 0x18;
pub const IQS625_INTERVAL_NUM: c_uint = 0x12;
pub const IQS622_PROX_SETTINGS_4: c_uint = 0x48;
pub const IQS620_PROX_SETTINGS_4: c_uint = 0x50;

pub const IQS621_ALS_CAL_DIV_LUX: c_uint = 0x82;
pub const IQS621_ALS_CAL_DIV_IR: c_uint = 0x83;
pub const IQS620_TEMP_CAL_MULT: c_uint = 0xC2;
pub const IQS620_TEMP_CAL_DIV: c_uint = 0xC3;
pub const IQS620_TEMP_CAL_OFFS: c_uint = 0xC4;
pub const IQS62X_SYS_SETTINGS: c_uint = 0xD0;

pub const IQS62X_PWR_SETTINGS: c_uint = 0xD2;

pub const IQS62X_PWR_SETTINGS_PWR_MODE_NORM: c_int = 0;
pub const IQS62X_OTP_CMD: c_uint = 0xF0;
pub const IQS62X_OTP_CMD_FG3: c_uint = 0x13;
pub const IQS62X_OTP_DATA: c_uint = 0xF1;
pub const IQS62X_MAX_REG: c_uint = 0xFF;

pub const IQS62X_FW_REC_TYPE_INFO: c_int = 0;
pub const IQS62X_FW_REC_TYPE_PROD: c_int = 1;
pub const IQS62X_FW_REC_TYPE_HALL: c_int = 2;
pub const IQS62X_FW_REC_TYPE_MASK: c_int = 3;
pub const IQS62X_FW_REC_TYPE_DATA: c_int = 4;
pub const IQS62X_ATI_STARTUP_MS: c_int = 350;
pub const IQS62X_FILT_SETTLE_MS: c_int = 250;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs62x_fw_rec {
    pub type: u8,
    pub addr: u8,
    pub len: u8,
    pub data: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs62x_fw_blk {
    pub list: list_head,
    pub addr: u8,
    pub mask: u8,
    pub len: u8,
    pub __counted_by(len): u8 data[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs62x_info {
    pub prod_num: u8,
    pub sw_num: u8,
    pub hw_num: u8,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn iqs62x_dev_init(iqs62x: *mut iqs62x_core) -> c_int {
    static int iqs62x_dev_init(struct iqs62x_core *iqs62x)
    {
    pub fw_blk: *mut iqs62x_fw_blk,
    pub val: c_uint,
    pub ret: c_int,
    list_for_each_entry(fw_blk, &iqs62x.fw_blk_head, list) {
//
// In case ATI is in progress, wait for it to complete before
// lowering the core clock frequency.
//
    if (fw_blk.addr == IQS62X_SYS_SETTINGS &&
// fw_blk->data & IQS62X_SYS_SETTINGS_CLK_DIV)
    if (fw_blk.mask)
    ret = regmap_update_bits(iqs62x.regmap, fw_blk.addr,
    pub fw_blk->data): *mut fw_blk->mask,,
    else
    ret = regmap_raw_write(iqs62x.regmap, fw_blk.addr,
    pub fw_blk->len): fw_blk->data,,
    if (ret)
    pub ret: return,
    }
    switch (iqs62x.dev_desc.prod_num) {
    case IQS620_PROD_NUM:
    case IQS622_PROD_NUM:
    ret = regmap_read(iqs62x.regmap,
    pub &val): iqs62x->dev_desc->prox_settings,,
    if (ret)
    pub ret: return,
    if (val & IQS620_PROX_SETTINGS_4_SAR_EN)
    pub IQS62X_UI_SAR1: iqs62x->ui_sel =,
    case IQS621_PROD_NUM:
    ret = regmap_write(iqs62x.regmap, IQS620_GLBL_EVENT_MASK,
    IQS620_GLBL_EVENT_MASK_PMU |
    iqs62x.dev_desc.prox_mask |
    iqs62x.dev_desc.sar_mask |
    iqs62x.dev_desc.hall_mask |
    iqs62x.dev_desc.hyst_mask |
    iqs62x.dev_desc.temp_mask |
    iqs62x.dev_desc.als_mask |
    if (ret)
    pub ret: return,
    default:
    ret = regmap_write(iqs62x.regmap, IQS624_HALL_UI,
    IQS624_HALL_UI_WHL_EVENT |
    IQS624_HALL_UI_INT_EVENT |
    if (ret)
    pub ret: return,
//
// The IQS625 default interval divider is below the minimum
// permissible value, and the datasheet mandates that it is
// corrected during initialization (unless an updated value
// has already been provided by firmware).
//
// To protect against an unacceptably low user-entered value
// stored in the firmware, the same check is extended to the
// IQS624 as well.
//
    pub &val): ret = regmap_read(iqs62x->regmap, IQS624_INTERVAL_DIV,,
    if (ret)
    pub ret: return,
    if (val >= iqs62x.dev_desc.interval_div)
    ret = regmap_write(iqs62x.regmap, IQS624_INTERVAL_DIV,
    if (ret)
    pub ret: return,
    }
//
// Place the device in streaming mode at first so as not to miss the
// limited number of interrupts that would otherwise occur after ATI
// completes. The device is subsequently placed in event mode by the
// interrupt handler.
//
// In the meantime, mask interrupts during ATI to prevent the device
// from soliciting I2C traffic until the noise-sensitive ATI process
// is complete.
//
    ret = regmap_update_bits(iqs62x.regmap, IQS62X_SYS_SETTINGS,
    IQS62X_SYS_SETTINGS_ACK_RESET |
    IQS62X_SYS_SETTINGS_EVENT_MODE |
    IQS62X_SYS_SETTINGS_COMM_ATI |
    IQS62X_SYS_SETTINGS_REDO_ATI,
    IQS62X_SYS_SETTINGS_ACK_RESET |
    if (ret)
    pub ret: return,
//
// The following delay gives the device time to deassert its RDY output
// in case a communication window was open while the REDO_ATI field was
// written. This prevents an interrupt from being serviced prematurely.
//
    pub 5100): usleep_range(5000,,
    pub 0: return,
    }
    static int iqs62x_firmware_parse(struct iqs62x_core *iqs62x,
    const struct firmware *fw)
    {
    pub iqs62x->client: *mut *mut i2c_client client =,
    pub fw_rec: *mut iqs62x_fw_rec,
    pub fw_blk: *mut iqs62x_fw_blk,
    pub val: c_uint,
    pub 0: size_t pos =,
    pub 0: int ret =,
    pub data: *mut u8 mask, len,,
    pub 0: u8 hall_cal_index =,
    while (pos < fw.size) {
    if (pos + sizeof(*fw_rec) > fw.size) {
    pub -EINVAL: ret =,
    }
    pub pos): *mut *mut fw_rec = (struct iqs62x_fw_rec )(fw->data +,
    pub sizeof(*fw_rec): *mut pos +=,
    if (!fw_rec.len || fw_rec.len - 1 > fw.size - pos) {
    pub -EINVAL: ret =,
    }
    pub 1: pos += fw_rec->len -,
    switch (fw_rec.type) {
    case IQS62X_FW_REC_TYPE_INFO:
    case IQS62X_FW_REC_TYPE_PROD:
    if (fw_rec.data == iqs62x.dev_desc.prod_num)
    dev_err(&client.dev,
    "Incompatible product number: 0x%02X\n",
    pub -EINVAL: ret =,
    case IQS62X_FW_REC_TYPE_HALL:
    if (!hall_cal_index) {
    ret = regmap_write(iqs62x.regmap,
    IQS62X_OTP_CMD,
    if (ret)
    ret = regmap_read(iqs62x.regmap,
    pub &val): IQS62X_OTP_DATA,,
    if (ret)
    pub IQS62X_HALL_CAL_MASK: hall_cal_index = val &,
    if (!hall_cal_index) {
    dev_err(&client.dev,
    pub device\n"): "Uncalibrated,
    pub -ENODATA: ret =,
    }
    }
    if (hall_cal_index > fw_rec.len) {
    pub -EINVAL: ret =,
    }
    pub 0: mask =,
    pub 1: data = &fw_rec->data + hall_cal_index -,
    pub sizeof(*data): *mut len =,
    case IQS62X_FW_REC_TYPE_MASK:
    if (fw_rec.len < (sizeof(mask) + sizeof(*data))) {
    pub -EINVAL: ret =,
    }
    pub fw_rec->data: mask =,
    pub sizeof(mask): data = &fw_rec->data +,
    pub sizeof(*data): *mut len =,
    case IQS62X_FW_REC_TYPE_DATA:
    pub 0: mask =,
    pub &fw_rec->data: data =,
    pub fw_rec->len: len =,
    default:
    dev_err(&client.dev,
    "Unrecognized record type: 0x%02X\n",
    pub -EINVAL: ret =,
    }
    if (ret)
    fw_blk = devm_kzalloc(&client.dev,
    struct_size(fw_blk, data, len),
    if (!fw_blk) {
    pub -ENOMEM: ret =,
    }
    pub fw_rec->addr: fw_blk->addr =,
    pub mask: fw_blk->mask =,
    pub len: fw_blk->len =,
    pub len): memcpy(fw_blk->data, data,,
    pub &iqs62x->fw_blk_head): list_add(&fw_blk->list,,
    }
    pub ret: return,
    }
    const struct iqs62x_event_desc iqs62x_events[IQS62X_NUM_EVENTS] = {
    [IQS62X_EVENT_PROX_CH0_T] = {
    .reg	= IQS62X_EVENT_PROX,
    .mask	= BIT(4),
    .val	= BIT(4),
    },
    [IQS62X_EVENT_PROX_CH0_P] = {
    .reg	= IQS62X_EVENT_PROX,
    .mask	= BIT(0),
    .val	= BIT(0),
    },
    [IQS62X_EVENT_PROX_CH1_T] = {
    .reg	= IQS62X_EVENT_PROX,
    .mask	= BIT(5),
    .val	= BIT(5),
    },
    [IQS62X_EVENT_PROX_CH1_P] = {
    .reg	= IQS62X_EVENT_PROX,
    .mask	= BIT(1),
    .val	= BIT(1),
    },
    [IQS62X_EVENT_PROX_CH2_T] = {
    .reg	= IQS62X_EVENT_PROX,
    .mask	= BIT(6),
    .val	= BIT(6),
    },
    [IQS62X_EVENT_PROX_CH2_P] = {
    .reg	= IQS62X_EVENT_PROX,
    .mask	= BIT(2),
    .val	= BIT(2),
    },
    [IQS62X_EVENT_HYST_POS_T] = {
    .reg	= IQS62X_EVENT_HYST,
    .mask	= BIT(6) | BIT(7),
    .val	= BIT(6),
    },
    [IQS62X_EVENT_HYST_POS_P] = {
    .reg	= IQS62X_EVENT_HYST,
    .mask	= BIT(5) | BIT(7),
    .val	= BIT(5),
    },
    [IQS62X_EVENT_HYST_NEG_T] = {
    .reg	= IQS62X_EVENT_HYST,
    .mask	= BIT(6) | BIT(7),
    .val	= BIT(6) | BIT(7),
    },
    [IQS62X_EVENT_HYST_NEG_P] = {
    .reg	= IQS62X_EVENT_HYST,
    .mask	= BIT(5) | BIT(7),
    .val	= BIT(5) | BIT(7),
    },
    [IQS62X_EVENT_SAR1_ACT] = {
    .reg	= IQS62X_EVENT_HYST,
    .mask	= BIT(4),
    .val	= BIT(4),
    },
    [IQS62X_EVENT_SAR1_QRD] = {
    .reg	= IQS62X_EVENT_HYST,
    .mask	= BIT(2),
    .val	= BIT(2),
    },
    [IQS62X_EVENT_SAR1_MOVE] = {
    .reg	= IQS62X_EVENT_HYST,
    .mask	= BIT(1),
    .val	= BIT(1),
    },
    [IQS62X_EVENT_SAR1_HALT] = {
    .reg	= IQS62X_EVENT_HYST,
    .mask	= BIT(0),
    .val	= BIT(0),
    },
    [IQS62X_EVENT_WHEEL_UP] = {
    .reg	= IQS62X_EVENT_WHEEL,
    .mask	= BIT(7) | BIT(6),
    .val	= BIT(7),
    },
    [IQS62X_EVENT_WHEEL_DN] = {
    .reg	= IQS62X_EVENT_WHEEL,
    .mask	= BIT(7) | BIT(6),
    .val	= BIT(7) | BIT(6),
    },
    [IQS62X_EVENT_HALL_N_T] = {
    .reg	= IQS62X_EVENT_HALL,
    .mask	= BIT(2) | BIT(0),
    .val	= BIT(2),
    },
    [IQS62X_EVENT_HALL_N_P] = {
    .reg	= IQS62X_EVENT_HALL,
    .mask	= BIT(1) | BIT(0),
    .val	= BIT(1),
    },
    [IQS62X_EVENT_HALL_S_T] = {
    .reg	= IQS62X_EVENT_HALL,
    .mask	= BIT(2) | BIT(0),
    .val	= BIT(2) | BIT(0),
    },
    [IQS62X_EVENT_HALL_S_P] = {
    .reg	= IQS62X_EVENT_HALL,
    .mask	= BIT(1) | BIT(0),
    .val	= BIT(1) | BIT(0),
    },
    [IQS62X_EVENT_SYS_RESET] = {
    .reg	= IQS62X_EVENT_SYS,
    .mask	= BIT(7),
    .val	= BIT(7),
    },
    [IQS62X_EVENT_SYS_ATI] = {
    .reg	= IQS62X_EVENT_SYS,
    .mask	= BIT(2),
    .val	= BIT(2),
    },
}

    EXPORT_SYMBOL_GPL(iqs62x_events);
#[no_mangle]
unsafe extern "C" fn iqs62x_irq(irq: c_int, context: *mut c_void) -> irqreturn_t {
    static irqreturn_t iqs62x_irq(int irq, void *context)
    {
    struct iqs62x_core *iqs62x = context;
    struct i2c_client *client = iqs62x.client;
    struct iqs62x_event_data event_data;
    struct iqs62x_event_desc event_desc;
    enum iqs62x_event_reg event_reg;
    let mut event_flags: c_ulong = 0;
    int ret, i, j;
    u8 event_map[IQS62X_EVENT_SIZE];
//
// The device asserts the RDY output to signal the beginning of a
// communication window, which is closed by an I2C stop condition.
// As such, all interrupt status is captured in a single read and
// broadcast to any interested sub-device drivers.
//
    ret = regmap_raw_read(iqs62x.regmap, IQS62X_SYS_FLAGS, event_map,
    sizeof(event_map));
    if (ret) {
    dev_err(&client.dev, "Failed to read device status: %d\n",
    ret);
    return IRQ_NONE;
    }
    for (i = 0; i < sizeof(event_map); i++) {
    event_reg = iqs62x.dev_desc.event_regs[iqs62x.ui_sel][i];
    switch (event_reg) {
    case IQS62X_EVENT_UI_LO:
    event_data.ui_data = get_unaligned_le16(&event_map[i]);
    fallthrough;
    case IQS62X_EVENT_UI_HI:
    case IQS62X_EVENT_NONE:
    continue;
    case IQS62X_EVENT_ALS:
    event_data.als_flags = event_map[i];
    continue;
    case IQS62X_EVENT_IR:
    event_data.ir_flags = event_map[i];
    continue;
    case IQS62X_EVENT_INTER:
    event_data.interval = event_map[i];
    continue;
    case IQS62X_EVENT_HYST:
    event_map[i] <<= iqs62x.dev_desc.hyst_shift;
    fallthrough;
    case IQS62X_EVENT_WHEEL:
    case IQS62X_EVENT_HALL:
    case IQS62X_EVENT_PROX:
    case IQS62X_EVENT_SYS:
    break;
    }
    for (j = 0; j < IQS62X_NUM_EVENTS; j++) {
    event_desc = iqs62x_events[j];
    if (event_desc.reg != event_reg)
    continue;
    if ((event_map[i] & event_desc.mask) == event_desc.val)
    event_flags |= BIT(j);
    }
    }
//
// The device resets itself in response to the I2C master stalling
// communication past a fixed timeout. In this case, all registers
// are restored and any interested sub-device drivers are notified.
//
    if (event_flags & BIT(IQS62X_EVENT_SYS_RESET)) {
    dev_err(&client.dev, "Unexpected device reset\n");
    ret = iqs62x_dev_init(iqs62x);
    if (ret) {
    dev_err(&client.dev,
    "Failed to re-initialize device: %d\n", ret);
    return IRQ_NONE;
    }
    iqs62x.event_cache |= BIT(IQS62X_EVENT_SYS_RESET);
    reinit_completion(&iqs62x.ati_done);
    } else if (event_flags & BIT(IQS62X_EVENT_SYS_ATI)) {
    iqs62x.event_cache |= BIT(IQS62X_EVENT_SYS_ATI);
    reinit_completion(&iqs62x.ati_done);
    } else if (!completion_done(&iqs62x.ati_done)) {
    ret = regmap_update_bits(iqs62x.regmap, IQS62X_SYS_SETTINGS,
    IQS62X_SYS_SETTINGS_EVENT_MODE, 0xFF);
    if (ret) {
    dev_err(&client.dev,
    "Failed to enable event mode: %d\n", ret);
    return IRQ_NONE;
    }
    msleep(IQS62X_FILT_SETTLE_MS);
    complete_all(&iqs62x.ati_done);
    }
//
// Reset and ATI events are not broadcast to the sub-device drivers
// until ATI has completed. Any other events that may have occurred
// during ATI are ignored.
//
    if (completion_done(&iqs62x.ati_done)) {
    event_flags |= iqs62x.event_cache;
    ret = blocking_notifier_call_chain(&iqs62x.nh, event_flags,
    &event_data);
    if (ret & NOTIFY_STOP_MASK)
    return IRQ_NONE;
    iqs62x.event_cache = 0;
    }
//
// Once the communication window is closed, a small delay is added to
// ensure the device's RDY output has been deasserted by the time the
// interrupt handler returns.
//
    usleep_range(150, 200);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn iqs62x_firmware_load(fw: *const firmware, context: *mut c_void) {
    static void iqs62x_firmware_load(const struct firmware *fw, void *context)
    {
    struct iqs62x_core *iqs62x = context;
    struct i2c_client *client = iqs62x.client;
    int ret;
    if (fw) {
    ret = iqs62x_firmware_parse(iqs62x, fw);
    if (ret) {
    dev_err(&client.dev, "Failed to parse firmware: %d\n",
    ret);
    goto err_out;
    }
    }
    ret = iqs62x_dev_init(iqs62x);
    if (ret) {
    dev_err(&client.dev, "Failed to initialize device: %d\n", ret);
    goto err_out;
    }
    ret = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), iqs62x_irq, IRQF_ONESHOT,
    client.name, iqs62x);
    if (ret) {
    dev_err(&client.dev, "Failed to request IRQ: %d\n", ret);
    goto err_out;
    }
    if (!wait_for_completion_timeout(&iqs62x.ati_done,
    msecs_to_jiffies(2000))) {
    dev_err(&client.dev, "Failed to complete ATI\n");
    goto err_out;
    }
    ret = devm_mfd_add_devices(&client.dev, PLATFORM_DEVID_NONE,
    iqs62x.dev_desc.sub_devs,
    iqs62x.dev_desc.num_sub_devs,
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret)
    dev_err(&client.dev, "Failed to add sub-devices: %d\n", ret);
    err_out:
    complete_all(&iqs62x.fw_done);
    }
    static const struct mfd_cell iqs620at_sub_devs[] = {
    {
    .name = "iqs62x-keys",
    .of_compatible = "azoteq,iqs620a-keys",
    },
    {
    .name = "iqs620a-pwm",
    .of_compatible = "azoteq,iqs620a-pwm",
    },
    { .name = "iqs620at-temp", },
    };
    static const struct mfd_cell iqs620a_sub_devs[] = {
    {
    .name = "iqs62x-keys",
    .of_compatible = "azoteq,iqs620a-keys",
    },
    {
    .name = "iqs620a-pwm",
    .of_compatible = "azoteq,iqs620a-pwm",
    },
    };
    static const struct mfd_cell iqs621_sub_devs[] = {
    {
    .name = "iqs62x-keys",
    .of_compatible = "azoteq,iqs621-keys",
    },
    { .name = "iqs621-als", },
    };
    static const struct mfd_cell iqs622_sub_devs[] = {
    {
    .name = "iqs62x-keys",
    .of_compatible = "azoteq,iqs622-keys",
    },
    { .name = "iqs621-als", },
    };
    static const struct mfd_cell iqs624_sub_devs[] = {
    {
    .name = "iqs62x-keys",
    .of_compatible = "azoteq,iqs624-keys",
    },
    { .name = "iqs624-pos", },
    };
    static const struct mfd_cell iqs625_sub_devs[] = {
    {
    .name = "iqs62x-keys",
    .of_compatible = "azoteq,iqs625-keys",
    },
    { .name = "iqs624-pos", },
    };
    static const u8 iqs620at_cal_regs[] = {
    IQS620_TEMP_CAL_MULT,
    IQS620_TEMP_CAL_DIV,
    IQS620_TEMP_CAL_OFFS,
    };
    static const u8 iqs621_cal_regs[] = {
    IQS621_ALS_CAL_DIV_LUX,
    IQS621_ALS_CAL_DIV_IR,
    };
    static const enum iqs62x_event_reg iqs620a_event_regs[][IQS62X_EVENT_SIZE] = {
    [IQS62X_UI_PROX] = {
    IQS62X_EVENT_SYS,	/* 0x10 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_PROX,	/* 0x12 */
    IQS62X_EVENT_HYST,	/* 0x13 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_HALL,	/* 0x16 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_NONE,
    },
    [IQS62X_UI_SAR1] = {
    IQS62X_EVENT_SYS,	/* 0x10 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_HYST,	/* 0x13 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_HALL,	/* 0x16 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_NONE,
    },
    };
    static const enum iqs62x_event_reg iqs621_event_regs[][IQS62X_EVENT_SIZE] = {
    [IQS62X_UI_PROX] = {
    IQS62X_EVENT_SYS,	/* 0x10 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_PROX,	/* 0x12 */
    IQS62X_EVENT_HYST,	/* 0x13 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_ALS,	/* 0x16 */
    IQS62X_EVENT_UI_LO,	/* 0x17 */
    IQS62X_EVENT_UI_HI,	/* 0x18 */
    IQS62X_EVENT_HALL,	/* 0x19 */
    },
    };
    static const enum iqs62x_event_reg iqs622_event_regs[][IQS62X_EVENT_SIZE] = {
    [IQS62X_UI_PROX] = {
    IQS62X_EVENT_SYS,	/* 0x10 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_PROX,	/* 0x12 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_ALS,	/* 0x14 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_IR,	/* 0x16 */
    IQS62X_EVENT_UI_LO,	/* 0x17 */
    IQS62X_EVENT_UI_HI,	/* 0x18 */
    IQS62X_EVENT_HALL,	/* 0x19 */
    },
    [IQS62X_UI_SAR1] = {
    IQS62X_EVENT_SYS,	/* 0x10 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_HYST,	/* 0x13 */
    IQS62X_EVENT_ALS,	/* 0x14 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_IR,	/* 0x16 */
    IQS62X_EVENT_UI_LO,	/* 0x17 */
    IQS62X_EVENT_UI_HI,	/* 0x18 */
    IQS62X_EVENT_HALL,	/* 0x19 */
    },
    };
    static const enum iqs62x_event_reg iqs624_event_regs[][IQS62X_EVENT_SIZE] = {
    [IQS62X_UI_PROX] = {
    IQS62X_EVENT_SYS,	/* 0x10 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_PROX,	/* 0x12 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_WHEEL,	/* 0x14 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_UI_LO,	/* 0x16 */
    IQS62X_EVENT_UI_HI,	/* 0x17 */
    IQS62X_EVENT_INTER,	/* 0x18 */
    IQS62X_EVENT_NONE,
    },
    };
    static const enum iqs62x_event_reg iqs625_event_regs[][IQS62X_EVENT_SIZE] = {
    [IQS62X_UI_PROX] = {
    IQS62X_EVENT_SYS,	/* 0x10 */
    IQS62X_EVENT_PROX,	/* 0x11 */
    IQS62X_EVENT_INTER,	/* 0x12 */
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_NONE,
    },
    };
    static const struct iqs62x_dev_desc iqs62x_devs[] = {
    {
    .dev_name	= "iqs620at",
    .sub_devs	= iqs620at_sub_devs,
    .num_sub_devs	= ARRAY_SIZE(iqs620at_sub_devs),
    .prod_num	= IQS620_PROD_NUM,
    .sw_num		= 0x08,
    .cal_regs	= iqs620at_cal_regs,
    .num_cal_regs	= ARRAY_SIZE(iqs620at_cal_regs),
    .prox_mask	= BIT(0),
    .sar_mask	= BIT(1) | BIT(7),
    .hall_mask	= BIT(2),
    .hyst_mask	= BIT(3),
    .temp_mask	= BIT(4),
    .prox_settings	= IQS620_PROX_SETTINGS_4,
    .hall_flags	= IQS620_HALL_FLAGS,
    .fw_name	= "iqs620a.bin",
    .event_regs	= &iqs620a_event_regs[IQS62X_UI_PROX],
    },
    {
    .dev_name	= "iqs620a",
    .sub_devs	= iqs620a_sub_devs,
    .num_sub_devs	= ARRAY_SIZE(iqs620a_sub_devs),
    .prod_num	= IQS620_PROD_NUM,
    .sw_num		= 0x08,
    .prox_mask	= BIT(0),
    .sar_mask	= BIT(1) | BIT(7),
    .hall_mask	= BIT(2),
    .hyst_mask	= BIT(3),
    .temp_mask	= BIT(4),
    .prox_settings	= IQS620_PROX_SETTINGS_4,
    .hall_flags	= IQS620_HALL_FLAGS,
    .fw_name	= "iqs620a.bin",
    .event_regs	= &iqs620a_event_regs[IQS62X_UI_PROX],
    },
    {
    .dev_name	= "iqs621",
    .sub_devs	= iqs621_sub_devs,
    .num_sub_devs	= ARRAY_SIZE(iqs621_sub_devs),
    .prod_num	= IQS621_PROD_NUM,
    .sw_num		= 0x09,
    .cal_regs	= iqs621_cal_regs,
    .num_cal_regs	= ARRAY_SIZE(iqs621_cal_regs),
    .prox_mask	= BIT(0),
    .hall_mask	= BIT(1),
    .als_mask	= BIT(2),
    .hyst_mask	= BIT(3),
    .temp_mask	= BIT(4),
    .als_flags	= IQS621_ALS_FLAGS,
    .hall_flags	= IQS621_HALL_FLAGS,
    .hyst_shift	= 5,
    .fw_name	= "iqs621.bin",
    .event_regs	= &iqs621_event_regs[IQS62X_UI_PROX],
    },
    {
    .dev_name	= "iqs622",
    .sub_devs	= iqs622_sub_devs,
    .num_sub_devs	= ARRAY_SIZE(iqs622_sub_devs),
    .prod_num	= IQS622_PROD_NUM,
    .sw_num		= 0x06,
    .prox_mask	= BIT(0),
    .sar_mask	= BIT(1),
    .hall_mask	= BIT(2),
    .als_mask	= BIT(3),
    .ir_mask	= BIT(4),
    .prox_settings	= IQS622_PROX_SETTINGS_4,
    .als_flags	= IQS622_ALS_FLAGS,
    .hall_flags	= IQS622_HALL_FLAGS,
    .fw_name	= "iqs622.bin",
    .event_regs	= &iqs622_event_regs[IQS62X_UI_PROX],
    },
    {
    .dev_name	= "iqs624",
    .sub_devs	= iqs624_sub_devs,
    .num_sub_devs	= ARRAY_SIZE(iqs624_sub_devs),
    .prod_num	= IQS624_PROD_NUM,
    .sw_num		= 0x0B,
    .interval	= IQS624_INTERVAL_NUM,
    .interval_div	= 3,
    .fw_name	= "iqs624.bin",
    .event_regs	= &iqs624_event_regs[IQS62X_UI_PROX],
    },
    {
    .dev_name	= "iqs625",
    .sub_devs	= iqs625_sub_devs,
    .num_sub_devs	= ARRAY_SIZE(iqs625_sub_devs),
    .prod_num	= IQS625_PROD_NUM,
    .sw_num		= 0x0B,
    .interval	= IQS625_INTERVAL_NUM,
    .interval_div	= 10,
    .fw_name	= "iqs625.bin",
    .event_regs	= &iqs625_event_regs[IQS62X_UI_PROX],
    },
    };
    static const struct regmap_config iqs62x_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = IQS62X_MAX_REG,
    };
#[no_mangle]
unsafe extern "C" fn iqs62x_probe(client: *mut i2c_client) -> c_int {
    static int iqs62x_probe(struct i2c_client *client)
    {
    struct iqs62x_core *iqs62x;
    struct iqs62x_info info;
    unsigned int val;
    int ret, i, j;
    const char *fw_name = core::ptr::null_mut();
    iqs62x = devm_kzalloc(&client.dev, sizeof(*iqs62x), GFP_KERNEL);
    if (!iqs62x)
    return -ENOMEM;
    i2c_set_clientdata(client, iqs62x);
    iqs62x.client = client;
    BLOCKING_INIT_NOTIFIER_HEAD(&iqs62x.nh);
    INIT_LIST_HEAD(&iqs62x.fw_blk_head);
    init_completion(&iqs62x.ati_done);
    init_completion(&iqs62x.fw_done);
    iqs62x.regmap = devm_regmap_init_i2c(client, &iqs62x_regmap_config);
    if (IS_ERR(iqs62x.regmap)) {
    ret = PTR_ERR(iqs62x.regmap);
    dev_err(&client.dev, "Failed to initialize register map: %d\n",
    ret);
    return ret;
    }
    ret = regmap_raw_read(iqs62x.regmap, IQS62X_PROD_NUM, &info,
    sizeof(info));
    if (ret)
    return ret;
//
// The following sequence validates the device's product and software
// numbers. It then determines if the device is factory-calibrated by
// checking for nonzero values in the device's designated calibration
// registers (if applicable). Depending on the device, the absence of
// calibration data indicates a reduced feature set or invalid device.
//
// For devices given in both calibrated and uncalibrated versions, the
// calibrated version (e.g. IQS620AT) appears first in the iqs62x_devs
// array. The uncalibrated version (e.g. IQS620A) appears next and has
// the same product and software numbers, but no calibration registers
// are specified.
//
    for (i = 0; i < ARRAY_SIZE(iqs62x_devs); i++) {
    if (info.prod_num != iqs62x_devs[i].prod_num)
    continue;
    iqs62x.dev_desc = &iqs62x_devs[i];
    if (info.sw_num < iqs62x.dev_desc.sw_num)
    continue;
    iqs62x.sw_num = info.sw_num;
    iqs62x.hw_num = info.hw_num;
//
// Read each of the device's designated calibration registers,
// if any, and exit from the inner loop early if any are equal
// to zero (indicating the device is uncalibrated). This could
// be acceptable depending on the device (e.g. IQS620A instead
// of IQS620AT).
//
    for (j = 0; j < iqs62x.dev_desc.num_cal_regs; j++) {
    ret = regmap_read(iqs62x.regmap,
    iqs62x.dev_desc.cal_regs[j], &val);
    if (ret)
    return ret;
    if (!val)
    break;
    }
//
// If the number of nonzero values read from the device equals
// the number of designated calibration registers (which could
// be zero), exit from the outer loop early to signal that the
// device's product and software numbers match a known device,
// and the device is calibrated (if applicable).
//
    if (j == iqs62x.dev_desc.num_cal_regs)
    break;
    }
    if (!iqs62x.dev_desc) {
    dev_err(&client.dev, "Unrecognized product number: 0x%02X\n",
    info.prod_num);
    return -EINVAL;
    }
    if (!iqs62x.sw_num) {
    dev_err(&client.dev, "Unrecognized software number: 0x%02X\n",
    info.sw_num);
    return -EINVAL;
    }
    if (i == ARRAY_SIZE(iqs62x_devs)) {
    dev_err(&client.dev, "Uncalibrated device\n");
    return -ENODATA;
    }
    device_property_read_string(&client.dev, "firmware-name", &fw_name);
    ret = request_firmware_nowait(THIS_MODULE, FW_ACTION_UEVENT,
    fw_name ? : iqs62x.dev_desc.fw_name,
    &client.dev, GFP_KERNEL, iqs62x,
    iqs62x_firmware_load);
    if (ret)
    dev_err(&client.dev, "Failed to request firmware: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iqs62x_remove(client: *mut i2c_client) {
    static void iqs62x_remove(struct i2c_client *client)
    {
    struct iqs62x_core *iqs62x = i2c_get_clientdata(client);
    wait_for_completion(&iqs62x.fw_done);
    }
#[no_mangle]
unsafe extern "C" fn iqs62x_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused iqs62x_suspend(struct device *dev)
    {
    struct iqs62x_core *iqs62x = dev_get_drvdata(dev);
    int ret;
    wait_for_completion(&iqs62x.fw_done);
//
// As per the datasheet, automatic mode switching must be disabled
// before the device is placed in or taken out of halt mode.
//
    ret = regmap_update_bits(iqs62x.regmap, IQS62X_PWR_SETTINGS,
    IQS62X_PWR_SETTINGS_DIS_AUTO, 0xFF);
    if (ret)
    return ret;
    return regmap_update_bits(iqs62x.regmap, IQS62X_PWR_SETTINGS,
    IQS62X_PWR_SETTINGS_PWR_MODE_MASK,
    IQS62X_PWR_SETTINGS_PWR_MODE_HALT);
    }
#[no_mangle]
unsafe extern "C" fn iqs62x_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused iqs62x_resume(struct device *dev)
    {
    struct iqs62x_core *iqs62x = dev_get_drvdata(dev);
    int ret;
    ret = regmap_update_bits(iqs62x.regmap, IQS62X_PWR_SETTINGS,
    IQS62X_PWR_SETTINGS_PWR_MODE_MASK,
    IQS62X_PWR_SETTINGS_PWR_MODE_NORM);
    if (ret)
    return ret;
    return regmap_update_bits(iqs62x.regmap, IQS62X_PWR_SETTINGS,
    IQS62X_PWR_SETTINGS_DIS_AUTO, 0);
    }
    static SIMPLE_DEV_PM_OPS(iqs62x_pm, iqs62x_suspend, iqs62x_resume);
    static const struct of_device_id iqs62x_of_match[] = {
    { .compatible = "azoteq,iqs620a" },
    { .compatible = "azoteq,iqs621" },
    { .compatible = "azoteq,iqs622" },
    { .compatible = "azoteq,iqs624" },
    { .compatible = "azoteq,iqs625" },
    { }
    };
    MODULE_DEVICE_TABLE(of, iqs62x_of_match);
    static struct i2c_driver iqs62x_i2c_driver = {
    .driver = {
    .name = "iqs62x",
    .of_match_table = iqs62x_of_match,
    .pm = &iqs62x_pm,
    },
    .probe = iqs62x_probe,
    .remove = iqs62x_remove,
    };
    module_i2c_driver(iqs62x_i2c_driver);
    MODULE_AUTHOR("Jeff LaBundy <jeff@labundy.com>");
    MODULE_DESCRIPTION("Azoteq IQS620A/621/622/624/625 Multi-Function Sensors");
    MODULE_LICENSE("GPL");

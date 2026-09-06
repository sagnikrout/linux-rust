//! Automatically rewritten from C to Rust
//! Source: drivers/extcon/extcon-lc824206xa.c
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
// ON Semiconductor LC824206XA Micro USB Switch driver
//
// Copyright (c) 2024 Hans de Goede <hansg@kernel.org>
//
// ON Semiconductor has an "Advance Information" datasheet available
// (ENA2222-D.PDF), but no full datasheet. So there is no documentation
// available for the registers.
//
// This driver is based on the register info from the extcon-fsa9285.c driver,
// from the Lollipop Android sources for the Lenovo Yoga Tablet 2 (Pro)
// 830 / 1050 / 1380 models. Note despite the name this is actually a driver
// for the LC824206XA not the FSA9285. The Android sources can be downloaded
// from Lenovo's support page for these tablets, filename:
// yoga_tab_2_osc_android_to_lollipop_201505.rar.
//

//
// Register defines as mentioned above there is no datasheet with register
// info, so this may not be 100% accurate.
//
pub const REG00: c_uint = 0x00;
pub const REG00_INIT_VALUE: c_uint = 0x01;
pub const REG_STATUS: c_uint = 0x01;

pub const STATUS_USB_ID_GND: c_uint = 0x80;
pub const STATUS_USB_ID_ACA: c_uint = 0xf0;
pub const STATUS_USB_ID_FLOAT: c_uint = 0xf8;
//
// This controls the DP/DM muxes + other switches,
// meaning of individual bits is unknown.
//
pub const REG_SWITCH_CONTROL: c_uint = 0x02;
pub const SWITCH_STEREO_MIC: c_uint = 0xc8;
pub const SWITCH_USB_HOST: c_uint = 0xec;
pub const SWITCH_DISCONNECTED: c_uint = 0xf8;
pub const SWITCH_USB_DEVICE: c_uint = 0xfc;
// 5 bits? ADC 0x10 GND, 0x1a-0x1f ACA, 0x1f float
pub const REG_ID_PIN_ADC_VALUE: c_uint = 0x03;
// Masks for all 3 interrupt registers

// Both of these get set after a continuous mode ADC conversion

// Charger type available in reg 0x09

// There are 7 interrupt sources, bit 6 use is unknown (OCP?)

// Unmask interrupts this driver cares about

    (INTR_ALL & ~(INTR_ID_PIN_CHANGE | INTR_VBUS_CHANGE | INTR_CHARGER_DET_DONE))
// Active (event happened and not cleared yet) interrupts
pub const REG_INTR_STATUS: c_uint = 0x04;
//
// Writing a 1 to a bit here clears it in INTR_STATUS. These bits do NOT
// auto-reset to 0, so these must be set to 0 manually after clearing.
//
pub const REG_INTR_CLEAR: c_uint = 0x05;
// Interrupts which bit is set to 1 here will not raise the HW IRQ
pub const REG_INTR_MASK: c_uint = 0x06;
// ID pin ADC control, meaning of individual bits is unknown
pub const REG_ID_PIN_ADC_CTRL: c_uint = 0x07;
pub const ID_PIN_ADC_AUTO: c_uint = 0x40;
pub const ID_PIN_ADC_CONTINUOUS: c_uint = 0x44;
pub const REG_CHARGER_DET: c_uint = 0x08;

pub const REG_CHARGER_TYPE: c_uint = 0x09;
pub const CHARGER_TYPE_UNKNOWN: c_uint = 0x00;
pub const CHARGER_TYPE_DCP: c_uint = 0x01;
pub const CHARGER_TYPE_SDP_OR_CDP: c_uint = 0x04;
pub const CHARGER_TYPE_QC: c_uint = 0x06;
pub const REG10: c_uint = 0x10;
pub const REG10_INIT_VALUE: c_uint = 0x00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lc824206xa_data {
    pub work: work_struct,
    pub client: *mut i2c_client,
    pub edev: *mut extcon_dev,
    pub psy: *mut power_supply,
    pub vbus_boost: *mut regulator,
    pub usb_type: c_uint,
    pub cable: c_uint,
    pub previous_cable: c_uint,
    pub switch_control: u8,
    pub previous_switch_control: u8,
    pub vbus_ok: bool,
    pub vbus_boost_enabled: bool,
    pub fastcharge_over_miclr: bool,
}

    static const unsigned int lc824206xa_cables[] = {
    EXTCON_USB_HOST,
    EXTCON_CHG_USB_SDP,
    EXTCON_CHG_USB_CDP,
    EXTCON_CHG_USB_DCP,
    EXTCON_CHG_USB_ACA,
    EXTCON_CHG_USB_FAST,
    EXTCON_NONE,
    };
// read/write reg helpers to add error logging to smbus byte functions
#[no_mangle]
unsafe extern "C" fn lc824206xa_read_reg(data: *mut lc824206xa_data, reg: u8) -> c_int {
    static int lc824206xa_read_reg(struct lc824206xa_data *data, u8 reg)
    {
    int ret;
    ret = i2c_smbus_read_byte_data(data.client, reg);
    if (ret < 0)
    dev_err(&data.client.dev, "Error %d reading reg 0x%02x\n", ret, reg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lc824206xa_write_reg(data: *mut lc824206xa_data, reg: u8, val: u8) -> c_int {
    static int lc824206xa_write_reg(struct lc824206xa_data *data, u8 reg, u8 val)
    {
    int ret;
    ret = i2c_smbus_write_byte_data(data.client, reg, val);
    if (ret < 0)
    dev_err(&data.client.dev, "Error %d writing reg 0x%02x\n", ret, reg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lc824206xa_get_id(data: *mut lc824206xa_data) -> c_int {
    static int lc824206xa_get_id(struct lc824206xa_data *data)
    {
    int ret;
    ret = lc824206xa_write_reg(data, REG_ID_PIN_ADC_CTRL, ID_PIN_ADC_CONTINUOUS);
    if (ret)
    return ret;
    ret = lc824206xa_read_reg(data, REG_ID_PIN_ADC_VALUE);
    lc824206xa_write_reg(data, REG_ID_PIN_ADC_CTRL, ID_PIN_ADC_AUTO);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lc824206xa_set_vbus_boost(data: *mut lc824206xa_data, enable: bool) {
    static void lc824206xa_set_vbus_boost(struct lc824206xa_data *data, bool enable)
    {
    int ret;
    if (data.vbus_boost_enabled == enable)
    return;
    if (enable)
    ret = regulator_enable(data.vbus_boost);
    else
    ret = regulator_disable(data.vbus_boost);
    if (ret == 0)
    data.vbus_boost_enabled = enable;
    else
    dev_err(&data.client.dev, "Error updating Vbus boost regulator: %d\n", ret);
    }
#[no_mangle]
unsafe extern "C" fn lc824206xa_charger_detect(data: *mut lc824206xa_data) {
    static void lc824206xa_charger_detect(struct lc824206xa_data *data)
    {
    int charger_type, ret;
    charger_type = lc824206xa_read_reg(data, REG_CHARGER_TYPE);
    if (charger_type < 0)
    return;
    dev_dbg(&data.client.dev, "charger type 0x%02x\n", charger_type);
    switch (charger_type) {
    case CHARGER_TYPE_UNKNOWN:
    data.usb_type = POWER_SUPPLY_USB_TYPE_UNKNOWN;
// Treat as SDP
    data.cable = EXTCON_CHG_USB_SDP;
    data.switch_control = SWITCH_USB_DEVICE;
    break;
    case CHARGER_TYPE_SDP_OR_CDP:
    data.usb_type = POWER_SUPPLY_USB_TYPE_SDP;
    data.cable = EXTCON_CHG_USB_SDP;
    data.switch_control = SWITCH_USB_DEVICE;
    ret = lc824206xa_write_reg(data, REG_CHARGER_DET,
    CHARGER_DET_CDP_ON | CHARGER_DET_ON);
    if (ret < 0)
    break;
    msleep(100);
    ret = lc824206xa_read_reg(data, REG_CHARGER_DET);
    if (ret >= 0 && (ret & CHARGER_DET_CDP_VAL)) {
    data.usb_type = POWER_SUPPLY_USB_TYPE_CDP;
    data.cable = EXTCON_CHG_USB_CDP;
    }
    lc824206xa_write_reg(data, REG_CHARGER_DET, CHARGER_DET_ON);
    break;
    case CHARGER_TYPE_DCP:
    data.usb_type = POWER_SUPPLY_USB_TYPE_DCP;
    data.cable = EXTCON_CHG_USB_DCP;
    if (data.fastcharge_over_miclr)
    data.switch_control = SWITCH_STEREO_MIC;
    else
    data.switch_control = SWITCH_DISCONNECTED;
    break;
    case CHARGER_TYPE_QC:
    data.usb_type = POWER_SUPPLY_USB_TYPE_DCP;
    data.cable = EXTCON_CHG_USB_DCP;
    data.switch_control = SWITCH_DISCONNECTED;
    break;
    default:
    dev_warn(&data.client.dev, "Unknown charger type: 0x%02x\n", charger_type);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn lc824206xa_work(work: *mut work_struct) {
    static void lc824206xa_work(struct work_struct *work)
    {
    struct lc824206xa_data *data = container_of(work, struct lc824206xa_data, work);
    let mut vbus_boost_enable: bool = false;
    int status, id;
    status = lc824206xa_read_reg(data, REG_STATUS);
    if (status < 0)
    return;
    dev_dbg(&data.client.dev, "status 0x%02x\n", status);
    data.vbus_ok = (status & (STATUS_VBUS_PRESENT | STATUS_OVP)) == STATUS_VBUS_PRESENT;
// Read id pin ADC if necessary
    switch (status & STATUS_USB_ID) {
    case STATUS_USB_ID_GND:
    case STATUS_USB_ID_FLOAT:
    break;
    default:
// Happens when the connector is inserted slowly, log at dbg level
    dev_dbg(&data.client.dev, "Unknown status 0x%02x\n", status);
    fallthrough;
    case STATUS_USB_ID_ACA:
    id = lc824206xa_get_id(data);
    dev_dbg(&data.client.dev, "RID 0x%02x\n", id);
    switch (id) {
    case 0x10:
    status = STATUS_USB_ID_GND;
    break;
    case 0x18 ... 0x1e:
    status = STATUS_USB_ID_ACA;
    break;
    case 0x1f:
    status = STATUS_USB_ID_FLOAT;
    break;
    default:
    dev_warn(&data.client.dev, "Unknown RID 0x%02x\n", id);
    return;
    }
    }
// Check for out of spec OTG charging hubs, treat as ACA
    if ((status & STATUS_USB_ID) == STATUS_USB_ID_GND &&
    data.vbus_ok && !data.vbus_boost_enabled) {
    dev_info(&data.client.dev, "Out of spec USB host adapter with Vbus present, not enabling 5V output\n");
    status = STATUS_USB_ID_ACA;
    }
    switch (status & STATUS_USB_ID) {
    case STATUS_USB_ID_ACA:
    data.usb_type = POWER_SUPPLY_USB_TYPE_ACA;
    data.cable = EXTCON_CHG_USB_ACA;
    data.switch_control = SWITCH_USB_HOST;
    break;
    case STATUS_USB_ID_GND:
    data.usb_type = POWER_SUPPLY_USB_TYPE_UNKNOWN;
    data.cable = EXTCON_USB_HOST;
    data.switch_control = SWITCH_USB_HOST;
    vbus_boost_enable = true;
    break;
    case STATUS_USB_ID_FLOAT:
// When fast charging with Vbus > 5V, OVP will be set
    if (data.fastcharge_over_miclr &&
    data.switch_control == SWITCH_STEREO_MIC &&
    (status & STATUS_OVP)) {
    data.cable = EXTCON_CHG_USB_FAST;
    break;
    }
    if (data.vbus_ok) {
    lc824206xa_charger_detect(data);
    } else {
    data.usb_type = POWER_SUPPLY_USB_TYPE_UNKNOWN;
    data.cable = EXTCON_NONE;
    data.switch_control = SWITCH_DISCONNECTED;
    }
    break;
    }
    lc824206xa_set_vbus_boost(data, vbus_boost_enable);
    if (data.switch_control != data.previous_switch_control) {
    lc824206xa_write_reg(data, REG_SWITCH_CONTROL, data.switch_control);
    data.previous_switch_control = data.switch_control;
    }
    if (data.cable != data.previous_cable) {
    extcon_set_state_sync(data.edev, data.previous_cable, false);
    extcon_set_state_sync(data.edev, data.cable, true);
    data.previous_cable = data.cable;
    }
    power_supply_changed(data.psy);
    }
#[no_mangle]
unsafe extern "C" fn lc824206xa_irq(irq: c_int, _data: *mut c_void) -> irqreturn_t {
    static irqreturn_t lc824206xa_irq(int irq, void *_data)
    {
    struct lc824206xa_data *data = _data;
    int intr_status;
    intr_status = lc824206xa_read_reg(data, REG_INTR_STATUS);
    if (intr_status < 0)
    intr_status = INTR_ALL; /* Should never happen, clear all */
    dev_dbg(&data.client.dev, "interrupt 0x%02x\n", intr_status);
    lc824206xa_write_reg(data, REG_INTR_CLEAR, intr_status);
    lc824206xa_write_reg(data, REG_INTR_CLEAR, 0);
    schedule_work(&data.work);
    return IRQ_HANDLED;
    }
//
// Newer charger (power_supply) drivers expect the max input current to be
// provided by a parent power_supply device for the charger chip.
//
    static int lc824206xa_psy_get_prop(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct lc824206xa_data *data = power_supply_get_drvdata(psy);
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = data.vbus_ok && !data.vbus_boost_enabled;
    break;
    case POWER_SUPPLY_PROP_USB_TYPE:
    val.intval = data.usb_type;
    break;
    case POWER_SUPPLY_PROP_CURRENT_MAX:
    switch (data.usb_type) {
    case POWER_SUPPLY_USB_TYPE_DCP:
    case POWER_SUPPLY_USB_TYPE_ACA:
    val.intval = 2000000;
    break;
    case POWER_SUPPLY_USB_TYPE_CDP:
    val.intval = 1500000;
    break;
    default:
    val.intval = 500000;
    }
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const enum power_supply_property lc824206xa_psy_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_USB_TYPE,
    POWER_SUPPLY_PROP_CURRENT_MAX,
    };
    static const struct power_supply_desc lc824206xa_psy_desc = {
    .name = "lc824206xa-charger-detect",
    .type = POWER_SUPPLY_TYPE_USB,
    .usb_types = BIT(POWER_SUPPLY_USB_TYPE_SDP) |
    BIT(POWER_SUPPLY_USB_TYPE_CDP) |
    BIT(POWER_SUPPLY_USB_TYPE_DCP) |
    BIT(POWER_SUPPLY_USB_TYPE_ACA) |
    BIT(POWER_SUPPLY_USB_TYPE_UNKNOWN),
    .properties = lc824206xa_psy_props,
    .num_properties = ARRAY_SIZE(lc824206xa_psy_props),
    .get_property = lc824206xa_psy_get_prop,
    };
#[no_mangle]
unsafe extern "C" fn lc824206xa_probe(client: *mut i2c_client) -> c_int {
    static int lc824206xa_probe(struct i2c_client *client)
    {
    let mut psy_cfg: power_supply_config = { };
    struct device *dev = &client.dev;
    struct lc824206xa_data *data;
    int ret;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    INIT_WORK(&data.work, lc824206xa_work);
    data.cable = EXTCON_NONE;
    data.previous_cable = EXTCON_NONE;
    data.usb_type = POWER_SUPPLY_USB_TYPE_UNKNOWN;
// Some designs use a custom fast-charge protocol over the mic L/R inputs
    data.fastcharge_over_miclr =
    device_property_read_bool(dev, "onnn,enable-miclr-for-dcp");
    data.vbus_boost = devm_regulator_get(dev, "vbus");
    if (IS_ERR(data.vbus_boost))
    return dev_err_probe(dev, PTR_ERR(data.vbus_boost),
    "getting regulator\n");
// Init
    ret = lc824206xa_write_reg(data, REG00, REG00_INIT_VALUE);
    ret |= lc824206xa_write_reg(data, REG10, REG10_INIT_VALUE);
    msleep(100);
    ret |= lc824206xa_write_reg(data, REG_INTR_CLEAR, INTR_ALL);
    ret |= lc824206xa_write_reg(data, REG_INTR_CLEAR, 0);
    ret |= lc824206xa_write_reg(data, REG_INTR_MASK, INTR_MASK);
    ret |= lc824206xa_write_reg(data, REG_ID_PIN_ADC_CTRL, ID_PIN_ADC_AUTO);
    ret |= lc824206xa_write_reg(data, REG_CHARGER_DET, CHARGER_DET_ON);
    if (ret)
    return -EIO;
// Initialize extcon device
    data.edev = devm_extcon_dev_allocate(dev, lc824206xa_cables);
    if (IS_ERR(data.edev))
    return PTR_ERR(data.edev);
    ret = devm_extcon_dev_register(dev, data.edev);
    if (ret)
    return dev_err_probe(dev, ret, "registering extcon device\n");
    psy_cfg.drv_data = data;
    data.psy = devm_power_supply_register(dev, &lc824206xa_psy_desc, &psy_cfg);
    if (IS_ERR(data.psy))
    return dev_err_probe(dev, PTR_ERR(data.psy), "registering power supply\n");
    ret = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(), lc824206xa_irq,
    IRQF_TRIGGER_LOW | IRQF_ONESHOT,
    KBUILD_MODNAME, data);
    if (ret)
    return dev_err_probe(dev, ret, "requesting IRQ\n");
// Sync initial state
    schedule_work(&data.work);
    return 0;
    }
    static const struct i2c_device_id lc824206xa_i2c_ids[] = {
    { "lc824206xa" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lc824206xa_i2c_ids);
    static struct i2c_driver lc824206xa_driver = {
    .driver = {
    .name = KBUILD_MODNAME,
    },
    .probe = lc824206xa_probe,
    .id_table = lc824206xa_i2c_ids,
    };
    module_i2c_driver(lc824206xa_driver);
    MODULE_AUTHOR("Hans de Goede <hansg@kernel.org>");
    MODULE_DESCRIPTION("LC824206XA Micro USB Switch driver");
    MODULE_LICENSE("GPL");

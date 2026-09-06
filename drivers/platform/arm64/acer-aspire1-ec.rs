//! Automatically rewritten from C to Rust
//! Source: drivers/platform/arm64/acer-aspire1-ec.c
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
// Copyright (c) 2024, Nikita Travkin <nikita@trvn.ru>

pub const MILLI_TO_MICRO: c_int = 1000;
pub const ASPIRE_EC_EVENT: c_uint = 0x05;
pub const ASPIRE_EC_EVENT_WATCHDOG: c_uint = 0x20;
pub const ASPIRE_EC_EVENT_KBD_BKL_ON: c_uint = 0x57;
pub const ASPIRE_EC_EVENT_KBD_BKL_OFF: c_uint = 0x58;
pub const ASPIRE_EC_EVENT_LID_CLOSE: c_uint = 0x9b;
pub const ASPIRE_EC_EVENT_LID_OPEN: c_uint = 0x9c;
pub const ASPIRE_EC_EVENT_BKL_UNBLANKED: c_uint = 0x9d;
pub const ASPIRE_EC_EVENT_BKL_BLANKED: c_uint = 0x9e;
pub const ASPIRE_EC_EVENT_FG_INF_CHG: c_uint = 0x85;
pub const ASPIRE_EC_EVENT_FG_STA_CHG: c_uint = 0xc6;
pub const ASPIRE_EC_EVENT_HPD_DIS: c_uint = 0xa3;
pub const ASPIRE_EC_EVENT_HPD_CON: c_uint = 0xa4;
pub const ASPIRE_EC_FG_DYNAMIC: c_uint = 0x07;
pub const ASPIRE_EC_FG_STATIC: c_uint = 0x08;

pub const ASPIRE_EC_RAM_READ: c_uint = 0x20;
pub const ASPIRE_EC_RAM_WRITE: c_uint = 0x21;
pub const ASPIRE_EC_RAM_WATCHDOG: c_uint = 0x19;

pub const ASPIRE_EC_RAM_KBD_MODE: c_uint = 0x43;

pub const ASPIRE_EC_RAM_KBD_MODE_2: c_uint = 0x60;

pub const ASPIRE_EC_RAM_HPD_STATUS: c_uint = 0xf4;
pub const ASPIRE_EC_HPD_CONNECTED: c_uint = 0x03;
pub const ASPIRE_EC_RAM_LID_STATUS: c_uint = 0x4c;

pub const ASPIRE_EC_RAM_ADP: c_uint = 0x40;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspire_ec {
    pub client: *mut i2c_client,
    pub bat_psy: *mut power_supply,
    pub adp_psy: *mut power_supply,
    pub idev: *mut input_dev,
    pub bridge_configured: bool,
    pub bridge: drm_bridge,
    pub work: work_struct,
}

#[no_mangle]
unsafe extern "C" fn aspire_ec_ram_read(client: *mut i2c_client, off: u8, data: *mut u8, data_len: u8) -> c_int {
    static int aspire_ec_ram_read(struct i2c_client *client, u8 off, u8 *data, u8 data_len)
    {
    i2c_smbus_write_byte_data(client, ASPIRE_EC_RAM_READ, off);
    i2c_smbus_read_i2c_block_data(client, ASPIRE_EC_RAM_READ, data_len, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aspire_ec_ram_write(client: *mut i2c_client, off: u8, data: u8) -> c_int {
    static int aspire_ec_ram_write(struct i2c_client *client, u8 off, u8 data)
    {
    u8 tmp[2] = {off, data};
    i2c_smbus_write_i2c_block_data(client, ASPIRE_EC_RAM_WRITE, sizeof(tmp), tmp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aspire_ec_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t aspire_ec_irq_handler(int irq, void *data)
    {
    struct aspire_ec *ec = data;
    int id;
    u8 tmp;
//
// The original ACPI firmware actually has a small sleep in the handler.
//
// It seems like in most cases it's not needed but when the device
// just exits suspend, our i2c driver has a brief time where data
// transfer is not possible yet. So this delay allows us to suppress
// quite a bunch of spurious error messages in dmesg. Thus it's kept.
//
    usleep_range(15000, 30000);
    id = i2c_smbus_read_byte_data(ec.client, ASPIRE_EC_EVENT);
    if (id < 0) {
    dev_err(&ec.client.dev, "Failed to read event id: %pe\n", ERR_PTR(id));
    return IRQ_HANDLED;
    }
    switch (id) {
    case 0x0: /* No event */
    break;
    case ASPIRE_EC_EVENT_WATCHDOG:
//
// Here acpi responds to the event and clears some bit.
// Notify (\_SB.I2C3.BAT1, 0x81) // Information Change
// Notify (\_SB.I2C3.ADP1, 0x80) // Status Change
//
    aspire_ec_ram_read(ec.client, ASPIRE_EC_RAM_WATCHDOG, &tmp, sizeof(tmp));
    tmp &= ~ASPIRE_EC_WATCHDOG_BIT;
    aspire_ec_ram_write(ec.client, ASPIRE_EC_RAM_WATCHDOG, tmp);
    break;
    case ASPIRE_EC_EVENT_LID_CLOSE:
// Notify (\_SB.LID0, 0x80) // Status Change
    input_report_switch(ec.idev, SW_LID, 1);
    input_sync(ec.idev);
    break;
    case ASPIRE_EC_EVENT_LID_OPEN:
// Notify (\_SB.LID0, 0x80) // Status Change
    input_report_switch(ec.idev, SW_LID, 0);
    input_sync(ec.idev);
    break;
    case ASPIRE_EC_EVENT_FG_INF_CHG:
// Notify (\_SB.I2C3.BAT1, 0x81) // Information Change
    fallthrough;
    case ASPIRE_EC_EVENT_FG_STA_CHG:
// Notify (\_SB.I2C3.BAT1, 0x80) // Status Change
    power_supply_changed(ec.bat_psy);
    power_supply_changed(ec.adp_psy);
    break;
    case ASPIRE_EC_EVENT_HPD_DIS:
    if (ec.bridge_configured)
    drm_bridge_hpd_notify(&ec.bridge, connector_status_disconnected);
    break;
    case ASPIRE_EC_EVENT_HPD_CON:
    if (ec.bridge_configured)
    drm_bridge_hpd_notify(&ec.bridge, connector_status_connected);
    break;
    case ASPIRE_EC_EVENT_BKL_BLANKED:
    case ASPIRE_EC_EVENT_BKL_UNBLANKED:
// Display backlight blanked on FN+F6. No action needed.
    break;
    case ASPIRE_EC_EVENT_KBD_BKL_ON:
    case ASPIRE_EC_EVENT_KBD_BKL_OFF:
//
// There is a keyboard backlight connector on Aspire 1 that is
// controlled by FN+F8. There is no kb backlight on the device though.
// Seems like this is used on other devices like Acer Spin 7.
// No action needed.
//
    break;
    default:
    dev_warn(&ec.client.dev, "Unknown event id=0x%x\n", id);
    }
    return IRQ_HANDLED;
    }
//
// Power supply.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspire_ec_bat_psy_static_data {
    pub unk1: u8,
    pub flags: u8,
    pub unk2: __le16,
    pub voltage_design: __le16,
    pub capacity_full: __le16,
    pub unk3: __le16,
    pub serial: __le16,
    pub model_id: u8,
    pub vendor_id: u8,
    pub __packed: },
    static const char * const aspire_ec_bat_psy_battery_model[] = {
    "AP18C4K",
    "AP18C8K",
    "AP19B8K",
    "AP16M4J",
    "AP16M5J",
}

    static const char * const aspire_ec_bat_psy_battery_vendor[] = {
    "SANYO",
    "SONY",
    "PANASONIC",
    "SAMSUNG",
    "SIMPLO",
    "MOTOROLA",
    "CELXPERT",
    "LGC",
    "GETAC",
    "MURATA",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspire_ec_bat_psy_dynamic_data {
    pub unk1: u8,
    pub flags: u8,
    pub unk2: u8,
    pub capacity_now: __le16,
    pub voltage_now: __le16,
    pub current_now: __le16,
    pub unk3: __le16,
    pub unk4: __le16,
    pub __packed: },
    static int aspire_ec_bat_psy_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    pub power_supply_get_drvdata(psy): *mut *mut aspire_ec ec =,
    pub sdat: aspire_ec_bat_psy_static_data,
    pub ddat: aspire_ec_bat_psy_dynamic_data,
    pub 0: int str_index =,
    pub )&sdat): *mut i2c_smbus_read_i2c_block_data(ec->client, ASPIRE_EC_FG_STATIC, sizeof(sdat), (u8,
    pub )&ddat): *mut i2c_smbus_read_i2c_block_data(ec->client, ASPIRE_EC_FG_DYNAMIC, sizeof(ddat), (u8,
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    pub POWER_SUPPLY_STATUS_UNKNOWN: val->intval =,
    if (ddat.flags & ASPIRE_EC_FG_FLAG_CHARGING)
    pub POWER_SUPPLY_STATUS_CHARGING: val->intval =,
#[no_mangle]
pub unsafe extern "C" fn if(ASPIRE_EC_FG_FLAG_DISCHARGING: ddat.flags &) -> else {
    else if (ddat.flags & ASPIRE_EC_FG_FLAG_DISCHARGING)
    pub POWER_SUPPLY_STATUS_DISCHARGING: val->intval =,
#[no_mangle]
pub unsafe extern "C" fn if(ASPIRE_EC_FG_FLAG_FULL: ddat.flags &) -> else {
    else if (ddat.flags & ASPIRE_EC_FG_FLAG_FULL)
    pub POWER_SUPPLY_STATUS_FULL: val->intval =,
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    pub MILLI_TO_MICRO: *mut *mut val->intval = get_unaligned_le16(&ddat.voltage_now),
    case POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN:
    pub MILLI_TO_MICRO: *mut *mut val->intval = le16_to_cpu(sdat.voltage_design),
    case POWER_SUPPLY_PROP_CHARGE_NOW:
    pub MILLI_TO_MICRO: *mut *mut val->intval = get_unaligned_le16(&ddat.capacity_now),
    case POWER_SUPPLY_PROP_CHARGE_FULL:
    pub MILLI_TO_MICRO: *mut *mut val->intval = le16_to_cpu(sdat.capacity_full),
    case POWER_SUPPLY_PROP_CAPACITY:
    pub 100: *mut *mut val->intval = get_unaligned_le16(&ddat.capacity_now),
    pub le16_to_cpu(sdat.capacity_full): val->intval /=,
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    pub MILLI_TO_MICRO: *mut *mut val->intval = (s16)get_unaligned_le16(&ddat.current_now),
    case POWER_SUPPLY_PROP_PRESENT:
    pub ASPIRE_EC_FG_FLAG_PRESENT): val->intval = !!(ddat.flags &,
    case POWER_SUPPLY_PROP_SCOPE:
    pub POWER_SUPPLY_SCOPE_SYSTEM: val->intval =,
    case POWER_SUPPLY_PROP_MODEL_NAME:
    pub 1: str_index = sdat.model_id -,
    if (str_index >= 0 && str_index < ARRAY_SIZE(aspire_ec_bat_psy_battery_model))
    pub aspire_ec_bat_psy_battery_model: [val->strval =; str_index],
    else
    pub "Unknown": val->strval =,
    case POWER_SUPPLY_PROP_MANUFACTURER:
    pub /: *mut *mut str_index = sdat.vendor_id - 3; / ACPI uses 3 as an offset here.,
    if (str_index >= 0 && str_index < ARRAY_SIZE(aspire_ec_bat_psy_battery_vendor))
    pub aspire_ec_bat_psy_battery_vendor: [val->strval =; str_index],
    else
    pub "Unknown": val->strval =,
    default:
    pub -EINVAL: return,
    }
    pub 0: return,
    }
    static enum power_supply_property aspire_ec_bat_psy_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN,
    POWER_SUPPLY_PROP_CHARGE_NOW,
    POWER_SUPPLY_PROP_CHARGE_FULL,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_SCOPE,
    POWER_SUPPLY_PROP_MODEL_NAME,
    POWER_SUPPLY_PROP_MANUFACTURER,
}

    static const struct power_supply_desc aspire_ec_bat_psy_desc = {
    .name		= "aspire-ec-bat",
    .type		= POWER_SUPPLY_TYPE_BATTERY,
    .get_property	= aspire_ec_bat_psy_get_property,
    .properties	= aspire_ec_bat_psy_props,
    .num_properties	= ARRAY_SIZE(aspire_ec_bat_psy_props),
    };
    static int aspire_ec_adp_psy_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct aspire_ec *ec = power_supply_get_drvdata(psy);
    u8 tmp;
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    aspire_ec_ram_read(ec.client, ASPIRE_EC_RAM_ADP, &tmp, sizeof(tmp));
    val.intval = !!(tmp & ASPIRE_EC_AC_STATUS);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static enum power_supply_property aspire_ec_adp_psy_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    };
    static const struct power_supply_desc aspire_ec_adp_psy_desc = {
    .name		= "aspire-ec-adp",
    .type		= POWER_SUPPLY_TYPE_MAINS,
    .get_property	= aspire_ec_adp_psy_get_property,
    .properties	= aspire_ec_adp_psy_props,
    .num_properties	= ARRAY_SIZE(aspire_ec_adp_psy_props),
    };
//
// USB-C DP Alt mode HPD.
//
    static int aspire_ec_bridge_attach(struct drm_bridge *bridge, struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    return flags & DRM_BRIDGE_ATTACH_NO_CONNECTOR ? 0 : -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn aspire_ec_bridge_update_hpd_work(work: *mut work_struct) {
    static void aspire_ec_bridge_update_hpd_work(struct work_struct *work)
    {
    struct aspire_ec *ec = container_of(work, struct aspire_ec, work);
    u8 tmp;
    aspire_ec_ram_read(ec.client, ASPIRE_EC_RAM_HPD_STATUS, &tmp, sizeof(tmp));
    if (tmp == ASPIRE_EC_HPD_CONNECTED)
    drm_bridge_hpd_notify(&ec.bridge, connector_status_connected);
    else
    drm_bridge_hpd_notify(&ec.bridge, connector_status_disconnected);
    }
#[no_mangle]
unsafe extern "C" fn aspire_ec_bridge_hpd_enable(bridge: *mut drm_bridge) {
    static void aspire_ec_bridge_hpd_enable(struct drm_bridge *bridge)
    {
    struct aspire_ec *ec = container_of(bridge, struct aspire_ec, bridge);
    schedule_work(&ec.work);
    }
    static const struct drm_bridge_funcs aspire_ec_bridge_funcs = {
    .hpd_enable = aspire_ec_bridge_hpd_enable,
    .attach = aspire_ec_bridge_attach,
    };
//
// Sysfs attributes.
//
#[no_mangle]
unsafe extern "C" fn fn_lock_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t fn_lock_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct aspire_ec *ec = i2c_get_clientdata(to_i2c_client(dev));
    u8 tmp;
    aspire_ec_ram_read(ec.client, ASPIRE_EC_RAM_KBD_MODE, &tmp, sizeof(tmp));
    return sysfs_emit(buf, "%u\n", !(tmp & ASPIRE_EC_RAM_KBD_MEDIA_ON_TOP));
    }
    static ssize_t fn_lock_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct aspire_ec *ec = i2c_get_clientdata(to_i2c_client(dev));
    u8 tmp;
    bool state;
    int ret;
    ret = kstrtobool(buf, &state);
    if (ret)
    return ret;
    aspire_ec_ram_read(ec.client, ASPIRE_EC_RAM_KBD_MODE, &tmp, sizeof(tmp));
    if (state)
    tmp &= ~ASPIRE_EC_RAM_KBD_MEDIA_ON_TOP;
    else
    tmp |= ASPIRE_EC_RAM_KBD_MEDIA_ON_TOP;
    aspire_ec_ram_write(ec.client, ASPIRE_EC_RAM_KBD_MODE, tmp);
    return count;
    }
    static DEVICE_ATTR_RW(fn_lock);
    static struct attribute *aspire_ec_attrs[] = {
    &dev_attr_fn_lock.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(aspire_ec);
#[no_mangle]
unsafe extern "C" fn aspire_ec_probe(client: *mut i2c_client) -> c_int {
    static int aspire_ec_probe(struct i2c_client *client)
    {
    let mut psy_cfg: power_supply_config = {0};
    struct device *dev = &client.dev;
    struct fwnode_handle *fwnode;
    struct aspire_ec *ec;
    int ret;
    u8 tmp;
    ec = devm_drm_bridge_alloc(dev, struct aspire_ec, bridge, &aspire_ec_bridge_funcs);
    if (IS_ERR(ec))
    return PTR_ERR(ec);
    ec.client = client;
    i2c_set_clientdata(client, ec);
// Battery status reports
    psy_cfg.drv_data = ec;
    ec.bat_psy = devm_power_supply_register(dev, &aspire_ec_bat_psy_desc, &psy_cfg);
    if (IS_ERR(ec.bat_psy))
    return dev_err_probe(dev, PTR_ERR(ec.bat_psy),
    "Failed to register battery power supply\n");
    ec.adp_psy = devm_power_supply_register(dev, &aspire_ec_adp_psy_desc, &psy_cfg);
    if (IS_ERR(ec.adp_psy))
    return dev_err_probe(dev, PTR_ERR(ec.adp_psy),
    "Failed to register AC power supply\n");
// Lid switch
    ec.idev = devm_input_allocate_device(dev);
    if (!ec.idev)
    return -ENOMEM;
    ec.idev.name = "aspire-ec";
    ec.idev.phys = "aspire-ec/input0";
    input_set_capability(ec.idev, EV_SW, SW_LID);
    ret = input_register_device(ec.idev);
    if (ret)
    return dev_err_probe(dev, ret, "Input device register failed\n");
// Enable the keyboard fn keys
    tmp = ASPIRE_EC_RAM_KBD_FN_EN | ASPIRE_EC_RAM_KBD_ALWAYS_SET;
    tmp |= ASPIRE_EC_RAM_KBD_MEDIA_ON_TOP;
    aspire_ec_ram_write(client, ASPIRE_EC_RAM_KBD_MODE, tmp);
    aspire_ec_ram_read(client, ASPIRE_EC_RAM_KBD_MODE_2, &tmp, sizeof(tmp));
    tmp |= ASPIRE_EC_RAM_KBD_MEDIA_NOTIFY;
    aspire_ec_ram_write(client, ASPIRE_EC_RAM_KBD_MODE_2, tmp);
// External Type-C display attach reports
    fwnode = device_get_named_child_node(dev, "connector");
    if (fwnode) {
    INIT_WORK(&ec.work, aspire_ec_bridge_update_hpd_work);
    ec.bridge.of_node = to_of_node(fwnode);
    ec.bridge.ops = DRM_BRIDGE_OP_HPD;
    ec.bridge.type = DRM_MODE_CONNECTOR_USB;
    ret = devm_drm_bridge_add(dev, &ec.bridge);
    if (ret) {
    fwnode_handle_put(fwnode);
    return dev_err_probe(dev, ret, "Failed to register drm bridge\n");
    }
    ec.bridge_configured = true;
    }
    ret = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(),
    aspire_ec_irq_handler, IRQF_ONESHOT,
    dev_name(dev), ec);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to request irq\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aspire_ec_resume(dev: *mut device) -> c_int {
    static int aspire_ec_resume(struct device *dev)
    {
    struct aspire_ec *ec = i2c_get_clientdata(to_i2c_client(dev));
    u8 tmp;
    aspire_ec_ram_read(ec.client, ASPIRE_EC_RAM_LID_STATUS, &tmp, sizeof(tmp));
    input_report_switch(ec.idev, SW_LID, !!(tmp & ASPIRE_EC_LID_OPEN));
    input_sync(ec.idev);
    return 0;
    }
    static const struct i2c_device_id aspire_ec_id[] = {
    { .name = "aspire1-ec" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, aspire_ec_id);
    static const struct of_device_id aspire_ec_of_match[] = {
    { .compatible = "acer,aspire1-ec", },
    { }
    };
    MODULE_DEVICE_TABLE(of, aspire_ec_of_match);
    static DEFINE_SIMPLE_DEV_PM_OPS(aspire_ec_pm_ops, core::ptr::null_mut(), aspire_ec_resume);
    static struct i2c_driver aspire_ec_driver = {
    .driver = {
    .name = "aspire-ec",
    .of_match_table = aspire_ec_of_match,
    .pm = pm_sleep_ptr(&aspire_ec_pm_ops),
    .dev_groups = aspire_ec_groups,
    },
    .probe = aspire_ec_probe,
    .id_table = aspire_ec_id,
    };
    module_i2c_driver(aspire_ec_driver);
    MODULE_DESCRIPTION("Acer Aspire 1 embedded controller");
    MODULE_AUTHOR("Nikita Travkin <nikita@trvn.ru>");
    MODULE_LICENSE("GPL");

//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/lp8727_charger.c
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
// Driver for LP8727 Micro/Mini USB IC with integrated charger
//
// Copyright (C) 2011 Texas Instruments
// Copyright (C) 2011 National Semiconductor
//

pub const LP8788_NUM_INTREGS: c_int = 2;
pub const DEFAULT_DEBOUNCE_MSEC: c_int = 270;
// Registers
pub const LP8727_CTRL1: c_uint = 0x1;
pub const LP8727_CTRL2: c_uint = 0x2;
pub const LP8727_SWCTRL: c_uint = 0x3;
pub const LP8727_INT1: c_uint = 0x4;
pub const LP8727_INT2: c_uint = 0x5;
pub const LP8727_STATUS1: c_uint = 0x6;
pub const LP8727_STATUS2: c_uint = 0x7;
pub const LP8727_CHGCTRL2: c_uint = 0x9;
// CTRL1 register

// CTRL2 register

// SWCTRL register

// INT1 register

// STATUS1 register

pub const LP8727_STAT_EOC: c_uint = 0x30;
// STATUS2 register

pub const LP8727_TEMP_SHIFT: c_int = 5;
// CHGCTRL2 register
pub const LP8727_ICHG_SHIFT: c_int = 4;
    enum lp8727_dev_id {
    LP8727_ID_NONE,
    LP8727_ID_TA,
    LP8727_ID_DEDICATED_CHG,
    LP8727_ID_USB_CHG,
    LP8727_ID_USB_DS,
    LP8727_ID_MAX,
    };
    enum lp8727_die_temp {
    LP8788_TEMP_75C,
    LP8788_TEMP_95C,
    LP8788_TEMP_115C,
    LP8788_TEMP_135C,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8727_psy {
    pub ac: *mut power_supply,
    pub usb: *mut power_supply,
    pub batt: *mut power_supply,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8727_chg {
    pub dev: *mut device,
    pub client: *mut i2c_client,
    pub xfer_lock: mutex,
    pub psy: *mut lp8727_psy,
    pub pdata: *mut lp8727_platform_data,
// Charger Data
    pub devid: enum lp8727_dev_id,
    pub chg_param: *mut lp8727_chg_param,
// Interrupt Handling
    pub irq: c_int,
    pub work: delayed_work,
    pub debounce_jiffies: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn lp8727_read_bytes(pchg: *mut lp8727_chg, reg: u8, data: *mut u8, len: u8) -> c_int {
    static int lp8727_read_bytes(struct lp8727_chg *pchg, u8 reg, u8 *data, u8 len)
    {
    s32 ret;
    mutex_lock(&pchg.xfer_lock);
    ret = i2c_smbus_read_i2c_block_data(pchg.client, reg, len, data);
    mutex_unlock(&pchg.xfer_lock);
    return (ret != len) ? -EIO : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn lp8727_read_byte(pchg: *mut lp8727_chg, reg: u8, data: *mut u8) -> c_int {
    static inline int lp8727_read_byte(struct lp8727_chg *pchg, u8 reg, u8 *data)
    {
    return lp8727_read_bytes(pchg, reg, data, 1);
    }
#[no_mangle]
unsafe extern "C" fn lp8727_write_byte(pchg: *mut lp8727_chg, reg: u8, data: u8) -> c_int {
    static int lp8727_write_byte(struct lp8727_chg *pchg, u8 reg, u8 data)
    {
    int ret;
    mutex_lock(&pchg.xfer_lock);
    ret = i2c_smbus_write_byte_data(pchg.client, reg, data);
    mutex_unlock(&pchg.xfer_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lp8727_is_charger_attached(name: *const c_char, id: c_int) -> bool {
    static bool lp8727_is_charger_attached(const char *name, int id)
    {
    if (!strcmp(name, "ac"))
    let mut id: return = = LP8727_ID_TA || id == LP8727_ID_DEDICATED_CHG;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(name, _arg: "usb")) -> else {
    else if (!strcmp(name, "usb"))
    let mut id: return = = LP8727_ID_USB_CHG;
    return id >= LP8727_ID_TA && id <= LP8727_ID_USB_CHG;
    }
#[no_mangle]
unsafe extern "C" fn lp8727_init_device(pchg: *mut lp8727_chg) -> c_int {
    static int lp8727_init_device(struct lp8727_chg *pchg)
    {
    u8 val;
    int ret;
    u8 intstat[LP8788_NUM_INTREGS];
// clear interrupts
    ret = lp8727_read_bytes(pchg, LP8727_INT1, intstat, LP8788_NUM_INTREGS);
    if (ret)
    return ret;
    val = LP8727_ID200_EN | LP8727_ADC_EN | LP8727_CP_EN;
    ret = lp8727_write_byte(pchg, LP8727_CTRL1, val);
    if (ret)
    return ret;
    val = LP8727_INT_EN | LP8727_CHGDET_EN;
    return lp8727_write_byte(pchg, LP8727_CTRL2, val);
    }
#[no_mangle]
unsafe extern "C" fn lp8727_is_dedicated_charger(pchg: *mut lp8727_chg) -> c_int {
    static int lp8727_is_dedicated_charger(struct lp8727_chg *pchg)
    {
    u8 val;
    lp8727_read_byte(pchg, LP8727_STATUS1, &val);
    return val & LP8727_DCPORT;
    }
#[no_mangle]
unsafe extern "C" fn lp8727_is_usb_charger(pchg: *mut lp8727_chg) -> c_int {
    static int lp8727_is_usb_charger(struct lp8727_chg *pchg)
    {
    u8 val;
    lp8727_read_byte(pchg, LP8727_STATUS1, &val);
    return val & LP8727_CHPORT;
    }
#[no_mangle]
pub unsafe extern "C" fn lp8727_ctrl_switch(pchg: *mut lp8727_chg, sw: u8) {
    static inline void lp8727_ctrl_switch(struct lp8727_chg *pchg, u8 sw)
    {
    lp8727_write_byte(pchg, LP8727_SWCTRL, sw);
    }
#[no_mangle]
unsafe extern "C" fn lp8727_id_detection(pchg: *mut lp8727_chg, id: u8, vbusin: c_int) {
    static void lp8727_id_detection(struct lp8727_chg *pchg, u8 id, int vbusin)
    {
    struct lp8727_platform_data *pdata = pchg.pdata;
    let mut devid: u8 = LP8727_ID_NONE;
    let mut swctrl: u8 = LP8727_SW_DM1_HiZ | LP8727_SW_DP2_HiZ;
    switch (id) {
    case 0x5:
    devid = LP8727_ID_TA;
    pchg.chg_param = pdata ? pdata.ac : core::ptr::null_mut();
    break;
    case 0xB:
    if (lp8727_is_dedicated_charger(pchg)) {
    pchg.chg_param = pdata ? pdata.ac : core::ptr::null_mut();
    devid = LP8727_ID_DEDICATED_CHG;
    } else if (lp8727_is_usb_charger(pchg)) {
    pchg.chg_param = pdata ? pdata.usb : core::ptr::null_mut();
    devid = LP8727_ID_USB_CHG;
    swctrl = LP8727_SW_DM1_DM | LP8727_SW_DP2_DP;
    } else if (vbusin) {
    devid = LP8727_ID_USB_DS;
    swctrl = LP8727_SW_DM1_DM | LP8727_SW_DP2_DP;
    }
    break;
    default:
    devid = LP8727_ID_NONE;
    pchg.chg_param = core::ptr::null_mut();
    break;
    }
    pchg.devid = devid;
    lp8727_ctrl_switch(pchg, swctrl);
    }
#[no_mangle]
unsafe extern "C" fn lp8727_enable_chgdet(pchg: *mut lp8727_chg) {
    static void lp8727_enable_chgdet(struct lp8727_chg *pchg)
    {
    u8 val;
    lp8727_read_byte(pchg, LP8727_CTRL2, &val);
    val |= LP8727_CHGDET_EN;
    lp8727_write_byte(pchg, LP8727_CTRL2, val);
    }
#[no_mangle]
unsafe extern "C" fn lp8727_delayed_func(_work: *mut work_struct) {
    static void lp8727_delayed_func(struct work_struct *_work)
    {
    struct lp8727_chg *pchg = container_of(_work, struct lp8727_chg,
    work.work);
    u8 intstat[LP8788_NUM_INTREGS];
    u8 idno;
    u8 vbus;
    if (lp8727_read_bytes(pchg, LP8727_INT1, intstat, LP8788_NUM_INTREGS)) {
    dev_err(pchg.dev, "can not read INT registers\n");
    return;
    }
    idno = intstat[0] & LP8727_IDNO;
    vbus = intstat[0] & LP8727_VBUS;
    lp8727_id_detection(pchg, idno, vbus);
    lp8727_enable_chgdet(pchg);
    power_supply_changed(pchg.psy.ac);
    power_supply_changed(pchg.psy.usb);
    power_supply_changed(pchg.psy.batt);
    }
#[no_mangle]
unsafe extern "C" fn lp8727_isr_func(irq: c_int, ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t lp8727_isr_func(int irq, void *ptr)
    {
    struct lp8727_chg *pchg = ptr;
    schedule_delayed_work(&pchg.work, pchg.debounce_jiffies);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn lp8727_setup_irq(pchg: *mut lp8727_chg) -> c_int {
    static int lp8727_setup_irq(struct lp8727_chg *pchg)
    {
    int ret;
    let mut irq: c_int = pchg.client.irq;
    unsigned delay_msec = pchg.pdata ? pchg.pdata.debounce_msec :
    DEFAULT_DEBOUNCE_MSEC;
    INIT_DELAYED_WORK(&pchg.work, lp8727_delayed_func);
    if (irq <= 0) {
    dev_warn(pchg.dev, "invalid irq number: %d\n", irq);
    return 0;
    }
    ret = request_threaded_irq(irq,	core::ptr::null_mut(), lp8727_isr_func,
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    "lp8727_irq", pchg);
    if (ret)
    return ret;
    pchg.irq = irq;
    pchg.debounce_jiffies = msecs_to_jiffies(delay_msec);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lp8727_release_irq(pchg: *mut lp8727_chg) {
    static void lp8727_release_irq(struct lp8727_chg *pchg)
    {
    if (pchg.irq)
    free_irq(pchg.irq, pchg);
    cancel_delayed_work_sync(&pchg.work);
    }
    static enum power_supply_property lp8727_charger_prop[] = {
    POWER_SUPPLY_PROP_ONLINE,
    };
    static enum power_supply_property lp8727_battery_prop[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_TEMP,
    };
    static char *battery_supplied_to[] = {
    "main_batt",
    };
    static int lp8727_charger_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct lp8727_chg *pchg = dev_get_drvdata(psy.dev.parent);
    if (psp != POWER_SUPPLY_PROP_ONLINE)
    return -EINVAL;
    val.intval = lp8727_is_charger_attached(psy.desc.name, pchg.devid);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lp8727_is_high_temperature(temp: enum lp8727_die_temp) -> bool {
    static bool lp8727_is_high_temperature(enum lp8727_die_temp temp)
    {
    switch (temp) {
    case LP8788_TEMP_95C:
    case LP8788_TEMP_115C:
    case LP8788_TEMP_135C:
    return true;
    default:
    return false;
    }
    }
    static int lp8727_battery_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct lp8727_chg *pchg = dev_get_drvdata(psy.dev.parent);
    struct lp8727_platform_data *pdata = pchg.pdata;
    enum lp8727_die_temp temp;
    u8 read;
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    if (!lp8727_is_charger_attached(psy.desc.name, pchg.devid)) {
    val.intval = POWER_SUPPLY_STATUS_DISCHARGING;
    return 0;
    }
    lp8727_read_byte(pchg, LP8727_STATUS1, &read);
    val.intval = (read & LP8727_CHGSTAT) == LP8727_STAT_EOC ?
    POWER_SUPPLY_STATUS_FULL :
    POWER_SUPPLY_STATUS_CHARGING;
    break;
    case POWER_SUPPLY_PROP_HEALTH:
    lp8727_read_byte(pchg, LP8727_STATUS2, &read);
    temp = (read & LP8727_TEMP_STAT) >> LP8727_TEMP_SHIFT;
    val.intval = lp8727_is_high_temperature(temp) ?
    POWER_SUPPLY_HEALTH_OVERHEAT :
    POWER_SUPPLY_HEALTH_GOOD;
    break;
    case POWER_SUPPLY_PROP_PRESENT:
    if (!pdata)
    return -EINVAL;
    if (pdata.get_batt_present)
    val.intval = pdata.get_batt_present();
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    if (!pdata)
    return -EINVAL;
    if (pdata.get_batt_level)
    val.intval = pdata.get_batt_level();
    break;
    case POWER_SUPPLY_PROP_CAPACITY:
    if (!pdata)
    return -EINVAL;
    if (pdata.get_batt_capacity)
    val.intval = pdata.get_batt_capacity();
    break;
    case POWER_SUPPLY_PROP_TEMP:
    if (!pdata)
    return -EINVAL;
    if (pdata.get_batt_temp)
    val.intval = pdata.get_batt_temp();
    break;
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lp8727_charger_changed(psy: *mut power_supply) {
    static void lp8727_charger_changed(struct power_supply *psy)
    {
    struct lp8727_chg *pchg = dev_get_drvdata(psy.dev.parent);
    u8 eoc_level;
    u8 ichg;
    u8 val;
// skip if no charger exists
    if (!lp8727_is_charger_attached(psy.desc.name, pchg.devid))
    return;
// update charging parameters
    if (pchg.chg_param) {
    eoc_level = pchg.chg_param.eoc_level;
    ichg = pchg.chg_param.ichg;
    val = (ichg << LP8727_ICHG_SHIFT) | eoc_level;
    lp8727_write_byte(pchg, LP8727_CHGCTRL2, val);
    }
    }
    static const struct power_supply_desc lp8727_ac_desc = {
    .name			= "ac",
    .type			= POWER_SUPPLY_TYPE_MAINS,
    .properties		= lp8727_charger_prop,
    .num_properties		= ARRAY_SIZE(lp8727_charger_prop),
    .get_property		= lp8727_charger_get_property,
    };
    static const struct power_supply_desc lp8727_usb_desc = {
    .name			= "usb",
    .type			= POWER_SUPPLY_TYPE_USB,
    .properties		= lp8727_charger_prop,
    .num_properties		= ARRAY_SIZE(lp8727_charger_prop),
    .get_property		= lp8727_charger_get_property,
    };
    static const struct power_supply_desc lp8727_batt_desc = {
    .name			= "main_batt",
    .type			= POWER_SUPPLY_TYPE_BATTERY,
    .properties		= lp8727_battery_prop,
    .num_properties		= ARRAY_SIZE(lp8727_battery_prop),
    .get_property		= lp8727_battery_get_property,
    .external_power_changed	= lp8727_charger_changed,
    };
#[no_mangle]
unsafe extern "C" fn lp8727_register_psy(pchg: *mut lp8727_chg) -> c_int {
    static int lp8727_register_psy(struct lp8727_chg *pchg)
    {
    struct power_supply_config psy_cfg = {}; /* Only for ac and usb */
    struct lp8727_psy *psy;
    psy = devm_kzalloc(pchg.dev, sizeof(*psy), GFP_KERNEL);
    if (!psy)
    return -ENOMEM;
    pchg.psy = psy;
    psy_cfg.supplied_to = battery_supplied_to;
    psy_cfg.num_supplicants = ARRAY_SIZE(battery_supplied_to);
    psy.ac = devm_power_supply_register(pchg.dev, &lp8727_ac_desc, &psy_cfg);
    if (IS_ERR(psy.ac))
    return -EPERM;
    psy.usb = devm_power_supply_register(pchg.dev, &lp8727_usb_desc,
    &psy_cfg);
    if (IS_ERR(psy.usb))
    return -EPERM;
    psy.batt = devm_power_supply_register(pchg.dev, &lp8727_batt_desc, core::ptr::null_mut());
    if (IS_ERR(psy.batt))
    return -EPERM;
    return 0;
    }

    static struct lp8727_chg_param
// lp8727_parse_charge_pdata(struct device *dev, struct device_node *np)
    {
    struct lp8727_chg_param *param;
    param = devm_kzalloc(dev, sizeof(*param), GFP_KERNEL);
    if (!param)
    goto out;
    of_property_read_u8(np, "eoc-level", (u8 *)&param.eoc_level);
    of_property_read_u8(np, "charging-current", (u8 *)&param.ichg);
    out:
    return param;
    }
    static struct lp8727_platform_data *lp8727_parse_dt(struct device *dev)
    {
    struct device_node *np = dev.of_node;
    struct device_node *child;
    struct lp8727_platform_data *pdata;
    const char *type;
    pdata = devm_kzalloc(dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return ERR_PTR(-ENOMEM);
    of_property_read_u32(np, "debounce-ms", &pdata.debounce_msec);
// If charging parameter is not defined, just skip parsing the dt
    if (of_get_child_count(np) == 0)
    return pdata;
    for_each_child_of_node(np, child) {
    of_property_read_string(child, "charger-type", &type);
    if (!strcmp(type, "ac"))
    pdata.ac = lp8727_parse_charge_pdata(dev, child);
    if (!strcmp(type, "usb"))
    pdata.usb = lp8727_parse_charge_pdata(dev, child);
    }
    return pdata;
    }

    static struct lp8727_platform_data *lp8727_parse_dt(struct device *dev)
    {
    return core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn lp8727_probe(cl: *mut i2c_client) -> c_int {
    static int lp8727_probe(struct i2c_client *cl)
    {
    struct lp8727_chg *pchg;
    struct lp8727_platform_data *pdata;
    int ret;
    if (!i2c_check_functionality(cl.adapter, I2C_FUNC_SMBUS_I2C_BLOCK))
    return -EIO;
    if (cl.dev.of_node) {
    pdata = lp8727_parse_dt(&cl.dev);
    if (IS_ERR(pdata))
    return PTR_ERR(pdata);
    } else {
    pdata = dev_get_platdata(&cl.dev);
    }
    pchg = devm_kzalloc(&cl.dev, sizeof(*pchg), GFP_KERNEL);
    if (!pchg)
    return -ENOMEM;
    pchg.client = cl;
    pchg.dev = &cl.dev;
    pchg.pdata = pdata;
    i2c_set_clientdata(cl, pchg);
    mutex_init(&pchg.xfer_lock);
    ret = lp8727_init_device(pchg);
    if (ret) {
    dev_err(pchg.dev, "i2c communication err: %d", ret);
    return ret;
    }
    ret = lp8727_register_psy(pchg);
    if (ret) {
    dev_err(pchg.dev, "power supplies register err: %d", ret);
    return ret;
    }
    ret = lp8727_setup_irq(pchg);
    if (ret) {
    dev_err(pchg.dev, "irq handler err: %d", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lp8727_remove(cl: *mut i2c_client) {
    static void lp8727_remove(struct i2c_client *cl)
    {
    struct lp8727_chg *pchg = i2c_get_clientdata(cl);
    lp8727_release_irq(pchg);
    }
    static const struct of_device_id lp8727_dt_ids[] __maybe_unused = {
    { .compatible = "ti,lp8727", },
    { }
    };
    MODULE_DEVICE_TABLE(of, lp8727_dt_ids);
    static const struct i2c_device_id lp8727_ids[] = {
    { .name = "lp8727" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lp8727_ids);
    static struct i2c_driver lp8727_driver = {
    .driver = {
    .name = "lp8727",
    .of_match_table = of_match_ptr(lp8727_dt_ids),
    },
    .probe = lp8727_probe,
    .remove = lp8727_remove,
    .id_table = lp8727_ids,
    };
    module_i2c_driver(lp8727_driver);
    MODULE_DESCRIPTION("TI/National Semiconductor LP8727 charger driver");
    MODULE_AUTHOR("Milo Kim <milo.kim@ti.com>, Daniel Jeong <daniel.jeong@ti.com>");
    MODULE_LICENSE("GPL");

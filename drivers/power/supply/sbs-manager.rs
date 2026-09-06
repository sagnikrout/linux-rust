//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/sbs-manager.c
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
// Driver for SBS compliant Smart Battery System Managers
//
// The device communicates via i2c at address 0x0a and multiplexes access to up
// to four smart batteries at address 0x0b.
//
// Via sysfs interface the online state and charge type are presented.
//
// Datasheet SBSM:    http://sbs-forum.org/specs/sbsm100b.pdf
// Datasheet LTC1760: http://cds.linear.com/docs/en/datasheet/1760fb.pdf
//
// Karl-Heinz Schneider <karl-heinz@schneider-inet.de>
//

pub const SBSM_MAX_BATS: c_int = 4;
pub const SBSM_RETRY_CNT: c_int = 3;
// registers addresses
pub const SBSM_CMD_BATSYSSTATE: c_uint = 0x01;
pub const SBSM_CMD_BATSYSSTATECONT: c_uint = 0x02;
pub const SBSM_CMD_BATSYSINFO: c_uint = 0x04;
pub const SBSM_CMD_LTC: c_uint = 0x3c;

pub const SBSM_SMB_BAT_OFFSET: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbsm_data {
    pub client: *mut i2c_client,
    pub muxc: *mut i2c_mux_core,
    pub psy: *mut power_supply,
    pub /: *mut *mut u8 cur_chan; / currently selected channel,
    pub chip: gpio_chip,
    pub /: *mut *mut bool is_ltc1760; / special capabilities,
    pub supported_bats: c_uint,
    pub last_state: c_uint,
    pub last_state_cont: c_uint,
}

    static enum power_supply_property sbsm_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_CHARGE_TYPE,
    };
#[no_mangle]
unsafe extern "C" fn sbsm_read_word(client: *mut i2c_client, address: u8) -> c_int {
    static int sbsm_read_word(struct i2c_client *client, u8 address)
    {
    int reg, retries;
    for (retries = SBSM_RETRY_CNT; retries > 0; retries--) {
    reg = i2c_smbus_read_word_data(client, address);
    if (reg >= 0)
    break;
    }
    if (reg < 0) {
    dev_err(&client.dev, "failed to read register 0x%02x\n",
    address);
    }
    return reg;
    }
#[no_mangle]
unsafe extern "C" fn sbsm_write_word(client: *mut i2c_client, address: u8, word: u16) -> c_int {
    static int sbsm_write_word(struct i2c_client *client, u8 address, u16 word)
    {
    int ret, retries;
    for (retries = SBSM_RETRY_CNT; retries > 0; retries--) {
    ret = i2c_smbus_write_word_data(client, address, word);
    if (ret >= 0)
    break;
    }
    if (ret < 0)
    dev_err(&client.dev, "failed to write to register 0x%02x\n",
    address);
    return ret;
    }
    static int sbsm_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct sbsm_data *data = power_supply_get_drvdata(psy);
    let mut regval: c_int = 0;
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    regval = sbsm_read_word(data.client, SBSM_CMD_BATSYSSTATECONT);
    if (regval < 0)
    return regval;
    val.intval = !!(regval & SBSM_BIT_AC_PRESENT);
    break;
    case POWER_SUPPLY_PROP_CHARGE_TYPE:
    regval = sbsm_read_word(data.client, SBSM_CMD_BATSYSSTATE);
    if (regval < 0)
    return regval;
    if ((regval & SBSM_MASK_CHARGE_BAT) == 0) {
    val.intval = POWER_SUPPLY_CHARGE_TYPE_NONE;
    return 0;
    }
    val.intval = POWER_SUPPLY_CHARGE_TYPE_TRICKLE;
    if (data.is_ltc1760) {
// charge mode fast if turbo is active
    regval = sbsm_read_word(data.client, SBSM_CMD_LTC);
    if (regval < 0)
    return regval;
#[no_mangle]
pub unsafe extern "C" fn if(SBSM_BIT_TURBO: regval &) -> else {
    else if (regval & SBSM_BIT_TURBO)
    val.intval = POWER_SUPPLY_CHARGE_TYPE_FAST;
    }
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int sbsm_prop_is_writeable(struct power_supply *psy,
    enum power_supply_property psp)
    {
    struct sbsm_data *data = power_supply_get_drvdata(psy);
    return (psp == POWER_SUPPLY_PROP_CHARGE_TYPE) && data.is_ltc1760;
    }
    static int sbsm_set_property(struct power_supply *psy,
    enum power_supply_property psp,
    const union power_supply_propval *val)
    {
    struct sbsm_data *data = power_supply_get_drvdata(psy);
    let mut ret: c_int = -EINVAL;
    u16 regval;
    switch (psp) {
    case POWER_SUPPLY_PROP_CHARGE_TYPE:
// write 1 to TURBO if type fast is given
    if (!data.is_ltc1760)
    break;
    regval = val.intval ==
    POWER_SUPPLY_CHARGE_TYPE_FAST ? SBSM_BIT_TURBO : 0;
    ret = sbsm_write_word(data.client, SBSM_CMD_LTC, regval);
    break;
    default:
    break;
    }
    return ret;
    }
//
// Switch to battery
// Parameter chan is directly the content of SMB_BAT* nibble
//
#[no_mangle]
unsafe extern "C" fn sbsm_select(muxc: *mut i2c_mux_core, chan: u32) -> c_int {
    static int sbsm_select(struct i2c_mux_core *muxc, u32 chan)
    {
    struct sbsm_data *data = i2c_mux_priv(muxc);
    struct device *dev = &data.client.dev;
    let mut ret: c_int = 0;
    u16 reg;
    if (data.cur_chan == chan)
    return ret;
// chan goes from 1 ... 4
    reg = BIT(SBSM_SMB_BAT_OFFSET + chan);
    ret = sbsm_write_word(data.client, SBSM_CMD_BATSYSSTATE, reg);
    if (ret)
    dev_err(dev, "Failed to select channel %i\n", chan);
    else
    data.cur_chan = chan;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sbsm_gpio_get_value(gc: *mut gpio_chip, off: c_uint) -> c_int {
    static int sbsm_gpio_get_value(struct gpio_chip *gc, unsigned int off)
    {
    struct sbsm_data *data = gpiochip_get_data(gc);
    int ret;
    ret = sbsm_read_word(data.client, SBSM_CMD_BATSYSSTATE);
    if (ret < 0)
    return ret;
    return !!(ret & BIT(off));
    }
//
// This needs to be defined or the GPIO lib fails to register the pin.
// But the 'gpio' is always an input.
//
#[no_mangle]
unsafe extern "C" fn sbsm_gpio_direction_input(gc: *mut gpio_chip, off: c_uint) -> c_int {
    static int sbsm_gpio_direction_input(struct gpio_chip *gc, unsigned int off)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sbsm_do_alert(dev: *mut device, d: *mut c_void) -> c_int {
    static int sbsm_do_alert(struct device *dev, void *d)
    {
    struct i2c_client *client = i2c_verify_client(dev);
    struct i2c_driver *driver;
    if (!client || client.addr != 0x0b)
    return 0;
    device_lock(dev);
    if (client.dev.driver) {
    driver = to_i2c_driver(client.dev.driver);
    if (driver.alert)
    driver.alert(client, I2C_PROTOCOL_SMBUS_ALERT, 0);
    else
    dev_warn(&client.dev, "no driver alert()!\n");
    } else {
    dev_dbg(&client.dev, "alert with no driver\n");
    }
    device_unlock(dev);
    return -EBUSY;
    }
    static void sbsm_alert(struct i2c_client *client, enum i2c_alert_protocol prot,
    unsigned int d)
    {
    struct sbsm_data *sbsm = i2c_get_clientdata(client);
    int ret, i, irq_bat = 0, state = 0;
    ret = sbsm_read_word(sbsm.client, SBSM_CMD_BATSYSSTATE);
    if (ret >= 0) {
    irq_bat = ret ^ sbsm.last_state;
    sbsm.last_state = ret;
    state = ret;
    }
    ret = sbsm_read_word(sbsm.client, SBSM_CMD_BATSYSSTATECONT);
    if ((ret >= 0) &&
    ((ret ^ sbsm.last_state_cont) & SBSM_BIT_AC_PRESENT)) {
    irq_bat |= sbsm.supported_bats & state;
    power_supply_changed(sbsm.psy);
    }
    sbsm.last_state_cont = ret;
    for (i = 0; i < SBSM_MAX_BATS; i++) {
    if (irq_bat & BIT(i)) {
    device_for_each_child(&sbsm.muxc.adapter[i].dev,
    core::ptr::null_mut(), sbsm_do_alert);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn sbsm_gpio_setup(data: *mut sbsm_data) -> c_int {
    static int sbsm_gpio_setup(struct sbsm_data *data)
    {
    struct gpio_chip *gc = &data.chip;
    struct i2c_client *client = data.client;
    struct device *dev = &client.dev;
    int ret;
    if (!device_property_present(dev, "gpio-controller"))
    return 0;
    ret  = sbsm_read_word(client, SBSM_CMD_BATSYSSTATE);
    if (ret < 0)
    return ret;
    data.last_state = ret;
    ret  = sbsm_read_word(client, SBSM_CMD_BATSYSSTATECONT);
    if (ret < 0)
    return ret;
    data.last_state_cont = ret;
    gc.get = sbsm_gpio_get_value;
    gc.direction_input  = sbsm_gpio_direction_input;
    gc.can_sleep = true;
    gc.base = -1;
    gc.ngpio = SBSM_MAX_BATS;
    gc.label = client.name;
    gc.parent = dev;
    gc.owner = THIS_MODULE;
    ret = devm_gpiochip_add_data(dev, gc, data);
    if (ret)
    return dev_err_probe(dev, ret, "devm_gpiochip_add_data failed\n");
    return ret;
    }
    static const struct power_supply_desc sbsm_default_psy_desc = {
    .type = POWER_SUPPLY_TYPE_MAINS,
    .properties = sbsm_props,
    .num_properties = ARRAY_SIZE(sbsm_props),
    .get_property = &sbsm_get_property,
    .set_property = &sbsm_set_property,
    .property_is_writeable = &sbsm_prop_is_writeable,
    };
#[no_mangle]
unsafe extern "C" fn sbsm_del_mux_adapter(data: *mut c_void) {
    static void sbsm_del_mux_adapter(void *data)
    {
    struct sbsm_data *sbsm = data;
    i2c_mux_del_adapters(sbsm.muxc);
    }
#[no_mangle]
unsafe extern "C" fn sbsm_probe(client: *mut i2c_client) -> c_int {
    static int sbsm_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct i2c_adapter *adapter = client.adapter;
    struct sbsm_data *data;
    struct device *dev = &client.dev;
    struct power_supply_desc *psy_desc;
    let mut psy_cfg: power_supply_config = {};
    let mut ret: c_int = 0, i;
// Device listens only at address 0x0a
    if (client.addr != 0x0a)
    return -EINVAL;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_WORD_DATA))
    return -EPFNOSUPPORT;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    i2c_set_clientdata(client, data);
    data.client = client;
    data.is_ltc1760 = !!strstr(id.name, "ltc1760");
    ret  = sbsm_read_word(client, SBSM_CMD_BATSYSINFO);
    if (ret < 0)
    return ret;
    data.supported_bats = ret & SBSM_MASK_BAT_SUPPORTED;
    data.muxc = i2c_mux_alloc(adapter, dev, SBSM_MAX_BATS, 0,
    I2C_MUX_LOCKED, &sbsm_select, core::ptr::null_mut());
    if (!data.muxc)
    return -ENOMEM;
    data.muxc.priv = data;
    ret = devm_add_action_or_reset(dev, sbsm_del_mux_adapter, data);
    if (ret)
    return ret;
// register muxed i2c channels. One for each supported battery
    for (i = 0; i < SBSM_MAX_BATS; ++i) {
    if (data.supported_bats & BIT(i)) {
    ret = i2c_mux_add_adapter(data.muxc, 0, i + 1);
    if (ret)
    break;
    }
    }
    if (ret)
    return dev_err_probe(dev, ret, "failed to register i2c mux channel %d\n", i + 1);
    psy_desc = devm_kmemdup(dev, &sbsm_default_psy_desc, sizeof(*psy_desc), GFP_KERNEL);
    if (!psy_desc)
    return -ENOMEM;
    psy_desc.name = devm_kasprintf(dev, GFP_KERNEL, "sbsm-%s", dev_name(&client.dev));
    if (!psy_desc.name)
    return -ENOMEM;
    ret = sbsm_gpio_setup(data);
    if (ret < 0)
    return ret;
    psy_cfg.drv_data = data;
    psy_cfg.fwnode = dev_fwnode(dev);
    data.psy = devm_power_supply_register(dev, psy_desc, &psy_cfg);
    if (IS_ERR(data.psy))
    return dev_err_probe(dev, PTR_ERR(data.psy),
    "failed to register power supply %s\n", psy_desc.name);
    return 0;
    }
    static const struct i2c_device_id sbsm_ids[] = {
    { .name = "sbs-manager" },
    { .name = "ltc1760" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, sbsm_ids);

    static const struct of_device_id sbsm_dt_ids[] = {
    { .compatible = "sbs,sbs-manager" },
    { .compatible = "lltc,ltc1760" },
    { }
    };
    MODULE_DEVICE_TABLE(of, sbsm_dt_ids);

    static struct i2c_driver sbsm_driver = {
    .driver = {
    .name = "sbsm",
    .of_match_table = of_match_ptr(sbsm_dt_ids),
    },
    .probe		= sbsm_probe,
    .alert		= sbsm_alert,
    .id_table	= sbsm_ids
    };
    module_i2c_driver(sbsm_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Karl-Heinz Schneider <karl-heinz@schneider-inet.de>");
    MODULE_DESCRIPTION("SBSM Smart Battery System Manager");

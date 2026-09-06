//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/hd3ss3220.c
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
// TI HD3SS3220 Type-C DRP Port Controller Driver
//
// Copyright (C) 2019 Renesas Electronics Corp.
//

pub const HD3SS3220_REG_CN_STAT: c_uint = 0x08;
pub const HD3SS3220_REG_CN_STAT_CTRL: c_uint = 0x09;
pub const HD3SS3220_REG_GEN_CTRL: c_uint = 0x0A;
pub const HD3SS3220_REG_DEV_REV: c_uint = 0xA0;
// Register HD3SS3220_REG_CN_STAT

pub const HD3SS3220_REG_CN_STAT_CURRENT_MODE_DEFAULT: c_uint = 0x00;

// Register HD3SS3220_REG_CN_STAT_CTRL

// Register HD3SS3220_REG_GEN_CTRL

pub const HD3SS3220_REG_GEN_CTRL_SRC_PREF_DRP_DEFAULT: c_uint = 0x00;

pub const HD3SS3220_REG_GEN_CTRL_MODE_SELECT_DEFAULT: c_uint = 0x00;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hd3ss3220 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub role_sw: *mut usb_role_switch,
    pub port: *mut typec_port,
    pub output_poll_work: delayed_work,
    pub role_state: enum usb_role,
    pub poll: bool,
    pub id_gpiod: *mut gpio_desc,
    pub id_irq: c_int,
    pub vbus: *mut regulator,
}

#[no_mangle]
unsafe extern "C" fn hd3ss3220_set_power_opmode(hd3ss3220: *mut hd3ss3220, power_opmode: c_int) -> c_int {
    static int hd3ss3220_set_power_opmode(struct hd3ss3220 *hd3ss3220, int power_opmode)
    {
    int current_mode;
    switch (power_opmode) {
    case TYPEC_PWR_MODE_USB:
    current_mode = HD3SS3220_REG_CN_STAT_CURRENT_MODE_DEFAULT;
    break;
    case TYPEC_PWR_MODE_1_5A:
    current_mode = HD3SS3220_REG_CN_STAT_CURRENT_MODE_MID;
    break;
    case TYPEC_PWR_MODE_3_0A:
    current_mode = HD3SS3220_REG_CN_STAT_CURRENT_MODE_HIGH;
    break;
    case TYPEC_PWR_MODE_PD: /* Power delivery not supported */
    default:
    dev_err(hd3ss3220.dev, "bad power operation mode: %d\n", power_opmode);
    return -EINVAL;
    }
    return regmap_update_bits(hd3ss3220.regmap, HD3SS3220_REG_CN_STAT,
    HD3SS3220_REG_CN_STAT_CURRENT_MODE_MASK,
    current_mode);
    }
#[no_mangle]
unsafe extern "C" fn hd3ss3220_set_port_type(hd3ss3220: *mut hd3ss3220, type: c_int) -> c_int {
    static int hd3ss3220_set_port_type(struct hd3ss3220 *hd3ss3220, int type)
    {
    int mode_select, err;
    switch (type) {
    case TYPEC_PORT_SRC:
    mode_select = HD3SS3220_REG_GEN_CTRL_MODE_SELECT_DFP;
    break;
    case TYPEC_PORT_SNK:
    mode_select = HD3SS3220_REG_GEN_CTRL_MODE_SELECT_UFP;
    break;
    case TYPEC_PORT_DRP:
    mode_select = HD3SS3220_REG_GEN_CTRL_MODE_SELECT_DRP;
    break;
    default:
    dev_err(hd3ss3220.dev, "bad port type: %d\n", type);
    return -EINVAL;
    }
// Disable termination before changing MODE_SELECT as required by datasheet
    err = regmap_update_bits(hd3ss3220.regmap, HD3SS3220_REG_GEN_CTRL,
    HD3SS3220_REG_GEN_CTRL_DISABLE_TERM,
    HD3SS3220_REG_GEN_CTRL_DISABLE_TERM);
    if (err < 0) {
    dev_err(hd3ss3220.dev, "Failed to disable port for mode change: %d\n", err);
    return err;
    }
    err = regmap_update_bits(hd3ss3220.regmap, HD3SS3220_REG_GEN_CTRL,
    HD3SS3220_REG_GEN_CTRL_MODE_SELECT_MASK,
    mode_select);
    if (err < 0) {
    dev_err(hd3ss3220.dev, "Failed to change mode: %d\n", err);
    regmap_update_bits(hd3ss3220.regmap, HD3SS3220_REG_GEN_CTRL,
    HD3SS3220_REG_GEN_CTRL_DISABLE_TERM, 0);
    return err;
    }
    err = regmap_update_bits(hd3ss3220.regmap, HD3SS3220_REG_GEN_CTRL,
    HD3SS3220_REG_GEN_CTRL_DISABLE_TERM, 0);
    if (err < 0)
    dev_err(hd3ss3220.dev, "Failed to re-enable port after mode change: %d\n", err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn hd3ss3220_set_source_pref(hd3ss3220: *mut hd3ss3220, prefer_role: c_int) -> c_int {
    static int hd3ss3220_set_source_pref(struct hd3ss3220 *hd3ss3220, int prefer_role)
    {
    int src_pref;
    switch (prefer_role) {
    case TYPEC_NO_PREFERRED_ROLE:
    src_pref = HD3SS3220_REG_GEN_CTRL_SRC_PREF_DRP_DEFAULT;
    break;
    case TYPEC_SINK:
    src_pref = HD3SS3220_REG_GEN_CTRL_SRC_PREF_DRP_TRY_SNK;
    break;
    case TYPEC_SOURCE:
    src_pref = HD3SS3220_REG_GEN_CTRL_SRC_PREF_DRP_TRY_SRC;
    break;
    default:
    dev_err(hd3ss3220.dev, "bad role preference: %d\n", prefer_role);
    return -EINVAL;
    }
    return regmap_update_bits(hd3ss3220.regmap, HD3SS3220_REG_GEN_CTRL,
    HD3SS3220_REG_GEN_CTRL_SRC_PREF_MASK,
    src_pref);
    }
#[no_mangle]
unsafe extern "C" fn hd3ss3220_get_attached_state(hd3ss3220: *mut hd3ss3220) -> enum usb_role {
    static enum usb_role hd3ss3220_get_attached_state(struct hd3ss3220 *hd3ss3220)
    {
    unsigned int reg_val;
    enum usb_role attached_state;
    int ret;
    ret = regmap_read(hd3ss3220.regmap, HD3SS3220_REG_CN_STAT_CTRL,
    &reg_val);
    if (ret < 0)
    return ret;
    switch (reg_val & HD3SS3220_REG_CN_STAT_CTRL_ATTACHED_STATE_MASK) {
    case HD3SS3220_REG_CN_STAT_CTRL_AS_DFP:
    attached_state = USB_ROLE_HOST;
    break;
    case HD3SS3220_REG_CN_STAT_CTRL_AS_UFP:
    attached_state = USB_ROLE_DEVICE;
    break;
    default:
    attached_state = USB_ROLE_NONE;
    break;
    }
    return attached_state;
    }
#[no_mangle]
unsafe extern "C" fn hd3ss3220_try_role(port: *mut typec_port, role: c_int) -> c_int {
    static int hd3ss3220_try_role(struct typec_port *port, int role)
    {
    struct hd3ss3220 *hd3ss3220 = typec_get_drvdata(port);
    return hd3ss3220_set_source_pref(hd3ss3220, role);
    }
#[no_mangle]
unsafe extern "C" fn hd3ss3220_port_type_set(port: *mut typec_port, type: enum typec_port_type) -> c_int {
    static int hd3ss3220_port_type_set(struct typec_port *port, enum typec_port_type type)
    {
    struct hd3ss3220 *hd3ss3220 = typec_get_drvdata(port);
    return hd3ss3220_set_port_type(hd3ss3220, type);
    }
    static const struct typec_operations hd3ss3220_ops = {
    .try_role = hd3ss3220_try_role,
    .port_type_set = hd3ss3220_port_type_set,
    };
#[no_mangle]
unsafe extern "C" fn hd3ss3220_regulator_control(hd3ss3220: *mut hd3ss3220, on: bool) {
    static void hd3ss3220_regulator_control(struct hd3ss3220 *hd3ss3220, bool on)
    {
    int ret;
    if (regulator_is_enabled(hd3ss3220.vbus) == on)
    return;
    if (on)
    ret = regulator_enable(hd3ss3220.vbus);
    else
    ret = regulator_disable(hd3ss3220.vbus);
    if (ret)
    dev_err(hd3ss3220.dev,
    "vbus regulator %s failed: %d\n", on ? "enable" : "disable", ret);
    }
#[no_mangle]
unsafe extern "C" fn hd3ss3220_set_role(hd3ss3220: *mut hd3ss3220) {
    static void hd3ss3220_set_role(struct hd3ss3220 *hd3ss3220)
    {
    let mut role_state: enum usb_role = hd3ss3220_get_attached_state(hd3ss3220);
    usb_role_switch_set_role(hd3ss3220.role_sw, role_state);
    switch (role_state) {
    case USB_ROLE_HOST:
    typec_set_data_role(hd3ss3220.port, TYPEC_HOST);
    break;
    case USB_ROLE_DEVICE:
    typec_set_data_role(hd3ss3220.port, TYPEC_DEVICE);
    break;
    default:
    break;
    }
    if (hd3ss3220.vbus && !hd3ss3220.id_gpiod)
    hd3ss3220_regulator_control(hd3ss3220, role_state == USB_ROLE_HOST);
    hd3ss3220.role_state = role_state;
    }
#[no_mangle]
unsafe extern "C" fn output_poll_execute(work: *mut work_struct) {
    static void output_poll_execute(struct work_struct *work)
    {
    struct delayed_work *delayed_work = to_delayed_work(work);
    struct hd3ss3220 *hd3ss3220 = container_of(delayed_work,
    struct hd3ss3220,
    output_poll_work);
    let mut role_state: enum usb_role = hd3ss3220_get_attached_state(hd3ss3220);
    if (hd3ss3220.role_state != role_state)
    hd3ss3220_set_role(hd3ss3220);
    schedule_delayed_work(&hd3ss3220.output_poll_work, HZ);
    }
#[no_mangle]
unsafe extern "C" fn hd3ss3220_irq(hd3ss3220: *mut hd3ss3220) -> irqreturn_t {
    static irqreturn_t hd3ss3220_irq(struct hd3ss3220 *hd3ss3220)
    {
    int err;
    hd3ss3220_set_role(hd3ss3220);
    err = regmap_write_bits(hd3ss3220.regmap, HD3SS3220_REG_CN_STAT_CTRL,
    HD3SS3220_REG_CN_STAT_CTRL_INT_STATUS,
    HD3SS3220_REG_CN_STAT_CTRL_INT_STATUS);
    if (err < 0)
    return IRQ_NONE;
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hd3ss3220_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t hd3ss3220_irq_handler(int irq, void *data)
    {
    struct i2c_client *client = to_i2c_client(data);
    struct hd3ss3220 *hd3ss3220 = i2c_get_clientdata(client);
    return hd3ss3220_irq(hd3ss3220);
    }
    static int hd3ss3220_configure_power_opmode(struct hd3ss3220 *hd3ss3220,
    struct fwnode_handle *connector)
    {
//
// Supported power operation mode can be configured through device tree
//
    const char *cap_str;
    int ret, power_opmode;
    ret = fwnode_property_read_string(connector, "typec-power-opmode", &cap_str);
    if (ret)
    return 0;
    power_opmode = typec_find_pwr_opmode(cap_str);
    return hd3ss3220_set_power_opmode(hd3ss3220, power_opmode);
    }
    static int hd3ss3220_configure_port_type(struct hd3ss3220 *hd3ss3220,
    struct fwnode_handle *connector,
    struct typec_capability *cap)
    {
//
// Port type can be configured through device tree
//
    const char *cap_str;
    int ret;
    ret = fwnode_property_read_string(connector, "power-role", &cap_str);
    if (ret)
    return 0;
    ret = typec_find_port_power_role(cap_str);
    if (ret < 0)
    return ret;
    cap.type = ret;
    return hd3ss3220_set_port_type(hd3ss3220, cap.type);
    }
    static int hd3ss3220_configure_source_pref(struct hd3ss3220 *hd3ss3220,
    struct fwnode_handle *connector,
    struct typec_capability *cap)
    {
//
// Preferred role can be configured through device tree
//
    const char *cap_str;
    int ret;
    ret = fwnode_property_read_string(connector, "try-power-role", &cap_str);
    if (ret)
    return 0;
    ret = typec_find_power_role(cap_str);
    if (ret < 0)
    return ret;
    cap.prefer_role = ret;
    return hd3ss3220_set_source_pref(hd3ss3220, cap.prefer_role);
    }
    static const struct regmap_config config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x0A,
    };
#[no_mangle]
unsafe extern "C" fn hd3ss3220_id_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t hd3ss3220_id_isr(int irq, void *dev_id)
    {
    struct hd3ss3220 *hd3ss3220 = dev_id;
    int id;
    id = gpiod_get_value_cansleep(hd3ss3220.id_gpiod);
    hd3ss3220_regulator_control(hd3ss3220, !id);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hd3ss3220_probe(client: *mut i2c_client) -> c_int {
    static int hd3ss3220_probe(struct i2c_client *client)
    {
    let mut typec_cap: typec_capability = { };
    struct fwnode_handle *connector, *ep;
    struct hd3ss3220 *hd3ss3220;
    struct regulator *vbus;
    unsigned int data;
    int ret;
    hd3ss3220 = devm_kzalloc(&client.dev, sizeof(struct hd3ss3220),
    GFP_KERNEL);
    if (!hd3ss3220)
    return -ENOMEM;
    i2c_set_clientdata(client, hd3ss3220);
    hd3ss3220.dev = &client.dev;
    hd3ss3220.regmap = devm_regmap_init_i2c(client, &config);
    if (IS_ERR(hd3ss3220.regmap))
    return PTR_ERR(hd3ss3220.regmap);
// For backward compatibility check the connector child node first
    connector = device_get_named_child_node(hd3ss3220.dev, "connector");
    if (connector) {
    hd3ss3220.role_sw = fwnode_usb_role_switch_get(connector);
    } else {
    ep = fwnode_graph_get_next_endpoint(dev_fwnode(hd3ss3220.dev), core::ptr::null_mut());
    if (!ep)
    return -ENODEV;
    connector = fwnode_graph_get_remote_port_parent(ep);
    fwnode_handle_put(ep);
    if (!connector)
    return -ENODEV;
    hd3ss3220.role_sw = usb_role_switch_get(hd3ss3220.dev);
    }
    if (IS_ERR(hd3ss3220.role_sw)) {
    ret = PTR_ERR(hd3ss3220.role_sw);
    goto err_put_fwnode;
    }
    vbus = devm_of_regulator_get_optional(hd3ss3220.dev,
    to_of_node(connector),
    "vbus");
    if (IS_ERR(vbus) && vbus != ERR_PTR(-ENODEV)) {
    ret = PTR_ERR(vbus);
    dev_err(hd3ss3220.dev, "failed to get vbus: %d", ret);
    goto err_put_fwnode;
    }
    hd3ss3220.vbus = (vbus == ERR_PTR(-ENODEV) ? core::ptr::null_mut() : vbus);
    if (hd3ss3220.vbus) {
    hd3ss3220.id_gpiod = devm_gpiod_get_optional(hd3ss3220.dev,
    "id",
    GPIOD_IN);
    if (IS_ERR(hd3ss3220.id_gpiod)) {
    ret = PTR_ERR(hd3ss3220.id_gpiod);
    goto err_put_fwnode;
    }
    }
    if (hd3ss3220.id_gpiod) {
    hd3ss3220.id_irq = gpiod_to_irq(hd3ss3220.id_gpiod);
    if (hd3ss3220.id_irq < 0) {
    ret = hd3ss3220.id_irq;
    dev_err(hd3ss3220.dev,
    "failed to get ID gpio: %d\n",
    hd3ss3220.id_irq);
    goto err_put_fwnode;
    }
    ret = devm_request_threaded_irq(hd3ss3220.dev,
    hd3ss3220.id_irq, core::ptr::null_mut(),
    hd3ss3220_id_isr,
    IRQF_TRIGGER_RISING |
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    dev_name(hd3ss3220.dev), hd3ss3220);
    if (ret < 0) {
    dev_err(hd3ss3220.dev, "failed to get ID irq: %d\n", ret);
    goto err_put_fwnode;
    }
    }
    typec_cap.prefer_role = TYPEC_NO_PREFERRED_ROLE;
    typec_cap.driver_data = hd3ss3220;
    typec_cap.type = TYPEC_PORT_DRP;
    typec_cap.data = TYPEC_PORT_DRD;
    typec_cap.ops = &hd3ss3220_ops;
    typec_cap.fwnode = connector;
    ret = hd3ss3220_configure_source_pref(hd3ss3220, connector, &typec_cap);
    if (ret < 0)
    goto err_put_role;
    ret = hd3ss3220_configure_port_type(hd3ss3220, connector, &typec_cap);
    if (ret < 0)
    goto err_put_role;
    hd3ss3220.port = typec_register_port(&client.dev, &typec_cap);
    if (IS_ERR(hd3ss3220.port)) {
    ret = PTR_ERR(hd3ss3220.port);
    goto err_put_role;
    }
    ret = hd3ss3220_configure_power_opmode(hd3ss3220, connector);
    if (ret < 0)
    goto err_unreg_port;
    hd3ss3220_set_role(hd3ss3220);
    ret = regmap_read(hd3ss3220.regmap, HD3SS3220_REG_CN_STAT_CTRL, &data);
    if (ret < 0)
    goto err_unreg_port;
    if (data & HD3SS3220_REG_CN_STAT_CTRL_INT_STATUS) {
    ret = regmap_write(hd3ss3220.regmap,
    HD3SS3220_REG_CN_STAT_CTRL,
    data | HD3SS3220_REG_CN_STAT_CTRL_INT_STATUS);
    if (ret < 0)
    goto err_unreg_port;
    }
    if (client.irq > 0) {
    ret = devm_request_threaded_irq(&client.dev, client.irq, core::ptr::null_mut(),
    hd3ss3220_irq_handler,
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    "hd3ss3220", &client.dev);
    if (ret)
    goto err_unreg_port;
    } else {
    INIT_DELAYED_WORK(&hd3ss3220.output_poll_work, output_poll_execute);
    hd3ss3220.poll = true;
    }
    ret = i2c_smbus_read_byte_data(client, HD3SS3220_REG_DEV_REV);
    if (ret < 0)
    goto err_unreg_port;
    fwnode_handle_put(connector);
    if (hd3ss3220.poll)
    schedule_delayed_work(&hd3ss3220.output_poll_work, HZ);
    dev_info(&client.dev, "probed revision=0x%x\n", ret);
    return 0;
    err_unreg_port:
    typec_unregister_port(hd3ss3220.port);
    err_put_role:
    usb_role_switch_put(hd3ss3220.role_sw);
    err_put_fwnode:
    fwnode_handle_put(connector);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hd3ss3220_remove(client: *mut i2c_client) {
    static void hd3ss3220_remove(struct i2c_client *client)
    {
    struct hd3ss3220 *hd3ss3220 = i2c_get_clientdata(client);
    if (hd3ss3220.poll)
    cancel_delayed_work_sync(&hd3ss3220.output_poll_work);
    typec_unregister_port(hd3ss3220.port);
    usb_role_switch_put(hd3ss3220.role_sw);
    }
    static const struct of_device_id dev_ids[] = {
    { .compatible = "ti,hd3ss3220"},
    {}
    };
    MODULE_DEVICE_TABLE(of, dev_ids);
    static struct i2c_driver hd3ss3220_driver = {
    .driver = {
    .name = "hd3ss3220",
    .of_match_table = dev_ids,
    },
    .probe = hd3ss3220_probe,
    .remove = hd3ss3220_remove,
    };
    module_i2c_driver(hd3ss3220_driver);
    MODULE_AUTHOR("Biju Das <biju.das@bp.renesas.com>");
    MODULE_DESCRIPTION("TI HD3SS3220 DRP Port Controller Driver");
    MODULE_LICENSE("GPL");

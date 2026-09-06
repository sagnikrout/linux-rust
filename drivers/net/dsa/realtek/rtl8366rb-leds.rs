//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/realtek/rtl8366rb-leds.c
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

#[no_mangle]
pub unsafe extern "C" fn rtl8366rb_led_group_port_mask(led_group: u8, port: u8) -> u32 {
    static inline u32 rtl8366rb_led_group_port_mask(u8 led_group, u8 port)
    {
    switch (led_group) {
    case 0:
    return FIELD_PREP(RTL8366RB_LED_0_X_CTRL_MASK, BIT(port));
    case 1:
    return FIELD_PREP(RTL8366RB_LED_X_1_CTRL_MASK, BIT(port));
    case 2:
    return FIELD_PREP(RTL8366RB_LED_2_X_CTRL_MASK, BIT(port));
    case 3:
    return FIELD_PREP(RTL8366RB_LED_X_3_CTRL_MASK, BIT(port));
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn rb8366rb_get_port_led(led: *mut rtl8366rb_led) -> c_int {
    static int rb8366rb_get_port_led(struct rtl8366rb_led *led)
    {
    struct realtek_priv *priv = led.priv;
    let mut led_group: u8 = led.led_group;
    let mut port_num: u8 = led.port_num;
    int ret;
    u32 val;
    ret = regmap_read(priv.map, RTL8366RB_LED_X_X_CTRL_REG(led_group),
    &val);
    if (ret) {
    dev_err(priv.dev, "error reading LED on port %d group %d\n",
    led_group, port_num);
    return ret;
    }
    return !!(val & rtl8366rb_led_group_port_mask(led_group, port_num));
    }
#[no_mangle]
unsafe extern "C" fn rb8366rb_set_port_led(led: *mut rtl8366rb_led, enable: bool) -> c_int {
    static int rb8366rb_set_port_led(struct rtl8366rb_led *led, bool enable)
    {
    struct realtek_priv *priv = led.priv;
    let mut led_group: u8 = led.led_group;
    let mut port_num: u8 = led.port_num;
    int ret;
    ret = regmap_update_bits(priv.map,
    RTL8366RB_LED_X_X_CTRL_REG(led_group),
    rtl8366rb_led_group_port_mask(led_group,
    port_num),
    enable ? 0xffff : 0);
    if (ret) {
    dev_err(priv.dev, "error updating LED on port %d group %d\n",
    led_group, port_num);
    return ret;
    }
// Change the LED group to manual controlled LEDs if required
    ret = rb8366rb_set_ledgroup_mode(priv, led_group,
    RTL8366RB_LEDGROUP_FORCE);
    if (ret) {
    dev_err(priv.dev, "error updating LED GROUP group %d\n",
    led_group);
    return ret;
    }
    return 0;
    }
    static int
    rtl8366rb_cled_brightness_set_blocking(struct led_classdev *ldev,
    enum led_brightness brightness)
    {
    struct rtl8366rb_led *led = container_of(ldev, struct rtl8366rb_led,
    cdev);
    return rb8366rb_set_port_led(led, brightness == LED_ON);
    }
    static int rtl8366rb_setup_led(struct realtek_priv *priv, struct dsa_port *dp,
    struct fwnode_handle *led_fwnode)
    {
    struct rtl8366rb *rb = priv.chip_data;
    let mut init_data: led_init_data = { };
    enum led_default_state state;
    struct rtl8366rb_led *led;
    char name[64];
    u32 led_group;
    int ret;
    ret = fwnode_property_read_u32(led_fwnode, "reg", &led_group);
    if (ret)
    return ret;
    if (led_group >= RTL8366RB_NUM_LEDGROUPS) {
    dev_warn(priv.dev, "Invalid LED reg %d defined for port %d",
    led_group, dp.index);
    return -EINVAL;
    }
    led = &rb.leds[dp.index][led_group];
    led.port_num = dp.index;
    led.led_group = led_group;
    led.priv = priv;
    state = led_init_default_state_get(led_fwnode);
    switch (state) {
    case LEDS_DEFSTATE_ON:
    led.cdev.brightness = 1;
    rb8366rb_set_port_led(led, 1);
    break;
    case LEDS_DEFSTATE_KEEP:
    led.cdev.brightness =
    rb8366rb_get_port_led(led);
    break;
    case LEDS_DEFSTATE_OFF:
    default:
    led.cdev.brightness = 0;
    rb8366rb_set_port_led(led, 0);
    }
    led.cdev.max_brightness = 1;
    led.cdev.brightness_set_blocking =
    rtl8366rb_cled_brightness_set_blocking;
    init_data.fwnode = led_fwnode;
    init_data.devname_mandatory = true;
    snprintf(name, sizeof(name), "Realtek-%d:0%d:%d",
    dp.ds.index, dp.index, led_group);
    init_data.devicename = name;
    ret = devm_led_classdev_register_ext(priv.dev, &led.cdev, &init_data);
    if (ret) {
    dev_warn(priv.dev, "Failed to init LED %d for port %d",
    led_group, dp.index);
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rtl8366rb_setup_leds(priv: *mut realtek_priv) -> c_int {
    int rtl8366rb_setup_leds(struct realtek_priv *priv)
    {
    struct dsa_switch *ds = &priv.ds;
    struct device_node *leds_np;
    struct dsa_port *dp;
    let mut ret: c_int = 0;
    dsa_switch_for_each_port(dp, ds) {
    if (!dp.dn)
    continue;
    leds_np = of_get_child_by_name(dp.dn, "leds");
    if (!leds_np) {
    dev_dbg(priv.dev, "No leds defined for port %d",
    dp.index);
    continue;
    }
    for_each_child_of_node_scoped(leds_np, led_np) {
    ret = rtl8366rb_setup_led(priv, dp,
    of_fwnode_handle(led_np));
    if (ret)
    break;
    }
    of_node_put(leds_np);
    if (ret)
    return ret;
    }
    return 0;
    }

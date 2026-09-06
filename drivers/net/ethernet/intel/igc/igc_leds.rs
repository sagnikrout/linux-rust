//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/intel/igc/igc_leds.c
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
// Copyright (C) 2024 Linutronix GmbH

pub const IGC_NUM_LEDS: c_int = 3;
pub const IGC_LEDCTL_LED0_MODE_SHIFT: c_int = 0;

pub const IGC_LEDCTL_LED1_MODE_SHIFT: c_int = 8;

pub const IGC_LEDCTL_LED2_MODE_SHIFT: c_int = 16;

pub const IGC_LEDCTL_MODE_ON: c_uint = 0x00;
pub const IGC_LEDCTL_MODE_OFF: c_uint = 0x01;
pub const IGC_LEDCTL_MODE_LINK_10: c_uint = 0x05;
pub const IGC_LEDCTL_MODE_LINK_100: c_uint = 0x06;
pub const IGC_LEDCTL_MODE_LINK_1000: c_uint = 0x07;
pub const IGC_LEDCTL_MODE_LINK_2500: c_uint = 0x08;
pub const IGC_LEDCTL_MODE_ACTIVITY: c_uint = 0x0b;

    (BIT(TRIGGER_NETDEV_LINK_2500) | BIT(TRIGGER_NETDEV_LINK_1000) | \
    BIT(TRIGGER_NETDEV_LINK_100) | BIT(TRIGGER_NETDEV_LINK_10) |	 \
    BIT(TRIGGER_NETDEV_RX) | BIT(TRIGGER_NETDEV_TX))

    (BIT(TRIGGER_NETDEV_RX) | BIT(TRIGGER_NETDEV_TX))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_led_classdev {
    pub netdev: *mut net_device,
    pub led: led_classdev,
    pub index: c_int,
}

    container_of(lcdev, struct igc_led_classdev, led)
    static void igc_led_select(struct igc_adapter *adapter, int led,
    u32 *mask, u32 *shift, u32 *blink)
    {
    switch (led) {
    case 0:
// mask  = IGC_LEDCTL_LED0_MODE_MASK;
// shift = IGC_LEDCTL_LED0_MODE_SHIFT;
// blink = IGC_LEDCTL_LED0_BLINK;
    break;
    case 1:
// mask  = IGC_LEDCTL_LED1_MODE_MASK;
// shift = IGC_LEDCTL_LED1_MODE_SHIFT;
// blink = IGC_LEDCTL_LED1_BLINK;
    break;
    case 2:
// mask  = IGC_LEDCTL_LED2_MODE_MASK;
// shift = IGC_LEDCTL_LED2_MODE_SHIFT;
// blink = IGC_LEDCTL_LED2_BLINK;
    break;
    default:
// mask = *shift = *blink = 0;
    netdev_err(adapter.netdev, "Unknown LED %d selected!\n", led);
    }
    }
    static void igc_led_set(struct igc_adapter *adapter, int led, u32 mode,
    bool blink)
    {
    u32 shift, mask, blink_bit, ledctl;
    struct igc_hw *hw = &adapter.hw;
    igc_led_select(adapter, led, &mask, &shift, &blink_bit);
    pm_runtime_get_sync(&adapter.pdev.dev);
    mutex_lock(&adapter.led_mutex);
// Set mode
    ledctl = rd32(IGC_LEDCTL);
    ledctl &= ~mask;
    ledctl |= mode << shift;
// Configure blinking
    if (blink)
    ledctl |= blink_bit;
    else
    ledctl &= ~blink_bit;
    wr32(IGC_LEDCTL, ledctl);
    mutex_unlock(&adapter.led_mutex);
    pm_runtime_put(&adapter.pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn igc_led_get(adapter: *mut igc_adapter, led: c_int) -> u32 {
    static u32 igc_led_get(struct igc_adapter *adapter, int led)
    {
    u32 shift, mask, blink_bit, ledctl;
    struct igc_hw *hw = &adapter.hw;
    igc_led_select(adapter, led, &mask, &shift, &blink_bit);
    pm_runtime_get_sync(&adapter.pdev.dev);
    mutex_lock(&adapter.led_mutex);
    ledctl = rd32(IGC_LEDCTL);
    mutex_unlock(&adapter.led_mutex);
    pm_runtime_put(&adapter.pdev.dev);
    return (ledctl & mask) >> shift;
    }
    static int igc_led_brightness_set_blocking(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct igc_led_classdev *ldev = lcdev_to_igc_ldev(led_cdev);
    struct igc_adapter *adapter = netdev_priv(ldev.netdev);
    u32 mode;
    if (brightness)
    mode = IGC_LEDCTL_MODE_ON;
    else
    mode = IGC_LEDCTL_MODE_OFF;
    netdev_dbg(adapter.netdev, "Set brightness for LED %d to mode %u!\n",
    ldev.index, mode);
    igc_led_set(adapter, ldev.index, mode, false);
    return 0;
    }
    static int igc_led_hw_control_is_supported(struct led_classdev *led_cdev,
    unsigned long flags)
    {
    if (flags & ~IGC_SUPPORTED_MODES)
    return -EOPNOTSUPP;
// If Tx and Rx selected, activity can be offloaded unless some other
// mode is selected as well.
//
    if ((flags & BIT(TRIGGER_NETDEV_TX)) &&
    (flags & BIT(TRIGGER_NETDEV_RX)) &&
    !(flags & ~IGC_ACTIVITY_MODES))
    return 0;
// Single Rx or Tx activity is not supported.
    if (flags & IGC_ACTIVITY_MODES)
    return -EOPNOTSUPP;
// Only one mode can be active at a given time.
    if (flags & (flags - 1))
    return -EOPNOTSUPP;
    return 0;
    }
    static int igc_led_hw_control_set(struct led_classdev *led_cdev,
    unsigned long flags)
    {
    struct igc_led_classdev *ldev = lcdev_to_igc_ldev(led_cdev);
    struct igc_adapter *adapter = netdev_priv(ldev.netdev);
    let mut mode: u32 = IGC_LEDCTL_MODE_OFF;
    let mut blink: bool = false;
    if (flags & BIT(TRIGGER_NETDEV_LINK_10))
    mode = IGC_LEDCTL_MODE_LINK_10;
    if (flags & BIT(TRIGGER_NETDEV_LINK_100))
    mode = IGC_LEDCTL_MODE_LINK_100;
    if (flags & BIT(TRIGGER_NETDEV_LINK_1000))
    mode = IGC_LEDCTL_MODE_LINK_1000;
    if (flags & BIT(TRIGGER_NETDEV_LINK_2500))
    mode = IGC_LEDCTL_MODE_LINK_2500;
    if ((flags & BIT(TRIGGER_NETDEV_TX)) &&
    (flags & BIT(TRIGGER_NETDEV_RX)))
    mode = IGC_LEDCTL_MODE_ACTIVITY;
    netdev_dbg(adapter.netdev, "Set HW control for LED %d to mode %u!\n",
    ldev.index, mode);
// blink is recommended for activity
    if (mode == IGC_LEDCTL_MODE_ACTIVITY)
    blink = true;
    igc_led_set(adapter, ldev.index, mode, blink);
    return 0;
    }
    static int igc_led_hw_control_get(struct led_classdev *led_cdev,
    unsigned long *flags)
    {
    struct igc_led_classdev *ldev = lcdev_to_igc_ldev(led_cdev);
    struct igc_adapter *adapter = netdev_priv(ldev.netdev);
    u32 mode;
    mode = igc_led_get(adapter, ldev.index);
    switch (mode) {
    case IGC_LEDCTL_MODE_ACTIVITY:
// flags = BIT(TRIGGER_NETDEV_TX) | BIT(TRIGGER_NETDEV_RX);
    break;
    case IGC_LEDCTL_MODE_LINK_10:
// flags = BIT(TRIGGER_NETDEV_LINK_10);
    break;
    case IGC_LEDCTL_MODE_LINK_100:
// flags = BIT(TRIGGER_NETDEV_LINK_100);
    break;
    case IGC_LEDCTL_MODE_LINK_1000:
// flags = BIT(TRIGGER_NETDEV_LINK_1000);
    break;
    case IGC_LEDCTL_MODE_LINK_2500:
// flags = BIT(TRIGGER_NETDEV_LINK_2500);
    break;
    }
    return 0;
    }
    static struct device *igc_led_hw_control_get_device(struct led_classdev *led_cdev)
    {
    struct igc_led_classdev *ldev = lcdev_to_igc_ldev(led_cdev);
    return &ldev.netdev.dev;
    }
    static void igc_led_get_name(struct igc_adapter *adapter, int index, char *buf,
    size_t buf_len)
    {
    snprintf(buf, buf_len, "igc-%x%x-led%d",
    pci_domain_nr(adapter.pdev.bus),
    pci_dev_id(adapter.pdev), index);
    }
    static int igc_setup_ldev(struct igc_led_classdev *ldev,
    struct net_device *netdev, int index)
    {
    struct igc_adapter *adapter = netdev_priv(netdev);
    struct led_classdev *led_cdev = &ldev.led;
    char led_name[LED_MAX_NAME_SIZE];
    ldev.netdev = netdev;
    ldev.index = index;
    igc_led_get_name(adapter, index, led_name, LED_MAX_NAME_SIZE);
    led_cdev.name = led_name;
    led_cdev.flags |= LED_RETAIN_AT_SHUTDOWN;
    led_cdev.max_brightness = 1;
    led_cdev.brightness_set_blocking = igc_led_brightness_set_blocking;
    led_cdev.hw_control_trigger = "netdev";
    led_cdev.hw_control_is_supported = igc_led_hw_control_is_supported;
    led_cdev.hw_control_set = igc_led_hw_control_set;
    led_cdev.hw_control_get = igc_led_hw_control_get;
    led_cdev.hw_control_get_device = igc_led_hw_control_get_device;
    return led_classdev_register(&netdev.dev, led_cdev);
    }
#[no_mangle]
pub unsafe extern "C" fn igc_led_setup(adapter: *mut igc_adapter) -> c_int {
    int igc_led_setup(struct igc_adapter *adapter)
    {
    struct net_device *netdev = adapter.netdev;
    struct igc_led_classdev *leds;
    int i, err;
    mutex_init(&adapter.led_mutex);
    leds = kzalloc_objs(*leds, IGC_NUM_LEDS);
    if (!leds)
    return -ENOMEM;
    for (i = 0; i < IGC_NUM_LEDS; i++) {
    err = igc_setup_ldev(leds + i, netdev, i);
    if (err)
    goto err;
    }
    adapter.leds = leds;
    return 0;
    err:
    for (i--; i >= 0; i--)
    led_classdev_unregister(&((leds + i).led));
    kfree(leds);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn igc_led_free(adapter: *mut igc_adapter) {
    void igc_led_free(struct igc_adapter *adapter)
    {
    struct igc_led_classdev *leds = adapter.leds;
    int i;
    for (i = 0; i < IGC_NUM_LEDS; i++)
    led_classdev_unregister(&((leds + i).led));
    kfree(leds);
    }

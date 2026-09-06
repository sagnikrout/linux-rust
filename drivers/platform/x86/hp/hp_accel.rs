//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/hp/hp_accel.c
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
// hp_accel.c - Interface between LIS3LV02DL driver and HP ACPI BIOS
//
// Copyright (C) 2007-2008 Yan Burman
// Copyright (C) 2008 Eric Piel
// Copyright (C) 2008-2009 Pavel Machek
//

// Delayed LEDs infrastructure ------------------------------------
// Special LED class that can defer work
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delayed_led_classdev {
    pub led_classdev: led_classdev,
    pub work: work_struct,
    pub new_brightness: enum led_brightness,
    pub /: *mut *mut unsigned int led; / For driver,
    pub value): *mut *mut *mut void (set_brightness)(struct delayed_led_classdev data, enum led_brightness,
}

#[no_mangle]
pub unsafe extern "C" fn delayed_set_status_worker(work: *mut work_struct) {
    static inline void delayed_set_status_worker(struct work_struct *work)
    {
    struct delayed_led_classdev *data =
    container_of(work, struct delayed_led_classdev, work);
    data.set_brightness(data, data.new_brightness);
    }
    static inline void delayed_sysfs_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct delayed_led_classdev *data = container_of(led_cdev,
    struct delayed_led_classdev, led_classdev);
    data.new_brightness = brightness;
    schedule_work(&data.work);
    }
// HP-specific accelerometer driver ------------------------------------
// e0 25, e0 26, e0 27, e0 28 are scan codes that the accelerometer with acpi id
// HPQ6000 sends through the keyboard bus
pub const ACCEL_1: c_uint = 0x25;
pub const ACCEL_2: c_uint = 0x26;
pub const ACCEL_3: c_uint = 0x27;
pub const ACCEL_4: c_uint = 0x28;
// For automatic insertion of the module
    static const struct acpi_device_id lis3lv02d_device_ids[] = {
    {"HPQ0004", 0}, /* HP Mobile Data Protection System PNP */
    {"HPQ6000", 0}, /* HP Mobile Data Protection System PNP */
    {"HPQ6007", 0}, /* HP Mobile Data Protection System PNP */
    {"", 0},
    };
    MODULE_DEVICE_TABLE(acpi, lis3lv02d_device_ids);
//
// lis3lv02d_acpi_init - initialize the device for ACPI
// @lis3: pointer to the device struct
//
// Returns 0 on success.
//
#[no_mangle]
unsafe extern "C" fn lis3lv02d_acpi_init(lis3: *mut lis3lv02d) -> c_int {
    static int lis3lv02d_acpi_init(struct lis3lv02d *lis3)
    {
    return 0;
    }
//
// lis3lv02d_acpi_read - ACPI ALRD method: read a register
// @lis3: pointer to the device struct
// @reg:    the register to read
// @ret:    result of the operation
//
// Returns 0 on success.
//
#[no_mangle]
unsafe extern "C" fn lis3lv02d_acpi_read(lis3: *mut lis3lv02d, reg: c_int, ret: *mut u8) -> c_int {
    static int lis3lv02d_acpi_read(struct lis3lv02d *lis3, int reg, u8 *ret)
    {
    struct acpi_device *dev = lis3.bus_priv;
    let mut arg0: union acpi_object = { ACPI_TYPE_INTEGER };
    let mut args: acpi_object_list = { 1, &arg0 };
    unsigned long long lret;
    acpi_status status;
    arg0.integer.value = reg;
    status = acpi_evaluate_integer(dev.handle, "ALRD", &args, &lret);
    if (ACPI_FAILURE(status))
    return -EINVAL;
// ret = lret;
    return 0;
    }
//
// lis3lv02d_acpi_write - ACPI ALWR method: write to a register
// @lis3: pointer to the device struct
// @reg:    the register to write to
// @val:    the value to write
//
// Returns 0 on success.
//
#[no_mangle]
unsafe extern "C" fn lis3lv02d_acpi_write(lis3: *mut lis3lv02d, reg: c_int, val: u8) -> c_int {
    static int lis3lv02d_acpi_write(struct lis3lv02d *lis3, int reg, u8 val)
    {
    struct acpi_device *dev = lis3.bus_priv;
    unsigned long long ret; /* Not used when writting */
    union acpi_object in_obj[2];
    let mut args: acpi_object_list = { 2, in_obj };
    in_obj[0].type          = ACPI_TYPE_INTEGER;
    in_obj[0].integer.value = reg;
    in_obj[1].type          = ACPI_TYPE_INTEGER;
    in_obj[1].integer.value = val;
    if (acpi_evaluate_integer(dev.handle, "ALWR", &args, &ret) != AE_OK)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lis3lv02d_dmi_matched(dmi: *const dmi_system_id) -> c_int {
    static int lis3lv02d_dmi_matched(const struct dmi_system_id *dmi)
    {
    lis3_dev.ac = *((union axis_conversion *)dmi.driver_data);
    pr_info("hardware type %s found\n", dmi.ident);
    return 1;
    }
// Represents, for each axis seen by userspace, the corresponding hw axis (+1).
// If the value is negative, the opposite of the hw value is used.

    static union axis_conversion lis3lv02d_axis_##name = \
    { .as_array = { x, y, z } }
    DEFINE_CONV(normal, 1, 2, 3);
    DEFINE_CONV(y_inverted, 1, -2, 3);
    DEFINE_CONV(x_inverted, -1, 2, 3);
    DEFINE_CONV(x_inverted_usd, -1, 2, -3);
    DEFINE_CONV(z_inverted, 1, 2, -3);
    DEFINE_CONV(xy_swap, 2, 1, 3);
    DEFINE_CONV(xy_rotated_left, -2, 1, 3);
    DEFINE_CONV(xy_rotated_left_usd, -2, 1, -3);
    DEFINE_CONV(xy_swap_inverted, -2, -1, 3);
    DEFINE_CONV(xy_rotated_right, 2, -1, 3);
    DEFINE_CONV(xy_swap_yz_inverted, 2, -1, -3);

    .ident = _ident,				\
    .callback = lis3lv02d_dmi_matched,		\
    .matches = {					\
    DMI_MATCH(DMI_PRODUCT_NAME, _name)	\
    },						\
    .driver_data = &lis3lv02d_axis_##_axis		\
    }

    _class2, _name2,	\
    _axis) {		\
    .ident = _ident,				\
    .callback = lis3lv02d_dmi_matched,		\
    .matches = {					\
    DMI_MATCH(DMI_##_class1, _name1),	\
    DMI_MATCH(DMI_##_class2, _name2),	\
    },						\
    .driver_data = &lis3lv02d_axis_##_axis		\
    }
    static const struct dmi_system_id lis3lv02d_dmi_ids[] = {
// product names are truncated to match all kinds of a same model
    AXIS_DMI_MATCH("NC64x0", "HP Compaq nc64", x_inverted),
    AXIS_DMI_MATCH("NC84x0", "HP Compaq nc84", z_inverted),
    AXIS_DMI_MATCH("NX9420", "HP Compaq nx9420", x_inverted),
    AXIS_DMI_MATCH("NW9440", "HP Compaq nw9440", x_inverted),
    AXIS_DMI_MATCH("NC2510", "HP Compaq 2510", y_inverted),
    AXIS_DMI_MATCH("NC2710", "HP Compaq 2710", xy_swap),
    AXIS_DMI_MATCH("NC8510", "HP Compaq 8510", xy_swap_inverted),
    AXIS_DMI_MATCH("HP2133", "HP 2133", xy_rotated_left),
    AXIS_DMI_MATCH("HP2140", "HP 2140", xy_swap_inverted),
    AXIS_DMI_MATCH("NC653x", "HP Compaq 653", xy_rotated_left_usd),
    AXIS_DMI_MATCH("NC6730b", "HP Compaq 6730b", xy_rotated_left_usd),
    AXIS_DMI_MATCH("NC6730s", "HP Compaq 6730s", xy_swap),
    AXIS_DMI_MATCH("NC651xx", "HP Compaq 651", xy_rotated_right),
    AXIS_DMI_MATCH("NC6710x", "HP Compaq 6710", xy_swap_yz_inverted),
    AXIS_DMI_MATCH("NC6715x", "HP Compaq 6715", y_inverted),
    AXIS_DMI_MATCH("NC693xx", "HP EliteBook 693", xy_rotated_right),
    AXIS_DMI_MATCH("NC693xx", "HP EliteBook 853", xy_swap),
    AXIS_DMI_MATCH("NC854xx", "HP EliteBook 854", y_inverted),
    AXIS_DMI_MATCH("NC273xx", "HP EliteBook 273", y_inverted),
// Intel-based HP Pavilion dv5
    AXIS_DMI_MATCH2("HPDV5_I",
    PRODUCT_NAME, "HP Pavilion dv5",
    BOARD_NAME, "3603",
    x_inverted),
// AMD-based HP Pavilion dv5
    AXIS_DMI_MATCH2("HPDV5_A",
    PRODUCT_NAME, "HP Pavilion dv5",
    BOARD_NAME, "3600",
    y_inverted),
    AXIS_DMI_MATCH("DV7", "HP Pavilion dv7", x_inverted),
    AXIS_DMI_MATCH("HP8710", "HP Compaq 8710", y_inverted),
    AXIS_DMI_MATCH("HDX18", "HP HDX 18", x_inverted),
    AXIS_DMI_MATCH("HPB432x", "HP ProBook 432", xy_rotated_left),
    AXIS_DMI_MATCH("HPB440G3", "HP ProBook 440 G3", x_inverted_usd),
    AXIS_DMI_MATCH("HPB440G4", "HP ProBook 440 G4", x_inverted),
    AXIS_DMI_MATCH("HPB442x", "HP ProBook 442", xy_rotated_left),
    AXIS_DMI_MATCH("HPB450G0", "HP ProBook 450 G0", x_inverted),
    AXIS_DMI_MATCH("HPB452x", "HP ProBook 452", y_inverted),
    AXIS_DMI_MATCH("HPB522x", "HP ProBook 522", xy_swap),
    AXIS_DMI_MATCH("HPB532x", "HP ProBook 532", y_inverted),
    AXIS_DMI_MATCH("HPB655x", "HP ProBook 655", xy_swap_inverted),
    AXIS_DMI_MATCH("Mini510x", "HP Mini 510", xy_rotated_left_usd),
    AXIS_DMI_MATCH("HPB63xx", "HP ProBook 63", xy_swap),
    AXIS_DMI_MATCH("HPB64xx", "HP ProBook 64", xy_swap),
    AXIS_DMI_MATCH("HPB64xx", "HP EliteBook 84", xy_swap),
    AXIS_DMI_MATCH("HPB65xx", "HP ProBook 65", x_inverted),
    AXIS_DMI_MATCH("HPZBook15", "HP ZBook 15", x_inverted),
    AXIS_DMI_MATCH("HPZBook17G5", "HP ZBook 17 G5", x_inverted),
    AXIS_DMI_MATCH("HPZBook17", "HP ZBook 17", xy_swap_yz_inverted),
    { core::ptr::null_mut(), }
// Laptop models without axis info (yet):
// "NC6910" "HP Compaq 6910"
// "NC2400" "HP Compaq nc2400"
// "NX74x0" "HP Compaq nx74"
// "NX6325" "HP Compaq nx6325"
// "NC4400" "HP Compaq nc4400"
//
    };
#[no_mangle]
unsafe extern "C" fn hpled_set(led_cdev: *mut delayed_led_classdev, value: enum led_brightness) {
    static void hpled_set(struct delayed_led_classdev *led_cdev, enum led_brightness value)
    {
    struct acpi_device *dev = lis3_dev.bus_priv;
    unsigned long long ret; /* Not used when writing */
    union acpi_object in_obj[1];
    let mut args: acpi_object_list = { 1, in_obj };
    in_obj[0].type          = ACPI_TYPE_INTEGER;
    in_obj[0].integer.value = !!value;
    acpi_evaluate_integer(dev.handle, "ALED", &args, &ret);
    }
    static struct delayed_led_classdev hpled_led = {
    .led_classdev = {
    .name			= "hp::hddprotect",
    .default_trigger	= "none",
    .brightness_set		= delayed_sysfs_set,
    .flags                  = LED_CORE_SUSPENDRESUME,
    },
    .set_brightness = hpled_set,
    };
    static bool hp_accel_i8042_filter(unsigned char data, unsigned char str,
    struct serio *port, void *context)
    {
    static bool extended;
    if (str & I8042_STR_AUXDATA)
    return false;
    if (data == 0xe0) {
    extended = true;
    return true;
    } else if (unlikely(extended)) {
    extended = false;
    switch (data) {
    case ACCEL_1:
    case ACCEL_2:
    case ACCEL_3:
    case ACCEL_4:
    return true;
    default:
    serio_interrupt(port, 0xe0, 0);
    return false;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn lis3lv02d_probe(device: *mut platform_device) -> c_int {
    static int lis3lv02d_probe(struct platform_device *device)
    {
    int ret;
    lis3_dev.bus_priv = ACPI_COMPANION(&device.dev);
    if (!lis3_dev.bus_priv)
    return -ENODEV;
    lis3_dev.init = lis3lv02d_acpi_init;
    lis3_dev.read = lis3lv02d_acpi_read;
    lis3_dev.write = lis3lv02d_acpi_write;
// obtain IRQ number of our device from ACPI
    ret = platform_get_irq_optional(device, 0);
    if (ret > 0)
    lis3_dev.irq = ret;
// If possible use a "standard" axes order
    if (lis3_dev.ac.x && lis3_dev.ac.y && lis3_dev.ac.z) {
    pr_info("Using custom axes %d,%d,%d\n",
    lis3_dev.ac.x, lis3_dev.ac.y, lis3_dev.ac.z);
    } else if (dmi_check_system(lis3lv02d_dmi_ids) == 0) {
    pr_info("laptop model unknown, using default axes configuration\n");
    lis3_dev.ac = lis3lv02d_axis_normal;
    }
// call the core layer do its init
    ret = lis3lv02d_init_device(&lis3_dev);
    if (ret)
    return ret;
// filter to remove HPQ6000 accelerometer data
// from keyboard bus stream
    if (strstr(dev_name(&device.dev), "HPQ6000"))
    i8042_install_filter(hp_accel_i8042_filter, core::ptr::null_mut());
    INIT_WORK(&hpled_led.work, delayed_set_status_worker);
    ret = led_classdev_register(core::ptr::null_mut(), &hpled_led.led_classdev);
    if (ret) {
    i8042_remove_filter(hp_accel_i8042_filter);
    lis3lv02d_joystick_disable(&lis3_dev);
    lis3lv02d_poweroff(&lis3_dev);
    flush_work(&hpled_led.work);
    lis3lv02d_remove_fs(&lis3_dev);
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lis3lv02d_remove(device: *mut platform_device) {
    static void lis3lv02d_remove(struct platform_device *device)
    {
    i8042_remove_filter(hp_accel_i8042_filter);
    lis3lv02d_joystick_disable(&lis3_dev);
    lis3lv02d_poweroff(&lis3_dev);
    led_classdev_unregister(&hpled_led.led_classdev);
    flush_work(&hpled_led.work);
    lis3lv02d_remove_fs(&lis3_dev);
    }
#[no_mangle]
unsafe extern "C" fn lis3lv02d_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused lis3lv02d_suspend(struct device *dev)
    {
// make sure the device is off when we suspend
    lis3lv02d_poweroff(&lis3_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lis3lv02d_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused lis3lv02d_resume(struct device *dev)
    {
    lis3lv02d_poweron(&lis3_dev);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(hp_accel_pm, lis3lv02d_suspend, lis3lv02d_resume);
// For the HP MDPS aka 3D Driveguard
    static struct platform_driver lis3lv02d_driver = {
    .probe	= lis3lv02d_probe,
    .remove	= lis3lv02d_remove,
    .driver	= {
    .name	= "hp_accel",
    .pm	= &hp_accel_pm,
    .acpi_match_table = lis3lv02d_device_ids,
    },
    };
    module_platform_driver(lis3lv02d_driver);
    MODULE_DESCRIPTION("Glue between LIS3LV02Dx and HP ACPI BIOS and support for disk protection LED.");
    MODULE_AUTHOR("Yan Burman, Eric Piel, Pavel Machek");
    MODULE_LICENSE("GPL");

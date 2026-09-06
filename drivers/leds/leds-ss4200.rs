//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-ss4200.c
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
// SS4200-E Hardware API
// Copyright (c) 2009, Intel Corporation.
// Copyright IBM Corporation, 2009
//
// Author: Dave Hansen <dave@sr71.net>
//

    MODULE_AUTHOR("Rodney Girod <rgirod@confocus.com>, Dave Hansen <dave@sr71.net>");
    MODULE_DESCRIPTION("Intel NAS/Home Server ICH7 GPIO Driver");
    MODULE_LICENSE("GPL");
//
// ICH7 LPC/GPIO PCI Config register offsets
//
pub const PMBASE: c_uint = 0x040;
pub const GPIO_BASE: c_uint = 0x048;
pub const GPIO_CTRL: c_uint = 0x04c;
pub const GPIO_EN: c_uint = 0x010;
//
// The ICH7 GPIO register block is 64 bytes in size.
//
pub const ICH7_GPIO_SIZE: c_int = 64;
//
// Define register offsets within the ICH7 register block.
//
pub const GPIO_USE_SEL: c_uint = 0x000;
pub const GP_IO_SEL: c_uint = 0x004;
pub const GP_LVL: c_uint = 0x00c;
pub const GPO_BLINK: c_uint = 0x018;
pub const GPI_INV: c_uint = 0x030;
pub const GPIO_USE_SEL2: c_uint = 0x034;
pub const GP_IO_SEL2: c_uint = 0x038;
pub const GP_LVL2: c_uint = 0x03c;
//
// PCI ID of the Intel ICH7 LPC Device within which the GPIO block lives.
//
    static const struct pci_device_id ich7_lpc_pci_id[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_ICH7_0) },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_ICH7_1) },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_ICH7_30) },
    { } /* core::ptr::null_mut() entry */
    };
    MODULE_DEVICE_TABLE(pci, ich7_lpc_pci_id);
#[no_mangle]
unsafe extern "C" fn ss4200_led_dmi_callback(id: *const dmi_system_id) -> int __init {
    static int __init ss4200_led_dmi_callback(const struct dmi_system_id *id)
    {
    pr_info("detected '%s'\n", id.ident);
    return 1;
    }
    static bool nodetect;
    module_param_named(nodetect, nodetect, bool, 0);
    MODULE_PARM_DESC(nodetect, "Skip DMI-based hardware detection");
//
// struct nas_led_whitelist - List of known good models
//
// Contains the known good models this driver is compatible with.
// When adding a new model try to be as strict as possible. This
// makes it possible to keep the false positives (the model is
// detected as working, but in reality it is not) as low as
// possible.
//
    static const struct dmi_system_id nas_led_whitelist[] __initconst = {
    {
    .callback = ss4200_led_dmi_callback,
    .ident = "Intel SS4200-E",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Intel"),
    DMI_MATCH(DMI_PRODUCT_NAME, "SS4200-E"),
    DMI_MATCH(DMI_PRODUCT_VERSION, "1.00.00")
    }
    },
    {
//
// FUJITSU SIEMENS SCALEO Home Server/SS4200-E
// BIOS V090L 12/19/2007
//
    .callback = ss4200_led_dmi_callback,
    .ident = "Fujitsu Siemens SCALEO Home Server",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU SIEMENS"),
    DMI_MATCH(DMI_PRODUCT_NAME, "SCALEO Home Server"),
    DMI_MATCH(DMI_PRODUCT_VERSION, "1.00.00")
    }
    },
    {}
    };
//
// Base I/O address assigned to the Power Management register block
//
    static u32 g_pm_io_base;
//
// Base I/O address assigned to the ICH7 GPIO register block
//
    static u32 nas_gpio_io_base;
//
// When we successfully register a region, we are returned a resource.
// We use these to identify which regions we need to release on our way
// back out.
//
    static struct resource *gp_gpio_resource;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nasgpio_led {
    pub name: *mut c_char,
    pub gpio_bit: u32,
    pub led_cdev: led_classdev,
}

//
// gpio_bit(s) are the ICH7 GPIO bit assignments
//
    static struct nasgpio_led nasgpio_leds[] = {
    { .name = "hdd1:blue:sata",	.gpio_bit = 0 },
    { .name = "hdd1:amber:sata",	.gpio_bit = 1 },
    { .name = "hdd2:blue:sata",	.gpio_bit = 2 },
    { .name = "hdd2:amber:sata",	.gpio_bit = 3 },
    { .name = "hdd3:blue:sata",	.gpio_bit = 4 },
    { .name = "hdd3:amber:sata",	.gpio_bit = 5 },
    { .name = "hdd4:blue:sata",	.gpio_bit = 6 },
    { .name = "hdd4:amber:sata",	.gpio_bit = 7 },
    { .name = "power:blue:power",	.gpio_bit = 27},
    { .name = "power:amber:power",  .gpio_bit = 28},
    };
pub const NAS_RECOVERY: c_uint = 0x00000400	/* GPIO10 */;
    static struct nasgpio_led *
    led_classdev_to_nasgpio_led(struct led_classdev *led_cdev)
    {
    return container_of(led_cdev, struct nasgpio_led, led_cdev);
    }
    static struct nasgpio_led *get_led_named(char *name)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(nasgpio_leds); i++) {
    if (strcmp(nasgpio_leds[i].name, name))
    continue;
    return &nasgpio_leds[i];
    }
    return core::ptr::null_mut();
    }
//
// This protects access to the gpio ports.
//
    static DEFINE_SPINLOCK(nasgpio_gpio_lock);
//
// There are two gpio ports, one for blinking and the other
// for power.  @port tells us if we're doing blinking or
// power control.
//
// Caller must hold nasgpio_gpio_lock
//
    static void __nasgpio_led_set_attr(struct led_classdev *led_cdev,
    u32 port, u32 value)
    {
    struct nasgpio_led *led = led_classdev_to_nasgpio_led(led_cdev);
    u32 gpio_out;
    gpio_out = inl(nas_gpio_io_base + port);
    if (value)
    gpio_out |= (1<<led.gpio_bit);
    else
    gpio_out &= ~(1<<led.gpio_bit);
    outl(gpio_out, nas_gpio_io_base + port);
    }
    static void nasgpio_led_set_attr(struct led_classdev *led_cdev,
    u32 port, u32 value)
    {
    spin_lock(&nasgpio_gpio_lock);
    __nasgpio_led_set_attr(led_cdev, port, value);
    spin_unlock(&nasgpio_gpio_lock);
    }
#[no_mangle]
unsafe extern "C" fn nasgpio_led_get_attr(led_cdev: *mut led_classdev, port: u32) -> u32 {
    static u32 nasgpio_led_get_attr(struct led_classdev *led_cdev, u32 port)
    {
    struct nasgpio_led *led = led_classdev_to_nasgpio_led(led_cdev);
    u32 gpio_in;
    spin_lock(&nasgpio_gpio_lock);
    gpio_in = inl(nas_gpio_io_base + port);
    spin_unlock(&nasgpio_gpio_lock);
    if (gpio_in & (1<<led.gpio_bit))
    return 1;
    return 0;
    }
//
// There is actual brightness control in the hardware,
// but it is via smbus commands and not implemented
// in this driver.
//
    static void nasgpio_led_set_brightness(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    let mut setting: u32 = 0;
    if (brightness >= LED_HALF)
    setting = 1;
//
// Hold the lock across both operations.  This ensures
// consistency so that both the "turn off blinking"
// and "turn light off" operations complete as a set.
//
    spin_lock(&nasgpio_gpio_lock);
//
// LED class documentation asks that past blink state
// be disabled when brightness is turned to zero.
//
    if (brightness == 0)
    __nasgpio_led_set_attr(led_cdev, GPO_BLINK, 0);
    __nasgpio_led_set_attr(led_cdev, GP_LVL, setting);
    spin_unlock(&nasgpio_gpio_lock);
    }
    static int nasgpio_led_set_blink(struct led_classdev *led_cdev,
    unsigned long *delay_on,
    unsigned long *delay_off)
    {
    let mut setting: u32 = 1;
    if (!(*delay_on == 0 && *delay_off == 0) &&
    !(*delay_on == 500 && *delay_off == 500))
    return -EINVAL;
//
// These are very approximate.
//
// delay_on = 500;
// delay_off = 500;
    nasgpio_led_set_attr(led_cdev, GPO_BLINK, setting);
    return 0;
    }
//
// Initialize the ICH7 GPIO registers for NAS usage.  The BIOS should have
// already taken care of this, but we will do so in a non destructive manner
// so that we have what we need whether the BIOS did it or not.
//
#[no_mangle]
unsafe extern "C" fn ich7_gpio_init(dev: *mut device) -> c_int {
    static int ich7_gpio_init(struct device *dev)
    {
    int i;
    let mut config_data: u32 = 0;
    let mut all_nas_led: u32 = 0;
    for (i = 0; i < ARRAY_SIZE(nasgpio_leds); i++)
    all_nas_led |= (1<<nasgpio_leds[i].gpio_bit);
    spin_lock(&nasgpio_gpio_lock);
//
// We need to enable all of the GPIO lines used by the NAS box,
// so we will read the current Use Selection and add our usage
// to it.  This should be benign with regard to the original
// BIOS configuration.
//
    config_data = inl(nas_gpio_io_base + GPIO_USE_SEL);
    dev_dbg(dev, ": Data read from GPIO_USE_SEL = 0x%08x\n", config_data);
    config_data |= all_nas_led + NAS_RECOVERY;
    outl(config_data, nas_gpio_io_base + GPIO_USE_SEL);
    config_data = inl(nas_gpio_io_base + GPIO_USE_SEL);
    dev_dbg(dev, ": GPIO_USE_SEL = 0x%08x\n\n", config_data);
//
// The LED GPIO outputs need to be configured for output, so we
// will ensure that all LED lines are cleared for output and the
// RECOVERY line ready for input.  This too should be benign with
// regard to BIOS configuration.
//
    config_data = inl(nas_gpio_io_base + GP_IO_SEL);
    dev_dbg(dev, ": Data read from GP_IO_SEL = 0x%08x\n",
    config_data);
    config_data &= ~all_nas_led;
    config_data |= NAS_RECOVERY;
    outl(config_data, nas_gpio_io_base + GP_IO_SEL);
    config_data = inl(nas_gpio_io_base + GP_IO_SEL);
    dev_dbg(dev, ": GP_IO_SEL = 0x%08x\n", config_data);
//
// In our final system, the BIOS will initialize the state of all
// of the LEDs.  For now, we turn them all off (or Low).
//
    config_data = inl(nas_gpio_io_base + GP_LVL);
    dev_dbg(dev, ": Data read from GP_LVL = 0x%08x\n", config_data);
//
// In our final system, the BIOS will initialize the blink state of all
// of the LEDs.  For now, we turn blink off for all of them.
//
    config_data = inl(nas_gpio_io_base + GPO_BLINK);
    dev_dbg(dev, ": Data read from GPO_BLINK = 0x%08x\n", config_data);
//
// At this moment, I am unsure if anything needs to happen with GPI_INV
//
    config_data = inl(nas_gpio_io_base + GPI_INV);
    dev_dbg(dev, ": Data read from GPI_INV = 0x%08x\n", config_data);
    spin_unlock(&nasgpio_gpio_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ich7_lpc_cleanup(dev: *mut device) {
    static void ich7_lpc_cleanup(struct device *dev)
    {
//
// If we were given exclusive use of the GPIO
// I/O Address range, we must return it.
//
    if (gp_gpio_resource) {
    dev_dbg(dev, ": Releasing GPIO I/O addresses\n");
    release_region(nas_gpio_io_base, ICH7_GPIO_SIZE);
    gp_gpio_resource = core::ptr::null_mut();
    }
    }
//
// The OS has determined that the LPC of the Intel ICH7 Southbridge is present
// so we can retrive the required operational information and prepare the GPIO.
//
    static struct pci_dev *nas_gpio_pci_dev;
    static int ich7_lpc_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    int status;
    let mut gc: u32 = 0;
    status = pci_enable_device(dev);
    if (status) {
    dev_err(&dev.dev, "pci_enable_device failed\n");
    return -EIO;
    }
    nas_gpio_pci_dev = dev;
    status = pci_read_config_dword(dev, PMBASE, &g_pm_io_base);
    if (status) {
    status = pcibios_err_to_errno(status);
    goto out;
    }
    g_pm_io_base &= 0x00000ff80;
    status = pci_read_config_dword(dev, GPIO_CTRL, &gc);
    if (!(GPIO_EN & gc)) {
    status = -EEXIST;
    dev_info(&dev.dev,
    "ERROR: The LPC GPIO Block has not been enabled.\n");
    goto out;
    }
    status = pci_read_config_dword(dev, GPIO_BASE, &nas_gpio_io_base);
    if (status) {
    dev_info(&dev.dev, "Unable to read GPIOBASE.\n");
    status = pcibios_err_to_errno(status);
    goto out;
    }
    dev_dbg(&dev.dev, ": GPIOBASE = 0x%08x\n", nas_gpio_io_base);
    nas_gpio_io_base &= 0x00000ffc0;
//
// Insure that we have exclusive access to the GPIO I/O address range.
//
    gp_gpio_resource = request_region(nas_gpio_io_base, ICH7_GPIO_SIZE,
    KBUILD_MODNAME);
    if (core::ptr::null_mut() == gp_gpio_resource) {
    dev_info(&dev.dev,
    "ERROR Unable to register GPIO I/O addresses.\n");
    status = -1;
    goto out;
    }
//
// Initialize the GPIO for NAS/Home Server Use
//
    ich7_gpio_init(&dev.dev);
    out:
    if (status) {
    ich7_lpc_cleanup(&dev.dev);
    pci_disable_device(dev);
    }
    return status;
    }
#[no_mangle]
unsafe extern "C" fn ich7_lpc_remove(dev: *mut pci_dev) {
    static void ich7_lpc_remove(struct pci_dev *dev)
    {
    ich7_lpc_cleanup(&dev.dev);
    pci_disable_device(dev);
    }
//
// pci_driver structure passed to the PCI modules
//
    static struct pci_driver nas_gpio_pci_driver = {
    .name = KBUILD_MODNAME,
    .id_table = ich7_lpc_pci_id,
    .probe = ich7_lpc_probe,
    .remove = ich7_lpc_remove,
    };
    static struct led_classdev *get_classdev_for_led_nr(int nr)
    {
    struct nasgpio_led *nas_led = &nasgpio_leds[nr];
    struct led_classdev *led = &nas_led.led_cdev;
    return led;
    }
#[no_mangle]
unsafe extern "C" fn set_power_light_amber_noblink() {
    static void set_power_light_amber_noblink(void)
    {
    struct nasgpio_led *amber = get_led_named("power:amber:power");
    struct nasgpio_led *blue = get_led_named("power:blue:power");
    if (!amber || !blue)
    return;
//
// LED_OFF implies disabling future blinking
//
    pr_debug("setting blue off and amber on\n");
    nasgpio_led_set_brightness(&blue.led_cdev, LED_OFF);
    nasgpio_led_set_brightness(&amber.led_cdev, LED_FULL);
    }
    static ssize_t blink_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct led_classdev *led = dev_get_drvdata(dev);
    let mut blinking: c_int = 0;
    if (nasgpio_led_get_attr(led, GPO_BLINK))
    blinking = 1;
    return sprintf(buf, "%d\n", blinking);
    }
    static ssize_t blink_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    int ret;
    struct led_classdev *led = dev_get_drvdata(dev);
    unsigned long blink_state;
    ret = kstrtoul(buf, 10, &blink_state);
    if (ret)
    return ret;
    nasgpio_led_set_attr(led, GPO_BLINK, blink_state);
    return size;
    }
    static DEVICE_ATTR_RW(blink);
    static struct attribute *nasgpio_led_attrs[] = {
    &dev_attr_blink.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(nasgpio_led);
#[no_mangle]
unsafe extern "C" fn register_nasgpio_led(led_nr: c_int) -> c_int {
    static int register_nasgpio_led(int led_nr)
    {
    struct nasgpio_led *nas_led = &nasgpio_leds[led_nr];
    struct led_classdev *led = get_classdev_for_led_nr(led_nr);
    led.name = nas_led.name;
    led.brightness = LED_OFF;
    if (nasgpio_led_get_attr(led, GP_LVL))
    led.brightness = LED_FULL;
    led.brightness_set = nasgpio_led_set_brightness;
    led.blink_set = nasgpio_led_set_blink;
    led.groups = nasgpio_led_groups;
    return led_classdev_register(&nas_gpio_pci_dev.dev, led);
    }
#[no_mangle]
unsafe extern "C" fn unregister_nasgpio_led(led_nr: c_int) {
    static void unregister_nasgpio_led(int led_nr)
    {
    struct led_classdev *led = get_classdev_for_led_nr(led_nr);
    led_classdev_unregister(led);
    }
//
// module load/initialization
//
#[no_mangle]
unsafe extern "C" fn nas_gpio_init() -> int __init {
    static int __init nas_gpio_init(void)
    {
    int i;
    let mut ret: c_int = 0;
    let mut nr_devices: c_int = 0;
    nr_devices = dmi_check_system(nas_led_whitelist);
    if (nodetect) {
    pr_info("skipping hardware autodetection\n");
    pr_info("Please send 'dmidecode' output to dave@sr71.net\n");
    nr_devices++;
    }
    if (nr_devices <= 0) {
    pr_info("no LED devices found\n");
    return -ENODEV;
    }
    pr_info("registering PCI driver\n");
    ret = pci_register_driver(&nas_gpio_pci_driver);
    if (ret)
    return ret;
    for (i = 0; i < ARRAY_SIZE(nasgpio_leds); i++) {
    ret = register_nasgpio_led(i);
    if (ret)
    goto out_err;
    }
//
// When the system powers on, the BIOS leaves the power
// light blue and blinking.  This will turn it solid
// amber once the driver is loaded.
//
    set_power_light_amber_noblink();
    return 0;
    out_err:
    for (i--; i >= 0; i--)
    unregister_nasgpio_led(i);
    pci_unregister_driver(&nas_gpio_pci_driver);
    return ret;
    }
//
// module unload
//
#[no_mangle]
unsafe extern "C" fn nas_gpio_exit() -> void __exit {
    static void __exit nas_gpio_exit(void)
    {
    int i;
    pr_info("Unregistering driver\n");
    for (i = 0; i < ARRAY_SIZE(nasgpio_leds); i++)
    unregister_nasgpio_led(i);
    pci_unregister_driver(&nas_gpio_pci_driver);
    }
    module_init(nas_gpio_init);
    module_exit(nas_gpio_exit);

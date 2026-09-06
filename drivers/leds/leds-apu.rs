//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-apu.c
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


//
// drivers/leds/leds-apu.c
// Copyright (C) 2017 Alan Mizrahi, alan at mizrahi dot com dot ve
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the names of the copyright holders nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
//

pub const APU1_FCH_ACPI_MMIO_BASE: c_uint = 0xFED80000;

pub const APU1_LEDON: c_uint = 0x08;
pub const APU1_LEDOFF: c_uint = 0xC8;
pub const APU1_NUM_GPIO: c_int = 3;

// LED access parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apu_param {
    pub /: *mut *mut *mut void __iomem addr; / for ioread/iowrite,
}

// LED private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apu_led_priv {
    pub cdev: led_classdev,
    pub param: apu_param,
}

// LED profile
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apu_led_profile {
    pub name: *const c_char,
    pub brightness: enum led_brightness,
    pub /: *mut *mut unsigned long offset; / for devm_ioremap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apu_led_pdata {
    pub pdev: *mut platform_device,
    pub pled: *mut apu_led_priv,
    pub lock: spinlock_t,
}

    static struct apu_led_pdata *apu_led;
    static const struct apu_led_profile apu1_led_profile[] = {
    { "apu:green:1", LED_ON,  APU1_FCH_GPIO_BASE + 0 * APU1_IOSIZE },
    { "apu:green:2", LED_OFF, APU1_FCH_GPIO_BASE + 1 * APU1_IOSIZE },
    { "apu:green:3", LED_OFF, APU1_FCH_GPIO_BASE + 2 * APU1_IOSIZE },
    };
    static const struct dmi_system_id apu_led_dmi_table[] __initconst = {
// PC Engines APU with factory bios "SageBios_PCEngines_APU-45"
    {
    .ident = "apu",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "PC Engines"),
    DMI_MATCH(DMI_PRODUCT_NAME, "APU")
    }
    },
// PC Engines APU with "Mainline" bios >= 4.6.8
    {
    .ident = "apu",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "PC Engines"),
    DMI_MATCH(DMI_PRODUCT_NAME, "apu1")
    }
    },
    {}
    };
    MODULE_DEVICE_TABLE(dmi, apu_led_dmi_table);
#[no_mangle]
unsafe extern "C" fn apu1_led_brightness_set(led: *mut led_classdev, value: enum led_brightness) {
    static void apu1_led_brightness_set(struct led_classdev *led, enum led_brightness value)
    {
    struct apu_led_priv *pled = cdev_to_priv(led);
    spin_lock(&apu_led.lock);
    iowrite8(value ? APU1_LEDON : APU1_LEDOFF, pled.param.addr);
    spin_unlock(&apu_led.lock);
    }
#[no_mangle]
unsafe extern "C" fn apu_led_config(dev: *mut device, apuld: *mut apu_led_pdata) -> c_int {
    static int apu_led_config(struct device *dev, struct apu_led_pdata *apuld)
    {
    int i;
    int err;
    apu_led.pled = devm_kcalloc(dev,
    ARRAY_SIZE(apu1_led_profile), sizeof(struct apu_led_priv),
    GFP_KERNEL);
    if (!apu_led.pled)
    return -ENOMEM;
    for (i = 0; i < ARRAY_SIZE(apu1_led_profile); i++) {
    struct apu_led_priv *pled = &apu_led.pled[i];
    struct led_classdev *led_cdev = &pled.cdev;
    led_cdev.name = apu1_led_profile[i].name;
    led_cdev.brightness = apu1_led_profile[i].brightness;
    led_cdev.max_brightness = 1;
    led_cdev.flags = LED_CORE_SUSPENDRESUME;
    led_cdev.brightness_set = apu1_led_brightness_set;
    pled.param.addr = devm_ioremap(dev,
    apu1_led_profile[i].offset, APU1_IOSIZE);
    if (!pled.param.addr) {
    err = -ENOMEM;
    goto error;
    }
    err = led_classdev_register(dev, led_cdev);
    if (err)
    goto error;
    apu1_led_brightness_set(led_cdev, apu1_led_profile[i].brightness);
    }
    return 0;
    error:
    while (i-- > 0)
    led_classdev_unregister(&apu_led.pled[i].cdev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn apu_led_probe(pdev: *mut platform_device) -> int __init {
    static int __init apu_led_probe(struct platform_device *pdev)
    {
    apu_led = devm_kzalloc(&pdev.dev, sizeof(*apu_led), GFP_KERNEL);
    if (!apu_led)
    return -ENOMEM;
    apu_led.pdev = pdev;
    spin_lock_init(&apu_led.lock);
    return apu_led_config(&pdev.dev, apu_led);
    }
    static struct platform_driver apu_led_driver = {
    .driver = {
    .name = KBUILD_MODNAME,
    },
    };
#[no_mangle]
unsafe extern "C" fn apu_led_init() -> int __init {
    static int __init apu_led_init(void)
    {
    struct platform_device *pdev;
    int err;
    if (!dmi_check_system(apu_led_dmi_table)) {
    pr_err("No PC Engines APUv1 board detected. For APUv2,3 support, enable CONFIG_PCENGINES_APU2\n");
    return -ENODEV;
    }
    pdev = platform_device_register_simple(KBUILD_MODNAME, -1, core::ptr::null_mut(), 0);
    if (IS_ERR(pdev)) {
    pr_err("Device allocation failed\n");
    return PTR_ERR(pdev);
    }
    err = platform_driver_probe(&apu_led_driver, apu_led_probe);
    if (err) {
    pr_err("Probe platform driver failed\n");
    platform_device_unregister(pdev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn apu_led_exit() -> void __exit {
    static void __exit apu_led_exit(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(apu1_led_profile); i++)
    led_classdev_unregister(&apu_led.pled[i].cdev);
    platform_device_unregister(apu_led.pdev);
    platform_driver_unregister(&apu_led_driver);
    }
    module_init(apu_led_init);
    module_exit(apu_led_exit);
    MODULE_AUTHOR("Alan Mizrahi");
    MODULE_DESCRIPTION("PC Engines APU1 front LED driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:leds_apu");

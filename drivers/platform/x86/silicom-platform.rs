//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/silicom-platform.c
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
// silicom-platform.c - Silicom MEC170x platform driver
//
// Copyright (C) 2023 Henry Shi <henrys@silicom-usa.com>

pub const MEC_POWER_CYCLE_ADDR: c_uint = 0x24;
pub const MEC_EFUSE_LSB_ADDR: c_uint = 0x28;
pub const MEC_GPIO_IN_POS: c_uint = 0x08;
pub const MEC_IO_BASE: c_uint = 0x0800;
pub const MEC_IO_LEN: c_uint = 0x8;
pub const IO_REG_BANK: c_uint = 0x0;
pub const DEFAULT_CHAN_LO: c_int = 0;
pub const DEFAULT_CHAN_HI: c_int = 0;
pub const DEFAULT_CHAN_LO_T: c_uint = 0xc;

pub const SILICOM_MEC_MAGIC: c_uint = 0x5a;

    static DEFINE_MUTEX(mec_io_mutex);
    static unsigned int efuse_status;
    static unsigned int mec_uc_version;
    static unsigned int power_cycle;
    static const struct hwmon_channel_info *silicom_fan_control_info[] = {
    HWMON_CHANNEL_INFO(fan, HWMON_F_INPUT | HWMON_F_LABEL),
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT | HWMON_T_LABEL),
    core::ptr::null_mut()
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct silicom_platform_info {
    pub io_base: c_int,
    pub io_len: c_int,
    pub led_info: *mut led_classdev_mc,
    pub gpiochip: *mut gpio_chip,
    pub gpio_channels: *mut u8,
    pub ngpio: u16,
}

    static const char * const plat_0222_gpio_names[] = {
    "AUTOM0_SFP_TX_FAULT",
    "SLOT2_LED_OUT",
    "SIM_M2_SLOT2_B_DET",
    "SIM_M2_SLOT2_A_DET",
    "SLOT1_LED_OUT",
    "SIM_M2_SLOT1_B_DET",
    "SIM_M2_SLOT1_A_DET",
    "SLOT0_LED_OUT",
    "WAN_SFP0_RX_LOS",
    "WAN_SFP0_PRSNT_N",
    "WAN_SFP0_TX_FAULT",
    "AUTOM1_SFP_RX_LOS",
    "AUTOM1_SFP_PRSNT_N",
    "AUTOM1_SFP_TX_FAULT",
    "AUTOM0_SFP_RX_LOS",
    "AUTOM0_SFP_PRSNT_N",
    "WAN_SFP1_RX_LOS",
    "WAN_SFP1_PRSNT_N",
    "WAN_SFP1_TX_FAULT",
    "SIM_M2_SLOT1_MUX_SEL",
    "W_DISABLE_M2_SLOT1_N",
    "W_DISABLE_MPCIE_SLOT0_N",
    "W_DISABLE_M2_SLOT0_N",
    "BT_COMMAND_MODE",
    "WAN_SFP1_TX_DISABLE",
    "WAN_SFP0_TX_DISABLE",
    "AUTOM1_SFP_TX_DISABLE",
    "AUTOM0_SFP_TX_DISABLE",
    "SIM_M2_SLOT2_MUX_SEL",
    "W_DISABLE_M2_SLOT2_N",
    "RST_CTL_M2_SLOT_1_N",
    "RST_CTL_M2_SLOT_2_N",
    "PM_USB_PWR_EN_BOT",
    "PM_USB_PWR_EN_TOP",
    };
    static u8 plat_0222_gpio_channels[] = {
    OFFSET_BIT_TO_CHANNEL(0x00, 0),
    OFFSET_BIT_TO_CHANNEL(0x00, 1),
    OFFSET_BIT_TO_CHANNEL(0x00, 2),
    OFFSET_BIT_TO_CHANNEL(0x00, 3),
    OFFSET_BIT_TO_CHANNEL(0x00, 4),
    OFFSET_BIT_TO_CHANNEL(0x00, 5),
    OFFSET_BIT_TO_CHANNEL(0x00, 6),
    OFFSET_BIT_TO_CHANNEL(0x00, 7),
    OFFSET_BIT_TO_CHANNEL(0x01, 0),
    OFFSET_BIT_TO_CHANNEL(0x01, 1),
    OFFSET_BIT_TO_CHANNEL(0x01, 2),
    OFFSET_BIT_TO_CHANNEL(0x01, 3),
    OFFSET_BIT_TO_CHANNEL(0x01, 4),
    OFFSET_BIT_TO_CHANNEL(0x01, 5),
    OFFSET_BIT_TO_CHANNEL(0x01, 6),
    OFFSET_BIT_TO_CHANNEL(0x01, 7),
    OFFSET_BIT_TO_CHANNEL(0x02, 0),
    OFFSET_BIT_TO_CHANNEL(0x02, 1),
    OFFSET_BIT_TO_CHANNEL(0x02, 2),
    OFFSET_BIT_TO_CHANNEL(0x09, 0),
    OFFSET_BIT_TO_CHANNEL(0x09, 1),
    OFFSET_BIT_TO_CHANNEL(0x09, 2),
    OFFSET_BIT_TO_CHANNEL(0x09, 3),
    OFFSET_BIT_TO_CHANNEL(0x0a, 0),
    OFFSET_BIT_TO_CHANNEL(0x0a, 1),
    OFFSET_BIT_TO_CHANNEL(0x0a, 2),
    OFFSET_BIT_TO_CHANNEL(0x0a, 3),
    OFFSET_BIT_TO_CHANNEL(0x0a, 4),
    OFFSET_BIT_TO_CHANNEL(0x0a, 5),
    OFFSET_BIT_TO_CHANNEL(0x0a, 6),
    OFFSET_BIT_TO_CHANNEL(0x0b, 0),
    OFFSET_BIT_TO_CHANNEL(0x0b, 1),
    OFFSET_BIT_TO_CHANNEL(0x0b, 2),
    OFFSET_BIT_TO_CHANNEL(0x0b, 3),
    };
    static struct platform_device *silicom_platform_dev;
    static struct led_classdev_mc *silicom_led_info __initdata;
    static struct gpio_chip *silicom_gpiochip __initdata;
    static u8 *silicom_gpio_channels __initdata;
#[no_mangle]
unsafe extern "C" fn silicom_mec_port_get(offset: c_uint) -> c_int {
    static int silicom_mec_port_get(unsigned int offset)
    {
    unsigned short mec_data_addr;
    unsigned short mec_port_addr;
    u8 reg;
    mec_data_addr = FIELD_GET(MEC_PORT_DWORD_OFFSET, offset) & MEC_DATA_OFFSET_MASK;
    mec_port_addr = FIELD_GET(MEC_PORT_DWORD_OFFSET, offset) & MEC_PORT_OFFSET_MASK;
    mutex_lock(&mec_io_mutex);
    outb(mec_port_addr, MEC_ADDR);
    reg = inb(MEC_DATA_OFFSET(mec_data_addr));
    mutex_unlock(&mec_io_mutex);
    return (reg >> (offset & MEC_PORT_CHANNEL_MASK)) & 0x01;
    }
#[no_mangle]
unsafe extern "C" fn silicom_mec_led_get(channel: c_int) -> enum led_brightness {
    static enum led_brightness silicom_mec_led_get(int channel)
    {
// Outputs are active low
    return silicom_mec_port_get(channel) ? LED_OFF : LED_ON;
    }
#[no_mangle]
unsafe extern "C" fn silicom_mec_port_set(channel: c_int, on: c_int) {
    static void silicom_mec_port_set(int channel, int on)
    {
    unsigned short mec_data_addr;
    unsigned short mec_port_addr;
    u8 reg;
    mec_data_addr = FIELD_GET(MEC_PORT_DWORD_OFFSET, channel) & MEC_DATA_OFFSET_MASK;
    mec_port_addr = FIELD_GET(MEC_PORT_DWORD_OFFSET, channel) & MEC_PORT_OFFSET_MASK;
    mutex_lock(&mec_io_mutex);
    outb(mec_port_addr, MEC_ADDR);
    reg = inb(MEC_DATA_OFFSET(mec_data_addr));
// Outputs are active low, so clear the bit for on, or set it for off
    if (on)
    reg &= ~(1 << (channel & MEC_PORT_CHANNEL_MASK));
    else
    reg |= 1 << (channel & MEC_PORT_CHANNEL_MASK);
    outb(reg, MEC_DATA_OFFSET(mec_data_addr));
    mutex_unlock(&mec_io_mutex);
    }
#[no_mangle]
unsafe extern "C" fn silicom_mec_led_mc_brightness_get(led_cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness silicom_mec_led_mc_brightness_get(struct led_classdev *led_cdev)
    {
    struct led_classdev_mc *mc_cdev = lcdev_to_mccdev(led_cdev);
    let mut brightness: enum led_brightness = LED_OFF;
    int i;
    for (i = 0; i < mc_cdev.num_colors; i++) {
    mc_cdev.subled_info[i].brightness =
    silicom_mec_led_get(mc_cdev.subled_info[i].channel);
// Mark the overall brightness as LED_ON if any of the subleds are on
    if (mc_cdev.subled_info[i].brightness != LED_OFF)
    brightness = LED_ON;
    }
    return brightness;
    }
    static void silicom_mec_led_mc_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct led_classdev_mc *mc_cdev = lcdev_to_mccdev(led_cdev);
    int i;
    led_mc_calc_color_components(mc_cdev, brightness);
    for (i = 0; i < mc_cdev.num_colors; i++) {
    silicom_mec_port_set(mc_cdev.subled_info[i].channel,
    mc_cdev.subled_info[i].brightness);
    }
    }
    static int silicom_gpio_get_direction(struct gpio_chip *gc,
    unsigned int offset)
    {
    u8 *channels = gpiochip_get_data(gc);
// Input registers have offsets between [0x00, 0x07]
    if (CHANNEL_TO_OFFSET(channels[offset]) < MEC_GPIO_IN_POS)
    return GPIO_LINE_DIRECTION_IN;
    return GPIO_LINE_DIRECTION_OUT;
    }
    static int silicom_gpio_direction_input(struct gpio_chip *gc,
    unsigned int offset)
    {
    let mut direction: c_int = silicom_gpio_get_direction(gc, offset);
    let mut direction: return = = GPIO_LINE_DIRECTION_IN ? 0 : -EINVAL;
    }
    static int silicom_gpio_set(struct gpio_chip *gc, unsigned int offset,
    int value)
    {
    u8 *channels = gpiochip_get_data(gc);
    let mut channel: c_int = channels[offset];
    silicom_mec_port_set(channel, !value);
    return 0;
    }
    static int silicom_gpio_direction_output(struct gpio_chip *gc,
    unsigned int offset,
    int value)
    {
    let mut direction: c_int = silicom_gpio_get_direction(gc, offset);
    if (direction == GPIO_LINE_DIRECTION_IN)
    return -EINVAL;
    silicom_gpio_set(gc, offset, value);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn silicom_gpio_get(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int silicom_gpio_get(struct gpio_chip *gc, unsigned int offset)
    {
    u8 *channels = gpiochip_get_data(gc);
    let mut channel: c_int = channels[offset];
    return silicom_mec_port_get(channel);
    }
    static struct mc_subled plat_0222_wan_mc_subled_info[] __initdata = {
    {
    .color_index = LED_COLOR_ID_WHITE,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0c, 7),
    },
    {
    .color_index = LED_COLOR_ID_YELLOW,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0c, 6),
    },
    {
    .color_index = LED_COLOR_ID_RED,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0c, 5),
    },
    };
    static struct mc_subled plat_0222_sys_mc_subled_info[] __initdata = {
    {
    .color_index = LED_COLOR_ID_WHITE,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0c, 4),
    },
    {
    .color_index = LED_COLOR_ID_AMBER,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0c, 3),
    },
    {
    .color_index = LED_COLOR_ID_RED,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0c, 2),
    },
    };
    static struct mc_subled plat_0222_stat1_mc_subled_info[] __initdata = {
    {
    .color_index = LED_COLOR_ID_RED,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0c, 1),
    },
    {
    .color_index = LED_COLOR_ID_GREEN,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0c, 0),
    },
    {
    .color_index = LED_COLOR_ID_BLUE,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0d, 7),
    },
    {
    .color_index = LED_COLOR_ID_YELLOW,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0d, 6),
    },
    };
    static struct mc_subled plat_0222_stat2_mc_subled_info[] __initdata = {
    {
    .color_index = LED_COLOR_ID_RED,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0d, 5),
    },
    {
    .color_index = LED_COLOR_ID_GREEN,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0d, 4),
    },
    {
    .color_index = LED_COLOR_ID_BLUE,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0d, 3),
    },
    {
    .color_index = LED_COLOR_ID_YELLOW,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0d, 2),
    },
    };
    static struct mc_subled plat_0222_stat3_mc_subled_info[] __initdata = {
    {
    .color_index = LED_COLOR_ID_RED,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0d, 1),
    },
    {
    .color_index = LED_COLOR_ID_GREEN,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0d, 0),
    },
    {
    .color_index = LED_COLOR_ID_BLUE,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0e, 1),
    },
    {
    .color_index = LED_COLOR_ID_YELLOW,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x0e, 0),
    },
    };
    static struct led_classdev_mc plat_0222_mc_led_info[] __initdata = {
    {
    .led_cdev = {
    .name = "platled::wan",
    .brightness = 0,
    .max_brightness = 1,
    .brightness_set = silicom_mec_led_mc_brightness_set,
    .brightness_get = silicom_mec_led_mc_brightness_get,
    },
    .num_colors = ARRAY_SIZE(plat_0222_wan_mc_subled_info),
    .subled_info = plat_0222_wan_mc_subled_info,
    },
    {
    .led_cdev = {
    .name = "platled::sys",
    .brightness = 0,
    .max_brightness = 1,
    .brightness_set = silicom_mec_led_mc_brightness_set,
    .brightness_get = silicom_mec_led_mc_brightness_get,
    },
    .num_colors = ARRAY_SIZE(plat_0222_sys_mc_subled_info),
    .subled_info = plat_0222_sys_mc_subled_info,
    },
    {
    .led_cdev = {
    .name = "platled::stat1",
    .brightness = 0,
    .max_brightness = 1,
    .brightness_set = silicom_mec_led_mc_brightness_set,
    .brightness_get = silicom_mec_led_mc_brightness_get,
    },
    .num_colors = ARRAY_SIZE(plat_0222_stat1_mc_subled_info),
    .subled_info = plat_0222_stat1_mc_subled_info,
    },
    {
    .led_cdev = {
    .name = "platled::stat2",
    .brightness = 0,
    .max_brightness = 1,
    .brightness_set = silicom_mec_led_mc_brightness_set,
    .brightness_get = silicom_mec_led_mc_brightness_get,
    },
    .num_colors = ARRAY_SIZE(plat_0222_stat2_mc_subled_info),
    .subled_info = plat_0222_stat2_mc_subled_info,
    },
    {
    .led_cdev = {
    .name = "platled::stat3",
    .brightness = 0,
    .max_brightness = 1,
    .brightness_set = silicom_mec_led_mc_brightness_set,
    .brightness_get = silicom_mec_led_mc_brightness_get,
    },
    .num_colors = ARRAY_SIZE(plat_0222_stat3_mc_subled_info),
    .subled_info = plat_0222_stat3_mc_subled_info,
    },
    { },
    };
    static struct gpio_chip silicom_gpio_chip = {
    .label = "silicom-gpio",
    .get_direction = silicom_gpio_get_direction,
    .direction_input = silicom_gpio_direction_input,
    .direction_output = silicom_gpio_direction_output,
    .get = silicom_gpio_get,
    .set = silicom_gpio_set,
    .base = -1,
    .ngpio = ARRAY_SIZE(plat_0222_gpio_channels),
    .names = plat_0222_gpio_names,
//
// We're using a mutex to protect the indirect access, so we can sleep
// if the lock blocks
//
    .can_sleep = true,
    };
    static struct silicom_platform_info silicom_plat_0222_cordoba_info __initdata = {
    .io_base = MEC_IO_BASE,
    .io_len = MEC_IO_LEN,
    .led_info = plat_0222_mc_led_info,
    .gpiochip = &silicom_gpio_chip,
    .gpio_channels = plat_0222_gpio_channels,
//
// The original generic cordoba does not have the last 4 outputs of the
// plat_0222 variant, the rest are the same, so use the same longer list,
// but ignore the last entries here
//
    .ngpio = ARRAY_SIZE(plat_0222_gpio_channels),
    };
    static struct mc_subled cordoba_fp_left_mc_subled_info[] __initdata = {
    {
    .color_index = LED_COLOR_ID_RED,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x08, 6),
    },
    {
    .color_index = LED_COLOR_ID_GREEN,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x08, 5),
    },
    {
    .color_index = LED_COLOR_ID_BLUE,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x09, 7),
    },
    {
    .color_index = LED_COLOR_ID_AMBER,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x09, 4),
    },
    };
    static struct mc_subled cordoba_fp_center_mc_subled_info[] __initdata = {
    {
    .color_index = LED_COLOR_ID_RED,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x08, 7),
    },
    {
    .color_index = LED_COLOR_ID_GREEN,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x08, 4),
    },
    {
    .color_index = LED_COLOR_ID_BLUE,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x08, 3),
    },
    {
    .color_index = LED_COLOR_ID_AMBER,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x09, 6),
    },
    };
    static struct mc_subled cordoba_fp_right_mc_subled_info[] __initdata = {
    {
    .color_index = LED_COLOR_ID_RED,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x08, 2),
    },
    {
    .color_index = LED_COLOR_ID_GREEN,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x08, 1),
    },
    {
    .color_index = LED_COLOR_ID_BLUE,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x08, 0),
    },
    {
    .color_index = LED_COLOR_ID_AMBER,
    .brightness = 1,
    .intensity = 0,
    .channel = OFFSET_BIT_TO_CHANNEL(0x09, 5),
    },
    };
    static struct led_classdev_mc cordoba_mc_led_info[] __initdata = {
    {
    .led_cdev = {
    .name = "platled::fp_left",
    .brightness = 0,
    .max_brightness = 1,
    .brightness_set = silicom_mec_led_mc_brightness_set,
    .brightness_get = silicom_mec_led_mc_brightness_get,
    },
    .num_colors = ARRAY_SIZE(cordoba_fp_left_mc_subled_info),
    .subled_info = cordoba_fp_left_mc_subled_info,
    },
    {
    .led_cdev = {
    .name = "platled::fp_center",
    .brightness = 0,
    .max_brightness = 1,
    .brightness_set = silicom_mec_led_mc_brightness_set,
    .brightness_get = silicom_mec_led_mc_brightness_get,
    },
    .num_colors = ARRAY_SIZE(cordoba_fp_center_mc_subled_info),
    .subled_info = cordoba_fp_center_mc_subled_info,
    },
    {
    .led_cdev = {
    .name = "platled::fp_right",
    .brightness = 0,
    .max_brightness = 1,
    .brightness_set = silicom_mec_led_mc_brightness_set,
    .brightness_get = silicom_mec_led_mc_brightness_get,
    },
    .num_colors = ARRAY_SIZE(cordoba_fp_right_mc_subled_info),
    .subled_info = cordoba_fp_right_mc_subled_info,
    },
    { },
    };
    static struct silicom_platform_info silicom_generic_cordoba_info __initdata = {
    .io_base = MEC_IO_BASE,
    .io_len = MEC_IO_LEN,
    .led_info = cordoba_mc_led_info,
    .gpiochip = &silicom_gpio_chip,
    .gpio_channels = plat_0222_gpio_channels,
    .ngpio = ARRAY_SIZE(plat_0222_gpio_channels),
    };
//
// sysfs interface
//
    static ssize_t efuse_status_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    u32 reg;
    mutex_lock(&mec_io_mutex);
// Select memory region
    outb(IO_REG_BANK, EC_ADDR_MSB);
    outb(MEC_EFUSE_LSB_ADDR, EC_ADDR_LSB);
// Get current data from the address
    reg = inl(MEC_DATA_OFFSET(DEFAULT_CHAN_LO));
    mutex_unlock(&mec_io_mutex);
    efuse_status = reg & 0x1;
    return sysfs_emit(buf, "%u\n", efuse_status);
    }
    static DEVICE_ATTR_RO(efuse_status);
    static ssize_t uc_version_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    int uc_version;
    u32 reg;
    mutex_lock(&mec_io_mutex);
    outb(IO_REG_BANK, EC_ADDR_MSB);
    outb(DEFAULT_CHAN_LO, EC_ADDR_LSB);
    reg = inl(MEC_DATA_OFFSET(DEFAULT_CHAN_LO));
    mutex_unlock(&mec_io_mutex);
    uc_version = FIELD_GET(MEC_VERSION_LOC, reg);
    if (uc_version >= 192)
    return -EINVAL;
    uc_version = FIELD_GET(MEC_VERSION_MAJOR, reg) * 100 +
    FIELD_GET(MEC_VERSION_MINOR, reg);
    mec_uc_version = uc_version;
    return sysfs_emit(buf, "%u\n", mec_uc_version);
    }
    static DEVICE_ATTR_RO(uc_version);
    static ssize_t power_cycle_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%u\n", power_cycle);
    }
#[no_mangle]
unsafe extern "C" fn powercycle_uc() {
    static void powercycle_uc(void)
    {
// Select memory region
    outb(IO_REG_BANK, EC_ADDR_MSB);
    outb(MEC_POWER_CYCLE_ADDR, EC_ADDR_LSB);
// Set to 1 for current data from the address
    outb(1, MEC_DATA_OFFSET(DEFAULT_CHAN_LO));
    }
    static ssize_t power_cycle_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    int rc;
    unsigned int power_cycle_cmd;
    rc = kstrtou32(buf, 0, &power_cycle_cmd);
    if (rc)
    return -EINVAL;
    if (power_cycle_cmd > 0) {
    mutex_lock(&mec_io_mutex);
    power_cycle = power_cycle_cmd;
    powercycle_uc();
    mutex_unlock(&mec_io_mutex);
    }
    return count;
    }
    static DEVICE_ATTR_RW(power_cycle);
    static struct attribute *silicom_attrs[] = {
    &dev_attr_efuse_status.attr,
    &dev_attr_uc_version.attr,
    &dev_attr_power_cycle.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(silicom);
    static struct platform_driver silicom_platform_driver = {
    .driver = {
    .name = "silicom-platform",
    .dev_groups = silicom_groups,
    },
    };
    static int __init silicom_mc_leds_register(struct device *dev,
    const struct led_classdev_mc *mc_leds)
    {
    let mut size: c_int = sizeof(struct mc_subled);
    struct led_classdev_mc *led;
    int i, err;
    for (i = 0; mc_leds[i].led_cdev.name; i++) {
    led = devm_kzalloc(dev, sizeof(*led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    memcpy(led, &mc_leds[i], sizeof(*led));
    led.subled_info = devm_kzalloc(dev, led.num_colors * size, GFP_KERNEL);
    if (!led.subled_info)
    return -ENOMEM;
    memcpy(led.subled_info, mc_leds[i].subled_info, led.num_colors * size);
    err = devm_led_classdev_multicolor_register(dev, led);
    if (err)
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpm_get() -> u32 {
    static u32 rpm_get(void)
    {
    u32 reg;
    mutex_lock(&mec_io_mutex);
// Select memory region
    outb(IO_REG_BANK, EC_ADDR_MSB);
    outb(DEFAULT_CHAN_LO_T, EC_ADDR_LSB);
    reg = inw(MEC_DATA_OFFSET(DEFAULT_CHAN_LO));
    mutex_unlock(&mec_io_mutex);
    return reg;
    }
#[no_mangle]
unsafe extern "C" fn temp_get() -> u32 {
    static u32 temp_get(void)
    {
    u32 reg;
    mutex_lock(&mec_io_mutex);
// Select memory region
    outb(IO_REG_BANK, EC_ADDR_MSB);
    outb(DEFAULT_CHAN_LO_T, EC_ADDR_LSB);
    reg = inl(MEC_DATA_OFFSET(DEFAULT_CHAN_LO));
    mutex_unlock(&mec_io_mutex);
    return FIELD_GET(MEC_TEMP_LOC, reg) * 100;
    }
#[no_mangle]
unsafe extern "C" fn silicom_fan_control_fan_is_visible(attr: u32) -> umode_t {
    static umode_t silicom_fan_control_fan_is_visible(const u32 attr)
    {
    switch (attr) {
    case hwmon_fan_input:
    case hwmon_fan_label:
    return 0444;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn silicom_fan_control_temp_is_visible(attr: u32) -> umode_t {
    static umode_t silicom_fan_control_temp_is_visible(const u32 attr)
    {
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_label:
    return 0444;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn silicom_fan_control_read_fan(dev: *mut device, attr: u32, val: *mut c_long) -> c_int {
    static int silicom_fan_control_read_fan(struct device *dev, u32 attr, long *val)
    {
    switch (attr) {
    case hwmon_fan_input:
// val = rpm_get();
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn silicom_fan_control_read_temp(dev: *mut device, attr: u32, val: *mut c_long) -> c_int {
    static int silicom_fan_control_read_temp(struct device *dev, u32 attr, long *val)
    {
    switch (attr) {
    case hwmon_temp_input:
// val = temp_get();
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    static umode_t silicom_fan_control_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_fan:
    return silicom_fan_control_fan_is_visible(attr);
    case hwmon_temp:
    return silicom_fan_control_temp_is_visible(attr);
    default:
    return 0;
    }
    }
    static int silicom_fan_control_read(struct device *dev,
    enum hwmon_sensor_types type,
    u32 attr, int channel,
    long *val)
    {
    switch (type) {
    case hwmon_fan:
    return silicom_fan_control_read_fan(dev, attr, val);
    case hwmon_temp:
    return silicom_fan_control_read_temp(dev, attr, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int silicom_fan_control_read_labels(struct device *dev,
    enum hwmon_sensor_types type,
    u32 attr, int channel,
    const char **str)
    {
    switch (type) {
    case hwmon_fan:
// str = "Silicom_platform: Fan Speed";
    return 0;
    case hwmon_temp:
// str = "Silicom_platform: Thermostat Sensor";
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    static const struct hwmon_ops silicom_fan_control_hwmon_ops = {
    .is_visible = silicom_fan_control_is_visible,
    .read = silicom_fan_control_read,
    .read_string = silicom_fan_control_read_labels,
    };
    static const struct hwmon_chip_info silicom_chip_info = {
    .ops = &silicom_fan_control_hwmon_ops,
    .info = silicom_fan_control_info,
    };
#[no_mangle]
unsafe extern "C" fn silicom_platform_probe(device: *mut platform_device) -> int __init {
    static int __init silicom_platform_probe(struct platform_device *device)
    {
    struct device *hwmon_dev;
    u8 magic, ver;
    int err;
    if (!devm_request_region(&device.dev, MEC_IO_BASE, MEC_IO_LEN, "mec")) {
    dev_err(&device.dev, "couldn't reserve MEC io ports\n");
    return -EBUSY;
    }
// Sanity check magic number read for EC
    outb(IO_REG_BANK, MEC_ADDR);
    magic = inb(MEC_DATA_OFFSET(DEFAULT_CHAN_LO));
    ver = inb(MEC_DATA_OFFSET(DEFAULT_CHAN_HI));
    dev_dbg(&device.dev, "EC magic 0x%02x, version 0x%02x\n", magic, ver);
    if (magic != SILICOM_MEC_MAGIC) {
    dev_err(&device.dev, "Bad EC magic 0x%02x!\n", magic);
    return -ENODEV;
    }
    err = silicom_mc_leds_register(&device.dev, silicom_led_info);
    if (err) {
    dev_err(&device.dev, "Failed to register LEDs\n");
    return err;
    }
    err = devm_gpiochip_add_data(&device.dev, silicom_gpiochip,
    silicom_gpio_channels);
    if (err) {
    dev_err(&device.dev, "Failed to register gpiochip: %d\n", err);
    return err;
    }
    hwmon_dev = devm_hwmon_device_register_with_info(&device.dev, "silicom_fan", core::ptr::null_mut(),
    &silicom_chip_info, core::ptr::null_mut());
    err = PTR_ERR_OR_ZERO(hwmon_dev);
    if (err) {
    dev_err(&device.dev, "Failed to register hwmon_dev: %d\n", err);
    return err;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn silicom_platform_info_init(id: *const dmi_system_id) -> int __init {
    static int __init silicom_platform_info_init(const struct dmi_system_id *id)
    {
    struct silicom_platform_info *info = id.driver_data;
    silicom_led_info = info.led_info;
    silicom_gpio_channels = info.gpio_channels;
    silicom_gpiochip = info.gpiochip;
    silicom_gpiochip.ngpio = info.ngpio;
    return 1;
    }
    static const struct dmi_system_id silicom_dmi_ids[] __initconst = {
    {
    .callback = silicom_platform_info_init,
    .ident = "Silicom Cordoba (Generic)",
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "Silicom"),
    DMI_MATCH(DMI_BOARD_NAME, "80300-0214-G"),
    },
    .driver_data = &silicom_generic_cordoba_info,
    },
    {
    .callback = silicom_platform_info_init,
    .ident = "Silicom Cordoba (Generic)",
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "Silicom"),
    DMI_MATCH(DMI_BOARD_NAME, "80500-0214-G"),
    },
    .driver_data = &silicom_generic_cordoba_info,
    },
    {
    .callback = silicom_platform_info_init,
    .ident = "Silicom Cordoba (plat_0222)",
    .matches = {
    DMI_MATCH(DMI_BOARD_VENDOR, "Silicom"),
    DMI_MATCH(DMI_BOARD_NAME, "80300-0222-G"),
    },
    .driver_data = &silicom_plat_0222_cordoba_info,
    },
    { },
    };
    MODULE_DEVICE_TABLE(dmi, silicom_dmi_ids);
#[no_mangle]
unsafe extern "C" fn silicom_platform_init() -> int __init {
    static int __init silicom_platform_init(void)
    {
    if (!dmi_check_system(silicom_dmi_ids)) {
    pr_err("No DMI match for this platform\n");
    return -ENODEV;
    }
    silicom_platform_dev = platform_create_bundle(&silicom_platform_driver,
    silicom_platform_probe,
    core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    return PTR_ERR_OR_ZERO(silicom_platform_dev);
    }
#[no_mangle]
unsafe extern "C" fn silicom_platform_exit() -> void __exit {
    static void __exit silicom_platform_exit(void)
    {
    platform_device_unregister(silicom_platform_dev);
    platform_driver_unregister(&silicom_platform_driver);
    }
    module_init(silicom_platform_init);
    module_exit(silicom_platform_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Henry Shi <henrys@silicom-usa.com>");
    MODULE_DESCRIPTION("Platform driver for Silicom network appliances");

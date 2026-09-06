//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-mlxbf.c
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

// Number of pins on BlueField
pub const MLXBF_GPIO_NR: c_int = 54;
// Pad Electrical Controls.
pub const MLXBF_GPIO_PAD_CONTROL_FIRST_WORD: c_uint = 0x0700;
pub const MLXBF_GPIO_PAD_CONTROL_1_FIRST_WORD: c_uint = 0x0708;
pub const MLXBF_GPIO_PAD_CONTROL_2_FIRST_WORD: c_uint = 0x0710;
pub const MLXBF_GPIO_PAD_CONTROL_3_FIRST_WORD: c_uint = 0x0718;
pub const MLXBF_GPIO_PIN_DIR_I: c_uint = 0x1040;
pub const MLXBF_GPIO_PIN_DIR_O: c_uint = 0x1048;
pub const MLXBF_GPIO_PIN_STATE: c_uint = 0x1000;
pub const MLXBF_GPIO_SCRATCHPAD: c_uint = 0x20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxbf_gpio_context_save_regs {
    pub scratchpad: u64,
    pub pad_control: [u64; MLXBF_GPIO_NR],
    pub pin_dir_i: u64,
    pub pin_dir_o: u64,
}

// Device state structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxbf_gpio_state {
    pub chip: gpio_generic_chip,
// Memory Address
    pub base: *mut void __iomem,

    pub csave_regs: mlxbf_gpio_context_save_regs,

}

#[no_mangle]
unsafe extern "C" fn mlxbf_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int mlxbf_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct mlxbf_gpio_state *gs;
    struct device *dev = &pdev.dev;
    struct gpio_chip *gc;
    int ret;
    gs = devm_kzalloc(&pdev.dev, sizeof(*gs), GFP_KERNEL);
    if (!gs)
    return -ENOMEM;
    gs.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(gs.base))
    return PTR_ERR(gs.base);
    gc = &gs.chip.gc;
    config = (struct gpio_generic_chip_config) {
    .dev = dev,
    .sz = 8,
    .dat = gs.base + MLXBF_GPIO_PIN_STATE,
    .dirout = gs.base + MLXBF_GPIO_PIN_DIR_O,
    .dirin =  gs.base + MLXBF_GPIO_PIN_DIR_I,
    };
    ret = gpio_generic_chip_init(&gs.chip, &config);
    if (ret)
    return -ENODEV;
    gc.owner = THIS_MODULE;
    gc.ngpio = MLXBF_GPIO_NR;
    ret = devm_gpiochip_add_data(dev, &gs.chip.gc, gs);
    if (ret) {
    dev_err(&pdev.dev, "Failed adding memory mapped gpiochip\n");
    return ret;
    }
    platform_set_drvdata(pdev, gs);
    dev_info(&pdev.dev, "registered Mellanox BlueField GPIO");
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn mlxbf_gpio_suspend(pdev: *mut platform_device, state: pm_message_t) -> c_int {
    static int mlxbf_gpio_suspend(struct platform_device *pdev, pm_message_t state)
    {
    struct mlxbf_gpio_state *gs = platform_get_drvdata(pdev);
    gs.csave_regs.scratchpad = readq(gs.base + MLXBF_GPIO_SCRATCHPAD);
    gs.csave_regs.pad_control[0] =
    readq(gs.base + MLXBF_GPIO_PAD_CONTROL_FIRST_WORD);
    gs.csave_regs.pad_control[1] =
    readq(gs.base + MLXBF_GPIO_PAD_CONTROL_1_FIRST_WORD);
    gs.csave_regs.pad_control[2] =
    readq(gs.base + MLXBF_GPIO_PAD_CONTROL_2_FIRST_WORD);
    gs.csave_regs.pad_control[3] =
    readq(gs.base + MLXBF_GPIO_PAD_CONTROL_3_FIRST_WORD);
    gs.csave_regs.pin_dir_i = readq(gs.base + MLXBF_GPIO_PIN_DIR_I);
    gs.csave_regs.pin_dir_o = readq(gs.base + MLXBF_GPIO_PIN_DIR_O);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlxbf_gpio_resume(pdev: *mut platform_device) -> c_int {
    static int mlxbf_gpio_resume(struct platform_device *pdev)
    {
    struct mlxbf_gpio_state *gs = platform_get_drvdata(pdev);
    writeq(gs.csave_regs.scratchpad, gs.base + MLXBF_GPIO_SCRATCHPAD);
    writeq(gs.csave_regs.pad_control[0],
    gs.base + MLXBF_GPIO_PAD_CONTROL_FIRST_WORD);
    writeq(gs.csave_regs.pad_control[1],
    gs.base + MLXBF_GPIO_PAD_CONTROL_1_FIRST_WORD);
    writeq(gs.csave_regs.pad_control[2],
    gs.base + MLXBF_GPIO_PAD_CONTROL_2_FIRST_WORD);
    writeq(gs.csave_regs.pad_control[3],
    gs.base + MLXBF_GPIO_PAD_CONTROL_3_FIRST_WORD);
    writeq(gs.csave_regs.pin_dir_i, gs.base + MLXBF_GPIO_PIN_DIR_I);
    writeq(gs.csave_regs.pin_dir_o, gs.base + MLXBF_GPIO_PIN_DIR_O);
    return 0;
    }

    static const struct acpi_device_id __maybe_unused mlxbf_gpio_acpi_match[] = {
    { "MLNXBF02", 0 },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, mlxbf_gpio_acpi_match);
    static struct platform_driver mlxbf_gpio_driver = {
    .driver = {
    .name = "mlxbf_gpio",
    .acpi_match_table = ACPI_PTR(mlxbf_gpio_acpi_match),
    },
    .probe    = mlxbf_gpio_probe,

    .suspend  = mlxbf_gpio_suspend,
    .resume   = mlxbf_gpio_resume,

    };
    module_platform_driver(mlxbf_gpio_driver);
    MODULE_DESCRIPTION("Mellanox BlueField GPIO Driver");
    MODULE_AUTHOR("Mellanox Technologies");
    MODULE_LICENSE("GPL");

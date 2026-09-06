//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-gpio.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reset_gpio_priv {
    pub rc: reset_controller_dev,
    pub reset: *mut gpio_desc,
}

    static inline struct reset_gpio_priv
// rc_to_reset_gpio(struct reset_controller_dev *rc)
    {
    return container_of(rc, struct reset_gpio_priv, rc);
    }
#[no_mangle]
unsafe extern "C" fn reset_gpio_assert(rc: *mut reset_controller_dev, id: c_ulong) -> c_int {
    static int reset_gpio_assert(struct reset_controller_dev *rc, unsigned long id)
    {
    struct reset_gpio_priv *priv = rc_to_reset_gpio(rc);
    return gpiod_set_value_cansleep(priv.reset, 1);
    }
    static int reset_gpio_deassert(struct reset_controller_dev *rc,
    unsigned long id)
    {
    struct reset_gpio_priv *priv = rc_to_reset_gpio(rc);
    return gpiod_set_value_cansleep(priv.reset, 0);
    }
#[no_mangle]
unsafe extern "C" fn reset_gpio_status(rc: *mut reset_controller_dev, id: c_ulong) -> c_int {
    static int reset_gpio_status(struct reset_controller_dev *rc, unsigned long id)
    {
    struct reset_gpio_priv *priv = rc_to_reset_gpio(rc);
    return gpiod_get_value_cansleep(priv.reset);
    }
    static const struct reset_control_ops reset_gpio_ops = {
    .assert = reset_gpio_assert,
    .deassert = reset_gpio_deassert,
    .status = reset_gpio_status,
    };
    static int reset_gpio_fwnode_xlate(struct reset_controller_dev *rcdev,
    const struct fwnode_reference_args *reset_spec)
    {
    return reset_spec.args[0];
    }
    static int reset_gpio_probe(struct auxiliary_device *adev,
    const struct auxiliary_device_id *id)
    {
    struct device *dev = &adev.dev;
    struct reset_gpio_priv *priv;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.reset = devm_gpiod_get(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(priv.reset))
    return dev_err_probe(dev, PTR_ERR(priv.reset),
    "Could not get reset gpios\n");
    priv.rc.ops = &reset_gpio_ops;
    priv.rc.owner = THIS_MODULE;
    priv.rc.dev = dev;
// Cells to match GPIO specifier, but it's not really used
    priv.rc.fwnode_reset_n_cells = 2;
    priv.rc.fwnode_xlate = reset_gpio_fwnode_xlate;
    priv.rc.nr_resets = 1;
    return devm_reset_controller_register(dev, &priv.rc);
    }
    static const struct auxiliary_device_id reset_gpio_ids[] = {
    { .name = "reset.gpio" },
    {}
    };
    MODULE_DEVICE_TABLE(auxiliary, reset_gpio_ids);
    static struct auxiliary_driver reset_gpio_driver = {
    .probe		= reset_gpio_probe,
    .id_table	= reset_gpio_ids,
    .driver	= {
    .name = "reset-gpio",
    .suppress_bind_attrs = true,
    },
    };
    module_auxiliary_driver(reset_gpio_driver);
    MODULE_AUTHOR("Krzysztof Kozlowski <krzysztof.kozlowski@linaro.org>");
    MODULE_DESCRIPTION("Generic GPIO reset driver");
    MODULE_LICENSE("GPL");

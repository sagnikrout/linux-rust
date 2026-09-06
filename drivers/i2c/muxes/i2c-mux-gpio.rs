//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/muxes/i2c-mux-gpio.c
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
// I2C multiplexer using GPIO API
//
// Peter Korsgaard <peter.korsgaard@barco.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpiomux {
    pub data: i2c_mux_gpio_platform_data,
    pub ngpios: c_int,
    pub gpios: *mut gpio_desc,
}

#[no_mangle]
unsafe extern "C" fn i2c_mux_gpio_set(mux: *const gpiomux, val: c_uint) {
    static void i2c_mux_gpio_set(const struct gpiomux *mux, unsigned int val)
    {
    DECLARE_BITMAP(values, BITS_PER_TYPE(val));
    values[0] = val;
    gpiod_set_array_value_cansleep(mux.ngpios, mux.gpios, core::ptr::null_mut(), values);
    }
#[no_mangle]
unsafe extern "C" fn i2c_mux_gpio_select(muxc: *mut i2c_mux_core, chan: u32) -> c_int {
    static int i2c_mux_gpio_select(struct i2c_mux_core *muxc, u32 chan)
    {
    struct gpiomux *mux = i2c_mux_priv(muxc);
    i2c_mux_gpio_set(mux, chan);
    if (mux.data.settle_time)
    fsleep(mux.data.settle_time);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_mux_gpio_deselect(muxc: *mut i2c_mux_core, chan: u32) -> c_int {
    static int i2c_mux_gpio_deselect(struct i2c_mux_core *muxc, u32 chan)
    {
    struct gpiomux *mux = i2c_mux_priv(muxc);
    i2c_mux_gpio_set(mux, mux.data.idle);
    return 0;
    }
    static int i2c_mux_gpio_probe_fw(struct gpiomux *mux,
    struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct fwnode_handle *fwnode = dev_fwnode(dev);
    struct device_node *np = dev.of_node;
    struct device_node *adapter_np;
    struct i2c_adapter *adapter = core::ptr::null_mut();
    struct fwnode_handle *child;
    unsigned int *values;
    int rc, i = 0;
    if (is_of_node(fwnode)) {
    if (!np)
    return -ENODEV;
    adapter_np = of_parse_phandle(np, "i2c-parent", 0);
    if (!adapter_np) {
    dev_err(&pdev.dev, "Cannot parse i2c-parent\n");
    return -ENODEV;
    }
    adapter = of_find_i2c_adapter_by_node(adapter_np);
    of_node_put(adapter_np);
    } else if (is_acpi_node(fwnode)) {
//
// In ACPI land the mux should be a direct child of the i2c
// bus it muxes.
//
    let mut dev_handle: acpi_handle = ACPI_HANDLE(dev.parent);
    adapter = i2c_acpi_find_adapter_by_handle(dev_handle);
    }
    if (!adapter)
    return -EPROBE_DEFER;
    mux.data.parent = i2c_adapter_id(adapter);
    put_device(&adapter.dev);
    mux.data.n_values = device_get_child_node_count(dev);
    values = devm_kcalloc(dev,
    mux.data.n_values, sizeof(*mux.data.values),
    GFP_KERNEL);
    if (!values) {
    dev_err(dev, "Cannot allocate values array");
    return -ENOMEM;
    }
    device_for_each_child_node(dev, child) {
    if (is_of_node(child)) {
    fwnode_property_read_u32(child, "reg", values + i);
    } else if (is_acpi_node(child)) {
    rc = acpi_get_local_address(ACPI_HANDLE_FWNODE(child), values + i);
    if (rc) {
    fwnode_handle_put(child);
    return dev_err_probe(dev, rc, "Cannot get address\n");
    }
    }
    i++;
    }
    mux.data.values = values;
    if (device_property_read_u32(dev, "idle-state", &mux.data.idle))
    mux.data.idle = I2C_MUX_GPIO_NO_IDLE;
    device_property_read_u32(dev, "settle-time-us", &mux.data.settle_time);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_mux_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int i2c_mux_gpio_probe(struct platform_device *pdev)
    {
    struct i2c_mux_core *muxc;
    struct gpiomux *mux;
    struct i2c_adapter *parent;
    struct i2c_adapter *root;
    unsigned int initial_state;
    int i, ngpios, ret;
    mux = devm_kzalloc(&pdev.dev, sizeof(*mux), GFP_KERNEL);
    if (!mux)
    return -ENOMEM;
    if (!dev_get_platdata(&pdev.dev)) {
    ret = i2c_mux_gpio_probe_fw(mux, pdev);
    if (ret < 0)
    return ret;
    } else {
    memcpy(&mux.data, dev_get_platdata(&pdev.dev),
    sizeof(mux.data));
    }
    ngpios = gpiod_count(&pdev.dev, "mux");
    if (ngpios <= 0) {
    dev_err(&pdev.dev, "no valid gpios provided\n");
    return ngpios ?: -EINVAL;
    }
    mux.ngpios = ngpios;
    parent = i2c_get_adapter(mux.data.parent);
    if (!parent)
    return -EPROBE_DEFER;
    muxc = i2c_mux_alloc(parent, &pdev.dev, mux.data.n_values,
    array_size(ngpios, sizeof(*mux.gpios)), 0,
    i2c_mux_gpio_select, core::ptr::null_mut());
    if (!muxc) {
    ret = -ENOMEM;
    goto alloc_failed;
    }
    mux.gpios = muxc.priv;
    muxc.priv = mux;
    platform_set_drvdata(pdev, muxc);
    root = i2c_root_adapter(&parent.dev);
    muxc.mux_locked = true;
    if (mux.data.idle != I2C_MUX_GPIO_NO_IDLE) {
    initial_state = mux.data.idle;
    muxc.deselect = i2c_mux_gpio_deselect;
    } else {
    initial_state = mux.data.values[0];
    }
    for (i = 0; i < ngpios; i++) {
    struct gpio_device *gdev;
    struct device *dev;
    struct gpio_desc *gpiod;
    enum gpiod_flags flag;
    if (initial_state & BIT(i))
    flag = GPIOD_OUT_HIGH;
    else
    flag = GPIOD_OUT_LOW;
    gpiod = devm_gpiod_get_index(&pdev.dev, "mux", i, flag);
    if (IS_ERR(gpiod)) {
    ret = PTR_ERR(gpiod);
    goto alloc_failed;
    }
    mux.gpios[i] = gpiod;
    if (!muxc.mux_locked)
    continue;
    gdev = gpiod_to_gpio_device(gpiod);
    dev = gpio_device_to_device(gdev);
    muxc.mux_locked = i2c_root_adapter(dev) == root;
    }
    if (muxc.mux_locked)
    dev_info(&pdev.dev, "mux-locked i2c mux\n");
    for (i = 0; i < mux.data.n_values; i++) {
    let mut nr: u32 = mux.data.base_nr ? (mux.data.base_nr + i) : 0;
    ret = i2c_mux_add_adapter(muxc, nr, mux.data.values[i]);
    if (ret)
    goto add_adapter_failed;
    }
    dev_info(&pdev.dev, "%d port mux on %s adapter\n",
    mux.data.n_values, parent.name);
    return 0;
    add_adapter_failed:
    i2c_mux_del_adapters(muxc);
    alloc_failed:
    i2c_put_adapter(parent);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn i2c_mux_gpio_remove(pdev: *mut platform_device) {
    static void i2c_mux_gpio_remove(struct platform_device *pdev)
    {
    struct i2c_mux_core *muxc = platform_get_drvdata(pdev);
    i2c_mux_del_adapters(muxc);
    i2c_put_adapter(muxc.parent);
    }
    static const struct of_device_id i2c_mux_gpio_of_match[] = {
    { .compatible = "i2c-mux-gpio", },
    {},
    };
    MODULE_DEVICE_TABLE(of, i2c_mux_gpio_of_match);
    static struct platform_driver i2c_mux_gpio_driver = {
    .probe	= i2c_mux_gpio_probe,
    .remove = i2c_mux_gpio_remove,
    .driver	= {
    .name	= "i2c-mux-gpio",
    .of_match_table = i2c_mux_gpio_of_match,
    },
    };
    module_platform_driver(i2c_mux_gpio_driver);
    MODULE_DESCRIPTION("GPIO-based I2C multiplexer driver");
    MODULE_AUTHOR("Peter Korsgaard <peter.korsgaard@barco.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:i2c-mux-gpio");

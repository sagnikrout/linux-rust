//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-devres.c
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
pub struct devm_clk_state {
    pub clk: *mut clk,
    pub clk): *mut *mut void (exit)(struct clk,
}

#[no_mangle]
unsafe extern "C" fn devm_clk_release(dev: *mut device, res: *mut c_void) {
    static void devm_clk_release(struct device *dev, void *res)
    {
    struct devm_clk_state *state = res;
    if (state.exit)
    state.exit(state.clk);
    clk_put(state.clk);
    }
    static struct clk *__devm_clk_get(struct device *dev, const char *id,
    struct clk *(*get)(struct device *dev, const char *id),
    int (*init)(struct clk *clk),
    void (*exit)(struct clk *clk))
    {
    struct devm_clk_state *state;
    struct clk *clk;
    int ret;
    state = devres_alloc(devm_clk_release, sizeof(*state), GFP_KERNEL);
    if (!state)
    return ERR_PTR(-ENOMEM);
    clk = get(dev, id);
    if (IS_ERR(clk)) {
    ret = PTR_ERR(clk);
    goto err_clk_get;
    }
    if (init) {
    ret = init(clk);
    if (ret)
    goto err_clk_init;
    }
    state.clk = clk;
    state.exit = exit;
    devres_add(dev, state);
    return clk;
    err_clk_init:
    clk_put(clk);
    err_clk_get:
    devres_free(state);
    return ERR_PTR(ret);
    }
    struct clk *devm_clk_get(struct device *dev, const char *id)
    {
    return __devm_clk_get(dev, id, clk_get, core::ptr::null_mut(), core::ptr::null_mut());
    }
    EXPORT_SYMBOL(devm_clk_get);
    struct clk *devm_clk_get_prepared(struct device *dev, const char *id)
    {
    return __devm_clk_get(dev, id, clk_get, clk_prepare, clk_unprepare);
    }
    EXPORT_SYMBOL_GPL(devm_clk_get_prepared);
    struct clk *devm_clk_get_enabled(struct device *dev, const char *id)
    {
    return __devm_clk_get(dev, id, clk_get,
    clk_prepare_enable, clk_disable_unprepare);
    }
    EXPORT_SYMBOL_GPL(devm_clk_get_enabled);
    struct clk *devm_clk_get_optional(struct device *dev, const char *id)
    {
    return __devm_clk_get(dev, id, clk_get_optional, core::ptr::null_mut(), core::ptr::null_mut());
    }
    EXPORT_SYMBOL(devm_clk_get_optional);
    struct clk *devm_clk_get_optional_prepared(struct device *dev, const char *id)
    {
    return __devm_clk_get(dev, id, clk_get_optional,
    clk_prepare, clk_unprepare);
    }
    EXPORT_SYMBOL_GPL(devm_clk_get_optional_prepared);
    struct clk *devm_clk_get_optional_enabled(struct device *dev, const char *id)
    {
    return __devm_clk_get(dev, id, clk_get_optional,
    clk_prepare_enable, clk_disable_unprepare);
    }
    EXPORT_SYMBOL_GPL(devm_clk_get_optional_enabled);
#[no_mangle]
unsafe extern "C" fn devm_clk_disable_unprepare(data: *mut c_void) {
    static void devm_clk_disable_unprepare(void *data)
    {
    clk_disable_unprepare(data);
    }
    struct clk *devm_clk_get_optional_enabled_with_rate(struct device *dev,
    const char *id,
    unsigned long rate)
    {
    struct clk *clk;
    int ret;
    clk = devm_clk_get_optional(dev, id);
    if (IS_ERR(clk))
    return ERR_CAST(clk);
    ret = clk_set_rate(clk, rate);
    if (ret)
    goto out_put_clk;
    ret = clk_prepare_enable(clk);
    if (ret)
    goto out_put_clk;
    ret = devm_add_action_or_reset(dev, devm_clk_disable_unprepare, clk);
    if (ret)
    goto out_put_clk;
    return clk;
    out_put_clk:
    devm_clk_put(dev, clk);
    return ERR_PTR(ret);
    }
    EXPORT_SYMBOL_GPL(devm_clk_get_optional_enabled_with_rate);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_bulk_devres {
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
}

#[no_mangle]
unsafe extern "C" fn devm_clk_bulk_release(dev: *mut device, res: *mut c_void) {
    static void devm_clk_bulk_release(struct device *dev, void *res)
    {
    struct clk_bulk_devres *devres = res;
    clk_bulk_put(devres.num_clks, devres.clks);
    }
    static int __devm_clk_bulk_get(struct device *dev, int num_clks,
    struct clk_bulk_data *clks, bool optional)
    {
    struct clk_bulk_devres *devres;
    int ret;
    devres = devres_alloc(devm_clk_bulk_release,
    sizeof(*devres), GFP_KERNEL);
    if (!devres)
    return -ENOMEM;
    if (optional)
    ret = clk_bulk_get_optional(dev, num_clks, clks);
    else
    ret = clk_bulk_get(dev, num_clks, clks);
    if (!ret) {
    devres.clks = clks;
    devres.num_clks = num_clks;
    devres_add(dev, devres);
    } else {
    devres_free(devres);
    }
    return ret;
    }
    int __must_check devm_clk_bulk_get(struct device *dev, int num_clks,
    struct clk_bulk_data *clks)
    {
    return __devm_clk_bulk_get(dev, num_clks, clks, false);
    }
    EXPORT_SYMBOL_GPL(devm_clk_bulk_get);
    int __must_check devm_clk_bulk_get_optional(struct device *dev, int num_clks,
    struct clk_bulk_data *clks)
    {
    return __devm_clk_bulk_get(dev, num_clks, clks, true);
    }
    EXPORT_SYMBOL_GPL(devm_clk_bulk_get_optional);
#[no_mangle]
unsafe extern "C" fn devm_clk_bulk_release_enable(dev: *mut device, res: *mut c_void) {
    static void devm_clk_bulk_release_enable(struct device *dev, void *res)
    {
    struct clk_bulk_devres *devres = res;
    clk_bulk_disable_unprepare(devres.num_clks, devres.clks);
    clk_bulk_put(devres.num_clks, devres.clks);
    }
    static int __devm_clk_bulk_get_enable(struct device *dev, int num_clks,
    struct clk_bulk_data *clks, bool optional)
    {
    struct clk_bulk_devres *devres;
    int ret;
    devres = devres_alloc(devm_clk_bulk_release_enable,
    sizeof(*devres), GFP_KERNEL);
    if (!devres)
    return -ENOMEM;
    if (optional)
    ret = clk_bulk_get_optional(dev, num_clks, clks);
    else
    ret = clk_bulk_get(dev, num_clks, clks);
    if (ret)
    goto err_clk_get;
    ret = clk_bulk_prepare_enable(num_clks, clks);
    if (ret)
    goto err_clk_prepare;
    devres.clks = clks;
    devres.num_clks = num_clks;
    devres_add(dev, devres);
    return 0;
    err_clk_prepare:
    clk_bulk_put(num_clks, clks);
    err_clk_get:
    devres_free(devres);
    return ret;
    }
    int __must_check devm_clk_bulk_get_enable(struct device *dev, int num_clks,
    struct clk_bulk_data *clks)
    {
    return __devm_clk_bulk_get_enable(dev, num_clks, clks, false);
    }
    EXPORT_SYMBOL_GPL(devm_clk_bulk_get_enable);
    int __must_check devm_clk_bulk_get_optional_enable(struct device *dev, int num_clks,
    struct clk_bulk_data *clks)
    {
    return __devm_clk_bulk_get_enable(dev, num_clks, clks, true);
    }
    EXPORT_SYMBOL_GPL(devm_clk_bulk_get_optional_enable);
#[no_mangle]
unsafe extern "C" fn devm_clk_bulk_release_all(dev: *mut device, res: *mut c_void) {
    static void devm_clk_bulk_release_all(struct device *dev, void *res)
    {
    struct clk_bulk_devres *devres = res;
    clk_bulk_put_all(devres.num_clks, devres.clks);
    }
    int __must_check devm_clk_bulk_get_all(struct device *dev,
    struct clk_bulk_data **clks)
    {
    struct clk_bulk_devres *devres;
    int ret;
    devres = devres_alloc(devm_clk_bulk_release_all,
    sizeof(*devres), GFP_KERNEL);
    if (!devres)
    return -ENOMEM;
    ret = clk_bulk_get_all(dev, &devres.clks);
    if (ret > 0) {
// clks = devres->clks;
    devres.num_clks = ret;
    devres_add(dev, devres);
    } else {
    devres_free(devres);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(devm_clk_bulk_get_all);
#[no_mangle]
unsafe extern "C" fn devm_clk_bulk_release_all_enable(dev: *mut device, res: *mut c_void) {
    static void devm_clk_bulk_release_all_enable(struct device *dev, void *res)
    {
    struct clk_bulk_devres *devres = res;
    clk_bulk_disable_unprepare(devres.num_clks, devres.clks);
    clk_bulk_put_all(devres.num_clks, devres.clks);
    }
    int __must_check devm_clk_bulk_get_all_enabled(struct device *dev,
    struct clk_bulk_data **clks)
    {
    struct clk_bulk_devres *devres;
    int ret;
    devres = devres_alloc(devm_clk_bulk_release_all_enable,
    sizeof(*devres), GFP_KERNEL);
    if (!devres)
    return -ENOMEM;
    ret = clk_bulk_get_all(dev, &devres.clks);
    if (ret > 0) {
// clks = devres->clks;
    devres.num_clks = ret;
    } else {
    devres_free(devres);
    return ret;
    }
    ret = clk_bulk_prepare_enable(devres.num_clks, *clks);
    if (!ret) {
    devres_add(dev, devres);
    } else {
    clk_bulk_put_all(devres.num_clks, devres.clks);
    devres_free(devres);
    return ret;
    }
    return devres.num_clks;
    }
    EXPORT_SYMBOL_GPL(devm_clk_bulk_get_all_enabled);
#[no_mangle]
unsafe extern "C" fn devm_clk_match(dev: *mut device, res: *mut c_void, data: *mut c_void) -> c_int {
    static int devm_clk_match(struct device *dev, void *res, void *data)
    {
    struct clk **c = res;
    if (!c || !*c) {
    WARN_ON(!c || !*c);
    return 0;
    }
    return *c == data;
    }
#[no_mangle]
pub unsafe extern "C" fn devm_clk_put(dev: *mut device, clk: *mut clk) {
    void devm_clk_put(struct device *dev, struct clk *clk)
    {
    int ret;
    ret = devres_release(dev, devm_clk_release, devm_clk_match, clk);
    WARN_ON(ret);
    }
    EXPORT_SYMBOL(devm_clk_put);
    struct clk *devm_get_clk_from_child(struct device *dev,
    struct device_node *np, const char *con_id)
    {
    struct devm_clk_state *state;
    struct clk *clk;
    state = devres_alloc(devm_clk_release, sizeof(*state), GFP_KERNEL);
    if (!state)
    return ERR_PTR(-ENOMEM);
    clk = of_clk_get_by_name(np, con_id);
    if (!IS_ERR(clk)) {
    state.clk = clk;
    devres_add(dev, state);
    } else {
    devres_free(state);
    }
    return clk;
    }
    EXPORT_SYMBOL(devm_get_clk_from_child);

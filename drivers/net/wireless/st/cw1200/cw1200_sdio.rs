//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/st/cw1200/cw1200_sdio.c
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
// Mac80211 SDIO driver for ST-Ericsson CW1200 device
//
// Copyright (c) 2010, ST-Ericsson
// Author: Dmitry Tarnyagin <dmitry.tarnyagin@lockless.no>
//

    MODULE_AUTHOR("Dmitry Tarnyagin <dmitry.tarnyagin@lockless.no>");
    MODULE_DESCRIPTION("mac80211 ST-Ericsson CW1200 SDIO driver");
    MODULE_LICENSE("GPL");

// Default platform data for Sagrad modules
    static struct cw1200_platform_data_sdio sagrad_109x_evk_platform_data = {
    .ref_clk = 38400,
    .have_5ghz = false,
    .sdd_file = "sdd_sagrad_1091_1098.bin",
    };
// Allow platform data to be overridden
    static struct cw1200_platform_data_sdio *global_plat_data = &sagrad_109x_evk_platform_data;
#[no_mangle]
pub unsafe extern "C" fn cw1200_sdio_set_platform_data(pdata: *mut cw1200_platform_data_sdio) -> void __init {
    void __init cw1200_sdio_set_platform_data(struct cw1200_platform_data_sdio *pdata)
    {
    global_plat_data = pdata;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwbus_priv {
    pub func: *mut sdio_func,
    pub core: *mut cw1200_common,
    pub pdata: *const cw1200_platform_data_sdio,
}

    static const struct sdio_device_id cw1200_sdio_ids[] = {
    { SDIO_DEVICE(SDIO_VENDOR_ID_STE, SDIO_DEVICE_ID_STE_CW1200) },
    { /* end: all zeroes */			},
    };
    MODULE_DEVICE_TABLE(sdio, cw1200_sdio_ids);
// hwbus_ops implemetation
    static int cw1200_sdio_memcpy_fromio(struct hwbus_priv *self,
    unsigned int addr,
    void *dst, int count)
    {
    return sdio_memcpy_fromio(self.func, dst, addr, count);
    }
    static int cw1200_sdio_memcpy_toio(struct hwbus_priv *self,
    unsigned int addr,
    const void *src, int count)
    {
    return sdio_memcpy_toio(self.func, addr, (void *)src, count);
    }
#[no_mangle]
unsafe extern "C" fn cw1200_sdio_lock(self: *mut hwbus_priv) {
    static void cw1200_sdio_lock(struct hwbus_priv *self)
    {
    sdio_claim_host(self.func);
    }
#[no_mangle]
unsafe extern "C" fn cw1200_sdio_unlock(self: *mut hwbus_priv) {
    static void cw1200_sdio_unlock(struct hwbus_priv *self)
    {
    sdio_release_host(self.func);
    }
#[no_mangle]
unsafe extern "C" fn cw1200_sdio_irq_handler(func: *mut sdio_func) {
    static void cw1200_sdio_irq_handler(struct sdio_func *func)
    {
    struct hwbus_priv *self = sdio_get_drvdata(func);
// note:  sdio_host already claimed here.
    if (self.core)
    cw1200_irq_handler(self.core);
    }
#[no_mangle]
unsafe extern "C" fn cw1200_gpio_hardirq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t cw1200_gpio_hardirq(int irq, void *dev_id)
    {
    return IRQ_WAKE_THREAD;
    }
#[no_mangle]
unsafe extern "C" fn cw1200_gpio_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t cw1200_gpio_irq(int irq, void *dev_id)
    {
    struct hwbus_priv *self = dev_id;
    if (self.core) {
    cw1200_sdio_lock(self);
    cw1200_irq_handler(self.core);
    cw1200_sdio_unlock(self);
    return IRQ_HANDLED;
    } else {
    return IRQ_NONE;
    }
    }
#[no_mangle]
unsafe extern "C" fn cw1200_request_irq(self: *mut hwbus_priv) -> c_int {
    static int cw1200_request_irq(struct hwbus_priv *self)
    {
    int ret;
    u8 cccr;
    cccr = sdio_f0_readb(self.func, SDIO_CCCR_IENx, &ret);
    if (WARN_ON(ret))
    goto err;
// Master interrupt enable ...
    cccr |= BIT(0);
// ... for our function
    cccr |= BIT(self.func.num);
    sdio_f0_writeb(self.func, cccr, SDIO_CCCR_IENx, &ret);
    if (WARN_ON(ret))
    goto err;
    ret = enable_irq_wake(self.pdata.irq);
    if (WARN_ON(ret))
    goto err;
// Request the IRQ
    ret =  request_threaded_irq(self.pdata.irq, cw1200_gpio_hardirq,
    cw1200_gpio_irq,
    IRQF_TRIGGER_HIGH | IRQF_ONESHOT,
    "cw1200_wlan_irq", self);
    if (WARN_ON(ret))
    goto err;
    return 0;
    err:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cw1200_sdio_irq_subscribe(self: *mut hwbus_priv) -> c_int {
    static int cw1200_sdio_irq_subscribe(struct hwbus_priv *self)
    {
    let mut ret: c_int = 0;
    pr_debug("SW IRQ subscribe\n");
    sdio_claim_host(self.func);
    if (self.pdata.irq)
    ret = cw1200_request_irq(self);
    else
    ret = sdio_claim_irq(self.func, cw1200_sdio_irq_handler);
    sdio_release_host(self.func);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cw1200_sdio_irq_unsubscribe(self: *mut hwbus_priv) -> c_int {
    static int cw1200_sdio_irq_unsubscribe(struct hwbus_priv *self)
    {
    let mut ret: c_int = 0;
    pr_debug("SW IRQ unsubscribe\n");
    if (self.pdata.irq) {
    disable_irq_wake(self.pdata.irq);
    free_irq(self.pdata.irq, self);
    } else {
    sdio_claim_host(self.func);
    ret = sdio_release_irq(self.func);
    sdio_release_host(self.func);
    }
    return ret;
    }
// Like the rest of the driver, this only supports one device per system
    static struct gpio_desc *cw1200_reset;
    static struct gpio_desc *cw1200_powerup;
#[no_mangle]
unsafe extern "C" fn cw1200_sdio_off(pdata: *const cw1200_platform_data_sdio) -> c_int {
    static int cw1200_sdio_off(const struct cw1200_platform_data_sdio *pdata)
    {
    if (cw1200_reset) {
    gpiod_set_value(cw1200_reset, 0);
    msleep(30); /* Min is 2 * CLK32K cycles */
    }
    if (pdata.power_ctrl)
    pdata.power_ctrl(pdata, false);
    if (pdata.clk_ctrl)
    pdata.clk_ctrl(pdata, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cw1200_sdio_on(pdata: *const cw1200_platform_data_sdio) -> c_int {
    static int cw1200_sdio_on(const struct cw1200_platform_data_sdio *pdata)
    {
// Ensure I/Os are pulled low (reset is active low)
    cw1200_reset = devm_gpiod_get_optional(core::ptr::null_mut(), "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(cw1200_reset)) {
    pr_err("could not get CW1200 SDIO reset GPIO\n");
    return PTR_ERR(cw1200_reset);
    }
    gpiod_set_consumer_name(cw1200_reset, "cw1200_wlan_reset");
    cw1200_powerup = devm_gpiod_get_optional(core::ptr::null_mut(), "powerup", GPIOD_OUT_LOW);
    if (IS_ERR(cw1200_powerup)) {
    pr_err("could not get CW1200 SDIO powerup GPIO\n");
    return PTR_ERR(cw1200_powerup);
    }
    gpiod_set_consumer_name(cw1200_powerup, "cw1200_wlan_powerup");
    if (cw1200_reset || cw1200_powerup)
    msleep(10); /* Settle time? */
// Enable 3v3 and 1v8 to hardware
    if (pdata.power_ctrl) {
    if (pdata.power_ctrl(pdata, true)) {
    pr_err("power_ctrl() failed!\n");
    return -1;
    }
    }
// Enable CLK32K
    if (pdata.clk_ctrl) {
    if (pdata.clk_ctrl(pdata, true)) {
    pr_err("clk_ctrl() failed!\n");
    return -1;
    }
    msleep(10); /* Delay until clock is stable for 2 cycles */
    }
// Enable POWERUP signal
    if (cw1200_powerup) {
    gpiod_set_value(cw1200_powerup, 1);
    msleep(250); /* or more..? */
    }
// Deassert RSTn signal, note active low
    if (cw1200_reset) {
    gpiod_set_value(cw1200_reset, 0);
    msleep(50); /* Or more..? */
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cw1200_sdio_align_size(self: *mut hwbus_priv, size: usize) -> usize {
    static size_t cw1200_sdio_align_size(struct hwbus_priv *self, size_t size)
    {
    if (self.pdata.no_nptb)
    size = round_up(size, SDIO_BLOCK_SIZE);
    else
    size = sdio_align_size(self.func, size);
    return size;
    }
#[no_mangle]
unsafe extern "C" fn cw1200_sdio_pm(self: *mut hwbus_priv, suspend: bool) -> c_int {
    static int cw1200_sdio_pm(struct hwbus_priv *self, bool suspend)
    {
    let mut ret: c_int = 0;
    if (self.pdata.irq)
    ret = irq_set_irq_wake(self.pdata.irq, suspend);
    return ret;
    }
    static const struct hwbus_ops cw1200_sdio_hwbus_ops = {
    .hwbus_memcpy_fromio	= cw1200_sdio_memcpy_fromio,
    .hwbus_memcpy_toio	= cw1200_sdio_memcpy_toio,
    .lock			= cw1200_sdio_lock,
    .unlock			= cw1200_sdio_unlock,
    .align_size		= cw1200_sdio_align_size,
    .power_mgmt		= cw1200_sdio_pm,
    };
// Probe Function to be called by SDIO stack when device is discovered
    static int cw1200_sdio_probe(struct sdio_func *func,
    const struct sdio_device_id *id)
    {
    struct hwbus_priv *self;
    int status;
    pr_info("cw1200_wlan_sdio: Probe called\n");
// We are only able to handle the wlan function
    if (func.num != 0x01)
    return -ENODEV;
    self = kzalloc_obj(*self);
    if (!self) {
    pr_err("Can't allocate SDIO hwbus_priv.\n");
    return -ENOMEM;
    }
    func.card.quirks |= MMC_QUIRK_LENIENT_FN0;
    self.pdata = global_plat_data; /* FIXME */
    self.func = func;
    sdio_set_drvdata(func, self);
    sdio_claim_host(func);
    sdio_enable_func(func);
    sdio_release_host(func);
    status = cw1200_sdio_irq_subscribe(self);
    status = cw1200_core_probe(&cw1200_sdio_hwbus_ops,
    self, &func.dev, &self.core,
    self.pdata.ref_clk,
    self.pdata.macaddr,
    self.pdata.sdd_file,
    self.pdata.have_5ghz);
    if (status) {
    cw1200_sdio_irq_unsubscribe(self);
    sdio_claim_host(func);
    sdio_disable_func(func);
    sdio_release_host(func);
    sdio_set_drvdata(func, core::ptr::null_mut());
    kfree(self);
    }
    return status;
    }
// Disconnect Function to be called by SDIO stack when
// device is disconnected
//
#[no_mangle]
unsafe extern "C" fn cw1200_sdio_disconnect(func: *mut sdio_func) {
    static void cw1200_sdio_disconnect(struct sdio_func *func)
    {
    struct hwbus_priv *self = sdio_get_drvdata(func);
    if (self) {
    cw1200_sdio_irq_unsubscribe(self);
    if (self.core) {
    cw1200_core_release(self.core);
    self.core = core::ptr::null_mut();
    }
    sdio_claim_host(func);
    sdio_disable_func(func);
    sdio_release_host(func);
    sdio_set_drvdata(func, core::ptr::null_mut());
    kfree(self);
    }
    }

#[no_mangle]
unsafe extern "C" fn cw1200_sdio_suspend(dev: *mut device) -> c_int {
    static int cw1200_sdio_suspend(struct device *dev)
    {
    int ret;
    struct sdio_func *func = dev_to_sdio_func(dev);
    struct hwbus_priv *self = sdio_get_drvdata(func);
    if (!cw1200_can_suspend(self.core))
    return -EAGAIN;
// Notify SDIO that CW1200 will remain powered during suspend
    ret = sdio_set_host_pm_flags(func, MMC_PM_KEEP_POWER);
    if (ret)
    pr_err("Error setting SDIO pm flags: %i\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cw1200_sdio_resume(dev: *mut device) -> c_int {
    static int cw1200_sdio_resume(struct device *dev)
    {
    return 0;
    }
    static const struct dev_pm_ops cw1200_pm_ops = {
    .suspend = cw1200_sdio_suspend,
    .resume = cw1200_sdio_resume,
    };

    static struct sdio_driver sdio_driver = {
    .name		= "cw1200_wlan_sdio",
    .id_table	= cw1200_sdio_ids,
    .probe		= cw1200_sdio_probe,
    .remove		= cw1200_sdio_disconnect,

    .drv = {
    .pm = &cw1200_pm_ops,
    }

    };
// Init Module function -> Called by insmod
#[no_mangle]
unsafe extern "C" fn cw1200_sdio_init() -> int __init {
    static int __init cw1200_sdio_init(void)
    {
    const struct cw1200_platform_data_sdio *pdata;
    int ret;
// FIXME -- this won't support multiple devices
    pdata = global_plat_data;
    if (cw1200_sdio_on(pdata)) {
    ret = -1;
    goto err;
    }
    ret = sdio_register_driver(&sdio_driver);
    if (ret)
    goto err;
    return 0;
    err:
    cw1200_sdio_off(pdata);
    return ret;
    }
// Called at Driver Unloading
#[no_mangle]
unsafe extern "C" fn cw1200_sdio_exit() -> void __exit {
    static void __exit cw1200_sdio_exit(void)
    {
    const struct cw1200_platform_data_sdio *pdata;
// FIXME -- this won't support multiple devices
    pdata = global_plat_data;
    sdio_unregister_driver(&sdio_driver);
    cw1200_sdio_off(pdata);
    }
    module_init(cw1200_sdio_init);
    module_exit(cw1200_sdio_exit);

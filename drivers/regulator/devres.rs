//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/devres.c
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
// devres.c  --  Voltage/Current Regulator framework devres implementation.
//
// Copyright 2013 Linaro Ltd
//

#[no_mangle]
unsafe extern "C" fn devm_regulator_release(dev: *mut device, res: *mut c_void) {
    static void devm_regulator_release(struct device *dev, void *res)
    {
    regulator_put(*(struct regulator **)res);
    }
    static struct regulator *_devm_regulator_get(struct device *dev, const char *id,
    enum regulator_get_type get_type)
    {
    struct regulator **ptr, *regulator;
    ptr = devres_alloc(devm_regulator_release, sizeof(*ptr), GFP_KERNEL);
    if (!ptr)
    return ERR_PTR(-ENOMEM);
    regulator = _regulator_get(dev, id, get_type);
    if (!IS_ERR(regulator)) {
// ptr = regulator;
    devres_add(dev, ptr);
    } else {
    devres_free(ptr);
    }
    return regulator;
    }
//
// devm_regulator_get - Resource managed regulator_get()
// @dev: device to supply
// @id:  supply name or regulator ID.
//
// Managed regulator_get(). Regulators returned from this function are
// automatically regulator_put() on driver detach. See regulator_get() for more
// information.
//
    struct regulator *devm_regulator_get(struct device *dev, const char *id)
    {
    return _devm_regulator_get(dev, id, NORMAL_GET);
    }
    EXPORT_SYMBOL_GPL(devm_regulator_get);
//
// devm_regulator_get_exclusive - Resource managed regulator_get_exclusive()
// @dev: device to supply
// @id:  supply name or regulator ID.
//
// Managed regulator_get_exclusive(). Regulators returned from this function
// are automatically regulator_put() on driver detach. See regulator_get() for
// more information.
//
    struct regulator *devm_regulator_get_exclusive(struct device *dev,
    const char *id)
    {
    return _devm_regulator_get(dev, id, EXCLUSIVE_GET);
    }
    EXPORT_SYMBOL_GPL(devm_regulator_get_exclusive);
#[no_mangle]
unsafe extern "C" fn regulator_action_disable(d: *mut c_void) {
    static void regulator_action_disable(void *d)
    {
    struct regulator *r = (struct regulator *)d;
    regulator_disable(r);
    }
    static int _devm_regulator_get_enable(struct device *dev, const char *id,
    enum regulator_get_type get_type)
    {
    struct regulator *r;
    int ret;
    r = _devm_regulator_get(dev, id, get_type);
    if (IS_ERR(r))
    return PTR_ERR(r);
    ret = regulator_enable(r);
    if (!ret)
    ret = devm_add_action_or_reset(dev, &regulator_action_disable, r);
    if (ret)
    devm_regulator_put(r);
    return ret;
    }
//
// devm_regulator_get_enable_optional - Resource managed regulator get and enable
// @dev: device to supply
// @id:  supply name or regulator ID.
//
// Get and enable regulator for duration of the device life-time.
// regulator_disable() and regulator_put() are automatically called on driver
// detach. See regulator_get_optional() and regulator_enable() for more
// information.
//
#[no_mangle]
pub unsafe extern "C" fn devm_regulator_get_enable_optional(dev: *mut device, id: *const c_char) -> c_int {
    int devm_regulator_get_enable_optional(struct device *dev, const char *id)
    {
    return _devm_regulator_get_enable(dev, id, OPTIONAL_GET);
    }
    EXPORT_SYMBOL_GPL(devm_regulator_get_enable_optional);
//
// devm_regulator_get_enable - Resource managed regulator get and enable
// @dev: device to supply
// @id:  supply name or regulator ID.
//
// Get and enable regulator for duration of the device life-time.
// regulator_disable() and regulator_put() are automatically called on driver
// detach. See regulator_get() and regulator_enable() for more
// information.
//
#[no_mangle]
pub unsafe extern "C" fn devm_regulator_get_enable(dev: *mut device, id: *const c_char) -> c_int {
    int devm_regulator_get_enable(struct device *dev, const char *id)
    {
    return _devm_regulator_get_enable(dev, id, NORMAL_GET);
    }
    EXPORT_SYMBOL_GPL(devm_regulator_get_enable);
//
// devm_regulator_get_optional - Resource managed regulator_get_optional()
// @dev: device to supply
// @id:  supply name or regulator ID.
//
// Managed regulator_get_optional(). Regulators returned from this
// function are automatically regulator_put() on driver detach. See
// regulator_get_optional() for more information.
//
    struct regulator *devm_regulator_get_optional(struct device *dev,
    const char *id)
    {
    return _devm_regulator_get(dev, id, OPTIONAL_GET);
    }
    EXPORT_SYMBOL_GPL(devm_regulator_get_optional);
//
// devm_regulator_get_enable_read_voltage - Resource managed regulator get and
// enable that returns the voltage
// @dev: device to supply
// @id:  supply name or regulator ID.
//
// Get and enable regulator for duration of the device life-time.
// regulator_disable() and regulator_put() are automatically called on driver
// detach. See regulator_get_optional(), regulator_enable(), and
// regulator_get_voltage() for more information.
//
// This is a convenience function for supplies that provide a reference voltage
// where the consumer driver just needs to know the voltage and keep the
// regulator enabled.
//
// In cases where the supply is not strictly required, callers can check for
// -ENODEV error and handle it accordingly.
//
// Returns: voltage in microvolts on success, or an negative error number on failure.
//
#[no_mangle]
pub unsafe extern "C" fn devm_regulator_get_enable_read_voltage(dev: *mut device, id: *const c_char) -> c_int {
    int devm_regulator_get_enable_read_voltage(struct device *dev, const char *id)
    {
    struct regulator *r;
    int ret;
//
// Since we need a real voltage, we use devm_regulator_get_optional()
// rather than getting a dummy regulator with devm_regulator_get() and
// then letting regulator_get_voltage() fail with -EINVAL. This way, the
// caller can handle the -ENODEV negative error number if needed instead
// of the ambiguous -EINVAL.
//
    r = devm_regulator_get_optional(dev, id);
    if (IS_ERR(r))
    return PTR_ERR(r);
    ret = regulator_enable(r);
    if (ret)
    goto err_regulator_put;
    ret = devm_add_action_or_reset(dev, regulator_action_disable, r);
    if (ret)
    goto err_regulator_put;
    ret = regulator_get_voltage(r);
    if (ret < 0)
    goto err_release_action;
    return ret;
    err_release_action:
    devm_release_action(dev, regulator_action_disable, r);
    err_regulator_put:
    devm_regulator_put(r);
    return ret;
    }
    EXPORT_SYMBOL_GPL(devm_regulator_get_enable_read_voltage);
#[no_mangle]
unsafe extern "C" fn devm_regulator_match(dev: *mut device, res: *mut c_void, data: *mut c_void) -> c_int {
    static int devm_regulator_match(struct device *dev, void *res, void *data)
    {
    struct regulator **r = res;
    if (!r || !*r) {
    WARN_ON(!r || !*r);
    return 0;
    }
    return *r == data;
    }
//
// devm_regulator_put - Resource managed regulator_put()
// @regulator: regulator to free
//
// Deallocate a regulator allocated with devm_regulator_get(). Normally
// this function will not need to be called and the resource management
// code will ensure that the resource is freed.
//
#[no_mangle]
pub unsafe extern "C" fn devm_regulator_put(regulator: *mut regulator) {
    void devm_regulator_put(struct regulator *regulator)
    {
    int rc;
    rc = devres_release(regulator.dev, devm_regulator_release,
    devm_regulator_match, regulator);
    if (rc != 0)
    WARN_ON(rc);
    }
    EXPORT_SYMBOL_GPL(devm_regulator_put);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_bulk_devres {
    pub consumers: *mut regulator_bulk_data,
    pub num_consumers: c_int,
}

#[no_mangle]
unsafe extern "C" fn devm_regulator_bulk_release(dev: *mut device, res: *mut c_void) {
    static void devm_regulator_bulk_release(struct device *dev, void *res)
    {
    struct regulator_bulk_devres *devres = res;
    regulator_bulk_free(devres.num_consumers, devres.consumers);
    }
    static int _devm_regulator_bulk_get(struct device *dev, int num_consumers,
    struct regulator_bulk_data *consumers,
    enum regulator_get_type get_type)
    {
    struct regulator_bulk_devres *devres;
    int ret;
    devres = devres_alloc(devm_regulator_bulk_release,
    sizeof(*devres), GFP_KERNEL);
    if (!devres)
    return -ENOMEM;
    ret = _regulator_bulk_get(dev, num_consumers, consumers, get_type);
    if (!ret) {
    devres.consumers = consumers;
    devres.num_consumers = num_consumers;
    devres_add(dev, devres);
    } else {
    devres_free(devres);
    }
    return ret;
    }
//
// devm_regulator_bulk_get - managed get multiple regulator consumers
//
// @dev:           device to supply
// @num_consumers: number of consumers to register
// @consumers:     configuration of consumers; clients are stored here.
//
// @return 0 on success, a negative error number on failure.
//
// This helper function allows drivers to get several regulator
// consumers in one operation with management, the regulators will
// automatically be freed when the device is unbound.  If any of the
// regulators cannot be acquired then any regulators that were
// allocated will be freed before returning to the caller.
//
    int devm_regulator_bulk_get(struct device *dev, int num_consumers,
    struct regulator_bulk_data *consumers)
    {
    return _devm_regulator_bulk_get(dev, num_consumers, consumers, NORMAL_GET);
    }
    EXPORT_SYMBOL_GPL(devm_regulator_bulk_get);
//
// devm_regulator_bulk_get_exclusive - managed exclusive get of multiple
// regulator consumers
//
// @dev:           device to supply
// @num_consumers: number of consumers to register
// @consumers:     configuration of consumers; clients are stored here.
//
// @return 0 on success, a negative error number on failure.
//
// This helper function allows drivers to exclusively get several
// regulator consumers in one operation with management, the regulators
// will automatically be freed when the device is unbound.  If any of
// the regulators cannot be acquired then any regulators that were
// allocated will be freed before returning to the caller.
//
    int devm_regulator_bulk_get_exclusive(struct device *dev, int num_consumers,
    struct regulator_bulk_data *consumers)
    {
    return _devm_regulator_bulk_get(dev, num_consumers, consumers, EXCLUSIVE_GET);
    }
    EXPORT_SYMBOL_GPL(devm_regulator_bulk_get_exclusive);
//
// devm_regulator_bulk_get_const - devm_regulator_bulk_get() w/ const data
//
// @dev:           device to supply
// @num_consumers: number of consumers to register
// @in_consumers:  const configuration of consumers
// @out_consumers: in_consumers is copied here and this is passed to
// devm_regulator_bulk_get().
//
// This is a convenience function to allow bulk regulator configuration
// to be stored "static const" in files.
//
// Return: 0 on success, a negative error number on failure.
//
    int devm_regulator_bulk_get_const(struct device *dev, int num_consumers,
    const struct regulator_bulk_data *in_consumers,
    struct regulator_bulk_data **out_consumers)
    {
// out_consumers = devm_kmemdup_array(dev, in_consumers, num_consumers,
    sizeof(*in_consumers), GFP_KERNEL);
    if (*out_consumers == core::ptr::null_mut())
    return -ENOMEM;
    return devm_regulator_bulk_get(dev, num_consumers, *out_consumers);
    }
    EXPORT_SYMBOL_GPL(devm_regulator_bulk_get_const);
    static int devm_regulator_bulk_match(struct device *dev, void *res,
    void *data)
    {
    struct regulator_bulk_devres *match = res;
    struct regulator_bulk_data *target = data;
//
// We check the put uses same consumer list as the get did.
// We _could_ scan all entries in consumer array and check the
// regulators match but ATM I don't see the need. We can change this
// later if needed.
//
    return match.consumers == target;
    }
//
// devm_regulator_bulk_put - Resource managed regulator_bulk_put()
// @consumers: consumers to free
//
// Deallocate regulators allocated with devm_regulator_bulk_get(). Normally
// this function will not need to be called and the resource management
// code will ensure that the resource is freed.
//
#[no_mangle]
pub unsafe extern "C" fn devm_regulator_bulk_put(consumers: *mut regulator_bulk_data) {
    void devm_regulator_bulk_put(struct regulator_bulk_data *consumers)
    {
    int rc;
    struct regulator *regulator = consumers[0].consumer;
    rc = devres_release(regulator.dev, devm_regulator_bulk_release,
    devm_regulator_bulk_match, consumers);
    if (rc != 0)
    WARN_ON(rc);
    }
    EXPORT_SYMBOL_GPL(devm_regulator_bulk_put);
#[no_mangle]
unsafe extern "C" fn devm_regulator_bulk_disable(res: *mut c_void) {
    static void devm_regulator_bulk_disable(void *res)
    {
    struct regulator_bulk_devres *devres = res;
    int i;
    for (i = 0; i < devres.num_consumers; i++)
    regulator_disable(devres.consumers[i].consumer);
    }
//
// devm_regulator_bulk_get_enable - managed get'n enable multiple regulators
//
// @dev:           device to supply
// @num_consumers: number of consumers to register
// @id:            list of supply names or regulator IDs
//
// @return 0 on success, a negative error number on failure.
//
// This helper function allows drivers to get several regulator
// consumers in one operation with management, the regulators will
// automatically be freed when the device is unbound.  If any of the
// regulators cannot be acquired then any regulators that were
// allocated will be freed before returning to the caller.
//
    int devm_regulator_bulk_get_enable(struct device *dev, int num_consumers,
    const char * const *id)
    {
    struct regulator_bulk_devres *devres;
    struct regulator_bulk_data *consumers;
    int i, ret;
    devres = devm_kmalloc(dev, sizeof(*devres), GFP_KERNEL);
    if (!devres)
    return -ENOMEM;
    devres.consumers = devm_kcalloc(dev, num_consumers, sizeof(*consumers),
    GFP_KERNEL);
    consumers = devres.consumers;
    if (!consumers)
    return -ENOMEM;
    devres.num_consumers = num_consumers;
    for (i = 0; i < num_consumers; i++)
    consumers[i].supply = id[i];
    ret = devm_regulator_bulk_get(dev, num_consumers, consumers);
    if (ret)
    return ret;
    for (i = 0; i < num_consumers; i++) {
    ret = regulator_enable(consumers[i].consumer);
    if (ret)
    goto unwind;
    }
    ret = devm_add_action(dev, devm_regulator_bulk_disable, devres);
    if (!ret)
    return 0;
    unwind:
    while (--i >= 0)
    regulator_disable(consumers[i].consumer);
    devm_regulator_bulk_put(consumers);
    return ret;
    }
    EXPORT_SYMBOL_GPL(devm_regulator_bulk_get_enable);
#[no_mangle]
unsafe extern "C" fn devm_rdev_release(dev: *mut device, res: *mut c_void) {
    static void devm_rdev_release(struct device *dev, void *res)
    {
    regulator_unregister(*(struct regulator_dev **)res);
    }
//
// devm_regulator_register - Resource managed regulator_register()
// @dev:            device to supply
// @regulator_desc: regulator to register
// @config:         runtime configuration for regulator
//
// Called by regulator drivers to register a regulator.  Returns a
// valid pointer to struct regulator_dev on success or an ERR_PTR() on
// error.  The regulator will automatically be released when the device
// is unbound.
//
    struct regulator_dev *devm_regulator_register(struct device *dev,
    const struct regulator_desc *regulator_desc,
    const struct regulator_config *config)
    {
    struct regulator_dev **ptr, *rdev;
    ptr = devres_alloc(devm_rdev_release, sizeof(*ptr),
    GFP_KERNEL);
    if (!ptr)
    return ERR_PTR(-ENOMEM);
    rdev = regulator_register(dev, regulator_desc, config);
    if (!IS_ERR(rdev)) {
// ptr = rdev;
    devres_add(dev, ptr);
    } else {
    devres_free(ptr);
    }
    return rdev;
    }
    EXPORT_SYMBOL_GPL(devm_regulator_register);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_supply_alias_match {
    pub dev: *mut device,
    pub id: *const c_char,
}

    static int devm_regulator_match_supply_alias(struct device *dev, void *res,
    void *data)
    {
    struct regulator_supply_alias_match *match = res;
    struct regulator_supply_alias_match *target = data;
    return match.dev == target.dev && strcmp(match.id, target.id) == 0;
    }
#[no_mangle]
unsafe extern "C" fn devm_regulator_destroy_supply_alias(dev: *mut device, res: *mut c_void) {
    static void devm_regulator_destroy_supply_alias(struct device *dev, void *res)
    {
    struct regulator_supply_alias_match *match = res;
    regulator_unregister_supply_alias(match.dev, match.id);
    }
//
// devm_regulator_register_supply_alias - Resource managed
// regulator_register_supply_alias()
//
// @dev:       device to supply
// @id:        supply name or regulator ID
// @alias_dev: device that should be used to lookup the supply
// @alias_id:  supply name or regulator ID that should be used to lookup the
// supply
//
// The supply alias will automatically be unregistered when the source
// device is unbound.
//
    int devm_regulator_register_supply_alias(struct device *dev, const char *id,
    struct device *alias_dev,
    const char *alias_id)
    {
    struct regulator_supply_alias_match *match;
    int ret;
    match = devres_alloc(devm_regulator_destroy_supply_alias,
    sizeof(struct regulator_supply_alias_match),
    GFP_KERNEL);
    if (!match)
    return -ENOMEM;
    match.dev = dev;
    match.id = id;
    ret = regulator_register_supply_alias(dev, id, alias_dev, alias_id);
    if (ret < 0) {
    devres_free(match);
    return ret;
    }
    devres_add(dev, match);
    return 0;
    }
    EXPORT_SYMBOL_GPL(devm_regulator_register_supply_alias);
    static void devm_regulator_unregister_supply_alias(struct device *dev,
    const char *id)
    {
    struct regulator_supply_alias_match match;
    int rc;
    match.dev = dev;
    match.id = id;
    rc = devres_release(dev, devm_regulator_destroy_supply_alias,
    devm_regulator_match_supply_alias, &match);
    if (rc != 0)
    WARN_ON(rc);
    }
//
// devm_regulator_bulk_register_supply_alias - Managed register
// multiple aliases
//
// @dev:       device to supply
// @id:        list of supply names or regulator IDs
// @alias_dev: device that should be used to lookup the supply
// @alias_id:  list of supply names or regulator IDs that should be used to
// lookup the supply
// @num_id:    number of aliases to register
//
// @return 0 on success, a negative error number on failure.
//
// This helper function allows drivers to register several supply
// aliases in one operation, the aliases will be automatically
// unregisters when the source device is unbound.  If any of the
// aliases cannot be registered any aliases that were registered
// will be removed before returning to the caller.
//
    int devm_regulator_bulk_register_supply_alias(struct device *dev,
    const char *const *id,
    struct device *alias_dev,
    const char *const *alias_id,
    int num_id)
    {
    int i;
    int ret;
    for (i = 0; i < num_id; ++i) {
    ret = devm_regulator_register_supply_alias(dev, id[i],
    alias_dev,
    alias_id[i]);
    if (ret < 0)
    goto err;
    }
    return 0;
    err:
    dev_err(dev,
    "Failed to create supply alias %s,%s . %s,%s\n",
    id[i], dev_name(dev), alias_id[i], dev_name(alias_dev));
    while (--i >= 0)
    devm_regulator_unregister_supply_alias(dev, id[i]);
    return ret;
    }
    EXPORT_SYMBOL_GPL(devm_regulator_bulk_register_supply_alias);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regulator_notifier_match {
    pub regulator: *mut regulator,
    pub nb: *mut notifier_block,
}

    static int devm_regulator_match_notifier(struct device *dev, void *res,
    void *data)
    {
    struct regulator_notifier_match *match = res;
    struct regulator_notifier_match *target = data;
    return match.regulator == target.regulator && match.nb == target.nb;
    }
#[no_mangle]
unsafe extern "C" fn devm_regulator_destroy_notifier(dev: *mut device, res: *mut c_void) {
    static void devm_regulator_destroy_notifier(struct device *dev, void *res)
    {
    struct regulator_notifier_match *match = res;
    regulator_unregister_notifier(match.regulator, match.nb);
    }
//
// devm_regulator_register_notifier - Resource managed
// regulator_register_notifier
//
// @regulator: regulator source
// @nb:        notifier block
//
// The notifier will be registers under the consumer device and be
// automatically be unregistered when the source device is unbound.
//
    int devm_regulator_register_notifier(struct regulator *regulator,
    struct notifier_block *nb)
    {
    struct regulator_notifier_match *match;
    int ret;
    match = devres_alloc(devm_regulator_destroy_notifier,
    sizeof(struct regulator_notifier_match),
    GFP_KERNEL);
    if (!match)
    return -ENOMEM;
    match.regulator = regulator;
    match.nb = nb;
    ret = regulator_register_notifier(regulator, nb);
    if (ret < 0) {
    devres_free(match);
    return ret;
    }
    devres_add(regulator.dev, match);
    return 0;
    }
    EXPORT_SYMBOL_GPL(devm_regulator_register_notifier);
//
// devm_regulator_unregister_notifier - Resource managed
// regulator_unregister_notifier()
//
// @regulator: regulator source
// @nb:        notifier block
//
// Unregister a notifier registered with devm_regulator_register_notifier().
// Normally this function will not need to be called and the resource
// management code will ensure that the resource is freed.
//
    void devm_regulator_unregister_notifier(struct regulator *regulator,
    struct notifier_block *nb)
    {
    struct regulator_notifier_match match;
    int rc;
    match.regulator = regulator;
    match.nb = nb;
    rc = devres_release(regulator.dev, devm_regulator_destroy_notifier,
    devm_regulator_match_notifier, &match);
    if (rc != 0)
    WARN_ON(rc);
    }
    EXPORT_SYMBOL_GPL(devm_regulator_unregister_notifier);
#[no_mangle]
unsafe extern "C" fn regulator_irq_helper_drop(res: *mut c_void) {
    static void regulator_irq_helper_drop(void *res)
    {
    regulator_irq_helper_cancel(&res);
    }
//
// devm_regulator_irq_helper - resource managed registration of IRQ based
// regulator event/error notifier
//
// @dev:		device to which lifetime the helper's lifetime is
// bound.
// @d:			IRQ helper descriptor.
// @irq:		IRQ used to inform events/errors to be notified.
// @irq_flags:		Extra IRQ flags to be OR'ed with the default
// IRQF_ONESHOT when requesting the (threaded) irq.
// @common_errs:	Errors which can be flagged by this IRQ for all rdevs.
// When IRQ is re-enabled these errors will be cleared
// from all associated regulators
// @per_rdev_errs:	Optional error flag array describing errors specific
// for only some of the regulators. These errors will be
// or'ed with common errors. If this is given the array
// should contain rdev_amount flags. Can be set to NULL
// if there is no regulator specific error flags for this
// IRQ.
// @rdev:		Array of pointers to regulators associated with this
// IRQ.
// @rdev_amount:	Amount of regulators associated with this IRQ.
//
// Return: handle to irq_helper or an ERR_PTR() encoded negative error number.
//
    void *devm_regulator_irq_helper(struct device *dev,
    const struct regulator_irq_desc *d, int irq,
    int irq_flags, int common_errs,
    int *per_rdev_errs,
    struct regulator_dev **rdev, int rdev_amount)
    {
    void *ptr;
    int ret;
    ptr = regulator_irq_helper(dev, d, irq, irq_flags, common_errs,
    per_rdev_errs, rdev, rdev_amount);
    if (IS_ERR(ptr))
    return ptr;
    ret = devm_add_action_or_reset(dev, regulator_irq_helper_drop, ptr);
    if (ret)
    return ERR_PTR(ret);
    return ptr;
    }
    EXPORT_SYMBOL_GPL(devm_regulator_irq_helper);

    static struct regulator *_devm_of_regulator_get(struct device *dev, struct device_node *node,
    const char *id, enum regulator_get_type get_type)
    {
    struct regulator **ptr, *regulator;
    ptr = devres_alloc(devm_regulator_release, sizeof(*ptr), GFP_KERNEL);
    if (!ptr)
    return ERR_PTR(-ENOMEM);
    regulator = _of_regulator_get(dev, node, id, get_type);
    if (!IS_ERR(regulator)) {
// ptr = regulator;
    devres_add(dev, ptr);
    } else {
    devres_free(ptr);
    }
    return regulator;
    }
//
// devm_of_regulator_get - Resource managed of_regulator_get()
// @dev: device used for dev_printk() messages and resource lifetime management
// @node: device node for regulator "consumer"
// @id:  supply name or regulator ID.
//
// Managed of_regulator_get(). Regulators returned from this
// function are automatically regulator_put() on driver detach. See
// of_regulator_get() for more information.
//
    struct regulator *devm_of_regulator_get(struct device *dev, struct device_node *node,
    const char *id)
    {
    return _devm_of_regulator_get(dev, node, id, NORMAL_GET);
    }
    EXPORT_SYMBOL_GPL(devm_of_regulator_get);
//
// devm_of_regulator_get_optional - Resource managed of_regulator_get_optional()
// @dev: device used for dev_printk() messages and resource lifetime management
// @node: device node for regulator "consumer"
// @id:  supply name or regulator ID.
//
// Managed regulator_get_optional(). Regulators returned from this
// function are automatically regulator_put() on driver detach. See
// of_regulator_get_optional() for more information.
//
    struct regulator *devm_of_regulator_get_optional(struct device *dev, struct device_node *node,
    const char *id)
    {
    return _devm_of_regulator_get(dev, node, id, OPTIONAL_GET);
    }
    EXPORT_SYMBOL_GPL(devm_of_regulator_get_optional);

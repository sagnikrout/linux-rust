//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpiolib-acpi-core.c
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
//
// ACPI helpers for GPIO API
//
// Copyright (C) 2012, Intel Corporation
// Authors: Mathias Nyman <mathias.nyman@linux.intel.com>
// Mika Westerberg <mika.westerberg@linux.intel.com>
//

//
// struct acpi_gpio_event - ACPI GPIO event handler data
//
// @node:	  list-entry of the events list of the struct acpi_gpio_chip
// @handle:	  handle of ACPI method to execute when the IRQ triggers
// @handler:	  handler function to pass to request_irq() when requesting the IRQ
// @pin:	  GPIO pin number on the struct gpio_chip
// @irq:	  Linux IRQ number for the event, for request_irq() / free_irq()
// @irqflags:	  flags to pass to request_irq() when requesting the IRQ
// @irq_is_wake:  If the ACPI flags indicate the IRQ is a wakeup source
// @irq_requested:True if request_irq() has been done
// @desc:	  struct gpio_desc for the GPIO pin for this event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpio_event {
    pub node: list_head,
    pub handle: acpi_handle,
    pub handler: irq_handler_t,
    pub pin: c_uint,
    pub irq: c_uint,
    pub irqflags: c_ulong,
    pub irq_is_wake: bool,
    pub irq_requested: bool,
    pub desc: *mut gpio_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpio_connection {
    pub node: list_head,
    pub pin: c_uint,
    pub desc: *mut gpio_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpio_chip {
//
// ACPICA requires that the first field of the context parameter
// passed to acpi_install_address_space_handler() is large enough
// to hold struct acpi_connection_info.
//
    pub conn_info: acpi_connection_info,
    pub conns: list_head,
    pub conn_lock: mutex,
    pub chip: *mut gpio_chip,
    pub events: list_head,
    pub deferred_req_irqs_list_entry: list_head,
}

//
// struct acpi_gpio_info - ACPI GPIO specific information
// @adev: reference to ACPI device which consumes GPIO resource
// @flags: GPIO initialization flags
// @gpioint: if %true this GPIO is of type GpioInt otherwise type is GpioIo
// @wake_capable: wake capability as provided by ACPI
// @pin_config: pin bias as provided by ACPI
// @polarity: interrupt polarity as provided by ACPI
// @triggering: triggering type as provided by ACPI
// @debounce: debounce timeout as provided by ACPI
// @quirks: Linux specific quirks as provided by struct acpi_gpio_mapping
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpio_info {
    pub adev: *mut acpi_device,
    pub flags: enum gpiod_flags,
    pub gpioint: bool,
    pub wake_capable: bool,
    pub pin_config: c_int,
    pub polarity: c_int,
    pub triggering: c_int,
    pub debounce: c_uint,
    pub quirks: c_uint,
}

#[no_mangle]
unsafe extern "C" fn acpi_gpiochip_find(gc: *mut gpio_chip, data: *const c_void) -> c_int {
    static int acpi_gpiochip_find(struct gpio_chip *gc, const void *data)
    {
// First check the actual GPIO device
    if (device_match_acpi_handle(&gc.gpiodev.dev, data))
    return true;
//
// When the ACPI device is artificially split to the banks of GPIOs,
// where each of them is represented by a separate GPIO device,
// the firmware node of the physical device may not be shared among
// the banks as they may require different values for the same property,
// e.g., number of GPIOs in a certain bank. In such case the ACPI handle
// of a GPIO device is NULL and can not be used. Hence we have to check
// the parent device to be sure that there is no match before bailing
// out.
//
    if (gc.parent)
    return device_match_acpi_handle(gc.parent, data);
    return false;
    }
//
// acpi_get_gpiod() - Translate ACPI GPIO pin to GPIO descriptor usable with GPIO API
// @path:	ACPI GPIO controller full path name, (e.g. "\\_SB.GPO1")
// @pin:	ACPI GPIO pin number (0-based, controller-relative)
//
// Returns:
// GPIO descriptor to use with Linux generic GPIO API.
// If the GPIO cannot be translated or there is an error an ERR_PTR is
// returned.
//
// Specifically returns %-EPROBE_DEFER if the referenced GPIO
// controller does not have GPIO chip registered at the moment. This is to
// support probe deferral.
//
    static struct gpio_desc *acpi_get_gpiod(char *path, unsigned int pin)
    {
    acpi_handle handle;
    acpi_status status;
    status = acpi_get_handle(core::ptr::null_mut(), path, &handle);
    if (ACPI_FAILURE(status))
    return ERR_PTR(-ENODEV);
    struct gpio_device *gdev __free(gpio_device_put) =
    gpio_device_find(handle, acpi_gpiochip_find);
    if (!gdev)
    return ERR_PTR(-EPROBE_DEFER);
    return gpio_device_get_desc(gdev, pin);
    }
#[no_mangle]
unsafe extern "C" fn acpi_gpio_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t acpi_gpio_irq_handler(int irq, void *data)
    {
    struct acpi_gpio_event *event = data;
    acpi_evaluate_object(event.handle, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn acpi_gpio_irq_handler_evt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t acpi_gpio_irq_handler_evt(int irq, void *data)
    {
    struct acpi_gpio_event *event = data;
    acpi_execute_simple_method(event.handle, core::ptr::null_mut(), event.pin);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn acpi_gpio_chip_dh(handle: acpi_handle, data: *mut c_void) {
    static void acpi_gpio_chip_dh(acpi_handle handle, void *data)
    {
// The address of this function is used as a key.
    }
    bool acpi_gpio_get_irq_resource(struct acpi_resource *ares,
    struct acpi_resource_gpio **agpio)
    {
    struct acpi_resource_gpio *gpio;
    if (ares.type != ACPI_RESOURCE_TYPE_GPIO)
    return false;
    gpio = &ares.data.gpio;
    if (gpio.connection_type != ACPI_RESOURCE_GPIO_TYPE_INT)
    return false;
// agpio = gpio;
    return true;
    }
    EXPORT_SYMBOL_GPL(acpi_gpio_get_irq_resource);
//
// acpi_gpio_get_io_resource - Fetch details of an ACPI resource if it is a GPIO
// I/O resource or return False if not.
// @ares:	Pointer to the ACPI resource to fetch
// @agpio:	Pointer to a &struct acpi_resource_gpio to store the output pointer
//
// Returns:
// %true if GpioIo resource is found, %false otherwise.
//
    bool acpi_gpio_get_io_resource(struct acpi_resource *ares,
    struct acpi_resource_gpio **agpio)
    {
    struct acpi_resource_gpio *gpio;
    if (ares.type != ACPI_RESOURCE_TYPE_GPIO)
    return false;
    gpio = &ares.data.gpio;
    if (gpio.connection_type != ACPI_RESOURCE_GPIO_TYPE_IO)
    return false;
// agpio = gpio;
    return true;
    }
    EXPORT_SYMBOL_GPL(acpi_gpio_get_io_resource);
    static void acpi_gpiochip_request_irq(struct acpi_gpio_chip *acpi_gpio,
    struct acpi_gpio_event *event)
    {
    struct device *parent = acpi_gpio.chip.parent;
    int ret, value;
    ret = request_threaded_irq(event.irq, core::ptr::null_mut(), event.handler,
    event.irqflags | IRQF_ONESHOT, "ACPI:Event", event);
    if (ret) {
    dev_err(parent, "Failed to setup interrupt handler for %d\n", event.irq);
    return;
    }
    if (event.irq_is_wake)
    enable_irq_wake(event.irq);
    event.irq_requested = true;
//
// Make sure we trigger the initial state of ActiveBoth IRQs.
//
// According to the Microsoft GPIO documentation, triggering GPIO
// interrupts marked as ActiveBoth during initialization is correct
// as long as the associated GPIO line is already "asserted"
// (logic level low). We should not trigger edge-based GPIO
// interrupts not marked as ActiveBoth.
//
// See: https://learn.microsoft.com/en-us/windows-hardware/drivers/bringup/general-purpose-i-o--gpio-
// Section: "GPIO controllers and ActiveBoth interrupts"
//
    if (acpi_gpio_need_run_edge_events_on_boot() &&
    ((event.irqflags & (IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING)) ==
    (IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING))) {
    value = gpiod_get_raw_value_cansleep(event.desc);
    if (value == 0)
    event.handler(event.irq, event);
    }
    }
#[no_mangle]
unsafe extern "C" fn acpi_gpiochip_request_irqs(acpi_gpio: *mut acpi_gpio_chip) {
    static void acpi_gpiochip_request_irqs(struct acpi_gpio_chip *acpi_gpio)
    {
    struct acpi_gpio_event *event;
    list_for_each_entry(event, &acpi_gpio.events, node)
    acpi_gpiochip_request_irq(acpi_gpio, event);
    }
    static enum gpiod_flags
    acpi_gpio_to_gpiod_flags(const struct acpi_resource_gpio *agpio, int polarity)
    {
// GpioInt() implies input configuration
    if (agpio.connection_type == ACPI_RESOURCE_GPIO_TYPE_INT)
    return GPIOD_IN;
    switch (agpio.io_restriction) {
    case ACPI_IO_RESTRICT_INPUT:
    return GPIOD_IN;
    case ACPI_IO_RESTRICT_OUTPUT:
//
// ACPI GPIO resources don't contain an initial value for the
// GPIO. Therefore we deduce that value from the pull field
// and the polarity instead. If the pin is pulled up we assume
// default to be high, if it is pulled down we assume default
// to be low, otherwise we leave pin untouched. For active low
// polarity values will be switched. See also
// Documentation/firmware-guide/acpi/gpio-properties.rst.
//
    switch (agpio.pin_config) {
    case ACPI_PIN_CONFIG_PULLUP:
    let mut polarity: return = = GPIO_ACTIVE_LOW ? GPIOD_OUT_LOW : GPIOD_OUT_HIGH;
    case ACPI_PIN_CONFIG_PULLDOWN:
    let mut polarity: return = = GPIO_ACTIVE_LOW ? GPIOD_OUT_HIGH : GPIOD_OUT_LOW;
    default:
    break;
    }
    break;
    default:
    break;
    }
//
// Assume that the BIOS has configured the direction and pull
// accordingly.
//
    return GPIOD_ASIS;
    }
    static void acpi_gpio_set_debounce_timeout(struct gpio_desc *desc,
    unsigned int acpi_debounce)
    {
    int ret;
// ACPI uses hundredths of milliseconds units
    acpi_debounce *= 10;
    ret = gpio_set_debounce_timeout(desc, acpi_debounce);
    if (ret)
    gpiod_warn(desc, "Failed to set debounce-timeout %u: %d\n",
    acpi_debounce, ret);
    }
    static struct gpio_desc *acpi_request_own_gpiod(struct gpio_chip *chip,
    struct acpi_resource_gpio *agpio,
    unsigned int index,
    const char *label)
    {
    enum gpiod_flags flags;
    struct gpio_desc *desc;
    unsigned int pin;
    int polarity;
    if (index >= agpio.pin_table_length)
    return ERR_PTR(-EINVAL);
    pin = agpio.pin_table[index];
    polarity = GPIO_ACTIVE_HIGH;
    flags = acpi_gpio_to_gpiod_flags(agpio, polarity);
    desc = gpiochip_request_own_desc(chip, pin, label, polarity, flags);
    if (IS_ERR(desc))
    return desc;
    acpi_gpio_set_debounce_timeout(desc, agpio.debounce_timeout);
    return desc;
    }
    static bool acpi_gpio_irq_is_wake(struct device *parent,
    const struct acpi_resource_gpio *agpio)
    {
    unsigned int pin;
    if (agpio.pin_table_length == 0)
    return false;
    pin = agpio.pin_table[0];
    if (agpio.wake_capable != ACPI_WAKE_CAPABLE)
    return false;
    if (acpi_gpio_in_ignore_list(ACPI_GPIO_IGNORE_WAKE, dev_name(parent), pin)) {
    dev_info(parent, "Ignoring wakeup on pin %u\n", pin);
    return false;
    }
    return true;
    }
// Always returns AE_OK so that we keep looping over the resources
    static acpi_status acpi_gpiochip_alloc_event(struct acpi_resource *ares,
    void *context)
    {
    struct acpi_gpio_chip *acpi_gpio = context;
    struct gpio_chip *chip = acpi_gpio.chip;
    struct acpi_resource_gpio *agpio;
    acpi_handle handle, evt_handle;
    struct acpi_gpio_event *event;
    let mut handler: irq_handler_t = core::ptr::null_mut();
    struct gpio_desc *desc;
    unsigned int pin;
    int ret, irq;
    if (!acpi_gpio_get_irq_resource(ares, &agpio))
    return AE_OK;
    if (agpio.pin_table_length == 0)
    return AE_OK;
    handle = ACPI_HANDLE(chip.parent);
    pin = agpio.pin_table[0];
    if (pin <= 255) {
    char ev_name[8];
    sprintf(ev_name, "_%c%02X",
    agpio.triggering == ACPI_EDGE_SENSITIVE ? 'E' : 'L',
    pin);
    if (ACPI_SUCCESS(acpi_get_handle(handle, ev_name, &evt_handle)))
    handler = acpi_gpio_irq_handler;
    }
    if (!handler) {
    if (ACPI_SUCCESS(acpi_get_handle(handle, "_EVT", &evt_handle)))
    handler = acpi_gpio_irq_handler_evt;
    }
    if (!handler)
    return AE_OK;
    if (acpi_gpio_in_ignore_list(ACPI_GPIO_IGNORE_INTERRUPT, dev_name(chip.parent), pin)) {
    dev_info(chip.parent, "Ignoring interrupt on pin %u\n", pin);
    return AE_OK;
    }
    desc = acpi_request_own_gpiod(chip, agpio, 0, "ACPI:Event");
    if (IS_ERR(desc)) {
    dev_err(chip.parent,
    "Failed to request GPIO for pin 0x%04X, err %pe\n",
    pin, desc);
    return AE_OK;
    }
    ret = gpiochip_lock_as_irq(chip, pin);
    if (ret) {
    dev_err(chip.parent,
    "Failed to lock GPIO pin 0x%04X as interrupt, err %d\n",
    pin, ret);
    goto fail_free_desc;
    }
    irq = gpiod_to_irq(desc);
    if (irq < 0) {
    dev_err(chip.parent,
    "Failed to translate GPIO pin 0x%04X to IRQ, err %d\n",
    pin, irq);
    goto fail_unlock_irq;
    }
    event = kzalloc_obj(*event);
    if (!event)
    goto fail_unlock_irq;
    event.irqflags = IRQF_ONESHOT;
    if (agpio.triggering == ACPI_LEVEL_SENSITIVE) {
    if (agpio.polarity == ACPI_ACTIVE_HIGH)
    event.irqflags |= IRQF_TRIGGER_HIGH;
    else
    event.irqflags |= IRQF_TRIGGER_LOW;
    } else {
    switch (agpio.polarity) {
    case ACPI_ACTIVE_HIGH:
    event.irqflags |= IRQF_TRIGGER_RISING;
    break;
    case ACPI_ACTIVE_LOW:
    event.irqflags |= IRQF_TRIGGER_FALLING;
    break;
    default:
    event.irqflags |= IRQF_TRIGGER_RISING |
    IRQF_TRIGGER_FALLING;
    break;
    }
    }
    event.handle = evt_handle;
    event.handler = handler;
    event.irq = irq;
    event.irq_is_wake = acpi_gpio_irq_is_wake(chip.parent, agpio);
    event.pin = pin;
    event.desc = desc;
    list_add_tail(&event.node, &acpi_gpio.events);
    return AE_OK;
    fail_unlock_irq:
    gpiochip_unlock_as_irq(chip, pin);
    fail_free_desc:
    gpiochip_free_own_desc(desc);
    return AE_OK;
    }
//
// acpi_gpiochip_request_interrupts() - Register isr for gpio chip ACPI events
// @chip:      GPIO chip
//
// ACPI5 platforms can use GPIO signaled ACPI events. These GPIO interrupts are
// handled by ACPI event methods which need to be called from the GPIO
// chip's interrupt handler. acpi_gpiochip_request_interrupts() finds out which
// GPIO pins have ACPI event methods and assigns interrupt handlers that calls
// the ACPI event methods for those pins.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_gpiochip_request_interrupts(chip: *mut gpio_chip) {
    void acpi_gpiochip_request_interrupts(struct gpio_chip *chip)
    {
    struct acpi_gpio_chip *acpi_gpio;
    acpi_handle handle;
    acpi_status status;
    if (!chip.parent || !chip.to_irq)
    return;
    handle = ACPI_HANDLE(chip.parent);
    if (!handle)
    return;
    status = acpi_get_data(handle, acpi_gpio_chip_dh, (void **)&acpi_gpio);
    if (ACPI_FAILURE(status))
    return;
    if (acpi_quirk_skip_gpio_event_handlers())
    return;
    acpi_walk_resources(handle, METHOD_NAME__AEI,
    acpi_gpiochip_alloc_event, acpi_gpio);
    if (acpi_gpio_add_to_deferred_list(&acpi_gpio.deferred_req_irqs_list_entry))
    return;
    acpi_gpiochip_request_irqs(acpi_gpio);
    }
    EXPORT_SYMBOL_GPL(acpi_gpiochip_request_interrupts);
//
// acpi_gpiochip_free_interrupts() - Free GPIO ACPI event interrupts.
// @chip:      GPIO chip
//
// Free interrupts associated with GPIO ACPI event method for the given
// GPIO chip.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_gpiochip_free_interrupts(chip: *mut gpio_chip) {
    void acpi_gpiochip_free_interrupts(struct gpio_chip *chip)
    {
    struct acpi_gpio_chip *acpi_gpio;
    struct acpi_gpio_event *event, *ep;
    acpi_handle handle;
    acpi_status status;
    if (!chip.parent || !chip.to_irq)
    return;
    handle = ACPI_HANDLE(chip.parent);
    if (!handle)
    return;
    status = acpi_get_data(handle, acpi_gpio_chip_dh, (void **)&acpi_gpio);
    if (ACPI_FAILURE(status))
    return;
    acpi_gpio_remove_from_deferred_list(&acpi_gpio.deferred_req_irqs_list_entry);
    list_for_each_entry_safe_reverse(event, ep, &acpi_gpio.events, node) {
    if (event.irq_requested) {
    if (event.irq_is_wake)
    disable_irq_wake(event.irq);
    free_irq(event.irq, event);
    }
    gpiochip_unlock_as_irq(chip, event.pin);
    gpiochip_free_own_desc(event.desc);
    list_del(&event.node);
    kfree(event);
    }
    }
    EXPORT_SYMBOL_GPL(acpi_gpiochip_free_interrupts);
#[no_mangle]
pub unsafe extern "C" fn acpi_gpio_process_deferred_list(list: *mut list_head) -> void __init {
    void __init acpi_gpio_process_deferred_list(struct list_head *list)
    {
    struct acpi_gpio_chip *acpi_gpio, *tmp;
    list_for_each_entry_safe(acpi_gpio, tmp, list, deferred_req_irqs_list_entry)
    acpi_gpiochip_request_irqs(acpi_gpio);
    }
    int acpi_dev_add_driver_gpios(struct acpi_device *adev,
    const struct acpi_gpio_mapping *gpios)
    {
    if (adev && gpios) {
    adev.driver_gpios = gpios;
    return 0;
    }
    return -EINVAL;
    }
    EXPORT_SYMBOL_GPL(acpi_dev_add_driver_gpios);
#[no_mangle]
pub unsafe extern "C" fn acpi_dev_remove_driver_gpios(adev: *mut acpi_device) {
    void acpi_dev_remove_driver_gpios(struct acpi_device *adev)
    {
    if (adev)
    adev.driver_gpios = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(acpi_dev_remove_driver_gpios);
#[no_mangle]
unsafe extern "C" fn acpi_dev_release_driver_gpios(adev: *mut c_void) {
    static void acpi_dev_release_driver_gpios(void *adev)
    {
    acpi_dev_remove_driver_gpios(adev);
    }
    int devm_acpi_dev_add_driver_gpios(struct device *dev,
    const struct acpi_gpio_mapping *gpios)
    {
    struct acpi_device *adev = ACPI_COMPANION(dev);
    int ret;
    ret = acpi_dev_add_driver_gpios(adev, gpios);
    if (ret)
    return ret;
    return devm_add_action_or_reset(dev, acpi_dev_release_driver_gpios, adev);
    }
    EXPORT_SYMBOL_GPL(devm_acpi_dev_add_driver_gpios);
    static bool acpi_get_driver_gpio_data(struct acpi_device *adev,
    const char *name, int index,
    struct fwnode_reference_args *args,
    unsigned int *quirks)
    {
    const struct acpi_gpio_mapping *gm;
    if (!adev || !adev.driver_gpios)
    return false;
    for (gm = adev.driver_gpios; gm.name; gm++)
    if (!strcmp(name, gm.name) && gm.data && index < gm.size) {
    const struct acpi_gpio_params *params = gm.data + index;
    args.fwnode = acpi_fwnode_handle(adev);
    args.args[0] = params.crs_entry_index;
    args.args[1] = params.line_index;
    args.args[2] = params.active_low;
    args.nargs = 3;
// quirks = gm->quirks;
    return true;
    }
    return false;
    }
    static int
    __acpi_gpio_update_gpiod_flags(enum gpiod_flags *flags, enum gpiod_flags update)
    {
    const enum gpiod_flags mask =
    GPIOD_FLAGS_BIT_DIR_SET | GPIOD_FLAGS_BIT_DIR_OUT |
    GPIOD_FLAGS_BIT_DIR_VAL;
    let mut ret: c_int = 0;
//
// Check if the BIOS has IoRestriction with explicitly set direction
// and update @flags accordingly. Otherwise use whatever caller asked
// for.
//
    if (update & GPIOD_FLAGS_BIT_DIR_SET) {
    let mut diff: enum gpiod_flags = *flags ^ update;
//
// Check if caller supplied incompatible GPIO initialization
// flags.
//
// Return %-EINVAL to notify that firmware has different
// settings and we are going to use them.
//
    if (((*flags & GPIOD_FLAGS_BIT_DIR_SET) && (diff & GPIOD_FLAGS_BIT_DIR_OUT)) ||
    ((*flags & GPIOD_FLAGS_BIT_DIR_OUT) && (diff & GPIOD_FLAGS_BIT_DIR_VAL)))
    ret = -EINVAL;
// flags = (*flags & ~mask) | (update & mask);
    }
    return ret;
    }
    static int acpi_gpio_update_gpiod_flags(enum gpiod_flags *flags,
    struct acpi_gpio_info *info)
    {
    struct device *dev = &info.adev.dev;
    let mut old: enum gpiod_flags = *flags;
    int ret;
    ret = __acpi_gpio_update_gpiod_flags(&old, info.flags);
    if (info.quirks & ACPI_GPIO_QUIRK_NO_IO_RESTRICTION) {
    if (ret)
    dev_warn(dev, FW_BUG "GPIO not in correct mode, fixing\n");
    } else {
    if (ret)
    dev_dbg(dev, "Override GPIO initialization flags\n");
// flags = old;
    }
    return ret;
    }
    static int acpi_gpio_update_gpiod_lookup_flags(unsigned long *lookupflags,
    struct acpi_gpio_info *info)
    {
    switch (info.pin_config) {
    case ACPI_PIN_CONFIG_PULLUP:
// lookupflags |= GPIO_PULL_UP;
    break;
    case ACPI_PIN_CONFIG_PULLDOWN:
// lookupflags |= GPIO_PULL_DOWN;
    break;
    case ACPI_PIN_CONFIG_NOPULL:
// lookupflags |= GPIO_PULL_DISABLE;
    break;
    default:
    break;
    }
    if (info.polarity == GPIO_ACTIVE_LOW)
// lookupflags |= GPIO_ACTIVE_LOW;
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpio_lookup {
    pub params: acpi_gpio_params,
    pub info: *mut acpi_gpio_info,
    pub desc: *mut gpio_desc,
    pub n: c_int,
}

#[no_mangle]
unsafe extern "C" fn acpi_populate_gpio_lookup(ares: *mut acpi_resource, data: *mut c_void) -> c_int {
    static int acpi_populate_gpio_lookup(struct acpi_resource *ares, void *data)
    {
    struct acpi_gpio_lookup *lookup = data;
    struct acpi_gpio_params *params = &lookup.params;
    struct acpi_gpio_info *info = lookup.info;
    if (ares.type != ACPI_RESOURCE_TYPE_GPIO)
    return 1;
    if (!lookup.desc) {
    const struct acpi_resource_gpio *agpio = &ares.data.gpio;
    let mut gpioint: bool = agpio.connection_type == ACPI_RESOURCE_GPIO_TYPE_INT;
    struct gpio_desc *desc;
    u16 pin_index;
    if (info.quirks & ACPI_GPIO_QUIRK_ONLY_GPIOIO && gpioint)
    params.crs_entry_index++;
    if (lookup.n++ != params.crs_entry_index)
    return 1;
    pin_index = params.line_index;
    if (pin_index >= agpio.pin_table_length)
    return 1;
    if (info.quirks & ACPI_GPIO_QUIRK_ABSOLUTE_NUMBER)
    desc = gpio_to_desc(agpio.pin_table[pin_index]);
    else
    desc = acpi_get_gpiod(agpio.resource_source.string_ptr,
    agpio.pin_table[pin_index]);
    lookup.desc = desc;
    info.pin_config = agpio.pin_config;
    info.debounce = agpio.debounce_timeout;
    info.gpioint = gpioint;
    info.wake_capable = acpi_gpio_irq_is_wake(&info.adev.dev, agpio);
//
// Polarity and triggering are only specified for GpioInt
// resource.
// Note: we expect here:
// - ACPI_ACTIVE_LOW == GPIO_ACTIVE_LOW
// - ACPI_ACTIVE_HIGH == GPIO_ACTIVE_HIGH
//
    if (info.gpioint) {
    info.polarity = agpio.polarity;
    info.triggering = agpio.triggering;
    } else {
    info.polarity = params.active_low;
    }
    info.flags = acpi_gpio_to_gpiod_flags(agpio, info.polarity);
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn acpi_gpio_resource_lookup(lookup: *mut acpi_gpio_lookup) -> c_int {
    static int acpi_gpio_resource_lookup(struct acpi_gpio_lookup *lookup)
    {
    struct acpi_gpio_info *info = lookup.info;
    struct acpi_device *adev = info.adev;
    struct list_head res_list;
    int ret;
    INIT_LIST_HEAD(&res_list);
    ret = acpi_dev_get_resources(adev, &res_list,
    acpi_populate_gpio_lookup,
    lookup);
    if (ret < 0)
    return ret;
    acpi_dev_free_resource_list(&res_list);
    if (!lookup.desc)
    return -ENOENT;
    return 0;
    }
    static int acpi_gpio_property_lookup(struct fwnode_handle *fwnode, const char *propname,
    struct acpi_gpio_lookup *lookup)
    {
    struct fwnode_reference_args args;
    struct acpi_gpio_params *params = &lookup.params;
    struct acpi_gpio_info *info = lookup.info;
    let mut index: c_uint = params.crs_entry_index;
    let mut quirks: c_uint = 0;
    int ret;
    memset(&args, 0, sizeof(args));
    ret = __acpi_node_get_property_reference(fwnode, propname, index, 3, &args);
    if (ret) {
    struct acpi_device *adev;
    adev = to_acpi_device_node(fwnode);
    if (!acpi_get_driver_gpio_data(adev, propname, index, &args, &quirks))
    return ret;
    }
//
// The property was found and resolved, so need to lookup the GPIO based
// on returned args.
//
    if (!to_acpi_device_node(args.fwnode))
    return -EINVAL;
    if (args.nargs != 3)
    return -EPROTO;
    params.crs_entry_index = args.args[0];
    params.line_index = args.args[1];
    params.active_low = !!args.args[2];
    info.adev = to_acpi_device_node(args.fwnode);
    info.quirks = quirks;
    return 0;
    }
//
// acpi_get_gpiod_by_index() - get a GPIO descriptor from device resources
// @adev: pointer to a ACPI device to get GPIO from
// @propname: Property name of the GPIO (optional)
// @lookup: pointer to struct acpi_gpio_lookup to fill in
//
// Function goes through ACPI resources for @adev and based on @lookup.index looks
// up a GpioIo/GpioInt resource, translates it to the Linux GPIO descriptor,
// and returns it. @lookup.index matches GpioIo/GpioInt resources only so if there
// are total 3 GPIO resources, the index goes from 0 to 2.
//
// If @propname is specified the GPIO is looked using device property. In
// that case @index is used to select the GPIO entry in the property value
// (in case of multiple).
//
// Returns:
// 0 on success, negative errno on failure.
//
// The @lookup is filled with GPIO descriptor to use with Linux generic GPIO API.
// If the GPIO cannot be translated an error will be returned.
//
// Note: if the GPIO resource has multiple entries in the pin list, this
// function only returns the first.
//
    static int acpi_get_gpiod_by_index(struct acpi_device *adev, const char *propname,
    struct acpi_gpio_lookup *lookup)
    {
    struct acpi_gpio_params *params = &lookup.params;
    struct acpi_gpio_info *info = lookup.info;
    int ret;
    if (propname) {
    dev_dbg(&adev.dev, "GPIO: looking up %s\n", propname);
    ret = acpi_gpio_property_lookup(acpi_fwnode_handle(adev), propname, lookup);
    if (ret)
    return ret;
    dev_dbg(&adev.dev, "GPIO: _DSD returned %s %u %u %u\n",
    dev_name(&info.adev.dev),
    params.crs_entry_index, params.line_index, params.active_low);
    } else {
    dev_dbg(&adev.dev, "GPIO: looking up %u in _CRS\n", params.crs_entry_index);
    info.adev = adev;
    }
    return acpi_gpio_resource_lookup(lookup);
    }
//
// acpi_get_gpiod_from_data() - get a GPIO descriptor from ACPI data node
// @fwnode: pointer to an ACPI firmware node to get the GPIO information from
// @propname: Property name of the GPIO
// @lookup: pointer to struct acpi_gpio_lookup to fill in
//
// This function uses the property-based GPIO lookup to get to the GPIO
// resource with the relevant information from a data-only ACPI firmware node
// and uses that to obtain the GPIO descriptor to return.
//
// Returns:
// 0 on success, negative errno on failure.
//
// The @lookup is filled with GPIO descriptor to use with Linux generic GPIO API.
// If the GPIO cannot be translated an error will be returned.
//
    static int acpi_get_gpiod_from_data(struct fwnode_handle *fwnode, const char *propname,
    struct acpi_gpio_lookup *lookup)
    {
    int ret;
    if (!is_acpi_data_node(fwnode))
    return -ENODEV;
    if (!propname)
    return -EINVAL;
    ret = acpi_gpio_property_lookup(fwnode, propname, lookup);
    if (ret)
    return ret;
    return acpi_gpio_resource_lookup(lookup);
    }
    static bool acpi_can_fallback_to_crs(struct acpi_device *adev,
    const char *con_id)
    {
// If there is no ACPI device, there is no _CRS to fall back to
    if (!adev)
    return false;
// Never allow fallback if the device has properties
    if (acpi_dev_has_props(adev) || adev.driver_gpios)
    return false;
    let mut con_id: return = = core::ptr::null_mut();
    }
    static struct gpio_desc *
    __acpi_find_gpio(struct fwnode_handle *fwnode, const char *con_id, unsigned int idx,
    bool can_fallback, struct acpi_gpio_info *info)
    {
    struct acpi_device *adev = to_acpi_device_node(fwnode);
    struct acpi_gpio_lookup lookup;
    struct gpio_desc *desc;
    char propname[32];
    int ret;
    memset(&lookup, 0, sizeof(lookup));
    lookup.params.crs_entry_index = idx;
    lookup.info = info;
// Try first from _DSD
    for_each_gpio_property_name(propname, con_id) {
    if (adev)
    ret = acpi_get_gpiod_by_index(adev, propname, &lookup);
    else
    ret = acpi_get_gpiod_from_data(fwnode, propname, &lookup);
    if (ret)
    continue;
    desc = lookup.desc;
    if (PTR_ERR(desc) == -EPROBE_DEFER)
    return desc;
    if (!IS_ERR(desc))
    return desc;
    }
// Then from plain _CRS GPIOs
    if (can_fallback) {
    ret = acpi_get_gpiod_by_index(adev, core::ptr::null_mut(), &lookup);
    if (ret)
    return ERR_PTR(ret);
    return lookup.desc;
    }
    return ERR_PTR(-ENOENT);
    }
    struct gpio_desc *acpi_find_gpio(struct fwnode_handle *fwnode,
    const char *con_id,
    unsigned int idx,
    enum gpiod_flags *dflags,
    unsigned long *lookupflags)
    {
    struct acpi_device *adev = to_acpi_device_node(fwnode);
    let mut can_fallback: bool = acpi_can_fallback_to_crs(adev, con_id);
    let mut info: acpi_gpio_info = {};
    struct gpio_desc *desc;
    desc = __acpi_find_gpio(fwnode, con_id, idx, can_fallback, &info);
    if (IS_ERR(desc))
    return desc;
    if (info.gpioint &&
    (*dflags == GPIOD_OUT_LOW || *dflags == GPIOD_OUT_HIGH)) {
    dev_dbg(&adev.dev, "refusing GpioInt() entry when doing GPIOD_OUT_* lookup\n");
    return ERR_PTR(-ENOENT);
    }
    acpi_gpio_update_gpiod_flags(dflags, &info);
    acpi_gpio_update_gpiod_lookup_flags(lookupflags, &info);
    acpi_gpio_set_debounce_timeout(desc, info.debounce);
    return desc;
    }
//
// acpi_dev_gpio_irq_wake_get_by() - Find GpioInt and translate it to Linux IRQ number
// @adev: pointer to a ACPI device to get IRQ from
// @con_id: optional name of GpioInt resource
// @index: index of GpioInt resource (starting from %0)
// @wake_capable: Set to true if the IRQ is wake capable
//
// If the device has one or more GpioInt resources, this function can be
// used to translate from the GPIO offset in the resource to the Linux IRQ
// number.
//
// The function is idempotent, though each time it runs it will configure GPIO
// pin direction according to the flags in GpioInt resource.
//
// The function takes optional @con_id parameter. If the resource has
// a @con_id in a property, then only those will be taken into account.
//
// The GPIO is considered wake capable if the GpioInt resource specifies
// SharedAndWake or ExclusiveAndWake.
//
// Returns:
// Linux IRQ number (> 0) on success, negative errno on failure.
//
    int acpi_dev_gpio_irq_wake_get_by(struct acpi_device *adev, const char *con_id, int index,
    bool *wake_capable)
    {
    struct fwnode_handle *fwnode = acpi_fwnode_handle(adev);
    int idx, i;
    unsigned int irq_flags;
    int ret;
    for (i = 0, idx = 0; idx <= index; i++) {
    let mut info: acpi_gpio_info = {};
    struct gpio_desc *desc;
// Ignore -EPROBE_DEFER, it only matters if idx matches
    desc = __acpi_find_gpio(fwnode, con_id, i, true, &info);
    if (IS_ERR(desc) && PTR_ERR(desc) != -EPROBE_DEFER)
    return PTR_ERR(desc);
    if (info.gpioint && idx++ == index) {
    let mut lflags: c_ulong = GPIO_LOOKUP_FLAGS_DEFAULT;
    let mut dflags: enum gpiod_flags = GPIOD_ASIS;
    char label[32];
    int irq;
    if (IS_ERR(desc))
    return PTR_ERR(desc);
    irq = gpiod_to_irq(desc);
    if (irq < 0)
    return irq;
    acpi_gpio_update_gpiod_flags(&dflags, &info);
    acpi_gpio_update_gpiod_lookup_flags(&lflags, &info);
    snprintf(label, sizeof(label), "%pfwP GpioInt(%d)", fwnode, index);
    ret = gpiod_set_consumer_name(desc, con_id ?: label);
    if (ret)
    return ret;
    ret = gpiod_configure_flags(desc, label, lflags, dflags);
    if (ret < 0)
    return ret;
// ACPI uses hundredths of milliseconds units
    ret = gpio_set_debounce_timeout(desc, info.debounce * 10);
    if (ret)
    return ret;
    irq_flags = acpi_dev_get_irq_type(info.triggering,
    info.polarity);
//
// If the IRQ is not already in use then set type
// if specified and different than the current one.
//
    if (can_request_irq(irq, irq_flags)) {
    if (irq_flags != IRQ_TYPE_NONE &&
    irq_flags != irq_get_trigger_type(irq))
    irq_set_irq_type(irq, irq_flags);
    } else {
    dev_dbg(&adev.dev, "IRQ %d already in use\n", irq);
    }
// avoid suspend issues with GPIOs when systems are using S3
    if (wake_capable && acpi_gbl_FADT.flags & ACPI_FADT_LOW_POWER_S0)
// wake_capable = info.wake_capable;
    return irq;
    }
    }
    return -ENOENT;
    }
    EXPORT_SYMBOL_GPL(acpi_dev_gpio_irq_wake_get_by);
    static acpi_status
    acpi_gpio_adr_space_handler(u32 function, acpi_physical_address address,
    u32 bits, u64 *value, void *handler_context,
    void *region_context)
    {
    struct acpi_gpio_chip *achip = region_context;
    struct gpio_chip *chip = achip.chip;
    struct acpi_resource_gpio *agpio;
    struct acpi_resource *ares;
    unsigned int length;
    acpi_status status;
    unsigned int i;
    u16 pin_index;
    status = acpi_buffer_to_resource(achip.conn_info.connection,
    achip.conn_info.length, &ares);
    if (ACPI_FAILURE(status))
    return status;
    if (WARN_ON(ares.type != ACPI_RESOURCE_TYPE_GPIO)) {
    ACPI_FREE(ares);
    return AE_BAD_PARAMETER;
    }
    agpio = &ares.data.gpio;
    if (WARN_ON(agpio.io_restriction == ACPI_IO_RESTRICT_INPUT &&
    function == ACPI_WRITE)) {
    ACPI_FREE(ares);
    return AE_BAD_PARAMETER;
    }
// address represents GPIO pin index in connection table
    if (address >= agpio.pin_table_length) {
    ACPI_FREE(ares);
    return AE_BAD_PARAMETER;
    }
    pin_index = address;
    length = min_t(unsigned int, agpio.pin_table_length, pin_index + bits);
    for (i = pin_index; i < length; ++i) {
    let mut pin: c_uint = agpio.pin_table[i];
    struct acpi_gpio_connection *conn;
    struct gpio_desc *desc;
    u16 word, shift;
    bool found;
    mutex_lock(&achip.conn_lock);
    found = false;
    list_for_each_entry(conn, &achip.conns, node) {
    if (conn.pin == pin) {
    found = true;
    desc = conn.desc;
    break;
    }
    }
//
// The same GPIO can be shared between operation region and
// event but only if the access here is ACPI_READ. In that
// case we "borrow" the event GPIO instead.
//
    if (!found && agpio.shareable == ACPI_SHARED &&
    function == ACPI_READ) {
    struct acpi_gpio_event *event;
    list_for_each_entry(event, &achip.events, node) {
    if (event.pin == pin) {
    desc = event.desc;
    found = true;
    break;
    }
    }
    }
    if (!found) {
    desc = acpi_request_own_gpiod(chip, agpio, i, "ACPI:OpRegion");
    if (IS_ERR(desc)) {
    mutex_unlock(&achip.conn_lock);
    status = AE_ERROR;
    goto out;
    }
    conn = kzalloc_obj(*conn);
    if (!conn) {
    gpiochip_free_own_desc(desc);
    mutex_unlock(&achip.conn_lock);
    status = AE_NO_MEMORY;
    goto out;
    }
    conn.pin = pin;
    conn.desc = desc;
    list_add_tail(&conn.node, &achip.conns);
    }
    mutex_unlock(&achip.conn_lock);
//
// For the cases when OperationRegion() consists of more than
// 64 bits calculate the word and bit shift to use that one to
// access the value.
//
    word = i / 64;
    shift = i % 64;
    if (function == ACPI_WRITE) {
    gpiod_set_raw_value_cansleep(desc, value[word] & BIT_ULL(shift));
    } else {
    if (gpiod_get_raw_value_cansleep(desc))
    value[word] |= BIT_ULL(shift);
    else
    value[word] &= ~BIT_ULL(shift);
    }
    }
    out:
    ACPI_FREE(ares);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn acpi_gpiochip_request_regions(achip: *mut acpi_gpio_chip) {
    static void acpi_gpiochip_request_regions(struct acpi_gpio_chip *achip)
    {
    struct gpio_chip *chip = achip.chip;
    let mut handle: acpi_handle = ACPI_HANDLE(chip.parent);
    acpi_status status;
    INIT_LIST_HEAD(&achip.conns);
    mutex_init(&achip.conn_lock);
    status = acpi_install_address_space_handler(handle, ACPI_ADR_SPACE_GPIO,
    acpi_gpio_adr_space_handler,
    core::ptr::null_mut(), achip);
    if (ACPI_FAILURE(status))
    dev_err(chip.parent,
    "Failed to install GPIO OpRegion handler\n");
    }
#[no_mangle]
unsafe extern "C" fn acpi_gpiochip_free_regions(achip: *mut acpi_gpio_chip) {
    static void acpi_gpiochip_free_regions(struct acpi_gpio_chip *achip)
    {
    struct gpio_chip *chip = achip.chip;
    let mut handle: acpi_handle = ACPI_HANDLE(chip.parent);
    struct acpi_gpio_connection *conn, *tmp;
    acpi_status status;
    status = acpi_remove_address_space_handler(handle, ACPI_ADR_SPACE_GPIO,
    acpi_gpio_adr_space_handler);
    if (ACPI_FAILURE(status)) {
    dev_err(chip.parent,
    "Failed to remove GPIO OpRegion handler\n");
    return;
    }
    list_for_each_entry_safe_reverse(conn, tmp, &achip.conns, node) {
    gpiochip_free_own_desc(conn.desc);
    list_del(&conn.node);
    kfree(conn);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_gpiochip_add(chip: *mut gpio_chip) {
    void acpi_gpiochip_add(struct gpio_chip *chip)
    {
    struct acpi_gpio_chip *acpi_gpio;
    struct acpi_device *adev;
    acpi_status status;
    if (!chip || !chip.parent)
    return;
    adev = ACPI_COMPANION(chip.parent);
    if (!adev)
    return;
    acpi_gpio = kzalloc_obj(*acpi_gpio);
    if (!acpi_gpio) {
    dev_err(chip.parent,
    "Failed to allocate memory for ACPI GPIO chip\n");
    return;
    }
    acpi_gpio.chip = chip;
    INIT_LIST_HEAD(&acpi_gpio.events);
    INIT_LIST_HEAD(&acpi_gpio.deferred_req_irqs_list_entry);
    status = acpi_attach_data(adev.handle, acpi_gpio_chip_dh, acpi_gpio);
    if (ACPI_FAILURE(status)) {
    dev_err(chip.parent, "Failed to attach ACPI GPIO chip\n");
    kfree(acpi_gpio);
    return;
    }
    acpi_gpiochip_request_regions(acpi_gpio);
    acpi_dev_clear_dependencies(adev);
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_gpiochip_remove(chip: *mut gpio_chip) {
    void acpi_gpiochip_remove(struct gpio_chip *chip)
    {
    struct acpi_gpio_chip *acpi_gpio;
    acpi_handle handle;
    acpi_status status;
    if (!chip || !chip.parent)
    return;
    handle = ACPI_HANDLE(chip.parent);
    if (!handle)
    return;
    status = acpi_get_data(handle, acpi_gpio_chip_dh, (void **)&acpi_gpio);
    if (ACPI_FAILURE(status)) {
    dev_warn(chip.parent, "Failed to retrieve ACPI GPIO chip\n");
    return;
    }
    acpi_gpiochip_free_regions(acpi_gpio);
    acpi_detach_data(handle, acpi_gpio_chip_dh);
    kfree(acpi_gpio);
    }
#[no_mangle]
unsafe extern "C" fn acpi_gpio_package_count(obj: *const union acpi_object) -> c_int {
    static int acpi_gpio_package_count(const union acpi_object *obj)
    {
    const union acpi_object *element = obj.package.elements;
    const union acpi_object *end = element + obj.package.count;
    let mut count: c_uint = 0;
    while (element < end) {
    switch (element.type) {
    case ACPI_TYPE_LOCAL_REFERENCE:
    case ACPI_TYPE_STRING:
    element += 3;
    fallthrough;
    case ACPI_TYPE_INTEGER:
    element++;
    count++;
    break;
    default:
    return -EPROTO;
    }
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn acpi_find_gpio_count(ares: *mut acpi_resource, data: *mut c_void) -> c_int {
    static int acpi_find_gpio_count(struct acpi_resource *ares, void *data)
    {
    unsigned int *count = data;
    if (ares.type == ACPI_RESOURCE_TYPE_GPIO)
// count += ares->data.gpio.pin_table_length;
    return 1;
    }
//
// acpi_gpio_count - count the GPIOs associated with a firmware node / function
// @fwnode:	firmware node of the GPIO consumer
// @con_id:	function within the GPIO consumer
//
// Returns:
// The number of GPIOs associated with a firmware node / function or %-ENOENT,
// if no GPIO has been assigned to the requested function.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_gpio_count(fwnode: *const fwnode_handle, con_id: *const c_char) -> c_int {
    int acpi_gpio_count(const struct fwnode_handle *fwnode, const char *con_id)
    {
    struct acpi_device *adev = to_acpi_device_node(fwnode);
    const union acpi_object *obj;
    const struct acpi_gpio_mapping *gm;
    let mut count: c_int = -ENOENT;
    int ret;
    char propname[32];
// Try first from _DSD
    for_each_gpio_property_name(propname, con_id) {
    ret = acpi_dev_get_property(adev, propname, ACPI_TYPE_ANY, &obj);
    if (ret == 0) {
    if (obj.type == ACPI_TYPE_LOCAL_REFERENCE)
    count = 1;
#[no_mangle]
pub unsafe extern "C" fn if(ACPI_TYPE_PACKAGE: obj->type ==) -> else {
    else if (obj.type == ACPI_TYPE_PACKAGE)
    count = acpi_gpio_package_count(obj);
    } else if (adev.driver_gpios) {
    for (gm = adev.driver_gpios; gm.name; gm++)
    if (strcmp(propname, gm.name) == 0) {
    count = gm.size;
    break;
    }
    }
    if (count > 0)
    break;
    }
// Then from plain _CRS GPIOs
    if (count < 0) {
    struct list_head resource_list;
    let mut crs_count: c_uint = 0;
    if (!acpi_can_fallback_to_crs(adev, con_id))
    return count;
    INIT_LIST_HEAD(&resource_list);
    acpi_dev_get_resources(adev, &resource_list,
    acpi_find_gpio_count, &crs_count);
    acpi_dev_free_resource_list(&resource_list);
    if (crs_count > 0)
    count = crs_count;
    }
    return count ? count : -ENOENT;
    }

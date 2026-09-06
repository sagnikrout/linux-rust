//! Automatically rewritten from C to Rust
//! Source: drivers/phy/motorola/phy-cpcap-usb.c
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
// Motorola CPCAP PMIC USB PHY driver
// Copyright (C) 2017 Tony Lindgren <tony@atomide.com>
//
// Some parts based on earlier Motorola Linux kernel tree code in
// board-mapphone-usb.c and cpcap-usb-det.c:
// Copyright (C) 2007 - 2011 Motorola, Inc.
//

// CPCAP_REG_USBC1 register bits

// CPCAP_REG_USBC2 register bits

// CPCAP_REG_USBC3 register bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpcap_usb_ints_state {
    pub id_ground: bool,
    pub id_float: bool,
    pub chrg_det: bool,
    pub rvrs_chrg: bool,
    pub vbusov: bool,
    pub chrg_se1b: bool,
    pub se0conn: bool,
    pub rvrs_mode: bool,
    pub chrgcurr1: bool,
    pub vbusvld: bool,
    pub sessvld: bool,
    pub sessend: bool,
    pub se1: bool,
    pub battdetb: bool,
    pub dm: bool,
    pub dp: bool,
}

    enum cpcap_gpio_mode {
    CPCAP_DM_DP,
    CPCAP_MDM_RX_TX,
    CPCAP_UNKNOWN_DISABLED,	/* Seems to disable USB lines */
    CPCAP_OTG_DM_DP,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpcap_phy_ddata {
    pub reg: *mut regmap,
    pub dev: *mut device,
    pub phy: usb_phy,
    pub detect_work: delayed_work,
    pub pins: *mut pinctrl,
    pub pins_ulpi: *mut pinctrl_state,
    pub pins_utmi: *mut pinctrl_state,
    pub pins_uart: *mut pinctrl_state,
    pub gpio: [*mut gpio_desc; 2],
    pub vbus: *mut iio_channel,
    pub id: *mut iio_channel,
    pub vusb: *mut regulator,
    pub active: core::sync::atomic::AtomicI32,
    pub vbus_provider:1: c_uint,
    pub docked:1: c_uint,
}

#[no_mangle]
unsafe extern "C" fn cpcap_usb_vbus_valid(ddata: *mut cpcap_phy_ddata) -> bool {
    static bool cpcap_usb_vbus_valid(struct cpcap_phy_ddata *ddata)
    {
    int error, value = 0;
    error = iio_read_channel_processed(ddata.vbus, &value);
    if (error >= 0)
    return value > 3900;
    dev_err(ddata.dev, "error reading VBUS: %i\n", error);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_usb_phy_set_host(otg: *mut usb_otg, host: *mut usb_bus) -> c_int {
    static int cpcap_usb_phy_set_host(struct usb_otg *otg, struct usb_bus *host)
    {
    otg.host = host;
    if (!host)
    otg.state = OTG_STATE_UNDEFINED;
    return 0;
    }
    static int cpcap_usb_phy_set_peripheral(struct usb_otg *otg,
    struct usb_gadget *gadget)
    {
    otg.gadget = gadget;
    if (!gadget)
    otg.state = OTG_STATE_UNDEFINED;
    return 0;
    }
    static const struct phy_ops ops = {
    .owner		= THIS_MODULE,
    };
    static int cpcap_phy_get_ints_state(struct cpcap_phy_ddata *ddata,
    struct cpcap_usb_ints_state *s)
    {
    int val, error;
    error = regmap_read(ddata.reg, CPCAP_REG_INTS1, &val);
    if (error)
    return error;
    s.id_ground = val & BIT(15);
    s.id_float = val & BIT(14);
    s.vbusov = val & BIT(11);
    error = regmap_read(ddata.reg, CPCAP_REG_INTS2, &val);
    if (error)
    return error;
    s.vbusvld = val & BIT(3);
    s.sessvld = val & BIT(2);
    s.sessend = val & BIT(1);
    s.se1 = val & BIT(0);
    error = regmap_read(ddata.reg, CPCAP_REG_INTS4, &val);
    if (error)
    return error;
    s.dm = val & BIT(1);
    s.dp = val & BIT(0);
    return 0;
    }
    static int cpcap_usb_set_uart_mode(struct cpcap_phy_ddata *ddata);
    static int cpcap_usb_set_usb_mode(struct cpcap_phy_ddata *ddata);
    static void cpcap_usb_try_musb_mailbox(struct cpcap_phy_ddata *ddata,
    enum musb_vbus_id_status status)
    {
    int error;
    error = musb_mailbox(status);
    if (!error)
    return;
    dev_dbg(ddata.dev, "%s: musb_mailbox failed: %i\n",
    __func__, error);
    }
#[no_mangle]
unsafe extern "C" fn cpcap_usb_detect(work: *mut work_struct) {
    static void cpcap_usb_detect(struct work_struct *work)
    {
    struct cpcap_phy_ddata *ddata;
    struct cpcap_usb_ints_state s;
    let mut vbus: bool = false;
    int error;
    ddata = container_of(work, struct cpcap_phy_ddata, detect_work.work);
    error = cpcap_phy_get_ints_state(ddata, &s);
    if (error)
    return;
    vbus = cpcap_usb_vbus_valid(ddata);
// We need to kick the VBUS as USB A-host
    if (s.id_ground && ddata.vbus_provider) {
    dev_dbg(ddata.dev, "still in USB A-host mode, kicking VBUS\n");
    cpcap_usb_try_musb_mailbox(ddata, MUSB_ID_GROUND);
    error = regmap_update_bits(ddata.reg, CPCAP_REG_USBC3,
    CPCAP_BIT_VBUSSTBY_EN |
    CPCAP_BIT_VBUSEN_SPI,
    CPCAP_BIT_VBUSEN_SPI);
    if (error)
    goto out_err;
    return;
    }
    if (vbus && s.id_ground && ddata.docked) {
    dev_dbg(ddata.dev, "still docked as A-host, signal ID down\n");
    cpcap_usb_try_musb_mailbox(ddata, MUSB_ID_GROUND);
    return;
    }
// No VBUS needed with docks
    if (vbus && s.id_ground && !ddata.vbus_provider) {
    dev_dbg(ddata.dev, "connected to a dock\n");
    ddata.docked = true;
    error = cpcap_usb_set_usb_mode(ddata);
    if (error)
    goto out_err;
    cpcap_usb_try_musb_mailbox(ddata, MUSB_ID_GROUND);
//
// Force check state again after musb has reoriented,
// otherwise devices won't enumerate after loading PHY
// driver.
//
    schedule_delayed_work(&ddata.detect_work,
    msecs_to_jiffies(1000));
    return;
    }
    if (s.id_ground && !ddata.docked) {
    dev_dbg(ddata.dev, "id ground, USB host mode\n");
    ddata.vbus_provider = true;
    error = cpcap_usb_set_usb_mode(ddata);
    if (error)
    goto out_err;
    cpcap_usb_try_musb_mailbox(ddata, MUSB_ID_GROUND);
    error = regmap_update_bits(ddata.reg, CPCAP_REG_USBC3,
    CPCAP_BIT_VBUSSTBY_EN |
    CPCAP_BIT_VBUSEN_SPI,
    CPCAP_BIT_VBUSEN_SPI);
    if (error)
    goto out_err;
    return;
    }
    error = regmap_update_bits(ddata.reg, CPCAP_REG_USBC3,
    CPCAP_BIT_VBUSSTBY_EN |
    CPCAP_BIT_VBUSEN_SPI, 0);
    if (error)
    goto out_err;
    vbus = cpcap_usb_vbus_valid(ddata);
// Otherwise assume we're connected to a USB host
    if (vbus) {
    dev_dbg(ddata.dev, "connected to USB host\n");
    error = cpcap_usb_set_usb_mode(ddata);
    if (error)
    goto out_err;
    cpcap_usb_try_musb_mailbox(ddata, MUSB_VBUS_VALID);
    return;
    }
    ddata.vbus_provider = false;
    ddata.docked = false;
    cpcap_usb_try_musb_mailbox(ddata, MUSB_VBUS_OFF);
// Default to debug UART mode
    error = cpcap_usb_set_uart_mode(ddata);
    if (error)
    goto out_err;
    dev_dbg(ddata.dev, "set UART mode\n");
    return;
    out_err:
    dev_err(ddata.dev, "error setting cable state: %i\n", error);
    }
#[no_mangle]
unsafe extern "C" fn cpcap_phy_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t cpcap_phy_irq_thread(int irq, void *data)
    {
    struct cpcap_phy_ddata *ddata = data;
    if (!atomic_read(&ddata.active))
    return IRQ_NONE;
    schedule_delayed_work(&ddata.detect_work, msecs_to_jiffies(1));
    return IRQ_HANDLED;
    }
    static int cpcap_usb_init_irq(struct platform_device *pdev,
    struct cpcap_phy_ddata *ddata,
    const char *name)
    {
    int irq, error;
    irq = platform_get_irq_byname(pdev, name);
    if (irq < 0)
    return -ENODEV;
    error = devm_request_threaded_irq(ddata.dev, irq, core::ptr::null_mut(),
    cpcap_phy_irq_thread,
    IRQF_SHARED |
    IRQF_ONESHOT,
    name, ddata);
    if (error) {
    dev_err(ddata.dev, "could not get irq %s: %i\n",
    name, error);
    return error;
    }
    return 0;
    }
    static const char * const cpcap_phy_irqs[] = {
// REG_INT_0
    "id_ground", "id_float",
// REG_INT1
    "se0conn", "vbusvld", "sessvld", "sessend", "se1",
// REG_INT_3
    "dm", "dp",
    };
    static int cpcap_usb_init_interrupts(struct platform_device *pdev,
    struct cpcap_phy_ddata *ddata)
    {
    int i, error;
    for (i = 0; i < ARRAY_SIZE(cpcap_phy_irqs); i++) {
    error = cpcap_usb_init_irq(pdev, ddata, cpcap_phy_irqs[i]);
    if (error)
    return error;
    }
    return 0;
    }
//
// Optional pins and modes. At least Motorola mapphone devices
// are using two GPIOs and dynamic pinctrl to multiplex PHY pins
// to UART, ULPI or UTMI mode.
//
    static int cpcap_usb_gpio_set_mode(struct cpcap_phy_ddata *ddata,
    enum cpcap_gpio_mode mode)
    {
    if (!ddata.gpio[0] || !ddata.gpio[1])
    return 0;
    gpiod_set_value(ddata.gpio[0], mode & 1);
    gpiod_set_value(ddata.gpio[1], mode >> 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_usb_set_uart_mode(ddata: *mut cpcap_phy_ddata) -> c_int {
    static int cpcap_usb_set_uart_mode(struct cpcap_phy_ddata *ddata)
    {
    int error;
// Disable lines to prevent glitches from waking up mdm6600
    error = cpcap_usb_gpio_set_mode(ddata, CPCAP_UNKNOWN_DISABLED);
    if (error)
    goto out_err;
    if (ddata.pins_uart) {
    error = pinctrl_select_state(ddata.pins, ddata.pins_uart);
    if (error)
    goto out_err;
    }
    error = regmap_update_bits(ddata.reg, CPCAP_REG_USBC1,
    CPCAP_BIT_VBUSPD,
    CPCAP_BIT_VBUSPD);
    if (error)
    goto out_err;
    error = regmap_update_bits(ddata.reg, CPCAP_REG_USBC2,
    0xffff, CPCAP_BIT_UARTMUX0 |
    CPCAP_BIT_EMUMODE0);
    if (error)
    goto out_err;
    error = regmap_update_bits(ddata.reg, CPCAP_REG_USBC3, 0x7fff,
    CPCAP_BIT_IDPU_SPI);
    if (error)
    goto out_err;
// Enable UART mode
    error = cpcap_usb_gpio_set_mode(ddata, CPCAP_DM_DP);
    if (error)
    goto out_err;
    return 0;
    out_err:
    dev_err(ddata.dev, "%s failed with %i\n", __func__, error);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_usb_set_usb_mode(ddata: *mut cpcap_phy_ddata) -> c_int {
    static int cpcap_usb_set_usb_mode(struct cpcap_phy_ddata *ddata)
    {
    int error;
// Disable lines to prevent glitches from waking up mdm6600
    error = cpcap_usb_gpio_set_mode(ddata, CPCAP_UNKNOWN_DISABLED);
    if (error)
    return error;
    if (ddata.pins_utmi) {
    error = pinctrl_select_state(ddata.pins, ddata.pins_utmi);
    if (error) {
    dev_err(ddata.dev, "could not set usb mode: %i\n",
    error);
    return error;
    }
    }
    error = regmap_update_bits(ddata.reg, CPCAP_REG_USBC1,
    CPCAP_BIT_VBUSPD, 0);
    if (error)
    goto out_err;
    error = regmap_update_bits(ddata.reg, CPCAP_REG_USBC3,
    CPCAP_BIT_PU_SPI |
    CPCAP_BIT_DMPD_SPI |
    CPCAP_BIT_DPPD_SPI |
    CPCAP_BIT_SUSPEND_SPI |
    CPCAP_BIT_ULPI_SPI_SEL, 0);
    if (error)
    goto out_err;
    error = regmap_update_bits(ddata.reg, CPCAP_REG_USBC2,
    CPCAP_BIT_USBXCVREN,
    CPCAP_BIT_USBXCVREN);
    if (error)
    goto out_err;
// Enable USB mode
    error = cpcap_usb_gpio_set_mode(ddata, CPCAP_OTG_DM_DP);
    if (error)
    goto out_err;
    return 0;
    out_err:
    dev_err(ddata.dev, "%s failed with %i\n", __func__, error);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_usb_init_optional_pins(ddata: *mut cpcap_phy_ddata) -> c_int {
    static int cpcap_usb_init_optional_pins(struct cpcap_phy_ddata *ddata)
    {
    ddata.pins = devm_pinctrl_get(ddata.dev);
    if (IS_ERR(ddata.pins)) {
    dev_info(ddata.dev, "default pins not configured: %ld\n",
    PTR_ERR(ddata.pins));
    ddata.pins = core::ptr::null_mut();
    return 0;
    }
    ddata.pins_ulpi = pinctrl_lookup_state(ddata.pins, "ulpi");
    if (IS_ERR(ddata.pins_ulpi)) {
    dev_info(ddata.dev, "ulpi pins not configured\n");
    ddata.pins_ulpi = core::ptr::null_mut();
    }
    ddata.pins_utmi = pinctrl_lookup_state(ddata.pins, "utmi");
    if (IS_ERR(ddata.pins_utmi)) {
    dev_info(ddata.dev, "utmi pins not configured\n");
    ddata.pins_utmi = core::ptr::null_mut();
    }
    ddata.pins_uart = pinctrl_lookup_state(ddata.pins, "uart");
    if (IS_ERR(ddata.pins_uart)) {
    dev_info(ddata.dev, "uart pins not configured\n");
    ddata.pins_uart = core::ptr::null_mut();
    }
    if (ddata.pins_uart)
    return pinctrl_select_state(ddata.pins, ddata.pins_uart);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_usb_init_optional_gpios(ddata: *mut cpcap_phy_ddata) {
    static void cpcap_usb_init_optional_gpios(struct cpcap_phy_ddata *ddata)
    {
    int i;
    for (i = 0; i < 2; i++) {
    ddata.gpio[i] = devm_gpiod_get_index(ddata.dev, "mode",
    i, GPIOD_OUT_HIGH);
    if (IS_ERR(ddata.gpio[i])) {
    dev_info(ddata.dev, "no mode change GPIO%i: %li\n",
    i, PTR_ERR(ddata.gpio[i]));
    ddata.gpio[i] = core::ptr::null_mut();
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn cpcap_usb_init_iio(ddata: *mut cpcap_phy_ddata) -> c_int {
    static int cpcap_usb_init_iio(struct cpcap_phy_ddata *ddata)
    {
    enum iio_chan_type type;
    int error;
    ddata.vbus = devm_iio_channel_get(ddata.dev, "vbus");
    if (IS_ERR(ddata.vbus)) {
    error = PTR_ERR(ddata.vbus);
    goto out_err;
    }
    if (!ddata.vbus.indio_dev) {
    error = -ENXIO;
    goto out_err;
    }
    error = iio_get_channel_type(ddata.vbus, &type);
    if (error < 0)
    goto out_err;
    if (type != IIO_VOLTAGE) {
    error = -EINVAL;
    goto out_err;
    }
    return 0;
    out_err:
    dev_err(ddata.dev, "could not initialize VBUS or ID IIO: %i\n",
    error);
    return error;
    }

    static const struct of_device_id cpcap_usb_phy_id_table[] = {
    {
    .compatible = "motorola,cpcap-usb-phy",
    },
    {
    .compatible = "motorola,mapphone-cpcap-usb-phy",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, cpcap_usb_phy_id_table);

#[no_mangle]
unsafe extern "C" fn cpcap_usb_phy_probe(pdev: *mut platform_device) -> c_int {
    static int cpcap_usb_phy_probe(struct platform_device *pdev)
    {
    struct cpcap_phy_ddata *ddata;
    struct phy *generic_phy;
    struct phy_provider *phy_provider;
    struct usb_otg *otg;
    int error;
    ddata = devm_kzalloc(&pdev.dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata)
    return -ENOMEM;
    ddata.reg = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!ddata.reg)
    return -ENODEV;
    otg = devm_kzalloc(&pdev.dev, sizeof(*otg), GFP_KERNEL);
    if (!otg)
    return -ENOMEM;
    ddata.dev = &pdev.dev;
    ddata.phy.dev = ddata.dev;
    ddata.phy.label = "cpcap_usb_phy";
    ddata.phy.otg = otg;
    ddata.phy.type = USB_PHY_TYPE_USB2;
    otg.set_host = cpcap_usb_phy_set_host;
    otg.set_peripheral = cpcap_usb_phy_set_peripheral;
    otg.usb_phy = &ddata.phy;
    INIT_DELAYED_WORK(&ddata.detect_work, cpcap_usb_detect);
    platform_set_drvdata(pdev, ddata);
    ddata.vusb = devm_regulator_get(&pdev.dev, "vusb");
    if (IS_ERR(ddata.vusb))
    return PTR_ERR(ddata.vusb);
    error = regulator_enable(ddata.vusb);
    if (error)
    return error;
    generic_phy = devm_phy_create(ddata.dev, core::ptr::null_mut(), &ops);
    if (IS_ERR(generic_phy)) {
    error = PTR_ERR(generic_phy);
    goto out_reg_disable;
    }
    phy_set_drvdata(generic_phy, ddata);
    phy_provider = devm_of_phy_provider_register(ddata.dev,
    of_phy_simple_xlate);
    if (IS_ERR(phy_provider)) {
    error = PTR_ERR(phy_provider);
    goto out_reg_disable;
    }
    error = cpcap_usb_init_optional_pins(ddata);
    if (error)
    goto out_reg_disable;
    cpcap_usb_init_optional_gpios(ddata);
    error = cpcap_usb_init_iio(ddata);
    if (error)
    goto out_reg_disable;
    error = cpcap_usb_init_interrupts(pdev, ddata);
    if (error)
    goto out_reg_disable;
    usb_add_phy_dev(&ddata.phy);
    atomic_set(&ddata.active, 1);
    schedule_delayed_work(&ddata.detect_work, msecs_to_jiffies(1));
    return 0;
    out_reg_disable:
    regulator_disable(ddata.vusb);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn cpcap_usb_phy_remove(pdev: *mut platform_device) {
    static void cpcap_usb_phy_remove(struct platform_device *pdev)
    {
    struct cpcap_phy_ddata *ddata = platform_get_drvdata(pdev);
    int error;
    atomic_set(&ddata.active, 0);
    error = cpcap_usb_set_uart_mode(ddata);
    if (error)
    dev_err(ddata.dev, "could not set UART mode\n");
    cpcap_usb_try_musb_mailbox(ddata, MUSB_VBUS_OFF);
    usb_remove_phy(&ddata.phy);
    cancel_delayed_work_sync(&ddata.detect_work);
    regulator_disable(ddata.vusb);
    }
    static struct platform_driver cpcap_usb_phy_driver = {
    .probe		= cpcap_usb_phy_probe,
    .remove		= cpcap_usb_phy_remove,
    .driver		= {
    .name	= "cpcap-usb-phy",
    .of_match_table = of_match_ptr(cpcap_usb_phy_id_table),
    },
    };
    module_platform_driver(cpcap_usb_phy_driver);
    MODULE_ALIAS("platform:cpcap_usb");
    MODULE_AUTHOR("Tony Lindgren <tony@atomide.com>");
    MODULE_DESCRIPTION("CPCAP usb phy driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_CONSUMER");

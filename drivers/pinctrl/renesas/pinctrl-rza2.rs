//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/renesas/pinctrl-rza2.c
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
// Combined GPIO and pin controller support for Renesas RZ/A2 (R7S9210) SoC
//
// Copyright (C) 2018 Chris Brandt
//
// This pin controller/gpio combined driver supports Renesas devices of RZ/A2
// family.
//

pub const RZA2_PINS_PER_PORT: c_int = 8;

//
// Use 16 lower bits [15:0] for pin identifier
// Use 16 higher bits [31:16] for pin mux function
//

pub const MUX_FUNC_OFFS: c_int = 16;

    static const char port_names[] = "0123456789ABCDEFGHJKLM";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rza2_pinctrl_priv {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub desc: pinctrl_desc,
    pub pctl: *mut pinctrl_dev,
    pub gpio_range: pinctrl_gpio_range,
    pub npins: c_uint,
    pub /: *mut *mut mutex mutex; / serialize adding groups and functions,
    pub __counted_by(npins): pinctrl_pin_desc pins[],
}

pub const RZA2_PWPR: c_uint = 0x02ff	/* Write Protect 8-bit */;
pub const RZA2_PFENET: c_uint = 0x0820	/* Ethernet Pins 8-bit */;
pub const RZA2_PPOC: c_uint = 0x0900	/* Dedicated Pins 32-bit */;
pub const RZA2_PHMOMO: c_uint = 0x0980	/* Peripheral Pins 32-bit */;
pub const RZA2_PCKIO: c_uint = 0x09d0	/* CKIO Drive 8-bit */;
pub const RZA2_PDR_INPUT: c_uint = 0x02;
pub const RZA2_PDR_OUTPUT: c_uint = 0x03;
pub const RZA2_PDR_MASK: c_uint = 0x03;

    static void rza2_set_pin_function(void __iomem *pfc_base, u8 port, u8 pin,
    u8 func)
    {
    u16 mask16;
    u16 reg16;
    u8 reg8;
// Set pin to 'Non-use (Hi-z input protection)'
    reg16 = readw(pfc_base + RZA2_PDR(port));
    mask16 = RZA2_PDR_MASK << (pin * 2);
    reg16 &= ~mask16;
    writew(reg16, pfc_base + RZA2_PDR(port));
// Temporarily switch to GPIO
    reg8 = readb(pfc_base + RZA2_PMR(port));
    reg8 &= ~BIT(pin);
    writeb(reg8, pfc_base + RZA2_PMR(port));
// PFS Register Write Protect : OFF
    writeb(0x00, pfc_base + RZA2_PWPR);		/* B0WI=0, PFSWE=0 */
    writeb(PWPR_PFSWE, pfc_base + RZA2_PWPR);	/* B0WI=0, PFSWE=1 */
// Set Pin function (interrupt disabled, ISEL=0)
    writeb(func, pfc_base + RZA2_PFS(port, pin));
// PFS Register Write Protect : ON
    writeb(0x00, pfc_base + RZA2_PWPR);	/* B0WI=0, PFSWE=0 */
    writeb(0x80, pfc_base + RZA2_PWPR);	/* B0WI=1, PFSWE=0 */
// Port Mode  : Peripheral module pin functions
    reg8 = readb(pfc_base + RZA2_PMR(port));
    reg8 |= BIT(pin);
    writeb(reg8, pfc_base + RZA2_PMR(port));
    }
    static void rza2_pin_to_gpio(void __iomem *pfc_base, unsigned int offset,
    u8 dir)
    {
    let mut port: u8 = RZA2_PIN_ID_TO_PORT(offset);
    let mut pin: u8 = RZA2_PIN_ID_TO_PIN(offset);
    u16 mask16;
    u16 reg16;
    reg16 = readw(pfc_base + RZA2_PDR(port));
    mask16 = RZA2_PDR_MASK << (pin * 2);
    reg16 &= ~mask16;
    if (dir)
    reg16 |= RZA2_PDR_INPUT << (pin * 2);	/* pin as input */
    else
    reg16 |= RZA2_PDR_OUTPUT << (pin * 2);	/* pin as output */
    writew(reg16, pfc_base + RZA2_PDR(port));
    }
#[no_mangle]
unsafe extern "C" fn rza2_chip_get_direction(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int rza2_chip_get_direction(struct gpio_chip *chip, unsigned int offset)
    {
    struct rza2_pinctrl_priv *priv = gpiochip_get_data(chip);
    let mut port: u8 = RZA2_PIN_ID_TO_PORT(offset);
    let mut pin: u8 = RZA2_PIN_ID_TO_PIN(offset);
    u16 reg16;
    reg16 = readw(priv.base + RZA2_PDR(port));
    reg16 = (reg16 >> (pin * 2)) & RZA2_PDR_MASK;
    if (reg16 == RZA2_PDR_OUTPUT)
    return GPIO_LINE_DIRECTION_OUT;
    if (reg16 == RZA2_PDR_INPUT)
    return GPIO_LINE_DIRECTION_IN;
//
// This GPIO controller has a default Hi-Z state that is not input or
// output, so force the pin to input now.
//
    rza2_pin_to_gpio(priv.base, offset, 1);
    return GPIO_LINE_DIRECTION_IN;
    }
    static int rza2_chip_direction_input(struct gpio_chip *chip,
    unsigned int offset)
    {
    struct rza2_pinctrl_priv *priv = gpiochip_get_data(chip);
    rza2_pin_to_gpio(priv.base, offset, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rza2_chip_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int rza2_chip_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct rza2_pinctrl_priv *priv = gpiochip_get_data(chip);
    let mut port: u8 = RZA2_PIN_ID_TO_PORT(offset);
    let mut pin: u8 = RZA2_PIN_ID_TO_PIN(offset);
    return !!(readb(priv.base + RZA2_PIDR(port)) & BIT(pin));
    }
#[no_mangle]
unsafe extern "C" fn rza2_chip_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int rza2_chip_set(struct gpio_chip *chip, unsigned int offset, int value)
    {
    struct rza2_pinctrl_priv *priv = gpiochip_get_data(chip);
    let mut port: u8 = RZA2_PIN_ID_TO_PORT(offset);
    let mut pin: u8 = RZA2_PIN_ID_TO_PIN(offset);
    u8 new_value;
    new_value = readb(priv.base + RZA2_PODR(port));
    if (value)
    new_value |= BIT(pin);
    else
    new_value &= ~BIT(pin);
    writeb(new_value, priv.base + RZA2_PODR(port));
    return 0;
    }
    static int rza2_chip_direction_output(struct gpio_chip *chip,
    unsigned int offset, int val)
    {
    struct rza2_pinctrl_priv *priv = gpiochip_get_data(chip);
    rza2_chip_set(chip, offset, val);
    rza2_pin_to_gpio(priv.base, offset, 0);
    return 0;
    }
    static const char * const rza2_gpio_names[] = {
    "P0_0", "P0_1", "P0_2", "P0_3", "P0_4", "P0_5", "P0_6", "P0_7",
    "P1_0", "P1_1", "P1_2", "P1_3", "P1_4", "P1_5", "P1_6", "P1_7",
    "P2_0", "P2_1", "P2_2", "P2_3", "P2_4", "P2_5", "P2_6", "P2_7",
    "P3_0", "P3_1", "P3_2", "P3_3", "P3_4", "P3_5", "P3_6", "P3_7",
    "P4_0", "P4_1", "P4_2", "P4_3", "P4_4", "P4_5", "P4_6", "P4_7",
    "P5_0", "P5_1", "P5_2", "P5_3", "P5_4", "P5_5", "P5_6", "P5_7",
    "P6_0", "P6_1", "P6_2", "P6_3", "P6_4", "P6_5", "P6_6", "P6_7",
    "P7_0", "P7_1", "P7_2", "P7_3", "P7_4", "P7_5", "P7_6", "P7_7",
    "P8_0", "P8_1", "P8_2", "P8_3", "P8_4", "P8_5", "P8_6", "P8_7",
    "P9_0", "P9_1", "P9_2", "P9_3", "P9_4", "P9_5", "P9_6", "P9_7",
    "PA_0", "PA_1", "PA_2", "PA_3", "PA_4", "PA_5", "PA_6", "PA_7",
    "PB_0", "PB_1", "PB_2", "PB_3", "PB_4", "PB_5", "PB_6", "PB_7",
    "PC_0", "PC_1", "PC_2", "PC_3", "PC_4", "PC_5", "PC_6", "PC_7",
    "PD_0", "PD_1", "PD_2", "PD_3", "PD_4", "PD_5", "PD_6", "PD_7",
    "PE_0", "PE_1", "PE_2", "PE_3", "PE_4", "PE_5", "PE_6", "PE_7",
    "PF_0", "PF_1", "PF_2", "PF_3", "PF_4", "PF_5", "PF_6", "PF_7",
    "PG_0", "PG_1", "PG_2", "PG_3", "PG_4", "PG_5", "PG_6", "PG_7",
    "PH_0", "PH_1", "PH_2", "PH_3", "PH_4", "PH_5", "PH_6", "PH_7",
// port I does not exist
    "PJ_0", "PJ_1", "PJ_2", "PJ_3", "PJ_4", "PJ_5", "PJ_6", "PJ_7",
    "PK_0", "PK_1", "PK_2", "PK_3", "PK_4", "PK_5", "PK_6", "PK_7",
    "PL_0", "PL_1", "PL_2", "PL_3", "PL_4", "PL_5", "PL_6", "PL_7",
    "PM_0", "PM_1", "PM_2", "PM_3", "PM_4", "PM_5", "PM_6", "PM_7",
    };
    static struct gpio_chip chip = {
    .names = rza2_gpio_names,
    .base = -1,
    .request = pinctrl_gpio_request,
    .free = pinctrl_gpio_free,
    .get_direction = rza2_chip_get_direction,
    .direction_input = rza2_chip_direction_input,
    .direction_output = rza2_chip_direction_output,
    .get = rza2_chip_get,
    .set = rza2_chip_set,
    };
#[no_mangle]
unsafe extern "C" fn rza2_gpio_register(priv: *mut rza2_pinctrl_priv) -> c_int {
    static int rza2_gpio_register(struct rza2_pinctrl_priv *priv)
    {
    struct device_node *np = priv.dev.of_node;
    struct of_phandle_args of_args;
    int ret;
    chip.label = devm_kasprintf(priv.dev, GFP_KERNEL, "%pOFn", np);
    if (!chip.label)
    return -ENOMEM;
    chip.parent = priv.dev;
    chip.ngpio = priv.npins;
    ret = of_parse_phandle_with_fixed_args(np, "gpio-ranges", 3, 0,
    &of_args);
    if (ret) {
    dev_err(priv.dev, "Unable to parse gpio-ranges\n");
    return ret;
    }
    of_node_put(of_args.np);
    if ((of_args.args[0] != 0) ||
    (of_args.args[1] != 0) ||
    (of_args.args[2] != priv.npins)) {
    dev_err(priv.dev, "gpio-ranges does not match selected SOC\n");
    return -EINVAL;
    }
    priv.gpio_range.id = 0;
    priv.gpio_range.pin_base = priv.gpio_range.base = 0;
    priv.gpio_range.npins = priv.npins;
    priv.gpio_range.name = chip.label;
    priv.gpio_range.gc = &chip;
// Register our gpio chip with gpiolib
    ret = devm_gpiochip_add_data(priv.dev, &chip, priv);
    if (ret)
    return ret;
// Register pin range with pinctrl core
    pinctrl_add_gpio_range(priv.pctl, &priv.gpio_range);
    dev_dbg(priv.dev, "Registered gpio controller\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rza2_pinctrl_register(priv: *mut rza2_pinctrl_priv) -> c_int {
    static int rza2_pinctrl_register(struct rza2_pinctrl_priv *priv)
    {
    unsigned int i;
    int ret;
    priv.desc.pins = priv.pins;
    priv.desc.npins = priv.npins;
    for (i = 0; i < priv.npins; i++) {
    priv.pins[i].number = i;
    priv.pins[i].name = rza2_gpio_names[i];
    }
    ret = devm_pinctrl_register_and_init(priv.dev, &priv.desc, priv,
    &priv.pctl);
    if (ret) {
    dev_err(priv.dev, "pinctrl registration failed\n");
    return ret;
    }
    ret = pinctrl_enable(priv.pctl);
    if (ret) {
    dev_err(priv.dev, "pinctrl enable failed\n");
    return ret;
    }
    ret = rza2_gpio_register(priv);
    if (ret) {
    dev_err(priv.dev, "GPIO registration failed\n");
    return ret;
    }
    return 0;
    }
//
// For each DT node, create a single pin mapping. That pin mapping will only
// contain a single group of pins, and that group of pins will only have a
// single function that can be selected.
//
    static int rza2_dt_node_to_map(struct pinctrl_dev *pctldev,
    struct device_node *np,
    struct pinctrl_map **map,
    unsigned int *num_maps)
    {
    struct rza2_pinctrl_priv *priv = pinctrl_dev_get_drvdata(pctldev);
    unsigned int *pins, *psel_val;
    int i, ret, npins, gsel, fsel;
    struct property *of_pins;
    const char **pin_fn;
// Find out how many pins to map
    of_pins = of_find_property(np, "pinmux", core::ptr::null_mut());
    if (!of_pins) {
    dev_info(priv.dev, "Missing pinmux property\n");
    return -ENOENT;
    }
    npins = of_pins.length / sizeof(u32);
    pins = devm_kcalloc(priv.dev, npins, sizeof(*pins), GFP_KERNEL);
    psel_val = devm_kcalloc(priv.dev, npins, sizeof(*psel_val),
    GFP_KERNEL);
    pin_fn = devm_kzalloc(priv.dev, sizeof(*pin_fn), GFP_KERNEL);
    if (!pins || !psel_val || !pin_fn)
    return -ENOMEM;
// Collect pin locations and mux settings from DT properties
    for (i = 0; i < npins; ++i) {
    u32 value;
    ret = of_property_read_u32_index(np, "pinmux", i, &value);
    if (ret)
    return ret;
    pins[i] = value & MUX_PIN_ID_MASK;
    psel_val[i] = MUX_FUNC(value);
    }
    mutex_lock(&priv.mutex);
// Register a single pin group listing all the pins we read from DT
    gsel = pinctrl_generic_add_group(pctldev, np.name, pins, npins, core::ptr::null_mut());
    if (gsel < 0) {
    ret = gsel;
    goto unlock;
    }
//
// Register a single group function where the 'data' is an array PSEL
// register values read from DT.
//
    pin_fn[0] = np.name;
    fsel = pinmux_generic_add_function(pctldev, np.name, pin_fn, 1,
    psel_val);
    if (fsel < 0) {
    ret = fsel;
    goto remove_group;
    }
    dev_dbg(priv.dev, "Parsed %pOF with %d pins\n", np, npins);
// Create map where to retrieve function and mux settings from
// num_maps = 0;
// map = kzalloc_obj(**map);
    if (!*map) {
    ret = -ENOMEM;
    goto remove_function;
    }
    (*map).type = PIN_MAP_TYPE_MUX_GROUP;
    (*map).data.mux.group = np.name;
    (*map).data.mux.function = np.name;
// num_maps = 1;
    mutex_unlock(&priv.mutex);
    return 0;
    remove_function:
    pinmux_generic_remove_function(pctldev, fsel);
    remove_group:
    pinctrl_generic_remove_group(pctldev, gsel);
    unlock:
    mutex_unlock(&priv.mutex);
    dev_err(priv.dev, "Unable to parse DT node %s\n", np.name);
    return ret;
    }
    static void rza2_dt_free_map(struct pinctrl_dev *pctldev,
    struct pinctrl_map *map, unsigned int num_maps)
    {
    kfree(map);
    }
    static const struct pinctrl_ops rza2_pinctrl_ops = {
    .get_groups_count	= pinctrl_generic_get_group_count,
    .get_group_name		= pinctrl_generic_get_group_name,
    .get_group_pins		= pinctrl_generic_get_group_pins,
    .dt_node_to_map		= rza2_dt_node_to_map,
    .dt_free_map		= rza2_dt_free_map,
    };
    static int rza2_set_mux(struct pinctrl_dev *pctldev, unsigned int selector,
    unsigned int group)
    {
    struct rza2_pinctrl_priv *priv = pinctrl_dev_get_drvdata(pctldev);
    const struct function_desc *func;
    unsigned int i, *psel_val;
    struct group_desc *grp;
    grp = pinctrl_generic_get_group(pctldev, group);
    if (!grp)
    return -EINVAL;
    func = pinmux_generic_get_function(pctldev, selector);
    if (!func)
    return -EINVAL;
    psel_val = func.data;
    for (i = 0; i < grp.grp.npins; ++i) {
    dev_dbg(priv.dev, "Setting P%c_%d to PSEL=%d\n",
    port_names[RZA2_PIN_ID_TO_PORT(grp.grp.pins[i])],
    RZA2_PIN_ID_TO_PIN(grp.grp.pins[i]),
    psel_val[i]);
    rza2_set_pin_function(
    priv.base,
    RZA2_PIN_ID_TO_PORT(grp.grp.pins[i]),
    RZA2_PIN_ID_TO_PIN(grp.grp.pins[i]),
    psel_val[i]);
    }
    return 0;
    }
    static const struct pinmux_ops rza2_pinmux_ops = {
    .get_functions_count	= pinmux_generic_get_function_count,
    .get_function_name	= pinmux_generic_get_function_name,
    .get_function_groups	= pinmux_generic_get_function_groups,
    .set_mux		= rza2_set_mux,
    .strict			= true,
    };
#[no_mangle]
unsafe extern "C" fn rza2_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int rza2_pinctrl_probe(struct platform_device *pdev)
    {
    struct rza2_pinctrl_priv *priv;
    unsigned int npins;
    int ret;
    npins = (uintptr_t)of_device_get_match_data(&pdev.dev) *
    RZA2_PINS_PER_PORT;
    priv = devm_kzalloc(&pdev.dev, struct_size(priv, pins, npins), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.npins = npins;
    priv.dev = &pdev.dev;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    mutex_init(&priv.mutex);
    platform_set_drvdata(pdev, priv);
    priv.desc.name		= DRIVER_NAME;
    priv.desc.pctlops	= &rza2_pinctrl_ops;
    priv.desc.pmxops	= &rza2_pinmux_ops;
    priv.desc.owner	= THIS_MODULE;
    ret = rza2_pinctrl_register(priv);
    if (ret)
    return ret;
    dev_info(&pdev.dev, "Registered ports P0 - P%c\n",
    port_names[priv.desc.npins / RZA2_PINS_PER_PORT - 1]);
    return 0;
    }
    static const struct of_device_id rza2_pinctrl_of_match[] = {
    { .compatible = "renesas,r7s9210-pinctrl", .data = (void *)22, },
    { /* sentinel */ }
    };
    static struct platform_driver rza2_pinctrl_driver = {
    .driver = {
    .name = DRIVER_NAME,
    .of_match_table = rza2_pinctrl_of_match,
    },
    .probe = rza2_pinctrl_probe,
    };
#[no_mangle]
unsafe extern "C" fn rza2_pinctrl_init() -> int __init {
    static int __init rza2_pinctrl_init(void)
    {
    return platform_driver_register(&rza2_pinctrl_driver);
    }
    core_initcall(rza2_pinctrl_init);
    MODULE_AUTHOR("Chris Brandt <chris.brandt@renesas.com>");
    MODULE_DESCRIPTION("Pin and gpio controller driver for RZ/A2 SoC");

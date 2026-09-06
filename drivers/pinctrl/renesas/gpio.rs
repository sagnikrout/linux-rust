//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/renesas/gpio.c
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
// SuperH Pin Function Controller GPIO driver.
//
// Copyright (C) 2008 Magnus Damm
// Copyright (C) 2009 - 2012 Paul Mundt
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_pfc_gpio_data_reg {
    pub info: *const pinmux_data_reg,
    pub shadow: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_pfc_gpio_pin {
    pub dbit: u8,
    pub dreg: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_pfc_chip {
    pub pfc: *mut sh_pfc,
    pub gpio_chip: gpio_chip,
    pub mem: *mut sh_pfc_window,
    pub regs: *mut sh_pfc_gpio_data_reg,
    pub pins: *mut sh_pfc_gpio_pin,
}

    static struct sh_pfc *gpio_to_pfc(struct gpio_chip *gc)
    {
    struct sh_pfc_chip *chip = gpiochip_get_data(gc);
    return chip.pfc;
    }
    static void gpio_get_data_reg(struct sh_pfc_chip *chip, unsigned int offset,
    struct sh_pfc_gpio_data_reg **reg,
    unsigned int *bit)
    {
    let mut idx: c_int = sh_pfc_get_pin_index(chip.pfc, offset);
    struct sh_pfc_gpio_pin *gpio_pin = &chip.pins[idx];
// reg = &chip->regs[gpio_pin->dreg];
// bit = gpio_pin->dbit;
    }
    static u32 gpio_read_data_reg(struct sh_pfc_chip *chip,
    const struct pinmux_data_reg *dreg)
    {
    let mut address: phys_addr_t = dreg.reg;
    void __iomem *mem = address - chip.mem.phys + chip.mem.virt;
    return sh_pfc_read_raw_reg(mem, dreg.reg_width);
    }
    static void gpio_write_data_reg(struct sh_pfc_chip *chip,
    const struct pinmux_data_reg *dreg, u32 value)
    {
    let mut address: phys_addr_t = dreg.reg;
    void __iomem *mem = address - chip.mem.phys + chip.mem.virt;
    sh_pfc_write_raw_reg(mem, dreg.reg_width, value);
    }
#[no_mangle]
unsafe extern "C" fn gpio_setup_data_reg(chip: *mut sh_pfc_chip, idx: unsigned) {
    static void gpio_setup_data_reg(struct sh_pfc_chip *chip, unsigned idx)
    {
    struct sh_pfc *pfc = chip.pfc;
    struct sh_pfc_gpio_pin *gpio_pin = &chip.pins[idx];
    const struct sh_pfc_pin *pin = &pfc.info.pins[idx];
    const struct pinmux_data_reg *dreg;
    unsigned int bit;
    unsigned int i;
    for (i = 0, dreg = pfc.info.data_regs; dreg.reg_width; ++i, ++dreg) {
    for (bit = 0; bit < dreg.reg_width; bit++) {
    if (dreg.enum_ids[bit] == pin.enum_id) {
    gpio_pin.dreg = i;
    gpio_pin.dbit = bit;
    return;
    }
    }
    }
    BUG();
    }
#[no_mangle]
unsafe extern "C" fn gpio_setup_data_regs(chip: *mut sh_pfc_chip) -> c_int {
    static int gpio_setup_data_regs(struct sh_pfc_chip *chip)
    {
    struct sh_pfc *pfc = chip.pfc;
    const struct pinmux_data_reg *dreg;
    unsigned int i;
// Count the number of data registers, allocate memory and initialize
// them.
//
    for (i = 0; pfc.info.data_regs[i].reg_width; ++i)
    ;
    chip.regs = devm_kcalloc(pfc.dev, i, sizeof(*chip.regs),
    GFP_KERNEL);
    if (chip.regs == core::ptr::null_mut())
    return -ENOMEM;
    for (i = 0, dreg = pfc.info.data_regs; dreg.reg_width; ++i, ++dreg) {
    chip.regs[i].info = dreg;
    chip.regs[i].shadow = gpio_read_data_reg(chip, dreg);
    }
    for (i = 0; i < pfc.info.nr_pins; i++) {
    if (pfc.info.pins[i].enum_id == 0)
    continue;
    gpio_setup_data_reg(chip, i);
    }
    return 0;
    }
// -----------------------------------------------------------------------------
// Pin GPIOs
//
#[no_mangle]
unsafe extern "C" fn gpio_pin_request(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int gpio_pin_request(struct gpio_chip *gc, unsigned offset)
    {
    struct sh_pfc *pfc = gpio_to_pfc(gc);
    let mut idx: c_int = sh_pfc_get_pin_index(pfc, offset);
    if (idx < 0 || pfc.info.pins[idx].enum_id == 0)
    return -EINVAL;
    return pinctrl_gpio_request(gc, offset);
    }
#[no_mangle]
unsafe extern "C" fn gpio_pin_free(gc: *mut gpio_chip, offset: unsigned) {
    static void gpio_pin_free(struct gpio_chip *gc, unsigned offset)
    {
    return pinctrl_gpio_free(gc, offset);
    }
    static void gpio_pin_set_value(struct sh_pfc_chip *chip, unsigned offset,
    int value)
    {
    struct sh_pfc_gpio_data_reg *reg;
    unsigned int bit;
    unsigned int pos;
    gpio_get_data_reg(chip, offset, &reg, &bit);
    pos = reg.info.reg_width - (bit + 1);
    if (value)
    reg.shadow |= BIT(pos);
    else
    reg.shadow &= ~BIT(pos);
    gpio_write_data_reg(chip, reg.info, reg.shadow);
    }
#[no_mangle]
unsafe extern "C" fn gpio_pin_direction_input(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int gpio_pin_direction_input(struct gpio_chip *gc, unsigned offset)
    {
    return pinctrl_gpio_direction_input(gc, offset);
    }
    static int gpio_pin_direction_output(struct gpio_chip *gc, unsigned offset,
    int value)
    {
    gpio_pin_set_value(gpiochip_get_data(gc), offset, value);
    return pinctrl_gpio_direction_output(gc, offset);
    }
#[no_mangle]
unsafe extern "C" fn gpio_pin_get(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int gpio_pin_get(struct gpio_chip *gc, unsigned offset)
    {
    struct sh_pfc_chip *chip = gpiochip_get_data(gc);
    struct sh_pfc_gpio_data_reg *reg;
    unsigned int bit;
    unsigned int pos;
    gpio_get_data_reg(chip, offset, &reg, &bit);
    pos = reg.info.reg_width - (bit + 1);
    return (gpio_read_data_reg(chip, reg.info) >> pos) & 1;
    }
#[no_mangle]
unsafe extern "C" fn gpio_pin_set(gc: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int gpio_pin_set(struct gpio_chip *gc, unsigned int offset, int value)
    {
    gpio_pin_set_value(gpiochip_get_data(gc), offset, value);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_pin_to_irq(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int gpio_pin_to_irq(struct gpio_chip *gc, unsigned offset)
    {
    struct sh_pfc *pfc = gpio_to_pfc(gc);
    unsigned int i, k;
    for (i = 0; i < pfc.info.gpio_irq_size; i++) {
    const short *gpios = pfc.info.gpio_irq[i].gpios;
    for (k = 0; gpios[k] >= 0; k++) {
    if (gpios[k] == offset)
    return pfc.irqs[i];
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_pin_setup(chip: *mut sh_pfc_chip) -> c_int {
    static int gpio_pin_setup(struct sh_pfc_chip *chip)
    {
    struct sh_pfc *pfc = chip.pfc;
    struct gpio_chip *gc = &chip.gpio_chip;
    int ret;
    chip.pins = devm_kcalloc(pfc.dev,
    pfc.info.nr_pins, sizeof(*chip.pins),
    GFP_KERNEL);
    if (chip.pins == core::ptr::null_mut())
    return -ENOMEM;
    ret = gpio_setup_data_regs(chip);
    if (ret < 0)
    return ret;
    gc.request = gpio_pin_request;
    gc.free = gpio_pin_free;
    gc.direction_input = gpio_pin_direction_input;
    gc.get = gpio_pin_get;
    gc.direction_output = gpio_pin_direction_output;
    gc.set = gpio_pin_set;
    gc.to_irq = gpio_pin_to_irq;
    gc.label = pfc.info.name;
    gc.parent = pfc.dev;
    gc.owner = THIS_MODULE;
    gc.base = IS_ENABLED(CONFIG_PINCTRL_SH_FUNC_GPIO) ? 0 : -1;
    gc.ngpio = pfc.nr_gpio_pins;
    return 0;
    }
// -----------------------------------------------------------------------------
// Function GPIOs
//

#[no_mangle]
unsafe extern "C" fn gpio_function_request(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int gpio_function_request(struct gpio_chip *gc, unsigned offset)
    {
    struct sh_pfc *pfc = gpio_to_pfc(gc);
    let mut mark: c_uint = pfc.info.func_gpios[offset].enum_id;
    unsigned long flags;
    int ret;
    dev_notice_once(pfc.dev,
    "Use of GPIO API for function requests is deprecated, convert to pinctrl\n");
    if (mark == 0)
    return -EINVAL;
    spin_lock_irqsave(&pfc.lock, flags);
    ret = sh_pfc_config_mux(pfc, mark, PINMUX_TYPE_FUNCTION);
    spin_unlock_irqrestore(&pfc.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gpio_function_setup(chip: *mut sh_pfc_chip) -> c_int {
    static int gpio_function_setup(struct sh_pfc_chip *chip)
    {
    struct sh_pfc *pfc = chip.pfc;
    struct gpio_chip *gc = &chip.gpio_chip;
    gc.request = gpio_function_request;
    gc.label = pfc.info.name;
    gc.owner = THIS_MODULE;
    gc.base = pfc.nr_gpio_pins;
    gc.ngpio = pfc.info.nr_func_gpios;
    return 0;
    }

// -----------------------------------------------------------------------------
// Register/unregister
//
    static struct sh_pfc_chip *
    sh_pfc_add_gpiochip(struct sh_pfc *pfc, int(*setup)(struct sh_pfc_chip *),
    struct sh_pfc_window *mem)
    {
    struct sh_pfc_chip *chip;
    int ret;
    chip = devm_kzalloc(pfc.dev, sizeof(*chip), GFP_KERNEL);
    if (unlikely(!chip))
    return ERR_PTR(-ENOMEM);
    chip.mem = mem;
    chip.pfc = pfc;
    ret = setup(chip);
    if (ret < 0)
    return ERR_PTR(ret);
    ret = devm_gpiochip_add_data(pfc.dev, &chip.gpio_chip, chip);
    if (unlikely(ret < 0))
    return ERR_PTR(ret);
    dev_info(pfc.dev, "%s handling gpio %u . %u\n",
    chip.gpio_chip.label, chip.gpio_chip.base,
    chip.gpio_chip.base + chip.gpio_chip.ngpio - 1);
    return chip;
    }
#[no_mangle]
pub unsafe extern "C" fn sh_pfc_register_gpiochip(pfc: *mut sh_pfc) -> c_int {
    int sh_pfc_register_gpiochip(struct sh_pfc *pfc)
    {
    struct sh_pfc_chip *chip;
    phys_addr_t address;
    unsigned int i;
    if (pfc.info.data_regs == core::ptr::null_mut())
    return 0;
// Find the memory window that contains the GPIO registers. Boards that
// register a separate GPIO device will not supply a memory resource
// that covers the data registers. In that case don't try to handle
// GPIOs.
//
    address = pfc.info.data_regs[0].reg;
    for (i = 0; i < pfc.num_windows; ++i) {
    struct sh_pfc_window *window = &pfc.windows[i];
    if (address >= window.phys &&
    address < window.phys + window.size)
    break;
    }
    if (i == pfc.num_windows)
    return 0;
// If we have IRQ resources make sure their number is correct.
    if (pfc.num_irqs != pfc.info.gpio_irq_size) {
    dev_err(pfc.dev, "invalid number of IRQ resources\n");
    return -EINVAL;
    }
// Register the real GPIOs chip.
    chip = sh_pfc_add_gpiochip(pfc, gpio_pin_setup, &pfc.windows[i]);
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    pfc.gpio = chip;
    if (IS_ENABLED(CONFIG_OF) && pfc.dev.of_node)
    return 0;

//
// Register the GPIO to pin mappings. As pins with GPIO ports
// must come first in the ranges, skip the pins without GPIO
// ports by stopping at the first range that contains such a
// pin.
//
    for (i = 0; i < pfc.nr_ranges; ++i) {
    const struct sh_pfc_pin_range *range = &pfc.ranges[i];
    int ret;
    if (range.start >= pfc.nr_gpio_pins)
    break;
    ret = gpiochip_add_pin_range(&chip.gpio_chip,
    dev_name(pfc.dev), range.start, range.start,
    range.end - range.start + 1);
    if (ret < 0)
    return ret;
    }
// Register the function GPIOs chip.
    if (pfc.info.nr_func_gpios) {
    chip = sh_pfc_add_gpiochip(pfc, gpio_function_setup, core::ptr::null_mut());
    if (IS_ERR(chip))
    return PTR_ERR(chip);
    }

    return 0;
    }

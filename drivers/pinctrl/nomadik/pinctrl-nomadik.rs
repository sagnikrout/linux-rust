//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/nomadik/pinctrl-nomadik.c
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
// Pinmux & pinconf driver for the IP block found in the Nomadik SoC. This
// depends on gpio-nomadik and some handling is intertwined; see nmk_gpio_chips
// which is used by this driver to access the GPIO banks array.
//
// Copyright (C) 2008,2009 STMicroelectronics
// Copyright (C) 2009 Alessandro Rubini <rubini@unipv.it>
// Rewritten based on work by Prafulla WADASKAR <prafulla.wadaskar@st.com>
// Copyright (C) 2011-2013 Linus Walleij <linus.walleij@linaro.org>
//

// Since we request GPIOs from ourself

//
// pin configurations are represented by 32-bit integers:
//
// bit  0.. 8 - Pin Number (512 Pins Maximum)
// bit  9..10 - Alternate Function Selection
// bit 11..12 - Pull up/down state
// bit     13 - Sleep mode behaviour
// bit     14 - Direction
// bit     15 - Value (if output)
// bit 16..18 - SLPM pull up/down state
// bit 19..20 - SLPM direction
// bit 21..22 - SLPM Value (if output)
// bit 23..25 - PDIS value (if input)
// bit	26 - Gpio mode
// bit	27 - Sleep mode
//
// to facilitate the definition, the following macros are provided
//
// PIN_CFG_DEFAULT - default config (0):
// pull up/down = disabled
// sleep mode = input/wakeup
// direction = input
// value = low
// SLPM direction = same as normal
// SLPM pull = same as normal
// SLPM value = same as normal
//
// PIN_CFG	   - default config with alternate function
//
pub const PIN_NUM_MASK: c_uint = 0x1ff;

pub const PIN_ALT_SHIFT: c_int = 9;

pub const PIN_PULL_SHIFT: c_int = 11;

pub const PIN_SLPM_SHIFT: c_int = 13;

// These two replace the above in DB8500v2+

pub const PIN_DIR_SHIFT: c_int = 14;

pub const PIN_VAL_SHIFT: c_int = 15;

pub const PIN_SLPM_PULL_SHIFT: c_int = 16;

    (((x) & PIN_SLPM_PULL_MASK) >> PIN_SLPM_PULL_SHIFT)

    ((1 + NMK_GPIO_PULL_NONE) << PIN_SLPM_PULL_SHIFT)

    ((1 + NMK_GPIO_PULL_UP) << PIN_SLPM_PULL_SHIFT)

    ((1 + NMK_GPIO_PULL_DOWN) << PIN_SLPM_PULL_SHIFT)
pub const PIN_SLPM_DIR_SHIFT: c_int = 19;

    (((x) & PIN_SLPM_DIR_MASK) >> PIN_SLPM_DIR_SHIFT)

pub const PIN_SLPM_VAL_SHIFT: c_int = 21;

    (((x) & PIN_SLPM_VAL_MASK) >> PIN_SLPM_VAL_SHIFT)

pub const PIN_SLPM_PDIS_SHIFT: c_int = 23;

    (((x) & PIN_SLPM_PDIS_MASK) >> PIN_SLPM_PDIS_SHIFT)

pub const PIN_LOWEMI_SHIFT: c_int = 25;

pub const PIN_GPIOMODE_SHIFT: c_int = 26;

pub const PIN_SLEEPMODE_SHIFT: c_int = 27;

// Shortcuts.  Use these instead of separate DIR, PULL, and VAL.

    (PIN_CFG_DEFAULT |\
    (PIN_NUM(num) | PIN_##alt))

    (PIN_CFG_DEFAULT |\
    (PIN_NUM(num) | PIN_##alt | PIN_INPUT_##pull))

    (PIN_CFG_DEFAULT |\
    (PIN_NUM(num) | PIN_##alt | PIN_OUTPUT_##val))
//
// struct nmk_pinctrl - state container for the Nomadik pin controller
// @dev: containing device pointer
// @pctl: corresponding pin controller device
// @soc: SoC data for this specific chip
// @prcm_base: PRCM register range virtual base
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nmk_pinctrl {
    pub dev: *mut device,
    pub pctl: *mut pinctrl_dev,
    pub soc: *const nmk_pinctrl_soc_data,
    pub prcm_base: *mut void __iomem,
}

// See nmk_gpio_populate_chip() that fills this array.
    struct nmk_gpio_chip *nmk_gpio_chips[NMK_MAX_BANKS];
    DEFINE_SPINLOCK(nmk_gpio_slpm_lock);
    static void __nmk_gpio_set_mode(struct nmk_gpio_chip *nmk_chip,
    unsigned int offset, int gpio_mode)
    {
    u32 afunc, bfunc;
    afunc = readl(nmk_chip.addr + NMK_GPIO_AFSLA) & ~BIT(offset);
    bfunc = readl(nmk_chip.addr + NMK_GPIO_AFSLB) & ~BIT(offset);
    if (gpio_mode & NMK_GPIO_ALT_A)
    afunc |= BIT(offset);
    if (gpio_mode & NMK_GPIO_ALT_B)
    bfunc |= BIT(offset);
    writel(afunc, nmk_chip.addr + NMK_GPIO_AFSLA);
    writel(bfunc, nmk_chip.addr + NMK_GPIO_AFSLB);
    }
    static void __nmk_gpio_set_pull(struct nmk_gpio_chip *nmk_chip,
    unsigned int offset, enum nmk_gpio_pull pull)
    {
    u32 pdis;
    pdis = readl(nmk_chip.addr + NMK_GPIO_PDIS);
    if (pull == NMK_GPIO_PULL_NONE) {
    pdis |= BIT(offset);
    nmk_chip.pull_up &= ~BIT(offset);
    } else {
    pdis &= ~BIT(offset);
    }
    writel(pdis, nmk_chip.addr + NMK_GPIO_PDIS);
    if (pull == NMK_GPIO_PULL_UP) {
    nmk_chip.pull_up |= BIT(offset);
    writel(BIT(offset), nmk_chip.addr + NMK_GPIO_DATS);
    } else if (pull == NMK_GPIO_PULL_DOWN) {
    nmk_chip.pull_up &= ~BIT(offset);
    writel(BIT(offset), nmk_chip.addr + NMK_GPIO_DATC);
    }
    }
    static void __nmk_gpio_set_lowemi(struct nmk_gpio_chip *nmk_chip,
    unsigned int offset, bool lowemi)
    {
    let mut enabled: bool = nmk_chip.lowemi & BIT(offset);
    if (lowemi == enabled)
    return;
    if (lowemi)
    nmk_chip.lowemi |= BIT(offset);
    else
    nmk_chip.lowemi &= ~BIT(offset);
    writel_relaxed(nmk_chip.lowemi,
    nmk_chip.addr + NMK_GPIO_LOWEMI);
    }
    static void __nmk_gpio_make_input(struct nmk_gpio_chip *nmk_chip,
    unsigned int offset)
    {
    writel(BIT(offset), nmk_chip.addr + NMK_GPIO_DIRC);
    }
    static void __nmk_gpio_set_mode_safe(struct nmk_gpio_chip *nmk_chip,
    unsigned int offset, int gpio_mode,
    bool glitch)
    {
    let mut rwimsc: u32 = nmk_chip.rwimsc;
    let mut fwimsc: u32 = nmk_chip.fwimsc;
    if (glitch && nmk_chip.set_ioforce) {
    let mut bit: u32 = BIT(offset);
// Prevent spurious wakeups
    writel(rwimsc & ~bit, nmk_chip.addr + NMK_GPIO_RWIMSC);
    writel(fwimsc & ~bit, nmk_chip.addr + NMK_GPIO_FWIMSC);
    nmk_chip.set_ioforce(true);
    }
    __nmk_gpio_set_mode(nmk_chip, offset, gpio_mode);
    if (glitch && nmk_chip.set_ioforce) {
    nmk_chip.set_ioforce(false);
    writel(rwimsc, nmk_chip.addr + NMK_GPIO_RWIMSC);
    writel(fwimsc, nmk_chip.addr + NMK_GPIO_FWIMSC);
    }
    }
    static void
    nmk_gpio_disable_lazy_irq(struct nmk_gpio_chip *nmk_chip, unsigned int offset)
    {
    let mut falling: u32 = nmk_chip.fimsc & BIT(offset);
    let mut rising: u32 = nmk_chip.rimsc & BIT(offset);
    let mut gpio: c_int = nmk_chip.chip.base + offset;
    let mut irq: c_int = irq_find_mapping(nmk_chip.chip.irq.domain, offset);
    struct irq_data *d = irq_get_irq_data(irq);
    if (!rising && !falling)
    return;
    if (!d || !irqd_irq_disabled(d))
    return;
    if (rising) {
    nmk_chip.rimsc &= ~BIT(offset);
    writel_relaxed(nmk_chip.rimsc,
    nmk_chip.addr + NMK_GPIO_RIMSC);
    }
    if (falling) {
    nmk_chip.fimsc &= ~BIT(offset);
    writel_relaxed(nmk_chip.fimsc,
    nmk_chip.addr + NMK_GPIO_FIMSC);
    }
    dev_dbg(nmk_chip.chip.parent, "%d: clearing interrupt mask\n", gpio);
    }
#[no_mangle]
unsafe extern "C" fn nmk_write_masked(reg: *mut void __iomem, mask: u32, value: u32) {
    static void nmk_write_masked(void __iomem *reg, u32 mask, u32 value)
    {
    u32 val;
    val = readl(reg);
    val = ((val & ~mask) | (value & mask));
    writel(val, reg);
    }
    static void nmk_prcm_altcx_set_mode(struct nmk_pinctrl *npct,
    unsigned int offset, unsigned int alt_num)
    {
    int i;
    u16 reg;
    u8 bit;
    u8 alt_index;
    const struct prcm_gpiocr_altcx_pin_desc *pin_desc;
    const u16 *gpiocr_regs;
    if (!npct.prcm_base)
    return;
    if (alt_num > PRCM_IDX_GPIOCR_ALTC_MAX) {
    dev_err(npct.dev, "PRCM GPIOCR: alternate-C%i is invalid\n",
    alt_num);
    return;
    }
    for (i = 0 ; i < npct.soc.npins_altcx ; i++) {
    if (npct.soc.altcx_pins[i].pin == offset)
    break;
    }
    if (i == npct.soc.npins_altcx) {
    dev_dbg(npct.dev, "PRCM GPIOCR: pin %i is not found\n",
    offset);
    return;
    }
    pin_desc = npct.soc.altcx_pins + i;
    gpiocr_regs = npct.soc.prcm_gpiocr_registers;
//
// If alt_num is NULL, just clear current ALTCx selection
// to make sure we come back to a pure ALTC selection
//
    if (!alt_num) {
    for (i = 0 ; i < PRCM_IDX_GPIOCR_ALTC_MAX ; i++) {
    if (pin_desc.altcx[i].used) {
    reg = gpiocr_regs[pin_desc.altcx[i].reg_index];
    bit = pin_desc.altcx[i].control_bit;
    if (readl(npct.prcm_base + reg) & BIT(bit)) {
    nmk_write_masked(npct.prcm_base + reg, BIT(bit), 0);
    dev_dbg(npct.dev,
    "PRCM GPIOCR: pin %i: alternate-C%i has been disabled\n",
    offset, i + 1);
    }
    }
    }
    return;
    }
    alt_index = alt_num - 1;
    if (!pin_desc.altcx[alt_index].used) {
    dev_warn(npct.dev,
    "PRCM GPIOCR: pin %i: alternate-C%i does not exist\n",
    offset, alt_num);
    return;
    }
//
// Check if any other ALTCx functions are activated on this pin
// and disable it first.
//
    for (i = 0 ; i < PRCM_IDX_GPIOCR_ALTC_MAX ; i++) {
    if (i == alt_index)
    continue;
    if (pin_desc.altcx[i].used) {
    reg = gpiocr_regs[pin_desc.altcx[i].reg_index];
    bit = pin_desc.altcx[i].control_bit;
    if (readl(npct.prcm_base + reg) & BIT(bit)) {
    nmk_write_masked(npct.prcm_base + reg, BIT(bit), 0);
    dev_dbg(npct.dev,
    "PRCM GPIOCR: pin %i: alternate-C%i has been disabled\n",
    offset, i + 1);
    }
    }
    }
    reg = gpiocr_regs[pin_desc.altcx[alt_index].reg_index];
    bit = pin_desc.altcx[alt_index].control_bit;
    dev_dbg(npct.dev, "PRCM GPIOCR: pin %i: alternate-C%i has been selected\n",
    offset, alt_index + 1);
    nmk_write_masked(npct.prcm_base + reg, BIT(bit), BIT(bit));
    }
//
// Safe sequence used to switch IOs between GPIO and Alternate-C mode:
// - Save SLPM registers
// - Set SLPM=0 for the IOs you want to switch and others to 1
// - Configure the GPIO registers for the IOs that are being switched
// - Set IOFORCE=1
// - Modify the AFLSA/B registers for the IOs that are being switched
// - Set IOFORCE=0
// - Restore SLPM registers
// - Any spurious wake up event during switch sequence to be ignored and
// cleared
//
#[no_mangle]
unsafe extern "C" fn nmk_gpio_glitch_slpm_init(slpm: *mut c_uint) -> c_int {
    static int nmk_gpio_glitch_slpm_init(unsigned int *slpm)
    {
    int i, j, ret;
    for (i = 0; i < NMK_MAX_BANKS; i++) {
    struct nmk_gpio_chip *chip = nmk_gpio_chips[i];
    let mut temp: c_uint = slpm[i];
    if (!chip)
    break;
    ret = clk_enable(chip.clk);
    if (ret) {
    for (j = 0; j < i; j++) {
    chip = nmk_gpio_chips[j];
    clk_disable(chip.clk);
    }
    return ret;
    }
    slpm[i] = readl(chip.addr + NMK_GPIO_SLPC);
    writel(temp, chip.addr + NMK_GPIO_SLPC);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nmk_gpio_glitch_slpm_restore(slpm: *mut c_uint) {
    static void nmk_gpio_glitch_slpm_restore(unsigned int *slpm)
    {
    int i;
    for (i = 0; i < NMK_MAX_BANKS; i++) {
    struct nmk_gpio_chip *chip = nmk_gpio_chips[i];
    if (!chip)
    break;
    writel(slpm[i], chip.addr + NMK_GPIO_SLPC);
    clk_disable(chip.clk);
    }
    }
// Only called by gpio-nomadik but requires knowledge of struct nmk_pinctrl.
#[no_mangle]
pub unsafe extern "C" fn nmk_prcm_gpiocr_get_mode(pctldev: *mut pinctrl_dev, gpio: c_int) -> int __maybe_unused {
    int __maybe_unused nmk_prcm_gpiocr_get_mode(struct pinctrl_dev *pctldev, int gpio)
    {
    int i;
    u16 reg;
    u8 bit;
    struct nmk_pinctrl *npct = pinctrl_dev_get_drvdata(pctldev);
    const struct prcm_gpiocr_altcx_pin_desc *pin_desc;
    const u16 *gpiocr_regs;
    if (!npct.prcm_base)
    return NMK_GPIO_ALT_C;
    for (i = 0; i < npct.soc.npins_altcx; i++) {
    if (npct.soc.altcx_pins[i].pin == gpio)
    break;
    }
    if (i == npct.soc.npins_altcx)
    return NMK_GPIO_ALT_C;
    pin_desc = npct.soc.altcx_pins + i;
    gpiocr_regs = npct.soc.prcm_gpiocr_registers;
    for (i = 0; i < PRCM_IDX_GPIOCR_ALTC_MAX; i++) {
    if (pin_desc.altcx[i].used) {
    reg = gpiocr_regs[pin_desc.altcx[i].reg_index];
    bit = pin_desc.altcx[i].control_bit;
    if (readl(npct.prcm_base + reg) & BIT(bit))
    return NMK_GPIO_ALT_C + i + 1;
    }
    }
    return NMK_GPIO_ALT_C;
    }
#[no_mangle]
unsafe extern "C" fn nmk_get_groups_cnt(pctldev: *mut pinctrl_dev) -> c_int {
    static int nmk_get_groups_cnt(struct pinctrl_dev *pctldev)
    {
    struct nmk_pinctrl *npct = pinctrl_dev_get_drvdata(pctldev);
    return npct.soc.ngroups;
    }
    static const char *nmk_get_group_name(struct pinctrl_dev *pctldev,
    unsigned int selector)
    {
    struct nmk_pinctrl *npct = pinctrl_dev_get_drvdata(pctldev);
    return npct.soc.groups[selector].grp.name;
    }
    static int nmk_get_group_pins(struct pinctrl_dev *pctldev, unsigned int selector,
    const unsigned int **pins,
    unsigned int *num_pins)
    {
    struct nmk_pinctrl *npct = pinctrl_dev_get_drvdata(pctldev);
// pins = npct->soc->groups[selector].grp.pins;
// num_pins = npct->soc->groups[selector].grp.npins;
    return 0;
    }
// This makes the mapping from pin number to a GPIO chip. We also return the pin
// offset in the GPIO chip for convenience (and to avoid a second loop).
//
    static struct nmk_gpio_chip *find_nmk_gpio_from_pin(unsigned int pin,
    unsigned int *offset)
    {
    int i, j = 0;
    struct nmk_gpio_chip *nmk_gpio;
// We assume that pins are allocated in bank order.
    for (i = 0; i < NMK_MAX_BANKS; i++) {
    nmk_gpio = nmk_gpio_chips[i];
    if (!nmk_gpio)
    continue;
    if (pin >= j && pin < j + nmk_gpio.chip.ngpio) {
    if (offset)
// offset = pin - j;
    return nmk_gpio;
    }
    j += nmk_gpio.chip.ngpio;
    }
    return core::ptr::null_mut();
    }
    static struct gpio_chip *find_gc_from_pin(unsigned int pin)
    {
    struct nmk_gpio_chip *nmk_gpio = find_nmk_gpio_from_pin(pin, core::ptr::null_mut());
    if (nmk_gpio)
    return &nmk_gpio.chip;
    return core::ptr::null_mut();
    }
    static void nmk_pin_dbg_show(struct pinctrl_dev *pctldev, struct seq_file *s,
    unsigned int offset)
    {
    struct gpio_chip *chip = find_gc_from_pin(offset);
    if (!chip) {
    seq_printf(s, "invalid pin offset");
    return;
    }
    nmk_gpio_dbg_show_one(s, pctldev, chip, offset - chip.base);
    }
    static int nmk_dt_add_map_mux(struct pinctrl_map **map, unsigned int *reserved_maps,
    unsigned int *num_maps, const char *group,
    const char *function)
    {
    if (*num_maps == *reserved_maps)
    return -ENOSPC;
    (*map)[*num_maps].type = PIN_MAP_TYPE_MUX_GROUP;
    (*map)[*num_maps].data.mux.group = group;
    (*map)[*num_maps].data.mux.function = function;
    (*num_maps)++;
    return 0;
    }
    static int nmk_dt_add_map_configs(struct pinctrl_map **map,
    unsigned int *reserved_maps,
    unsigned int *num_maps, const char *group,
    unsigned long *configs, unsigned int num_configs)
    {
    unsigned long *dup_configs;
    if (*num_maps == *reserved_maps)
    return -ENOSPC;
    dup_configs = kmemdup_array(configs, num_configs, sizeof(*dup_configs), GFP_KERNEL);
    if (!dup_configs)
    return -ENOMEM;
    (*map)[*num_maps].type = PIN_MAP_TYPE_CONFIGS_PIN;
    (*map)[*num_maps].data.configs.group_or_pin = group;
    (*map)[*num_maps].data.configs.configs = dup_configs;
    (*map)[*num_maps].data.configs.num_configs = num_configs;
    (*num_maps)++;
    return 0;
    }

    .size = ARRAY_SIZE(y), }
    static const unsigned long nmk_pin_input_modes[] = {
    PIN_INPUT_NOPULL,
    PIN_INPUT_PULLUP,
    PIN_INPUT_PULLDOWN,
    };
    static const unsigned long nmk_pin_output_modes[] = {
    PIN_OUTPUT_LOW,
    PIN_OUTPUT_HIGH,
    PIN_DIR_OUTPUT,
    };
    static const unsigned long nmk_pin_sleep_modes[] = {
    PIN_SLEEPMODE_DISABLED,
    PIN_SLEEPMODE_ENABLED,
    };
    static const unsigned long nmk_pin_sleep_input_modes[] = {
    PIN_SLPM_INPUT_NOPULL,
    PIN_SLPM_INPUT_PULLUP,
    PIN_SLPM_INPUT_PULLDOWN,
    PIN_SLPM_DIR_INPUT,
    };
    static const unsigned long nmk_pin_sleep_output_modes[] = {
    PIN_SLPM_OUTPUT_LOW,
    PIN_SLPM_OUTPUT_HIGH,
    PIN_SLPM_DIR_OUTPUT,
    };
    static const unsigned long nmk_pin_sleep_wakeup_modes[] = {
    PIN_SLPM_WAKEUP_DISABLE,
    PIN_SLPM_WAKEUP_ENABLE,
    };
    static const unsigned long nmk_pin_gpio_modes[] = {
    PIN_GPIOMODE_DISABLED,
    PIN_GPIOMODE_ENABLED,
    };
    static const unsigned long nmk_pin_sleep_pdis_modes[] = {
    PIN_SLPM_PDIS_DISABLED,
    PIN_SLPM_PDIS_ENABLED,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nmk_cfg_param {
    pub property: *const c_char,
    pub config: c_ulong,
    pub choice: *const c_ulong,
    pub size: c_int,
}

    static const struct nmk_cfg_param nmk_cfg_params[] = {
    NMK_CONFIG_PIN_ARRAY("ste,input",		nmk_pin_input_modes),
    NMK_CONFIG_PIN_ARRAY("ste,output",		nmk_pin_output_modes),
    NMK_CONFIG_PIN_ARRAY("ste,sleep",		nmk_pin_sleep_modes),
    NMK_CONFIG_PIN_ARRAY("ste,sleep-input",		nmk_pin_sleep_input_modes),
    NMK_CONFIG_PIN_ARRAY("ste,sleep-output",	nmk_pin_sleep_output_modes),
    NMK_CONFIG_PIN_ARRAY("ste,sleep-wakeup",	nmk_pin_sleep_wakeup_modes),
    NMK_CONFIG_PIN_ARRAY("ste,gpio",		nmk_pin_gpio_modes),
    NMK_CONFIG_PIN_ARRAY("ste,sleep-pull-disable",	nmk_pin_sleep_pdis_modes),
    };
#[no_mangle]
unsafe extern "C" fn nmk_dt_pin_config(index: c_int, val: c_int, config: *mut c_ulong) -> c_int {
    static int nmk_dt_pin_config(int index, int val, unsigned long *config)
    {
    if (!nmk_cfg_params[index].choice) {
// config = nmk_cfg_params[index].config;
    } else {
// test if out of range
    if  (val < nmk_cfg_params[index].size) {
// config = nmk_cfg_params[index].config |
    nmk_cfg_params[index].choice[val];
    }
    }
    return 0;
    }
    static const char *nmk_find_pin_name(struct pinctrl_dev *pctldev, const char *pin_name)
    {
    int i, pin_number;
    struct nmk_pinctrl *npct = pinctrl_dev_get_drvdata(pctldev);
    if (sscanf((char *)pin_name, "GPIO%d", &pin_number) == 1)
    for (i = 0; i < npct.soc.npins; i++)
    if (npct.soc.pins[i].number == pin_number)
    return npct.soc.pins[i].name;
    return core::ptr::null_mut();
    }
    static bool nmk_pinctrl_dt_get_config(struct device_node *np,
    unsigned long *configs)
    {
    let mut has_config: bool = 0;
    let mut cfg: c_ulong = 0;
    int i, val, ret;
    for (i = 0; i < ARRAY_SIZE(nmk_cfg_params); i++) {
    ret = of_property_read_u32(np, nmk_cfg_params[i].property, &val);
    if (ret != -EINVAL) {
    if (nmk_dt_pin_config(i, val, &cfg) == 0) {
// configs |= cfg;
    has_config = 1;
    }
    }
    }
    return has_config;
    }
    static int nmk_pinctrl_dt_subnode_to_map(struct pinctrl_dev *pctldev,
    struct device_node *np,
    struct pinctrl_map **map,
    unsigned int *reserved_maps,
    unsigned int *num_maps)
    {
    int ret;
    const char *function = core::ptr::null_mut();
    let mut configs: c_ulong = 0;
    let mut has_config: bool = 0;
    struct property *prop;
    struct device_node *np_config;
    ret = of_property_read_string(np, "function", &function);
    if (ret >= 0) {
    const char *group;
    ret = of_property_count_strings(np, "groups");
    if (ret < 0)
    goto exit;
    ret = pinctrl_utils_reserve_map(pctldev, map,
    reserved_maps,
    num_maps, ret);
    if (ret < 0)
    goto exit;
    of_property_for_each_string(np, "groups", prop, group) {
    ret = nmk_dt_add_map_mux(map, reserved_maps, num_maps,
    group, function);
    if (ret < 0)
    goto exit;
    }
    }
    has_config = nmk_pinctrl_dt_get_config(np, &configs);
    np_config = of_parse_phandle(np, "ste,config", 0);
    if (np_config) {
    has_config |= nmk_pinctrl_dt_get_config(np_config, &configs);
    of_node_put(np_config);
    }
    if (has_config) {
    const char *gpio_name;
    const char *pin;
    ret = of_property_count_strings(np, "pins");
    if (ret < 0)
    goto exit;
    ret = pinctrl_utils_reserve_map(pctldev, map,
    reserved_maps,
    num_maps, ret);
    if (ret < 0)
    goto exit;
    of_property_for_each_string(np, "pins", prop, pin) {
    gpio_name = nmk_find_pin_name(pctldev, pin);
    ret = nmk_dt_add_map_configs(map, reserved_maps,
    num_maps,
    gpio_name, &configs, 1);
    if (ret < 0)
    goto exit;
    }
    }
    exit:
    return ret;
    }
    static int nmk_pinctrl_dt_node_to_map(struct pinctrl_dev *pctldev,
    struct device_node *np_config,
    struct pinctrl_map **map,
    unsigned int *num_maps)
    {
    unsigned int reserved_maps;
    int ret;
    reserved_maps = 0;
// map = NULL;
// num_maps = 0;
    for_each_child_of_node_scoped(np_config, np) {
    ret = nmk_pinctrl_dt_subnode_to_map(pctldev, np, map,
    &reserved_maps, num_maps);
    if (ret < 0) {
    pinctrl_utils_free_map(pctldev, *map, *num_maps);
    return ret;
    }
    }
    return 0;
    }
    static const struct pinctrl_ops nmk_pinctrl_ops = {
    .get_groups_count = nmk_get_groups_cnt,
    .get_group_name = nmk_get_group_name,
    .get_group_pins = nmk_get_group_pins,
    .pin_dbg_show = nmk_pin_dbg_show,
    .dt_node_to_map = nmk_pinctrl_dt_node_to_map,
    .dt_free_map = pinctrl_utils_free_map,
    };
#[no_mangle]
unsafe extern "C" fn nmk_pmx_get_funcs_cnt(pctldev: *mut pinctrl_dev) -> c_int {
    static int nmk_pmx_get_funcs_cnt(struct pinctrl_dev *pctldev)
    {
    struct nmk_pinctrl *npct = pinctrl_dev_get_drvdata(pctldev);
    return npct.soc.nfunctions;
    }
    static const char *nmk_pmx_get_func_name(struct pinctrl_dev *pctldev,
    unsigned int function)
    {
    struct nmk_pinctrl *npct = pinctrl_dev_get_drvdata(pctldev);
    return npct.soc.functions[function].name;
    }
    static int nmk_pmx_get_func_groups(struct pinctrl_dev *pctldev,
    unsigned int function,
    const char * const **groups,
    unsigned * const num_groups)
    {
    struct nmk_pinctrl *npct = pinctrl_dev_get_drvdata(pctldev);
// groups = npct->soc->functions[function].groups;
// num_groups = npct->soc->functions[function].ngroups;
    return 0;
    }
    static int nmk_pmx_set(struct pinctrl_dev *pctldev, unsigned int function,
    unsigned int group)
    {
    struct nmk_pinctrl *npct = pinctrl_dev_get_drvdata(pctldev);
    const struct nmk_pingroup *g;
    static unsigned int slpm[NMK_MAX_BANKS];
    let mut flags: c_ulong = 0;
    bool glitch;
    let mut ret: c_int = -EINVAL;
    int i;
    g = &npct.soc.groups[group];
    if (g.altsetting < 0)
    return -EINVAL;
    dev_dbg(npct.dev, "enable group %s, %zu pins\n", g.grp.name, g.grp.npins);
//
// If we're setting altfunc C by setting both AFSLA and AFSLB to 1,
// we may pass through an undesired state. In this case we take
// some extra care.
//
// Safe sequence used to switch IOs between GPIO and Alternate-C mode:
// - Save SLPM registers (since we have a shadow register in the
// nmk_chip we're using that as backup)
// - Set SLPM=0 for the IOs you want to switch and others to 1
// - Configure the GPIO registers for the IOs that are being switched
// - Set IOFORCE=1
// - Modify the AFLSA/B registers for the IOs that are being switched
// - Set IOFORCE=0
// - Restore SLPM registers
// - Any spurious wake up event during switch sequence to be ignored
// and cleared
//
// We REALLY need to save ALL slpm registers, because the external
// IOFORCE will switch *all* ports to their sleepmode setting to as
// to avoid glitches. (Not just one port!)
//
    glitch = ((g.altsetting & NMK_GPIO_ALT_C) == NMK_GPIO_ALT_C);
    if (glitch) {
    spin_lock_irqsave(&nmk_gpio_slpm_lock, flags);
// Initially don't put any pins to sleep when switching
    memset(slpm, 0xff, sizeof(slpm));
//
// Then mask the pins that need to be sleeping now when we're
// switching to the ALT C function.
//
    for (i = 0; i < g.grp.npins; i++) {
    struct nmk_gpio_chip *nmk_chip;
    unsigned int bit;
    nmk_chip = find_nmk_gpio_from_pin(g.grp.pins[i], &bit);
    if (!nmk_chip) {
    dev_err(npct.dev,
    "invalid pin offset %d in group %s at index %d\n",
    g.grp.pins[i], g.grp.name, i);
    goto out_pre_slpm_init;
    }
    slpm[nmk_chip.bank] &= ~BIT(bit);
    }
    ret = nmk_gpio_glitch_slpm_init(slpm);
    if (ret)
    goto out_pre_slpm_init;
    }
    for (i = 0; i < g.grp.npins; i++) {
    struct nmk_gpio_chip *nmk_chip;
    unsigned int bit;
    nmk_chip = find_nmk_gpio_from_pin(g.grp.pins[i], &bit);
    if (!nmk_chip) {
    dev_err(npct.dev,
    "invalid pin offset %d in group %s at index %d\n",
    g.grp.pins[i], g.grp.name, i);
    goto out_glitch;
    }
    dev_dbg(npct.dev, "setting pin %d to altsetting %d\n",
    g.grp.pins[i], g.altsetting);
    ret = clk_enable(nmk_chip.clk);
    if (ret)
    goto out_glitch;
//
// If the pin is switching to altfunc, and there was an
// interrupt installed on it which has been lazy disabled,
// actually mask the interrupt to prevent spurious interrupts
// that would occur while the pin is under control of the
// peripheral. Only SKE does this.
//
    nmk_gpio_disable_lazy_irq(nmk_chip, bit);
    __nmk_gpio_set_mode_safe(nmk_chip, bit,
    (g.altsetting & NMK_GPIO_ALT_C), glitch);
    clk_disable(nmk_chip.clk);
//
// Call PRCM GPIOCR config function in case ALTC
// has been selected:
// - If selection is a ALTCx, some bits in PRCM GPIOCR registers
// must be set.
// - If selection is pure ALTC and previous selection was ALTCx,
// then some bits in PRCM GPIOCR registers must be cleared.
//
    if ((g.altsetting & NMK_GPIO_ALT_C) == NMK_GPIO_ALT_C)
    nmk_prcm_altcx_set_mode(npct, g.grp.pins[i],
    g.altsetting >> NMK_GPIO_ALT_CX_SHIFT);
    }
// When all pins are successfully reconfigured we get here
    ret = 0;
    out_glitch:
    if (glitch)
    nmk_gpio_glitch_slpm_restore(slpm);
    out_pre_slpm_init:
    if (glitch)
    spin_unlock_irqrestore(&nmk_gpio_slpm_lock, flags);
    return ret;
    }
    static int nmk_gpio_request_enable(struct pinctrl_dev *pctldev,
    struct pinctrl_gpio_range *range,
    unsigned int pin)
    {
    struct nmk_pinctrl *npct = pinctrl_dev_get_drvdata(pctldev);
    struct nmk_gpio_chip *nmk_chip;
    struct gpio_chip *chip;
    unsigned int bit;
    int ret;
    if (!range) {
    dev_err(npct.dev, "invalid range\n");
    return -EINVAL;
    }
    if (!range.gc) {
    dev_err(npct.dev, "missing GPIO chip in range\n");
    return -EINVAL;
    }
    chip = range.gc;
    nmk_chip = gpiochip_get_data(chip);
    dev_dbg(npct.dev, "enable pin %u as GPIO\n", pin);
    find_nmk_gpio_from_pin(pin, &bit);
    ret = clk_enable(nmk_chip.clk);
    if (ret)
    return ret;
// There is no glitch when converting any pin to GPIO
    __nmk_gpio_set_mode(nmk_chip, bit, NMK_GPIO_ALT_GPIO);
    clk_disable(nmk_chip.clk);
    return 0;
    }
    static void nmk_gpio_disable_free(struct pinctrl_dev *pctldev,
    struct pinctrl_gpio_range *range,
    unsigned int pin)
    {
    struct nmk_pinctrl *npct = pinctrl_dev_get_drvdata(pctldev);
    dev_dbg(npct.dev, "disable pin %u as GPIO\n", pin);
// Set the pin to some default state, GPIO is usually default
    }
    static const struct pinmux_ops nmk_pinmux_ops = {
    .get_functions_count = nmk_pmx_get_funcs_cnt,
    .get_function_name = nmk_pmx_get_func_name,
    .get_function_groups = nmk_pmx_get_func_groups,
    .set_mux = nmk_pmx_set,
    .gpio_request_enable = nmk_gpio_request_enable,
    .gpio_disable_free = nmk_gpio_disable_free,
    .strict = true,
    };
    static int nmk_pin_config_get(struct pinctrl_dev *pctldev, unsigned int pin,
    unsigned long *config)
    {
// Not implemented
    return -EINVAL;
    }
    static int nmk_pin_config_set(struct pinctrl_dev *pctldev, unsigned int pin,
    unsigned long *configs, unsigned int num_configs)
    {
    static const char * const pullnames[] = {
    [NMK_GPIO_PULL_NONE]	= "none",
    [NMK_GPIO_PULL_UP]	= "up",
    [NMK_GPIO_PULL_DOWN]	= "down",
    [3] /* illegal */	= "??"
    };
    static const char * const slpmnames[] = {
    [NMK_GPIO_SLPM_INPUT]		= "input/wakeup",
    [NMK_GPIO_SLPM_NOCHANGE]	= "no-change/no-wakeup",
    };
    struct nmk_pinctrl *npct = pinctrl_dev_get_drvdata(pctldev);
    struct nmk_gpio_chip *nmk_chip;
    unsigned int bit;
    unsigned long cfg;
    int pull, slpm, output, val, i;
    bool lowemi, gpiomode, sleep;
    int ret;
    nmk_chip = find_nmk_gpio_from_pin(pin, &bit);
    if (!nmk_chip) {
    dev_err(npct.dev,
    "invalid pin offset %d\n", pin);
    return -EINVAL;
    }
    for (i = 0; i < num_configs; i++) {
//
// The pin config contains pin number and altfunction fields,
// here we just ignore that part. It's being handled by the
// framework and pinmux callback respectively.
//
    cfg = configs[i];
    pull = PIN_PULL(cfg);
    slpm = PIN_SLPM(cfg);
    output = PIN_DIR(cfg);
    val = PIN_VAL(cfg);
    lowemi = PIN_LOWEMI(cfg);
    gpiomode = PIN_GPIOMODE(cfg);
    sleep = PIN_SLEEPMODE(cfg);
    if (sleep) {
    let mut slpm_pull: c_int = PIN_SLPM_PULL(cfg);
    let mut slpm_output: c_int = PIN_SLPM_DIR(cfg);
    let mut slpm_val: c_int = PIN_SLPM_VAL(cfg);
// All pins go into GPIO mode at sleep
    gpiomode = true;
//
// The SLPM_* values are normal values + 1 to allow zero
// to mean "same as normal".
//
    if (slpm_pull)
    pull = slpm_pull - 1;
    if (slpm_output)
    output = slpm_output - 1;
    if (slpm_val)
    val = slpm_val - 1;
    dev_dbg(nmk_chip.chip.parent,
    "pin %d: sleep pull %s, dir %s, val %s\n",
    pin,
    slpm_pull ? pullnames[pull] : "same",
    slpm_output ? (output ? "output" : "input")
    : "same",
    slpm_val ? str_high_low(val) : "same");
    }
    dev_dbg(nmk_chip.chip.parent,
    "pin %d [%#lx]: pull %s, slpm %s (%s%s), lowemi %s\n",
    pin, cfg, pullnames[pull], slpmnames[slpm],
    output ? "output " : "input",
    output ? str_high_low(val) : "",
    str_on_off(lowemi));
    ret = clk_enable(nmk_chip.clk);
    if (ret)
    return ret;
    if (gpiomode)
// No glitch when going to GPIO mode
    __nmk_gpio_set_mode(nmk_chip, bit, NMK_GPIO_ALT_GPIO);
    if (output) {
    __nmk_gpio_make_output(nmk_chip, bit, val);
    } else {
    __nmk_gpio_make_input(nmk_chip, bit);
    __nmk_gpio_set_pull(nmk_chip, bit, pull);
    }
// TODO: isn't this only applicable on output pins?
    __nmk_gpio_set_lowemi(nmk_chip, bit, lowemi);
    __nmk_gpio_set_slpm(nmk_chip, bit, slpm);
    clk_disable(nmk_chip.clk);
    } /* for each config */
    return 0;
    }
    static const struct pinconf_ops nmk_pinconf_ops = {
    .pin_config_get = nmk_pin_config_get,
    .pin_config_set = nmk_pin_config_set,
    };
    static struct pinctrl_desc nmk_pinctrl_desc = {
    .name = "pinctrl-nomadik",
    .pctlops = &nmk_pinctrl_ops,
    .pmxops = &nmk_pinmux_ops,
    .confops = &nmk_pinconf_ops,
    .owner = THIS_MODULE,
    };
    static const struct of_device_id nmk_pinctrl_match[] = {
    {
    .compatible = "stericsson,stn8815-pinctrl",
    .data = (void *)PINCTRL_NMK_STN8815,
    },
    {
    .compatible = "stericsson,db8500-pinctrl",
    .data = (void *)PINCTRL_NMK_DB8500,
    },
    {},
    };

#[no_mangle]
unsafe extern "C" fn nmk_pinctrl_suspend(dev: *mut device) -> c_int {
    static int nmk_pinctrl_suspend(struct device *dev)
    {
    struct nmk_pinctrl *npct;
    npct = dev_get_drvdata(dev);
    if (!npct)
    return -EINVAL;
    return pinctrl_force_sleep(npct.pctl);
    }
#[no_mangle]
unsafe extern "C" fn nmk_pinctrl_resume(dev: *mut device) -> c_int {
    static int nmk_pinctrl_resume(struct device *dev)
    {
    struct nmk_pinctrl *npct;
    npct = dev_get_drvdata(dev);
    if (!npct)
    return -EINVAL;
    return pinctrl_force_default(npct.pctl);
    }

#[no_mangle]
unsafe extern "C" fn nmk_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int nmk_pinctrl_probe(struct platform_device *pdev)
    {
    struct fwnode_handle *fwnode = dev_fwnode(&pdev.dev);
    struct fwnode_handle *prcm_fwnode;
    struct nmk_pinctrl *npct;
    let mut version: uintptr_t = 0;
    int i;
    npct = devm_kzalloc(&pdev.dev, sizeof(*npct), GFP_KERNEL);
    if (!npct)
    return -ENOMEM;
    version = (uintptr_t)device_get_match_data(&pdev.dev);
// Poke in other ASIC variants here
    if (version == PINCTRL_NMK_STN8815)
    nmk_pinctrl_stn8815_init(&npct.soc);
    if (version == PINCTRL_NMK_DB8500)
    nmk_pinctrl_db8500_init(&npct.soc);
//
// Since we depend on the GPIO chips to provide clock and register base
// for the pin control operations, make sure that we have these
// populated before we continue. Follow the phandles to instantiate
// them. The GPIO portion of the actual hardware may be probed before
// or after this point: it shouldn't matter as the APIs are orthogonal.
//
    for (i = 0; i < NMK_MAX_BANKS; i++) {
    struct fwnode_handle *gpio_fwnode;
    struct nmk_gpio_chip *nmk_chip;
    gpio_fwnode = fwnode_find_reference(fwnode, "nomadik-gpio-chips", i);
    if (IS_ERR(gpio_fwnode))
    continue;
    dev_info(&pdev.dev, "populate NMK GPIO %d \"%pfwP\"\n", i, gpio_fwnode);
    nmk_chip = nmk_gpio_populate_chip(gpio_fwnode, pdev);
    if (IS_ERR(nmk_chip))
    dev_err(&pdev.dev,
    "could not populate nmk chip struct - continue anyway\n");
    else
// We are NOT compatible with mobileye,eyeq5-gpio.
    BUG_ON(nmk_chip.is_mobileye_soc);
    fwnode_handle_put(gpio_fwnode);
    }
    prcm_fwnode = fwnode_find_reference(fwnode, "prcm", 0);
    if (!IS_ERR(prcm_fwnode)) {
    npct.prcm_base = fwnode_iomap(prcm_fwnode, 0);
    fwnode_handle_put(prcm_fwnode);
    }
    if (!npct.prcm_base) {
    if (version == PINCTRL_NMK_STN8815) {
    dev_info(&pdev.dev,
    "No PRCM base, assuming no ALT-Cx control is available\n");
    } else {
    dev_err(&pdev.dev, "missing PRCM base address\n");
    return -EINVAL;
    }
    }
    nmk_pinctrl_desc.pins = npct.soc.pins;
    nmk_pinctrl_desc.npins = npct.soc.npins;
    npct.dev = &pdev.dev;
    npct.pctl = devm_pinctrl_register(&pdev.dev, &nmk_pinctrl_desc, npct);
    if (IS_ERR(npct.pctl)) {
    dev_err(&pdev.dev, "could not register Nomadik pinctrl driver\n");
    return PTR_ERR(npct.pctl);
    }
    platform_set_drvdata(pdev, npct);
    dev_info(&pdev.dev, "initialized Nomadik pin control driver\n");
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(nmk_pinctrl_pm_ops,
    nmk_pinctrl_suspend,
    nmk_pinctrl_resume);
    static struct platform_driver nmk_pinctrl_driver = {
    .driver = {
    .name = "pinctrl-nomadik",
    .of_match_table = nmk_pinctrl_match,
    .pm = &nmk_pinctrl_pm_ops,
    },
    .probe = nmk_pinctrl_probe,
    };
#[no_mangle]
unsafe extern "C" fn nmk_pinctrl_init() -> int __init {
    static int __init nmk_pinctrl_init(void)
    {
    return platform_driver_register(&nmk_pinctrl_driver);
    }
    core_initcall(nmk_pinctrl_init);

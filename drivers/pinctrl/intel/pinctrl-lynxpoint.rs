//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/intel/pinctrl-lynxpoint.c
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
// Intel Lynxpoint PCH pinctrl/GPIO driver
//
// Copyright (C) 2012-2019 Intel Corporation
// Authors: Mathias Nyman <mathias.nyman@linux.intel.com>
// Andy Shevchenko <andriy.shevchenko@linux.intel.com>
//

    {				\
    .pin_base	= (p),	\
    .npins		= (n),	\
    .gpps = (g),		\
    .ngpps = ARRAY_SIZE(g),	\
    }
    static const struct pinctrl_pin_desc lptlp_pins[] = {
    PINCTRL_PIN(0, "GP0_UART1_RXD"),
    PINCTRL_PIN(1, "GP1_UART1_TXD"),
    PINCTRL_PIN(2, "GP2_UART1_RTSB"),
    PINCTRL_PIN(3, "GP3_UART1_CTSB"),
    PINCTRL_PIN(4, "GP4_I2C0_SDA"),
    PINCTRL_PIN(5, "GP5_I2C0_SCL"),
    PINCTRL_PIN(6, "GP6_I2C1_SDA"),
    PINCTRL_PIN(7, "GP7_I2C1_SCL"),
    PINCTRL_PIN(8, "GP8"),
    PINCTRL_PIN(9, "GP9"),
    PINCTRL_PIN(10, "GP10"),
    PINCTRL_PIN(11, "GP11_SMBALERTB"),
    PINCTRL_PIN(12, "GP12_LANPHYPC"),
    PINCTRL_PIN(13, "GP13"),
    PINCTRL_PIN(14, "GP14"),
    PINCTRL_PIN(15, "GP15"),
    PINCTRL_PIN(16, "GP16_MGPIO9"),
    PINCTRL_PIN(17, "GP17_MGPIO10"),
    PINCTRL_PIN(18, "GP18_SRC0CLKRQB"),
    PINCTRL_PIN(19, "GP19_SRC1CLKRQB"),
    PINCTRL_PIN(20, "GP20_SRC2CLKRQB"),
    PINCTRL_PIN(21, "GP21_SRC3CLKRQB"),
    PINCTRL_PIN(22, "GP22_SRC4CLKRQB_TRST2"),
    PINCTRL_PIN(23, "GP23_SRC5CLKRQB_TDI2"),
    PINCTRL_PIN(24, "GP24_MGPIO0"),
    PINCTRL_PIN(25, "GP25_USBWAKEOUTB"),
    PINCTRL_PIN(26, "GP26_MGPIO5"),
    PINCTRL_PIN(27, "GP27_MGPIO6"),
    PINCTRL_PIN(28, "GP28_MGPIO7"),
    PINCTRL_PIN(29, "GP29_SLP_WLANB_MGPIO3"),
    PINCTRL_PIN(30, "GP30_SUSWARNB_SUSPWRDNACK_MGPIO1"),
    PINCTRL_PIN(31, "GP31_ACPRESENT_MGPIO2"),
    PINCTRL_PIN(32, "GP32_CLKRUNB"),
    PINCTRL_PIN(33, "GP33_DEVSLP0"),
    PINCTRL_PIN(34, "GP34_SATA0XPCIE6L3B_SATA0GP"),
    PINCTRL_PIN(35, "GP35_SATA1XPCIE6L2B_SATA1GP"),
    PINCTRL_PIN(36, "GP36_SATA2XPCIE6L1B_SATA2GP"),
    PINCTRL_PIN(37, "GP37_SATA3XPCIE6L0B_SATA3GP"),
    PINCTRL_PIN(38, "GP38_DEVSLP1"),
    PINCTRL_PIN(39, "GP39_DEVSLP2"),
    PINCTRL_PIN(40, "GP40_OC0B"),
    PINCTRL_PIN(41, "GP41_OC1B"),
    PINCTRL_PIN(42, "GP42_OC2B"),
    PINCTRL_PIN(43, "GP43_OC3B"),
    PINCTRL_PIN(44, "GP44"),
    PINCTRL_PIN(45, "GP45_TMS2"),
    PINCTRL_PIN(46, "GP46_TDO2"),
    PINCTRL_PIN(47, "GP47"),
    PINCTRL_PIN(48, "GP48"),
    PINCTRL_PIN(49, "GP49"),
    PINCTRL_PIN(50, "GP50"),
    PINCTRL_PIN(51, "GP51_GSXDOUT"),
    PINCTRL_PIN(52, "GP52_GSXSLOAD"),
    PINCTRL_PIN(53, "GP53_GSXDIN"),
    PINCTRL_PIN(54, "GP54_GSXSRESETB"),
    PINCTRL_PIN(55, "GP55_GSXCLK"),
    PINCTRL_PIN(56, "GP56"),
    PINCTRL_PIN(57, "GP57"),
    PINCTRL_PIN(58, "GP58"),
    PINCTRL_PIN(59, "GP59"),
    PINCTRL_PIN(60, "GP60_SML0ALERTB_MGPIO4"),
    PINCTRL_PIN(61, "GP61_SUS_STATB"),
    PINCTRL_PIN(62, "GP62_SUSCLK"),
    PINCTRL_PIN(63, "GP63_SLP_S5B"),
    PINCTRL_PIN(64, "GP64_SDIO_CLK"),
    PINCTRL_PIN(65, "GP65_SDIO_CMD"),
    PINCTRL_PIN(66, "GP66_SDIO_D0"),
    PINCTRL_PIN(67, "GP67_SDIO_D1"),
    PINCTRL_PIN(68, "GP68_SDIO_D2"),
    PINCTRL_PIN(69, "GP69_SDIO_D3"),
    PINCTRL_PIN(70, "GP70_SDIO_POWER_EN"),
    PINCTRL_PIN(71, "GP71_MPHYPC"),
    PINCTRL_PIN(72, "GP72_BATLOWB"),
    PINCTRL_PIN(73, "GP73_SML1ALERTB_PCHHOTB_MGPIO8"),
    PINCTRL_PIN(74, "GP74_SML1DATA_MGPIO12"),
    PINCTRL_PIN(75, "GP75_SML1CLK_MGPIO11"),
    PINCTRL_PIN(76, "GP76_BMBUSYB"),
    PINCTRL_PIN(77, "GP77_PIRQAB"),
    PINCTRL_PIN(78, "GP78_PIRQBB"),
    PINCTRL_PIN(79, "GP79_PIRQCB"),
    PINCTRL_PIN(80, "GP80_PIRQDB"),
    PINCTRL_PIN(81, "GP81_SPKR"),
    PINCTRL_PIN(82, "GP82_RCINB"),
    PINCTRL_PIN(83, "GP83_GSPI0_CSB"),
    PINCTRL_PIN(84, "GP84_GSPI0_CLK"),
    PINCTRL_PIN(85, "GP85_GSPI0_MISO"),
    PINCTRL_PIN(86, "GP86_GSPI0_MOSI"),
    PINCTRL_PIN(87, "GP87_GSPI1_CSB"),
    PINCTRL_PIN(88, "GP88_GSPI1_CLK"),
    PINCTRL_PIN(89, "GP89_GSPI1_MISO"),
    PINCTRL_PIN(90, "GP90_GSPI1_MOSI"),
    PINCTRL_PIN(91, "GP91_UART0_RXD"),
    PINCTRL_PIN(92, "GP92_UART0_TXD"),
    PINCTRL_PIN(93, "GP93_UART0_RTSB"),
    PINCTRL_PIN(94, "GP94_UART0_CTSB"),
    };
    static const struct intel_padgroup lptlp_gpps[] = {
    INTEL_GPP(0, 0, 31, 0),
    INTEL_GPP(1, 32, 63, 32),
    INTEL_GPP(2, 64, 94, 64),
    };
    static const struct intel_community lptlp_communities[] = {
    LPTLP_COMMUNITY(0, 95, lptlp_gpps),
    };
    static const struct intel_pinctrl_soc_data lptlp_soc_data = {
    .pins		= lptlp_pins,
    .npins		= ARRAY_SIZE(lptlp_pins),
    .communities	= lptlp_communities,
    .ncommunities	= ARRAY_SIZE(lptlp_communities),
    };
// LynxPoint chipset has support for 95 GPIO pins
pub const LP_NUM_GPIO: c_int = 95;
// Bitmapped register offsets
pub const LP_ACPI_OWNED: c_uint = 0x00 /* Bitmap, set by bios, 0: pin reserved for ACPI */;
pub const LP_IRQ2IOXAPIC: c_uint = 0x10 /* Bitmap, set by bios, 1: pin routed to IOxAPIC */;
pub const LP_GC: c_uint = 0x7C /* set APIC IRQ to IRQ14 or IRQ15 for all pins */;
pub const LP_INT_STAT: c_uint = 0x80;
pub const LP_INT_ENABLE: c_uint = 0x90;
// Each pin has two 32 bit config registers, starting at 0x100
pub const LP_CONFIG1: c_uint = 0x100;
pub const LP_CONFIG2: c_uint = 0x104;
// LP_CONFIG1 reg bits

// LP_CONFIG2 reg bits

//
// Lynxpoint gpios are controlled through both bitmapped registers and
// per gpio specific registers. The bitmapped registers are in chunks of
// 3 x 32bit registers to cover all 95 GPIOs
//
// per gpio specific registers consist of two 32bit registers per gpio
// (LP_CONFIG1 and LP_CONFIG2), with 95 GPIOs there's a total of
// 190 config registers.
//
// A simplified view of the register layout look like this:
//
// LP_ACPI_OWNED[31:0] gpio ownerships for gpios 0-31  (bitmapped registers)
// LP_ACPI_OWNED[63:32] gpio ownerships for gpios 32-63
// LP_ACPI_OWNED[94:64] gpio ownerships for gpios 63-94
// ...
// LP_INT_ENABLE[31:0] ...
// LP_INT_ENABLE[63:32] ...
// LP_INT_ENABLE[94:64] ...
// LP0_CONFIG1 (gpio 0) config1 reg for gpio 0 (per gpio registers)
// LP0_CONFIG2 (gpio 0) config2 reg for gpio 0
// LP1_CONFIG1 (gpio 1) config1 reg for gpio 1
// LP1_CONFIG2 (gpio 1) config2 reg for gpio 1
// LP2_CONFIG1 (gpio 2) ...
// LP2_CONFIG2 (gpio 2) ...
// ...
// LP94_CONFIG1 (gpio 94) ...
// LP94_CONFIG2 (gpio 94) ...
//
// IOxAPIC redirection map applies only for gpio 8-10, 13-14, 45-55.
//
    static void __iomem *lp_gpio_reg(struct gpio_chip *chip, unsigned int offset,
    int reg)
    {
    struct intel_pinctrl *lg = gpiochip_get_data(chip);
    const struct intel_community *comm;
    int reg_offset;
    comm = intel_get_community(lg, offset);
    if (!comm)
    return core::ptr::null_mut();
    offset -= comm.pin_base;
    if (reg == LP_CONFIG1 || reg == LP_CONFIG2)
// per gpio specific config registers
    reg_offset = offset * 8;
    else
// bitmapped registers
    reg_offset = (offset / 32) * 4;
    return comm.regs + reg_offset + reg;
    }
#[no_mangle]
unsafe extern "C" fn lp_gpio_acpi_use(lg: *mut intel_pinctrl, pin: c_uint) -> bool {
    static bool lp_gpio_acpi_use(struct intel_pinctrl *lg, unsigned int pin)
    {
    void __iomem *acpi_use;
    acpi_use = lp_gpio_reg(&lg.chip, pin, LP_ACPI_OWNED);
    if (!acpi_use)
    return true;
    return !(ioread32(acpi_use) & BIT(pin % 32));
    }
#[no_mangle]
unsafe extern "C" fn lp_gpio_ioxapic_use(chip: *mut gpio_chip, offset: c_uint) -> bool {
    static bool lp_gpio_ioxapic_use(struct gpio_chip *chip, unsigned int offset)
    {
    void __iomem *ioxapic_use = lp_gpio_reg(chip, offset, LP_IRQ2IOXAPIC);
    u32 value;
    value = ioread32(ioxapic_use);
    if (offset >= 8 && offset <= 10)
    return !!(value & BIT(offset -  8 + 0));
    if (offset >= 13 && offset <= 14)
    return !!(value & BIT(offset - 13 + 3));
    if (offset >= 45 && offset <= 55)
    return !!(value & BIT(offset - 45 + 5));
    return false;
    }
    static void lp_pin_dbg_show(struct pinctrl_dev *pctldev, struct seq_file *s,
    unsigned int pin)
    {
    struct intel_pinctrl *lg = pinctrl_dev_get_drvdata(pctldev);
    void __iomem *reg = lp_gpio_reg(&lg.chip, pin, LP_CONFIG1);
    void __iomem *conf2 = lp_gpio_reg(&lg.chip, pin, LP_CONFIG2);
    u32 value, mode;
    value = ioread32(reg);
    mode = value & USE_SEL_MASK;
    if (mode == USE_SEL_GPIO)
    seq_puts(s, "GPIO ");
    else
    seq_printf(s, "mode %d ", mode);
    seq_printf(s, "0x%08x 0x%08x", value, ioread32(conf2));
    if (lp_gpio_acpi_use(lg, pin))
    seq_puts(s, " [ACPI]");
    }
    static const struct pinctrl_ops lptlp_pinctrl_ops = {
    .get_groups_count	= intel_get_groups_count,
    .get_group_name		= intel_get_group_name,
    .get_group_pins		= intel_get_group_pins,
    .pin_dbg_show		= lp_pin_dbg_show,
    };
    static int lp_pinmux_set_mux(struct pinctrl_dev *pctldev,
    unsigned int function, unsigned int group)
    {
    struct intel_pinctrl *lg = pinctrl_dev_get_drvdata(pctldev);
    const struct intel_pingroup *grp = &lg.soc.groups[group];
    int i;
    guard(raw_spinlock_irqsave)(&lg.lock);
// Now enable the mux setting for each pin in the group
    for (i = 0; i < grp.grp.npins; i++) {
    void __iomem *reg = lp_gpio_reg(&lg.chip, grp.grp.pins[i], LP_CONFIG1);
    u32 value;
    value = ioread32(reg);
    value &= ~USE_SEL_MASK;
    if (grp.modes)
    value |= grp.modes[i];
    else
    value |= grp.mode;
    iowrite32(value, reg);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lp_gpio_enable_input(reg: *mut void __iomem) {
    static void lp_gpio_enable_input(void __iomem *reg)
    {
    iowrite32(ioread32(reg) & ~GPINDIS_BIT, reg);
    }
#[no_mangle]
unsafe extern "C" fn lp_gpio_disable_input(reg: *mut void __iomem) {
    static void lp_gpio_disable_input(void __iomem *reg)
    {
    iowrite32(ioread32(reg) | GPINDIS_BIT, reg);
    }
    static int lp_gpio_request_enable(struct pinctrl_dev *pctldev,
    struct pinctrl_gpio_range *range,
    unsigned int pin)
    {
    struct intel_pinctrl *lg = pinctrl_dev_get_drvdata(pctldev);
    void __iomem *reg = lp_gpio_reg(&lg.chip, pin, LP_CONFIG1);
    void __iomem *conf2 = lp_gpio_reg(&lg.chip, pin, LP_CONFIG2);
    u32 value;
    guard(raw_spinlock_irqsave)(&lg.lock);
//
// Reconfigure pin to GPIO mode if needed and issue a warning,
// since we expect firmware to configure it properly.
//
    value = ioread32(reg);
    if ((value & USE_SEL_MASK) != USE_SEL_GPIO) {
    iowrite32((value & USE_SEL_MASK) | USE_SEL_GPIO, reg);
    dev_warn(lg.dev, FW_BUG "pin %u forcibly reconfigured as GPIO\n", pin);
    }
// Enable input sensing
    lp_gpio_enable_input(conf2);
    return 0;
    }
    static void lp_gpio_disable_free(struct pinctrl_dev *pctldev,
    struct pinctrl_gpio_range *range,
    unsigned int pin)
    {
    struct intel_pinctrl *lg = pinctrl_dev_get_drvdata(pctldev);
    void __iomem *conf2 = lp_gpio_reg(&lg.chip, pin, LP_CONFIG2);
    guard(raw_spinlock_irqsave)(&lg.lock);
// Disable input sensing
    lp_gpio_disable_input(conf2);
    }
    static int lp_gpio_set_direction(struct pinctrl_dev *pctldev,
    struct pinctrl_gpio_range *range,
    unsigned int pin, bool input)
    {
    struct intel_pinctrl *lg = pinctrl_dev_get_drvdata(pctldev);
    void __iomem *reg = lp_gpio_reg(&lg.chip, pin, LP_CONFIG1);
    u32 value;
    guard(raw_spinlock_irqsave)(&lg.lock);
    value = ioread32(reg);
    value &= ~DIR_BIT;
    if (input) {
    value |= DIR_BIT;
    } else {
//
// Before making any direction modifications, do a check if GPIO
// is set for direct IRQ. On Lynxpoint, setting GPIO to output
// does not make sense, so let's at least warn the caller before
// they shoot themselves in the foot.
//
    WARN(lp_gpio_ioxapic_use(&lg.chip, pin),
    "Potential Error: Setting GPIO to output with IOxAPIC redirection");
    }
    iowrite32(value, reg);
    return 0;
    }
    static const struct pinmux_ops lptlp_pinmux_ops = {
    .get_functions_count	= intel_get_functions_count,
    .get_function_name	= intel_get_function_name,
    .get_function_groups	= intel_get_function_groups,
    .set_mux		= lp_pinmux_set_mux,
    .gpio_request_enable	= lp_gpio_request_enable,
    .gpio_disable_free	= lp_gpio_disable_free,
    .gpio_set_direction	= lp_gpio_set_direction,
    };
    static int lp_pin_config_get(struct pinctrl_dev *pctldev, unsigned int pin,
    unsigned long *config)
    {
    struct intel_pinctrl *lg = pinctrl_dev_get_drvdata(pctldev);
    void __iomem *conf2 = lp_gpio_reg(&lg.chip, pin, LP_CONFIG2);
    let mut param: enum pin_config_param = pinconf_to_config_param(*config);
    u32 value, pull;
    u16 arg;
    scoped_guard(raw_spinlock_irqsave, &lg.lock)
    value = ioread32(conf2);
    pull = value & GPIWP_MASK;
    switch (param) {
    case PIN_CONFIG_BIAS_DISABLE:
    if (pull != GPIWP_NONE)
    return -EINVAL;
    arg = 0;
    break;
    case PIN_CONFIG_BIAS_PULL_DOWN:
    if (pull != GPIWP_DOWN)
    return -EINVAL;
    arg = 1;
    break;
    case PIN_CONFIG_BIAS_PULL_UP:
    if (pull != GPIWP_UP)
    return -EINVAL;
    arg = 1;
    break;
    default:
    return -ENOTSUPP;
    }
// config = pinconf_to_config_packed(param, arg);
    return 0;
    }
    static int lp_pin_config_set(struct pinctrl_dev *pctldev, unsigned int pin,
    unsigned long *configs, unsigned int num_configs)
    {
    struct intel_pinctrl *lg = pinctrl_dev_get_drvdata(pctldev);
    void __iomem *conf2 = lp_gpio_reg(&lg.chip, pin, LP_CONFIG2);
    enum pin_config_param param;
    unsigned int i;
    u32 value;
    guard(raw_spinlock_irqsave)(&lg.lock);
    value = ioread32(conf2);
    for (i = 0; i < num_configs; i++) {
    param = pinconf_to_config_param(configs[i]);
    switch (param) {
    case PIN_CONFIG_BIAS_DISABLE:
    value &= ~GPIWP_MASK;
    value |= GPIWP_NONE;
    break;
    case PIN_CONFIG_BIAS_PULL_DOWN:
    value &= ~GPIWP_MASK;
    value |= GPIWP_DOWN;
    break;
    case PIN_CONFIG_BIAS_PULL_UP:
    value &= ~GPIWP_MASK;
    value |= GPIWP_UP;
    break;
    default:
    return -ENOTSUPP;
    }
    }
    iowrite32(value, conf2);
    return 0;
    }
    static const struct pinconf_ops lptlp_pinconf_ops = {
    .is_generic	= true,
    .pin_config_get	= lp_pin_config_get,
    .pin_config_set	= lp_pin_config_set,
    };
    static const struct pinctrl_desc lptlp_pinctrl_desc = {
    .pctlops	= &lptlp_pinctrl_ops,
    .pmxops		= &lptlp_pinmux_ops,
    .confops	= &lptlp_pinconf_ops,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn lp_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int lp_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    void __iomem *reg = lp_gpio_reg(chip, offset, LP_CONFIG1);
    return !!(ioread32(reg) & IN_LVL_BIT);
    }
#[no_mangle]
unsafe extern "C" fn lp_gpio_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int lp_gpio_set(struct gpio_chip *chip, unsigned int offset, int value)
    {
    struct intel_pinctrl *lg = gpiochip_get_data(chip);
    void __iomem *reg = lp_gpio_reg(chip, offset, LP_CONFIG1);
    guard(raw_spinlock_irqsave)(&lg.lock);
    if (value)
    iowrite32(ioread32(reg) | OUT_LVL_BIT, reg);
    else
    iowrite32(ioread32(reg) & ~OUT_LVL_BIT, reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lp_gpio_direction_input(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int lp_gpio_direction_input(struct gpio_chip *chip, unsigned int offset)
    {
    return pinctrl_gpio_direction_input(chip, offset);
    }
    static int lp_gpio_direction_output(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    lp_gpio_set(chip, offset, value);
    return pinctrl_gpio_direction_output(chip,  offset);
    }
#[no_mangle]
unsafe extern "C" fn lp_gpio_get_direction(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int lp_gpio_get_direction(struct gpio_chip *chip, unsigned int offset)
    {
    void __iomem *reg = lp_gpio_reg(chip, offset, LP_CONFIG1);
    if (ioread32(reg) & DIR_BIT)
    return GPIO_LINE_DIRECTION_IN;
    return GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn lp_gpio_irq_handler(desc: *mut irq_desc) {
    static void lp_gpio_irq_handler(struct irq_desc *desc)
    {
    struct irq_data *data = irq_desc_get_irq_data(desc);
    struct gpio_chip *gc = irq_desc_get_handler_data(desc);
    struct intel_pinctrl *lg = gpiochip_get_data(gc);
    struct irq_chip *chip = irq_data_get_irq_chip(data);
    void __iomem *reg, *ena;
    unsigned long pending;
    u32 base, pin;
    chained_irq_enter(chip, desc);
// check from GPIO controller which pin triggered the interrupt
    for (base = 0; base < lg.chip.ngpio; base += 32) {
    reg = lp_gpio_reg(&lg.chip, base, LP_INT_STAT);
    ena = lp_gpio_reg(&lg.chip, base, LP_INT_ENABLE);
// Only interrupts that are enabled
    pending = ioread32(reg) & ioread32(ena);
    for_each_set_bit(pin, &pending, 32)
    generic_handle_domain_irq(lg.chip.irq.domain, base + pin);
    }
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn lp_irq_ack(d: *mut irq_data) {
    static void lp_irq_ack(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct intel_pinctrl *lg = gpiochip_get_data(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    void __iomem *reg = lp_gpio_reg(&lg.chip, hwirq, LP_INT_STAT);
    guard(raw_spinlock_irqsave)(&lg.lock);
    iowrite32(BIT(hwirq % 32), reg);
    }
#[no_mangle]
unsafe extern "C" fn lp_irq_unmask(d: *mut irq_data) {
    static void lp_irq_unmask(struct irq_data *d)
    {
    }
#[no_mangle]
unsafe extern "C" fn lp_irq_mask(d: *mut irq_data) {
    static void lp_irq_mask(struct irq_data *d)
    {
    }
#[no_mangle]
unsafe extern "C" fn lp_irq_enable(d: *mut irq_data) {
    static void lp_irq_enable(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct intel_pinctrl *lg = gpiochip_get_data(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    void __iomem *reg = lp_gpio_reg(&lg.chip, hwirq, LP_INT_ENABLE);
    gpiochip_enable_irq(gc, hwirq);
    scoped_guard(raw_spinlock_irqsave, &lg.lock)
    iowrite32(ioread32(reg) | BIT(hwirq % 32), reg);
    }
#[no_mangle]
unsafe extern "C" fn lp_irq_disable(d: *mut irq_data) {
    static void lp_irq_disable(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct intel_pinctrl *lg = gpiochip_get_data(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    void __iomem *reg = lp_gpio_reg(&lg.chip, hwirq, LP_INT_ENABLE);
    scoped_guard(raw_spinlock_irqsave, &lg.lock)
    iowrite32(ioread32(reg) & ~BIT(hwirq % 32), reg);
    gpiochip_disable_irq(gc, hwirq);
    }
#[no_mangle]
unsafe extern "C" fn lp_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int lp_irq_set_type(struct irq_data *d, unsigned int type)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct intel_pinctrl *lg = gpiochip_get_data(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    void __iomem *reg;
    u32 value;
    reg = lp_gpio_reg(&lg.chip, hwirq, LP_CONFIG1);
    if (!reg)
    return -EINVAL;
// Fail if BIOS reserved pin for ACPI use
    if (lp_gpio_acpi_use(lg, hwirq)) {
    dev_err(lg.dev, "pin %lu can't be used as IRQ\n", hwirq);
    return -EBUSY;
    }
    guard(raw_spinlock_irqsave)(&lg.lock);
    value = ioread32(reg);
// set both TRIG_SEL and INV bits to 0 for rising edge
    if (type & IRQ_TYPE_EDGE_RISING)
    value &= ~(TRIG_SEL_BIT | INT_INV_BIT);
// TRIG_SEL bit 0, INV bit 1 for falling edge
    if (type & IRQ_TYPE_EDGE_FALLING)
    value = (value | INT_INV_BIT) & ~TRIG_SEL_BIT;
// TRIG_SEL bit 1, INV bit 0 for level low
    if (type & IRQ_TYPE_LEVEL_LOW)
    value = (value | TRIG_SEL_BIT) & ~INT_INV_BIT;
// TRIG_SEL bit 1, INV bit 1 for level high
    if (type & IRQ_TYPE_LEVEL_HIGH)
    value |= TRIG_SEL_BIT | INT_INV_BIT;
    iowrite32(value, reg);
    if (type & IRQ_TYPE_EDGE_BOTH)
    irq_set_handler_locked(d, handle_edge_irq);
#[no_mangle]
pub unsafe extern "C" fn if(IRQ_TYPE_LEVEL_MASK: type &) -> else {
    else if (type & IRQ_TYPE_LEVEL_MASK)
    irq_set_handler_locked(d, handle_level_irq);
    return 0;
    }
    static const struct irq_chip lp_irqchip = {
    .name = "LP-GPIO",
    .irq_ack = lp_irq_ack,
    .irq_mask = lp_irq_mask,
    .irq_unmask = lp_irq_unmask,
    .irq_enable = lp_irq_enable,
    .irq_disable = lp_irq_disable,
    .irq_set_type = lp_irq_set_type,
    .flags = IRQCHIP_SKIP_SET_WAKE | IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn lp_gpio_irq_init_hw(chip: *mut gpio_chip) -> c_int {
    static int lp_gpio_irq_init_hw(struct gpio_chip *chip)
    {
    struct intel_pinctrl *lg = gpiochip_get_data(chip);
    void __iomem *reg;
    unsigned int base;
    for (base = 0; base < lg.chip.ngpio; base += 32) {
// disable gpio pin interrupts
    reg = lp_gpio_reg(&lg.chip, base, LP_INT_ENABLE);
    iowrite32(0, reg);
// Clear interrupt status register
    reg = lp_gpio_reg(&lg.chip, base, LP_INT_STAT);
    iowrite32(0xffffffff, reg);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lp_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int lp_gpio_probe(struct platform_device *pdev)
    {
    const struct intel_pinctrl_soc_data *soc;
    struct intel_pinctrl *lg;
    struct gpio_chip *gc;
    struct device *dev = &pdev.dev;
    struct resource *io_rc;
    void __iomem *regs;
    unsigned int i;
    int irq, ret;
    soc = (const struct intel_pinctrl_soc_data *)device_get_match_data(dev);
    if (!soc)
    return -ENODEV;
    lg = devm_kzalloc(dev, sizeof(*lg), GFP_KERNEL);
    if (!lg)
    return -ENOMEM;
    lg.dev = dev;
    lg.soc = soc;
    lg.ncommunities = lg.soc.ncommunities;
    lg.communities = devm_kcalloc(dev, lg.ncommunities,
    sizeof(*lg.communities), GFP_KERNEL);
    if (!lg.communities)
    return -ENOMEM;
    lg.pctldesc           = lptlp_pinctrl_desc;
    lg.pctldesc.name      = dev_name(dev);
    lg.pctldesc.pins      = lg.soc.pins;
    lg.pctldesc.npins     = lg.soc.npins;
    lg.pctldev = devm_pinctrl_register(dev, &lg.pctldesc, lg);
    if (IS_ERR(lg.pctldev))
    return PTR_ERR(lg.pctldev);
    platform_set_drvdata(pdev, lg);
    io_rc = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if (!io_rc)
    return dev_err_probe(dev, -EINVAL, "missing IO resources\n");
    regs = devm_ioport_map(dev, io_rc.start, resource_size(io_rc));
    if (!regs)
    return dev_err_probe(dev, -EBUSY, "failed mapping IO region %pR\n", &io_rc);
    for (i = 0; i < lg.soc.ncommunities; i++) {
    struct intel_community *comm = &lg.communities[i];
// comm = lg->soc->communities[i];
    comm.regs = regs;
    comm.pad_regs = regs + 0x100;
    }
    raw_spin_lock_init(&lg.lock);
    gc = &lg.chip;
    gc.label = dev_name(dev);
    gc.owner = THIS_MODULE;
    gc.request = gpiochip_generic_request;
    gc.free = gpiochip_generic_free;
    gc.direction_input = lp_gpio_direction_input;
    gc.direction_output = lp_gpio_direction_output;
    gc.get = lp_gpio_get;
    gc.set = lp_gpio_set;
    gc.set_config = gpiochip_generic_config;
    gc.get_direction = lp_gpio_get_direction;
    gc.base = -1;
    gc.ngpio = LP_NUM_GPIO;
    gc.can_sleep = false;
    gc.add_pin_ranges = intel_gpio_add_pin_ranges;
    gc.parent = dev;
// set up interrupts
    irq = platform_get_irq_optional(pdev, 0);
    if (irq > 0) {
    struct gpio_irq_chip *girq;
    girq = &gc.irq;
    gpio_irq_chip_set_chip(girq, &lp_irqchip);
    girq.init_hw = lp_gpio_irq_init_hw;
    girq.parent_handler = lp_gpio_irq_handler;
    girq.num_parents = 1;
    girq.parents = devm_kcalloc(dev, girq.num_parents,
    sizeof(*girq.parents),
    GFP_KERNEL);
    if (!girq.parents)
    return -ENOMEM;
    girq.parents[0] = irq;
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_bad_irq;
    }
    ret = devm_gpiochip_add_data(dev, gc, lg);
    if (ret)
    return dev_err_probe(dev, ret, "failed to register gpiochip\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lp_gpio_resume(dev: *mut device) -> c_int {
    static int lp_gpio_resume(struct device *dev)
    {
    struct intel_pinctrl *lg = dev_get_drvdata(dev);
    struct gpio_chip *chip = &lg.chip;
    const char *dummy;
    int i;
// on some hardware suspend clears input sensing, re-enable it here
    for_each_requested_gpio(chip, i, dummy)
    lp_gpio_enable_input(lp_gpio_reg(chip, i, LP_CONFIG2));
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(lp_gpio_pm_ops, core::ptr::null_mut(), lp_gpio_resume);
    static const struct acpi_device_id lynxpoint_gpio_acpi_match[] = {
    { "INT33C7", (kernel_ulong_t)&lptlp_soc_data },
    { "INT3437", (kernel_ulong_t)&lptlp_soc_data },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, lynxpoint_gpio_acpi_match);
    static struct platform_driver lp_gpio_driver = {
    .probe          = lp_gpio_probe,
    .driver         = {
    .name   = "lp_gpio",
    .pm	= pm_sleep_ptr(&lp_gpio_pm_ops),
    .acpi_match_table = lynxpoint_gpio_acpi_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn lp_gpio_init() -> int __init {
    static int __init lp_gpio_init(void)
    {
    return platform_driver_register(&lp_gpio_driver);
    }
    subsys_initcall(lp_gpio_init);
#[no_mangle]
unsafe extern "C" fn lp_gpio_exit() -> void __exit {
    static void __exit lp_gpio_exit(void)
    {
    platform_driver_unregister(&lp_gpio_driver);
    }
    module_exit(lp_gpio_exit);
    MODULE_AUTHOR("Mathias Nyman (Intel)");
    MODULE_AUTHOR("Andy Shevchenko (Intel)");
    MODULE_DESCRIPTION("Intel Lynxpoint pinctrl driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:lp_gpio");
    MODULE_IMPORT_NS("PINCTRL_INTEL");

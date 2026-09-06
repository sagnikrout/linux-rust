//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/44x/warp.c
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
// PIKA Warp(tm) board specific routines
//
// Copyright (c) 2008-2009 PIKA Technologies
// Sean MacLennan <smaclennan@pikatech.com>
//

    static const struct of_device_id warp_of_bus[] __initconst = {
    { .compatible = "ibm,plb4", },
    { .compatible = "ibm,opb", },
    { .compatible = "ibm,ebc", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn warp_device_probe() -> int __init {
    static int __init warp_device_probe(void)
    {
    of_platform_bus_probe(core::ptr::null_mut(), warp_of_bus, core::ptr::null_mut());
    return 0;
    }
    machine_device_initcall(warp, warp_device_probe);
    define_machine(warp) {
    .name		= "Warp",
    .compatible	= "pika,warp",
    .progress 	= udbg_progress,
    .init_IRQ 	= uic_init_tree,
    .get_irq 	= uic_get_irq,
    .restart	= ppc4xx_reset_system,
    };
#[no_mangle]
unsafe extern "C" fn warp_post_info() -> int __init {
    static int __init warp_post_info(void)
    {
    struct device_node *np;
    void __iomem *fpga;
    u32 post1, post2;
// Sighhhh... POST information is in the sd area.
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "pika,fpga-sd");
    if (np == core::ptr::null_mut())
    return -ENOENT;
    fpga = of_iomap(np, 0);
    of_node_put(np);
    if (fpga == core::ptr::null_mut())
    return -ENOENT;
    post1 = in_be32(fpga + 0x40);
    post2 = in_be32(fpga + 0x44);
    iounmap(fpga);
    if (post1 || post2)
    printk(KERN_INFO "Warp POST %08x %08x\n", post1, post2);
    else
    printk(KERN_INFO "Warp POST OK\n");
    return 0;
    }

    static void __iomem *dtm_fpga;
pub const WARP_GREEN_LED: c_int = 0;
pub const WARP_RED_LED: c_int = 1;
    static struct gpio_led warp_gpio_led_pins[] = {
    [WARP_GREEN_LED] = {
    .name		= "green",
    .default_state	= LEDS_DEFSTATE_KEEP,
    .gpiod		= core::ptr::null_mut(), /* to be filled by pika_setup_leds() */
    },
    [WARP_RED_LED] = {
    .name		= "red",
    .default_state	= LEDS_DEFSTATE_KEEP,
    .gpiod		= core::ptr::null_mut(), /* to be filled by pika_setup_leds() */
    },
    };
    static struct gpio_led_platform_data warp_gpio_led_data = {
    .leds		= warp_gpio_led_pins,
    .num_leds	= ARRAY_SIZE(warp_gpio_led_pins),
    };
    static struct platform_device warp_gpio_leds = {
    .name	= "leds-gpio",
    .id	= -1,
    .dev	= {
    .platform_data = &warp_gpio_led_data,
    },
    };
#[no_mangle]
unsafe extern "C" fn temp_isr(irq: c_int, context: *mut c_void) -> irqreturn_t {
    static irqreturn_t temp_isr(int irq, void *context)
    {
    let mut value: c_int = 1;
    local_irq_disable();
    gpiod_set_value(warp_gpio_led_pins[WARP_GREEN_LED].gpiod, 0);
    printk(KERN_EMERG "\n\nCritical Temperature Shutdown\n\n");
    while (1) {
    if (dtm_fpga) {
    let mut reset: unsigned = in_be32(dtm_fpga + 0x14);
    out_be32(dtm_fpga + 0x14, reset);
    }
    gpiod_set_value(warp_gpio_led_pins[WARP_RED_LED].gpiod, value);
    value ^= 1;
    mdelay(500);
    }
// Not reached
    return IRQ_HANDLED;
    }
//
// Because green and red power LEDs are normally driven by leds-gpio driver,
// but in case of critical temperature shutdown we want to drive them
// ourselves, we acquire both and then create leds-gpio platform device
// ourselves, instead of doing it through device tree. This way we can still
// keep access to the gpios and use them when needed.
//
#[no_mangle]
unsafe extern "C" fn pika_setup_leds() -> c_int {
    static int pika_setup_leds(void)
    {
    struct device_node *np, *child;
    struct gpio_desc *gpio;
    struct gpio_led *led;
    let mut led_count: c_int = 0;
    int error;
    int i;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "warp-power-leds");
    if (!np) {
    printk(KERN_ERR __FILE__ ": Unable to find leds\n");
    return -ENOENT;
    }
    for_each_child_of_node(np, child) {
    for (i = 0; i < ARRAY_SIZE(warp_gpio_led_pins); i++) {
    led = &warp_gpio_led_pins[i];
    if (!of_node_name_eq(child, led.name))
    continue;
    if (led.gpiod) {
    printk(KERN_ERR __FILE__ ": %s led has already been defined\n",
    led.name);
    continue;
    }
    gpio = fwnode_gpiod_get_index(of_fwnode_handle(child),
    core::ptr::null_mut(), 0, GPIOD_ASIS,
    led.name);
    error = PTR_ERR_OR_ZERO(gpio);
    if (error) {
    printk(KERN_ERR __FILE__ ": Failed to get %s led gpio: %d\n",
    led.name, error);
    of_node_put(child);
    goto err_cleanup_pins;
    }
    led.gpiod = gpio;
    led_count++;
    }
    }
    of_node_put(np);
// Skip device registration if no leds have been defined
    if (led_count) {
    error = platform_device_register(&warp_gpio_leds);
    if (error) {
    printk(KERN_ERR __FILE__ ": Unable to add leds-gpio: %d\n",
    error);
    goto err_cleanup_pins;
    }
    }
    return 0;
    err_cleanup_pins:
    for (i = 0; i < ARRAY_SIZE(warp_gpio_led_pins); i++) {
    led = &warp_gpio_led_pins[i];
    gpiod_put(led.gpiod);
    led.gpiod = core::ptr::null_mut();
    }
    return error;
    }
    static void pika_setup_critical_temp(struct device_node *np,
    struct i2c_client *client)
    {
    int irq, rc;
// Do this before enabling critical temp interrupt since we
// may immediately interrupt.
//
    pika_setup_leds();
// These registers are in 1 degree increments.
    i2c_smbus_write_byte_data(client, 2, 65); /* Thigh */
    i2c_smbus_write_byte_data(client, 3,  0); /* Tlow */
    irq = irq_of_parse_and_map(np, 0);
    if (!irq) {
    printk(KERN_ERR __FILE__ ": Unable to get ad7414 irq\n");
    return;
    }
    rc = request_irq(irq, temp_isr, 0, "ad7414", core::ptr::null_mut());
    if (rc) {
    printk(KERN_ERR __FILE__
    ": Unable to request ad7414 irq %d = %d\n", irq, rc);
    return;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pika_dtm_check_fan(fpga: *mut void __iomem) {
    static inline void pika_dtm_check_fan(void __iomem *fpga)
    {
    static int fan_state;
    let mut fan: u32 = in_be32(fpga + 0x34) & (1 << 14);
    if (fan_state != fan) {
    fan_state = fan;
    if (fan)
    printk(KERN_WARNING "Fan rotation error detected."
    " Please check hardware.\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn pika_dtm_thread(fpga: *mut void __iomem) -> c_int {
    static int pika_dtm_thread(void __iomem *fpga)
    {
    struct device_node *np;
    struct i2c_client *client;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "adi,ad7414");
    if (np == core::ptr::null_mut())
    return -ENOENT;
    client = of_find_i2c_device_by_node(np);
    if (client == core::ptr::null_mut()) {
    of_node_put(np);
    return -ENOENT;
    }
    pika_setup_critical_temp(np, client);
    of_node_put(np);
    printk(KERN_INFO "Warp DTM thread running.\n");
    while (!kthread_should_stop()) {
    int val;
    val = i2c_smbus_read_word_data(client, 0);
    if (val < 0)
    dev_dbg(&client.dev, "DTM read temp failed.\n");
    else {
    let mut temp: i16 = swab16(val);
    out_be32(fpga + 0x20, temp);
    }
    pika_dtm_check_fan(fpga);
    set_current_state(TASK_INTERRUPTIBLE);
    schedule_timeout(HZ);
    }
    put_device(&client.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pika_dtm_start() -> int __init {
    static int __init pika_dtm_start(void)
    {
    struct task_struct *dtm_thread;
    struct device_node *np;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "pika,fpga");
    if (np == core::ptr::null_mut())
    return -ENOENT;
    dtm_fpga = of_iomap(np, 0);
    of_node_put(np);
    if (dtm_fpga == core::ptr::null_mut())
    return -ENOENT;
// Must get post info before thread starts.
    warp_post_info();
    dtm_thread = kthread_run(pika_dtm_thread, dtm_fpga, "pika-dtm");
    if (IS_ERR(dtm_thread)) {
    iounmap(dtm_fpga);
    return PTR_ERR(dtm_thread);
    }
    return 0;
    }
    machine_late_initcall(warp, pika_dtm_start);

    machine_late_initcall(warp, warp_post_info);

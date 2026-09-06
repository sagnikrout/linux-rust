//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/pwm_bl.c
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
// Simple PWM based backlight control, board code has to setup
// 1) pin configuration so PWM waveforms can output
// 2) platform_data being correctly configured
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_bl_data {
    pub pwm: *mut pwm_device,
    pub dev: *mut device,
    pub lth_brightness: c_uint,
    pub levels: *mut c_uint,
    pub enabled: bool,
    pub power_supply: *mut regulator,
    pub enable_gpio: *mut gpio_desc,
    pub scale: c_uint,
    pub post_pwm_on_delay: c_uint,
    pub pwm_off_delay: c_uint,
    int			(*notify)(struct device *,
    pub brightness): c_int,
    void			(*notify_after)(struct device *,
    pub brightness): c_int,
    pub ): *mut *mut void (exit)(struct device,
}

#[no_mangle]
unsafe extern "C" fn pwm_backlight_power_on(pb: *mut pwm_bl_data) {
    static void pwm_backlight_power_on(struct pwm_bl_data *pb)
    {
    int err;
    if (pb.enabled)
    return;
    if (pb.power_supply) {
    err = regulator_enable(pb.power_supply);
    if (err < 0)
    dev_err(pb.dev, "failed to enable power supply\n");
    }
    if (pb.post_pwm_on_delay)
    msleep(pb.post_pwm_on_delay);
    gpiod_set_value_cansleep(pb.enable_gpio, 1);
    pb.enabled = true;
    }
#[no_mangle]
unsafe extern "C" fn pwm_backlight_power_off(pb: *mut pwm_bl_data) {
    static void pwm_backlight_power_off(struct pwm_bl_data *pb)
    {
    if (!pb.enabled)
    return;
    gpiod_set_value_cansleep(pb.enable_gpio, 0);
    if (pb.pwm_off_delay)
    msleep(pb.pwm_off_delay);
    if (pb.power_supply)
    regulator_disable(pb.power_supply);
    pb.enabled = false;
    }
#[no_mangle]
unsafe extern "C" fn compute_duty_cycle(pb: *mut pwm_bl_data, brightness: c_int, state: *mut pwm_state) -> c_int {
    static int compute_duty_cycle(struct pwm_bl_data *pb, int brightness, struct pwm_state *state)
    {
    let mut lth: c_uint = pb.lth_brightness;
    u64 duty_cycle;
    if (pb.levels)
    duty_cycle = pb.levels[brightness];
    else
    duty_cycle = brightness;
    duty_cycle *= state.period - lth;
    do_div(duty_cycle, pb.scale);
    return duty_cycle + lth;
    }
#[no_mangle]
unsafe extern "C" fn pwm_backlight_update_status(bl: *mut backlight_device) -> c_int {
    static int pwm_backlight_update_status(struct backlight_device *bl)
    {
    struct pwm_bl_data *pb = bl_get_data(bl);
    let mut brightness: c_int = backlight_get_brightness(bl);
    struct pwm_state state;
    if (pb.notify)
    brightness = pb.notify(pb.dev, brightness);
    if (brightness > 0) {
    pwm_get_state(pb.pwm, &state);
    state.duty_cycle = compute_duty_cycle(pb, brightness, &state);
    state.enabled = true;
    pwm_apply_might_sleep(pb.pwm, &state);
    pwm_backlight_power_on(pb);
    } else {
    pwm_backlight_power_off(pb);
    pwm_get_state(pb.pwm, &state);
    state.duty_cycle = 0;
//
// We cannot assume a disabled PWM to drive its output to the
// inactive state. If we have an enable GPIO and/or a regulator
// we assume that this isn't relevant and we can disable the PWM
// to save power. If however there is neither an enable GPIO nor
// a regulator keep the PWM on be sure to get a constant
// inactive output.
//
    state.enabled = !pb.power_supply && !pb.enable_gpio;
    pwm_apply_might_sleep(pb.pwm, &state);
    }
    if (pb.notify_after)
    pb.notify_after(pb.dev, brightness);
    return 0;
    }
    static const struct backlight_ops pwm_backlight_ops = {
    .update_status	= pwm_backlight_update_status,
    };

pub const PWM_LUMINANCE_SHIFT: c_int = 16;

//
// CIE lightness to PWM conversion.
//
// The CIE 1931 lightness formula is what actually describes how we perceive
// light:
// Y = (L* / 903.3)           if L* ≤ 8
// Y = ((L* + 16) / 116)^3    if L* > 8
//
// Where Y is the luminance, the amount of light coming out of the screen, and
// is a number between 0.0 and 1.0; and L* is the lightness, how bright a human
// perceives the screen to be, and is a number between 0 and 100.
//
// The following function does the fixed point maths needed to implement the
// above formula.
//
#[no_mangle]
unsafe extern "C" fn cie1931(lightness: c_uint) -> u64 {
    static u64 cie1931(unsigned int lightness)
    {
    u64 retval;
//
// @lightness is given as a number between 0 and 1, expressed
// as a fixed-point number in scale
// PWM_LUMINANCE_SCALE. Convert to a percentage, still
// expressed as a fixed-point number, so the above formulas
// can be applied.
//
    lightness *= 100;
    if (lightness <= (8 * PWM_LUMINANCE_SCALE)) {
    retval = DIV_ROUND_CLOSEST(lightness * 10, 9033);
    } else {
    retval = (lightness + (16 * PWM_LUMINANCE_SCALE)) / 116;
    retval *= retval * retval;
    retval += 1ULL << (2*PWM_LUMINANCE_SHIFT - 1);
    retval >>= 2*PWM_LUMINANCE_SHIFT;
    }
    return retval;
    }
//
// Create a default correction table for PWM values to create linear brightness
// for LED based backlights using the CIE1931 algorithm.
//
    static
    int pwm_backlight_brightness_default(struct device *dev,
    struct platform_pwm_backlight_data *data,
    unsigned int period)
    {
    unsigned int i;
    u64 retval;
//
// Once we have 4096 levels there's little point going much higher...
// neither interactive sliders nor animation benefits from having
// more values in the table.
//
    data.max_brightness =
    min((int)DIV_ROUND_UP(period, fls(period)), 4096);
    data.levels = devm_kcalloc(dev, data.max_brightness,
    sizeof(*data.levels), GFP_KERNEL);
    if (!data.levels)
    return -ENOMEM;
// Fill the table using the cie1931 algorithm
    for (i = 0; i < data.max_brightness; i++) {
    retval = cie1931((i * PWM_LUMINANCE_SCALE) /
    data.max_brightness) * period;
    retval = DIV_ROUND_CLOSEST_ULL(retval, PWM_LUMINANCE_SCALE);
    if (retval > UINT_MAX)
    return -EINVAL;
    data.levels[i] = (unsigned int)retval;
    }
    data.dft_brightness = data.max_brightness / 2;
    data.max_brightness--;
    return 0;
    }
    static int pwm_backlight_parse_dt(struct device *dev,
    struct platform_pwm_backlight_data *data)
    {
    struct device_node *node = dev.of_node;
    unsigned int num_levels;
    let mut num_steps: c_uint = 0;
    struct property *prop;
    unsigned int *table;
    int length;
    u32 value;
    int ret;
    if (!node)
    return -ENODEV;
    memset(data, 0, sizeof(*data));
//
// These values are optional and set as 0 by default, the out values
// are modified only if a valid u32 value can be decoded.
//
    of_property_read_u32(node, "post-pwm-on-delay-ms",
    &data.post_pwm_on_delay);
    of_property_read_u32(node, "pwm-off-delay-ms", &data.pwm_off_delay);
//
// Determine the number of brightness levels, if this property is not
// set a default table of brightness levels will be used.
//
    prop = of_find_property(node, "brightness-levels", &length);
    if (!prop)
    return 0;
    num_levels = length / sizeof(u32);
// read brightness levels from DT property
    if (num_levels > 0) {
    data.levels = devm_kcalloc(dev, num_levels,
    sizeof(*data.levels), GFP_KERNEL);
    if (!data.levels)
    return -ENOMEM;
    ret = of_property_read_u32_array(node, "brightness-levels",
    data.levels,
    num_levels);
    if (ret < 0)
    return ret;
    ret = of_property_read_u32(node, "default-brightness-level",
    &value);
    if (ret < 0)
    return ret;
    data.dft_brightness = value;
//
// This property is optional, if is set enables linear
// interpolation between each of the values of brightness levels
// and creates a new pre-computed table.
//
    of_property_read_u32(node, "num-interpolated-steps",
    &num_steps);
//
// Make sure that there is at least two entries in the
// brightness-levels table, otherwise we can't interpolate
// between two points.
//
    if (num_steps) {
    let mut num_input_levels: c_uint = num_levels;
    unsigned int i;
    u32 x1, x2, x, dx;
    u32 y1, y2;
    s64 dy;
    if (num_input_levels < 2) {
    dev_err(dev, "can't interpolate\n");
    return -EINVAL;
    }
//
// Recalculate the number of brightness levels, now
// taking in consideration the number of interpolated
// steps between two levels.
//
    num_levels = (num_input_levels - 1) * num_steps + 1;
    dev_dbg(dev, "new number of brightness levels: %d\n",
    num_levels);
//
// Create a new table of brightness levels with all the
// interpolated steps.
//
    table = devm_kcalloc(dev, num_levels, sizeof(*table),
    GFP_KERNEL);
    if (!table)
    return -ENOMEM;
//
// Fill the interpolated table[x] = y
// by draw lines between each (x1, y1) to (x2, y2).
//
    dx = num_steps;
    for (i = 0; i < num_input_levels - 1; i++) {
    x1 = i * dx;
    x2 = x1 + dx;
    y1 = data.levels[i];
    y2 = data.levels[i + 1];
    dy = (s64)y2 - y1;
    for (x = x1; x < x2; x++) {
    table[x] = y1 +
    div_s64(dy * (x - x1), dx);
    }
    }
// Fill in the last point, since no line starts here.
    table[x2] = y2;
//
// As we use interpolation lets remove current
// brightness levels table and replace for the
// new interpolated table.
//
    devm_kfree(dev, data.levels);
    data.levels = table;
    }
    data.max_brightness = num_levels - 1;
    }
    return 0;
    }
    static const struct of_device_id pwm_backlight_of_match[] = {
    { .compatible = "pwm-backlight" },
    { }
    };
    MODULE_DEVICE_TABLE(of, pwm_backlight_of_match);

    static int pwm_backlight_parse_dt(struct device *dev,
    struct platform_pwm_backlight_data *data)
    {
    return -ENODEV;
    }
    static
    int pwm_backlight_brightness_default(struct device *dev,
    struct platform_pwm_backlight_data *data,
    unsigned int period)
    {
    return -ENODEV;
    }

#[no_mangle]
unsafe extern "C" fn pwm_backlight_is_linear(data: *mut platform_pwm_backlight_data) -> bool {
    static bool pwm_backlight_is_linear(struct platform_pwm_backlight_data *data)
    {
    let mut nlevels: c_uint = data.max_brightness + 1;
    let mut min_val: c_uint = data.levels[0];
    let mut max_val: c_uint = data.levels[nlevels - 1];
//
// Multiplying by 128 means that even in pathological cases such
// as (max_val - min_val) == nlevels the error at max_val is less
// than 1%.
//
    let mut slope: c_uint = (128 * (max_val - min_val)) / nlevels;
    unsigned int margin = (max_val - min_val) / 20; /* 5% */
    int i;
    for (i = 1; i < nlevels; i++) {
    let mut linear_value: c_uint = min_val + ((i * slope) / 128);
    let mut delta: c_uint = abs(linear_value - data.levels[i]);
    if (delta > margin)
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn pwm_backlight_initial_power_state(pb: *const pwm_bl_data) -> c_int {
    static int pwm_backlight_initial_power_state(const struct pwm_bl_data *pb)
    {
    struct device_node *node = pb.dev.of_node;
    let mut active: bool = true;
//
// If the enable GPIO is present, observable (either as input
// or output) and off then the backlight is not currently active.
//
    if (pb.enable_gpio && gpiod_get_value_cansleep(pb.enable_gpio) == 0)
    active = false;
    if (pb.power_supply && !regulator_is_enabled(pb.power_supply))
    active = false;
    if (!pwm_is_enabled(pb.pwm))
    active = false;
//
// Synchronize the enable_gpio with the observed state of the
// hardware.
//
    gpiod_direction_output(pb.enable_gpio, active);
//
// Do not change pb->enabled here! pb->enabled essentially
// tells us if we own one of the regulator's use counts and
// right now we do not.
//
// Not booted with device tree or no phandle link to the node
    if (!node || !node.phandle)
    return BACKLIGHT_POWER_ON;
//
// If the driver is probed from the device tree and there is a
// phandle link pointing to the backlight node, it is safe to
// assume that another driver will enable the backlight at the
// appropriate time. Therefore, if it is disabled, keep it so.
//
    return active ? BACKLIGHT_POWER_ON : BACKLIGHT_POWER_OFF;
    }
#[no_mangle]
unsafe extern "C" fn pwm_backlight_probe(pdev: *mut platform_device) -> c_int {
    static int pwm_backlight_probe(struct platform_device *pdev)
    {
    struct platform_pwm_backlight_data *data = dev_get_platdata(&pdev.dev);
    struct platform_pwm_backlight_data defdata;
    struct backlight_properties props;
    struct backlight_device *bl;
    struct pwm_bl_data *pb;
    struct pwm_state state;
    unsigned int i;
    int ret;
    if (!data) {
    ret = pwm_backlight_parse_dt(&pdev.dev, &defdata);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, ret,
    "failed to find platform data\n");
    data = &defdata;
    }
    if (data.init) {
    ret = data.init(&pdev.dev);
    if (ret < 0)
    return ret;
    }
    pb = devm_kzalloc(&pdev.dev, sizeof(*pb), GFP_KERNEL);
    if (!pb) {
    ret = -ENOMEM;
    goto err_alloc;
    }
    pb.notify = data.notify;
    pb.notify_after = data.notify_after;
    pb.exit = data.exit;
    pb.dev = &pdev.dev;
    pb.enabled = false;
    pb.post_pwm_on_delay = data.post_pwm_on_delay;
    pb.pwm_off_delay = data.pwm_off_delay;
    pb.enable_gpio = devm_gpiod_get_optional(&pdev.dev, "enable",
    GPIOD_ASIS);
    if (IS_ERR(pb.enable_gpio)) {
    ret = dev_err_probe(&pdev.dev, PTR_ERR(pb.enable_gpio),
    "failed to acquire enable GPIO\n");
    goto err_alloc;
    }
    pb.power_supply = devm_regulator_get_optional(&pdev.dev, "power");
    if (IS_ERR(pb.power_supply)) {
    ret = PTR_ERR(pb.power_supply);
    if (ret == -ENODEV) {
    pb.power_supply = core::ptr::null_mut();
    } else {
    dev_err_probe(&pdev.dev, ret,
    "failed to acquire power regulator\n");
    goto err_alloc;
    }
    }
    pb.pwm = devm_pwm_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(pb.pwm)) {
    ret = dev_err_probe(&pdev.dev, PTR_ERR(pb.pwm),
    "unable to request PWM\n");
    goto err_alloc;
    }
    dev_dbg(&pdev.dev, "got pwm for backlight\n");
// Sync up PWM state.
    pwm_init_state(pb.pwm, &state);
//
// The DT case will set the pwm_period_ns field to 0 and store the
// period, parsed from the DT, in the PWM device. For the non-DT case,
// set the period from platform data if it has not already been set
// via the PWM lookup table.
//
    if (!state.period && (data.pwm_period_ns > 0))
    state.period = data.pwm_period_ns;
    ret = pwm_apply_might_sleep(pb.pwm, &state);
    if (ret) {
    dev_err_probe(&pdev.dev, ret,
    "failed to apply initial PWM state");
    goto err_alloc;
    }
    memset(&props, 0, sizeof(struct backlight_properties));
    if (data.levels) {
    pb.levels = data.levels;
//
// For the DT case, only when brightness levels is defined
// data->levels is filled. For the non-DT case, data->levels
// can come from platform data, however is not usual.
//
    for (i = 0; i <= data.max_brightness; i++)
    if (data.levels[i] > pb.scale)
    pb.scale = data.levels[i];
    if (pwm_backlight_is_linear(data))
    props.scale = BACKLIGHT_SCALE_LINEAR;
    else
    props.scale = BACKLIGHT_SCALE_NON_LINEAR;
    } else if (!data.max_brightness) {
//
// If no brightness levels are provided and max_brightness is
// not set, use the default brightness table. For the DT case,
// max_brightness is set to 0 when brightness levels is not
// specified. For the non-DT case, max_brightness is usually
// set to some value.
//
// Get the PWM period (in nanoseconds)
    pwm_get_state(pb.pwm, &state);
    ret = pwm_backlight_brightness_default(&pdev.dev, data,
    state.period);
    if (ret < 0) {
    dev_err_probe(&pdev.dev, ret,
    "failed to setup default brightness table\n");
    goto err_alloc;
    }
    for (i = 0; i <= data.max_brightness; i++) {
    if (data.levels[i] > pb.scale)
    pb.scale = data.levels[i];
    pb.levels = data.levels;
    }
    props.scale = BACKLIGHT_SCALE_NON_LINEAR;
    } else {
//
// That only happens for the non-DT case, where platform data
// sets the max_brightness value.
//
    pb.scale = data.max_brightness;
    }
    pb.lth_brightness = data.lth_brightness * (div_u64(state.period,
    pb.scale));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = data.max_brightness;
    bl = backlight_device_register(dev_name(&pdev.dev), &pdev.dev, pb,
    &pwm_backlight_ops, &props);
    if (IS_ERR(bl)) {
    ret = dev_err_probe(&pdev.dev, PTR_ERR(bl),
    "failed to register backlight\n");
    goto err_alloc;
    }
    if (data.dft_brightness > data.max_brightness) {
    dev_warn(&pdev.dev,
    "invalid default brightness level: %u, using %u\n",
    data.dft_brightness, data.max_brightness);
    data.dft_brightness = data.max_brightness;
    }
    bl.props.brightness = data.dft_brightness;
    bl.props.power = pwm_backlight_initial_power_state(pb);
    backlight_update_status(bl);
    platform_set_drvdata(pdev, bl);
    return 0;
    err_alloc:
    if (data.exit)
    data.exit(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pwm_backlight_remove(pdev: *mut platform_device) {
    static void pwm_backlight_remove(struct platform_device *pdev)
    {
    struct backlight_device *bl = platform_get_drvdata(pdev);
    struct pwm_bl_data *pb = bl_get_data(bl);
    struct pwm_state state;
    backlight_device_unregister(bl);
    pwm_backlight_power_off(pb);
    pwm_get_state(pb.pwm, &state);
    state.duty_cycle = 0;
    state.enabled = false;
    pwm_apply_might_sleep(pb.pwm, &state);
    if (pb.exit)
    pb.exit(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn pwm_backlight_shutdown(pdev: *mut platform_device) {
    static void pwm_backlight_shutdown(struct platform_device *pdev)
    {
    struct backlight_device *bl = platform_get_drvdata(pdev);
    struct pwm_bl_data *pb = bl_get_data(bl);
    struct pwm_state state;
    pwm_backlight_power_off(pb);
    pwm_get_state(pb.pwm, &state);
    state.duty_cycle = 0;
    state.enabled = false;
    pwm_apply_might_sleep(pb.pwm, &state);
    }

#[no_mangle]
unsafe extern "C" fn pwm_backlight_suspend(dev: *mut device) -> c_int {
    static int pwm_backlight_suspend(struct device *dev)
    {
    struct backlight_device *bl = dev_get_drvdata(dev);
    struct pwm_bl_data *pb = bl_get_data(bl);
    struct pwm_state state;
    if (pb.notify)
    pb.notify(pb.dev, 0);
    pwm_backlight_power_off(pb);
//
// Note that disabling the PWM doesn't guarantee that the output stays
// at its inactive state. However without the PWM disabled, the PWM
// driver refuses to suspend. So disable here even though this might
// enable the backlight on poorly designed boards.
//
    pwm_get_state(pb.pwm, &state);
    state.duty_cycle = 0;
    state.enabled = false;
    pwm_apply_might_sleep(pb.pwm, &state);
    if (pb.notify_after)
    pb.notify_after(pb.dev, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwm_backlight_resume(dev: *mut device) -> c_int {
    static int pwm_backlight_resume(struct device *dev)
    {
    struct backlight_device *bl = dev_get_drvdata(dev);
    backlight_update_status(bl);
    return 0;
    }

    static const struct dev_pm_ops pwm_backlight_pm_ops = {

    .suspend = pwm_backlight_suspend,
    .resume = pwm_backlight_resume,
    .poweroff = pwm_backlight_suspend,
    .restore = pwm_backlight_resume,

    };
    static struct platform_driver pwm_backlight_driver = {
    .driver		= {
    .name		= "pwm-backlight",
    .pm		= &pwm_backlight_pm_ops,
    .of_match_table	= of_match_ptr(pwm_backlight_of_match),
    },
    .probe		= pwm_backlight_probe,
    .remove		= pwm_backlight_remove,
    .shutdown	= pwm_backlight_shutdown,
    };
    module_platform_driver(pwm_backlight_driver);
    MODULE_DESCRIPTION("PWM based Backlight Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:pwm-backlight");

//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/rtas-proc.c
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
// Copyright (C) 2000 Tilmann Bitterberg
// (tilmann@bitterberg.de)
//
// RTAS (Runtime Abstraction Services) stuff
// Intention is to provide a clean user interface
// to use the RTAS.
//
// TODO:
// Split off a header file and maybe move it to a different
// location. Write Documentation on what the /proc/rtas/ entries
// actually do.
//

// Token for Sensors
pub const KEY_SWITCH: c_uint = 0x0001;
pub const ENCLOSURE_SWITCH: c_uint = 0x0002;
pub const THERMAL_SENSOR: c_uint = 0x0003;
pub const LID_STATUS: c_uint = 0x0004;
pub const POWER_SOURCE: c_uint = 0x0005;
pub const BATTERY_VOLTAGE: c_uint = 0x0006;
pub const BATTERY_REMAINING: c_uint = 0x0007;
pub const BATTERY_PERCENTAGE: c_uint = 0x0008;
pub const EPOW_SENSOR: c_uint = 0x0009;
pub const BATTERY_CYCLESTATE: c_uint = 0x000a;
pub const BATTERY_CHARGING: c_uint = 0x000b;
// IBM specific sensors
pub const IBM_SURVEILLANCE: c_uint = 0x2328 /* 9000 */;
pub const IBM_FANRPM: c_uint = 0x2329 /* 9001 */;
pub const IBM_VOLTAGE: c_uint = 0x232a /* 9002 */;
pub const IBM_DRCONNECTOR: c_uint = 0x232b /* 9003 */;
pub const IBM_POWERSUPPLY: c_uint = 0x232c /* 9004 */;
// Status return values
pub const SENSOR_CRITICAL_HIGH: c_int = 13;
pub const SENSOR_WARNING_HIGH: c_int = 12;
pub const SENSOR_NORMAL: c_int = 11;
pub const SENSOR_WARNING_LOW: c_int = 10;
pub const SENSOR_CRITICAL_LOW: c_int = 9;
pub const SENSOR_SUCCESS: c_int = 0;

// Location Codes

// reserved / not used		'H'

// reserved / not used		'J'

// Tokens for indicators
pub const TONE_FREQUENCY: c_uint = 0x0001 /* 0 - 1000 (HZ)*/;
pub const TONE_VOLUME: c_uint = 0x0002 /* 0 - 100 (%) */;
pub const SYSTEM_POWER_STATE: c_uint = 0x0003;
pub const WARNING_LIGHT: c_uint = 0x0004;
pub const DISK_ACTIVITY_LIGHT: c_uint = 0x0005;
pub const HEX_DISPLAY_UNIT: c_uint = 0x0006;
pub const BATTERY_WARNING_TIME: c_uint = 0x0007;
pub const CONDITION_CYCLE_REQUEST: c_uint = 0x0008;
pub const SURVEILLANCE_INDICATOR: c_uint = 0x2328 /* 9000 */;
pub const DR_ACTION: c_uint = 0x2329 /* 9001 */;
pub const DR_INDICATOR: c_uint = 0x232a /* 9002 */;
// 9003 - 9004: Vendor specific
// 9006 - 9999: Vendor specific
// other

pub const MAX_LINELENGTH: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct individual_sensor {
    pub token: c_uint,
    pub quant: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtas_sensors {
    pub sensor: [individual_sensor; MAX_SENSORS],
    pub quant: c_uint,
}

// Globals
    static struct rtas_sensors sensors;
    static struct device_node *rtas_node = core::ptr::null_mut();
    static unsigned long power_on_time = 0; /* Save the time the user set */
    static char progress_led[MAX_LINELENGTH];
    let mut rtas_tone_frequency: static unsigned long = 1000;
    let mut rtas_tone_volume: static unsigned long = 0;
// ******************************************************************
// Declarations
    static int ppc_rtas_sensors_show(struct seq_file *m, void *v);
    static int ppc_rtas_clock_show(struct seq_file *m, void *v);
    static ssize_t ppc_rtas_clock_write(struct file *file,
    const char __user *buf, size_t count, loff_t *ppos);
    static int ppc_rtas_progress_show(struct seq_file *m, void *v);
    static ssize_t ppc_rtas_progress_write(struct file *file,
    const char __user *buf, size_t count, loff_t *ppos);
    static int ppc_rtas_poweron_show(struct seq_file *m, void *v);
    static ssize_t ppc_rtas_poweron_write(struct file *file,
    const char __user *buf, size_t count, loff_t *ppos);
    static ssize_t ppc_rtas_tone_freq_write(struct file *file,
    const char __user *buf, size_t count, loff_t *ppos);
    static int ppc_rtas_tone_freq_show(struct seq_file *m, void *v);
    static ssize_t ppc_rtas_tone_volume_write(struct file *file,
    const char __user *buf, size_t count, loff_t *ppos);
    static int ppc_rtas_tone_volume_show(struct seq_file *m, void *v);
    static int ppc_rtas_rmo_buf_show(struct seq_file *m, void *v);
#[no_mangle]
unsafe extern "C" fn poweron_open(inode: *mut inode, file: *mut file) -> c_int {
    static int poweron_open(struct inode *inode, struct file *file)
    {
    return single_open(file, ppc_rtas_poweron_show, core::ptr::null_mut());
    }
    static const struct proc_ops ppc_rtas_poweron_proc_ops = {
    .proc_open	= poweron_open,
    .proc_read	= seq_read,
    .proc_lseek	= seq_lseek,
    .proc_write	= ppc_rtas_poweron_write,
    .proc_release	= single_release,
    };
#[no_mangle]
unsafe extern "C" fn progress_open(inode: *mut inode, file: *mut file) -> c_int {
    static int progress_open(struct inode *inode, struct file *file)
    {
    return single_open(file, ppc_rtas_progress_show, core::ptr::null_mut());
    }
    static const struct proc_ops ppc_rtas_progress_proc_ops = {
    .proc_open	= progress_open,
    .proc_read	= seq_read,
    .proc_lseek	= seq_lseek,
    .proc_write	= ppc_rtas_progress_write,
    .proc_release	= single_release,
    };
#[no_mangle]
unsafe extern "C" fn clock_open(inode: *mut inode, file: *mut file) -> c_int {
    static int clock_open(struct inode *inode, struct file *file)
    {
    return single_open(file, ppc_rtas_clock_show, core::ptr::null_mut());
    }
    static const struct proc_ops ppc_rtas_clock_proc_ops = {
    .proc_open	= clock_open,
    .proc_read	= seq_read,
    .proc_lseek	= seq_lseek,
    .proc_write	= ppc_rtas_clock_write,
    .proc_release	= single_release,
    };
#[no_mangle]
unsafe extern "C" fn tone_freq_open(inode: *mut inode, file: *mut file) -> c_int {
    static int tone_freq_open(struct inode *inode, struct file *file)
    {
    return single_open(file, ppc_rtas_tone_freq_show, core::ptr::null_mut());
    }
    static const struct proc_ops ppc_rtas_tone_freq_proc_ops = {
    .proc_open	= tone_freq_open,
    .proc_read	= seq_read,
    .proc_lseek	= seq_lseek,
    .proc_write	= ppc_rtas_tone_freq_write,
    .proc_release	= single_release,
    };
#[no_mangle]
unsafe extern "C" fn tone_volume_open(inode: *mut inode, file: *mut file) -> c_int {
    static int tone_volume_open(struct inode *inode, struct file *file)
    {
    return single_open(file, ppc_rtas_tone_volume_show, core::ptr::null_mut());
    }
    static const struct proc_ops ppc_rtas_tone_volume_proc_ops = {
    .proc_open	= tone_volume_open,
    .proc_read	= seq_read,
    .proc_lseek	= seq_lseek,
    .proc_write	= ppc_rtas_tone_volume_write,
    .proc_release	= single_release,
    };
    static int ppc_rtas_find_all_sensors(void);
    static void ppc_rtas_process_sensor(struct seq_file *m,
    struct individual_sensor *s, int state, int error, const char *loc);
    static char *ppc_rtas_process_error(int error);
    static void get_location_code(struct seq_file *m,
    struct individual_sensor *s, const char *loc);
    static void check_location_string(struct seq_file *m, const char *c);
    static void check_location(struct seq_file *m, const char *c);
#[no_mangle]
unsafe extern "C" fn proc_rtas_init() -> int __init {
    static int __init proc_rtas_init(void)
    {
    if (!machine_is(pseries))
    return -ENODEV;
    rtas_node = of_find_node_by_name(core::ptr::null_mut(), "rtas");
    if (rtas_node == core::ptr::null_mut())
    return -ENODEV;
    proc_create("powerpc/rtas/progress", 0644, core::ptr::null_mut(),
    &ppc_rtas_progress_proc_ops);
    proc_create("powerpc/rtas/clock", 0644, core::ptr::null_mut(),
    &ppc_rtas_clock_proc_ops);
    proc_create("powerpc/rtas/poweron", 0644, core::ptr::null_mut(),
    &ppc_rtas_poweron_proc_ops);
    proc_create_single("powerpc/rtas/sensors", 0444, core::ptr::null_mut(),
    ppc_rtas_sensors_show);
    proc_create("powerpc/rtas/frequency", 0644, core::ptr::null_mut(),
    &ppc_rtas_tone_freq_proc_ops);
    proc_create("powerpc/rtas/volume", 0644, core::ptr::null_mut(),
    &ppc_rtas_tone_volume_proc_ops);
    proc_create_single("powerpc/rtas/rmo_buffer", 0400, core::ptr::null_mut(),
    ppc_rtas_rmo_buf_show);
    return 0;
    }
    __initcall(proc_rtas_init);
#[no_mangle]
unsafe extern "C" fn parse_number(p: *const char __user, count: usize, val: *mut u64) -> c_int {
    static int parse_number(const char __user *p, size_t count, u64 *val)
    {
    char buf[40];
    if (count > 39)
    return -EINVAL;
    if (copy_from_user(buf, p, count))
    return -EFAULT;
    buf[count] = 0;
    return kstrtoull(buf, 10, val);
    }
// ******************************************************************
// POWER-ON-TIME
// ******************************************************************
    static ssize_t ppc_rtas_poweron_write(struct file *file,
    const char __user *buf, size_t count, loff_t *ppos)
    {
    struct rtc_time tm;
    time64_t nowtime;
    let mut error: c_int = parse_number(buf, count, &nowtime);
    if (error)
    return error;
    power_on_time = nowtime; /* save the time */
    rtc_time64_to_tm(nowtime, &tm);
    error = rtas_call(rtas_function_token(RTAS_FN_SET_TIME_FOR_POWER_ON), 7, 1, core::ptr::null_mut(),
    tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday,
    tm.tm_hour, tm.tm_min, tm.tm_sec, 0 /* nano */);
    if (error)
    printk(KERN_WARNING "error: setting poweron time returned: %s\n",
    ppc_rtas_process_error(error));
    return count;
    }
// ******************************************************************
#[no_mangle]
unsafe extern "C" fn ppc_rtas_poweron_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ppc_rtas_poweron_show(struct seq_file *m, void *v)
    {
    if (power_on_time == 0)
    seq_printf(m, "Power on time not set\n");
    else
    seq_printf(m, "%lu\n",power_on_time);
    return 0;
    }
// ******************************************************************
// PROGRESS
// ******************************************************************
    static ssize_t ppc_rtas_progress_write(struct file *file,
    const char __user *buf, size_t count, loff_t *ppos)
    {
    unsigned long hex;
    if (count >= MAX_LINELENGTH)
    count = MAX_LINELENGTH -1;
    if (copy_from_user(progress_led, buf, count)) { /* save the string */
    return -EFAULT;
    }
    progress_led[count] = 0;
// Lets see if the user passed hexdigits
    hex = simple_strtoul(progress_led, core::ptr::null_mut(), 10);
    rtas_progress ((char *)progress_led, hex);
    return count;
// clear the line
// rtas_progress("                   ", 0xffff);
    }
// ******************************************************************
#[no_mangle]
unsafe extern "C" fn ppc_rtas_progress_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ppc_rtas_progress_show(struct seq_file *m, void *v)
    {
    if (progress_led[0])
    seq_printf(m, "%s\n", progress_led);
    return 0;
    }
// ******************************************************************
// CLOCK
// ******************************************************************
    static ssize_t ppc_rtas_clock_write(struct file *file,
    const char __user *buf, size_t count, loff_t *ppos)
    {
    struct rtc_time tm;
    time64_t nowtime;
    let mut error: c_int = parse_number(buf, count, &nowtime);
    if (error)
    return error;
    rtc_time64_to_tm(nowtime, &tm);
    error = rtas_call(rtas_function_token(RTAS_FN_SET_TIME_OF_DAY), 7, 1, core::ptr::null_mut(),
    tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday,
    tm.tm_hour, tm.tm_min, tm.tm_sec, 0);
    if (error)
    printk(KERN_WARNING "error: setting the clock returned: %s\n",
    ppc_rtas_process_error(error));
    return count;
    }
// ******************************************************************
#[no_mangle]
unsafe extern "C" fn ppc_rtas_clock_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ppc_rtas_clock_show(struct seq_file *m, void *v)
    {
    int ret[8];
    let mut error: c_int = rtas_call(rtas_function_token(RTAS_FN_GET_TIME_OF_DAY), 0, 8, ret);
    if (error) {
    printk(KERN_WARNING "error: reading the clock returned: %s\n",
    ppc_rtas_process_error(error));
    seq_printf(m, "0");
    } else {
    unsigned int year, mon, day, hour, min, sec;
    year = ret[0]; mon  = ret[1]; day  = ret[2];
    hour = ret[3]; min  = ret[4]; sec  = ret[5];
    seq_printf(m, "%lld\n",
    mktime64(year, mon, day, hour, min, sec));
    }
    return 0;
    }
// ******************************************************************
// SENSOR STUFF
// ******************************************************************
#[no_mangle]
unsafe extern "C" fn ppc_rtas_sensors_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ppc_rtas_sensors_show(struct seq_file *m, void *v)
    {
    int i,j;
    int state, error;
    let mut get_sensor_state: c_int = rtas_function_token(RTAS_FN_GET_SENSOR_STATE);
    seq_printf(m, "RTAS (RunTime Abstraction Services) Sensor Information\n");
    seq_printf(m, "Sensor\t\tValue\t\tCondition\tLocation\n");
    seq_printf(m, "********************************************************\n");
    if (ppc_rtas_find_all_sensors() != 0) {
    seq_printf(m, "\nNo sensors are available\n");
    return 0;
    }
    for (i=0; i<sensors.quant; i++) {
    struct individual_sensor *p = &sensors.sensor[i];
    char rstr[64];
    const char *loc;
    int llen, offs;
    sprintf (rstr, SENSOR_PREFIX"%04d", p.token);
    loc = of_get_property(rtas_node, rstr, &llen);
// A sensor may have multiple instances
    for (j = 0, offs = 0; j <= p.quant; j++) {
    error =	rtas_call(get_sensor_state, 2, 2, &state,
    p.token, j);
    ppc_rtas_process_sensor(m, p, state, error, loc);
    seq_putc(m, '\n');
    if (loc) {
    offs += strlen(loc) + 1;
    loc += strlen(loc) + 1;
    if (offs >= llen)
    loc = core::ptr::null_mut();
    }
    }
    }
    return 0;
    }
// ******************************************************************
#[no_mangle]
unsafe extern "C" fn ppc_rtas_find_all_sensors() -> c_int {
    static int ppc_rtas_find_all_sensors(void)
    {
    const unsigned int *utmp;
    int len, i;
    utmp = of_get_property(rtas_node, "rtas-sensors", &len);
    if (utmp == core::ptr::null_mut()) {
    printk (KERN_ERR "error: could not get rtas-sensors\n");
    return 1;
    }
    sensors.quant = len / 8;      /* int + int */
    for (i=0; i<sensors.quant; i++) {
    sensors.sensor[i].token = *utmp++;
    sensors.sensor[i].quant = *utmp++;
    }
    return 0;
    }
// ******************************************************************
//
// Builds a string of what rtas returned
//
    static char *ppc_rtas_process_error(int error)
    {
    switch (error) {
    case SENSOR_CRITICAL_HIGH:
    return "(critical high)";
    case SENSOR_WARNING_HIGH:
    return "(warning high)";
    case SENSOR_NORMAL:
    return "(normal)";
    case SENSOR_WARNING_LOW:
    return "(warning low)";
    case SENSOR_CRITICAL_LOW:
    return "(critical low)";
    case SENSOR_SUCCESS:
    return "(read ok)";
    case SENSOR_HW_ERROR:
    return "(hardware error)";
    case SENSOR_BUSY:
    return "(busy)";
    case SENSOR_NOT_EXIST:
    return "(non existent)";
    case SENSOR_DR_ENTITY:
    return "(dr entity removed)";
    default:
    return "(UNKNOWN)";
    }
    }
// ******************************************************************
//
// Builds a string out of what the sensor said
//
    static void ppc_rtas_process_sensor(struct seq_file *m,
    struct individual_sensor *s, int state, int error, const char *loc)
    {
// Defined return vales
    const char * key_switch[]        = { "Off\t", "Normal\t", "Secure\t",
    "Maintenance" };
    const char * enclosure_switch[]  = { "Closed", "Open" };
    const char * lid_status[]        = { " ", "Open", "Closed" };
    const char * power_source[]      = { "AC\t", "Battery",
    "AC & Battery" };
    const char * battery_remaining[] = { "Very Low", "Low", "Mid", "High" };
    const char * epow_sensor[]       = {
    "EPOW Reset", "Cooling warning", "Power warning",
    "System shutdown", "System halt", "EPOW main enclosure",
    "EPOW power off" };
    const char * battery_cyclestate[]  = { "None", "In progress",
    "Requested" };
    const char * battery_charging[]    = { "Charging", "Discharging",
    "No current flow" };
    const char * ibm_drconnector[]     = { "Empty", "Present", "Unusable",
    "Exchange" };
    let mut have_strings: c_int = 0;
    let mut num_states: c_int = 0;
    let mut temperature: c_int = 0;
    let mut unknown: c_int = 0;
// What kind of sensor do we have here?
    switch (s.token) {
    case KEY_SWITCH:
    seq_printf(m, "Key switch:\t");
    num_states = sizeof(key_switch) / sizeof(char *);
    if (state < num_states) {
    seq_printf(m, "%s\t", key_switch[state]);
    have_strings = 1;
    }
    break;
    case ENCLOSURE_SWITCH:
    seq_printf(m, "Enclosure switch:\t");
    num_states = sizeof(enclosure_switch) / sizeof(char *);
    if (state < num_states) {
    seq_printf(m, "%s\t",
    enclosure_switch[state]);
    have_strings = 1;
    }
    break;
    case THERMAL_SENSOR:
    seq_printf(m, "Temp. (C/F):\t");
    temperature = 1;
    break;
    case LID_STATUS:
    seq_printf(m, "Lid status:\t");
    num_states = sizeof(lid_status) / sizeof(char *);
    if (state < num_states) {
    seq_printf(m, "%s\t", lid_status[state]);
    have_strings = 1;
    }
    break;
    case POWER_SOURCE:
    seq_printf(m, "Power source:\t");
    num_states = sizeof(power_source) / sizeof(char *);
    if (state < num_states) {
    seq_printf(m, "%s\t",
    power_source[state]);
    have_strings = 1;
    }
    break;
    case BATTERY_VOLTAGE:
    seq_printf(m, "Battery voltage:\t");
    break;
    case BATTERY_REMAINING:
    seq_printf(m, "Battery remaining:\t");
    num_states = sizeof(battery_remaining) / sizeof(char *);
    if (state < num_states)
    {
    seq_printf(m, "%s\t",
    battery_remaining[state]);
    have_strings = 1;
    }
    break;
    case BATTERY_PERCENTAGE:
    seq_printf(m, "Battery percentage:\t");
    break;
    case EPOW_SENSOR:
    seq_printf(m, "EPOW Sensor:\t");
    num_states = sizeof(epow_sensor) / sizeof(char *);
    if (state < num_states) {
    seq_printf(m, "%s\t", epow_sensor[state]);
    have_strings = 1;
    }
    break;
    case BATTERY_CYCLESTATE:
    seq_printf(m, "Battery cyclestate:\t");
    num_states = sizeof(battery_cyclestate) /
    sizeof(char *);
    if (state < num_states) {
    seq_printf(m, "%s\t",
    battery_cyclestate[state]);
    have_strings = 1;
    }
    break;
    case BATTERY_CHARGING:
    seq_printf(m, "Battery Charging:\t");
    num_states = sizeof(battery_charging) / sizeof(char *);
    if (state < num_states) {
    seq_printf(m, "%s\t",
    battery_charging[state]);
    have_strings = 1;
    }
    break;
    case IBM_SURVEILLANCE:
    seq_printf(m, "Surveillance:\t");
    break;
    case IBM_FANRPM:
    seq_printf(m, "Fan (rpm):\t");
    break;
    case IBM_VOLTAGE:
    seq_printf(m, "Voltage (mv):\t");
    break;
    case IBM_DRCONNECTOR:
    seq_printf(m, "DR connector:\t");
    num_states = sizeof(ibm_drconnector) / sizeof(char *);
    if (state < num_states) {
    seq_printf(m, "%s\t",
    ibm_drconnector[state]);
    have_strings = 1;
    }
    break;
    case IBM_POWERSUPPLY:
    seq_printf(m, "Powersupply:\t");
    break;
    default:
    seq_printf(m,  "Unknown sensor (type %d), ignoring it\n",
    s.token);
    unknown = 1;
    have_strings = 1;
    break;
    }
    if (have_strings == 0) {
    if (temperature) {
    seq_printf(m, "%4d /%4d\t", state, cel_to_fahr(state));
    } else
    seq_printf(m, "%10d\t", state);
    }
    if (unknown == 0) {
    seq_printf(m, "%s\t", ppc_rtas_process_error(error));
    get_location_code(m, s, loc);
    }
    }
// ******************************************************************
#[no_mangle]
unsafe extern "C" fn check_location(m: *mut seq_file, c: *const c_char) {
    static void check_location(struct seq_file *m, const char *c)
    {
    switch (c[0]) {
    case LOC_PLANAR:
    seq_printf(m, "Planar #%c", c[1]);
    break;
    case LOC_CPU:
    seq_printf(m, "CPU #%c", c[1]);
    break;
    case LOC_FAN:
    seq_printf(m, "Fan #%c", c[1]);
    break;
    case LOC_RACKMOUNTED:
    seq_printf(m, "Rack #%c", c[1]);
    break;
    case LOC_VOLTAGE:
    seq_printf(m, "Voltage #%c", c[1]);
    break;
    case LOC_LCD:
    seq_printf(m, "LCD #%c", c[1]);
    break;
    case '.':
    seq_printf(m, "- %c", c[1]);
    break;
    default:
    seq_printf(m, "Unknown location");
    break;
    }
    }
// ******************************************************************
//
// Format:
// ${LETTER}${NUMBER}[[-/]${LETTER}${NUMBER} [ ... ] ]
// the '.' may be an abbreviation
//
#[no_mangle]
unsafe extern "C" fn check_location_string(m: *mut seq_file, c: *const c_char) {
    static void check_location_string(struct seq_file *m, const char *c)
    {
    while (*c) {
    if (isalpha(*c) || *c == '.')
    check_location(m, c);
#[no_mangle]
pub unsafe extern "C" fn if('-': *mut *mut *mut c == '/' || c ==) -> else {
    else if (*c == '/' || *c == '-')
    seq_printf(m, " at ");
    c++;
    }
    }
// ******************************************************************
    static void get_location_code(struct seq_file *m, struct individual_sensor *s,
    const char *loc)
    {
    if (!loc || !*loc) {
    seq_printf(m, "---");/* does not have a location */
    } else {
    check_location_string(m, loc);
    }
    seq_putc(m, ' ');
    }
// ******************************************************************
// INDICATORS - Tone Frequency
// ******************************************************************
    static ssize_t ppc_rtas_tone_freq_write(struct file *file,
    const char __user *buf, size_t count, loff_t *ppos)
    {
    u64 freq;
    let mut error: c_int = parse_number(buf, count, &freq);
    if (error)
    return error;
    rtas_tone_frequency = freq; /* save it for later */
    error = rtas_call(rtas_function_token(RTAS_FN_SET_INDICATOR), 3, 1, core::ptr::null_mut(),
    TONE_FREQUENCY, 0, freq);
    if (error)
    printk(KERN_WARNING "error: setting tone frequency returned: %s\n",
    ppc_rtas_process_error(error));
    return count;
    }
// ******************************************************************
#[no_mangle]
unsafe extern "C" fn ppc_rtas_tone_freq_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ppc_rtas_tone_freq_show(struct seq_file *m, void *v)
    {
    seq_printf(m, "%lu\n", rtas_tone_frequency);
    return 0;
    }
// ******************************************************************
// INDICATORS - Tone Volume
// ******************************************************************
    static ssize_t ppc_rtas_tone_volume_write(struct file *file,
    const char __user *buf, size_t count, loff_t *ppos)
    {
    u64 volume;
    let mut error: c_int = parse_number(buf, count, &volume);
    if (error)
    return error;
    if (volume > 100)
    volume = 100;
    rtas_tone_volume = volume; /* save it for later */
    error = rtas_call(rtas_function_token(RTAS_FN_SET_INDICATOR), 3, 1, core::ptr::null_mut(),
    TONE_VOLUME, 0, volume);
    if (error)
    printk(KERN_WARNING "error: setting tone volume returned: %s\n",
    ppc_rtas_process_error(error));
    return count;
    }
// ******************************************************************
#[no_mangle]
unsafe extern "C" fn ppc_rtas_tone_volume_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ppc_rtas_tone_volume_show(struct seq_file *m, void *v)
    {
    seq_printf(m, "%lu\n", rtas_tone_volume);
    return 0;
    }
//
// ppc_rtas_rmo_buf_show() - Describe RTAS-addressable region for user space.
// @m: seq_file output target.
// @v: Unused.
//
// Base + size description of a range of RTAS-addressable memory set
// aside for user space to use as work area(s) for certain RTAS
// functions. User space accesses this region via /dev/mem. Apart from
// security policies, the kernel does not arbitrate or serialize
// access to this region, and user space must ensure that concurrent
// users do not interfere with each other.
//
#[no_mangle]
unsafe extern "C" fn ppc_rtas_rmo_buf_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ppc_rtas_rmo_buf_show(struct seq_file *m, void *v)
    {
    seq_printf(m, "%016lx %x\n", rtas_rmo_buf, RTAS_USER_REGION_SIZE);
    return 0;
    }

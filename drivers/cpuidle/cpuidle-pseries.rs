//! Automatically rewritten from C to Rust
//! Source: drivers/cpuidle/cpuidle-pseries.c
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
// cpuidle-pseries - idle state cpuidle driver.
// Adapted from drivers/idle/intel_idle.c and
// drivers/acpi/processor_idle.c
//

    static struct cpuidle_driver pseries_idle_driver = {
    .name             = "pseries_idle",
    .owner            = THIS_MODULE,
    };
    static int max_idle_state __read_mostly;
    static struct cpuidle_state *cpuidle_state_table __read_mostly;
    static u64 snooze_timeout __read_mostly;
    static bool snooze_timeout_en __read_mostly;
    static __cpuidle
    int snooze_loop(struct cpuidle_device *dev, struct cpuidle_driver *drv,
    int index)
    {
    u64 snooze_exit_time;
    set_thread_flag(TIF_POLLING_NRFLAG);
    pseries_idle_prolog();
    raw_local_irq_enable();
    snooze_exit_time = get_tb() + snooze_timeout;
    dev.poll_time_limit = false;
    while (!need_resched()) {
    HMT_low();
    HMT_very_low();
    if (likely(snooze_timeout_en) && get_tb() > snooze_exit_time) {
//
// Task has not woken up but we are exiting the polling
// loop anyway. Require a barrier after polling is
// cleared to order subsequent test of need_resched().
//
    dev.poll_time_limit = true;
    clear_thread_flag(TIF_POLLING_NRFLAG);
    smp_mb();
    break;
    }
    }
    HMT_medium();
// Avoid double clear when breaking
    if (!dev.poll_time_limit)
    clear_thread_flag(TIF_POLLING_NRFLAG);
    raw_local_irq_disable();
    pseries_idle_epilog();
    return index;
    }
#[no_mangle]
unsafe extern "C" fn check_and_cede_processor() -> __cpuidle void {
    static __cpuidle void check_and_cede_processor(void)
    {
//
// Ensure our interrupt state is properly tracked,
// also checks if no interrupt has occurred while we
// were soft-disabled
//
    if (prep_irq_for_idle()) {
    cede_processor();

// Ensure that H_CEDE returns with IRQs on
    if (WARN_ON(!(mfmsr() & MSR_EE)))
    __hard_irq_enable();

    }
    }
//
// XCEDE: Extended CEDE states discovered through the
// "ibm,get-systems-parameter" RTAS call with the token
// CEDE_LATENCY_TOKEN
//
// Section 7.3.16 System Parameters Option of PAPR version 2.8.1 has a
// table with all the parameters to ibm,get-system-parameters.
// CEDE_LATENCY_TOKEN corresponds to the token value for Cede Latency
// Settings Information.
//
pub const CEDE_LATENCY_TOKEN: c_int = 45;
//
// If the platform supports the cede latency settings information system
// parameter it must provide the following information in the NULL terminated
// parameter string:
//
// a. The first byte is the length “N” of each cede latency setting record minus
// one (zero indicates a length of 1 byte).
//
// b. For each supported cede latency setting a cede latency setting record
// consisting of the first “N” bytes as per the following table.
//
// -----------------------------
// | Field           | Field   |
// | Name            | Length  |
// -----------------------------
// | Cede Latency    | 1 Byte  |
// | Specifier Value |         |
// -----------------------------
// | Maximum wakeup  |         |
// | latency in      | 8 Bytes |
// | tb-ticks        |         |
// -----------------------------
// | Responsive to   |         |
// | external        | 1 Byte  |
// | interrupts      |         |
// -----------------------------
//
// This version has cede latency record size = 10.
//
// The structure xcede_latency_payload represents a) and b) with
// xcede_latency_record representing the table in b).
//
// xcede_latency_parameter is what gets returned by
// ibm,get-systems-parameter RTAS call when made with
// CEDE_LATENCY_TOKEN.
//
// These structures are only used to represent the data obtained by the RTAS
// call. The data is in big-endian.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xcede_latency_record {
    pub hint: u8,
    pub latency_ticks: __be64,
    pub wake_on_irqs: u8,
    pub __packed: },
// Make space for 16 records, which "should be enough".
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xcede_latency_payload {
    pub record_size: u8,
    pub records: [xcede_latency_record; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xcede_latency_parameter {
    pub payload_size: __be16,
    pub payload: xcede_latency_payload,
    pub null_char: u8,
    pub __packed: },
    pub nr_xcede_records: static unsigned int,
    pub __initdata: static struct xcede_latency_parameter xcede_latency_parameter,
#[no_mangle]
unsafe extern "C" fn parse_cede_parameters() -> int __init {
    static int __init parse_cede_parameters(void)
    {
    pub payload: *mut xcede_latency_payload,
    pub total_xcede_records_size: u32,
    pub xcede_record_size: u8,
    pub payload_size: u16,
    pub i: int ret,,
    ret = rtas_call(rtas_token("ibm,get-system-parameter"), 3, 1,
    core::ptr::null_mut(), CEDE_LATENCY_TOKEN, __pa(&xcede_latency_parameter),
    if (ret) {
    pub CEDE_LATENCY_TOKEN\n"): pr_err("xcede: Error parsing,
    pub ret: return,
    }
    pub be16_to_cpu(xcede_latency_parameter.payload_size): payload_size =,
    pub &xcede_latency_parameter.payload: payload =,
    pub 1: xcede_record_size = payload->record_size +,
    if (xcede_record_size != sizeof(struct xcede_latency_record)) {
    pr_err("xcede: Expected record-size %lu. Observed size %u.\n",
    pub xcede_record_size): sizeof(struct xcede_latency_record),,
    pub -EINVAL: return,
    }
    pub xcede_record_size): pr_info("xcede: xcede_record_size = %d\n",,
//
// Since the payload_size includes the last NULL byte and the
// xcede_record_size, the remaining bytes correspond to array of all
// cede_latency settings.
//
    pub 2: total_xcede_records_size = payload_size -,
    pub xcede_record_size: nr_xcede_records = total_xcede_records_size /,
    pub {: for (i = 0; i < nr_xcede_records; i++),
    pub &payload->records[i]: *mut *mut xcede_latency_record record =,
    pub be64_to_cpu(record->latency_ticks): u64 latency_ticks =,
    pub record->wake_on_irqs: u8 wake_on_irqs =,
    pub record->hint: u8 hint =,
    pr_info("xcede: Record %d : hint = %u, latency = 0x%llx tb ticks, Wake-on-irq = %u\n",
    pub wake_on_irqs): i, hint, latency_ticks,,
    }
    pub 0: return,
    }
    pub cede_latency_hint: [static u8; NR_DEDICATED_STATES],
    static __cpuidle
    int dedicated_cede_loop(struct cpuidle_device *dev, struct cpuidle_driver *drv,
    int index)
    {
    pub old_latency_hint: u8,
    pub 1: get_lppaca()->donate_dedicated_cpu =,
    pub get_lppaca()->cede_latency_hint: old_latency_hint =,
    pub cede_latency_hint: [get_lppaca()->cede_latency_hint =; index],
    pub 0: get_lppaca()->donate_dedicated_cpu =,
    pub old_latency_hint: get_lppaca()->cede_latency_hint =,
    pub index: return,
    }
    static __cpuidle
    int shared_cede_loop(struct cpuidle_device *dev, struct cpuidle_driver *drv,
    int index)
    {
//
// Yield the processor to the hypervisor.  We return if
// an external interrupt occurs (which are driven prior
// to returning here) or if a prod occurs from another
// processor. When returning here, external interrupts
// are enabled.
//
    pub index: return,
    }
//
// States for dedicated partition case.
//
    static struct cpuidle_state dedicated_states[NR_DEDICATED_STATES] = {
    { /* Snooze */
    .name = "snooze",
    .desc = "snooze",
    .exit_latency = 0,
    .target_residency = 0,
    .enter = &snooze_loop,
    .flags = CPUIDLE_FLAG_POLLING },
    { /* CEDE */
    .name = "CEDE",
    .desc = "CEDE",
    .exit_latency = 10,
    .target_residency = 100,
    .enter = &dedicated_cede_loop },
}

//
// States for shared partition case.
//
    static struct cpuidle_state shared_states[] = {
    { /* Snooze */
    .name = "snooze",
    .desc = "snooze",
    .exit_latency = 0,
    .target_residency = 0,
    .enter = &snooze_loop,
    .flags = CPUIDLE_FLAG_POLLING },
    { /* Shared Cede */
    .name = "Shared Cede",
    .desc = "Shared Cede",
    .exit_latency = 10,
    .target_residency = 100,
    .enter = &shared_cede_loop },
    };
#[no_mangle]
unsafe extern "C" fn pseries_cpuidle_cpu_online(cpu: c_uint) -> c_int {
    static int pseries_cpuidle_cpu_online(unsigned int cpu)
    {
    struct cpuidle_device *dev = per_cpu(cpuidle_devices, cpu);
    if (dev && cpuidle_get_driver()) {
    cpuidle_pause_and_lock();
    cpuidle_enable_device(dev);
    cpuidle_resume_and_unlock();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pseries_cpuidle_cpu_dead(cpu: c_uint) -> c_int {
    static int pseries_cpuidle_cpu_dead(unsigned int cpu)
    {
    struct cpuidle_device *dev = per_cpu(cpuidle_devices, cpu);
    if (dev && cpuidle_get_driver()) {
    cpuidle_pause_and_lock();
    cpuidle_disable_device(dev);
    cpuidle_resume_and_unlock();
    }
    return 0;
    }
//
// pseries_cpuidle_driver_init()
//
#[no_mangle]
unsafe extern "C" fn pseries_cpuidle_driver_init() -> c_int {
    static int pseries_cpuidle_driver_init(void)
    {
    int idle_state;
    struct cpuidle_driver *drv = &pseries_idle_driver;
    drv.state_count = 0;
    for (idle_state = 0; idle_state < max_idle_state; ++idle_state) {
// Is the state not enabled?
    if (cpuidle_state_table[idle_state].enter == core::ptr::null_mut())
    continue;
    drv.states[drv.state_count] =	/* structure copy */
    cpuidle_state_table[idle_state];
    drv.state_count += 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fixup_cede0_latency() -> void __init {
    static void __init fixup_cede0_latency(void)
    {
    struct xcede_latency_payload *payload;
    let mut min_xcede_latency_us: u64 = UINT_MAX;
    int i;
    if (parse_cede_parameters())
    return;
    pr_info("cpuidle: Skipping the %d Extended CEDE idle states\n",
    nr_xcede_records);
    payload = &xcede_latency_parameter.payload;
//
// The CEDE idle state maps to CEDE(0). While the hypervisor
// does not advertise CEDE(0) exit latency values, it does
// advertise the latency values of the extended CEDE states.
// We use the lowest advertised exit latency value as a proxy
// for the exit latency of CEDE(0).
//
    for (i = 0; i < nr_xcede_records; i++) {
    struct xcede_latency_record *record = &payload.records[i];
    let mut hint: u8 = record.hint;
    let mut latency_tb: u64 = be64_to_cpu(record.latency_ticks);
    let mut latency_us: u64 = DIV_ROUND_UP_ULL(tb_to_ns(latency_tb), NSEC_PER_USEC);
//
// We expect the exit latency of an extended CEDE
// state to be non-zero, it to since it takes at least
// a few nanoseconds to wakeup the idle CPU and
// dispatch the virtual processor into the Linux
// Guest.
//
// So we consider only non-zero value for performing
// the fixup of CEDE(0) latency.
//
    if (latency_us == 0) {
    pr_warn("cpuidle: Skipping xcede record %d [hint=%d]. Exit latency = 0us\n",
    i, hint);
    continue;
    }
    if (latency_us < min_xcede_latency_us)
    min_xcede_latency_us = latency_us;
    }
    if (min_xcede_latency_us != UINT_MAX) {
    dedicated_states[1].exit_latency = min_xcede_latency_us;
    dedicated_states[1].target_residency = 10 * (min_xcede_latency_us);
    pr_info("cpuidle: Fixed up CEDE exit latency to %llu us\n",
    min_xcede_latency_us);
    }
    }
//
// pseries_idle_probe()
// Choose state table for shared versus dedicated partition
//
#[no_mangle]
unsafe extern "C" fn pseries_idle_probe() -> int __init {
    static int __init pseries_idle_probe(void)
    {
    if (cpuidle_disable != IDLE_NO_OVERRIDE)
    return -ENODEV;
    if (firmware_has_feature(FW_FEATURE_SPLPAR)) {
    if (lppaca_shared_proc()) {
    cpuidle_state_table = shared_states;
    max_idle_state = ARRAY_SIZE(shared_states);
    } else {
//
// Use firmware provided latency values
// starting with POWER10 platforms. In the
// case that we are running on a POWER10
// platform but in an earlier compat mode, we
// can still use the firmware provided values.
//
// However, on platforms prior to POWER10, we
// cannot rely on the accuracy of the firmware
// provided latency values. On such platforms,
// go with the conservative default estimate
// of 10us.
//
    if (cpu_has_feature(CPU_FTR_ARCH_31) || pvr_version_is(PVR_POWER10))
    fixup_cede0_latency();
    cpuidle_state_table = dedicated_states;
    max_idle_state = NR_DEDICATED_STATES;
    }
    } else
    return -ENODEV;
    if (max_idle_state > 1) {
    snooze_timeout_en = true;
    snooze_timeout = cpuidle_state_table[1].target_residency *
    tb_ticks_per_usec;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pseries_processor_idle_init() -> int __init {
    static int __init pseries_processor_idle_init(void)
    {
    int retval;
    retval = pseries_idle_probe();
    if (retval)
    return retval;
    pseries_cpuidle_driver_init();
    retval = cpuidle_register(&pseries_idle_driver, core::ptr::null_mut());
    if (retval) {
    printk(KERN_DEBUG "Registration of pseries driver failed.\n");
    return retval;
    }
    retval = cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN,
    "cpuidle/pseries:online",
    pseries_cpuidle_cpu_online, core::ptr::null_mut());
    WARN_ON(retval < 0);
    retval = cpuhp_setup_state_nocalls(CPUHP_CPUIDLE_DEAD,
    "cpuidle/pseries:DEAD", core::ptr::null_mut(),
    pseries_cpuidle_cpu_dead);
    WARN_ON(retval < 0);
    printk(KERN_DEBUG "pseries_idle_driver registered\n");
    return 0;
    }
    device_initcall(pseries_processor_idle_init);

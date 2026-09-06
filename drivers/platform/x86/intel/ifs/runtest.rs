//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/ifs/runtest.c
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
// Copyright(c) 2022 Intel Corporation.

//
// Note all code and data in this file is protected by
// ifs_sem. On HT systems all threads on a core will
// execute together, but only the first thread on the
// core will update results of the test.
//
// Macro flag: #define CREATE_TRACE_POINTS

// Max retries on the same chunk
pub const MAX_IFS_RETRIES: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct run_params {
    pub ifsd: *mut ifs_data,
    pub activate: *mut union ifs_scan,
    pub status: union ifs_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbaf_run_params {
    pub ifsd: *mut ifs_data,
    pub retry_cnt: *mut c_int,
    pub activate: *mut union ifs_sbaf,
    pub status: union ifs_sbaf_status,
}

//
// Number of TSC cycles that a logical CPU will wait for the other
// logical CPU on the core in the WRMSR(ACTIVATE_SCAN).
//
pub const IFS_THREAD_WAIT: c_int = 100000;
    enum ifs_status_err_code {
    IFS_NO_ERROR				= 0,
    IFS_OTHER_THREAD_COULD_NOT_JOIN		= 1,
    IFS_INTERRUPTED_BEFORE_RENDEZVOUS	= 2,
    IFS_POWER_MGMT_INADEQUATE_FOR_SCAN	= 3,
    IFS_INVALID_CHUNK_RANGE			= 4,
    IFS_MISMATCH_ARGUMENTS_BETWEEN_THREADS	= 5,
    IFS_CORE_NOT_CAPABLE_CURRENTLY		= 6,
    IFS_UNASSIGNED_ERROR_CODE		= 7,
    IFS_EXCEED_NUMBER_OF_THREADS_CONCURRENT	= 8,
    IFS_INTERRUPTED_DURING_EXECUTION	= 9,
    IFS_UNASSIGNED_ERROR_CODE_0xA		= 0xA,
    IFS_CORRUPTED_CHUNK		= 0xB,
    };
    static const char * const scan_test_status[] = {
    [IFS_NO_ERROR] = "SCAN no error",
    [IFS_OTHER_THREAD_COULD_NOT_JOIN] = "Other thread could not join.",
    [IFS_INTERRUPTED_BEFORE_RENDEZVOUS] = "Interrupt occurred prior to SCAN coordination.",
    [IFS_POWER_MGMT_INADEQUATE_FOR_SCAN] =
    "Core Abort SCAN Response due to power management condition.",
    [IFS_INVALID_CHUNK_RANGE] = "Non valid chunks in the range",
    [IFS_MISMATCH_ARGUMENTS_BETWEEN_THREADS] = "Mismatch in arguments between threads T0/T1.",
    [IFS_CORE_NOT_CAPABLE_CURRENTLY] = "Core not capable of performing SCAN currently",
    [IFS_UNASSIGNED_ERROR_CODE] = "Unassigned error code 0x7",
    [IFS_EXCEED_NUMBER_OF_THREADS_CONCURRENT] =
    "Exceeded number of Logical Processors (LP) allowed to run Scan-At-Field concurrently",
    [IFS_INTERRUPTED_DURING_EXECUTION] = "Interrupt occurred prior to SCAN start",
    [IFS_UNASSIGNED_ERROR_CODE_0xA] = "Unassigned error code 0xA",
    [IFS_CORRUPTED_CHUNK] = "Scan operation aborted due to corrupted image. Try reloading",
    };
#[no_mangle]
unsafe extern "C" fn message_not_tested(dev: *mut device, cpu: c_int, status: union ifs_status) {
    static void message_not_tested(struct device *dev, int cpu, union ifs_status status)
    {
    struct ifs_data *ifsd = ifs_get_data(dev);
//
// control_error is set when the microcode runs into a problem
// loading the image from the reserved BIOS memory, or it has
// been corrupted. Reloading the image may fix this issue.
//
    if (status.control_error) {
    dev_warn(dev, "CPU(s) %*pbl: Scan controller error. Batch: %02x version: 0x%x\n",
    cpumask_pr_args(cpu_smt_mask(cpu)), ifsd.cur_batch, ifsd.loaded_version);
    return;
    }
    if (status.error_code < ARRAY_SIZE(scan_test_status)) {
    dev_info(dev, "CPU(s) %*pbl: SCAN operation did not start. %s\n",
    cpumask_pr_args(cpu_smt_mask(cpu)),
    scan_test_status[status.error_code]);
    } else if (status.error_code == IFS_SW_TIMEOUT) {
    dev_info(dev, "CPU(s) %*pbl: software timeout during scan\n",
    cpumask_pr_args(cpu_smt_mask(cpu)));
    } else if (status.error_code == IFS_SW_PARTIAL_COMPLETION) {
    dev_info(dev, "CPU(s) %*pbl: %s\n",
    cpumask_pr_args(cpu_smt_mask(cpu)),
    "Not all scan chunks were executed. Maximum forward progress retries exceeded");
    } else {
    dev_info(dev, "CPU(s) %*pbl: SCAN unknown status %llx\n",
    cpumask_pr_args(cpu_smt_mask(cpu)), status.data);
    }
    }
#[no_mangle]
unsafe extern "C" fn message_fail(dev: *mut device, cpu: c_int, status: union ifs_status) {
    static void message_fail(struct device *dev, int cpu, union ifs_status status)
    {
    struct ifs_data *ifsd = ifs_get_data(dev);
//
// signature_error is set when the output from the scan chains does not
// match the expected signature. This might be a transient problem (e.g.
// due to a bit flip from an alpha particle or neutron). If the problem
// repeats on a subsequent test, then it indicates an actual problem in
// the core being tested.
//
    if (status.signature_error) {
    dev_err(dev, "CPU(s) %*pbl: test signature incorrect. Batch: %02x version: 0x%x\n",
    cpumask_pr_args(cpu_smt_mask(cpu)), ifsd.cur_batch, ifsd.loaded_version);
    }
    }
#[no_mangle]
unsafe extern "C" fn can_restart(status: union ifs_status) -> bool {
    static bool can_restart(union ifs_status status)
    {
    let mut err_code: enum ifs_status_err_code = status.error_code;
// Signature for chunk is bad, or scan test failed
    if (status.signature_error || status.control_error)
    return false;
    switch (err_code) {
    case IFS_NO_ERROR:
    case IFS_OTHER_THREAD_COULD_NOT_JOIN:
    case IFS_INTERRUPTED_BEFORE_RENDEZVOUS:
    case IFS_POWER_MGMT_INADEQUATE_FOR_SCAN:
    case IFS_EXCEED_NUMBER_OF_THREADS_CONCURRENT:
    case IFS_INTERRUPTED_DURING_EXECUTION:
    return true;
    case IFS_INVALID_CHUNK_RANGE:
    case IFS_MISMATCH_ARGUMENTS_BETWEEN_THREADS:
    case IFS_CORE_NOT_CAPABLE_CURRENTLY:
    case IFS_UNASSIGNED_ERROR_CODE:
    case IFS_UNASSIGNED_ERROR_CODE_0xA:
    case IFS_CORRUPTED_CHUNK:
    break;
    }
    return false;
    }

    static atomic_t array_cpus_in;
    static atomic_t scan_cpus_in;
    static atomic_t sbaf_cpus_in;
//
// Simplified cpu sibling rendezvous loop based on microcode loader __wait_for_cpus()
//
#[no_mangle]
unsafe extern "C" fn wait_for_sibling_cpu(t: *mut core::sync::atomic::AtomicI32, timeout: c_longlong) {
    static void wait_for_sibling_cpu(atomic_t *t, long long timeout)
    {
    let mut cpu: c_int = smp_processor_id();
    const struct cpumask *smt_mask = cpu_smt_mask(cpu);
    let mut all_cpus: c_int = cpumask_weight(smt_mask);
    atomic_inc(t);
    while (atomic_read(t) < all_cpus) {
    if (timeout < SPINUNIT)
    return;
    ndelay(SPINUNIT);
    timeout -= SPINUNIT;
    touch_nmi_watchdog();
    }
    }
//
// Execute the scan. Called "simultaneously" on all threads of a core
// at high priority using the stop_cpus mechanism.
//
#[no_mangle]
unsafe extern "C" fn doscan(data: *mut c_void) -> c_int {
    static int doscan(void *data)
    {
    let mut cpu: c_int = smp_processor_id(), start, stop;
    struct run_params *params = data;
    union ifs_status status;
    struct ifs_data *ifsd;
    int first;
    ifsd = params.ifsd;
    if (ifsd.generation) {
    start = params.activate.gen2.start;
    stop = params.activate.gen2.stop;
    } else {
    start = params.activate.gen0.start;
    stop = params.activate.gen0.stop;
    }
// Only the first logical CPU on a core reports result
    first = cpumask_first(cpu_smt_mask(cpu));
    wait_for_sibling_cpu(&scan_cpus_in, NSEC_PER_SEC);
//
// This WRMSR will wait for other HT threads to also write
// to this MSR (at most for activate.delay cycles). Then it
// starts scan of each requested chunk. The core scan happens
// during the "execution" of the WRMSR. This instruction can
// take up to 200 milliseconds (in the case where all chunks
// are processed in a single pass) before it retires.
//
    wrmsrq(MSR_ACTIVATE_SCAN, params.activate.data);
    rdmsrq(MSR_SCAN_STATUS, status.data);
    trace_ifs_status(ifsd.cur_batch, start, stop, status.data);
// Pass back the result of the scan
    if (cpu == first)
    params.status = status;
    return 0;
    }
//
// Use stop_core_cpuslocked() to synchronize writing to MSR_ACTIVATE_SCAN
// on all threads of the core to be tested. Loop if necessary to complete
// run of all chunks. Include some defensive tests to make sure forward
// progress is made, and that the whole test completes in a reasonable time.
//
#[no_mangle]
unsafe extern "C" fn ifs_test_core(cpu: c_int, dev: *mut device) {
    static void ifs_test_core(int cpu, struct device *dev)
    {
    let mut status: union ifs_status = {};
    union ifs_scan activate;
    unsigned long timeout;
    struct ifs_data *ifsd;
    int to_start, to_stop;
    int status_chunk;
    struct run_params params;
    int retries;
    ifsd = ifs_get_data(dev);
    activate.gen0.rsvd = 0;
    activate.delay = IFS_THREAD_WAIT;
    activate.sigmce = 0;
    to_start = 0;
    to_stop = ifsd.valid_chunks - 1;
    params.ifsd = ifs_get_data(dev);
    if (ifsd.generation) {
    activate.gen2.start = to_start;
    activate.gen2.stop = to_stop;
    } else {
    activate.gen0.start = to_start;
    activate.gen0.stop = to_stop;
    }
    timeout = jiffies + HZ / 2;
    retries = MAX_IFS_RETRIES;
    while (to_start <= to_stop) {
    if (time_after(jiffies, timeout)) {
    status.error_code = IFS_SW_TIMEOUT;
    break;
    }
    params.activate = &activate;
    atomic_set(&scan_cpus_in, 0);
    stop_core_cpuslocked(cpu, doscan, &params);
    status = params.status;
// Some cases can be retried, give up for others
    if (!can_restart(status))
    break;
    status_chunk = ifsd.generation ? status.gen2.chunk_num : status.gen0.chunk_num;
    if (status_chunk == to_start) {
// Check for forward progress
    if (--retries == 0) {
    if (status.error_code == IFS_NO_ERROR)
    status.error_code = IFS_SW_PARTIAL_COMPLETION;
    break;
    }
    } else {
    retries = MAX_IFS_RETRIES;
    if (ifsd.generation)
    activate.gen2.start = status_chunk;
    else
    activate.gen0.start = status_chunk;
    to_start = status_chunk;
    }
    }
// Update status for this core
    ifsd.scan_details = status.data;
    if (status.signature_error) {
    ifsd.status = SCAN_TEST_FAIL;
    message_fail(dev, cpu, status);
    } else if (status.control_error || status.error_code) {
    ifsd.status = SCAN_NOT_TESTED;
    message_not_tested(dev, cpu, status);
    } else {
    ifsd.status = SCAN_TEST_PASS;
    }
    }
#[no_mangle]
unsafe extern "C" fn do_array_test(data: *mut c_void) -> c_int {
    static int do_array_test(void *data)
    {
    union ifs_array *command = data;
    let mut cpu: c_int = smp_processor_id();
    int first;
    wait_for_sibling_cpu(&array_cpus_in, NSEC_PER_SEC);
//
// Only one logical CPU on a core needs to trigger the Array test via MSR write.
//
    first = cpumask_first(cpu_smt_mask(cpu));
    if (cpu == first) {
    wrmsrq(MSR_ARRAY_BIST, command.data);
// Pass back the result of the test
    rdmsrq(MSR_ARRAY_BIST, command.data);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ifs_array_test_core(cpu: c_int, dev: *mut device) {
    static void ifs_array_test_core(int cpu, struct device *dev)
    {
    let mut command: union ifs_array = {};
    let mut timed_out: bool = false;
    struct ifs_data *ifsd;
    unsigned long timeout;
    ifsd = ifs_get_data(dev);
    command.array_bitmask = ~0U;
    timeout = jiffies + HZ / 2;
    do {
    if (time_after(jiffies, timeout)) {
    timed_out = true;
    break;
    }
    atomic_set(&array_cpus_in, 0);
    stop_core_cpuslocked(cpu, do_array_test, &command);
    if (command.ctrl_result)
    break;
    } while (command.array_bitmask);
    ifsd.scan_details = command.data;
    if (command.ctrl_result)
    ifsd.status = SCAN_TEST_FAIL;
#[no_mangle]
pub unsafe extern "C" fn if(command.array_bitmask: timed_out ||) -> else {
    else if (timed_out || command.array_bitmask)
    ifsd.status = SCAN_NOT_TESTED;
    else
    ifsd.status = SCAN_TEST_PASS;
    }
pub const ARRAY_GEN1_TEST_ALL_ARRAYS: c_uint = 0x0ULL;
pub const ARRAY_GEN1_STATUS_FAIL: c_uint = 0x1ULL;
#[no_mangle]
unsafe extern "C" fn do_array_test_gen1(status: *mut c_void) -> c_int {
    static int do_array_test_gen1(void *status)
    {
    let mut cpu: c_int = smp_processor_id();
    int first;
    first = cpumask_first(cpu_smt_mask(cpu));
    if (cpu == first) {
    wrmsrq(MSR_ARRAY_TRIGGER, ARRAY_GEN1_TEST_ALL_ARRAYS);
    rdmsrq(MSR_ARRAY_STATUS, *((u64 *)status));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ifs_array_test_gen1(cpu: c_int, dev: *mut device) {
    static void ifs_array_test_gen1(int cpu, struct device *dev)
    {
    struct ifs_data *ifsd = ifs_get_data(dev);
    let mut status: u64 = 0;
    stop_core_cpuslocked(cpu, do_array_test_gen1, &status);
    ifsd.scan_details = status;
    if (status & ARRAY_GEN1_STATUS_FAIL)
    ifsd.status = SCAN_TEST_FAIL;
    else
    ifsd.status = SCAN_TEST_PASS;
    }
pub const SBAF_STATUS_PASS: c_int = 0;
pub const SBAF_STATUS_SIGN_FAIL: c_int = 1;
pub const SBAF_STATUS_INTR: c_int = 2;
pub const SBAF_STATUS_TEST_FAIL: c_int = 3;
    enum sbaf_status_err_code {
    IFS_SBAF_NO_ERROR				= 0,
    IFS_SBAF_OTHER_THREAD_COULD_NOT_JOIN		= 1,
    IFS_SBAF_INTERRUPTED_BEFORE_RENDEZVOUS		= 2,
    IFS_SBAF_UNASSIGNED_ERROR_CODE3			= 3,
    IFS_SBAF_INVALID_BUNDLE_INDEX			= 4,
    IFS_SBAF_MISMATCH_ARGS_BETWEEN_THREADS		= 5,
    IFS_SBAF_CORE_NOT_CAPABLE_CURRENTLY		= 6,
    IFS_SBAF_UNASSIGNED_ERROR_CODE7			= 7,
    IFS_SBAF_EXCEED_NUMBER_OF_THREADS_CONCURRENT	= 8,
    IFS_SBAF_INTERRUPTED_DURING_EXECUTION		= 9,
    IFS_SBAF_INVALID_PROGRAM_INDEX			= 0xA,
    IFS_SBAF_CORRUPTED_CHUNK			= 0xB,
    IFS_SBAF_DID_NOT_START				= 0xC,
    };
    static const char * const sbaf_test_status[] = {
    [IFS_SBAF_NO_ERROR] = "SBAF no error",
    [IFS_SBAF_OTHER_THREAD_COULD_NOT_JOIN] = "Other thread could not join.",
    [IFS_SBAF_INTERRUPTED_BEFORE_RENDEZVOUS] = "Interrupt occurred prior to SBAF coordination.",
    [IFS_SBAF_UNASSIGNED_ERROR_CODE3] = "Unassigned error code 0x3",
    [IFS_SBAF_INVALID_BUNDLE_INDEX] = "Non-valid sbaf bundles. Reload test image",
    [IFS_SBAF_MISMATCH_ARGS_BETWEEN_THREADS] = "Mismatch in arguments between threads T0/T1.",
    [IFS_SBAF_CORE_NOT_CAPABLE_CURRENTLY] = "Core not capable of performing SBAF currently",
    [IFS_SBAF_UNASSIGNED_ERROR_CODE7] = "Unassigned error code 0x7",
    [IFS_SBAF_EXCEED_NUMBER_OF_THREADS_CONCURRENT] = "Exceeded number of Logical Processors (LP) allowed to run Scan-At-Field concurrently",
    [IFS_SBAF_INTERRUPTED_DURING_EXECUTION] = "Interrupt occurred prior to SBAF start",
    [IFS_SBAF_INVALID_PROGRAM_INDEX] = "SBAF program index not valid",
    [IFS_SBAF_CORRUPTED_CHUNK] = "SBAF operation aborted due to corrupted chunk",
    [IFS_SBAF_DID_NOT_START] = "SBAF operation did not start",
    };
#[no_mangle]
unsafe extern "C" fn sbaf_message_not_tested(dev: *mut device, cpu: c_int, status_data: u64) {
    static void sbaf_message_not_tested(struct device *dev, int cpu, u64 status_data)
    {
    let mut status: union ifs_sbaf_status = (union ifs_sbaf_status)status_data;
    if (status.error_code < ARRAY_SIZE(sbaf_test_status)) {
    dev_info(dev, "CPU(s) %*pbl: SBAF operation did not start. %s\n",
    cpumask_pr_args(cpu_smt_mask(cpu)),
    sbaf_test_status[status.error_code]);
    } else if (status.error_code == IFS_SW_TIMEOUT) {
    dev_info(dev, "CPU(s) %*pbl: software timeout during scan\n",
    cpumask_pr_args(cpu_smt_mask(cpu)));
    } else if (status.error_code == IFS_SW_PARTIAL_COMPLETION) {
    dev_info(dev, "CPU(s) %*pbl: %s\n",
    cpumask_pr_args(cpu_smt_mask(cpu)),
    "Not all SBAF bundles executed. Maximum forward progress retries exceeded");
    } else {
    dev_info(dev, "CPU(s) %*pbl: SBAF unknown status %llx\n",
    cpumask_pr_args(cpu_smt_mask(cpu)), status.data);
    }
    }
#[no_mangle]
unsafe extern "C" fn sbaf_message_fail(dev: *mut device, cpu: c_int, status: union ifs_sbaf_status) {
    static void sbaf_message_fail(struct device *dev, int cpu, union ifs_sbaf_status status)
    {
// Failed signature check is set when SBAF signature did not match the expected value
    if (status.sbaf_status == SBAF_STATUS_SIGN_FAIL) {
    dev_err(dev, "CPU(s) %*pbl: Failed signature check\n",
    cpumask_pr_args(cpu_smt_mask(cpu)));
    }
// Failed to reach end of test
    if (status.sbaf_status == SBAF_STATUS_TEST_FAIL) {
    dev_err(dev, "CPU(s) %*pbl: Failed to complete test\n",
    cpumask_pr_args(cpu_smt_mask(cpu)));
    }
    }
#[no_mangle]
unsafe extern "C" fn sbaf_bundle_completed(status: union ifs_sbaf_status) -> bool {
    static bool sbaf_bundle_completed(union ifs_sbaf_status status)
    {
    return !(status.sbaf_status || status.error_code);
    }
#[no_mangle]
unsafe extern "C" fn sbaf_can_restart(status: union ifs_sbaf_status) -> bool {
    static bool sbaf_can_restart(union ifs_sbaf_status status)
    {
    let mut err_code: enum sbaf_status_err_code = status.error_code;
// Signature for chunk is bad, or scan test failed
    if (status.sbaf_status == SBAF_STATUS_SIGN_FAIL ||
    status.sbaf_status == SBAF_STATUS_TEST_FAIL)
    return false;
    switch (err_code) {
    case IFS_SBAF_NO_ERROR:
    case IFS_SBAF_OTHER_THREAD_COULD_NOT_JOIN:
    case IFS_SBAF_INTERRUPTED_BEFORE_RENDEZVOUS:
    case IFS_SBAF_EXCEED_NUMBER_OF_THREADS_CONCURRENT:
    case IFS_SBAF_INTERRUPTED_DURING_EXECUTION:
    return true;
    case IFS_SBAF_UNASSIGNED_ERROR_CODE3:
    case IFS_SBAF_INVALID_BUNDLE_INDEX:
    case IFS_SBAF_MISMATCH_ARGS_BETWEEN_THREADS:
    case IFS_SBAF_CORE_NOT_CAPABLE_CURRENTLY:
    case IFS_SBAF_UNASSIGNED_ERROR_CODE7:
    case IFS_SBAF_INVALID_PROGRAM_INDEX:
    case IFS_SBAF_CORRUPTED_CHUNK:
    case IFS_SBAF_DID_NOT_START:
    break;
    }
    return false;
    }
//
// Execute the SBAF test. Called "simultaneously" on all threads of a core
// at high priority using the stop_cpus mechanism.
//
#[no_mangle]
unsafe extern "C" fn dosbaf(data: *mut c_void) -> c_int {
    static int dosbaf(void *data)
    {
    struct sbaf_run_params *run_params = data;
    let mut cpu: c_int = smp_processor_id();
    union ifs_sbaf_status status;
    struct ifs_data *ifsd;
    int first;
    ifsd = run_params.ifsd;
// Only the first logical CPU on a core reports result
    first = cpumask_first(cpu_smt_mask(cpu));
    wait_for_sibling_cpu(&sbaf_cpus_in, NSEC_PER_SEC);
//
// This WRMSR will wait for other HT threads to also write
// to this MSR (at most for activate.delay cycles). Then it
// starts scan of each requested bundle. The core test happens
// during the "execution" of the WRMSR.
//
    wrmsrq(MSR_ACTIVATE_SBAF, run_params.activate.data);
    rdmsrq(MSR_SBAF_STATUS, status.data);
    trace_ifs_sbaf(ifsd.cur_batch, *run_params.activate, status);
// Pass back the result of the test
    if (cpu == first)
    run_params.status = status;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ifs_sbaf_test_core(cpu: c_int, dev: *mut device) {
    static void ifs_sbaf_test_core(int cpu, struct device *dev)
    {
    struct sbaf_run_params run_params;
    let mut status: union ifs_sbaf_status = {};
    union ifs_sbaf activate;
    unsigned long timeout;
    struct ifs_data *ifsd;
    int stop_bundle;
    int retries;
    ifsd = ifs_get_data(dev);
    activate.data = 0;
    activate.delay = IFS_THREAD_WAIT;
    timeout = jiffies + 2 * HZ;
    retries = MAX_IFS_RETRIES;
    activate.bundle_idx = 0;
    stop_bundle = ifsd.max_bundle;
    while (activate.bundle_idx <= stop_bundle) {
    if (time_after(jiffies, timeout)) {
    status.error_code = IFS_SW_TIMEOUT;
    break;
    }
    atomic_set(&sbaf_cpus_in, 0);
    run_params.ifsd = ifsd;
    run_params.activate = &activate;
    run_params.retry_cnt = &retries;
    stop_core_cpuslocked(cpu, dosbaf, &run_params);
    status = run_params.status;
    if (sbaf_bundle_completed(status)) {
    activate.bundle_idx = status.bundle_idx + 1;
    activate.pgm_idx = 0;
    retries = MAX_IFS_RETRIES;
    continue;
    }
// Some cases can be retried, give up for others
    if (!sbaf_can_restart(status))
    break;
    if (status.pgm_idx == activate.pgm_idx) {
// If no progress retry
    if (--retries == 0) {
    if (status.error_code == IFS_NO_ERROR)
    status.error_code = IFS_SW_PARTIAL_COMPLETION;
    break;
    }
    } else {
// if some progress, more pgms remaining in bundle, reset retries
    retries = MAX_IFS_RETRIES;
    activate.bundle_idx = status.bundle_idx;
    activate.pgm_idx = status.pgm_idx;
    }
    }
// Update status for this core
    ifsd.scan_details = status.data;
    if (status.sbaf_status == SBAF_STATUS_SIGN_FAIL ||
    status.sbaf_status == SBAF_STATUS_TEST_FAIL) {
    ifsd.status = SCAN_TEST_FAIL;
    sbaf_message_fail(dev, cpu, status);
    } else if (status.error_code || status.sbaf_status == SBAF_STATUS_INTR ||
    (activate.bundle_idx < stop_bundle)) {
    ifsd.status = SCAN_NOT_TESTED;
    sbaf_message_not_tested(dev, cpu, status.data);
    } else {
    ifsd.status = SCAN_TEST_PASS;
    }
    }
//
// Initiate per core test. It wakes up work queue threads on the target cpu and
// its sibling cpu. Once all sibling threads wake up, the scan test gets executed and
// wait for all sibling threads to finish the scan test.
//
#[no_mangle]
pub unsafe extern "C" fn do_core_test(cpu: c_int, dev: *mut device) -> c_int {
    int do_core_test(int cpu, struct device *dev)
    {
    const struct ifs_test_caps *test = ifs_get_test_caps(dev);
    struct ifs_data *ifsd = ifs_get_data(dev);
    let mut ret: c_int = 0;
// Prevent CPUs from being taken offline during the scan test
    cpus_read_lock();
    if (!cpu_online(cpu)) {
    dev_info(dev, "cannot test on the offline cpu %d\n", cpu);
    ret = -EINVAL;
    goto out;
    }
    switch (test.test_num) {
    case IFS_TYPE_SAF:
    if (!ifsd.loaded)
    ret = -EPERM;
    else
    ifs_test_core(cpu, dev);
    break;
    case IFS_TYPE_ARRAY_BIST:
    if (ifsd.array_gen == ARRAY_GEN0)
    ifs_array_test_core(cpu, dev);
    else
    ifs_array_test_gen1(cpu, dev);
    break;
    case IFS_TYPE_SBAF:
    if (!ifsd.loaded)
    ret = -EPERM;
    else
    ifs_sbaf_test_core(cpu, dev);
    break;
    default:
    ret = -EINVAL;
    }
    out:
    cpus_read_unlock();
    return ret;
    }

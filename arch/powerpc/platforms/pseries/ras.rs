//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/ras.c
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
// Copyright (C) 2001 Dave Engebretsen IBM Corporation
//

    static unsigned char ras_log_buf[RTAS_ERROR_LOG_MAX];
    static DEFINE_SPINLOCK(ras_log_buf_lock);
    static int ras_check_exception_token;
pub const EPOW_SENSOR_TOKEN: c_int = 9;
pub const EPOW_SENSOR_INDEX: c_int = 0;
// EPOW events counter variable
    static int num_epow_events;
    static irqreturn_t ras_hotplug_interrupt(int irq, void *dev_id);
    static irqreturn_t ras_epow_interrupt(int irq, void *dev_id);
    static irqreturn_t ras_error_interrupt(int irq, void *dev_id);
// RTAS pseries MCE errorlog section.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pseries_mc_errorlog {
    pub fru_id: __be32,
    pub proc_id: __be32,
    pub error_type: u8,
//
// sub_err_type (1 byte). Bit fields depends on error_type
//
// MSB0
// |
// V
// 01234567
// XXXXXXXX
//
// For error_type == MC_ERROR_TYPE_UE
// XXXXXXXX
// X		1: Permanent or Transient UE.
// X		1: Effective address provided.
// X	1: Logical address provided.
// XX	2: Reserved.
// XXX	3: Type of UE error.
//
// For error_type == MC_ERROR_TYPE_SLB/ERAT/TLB
// XXXXXXXX
// X		1: Effective address provided.
// XXXXX	5: Reserved.
// XX	2: Type of SLB/ERAT/TLB error.
//
// For error_type == MC_ERROR_TYPE_CTRL_MEM_ACCESS
// XXXXXXXX
// X		1: Error causing address provided.
// XXX	3: Type of error.
// XXXX	4: Reserved.
//
    pub sub_err_type: u8,
    pub reserved_1: [u8; 6],
    pub effective_address: __be64,
    pub logical_address: __be64,
    pub __packed: },
// RTAS pseries MCE error types
pub const MC_ERROR_TYPE_UE: c_uint = 0x00;
pub const MC_ERROR_TYPE_SLB: c_uint = 0x01;
pub const MC_ERROR_TYPE_ERAT: c_uint = 0x02;
pub const MC_ERROR_TYPE_UNKNOWN: c_uint = 0x03;
pub const MC_ERROR_TYPE_TLB: c_uint = 0x04;
pub const MC_ERROR_TYPE_D_CACHE: c_uint = 0x05;
pub const MC_ERROR_TYPE_I_CACHE: c_uint = 0x07;
pub const MC_ERROR_TYPE_CTRL_MEM_ACCESS: c_uint = 0x08;
// RTAS pseries MCE error sub types
pub const MC_ERROR_UE_INDETERMINATE: c_int = 0;
pub const MC_ERROR_UE_IFETCH: c_int = 1;
pub const MC_ERROR_UE_PAGE_TABLE_WALK_IFETCH: c_int = 2;
pub const MC_ERROR_UE_LOAD_STORE: c_int = 3;
pub const MC_ERROR_UE_PAGE_TABLE_WALK_LOAD_STORE: c_int = 4;
pub const UE_EFFECTIVE_ADDR_PROVIDED: c_uint = 0x40;
pub const UE_LOGICAL_ADDR_PROVIDED: c_uint = 0x20;
pub const MC_EFFECTIVE_ADDR_PROVIDED: c_uint = 0x80;
pub const MC_ERROR_SLB_PARITY: c_int = 0;
pub const MC_ERROR_SLB_MULTIHIT: c_int = 1;
pub const MC_ERROR_SLB_INDETERMINATE: c_int = 2;
pub const MC_ERROR_ERAT_PARITY: c_int = 1;
pub const MC_ERROR_ERAT_MULTIHIT: c_int = 2;
pub const MC_ERROR_ERAT_INDETERMINATE: c_int = 3;
pub const MC_ERROR_TLB_PARITY: c_int = 1;
pub const MC_ERROR_TLB_MULTIHIT: c_int = 2;
pub const MC_ERROR_TLB_INDETERMINATE: c_int = 3;
pub const MC_ERROR_CTRL_MEM_ACCESS_PTABLE_WALK: c_int = 0;
pub const MC_ERROR_CTRL_MEM_ACCESS_OP_ACCESS: c_int = 1;
#[no_mangle]
pub unsafe extern "C" fn rtas_mc_error_sub_type(mlog: *const pseries_mc_errorlog) -> u8 {
    static inline u8 rtas_mc_error_sub_type(const struct pseries_mc_errorlog *mlog)
    {
    switch (mlog.error_type) {
    case	MC_ERROR_TYPE_UE:
    pub 0x07): return (mlog->sub_err_type &,
    case	MC_ERROR_TYPE_SLB:
    case	MC_ERROR_TYPE_ERAT:
    case	MC_ERROR_TYPE_TLB:
    pub 0x03): return (mlog->sub_err_type &,
    case	MC_ERROR_TYPE_CTRL_MEM_ACCESS:
    pub 4: return (mlog->sub_err_type & 0x70) >>,
    default:
    pub 0: return,
    }
    }
//
// Enable the hotplug interrupt late because processing them may touch other
// devices or systems (e.g. hugepages) that have not been initialized at the
// subsys stage.
//
#[no_mangle]
unsafe extern "C" fn init_ras_hotplug_IRQ() -> int __init {
    static int __init init_ras_hotplug_IRQ(void)
    {
    pub np: *mut device_node,
// Hotplug Events
    pub of_find_node_by_path("/event-sources/hot-plug-events"): np =,
    if (np != core::ptr::null_mut()) {
    if (dlpar_workqueue_init() == 0)
    request_event_sources_irqs(np, ras_hotplug_interrupt,
    }
    pub 0: return,
    }
    pub init_ras_hotplug_IRQ): machine_late_initcall(pseries,,
//
// Initialize handlers for the set of interrupts caused by hardware errors
// and power system events.
//
#[no_mangle]
unsafe extern "C" fn init_ras_IRQ() -> int __init {
    static int __init init_ras_IRQ(void)
    {
    pub np: *mut device_node,
    pub rtas_function_token(RTAS_FN_CHECK_EXCEPTION): ras_check_exception_token =,
// Internal Errors
    pub of_find_node_by_path("/event-sources/internal-errors"): np =,
    if (np != core::ptr::null_mut()) {
    request_event_sources_irqs(np, ras_error_interrupt,
    }
// EPOW Events
    pub of_find_node_by_path("/event-sources/epow-events"): np =,
    if (np != core::ptr::null_mut()) {
    pub "RAS_EPOW"): request_event_sources_irqs(np, ras_epow_interrupt,,
    }
    pub 0: return,
    }
    pub init_ras_IRQ): machine_subsys_initcall(pseries,,
pub const EPOW_SHUTDOWN_NORMAL: c_int = 1;
pub const EPOW_SHUTDOWN_ON_UPS: c_int = 2;
pub const EPOW_SHUTDOWN_LOSS_OF_CRITICAL_FUNCTIONS: c_int = 3;
pub const EPOW_SHUTDOWN_AMBIENT_TEMPERATURE_TOO_HIGH: c_int = 4;
#[no_mangle]
unsafe extern "C" fn handle_system_shutdown(event_modifier: c_char) {
    static void handle_system_shutdown(char event_modifier)
    {
    switch (event_modifier) {
    case EPOW_SHUTDOWN_NORMAL:
    pub requested\n"): pr_emerg("Power off,
    case EPOW_SHUTDOWN_ON_UPS:
    pr_emerg("Loss of system power detected. System is running on"
    pub details\n"): " UPS/battery. Check RTAS error log for,
    case EPOW_SHUTDOWN_LOSS_OF_CRITICAL_FUNCTIONS:
    pr_emerg("Loss of system critical functions detected. Check"
    pub details\n"): " RTAS error log for,
    case EPOW_SHUTDOWN_AMBIENT_TEMPERATURE_TOO_HIGH:
    pr_emerg("High ambient temperature detected. Check RTAS"
    pub details\n"): " error log for,
    default:
    pr_err("Unknown power/cooling shutdown event (modifier = %d)\n",
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct epow_errorlog {
    pub sensor_value: c_uchar,
    pub event_modifier: c_uchar,
    pub extended_modifier: c_uchar,
    pub reserved: c_uchar,
    pub platform_reason: c_uchar,
}

pub const EPOW_RESET: c_int = 0;
pub const EPOW_WARN_COOLING: c_int = 1;
pub const EPOW_WARN_POWER: c_int = 2;
pub const EPOW_SYSTEM_SHUTDOWN: c_int = 3;
pub const EPOW_SYSTEM_HALT: c_int = 4;
pub const EPOW_MAIN_ENCLOSURE: c_int = 5;
pub const EPOW_POWER_OFF: c_int = 7;
#[no_mangle]
unsafe extern "C" fn rtas_parse_epow_errlog(log: *mut rtas_error_log) {
    static void rtas_parse_epow_errlog(struct rtas_error_log *log)
    {
    struct pseries_errorlog *pseries_log;
    struct epow_errorlog *epow_log;
    char action_code;
    char modifier;
    pseries_log = get_pseries_errorlog(log, PSERIES_ELOG_SECT_ID_EPOW);
    if (pseries_log == core::ptr::null_mut())
    return;
    epow_log = (struct epow_errorlog *)pseries_log.data;
    action_code = epow_log.sensor_value & 0xF;	/* bottom 4 bits */
    modifier = epow_log.event_modifier & 0xF;	/* bottom 4 bits */
    switch (action_code) {
    case EPOW_RESET:
    if (num_epow_events) {
    pr_info("Non critical power/cooling issue cleared\n");
    num_epow_events--;
    }
    break;
    case EPOW_WARN_COOLING:
    pr_info("Non-critical cooling issue detected. Check RTAS error"
    " log for details\n");
    break;
    case EPOW_WARN_POWER:
    pr_info("Non-critical power issue detected. Check RTAS error"
    " log for details\n");
    break;
    case EPOW_SYSTEM_SHUTDOWN:
    handle_system_shutdown(modifier);
    break;
    case EPOW_SYSTEM_HALT:
    pr_emerg("Critical power/cooling issue detected. Check RTAS"
    " error log for details. Powering off.\n");
    orderly_poweroff(true);
    break;
    case EPOW_MAIN_ENCLOSURE:
    case EPOW_POWER_OFF:
    pr_emerg("System about to lose power. Check RTAS error log "
    " for details. Powering off immediately.\n");
    emergency_sync();
    kernel_power_off();
    break;
    default:
    pr_err("Unknown power/cooling event (action code  = %d)\n",
    action_code);
    }
// Increment epow events counter variable
    if (action_code != EPOW_RESET)
    num_epow_events++;
    }
#[no_mangle]
unsafe extern "C" fn ras_hotplug_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ras_hotplug_interrupt(int irq, void *dev_id)
    {
    struct pseries_errorlog *pseries_log;
    struct pseries_hp_errorlog *hp_elog;
    spin_lock(&ras_log_buf_lock);
    rtas_call(ras_check_exception_token, 6, 1, core::ptr::null_mut(),
    RTAS_VECTOR_EXTERNAL_INTERRUPT, virq_to_hw(irq),
    RTAS_HOTPLUG_EVENTS, 0, __pa(&ras_log_buf),
    rtas_get_error_log_max());
    pseries_log = get_pseries_errorlog((struct rtas_error_log *)ras_log_buf,
    PSERIES_ELOG_SECT_ID_HOTPLUG);
    hp_elog = (struct pseries_hp_errorlog *)pseries_log.data;
//
// Since PCI hotplug is not currently supported on pseries, put PCI
// hotplug events on the ras_log_buf to be handled by rtas_errd.
//
    if (hp_elog.resource == PSERIES_HP_ELOG_RESOURCE_MEM ||
    hp_elog.resource == PSERIES_HP_ELOG_RESOURCE_CPU ||
    hp_elog.resource == PSERIES_HP_ELOG_RESOURCE_PMEM)
    queue_hotplug_event(hp_elog);
    else
    log_error(ras_log_buf, ERR_TYPE_RTAS_LOG, 0);
    spin_unlock(&ras_log_buf_lock);
    return IRQ_HANDLED;
    }
// Handle environmental and power warning (EPOW) interrupts.
#[no_mangle]
unsafe extern "C" fn ras_epow_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ras_epow_interrupt(int irq, void *dev_id)
    {
    int state;
    int critical;
    rtas_get_sensor_fast(EPOW_SENSOR_TOKEN, EPOW_SENSOR_INDEX, &state);
    if (state > 3)
    critical = 1;		/* Time Critical */
    else
    critical = 0;
    spin_lock(&ras_log_buf_lock);
    rtas_call(ras_check_exception_token, 6, 1, core::ptr::null_mut(), RTAS_VECTOR_EXTERNAL_INTERRUPT,
    virq_to_hw(irq), RTAS_EPOW_WARNING, critical, __pa(&ras_log_buf),
    rtas_get_error_log_max());
    log_error(ras_log_buf, ERR_TYPE_RTAS_LOG, 0);
    rtas_parse_epow_errlog((struct rtas_error_log *)ras_log_buf);
    spin_unlock(&ras_log_buf_lock);
    return IRQ_HANDLED;
    }
//
// Handle hardware error interrupts.
//
// RTAS check-exception is called to collect data on the exception.  If
// the error is deemed recoverable, we log a warning and return.
// For nonrecoverable errors, an error is logged and we stop all processing
// as quickly as possible in order to prevent propagation of the failure.
//
#[no_mangle]
unsafe extern "C" fn ras_error_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ras_error_interrupt(int irq, void *dev_id)
    {
    struct rtas_error_log *rtas_elog;
    int status;
    int fatal;
    spin_lock(&ras_log_buf_lock);
    status = rtas_call(ras_check_exception_token, 6, 1, core::ptr::null_mut(),
    RTAS_VECTOR_EXTERNAL_INTERRUPT,
    virq_to_hw(irq),
    RTAS_INTERNAL_ERROR, 1 /* Time Critical */,
    __pa(&ras_log_buf),
    rtas_get_error_log_max());
    rtas_elog = (struct rtas_error_log *)ras_log_buf;
    if (status == 0 &&
    rtas_error_severity(rtas_elog) >= RTAS_SEVERITY_ERROR_SYNC)
    fatal = 1;
    else
    fatal = 0;
// format and print the extended information
    log_error(ras_log_buf, ERR_TYPE_RTAS_LOG, fatal);
    if (fatal) {
    pr_emerg("Fatal hardware error detected. Check RTAS error"
    " log for details. Powering off immediately\n");
    emergency_sync();
    kernel_power_off();
    } else {
    pr_err("Recoverable hardware error detected\n");
    }
    spin_unlock(&ras_log_buf_lock);
    return IRQ_HANDLED;
    }
//
// Some versions of FWNMI place the buffer inside the 4kB page starting at
// 0x7000. Other versions place it inside the rtas buffer. We check both.
// Minimum size of the buffer is 16 bytes.
//

    ((((A) >= 0x7000) && ((A) <= 0x8000 - 16)) || \
    (((A) >= rtas.base) && ((A) <= (rtas.base + rtas.size - 16))))
    static inline struct rtas_error_log *fwnmi_get_errlog(void)
    {
    return (struct rtas_error_log *)local_paca.mce_data_buf;
    }
    static __be64 *fwnmi_get_savep(struct pt_regs *regs)
    {
    unsigned long savep_ra;
// Mask top two bits
    savep_ra = regs.gpr[3] & ~(0x3UL << 62);
    if (!VALID_FWNMI_BUFFER(savep_ra)) {
    printk(KERN_ERR "FWNMI: corrupt r3 0x%016lx\n", regs.gpr[3]);
    return core::ptr::null_mut();
    }
    return __va(savep_ra);
    }
//
// Get the error information for errors coming through the
// FWNMI vectors.  The pt_regs' r3 will be updated to reflect
// the actual r3 if possible, and a ptr to the error log entry
// will be returned if found.
//
// Use one buffer mce_data_buf per cpu to store RTAS error.
//
// The mce_data_buf does not have any locks or protection around it,
// if a second machine check comes in, or a system reset is done
// before we have logged the error, then we will get corruption in the
// error log.  This is preferable over holding off on calling
// ibm,nmi-interlock which would result in us checkstopping if a
// second machine check did come in.
//
    static struct rtas_error_log *fwnmi_get_errinfo(struct pt_regs *regs)
    {
    struct rtas_error_log *h;
    u32 extended_log_length;
    size_t len;
    __be64 *savep;
    savep = fwnmi_get_savep(regs);
    if (!savep)
    return core::ptr::null_mut();
    regs.gpr[3] = be64_to_cpu(savep[0]); /* restore original r3 */
    h = (struct rtas_error_log *)&savep[1];
    extended_log_length = rtas_error_extended(h) ? rtas_error_extended_log_length(h) : 0;
    len = struct_size(h, buffer, extended_log_length);
    len = min(len, RTAS_ERROR_LOG_MAX);
// Use the per cpu buffer from paca to store rtas error log
    memset(local_paca.mce_data_buf, 0, RTAS_ERROR_LOG_MAX);
    memcpy(local_paca.mce_data_buf, h, len);
    return (struct rtas_error_log *)local_paca.mce_data_buf;
    }
// Call this when done with the data returned by FWNMI_get_errinfo.
// It will release the saved data area for other CPUs in the
// partition to receive FWNMI errors.
//
#[no_mangle]
unsafe extern "C" fn fwnmi_release_errinfo() {
    static void fwnmi_release_errinfo(void)
    {
    struct rtas_args rtas_args;
    int ret;
//
// On pseries, the machine check stack is limited to under 4GB, so
// args can be on-stack.
//
    rtas_call_unlocked(&rtas_args, ibm_nmi_interlock_token, 0, 1, core::ptr::null_mut());
    ret = be32_to_cpu(rtas_args.rets[0]);
    if (ret != 0)
    printk(KERN_ERR "FWNMI: nmi-interlock failed: %d\n", ret);
    }
#[no_mangle]
pub unsafe extern "C" fn pSeries_system_reset_exception(regs: *mut pt_regs) -> c_int {
    int pSeries_system_reset_exception(struct pt_regs *regs)
    {

//
// Some firmware byteswaps SRR registers and gives incorrect SRR1. Try
// to detect the bad SRR1 pattern here. Flip the NIP back to correct
// endian for reporting purposes. Unfortunately the MSR can't be fixed,
// so clear it. It will be missing MSR_RI so we won't try to recover.
//
    if ((be64_to_cpu(regs.msr) &
    (MSR_LE|MSR_RI|MSR_DR|MSR_IR|MSR_ME|MSR_PR|
    MSR_ILE|MSR_HV|MSR_SF)) == (MSR_DR|MSR_SF)) {
    regs_set_return_ip(regs, be64_to_cpu((__be64)regs.nip));
    regs_set_return_msr(regs, 0);
    }

    if (fwnmi_active) {
    __be64 *savep;
//
// Firmware (PowerVM and KVM) saves r3 to a save area like
// machine check, which is not exactly what PAPR (2.9)
// suggests but there is no way to detect otherwise, so this
// is the interface now.
//
// System resets do not save any error log or require an
// "ibm,nmi-interlock" rtas call to release.
//
    savep = fwnmi_get_savep(regs);
    if (savep)
    regs.gpr[3] = be64_to_cpu(savep[0]); /* restore original r3 */
    }
    if (smp_handle_nmi_ipi(regs))
    return 1;
    return 0; /* need to perform reset */
    }
#[no_mangle]
unsafe extern "C" fn mce_handle_err_realmode(disposition: c_int, error_type: u8) -> c_int {
    static int mce_handle_err_realmode(int disposition, u8 error_type)
    {

    if (disposition == RTAS_DISP_NOT_RECOVERED) {
    switch (error_type) {
    case	MC_ERROR_TYPE_ERAT:
    flush_erat();
    disposition = RTAS_DISP_FULLY_RECOVERED;
    break;
    case	MC_ERROR_TYPE_SLB:

//
// Store the old slb content in paca before flushing.
// Print this when we go to virtual mode.
// There are chances that we may hit MCE again if there
// is a parity error on the SLB entry we trying to read
// for saving. Hence limit the slb saving to single
// level of recursion.
//
    if (local_paca.in_mce == 1)
    slb_save_contents(local_paca.mce_faulty_slbs);
    flush_and_reload_slb();
    disposition = RTAS_DISP_FULLY_RECOVERED;

    break;
    default:
    break;
    }
    } else if (disposition == RTAS_DISP_LIMITED_RECOVERY) {
// Platform corrected itself but could be degraded
    pr_err("MCE: limited recovery, system may be degraded\n");
    disposition = RTAS_DISP_FULLY_RECOVERED;
    }

    return disposition;
    }
    static int mce_handle_err_virtmode(struct pt_regs *regs,
    struct rtas_error_log *errp,
    struct pseries_mc_errorlog *mce_log,
    int disposition)
    {
    let mut mce_err: mce_error_info = { 0 };
    let mut initiator: c_int = rtas_error_initiator(errp);
    let mut severity: c_int = rtas_error_severity(errp);
    let mut eaddr: c_ulong = 0, paddr = 0;
    u8 error_type, err_sub_type;
    if (!mce_log)
    goto out;
    error_type = mce_log.error_type;
    err_sub_type = rtas_mc_error_sub_type(mce_log);
    if (initiator == RTAS_INITIATOR_UNKNOWN)
    mce_err.initiator = MCE_INITIATOR_UNKNOWN;
#[no_mangle]
pub unsafe extern "C" fn if(RTAS_INITIATOR_CPU: initiator ==) -> else {
    else if (initiator == RTAS_INITIATOR_CPU)
    mce_err.initiator = MCE_INITIATOR_CPU;
#[no_mangle]
pub unsafe extern "C" fn if(RTAS_INITIATOR_PCI: initiator ==) -> else {
    else if (initiator == RTAS_INITIATOR_PCI)
    mce_err.initiator = MCE_INITIATOR_PCI;
#[no_mangle]
pub unsafe extern "C" fn if(RTAS_INITIATOR_ISA: initiator ==) -> else {
    else if (initiator == RTAS_INITIATOR_ISA)
    mce_err.initiator = MCE_INITIATOR_ISA;
#[no_mangle]
pub unsafe extern "C" fn if(RTAS_INITIATOR_MEMORY: initiator ==) -> else {
    else if (initiator == RTAS_INITIATOR_MEMORY)
    mce_err.initiator = MCE_INITIATOR_MEMORY;
#[no_mangle]
pub unsafe extern "C" fn if(RTAS_INITIATOR_POWERMGM: initiator ==) -> else {
    else if (initiator == RTAS_INITIATOR_POWERMGM)
    mce_err.initiator = MCE_INITIATOR_POWERMGM;
    else
    mce_err.initiator = MCE_INITIATOR_UNKNOWN;
    if (severity == RTAS_SEVERITY_NO_ERROR)
    mce_err.severity = MCE_SEV_NO_ERROR;
#[no_mangle]
pub unsafe extern "C" fn if(RTAS_SEVERITY_EVENT: severity ==) -> else {
    else if (severity == RTAS_SEVERITY_EVENT)
    mce_err.severity = MCE_SEV_WARNING;
#[no_mangle]
pub unsafe extern "C" fn if(RTAS_SEVERITY_WARNING: severity ==) -> else {
    else if (severity == RTAS_SEVERITY_WARNING)
    mce_err.severity = MCE_SEV_WARNING;
#[no_mangle]
pub unsafe extern "C" fn if(RTAS_SEVERITY_ERROR_SYNC: severity ==) -> else {
    else if (severity == RTAS_SEVERITY_ERROR_SYNC)
    mce_err.severity = MCE_SEV_SEVERE;
#[no_mangle]
pub unsafe extern "C" fn if(RTAS_SEVERITY_ERROR: severity ==) -> else {
    else if (severity == RTAS_SEVERITY_ERROR)
    mce_err.severity = MCE_SEV_SEVERE;
    else
    mce_err.severity = MCE_SEV_FATAL;
    if (severity <= RTAS_SEVERITY_ERROR_SYNC)
    mce_err.sync_error = true;
    else
    mce_err.sync_error = false;
    mce_err.error_type = MCE_ERROR_TYPE_UNKNOWN;
    mce_err.error_class = MCE_ECLASS_UNKNOWN;
    switch (error_type) {
    case MC_ERROR_TYPE_UE:
    mce_err.error_type = MCE_ERROR_TYPE_UE;
    mce_common_process_ue(regs, &mce_err);
    if (mce_err.ignore_event)
    disposition = RTAS_DISP_FULLY_RECOVERED;
    switch (err_sub_type) {
    case MC_ERROR_UE_IFETCH:
    mce_err.u.ue_error_type = MCE_UE_ERROR_IFETCH;
    break;
    case MC_ERROR_UE_PAGE_TABLE_WALK_IFETCH:
    mce_err.u.ue_error_type = MCE_UE_ERROR_PAGE_TABLE_WALK_IFETCH;
    break;
    case MC_ERROR_UE_LOAD_STORE:
    mce_err.u.ue_error_type = MCE_UE_ERROR_LOAD_STORE;
    break;
    case MC_ERROR_UE_PAGE_TABLE_WALK_LOAD_STORE:
    mce_err.u.ue_error_type = MCE_UE_ERROR_PAGE_TABLE_WALK_LOAD_STORE;
    break;
    case MC_ERROR_UE_INDETERMINATE:
    default:
    mce_err.u.ue_error_type = MCE_UE_ERROR_INDETERMINATE;
    break;
    }
    if (mce_log.sub_err_type & UE_EFFECTIVE_ADDR_PROVIDED)
    eaddr = be64_to_cpu(mce_log.effective_address);
    if (mce_log.sub_err_type & UE_LOGICAL_ADDR_PROVIDED) {
    paddr = be64_to_cpu(mce_log.logical_address);
    } else if (mce_log.sub_err_type & UE_EFFECTIVE_ADDR_PROVIDED) {
    unsigned long pfn;
    pfn = addr_to_pfn(regs, eaddr);
    if (pfn != ULONG_MAX)
    paddr = pfn << PAGE_SHIFT;
    }
    break;
    case MC_ERROR_TYPE_SLB:
    mce_err.error_type = MCE_ERROR_TYPE_SLB;
    switch (err_sub_type) {
    case MC_ERROR_SLB_PARITY:
    mce_err.u.slb_error_type = MCE_SLB_ERROR_PARITY;
    break;
    case MC_ERROR_SLB_MULTIHIT:
    mce_err.u.slb_error_type = MCE_SLB_ERROR_MULTIHIT;
    break;
    case MC_ERROR_SLB_INDETERMINATE:
    default:
    mce_err.u.slb_error_type = MCE_SLB_ERROR_INDETERMINATE;
    break;
    }
    if (mce_log.sub_err_type & MC_EFFECTIVE_ADDR_PROVIDED)
    eaddr = be64_to_cpu(mce_log.effective_address);
    break;
    case MC_ERROR_TYPE_ERAT:
    mce_err.error_type = MCE_ERROR_TYPE_ERAT;
    switch (err_sub_type) {
    case MC_ERROR_ERAT_PARITY:
    mce_err.u.erat_error_type = MCE_ERAT_ERROR_PARITY;
    break;
    case MC_ERROR_ERAT_MULTIHIT:
    mce_err.u.erat_error_type = MCE_ERAT_ERROR_MULTIHIT;
    break;
    case MC_ERROR_ERAT_INDETERMINATE:
    default:
    mce_err.u.erat_error_type = MCE_ERAT_ERROR_INDETERMINATE;
    break;
    }
    if (mce_log.sub_err_type & MC_EFFECTIVE_ADDR_PROVIDED)
    eaddr = be64_to_cpu(mce_log.effective_address);
    break;
    case MC_ERROR_TYPE_TLB:
    mce_err.error_type = MCE_ERROR_TYPE_TLB;
    switch (err_sub_type) {
    case MC_ERROR_TLB_PARITY:
    mce_err.u.tlb_error_type = MCE_TLB_ERROR_PARITY;
    break;
    case MC_ERROR_TLB_MULTIHIT:
    mce_err.u.tlb_error_type = MCE_TLB_ERROR_MULTIHIT;
    break;
    case MC_ERROR_TLB_INDETERMINATE:
    default:
    mce_err.u.tlb_error_type = MCE_TLB_ERROR_INDETERMINATE;
    break;
    }
    if (mce_log.sub_err_type & MC_EFFECTIVE_ADDR_PROVIDED)
    eaddr = be64_to_cpu(mce_log.effective_address);
    break;
    case MC_ERROR_TYPE_D_CACHE:
    mce_err.error_type = MCE_ERROR_TYPE_DCACHE;
    break;
    case MC_ERROR_TYPE_I_CACHE:
    mce_err.error_type = MCE_ERROR_TYPE_ICACHE;
    break;
    case MC_ERROR_TYPE_CTRL_MEM_ACCESS:
    mce_err.error_type = MCE_ERROR_TYPE_RA;
    switch (err_sub_type) {
    case MC_ERROR_CTRL_MEM_ACCESS_PTABLE_WALK:
    mce_err.u.ra_error_type =
    MCE_RA_ERROR_PAGE_TABLE_WALK_LOAD_STORE_FOREIGN;
    break;
    case MC_ERROR_CTRL_MEM_ACCESS_OP_ACCESS:
    mce_err.u.ra_error_type =
    MCE_RA_ERROR_LOAD_STORE_FOREIGN;
    break;
    }
    if (mce_log.sub_err_type & MC_EFFECTIVE_ADDR_PROVIDED)
    eaddr = be64_to_cpu(mce_log.effective_address);
    break;
    case MC_ERROR_TYPE_UNKNOWN:
    default:
    mce_err.error_type = MCE_ERROR_TYPE_UNKNOWN;
    break;
    }
    out:
    save_mce_event(regs, disposition == RTAS_DISP_FULLY_RECOVERED,
    &mce_err, regs.nip, eaddr, paddr);
    return disposition;
    }
#[no_mangle]
unsafe extern "C" fn mce_handle_error(regs: *mut pt_regs, errp: *mut rtas_error_log) -> c_int {
    static int mce_handle_error(struct pt_regs *regs, struct rtas_error_log *errp)
    {
    struct pseries_errorlog *pseries_log;
    struct pseries_mc_errorlog *mce_log = core::ptr::null_mut();
    let mut disposition: c_int = rtas_error_disposition(errp);
    u8 error_type;
    if (!rtas_error_extended(errp))
    goto out;
    pseries_log = get_pseries_errorlog(errp, PSERIES_ELOG_SECT_ID_MCE);
    if (!pseries_log)
    goto out;
    mce_log = (struct pseries_mc_errorlog *)pseries_log.data;
    error_type = mce_log.error_type;
    disposition = mce_handle_err_realmode(disposition, error_type);
    out:
    disposition = mce_handle_err_virtmode(regs, errp, mce_log,
    disposition);
    return disposition;
    }
//
// Process MCE rtas errlog event.
//
#[no_mangle]
pub unsafe extern "C" fn pSeries_machine_check_log_err() {
    void pSeries_machine_check_log_err(void)
    {
    struct rtas_error_log *err;
    err = fwnmi_get_errlog();
    log_error((char *)err, ERR_TYPE_RTAS_LOG, 0);
    }
//
// See if we can recover from a machine check exception.
// This is only called on power4 (or above) and only via
// the Firmware Non-Maskable Interrupts (fwnmi) handler
// which provides the error analysis for us.
//
// Return 1 if corrected (or delivered a signal).
// Return 0 if there is nothing we can do.
//
#[no_mangle]
unsafe extern "C" fn recover_mce(regs: *mut pt_regs, evt: *mut machine_check_event) -> c_int {
    static int recover_mce(struct pt_regs *regs, struct machine_check_event *evt)
    {
    let mut recovered: c_int = 0;
    if (regs_is_unrecoverable(regs)) {
// If MSR_RI isn't set, we cannot recover
    pr_err("Machine check interrupt unrecoverable: MSR(RI=0)\n");
    recovered = 0;
    } else if (evt.disposition == MCE_DISPOSITION_RECOVERED) {
// Platform corrected itself
    recovered = 1;
    } else if (evt.severity == MCE_SEV_FATAL) {
// Fatal machine check
    pr_err("Machine check interrupt is fatal\n");
    recovered = 0;
    }
    if (!recovered && evt.sync_error) {
//
// Try to kill processes if we get a synchronous machine check
// (e.g., one caused by execution of this instruction). This
// will devolve into a panic if we try to kill init or are in
// an interrupt etc.
//
// TODO: Queue up this address for hwpoisioning later.
// TODO: This is not quite right for d-side machine
// checks ->nip is not necessarily the important
// address.
//
    if ((user_mode(regs))) {
    _exception(SIGBUS, regs, BUS_MCEERR_AR, regs.nip);
    recovered = 1;
    } else if (die_will_crash()) {
//
// die() would kill the kernel, so better to go via
// the platform reboot code that will log the
// machine check.
//
    recovered = 0;
    } else {
    die_mce("Machine check", regs, SIGBUS);
    recovered = 1;
    }
    }
    return recovered;
    }
//
// Handle a machine check.
//
// Note that on Power 4 and beyond Firmware Non-Maskable Interrupts (fwnmi)
// should be present.  If so the handler which called us tells us if the
// error was recovered (never true if RI=0).
//
// On hardware prior to Power 4 these exceptions were asynchronous which
// means we can't tell exactly where it occurred and so we can't recover.
//
#[no_mangle]
pub unsafe extern "C" fn pSeries_machine_check_exception(regs: *mut pt_regs) -> c_int {
    int pSeries_machine_check_exception(struct pt_regs *regs)
    {
    struct machine_check_event evt;
    if (!get_mce_event(&evt, MCE_EVENT_RELEASE))
    return 0;
// Print things out
    if (evt.version != MCE_V1) {
    pr_err("Machine Check Exception, Unknown event version %d !\n",
    evt.version);
    return 0;
    }
    machine_check_print_event_info(&evt, user_mode(regs), false);
    if (recover_mce(regs, &evt))
    return 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pseries_machine_check_realmode(regs: *mut pt_regs) -> c_long {
    long pseries_machine_check_realmode(struct pt_regs *regs)
    {
    struct rtas_error_log *errp;
    int disposition;
    if (fwnmi_active) {
    errp = fwnmi_get_errinfo(regs);
//
// Call to fwnmi_release_errinfo() in real mode causes kernel
// to panic. Hence we will call it as soon as we go into
// virtual mode.
//
    disposition = mce_handle_error(regs, errp);
    fwnmi_release_errinfo();
    if (disposition == RTAS_DISP_FULLY_RECOVERED)
    return 1;
    }
    return 0;
    }

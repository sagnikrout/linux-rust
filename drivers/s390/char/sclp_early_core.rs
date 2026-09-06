//! Automatically rewritten from C to Rust
//! Source: drivers/s390/char/sclp_early_core.c
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
// Copyright IBM Corp. 2015
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
//

    static struct read_info_sccb __bootdata(sclp_info_sccb);
    static int __bootdata(sclp_info_sccb_valid);
    char *__bootdata_preserved(sclp_early_sccb);
    let mut sclp_init_state: c_int = sclp_init_state_uninitialized;
//
// Used to keep track of the size of the event masks. Qemu until version 2.11
// only supports 4 and needs a workaround.
//
    bool sclp_mask_compat_mode;
#[no_mangle]
pub unsafe extern "C" fn sclp_early_wait_irq() {
    void sclp_early_wait_irq(void)
    {
    unsigned long psw_mask, addr;
    psw_t psw_ext_save, psw_wait;
    union ctlreg0 cr0, cr0_new;
    local_ctl_store(0, &cr0.reg);
    cr0_new.val = cr0.val & ~CR0_IRQ_SUBCLASS_MASK;
    cr0_new.lap = 0;
    cr0_new.sssm = 1;
    local_ctl_load(0, &cr0_new.reg);
    psw_ext_save = get_lowcore().external_new_psw;
    psw_mask = __extract_psw();
    get_lowcore().external_new_psw.mask = psw_mask;
    psw_wait.mask = psw_mask | PSW_MASK_EXT | PSW_MASK_WAIT;
    get_lowcore().ext_int_code = 0;
    do {
    asm volatile(
    "	larl	%[addr],0f\n"
    "	stg	%[addr],%[psw_wait_addr]\n"
    "	stg	%[addr],%[psw_ext_addr]\n"
    "	lpswe	%[psw_wait]\n"
    "0:"
    : [addr] "=&d" (addr),
    [psw_wait_addr] "=Q" (psw_wait.addr),
    [psw_ext_addr] "=Q" (get_lowcore().external_new_psw.addr)
    : [psw_wait] "Q" (psw_wait)
    : "cc", "memory");
    } while (get_lowcore().ext_int_code != EXT_IRQ_SERVICE_SIG);
    get_lowcore().external_new_psw = psw_ext_save;
    local_ctl_load(0, &cr0.reg);
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_early_cmd(cmd: sclp_cmdw_t, sccb: *mut c_void) -> c_int {
    int sclp_early_cmd(sclp_cmdw_t cmd, void *sccb)
    {
    unsigned long flags;
    int rc;
    flags = arch_local_irq_save();
    rc = sclp_service_call(cmd, sccb);
    if (rc)
    goto out;
    sclp_early_wait_irq();
    out:
    arch_local_irq_restore(flags);
    return rc;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct write_sccb {
    pub header: sccb_header,
    pub msg: msg_buf,
    pub __packed: },
// Output multi-line text using SCLP Message interface.
#[no_mangle]
unsafe extern "C" fn sclp_early_print_lm(str: *const c_char, len: c_uint) {
    static void sclp_early_print_lm(const char *str, unsigned int len)
    {
    pub ch: *mut *mut *mut unsigned char ptr, end,,
    pub offset: unsigned int count,,
    pub sccb: *mut write_sccb,
    pub msg: *mut msg_buf,
    pub mdb: *mut mdb,
    pub mto: *mut mto,
    pub go: *mut go,
    pub sclp_early_sccb: *mut *mut sccb = (struct write_sccb ),
    pub 1: *mut *mut end = (unsigned char ) sccb + EARLY_SCCB_SIZE -,
    pub sizeof(*sccb)): *mut memset(sccb, 0,,
    pub &sccb->msg.mdb.mto: *mut *mut ptr = (unsigned char ),
    pub 0: offset =,
    do {
    pub {: *mut *mut for (count = sizeof(mto); offset < len; count++),
    pub str: [ch =; offset++],
    if ((ch == 0x0a) || (ptr + count > end))
    pub _ascebc: [ptr[count] =; ch],
    }
    pub ptr: *mut *mut mto = (struct mto ),
    pub sizeof(*mto)): *mut memset(mto, 0,,
    pub count: mto->length =,
    pub 4: mto->type =,
    pub LNTPFLGS_ENDTEXT: mto->line_type_flags =,
    pub count: ptr +=,
    pub end)): *mut *mut } while ((offset < len) && (ptr + sizeof(mto) <=,
    pub sccb: *mut *mut len = ptr - (unsigned char ),
    pub header): sccb->header.length = len - offsetof(struct write_sccb,,
    pub &sccb->msg: msg =,
    pub EVTYP_MSG: msg->header.type =,
    pub msg.header): msg->header.length = len - offsetof(struct write_sccb,,
    pub &msg->mdb: mdb =,
    pub 1: mdb->header.type =,
    pub 0xD4C4C240: mdb->header.tag =,
    pub 1: mdb->header.revision_code =,
    pub msg.mdb.header): mdb->header.length = len - offsetof(struct write_sccb,,
    pub &mdb->go: go =,
    pub sizeof(*go): *mut go->length =,
    pub 1: go->type =,
    pub sccb): sclp_early_cmd(SCLP_CMDW_WRITE_EVENT_DATA,,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt220_sccb {
    pub header: sccb_header,
    struct {
    pub header: evbuf_header,
    pub data: [c_char; ],
    pub msg: },
    pub __packed: },
// Output multi-line text using SCLP VT220 interface.
#[no_mangle]
unsafe extern "C" fn sclp_early_print_vt220(str: *const c_char, len: c_uint) {
    static void sclp_early_print_vt220(const char *str, unsigned int len)
    {
    pub sccb: *mut vt220_sccb,
    pub sclp_early_sccb: *mut *mut sccb = (struct vt220_sccb ),
    if (sizeof(*sccb) + len >= EARLY_SCCB_SIZE)
    pub sizeof(*sccb): *mut len = EARLY_SCCB_SIZE -,
    pub sizeof(*sccb)): *mut memset(sccb, 0,,
    pub len): memcpy(&sccb->msg.data, str,,
    pub len: *mut *mut sccb->header.length = sizeof(sccb) +,
    pub len: sccb->msg.header.length = sizeof(sccb->msg) +,
    pub EVTYP_VT220MSG: sccb->msg.header.type =,
    pub sccb): sclp_early_cmd(SCLP_CMDW_WRITE_EVENT_DATA,,
    }
    int sclp_early_set_event_mask(struct init_sccb *sccb,
    sccb_mask_t receive_mask,
    sccb_mask_t send_mask)
    {
    retry:
    pub sizeof(*sccb)): *mut memset(sccb, 0,,
    pub sizeof(*sccb): *mut sccb->header.length =,
    if (sclp_mask_compat_mode)
    pub SCLP_MASK_SIZE_COMPAT: sccb->mask_length =,
    else
    pub sizeof(sccb_mask_t): sccb->mask_length =,
    pub receive_mask): sccb_set_recv_mask(sccb,,
    pub send_mask): sccb_set_send_mask(sccb,,
    if (sclp_early_cmd(SCLP_CMDW_WRITE_EVENT_MASK, sccb))
    pub -EIO: return,
    if ((sccb.header.response_code == 0x74f0) && !sclp_mask_compat_mode) {
    pub true: sclp_mask_compat_mode =,
    pub retry: goto,
    }
    if (sccb.header.response_code != 0x20)
    pub -EIO: return,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_early_con_check_linemode(sccb: *mut init_sccb) -> c_uint {
    unsigned int sclp_early_con_check_linemode(struct init_sccb *sccb)
    {
    if (!(sccb_get_sclp_send_mask(sccb) & EVTYP_OPCMD_MASK))
    pub 0: return,
    if (!(sccb_get_sclp_recv_mask(sccb) & (EVTYP_MSG_MASK | EVTYP_PMSGCMD_MASK)))
    pub 0: return,
    pub 1: return,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_early_con_check_vt220(sccb: *mut init_sccb) -> c_uint {
    unsigned int sclp_early_con_check_vt220(struct init_sccb *sccb)
    {
    if (sccb_get_sclp_send_mask(sccb) & EVTYP_VT220MSG_MASK)
    pub 1: return,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn sclp_early_setup(disable: c_int, have_linemode: *mut c_int, have_vt220: *mut c_int) -> c_int {
    static int sclp_early_setup(int disable, int *have_linemode, int *have_vt220)
    {
    pub send_mask: unsigned long receive_mask,,
    pub sccb: *mut init_sccb,
    pub rc: c_int,
    pub PAGE_SIZE): BUILD_BUG_ON(sizeof(struct init_sccb) >,
// have_linemode = *have_vt220 = 0;
    pub sclp_early_sccb: *mut *mut sccb = (struct init_sccb ),
    pub EVTYP_OPCMD_MASK: receive_mask = disable ? 0 :,
    pub EVTYP_MSG_MASK: send_mask = disable ? 0 : EVTYP_VT220MSG_MASK |,
    pub send_mask): rc = sclp_early_set_event_mask(sccb, receive_mask,,
    if (rc)
    pub rc: return,
// have_linemode = sclp_early_con_check_linemode(sccb);
// have_vt220 = !!(sccb_get_send_mask(sccb) & EVTYP_VT220MSG_MASK);
    pub rc: return,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_early_set_buffer(sccb: *mut c_void) {
    void sclp_early_set_buffer(void *sccb)
    {
    pub sccb: sclp_early_sccb =,
    }
//
// Output one or more lines of text on the SCLP console (VT220 and
// or line-mode).
//
#[no_mangle]
pub unsafe extern "C" fn __sclp_early_printk(str: *const c_char, len: c_uint) {
    void __sclp_early_printk(const char *str, unsigned int len)
    {
    pub have_vt220: int have_linemode,,
    if (sclp_init_state != sclp_init_state_uninitialized)
    if (sclp_early_setup(0, &have_linemode, &have_vt220) != 0)
    if (have_linemode)
    pub len): sclp_early_print_lm(str,,
    if (have_vt220)
    pub len): sclp_early_print_vt220(str,,
    pub &have_vt220): sclp_early_setup(1, &have_linemode,,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_early_printk(str: *const c_char) {
    void sclp_early_printk(const char *str)
    {
    pub strlen(str)): __sclp_early_printk(str,,
    }
//
// Use sclp_emergency_printk() to print a string when the system is in a
// state where regular console drivers cannot be assumed to work anymore.
//
// Callers must make sure that no concurrent SCLP requests are outstanding
// and all other CPUs are stopped, or at least disabled for external
// interrupts.
//
#[no_mangle]
pub unsafe extern "C" fn sclp_emergency_printk(str: *const c_char) {
    void sclp_emergency_printk(const char *str)
    {
    pub have_vt220: int have_linemode,,
    pub len: c_uint,
    pub strlen(str): len =,
//
// Don't care about return values; if requests fail, just ignore and
// continue to have a rather high chance that anything is printed.
//
    pub &have_vt220): sclp_early_setup(0, &have_linemode,,
    pub len): sclp_early_print_lm(str,,
    pub len): sclp_early_print_vt220(str,,
    pub &have_vt220): sclp_early_setup(1, &have_linemode,,
    }
//
// We can't pass sclp_info_sccb to sclp_early_cmd() here directly,
// because it might not fulfil the requiremets for a SCLP communication buffer:
// - lie below 2G in memory
// - be page-aligned
// Therefore, we use the buffer sclp_early_sccb (which fulfils all those
// requirements) temporarily for communication and copy a received response
// back into the buffer sclp_info_sccb upon successful completion.
//
#[no_mangle]
pub unsafe extern "C" fn sclp_early_read_info() -> int __init {
    int __init sclp_early_read_info(void)
    {
    pub i: c_int,
    pub PAGE_SIZE: int length = test_facility(140) ? EXT_SCCB_READ_SCP :,
    pub )sclp_early_sccb: *mut *mut read_info_sccb sccb = (read_info_sccb,
    sclp_cmdw_t commands[] = {SCLP_CMDW_READ_SCP_INFO_FORCED,
    pub {: for (i = 0; i < ARRAY_SIZE(commands); i++),
    pub length): memset(sccb, 0,,
    pub length: sccb->header.length =,
    pub 0x80: sccb->header.function_code =,
    pub 0x80: sccb->header.control_mask[2] =,
    if (sclp_early_cmd(commands[i], sccb))
    if (sccb.header.response_code == 0x10) {
    pub length): memcpy(&sclp_info_sccb, sccb,,
    pub 1: sclp_info_sccb_valid =,
    pub 0: return,
    }
    if (sccb.header.response_code != 0x1f0)
    }
    pub -EIO: return,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_early_get_info() -> *mut read_info_sccb  __init {
    struct read_info_sccb * __init sclp_early_get_info(void)
    {
    if (!sclp_info_sccb_valid)
    pub NULL: return,
    pub &sclp_info_sccb: return,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_early_get_memsize(mem: *mut c_ulong) -> int __init {
    int __init sclp_early_get_memsize(unsigned long *mem)
    {
    pub rnmax: c_ulong,
    pub rnsize: c_ulong,
    pub &sclp_info_sccb: *mut *mut read_info_sccb sccb =,
    if (!sclp_info_sccb_valid)
    pub -EIO: return,
    pub sccb->rnmax2: rnmax = sccb->rnmax ? sccb->rnmax :,
    pub sccb->rnsize2: rnsize = sccb->rnsize ? sccb->rnsize :,
    pub 20: rnsize <<=,
// mem = rnsize * rnmax;
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_early_get_hsa_size(hsa_size: *mut c_ulong) -> int __init {
    int __init sclp_early_get_hsa_size(unsigned long *hsa_size)
    {
    if (!sclp_info_sccb_valid)
    pub -EIO: return,
// hsa_size = 0;
    if (sclp_info_sccb.hsa_size)
// hsa_size = (sclp_info_sccb.hsa_size - 1) * PAGE_SIZE;
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn sclp_early_detect_machine_features() -> void __init {
    void __init sclp_early_detect_machine_features(void)
    {
    pub &sclp_info_sccb: *mut *mut read_info_sccb sccb =,
    if (!sclp_info_sccb_valid)
    if (sccb.fac85 & 0x02)
    if (sccb.fac91 & 0x40)
    }
pub const SCLP_STORAGE_INFO_FACILITY: c_uint = 0x0000400000000000UL;
    void __weak __init add_physmem_online_range(u64 start, u64 end) {}
#[no_mangle]
pub unsafe extern "C" fn sclp_early_read_storage_info() -> int __init {
    int __init sclp_early_read_storage_info(void)
    {
    pub )sclp_early_sccb: *mut *mut read_storage_sccb sccb = (read_storage_sccb,
    pub 0: int rc, id, max_id =,
    pub rzm: unsigned long rn,,
    pub command: sclp_cmdw_t,
    pub sn: u16,
    if (!sclp_info_sccb_valid)
    pub -EIO: return,
    if (!(sclp_info_sccb.facilities & SCLP_STORAGE_INFO_FACILITY))
    pub -EOPNOTSUPP: return,
    pub sclp_info_sccb.rnsize2: rzm = sclp_info_sccb.rnsize ?:,
    pub 20: rzm <<=,
    pub {: for (id = 0; id <= max_id; id++),
    pub EARLY_SCCB_SIZE): memset(sclp_early_sccb, 0,,
    pub EARLY_SCCB_SIZE: sccb->header.length =,
    pub 8): command = SCLP_CMDW_READ_STORAGE_INFO | (id <<,
    pub sccb): rc = sclp_early_cmd(command,,
    if (rc)
    pub fail: goto,
    pub sccb->max_id: max_id =,
    switch (sccb.header.response_code) {
    case 0x0010:
    pub {: for (sn = 0; sn < sccb->assigned; sn++),
    if (!sccb.entries[sn])
    pub 16: rn = sccb->entries[sn] >>,
    pub rzm): *mut *mut *mut add_physmem_online_range((rn - 1)  rzm, rn,
    }
    case 0x0310:
    case 0x0410:
    default:
    pub fail: goto,
    }
    }
    pub 0: return,
    fail:
    pub 0: physmem_info.range_count =,
    pub -EIO: return,
    }

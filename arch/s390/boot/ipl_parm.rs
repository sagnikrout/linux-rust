//! Automatically rewritten from C to Rust
//! Source: arch/s390/boot/ipl_parm.c
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

    struct parmarea parmarea __section(".parmarea") = {
    .kernel_version		= (unsigned long)kernel_version,
    .max_command_line_size	= COMMAND_LINE_SIZE,
    .command_line		= "root=/dev/ram0 ro",
    };
    char __bootdata(early_command_line)[COMMAND_LINE_SIZE];
    static char command_line_buf[COMMAND_LINE_SIZE];
    unsigned int __bootdata_preserved(zlib_dfltcc_support) = ZLIB_DFLTCC_FULL;
    struct ipl_parameter_block __bootdata_preserved(ipl_block);
    int __bootdata_preserved(ipl_block_valid);
    int __bootdata_preserved(__kaslr_enabled);
    int __bootdata_preserved(cmma_flag) = 1;
    let mut vmalloc_size: c_ulong = VMALLOC_DEFAULT_SIZE;
    unsigned long memory_limit;
    int vmalloc_size_set;
#[no_mangle]
pub unsafe extern "C" fn __diag308(subcode: c_ulong, addr: *mut c_void) -> c_int {
    static inline int __diag308(unsigned long subcode, void *addr)
    {
    let mut r1: union register_pair = { .even = (unsigned long)addr, .odd = 0 };
    asm_inline volatile(
    "	diag	%[r1],%[subcode],0x308\n"
    "0:\n"
    EX_TABLE(0b, 0b)
    : [r1] "+d" (r1.pair)
    : [subcode] "d" (subcode)
    : "cc", "memory");
    return r1.odd;
    }
#[no_mangle]
pub unsafe extern "C" fn store_ipl_parmblock() {
    void store_ipl_parmblock(void)
    {
    int rc;
    rc = __diag308(DIAG308_STORE, &ipl_block);
    if (rc == DIAG308_RC_OK &&
    ipl_block.hdr.version <= IPL_MAX_SUPPORTED_VERSION)
    ipl_block_valid = 1;
    }
#[no_mangle]
pub unsafe extern "C" fn is_ipl_block_dump() -> bool {
    bool is_ipl_block_dump(void)
    {
    if (ipl_block.pb0_hdr.pbt == IPL_PBT_FCP &&
    ipl_block.fcp.opt == IPL_PB0_FCP_OPT_DUMP)
    return true;
    if (ipl_block.pb0_hdr.pbt == IPL_PBT_NVME &&
    ipl_block.nvme.opt == IPL_PB0_NVME_OPT_DUMP)
    return true;
    if (ipl_block.pb0_hdr.pbt == IPL_PBT_ECKD &&
    ipl_block.eckd.opt == IPL_PB0_ECKD_OPT_DUMP)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn scpdata_length(buf: *const u8, count: usize) -> usize {
    static size_t scpdata_length(const u8 *buf, size_t count)
    {
    while (count) {
    if (buf[count - 1] != '\0' && buf[count - 1] != ' ')
    break;
    count--;
    }
    return count;
    }
    static size_t ipl_block_get_ascii_scpdata(char *dest, size_t size,
    const struct ipl_parameter_block *ipb)
    {
    const __u8 *scp_data;
    __u32 scp_data_len;
    int has_lowercase;
    let mut count: usize = 0;
    size_t i;
    switch (ipb.pb0_hdr.pbt) {
    case IPL_PBT_FCP:
    scp_data_len = ipb.fcp.scp_data_len;
    scp_data = ipb.fcp.scp_data;
    break;
    case IPL_PBT_NVME:
    scp_data_len = ipb.nvme.scp_data_len;
    scp_data = ipb.nvme.scp_data;
    break;
    case IPL_PBT_ECKD:
    scp_data_len = ipb.eckd.scp_data_len;
    scp_data = ipb.eckd.scp_data;
    break;
    default:
    goto out;
    }
    count = min(size - 1, scpdata_length(scp_data, scp_data_len));
    if (!count)
    goto out;
    has_lowercase = 0;
    for (i = 0; i < count; i++) {
    if (!isascii(scp_data[i])) {
    count = 0;
    goto out;
    }
    if (!has_lowercase && islower(scp_data[i]))
    has_lowercase = 1;
    }
    if (has_lowercase)
    memcpy(dest, scp_data, count);
    else
    for (i = 0; i < count; i++)
    dest[i] = tolower(scp_data[i]);
    out:
    dest[count] = '\0';
    return count;
    }
#[no_mangle]
unsafe extern "C" fn append_ipl_block_parm() {
    static void append_ipl_block_parm(void)
    {
    size_t len, extra = 0;
    char *delim;
    len = strlen(early_command_line);
    delim = early_command_line + len; /* '\0' character position */
    switch (ipl_block.pb0_hdr.pbt) {
    case IPL_PBT_CCW:
    extra = ipl_block_get_ascii_vmparm(command_line_buf, sizeof(command_line_buf), &ipl_block);
    break;
    case IPL_PBT_FCP:
    case IPL_PBT_NVME:
    case IPL_PBT_ECKD:
    extra = ipl_block_get_ascii_scpdata(command_line_buf, sizeof(command_line_buf), &ipl_block);
    break;
    }
    if (extra) {
    if (command_line_buf[0] == '=') {
    memmove(early_command_line, command_line_buf + 1, extra);
    } else if (len < COMMAND_LINE_SIZE - 2) {
// delim = ' '; /* replace '\0' with space
    sized_strscpy(delim + 1, command_line_buf, COMMAND_LINE_SIZE - len - 1);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn has_ebcdic_char(str: *const c_char) -> c_int {
    static inline int has_ebcdic_char(const char *str)
    {
    int i;
    for (i = 0; str[i]; i++)
    if (str[i] & 0x80)
    return 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn setup_boot_command_line() {
    void setup_boot_command_line(void)
    {
    parmarea.command_line[COMMAND_LINE_SIZE - 1] = 0;
// convert arch command line to ascii if necessary
    if (has_ebcdic_char(parmarea.command_line))
    EBCASC(parmarea.command_line, COMMAND_LINE_SIZE);
// copy arch command line
    strscpy(early_command_line, strim(parmarea.command_line));
// append IPL PARM data to the boot command line
    if (!is_prot_virt_guest() && ipl_block_valid)
    append_ipl_block_parm();
    }
#[no_mangle]
unsafe extern "C" fn modify_facility(nr: c_ulong, clear: bool) {
    static void modify_facility(unsigned long nr, bool clear)
    {
    if (clear)
    __clear_facility(nr, stfle_fac_list);
    else
    __set_facility(nr, stfle_fac_list);
    }
#[no_mangle]
unsafe extern "C" fn check_cleared_facilities() {
    static void check_cleared_facilities(void)
    {
    unsigned long als[] = { FACILITIES_ALS };
    int i;
    for (i = 0; i < ARRAY_SIZE(als); i++) {
    if ((stfle_fac_list[i] & als[i]) != als[i]) {
    boot_emerg("The Linux kernel requires facilities cleared via command line option\n");
    print_missing_facilities();
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn modify_fac_list(str: *mut c_char) {
    static void modify_fac_list(char *str)
    {
    unsigned long val, endval;
    char *endp;
    bool clear;
    while (*str) {
    clear = false;
    if (*str == '!') {
    clear = true;
    str++;
    }
    val = simple_strtoull(str, &endp, 0);
    if (str == endp)
    break;
    str = endp;
    if (*str == '-') {
    str++;
    endval = simple_strtoull(str, &endp, 0);
    if (str == endp)
    break;
    str = endp;
    while (val <= endval && val < MAX_FACILITY_BIT) {
    modify_facility(val, clear);
    val++;
    }
    } else {
    modify_facility(val, clear);
    }
    if (*str != ',')
    break;
    str++;
    }
    check_cleared_facilities();
    }
#[no_mangle]
pub unsafe extern "C" fn parse_boot_command_line() {
    void parse_boot_command_line(void)
    {
    char *param, *val;
    bool enabled;
    char *args;
    int rc;
    __kaslr_enabled = IS_ENABLED(CONFIG_RANDOMIZE_BASE);
    strscpy(command_line_buf, early_command_line);
    args = command_line_buf;
    while (*args) {
    args = next_arg(args, &param, &val);
    if (!strcmp(param, "mem") && val)
    memory_limit = round_down(memparse(val, core::ptr::null_mut()), PAGE_SIZE);
    if (!strcmp(param, "vmalloc") && val) {
    vmalloc_size = round_up(memparse(val, core::ptr::null_mut()), _SEGMENT_SIZE);
    vmalloc_size_set = 1;
    }
    if (!strcmp(param, "dfltcc") && val) {
    if (!strcmp(val, "off"))
    zlib_dfltcc_support = ZLIB_DFLTCC_DISABLED;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(val, _arg: "on")) -> else {
    else if (!strcmp(val, "on"))
    zlib_dfltcc_support = ZLIB_DFLTCC_FULL;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(val, _arg: "def_only")) -> else {
    else if (!strcmp(val, "def_only"))
    zlib_dfltcc_support = ZLIB_DFLTCC_DEFLATE_ONLY;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(val, _arg: "inf_only")) -> else {
    else if (!strcmp(val, "inf_only"))
    zlib_dfltcc_support = ZLIB_DFLTCC_INFLATE_ONLY;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(val, _arg: "always")) -> else {
    else if (!strcmp(val, "always"))
    zlib_dfltcc_support = ZLIB_DFLTCC_FULL_DEBUG;
    }
    if (!strcmp(param, "facilities") && val)
    modify_fac_list(val);
    if (!strcmp(param, "debug-alternative"))
    alt_debug_setup(val);
    if (!strcmp(param, "nokaslr"))
    __kaslr_enabled = 0;
    if (!strcmp(param, "cmma")) {
    rc = kstrtobool(val, &enabled);
    if (!rc && !enabled)
    cmma_flag = 0;
    }

    if (!strcmp(param, "debug_stackprotector"))
    stack_protector_debug = 1;

    if (!strcmp(param, "prot_virt")) {
    rc = kstrtobool(val, &enabled);
    if (!rc && enabled)
    prot_virt_host = 1;
    }

    if (!strcmp(param, "relocate_lowcore") && test_facility(193))
    set_machine_feature(MFEATURE_LOWCORE);
    if (!strcmp(param, "earlyprintk"))
    boot_earlyprintk = true;
    if (!strcmp(param, "debug"))
    boot_console_loglevel = CONSOLE_LOGLEVEL_DEBUG;
    if (!strcmp(param, "bootdebug")) {
    bootdebug = true;
    if (val)
    strscpy(bootdebug_filter, val);
    }
    if (!strcmp(param, "quiet"))
    boot_console_loglevel = CONSOLE_LOGLEVEL_QUIET;
    if (!strcmp(param, "ignore_loglevel"))
    boot_ignore_loglevel = true;
    if (!strcmp(param, "loglevel")) {
    boot_console_loglevel = simple_strtoull(val, core::ptr::null_mut(), 10);
    if (boot_console_loglevel < CONSOLE_LOGLEVEL_MIN)
    boot_console_loglevel = CONSOLE_LOGLEVEL_MIN;
    }
    }
    }

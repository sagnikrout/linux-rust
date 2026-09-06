//! Automatically rewritten from C to Rust
//! Source: drivers/accessibility/speakup/varhandlers.c
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

    static struct st_var_header var_headers[] = {
    { "version", VERSION, VAR_PROC, core::ptr::null_mut(), core::ptr::null_mut() },
    { "synth_name", SYNTH, VAR_PROC, core::ptr::null_mut(), core::ptr::null_mut() },
    { "keymap", KEYMAP, VAR_PROC, core::ptr::null_mut(), core::ptr::null_mut() },
    { "silent", SILENT, VAR_PROC, core::ptr::null_mut(), core::ptr::null_mut() },
    { "punc_some", PUNC_SOME, VAR_PROC, core::ptr::null_mut(), core::ptr::null_mut() },
    { "punc_most", PUNC_MOST, VAR_PROC, core::ptr::null_mut(), core::ptr::null_mut() },
    { "punc_all", PUNC_ALL, VAR_PROC, core::ptr::null_mut(), core::ptr::null_mut() },
    { "delimiters", DELIM, VAR_PROC, core::ptr::null_mut(), core::ptr::null_mut() },
    { "repeats", REPEATS, VAR_PROC, core::ptr::null_mut(), core::ptr::null_mut() },
    { "ex_num", EXNUMBER, VAR_PROC, core::ptr::null_mut(), core::ptr::null_mut() },
    { "characters", CHARS, VAR_PROC, core::ptr::null_mut(), core::ptr::null_mut() },
    { "synth_direct", SYNTH_DIRECT, VAR_PROC, core::ptr::null_mut(), core::ptr::null_mut() },
    { "caps_start", CAPS_START, VAR_STRING, spk_str_caps_start, core::ptr::null_mut() },
    { "caps_stop", CAPS_STOP, VAR_STRING, spk_str_caps_stop, core::ptr::null_mut() },
    { "delay_time", DELAY, VAR_TIME, core::ptr::null_mut(), core::ptr::null_mut() },
    { "trigger_time", TRIGGER, VAR_TIME, core::ptr::null_mut(), core::ptr::null_mut() },
    { "jiffy_delta", JIFFY, VAR_TIME, core::ptr::null_mut(), core::ptr::null_mut() },
    { "full_time", FULL, VAR_TIME, core::ptr::null_mut(), core::ptr::null_mut() },
    { "flush_time", FLUSH, VAR_TIME, core::ptr::null_mut(), core::ptr::null_mut() },
    { "spell_delay", SPELL_DELAY, VAR_NUM, &spk_spell_delay, core::ptr::null_mut() },
    { "bleeps", BLEEPS, VAR_NUM, &spk_bleeps, core::ptr::null_mut() },
    { "attrib_bleep", ATTRIB_BLEEP, VAR_NUM, &spk_attrib_bleep, core::ptr::null_mut() },
    { "bleep_time", BLEEP_TIME, VAR_TIME, &spk_bleep_time, core::ptr::null_mut() },
    { "cursor_time", CURSOR_TIME, VAR_TIME, core::ptr::null_mut(), core::ptr::null_mut() },
    { "punc_level", PUNC_LEVEL, VAR_NUM, &spk_punc_level, core::ptr::null_mut() },
    { "reading_punc", READING_PUNC, VAR_NUM, &spk_reading_punc, core::ptr::null_mut() },
    { "say_control", SAY_CONTROL, VAR_NUM, &spk_say_ctrl, core::ptr::null_mut() },
    { "say_word_ctl", SAY_WORD_CTL, VAR_NUM, &spk_say_word_ctl, core::ptr::null_mut() },
    { "no_interrupt", NO_INTERRUPT, VAR_NUM, &spk_no_intr, core::ptr::null_mut() },
    { "key_echo", KEY_ECHO, VAR_NUM, &spk_key_echo, core::ptr::null_mut() },
    { "bell_pos", BELL_POS, VAR_NUM, &spk_bell_pos, core::ptr::null_mut() },
    { "rate", RATE, VAR_NUM, core::ptr::null_mut(), core::ptr::null_mut() },
    { "pitch", PITCH, VAR_NUM, core::ptr::null_mut(), core::ptr::null_mut() },
    { "inflection", INFLECTION, VAR_NUM, core::ptr::null_mut(), core::ptr::null_mut() },
    { "vol", VOL, VAR_NUM, core::ptr::null_mut(), core::ptr::null_mut() },
    { "tone", TONE, VAR_NUM, core::ptr::null_mut(), core::ptr::null_mut() },
    { "punct", PUNCT, VAR_NUM, core::ptr::null_mut(), core::ptr::null_mut()   },
    { "voice", VOICE, VAR_NUM, core::ptr::null_mut(), core::ptr::null_mut() },
    { "freq", FREQUENCY, VAR_NUM, core::ptr::null_mut(), core::ptr::null_mut() },
    { "lang", LANG, VAR_NUM, core::ptr::null_mut(), core::ptr::null_mut() },
    { "chartab", CHARTAB, VAR_PROC, core::ptr::null_mut(), core::ptr::null_mut() },
    { "direct", DIRECT, VAR_NUM, core::ptr::null_mut(), core::ptr::null_mut() },
    { "pause", PAUSE, VAR_STRING, spk_str_pause, core::ptr::null_mut() },
    { "cur_phonetic", CUR_PHONETIC, VAR_NUM, &spk_cur_phonetic, core::ptr::null_mut() },
    };
    static struct st_var_header *var_ptrs[MAXVARS] = { core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut() };
    static struct punc_var_t punc_vars[] = {
    { PUNC_SOME, 1 },
    { PUNC_MOST, 2 },
    { PUNC_ALL, 3 },
    { DELIM, 4 },
    { REPEATS, 5 },
    { EXNUMBER, 6 },
    { -1, -1 },
    };
#[no_mangle]
pub unsafe extern "C" fn spk_chartab_get_value(keyword: *mut c_char) -> c_int {
    int spk_chartab_get_value(char *keyword)
    {
    let mut value: c_int = 0;
    if (!strcmp(keyword, "ALPHA"))
    value = ALPHA;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(keyword, _arg: "B_CTL")) -> else {
    else if (!strcmp(keyword, "B_CTL"))
    value = B_CTL;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(keyword, _arg: "WDLM")) -> else {
    else if (!strcmp(keyword, "WDLM"))
    value = WDLM;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(keyword, _arg: "A_PUNC")) -> else {
    else if (!strcmp(keyword, "A_PUNC"))
    value = A_PUNC;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(keyword, _arg: "PUNC")) -> else {
    else if (!strcmp(keyword, "PUNC"))
    value = PUNC;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(keyword, _arg: "NUM")) -> else {
    else if (!strcmp(keyword, "NUM"))
    value = NUM;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(keyword, _arg: "A_CAP")) -> else {
    else if (!strcmp(keyword, "A_CAP"))
    value = A_CAP;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(keyword, _arg: "B_CAPSYM")) -> else {
    else if (!strcmp(keyword, "B_CAPSYM"))
    value = B_CAPSYM;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(keyword, _arg: "B_SYM")) -> else {
    else if (!strcmp(keyword, "B_SYM"))
    value = B_SYM;
    return value;
    }
#[no_mangle]
pub unsafe extern "C" fn speakup_register_var(var: *mut var_t) {
    void speakup_register_var(struct var_t *var)
    {
    static char nothing[2] = "\0";
    int i;
    struct st_var_header *p_header;
    BUG_ON(!var || var.var_id < 0 || var.var_id >= MAXVARS);
    if (!var_ptrs[0]) {
    for (i = 0; i < MAXVARS; i++) {
    p_header = &var_headers[i];
    var_ptrs[p_header.var_id] = p_header;
    p_header.data = core::ptr::null_mut();
    }
    }
    p_header = var_ptrs[var.var_id];
    if (p_header.data)
    return;
    p_header.data = var;
    switch (p_header.var_type) {
    case VAR_STRING:
    spk_set_string_var(nothing, p_header, 0);
    break;
    case VAR_NUM:
    case VAR_TIME:
    spk_set_num_var(0, p_header, E_DEFAULT);
    break;
    default:
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn speakup_unregister_var(var_id: enum var_id_t) {
    void speakup_unregister_var(enum var_id_t var_id)
    {
    struct st_var_header *p_header;
    BUG_ON(var_id < 0 || var_id >= MAXVARS);
    p_header = var_ptrs[var_id];
    p_header.data = core::ptr::null_mut();
    }
    struct st_var_header *spk_get_var_header(enum var_id_t var_id)
    {
    struct st_var_header *p_header;
    if (var_id < 0 || var_id >= MAXVARS)
    return core::ptr::null_mut();
    p_header = var_ptrs[var_id];
    if (!p_header.data)
    return core::ptr::null_mut();
    return p_header;
    }
    EXPORT_SYMBOL_GPL(spk_get_var_header);
    struct st_var_header *spk_var_header_by_name(const char *name)
    {
    int i;
    if (!name)
    return core::ptr::null_mut();
    for (i = 0; i < MAXVARS; i++) {
    if (strcmp(name, var_ptrs[i].name) == 0)
    return var_ptrs[i];
    }
    return core::ptr::null_mut();
    }
    struct var_t *spk_get_var(enum var_id_t var_id)
    {
    BUG_ON(var_id < 0 || var_id >= MAXVARS);
    BUG_ON(!var_ptrs[var_id]);
    return var_ptrs[var_id].data;
    }
    EXPORT_SYMBOL_GPL(spk_get_var);
    struct punc_var_t *spk_get_punc_var(enum var_id_t var_id)
    {
    struct punc_var_t *rv = core::ptr::null_mut();
    struct punc_var_t *where;
    where = punc_vars;
    while ((where.var_id != -1) && (!rv)) {
    if (where.var_id == var_id)
    rv = where;
    else
    where++;
    }
    return rv;
    }
// handlers for setting vars
#[no_mangle]
pub unsafe extern "C" fn spk_set_num_var(input: c_int, var: *mut st_var_header, how: c_int) -> c_int {
    int spk_set_num_var(int input, struct st_var_header *var, int how)
    {
    int val;
    int *p_val = var.p_val;
    char buf[32];
    char *cp;
    struct var_t *var_data = var.data;
    if (!var_data)
    return -ENODATA;
    val = var_data.u.n.value;
    switch (how) {
    case E_NEW_DEFAULT:
    if (input < var_data.u.n.low || input > var_data.u.n.high)
    return -ERANGE;
    var_data.u.n.default_val = input;
    return 0;
    case E_DEFAULT:
    val = var_data.u.n.default_val;
    break;
    case E_SET:
    val = input;
    break;
    case E_INC:
    val += input;
    break;
    case E_DEC:
    val -= input;
    break;
    }
    if (val < var_data.u.n.low || val > var_data.u.n.high)
    return -ERANGE;
    var_data.u.n.value = val;
    if (var.var_type == VAR_TIME && p_val) {
// p_val = msecs_to_jiffies(val);
    return 0;
    }
    if (p_val)
// p_val = val;
    if (var.var_id == PUNC_LEVEL) {
    spk_punc_mask = spk_punc_masks[val];
    }
    if (var_data.u.n.multiplier != 0)
    val *= var_data.u.n.multiplier;
    val += var_data.u.n.offset;
    if (!synth)
    return 0;
    if (synth.synth_adjust && synth.synth_adjust(synth, var))
    return 0;
    if (var.var_id < FIRST_SYNTH_VAR)
    return 0;
    if (!var_data.u.n.synth_fmt)
    return 0;
    if (var.var_id == PITCH)
    cp = spk_pitch_buff;
    else
    cp = buf;
    if (!var_data.u.n.out_str)
    sprintf(cp, var_data.u.n.synth_fmt, (int)val);
    else
    sprintf(cp, var_data.u.n.synth_fmt,
    var_data.u.n.out_str[val]);
    synth_printf("%s", cp);
    return 0;
    }
    EXPORT_SYMBOL_GPL(spk_set_num_var);
#[no_mangle]
pub unsafe extern "C" fn spk_set_string_var(page: *const c_char, var: *mut st_var_header, len: c_int) -> c_int {
    int spk_set_string_var(const char *page, struct st_var_header *var, int len)
    {
    struct var_t *var_data = var.data;
    if (!var_data)
    return -ENODATA;
    if (len > MAXVARLEN)
    return -E2BIG;
    if (!len) {
    if (!var_data.u.s.default_val)
    return 0;
    if (!var.p_val)
    var.p_val = var_data.u.s.default_val;
    if (var.p_val != var_data.u.s.default_val)
    strcpy((char *)var.p_val, var_data.u.s.default_val);
    return -ERESTART;
    } else if (var.p_val) {
    strcpy((char *)var.p_val, page);
    } else {
    return -E2BIG;
    }
    return 0;
    }
//
// spk_set_mask_bits sets or clears the punc/delim/repeat bits,
// if input is null uses the defaults.
// values for how: 0 clears bits of chars supplied,
// 1 clears allk, 2 sets bits for chars
//
#[no_mangle]
pub unsafe extern "C" fn spk_set_mask_bits(input: *const c_char, which: c_int, how: c_int) -> c_int {
    int spk_set_mask_bits(const char *input, const int which, const int how)
    {
    u_char *cp;
    let mut mask: c_short = spk_punc_info[which].mask;
    if (how & 1) {
    for (cp = (u_char *)spk_punc_info[3].value; *cp; cp++)
    spk_chartab[*cp] &= ~mask;
    }
    cp = (u_char *)input;
    if (!cp) {
    cp = spk_punc_info[which].value;
    } else {
    for (; *cp; cp++) {
    if (*cp < SPACE)
    break;
    if (mask < PUNC) {
    if (!(spk_chartab[*cp] & PUNC))
    break;
    } else if (spk_chartab[*cp] & B_NUM) {
    break;
    }
    }
    if (*cp)
    return -EINVAL;
    cp = (u_char *)input;
    }
    if (how & 2) {
    for (; *cp; cp++)
    if (*cp > SPACE)
    spk_chartab[*cp] |= mask;
    } else {
    for (; *cp; cp++)
    if (*cp > SPACE)
    spk_chartab[*cp] &= ~mask;
    }
    return 0;
    }
    char *spk_strlwr(char *s)
    {
    char *p;
    if (!s)
    return core::ptr::null_mut();
    for (p = s; *p; p++)
// p = tolower(*p);
    return s;
    }
    char *spk_s2uchar(char *start, char *dest)
    {
    int val;
// Do not replace with kstrtoul: here we need start to be updated
    val = simple_strtoul(skip_spaces(start), &start, 10);
    if (*start == ',')
    start++;
// dest = (u_char)val;
    return start;
    }

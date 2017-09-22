// Licensed under the 2-Clause BSD license <LICENSE or
// https://opensource.org/licenses/BSD-2-Clause>. This
// file may not be copied, modified, or distributed
// except according to those terms.

pub(crate) static OPERATION_STRINGS: [&str; 621] = [
    "",
    "aaa",
    "aad",
    "aam",
    "aas",
    "add",
    "adc",
    "and",
    "arpl",
    "blendpd",
    "blendps",
    "blendvpd",
    "blendvps",
    "bound",
    "bsf",
    "bsr",
    "bswap",
    "bt",
    "btc",
    "btr",
    "bts",
    "callf",
    "call",
    "clc",
    "cld",
    "clflush",
    "cli",
    "clts",
    "cmc",
    "cmp",
    "cmpxch16b",
    "cmpxch8b",
    "cmpxchg",
    "cpuid",
    "crc32",
    "daa",
    "das",
    "dec",
    "div",
    "dppd",
    "dpps",
    "emms",
    "enter",
    "f2xm1",
    "fabs",
    "fadd",
    "faddp",
    "fbld",
    "fbstp",
    "fchs",
    "fclex",
    "fcmovb",
    "fcmovbe",
    "fcmove",
    "fcmovnb",
    "fcmovnbe",
    "fcmovne",
    "fcmovnu",
    "fcmovu",
    "fcom",
    "fcomi",
    "fcomip",
    "fcomp",
    "fcompp",
    "fcos",
    "fdecstp",
    "fdisi",
    "fdiv",
    "fdivp",
    "fdivr",
    "fdivrp",
    "femms",
    "feni",
    "ffree",
    "ffreep",
    "fiadd",
    "ficom",
    "ficomp",
    "fidiv",
    "fidivr",
    "fild",
    "fimul",
    "fincstp",
    "finit",
    "fist",
    "fistp",
    "fisttp",
    "fisub",
    "fisubr",
    "fld",
    "fld1",
    "fldcw",
    "fldenv",
    "fldl2e",
    "fldl2t",
    "fldlg2",
    "fldln2",
    "fldpi",
    "fldz",
    "fmul",
    "fmulp",
    "fnop",
    "fpatan",
    "fprem",
    "fprem1",
    "fptan",
    "frichop",
    "frinear",
    "frint2",
    "frndint",
    "frstor",
    "frstpm",
    "fsave",
    "fscale",
    "fsetpm",
    "fsin",
    "fsincos",
    "fsqrt",
    "fst",
    "fstcw",
    "fstdw",
    "fstenv",
    "fstp",
    "fstsg",
    "fstsw",
    "fsub",
    "fsubp",
    "fsubr",
    "fsubrp",
    "ftst",
    "fucom",
    "fucomi",
    "fucomip",
    "fucomp",
    "fucompp",
    "fwait",
    "fxam",
    "fxch",
    "fxrstor",
    "fxsave",
    "fxtract",
    "fyl2x",
    "fyl2xp1",
    "getsec",
    "hlt",
    "idiv",
    "imul",
    "in",
    "inc",
    "int",
    "int1",
    "int3",
    "into",
    "invd",
    "invlpg",
    "iret",
    "jmpf",
    "jmp",
    "lahf",
    "lar",
    "ldmxcsr",
    "lds",
    "lea",
    "leave",
    "les",
    "lfence",
    "lfs",
    "lgs",
    "loop",
    "loope",
    "loopne",
    "lsl",
    "lss",
    "mfence",
    "mov",
    "movnti",
    "movss",
    "movsx",
    "movsxd",
    "movupd",
    "movups",
    "movzx",
    "mpsadbw",
    "mul",
    "neg",
    "nop",
    "not",
    "or",
    "out",
    "packssdw",
    "packsswb",
    "packusdw",
    "packuswb",
    "pabsb",
    "pabsd",
    "pabsw",
    "paddb",
    "paddd",
    "paddq",
    "paddw",
    "paddsb",
    "paddsw",
    "paddusb",
    "paddusw",
    "palignr",
    "pand",
    "pandn",
    "pause",
    "pavgb",
    "pavgusb",
    "pavgw",
    "pblendvb",
    "pblendw",
    "pcmpeqb",
    "pcmpeqd",
    "pcmpeqq",
    "pcmpeqw",
    "pcmpestri",
    "pcmpestrm",
    "pcmpgtb",
    "pcmpgtd",
    "pcmpgtq",
    "pcmpgtw",
    "pcmpistri",
    "pcmpistrm",
    "pf2id",
    "pf2iw",
    "pfacc",
    "pfadd",
    "pfcmpeq",
    "pfcmpge",
    "pfcmpgt",
    "pfmax",
    "pfmin",
    "pfmul",
    "pfnacc",
    "pfpnacc",
    "pfrcp",
    "pfrcpit1",
    "pfrcpit2",
    "pfrcpv",
    "pfrsqit1",
    "pfrsqrt",
    "pfrsqrtv",
    "pfsub",
    "pfsubr",
    "phaddd",
    "phaddsw",
    "phaddw",
    "phminposuw",
    "phsubd",
    "phsubsw",
    "phsubw",
    "pi2fd",
    "pi2fw",
    "pmaddwd",
    "pmaddubsw",
    "pmaxsb",
    "pmaxsd",
    "pmaxsw",
    "pmaxub",
    "pmaxud",
    "pmaxuw",
    "pminsb",
    "pminsd",
    "pminsw",
    "pminub",
    "pminud",
    "pminuw",
    "pmuldq",
    "pmulhrsw",
    "pmulhrw",
    "pmulhuw",
    "pmulhw",
    "pmulld",
    "pmullw",
    "pmuludq",
    "pop",
    "popcnt",
    "por",
    "psadbw",
    "pshufb",
    "psignb",
    "psignd",
    "psignw",
    "pslld",
    "pslldq",
    "psllq",
    "psllw",
    "psrad",
    "psraw",
    "psrld",
    "psrldq",
    "psrlq",
    "psrlw",
    "psubb",
    "psubd",
    "psubq",
    "psubw",
    "psubsb",
    "psubsw",
    "psubusb",
    "psubusw",
    "pswapd",
    "ptest",
    "punpckhbw",
    "punpckhdq",
    "punpckhqdq",
    "punpckhwd",
    "punpcklqdq",
    "push",
    "pxor",
    "rdmsr",
    "rdpmc",
    "rdtsc",
    "retf",
    "retn",
    "rcl",
    "rcr",
    "rol",
    "ror",
    "roundps",
    "roundpd",
    "rsm",
    "sahf",
    "salc",
    "sar",
    "sbb",
    "sfence",
    "shl",
    "shld",
    "shr",
    "shrd",
    "sub",
    "stc",
    "std",
    "sti",
    "stmxcsr",
    "syscall",
    "sysenter",
    "sysexit",
    "sysret",
    "test",
    "ud2",
    "vmread",
    "vmwrite",
    "wbinvd",
    "wrmsr",
    "xchg",
    "xlat",
    "xadd",
    "xor",
    "xrstor",
    "xsave",
    "addps",
    "addpd",
    "addsd",
    "addss",
    "addsubpd",
    "addsubps",
    "andnps",
    "andnpd",
    "andps",
    "andpd",
    "cbw",
    "cwde",
    "cdqe",
    "cmpsb",
    "cmpsw",
    "cmpsd",
    "cmpsq",
    "cmovo",
    "cmovno",
    "cmovb",
    "cmovae",
    "cmove",
    "cmovne",
    "cmovbe",
    "cmova",
    "cmovs",
    "cmovns",
    "cmovpe",
    "cmovpo",
    "cmovl",
    "cmovge",
    "cmovle",
    "cmovg",
    "cwd",
    "cdq",
    "cqo",
    "divps",
    "divpd",
    "divsd",
    "divss",
    "insb",
    "insw",
    "insd",
    "insq",
    "jcxz",
    "jecxz",
    "jrcxz",
    "jo",
    "jno",
    "jb",
    "jae",
    "je",
    "jne",
    "jbe",
    "ja",
    "js",
    "jns",
    "jpe",
    "jpo",
    "jl",
    "jge",
    "jle",
    "jg",
    "lodsb",
    "lodsw",
    "lodsd",
    "lodsq",
    "maxps",
    "maxpd",
    "maxsd",
    "maxss",
    "minps",
    "minpd",
    "minsd",
    "minss",
    "movd",
    "movq",
    "movsb",
    "movsw",
    "movsd",
    "movsq",
    "mulps",
    "mulpd",
    "mulsd",
    "mulss",
    "orps",
    "orpd",
    "outsb",
    "outsw",
    "outsd",
    "outsq",
    "pextrd",
    "pextrq",
    "pinsrd",
    "pinsrq",
    "popa",
    "popad",
    "popf",
    "popfd",
    "popfq",
    "pusha",
    "pushad",
    "pushf",
    "pushfd",
    "pushfq",
    "rcpps",
    "rcpss",
    "rsqrtps",
    "rsqrtss",
    "scasb",
    "scasw",
    "scasd",
    "scasq",
    "seto",
    "setno",
    "setb",
    "setae",
    "sete",
    "setne",
    "setbe",
    "seta",
    "sets",
    "setns",
    "setpe",
    "setpo",
    "setl",
    "setge",
    "setle",
    "setg",
    "sqrtps",
    "sqrtpd",
    "sqrtsd",
    "sqrtss",
    "stosb",
    "stosw",
    "stosd",
    "stosq",
    "subps",
    "subpd",
    "subsd",
    "subss",
    "xorps",
    "xorpd",
    "cmppd",
    "cmpps",
    "cmpss",
    "comisd",
    "comiss",
    "cvtdq2pd",
    "cvtdq2ps",
    "cvtpd2dq",
    "cvtpd2pi",
    "cvtpd2ps",
    "cvtpi2pd",
    "cvtpi2ps",
    "cvtps2dq",
    "cvtps2pd",
    "cvtps2pi",
    "cvtsd2si",
    "cvtsd2ss",
    "cvtsi2sd",
    "cvtsi2ss",
    "cvtss2sd",
    "cvtss2si",
    "cvttpd2dq",
    "cvttpd2pi",
    "cvttps2dq",
    "cvttps2pi",
    "cvttsd2si",
    "cvttss2si",
    "extractps",
    "haddpd",
    "haddps",
    "hsubpd",
    "hsubps",
    "insertps",
    "lddqu",
    "lgdt",
    "lidt",
    "lldt",
    "lmsw",
    "ltr",
    "maskmovq",
    "maskmovdqu",
    "mmxnop",
    "monitor",
    "movapd",
    "movaps",
    "movddup",
    "movdq2q",
    "movdqa",
    "movdqu",
    "movhlps",
    "movhpd",
    "movhps",
    "movshdup",
    "movsldup",
    "movlhps",
    "movlpd",
    "movlps",
    "movmskpd",
    "movmskps",
    "movntdq",
    "movntdqa",
    "movntpd",
    "movntps",
    "movntq",
    "movq2dq",
    "mwait",
    "pinsrb",
    "pinsrw",
    "pextrb",
    "pextrw",
    "pmovmskb",
    "pmovsxbd",
    "pmovsxbq",
    "pmovsxdq",
    "pmovsxbw",
    "pmovsxwd",
    "pmovsxwq",
    "pmovzxbd",
    "pmovzxbq",
    "pmovzxdq",
    "pmovzxbw",
    "pmovzxwd",
    "pmovzxwq",
    "prefetch",
    "prefetchnta",
    "prefetcht0",
    "prefetcht1",
    "prefetcht2",
    "prefetchw",
    "pshufd",
    "pshufhw",
    "pshuflw",
    "pshufw",
    "punpcklbw",
    "punpckldq",
    "punpcklwd",
    "roundsd",
    "roundss",
    "sgdt",
    "sidt",
    "sldt",
    "shufpd",
    "shufps",
    "smsw",
    "str",
    "swapgs",
    "ucomisd",
    "ucomiss",
    "unpckhpd",
    "unpckhps",
    "unpcklpd",
    "unpcklps",
    "verr",
    "verw",
    "vmcall",
    "vmclear",
    "vmlaunch",
    "vmptrld",
    "vmptrst",
    "vmresume",
    "vmxoff",
    "vmxon",
    "xgetbv",
    "xsetbv",
];

#[derive(Clone, Copy, PartialEq, PartialOrd)]
#[repr(i32)]
pub enum InstructionOperation {
    INVALID = 0i32,
    AAA,
    AAD,
    AAM,
    AAS,
    ADD,
    ADC,
    AND,
    ARPL,
    BLENDPD,
    BLENDPS,
    BLENDVPD,
    BLENDVPS,
    BOUND,
    BSF,
    BSR,
    BSWAP,
    BT,
    BTC,
    BTR,
    BTS,
    CALLF,
    CALL,
    CLC,
    CLD,
    CLFLUSH,
    CLI,
    CLTS,
    CMC,
    CMP,
    CMPXCH16B,
    CMPXCH8B,
    CMPXCHG,
    CPUID,
    CRC32,
    DAA,
    DAS,
    DEC,
    DIV,
    DPPD,
    DPPS,
    EMMS,
    ENTER,
    F2XM1,
    FABS,
    FADD,
    FADDP,
    FBLD,
    FBSTP,
    FCHS,
    FCLEX,
    FCMOVB,
    FCMOVBE,
    FCMOVE,
    FCMOVNB,
    FCMOVNBE,
    FCMOVNE,
    FCMOVNU,
    FCMOVU,
    FCOM,
    FCOMI,
    FCOMIP,
    FCOMP,
    FCOMPP,
    FCOS,
    FDECSTP,
    FDISI,
    FDIV,
    FDIVP,
    FDIVR,
    FDIVRP,
    FEMMS,
    FENI,
    FFREE,
    FFREEP,
    FIADD,
    FICOM,
    FICOMP,
    FIDIV,
    FIDIVR,
    FILD,
    FIMUL,
    FINCSTP,
    FINIT,
    FIST,
    FISTP,
    FISTTP,
    FISUB,
    FISUBR,
    FLD,
    FLD1,
    FLDCW,
    FLDENV,
    FLDL2E,
    FLDL2T,
    FLDLG2,
    FLDLN2,
    FLDPI,
    FLDZ,
    FMUL,
    FMULP,
    FNOP,
    FPATAN,
    FPREM,
    FPREM1,
    FPTAN,
    FRICHOP,
    FRINEAR,
    FRINT2,
    FRNDINT,
    FRSTOR,
    FRSTPM,
    FSAVE,
    FSCALE,
    FSETPM,
    FSIN,
    FSINCOS,
    FSQRT,
    FST,
    FSTCW,
    FSTDW,
    FSTENV,
    FSTP,
    FSTSG,
    FSTSW,
    FSUB,
    FSUBP,
    FSUBR,
    FSUBRP,
    FTST,
    FUCOM,
    FUCOMI,
    FUCOMIP,
    FUCOMP,
    FUCOMPP,
    FWAIT,
    FXAM,
    FXCH,
    FXRSTOR,
    FXSAVE,
    FXTRACT,
    FYL2X,
    FYL2XP1,
    GETSEC,
    HLT,
    IDIV,
    IMUL,
    IN,
    INC,
    INT,
    INT1,
    INT3,
    INTO,
    INVD,
    INVLPG,
    IRET,
    JMPF,
    JMP,
    LAHF,
    LAR,
    LDMXCSR,
    LDS,
    LEA,
    LEAVE,
    LES,
    LFENCE,
    LFS,
    LGS,
    LOOP,
    LOOPE,
    LOOPNE,
    LSL,
    LSS,
    MFENCE,
    MOV,
    MOVNTI,
    MOVSS,
    MOVSX,
    MOVSXD,
    MOVUPD,
    MOVUPS,
    MOVZX,
    MPSADBW,
    MUL,
    NEG,
    NOP,
    NOT,
    OR,
    OUT,
    PACKSSDW,
    PACKSSWB,
    PACKUSDW,
    PACKUSWB,
    PABSB,
    PABSD,
    PABSW,
    PADDB,
    PADDD,
    PADDQ,
    PADDW,
    PADDSB,
    PADDSW,
    PADDUSB,
    PADDUSW,
    PALIGNR,
    PAND,
    PANDN,
    PAUSE,
    PAVGB,
    PAVGUSB,
    PAVGW,
    PBLENDVB,
    PBLENDW,
    PCMPEQB,
    PCMPEQD,
    PCMPEQQ,
    PCMPEQW,
    PCMPESTRI,
    PCMPESTRM,
    PCMPGTB,
    PCMPGTD,
    PCMPGTQ,
    PCMPGTW,
    PCMPISTRI,
    PCMPISTRM,
    PF2ID,
    PF2IW,
    PFACC,
    PFADD,
    PFCMPEQ,
    PFCMPGE,
    PFCMPGT,
    PFMAX,
    PFMIN,
    PFMUL,
    PFNACC,
    PFPNACC,
    PFRCP,
    PFRCPIT1,
    PFRCPIT2,
    PFRCPV,
    PFRSQIT1,
    PFRSQRT,
    PFRSQRTV,
    PFSUB,
    PFSUBR,
    PHADDD,
    PHADDSW,
    PHADDW,
    PHMINPOSUW,
    PHSUBD,
    PHSUBSW,
    PHSUBW,
    PI2FD,
    PI2FW,
    PMADDWD,
    PMADDUBSW,
    PMAXSB,
    PMAXSD,
    PMAXSW,
    PMAXUB,
    PMAXUD,
    PMAXUW,
    PMINSB,
    PMINSD,
    PMINSW,
    PMINUB,
    PMINUD,
    PMINUW,
    PMULDQ,
    PMULHRSW,
    PMULHRW,
    PMULHUW,
    PMULHW,
    PMULLD,
    PMULLW,
    PMULUDQ,
    POP,
    POPCNT,
    POR,
    PSADBW,
    PSHUFB,
    PSIGNB,
    PSIGND,
    PSIGNW,
    PSLLD,
    PSLLDQ,
    PSLLQ,
    PSLLW,
    PSRAD,
    PSRAW,
    PSRLD,
    PSRLDQ,
    PSRLQ,
    PSRLW,
    PSUBB,
    PSUBD,
    PSUBQ,
    PSUBW,
    PSUBSB,
    PSUBSW,
    PSUBUSB,
    PSUBUSW,
    PSWAPD,
    PTEST,
    PUNPCKHBW,
    PUNPCKHDQ,
    PUNPCKHQDQ,
    PUNPCKHWD,
    PUNPCKLQDQ,
    PUSH,
    PXOR,
    RDMSR,
    RDPMC,
    RDTSC,
    RETF,
    RETN,
    RCL,
    RCR,
    ROL,
    ROR,
    ROUNDPS,
    ROUNDPD,
    RSM,
    SAHF,
    SALC,
    SAR,
    SBB,
    SFENCE,
    SHL,
    SHLD,
    SHR,
    SHRD,
    SUB,
    STC,
    STD,
    STI,
    STMXCSR,
    SYSCALL,
    SYSENTER,
    SYSEXIT,
    SYSRET,
    TEST,
    UD2,
    VMREAD,
    VMWRITE,
    WBINVD,
    WRMSR,
    XCHG,
    XLAT,
    XADD,
    XOR,
    XRSTOR,
    XSAVE,
    ADDPS,
    ADDPD,
    ADDSD,
    ADDSS,
    ADDSUBPD,
    ADDSUBPS,
    ANDNPS,
    ANDNPD,
    ANDPS,
    ANDPD,
    CBW,
    CWDE,
    CDQE,
    CMPSB,
    CMPSW,
    CMPSD,
    CMPSQ,
    CMOVO,
    CMOVNO,
    CMOVB,
    CMOVAE,
    CMOVE,
    CMOVNE,
    CMOVBE,
    CMOVA,
    CMOVS,
    CMOVNS,
    CMOVPE,
    CMOVPO,
    CMOVL,
    CMOVGE,
    CMOVLE,
    CMOVG,
    CWD,
    CDQ,
    CQO,
    DIVPS,
    DIVPD,
    DIVSD,
    DIVSS,
    INSB,
    INSW,
    INSD,
    INSQ,
    JCXZ,
    JECXZ,
    JRCXZ,
    JO,
    JNO,
    JB,
    JAE,
    JE,
    JNE,
    JBE,
    JA,
    JS,
    JNS,
    JPE,
    JPO,
    JL,
    JGE,
    JLE,
    JG,
    LODSB,
    LODSW,
    LODSD,
    LODSQ,
    MAXPS,
    MAXPD,
    MAXSD,
    MAXSS,
    MINPS,
    MINPD,
    MINSD,
    MINSS,
    MOVD,
    MOVQ,
    MOVSB,
    MOVSW,
    MOVSD,
    MOVSQ,
    MULPS,
    MULPD,
    MULSD,
    MULSS,
    ORPS,
    ORPD,
    OUTSB,
    OUTSW,
    OUTSD,
    OUTSQ,
    PEXTRD,
    PEXTRQ,
    PINSRD,
    PINSRQ,
    POPA,
    POPAD,
    POPF,
    POPFD,
    POPFQ,
    PUSHA,
    PUSHAD,
    PUSHF,
    PUSHFD,
    PUSHFQ,
    RCPPS,
    RCPSS,
    RSQRTPS,
    RSQRTSS,
    SCASB,
    SCASW,
    SCASD,
    SCASQ,
    SETO,
    SETNO,
    SETB,
    SETAE,
    SETE,
    SETNE,
    SETBE,
    SETA,
    SETS,
    SETNS,
    SETPE,
    SETPO,
    SETL,
    SETGE,
    SETLE,
    SETG,
    SQRTPS,
    SQRTPD,
    SQRTSD,
    SQRTSS,
    STOSB,
    STOSW,
    STOSD,
    STOSQ,
    SUBPS,
    SUBPD,
    SUBSD,
    SUBSS,
    XORPS,
    XORPD,
    CMPPD,
    CMPPS,
    CMPSS,
    COMISD,
    COMISS,
    CVTDQ2PD,
    CVTDQ2PS,
    CVTPD2DQ,
    CVTPD2PI,
    CVTPD2PS,
    CVTPI2PD,
    CVTPI2PS,
    CVTPS2DQ,
    CVTPS2PD,
    CVTPS2PI,
    CVTSD2SI,
    CVTSD2SS,
    CVTSI2SD,
    CVTSI2SS,
    CVTSS2SD,
    CVTSS2SI,
    CVTTPD2DQ,
    CVTTPD2PI,
    CVTTPS2DQ,
    CVTTPS2PI,
    CVTTSD2SI,
    CVTTSS2SI,
    EXTRACTPS,
    HADDPD,
    HADDPS,
    HSUBPD,
    HSUBPS,
    INSERTPS,
    LDDQU,
    LGDT,
    LIDT,
    LLDT,
    LMSW,
    LTR,
    MASKMOVQ,
    MASKMOVDQU,
    MMXNOP,
    MONITOR,
    MOVAPD,
    MOVAPS,
    MOVDDUP,
    MOVDQ2Q,
    MOVDQA,
    MOVDQU,
    MOVHLPS,
    MOVHPD,
    MOVHPS,
    MOVSHDUP,
    MOVSLDUP,
    MOVLHPS,
    MOVLPD,
    MOVLPS,
    MOVMSKPD,
    MOVMSKPS,
    MOVNTDQ,
    MOVNTDQA,
    MOVNTPD,
    MOVNTPS,
    MOVNTQ,
    MOVQ2DQ,
    MWAIT,
    PINSRB,
    PINSRW,
    PEXTRB,
    PEXTRW,
    PMOVMSKB,
    PMOVSXBD,
    PMOVSXBQ,
    PMOVSXDQ,
    PMOVSXBW,
    PMOVSXWD,
    PMOVSXWQ,
    PMOVZXBD,
    PMOVZXBQ,
    PMOVZXDQ,
    PMOVZXBW,
    PMOVZXWD,
    PMOVZXWQ,
    PREFETCH,
    PREFETCHNTA,
    PREFETCHT0,
    PREFETCHT1,
    PREFETCHT2,
    PREFETCHW,
    PSHUFD,
    PSHUFHW,
    PSHUFLW,
    PSHUFW,
    PUNPCKLBW,
    PUNPCKLDQ,
    PUNPCKLWD,
    ROUNDSD,
    ROUNDSS,
    SGDT,
    SIDT,
    SLDT,
    SHUFPD,
    SHUFPS,
    SMSW,
    STR,
    SWAPGS,
    UCOMISD,
    UCOMISS,
    UNPCKHPD,
    UNPCKHPS,
    UNPCKLPD,
    UNPCKLPS,
    VERR,
    VERW,
    VMCALL,
    VMCLEAR,
    VMLAUNCH,
    VMPTRLD,
    VMPTRST,
    VMRESUME,
    VMXOFF,
    VMXON,
    XGETBV,
    XSETBV,
}

impl InstructionOperation {
    pub fn from_i32(_i: i32) -> Self {
        unimplemented!();
    }
}

impl Default for InstructionOperation {
    fn default() -> Self {
        InstructionOperation::INVALID
    }
}
.section .ivr, "ax", @progbits
.align 1

ivr:
    jmp _ivr_reset

_ivr_undefined:
    reti

;;; By default, start all interrupt handlers as doing a no-op
;;; return. Since these symbols are weakly linked, user code can
;;; override each one on demand.
.macro _ivr_default name target=_ivr_undefined
.weak \name
.set  \name, \target
.endm

_ivr_default _ivr_reset main    ; Default to `main` here, no need for indirection

//!comment /* */ //
//!string " '


variable:
    leti := let WORD \: WORD = t:term EOL
    letiinf := let WORD = t:term EOL
    let := let WORD \: WORD EOL
    letinf := let WORD EOL

    mutiinf := mut WORD = t:term EOL
    muti := mut WORD \: WORD = t:term EOL
    mut := mut WORD \: WORD EOL
    mutinf := mut WORD EOL

assign:
    assign := WORD = e:expr EOL

op!
    equ := ==
    add := +
    gr := >
    sub := -

expr!
    binary := t:term o:op e:expr
    term := t:term

if:
    if := if e:expr EOL INDENT

func:
    fdef := WORD \: WORD ( p:pars ) EOL INDENT
    fdefinf := WORD \: ( p:pars ) EOL INDENT
    fdefv := WORD \: WORD () EOL INDENT

    fdec := WORD \: WORD ( p:pars ) EOL
    fdecv := WORD \: WORD () EOL

    ret := return e:expr EOL

call:
    call := WORD ( a:args ) EOL

while:
    while:= while e:expr EOL INDENT

pars!
    tpar := w:WORD \: w:WORD , p:pars
    par := w:WORD , p:pars
    tpar := w:WORD \: w:WORD
    par := w:WORD

args!
    arg := t:term , a:args
    arg := t:term

term!
    call := WORD ( a:args )
    number := NUMBER
    word := WORD
    string := STRING

delim:
    eol := EOL
    dedent := DEDENT
    indent := INDENT

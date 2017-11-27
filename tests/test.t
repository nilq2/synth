!/def/comment /* */ //
!/def/string " '


variable:
    leti := let name:WORD \: type:WORD = e:expr EOL
    letiinf := let name:WORD = e:expr EOL
    let := let name:WORD \: type:WORD EOL
    letinf := let name:WORD EOL

    mutiinf := mut name:WORD = e:expr EOL
    muti := mut name:WORD \: type:WORD = e:expr EOL
    mut := mut name:WORD \: type:WORD EOL
    mutinf := mut name:WORD EOL

assign:
    assign := name:WORD = e:expr EOL

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
    fdef := fname:WORD \: type:WORD ( p:pars ) EOL INDENT
    fdefinf := fname:WORD \: ( p:pars ) EOL INDENT
    fdefv := fname:WORD \: type:WORD () EOL INDENT

    fdec := fname:WORD \: type:WORD ( p:pars ) EOL
    fdecv := fname:WORD \: type:WORD () EOL

    ret := return e:expr EOL

call:
    call := fname:WORD ( a:args ) EOL

while:
    while:= while e:expr EOL INDENT

pars!
    tpar := name:WORD \: type:WORD , p:pars
    par := name:WORD , p:pars
    tpar := name:WORD \: name:WORD
    par := name:WORD

args!
    arg := arg:term , a:args
    arg := arg:term

term!
    call := fname:WORD ( a:args )
    number := num:NUMBER
    word := word:WORD
    string := str:STRING

delim:
    eol := EOL
    dedent := DEDENT
    indent := INDENT

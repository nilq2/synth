!/def/comment /* */ //
!/def/string " '


variable:
    leti := let name:WORD \: type:WORD = e:expr EOL
        [output]
            write: const {type} {name} = {expr} ;

    letiinf := let name:WORD = e:expr EOL
        [output]
            write: const __typeof__({expr}) {name} = {expr} ;

    let := let name:WORD \: type:WORD EOL
        [output]
            error: name, "immutable variable must be initialized"

    letinf := let name:WORD EOL
        [output]
            error: name, "immutable variable must be initialized"

    muti := mut name:WORD \: type:WORD = e:expr EOL
        [output]
            write: {type} {name} = {expr} ;

    mutiinf := mut name:WORD = e:expr EOL
        [output]
            write: __typeof__({expr}) {name} = {expr} ;

    mut := mut name:WORD \: type:WORD EOL
        [output]
            write: {type} {name} ;

    mutinf := mut name:WORD EOL
        [output]
            error: name, "cross-code type inference not yet implemented"

assign:
    assign := name:WORD = e:expr EOL
        [output]
            write: {name} = {expr} ;

op!
    equ := ==
        [output]
            write: (==)
    add := +
        [output]
            write: +
    gr := >
        [output]
            write: >
    sub := -
        [output]
            write: -

expr!
    binary := t:term o:op e:expr
        [output]
            write: {term} {op} {expr}

    term := t:term
        [output]
            write: {term}

if:
    if := if e:expr EOL INDENT
        [output]
            write: if \( {expr} ) {

func:
    fdef := fname:WORD \: type:WORD ( p:pars ) EOL INDENT
        [output]
            write: {type} {fname} \( {p} ) {

    fdefinf := fname:WORD \: ( p:pars ) EOL INDENT
        [output]
            error: fname, "return type inference not yet implemented"

    fdefv := fname:WORD \: type:WORD () EOL INDENT
        [output]
            write: {type} {fname} \( void ) {

    fdec := fname:WORD \: type:WORD ( p:pars ) EOL
        [output]
            write: {type} {fname} \( {p} ) ;

    fdecv := fname:WORD \: type:WORD () EOL
        [output]
            write: {type} {fname} \( void ) ;

    ret := return e:expr EOL
        [output]
            write: return {expr} ;

call:
    call := fname:WORD ( a:args ) EOL
        [output]
            write: {fname} \( {a} )

while:
    while:= while e:expr EOL INDENT
        [output]
            write: while \( {expr} ) {

pars!
    tpar := name:WORD \: type:WORD , p:pars
        [output]
            write: {type} {name} , {p}

    par := name:WORD , p:pars
        [output]
            error: name, "parameter type not specified"

    tpar := name:WORD \: name:WORD
        [output]
            write: {type} {name}

    par := name:WORD
        [output]
            error: name, "parameter type not specified"

args!
    arg := arg:expr , a:args
        [output]
            write: {arg} , {a}

    arg := arg:expr
        [output]
            write: {arg}

term!
    call := fname:WORD ( a:args )
        [output]
            write: {fname} \( {a} )

    number := num:NUMBER
        [output]
            write: {num}

    word := word:WORD
        [output]
            write: {word}

    string := str:STRING
        [output]
            write: {str}

delim:
    eol := EOL
        [output]
            write: ;

    dedent := DEDENT
        [output]
            write: }

    indent := INDENT
        [output]
            write: {

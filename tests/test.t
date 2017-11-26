//!comment /* */ //
//!string " '


variable:
    linit := let v:WORD \: WORD = t:term EOL
    minit := mut WORD \: WORD = t:term EOL
    let := let WORD \: WORD EOL
    mut := mut WORD \: WORD EOL

if:
    if := if WORD == NUMBER EOL

func:
    call := WORD ( t:term )
    decl := WORD \: WORD ()
    def := WORD \: WORD () INDENT

return:
    ret := return t:term

term:
    number := NUMBER
    word := WORD
    string := STRING

eol:
    eol := EOL

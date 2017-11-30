!/def/comment /* */ //
!/def/string " '

variable:
    leti := let name:WORD \: type:WORD = e:expr EOL
        [output]
            write: const {type} {name} = {expr} ;

    grr :=

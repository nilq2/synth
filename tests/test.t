//!comment /* */ //
//!string " '


function:
    decl := name:WORD ( para:params ) type:type
        [output]
            :type :name ( :para )

params:
    begin := name:WORD type:type , para:params
        [output]
            :type :name , :para

    last := name:WORD type:type
        [output]
            :type :name

type:
    t := \: name:WORD
        [output]
            :name

newline:
    newl := NEWLINE

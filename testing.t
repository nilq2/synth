function:
   decl := name:WORD ( para:params ) type:type
      [generate]
         :type :name ( :para )

params:
   begin := name:WORD type:type , para:params
      [generate]
         :type :name , :para

   last := name:WORD type:type
      [generate]
         :type :name

type:
   t := \: name:WORD
      [generate]
         :name

newline:
   newl := NEWLINE

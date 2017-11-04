# Synth

The parser templating engine is at the core of Poli compilation and parsing. It defines Rules that match a concrete Pattern and replaces it with a Substitution. Source is tokenized into predefined generic Tokens which then can be used by the engine to translate.

## Format

```
Rule := Pattern
   Substitution
// Comment
```

**Rule** - the name of the rule, the alias of the pattern (see Pattern expansion).
**Pattern** - the pattern of Tokens that must match.
**Substitution** - the replacement Tokens that will be inserted in place of the pattern. It is tokenized the same way as the source.

```
     TEMPLATE     |    INPUT    |   OUTPUT
 -----------------+-------------+-----------
 example := hello | hello world | bye world
     bye          |             |
```

## Pattern rules

Longer patterns are matched first. If two or more patterns have the same length, whichever pattern comes first in the definition is matched first.

Patterns are based on Token Types and Lexemes (the string part of a Token). Token Types are denoted in all CAPITAL letters, and will only be read as such in the Pattern, while everything else is matched as a Lexeme. If a Lexeme with all CAPITALs is needed, it must be escaped with `\` like `\INTEGER`. To use `\`, it must be escaped as well using itself like `\\`.

### Token Types

-  INTEGER
-  FLOAT
-  STRING *(wrapped in `"` or `'`)*
-  WORD
-  SYMBOL *(any non-alphanumeric)*
-  INDENT
-  DEDENT
-  NEWLINE

### Aliasing

Pattern elements can be aliased using the `:` operator for later expansion/retrieval. If a Rule *`(a:rule)`* is aliased, the alias will contain the Substitution the Rule created. Aliases can be retreived in Substitution by prepending them with `:`.
To use the `:` Symbol in Pattern or Substitution, it must be escaped with `\` like `\:`.

```
          TEMPLATE        |  INPUT   |   OUTPUT
 -------------------------+----------+-------------
 rule := alias:INTEGER    | 1234567  | foo 1234567
     foo :alias           | 123 farb | farb 123
 thing := r:rule w:WORD   |          |
     :w :r                |          |
```

### Non-matching Rules

Rules can be marked as non-matching using the `!` operator. These Rules will not match patterns on their own, but can still be used within other patterns.

```
        TEMPLATE       |    INPUT    |    OUTPUT
 ----------------------+-------------+-------------
 !bar := foo a:INTEGER | foo 1234567 | foo 1234567
     bar :a            | bar 1234567 | foo 1234567
 foo := bar            |             |
 	foo                |             |
```

### Concatenation

Substitution elements can be concatenated using the `:()` operator.  This is useful for operators that consist of multiple Symbols, as the Tokenizer does not concatenate them in order to be more generic.
To use `:(` Symbol in Substitution, `:` must be escaped.

```
               TEMPLATE              |     INPUT     |      OUTPUT
 ------------------------------------+---------------+------------------
 rule := a:SYMBOL b:SYMBOL c:WORD    | thing = = bar | thing == ( bar )
     :(:a :b) (:c)                   |               |
```

### Recursion

Rule Variants can be defined with the same name multiple times, as well as refer to themselves in the Pattern. Recursive Rule should be preceded by a different Rule, Type or Lexeme, otherwise infinite recursion will occur.

```
         TEMPLATE              |      INPUT      |     OUTPUT
 ------------------------------+-----------------+---------------
 exp := INTEGER                | 123 + 456 + 789 | r o (r o (t))
    t                          | 123 + 456       | r o (t)
 exp := INTEGER SYMBOL l:exp   | 123             | t
   	r o (:l)                   |                 |
```

### Functions and Conditional Substitution

Rules can act like functions, more akin to C macros. Rules can be called with arguments using the `@` operation, values passed in will be aliased to the parameters, which then can be retreived like normal aliases. Parametrised Rules can still be retrieved normally from aliases and can be called with no arguments, which will set the parameter to an empty value.
With `:?` and `:*` operations, parameters can be used for conditional Substitution. Within `:?` operation, parameters can be compared to values using the `=` operator.  If the parameter is used without the `=` operator, it will check if it's aliased to anything. If a parameter is not aliased or doesn't match any value, the `:*` branch will be run.
Currently only 1 parameter/argument is supported.

```
if := if e:exp b:block elif:elif
	if (:e) { :b }
	@elif(1)

if := if e:exp b:block else:else
	if (:e) { :b }
	@else(1)

if := if e:exp b:block
	if (:e) { :b }

elif(if) := elif e:exp b:block elif:elif
	:? if = 1 // testing equality
		else if (:e) { :b }
		@elif(1)
	:*
		:!"elif without if"

elif(if) := elif e:exp b:block else:else
	:? if // testing if aliased, in this case either test is valid
		else if (:e) { :b }
		@else(1)
	:*
		:!"elif without if"

elif(if) := elif e:exp b:block
	:? if = 1
		else if (:e) { :b }
	:* 
		:!"elif without if"
	
else(if) := else b:block
	:? if = 1
		else { :b }
	:*
		:!"else without if"
```

### Errors and Warnings

Errors can be thrown by using the `:!` operator, mainly used in conjunction with Flag operators `:?` and `:*`.
Warnings can be shows by using the `:#` operator.

```
break(while) := break
	:? while = 1
		break;
	:*
		:!"break statement outside of a while loop!"
```

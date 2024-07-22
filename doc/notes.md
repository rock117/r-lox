A formal grammar’s job is to specify which strings are valid and which aren’t

Any time we hit a rule that had multiple productions, we just picked one
arbitrarily

BNF sample
```  
breakfast → protein ( "with" breakfast "on the side" )?
          | bread ;
protein → "really"+ "crispy" "bacon"
        | "sausage"
        | ( "scrambled" | "poached" | "fried" ) "eggs" ;
bread → "toast" | "biscuits" | "English muffin" ;
```

**Terminology**
- Literals – Numbers, strings, Booleans, and nil.
- Unary expressions – A prefix ! to perform a logical not, and - to negate a number.
- Binary expressions – The infix arithmetic (+, -, *, /) and logic (==, !=,<, <=, >, >=) operators we know and love.
- Parentheses – A pair of ( and ) wrapped around an expression.

**Grammar**
``` 
expression → literal
           | unary
           | binary
           | grouping ;
literal → NUMBER | STRING | "true" | "false" | "nil" ;
grouping → "(" expression ")" ;
unary → ( "-" | "!" ) expression ;
binary → expression operator expression ;
operator → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+" | "-" | "*" | "/" ;
```

***op***
```
Name        Operators   Associates
Equality    == !=       Left
Comparison  > >= < <=   Left
Term        - +         Left
Factor     / *          Left
Unary      ! -          Right
```

**final grammar***
``` 
expression → equality ;
equality → comparison ( ( "!=" | "==" ) comparison )* ;
comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term → factor ( ( "-" | "+" ) factor )* ;
factor → unary ( ( "/" | "*" ) unary )* ;
unary → ( "!" | "-" ) unary
      | primary ;
primary → NUMBER | STRING | "true" | "false" | "nil"
        | "(" expression ")" ;
```

A recursive descent parser is a literal translation of the grammar’s rules straight
into imperative code. Each rule becomes a function. The body of the rule
translates to code roughly like

 
```
#   Grammar notation   Code representation
    Terminal           Code to match and consume a token
    Nonterminal        Call to that rule’s function
    |                  if or switch statement
    * or +             while or for loop
    ?                  if statement
```

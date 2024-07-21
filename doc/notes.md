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
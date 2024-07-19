# data type
- Boolean
```lox
true; // Not false.
false; // Not *not* false.
```
- Number
```lox 
1234; // An integer.
12.34; // A decimal number.
```
- String
```lox 
"I am a string";
""; // The empty string.
"123"; // This is a string, not a number.
```
- Nill

There’s one last built-in value who’s never invited to the party but
always seems to show up. It represents “no value”. It’s called “null” in many
other languages. In Lox we spell it nil. 

# arithmetic
```lox
add + me;
subtract - me;
multiply * me;
divide / me;

-negateMe; // prefix operator
```

# comparison and equity
```lox 
less < than;
lessThan <= orEqual;
greater > than;
greaterThan >= orEqual;

1 == 2; // false.
"cat" != "dog"; // true.

314 == "pi"; // false
123 == "123"; // false
```
# logical operators
```lox 
!true; // false.
!false; // true.

true and false; // false.
true and true; // true.

false or false; // false.
true or false; // true.
```
# precedence and grouping
```lox 
var average = (min + max) / 2;

```
# statements
Where an expression’s main job is to produce a value,
a statement’s job is to produce an effect. Since, by definition, statements don’t
evaluate to a value, to be useful they have to otherwise change the world in
some way—usually modifying some state, reading input, or producing output.
```lox
print "Hello, world!";
```
An expression followed by a semicolon (;) promotes the expression to
statement-hood. This is called (imaginatively enough), an expression
statement.
```lox 
"some expression";
```
you can also pack a series of statements like below
```lox
{
 print "One statement.";
 print "Two statements.";
}
```
# variables
```lox 
var imAVariable = "here is my value";
var iAmNil;

var breakfast = "bagels";
print breakfast; // "bagels".
breakfast = "beignets";
print breakfast; // "beignets".
```


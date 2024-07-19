# Data Type
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

# Arithmetic
```lox
add + me;
subtract - me;
multiply * me;
divide / me;

-negateMe; // prefix operator
```

# Comparison and Equity
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
# Logical Operators
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
# Statements
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
# Variables
```lox 
var imAVariable = "here is my value";
var iAmNil;

var breakfast = "bagels";
print breakfast; // "bagels".
breakfast = "beignets";
print breakfast; // "beignets".
```
# Control Flow
- if else
```lox 
if (condition) {
 print "yes";
} else {
 print "no";
}

```
- while
```lox
var a = 1;
while (a < 10) {
 print a;
 a = a + 1;
}
```
- for
```lox 
for (var a = 1; a < 10; a = a + 1) {
 print a;
}
```
# Function
```lox 
makeBreakfast(bacon, eggs, toast);

makeBreakfast();

fun printSum(a, b) {
 print a + b;
}
```
# Closures
Functions are first class in Lox, which just means they are real values that you
can get a reference to, store in variables, pass around, etc.
```lox 
fun addPair(a, b) {
 return a + b;
}

fun identity(a) {
 return a;
}

print identity(addPair)(1, 2); // Prints "3"
```
combine local functions, first-class functions, and block scope
```lox
fun returnFunction() {
 var outside = "outside";
 fun inner() {
 print outside;
 }
 return inner;
}
var fn = returnFunction();
fn();
```
# Classes
## define class
```lox 
class Breakfast {
 cook() {
 print "Eggs a-fryin'!";
 }
 serve(who) {
 print "Enjoy your breakfast, " + who + ".";
 }
}

// Store it in variables.
var someVariable = Breakfast;
// Pass it to functions.
someFunction(Breakfast);

var breakfast = Breakfast();
print breakfast; // "Breakfast instance".
```
## Instantiation and initialzation
assign/set field to instance
```lox 
breakfast.meat = "sausage";
breakfast.bread = "sourdough";
```
access field or method in current method
```lox
class Breakfast {
 serve(who) {
     print "Enjoy your " + this.meat + " and " +
     this.bread + ", " + who + ".";
 }
 // ...
}
```
**constructor(init)**
```lox
class Breakfast {
 init(meat, bread) {
     this.meat = meat;
     this.bread = bread;
 }
 // ...
}
var baconAndToast = Breakfast("bacon", "toast");
baconAndToast.serve("Dear Reader");
// "Enjoy your bacon and toast, Dear Reader."
```
## Inheritance
```lox 
class Brunch < Breakfast {
 init(meat, bread, drink) {
     super.init(meat, bread);
     this.drink = drink;
 }
 drink() {
    print "How about a Bloody Mary?";
 }
}
```
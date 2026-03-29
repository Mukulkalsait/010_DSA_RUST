JavaScript and TypeScript are closely related, but they have key differences, particularly in how methods and method signatures are handled. TypeScript is a superset of JavaScript, meaning all valid JavaScript code is also valid TypeScript code, but TypeScript introduces additional features like static typing.

Let’s explore how methods in JavaScript and methods in TypeScript differ, and what concepts are shared between the two languages.
Key Differences and Similarities Between JavaScript and TypeScript Methods

1. Static Typing vs. Dynamic Typing

   JavaScript is a dynamically typed language, meaning method parameters and return values do not require type declarations. The types are inferred at runtime.
   TypeScript is a statically typed superset of JavaScript. It allows you to define types for method parameters, return values, and even for the method itself.

Example:

    JavaScript (Dynamic Typing):

function greet(name) {
return "Hello, " + name;
}
greet(123); // This works, but not type-safe

TypeScript (Static Typing):

    function greet(name: string): string {
        return "Hello, " + name;
    }
    greet(123);  // Error: Argument of type 'number' is not assignable to parameter of type 'string'

In TypeScript, you specify the types explicitly (name: string and string as the return type), which helps catch errors at compile time, before the code runs. 2. Method Signatures

In JavaScript, method signatures are inferred at runtime, while in TypeScript, method signatures are explicitly defined using types.
JavaScript:

const add = (a, b) => a + b;
console.log(add(5, 10)); // Output: 15

TypeScript:

const add = (a: number, b: number): number => a + b;
console.log(add(5, 10)); // Output: 15

In TypeScript, you can specify the types of parameters and the return type to ensure more robust and predictable behavior. This is useful for code completion and early error detection. 3. Optional and Default Parameters

Both JavaScript and TypeScript allow optional parameters and default values for function parameters. However, TypeScript provides more explicit type-checking for them.
JavaScript:

function greet(name = "Guest") {
return "Hello, " + name;
}
console.log(greet()); // Output: Hello, Guest
console.log(greet("Alice")); // Output: Hello, Alice

TypeScript:

function greet(name: string = "Guest"): string {
return "Hello, " + name;
}
console.log(greet()); // Output: Hello, Guest
console.log(greet("Alice")); // Output: Hello, Alice

In TypeScript, you can explicitly declare the type of the parameter (name: string), and it will enforce that only a string can be passed to this method, even if the parameter has a default value. 4. Method Overloading

JavaScript doesn't support method overloading in the traditional sense (like Java does), but you can simulate it by checking the number or types of arguments at runtime.

In TypeScript, you can define method overloads by specifying multiple signatures for a function. TypeScript will then enforce which signature is valid based on the arguments passed.
JavaScript:

function greet(name) {
if (typeof name === "string") {
return "Hello, " + name;
} else if (typeof name === "undefined") {
return "Hello, Guest";
}
}

console.log(greet("Alice")); // Output: Hello, Alice
console.log(greet()); // Output: Hello, Guest

TypeScript (Method Overloading):

function greet(name: string): string;
function greet(): string;
function greet(name?: string): string {
if (name) {
return "Hello, " + name;
} else {
return "Hello, Guest";
}
}

console.log(greet("Alice")); // Output: Hello, Alice
console.log(greet()); // Output: Hello, Guest

In TypeScript, we define multiple method signatures (overloads) and then provide a single implementation that handles all the cases. 5. Interfaces for Methods (TypeScript)

In TypeScript, you can define interfaces for methods or function signatures, which is not possible in plain JavaScript. This allows you to define a contract for the method’s parameters and return values.
TypeScript (Interface for Method Signature):

interface Greeter {
greet(name: string): string;
}

class EnglishGreeter implements Greeter {
greet(name: string): string {
return "Hello, " + name;
}
}

let greeter: Greeter = new EnglishGreeter();
console.log(greeter.greet("Alice")); // Output: Hello, Alice

Interfaces allow you to create reusable and strongly-typed contracts for method signatures, ensuring that any class or object implementing the interface follows the defined structure. 6. Arrow Functions

Both JavaScript and TypeScript support arrow functions, but the difference is that in TypeScript, the types of parameters and the return type of an arrow function can be explicitly defined.
JavaScript:

const add = (a, b) => a + b;
console.log(add(5, 10)); // Output: 15

TypeScript:

const add = (a: number, b: number): number => a + b;
console.log(add(5, 10)); // Output: 15

As you can see, in TypeScript, you can explicitly declare the types for parameters and the return type of the arrow function. 7. this Context in Methods

In JavaScript, this inside a method can change depending on how the function is called. This is particularly problematic when passing methods around as callbacks, as the context of this can be lost.

In TypeScript, this can be explicitly typed using the this keyword in method signatures to prevent unexpected behavior.
JavaScript:

const obj = {
name: "JavaScript",
greet: function() {
console.log("Hello, " + this.name);
}
};
const greetFunc = obj.greet;
greetFunc(); // 'this' is undefined in non-method call, so this.name is undefined

TypeScript:

const obj = {
name: "TypeScript",
greet(this: { name: string }) {
console.log("Hello, " + this.name);
}
};

const greetFunc = obj.greet;
greetFunc(); // Error: This expression is not callable

In TypeScript, using this: { name: string } ensures that the method only works if this has a name property. This makes this binding more predictable and avoids runtime errors. 8. Classes and Methods

Both JavaScript (ES6+) and TypeScript support classes and methods, but TypeScript provides additional type safety.
JavaScript (ES6 Class):

class Person {
constructor(name) {
this.name = name;
}

    greet() {
        return "Hello, " + this.name;
    }

}

TypeScript (Class with Types):

class Person {
name: string;

    constructor(name: string) {
        this.name = name;
    }

    greet(): string {
        return "Hello, " + this.name;
    }

}

In TypeScript, the name property is explicitly typed (name: string), ensuring that only a string value can be assigned to it, which isn't the case in plain JavaScript.
Conclusion:

    JavaScript and TypeScript share the same core syntax and method concepts, so if you already know JavaScript, you'll find TypeScript methods quite similar, but with extra features like static typing, interfaces, overloading, and better handling of this.
    TypeScript adds type annotations, type checking, and other features that help catch errors at compile time, improving the robustness and maintainability of your code.
    If you're comfortable with JavaScript methods, you'll adapt to TypeScript methods easily, especially when you understand the additional type-related features TypeScript offers.

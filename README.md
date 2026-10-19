## C±(c-plus-minus/cpm/c+-) compiler
is a ± serious compiler project that aims to create even worse C++ experience for those weird ppl who love how "std::cout" looks, having different "::", "->", "." ops and other weird things.
Tho I somehow messed up this part, so now its ± usable.

I hate rust btw.

To launch this project you need `rustc` + `cargo` installed (see https://rust-lang.org) and `gcc` installed AND added to the system PATH. or u can probably just drop the .exe/elf into the folder idk lol.

C± hello world example:
```cpm
include <std.xd>;   // contains println and print definitions
println("hello world!");
```

(for more code, see the `basic code` section!)


too easy, ik. Just wait for it, i'll make this as messed up as i can in the future.

Building a C± source:
- `cargo run [source_path]` from the rust project dir, `source_path` defaults to `main.cpm` if it is not present -> your cpm program is now compiled at `build/out[.exe]` if there were no errors!
- `c_plus_minus[.exe] [source_path]` for using a built compiler (see "building the C± compiler")

Building the C± compiler:
1. `cargo build --release` to build a compiler release
2. `.\target\release\c_plus_minus[.exe]` is your new cpm compiler!!!

## language features (why u SHOULD use C± (over C at least)):
### +:
+ it's pretty fast! (~C speed)
+ it has `stonks` operator. exciting.
+ monolithic `std.xd` lib has all you could ask for! (probably)
+ `func` definitions with overloading support (C± is better than C!!!)
+ calling a function before its definition is ok (C± is again better than C!!!)
+ `idk`/`manual` system for MORE OBVIOUS types
+ you can pay respects to vars(not yet implemented in codegen ver lol)
### -:
- it has `if`/`else` and `while` if u really want to write boring code
- it has a broken `vm`! (thats not a + but a -, C± yk)
- it does work somehow (another -)
- it is still very early WIP.

## how does it work?
just as any other low-quality compiler, ig?
1. Lexer: it goes through ur source code, reads every symbol 1-by-1 and tries to build `Tokens` out of it. basically `Token` is a smallest part of your code compiler can understand. any word, like `if`/`some_number`/`4` is a `Token` of some type. For example, in `include<std.xd>;` there are quite many of them: `include`, `<`, `std`, `.`, `xd`, `>`, `;`.
2. Parser: now it goes through every `Token` and tries to interpret them logically, building sentences that make sense for a compiler. every instruction is a node of a program, some nodes include another nodes as their children. the whole tree of those nodes is called `AST`, or `Abstract Syntax Tree`. our `include`, `<`, `std`, `.`, `xd`, `>`, `;` get read by parser and become `AST` node `Include(libname="std.xd")`. `AST` has no need for `semicolons`(`;`) anymore.
3. Typechecker/Semantic Analysis: idk if it counts as semantic analysis but lmao. Now compiler goes through every of those `AST` nodes a few more times. first, it tries to remember all the `functions` you have declared in cpm source(so it knows types of `arguments` and `return types` for later), second, it tries to check if all the `types` of `assignments` and `function args` are correct, also points out if you tried to use a `function` that is `not declared`. then the result of an analysis becomes a final tree - `Typed AST`. it contains all the type info our compiler backend needs.
4. Compiler Backend: now is the fun part. you have to turn `Typed AST` into a code that your `OS` can run. there are many ways like `interpreters`, `vms` and `binary compilers`, BUT... i dont have all the time for `binary`/`asm` backend, i wanna have a speedy speed and i love `binary builds` since they're easy to share. so i chose the `Transpiler` way. `transpiler` is a program that takes source code in your language (`C±`, for example), and turns it into code for another language (`C`, for example). so basically `C±` turns into `C` => `C` turns into `.exe`/`elf`. that's it. that's what my project does.

oh, how does `include` work? its a `preprocessor` command, `C` at its finest.

## basic code
so, first of all, C± has two `unique` ways to define a var:
`manual <type> <name>` and `idk <name>`. vars can be of `string` type (`str` or `string`), or `int` type (`int`)
for now, you should always initialize it during definition.

Example C± variable usage:
```cpm
include <std.xd>;

idk a = 4;          // if u dont know what type ur var should be,
                    // compiler will figure it out for u

manual int a2 = 4;  // if u really like boilerplate, you can force
                    // the type you want to the var

println(a);         // println automatically prints
                    // what you pass inside + newline

a = 2;              // you can assign value to a var that already exists
println(a);


idk b = "asdas";
println(b);         // println is overloaded to print both
                    // strings and ints! how convenient!
```
to define a function, you should use the `func` keyword, followed by `parameters`, `return type` and `body` of that func. to overload a func, you can use func with the same name but different args or return type. you can use return to return a value from a function, otherwise it would return 0.

Example in C±:
```cpm
include <std.xd>;

func hello_world(): int {
    println("hello world");
}

hello_world();              // will print `hello world`

// overloading hello_world
func hello_world(manual str name): int {
    print("hello ");
    println(name);
}

hello_world("Foo");         // will print `hello Foo`
hello_world();              // will print `hello world`

func mod(manual int a, manual int b): int {
    return a%b;
}

println(mod(5, 2));         // will print `1`

println(hello_world());     // will print `hello world` 
                            // and `0` in a new line.
```

to make a conditional branching, you can use `stonks`/`not_stonks` keyword. or  `if`/`else` if you love being a boring person.

`stonks` requires exactly the same structure as
```cpm
stonks (a > b) {

} not_stonks {

}
```
where `a` and `b` may be any expression you want. if you try to use other operator than `>`, you'll get a syntax error. stonks should always be stonks. also `not_stonks` part cannot be skipped, BUT can be empty.

else works just as in any other language, `else` is optional and `condition` can contain any operator.

Example in C±:
```cpm
include <std.xd>;

idk a = 4;
stonks (a > 0) {
    println("stonks");          // 4 > 0 so it'll print `stonks`
} not_stonks {
    println("not stonks");
}

// if you wanna use other ops or you're boring
if (0 < a) {
    println("boring-if");       // 0 < 4 so it'll print `boring-if`
} else {
    println("boring-else");
}
```

for `loops` C± only has `while` keyword. it repeats logic inside it's `body` while it's `condition` is true.

Example in C±:
```cpm
include <std.xd>;

idk a = 0;
// this code prints `1 2 3 4 5`
while (a < 5) {
    a = a + 1;
    print(a);
    print(" ");
}
// this code prints `fin` after loop exit
println("fin");
```

now for the fun things! to read user's input you have **2** interfaces. those are `read_int()` and `read_str()`. they take no `arguments` and `return` user's input. it's as simple, yeah.

Example in C±:
```cpm
include <std.xd>; // contains read_int and read_str definitions

// ask for user's name and age
print("enter your name: ");
idk name = read_str();

print("enter your age: ");
idk age  = read_int();

// and print them!
print("Your name is ");
println(name);

print("Your age is ");
println(age);
```

-random! yes, you can gamble in C± out of the box! and it's pretty simple, too!

Example in C±:
```cpm
include<std.xd>;            // contains random definition, wow.

println(random());          // big random number

func randlim(manual int limit): int {
    return random()%limit;  // a%b limits a to b. somehow. ik.
}

println(randlim(100));      // 0 to 99

```

`time`. look: it's awful but it works. `time()` returns amount of seconds passed since `1970's 1 jan`. i hate this, but i trust in ya.

Example in C±:
```cpm
include<std.xd>;

idk a = time();                         // get program start time
print("wait a bit and then send smth: ");
read_str();                             // to take some time
print("seconds passed: ");  
println(time()-a);                      // prints how much time has passed
```

`F`. this one worked during `VM` development but now it is just reserved for a future use with dynamic memory allocation. it does nothing so idk, dont use it lol

Example in C±:
```cpm
include<std.xd>;

idk a = 1;
F a;        // should remove var from memory/scope, does nothing for now
```

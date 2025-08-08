# ezz
ezz - Focused on customizability, ergonomics, and macroprogramming, ezz is a powerful low-level language that will ease any stress of coding on your fingers and hands, whilst also allowing you to customize the very language itself into any syntactic forms or anything else you wish!

# Warning for developers - Arbitrary Code Execution
Whilst it is still under implementation, there is a risk of libraries written in ezz that can install or perform malware using the ``api`` keyword from ezz to execute arbitrary code on your computer <i><b>WHILE COMPILING</i></b>. Please be careful if you install any unknown libraries, and please be careful using the ``api`` keyword in your own programs!

# Speaking of ``api``
``api`` is the only keyword in ezz. It allows you to call any of ezz's API functions that allow you many things, such as, emitting a target language (e.g. FASM or LLVM), interpreting ezz code at compile-time, and more.
``api`` is used to implement most of the core functions in ezz, which are included in the dev environment by default. Functions such as ``let``, ``mut``, ``if``, ``while``, ``for``, and many more are simply coded in ezz using ``api`` calls.

As a usability example:
```
fn def_variables(Arg ...)
    for a in arg
        api add_var a
    0
0
```
This example function takes a variatic containing the ezz Type Arg (arguments), and defines all of them within the scope this function is called, allowing macros in ezz.

# Contributing
This project is currently unfinished and until an alpha version is done, I'd prefer if you wait to contribute. But, if you'd really like, just make a new branch and make a PR please!

To compile this project, you simply need to clone the repo.
<a href="https://www.rust-lang.org/tools/install">Download Rustc/Cargo</a>

Build the project with ``cargo build``, test with ``cargo test``, and if you'd like to see the compiler in action, run ``make``.

Thank you for reading! If you got this far, you're a legend!

# Background

FIPS 203 defines the Module-Lattice-Based Key-Encapsulation Mechanism (ML-KEM), 
a standard for establishing a shared secret between two parties. The shared secret can be 
subsequently used to derive fresh symmetric keys for encrypting data.

ML-KEM's security rests on the computational hardness of lattice problems — specifically, 
the Module Learning with Errors (MLWE) problem. FIPS 203 specifies three parameter sets for ML-KEM, 
each representing a different trade-off between security strength and performance.


# ML-KEM-Easy
This project annotates the FIPS 203 publication and supplements it with additional material.
The FIPS 203 standard does an excellent job specifying all algorithms in fine detail. The pseudocode
is readable and can, in fact, be directly translated into working code. However, there are places 
where additional clarity would help. One way to improve readability is to add type hints to 
identifiers and mathematical objects used in the pseudocode — we have done this in the annotated 
version of each algorithm.


## Lattices, Modules, Polynomials and NTTs
One of the fundamental objects in ML-KEM is the polynomial. Polynomials in ML-KEM have degree 255. 
They are frequently added, subtracted, and multiplied throughout different algorithms. Number Theoretic 
Transforms (NTTs) provide an efficient means of performing polynomial arithmetic. We describe 
NTTs in considerable depth and connect them directly to code, covering many of their deeper and more 
interesting aspects through diagrams and code snippets in Python and Rust.

We provide additional explanation of the mathematical ideas underlying lattices, modules, and 
the Learning with Errors problem. We have includedd definitions, examples, and code snippets that 
clarify many fine details.

## Rust Implementation
The primary goal of this project is to teach ML-KEM from the ground up in a classroom setting. 
This demands building ideas from scratch, and programming is central to that goal — turning vague intuition 
into something concrete and testable. To that end, we have written a complete implementation of ML-KEM in Rust, 
intended purely for educational purposes.

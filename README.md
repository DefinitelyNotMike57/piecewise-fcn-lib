# Piecewise Functions

Provides utilities for users to create customized functions based on
sub-function building blocks.

## Why use this?

On several occasions, I have identified the need for a continuous
x-domain function over some region where different sub-regions behave
based on different polynomials.

## How to use

Add the following line to your Cargo.toml __dependendies__ section.
UPDATE THIS

The code segment below is how you would synthesize the piecewise
function in the image below.

```rust
let mut fcn = Function::new();
fcn.add_domain(subfcn::Polynomial::new(0.5,vec![-1.0,1.0],vec![-2.0,0.0,2.0],false));
fcn.add_domain(subfcn::Polynomial::new(1.0,vec![0.0,1.0],vec![0.0,4.0],false));
fcn.add_domain(subfcn::Polynomial::new(3.0,vec![0.0,1.5],vec![4.0,0.0,-4.0],false));
```

### Function 1

The equation
<img src="https://render.githubusercontent.com/render/math?math=-2x^0+0x^1+2x^2=y">
is evaluated over the region (-1.0,1.0], is scaled to a range of 0.5,
and is not inverted. This is the first domain of ```fcn```.

### Function 2

The equation
<img src="https://render.githubusercontent.com/render/math?math=0x^0+4x^1=y">
is evaluated over the region (0.0,1.0], is scaled to a range of 1.0,
and is not inverted. This is the second domain of ```fcn```.

### Function 3

The equation
<img src="https://render.githubusercontent.com/render/math?math=4x^0+0x^1-4x^2=y">
is evaluated over the region (0.0,1.5], is scaled to a range of 3.0,
and is not inverted. This is the third and last domain of ```fcn```.

![Piecewise Example](doc/example.png)

## More Details

A sub-domain is defined over the range [x<sup>1</sup> --> x<sup>2</sup>)
inclusive of x<sup>1</sup> and exclusive of x<sup>2</sup>. Each
piecewise function is defined over a sub-domain and scaled to the
duration. This was done to enable the user to identify a portion of a
function that is desirble and use it for a particular sub-domain.



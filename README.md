Bibber 🧊🧬
===========

My little toy molecular dynamics engine.

> Disclaimer:
>
> Do not ever use anything I create ever.
>
> Well, I mean, go ahead actually.
> But I cannot guarantee it is correct.
>
> With this project, I am in fact utterly sure that it is incorrect.

# About

This is basically a naive (read 'bad') implementation an canonical ensemble.
In other words, the number of particles, volume, and temperature are held constant (NVT).

The system has a constant number of particles that are randomly dispersed over the boundary volume.
The volume is held constant by... not doing anything to the boundary.
The temperature is kept constant by means of a [Berendsen thermostat](https://pure.rug.nl/ws/portalfiles/portal/64380902/1.448118.pdf) (again, my work here is still a terrible, incorrect implementation).
Under this model, the velocity vectors of all particles in the system are rescaled to bring the temperature to the set point.
If the temperature is lower than the set point, the velocities are increased, and when lower, velocities are scaled down.

# Usage

You must configure a system using a `recipe.bibber` file. See [Configuration](#configuration). This file must exist in the current working directory.

For now, _bibber_ writes a trajectory to standard out in `.gro` format.
You can redirect the stdout stream into a file.

```console
bibber > out.gro
```

After the simulation is completed, you can inspect the `out.gro` file in any way you like.
I use [pymol](https://pymol.org/).

# Installation

It's just a simple rust project, so you might already know the drill.

Clone the project. Build in release mode.

```console
git clone https://github.com/koenwestendorp/bibber
cd bibber
cargo build --release
# The executable can be found at target/release/bibber
```

To run, you can simply

```console
cargo run --release
```

You can also install it, if you like.

```console
cargo install --path .
```

Now you can run it as shown in the [Usage](#usage) section :)

# Configuration

Simulations can be specified and configured using a `recipe.bibber` file.

## Example

Here is an example.

`recipe.bibber`
```bibber
title       Our little system

start       0:ns
end         0.01:ns
timestep    10:fs
snapshot    1:ps

temperature 300:K

boundary    cubic   100:nm   100:nm   100:nm
```

## Syntax

Here is a small spec for how _bibber_ configuration is to be written and interpreted.

> Note:
> 
> This specification is incomplete and might not be updated in exact
> lockstep with the implementation as long as this little box is
> here.
> Be kind and open a small issue if you notice a mistake :)

### Entries

The possible entries are:

- `title` _string_
- `start` _time_
- `end` _time_
- `timestep` _time_
- `snapshot` _time_
- `temperature` _temperature_
- `boundary` _condition_ _length_ _length_ _length_

### Fields

The fields can be specified as follows:

- _string_: just a string.
- _time_: a value with a time unit.
- _length_: a value with a length unit.
- _temperature_: a value with a temperature unit.
- _condition_: a string describing the shape of the periodic boundary conditions (currently, only `cubic` is implemented).

### Units

A value with some unit is specified as a float followed directly (no space) by a colon (`:`) and some appropriate unit.

Any other units than those specified here are invalid.
Units cannot be omitted.

#### Time

- `:s` seconds
- `:ms` milliseconds
- `:us` microseconds
- `:ns` nanoseconds
- `:ps` picoseconds
- `:fs` femtoseconds

#### Length

- `:km` kilometers
- `:m` meters
- `:dm` decimeters
- `:cm` centimeters
- `:mm` millimeters
- `:um` micrometers
- `:nm` nanometers
- `:pm` picometers
- `:fm` femtometers

#### Temperature

- `:K` Kelvin
- `:C` Celsius (0 °C is 273.15 K)

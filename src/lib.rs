// TODO disable this
#![allow(unused)]

mod schedule;

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[cfg(test)]
mod tests {
    use super::*;
}

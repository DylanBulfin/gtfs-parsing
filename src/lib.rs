// TODO disable this
#![allow(unused)]

pub mod schedule;

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[cfg(test)]
mod tests {
    use super::*;
}

# Project has been moved to KDE Invent
Follow development on KDE's [Invent](https://invent.kde.org/libraries/cxx-kde-frameworks) or [GitHub mirror](https://github.com/KDE/cxx-kde-frameworks)

# cxx-kde-frameworks

Extends [cxx-qt](https://github.com/KDAB/cxx-qt) by adding bindings for KDE Frameworks.
Drop-In crate that takes care of building and linking against KDE Frameworks.

## Usage
 - Add `cxx-kde-frameworks` in your Cargo.toml along with `cxx-qt`.
 - Use the regular cxx-qt build system to create and build your module.
 - Wrapped KDE Frameworks modules should be available to you and you can use them freely.

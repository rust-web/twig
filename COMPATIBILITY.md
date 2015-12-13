# Differences and Compatibility

Despite all efforts for compatibility, [rust-web/twig][documentation] and [twigPHP] differ in the following aspects:

## Twig Templates

* **Encoding** - Twig Rust only supports UTF8 encoded templates. Templates must be converted prior usage.

* **Not yet implemented** - the following functionality is not yet implemented, but about to come.
 * ... almost everything. But stay tuned.

## Twig Extensions

* **Rust** - twig-rust extensions must be written in rust, instead of php. (see `src/api` in the [documentation] for details)

[twigphp]: http://twig.sensiolabs.org/documentation
[documentation]: http://rust-web.github.io/twig

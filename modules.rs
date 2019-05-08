// declare modules
mod base_mod;
mod folder_mod;
mod nested_mod;

// bring specific modules into scope and rename
use base_mod::foo as base_foo;
use folder_mod::foo as folder_foo;
use nested_mod::nested_mod_a::foo as nested_mod_a_foo;
use nested_mod::nested_mod_b::foo as nested_mod_b_foo;

fn main() {
    base_foo::outer_fn("eve");
    base_foo::bar::inner_fn();
    base_foo::bar::inner_fn_2();
    folder_foo::outer_fn("eve");
    folder_foo::bar::inner_fn();
    folder_foo::bar::inner_fn_2();
    nested_mod_a_foo::bar::inner_fn();
    nested_mod_a_foo::bar::inner_fn_2();
    nested_mod_b_foo::bar::inner_fn();
    nested_mod_b_foo::bar::inner_fn_2();
}
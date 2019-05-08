pub fn outer_fn(s: &str) {
    println!("hello from nested_mod::nested_mod_a, {}!", s);
    private_outer_fn();
}

fn private_outer_fn() {
    // do something private
}

pub mod bar {
    pub fn inner_fn() {
        super::outer_fn("alice");
        private_inner_fn();
    }

    pub fn inner_fn_2() {
        super::outer_fn("bob");
        private_inner_fn();
    }

    fn private_inner_fn() {
        // do something private
    }
}
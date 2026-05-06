macro_rules! create_print_function {
    ($func_name:ident, $type:ty) => {
        fn $func_name(x: $type) {
            println!("{}", x);
        }
    };
}

create_print_function!(print_i32, i32);
create_print_function!(print_f64, f64);
create_print_function!(print_str, &str);

fn main() {
    print_i32(10);
    print_f64(3.14);
    print_str("hello");
}
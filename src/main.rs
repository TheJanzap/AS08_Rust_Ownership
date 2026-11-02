use lc::eval::*;
use lc::term::*;

/// Driver code to run the lambda calculus evaluator.
fn main() {
    // (λx. x y) z
    let input = app(abs("x", app(var("x"), var("y"))), var("z"));

    println!("Original term: {}", input);
    let result = eval(&input);
    println!("Evaluated term: {}", result);

    // Define Church numerals
    // 2 = λf. λx. f (f x)
    // 3 = λf. λx. f (f (f x))
    let two = abs("f", abs("x", app(var("f"), app(var("f"), var("x")))));
    let three = abs(
        "f",
        abs("x", app(var("f"), app(var("f"), app(var("f"), var("x"))))),
    );

    // Define addition function for Church numerals

    let add = abs(
        "m",
        abs(
            "n",
            abs(
                "f",
                abs(
                    "x",
                    app(
                        app(var("m"), var("f")),
                        app(app(var("n"), var("f")), var("x")),
                    ),
                ),
            ),
        ),
    );

    // Add two and three
    let sum = app(app(add, two), three);

    let result = eval(&sum);
    println!("The result of adding two and three is: {}", result); // Should print 5
}

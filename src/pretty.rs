use crate::term::*;
use std::fmt;

/// Pretty prints a term.
pub fn pretty_print(term: &Term) -> String {
    // TODO: Implement pretty printing for lambda calculus terms.
    // Not sure if this format is correct
    match term {
        Term::Var(name) => name.clone(),
        Term::Abs(name, t) => format!("λ{name}. {}", pretty_print(t)),
        Term::App(l, r) => {
            // Add parentheses when left term is an abstraction
            let l = match l.as_ref() {
                Term::Abs(_, _) => format!("({})", pretty_print(l)),
                Term::Var(_) | Term::App(_, _) => pretty_print(l),
            };
            // Add parentheses when right term is an abstraction or an application
            let r = match r.as_ref() {
                Term::Abs(_, _) | Term::App(_, _) => format!("({})", pretty_print(r)),
                Term::Var(_) => pretty_print(&r),
            };
            format!("{l} {r}")
        }
    }
}

/// Display trait implementation for Term.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pretty_print(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var() {
        let name = "a";
        let input = var(name);
        let result = pretty_print(&input);
        assert_eq!(result, name)
    }

    #[test]
    fn test_abs() {
        let term = abs("x", var("a"));
        let result = pretty_print(&term);
        assert_eq!(result, "λx. a")
    }

    #[test]
    fn test_abs_abs() {
        let term = abs("x", abs("y", var("x")));
        let result = pretty_print(&term);
        assert_eq!(result, "λx. λy. x")
    }

    #[test]
    fn test_app_var_var() {
        let term = app(var("a"), var("b"));
        let result = pretty_print(&term);
        assert_eq!(result, "a b")
    }

    #[test]
    fn test_app_abs_abs() {
        let term = app(abs("x", var("x")), abs("y", var("y")));
        let result = pretty_print(&term);
        assert_eq!(result, "(λx. x) (λy. y)")
    }

    #[test]
    fn test_app_abs_var() {
        let term = app(abs("x", var("x")), var("y"));
        let result = pretty_print(&term);
        assert_eq!(result, "(λx. x) y")
    }

    #[test]
    fn test_church_numeral_3() {
        let term = abs(
            "f",
            abs("x", app(var("f"), app(var("f"), app(var("f"), var("x"))))),
        );
        let result = pretty_print(&term);
        assert_eq!(result, "λf. λx. f (f (f x))");
    }
}

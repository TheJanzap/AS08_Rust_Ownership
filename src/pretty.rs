use crate::term::*;
use std::fmt;

/// Pretty prints a term.
pub fn pretty_print(term: &Term) -> String {
    // TODO: Implement pretty printing for lambda calculus terms.
    // Not sure if this format is correct
    match term {
        Term::Var(name) => name.clone(),
        Term::Abs(name, t) => format!("(λ{name}. {})", pretty_print(t)),
        Term::App(l, r) => format!("{} {}", pretty_print(l), pretty_print(r)),
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
        assert_eq!(result, "(λx. a)")
    }

    #[test]
    fn test_app() {
        let term = app(var("a"), var("b"));
        let result = pretty_print(&term);
        assert_eq!(result, "a b")
    }
}

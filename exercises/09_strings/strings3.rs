#[allow(unused_parens)]
fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    let mut ptr: *const &str = &input;
    for i in input.chars().rev(){
       if(i == ' '){
         println!("{:?}", ptr);
        // ptr = ptr.wrapping_add(1);
        ptr = ptr.wrapping_add(1);
        let a: &str = *ptr;
        println!("{}",a);
       }
       else {
         break;
       }
       println!("{i}) ");
    }
    return input;
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
   return String::from("");
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    
    return String::from("");
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}

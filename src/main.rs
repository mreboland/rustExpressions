fn main() {
    println!("Hello, world!");



    // An Expression Language

    // Rust resembles the C family of languages, however it's a but of a ruse. In C, there's a big distinction between expressions. Bits of code like so:
    5 * (fahr-32) / 9
    // and statements, like so:
    for (; begin != end; ++begin) {
        if (*begin == target)
            break;
    }

    // Expressions have values, statements don't.

    // Rust is what is called an expression language. This means it follows an older tradition, dating back to Lisp, where expressions do all the work.

    // In C, if and switch are statements. They don't produce a value, and they can't be used in the middle of an expression. In Rust, if and match can produce values. A match expression that produces a numeric value:
    pixels[r * bound.0 + c] =
        match escapes(Complex { re: point.0, im: point.1}, 255) {
            None => 0,
            Some(count) => 255 - count as u8
        };

    // An if expression can be used to initialize a variable:
    let status = 
        if cpu.temperature <= MAX_TEMP {
            HttpStatus::Ok
        } else {
            HttpStatus::ServerError // server melted
        };

    // A match expression can be passed as an argument to a function or macro:
    println!("Inside the vat, you see {}.",
        match vat.contents {
            Some(brain) => brain.desc(),
            None => "nothing of interest"
        });



        // Blocks and Semicolons

        // Blocks, too, are expressions. A block produces a value and can be used anywhere a value is needed:
        let display_name = match post.author() {
            Some(author) => author.name(),
            None => {
                let network_info = post.get_network_metadata()?;
                let ip = network_info.client_address();
                ip.to_string()
            }
        };

        // The code after Some(author) => is the simple expression author.name(). The code after None => is a block expression. It makes no difference to Rust. The value of the block is the value of its last expression, ip.to_string().

        // Note that there is no semicolon after that expression. Most lines of Rust code do end with either a a semicolon or curly braces. As mentioned in chapt 2, when we leave the semicolon off the last line of a block, we're making that block produce a value. The value of the final expression.

        // In languages like JS, we're allowed to omit semicolons as the language auto adds them. In Rust, the semicolon actually means something.
        let msg = {
            // let-declaration: semicolon is always required
            let dandelion_control = puffball.open();

            // expression + semicolon: method is called, return value dropped
            dandelion_control.release_all_seeds(launch_codes);

            // expression with no semicolon: method is called, return value stored in 'msg'
            dandelion_control.get_status()
        };

        // This ability of blocks to contain declarations and also produce a value at the end is a neat feature, one that quickly comes to feel natural. The one drawback is that it leads to an odd error message when we leave out a semicolon by accident.
        ...
        if preferences.changed() {
            page.compute_size() // oops, missing semicolon
        }
        ...

        // Rust will error out saying:
        // mismatched types...
        // expressions_missing_semicolon....

        // Rust assumes we've omitted the semicolon on purpose. It doesn't consider the possibility that it's just a typo. A confused error message is the result. When we see expected type `()`, look for a missing semicolon first.

        // Empty statements are also allowed in blocks. An empty statement consists of a stray semicolon, all by itself:
        loop {
            work();
            play();
            ; // <-- empty statement
        }

        // Rust follows the tradition of C in allowing this. Empty statements do nothing except convey a slight feeling of melancholy. We mention them only for completeness.



        // Declarations

        // In addition to expressions and semicolons, a block may contain any number of declarations. The most common are let declarations, which declare local variables:
        let name: type = expr;

        // The type and initializer are optional, the semicolon is required.

        // A let declaration can declare a variable without initializing it. The variable can then be initialized with a later assignment. This is occasionally useful because sometimes a var should be initialized from the middle of some sort of control flow construct:
        let name;
        if user.has_nickname() {
            name = user.nickname();
        } else {
            name = generate_unique_name();
            user.register(&name);
        }

        // There are two diff ways to the local var name might be initialized. But either way it will be init exactly once, so name does not need to be declared mut.

        // It's an error to use a var before it's initialized. We may occasionally see code that seems to redeclare an existing var, like so:
        for line in file.lines() {
            let line = line?;
            ...
        }

        // This is equivalent to:
        for line_result in file.lines() {
            let line = line_result?;
            ...
        }

        // The let declaration creates a new, second var, of a different type. The type of line_result is Result<String, io::Error>. The second variable, line, is a String. It's legal to give the second var the same name as the first. In this book, for this situation, we'll use _result suffix so vars have distinct names.

        // A block can also contain item declarations. An item is simply any declaration that could appear globally in a program or module, such as a fn, struct, or use. Items will be covered in a later chapter, using fn is sufficient enough an example. Any block may contain a fn:
        use std::io;
        use std::cmp::Ordering;

        fn show_files() -> io::Result<()> {
            let mut v = vec![];
            ...

            fn cmp_by_timestamp_then_name(a: &FileInfo, b: &FileInfo) -> Ordering {
                a.timestamp.cmp(&b.timestamp) // first, compare timestamps
                    .reverse() // newest file first
                    .then(a.path.cmp(&b.path)) // compare paths to break ties
            }

            v.sort_by(cmp_by_timestamp_then_name);
            ...
        }

        // When a fn is declared inside a block, its scope is the entire block. That is, it can be used throughout the enclosing block. But a nested fn cannot access local variables or arguments that happen to be in scope. For example, the function cmp_by_timestamp... could not use v directly (Rust has closures, which do see into enclosing scopes, covered in chap 14). A block can even contain a whole module.



        // if and match

        // The form of an if expression is familiar:
        if condition 1 {
            block1
        } else if condition2 {
            block2
        } else {
            block_n
        }

        // Each condition must be an expression of type bool. Rust, true to form, does not implicitly convert numbers or pointers to Boolean values. Parentheses are not required, the curly braces however, are.

        // The else if blocks, as well as the final else, are optional. An if expression with no else block behaves exactly as though it had an empty else block.

        // match expressions:
        match code {
            0 => println!("OK"),
            1 => println!("Wires Tangles"),
            2 => println!("User Asleep"),
            _ => println!("Unrecognized Error {}", code)
        }

        // Exactly one of the four arms of this match expression will execute, depending on the value of code. The wildcard pattern _ matches everything, so it serves as the default case.

        // The compiler can optimize this kind of match using a jump table. A similar optimization is applied when each arm of a match produces a constant value. In that case, the compiler builds an array of those values, and the match is compiled into an array access. Apart from a bounds check, there is no branching at all in the compiled code.

        // The versatility of match stems from the variety of supported patterns that can be used to the left of => in each arm. Above, each pattern is simply a constant integer. We've also shown match expressions that distinguish two kinds of Option value:
        match params.get("name") {
            Some(name) => println!("Hello, {}", name),
            None => println!("Greetings, stranger.")
        }

        // A pattern can match a range of value. It can unpack tuples. It can match against individual fields of structs. It can chase references, borrow parts of a value, and more. Rust's patterns are a mini-language of their own. More in chapter 10.

        // The general form of a match expression is:
        match value {
            pattern => expr,
            ...
        }

        // The comma after an arm may be dropped if the expr is a block.

        // Rust checks the given value against each patter in turn, starting with the first. When a pattern matches, the corresponding expr is evaluated and the match expression is complete. No further patterns are checked. At least one of the patterns must match. Rust prohibits match expressions that do not cover all possible values:
        let score = match card.rank {
            Jack => 10,
            Queen => 10,
            Ace => 11
        }; // error, non-exhaustive patterns

        // All blocks of an if expression must produce values of the same type:
        let suggested_pet =
            if with_wings { Pet::Buzzard } else { Pet::Hyena }; // ok

        let favourite_number =
            if user.is_hobbit() { "eleventy-one" } else { 9 }; // error

        let best_sports_team =
            if is_hockey_season() { "Predators" }; //error

        // The last example is an error because in July, the result would be ().

        // Similarly, all arms of a match expression must have the same type:
        let suggested_pet =
            match favourites.element {
                Fire => Pet::RedPanda,
                Air => Pet::Buffalo,
                Water => Pet::Orca,
                _ => None // error, incompatible types
            };

        // if let
        if let patter = expr {
            block1
        } else {
            block2
        }

        // The given expr either matches the pattern, in which case block 1 runs, or it doesn't, and block2 runs. Sometimes this is a nice way to get data out of an Option or Result:
        if let Some(cookie) = request.session_cookie {
            return restore_session(cookie);
        }

        if let Err(err) = present_cheesy_anti_robot_task() {
            log_robot_attempt(err);
            politely_accuse_user_of_being_a_robot();
        } else {
            session.mark_as_human();
        }

        // It's never strictly necessary to use if let, because match can do everything if let can do. An if let expression is shorthand for a match with just one patter:
        match expr {
            patter => { block1 }
            _ => { block2 }
        }

        
}

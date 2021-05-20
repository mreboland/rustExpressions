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
}

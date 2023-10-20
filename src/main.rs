use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Parser;
use logos::Logos;

/// Inline an entire crate into a single file
///
/// Aimed at making it easy to distribute reproducers, or run minimizers.
///
/// If an output path is passed, then outputs to the file, otherwise outputs to stdout.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about)]
struct CliArgs {
    /// The input crate root
    crate_root: PathBuf,
    /// The output file (default: stdout)
    output_file: Option<PathBuf>,
    /// Force writing, even if the file exists
    #[clap(long = "force", short = 'f')]
    force_output: bool,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
enum Token {
    #[token("mod")]
    Mod,
    #[token(";")]
    Semi,
    #[regex(r"[a-zA-Z0-9_]+")]
    Identifier,
    #[regex(r"[^ \t\n\fa-zA-Z0-9_]")]
    Misc,

    /// A comment token that will never be retuend by the lexer
    #[regex("//.*\n", logos::skip)]
    #[token("/*", multiline_comment_skip)]
    Comment,
}

fn multiline_comment_skip(lex: &mut logos::Lexer<Token>) -> logos::FilterResult<(), ()> {
    match lex.remainder().find("*/") {
        Some(posn) => {
            // Skip ahead to the end of the comment directly
            lex.bump(posn + 2);
            logos::FilterResult::Skip
        }
        None => logos::FilterResult::Error(()),
    }
}

fn flatten(base: &Path, contents: String) -> Result<String> {
    // Find locations with `mod foo;` in contents, and replace with `mod foo { <expanded foo> }`

    let mut ret = String::new();

    let mut last_copied_over = 0usize;
    let mut lex = Token::lexer(&contents);

    while let Some(token) = lex.next() {
        if !matches!(token, Ok(Token::Mod)) {
            continue;
        }

        // Push over everything up to the start of the module declaration
        ret.push_str(&contents[last_copied_over..lex.span().start]);
        last_copied_over = lex.span().start;

        // Now we're at the start of a module declaration
        let module_tok = lex.next().unwrap().unwrap();
        if !matches!(module_tok, Token::Identifier) {
            // Not actually a module declaration, don't do anything, carry on
            continue;
        }
        let module_name = lex.slice();

        // Only continue if actually a module declaration
        let semi_tok = lex.next().unwrap().unwrap();
        if !matches!(semi_tok, Token::Semi) {
            // Not a module declaration, don't do anything, carry on
            continue;
        }

        // Grab the expanded module
        let module_path = {
            let p = base.join(format!("{}.rs", module_name));
            if p.is_file() {
                p
            } else {
                base.join(module_name).join("mod.rs")
            }
        };
        let expanded = expand(&module_path)?;

        // Push it over
        ret.push_str("mod ");
        ret.push_str(module_name);
        ret.push_str("{\n");
        ret.push_str(&expanded);
        ret.push_str("\n}\n");

        // Mark as copied over
        last_copied_over = lex.span().end;
    }

    ret.push_str(&contents[last_copied_over..]);

    Ok(ret)
}

fn expand(p: &Path) -> Result<String> {
    assert!(p.is_file());
    let base = p.parent().unwrap();
    let contents = std::fs::read_to_string(p).unwrap();
    flatten(base, contents)
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    if !args.crate_root.is_file() {
        anyhow::bail!(
            "Not a valid crate root. Expected something like foo/lib.rs or foo/main.rs or foo.rs"
        )
    }

    let root = args.crate_root.canonicalize().unwrap();
    let ret = expand(&root)?;

    if let Some(out) = args.output_file {
        if args.force_output || !out.exists() {
            let mut outfile = std::fs::File::create(out).unwrap();
            write!(outfile, "{ret}").unwrap();
            Ok(())
        } else {
            anyhow::bail!("Not writing to existing file. Pass `--force` to force")
        }
    } else {
        println!("{ret}");
        Ok(())
    }
}

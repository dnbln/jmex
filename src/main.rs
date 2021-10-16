use std::{error::Error, fs::File, path::PathBuf};

use clap::Clap;
use jmex::Input;

#[derive(Clap, Debug)]
pub struct App {
    /// Code to execute
    #[clap(default_value = ".")]
    exe: String,
    /// File to read input from, or stdin if not specified
    file: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let app: App = App::parse();
    dbg!(&app);

    let input = match app.file.as_ref() {
        Some(f) => Input::File(File::open(f)?),
        None => Input::Stdin(std::io::stdin()),
    };

    struct Vis;

    impl jmex::Visitor for Vis {
        fn visit_object(&mut self, branch: &jmex::ParserBranch) -> jmex::VisitorAction {
            println!("Visiting object at {:?}", branch);
            jmex::VisitorAction::Recurse
        }

        fn visit_object_end(&mut self, branch: &jmex::ParserBranch) {
            println!("Visited object at {:?}", branch);
        }

        fn visit_property(&mut self, branch: &jmex::ParserBranch) -> jmex::VisitorAction {
            println!("Visiting property at {:?}", branch);
            jmex::VisitorAction::Recurse
        }

        fn visit_array(&mut self, branch: &jmex::ParserBranch) -> jmex::VisitorAction {
            jmex::VisitorAction::Recurse
        }

        fn visit_array_end(&mut self, branch: &jmex::ParserBranch) {}

        fn visit_array_element(&mut self, branch: &jmex::ParserBranch) -> jmex::VisitorAction {
            jmex::VisitorAction::Recurse
        }

        fn visit_number(&mut self, branch: &jmex::ParserBranch, num: f64) {}

        fn visit_bool(&mut self, branch: &jmex::ParserBranch, b: bool) {}

        fn visit_null(&mut self, branch: &jmex::ParserBranch) {}

        fn visit_str_strategy(
            &mut self,
            branch: &jmex::ParserBranch,
        ) -> Option<jmex::VisitStrStrategy> {
            None
        }

        fn visit_str_chunks_begin(&mut self, branch: &jmex::ParserBranch) {}

        fn visit_str_chunk(&mut self, branch: &jmex::ParserBranch, chunk: &[u8]) {}

        fn visit_str_chunks_end(&mut self, branch: &jmex::ParserBranch) {}

        fn visit_str(&mut self, branch: &jmex::ParserBranch, s: String) {}
    }

    jmex::run(input, app.exe, &mut Vis)?;

    Ok(())
}
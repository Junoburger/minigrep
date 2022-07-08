use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    #[derive(Debug)]
    struct Uploader {
        id: Option<i32>,
        files: Vec<File>,
        state: UploaderState,
    }

    #[derive(Debug)]
    enum UploaderState {
        Intake,
        Chunking,
        Uploading,
        Complete,
        Extracting,
        Done,
    }

    impl Uploader {
        fn new() -> Uploader {
            Uploader {
                id: None, // Should be a cryptographic hash
                files: Vec::new(),
                state: UploaderState::Intake,
            }
        }

        fn intake(&mut self) {
            self.state = UploaderState::Intake;
        }

        fn chunking(&mut self) {
            self.state = UploaderState::Chunking;
        }

        fn uploading(&mut self) {
            self.state = UploaderState::Uploading;
        }

        fn complete(&mut self) {
            self.state = UploaderState::Complete;
        }

        fn extracting(&mut self) {
            self.state = UploaderState::Extracting;
        }

        fn done(&mut self) {
            self.state = UploaderState::Done;
        }
    }
}

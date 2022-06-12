use filedescriptor::FileDescriptor;
use StateMachine::*;

enum StateMachine {
    Chunk0(),
    Chunk1(String),
    Chunk2(String),
    Chunk3(FileDescriptor),
    Chunk4(),
}

enum AsyncResult {
    Ready,
    NotReady,
}

fn executor() {
    let mut state = StateMachine::Chunk0();
    /*
    rest...
    */
}

fn write_bytes_new_file(state: &mut StateMachine) -> AsyncResult {
    match state {
        Chunk0() => {
            let extension = ".log";
            *state = StateMachine::Chunk1(extension.to_string());
            return AsyncResult::NotReady;
        }
        Chunk1(extension) => {
            let file_name_res: Option<String> = get_file_name_from_user();
            if let Some(file_name) = file_name_res {
                let full_name = file_name + extension;
                *state = StateMachine::Chunk2(file_name);
            }
            return AsyncResult::NotReady;
        }

        Chunk2(full_name) => {
            let file_descriptor = create_file(full_name);
            if file_descriptor.is_some() {
                println!("File created!");
                *state = StateMachine::Chunk3(file_descriptor);
            }
            return AsyncResult::NotReady;
        }
        Chunk3(file_descriptor) => {
            let result = open_write_bytes(file_descriptor);
            if result.is_some() {
                println!("Write successful!");
                *state = StateMachine::Chunk4()
            }
            return AsyncResult::NotReady;
        }
        chunk4 => return AsyncResult::Ready,
    }
}

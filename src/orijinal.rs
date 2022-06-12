use filedescriptor::FileDescriptor;

async fn write_bytes_new_file() -> u32 {
    let extension = ".log";
    let file_name = get_file_name_from_user().await;
    let full_name = file_name + extension;
    let file_descriptor = create_file(full_name).await;
    println!("File created!");
    return open_write_bytes(file_descriptor).await;
    println!("Write successful!");
}

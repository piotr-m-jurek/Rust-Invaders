pub fn format_size (size: u32) -> String {
  
    if size >= 1024 * 1024 {
        format!("{}MB", size / 1024 / 1024)
    } else if size >= 1024 {
        format!("{}kB", size / 1024)
    } else {
        format!("{}B", size)
    }

}

#[cfg(target_os = "windows")]
pub fn hello() { println!("Hello, I'm Windows!"); }

#[cfg(target_os = "linux")]
pub fn hello() { println!("Hello, I'm Linux!"); }

#[cfg(target_os = "macos")]
pub fn hello() { println!("hello, I'm MacOS."); }

fn main(){
    hello();
}

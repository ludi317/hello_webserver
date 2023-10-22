
>Chapter 20 of the Rust book gives a step-by-step guide to building a simple web server. As the final project, it exercises many Rust concepts taught in the book. The server is hosted at localhost:7878 and returns plain HTML without external resources, distributing work across multiple threads.
> 
>
>The story should end here, but there’s a problem. The code, as copied from the book, panics when requests are made from a Chrome browser.  Requests made from curl or Firefox work fine.
>
>This repo is a patched web server that fixes the panic. I’ve shared it for the benefit of those who might encounter this error. I’ve also investigated why Chrome, but not Firefox, causes the server to crash.


See my [blog](https://medium.com/@ludirehak/why-chrome-crashes-the-rust-books-web-server-30265b18d32c) for the thrilling conclusion.

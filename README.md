### Very basic authentication in a request

The `main` function is the entry point of the program and returns a `Result<(), Error>`, indicating whether the operation was successful or returned an error related to HTTP requests.

A new `reqwest::blocking::Client` instance is created using the `new()` method. 
This client will be used to make HTTP requests synchronously.
A username (`user`) and an optional password (`passwd`) are defined. 
In this case, `passwd` is set to `None`, indicating that no password is provided.

The client sends a GET request to the URL `"http://httpbin.org/get"`. The `basic_auth()` method is used to add basic authentication to the request. 
It takes a username and an optional password as arguments. In this case, `user` is the username, and `passwd` is the optional password. 

The `send()` method sends the request synchronously.
The `response` variable holds the response from the server, which is of type `Result<Response, Error>`.
The `response` variable is printed to the console. This will print a debug representation of the response, including its status code, headers, and body.

**Overall, this code demonstrates how to:**

✓ make a synchronous GET request to a URL using the `reqwest` crate in Rust.

✓ It also shows how to add basic authentication to the request and handle the response.

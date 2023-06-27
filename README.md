# AsynNetwork
This is a test program to use tokio in rust: a echo server and a client
The AsynServer is a rust program that receives messages from the client, using splitted I/O. When its reader task receives a message from client, it sends the message to the writer thread through channel, then the writer send the message back to the client.
The AsynClient sends a message to the server and receives the echo from the server in a seperate task.

# rusty_m42
A webserver+client for matrix42/empirium ticketing system that hopefully won't suck.
I wanted to learn Rust, so I've put that into practice interfacing with
the public API to modify and read data and make a web client that 
can mass filter and edit/delete data.

This is made so that admins who (definitely) know what they are doing can directly filter fields with regex.
We can probably log in to the server running the software and modify the database directly
but it's all proprietary and even the person responsible doesn't know how anyways.
I saw that as opportunity to build this client to practice and maybe produce something
semi-useful out of it.

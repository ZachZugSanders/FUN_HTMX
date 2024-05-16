# Lunch and Learn HTMX

## What is HTMX?

ANOTHER JS library! This one enables web applications to use Hypermedia as the Engine of Application State (HATEOAS), a not so new concept from REST application architecture.

## Why did I choose to learn this library?

- I wanted to learn more about web application architecture.
- Its easy to write bad Javascript code and I am confident in my ability to write bad Javascript code.
- It enabled me to write most of the UI logic in python.
- My YouTube algorithm recommended videos about it.
- The use case for the application I wanted to write fits  well into this library's primary purpose.

## What does a RESTful application mean?

Representational State Transfer (REST):

1. Uniform Interface
2. Client-Server Separation
3. Resource-based
4. Resource Representation
5. Self-descriptive Messages
6. HATEOS

Below outlines Richardson Maturity Model (RMM) which is a framework for evaluating the maturity of web services in terms of their adherence to RESTful principles:

- The first level of REST architecture, also known as "The Swamp of POX (Plain Old XML)", involves using HTTP as a transport system for remote interactions, but without leveraging any of the system's built-in capabilities such as methods, status codes, or caching. It's essentially using HTTP as a tunnel for your own remote interaction design.

- The second level of REST architecture, known as "Resources", involves breaking down the service into distinct resources. Each resource is identified by a specific URI (Uniform Resource Identifier). This level introduces the concept of addressing and interacting with resources individually, which is a key aspect of the REST architectural style.

- The third level of REST architecture, known as "HTTP Verbs", involves using the standard HTTP methods like GET, POST, PUT, DELETE, etc. in a way that's consistent with their defined semantics. This level leverages the built-in capabilities of HTTP to perform CRUD (Create, Read, Update, Delete) operations, making the API more intuitive and aligned with the protocol's design.

- The fourth level of REST architecture, known as "Hypermedia Controls", involves including hypermedia within the response to guide the client to other resources. This level introduces the concept of HATEOAS (Hypermedia as the Engine of Application State), which means that the client interacts with the server entirely through hypermedia provided dynamically by application servers. This makes the API self-descriptive and allows the client to navigate the API dynamically, as opposed to hard-coding paths to resources.

![alt text](image.png)

As RMM is defined we can look at most web applications written these days and say that they are at level 2 with no intention of even trying to achieve the third level. Why is that?

Well, the emergence of JSON to replace SOAP/XML-RPC and all the flavors of the JS framework rainbow have made it really easy to couple the backend and frontend to make thick client web apps, because who doesn't love implementing things twice!

---

Lets run this application!

```RUST_LOG=debug cargo run```

# Introduction & Motivation
This project is an attempt to increase my coding skills while producing something that I can get behind at that provides a value to me. I'm not very good at coding a calculator, when instead I could press the Win-key, type "calc" and hit enter. I am however very good at immersing myself into the projects that have meaning.
So I created meaning, by writing a webpage of my own with a bit of html and css. For that webpage I set out to create a contact form, which now needs some means of actually delivering the content and data of that form to me for further action (like anyone is ever gonna send ME a mail, hue!). This is where the form-handler coems into play. It connects my webpage to my mail inbox and provides real benefit: Exposing myself to the net without giving my private email address away openly.

# Scope
With the goal set, let's look at some key deliverables that I have elicited so far (Mh yeah, fancy BA talk comming right in!).
The application needs to be able to:

**Listen.**  to a port where the content form is routed to by the webpage. The latter isn't yet in place, but will eventually be.
**Deconstruct.** the http message it receives into the parts that we actually care about and store the form's content in memory.
**Formulate.** an smtp message that it can deliver to my inbox.
**Communicate.** that message via SMTP protocol to my inbox.

In addition to these functional requirements, I also would like the aplication to:

**Ship.** as Docker Container. This is less a real requirement, as there is one lousy pc hosting the webpage that is up like not even 20% of the time currently, and while stting up a server is a plan of mine, that plan is in the future. I am blatantly using this requirement to force myself to getting to know Docker.
**Rely.** on as little crates outside the std environment of Rust as possible. The application would probably be writable in like 100 lines with crates that provide all the necessary functions and structs for me. But where's the learning in that? So no tokio (questionable if I even need any async functionality), smtp, oauth crates. I wanna do it myself.

And some examples of what is specifically out of scope for this project:

- **Flexibility.** to http messages. There is no intention of this service to ever take in any other kind of communication than the specific form on my webpage. Any other incoming message will therefore be met with a 400 Bad Request response, and terminating the connection gracefully.
**Data and Computation.** will be handled the same, like corrupted data or any errors from underlying crates. If a std::String::fromUtf8() call throws an exception, it won't be handled. It will be met with a 400 Bad Request or 500 Internal Server Error response, as applicable, and terminating the connection gracefully. This is mainly due to knowledge restraints as to how to navigate these cases. Maybe in the next project. *It's a learnin' process, aight people?*

# Roadmap
Items listed here do not follow any chronological order, are not sorted by t-shirt size, don't aim to be exhaustive and aren't guaranteed to be put into the final product. They are a mere collection of things that crossed my mind. That being said, I will prioritize using practices and techniques at the edge of my understanding over coding style over features equal to bug fixes.
## Features

## Issues

## Refactoring
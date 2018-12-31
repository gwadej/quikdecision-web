# QuikDecision Web

Web interface over the QuikDecision library I wrote in Rust. (See
[QuikDecision Rust](https://github.com/gwadej/quikdecision_rust) for details.)
The web application is based on the Hyper library. I was looking to build
this with minimal library help to learn about doing web applications in
Rust, without having the library do too much of the work.

A running version of the application is available at [QuikDecision](http://qd.gwadej.org/).

## Architecture

The main user interface is provided as a single HTML file served from the
root of the web app. The functionality of the application is provided by
a JSON-based API accessed by the UI.

In a fit of definite overkill, I've added an OpenAPI description of the API
to the application.

The active portion of the UI is provided through jQuery. Similar to the decision
about the Hyper Web Framework, I've been out of the JavaScript UI game for
a long while. Rather than spend a lot of time digging through multiple JS
frameworks, I struck with functionality a little more advanced that what I used
years ago.

## Project Goals

The main goal was to explore programming in Rust. I keep coming back to different
portions of the library/application stack as I learn more idiomatic Rust. I have
been continuing to improve the code and in a few cases add new functionality as
it becomes needed.

## Future Directions

At some point, I will probably replace the Hyper web framework with something
more full-featured. I may also replace the relatively simple JavaScript code
with more powerful libraries.

As ideas come to me for other approaches to quick decision making, I may add
new functionality. Since the overriding concern of the applications is fun,
upgrades will probably be sporadic.

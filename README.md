THE SOFTWARE IS PROVIDED "AS IS" AND BRIAN SMITH AND THE AUTHORS DISCLAIM
ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES
OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL BRIAN SMITH OR THE AUTHORS
BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY
DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

*circ*
======

*circ* is a verbally opinionated, easy-to-use command-line utility for using
Brian Smith's *ring* cryptography library:

> *ring* is focused on the implementation, testing, and optimization of a core
> set of cryptographic operations exposed via an easy-to-use (and hard-to-misuse)
> API. *ring* exposes a [Rust](https://www.rust-lang.org/) API and is written in
> a hybrid of Rust, C, and assembly language.

Principle goals of *circ*:

1. Provide example usages of the *ring* API in code. At the time of writing,
  the *ring* examples are far too sparse for developers to understand how to
  realistically use it. *circ* will provide many individual examples of using
  *ring* that you can copy for your own applications. In order to achieve this
  goal, extensive code comments can be found throughout any usage of *ring*
  APIs to explain this usage.
2. Use sane cryptographic defaults, and let you know through passive-aggressive
  warnings when you appear to be doing something silly...
3. Produce beautiful output at the command line for each operation so it is
  easy to understand what is going on, and what has happened after each
  command.

*circ* is a work in progress. It is not recommended for use.
-------

License
-------

BSD 3-clause for this library, *ring* uses a different license. See [LICENSE](LICENSE).
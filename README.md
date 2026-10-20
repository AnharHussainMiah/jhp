# JHP: JavaScript Hypertext Processor

![](logo.jpg)

![OS](https://img.shields.io/badge/OS-Linux%20%7C%20Mac%20%7C%20WSL-blue)
![License](https://img.shields.io/badge/License-CCAI%20License%20v1.0-green)
![Release](https://img.shields.io/badge/Release-v0.1.0-pink)
![Language](https://img.shields.io/badge/Language-Rust-orange)
![Arch: Single Binary](https://img.shields.io/badge/Arch-Single%20Binary-yellow)
![Code: Human hand crafted](https://img.shields.io/badge/Code-Human%20hand%20crafted-blueviolet)

<!-- ![VSCode Plugin: Manual install only](https://img.shields.io/badge/VSCode%20Plugin-Manual%20install%20only-blue) -->

## Kinda like PHP but with JavaScript instead...

I got up one morning thinking, us devs hate JavaScript and PHP, what if we combined them together so that we had something that worked just like PHP but with JavaScript as the templating language instead? and thus JHP was born!

And works pretty much as you would expect it!

## But serioulsy why?!

On a semi serious note, `jhp` at the moment compiles down to around ~~3mb~~ 7mb, although I haven't tried compiling under alpine, imagine building a full blown web app server container image that's under 10mb! and now compare that to a `php` container image and its around 400mb. You can get that down to around 150-200mb if you use something like Alpine + Nginx and lots of elbow grease, but honestly that's still huge!

Using Node, Deno or Bun is kinda bad as well, with Node + Alpine its around 250mb, while Deno and Bun can be squeezed further to around 50mb, but that's still big!

Size issues aside, the other main point is we get to use vanilla JavaScript on the server side template engine, maybe this was what `nodejs` should've been in the first place? 😂

And unlike `php` everything is a fully self contained single binary, so no more messing around and integrating with Apache or Nginx.

## Ok enough talk, show me the code!

Instead of having `.php` files we have `.jhp` files, and they work the same way, instead of using `<?php` tags, use `<?jhp` and instead of writing `php` we write vanilla JavaScript and then just dump
the "website" files into a `public` folder including all your `.jhp` files.

**Example:**

A `index.jhp` file can contain normal HTML with JavaScript embedded anywhere you need it just like in PHP:

```html
<h1>Welcome!</h1>

<?jhp const name = "this is a serioulsy bad idea!"; ?>

<p>Hello, <?jhp console.log(name); ?>.</p>
```

assuming you have the `jhp` binary compiled and installed on your machine, as long as the `public` folder is in your
current directory just run `jhp` and it will start serving the `public` folder, you should see `jhp` booted up:

```shell


            ╔═══╗ ╔═╗   ╔═╗ ╔═══════╗
            ╚═╗ ║ ║ ║   ║ ║ ║ ╔═══╗ ║
              ║ ║ ║ ╚═══╝ ║ ║ ╚═══╝ ║
        ╔═╗   ║ ║ ║ ╔═══╗ ║ ║ ╔═════╝
        ║ ╚═══╝ ║ ║ ║   ║ ║ ║ ║
        ╚═══════╝ ╚═╝   ╚═╝ ╚═╝

    What nodejs should've been (maybe?)
                v0.1.0


Serving ./public on http://0.0.0.0:8080

```

And now nagivate to it in your browser `http://localhost:8080/index.jhp` and bask in the glory that is `JHP` rendered awesomeness.

Another example `fruits.jhp` :

```
<h1>Hello World!</h1>

<h2>Fruits:</h2>

<?jhp

const data = ["avacado", "plum", "strawberry", "mango", "banana","cherry", "apple"];

for(var i in data) {
    console.log(data[i] + "<br />");
}

?>
```

## Accessing headers

Headers are available in the `HEADERS` super global json object, for example:

```javascript
const accept = HEADERS["accept"];

console.log(accept);
// returns e.g "text/html"
```

## Accessing Method, Uri, Path, and QueryString

Uri, path and querystring are available in super global objects as shown below:

```javascript
console.log(METHOD);
console.log(URI);
console.log(PATH);
console.log(QUERYSTRING);
```

## Accessing Environment variables

Environment variables are available in the `ENV` super global json object, for example:

```javascript
const conn = ENV["PG_CONNECTION_STRING"];

console.log(conn);
```

## Accessing the body data

The body payload is available in the `BODY` super global _(please note 1mb max size)_, for example:

```javascript
console.log(BODY);
```

## Returning custom headers and HTTP Status Code

There is a super global object called `RESPONSE` that has the following stucture:

```json
{
  "status": 200,
  "headers": {
    "content-type": "text/html; charset=utf-8",
    "cache-control": "no-cache"
  },
  "body": ""
}
```

Simply mutate this object to set the status code and headers that you want to return to the client browser, example:

```javascript
RESPONSE.status = 404;
RESPONSE.headers["x-custom-header"] = "some custom value here";
```

> [!NOTE]
> doing `console.log` appends to the `RESPONSE.body` automatically, so this does not need to be mutated manually, unless you wanted to forcefully overide the template parsing. Generally you should never need to overwrite this manually.

## Host function: Logging

If you want to emit to the `stdout` for logging purposes, confusingly we don't use `console.log` as this actually appends to the data sent back to the browser, think of `console.log` as the same as `echo` in `PHP`.

You can use the global function `LOG` to log and print out to the actual console e.g:

```javascript
LOG("user credentials are invalid!");

// will print to the terminal:
// STDOUT| user credentials are invalid!
```

## Host function: HTTP Get/Post

Send a HTTP GET using the `FETCH` super global function for example:

```javascript

// FETCH(url, headers), headers has to passed as string, just use JSON.stringify

const result = FETCH("https://httpbin.org/get", "{}");

// if the fetch fails, you will get an empty string
if (result != "") {
  let response = JSON.parse(result);
  console.log(response.body);
}

// the returned string when parsed into json is the structure:

{
    status: 200,
    headers: {
        "accept": "application/json",
        "cache": "no-cache"
    },
    body: 'data'
}
```

Send a HTTP POST using the `POST` super global function for example:

```javascript

// POST(url, headers, body), headers has to passed as string, just use JSON.stringify

const result = FETCH("https://httpbin.org/get", "{}");

// if the post fails, you will get an empty string
if (result != "") {
  let response = JSON.parse(result);
  console.log(response.body);
}

// the returned string when parsed into json is the structure:

{
    status: 200,
    headers: {
        "accept": "application/json",
        "cache": "no-cache"
    },
    body: 'data'
}
```

## Host function: PostgreSQL

[TODO]

## Host function: File I/O

File I/O is done using the `slurp`, `spew`, and `pour` super global functions for example:

```javascript
// read the contents of a file into a string
const contents = SLURP("filename.txt");

// write a file called "test.md" with the string contents "some text"
// this returns a boolean, true if the operation worked

const is_file_written = SPEW("test.md", "some text");
// true

// if you want to append to an existing file (or it will create a new file if one does not exist)
POUR("log.txt", "some logging");
POUR("log.txt", "some more logging");
POUR("log.txt", "evn more logging!");
```

## Imports aka "includes"

`jhp` wouldn't be complete without `php` "include" style template imports.

In `jhp` the syntax to include or "import" another `jhp` file into the current file is to use the following
syntax:

```html
<# components/button.jhp #>
```

This behaves the same as `php`'s "include" however unlike `php` this automatically detects
cyclic references and simply stops processing them if there are infinite cycles, this is why we
don't need to worry about `include` vs `include_once` etc in `jhp`.

> [!NOTE]
> **IMPORTANT:** when pointing to a file you must use the "absolute" path that is relative to the `public` folder. So for example if you had a file that was `public/lib/util.jhp` then that file would always be referenced as `<# lib/util.jhp #>`, this helps remove all the annoying issues in `php` when files are moved that end up breaking relative imports, hopefully this is way saner and consitent!

## Global Handler

When building SPA _(Single Page Applications)_ they usually have "virtual pages" that are "links" to pages
that don't actually exist for example `/user-account`, normally the SPA will hijack these links client side and then use the browser history API to add the link as visited. This works fine until these links are shared the page is refreshed, at this point the browser will try and fetch this page that doesn't actually exist.

In `jhp` if you drop a `global.jhp` file into the root folder (the `public/` directory), then if a non existent path is requested `jhp` will internally call `global.jhp`. You can then use whatever routing logic you like e.g route back to `index.html`.

## Error screenshot

If your JavaScript has an error, then you'll get a glorious animated Amiga 500 `Guru Meditation` warning:

![](error.png)

## What's left

**Mandatory**

- [x] Inject server "variables" e.g headers, querystring and other meta information
- [x] Handle writing response headers, and http statuses
- [x] Host function to print to `stdout`
- [x] File I/O _(read, write, append)_
- [x] Create host http get/post functions
- [x] Handle "Import" Pragmas _(will need to recursively walk and handle cycle detection)_
- [x] Global fallback handler something like `global.jhp`
- [ ] Host database function _(probably PostgreSQL?)_

In theory once all the above features are completed, `jhp` could realistically be used to build
pretty much any fully featured Web Application. It would be really hilarious if someone used it to
create something WordPress using `jhp`

**Nice to Have**

- [ ] Visual Studio extension for `jhp`
- [ ] Handle caching once parsed

# Quick Start

## Building from source

This the method I'd recommend the most, build it from source, just clone this repo and use cargo as
usual.

```shell
$ git clone https://github.com/AnharHussainMiah/jhp
$ cd jhp/
$ cargo build --release
# binary should be under target/release
```

## Download from precompiled releases

If you happen to have the same compile target, then you can pick the latest version from the releases
section:

[Latest Precompiled (Linux/Mac) Releases](https://github.com/AnharHussainMiah/jhp/releases)

## Curl | Bash for the lazy

Before you curl bash, remember my video: [Curl Bash Installation aka "Trust me bro!"](https://www.youtube.com/watch?v=KKK2JBJ9QCc). I've taken care to keep the script as simple as possible, have a read and only
if you're happy:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/AnharHussainMiah/jhp/refs/heads/main/install.sh | sh
```

[todo]

# FAQs

## Was AI used?

Almost all the code was handwritten with rare exceptions: I used chatGPT to search for bits of boilerplate, for example I couldn't be bothered with bashing out the axum request handler code. I could have probably gotten the same boilerplate from the actual axum documentation, but it was too boring and tedious for me, oh and the logo that's Google Gemini, because I'm not a talented artist and its a placeholder for now. I hope at some point the logo can be done by a real artist.

You can tell if you read the codebase, it's not a bazillion lines of random code splattered all over the place, so no AI code vomit / slop here!

The `license` is the new `Contextual Copyleft AI (CCAI) License v1.0`, because why should big tech hoover everything up without proper attribution? _(aka stealing)_. Isn't it amazing that it's totally fine for big tech to copy from everyon else, but when anyone tries to "distill" their models _(that they scraped from us)_ suddenly this is somehow a problem?!

## Is JHP available for Windows/MacOS?

I haven't tried compiling `jhp` for windows, if you're a windows user give it go, it might compile
just fine, there isn't any Linux specific calls at the moment.

Of course, you could also just use WSL and that should work just fine.

MacOS most likely will work fine given its a \*nix family _(assuming it compiles)_.

## Hey it doesn't have host function X?

Raise a GitHub issue, if it's really popular I might be able to add it, or even better send in a PR!

## Can I use this in production?

TL;DR **Don't?** unless you _really_ want to, do your own tests first.

## Contributions

Always welcome, but just a few things:

- You actually know Rust, please I don't want to deal with AI slop code.
- It has to be useful to a lot of people, not something that might be cool.
- Please follow the existing codebase style! I like simple flat modules, with very little abstraction.
- If you build a large feature, you own it.
- I don't mind _some_ AI use if used tastefully, but as per the previous statement, I take the Linus Torvalds approach: the code your create is now your responsibility, if it breaks you have to fix it.

## License

Contextual Copyleft AI (CCAI) [License v1.0](license)

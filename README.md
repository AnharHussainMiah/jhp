# JHP: JavaScript Hypertext Processor

![](logo.jpg)

## Kinda like PHP but with JavaScript instead...

I got up one morning thinking, us devs hate JavaScript and PHP, what if we combined them together so that we had something that worked just like PHP but with JavaScript as the templating language instead? and thus JHP was born!

And it pretty much works as you would expect it!

## But serioulsy why?!

On a semi serious note, `jhp` at the moment compiles down to around 3mb, although I haven't tried compiling under alpine, imagine building the container image, it could be end up under 10mb! compare that to a `php` container image and it's around 400mb, you can get that down to around 150-200mb if you use something like Alpine + Nginx and lots of elbow grease, but honestly that's still huge!

Using Node, Deno or Bun is kinda bad as well, with Node + Alpine around 250mb, while with Deno and Bun can be squeezed to around 50mb, but that's still big!

Size aside we get to use vanilla JavaScript, perhaps this was what `nodejs` was really supposed to be :D

## Ok enough talk, show me the code!

Instead of having `.php` files we have `.jhp` files, and they work almost the same way, instead of using `<?php` tags, use `<?jhp` and instead of writing `php` we write vanilla JavaScript and then just dump
the "website" files into a `public` folder including all your `.jhp` files.

**Example:**

A `index.jhp` file can contain normal HTML with JavaScript embedded anywhere you need it just like in PHP:

```html
<h1>Welcome!</h1>

<?jhp const name = "this is a serioulsy bad idea!"; ?>

<p>Hello, <?jhp console.log(name); ?>.</p>
```

assuming you have the `jhp` binary compiled and installed on your machine, as long as the `public` folder is in your
current directory just run `jhp` and it will start serving the `public` folder.

And now nagivate to it in your browser `http://localhost:8080/index.jhp` and bask in the glory that is `JHP` rendered goodness.

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
    status: 200,
    headers: {
        "content-type": "text/html; charset=utf-8",
        "cache-control": "no-cache"
    },
    body: ''
};
```

Simply mutate this object to set the status code and headers that you want to return to the client browser, example:

```javascript
RESPONSE.status = 404;
RESPONSE.headers["x-custom-header"] = "some custom value here";
```

**Note:** doing `console.log` appends to the `RESPONSE.body` automatically, so this does not need to be mutated
manually, unless you wanted to forcefully overide the template parsing.

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
- [ ] Host database function _(probably PostgreSQL?)_
- [ ] Handle "Import" Pragmas _(will need to recursively walk and handle cycle detection)_

In theory once all the above features are completed, `jhp` could realistically be used to build
pretty much any fully featured Web Application. It would be tototally hilarious if someone used it to
create something WordPress using `jhp`

**Nice to Have**

- [ ] Handle caching once parsed

# Quick Start

## Building from source

This the method I'd recommend the most, built it from source, just clone this repo and use cargo.

```shell
$ git clone https://github.com/AnharHussainMiah/jhp
$ cd jhp/
$ cargo build --release
# binary should be under target/release
```

## Download from precompiled releases

If you happen to have the same compile target, then you can pick the latest version from the releases
section:

[todo]

## Curl | Bash for the lazy

If you're feeling super lazy, just curl bash:

[todo]

# FAQs

## Was AI used?

Almost all the code was handwritten with rare exceptions: for example I used chatGPT to search for bits of boilerplate, for example I couldn't be bothered with bashing out the axum request handler code. I could have probably gotten the same boilerplate from the actual axum documentation, but it was too boring and tedious for me, oh and the logo that's Google Gemini, because I'm not a talented artist and its a placeholder for now. I hope at some point the logo can be done by a real artist.

You can tell if you read the codebase, it's not a bazillion lines of random code splattered all over the place, so no AI code vomit / slop here!

The `license` is the new `Contextual Copyleft AI (CCAI) License v1.0`, because why should big tech hoover everything up without proper attribution? _(aka stealing)_. Isn't it amazing that it's totally fine for big tech to copy from everyon else, but when anyone tries to "distill" their models _(that they scraped from us)_ suddenly this is somehow a problem?!

## Is JHP available for Windows/MacOS?

I haven't tried compiling `jhp` for windows, if you're a windows user give it go, it might compile
just fine, there isn't any Linux specific calls at the moment.

Of course, you could also just use WSL and that should work just fine.

MacOS mostly likely will work fine given its a \*nix OS _(assuming it compiles)_.

## Hey it doesn't have host function X?

raise a GitHub issue, if it's really popular I might be able to add it, or even better send in a PR!

## Can I use this in production?

TL;DR **Don't**

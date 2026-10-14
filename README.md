# JHP: JavaScript Hypertext Processor

![](logo.jpg)

## Kinda like PHP but with JavaScript instead...

I got up one morning thinking, us devs hate JavaScript and we also hate PHP, what if we combined them together so that we had something that worked just like PHP but with JavaScript as the templating language instead? and thus JHP was born!

And it pretty much works as you would expect it!

## But serioulsy why?!

On a semi serious note, `jhp` at the moment compiles down to around 3mb, although I haven't tried that under alpine, imagine building the docker image, it could be under 10mb, compare that to a `php` container image and it's around 400mb, you can get that down to around 150-200mb if you use something like Alpine + nginx and lots of elbow grease, but honestly that's still huge!

Using Node, Deno or Bun is kinda bad as well, with Node + Alpine around 250mb, and Deno and Bun can be squeezed to around 50mb, but that's still big!

Size aside we get to use vanilla JavaScript, perhaps this was what `nodejs` was really supposed to be :D

## Ok enough talk, show me the code!

Instead of having `.php` files we have `.jhp` files, and they work almost the same way, instead of using `<?php` tags, use `<?jhp` and instead of writing `php` we write vanilla JavaScript and then just dump
the "website" files into a `public` folder including all your `.jhp` files.

**Example:**

A `index.jhp` file can contain normal HTML with JavaScript embedded wherever you need it just like in PHP:

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

## Accessing Uri, Path, and QueryString

Uri, path and querystring are available in super global objects as shown below:

```javascript
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

## Accessing body data

The body payload is available in the `BODY` super global _(please note 1mb max size)_, for example:

```javascript
console.log(BODY);
```

## Error screenshot

If your JavaScript has an error, then you'll get a glorious animated Amiga 500 `Guru Meditation` warning:

![](error.png)

## What's left

- [ ] Create host http get/post functions
- [x] Inject server "variables" e.g headers, querystring and other meta information
- [ ] Handle writing headers, and http statuses
- [ ] Host database function _(probably postgresql?)_
- [ ] Handle caching once parsed
- [ ] File I/O
- [ ] Host function to print to `stdout`
- [ ] Handle "Import" Pragmas _(will need to recursively walk and handle cycle detection)_

In theory once all the above features are completed, `jhp` could realistically be used to build
pretty much any fully featured Web Application. It would be tototally hilarious if someone used it to
port WordPress using `jhp`

## Was AI used?

Almost none, all the code was handwritten with the rare exceptions I used chatGPT to search for bits of boilerplate, for example I couldn't be bothered
with bashing out the axum request handler code. I could have probably gotten the same boilerplate from the actual axum documentation, but it was too boring and tedious for me, oh and the logo that's Google Gemini, because I'm not a talented artist and its a placeholder for now. I hope at some point the logo can be done by a real artist.

You can tell if you read the codebase, it's not a bazillion lines of random code splattered all over the place, so no AI code vomit / slop here!

The `license` is the new `Contextual Copyleft AI (CCAI) License v1.0`, because why should big tech hoover everything up without
proper attribution _(aka stealing)_, but when others "distill" their models they get all angry?

## Can I use in production?

TL;DR **Don't**

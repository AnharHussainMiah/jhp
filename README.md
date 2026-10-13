# JHP: JavaScript Hypertext Processor

![](jhp-logo.jpg)

## Kinda like PHP but with JavaScript instead...

I got up one morning thinking, us devs hate JavaScript and we also hate PHP, so what if we had something like PHP, but instead of PHP we could write it in JavaSciptt? and thus JHP was born!

It's pretty much as you would expect it!

## Enough talk, show me the code!

Instead of having `.php` files we have `.jhp` files instead, and they work almost the same way, just dump
your "website" files into a `public` folder including all your `.jhp` files.

**Example:**

A `index.jhp` file can contain normal HTML with JavaScript embedded wherever you need it:

```html
<h1>Welcome!</h1>

<?jhp const name = "this is a serioulsy bad idea!"; ?>

<p>Hello, <?jhp console.log(name); ?>.</p>
```

assuming you have the `jhp` binary compiled installed on your machine, as long as the `public` folder is in your
current directory just run `jhp` and it will start serving the `public` folder.

And now nagivate to it in your browser `http://localhost:8080/hello.jhp` and bask in the glory that is `JHP` rendered goodness.

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

## Error screenshot

If your JavaScript has an error, then you'll get a glorious animated Amiga 500 `Guru Meditation` warning:

![](error.png)

## What's left

- [ ] Create host http get/post functions
- [ ] Inject server "variables" e.g headers, querystring and other meta information
- [ ] Handle writing headers, and http statuses
- [ ] Host database function _(probably postgresql?)_
- [ ] Handle caching once parsed
- [ ] File I/O
- [ ] Host function to print to `stdout`
- [ ] Handle "Import" Pragmas _(will need to recursively walk and handle cycle detection)_

In theory once all the above features are completed, `jhp` could realistically be used to build
pretty much fully featured Web Application. It would be tototally hilarious is someone used it to
re-create WordPress using `jhp`

## Production Use

TL;DR

**Don't**

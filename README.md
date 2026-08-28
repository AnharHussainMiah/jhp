# JHP: Java HyperScript Preprocessor

![](logo.jpg)

## The worst of all worlds, and a technology no one asked for

I got up one morning thinking, devs hate JavaScript and they also hate PHP, hey why not combine them? and JHP was born. JHP is a tiny static file server written in Rust that lets you run JavaScript when serving `.jhp` files. Yes, **JavaScript**, on the server... inside your HTML. I know! Normal files are served well normally. But files ending in `.jhp` are treated as server templates, allowing JavaScript to be embedded between `<?jhp` and `?>` tags and the JavaScript is then evaluated using **QuickJS** before the resulting content is sent to the client. So you can do things like generate dynamic HTML, calculate values, make decisions, and generally do server side rendering just like PHP has been doing this since the dinosaurs roamed the Earth. I just wanted to see what would happen, but why you ask? _morbid curiosity_. JHP was created to answer one of the most dangerous questions in software engineering:

_"What if we did this?"_

And with this questionable idea along with Rust and QuickJS, and enough free time to I was able to make the questionable idea real. The project is not intended to revolutionise web development. It is mostly an experiment to see how far we can take the concept of **"this is probably a bad idea, but..."**

## Enough talk, show me the code!

Dump your static web `dist` into a folder called `public`, and the once you have compiled `jhp` just run it
relateive to where ever your `public` folder is e.g. `./jhp`

A `hello.jhp` file can contain normal HTML with JavaScript embedded wherever you need it:

```html
<h1>Welcome!</h1>

<?jhp const name = "this is aserioulsy bad idea!"; ?>

<p>Hello, <?jhp console.log(name); ?>.</p>
```

And now nagivate to it in your browser `http://localhost:8080/hello.jhp` and bask in the glory that is `JHP` rendered goodness.

JHP evaluates the JavaScript and sends the resulting document to the browser. The browser gets HTML.
Your server gets JavaScript. Everyone gets what they deserve. `.jhp` files are executable code. If an untrusted user can create or modify one, you should assume they can execute JavaScript with whatever privileges and capabilities are available to the JHP process. Do not treat .jhp files as harmless user content. Do not let random people upload them and then act surprised when something terrible happens. Understand the runtime and isolate it appropriately if you're doing anything remotely serious.

## Error Screenshot

If your JavaScript has an error, you'll get an animated `Guru Meditation` _(Amiga 500, what a time)_ warning:

![](error.png)

## Production Use

**Don't.**

JHP is an experimental project and production use is not advised. There are already mature web servers, template engines, frameworks, and server side JavaScript runtimes that exist for very good reasons. JHP exists because I wondered what would happen if I built this. If you're deploying it to production, you've gone beyond the scope of the experiment and into the realm of your own poor decision making. I respect that!
but I do not endorse it.

# Introduction

There are a lot of resources out there about using Rust and WASM for web development. However, there is almost nothing out there about using Rust and WASM for web _extension_ development. And in fact, there are currently a few hurdles to get a web extension started with Rust. So I thought I'd go through a simple example to show how to use Rust with web extensions in some more detail.

This book will be structured as a tutorial. I'm going to go through the steps I took while investigating this topic and go into more detail with some of the challenges I faced, either with WASM generally or specifically with web extensions. Feel free to follow along, or just read through the chapters.

## *Disclaimer*

I am not a web developer. I don't consider myself particularly strong in web tech either. I originally wanted to use Rust and WASM to avoid using JavaScript and webpack at all, but I definitely needed some JavaScript and webpack knowledge to get through this. Basically, forgive me if I make a silly JavaScript mistake or have a weird webpack configuration. This is all still fairly new to me.

Also, don't take this as a "best-practices" without confirming the actual best-practices yourself. This is my own investigation into Rust and WASM, and I'm sure to be doing some less-than-optimal things throughout this book.

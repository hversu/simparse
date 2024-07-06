just a simple http request and scrape

## config

requires tor running on 9050

## usage
`cargo run <url> <tag>`

## example usage
get all \<td>, \<p>, \<h1>,... etc tags from lipsum.com, including href using path string format (e.g. `a.href`):

`cargo run https://news.google.com h1,h2,h3,h4,p,article,td,ul,li,lo,a,a.href`

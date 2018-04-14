# jofogarss

Convert [Jófogás](https://www.jofogas.hu) searches into an RSS feed.

## Build

Use cargo to build the binary.

```
cargo build --release
```

## Usage

Add a query string as the first command line argument and the tool will print
the rss feed to the standard output.

Use a script or cron or run it manually every once in a while, redirect the
output to a file. Put the XML file on your RaspberryPi, on a server, on S3 or
anywhere you can point your rss reader to.

## Example

A simple bash infinite loop that will update the feed every 6 minutes:


```bash
#!/bin/bash
while true
do
    ./jofogarrs QUERY > /path/to/my/feed.xml
    sleep 360
done
```
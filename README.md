# Some graphs of exponential back-off retry rates

This repo really isn't much. I'm writing a blog article to go up at
https://arbitrary.name/blog/all/backoff.html, and as part of that I
want to put up some graphs of retry rates versus time. This repo
simply contains the code to generate the data as CSVs, and then to
convert the data into PNGs.

The only thing that's really new for me is using a command-line tool
to generate the graphs. Usually I'm lazy and load the data into Google
Sheets, generate a graph in there, and take a screenshot of it! This
is convenient, but not very reproducable, so I thought I'd try the
scripted tool approach.

gnuplot was as unpleasant as ever, so I decided to try
[matplotlib](matplotlib.org). Averse as I am to using Python, it was a
minimally unpleasant experience. Would recommend!

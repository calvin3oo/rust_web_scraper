# Overview

This repo is a simple program that is a demonstration of web scraping walmart.com in rust. 

My purpose for writing this is that I have recently discovered how simple it is to start with web scrapers and how practical they can be in getting a lot of data.

<!-- TODO: [Software Demo Video](http://youtube.link.goes.here) -->

# Development Environment

I used the following libraries:
reqwest for making the API calls.
scraper for parsing the HTML.
urlencoding for encoding the search query.
std for getting input from the console.

# Useful Websites

{Make a list of websites that you found helpful in this project}

- [Web Scraping. Rust Lang - Youtube](https://www.youtube.com/watch?v=xOhfeuWIoms&ab_channel=web3.online)
- [Web Scraping with Python - Start HERE](https://www.youtube.com/watch?v=1PCWwK0AsE0&ab_channel=JohnWatsonRooney)

# Future Improvement Ideas

- Make a demo video
- Add more functionality, sometimes the scraper doesn't work and I could add in more error handling and more robust query selectors
- Add in more functionality to the program, such as being able to search for multiple items at once
- Better my query selectors to be able to get the specific price as a number rather than just strings of text
---
id: xmlhttp-request
aliases: []
tags:
  - #learn
  - #http
  - #JavaScript
---

# xmlhttp-request
2026-03-18  

## what is it
  The XMLHttpRequest API enable app to make HTTP requests to web servers and receive the response programmatically using JavaScript. This let's website to update just part of the page with data form server, rather than having to navigate to whole new page. Some time also knows as AJAX

## how it works
To make an HTTP requests:
1. Create a new XMLHttpRequest instance by calling a constructor `new XMLHttpRequest()`.
2. Initialize by calling `.open()` method to provide URL for the request, and optionally username and password.
3. Attach event handlers to get the result of the request such as `.load` event is fired when the request has successfully complete, and the `.error` event is fired in various conditions.
4. Send the requests by calling `.send()`

## example
```JavaScript
  // Create XMLHttpRequest object
  const xhr = new XMLHttpRequest();

  // (true) is Async flag
  xhr.open('GET', 'https://example.com', true);

  // Set event listener
  xhr.onload = () => {
    console.log(xhr.status, xhr.responseText);
  };
  xhr.onerror = () => {
    console.log('error occurred')
  };
  // Send request
  xhr.send();
  ``````

## gotchas

## links
[XMLHttpRequest](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/XMLHttpRequest)
[[http-rest-api]]

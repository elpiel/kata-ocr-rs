# Extract the numbers from an old text format.

~~~
    _  _     _  _  _  _  _  _ 
  | _| _||_||_ |_   ||_||_|| |
  ||_  _|  | _||_|  ||_| _||_|
~~~

Each account number is written on 4 lines: 3 with characters, followed by one blank line.

This implementation can work with 1+ numbers in the string and can read only 1 line of numbers for now.

## More examples
~~~
    _  _     _  _  _  _  _
  | _| _||_||_ |_   ||_||_|
  ||_  _|  | _||_|  ||_| _| 
~~~
=> 123456789
~~~
 _  _  _  _  _  _  _  _  _
| || || || || || || || || |
|_||_||_||_||_||_||_||_||_|
~~~~
=> 000000000
~~~
  |  |  |  |  |  |  |  |  |
  |  |  |  |  |  |  |  |  |
~~~
=> 111111111
~~~
 _  _  _  _  _  _  _  _  _
 _| _| _| _| _| _| _| _| _|
|_ |_ |_ |_ |_ |_ |_ |_ |_ 
~~~
=> 222222222
~~~
 _  _  _  _  _  _  _  _  _
 _| _| _| _| _| _| _| _| _|
 _| _| _| _| _| _| _| _| _|
~~~
=> 333333333
~~~
|_||_||_||_||_||_||_||_||_|
  |  |  |  |  |  |  |  |  |
~~~
=> 444444444
~~~
 _  _  _  _  _  _  _  _  _
|_ |_ |_ |_ |_ |_ |_ |_ |_
 _| _| _| _| _| _| _| _| _|
~~~
=> 555555555
~~~
 _  _  _  _  _  _  _  _  _
|_ |_ |_ |_ |_ |_ |_ |_ |_
|_||_||_||_||_||_||_||_||_|
~~~
=> 666666666
~~~
 _  _  _  _  _  _  _  _  _
  |  |  |  |  |  |  |  |  |
  |  |  |  |  |  |  |  |  |
~~~
=> 777777777
~~~
 _  _  _  _  _  _  _  _  _
|_||_||_||_||_||_||_||_||_|
|_||_||_||_||_||_||_||_||_|
~~~
=> 888888888
~~~
 _  _  _  _  _  _  _  _  _
|_||_||_||_||_||_||_||_||_|
 _| _| _| _| _| _| _| _| _|
~~~
=> 999999999


## References:
* Original source: [coding-dojo/bank-ocr](https://code.joejag.com/coding-dojo/bank-ocr/)
* [DomCode kata](https://github.com/domcode/kataday)
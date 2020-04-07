# Tests
This file shows test outputs. Tests resides with client in `tests` directory.
```
Welcome to Pygmy.com - the World’s smallest book store
Server URL is: http://localhost:34800
You can
1. Search books by topic ('Distributed systems' and 'Graduate School')
2. Get available book list
3. Get information about one book
4. Buy a book, for free!!!
5. Test
6. Leave
Select by input the number: 
5
Run tests to see if everything is working
Running test 1 of 5
Getting all books in store and verify the number...1. How to get a good grade in 677 in 20 minutes a day - Price: 19.9, Topic: Distributed systems, Stock: 4
2. RPCs for Dummies - Price: 29.9, Topic: Distributed systems, Stock: 17
3. Xen and the Art of Surviving Graduate School - Price: 9.9, Topic: Graduate School, Stock: 25
4. Cooking for the Impatient Graduate Student - Price: 30.9, Topic: Graduate School, Stock: 15
PASSED
Running test 2 of 5
Get one book, number 1, should be the 'How to get a good grade in 677 in 20 minutes a day'...PASSED
Running test 3 of 5
Search topic related books 'sys', should support fuzzy matching and match 1 topic and 2 books
1. How to get a good grade in 677 in 20 minutes a day - Price: 19.9, Topic: Distributed systems, Stock: 4
2. RPCs for Dummies - Price: 29.9, Topic: Distributed systems, Stock: 17
PASSED
Running test 4 of 5
Buy a book, number 2, amount 1...
Bought book RPCs for Dummies, amount 1
PASSED
Running test 5 of 5
Checking remaining stock of book 2...PASSED
```
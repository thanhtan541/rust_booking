As a customer,
I can look up all available rooms from x to y
So I am able to make a book

ACs:

I have input start_date x and end_date y and x < y
I click search button
I see a list of available rooms 

I have input start_date x and end_date y and x > y
I click search button
I should see an error since x should not greater than y

I have input start_date x and end_date y and x > y
I click search button
There is no results, the page should be empty

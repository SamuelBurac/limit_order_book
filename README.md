# A limit order book implementation

Requirements

- Add an limit buy order
- Add a limit sell order
- Cancel an order

## Goal

Simulate 1,000,000 orders

An idea for a new data structure
The Number Line
Like a hashmap but no multiple references for one index
and each entry has a reference to the next which may or
may not be the next index.
This is because for this project the goal is to find
the next available lowest seller or highest buyer.
This is easily done with a number line where you locate the
max or the min and work your way up or down.

Number Line
int maxPrice
int minPrice
&LineItem minItem
&LineItem maxItem // max an min priced items
LineItem[] arr // an array of queues which contains all the entries for a price
int size

LineItem
u64 price // int this case this is the price of the order
T data
&LineItem next
&LineItem previous

When entering a Line item
It's placed at the mod of the number / hash
and if there's overlap then the array in increased in size.
For something like this orderbook where we're testing millions
of orders we could give an initial size to the array of like 100k
and in this way we reserve a ton of memory upfront we know we'll need.
But maybe we have an issue where the numbers are too close together?
This should be a good thing then we won't have to expand the array they'll
just end up at the index next to one another. But also there could be
orders for the same price. In that case we should have a queue

Maybe in order to use smaller numbers subtract the original price by the min val
and use the mod of that as the index. That's an interesting experiment.

Referencing the next item may be messy since when you fulfill an order.
It needs to be removed from the orderbook.

> [!NOTE]
> I wonder how much time generating uuid's takes

Also the price should just be the price in pennies that way math is perfect.

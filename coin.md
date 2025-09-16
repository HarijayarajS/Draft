Got it 👍 — your code is meant to update the coin-related payment types in orderSettingUpdateCoins.
Here are the bullet steps of what it does:

Fetch existing settings

Calls orderSettingGet() to retrieve the current setting.

Extracts paymentTypes from it, or uses an empty array if none exist.


Prepare input

Takes data (list of OrderSettingCoin) or defaults to an empty array.


Iterate through each coin
For each coin in data:

1. Check if a payment type already exists with the same id.

If found:

Update its title, type (coins_${coin.name}), and isRevenueIgnored.


If not found:

Create a new OrderSettingPaymentType object with:

id = coin.id

isActive = true

isCustom = false

color = 'green'

icon = 'iconCashCoin'

title = coin.name

type = coins_${coin.name}

sortNo = paymentTypes.length + 1

isRevenueIgnored = coin.isIgnored


Push the new item into paymentTypes.




2. Add the coin.id to a set paymentTypeIds for tracking.



Clean up old coin payment types

Remove from paymentTypes any element where:

el.type starts with "coins_", and

Its id is not in the current paymentTypeIds.



Save updates

Call orderSettingUpdate({ coins, paymentTypes }).

Return its result (Promise<boolean>).



Would you like me to rewrite this function cleanly (proper TypeScript syntax, no typos) so it’s easier to maintain, or do you just want the high-level steps?

